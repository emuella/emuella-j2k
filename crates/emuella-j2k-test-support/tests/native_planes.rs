use emuella_j2k_core::{
    ColorModel, ComponentLayout, ComponentSelection, DecodeMode, DecodeOptions, Image, ImageData,
    ImageViewMut, J2kError, Part1DecodeWorkspace, PartialDecodeOptions, PlaneMut, SampleFormat,
    decode, decode_into, decode_into_with_workspace, decode_shape, prepare_part1_decode,
};
use emuella_j2k_test_support::native_planes::{
    codestream, empty_codestream, jp2, jp2_box, late_invalid_segment,
};

fn options(layout: ComponentLayout) -> DecodeOptions {
    DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: layout,
        ..DecodeOptions::default()
    }
}

fn wrap(raw: &[u8], width: u32, height: u32, count: usize) -> Vec<u8> {
    let mut cdef = Vec::new();
    if count == 2 || count == 4 {
        cdef.extend((count as u16).to_be_bytes());
        for channel in 0..count {
            cdef.extend((channel as u16).to_be_bytes());
            cdef.extend(if channel + 1 == count { 1_u16 } else { 0 }.to_be_bytes());
            cdef.extend(
                if channel + 1 == count {
                    0_u16
                } else {
                    channel as u16 + 1
                }
                .to_be_bytes(),
            );
        }
        cdef = jp2_box(*b"cdef", &cdef);
    }
    jp2(
        raw,
        width,
        height,
        count as u16,
        if count >= 3 { 16 } else { 17 },
        &cdef,
    )
}

fn assert_caller_output(input: &[u8], options: &DecodeOptions, expected: &Image, fail: bool) {
    let width = expected.info.width as usize;
    let height = expected.info.height as usize;
    // Exercise both workspace and ordinary adapters, including their padding.
    for reusable in [false, true] {
        let mut workspace = Part1DecodeWorkspace::new();
        let mut run = |target: &mut ImageViewMut<'_>| {
            let result = if reusable {
                decode_into_with_workspace(input, target, options, &mut workspace)
            } else {
                decode_into(input, target, options)
            };
            if fail {
                assert!(result.is_err());
            } else {
                result.unwrap();
            }
        };
        match &expected.data {
            ImageData::Planes(expected_planes) => {
                let stride = width + 3;
                let mut buffers = vec![vec![0x6d; stride * height + 7]; expected_planes.len()];
                let original = buffers.clone();
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
                if fail {
                    assert_eq!(buffers, original);
                } else {
                    for (buffer, plane) in buffers.iter().zip(expected_planes) {
                        for y in 0..height {
                            assert_eq!(
                                &buffer[y * stride..y * stride + width],
                                &plane[y * width..(y + 1) * width]
                            );
                            assert!(
                                buffer[y * stride + width..(y + 1) * stride]
                                    .iter()
                                    .all(|v| *v == 0x6d)
                            );
                        }
                        assert!(buffer[stride * height..].iter().all(|v| *v == 0x6d));
                    }
                }
            }
            ImageData::Interleaved(expected_bytes) => {
                let row_bytes = width * usize::from(expected.info.components);
                let stride = row_bytes + 5;
                let mut buffer = vec![0x6d; stride * height + 7];
                run(&mut ImageViewMut::Interleaved {
                    info: &expected.info,
                    samples: &mut buffer,
                    stride_bytes: stride,
                });
                if fail {
                    assert!(buffer.iter().all(|v| *v == 0x6d));
                } else {
                    for y in 0..height {
                        assert_eq!(
                            &buffer[y * stride..y * stride + row_bytes],
                            &expected_bytes[y * row_bytes..(y + 1) * row_bytes]
                        );
                        assert!(
                            buffer[y * stride + row_bytes..(y + 1) * stride]
                                .iter()
                                .all(|v| *v == 0x6d)
                        );
                    }
                    assert!(buffer[stride * height..].iter().all(|v| *v == 0x6d));
                }
            }
        }
    }
}

#[test]
fn hand_built_native_planes_have_independently_specified_samples() {
    let planes = [
        vec![
            0, 1, 127, 128, 255, 254, 2, 129, 17, 250, 3, 200, 60, 130, 9,
        ],
        vec![255, 0, 64, 32, 16, 8, 4, 2, 1, 0, 128, 200, 17, 19, 23],
        vec![7, 90, 33, 21, 200, 87, 6, 55, 44, 3, 2, 1, 250, 124, 93],
        vec![
            0, 128, 255, 0, 255, 64, 255, 128, 0, 255, 0, 128, 64, 32, 16,
        ],
    ];
    for count in 1..=4 {
        let refs = planes[..count]
            .iter()
            .map(Vec::as_slice)
            .collect::<Vec<_>>();
        let raw = codestream(5, 3, &refs);
        for input in [&raw, &wrap(&raw, 5, 3, count)] {
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                for selection in [
                    ComponentSelection::All,
                    ComponentSelection::Indices((0..count as u16).rev().collect()),
                ] {
                    let selected: Vec<_> = match &selection {
                        ComponentSelection::All => (0..count).collect(),
                        ComponentSelection::Indices(indices) => {
                            indices.iter().map(|i| usize::from(*i)).collect()
                        }
                    };
                    let options = DecodeOptions {
                        requested_components: selection.clone(),
                        ..options(layout)
                    };
                    let image = decode(input, &options).unwrap();
                    let shape = decode_shape(input, &options).unwrap();
                    assert_eq!(
                        (
                            shape.width,
                            shape.height,
                            shape.codestream_components,
                            shape.colour_channels,
                            shape.output_components
                        ),
                        (5, 3, count as u16, count as u16, count as u16)
                    );
                    assert_eq!(
                        (
                            image.info.width,
                            image.info.height,
                            image.info.components,
                            image.info.layout,
                            image.info.sample_format,
                            image.info.color_model
                        ),
                        (
                            shape.width,
                            shape.height,
                            shape.output_components,
                            shape.layout,
                            shape.sample_format,
                            shape.color_model
                        )
                    );
                    assert_eq!(shape.sample_format, SampleFormat::U8);
                    assert_eq!(shape.byte_order, None);
                    if selection != ComponentSelection::All || count == 2 || count == 4 {
                        assert_eq!(shape.color_model, ColorModel::Unknown);
                    }
                    for (component, source) in image.component_info.iter().zip(&selected) {
                        assert_eq!(
                            (
                                component.width,
                                component.height,
                                component.x_origin,
                                component.y_origin,
                                component.horizontal_separation,
                                component.vertical_separation
                            ),
                            (5, 3, 0, 0, 1, 1)
                        );
                        assert_eq!(component.source_component, Some(*source as u16));
                        assert_eq!(component.sample_format, SampleFormat::U8);
                    }
                    let expected = match layout {
                        ComponentLayout::Planar => {
                            ImageData::Planes(selected.iter().map(|i| planes[*i].clone()).collect())
                        }
                        ComponentLayout::Interleaved => ImageData::Interleaved(
                            (0..15)
                                .flat_map(|i| {
                                    selected.iter().map(|p| planes[*p][i]).collect::<Vec<_>>()
                                })
                                .collect(),
                        ),
                    };
                    assert_eq!(image.data, expected, "count={count}, layout={layout:?}");
                    assert_caller_output(input, &options, &image, false);
                }
            }
        }
        let empty = decode(
            &empty_codestream(5, 3, count as u16),
            &options(ComponentLayout::Planar),
        )
        .unwrap();
        assert_eq!(empty.data, ImageData::Planes(vec![vec![128; 15]; count]));
    }
}

#[test]
fn late_native_entropy_failure_preserves_caller_planes() {
    for count in 2..=4 {
        let first = [17; 15];
        let preceding = vec![first.as_slice(); count - 1];
        let valid = codestream(5, 3, &vec![first.as_slice(); count]);
        let broken = late_invalid_segment(5, 3, &preceding);
        for (good, bad) in [
            (&valid, &broken),
            (&wrap(&valid, 5, 3, count), &wrap(&broken, 5, 3, count)),
        ] {
            // Packet preparation succeeds: this fails during final-plane entropy
            // reconstruction, after earlier planes could already have committed.
            prepare_part1_decode(bad, &PartialDecodeOptions::default()).unwrap();
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                let options = options(layout);
                let expected = decode(good, &options).unwrap();
                assert_eq!(decode_shape(bad, &options), decode_shape(good, &options));
                let failure = decode(bad, &options).unwrap_err();
                assert!(
                    matches!(failure, J2kError::InvalidInput { .. }),
                    "{failure:?}"
                );
                assert_caller_output(bad, &options, &expected, true);
            }
        }
    }
}

#[test]
fn multiple_blocks_and_odd_edges_preserve_native_values() {
    // Additional geometry evidence uses the existing project image fixture
    // utility, with expected values still specified independently by this test.
    let planes = (0..4)
        .map(|c| {
            (0..65 * 67)
                .map(|i| ((i * 19 + c * 31) % 256) as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let refs = planes.iter().map(Vec::as_slice).collect::<Vec<_>>();
    let raw = emuella_j2k_core::codestream::encode_planar_u8_no_decomp_test_fixture(65, 67, &refs)
        .unwrap();
    for input in [&raw, &wrap(&raw, 65, 67, 4)] {
        let options = options(ComponentLayout::Planar);
        let image = decode(input, &options).unwrap();
        assert_eq!(image.data, ImageData::Planes(planes.clone()));
        assert_caller_output(input, &options, &image, false);
    }
}

#[test]
fn malformed_headers_and_unsupported_registration_preserve_callers() {
    let planes = vec![vec![17; 15]; 4];
    let refs = planes.iter().map(Vec::as_slice).collect::<Vec<_>>();
    let raw = codestream(5, 3, &refs);
    let marker = |code| raw.windows(2).position(|v| v == [255, code]).unwrap();
    let mut bad_length = raw.clone();
    let sot = marker(0x90);
    bad_length[sot + 6..sot + 10].copy_from_slice(&u32::MAX.to_be_bytes());
    let mut bad_qcd = raw.clone();
    bad_qcd[marker(0x5c) + 4] = 0x23;
    let mut registration = raw.clone();
    let mut crg = vec![255, 99, 0, 18];
    crg.extend([0; 16]);
    registration.splice(sot..sot, crg);
    for (candidate, unsupported) in [(bad_length, false), (bad_qcd, false), (registration, true)] {
        for input in [&candidate, &wrap(&candidate, 5, 3, 4)] {
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                let options = options(layout);
                let result = decode(input, &options);
                if unsupported {
                    assert!(matches!(result, Err(J2kError::Unsupported { .. })));
                } else {
                    assert!(result.is_err());
                }
                assert!(decode_shape(input, &options).is_err());
                let expected = decode(&raw, &options).unwrap();
                assert_caller_output(input, &options, &expected, true);
            }
        }
    }
}
