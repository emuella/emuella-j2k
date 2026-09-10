//! One fresh-process native operation for provisional style-0/style-1 comparison.
//! The calling harness owns source/build identity and authorised input locations.
use emuella_j2k as api;
use emuella_j2k_codestream as cs;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path, time::Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn string<'a>(v: &'a Value, name: &str) -> Result<&'a str> {
    v[name]
        .as_str()
        .ok_or_else(|| format!("missing {name}").into())
}
fn number(v: &Value, name: &str) -> Result<u64> {
    v[name]
        .as_u64()
        .ok_or_else(|| format!("missing {name}").into())
}

fn complete_raw_syntax(bytes: &[u8]) -> Result<usize> {
    if bytes.last() == Some(&0xff) || bytes.windows(2).any(|p| p[0] == 0xff && p[1] & 0x80 != 0) {
        return Err("generated completed raw segment violates encoder byte syntax".into());
    }
    Ok(bytes.windows(2).filter(|p| p[0] == 0xff).count())
}

fn profile(
    bytes: &[u8],
    width: u32,
    height: u32,
    components: usize,
    bits: u8,
    style: u8,
) -> Result<Value> {
    // Inspect the actual complete main header, excluding all effective overrides.
    let parsed = cs::parse(bytes)?;
    if parsed.siz.reference_grid_width != width
        || parsed.siz.reference_grid_height != height
        || parsed.siz.image_origin_x != 0
        || parsed.siz.image_origin_y != 0
        || parsed.siz.tile_origin_x != 0
        || parsed.siz.tile_origin_y != 0
        || parsed.siz.tile_width != width
        || parsed.siz.tile_height != height
        || parsed.siz.components.len() != components
        || parsed.siz.components.iter().any(|c| {
            c.bits_per_sample != bits
                || c.signed
                || c.horizontal_separation != 1
                || c.vertical_separation != 1
        })
        || parsed.markers.iter().any(|m| {
            matches!(
                m.marker,
                cs::Marker::Coc | cs::Marker::Qcc | cs::Marker::Rgn | cs::Marker::Poc
            )
        })
        || parsed
            .markers
            .iter()
            .filter(|m| m.marker == cs::Marker::Cod)
            .count()
            != 1
        || parsed
            .markers
            .iter()
            .filter(|m| m.marker == cs::Marker::Qcd)
            .count()
            != 1
        || parsed
            .markers
            .iter()
            .filter(|m| m.marker == cs::Marker::Sot)
            .count()
            != 1
    {
        return Err("stream exceeds the frozen D2 contrast".into());
    }
    // Marker field positions are fixed for the required default-precinct COD.
    let mut offset = 2usize;
    let mut found = false;
    while bytes.get(offset..offset + 2) != Some(&[0xff, 0x90]) {
        let marker = bytes.get(offset..offset + 4).ok_or("truncated marker")?;
        let len = usize::from(u16::from_be_bytes([marker[2], marker[3]]));
        let end = offset.checked_add(2 + len).ok_or("marker overflow")?;
        let segment = bytes.get(offset..end).ok_or("truncated marker segment")?;
        if marker[..2] == [0xff, 0x52] {
            if segment
                != [
                    0xff,
                    0x52,
                    0,
                    12,
                    0,
                    0,
                    0,
                    1,
                    u8::from(components == 3),
                    2,
                    4,
                    4,
                    style,
                    1,
                ]
            {
                return Err("COD differs from the frozen D2 contrast".into());
            }
            found = true;
        }
        if len < 2 {
            return Err("invalid marker size".into());
        }
        offset = end;
    }
    if !found {
        return Err("missing COD".into());
    }
    let tile = parsed.tiles.first().ok_or("missing tile")?;
    let start = tile.payload_offset.ok_or("missing payload offset")?;
    let end = start
        .checked_add(tile.payload_len.ok_or("missing payload length")?)
        .ok_or("payload overflow")?;
    let payload = bytes.get(start..end).ok_or("truncated payload")?;
    let blocks = cs::parse_default_precinct_lrcp_packets(
        bytes,
        &parsed,
        cs::TileRect {
            tile_index: 0,
            tile_x: 0,
            tile_y: 0,
            x: 0,
            y: 0,
            width,
            height,
        },
        payload,
    )?;
    let mut raw_segments = 0usize;
    let mut stuffed_raw_bytes = 0usize;
    for block in &blocks {
        if block.ht_coded || !block.segment_ranges.is_empty() || block.coding_passes % 3 != 1 {
            return Err("generated block does not contain a complete classic contribution".into());
        }
        let end = block
            .payload_offset
            .checked_add(block.codeword_len)
            .ok_or("block overflow")?;
        let codeword = payload
            .get(block.payload_offset..end)
            .ok_or("truncated block")?;
        if codeword.last() == Some(&0xff) {
            return Err("generated block contribution ends FF".into());
        }
        if style == 1 {
            let mut pass = 0u16;
            let mut offset = 0usize;
            for segment in &block.coding_segments {
                if pass >= block.coding_passes {
                    return Err("extra segment".into());
                }
                let capacity = if pass < 10 {
                    10 - pass
                } else if (pass - 10).is_multiple_of(3) {
                    2
                } else {
                    1
                };
                if segment.coding_passes != capacity.min(block.coding_passes - pass) {
                    return Err("generated segment topology differs".into());
                }
                let end = offset
                    .checked_add(segment.byte_len)
                    .ok_or("segment overflow")?;
                let segment_bytes = codeword.get(offset..end).ok_or("truncated segment")?;
                if pass >= 10 && !pass.is_multiple_of(3) {
                    if segment.coding_passes != 2 {
                        return Err("incomplete generated raw pair".into());
                    }
                    stuffed_raw_bytes += complete_raw_syntax(segment_bytes)?;
                    raw_segments += 1;
                }
                offset = end;
                pass = pass
                    .checked_add(segment.coding_passes)
                    .ok_or("pass overflow")?;
            }
            if offset != codeword.len() || pass != block.coding_passes {
                return Err("segment totals differ".into());
            }
        }
    }
    Ok(
        json!({"complete_contributions": blocks.len(), "raw_segments": raw_segments,
        "stuffed_raw_bytes": stuffed_raw_bytes, "encoder_syntax_exact": true,
        "scope": "generated complete single-layer profile; not a decoder rejection policy"}),
    )
}

fn verify(
    decoded: &cs::DecodedImage,
    raw: &[u8],
    width: u32,
    height: u32,
    components: usize,
    bits: u8,
    planar: bool,
) -> Result<()> {
    let bytes = usize::from(bits / 8);
    let pixels = width as usize * height as usize;
    let plane_bytes = pixels * bytes;
    if decoded.width != width
        || decoded.height != height
        || decoded.bits_per_sample != bits
        || decoded.signed
        || decoded.components.len() != components
    {
        return Err("decoded shape differs".into());
    }
    for (c, plane) in decoded.components.iter().enumerate() {
        if plane.samples.len() != plane_bytes {
            return Err("decoded plane size differs".into());
        }
        for i in 0..pixels {
            let offset = if planar {
                c * plane_bytes + i * bytes
            } else {
                (i * components + c) * bytes
            };
            if plane.samples[i * bytes..(i + 1) * bytes] != raw[offset..offset + bytes] {
                return Err("native samples differ".into());
            }
        }
    }
    Ok(())
}

fn run() -> Result<Value> {
    if !cfg!(feature = "parallel") || cfg!(feature = "simd") {
        return Err("this contrast requires parallel enabled and SIMD disabled".into());
    }
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 2 {
        return Err("usage: lossless_bypass_batch REQUEST.json".into());
    }
    let request: Value = serde_json::from_slice(&fs::read(&args[1])?)?;
    let width = u32::try_from(number(&request, "width")?)?;
    let height = u32::try_from(number(&request, "height")?)?;
    let components = usize::try_from(number(&request, "components")?)?;
    let bits = u8::try_from(number(&request, "bits")?)?;
    let style = u8::try_from(number(&request, "style")?)?;
    let operation = string(&request, "operation")?;
    let boundary = request["boundary"]
        .as_str()
        .unwrap_or("native_codestream_operation");
    let facade = match boundary {
        "native_codestream_operation" => false,
        "facade_operation" => true,
        _ => return Err("unsupported operation boundary".into()),
    };
    let workers = usize::try_from(request["workers"].as_u64().unwrap_or(1))?;
    if !matches!(workers, 1 | 2 | 4 | 8) {
        return Err("unsupported pool width".into());
    }
    if !matches!(operation, "prepare" | "encode" | "decode")
        || style > 1
        || !matches!(components, 1 | 3 | 8)
        || !matches!(bits, 8 | 16)
    {
        return Err("unsupported contrast request".into());
    }
    let planar = match string(&request, "layout")? {
        "planar" => true,
        "interleaved" => false,
        _ => return Err("invalid layout".into()),
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()?;
    pool.install(|| {
        let requirements = if style == 0 {
            cs::lossless_d2_requirements
        } else {
            cs::lossless_d2_bypass_requirements
        };
        requirements(
            width,
            height,
            components as u16,
            cs::LosslessEncodeLimits::default(),
        )
    })?;
    let sample_bytes = usize::from(bits / 8);
    let pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or("size overflow")?;
    let plane_bytes = pixels.checked_mul(sample_bytes).ok_or("size overflow")?;
    let raw = fs::read(string(&request, "raw_path")?)?;
    let raw_sha = hash(&raw);
    if raw_sha != string(&request, "raw_sha256")?
        || raw.len() != plane_bytes.checked_mul(components).ok_or("size overflow")?
    {
        return Err("raw identity or length differs".into());
    }
    let stream_path = Path::new(string(&request, "stream_path")?);
    let stream = if operation == "prepare" {
        Vec::new()
    } else {
        fs::read(stream_path)?
    };
    let expected_hash = if operation == "prepare" {
        String::new()
    } else {
        hash(&stream)
    };
    let source_syntax = if operation != "prepare" {
        if expected_hash != string(&request, "stream_sha256")? {
            return Err("stream identity differs".into());
        }
        Some(profile(&stream, width, height, components, bits, style)?)
    } else {
        None
    };
    let planes: Vec<_> = (0..components)
        .map(|c| cs::LosslessD2Plane {
            samples: &raw[if planar {
                c * plane_bytes
            } else {
                c * sample_bytes
            }..],
            stride_bytes: width as usize * sample_bytes * if planar { 1 } else { components },
            sample_step_bytes: sample_bytes * if planar { 1 } else { components },
        })
        .collect();
    let layout = if planar {
        api::ComponentLayout::Planar
    } else {
        api::ComponentLayout::Interleaved
    };
    let info = api::ImageInfo::new(
        width,
        height,
        components as u16,
        if bits == 8 {
            api::SampleFormat::U8
        } else {
            api::SampleFormat::U16_LE
        },
        match components {
            1 => api::ColorModel::Grayscale,
            3 => api::ColorModel::Rgb,
            _ => api::ColorModel::Unknown,
        },
        layout,
    )?;
    let options = api::EncodeOptions {
        format: api::OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let api_planes: Vec<_> = if planar {
        raw.chunks_exact(plane_bytes)
            .map(|samples| {
                api::Plane::new(
                    samples,
                    width,
                    height,
                    width as usize * sample_bytes,
                    info.sample_format,
                )
            })
            .collect::<std::result::Result<_, _>>()?
    } else {
        Vec::new()
    };
    let view = if planar {
        api::ImageView::Planar {
            info: &info,
            planes: &api_planes,
        }
    } else {
        api::ImageView::Interleaved {
            info: &info,
            samples: &raw,
            stride_bytes: width as usize * sample_bytes * components,
        }
    };
    let decode_options = api::DecodeOptions {
        mode: api::DecodeMode::Components,
        target_layout: layout,
        ..Default::default()
    };
    let verify_api = |decoded: &api::Image| -> Result<()> {
        if decoded.info.width != width
            || decoded.info.height != height
            || decoded.info.components as usize != components
            || decoded.info.sample_format != info.sample_format
        {
            return Err("facade decoded shape differs".into());
        }
        let exact = match &decoded.data {
            api::ImageData::Interleaved(samples) => !planar && samples == &raw,
            api::ImageData::Planes(planes) => {
                planar
                    && planes.len() == components
                    && planes
                        .iter()
                        .zip(raw.chunks_exact(plane_bytes))
                        .all(|(p, expected)| p == expected)
            }
        };
        if !exact {
            return Err("facade decoded samples differ".into());
        }
        Ok(())
    };
    let encode = || -> Result<Vec<u8>> {
        if facade {
            let encode = if style == 0 {
                api::encode_with_limits
            } else {
                api::encode_lossless_bypass_with_limits
            };
            Ok(encode(
                view,
                &options,
                &api::LosslessEncodeLimits::default(),
            )?)
        } else {
            let encode = if style == 0 {
                cs::encode_lossless_d2
            } else {
                cs::encode_lossless_d2_bypass
            };
            Ok(encode(
                width,
                height,
                bits,
                &planes,
                cs::LosslessEncodeLimits::default(),
            )?)
        }
    };
    let (elapsed, actual_hash, stream_bytes, syntax) = pool.install(|| -> Result<_> {
        if rayon::current_num_threads() != workers {
            return Err("unexpected pool width".into());
        }
        if operation == "decode" {
            let elapsed = if facade {
                let start = Instant::now();
                let decoded = api::decode(&stream, &decode_options)?;
                let elapsed = u64::try_from(start.elapsed().as_nanos())?;
                verify_api(&decoded)?;
                elapsed
            } else {
                let start = Instant::now();
                let decoded = cs::decode_baseline_owned_components(&stream)?;
                let elapsed = u64::try_from(start.elapsed().as_nanos())?;
                verify(&decoded, &raw, width, height, components, bits, planar)?;
                elapsed
            };
            Ok((
                elapsed,
                expected_hash.clone(),
                stream.len(),
                source_syntax.clone().ok_or("missing source syntax")?,
            ))
        } else {
            let start = Instant::now();
            let encoded = encode()?;
            let elapsed = u64::try_from(start.elapsed().as_nanos())?;
            let syntax = profile(&encoded, width, height, components, bits, style)?;
            let actual_hash = hash(&encoded);
            if operation == "encode" && actual_hash != expected_hash {
                return Err("encoded stream changed".into());
            }
            if facade {
                verify_api(&api::decode(&encoded, &decode_options)?)?;
            } else {
                let decoded = cs::decode_baseline_owned_components(&encoded)?;
                verify(&decoded, &raw, width, height, components, bits, planar)?;
            }
            if operation == "prepare" {
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(stream_path)?
                    .write_all(&encoded)?;
            }
            Ok((elapsed, actual_hash, encoded.len(), syntax))
        }
    })?;
    Ok(json!({
        "schema_version": 1, "contrast": "native_d2_style0_vs_style1", "operation": operation,
        "case_id": string(&request, "case_id")?, "round": number(&request, "round")?,
        "style": style, "width": width, "height": height, "components": components, "bits": bits,
        "layout": string(&request, "layout")?, "rct": components == 3, "native_exact": true,
        "raw_sha256": raw_sha, "stream_sha256": actual_hash, "complete_stream_bytes": stream_bytes,
        "encoder_syntax": syntax,
        "binary_sha256": hash(&fs::read(std::env::current_exe()?)?), "parallel": true, "simd": false, "workers": workers,
        "samples_ns": if operation == "prepare" { Vec::<u64>::new() } else { vec![elapsed] },
        "warmup": 0, "samples_per_batch": 1, "boundary": boundary,
        "decode_output": if facade { "owned_facade_image_in_requested_layout" } else { "owned_native_component_planes" }, "input_policy": "loaded_and_hash_checked_before_clock",
        "context_policy": "fresh_process_per_batch", "verification": "outside_clock_every_sample",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_constant_streams_have_complete_encoder_syntax() {
        let width = 133u32;
        let height = 129u32;
        for (components, bits) in [(1usize, 8u8), (1, 16), (3, 8), (3, 16), (8, 16)] {
            // Zero samples exercise constant low-pass blocks; the unsigned
            // midpoint also makes every transformed coefficient zero.
            for value in [0u16, 1u16 << (bits - 1)] {
                let sample = value.to_le_bytes();
                let raw: Vec<_> = (0..width as usize * height as usize * components)
                    .flat_map(|_| sample[..usize::from(bits / 8)].iter().copied())
                    .collect();
                let plane_bytes = width as usize * height as usize * usize::from(bits / 8);
                let planes: Vec<_> = (0..components)
                    .map(|component| cs::LosslessD2Plane {
                        samples: &raw[component * plane_bytes..],
                        stride_bytes: width as usize * usize::from(bits / 8),
                        sample_step_bytes: usize::from(bits / 8),
                    })
                    .collect();
                for style in [0, 1] {
                    let encode = if style == 0 {
                        cs::encode_lossless_d2
                    } else {
                        cs::encode_lossless_d2_bypass_test_fixture
                    };
                    let encoded = encode(
                        width,
                        height,
                        bits,
                        &planes,
                        cs::LosslessEncodeLimits::default(),
                    )
                    .unwrap();
                    let observed =
                        profile(&encoded, width, height, components, bits, style).unwrap();
                    assert_eq!(observed["encoder_syntax_exact"], true);
                    let decoded = cs::decode_baseline_owned_components(&encoded).unwrap();
                    verify(&decoded, &raw, width, height, components, bits, true).unwrap();
                }
            }
        }
    }

    #[test]
    fn generated_raw_syntax_distinguishes_stuffing_from_padding() {
        assert_eq!(complete_raw_syntax(&[0xff, 0x7f, 0x80]).unwrap(), 1);
        assert_eq!(complete_raw_syntax(&[0x80]).unwrap(), 0);
        assert_eq!(complete_raw_syntax(&[]).unwrap(), 0);
        assert!(complete_raw_syntax(&[0xff, 0x80]).is_err());
        assert!(complete_raw_syntax(&[0xff]).is_err());
    }

    #[test]
    fn authored_native_stream_has_positive_encoder_syntax() {
        let width = 133u32;
        let height = 129u32;
        let raw: Vec<_> = (0..width * height)
            .flat_map(|i| ((i * 79) as u16).to_le_bytes())
            .collect();
        let planes = [cs::LosslessD2Plane {
            samples: &raw,
            stride_bytes: width as usize * 2,
            sample_step_bytes: 2,
        }];
        let encoded = cs::encode_lossless_d2_bypass_test_fixture(
            width,
            height,
            16,
            &planes,
            cs::LosslessEncodeLimits::default(),
        )
        .unwrap();
        let observed = profile(&encoded, width, height, 1, 16, 1).unwrap();
        assert!(observed["raw_segments"].as_u64().unwrap() > 0);
        assert!(observed["stuffed_raw_bytes"].as_u64().unwrap() > 0);
    }
}

fn main() {
    match run() {
        Ok(result) => println!("{result}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
