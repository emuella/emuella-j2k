use super::*;

fn fill(rect: TileRect, planes: &mut [Vec<u8>], bits: u8) -> Result<()> {
    for (c, plane) in planes.iter_mut().enumerate() {
        for (i, bytes) in plane.chunks_exact_mut(usize::from(bits / 8)).enumerate() {
            let x = rect.x + i as u32 % rect.width;
            let y = rect.y + i as u32 / rect.width;
            let v = x
                .wrapping_mul(977)
                .wrapping_add(y.wrapping_mul(1393))
                .wrapping_add((c as u32).wrapping_mul(9973))
                .wrapping_add(x.wrapping_mul(y).wrapping_mul(41)) as u16;
            if bits == 8 {
                bytes[0] = v as u8;
            } else {
                bytes.copy_from_slice(&v.to_le_bytes());
            }
        }
    }
    Ok(())
}
fn encode(
    width: u32,
    height: u32,
    edge: u32,
    bits: u8,
    components: u16,
) -> (Vec<u8>, IndexedLossyHt, usize) {
    let mut bytes = Vec::new();
    let mut maximum_source_bytes = 0;
    let index = encode_tiled(
        TiledLossyHtProfile {
            width,
            height,
            tile_edge: edge,
            bits_per_sample: bits,
            components,
            bits_per_pixel: 2.0,
        },
        |rect, planes| {
            maximum_source_bytes = maximum_source_bytes.max(planes.iter().map(Vec::len).sum());
            fill(rect, planes, bits)
        },
        |chunk| {
            bytes.extend_from_slice(chunk);
            Ok(())
        },
    )
    .unwrap();
    (bytes, index, maximum_source_bytes)
}
fn decode(plan: &IndexedHtRegion<'_>, bytes: &[u8], reads: &mut Vec<Range<u64>>) -> Vec<Vec<u8>> {
    plan.decode(
        |offset, dst| {
            reads.push(offset..offset + dst.len() as u64);
            dst.copy_from_slice(&bytes[offset as usize..offset as usize + dst.len()]);
            Ok(())
        },
        &mut ht_lossy::LossyHtSpatialRegionWorkspace::new(),
    )
    .unwrap()
}
fn full_tile(bytes: &[u8], index: &IndexedLossyHt, ordinal: usize) -> Vec<Vec<u8>> {
    let tile = &index.tiles[ordinal];
    let mut header = bytes[..index.main_header.end as usize].to_vec();
    for (offset, value) in [
        (8, tile.rect.width),
        (12, tile.rect.height),
        (24, tile.rect.width),
        (28, tile.rect.height),
    ] {
        header[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
    let q = &index.tile_headers[ordinal];
    let cap = parse(bytes)
        .unwrap()
        .markers
        .iter()
        .find(|m| m.marker == Marker::Cap)
        .unwrap()
        .data_offset;
    header[cap + 4..cap + 6].copy_from_slice(&0x002a_u16.to_be_bytes());
    let n = header.len();
    header[n - 19..].copy_from_slice(&bytes[q.start as usize..q.end as usize]);
    let end = index
        .precincts
        .iter()
        .filter(|p| usize::from(p.tile) == ordinal)
        .map(|p| p.body.end)
        .max()
        .unwrap();
    write_tile_part(
        &mut header,
        0,
        &bytes[tile.payload_offset as usize..end as usize],
        true,
    )
    .unwrap();
    ht_lossy::decode_owned_with_workspace(&header, &mut HtCodestreamDecodeWorkspace::new())
        .unwrap()
        .unwrap()
        .components
        .into_iter()
        .map(|c| c.samples)
        .collect()
}

#[test]
fn single_stream_packet_ranges_and_sparse_cross_tile_match_full_decoder() {
    for (bits, components) in [(8, 1), (8, 3), (16, 1), (16, 3)] {
        let (bytes, index, max_source) = encode(264, 264, 256, bits, components);
        assert_eq!(
            max_source,
            256 * 256 * usize::from(bits / 8) * usize::from(components)
        );
        assert_eq!(index.encoded_bytes(), bytes.len() as u64);
        let parsed = parse(&bytes).unwrap();
        assert_eq!(parsed.tiles.len(), 4);
        assert_eq!(parsed.siz.reference_grid_width, 264);
        assert_eq!(parsed.siz.tile_width, 256);
        validate_part15_packet_signalling(&bytes, &parsed).unwrap();
        assert_eq!(index.precincts().len(), 4 * 3 * usize::from(components));
        for tile in &index.tiles {
            let packets: Vec<_> = index
                .precincts
                .iter()
                .filter(|p| p.tile == tile.rect.tile_index)
                .collect();
            assert_eq!(packets[0].header.start, tile.payload_offset);
            for pair in packets.windows(2) {
                assert_eq!(pair[0].body.end, pair[1].header.start);
            }
        }
        let region = TileRegionRequest {
            x: 249,
            y: 251,
            width: 15,
            height: 13,
        };
        let selection: Vec<_> = (0..components).rev().collect();
        let plan = index.plan(region, 0, &selection).unwrap();
        let mut reads = Vec::new();
        let actual = decode(&plan, &bytes, &mut reads);
        assert_eq!(reads, plan.block_ranges());
        for (slot, &component) in selection.iter().enumerate() {
            let mut expected =
                vec![0; region.width as usize * region.height as usize * usize::from(bits / 8)];
            for (ordinal, tile) in index.tiles.iter().enumerate() {
                let full = full_tile(&bytes, &index, ordinal);
                for y in region.y..region.y + region.height {
                    for x in region.x..region.x + region.width {
                        if x < tile.rect.x
                            || y < tile.rect.y
                            || x >= tile.rect.x + tile.rect.width
                            || y >= tile.rect.y + tile.rect.height
                        {
                            continue;
                        }
                        let src = ((y - tile.rect.y) * tile.rect.width + x - tile.rect.x) as usize
                            * usize::from(bits / 8);
                        let dst = ((y - region.y) * region.width + x - region.x) as usize
                            * usize::from(bits / 8);
                        expected[dst..dst + usize::from(bits / 8)].copy_from_slice(
                            &full[usize::from(component)][src..src + usize::from(bits / 8)],
                        );
                    }
                }
            }
            assert_eq!(actual[slot], expected);
        }
        for discard in 1..=2 {
            let whole = index
                .plan(
                    TileRegionRequest {
                        x: 0,
                        y: 0,
                        width: 264,
                        height: 264,
                    },
                    discard,
                    &[0],
                )
                .unwrap();
            let reference = decode(&whole, &bytes, &mut Vec::new());
            let part = index.plan(region, discard, &[0]).unwrap();
            let actual = decode(&part, &bytes, &mut Vec::new());
            let r = part.output_region();
            let w = 264 >> discard;
            let b = usize::from(bits / 8);
            let expected: Vec<u8> = (r.y..r.y + r.height)
                .flat_map(|y| {
                    reference[0][(y * w + r.x) as usize * b..(y * w + r.x + r.width) as usize * b]
                        .iter()
                        .copied()
                })
                .collect();
            assert_eq!(actual[0], expected);
        }
    }
}

#[test]
fn sparse_repeated_plans_read_only_selected_bodies_and_fail_on_missing_data() {
    let (bytes, index, _) = encode(512, 512, 256, 16, 1);
    let mut workspace = ht_lossy::LossyHtSpatialRegionWorkspace::new();
    for x in [100, 104, 300, 100] {
        let plan = index
            .plan(
                TileRegionRequest {
                    x,
                    y: 100,
                    width: 12,
                    height: 12,
                },
                0,
                &[0],
            )
            .unwrap();
        assert!(
            plan.selected_code_blocks() < index.tiles.iter().map(|t| t.contributions.len()).sum()
        );
        assert!(
            plan.block_ranges()
                .iter()
                .map(|r| r.end - r.start)
                .sum::<u64>()
                < bytes.len() as u64
        );
        assert!(
            plan.decode(|_, _| Err(resource_error()), &mut workspace)
                .is_err()
        );
        let a = plan
            .decode(
                |offset, dst| {
                    dst.copy_from_slice(&bytes[offset as usize..offset as usize + dst.len()]);
                    Ok(())
                },
                &mut workspace,
            )
            .unwrap();
        assert_eq!(a[0].len(), 288);
        workspace.set_maximum_bytes(plan.required_workspace_bytes() - 1);
        assert!(
            plan.decode(|_, _| panic!("limit before reads"), &mut workspace)
                .is_err()
        );
        workspace.set_maximum_bytes(64 * 1024 * 1024);
    }
    for r in [
        TileRegionRequest {
            x: 512,
            y: 0,
            width: 1,
            height: 1,
        },
        TileRegionRequest {
            x: u32::MAX,
            y: 0,
            width: 2,
            height: 2,
        },
    ] {
        assert!(index.plan(r, 0, &[0]).is_err());
    }
    assert!(
        index
            .plan(
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 4,
                    height: 4
                },
                0,
                &[0, 0]
            )
            .is_err()
    );
}

#[test]
#[ignore = "optimised geometry calibration; run explicitly"]
fn geometry_calibration() {
    for (size, components, edge) in [
        (2048, 1, 256),
        (2048, 1, 512),
        (2048, 1, 1024),
        (2048, 3, 256),
        (2048, 3, 512),
        (2048, 3, 1024),
        (4096, 1, 256),
    ] {
        let start = std::time::Instant::now();
        let (bytes, index, source) = encode(size, size, edge, 16, components);
        let parsed = parse(&bytes).unwrap();
        // The existing full-image facade deliberately retains its 1 Mi-pixel
        // admission ceiling. Validate this separate tiled profile per tile.
        for rect in tile_rects(&parsed).unwrap() {
            let part = &parsed.tiles[usize::from(rect.tile_index)];
            let payload = tile_payload(&bytes, part).unwrap();
            let contributions =
                parse_default_precinct_lrcp_packets(&bytes, &parsed, rect, payload).unwrap();
            validate_part15_single_ht_packet_sets(&parsed, &contributions).unwrap();
        }
        let encode_ms = start.elapsed().as_millis();
        let region = TileRegionRequest {
            x: 901,
            y: 901,
            width: 32,
            height: 32,
        };
        let plan = index.plan(region, 0, &[0]).unwrap();
        let packet_bytes: u64 = plan
            .precinct_indices()
            .iter()
            .map(|&i| index.precincts[i].body.end - index.precincts[i].header.start)
            .sum();
        let entropy_bytes: u64 = plan.block_ranges().iter().map(|r| r.end - r.start).sum();
        let start = std::time::Instant::now();
        for _ in 0..20 {
            let plan = index.plan(region, 0, &[0]).unwrap();
            decode(&plan, &bytes, &mut Vec::new());
        }
        println!(
            "size={size} components={components} edge={edge} source={source} stream={} index={} packets={packet_bytes} entropy={entropy_bytes} blocks={} coefficients={} workspace={} encode_ms={encode_ms} repeated20_ms={}",
            bytes.len(),
            index.retained_heap_bytes(),
            plan.selected_code_blocks(),
            plan.selected_block_coefficients(),
            plan.required_workspace_bytes(),
            start.elapsed().as_millis()
        );
    }
}

#[test]
fn odd_boundary_tiles_and_empty_packets_preserve_native_samples() {
    let (bytes, index, _) = encode(263, 269, 256, 16, 3);
    let region = TileRegionRequest {
        x: 256,
        y: 256,
        width: 7,
        height: 13,
    };
    let plan = index.plan(region, 0, &[0, 1, 2]).unwrap();
    assert_eq!(
        decode(&plan, &bytes, &mut Vec::new()),
        full_tile(&bytes, &index, 3)
    );
    let mut bytes = Vec::new();
    let index = encode_tiled(
        TiledLossyHtProfile {
            width: 512,
            height: 512,
            tile_edge: 256,
            bits_per_sample: 16,
            components: 1,
            bits_per_pixel: 2.0,
        },
        |_, planes| {
            for plane in planes {
                for sample in plane.chunks_exact_mut(2) {
                    sample.copy_from_slice(&32768_u16.to_le_bytes());
                }
            }
            Ok(())
        },
        |chunk| {
            bytes.extend_from_slice(chunk);
            Ok(())
        },
    )
    .unwrap();
    let plan = index
        .plan(
            TileRegionRequest {
                x: 251,
                y: 251,
                width: 10,
                height: 10,
            },
            0,
            &[0],
        )
        .unwrap();
    assert_eq!(plan.selected_code_blocks(), 0);
    assert_eq!(plan.precinct_indices().len(), 12);
    let output = plan
        .decode(
            |_, _| panic!("empty packets need no entropy reads"),
            &mut ht_lossy::LossyHtSpatialRegionWorkspace::new(),
        )
        .unwrap();
    assert!(
        output[0]
            .chunks_exact(2)
            .all(|p| p == 32768_u16.to_le_bytes())
    );
}

#[test]
fn invalid_profiles_and_callback_failures_never_return_an_index() {
    let base = TiledLossyHtProfile {
        width: 512,
        height: 512,
        tile_edge: 256,
        bits_per_sample: 16,
        components: 1,
        bits_per_pixel: 2.0,
    };
    for p in [
        TiledLossyHtProfile {
            tile_edge: 0,
            ..base
        },
        TiledLossyHtProfile { width: 257, ..base },
        TiledLossyHtProfile {
            bits_per_pixel: f32::NAN,
            ..base
        },
        TiledLossyHtProfile {
            width: u32::MAX,
            height: u32::MAX,
            ..base
        },
    ] {
        assert!(
            encode_tiled(
                p,
                |_, _| panic!("profile before read"),
                |_| panic!("profile before write")
            )
            .is_err()
        );
    }
    assert!(
        encode_tiled(
            base,
            |_, _| Err(resource_error()),
            |_| panic!("read before write")
        )
        .is_err()
    );
    assert!(
        encode_tiled(
            base,
            |rect, planes| fill(rect, planes, 16),
            |_| Err(resource_error())
        )
        .is_err()
    );
}
