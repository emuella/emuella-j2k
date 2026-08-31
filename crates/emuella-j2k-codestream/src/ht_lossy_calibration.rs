//! Project-authored provisional irreversible HT calibration, not a public API.
//! Standards and the selected contract are recorded in docs/ht-lossy-calibration.md.
use super::*;
use emuella_j2k_container as container;
use sha2::{Digest, Sha256};

fn analyse(width: u32, height: u32, bits: u8, source: &[Vec<u16>]) -> Vec<Vec<f32>> {
    let mut planes = source
        .iter()
        .map(|p| {
            p.iter()
                .map(|&v| f32::from(v) - (1_u32 << (bits - 1)) as f32)
                .collect()
        })
        .collect::<Vec<_>>();
    ht_lossy::analyse(width, height, &mut planes).unwrap();
    planes
}
fn search(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[Vec<f32>],
    budget: usize,
) -> (Vec<u8>, u32, usize) {
    ht_lossy::search(width, height, bits, planes, budget).unwrap()
}

fn native(raw: &[u8]) -> Vec<Vec<u16>> {
    let decoded =
        ht_lossy::decode_owned_with_workspace(raw, &mut HtCodestreamDecodeWorkspace::new())
            .unwrap()
            .unwrap();
    decoded
        .components
        .iter()
        .map(|c| {
            if decoded.bits_per_sample == 8 {
                c.samples.iter().map(|&v| u16::from(v)).collect()
            } else {
                c.samples
                    .chunks_exact(2)
                    .map(|v| u16::from_le_bytes([v[0], v[1]]))
                    .collect()
            }
        })
        .collect()
}

fn wrapper(raw: &[u8], width: u32, height: u32, bits: u8, components: u16) -> Vec<u8> {
    let mut out = Vec::new();
    container::write_signature_box(&mut out).unwrap();
    container::write_file_type_box(&mut out, container::ContainerKind::Jph, 0, &[]).unwrap();
    let mut header = Vec::new();
    container::write_image_header_box(
        &mut header,
        container::ImageHeaderBox {
            width,
            height,
            components,
            bits_per_component: bits - 1,
            compression_type: 7,
            unknown_color_space: false,
            intellectual_property: false,
        },
    )
    .unwrap();
    container::write_color_specification_box(
        &mut header,
        container::ColorSpecificationBox {
            method: container::ColorSpecificationMethod::Enumerated,
            precedence: 0,
            approximation: 0,
            enumerated_color_space: Some(if components == 1 {
                container::EnumeratedColorSpace::Greyscale
            } else {
                container::EnumeratedColorSpace::SRgb
            }),
        },
    )
    .unwrap();
    container::write_jp2_header_box(&mut out, &header).unwrap();
    container::write_contiguous_codestream_box(&mut out, raw).unwrap();
    assert_eq!(&out[out.len() - raw.len()..], raw);
    out
}

pub(super) fn source(
    width: u32,
    height: u32,
    bits: u8,
    components: u16,
    pattern: u32,
) -> Vec<Vec<u16>> {
    let max = (1_u32 << bits) - 1;
    (0..u32::from(components))
        .map(|c| {
            (0..height)
                .flat_map(|y| {
                    (0..width).map(move |x| {
                        let mixed = x
                            .wrapping_mul(977)
                            .wrapping_add(y.wrapping_mul(1393))
                            .wrapping_add(c.wrapping_mul(9973))
                            .wrapping_add(x.wrapping_mul(y).wrapping_mul(41))
                            .wrapping_add((x ^ y).wrapping_mul(271))
                            .wrapping_add((x / 7).wrapping_mul((y / 5) + 1).wrapping_mul(613));
                        let mut noise =
                            (x + y * width + c * width * height + 1).wrapping_mul(0x9e3779b9);
                        noise ^= noise >> 16;
                        noise = noise.wrapping_mul(0x85ebca6b);
                        noise ^= noise >> 13;
                        noise = noise.wrapping_mul(0xc2b2ae35);
                        noise ^= noise >> 16;
                        let value = match pattern {
                            0 => mixed & max,
                            1 => noise & max,
                            2 => {
                                let base = if ((x / 17) + (y / 13) + c) % 2 == 0 {
                                    0
                                } else {
                                    max * 3 / 4
                                };
                                base + (noise & (max / 4))
                            }
                            3 => {
                                let base = if (x + y + c) % 2 == 0 { 0 } else { max / 2 };
                                base + (noise & (max / 2))
                            }
                            4 => {
                                if (x + y + c) % 2 == 0 {
                                    0
                                } else {
                                    max
                                }
                            }
                            5 => {
                                if x == width / 2 && y == height / 2 {
                                    max
                                } else {
                                    0
                                }
                            }
                            6 => max / 2,
                            8 => {
                                let negative = |v: u32| matches!(v % 8, 1 | 2 | 4 | 5);
                                if negative(x) == negative(y) { max } else { 0 }
                            }
                            _ => ((x + y + c * 17) * max / (width + height + 34)).min(max),
                        };
                        value as u16
                    })
                })
                .collect()
        })
        .collect()
}

fn interleaved(planes: &[Vec<u16>], bits: u8) -> Vec<u8> {
    let mut out = Vec::new();
    for pixel in 0..planes[0].len() {
        for p in planes {
            if bits == 8 {
                out.push(p[pixel] as u8);
            } else {
                out.extend_from_slice(&p[pixel].to_le_bytes());
            }
        }
    }
    out
}

#[test]
#[ignore = "bounded calibration; optional output must be directed to private registered scratch"]
fn measure_irreversible_ht_probe() {
    let env = |name: &str, default: u32| {
        std::env::var(name)
            .ok()
            .map(|s| s.parse::<u32>().unwrap())
            .unwrap_or(default)
    };
    let width = env("EMUELLA_HT_CALIBRATION_WIDTH", 257);
    let height = env("EMUELLA_HT_CALIBRATION_HEIGHT", 193);
    let selected_pattern = std::env::var("EMUELLA_HT_CALIBRATION_PATTERN")
        .ok()
        .map(|s| s.parse::<u32>().unwrap());
    let selected_bits = std::env::var("EMUELLA_HT_CALIBRATION_BITS")
        .ok()
        .map(|s| s.parse::<u8>().unwrap());
    let selected_components = std::env::var("EMUELLA_HT_CALIBRATION_COMPONENTS")
        .ok()
        .map(|s| s.parse::<u16>().unwrap());
    let output = std::env::var_os("EMUELLA_HT_CALIBRATION_OUTPUT").map(std::path::PathBuf::from);
    if let Some(path) = &output {
        assert!(path.is_dir());
    }
    let first_only = std::env::var_os("EMUELLA_HT_CALIBRATION_FIRST_ONLY").is_some();
    for pattern in 0..9 {
        if selected_pattern.is_some_and(|p| p != pattern) {
            continue;
        }
        for bits in [8, 16] {
            if selected_bits.is_some_and(|b| b != bits) {
                continue;
            }
            for components in [1, 3] {
                if selected_components.is_some_and(|c| c != components) {
                    continue;
                }
                let source = source(width, height, bits, components, pattern);
                let planes = analyse(width, height, bits, &source);
                let mut previous = u128::MAX;
                for rate in [1_u32, 2, 4] {
                    let budget = (width * height * rate / 8) as usize;
                    let now = std::time::Instant::now();
                    let (raw, coarseness, attempts) = search(width, height, bits, &planes, budget);
                    let millis = now.elapsed().as_millis();
                    let decoded = native(&raw);
                    let sse: u128 = source
                        .iter()
                        .flatten()
                        .zip(decoded.iter().flatten())
                        .map(|(&a, &b)| u128::from(a.abs_diff(b)).pow(2))
                        .sum();
                    let peak = source
                        .iter()
                        .flatten()
                        .zip(decoded.iter().flatten())
                        .map(|(&a, &b)| a.abs_diff(b))
                        .max()
                        .unwrap();
                    let max = (1_u32 << bits) - 1;
                    let nmse = sse as f64
                        / (f64::from(width * height * u32::from(components))
                            * f64::from(max).powi(2));
                    let monotonic = sse <= previous;
                    assert!(attempts <= 17);
                    if width == 257 && height == 193 && pattern < 4 {
                        assert!(raw.len() <= budget);
                        assert!(budget - raw.len() <= 32_usize.max(budget.div_ceil(500)));
                        let ceiling = match rate {
                            1 => 125,
                            2 => 60,
                            4 => 40,
                            _ => unreachable!(),
                        };
                        assert!(
                            sse * 1000
                                <= u128::from(width * height * u32::from(components))
                                    * u128::from(max).pow(2)
                                    * ceiling
                        );
                        assert!(monotonic);
                    }
                    previous = sse;
                    assert_eq!(search(width, height, bits, &planes, budget).0, raw);
                    let wrapped = wrapper(&raw, width, height, bits, components);
                    let input = interleaved(&source, bits);
                    let native_bytes = interleaved(&decoded, bits);
                    let name = format!("p{pattern}-u{bits}-c{components}-r{rate}");
                    println!(
                        "HTCAL,{name},{budget},{},{},{coarseness},{attempts},{sse},{nmse:.12},{peak},{monotonic},{millis},{:x},{:x},{:x}",
                        raw.len(),
                        budget.saturating_sub(raw.len()),
                        Sha256::digest(&input),
                        Sha256::digest(&raw),
                        Sha256::digest(&native_bytes)
                    );
                    if let Some(path) = &output {
                        for (extension, data) in [
                            ("j2c", &raw),
                            ("jph", &wrapped),
                            ("input", &input),
                            ("native", &native_bytes),
                        ] {
                            std::fs::write(path.join(format!("{name}.{extension}")), data).unwrap();
                        }
                    }
                    if first_only {
                        return;
                    }
                }
            }
        }
    }
}
