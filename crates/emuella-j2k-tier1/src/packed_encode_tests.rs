use super::*;
use alloc::vec;

fn scratch(packed: bool, trace: bool) -> CodeBlockEncodeScratch {
    let mut scratch = CodeBlockEncodeScratch::new();
    scratch.backend_override = Some(packed);
    scratch.trace.enabled = trace;
    scratch.packed.trace.enabled = trace;
    scratch
}

fn coefficients(width: usize, height: usize, planes: u8, pattern: usize) -> Vec<i32> {
    let mask = if planes >= 31 {
        i32::MAX as u32
    } else {
        (1 << planes) - 1
    };
    let mut random = 0xb374_19d2_u32;
    let mut result = (0..width * height)
        .map(|index| {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            let (x, y) = (index % width, index / width);
            let active = match pattern % 5 {
                0 => true,
                1 => index.is_multiple_of(31),
                2 => (x / 4 + y / 4).is_multiple_of(2),
                3 => x == y || x + y == width - 1,
                _ => x == width / 2 || y == height / 2 || y % 4 == 3,
            };
            let magnitude = if active { (random & mask) as i32 } else { 0 };
            if random & (1 << 31) == 0 {
                magnitude
            } else {
                -magnitude
            }
        })
        .collect::<Vec<_>>();
    result[0] = if planes == 32 { i32::MIN } else { mask as i32 };
    result
}

fn spec(width: u16, height: u16, planes: u8, style: u8, subband: Subband) -> CodeBlockEncodeSpec {
    CodeBlockEncodeSpec {
        dimensions: CodeBlockDimensions::new(width, height).unwrap(),
        subband,
        available_bitplanes: planes,
        code_block_style: style,
    }
}

fn assert_trace_equal(reference: &[encode_trace::Event], packed: &[encode_trace::Event]) {
    assert_eq!(
        reference.len(),
        packed.len(),
        "decision/boundary event count"
    );
    for (index, (reference, packed)) in reference.iter().zip(packed).enumerate() {
        assert_eq!(reference, packed, "trace event {index}");
    }
}

#[test]
fn encoder_build_selection_routes_real_entry_points_and_scratch() {
    let forced_reference = option_env!("EMUELLA_TIER1_ENCODER") == Some("reference");
    let source = coefficients(17, 11, 16, 0);
    for style in [0, 1] {
        let spec = spec(17, 11, 16, style, Subband::HighLow);
        let mut expected = vec![0x23, 0xff];
        let expected_result = encode_baseline_code_block_with_scratch(
            &source,
            spec,
            &mut expected,
            &mut scratch(false, false),
        )
        .unwrap();
        for route in 0..5 {
            // Use ordinary construction: the build selection must own routing.
            let mut work = CodeBlockEncodeScratch::new();
            let mut actual = vec![0x23, 0xff];
            let mut lengths = Vec::new();
            let result = match route {
                0 => encode_baseline_code_block_with_scratch(&source, spec, &mut actual, &mut work),
                1 => encode_baseline_code_block_segments_with_scratch(
                    &source,
                    spec,
                    &mut actual,
                    &mut lengths,
                    &mut work,
                ),
                2 => encode_baseline_code_block_with_known_max_scratch(
                    &source,
                    65535,
                    spec,
                    &mut actual,
                    &mut work,
                ),
                3 => encode_baseline_code_block_with_strided_scratch(
                    &source,
                    17,
                    spec,
                    &mut actual,
                    &mut work,
                ),
                _ => encode_baseline_code_block_segments_with_strided_scratch(
                    &source,
                    17,
                    spec,
                    &mut actual,
                    &mut lengths,
                    &mut work,
                ),
            }
            .unwrap();
            assert_eq!(result, expected_result);
            assert_eq!(actual, expected);
            assert!(work.backend_override.is_none());
            if forced_reference {
                assert!(!work.coefficient_states.is_empty());
                assert_eq!(work.packed.capacities(), [0; 4]);
            } else {
                assert!(work.coefficient_states.is_empty());
                assert!(
                    work.packed
                        .capacities()
                        .into_iter()
                        .all(|capacity| capacity > 0)
                );
            }
        }
    }
}

#[test]
fn packed_encoder_matches_reference_decisions_bytes_segments_and_reconstruction() {
    let shapes = [
        (1, 1),
        (1, 64),
        (64, 1),
        (3, 3),
        (4, 4),
        (15, 3),
        (16, 4),
        (17, 5),
        (31, 7),
        (32, 8),
        (33, 9),
        (63, 63),
        (64, 64),
        (17, 11),
    ];
    let bands = [
        Subband::LowLow,
        Subband::LowHigh,
        Subband::HighLow,
        Subband::HighHigh,
    ];
    let mut reference = scratch(false, true);
    let mut packed = scratch(true, true);
    let mut stuffed = 0;
    let mut raw = 0;
    for (shape, &(width, height)) in shapes.iter().enumerate() {
        for (plane_index, planes) in [1, 4, 5, 8, 16, 24, 30, 31, 32].into_iter().enumerate() {
            for (band, subband) in bands.into_iter().enumerate() {
                let source = coefficients(
                    width as usize,
                    height as usize,
                    planes,
                    shape + plane_index + band,
                );
                for style in [0, 1] {
                    let spec = spec(width, height, planes, style, subband);
                    let mut expected = vec![0x79, 0xff];
                    let mut actual = expected.clone();
                    let mut expected_lengths = vec![usize::MAX];
                    let mut actual_lengths = expected_lengths.clone();
                    let expected_result = encode_baseline_code_block_segments_with_scratch(
                        &source,
                        spec,
                        &mut expected,
                        &mut expected_lengths,
                        &mut reference,
                    )
                    .unwrap();
                    let actual_result = encode_baseline_code_block_segments_with_scratch(
                        &source,
                        spec,
                        &mut actual,
                        &mut actual_lengths,
                        &mut packed,
                    )
                    .unwrap();
                    assert_eq!(expected_result, actual_result, "{spec:?}");
                    assert_eq!(expected, actual, "{spec:?}");
                    assert_eq!(expected_lengths, actual_lengths, "{spec:?}");
                    assert_eq!(actual_lengths.iter().sum::<usize>(), actual_result.byte_len);
                    assert_trace_equal(&reference.trace.events, &packed.packed.trace.events);
                    mq::oracle_tests::assert_trace(&packed.packed.trace.events, &actual[2..]);
                    stuffed += usize::from(actual[2..].contains(&0xff));
                    raw += packed
                        .packed
                        .trace
                        .events
                        .iter()
                        .filter(|event| matches!(event, encode_trace::Event::Raw { .. }))
                        .count();
                    if planes <= 30 {
                        let segments = super::tests::segment_descriptors(
                            CodeBlockStyle::from_bits(style),
                            &actual_lengths,
                            actual_result.pass_count,
                        );
                        let decode_spec = CodeBlockDecodeSpec {
                            dimensions: spec.dimensions,
                            subband,
                            available_bitplanes: planes,
                            missing_most_significant_bitplanes: actual_result.missing_bitplanes,
                            coding_passes: actual_result.pass_count,
                            style: CodeBlockStyle::from_bits(style),
                        };
                        let mut decoded = vec![0; source.len()];
                        decode_baseline_code_block_segments(
                            &actual[2..],
                            &segments,
                            decode_spec,
                            &mut decoded,
                        )
                        .unwrap();
                        assert_eq!(decoded, source);
                    }
                }
            }
        }
    }
    assert!(stuffed > 0);
    assert!(raw > 0);
}

#[test]
fn packed_encoder_entry_points_preserve_strides_prefixes_zeroes_and_known_maximum() {
    for style in [0, 1] {
        let spec = spec(17, 11, 16, style, Subband::HighLow);
        let source = coefficients(17, 11, 16, 0);
        let mut strided = vec![i32::MIN; 23 * 11];
        for y in 0..11 {
            strided[y * 23..y * 23 + 17].copy_from_slice(&source[y * 17..y * 17 + 17]);
        }
        let mut expected = vec![0x23, 0xff];
        let result = encode_baseline_code_block_with_scratch(
            &source,
            spec,
            &mut expected,
            &mut scratch(false, false),
        )
        .unwrap();
        for route in 0..5 {
            let mut actual = vec![0x23, 0xff];
            let mut lengths = vec![123];
            let mut packed = scratch(true, false);
            let observed = match route {
                0 => {
                    encode_baseline_code_block_with_scratch(&source, spec, &mut actual, &mut packed)
                }
                1 => encode_baseline_code_block_segments_with_scratch(
                    &source,
                    spec,
                    &mut actual,
                    &mut lengths,
                    &mut packed,
                ),
                2 => encode_baseline_code_block_with_known_max_scratch(
                    &source,
                    65535,
                    spec,
                    &mut actual,
                    &mut packed,
                ),
                3 => encode_baseline_code_block_with_strided_scratch(
                    &strided,
                    23,
                    spec,
                    &mut actual,
                    &mut packed,
                ),
                _ => encode_baseline_code_block_segments_with_strided_scratch(
                    &strided,
                    23,
                    spec,
                    &mut actual,
                    &mut lengths,
                    &mut packed,
                ),
            }
            .unwrap();
            assert_eq!(observed, result);
            assert_eq!(actual, expected);
            assert!(packed.coefficient_states.is_empty());
            if matches!(route, 1 | 4) {
                assert_eq!(lengths.iter().sum::<usize>(), observed.byte_len);
            }
        }
        for selected in [false, true] {
            let mut scratch = scratch(selected, false);
            let mut output = vec![0x87];
            // A caller-supplied zero is an existing fast path, even for nonzero source.
            let zero = encode_baseline_code_block_with_known_max_scratch(
                &source,
                0,
                spec,
                &mut output,
                &mut scratch,
            )
            .unwrap();
            assert!(!zero.included);
            assert_eq!(zero.missing_bitplanes, 16);
            assert_eq!(output, [0x87]);
            let mut lengths = vec![123];
            let zero = encode_baseline_code_block_segments_with_strided_scratch(
                &[0; 23 * 11],
                23,
                spec,
                &mut output,
                &mut lengths,
                &mut scratch,
            )
            .unwrap();
            assert!(!zero.included);
            assert!(lengths.is_empty());
            assert_eq!(output, [0x87]);
        }
    }
}

#[test]
fn packed_encoder_missing_planes_and_empty_blocks_preserve_metadata() {
    for style in [0, 1] {
        for available in [8, 16, 31, 32, 255] {
            let spec = spec(3, 5, available, style, Subband::LowHigh);
            for value in [0, 1, -127] {
                let source = [value; 15];
                let mut expected = vec![0xff];
                let mut actual = expected.clone();
                let mut a_lengths = vec![999];
                let mut b_lengths = a_lengths.clone();
                let mut reference = scratch(false, true);
                let mut packed = scratch(true, true);
                let a = encode_baseline_code_block_segments_with_scratch(
                    &source,
                    spec,
                    &mut expected,
                    &mut a_lengths,
                    &mut reference,
                )
                .unwrap();
                let b = encode_baseline_code_block_segments_with_scratch(
                    &source,
                    spec,
                    &mut actual,
                    &mut b_lengths,
                    &mut packed,
                )
                .unwrap();
                assert_eq!(a, b);
                assert_eq!(expected, actual);
                assert_eq!(a_lengths, b_lengths);
                assert_trace_equal(&reference.trace.events, &packed.packed.trace.events);
                mq::oracle_tests::assert_trace(&packed.packed.trace.events, &actual[1..]);
                if value == 0 {
                    assert_eq!(actual, [0xff]);
                    assert_eq!(b.missing_bitplanes, available);
                    assert!(!b.included);
                    assert_eq!(packed.packed.capacities(), [0; 4]);
                } else {
                    assert_eq!(
                        b.missing_bitplanes,
                        available - if value == 1 { 1 } else { 7 }
                    );
                }
            }
        }
    }
}

#[test]
fn packed_encoder_falls_back_for_wide_shapes_and_every_other_accepted_style() {
    for (width, height) in [(65, 3), (3, 65), (4096, 1), (1, 4096), (17, 11)] {
        for style in 0..64 {
            let spec = spec(width, height, 8, style, Subband::HighHigh);
            if packed_encode::eligible(spec) {
                continue;
            }
            let source = coefficients(width as usize, height as usize, 8, 2);
            let mut expected = Vec::new();
            let mut actual = Vec::new();
            let mut packed = scratch(true, true);
            let mut reference = scratch(false, true);
            let a = encode_baseline_code_block_with_scratch(
                &source,
                spec,
                &mut expected,
                &mut reference,
            )
            .unwrap();
            let b =
                encode_baseline_code_block_with_scratch(&source, spec, &mut actual, &mut packed)
                    .unwrap();
            assert_eq!(a, b);
            assert_eq!(expected, actual);
            assert_trace_equal(&reference.trace.events, &packed.trace.events);
            assert_eq!(packed.packed.capacities(), [0; 4]);
        }
    }
}

#[test]
fn packed_encoder_validation_and_reuse_after_errors_match_reference() {
    let valid = spec(4, 4, 8, 1, Subband::LowLow);
    for selected in [false, true] {
        let mut work = scratch(selected, false);
        let mut output = vec![0x71; 7];
        let mut lengths = vec![99];
        for (source, stride, spec) in [
            (&[0; 16][..], 3, valid),
            (&[0; 15][..], 4, valid),
            (&[0; 16][..], usize::MAX, valid),
            (
                &[0; 16][..],
                4,
                CodeBlockEncodeSpec {
                    available_bitplanes: 0,
                    ..valid
                },
            ),
            (
                &[0; 16][..],
                4,
                CodeBlockEncodeSpec {
                    code_block_style: 64,
                    ..valid
                },
            ),
            (&[256; 16][..], 4, valid),
        ] {
            let mut expected = output.clone();
            let mut expected_lengths = vec![99];
            let reference = encode_baseline_code_block_segments_with_strided_scratch(
                source,
                stride,
                spec,
                &mut expected,
                &mut expected_lengths,
                &mut scratch(false, false),
            );
            let actual = encode_baseline_code_block_segments_with_strided_scratch(
                source,
                stride,
                spec,
                &mut output,
                &mut lengths,
                &mut work,
            );
            assert_eq!(actual, reference);
            assert!(actual.is_err());
            assert_eq!(output, expected);
            assert_eq!(lengths, expected_lengths);
            assert!(lengths.is_empty());
        }
        let mut expected = output.clone();
        let source = coefficients(4, 4, 8, 0);
        let a = encode_baseline_code_block_with_scratch(
            &source,
            valid,
            &mut expected,
            &mut scratch(false, false),
        )
        .unwrap();
        let b = encode_baseline_code_block_with_scratch(&source, valid, &mut output, &mut work)
            .unwrap();
        assert_eq!(a, b);
        assert_eq!(expected, output);
    }
}

#[test]
fn packed_encoder_retained_capacity_fits_the_existing_worker_allowance() {
    let mut work = scratch(false, false);
    let mut bytes = Vec::new();
    let mut lengths = Vec::new();
    let mut peak_scratch = 0;
    for (width, height) in [(1, 1), (1, 64), (64, 1), (17, 11), (33, 63), (64, 64)] {
        for selected in [false, true] {
            for style in [0, 1] {
                work.backend_override = Some(selected);
                bytes.clear();
                let source = coefficients(width, height, 31, 0);
                encode_baseline_code_block_segments_with_scratch(
                    &source,
                    spec(width as u16, height as u16, 31, style, Subband::HighHigh),
                    &mut bytes,
                    &mut lengths,
                    &mut work,
                )
                .unwrap();
                let packed = work.packed.capacities();
                assert!(packed[..3].iter().all(|&capacity| capacity <= 66 * 66));
                assert!(packed[3] <= 64);
                let reference = work.coefficient_states.capacity()
                    * core::mem::size_of::<CoefficientState>()
                    + work.signs.capacity()
                    + 4 * work.magnitudes.capacity();
                let retained = reference
                    + 2 * packed[0]
                    + packed[1]
                    + 4 * packed[2]
                    + 5 * core::mem::size_of::<u64>() * packed[3]
                    + core::mem::size_of::<CodeBlockEncodeScratch>();
                peak_scratch = peak_scratch.max(retained);
                // Also cover a conservative previous scratch allocation while growing.
                assert!(retained + 4 * 2 * 66 * 66 < 192 * 1024);
                assert!(bytes.len() < 1024 * 1024);
                assert!(lengths.len() <= 55);
                assert!(
                    retained
                        + bytes.capacity() * 2
                        + lengths.capacity() * core::mem::size_of::<usize>()
                        + 4096
                        < 4 * 1024 * 1024
                );
            }
        }
    }
    let capacities = work.packed.capacities();
    let reference = [
        work.coefficient_states.capacity(),
        work.signs.capacity(),
        work.magnitudes.capacity(),
    ];
    for (width, height) in [(63, 63), (17, 11), (1, 64), (64, 1), (1, 1), (64, 64)] {
        work.clear();
        for selected in [false, true] {
            work.backend_override = Some(selected);
            bytes.clear();
            encode_baseline_code_block_with_scratch(
                &vec![1; width * height],
                spec(width as u16, height as u16, 1, 0, Subband::LowLow),
                &mut bytes,
                &mut work,
            )
            .unwrap();
            assert_eq!(work.packed.capacities(), capacities);
            assert_eq!(
                [
                    work.coefficient_states.capacity(),
                    work.signs.capacity(),
                    work.magnitudes.capacity()
                ],
                reference
            );
        }
    }
    assert!(peak_scratch > 0);
}
