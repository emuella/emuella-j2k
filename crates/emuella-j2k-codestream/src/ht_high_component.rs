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
        && s.layers == 1
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
        && a.layer_end == 1 && b.layer_end == 1
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
