//! HT-owned native ROI admission and coefficient restoration.
//!
//! Project-authored from ISO/IEC 15444-1:2024 A.6.3, A.6.6, B.12.3 and H.1
//! (physical pages 51–52, 54–55, 99 and 156), retrieval
//! 34e5d1639b9f121807e620c001893ca9d2c8f977, and Part 15:2019 A.5 (page 38),
//! retrieval 10baf9472429d52f5d6b5f9b7a892dbed395b1db. No external payloads or
//! implementation material are used. This module does not grant Part 1 routes.

use super::*;

pub(super) fn envelope(c: &Codestream) -> bool {
    c.kind == CodestreamKind::Htj2k
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
    let roi = resolve_maxshift(input, &codestream)?;
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
