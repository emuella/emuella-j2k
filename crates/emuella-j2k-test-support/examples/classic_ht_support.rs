//! Bounded support probe; source pixels remain in memory and only facts are emitted.
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
    if args.len() != 5 {
        return Err("usage: classic_ht_support RAW WIDTH HEIGHT COMPONENTS".into());
    }
    let width: usize = args[2].parse()?;
    let height: usize = args[3].parse()?;
    let components: usize = args[4].parse()?;
    if width < 256 || height < 256 || !matches!(components, 1 | 3) {
        return Err("requires full unsigned16 PAN/RGB input at least 256 square".into());
    }
    let source = std::fs::read(&args[1])?;
    if source.len()
        != width
            .checked_mul(height)
            .and_then(|n| n.checked_mul(components))
            .and_then(|n| n.checked_mul(2))
            .ok_or("source shape overflow")?
    {
        return Err("source length differs".into());
    }
    let raw: Vec<u8> = (0..256)
        .flat_map(|y| {
            source[y * width * components * 2..(y * width + 256) * components * 2]
                .iter()
                .copied()
        })
        .collect();
    let info = ImageInfo::new(
        256,
        256,
        components as u16,
        SampleFormat::U16_LE,
        if components == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        ComponentLayout::Interleaved,
    )?;
    let view = ImageView::Interleaved {
        info: &info,
        samples: &raw,
        stride_bytes: 256 * components * 2,
    };
    let options = EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let limits = LosslessEncodeLimits {
        max_output_bytes: 64 * 1024 * 1024,
        max_working_bytes: 768 * 1024 * 1024,
    };
    let bypass = encode_lossless_bypass_with_limits(view, &options, &limits)?;
    let decoded = decode(
        &bypass,
        &DecodeOptions {
            mode: DecodeMode::Components,
            target_layout: ComponentLayout::Interleaved,
            ..Default::default()
        },
    )?;
    if decoded.data != ImageData::Interleaved(raw.clone()) {
        return Err("bypass round trip differs".into());
    }
    let ht = encode_htj2k(
        view,
        &Htj2kEncodeOptions {
            decomposition_levels: 2,
        },
    );
    let d1 = lossless_bypass_encode_requirements(
        &info,
        &EncodeOptions {
            decomposition_levels: 1,
            ..options
        },
        &limits,
    );
    if !matches!(ht, Err(J2kError::Unsupported { .. }))
        || !matches!(d1, Err(J2kError::Unsupported { .. }))
    {
        return Err("support changed; reconsider the bounded probe".into());
    }
    println!(
        "{}",
        json!({"source_sha256":hash(&source),
        "window_sha256":hash(&raw),"source_width":width,"source_height":height,
        "components":components,"bits":16,"window":[0,0,256,256],"bypass_d2_exact":true,
        "bypass_d2_bytes":bypass.len(),"ht_d2":format!("{:?}",ht.unwrap_err()),
        "bypass_d1":format!("{:?}",d1.unwrap_err()),"matched_profile":false,
        "conclusion":"No common admitted decomposition depth; no matched timing or codec ranking"})
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
