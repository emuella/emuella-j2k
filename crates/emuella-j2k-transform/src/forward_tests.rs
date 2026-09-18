use super::*;
use alloc::{vec, vec::Vec};

// Retain the original single-column traversal independently of the candidate's
// const-generic body. Checked lifting below supplies a separate arithmetic oracle.
fn original_columns(plane: &mut [i32], config: Reversible53Config, scratch: &mut [i32]) {
    let axis = config.width.max(config.height);
    let (_, rest) = scratch.split_at_mut(axis);
    let (read, coeffs) = rest.split_at_mut(axis);
    for x in 0..config.width {
        copy_strided_column_to_line(plane, config.stride, x, &mut read[..config.height]);
        transform_line_forward_from_read_to_strided_bounded(
            config.height,
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &read[..config.height],
            &mut coeffs[..config.height],
            plane,
            config.stride,
            x,
        );
    }
    for y in 0..config.height {
        let start = y * config.stride;
        transform_line_forward_bounded(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut coeffs[..config.width],
        );
    }
}

#[test]
fn paired_columns_match_original_and_checked_with_exact_scratch() {
    for (width, height) in [
        (1, 1),
        (1, 17),
        (18, 1),
        (2, 2),
        (3, 5),
        (4, 4),
        (7, 8),
        (8, 7),
        (63, 65),
        (64, 64),
        (65, 63),
        (129, 127),
    ] {
        for (origin_x, origin_y) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            for padding in [0, 5] {
                let config = Reversible53Config {
                    width,
                    height,
                    stride: width + padding,
                    edges: Reversible53Edges::from_tile_origin(origin_x, origin_y, width, height),
                    sample_range: ComponentSampleRange::signed(32),
                };
                let source: Vec<i32> = (0..(height - 1) * config.stride + width)
                    .map(|n| match n % 5 {
                        0 => -65535,
                        1 => 65535,
                        2 => -1,
                        3 => 0,
                        _ => ((n * 7919 + n / 7) % 131071) as i32 - 65535,
                    })
                    .collect();
                let mut candidate = source.clone();
                let mut original = source.clone();
                let mut checked = source;
                let required = config.scratch_len();
                let mut candidate_storage = vec![0x123456; required + 18];
                let mut reference_scratch = vec![0; required];
                forward_reversible_5_3_bounded_paired_columns(
                    &mut candidate,
                    config,
                    &mut candidate_storage[9..9 + required],
                )
                .unwrap();
                original_columns(&mut original, config, &mut reference_scratch);
                forward_reversible_5_3(&mut checked, config, &mut reference_scratch).unwrap();
                assert_eq!(
                    candidate, original,
                    "original {width}x{height}, phase {origin_x},{origin_y}"
                );
                assert_eq!(
                    candidate, checked,
                    "checked {width}x{height}, phase {origin_x},{origin_y}"
                );
                assert_eq!(&candidate_storage[..9], &[0x123456; 9]);
                assert_eq!(&candidate_storage[9 + required..], &[0x123456; 9]);
            }
        }
    }
}

#[test]
fn paired_columns_preserve_two_level_ll_stride_and_other_subbands() {
    for (width, height) in [(4, 4), (5, 7), (64, 65), (65, 64), (129, 131)] {
        let source: Vec<i32> = (0..width * height)
            .map(|n| ((n * 65521 + n / 3) % 131071) as i32 - 65535)
            .collect();
        let mut candidate = source.clone();
        let mut original = source.clone();
        let mut checked = source;
        let mut scratch = vec![0; 3 * width.max(height)];
        for level in 0..2 {
            let active_width = width.div_ceil(1 << level);
            let active_height = height.div_ceil(1 << level);
            let config = Reversible53Config {
                width: active_width,
                height: active_height,
                stride: width,
                edges: Reversible53Edges::from_tile_origin(0, 0, active_width, active_height),
                sample_range: ComponentSampleRange::signed(32),
            };
            forward_reversible_5_3_bounded_paired_columns(&mut candidate, config, &mut scratch)
                .unwrap();
            original_columns(&mut original, config, &mut scratch);
            forward_reversible_5_3(&mut checked, config, &mut scratch).unwrap();
            assert_eq!(candidate, original);
            assert_eq!(candidate, checked);
        }
    }
}

#[test]
fn paired_columns_preserve_validation_and_failure_atomicity() {
    let valid = Reversible53Config {
        width: 4,
        height: 5,
        stride: 4,
        edges: Reversible53Edges::from_tile_origin(0, 0, 4, 5),
        sample_range: ComponentSampleRange::signed(8),
    };
    for (config, length, scratch_length, value) in [
        (valid, 20, valid.scratch_len() - 1, 0),
        (valid, 19, valid.scratch_len(), 0),
        (
            Reversible53Config { stride: 3, ..valid },
            20,
            valid.scratch_len(),
            0,
        ),
        (
            Reversible53Config { width: 0, ..valid },
            20,
            valid.scratch_len(),
            0,
        ),
        (valid, 20, valid.scratch_len(), 128),
    ] {
        let source = vec![value; length];
        let mut candidate = source.clone();
        let mut original = source.clone();
        let mut scratch = vec![0; scratch_length];
        let a = forward_reversible_5_3_bounded_paired_columns(&mut candidate, config, &mut scratch);
        let b = forward_reversible_5_3_bounded(&mut original, config, &mut scratch);
        assert!(a.is_err());
        assert_eq!(a, b);
        assert_eq!(candidate, source);
        assert_eq!(original, source);
    }
}
