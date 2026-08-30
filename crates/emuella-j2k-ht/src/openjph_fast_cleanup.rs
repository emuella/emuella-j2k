// SPDX-License-Identifier: BSD-2-Clause
//
// This module isolates the HT cleanup MEL, VLC, and MagSgn reader architecture
// aligned with the OpenJPH block decoder family at the pinned revision below.
//
// Source: https://github.com/aous72/OpenJPH
// Commit: 2d0a033a135fb58dab87ea9551db8870e5b68548
// Files: src/core/coding/ojph_block_decoder32.cpp,
//        src/core/coding/ojph_block_decoder64.cpp, and
//        src/core/coding/ojph_block_decoder_avx2.cpp
// Copyright (c) 2019, Aous Naman
// Copyright (c) 2019, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2019, The University of New South Wales, Australia
// Copyright (c) 2022, Aous Naman
// Copyright (c) 2022, Kakadu Software Pty Ltd, Australia
// Copyright (c) 2022, The University of New South Wales, Australia
// Copyright (c) 2024, Intel Corporation
// Copyright (c) 2026, Osamu Watanabe
// All rights reserved.
//
// Modified for Emuella: translated to safe, checked Rust readers; split into
// typed MEL, VLC, and MagSgn cursors; added reusable prepared storage and
// Emuella backend dispatch; and retained scalar fallbacks. See THIRD_PARTY.md
// and LICENSE-BSD-2-CLAUSE in the packaged crate.

use super::{
    HT_MEL_EXPONENTS, HtCleanupPassSegmentBitCounts, HtCleanupPassSegmentByteAlignment,
    HtCleanupPassSegmentByteCounts, HtCleanupPassSegmentViews, HtCleanupStreamKind, HtLayoutError,
    HtMelEvent, HtVlcContext, HtVlcContextProgressionMode, HtVlcInitialUvlcMode, HtVlcLookupTable,
    HtVlcNonInitialUvlcMode, HtVlcQuadCodeword, HtVlcUvlcPair,
};

#[cfg(feature = "std")]
use emuella_j2k_accel::{
    HtCleanupBackend, HtCleanupBackendError, HtCleanupDispatch, HtCleanupOctetError,
};

#[cfg(feature = "std")]
const DENSE_MAGSGN_BITS_PER_COEFFICIENT: usize = 9;

#[cfg(feature = "std")]
// A prepared source cannot have this length; the sentinel avoids growing hot reader state.
const DIRECT_MAGSGN_SOURCE_BITS: usize = usize::MAX;

#[cfg(feature = "std")]
fn is_dense_magsgn(source_data_bits: usize, coefficient_count: usize) -> bool {
    coefficient_count != 0
        && source_data_bits >= coefficient_count.saturating_mul(DENSE_MAGSGN_BITS_PER_COEFFICIENT)
}

#[derive(Debug, Clone)]
pub(super) struct PreparedHtCleanupBlock<'a> {
    pub(super) magnitude_sign: FastMagSgnReader<'a>,
    pub(super) mel: FastMelReader<'a>,
    pub(super) vlc: FastVlcReader<'a>,
}

impl<'a> PreparedHtCleanupBlock<'a> {
    #[cfg(test)]
    pub(super) fn new(views: HtCleanupPassSegmentViews<'a>) -> Result<Self, HtLayoutError> {
        Self::new_with_coefficient_count(views, None)
    }

    #[cfg(feature = "std")]
    pub(super) fn new_for_block(
        views: HtCleanupPassSegmentViews<'a>,
        coefficient_count: usize,
    ) -> Result<Self, HtLayoutError> {
        Self::new_with_coefficient_count(views, Some(coefficient_count))
    }

    fn new_with_coefficient_count(
        views: HtCleanupPassSegmentViews<'a>,
        coefficient_count: Option<usize>,
    ) -> Result<Self, HtLayoutError> {
        if views.mel_vlc.len() < 2 {
            return Err(HtLayoutError::InvalidCleanupPassSegment {
                lcup: views.magnitude_sign.len() + views.mel_vlc.len(),
                scup: views.mel_vlc.len(),
            });
        }
        Ok(Self {
            magnitude_sign: FastMagSgnReader::new_with_coefficient_count(
                views.magnitude_sign,
                coefficient_count,
            ),
            mel: FastMelReader::new(&views.mel_vlc[..views.mel_vlc.len() - 1]),
            vlc: FastVlcReader::new(views.mel_vlc)?,
        })
    }

    #[cfg(feature = "std")]
    pub(super) fn new_for_block_with_magsgn_storage(
        views: HtCleanupPassSegmentViews<'a>,
        coefficient_count: usize,
        storage: PreparedMagSgnStorage,
        dispatch: HtCleanupDispatch,
    ) -> Result<Self, (HtLayoutError, PreparedMagSgnStorage)> {
        if views.mel_vlc.len() < 2 {
            return Err((
                HtLayoutError::InvalidCleanupPassSegment {
                    lcup: views.magnitude_sign.len() + views.mel_vlc.len(),
                    scup: views.mel_vlc.len(),
                },
                storage,
            ));
        }
        let vlc = match FastVlcReader::new(views.mel_vlc) {
            Ok(vlc) => vlc,
            Err(error) => return Err((error, storage)),
        };
        Ok(Self {
            magnitude_sign: FastMagSgnReader::new_with_coefficient_count_and_storage(
                views.magnitude_sign,
                coefficient_count,
                storage,
                dispatch,
            ),
            mel: FastMelReader::new(&views.mel_vlc[..views.mel_vlc.len() - 1]),
            vlc,
        })
    }

    pub(super) fn consumed_bits(&self) -> HtCleanupPassSegmentBitCounts {
        HtCleanupPassSegmentBitCounts {
            magnitude_sign: self.magnitude_sign.consumed_bits(),
            mel: self.mel.consumed_bits(),
            vlc: self.vlc.consumed_bits(),
        }
    }

    pub(super) fn remaining_bits(&self) -> HtCleanupPassSegmentBitCounts {
        HtCleanupPassSegmentBitCounts {
            magnitude_sign: self.magnitude_sign.remaining_bits(),
            mel: self.mel.remaining_bits(),
            vlc: self.vlc.remaining_bits(),
        }
    }

    pub(super) fn consumed_bytes(&self) -> HtCleanupPassSegmentByteCounts {
        HtCleanupPassSegmentByteCounts {
            magnitude_sign: self.magnitude_sign.consumed_bytes(),
            mel: self.mel.consumed_bytes(),
            vlc: self.vlc.consumed_bytes(),
        }
    }

    pub(super) fn remaining_bytes(&self) -> HtCleanupPassSegmentByteCounts {
        HtCleanupPassSegmentByteCounts {
            magnitude_sign: self.magnitude_sign.remaining_bytes(),
            mel: self.mel.remaining_bytes(),
            vlc: self.vlc.remaining_bytes(),
        }
    }

    pub(super) fn byte_alignment(&self) -> HtCleanupPassSegmentByteAlignment {
        HtCleanupPassSegmentByteAlignment {
            magnitude_sign: self.magnitude_sign.is_byte_aligned(),
            mel: self.mel.is_byte_aligned(),
            vlc: self.vlc.is_byte_aligned(),
        }
    }

    #[cfg(feature = "std")]
    pub(super) fn take_prepared_magsgn_storage(&mut self) -> Option<PreparedMagSgnStorage> {
        self.magnitude_sign.take_prepared_magsgn_storage()
    }
}

#[derive(Debug, Clone)]
pub(super) struct FastMagSgnReader<'a> {
    bytes: &'a [u8],
    next: usize,
    reservoir: u64,
    available: u32,
    physical_available: u32,
    previous_was_ff: bool,
    #[cfg(feature = "std")]
    prepared_acceleration: Option<PreparedMagSgnAcceleration>,
}

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
struct PreparedMagSgnAcceleration {
    storage: PreparedMagSgnStorage,
    bit_offset: usize,
    source_data_bits: usize,
    dense: bool,
    dispatch: HtCleanupDispatch,
}

#[cfg(feature = "std")]
impl PreparedMagSgnAcceleration {
    fn is_direct(&self) -> bool {
        self.source_data_bits == DIRECT_MAGSGN_SOURCE_BITS
    }

    fn raw_consumed_bits(&self) -> usize {
        let logical_bits = self.bit_offset.min(self.source_data_bits);
        logical_bits
            + self
                .storage
                .stuffed_bit_offsets
                .partition_point(|&offset| offset < logical_bits)
    }
}

#[cfg(feature = "std")]
#[derive(Debug, Clone, Default)]
pub(super) struct PreparedMagSgnStorage {
    bytes: Vec<u8>,
    stuffed_bit_offsets: Vec<usize>,
}

#[cfg(feature = "std")]
impl PreparedMagSgnStorage {
    pub(super) fn is_empty(&self) -> bool {
        self.bytes.is_empty() && self.stuffed_bit_offsets.is_empty()
    }
}

#[cfg(feature = "std")]
struct PreparedMagSgnBytes {
    storage: PreparedMagSgnStorage,
    source_data_bits: usize,
}

#[cfg(feature = "std")]
pub(super) fn prepared_acceleration_dispatch()
-> Result<Option<HtCleanupDispatch>, HtCleanupBackendError> {
    emuella_j2k_accel::ht_cleanup_dispatch()
        .copied()
        .map(|dispatch| {
            matches!(
                dispatch.backend(),
                HtCleanupBackend::Avx2 | HtCleanupBackend::Avx2Bmi2 | HtCleanupBackend::Neon
            )
            .then_some(dispatch)
        })
}

impl<'a> FastMagSgnReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self::new_with_coefficient_count(bytes, None)
    }

    fn new_with_coefficient_count(bytes: &'a [u8], coefficient_count: Option<usize>) -> Self {
        #[cfg(feature = "std")]
        let prepared_acceleration = emuella_j2k_accel::ht_cleanup_dispatch()
            .ok()
            .filter(|dispatch| {
                matches!(
                    dispatch.backend(),
                    HtCleanupBackend::Avx2 | HtCleanupBackend::Avx2Bmi2 | HtCleanupBackend::Neon
                )
            })
            .map(|dispatch| {
                if dispatch.backend() == HtCleanupBackend::Avx2 {
                    return PreparedMagSgnAcceleration {
                        storage: PreparedMagSgnStorage::default(),
                        bit_offset: 0,
                        source_data_bits: DIRECT_MAGSGN_SOURCE_BITS,
                        dense: false,
                        dispatch: *dispatch,
                    };
                }
                let prepared = prepare_magsgn_bytes(bytes, PreparedMagSgnStorage::default());
                let dense = coefficient_count.is_some_and(|coefficient_count| {
                    is_dense_magsgn(prepared.source_data_bits, coefficient_count)
                });
                PreparedMagSgnAcceleration {
                    storage: prepared.storage,
                    bit_offset: 0,
                    source_data_bits: prepared.source_data_bits,
                    dense,
                    dispatch: *dispatch,
                }
            });
        Self {
            bytes,
            next: 0,
            reservoir: 0,
            available: 0,
            physical_available: 0,
            previous_was_ff: false,
            #[cfg(feature = "std")]
            prepared_acceleration,
        }
    }

    #[cfg(feature = "std")]
    fn new_with_coefficient_count_and_storage(
        bytes: &'a [u8],
        coefficient_count: usize,
        storage: PreparedMagSgnStorage,
        dispatch: HtCleanupDispatch,
    ) -> Self {
        if dispatch.backend() == HtCleanupBackend::Avx2 {
            return Self {
                bytes,
                next: 0,
                reservoir: 0,
                available: 0,
                physical_available: 0,
                previous_was_ff: false,
                prepared_acceleration: Some(PreparedMagSgnAcceleration {
                    storage,
                    bit_offset: 0,
                    source_data_bits: DIRECT_MAGSGN_SOURCE_BITS,
                    dense: false,
                    dispatch,
                }),
            };
        }
        let prepared = prepare_magsgn_bytes(bytes, storage);
        let dense = is_dense_magsgn(prepared.source_data_bits, coefficient_count);
        Self {
            bytes,
            next: 0,
            reservoir: 0,
            available: 0,
            physical_available: 0,
            previous_was_ff: false,
            prepared_acceleration: Some(PreparedMagSgnAcceleration {
                storage: prepared.storage,
                bit_offset: 0,
                source_data_bits: prepared.source_data_bits,
                dense,
                dispatch,
            }),
        }
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn acceleration_enabled(&self) -> bool {
        self.prepared_acceleration.is_some()
    }

    #[cfg(feature = "std")]
    fn direct_acceleration_enabled(&self) -> bool {
        self.prepared_acceleration
            .as_ref()
            .is_some_and(PreparedMagSgnAcceleration::is_direct)
    }

    #[cfg(feature = "std")]
    fn decode_accelerated_codeword_octet(
        &mut self,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> Result<Option<emuella_j2k_accel::HtCleanupOctetOutput>, HtCleanupOctetError> {
        if let Some(dispatch) = self
            .prepared_acceleration
            .as_ref()
            .filter(|prepared| prepared.is_direct())
            .map(|prepared| prepared.dispatch)
        {
            return self
                .decode_direct_codeword_octet(
                    dispatch,
                    first_codeword,
                    second_codeword,
                    first_u,
                    second_u,
                    shift,
                )
                .map(Some);
        }
        let Some(prepared) = self.prepared_acceleration.as_mut() else {
            return Ok(None);
        };
        debug_assert!(!prepared.is_direct());
        let decode = if prepared.dense {
            HtCleanupDispatch::decode_prepared_dense_codeword_octet
        } else {
            HtCleanupDispatch::decode_prepared_codeword_octet
        };
        let output = decode(
            prepared.dispatch,
            &prepared.storage.bytes,
            prepared.bit_offset,
            first_codeword,
            second_codeword,
            first_u,
            second_u,
            shift,
        )?;
        prepared.bit_offset += usize::from(output.consumed_bits);
        Ok(Some(output))
    }

    #[cfg(feature = "std")]
    #[allow(clippy::too_many_arguments)]
    fn decode_direct_codeword_octet(
        &mut self,
        dispatch: HtCleanupDispatch,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> Result<emuella_j2k_accel::HtCleanupOctetOutput, HtCleanupOctetError> {
        let mut deferred = DeferredMagSgnOctetReader::new_on_demand(self);
        let mut predictors = [0_u32; 8];
        let mut negative = 0_u8;
        let mut consumed_bits = 0_u16;

        for (quad, (codeword, u_value)) in [(first_codeword, first_u), (second_codeword, second_u)]
            .into_iter()
            .enumerate()
        {
            for slot in 0_u8..4 {
                let lane = quad * 4 + usize::from(slot);
                let flags = codeword >> slot;
                if flags & 0x10 == 0 {
                    if flags & 0x1100 != 0 {
                        deferred.commit(self);
                        return Err(HtCleanupOctetError::InvalidCodewordFlags);
                    }
                    continue;
                }

                let length = u_value - ((flags >> 12) & 1);
                if length > 16 {
                    deferred.commit(self);
                    return Err(HtCleanupOctetError::InvalidBitLength);
                }
                let raw = deferred.take_u16(self, length);
                let predictor =
                    u32::from(raw) | (u32::from((flags >> 8) & 1) << u32::from(length)) | 1;
                predictors[lane] = predictor;
                negative |= ((raw & 1) as u8) << lane;
                consumed_bits += length;
            }
        }
        deferred.commit(self);

        Ok(emuella_j2k_accel::HtCleanupOctetOutput {
            coefficients: dispatch.reconstruct(predictors, negative, shift),
            predictors,
            consumed_bits,
        })
    }

    #[inline(always)]
    pub(super) fn take(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        #[cfg(feature = "std")]
        self.synchronize_prepared_acceleration();
        self.take_on_demand(count)
    }

    #[inline(always)]
    fn take_on_demand(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        if count > 32 {
            return Err(HtLayoutError::StreamBitReadUnavailable {
                stream: HtCleanupStreamKind::CleanupForward,
                requested_bits: count as usize,
                remaining_bits: self.remaining_bits(),
            });
        }
        while self.available < count {
            self.refill();
        }
        let mask = if count == 32 {
            u64::from(u32::MAX)
        } else if count == 0 {
            0
        } else {
            (1_u64 << count) - 1
        };
        let value = (self.reservoir & mask) as u32;
        self.reservoir >>= count;
        self.available -= count;
        self.physical_available -= count.min(self.physical_available);
        Ok(value)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn synchronize_prepared_acceleration(&mut self) {
        if self.prepared_acceleration.is_some() {
            self.synchronize_prepared_acceleration_slow();
        }
    }

    #[cfg(feature = "std")]
    #[cold]
    #[inline(never)]
    fn synchronize_prepared_acceleration_slow(&mut self) {
        if self
            .prepared_acceleration
            .as_ref()
            .is_some_and(PreparedMagSgnAcceleration::is_direct)
        {
            return;
        }
        let Some(prepared) = self.prepared_acceleration.take() else {
            return;
        };
        debug_assert!(!prepared.is_direct());
        self.replay_prepared_bits(prepared.bit_offset)
            .expect("prepared MagSgn progress must remain readable on demand");
    }

    #[cfg(feature = "std")]
    fn replay_prepared_bits(&mut self, count: usize) -> Result<(), HtLayoutError> {
        let mut remaining = count;
        while remaining != 0 {
            let chunk = remaining.min(32) as u32;
            self.take_on_demand(chunk)?;
            remaining -= chunk as usize;
        }
        Ok(())
    }

    #[cfg(feature = "std")]
    fn advance_prepared_acceleration(&mut self, count: usize) {
        if let Some(prepared) = self.prepared_acceleration.as_mut()
            && !prepared.is_direct()
        {
            prepared.bit_offset += count;
        }
    }

    #[cfg(feature = "std")]
    fn take_prepared_magsgn_storage(&mut self) -> Option<PreparedMagSgnStorage> {
        self.prepared_acceleration
            .take()
            .map(|prepared| prepared.storage)
    }

    #[cfg(test)]
    #[inline(always)]
    fn take_u16(&mut self, count: u16) -> u16 {
        self.take(u32::from(count)).unwrap() as u16
    }

    #[inline(always)]
    fn commit_physical_consumption(&mut self, consumed: u32) {
        self.physical_available -= consumed.min(self.physical_available);
    }

    fn remaining_bits_after_deferred_consumption(&self, consumed: u32) -> usize {
        let physical_available = self.physical_available - consumed.min(self.physical_available);
        let consumed_bits = self
            .next
            .saturating_mul(8)
            .saturating_sub(physical_available as usize);
        self.bytes.len() * 8 - consumed_bits
    }

    fn consumed_bits(&self) -> usize {
        #[cfg(feature = "std")]
        if let Some(prepared) = self.prepared_acceleration.as_ref()
            && !prepared.is_direct()
        {
            return prepared.raw_consumed_bits();
        }
        self.next
            .saturating_mul(8)
            .saturating_sub(self.physical_available as usize)
    }

    pub(super) fn remaining_bits(&self) -> usize {
        self.bytes
            .len()
            .saturating_mul(8)
            .saturating_sub(self.consumed_bits())
    }

    fn consumed_bytes(&self) -> usize {
        self.consumed_bits().div_ceil(8)
    }

    fn remaining_bytes(&self) -> usize {
        self.remaining_bits() / 8
    }

    fn is_byte_aligned(&self) -> bool {
        self.consumed_bits().is_multiple_of(8)
    }

    #[inline(always)]
    fn refill(&mut self) {
        while self.available <= 56 && self.next < self.bytes.len() {
            let byte = self.bytes[self.next];
            self.next += 1;
            let bit_count = if self.previous_was_ff { 7 } else { 8 };
            let mask = (1_u16 << bit_count) - 1;
            self.reservoir |= u64::from(u16::from(byte) & mask) << self.available;
            self.available += bit_count;
            self.physical_available += bit_count;
            self.previous_was_ff = byte == 0xff;
            if self.previous_was_ff {
                return;
            }
        }

        if self.available <= 56 {
            let bit_count = if self.previous_was_ff { 7 } else { 8 };
            let mask = (1_u16 << bit_count) - 1;
            self.reservoir |= u64::from(0xff_u16 & mask) << self.available;
            self.available += bit_count;
            self.previous_was_ff = true;
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct FastMelReader<'a> {
    bytes: &'a [u8],
    next: usize,
    reservoir: u64,
    available: u32,
    physical_loaded_bits: usize,
    previous_was_ff: bool,
    run_state: u8,
    pending_zero_events: u8,
    pending_one_event: bool,
}

impl<'a> FastMelReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            next: 0,
            reservoir: 0,
            available: 0,
            physical_loaded_bits: 0,
            previous_was_ff: false,
            run_state: 0,
            pending_zero_events: 0,
            pending_one_event: false,
        }
    }

    #[inline(always)]
    fn decode_event(&mut self) -> Result<HtMelEvent, HtLayoutError> {
        loop {
            if self.pending_zero_events != 0 {
                self.pending_zero_events -= 1;
                return Ok(HtMelEvent::Zero);
            }
            if self.pending_one_event {
                self.pending_one_event = false;
                return Ok(HtMelEvent::One);
            }

            let exponent = HT_MEL_EXPONENTS[usize::from(self.run_state)];
            if self.take(1)? != 0 {
                self.run_state = (self.run_state + 1).min(12);
                self.pending_zero_events = 1_u8 << exponent;
                self.pending_one_event = false;
            } else {
                self.pending_zero_events = self.take_msb(u32::from(exponent))? as u8;
                self.pending_one_event = true;
                self.run_state = self.run_state.saturating_sub(1);
            }
        }
    }

    #[inline(always)]
    fn take(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        if count > 32 {
            return Err(self.unavailable(count as usize));
        }
        while self.available < count {
            self.refill()?;
        }
        let mask = if count == 0 { 0 } else { (1_u64 << count) - 1 };
        let value = (self.reservoir & mask) as u32;
        self.reservoir >>= count;
        self.available -= count;
        Ok(value)
    }

    #[inline(always)]
    fn take_msb(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        if count > 32 {
            return Err(self.unavailable(count as usize));
        }
        while self.available < count {
            if let Err(error) = self.refill() {
                // The checked MEL oracle reads a run suffix one bit at a time.
                // Preserve that partial-progress contract when a batched fast
                // read reaches truncation or an invalid stuffed byte.
                self.reservoir = 0;
                self.available = 0;
                return Err(match error {
                    HtLayoutError::StreamBitReadUnavailable { .. } => self.unavailable(1),
                    HtLayoutError::StreamStuffedByteInvalid { stream, byte, .. } => {
                        HtLayoutError::StreamStuffedByteInvalid {
                            stream,
                            byte,
                            consumed_bits: self.consumed_bits(),
                        }
                    }
                    error => error,
                });
            }
        }
        let mask = if count == 0 { 0 } else { (1_u64 << count) - 1 };
        let value = (self.reservoir & mask) as u32;
        self.reservoir >>= count;
        self.available -= count;
        if count == 0 {
            Ok(0)
        } else {
            Ok(value.reverse_bits() >> (u32::BITS - count))
        }
    }

    fn consumed_bits(&self) -> usize {
        self.physical_loaded_bits - self.available as usize
    }

    fn remaining_bits(&self) -> usize {
        self.bytes.len() * 8 - self.consumed_bits()
    }

    fn consumed_bytes(&self) -> usize {
        self.consumed_bits().div_ceil(8)
    }

    fn remaining_bytes(&self) -> usize {
        self.remaining_bits() / 8
    }

    fn is_byte_aligned(&self) -> bool {
        self.available == 0
    }

    #[inline(always)]
    fn refill(&mut self) -> Result<(), HtLayoutError> {
        let Some(&byte) = self.bytes.get(self.next) else {
            return Err(self.unavailable(1));
        };
        if self.previous_was_ff && byte > 0x8f {
            return Err(HtLayoutError::StreamStuffedByteInvalid {
                stream: HtCleanupStreamKind::Mel,
                byte,
                consumed_bits: self.consumed_bits(),
            });
        }
        self.next += 1;
        self.physical_loaded_bits += 8;
        let bit_count = if self.previous_was_ff { 7 } else { 8 };
        let bits = if bit_count == 8 {
            byte.reverse_bits()
        } else {
            byte.reverse_bits() >> 1
        };
        self.reservoir |= u64::from(bits) << self.available;
        self.available += bit_count;
        self.previous_was_ff = byte == 0xff;
        Ok(())
    }

    fn unavailable(&self, requested_bits: usize) -> HtLayoutError {
        HtLayoutError::StreamBitReadUnavailable {
            stream: HtCleanupStreamKind::Mel,
            requested_bits,
            remaining_bits: self.remaining_bits(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct FastVlcReader<'a> {
    bytes: &'a [u8],
    next: usize,
    reservoir: u64,
    available: u32,
    raw_bit_limit: usize,
    raw_consumed: usize,
    initial_nibble: bool,
    previous_byte: u8,
}

impl<'a> FastVlcReader<'a> {
    pub(super) fn new(mel_vlc: &'a [u8]) -> Result<Self, HtLayoutError> {
        if mel_vlc.len() < 2 {
            return Err(HtLayoutError::InvalidCleanupPassSegment {
                lcup: mel_vlc.len(),
                scup: mel_vlc.len(),
            });
        }
        let bytes = &mel_vlc[..mel_vlc.len() - 1];
        Ok(Self {
            bytes,
            next: bytes.len(),
            reservoir: 0,
            available: 0,
            raw_bit_limit: (mel_vlc.len() - 2) * 8 + 4,
            raw_consumed: 0,
            initial_nibble: true,
            previous_byte: 0,
        })
    }

    pub(super) fn consumed_bits(&self) -> usize {
        self.raw_consumed
    }

    pub(super) fn remaining_bits(&self) -> usize {
        self.raw_bit_limit.saturating_sub(self.raw_consumed)
    }

    pub(super) fn consumed_bytes(&self) -> usize {
        self.raw_consumed.div_ceil(8)
    }

    pub(super) fn remaining_bytes(&self) -> usize {
        self.remaining_bits() / 8
    }

    pub(super) fn is_byte_aligned(&self) -> bool {
        self.raw_consumed == 0
            || (self.raw_consumed >= 4 && (self.raw_consumed - 4).is_multiple_of(8))
    }

    #[inline(always)]
    pub(super) fn take(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        if count > 32 {
            return Err(self.unavailable(count as usize));
        }
        self.ensure(count)?;
        let mask = if count == 32 {
            u32::MAX as u64
        } else if count == 0 {
            0
        } else {
            (1_u64 << count) - 1
        };
        let value = (self.reservoir & mask) as u32;
        self.reservoir >>= count;
        self.available -= count;
        Ok(value)
    }

    #[inline(always)]
    pub(super) fn decode_codeword(
        &mut self,
        table: HtVlcLookupTable<'_>,
        context: HtVlcContext,
        zero_context_mel_event: Option<bool>,
    ) -> Result<HtVlcQuadCodeword, HtLayoutError> {
        if context.get() == 0 && zero_context_mel_event == Some(false) {
            return Ok(HtVlcQuadCodeword::ZERO);
        }
        while self.available < 7 && self.next != 0 {
            self.refill()?;
        }
        // The table is expanded to seven-bit prefixes, but a final VLC
        // codeword may need fewer physical bits. Zero-padding is safe for the
        // lookup only; the selected codeword remains bounded by `available`.
        let prefix = (self.reservoir & 0x7f) as u8;
        let mut codeword = table.lookup(context, prefix);
        if context.get() == 0
            && let Some(mel_event) = zero_context_mel_event
        {
            codeword = codeword.gated_by_zero_context_mel_event(mel_event);
        }
        let consumed_bits = u32::from(codeword.consumed_bits());
        debug_assert!(consumed_bits <= 7);
        if consumed_bits > self.available {
            return Err(self.unavailable(consumed_bits as usize));
        }
        self.consume_codeword(consumed_bits);
        Ok(codeword)
    }

    #[inline(always)]
    pub(super) fn decode_codeword_steady(
        &mut self,
        table: HtVlcLookupTable<'_>,
        context: HtVlcContext,
        zero_context_mel_event: Option<bool>,
    ) -> Result<HtVlcQuadCodeword, HtLayoutError> {
        debug_assert!(!self.initial_nibble);
        if context.get() == 0 && zero_context_mel_event == Some(false) {
            return Ok(HtVlcQuadCodeword::ZERO);
        }
        while self.available < 7 && self.next != 0 {
            self.refill_steady()?;
        }
        // Match the checked tail behaviour above on steady-state line pairs.
        let prefix = (self.reservoir & 0x7f) as u8;
        let mut codeword = table.lookup(context, prefix);
        if context.get() == 0
            && let Some(mel_event) = zero_context_mel_event
        {
            codeword = codeword.gated_by_zero_context_mel_event(mel_event);
        }
        let consumed_bits = u32::from(codeword.consumed_bits());
        debug_assert!(consumed_bits <= 7);
        if consumed_bits > self.available {
            return Err(self.unavailable(consumed_bits as usize));
        }
        self.consume_codeword(consumed_bits);
        Ok(codeword)
    }

    pub(super) fn decode_single_uvlc(
        &mut self,
        codeword: HtVlcQuadCodeword,
    ) -> Result<u16, HtLayoutError> {
        if !codeword.u_offset() {
            return Ok(1);
        }
        Ok(self.decode_uvlc_value()? + 1)
    }

    #[inline(always)]
    pub(super) fn decode_initial_uvlc_pair(
        &mut self,
        mode: HtVlcInitialUvlcMode,
    ) -> Result<HtVlcUvlcPair, HtLayoutError> {
        match mode {
            HtVlcInitialUvlcMode::BothZero => Ok(HtVlcUvlcPair {
                first: 1,
                second: 1,
                consumed_bits: 0,
            }),
            HtVlcInitialUvlcMode::FirstUsesUvlc => {
                let prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = prefix.consumed_bits();
                let first = self.finish_uvlc_value(prefix)? + 1;
                Ok(Self::pair(first, 1, consumed_bits))
            }
            HtVlcInitialUvlcMode::SecondUsesUvlc => {
                let prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = prefix.consumed_bits();
                let second = self.finish_uvlc_value(prefix)? + 1;
                Ok(Self::pair(1, second, consumed_bits))
            }
            HtVlcInitialUvlcMode::BothUseUvlc { mel_event: false } => {
                self.decode_initial_both_zero_mel_uvlc_pair()
            }
            HtVlcInitialUvlcMode::BothUseUvlc { mel_event: true } => {
                let first_prefix = self.decode_uvlc_prefix()?;
                let second_prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = first_prefix.consumed_bits() + second_prefix.consumed_bits();
                let first = self.finish_uvlc_value(first_prefix)? + 3;
                let second = self.finish_uvlc_value(second_prefix)? + 3;
                Ok(Self::pair(first, second, consumed_bits))
            }
        }
    }

    #[inline(always)]
    pub(super) fn decode_noninitial_uvlc_pair(
        &mut self,
        mode: HtVlcNonInitialUvlcMode,
    ) -> Result<HtVlcUvlcPair, HtLayoutError> {
        match mode {
            HtVlcNonInitialUvlcMode::BothZero => Ok(HtVlcUvlcPair {
                first: 1,
                second: 1,
                consumed_bits: 0,
            }),
            HtVlcNonInitialUvlcMode::FirstUsesUvlc => {
                let prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = prefix.consumed_bits();
                let first = self.finish_uvlc_value(prefix)? + 1;
                Ok(Self::pair(first, 1, consumed_bits))
            }
            HtVlcNonInitialUvlcMode::SecondUsesUvlc => {
                let prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = prefix.consumed_bits();
                let second = self.finish_uvlc_value(prefix)? + 1;
                Ok(Self::pair(1, second, consumed_bits))
            }
            HtVlcNonInitialUvlcMode::BothUseUvlc => {
                let first_prefix = self.decode_uvlc_prefix()?;
                let second_prefix = self.decode_uvlc_prefix()?;
                let consumed_bits = first_prefix.consumed_bits() + second_prefix.consumed_bits();
                let first = self.finish_uvlc_value(first_prefix)? + 1;
                let second = self.finish_uvlc_value(second_prefix)? + 1;
                Ok(Self::pair(first, second, consumed_bits))
            }
        }
    }

    #[inline(always)]
    pub(super) fn decode_noninitial_uvlc_pair_steady(
        &mut self,
        mode: HtVlcNonInitialUvlcMode,
    ) -> Result<HtVlcUvlcPair, HtLayoutError> {
        debug_assert!(!self.initial_nibble);
        match mode {
            HtVlcNonInitialUvlcMode::BothZero => Ok(HtVlcUvlcPair {
                first: 1,
                second: 1,
                consumed_bits: 0,
            }),
            HtVlcNonInitialUvlcMode::FirstUsesUvlc => {
                let prefix = self.decode_uvlc_prefix_steady()?;
                let consumed_bits = prefix.consumed_bits();
                let first = self.finish_uvlc_value_steady(prefix)? + 1;
                Ok(Self::pair(first, 1, consumed_bits))
            }
            HtVlcNonInitialUvlcMode::SecondUsesUvlc => {
                let prefix = self.decode_uvlc_prefix_steady()?;
                let consumed_bits = prefix.consumed_bits();
                let second = self.finish_uvlc_value_steady(prefix)? + 1;
                Ok(Self::pair(1, second, consumed_bits))
            }
            HtVlcNonInitialUvlcMode::BothUseUvlc => {
                let first_prefix = self.decode_uvlc_prefix_steady()?;
                let second_prefix = self.decode_uvlc_prefix_steady()?;
                let consumed_bits = first_prefix.consumed_bits() + second_prefix.consumed_bits();
                let first = self.finish_uvlc_value_steady(first_prefix)? + 1;
                let second = self.finish_uvlc_value_steady(second_prefix)? + 1;
                Ok(Self::pair(first, second, consumed_bits))
            }
        }
    }

    #[inline(always)]
    fn decode_initial_both_zero_mel_uvlc_pair(&mut self) -> Result<HtVlcUvlcPair, HtLayoutError> {
        let first_prefix = self.decode_uvlc_prefix()?;
        let first_consumed_bits = first_prefix.consumed_bits();
        let (first, second, consumed_bits) = if first_prefix.prefix_bits > 2 {
            let second_prefix_value = self.take(1)? as u16;
            let first = self.finish_uvlc_value(first_prefix)? + 1;
            (first, second_prefix_value + 2, first_consumed_bits + 1)
        } else {
            let second_prefix = self.decode_uvlc_prefix()?;
            let consumed_bits = first_consumed_bits + second_prefix.consumed_bits();
            let first = self.finish_uvlc_value(first_prefix)? + 1;
            let second = self.finish_uvlc_value(second_prefix)? + 1;
            (first, second, consumed_bits)
        };
        Ok(Self::pair(first, second, consumed_bits))
    }

    #[inline(always)]
    fn decode_uvlc_value(&mut self) -> Result<u16, HtLayoutError> {
        let prefix = self.decode_uvlc_prefix()?;
        self.finish_uvlc_value(prefix)
    }

    #[inline(always)]
    fn decode_uvlc_prefix(&mut self) -> Result<FastUvlcPrefix, HtLayoutError> {
        if self.available >= 3 {
            let prefix = FAST_UVLC_PREFIXES[(self.reservoir & 0x7) as usize];
            self.consume_available(u32::from(prefix.prefix_bits));
            return Ok(prefix);
        }
        self.decode_uvlc_prefix_tail()
    }

    #[inline(always)]
    fn decode_uvlc_prefix_steady(&mut self) -> Result<FastUvlcPrefix, HtLayoutError> {
        if self.available >= 3 {
            let prefix = FAST_UVLC_PREFIXES[(self.reservoir & 0x7) as usize];
            self.consume_available(u32::from(prefix.prefix_bits));
            return Ok(prefix);
        }
        self.decode_uvlc_prefix_tail_steady()
    }

    // Reservoir-boundary prefixes are frequent on full blocks, but inlining
    // this path duplicates refill and error handling in the octet loop.
    #[inline(never)]
    fn decode_uvlc_prefix_tail(&mut self) -> Result<FastUvlcPrefix, HtLayoutError> {
        self.ensure(1)?;
        if self.reservoir & 1 != 0 {
            self.consume_available(1);
            return Ok(FastUvlcPrefix::new(1, 1, 0));
        }
        self.ensure(2)?;
        if self.reservoir & 2 != 0 {
            self.consume_available(2);
            return Ok(FastUvlcPrefix::new(2, 2, 0));
        }
        self.ensure(3)?;
        let third_bit = self.reservoir & 4 != 0;
        self.consume_available(3);
        if third_bit {
            return Ok(FastUvlcPrefix::new(3, 3, 1));
        }
        Ok(FastUvlcPrefix::new(5, 3, 5))
    }

    #[inline(never)]
    fn decode_uvlc_prefix_tail_steady(&mut self) -> Result<FastUvlcPrefix, HtLayoutError> {
        self.ensure_steady(1)?;
        if self.reservoir & 1 != 0 {
            self.consume_available(1);
            return Ok(FastUvlcPrefix::new(1, 1, 0));
        }
        self.ensure_steady(2)?;
        if self.reservoir & 2 != 0 {
            self.consume_available(2);
            return Ok(FastUvlcPrefix::new(2, 2, 0));
        }
        self.ensure_steady(3)?;
        let third_bit = self.reservoir & 4 != 0;
        self.consume_available(3);
        if third_bit {
            return Ok(FastUvlcPrefix::new(3, 3, 1));
        }
        Ok(FastUvlcPrefix::new(5, 3, 5))
    }

    #[inline(always)]
    fn finish_uvlc_value(&mut self, prefix: FastUvlcPrefix) -> Result<u16, HtLayoutError> {
        if prefix.suffix_bits == 0 {
            Ok(prefix.prefix_value)
        } else {
            Ok(prefix.prefix_value + self.take(u32::from(prefix.suffix_bits))? as u16)
        }
    }

    #[inline(always)]
    fn finish_uvlc_value_steady(&mut self, prefix: FastUvlcPrefix) -> Result<u16, HtLayoutError> {
        if prefix.suffix_bits == 0 {
            Ok(prefix.prefix_value)
        } else {
            self.ensure_steady(u32::from(prefix.suffix_bits))?;
            let count = u32::from(prefix.suffix_bits);
            let mask = (1_u64 << count) - 1;
            let suffix = (self.reservoir & mask) as u16;
            self.consume_available(count);
            Ok(prefix.prefix_value + suffix)
        }
    }

    #[inline(always)]
    fn consume_available(&mut self, count: u32) {
        debug_assert!(self.available >= count);
        self.reservoir >>= count;
        self.available -= count;
    }

    #[inline(always)]
    fn consume_codeword(&mut self, count: u32) {
        debug_assert!(self.available >= count);
        self.reservoir >>= count;
        self.available -= count;
    }

    #[inline(always)]
    const fn pair(first: u16, second: u16, consumed_bits: u8) -> HtVlcUvlcPair {
        HtVlcUvlcPair {
            first,
            second,
            consumed_bits,
        }
    }

    #[inline(always)]
    fn ensure(&mut self, count: u32) -> Result<(), HtLayoutError> {
        if count > 32 {
            return Err(self.unavailable(count as usize));
        }
        while self.available < count {
            self.refill()?;
        }
        Ok(())
    }

    #[inline(always)]
    fn ensure_steady(&mut self, count: u32) -> Result<(), HtLayoutError> {
        debug_assert!(!self.initial_nibble);
        if count > 32 {
            return Err(self.unavailable(count as usize));
        }
        while self.available < count {
            self.refill_steady()?;
        }
        Ok(())
    }

    #[inline(always)]
    fn refill(&mut self) -> Result<(), HtLayoutError> {
        let consumed_before = self.raw_consumed;
        if self.next == 0 {
            return Err(self.unavailable(if self.initial_nibble { 4 } else { 8 }));
        }
        self.next -= 1;
        let byte = self.bytes[self.next];

        let (bits, bit_count, raw_bits) = if self.initial_nibble {
            self.initial_nibble = false;
            let nibble = byte >> 4;
            self.previous_byte = byte | 0x0f;
            (nibble, 4 - u32::from((nibble & 0x7) == 0x7), 4)
        } else {
            let bit_count = if self.previous_byte > 0x8f {
                match byte {
                    0x7f => 7,
                    0xff => {
                        return Err(HtLayoutError::StreamStuffedByteInvalid {
                            stream: HtCleanupStreamKind::Vlc,
                            byte,
                            consumed_bits: consumed_before,
                        });
                    }
                    _ => 8,
                }
            } else {
                8
            };
            self.previous_byte = byte;
            (byte, bit_count, 8)
        };

        self.raw_consumed += raw_bits;
        let data_mask = ((1_u16 << bit_count) - 1) as u8;
        self.reservoir |= u64::from(bits & data_mask) << self.available;
        self.available += bit_count;
        Ok(())
    }

    #[inline(always)]
    fn refill_steady(&mut self) -> Result<(), HtLayoutError> {
        debug_assert!(!self.initial_nibble);
        let consumed_before = self.raw_consumed;
        if self.next == 0 {
            return Err(self.unavailable(8));
        }
        self.next -= 1;
        let byte = self.bytes[self.next];
        let bit_count = if self.previous_byte > 0x8f {
            match byte {
                0x7f => 7,
                0xff => {
                    return Err(HtLayoutError::StreamStuffedByteInvalid {
                        stream: HtCleanupStreamKind::Vlc,
                        byte,
                        consumed_bits: consumed_before,
                    });
                }
                _ => 8,
            }
        } else {
            8
        };
        self.previous_byte = byte;
        self.raw_consumed += 8;
        let data_mask = ((1_u16 << bit_count) - 1) as u8;
        self.reservoir |= u64::from(byte & data_mask) << self.available;
        self.available += bit_count;
        Ok(())
    }

    fn unavailable(&self, requested_bits: usize) -> HtLayoutError {
        HtLayoutError::StreamBitReadUnavailable {
            stream: HtCleanupStreamKind::Vlc,
            requested_bits,
            remaining_bits: self.remaining_bits(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FastUvlcPrefix {
    prefix_value: u16,
    prefix_bits: u8,
    suffix_bits: u8,
}

impl FastUvlcPrefix {
    const fn new(prefix_value: u16, prefix_bits: u8, suffix_bits: u8) -> Self {
        Self {
            prefix_value,
            prefix_bits,
            suffix_bits,
        }
    }

    const fn consumed_bits(self) -> u8 {
        self.prefix_bits + self.suffix_bits
    }
}

const FAST_UVLC_PREFIXES: [FastUvlcPrefix; 8] = [
    FastUvlcPrefix::new(5, 3, 5),
    FastUvlcPrefix::new(1, 1, 0),
    FastUvlcPrefix::new(2, 2, 0),
    FastUvlcPrefix::new(1, 1, 0),
    FastUvlcPrefix::new(3, 3, 1),
    FastUvlcPrefix::new(1, 1, 0),
    FastUvlcPrefix::new(2, 2, 0),
    FastUvlcPrefix::new(1, 1, 0),
];

#[inline(always)]
pub(super) fn decode_zero_context_event(
    mel: &mut FastMelReader<'_>,
    context: HtVlcContext,
) -> Result<Option<bool>, HtLayoutError> {
    if context.get() == 0 {
        Ok(Some(mel.decode_event()? == HtMelEvent::One))
    } else {
        Ok(None)
    }
}

#[inline(always)]
pub(super) fn decode_uvlc_pair(
    reader: &mut FastVlcReader<'_>,
    mode: HtVlcContextProgressionMode,
    mel: &mut FastMelReader<'_>,
    first_codeword: HtVlcQuadCodeword,
    second_codeword: HtVlcQuadCodeword,
) -> Result<HtVlcUvlcPair, HtLayoutError> {
    match mode {
        HtVlcContextProgressionMode::Initial => {
            let both_offsets_mel_event = if first_codeword.u_offset() && second_codeword.u_offset()
            {
                mel.decode_event()? == HtMelEvent::One
            } else {
                false
            };
            reader.decode_initial_uvlc_pair(HtVlcInitialUvlcMode::from_u_offsets(
                first_codeword.u_offset(),
                second_codeword.u_offset(),
                both_offsets_mel_event,
            ))
        }
        HtVlcContextProgressionMode::NonInitial => {
            reader.decode_noninitial_uvlc_pair(HtVlcNonInitialUvlcMode::from_u_offsets(
                first_codeword.u_offset(),
                second_codeword.u_offset(),
            ))
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
pub(super) fn decode_full_octet(
    magnitude_sign: &mut FastMagSgnReader<'_>,
    first_codeword: HtVlcQuadCodeword,
    second_codeword: HtVlcQuadCodeword,
    first_u: u16,
    second_u: u16,
    cleanup_shift: u32,
    top_output: &mut [i32; 4],
    bottom_output: &mut [i32; 4],
    predictors: &mut [u32; 3],
) -> Result<(), HtLayoutError> {
    #[cfg(feature = "std")]
    if magnitude_sign.acceleration_enabled()
        && decode_full_octet_accelerated_into(
            magnitude_sign,
            first_codeword,
            second_codeword,
            first_u,
            second_u,
            cleanup_shift,
            top_output,
            bottom_output,
            predictors,
        )?
    {
        return Ok(());
    }

    if uniform_full_octet_width::<13>(first_codeword, second_codeword, first_u, second_u) {
        return decode_uniform_full_octet::<13>(
            magnitude_sign,
            first_codeword,
            second_codeword,
            cleanup_shift,
            top_output,
            bottom_output,
            predictors,
        );
    }
    if uniform_full_octet_width::<7>(first_codeword, second_codeword, first_u, second_u) {
        return decode_uniform_full_octet::<7>(
            magnitude_sign,
            first_codeword,
            second_codeword,
            cleanup_shift,
            top_output,
            bottom_output,
            predictors,
        );
    }

    let mut deferred_magnitude_sign = DeferredMagSgnOctetReader::new_on_demand(magnitude_sign);
    macro_rules! decode_sample {
        ($codeword:expr, $u_value:expr, $slot:expr) => {
            match decode_full_sample(
                magnitude_sign,
                &mut deferred_magnitude_sign,
                $codeword,
                $u_value,
                $slot,
                cleanup_shift,
            ) {
                Ok(sample) => sample,
                Err(error) => {
                    deferred_magnitude_sign.commit(magnitude_sign);
                    return Err(error);
                }
            }
        };
    }
    let first_0 = decode_sample!(first_codeword, first_u, 0);
    let first_1 = decode_sample!(first_codeword, first_u, 1);
    let first_2 = decode_sample!(first_codeword, first_u, 2);
    let first_3 = decode_sample!(first_codeword, first_u, 3);
    let second_0 = decode_sample!(second_codeword, second_u, 0);
    let second_1 = decode_sample!(second_codeword, second_u, 1);
    let second_2 = decode_sample!(second_codeword, second_u, 2);
    let second_3 = decode_sample!(second_codeword, second_u, 3);
    deferred_magnitude_sign.commit(magnitude_sign);

    *top_output = [
        first_0.coefficient,
        first_2.coefficient,
        second_0.coefficient,
        second_2.coefficient,
    ];
    *bottom_output = [
        first_1.coefficient,
        first_3.coefficient,
        second_1.coefficient,
        second_3.coefficient,
    ];

    predictors[0] |= first_1.predictor;
    predictors[1] = first_3.predictor | second_1.predictor;
    predictors[2] |= second_3.predictor;
    Ok(())
}

#[inline(always)]
fn uniform_full_octet_width<const WIDTH: u16>(
    first: HtVlcQuadCodeword,
    second: HtVlcQuadCodeword,
    first_u: u16,
    second_u: u16,
) -> bool {
    #[inline(always)]
    fn quad_matches<const WIDTH: u16>(codeword: HtVlcQuadCodeword, u_value: u16) -> bool {
        codeword.significance_bits() == 0x0f
            && ((u_value == WIDTH && codeword.magnitude_exponent_reduction_bits() == 0)
                || (u_value == WIDTH + 1 && codeword.magnitude_exponent_reduction_bits() == 0x0f))
    }

    quad_matches::<WIDTH>(first, first_u) && quad_matches::<WIDTH>(second, second_u)
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn decode_uniform_full_octet<const WIDTH: u16>(
    magnitude_sign: &mut FastMagSgnReader<'_>,
    first: HtVlcQuadCodeword,
    second: HtVlcQuadCodeword,
    cleanup_shift: u32,
    top_output: &mut [i32; 4],
    bottom_output: &mut [i32; 4],
    predictors: &mut [u32; 3],
) -> Result<(), HtLayoutError> {
    debug_assert!(matches!(WIDTH, 7 | 13));
    let mut local = UniformMagSgnOctetReader::new_on_demand(magnitude_sign);
    let octet = local.take_octet::<WIDTH>();
    local.commit(magnitude_sign);
    let sample_mask = (1_u64 << u32::from(WIDTH)) - 1;

    macro_rules! decode_sample {
        ($packed:expr, $codeword:expr, $slot:expr) => {{
            let value = (($packed >> (u32::from(WIDTH) * $slot)) & sample_mask) as u16;
            let embedded = (($codeword.raw() >> (8 + $slot)) & 1) as u32;
            let mut magnitude_code = u32::from(value);
            magnitude_code |= embedded << u32::from(WIDTH);
            magnitude_code |= 1;
            let scaled = (magnitude_code + 2).wrapping_shl(cleanup_shift);
            let magnitude = (scaled & 0x7fff_ffff) as i32;
            FastDecodedSample {
                coefficient: if value & 1 != 0 {
                    -magnitude
                } else {
                    magnitude
                },
                predictor: magnitude_code,
            }
        }};
    }

    let first_0 = decode_sample!(octet.first_quad, first, 0);
    let first_1 = decode_sample!(octet.first_quad, first, 1);
    let first_2 = decode_sample!(octet.first_quad, first, 2);
    let first_3 = decode_sample!(octet.first_quad, first, 3);
    let second_0 = decode_sample!(octet.second_quad, second, 0);
    let second_1 = decode_sample!(octet.second_quad, second, 1);
    let second_2 = decode_sample!(octet.second_quad, second, 2);
    let second_3 = decode_sample!(octet.second_quad, second, 3);

    *top_output = [
        first_0.coefficient,
        first_2.coefficient,
        second_0.coefficient,
        second_2.coefficient,
    ];
    *bottom_output = [
        first_1.coefficient,
        first_3.coefficient,
        second_1.coefficient,
        second_3.coefficient,
    ];
    predictors[0] |= first_1.predictor;
    predictors[1] = first_3.predictor | second_1.predictor;
    predictors[2] |= second_3.predictor;
    Ok(())
}

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn decode_full_octet_accelerated_into(
    magnitude_sign: &mut FastMagSgnReader<'_>,
    first: HtVlcQuadCodeword,
    second: HtVlcQuadCodeword,
    first_u: u16,
    second_u: u16,
    shift: u32,
    top_output: &mut [i32; 4],
    bottom_output: &mut [i32; 4],
    predictors: &mut [u32; 3],
) -> Result<bool, HtLayoutError> {
    if magnitude_sign.direct_acceleration_enabled()
        && (uniform_full_octet_width::<13>(first, second, first_u, second_u)
            || uniform_full_octet_width::<7>(first, second, first_u, second_u))
    {
        return Ok(false);
    }
    let decoded = match magnitude_sign.decode_accelerated_codeword_octet(
        first.raw(),
        second.raw(),
        first_u,
        second_u,
        shift,
    ) {
        Ok(Some(output)) => output,
        Ok(None) => return Ok(false),
        Err(HtCleanupOctetError::InvalidCodewordFlags) => {
            return Err(HtLayoutError::InvalidVlcCleanupOutput {
                reason: "insignificant coefficient declares magnitude/sign data",
            });
        }
        Err(HtCleanupOctetError::InvalidBitLength) => {
            return Err(accelerated_octet_bit_length_error(
                magnitude_sign,
                first,
                second,
                first_u,
                second_u,
            ));
        }
        Err(HtCleanupOctetError::InputTooShort) => return Err(HtLayoutError::SizeOverflow),
    };

    let coefficients = decoded.coefficients;
    *top_output = [
        coefficients[0],
        coefficients[2],
        coefficients[4],
        coefficients[6],
    ];
    *bottom_output = [
        coefficients[1],
        coefficients[3],
        coefficients[5],
        coefficients[7],
    ];

    let decoded_predictors = decoded.predictors;
    predictors[0] |= decoded_predictors[1];
    predictors[1] = decoded_predictors[3] | decoded_predictors[5];
    predictors[2] |= decoded_predictors[7];
    Ok(true)
}

#[cold]
#[inline(never)]
fn accelerated_octet_bit_length_error(
    magnitude_sign: &mut FastMagSgnReader<'_>,
    first: HtVlcQuadCodeword,
    second: HtVlcQuadCodeword,
    first_u: u16,
    second_u: u16,
) -> HtLayoutError {
    let significance = first.significance_bits() | (second.significance_bits() << 4);
    let reductions = first.magnitude_exponent_reduction_bits()
        | (second.magnitude_exponent_reduction_bits() << 4);
    let mut prior_bits = 0_usize;
    for lane in 0..8 {
        if significance & (1 << lane) == 0 {
            continue;
        }
        let u_value = if lane < 4 { first_u } else { second_u };
        let reduction = u16::from(reductions >> lane & 1);
        let Some(bits) = u_value.checked_sub(reduction) else {
            return HtLayoutError::SizeOverflow;
        };
        let Ok(length) = u8::try_from(bits) else {
            return HtLayoutError::SizeOverflow;
        };
        if length > 16 {
            magnitude_sign.advance_prepared_acceleration(prior_bits);
            magnitude_sign.synchronize_prepared_acceleration();
            return HtLayoutError::StreamBitReadUnavailable {
                stream: HtCleanupStreamKind::CleanupForward,
                requested_bits: usize::from(length),
                remaining_bits: magnitude_sign.remaining_bits(),
            };
        }
        prior_bits += usize::from(length);
    }

    HtLayoutError::InvalidVlcCleanupOutput {
        reason: "accelerated magnitude/sign validation mismatch",
    }
}

#[derive(Debug, Clone, Copy)]
struct FastDecodedSample {
    coefficient: i32,
    predictor: u32,
}

#[derive(Debug, Clone, Copy)]
struct DeferredMagSgnOctetReader {
    reservoir: u64,
    available: u32,
    physical_consumed: u32,
}

#[derive(Debug, Clone, Copy)]
struct UniformMagSgnOctetReader<'a> {
    bytes: &'a [u8],
    next: usize,
    reservoir: u64,
    available: u32,
    physical_available: u32,
    previous_was_ff: bool,
}

#[derive(Debug, Clone, Copy)]
struct UniformMagSgnOctet {
    first_quad: u64,
    second_quad: u64,
}

impl<'a> UniformMagSgnOctetReader<'a> {
    #[cfg(test)]
    #[inline(always)]
    fn new(reader: &mut FastMagSgnReader<'a>) -> Self {
        #[cfg(feature = "std")]
        reader.synchronize_prepared_acceleration();
        Self::new_on_demand(reader)
    }

    #[inline(always)]
    fn new_on_demand(reader: &FastMagSgnReader<'a>) -> Self {
        Self {
            bytes: reader.bytes,
            next: reader.next,
            reservoir: reader.reservoir,
            available: reader.available,
            physical_available: reader.physical_available,
            previous_was_ff: reader.previous_was_ff,
        }
    }

    #[inline(always)]
    fn take_octet<const WIDTH: u16>(&mut self) -> UniformMagSgnOctet {
        const { assert!(matches!(WIDTH, 7 | 13)) };
        if WIDTH == 7 {
            let packed = self.take_word(56);
            UniformMagSgnOctet {
                first_quad: packed,
                second_quad: packed >> 28,
            }
        } else {
            UniformMagSgnOctet {
                first_quad: self.take_word(52),
                second_quad: self.take_word(52),
            }
        }
    }

    #[inline(always)]
    fn take_word(&mut self, count: u32) -> u64 {
        debug_assert!(count < 64);
        while self.available < count {
            self.refill();
        }
        let mask = (1_u64 << count) - 1;
        let value = self.reservoir & mask;
        self.reservoir >>= count;
        self.available -= count;
        self.physical_available -= count.min(self.physical_available);
        value
    }

    #[inline(always)]
    fn refill(&mut self) {
        while self.available <= 56 && self.next < self.bytes.len() {
            let byte = self.bytes[self.next];
            self.next += 1;
            let bit_count = if self.previous_was_ff { 7 } else { 8 };
            let mask = (1_u16 << bit_count) - 1;
            self.reservoir |= u64::from(u16::from(byte) & mask) << self.available;
            self.available += bit_count;
            self.physical_available += bit_count;
            self.previous_was_ff = byte == 0xff;
            if self.previous_was_ff {
                return;
            }
        }

        if self.available <= 56 {
            let bit_count = if self.previous_was_ff { 7 } else { 8 };
            let mask = (1_u16 << bit_count) - 1;
            self.reservoir |= u64::from(0xff_u16 & mask) << self.available;
            self.available += bit_count;
            self.previous_was_ff = true;
        }
    }

    #[inline(always)]
    fn commit(self, reader: &mut FastMagSgnReader<'a>) {
        reader.next = self.next;
        reader.reservoir = self.reservoir;
        reader.available = self.available;
        reader.physical_available = self.physical_available;
        reader.previous_was_ff = self.previous_was_ff;
    }
}

impl DeferredMagSgnOctetReader {
    #[cfg(test)]
    #[inline(always)]
    fn new(reader: &mut FastMagSgnReader<'_>) -> Self {
        #[cfg(feature = "std")]
        reader.synchronize_prepared_acceleration();
        Self::new_on_demand(reader)
    }

    #[inline(always)]
    fn new_on_demand(reader: &FastMagSgnReader<'_>) -> Self {
        Self {
            reservoir: reader.reservoir,
            available: reader.available,
            physical_consumed: 0,
        }
    }

    #[inline(always)]
    fn take_u16(&mut self, reader: &mut FastMagSgnReader<'_>, count: u16) -> u16 {
        debug_assert!(count <= 16);
        let count = u32::from(count);
        while self.available < count {
            reader.reservoir = self.reservoir;
            reader.available = self.available;
            reader.refill();
            self.reservoir = reader.reservoir;
            self.available = reader.available;
        }
        let mask = (1_u64 << count) - 1;
        let value = (self.reservoir & mask) as u16;
        self.reservoir >>= count;
        self.available -= count;
        self.physical_consumed += count;
        value
    }

    #[inline(always)]
    fn commit(self, reader: &mut FastMagSgnReader<'_>) {
        reader.reservoir = self.reservoir;
        reader.available = self.available;
        reader.commit_physical_consumption(self.physical_consumed);
    }
}

#[inline(always)]
fn decode_full_sample(
    magnitude_sign: &mut FastMagSgnReader<'_>,
    deferred: &mut DeferredMagSgnOctetReader,
    codeword: HtVlcQuadCodeword,
    u_value: u16,
    slot: u8,
    cleanup_shift: u32,
) -> Result<FastDecodedSample, HtLayoutError> {
    let flags = codeword.raw() >> slot;
    if flags & 0x10 == 0 {
        if flags & 0x1100 != 0 {
            return Err(HtLayoutError::InvalidVlcCleanupOutput {
                reason: "insignificant coefficient declares magnitude/sign data",
            });
        }
        return Ok(FastDecodedSample {
            coefficient: 0,
            predictor: 0,
        });
    }

    debug_assert!(u_value >= 1);
    let magnitude_sign_bits = u_value - ((flags >> 12) & 1);
    if magnitude_sign_bits > 16 {
        return Err(HtLayoutError::StreamBitReadUnavailable {
            stream: HtCleanupStreamKind::CleanupForward,
            requested_bits: usize::from(magnitude_sign_bits),
            remaining_bits: magnitude_sign
                .remaining_bits_after_deferred_consumption(deferred.physical_consumed),
        });
    }
    let value = deferred.take_u16(magnitude_sign, magnitude_sign_bits);
    let mut magnitude_code = u32::from(value);
    magnitude_code |= u32::from((flags >> 8) & 1) << u32::from(magnitude_sign_bits);
    magnitude_code |= 1;
    let scaled = (magnitude_code + 2).wrapping_shl(cleanup_shift);
    let sign_magnitude = (u32::from(value & 1) << 31) | scaled;
    let magnitude = (sign_magnitude & 0x7fff_ffff) as i32;
    let coefficient = if sign_magnitude & (1_u32 << 31) != 0 {
        -magnitude
    } else {
        magnitude
    };
    Ok(FastDecodedSample {
        coefficient,
        predictor: magnitude_code,
    })
}

pub(super) fn benchmark_reads(
    mel_vlc: &[u8],
    widths: &[u8],
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let mut reader = FastVlcReader::new(mel_vlc)?;
    let mut checksum = 0_u64;
    for (index, &width) in widths.iter().enumerate() {
        let value = reader.take(u32::from(width))?;
        checksum = checksum.rotate_left(7) ^ u64::from(value) ^ index as u64;
    }
    Ok(crate::HtCleanupReaderBenchResult {
        checksum,
        read_count: widths.len(),
        consumed_bits: reader.consumed_bits(),
    })
}

#[derive(Debug)]
struct PredestuffedBitReader<'a> {
    words: &'a [u64],
    bit_len: usize,
    consumed: usize,
    stream: HtCleanupStreamKind,
}

impl<'a> PredestuffedBitReader<'a> {
    fn new(words: &'a [u64], bit_len: usize, stream: HtCleanupStreamKind) -> Self {
        Self {
            words,
            bit_len,
            consumed: 0,
            stream,
        }
    }

    #[inline(always)]
    fn take(&mut self, count: u32) -> Result<u32, HtLayoutError> {
        let count = count as usize;
        if count > 32 || self.consumed + count > self.bit_len {
            return Err(HtLayoutError::StreamBitReadUnavailable {
                stream: self.stream,
                requested_bits: count,
                remaining_bits: self.bit_len.saturating_sub(self.consumed),
            });
        }
        let word_index = self.consumed / 64;
        let shift = self.consumed % 64;
        let mut value = self.words[word_index] >> shift;
        if shift + count > 64 {
            value |= self.words[word_index + 1] << (64 - shift);
        }
        self.consumed += count;
        let mask = if count == 32 {
            u64::from(u32::MAX)
        } else if count == 0 {
            0
        } else {
            (1_u64 << count) - 1
        };
        Ok((value & mask) as u32)
    }
}

fn prepare_word_storage(words: &mut Vec<u64>, maximum_bits: usize) {
    words.clear();
    words.resize(maximum_bits.div_ceil(64) + 1, 0);
}

#[inline(always)]
fn append_prepared_bits(words: &mut [u64], bit_offset: usize, bits: u8, count: u32) {
    let word_index = bit_offset / 64;
    let shift = bit_offset % 64;
    words[word_index] |= u64::from(bits) << shift;
    if shift + count as usize > 64 {
        words[word_index + 1] |= u64::from(bits) >> (64 - shift);
    }
}

fn prepare_vlc_words(words: &mut Vec<u64>, mel_vlc: &[u8]) -> Result<usize, HtLayoutError> {
    if mel_vlc.len() < 2 {
        return Err(HtLayoutError::InvalidCleanupPassSegment {
            lcup: mel_vlc.len(),
            scup: mel_vlc.len(),
        });
    }
    let bytes = &mel_vlc[..mel_vlc.len() - 1];
    prepare_word_storage(words, bytes.len() * 8);
    let mut bit_offset = 0;
    let mut previous = None;
    for (reverse_index, &byte) in bytes.iter().rev().enumerate() {
        let (bits, count) = if reverse_index == 0 {
            let nibble = byte >> 4;
            previous = Some(byte | 0x0f);
            (nibble, 4 - u32::from((nibble & 0x7) == 0x7))
        } else {
            let stuffed = previous.is_some_and(|value| value > 0x8f) && (byte & 0x7f) == 0x7f;
            if stuffed && byte & 0x80 != 0 {
                return Err(HtLayoutError::StreamStuffedByteInvalid {
                    stream: HtCleanupStreamKind::Vlc,
                    byte,
                    consumed_bits: reverse_index * 8 - 4,
                });
            }
            previous = Some(byte);
            (byte, 8 - u32::from(stuffed))
        };
        let mask = ((1_u16 << count) - 1) as u8;
        append_prepared_bits(words, bit_offset, bits & mask, count);
        bit_offset += count as usize;
    }
    Ok(bit_offset)
}

fn prepare_magsgn_words(words: &mut Vec<u64>, bytes: &[u8]) -> usize {
    prepare_word_storage(words, bytes.len() * 8);
    let mut bit_offset = 0;
    let mut previous_was_ff = false;
    for &byte in bytes {
        let count = if previous_was_ff { 7 } else { 8 };
        let payload = if previous_was_ff { byte & 0x7f } else { byte };
        append_prepared_bits(words, bit_offset, payload, count);
        bit_offset += count as usize;
        previous_was_ff = byte == 0xff;
    }
    bit_offset
}

#[cfg(feature = "std")]
fn prepare_magsgn_bytes(bytes: &[u8], mut storage: PreparedMagSgnStorage) -> PreparedMagSgnBytes {
    const VIRTUAL_PADDING_BYTES: usize = 16;
    storage.bytes.clear();
    storage
        .bytes
        .resize(bytes.len() + VIRTUAL_PADDING_BYTES + 8, 0);
    storage.stuffed_bit_offsets.clear();
    let mut stuffed_bit_count = 0;
    let mut bit_offset = 0;
    let mut previous_was_ff = false;
    for (index, byte) in bytes
        .iter()
        .copied()
        .chain(core::iter::repeat_n(0xff, VIRTUAL_PADDING_BYTES))
        .enumerate()
    {
        let count = if previous_was_ff { 7 } else { 8 };
        if index < bytes.len() && count == 7 {
            stuffed_bit_count += 1;
            storage.stuffed_bit_offsets.push(bit_offset);
        }
        let payload = if count == 7 { byte & 0x7f } else { byte };
        let byte_offset = bit_offset / 8;
        let shift = bit_offset % 8;
        storage.bytes[byte_offset] |= payload << shift;
        if shift != 0 {
            storage.bytes[byte_offset + 1] |= payload >> (8 - shift);
        }
        bit_offset += count;
        previous_was_ff = byte == 0xff;
    }
    PreparedMagSgnBytes {
        storage,
        source_data_bits: bytes.len() * 8 - stuffed_bit_count,
    }
}

pub(super) fn benchmark_predestuffed_vlc_reads(
    scratch: &mut crate::HtCleanupPredestuffBenchScratch,
    mel_vlc: &[u8],
    widths: &[u8],
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let bit_len = prepare_vlc_words(&mut scratch.vlc_words, mel_vlc)?;
    let mut reader =
        PredestuffedBitReader::new(&scratch.vlc_words, bit_len, HtCleanupStreamKind::Vlc);
    benchmark_predestuffed_reads(&mut reader, widths)
}

pub(super) fn benchmark_predestuffed_magsgn_reads(
    scratch: &mut crate::HtCleanupPredestuffBenchScratch,
    bytes: &[u8],
    widths: &[u8],
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let bit_len = prepare_magsgn_words(&mut scratch.magnitude_sign_words, bytes);
    let mut reader = PredestuffedBitReader::new(
        &scratch.magnitude_sign_words,
        bit_len,
        HtCleanupStreamKind::CleanupForward,
    );
    benchmark_predestuffed_reads(&mut reader, widths)
}

fn benchmark_predestuffed_reads(
    reader: &mut PredestuffedBitReader<'_>,
    widths: &[u8],
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let mut checksum = 0_u64;
    for (index, &width) in widths.iter().enumerate() {
        let value = reader.take(u32::from(width))?;
        checksum = checksum.rotate_left(7) ^ u64::from(value) ^ index as u64;
    }
    Ok(crate::HtCleanupReaderBenchResult {
        checksum,
        read_count: widths.len(),
        consumed_bits: reader.consumed,
    })
}

pub(super) fn benchmark_magsgn_reads(
    bytes: &[u8],
    widths: &[u8],
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let mut reader = FastMagSgnReader::new(bytes);
    let mut checksum = 0_u64;
    for (index, &width) in widths.iter().enumerate() {
        let value = reader.take(u32::from(width))?;
        checksum = checksum.rotate_left(7) ^ u64::from(value) ^ index as u64;
    }
    Ok(crate::HtCleanupReaderBenchResult {
        checksum,
        read_count: widths.len(),
        consumed_bits: reader.consumed_bits(),
    })
}

pub(super) fn benchmark_mel_events(
    bytes: &[u8],
    event_count: usize,
) -> Result<crate::HtCleanupReaderBenchResult, HtLayoutError> {
    let mut reader = FastMelReader::new(bytes);
    let mut checksum = 0_u64;
    for index in 0..event_count {
        let event = reader.decode_event()?;
        checksum = checksum.rotate_left(3) ^ u64::from(event == HtMelEvent::One) ^ index as u64;
    }
    Ok(crate::HtCleanupReaderBenchResult {
        checksum,
        read_count: event_count,
        consumed_bits: reader.consumed_bits(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vlc_tail_lookup_uses_only_the_selected_codeword_bits() {
        let table = crate::ht_vlc_initial_lookup_table();
        let mut accepted_short_codewords = 0;
        let mut rejected_overlong_codewords = 0;

        for nibble in 0_u8..16 {
            let segment = [nibble << 4, 0];
            let physical_bits = 4 - u8::from((nibble & 0x7) == 0x7);
            let prefix = nibble & ((1_u8 << physical_bits) - 1);
            for context_value in 1..=HtVlcContext::MAX {
                let context = HtVlcContext::new(context_value).unwrap();
                let expected = table.lookup(context, prefix);
                let mut reader = FastVlcReader::new(&segment).unwrap();
                let decoded = reader.decode_codeword(table, context, None);

                if expected.consumed_bits() <= physical_bits {
                    assert_eq!(decoded.unwrap(), expected);
                    accepted_short_codewords += 1;
                } else {
                    assert!(matches!(
                        decoded,
                        Err(HtLayoutError::StreamBitReadUnavailable {
                            stream: HtCleanupStreamKind::Vlc,
                            ..
                        })
                    ));
                    rejected_overlong_codewords += 1;
                }
            }
        }

        assert!(accepted_short_codewords > 0);
        assert!(rejected_overlong_codewords > 0);
    }
}
