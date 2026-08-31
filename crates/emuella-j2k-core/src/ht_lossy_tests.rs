//! Full-image and negative-path proof for the shared irreversible HT boundary.
use super::*;

fn source(width: u32, height: u32, bits: u8, components: u16) -> Vec<Vec<u8>> {
    codestream::ht_lossy_test_support::source(width, height, bits, components, 0)
        .iter()
        .map(|plane| {
            plane
                .iter()
                .flat_map(|&v| {
                    if bits == 8 {
                        vec![v as u8]
                    } else {
                        v.to_le_bytes().to_vec()
                    }
                })
                .collect()
        })
        .collect()
}
fn encoded(bits: u8, components: u16, rate: f32) -> (Vec<Vec<u8>>, Vec<u8>) {
    let planes = source(257, 193, bits, components);
    let info = ImageInfo::new(
        257,
        193,
        components,
        if bits == 8 {
            SampleFormat::U8
        } else {
            SampleFormat::U16_LE
        },
        if components == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        ComponentLayout::Planar,
    )
    .unwrap();
    let views = planes
        .iter()
        .map(|samples| {
            Plane::new(
                samples,
                257,
                193,
                257 * usize::from(bits / 8),
                info.sample_format,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let raw = encode_htj2k_lossy(
        ImageView::Planar {
            info: &info,
            planes: &views,
        },
        &Htj2kLossyEncodeOptions {
            bits_per_pixel: rate,
        },
    )
    .unwrap();
    (planes, raw)
}
fn wrapped(raw: &[u8], bits: u8, components: u16) -> Vec<u8> {
    let info = ImageInfo::new(
        257,
        193,
        components,
        if bits == 8 {
            SampleFormat::U8
        } else {
            SampleFormat::U16_LE
        },
        if components == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        ComponentLayout::Planar,
    )
    .unwrap();
    let mut bytes = Vec::new();
    write_jph_encode_output(&info, raw, &mut bytes).unwrap();
    assert_eq!(
        container::parse(&bytes)
            .unwrap()
            .primary_codestream(&bytes)
            .unwrap(),
        Some(raw)
    );
    bytes
}
pub(super) fn options(layout: ComponentLayout) -> DecodeOptions {
    DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: layout,
        ..DecodeOptions::default()
    }
}
pub(super) fn assert_caller(input: &[u8], expected: &Image, failure: bool) {
    let width = expected.info.width as usize;
    let height = expected.info.height as usize;
    let bytes = usize::from(expected.info.sample_format.bits_per_sample / 8);
    let info = &expected.info;
    let opts = options(info.layout);
    match &expected.data {
        ImageData::Planes(planes) => {
            let row = width * bytes;
            let stride = row + 11;
            let mut buffers = vec![vec![0xa5; stride * height + 7]; planes.len()];
            let mut targets = buffers
                .iter_mut()
                .map(|buffer| {
                    PlaneMut::new(buffer, info.width, info.height, stride, info.sample_format)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let result = decode_into(
                input,
                &mut ImageViewMut::Planar {
                    info,
                    planes: &mut targets,
                },
                &opts,
            );
            assert_eq!(result.is_err(), failure, "{result:?}");
            for (buffer, plane) in buffers.iter().zip(planes) {
                for y in 0..height {
                    assert_eq!(
                        &buffer[y * stride..y * stride + row],
                        if failure {
                            vec![0xa5; row]
                        } else {
                            plane[y * row..(y + 1) * row].to_vec()
                        }
                    );
                    assert!(
                        buffer[y * stride + row..(y + 1) * stride]
                            .iter()
                            .all(|&v| v == 0xa5)
                    );
                }
                assert!(buffer[stride * height..].iter().all(|&v| v == 0xa5));
            }
        }
        ImageData::Interleaved(samples) => {
            let row = width * bytes * usize::from(info.components);
            let stride = row + 13;
            let mut buffer = vec![0xa5; stride * height + 7];
            let result = decode_into(
                input,
                &mut ImageViewMut::Interleaved {
                    info,
                    samples: &mut buffer,
                    stride_bytes: stride,
                },
                &opts,
            );
            assert_eq!(result.is_err(), failure, "{result:?}");
            for y in 0..height {
                assert_eq!(
                    &buffer[y * stride..y * stride + row],
                    if failure {
                        vec![0xa5; row]
                    } else {
                        samples[y * row..(y + 1) * row].to_vec()
                    }
                );
                assert!(
                    buffer[y * stride + row..(y + 1) * stride]
                        .iter()
                        .all(|&v| v == 0xa5)
                );
            }
            assert!(buffer[stride * height..].iter().all(|&v| v == 0xa5));
        }
    }
}
pub(super) fn sse(source: &[Vec<u8>], image: &Image, bits: u8) -> u128 {
    let ImageData::Planes(planes) = &image.data else {
        panic!("planar result required")
    };
    source
        .iter()
        .zip(planes)
        .flat_map(|(a, b)| {
            a.chunks_exact(usize::from(bits / 8))
                .zip(b.chunks_exact(usize::from(bits / 8)))
        })
        .map(|(a, b)| {
            let sample = |v: &[u8]| {
                if bits == 8 {
                    u16::from(v[0])
                } else {
                    u16::from_le_bytes([v[0], v[1]])
                }
            };
            u128::from(sample(a).abs_diff(sample(b))).pow(2)
        })
        .sum()
}

#[test]
fn lossy_ht_all_native_formats_rates_raw_jph_owned_shape_and_padded_caller() {
    for bits in [8, 16] {
        for components in [1, 3] {
            let mut previous = u128::MAX;
            for (rate, ceiling) in [(1, 125), (2, 60), (4, 40)] {
                let (source, raw) = encoded(bits, components, rate as f32);
                let budget = 257 * 193 * rate / 8;
                assert!(raw.len() <= budget && budget - raw.len() <= 32);
                // Padded planar views resolve to the same complete codestream.
                let row = 257 * usize::from(bits / 8);
                let padded = source
                    .iter()
                    .map(|plane| {
                        let mut padded = vec![0xee; (row + 3) * 193];
                        for y in 0..193 {
                            padded[y * (row + 3)..y * (row + 3) + row]
                                .copy_from_slice(&plane[y * row..(y + 1) * row]);
                        }
                        padded
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    raw,
                    codestream::ht_lossy::encode_planar(
                        257,
                        193,
                        bits,
                        &padded.iter().map(Vec::as_slice).collect::<Vec<_>>(),
                        &vec![row + 3; usize::from(components)],
                        rate as f32
                    )
                    .unwrap()
                );
                let planar = decode(&raw, &options(ComponentLayout::Planar)).unwrap();
                let error = sse(&source, &planar, bits);
                assert!(error <= previous);
                assert!(
                    error * 1000
                        <= 257
                            * 193
                            * u128::from(components)
                            * ((1_u128 << bits) - 1).pow(2)
                            * ceiling
                );
                previous = error;
                for input in [&raw, &wrapped(&raw, bits, components)] {
                    assert_eq!(
                        inspect(input, &InspectOptions::default()).unwrap().support,
                        SupportStatus::Supported
                    );
                    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                        let opts = options(layout);
                        let image = decode(input, &opts).unwrap();
                        let shape = decode_shape(input, &opts).unwrap();
                        assert_eq!(shape.image_info().unwrap(), image.info);
                        assert_eq!(
                            decode_htj2k_with_workspace(
                                input,
                                &opts,
                                &mut Htj2kDecodeWorkspace::new()
                            )
                            .unwrap(),
                            Some(image.clone())
                        );
                        if layout == ComponentLayout::Planar {
                            assert_eq!(image, planar);
                        } else {
                            assert_eq!(
                                image.data,
                                ImageData::Interleaved(
                                    interleave_planes(
                                        match &planar.data {
                                            ImageData::Planes(p) => p,
                                            _ => unreachable!(),
                                        },
                                        257,
                                        193,
                                        planar.info.sample_format
                                    )
                                    .unwrap()
                                )
                            );
                        }
                        assert_caller(input, &image, false);
                    }
                    assert!(decode(input, &DecodeOptions::default()).is_err());
                    assert!(decode_shape(input, &DecodeOptions::default()).is_err());
                    let subset = DecodeOptions {
                        requested_components: ComponentSelection::Indices(vec![0]),
                        ..options(ComponentLayout::Planar)
                    };
                    assert!(decode(input, &subset).is_err());
                    assert!(decode_shape(input, &subset).is_err());
                }
            }
        }
    }
}

#[test]
fn lossy_ht_late_entropy_failure_preserves_every_caller_byte_and_workspace_reuse() {
    for bits in [8, 16] {
        let (_, raw) = encoded(bits, 3, 2.0);
        let mut corrupt = raw.clone();
        // Final cleanup segment: invalidate its termination length while leaving
        // marker and packet headers intact. Earlier components reconstruct first.
        let end = corrupt.len() - 2;
        corrupt[end - 1] = 0;
        corrupt[end - 2] &= 0xf0;
        let parsed = codestream::parse(&corrupt).unwrap();
        assert!(codestream::ht_lossy::is_profile(&corrupt, &parsed));
        for input in [&corrupt, &wrapped(&corrupt, bits, 3)] {
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                let opts = options(layout);
                assert!(decode_shape(input, &opts).is_ok());
                let image = decode(&raw, &opts).unwrap();
                assert_caller(input, &image, true);
                let mut workspace = Htj2kDecodeWorkspace::new();
                assert!(decode_htj2k_with_workspace(input, &opts, &mut workspace).is_err());
                assert_eq!(
                    decode_htj2k_with_workspace(&raw, &opts, &mut workspace).unwrap(),
                    Some(image)
                );
            }
        }
    }
}

#[test]
fn lossy_ht_neighbour_support_shape_and_caller_admission_agree() {
    let (_, raw) = encoded(8, 3, 1.0);
    let parsed = codestream::parse(&raw).unwrap();
    let cod = parsed
        .markers
        .iter()
        .find(|m| m.marker == codestream::Marker::Cod)
        .unwrap()
        .offset;
    let qcd = parsed
        .markers
        .iter()
        .find(|m| m.marker == codestream::Marker::Qcd)
        .unwrap()
        .offset;
    let mut variants = Vec::new();
    for (offset, value) in [
        (cod + 8, 1),
        (cod + 9, 3),
        (cod + 10, 3),
        (qcd + 4, 0x42),
        (42, 6),
        (43, 2),
    ] {
        let mut bytes = raw.clone();
        bytes[offset] = value;
        variants.push(bytes);
    }
    variants.push(raw[..raw.len() - 1].to_vec());
    for bytes in variants {
        for input in [&bytes, &wrapped(&bytes, 8, 3)] {
            if let Ok(metadata) = inspect(input, &InspectOptions::default()) {
                assert!(!matches!(metadata.support, SupportStatus::Supported));
            }
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                let opts = options(layout);
                assert!(decode_shape(input, &opts).is_err());
                assert!(decode(input, &opts).is_err());
                assert_caller(input, &decode(&raw, &opts).unwrap(), true);
            }
        }
    }
    let image = decode(&raw, &options(ComponentLayout::Interleaved)).unwrap();
    let mut wrong_info = image.info.clone();
    wrong_info.width -= 1;
    let mut buffer = vec![0xa5; 257 * 193 * 3];
    assert!(
        decode_into(
            &raw,
            &mut ImageViewMut::Interleaved {
                info: &wrong_info,
                samples: &mut buffer,
                stride_bytes: 257 * 3
            },
            &options(ComponentLayout::Interleaved)
        )
        .is_err()
    );
    assert!(buffer.iter().all(|&v| v == 0xa5));
    assert!(
        decode_into(
            &raw,
            &mut ImageViewMut::Interleaved {
                info: &image.info,
                samples: &mut buffer,
                stride_bytes: 257 * 3 - 1
            },
            &options(ComponentLayout::Interleaved)
        )
        .is_err()
    );
    assert!(buffer.iter().all(|&v| v == 0xa5));
}

#[test]
fn lossy_ht_full_admission_does_not_enable_partial_metadata_or_crops() {
    let (_, raw) = encoded(8, 3, 1.0);
    for input in [&raw, &wrapped(&raw, 8, 3)] {
        let mut requests = vec![PartialDecodeOptions::default()];
        requests.push(PartialDecodeOptions {
            region: Some(Region {
                x: 1,
                y: 1,
                width: 7,
                height: 9,
            }),
            ..PartialDecodeOptions::default()
        });
        requests.push(PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 0,
                tile_y: 0,
            }),
            ..PartialDecodeOptions::default()
        });
        requests.push(PartialDecodeOptions {
            resolution: ResolutionLevel::Reduced { discard_levels: 2 },
            components: ComponentSelection::Indices(vec![0]),
            ..PartialDecodeOptions::default()
        });
        requests.push(PartialDecodeOptions {
            max_quality_layers: Some(1),
            ..PartialDecodeOptions::default()
        });
        for request in requests {
            assert!(decode_partial_info(input, &request).is_err());
            assert!(decode_partial(input, &request).is_err());
            assert!(decode_partial_component_info(input, &request).is_err());
        }
    }
}
