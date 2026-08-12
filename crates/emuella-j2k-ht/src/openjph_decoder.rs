// SPDX-License-Identifier: BSD-2-Clause
//
// This module isolates the 32-bit HT cleanup representation aligned with the
// OpenJPH block decoders at the pinned revision below.
//
// Source: https://github.com/aous72/OpenJPH
// Commit: 2d0a033a135fb58dab87ea9551db8870e5b68548
// Files: src/core/coding/ojph_block_decoder32.cpp and
//        src/core/coding/ojph_block_decoder64.cpp
// Copyright (c) 2019, Aous Naman
// Copyright (c) 2019, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2019, The University of New South Wales, Australia
// All rights reserved.
//
// Modified for Emuella: translated to safe Rust with checked arithmetic,
// structured errors, and typed cleanup-coefficient inputs. See THIRD_PARTY.md
// and LICENSE-BSD-2-CLAUSE in the packaged crate.

use super::{HtLayoutError, HtVlcCleanupCoefficientOutput};

pub(super) fn cleanup_bitplane_from_missing_msbs(
    missing_most_significant_bitplanes: u8,
) -> Result<u8, HtLayoutError> {
    if missing_most_significant_bitplanes >= 30 {
        return Err(HtLayoutError::InvalidCleanupBitplane {
            missing_most_significant_bitplanes,
        });
    }
    Ok(30 - missing_most_significant_bitplanes)
}

impl HtVlcCleanupCoefficientOutput {
    /// Reconstruct this cleanup coefficient using OpenJPH's unsigned
    /// sign-magnitude intermediate representation.
    ///
    /// Bit 31 carries the sign and bits 0..30 carry the centered magnitude.
    /// OpenJPH keeps this representation until the block-to-sample transfer
    /// stage, so diagnostics that compare against its code-block buffer need
    /// this form rather than Rust's signed coefficient form.
    pub fn reconstruct_cleanup_sign_magnitude_coefficient(
        self,
        missing_most_significant_bitplanes: u8,
    ) -> Result<u32, HtLayoutError> {
        let cleanup_bitplane =
            cleanup_bitplane_from_missing_msbs(missing_most_significant_bitplanes)?;
        if !self.significant {
            if self.magnitude_sign_bits != 0
                || self.magnitude_sign_value != 0
                || self.embedded_magnitude_bit
                || self.magnitude_exponent_reduction
            {
                return Err(HtLayoutError::InvalidVlcCleanupOutput {
                    reason: "insignificant coefficient declares magnitude/sign data",
                });
            }
            return Ok(0);
        }
        if self.magnitude_sign_bits > 16 {
            return Err(HtLayoutError::SizeOverflow);
        }

        let exponent_reduction = u16::from(self.magnitude_exponent_reduction);
        let u_value = self
            .magnitude_sign_bits
            .checked_add(exponent_reduction)
            .ok_or(HtLayoutError::SizeOverflow)?;
        if u_value == 0 || u_value > 31 {
            return Err(HtLayoutError::SizeOverflow);
        }

        let explicit_mask = if self.magnitude_sign_bits == 16 {
            u16::MAX
        } else {
            (1_u16 << self.magnitude_sign_bits) - 1
        };
        let sign = if (self.magnitude_sign_value & 1) != 0 {
            1_u32 << 31
        } else {
            0
        };
        let mut magnitude_code = u32::from(self.magnitude_sign_value & explicit_mask);
        magnitude_code |= u32::from(self.embedded_magnitude_bit) << self.magnitude_sign_bits;
        magnitude_code |= 1;

        let centered_magnitude = magnitude_code
            .checked_add(2)
            .ok_or(HtLayoutError::SizeOverflow)?;
        let scaled = centered_magnitude
            .checked_shl(u32::from(cleanup_bitplane - 1))
            .ok_or(HtLayoutError::SizeOverflow)?;
        Ok(sign | scaled)
    }

    /// Reconstruct this cleanup coefficient with the 32-bit HT cleanup formula.
    ///
    /// The cleanup bitplane is derived from the packet's missing most
    /// significant bitplane count using the same 32-bit bound as OpenJPH:
    /// `p = 30 - missing_msbs`, with `p == 0` intentionally rejected because
    /// this centered reconstruction path needs one lower bit of precision.
    /// The MagSgn value carries the sign in bit 0; `embedded_magnitude_bit`
    /// and `magnitude_exponent_reduction` are the `e_1` and `e_k` flags
    /// decoded from the VLC word.
    pub fn reconstruct_cleanup_coefficient(
        self,
        missing_most_significant_bitplanes: u8,
    ) -> Result<i32, HtLayoutError> {
        if !self.significant {
            if self.magnitude_sign_bits != 0
                || self.magnitude_sign_value != 0
                || self.embedded_magnitude_bit
                || self.magnitude_exponent_reduction
            {
                return Err(HtLayoutError::InvalidVlcCleanupOutput {
                    reason: "insignificant coefficient declares magnitude/sign data",
                });
            }
            return Ok(0);
        }
        if self.magnitude_sign_bits > 16 {
            return Err(HtLayoutError::SizeOverflow);
        }

        let sign_magnitude = self
            .reconstruct_cleanup_sign_magnitude_coefficient(missing_most_significant_bitplanes)?;
        let negative = (sign_magnitude & (1_u32 << 31)) != 0;
        let scaled = sign_magnitude & 0x7fff_ffff;
        let signed = if negative {
            -i64::from(scaled)
        } else {
            i64::from(scaled)
        };
        if signed < i64::from(i32::MIN) || signed > i64::from(i32::MAX) {
            return Err(HtLayoutError::SizeOverflow);
        }
        Ok(signed as i32)
    }
}
