//! Zero-length HT refinement segments in packet headers.
//!
//! An HT set whose refinement segments are all empty has `Z_blk = 1`;
//! otherwise `Z_blk` is every pass the set announces, whichever packets
//! carry them. ISO/IEC 15444-15:2019, B.3, physical page 41; retrieval
//! 10baf9472429d52f5d6b5f9b7a892dbed395b1db.

use super::*;

/// How a packet's contribution relates to the fixture's HT sets.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ContributionKind {
    /// Starts the set with the fixture's Cleanup bytes.
    Starts,
    /// Continues the current set's refinement segment.
    Continues,
    /// Starts a later set at a set boundary with a zero-length Cleanup
    /// segment, followed by its refinement length field if it announces more
    /// than the Cleanup pass.
    Empty,
    /// Reaches a later empty set through a bridge: one length field per
    /// segment before the packet's final set, the first carrying the
    /// contribution's refinement bytes, then the final set's zero-length
    /// Cleanup field and, if it announces more passes, its refinement field.
    BridgedEmpty,
}

/// One packet's contribution for the fixture's single code-block.
#[derive(Clone, Copy)]
struct LayerContribution<'a> {
    /// Coding passes announced in this packet; zero writes an empty packet.
    passes: u16,
    kind: ContributionKind,
    refinement: &'a [u8],
}

const fn starts_set(passes: u16, refinement: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        kind: ContributionKind::Starts,
        refinement,
    }
}

const fn continues_set(passes: u16, refinement: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        kind: ContributionKind::Continues,
        refinement,
    }
}

const fn empty_set(passes: u16, refinement: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        kind: ContributionKind::Empty,
        refinement,
    }
}

const fn bridged_empty_set(passes: u16, bridge_bytes: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        kind: ContributionKind::BridgedEmpty,
        refinement: bridge_bytes,
    }
}

const EMPTY: LayerContribution<'static> = LayerContribution {
    passes: 0,
    kind: ContributionKind::Continues,
    refinement: &[],
};

// Arbitrary refinement bytes; an HT segment cannot end in 0xFF.
const REFINEMENT: &[u8] = &[0x5a, 0xc3, 0x96, 0x3c, 0xa5, 0x69];

fn fixture_samples() -> Vec<u8> {
    (0_u16..64)
        // Within 64 of the level shift, so doubled magnitudes stay in range.
        .map(|sample| u8::try_from(96 + (sample * 37 + 11) % 64).unwrap())
        .collect()
}

fn cleanup_only_fixture() -> Vec<u8> {
    let samples = fixture_samples();
    encode_htj2k_grayscale_u8_no_decomp(GrayscaleU8Encode {
        width: 8,
        height: 8,
        samples: &samples,
        stride_bytes: 8,
    })
    .unwrap()
}

/// Rewrites the fixture's packets so its single code-block's Cleanup bytes
/// and `layers` reach the decoder through the given packet headers.
///
/// The lossless fixture's Cleanup pass reaches the least-significant
/// bit-plane, which would leave refinement passes nothing to refine. The
/// rewritten header signals one fewer missing bit-plane, so the Cleanup pass
/// stops one bit-plane higher and the refinement passes change samples.
fn layered_fixture(layers: &[LayerContribution<'_>]) -> Vec<u8> {
    let mut codestream = cleanup_only_fixture();
    let parsed = parse(&codestream).unwrap();
    let tile_rect = tile_rects(&parsed).unwrap()[0];
    let tile_part = parsed.tiles[0];
    let payload = tile_payload(&codestream, &tile_part).unwrap();
    let [contribution] =
        parse_default_precinct_lrcp_packets(&codestream, &parsed, tile_rect, payload)
            .unwrap()
            .try_into()
            .unwrap();
    assert_eq!(contribution.coding_passes, 1);
    let cleanup = payload
        [contribution.payload_offset..contribution.payload_offset + contribution.codeword_len]
        .to_vec();

    let mut replacement = Vec::new();
    let mut included = false;
    let mut l_block = 3_u8;
    let mut first_pass = 0_u16;
    for layer in layers {
        let pass = first_pass;
        first_pass += layer.passes;
        let mut writer = PacketBitWriter::new();
        if layer.passes == 0 {
            writer.write_bit(0).unwrap();
            writer.align();
            replacement.extend_from_slice(writer.bytes());
            continue;
        }
        writer.write_bit(1).unwrap();
        if included {
            writer.write_bit(1).unwrap();
        } else {
            assert!(
                layer.kind == ContributionKind::Starts,
                "the first contribution starts the set"
            );
            let x = contribution.code_block_x;
            let y = contribution.code_block_y;
            EncTagTree::new(1, 1, [0])
                .unwrap()
                .encode(&mut writer, x, y, 1)
                .unwrap();
            EncTagTree::new(
                1,
                1,
                [u32::from(contribution.missing_most_significant_bitplanes) - 1],
            )
            .unwrap()
            .encode(&mut writer, x, y, u32::MAX)
            .unwrap();
            included = true;
        }
        write_coding_pass_count(&mut writer, layer.passes).unwrap();
        // (length, extra length bits) in header order.
        let mut lengths = Vec::new();
        match layer.kind {
            ContributionKind::Starts | ContributionKind::Empty => {
                let cleanup_len = if layer.kind == ContributionKind::Starts {
                    cleanup.len()
                } else {
                    0
                };
                lengths.push((cleanup_len, 0_u8));
                if layer.passes > 1 {
                    lengths.push((layer.refinement.len(), u8::from(layer.passes > 2)));
                }
            }
            ContributionKind::Continues => {
                lengths.push((layer.refinement.len(), u8::from(layer.passes > 1)));
            }
            ContributionKind::BridgedEmpty => {
                let pass_end = pass + layer.passes;
                let final_set_start = (pass_end - 1) / 3 * 3;
                assert!(final_set_start > pass, "a bridge precedes the final set");
                let mut segment_start = pass;
                while segment_start < final_set_start {
                    let segment_end = if segment_start.is_multiple_of(3) {
                        segment_start + 1
                    } else {
                        (segment_start / 3 + 1) * 3
                    };
                    let len = if segment_start == pass {
                        layer.refinement.len()
                    } else {
                        0
                    };
                    lengths.push((len, u8::from(segment_end - segment_start > 1)));
                    segment_start = segment_end;
                }
                lengths.push((0, 0));
                if pass_end - final_set_start > 1 {
                    lengths.push((0, u8::from(pass_end - final_set_start > 2)));
                }
            }
        }
        while lengths
            .iter()
            .any(|&(len, extra)| len >= 1_usize << u32::from(l_block + extra))
        {
            writer.write_bit(1).unwrap();
            l_block += 1;
        }
        writer.write_bit(0).unwrap();
        for &(len, extra) in &lengths {
            writer
                .write_bits(u32::try_from(len).unwrap(), l_block + extra)
                .unwrap();
        }
        writer.align();
        replacement.extend_from_slice(writer.bytes());
        if layer.kind == ContributionKind::Starts {
            replacement.extend_from_slice(&cleanup);
        }
        if layer.kind != ContributionKind::Continues && layer.passes == 1 {
            assert!(layer.refinement.is_empty());
        } else {
            replacement.extend_from_slice(layer.refinement);
        }
    }

    let payload_offset = tile_part.payload_offset.unwrap();
    let payload_len = tile_part.payload_len.unwrap();
    codestream.splice(
        payload_offset..payload_offset + payload_len,
        replacement.iter().copied(),
    );
    let cod = find_marker(&codestream, 0, Marker::Cod).unwrap();
    codestream[cod + 6..cod + 8]
        .copy_from_slice(&u16::try_from(layers.len()).unwrap().to_be_bytes());
    let sot = find_marker(&codestream, 0, Marker::Sot).unwrap();
    let psot = read_u32(&codestream, sot + 6).unwrap();
    let replacement_len = u32::try_from(replacement.len()).unwrap();
    let payload_len = u32::try_from(payload_len).unwrap();
    codestream[sot + 6..sot + 10]
        .copy_from_slice(&(psot - payload_len + replacement_len).to_be_bytes());
    codestream
}

/// As `layered_fixture`, with Ccap^15 declaring MULTIHT so that packets may
/// start later HT sets.
fn multiple_set_fixture(layers: &[LayerContribution<'_>]) -> Vec<u8> {
    let mut codestream = layered_fixture(layers);
    let cap = find_marker(&codestream, 0, Marker::Cap).unwrap();
    codestream[cap + 8..cap + 10].copy_from_slice(&0x2000_u16.to_be_bytes());
    codestream
}

fn contributions(codestream: &[u8]) -> Result<Vec<PacketCodeBlockContribution>> {
    let parsed = parse(codestream)?;
    parse_default_precinct_lrcp_packets(
        codestream,
        &parsed,
        tile_rects(&parsed)?[0],
        tile_payload(codestream, &parsed.tiles[0])?,
    )
}

fn decoded_samples(codestream: &[u8]) -> Vec<i32> {
    let decoded = decode_htj2k_lossless_owned(codestream)
        .unwrap()
        .expect("native HT decode admits the fixture");
    assert_eq!((decoded.width, decoded.height), (8, 8));
    decoded.components[0]
        .samples
        .iter()
        .map(|&sample| i32::from(sample))
        .collect()
}

fn cleanup_only_samples() -> Vec<i32> {
    let original = fixture_samples()
        .into_iter()
        .map(i32::from)
        .collect::<Vec<_>>();
    assert_eq!(decoded_samples(&cleanup_only_fixture()), original);
    let cleanup_only = decoded_samples(&layered_fixture(&[starts_set(1, &[])]));
    assert_ne!(cleanup_only, original);
    cleanup_only
}

#[test]
fn single_set_reader_accepts_a_zero_refinement_length_as_cleanup_only() {
    for announced in [2_u16, 3] {
        let mut writer = PacketBitWriter::new();
        writer.write_bits(5, 3).unwrap();
        writer.write_bits(0, 3 + u8::from(announced > 2)).unwrap();
        writer.write_bit(1).unwrap();
        let source = ContiguousPacketSource {
            bytes: writer.bytes(),
        };
        let mut reader = PacketBitReader::at(&source, 0).unwrap();
        let (codeword_len, segments, coding_passes, placeholder_sets) =
            read_ht_codeword_lengths(&mut reader, 3, announced).unwrap();
        assert_eq!(
            (codeword_len, coding_passes, placeholder_sets),
            (5, 1, 0),
            "announced {announced}"
        );
        assert_eq!(
            segments,
            [tier1::CodeBlockSegment {
                byte_len: 5,
                coding_passes: 1,
            }]
        );
        assert_eq!(reader.read_bits_with_stuffing(1).unwrap(), 1);
    }
}

#[test]
fn one_layer_zero_refinement_length_decodes_the_cleanup_pass_only() {
    let expected = cleanup_only_samples();
    for announced in [2_u16, 3] {
        let codestream = layered_fixture(&[starts_set(announced, &[])]);
        assert_eq!(
            decoded_samples(&codestream),
            expected,
            "announced {announced}"
        );
    }
}

#[test]
fn layered_zero_refinement_lengths_keep_z_blk_at_one() {
    let expected = cleanup_only_samples();
    for layers in [
        [starts_set(2, &[]), EMPTY],
        [starts_set(3, &[]), EMPTY],
        [starts_set(2, &[]), continues_set(1, &[])],
        [starts_set(1, &[]), continues_set(2, &[])],
        [starts_set(1, &[]), continues_set(1, &[])],
    ] {
        assert_eq!(decoded_samples(&layered_fixture(&layers)), expected);
    }
}

#[test]
fn refinement_bytes_after_a_zero_length_decode_every_announced_pass() {
    let three_passes = decoded_samples(&layered_fixture(&[starts_set(3, REFINEMENT)]));
    assert_ne!(three_passes, cleanup_only_samples());
    assert_eq!(
        decoded_samples(&layered_fixture(&[starts_set(3, REFINEMENT), EMPTY])),
        three_passes
    );
    // Unchanged route: the set's refinement bytes all arrive in the second
    // packet after a Cleanup-only first packet.
    assert_eq!(
        decoded_samples(&layered_fixture(&[
            starts_set(1, &[]),
            continues_set(2, REFINEMENT)
        ])),
        three_passes
    );
    // The first packet announces a refinement pass with no bytes, so Z_blk
    // grows from 1 to 3 when the second packet supplies them.
    assert_eq!(
        decoded_samples(&layered_fixture(&[
            starts_set(2, &[]),
            continues_set(1, REFINEMENT)
        ])),
        three_passes
    );
}

#[test]
fn zero_length_continuation_after_refinement_bytes_adds_its_passes() {
    let three_passes = decoded_samples(&layered_fixture(&[starts_set(3, REFINEMENT)]));
    let two_passes = decoded_samples(&layered_fixture(&[starts_set(2, REFINEMENT)]));
    assert_ne!(three_passes, two_passes);
    assert_eq!(
        decoded_samples(&layered_fixture(&[
            starts_set(2, REFINEMENT),
            continues_set(1, &[])
        ])),
        three_passes
    );
    assert_eq!(
        decoded_samples(&layered_fixture(&[starts_set(2, REFINEMENT), EMPTY])),
        two_passes
    );
}

fn coding_sets(layers: &[LayerContribution<'_>]) -> Vec<HtCodeBlockCodingSet> {
    let [contribution] = contributions(&multiple_set_fixture(layers))
        .unwrap()
        .try_into()
        .unwrap();
    contribution.ht_coding_sets().collect()
}

#[test]
fn zero_length_continuation_of_an_empty_set_keeps_it_empty() {
    // Pass 0 is set 0's Cleanup pass and passes 1-2 continue it with no
    // bytes. Set 1 has a zero-length Cleanup segment (pass 3), and pass 4
    // continues it with no bytes.
    let sets = coding_sets;
    let continued = sets(&[
        starts_set(1, &[]),
        continues_set(2, &[]),
        empty_set(1, &[]),
        continues_set(1, &[]),
    ]);
    assert_eq!(continued.len(), 2);
    assert_eq!(continued[0].coding_passes, 1);
    assert_eq!(
        (
            continued[1].byte_len,
            continued[1].cleanup_byte_len,
            continued[1].coding_passes
        ),
        (0, 0, 0)
    );
    assert_eq!(
        continued,
        sets(&[
            starts_set(1, &[]),
            continues_set(2, &[]),
            empty_set(1, &[]),
            EMPTY
        ])
    );

    // B.3 requires the refinement segment of an empty set to be empty too.
    let error = contributions(&multiple_set_fixture(&[
        starts_set(1, &[]),
        continues_set(2, &[]),
        empty_set(1, &[]),
        continues_set(1, REFINEMENT),
    ]))
    .unwrap_err();
    assert!(
        format!("{error:?}").contains("follows a zero-length cleanup segment"),
        "{error:?}"
    );
}

#[test]
fn later_empty_set_reader_consumes_its_refinement_length() {
    for announced in [2_u16, 3] {
        let width = 3 + u8::from(announced > 2);
        let read = |refinement_len: u32| {
            let mut writer = PacketBitWriter::new();
            writer.write_bits(0, 3).unwrap();
            writer.write_bits(refinement_len, width).unwrap();
            writer.write_bit(1).unwrap();
            let source = ContiguousPacketSource {
                bytes: writer.bytes(),
            };
            let mut reader = PacketBitReader::at(&source, 0).unwrap();
            read_layered_ht_codeword_lengths(&mut reader, 3, 3, announced, true)
                .map(|lengths| (lengths, reader.read_bits_with_stuffing(1).unwrap()))
        };
        let ((codeword_len, segments, coding_passes, cleanup_set), next_bit) = read(0).unwrap();
        assert_eq!(
            (codeword_len, segments.len(), coding_passes, cleanup_set),
            (0, 0, 0, None),
            "announced {announced}"
        );
        assert_eq!(next_bit, 1, "announced {announced}");

        // B.3: the empty set's refinement segment must be empty too.
        let error = read(5).unwrap_err();
        assert!(
            format!("{error:?}").contains("follows a zero-length cleanup segment"),
            "announced {announced}: {error:?}"
        );
    }
}

#[test]
fn later_contribution_reads_one_length_per_bridge_segment() {
    // (first coding pass, announced passes, bridge segment widths) after the
    // first non-empty set. The final set starts with a five-byte Cleanup
    // segment in a 3-bit field.
    for (first, announced, bridge) in [
        (1_u16, 3_u16, &[4_u8][..]),
        (2, 2, &[3]),
        (3, 4, &[3, 4]),
        (1, 6, &[4, 3, 4]),
    ] {
        let read = |bridge_len: u32| {
            let mut writer = PacketBitWriter::new();
            for (index, &width) in bridge.iter().enumerate() {
                let len = if index == 0 { bridge_len } else { 0 };
                writer.write_bits(len, width).unwrap();
            }
            writer.write_bits(5, 3).unwrap();
            writer.write_bit(1).unwrap();
            let source = ContiguousPacketSource {
                bytes: writer.bytes(),
            };
            let mut reader = PacketBitReader::at(&source, 0).unwrap();
            read_layered_ht_codeword_lengths(&mut reader, 3, first, announced, true)
                .map(|lengths| (lengths, reader.read_bits_with_stuffing(1).unwrap()))
        };
        let ((codeword_len, _, coding_passes, cleanup_set), next_bit) = read(0).unwrap();
        let final_set = u8::try_from((first + announced - 1) / 3).unwrap();
        assert_eq!(
            (codeword_len, coding_passes, cleanup_set, next_bit),
            (5, 1, Some((final_set, 5)), 1),
            "first {first}, announced {announced}"
        );

        let error = read(2).unwrap_err();
        assert!(
            matches!(
                error,
                CodestreamError::Unsupported {
                    construct: UnsupportedConstruct::PacketDecode,
                    ..
                }
            ),
            "first {first}, announced {announced}: {error:?}"
        );
    }
}

#[test]
fn empty_bridge_completes_the_preceding_set_as_split_packets_do() {
    // Set 0 has refinement bytes, so the bridge's zero-length MagRef pass
    // raises Z_blk from 2 to 3, as a separate zero-length continuation does.
    let with_bytes = coding_sets(&[starts_set(2, REFINEMENT), bridged_empty_set(2, &[])]);
    assert_eq!(with_bytes[0].coding_passes, 3);
    assert_eq!(
        with_bytes,
        coding_sets(&[
            starts_set(2, REFINEMENT),
            continues_set(1, &[]),
            empty_set(1, &[])
        ])
    );

    // A Cleanup-only set keeps Z_blk = 1.
    let cleanup_only = coding_sets(&[starts_set(1, &[]), bridged_empty_set(3, &[])]);
    assert_eq!(cleanup_only[0].coding_passes, 1);
    assert_eq!(
        cleanup_only,
        coding_sets(&[starts_set(1, &[]), continues_set(2, &[]), empty_set(1, &[])])
    );

    // A bridge may cross a whole empty set. Only the packet's final set is
    // retained from the bridged contribution.
    let crossed = coding_sets(&[starts_set(1, &[]), bridged_empty_set(6, &[])]);
    let split = coding_sets(&[
        starts_set(1, &[]),
        continues_set(2, &[]),
        empty_set(3, &[]),
        empty_set(1, &[]),
    ]);
    assert_eq!(crossed.len(), 2);
    assert_eq!(split.len(), 3);
    assert_eq!(crossed.first(), split.first());
    assert_eq!(crossed.last(), split.last());

    // Bytes before the final set would need another retained set.
    let error = contributions(&multiple_set_fixture(&[
        starts_set(1, &[]),
        bridged_empty_set(3, REFINEMENT),
    ]))
    .unwrap_err();
    assert!(
        matches!(
            error,
            CodestreamError::Unsupported {
                construct: UnsupportedConstruct::PacketDecode,
                ..
            }
        ),
        "{error:?}"
    );
}

#[test]
fn later_empty_set_refinement_length_keeps_the_packet_header_in_step() {
    // Set 0 is passes 0-2 and set 1 is an empty set from pass 3. However its
    // passes are split across packets, set 1 stays a zero-byte, zero-pass
    // set and the following packets still parse.
    let set0 = starts_set(3, REFINEMENT);
    let expected = coding_sets(&[set0, empty_set(3, &[]), EMPTY]);
    assert_eq!(expected.len(), 2);
    assert_eq!(expected[0].coding_passes, 3);
    assert_eq!(
        (
            expected[1].byte_len,
            expected[1].cleanup_byte_len,
            expected[1].coding_passes
        ),
        (0, 0, 0)
    );
    for layers in [
        [set0, empty_set(2, &[]), continues_set(1, &[])],
        [set0, empty_set(1, &[]), continues_set(2, &[])],
        [set0, empty_set(2, &[]), EMPTY],
    ] {
        assert_eq!(coding_sets(&layers), expected);
    }

    let error = contributions(&multiple_set_fixture(&[
        set0,
        empty_set(2, REFINEMENT),
        EMPTY,
    ]))
    .unwrap_err();
    assert!(
        format!("{error:?}").contains("follows a zero-length cleanup segment"),
        "{error:?}"
    );
}
