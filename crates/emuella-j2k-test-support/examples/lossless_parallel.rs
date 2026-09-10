//! Linux operation CPU/allocation diagnostics; separate from headline timings.
#![allow(unsafe_code)]
#[path = "support/allocation_meter.rs"]
mod allocation_meter;
use emuella_j2k_codestream as cs;
use emuella_j2k_core::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, time::Instant};

fn cpu_ticks() -> std::result::Result<BTreeMap<u32, u64>, Box<dyn std::error::Error>> {
    let mut tasks = BTreeMap::new();
    for entry in std::fs::read_dir("/proc/self/task")? {
        let entry = entry?;
        let tid = entry.file_name().to_string_lossy().parse()?;
        let stat = std::fs::read_to_string(entry.path().join("stat"))?;
        let fields: Vec<_> = stat
            .rsplit_once(')')
            .ok_or("malformed task stat")?
            .1
            .split_whitespace()
            .collect();
        tasks.insert(tid, fields[11].parse::<u64>()? + fields[12].parse::<u64>()?);
    }
    Ok(tasks)
}
fn cpu_delta(
    before: &BTreeMap<u32, u64>,
    after: &BTreeMap<u32, u64>,
    hz: u64,
) -> serde_json::Value {
    let ticks: Vec<_> = after
        .iter()
        .map(|(tid, end)| (*tid, end.saturating_sub(*before.get(tid).unwrap_or(&0))))
        .collect();
    json!({"seconds": ticks.iter().map(|(_, ticks)| ticks).sum::<u64>() as f64 / hz as f64,
        "tick_hz": hz, "active_os_threads_at_tick_resolution": ticks.iter().filter(|(_, n)| *n > 0).count(),
        "per_thread_ticks": ticks })
}
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn run() -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 9 {
        return Err("usage: lossless_parallel RAW CODESTREAM WIDTH HEIGHT COMPONENTS BITS planar|interleaved WORKERS".into());
    }
    let width: u32 = args[3].parse()?;
    let height: u32 = args[4].parse()?;
    let components: u16 = args[5].parse()?;
    let bits: u8 = args[6].parse()?;
    let workers: usize = args[8].parse()?;
    if workers == 0
        || !matches!(components, 1 | 3 | 8)
        || !matches!(bits, 8 | 16)
        || (components == 8 && bits != 16)
    {
        return Err("unsupported diagnostic sample model or worker count".into());
    }
    if !cfg!(feature = "parallel") && workers != 1 {
        return Err("multiple workers require the parallel feature".into());
    }
    let layout = match args[7].as_str() {
        "planar" => ComponentLayout::Planar,
        "interleaved" => ComponentLayout::Interleaved,
        _ => return Err("invalid layout".into()),
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    pool.broadcast(|_| ());
    let hz: u64 = String::from_utf8(
        std::process::Command::new("getconf")
            .arg("CLK_TCK")
            .output()?
            .stdout,
    )?
    .trim()
    .parse()?;
    let samples = std::fs::read(&args[1])?;
    let input_stream = std::fs::read(&args[2])?;
    let info = ImageInfo::new(
        width,
        height,
        components,
        if bits == 8 {
            SampleFormat::U8
        } else {
            SampleFormat::U16_LE
        },
        match components {
            1 => ColorModel::Grayscale,
            3 => ColorModel::Rgb,
            _ => ColorModel::Unknown,
        },
        layout,
    )?;
    let bytes = usize::from(bits / 8);
    let plane_len = width as usize * height as usize * bytes;
    if samples.len() != plane_len * components as usize {
        return Err("RAW length mismatch".into());
    }
    let limits = LosslessEncodeLimits::default();
    let options = EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let requirements = pool.install(|| lossless_encode_requirements(&info, &options, &limits))?;
    let parsed = cs::parse(&input_stream)?;
    let coding = parsed.coding_style.as_ref().ok_or("missing COD")?;
    if parsed.image_width() != width
        || parsed.image_height() != height
        || parsed.siz.component_count() != components
        || parsed.siz.components.iter().any(|c| {
            c.bits_per_sample != bits
                || c.signed
                || c.horizontal_separation != 1
                || c.vertical_separation != 1
        })
        || coding.decomposition_levels != 2
        || coding.entropy_coder != cs::EntropyCoder::ClassicTier1
        || coding.transform != cs::WaveletTransform::Reversible53
        || coding.progression_order != cs::ProgressionOrder::Lrcp
        || coding.layers != 1
        || coding.code_block_style != 0
        || coding.code_block_width_exponent != 6
        || coding.code_block_height_exponent != 6
        || coding.sop_markers
        || coding.eph_markers
        || coding.precincts_declared
        || (components != 3 && coding.multiple_component_transform)
        || !parsed.component_coding_styles.is_empty()
        || parsed.tiles.len() != 1
        || parsed.siz.image_origin_x != 0
        || parsed.siz.image_origin_y != 0
        || parsed.siz.tile_origin_x != 0
        || parsed.siz.tile_origin_y != 0
        || parsed.siz.tile_width != width
        || parsed.siz.tile_height != height
    {
        return Err("decode stream differs from frozen D2 input profile".into());
    }
    let planes: Vec<_> = (0..components as usize)
        .map(|c| cs::LosslessD2Plane {
            samples: if layout == ComponentLayout::Planar {
                &samples[c * plane_len..(c + 1) * plane_len]
            } else {
                &samples[c * bytes..]
            },
            stride_bytes: width as usize
                * bytes
                * if layout == ComponentLayout::Planar {
                    1
                } else {
                    components as usize
                },
            sample_step_bytes: bytes
                * if layout == ComponentLayout::Planar {
                    1
                } else {
                    components as usize
                },
        })
        .collect();
    let core_planes: Vec<_> = if layout == ComponentLayout::Planar {
        samples
            .chunks_exact(plane_len)
            .map(|p| Plane::new(p, width, height, width as usize * bytes, info.sample_format))
            .collect::<std::result::Result<_, _>>()?
    } else {
        vec![]
    };
    let view = if layout == ComponentLayout::Planar {
        ImageView::Planar {
            info: &info,
            planes: &core_planes,
        }
    } else {
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: width as usize * components as usize * bytes,
        }
    };
    let before = cpu_ticks()?;
    let baseline = allocation_meter::reset();
    let start = Instant::now();
    let encoded = pool.install(|| encode_with_limits(view, &options, &limits))?;
    let encode_ns = start.elapsed().as_nanos();
    let encode_peak = allocation_meter::peak_since(baseline);
    let after = cpu_ticks()?;
    let encode_cpu = cpu_delta(&before, &after, hz);
    if encode_peak as u64 > requirements.working_bytes
        || encoded.capacity() as u64 > limits.max_output_bytes
    {
        return Err("encoder allocation bound exceeded".into());
    }
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: layout,
        ..Default::default()
    };
    let before = cpu_ticks()?;
    let baseline = allocation_meter::reset();
    let start = Instant::now();
    // This is the exact ordinary pool.install + core decode route. No prepared
    // reconstruction or stage profiler substitutes another scheduling policy.
    let decoded = pool.install(|| decode(&input_stream, &decode_options))?;
    let decode_ns = start.elapsed().as_nanos();
    let decode_peak = allocation_meter::peak_since(baseline);
    let after = cpu_ticks()?;
    let decode_cpu = cpu_delta(&before, &after, hz);
    let expected = if layout == ComponentLayout::Planar {
        ImageData::Planes(
            samples
                .chunks_exact(plane_len)
                .map(<[u8]>::to_vec)
                .collect(),
        )
    } else {
        ImageData::Interleaved(samples.clone())
    };
    if decoded.data != expected
        || pool.install(|| decode(&encoded, &decode_options))?.data != expected
    {
        return Err("native samples differ".into());
    }
    let (profiled, timing) =
        pool.install(|| cs::encode_lossless_d2_profiled(width, height, bits, &planes, limits))?;
    if profiled != encoded {
        return Err("profiled stream differs".into());
    }
    Ok(
        json!({"schema": 1, "identity": "ordinary-pool-operation-diagnostic", "source_revision_label": std::env::var("EMUELLA_DIAGNOSTIC_REVISION").ok(),
        "parallel_feature": cfg!(feature="parallel"), "simd_feature": cfg!(feature="simd"), "requested_pool_workers": workers,
        "width": width, "height": height, "components": components, "bits": bits, "layout": args[7],
        "raw_sha256": hash(&samples), "decode_stream_sha256": hash(&input_stream), "decode_mct": coding.multiple_component_transform, "encoded_sha256": hash(&encoded), "encoded_bytes": encoded.len(), "exact": true,
        "encode": {"wall_ns": encode_ns, "cpu": encode_cpu, "peak_requested_bytes": encode_peak, "working_bound": requirements.working_bytes, "output_capacity": encoded.capacity()},
        "decode": {"wall_ns": decode_ns, "cpu": decode_cpu, "peak_requested_bytes": decode_peak},
        "separate_profiled_encode": {"effective_workers": timing.effective_workers, "participating_tier1_workers": timing.participating_workers, "max_batch_blocks": timing.max_batch_blocks, "blocks": timing.checked_tier1_blocks},
        "measurement": "allocator-instrumented; /proc per-task CPU tick resolution; profiler participation collected in a separate invocation; not headline throughput"}),
    )
}
fn main() {
    match run() {
        Ok(report) => println!("{report}"),
        Err(error) => {
            eprintln!("{}", json!({"error":error.to_string()}));
            std::process::exit(1);
        }
    }
}
