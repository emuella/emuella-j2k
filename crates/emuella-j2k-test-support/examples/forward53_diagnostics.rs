//! Separate-build writer diagnostics. Reads one authorised packed RAW input;
//! emits aggregate identities and counters only, never samples or coefficients.
#[cfg(not(all(feature = "parallel", feature = "classic-execution-diagnostics")))]
fn main() {
    panic!("forward53_diagnostics requires parallel,classic-execution-diagnostics");
}

#[cfg(all(feature = "parallel", feature = "classic-execution-diagnostics"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use emuella_j2k_codestream as cs;
    use serde_json::json;
    use sha2::{Digest, Sha256};
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 10 {
        return Err("usage: forward53_diagnostics RAW WIDTH HEIGHT COMPONENTS BITS WORKERS STYLE reference|scalar|parallel PANEL_WIDTH".into());
    }
    let width: u32 = args[2].parse()?;
    let height: u32 = args[3].parse()?;
    let components: usize = args[4].parse()?;
    let bits: u8 = args[5].parse()?;
    let workers: usize = args[6].parse()?;
    let style: u8 = args[7].parse()?;
    let backend = match args[8].as_str() {
        "reference" => cs::Forward53Backend::Reference,
        "scalar" => cs::Forward53Backend::RowPanelScalar,
        "parallel" => cs::Forward53Backend::RowPanelParallel,
        _ => return Err("unknown diagnostic backend".into()),
    };
    let panel_width: usize = args[9].parse()?;
    if !matches!(components, 1 | 3 | 8)
        || !matches!(bits, 8 | 16)
        || !matches!(workers, 1 | 2 | 4 | 8)
        || !matches!(panel_width, 8 | 16 | 32)
        || style > 1
    {
        return Err("unsupported diagnostic shape".into());
    }
    let raw = std::fs::read(&args[1])?;
    let bytes = usize::from(bits / 8);
    let required = (width as usize)
        .checked_mul(height as usize)
        .and_then(|n| n.checked_mul(components))
        .and_then(|n| n.checked_mul(bytes))
        .ok_or("shape overflow")?;
    if raw.len() != required {
        return Err("RAW length does not match packed shape".into());
    }
    let planes: Vec<_> = (0..components)
        .map(|c| cs::LosslessD2Plane {
            samples: &raw[c * bytes..],
            stride_bytes: width as usize * components * bytes,
            sample_step_bytes: components * bytes,
        })
        .collect();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    pool.broadcast(|_| ());
    let (stream, d) = pool.install(|| {
        cs::with_forward53_diagnostic_policy(backend, panel_width, || {
            cs::observe_lossless_encode(|| {
                if style == 0 {
                    cs::encode_lossless_d2(
                        width,
                        height,
                        bits,
                        &planes,
                        cs::LosslessEncodeLimits::default(),
                    )
                } else {
                    cs::encode_lossless_d2_bypass(
                        width,
                        height,
                        bits,
                        &planes,
                        cs::LosslessEncodeLimits::default(),
                    )
                }
            })
        })
    });
    let stream = stream?;
    let hash = |bytes: &[u8]| {
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    };
    println!(
        "{}",
        json!({
            "schema":"forward53-diagnostics/v1", "width":width,"height":height,"components":components,
            "bits":bits,"style":style,"requested_workers":workers,"input_sha256":hash(&raw),
            "stream_sha256":hash(&stream),"stream_bytes":stream.len(),"total_ns":d.timings.total_ns,
            "forward_dwt_ns":d.timings.forward_dwt_ns,"workspace_prepare_ns":d.dwt_scratch_resize_ns,
            "workspace_drop_ns":d.dwt_scratch_drop_ns,"tier1_admitted_workers":d.execution.effective_workers,
            "level_backends":d.forward53.level_backends.map(|b|b.map(|b|format!("{b:?}"))),
            "panel_width":d.forward53.panel_width,"logical_slots":d.forward53.logical_slots,
            "workspace_capacity_bytes":d.forward53.workspace_capacity_bytes,
            "gather_lift_join_ns":d.forward53.gather_lift_join_ns,"scatter_join_ns":d.forward53.scatter_join_ns,
            "horizontal_join_ns":d.forward53.horizontal_join_ns,"peak_active_slots":d.forward53.peak_active_slots,
            "participating_workers":d.forward53.participating_workers,
            "scalar_axis_ns":d.dwt_vertical_gather_ns+d.dwt_vertical_lifting_ns+d.dwt_vertical_store_ns+d.dwt_horizontal_lifting_ns+d.dwt_horizontal_copy_ns
        })
    );
    Ok(())
}
