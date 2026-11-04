//! HT cleanup MEL reads at the end of the cleanup segment.
//!
//! ISO/IEC 15444-15:2019 | ITU-T T.814, 7.1.3 (physical page 14; retrieval
//! 10baf9472429d52f5d6b5f9b7a892dbed395b1db) reads the MEL byte-stream through
//! the modified segment view of 7.1.1 (physical page 12): byte `Lcup-2` has its
//! low nibble set, and every read beyond it yields `0xFF`. An encoder may
//! therefore end the MEL byte-stream early and rely on the decoder for the
//! remaining one bits.
//!
//! Each synthetic segment below holds no MagSgn or VLC data. Its whole MEL
//! byte-stream is byte `Lcup-2`, `0xF2`, whose low nibble is the low half of
//! `Scup = 2`. Seen as `0xFF` followed by fill, the MEL yields only zero
//! events, so every quad is insignificant. Read as stored, its fifth bit
//! yields a one event and a significant quad with no VLC data to describe it.
//! Both the fast decode path and the checked segment-cursor path must decode
//! the all-zero block and agree on the MEL bit accounting.

use emuella_j2k_ht::*;

/// `Lcup = Scup = 2`: one stored MEL byte, `Lcup-2`, then the `Scup` byte.
const SEGMENT: [u8; 2] = [0xf2, 0x00];
const MISSING_MOST_SIGNIFICANT_BITPLANES: u8 = 7;

fn candidate(width: u16, height: u16) -> HtDecodeCandidate {
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
        code_block_width: Some(width),
        code_block_height: Some(height),
    })
    .unwrap()
}

fn request(candidate: HtDecodeCandidate, segment: &[u8]) -> HtCodeBlockDecodeRequest<'_> {
    let layout = HtCleanupPassSegmentLayout::from_cleanup_pass_bytes(segment).unwrap();
    assert_eq!((layout.lcup(), layout.scup()), (2, 2));
    HtCodeBlockDecodeRequest::new_with_active_dimensions_and_cleanup_pass_segment_layout(
        candidate,
        candidate.block().dimensions(),
        MISSING_MOST_SIGNIFICANT_BITPLANES,
        1,
        layout,
        segment,
    )
}

fn decode_fast(
    candidate: HtDecodeCandidate,
    segment: &[u8],
    len: usize,
) -> Result<(Vec<i32>, HtVlcBlockCleanupSegmentProgress), HtLayoutError> {
    let mut output = vec![i32::MIN; len];
    let mut scratch = HtBlockScratchBuffer::new();
    let (outcome, progress) = scratch
        .decode_code_block_request_into_with_progress(request(candidate, segment), &mut output)?;
    let HtCodeBlockDecodeOutcome::Decoded(decoded) = outcome else {
        panic!("cleanup request was not decoded by the fast path");
    };
    Ok((decoded.coefficients.to_vec(), progress.unwrap()))
}

fn decode_checked(
    candidate: HtDecodeCandidate,
    segment: &[u8],
    len: usize,
) -> Result<(Vec<i32>, HtVlcBlockCleanupSegmentProgress), HtLayoutError> {
    let mut output = vec![i32::MIN; len];
    let mut scratch = HtBlockScratchBuffer::new();
    let outcome = scratch
        .decode_code_block_cleanup_vlc_outputs_with_reusable_buffers(request(candidate, segment))?
        .expect("cleanup request was not decoded by the checked path");
    let progress = outcome.progress();
    outcome
        .materialize_coefficients(MISSING_MOST_SIGNIFICANT_BITPLANES, &mut output)?
        .expect("checked cleanup outputs were not materialised");
    Ok((output, progress))
}

/// Decode the segment through both paths, check that they agree, and return
/// the MEL bits consumed from stored bytes.
fn assert_all_zero_block(width: u16, height: u16) -> usize {
    let candidate = candidate(width, height);
    let len = usize::from(width) * usize::from(height);
    let (fast, fast_progress) =
        decode_fast(candidate, &SEGMENT, len).expect("fast path must read the MEL per T.814 7.1.3");
    let (checked, checked_progress) = decode_checked(candidate, &SEGMENT, len)
        .expect("checked path must read the MEL per T.814 7.1.3");

    assert_eq!(fast, vec![0; len]);
    assert_eq!(checked, fast);
    assert_eq!(fast_progress.decode, checked_progress.decode);
    assert_eq!(fast_progress.consumed_bits, checked_progress.consumed_bits);
    assert_eq!(
        fast_progress.remaining_bits,
        checked_progress.remaining_bits
    );
    assert_eq!(
        fast_progress.consumed_bytes,
        checked_progress.consumed_bytes
    );
    assert_eq!(
        fast_progress.remaining_bytes,
        checked_progress.remaining_bytes
    );
    assert_eq!(
        fast_progress.byte_alignment,
        checked_progress.byte_alignment
    );

    // Only the stored byte `Lcup-2` counts towards MEL accounting.
    let mel = fast_progress.consumed_bits.mel;
    assert_eq!(mel + fast_progress.remaining_bits.mel, 8);
    assert_eq!(fast_progress.consumed_bits.magnitude_sign, 0);
    assert_eq!(fast_progress.consumed_bits.vlc, 0);
    mel
}

#[test]
fn mel_reads_byte_lcup_minus_2_with_its_low_nibble_set() {
    // Eight quads take eight zero events, which six one bits supply. As
    // stored, the fifth MEL bit of 0xF2 is zero.
    let consumed = assert_all_zero_block(8, 4);
    assert_eq!(consumed, 6);
}

#[test]
fn mel_reads_ff_fill_after_the_segment() {
    // Sixty-four quads need more one bits than byte `Lcup-2` holds, so the
    // MEL continues into fill; fill bits are not counted as consumed.
    let consumed = assert_all_zero_block(16, 16);
    assert_eq!(consumed, 8);
}
