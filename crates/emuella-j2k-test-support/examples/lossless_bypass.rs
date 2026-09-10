//! Native selective-bypass exploration driver using project-authored inputs.
use emuella_j2k_codestream as cs;
use sha2::{Digest, Sha256};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 9 {
        return Err("usage: lossless_bypass RAW CODESTREAM WIDTH HEIGHT COMPONENTS BITS planar|interleaved 0|1|decode".into());
    }
    let width: u32 = args[3].parse()?;
    let height: u32 = args[4].parse()?;
    let components: usize = args[5].parse()?;
    let bits: u8 = args[6].parse()?;
    let style: u8 = if args[8] == "decode" {
        2
    } else {
        args[8].parse()?
    };
    if !matches!(bits, 8 | 16) || !matches!(components, 1 | 3 | 8) || style > 2 {
        return Err("unsupported probe sample model or style".into());
    }
    let planar = match args[7].as_str() {
        "planar" => true,
        "interleaved" => false,
        _ => return Err("invalid layout".into()),
    };
    let raw = std::fs::read(&args[1])?;
    let sample_bytes = usize::from(bits / 8);
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or("size overflow")?;
    let plane_bytes = pixels.checked_mul(sample_bytes).ok_or("size overflow")?;
    if raw.len() != plane_bytes.checked_mul(components).ok_or("size overflow")? {
        return Err("raw size mismatch".into());
    }
    let planes: Vec<_> = (0..components)
        .map(|component| cs::LosslessD2Plane {
            samples: &raw[if planar {
                component * plane_bytes
            } else {
                component * sample_bytes
            }..],
            stride_bytes: width as usize * sample_bytes * if planar { 1 } else { components },
            sample_step_bytes: sample_bytes * if planar { 1 } else { components },
        })
        .collect();
    let encoded = if style == 0 {
        cs::encode_lossless_d2(
            width,
            height,
            bits,
            &planes,
            cs::LosslessEncodeLimits::default(),
        )?
    } else if style == 1 {
        cs::encode_lossless_d2_bypass_test_fixture(
            width,
            height,
            bits,
            &planes,
            cs::LosslessEncodeLimits::default(),
        )?
    } else {
        std::fs::read(&args[2])?
    };
    let decoded = cs::decode_baseline_owned_components(&encoded)?;
    if decoded.width != width || decoded.height != height || decoded.components.len() != components
    {
        return Err("decoded shape mismatch".into());
    }
    for (component, plane) in decoded.components.iter().enumerate() {
        if plane.samples.len() != plane_bytes {
            return Err("decoded plane size mismatch".into());
        }
        for i in 0..pixels {
            let offset = if planar {
                component * plane_bytes + i * sample_bytes
            } else {
                (i * components + component) * sample_bytes
            };
            if plane.samples[i * sample_bytes..(i + 1) * sample_bytes]
                != raw[offset..offset + sample_bytes]
            {
                return Err("native samples differ".into());
            }
        }
    }
    if style != 2 {
        std::fs::write(&args[2], &encoded)?;
    }
    println!(
        "{}",
        serde_json::json!({
            "style": style, "width": width, "height": height, "components": components,
            "bits": bits, "rct": components == 3, "native_exact": true,
            "stream_bytes": encoded.len(),
            "input_sha256": format!("{:x}", Sha256::digest(&raw)),
            "stream_sha256": format!("{:x}", Sha256::digest(&encoded)),
            "evidence_role": "synthetic correctness only; no performance evidence",
        })
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
