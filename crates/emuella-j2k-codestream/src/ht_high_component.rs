//! HT-owned high-component packet admission and native component-zero output.
//!
//! Project-authored from ISO/IEC 15444-1:2024 A.6.1–A.6.6 and B.12
//! (physical pages 46, 49–55, 96 and 98–99), retrieval
//! 34e5d1639b9f121807e620c001893ca9d2c8f977; Part 15:2019 A.5 (page 38),
//! retrieval 10baf9472429d52f5d6b5f9b7a892dbed395b1db. No external payload,
//! reference pixels or implementation source is used.

use super::*;

#[cfg(test)]
std::thread_local! {
    static STRUCTURAL_CALLS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

fn outside(detail: &'static str) -> CodestreamError {
    unsupported(None, None, UnsupportedConstruct::HtBlockDecode, detail)
}

fn style_supported(s: CodingStyleMarker) -> bool {
    s.entropy_coder == EntropyCoder::HtBlockCoding
        && s.code_block_style == 0x40
        && s.decomposition_levels == 1
        && s.transform == WaveletTransform::Reversible53
        && (1..=2).contains(&s.layers)
        && s.progression_order == ProgressionOrder::Rlcp
        && s.multiple_component_transform
        && !s.sop_markers
        && !s.eph_markers
        && matches!(s.code_block_width_exponent, 5 | 6)
        && matches!(s.code_block_height_exponent, 5 | 6)
        && s.precincts_declared
        && s.precinct_exponents[..2] == [0x77, 0x88]
}

/// Resource and grammar preflight, before any packet topology or walking.
pub(super) fn envelope(c: &Codestream) -> bool {
    c.kind == CodestreamKind::Htj2k
        && (4..=257).contains(&c.siz.components.len())
        && c.siz.components.iter().all(|s| {
            (8..=16).contains(&s.bits_per_sample)
                && s.horizontal_separation == 1
                && s.vertical_separation == 1
        })
        && c.siz.components[..3].iter().all(|s| {
            s.bits_per_sample == c.siz.components[0].bits_per_sample
                && s.signed == c.siz.components[0].signed
        })
        && (1..=64).contains(&c.image_width())
        && (1..=64).contains(&c.image_height())
        && c.siz.image_origin_x == 0
        && c.siz.image_origin_y == 0
        && c.siz.tile_origin_x == 0
        && c.siz.tile_origin_y == 0
        && c.siz.tile_width == c.image_width()
        && c.siz.tile_height == c.image_height()
        && matches!(c.tiles.as_slice(), [t] if t.tile_index == 0
            && t.tile_part_index == 0 && t.tile_part_count == Some(1))
        && c.coding_style.is_some_and(style_supported)
        && c.component_coding_styles
            .iter()
            .all(|s| style_supported(s.coding_style))
        && markers_supported(c.markers.iter())
}

fn markers_supported<'a>(markers: impl Iterator<Item = &'a MarkerSegment>) -> bool {
    let (mut tile, mut qcd, mut rgn, mut poc) = (false, 0, 0, 0);
    for m in markers {
        match m.marker {
            Marker::Sot => tile = true,
            Marker::Cod | Marker::Coc | Marker::Qcc if !tile => {}
            Marker::Qcd if !tile => qcd += 1,
            Marker::Rgn if !tile => rgn += 1,
            Marker::Poc if !tile => poc += 1,
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
    tile && qcd == 1 && rgn == 1 && poc == 1
}

pub(super) fn resolve_maxshift(input: &[u8], c: &Codestream) -> Result<BoundedTileMaxshift> {
    if !envelope(c) {
        return Err(outside(
            "HT high-component output requires its bounded main-header envelope",
        ));
    }
    let marker = c
        .markers
        .iter()
        .find(|m| m.marker == Marker::Rgn)
        .ok_or(CodestreamError::SizeOverflow)?;
    let rgn = parse_rgn_declaration(input, marker, &c.siz)?;
    // Legal but unimplemented shifts are still available to structural packet
    // validation. Native admission applies its narrower bound afterwards.
    Ok(BoundedTileMaxshift {
        tile_index: 0,
        component_index: rgn.component_index,
        shift: rgn.shift,
    })
}

pub(super) fn validate_schedule(c: &Codestream, volumes: &[ProgressionVolume]) -> Result<()> {
    if !matches!(volumes, [a, b] if
        a.resolution_start == 0 && b.resolution_start == 0
        && a.resolution_end >= 2 && b.resolution_end >= 2
        && a.layer_end == c.coding_style.map_or(0, |s| s.layers) && b.layer_end == a.layer_end
        && a.component_start == 0 && a.component_end == b.component_start
        && b.component_end == c.siz.component_count()
        && a.progression_order == ProgressionOrder::Rlcp
        && b.progression_order == ProgressionOrder::Cprl
        && a.declared_tile_part == 0 && b.declared_tile_part == 0)
    {
        return Err(outside(
            "HT high-component output requires adjacent complete RLCP and CPRL component volumes",
        ));
    }
    Ok(())
}

/// Prepare native full-resolution component zero after validating all packets.
///
/// This independent HT route admits the documented high-component main-header
/// ROI envelope. ROI must belong to an unselected component. The returned plan
/// reuses reversible selected-plane execution with zero discarded levels;
/// it neither performs inverse RCT nor reconstructs discarded components.
#[cfg(feature = "std")]
pub fn prepare_htj2k_high_component_decode(
    input: &[u8],
) -> Result<Option<PreparedHtj2kReducedComponentDecode<'_>>> {
    let c = parse(input)?;
    if !envelope(&c) {
        return Ok(None);
    }
    let roi = resolve_maxshift(input, &c)?;
    #[cfg(test)]
    STRUCTURAL_CALLS.with(|n| n.set(n.get() + 1));
    validate_part15_packet_signalling(input, &c)?;
    let config = PacketOrganisationConfig::HT_HIGH_COMPONENT;
    let styles = packet_component_styles(&c, config)?;
    let counts = alloc::vec![4; styles.len()];
    let q = parse_component_quantization_for_styles(input, &c, &styles, &counts, 4, None)?;
    let (tile_rect, payload) = single_part1_profile_tile(input, &c)?;
    // Complete packet validation precedes native-only capability declines,
    // including an ROI on zero and a legal but unsupported cleanup bound.
    let contributions = parse_default_precinct_packets_from_source_with_ht_retention(
        input,
        &c,
        tile_rect,
        &ContiguousPacketSource { bytes: payload },
        None,
        None,
        None,
        config,
        None,
        HtCodingSetRetention::NativeAdmission,
    )?
    .contributions;
    let cap = c
        .capability
        .as_ref()
        .and_then(|p| p.part15)
        .ok_or(CodestreamError::SizeOverflow)?;
    if cap.code_block_mode != Part15CodeBlockMode::HtOnly
        || cap.cleanup_magnitude_bound > 18
        || styles[0].layers != 1
        || roi.component_index == 0
        || !(1..=15).contains(&roi.shift)
    {
        return Err(outside(
            "HT high-component output requires HTONLY, cleanup bound at most eighteen and one unselected Maxshift of one through fifteen",
        ));
    }
    for (i, quantiser) in q.iter().enumerate() {
        if quantiser.style != transform::QuantizationStyle::NoQuantization
            || quantiser.steps.iter().any(|s| {
                s.exponent == 0
                    || s.exponent
                        .checked_add(quantiser.guard_bits)
                        .and_then(|n| n.checked_sub(1))
                        .and_then(|n| {
                            n.checked_add(if i == usize::from(roi.component_index) {
                                roi.shift
                            } else {
                                0
                            })
                        })
                        .is_none_or(|n| n > 30)
            })
        {
            return Err(outside(
                "HT high-component output requires bounded reversible quantisers for every component",
            ));
        }
    }
    classify_htonly_native_packet_mechanisms(&contributions).map_err(|d| outside(d.detail))?;
    let coding_style = styles[0];
    let mut state = ht_marker_state(&c).ok_or(CodestreamError::SizeOverflow)?;
    state.components = 1;
    state.all_components_same_sample_format = true;
    state.reversible_transform = true;
    state.packet_progression_supported = true;
    state.precincts_declared = false;
    state.code_block_width =
        ht_code_block_dimension_from_exponent(coding_style.code_block_width_exponent);
    state.code_block_height =
        ht_code_block_dimension_from_exponent(coding_style.code_block_height_exponent);
    let marker_candidate =
        ht::plan_decode_candidate(state).map_err(|d| outside(d.reason.message()))?;
    let k_max = q[0].steps[0].exponent + q[0].guard_bits - 1;
    let transfer = HtReversibleCodeBlockTransfer {
        qcd_guard_bits: q[0].guard_bits,
        qcd_exponent: q[0].steps[0].exponent,
        k_max,
        shift: 31 - k_max,
    };
    Ok(Some(PreparedHtj2kReducedComponentDecode {
        input,
        candidate: HtCodestreamDecodeCandidate {
            marker_candidate,
            tile_part: c.tiles[0],
        },
        coding_style,
        reconstruction: Htj2kReducedComponentReconstruction::Reversible(transfer),
        tile_rect,
        contributions: contributions
            .into_iter()
            .filter(|p| p.component_index == 0)
            .collect(),
        request: Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 0,
        },
        output_width: c.image_width(),
        output_height: c.image_height(),
        codestream: c,
    }))
}

/// Project-authored high-component packet fixture, not an application encoder.
#[doc(hidden)]
pub fn encode_htj2k_high_component_test_fixture(
    width: u32,
    height: u32,
    components: u16,
) -> Result<Vec<u8>> {
    if !(4..=257).contains(&components) || !(1..=64).contains(&width) || !(1..=64).contains(&height)
    {
        return Err(CodestreamError::SizeOverflow);
    }
    fixture(
        width,
        height,
        components,
        8,
        false,
        components - 1,
        3,
        components / 2,
        false,
    )
}

/// Project-authored contradiction for structural/admission parity tests.
#[cfg(any(test, feature = "test-fixtures"))]
#[doc(hidden)]
pub fn encode_htj2k_high_component_multiple_set_test_fixture() -> Result<Vec<u8>> {
    fixture(1, 1, 257, 8, false, 256, 3, 128, true)
}

/// Project-authored selected entropy failure after complete packet admission.
#[cfg(all(feature = "std", any(test, feature = "test-fixtures")))]
#[doc(hidden)]
pub fn encode_htj2k_high_component_entropy_failure_test_fixture() -> Result<Vec<u8>> {
    let mut bytes = encode_htj2k_high_component_test_fixture(1, 1, 257)?;
    let p = prepare_htj2k_high_component_decode(&bytes)?.ok_or(CodestreamError::SizeOverflow)?;
    let first = p
        .contributions
        .first()
        .ok_or(CodestreamError::SizeOverflow)?;
    let end = p.codestream.tiles[0]
        .payload_offset
        .ok_or(CodestreamError::SizeOverflow)?
        + first.payload_offset
        + first.codeword_len;
    bytes[end - 2..end].copy_from_slice(&[0xff, 0xff]);
    Ok(bytes)
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    width: u32,
    height: u32,
    components: u16,
    bits: u8,
    signed: bool,
    roi: u16,
    shift: u8,
    split: u16,
    multiple: bool,
) -> Result<Vec<u8>> {
    let layers = if multiple { 2 } else { 1 };
    let mut output = Vec::new();
    write_native_main_header(
        &mut output,
        width,
        height,
        width,
        height,
        bits,
        components,
        true,
        1,
        &[8, 9, 10, 11],
        true,
        0,
        layers,
    )?;
    let siz = find_marker(&output, 0, Marker::Siz).ok_or(CodestreamError::SizeOverflow)?;
    for c in 0..usize::from(components) {
        output[siz + 40 + c * 3] = (bits - 1) | if signed { 0x80 } else { 0 };
    }
    let cap = find_marker(&output, 0, Marker::Cap).ok_or(CodestreamError::SizeOverflow)?;
    output[cap + 8..cap + 10].copy_from_slice(&0x100a_u16.to_be_bytes());
    let cod = find_marker(&output, 0, Marker::Cod).ok_or(CodestreamError::SizeOverflow)?;
    output[cod + 2..cod + 4].copy_from_slice(&14_u16.to_be_bytes());
    output[cod + 4] = 1;
    output[cod + 5] = 1;
    output[cod + 10] = 3;
    output[cod + 11] = 3;
    output.splice(cod + 14..cod + 14, [0x77, 0x88]);
    let selector = |c: u16| -> Vec<u8> {
        if components < 257 {
            alloc::vec![c as u8]
        } else {
            c.to_be_bytes().to_vec()
        }
    };
    let marker = |out: &mut Vec<u8>, code: u8, data: &[u8]| {
        out.extend_from_slice(&[0xff, code]);
        out.extend_from_slice(&(data.len() as u16 + 2).to_be_bytes());
        out.extend_from_slice(data);
    };
    let mut rgn = selector(roi);
    rgn.extend_from_slice(&[0, shift]);
    marker(&mut output, 0x5e, &rgn);
    let mut poc = Vec::new();
    for (start, end, order) in [(0, split, 1), (split, components, 4)] {
        poc.push(0);
        poc.extend(selector(start));
        poc.extend_from_slice(&[0, layers as u8, 33]);
        poc.extend(selector(end));
        poc.push(order);
    }
    marker(&mut output, 0x5f, &poc);
    let mut segments = Vec::new();
    let mut bands = Vec::new();
    for c in 0..components {
        let block = if c % 2 == 0 { 32 } else { 64 };
        let mut coc = selector(c);
        coc.extend_from_slice(&[
            1,
            1,
            if block == 32 { 3 } else { 4 },
            if block == 32 { 3 } else { 4 },
            0x40,
            1,
            0x77,
            0x88,
        ]);
        marker(&mut output, 0x53, &coc);
        let guard = 1 + (c % 3) as u8;
        let mut qcc = selector(c);
        qcc.push(guard << 5);
        for exponent in [8, 9, 10, 11] {
            qcc.push(exponent << 3);
        }
        marker(&mut output, 0x5d, &qcc);
        let mut plane = alloc::vec![0_i32; checked_component_sample_count(width,height)?];
        let specs = decomp_subband_specs(width, height, 1)?;
        for spec in &specs {
            for y in 0..spec.height {
                for x in 0..spec.width {
                    let n = x + 3 * y + u32::from(spec.index) + u32::from(c);
                    let v = (n % 9) as i32 - 4;
                    let v = if v == 0 { 1 } else { v };
                    plane[((spec.y + y) * width + spec.x + x) as usize] = if c == roi && n % 2 == 0
                    {
                        v * (1_i32 << shift)
                    } else {
                        v
                    };
                }
            }
        }
        let mut component = Vec::new();
        for spec in specs {
            component.push(encode_ht_decomp_subband_with_block_size(
                width,
                &plane,
                spec,
                8 + spec.index + guard - 1 + if c == roi { shift } else { 0 },
                &mut segments,
                block,
            )?);
        }
        bands.push(component);
    }
    // Independently enumerate the two volumes; never call the production
    // progression planner when constructing this oracle's packet order.
    let schedule = (0..=1)
        .flat_map(|r| (0..layers).flat_map(move |l| (0..split).map(move |c| (c, r, l))))
        .chain(
            (split..components)
                .flat_map(|c| (0..=1).flat_map(move |r| (0..layers).map(move |l| (c, r, l)))),
        );
    let mut packets = Vec::new();
    for (c, r, l) in schedule {
        if l != 0 {
            let mut writer = PacketBitWriter::new();
            if c == 0 && r == 0 {
                writer.write_bit(1)?;
                writer.write_bit(1)?;
                write_coding_pass_count(&mut writer, 3)?;
                writer.write_bit(0)?;
                let len = bands[0][0].code_blocks[0].segment_len;
                let lblock = (usize::BITS - len.leading_zeros()).max(3) as u8;
                writer.write_bits(0, lblock + 1)?;
            } else {
                writer.write_bit(0)?;
            }
            writer.align();
            packets.extend_from_slice(writer.bytes());
            continue;
        }
        let active = bands[usize::from(c)]
            .iter()
            .filter(|b| b.resolution == r && !b.code_blocks.is_empty())
            .collect::<Vec<_>>();
        let mut writer = PacketBitWriter::new();
        writer.write_bit(1)?;
        for b in &active {
            write_component_packet_header(
                &mut writer,
                b.code_block_cols,
                b.code_block_rows,
                &b.code_blocks,
            )?;
        }
        writer.align();
        packets.extend_from_slice(writer.bytes());
        for b in active {
            for block in b.code_blocks.iter().filter(|b| b.included) {
                packets.extend_from_slice(checked_slice(
                    &segments,
                    block.segment_offset,
                    block.segment_len,
                )?);
            }
        }
    }
    write_tile_part(&mut output, 0, &packets, true)?;
    Ok(output)
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn prepared(bytes: &[u8]) -> PreparedHtj2kReducedComponentDecode<'_> {
        prepare_htj2k_high_component_decode(bytes).unwrap().unwrap()
    }

    fn offset(bytes: &[u8], marker: Marker, last: bool) -> usize {
        let c = parse(bytes).unwrap();
        let mut matches = c.markers.iter().filter(|m| m.marker == marker);
        if last {
            matches.next_back().unwrap().data_offset
        } else {
            matches.next().unwrap().data_offset
        }
    }

    #[test]
    fn high_component_checks_discarded_state_and_preflights_resources() {
        let bytes = encode_htj2k_high_component_test_fixture(3, 5, 257).unwrap();
        let p = prepared(&bytes);
        let qcc = offset(&bytes, Marker::Qcc, true);
        let rgn = offset(&bytes, Marker::Rgn, false);
        let coc = offset(&bytes, Marker::Coc, true);
        let poc = offset(&bytes, Marker::Poc, false);
        let qcd = offset(&bytes, Marker::Qcd, false);
        for (at, value) in [
            (qcc + 2, 3),
            (qcc + 3, 0),
            (qcc + 3, 31 << 3),
            (rgn + 2, 1),
            (rgn + 3, 38),
            (coc + 2, 2),
            (poc + 8, 0),
            (poc + 15, 0),
            (qcd, 3),
        ] {
            let mut bad = bytes.clone();
            bad[at] = value;
            assert!(
                prepare_htj2k_high_component_decode(&bad).is_err(),
                "at={at} value={value}"
            );
        }
        // Syntactically legal unsupported effective transforms stay closed.
        let mut irreversible = bytes.clone();
        irreversible[coc + 7] = 0;
        let cap = offset(&bytes, Marker::Cap, false);
        irreversible[cap + 4..cap + 6].copy_from_slice(&0x182a_u16.to_be_bytes());
        assert!(
            prepare_htj2k_high_component_decode(&irreversible)
                .unwrap()
                .is_none()
        );
        // The final unselected packet must be parsed even after zero is ready.
        let mut late = bytes.clone();
        let end = p.codestream.tiles[0].payload_offset.unwrap()
            + p.codestream.tiles[0].payload_len.unwrap();
        late.remove(end - 1);
        let sot = offset(&bytes, Marker::Sot, false);
        let length = read_u32(&bytes, sot + 2).unwrap() - 1;
        late[sot + 2..sot + 6].copy_from_slice(&length.to_be_bytes());
        assert!(prepare_htj2k_high_component_decode(&late).is_err());
        for size in [65, 32768, u32::MAX] {
            let mut big = bytes.clone();
            let siz = offset(&bytes, Marker::Siz, false);
            big[siz + 2..siz + 6].copy_from_slice(&size.to_be_bytes());
            big[siz + 18..siz + 22].copy_from_slice(&size.to_be_bytes());
            STRUCTURAL_CALLS.with(|n| n.set(0));
            assert!(prepare_htj2k_high_component_decode(&big).unwrap().is_none());
            assert_eq!(STRUCTURAL_CALLS.with(|n| n.get()), 0);
        }
        // One pass over marker headers; no component-by-marker scanning here.
        for extra in [0, 257, 4096] {
            let mut c = p.codestream.clone();
            let com = MarkerSegment {
                marker: Marker::Com,
                offset: 0,
                data_offset: 0,
                data_len: 0,
            };
            c.markers.extend(core::iter::repeat_n(com, extra));
            let mut visits = 0;
            assert!(markers_supported(c.markers.iter().inspect(|_| visits += 1)));
            assert_eq!(visits, c.markers.len());
        }
    }

    #[test]
    fn high_component_structural_contradictions_precede_native_declines() {
        let bytes = encode_htj2k_high_component_multiple_set_test_fixture().unwrap();
        let cap = offset(&bytes, Marker::Cap, false);
        let rgn = offset(&bytes, Marker::Rgn, false);
        for (ccap, shift, roi) in [
            (0x100a_u16, 3, 256_u16),
            (0x100b, 3, 256),
            (0x100a, 16, 256),
            (0x100a, 3, 0),
        ] {
            let mut bad = bytes.clone();
            bad[cap + 4..cap + 6].copy_from_slice(&ccap.to_be_bytes());
            bad[rgn..rgn + 2].copy_from_slice(&roi.to_be_bytes());
            bad[rgn + 3] = shift;
            assert!(matches!(
                prepare_htj2k_high_component_decode(&bad),
                Err(CodestreamError::InvalidMarker {
                    marker: Some(Marker::Cap),
                    ..
                })
            ));
            assert!(matches!(
                validate_part15_packet_signalling(&bad, &parse(&bad).unwrap()),
                Err(CodestreamError::InvalidMarker {
                    marker: Some(Marker::Cap),
                    ..
                })
            ));
        }
        let mut permitted = bytes.clone();
        permitted[cap + 4..cap + 6].copy_from_slice(&0x300a_u16.to_be_bytes());
        validate_part15_packet_signalling(&permitted, &parse(&permitted).unwrap()).unwrap();
        assert!(matches!(
            prepare_htj2k_high_component_decode(&permitted),
            Err(CodestreamError::Unsupported { .. })
        ));
    }

    #[test]
    fn high_component_index_widths_progression_and_native_output() {
        for (count, split, roi, shift, width, height, bits, signed) in [
            (4, 2, 3, 1, 17, 29, 8, false),
            (255, 127, 254, 3, 1, 1, 12, true),
            (256, 255, 255, 11, 3, 5, 16, false),
            (257, 256, 256, 15, 7, 3, 16, true),
            (257, 128, 3, 11, 1, 1, 8, false),
            (9, 4, 8, 7, 64, 64, 8, true),
        ] {
            let bytes =
                fixture(width, height, count, bits, signed, roi, shift, split, false).unwrap();
            let p = prepared(&bytes);
            assert_eq!(
                component_selector_len(&p.codestream.siz),
                if count < 257 { 1 } else { 2 }
            );
            let packets = parse_default_precinct_packets_from_source_with_ht_retention(
                &bytes,
                &p.codestream,
                p.tile_rect,
                &ContiguousPacketSource {
                    bytes: single_part1_profile_tile(&bytes, &p.codestream).unwrap().1,
                },
                None,
                None,
                None,
                PacketOrganisationConfig::HT_HIGH_COMPONENT,
                None,
                HtCodingSetRetention::NativeAdmission,
            )
            .unwrap();
            assert_eq!(packets.packet_count, u64::from(count) * 2);
            assert!(
                packets
                    .contributions
                    .iter()
                    .any(|c| c.component_index == count - 1)
            );
            assert!(p.contributions.iter().all(|c| c.component_index == 0));
            for contribution in &packets.contributions {
                let component = contribution.component_index;
                assert_eq!(
                    contribution.available_bitplanes,
                    // Reversible HT packets retain exponent+1; guard bits
                    // are resolved later by the selected transfer.
                    9 + contribution.subband_index + if component == roi { shift } else { 0 }
                );
            }
            let mut expected = vec![0_i32; (width * height) as usize];
            for s in decomp_subband_specs(width, height, 1).unwrap() {
                for y in 0..s.height {
                    for x in 0..s.width {
                        let v = ((x + 3 * y + u32::from(s.index)) % 9) as i32 - 4;
                        expected[((s.y + y) * width + s.x + x) as usize] =
                            if v == 0 { 1 } else { v };
                    }
                }
            }
            inverse_reversible_5_3_levels_with_scratch(
                &mut expected,
                width as usize,
                width,
                height,
                1,
                &mut vec![0; width.max(height) as usize * 3],
            )
            .unwrap();
            let expected = component_sample_slice_to_bytes(bits, signed, &expected).unwrap();
            let decoded = decode_prepared_htj2k_reduced_component_owned_with_workspace(
                &p,
                &mut HtCodestreamDecodeWorkspace::new(),
            )
            .unwrap();
            assert_eq!(decoded.components.len(), 1);
            assert_eq!(
                decoded.components[0].samples, expected,
                "count={count} roi={roi} shift={shift}"
            );
        }
    }

    #[test]
    fn high_component_discarded_entropy_and_formats_do_not_leak_into_output() {
        let bytes = fixture(3, 5, 257, 8, false, 256, 11, 128, false).unwrap();
        let p = prepared(&bytes);
        let expected = decode_prepared_htj2k_reduced_component_owned_with_workspace(
            &p,
            &mut HtCodestreamDecodeWorkspace::new(),
        )
        .unwrap();
        let payload = single_part1_profile_tile(&bytes, &p.codestream).unwrap().1;
        let packets = parse_default_precinct_packets_from_source_with_ht_retention(
            &bytes,
            &p.codestream,
            p.tile_rect,
            &ContiguousPacketSource { bytes: payload },
            None,
            None,
            None,
            PacketOrganisationConfig::HT_HIGH_COMPONENT,
            None,
            HtCodingSetRetention::NativeAdmission,
        )
        .unwrap();
        let last = packets
            .contributions
            .iter()
            .rev()
            .find(|c| c.component_index == 256)
            .unwrap();
        let end =
            p.codestream.tiles[0].payload_offset.unwrap() + last.payload_offset + last.codeword_len;
        let mut changed = bytes.clone();
        changed[end - 2..end].copy_from_slice(&[0xff, 0xff]);
        let siz = offset(&bytes, Marker::Siz, false);
        changed[siz + 36 + 256 * 3] = 0x8f; // Unselected signed 16-bit format.
        let changed = prepared(&changed);
        let actual = decode_prepared_htj2k_reduced_component_owned_with_workspace(
            &changed,
            &mut HtCodestreamDecodeWorkspace::new(),
        )
        .unwrap();
        assert_eq!(actual, expected);
        let mut inherited = bytes.clone();
        for m in p
            .codestream
            .markers
            .iter()
            .rev()
            .filter(|m| matches!(m.marker, Marker::Coc | Marker::Qcc))
        {
            let component = read_u16(&bytes, m.data_offset).unwrap();
            if matches!(component, 0 | 1 | 255) {
                inherited.drain(m.offset..m.data_offset + m.data_len);
            }
        }
        let inherited = prepared(&inherited);
        assert_eq!(
            decode_prepared_htj2k_reduced_component_owned_with_workspace(
                &inherited,
                &mut HtCodestreamDecodeWorkspace::new()
            )
            .unwrap(),
            expected
        );
        let oversized = fixture(1, 1, 258, 8, false, 257, 3, 128, false).unwrap();
        STRUCTURAL_CALLS.with(|n| n.set(0));
        assert!(
            prepare_htj2k_high_component_decode(&oversized)
                .unwrap()
                .is_none()
        );
        assert_eq!(STRUCTURAL_CALLS.with(|n| n.get()), 0);
        // Packet magnitude accounting uses the ROI-extended quantiser, not
        // sample precision. The existing sign/threshold restoration is not
        // applied to selected zero by this independently admitted route.
        for shift in [1, 3, 11, 15] {
            let t = 1_i32 << shift;
            let mut values = [-t - 1, -t, -t + 1, -1, 0, 1, t - 1, t, t + 1];
            realign_bounded_maxshift_coefficients(&mut values, shift).unwrap();
            assert_eq!(values, [-1, -1, -t + 1, -1, 0, 1, t - 1, 1, 1]);
        }
    }
}
