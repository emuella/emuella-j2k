//! JPEG 2000 MQ arithmetic coding primitives.
//!
//! This module follows the register procedures in ISO/IEC 15444-1:2024,
//! Annex C, and the Tier-1 initialization and termination requirements in
//! Annex D. It deliberately keeps probability state, the current MPS, and
//! the pending output byte as separate values so the implementation mirrors
//! the model rather than an upstream codec's packed representation.

use super::{Result, Tier1Error};
use alloc::vec::Vec;

pub(super) const CONTEXT_COUNT: usize = 19;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct Context {
    state: u8,
    mps: u8,
}

impl Context {
    const fn with_state(state: u8) -> Self {
        Self { state, mps: 0 }
    }
}

pub(super) fn initial_contexts() -> [Context; CONTEXT_COUNT] {
    let mut contexts = [Context::default(); CONTEXT_COUNT];
    // ISO/IEC 15444-1:2024, Table D.7.
    contexts[0] = Context::with_state(4);
    contexts[17] = Context::with_state(3);
    contexts[18] = Context::with_state(46);
    contexts
}

pub(super) fn reset_contexts(contexts: &mut [Context; CONTEXT_COUNT]) {
    *contexts = initial_contexts();
}

#[derive(Debug, Clone, Copy)]
struct ProbabilityEstimate {
    qe: u32,
    next_mps: u8,
    next_lps: u8,
    switch_mps: bool,
}

macro_rules! probability_estimates {
    ($($qe:expr, $next_mps:expr, $next_lps:expr, $switch_mps:expr),+ $(,)?) => {
        [$(ProbabilityEstimate {
            qe: $qe,
            next_mps: $next_mps,
            next_lps: $next_lps,
            switch_mps: $switch_mps,
        }),+]
    };
}

// ISO/IEC 15444-1:2024, Table C.2. The table values are normative data.
#[rustfmt::skip]
const PROBABILITY_ESTIMATES: [ProbabilityEstimate; 47] = probability_estimates!(
    0x5601, 1, 1, true,
    0x3401, 2, 6, false,
    0x1801, 3, 9, false,
    0x0ac1, 4, 12, false,
    0x0521, 5, 29, false,
    0x0221, 38, 33, false,
    0x5601, 7, 6, true,
    0x5401, 8, 14, false,
    0x4801, 9, 14, false,
    0x3801, 10, 14, false,
    0x3001, 11, 17, false,
    0x2401, 12, 18, false,
    0x1c01, 13, 20, false,
    0x1601, 29, 21, false,
    0x5601, 15, 14, true,
    0x5401, 16, 14, false,
    0x5101, 17, 15, false,
    0x4801, 18, 16, false,
    0x3801, 19, 17, false,
    0x3401, 20, 18, false,
    0x3001, 21, 19, false,
    0x2801, 22, 19, false,
    0x2401, 23, 20, false,
    0x2201, 24, 21, false,
    0x1c01, 25, 22, false,
    0x1801, 26, 23, false,
    0x1601, 27, 24, false,
    0x1401, 28, 25, false,
    0x1201, 29, 26, false,
    0x1101, 30, 27, false,
    0x0ac1, 31, 28, false,
    0x09c1, 32, 29, false,
    0x08a1, 33, 30, false,
    0x0521, 34, 31, false,
    0x0441, 35, 32, false,
    0x02a1, 36, 33, false,
    0x0221, 37, 34, false,
    0x0141, 38, 35, false,
    0x0111, 39, 36, false,
    0x0085, 40, 37, false,
    0x0049, 41, 38, false,
    0x0025, 42, 39, false,
    0x0015, 43, 40, false,
    0x0009, 44, 41, false,
    0x0005, 45, 42, false,
    0x0001, 45, 43, false,
    0x5601, 46, 46, false,
);

pub(super) struct Decoder<'a> {
    bytes: &'a [u8],
    interval: u32,
    code: u32,
    cursor: usize,
    bits_available: u32,
    synthetic_marker_reads: u8,
}

impl<'a> Decoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        let first = u32::from(bytes.first().copied().unwrap_or(0xff));
        let mut decoder = Self {
            bytes,
            interval: 0x8000,
            code: first << 16,
            cursor: 0,
            bits_available: 0,
            synthetic_marker_reads: 0,
        };
        decoder.byte_in();
        decoder.code <<= 7;
        decoder.bits_available -= 7;
        decoder
    }

    pub(super) fn read_bit(&mut self, context: &mut Context) -> u32 {
        let estimate = PROBABILITY_ESTIMATES[usize::from(context.state)];
        self.interval -= estimate.qe;

        let decision = if (self.code >> 16) < estimate.qe {
            self.exchange_on_lps_path(context, estimate, self.interval)
        } else {
            self.code -= estimate.qe << 16;
            if self.interval & 0x8000 != 0 {
                return u32::from(context.mps);
            }
            self.exchange_on_mps_path(context, estimate)
        };

        self.renormalize();
        decision
    }

    pub(super) fn read_bit_value(&mut self, mut context: Context) -> (u32, Context) {
        let decision = self.read_bit(&mut context);
        (decision, context)
    }

    pub(super) fn read_packed_bit(&mut self, context: &mut Context) -> u32 {
        self.read_bit(context)
    }

    pub(super) fn read_packed_bit_value(&mut self, context: Context) -> (u32, Context) {
        self.read_bit_value(context)
    }

    pub(super) fn validate_predictable_termination(&self) -> Result<()> {
        if self.bytes.len().saturating_sub(self.cursor) > 2 {
            return Err(Tier1Error::MalformedBitstream {
                reason: "predictable MQ termination left more than two unread code-block bytes",
            });
        }
        if self.synthetic_marker_reads > 2 {
            return Err(Tier1Error::MalformedBitstream {
                reason: "predictable MQ termination consumed too many synthesized marker bytes",
            });
        }
        Ok(())
    }

    fn exchange_on_mps_path(
        &mut self,
        context: &mut Context,
        estimate: ProbabilityEstimate,
    ) -> u32 {
        if self.interval < estimate.qe {
            let decision = u32::from(context.mps ^ 1);
            context.state = estimate.next_lps;
            if estimate.switch_mps {
                context.mps ^= 1;
            }
            decision
        } else {
            context.state = estimate.next_mps;
            u32::from(context.mps)
        }
    }

    fn exchange_on_lps_path(
        &mut self,
        context: &mut Context,
        estimate: ProbabilityEstimate,
        mps_interval: u32,
    ) -> u32 {
        self.interval = estimate.qe;
        if mps_interval < estimate.qe {
            context.state = estimate.next_mps;
            u32::from(context.mps)
        } else {
            let decision = u32::from(context.mps ^ 1);
            context.state = estimate.next_lps;
            if estimate.switch_mps {
                context.mps ^= 1;
            }
            decision
        }
    }

    fn renormalize(&mut self) {
        while self.interval & 0x8000 == 0 {
            if self.bits_available == 0 {
                self.byte_in();
            }
            self.interval <<= 1;
            self.code <<= 1;
            self.bits_available -= 1;
        }
    }

    fn byte_in(&mut self) {
        let current = self.byte_at(self.cursor);
        let next = self.byte_at(self.cursor.saturating_add(1));
        if current == 0xff {
            if next > 0x8f {
                self.code = self.code.wrapping_add(0xff00);
                self.bits_available = 8;
                if self.cursor >= self.bytes.len().saturating_sub(1) {
                    self.synthetic_marker_reads = self.synthetic_marker_reads.saturating_add(1);
                }
            } else {
                self.cursor = self.cursor.saturating_add(1);
                self.code = self.code.wrapping_add(u32::from(next) << 9);
                self.bits_available = 7;
            }
        } else {
            self.cursor = self.cursor.saturating_add(1);
            self.code = self.code.wrapping_add(u32::from(next) << 8);
            self.bits_available = 8;
        }
    }

    fn byte_at(&self, index: usize) -> u8 {
        self.bytes.get(index).copied().unwrap_or(0xff)
    }
}

pub(super) struct RawDecoder<'a> {
    bytes: &'a [u8],
    cursor: usize,
    current: u8,
    bits_available: u8,
}

impl<'a> RawDecoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0,
            current: 0,
            bits_available: 0,
        }
    }

    #[inline(always)]
    pub(super) fn read_bit(&mut self) -> u32 {
        if self.bits_available == 0 {
            let next = self.bytes.get(self.cursor).copied().unwrap_or(0xff);
            if self.current == 0xff && next <= 0x8f {
                self.current = next;
                self.cursor += 1;
                self.bits_available = 7;
            } else if self.current == 0xff {
                self.current = 0xff;
                self.bits_available = 8;
            } else {
                self.current = next;
                self.cursor += usize::from(self.cursor < self.bytes.len());
                self.bits_available = 8;
            }
        }
        self.bits_available -= 1;
        u32::from((self.current >> self.bits_available) & 1)
    }
}

pub(super) struct Encoder<'a> {
    bytes: &'a mut Vec<u8>,
    segment_start: usize,
    contexts: [Context; CONTEXT_COUNT],
    interval: u32,
    code: u32,
    countdown: u32,
    pending_byte: Option<u8>,
    raw_byte: u8,
    raw_capacity: u8,
    raw_free: u8,
}

impl<'a> Encoder<'a> {
    pub(super) fn new(bytes: &'a mut Vec<u8>) -> Self {
        let segment_start = bytes.len();
        Self {
            bytes,
            segment_start,
            contexts: initial_contexts(),
            interval: 0x8000,
            code: 0,
            countdown: 12,
            pending_byte: None,
            raw_byte: 0,
            raw_capacity: 8,
            raw_free: 8,
        }
    }

    pub(super) fn len(&self) -> usize {
        self.bytes.len()
    }

    pub(super) fn current_segment_len(&self) -> usize {
        self.bytes.len() - self.segment_start
    }

    pub(super) fn restart_segment(&mut self) {
        debug_assert!(self.pending_byte.is_none());
        self.segment_start = self.bytes.len();
        self.interval = 0x8000;
        self.code = 0;
        self.countdown = 12;
    }

    pub(super) fn restart_raw_segment(&mut self) {
        self.segment_start = self.bytes.len();
        self.raw_byte = 0;
        self.raw_capacity = 8;
        self.raw_free = 8;
    }

    pub(super) fn write_bit(&mut self, context_label: u8, decision: u32) {
        let context_index = usize::from(context_label);
        let context = self.contexts[context_index];
        let estimate = PROBABILITY_ESTIMATES[usize::from(context.state)];
        self.interval -= estimate.qe;

        if decision == u32::from(context.mps) {
            if self.interval & 0x8000 != 0 {
                self.code = self.code.wrapping_add(estimate.qe);
                return;
            }
            if self.interval < estimate.qe {
                self.interval = estimate.qe;
            } else {
                self.code = self.code.wrapping_add(estimate.qe);
            }
            self.contexts[context_index].state = estimate.next_mps;
        } else {
            if self.interval < estimate.qe {
                self.code = self.code.wrapping_add(estimate.qe);
            } else {
                self.interval = estimate.qe;
            }
            self.contexts[context_index].state = estimate.next_lps;
            if estimate.switch_mps {
                self.contexts[context_index].mps ^= 1;
            }
        }
        self.renormalize();
    }

    #[inline(always)]
    pub(super) fn write_raw_bit(&mut self, bit: u32) {
        self.raw_free -= 1;
        self.raw_byte |= (bit as u8) << self.raw_free;
        if self.raw_free == 0 {
            self.emit_raw_byte();
        }
    }

    pub(super) fn finish_raw(&mut self, predictable: bool) {
        let has_partial_bits = self.raw_free < self.raw_capacity;
        let follows_ff = self
            .bytes
            .get(self.segment_start..)
            .is_some_and(|segment| segment.last().copied() == Some(0xff));
        if !has_partial_bits && !follows_ff {
            return;
        }

        let mut fill_bit = 0_u8;
        while self.raw_free > 0 {
            self.raw_free -= 1;
            self.raw_byte |= fill_bit << self.raw_free;
            if predictable {
                fill_bit ^= 1;
            }
        }
        self.emit_raw_byte();
        debug_assert_ne!(self.bytes.last().copied(), Some(0xff));
    }

    pub(super) fn reset_contexts(&mut self) {
        reset_contexts(&mut self.contexts);
    }

    pub(super) fn finish(&mut self) {
        self.set_final_bits();
        self.code <<= self.countdown;
        self.byte_out();
        self.code <<= self.countdown;
        self.byte_out();
        if self.pending_byte != Some(0xff) {
            self.commit_pending();
        } else {
            self.pending_byte = None;
        }
    }

    pub(super) fn finish_predictable(&mut self) {
        // ISO/IEC 15444-1:2024, D.4.2: k = (11 - CT) + 1.
        let mut remaining = 12_i32 - self.countdown as i32;
        while remaining > 0 {
            self.code <<= self.countdown;
            self.countdown = 0;
            self.byte_out();
            remaining -= self.countdown as i32;
        }
        if self.pending_byte != Some(0xff) {
            self.commit_pending();
        } else {
            self.pending_byte = None;
        }
    }

    fn renormalize(&mut self) {
        while self.interval & 0x8000 == 0 {
            self.interval <<= 1;
            self.code <<= 1;
            self.countdown -= 1;
            if self.countdown == 0 {
                self.byte_out();
            }
        }
    }

    fn byte_out(&mut self) {
        let previous = self.pending_byte.take();
        if previous == Some(0xff) {
            self.commit_byte(0xff);
            self.pending_byte = Some((self.code >> 20) as u8);
            self.code &= 0x000f_ffff;
            self.countdown = 7;
            return;
        }

        let carry = self.code >= 0x0800_0000;
        if let Some(mut previous) = previous {
            if carry {
                previous = previous.wrapping_add(1);
                if previous == 0xff {
                    self.code &= 0x07ff_ffff;
                    self.commit_byte(previous);
                    self.pending_byte = Some((self.code >> 20) as u8);
                    self.code &= 0x000f_ffff;
                    self.countdown = 7;
                    return;
                }
            }
            self.commit_byte(previous);
        }

        self.pending_byte = Some(((self.code >> 19) & 0xff) as u8);
        self.code &= 0x0007_ffff;
        self.countdown = 8;
    }

    fn set_final_bits(&mut self) {
        let upper_bound = self.code.wrapping_add(self.interval);
        self.code |= 0x0000_ffff;
        if self.code >= upper_bound {
            self.code = self.code.wrapping_sub(0x8000);
        }
    }

    fn emit_raw_byte(&mut self) {
        let byte = self.raw_byte;
        self.bytes.push(byte);
        self.raw_capacity = if byte == 0xff { 7 } else { 8 };
        self.raw_free = self.raw_capacity;
        self.raw_byte = 0;
    }

    fn commit_pending(&mut self) {
        if let Some(byte) = self.pending_byte.take() {
            self.commit_byte(byte);
        }
    }

    fn commit_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier1_contexts_match_table_d7() {
        let contexts = initial_contexts();
        assert_eq!(contexts[0], Context::with_state(4));
        assert_eq!(contexts[17], Context::with_state(3));
        assert_eq!(contexts[18], Context::with_state(46));
        assert!(
            contexts[1..17]
                .iter()
                .all(|context| *context == Context::default())
        );
    }

    #[test]
    fn mq_round_trip_crosses_probability_states_and_stuffed_bytes() {
        let decisions = (0..2048)
            .map(|index| u32::from(((index * 73 + index / 7 + 3) & 1) != 0))
            .collect::<Vec<_>>();
        let mut encoded = Vec::new();
        let mut encoder = Encoder::new(&mut encoded);
        for (index, decision) in decisions.iter().copied().enumerate() {
            encoder.write_bit((index % CONTEXT_COUNT) as u8, decision);
        }
        encoder.finish();

        let mut decoder = Decoder::new(&encoded);
        let mut contexts = initial_contexts();
        let decoded = (0..decisions.len())
            .map(|index| decoder.read_bit(&mut contexts[index % CONTEXT_COUNT]))
            .collect::<Vec<_>>();
        assert_eq!(decoded, decisions);
    }

    #[test]
    fn raw_writer_never_terminates_with_ff() {
        let mut encoded = Vec::new();
        let mut encoder = Encoder::new(&mut encoded);
        for _ in 0..8 {
            encoder.write_raw_bit(1);
        }
        encoder.finish_raw(false);
        assert_eq!(encoded[0], 0xff);
        assert_ne!(encoded.last().copied(), Some(0xff));
    }
}
