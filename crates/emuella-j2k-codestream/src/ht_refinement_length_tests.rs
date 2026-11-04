//! Zero-length HT refinement segments in packet headers.
//!
//! An HT set whose refinement segments are all empty has `Z_blk = 1`;
//! otherwise `Z_blk` is every pass the set announces, whichever packets
//! carry them. ISO/IEC 15444-15:2019, B.3, physical page 41; retrieval
//! 10baf9472429d52f5d6b5f9b7a892dbed395b1db.

use super::*;

/// One packet's contribution for the fixture's single code-block.
#[derive(Clone, Copy)]
struct LayerContribution<'a> {
    /// Coding passes announced in this packet; zero writes an empty packet.
    passes: u16,
    /// Starts the set with the fixture's Cleanup bytes; otherwise the packet
    /// continues the set's refinement segment.
    starts_set: bool,
    /// Starts a later set with a zero-length Cleanup segment after this many
    /// placeholder passes; the packet's last pass is that Cleanup pass.
    empty_set_after: Option<u16>,
    refinement: &'a [u8],
}

const fn starts_set(passes: u16, refinement: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        starts_set: true,
        empty_set_after: None,
        refinement,
    }
}

const fn continues_set(passes: u16, refinement: &[u8]) -> LayerContribution<'_> {
    LayerContribution {
        passes,
        starts_set: false,
        empty_set_after: None,
        refinement,
    }
}

const fn empty_set_after(placeholders: u16) -> LayerContribution<'static> {
    LayerContribution {
        passes: placeholders + 1,
        starts_set: false,
        empty_set_after: Some(placeholders),
        refinement: &[],
    }
}

const EMPTY: LayerContribution<'static> = LayerContribution {
    passes: 0,
    starts_set: false,
    empty_set_after: None,
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
    for layer in layers {
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
            assert!(layer.starts_set, "the first contribution starts the set");
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
        if let Some(placeholders) = layer.empty_set_after {
            lengths.push((
                0,
                u8::try_from((u32::from(placeholders) + 1).ilog2()).unwrap(),
            ));
        } else if layer.starts_set {
            lengths.push((cleanup.len(), 0_u8));
            if layer.passes > 1 {
                lengths.push((layer.refinement.len(), u8::from(layer.passes > 2)));
            }
        } else {
            lengths.push((layer.refinement.len(), u8::from(layer.passes > 1)));
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
        if layer.starts_set {
            replacement.extend_from_slice(&cleanup);
        }
        if layer.starts_set && layer.passes == 1 {
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

#[test]
fn zero_length_continuation_of_an_empty_set_keeps_it_empty() {
    // Pass 0 is set 0's Cleanup pass and passes 1-2 are placeholders. Set 1
    // has a zero-length Cleanup segment (pass 3), and pass 4 continues it
    // with no bytes.
    let sets = |layers: &[LayerContribution<'_>]| {
        let [contribution] = contributions(&multiple_set_fixture(layers))
            .unwrap()
            .try_into()
            .unwrap();
        contribution.ht_coding_sets().collect::<Vec<_>>()
    };
    let continued = sets(&[
        starts_set(1, &[]),
        empty_set_after(2),
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
        sets(&[starts_set(1, &[]), empty_set_after(2), EMPTY])
    );

    // B.3 requires the refinement segment of an empty set to be empty too.
    let error = contributions(&multiple_set_fixture(&[
        starts_set(1, &[]),
        empty_set_after(2),
        continues_set(1, REFINEMENT),
    ]))
    .unwrap_err();
    assert!(
        format!("{error:?}").contains("follows a zero-length cleanup segment"),
        "{error:?}"
    );
}
