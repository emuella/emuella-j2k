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

fn try_encoded_u16_greyscale(
    width: u32,
    height: u32,
    pattern: u32,
    rate: f32,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let source = codestream::ht_lossy_test_support::source(width, height, 16, 1, pattern)[0]
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
    )?;
    Ok((source, raw))
}

fn encoded_u16_greyscale(width: u32, height: u32, rate: f32) -> (Vec<u8>, Vec<u8>) {
    try_encoded_u16_greyscale(width, height, 0, rate).unwrap()
}

fn reduced_u16_options(discard_levels: u8) -> PartialDecodeOptions {
    PartialDecodeOptions {
        resolution: ResolutionLevel::Reduced { discard_levels },
        components: ComponentSelection::Indices(vec![0]),
        target_layout: ComponentLayout::Planar,
        ..PartialDecodeOptions::default()
    }
}

fn spatial_u16_options(region: Region, discard_levels: u8) -> PartialDecodeOptions {
    PartialDecodeOptions {
        region: Some(region),
        resolution: if discard_levels == 0 {
            ResolutionLevel::Full
        } else {
            ResolutionLevel::Reduced { discard_levels }
        },
        components: ComponentSelection::Indices(vec![0]),
        target_layout: ComponentLayout::Planar,
        ..PartialDecodeOptions::default()
    }
}

fn projected_region(region: Region, discard_levels: u8) -> Region {
    let divisor = 1_u32 << discard_levels;
    let x0 = region.x.div_ceil(divisor);
    let y0 = region.y.div_ceil(divisor);
    let x1 = (region.x + region.width).div_ceil(divisor);
    let y1 = (region.y + region.height).div_ceil(divisor);
    Region {
        x: x0,
        y: y0,
        width: x1 - x0,
        height: y1 - y0,
    }
}

fn crop_u16(plane: &[u8], source_width: u32, region: Region) -> Vec<u8> {
    let source_row_bytes = source_width as usize * 2;
    let output_row_bytes = region.width as usize * 2;
    let mut output = Vec::with_capacity(output_row_bytes * region.height as usize);
    for y in region.y..region.y + region.height {
        let start = y as usize * source_row_bytes + region.x as usize * 2;
        output.extend_from_slice(&plane[start..start + output_row_bytes]);
    }
    output
}

fn assert_spatial_u16_routes(
    raw: &[u8],
    full_width: u32,
    full_height: u32,
    region: Region,
    discard_levels: u8,
    workspace: &mut Part1DecodeWorkspace,
) -> Image {
    let request = spatial_u16_options(region, discard_levels);
    let projected = projected_region(region, discard_levels);
    let info = decode_partial_info(raw, &request)
        .unwrap_or_else(|error| panic!("spatial request {request:?} failed: {error:?}"));
    assert_eq!(
        info,
        ImageInfo::new(
            projected.width,
            projected.height,
            1,
            SampleFormat::U16_LE,
            ColorModel::Unknown,
            ComponentLayout::Planar,
        )
        .unwrap()
    );
    let descriptors = decode_partial_component_info(raw, &request).unwrap();
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].source_component, Some(0));
    assert_eq!(descriptors[0].sample_format, SampleFormat::U16_LE);
    assert_eq!(
        (
            descriptors[0].x_origin,
            descriptors[0].y_origin,
            descriptors[0].width,
            descriptors[0].height,
        ),
        (projected.x, projected.y, projected.width, projected.height,)
    );

    let oracle = if discard_levels == 0 {
        decode(raw, &options(ComponentLayout::Planar)).unwrap()
    } else {
        decode_partial(raw, &reduced_u16_options(discard_levels)).unwrap()
    };
    let ImageData::Planes(oracle_planes) = &oracle.data else {
        panic!("lossy HT component oracle was not planar")
    };
    let oracle_width = if discard_levels == 0 {
        full_width
    } else {
        full_width.div_ceil(1_u32 << discard_levels)
    };
    let expected = crop_u16(&oracle_planes[0], oracle_width, projected);
    let owned = decode_partial(raw, &request).unwrap();
    assert_eq!(owned.info, info);
    assert_eq!(owned.component_info, descriptors);
    assert!(matches!(&owned.data, ImageData::Planes(planes) if planes == &[expected.clone()]));
    assert_eq!(decode_partial(raw, &request).unwrap(), owned);

    let row_bytes = projected.width as usize * 2;
    let stride = row_bytes + 11;
    let mut caller = vec![0xa5; stride * projected.height as usize + 7];
    {
        let plane = PlaneMut::new(
            &mut caller,
            projected.width,
            projected.height,
            stride,
            SampleFormat::U16_LE,
        )
        .unwrap();
        let mut planes = [plane];
        decode_partial_into_with_workspace(
            raw,
            &mut ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            },
            &request,
            workspace,
        )
        .unwrap();
    }
    for y in 0..projected.height as usize {
        assert_eq!(
            &caller[y * stride..y * stride + row_bytes],
            &expected[y * row_bytes..(y + 1) * row_bytes]
        );
        assert!(
            caller[y * stride + row_bytes..(y + 1) * stride]
                .iter()
                .all(|byte| *byte == 0xa5)
        );
    }
    assert!(
        caller[stride * projected.height as usize..]
            .iter()
            .all(|byte| *byte == 0xa5)
    );
    assert_eq!(
        full_height.div_ceil(1_u32 << discard_levels),
        oracle.info.height
    );
    owned
}

fn assert_encoder_rate_unattainable(error: &J2kError) {
    assert!(
        matches!(
            error,
            J2kError::Unsupported { detail, .. }
                if detail == "irreversible HT target rate is unattainable within the bounded non-padding tolerance"
        ),
        "expected EncoderRateUnattainable, got {error:?}"
    );
}

fn assert_reduced_u16_routes(
    raw: &[u8],
    discard_levels: u8,
    width: u32,
    height: u32,
    workspace: &mut Part1DecodeWorkspace,
) -> Image {
    let retained_workspace_bytes = workspace.retained_heap_bytes();
    let request = reduced_u16_options(discard_levels);
    let info = decode_partial_info(raw, &request).unwrap();
    assert_eq!(
        info,
        ImageInfo::new(
            width,
            height,
            1,
            SampleFormat::U16_LE,
            ColorModel::Unknown,
            ComponentLayout::Planar,
        )
        .unwrap()
    );
    let component_info = decode_partial_component_info(raw, &request).unwrap();
    assert_eq!(component_info.len(), 1);
    assert_eq!(component_info[0].source_component, Some(0));
    assert_eq!(
        (component_info[0].width, component_info[0].height),
        (width, height)
    );
    assert_eq!(component_info[0].sample_format, SampleFormat::U16_LE);

    let owned = decode_partial(raw, &request).unwrap();
    assert_eq!(owned.info, info);
    assert_eq!(owned.component_info, component_info);
    let ImageData::Planes(owned_planes) = &owned.data else {
        panic!("reduced component output was not planar")
    };
    assert_eq!(owned_planes.len(), 1);
    assert_eq!(owned_planes[0].len(), width as usize * height as usize * 2);
    assert_eq!(decode_partial(raw, &request).unwrap(), owned);

    let prepared = codestream::ht_lossy::prepare_reduced_component_decode(
        raw,
        codestream::Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels,
        },
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        (prepared.output_width(), prepared.output_height()),
        (width, height)
    );
    let internal = codestream::decode_prepared_htj2k_reduced_component_owned_with_workspace(
        &prepared,
        &mut codestream::HtCodestreamDecodeWorkspace::new(),
    )
    .unwrap();
    assert_eq!(internal.bits_per_sample, 16);
    assert!(!internal.signed);
    assert_eq!(internal.components[0].samples, owned_planes[0]);

    let row_bytes = width as usize * 2;
    let stride = row_bytes + 7;
    for sentinel in [0xa5, 0x6d] {
        let mut buffer = vec![sentinel; stride * height as usize + 5];
        {
            let plane =
                PlaneMut::new(&mut buffer, width, height, stride, SampleFormat::U16_LE).unwrap();
            let mut planes = [plane];
            decode_partial_into_with_workspace(
                raw,
                &mut ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                },
                &request,
                workspace,
            )
            .unwrap();
        }
        for y in 0..height as usize {
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
        assert!(
            buffer[stride * height as usize..]
                .iter()
                .all(|byte| *byte == sentinel)
        );
    }
    assert_eq!(workspace.retained_heap_bytes(), retained_workspace_bytes);
    workspace.clear();
    assert_eq!(workspace.retained_heap_bytes(), retained_workspace_bytes);
    owned
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
fn lossy_ht_u16_greyscale_discard_one_and_two_geometry_and_routes_agree() {
    let (_, raw) = encoded_u16_greyscale(258, 193, 2.0);
    let mut workspace = Part1DecodeWorkspace::new();
    let discard_one = assert_reduced_u16_routes(&raw, 1, 129, 97, &mut workspace);
    let discard_two = assert_reduced_u16_routes(&raw, 2, 65, 49, &mut workspace);
    let ImageData::Planes(discard_one_planes) = &discard_one.data else {
        unreachable!()
    };
    let ImageData::Planes(discard_two_planes) = &discard_two.data else {
        unreachable!()
    };
    assert!(
        discard_one_planes[0]
            .chunks_exact(2)
            .any(|sample| sample[0] != sample[1])
    );
    assert!(
        discard_two_planes[0]
            .chunks_exact(2)
            .any(|sample| sample[0] != sample[1])
    );
}

#[test]
fn lossy_ht_spatial_calibration_probe_is_public_encoder_output() {
    use sha2::{Digest, Sha256};

    let (source, raw) = try_encoded_u16_greyscale(1024, 1024, 1, 4.0).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&source)),
        "75855d11fddce88d3377bcaeab864905cef5cc724f0059da81271832010c9054"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&raw)),
        "f376f5c04b13c640fec6b80ba52bfb198cc1832d75f6850411ba801e035da597"
    );
}

#[test]
fn lossy_ht_spatial_public_geometry_routes_and_reuse_match_established_oracles() {
    let (_, raw) = try_encoded_u16_greyscale(193, 137, 0, 4.0).unwrap();
    let regions = [
        Region {
            x: 0,
            y: 0,
            width: 193,
            height: 137,
        },
        Region {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        },
        Region {
            x: 192,
            y: 0,
            width: 1,
            height: 1,
        },
        Region {
            x: 0,
            y: 136,
            width: 1,
            height: 1,
        },
        Region {
            x: 192,
            y: 136,
            width: 1,
            height: 1,
        },
        Region {
            x: 0,
            y: 71,
            width: 31,
            height: 1,
        },
        Region {
            x: 162,
            y: 71,
            width: 31,
            height: 1,
        },
        Region {
            x: 91,
            y: 0,
            width: 1,
            height: 29,
        },
        Region {
            x: 91,
            y: 108,
            width: 1,
            height: 29,
        },
        Region {
            x: 1,
            y: 3,
            width: 7,
            height: 9,
        },
        Region {
            x: 63,
            y: 63,
            width: 3,
            height: 3,
        },
        Region {
            x: 64,
            y: 64,
            width: 64,
            height: 64,
        },
        Region {
            x: 61,
            y: 62,
            width: 70,
            height: 69,
        },
        Region {
            x: 117,
            y: 83,
            width: 41,
            height: 37,
        },
    ];
    let mut workspace = Part1DecodeWorkspace::new();
    for discard_levels in 0..=2 {
        for region in regions {
            let projected = projected_region(region, discard_levels);
            if projected.width == 0 || projected.height == 0 {
                continue;
            }
            assert_spatial_u16_routes(&raw, 193, 137, region, discard_levels, &mut workspace);
        }
    }

    // Reuse the same retained allocations while growing, shrinking and
    // relocating the active region across every admitted resolution.
    for (region, discard_levels) in [
        (
            Region {
                x: 1,
                y: 1,
                width: 191,
                height: 127,
            },
            0,
        ),
        (
            Region {
                x: 127,
                y: 71,
                width: 65,
                height: 63,
            },
            1,
        ),
        (
            Region {
                x: 128,
                y: 72,
                width: 65,
                height: 65,
            },
            2,
        ),
        (
            Region {
                x: 7,
                y: 9,
                width: 11,
                height: 13,
            },
            0,
        ),
    ] {
        assert_spatial_u16_routes(&raw, 193, 137, region, discard_levels, &mut workspace);
    }
    let retained = workspace.retained_heap_bytes();
    assert!(retained > 0);
    workspace.clear();
    assert_eq!(workspace.retained_heap_bytes(), retained);
}

#[test]
fn lossy_ht_spatial_public_large_projection_and_pattern_rate_matrix() {
    let mut workspace = Part1DecodeWorkspace::new();
    for (pattern, rate) in [(0, 1.0), (1, 2.0), (7, 1.0)] {
        let Ok((_, raw)) = try_encoded_u16_greyscale(257, 193, pattern, rate) else {
            continue;
        };
        for discard_levels in 0..=2 {
            assert_spatial_u16_routes(
                &raw,
                257,
                193,
                Region {
                    x: 29,
                    y: 31,
                    width: 137,
                    height: 111,
                },
                discard_levels,
                &mut workspace,
            );
        }
    }

    let (_, raw) = try_encoded_u16_greyscale(1024, 1024, 1, 4.0).unwrap();
    let region = Region {
        x: 256,
        y: 256,
        width: 256,
        height: 256,
    };
    for (discard_levels, expected) in [(0, 256), (1, 128), (2, 64)] {
        let selected =
            assert_spatial_u16_routes(&raw, 1024, 1024, region, discard_levels, &mut workspace);
        assert_eq!(
            (selected.info.width, selected.info.height),
            (expected, expected)
        );
    }
    let full = assert_spatial_u16_routes(
        &raw,
        1024,
        1024,
        Region {
            x: 0,
            y: 0,
            width: 1024,
            height: 1024,
        },
        2,
        &mut workspace,
    );
    assert_eq!((full.info.width, full.info.height), (256, 256));
}

#[test]
fn lossy_ht_spatial_public_failures_are_atomic_and_workspace_recovers() {
    let (_, raw) = try_encoded_u16_greyscale(257, 193, 1, 2.0).unwrap();
    let region = Region {
        x: 57,
        y: 61,
        width: 101,
        height: 79,
    };
    let request = spatial_u16_options(region, 1);
    let info = decode_partial_info(&raw, &request).unwrap();
    let prepared = codestream::ht_lossy::prepare_lossy_ht_spatial_region(
        &raw,
        codestream::ht_lossy::LossyHtSpatialRegionRequest::new(
            codestream::TileRegionRequest {
                x: region.x,
                y: region.y,
                width: region.width,
                height: region.height,
            },
            1,
        ),
    )
    .unwrap();
    let mut workspace = Part1DecodeWorkspace::new();
    let required = prepared.required_workspace_bytes();
    workspace.set_lossy_ht_spatial_region_memory_limit(required - 1);

    let assert_atomic =
        |input: &[u8], options: &PartialDecodeOptions, workspace: &mut Part1DecodeWorkspace| {
            let row_bytes = info.width as usize * 2;
            let stride = row_bytes + 9;
            let mut caller = vec![0x6d; stride * info.height as usize + 5];
            {
                let plane = PlaneMut::new(
                    &mut caller,
                    info.width,
                    info.height,
                    stride,
                    SampleFormat::U16_LE,
                )
                .unwrap();
                let mut planes = [plane];
                assert!(
                    decode_partial_into_with_workspace(
                        input,
                        &mut ImageViewMut::Planar {
                            info: &info,
                            planes: &mut planes
                        },
                        options,
                        workspace,
                    )
                    .is_err()
                );
            }
            assert!(caller.iter().all(|byte| *byte == 0x6d));
        };
    assert_atomic(&raw, &request, &mut workspace);
    workspace.set_lossy_ht_spatial_region_memory_limit(64 * 1024 * 1024);
    assert_spatial_u16_routes(&raw, 257, 193, region, 1, &mut workspace);

    // Retain an allocation, then prove that a small request's active-use
    // ceiling is not confused with the workspace's existing capacity.
    let small_region = Region {
        x: 9,
        y: 11,
        width: 13,
        height: 15,
    };
    let small_prepared = codestream::ht_lossy::prepare_lossy_ht_spatial_region(
        &raw,
        codestream::ht_lossy::LossyHtSpatialRegionRequest::new(
            codestream::TileRegionRequest {
                x: small_region.x,
                y: small_region.y,
                width: small_region.width,
                height: small_region.height,
            },
            2,
        ),
    )
    .unwrap();
    let retained = workspace.retained_heap_bytes();
    let small_active = small_prepared.required_workspace_bytes();
    assert!(retained > 0);
    assert!(small_active > 0);
    workspace.set_lossy_ht_spatial_region_memory_limit(small_active);
    assert_spatial_u16_routes(&raw, 257, 193, small_region, 2, &mut workspace);
    assert!(workspace.retained_heap_bytes() > 0);
    workspace.set_lossy_ht_spatial_region_memory_limit(64 * 1024 * 1024);

    let mut corrupt = raw.clone();
    let parsed = codestream::parse(&raw).unwrap();
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
                && contribution.codeword_len >= 2
        })
        .unwrap();
    let cleanup_end = payload_offset + retained.payload_offset + retained.codeword_len;
    corrupt[cleanup_end - 2] &= 0xf0;
    corrupt[cleanup_end - 1] = 0;
    assert!(decode_partial_info(&corrupt, &request).is_ok());
    assert_atomic(&corrupt, &request, &mut workspace);
    assert_atomic(&raw[..raw.len() - 1], &request, &mut workspace);
    assert_spatial_u16_routes(&raw, 257, 193, region, 1, &mut workspace);
}

#[test]
fn lossy_ht_spatial_public_rejects_geometry_layout_and_profile_neighbours() {
    let (_, raw) = try_encoded_u16_greyscale(257, 193, 1, 2.0).unwrap();
    let admitted = spatial_u16_options(
        Region {
            x: 17,
            y: 19,
            width: 83,
            height: 71,
        },
        1,
    );
    let rejected = [
        PartialDecodeOptions {
            region: Some(Region {
                x: 0,
                y: 0,
                width: 0,
                height: 1,
            }),
            ..admitted.clone()
        },
        PartialDecodeOptions {
            region: Some(Region {
                x: 257,
                y: 0,
                width: 1,
                height: 1,
            }),
            ..admitted.clone()
        },
        PartialDecodeOptions {
            region: Some(Region {
                x: u32::MAX,
                y: 0,
                width: 2,
                height: 1,
            }),
            ..admitted.clone()
        },
        // Independent endpoint ceilings produce an empty discard-two result.
        spatial_u16_options(
            Region {
                x: 1,
                y: 1,
                width: 1,
                height: 1,
            },
            2,
        ),
        spatial_u16_options(
            Region {
                x: 17,
                y: 19,
                width: 83,
                height: 71,
            },
            3,
        ),
        PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 0,
                tile_y: 0,
            }),
            ..admitted.clone()
        },
        PartialDecodeOptions {
            components: ComponentSelection::All,
            ..admitted.clone()
        },
        PartialDecodeOptions {
            components: ComponentSelection::Indices(vec![1]),
            ..admitted.clone()
        },
        PartialDecodeOptions {
            max_quality_layers: Some(1),
            ..admitted.clone()
        },
        PartialDecodeOptions {
            target_layout: ComponentLayout::Interleaved,
            ..admitted.clone()
        },
    ];
    for request in rejected {
        assert!(decode_partial_info(&raw, &request).is_err(), "{request:?}");
        assert!(
            decode_partial_component_info(&raw, &request).is_err(),
            "{request:?}"
        );
        assert!(decode_partial(&raw, &request).is_err(), "{request:?}");
    }

    let jph = wrapped_dimensions(&raw, 257, 193, 16, 1);
    assert!(decode_partial_info(&jph, &admitted).is_err());
    assert!(decode_partial(&jph, &admitted).is_err());
    let wrapper_info = ImageInfo::new(
        257,
        193,
        1,
        SampleFormat::U16_LE,
        ColorModel::Grayscale,
        ComponentLayout::Planar,
    )
    .unwrap();
    let mut jp2 = Vec::new();
    write_jp2_encode_output(&wrapper_info, &raw, &EncodeOptions::default(), &mut jp2).unwrap();
    assert!(decode_partial_info(&jp2, &admitted).is_err());
    assert!(decode_partial(&jp2, &admitted).is_err());
    let (_, u8_raw) = encoded(8, 1, 2.0);
    let (_, rgb_raw) = encoded(16, 3, 2.0);
    for neighbour in [&u8_raw, &rgb_raw] {
        assert!(decode_partial_info(neighbour, &admitted).is_err());
        assert!(decode_partial(neighbour, &admitted).is_err());
    }

    let parsed = codestream::parse(&raw).unwrap();
    let mut signed = raw.clone();
    signed[42] |= 0x80;
    assert!(decode_partial_info(&signed, &admitted).is_err());
    assert!(decode_partial(&signed, &admitted).is_err());
    let cod = parsed
        .markers
        .iter()
        .find(|marker| marker.marker == codestream::Marker::Cod)
        .unwrap()
        .offset;
    let qcd = parsed
        .markers
        .iter()
        .find(|marker| marker.marker == codestream::Marker::Qcd)
        .unwrap()
        .offset;
    for (offset, value) in [
        (cod + 8, 1),
        (cod + 9, 1),
        (cod + 10, 3),
        (cod + 12, 0),
        (cod + 13, 1),
        (qcd + 4, 0x42),
    ] {
        let mut neighbour = raw.clone();
        neighbour[offset] = value;
        assert!(decode_partial_info(&neighbour, &admitted).is_err());
        assert!(decode_partial(&neighbour, &admitted).is_err());
    }

    // Target validation, including final extent and stride, precedes all
    // publication and preserves active bytes plus trailing storage.
    let info = decode_partial_info(&raw, &admitted).unwrap();
    let mut wrong_info = info.clone();
    wrong_info.width -= 1;
    let mut caller = vec![0x3c; info.width as usize * info.height as usize * 2 + 13];
    let row_bytes = info.width as usize * 2;
    {
        let plane = PlaneMut {
            samples: &mut caller,
            width: info.width,
            height: info.height,
            stride_bytes: row_bytes - 1,
            sample_format: SampleFormat::U16_LE,
        };
        let mut planes = [plane];
        assert!(
            decode_partial_into(
                &raw,
                &mut ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                },
                &admitted,
            )
            .is_err()
        );
    }
    assert!(caller.iter().all(|byte| *byte == 0x3c));
    let mut short = vec![0x4d; row_bytes * info.height as usize - 1];
    {
        let plane = PlaneMut {
            samples: &mut short,
            width: info.width,
            height: info.height,
            stride_bytes: row_bytes,
            sample_format: SampleFormat::U16_LE,
        };
        let mut planes = [plane];
        assert!(
            decode_partial_into(
                &raw,
                &mut ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                },
                &admitted,
            )
            .is_err()
        );
    }
    assert!(short.iter().all(|byte| *byte == 0x4d));
    let mut planes = [PlaneMut::new(
        &mut caller,
        info.width,
        info.height,
        row_bytes,
        SampleFormat::U16_LE,
    )
    .unwrap()];
    assert!(
        decode_partial_into(
            &raw,
            &mut ImageViewMut::Planar {
                info: &wrong_info,
                planes: &mut planes,
            },
            &admitted,
        )
        .is_err()
    );
    assert!(caller.iter().all(|byte| *byte == 0x3c));
}

#[test]
fn lossy_ht_u16_greyscale_reduced_acceptance_matrix() {
    struct RateCase {
        label: &'static str,
        width: u32,
        height: u32,
        pattern: u32,
        expected: [bool; 3],
    }

    let cases = [
        RateCase {
            label: "smooth odd non-power-of-two",
            width: 257,
            height: 193,
            pattern: 7,
            expected: [true, false, false],
        },
        RateCase {
            label: "structured high-contrast odd non-power-of-two",
            width: 257,
            height: 193,
            pattern: 4,
            expected: [false, false, false],
        },
        RateCase {
            label: "high-entropy odd non-power-of-two",
            width: 257,
            height: 193,
            pattern: 1,
            expected: [true, true, true],
        },
        RateCase {
            label: "structured even non-power-of-two",
            width: 258,
            height: 194,
            pattern: 0,
            expected: [true, true, true],
        },
    ];
    let mut workspace = Part1DecodeWorkspace::new();
    for case in cases {
        for (index, rate) in [1.0, 2.0, 4.0].into_iter().enumerate() {
            let result = try_encoded_u16_greyscale(case.width, case.height, case.pattern, rate);
            assert_eq!(
                result.is_ok(),
                case.expected[index],
                "{} at {rate} bpp: {result:?}",
                case.label
            );
            let Ok((_, raw)) = result else {
                assert_encoder_rate_unattainable(&result.unwrap_err());
                continue;
            };
            assert_eq!(
                try_encoded_u16_greyscale(case.width, case.height, case.pattern, rate)
                    .unwrap()
                    .1,
                raw,
                "non-deterministic encoder output for {} at {rate} bpp",
                case.label
            );

            let full_options = options(ComponentLayout::Planar);
            let full_raw = decode(&raw, &full_options).unwrap();
            let jph = wrapped_dimensions(&raw, case.width, case.height, 16, 1);
            assert_eq!(
                container::parse(&jph)
                    .unwrap()
                    .primary_codestream(&jph)
                    .unwrap(),
                Some(raw.as_slice())
            );
            assert_eq!(decode(&jph, &full_options).unwrap(), full_raw);

            for discard_levels in [1, 2] {
                let divisor = 1_u32 << discard_levels;
                let reduced_width = case.width.div_ceil(divisor);
                let reduced_height = case.height.div_ceil(divisor);
                assert_reduced_u16_routes(
                    &raw,
                    discard_levels,
                    reduced_width,
                    reduced_height,
                    &mut workspace,
                );
            }
        }
    }

    let (_, raw) = try_encoded_u16_greyscale(1024, 1024, 1, 2.0).unwrap();
    let image = assert_reduced_u16_routes(&raw, 2, 256, 256, &mut workspace);
    assert_eq!((image.info.width, image.info.height), (256, 256));
    assert_reduced_u16_routes(&raw, 1, 512, 512, &mut workspace);
}

#[test]
fn lossy_ht_u16_greyscale_reduced_rejects_neighbours_atomically() {
    let (_, raw) = encoded_u16_greyscale(257, 193, 2.0);
    let admitted = reduced_u16_options(1);
    let info = decode_partial_info(&raw, &admitted).unwrap();

    let assert_atomic_rejection =
        |input: &[u8], request: &PartialDecodeOptions, target_info: &ImageInfo| {
            let stride = target_info.width as usize * 2 + 5;
            let mut buffer = vec![0xa5; stride * target_info.height as usize + 3];
            {
                let plane = PlaneMut::new(
                    &mut buffer,
                    target_info.width,
                    target_info.height,
                    stride,
                    SampleFormat::U16_LE,
                )
                .unwrap();
                let mut planes = [plane];
                assert!(
                    decode_partial_into(
                        input,
                        &mut ImageViewMut::Planar {
                            info: target_info,
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

    assert!(decode_partial_info(&raw, &reduced_u16_options(2)).is_ok());
    for discard_levels in [0, 3, 4, 5, 6] {
        let request = reduced_u16_options(discard_levels);
        assert!(decode_partial_info(&raw, &request).is_err());
        assert!(decode_partial_component_info(&raw, &request).is_err());
        assert!(decode_partial(&raw, &request).is_err());
        assert_atomic_rejection(&raw, &request, &info);
    }

    let mut requests = vec![PartialDecodeOptions::default()];
    requests.push(PartialDecodeOptions {
        components: ComponentSelection::All,
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        components: ComponentSelection::Indices(Vec::new()),
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        components: ComponentSelection::Indices(vec![0, 0]),
        ..admitted.clone()
    });
    requests.push(PartialDecodeOptions {
        components: ComponentSelection::Indices(vec![1]),
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
        assert_atomic_rejection(&raw, &request, &info);
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
    assert_atomic_rejection(&jph, &admitted, &info);
    let (_, u8_raw) = encoded(8, 1, 2.0);
    assert!(decode_partial_info(&u8_raw, &admitted).is_err());
    assert_atomic_rejection(&u8_raw, &admitted, &info);
    let (_, rgb_raw) = encoded(16, 3, 2.0);
    assert!(decode_partial_info(&rgb_raw, &admitted).is_err());
    assert_atomic_rejection(&rgb_raw, &admitted, &info);

    let mut signed = raw.clone();
    signed[42] |= 0x80;
    assert!(decode_partial_info(&signed, &admitted).is_err());
    assert_atomic_rejection(&signed, &admitted, &info);
    let parsed = codestream::parse(&raw).unwrap();
    assert!(decode(&raw, &DecodeOptions::default()).is_err());
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
        assert_atomic_rejection(&other_decomposition, &admitted, &info);
    }
    let siz = parsed
        .markers
        .iter()
        .find(|marker| marker.marker == codestream::Marker::Siz)
        .unwrap()
        .offset;
    let qcd = parsed
        .markers
        .iter()
        .find(|marker| marker.marker == codestream::Marker::Qcd)
        .unwrap()
        .offset;
    let mut neighbours = Vec::new();
    for (offset, value) in [
        (cod + 8, 1),
        (cod + 10, 3),
        (cod + 12, 0),
        (cod + 13, 1),
        (qcd + 4, 0x42),
        (siz + 5, 1),
        (siz + 17, 1),
    ] {
        let mut neighbour = raw.clone();
        neighbour[offset] = value;
        neighbours.push(neighbour);
    }
    let mut extra_layer = raw.clone();
    extra_layer[cod + 6] = 0;
    extra_layer[cod + 7] = 2;
    neighbours.push(extra_layer);
    for neighbour in neighbours {
        assert!(decode_partial_info(&neighbour, &admitted).is_err());
        assert!(decode_partial(&neighbour, &admitted).is_err());
        assert_atomic_rejection(&neighbour, &admitted, &info);
    }
    let sod = raw
        .windows(2)
        .position(|bytes| bytes == codestream::Marker::Sod.code().to_be_bytes())
        .unwrap();
    let mut invalid_packet = raw.clone();
    invalid_packet[sod + 2] ^= 0x40;
    assert!(decode_partial_info(&invalid_packet, &admitted).is_err());
    assert_atomic_rejection(&invalid_packet, &admitted, &info);
    let truncated = &raw[..raw.len() - 1];
    assert!(decode_partial_info(truncated, &admitted).is_err());
    assert_atomic_rejection(truncated, &admitted, &info);

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
    for discard_levels in [1, 2] {
        let request = reduced_u16_options(discard_levels);
        let target_info = decode_partial_info(&raw, &request).unwrap();
        let retained_resolution = 2 - discard_levels;
        let retained = contributions
            .iter()
            .find(|contribution| {
                contribution.component_index == 0
                    && contribution.resolution <= retained_resolution
                    && contribution.coding_passes == 1
                    && contribution.codeword_len >= 2
            })
            .unwrap();
        let cleanup_end = payload_offset + retained.payload_offset + retained.codeword_len;
        let mut late = raw.clone();
        late[cleanup_end - 2] &= 0xf0;
        late[cleanup_end - 1] = 0;
        assert!(decode_partial_info(&late, &request).is_ok());
        assert!(decode_partial(&late, &request).is_err());
        assert_atomic_rejection(&late, &request, &target_info);

        let mut workspace = Part1DecodeWorkspace::new();
        let mut failed = vec![0xa5; target_info.width as usize * target_info.height as usize * 2];
        {
            let plane = PlaneMut::new(
                &mut failed,
                target_info.width,
                target_info.height,
                target_info.width as usize * 2,
                SampleFormat::U16_LE,
            )
            .unwrap();
            let mut planes = [plane];
            assert!(
                decode_partial_into_with_workspace(
                    &late,
                    &mut ImageViewMut::Planar {
                        info: &target_info,
                        planes: &mut planes,
                    },
                    &request,
                    &mut workspace,
                )
                .is_err()
            );
        }
        assert!(failed.iter().all(|byte| *byte == 0xa5));
        assert_reduced_u16_routes(
            &raw,
            discard_levels,
            target_info.width,
            target_info.height,
            &mut workspace,
        );
    }
}
