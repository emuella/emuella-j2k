//! Project-authored incremental coefficient state for classic encoding.
//!
//! The reference encoder remains independent. This backend changes coefficient
//! storage and neighbour queries, while using the existing MQ/raw writer.

use super::*;

pub(super) const SELECTED: bool = match option_env!("EMUELLA_TIER1_ENCODER") {
    None => false,
    Some(value) => match value.as_bytes() {
        b"reference" => false,
        b"packed" => true,
        _ => panic!("EMUELLA_TIER1_ENCODER must be reference or packed"),
    },
};

pub(super) fn eligible(spec: CodeBlockEncodeSpec) -> bool {
    matches!(spec.code_block_style, 0 | 1)
        && spec.dimensions.width() <= 64
        && spec.dimensions.height() <= 64
}

// Direction bits retain Neighborhood::from_mask's project-owned mapping.
const BOTTOM: u16 = 1;
const BOTTOM_RIGHT: u16 = 1 << 1;
const RIGHT: u16 = 1 << 2;
const BOTTOM_LEFT: u16 = 1 << 3;
const LEFT: u16 = 1 << 4;
const TOP_RIGHT: u16 = 1 << 5;
const TOP: u16 = 1 << 6;
const TOP_LEFT: u16 = 1 << 7;
const SIGNIFICANT: u16 = 1 << 8;
// Four rows by sixteen columns: increasing bit positions follow coding order.
// The valid mask excludes partial stripes and columns before any traversal.
#[derive(Clone, Default)]
struct Word {
    significant: u64,
    neighbours: u64,
    visited: u64,
    refined: u64,
    valid: u64,
}
const _: () = assert!(core::mem::size_of::<Word>() == 5 * core::mem::size_of::<u64>());

#[derive(Default)]
pub(super) struct Scratch {
    states: Vec<u16>,
    signs: Vec<u8>,
    magnitudes: Vec<u32>,
    words: Vec<Word>,
    #[cfg(test)]
    pub(super) trace: encode_trace::Trace,
}

impl Scratch {
    pub(super) fn clear(&mut self) {
        self.states.clear();
        self.signs.clear();
        self.magnitudes.clear();
        self.words.clear();
    }

    fn prepare(
        &mut self,
        coefficients: &[i32],
        row_stride: usize,
        spec: CodeBlockEncodeSpec,
    ) -> (Context<'_>, u32) {
        let width = usize::from(spec.dimensions.width());
        let height = usize::from(spec.dimensions.height());
        let stride = width + 2;
        let len = stride * (height + 2);
        // Exact reservation bounds each retained capacity by the largest padded
        // shape seen, without relying on amortised vector growth for this state.
        prepare_buffer(&mut self.states, len);
        prepare_buffer(&mut self.signs, len);
        prepare_buffer(&mut self.magnitudes, len);
        self.states.fill(0);
        let chunks = width.div_ceil(16);
        prepare_buffer(&mut self.words, height.div_ceil(4) * chunks);
        self.words.fill(Word::default());
        for stripe in 0..height.div_ceil(4) {
            let rows = (height - stripe * 4).min(4);
            // Repeat the valid row nibble over all sixteen columns.
            let row_mask = (u64::MAX / 15) * ((1_u64 << rows) - 1);
            for chunk in 0..chunks {
                let columns = (width - chunk * 16).min(16);
                let column_mask = if columns == 16 {
                    u64::MAX
                } else {
                    (1_u64 << (columns * 4)) - 1
                };
                self.words[stripe * chunks + chunk].valid = row_mask & column_mask;
            }
        }
        let mut maximum = 0;
        for y in 0..height {
            let source = &coefficients[y * row_stride..y * row_stride + width];
            let destination = (y + 1) * stride + 1;
            for (x, &coefficient) in source.iter().enumerate() {
                let magnitude = coefficient.unsigned_abs();
                self.magnitudes[destination + x] = magnitude;
                self.signs[destination + x] = u8::from(coefficient < 0);
                maximum = maximum.max(magnitude);
            }
        }
        (
            Context {
                states: &mut self.states,
                signs: &self.signs,
                magnitudes: &self.magnitudes,
                words: &mut self.words,
                chunks,
                width,
                height,
                stride,
                subband: spec.subband,
                bit_position: 0,
                #[cfg(test)]
                trace: &mut self.trace,
            },
            maximum,
        )
    }

    #[cfg(test)]
    pub(super) fn capacities(&self) -> [usize; 4] {
        [
            self.states.capacity(),
            self.signs.capacity(),
            self.magnitudes.capacity(),
            self.words.capacity(),
        ]
    }
}

fn prepare_buffer<T: Default + Clone>(buffer: &mut Vec<T>, len: usize) {
    if buffer.capacity() < len {
        buffer.reserve_exact(len - buffer.len());
    }
    buffer.resize(len, T::default());
}

struct Context<'a> {
    states: &'a mut [u16],
    signs: &'a [u8],
    magnitudes: &'a [u32],
    words: &'a mut [Word],
    chunks: usize,
    width: usize,
    height: usize,
    stride: usize,
    subband: Subband,
    bit_position: u8,
    #[cfg(test)]
    trace: &'a mut encode_trace::Trace,
}

impl Context<'_> {
    fn index(&self, x: usize, y: usize) -> usize {
        (y + 1) * self.stride + x + 1
    }

    fn magnitude_bit(&self, index: usize) -> u32 {
        (self.magnitudes[index] >> self.bit_position) & 1
    }

    fn zero_context(&self, index: usize) -> u8 {
        zero_coding_context(
            self.subband,
            Neighborhood::from_mask(self.states[index] as u8),
        )
    }

    fn sign_context(&self, index: usize) -> (u8, u8) {
        let state = self.states[index];
        sign_context(
            sign_contribution(
                state & LEFT != 0,
                self.signs[index - 1],
                state & RIGHT != 0,
                self.signs[index + 1],
            ),
            sign_contribution(
                state & TOP != 0,
                self.signs[index - self.stride],
                state & BOTTOM != 0,
                self.signs[index + self.stride],
            ),
        )
    }

    fn word_position(&self, x: usize, y: usize) -> (usize, u64) {
        (
            (y / 4) * self.chunks + x / 16,
            1_u64 << ((x % 16) * 4 + y % 4),
        )
    }

    fn word_base(&self, word: usize) -> usize {
        self.index((word % self.chunks) * 16, (word / self.chunks) * 4)
    }

    fn mark_neighbour(&mut self, x: usize, y: usize) {
        let (word, mask) = self.word_position(x, y);
        self.words[word].neighbours |= mask;
    }

    fn set_significant(&mut self, index: usize, word: usize, bit: usize) {
        debug_assert_eq!(self.states[index] & SIGNIFICANT, 0);
        debug_assert_ne!(self.words[word].valid & (1_u64 << bit), 0);
        self.states[index] |= SIGNIFICANT;
        let above = index - self.stride;
        let below = index + self.stride;
        self.states[above - 1] |= BOTTOM_RIGHT;
        self.states[above] |= BOTTOM;
        self.states[above + 1] |= BOTTOM_LEFT;
        self.states[index - 1] |= RIGHT;
        self.states[index + 1] |= LEFT;
        self.states[below - 1] |= TOP_RIGHT;
        self.states[below] |= TOP;
        self.states[below + 1] |= TOP_LEFT;

        self.words[word].significant |= 1_u64 << bit;
        let x = (word % self.chunks) * 16 + bit / 4;
        let y = (word / self.chunks) * 4 + bit % 4;
        if x != 0 {
            self.mark_neighbour(x - 1, y);
            if y != 0 {
                self.mark_neighbour(x - 1, y - 1);
            }
            if y + 1 < self.height {
                self.mark_neighbour(x - 1, y + 1);
            }
        }
        if x + 1 < self.width {
            self.mark_neighbour(x + 1, y);
            if y != 0 {
                self.mark_neighbour(x + 1, y - 1);
            }
            if y + 1 < self.height {
                self.mark_neighbour(x + 1, y + 1);
            }
        }
        if y != 0 {
            self.mark_neighbour(x, y - 1);
        }
        if y + 1 < self.height {
            self.mark_neighbour(x, y + 1);
        }
    }

    fn reset_visits(&mut self) {
        for word in self.words.iter_mut() {
            word.visited = 0;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn encode(
    coefficients: &[i32],
    row_stride: usize,
    known_maximum: Option<u32>,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    mut segment_lengths: Option<&mut Vec<usize>>,
    scratch: &mut Scratch,
) -> Result<CodeBlockEncode> {
    debug_assert!(eligible(spec));
    let (mut context, observed_maximum) = scratch.prepare(coefficients, row_stride, spec);
    let maximum = known_maximum.unwrap_or(observed_maximum);
    if maximum == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }
    let planes = (32 - maximum.leading_zeros()) as u8;
    if planes > spec.available_bitplanes {
        return Err(Tier1Error::MalformedBitstream {
            reason: "coefficient magnitude exceeds available bit-planes",
        });
    }
    let pass_count = 1 + 3 * u16::from(planes - 1);
    let style = CodeBlockStyle::from_bits(spec.code_block_style);
    let start = output.len();
    let mut encoder = ArithmeticEncoder::new(output);
    #[cfg(test)]
    encoder.enable_trace(context.trace.enabled);
    for pass in 0..pass_count {
        context.bit_position = planes - 1 - pass.div_ceil(3) as u8;
        #[cfg(test)]
        encoder.pass(pass, context.bit_position);
        let raw = is_raw_coding_pass(style, pass);
        match coding_pass_for_index(pass) {
            CodingPass::Cleanup => {
                cleanup(&mut context, &mut encoder);
                context.reset_visits();
            }
            CodingPass::SignificancePropagation => {
                if raw {
                    significance::<true>(&mut context, &mut encoder);
                } else {
                    significance::<false>(&mut context, &mut encoder);
                }
            }
            CodingPass::MagnitudeRefinement => {
                if raw {
                    refinement::<true>(&mut context, &mut encoder);
                } else {
                    refinement::<false>(&mut context, &mut encoder);
                }
            }
        }
        if coding_segment_ends_after(style, pass, pass_count) {
            if raw {
                encoder.finish_raw(false);
            } else {
                encoder.finish();
            }
            if let Some(lengths) = segment_lengths.as_deref_mut() {
                lengths.push(encoder.current_segment_len());
            }
            if pass + 1 < pass_count {
                if is_raw_coding_pass(style, pass + 1) {
                    encoder.restart_raw_segment();
                } else {
                    encoder.restart_segment();
                }
            }
        }
    }
    #[cfg(test)]
    encoder.take_trace(context.trace);
    Ok(CodeBlockEncode {
        pass_count,
        included: true,
        missing_bitplanes: spec.available_bitplanes - planes,
        byte_len: encoder.len() - start,
    })
}

fn sign(index: usize, context: &Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    #[cfg(test)]
    encoder.position(index);
    let (label, prediction) = context.sign_context(index);
    encoder.write_bit(label, u32::from(context.signs[index] ^ prediction));
}

fn cleanup(context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    for word in 0..context.words.len() {
        let base = context.word_base(word);
        for column in 0..16 {
            let column_mask = 0x0f_u64 << (column * 4);
            let valid = context.words[word].valid & column_mask;
            if valid == 0 {
                break;
            }
            let unavailable = context.words[word].significant | context.words[word].visited;
            let available = valid & !unavailable;
            if available == 0 {
                continue;
            }
            let run_mode = valid == column_mask
                && (unavailable | context.words[word].neighbours) & column_mask == 0;
            let start = base + column;
            #[cfg(test)]
            encoder.position(start);
            if run_mode {
                let run =
                    (0..4).find(|&row| context.magnitude_bit(start + row * context.stride) != 0);
                if let Some(run) = run {
                    encoder.write_bit(17, 1);
                    encoder.write_bit(18, (run >> 1) as u32);
                    encoder.write_bit(18, (run & 1) as u32);
                    let index = start + run * context.stride;
                    sign(index, context, encoder);
                    context.set_significant(index, word, column * 4 + run);
                    for row in (run + 1)..4 {
                        cleanup_available(
                            start + row * context.stride,
                            word,
                            column * 4 + row,
                            context,
                            encoder,
                        );
                    }
                } else {
                    encoder.write_bit(17, 0);
                }
            } else {
                let mut available = available;
                while available != 0 {
                    let bit = available.trailing_zeros() as usize;
                    available &= available - 1;
                    cleanup_available(
                        start + (bit % 4) * context.stride,
                        word,
                        bit,
                        context,
                        encoder,
                    );
                }
            }
        }
    }
}

fn cleanup_available(
    index: usize,
    word: usize,
    bit: usize,
    context: &mut Context<'_>,
    encoder: &mut ArithmeticEncoder<'_>,
) {
    #[cfg(test)]
    encoder.position(index);
    let decision = context.magnitude_bit(index);
    encoder.write_bit(context.zero_context(index), decision);
    if decision != 0 {
        sign(index, context, encoder);
        context.set_significant(index, word, bit);
    }
}

fn significance<const RAW: bool>(context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    for word in 0..context.words.len() {
        let base = context.word_base(word);
        let mut remaining = u64::MAX;
        loop {
            // New significance can enable later positions in this same word.
            // Re-read live state, but never return to an earlier scan position.
            let candidates = context.words[word].neighbours
                & !context.words[word].significant
                & context.words[word].valid
                & remaining;
            if candidates == 0 {
                break;
            }
            let bit = candidates.trailing_zeros() as usize;
            remaining = if bit == 63 { 0 } else { u64::MAX << (bit + 1) };
            let index = base + (bit / 4) + (bit % 4) * context.stride;
            #[cfg(test)]
            encoder.position(index);
            let decision = context.magnitude_bit(index);
            if RAW {
                encoder.write_raw_bit(decision);
            } else {
                encoder.write_bit(context.zero_context(index), decision);
            }
            context.words[word].visited |= 1_u64 << bit;
            if decision != 0 {
                if RAW {
                    encoder.write_raw_bit(u32::from(context.signs[index]));
                } else {
                    sign(index, context, encoder);
                }
                context.set_significant(index, word, bit);
            }
        }
    }
}

fn refinement<const RAW: bool>(context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    for word in 0..context.words.len() {
        let base = context.word_base(word);
        let mut candidates = context.words[word].significant & !context.words[word].visited;
        while candidates != 0 {
            let bit = candidates.trailing_zeros() as usize;
            candidates &= candidates - 1;
            let mask = 1_u64 << bit;
            let index = base + (bit / 4) + (bit % 4) * context.stride;
            #[cfg(test)]
            encoder.position(index);
            let decision = context.magnitude_bit(index);
            if RAW {
                encoder.write_raw_bit(decision);
            } else {
                let label = if context.words[word].refined & mask != 0 {
                    16
                } else {
                    14 + u8::from(context.words[word].neighbours & mask != 0)
                };
                encoder.write_bit(label, decision);
            }
            context.words[word].refined |= mask;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_encoder_words_exclude_padding_for_every_eligible_geometry() {
        let mut scratch = Scratch::default();
        for width in 1..=64 {
            for height in 1..=64 {
                let spec = CodeBlockEncodeSpec {
                    dimensions: CodeBlockDimensions::new(width, height).unwrap(),
                    subband: Subband::LowLow,
                    available_bitplanes: 1,
                    code_block_style: 0,
                };
                let source = alloc::vec![0; usize::from(width) * usize::from(height)];
                let (context, _) = scratch.prepare(&source, usize::from(width), spec);
                assert!(context.words.len() <= 64);
                assert_eq!(
                    context
                        .words
                        .iter()
                        .map(|word| word.valid.count_ones())
                        .sum::<u32>(),
                    u32::from(width) * u32::from(height)
                );
                for (word, state) in context.words.iter().enumerate() {
                    for bit in 0..64 {
                        let x = (word % context.chunks) * 16 + bit / 4;
                        let y = (word / context.chunks) * 4 + bit % 4;
                        assert_eq!(
                            state.valid & (1_u64 << bit) != 0,
                            x < context.width && y < context.height
                        );
                        if state.valid & (1_u64 << bit) != 0 {
                            assert_eq!(context.word_position(x, y), (word, 1_u64 << bit));
                            assert_eq!(
                                context.word_base(word) + bit / 4 + (bit % 4) * context.stride,
                                context.index(x, y)
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn packed_encoder_live_candidates_preserve_the_consumed_prefix_and_word_crossings() {
        let spec = CodeBlockEncodeSpec {
            dimensions: CodeBlockDimensions::new(33, 5).unwrap(),
            subband: Subband::HighLow,
            available_bitplanes: 1,
            code_block_style: 0,
        };
        let source = alloc::vec![1; 33 * 5];
        for raw in [false, true] {
            let mut packed_scratch = Scratch::default();
            let (mut packed, _) = packed_scratch.prepare(&source, 33, spec);
            let mut reference_scratch = CodeBlockEncodeScratch::new();
            let mut reference = reference_scratch.prepare(33, 5, spec.subband, &source);
            let seed = packed.index(2, 1);
            let (word, mask) = packed.word_position(2, 1);
            packed.set_significant(seed, word, mask.trailing_zeros() as usize);
            reference.set_significant_at::<false>(seed);
            let mut actual = Vec::new();
            let mut expected = Vec::new();
            let mut actual_encoder = ArithmeticEncoder::new(&mut actual);
            let mut reference_encoder = ArithmeticEncoder::new(&mut expected);
            actual_encoder.enable_trace(true);
            reference_encoder.enable_trace(true);
            if raw {
                significance::<true>(&mut packed, &mut actual_encoder);
                significance_propagation_pass_encode_raw::<false>(
                    &mut reference,
                    &mut reference_encoder,
                );
                actual_encoder.finish_raw(false);
                reference_encoder.finish_raw(false);
            } else {
                significance::<false>(&mut packed, &mut actual_encoder);
                significance_propagation_pass_encode::<false>(
                    &mut reference,
                    &mut reference_encoder,
                );
                actual_encoder.finish();
                reference_encoder.finish();
            }
            actual_encoder.take_trace(packed.trace);
            reference_encoder.take_trace(reference.trace);
            drop(actual_encoder);
            drop(reference_encoder);
            assert_eq!(actual, expected);
            assert_eq!(packed.trace.events, reference.trace.events);
            // (1,0) becomes significant first and enables (0,0), which is now
            // behind the cursor. Later positions in the current word and in
            // subsequent words must still be discovered during this pass.
            let (behind_word, behind_mask) = packed.word_position(0, 0);
            assert_ne!(packed.words[behind_word].neighbours & behind_mask, 0);
            assert_eq!(packed.words[behind_word].visited & behind_mask, 0);
            for (x, y) in [(15, 3), (16, 0), (31, 3), (32, 0), (1, 4), (32, 4)] {
                let (word, mask) = packed.word_position(x, y);
                assert_ne!(packed.words[word].visited & mask, 0, "{x},{y}");
            }
            for y in 0..5 {
                for x in 0..33 {
                    let index = packed.index(x, y);
                    let (word, mask) = packed.word_position(x, y);
                    assert_eq!(
                        packed.words[word].visited & mask != 0,
                        reference.is_zero_coded_at(index)
                    );
                    assert_eq!(
                        packed.words[word].significant & mask != 0,
                        reference.is_significant_at(index)
                    );
                }
            }
            packed.reset_visits();
            assert!(packed.words.iter().all(|word| word.visited == 0));
        }
    }

    #[test]
    fn packed_encoder_incremental_neighbours_match_independent_gather_at_every_edge() {
        for (width, height) in [(1, 1), (1, 5), (5, 1), (3, 3), (17, 5)] {
            let spec = CodeBlockEncodeSpec {
                dimensions: CodeBlockDimensions::new(width, height).unwrap(),
                subband: Subband::HighHigh,
                available_bitplanes: 1,
                code_block_style: 0,
            };
            let (width, height) = (usize::from(width), usize::from(height));
            let source = (0..width * height)
                .map(|index| if index % 3 == 1 { -1 } else { 1 })
                .collect::<Vec<_>>();
            for reverse in [false, true] {
                let mut scratch = Scratch::default();
                let (mut packed, _) = scratch.prepare(&source, width, spec);
                let mut reference_scratch = CodeBlockEncodeScratch::new();
                let mut reference = reference_scratch.prepare(width, height, spec.subband, &source);
                for step in 0..source.len() {
                    let sample = if reverse {
                        source.len() - 1 - step
                    } else {
                        step
                    };
                    let index = packed.index(sample % width, sample / width);
                    let (word, mask) = packed.word_position(sample % width, sample / width);
                    packed.set_significant(index, word, mask.trailing_zeros() as usize);
                    reference.set_significant_at::<false>(index);
                    for y in 0..height {
                        for x in 0..width {
                            let index = packed.index(x, y);
                            assert_eq!(
                                Neighborhood::from_mask(packed.states[index] as u8),
                                neighborhood_at::<false>(
                                    reference.coefficient_states,
                                    index,
                                    reference.padded_width
                                )
                            );
                            assert_eq!(
                                packed.sign_context(index),
                                context_label_sign_coding_encode_at::<false>(index, &reference)
                            );
                            let (word, mask) = packed.word_position(x, y);
                            assert_eq!(
                                packed.words[word].neighbours & mask != 0,
                                neighborhood_at::<false>(
                                    reference.coefficient_states,
                                    index,
                                    reference.padded_width
                                )
                                .any()
                            );
                            assert_eq!(
                                packed.words[word].significant & mask != 0,
                                reference.is_significant_at(index)
                            );
                        }
                    }
                }
            }
        }
    }
}
