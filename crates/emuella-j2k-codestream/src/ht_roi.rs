//! HT-owned native ROI admission and coefficient restoration.
//!
//! Project-authored from ISO/IEC 15444-1:2024 A.6.3, A.6.6, B.12.3 and H.1
//! (physical pages 51–52, 54–55, 99 and 156), retrieval
//! 34e5d1639b9f121807e620c001893ca9d2c8f977, and Part 15:2019 A.5 (page 38),
//! retrieval 10baf9472429d52f5d6b5f9b7a892dbed395b1db. No external payloads or
//! implementation material are used. This module does not grant Part 1 routes.

use super::*;

#[cfg(test)]
std::thread_local! {
    static STRUCTURAL_VALIDATION_CALLS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

pub(super) fn envelope(c: &Codestream) -> bool {
    c.kind == CodestreamKind::Htj2k
        && c.markers.iter().any(|m| m.marker == Marker::Rgn)
        && c.siz.components.len() == 1
        && c.siz.components.iter().all(|component| {
            (1..=16).contains(&component.bits_per_sample)
                && component.horizontal_separation == 1
                && component.vertical_separation == 1
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
            .is_some_and(|n| n <= 64)
        && c.coding_style.is_some_and(|style| {
            style.entropy_coder == EntropyCoder::HtBlockCoding
                && style.code_block_style == 0x40
                && style.decomposition_levels == 1
                && style.transform == WaveletTransform::Reversible53
                && !style.multiple_component_transform
                && (1..=8).contains(&style.layers)
                && style.progression_order == ProgressionOrder::Pcrl
                && matches!(style.code_block_width_exponent, 5 | 6)
                && matches!(style.code_block_height_exponent, 5 | 6)
                && style.precincts_declared
                && style.precinct_exponents[..2] == [0x77, 0x88]
        })
        && c.markers.iter().all(|m| {
            matches!(
                m.marker,
                Marker::Soc
                    | Marker::Siz
                    | Marker::Cap
                    | Marker::Cpf
                    | Marker::Cod
                    | Marker::Qcd
                    | Marker::Qcc
                    | Marker::Poc
                    | Marker::Crg
                    | Marker::Com
                    | Marker::Tlm
                    | Marker::Rgn
                    | Marker::Sot
                    | Marker::Sod
                    | Marker::Eoc
            )
        })
}

fn outside(marker: Marker, detail: &'static str) -> CodestreamError {
    unsupported(
        None,
        Some(marker),
        UnsupportedConstruct::HtBlockDecode,
        detail,
    )
}

/// Recheck the independently granted HT packet envelope, including every
/// header scope. Empty trailing parts do not create additional packet sources.
pub(super) fn resolve_maxshift(input: &[u8], c: &Codestream) -> Result<BoundedTileMaxshift> {
    if !envelope(c) {
        return Err(outside(
            Marker::Cod,
            "HT ROI window requires its bounded native coding envelope",
        ));
    }
    let mut active_part = None;
    let mut assignment = None;
    for marker in &c.markers {
        match marker.marker {
            Marker::Sot => {
                active_part = Some(parse_sot(
                    checked_slice(input, marker.data_offset, marker.data_len)?,
                    marker.offset,
                )?)
            }
            Marker::Rgn => {
                let part = active_part.ok_or_else(|| {
                    outside(Marker::Rgn, "HT ROI requires a tile-zero header assignment")
                })?;
                let rgn = parse_rgn_declaration(input, marker, &c.siz)?;
                if part.tile_index != 0
                    || part.tile_part_index != 0
                    || rgn.component_index != 0
                    || rgn.style != 0
                    || !(1..=15).contains(&rgn.shift)
                    || assignment.is_some()
                {
                    return Err(outside(
                        Marker::Rgn,
                        "HT ROI requires one tile-zero/component-zero Maxshift of one through fifteen",
                    ));
                }
                assignment = Some(BoundedTileMaxshift {
                    tile_index: 0,
                    component_index: 0,
                    shift: rgn.shift,
                });
            }
            Marker::Cod | Marker::Qcd | Marker::Qcc | Marker::Poc | Marker::Crg | Marker::Tlm
                if active_part.is_some() =>
            {
                return Err(outside(
                    marker.marker,
                    "HT ROI does not admit other functional tile-header overrides",
                ));
            }
            _ => {}
        }
    }
    let assignment = assignment.ok_or_else(|| {
        outside(
            Marker::Rgn,
            "HT ROI window requires an actual Maxshift assignment",
        )
    })?;
    for tile in tile_rects(c)? {
        let parts = c
            .tiles
            .iter()
            .filter(|part| part.tile_index == tile.tile_index)
            .collect::<Vec<_>>();
        if !(1..=4).contains(&parts.len())
            || parts.iter().enumerate().any(|(i, part)| {
                usize::from(part.tile_part_index) != i
                    || part.tile_part_count.map(usize::from) != Some(parts.len())
                    || (i != 0 && part.payload_len != Some(0))
            })
        {
            return Err(outside(
                Marker::Sot,
                "HT ROI admits one payload part and at most three empty trailing parts per tile",
            ));
        }
    }
    if parse_bounded_main_header_poc(input, c)?.is_none() {
        return Err(outside(
            Marker::Poc,
            "HT ROI requires a full-domain main-header LRCP progression volume",
        ));
    }
    validate_bounded_informational_crg(input, c)?;
    Ok(assignment)
}

/// An independently admitted full-resolution window inside native tile zero.
/// All tiles' packets have been validated; only tile zero is reconstructed.
#[cfg(feature = "std")]
pub struct PreparedHtj2kRoiWindowDecode<'a> {
    codestream: Codestream,
    candidate: HtCodestreamDecodeCandidate,
    payload: &'a [u8],
    contributions: Vec<PacketCodeBlockContribution>,
    tile: TileRect,
    region: TileRegionRequest,
    transfer: HtReversibleCodeBlockTransfer,
    maxshift: u8,
}

#[cfg(feature = "std")]
impl PreparedHtj2kRoiWindowDecode<'_> {
    pub fn bits_per_sample(&self) -> u8 {
        self.codestream.siz.components[0].bits_per_sample
    }
    pub fn signed(&self) -> bool {
        self.codestream.siz.components[0].signed
    }
    pub fn region(&self) -> TileRegionRequest {
        self.region
    }
}

/// Prepare planar component-zero output at full resolution, confined to the
/// first tile of the bounded one-level HT ROI profile. Non-HT inputs return
/// `None`; incompatible HT profiles remain unsupported. No pixels are exposed.
#[cfg(feature = "std")]
pub fn prepare_htj2k_roi_window_decode(
    input: &[u8],
    region: TileRegionRequest,
) -> Result<Option<PreparedHtj2kRoiWindowDecode<'_>>> {
    let codestream = parse(input)?;
    if codestream.kind != CodestreamKind::Htj2k
        || !codestream.markers.iter().any(|m| m.marker == Marker::Rgn)
    {
        return Ok(None);
    }
    // Resolve the bounded tile/component/style resource envelope before any
    // structural packet walk. It must not enumerate unsupported large grids.
    let roi = resolve_maxshift(input, &codestream)?;
    // Within that envelope, contradictory SINGLEHT signalling is invalid even
    // when the requested window or native coefficient bounds are unsupported.
    #[cfg(test)]
    STRUCTURAL_VALIDATION_CALLS.with(|calls| calls.set(calls.get() + 1));
    validate_part15_packet_signalling(input, &codestream)?;
    let part15 = codestream
        .capability
        .as_ref()
        .and_then(|cap| cap.part15)
        .ok_or(CodestreamError::SizeOverflow)?;
    if part15.code_block_mode != Part15CodeBlockMode::HtOnly || part15.cleanup_magnitude_bound > 18
    {
        return Err(outside(
            Marker::Cap,
            "HT ROI requires HTONLY and cleanup magnitude bound at most eighteen",
        ));
    }
    let tiles = tile_rects(&codestream)?;
    let tile = *tiles.first().ok_or(CodestreamError::SizeOverflow)?;
    if region.width == 0
        || region.height == 0
        || region
            .x
            .checked_add(region.width)
            .is_none_or(|end| end > tile.width)
        || region
            .y
            .checked_add(region.height)
            .is_none_or(|end| end > tile.height)
    {
        return Err(outside(
            Marker::Siz,
            "HT ROI window must lie inside native tile zero",
        ));
    }
    let quantization = parse_component_quantization(input, &codestream, 4)?;
    let q = &quantization[0];
    if q.style != transform::QuantizationStyle::NoQuantization
        || q.steps.iter().any(|step| {
            step.exponent == 0
                || step.exponent.checked_add(roi.shift).is_none_or(|e| e > 29)
                || step
                    .exponent
                    .checked_add(q.guard_bits)
                    .and_then(|v| v.checked_sub(1))
                    .and_then(|v| v.checked_add(roi.shift))
                    .is_none_or(|bits| bits > 30)
        })
    {
        return Err(outside(
            Marker::Qcc,
            "HT ROI quantisation and shift exceed the bounded coefficient store",
        ));
    }
    let exponent = q.steps[0].exponent;
    let k_max = exponent + q.guard_bits - 1;
    let transfer = HtReversibleCodeBlockTransfer {
        qcd_guard_bits: q.guard_bits,
        qcd_exponent: exponent,
        k_max,
        shift: 31 - k_max,
    };
    let mut selected = None;
    for rect in tiles {
        let part = codestream
            .tiles
            .iter()
            .find(|part| part.tile_index == rect.tile_index)
            .ok_or(CodestreamError::SizeOverflow)?;
        let payload = tile_payload(input, part)?;
        let contributions = parse_default_precinct_packets_from_source_with_ht_retention(
            input,
            &codestream,
            rect,
            &ContiguousPacketSource { bytes: payload },
            None,
            None,
            None,
            PacketOrganisationConfig::HT_ROI_WINDOW,
            None,
            HtCodingSetRetention::NativeAdmission,
        )?
        .contributions;
        classify_htonly_native_packet_mechanisms(&contributions)
            .map_err(|d| unsupported(None, None, d.construct, d.detail))?;
        if rect.tile_index == 0 {
            selected = Some((payload, contributions));
        }
    }
    let mut state = ht_marker_state(&codestream).ok_or(CodestreamError::SizeOverflow)?;
    // Packet topology, progression and native output are independently owned
    // here; the block candidate receives only the admitted coefficient layout.
    state.precincts_declared = false;
    let marker_candidate = ht::plan_decode_candidate_for_native_roi(state)
        .map_err(|c| outside(Marker::Cod, c.reason.message()))?;
    let candidate = HtCodestreamDecodeCandidate {
        marker_candidate,
        tile_part: codestream.tiles[0],
    };
    let (payload, contributions) = selected.ok_or(CodestreamError::SizeOverflow)?;
    Ok(Some(PreparedHtj2kRoiWindowDecode {
        codestream,
        candidate,
        payload,
        contributions,
        tile,
        region,
        transfer,
        maxshift: roi.shift,
    }))
}

/// Execute a prepared native window privately before any caller publication.
#[cfg(feature = "std")]
pub fn decode_prepared_htj2k_roi_window_owned(
    prepared: &PreparedHtj2kRoiWindowDecode<'_>,
) -> Result<DecodedImage> {
    let mut workspace = HtCodestreamDecodeWorkspace::new();
    let (_, _, components) = decode_htj2k_lossless_decomp_components(
        prepared.candidate,
        prepared.payload,
        &prepared.contributions,
        prepared.transfer,
        &prepared.codestream,
        prepared
            .codestream
            .coding_style
            .ok_or(CodestreamError::SizeOverflow)?,
        prepared.tile,
        None,
        Some(prepared.maxshift),
        &mut workspace,
    )?;
    let bytes_per_sample = usize::from(prepared.bits_per_sample().div_ceil(8));
    let stride = prepared.region.width as usize * bytes_per_sample;
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(stride * prepared.region.height as usize)
        .map_err(|_| CodestreamError::SizeOverflow)?;
    samples.resize(stride * prepared.region.height as usize, 0);
    copy_tile_intersection_to_region(
        &mut samples,
        stride,
        bytes_per_sample,
        prepared.region,
        prepared.tile,
        prepared.region,
        &components[0].samples,
    )?;
    Ok(DecodedImage {
        width: prepared.region.width,
        height: prepared.region.height,
        bits_per_sample: prepared.bits_per_sample(),
        signed: prepared.signed(),
        components: alloc::vec![DecodedComponent { samples }],
    })
}

/// Generate coefficient-domain ROI evidence without external source samples.
/// This is synthetic test support, not an application encoder profile.
#[doc(hidden)]
pub fn encode_htj2k_roi_window_test_fixture(
    width: u32,
    height: u32,
    bits: u8,
    signed: bool,
    shift: u8,
    parts: u8,
) -> Result<Vec<u8>> {
    fixture(width, height, bits, signed, shift, parts, 8, 64, true, true).map(|(bytes, _)| bytes)
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    width: u32,
    height: u32,
    bits: u8,
    signed: bool,
    shift: u8,
    parts: u8,
    layers: u16,
    block_size: u32,
    sop: bool,
    eph: bool,
) -> Result<(Vec<u8>, Vec<i32>)> {
    if width == 0
        || height == 0
        || width > 1024
        || height > 1024
        || !(1..=16).contains(&bits)
        || !(1..=15).contains(&shift)
        || !(1..=4).contains(&parts)
        || !(1..=8).contains(&layers)
        || !matches!(block_size, 32 | 64)
    {
        return Err(CodestreamError::SizeOverflow);
    }
    let mut output = Vec::new();
    // Coefficient values need four magnitude bits independent of native sample
    // precision. Guard two exercises the non-default HT transfer alignment.
    write_native_main_header(
        &mut output,
        width,
        height,
        128,
        128,
        bits,
        1,
        false,
        1,
        &[5, 5, 5, 5],
        true,
        0,
        layers,
    )?;
    output[42] |= if signed { 0x80 } else { 0 };
    let cap = find_marker(&output, 0, Marker::Cap).ok_or(CodestreamError::SizeOverflow)?;
    output[cap + 8..cap + 10].copy_from_slice(&0x180a_u16.to_be_bytes());
    let cod = find_marker(&output, 0, Marker::Cod).ok_or(CodestreamError::SizeOverflow)?;
    output[cod + 2..cod + 4].copy_from_slice(&14_u16.to_be_bytes());
    output[cod + 4] = 1 | (u8::from(sop) << 1) | (u8::from(eph) << 2);
    output[cod + 5] = 3;
    output[cod + 10] = if block_size == 32 { 3 } else { 4 };
    output[cod + 11] = output[cod + 10];
    output.splice(cod + 14..cod + 14, [0x77, 0x88]);
    let qcd = find_marker(&output, 0, Marker::Qcd).ok_or(CodestreamError::SizeOverflow)?;
    output[qcd + 4] = 0x40;
    // Effective QCC differs from the unused QCD default.
    output.extend_from_slice(&[0xff, 0x5d, 0, 8, 0, 0x40, 0x20, 0x20, 0x20, 0x20]);
    output.extend_from_slice(&[0xff, 0x5f, 0, 9, 0, 0]);
    output.extend_from_slice(&layers.to_be_bytes());
    output.extend_from_slice(&[33, 255, 0]);
    output.extend_from_slice(&[0xff, 0x63, 0, 6, 0, 11, 0, 17]);
    let mut expected = Vec::new();
    for ty in 0..height.div_ceil(128) {
        for tx in 0..width.div_ceil(128) {
            let tile = (ty * width.div_ceil(128) + tx) as u16;
            let w = (width - tx * 128).min(128);
            let h = (height - ty * 128).min(128);
            let original = (0..w * h)
                .map(|i| {
                    let value = (i.wrapping_mul(17).wrapping_add(i / 7) % 9) as i32 - 4;
                    if i % 3 == 0 { value } else { value.signum() }
                })
                .collect::<Vec<_>>();
            let encoded = original
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if tile == 0 && i % 3 == 0 {
                        *v * (1_i32 << shift)
                    } else {
                        *v
                    }
                })
                .collect::<Vec<_>>();
            if tile == 0 {
                expected = original;
            }
            let mut segments = Vec::new();
            let subbands = decomp_subband_specs(w, h, 1)?
                .into_iter()
                .map(|spec| {
                    encode_ht_decomp_subband_with_block_size(
                        w,
                        &encoded,
                        spec,
                        5 + if tile == 0 { shift } else { 0 },
                        &mut segments,
                        block_size,
                    )
                })
                .collect::<Result<Vec<_>>>()?;
            let mut packet = Vec::new();
            for layer in 0..layers {
                for resolution in 0..=1_u8 {
                    if sop {
                        packet.extend_from_slice(&[0xff, 0x91, 0, 4]);
                        packet
                            .extend_from_slice(&(2 * layer + u16::from(resolution)).to_be_bytes());
                    }
                    let contributes = layer == u16::from(resolution) % layers;
                    let active = subbands
                        .iter()
                        .filter(|s| s.resolution == resolution && !s.code_blocks.is_empty())
                        .collect::<Vec<_>>();
                    let mut writer = PacketBitWriter::new();
                    writer.write_bit(u32::from(contributes))?;
                    if contributes {
                        for subband in &active {
                            write_component_packet_header(
                                &mut writer,
                                subband.code_block_cols,
                                subband.code_block_rows,
                                &subband.code_blocks,
                            )?;
                        }
                    }
                    writer.align();
                    packet.extend_from_slice(writer.bytes());
                    if eph {
                        packet.extend_from_slice(&[0xff, 0x92]);
                    }
                    if contributes {
                        for subband in &active {
                            for b in subband.code_blocks.iter().filter(|b| b.included) {
                                packet.extend_from_slice(
                                    &segments[b.segment_offset..b.segment_offset + b.segment_len],
                                );
                            }
                        }
                    }
                }
            }
            for part in 0..parts {
                let start = output.len();
                output.extend_from_slice(&[0xff, 0x90, 0, 10]);
                output.extend_from_slice(&tile.to_be_bytes());
                output.extend_from_slice(&[0; 4]);
                output.extend_from_slice(&[part, parts]);
                if tile == 0 && part == 0 {
                    output.extend_from_slice(&[0xff, 0x5e, 0, 5, 0, 0, shift]);
                }
                output.extend_from_slice(&[0xff, 0x93]);
                if part == 0 {
                    output.extend_from_slice(&packet);
                }
                let len = (output.len() - start) as u32;
                output[start + 6..start + 10].copy_from_slice(&len.to_be_bytes());
            }
        }
    }
    output.extend_from_slice(&[0xff, 0xd9]);
    if parts > 1 {
        let parsed = parse(&output)?;
        let first = parsed
            .markers
            .iter()
            .find(|m| m.marker == Marker::Sot)
            .ok_or(CodestreamError::SizeOverflow)?
            .offset;
        let mut tlm = alloc::vec![0xff, 0x55];
        tlm.extend_from_slice(&(4_u16 + 5 * parsed.tiles.len() as u16).to_be_bytes());
        tlm.extend_from_slice(&[0, 0x50]);
        for part in &parsed.tiles {
            tlm.push(part.tile_index as u8);
            tlm.extend_from_slice(
                &part
                    .tile_part_length
                    .ok_or(CodestreamError::SizeOverflow)?
                    .to_be_bytes(),
            );
        }
        output.splice(first..first, tlm);
    }
    Ok((output, expected))
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn region(w: u32, h: u32) -> TileRegionRequest {
        TileRegionRequest {
            x: 0,
            y: 0,
            width: w,
            height: h,
        }
    }

    #[test]
    fn ht_roi_restores_signed_thresholds_and_native_windows() {
        for shift in [1, 3, 7, 15] {
            for bits in [1, 4, 8, 12, 16] {
                for signed in [false, true] {
                    for (w, h, parts, layers, block, sop, eph) in [
                        (17, 29, 1, 1, 32, false, false),
                        (129, 135, 4, 8, 64, true, true),
                    ] {
                        let (bytes, mut expected) =
                            fixture(w, h, bits, signed, shift, parts, layers, block, sop, eph)
                                .unwrap_or_else(|error| panic!("fixture {w}x{h} bits={bits} signed={signed} shift={shift}: {error:?}"));
                        let tw = w.min(128);
                        let th = h.min(128);
                        let prepared = prepare_htj2k_roi_window_decode(&bytes, region(tw, th))
                            .unwrap()
                            .unwrap();
                        assert_eq!(prepared.maxshift, shift);
                        if bits < 8 {
                            let mut state = ht_marker_state(&prepared.codestream).unwrap();
                            state.precincts_declared = false;
                            assert_eq!(
                                ht::plan_decode_candidate(state).unwrap_err().reason,
                                ht::HtUnsupportedReason::SamplePrecision
                            );
                        }
                        assert_eq!(
                            prepared.candidate.marker_candidate.sample_precision_bits(),
                            bits
                        );
                        inverse_reversible_5_3_levels_with_scratch(
                            &mut expected,
                            tw as usize,
                            tw,
                            th,
                            1,
                            &mut vec![0; tw.max(th) as usize * 3],
                        )
                        .unwrap();
                        let expected =
                            component_sample_slice_to_bytes(bits, signed, &expected).unwrap();
                        let decoded = decode_prepared_htj2k_roi_window_owned(&prepared).unwrap();
                        assert_eq!(
                            decoded.components[0].samples, expected,
                            "shift={shift} bits={bits} signed={signed}"
                        );
                        let cropped = prepare_htj2k_roi_window_decode(
                            &bytes,
                            TileRegionRequest {
                                x: 3,
                                y: 5,
                                width: 7,
                                height: 9,
                            },
                        )
                        .unwrap()
                        .unwrap();
                        let decoded = decode_prepared_htj2k_roi_window_owned(&cropped).unwrap();
                        let b = usize::from(bits.div_ceil(8));
                        let oracle = (5..14)
                            .flat_map(|y| {
                                expected[(y * tw as usize + 3) * b..(y * tw as usize + 10) * b]
                                    .iter()
                                    .copied()
                            })
                            .collect::<Vec<_>>();
                        assert_eq!(decoded.components[0].samples, oracle);
                    }
                }
            }
        }
    }

    #[test]
    fn ht_roi_transfer_threshold_is_in_the_extended_magnitude_domain() {
        for shift in [1, 3, 7, 15] {
            let threshold = 1_i32 << shift;
            let input = [
                -threshold - 1,
                -threshold,
                -threshold + 1,
                -1,
                0,
                1,
                threshold - 1,
                threshold,
                threshold + 1,
            ];
            let mut restored = input;
            realign_bounded_maxshift_coefficients(&mut restored, shift).unwrap();
            assert_eq!(
                restored,
                [-1, -1, -threshold + 1, -1, 0, 1, threshold - 1, 1, 1]
            );
        }
    }

    #[test]
    fn ht_roi_resolves_guard_alignment_and_small_native_tile_axes() {
        let (bytes, _) = fixture(17, 29, 12, true, 7, 1, 3, 32, true, false).unwrap();
        let original = decode_prepared_htj2k_roi_window_owned(
            &prepare_htj2k_roi_window_decode(&bytes, region(17, 29))
                .unwrap()
                .unwrap(),
        )
        .unwrap();
        let qcc = find_marker(&bytes, 0, Marker::Qcc).unwrap();
        let siz = find_marker(&bytes, 0, Marker::Siz).unwrap();
        for guard in [1, 2, 3] {
            for (width, height) in [(32_u32, 64_u32), (64, 32), (128, 128)] {
                let mut changed = bytes.clone();
                changed[qcc + 5] = guard << 5;
                changed[qcc + 6..qcc + 10].fill((6 - guard) << 3);
                changed[siz + 22..siz + 26].copy_from_slice(&width.to_be_bytes());
                changed[siz + 26..siz + 30].copy_from_slice(&height.to_be_bytes());
                let prepared = prepare_htj2k_roi_window_decode(&changed, region(17, 29))
                    .unwrap()
                    .unwrap();
                assert_eq!(
                    decode_prepared_htj2k_roi_window_owned(&prepared).unwrap(),
                    original
                );
            }
        }
    }

    #[test]
    fn ht_roi_singleht_contradictions_remain_invalid_before_admission() {
        let mut bytes = encode_htj2k_one_decomp_two_layer_multiple_set_test_fixture().unwrap();
        let siz = find_marker(&bytes, 0, Marker::Siz).unwrap();
        bytes[siz + 22..siz + 26].copy_from_slice(&128_u32.to_be_bytes());
        bytes[siz + 26..siz + 30].copy_from_slice(&128_u32.to_be_bytes());
        let cod = find_marker(&bytes, 0, Marker::Cod).unwrap();
        bytes[cod + 2..cod + 4].copy_from_slice(&14_u16.to_be_bytes());
        bytes[cod + 4] = 1;
        bytes[cod + 5] = 3;
        bytes.splice(cod + 14..cod + 14, [0x77, 0x88]);
        let sot = find_marker(&bytes, 0, Marker::Sot).unwrap();
        bytes.splice(sot..sot, [0xff, 0x5f, 0, 9, 0, 0, 0, 2, 33, 255, 0]);
        // Without RGN this belongs to the existing default structural route.
        // The ROI selector must not intercept it and hide the contradiction.
        let parsed = parse(&bytes).unwrap();
        assert!(!envelope(&parsed));
        assert!(matches!(
            validate_part15_packet_signalling(&bytes, &parsed),
            Err(CodestreamError::InvalidMarker {
                marker: Some(Marker::Cap),
                ..
            })
        ));
        let sot = find_marker(&bytes, 0, Marker::Sot).unwrap();
        let len = read_u32(&bytes, sot + 6).unwrap();
        bytes[sot + 6..sot + 10].copy_from_slice(&(len + 7).to_be_bytes());
        bytes.splice(sot + 12..sot + 12, [0xff, 0x5e, 0, 5, 0, 0, 7]);
        let cap = find_marker(&bytes, 0, Marker::Cap).unwrap();
        bytes[cap + 8] = 0x18;
        let parsed = parse(&bytes).unwrap();
        assert!(envelope(&parsed));
        assert!(matches!(
            validate_part15_packet_signalling(&bytes, &parsed),
            Err(CodestreamError::InvalidMarker {
                marker: Some(Marker::Cap),
                ..
            })
        ));
        assert!(matches!(
            prepare_htj2k_roi_window_decode(&bytes, region(8, 8)),
            Err(CodestreamError::InvalidMarker {
                marker: Some(Marker::Cap),
                ..
            })
        ));
        bytes[cap + 8] = 0x38;
        assert!(validate_part15_packet_signalling(&bytes, &parse(&bytes).unwrap()).is_ok());
        assert!(matches!(
            prepare_htj2k_roi_window_decode(&bytes, region(8, 8)),
            Err(CodestreamError::Unsupported {
                construct: UnsupportedConstruct::HtBlockDecode,
                ..
            })
        ));
    }

    #[test]
    fn ht_roi_resource_preflight_precedes_structural_packet_walking() {
        let (bytes, _) = fixture(8, 8, 4, true, 7, 1, 8, 64, false, false).unwrap();
        STRUCTURAL_VALIDATION_CALLS.with(|calls| calls.set(0));
        prepare_htj2k_roi_window_decode(&bytes, region(8, 8)).unwrap();
        STRUCTURAL_VALIDATION_CALLS.with(|calls| assert_eq!(calls.get(), 1));

        // Structurally legal, tiny empty tile parts make a large unsupported
        // grid without a large image allocation. Reject before packet walking,
        // not after the shared walk's per-tile source lookup work.
        let sot = find_marker(&bytes, 0, Marker::Sot).unwrap();
        for count in [65_u16, 256, 1024] {
            let mut oversized = bytes[..sot].to_vec();
            let siz = find_marker(&oversized, 0, Marker::Siz).unwrap();
            oversized[siz + 6..siz + 10].copy_from_slice(&(128 * u32::from(count)).to_be_bytes());
            for index in 0..count {
                oversized.extend_from_slice(&[0xff, 0x90, 0, 10]);
                oversized.extend_from_slice(&index.to_be_bytes());
                oversized.extend_from_slice(&(if index == 0 { 21_u32 } else { 14 }).to_be_bytes());
                oversized.extend_from_slice(&[0, 1]);
                if index == 0 {
                    oversized.extend_from_slice(&[0xff, 0x5e, 0, 5, 0, 0, 7]);
                }
                oversized.extend_from_slice(&[0xff, 0x93]);
            }
            oversized.extend_from_slice(&[0xff, 0xd9]);
            assert_eq!(parse(&oversized).unwrap().tiles.len(), usize::from(count));
            STRUCTURAL_VALIDATION_CALLS.with(|calls| calls.set(0));
            assert!(matches!(
                prepare_htj2k_roi_window_decode(&oversized, region(8, 8)),
                Err(CodestreamError::Unsupported {
                    construct: UnsupportedConstruct::HtBlockDecode,
                    ..
                })
            ));
            STRUCTURAL_VALIDATION_CALLS.with(|calls| assert_eq!(calls.get(), 0));
        }
    }

    #[test]
    fn ht_roi_validates_all_packets_headers_and_resource_boundaries() {
        let (bytes, _) = fixture(259, 263, 4, true, 7, 4, 8, 32, true, true).unwrap();
        let c = parse(&bytes).unwrap();
        let at = |kind| c.markers.iter().find(|m| m.marker == kind).unwrap().offset;
        let mut rejected = Vec::new();
        for (offset, value) in [
            (at(Marker::Rgn) + 4, 1),
            (at(Marker::Rgn) + 5, 1),
            (at(Marker::Rgn) + 6, 0),
            (at(Marker::Rgn) + 6, 16),
            (at(Marker::Rgn) + 6, 38),
            (at(Marker::Cod) + 5, 4),
            (at(Marker::Cod) + 9, 2),
            (at(Marker::Cod) + 13, 0),
            (at(Marker::Cod) + 14, 0x66),
            (at(Marker::Poc) + 10, 1),
            (at(Marker::Poc) + 8, 1),
            (at(Marker::Qcc) + 6, 31 << 3),
            (at(Marker::Cap) + 8, 0x08),
            (at(Marker::Cap) + 9, 11),
            (at(Marker::Siz) + 41, 2),
            (at(Marker::Crg) + 3, 5),
        ] {
            let mut changed = bytes.clone();
            changed[offset] = value;
            rejected.push(changed);
        }
        // Late unselected packet syntax must be validated before output geometry.
        let last = c
            .tiles
            .iter()
            .rfind(|part| part.tile_part_index == 0)
            .unwrap();
        let mut late = bytes.clone();
        let end = last.payload_offset.unwrap() + last.payload_len.unwrap();
        late[end - 1] = 0xff;
        rejected.push(late);
        // Non-empty trailing payload does not silently become a supported part.
        let trailing = c.tiles[1];
        let start = trailing.payload_offset.unwrap();
        let sot = c
            .markers
            .iter()
            .filter(|m| m.marker == Marker::Sot)
            .nth(1)
            .unwrap()
            .offset;
        let mut part = bytes.clone();
        part.splice(start..start, [0]);
        part[sot + 6..sot + 10].copy_from_slice(&15_u32.to_be_bytes());
        rejected.push(part);
        let mut oversized = bytes.clone();
        oversized[at(Marker::Siz) + 6..at(Marker::Siz) + 10]
            .copy_from_slice(&u32::MAX.to_be_bytes());
        rejected.push(oversized);
        for input in rejected {
            assert!(prepare_htj2k_roi_window_decode(&input, region(128, 128)).is_err());
        }
        for r in [
            region(0, 1),
            region(129, 1),
            TileRegionRequest {
                x: u32::MAX,
                y: 0,
                width: 2,
                height: 1,
            },
        ] {
            assert!(prepare_htj2k_roi_window_decode(&bytes, r).is_err());
        }
        // The entire largest admitted grid is still one visit per packet.
        let (many, _) = fixture(1024, 1024, 4, true, 7, 4, 8, 64, true, true).unwrap();
        let c = parse(&many).unwrap();
        let mut packets = 0;
        for rect in tile_rects(&c).unwrap() {
            let p = c
                .tiles
                .iter()
                .find(|p| p.tile_index == rect.tile_index)
                .unwrap();
            let payload = tile_payload(&many, p).unwrap();
            let parsed = parse_default_precinct_packets_from_source_with_ht_retention(
                &many,
                &c,
                rect,
                &ContiguousPacketSource { bytes: payload },
                None,
                None,
                None,
                PacketOrganisationConfig::HT_ROI_WINDOW,
                None,
                HtCodingSetRetention::NativeAdmission,
            )
            .unwrap();
            packets += parsed.packet_count;
            assert!(
                parsed
                    .contributions
                    .iter()
                    .all(|c| c.ht_coding_set_count() <= 1)
            );
        }
        assert_eq!(packets, 64 * 2 * 8);
        assert!(prepare_htj2k_roi_window_decode(&many, region(128, 128)).is_ok());
    }
}
