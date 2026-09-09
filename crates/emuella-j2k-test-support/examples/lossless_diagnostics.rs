//! Read-only, opt-in native D2 stage diagnostic. Emits aggregate JSON only.
#![allow(unsafe_code)]
#[path = "support/allocation_meter.rs"]
mod allocation_meter;
use emuella_j2k_codestream as cs;
use emuella_j2k_core::*;
use emuella_j2k_test_support::lossless_diagnostics::decode_native_profiled;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::Instant;

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn run(args: &[String]) -> std::result::Result<serde_json::Value, Box<dyn std::error::Error>> {
    if args.len() != 8 {
        return Err("usage: lossless_diagnostics RAW CODESTREAM WIDTH HEIGHT COMPONENTS BITS planar|interleaved".into());
    }
    let width: u32 = args[3].parse()?;
    let height: u32 = args[4].parse()?;
    let components: u16 = args[5].parse()?;
    let bits: u8 = args[6].parse()?;
    let layout = match args[7].as_str() {
        "planar" => ComponentLayout::Planar,
        "interleaved" => ComponentLayout::Interleaved,
        _ => return Err("layout must be planar or interleaved".into()),
    };
    if !matches!(bits, 8 | 16)
        || !matches!(components, 1 | 3 | 8)
        || (components == 8 && bits != 16)
    {
        return Err(
            "diagnostic requires unsigned U8/U16 grey/RGB or exactly eight U16 bands".into(),
        );
    }
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
    let options = EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let limits = LosslessEncodeLimits::default();
    let requirements = lossless_encode_requirements(&info, &options, &limits)?;
    let bytes = usize::from(bits / 8);
    let plane_len = width as usize * height as usize * bytes;
    // Materialise once, read-only. No input, codestream or reconstructed pixels
    // are written to a file. Input I/O and identity hashing precede measurement.
    let samples = std::fs::read(&args[1])?;
    let input_stream = std::fs::read(&args[2])?;
    if samples.len() != plane_len * usize::from(components) {
        return Err("RAW length disagrees with packed geometry".into());
    }
    let input_hash = hash(&samples);
    let stream_hash = hash(&input_stream);
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
        return Err(
            "codestream geometry/precision or D2 diagnostic profile disagrees with RAW".into(),
        );
    }
    let mct = coding.multiple_component_transform;
    let planes: Vec<_> = (0..usize::from(components))
        .map(|i| cs::LosslessD2Plane {
            samples: if layout == ComponentLayout::Planar {
                &samples[i * plane_len..(i + 1) * plane_len]
            } else {
                &samples[i * bytes..]
            },
            stride_bytes: width as usize
                * bytes
                * if layout == ComponentLayout::Planar {
                    1
                } else {
                    usize::from(components)
                },
            sample_step_bytes: bytes
                * if layout == ComponentLayout::Planar {
                    1
                } else {
                    usize::from(components)
                },
        })
        .collect();
    let baseline = allocation_meter::reset();
    let start = Instant::now();
    let (encoded, e) = cs::encode_lossless_d2_profiled(width, height, bits, &planes, limits)?;
    let encode_ns = start.elapsed().as_nanos();
    let peak = allocation_meter::peak_since(baseline);
    // Freeze encoder allocation measurement before decode and all verification.
    let start = Instant::now();
    let native = decode_native_profiled(&input_stream, layout, true)?;
    let decode_ns = start.elapsed().as_nanos();
    let data = &native.data;
    let d = &native.timings;
    let packing_ns = native.packing_ns;
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
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: layout,
        ..Default::default()
    };
    let ordinary_decoded = decode(&input_stream, &decode_options)?;
    let encoded_decoded = decode(&encoded, &decode_options)?;
    let ordinary_planes = if layout == ComponentLayout::Planar {
        samples
            .chunks_exact(plane_len)
            .map(|s| Plane::new(s, width, height, width as usize * bytes, info.sample_format))
            .collect::<Result<Vec<_>>>()?
    } else {
        Vec::new()
    };
    let view = if layout == ComponentLayout::Planar {
        ImageView::Planar {
            info: &info,
            planes: &ordinary_planes,
        }
    } else {
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: width as usize * bytes * usize::from(components),
        }
    };
    let ordinary_encoded = encode(view, &options)?;
    let exact = native.width == width
        && native.height == height
        && native.bits_per_sample == bits
        && !native.signed
        && native.components == usize::from(components)
        && *data == expected
        && ordinary_decoded.data == expected
        && encoded_decoded.data == expected
        && encoded == ordinary_encoded;
    let allocation_within_limits = peak as u64 <= requirements.working_bytes
        && encoded.capacity() as u64 <= limits.max_output_bytes;
    let routes = d.tier1_routes;
    let work = &d.tier1_work_counters;
    let stage_sum = e.conversion_level_shift_rct_ns
        + e.forward_dwt_ns
        + e.block_preparation_ns
        + e.tier1_ns
        + e.packet_headers_ns
        + e.assembly_ns;
    Ok(json!({
        "schema": "emuella-lossless-diagnostics-v1", "exact": exact, "allocation_within_limits": allocation_within_limits,
        "input": { "sha256": input_hash, "codestream_sha256": stream_hash, "width": width, "height": height, "components": components, "bits": bits, "layout": args[7], "decode_mct": mct },
        "build": { "source_revision_label": std::env::var("EMUELLA_DIAGNOSTIC_REVISION").ok(), "parallel_feature": cfg!(feature="parallel"), "simd_feature": cfg!(feature="simd") },
        "verification": { "native_sample_exact": *data == expected, "ordinary_decode_exact": ordinary_decoded.data == expected, "encoded_roundtrip_exact": encoded_decoded.data == expected, "ordinary_encode_byte_exact": encoded == ordinary_encoded },
        "encode": { "wall_ns": encode_ns, "internal_total_ns": e.total_ns, "conversion_level_shift_rct_ns": e.conversion_level_shift_rct_ns, "forward_dwt_ns": e.forward_dwt_ns, "block_preparation_ns": e.block_preparation_ns, "tier1_ns": e.tier1_ns, "packet_headers_ns": e.packet_headers_ns, "assembly_ns": e.assembly_ns, "unattributed_ns": e.total_ns.saturating_sub(stage_sum), "tier1_backend": "checked-baseline-sequential", "component_samples": e.component_samples, "rct_pixels": e.rct_pixels, "tier1_blocks": e.checked_tier1_blocks, "included_tier1_blocks": e.included_tier1_blocks, "tier1_coefficients": e.tier1_coefficients, "tier1_coding_passes": e.tier1_coding_passes, "tier1_codeword_bytes": e.tier1_codeword_bytes, "packets": e.packets, "packet_header_bytes": e.packet_header_bytes, "packet_body_bytes_moved": e.packet_body_bytes_moved, "tier1_inner_operation_counts": null, "bytes": encoded.len(), "capacity": encoded.capacity(), "sha256": hash(&encoded), "peak_requested_bytes": peak, "working_bound": requirements.working_bytes },
        "decode": { "wall_including_native_packing_ns": decode_ns, "internal_total_ns": d.total_ns, "native_packing_ns": packing_ns, "marker_parse_ns": d.marker_parse_ns, "support_classification_ns": d.support_classification_ns, "tile_payload_ns": d.tile_payload_ns, "packet_headers_ns": d.packet_header_parse_ns, "segment_acquisition_ns": d.code_block_segment_acquisition_ns, "tier1_ns": d.tier1_decode_ns, "coefficient_placement_ns": d.coefficient_placement_ns, "inverse_dwt_ns": d.inverse_reversible_5_3_ns, "inverse_rct_ns": d.inverse_rct_ns, "sample_conversion_ns": d.sample_conversion_ns,
        "checked": { "blocks": routes.executed_checked.blocks, "coefficients": routes.executed_checked.coefficients, "segment_bytes": routes.executed_checked.segment_bytes },
        "packed_dense": { "blocks": routes.executed_packed_dense.blocks, "coefficients": routes.executed_packed_dense.coefficients, "segment_bytes": routes.executed_packed_dense.segment_bytes },
        "packed_sparse": { "blocks": routes.executed_packed_sparse.blocks, "coefficients": routes.executed_packed_sparse.coefficients, "segment_bytes": routes.executed_packed_sparse.segment_bytes },
        "work": { "cleanup_positions_visited": work.cleanup_positions_visited, "cleanup_mq_reads": work.cleanup_mq_reads, "significance_positions_visited": work.significance_positions_visited, "significance_mq_reads": work.significance_mq_reads, "magnitude_positions_visited": work.magnitude_positions_visited, "magnitude_mq_reads": work.magnitude_mq_reads, "sign_mq_reads": work.sign_mq_reads, "run_mode_mq_reads": work.run_mode_mq_reads }, "parallel_dispatch_suppressed_by_profiling": true }
    }))
}
fn main() {
    match run(&std::env::args().collect::<Vec<_>>()) {
        Ok(report) => {
            let success = report["exact"] == true && report["allocation_within_limits"] == true;
            println!("{report}");
            if !success {
                std::process::exit(1);
            }
        }
        Err(error) => {
            println!(
                "{}",
                json!({"schema":"emuella-lossless-diagnostics-v1", "status":"error", "error":error.to_string()})
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn driver_reports_json_exactness_after_measurement() {
        let root = std::env::temp_dir().join(format!(
            "emuella-lossless-diagnostics-{}",
            std::process::id()
        ));
        std::fs::create_dir(&root).unwrap();
        for components in [1, 3, 8] {
            let bytes: Vec<_> = (0..8 * 8 * components * 2)
                .map(|i| (i * 73 + i / 19) as u8)
                .collect();
            let planes: Vec<_> = (0..components)
                .map(|c| cs::LosslessD2Plane {
                    samples: &bytes[c * 2..],
                    stride_bytes: 8 * components * 2,
                    sample_step_bytes: components * 2,
                })
                .collect();
            let stream =
                cs::encode_lossless_d2(8, 8, 16, &planes, LosslessEncodeLimits::default()).unwrap();
            let raw_path = root.join("authored.raw");
            let stream_path = root.join("authored.j2k");
            std::fs::write(&raw_path, &bytes).unwrap();
            std::fs::write(&stream_path, &stream).unwrap();
            let args = vec![
                "diagnostic".into(),
                raw_path.to_str().unwrap().into(),
                stream_path.to_str().unwrap().into(),
                "8".into(),
                "8".into(),
                components.to_string(),
                "16".into(),
                "interleaved".into(),
            ];
            let report = run(&args).unwrap();
            assert_eq!(report["exact"], true);
            assert_eq!(report["allocation_within_limits"], true);
            assert_eq!(
                report["encode"]["tier1_backend"],
                "checked-baseline-sequential"
            );
            assert!(report["encode"]["tier1_inner_operation_counts"].is_null());
            assert_eq!(report["input"]["decode_mct"], components == 3);
            assert_eq!(std::fs::read(raw_path).unwrap(), bytes);
            assert_eq!(std::fs::read(stream_path).unwrap(), stream);
        }
        std::fs::remove_dir_all(root).unwrap();
        assert!(run(&[]).is_err());
    }
}
