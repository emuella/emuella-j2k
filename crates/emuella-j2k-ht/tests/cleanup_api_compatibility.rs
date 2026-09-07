use emuella_j2k_ht::*;

#[test]
fn legacy_unsuffixed_literal_caller_still_compiles() {
    // Keep this downstream call unannotated: the established type fixes u16.
    let block = HtBlockLayout::new(HtCodeBlockDimensions::new(1, 1).unwrap());
    let outputs = [HtVlcCleanupCoefficientOutput {
        position: block.coefficient_position(0, 0).unwrap(),
        significant: false,
        magnitude_sign_bits: 0,
        magnitude_sign_value: 0,
        embedded_magnitude_bit: false,
        magnitude_exponent_reduction: false,
    }];
    let _ = block.materialize_vlc_cleanup_coefficient_outputs(&outputs, 29, &mut [0]);
    let mut coefficients = [123];
    let progress = block
        .materialize_vlc_cleanup_coefficient_outputs_with_progress(&outputs, 29, &mut coefficients)
        .unwrap()
        .unwrap();
    progress.check_complete().unwrap();
    assert_eq!(coefficients, [0]);
}

#[test]
fn legacy_empty_storage_calls_need_no_magnitude_type_annotation() {
    let candidate = plan_decode_candidate(HtMarkerState {
        has_cap_marker: true,
        components: 1,
        sample_precision_bits: 8,
        all_components_same_sample_format: true,
        all_components_unit_sampled: true,
        single_tile: true,
        has_tile_part: true,
        packet_progression_supported: true,
        reversible_transform: true,
        sop_markers: false,
        eph_markers: false,
        precincts_declared: false,
        code_block_width: Some(1),
        code_block_height: Some(1),
    })
    .unwrap();
    let block = candidate.block();
    let layout = HtBlockScratchLayout::new(block).unwrap();
    let mut scratch = vec![0; layout.total_words()];
    let mut mel = HtMelEventState::new();
    let mut contexts = [HtVlcContextProgression::initial()];
    let mut coefficients = [123];
    assert!(
        block
            .materialize_vlc_cleanup_coefficient_outputs(&[], 29, &mut coefficients)
            .is_err()
    );
    assert!(
        block
            .materialize_vlc_cleanup_coefficient_outputs_with_progress(&[], 29, &mut coefficients)
            .is_err()
    );
    let request = HtCodeBlockDecodeRequest::new(candidate, 29, 1, &[2, 0]);
    assert!(
        HtCodeBlockDirectCleanupDecodeScratchRequest::new(
            request,
            layout,
            &mut scratch,
            &mut [],
            &mut contexts,
            &mut mel,
            &mut coefficients,
        )
        .is_err()
    );
    assert!(
        HtVlcCleanupOutputSegmentGroupMaterializationRequest::new(
            layout,
            &contexts,
            &mut [],
            29,
            &scratch,
            &coefficients,
        )
        .is_err()
    );
    let mut cursors = HtCleanupPassSegmentViews {
        magnitude_sign: &[],
        mel_vlc: &[2, 0],
    }
    .bit_cursors()
    .unwrap();
    let before = cursors.consumed_bits();
    assert!(
        layout
            .try_decode_vlc_cleanup_segment_groups_to_outputs_and_materialize(
                &mut scratch,
                &mut cursors,
                &mut contexts,
                &mut mel,
                &mut [],
                29,
                &mut coefficients,
            )
            .is_err()
    );
    let mut views = layout
        .prepare_cleanup_decode(
            &mut scratch,
            HtCleanupStreamLayout::from_lengths(0, 0, 0, 0).unwrap(),
            &[],
        )
        .unwrap();
    assert!(
        views
            .try_decode_vlc_cleanup_outputs_with_standard_tables_segment_groups_progress(
                block,
                &mut cursors,
                &mut contexts,
                &mut mel,
                &mut [],
            )
            .is_err()
    );
    let mut line_pair = views.line_pair_cleanup_decode_view_mut(block, 0).unwrap();
    assert!(
        line_pair
            .try_decode_vlc_line_pair_cleanup_outputs_with_standard_tables_from_segment_groups(
                &mut cursors,
                &mut contexts[0],
                &mut mel,
                None,
                &mut [],
            )
            .is_err()
    );
    assert_eq!(cursors.consumed_bits(), before);
    assert_eq!(coefficients, [123]);
}

#[test]
fn separately_named_wide_materialisation_retains_17_and_18_bit_words() {
    let block = HtBlockLayout::new(HtCodeBlockDimensions::new(1, 1).unwrap());
    for bits in [17, 18] {
        for sign in [0, 1] {
            let value = (1_u32 << (bits - 1)) | sign;
            let outputs = [HtVlcCleanupCoefficientOutputWithMagnitude {
                position: block.coefficient_position(0, 0).unwrap(),
                significant: true,
                magnitude_sign_bits: bits,
                magnitude_sign_value: value,
                embedded_magnitude_bit: false,
                magnitude_exponent_reduction: false,
            }];
            let mut coefficients = [0];
            block
                .materialize_vlc_cleanup_coefficient_outputs_with_magnitude(
                    &outputs,
                    29,
                    &mut coefficients,
                )
                .unwrap()
                .unwrap();
            let magnitude = ((value | 1) + 2) as i32;
            assert_eq!(
                coefficients,
                [if sign == 0 { magnitude } else { -magnitude }]
            );
            block
                .materialize_vlc_cleanup_coefficient_outputs_with_progress_with_magnitude(
                    &outputs,
                    29,
                    &mut coefficients,
                )
                .unwrap()
                .unwrap()
                .check_complete()
                .unwrap();
        }
    }
}
