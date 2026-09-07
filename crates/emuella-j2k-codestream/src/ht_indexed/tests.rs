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
            decomposition_levels: 2,
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
            decomposition_levels: 2,
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
        decomposition_levels: 2,
        width: 512,
        height: 512,
        tile_edge: 256,
        bits_per_sample: 16,
        components: 1,
        bits_per_pixel: 2.0,
    };
    for p in [
        TiledLossyHtProfile {
            decomposition_levels: 2,
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

fn authored(rect: TileRect, planes: &mut [Vec<u8>], bits: u8) -> Result<()> {
    let max = (1_u32 << bits) - 1;
    for (c, plane) in planes.iter_mut().enumerate() {
        for (i, sample) in plane
            .chunks_exact_mut(usize::from(bits.div_ceil(8)))
            .enumerate()
        {
            let x = rect.x + i as u32 % rect.width;
            let y = rect.y + i as u32 / rect.width;
            // Smooth background, faint texture, tiny bright/dark objects, and
            // hard edges deliberately crossing tile and image boundaries.
            let mut v = max / 3 + ((x * 13 + y * 7 + c as u32 * 17) % 31) * max / 2048;
            if x % 127 < 3 && y % 113 < 5 {
                v = max * 9 / 10;
            }
            if (x + y) % 251 < 2 {
                v = max / 16;
            }
            if x % 509 > 400 {
                v += max / 5;
            }
            let v = v.min(max);
            if bits == 8 {
                sample[0] = v as u8;
            } else {
                sample.copy_from_slice(&(v as u16).to_le_bytes());
            }
        }
    }
    Ok(())
}
fn encode_depth(
    width: u32,
    height: u32,
    edge: u32,
    bits: u8,
    components: u16,
    levels: u8,
) -> (Vec<u8>, IndexedLossyHt) {
    let mut bytes = Vec::new();
    let index = encode_tiled(
        TiledLossyHtProfile {
            width,
            height,
            tile_edge: edge,
            decomposition_levels: levels,
            bits_per_sample: bits,
            components,
            bits_per_pixel: 2.0,
        },
        |r, p| authored(r, p, bits),
        |b| {
            bytes.extend_from_slice(b);
            Ok(())
        },
    )
    .unwrap();
    (bytes, index)
}
fn full_depth_tile(
    bytes: &[u8],
    index: &IndexedLossyHt,
    tile: u16,
    discard: u8,
    component: u16,
) -> Vec<u8> {
    let metadata = &index.tiles[usize::from(tile)];
    let descriptor = &metadata.descriptor;
    let header_len = u32::from_le_bytes(descriptor[18..22].try_into().unwrap()) as usize;
    let mut local = descriptor[22..22 + header_len].to_vec();
    let end = index
        .precincts
        .iter()
        .filter(|p| p.tile == tile)
        .map(|p| p.body.end)
        .max()
        .unwrap();
    let payload = &bytes[metadata.payload_offset as usize..end as usize];
    write_tile_part(&mut local, 0, payload, true).unwrap();
    let parsed = parse(&local).unwrap();
    let (tile_rect, _) = single_part1_profile_tile(&local, &parsed).unwrap();
    let contributions =
        parse_default_precinct_lrcp_packets(&local, &parsed, tile_rect, payload).unwrap();
    let prepared = PreparedHtj2kReducedComponentDecode {
        input: &local,
        coding_style: uniform_effective_coding_style(&parsed).unwrap(),
        candidate: metadata.candidate,
        codestream: parsed,
        reconstruction: Htj2kReducedComponentReconstruction::IrreversibleFullImage,
        tile_rect,
        contributions,
        request: Htj2kReducedComponentDecodeRequest {
            component_index: component,
            discard_levels: discard,
        },
        output_width: tile_rect.width.div_ceil(1 << discard),
        output_height: tile_rect.height.div_ceil(1 << discard),
    };
    decode_htj2k_reduced_irreversible_component(
        &prepared,
        payload,
        &mut HtCodestreamDecodeWorkspace::new(),
    )
    .unwrap()
    .2
    .remove(0)
    .samples
}

#[test]
fn deep_sparse_descriptors_preserve_regions_precision_and_fail_closed() {
    for levels in [2, 5, 6] {
        for (bits, components) in [(8, 3), (11, 1), (16, 1), (16, 3)] {
            let (bytes, index) = encode_depth(528, 370, 512, bits, components, levels);
            let mut sparse =
                IndexedLossyHt::sparse(index.profile(), index.encoded_bytes(), index.main_header())
                    .unwrap();
            let region = TileRegionRequest {
                x: 490,
                y: 91,
                width: 38,
                height: 43,
            };
            assert!(sparse.plan(region, 0, &[0]).is_err());
            for ordinal in [1, 0] {
                let descriptor = index.export_tile_descriptor(ordinal).unwrap();
                sparse.import_tile_descriptor(descriptor).unwrap();
                assert!(sparse.import_tile_descriptor(descriptor).is_err());
            }
            for discard in 0..=levels {
                let components = if components == 3 { vec![2, 0] } else { vec![0] };
                let plan = sparse.plan(region, discard, &components).unwrap();
                let actual = decode(&plan, &bytes, &mut Vec::new());
                let full = index
                    .plan(
                        TileRegionRequest {
                            x: 0,
                            y: 0,
                            width: 528,
                            height: 370,
                        },
                        discard,
                        &components,
                    )
                    .unwrap();
                let expected = decode(&full, &bytes, &mut Vec::new());
                let out = plan.output_region();
                let full_out = full.output_region();
                let b = usize::from(bits.div_ceil(8));
                for &component in &components {
                    for tile in 0..2_u16 {
                        let rect = index.tiles[usize::from(tile)].rect;
                        let plan = index
                            .plan(
                                TileRegionRequest {
                                    x: rect.x,
                                    y: rect.y,
                                    width: rect.width,
                                    height: rect.height,
                                },
                                discard,
                                &[component],
                            )
                            .unwrap();
                        let bounded = decode(&plan, &bytes, &mut Vec::new()).remove(0);
                        let complete = full_depth_tile(&bytes, &index, tile, discard, component);
                        assert_eq!(
                            bounded, complete,
                            "complete reference levels={levels} bits={bits} discard={discard} tile={tile}"
                        );
                    }
                }
                for (a, e) in actual.iter().zip(&expected) {
                    for y in 0..out.height as usize {
                        let offset =
                            ((out.y as usize + y) * full_out.width as usize + out.x as usize) * b;
                        assert_eq!(
                            &a[y * out.width as usize * b..(y + 1) * out.width as usize * b],
                            &e[offset..offset + out.width as usize * b],
                            "levels={levels} bits={bits} discard={discard}"
                        );
                    }
                    if bits == 11 {
                        assert!(
                            a.chunks_exact(2)
                                .all(|v| u16::from_le_bytes([v[0], v[1]]) < 2048)
                        );
                    }
                }
                if !plan.block_ranges().is_empty() {
                    assert!(
                        plan.decode(
                            |_, _| Err(invalid(None, None, "compressed cache is incomplete")),
                            &mut ht_lossy::LossyHtSpatialRegionWorkspace::new()
                        )
                        .is_err()
                    );
                }
            }
            let good = index.export_tile_descriptor(0).unwrap();
            for n in 0..good.len() {
                let mut missing = IndexedLossyHt::sparse(
                    index.profile(),
                    index.encoded_bytes(),
                    index.main_header(),
                )
                .unwrap();
                assert!(missing.import_tile_descriptor(&good[..n]).is_err());
            }
            for offset in [0, 8, 17, 18, 22 + 8, 22 + 12, 22 + 42] {
                let mut bad = good.to_vec();
                bad[offset] ^= 0xff;
                let mut missing = IndexedLossyHt::sparse(
                    index.profile(),
                    index.encoded_bytes(),
                    index.main_header(),
                )
                .unwrap();
                assert!(
                    missing.import_tile_descriptor(&bad).is_err(),
                    "offset {offset}"
                );
            }
        }
    }
}

#[test]
#[ignore = "geometry/depth measurement; run explicitly in release mode"]
fn depth_geometry_calibration() {
    for (bits, components) in [(16, 1), (8, 3), (16, 3)] {
        for edge in [256, 512, 1024] {
            for levels in [2, 5, 6] {
                let start = std::time::Instant::now();
                let (bytes, index) = encode_depth(1040, 1138, edge, bits, components, levels);
                let prepare_ms = start.elapsed().as_millis();
                let descriptors: usize = index.tiles.iter().map(|t| t.descriptor.len()).sum();
                let start = std::time::Instant::now();
                let mut sparse = IndexedLossyHt::sparse(
                    index.profile(),
                    index.encoded_bytes(),
                    index.main_header(),
                )
                .unwrap();
                let cols = 1040_u32.div_ceil(edge);
                let tile = (500 / edge * cols + 500 / edge) as u16;
                sparse
                    .import_tile_descriptor(index.export_tile_descriptor(tile).unwrap())
                    .unwrap();
                let import_us = start.elapsed().as_micros();
                let region = TileRegionRequest {
                    x: 500,
                    y: 500,
                    width: 8,
                    height: 8,
                };
                let plan = sparse.plan(region, 0, &[0]).unwrap();
                let packet_bytes: u64 = plan
                    .precinct_indices()
                    .iter()
                    .map(|&i| {
                        let p = &sparse.precincts()[i];
                        p.body.end - p.header.start
                    })
                    .sum();
                let entropy: u64 = plan.block_ranges().iter().map(|r| r.end - r.start).sum();
                let start = std::time::Instant::now();
                for _ in 0..10 {
                    decode(&plan, &bytes, &mut Vec::new());
                }
                let requests_us = start.elapsed().as_micros();
                println!(
                    "depth_probe bits={bits} components={components} edge={edge} levels={levels} source_tile={} stream={} descriptors={} retained={} sparse={} packet={} entropy={} blocks={} coefficients={} workspace={} prepare_ms={prepare_ms} import_us={import_us} ten_requests_us={requests_us}",
                    edge as usize
                        * edge as usize
                        * usize::from(bits.div_ceil(8))
                        * usize::from(components),
                    bytes.len(),
                    descriptors,
                    index.retained_heap_bytes(),
                    sparse.retained_heap_bytes(),
                    packet_bytes,
                    entropy,
                    plan.selected_code_blocks(),
                    plan.selected_block_coefficients(),
                    plan.required_workspace_bytes()
                );
            }
        }
    }
}

#[test]
fn imported_empty_packets_and_native_precision_bounds() {
    for bits in 9..=16 {
        let p = TiledLossyHtProfile {
            width: 16,
            height: 16,
            tile_edge: 512,
            decomposition_levels: 6,
            bits_per_sample: bits,
            components: 1,
            bits_per_pixel: 8.0,
        };
        let mut bytes = Vec::new();
        let index = encode_tiled(
            p,
            |_, planes| {
                for value in planes[0].chunks_exact_mut(2) {
                    value.copy_from_slice(&(1_u16 << (bits - 1)).to_le_bytes());
                }
                Ok(())
            },
            |b| {
                bytes.extend_from_slice(b);
                Ok(())
            },
        )
        .unwrap();
        let mut sparse =
            IndexedLossyHt::sparse(p, index.encoded_bytes(), index.main_header()).unwrap();
        sparse
            .import_tile_descriptor(index.export_tile_descriptor(0).unwrap())
            .unwrap();
        let plan = sparse
            .plan(
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 16,
                    height: 16,
                },
                0,
                &[0],
            )
            .unwrap();
        assert!(plan.block_ranges().is_empty());
        let samples = plan
            .decode(
                |_, _| panic!("empty packet entropy read"),
                &mut ht_lossy::LossyHtSpatialRegionWorkspace::new(),
            )
            .unwrap();
        assert!(
            samples[0]
                .chunks_exact(2)
                .all(|v| u16::from_le_bytes([v[0], v[1]]) == 1 << (bits - 1))
        );
        if bits < 16 {
            assert!(
                encode_tiled(
                    p,
                    |_, planes| {
                        planes[0][..2].copy_from_slice(&(1_u16 << bits).to_le_bytes());
                        Ok(())
                    },
                    |_| Ok(())
                )
                .is_err()
            );
        }
    }
}

#[test]
#[ignore = "selected deep-profile scaling measurement"]
fn deep_profile_scaling_calibration() {
    for (dimension, bits, components) in [(2048, 11, 1), (4096, 11, 1), (2048, 16, 3)] {
        let start = std::time::Instant::now();
        let mut maximum_source = 0;
        let index = encode_tiled(
            TiledLossyHtProfile {
                width: dimension,
                height: dimension,
                tile_edge: 512,
                decomposition_levels: 6,
                bits_per_sample: bits,
                components,
                bits_per_pixel: 2.0,
            },
            |r, p| {
                maximum_source = maximum_source.max(p.iter().map(Vec::len).sum::<usize>());
                authored(r, p, bits)
            },
            |_| Ok(()),
        )
        .unwrap();
        let prepare_ms = start.elapsed().as_millis();
        let mut sparse =
            IndexedLossyHt::sparse(index.profile(), index.encoded_bytes(), index.main_header())
                .unwrap();
        let tile = (dimension / 512 + 1) as u16;
        sparse
            .import_tile_descriptor(index.export_tile_descriptor(tile).unwrap())
            .unwrap();
        let plan = sparse
            .plan(
                TileRegionRequest {
                    x: 901,
                    y: 901,
                    width: 32,
                    height: 32,
                },
                0,
                &[0],
            )
            .unwrap();
        let descriptors: usize = index.tiles.iter().map(|t| t.descriptor.len()).sum();
        let overview = index
            .plan(
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: dimension,
                    height: dimension,
                },
                6,
                &[0],
            )
            .unwrap();
        println!(
            "scaling_probe dimension={dimension} bits={bits} components={components} source_tile={maximum_source} stream={} descriptors={descriptors} retained={} sparse={} blocks={} coefficients={} workspace={} overview_pixels={} overview_packet={} prepare_ms={prepare_ms}",
            index.encoded_bytes(),
            index.retained_heap_bytes(),
            sparse.retained_heap_bytes(),
            plan.selected_code_blocks(),
            plan.selected_block_coefficients(),
            plan.required_workspace_bytes(),
            overview.output.width * overview.output.height,
            overview
                .precinct_indices()
                .iter()
                .map(|&i| {
                    let p = &index.precincts[i];
                    p.body.end - p.header.start
                })
                .sum::<u64>()
        );
    }
}

#[test]
fn streaming_descriptors_match_retained_encoding_and_propagate_sink_failure() {
    let (expected, index) = encode_depth(528, 370, 512, 11, 1, 6);
    let mut bytes = Vec::new();
    let mut descriptors = Vec::new();
    let summary = encode_tiled_to_descriptors(
        index.profile(),
        |r, p| authored(r, p, 11),
        |b| {
            bytes.extend_from_slice(b);
            Ok(())
        },
        |tile, b| {
            descriptors.push((tile, b.to_vec()));
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(summary.encoded_bytes, expected.len() as u64);
    assert_eq!(bytes, expected);
    assert_eq!(summary.tile_count, 2);
    assert_eq!(
        summary.descriptor_bytes,
        descriptors.iter().map(|(_, b)| b.len() as u64).sum::<u64>()
    );
    assert!(summary.peak_tile_index_bytes > 0);
    let mut sparse =
        IndexedLossyHt::sparse(summary.profile, summary.encoded_bytes, summary.main_header)
            .unwrap();
    for (tile, b) in descriptors {
        assert_eq!(b, index.export_tile_descriptor(tile).unwrap());
        sparse.import_tile_descriptor(&b).unwrap();
    }
    let region = TileRegionRequest {
        x: 490,
        y: 91,
        width: 38,
        height: 43,
    };
    assert_eq!(
        decode(
            &sparse.plan(region, 0, &[0]).unwrap(),
            &bytes,
            &mut Vec::new()
        ),
        decode(
            &index.plan(region, 0, &[0]).unwrap(),
            &bytes,
            &mut Vec::new()
        )
    );
    let mut calls = 0;
    assert!(
        encode_tiled_to_descriptors(
            index.profile(),
            |r, p| authored(r, p, 11),
            |_| Ok(()),
            |_, _| {
                calls += 1;
                Err(invalid(None, None, "descriptor sink failed"))
            }
        )
        .is_err()
    );
    assert_eq!(calls, 1);
}

#[test]
#[ignore = "large authored flat-field geometry and streaming storage probe"]
fn large_streaming_geometry_calibration() {
    for (width, height, bits, components) in [
        (4096, 4096, 11, 1),
        (43008, 43008, 11, 1),
        (43280, 19058, 8, 3),
    ] {
        let start = std::time::Instant::now();
        let mut source_max = 0;
        let mut stream_bytes = 0_u64;
        let mut descriptors = 0;
        let mut descriptor_max = 0;
        let mut last = Vec::new();
        let summary = encode_tiled_to_descriptors(
            TiledLossyHtProfile {
                width,
                height,
                tile_edge: 512,
                decomposition_levels: 6,
                bits_per_sample: bits,
                components,
                bits_per_pixel: 2.0,
            },
            |_, planes| {
                source_max = source_max.max(planes.iter().map(Vec::len).sum::<usize>());
                for p in planes {
                    if bits == 8 {
                        p.fill(128);
                    } else {
                        for v in p.chunks_exact_mut(2) {
                            v.copy_from_slice(&1024_u16.to_le_bytes());
                        }
                    }
                }
                Ok(())
            },
            |b| {
                stream_bytes += b.len() as u64;
                Ok(())
            },
            |_, b| {
                descriptors += 1;
                descriptor_max = descriptor_max.max(b.len());
                last.clear();
                last.extend_from_slice(b);
                Ok(())
            },
        )
        .unwrap();
        assert!(summary.peak_tile_index_bytes < 4096);
        assert_eq!(stream_bytes, summary.encoded_bytes);
        assert_eq!(descriptors, usize::from(summary.tile_count));
        let mut sparse =
            IndexedLossyHt::sparse(summary.profile, summary.encoded_bytes, summary.main_header)
                .unwrap();
        sparse.import_tile_descriptor(&last).unwrap();
        let plan = sparse
            .plan(
                TileRegionRequest {
                    x: width - 32,
                    y: height - 32,
                    width: 32,
                    height: 32,
                },
                0,
                &[0],
            )
            .unwrap();
        assert!(plan.block_ranges().is_empty());
        let output = plan
            .decode(
                |_, _| panic!("flat field entropy read"),
                &mut ht_lossy::LossyHtSpatialRegionWorkspace::new(),
            )
            .unwrap();
        assert_eq!(output[0].len(), 32 * 32 * usize::from(bits.div_ceil(8)));
        println!(
            "large_geometry width={width} height={height} bits={bits} components={components} tile_count={} descriptor_count={descriptors} descriptor_bytes={} max_descriptor={descriptor_max} source_tile={source_max} peak_tile_index={} sparse={} stream={stream_bytes} last_width={} last_height={} elapsed_ms={}",
            summary.tile_count,
            summary.descriptor_bytes,
            summary.peak_tile_index_bytes,
            sparse.retained_heap_bytes(),
            sparse.tiles[0].rect.width,
            sparse.tiles[0].rect.height,
            start.elapsed().as_millis()
        );
    }
}

#[test]
fn streaming_metadata_capacity_is_independent_of_tile_count() {
    let mut peaks = Vec::new();
    for width in [512, 1024, 4096] {
        let summary = encode_tiled_to_descriptors(
            TiledLossyHtProfile {
                width,
                height: 512,
                tile_edge: 512,
                decomposition_levels: 6,
                bits_per_sample: 11,
                components: 1,
                bits_per_pixel: 2.0,
            },
            |_, p| {
                for v in p[0].chunks_exact_mut(2) {
                    v.copy_from_slice(&1024_u16.to_le_bytes());
                }
                Ok(())
            },
            |_| Ok(()),
            |_, _| Ok(()),
        )
        .unwrap();
        peaks.push(summary.peak_tile_index_bytes);
    }
    assert!(peaks[0] > 0);
    assert!(peaks.iter().all(|&p| p == peaks[0]));
}

#[test]
fn odd_origin_large_reduced_cross_tile_window_matches_complete_tiles() {
    let mut bytes = Vec::new();
    let index = encode_tiled(
        TiledLossyHtProfile {
            width: 2048,
            height: 2048,
            tile_edge: 512,
            decomposition_levels: 5,
            bits_per_sample: 16,
            components: 1,
            bits_per_pixel: 2.0,
        },
        |r, p| {
            for (i, sample) in p[0].chunks_exact_mut(2).enumerate() {
                let x = r.x + i as u32 % r.width;
                let y = r.y + i as u32 / r.width;
                let mut value = 65535 / 8 + ((x * 13 + y * 7) % 65535) / 2;
                if (x % 257).abs_diff(123) < 3 && (y % 263).abs_diff(111) < 2 {
                    value = 65535;
                }
                if x.is_multiple_of(509) || y.is_multiple_of(503) {
                    value = 65535 * 3 / 4;
                }
                if x % 97 < 5 && y % 89 < 5 {
                    value = (value + 65535 / 100).min(65535);
                }
                sample.copy_from_slice(&(value as u16).to_le_bytes());
            }
            Ok(())
        },
        |b| {
            bytes.extend_from_slice(b);
            Ok(())
        },
    )
    .unwrap();
    let region = TileRegionRequest {
        x: 247,
        y: 117,
        width: 769,
        height: 513,
    };
    for discard in [2, 0, 1, 3, 4, 5] {
        let plan = index.plan(region, discard, &[0]).unwrap();
        let actual = decode(&plan, &bytes, &mut Vec::new());
        let full = index
            .plan(
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 2048,
                    height: 2048,
                },
                discard,
                &[0],
            )
            .unwrap();
        let expected = decode(&full, &bytes, &mut Vec::new());
        let out = plan.output_region();
        for y in 0..out.height as usize {
            let offset = ((out.y as usize + y) * full.output.width as usize + out.x as usize) * 2;
            assert_eq!(
                &actual[0][y * out.width as usize * 2..(y + 1) * out.width as usize * 2],
                &expected[0][offset..offset + out.width as usize * 2]
            );
        }
    }
}
