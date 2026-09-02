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

fn encoded_u16_greyscale(width: u32, height: u32, rate: f32) -> (Vec<u8>, Vec<u8>) {
    let source = codestream::ht_lossy_test_support::source(width, height, 16, 1, 0)[0]
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect::<Vec<_>>();
    let info = ImageInfo::new(
        width,
        height,
        1,
        SampleFormat::U16_LE,
        ColorModel::Grayscale,
        ComponentLayout::Planar,
    )
    .unwrap();
    let plane = Plane::new(
        &source,
        width,
        height,
        width as usize * 2,
        SampleFormat::U16_LE,
    )
    .unwrap();
    let raw = encode_htj2k_lossy(
        ImageView::Planar {
            info: &info,
            planes: &[plane],
        },
        &Htj2kLossyEncodeOptions {
            bits_per_pixel: rate,
        },
    )
    .unwrap();
    (source, raw)
}

fn reduced_u16_options(discard_levels: u8) -> PartialDecodeOptions {
    PartialDecodeOptions {
        resolution: ResolutionLevel::Reduced { discard_levels },
        components: ComponentSelection::Indices(vec![0]),
        target_layout: ComponentLayout::Planar,
        ..PartialDecodeOptions::default()
    }
}

fn wrapped_dimensions(raw: &[u8], width: u32, height: u32, bits: u8, components: u16) -> Vec<u8> {
    let info = ImageInfo::new(
        width,
        height,
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
fn lossy_ht_full_admission_does_not_enable_unqualified_partial_requests() {
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

#[test]
fn lossy_ht_u16_greyscale_discard_one_geometry_and_routes_agree() {
    let (_, raw) = encoded_u16_greyscale(258, 193, 2.0);
    let request = reduced_u16_options(1);
    let info = decode_partial_info(&raw, &request).unwrap();
    assert_eq!(
        info,
        ImageInfo::new(
            129,
            97,
            1,
            SampleFormat::U16_LE,
            ColorModel::Unknown,
            ComponentLayout::Planar,
        )
        .unwrap()
    );
    let component_info = decode_partial_component_info(&raw, &request).unwrap();
    assert_eq!(component_info.len(), 1);
    assert_eq!(component_info[0].source_component, Some(0));
    assert_eq!(
        (component_info[0].width, component_info[0].height),
        (129, 97)
    );
    assert_eq!(component_info[0].sample_format, SampleFormat::U16_LE);

    let owned = decode_partial(&raw, &request).unwrap();
    assert_eq!(owned.info, info);
    assert_eq!(owned.component_info, component_info);
    let ImageData::Planes(owned_planes) = &owned.data else {
        panic!("reduced component output was not planar")
    };
    assert_eq!(owned_planes.len(), 1);
    assert_eq!(owned_planes[0].len(), 129 * 97 * 2);
    assert!(
        owned_planes[0]
            .chunks_exact(2)
            .any(|sample| sample[0] != sample[1])
    );
    assert_eq!(decode_partial(&raw, &request).unwrap(), owned);

    let prepared = codestream::ht_lossy::prepare_reduced_component_decode(
        &raw,
        codestream::Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 1,
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        (prepared.output_width(), prepared.output_height()),
        (129, 97)
    );
    let internal = codestream::decode_prepared_htj2k_reduced_component_owned_with_workspace(
        &prepared,
        &mut codestream::HtCodestreamDecodeWorkspace::new(),
    )
    .unwrap();
    assert_eq!(internal.bits_per_sample, 16);
    assert!(!internal.signed);
    assert_eq!(internal.components[0].samples, owned_planes[0]);

    let row_bytes = 129 * 2;
    let stride = row_bytes + 7;
    let mut workspace = Part1DecodeWorkspace::new();
    for sentinel in [0xa5, 0x6d] {
        let mut buffer = vec![sentinel; stride * 97 + 5];
        {
            let plane = PlaneMut::new(&mut buffer, 129, 97, stride, SampleFormat::U16_LE).unwrap();
            let mut planes = [plane];
            decode_partial_into_with_workspace(
                &raw,
                &mut ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                },
                &request,
                &mut workspace,
            )
            .unwrap();
        }
        for y in 0..97 {
            assert_eq!(
                &buffer[y * stride..y * stride + row_bytes],
                &owned_planes[0][y * row_bytes..(y + 1) * row_bytes]
            );
            assert!(
                buffer[y * stride + row_bytes..(y + 1) * stride]
                    .iter()
                    .all(|byte| *byte == sentinel)
            );
        }
        assert!(buffer[stride * 97..].iter().all(|byte| *byte == sentinel));
    }
}

#[test]
fn lossy_ht_u16_greyscale_discard_one_rejects_neighbours_atomically() {
    let (_, raw) = encoded_u16_greyscale(257, 193, 2.0);
    let admitted = reduced_u16_options(1);
    let info = decode_partial_info(&raw, &admitted).unwrap();

    let assert_atomic_rejection = |input: &[u8], request: &PartialDecodeOptions| {
        let stride = info.width as usize * 2 + 5;
        let mut buffer = vec![0xa5; stride * info.height as usize + 3];
        {
            let plane = PlaneMut::new(
                &mut buffer,
                info.width,
                info.height,
                stride,
                SampleFormat::U16_LE,
            )
            .unwrap();
            let mut planes = [plane];
            assert!(
                decode_partial_into(
                    input,
                    &mut ImageViewMut::Planar {
                        info: &info,
                        planes: &mut planes,
                    },
                    request,
                )
                .is_err(),
                "request unexpectedly succeeded: {request:?}"
            );
        }
        assert!(buffer.iter().all(|byte| *byte == 0xa5));
    };

    for discard_levels in [2, 3, 4, 5, 6] {
        let request = reduced_u16_options(discard_levels);
        assert!(decode_partial_info(&raw, &request).is_err());
        assert!(decode_partial_component_info(&raw, &request).is_err());
        assert!(decode_partial(&raw, &request).is_err());
        assert_atomic_rejection(&raw, &request);
    }

    let mut requests = vec![PartialDecodeOptions::default()];
    requests.push(PartialDecodeOptions {
        components: ComponentSelection::Indices(vec![1]),
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        region: Some(Region {
            x: 0,
            y: 0,
            width: 16,
            height: 16,
        }),
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        tile: Some(TileSelection {
            tile_x: 0,
            tile_y: 0,
        }),
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        max_quality_layers: Some(1),
        ..admitted.clone()
    });
    for request in requests {
        assert!(decode_partial_info(&raw, &request).is_err());
        assert!(decode_partial(&raw, &request).is_err());
        assert_atomic_rejection(&raw, &request);
    }
    let interleaved_request = PartialDecodeOptions {
        target_layout: ComponentLayout::Interleaved,
        ..admitted.clone()
    };
    assert!(decode_partial_info(&raw, &interleaved_request).is_err());
    assert!(decode_partial(&raw, &interleaved_request).is_err());
    let interleaved_info = ImageInfo {
        layout: ComponentLayout::Interleaved,
        ..info.clone()
    };
    let mut interleaved = vec![0xa5; info.width as usize * info.height as usize * 2];
    assert!(
        decode_partial_into(
            &raw,
            &mut ImageViewMut::Interleaved {
                info: &interleaved_info,
                samples: &mut interleaved,
                stride_bytes: info.width as usize * 2,
            },
            &admitted,
        )
        .is_err()
    );
    assert!(interleaved.iter().all(|byte| *byte == 0xa5));

    let jph = wrapped_dimensions(&raw, 257, 193, 16, 1);
    assert!(decode_partial_info(&jph, &admitted).is_err());
    assert_atomic_rejection(&jph, &admitted);
    let (_, u8_raw) = encoded(8, 1, 2.0);
    assert!(decode_partial_info(&u8_raw, &admitted).is_err());
    assert_atomic_rejection(&u8_raw, &admitted);
    let (_, rgb_raw) = encoded(16, 3, 2.0);
    assert!(decode_partial_info(&rgb_raw, &admitted).is_err());
    assert_atomic_rejection(&rgb_raw, &admitted);

    let mut signed = raw.clone();
    signed[42] |= 0x80;
    assert!(decode_partial_info(&signed, &admitted).is_err());
    assert_atomic_rejection(&signed, &admitted);
    let parsed = codestream::parse(&raw).unwrap();
    let cod = parsed
        .markers
        .iter()
        .find(|marker| marker.marker == codestream::Marker::Cod)
        .unwrap()
        .offset;
    for levels in [1, 3] {
        let mut other_decomposition = raw.clone();
        other_decomposition[cod + 9] = levels;
        assert!(decode_partial_info(&other_decomposition, &admitted).is_err());
        assert_atomic_rejection(&other_decomposition, &admitted);
    }
    let sod = raw
        .windows(2)
        .position(|bytes| bytes == codestream::Marker::Sod.code().to_be_bytes())
        .unwrap();
    let mut invalid_packet = raw.clone();
    invalid_packet[sod + 2] ^= 0x40;
    assert!(decode_partial_info(&invalid_packet, &admitted).is_err());
    assert_atomic_rejection(&invalid_packet, &admitted);
    let truncated = &raw[..raw.len() - 1];
    assert!(decode_partial_info(truncated, &admitted).is_err());
    assert_atomic_rejection(truncated, &admitted);

    let tile = parsed.tiles[0];
    let payload_offset = tile.payload_offset.unwrap();
    let payload_len = tile.payload_len.unwrap();
    let contributions = codestream::parse_default_precinct_lrcp_packets(
        &raw,
        &parsed,
        codestream::TileRect {
            tile_index: 0,
            tile_x: 0,
            tile_y: 0,
            x: 0,
            y: 0,
            width: 257,
            height: 193,
        },
        &raw[payload_offset..payload_offset + payload_len],
    )
    .unwrap();
    let retained = contributions
        .iter()
        .find(|contribution| {
            contribution.component_index == 0
                && contribution.resolution <= 1
                && contribution.coding_passes == 1
                && contribution.codeword_len >= 2
        })
        .unwrap();
    let cleanup_end = payload_offset + retained.payload_offset + retained.codeword_len;
    let mut late = raw.clone();
    late[cleanup_end - 2] &= 0xf0;
    late[cleanup_end - 1] = 0;
    assert!(decode_partial_info(&late, &admitted).is_ok());
    assert!(decode_partial(&late, &admitted).is_err());
    assert_atomic_rejection(&late, &admitted);
}
