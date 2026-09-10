//! Authored selective-bypass feasibility bridge; not a public encode profile.
use super::*;

/// Exercise native D2 packet assembly with actual selectively bypassed segments.
/// This serial exploration entry point is not a qualified resource/API contract.
#[cfg(feature = "test-fixtures")]
#[doc(hidden)]
pub fn encode_lossless_d2_bypass_test_fixture(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    encode_lossless_d2_impl::<false, true>(
        width,
        height,
        bits,
        planes,
        limits,
        &mut LosslessEncodeTimings::default(),
        &mut LosslessEncodeExecution::default(),
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn encode_subband(
    image_width: u32,
    plane: &[i32],
    spec: DecompSubbandSpec,
    available_bitplanes: u8,
    output: &mut Vec<u8>,
    maximum: usize,
    scratch: &mut tier1::CodeBlockEncodeScratch,
) -> Result<(NativeDecompSubband, Vec<Vec<usize>>)> {
    if spec.grid_x0 != 0 || spec.grid_y0 != 0 {
        return Err(CodestreamError::SizeOverflow);
    }
    let cols = u16::try_from(spec.width.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
    let rows =
        u16::try_from(spec.height.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
    let count = usize::from(cols) * usize::from(rows);
    let mut blocks = Vec::with_capacity(count);
    let mut all_lengths = Vec::with_capacity(count);
    let mut bytes = Vec::new();
    for y in 0..rows {
        for x in 0..cols {
            let local_x = u32::from(x) * 64;
            let local_y = u32::from(y) * 64;
            let width = (spec.width - local_x).min(64) as u16;
            let height = (spec.height - local_y).min(64) as u16;
            let offset = (spec.y as usize + local_y as usize)
                .checked_mul(image_width as usize)
                .and_then(|n| n.checked_add(spec.x as usize + local_x as usize))
                .ok_or(CodestreamError::SizeOverflow)?;
            let source = plane.get(offset..).ok_or(CodestreamError::SizeOverflow)?;
            let mut lengths = Vec::new();
            bytes.clear();
            let encoded = tier1::encode_baseline_code_block_segments_with_strided_scratch(
                source,
                image_width as usize,
                tier1::CodeBlockEncodeSpec {
                    dimensions: tier1::CodeBlockDimensions::new(width, height)
                        .map_err(map_tier1_error)?,
                    subband: spec.kind.tier1_subband(),
                    available_bitplanes,
                    code_block_style: tier1::CodeBlockStyle::SELECTIVE_ARITHMETIC_BYPASS,
                },
                &mut bytes,
                &mut lengths,
                scratch,
            )
            .map_err(map_tier1_error)?;
            validate_lengths(encoded.pass_count, encoded.byte_len, &lengths)?;
            if bytes.last() == Some(&0xff) {
                return Err(CodestreamError::SizeOverflow);
            }
            reserve_output(output, bytes.len(), maximum)?;
            blocks.push(EncodedCodeBlock {
                x,
                y,
                width,
                height,
                included: encoded.included,
                missing_bitplanes: encoded
                    .missing_bitplanes
                    .checked_add(1)
                    .ok_or(CodestreamError::SizeOverflow)?,
                coding_passes: encoded.pass_count,
                segment_offset: output.len(),
                segment_len: encoded.byte_len,
            });
            output.extend_from_slice(&bytes);
            all_lengths.push(lengths);
        }
    }
    Ok((
        NativeDecompSubband {
            index: spec.index,
            resolution: spec.resolution,
            kind: spec.kind,
            x: spec.x,
            y: spec.y,
            width: spec.width,
            height: spec.height,
            code_block_cols: cols,
            code_block_rows: rows,
            available_bitplanes,
            code_blocks: blocks,
        },
        all_lengths,
    ))
}

fn validate_lengths(passes: u16, byte_len: usize, lengths: &[usize]) -> Result<()> {
    let mut pass = 0;
    let mut total = 0usize;
    for &length in lengths {
        if pass >= passes {
            return Err(CodestreamError::SizeOverflow);
        }
        pass += bypass_segment_pass_capacity(pass).min(passes - pass);
        total = total
            .checked_add(length)
            .ok_or(CodestreamError::SizeOverflow)?;
    }
    if pass != passes || total != byte_len || passes > 164 {
        return Err(CodestreamError::SizeOverflow);
    }
    Ok(())
}

pub(super) fn write_packet_header(
    writer: &mut PacketBitWriter,
    band: &NativeDecompSubband,
    all_lengths: &[Vec<usize>],
) -> Result<()> {
    if band.code_blocks.len() != all_lengths.len() {
        return Err(CodestreamError::SizeOverflow);
    }
    let mut inclusion = EncTagTree::new(
        band.code_block_cols,
        band.code_block_rows,
        band.code_blocks
            .iter()
            .map(|b| if b.included { 0 } else { 1 }),
    )?;
    let mut missing = EncTagTree::new(
        band.code_block_cols,
        band.code_block_rows,
        band.code_blocks
            .iter()
            .map(|b| u32::from(b.missing_bitplanes)),
    )?;
    for (block, lengths) in band.code_blocks.iter().zip(all_lengths) {
        inclusion.encode(writer, block.x, block.y, 1)?;
        if !block.included {
            continue;
        }
        missing.encode(writer, block.x, block.y, u32::MAX)?;
        write_coding_pass_count(writer, block.coding_passes)?;
        write_lengths(writer, block.coding_passes, block.segment_len, lengths)?;
    }
    Ok(())
}

fn write_lengths(
    writer: &mut PacketBitWriter,
    passes: u16,
    bytes: usize,
    lengths: &[usize],
) -> Result<()> {
    validate_lengths(passes, bytes, lengths)?;
    let mut pass = 0;
    let mut lblock = 3u8;
    for &length in lengths {
        let count = bypass_segment_pass_capacity(pass).min(passes - pass);
        let pass_bits = count.ilog2() as u8;
        let length_bits = (usize::BITS - length.leading_zeros()) as u8;
        lblock = lblock.max(length_bits.saturating_sub(pass_bits));
        pass += count;
    }
    for _ in 3..lblock {
        writer.write_bit(1)?;
    }
    writer.write_bit(0)?;
    pass = 0;
    for &length in lengths {
        let count = bypass_segment_pass_capacity(pass).min(passes - pass);
        let bits = lblock
            .checked_add(count.ilog2() as u8)
            .ok_or(CodestreamError::SizeOverflow)?;
        writer.write_bits(
            u32::try_from(length).map_err(|_| CodestreamError::SizeOverflow)?,
            bits,
        )?;
        pass += count;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selective_bypass_malformed_observations() {
        let plane: Vec<_> = (0..4096)
            .map(|i| if i % 2 == 0 { 31 } else { -17 })
            .collect();
        let spec = DecompSubbandSpec {
            index: 0,
            resolution: 0,
            kind: PacketSubbandKind::LowLow,
            x: 0,
            y: 0,
            grid_x0: 0,
            grid_y0: 0,
            width: 64,
            height: 64,
        };
        let mut bytes = Vec::new();
        let (band, lengths) = encode_subband(
            64,
            &plane,
            spec,
            5,
            &mut bytes,
            1 << 20,
            &mut tier1::CodeBlockEncodeScratch::new(),
        )
        .unwrap();
        let block = &band.code_blocks[0];
        let decode_spec = tier1::CodeBlockDecodeSpec {
            dimensions: tier1::CodeBlockDimensions::new(64, 64).unwrap(),
            available_bitplanes: 5,
            missing_most_significant_bitplanes: block.missing_bitplanes - 1,
            coding_passes: block.coding_passes,
            style: tier1::CodeBlockStyle::from_bits(1),
            subband: tier1::Subband::LowLow,
        };
        let segments: Vec<_> = lengths[0]
            .iter()
            .zip([10, 2, 1])
            .map(|(&byte_len, coding_passes)| tier1::CodeBlockSegment {
                byte_len,
                coding_passes,
            })
            .collect();
        let observe = |name, input: &[u8], descriptors: &[tier1::CodeBlockSegment]| {
            let mut decoded = vec![0; 4096];
            let outcome = tier1::decode_baseline_code_block_segments(
                input,
                descriptors,
                decode_spec,
                &mut decoded,
            );
            println!(
                "bypass observation {name}: rejected={}, exact={}",
                outcome.is_err(),
                outcome.is_ok() && decoded == plane
            );
            (outcome.is_ok(), decoded == plane)
        };
        assert_eq!(observe("valid", &bytes, &segments), (true, true));
        let raw_start = lengths[0][0];
        let raw_end = raw_start + lengths[0][1];
        assert!(raw_end - raw_start >= 2);
        let evidence =
            std::env::var_os("EMUELLA_BYPASS_PROBE_OUTPUT").map(std::path::PathBuf::from);
        if let Some(root) = &evidence {
            std::fs::create_dir_all(root).unwrap();
            std::fs::write(root.join("valid-block.bin"), &bytes).unwrap();
            std::fs::write(root.join("partition.txt"), format!(
                "Authored 64x64 LL coefficients alternating 31,-17; available planes5; missing0; style1; passes13\nMQ passes0..10 bytes0..{raw_start}\nraw passes10..12 bytes{raw_start}..{raw_end}\nMQ cleanup pass12 bytes{raw_end}..{}\nlengths={:?}\n", bytes.len(), lengths[0]
            )).unwrap();
        }
        let mut changed = bytes.clone();
        changed[raw_start..raw_start + 2].copy_from_slice(&[0xff, 0x80]);
        if let Some(root) = &evidence {
            std::fs::write(root.join("raw-ff80.bin"), &changed).unwrap();
        }
        observe("raw_nonzero_stuffed_msb", &changed, &segments);
        changed.clone_from(&bytes);
        changed[raw_end - 1] = 0xff;
        if let Some(root) = &evidence {
            std::fs::write(root.join("raw-terminal-ff.bin"), &changed).unwrap();
        }
        observe("completed_raw_ends_ff", &changed, &segments);
        let mut shifted = segments.clone();
        shifted[0].byte_len += 1;
        shifted[1].byte_len -= 1;
        observe("boundary_shift_one", &bytes, &shifted);
        observe("length_truncation", &bytes[..bytes.len() - 1], &segments);
        let mut inflated = segments.clone();
        inflated[1].byte_len += 1;
        observe("length_inflation", &bytes, &inflated);
        let mut overflow = PacketBitReader::new(&[0; 64]);
        println!(
            "bypass observation lblock_u8_overflow: rejected={}",
            read_classic_codeword_lengths(&mut overflow, 255, 0, 13, 1).is_err()
        );
        let mut wide = PacketBitReader::new(&[0; 64]);
        println!(
            "bypass observation length_field_above_u32: rejected={}",
            read_classic_codeword_lengths(&mut wide, 33, 0, 13, 1).is_err()
        );
        let mut stuffing = PacketBitReader::new(&[0xff, 0x80]);
        println!(
            "bypass observation header_nonzero_stuffing: rejected={}",
            stuffing.read_bits_with_stuffing(9).is_err()
        );
    }

    #[test]
    fn selective_bypass_actual_five_and_six_plane_segments() {
        for (magnitude, passes, groups) in
            [(16, 13, vec![10, 2, 1]), (32, 16, vec![10, 2, 1, 2, 1])]
        {
            let plane: Vec<_> = (0..64 * 64)
                .map(|i| {
                    if i % 2 == 0 {
                        magnitude
                    } else {
                        -magnitude + 1
                    }
                })
                .collect();
            let spec = DecompSubbandSpec {
                index: 0,
                resolution: 0,
                kind: PacketSubbandKind::LowLow,
                x: 0,
                y: 0,
                grid_x0: 0,
                grid_y0: 0,
                width: 64,
                height: 64,
            };
            let mut output = Vec::new();
            let (band, lengths) = encode_subband(
                64,
                &plane,
                spec,
                6,
                &mut output,
                1 << 20,
                &mut tier1::CodeBlockEncodeScratch::new(),
            )
            .unwrap();
            let block = &band.code_blocks[0];
            assert_eq!(block.coding_passes, passes);
            assert_eq!(lengths[0].len(), groups.len());
            let mut writer = PacketBitWriter::new();
            write_lengths(&mut writer, passes, output.len(), &lengths[0]).unwrap();
            writer.align();
            let mut reader = PacketBitReader::new(writer.bytes());
            let mut lblock = 3;
            while reader.read_bits_with_stuffing(1).unwrap() != 0 {
                lblock += 1;
            }
            let (total, segments, continuing) =
                read_classic_codeword_lengths(&mut reader, lblock, 0, passes, 1).unwrap();
            assert_eq!(total, output.len());
            assert!(!continuing);
            assert_eq!(
                segments.iter().map(|s| s.coding_passes).collect::<Vec<_>>(),
                groups
            );
            assert_eq!(
                segments.iter().map(|s| s.byte_len).collect::<Vec<_>>(),
                lengths[0]
            );
        }
    }

    #[test]
    fn selective_bypass_packet_length_fields_have_authored_bits() {
        let mut writer = PacketBitWriter::new();
        write_lengths(&mut writer, 13, 19, &[16, 2, 1]).unwrap();
        writer.align();
        // Unary zero, six-bit 16, four-bit 2, three-bit 1, then alignment.
        assert_eq!(writer.bytes(), &[0b00100000, 0b01000100]);
        assert!(validate_lengths(13, 19, &[16, 3]).is_err());
        assert!(validate_lengths(13, 19, &[16, 2, 2]).is_err());
        assert!(validate_lengths(13, 19, &[usize::MAX, 2, 1]).is_err());
    }

    #[test]
    fn selective_bypass_d2_native_roundtrip() {
        let (width, height) = (133, 129);
        for components in [1, 3, 8] {
            for amplitude in [16, 32, 65536] {
                let source: Vec<Vec<u8>> = (0..components)
                    .map(|c| {
                        (0..width * height)
                            .flat_map(|i| {
                                let sample =
                                    ((i * 79 + i / width * 113 + c * 131) % amplitude) as u16;
                                sample.to_le_bytes()
                            })
                            .collect()
                    })
                    .collect();
                let planes: Vec<_> = source
                    .iter()
                    .map(|samples| LosslessD2Plane {
                        samples,
                        stride_bytes: width as usize * 2,
                        sample_step_bytes: 2,
                    })
                    .collect();
                let bytes = encode_lossless_d2_impl::<false, true>(
                    width,
                    height,
                    16,
                    &planes,
                    LosslessEncodeLimits::default(),
                    &mut LosslessEncodeTimings::default(),
                    &mut LosslessEncodeExecution::default(),
                )
                .unwrap();
                let decoded = decode_baseline_owned_components(&bytes).unwrap();
                for (expected, actual) in source.iter().zip(&decoded.components) {
                    assert_eq!(
                        expected, &actual.samples,
                        "components={components}, amplitude={amplitude}"
                    );
                }
            }
        }
    }
}
