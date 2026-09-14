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
const NEIGHBOURS: u16 = 0xff;
const SIGNIFICANT: u16 = 1 << 8;
const VISITED: u16 = 1 << 9;
const REFINED: u16 = 1 << 10;

#[derive(Default)]
pub(super) struct Scratch {
    states: Vec<u16>,
    signs: Vec<u8>,
    magnitudes: Vec<u32>,
    #[cfg(test)]
    pub(super) trace: encode_trace::Trace,
}

impl Scratch {
    pub(super) fn clear(&mut self) {
        self.states.clear();
        self.signs.clear();
        self.magnitudes.clear();
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
    pub(super) fn capacities(&self) -> [usize; 3] {
        [
            self.states.capacity(),
            self.signs.capacity(),
            self.magnitudes.capacity(),
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

    fn set_significant(&mut self, index: usize) {
        debug_assert_eq!(self.states[index] & SIGNIFICANT, 0);
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
    }

    fn reset_visits(&mut self) {
        for state in self.states.iter_mut() {
            *state &= !VISITED;
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
    for base_row in (0..context.height).step_by(4) {
        let stripe_height = (context.height - base_row).min(4);
        for x in 0..context.width {
            let start = context.index(x, base_row);
            #[cfg(test)]
            encoder.position(start);
            let run_mode = stripe_height == 4
                && (0..4).all(|row| {
                    context.states[start + row * context.stride]
                        & (SIGNIFICANT | VISITED | NEIGHBOURS)
                        == 0
                });
            if run_mode {
                let run =
                    (0..4).find(|&row| context.magnitude_bit(start + row * context.stride) != 0);
                if let Some(run) = run {
                    encoder.write_bit(17, 1);
                    encoder.write_bit(18, (run >> 1) as u32);
                    encoder.write_bit(18, (run & 1) as u32);
                    let index = start + run * context.stride;
                    sign(index, context, encoder);
                    context.set_significant(index);
                    for row in (run + 1)..stripe_height {
                        cleanup_position(start + row * context.stride, context, encoder);
                    }
                } else {
                    encoder.write_bit(17, 0);
                }
            } else {
                for row in 0..stripe_height {
                    cleanup_position(start + row * context.stride, context, encoder);
                }
            }
        }
    }
}

fn cleanup_position(index: usize, context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    if context.states[index] & (SIGNIFICANT | VISITED) == 0 {
        #[cfg(test)]
        encoder.position(index);
        let bit = context.magnitude_bit(index);
        encoder.write_bit(context.zero_context(index), bit);
        if bit != 0 {
            sign(index, context, encoder);
            context.set_significant(index);
        }
    }
}

fn significance<const RAW: bool>(context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    for base_row in (0..context.height).step_by(4) {
        let stripe_height = (context.height - base_row).min(4);
        for x in 0..context.width {
            let mut index = context.index(x, base_row);
            for _ in 0..stripe_height {
                let state = context.states[index];
                if state & SIGNIFICANT == 0 && state & NEIGHBOURS != 0 {
                    #[cfg(test)]
                    encoder.position(index);
                    let bit = context.magnitude_bit(index);
                    if RAW {
                        encoder.write_raw_bit(bit);
                    } else {
                        encoder.write_bit(context.zero_context(index), bit);
                    }
                    context.states[index] |= VISITED;
                    if bit != 0 {
                        if RAW {
                            encoder.write_raw_bit(u32::from(context.signs[index]));
                        } else {
                            sign(index, context, encoder);
                        }
                        context.set_significant(index);
                    }
                }
                index += context.stride;
            }
        }
    }
}

fn refinement<const RAW: bool>(context: &mut Context<'_>, encoder: &mut ArithmeticEncoder<'_>) {
    for base_row in (0..context.height).step_by(4) {
        let stripe_height = (context.height - base_row).min(4);
        for x in 0..context.width {
            let mut index = context.index(x, base_row);
            for _ in 0..stripe_height {
                let state = context.states[index];
                if state & (SIGNIFICANT | VISITED) == SIGNIFICANT {
                    #[cfg(test)]
                    encoder.position(index);
                    let bit = context.magnitude_bit(index);
                    if RAW {
                        encoder.write_raw_bit(bit);
                    } else {
                        let label = if state & REFINED != 0 {
                            16
                        } else {
                            14 + u8::from(state & NEIGHBOURS != 0)
                        };
                        encoder.write_bit(label, bit);
                    }
                    context.states[index] |= REFINED;
                }
                index += context.stride;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    packed.set_significant(index);
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
                        }
                    }
                }
            }
        }
    }
}
