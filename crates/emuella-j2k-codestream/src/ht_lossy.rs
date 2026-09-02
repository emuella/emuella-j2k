//! Implementation-facing bounded irreversible HT orchestration.
//! This module owns the selected two-level target-rate contract documented in
//! ht-lossy-calibration.md, shared with the additive high-level lossy API.
use super::*;

const IRREVERSIBLE_QCD_GUARD_BITS: u8 = 3;
pub const MAX_CODESTREAM_BYTES: usize = 32 * 1024 * 1024;
const MAX_PIXELS: usize = 1_048_576;

fn resource_error() -> CodestreamError {
    unsupported(
        None,
        Some(Marker::Siz),
        UnsupportedConstruct::PacketDecode,
        "irreversible HT resource limit or working allocation exceeded",
    )
}
fn reserved<T>(len: usize) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| resource_error())?;
    Ok(values)
}
fn zeroed<T: Clone>(len: usize, value: T) -> Result<Vec<T>> {
    let mut values = reserved(len)?;
    values.resize(len, value);
    Ok(values)
}
fn dimensions(width: u32, height: u32, components: usize) -> Result<usize> {
    if !(4..=8192).contains(&width) || !(4..=8192).contains(&height) || !matches!(components, 1 | 3)
    {
        return Err(resource_error());
    }
    let pixels = checked_component_sample_count(width, height)?;
    if pixels > MAX_PIXELS {
        return Err(resource_error());
    }
    Ok(pixels)
}

/// Validate the shared format, geometry, resources and rate before layout conversion.
/// Returns the floored raw byte budget without allocating image-sized storage.
pub fn encode_byte_budget(
    width: u32,
    height: u32,
    bits: u8,
    components: usize,
    bits_per_pixel: f32,
) -> Result<usize> {
    let pixels = dimensions(width, height, components)?;
    if !matches!(bits, 8 | 16) {
        return Err(unsupported(
            None,
            Some(Marker::Siz),
            UnsupportedConstruct::SamplePrecision,
            "irreversible HT requires matching unsigned U8 or U16_LE planes",
        ));
    }
    byte_budget(pixels, bits_per_pixel)
}

/// Encode the selected internal HT profile from unsigned planar byte views.
/// Strides are bytes, U16 samples are little-endian, and all planes have the
/// same 8- or 16-bit precision. No MCT or layout conversion is performed.
/// Rate is complete raw-codestream bits per reference pixel, excluding wrapping.
/// The exact f32 is widened to f64, whole bits and then whole bytes are floored.
/// Invalid or unattainable rates fail; successful output is never padded.
pub fn encode_planar(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[&[u8]],
    strides: &[usize],
    bits_per_pixel: f32,
) -> Result<Vec<u8>> {
    let pixels = dimensions(width, height, planes.len())?;
    if !matches!(bits, 8 | 16) || strides.len() != planes.len() {
        return Err(unsupported(
            None,
            Some(Marker::Siz),
            UnsupportedConstruct::SamplePrecision,
            "irreversible HT requires matching unsigned U8 or U16_LE planes",
        ));
    }
    let budget = encode_byte_budget(width, height, bits, planes.len(), bits_per_pixel)?;
    let row_bytes = width as usize * usize::from(bits / 8);
    for (plane, &stride) in planes.iter().zip(strides) {
        let extent = stride
            .checked_mul(height as usize - 1)
            .and_then(|n| n.checked_add(row_bytes))
            .ok_or(CodestreamError::SizeOverflow)?;
        if stride < row_bytes || plane.len() < extent {
            return Err(invalid(
                None,
                Some(Marker::Siz),
                "irreversible HT plane stride or extent is too short",
            ));
        }
    }
    let mut analysed = reserved(planes.len())?;
    for (source, &stride) in planes.iter().zip(strides) {
        let mut plane = reserved(pixels)?;
        for y in 0..height as usize {
            for sample in
                source[y * stride..y * stride + row_bytes].chunks_exact(usize::from(bits / 8))
            {
                let value = if bits == 8 {
                    u16::from(sample[0])
                } else {
                    u16::from_le_bytes([sample[0], sample[1]])
                };
                plane.push(f32::from(value) - (1_u32 << (bits - 1)) as f32);
            }
        }
        analysed.push(plane);
    }
    analyse(width, height, &mut analysed)?;
    let (raw, _, _) = search(width, height, bits, &analysed, budget)?;
    if raw.len() > budget || budget - raw.len() > 32_usize.max(budget.div_ceil(500)) {
        return Err(unsupported(
            None,
            Some(Marker::Sot),
            UnsupportedConstruct::PacketDecode,
            "irreversible HT target rate is unattainable within the bounded non-padding tolerance",
        ));
    }
    Ok(raw)
}
fn byte_budget(pixels: usize, rate: f32) -> Result<usize> {
    let whole_bits = f64::from(rate) * pixels as f64;
    // Strictly below the cast boundary; do not rely on saturating float casts.
    if !rate.is_finite() || rate <= 0.0 || whole_bits < 8.0 || whole_bits >= usize::MAX as f64 {
        return Err(invalid(
            None,
            None,
            "irreversible HT rate must produce a finite positive representable byte budget",
        ));
    }
    Ok((whole_bits as usize) / 8)
}
pub(super) fn analyse(width: u32, height: u32, planes: &mut [Vec<f32>]) -> Result<()> {
    let mut scratch = zeroed(2 * width.max(height) as usize, 0.0)?;
    for plane in planes {
        for level in 0..2 {
            let (w, h) = resolution_dimensions(width, height, 2, 2 - level)?;
            let config = transform::Irreversible97Config {
                width: w as usize,
                height: h as usize,
                stride: width as usize,
                edges: transform::Irreversible97Edges::from_tile_origin(
                    0, 0, w as usize, h as usize,
                ),
            };
            transform::forward_irreversible_9_7(plane, config, &mut scratch)
                .map_err(|_| CodestreamError::SizeOverflow)?;
        }
    }
    Ok(())
}
// Also used by the historical observation probe: it retains unsuccessful
// complete trials for diagnosis, while encode_planar enforces the success gate.
pub(super) fn search(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[Vec<f32>],
    budget: usize,
) -> Result<(Vec<u8>, u32, usize)> {
    let specs = decomp_subband_specs(width, height, 2)?;
    let mut quantized_planes = reserved(planes.len())?;
    for plane in planes {
        quantized_planes.push(zeroed(plane.len(), 0_i32)?);
    }
    let mut lower = 4096;
    let mut upper = 65535;
    let mut best = candidate(
        width,
        height,
        bits,
        planes,
        &specs,
        upper,
        &mut quantized_planes,
    )?
    .ok_or_else(resource_error)?;
    let mut selected = upper;
    let mut visits = 1;
    if best.len() > budget {
        return Ok((best, selected, visits));
    }
    while lower <= upper && visits < 17 {
        let midpoint = lower + (upper - lower) / 2;
        visits += 1;
        let Some(raw) = candidate(
            width,
            height,
            bits,
            planes,
            &specs,
            midpoint,
            &mut quantized_planes,
        )?
        else {
            lower = midpoint + 1;
            continue;
        };
        if raw.len() <= budget {
            if raw.len() > best.len() {
                best = raw;
                selected = midpoint;
            }
            upper = midpoint - 1;
        } else {
            lower = midpoint + 1;
        }
    }
    Ok((best, selected, visits))
}

#[allow(clippy::too_many_arguments)]
fn candidate(
    width: u32,
    height: u32,
    bits_per_sample: u8,
    transformed_planes: &[Vec<f32>],
    specs: &[DecompSubbandSpec],
    coarseness: u32,
    quantized_planes: &mut [Vec<i32>],
) -> Result<Option<Vec<u8>>> {
    let octave = coarseness / 2048;
    let mantissa = u16::try_from(coarseness % 2048).map_err(|_| CodestreamError::SizeOverflow)?;
    let base_exponent = 31_u8
        .checked_sub(u8::try_from(octave).map_err(|_| CodestreamError::SizeOverflow)?)
        .ok_or(CodestreamError::SizeOverflow)?;
    let qcd_steps = specs
        .iter()
        .map(|spec| {
            let gain = match spec.kind {
                PacketSubbandKind::LowLow => 0,
                PacketSubbandKind::HighLow | PacketSubbandKind::LowHigh => 1,
                PacketSubbandKind::HighHigh => 2,
            };
            transform::IrreversibleQuantizationStep::new(
                base_exponent
                    .checked_add(gain)
                    .ok_or(CodestreamError::SizeOverflow)?,
                mantissa,
            )
            .map_err(|_| CodestreamError::SizeOverflow)
        })
        .collect::<Result<Vec<_>>>()?;
    for step in &qcd_steps {
        let decoder_available_bitplanes = IRREVERSIBLE_QCD_GUARD_BITS
            .checked_add(step.exponent)
            .and_then(|value| value.checked_sub(1))
            .ok_or(CodestreamError::SizeOverflow)?;
        if decoder_available_bitplanes > CLASSIC_COMPONENT_MAX_MAGNITUDE_BITPLANES {
            return Ok(None);
        }
    }
    let available_bitplanes = qcd_steps
        .iter()
        .map(|step| {
            IRREVERSIBLE_QCD_GUARD_BITS
                .checked_add(step.exponent)
                .and_then(|value| value.checked_sub(1))
                .ok_or(CodestreamError::SizeOverflow)
        })
        .collect::<Result<Vec<_>>>()?;
    let stride = usize::try_from(width).map_err(|_| CodestreamError::SizeOverflow)?;
    for (source, quantized) in transformed_planes.iter().zip(quantized_planes.iter_mut()) {
        for (spec, step) in specs.iter().zip(&qcd_steps) {
            let gain = match spec.kind {
                PacketSubbandKind::LowLow => 0,
                PacketSubbandKind::HighLow | PacketSubbandKind::LowHigh => 1,
                PacketSubbandKind::HighHigh => 2,
            };
            let delta = step
                .delta(bits_per_sample, gain)
                .map_err(|_| CodestreamError::SizeOverflow)?;
            for y in 0..usize::try_from(spec.height).map_err(|_| CodestreamError::SizeOverflow)? {
                let row = usize::try_from(spec.y)
                    .map_err(|_| CodestreamError::SizeOverflow)?
                    .checked_add(y)
                    .and_then(|value| value.checked_mul(stride))
                    .and_then(|value| value.checked_add(usize::try_from(spec.x).ok()?))
                    .ok_or(CodestreamError::SizeOverflow)?;
                let width =
                    usize::try_from(spec.width).map_err(|_| CodestreamError::SizeOverflow)?;
                for x in 0..width {
                    let value = source[row + x];
                    let scaled_magnitude = value.abs() / delta;
                    if scaled_magnitude > i32::MAX as f32 {
                        return Err(CodestreamError::SizeOverflow);
                    }
                    let magnitude = scaled_magnitude as i32;
                    if magnitude >= (1 << 17) {
                        return Ok(None);
                    }
                    quantized[row + x] = if value.is_sign_negative() {
                        -magnitude
                    } else {
                        magnitude
                    };
                }
            }
        }
    }

    let plane_refs = quantized_planes
        .iter()
        .map(Vec::as_slice)
        .collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut component_subbands = Vec::with_capacity(plane_refs.len());
    for plane in &plane_refs {
        let mut subbands = Vec::new();
        for (spec, depth) in specs.iter().zip(&available_bitplanes) {
            subbands.push(encode_ht_decomp_subband(
                width,
                plane,
                *spec,
                *depth,
                &mut segments,
            )?);
        }
        component_subbands.push(subbands);
    }
    let capacity = native_decomp_packet_capacity_hint(&component_subbands, &segments)?;
    if capacity > MAX_CODESTREAM_BYTES {
        return Err(resource_error());
    }
    let mut packet = reserved(capacity)?;
    write_native_decomp_packets(&mut packet, 2, &component_subbands, &segments)?;
    let mut codestream = reserved(
        packet
            .len()
            .checked_add(128)
            .ok_or(CodestreamError::SizeOverflow)?,
    )?;
    write_irreversible_main_header(
        &mut codestream,
        width,
        height,
        bits_per_sample,
        u16::try_from(plane_refs.len()).map_err(|_| CodestreamError::SizeOverflow)?,
        false,
        2,
        &qcd_steps,
        true,
    )?;
    write_tile_part(&mut codestream, 0, &packet, true)?;
    if codestream.len() > MAX_CODESTREAM_BYTES {
        return Err(resource_error());
    }
    Ok(Some(codestream))
}

fn selected_transform(codestream: &Codestream) -> bool {
    codestream.kind == CodestreamKind::Htj2k
        && codestream.coding_style.is_some_and(|style| {
            style.transform == WaveletTransform::Irreversible97 && style.decomposition_levels == 2
        })
}

// The generic SINGLEHT validator may walk packets before native classification.
// Put the new shape's resource boundary in front of that walk as well.
pub(super) fn preflight_packet_resources(input: &[u8], codestream: &Codestream) -> Result<()> {
    if selected_transform(codestream) {
        dimensions(
            codestream.image_width(),
            codestream.image_height(),
            usize::from(codestream.siz.component_count()),
        )?;
        if input.len() > MAX_CODESTREAM_BYTES {
            return Err(resource_error());
        }
    }
    Ok(())
}

fn envelope(input: &[u8], parsed: &Codestream) -> Result<bool> {
    if !selected_transform(parsed) {
        return Ok(false);
    }
    preflight_packet_resources(input, parsed)?;
    let siz = &parsed.siz;
    let style = parsed.coding_style.ok_or(CodestreamError::SizeOverflow)?;
    let Some(first) = siz.components.first() else {
        return Ok(false);
    };
    if siz.capabilities != 0x4000
        || siz.image_origin_x != 0
        || siz.image_origin_y != 0
        || siz.tile_origin_x != 0
        || siz.tile_origin_y != 0
        || siz.tile_width != siz.reference_grid_width
        || siz.tile_height != siz.reference_grid_height
        || !matches!(first.bits_per_sample, 8 | 16)
        || siz.components.iter().any(|c| {
            c.bits_per_sample != first.bits_per_sample
                || c.signed
                || c.horizontal_separation != 1
                || c.vertical_separation != 1
        })
        || style.entropy_coder != EntropyCoder::HtBlockCoding
        || style.multiple_component_transform
        || style.sop_markers
        || style.eph_markers
        || style.precincts_declared
        || style.layers != 1
        || style.progression_order != ProgressionOrder::Lrcp
        || style.code_block_style != 0x40
        || style.code_block_width_exponent != 6
        || style.code_block_height_exponent != 6
        || parsed
            .capability
            .as_ref()
            .is_none_or(|cap| cap.pcap != 0x20000 || cap.part15.is_none_or(|part| part.raw != 0x2a))
        || parsed.tiles.len() != 1
        || parsed.tiles[0].tile_index != 0
        || parsed.tiles[0].tile_part_index != 0
        || parsed.tiles[0].tile_part_count != Some(1)
    {
        return Ok(false);
    }
    // No override, progression, ROI, relocation or optional-marker expansion.
    let expected = [
        Marker::Siz,
        Marker::Cap,
        Marker::Cod,
        Marker::Qcd,
        Marker::Sot,
        Marker::Sod,
        Marker::Eoc,
    ];
    if parsed.markers.len() != expected.len()
        || !parsed
            .markers
            .iter()
            .zip(expected)
            .all(|(m, expected)| m.marker == expected)
    {
        return Ok(false);
    }
    let qcd = &parsed.markers[3];
    let data = checked_slice(input, qcd.data_offset, qcd.data_len)?;
    if data.len() != 15 || data[0] != 0x62 {
        return Ok(false);
    }
    let base = u16::from_be_bytes([data[1], data[2]]);
    let exponent = (base >> 11) as u8;
    let mantissa = base & 0x7ff;
    // Exactly the selected scalar-step family, including decoder-safe widths.
    for (pair, gain) in data[1..].chunks_exact(2).zip([0, 1, 1, 2, 1, 1, 2]) {
        let value = u16::from_be_bytes([pair[0], pair[1]]);
        let Some(expected) = exponent.checked_add(gain) else {
            return Ok(false);
        };
        if expected + 2 > 30 || value >> 11 != u16::from(expected) || value & 0x7ff != mantissa {
            return Ok(false);
        }
    }
    Ok(true)
}

fn prepare<'a>(
    input: &'a [u8],
    parsed: Codestream,
) -> Result<Option<PreparedHtj2kReducedComponentDecode<'a>>> {
    if !envelope(input, &parsed)? {
        return Ok(None);
    }
    validate_part15_packet_signalling(input, &parsed)?;
    classify_htj2k_profile_markers(&parsed, true, false)
        .map_err(|d| unsupported(None, None, d.construct, d.detail))?;
    let style = uniform_effective_coding_style(&parsed)?;
    let candidate = ht_decode_candidate_with_transform_permission(&parsed, true)
        .and_then(core::result::Result::ok)
        .ok_or_else(resource_error)?;
    let (tile_rect, payload) = single_part1_profile_tile(input, &parsed)?;
    let contributions = parse_default_precinct_lrcp_packets(input, &parsed, tile_rect, payload)?;
    classify_htonly_native_packet_mechanisms(&contributions)
        .map_err(|d| unsupported(None, None, d.construct, d.detail))?;
    if contributions.iter().any(|c| {
        c.coding_passes != 1
            || c.expanded_ht_coding_sets
                .as_ref()
                .is_some_and(|sets| sets.len() != 1)
            || c.available_bitplanes > 30
    }) {
        return Err(unsupported(
            None,
            Some(Marker::Sod),
            UnsupportedConstruct::HtBlockDecode,
            "irreversible full-image HT requires one cleanup pass per included block",
        ));
    }
    Ok(Some(PreparedHtj2kReducedComponentDecode {
        input,
        codestream: parsed,
        candidate,
        coding_style: style,
        reconstruction: Htj2kReducedComponentReconstruction::IrreversibleFullImage,
        tile_rect,
        contributions,
        request: Htj2kReducedComponentDecodeRequest {
            component_index: 0,
            discard_levels: 0,
        },
        output_width: tile_rect.width,
        output_height: tile_rect.height,
    }))
}

/// Exact full-resolution image-relative request for the bounded lossy HT
/// spatial decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LossyHtSpatialRegionRequest {
    region: TileRegionRequest,
    discard_levels: u8,
}

impl LossyHtSpatialRegionRequest {
    /// Construct one non-empty full-resolution request. Geometry and the
    /// supported discard count are checked during preparation.
    pub fn new(region: TileRegionRequest, discard_levels: u8) -> Self {
        Self {
            region,
            discard_levels,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct LossyHtSpatialRegionAccounting {
    total_code_blocks: u64,
    selected_code_blocks: u64,
    selected_block_coefficients: u64,
    maximum_block_coefficients: u64,
    maximum_segment_bytes: u64,
    entropy_scratch_ceiling_bytes: u64,
    compact_coefficient_samples: u64,
    synthesis_workspace_ceiling_samples: u64,
    output_samples: u64,
    deterministic_workspace_ceiling_bytes: u64,
}

pub struct PreparedLossyHtSpatialRegion<'a> {
    prepared: PreparedHtj2kReducedComponentDecode<'a>,
    region: TileRegionRequest,
    retained_resolution: u8,
    projected_region: TileRegionRequest,
    synthesis: SynthesisWindowPlan,
    selected_contribution_indices: Vec<usize>,
    accounting: LossyHtSpatialRegionAccounting,
}

impl PreparedLossyHtSpatialRegion<'_> {
    /// Full-resolution request retained by this plan.
    pub fn region(&self) -> TileRegionRequest {
        self.region
    }

    /// Output rectangle after independent ceiling projection of both
    /// half-open endpoints.
    pub fn projected_region(&self) -> TileRegionRequest {
        self.projected_region
    }

    pub fn retained_resolution(&self) -> u8 {
        self.retained_resolution
    }

    /// Selected component index. The public bounded route admits only zero.
    pub fn component_index(&self) -> u16 {
        0
    }

    pub fn bits_per_sample(&self) -> u8 {
        self.prepared.bits_per_sample()
    }

    pub fn signed(&self) -> bool {
        self.prepared.signed()
    }

    /// Deterministic active-use ceiling checked before reconstruction.
    pub fn required_workspace_bytes(&self) -> u64 {
        self.accounting.deterministic_workspace_ceiling_bytes
    }
}

const DEFAULT_LOSSY_HT_SPATIAL_WORKSPACE_LIMIT_BYTES: u64 = 64 * 1024 * 1024;

pub struct LossyHtSpatialRegionWorkspace {
    coefficients: Option<transform::WindowCoefficientPlane<f32>>,
    synthesis: transform::WindowSynthesisWorkspace<f32>,
    segment: Vec<u8>,
    maximum_bytes: u64,
}

impl LossyHtSpatialRegionWorkspace {
    pub fn new() -> Self {
        Self::with_maximum_bytes(DEFAULT_LOSSY_HT_SPATIAL_WORKSPACE_LIMIT_BYTES)
    }

    pub fn with_maximum_bytes(maximum_bytes: u64) -> Self {
        Self {
            coefficients: None,
            synthesis: transform::WindowSynthesisWorkspace::new(),
            segment: Vec::new(),
            maximum_bytes,
        }
    }

    /// Change the deterministic active-use ceiling. Retained allocation
    /// capacity is deliberately not treated as use by the next decode.
    pub fn set_maximum_bytes(&mut self, maximum_bytes: u64) {
        self.maximum_bytes = maximum_bytes;
    }

    pub fn maximum_bytes(&self) -> u64 {
        self.maximum_bytes
    }

    pub fn retained_heap_bytes(&self) -> u64 {
        self.coefficients
            .as_ref()
            .map_or(0, transform::WindowCoefficientPlane::retained_heap_bytes)
            .saturating_add(self.synthesis.retained_heap_bytes())
            .saturating_add(u64::try_from(self.segment.capacity()).unwrap_or(u64::MAX))
    }

    pub fn clear(&mut self) {
        if let Some(coefficients) = &mut self.coefficients {
            coefficients.clear();
        }
        self.synthesis.clear();
        self.segment.clear();
    }

    #[cfg(test)]
    fn compact_coefficient_retained_bytes(&self) -> u64 {
        self.coefficients
            .as_ref()
            .map_or(0, transform::WindowCoefficientPlane::retained_heap_bytes)
    }

    #[cfg(test)]
    fn synthesis_retained_bytes(&self) -> u64 {
        self.synthesis.retained_heap_bytes()
    }
}

impl Default for LossyHtSpatialRegionWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

fn checked_region_samples(region: AxisAlignedRegion) -> Result<u64> {
    u64::from(region.width)
        .checked_mul(u64::from(region.height))
        .ok_or(CodestreamError::SizeOverflow)
}

fn checked_add_u64(left: u64, right: u64) -> Result<u64> {
    left.checked_add(right).ok_or(CodestreamError::SizeOverflow)
}

fn checked_count_bytes(count: usize, element_bytes: usize) -> Result<u64> {
    u64::try_from(count)
        .ok()
        .and_then(|count| count.checked_mul(element_bytes as u64))
        .ok_or(CodestreamError::SizeOverflow)
}

fn ht_block_layout_scratch_ceiling_bytes(width: u16, height: u16) -> Result<u64> {
    let dimensions =
        ht::HtCodeBlockDimensions::new(width, height).map_err(|_| CodestreamError::SizeOverflow)?;
    let block = ht::HtBlockLayout::new(dimensions);
    let scratch =
        ht::HtBlockScratchLayout::new(block).map_err(|_| CodestreamError::SizeOverflow)?;
    let coefficients = dimensions.coefficient_count();
    let side_inputs = block.vlc_cleanup_side_input_counts();
    let coefficient_record_bytes = core::mem::size_of::<ht::HtCleanupCoefficientSymbol>()
        .checked_add(core::mem::size_of::<ht::HtCoefficientRefinementValue>())
        .and_then(|value| {
            value.checked_add(core::mem::size_of::<ht::HtCoefficientMagnitudeSignValue>())
        })
        .and_then(|value| {
            value.checked_add(core::mem::size_of::<ht::HtVlcCleanupCoefficientOutput>())
        })
        .ok_or(CodestreamError::SizeOverflow)?;
    let mut logical_bytes =
        checked_count_bytes(scratch.total_words(), core::mem::size_of::<u16>())?;
    logical_bytes = checked_add_u64(
        logical_bytes,
        checked_count_bytes(coefficients, coefficient_record_bytes)?,
    )?;
    logical_bytes = checked_add_u64(
        logical_bytes,
        checked_count_bytes(coefficients, core::mem::size_of::<i32>())?,
    )?;
    logical_bytes = checked_add_u64(
        logical_bytes,
        checked_count_bytes(
            side_inputs.quad_side_bits,
            core::mem::size_of::<ht::HtVlcQuadCleanupSideBits>(),
        )?,
    )?;
    logical_bytes = checked_add_u64(
        logical_bytes,
        checked_count_bytes(side_inputs.odd_tail_u_values, core::mem::size_of::<u16>())?,
    )?;
    logical_bytes = checked_add_u64(
        logical_bytes,
        checked_count_bytes(
            block.line_pair_count(),
            core::mem::size_of::<ht::HtVlcContextProgression>(),
        )?,
    )?;
    // The entropy workspace is fresh for each public call. Twice the logical
    // element storage conservatively covers Vec growth while retaining the
    // exact selected-block geometry as the accounting owner.
    logical_bytes
        .checked_mul(2)
        .ok_or(CodestreamError::SizeOverflow)
}

fn prepared_magsgn_scratch_ceiling_bytes(maximum_segment_bytes: u64) -> Result<u64> {
    // The accelerated representation retains at most the physical bytes plus
    // 24 padding bytes, and one usize stuffed-bit offset per physical byte.
    // Double that logical storage to cover fresh Vec capacity growth.
    maximum_segment_bytes
        .checked_add(24)
        .and_then(|bytes| {
            maximum_segment_bytes
                .checked_mul(core::mem::size_of::<usize>() as u64)
                .and_then(|offsets| bytes.checked_add(offsets))
        })
        .and_then(|bytes| bytes.checked_mul(2))
        .ok_or(CodestreamError::SizeOverflow)
}

fn lossy_ht_window_storage_accounting(
    synthesis: &SynthesisWindowPlan,
    contributions: &[PacketCodeBlockContribution],
    selected_contribution_indices: &[usize],
) -> Result<LossyHtSpatialRegionAccounting> {
    let mut compact_coefficient_samples = checked_region_samples(synthesis.lowest_low_low)?;
    let mut maximum_plane_samples = compact_coefficient_samples;
    let mut maximum_horizontal_low = 0_u64;
    let mut maximum_horizontal_high = 0_u64;
    let mut maximum_line = 1_u64;
    for level in &synthesis.levels {
        for region in [level.high_low, level.low_high, level.high_high] {
            compact_coefficient_samples =
                checked_add_u64(compact_coefficient_samples, checked_region_samples(region)?)?;
        }
        maximum_plane_samples = maximum_plane_samples.max(checked_region_samples(level.output)?);
        maximum_horizontal_low = maximum_horizontal_low.max(
            u64::from(level.output.width)
                .checked_mul(u64::from(level.low_low.height))
                .ok_or(CodestreamError::SizeOverflow)?,
        );
        maximum_horizontal_high = maximum_horizontal_high.max(
            u64::from(level.output.width)
                .checked_mul(u64::from(level.low_high.height))
                .ok_or(CodestreamError::SizeOverflow)?,
        );
        for (low, high) in [
            (level.low_low.width, level.high_low.width),
            (level.low_high.width, level.high_high.width),
            (level.low_low.height, level.low_high.height),
        ] {
            maximum_line = maximum_line.max(
                u64::from(low)
                    .checked_add(u64::from(high))
                    .ok_or(CodestreamError::SizeOverflow)?,
            );
        }
    }
    // Serial WindowSynthesisWorkspace::reserve_for_plan retains two maximum
    // planes, the two maximum horizontal intermediates, and two line buffers.
    let synthesis_workspace_ceiling_samples = maximum_plane_samples
        .checked_mul(2)
        .and_then(|value| value.checked_add(maximum_horizontal_low))
        .and_then(|value| value.checked_add(maximum_horizontal_high))
        .and_then(|value| value.checked_add(maximum_line.checked_mul(2)?))
        .ok_or(CodestreamError::SizeOverflow)?;
    let mut selected_block_coefficients = 0_u64;
    let mut maximum_block_coefficients = 0_u64;
    let mut maximum_segment_bytes = 0_u64;
    let mut maximum_layout_scratch_bytes = 0_u64;
    for &index in selected_contribution_indices {
        let contribution = contributions
            .get(index)
            .ok_or(CodestreamError::SizeOverflow)?;
        let coefficients = u64::from(contribution.width)
            .checked_mul(u64::from(contribution.height))
            .ok_or(CodestreamError::SizeOverflow)?;
        selected_block_coefficients = checked_add_u64(selected_block_coefficients, coefficients)?;
        maximum_block_coefficients = maximum_block_coefficients.max(coefficients);
        maximum_layout_scratch_bytes = maximum_layout_scratch_bytes.max(
            ht_block_layout_scratch_ceiling_bytes(contribution.width, contribution.height)?,
        );
        maximum_segment_bytes = maximum_segment_bytes.max(
            u64::try_from(contribution.codeword_len).map_err(|_| CodestreamError::SizeOverflow)?,
        );
    }
    let entropy_scratch_ceiling_bytes = maximum_layout_scratch_bytes
        .checked_add(prepared_magsgn_scratch_ceiling_bytes(
            maximum_segment_bytes,
        )?)
        .ok_or(CodestreamError::SizeOverflow)?;
    let output_samples = checked_region_samples(synthesis.output_region)?;
    let f32_samples = compact_coefficient_samples
        .checked_add(synthesis_workspace_ceiling_samples)
        .and_then(|value| value.checked_add(output_samples))
        .ok_or(CodestreamError::SizeOverflow)?;
    let deterministic_workspace_ceiling_bytes = f32_samples
        .checked_mul(core::mem::size_of::<f32>() as u64)
        .and_then(|value| {
            maximum_block_coefficients
                .checked_mul(core::mem::size_of::<i32>() as u64)
                .and_then(|bytes| value.checked_add(bytes))
        })
        .and_then(|value| value.checked_add(maximum_segment_bytes))
        .and_then(|value| value.checked_add(entropy_scratch_ceiling_bytes))
        .and_then(|value| {
            output_samples
                .checked_mul(2)
                .and_then(|bytes| value.checked_add(bytes))
        })
        .ok_or(CodestreamError::SizeOverflow)?;
    Ok(LossyHtSpatialRegionAccounting {
        total_code_blocks: u64::try_from(contributions.len())
            .map_err(|_| CodestreamError::SizeOverflow)?,
        selected_code_blocks: u64::try_from(selected_contribution_indices.len())
            .map_err(|_| CodestreamError::SizeOverflow)?,
        selected_block_coefficients,
        maximum_block_coefficients,
        maximum_segment_bytes,
        entropy_scratch_ceiling_bytes,
        compact_coefficient_samples,
        synthesis_workspace_ceiling_samples,
        output_samples,
        deterministic_workspace_ceiling_bytes,
    })
}

pub fn prepare_lossy_ht_spatial_region(
    input: &[u8],
    request: LossyHtSpatialRegionRequest,
) -> Result<PreparedLossyHtSpatialRegion<'_>> {
    if !matches!(request.discard_levels, 0..=2) {
        return Err(unsupported(
            None,
            Some(Marker::Cod),
            UnsupportedConstruct::Transform,
            "lossy HT spatial reconstruction accepts zero, one or two discarded resolution levels",
        ));
    }
    prepare_lossy_ht_spatial_region_at_discard(input, request)
}

fn prepare_lossy_ht_spatial_region_at_discard(
    input: &[u8],
    request: LossyHtSpatialRegionRequest,
) -> Result<PreparedLossyHtSpatialRegion<'_>> {
    if request.discard_levels > 2 {
        return Err(CodestreamError::SizeOverflow);
    }
    let parsed = parse(input)?;
    if !envelope(input, &parsed)?
        || parsed.siz.component_count() != 1
        || parsed.siz.components.first().is_none_or(|component| {
            component.bits_per_sample != 16
                || component.signed
                || component.horizontal_separation != 1
                || component.vertical_separation != 1
        })
    {
        return Err(unsupported(
            None,
            Some(Marker::Siz),
            UnsupportedConstruct::ComponentSampling,
            "lossy HT spatial reconstruction requires the raw U16 greyscale encoder envelope",
        ));
    }
    // prepare() validates the complete packet stream before selection.
    let prepared = prepare(input, parsed)?.ok_or_else(resource_error)?;
    let reduced_request = Htj2kReducedComponentDecodeRequest {
        component_index: 0,
        discard_levels: request.discard_levels,
    };
    let (retained_resolution, output_width, output_height, _) =
        htj2k_reduced_component_output_geometry(
            prepared.tile_rect.width,
            prepared.tile_rect.height,
            prepared.coding_style.decomposition_levels,
            reduced_request,
        )?;
    let projected_region = reduced_part1_region(
        &prepared.codestream.siz,
        request.region,
        request.discard_levels,
    )?;
    let synthesis = plan_synthesis_window(
        output_width,
        output_height,
        retained_resolution,
        AxisAlignedRegion {
            x: projected_region.x,
            y: projected_region.y,
            width: projected_region.width,
            height: projected_region.height,
        },
        WaveletTransform::Irreversible97,
    )?;
    let mut selected_contribution_indices = reserved(prepared.contributions.len())?;
    for (index, contribution) in prepared.contributions.iter().enumerate() {
        if contribution.component_index == 0
            && contribution.resolution <= retained_resolution
            && synthesis_window_dependency_selects_contribution(&synthesis, contribution)?
        {
            selected_contribution_indices.push(index);
        }
    }
    if selected_contribution_indices.is_empty() {
        return Err(CodestreamError::SizeOverflow);
    }
    let accounting = lossy_ht_window_storage_accounting(
        &synthesis,
        &prepared.contributions,
        &selected_contribution_indices,
    )?;
    Ok(PreparedLossyHtSpatialRegion {
        prepared,
        region: request.region,
        retained_resolution,
        projected_region,
        synthesis,
        selected_contribution_indices,
        accounting,
    })
}

pub fn decode_prepared_lossy_ht_spatial_region(
    plan: &PreparedLossyHtSpatialRegion<'_>,
    workspace: &mut LossyHtSpatialRegionWorkspace,
) -> Result<(Vec<u8>, transform::WindowSynthesisReport)> {
    if plan.accounting.deterministic_workspace_ceiling_bytes > workspace.maximum_bytes {
        return Err(resource_error());
    }
    let mut decode = HtCodestreamDecodeWorkspace::new();
    let (_, payload) = single_part1_profile_tile(plan.prepared.input, &plan.prepared.codestream)?;
    let component = plan
        .prepared
        .codestream
        .siz
        .components
        .first()
        .ok_or(CodestreamError::SizeOverflow)?;
    if let Some(coefficients) = &mut workspace.coefficients {
        coefficients.reset_for_plan(&plan.synthesis)?;
    } else {
        workspace.coefficients = Some(transform::WindowCoefficientPlane::<f32>::new(
            &plan.synthesis,
        )?);
    }
    let coefficients_plane = workspace
        .coefficients
        .as_mut()
        .ok_or(CodestreamError::SizeOverflow)?;
    for &index in &plan.selected_contribution_indices {
        let contribution = plan
            .prepared
            .contributions
            .get(index)
            .ok_or(CodestreamError::SizeOverflow)?;
        let expanded_coding_set = contribution
            .expanded_ht_coding_sets
            .as_deref()
            .and_then(<[HtCodeBlockCodingSet]>::last)
            .copied();
        let coding_passes = expanded_coding_set.map_or(contribution.coding_passes, |coding_set| {
            coding_set.coding_passes
        });
        if !(1..=3).contains(&coding_passes) {
            return Err(unsupported(
                None,
                Some(Marker::Sod),
                UnsupportedConstruct::HtBlockDecode,
                "lossy HT spatial reconstruction accepts up to three coding passes per HT set",
            ));
        }
        let active_dimensions =
            ht::HtCodeBlockDimensions::new(contribution.width, contribution.height)
                .map_err(|_| CodestreamError::SizeOverflow)?;
        let code_block_segment =
            code_block_segment_for_decode(payload, contribution, &mut workspace.segment)?;
        let (segment, cleanup_len, missing_most_significant_bitplanes) =
            if let Some(coding_set) = expanded_coding_set {
                let end = coding_set
                    .byte_offset
                    .checked_add(coding_set.byte_len)
                    .ok_or(CodestreamError::SizeOverflow)?;
                (
                    code_block_segment
                        .get(coding_set.byte_offset..end)
                        .ok_or(CodestreamError::SizeOverflow)?,
                    coding_set.cleanup_byte_len,
                    coding_set.missing_most_significant_bitplanes,
                )
            } else {
                if !contribution.ht_coded {
                    return Err(CodestreamError::SizeOverflow);
                }
                (
                    code_block_segment,
                    contribution
                        .coding_segments
                        .first()
                        .map_or(code_block_segment.len(), |cleanup| cleanup.byte_len),
                    contribution.missing_most_significant_bitplanes,
                )
            };
        let cleanup_segment = segment
            .get(..cleanup_len)
            .ok_or(CodestreamError::SizeOverflow)?;
        let segment_layout =
            ht::HtCleanupPassSegmentLayout::from_cleanup_pass_bytes(cleanup_segment)
                .map_err(ht_cleanup_pass_segment_layout_error)?;
        let code_block_input = HtCodestreamCodeBlockInput {
            candidate: plan.prepared.candidate,
            active_dimensions,
            missing_most_significant_bitplanes,
            coding_passes,
            segment_layout,
            segment,
        };
        let coefficient_count = active_dimensions.coefficient_count();
        if decode.coefficients.len() < coefficient_count {
            decode
                .coefficients
                .try_reserve_exact(coefficient_count - decode.coefficients.len())
                .map_err(|_| resource_error())?;
        }
        decode.coefficients.resize(coefficient_count, 0);
        let (decoded, _) = decode.block.decode_code_block_input_into_with_progress(
            code_block_input,
            &mut decode.coefficients[..coefficient_count],
        )?;
        if decoded.is_none() {
            return Err(CodestreamError::SizeOverflow);
        }
        let step = contribution
            .irreversible_quantization_step
            .ok_or(CodestreamError::SizeOverflow)?;
        let gain = match contribution.subband {
            PacketSubbandKind::LowLow => 0,
            PacketSubbandKind::HighLow | PacketSubbandKind::LowHigh => 1,
            PacketSubbandKind::HighHigh => 2,
        };
        let delta = step
            .delta(component.bits_per_sample, gain)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        for &coefficient in &decode.coefficients[..coefficient_count] {
            let doubled = ht_irreversible_doubled_half_step_coefficient(
                coefficient,
                contribution.available_bitplanes,
            )?;
            if doubled.abs() > 262143.0 {
                return Err(invalid(
                    None,
                    Some(Marker::Sod),
                    "lossy HT spatial coefficient exceeds the selected magnitude bound",
                ));
            }
        }
        let shift = 30_u8
            .checked_sub(contribution.available_bitplanes)
            .ok_or(CodestreamError::SizeOverflow)?;
        let alignment = (1_u32
            .checked_shl(u32::from(shift))
            .ok_or(CodestreamError::SizeOverflow)?) as f32;
        place_direct_window_coefficients(
            coefficients_plane,
            contribution,
            &decode.coefficients[..coefficient_count],
            |coefficient| coefficient as f32 / alignment * (0.5 * delta),
        )?;
    }
    if coefficients_plane.sample_count() != plan.accounting.compact_coefficient_samples {
        return Err(CodestreamError::SizeOverflow);
    }
    workspace.synthesis.reserve_for_plan(&plan.synthesis)?;
    let report = transform::inverse_irreversible_9_7_window(
        coefficients_plane,
        &plan.synthesis,
        &mut workspace.synthesis,
        false,
    )?;
    if report.work.output_samples != plan.accounting.output_samples
        || report.peak_value_bytes
            > plan
                .accounting
                .synthesis_workspace_ceiling_samples
                .checked_mul(core::mem::size_of::<f32>() as u64)
                .ok_or(CodestreamError::SizeOverflow)?
    {
        return Err(CodestreamError::SizeOverflow);
    }
    let samples = irreversible_component_samples_to_bytes(component, workspace.synthesis.output())?;
    Ok((samples, report))
}

#[cfg(test)]
fn prepare_lossy_ht_spatial_region_calibration(
    input: &[u8],
    request: LossyHtSpatialRegionRequest,
) -> Result<PreparedLossyHtSpatialRegion<'_>> {
    prepare_lossy_ht_spatial_region_at_discard(input, request)
}

/// Prepare component zero from the bounded unsigned U16 greyscale encoder
/// profile at exactly one or two discarded resolution levels.
///
/// This reuses the complete full-image envelope and packet admission before
/// reducing the retained packet set and checked output geometry. It does not
/// admit JPH wrapping, other sample formats, component selections or discard
/// counts.
pub fn prepare_reduced_component_decode(
    input: &[u8],
    request: Htj2kReducedComponentDecodeRequest,
) -> Result<Option<PreparedHtj2kReducedComponentDecode<'_>>> {
    let parsed = parse(input)?;
    if parsed.kind != CodestreamKind::Htj2k {
        return Ok(None);
    }
    if !envelope(input, &parsed)? {
        return Ok(None);
    }
    if request.component_index != 0 || !matches!(request.discard_levels, 1 | 2) {
        return Err(unsupported(
            None,
            Some(Marker::Cod),
            UnsupportedConstruct::ComponentCount,
            "reduced irreversible HT encoder output selects component zero at exactly one or two discarded resolution levels",
        ));
    }
    if parsed.siz.component_count() != 1
        || parsed.siz.components.first().is_none_or(|component| {
            component.bits_per_sample != 16
                || component.signed
                || component.horizontal_separation != 1
                || component.vertical_separation != 1
        })
    {
        return Err(unsupported(
            None,
            Some(Marker::Siz),
            UnsupportedConstruct::ComponentSampling,
            "reduced irreversible HT encoder output requires one unsigned U16 unit-sampled component",
        ));
    }

    let Some(mut prepared) = prepare(input, parsed)? else {
        return Ok(None);
    };
    let (retained_resolution, output_width, output_height, _) =
        htj2k_reduced_component_output_geometry(
            prepared.tile_rect.width,
            prepared.tile_rect.height,
            prepared.coding_style.decomposition_levels,
            request,
        )?;
    prepared
        .contributions
        .retain(|contribution| contribution.resolution <= retained_resolution);
    prepared.request = request;
    prepared.output_width = output_width;
    prepared.output_height = output_height;
    Ok(Some(prepared))
}

/// Classify the exact full-image irreversible profile, including packet grammar.
/// Entropy payload validity is established by decode, not by support inspection.
pub fn is_profile(input: &[u8], parsed: &Codestream) -> bool {
    envelope(input, parsed).is_ok_and(|admitted| admitted)
        && prepare(input, parsed.clone()).is_ok_and(|plan| plan.is_some())
}

/// Decode every native component of the bounded irreversible HT profile.
/// Reconstruction is staged in owned planes; no caller memory is published here.
pub fn decode_owned_with_workspace(
    input: &[u8],
    workspace: &mut HtCodestreamDecodeWorkspace,
) -> Result<Option<DecodedImage>> {
    let parsed = parse(input)?;
    let Some(mut prepared) = prepare(input, parsed)? else {
        return Ok(None);
    };
    let mut components = reserved(usize::from(prepared.codestream.siz.component_count()))?;
    for index in 0..prepared.codestream.siz.component_count() {
        prepared.request.component_index = index;
        let decoded =
            decode_prepared_htj2k_reduced_component_owned_with_workspace(&prepared, workspace)?;
        components.extend(decoded.components);
    }
    Ok(Some(DecodedImage {
        width: prepared.output_width,
        height: prepared.output_height,
        bits_per_sample: prepared.bits_per_sample(),
        signed: false,
        components,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn encoded() -> Vec<u8> {
        let source = crate::ht_lossy_test_support::source(257, 193, 8, 3, 0);
        let planes = source
            .iter()
            .map(|p| p.iter().map(|&v| v as u8).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        encode_planar(
            257,
            193,
            8,
            &planes.iter().map(Vec::as_slice).collect::<Vec<_>>(),
            &[257; 3],
            2.0,
        )
        .unwrap()
    }
    fn rejects(bytes: &[u8]) {
        if let Ok(parsed) = parse(bytes) {
            assert!(!is_profile(bytes, &parsed));
        }
        assert!(
            decode_owned_with_workspace(bytes, &mut HtCodestreamDecodeWorkspace::new())
                .map_or(true, |image| image.is_none())
        );
    }

    fn crop_u16(plane: &[u8], plane_width: u32, region: TileRegionRequest) -> Vec<u8> {
        let row_bytes = usize::try_from(region.width).unwrap() * 2;
        let mut cropped = Vec::with_capacity(row_bytes * usize::try_from(region.height).unwrap());
        for y in region.y..region.y + region.height {
            let start = (usize::try_from(y).unwrap() * usize::try_from(plane_width).unwrap()
                + usize::try_from(region.x).unwrap())
                * 2;
            cropped.extend_from_slice(&plane[start..start + row_bytes]);
        }
        cropped
    }

    fn established_component(raw: &[u8], discard_levels: u8) -> DecodedImage {
        let mut workspace = HtCodestreamDecodeWorkspace::new();
        if discard_levels == 0 {
            return decode_owned_with_workspace(raw, &mut workspace)
                .unwrap()
                .unwrap();
        }
        let prepared = prepare_reduced_component_decode(
            raw,
            Htj2kReducedComponentDecodeRequest {
                component_index: 0,
                discard_levels,
            },
        )
        .unwrap()
        .unwrap();
        decode_prepared_htj2k_reduced_component_owned_with_workspace(&prepared, &mut workspace)
            .unwrap()
    }

    #[test]
    fn spatial_region_calibration_matches_established_reconstruction_with_bounded_storage() {
        let width = 1024_u32;
        let height = 1024_u32;
        let source = crate::ht_lossy_test_support::source(width, height, 16, 1, 1)
            .pop()
            .unwrap();
        let source_bytes = source
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect::<Vec<_>>();
        let raw = encode_planar(
            width,
            height,
            16,
            &[source_bytes.as_slice()],
            &[width as usize * 2],
            4.0,
        )
        .unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&source_bytes)),
            "75855d11fddce88d3377bcaeab864905cef5cc724f0059da81271832010c9054"
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(&raw)),
            "f376f5c04b13c640fec6b80ba52bfb198cc1832d75f6850411ba801e035da597"
        );

        let regions = [
            TileRegionRequest {
                x: 137,
                y: 211,
                width: 43,
                height: 35,
            },
            TileRegionRequest {
                x: 973,
                y: 981,
                width: 51,
                height: 43,
            },
        ];
        let mut observed = Vec::new();
        for discard_levels in 0..=2 {
            let established = established_component(&raw, discard_levels);
            assert_eq!(established.components.len(), 1);
            for region in regions {
                let request = LossyHtSpatialRegionRequest {
                    region,
                    discard_levels,
                };
                let plan = prepare_lossy_ht_spatial_region_calibration(&raw, request).unwrap();
                let mut workspace = LossyHtSpatialRegionWorkspace::new();
                let (actual, report) =
                    decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
                let expected = crop_u16(
                    &established.components[0].samples,
                    established.width,
                    plan.projected_region,
                );
                assert_eq!(actual, expected);
                assert_eq!(plan.retained_resolution, 2 - discard_levels);
                assert_eq!(
                    plan.accounting.output_samples,
                    u64::from(plan.projected_region.width)
                        * u64::from(plan.projected_region.height)
                );
                assert_eq!(report.work.output_samples, plan.accounting.output_samples);
                assert!(plan.accounting.selected_code_blocks < plan.accounting.total_code_blocks);
                assert_eq!(
                    plan.accounting.selected_code_blocks,
                    plan.selected_contribution_indices.len() as u64
                );
                assert!(
                    plan.accounting.selected_block_coefficients
                        >= plan.accounting.compact_coefficient_samples
                );
                assert!(
                    plan.accounting.compact_coefficient_samples
                        < u64::from(established.width) * u64::from(established.height)
                );
                assert!(plan.accounting.synthesis_workspace_ceiling_samples < 1024_u64 * 1024);
                assert!(plan.accounting.output_samples < 1024_u64 * 1024);
                for &index in &plan.selected_contribution_indices {
                    let contribution = &plan.prepared.contributions[index];
                    assert!(
                        synthesis_window_dependency_selects_contribution(
                            &plan.synthesis,
                            contribution
                        )
                        .unwrap()
                    );
                }
                observed.push((region, discard_levels, plan.accounting));
            }
        }
        let expected_accounting = [
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 11,
                selected_block_coefficients: 45_056,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 2613,
                entropy_scratch_ceiling_bytes: 2_490_474,
                compact_coefficient_samples: 4364,
                synthesis_workspace_ceiling_samples: 5366,
                output_samples: 1505,
                deterministic_workspace_ceiling_bytes: 2_557_421,
            },
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 7,
                selected_block_coefficients: 28_672,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 2636,
                entropy_scratch_ceiling_bytes: 2_490_888,
                compact_coefficient_samples: 3632,
                synthesis_workspace_ceiling_samples: 7158,
                output_samples: 2193,
                deterministic_workspace_ceiling_bytes: 2_566_226,
            },
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 8,
                selected_block_coefficients: 32_768,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 2156,
                entropy_scratch_ceiling_bytes: 2_482_248,
                compact_coefficient_samples: 1292,
                synthesis_workspace_ceiling_samples: 1504,
                output_samples: 357,
                deterministic_workspace_ceiling_bytes: 2_514_114,
            },
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 4,
                selected_block_coefficients: 16_384,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 2133,
                entropy_scratch_ceiling_bytes: 2_481_834,
                compact_coefficient_samples: 1020,
                synthesis_workspace_ceiling_samples: 1868,
                output_samples: 525,
                deterministic_workspace_ceiling_bytes: 2_515_053,
            },
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 1,
                selected_block_coefficients: 4096,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 654,
                entropy_scratch_ceiling_bytes: 2_455_212,
                compact_coefficient_samples: 90,
                synthesis_workspace_ceiling_samples: 182,
                output_samples: 90,
                deterministic_workspace_ceiling_bytes: 2_473_878,
            },
            LossyHtSpatialRegionAccounting {
                total_code_blocks: 256,
                selected_code_blocks: 1,
                selected_block_coefficients: 4096,
                maximum_block_coefficients: 4096,
                maximum_segment_bytes: 653,
                entropy_scratch_ceiling_bytes: 2_455_194,
                compact_coefficient_samples: 120,
                synthesis_workspace_ceiling_samples: 242,
                output_samples: 120,
                deterministic_workspace_ceiling_bytes: 2_474_399,
            },
        ];
        assert_eq!(
            observed
                .iter()
                .map(|(_, _, accounting)| *accounting)
                .collect::<Vec<_>>(),
            expected_accounting
        );
        println!("LOSSY_HT_SPATIAL_ACCOUNTING={observed:#?}");
    }

    #[test]
    fn spatial_region_calibration_rejects_unchecked_or_malformed_geometry_and_accounting() {
        let source = crate::ht_lossy_test_support::source(64, 64, 16, 1, 0)
            .pop()
            .unwrap();
        let bytes = source
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect::<Vec<_>>();
        let raw = encode_planar(64, 64, 16, &[&bytes], &[128], 4.0).unwrap();
        for request in [
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 1,
                },
                discard_levels: 0,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 63,
                    y: 63,
                    width: 2,
                    height: 1,
                },
                discard_levels: 2,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: u32::MAX,
                    y: 0,
                    width: 2,
                    height: 1,
                },
                discard_levels: 2,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 1,
                    height: 1,
                },
                discard_levels: 3,
            },
        ] {
            assert!(prepare_lossy_ht_spatial_region_calibration(&raw, request).is_err());
        }
        let valid = prepare_lossy_ht_spatial_region_calibration(
            &raw,
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 5,
                    y: 7,
                    width: 9,
                    height: 11,
                },
                discard_levels: 1,
            },
        )
        .unwrap();
        assert!(
            lossy_ht_window_storage_accounting(
                &valid.synthesis,
                &valid.prepared.contributions,
                &[usize::MAX]
            )
            .is_err()
        );
    }

    fn encode_u16_grey(width: u32, height: u32, pattern: u32, rate: f32) -> Vec<u8> {
        let source = crate::ht_lossy_test_support::source(width, height, 16, 1, pattern)
            .pop()
            .unwrap();
        let bytes = source
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect::<Vec<_>>();
        encode_planar(
            width,
            height,
            16,
            &[bytes.as_slice()],
            &[width as usize * 2],
            rate,
        )
        .unwrap()
    }

    #[test]
    fn full_resolution_spatial_regions_match_full_image_across_geometry_matrix() {
        let (width, height) = (193_u32, 137_u32);
        let raw = encode_u16_grey(width, height, 0, 4.0);
        let established = established_component(&raw, 0);
        let regions = [
            TileRegionRequest {
                x: 0,
                y: 0,
                width,
                height,
            },
            TileRegionRequest {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 192,
                y: 0,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 0,
                y: 136,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 192,
                y: 136,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 31,
                y: 0,
                width: 67,
                height: 9,
            },
            TileRegionRequest {
                x: 45,
                y: 132,
                width: 33,
                height: 5,
            },
            TileRegionRequest {
                x: 0,
                y: 21,
                width: 7,
                height: 55,
            },
            TileRegionRequest {
                x: 188,
                y: 19,
                width: 5,
                height: 71,
            },
            TileRegionRequest {
                x: 97,
                y: 73,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 3,
                y: 63,
                width: 129,
                height: 1,
            },
            TileRegionRequest {
                x: 64,
                y: 3,
                width: 1,
                height: 129,
            },
            TileRegionRequest {
                x: 5,
                y: 7,
                width: 37,
                height: 29,
            },
            TileRegionRequest {
                x: 64,
                y: 64,
                width: 64,
                height: 64,
            },
            TileRegionRequest {
                x: 63,
                y: 63,
                width: 64,
                height: 64,
            },
            TileRegionRequest {
                x: 61,
                y: 62,
                width: 9,
                height: 7,
            },
            TileRegionRequest {
                x: 3,
                y: 4,
                width: 17,
                height: 19,
            },
            TileRegionRequest {
                x: 124,
                y: 125,
                width: 13,
                height: 11,
            },
        ];
        let mut workspace = LossyHtSpatialRegionWorkspace::new();
        for region in regions {
            let plan = prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region,
                    discard_levels: 0,
                },
            )
            .unwrap();
            let (actual, report) =
                decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
            assert_eq!(
                actual,
                crop_u16(&established.components[0].samples, width, region)
            );
            assert_eq!(report.work.output_samples, plan.accounting.output_samples);
            assert_eq!(plan.projected_region, region);
            assert_eq!(plan.retained_resolution, 2);
        }
    }

    #[test]
    fn full_resolution_spatial_regions_cover_representative_encoder_patterns() {
        let region = TileRegionRequest {
            x: 29,
            y: 17,
            width: 73,
            height: 61,
        };
        // Pattern 4 is deliberately absent: the established acceptance matrix
        // proves that high-contrast source is unattainable at every supported
        // target rate, so there is no successful encoder output to qualify.
        for (pattern, rate) in [(7, 1.0), (0, 4.0), (1, 4.0)] {
            let raw = encode_u16_grey(257, 193, pattern, rate);
            let established = established_component(&raw, 0);
            let plan = prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region,
                    discard_levels: 0,
                },
            )
            .unwrap();
            let (actual, _) = decode_prepared_lossy_ht_spatial_region(
                &plan,
                &mut LossyHtSpatialRegionWorkspace::new(),
            )
            .unwrap();
            assert_eq!(
                actual,
                crop_u16(&established.components[0].samples, 257, region),
                "pattern {pattern}"
            );
        }
    }

    #[test]
    fn discarded_spatial_regions_match_reduced_image_across_geometry_matrix() {
        let (width, height) = (193_u32, 137_u32);
        let raw = encode_u16_grey(width, height, 0, 4.0);
        let regions = [
            TileRegionRequest {
                x: 0,
                y: 0,
                width,
                height,
            },
            TileRegionRequest {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 192,
                y: 0,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 0,
                y: 136,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 192,
                y: 136,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 31,
                y: 0,
                width: 67,
                height: 9,
            },
            TileRegionRequest {
                x: 45,
                y: 132,
                width: 33,
                height: 5,
            },
            TileRegionRequest {
                x: 0,
                y: 21,
                width: 7,
                height: 55,
            },
            TileRegionRequest {
                x: 188,
                y: 19,
                width: 5,
                height: 71,
            },
            TileRegionRequest {
                x: 96,
                y: 72,
                width: 1,
                height: 1,
            },
            TileRegionRequest {
                x: 3,
                y: 64,
                width: 129,
                height: 1,
            },
            TileRegionRequest {
                x: 64,
                y: 3,
                width: 1,
                height: 129,
            },
            TileRegionRequest {
                x: 5,
                y: 7,
                width: 37,
                height: 29,
            },
            TileRegionRequest {
                x: 64,
                y: 64,
                width: 64,
                height: 64,
            },
            TileRegionRequest {
                x: 63,
                y: 63,
                width: 64,
                height: 64,
            },
            TileRegionRequest {
                x: 61,
                y: 62,
                width: 9,
                height: 7,
            },
            TileRegionRequest {
                x: 3,
                y: 4,
                width: 17,
                height: 19,
            },
            TileRegionRequest {
                x: 124,
                y: 125,
                width: 13,
                height: 11,
            },
        ];
        for discard_levels in [1, 2] {
            let established = established_component(&raw, discard_levels);
            let divisor = 1_u32 << discard_levels;
            let retained_resolution = 2 - discard_levels;
            let mut workspace = LossyHtSpatialRegionWorkspace::new();
            for region in regions {
                let plan = prepare_lossy_ht_spatial_region(
                    &raw,
                    LossyHtSpatialRegionRequest {
                        region,
                        discard_levels,
                    },
                )
                .unwrap_or_else(|error| panic!("discard {discard_levels}, {region:?}: {error:?}"));
                let projected_x = region.x.div_ceil(divisor);
                let projected_y = region.y.div_ceil(divisor);
                let projected_right = (region.x + region.width).div_ceil(divisor);
                let projected_bottom = (region.y + region.height).div_ceil(divisor);
                let projected = TileRegionRequest {
                    x: projected_x,
                    y: projected_y,
                    width: projected_right - projected_x,
                    height: projected_bottom - projected_y,
                };
                let (actual, report) =
                    decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
                assert_eq!(plan.projected_region, projected);
                assert_eq!(
                    actual.len(),
                    projected.width as usize * projected.height as usize * 2
                );
                assert_eq!(
                    actual,
                    crop_u16(
                        &established.components[0].samples,
                        established.width,
                        projected
                    )
                );
                assert_eq!(plan.retained_resolution, retained_resolution);
                assert_eq!(plan.synthesis.levels.len(), retained_resolution as usize);
                assert_eq!(report.work.output_samples, plan.accounting.output_samples);
                assert_eq!(
                    plan.accounting.output_samples,
                    u64::from(projected.width) * u64::from(projected.height)
                );
                let required_coefficient_samples = plan
                    .synthesis
                    .levels
                    .iter()
                    .flat_map(|level| [level.high_low, level.low_high, level.high_high])
                    .fold(
                        plan.synthesis.lowest_low_low.sample_count(),
                        |total, required| total + required.sample_count(),
                    );
                assert_eq!(
                    required_coefficient_samples,
                    plan.accounting.compact_coefficient_samples
                );
                assert_eq!(
                    plan.accounting.selected_code_blocks,
                    plan.selected_contribution_indices.len() as u64
                );
                assert!(plan.accounting.selected_code_blocks > 0);
                assert!(plan.accounting.selected_code_blocks <= plan.accounting.total_code_blocks);
                assert!(
                    plan.accounting.deterministic_workspace_ceiling_bytes
                        <= DEFAULT_LOSSY_HT_SPATIAL_WORKSPACE_LIMIT_BYTES
                );
                for &index in &plan.selected_contribution_indices {
                    let contribution = &plan.prepared.contributions[index];
                    assert!(contribution.resolution <= retained_resolution);
                    assert!(
                        synthesis_window_dependency_selects_contribution(
                            &plan.synthesis,
                            contribution
                        )
                        .unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn discarded_spatial_regions_cover_representative_encoder_patterns() {
        let region = TileRegionRequest {
            x: 29,
            y: 17,
            width: 73,
            height: 61,
        };
        for (discard_levels, projected) in [
            (
                1,
                TileRegionRequest {
                    x: 15,
                    y: 9,
                    width: 36,
                    height: 30,
                },
            ),
            (
                2,
                TileRegionRequest {
                    x: 8,
                    y: 5,
                    width: 18,
                    height: 15,
                },
            ),
        ] {
            for (pattern, rate) in [(7, 1.0), (0, 4.0), (1, 4.0)] {
                let raw = encode_u16_grey(257, 193, pattern, rate);
                let established = established_component(&raw, discard_levels);
                let plan = prepare_lossy_ht_spatial_region(
                    &raw,
                    LossyHtSpatialRegionRequest {
                        region,
                        discard_levels,
                    },
                )
                .unwrap();
                let (actual, _) = decode_prepared_lossy_ht_spatial_region(
                    &plan,
                    &mut LossyHtSpatialRegionWorkspace::new(),
                )
                .unwrap();
                assert_eq!(plan.projected_region, projected);
                assert_eq!(
                    actual,
                    crop_u16(
                        &established.components[0].samples,
                        established.width,
                        projected
                    ),
                    "discard {discard_levels}, pattern {pattern}"
                );
            }
        }
    }

    #[test]
    fn discard_two_large_geometry_retains_only_required_lowest_low_low() {
        let width = 1024_u32;
        let height = 1024_u32;
        let raw = encode_u16_grey(width, height, 1, 4.0);
        assert_eq!(
            format!("{:x}", Sha256::digest(&raw)),
            "f376f5c04b13c640fec6b80ba52bfb198cc1832d75f6850411ba801e035da597"
        );
        let established = established_component(&raw, 2);
        let requests = [
            (
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width,
                    height,
                },
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 256,
                    height: 256,
                },
                16_u64,
            ),
            (
                TileRegionRequest {
                    x: 256,
                    y: 256,
                    width: 256,
                    height: 256,
                },
                TileRegionRequest {
                    x: 64,
                    y: 64,
                    width: 64,
                    height: 64,
                },
                1_u64,
            ),
        ];
        let mut workspace = LossyHtSpatialRegionWorkspace::new();
        let mut observed = Vec::new();
        for (region, projected, selected_code_blocks) in requests {
            let plan = prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region,
                    discard_levels: 2,
                },
            )
            .unwrap();
            let (actual, report) =
                decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
            assert_eq!(plan.projected_region, projected);
            assert_eq!(plan.retained_resolution, 0);
            assert!(plan.synthesis.levels.is_empty());
            assert_eq!(
                plan.synthesis.lowest_low_low,
                AxisAlignedRegion {
                    x: projected.x,
                    y: projected.y,
                    width: projected.width,
                    height: projected.height,
                }
            );
            assert_eq!(plan.synthesis.output_region, plan.synthesis.lowest_low_low);
            assert_eq!(plan.accounting.selected_code_blocks, selected_code_blocks);
            assert_eq!(plan.accounting.total_code_blocks, 256);
            assert_eq!(
                plan.accounting.compact_coefficient_samples,
                u64::from(projected.width) * u64::from(projected.height)
            );
            assert_eq!(report.work.output_samples, plan.accounting.output_samples);
            assert_eq!(report.work.horizontal_values, 0);
            assert_eq!(report.work.vertical_values, 0);
            assert_eq!(report.work.lifting_updates, 0);
            assert_eq!(
                actual,
                crop_u16(
                    &established.components[0].samples,
                    established.width,
                    projected
                )
            );
            for &index in &plan.selected_contribution_indices {
                let contribution = &plan.prepared.contributions[index];
                assert_eq!(contribution.resolution, 0);
                assert_eq!(contribution.subband, PacketSubbandKind::LowLow);
                assert!(
                    synthesis_window_dependency_selects_contribution(&plan.synthesis, contribution)
                        .unwrap()
                );
            }
            observed.push(plan.accounting);
        }
        assert_eq!(
            observed,
            [
                LossyHtSpatialRegionAccounting {
                    total_code_blocks: 256,
                    selected_code_blocks: 16,
                    selected_block_coefficients: 65_536,
                    maximum_block_coefficients: 4096,
                    maximum_segment_bytes: 688,
                    entropy_scratch_ceiling_bytes: 2_455_824,
                    compact_coefficient_samples: 65_536,
                    synthesis_workspace_ceiling_samples: 131_074,
                    output_samples: 65_536,
                    deterministic_workspace_ceiling_bytes: 3_652_552,
                },
                LossyHtSpatialRegionAccounting {
                    total_code_blocks: 256,
                    selected_code_blocks: 1,
                    selected_block_coefficients: 4096,
                    maximum_block_coefficients: 4096,
                    maximum_segment_bytes: 653,
                    entropy_scratch_ceiling_bytes: 2_455_194,
                    compact_coefficient_samples: 4096,
                    synthesis_workspace_ceiling_samples: 8194,
                    output_samples: 4096,
                    deterministic_workspace_ceiling_bytes: 2_545_967,
                },
            ]
        );
        println!("LOSSY_HT_DISCARD_TWO_LARGE_ACCOUNTING={observed:#?}");
    }

    #[test]
    fn full_resolution_workspace_reuses_bounded_storage_and_enforces_limit() {
        let raw = encode_u16_grey(257, 193, 1, 4.0);
        let requests = [
            TileRegionRequest {
                x: 17,
                y: 19,
                width: 151,
                height: 113,
            },
            TileRegionRequest {
                x: 211,
                y: 167,
                width: 13,
                height: 11,
            },
            TileRegionRequest {
                x: 3,
                y: 5,
                width: 7,
                height: 9,
            },
        ];
        let mut workspace = LossyHtSpatialRegionWorkspace::new();
        let mut retained = None;
        for region in requests {
            let plan = prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region,
                    discard_levels: 0,
                },
            )
            .unwrap();
            decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
            let now = (
                workspace.compact_coefficient_retained_bytes(),
                workspace.synthesis_retained_bytes(),
            );
            if let Some(previous) = retained {
                assert_eq!(now, previous);
            }
            retained = Some(now);
        }

        let plan = prepare_lossy_ht_spatial_region(
            &raw,
            LossyHtSpatialRegionRequest {
                region: requests[1],
                discard_levels: 0,
            },
        )
        .unwrap();
        let duplicate = prepare_lossy_ht_spatial_region(
            &raw,
            LossyHtSpatialRegionRequest {
                region: requests[1],
                discard_levels: 0,
            },
        )
        .unwrap();
        assert_eq!(plan.synthesis, duplicate.synthesis);
        assert_eq!(
            plan.selected_contribution_indices,
            duplicate.selected_contribution_indices
        );
        assert_eq!(plan.accounting, duplicate.accounting);
        assert_eq!(plan.synthesis.levels.len(), 2);
        assert_eq!(
            plan.synthesis.output_region.sample_count(),
            plan.accounting.output_samples
        );
        let required_coefficient_samples = plan
            .synthesis
            .levels
            .iter()
            .flat_map(|level| [level.high_low, level.low_high, level.high_high])
            .fold(
                plan.synthesis.lowest_low_low.sample_count(),
                |total, region| total + region.sample_count(),
            );
        assert_eq!(
            required_coefficient_samples,
            plan.accounting.compact_coefficient_samples
        );
        assert_eq!(
            plan.accounting.selected_code_blocks,
            plan.selected_contribution_indices.len() as u64
        );
        assert!(plan.accounting.compact_coefficient_samples < 257 * 193);
        assert!(plan.accounting.output_samples < 257 * 193);
        let mut limited = LossyHtSpatialRegionWorkspace::with_maximum_bytes(
            plan.accounting.deterministic_workspace_ceiling_bytes - 1,
        );
        assert!(decode_prepared_lossy_ht_spatial_region(&plan, &mut limited).is_err());
        assert_eq!(limited.compact_coefficient_retained_bytes(), 0);
        assert_eq!(limited.synthesis_retained_bytes(), 0);
    }

    #[test]
    fn workspace_reuses_storage_across_full_and_both_discard_levels() {
        let raw = encode_u16_grey(257, 193, 1, 4.0);
        let requests = [
            (
                TileRegionRequest {
                    x: 17,
                    y: 19,
                    width: 151,
                    height: 113,
                },
                0,
            ),
            (
                TileRegionRequest {
                    x: 211,
                    y: 167,
                    width: 13,
                    height: 11,
                },
                1,
            ),
            (
                TileRegionRequest {
                    x: 3,
                    y: 5,
                    width: 7,
                    height: 9,
                },
                0,
            ),
            (
                TileRegionRequest {
                    x: 101,
                    y: 41,
                    width: 73,
                    height: 87,
                },
                1,
            ),
            (
                TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 257,
                    height: 193,
                },
                2,
            ),
            (
                TileRegionRequest {
                    x: 7,
                    y: 9,
                    width: 17,
                    height: 19,
                },
                2,
            ),
            (
                TileRegionRequest {
                    x: 173,
                    y: 117,
                    width: 63,
                    height: 51,
                },
                2,
            ),
        ];
        let mut workspace = LossyHtSpatialRegionWorkspace::new();
        let mut retained_ceiling = None;
        for (region, discard_levels) in requests {
            let established = established_component(&raw, discard_levels);
            let plan = prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region,
                    discard_levels,
                },
            )
            .unwrap();
            let (actual, _) =
                decode_prepared_lossy_ht_spatial_region(&plan, &mut workspace).unwrap();
            assert_eq!(
                actual,
                crop_u16(
                    &established.components[0].samples,
                    established.width,
                    plan.projected_region
                )
            );
            let now = (
                workspace.compact_coefficient_retained_bytes(),
                workspace.synthesis_retained_bytes(),
            );
            if let Some((coefficient_ceiling, synthesis_ceiling)) = retained_ceiling {
                assert!(now.0 <= coefficient_ceiling);
                assert!(now.1 <= synthesis_ceiling);
            } else {
                retained_ceiling = Some(now);
            }
        }
    }

    #[test]
    fn spatial_region_private_boundary_rejects_neighbours_and_corruption() {
        let raw = encode_u16_grey(64, 64, 0, 4.0);
        for request in [
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 9,
                    height: 7,
                },
                discard_levels: 3,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 9,
                    height: 7,
                },
                discard_levels: u8::MAX,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 1,
                },
                discard_levels: 0,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 63,
                    y: 63,
                    width: 2,
                    height: 1,
                },
                discard_levels: 2,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 1,
                    height: 1,
                },
                discard_levels: 2,
            },
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: u32::MAX,
                    y: 0,
                    width: 2,
                    height: 1,
                },
                discard_levels: 1,
            },
        ] {
            assert!(prepare_lossy_ht_spatial_region(&raw, request).is_err());
        }
        assert!(matches!(
            prepare_lossy_ht_spatial_region(
                &raw,
                LossyHtSpatialRegionRequest {
                    region: TileRegionRequest {
                        x: 1,
                        y: 1,
                        width: 9,
                        height: 7,
                    },
                    discard_levels: 3,
                },
            ),
            Err(CodestreamError::Unsupported {
                construct: UnsupportedConstruct::Transform,
                message,
                ..
            }) if message.contains("zero, one or two")
        ));
        assert!(
            prepare_lossy_ht_spatial_region(
                &raw[..raw.len() - 1],
                LossyHtSpatialRegionRequest {
                    region: TileRegionRequest {
                        x: 1,
                        y: 1,
                        width: 9,
                        height: 7
                    },
                    discard_levels: 2,
                },
            )
            .is_err()
        );

        let mut plan = prepare_lossy_ht_spatial_region(
            &raw,
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 9,
                    height: 7,
                },
                discard_levels: 2,
            },
        )
        .unwrap();
        let mut limited = LossyHtSpatialRegionWorkspace::with_maximum_bytes(
            plan.accounting.deterministic_workspace_ceiling_bytes - 1,
        );
        assert!(decode_prepared_lossy_ht_spatial_region(&plan, &mut limited).is_err());
        assert_eq!(limited.compact_coefficient_retained_bytes(), 0);
        assert_eq!(limited.synthesis_retained_bytes(), 0);

        let selected = plan.selected_contribution_indices[0];
        plan.prepared.contributions[selected].codeword_len = usize::MAX;
        assert!(
            decode_prepared_lossy_ht_spatial_region(
                &plan,
                &mut LossyHtSpatialRegionWorkspace::new()
            )
            .is_err()
        );

        let corruption_plan = prepare_lossy_ht_spatial_region(
            &raw,
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 9,
                    height: 7,
                },
                discard_levels: 2,
            },
        )
        .unwrap();
        let selected = &corruption_plan.prepared.contributions
            [corruption_plan.selected_contribution_indices[0]];
        let (_, payload) = single_part1_profile_tile(
            corruption_plan.prepared.input,
            &corruption_plan.prepared.codestream,
        )
        .unwrap();
        let payload_start = payload.as_ptr() as usize - raw.as_ptr() as usize;
        let selected_start = payload_start + selected.payload_offset;
        let selected_end = selected_start + selected.codeword_len;
        drop(corruption_plan);
        let mut corrupt = raw.clone();
        corrupt[selected_start..selected_end].fill(0);
        let corrupt_plan = prepare_lossy_ht_spatial_region(
            &corrupt,
            LossyHtSpatialRegionRequest {
                region: TileRegionRequest {
                    x: 1,
                    y: 1,
                    width: 9,
                    height: 7,
                },
                discard_levels: 2,
            },
        )
        .unwrap();
        assert!(
            decode_prepared_lossy_ht_spatial_region(
                &corrupt_plan,
                &mut LossyHtSpatialRegionWorkspace::new()
            )
            .is_err()
        );
    }

    #[test]
    fn large_noise_packets_exceed_capacity_hint_without_changing_calibrated_bytes() {
        use sha2::{Digest, Sha256};

        let source = crate::ht_lossy_test_support::source(1024, 1024, 16, 3, 1);
        let mut planes = source
            .iter()
            .map(|plane| {
                plane
                    .iter()
                    .map(|&sample| f32::from(sample) - 32768.0)
                    .collect()
            })
            .collect::<Vec<Vec<f32>>>();
        analyse(1024, 1024, &mut planes).unwrap();
        let specs = decomp_subband_specs(1024, 1024, 2).unwrap();
        let mut quantized = vec![vec![0; 1024 * 1024]; 3];
        // Retained 2/4-bpp candidates exercise the actual large-shape writer
        // without repeating all seventeen search visits in the ordinary suite.
        for (coarseness, expected_header_bytes, expected_packet_bytes, expected_hash) in [
            (
                61689,
                1557,
                261857,
                "dfd296ef4947f06c5cd495de7438c557e04e8e541edd7d4f7ec1fb0218132c72",
            ),
            (
                60344,
                1950,
                524171,
                "728142639df1c29863df3a450b133fc86c3aea587f992f38a99079bc70c5ce6b",
            ),
        ] {
            let raw = candidate(1024, 1024, 16, &planes, &specs, coarseness, &mut quantized)
                .unwrap()
                .unwrap();
            let parsed = parse(&raw).unwrap();
            let (rect, payload) = single_part1_profile_tile(&raw, &parsed).unwrap();
            let contributions =
                parse_default_precinct_lrcp_packets(&raw, &parsed, rect, payload).unwrap();
            let body_bytes = contributions
                .iter()
                .map(|block| block.codeword_len)
                .sum::<usize>();
            let header_bytes = payload.len() - body_bytes;
            assert_eq!(header_bytes, expected_header_bytes);
            assert_eq!(payload.len(), expected_packet_bytes);
            // Seven subbands per component supplied only 21 * 64 header bytes
            // in the original capacity hint, so both streams require growth.
            assert!(header_bytes > 3 * 7 * 64);
            assert_eq!(format!("{:x}", Sha256::digest(&raw)), expected_hash);
            assert!(is_profile(&raw, &parsed));
        }
    }

    #[test]
    fn resource_rate_and_plane_preflight_precedes_working_allocation() {
        for rate in [
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            0.0,
            -1.0,
            f32::MIN_POSITIVE,
            f32::MAX,
        ] {
            assert!(byte_budget(257 * 193, rate).is_err());
            assert!(encode_planar(257, 193, 8, &[&[]], &[257], rate).is_err());
        }
        for (pixels, rate) in [(16, 0.5), (497, 1.1), (257 * 193, 1.001), (1_048_576, 4.0)] {
            assert_eq!(
                byte_budget(pixels, rate).unwrap(),
                ((f64::from(rate) * pixels as f64).floor() / 8.0).floor() as usize
            );
        }
        for (w, h) in [(4, 4), (8192, 128), (128, 8192), (1024, 1024)] {
            assert!(dimensions(w, h, 3).is_ok());
        }
        for (w, h) in [
            (3, 4),
            (4, 3),
            (8193, 4),
            (4, 8193),
            (8192, 129),
            (1025, 1024),
            (u32::MAX, u32::MAX),
        ] {
            assert!(encode_planar(w, h, 8, &[&[]], &[usize::MAX], 2.0).is_err());
        }
        for count in [0, 2, 4] {
            assert!(dimensions(4, 4, count).is_err());
        }
        for bits in [0, 1, 7, 9, 15, 17, 32] {
            assert!(encode_planar(4, 4, bits, &[&[]], &[4], 2.0).is_err());
        }
        for (plane, stride) in [
            (&[0_u8; 64][..], 3),
            (&[0_u8; 11][..], 4),
            (&[0_u8; 64][..], usize::MAX),
        ] {
            assert!(encode_planar(4, 4, 8, &[plane], &[stride], 16.0).is_err());
        }
        assert!(encode_planar(4, 4, 8, &[&[0; 16]], &[], 16.0).is_err());
        // Constant and minimum-size requests must not become padded successes.
        assert!(encode_planar(4, 4, 8, &[&[0; 16]], &[4], 4.0).is_err());
        assert!(encode_planar(257, 193, 8, &[&vec![128; 257 * 193]], &[257], 1.0).is_err());
    }

    #[test]
    fn exact_profile_rejects_marker_precision_grid_and_quantisation_neighbours() {
        let raw = encoded();
        let parsed = parse(&raw).unwrap();
        assert!(is_profile(&raw, &parsed));
        assert!(!is_htj2k_lossless_profile(&raw, &parsed));
        let cod = find_marker(&raw, 0, Marker::Cod).unwrap();
        let cap = find_marker(&raw, 0, Marker::Cap).unwrap();
        let qcd = find_marker(&raw, 0, Marker::Qcd).unwrap();
        let sot = find_marker(&raw, 0, Marker::Sot).unwrap();
        for (offset, value) in [
            (cod + 4, 2),
            (cod + 4, 4),
            (cod + 5, 1),
            (cod + 7, 2),
            (cod + 8, 1),
            (cod + 9, 1),
            (cod + 9, 3),
            (cod + 10, 3),
            (cod + 11, 3),
            (cod + 12, 0xc0),
            (cod + 13, 1),
            (cap + 8, 0x40),
            (cap + 9, 0x6a),
            (cap + 9, 0x2b),
            (qcd + 4, 0x42),
            (qcd + 4, 0x61),
            (qcd + 4, 0x60),
            (42, 6),
            (42, 8),
            (42, 0x87),
            (43, 2),
            (44, 2),
            (45, 15),
            (sot + 10, 1),
            (sot + 11, 2),
        ] {
            let mut bytes = raw.clone();
            bytes[offset] = value;
            rejects(&bytes);
        }
        // Unit origins and one exact tile are part of admission, even where legal.
        for (offset, value) in [
            (16, 1_u32),
            (20, 1),
            (24, 128),
            (28, 128),
            (32, 1),
            (36, 1),
            (8, 8193),
            (12, 8193),
            (8, 3),
            (8, 8192),
            (12, 8192),
        ] {
            let mut bytes = raw.clone();
            bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
            rejects(&bytes);
        }
        // Only the selected equal absolute-step family is accepted. A valid
        // but differently quantised orientation must not use this full route.
        let mut bytes = raw.clone();
        bytes[qcd + 8] ^= 1;
        rejects(&bytes);
        let mut bytes = raw.clone();
        for (index, gain) in [0_u16, 1, 1, 2, 1, 1, 2].into_iter().enumerate() {
            let value = ((27 + gain) << 11).to_be_bytes();
            bytes[qcd + 5 + index * 2..qcd + 7 + index * 2].copy_from_slice(&value);
        }
        rejects(&bytes); // HH would require 31 magnitude planes.
        for marker in [
            vec![0xff, 0x64, 0, 4, 0, 1],
            vec![0xff, 0x5e, 0, 5, 0, 0, 1],
            vec![0xff, 0x53, 0, 9, 0, 0, 2, 4, 4, 0x40, 0],
        ] {
            let mut bytes = raw.clone();
            bytes.splice(sot..sot, marker);
            rejects(&bytes);
        }
        // Main QCC and a tile QCD are rejected even if equivalent to default.
        let mut qcc = vec![0xff, 0x5d, 0, 18, 0];
        qcc.extend_from_slice(&raw[qcd + 4..qcd + 19]);
        let mut bytes = raw.clone();
        bytes.splice(sot..sot, qcc);
        rejects(&bytes);
        let mut bytes = raw.clone();
        let tile_qcd = raw[qcd..qcd + 19].to_vec();
        bytes.splice(sot + 12..sot + 12, tile_qcd);
        let length = u32::from_be_bytes(raw[sot + 6..sot + 10].try_into().unwrap()) + 19;
        bytes[sot + 6..sot + 10].copy_from_slice(&length.to_be_bytes());
        rejects(&bytes);
        for removed in [1, 2, 3, raw.len() / 2] {
            rejects(&raw[..raw.len() - removed]);
        }
        let mut oversized = raw.clone();
        oversized.resize(MAX_CODESTREAM_BYTES + 1, 0);
        assert!(preflight_packet_resources(&oversized, &parsed).is_err());
        assert!(!is_profile(&oversized, &parsed));
    }

    #[test]
    fn candidate_visits_and_coefficient_precision_limits_are_bounded() {
        let specs = decomp_subband_specs(4, 4, 2).unwrap();
        let mut quantized = vec![vec![0; 16]];
        let huge = vec![vec![f32::MAX; 16]];
        assert!(
            candidate(4, 4, 16, &huge, &specs, 4096, &mut quantized)
                .unwrap()
                .is_none()
        );
        let source = vec![vec![131072.0; 16]];
        // index 15*2048 resolves a unit absolute step for unsigned U16.
        assert!(
            candidate(4, 4, 16, &source, &specs, 15 * 2048, &mut quantized)
                .unwrap()
                .is_none()
        );
        let source = vec![vec![131071.0; 16]];
        assert!(
            candidate(4, 4, 16, &source, &specs, 15 * 2048, &mut quantized)
                .unwrap()
                .is_some()
        );
        let source = vec![vec![0.0; 16]];
        for budget in [1, 32, 128, 1024, usize::MAX] {
            let (_, _, visits) = search(4, 4, 8, &source, budget).unwrap();
            assert!(visits <= 17);
        }
    }
    #[test]
    fn full_decode_checks_actual_coefficient_magnitude_before_reconstruction() {
        let specs = decomp_subband_specs(4, 4, 2).unwrap();
        let steps = [0_u8, 1, 1, 2, 1, 1, 2]
            .map(|gain| transform::IrreversibleQuantizationStep::new(16 + gain, 0).unwrap());
        for magnitude in [131071, 131072, -131072] {
            let plane = vec![magnitude; 16];
            let mut segments = Vec::new();
            let subbands = specs
                .iter()
                .zip(steps)
                .map(|(spec, step)| {
                    encode_ht_decomp_subband(4, &plane, *spec, step.exponent + 2, &mut segments)
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let mut packet = Vec::new();
            write_native_decomp_packets(&mut packet, 2, &[subbands], &segments).unwrap();
            let mut raw = Vec::new();
            write_irreversible_main_header(&mut raw, 4, 4, 16, 1, false, 2, &steps, true).unwrap();
            write_tile_part(&mut raw, 0, &packet, true).unwrap();
            assert!(is_profile(&raw, &parse(&raw).unwrap()));
            let result = decode_owned_with_workspace(&raw, &mut HtCodestreamDecodeWorkspace::new());
            if magnitude == 131071 {
                assert!(result.unwrap().is_some());
            } else {
                assert!(result.is_err());
            }
        }
    }
}
