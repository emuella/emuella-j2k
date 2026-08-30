// SPDX-License-Identifier: BSD-2-Clause
//
// This scalar HT cleanup encoder follows Rec. ITU-T T.814 Annex F and is
// derived in part from OpenJPH's BSD-2-Clause scalar block encoder:
// https://github.com/aous72/OpenJPH/blob/2d0a033a135fb58dab87ea9551db8870e5b68548/src/core/coding/ojph_block_encoder.cpp
// Copyright (c) 2019, Aous Naman
// Copyright (c) 2019, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2019, The University of New South Wales, Australia
// All rights reserved.
//
// Modified for Emuella: translated to safe Rust, adapted to Emuella's checked
// coefficient representation, and changed to return structured Rust errors.
// See THIRD_PARTY.md and LICENSE-BSD-2-CLAUSE in the packaged crate, or the
// corresponding root records in the source workspace.

use alloc::vec::Vec;
use core::fmt;

use crate::ht_vlc_tables::{HtVlcStandardTableKind, ht_vlc_encoder_word};

const MAX_BLOCK_DIMENSION: usize = 1024;
const MAX_SCUP: usize = 0x0fff;

/// One cleanup-only HT code-block segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtCleanupEncodedBlock {
    /// Packet-header zero bit-plane count.
    pub missing_most_significant_bitplanes: u8,
    /// Cleanup-only HT blocks contain one coding pass.
    pub coding_passes: u16,
    /// Complete cleanup segment, including its 12-bit `Scup` locator.
    pub segment: Vec<u8>,
}

/// Fail-closed errors from scalar HT cleanup encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtCleanupEncodeError {
    InvalidDimensions,
    InvalidStride,
    CoefficientBufferTooSmall,
    InvalidMagnitudeBitDepth,
    CoefficientMagnitudeTooLarge,
    MissingVlcCodeword,
    UvlcValueTooLarge,
    CleanupSuffixTooLarge,
}

impl fmt::Display for HtCleanupEncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidDimensions => "HT code-block dimensions must be in 1..=1024",
            Self::InvalidStride => "HT code-block stride must be at least its width",
            Self::CoefficientBufferTooSmall => {
                "HT code-block coefficient input does not cover its declared dimensions"
            }
            Self::InvalidMagnitudeBitDepth => "HT cleanup magnitude bit depth must be in 2..=31",
            Self::CoefficientMagnitudeTooLarge => {
                "HT cleanup coefficient exceeds the declared magnitude bit depth"
            }
            Self::MissingVlcCodeword => "no Annex C VLC codeword exists for the cleanup tuple",
            Self::UvlcValueTooLarge => "HT cleanup U-VLC residual exceeds the supported bound",
            Self::CleanupSuffixTooLarge => "HT cleanup MEL/VLC suffix exceeds 12-bit Scup",
        };
        f.write_str(message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HtCleanupEncodeError {}

/// Encode one arbitrary cleanup-only HT code-block.
///
/// `magnitude_bit_depth` is the subband `Kmax` value used by packet-header
/// zero-bit-plane signalling. An all-zero block returns `Ok(None)` and should
/// be represented as an unincluded code-block in the packet header.
pub fn encode_ht_cleanup_block(
    coefficients: &[i32],
    width: u16,
    height: u16,
    stride: usize,
    magnitude_bit_depth: u8,
) -> Result<Option<HtCleanupEncodedBlock>, HtCleanupEncodeError> {
    let width = usize::from(width);
    let height = usize::from(height);
    if width == 0 || height == 0 || width > MAX_BLOCK_DIMENSION || height > MAX_BLOCK_DIMENSION {
        return Err(HtCleanupEncodeError::InvalidDimensions);
    }
    if stride < width {
        return Err(HtCleanupEncodeError::InvalidStride);
    }
    let required = (height - 1)
        .checked_mul(stride)
        .and_then(|offset| offset.checked_add(width))
        .ok_or(HtCleanupEncodeError::CoefficientBufferTooSmall)?;
    if coefficients.len() < required {
        return Err(HtCleanupEncodeError::CoefficientBufferTooSmall);
    }
    if !(2..=31).contains(&magnitude_bit_depth) {
        return Err(HtCleanupEncodeError::InvalidMagnitudeBitDepth);
    }
    let magnitude_limit = 1_u64 << magnitude_bit_depth;
    let mut nonzero = false;
    for y in 0..height {
        for &coefficient in &coefficients[y * stride..y * stride + width] {
            let magnitude = u64::from(coefficient.unsigned_abs());
            if magnitude >= magnitude_limit {
                return Err(HtCleanupEncodeError::CoefficientMagnitudeTooLarge);
            }
            nonzero |= magnitude != 0;
        }
    }
    if !nonzero {
        return Ok(None);
    }

    let mut encoder = CleanupEncoder::new(coefficients, width, height, stride);
    encoder.encode()?;
    let segment = encoder.finish()?;
    Ok(Some(HtCleanupEncodedBlock {
        missing_most_significant_bitplanes: magnitude_bit_depth - 1,
        coding_passes: 1,
        segment,
    }))
}

#[derive(Clone, Copy, Default)]
struct Quad {
    rho: u8,
    exponents: [u8; 4],
    max_exponent: u8,
    magnitude_sign: [u32; 4],
}

struct CleanupEncoder<'a> {
    coefficients: &'a [i32],
    width: usize,
    height: usize,
    stride: usize,
    lower_exponents: Vec<u8>,
    lower_contexts: Vec<u8>,
    mel: MelWriter,
    vlc: VlcWriter,
    magsgn: MagSgnWriter,
}

impl<'a> CleanupEncoder<'a> {
    fn new(coefficients: &'a [i32], width: usize, height: usize, stride: usize) -> Self {
        // Two sentinels cover the right-neighbour context read after a
        // cropped final quad.
        let boundary_count = width.div_ceil(2) + 2;
        Self {
            coefficients,
            width,
            height,
            stride,
            lower_exponents: alloc::vec![0; boundary_count],
            lower_contexts: alloc::vec![0; boundary_count],
            mel: MelWriter::default(),
            vlc: VlcWriter::new(),
            magsgn: MagSgnWriter::default(),
        }
    }

    fn encode(&mut self) -> Result<(), HtCleanupEncodeError> {
        self.encode_initial_line()?;
        for y in (2..self.height).step_by(2) {
            self.encode_noninitial_line(y)?;
        }
        Ok(())
    }

    fn encode_initial_line(&mut self) -> Result<(), HtCleanupEncodeError> {
        let mut context0 = 0_u8;
        for x in (0..self.width).step_by(4) {
            let first = self.quad(x, 0);
            let second = (x + 2 < self.width).then(|| self.quad(x + 2, 0));
            let first_residual = first.max_exponent.max(1) - 1;
            self.encode_quad(
                HtVlcStandardTableKind::Initial,
                context0,
                first,
                1,
                first_residual,
            )?;
            self.store_lower_state(x / 2, first);

            let mut second_residual = 0;
            if let Some(second) = second {
                let context1 = (first.rho >> 1) | (first.rho & 1);
                second_residual = second.max_exponent.max(1) - 1;
                self.encode_quad(
                    HtVlcStandardTableKind::Initial,
                    context1,
                    second,
                    1,
                    second_residual,
                )?;
                self.store_lower_state((x + 2) / 2, second);
                context0 = (second.rho >> 1) | (second.rho & 1);
            } else {
                context0 = 0;
            }
            self.encode_initial_uvlc_pair(first_residual, second_residual)?;
        }
        Ok(())
    }

    fn encode_noninitial_line(&mut self, y: usize) -> Result<(), HtCleanupEncodeError> {
        let mut boundary = 0_usize;
        let mut max_e = self.lower_exponents[0].max(self.lower_exponents[1]) as i16 - 1;
        self.lower_exponents[0] = 0;
        let mut context0 = self.lower_contexts[0] | (self.lower_contexts[1] << 2);
        self.lower_contexts[0] = 0;

        for x in (0..self.width).step_by(4) {
            let first = self.quad(x, y);
            let first_kappa = if first.rho.count_ones() > 1 {
                max_e.max(1) as u8
            } else {
                1
            };
            let first_residual = first.max_exponent.max(first_kappa) - first_kappa;
            self.encode_quad(
                HtVlcStandardTableKind::NonInitial,
                context0,
                first,
                first_kappa,
                first_residual,
            )?;
            let (next_max_e, next_context) = self.advance_lower_state(&mut boundary, first);
            max_e = next_max_e;
            let context1 = next_context | ((first.rho & 4) >> 1) | ((first.rho & 8) >> 2);

            let mut second_residual = 0;
            if x + 2 < self.width {
                let second = self.quad(x + 2, y);
                let second_kappa = if second.rho.count_ones() > 1 {
                    max_e.max(1) as u8
                } else {
                    1
                };
                second_residual = second.max_exponent.max(second_kappa) - second_kappa;
                self.encode_quad(
                    HtVlcStandardTableKind::NonInitial,
                    context1,
                    second,
                    second_kappa,
                    second_residual,
                )?;
                let (next_max_e, next_context) = self.advance_lower_state(&mut boundary, second);
                max_e = next_max_e;
                context0 = next_context | ((second.rho & 4) >> 1) | ((second.rho & 8) >> 2);
            } else {
                context0 = 0;
            }
            self.encode_uvlc_prefix(first_residual)?;
            self.encode_uvlc_prefix(second_residual)?;
            self.encode_uvlc_suffix(first_residual)?;
            self.encode_uvlc_suffix(second_residual)?;
        }
        Ok(())
    }

    fn quad(&self, x: usize, y: usize) -> Quad {
        let mut quad = Quad::default();
        for local_x in 0..2 {
            for local_y in 0..2 {
                if x + local_x >= self.width || y + local_y >= self.height {
                    continue;
                }
                let slot = local_x * 2 + local_y;
                let coefficient = self.coefficients[(y + local_y) * self.stride + x + local_x];
                let magnitude = coefficient.unsigned_abs();
                if magnitude == 0 {
                    continue;
                }
                quad.rho |= 1 << slot;
                let twice_minus_one = magnitude * 2 - 1;
                let exponent = (u32::BITS - twice_minus_one.leading_zeros()) as u8;
                quad.exponents[slot] = exponent;
                quad.max_exponent = quad.max_exponent.max(exponent);
                quad.magnitude_sign[slot] = (magnitude - 1) * 2 + u32::from(coefficient < 0);
            }
        }
        quad
    }

    fn encode_quad(
        &mut self,
        table: HtVlcStandardTableKind,
        context: u8,
        quad: Quad,
        kappa: u8,
        residual: u8,
    ) -> Result<u16, HtCleanupEncodeError> {
        let mut embedded = 0_u8;
        if residual > 0 {
            for slot in 0..4 {
                embedded |= u8::from(quad.exponents[slot] == quad.max_exponent) << slot;
            }
        }
        let word = ht_vlc_encoder_word(table, context, quad.rho, embedded);
        if word == 0 && (quad.rho != 0 || context != 0) {
            return Err(HtCleanupEncodeError::MissingVlcCodeword);
        }
        self.vlc
            .write(u32::from(word >> 8), ((word >> 4) & 7) as u8);
        if context == 0 {
            self.mel.encode(quad.rho != 0);
        }
        let upper = quad.max_exponent.max(kappa);
        for slot in 0..4 {
            if quad.rho & (1 << slot) == 0 {
                continue;
            }
            let reduction = ((word & 0x0f) >> slot) & 1;
            let bit_count = upper - reduction as u8;
            self.magsgn.write(quad.magnitude_sign[slot], bit_count);
        }
        Ok(word)
    }

    fn store_lower_state(&mut self, boundary: usize, quad: Quad) {
        self.lower_exponents[boundary] = self.lower_exponents[boundary].max(quad.exponents[1]);
        self.lower_exponents[boundary + 1] = quad.exponents[3];
        self.lower_contexts[boundary] |= (quad.rho & 2) >> 1;
        self.lower_contexts[boundary + 1] = (quad.rho & 8) >> 3;
    }

    fn advance_lower_state(&mut self, boundary: &mut usize, quad: Quad) -> (i16, u8) {
        self.lower_exponents[*boundary] = self.lower_exponents[*boundary].max(quad.exponents[1]);
        self.lower_contexts[*boundary] |= (quad.rho & 2) >> 1;
        *boundary += 1;

        let max_e =
            self.lower_exponents[*boundary].max(self.lower_exponents[*boundary + 1]) as i16 - 1;
        let context = self.lower_contexts[*boundary] | (self.lower_contexts[*boundary + 1] << 2);
        self.lower_exponents[*boundary] = quad.exponents[3];
        self.lower_contexts[*boundary] = (quad.rho & 8) >> 3;
        (max_e, context)
    }

    fn encode_initial_uvlc_pair(
        &mut self,
        first: u8,
        second: u8,
    ) -> Result<(), HtCleanupEncodeError> {
        if first > 0 && second > 0 {
            self.mel.encode(first.min(second) > 2);
        }
        if first > 2 && second > 2 {
            self.encode_uvlc_prefix(first - 2)?;
            self.encode_uvlc_prefix(second - 2)?;
            self.encode_uvlc_suffix(first - 2)?;
            self.encode_uvlc_suffix(second - 2)?;
        } else if first > 2 && second > 0 {
            self.encode_uvlc_prefix(first)?;
            self.vlc.write(u32::from(second - 1), 1);
            self.encode_uvlc_suffix(first)?;
        } else {
            self.encode_uvlc_prefix(first)?;
            self.encode_uvlc_prefix(second)?;
            self.encode_uvlc_suffix(first)?;
            self.encode_uvlc_suffix(second)?;
        }
        Ok(())
    }

    fn encode_uvlc_prefix(&mut self, value: u8) -> Result<(), HtCleanupEncodeError> {
        let code = UvlcCode::new(value)?;
        self.vlc.write(u32::from(code.prefix), code.prefix_len);
        Ok(())
    }

    fn encode_uvlc_suffix(&mut self, value: u8) -> Result<(), HtCleanupEncodeError> {
        let code = UvlcCode::new(value)?;
        self.vlc.write(u32::from(code.suffix), code.suffix_len);
        self.vlc
            .write(u32::from(code.extension), code.extension_len);
        Ok(())
    }

    fn finish(mut self) -> Result<Vec<u8>, HtCleanupEncodeError> {
        self.mel.terminate_run();
        self.vlc.terminate_with_mel(&mut self.mel);
        self.magsgn.terminate();

        let vlc = self.vlc.into_forward_bytes();
        let scup = self.mel.bytes.len() + vlc.len();
        if scup > MAX_SCUP {
            return Err(HtCleanupEncodeError::CleanupSuffixTooLarge);
        }
        let mut segment = self.magsgn.bytes;
        segment.extend_from_slice(&self.mel.bytes);
        segment.extend_from_slice(&vlc);
        let len = segment.len();
        if len < 2 {
            return Err(HtCleanupEncodeError::CleanupSuffixTooLarge);
        }
        segment[len - 1] = (scup >> 4) as u8;
        segment[len - 2] = (segment[len - 2] & 0xf0) | (scup as u8 & 0x0f);
        Ok(segment)
    }
}

#[derive(Default)]
struct MelWriter {
    bytes: Vec<u8>,
    remaining_bits: u8,
    current: u8,
    run: usize,
    state: usize,
}

impl MelWriter {
    fn encode(&mut self, event: bool) {
        const EXPONENTS: [u8; 13] = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 4, 5];
        if self.remaining_bits == 0 {
            self.remaining_bits = 8;
        }
        let threshold = 1_usize << EXPONENTS[self.state];
        if !event {
            self.run += 1;
            if self.run >= threshold {
                self.emit_bit(true);
                self.run = 0;
                self.state = (self.state + 1).min(12);
            }
        } else {
            self.emit_bit(false);
            for shift in (0..EXPONENTS[self.state]).rev() {
                self.emit_bit(((self.run >> shift) & 1) != 0);
            }
            self.run = 0;
            self.state = self.state.saturating_sub(1);
        }
    }

    fn emit_bit(&mut self, bit: bool) {
        if self.remaining_bits == 0 {
            self.remaining_bits = 8;
        }
        self.current = (self.current << 1) | u8::from(bit);
        self.remaining_bits -= 1;
        if self.remaining_bits == 0 {
            self.bytes.push(self.current);
            self.remaining_bits = if self.current == 0xff { 7 } else { 8 };
            self.current = 0;
        }
    }

    fn terminate_run(&mut self) {
        if self.run > 0 {
            self.emit_bit(true);
        }
    }
}

struct VlcWriter {
    reverse_bytes: Vec<u8>,
    used_bits: u8,
    current: u8,
    previous_greater_than_8f: bool,
}

impl VlcWriter {
    fn new() -> Self {
        Self {
            reverse_bytes: alloc::vec![0xff],
            used_bits: 4,
            current: 0x0f,
            previous_greater_than_8f: true,
        }
    }

    fn write(&mut self, mut codeword: u32, mut bit_count: u8) {
        while bit_count > 0 {
            let available = 8 - u8::from(self.previous_greater_than_8f) - self.used_bits;
            let take = available.min(bit_count);
            let mask = (1_u32 << take) - 1;
            self.current |= ((codeword & mask) as u8) << self.used_bits;
            self.used_bits += take;
            bit_count -= take;
            codeword >>= take;
            if self.used_bits + u8::from(self.previous_greater_than_8f) == 8 {
                if self.previous_greater_than_8f && self.current != 0x7f {
                    self.previous_greater_than_8f = false;
                    continue;
                }
                self.reverse_bytes.push(self.current);
                self.previous_greater_than_8f = self.current > 0x8f;
                self.current = 0;
                self.used_bits = 0;
            }
        }
    }

    fn terminate_with_mel(&mut self, mel: &mut MelWriter) {
        if mel.remaining_bits == 0 {
            mel.remaining_bits = 8;
        }
        let (mel_current, mel_mask) = if mel.remaining_bits == 8 {
            // No partial MEL byte remains to fuse. Avoid shifting a `u8` by
            // its width after the empty state is normalised above.
            (0, 0)
        } else {
            (
                mel.current << mel.remaining_bits,
                (0xff_u16 << mel.remaining_bits) as u8,
            )
        };
        let vlc_mask = if self.used_bits == 0 {
            0
        } else {
            0xff >> (8 - self.used_bits)
        };
        if mel_mask | vlc_mask == 0 {
            return;
        }
        let fused = mel_current | self.current;
        let compatible =
            (((fused ^ mel_current) & mel_mask) | ((fused ^ self.current) & vlc_mask)) == 0;
        if compatible && fused != 0xff && self.reverse_bytes.len() > 1 {
            mel.bytes.push(fused);
        } else {
            mel.bytes.push(mel_current);
            self.reverse_bytes.push(self.current);
        }
    }

    fn into_forward_bytes(mut self) -> Vec<u8> {
        self.reverse_bytes.reverse();
        self.reverse_bytes
    }
}

#[derive(Default)]
struct MagSgnWriter {
    bytes: Vec<u8>,
    max_bits: u8,
    used_bits: u8,
    current: u8,
}

impl MagSgnWriter {
    fn write(&mut self, mut value: u32, mut bit_count: u8) {
        if self.max_bits == 0 {
            self.max_bits = 8;
        }
        while bit_count > 0 {
            let take = (self.max_bits - self.used_bits).min(bit_count);
            let mask = (1_u32 << take) - 1;
            self.current |= ((value & mask) as u8) << self.used_bits;
            self.used_bits += take;
            value >>= take;
            bit_count -= take;
            if self.used_bits == self.max_bits {
                self.bytes.push(self.current);
                self.max_bits = if self.current == 0xff { 7 } else { 8 };
                self.current = 0;
                self.used_bits = 0;
            }
        }
    }

    fn terminate(&mut self) {
        if self.max_bits == 0 {
            self.max_bits = 8;
        }
        if self.used_bits > 0 {
            let unused = self.max_bits - self.used_bits;
            self.current |= (((1_u16 << unused) - 1) as u8) << self.used_bits;
            if self.current != 0xff {
                self.bytes.push(self.current);
            }
        } else if self.max_bits == 7 {
            self.bytes.pop();
        }
    }
}

struct UvlcCode {
    prefix: u8,
    prefix_len: u8,
    suffix: u8,
    suffix_len: u8,
    extension: u8,
    extension_len: u8,
}

impl UvlcCode {
    fn new(value: u8) -> Result<Self, HtCleanupEncodeError> {
        let code = match value {
            0 => Self::parts(0, 0, 0, 0, 0, 0),
            1 => Self::parts(1, 1, 0, 0, 0, 0),
            2 => Self::parts(2, 2, 0, 0, 0, 0),
            3 => Self::parts(4, 3, 0, 1, 0, 0),
            4 => Self::parts(4, 3, 1, 1, 0, 0),
            5..=32 => Self::parts(0, 3, value - 5, 5, 0, 0),
            33..=74 => Self::parts(0, 3, 28 + (value - 33) % 4, 5, (value - 33) / 4, 4),
            _ => return Err(HtCleanupEncodeError::UvlcValueTooLarge),
        };
        Ok(code)
    }

    const fn parts(
        prefix: u8,
        prefix_len: u8,
        suffix: u8,
        suffix_len: u8,
        extension: u8,
        extension_len: u8,
    ) -> Self {
        Self {
            prefix,
            prefix_len,
            suffix,
            suffix_len,
            extension,
            extension_len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::encode_ht_cleanup_block;

    #[test]
    fn byte_aligned_empty_mel_suffix_encodes_sparse_23_by_1_block() {
        let mut coefficients = [0_i32; 23];
        coefficients[0] = 1;

        let encoded = encode_ht_cleanup_block(&coefficients, 23, 1, 23, 2)
            .expect("sparse block is encodable")
            .expect("sparse block is included");

        assert_eq!(encoded.missing_most_significant_bitplanes, 1);
        assert_eq!(encoded.coding_passes, 1);
        assert!(encoded.segment.len() >= 2);
    }
}
