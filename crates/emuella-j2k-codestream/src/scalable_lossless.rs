//! Resource admission for the project-authored single-tile classic D2 writer.
use super::*;

/// Byte limits for owned raw classic lossless D2 encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LosslessEncodeLimits {
    /// Additional requested live allocation allowance, excluding caller input
    /// and the retained output allocation. Includes conservative realloc overlap.
    pub max_working_bytes: u64,
    /// Maximum capacity of the returned codestream allocation.
    pub max_output_bytes: u64,
}

impl Default for LosslessEncodeLimits {
    fn default() -> Self {
        Self {
            max_working_bytes: 4 * 1024 * 1024 * 1024,
            max_output_bytes: 1024 * 1024 * 1024,
        }
    }
}

/// Checked conservative admission accounting, not measured RSS or a reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LosslessEncodeRequirements {
    /// Total samples across all components (width × height × components).
    pub total_component_samples: u64,
    pub code_blocks: u64,
    pub working_bytes: u64,
    pub output_capacity_limit: u64,
}

/// Borrowed native component with an explicit byte step between samples.
#[derive(Clone, Copy)]
pub struct LosslessD2Plane<'a> {
    pub samples: &'a [u8],
    pub stride_bytes: usize,
    pub sample_step_bytes: usize,
}

/// Geometry-only admission for raw D2 grey/RGB or eight native components.
/// Eight-component encoding requires U16 at the sample-bearing entry point.
pub fn lossless_d2_requirements(
    width: u32,
    height: u32,
    components: u16,
    limits: LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    let pixels = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(CodestreamError::SizeOverflow)?;
    if width < 4
        || height < 4
        || width > 32768
        || height > 32768
        || pixels > 64 * 1024 * 1024
        || !matches!(components, 1 | 3 | 8)
        || (components == 8 && pixels > 32 * 1024 * 1024)
    {
        return Err(resource_error(
            "lossless D2 requires 4..=32768 axes, at most 64 Mi pixels for grey/RGB or 32 Mi pixels for eight components",
        ));
    }
    if limits.max_output_bytes < 128 {
        return Err(resource_error(
            "lossless D2 output-capacity budget must allow at least 128 bytes",
        ));
    }
    if limits.max_output_bytes > u64::from(u32::MAX) || limits.max_output_bytes > isize::MAX as u64
    {
        return Err(CodestreamError::SizeOverflow);
    }
    let blocks = decomp_subband_specs(width, height, 2)?
        .iter()
        .try_fold(0_u64, |sum, band| {
            sum.checked_add(
                u64::from(band.width)
                    .div_ceil(64)
                    .checked_mul(u64::from(band.height).div_ceil(64))
                    .ok_or(CodestreamError::SizeOverflow)?,
            )
            .ok_or(CodestreamError::SizeOverflow)
        })?
        .checked_mul(u64::from(components))
        .ok_or(CodestreamError::SizeOverflow)?;
    let samples = pixels
        .checked_mul(u64::from(components))
        .ok_or(CodestreamError::SizeOverflow)?;
    // See docs/scalable-lossless.md: planes, descriptors/trees/header growth,
    // DWT lines, output reallocation overlap, and bounded Tier-1/local storage.
    let working = samples
        .checked_mul(4)
        .and_then(|n| n.checked_add(blocks.checked_mul(4096)?))
        .and_then(|n| n.checked_add(u64::from(width.max(height)).checked_mul(12)?))
        .and_then(|n| n.checked_add(limits.max_output_bytes.checked_mul(2)?))
        .and_then(|n| n.checked_add(4 * 1024 * 1024))
        .ok_or(CodestreamError::SizeOverflow)?;
    if working > limits.max_working_bytes {
        return Err(resource_error(
            "lossless D2 working-memory budget is insufficient",
        ));
    }
    Ok(LosslessEncodeRequirements {
        total_component_samples: samples,
        code_blocks: blocks,
        working_bytes: working,
        output_capacity_limit: limits.max_output_bytes,
    })
}

fn resource_error(message: &'static str) -> CodestreamError {
    unsupported(
        None,
        Some(Marker::Siz),
        UnsupportedConstruct::PacketDecode,
        message,
    )
}

pub(super) fn reserve_output(
    output: &mut Vec<u8>,
    additional: usize,
    maximum: usize,
) -> Result<()> {
    let needed = output
        .len()
        .checked_add(additional)
        .ok_or(CodestreamError::SizeOverflow)?;
    if needed > maximum {
        return Err(resource_error(
            "lossless D2 output-capacity budget is insufficient",
        ));
    }
    if needed > output.capacity() {
        let target = needed.max(output.capacity().saturating_mul(2).min(maximum));
        output
            .try_reserve_exact(target - output.len())
            .map_err(|_| CodestreamError::SizeOverflow)?;
        // Capacity is part of the contract, even on an allocator/library that
        // chooses a larger capacity for an exact request.
        if output.capacity() > maximum {
            return Err(resource_error(
                "lossless D2 allocation exceeds output-capacity budget",
            ));
        }
    }
    Ok(())
}

/// Encode the bounded profile without packed input or complete packet copies.
/// Eight unsigned U16 components are coded independently without MCT.
pub fn encode_lossless_d2(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    lossless_d2_requirements(
        width,
        height,
        u16::try_from(planes.len()).map_err(|_| CodestreamError::SizeOverflow)?,
        limits,
    )?;
    if !matches!(bits, 8 | 16) || (planes.len() == 8 && bits != 16) {
        return Err(resource_error(
            "lossless D2 requires unsigned 8-bit or 16-bit input",
        ));
    }
    let bytes = usize::from(bits / 8);
    let width_usize = width as usize;
    let count = width_usize
        .checked_mul(height as usize)
        .ok_or(CodestreamError::SizeOverflow)?;
    // Validate every borrowed component before allocating coefficient storage.
    for plane in planes {
        let row = (width_usize - 1)
            .checked_mul(plane.sample_step_bytes)
            .and_then(|n| n.checked_add(bytes))
            .ok_or(CodestreamError::SizeOverflow)?;
        let required = (height as usize - 1)
            .checked_mul(plane.stride_bytes)
            .and_then(|n| n.checked_add(row))
            .ok_or(CodestreamError::SizeOverflow)?;
        if plane.sample_step_bytes < bytes
            || plane.stride_bytes < row
            || plane.samples.len() < required
        {
            return Err(resource_error(
                "lossless D2 component storage is too short or has an invalid stride",
            ));
        }
    }
    let mut coefficients = Vec::with_capacity(planes.len());
    for plane in planes {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if values.capacity() != count {
            return Err(CodestreamError::SizeOverflow);
        }
        for y in 0..height as usize {
            for x in 0..width_usize {
                let offset = y * plane.stride_bytes + x * plane.sample_step_bytes;
                let sample = if bits == 8 {
                    i32::from(plane.samples[offset])
                } else {
                    i32::from(u16::from_le_bytes([
                        plane.samples[offset],
                        plane.samples[offset + 1],
                    ]))
                };
                values.push(sample - (1_i32 << (bits - 1)));
            }
        }
        coefficients.push(values);
    }
    if let [red, green, blue] = coefficients.as_mut_slice() {
        transform::forward_reversible_color_transform_bounded(red, green, blue)
            .map_err(|_| CodestreamError::SizeOverflow)?;
    }
    let mut transform_scratch = Vec::new();
    for plane in &mut coefficients {
        forward_reversible_5_3_levels_with_scratch(
            width,
            height,
            plane,
            2,
            "lossless D2 transform failed",
            &mut transform_scratch,
        )?;
    }
    drop(transform_scratch);
    let specs = decomp_subband_specs(width, height, 2)?;
    let refs = coefficients.iter().map(Vec::as_slice).collect::<Vec<_>>();
    let exponents = specs
        .iter()
        .map(|spec| max_component_subband_available_bitplanes(width, &refs, *spec))
        .collect::<Result<Vec<_>>>()?;
    let maximum =
        usize::try_from(limits.max_output_bytes).map_err(|_| CodestreamError::SizeOverflow)?;
    let mut output = Vec::new();
    reserve_output(&mut output, 128, maximum)?;
    write_native_part1_main_header(
        &mut output,
        width,
        height,
        width,
        height,
        bits,
        planes.len() as u16,
        planes.len() == 3,
        2,
        &exponents,
    )?;
    let sot = output.len();
    output.extend_from_slice(&[0xff, 0x90, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1, 0xff, 0x93]);
    let mut scratch = tier1::CodeBlockEncodeScratch::new();
    for resolution in 0..=2 {
        for plane in &coefficients {
            let body_start = output.len();
            let mut bands = Vec::with_capacity(3);
            for (spec, exponent) in specs
                .iter()
                .zip(&exponents)
                .filter(|(spec, _)| spec.resolution == resolution)
            {
                bands.push(encode_decomp_subband_with_output_limit(
                    width,
                    plane,
                    *spec,
                    *exponent,
                    &mut output,
                    &mut scratch,
                    Some(maximum),
                )?);
            }
            let mut header = PacketBitWriter::new();
            let present = bands
                .iter()
                .any(|band| band.code_blocks.iter().any(|block| block.included));
            header.write_bit(u32::from(present))?;
            if present {
                for band in &bands {
                    write_component_packet_header(
                        &mut header,
                        band.code_block_cols,
                        band.code_block_rows,
                        &band.code_blocks,
                    )?;
                }
            }
            header.align();
            let header = header.bytes();
            reserve_output(&mut output, header.len(), maximum)?;
            let old_len = output.len();
            output.resize(old_len + header.len(), 0);
            output.copy_within(body_start..old_len, body_start + header.len());
            output[body_start..body_start + header.len()].copy_from_slice(header);
        }
    }
    let tile_len = u32::try_from(output.len() - sot).map_err(|_| CodestreamError::SizeOverflow)?;
    output[sot + 6..sot + 10].copy_from_slice(&tile_len.to_be_bytes());
    reserve_output(&mut output, 2, maximum)?;
    output.extend_from_slice(&[0xff, 0xd9]);
    Ok(output)
}
