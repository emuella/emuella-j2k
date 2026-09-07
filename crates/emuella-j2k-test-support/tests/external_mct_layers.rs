//! Opt-in qualification using a caller-supplied, unchanged GDAL test input.
//! See docs/mct-layer-continuation-qualification.md for provenance and use.

use std::collections::BTreeSet;

use emuella_j2k_core::{
    ImageViewMut, Part1DecodeWorkspace, PlaneMut, SampleFormat, codestream,
    execute_prepared_part1_decode_into_with_workspace, prepare_part1_decode_from_source,
};
use sha2::{Digest, Sha256};

#[test]
#[ignore = "requires the authorised GDAL input in EMUELLA_MCT_NITF_INPUT"]
fn external_mct_layers_have_staggered_inclusion_continuation_and_exact_regions() {
    let path = std::env::var_os("EMUELLA_MCT_NITF_INPUT")
        .expect("supply the existing authorised GDAL test_jp2_ecw33.ntf path");
    let file = std::fs::read(path).expect("read the input in place");
    assert_eq!(file.len(), 2525, "unexpected input length");
    assert!(
        Sha256::digest(&file)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
            == "cc2e868e1b4bb9878703333090d60a5cfa6101ae231fc3bab4233549c154f5ed",
        "unexpected input identity"
    );
    // The locked NITF has a 404-byte file header and 529-byte image subheader.
    // Borrow its C8 segment in memory; never write an extracted codestream.
    let input = &file[933..];
    let parsed = codestream::parse(input).expect("parse the locked C8 segment");
    let style = parsed.uniform_effective_coding_style().unwrap();
    assert_eq!((parsed.image_width(), parsed.image_height()), (200, 100));
    assert_eq!((style.layers, style.decomposition_levels), (19, 5));
    assert!(style.multiple_component_transform && style.eph_markers);
    assert_eq!(parsed.tiles.len(), 1);
    let tile = &parsed.tiles[0];
    let start = tile.payload_offset.unwrap();
    let payload = &input[start..start + tile.payload_len.unwrap()];
    let blocks = codestream::parse_default_precinct_lrcp_packets(
        input,
        &parsed,
        codestream::TileRect {
            tile_index: 0,
            tile_x: 0,
            tile_y: 0,
            x: 0,
            y: 0,
            width: 200,
            height: 100,
        },
        payload,
    )
    .expect("parse every declared layer");

    // The integrity-locked input has one PLT at 144, with Iplt at 148.
    assert!(input[144..149] == [0xff, 0x58, 0x01, 0x59, 0]);
    let mut packet_ends = Vec::new();
    let mut length = 0_usize;
    let mut total = 0_usize;
    for byte in &input[149..491] {
        length = (length << 7) | usize::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            total += length;
            packet_ends.push(total);
            length = 0;
        }
    }
    assert_eq!(packet_ends.len(), 342);
    assert_eq!(total, payload.len());

    let mut first_layers = BTreeSet::new();
    let mut component_activity = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
    let mut active_resolutions = BTreeSet::new();
    let mut continued_blocks = 0;
    for block in &blocks {
        assert!(
            block.segment_ranges.len() > 1,
            "expected continued codeword"
        );
        assert!(
            block.coding_segments.is_empty(),
            "expected continuous MQ coding"
        );
        let mut previous_layer = None;
        for (index, range) in block.segment_ranges.iter().enumerate() {
            let packet = packet_ends.partition_point(|&end| {
                end < range.payload_offset
                    || (end == range.payload_offset && range.codeword_len != 0)
            });
            assert!(range.payload_offset + range.codeword_len <= packet_ends[packet]);
            let layer = packet / 18;
            assert_eq!((packet % 18) / 3, usize::from(block.resolution));
            assert_eq!(packet % 3, usize::from(block.component_index));
            if let Some(previous) = previous_layer {
                assert!(layer > previous);
            }
            previous_layer = Some(layer);
            if index == 0 {
                first_layers.insert(layer);
            }
            component_activity[usize::from(block.component_index)].insert(layer);
        }
        continued_blocks += 1;
        active_resolutions.insert(block.resolution);
    }
    assert_eq!(continued_blocks, 5);
    assert_eq!(first_layers, BTreeSet::from([0, 1, 2, 4]));
    assert_eq!(active_resolutions, BTreeSet::from([0, 3, 4]));
    assert_ne!(component_activity[0], component_activity[1]);
    assert_ne!(component_activity[1], component_activity[2]);
    assert_ne!(component_activity[0], component_activity[2]);
    assert!(component_activity.iter().all(|layers| layers.contains(&18)));

    // The existing GDAL qualification establishes this interior horizontal-ramp
    // oracle. Keep external and decoded pixels in process memory, including on
    // assertion failure; report only factual pass/fail and structure counts.
    for components in [&[0_u16][..], &[1][..], &[2][..], &[2, 0, 1][..]] {
        let source = codestream::source::SliceSource::new(input);
        let prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: components,
                region: codestream::TileRegionRequest {
                    x: 37,
                    y: 23,
                    width: 61,
                    height: 29,
                },
                discard_levels: 0,
                max_layers: None,
            },
        )
        .expect("prepare an all-layer MCT region");
        assert_eq!(prepared.reconstruction_component_indices(), [0, 1, 2]);
        let mut buffers = vec![vec![0xa5; 68 * 29]; components.len()];
        let mut planes = buffers
            .iter_mut()
            .map(|buffer| PlaneMut::new(buffer, 61, 29, 68, SampleFormat::U8).unwrap())
            .collect::<Vec<_>>();
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut ImageViewMut::Planar {
                info: prepared.info(),
                planes: &mut planes,
            },
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions::default(),
        )
        .expect("execute the selected region");
        for (component, buffer) in components.iter().zip(&buffers) {
            let offset = match component {
                0 => 0,
                1 => 20,
                2 => 30,
                _ => unreachable!(),
            };
            assert!(
                buffer.chunks_exact(68).all(|row| {
                    row[..61]
                        .iter()
                        .enumerate()
                        .all(|(x, &sample)| usize::from(sample) == 37 + x + offset)
                        && row[61..].iter().all(|&sample| sample == 0xa5)
                }),
                "regional sample or padding mismatch"
            );
        }
    }
    println!(
        "PASS: five continued blocks; first inclusion layers 0,1,2,4; distinct component schedules; resolutions 0,3,4; four exact regional selections"
    );
}
