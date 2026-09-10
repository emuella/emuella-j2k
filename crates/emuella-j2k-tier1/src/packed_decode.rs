use super::{
    ArithmeticDecoder, ArithmeticDecoderContext, CONTEXT_COUNT, CodeBlockDecodeSpec,
    CodeBlockSegment, CodingPass, CoefficientReconstruction, Neighborhood, RawDecoder, Result,
    Tier1Error, coding_pass_for_index, decode_segmentation_symbol, is_raw_coding_pass,
    reset_arithmetic_contexts, sign_context, sign_contribution, validate_bitplane_pass_count,
    zero_coding_context,
};
use alloc::vec::Vec;
use core::mem::size_of;

#[derive(Debug, Clone, Copy, Default)]
struct CardinalSigns {
    top: Option<u8>,
    left: Option<u8>,
    right: Option<u8>,
    bottom: Option<u8>,
}

impl CardinalSigns {
    fn context(self) -> (u8, u8) {
        let contribution = |first: Option<u8>, second: Option<u8>| {
            sign_contribution(
                first.is_some(),
                first.unwrap_or(0),
                second.is_some(),
                second.unwrap_or(0),
            )
        };
        sign_context(
            contribution(self.left, self.right),
            contribution(self.top, self.bottom),
        )
    }
}

/// Packed decode keeps sign separately from magnitude so refinement does not
/// need to inspect the coefficient sign for every MQ-coded bit.
#[derive(Copy, Clone, Debug, Default)]
struct PackedCoefficient(u32);

impl PackedCoefficient {
    const SIGN: u32 = 1 << 31;

    #[inline(always)]
    fn set_new_significant(&mut self, sign: u8, position: u8) {
        let one = 1_u32 << (u32::from(position) + 1);
        let magnitude = one | (one >> 1);
        self.0 = magnitude | (u32::from(sign & 1) << 31);
    }

    #[inline(always)]
    fn refine(&mut self, bit: u32, position: u8) {
        let direction = (bit as i32) * 2 - 1;
        let delta = direction << u32::from(position);
        self.0 = self.0.wrapping_add_signed(delta);
    }

    #[inline(always)]
    fn signed_magnitude(self, divisor_shift: u32) -> i32 {
        let magnitude = ((self.0 & !Self::SIGN) >> divisor_shift) as i32;
        if self.0 & Self::SIGN == 0 {
            magnitude
        } else {
            -magnitude
        }
    }

    fn get(self) -> i32 {
        self.signed_magnitude(1)
    }

    fn doubled_half_step(self) -> i32 {
        self.signed_magnitude(0)
    }
}

#[derive(Default)]
pub(super) struct PackedDecodeScratch {
    stripe_significant: Vec<u64>,
    stripe_neighbor: Vec<u64>,
    stripe_visited: Vec<u64>,
    stripe_refined: Vec<u64>,
    // Used only by the sparse backend. One sign bit per packed coefficient.
    stripe_negative: Vec<u64>,
    // Used only by the sparse backend. Each plane marks one JPEG 2000
    // zero-coding neighbor direction for every packed coefficient.
    stripe_directional_neighbors: [Vec<u64>; 8],
    // Used only by the dense backend. Named directions mirror Figure D.2.
    dense_neighborhoods: Vec<Neighborhood>,
    dense_cardinal_signs: Vec<CardinalSigns>,
    // Matches each 4x16 bitboard word so refinement candidates index directly.
    coefficients: Vec<PackedCoefficient>,
}

impl PackedDecodeScratch {
    pub(super) fn retained_heap_bytes(&self) -> u64 {
        let capacity_bytes = |capacity: usize, element_bytes: usize| {
            u64::try_from(capacity)
                .unwrap_or(u64::MAX)
                .saturating_mul(u64::try_from(element_bytes).unwrap_or(u64::MAX))
        };
        capacity_bytes(self.stripe_significant.capacity(), size_of::<u64>())
            .saturating_add(capacity_bytes(
                self.stripe_neighbor.capacity(),
                size_of::<u64>(),
            ))
            .saturating_add(capacity_bytes(
                self.stripe_visited.capacity(),
                size_of::<u64>(),
            ))
            .saturating_add(capacity_bytes(
                self.stripe_refined.capacity(),
                size_of::<u64>(),
            ))
            .saturating_add(capacity_bytes(
                self.stripe_negative.capacity(),
                size_of::<u64>(),
            ))
            .saturating_add(self.stripe_directional_neighbors.iter().fold(
                0_u64,
                |total, direction| {
                    total.saturating_add(capacity_bytes(direction.capacity(), size_of::<u64>()))
                },
            ))
            .saturating_add(capacity_bytes(
                self.dense_neighborhoods.capacity(),
                size_of::<Neighborhood>(),
            ))
            .saturating_add(capacity_bytes(
                self.dense_cardinal_signs.capacity(),
                size_of::<CardinalSigns>(),
            ))
            .saturating_add(capacity_bytes(
                self.coefficients.capacity(),
                size_of::<PackedCoefficient>(),
            ))
    }

    pub(super) fn clear(&mut self) {
        self.stripe_significant.clear();
        self.stripe_neighbor.clear();
        self.stripe_visited.clear();
        self.stripe_refined.clear();
        self.stripe_negative.clear();
        for direction in &mut self.stripe_directional_neighbors {
            direction.clear();
        }
        self.dense_neighborhoods.clear();
        self.dense_cardinal_signs.clear();
        self.coefficients.clear();
    }

    fn prepare<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
        &mut self,
        spec: CodeBlockDecodeSpec,
    ) -> PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL> {
        let width = usize::from(spec.dimensions.width());
        let height = usize::from(spec.dimensions.height());
        let chunks_per_stripe = width.div_ceil(16);
        let stripe_word_count = height.div_ceil(4) * chunks_per_stripe;
        let context_stride = width + 2;
        self.stripe_significant.resize(stripe_word_count, 0);
        self.stripe_neighbor.resize(stripe_word_count, 0);
        self.stripe_visited.resize(stripe_word_count, 0);
        self.stripe_refined.resize(stripe_word_count, 0);
        if SPARSE {
            self.stripe_negative.resize(stripe_word_count, 0);
            for direction in &mut self.stripe_directional_neighbors {
                direction.resize(stripe_word_count, 0);
            }
            self.dense_neighborhoods.clear();
            self.dense_cardinal_signs.clear();
        } else {
            self.stripe_negative.clear();
            for direction in &mut self.stripe_directional_neighbors {
                direction.clear();
            }
            self.dense_neighborhoods
                .resize(context_stride * (height + 2), Neighborhood::default());
            self.dense_cardinal_signs
                .resize(context_stride * (height + 2), CardinalSigns::default());
        }
        self.coefficients
            .resize(stripe_word_count * 64, PackedCoefficient::default());
        self.stripe_significant.fill(0);
        self.stripe_neighbor.fill(0);
        self.stripe_visited.fill(0);
        self.stripe_refined.fill(0);
        if SPARSE {
            self.stripe_negative.fill(0);
            for direction in &mut self.stripe_directional_neighbors {
                direction.fill(0);
            }
        }
        self.dense_neighborhoods.fill(Neighborhood::default());
        self.dense_cardinal_signs.fill(CardinalSigns::default());
        self.coefficients.fill(PackedCoefficient::default());
        let mut context = PackedDecodeContext {
            stripe_significant: &mut self.stripe_significant,
            stripe_neighbor: &mut self.stripe_neighbor,
            stripe_visited: &mut self.stripe_visited,
            stripe_refined: &mut self.stripe_refined,
            stripe_negative: &mut self.stripe_negative,
            stripe_directional_neighbors: &mut self.stripe_directional_neighbors,
            dense_neighborhoods: &mut self.dense_neighborhoods,
            dense_cardinal_signs: &mut self.dense_cardinal_signs,
            coefficients: &mut self.coefficients,
            width,
            height,
            chunks_per_stripe,
            context_stride,
            subband: spec.subband,
            contexts: [ArithmeticDecoderContext::default(); CONTEXT_COUNT],
            current_bit_position: 0,
        };
        context.reset_contexts();
        context
    }
}

pub(super) fn decode(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut PackedDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    decode_backend::<false>(
        segment,
        coding_segments,
        spec,
        output,
        scratch,
        reconstruction,
    )
}

pub(super) fn decode_sparse(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut PackedDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    decode_backend::<true>(
        segment,
        coding_segments,
        spec,
        output,
        scratch,
        reconstruction,
    )
}

fn decode_backend<const SPARSE: bool>(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut PackedDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let bitplanes = validate_bitplane_pass_count(spec)?;
    match (
        spec.style.resets_contexts(),
        spec.style.is_vertically_causal(),
    ) {
        (false, false) => decode_with_policy::<false, false, SPARSE>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (true, false) => decode_with_policy::<true, false, SPARSE>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (false, true) => decode_with_policy::<false, true, SPARSE>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (true, true) => decode_with_policy::<true, true, SPARSE>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
    }
}

fn decode_with_policy<
    const RESET_CONTEXTS: bool,
    const VERTICAL_CAUSAL: bool,
    const SPARSE: bool,
>(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    bitplanes: u8,
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut PackedDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let mut context = scratch.prepare::<SPARSE, VERTICAL_CAUSAL>(spec);
    let mut byte_offset = 0usize;
    let mut coding_pass = 0u16;
    for coding_segment in coding_segments {
        let segment_end = byte_offset + coding_segment.byte_len;
        if is_raw_coding_pass(spec.style, coding_pass) {
            let mut decoder = RawDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_bit_position(&mut context, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::SignificancePropagation => {
                        significance_propagation_pass_raw::<SPARSE, VERTICAL_CAUSAL>(
                            &mut context,
                            &mut decoder,
                        );
                    }
                    CodingPass::MagnitudeRefinement => {
                        magnitude_refinement_pass_raw::<SPARSE, VERTICAL_CAUSAL>(
                            &mut context,
                            &mut decoder,
                        );
                    }
                    CodingPass::Cleanup => {
                        return Err(Tier1Error::MalformedBitstream {
                            reason: "BYPASS raw segment contains a cleanup pass",
                        });
                    }
                }
                if RESET_CONTEXTS {
                    context.reset_contexts();
                }
                coding_pass += 1;
            }
        } else {
            let mut decoder = ArithmeticDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_bit_position(&mut context, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::Cleanup => {
                        cleanup_pass::<SPARSE, VERTICAL_CAUSAL>(&mut context, &mut decoder)?;
                        if spec.style.has_segmentation_symbols() {
                            decode_segmentation_symbol(&mut decoder, &mut context.contexts[18])?;
                        }
                        context.stripe_visited.fill(0);
                    }
                    CodingPass::SignificancePropagation => {
                        significance_propagation_pass::<SPARSE, VERTICAL_CAUSAL>(
                            &mut context,
                            &mut decoder,
                        );
                    }
                    CodingPass::MagnitudeRefinement => {
                        magnitude_refinement_pass::<SPARSE, VERTICAL_CAUSAL>(
                            &mut context,
                            &mut decoder,
                        );
                    }
                }
                if RESET_CONTEXTS {
                    context.reset_contexts();
                }
                coding_pass += 1;
            }
            if spec.style.has_predictable_termination() {
                decoder.validate_predictable_termination()?;
            }
        }
        byte_offset = segment_end;
    }
    context.copy_coefficients_to(output, reconstruction);
    Ok(())
}

fn set_bit_position<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    bitplanes: u8,
    coding_pass: u16,
) -> Result<()> {
    let current_bitplane = coding_pass.div_ceil(3);
    context.current_bit_position = bitplanes
        .checked_sub(1)
        .and_then(|value| value.checked_sub(current_bitplane as u8))
        .ok_or(Tier1Error::MalformedBitstream {
            reason: "coding pass exceeds available bit-planes",
        })?;
    Ok(())
}

struct PackedDecodeContext<'a, const SPARSE: bool, const VERTICAL_CAUSAL: bool> {
    stripe_significant: &'a mut [u64],
    stripe_neighbor: &'a mut [u64],
    stripe_visited: &'a mut [u64],
    stripe_refined: &'a mut [u64],
    stripe_negative: &'a mut [u64],
    stripe_directional_neighbors: &'a mut [Vec<u64>; 8],
    dense_neighborhoods: &'a mut [Neighborhood],
    dense_cardinal_signs: &'a mut [CardinalSigns],
    coefficients: &'a mut [PackedCoefficient],
    width: usize,
    height: usize,
    chunks_per_stripe: usize,
    context_stride: usize,
    subband: super::Subband,
    contexts: [ArithmeticDecoderContext; CONTEXT_COUNT],
    current_bit_position: u8,
}

impl<const SPARSE: bool, const VERTICAL_CAUSAL: bool>
    PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>
{
    fn reset_contexts(&mut self) {
        reset_arithmetic_contexts(&mut self.contexts);
    }

    #[inline(always)]
    fn stripe_position(&self, x: usize, y: usize) -> (usize, u64) {
        let word = (y / 4) * self.chunks_per_stripe + x / 16;
        let bit = (x % 16) * 4 + y % 4;
        (word, 1_u64 << bit)
    }

    #[inline(always)]
    fn row_major_index(&self, x: usize, y: usize) -> usize {
        (y + 1) * self.context_stride + x + 1
    }

    fn copy_coefficients_to(&self, output: &mut [i32], reconstruction: CoefficientReconstruction) {
        debug_assert!(output.len() >= self.width * self.height);
        for stripe in 0..self.height.div_ceil(4) {
            let y_base = stripe * 4;
            let stripe_height = (self.height - y_base).min(4);
            for y_offset in 0..stripe_height {
                let output_row = (y_base + y_offset) * self.width;
                for chunk in 0..self.chunks_per_stripe {
                    let x_base = chunk * 16;
                    let chunk_width = (self.width - x_base).min(16);
                    let packed_base = (stripe * self.chunks_per_stripe + chunk) * 64;
                    for x_offset in 0..chunk_width {
                        let source_base = packed_base + x_offset * 4;
                        let coefficient = self.coefficients[source_base + y_offset];
                        output[output_row + x_base + x_offset] = match reconstruction {
                            CoefficientReconstruction::ReversibleInteger => coefficient.get(),
                            CoefficientReconstruction::IrreversibleDoubledHalfStep => {
                                coefficient.doubled_half_step()
                            }
                        };
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn set_significant(&mut self, x: usize, y: usize, sign: u8) {
        let (word, stripe_mask) = self.stripe_position(x, y);
        self.stripe_significant[word] |= stripe_mask;
        if SPARSE && sign & 1 != 0 {
            self.stripe_negative[word] |= stripe_mask;
        }
        self.mark_stripe_neighbors(word, stripe_mask, x, y);
        if SPARSE {
            self.mark_directional_neighbors(x, y);
        }
        if !SPARSE {
            self.mark_dense_neighbors(x, y, sign);
        }
        let index = word * 64 + stripe_mask.trailing_zeros() as usize;
        self.coefficients[index].set_new_significant(sign, self.current_bit_position);
    }

    #[inline(always)]
    fn mark_stripe_neighbors(&mut self, word: usize, center: u64, x: usize, y: usize) {
        let chunk_x = x % 16;
        let stripe_y = y % 4;
        if x != 0 && x + 1 < self.width && (1..15).contains(&chunk_x) {
            match stripe_y {
                0 => {
                    self.stripe_neighbor[word] |= (center >> 4)
                        | (center << 4)
                        | (center >> 3)
                        | (center << 1)
                        | (center << 5);
                    if y != 0 && !VERTICAL_CAUSAL {
                        let above = 1_u64 << (chunk_x * 4 + 3);
                        self.stripe_neighbor[word - self.chunks_per_stripe] |=
                            (above >> 4) | above | (above << 4);
                    }
                }
                1 | 2 => {
                    self.stripe_neighbor[word] |= (center >> 5)
                        | (center >> 4)
                        | (center >> 3)
                        | (center >> 1)
                        | (center << 1)
                        | (center << 3)
                        | (center << 4)
                        | (center << 5);
                }
                3 => {
                    self.stripe_neighbor[word] |= (center >> 5)
                        | (center >> 4)
                        | (center >> 1)
                        | (center << 3)
                        | (center << 4);
                    if y + 1 < self.height {
                        let below = 1_u64 << (chunk_x * 4);
                        self.stripe_neighbor[word + self.chunks_per_stripe] |=
                            (below >> 4) | below | (below << 4);
                    }
                }
                _ => unreachable!(),
            }
            return;
        }
        for delta_y in -1_isize..=1 {
            if VERTICAL_CAUSAL && y.is_multiple_of(4) && delta_y == -1 {
                continue;
            }
            for delta_x in -1_isize..=1 {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                let neighbor_x = x as isize + delta_x;
                let neighbor_y = y as isize + delta_y;
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x as usize >= self.width
                    || neighbor_y as usize >= self.height
                {
                    continue;
                }
                let (neighbor_word, neighbor_mask) =
                    self.stripe_position(neighbor_x as usize, neighbor_y as usize);
                self.stripe_neighbor[neighbor_word] |= neighbor_mask;
            }
        }
    }

    #[inline(always)]
    fn mark_dense_neighbors(&mut self, x: usize, y: usize, sign: u8) {
        let index = self.row_major_index(x, y);
        if !VERTICAL_CAUSAL || !y.is_multiple_of(4) {
            let above = index - self.context_stride;
            self.dense_neighborhoods[above - 1].bottom_right = true;
            self.dense_neighborhoods[above].bottom = true;
            self.dense_cardinal_signs[above].bottom = Some(sign);
            self.dense_neighborhoods[above + 1].bottom_left = true;
        }
        self.dense_neighborhoods[index - 1].right = true;
        self.dense_cardinal_signs[index - 1].right = Some(sign);
        self.dense_neighborhoods[index + 1].left = true;
        self.dense_cardinal_signs[index + 1].left = Some(sign);
        let below = index + self.context_stride;
        self.dense_neighborhoods[below - 1].top_right = true;
        self.dense_neighborhoods[below].top = true;
        self.dense_cardinal_signs[below].top = Some(sign);
        self.dense_neighborhoods[below + 1].top_left = true;
    }

    #[inline(always)]
    fn mark_directional_neighbors(&mut self, x: usize, y: usize) {
        const DIRECTIONS: [(isize, isize); 8] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (-1, -1),
        ];
        for (direction, (delta_x, delta_y)) in DIRECTIONS.into_iter().enumerate() {
            let Some(target_x) = x.checked_add_signed(-delta_x) else {
                continue;
            };
            let Some(target_y) = y.checked_add_signed(-delta_y) else {
                continue;
            };
            if target_x >= self.width || target_y >= self.height {
                continue;
            }
            if VERTICAL_CAUSAL && matches!(direction, 0 | 1 | 5) && (target_y + 1).is_multiple_of(4)
            {
                continue;
            }
            let (word, mask) = self.stripe_position(target_x, target_y);
            self.stripe_directional_neighbors[direction][word] |= mask;
        }
    }

    #[inline(always)]
    fn neighbors(&self, x: usize, y: usize) -> Neighborhood {
        if !SPARSE {
            return self.dense_neighborhoods[self.row_major_index(x, y)];
        }
        let mut neighbors = 0_u8;
        let (word, mask) = self.stripe_position(x, y);
        for (direction, direction_words) in self.stripe_directional_neighbors.iter().enumerate() {
            if direction_words[word] & mask != 0 {
                neighbors |= 1 << direction;
            }
        }
        Neighborhood::from_mask(neighbors)
    }

    fn decode_sign(&mut self, x: usize, y: usize, decoder: &mut ArithmeticDecoder<'_>) -> u8 {
        let (label, xor_bit) = if SPARSE {
            self.lazy_sign_context(x, y)
        } else {
            self.dense_cardinal_signs[self.row_major_index(x, y)].context()
        };
        (decoder.read_packed_bit(&mut self.contexts[usize::from(label)]) as u8) ^ xor_bit
    }

    #[inline(always)]
    fn relative_position(
        &self,
        word: usize,
        bit_index: usize,
        x: usize,
        y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> Option<(usize, u64)> {
        let neighbor_x = x.checked_add_signed(delta_x)?;
        let neighbor_y = y.checked_add_signed(delta_y)?;
        if neighbor_x >= self.width || neighbor_y >= self.height {
            return None;
        }
        let same_chunk = match delta_x {
            -1 => x & 15 != 0,
            0 => true,
            1 => x & 15 != 15,
            _ => false,
        };
        let same_stripe = match delta_y {
            -1 => y & 3 != 0,
            0 => true,
            1 => y & 3 != 3,
            _ => false,
        };
        if same_chunk && same_stripe {
            let neighbor_bit = bit_index
                .checked_add_signed(delta_x * 4 + delta_y)
                .expect("same packed word offset stays in range");
            Some((word, 1_u64 << neighbor_bit))
        } else {
            Some(self.stripe_position(neighbor_x, neighbor_y))
        }
    }

    #[inline(always)]
    fn negative_relative(
        &self,
        word: usize,
        bit_index: usize,
        x: usize,
        y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> Option<bool> {
        let (neighbor_word, neighbor_mask) =
            self.relative_position(word, bit_index, x, y, delta_x, delta_y)?;
        if self.stripe_significant[neighbor_word] & neighbor_mask == 0 {
            None
        } else {
            Some(self.stripe_negative[neighbor_word] & neighbor_mask != 0)
        }
    }

    #[inline(always)]
    fn lazy_sign_context(&self, x: usize, y: usize) -> (u8, u8) {
        let (word, mask) = self.stripe_position(x, y);
        let bit_index = mask.trailing_zeros() as usize;
        let top = self.negative_relative(word, bit_index, x, y, 0, -1);
        let left = self.negative_relative(word, bit_index, x, y, -1, 0);
        let right = self.negative_relative(word, bit_index, x, y, 1, 0);
        let bottom = if VERTICAL_CAUSAL && (y + 1).is_multiple_of(4) {
            None
        } else {
            self.negative_relative(word, bit_index, x, y, 0, 1)
        };
        CardinalSigns {
            top: top.map(u8::from),
            left: left.map(u8::from),
            right: right.map(u8::from),
            bottom: bottom.map(u8::from),
        }
        .context()
    }
}

fn cleanup_pass<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    if context.width.is_multiple_of(16) && context.height.is_multiple_of(4) {
        cleanup_pass_full_words::<SPARSE, VERTICAL_CAUSAL>(context, decoder)
    } else {
        cleanup_pass_partial_words::<SPARSE, VERTICAL_CAUSAL>(context, decoder)
    }
}

#[inline(never)]
fn cleanup_pass_partial_words<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    let mut run_context = context.contexts[17];
    let mut uniform_context = context.contexts[18];
    for base_row in (0..context.height).step_by(4) {
        let stripe_height = (context.height - base_row).min(4);
        for x in 0..context.width {
            let (word, first_mask) = context.stripe_position(x, base_row);
            let column_mask =
                first_mask | (first_mask << 1) | (first_mask << 2) | (first_mask << 3);
            let run_mode = stripe_height == 4
                && (context.stripe_significant[word]
                    | context.stripe_visited[word]
                    | context.stripe_neighbor[word])
                    & column_mask
                    == 0;
            if run_mode {
                let (run_bit, next_context) = decoder.read_packed_bit_value(run_context);
                run_context = next_context;
                if run_bit == 0 {
                    continue;
                }
                let (run_high, next_context) = decoder.read_packed_bit_value(uniform_context);
                let (run_low, next_context) = decoder.read_packed_bit_value(next_context);
                uniform_context = next_context;
                let run_length = ((run_high << 1) | run_low) as usize;
                let y = base_row + run_length;
                let sign = context.decode_sign(x, y, decoder);
                context.set_significant(x, y, sign);
                for y in (y + 1)..(base_row + stripe_height) {
                    cleanup_position::<SPARSE, VERTICAL_CAUSAL>(context, decoder, x, y);
                }
            } else {
                for y in base_row..(base_row + stripe_height) {
                    cleanup_position::<SPARSE, VERTICAL_CAUSAL>(context, decoder, x, y);
                }
            }
        }
    }
    context.contexts[17] = run_context;
    context.contexts[18] = uniform_context;
    Ok(())
}

fn cleanup_pass_full_words<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    let mut run_context = context.contexts[17];
    let mut uniform_context = context.contexts[18];
    for stripe in 0..context.height / 4 {
        let base_row = stripe * 4;
        for chunk in 0..context.chunks_per_stripe {
            let word = stripe * context.chunks_per_stripe + chunk;
            for column in 0..16 {
                let x = chunk * 16 + column;
                let first_mask = 1_u64 << (column * 4);
                let column_mask = first_mask * 0x0f;
                let unavailable = context.stripe_significant[word] | context.stripe_visited[word];
                let run_mode = (unavailable | context.stripe_neighbor[word]) & column_mask == 0;
                if run_mode {
                    let (run_bit, next_context) = decoder.read_packed_bit_value(run_context);
                    run_context = next_context;
                    if run_bit == 0 {
                        continue;
                    }
                    let (run_high, next_context) = decoder.read_packed_bit_value(uniform_context);
                    let (run_low, next_context) = decoder.read_packed_bit_value(next_context);
                    uniform_context = next_context;
                    let run_length = ((run_high << 1) | run_low) as usize;
                    let y = base_row + run_length;
                    let sign = context.decode_sign(x, y, decoder);
                    context.set_significant(x, y, sign);
                    for offset in (run_length + 1)..4 {
                        cleanup_available_position::<SPARSE, VERTICAL_CAUSAL>(
                            context,
                            decoder,
                            x,
                            base_row + offset,
                        );
                    }
                } else {
                    let mut available = column_mask & !unavailable;
                    while available != 0 {
                        let bit_index = available.trailing_zeros() as usize;
                        available &= available - 1;
                        cleanup_available_position::<SPARSE, VERTICAL_CAUSAL>(
                            context,
                            decoder,
                            x,
                            base_row + bit_index % 4,
                        );
                    }
                }
            }
        }
    }
    context.contexts[17] = run_context;
    context.contexts[18] = uniform_context;
    Ok(())
}

#[inline(always)]
fn cleanup_position<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
    x: usize,
    y: usize,
) {
    let (word, mask) = context.stripe_position(x, y);
    cleanup_position_at::<SPARSE, VERTICAL_CAUSAL>(context, decoder, x, y, word, mask);
}

#[inline(always)]
fn cleanup_position_at<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
    x: usize,
    y: usize,
    word: usize,
    mask: u64,
) {
    if (context.stripe_significant[word] | context.stripe_visited[word]) & mask != 0 {
        return;
    }
    cleanup_available_position::<SPARSE, VERTICAL_CAUSAL>(context, decoder, x, y);
}

#[inline(always)]
fn cleanup_available_position<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
    x: usize,
    y: usize,
) {
    let label = zero_coding_context(context.subband, context.neighbors(x, y));
    if decoder.read_packed_bit(&mut context.contexts[usize::from(label)]) != 0 {
        let sign = context.decode_sign(x, y, decoder);
        context.set_significant(x, y, sign);
    }
}

fn significance_propagation_pass<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    if context.width.is_multiple_of(16) && context.height.is_multiple_of(4) {
        significance_propagation_pass_impl::<true, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    } else {
        significance_propagation_pass_impl::<false, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    }
}

fn significance_propagation_pass_impl<
    const FULL_WORDS: bool,
    const SPARSE: bool,
    const VERTICAL_CAUSAL: bool,
>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    for stripe in 0..context.height.div_ceil(4) {
        for chunk in 0..context.chunks_per_stripe {
            let word = stripe * context.chunks_per_stripe + chunk;
            let mut consumed_mask = 0_u64;
            loop {
                let candidates = context.stripe_neighbor[word]
                    & !context.stripe_significant[word]
                    & !consumed_mask;
                if candidates == 0 {
                    break;
                }
                let bit_index = candidates.trailing_zeros() as usize;
                consumed_mask |= if bit_index == 63 {
                    u64::MAX
                } else {
                    (1_u64 << (bit_index + 1)) - 1
                };
                let x = chunk * 16 + bit_index / 4;
                let y = stripe * 4 + bit_index % 4;
                if !FULL_WORDS && (x >= context.width || y >= context.height) {
                    continue;
                }
                let label = zero_coding_context(context.subband, context.neighbors(x, y));
                let mask = 1_u64 << bit_index;
                context.stripe_visited[word] |= mask;
                if decoder.read_packed_bit(&mut context.contexts[usize::from(label)]) != 0 {
                    let sign = context.decode_sign(x, y, decoder);
                    context.set_significant(x, y, sign);
                }
            }
        }
    }
}

fn significance_propagation_pass_raw<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut RawDecoder<'_>,
) {
    if context.width.is_multiple_of(16) && context.height.is_multiple_of(4) {
        significance_propagation_pass_raw_impl::<true, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    } else {
        significance_propagation_pass_raw_impl::<false, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    }
}

fn significance_propagation_pass_raw_impl<
    const FULL_WORDS: bool,
    const SPARSE: bool,
    const VERTICAL_CAUSAL: bool,
>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut RawDecoder<'_>,
) {
    for stripe in 0..context.height.div_ceil(4) {
        for chunk in 0..context.chunks_per_stripe {
            let word = stripe * context.chunks_per_stripe + chunk;
            let mut consumed_mask = 0_u64;
            loop {
                let candidates = context.stripe_neighbor[word]
                    & !context.stripe_significant[word]
                    & !consumed_mask;
                if candidates == 0 {
                    break;
                }
                let bit_index = candidates.trailing_zeros() as usize;
                consumed_mask |= if bit_index == 63 {
                    u64::MAX
                } else {
                    (1_u64 << (bit_index + 1)) - 1
                };
                let x = chunk * 16 + bit_index / 4;
                let y = stripe * 4 + bit_index % 4;
                if !FULL_WORDS && (x >= context.width || y >= context.height) {
                    continue;
                }
                let mask = 1_u64 << bit_index;
                context.stripe_visited[word] |= mask;
                if decoder.read_bit() != 0 {
                    let sign = decoder.read_bit() as u8;
                    context.set_significant(x, y, sign);
                }
            }
        }
    }
}

fn magnitude_refinement_pass<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    if context.width.is_multiple_of(16) && context.height.is_multiple_of(4) {
        if SPARSE {
            magnitude_refinement_pass_impl::<true, SPARSE, VERTICAL_CAUSAL>(context, decoder);
        } else {
            magnitude_refinement_pass_dense_full(context, decoder);
        }
    } else {
        magnitude_refinement_pass_impl::<false, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    }
}

fn magnitude_refinement_pass_dense_full<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    debug_assert!(!SPARSE);
    let mut first_no_neighbor_context = context.contexts[14];
    let mut first_neighbor_context = context.contexts[15];
    let mut repeat_context = context.contexts[16];
    let chunks_per_stripe = context.chunks_per_stripe;
    let context_stride = context.context_stride;
    let dense_neighborhoods = &*context.dense_neighborhoods;
    for (word, (((significant, visited), refined), coefficients)) in context
        .stripe_significant
        .iter()
        .copied()
        .zip(context.stripe_visited.iter().copied())
        .zip(context.stripe_refined.iter_mut())
        .zip(context.coefficients.chunks_exact_mut(64))
        .enumerate()
    {
        let mut candidates = significant & !visited;
        if candidates & !*refined == 0 {
            repeat_context = refine_repeat_candidates_resident(
                candidates,
                coefficients,
                context.current_bit_position,
                repeat_context,
                decoder,
            );
            continue;
        }
        let stripe = word / chunks_per_stripe;
        let chunk = word % chunks_per_stripe;
        let mut refined_word = *refined;
        while candidates != 0 {
            let bit_index = candidates.trailing_zeros() as usize;
            candidates &= candidates - 1;
            let mask = 1_u64 << bit_index;
            let value = if refined_word & mask != 0 {
                let (value, next_context) = decoder.read_packed_bit_value(repeat_context);
                repeat_context = next_context;
                value
            } else {
                let x = chunk * 16 + bit_index / 4;
                let y = stripe * 4 + bit_index % 4;
                let neighbors = dense_neighborhoods[(y + 1) * context_stride + x + 1];
                if neighbors.any() {
                    let (value, next_context) =
                        decoder.read_packed_bit_value(first_neighbor_context);
                    first_neighbor_context = next_context;
                    value
                } else {
                    let (value, next_context) =
                        decoder.read_packed_bit_value(first_no_neighbor_context);
                    first_no_neighbor_context = next_context;
                    value
                }
            };
            coefficients[bit_index].refine(value, context.current_bit_position);
            refined_word |= mask;
        }
        *refined = refined_word;
    }
    context.contexts[14] = first_no_neighbor_context;
    context.contexts[15] = first_neighbor_context;
    context.contexts[16] = repeat_context;
}

/// Decode repeat-context refinement candidates through the shared Annex C
/// state machine.
fn refine_repeat_candidates_resident(
    mut candidates: u64,
    coefficients: &mut [PackedCoefficient],
    bit_position: u8,
    mut context: ArithmeticDecoderContext,
    decoder: &mut ArithmeticDecoder<'_>,
) -> ArithmeticDecoderContext {
    while candidates != 0 {
        let bit_index = candidates.trailing_zeros() as usize;
        candidates &= candidates - 1;
        let (decision, next_context) = decoder.read_packed_bit_value(context);
        context = next_context;
        coefficients[bit_index].refine(decision, bit_position);
    }
    context
}

fn magnitude_refinement_pass_impl<
    const FULL_WORDS: bool,
    const SPARSE: bool,
    const VERTICAL_CAUSAL: bool,
>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    let mut first_no_neighbor_context = context.contexts[14];
    let mut first_neighbor_context = context.contexts[15];
    let mut repeat_context = context.contexts[16];
    for stripe in 0..context.height.div_ceil(4) {
        for chunk in 0..context.chunks_per_stripe {
            let word = stripe * context.chunks_per_stripe + chunk;
            let mut candidates = context.stripe_significant[word] & !context.stripe_visited[word];
            let mut refined = context.stripe_refined[word];
            let coefficient_base = word * 64;
            if candidates & !refined == 0 {
                while candidates != 0 {
                    let bit_index = candidates.trailing_zeros() as usize;
                    candidates &= candidates - 1;
                    let (value, next_context) = decoder.read_packed_bit_value(repeat_context);
                    repeat_context = next_context;
                    context.coefficients[coefficient_base + bit_index]
                        .refine(value, context.current_bit_position);
                }
                continue;
            }
            while candidates != 0 {
                let bit_index = candidates.trailing_zeros() as usize;
                candidates &= candidates - 1;
                let x = chunk * 16 + bit_index / 4;
                let y = stripe * 4 + bit_index % 4;
                if !FULL_WORDS && (x >= context.width || y >= context.height) {
                    continue;
                }
                let mask = 1_u64 << bit_index;
                let value = if refined & mask != 0 {
                    let (value, next_context) = decoder.read_packed_bit_value(repeat_context);
                    repeat_context = next_context;
                    value
                } else if context.neighbors(x, y).any() {
                    let (value, next_context) =
                        decoder.read_packed_bit_value(first_neighbor_context);
                    first_neighbor_context = next_context;
                    value
                } else {
                    let (value, next_context) =
                        decoder.read_packed_bit_value(first_no_neighbor_context);
                    first_no_neighbor_context = next_context;
                    value
                };
                let index = coefficient_base + bit_index;
                context.coefficients[index].refine(value, context.current_bit_position);
                refined |= mask;
            }
            context.stripe_refined[word] = refined;
        }
    }
    context.contexts[14] = first_no_neighbor_context;
    context.contexts[15] = first_neighbor_context;
    context.contexts[16] = repeat_context;
}

fn magnitude_refinement_pass_raw<const SPARSE: bool, const VERTICAL_CAUSAL: bool>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut RawDecoder<'_>,
) {
    if context.width.is_multiple_of(16) && context.height.is_multiple_of(4) {
        magnitude_refinement_pass_raw_impl::<true, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    } else {
        magnitude_refinement_pass_raw_impl::<false, SPARSE, VERTICAL_CAUSAL>(context, decoder);
    }
}

fn magnitude_refinement_pass_raw_impl<
    const FULL_WORDS: bool,
    const SPARSE: bool,
    const VERTICAL_CAUSAL: bool,
>(
    context: &mut PackedDecodeContext<'_, SPARSE, VERTICAL_CAUSAL>,
    decoder: &mut RawDecoder<'_>,
) {
    for stripe in 0..context.height.div_ceil(4) {
        for chunk in 0..context.chunks_per_stripe {
            let word = stripe * context.chunks_per_stripe + chunk;
            let mut candidates = context.stripe_significant[word] & !context.stripe_visited[word];
            let mut refined = context.stripe_refined[word];
            let coefficient_base = word * 64;
            while candidates != 0 {
                let bit_index = candidates.trailing_zeros() as usize;
                candidates &= candidates - 1;
                let x = chunk * 16 + bit_index / 4;
                let y = stripe * 4 + bit_index % 4;
                if !FULL_WORDS && (x >= context.width || y >= context.height) {
                    continue;
                }
                let mask = 1_u64 << bit_index;
                let value = decoder.read_bit();
                let index = coefficient_base + bit_index;
                context.coefficients[index].refine(value, context.current_bit_position);
                refined |= mask;
            }
            context.stripe_refined[word] = refined;
        }
    }
}
