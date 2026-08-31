//! Independent display vectors for the bounded JP2 presentation contract.
use emuella_j2k_core::*;
use emuella_j2k_test_support::native_planes::{
    codestream, empty_codestream, jp2, jp2_box, late_invalid_segment,
};

fn palette(columns: &[u8], values: &[u8]) -> Vec<u8> {
    let row_bytes: usize = columns
        .iter()
        .map(|v| usize::from((v & 127) + 1).div_ceil(8))
        .sum();
    let mut bytes = u16::try_from(values.len() / row_bytes)
        .unwrap()
        .to_be_bytes()
        .to_vec();
    bytes.push(columns.len() as u8);
    bytes.extend(columns);
    bytes.extend(values);
    jp2_box(*b"pclr", &bytes)
}
fn mapping(entries: &[(u16, u8, u8)]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &(component, kind, column) in entries {
        bytes.extend(component.to_be_bytes());
        bytes.extend([kind, column]);
    }
    jp2_box(*b"cmap", &bytes)
}
fn definitions(entries: &[(u16, u16, u16)]) -> Vec<u8> {
    let mut bytes = (entries.len() as u16).to_be_bytes().to_vec();
    for &(channel, kind, association) in entries {
        bytes.extend(channel.to_be_bytes());
        bytes.extend(kind.to_be_bytes());
        bytes.extend(association.to_be_bytes());
    }
    jp2_box(*b"cdef", &bytes)
}
fn repeat(values: &[u8]) -> Vec<u8> {
    values.iter().copied().cycle().take(15).collect()
}
fn make(planes: &[Vec<u8>], colour: u32, extra: &[u8]) -> Vec<u8> {
    let refs = planes.iter().map(Vec::as_slice).collect::<Vec<_>>();
    jp2(
        &codestream(5, 3, &refs),
        5,
        3,
        planes.len() as u16,
        colour,
        extra,
    )
}
fn rgba_def() -> Vec<u8> {
    definitions(&[(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 1, 0)])
}
fn caller(input: &[u8], options: &DecodeOptions, expected: &Image, failure: bool) {
    let width = expected.info.width as usize;
    let height = expected.info.height as usize;
    for reusable in [false, true] {
        let mut workspace = Part1DecodeWorkspace::new();
        let mut run = |target: &mut ImageViewMut<'_>| {
            // Caller target layout governs, even when the option requests its opposite.
            let options = DecodeOptions {
                target_layout: if options.target_layout == ComponentLayout::Planar {
                    ComponentLayout::Interleaved
                } else {
                    ComponentLayout::Planar
                },
                ..options.clone()
            };
            let result = if reusable {
                decode_into_with_workspace(input, target, &options, &mut workspace)
            } else {
                decode_into(input, target, &options)
            };
            if failure {
                assert!(result.is_err());
            } else {
                result.unwrap();
            }
        };
        match &expected.data {
            ImageData::Planes(expected_planes) => {
                let stride = width + 3;
                let mut buffers = vec![vec![0x6d; stride * height + 7]; expected_planes.len()];
                let mut oracle = buffers.clone();
                if !failure {
                    for (buffer, plane) in oracle.iter_mut().zip(expected_planes) {
                        for y in 0..height {
                            buffer[y * stride..y * stride + width]
                                .copy_from_slice(&plane[y * width..(y + 1) * width]);
                        }
                    }
                }
                {
                    let mut planes = buffers
                        .iter_mut()
                        .map(|bytes| {
                            PlaneMut::new(
                                bytes,
                                width as u32,
                                height as u32,
                                stride,
                                SampleFormat::U8,
                            )
                            .unwrap()
                        })
                        .collect::<Vec<_>>();
                    run(&mut ImageViewMut::Planar {
                        info: &expected.info,
                        planes: &mut planes,
                    });
                }
                assert_eq!(buffers, oracle);
            }
            ImageData::Interleaved(expected_bytes) => {
                let row = width * usize::from(expected.info.components);
                let stride = row + 5;
                let mut buffer = vec![0x6d; stride * height + 7];
                let mut oracle = buffer.clone();
                if !failure {
                    for y in 0..height {
                        oracle[y * stride..y * stride + row]
                            .copy_from_slice(&expected_bytes[y * row..(y + 1) * row]);
                    }
                }
                run(&mut ImageViewMut::Interleaved {
                    info: &expected.info,
                    samples: &mut buffer,
                    stride_bytes: stride,
                });
                assert_eq!(buffer, oracle);
            }
        }
    }
}
fn oracle(planes: &[Vec<u8>], model: ColorModel, layout: ComponentLayout) -> Image {
    let info = ImageInfo::new(5, 3, planes.len() as u16, SampleFormat::U8, model, layout).unwrap();
    let data = match layout {
        ComponentLayout::Planar => ImageData::Planes(planes.to_vec()),
        ComponentLayout::Interleaved => ImageData::Interleaved(
            (0..15)
                .flat_map(|i| planes.iter().map(move |p| p[i]))
                .collect(),
        ),
    };
    Image {
        info,
        data,
        component_info: Vec::new(),
    }
}
fn check(input: &[u8], native: &[Vec<u8>], displayed: &[Vec<u8>], model: ColorModel) {
    assert_eq!(
        inspect(input, &InspectOptions::default()).unwrap().support,
        SupportStatus::Supported
    );
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        let options = DecodeOptions {
            target_layout: layout,
            ..DecodeOptions::default()
        };
        let expected = oracle(displayed, model, layout);
        let shape = decode_shape(input, &options).unwrap();
        assert_eq!(shape.codestream_components, native.len() as u16);
        assert_eq!(shape.colour_channels, displayed.len() as u16);
        assert_eq!(shape.output_components, displayed.len() as u16);
        assert_eq!(shape.mode, DecodeMode::Rendered);
        assert_eq!(shape.byte_order, None);
        assert_eq!(
            ImageInfo::new(
                shape.width,
                shape.height,
                shape.output_components,
                shape.sample_format,
                shape.color_model,
                shape.layout
            )
            .unwrap(),
            expected.info
        );
        let image = decode(input, &options).unwrap();
        assert_eq!(image.info, expected.info);
        assert_eq!(image.data, expected.data);
        assert_eq!(image.component_info.len(), displayed.len());
        for c in &image.component_info {
            assert_eq!(
                (
                    c.width,
                    c.height,
                    c.x_origin,
                    c.y_origin,
                    c.horizontal_separation,
                    c.vertical_separation,
                    c.sample_format,
                    c.source_component
                ),
                (5, 3, 0, 0, 1, 1, SampleFormat::U8, None)
            );
        }
        caller(input, &options, &expected, false);
        let native_options = DecodeOptions {
            mode: DecodeMode::Components,
            ..options
        };
        let decoded_native = decode(input, &native_options).unwrap();
        assert_eq!(
            decoded_native.data,
            oracle(native, ColorModel::Unknown, layout).data
        );
        for (index, c) in decoded_native.component_info.iter().enumerate() {
            assert_eq!(c.source_component, Some(index as u16));
        }
    }
}
fn rejected(input: &[u8], malformed: bool, planes: &[Vec<u8>], model: ColorModel) {
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        let options = DecodeOptions {
            target_layout: layout,
            ..DecodeOptions::default()
        };
        for result in [
            decode(input, &options).map(|_| ()),
            decode_shape(input, &options).map(|_| ()),
        ] {
            if malformed {
                assert!(
                    matches!(
                        result,
                        Err(J2kError::InvalidInput { .. }) | Err(J2kError::TruncatedInput { .. })
                    ),
                    "{result:?}"
                );
            } else {
                assert!(
                    matches!(result, Err(J2kError::Unsupported { .. })),
                    "{result:?}"
                );
            }
        }
        caller(input, &options, &oracle(planes, model, layout), true);
    }
}

#[test]
fn palettes_expand_greyscale_rgb_and_table_boundaries() {
    let indices = vec![repeat(&[0, 2, 1])];
    let grey = [palette(&[7], &[91, 7, 203]), mapping(&[(0, 1, 0)])].concat();
    check(
        &make(&indices, 17, &grey),
        &indices,
        &[repeat(&[91, 203, 7])],
        ColorModel::Grayscale,
    );
    let rgb = [
        palette(&[7, 7, 7], &[91, 3, 17, 7, 129, 255, 203, 48, 0]),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2)]),
    ]
    .concat();
    check(
        &make(&indices, 16, &rgb),
        &indices,
        &[
            repeat(&[91, 203, 7]),
            repeat(&[3, 48, 129]),
            repeat(&[17, 0, 255]),
        ],
        ColorModel::Rgb,
    );
    for rows in [1, 256, 1024] {
        let native = vec![repeat(if rows == 1 { &[0] } else { &[0, 255] })];
        let values = (0..rows)
            .map(|i| if i == 255 { 29 } else { 231 })
            .collect::<Vec<_>>();
        let extra = [palette(&[7], &values), mapping(&[(0, 1, 0)])].concat();
        check(
            &make(&native, 17, &extra),
            &native,
            &[repeat(if rows == 1 { &[231] } else { &[231, 29] })],
            ColorModel::Grayscale,
        );
    }
}

fn permutations(values: &[u16]) -> Vec<Vec<u16>> {
    if values.is_empty() {
        return vec![vec![]];
    }
    let mut results = Vec::new();
    for (index, first) in values.iter().enumerate() {
        let rest = values
            .iter()
            .enumerate()
            .filter_map(|(i, v)| (i != index).then_some(*v))
            .collect::<Vec<_>>();
        for mut suffix in permutations(&rest) {
            suffix.insert(0, *first);
            results.push(suffix);
        }
    }
    results
}

#[test]
fn direct_mapping_and_channel_definition_permutations() {
    let native = vec![
        repeat(&[9, 42, 211]),
        repeat(&[101, 3, 64]),
        repeat(&[27, 199, 8]),
        repeat(&[0, 255, 128]),
    ];
    for order in permutations(&[0, 1, 2, 3]) {
        // Logical channel order varies independently of source-plane order.
        let maps = order
            .iter()
            .map(|source| (*source, 0, 0))
            .collect::<Vec<_>>();
        let defs = order
            .iter()
            .enumerate()
            .map(|(channel, source)| {
                (
                    channel as u16,
                    if *source == 3 { 1 } else { 0 },
                    if *source == 3 { 0 } else { *source + 1 },
                )
            })
            .collect::<Vec<_>>();
        let extra = [palette(&[7], &[0]), mapping(&maps), definitions(&defs)].concat();
        check(
            &make(&native, 16, &extra),
            &native,
            &native,
            ColorModel::Rgba,
        );
        // Without cmap, cdef still addresses logical channels and reorders RGB/A.
        let reordered = order
            .iter()
            .map(|i| native[usize::from(*i)].clone())
            .collect::<Vec<_>>();
        check(
            &make(&reordered, 16, &definitions(&defs)),
            &reordered,
            &native,
            ColorModel::Rgba,
        );
    }
    for order in permutations(&[0, 1, 2]) {
        let maps = order
            .iter()
            .map(|source| (*source, 0, 0))
            .collect::<Vec<_>>();
        let expected = order
            .iter()
            .map(|source| native[usize::from(*source)].clone())
            .collect::<Vec<_>>();
        let extra = [palette(&[7], &[99]), mapping(&maps)].concat();
        check(
            &make(&native[..3], 16, &extra),
            &native[..3],
            &expected,
            ColorModel::Rgb,
        );
        if order != [0, 1, 2] {
            let defs = order
                .iter()
                .enumerate()
                .map(|(channel, source)| (channel as u16, 0, *source + 1))
                .collect::<Vec<_>>();
            let reordered = order
                .iter()
                .map(|i| native[usize::from(*i)].clone())
                .collect::<Vec<_>>();
            check(
                &make(&reordered, 16, &definitions(&defs)),
                &reordered,
                &native[..3],
                ColorModel::Rgb,
            );
        }
    }
}

#[test]
fn palette_column_source_and_channel_permutations() {
    let native = vec![repeat(&[0, 2, 1]), repeat(&[2, 1, 0]), repeat(&[1, 0, 2])];
    let table = palette(&[7, 7, 7], &[91, 3, 17, 7, 129, 255, 203, 48, 0]);
    // Independent display vectors for each source's index sequence and each column.
    let display = [
        [
            repeat(&[91, 203, 7]),
            repeat(&[3, 48, 129]),
            repeat(&[17, 0, 255]),
        ],
        [
            repeat(&[203, 7, 91]),
            repeat(&[48, 129, 3]),
            repeat(&[0, 255, 17]),
        ],
        [
            repeat(&[7, 91, 203]),
            repeat(&[129, 3, 48]),
            repeat(&[255, 17, 0]),
        ],
    ];
    for sources in permutations(&[0, 1, 2]) {
        for columns in permutations(&[0, 1, 2]) {
            let maps = (0..3)
                .map(|i| (sources[i], 1, columns[i] as u8))
                .collect::<Vec<_>>();
            let expected = (0..3)
                .map(|i| display[sources[i] as usize][columns[i] as usize].clone())
                .collect::<Vec<_>>();
            check(
                &make(&native, 16, &[table.clone(), mapping(&maps)].concat()),
                &native,
                &expected,
                ColorModel::Rgb,
            );
        }
    }
    let extra = [table, mapping(&[(2, 1, 1), (2, 1, 1), (2, 1, 0)])].concat();
    check(
        &make(&native, 16, &extra),
        &native,
        &[
            display[2][1].clone(),
            display[2][1].clone(),
            display[2][0].clone(),
        ],
        ColorModel::Rgb,
    );
}

#[test]
fn mixed_mapping_and_direct_or_palette_straight_alpha() {
    let native = vec![
        repeat(&[0, 2, 1]),
        repeat(&[19, 97, 218]),
        repeat(&[0, 255, 128]),
    ];
    let table = palette(&[7, 7, 7], &[3, 17, 0, 129, 255, 128, 48, 0, 255]);
    for alpha in [(2, 0, 0), (0, 1, 2)] {
        let extra = [
            table.clone(),
            mapping(&[(1, 0, 0), (0, 1, 0), (0, 1, 1), alpha]),
            rgba_def(),
        ]
        .concat();
        let expected = [
            native[1].clone(),
            repeat(&[3, 48, 129]),
            repeat(&[17, 0, 255]),
            repeat(&[0, 255, 128]),
        ];
        check(
            &make(&native, 16, &extra),
            &native,
            &expected,
            ColorModel::Rgba,
        );
    }
    let grey_native = vec![repeat(&[19, 97, 218]), repeat(&[0, 255, 128])];
    let expected = [
        grey_native[0].clone(),
        grey_native[0].clone(),
        grey_native[0].clone(),
        grey_native[1].clone(),
    ];
    check(
        &make(&grey_native, 17, &definitions(&[(0, 0, 1), (1, 1, 0)])),
        &grey_native,
        &expected,
        ColorModel::Rgba,
    );
    let indices = vec![repeat(&[0, 2, 1])];
    let extra = [
        palette(
            &[7, 7, 7, 7],
            &[9, 101, 27, 0, 211, 64, 8, 128, 42, 3, 199, 255],
        ),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)]),
        rgba_def(),
    ]
    .concat();
    check(
        &make(&indices, 16, &extra),
        &indices,
        &[
            repeat(&[9, 42, 211]),
            repeat(&[101, 3, 64]),
            repeat(&[27, 199, 8]),
            repeat(&[0, 255, 128]),
        ],
        ColorModel::Rgba,
    );
    let extra = [
        palette(&[7, 7], &[19, 0, 218, 128, 97, 255]),
        mapping(&[(0, 1, 0), (0, 1, 1)]),
        definitions(&[(0, 0, 1), (1, 1, 0)]),
    ]
    .concat();
    check(
        &make(&indices, 17, &extra),
        &indices,
        &[
            repeat(&[19, 97, 218]),
            repeat(&[19, 97, 218]),
            repeat(&[19, 97, 218]),
            repeat(&[0, 255, 128]),
        ],
        ColorModel::Rgba,
    );
}

#[test]
fn redundant_and_shared_channel_descriptions_are_resolved() {
    let native = vec![repeat(&[27, 199, 8]), repeat(&[0, 255, 128])];
    let defs = definitions(&[
        (0, 0, 1),
        (0, 0, 2),
        (0, 0, 3),
        (1, 1, 0),
        (0, 0, 2),
        (1, 1, 0),
    ]);
    check(
        &make(&native, 16, &defs),
        &native,
        &[
            native[0].clone(),
            native[0].clone(),
            native[0].clone(),
            native[1].clone(),
        ],
        ColorModel::Rgba,
    );
}

#[test]
fn literal_packets_supply_an_encoder_independent_palette_family() {
    let raw = empty_codestream(5, 3, 1);
    let mut values = vec![9; 129 * 4];
    values[128 * 4..].copy_from_slice(&[31, 209, 74, 0]);
    let extra = [
        palette(&[7; 4], &values),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)]),
        rgba_def(),
    ]
    .concat();
    check(
        &jp2(&raw, 5, 3, 1, 16, &extra),
        &[vec![128; 15]],
        &[vec![31; 15], vec![209; 15], vec![74; 15], vec![0; 15]],
        ColorModel::Rgba,
    );
}

#[test]
fn out_of_table_indices_and_late_entropy_failure_are_atomic() {
    let native = vec![repeat(&[0, 1, 2])];
    let extra = [palette(&[7], &[17, 81]), mapping(&[(0, 1, 0)])].concat();
    let input = make(&native, 17, &extra);
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        let options = DecodeOptions {
            target_layout: layout,
            ..DecodeOptions::default()
        };
        assert!(decode_shape(&input, &options).is_ok());
        assert!(
            matches!(decode(&input,&options),Err(J2kError::Unsupported{detail,..}) if detail.contains("indeterminate"))
        );
        caller(
            &input,
            &options,
            &oracle(&[vec![17; 15]], ColorModel::Grayscale, layout),
            true,
        );
        assert_eq!(
            decode(
                &input,
                &DecodeOptions {
                    mode: DecodeMode::Components,
                    ..options
                }
            )
            .unwrap()
            .data,
            oracle(&native, ColorModel::Unknown, layout).data
        );
    }
    let raw = late_invalid_segment(5, 3, &[&[17; 15], &[81; 15], &[129; 15]]);
    let input = jp2(&raw, 5, 3, 4, 16, &rgba_def());
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        let options = DecodeOptions {
            target_layout: layout,
            ..DecodeOptions::default()
        };
        assert!(decode_shape(&input, &options).is_ok());
        assert!(matches!(
            decode(&input, &options),
            Err(J2kError::InvalidInput { .. })
        ));
        caller(
            &input,
            &options,
            &oracle(&vec![vec![17; 15]; 4], ColorModel::Rgba, layout),
            true,
        );
    }
}

#[test]
fn malformed_mapping_palette_and_roles_are_distinct_from_unsupported_features() {
    let native = vec![repeat(&[0, 1, 2])];
    let good_palette = palette(&[7], &[19, 31, 92]);
    for extra in [
        good_palette.clone(),
        mapping(&[(0, 0, 0)]),
        [good_palette.clone(), mapping(&[(1, 1, 0)])].concat(),
        [good_palette.clone(), mapping(&[(0, 1, 1)])].concat(),
        [good_palette.clone(), mapping(&[(0, 0, 1)])].concat(),
        [good_palette.clone(), mapping(&[(0, 2, 0)])].concat(),
        [palette(&[0x87], &[19, 31, 92]), mapping(&[(0, 1, 0)])].concat(),
        [jp2_box(*b"pclr", &[0, 3, 1, 7, 19]), mapping(&[(0, 1, 0)])].concat(),
    ] {
        rejected(
            &make(&native, 17, &extra),
            true,
            &[vec![0; 15]],
            ColorModel::Grayscale,
        );
    }
    let four = vec![vec![9; 15]; 4];
    for defs in [
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3)],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 4), (3, 1, 0)],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (4, 1, 0)],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 0, 1)],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 1, 0), (3, 2, 0)],
    ] {
        rejected(
            &make(&four, 16, &definitions(&defs)),
            true,
            &four,
            ColorModel::Rgba,
        );
    }
    for defs in [
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 2, 0)],
        vec![
            (0, 0, 1),
            (1, 0, 2),
            (2, 0, 3),
            (3, 1, 1),
            (3, 1, 2),
            (3, 1, 3),
        ],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, u16::MAX, u16::MAX)],
        vec![(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 1, u16::MAX)],
    ] {
        let input = make(&four, 16, &definitions(&defs));
        rejected(&input, false, &four, ColorModel::Rgba);
        assert!(
            decode(
                &input,
                &DecodeOptions {
                    mode: DecodeMode::Components,
                    ..DecodeOptions::default()
                }
            )
            .is_ok()
        );
    }
    let high = make(
        &native,
        17,
        &[palette(&[8], &[0, 19, 0, 31, 0, 92]), mapping(&[(0, 1, 0)])].concat(),
    );
    rejected(&high, false, &[vec![0; 15]], ColorModel::Grayscale);
}

#[test]
fn coding_neighbours_and_expanded_output_limits_fail_before_allocation() {
    let raw = codestream(5, 3, &[&[0; 15]]);
    let extra = [
        palette(&[7, 7, 7], &[19, 31, 92]),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2)]),
    ]
    .concat();
    let cod = raw.windows(2).position(|v| v == [255, 82]).unwrap();
    let mut rlcp = raw.clone();
    rlcp[cod + 5] = 1;
    let sot = raw.windows(2).position(|v| v == [255, 144]).unwrap();
    let mut registration = raw.clone();
    registration.splice(sot..sot, [255, 99, 0, 6, 0, 0, 0, 0]);
    for bytes in [rlcp, registration] {
        rejected(
            &jp2(&bytes, 5, 3, 1, 16, &extra),
            false,
            &vec![vec![0; 15]; 3],
            ColorModel::Rgb,
        );
    }
    // Native allocation remains below its bound, while RGB expansion exceeds it.
    let large = jp2(&empty_codestream(4096, 2048, 1), 4096, 2048, 1, 16, &extra);
    assert!(
        matches!(decode_shape(&large,&DecodeOptions::default()),Err(J2kError::Unsupported{detail,..}) if detail.contains("expanded output"))
    );
    assert!(
        matches!(decode(&large,&DecodeOptions::default()),Err(J2kError::Unsupported{detail,..}) if detail.contains("expanded output"))
    );
    let mut buffer = vec![0x6d; 15];
    let info = ImageInfo::new(
        5,
        3,
        1,
        SampleFormat::U8,
        ColorModel::Grayscale,
        ComponentLayout::Interleaved,
    )
    .unwrap();
    assert!(
        decode_into(
            &large,
            &mut ImageViewMut::Interleaved {
                info: &info,
                samples: &mut buffer,
                stride_bytes: 5
            },
            &DecodeOptions::default()
        )
        .is_err()
    );
    assert_eq!(buffer, vec![0x6d; 15]);
}

#[test]
fn first_colour_controls_and_rendered_request_neighbours_remain_closed() {
    let native = vec![repeat(&[0, 1, 2])];
    let extra = [palette(&[7], &[19, 31, 92]), mapping(&[(0, 1, 0)])].concat();
    let good = make(&native, 17, &extra);
    let mut alternate = vec![1, 0, 0];
    alternate.extend(17_u32.to_be_bytes());
    let second = jp2_box(*b"colr", &alternate);
    // colr boxes must be consecutive: insert the second before optional metadata.
    for first in [17, 99] {
        let input = make(&native, first, &[second.clone(), extra.clone()].concat());
        rejected(&input, false, &[vec![0; 15]], ColorModel::Grayscale);
    }
    for options in [
        DecodeOptions {
            requested_components: ComponentSelection::Indices(vec![0]),
            ..DecodeOptions::default()
        },
        DecodeOptions {
            max_quality_layers: Some(1),
            ..DecodeOptions::default()
        },
    ] {
        assert!(matches!(
            decode(&good, &options),
            Err(J2kError::Unsupported { .. })
        ));
        assert!(matches!(
            decode_shape(&good, &options),
            Err(J2kError::Unsupported { .. })
        ));
        caller(
            &good,
            &options,
            &oracle(
                &[vec![0; 15]],
                ColorModel::Grayscale,
                ComponentLayout::Planar,
            ),
            true,
        );
    }
    for options in [
        PartialDecodeOptions::default(),
        PartialDecodeOptions {
            resolution: ResolutionLevel::Reduced { discard_levels: 1 },
            ..PartialDecodeOptions::default()
        },
    ] {
        assert!(decode_rendered_partial(&good, &options).is_err());
        assert!(decode_rendered_partial_info(&good, &options).is_err());
    }
}

#[test]
fn signed_result_validity_does_not_change_direct_native_admission() {
    fn signed(components: u16, colour: u32, extra: &[u8]) -> Vec<u8> {
        let mut raw = empty_codestream(5, 3, components);
        let siz = raw.windows(2).position(|b| b == [255, 81]).unwrap();
        for component in 0..usize::from(components) {
            raw[siz + 40 + 3 * component] = 0x87;
        }
        let mut input = jp2(&raw, 5, 3, components, colour, extra);
        let ihdr = input.windows(4).position(|b| b == b"ihdr").unwrap();
        input[ihdr + 14] = 0x87;
        input
    }
    let plain = signed(1, 17, &[]);
    // Existing direct/native metadata admission remains unchanged.
    inspect(&plain, &InspectOptions::default()).unwrap();
    assert!(matches!(
        decode(&plain, &DecodeOptions::default()),
        Err(J2kError::Unsupported { .. })
    ));
    let direct = signed(
        1,
        17,
        &[palette(&[7], &[0]), mapping(&[(0, 0, 0)])].concat(),
    );
    rejected(&direct, true, &[vec![0; 15]], ColorModel::Grayscale);
    let reordered = signed(3, 16, &definitions(&[(0, 0, 2), (1, 0, 1), (2, 0, 3)]));
    rejected(&reordered, true, &vec![vec![0; 15]; 3], ColorModel::Rgb);
    let index = signed(
        1,
        17,
        &[palette(&[7], &[81; 129]), mapping(&[(0, 1, 0)])].concat(),
    );
    // Signed native indices do not make their unsigned palette results malformed.
    inspect(&index, &InspectOptions::default()).unwrap();
    rejected(&index, false, &[vec![0; 15]], ColorModel::Grayscale);
    let native = vec![repeat(&[0, 1, 2])];
    let signed_alpha = [
        palette(&[7, 0x87], &[19, 0, 31, 0, 92, 0]),
        mapping(&[(0, 1, 0), (0, 1, 1)]),
        definitions(&[(0, 0, 1), (1, 1, 0)]),
    ]
    .concat();
    rejected(
        &make(&native, 17, &signed_alpha),
        true,
        &vec![vec![0; 15]; 4],
        ColorModel::Rgba,
    );
}

#[test]
fn opacity_conflicts_extra_channels_and_reserved_definitions_are_not_rendered() {
    let native = vec![vec![9; 15]; 4];
    for alpha in [
        vec![(3, 1, 0), (0, 1, 1)],
        vec![(0, 1, 1), (3, 1, 0)],
        vec![(3, 1, 0), (3, 2, 1)],
    ] {
        let mut defs = vec![(0, 0, 1), (1, 0, 2), (2, 0, 3)];
        defs.extend(alpha);
        rejected(
            &make(&native, 16, &definitions(&defs)),
            true,
            &native,
            ColorModel::Rgba,
        );
    }
    let five = [
        palette(&[7], &[0]),
        mapping(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 0, 0)]),
        definitions(&[
            (0, 0, 1),
            (1, 0, 2),
            (2, 0, 3),
            (3, 1, 0),
            (4, u16::MAX, u16::MAX),
        ]),
    ]
    .concat();
    rejected(&make(&native, 16, &five), false, &native, ColorModel::Rgba);
    let reserved = definitions(&[(0, 0, 1), (1, 0, 2), (2, 0, 3), (3, 3, 0)]);
    rejected(
        &make(&native, 16, &reserved),
        true,
        &native,
        ColorModel::Rgba,
    );
    let mut multiple = make(&native, 16, &rgba_def());
    multiple.extend(jp2_box(*b"jp2c", &empty_codestream(5, 3, 4)));
    rejected(&multiple, false, &native, ColorModel::Rgba);
}

#[test]
fn palette_limits_and_reordered_palette_channels_use_full_field_domains() {
    let native = vec![repeat(&[0, 1, 2])];
    let mut values = vec![0; 3 * 255];
    values[254] = 91;
    values[509] = 7;
    values[764] = 203;
    check(
        &make(
            &native,
            17,
            &[palette(&[7; 255], &values), mapping(&[(0, 1, 254)])].concat(),
        ),
        &native,
        &[repeat(&[91, 7, 203])],
        ColorModel::Grayscale,
    );
    let direct = [palette(&[7], &[99]), mapping(&[(0, 0, 0)])].concat();
    check(
        &make(&native, 17, &direct),
        &native,
        &native,
        ColorModel::Grayscale,
    );
    let table = palette(&[7; 4], &[9, 101, 27, 0, 42, 3, 199, 255, 211, 64, 8, 128]);
    let expected = [
        repeat(&[9, 42, 211]),
        repeat(&[101, 3, 64]),
        repeat(&[27, 199, 8]),
        repeat(&[0, 255, 128]),
    ];
    for columns in permutations(&[0, 1, 2, 3]) {
        let maps = columns
            .iter()
            .map(|column| (0, 1, *column as u8))
            .collect::<Vec<_>>();
        let defs = columns
            .iter()
            .enumerate()
            .map(|(channel, column)| {
                (
                    channel as u16,
                    if *column == 3 { 1 } else { 0 },
                    if *column == 3 { 0 } else { *column + 1 },
                )
            })
            .collect::<Vec<_>>();
        check(
            &make(
                &native,
                16,
                &[table.clone(), mapping(&maps), definitions(&defs)].concat(),
            ),
            &native,
            &expected,
            ColorModel::Rgba,
        );
    }
    let extra = [
        palette(&[7; 4], &[9, 101, 27, 0]),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)]),
        rgba_def(),
    ]
    .concat();
    let edge = jp2(&empty_codestream(2048, 2048, 1), 2048, 2048, 1, 16, &extra);
    assert_eq!(
        decode_shape(&edge, &DecodeOptions::default())
            .unwrap()
            .output_components,
        4
    );
}

#[test]
fn projection_handles_multiple_blocks_and_odd_image_edges() {
    let indices = (0..65 * 67).map(|i| (i % 3) as u8).collect::<Vec<_>>();
    let raw = codestream::encode_planar_u8_no_decomp_test_fixture(65, 67, &[&indices]).unwrap();
    let extra = [
        palette(&[7; 4], &[9, 101, 27, 0, 42, 3, 199, 255, 211, 64, 8, 128]),
        mapping(&[(0, 1, 0), (0, 1, 1), (0, 1, 2), (0, 1, 3)]),
        rgba_def(),
    ]
    .concat();
    let input = jp2(&raw, 65, 67, 1, 16, &extra);
    let rows = [[9, 101, 27, 0], [42, 3, 199, 255], [211, 64, 8, 128]];
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        let options = DecodeOptions {
            target_layout: layout,
            ..DecodeOptions::default()
        };
        let expected = Image {
            info: ImageInfo::new(65, 67, 4, SampleFormat::U8, ColorModel::Rgba, layout).unwrap(),
            component_info: Vec::new(),
            data: match layout {
                ComponentLayout::Planar => ImageData::Planes(
                    (0..4)
                        .map(|c| (0..65 * 67).map(|i| rows[i % 3][c]).collect())
                        .collect(),
                ),
                ComponentLayout::Interleaved => {
                    ImageData::Interleaved((0..65 * 67).flat_map(|i| rows[i % 3]).collect())
                }
            },
        };
        let result = decode(&input, &options).unwrap();
        assert_eq!(result.info, expected.info);
        assert_eq!(result.data, expected.data);
        caller(&input, &options, &expected, false);
    }
}

#[test]
fn unspecified_descriptions_do_not_justify_default_colour_definitions() {
    for (count, colour, model) in [(1, 17, ColorModel::Grayscale), (3, 16, ColorModel::Rgb)] {
        let native = vec![vec![128; 15]; count];
        let known = (0..count)
            .map(|channel| (channel as u16, 0, channel as u16 + 1))
            .collect::<Vec<_>>();
        for order in permutations(&(0..count as u16).collect::<Vec<_>>()) {
            for association in [0, 1, u16::MAX] {
                for repeated in [false, true] {
                    let mut descriptions = order
                        .iter()
                        .map(|index| known[usize::from(*index)])
                        .collect::<Vec<_>>();
                    descriptions.push((0, u16::MAX, association));
                    if repeated {
                        descriptions.extend(descriptions.clone());
                    }
                    for reversed in [false, true] {
                        let mut entries = descriptions.clone();
                        if reversed {
                            entries.reverse();
                        }
                        let input = make(&native, colour, &definitions(&entries));
                        assert!(
                            matches!(container::parse(&input), Err(container::ContainerError::InvalidBox { message, .. }) if message.contains("must omit"))
                        );
                        assert!(
                            matches!(inspect(&input, &InspectOptions::default()), Err(J2kError::InvalidInput { message, .. }) if message.contains("must omit"))
                        );
                        rejected(&input, true, &native, model);
                    }
                }
            }
        }
    }
    // An actual extra channel or a reordered colour channel still needs cdef.
    // Its unspecified description remains structurally valid but unrendered.
    for (native, colour, entries) in [
        (
            vec![vec![128; 15]; 2],
            17,
            vec![(0, 0, 1), (1, u16::MAX, u16::MAX)],
        ),
        (
            vec![vec![128; 15]; 3],
            16,
            vec![(0, 0, 2), (1, 0, 1), (2, 0, 3), (0, u16::MAX, u16::MAX)],
        ),
        (
            vec![vec![128; 15]; 2],
            16,
            vec![(0, 0, 1), (0, 0, 2), (0, 0, 3), (1, u16::MAX, u16::MAX)],
        ),
    ] {
        for reversed in [false, true] {
            let mut entries = entries.clone();
            if reversed {
                entries.reverse();
            }
            let input = make(&native, colour, &definitions(&entries));
            container::parse(&input).unwrap();
            rejected(&input, false, &vec![vec![0; 15]; 3], ColorModel::Rgb);
        }
    }
}

#[test]
fn unassociated_opacity_types_are_not_colour_association_conflicts() {
    let native = vec![vec![128; 15]; 3];
    let base = [(0, 0, 1), (1, 1, u16::MAX), (2, 2, u16::MAX)];
    for order in permutations(&[0, 1, 2]) {
        for repeated in [false, true] {
            let mut entries = order
                .iter()
                .map(|index| base[usize::from(*index)])
                .collect::<Vec<_>>();
            if repeated {
                entries.extend(entries.clone());
            }
            let input = make(&native, 17, &definitions(&entries));
            container::parse(&input).unwrap();
            assert!(matches!(
                inspect(&input, &InspectOptions::default()).unwrap().support,
                SupportStatus::Unsupported { .. }
            ));
            rejected(&input, false, &native, ColorModel::Rgb);
            let decoded = decode(
                &input,
                &DecodeOptions {
                    mode: DecodeMode::Components,
                    target_layout: ComponentLayout::Planar,
                    ..DecodeOptions::default()
                },
            )
            .unwrap();
            assert_eq!(decoded.data, ImageData::Planes(native.clone()));
        }
    }
    for association in [0, 1] {
        for reversed in [false, true] {
            let mut entries = vec![(0, 0, 1), (1, 1, association), (2, 2, association)];
            if reversed {
                entries.reverse();
            }
            let input = make(&native, 17, &definitions(&entries));
            assert!(
                matches!(container::parse(&input),Err(container::ContainerError::InvalidBox{message,..}) if message.contains("cannot mix opacity"))
            );
            rejected(&input, true, &native, ColorModel::Rgb);
        }
    }
}
