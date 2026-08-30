//! Bounded HT tile-scoped progression and native component-zero windows.
//!
//! Project-authored from ISO/IEC 15444-1:2024 A.6.6 and B.12.3, physical
//! pages 54–55 and 99, retrieval 34e5d1639b9f121807e620c001893ca9d2c8f977.
//! No external payloads, pixels or implementation material are used.

use super::*;

#[cfg(test)]
std::thread_local! {
    static PACKETS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    static MARKERS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

fn outside(detail: &'static str) -> CodestreamError {
    unsupported(None, None, UnsupportedConstruct::HtBlockDecode, detail)
}

/// A resource/grammar permission, not native admission. In particular CAP's
/// cleanup bound and actual coding-set multiplicity are checked after packets.
pub(super) fn envelope(c: &Codestream) -> bool {
    c.kind == CodestreamKind::Htj2k
        && c.siz.components.len() == 3
        && c.siz.components.iter().all(|s| {
            (8..=16).contains(&s.bits_per_sample)
                && s.horizontal_separation == 1
                && s.vertical_separation == 1
        })
        && c.siz.image_origin_x == 0
        && c.siz.image_origin_y == 0
        && c.siz.tile_origin_x == 0
        && c.siz.tile_origin_y == 0
        && matches!(c.siz.tile_width, 32 | 64 | 128)
        && matches!(c.siz.tile_height, 32 | 64 | 128)
        && c.siz
            .tile_count_x()
            .ok()
            .zip(c.siz.tile_count_y().ok())
            .and_then(|(x, y)| x.checked_mul(y))
            .is_some_and(|n| n <= 256)
        && c.tiles.len() <= 257
        && c.component_coding_styles.is_empty()
        && c.coding_style.is_some_and(|s| {
            s.entropy_coder == EntropyCoder::HtBlockCoding
                && s.code_block_style == 0x40
                && s.decomposition_levels == 3
                && s.transform == WaveletTransform::Reversible53
                && (1..=8).contains(&s.layers)
                && s.progression_order == ProgressionOrder::Rlcp
                && !s.multiple_component_transform
                && matches!(s.code_block_width_exponent, 5 | 6)
                && matches!(s.code_block_height_exponent, 5 | 6)
                && s.precincts_declared
                && s.precinct_exponents[..4] == [0x77, 0x88, 0x88, 0x88]
        })
        && markers_supported(c)
}

fn markers_supported(c: &Codestream) -> bool {
    let (mut tile, mut cod, mut qcd, mut poc) = (false, 0, 0, 0);
    for m in &c.markers {
        match m.marker {
            Marker::Sot => tile = true,
            Marker::Cod if !tile => cod += 1,
            Marker::Qcd if !tile => qcd += 1,
            Marker::Poc if tile => poc += 1,
            Marker::Soc
            | Marker::Siz
            | Marker::Cap
            | Marker::Cpf
            | Marker::Com
            | Marker::Sod
            | Marker::Eoc => {}
            _ => return false,
        }
    }
    tile && cod == 1 && qcd == 1 && poc == 2
}

/// Partition retained headers once. The shared packet machinery sees the main
/// header plus at most two local parts, avoiding a global marker scan per tile.
fn tile_scopes(c: &Codestream) -> Result<Vec<Codestream>> {
    let count = c
        .siz
        .tile_count_x()?
        .checked_mul(c.siz.tile_count_y()?)
        .ok_or(CodestreamError::SizeOverflow)? as usize;
    let first = c
        .markers
        .iter()
        .position(|m| m.marker == Marker::Sot)
        .ok_or(CodestreamError::SizeOverflow)?;
    let mut base = Codestream {
        kind: c.kind,
        siz: c.siz.clone(),
        coding_style: c.coding_style,
        component_coding_styles: Vec::new(),
        capability: c.capability.clone(),
        profile: c.profile.clone(),
        corresponding_profile: c.corresponding_profile.clone(),
        tile_part_lengths: None,
        markers: c.markers[..first].to_vec(),
        tiles: Vec::new(),
        progression: c.progression.clone(),
    };
    // Informational comments do not affect packet grammar; avoid multiplying
    // an arbitrarily long main-header comment list by the tile count.
    base.markers.retain(|m| m.marker != Marker::Com);
    let mut scopes = alloc::vec![base; count];
    let ranges = tile_part_header_marker_ranges(&c.markers);
    if ranges.len() != c.tiles.len() {
        return Err(CodestreamError::SizeOverflow);
    }
    for ((start, end), part) in ranges.into_iter().zip(&c.tiles) {
        let scope = scopes
            .get_mut(usize::from(part.tile_index))
            .ok_or(CodestreamError::SizeOverflow)?;
        scope.tiles.push(*part);
        scope.markers.extend(
            c.markers[start..=end]
                .iter()
                .filter(|m| m.marker != Marker::Com)
                .copied(),
        );
    }
    Ok(scopes)
}

fn schedule(input: &[u8], c: &Codestream, tile: TileRect) -> Result<()> {
    let layers = c.coding_style.ok_or(CodestreamError::SizeOverflow)?.layers;
    let v = effective_progression_volumes(input, c, tile.tile_index, layers, 4)?;
    let complete = |p: &ProgressionVolume| {
        p.resolution_start == 0
            && p.component_start == 0
            && p.component_end >= 3
            && p.layer_end >= layers
    };
    let admitted = if tile.tile_index == 0 {
        matches!(c.tiles.as_slice(), [a,b] if a.tile_part_index == 0
            && b.tile_part_index == 1 && a.tile_part_count.is_none_or(|n| n == 2)
            && b.tile_part_count == Some(2))
            && matches!(v.as_slice(), [a,b] if complete(a) && complete(b)
                && (1..=3).contains(&a.resolution_end) && b.resolution_end >= 4
                && a.progression_order == ProgressionOrder::Lrcp
                && b.progression_order == ProgressionOrder::Lrcp
                && a.declared_tile_part == 0 && b.declared_tile_part == 1)
    } else {
        matches!(c.tiles.as_slice(), [a] if a.tile_part_index == 0 && a.tile_part_count == Some(1))
            && matches!(v.as_slice(), [a] if complete(a) && a.resolution_end == 4
                && a.progression_order == ProgressionOrder::Rlcp && a.marker_offset.is_none())
    };
    if !admitted {
        return Err(outside(
            "HT tile windows require two tile-zero LRCP volumes and one RLCP part in every other tile",
        ));
    }
    Ok(())
}

struct Selected {
    codestream: Codestream,
    contributions: Vec<PacketCodeBlockContribution>,
    tile: TileRect,
}

/// Validate every tile, retaining only selected packet state. Native-only
/// declines cannot conceal a later SINGLEHT contradiction or malformed packet.
fn walk(input: &[u8], c: &Codestream) -> Result<Selected> {
    if input.len() > 64 * 1024 * 1024 || !envelope(c) {
        return Err(outside(
            "HT tile window exceeds its bounded header/resource envelope",
        ));
    }
    let scopes = tile_scopes(c)?;
    let rects = tile_rects(c)?;
    let mut selected = None;
    let mut decline = None;
    for (scope, tile) in scopes.into_iter().zip(rects) {
        if let Err(e) = schedule(input, &scope, tile) {
            decline.get_or_insert(e);
            continue;
        }
        #[cfg(test)]
        MARKERS.with(|n| n.set(n.get() + scope.markers.len()));
        let payload = tile_payload_spans_for_rect(input, &scope, tile)?;
        let packets = match parse_default_precinct_packets_from_source_with_ht_retention(
            input,
            &scope,
            tile,
            &payload,
            None,
            None,
            None,
            PacketOrganisationConfig::HT_TILE_WINDOW,
            None,
            HtCodingSetRetention::NativeAdmission,
        ) {
            Ok(p) => p,
            Err(e @ CodestreamError::Unsupported { .. }) => {
                decline.get_or_insert(e);
                continue;
            }
            Err(e) => return Err(e),
        };
        #[cfg(test)]
        PACKETS.with(|n| n.set(n.get() + packets.packet_count));
        if let Err(d) = classify_htonly_native_packet_mechanisms(&packets.contributions) {
            decline.get_or_insert_with(|| outside(d.detail));
        }
        if tile.tile_index == 0 {
            selected = Some(Selected {
                codestream: scope,
                tile,
                contributions: packets
                    .contributions
                    .into_iter()
                    .filter(|p| p.component_index == 0)
                    .collect(),
            });
        }
    }
    if let Some(e) = decline {
        return Err(e);
    }
    selected.ok_or(CodestreamError::SizeOverflow)
}

pub(super) fn validate_signalling(input: &[u8], c: &Codestream) -> Result<()> {
    if c.capability
        .as_ref()
        .and_then(|p| p.part15)
        .is_some_and(|p| {
            p.multiple_ht_sets_allowed || p.code_block_mode == Part15CodeBlockMode::Mixed
        })
    {
        return Ok(());
    }
    if let Err(
        e @ CodestreamError::InvalidMarker {
            marker: Some(Marker::Cap),
            ..
        },
    ) = walk(input, c)
    {
        return Err(e);
    }
    Ok(())
}

/// Prepared raw native window. All tiles are packet-validated; only component
/// zero of tile zero is reconstructed, without inverse colour transformation.
#[cfg(feature = "std")]
pub struct PreparedHtj2kTileWindowDecode<'a> {
    input: &'a [u8],
    selected: Selected,
    region: TileRegionRequest,
    candidate: HtCodestreamDecodeCandidate,
    transfer: HtReversibleCodeBlockTransfer,
}

#[cfg(feature = "std")]
impl PreparedHtj2kTileWindowDecode<'_> {
    pub fn bits_per_sample(&self) -> u8 {
        self.selected.codestream.siz.components[0].bits_per_sample
    }
    pub fn signed(&self) -> bool {
        self.selected.codestream.siz.components[0].signed
    }
}

#[cfg(feature = "std")]
pub fn prepare_htj2k_tile_window_decode(
    input: &[u8],
    region: TileRegionRequest,
) -> Result<Option<PreparedHtj2kTileWindowDecode<'_>>> {
    let c = parse(input)?;
    if !envelope(&c) {
        return Ok(None);
    }
    let cap = c
        .capability
        .as_ref()
        .and_then(|p| p.part15)
        .ok_or(CodestreamError::SizeOverflow)?;
    if cap.code_block_mode != Part15CodeBlockMode::HtOnly {
        return Err(outside("HT tile windows require HTONLY packet grammar"));
    }
    let selected = walk(input, &c)?;
    if cap.code_block_mode != Part15CodeBlockMode::HtOnly || cap.cleanup_magnitude_bound > 18 {
        return Err(outside(
            "HT tile windows require HTONLY and cleanup magnitude at most eighteen",
        ));
    }
    if region.width == 0
        || region.height == 0
        || region
            .x
            .checked_add(region.width)
            .is_none_or(|n| n > selected.tile.width)
        || region
            .y
            .checked_add(region.height)
            .is_none_or(|n| n > selected.tile.height)
    {
        return Err(outside("HT native window must lie inside tile zero"));
    }
    let q = parse_component_quantization(input, &selected.codestream, 10)?;
    if q.iter().any(|q| {
        q.style != transform::QuantizationStyle::NoQuantization
            || q.steps.iter().any(|s| {
                s.exponent == 0
                    || s.exponent
                        .checked_add(q.guard_bits)
                        .and_then(|n| n.checked_sub(1))
                        .is_none_or(|n| n > 30)
            })
    }) {
        return Err(outside(
            "HT tile windows require bounded reversible quantisers",
        ));
    }
    let q = &q[0];
    let k_max = q.steps[0].exponent + q.guard_bits - 1;
    let transfer = HtReversibleCodeBlockTransfer {
        qcd_guard_bits: q.guard_bits,
        qcd_exponent: q.steps[0].exponent,
        k_max,
        shift: 31 - k_max,
    };
    let mut state = ht_marker_state(&selected.codestream).ok_or(CodestreamError::SizeOverflow)?;
    state.components = 1;
    state.all_components_same_sample_format = true;
    state.single_tile = true;
    let marker_candidate =
        ht::plan_decode_candidate(state).map_err(|d| outside(d.reason.message()))?;
    let candidate = HtCodestreamDecodeCandidate {
        marker_candidate,
        tile_part: selected.codestream.tiles[0],
    };
    Ok(Some(PreparedHtj2kTileWindowDecode {
        input,
        selected,
        region,
        candidate,
        transfer,
    }))
}

#[cfg(feature = "std")]
pub fn decode_prepared_htj2k_tile_window_owned(
    p: &PreparedHtj2kTileWindowDecode<'_>,
) -> Result<DecodedImage> {
    let s = &p.selected;
    // A small selected tile is materialised in memory for the existing HT
    // numerical seam. Unselected tile bodies are never copied or decoded.
    let payload = tile_payload_for_rect(p.input, &s.codestream, s.tile)?;
    let (_, _, components) = decode_htj2k_lossless_decomp_components(
        p.candidate,
        &payload,
        &s.contributions,
        p.transfer,
        &s.codestream,
        s.codestream
            .coding_style
            .ok_or(CodestreamError::SizeOverflow)?,
        s.tile,
        Some(Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 0,
        }),
        None,
        &mut HtCodestreamDecodeWorkspace::new(),
    )?;
    let bytes = usize::from(p.bits_per_sample().div_ceil(8));
    let stride = p.region.width as usize * bytes;
    let mut samples = alloc::vec![0; stride * p.region.height as usize];
    copy_tile_intersection_to_region(
        &mut samples,
        stride,
        bytes,
        p.region,
        s.tile,
        p.region,
        &components[0].samples,
    )?;
    Ok(DecodedImage {
        width: p.region.width,
        height: p.region.height,
        bits_per_sample: p.bits_per_sample(),
        signed: p.signed(),
        components: alloc::vec![DecodedComponent { samples }],
    })
}

/// Project-authored tile progression test support, not an application encoder.
#[cfg(any(test, feature = "test-fixtures"))]
#[doc(hidden)]
pub fn encode_htj2k_tile_window_test_fixture(tiles: u16, multiple_sets: bool) -> Result<Vec<u8>> {
    fixture(tiles, 32, 12, true, 2, 2, true, true, multiple_sets)
}

#[cfg(any(test, feature = "test-fixtures"))]
#[allow(clippy::too_many_arguments)]
fn fixture(
    tiles: u16,
    side: u32,
    bits: u8,
    signed: bool,
    layers: u16,
    split: u8,
    sop: bool,
    eph: bool,
    multiple_sets: bool,
) -> Result<Vec<u8>> {
    if !(1..=256).contains(&tiles)
        || !matches!(side, 32 | 64 | 128)
        || !(8..=16).contains(&bits)
        || !(1..=8).contains(&layers)
        || !(1..=3).contains(&split)
        || (multiple_sets && layers < 2)
    {
        return Err(CodestreamError::SizeOverflow);
    }
    let mut out = Vec::new();
    write_native_main_header(
        &mut out,
        side * u32::from(tiles),
        side,
        side,
        side,
        bits,
        3,
        false,
        3,
        &[8; 10],
        true,
        0,
        layers,
    )?;
    for c in 0..3 {
        out[42 + 3 * c] |= if signed { 0x80 } else { 0 };
    }
    let cap = find_marker(&out, 0, Marker::Cap).ok_or(CodestreamError::SizeOverflow)?;
    out[cap + 8] |= 0x08;
    let cod = find_marker(&out, 0, Marker::Cod).ok_or(CodestreamError::SizeOverflow)?;
    out[cod + 2..cod + 4].copy_from_slice(&16_u16.to_be_bytes());
    out[cod + 4] = 1 | (u8::from(sop) << 1) | (u8::from(eph) << 2);
    out[cod + 5] = 1;
    out.splice(cod + 14..cod + 14, [0x77, 0x88, 0x88, 0x88]);
    let q = find_marker(&out, 0, Marker::Qcd).ok_or(CodestreamError::SizeOverflow)?;
    out[q + 4] = 0x40;
    let mut trailing = Vec::new();
    for tile in 0..tiles {
        let mut segments = Vec::new();
        let mut bands = Vec::new();
        for component in 0..3 {
            let plane = (0..side * side)
                .map(|i| ((i * 7 + i / side + component * 3) % 13) as i32 - 6)
                .collect::<Vec<_>>();
            bands.push(
                decomp_subband_specs(side, side, 3)?
                    .into_iter()
                    .map(|s| encode_ht_decomp_subband(side, &plane, s, 9, &mut segments))
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        let mut sequence = 0_u16;
        let mut parts = [Vec::new(), Vec::new()];
        let mut order = Vec::new();
        if tile == 0 {
            for (part, rs, re) in [(0, 0, split), (1, split, 4)] {
                for l in 0..layers {
                    for r in rs..re {
                        for c in 0..3 {
                            order.push((part, l, r, c));
                        }
                    }
                }
            }
        } else {
            for r in 0..4 {
                for l in 0..layers {
                    for c in 0..3 {
                        order.push((0, l, r, c));
                    }
                }
            }
        }
        for (part, layer, resolution, component) in order {
            let packet = &mut parts[part];
            if sop {
                packet.extend_from_slice(&[0xff, 0x91, 0, 4]);
                packet.extend_from_slice(&sequence.to_be_bytes());
            }
            sequence += 1;
            let active = bands[component]
                .iter()
                .filter(|s| s.resolution == resolution)
                .collect::<Vec<_>>();
            let repeated = multiple_sets
                && tile == tiles - 1
                && component == 2
                && resolution == 0
                && layer == 1;
            if repeated {
                let b = &active[0].code_blocks[0];
                let cleanup = &segments[b.segment_offset..b.segment_offset + b.segment_len];
                let repeated = repeated_cleanup_second_ht_set_packet(cleanup)?;
                let header_len = repeated.len() - cleanup.len();
                packet.extend_from_slice(&repeated[..header_len]);
                if eph {
                    packet.extend_from_slice(&[0xff, 0x92]);
                }
                packet.extend_from_slice(cleanup);
            } else {
                let mut writer = PacketBitWriter::new();
                writer.write_bit(u32::from(layer == 0))?;
                if layer == 0 {
                    for b in &active {
                        write_component_packet_header(
                            &mut writer,
                            b.code_block_cols,
                            b.code_block_rows,
                            &b.code_blocks,
                        )?;
                    }
                }
                writer.align();
                packet.extend_from_slice(writer.bytes());
                if eph {
                    packet.extend_from_slice(&[0xff, 0x92]);
                }
                if layer == 0 {
                    for band in active {
                        for b in band.code_blocks.iter().filter(|b| b.included) {
                            packet.extend_from_slice(
                                &segments[b.segment_offset..b.segment_offset + b.segment_len],
                            );
                        }
                    }
                }
            }
        }
        let count = if tile == 0 { 2 } else { 1 };
        for part in 0..count {
            let dst = if part == 1 { &mut trailing } else { &mut out };
            let start = dst.len();
            dst.extend_from_slice(&[0xff, 0x90, 0, 10]);
            dst.extend_from_slice(&tile.to_be_bytes());
            dst.extend_from_slice(&[0; 4]);
            dst.extend_from_slice(&[part, if tile == 0 && part == 0 { 0 } else { count }]);
            if tile == 0 {
                dst.extend_from_slice(&[0xff, 0x5f, 0, 9, 0, 0]);
                dst.extend_from_slice(&(layers + 1).to_be_bytes());
                dst.extend_from_slice(&[if part == 0 { split } else { 33 }, 3, 0]);
            }
            dst.extend_from_slice(&[0xff, 0x93]);
            dst.extend_from_slice(&parts[usize::from(part)]);
            let len = (dst.len() - start) as u32;
            dst[start + 6..start + 10].copy_from_slice(&len.to_be_bytes());
        }
    }
    out.extend_from_slice(&trailing);
    out.extend_from_slice(&[0xff, 0xd9]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn region() -> TileRegionRequest {
        TileRegionRequest {
            x: 3,
            y: 5,
            width: 17,
            height: 19,
        }
    }

    #[test]
    fn tile_windows_reconstruct_authored_coefficients_across_native_formats() {
        for (side, bits, signed, layers, split, sop, eph) in [
            (32, 8, false, 1, 1, false, false),
            (64, 12, true, 2, 2, true, false),
            (128, 16, false, 8, 3, false, true),
            (128, 8, true, 8, 1, true, true),
        ] {
            let bytes = fixture(3, side, bits, signed, layers, split, sop, eph, false).unwrap();
            let prepared = prepare_htj2k_tile_window_decode(&bytes, region())
                .unwrap()
                .unwrap();
            let decoded = decode_prepared_htj2k_tile_window_owned(&prepared).unwrap();
            let mut oracle = (0..side * side)
                .map(|i| ((i * 7 + i / side) % 13) as i32 - 6)
                .collect::<Vec<_>>();
            let mut scratch = vec![0; side as usize * 4];
            inverse_reversible_5_3_levels_with_scratch(
                &mut oracle,
                side as usize,
                side,
                side,
                3,
                &mut scratch,
            )
            .unwrap();
            let mut expected = Vec::new();
            for y in 5..24 {
                for x in 3..20 {
                    let v =
                        oracle[(y * side + x) as usize] + if signed { 0 } else { 1 << (bits - 1) };
                    let v = if signed {
                        v.clamp(-(1 << (bits - 1)), (1 << (bits - 1)) - 1)
                    } else {
                        v.clamp(0, (1 << bits) - 1)
                    };
                    expected.push(v as u8);
                    if bits > 8 {
                        expected.push((v >> 8) as u8);
                    }
                }
            }
            assert_eq!(decoded.components[0].samples, expected);
            assert_eq!((decoded.bits_per_sample, decoded.signed), (bits, signed));
            assert!(
                prepared
                    .selected
                    .contributions
                    .iter()
                    .all(|p| p.component_index == 0)
            );
        }
    }

    #[test]
    fn tile_windows_bound_global_work_and_reject_large_grids_before_packets() {
        for tiles in [1, 16, 256] {
            let bytes = encode_htj2k_tile_window_test_fixture(tiles, false).unwrap();
            PACKETS.with(|n| n.set(0));
            MARKERS.with(|n| n.set(0));
            prepare_htj2k_tile_window_decode(&bytes, region())
                .unwrap()
                .unwrap();
            PACKETS.with(|n| assert_eq!(n.get(), u64::from(tiles) * 24));
            MARKERS.with(|n| assert!(n.get() <= usize::from(tiles) * 10 + 10));
            let mut c = parse(&bytes).unwrap();
            c.siz.reference_grid_width = 32 * 257;
            PACKETS.with(|n| n.set(0));
            assert!(!envelope(&c));
            assert!(walk(&bytes, &c).is_err());
            PACKETS.with(|n| assert_eq!(n.get(), 0));
        }
    }

    #[test]
    fn tile_windows_validate_unselected_sets_before_native_declines() {
        let bytes = encode_htj2k_tile_window_test_fixture(3, true).unwrap();
        for bound in [10, 11] {
            let mut bytes = bytes.clone();
            let cap = find_marker(&bytes, 0, Marker::Cap).unwrap();
            bytes[cap + 9] = bound;
            for window in [region(), TileRegionRequest { x: 31, ..region() }] {
                assert!(matches!(
                    prepare_htj2k_tile_window_decode(&bytes, window),
                    Err(CodestreamError::InvalidMarker {
                        marker: Some(Marker::Cap),
                        ..
                    })
                ));
            }
            validate_part15_packet_signalling(&bytes, &parse(&bytes).unwrap()).unwrap_err();
            bytes[cap + 8] |= 0x20;
            validate_part15_packet_signalling(&bytes, &parse(&bytes).unwrap()).unwrap();
            assert!(matches!(
                prepare_htj2k_tile_window_decode(&bytes, region()),
                Err(CodestreamError::Unsupported { .. })
            ));
        }
        for sets in [false, true] {
            let mut input = encode_htj2k_tile_window_test_fixture(3, sets).unwrap();
            let q = find_marker(&input, 0, Marker::Qcd).unwrap();
            input[q + 5..q + 15].fill(30 << 3); // Thirty-one structural magnitude bits.
            let result = prepare_htj2k_tile_window_decode(&input, region());
            if sets {
                assert!(matches!(
                    result,
                    Err(CodestreamError::InvalidMarker {
                        marker: Some(Marker::Cap),
                        ..
                    })
                ));
            } else {
                assert!(matches!(result, Err(CodestreamError::Unsupported { .. })));
            }
        }
    }

    #[test]
    fn tile_windows_reject_late_packets_premature_volumes_and_neighbouring_headers() {
        let bytes = encode_htj2k_tile_window_test_fixture(3, false).unwrap();
        let c = parse(&bytes).unwrap();
        let last = c.tiles.iter().find(|t| t.tile_index == 2).unwrap();
        let mut malformed = bytes.clone();
        malformed[last.payload_offset.unwrap() + last.payload_len.unwrap() - 1] = 0xff;
        assert!(matches!(
            prepare_htj2k_tile_window_decode(&malformed, region()),
            Err(CodestreamError::InvalidMarker { .. })
        ));
        let first_poc = c.markers.iter().find(|m| m.marker == Marker::Poc).unwrap();
        let mut premature = bytes.clone();
        premature[first_poc.data_offset + 4] = 3;
        assert!(prepare_htj2k_tile_window_decode(&premature, region()).is_err());
        let cap = c.markers.iter().find(|m| m.marker == Marker::Cap).unwrap();
        malformed[cap.data_offset + 5] = 11;
        assert!(matches!(
            prepare_htj2k_tile_window_decode(&malformed, region()),
            Err(CodestreamError::InvalidMarker { .. })
        ));

        let first = c
            .markers
            .iter()
            .find(|m| m.marker == Marker::Sot)
            .unwrap()
            .offset;
        for marker in [
            vec![
                0xff, 0x53, 0, 13, 0, 1, 3, 4, 4, 0x40, 1, 0x77, 0x88, 0x88, 0x88,
            ],
            [vec![0xff, 0x5d, 0, 14, 0, 0x40], vec![0x40; 10]].concat(),
            vec![0xff, 0x5e, 0, 5, 1, 0, 1],
            vec![0xff, 0x5f, 0, 9, 0, 0, 0, 2, 4, 3, 1],
        ] {
            let mut altered = bytes.clone();
            altered[cap.data_offset + 4] |= 0x10;
            altered.splice(first..first, marker);
            assert!(!envelope(&parse(&altered).unwrap()));
            assert!(
                prepare_htj2k_tile_window_decode(&altered, region())
                    .unwrap()
                    .is_none()
            );
        }
        let cod = c.markers.iter().find(|m| m.marker == Marker::Cod).unwrap();
        let mut mct = bytes.clone();
        mct[cod.data_offset + 4] = 1;
        assert!(
            prepare_htj2k_tile_window_decode(&mct, region())
                .unwrap()
                .is_none()
        );
        // Scope membership, not marker bytes alone, controls override admission.
        for marker in [Marker::Cod, Marker::Qcd] {
            let m = c.markers.iter().find(|m| m.marker == marker).unwrap();
            let raw = &bytes[m.offset..m.data_offset + m.data_len];
            let mut altered = bytes.clone();
            let len = read_u32(&altered, first + 6).unwrap() + raw.len() as u32;
            altered[first + 6..first + 10].copy_from_slice(&len.to_be_bytes());
            altered.splice(first + 12..first + 12, raw.iter().copied());
            assert!(!envelope(&parse(&altered).unwrap()));
        }
    }
}
