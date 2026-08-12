// SPDX-License-Identifier: BSD-2-Clause
//
// This module isolates the AVX2 HT cleanup gather and reconstruction kernels
// aligned with OpenJPH's AVX2 block decoder at the pinned revision below.
//
// Source: https://github.com/aous72/OpenJPH
// Commit: 2d0a033a135fb58dab87ea9551db8870e5b68548
// File: src/core/coding/ojph_block_decoder_avx2.cpp
// Copyright (c) 2022, Aous Naman
// Copyright (c) 2022, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2022, The University of New South Wales, Australia
// Copyright (c) 2024, Intel Corporation
// Copyright (c) 2026, Osamu Watanabe
// All rights reserved.
//
// Modified for Emuella: adapted to prepared, checked Rust slices and typed
// packed-codeword inputs; uses gather-based extraction and Emuella's dispatch
// boundary; returns coefficients and predictor codes in a Rust value. See
// THIRD_PARTY.md and LICENSE-BSD-2-CLAUSE in the packaged crate.

use super::HtCleanupOctetOutput;
use core::arch::x86_64::*;

#[target_feature(enable = "avx2")]
pub(super) unsafe fn decode_prepared_octet_avx2_gather(
    prepared: &[u8],
    bit_offset: usize,
    lengths: [u8; 8],
    significance: u8,
    embedded: u8,
    shift: u32,
) -> HtCleanupOctetOutput {
    let base_byte_offset = bit_offset / 8;
    let starting_bit = bit_offset % 8;
    let mut byte_offsets = [0_i32; 8];
    let mut bit_shifts = [0_i32; 8];
    let mut offset = starting_bit;
    for lane in 0..8 {
        byte_offsets[lane] = (offset / 8) as i32;
        bit_shifts[lane] = (offset % 8) as i32;
        offset += usize::from(lengths[lane]);
    }

    let byte_offsets = unsafe { _mm256_loadu_si256(byte_offsets.as_ptr().cast()) };
    let bit_shifts = unsafe { _mm256_loadu_si256(bit_shifts.as_ptr().cast()) };
    let base = unsafe { prepared.as_ptr().add(base_byte_offset).cast::<i32>() };
    let windows = unsafe { _mm256_i32gather_epi32(base, byte_offsets, 1) };
    let raw = _mm256_srlv_epi32(windows, bit_shifts);

    let packed_lengths = unsafe { _mm_loadl_epi64(lengths.as_ptr().cast()) };
    let lengths = _mm256_cvtepu8_epi32(packed_lengths);
    let one = _mm256_set1_epi32(1);
    let masks = _mm256_sub_epi32(_mm256_sllv_epi32(one, lengths), one);
    let raw = _mm256_and_si256(raw, masks);

    let lane_bits = _mm256_setr_epi32(1, 2, 4, 8, 16, 32, 64, 128);
    let significant = _mm256_cmpeq_epi32(
        _mm256_and_si256(_mm256_set1_epi32(i32::from(significance)), lane_bits),
        lane_bits,
    );
    let embedded = _mm256_and_si256(
        _mm256_cmpeq_epi32(
            _mm256_and_si256(_mm256_set1_epi32(i32::from(embedded)), lane_bits),
            lane_bits,
        ),
        one,
    );
    let embedded = _mm256_sllv_epi32(embedded, lengths);
    let predictors = _mm256_and_si256(
        _mm256_or_si256(_mm256_or_si256(raw, embedded), one),
        significant,
    );

    let nonzero = _mm256_cmpgt_epi32(predictors, _mm256_setzero_si256());
    let scaled = _mm256_sllv_epi32(
        _mm256_add_epi32(predictors, _mm256_set1_epi32(2)),
        _mm256_set1_epi32(shift as i32),
    );
    let magnitude = _mm256_and_si256(scaled, _mm256_set1_epi32(0x7fff_ffff));
    let explicit_sign = _mm256_srai_epi32(
        _mm256_and_si256(_mm256_slli_epi32(raw, 31), significant),
        31,
    );
    let sign = _mm256_or_si256(explicit_sign, _mm256_srai_epi32(scaled, 31));
    let coefficients = _mm256_and_si256(
        _mm256_sub_epi32(_mm256_xor_si256(magnitude, sign), sign),
        nonzero,
    );

    let mut output = HtCleanupOctetOutput {
        coefficients: [0; 8],
        predictors: [0; 8],
        consumed_bits: (offset - starting_bit) as u16,
    };
    unsafe {
        _mm256_storeu_si256(output.predictors.as_mut_ptr().cast(), predictors);
        _mm256_storeu_si256(output.coefficients.as_mut_ptr().cast(), coefficients);
    }
    output
}

#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn decode_prepared_codeword_octet_avx2_gather(
    prepared: &[u8],
    bit_offset: usize,
    first_codeword: u16,
    second_codeword: u16,
    first_u: u16,
    second_u: u16,
    shift: u32,
) -> HtCleanupOctetOutput {
    let significance =
        ((first_codeword >> 4) & 0xf) as u8 | (((second_codeword >> 4) & 0xf) as u8) << 4;
    let embedded =
        ((first_codeword >> 8) & 0xf) as u8 | (((second_codeword >> 8) & 0xf) as u8) << 4;
    let reductions =
        ((first_codeword >> 12) & 0xf) as u8 | (((second_codeword >> 12) & 0xf) as u8) << 4;

    let one = _mm256_set1_epi32(1);
    let lane_bits = _mm256_setr_epi32(1, 2, 4, 8, 16, 32, 64, 128);
    let significant = _mm256_cmpeq_epi32(
        _mm256_and_si256(_mm256_set1_epi32(i32::from(significance)), lane_bits),
        lane_bits,
    );
    let reduced = _mm256_and_si256(
        _mm256_cmpeq_epi32(
            _mm256_and_si256(_mm256_set1_epi32(i32::from(reductions)), lane_bits),
            lane_bits,
        ),
        one,
    );
    let u_values = _mm256_setr_epi32(
        i32::from(first_u),
        i32::from(first_u),
        i32::from(first_u),
        i32::from(first_u),
        i32::from(second_u),
        i32::from(second_u),
        i32::from(second_u),
        i32::from(second_u),
    );
    let lengths = _mm256_and_si256(_mm256_sub_epi32(u_values, reduced), significant);

    let adjacent = _mm256_add_epi32(lengths, _mm256_slli_si256(lengths, 4));
    let half_inclusive = _mm256_add_epi32(adjacent, _mm256_slli_si256(adjacent, 8));
    let first_half_total = _mm256_extract_epi32::<3>(half_inclusive);
    let inclusive = _mm256_add_epi32(
        half_inclusive,
        _mm256_setr_epi32(
            0,
            0,
            0,
            0,
            first_half_total,
            first_half_total,
            first_half_total,
            first_half_total,
        ),
    );
    let positions = _mm256_add_epi32(
        _mm256_sub_epi32(inclusive, lengths),
        _mm256_set1_epi32((bit_offset % 8) as i32),
    );
    let byte_offsets = _mm256_srli_epi32::<3>(positions);
    let bit_shifts = _mm256_and_si256(positions, _mm256_set1_epi32(7));

    let base = unsafe { prepared.as_ptr().add(bit_offset / 8).cast::<i32>() };
    let windows = unsafe { _mm256_i32gather_epi32(base, byte_offsets, 1) };
    let raw = _mm256_srlv_epi32(windows, bit_shifts);
    let masks = _mm256_sub_epi32(_mm256_sllv_epi32(one, lengths), one);
    let raw = _mm256_and_si256(raw, masks);

    let embedded = _mm256_and_si256(
        _mm256_cmpeq_epi32(
            _mm256_and_si256(_mm256_set1_epi32(i32::from(embedded)), lane_bits),
            lane_bits,
        ),
        one,
    );
    let predictors = _mm256_and_si256(
        _mm256_or_si256(
            _mm256_or_si256(raw, _mm256_sllv_epi32(embedded, lengths)),
            one,
        ),
        significant,
    );
    let nonzero = _mm256_cmpgt_epi32(predictors, _mm256_setzero_si256());
    let scaled = _mm256_sllv_epi32(
        _mm256_add_epi32(predictors, _mm256_set1_epi32(2)),
        _mm256_set1_epi32(shift as i32),
    );
    let magnitude = _mm256_and_si256(scaled, _mm256_set1_epi32(0x7fff_ffff));
    let explicit_sign = _mm256_srai_epi32(
        _mm256_and_si256(_mm256_slli_epi32(raw, 31), significant),
        31,
    );
    let sign = _mm256_or_si256(explicit_sign, _mm256_srai_epi32(scaled, 31));
    let coefficients = _mm256_and_si256(
        _mm256_sub_epi32(_mm256_xor_si256(magnitude, sign), sign),
        nonzero,
    );

    let mut output = HtCleanupOctetOutput {
        coefficients: [0; 8],
        predictors: [0; 8],
        consumed_bits: _mm256_extract_epi32::<7>(inclusive) as u16,
    };
    unsafe {
        _mm256_storeu_si256(output.predictors.as_mut_ptr().cast(), predictors);
        _mm256_storeu_si256(output.coefficients.as_mut_ptr().cast(), coefficients);
    }
    output
}
