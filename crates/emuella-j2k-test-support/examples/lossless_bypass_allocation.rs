//! Allocation-only diagnostics, separate from uninstrumented operation timing.
#![allow(unsafe_code)]
#[path = "support/allocation_meter.rs"]
mod allocation_meter;
use emuella_j2k::*;
use serde_json::json;
use sha2::{Digest, Sha256};
fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if !matches!(args.len(), 8 | 10) {
        return Err(
            "usage: lossless_bypass_allocation RAW WIDTH HEIGHT COMPONENTS BITS WORKERS STYLE [WORKING_BYTES OUTPUT_BYTES]"
                .into(),
        );
    }
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;
    let components: u16 = args[4].parse()?;
    let bits: u8 = args[5].parse()?;
    let workers: usize = args[6].parse()?;
    let style: u8 = args[7].parse()?;
    if !cfg!(feature = "parallel")
        || cfg!(feature = "simd")
        || !matches!(workers, 1 | 2 | 4 | 8)
        || style > 1
        || !matches!(bits, 8 | 16)
    {
        return Err("unsupported diagnostic build or shape".into());
    }
    let raw = std::fs::read(&args[1])?;
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
        ComponentLayout::Interleaved,
    )?;
    let view = ImageView::Interleaved {
        info: &info,
        samples: &raw,
        stride_bytes: width as usize * components as usize * usize::from(bits / 8),
    };
    let options = EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    pool.broadcast(|_| ());
    // Diagnose the admitted production envelope, including large complete
    // streams; report the exact queried bound alongside the observed peak.
    let limits = if args.len() == 10 {
        LosslessEncodeLimits {
            max_working_bytes: args[8].parse()?,
            max_output_bytes: args[9].parse()?,
        }
    } else {
        LosslessEncodeLimits::default()
    };
    let requirements = pool.install(|| {
        if style == 0 {
            lossless_encode_requirements(&info, &options, &limits)
        } else {
            lossless_bypass_encode_requirements(&info, &options, &limits)
        }
    })?;
    let baseline = allocation_meter::reset();
    let stream = pool.install(|| {
        if style == 0 {
            encode_with_limits(view, &options, &limits)
        } else {
            encode_lossless_bypass_with_limits(view, &options, &limits)
        }
    })?;
    let peak = allocation_meter::peak_since(baseline);
    if peak as u64 > requirements.working_bytes
        || stream.capacity() as u64 > limits.max_output_bytes
    {
        return Err("observed requested allocation exceeds admission".into());
    }
    let baseline = allocation_meter::reset();
    let decoded = pool.install(|| {
        decode(
            &stream,
            &DecodeOptions {
                mode: DecodeMode::Components,
                target_layout: ComponentLayout::Interleaved,
                ..Default::default()
            },
        )
    })?;
    let decode_peak = allocation_meter::peak_since(baseline);
    if decoded.data != ImageData::Interleaved(raw.clone()) {
        return Err("diagnostic reconstruction differs".into());
    }
    println!(
        "{}",
        json!({"width":width,"height":height,"components":components,"bits":bits,"workers":workers,"style":style,
        "raw_sha256":hash(&raw),"stream_sha256":hash(&stream),"complete_stream_bytes":stream.len(),"native_exact":true,
        "encode_peak_requested_bytes":peak,"decode_peak_requested_bytes":decode_peak,"working_bytes":requirements.working_bytes,
        "output_capacity":stream.capacity(),"output_capacity_limit":limits.max_output_bytes,
        "scope":"additional requested allocations during actual facade calls; conservative reallocation overlap; not RSS or headline timing"})
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
