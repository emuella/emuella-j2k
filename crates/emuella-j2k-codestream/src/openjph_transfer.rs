// SPDX-License-Identifier: BSD-2-Clause
//
// This module isolates the reversible 32-bit code-block transfer aligned with
// OpenJPH's generic codestream implementation at the pinned revision below.
//
// Source: https://github.com/aous72/OpenJPH
// Commit: 2d0a033a135fb58dab87ea9551db8870e5b68548
// Files: src/core/coding/ojph_block_decoder32.cpp and
//        src/core/codestream/ojph_codestream_gen.cpp
// Copyright (c) 2019, Aous Naman
// Copyright (c) 2019, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2019, The University of New South Wales, Australia
// Copyright (c) 2022, Aous Naman
// Copyright (c) 2022, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2022, The University of New South Wales, Australia
// All rights reserved.
//
// Modified for Emuella: expressed as checked scalar Rust helpers operating on
// typed transfer metadata and both signed and sign-magnitude coefficient
// inputs. See THIRD_PARTY.md and LICENSE-BSD-2-CLAUSE in the packaged crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HtReversibleCodeBlockTransfer {
    pub(super) qcd_guard_bits: u8,
    pub(super) qcd_exponent: u8,
    pub(super) k_max: u8,
    pub(super) shift: u8,
}

pub(super) fn ht_cleanup_bitplane_from_missing_msbs(
    missing_most_significant_bitplanes: u8,
) -> Option<u8> {
    if missing_most_significant_bitplanes < 30 {
        Some(30 - missing_most_significant_bitplanes)
    } else {
        None
    }
}

pub(super) fn openjph_reversible_transfer_coefficient(
    coefficient: i32,
    transfer: HtReversibleCodeBlockTransfer,
) -> i32 {
    let magnitude = coefficient.unsigned_abs().min(0x7fff_ffff);
    let shifted = (magnitude >> transfer.shift) as i32;
    if coefficient < 0 { -shifted } else { shifted }
}

#[inline(always)]
pub(super) fn openjph_reversible_transfer_decoded_coefficient(
    coefficient: i32,
    transfer: HtReversibleCodeBlockTransfer,
) -> i32 {
    debug_assert_ne!(coefficient, i32::MIN);
    let negative = coefficient >> 31;
    let truncation_bias = ((1_u32 << transfer.shift) - 1) as i32;
    (coefficient + (negative & truncation_bias)) >> transfer.shift
}

pub(super) fn openjph_reversible_transfer_sign_magnitude_coefficient(
    coefficient: u32,
    transfer: HtReversibleCodeBlockTransfer,
) -> i32 {
    let magnitude = (coefficient & 0x7fff_ffff) >> transfer.shift;
    let shifted = magnitude as i32;
    if (coefficient & (1_u32 << 31)) != 0 {
        -shifted
    } else {
        shifted
    }
}
