//! Independently admitted HTONLY sampled-RPCL reduced ROI reconstruction.
//! Project-authored from Part 1:2024 A.6.3, B.12.1.3, E.1 and H.1
//! (pages 51–52, 97, 129–130, 156), retrieval
//! 34e5d1639b9f121807e620c001893ca9d2c8f977; Part 15:2019 A.5 (page 38),
//! retrieval 10baf9472429d52f5d6b5f9b7a892dbed395b1db.

use super::*;

#[cfg(test)]
std::thread_local! {
    static STRUCTURAL_CALLS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

fn outside(detail: &'static str) -> CodestreamError {
    unsupported(None, None, UnsupportedConstruct::HtBlockDecode, detail)
}

pub(super) fn envelope(c: &Codestream) -> bool {
    c.kind == CodestreamKind::Htj2k
        && c.siz.components.len() == 4
        && c.siz.components.iter().enumerate().all(|(i, component)| {
            (8..=16).contains(&component.bits_per_sample)
                && component.horizontal_separation == 1 + (i as u8 & 1)
                && component.vertical_separation == 1 + (i as u8 >> 1)
        })
        && c.siz.image_origin_x == 0
        && c.siz.image_origin_y == 0
        && c.siz.tile_origin_x == 0
        && c.siz.tile_origin_y == 0
        && c.siz.tile_count_x() == Ok(1)
        && c.siz.tile_count_y() == Ok(1)
        && matches!(c.tiles.as_slice(), [part] if part.tile_index == 0
            && part.tile_part_index == 0 && part.tile_part_count == Some(1))
        && u64::from(c.image_width()) * u64::from(c.image_height())
            <= MAX_NATIVE_PART1_PROFILE_COMPONENT_SAMPLES
        && (0..4).all(|i| {
            c.effective_coding_style(i).is_some_and(|s| {
                s.entropy_coder == EntropyCoder::HtBlockCoding
                    && s.code_block_style == 0x40
                    && s.decomposition_levels == 6
                    && s.transform
                        == if i == 3 {
                            WaveletTransform::Reversible53
                        } else {
                            WaveletTransform::Irreversible97
                        }
                    && !s.multiple_component_transform
                    && s.progression_order == ProgressionOrder::Rpcl
                    && (1..=4).contains(&s.layers)
                    && s.code_block_width_exponent == 6
                    && s.code_block_height_exponent == 6
                    && !s.sop_markers
                    && !s.eph_markers
                    && s.precincts_declared
                    && s.precinct_exponents[..7]
                        .iter()
                        .all(|&p| matches!(p, 0x77 | 0x88))
            })
        })
        && markers(c.markers.iter())
}

fn markers<'a>(segments: impl Iterator<Item = &'a MarkerSegment>) -> bool {
    let mut tile = false;
    let mut qcd = 0;
    let mut main_rgn = 0;
    let mut tile_rgn = 0;
    for segment in segments {
        match segment.marker {
            Marker::Sot => tile = true,
            Marker::Cod | Marker::Coc | Marker::Qcc if !tile => {}
            Marker::Qcd if !tile => qcd += 1,
            Marker::Rgn if tile => tile_rgn += 1,
            Marker::Rgn => main_rgn += 1,
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
    tile && qcd == 1 && main_rgn == 1 && tile_rgn == 1
}

pub(super) fn resolve_maxshift(input: &[u8], c: &Codestream) -> Result<BoundedTileMaxshift> {
    if !envelope(c) {
        return Err(outside(
            "HT reduced ROI requires its bounded sampled-RPCL resource envelope",
        ));
    }
    let mut effective = None;
    for marker in c.markers.iter().filter(|m| m.marker == Marker::Rgn) {
        let rgn = parse_rgn_declaration(input, marker, &c.siz)?;
        if rgn.component_index != 0 || rgn.style != 0 || rgn.shift > 37 {
            return Err(outside(
                "HT reduced ROI requires legal component-zero main and tile Maxshift",
            ));
        }
        // The checked one-part envelope orders the sole tile assignment after
        // the sole main assignment. The latter remains validated, not applied.
        effective = Some(BoundedTileMaxshift {
            tile_index: 0,
            component_index: 0,
            shift: rgn.shift,
        });
    }
    effective.ok_or(CodestreamError::SizeOverflow)
}

#[cfg(feature = "std")]
pub(super) fn prepare(
    input: &[u8],
    c: Codestream,
    request: Htj2kReducedComponentDecodeRequest,
) -> Result<PreparedHtj2kReducedComponentDecode<'_>> {
    let roi = resolve_maxshift(input, &c)?;
    // Preflight above bounds geometry, components, levels and packet state.
    // Inside that envelope, signalling contradictions precede native declines.
    #[cfg(test)]
    STRUCTURAL_CALLS.with(|calls| calls.set(calls.get() + 1));
    validate_part15_packet_signalling(input, &c)?;
    for marker in c.markers.iter().filter(|m| m.marker == Marker::Rgn) {
        if !(1..=15).contains(&parse_rgn_declaration(input, marker, &c.siz)?.shift) {
            return Err(outside(
                "HT reduced ROI requires main and tile shifts of one through fifteen",
            ));
        }
    }
    if roi.shift > 9 {
        return Err(outside(
            "HT reduced ROI admits effective Maxshift of one through nine",
        ));
    }
    let p = c
        .capability
        .as_ref()
        .and_then(|cap| cap.part15)
        .ok_or(CodestreamError::SizeOverflow)?;
    if p.code_block_mode != Part15CodeBlockMode::HtOnly || p.cleanup_magnitude_bound > 18 {
        return Err(outside(
            "HT reduced ROI requires HTONLY with cleanup magnitude bound at most eighteen",
        ));
    }
    if request.component_index != 0 || request.discard_levels != 3 {
        return Err(outside(
            "HT reduced ROI selects component zero at reduction three",
        ));
    }
    let config = PacketOrganisationConfig::HT_REDUCED_ROI;
    let styles = packet_component_styles(&c, config)?;
    let q = parse_component_quantization_for_styles(input, &c, &styles, &[19; 4], 19, None)?;
    for (i, quantizer) in q.iter().enumerate() {
        let expected = if i == 3 {
            transform::QuantizationStyle::NoQuantization
        } else {
            transform::QuantizationStyle::ScalarExpounded
        };
        if quantizer.style != expected
            || quantizer.steps.iter().any(|step| {
                step.exponent == 0
                    || step
                        .exponent
                        .checked_add(quantizer.guard_bits)
                        .and_then(|n| n.checked_sub(1))
                        .and_then(|n| n.checked_add(if i == 0 { roi.shift } else { 0 }))
                        .is_none_or(|n| n > 30)
            })
        {
            return Err(outside(
                "HT reduced ROI requires bounded effective scalar-expounded and reversible quantisers",
            ));
        }
    }
    let (tile_rect, payload) = single_part1_profile_tile(input, &c)?;
    let coding_style = styles[0];
    let (retained, output_width, output_height, _) =
        htj2k_reduced_component_output_geometry(tile_rect.width, tile_rect.height, 6, request)?;
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
    classify_htonly_native_packet_mechanisms(&contributions)
        .map_err(|d| unsupported(None, None, d.construct, d.detail))?;
    let mut state = ht_marker_state(&c).ok_or(CodestreamError::SizeOverflow)?;
    state.components = 1;
    state.all_components_same_sample_format = true;
    state.all_components_unit_sampled = true;
    state.packet_progression_supported = true;
    state.reversible_transform = true;
    state.precincts_declared = false;
    state.code_block_width =
        ht_code_block_dimension_from_exponent(coding_style.code_block_width_exponent);
    state.code_block_height =
        ht_code_block_dimension_from_exponent(coding_style.code_block_height_exponent);
    let marker_candidate =
        ht::plan_decode_candidate(state).map_err(|d| outside(d.reason.message()))?;
    let candidate = HtCodestreamDecodeCandidate {
        marker_candidate,
        tile_part: c.tiles[0],
    };
    Ok(PreparedHtj2kReducedComponentDecode {
        input,
        codestream: c,
        candidate,
        coding_style,
        reconstruction: Htj2kReducedComponentReconstruction::IrreversibleRoi(roi.shift),
        tile_rect,
        contributions: contributions
            .into_iter()
            .filter(|p| p.component_index == 0 && p.resolution <= retained)
            .collect(),
        request,
        output_width,
        output_height,
    })
}

#[cfg(feature = "std")]
pub(super) fn doubled_coefficient(coefficient: i32, available: u8, shift: u8) -> Result<f32> {
    if !(1..=15).contains(&shift) || !(shift + 1..=30).contains(&available) {
        return Err(CodestreamError::SizeOverflow);
    }
    let alignment = 30 - available;
    let magnitude = coefficient.unsigned_abs();
    let threshold = 1_u32 << (alignment + shift + 1);
    if magnitude < threshold {
        return ht_irreversible_doubled_half_step_coefficient(coefficient, available);
    }
    // Restore the doubled coefficient before floating-point conversion. ROI
    // uses zero reconstruction bias; background retains the HT half-step.
    let restored = (magnitude >> (alignment + shift)) as f32;
    Ok(if coefficient < 0 { -restored } else { restored })
}

/// Project-authored coefficient-domain fixture, not an application encoder.
#[doc(hidden)]
pub fn encode_htj2k_reduced_roi_test_fixture(width: u32, height: u32) -> Result<Vec<u8>> {
    fixture(width, height, 12, false, 9, 4, 0x88)
}

/// Project-authored later empty HT-set announcement for signalling tests.
#[cfg(feature = "std")]
#[doc(hidden)]
pub fn encode_htj2k_reduced_roi_multiple_set_test_fixture() -> Result<Vec<u8>> {
    let input = encode_htj2k_reduced_roi_test_fixture(65, 97)?;
    let p = prepare_htj2k_reduced_component_decode(
        &input,
        Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 3,
        },
    )?
    .ok_or(CodestreamError::SizeOverflow)?;
    let first = &p.contributions[0];
    let after = p.codestream.tiles[0]
        .payload_offset
        .ok_or(CodestreamError::SizeOverflow)?
        + first.payload_offset
        + first.codeword_len;
    let mut writer = PacketBitWriter::new();
    writer.write_bit(1)?;
    writer.write_bit(1)?;
    write_coding_pass_count(&mut writer, 3)?;
    writer.write_bit(0)?;
    let lblock = (usize::BITS - first.codeword_len.leading_zeros()).max(3) as u8;
    writer.write_bits(0, lblock + 1)?;
    writer.align();
    let sot = find_marker(&input, 0, Marker::Sot).ok_or(CodestreamError::SizeOverflow)?;
    let length = read_u32(&input, sot + 6)? + writer.bytes().len() as u32 - 1;
    let mut multiple = input.clone();
    multiple.splice(after..after + 1, writer.bytes().iter().copied());
    multiple[sot + 6..sot + 10].copy_from_slice(&length.to_be_bytes());
    Ok(multiple)
}

fn fixture(
    width: u32,
    height: u32,
    bits: u8,
    signed: bool,
    shift: u8,
    layers: u16,
    high: u8,
) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    write_native_main_header(
        &mut output,
        width,
        height,
        width,
        height,
        bits,
        4,
        false,
        6,
        &[8; 19],
        true,
        0,
        1,
    )?;
    let siz = find_marker(&output, 0, Marker::Siz).ok_or(CodestreamError::SizeOverflow)?;
    for i in 0..4 {
        output[siz + 40 + i * 3] = (bits - 1) | if signed { 0x80 } else { 0 };
        output[siz + 41 + i * 3] = 1 + (i as u8 & 1);
        output[siz + 42 + i * 3] = 1 + (i as u8 >> 1);
    }
    let cap = find_marker(&output, 0, Marker::Cap).ok_or(CodestreamError::SizeOverflow)?;
    output[cap + 8..cap + 10].copy_from_slice(&0x182a_u16.to_be_bytes());
    let cod = find_marker(&output, 0, Marker::Cod).ok_or(CodestreamError::SizeOverflow)?;
    output[cod + 4] = 1;
    output[cod + 5] = 2;
    output[cod + 6..cod + 8].copy_from_slice(&layers.to_be_bytes());
    output[cod + 13] = 0;
    output[cod + 2..cod + 4].copy_from_slice(&19_u16.to_be_bytes());
    let qcd = find_marker(&output, 0, Marker::Qcd).ok_or(CodestreamError::SizeOverflow)?;
    output.truncate(qcd);
    output.extend_from_slice(&[0x77, high, high, high, high, high, high]);
    output.extend_from_slice(&[0xff, 0x5c, 0, 41, 0x42]);
    for index in 0..19_u16 {
        output.extend_from_slice(&(((6 + index % 3) << 11) | (256 + 16 * index)).to_be_bytes());
    }
    output.extend_from_slice(&[
        0xff,
        0x5e,
        0,
        5,
        0,
        0,
        if shift == 15 { 14 } else { shift + 1 },
    ]);
    let mut segments = Vec::new();
    let mut bands = Vec::new();
    let mut schedule = BTreeMap::new();
    for i in 0..4_u8 {
        let sx = 1 + (i & 1);
        let sy = 1 + (i >> 1);
        let cw = width.div_ceil(u32::from(sx));
        let ch = height.div_ceil(u32::from(sy));
        let precincts = [0x77, high, high, high, high, high, high];
        let mut coc = alloc::vec![i, 1, 6, 4, 4, 0x40, u8::from(i == 3)];
        coc.extend_from_slice(&precincts);
        output.extend_from_slice(&[0xff, 0x53]);
        output.extend_from_slice(&(coc.len() as u16 + 2).to_be_bytes());
        output.extend_from_slice(&coc);
        if i != 0 {
            let mut qcc = alloc::vec![i, ((i + 2) << 5) | if i == 3 { 0 } else { 2 }];
            for index in 0..19_u16 {
                let exponent = 6 + index % 3;
                if i == 3 {
                    qcc.push((exponent as u8) << 3);
                } else {
                    qcc.extend_from_slice(&((exponent << 11) | (256 + 16 * index)).to_be_bytes());
                }
            }
            output.extend_from_slice(&[0xff, 0x5d]);
            output.extend_from_slice(&(qcc.len() as u16 + 2).to_be_bytes());
            output.extend_from_slice(&qcc);
        }
        let specs = decomp_subband_specs(cw, ch, 6)?;
        let mut plane = alloc::vec![0_i32;checked_component_sample_count(cw,ch)?];
        for spec in &specs {
            for y in 0..spec.height {
                for x in 0..spec.width {
                    let n = x + 3 * y + 5 * u32::from(spec.index) + u32::from(i);
                    let v = (n % 15) as i32 - 7;
                    plane[((spec.y + y) * cw + spec.x + x) as usize] = if i == 0 {
                        if n % 3 == 0 {
                            v * (1_i32 << shift)
                        } else {
                            v.signum()
                        }
                    } else {
                        v
                    };
                }
            }
        }
        let mut component_bands = Vec::new();
        for spec in specs {
            let available = 6 + spec.index % 3 + i + 1 + if i == 0 { shift } else { 0 };
            component_bands.push(encode_ht_decomp_subband_with_block_size(
                cw,
                &plane,
                spec,
                available,
                &mut segments,
                64,
            )?);
        }
        bands.push(component_bands);
        for r in 0..=6_u8 {
            let side = 1_u32 << (precincts[usize::from(r)] & 15);
            let (rw, rh) = resolution_dimensions(cw, ch, 6, r)?;
            for py in 0..rh.div_ceil(side) {
                for px in 0..rw.div_ceil(side) {
                    // Actual precinct origins in the reference grid, sorted
                    // by resolution then position then component: RPCL.
                    schedule.insert(
                        (
                            r,
                            py * side * (u32::from(sy) << (6 - r)),
                            px * side * (u32::from(sx) << (6 - r)),
                            i,
                        ),
                        (px, py, side),
                    );
                }
            }
        }
    }
    let mut packets = Vec::new();
    for ((r, _, _, i), (px, py, side)) in schedule {
        let axis = (if r == 0 { side } else { side / 2 }) / 64;
        let mut precinct = Vec::new();
        for band in bands[usize::from(i)].iter().filter(|b| b.resolution == r) {
            let mut part = band.clone();
            part.code_blocks
                .retain(|b| u32::from(b.x) / axis == px && u32::from(b.y) / axis == py);
            if part.code_blocks.is_empty() {
                continue;
            }
            for b in &mut part.code_blocks {
                b.x -= (px * axis) as u16;
                b.y -= (py * axis) as u16;
            }
            part.code_block_cols = part.code_blocks.iter().map(|b| b.x).max().unwrap() + 1;
            part.code_block_rows = part.code_blocks.iter().map(|b| b.y).max().unwrap() + 1;
            precinct.push(part);
        }
        for layer in 0..layers {
            let active = layer == (u16::from(i) + u16::from(r)) % layers;
            let mut writer = PacketBitWriter::new();
            writer.write_bit(u32::from(active))?;
            if active {
                for p in &precinct {
                    write_component_packet_header(
                        &mut writer,
                        p.code_block_cols,
                        p.code_block_rows,
                        &p.code_blocks,
                    )?;
                }
            }
            writer.align();
            packets.extend_from_slice(writer.bytes());
            if active {
                for p in &precinct {
                    for b in p.code_blocks.iter().filter(|b| b.included) {
                        packets.extend_from_slice(checked_slice(
                            &segments,
                            b.segment_offset,
                            b.segment_len,
                        )?);
                    }
                }
            }
        }
    }
    let start = output.len();
    write_tile_part(&mut output, 0, &packets, true)?;
    let length = read_u32(&output, start + 6)?;
    output[start + 6..start + 10].copy_from_slice(&(length + 7).to_be_bytes());
    output.splice(start + 12..start + 12, [0xff, 0x5e, 0, 5, 0, 0, shift]);
    Ok(output)
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn request() -> Htj2kReducedComponentDecodeRequest {
        Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 3,
        }
    }

    #[test]
    fn reduced_roi_signed_thresholds_preserve_subband_alignment() {
        for shift in [1, 3, 9, 15] {
            for base in [1, 7, 15] {
                let available = base + shift;
                let alignment = 30 - available;
                let threshold = 1_i32 << (shift + 1);
                for doubled in [
                    0,
                    1,
                    2,
                    3,
                    threshold - 1,
                    threshold,
                    threshold + 1,
                    threshold * 3 + 1,
                ] {
                    for sign in [-1, 1] {
                        let raw = i64::from(doubled * sign) << alignment;
                        let Ok(raw) = i32::try_from(raw) else {
                            continue;
                        };
                        let expected = if doubled < threshold {
                            doubled as f32
                        } else {
                            (doubled >> shift) as f32
                        };
                        assert_eq!(
                            doubled_coefficient(raw, available, shift).unwrap(),
                            expected * sign as f32
                        );
                    }
                }
            }
        }
        for (available, shift) in [(31, 9), (9, 9), (0, 1), (18, 0), (30, 16)] {
            assert!(doubled_coefficient(1, available, shift).is_err());
        }
    }

    #[test]
    fn reduced_roi_rpcl_matches_independent_coefficient_and_geometry_oracles() {
        for (width, height, layers, precinct) in
            [(65, 97, 1, 0x77), (529, 401, 4, 0x88), (2057, 65, 3, 0x88)]
        {
            for (bits, signed, shift) in
                [(8, false, 1), (12, false, 9), (12, true, 3), (16, true, 9)]
            {
                let input = fixture(width, height, bits, signed, shift, layers, precinct).unwrap();
                let p = prepare_htj2k_reduced_component_decode(&input, request())
                    .unwrap()
                    .unwrap();
                let (ow, oh) = (width.div_ceil(8), height.div_ceil(8));
                assert_eq!(
                    (
                        p.output_width(),
                        p.output_height(),
                        p.bits_per_sample(),
                        p.signed()
                    ),
                    (ow, oh, bits, signed)
                );
                assert!(
                    p.contributions
                        .iter()
                        .all(|b| b.component_index == 0 && b.resolution <= 3)
                );
                for b in &p.contributions {
                    let index = match b.subband {
                        PacketSubbandKind::LowLow => 0,
                        PacketSubbandKind::HighLow => 3 * b.resolution - 2,
                        PacketSubbandKind::LowHigh => 3 * b.resolution - 1,
                        PacketSubbandKind::HighHigh => 3 * b.resolution,
                    };
                    assert_eq!(b.available_bitplanes, 7 + index % 3 + shift);
                    assert_eq!(
                        b.irreversible_quantization_step.unwrap().exponent,
                        6 + index % 3
                    );
                }
                let mut oracle = vec![0.0_f32; (ow * oh) as usize];
                for spec in decomp_subband_specs(ow, oh, 3).unwrap() {
                    let gain = match spec.kind {
                        PacketSubbandKind::LowLow => 1.0,
                        PacketSubbandKind::HighHigh => 4.0,
                        _ => 2.0,
                    };
                    let exponent = 6 + spec.index % 3;
                    let delta = (2.0_f32).powi(i32::from(bits) - i32::from(exponent))
                        * (1.0 + (256 + 16 * u32::from(spec.index)) as f32 / 2048.0)
                        * gain;
                    for y in 0..spec.height {
                        for x in 0..spec.width {
                            let n = x + 3 * y + 5 * u32::from(spec.index);
                            let v = (n % 15) as i32 - 7;
                            let q = if n % 3 == 0 {
                                v as f32
                            } else {
                                v.signum() as f32 * 1.5
                            };
                            oracle[((spec.y + y) * ow + spec.x + x) as usize] = q * delta;
                        }
                    }
                }
                inverse_irreversible_9_7_levels_with_scratch(
                    &mut oracle,
                    ow as usize,
                    ow,
                    oh,
                    3,
                    &mut vec![0.0; ow.max(oh) as usize * 2],
                )
                .unwrap();
                let mut expected = Vec::new();
                let level = if signed { 0 } else { 1_i32 << (bits - 1) };
                let low = if signed { -(1_i32 << (bits - 1)) } else { 0 };
                let high = (1_i32 << (bits - u8::from(signed))) - 1;
                for v in oracle {
                    let value = (v.round_ties_even() as i32 + level).clamp(low, high);
                    if bits <= 8 {
                        expected.push(value as u8);
                    } else {
                        expected.extend_from_slice(&(value as i16).to_le_bytes());
                    }
                }
                let decoded = decode_htj2k_reduced_component_owned(&input, request())
                    .unwrap_or_else(|error| {
                        panic!(
                            "{width}x{height} bits={bits} signed={signed} shift={shift}: {error:?}"
                        )
                    })
                    .unwrap();
                assert_eq!(
                    decoded.components[0].samples, expected,
                    "{width}x{height} bits={bits} signed={signed} shift={shift}"
                );
            }
        }
    }

    #[test]
    fn reduced_roi_quantiser_precedence_packet_counts_and_bounded_work() {
        let input = fixture(529, 145, 12, false, 9, 4, 0x88).unwrap();
        let c = parse(&input).unwrap();
        assert!(envelope(&c));
        assert_eq!(resolve_maxshift(&input, &c).unwrap().shift, 9);
        let (tile, payload) = single_part1_profile_tile(&input, &c).unwrap();
        let packets = parse_default_precinct_packets_from_source_with_ht_retention(
            &input,
            &c,
            tile,
            &ContiguousPacketSource { bytes: payload },
            None,
            None,
            None,
            PacketOrganisationConfig::HT_REDUCED_ROI,
            None,
            HtCodingSetRetention::NativeAdmission,
        )
        .unwrap();
        assert_eq!(packets.packet_count, 144);
        assert!((0..4).all(|i| packets.contributions.iter().any(|b| b.component_index == i)));
        let baseline = decode_htj2k_reduced_component_owned(&input, request())
            .unwrap()
            .unwrap();
        let rgn = c.markers.iter().find(|m| m.marker == Marker::Rgn).unwrap();
        let qcc = c.markers.iter().find(|m| m.marker == Marker::Qcc).unwrap();
        for offset in [rgn.offset + 6, qcc.offset + 7] {
            let mut changed = input.clone();
            changed[offset] ^= 1;
            assert_eq!(
                decode_htj2k_reduced_component_owned(&changed, request())
                    .unwrap()
                    .unwrap(),
                baseline
            );
        }
        let qcd = c.markers.iter().find(|m| m.marker == Marker::Qcd).unwrap();
        let mut changed = input.clone();
        changed[qcd.offset + 6] ^= 127;
        assert_ne!(
            decode_htj2k_reduced_component_owned(&changed, request())
                .unwrap()
                .unwrap(),
            baseline
        );
        let siz = c.markers.iter().find(|m| m.marker == Marker::Siz).unwrap();
        for dimension in [4097_u32, 32768, u32::MAX] {
            let mut huge = input.clone();
            for offset in [
                siz.offset + 6,
                siz.offset + 10,
                siz.offset + 22,
                siz.offset + 26,
            ] {
                huge[offset..offset + 4].copy_from_slice(&dimension.to_be_bytes());
            }
            STRUCTURAL_CALLS.with(|calls| calls.set(0));
            assert!(prepare_htj2k_reduced_component_decode(&huge, request()).is_err());
            STRUCTURAL_CALLS.with(|calls| assert_eq!(calls.get(), 0));
        }
        for copies in [1, 64, 1024] {
            let mut extended = c.markers.clone();
            let com = MarkerSegment {
                marker: Marker::Com,
                offset: 0,
                data_offset: 0,
                data_len: 0,
            };
            extended.splice(1..1, core::iter::repeat_n(com, copies));
            let visits = core::cell::Cell::new(0);
            assert!(markers(
                extended.iter().inspect(|_| visits.set(visits.get() + 1))
            ));
            assert_eq!(visits.get(), extended.len());
        }
    }

    #[test]
    fn reduced_roi_rejects_neighbouring_headers_and_late_unselected_packets() {
        let input = fixture(65, 97, 12, false, 9, 4, 0x88).unwrap();
        let c = parse(&input).unwrap();
        let at = |kind| c.markers.iter().find(|m| m.marker == kind).unwrap().offset;
        for (offset, value) in [
            (at(Marker::Rgn) + 4, 1),
            (at(Marker::Rgn) + 5, 1),
            (at(Marker::Rgn) + 6, 16),
            (at(Marker::Qcc) + 5, 0x63),
            (at(Marker::Qcc) + 6, 0),
            (at(Marker::Cod) + 5, 3),
            (at(Marker::Cod) + 9, 5),
            (at(Marker::Coc) + 7, 3),
            (at(Marker::Cod) + 4, 3),
            (at(Marker::Cap) + 8, 0x08),
            (at(Marker::Cap) + 9, 11),
            (at(Marker::Siz) + 44, 3),
        ] {
            let mut changed = input.clone();
            changed[offset] = value;
            assert!(
                prepare_htj2k_reduced_component_decode(&changed, request()).is_err(),
                "offset={offset} value={value}"
            );
        }
        let mut late = input.clone();
        let end = late.len() - 3;
        late[end] = 0xff;
        assert!(prepare_htj2k_reduced_component_decode(&late, request()).is_err());
        for component_index in [0, 1] {
            for discard_levels in [0, 2, 3, 4, 5] {
                if component_index == 0 && discard_levels == 3 {
                    continue;
                }
                assert!(
                    prepare_htj2k_reduced_component_decode(
                        &input,
                        Htj2kReducedComponentDecodeRequest {
                            component_index,
                            discard_levels
                        }
                    )
                    .is_err()
                );
            }
        }
    }

    #[test]
    fn reduced_roi_singleht_invalidity_precedes_native_declines() {
        let multiple = encode_htj2k_reduced_roi_multiple_set_test_fixture().unwrap();
        let cap = find_marker(&multiple, 0, Marker::Cap).unwrap();
        for bound in [0x2a, 0x2b] {
            for discard_levels in [2, 3, 5] {
                let mut input = multiple.clone();
                input[cap + 9] = bound;
                let c = parse(&input).unwrap();
                assert!(matches!(
                    validate_part15_packet_signalling(&input, &c),
                    Err(CodestreamError::InvalidMarker {
                        marker: Some(Marker::Cap),
                        ..
                    })
                ));
                assert!(matches!(
                    prepare_htj2k_reduced_component_decode(
                        &input,
                        Htj2kReducedComponentDecodeRequest {
                            component_index: 0,
                            discard_levels
                        }
                    ),
                    Err(CodestreamError::InvalidMarker {
                        marker: Some(Marker::Cap),
                        ..
                    })
                ));
            }
        }
        // An unused main shift is a native scope decline, not permission to
        // hide contradictory packets whose effective tile shift is unchanged.
        for main_shift in [0, 16, 37] {
            let mut input = multiple.clone();
            let main = find_marker(&input, 0, Marker::Rgn).unwrap();
            input[main + 6] = main_shift;
            assert!(matches!(
                prepare_htj2k_reduced_component_decode(&input, request()),
                Err(CodestreamError::InvalidMarker {
                    marker: Some(Marker::Cap),
                    ..
                })
            ));
        }
        let mut permitted = multiple;
        permitted[cap + 8] |= 0x20;
        assert!(validate_part15_packet_signalling(&permitted, &parse(&permitted).unwrap()).is_ok());
        assert!(
            matches!(prepare_htj2k_reduced_component_decode(&permitted,request()),Err(CodestreamError::Unsupported{message,..}) if message.contains("multiple effective HT coding sets"))
        );
    }
}
