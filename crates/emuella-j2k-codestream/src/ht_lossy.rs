//! Implementation-facing bounded irreversible HT orchestration.
//! The high-level public encoder API remains lossless. This module owns the
//! selected two-level target-rate contract documented in ht-lossy-calibration.md.
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
    let budget = byte_budget(pixels, bits_per_pixel)?;
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

    fn encoded() -> Vec<u8> {
        let source = super::super::ht_lossy_calibration::source(257, 193, 8, 3, 0);
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
