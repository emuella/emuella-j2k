//! Project-authored provisional irreversible HT calibration, not a public API.
//! Standards and the selected contract are recorded in docs/ht-lossy-calibration.md.
use super::*;
use emuella_j2k_container as container;
use sha2::{Digest, Sha256};

#[allow(clippy::too_many_arguments)]
fn candidate(
    width: u32,
    height: u32,
    bits_per_sample: u8,
    decomposition_levels: u8,
    transformed_planes: &[Vec<f32>],
    multiple_component_transform: bool,
    specs: &[DecompSubbandSpec],
    coarseness: u32,
) -> Result<Option<Vec<u8>>> {
    let octave = coarseness / 2048;
    let mantissa = u16::try_from(coarseness % 2048).map_err(|_| CodestreamError::SizeOverflow)?;
    let base_exponent = 31_u8
        .checked_sub(u8::try_from(octave).map_err(|_| CodestreamError::SizeOverflow)?)
        .ok_or(CodestreamError::SizeOverflow)?;
    let qcd_steps = specs
        .iter()
        .map(|spec| {
            let gain = match spec.kind {
                PacketSubbandKind::LowLow => 0,
                PacketSubbandKind::HighLow | PacketSubbandKind::LowHigh => 1,
                PacketSubbandKind::HighHigh => 2,
            };
            transform::IrreversibleQuantizationStep::new(
                base_exponent
                    .checked_add(gain)
                    .ok_or(CodestreamError::SizeOverflow)?,
                mantissa,
            )
            .map_err(|_| CodestreamError::SizeOverflow)
        })
        .collect::<Result<Vec<_>>>()?;
    for step in &qcd_steps {
        let decoder_available_bitplanes = IRREVERSIBLE_QCD_GUARD_BITS
            .checked_add(step.exponent)
            .and_then(|value| value.checked_sub(1))
            .ok_or(CodestreamError::SizeOverflow)?;
        if decoder_available_bitplanes > CLASSIC_COMPONENT_MAX_MAGNITUDE_BITPLANES {
            return Ok(None);
        }
    }
    let available_bitplanes = qcd_steps
        .iter()
        .map(|step| {
            IRREVERSIBLE_QCD_GUARD_BITS
                .checked_add(step.exponent)
                .and_then(|value| value.checked_sub(1))
                .ok_or(CodestreamError::SizeOverflow)
        })
        .collect::<Result<Vec<_>>>()?;
    let mut quantized_planes = transformed_planes
        .iter()
        .map(|plane| alloc::vec![0_i32; plane.len()])
        .collect::<Vec<_>>();
    let stride = usize::try_from(width).map_err(|_| CodestreamError::SizeOverflow)?;
    for (source, quantized) in transformed_planes.iter().zip(&mut quantized_planes) {
        for (spec, step) in specs.iter().zip(&qcd_steps) {
            let gain = match spec.kind {
                PacketSubbandKind::LowLow => 0,
                PacketSubbandKind::HighLow | PacketSubbandKind::LowHigh => 1,
                PacketSubbandKind::HighHigh => 2,
            };
            let delta = step
                .delta(bits_per_sample, gain)
                .map_err(|_| CodestreamError::SizeOverflow)?;
            for y in 0..usize::try_from(spec.height).map_err(|_| CodestreamError::SizeOverflow)? {
                let row = usize::try_from(spec.y)
                    .map_err(|_| CodestreamError::SizeOverflow)?
                    .checked_add(y)
                    .and_then(|value| value.checked_mul(stride))
                    .and_then(|value| value.checked_add(usize::try_from(spec.x).ok()?))
                    .ok_or(CodestreamError::SizeOverflow)?;
                let width =
                    usize::try_from(spec.width).map_err(|_| CodestreamError::SizeOverflow)?;
                for x in 0..width {
                    let value = source[row + x];
                    let scaled_magnitude = value.abs() / delta;
                    if scaled_magnitude > i32::MAX as f32 {
                        return Err(CodestreamError::SizeOverflow);
                    }
                    let magnitude = scaled_magnitude as i32;
                    if magnitude >= (1 << 17) {
                        return Ok(None);
                    }
                    quantized[row + x] = if value.is_sign_negative() {
                        -magnitude
                    } else {
                        magnitude
                    };
                }
            }
        }
    }

    let plane_refs = quantized_planes
        .iter()
        .map(Vec::as_slice)
        .collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut component_subbands = Vec::with_capacity(plane_refs.len());
    for plane in &plane_refs {
        let mut subbands = Vec::new();
        for (spec, depth) in specs.iter().zip(&available_bitplanes) {
            subbands.push(encode_ht_decomp_subband(
                width,
                plane,
                *spec,
                *depth,
                &mut segments,
            )?);
        }
        component_subbands.push(subbands);
    }
    let mut packet = Vec::with_capacity(native_decomp_packet_capacity_hint(
        &component_subbands,
        &segments,
    )?);
    write_native_decomp_packets(
        &mut packet,
        decomposition_levels,
        &component_subbands,
        &segments,
    )?;
    let mut codestream = Vec::new();
    write_native_irreversible_main_header(
        &mut codestream,
        width,
        height,
        bits_per_sample,
        u16::try_from(plane_refs.len()).map_err(|_| CodestreamError::SizeOverflow)?,
        multiple_component_transform,
        decomposition_levels,
        &qcd_steps,
    )?;
    // Test-only adaptation: ordinary Part 1 quantisation with HT block signalling.
    codestream[6..8].copy_from_slice(&0x4000_u16.to_be_bytes());
    let cod = find_marker(&codestream, 0, Marker::Cod).unwrap();
    codestream[cod + 12] = 0x40;
    codestream.splice(cod..cod, [0xff, 0x50, 0, 8, 0, 2, 0, 0, 0, 0x2a]);
    write_tile_part(&mut codestream, 0, &packet, true)?;
    Ok(Some(codestream))
}

fn analyse(width: u32, height: u32, bits: u8, source: &[Vec<u16>]) -> Vec<Vec<f32>> {
    let mut planes = source
        .iter()
        .map(|p| {
            p.iter()
                .map(|&v| f32::from(v) - (1_u32 << (bits - 1)) as f32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for plane in &mut planes {
        for level in 0..2 {
            let (w, h) = resolution_dimensions(width, height, 2, 2 - level).unwrap();
            let config = transform::Irreversible97Config {
                width: w as usize,
                height: h as usize,
                stride: width as usize,
                edges: transform::Irreversible97Edges::from_tile_origin(
                    0, 0, w as usize, h as usize,
                ),
            };
            let mut scratch = vec![0.; config.scratch_len()];
            transform::forward_irreversible_9_7(plane, config, &mut scratch).unwrap();
        }
    }
    planes
}

fn search(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[Vec<f32>],
    budget: usize,
) -> (Vec<u8>, u32, usize) {
    let specs = decomp_subband_specs(width, height, 2).unwrap();
    let mut lower = IRREVERSIBLE_COARSENESS_MIN;
    let mut upper = IRREVERSIBLE_COARSENESS_MAX;
    let mut best = candidate(width, height, bits, 2, planes, false, &specs, upper)
        .unwrap()
        .unwrap();
    let mut selected = upper;
    let mut attempts = 1;
    if best.len() > budget {
        return (best, selected, attempts);
    }
    while lower <= upper {
        let midpoint = lower + (upper - lower) / 2;
        attempts += 1;
        let Some(encoded) =
            candidate(width, height, bits, 2, planes, false, &specs, midpoint).unwrap()
        else {
            lower = midpoint + 1;
            continue;
        };
        if encoded.len() <= budget {
            if encoded.len() > best.len() {
                best = encoded;
                selected = midpoint;
            }
            upper = midpoint - 1;
        } else {
            lower = midpoint + 1;
        }
    }
    (best, selected, attempts)
}

fn native(raw: &[u8]) -> Vec<Vec<u16>> {
    let parsed = parse(raw).unwrap();
    validate_part15_packet_signalling(raw, &parsed).unwrap();
    let style = uniform_effective_coding_style(&parsed).unwrap();
    assert_eq!(style.decomposition_levels, 2);
    assert_eq!(style.transform, WaveletTransform::Irreversible97);
    assert!(!style.multiple_component_transform);
    let candidate = ht_decode_candidate_with_transform_permission(&parsed, true)
        .unwrap()
        .unwrap();
    let (rect, payload) = single_part1_profile_tile(raw, &parsed).unwrap();
    let contributions = parse_default_precinct_lrcp_packets(raw, &parsed, rect, payload).unwrap();
    classify_htonly_native_packet_mechanisms(&contributions).unwrap();
    let mut result = Vec::new();
    for component in 0..parsed.siz.component_count() {
        // The test admits its own exact encoder shape. Production admission remains unchanged.
        let prepared = PreparedHtj2kReducedComponentDecode {
            input: raw,
            codestream: parsed.clone(),
            candidate,
            coding_style: style,
            reconstruction: Htj2kReducedComponentReconstruction::Irreversible,
            tile_rect: rect,
            contributions: contributions.clone(),
            request: Htj2kReducedComponentDecodeRequest {
                component_index: component,
                discard_levels: 0,
            },
            output_width: rect.width,
            output_height: rect.height,
        };
        let decoded = decode_prepared_htj2k_reduced_component_owned_with_workspace(
            &prepared,
            &mut HtCodestreamDecodeWorkspace::new(),
        )
        .unwrap();
        let bytes = &decoded.components[0].samples;
        result.push(if decoded.bits_per_sample == 8 {
            bytes.iter().map(|&v| u16::from(v)).collect()
        } else {
            bytes
                .chunks_exact(2)
                .map(|v| u16::from_le_bytes([v[0], v[1]]))
                .collect()
        });
    }
    result
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

fn source(width: u32, height: u32, bits: u8, components: u16, pattern: u32) -> Vec<Vec<u16>> {
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
    let width = 257;
    let height = 193;
    let output = std::env::var_os("EMUELLA_HT_CALIBRATION_OUTPUT").map(std::path::PathBuf::from);
    if let Some(path) = &output {
        assert!(path.is_dir());
    }
    let first_only = std::env::var_os("EMUELLA_HT_CALIBRATION_FIRST_ONLY").is_some();
    for pattern in 0..8 {
        for bits in [8, 16] {
            for components in [1, 3] {
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
