//! HT refinement segments that end before the bits their passes consume.
//!
//! ISO/IEC 15444-15:2019 | ITU-T T.814, 7.1.5 and 7.1.6 (physical pages
//! 15-17; retrieval 10baf9472429d52f5d6b5f9b7a892dbed395b1db) define SigProp
//! reads beyond the end of the HT refinement segment, and MagRef reads before
//! its start, as zero bytes. An encoder may therefore drop the trailing zero
//! bytes of either stream. These synthetic segments compare a decode of each
//! zero-padded segment with a decode of the same segment after trimming.

use emuella_j2k_ht::*;

const MAGNITUDE_BIT_DEPTH: u8 = 8;

fn candidate(edge: u16) -> HtDecodeCandidate {
    plan_decode_candidate(HtMarkerState {
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
        code_block_width: Some(edge),
        code_block_height: Some(edge),
    })
    .unwrap()
}

/// Sparse cleanup content: many insignificant coefficients border a
/// significant one, so the SigProp pass reads well over one byte of bits.
fn sparse_coefficients(edge: u16) -> Vec<i32> {
    let edge = usize::from(edge);
    (0..edge * edge)
        .map(|i| {
            let (x, y) = (i % edge, i / edge);
            if (x * 3 + y * 5) % 7 == 0 {
                let magnitude = 1 + ((x * 11 + y * 7) % 90) as i32;
                if (x + y) % 2 == 0 {
                    magnitude
                } else {
                    -magnitude
                }
            } else {
                0
            }
        })
        .collect()
}

/// Dense cleanup content: every coefficient is significant, so the SigProp
/// pass reads no bits and the MagRef pass reads one bit per coefficient.
fn dense_coefficients(edge: u16) -> Vec<i32> {
    let edge = usize::from(edge);
    (0..edge * edge)
        .map(|i| {
            let magnitude = 1 + (i * 37 % 120) as i32;
            if i % 3 == 0 { -magnitude } else { magnitude }
        })
        .collect()
}

fn decode(
    edge: u16,
    coefficients: &[i32],
    coding_passes: u16,
    refinement: &[u8],
) -> Result<(Vec<i32>, HtRefinementPassProgress), HtLayoutError> {
    let encoded = encode_ht_cleanup_block(
        coefficients,
        edge,
        edge,
        usize::from(edge),
        MAGNITUDE_BIT_DEPTH,
    )
    .unwrap()
    .unwrap();
    let cleanup = encoded.segment;
    let layout = HtCleanupPassSegmentLayout::from_cleanup_pass_bytes(&cleanup).unwrap();
    let mut segment = cleanup.clone();
    segment.extend_from_slice(refinement);
    let candidate = candidate(edge);
    let request = HtCodeBlockDecodeRequest::new_with_active_dimensions_and_pass_segment_layout(
        candidate,
        candidate.block().dimensions(),
        encoded.missing_most_significant_bitplanes,
        coding_passes,
        layout,
        cleanup.len(),
        &segment,
    );
    assert_eq!(
        classify_code_block_decode_request(request),
        HtCodeBlockDecodeRequestClass::DirectRefinementPasses
    );
    let mut output = vec![0; usize::from(edge) * usize::from(edge)];
    let mut scratch = HtBlockScratchBuffer::new();
    let (outcome, progress) =
        scratch.decode_code_block_request_into_with_dispatch_progress(request, &mut output)?;
    let HtCodeBlockDecodeOutcome::Decoded(decoded) = outcome else {
        panic!("refinement request was not decoded");
    };
    let coefficients = decoded.coefficients.to_vec();
    Ok((coefficients, progress.refinement_pass_progress.unwrap()))
}

/// Zero padding long enough that no pass reads past the padded segment.
fn padding_len(edge: u16) -> usize {
    // Each coefficient costs at most one SigProp bit and one sign bit, or one
    // MagRef bit; add a byte for each stream's stuffing slack.
    usize::from(edge) * usize::from(edge) * 2 / 8 + 2
}

fn assert_trimmed_sigprop_matches_padded(edge: u16) {
    let coefficients = sparse_coefficients(edge);
    let trimmed = [0x5a_u8];
    let mut padded = trimmed.to_vec();
    padded.resize(padding_len(edge), 0);

    let (expected, padded_progress) = decode(edge, &coefficients, 2, &padded).unwrap();
    let (actual, trimmed_progress) = decode(edge, &coefficients, 2, &trimmed)
        .expect("a SigProp stream may end before its trailing zero bytes");

    assert!(
        padded_progress.significance_propagation_bits > trimmed.len() * 8,
        "the fixture must read SigProp bits beyond the trimmed segment"
    );
    assert!(padded_progress.newly_significant_coefficients > 0);
    assert_eq!(trimmed_progress, padded_progress);
    assert_eq!(actual, expected);
}

fn assert_trimmed_magref_matches_padded(edge: u16) {
    let coefficients = dense_coefficients(edge);
    // MagRef reads backwards from the last byte, so its trailing zero bytes
    // are the ones at the start of the HT refinement segment.
    let trimmed = [0xb6_u8];
    let mut padded = vec![0; padding_len(edge) - 1];
    padded.extend_from_slice(&trimmed);

    let (expected, padded_progress) = decode(edge, &coefficients, 3, &padded).unwrap();
    let (actual, trimmed_progress) = decode(edge, &coefficients, 3, &trimmed)
        .expect("a MagRef stream may end before its trailing zero bytes");

    assert_eq!(padded_progress.significance_propagation_bits, 0);
    assert!(
        padded_progress.magnitude_refinement_bits > trimmed.len() * 8,
        "the fixture must read MagRef bits before the trimmed segment start"
    );
    assert_eq!(trimmed_progress, padded_progress);
    assert_eq!(actual, expected);
}

#[test]
fn trimmed_sigprop_segment_decodes_as_zero_padded_on_generic_block() {
    assert_trimmed_sigprop_matches_padded(8);
}

#[test]
fn trimmed_sigprop_segment_decodes_as_zero_padded_on_full_64x64_block() {
    assert_trimmed_sigprop_matches_padded(64);
}

#[test]
fn trimmed_magref_segment_decodes_as_zero_padded_on_generic_block() {
    assert_trimmed_magref_matches_padded(8);
}

#[test]
fn trimmed_magref_segment_decodes_as_zero_padded_on_full_64x64_block() {
    assert_trimmed_magref_matches_padded(64);
}

#[test]
fn both_refinement_streams_read_zero_beyond_a_one_byte_segment() {
    // With a sparse cleanup both passes run: SigProp reads forwards past the
    // single byte and MagRef reads backwards past the same byte.
    let edge = 16;
    let coefficients = sparse_coefficients(edge);
    let (output, progress) = decode(edge, &coefficients, 3, &[0x00]).unwrap();
    let (padded_output, padded_progress) =
        decode(edge, &coefficients, 3, &vec![0; padding_len(edge)]).unwrap();
    assert!(progress.significance_propagation_bits > 8);
    assert!(progress.magnitude_refinement_bits > 8);
    assert_eq!(progress.newly_significant_coefficients, 0);
    assert_eq!(progress, padded_progress);
    assert_eq!(output, padded_output);
}

#[test]
fn sigprop_stuffed_final_byte_continues_into_zero_fill() {
    // A segment cannot end in 0xFF, so a stuffed byte before the fill is the
    // next-to-last byte: the last stored byte then gives 7 bits, and the
    // zero bytes beyond the segment give 8 bits each. The stuffed bit and the
    // fill are both zero, so the boundary cannot change the decoded bits;
    // this pins that the conforming boundary is not rejected.
    let edge = 64;
    let coefficients = sparse_coefficients(edge);
    let trimmed = [0x5a_u8, 0xff, 0x3c];
    let mut padded = trimmed.to_vec();
    padded.resize(padding_len(edge), 0);

    let (expected, padded_progress) = decode(edge, &coefficients, 2, &padded).unwrap();
    let (actual, trimmed_progress) = decode(edge, &coefficients, 2, &trimmed)
        .expect("a SigProp stream may end after a stuffed byte");

    assert!(
        padded_progress.significance_propagation_bits > 8 + 8 + 7,
        "the fixture must read SigProp bits beyond the stuffed final byte"
    );
    assert_eq!(trimmed_progress, padded_progress);
    assert_eq!(actual, expected);
}

#[test]
fn magref_stuffing_state_at_the_segment_start_carries_into_zero_fill() {
    // MagRef reads backwards: 0xA5 gives 8 bits, then byte 0 (0x7F) has its
    // seven LSBs set after a byte above 0x8F, so it is stuffed and gives 7
    // bits. The implied zero bytes before the segment start then give 8 bits
    // each. As for SigProp, the stuffed bit and the fill are both zero; this
    // pins that the conforming boundary is not rejected.
    let edge = 16;
    let coefficients = dense_coefficients(edge);
    let trimmed = [0x7f_u8, 0xa5];
    let mut padded = vec![0; padding_len(edge) - trimmed.len()];
    padded.extend_from_slice(&trimmed);

    let (expected, padded_progress) = decode(edge, &coefficients, 3, &padded).unwrap();
    let (actual, trimmed_progress) = decode(edge, &coefficients, 3, &trimmed)
        .expect("a MagRef stream may reach its start in a stuffing state");

    assert_eq!(padded_progress.significance_propagation_bits, 0);
    assert!(
        padded_progress.magnitude_refinement_bits > 8 + 7,
        "the fixture must read MagRef bits before the segment start"
    );
    assert_eq!(trimmed_progress, padded_progress);
    assert_eq!(actual, expected);
}
