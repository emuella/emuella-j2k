#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, allow(dead_code))]
//! Classic JPEG 2000 tier-1 block coding boundary.
//!
//! This crate owns arithmetic coding state and code-block pass handling for
//! Part 1 codestreams. It exposes narrow block-level APIs to codestream
//! orchestration; it does not own images, containers, or CLI behavior.

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

mod mq;
mod packed_decode;

use mq::{
    Context as ArithmeticDecoderContext, Decoder as ArithmeticDecoder,
    Encoder as ArithmeticEncoder, RawDecoder, reset_contexts as reset_arithmetic_contexts,
};

/// Result type returned by tier-1 block decode APIs.
pub type Result<T> = core::result::Result<T, Tier1Error>;

/// Tier-1 coding pass family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodingPass {
    /// Significance propagation pass.
    SignificancePropagation,
    /// Magnitude refinement pass.
    MagnitudeRefinement,
    /// Cleanup pass.
    Cleanup,
}

/// Errors reported by classic tier-1 block decode primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tier1Error {
    /// Code-block dimensions are invalid for the narrow baseline decoder.
    InvalidCodeBlock {
        /// Code-block width in coefficients.
        width: u16,
        /// Code-block height in coefficients.
        height: u16,
    },
    /// The caller-provided coefficient buffer cannot hold the code-block.
    CoefficientBufferTooSmall {
        /// Required coefficient count.
        required: usize,
        /// Provided coefficient count.
        actual: usize,
    },
    /// The code-block style uses modes outside the first accepted subset.
    UnsupportedCodingStyle {
        /// Raw JPEG 2000 code-block style byte.
        style: u8,
        /// Bits that are not accepted by this crate's baseline profile.
        unsupported_bits: u8,
    },
    /// The code-block bitstream is structurally malformed.
    MalformedBitstream {
        /// Static diagnostic suitable for mapping by higher layers.
        reason: &'static str,
    },
    /// A requested coding pass is outside the implemented fail-closed slice.
    UnsupportedCodingPass {
        /// Coding pass family that would have to be decoded next.
        pass: CodingPass,
        /// Static diagnostic suitable for mapping by higher layers.
        reason: &'static str,
    },
}

impl fmt::Display for Tier1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCodeBlock { width, height } => {
                write!(
                    f,
                    "invalid classic tier-1 code-block dimensions {width}x{height}"
                )
            }
            Self::CoefficientBufferTooSmall { required, actual } => {
                write!(
                    f,
                    "coefficient buffer is too small for code-block: required {required}, got {actual}"
                )
            }
            Self::UnsupportedCodingStyle {
                style,
                unsupported_bits,
            } => write!(
                f,
                "unsupported classic tier-1 code-block style 0x{style:02x} (unsupported bits 0x{unsupported_bits:02x})"
            ),
            Self::MalformedBitstream { reason } => {
                write!(f, "malformed classic tier-1 bitstream: {reason}")
            }
            Self::UnsupportedCodingPass { pass, reason } => {
                write!(f, "unsupported classic tier-1 {pass:?} pass: {reason}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Tier1Error {}

/// Code-block dimensions accepted by the narrow baseline tier-1 decoder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockDimensions {
    width: u16,
    height: u16,
}

impl CodeBlockDimensions {
    /// JPEG 2000 Part 1 code-blocks are limited to 4096 samples.
    pub const MAX_COEFFICIENTS: usize = 4096;

    /// Create validated code-block dimensions.
    pub fn new(width: u16, height: u16) -> Result<Self> {
        let dimensions = Self { width, height };
        let samples = dimensions
            .coefficient_count_checked()
            .ok_or(Tier1Error::InvalidCodeBlock { width, height })?;
        if samples == 0 || samples > Self::MAX_COEFFICIENTS {
            return Err(Tier1Error::InvalidCodeBlock { width, height });
        }
        Ok(dimensions)
    }

    /// Width in coefficients.
    pub fn width(self) -> u16 {
        self.width
    }

    /// Height in coefficients.
    pub fn height(self) -> u16 {
        self.height
    }

    /// Total coefficient count.
    pub fn coefficient_count(self) -> usize {
        usize::from(self.width) * usize::from(self.height)
    }

    fn coefficient_count_checked(self) -> Option<usize> {
        usize::from(self.width).checked_mul(usize::from(self.height))
    }
}

/// Raw classic Part 1 code-block style byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockStyle {
    bits: u8,
}

impl CodeBlockStyle {
    /// No selective arithmetic bypass, resets, terminations, vertical causal
    /// mode, predictable termination, or segmentation symbols.
    pub const NONE: Self = Self { bits: 0 };

    /// Selective arithmetic coding bypass.
    pub const SELECTIVE_ARITHMETIC_BYPASS: u8 = 0x01;
    /// Reset context probabilities on coding pass boundaries.
    pub const RESET_CONTEXTS: u8 = 0x02;
    /// Terminate every coding pass.
    pub const TERMINATE_EACH_PASS: u8 = 0x04;
    /// Vertically stripe-causal context formation.
    pub const VERTICALLY_CAUSAL: u8 = 0x08;
    /// Predictable MQ termination.
    pub const PREDICTABLE_TERMINATION: u8 = 0x10;
    /// Segmentation symbols in cleanup passes.
    pub const SEGMENTATION_SYMBOLS: u8 = 0x20;

    /// Create a style wrapper from the COD/COC style byte.
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }

    /// Raw style bits.
    pub fn bits(self) -> u8 {
        self.bits
    }

    /// Validate this style against the repo-owned classic Tier-1 subset.
    pub fn validate_baseline(self) -> Result<()> {
        let supported_bits = Self::SELECTIVE_ARITHMETIC_BYPASS
            | Self::RESET_CONTEXTS
            | Self::TERMINATE_EACH_PASS
            | Self::VERTICALLY_CAUSAL
            | Self::PREDICTABLE_TERMINATION
            | Self::SEGMENTATION_SYMBOLS;
        let unsupported_bits = self.bits & !supported_bits;
        if unsupported_bits == 0 {
            return Ok(());
        }
        Err(Tier1Error::UnsupportedCodingStyle {
            style: self.bits,
            unsupported_bits,
        })
    }

    fn has_segmentation_symbols(self) -> bool {
        self.bits & Self::SEGMENTATION_SYMBOLS != 0
    }

    fn resets_contexts(self) -> bool {
        self.bits & Self::RESET_CONTEXTS != 0
    }

    fn is_vertically_causal(self) -> bool {
        self.bits & Self::VERTICALLY_CAUSAL != 0
    }

    fn has_predictable_termination(self) -> bool {
        self.bits & Self::PREDICTABLE_TERMINATION != 0
    }

    fn uses_selective_arithmetic_bypass(self) -> bool {
        self.bits & Self::SELECTIVE_ARITHMETIC_BYPASS != 0
    }

    fn terminates_each_pass(self) -> bool {
        self.bits & Self::TERMINATE_EACH_PASS != 0
    }
}

/// JPEG 2000 subband kind used for tier-1 zero-coding context formation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subband {
    LowLow,
    LowHigh,
    HighLow,
    HighHigh,
}

/// Immutable description for decoding one classic tier-1 code-block segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockDecodeSpec {
    /// Code-block dimensions in coefficients.
    pub dimensions: CodeBlockDimensions,
    /// Number of magnitude bitplanes available to the code-block decoder.
    ///
    /// The narrow no-decomposition unsigned profile rows pass component
    /// precision plus one for the JPEG 2000 DC level-shifted coefficient range.
    pub available_bitplanes: u8,
    /// Number of most-significant bit-planes skipped by packet headers.
    pub missing_most_significant_bitplanes: u8,
    /// Number of coding passes contributed by the packet/layer state.
    pub coding_passes: u16,
    /// Code-block style from COD/COC.
    pub style: CodeBlockStyle,
    /// Subband context for zero-coding labels.
    pub subband: Subband,
}

/// One independently terminated arithmetic-coding segment within a code-block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockSegment {
    /// Number of bytes occupied by this segment in the contiguous input.
    pub byte_len: usize,
    /// Number of coding passes carried by this segment.
    pub coding_passes: u16,
}

impl CodeBlockDecodeSpec {
    /// Validate dimensions, style, and basic packet-derived fields.
    pub fn validate(self) -> Result<()> {
        self.style.validate_baseline()?;
        if self.coding_passes > 0xff {
            return Err(Tier1Error::UnsupportedCodingPass {
                pass: coding_pass_for_index(0),
                reason: "more than 255 coding passes are outside the baseline block slice",
            });
        }
        if self.available_bitplanes == 0 {
            return Err(Tier1Error::MalformedBitstream {
                reason: "code-block decode requires at least one available bit-plane",
            });
        }
        Ok(())
    }
}

/// Immutable description for encoding one classic tier-1 code-block segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockEncodeSpec {
    /// Code-block dimensions in coefficients.
    pub dimensions: CodeBlockDimensions,
    /// Subband context for zero-coding labels.
    pub subband: Subband,
    /// Number of magnitude bitplanes available to the code-block encoder.
    pub available_bitplanes: u8,
    /// Raw JPEG 2000 code-block style byte.
    pub code_block_style: u8,
}

impl CodeBlockEncodeSpec {
    /// Validate dimensions, style, and basic encoder fields.
    pub fn validate(self) -> Result<()> {
        CodeBlockStyle::from_bits(self.code_block_style).validate_baseline()?;
        if self.available_bitplanes == 0 {
            return Err(Tier1Error::MalformedBitstream {
                reason: "code-block encode requires at least one available bit-plane",
            });
        }
        Ok(())
    }
}

/// Summary returned after a code-block encode attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockEncode {
    /// Number of coding passes emitted for this segment.
    pub pass_count: u16,
    /// Whether this code-block is included in the packet.
    pub included: bool,
    /// Number of most-significant bitplanes skipped by packet headers.
    pub missing_bitplanes: u8,
    /// Number of bytes appended to the output buffer.
    pub byte_len: usize,
}

/// A stable byte-prefix boundary after an actual coding pass.
///
/// This is fixture-only encoder evidence. `stable_byte_len` counts bytes that
/// have already left the MQ coder's carry-sensitive pending state, so later
/// passes cannot rewrite the reported prefix.
#[cfg(feature = "test-fixtures")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockPassBoundary {
    /// Number of leading coding passes represented at this boundary.
    pub coding_passes: u16,
    /// Stable leading bytes available at this boundary.
    pub stable_byte_len: usize,
}

/// Summary returned after a code-block decode attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockDecode {
    /// Number of coded bytes consumed from the code-block segment.
    pub bytes_consumed: usize,
    /// Number of coding passes reconstructed into the output buffer.
    pub passes_decoded: u16,
    /// Number of coefficients covered by the block.
    pub coefficients: usize,
}

/// Classic Tier-1 state backend that actually executed a code-block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier1DecodeBackend {
    /// Checked padded row-major state and coefficient planes.
    Checked,
    /// Dense-oriented 4x16 packed candidate state with row-major contexts.
    PackedDense,
    /// Sparse-oriented packed state without a row-major context plane.
    PackedSparse,
}

impl Tier1DecodeBackend {
    /// Stable diagnostic name used by benchmark and qualification output.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::PackedDense => "packed-dense",
            Self::PackedSparse => "packed-sparse",
        }
    }
}

/// Decode summary plus authoritative executed-backend and byte attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeBlockDecodeOutcome {
    /// Existing decode summary.
    pub decoded: CodeBlockDecode,
    /// Backend that actually reconstructed the block.
    pub backend: Tier1DecodeBackend,
    /// Bytes decoded through arithmetic MQ segments.
    pub mq_bytes: usize,
    /// Bytes decoded through selective arithmetic-bypass raw segments.
    pub raw_bypass_bytes: usize,
}

/// Reusable scratch buffers for classic tier-1 code-block decode.
///
/// The buffers are opaque so callers can reuse allocation capacity without
/// depending on coefficient-state internals.
#[derive(Default)]
pub struct CodeBlockDecodeScratch {
    coefficient_states: Vec<CoefficientState>,
    coefficients: Vec<Coefficient>,
    packed: packed_decode::PackedDecodeScratch,
    sparse: packed_decode::PackedDecodeScratch,
}

impl CodeBlockDecodeScratch {
    /// Create an empty scratch object. Capacity grows to the largest decoded
    /// code-block and is reused by later calls.
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop current scratch lengths while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.coefficient_states.clear();
        self.coefficients.clear();
        self.packed.clear();
        self.sparse.clear();
    }

    /// Capacity-based heap bytes retained for reuse.
    ///
    /// Allocator metadata is excluded. The result counts each vector's
    /// retained element capacity, including the alternate packed backend.
    pub fn retained_heap_bytes(&self) -> u64 {
        let capacity_bytes = |capacity: usize, element_bytes: usize| {
            u64::try_from(capacity)
                .unwrap_or(u64::MAX)
                .saturating_mul(u64::try_from(element_bytes).unwrap_or(u64::MAX))
        };
        capacity_bytes(
            self.coefficient_states.capacity(),
            core::mem::size_of::<CoefficientState>(),
        )
        .saturating_add(capacity_bytes(
            self.coefficients.capacity(),
            core::mem::size_of::<Coefficient>(),
        ))
        .saturating_add(self.packed.retained_heap_bytes())
        .saturating_add(self.sparse.retained_heap_bytes())
    }

    fn prepare(
        &mut self,
        width: usize,
        height: usize,
        subband: Subband,
    ) -> BitPlaneDecodeContext<'_> {
        let padded_width = width + COEFFICIENTS_PADDING * 2;
        let padded_height = height + COEFFICIENTS_PADDING * 2;
        let len = padded_width * padded_height;
        self.coefficient_states
            .resize(len, CoefficientState::default());
        self.coefficients.resize(len, Coefficient::default());
        self.coefficient_states[..len].fill(CoefficientState::default());
        self.coefficients[..len].fill(Coefficient::default());

        let mut ctx = BitPlaneDecodeContext {
            coefficient_states: &mut self.coefficient_states[..len],
            coefficients: &mut self.coefficients[..len],
            width,
            padded_width,
            height,
            subband,
            contexts: [ArithmeticDecoderContext::default(); CONTEXT_COUNT],
            current_bit_position: 0,
        };
        ctx.reset_contexts();
        ctx
    }
}

/// Fine-grained opt-in timing counters for classic tier-1 decode.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeBlockDecodeTimings {
    pub scratch_prepare_ns: u128,
    pub cleanup_pass_ns: u128,
    pub bitplane_reset_ns: u128,
    pub significance_propagation_pass_ns: u128,
    pub magnitude_refinement_pass_ns: u128,
    pub coefficient_copy_out_ns: u128,
}

#[cfg(feature = "std")]
impl CodeBlockDecodeTimings {
    pub fn add_assign(&mut self, other: &Self) {
        self.scratch_prepare_ns += other.scratch_prepare_ns;
        self.cleanup_pass_ns += other.cleanup_pass_ns;
        self.bitplane_reset_ns += other.bitplane_reset_ns;
        self.significance_propagation_pass_ns += other.significance_propagation_pass_ns;
        self.magnitude_refinement_pass_ns += other.magnitude_refinement_pass_ns;
        self.coefficient_copy_out_ns += other.coefficient_copy_out_ns;
    }
}

/// Opt-in work counters for classic tier-1 decode profiling.
///
/// These counters are profiling provenance for benchmark runs. They are not
/// updated by the normal decode path.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodeBlockDecodeWorkCounters {
    pub cleanup_positions_visited: u64,
    pub cleanup_mq_reads: u64,
    pub cleanup_new_significant: u64,
    pub significance_positions_visited: u64,
    pub significance_candidates: u64,
    pub significance_mq_reads: u64,
    pub significance_new_significant: u64,
    pub magnitude_positions_visited: u64,
    pub magnitude_candidates: u64,
    pub magnitude_mq_reads: u64,
    pub magnitude_refinements: u64,
    pub magnitude_first_refinements: u64,
    pub magnitude_repeat_refinements: u64,
    pub sign_mq_reads: u64,
    pub run_mode_mq_reads: u64,
    pub context_reads_by_label: [u64; CONTEXT_COUNT],
}

impl CodeBlockDecodeWorkCounters {
    pub fn add_assign(&mut self, other: &Self) {
        self.cleanup_positions_visited += other.cleanup_positions_visited;
        self.cleanup_mq_reads += other.cleanup_mq_reads;
        self.cleanup_new_significant += other.cleanup_new_significant;
        self.significance_positions_visited += other.significance_positions_visited;
        self.significance_candidates += other.significance_candidates;
        self.significance_mq_reads += other.significance_mq_reads;
        self.significance_new_significant += other.significance_new_significant;
        self.magnitude_positions_visited += other.magnitude_positions_visited;
        self.magnitude_candidates += other.magnitude_candidates;
        self.magnitude_mq_reads += other.magnitude_mq_reads;
        self.magnitude_refinements += other.magnitude_refinements;
        self.magnitude_first_refinements += other.magnitude_first_refinements;
        self.magnitude_repeat_refinements += other.magnitude_repeat_refinements;
        self.sign_mq_reads += other.sign_mq_reads;
        self.run_mode_mq_reads += other.run_mode_mq_reads;
        for (dst, src) in self
            .context_reads_by_label
            .iter_mut()
            .zip(other.context_reads_by_label)
        {
            *dst += src;
        }
    }
}

/// Decode one classic Part 1 code-block for the accepted baseline subset.
///
/// A zero-pass block is fully decoded as all-zero coefficients. Nonzero blocks
/// execute the complete cleanup, significance-propagation, sign, and
/// magnitude-refinement pass sequence for the admitted classic coding styles.
pub fn decode_baseline_code_block(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
) -> Result<CodeBlockDecode> {
    let mut scratch = CodeBlockDecodeScratch::new();
    decode_baseline_code_block_with_scratch(segment, spec, coefficients, &mut scratch)
}

/// Decode one classic Part 1 code-block using caller-provided reusable scratch.
pub fn decode_baseline_code_block_with_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_baseline_code_block_segments_with_scratch(
        segment,
        &coding_segments,
        spec,
        coefficients,
        scratch,
    )
}

/// Decode one irreversible classic Part 1 code-block while retaining Tier-1's
/// doubled one-plus-half reconstruction state.
///
/// A returned positive quantization index `q` is represented as `2*q + 1`, a
/// negative index as `2*q - 1`, and zero remains zero. Multiply by
/// `0.5 * Delta_b` during dequantization. This deliberately separate API keeps
/// the established reversible integer output contract unchanged.
pub fn decode_irreversible_code_block_with_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_irreversible_code_block_segments_with_scratch(
        segment,
        &coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
}

/// Decode one classic Part 1 code-block with explicit terminated segments.
///
/// TERMALL code-blocks require one descriptor per coding pass. The input bytes
/// remain contiguous and descriptors identify the MQ restart boundaries.
pub fn decode_baseline_code_block_segments(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
) -> Result<CodeBlockDecode> {
    let mut scratch = CodeBlockDecodeScratch::new();
    decode_baseline_code_block_segments_with_scratch(
        segment,
        coding_segments,
        spec,
        coefficients,
        &mut scratch,
    )
}

/// Decode explicitly terminated classic Part 1 segments with reusable scratch.
pub fn decode_baseline_code_block_segments_with_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    if spec.coding_passes == 0 {
        coefficients[..coefficient_count].fill(0);
        return Ok(CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        });
    }

    decode_normal_bitplanes_with_scratch_segments(
        segment,
        coding_segments,
        spec,
        &mut coefficients[..coefficient_count],
        scratch,
        CoefficientReconstruction::ReversibleInteger,
    )?;

    Ok(CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    })
}

/// Decode explicitly terminated irreversible code-block segments while
/// retaining doubled one-plus-half coefficients for dequantization.
pub fn decode_irreversible_code_block_segments_with_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if doubled_half_step_coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: doubled_half_step_coefficients.len(),
        });
    }
    if spec.coding_passes == 0 {
        doubled_half_step_coefficients[..coefficient_count].fill(0);
        return Ok(CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        });
    }

    decode_normal_bitplanes_with_scratch_segments(
        segment,
        coding_segments,
        spec,
        &mut doubled_half_step_coefficients[..coefficient_count],
        scratch,
        CoefficientReconstruction::IrreversibleDoubledHalfStep,
    )?;
    Ok(CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    })
}

/// Decode through the packed-row state backend.
#[doc(hidden)]
pub fn decode_baseline_code_block_with_packed_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_baseline_code_block_segments_with_packed_scratch(
        segment,
        &coding_segments,
        spec,
        coefficients,
        scratch,
    )
}

/// Decode explicitly terminated segments through the packed-row backend.
#[doc(hidden)]
pub fn decode_baseline_code_block_segments_with_packed_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_baseline_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode reversible segments through the forced packed route and report the
/// backend that actually executed. Widths above the packed limit fall back to
/// checked execution and report that fact.
#[doc(hidden)]
pub fn decode_baseline_code_block_segments_with_packed_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    decode_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        coefficients,
        scratch,
        CoefficientReconstruction::ReversibleInteger,
        Tier1DecodeBackend::PackedDense,
    )
}

/// Decode reversible terminated segments through the forced sparse packed
/// route and report the backend that actually executed.
#[doc(hidden)]
pub fn decode_baseline_code_block_segments_with_sparse_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    decode_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        coefficients,
        scratch,
        CoefficientReconstruction::ReversibleInteger,
        Tier1DecodeBackend::PackedSparse,
    )
}

/// Decode one irreversible block through the forced packed route while
/// retaining doubled one-plus-half reconstruction coefficients.
#[doc(hidden)]
pub fn decode_irreversible_code_block_with_packed_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_irreversible_code_block_segments_with_packed_scratch_outcome(
        segment,
        &coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode irreversible terminated segments through the forced packed route.
#[doc(hidden)]
pub fn decode_irreversible_code_block_segments_with_packed_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_irreversible_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode irreversible terminated segments through the forced packed route and
/// report the backend that actually executed.
#[doc(hidden)]
pub fn decode_irreversible_code_block_segments_with_packed_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    decode_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
        CoefficientReconstruction::IrreversibleDoubledHalfStep,
        Tier1DecodeBackend::PackedDense,
    )
}

/// Decode irreversible terminated segments through the forced sparse packed
/// route and report the backend that actually executed.
#[doc(hidden)]
pub fn decode_irreversible_code_block_segments_with_sparse_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    decode_code_block_segments_with_packed_scratch_outcome(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
        CoefficientReconstruction::IrreversibleDoubledHalfStep,
        Tier1DecodeBackend::PackedSparse,
    )
}

fn decode_code_block_segments_with_packed_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    reconstruction: CoefficientReconstruction,
    requested_backend: Tier1DecodeBackend,
) -> Result<CodeBlockDecodeOutcome> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;
    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }
    if spec.coding_passes == 0 {
        coefficients[..coefficient_count].fill(0);
        let decoded = CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        };
        return decode_outcome(decoded, Tier1DecodeBackend::Checked, coding_segments, spec);
    }
    if spec.dimensions.width() > 64 {
        let decoded = match reconstruction {
            CoefficientReconstruction::ReversibleInteger => {
                decode_baseline_code_block_segments_with_scratch(
                    segment,
                    coding_segments,
                    spec,
                    coefficients,
                    scratch,
                )
            }
            CoefficientReconstruction::IrreversibleDoubledHalfStep => {
                decode_irreversible_code_block_segments_with_scratch(
                    segment,
                    coding_segments,
                    spec,
                    coefficients,
                    scratch,
                )
            }
        }?;
        return decode_outcome(decoded, Tier1DecodeBackend::Checked, coding_segments, spec);
    }
    match requested_backend {
        Tier1DecodeBackend::PackedDense => packed_decode::decode(
            segment,
            coding_segments,
            spec,
            &mut coefficients[..coefficient_count],
            &mut scratch.packed,
            reconstruction,
        )?,
        Tier1DecodeBackend::PackedSparse => packed_decode::decode_sparse(
            segment,
            coding_segments,
            spec,
            &mut coefficients[..coefficient_count],
            &mut scratch.sparse,
            reconstruction,
        )?,
        Tier1DecodeBackend::Checked => unreachable!("forced packed decode requested checked"),
    }
    let decoded = CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    };
    decode_outcome(decoded, requested_backend, coding_segments, spec)
}

/// Report whether the packed backend is recommended for this code-block.
///
/// The packed state currently wins on operational reversible blocks up to the
/// common 64-column shape at up to 32 coefficients per coded byte. Sparser
/// blocks retain the checked backend, whose direct context plane is cheaper
/// than preparing the packed candidate masks for that workload.
#[doc(hidden)]
pub fn packed_decode_is_recommended(segment_len: usize, spec: CodeBlockDecodeSpec) -> bool {
    recommended_decode_backend(segment_len, spec) == Tier1DecodeBackend::PackedDense
}

/// Return the measured production backend recommendation for one block.
#[doc(hidden)]
pub fn recommended_decode_backend(
    segment_len: usize,
    spec: CodeBlockDecodeSpec,
) -> Tier1DecodeBackend {
    if usize::from(spec.dimensions.width()) <= 64
        && spec.dimensions.coefficient_count() <= segment_len.saturating_mul(32)
    {
        Tier1DecodeBackend::PackedDense
    } else {
        Tier1DecodeBackend::Checked
    }
}

/// Return the measured irreversible production backend recommendation.
///
/// Operational 9/7 products show the dense packed state winning through 2.5
/// coefficients per coded byte. Reversible dispatch retains its narrower
/// corpus-qualified threshold because sparse reversible qualification samples do not share
/// that win consistently.
#[doc(hidden)]
pub fn recommended_irreversible_decode_backend(
    segment_len: usize,
    spec: CodeBlockDecodeSpec,
) -> Tier1DecodeBackend {
    if usize::from(spec.dimensions.width()) <= 64
        && spec.dimensions.coefficient_count().saturating_mul(2) <= segment_len.saturating_mul(5)
    {
        Tier1DecodeBackend::PackedDense
    } else {
        Tier1DecodeBackend::Checked
    }
}

/// Decode one code-block through the measured packed/checked backend policy.
#[doc(hidden)]
pub fn decode_baseline_code_block_with_adaptive_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_baseline_code_block_with_adaptive_scratch_outcome(segment, spec, coefficients, scratch)
        .map(|outcome| outcome.decoded)
}

/// Decode one reversible block through adaptive dispatch and report the exact
/// backend that executed.
#[doc(hidden)]
pub fn decode_baseline_code_block_with_adaptive_scratch_outcome(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_baseline_code_block_segments_with_adaptive_scratch_outcome(
        segment,
        &coding_segments,
        spec,
        coefficients,
        scratch,
    )
}

/// Decode explicitly terminated segments through the measured backend policy.
#[doc(hidden)]
pub fn decode_baseline_code_block_segments_with_adaptive_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_baseline_code_block_segments_with_adaptive_scratch_outcome(
        segment,
        coding_segments,
        spec,
        coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode reversible terminated segments through adaptive dispatch and report
/// the exact backend that executed.
#[doc(hidden)]
pub fn decode_baseline_code_block_segments_with_adaptive_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    if recommended_decode_backend(segment.len(), spec) == Tier1DecodeBackend::PackedDense {
        decode_baseline_code_block_segments_with_packed_scratch_outcome(
            segment,
            coding_segments,
            spec,
            coefficients,
            scratch,
        )
    } else {
        let decoded = decode_baseline_code_block_segments_with_scratch(
            segment,
            coding_segments,
            spec,
            coefficients,
            scratch,
        )?;
        decode_outcome(decoded, Tier1DecodeBackend::Checked, coding_segments, spec)
    }
}

/// Decode one irreversible block through adaptive dispatch while retaining
/// doubled one-plus-half coefficients.
#[doc(hidden)]
pub fn decode_irreversible_code_block_with_adaptive_scratch(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_irreversible_code_block_with_adaptive_scratch_outcome(
        segment,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode one irreversible block through adaptive dispatch and report the
/// exact backend that executed.
#[doc(hidden)]
pub fn decode_irreversible_code_block_with_adaptive_scratch_outcome(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_irreversible_code_block_segments_with_adaptive_scratch_outcome(
        segment,
        &coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
}

/// Decode irreversible terminated segments through adaptive dispatch.
#[doc(hidden)]
pub fn decode_irreversible_code_block_segments_with_adaptive_scratch(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecode> {
    decode_irreversible_code_block_segments_with_adaptive_scratch_outcome(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
    )
    .map(|outcome| outcome.decoded)
}

/// Decode irreversible terminated segments through adaptive dispatch and
/// report the exact backend that executed.
#[doc(hidden)]
pub fn decode_irreversible_code_block_segments_with_adaptive_scratch_outcome(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
) -> Result<CodeBlockDecodeOutcome> {
    if recommended_irreversible_decode_backend(segment.len(), spec)
        == Tier1DecodeBackend::PackedDense
    {
        decode_irreversible_code_block_segments_with_packed_scratch_outcome(
            segment,
            coding_segments,
            spec,
            doubled_half_step_coefficients,
            scratch,
        )
    } else {
        let decoded = decode_irreversible_code_block_segments_with_scratch(
            segment,
            coding_segments,
            spec,
            doubled_half_step_coefficients,
            scratch,
        )?;
        decode_outcome(decoded, Tier1DecodeBackend::Checked, coding_segments, spec)
    }
}

/// Decode one classic Part 1 code-block with reusable scratch and opt-in
/// fine-grained tier-1 timing.
#[cfg(feature = "std")]
pub fn decode_baseline_code_block_with_scratch_profiled(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_baseline_code_block_segments_with_scratch_profiled(
        segment,
        &coding_segments,
        spec,
        coefficients,
        scratch,
        timings,
    )
}

/// Decode explicitly terminated segments with reusable scratch and timings.
#[cfg(feature = "std")]
pub fn decode_baseline_code_block_segments_with_scratch_profiled(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
) -> Result<CodeBlockDecode> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    if spec.coding_passes == 0 {
        coefficients[..coefficient_count].fill(0);
        return Ok(CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        });
    }

    decode_normal_bitplanes_with_scratch_profiled_segments(
        segment,
        coding_segments,
        spec,
        &mut coefficients[..coefficient_count],
        scratch,
        timings,
        None,
        CoefficientReconstruction::ReversibleInteger,
    )?;

    Ok(CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    })
}

/// Decode one classic Part 1 code-block with reusable scratch, fine timings,
/// and opt-in work counters.
#[cfg(feature = "std")]
pub fn decode_baseline_code_block_with_scratch_profiled_and_counters(
    segment: &[u8],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<CodeBlockDecode> {
    let coding_segments = [CodeBlockSegment {
        byte_len: segment.len(),
        coding_passes: spec.coding_passes,
    }];
    decode_baseline_code_block_segments_with_scratch_profiled_and_counters(
        segment,
        &coding_segments,
        spec,
        coefficients,
        scratch,
        timings,
        counters,
    )
}

/// Decode explicitly terminated segments with timings and work counters.
#[cfg(feature = "std")]
pub fn decode_baseline_code_block_segments_with_scratch_profiled_and_counters(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<CodeBlockDecode> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    if spec.coding_passes == 0 {
        coefficients[..coefficient_count].fill(0);
        return Ok(CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        });
    }

    decode_normal_bitplanes_with_scratch_profiled_segments(
        segment,
        coding_segments,
        spec,
        &mut coefficients[..coefficient_count],
        scratch,
        timings,
        Some(counters),
        CoefficientReconstruction::ReversibleInteger,
    )?;

    Ok(CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    })
}

/// Decode explicitly terminated irreversible segments with reusable scratch
/// and opt-in fine-grained tier-1 timing.
#[cfg(feature = "std")]
pub fn decode_irreversible_code_block_segments_with_scratch_profiled(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
) -> Result<CodeBlockDecode> {
    decode_irreversible_code_block_segments_with_scratch_profiled_impl(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
        timings,
        None,
    )
}

/// Decode explicitly terminated irreversible segments with timings and
/// opt-in work counters.
#[cfg(feature = "std")]
pub fn decode_irreversible_code_block_segments_with_scratch_profiled_and_counters(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<CodeBlockDecode> {
    decode_irreversible_code_block_segments_with_scratch_profiled_impl(
        segment,
        coding_segments,
        spec,
        doubled_half_step_coefficients,
        scratch,
        timings,
        Some(counters),
    )
}

#[cfg(feature = "std")]
fn decode_irreversible_code_block_segments_with_scratch_profiled_impl(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    doubled_half_step_coefficients: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    counters: Option<&mut CodeBlockDecodeWorkCounters>,
) -> Result<CodeBlockDecode> {
    spec.validate()?;
    validate_code_block_segments(segment, coding_segments, spec)?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if doubled_half_step_coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: doubled_half_step_coefficients.len(),
        });
    }
    if spec.coding_passes == 0 {
        doubled_half_step_coefficients[..coefficient_count].fill(0);
        return Ok(CodeBlockDecode {
            bytes_consumed: 0,
            passes_decoded: 0,
            coefficients: coefficient_count,
        });
    }

    decode_normal_bitplanes_with_scratch_profiled_segments(
        segment,
        coding_segments,
        spec,
        &mut doubled_half_step_coefficients[..coefficient_count],
        scratch,
        timings,
        counters,
        CoefficientReconstruction::IrreversibleDoubledHalfStep,
    )?;
    Ok(CodeBlockDecode {
        bytes_consumed: segment.len(),
        passes_decoded: spec.coding_passes,
        coefficients: coefficient_count,
    })
}

/// Return the JPEG 2000 coding-pass family for a zero-based pass index.
pub fn coding_pass_for_index(index: u16) -> CodingPass {
    match index % 3 {
        0 => CodingPass::Cleanup,
        1 => CodingPass::SignificancePropagation,
        _ => CodingPass::MagnitudeRefinement,
    }
}

/// Encode one classic Part 1 code-block for the accepted baseline subset.
pub fn encode_baseline_code_block(
    coefficients: &[i32],
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
) -> Result<CodeBlockEncode> {
    let mut scratch = CodeBlockEncodeScratch::new();
    encode_baseline_code_block_with_scratch(coefficients, spec, output, &mut scratch)
}

/// Encode one classic Part 1 code-block using caller-provided reusable scratch.
pub fn encode_baseline_code_block_with_scratch(
    coefficients: &[i32],
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    scratch: &mut CodeBlockEncodeScratch,
) -> Result<CodeBlockEncode> {
    spec.validate()?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    let coefficients = &coefficients[..coefficient_count];
    let max_magnitude = coefficients
        .iter()
        .map(|coefficient| coefficient.unsigned_abs())
        .max()
        .unwrap_or(0);
    if max_magnitude == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }

    let width = usize::from(spec.dimensions.width());
    let height = usize::from(spec.dimensions.height());
    let mut ctx = scratch.prepare(width, height, spec.subband, coefficients);
    encode_prepared_baseline_code_block(&mut ctx, max_magnitude, spec, output, None)
}

/// Encode one code-block and return each independently terminated byte length.
///
/// Style-zero output reports one segment. BYPASS output reports each MQ/raw
/// coding group, and TERMALL output reports one segment for every coding pass.
pub fn encode_baseline_code_block_segments_with_scratch(
    coefficients: &[i32],
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    segment_byte_lengths: &mut Vec<usize>,
    scratch: &mut CodeBlockEncodeScratch,
) -> Result<CodeBlockEncode> {
    segment_byte_lengths.clear();
    spec.validate()?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    let coefficients = &coefficients[..coefficient_count];
    let max_magnitude = coefficients
        .iter()
        .map(|coefficient| coefficient.unsigned_abs())
        .max()
        .unwrap_or(0);
    if max_magnitude == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }

    let width = usize::from(spec.dimensions.width());
    let height = usize::from(spec.dimensions.height());
    let mut ctx = scratch.prepare(width, height, spec.subband, coefficients);
    encode_prepared_baseline_code_block(
        &mut ctx,
        max_magnitude,
        spec,
        output,
        Some(segment_byte_lengths),
    )
}

/// Encode one classic Part 1 code-block using a caller-provided maximum
/// coefficient magnitude.
///
/// This is intended for callers that already scanned or gathered the
/// code-block coefficients. The supplied maximum must match the coefficient
/// view; use [`encode_baseline_code_block_with_scratch`] when that is not
/// already known.
pub fn encode_baseline_code_block_with_known_max_scratch(
    coefficients: &[i32],
    max_magnitude: u32,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    scratch: &mut CodeBlockEncodeScratch,
) -> Result<CodeBlockEncode> {
    spec.validate()?;

    let coefficient_count = spec.dimensions.coefficient_count();
    if coefficients.len() < coefficient_count {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: coefficient_count,
            actual: coefficients.len(),
        });
    }

    if max_magnitude == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }

    let width = usize::from(spec.dimensions.width());
    let height = usize::from(spec.dimensions.height());
    let coefficients = &coefficients[..coefficient_count];
    debug_assert_eq!(
        max_magnitude,
        coefficients
            .iter()
            .map(|coefficient| coefficient.unsigned_abs())
            .max()
            .unwrap_or(0)
    );
    let mut ctx = scratch.prepare(width, height, spec.subband, coefficients);
    encode_prepared_baseline_code_block(&mut ctx, max_magnitude, spec, output, None)
}

/// Encode one classic Part 1 code-block from a row-strided coefficient view.
pub fn encode_baseline_code_block_with_strided_scratch(
    coefficients: &[i32],
    row_stride: usize,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    scratch: &mut CodeBlockEncodeScratch,
) -> Result<CodeBlockEncode> {
    spec.validate()?;

    let width = usize::from(spec.dimensions.width());
    let height = usize::from(spec.dimensions.height());
    if row_stride < width {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: width,
            actual: row_stride,
        });
    }
    let required_len = (height - 1)
        .checked_mul(row_stride)
        .and_then(|offset| offset.checked_add(width))
        .ok_or(Tier1Error::MalformedBitstream {
            reason: "code-block coefficient view size overflows usize",
        })?;
    if coefficients.len() < required_len {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: required_len,
            actual: coefficients.len(),
        });
    }

    let (mut ctx, max_magnitude) =
        scratch.prepare_strided_with_max(width, height, spec.subband, coefficients, row_stride);
    if max_magnitude == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }

    encode_prepared_baseline_code_block(&mut ctx, max_magnitude, spec, output, None)
}

/// Encode a strided fixture block and retain stable coding-pass boundaries.
///
/// Production encoders deliberately use the ordinary entry point. This typed
/// fixture hook observes the real MQ stream without inserting terminations or
/// inventing byte offsets.
#[cfg(feature = "test-fixtures")]
#[doc(hidden)]
pub fn encode_baseline_code_block_with_strided_scratch_and_pass_boundaries(
    coefficients: &[i32],
    row_stride: usize,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    pass_boundaries: &mut Vec<CodeBlockPassBoundary>,
    scratch: &mut CodeBlockEncodeScratch,
) -> Result<CodeBlockEncode> {
    pass_boundaries.clear();
    spec.validate()?;

    let width = usize::from(spec.dimensions.width());
    let height = usize::from(spec.dimensions.height());
    if row_stride < width {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: width,
            actual: row_stride,
        });
    }
    let required_len = (height - 1)
        .checked_mul(row_stride)
        .and_then(|offset| offset.checked_add(width))
        .ok_or(Tier1Error::MalformedBitstream {
            reason: "code-block coefficient view size overflows usize",
        })?;
    if coefficients.len() < required_len {
        return Err(Tier1Error::CoefficientBufferTooSmall {
            required: required_len,
            actual: coefficients.len(),
        });
    }

    let (mut ctx, max_magnitude) =
        scratch.prepare_strided_with_max(width, height, spec.subband, coefficients, row_stride);
    if max_magnitude == 0 {
        return Ok(CodeBlockEncode {
            pass_count: 0,
            included: false,
            missing_bitplanes: spec.available_bitplanes,
            byte_len: 0,
        });
    }

    encode_prepared_baseline_code_block_with_pass_boundaries(
        &mut ctx,
        max_magnitude,
        spec,
        output,
        pass_boundaries,
    )
}

#[cfg(feature = "test-fixtures")]
fn encode_prepared_baseline_code_block_with_pass_boundaries(
    ctx: &mut BitPlaneEncodeContext<'_>,
    max_magnitude: u32,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    pass_boundaries: &mut Vec<CodeBlockPassBoundary>,
) -> Result<CodeBlockEncode> {
    let significant_bitplanes = u8::try_from(32 - max_magnitude.leading_zeros()).map_err(|_| {
        Tier1Error::MalformedBitstream {
            reason: "coefficient magnitude bit-plane count does not fit u8",
        }
    })?;
    if significant_bitplanes > spec.available_bitplanes {
        return Err(Tier1Error::MalformedBitstream {
            reason: "coefficient magnitude exceeds available bit-planes",
        });
    }
    let missing_bitplanes = spec.available_bitplanes - significant_bitplanes;
    let pass_count = 1 + 3 * u16::from(significant_bitplanes - 1);
    let style = CodeBlockStyle::from_bits(spec.code_block_style);
    let start_len = output.len();
    let byte_len = {
        let mut encoder = ArithmeticEncoder::new(output);
        encode_coding_passes_with_boundaries(
            ctx,
            &mut encoder,
            style,
            significant_bitplanes,
            pass_count,
            pass_boundaries,
        )?;
        encoder.len() - start_len
    };
    Ok(CodeBlockEncode {
        pass_count,
        included: true,
        missing_bitplanes,
        byte_len,
    })
}

#[cfg(feature = "test-fixtures")]
fn encode_coding_passes_with_boundaries(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder<'_>,
    style: CodeBlockStyle,
    coded_bitplanes: u8,
    pass_count: u16,
    pass_boundaries: &mut Vec<CodeBlockPassBoundary>,
) -> Result<()> {
    if style.bits() != 0 {
        return Err(Tier1Error::UnsupportedCodingStyle {
            style: style.bits(),
            unsupported_bits: style.bits(),
        });
    }
    for coding_pass in 0..pass_count {
        let current_bitplane = coding_pass.div_ceil(3);
        ctx.current_bit_position = coded_bitplanes
            .checked_sub(1)
            .and_then(|value| value.checked_sub(current_bitplane as u8))
            .ok_or(Tier1Error::MalformedBitstream {
                reason: "coding pass exceeds available bit-planes",
            })?;
        match coding_pass_for_index(coding_pass) {
            CodingPass::Cleanup => {
                cleanup_pass_encode::<false>(ctx, encoder)?;
                ctx.reset_for_next_bitplane();
            }
            CodingPass::SignificancePropagation => {
                significance_propagation_pass_encode::<false>(ctx, encoder);
            }
            CodingPass::MagnitudeRefinement => {
                magnitude_refinement_pass_encode::<false>(ctx, encoder);
            }
        }
        pass_boundaries.push(CodeBlockPassBoundary {
            coding_passes: coding_pass + 1,
            stable_byte_len: encoder.current_segment_len(),
        });
    }
    encoder.finish();
    if let Some(final_boundary) = pass_boundaries.last_mut() {
        final_boundary.stable_byte_len = encoder.current_segment_len();
    }
    Ok(())
}

fn encode_prepared_baseline_code_block(
    ctx: &mut BitPlaneEncodeContext<'_>,
    max_magnitude: u32,
    spec: CodeBlockEncodeSpec,
    output: &mut Vec<u8>,
    mut segment_byte_lengths: Option<&mut Vec<usize>>,
) -> Result<CodeBlockEncode> {
    let significant_bitplanes = u8::try_from(32 - max_magnitude.leading_zeros()).map_err(|_| {
        Tier1Error::MalformedBitstream {
            reason: "coefficient magnitude bit-plane count does not fit u8",
        }
    })?;
    if significant_bitplanes > spec.available_bitplanes {
        return Err(Tier1Error::MalformedBitstream {
            reason: "coefficient magnitude exceeds available bit-planes",
        });
    }
    let coded_bitplanes = significant_bitplanes;
    let missing_bitplanes = spec.available_bitplanes - significant_bitplanes;
    let pass_count = 1 + 3 * u16::from(coded_bitplanes - 1);
    let style = CodeBlockStyle::from_bits(spec.code_block_style);

    let start_len = output.len();
    let byte_len = {
        let mut encoder = ArithmeticEncoder::new(output);
        match (style.resets_contexts(), style.is_vertically_causal()) {
            (false, false) => encode_coding_passes::<false, false>(
                ctx,
                &mut encoder,
                style,
                coded_bitplanes,
                pass_count,
                segment_byte_lengths.as_deref_mut(),
            )?,
            (true, false) => encode_coding_passes::<true, false>(
                ctx,
                &mut encoder,
                style,
                coded_bitplanes,
                pass_count,
                segment_byte_lengths.as_deref_mut(),
            )?,
            (false, true) => encode_coding_passes::<false, true>(
                ctx,
                &mut encoder,
                style,
                coded_bitplanes,
                pass_count,
                segment_byte_lengths.as_deref_mut(),
            )?,
            (true, true) => encode_coding_passes::<true, true>(
                ctx,
                &mut encoder,
                style,
                coded_bitplanes,
                pass_count,
                segment_byte_lengths,
            )?,
        }
        encoder.len() - start_len
    };

    Ok(CodeBlockEncode {
        pass_count,
        included: true,
        missing_bitplanes,
        byte_len,
    })
}

fn encode_coding_passes<const RESET_CONTEXTS: bool, const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder<'_>,
    style: CodeBlockStyle,
    coded_bitplanes: u8,
    pass_count: u16,
    mut segment_byte_lengths: Option<&mut Vec<usize>>,
) -> Result<()> {
    for coding_pass in 0..pass_count {
        let current_bitplane = coding_pass.div_ceil(3);
        ctx.current_bit_position = coded_bitplanes
            .checked_sub(1)
            .and_then(|value| value.checked_sub(current_bitplane as u8))
            .ok_or(Tier1Error::MalformedBitstream {
                reason: "coding pass exceeds available bit-planes",
            })?;

        let raw_pass = is_raw_coding_pass(style, coding_pass);
        match (raw_pass, coding_pass_for_index(coding_pass)) {
            (true, CodingPass::SignificancePropagation) => {
                significance_propagation_pass_encode_raw::<VERTICAL_CAUSAL>(ctx, encoder);
            }
            (true, CodingPass::MagnitudeRefinement) => {
                magnitude_refinement_pass_encode_raw(ctx, encoder);
            }
            (true, CodingPass::Cleanup) => {
                return Err(Tier1Error::MalformedBitstream {
                    reason: "BYPASS raw segment contains a cleanup pass",
                });
            }
            (false, CodingPass::Cleanup) => {
                cleanup_pass_encode::<VERTICAL_CAUSAL>(ctx, encoder)?;
                if style.has_segmentation_symbols() {
                    encode_segmentation_symbol(encoder);
                }
                ctx.reset_for_next_bitplane();
            }
            (false, CodingPass::SignificancePropagation) => {
                significance_propagation_pass_encode::<VERTICAL_CAUSAL>(ctx, encoder);
            }
            (false, CodingPass::MagnitudeRefinement) => {
                magnitude_refinement_pass_encode::<VERTICAL_CAUSAL>(ctx, encoder);
            }
        }
        if RESET_CONTEXTS {
            encoder.reset_contexts();
        }
        if coding_segment_ends_after(style, coding_pass, pass_count) {
            if raw_pass {
                encoder.finish_raw(style.has_predictable_termination());
            } else {
                terminate_arithmetic_segment(encoder, style);
            }
            if let Some(segment_byte_lengths) = segment_byte_lengths.as_deref_mut() {
                segment_byte_lengths.push(encoder.current_segment_len());
            }
            if coding_pass + 1 < pass_count {
                if is_raw_coding_pass(style, coding_pass + 1) {
                    encoder.restart_raw_segment();
                } else {
                    encoder.restart_segment();
                }
            }
        }
    }
    Ok(())
}

fn coding_segment_ends_after(style: CodeBlockStyle, coding_pass: u16, pass_count: u16) -> bool {
    if coding_pass + 1 == pass_count || style.terminates_each_pass() {
        return true;
    }
    style.uses_selective_arithmetic_bypass()
        && (coding_pass == 9
            || (coding_pass >= 10
                && matches!(
                    coding_pass_for_index(coding_pass),
                    CodingPass::Cleanup | CodingPass::MagnitudeRefinement
                )))
}

fn terminate_arithmetic_segment(encoder: &mut ArithmeticEncoder<'_>, style: CodeBlockStyle) {
    if style.has_predictable_termination() {
        encoder.finish_predictable();
    } else {
        encoder.finish();
    }
}

/// Minimal MQ byte input helper with JPEG 2000 byte-stuffing checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MqByteInput<'a> {
    bytes: &'a [u8],
    offset: usize,
    previous_was_ff: bool,
}

const CONTEXT_COUNT: usize = 19;
const COEFFICIENTS_PADDING: usize = 1;
const SEGMENTATION_SYMBOL: u32 = 0x0a;
const MAX_RECONSTRUCTED_MAGNITUDE_BITPLANES: u8 = 30;

fn maximum_coding_passes(bitplanes: u8) -> u16 {
    if bitplanes == 0 {
        0
    } else {
        1 + 3 * u16::from(bitplanes - 1)
    }
}

fn validate_bitplane_pass_count(spec: CodeBlockDecodeSpec) -> Result<u8> {
    let available_bitplanes = spec.available_bitplanes;
    let bitplanes = available_bitplanes
        .checked_sub(spec.missing_most_significant_bitplanes)
        .ok_or(Tier1Error::MalformedBitstream {
            reason: "missing most-significant bit-planes exceed the available magnitude planes",
        })?;
    let max_passes = maximum_coding_passes(bitplanes);
    if spec.coding_passes > max_passes {
        return Err(Tier1Error::UnsupportedCodingPass {
            pass: coding_pass_for_index(max_passes),
            reason: "packet requests more coding passes than the baseline bit-plane count permits",
        });
    }
    if spec.coding_passes != 0 && bitplanes > MAX_RECONSTRUCTED_MAGNITUDE_BITPLANES {
        return Err(Tier1Error::UnsupportedCodingPass {
            pass: coding_pass_for_index(0),
            reason: "classic coefficient storage supports at most 30 reconstructed magnitude bit-planes",
        });
    }
    Ok(bitplanes)
}

fn validate_code_block_segments(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
) -> Result<()> {
    if spec.coding_passes == 0 {
        if !segment.is_empty()
            || coding_segments
                .iter()
                .any(|entry| entry.byte_len != 0 || entry.coding_passes != 0)
        {
            return Err(Tier1Error::MalformedBitstream {
                reason: "zero-pass code-block has nonempty coding segments",
            });
        }
        return Ok(());
    }
    if coding_segments.is_empty() {
        return Err(Tier1Error::MalformedBitstream {
            reason: "nonzero coding-pass count has no code-block segments",
        });
    }
    if segment.is_empty() {
        return Err(Tier1Error::MalformedBitstream {
            reason: "nonzero coding-pass count has an empty code-block segment",
        });
    }

    let mut total_bytes = 0usize;
    let mut total_passes = 0u16;
    for entry in coding_segments {
        if entry.coding_passes == 0 {
            return Err(Tier1Error::MalformedBitstream {
                reason: "code-block segment has zero coding passes",
            });
        }
        total_bytes =
            total_bytes
                .checked_add(entry.byte_len)
                .ok_or(Tier1Error::MalformedBitstream {
                    reason: "code-block segment byte count overflows usize",
                })?;
        total_passes = total_passes.checked_add(entry.coding_passes).ok_or(
            Tier1Error::MalformedBitstream {
                reason: "code-block segment pass count overflows u16",
            },
        )?;
    }
    if total_bytes != segment.len() {
        return Err(Tier1Error::MalformedBitstream {
            reason: "code-block segment lengths do not consume the input bytes",
        });
    }
    if total_passes != spec.coding_passes {
        return Err(Tier1Error::MalformedBitstream {
            reason: "code-block segment pass counts do not match the decode specification",
        });
    }
    if spec.style.terminates_each_pass() {
        if coding_segments.iter().any(|entry| entry.coding_passes != 1) {
            return Err(Tier1Error::MalformedBitstream {
                reason: "TERMALL requires one arithmetic segment per coding pass",
            });
        }
    } else if spec.style.uses_selective_arithmetic_bypass() {
        let mut coding_pass = 0u16;
        for (index, entry) in coding_segments.iter().enumerate() {
            let capacity = bypass_segment_pass_capacity(coding_pass);
            if entry.coding_passes > capacity
                || (entry.coding_passes < capacity && index + 1 != coding_segments.len())
            {
                return Err(Tier1Error::MalformedBitstream {
                    reason: "BYPASS coding segment crosses a coder restart boundary",
                });
            }
            coding_pass = coding_pass.checked_add(entry.coding_passes).ok_or(
                Tier1Error::MalformedBitstream {
                    reason: "BYPASS coding segment pass count overflows u16",
                },
            )?;
        }
    } else if coding_segments.len() != 1 {
        return Err(Tier1Error::MalformedBitstream {
            reason: "multiple arithmetic segments require a terminating code-block style",
        });
    }
    Ok(())
}

fn bypass_segment_pass_capacity(coding_pass: u16) -> u16 {
    if coding_pass < 10 {
        return 10 - coding_pass;
    }
    match (coding_pass - 10) % 3 {
        0 => 2,
        1 | 2 => 1,
        _ => unreachable!(),
    }
}

fn is_raw_coding_pass(style: CodeBlockStyle, coding_pass: u16) -> bool {
    style.uses_selective_arithmetic_bypass() && coding_pass >= 10 && !coding_pass.is_multiple_of(3)
}

/// Attribute validated terminated-segment bytes to MQ or raw BYPASS coding.
#[doc(hidden)]
pub fn code_block_segment_byte_classes(
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
) -> Result<(usize, usize)> {
    let mut coding_pass = 0_u16;
    let mut mq_bytes = 0_usize;
    let mut raw_bypass_bytes = 0_usize;
    for coding_segment in coding_segments {
        let destination = if is_raw_coding_pass(spec.style, coding_pass) {
            &mut raw_bypass_bytes
        } else {
            &mut mq_bytes
        };
        *destination = destination.checked_add(coding_segment.byte_len).ok_or(
            Tier1Error::MalformedBitstream {
                reason: "code-block segment byte attribution overflowed",
            },
        )?;
        coding_pass = coding_pass
            .checked_add(coding_segment.coding_passes)
            .ok_or(Tier1Error::MalformedBitstream {
                reason: "code-block segment pass attribution overflowed",
            })?;
    }
    if coding_pass != spec.coding_passes {
        return Err(Tier1Error::MalformedBitstream {
            reason: "code-block segment pass attribution did not cover declared passes",
        });
    }
    Ok((mq_bytes, raw_bypass_bytes))
}

fn decode_outcome(
    decoded: CodeBlockDecode,
    backend: Tier1DecodeBackend,
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
) -> Result<CodeBlockDecodeOutcome> {
    let (mq_bytes, raw_bypass_bytes) = code_block_segment_byte_classes(coding_segments, spec)?;
    Ok(CodeBlockDecodeOutcome {
        decoded,
        backend,
        mq_bytes,
        raw_bypass_bytes,
    })
}

/// Attach authoritative checked-backend attribution to a decode performed by
/// one of the explicitly checked/profiled entry points.
#[doc(hidden)]
pub fn checked_decode_outcome(
    decoded: CodeBlockDecode,
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
) -> Result<CodeBlockDecodeOutcome> {
    decode_outcome(decoded, Tier1DecodeBackend::Checked, coding_segments, spec)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoefficientReconstruction {
    ReversibleInteger,
    IrreversibleDoubledHalfStep,
}

fn decode_normal_bitplanes_with_scratch_segments(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let bitplanes = validate_bitplane_pass_count(spec)?;
    match (
        spec.style.resets_contexts(),
        spec.style.is_vertically_causal(),
    ) {
        (false, false) => decode_normal_bitplanes_with_scratch_policy::<false, false>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (true, false) => decode_normal_bitplanes_with_scratch_policy::<true, false>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (false, true) => decode_normal_bitplanes_with_scratch_policy::<false, true>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            reconstruction,
        ),
        (true, true) => decode_normal_bitplanes_with_scratch_policy::<true, true>(
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

fn decode_normal_bitplanes_with_scratch_policy<
    const RESET_CONTEXTS: bool,
    const VERTICAL_CAUSAL: bool,
>(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    bitplanes: u8,
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let mut ctx = scratch.prepare(
        usize::from(spec.dimensions.width()),
        usize::from(spec.dimensions.height()),
        spec.subband,
    );
    let mut byte_offset = 0usize;
    let mut coding_pass = 0u16;
    for coding_segment in coding_segments {
        let segment_end = byte_offset + coding_segment.byte_len;
        if is_raw_coding_pass(spec.style, coding_pass) {
            let mut decoder = RawDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_decode_bit_position(&mut ctx, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::SignificancePropagation => {
                        significance_propagation_pass_raw::<VERTICAL_CAUSAL>(
                            &mut ctx,
                            &mut decoder,
                        )?;
                    }
                    CodingPass::MagnitudeRefinement => {
                        magnitude_refinement_pass_raw(&mut ctx, &mut decoder)?;
                    }
                    CodingPass::Cleanup => {
                        return Err(Tier1Error::MalformedBitstream {
                            reason: "BYPASS raw segment contains a cleanup pass",
                        });
                    }
                }
                if RESET_CONTEXTS {
                    ctx.reset_contexts();
                }
                coding_pass += 1;
            }
        } else {
            let mut decoder = ArithmeticDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_decode_bit_position(&mut ctx, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::Cleanup => {
                        cleanup_pass::<VERTICAL_CAUSAL>(&mut ctx, &mut decoder)?;
                        if spec.style.has_segmentation_symbols() {
                            decode_segmentation_symbol(
                                &mut decoder,
                                ctx.arithmetic_decoder_context(18),
                            )?;
                        }
                        ctx.reset_for_next_bitplane();
                    }
                    CodingPass::SignificancePropagation => {
                        significance_propagation_pass::<VERTICAL_CAUSAL>(&mut ctx, &mut decoder)?;
                    }
                    CodingPass::MagnitudeRefinement => {
                        magnitude_refinement_pass::<VERTICAL_CAUSAL>(&mut ctx, &mut decoder)?;
                    }
                }
                if RESET_CONTEXTS {
                    ctx.reset_contexts();
                }
                coding_pass += 1;
            }
            if spec.style.has_predictable_termination() {
                decoder.validate_predictable_termination()?;
            }
        }
        byte_offset = segment_end;
    }

    ctx.copy_coefficients_to(output, reconstruction);

    Ok(())
}

fn set_decode_bit_position(
    ctx: &mut BitPlaneDecodeContext<'_>,
    bitplanes: u8,
    coding_pass: u16,
) -> Result<()> {
    let current_bitplane = coding_pass.div_ceil(3);
    ctx.current_bit_position = bitplanes
        .checked_sub(1)
        .and_then(|value| value.checked_sub(current_bitplane as u8))
        .ok_or(Tier1Error::MalformedBitstream {
            reason: "coding pass exceeds available bit-planes",
        })?;
    Ok(())
}

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments)]
fn decode_normal_bitplanes_with_scratch_profiled_segments(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    counters: Option<&mut CodeBlockDecodeWorkCounters>,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let bitplanes = validate_bitplane_pass_count(spec)?;
    match (
        spec.style.resets_contexts(),
        spec.style.is_vertically_causal(),
    ) {
        (false, false) => decode_normal_bitplanes_with_scratch_profiled_policy::<false, false>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            timings,
            counters,
            reconstruction,
        ),
        (true, false) => decode_normal_bitplanes_with_scratch_profiled_policy::<true, false>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            timings,
            counters,
            reconstruction,
        ),
        (false, true) => decode_normal_bitplanes_with_scratch_profiled_policy::<false, true>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            timings,
            counters,
            reconstruction,
        ),
        (true, true) => decode_normal_bitplanes_with_scratch_profiled_policy::<true, true>(
            segment,
            coding_segments,
            bitplanes,
            spec,
            output,
            scratch,
            timings,
            counters,
            reconstruction,
        ),
    }
}

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments)]
fn decode_normal_bitplanes_with_scratch_profiled_policy<
    const RESET_CONTEXTS: bool,
    const VERTICAL_CAUSAL: bool,
>(
    segment: &[u8],
    coding_segments: &[CodeBlockSegment],
    bitplanes: u8,
    spec: CodeBlockDecodeSpec,
    output: &mut [i32],
    scratch: &mut CodeBlockDecodeScratch,
    timings: &mut CodeBlockDecodeTimings,
    mut counters: Option<&mut CodeBlockDecodeWorkCounters>,
    reconstruction: CoefficientReconstruction,
) -> Result<()> {
    let stage_started = std::time::Instant::now();
    let mut ctx = scratch.prepare(
        usize::from(spec.dimensions.width()),
        usize::from(spec.dimensions.height()),
        spec.subband,
    );
    timings.scratch_prepare_ns += stage_started.elapsed().as_nanos();

    let mut byte_offset = 0usize;
    let mut coding_pass = 0u16;
    for coding_segment in coding_segments {
        let segment_end = byte_offset + coding_segment.byte_len;
        if is_raw_coding_pass(spec.style, coding_pass) {
            let mut decoder = RawDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_decode_bit_position(&mut ctx, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::SignificancePropagation => {
                        let stage_started = std::time::Instant::now();
                        if let Some(counters) = counters.as_deref_mut() {
                            significance_propagation_pass_raw_profiled::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                                counters,
                            )?;
                        } else {
                            significance_propagation_pass_raw::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                            )?;
                        }
                        timings.significance_propagation_pass_ns +=
                            stage_started.elapsed().as_nanos();
                    }
                    CodingPass::MagnitudeRefinement => {
                        let stage_started = std::time::Instant::now();
                        if let Some(counters) = counters.as_deref_mut() {
                            magnitude_refinement_pass_raw_profiled(
                                &mut ctx,
                                &mut decoder,
                                counters,
                            )?;
                        } else {
                            magnitude_refinement_pass_raw(&mut ctx, &mut decoder)?;
                        }
                        timings.magnitude_refinement_pass_ns += stage_started.elapsed().as_nanos();
                    }
                    CodingPass::Cleanup => {
                        return Err(Tier1Error::MalformedBitstream {
                            reason: "BYPASS raw segment contains a cleanup pass",
                        });
                    }
                }
                if RESET_CONTEXTS {
                    ctx.reset_contexts();
                }
                coding_pass += 1;
            }
        } else {
            let mut decoder = ArithmeticDecoder::new(&segment[byte_offset..segment_end]);
            for _ in 0..coding_segment.coding_passes {
                set_decode_bit_position(&mut ctx, bitplanes, coding_pass)?;
                match coding_pass_for_index(coding_pass) {
                    CodingPass::Cleanup => {
                        let stage_started = std::time::Instant::now();
                        if let Some(counters) = counters.as_deref_mut() {
                            cleanup_pass_profiled::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                                counters,
                            )?;
                        } else {
                            cleanup_pass::<VERTICAL_CAUSAL>(&mut ctx, &mut decoder)?;
                        }
                        if spec.style.has_segmentation_symbols() {
                            if let Some(counters) = counters.as_deref_mut() {
                                decode_segmentation_symbol_profiled(
                                    &mut ctx,
                                    &mut decoder,
                                    counters,
                                )?;
                            } else {
                                decode_segmentation_symbol(
                                    &mut decoder,
                                    ctx.arithmetic_decoder_context(18),
                                )?;
                            }
                        }
                        timings.cleanup_pass_ns += stage_started.elapsed().as_nanos();
                        let stage_started = std::time::Instant::now();
                        ctx.reset_for_next_bitplane();
                        timings.bitplane_reset_ns += stage_started.elapsed().as_nanos();
                    }
                    CodingPass::SignificancePropagation => {
                        let stage_started = std::time::Instant::now();
                        if let Some(counters) = counters.as_deref_mut() {
                            significance_propagation_pass_profiled::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                                counters,
                            )?;
                        } else {
                            significance_propagation_pass::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                            )?;
                        }
                        timings.significance_propagation_pass_ns +=
                            stage_started.elapsed().as_nanos();
                    }
                    CodingPass::MagnitudeRefinement => {
                        let stage_started = std::time::Instant::now();
                        if let Some(counters) = counters.as_deref_mut() {
                            magnitude_refinement_pass_profiled::<VERTICAL_CAUSAL>(
                                &mut ctx,
                                &mut decoder,
                                counters,
                            )?;
                        } else {
                            magnitude_refinement_pass::<VERTICAL_CAUSAL>(&mut ctx, &mut decoder)?;
                        }
                        timings.magnitude_refinement_pass_ns += stage_started.elapsed().as_nanos();
                    }
                }
                if RESET_CONTEXTS {
                    ctx.reset_contexts();
                }
                coding_pass += 1;
            }
            if spec.style.has_predictable_termination() {
                decoder.validate_predictable_termination()?;
            }
        }
        byte_offset = segment_end;
    }

    let stage_started = std::time::Instant::now();
    ctx.copy_coefficients_to(output, reconstruction);
    timings.coefficient_copy_out_ns += stage_started.elapsed().as_nanos();

    Ok(())
}

#[cfg(feature = "std")]
#[derive(Clone, Copy)]
enum ProfiledPassFamily {
    Cleanup,
    SignificancePropagation,
    MagnitudeRefinement,
}

#[derive(Default, Copy, Clone)]
struct CoefficientState {
    significant: bool,
    coded_in_significance_pass: bool,
    magnitude_refined: bool,
}

impl CoefficientState {
    fn set_significant(&mut self) {
        self.significant = true;
    }

    fn mark_coded_in_significance_pass(&mut self) {
        self.coded_in_significance_pass = true;
    }

    fn clear_significance_pass_visit(&mut self) {
        self.coded_in_significance_pass = false;
    }

    fn set_magnitude_refined(&mut self) {
        self.magnitude_refined = true;
    }

    fn is_significant(self) -> bool {
        self.significant
    }

    fn was_magnitude_refined(self) -> bool {
        self.magnitude_refined
    }

    fn was_coded_in_significance_pass(self) -> bool {
        self.coded_in_significance_pass
    }
}

/// Significance of the eight neighbours named in Figure D.2.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct Neighborhood {
    top_left: bool,
    top: bool,
    top_right: bool,
    left: bool,
    right: bool,
    bottom_left: bool,
    bottom: bool,
    bottom_right: bool,
}

impl Neighborhood {
    pub(crate) fn from_mask(mask: u8) -> Self {
        Self {
            top_left: mask & (1 << 7) != 0,
            top: mask & (1 << 6) != 0,
            top_right: mask & (1 << 5) != 0,
            left: mask & (1 << 4) != 0,
            bottom_left: mask & (1 << 3) != 0,
            right: mask & (1 << 2) != 0,
            bottom_right: mask & (1 << 1) != 0,
            bottom: mask & 1 != 0,
        }
    }
    pub(crate) fn any(self) -> bool {
        self.horizontal_count() + self.vertical_count() + self.diagonal_count() != 0
    }

    fn horizontal_count(self) -> u8 {
        u8::from(self.left) + u8::from(self.right)
    }

    fn vertical_count(self) -> u8 {
        u8::from(self.top) + u8::from(self.bottom)
    }

    fn diagonal_count(self) -> u8 {
        u8::from(self.top_left)
            + u8::from(self.top_right)
            + u8::from(self.bottom_left)
            + u8::from(self.bottom_right)
    }
}

fn neighborhood_at<const VERTICAL_CAUSAL: bool>(
    states: &[CoefficientState],
    index: usize,
    padded_width: usize,
) -> Neighborhood {
    let row = index / padded_width - COEFFICIENTS_PADDING;
    let ignore_next_stripe = VERTICAL_CAUSAL && row % 4 == 3;
    Neighborhood {
        top_left: states[index - padded_width - 1].is_significant(),
        top: states[index - padded_width].is_significant(),
        top_right: states[index - padded_width + 1].is_significant(),
        left: states[index - 1].is_significant(),
        right: states[index + 1].is_significant(),
        bottom_left: !ignore_next_stripe && states[index + padded_width - 1].is_significant(),
        bottom: !ignore_next_stripe && states[index + padded_width].is_significant(),
        bottom_right: !ignore_next_stripe && states[index + padded_width + 1].is_significant(),
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Coefficient(i32);

impl Coefficient {
    fn get(self) -> i32 {
        self.0 / 2
    }

    fn doubled_half_step(self) -> i32 {
        self.0
    }

    fn sign(self) -> u32 {
        u32::from(self.0 < 0)
    }

    fn sign_bit(self) -> u8 {
        u8::from(self.0 < 0)
    }

    fn set_new_significant(&mut self, sign: u8, position: u8) {
        let one = 1_i32 << (u32::from(position) + 1);
        let half = one >> 1;
        let value = one | half;
        self.0 = if sign == 1 { -value } else { value };
    }

    fn refine(&mut self, bit: u32, position: u8) {
        let pos_half = 1_i32 << u32::from(position);
        if (bit ^ self.sign()) == 1 {
            self.0 += pos_half;
        } else {
            self.0 -= pos_half;
        }
    }
}

struct BitPlaneDecodeContext<'a> {
    coefficient_states: &'a mut [CoefficientState],
    coefficients: &'a mut [Coefficient],
    width: usize,
    padded_width: usize,
    height: usize,
    subband: Subband,
    contexts: [ArithmeticDecoderContext; CONTEXT_COUNT],
    current_bit_position: u8,
}

/// Reusable scratch buffers for classic tier-1 code-block encode.
///
/// The buffers are opaque so callers can reuse allocation capacity without
/// depending on coefficient-state internals.
#[derive(Default)]
pub struct CodeBlockEncodeScratch {
    coefficient_states: Vec<CoefficientState>,
    signs: Vec<u8>,
    magnitudes: Vec<u32>,
}

impl CodeBlockEncodeScratch {
    /// Create an empty scratch object. Capacity grows to the largest encoded
    /// code-block and is reused by later calls.
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop current scratch lengths while retaining allocated capacity.
    pub fn clear(&mut self) {
        self.coefficient_states.clear();
        self.signs.clear();
        self.magnitudes.clear();
    }

    fn prepare<'a>(
        &'a mut self,
        width: usize,
        height: usize,
        subband: Subband,
        coefficients: &[i32],
    ) -> BitPlaneEncodeContext<'a> {
        let padded_width = width + COEFFICIENTS_PADDING * 2;
        let padded_height = height + COEFFICIENTS_PADDING * 2;
        let len = padded_width * padded_height;
        self.coefficient_states
            .resize(len, CoefficientState::default());
        self.signs.resize(len, 0);
        self.magnitudes.resize(len, 0);
        self.coefficient_states[..len].fill(CoefficientState::default());

        for y in 0..height {
            let src_start = y * width;
            let src = &coefficients[src_start..src_start + width];
            let dst_start = (y + COEFFICIENTS_PADDING) * padded_width + COEFFICIENTS_PADDING;
            let signs = &mut self.signs[dst_start..dst_start + width];
            let magnitudes = &mut self.magnitudes[dst_start..dst_start + width];
            for ((sign, magnitude), coefficient) in signs.iter_mut().zip(magnitudes).zip(src) {
                *sign = u8::from(*coefficient < 0);
                *magnitude = coefficient.unsigned_abs();
            }
        }

        BitPlaneEncodeContext {
            coefficient_states: &mut self.coefficient_states[..len],
            signs: &mut self.signs[..len],
            magnitudes: &mut self.magnitudes[..len],
            width,
            padded_width,
            height,
            subband,
            current_bit_position: 0,
        }
    }

    fn prepare_strided_with_max<'a>(
        &'a mut self,
        width: usize,
        height: usize,
        subband: Subband,
        coefficients: &[i32],
        row_stride: usize,
    ) -> (BitPlaneEncodeContext<'a>, u32) {
        let padded_width = width + COEFFICIENTS_PADDING * 2;
        let padded_height = height + COEFFICIENTS_PADDING * 2;
        let len = padded_width * padded_height;
        self.coefficient_states
            .resize(len, CoefficientState::default());
        self.signs.resize(len, 0);
        self.magnitudes.resize(len, 0);
        self.coefficient_states[..len].fill(CoefficientState::default());

        let mut max_magnitude = 0_u32;
        for y in 0..height {
            let src_start = y * row_stride;
            let src = &coefficients[src_start..src_start + width];
            let dst_start = (y + COEFFICIENTS_PADDING) * padded_width + COEFFICIENTS_PADDING;
            let signs = &mut self.signs[dst_start..dst_start + width];
            let magnitudes = &mut self.magnitudes[dst_start..dst_start + width];
            for ((sign, dst_magnitude), coefficient) in signs.iter_mut().zip(magnitudes).zip(src) {
                let magnitude = coefficient.unsigned_abs();
                *sign = u8::from(*coefficient < 0);
                *dst_magnitude = magnitude;
                max_magnitude = max_magnitude.max(magnitude);
            }
        }

        (
            BitPlaneEncodeContext {
                coefficient_states: &mut self.coefficient_states[..len],
                signs: &mut self.signs[..len],
                magnitudes: &mut self.magnitudes[..len],
                width,
                padded_width,
                height,
                subband,
                current_bit_position: 0,
            },
            max_magnitude,
        )
    }
}

struct BitPlaneEncodeContext<'a> {
    coefficient_states: &'a mut [CoefficientState],
    signs: &'a mut [u8],
    magnitudes: &'a mut [u32],
    width: usize,
    padded_width: usize,
    height: usize,
    subband: Subband,
    current_bit_position: u8,
}

impl BitPlaneDecodeContext<'_> {
    fn copy_coefficients_to(&self, output: &mut [i32], reconstruction: CoefficientReconstruction) {
        for y in 0..self.height {
            let src_start = (y + COEFFICIENTS_PADDING) * self.padded_width + COEFFICIENTS_PADDING;
            let src = &self.coefficients[src_start..src_start + self.width];
            let dst_start = y * self.width;
            let dst = &mut output[dst_start..dst_start + self.width];
            for (dst, coeff) in dst.iter_mut().zip(src) {
                *dst = match reconstruction {
                    CoefficientReconstruction::ReversibleInteger => coeff.get(),
                    CoefficientReconstruction::IrreversibleDoubledHalfStep => {
                        coeff.doubled_half_step()
                    }
                };
            }
        }
    }

    fn reset_contexts(&mut self) {
        reset_arithmetic_contexts(&mut self.contexts);
    }

    fn reset_for_next_bitplane(&mut self) {
        for y in 0..self.height {
            let row_start = (y + COEFFICIENTS_PADDING) * self.padded_width + COEFFICIENTS_PADDING;
            for context in &mut self.coefficient_states[row_start..row_start + self.width] {
                context.clear_significance_pass_visit();
            }
        }
    }

    fn arithmetic_decoder_context(&mut self, ctx_label: u8) -> &mut ArithmeticDecoderContext {
        &mut self.contexts[usize::from(ctx_label)]
    }

    fn coefficient_index(&self, x: usize, y: usize) -> usize {
        (y + COEFFICIENTS_PADDING) * self.padded_width + x + COEFFICIENTS_PADDING
    }

    fn set_significant_at<const VERTICAL_CAUSAL: bool>(&mut self, index: usize) {
        debug_assert!(!self.coefficient_states[index].is_significant());
        self.coefficient_states[index].set_significant();
        let _ = VERTICAL_CAUSAL;
    }

    fn set_zero_coded_at(&mut self, index: usize) {
        self.coefficient_states[index].mark_coded_in_significance_pass();
    }

    fn set_new_significant_coefficient_at(&mut self, index: usize, sign: u8) {
        self.coefficients[index].set_new_significant(sign, self.current_bit_position);
    }
}

impl BitPlaneEncodeContext<'_> {
    fn reset_for_next_bitplane(&mut self) {
        for y in 0..self.height {
            let row_start = (y + COEFFICIENTS_PADDING) * self.padded_width + COEFFICIENTS_PADDING;
            for state in &mut self.coefficient_states[row_start..row_start + self.width] {
                state.clear_significance_pass_visit();
            }
        }
    }

    fn coefficient_index(&self, x: usize, y: usize) -> usize {
        (y + COEFFICIENTS_PADDING) * self.padded_width + x + COEFFICIENTS_PADDING
    }

    fn is_significant_at(&self, index: usize) -> bool {
        self.coefficient_states[index].is_significant()
    }

    fn set_significant_at<const VERTICAL_CAUSAL: bool>(&mut self, index: usize) {
        debug_assert!(!self.coefficient_states[index].is_significant());
        self.coefficient_states[index].set_significant();
        let _ = VERTICAL_CAUSAL;
    }

    fn set_zero_coded_at(&mut self, index: usize) {
        self.coefficient_states[index].mark_coded_in_significance_pass();
    }

    fn set_magnitude_refined_at(&mut self, index: usize) {
        self.coefficient_states[index].set_magnitude_refined();
    }

    fn magnitude_refinement_at(&self, index: usize) -> bool {
        self.coefficient_states[index].was_magnitude_refined()
    }

    fn is_zero_coded_at(&self, index: usize) -> bool {
        self.coefficient_states[index].was_coded_in_significance_pass()
    }

    fn sign_at(&self, index: usize) -> u8 {
        self.signs[index]
    }

    fn magnitude_bit_at(&self, index: usize) -> u32 {
        (self.magnitudes[index] >> self.current_bit_position) & 1
    }
}

fn cleanup_pass_encode<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let stripe_start = ctx.coefficient_index(x, base_row);
            if stripe_height == 4
                && cleanup_run_mode_applies_encode_at::<VERTICAL_CAUSAL>(ctx, stripe_start)
            {
                let mut run_length = None;
                let mut index = stripe_start;
                for offset in 0..4 {
                    if ctx.magnitude_bit_at(index) == 1 {
                        run_length = Some(offset);
                        break;
                    }
                    index += ctx.padded_width;
                }
                if let Some(run_length) = run_length {
                    encoder.write_bit(17, 1);
                    encoder.write_bit(18, ((run_length >> 1) & 1) as u32);
                    encoder.write_bit(18, (run_length & 1) as u32);
                    let significant = stripe_start + run_length * ctx.padded_width;
                    encode_sign_bit_at::<VERTICAL_CAUSAL>(significant, ctx, encoder);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(significant);

                    let mut index = significant + ctx.padded_width;
                    for _ in (run_length + 1)..stripe_height {
                        cleanup_encode_position_at::<VERTICAL_CAUSAL>(index, ctx, encoder);
                        index += ctx.padded_width;
                    }
                } else {
                    encoder.write_bit(17, 0);
                }
            } else {
                let mut index = stripe_start;
                for _ in 0..stripe_height {
                    cleanup_encode_position_at::<VERTICAL_CAUSAL>(index, ctx, encoder);
                    index += ctx.padded_width;
                }
            }
        }
    }
    Ok(())
}

fn cleanup_run_mode_applies_encode_at<const VERTICAL_CAUSAL: bool>(
    ctx: &BitPlaneEncodeContext<'_>,
    stripe_start: usize,
) -> bool {
    let mut index = stripe_start;
    (0..4).all(|offset| {
        let applies = !ctx.is_significant_at(index)
            && !ctx.is_zero_coded_at(index)
            && !neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width)
                .any();
        if offset != 3 {
            index += ctx.padded_width;
        }
        applies
    })
}

fn cleanup_encode_position_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder,
) {
    if !ctx.is_significant_at(index) && !ctx.is_zero_coded_at(index) {
        let bit = ctx.magnitude_bit_at(index);
        encoder.write_bit(
            context_label_zero_coding_encode_at::<VERTICAL_CAUSAL>(index, ctx),
            bit,
        );
        if bit == 1 {
            encode_sign_bit_at::<VERTICAL_CAUSAL>(index, ctx, encoder);
            ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
        }
    }
}

fn significance_propagation_pass_encode<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder,
) {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                if ctx.is_significant_at(index)
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                let bit = ctx.magnitude_bit_at(index);
                encoder.write_bit(
                    context_label_zero_coding_encode_at::<VERTICAL_CAUSAL>(index, ctx),
                    bit,
                );
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    encode_sign_bit_at::<VERTICAL_CAUSAL>(index, ctx, encoder);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                }
                index += ctx.padded_width;
            }
        }
    }
}

fn significance_propagation_pass_encode_raw<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder<'_>,
) {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                if ctx.is_significant_at(index)
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                let bit = ctx.magnitude_bit_at(index);
                encoder.write_raw_bit(bit);
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    encoder.write_raw_bit(u32::from(ctx.sign_at(index)));
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                }
                index += ctx.padded_width;
            }
        }
    }
}

fn magnitude_refinement_pass_encode<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder,
) {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                if ctx.is_significant_at(index) && !ctx.is_zero_coded_at(index) {
                    let ctx_label = context_label_magnitude_refinement_coding_encode_at::<
                        VERTICAL_CAUSAL,
                    >(index, ctx);
                    encoder.write_bit(ctx_label, ctx.magnitude_bit_at(index));
                    ctx.set_magnitude_refined_at(index);
                }
                index += ctx.padded_width;
            }
        }
    }
}

fn magnitude_refinement_pass_encode_raw(
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder<'_>,
) {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                if ctx.is_significant_at(index) && !ctx.is_zero_coded_at(index) {
                    encoder.write_raw_bit(ctx.magnitude_bit_at(index));
                    ctx.set_magnitude_refined_at(index);
                }
                index += ctx.padded_width;
            }
        }
    }
}

fn encode_sign_bit_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneEncodeContext<'_>,
    encoder: &mut ArithmeticEncoder,
) {
    let (ctx_label, xor_bit) = context_label_sign_coding_encode_at::<VERTICAL_CAUSAL>(index, ctx);
    encoder.write_bit(ctx_label, u32::from(ctx.sign_at(index) ^ xor_bit));
}

fn cleanup_pass<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    let mut aggregation_context = ctx.contexts[17];
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        let row_start = (base_row + COEFFICIENTS_PADDING) * ctx.padded_width + COEFFICIENTS_PADDING;
        for x in 0..ctx.width {
            let stripe_start = row_start + x;
            if stripe_height == 4
                && cleanup_run_mode_applies_at::<VERTICAL_CAUSAL>(ctx, stripe_start)
            {
                let (aggregation, next_context) = decoder.read_bit_value(aggregation_context);
                aggregation_context = next_context;
                if aggregation == 0 {
                    continue;
                }

                let run_length = usize::try_from(
                    (decoder.read_bit(ctx.arithmetic_decoder_context(18)) << 1)
                        | decoder.read_bit(ctx.arithmetic_decoder_context(18)),
                )
                .map_err(|_| Tier1Error::MalformedBitstream {
                    reason: "cleanup run length does not fit usize",
                })?;
                let significant = stripe_start + run_length * ctx.padded_width;
                let sign = decode_sign_bit_at::<VERTICAL_CAUSAL>(significant, ctx, decoder);
                ctx.set_new_significant_coefficient_at(significant, sign);
                ctx.set_significant_at::<VERTICAL_CAUSAL>(significant);

                let mut index = significant + ctx.padded_width;
                for _ in (run_length + 1)..stripe_height {
                    cleanup_decode_position_at::<VERTICAL_CAUSAL>(index, ctx, decoder);
                    index += ctx.padded_width;
                }
            } else {
                let mut index = stripe_start;
                for _ in 0..stripe_height {
                    cleanup_decode_position_at::<VERTICAL_CAUSAL>(index, ctx, decoder);
                    index += ctx.padded_width;
                }
            }
        }
    }
    ctx.contexts[17] = aggregation_context;
    Ok(())
}

fn cleanup_run_mode_applies_at<const VERTICAL_CAUSAL: bool>(
    ctx: &BitPlaneDecodeContext<'_>,
    stripe_start: usize,
) -> bool {
    (0..4).all(|offset| {
        let index = stripe_start + offset * ctx.padded_width;
        let state = ctx.coefficient_states[index];
        !state.is_significant()
            && !state.was_coded_in_significance_pass()
            && !neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width)
                .any()
    })
}

fn cleanup_decode_position_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
) {
    let state = ctx.coefficient_states[index];
    if !state.is_significant() && !state.was_coded_in_significance_pass() {
        let ctx_label = context_label_zero_coding_at::<VERTICAL_CAUSAL>(index, ctx);
        let bit = decoder.read_bit(ctx.arithmetic_decoder_context(ctx_label));
        if bit == 1 {
            let sign = decode_sign_bit_at::<VERTICAL_CAUSAL>(index, ctx, decoder);
            ctx.set_new_significant_coefficient_at(index, sign);
            ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
        }
    }
}

fn significance_propagation_pass<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                let state = ctx.coefficient_states[index];
                if state.is_significant()
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                let ctx_label = context_label_zero_coding_at::<VERTICAL_CAUSAL>(index, ctx);
                let bit = decoder.read_bit(ctx.arithmetic_decoder_context(ctx_label));
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    let sign = decode_sign_bit_at::<VERTICAL_CAUSAL>(index, ctx, decoder);
                    ctx.set_new_significant_coefficient_at(index, sign);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

fn significance_propagation_pass_raw<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut RawDecoder<'_>,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                let state = ctx.coefficient_states[index];
                if state.is_significant()
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                let bit = decoder.read_bit();
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    let sign = decoder.read_bit() as u8;
                    ctx.set_new_significant_coefficient_at(index, sign);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

fn magnitude_refinement_pass<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> Result<()> {
    let bit_position = ctx.current_bit_position;
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                let state = ctx.coefficient_states[index];
                if state.is_significant() && !state.was_coded_in_significance_pass() {
                    let context_label =
                        context_label_magnitude_refinement_at::<VERTICAL_CAUSAL>(index, ctx);
                    let bit = decoder.read_bit(ctx.arithmetic_decoder_context(context_label));
                    ctx.coefficients[index].refine(bit, bit_position);
                    ctx.coefficient_states[index].set_magnitude_refined();
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

fn magnitude_refinement_pass_raw(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut RawDecoder<'_>,
) -> Result<()> {
    let bit_position = ctx.current_bit_position;
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                let state = ctx.coefficient_states[index];
                if state.is_significant() && !state.was_coded_in_significance_pass() {
                    let bit = decoder.read_bit();
                    ctx.coefficients[index].refine(bit, bit_position);
                    ctx.coefficient_states[index].set_magnitude_refined();
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

fn decode_sign_bit_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
) -> u8 {
    let (ctx_label, xor_bit) = context_label_sign_coding_at::<VERTICAL_CAUSAL>(index, ctx);
    let sign_bit = decoder.read_bit(ctx.arithmetic_decoder_context(ctx_label)) ^ u32::from(xor_bit);
    sign_bit as u8
}

fn decode_segmentation_symbol(
    decoder: &mut ArithmeticDecoder<'_>,
    context: &mut ArithmeticDecoderContext,
) -> Result<()> {
    let mut symbol = 0_u32;
    for _ in 0..4 {
        symbol = (symbol << 1) | decoder.read_bit(context);
    }
    if symbol != SEGMENTATION_SYMBOL {
        return Err(Tier1Error::MalformedBitstream {
            reason: "cleanup segmentation symbol is not 0xa",
        });
    }
    Ok(())
}

fn encode_segmentation_symbol(encoder: &mut ArithmeticEncoder<'_>) {
    for shift in (0..4).rev() {
        encoder.write_bit(18, (SEGMENTATION_SYMBOL >> shift) & 1);
    }
}

#[cfg(feature = "std")]
fn cleanup_pass_profiled<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        let row_start = (base_row + COEFFICIENTS_PADDING) * ctx.padded_width + COEFFICIENTS_PADDING;
        for x in 0..ctx.width {
            let stripe_start = row_start + x;
            if stripe_height == 4
                && cleanup_run_mode_applies_at::<VERTICAL_CAUSAL>(ctx, stripe_start)
            {
                counters.cleanup_positions_visited += 4;
                let aggregation = read_context_bit_profiled(
                    ctx,
                    decoder,
                    17,
                    ProfiledPassFamily::Cleanup,
                    counters,
                );
                counters.run_mode_mq_reads += 1;
                if aggregation == 0 {
                    continue;
                }

                let run_msb = read_context_bit_profiled(
                    ctx,
                    decoder,
                    18,
                    ProfiledPassFamily::Cleanup,
                    counters,
                );
                counters.run_mode_mq_reads += 1;
                let run_lsb = read_context_bit_profiled(
                    ctx,
                    decoder,
                    18,
                    ProfiledPassFamily::Cleanup,
                    counters,
                );
                counters.run_mode_mq_reads += 1;
                let run_length = usize::try_from((run_msb << 1) | run_lsb).map_err(|_| {
                    Tier1Error::MalformedBitstream {
                        reason: "cleanup run length does not fit usize",
                    }
                })?;
                let significant = stripe_start + run_length * ctx.padded_width;
                let sign = decode_sign_bit_profiled::<VERTICAL_CAUSAL>(
                    significant,
                    ctx,
                    decoder,
                    counters,
                    ProfiledPassFamily::Cleanup,
                );
                ctx.set_new_significant_coefficient_at(significant, sign);
                ctx.set_significant_at::<VERTICAL_CAUSAL>(significant);
                counters.cleanup_new_significant += 1;

                let mut index = significant + ctx.padded_width;
                for _ in (run_length + 1)..stripe_height {
                    cleanup_decode_position_profiled::<VERTICAL_CAUSAL>(
                        index, ctx, decoder, counters,
                    );
                    index += ctx.padded_width;
                }
            } else {
                let mut index = stripe_start;
                for _ in 0..stripe_height {
                    cleanup_decode_position_profiled::<VERTICAL_CAUSAL>(
                        index, ctx, decoder, counters,
                    );
                    index += ctx.padded_width;
                }
            }
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn cleanup_decode_position_profiled<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) {
    counters.cleanup_positions_visited += 1;
    let state = ctx.coefficient_states[index];
    if !state.is_significant() && !state.was_coded_in_significance_pass() {
        let ctx_label = context_label_zero_coding_at::<VERTICAL_CAUSAL>(index, ctx);
        let bit = read_context_bit_profiled(
            ctx,
            decoder,
            ctx_label,
            ProfiledPassFamily::Cleanup,
            counters,
        );
        if bit == 1 {
            let sign = decode_sign_bit_profiled::<VERTICAL_CAUSAL>(
                index,
                ctx,
                decoder,
                counters,
                ProfiledPassFamily::Cleanup,
            );
            ctx.set_new_significant_coefficient_at(index, sign);
            ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
            counters.cleanup_new_significant += 1;
        }
    }
}

#[cfg(feature = "std")]
fn significance_propagation_pass_profiled<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                counters.significance_positions_visited += 1;
                let state = ctx.coefficient_states[index];
                if state.is_significant()
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                counters.significance_candidates += 1;
                let ctx_label = context_label_zero_coding_at::<VERTICAL_CAUSAL>(index, ctx);
                let bit = read_context_bit_profiled(
                    ctx,
                    decoder,
                    ctx_label,
                    ProfiledPassFamily::SignificancePropagation,
                    counters,
                );
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    let sign = decode_sign_bit_profiled::<VERTICAL_CAUSAL>(
                        index,
                        ctx,
                        decoder,
                        counters,
                        ProfiledPassFamily::SignificancePropagation,
                    );
                    ctx.set_new_significant_coefficient_at(index, sign);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                    counters.significance_new_significant += 1;
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn significance_propagation_pass_raw_profiled<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut RawDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                counters.significance_positions_visited += 1;
                let state = ctx.coefficient_states[index];
                if state.is_significant()
                    || !neighborhood_at::<VERTICAL_CAUSAL>(
                        ctx.coefficient_states,
                        index,
                        ctx.padded_width,
                    )
                    .any()
                {
                    index += ctx.padded_width;
                    continue;
                }
                counters.significance_candidates += 1;
                let bit = decoder.read_bit();
                ctx.set_zero_coded_at(index);
                if bit == 1 {
                    let sign = decoder.read_bit() as u8;
                    ctx.set_new_significant_coefficient_at(index, sign);
                    ctx.set_significant_at::<VERTICAL_CAUSAL>(index);
                    counters.significance_new_significant += 1;
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn magnitude_refinement_pass_profiled<const VERTICAL_CAUSAL: bool>(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    let bit_position = ctx.current_bit_position;
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                counters.magnitude_positions_visited += 1;
                let state = ctx.coefficient_states[index];
                if state.is_significant() && !state.was_coded_in_significance_pass() {
                    counters.magnitude_candidates += 1;
                    let magnitude_refined = state.was_magnitude_refined();
                    let ctx_label =
                        context_label_magnitude_refinement_at::<VERTICAL_CAUSAL>(index, ctx);
                    let bit = read_context_bit_profiled(
                        ctx,
                        decoder,
                        ctx_label,
                        ProfiledPassFamily::MagnitudeRefinement,
                        counters,
                    );
                    ctx.coefficients[index].refine(bit, bit_position);
                    ctx.coefficient_states[index].set_magnitude_refined();
                    counters.magnitude_refinements += 1;
                    if magnitude_refined {
                        counters.magnitude_repeat_refinements += 1;
                    } else {
                        counters.magnitude_first_refinements += 1;
                    }
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn magnitude_refinement_pass_raw_profiled(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut RawDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    let bit_position = ctx.current_bit_position;
    for base_row in (0..ctx.height).step_by(4) {
        let stripe_height = (ctx.height - base_row).min(4);
        for x in 0..ctx.width {
            let mut index = ctx.coefficient_index(x, base_row);
            for _ in 0..stripe_height {
                counters.magnitude_positions_visited += 1;
                let state = ctx.coefficient_states[index];
                if state.is_significant() && !state.was_coded_in_significance_pass() {
                    counters.magnitude_candidates += 1;
                    let magnitude_refined = state.was_magnitude_refined();
                    let bit = decoder.read_bit();
                    ctx.coefficients[index].refine(bit, bit_position);
                    ctx.coefficient_states[index].set_magnitude_refined();
                    counters.magnitude_refinements += 1;
                    if magnitude_refined {
                        counters.magnitude_repeat_refinements += 1;
                    } else {
                        counters.magnitude_first_refinements += 1;
                    }
                }
                index += ctx.padded_width;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn decode_sign_bit_profiled<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
    pass_family: ProfiledPassFamily,
) -> u8 {
    let (ctx_label, xor_bit) = context_label_sign_coding_at::<VERTICAL_CAUSAL>(index, ctx);
    counters.sign_mq_reads += 1;
    let sign_bit = read_context_bit_profiled(ctx, decoder, ctx_label, pass_family, counters)
        ^ u32::from(xor_bit);
    sign_bit as u8
}

#[cfg(feature = "std")]
fn decode_segmentation_symbol_profiled(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> Result<()> {
    let mut symbol = 0_u32;
    for _ in 0..4 {
        symbol = (symbol << 1)
            | read_context_bit_profiled(ctx, decoder, 18, ProfiledPassFamily::Cleanup, counters);
    }
    if symbol != SEGMENTATION_SYMBOL {
        return Err(Tier1Error::MalformedBitstream {
            reason: "cleanup segmentation symbol is not 0xa",
        });
    }
    Ok(())
}

#[cfg(feature = "std")]
fn read_context_bit_profiled(
    ctx: &mut BitPlaneDecodeContext<'_>,
    decoder: &mut ArithmeticDecoder<'_>,
    ctx_label: u8,
    pass_family: ProfiledPassFamily,
    counters: &mut CodeBlockDecodeWorkCounters,
) -> u32 {
    match pass_family {
        ProfiledPassFamily::Cleanup => counters.cleanup_mq_reads += 1,
        ProfiledPassFamily::SignificancePropagation => counters.significance_mq_reads += 1,
        ProfiledPassFamily::MagnitudeRefinement => counters.magnitude_mq_reads += 1,
    }
    counters.context_reads_by_label[usize::from(ctx_label)] += 1;
    decoder.read_bit(ctx.arithmetic_decoder_context(ctx_label))
}

fn context_label_sign_coding_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneDecodeContext<'_>,
) -> (u8, u8) {
    let neighbors =
        neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width);
    sign_context(
        sign_contribution(
            neighbors.left,
            ctx.coefficients[index - 1].sign_bit(),
            neighbors.right,
            ctx.coefficients[index + 1].sign_bit(),
        ),
        sign_contribution(
            neighbors.top,
            ctx.coefficients[index - ctx.padded_width].sign_bit(),
            neighbors.bottom,
            ctx.coefficients[index + ctx.padded_width].sign_bit(),
        ),
    )
}

fn context_label_sign_coding_encode_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneEncodeContext<'_>,
) -> (u8, u8) {
    let neighbors =
        neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width);
    sign_context(
        sign_contribution(
            neighbors.left,
            ctx.sign_at(index - 1),
            neighbors.right,
            ctx.sign_at(index + 1),
        ),
        sign_contribution(
            neighbors.top,
            ctx.sign_at(index - ctx.padded_width),
            neighbors.bottom,
            ctx.sign_at(index + ctx.padded_width),
        ),
    )
}

fn context_label_zero_coding_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneDecodeContext<'_>,
) -> u8 {
    zero_coding_context(
        ctx.subband,
        neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width),
    )
}

fn context_label_zero_coding_encode_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneEncodeContext<'_>,
) -> u8 {
    zero_coding_context(
        ctx.subband,
        neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width),
    )
}

fn context_label_magnitude_refinement_coding_encode_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneEncodeContext<'_>,
) -> u8 {
    if ctx.magnitude_refinement_at(index) {
        16
    } else {
        14 + u8::from(
            neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width)
                .any(),
        )
    }
}

fn context_label_magnitude_refinement_at<const VERTICAL_CAUSAL: bool>(
    index: usize,
    ctx: &BitPlaneDecodeContext<'_>,
) -> u8 {
    if ctx.coefficient_states[index].was_magnitude_refined() {
        16
    } else {
        14 + u8::from(
            neighborhood_at::<VERTICAL_CAUSAL>(ctx.coefficient_states, index, ctx.padded_width)
                .any(),
        )
    }
}

pub(crate) fn sign_contribution(
    first_significant: bool,
    first_sign: u8,
    second_significant: bool,
    second_sign: u8,
) -> i8 {
    let first = if first_significant {
        1 - 2 * (first_sign as i8)
    } else {
        0
    };
    let second = if second_significant {
        1 - 2 * (second_sign as i8)
    } else {
        0
    };
    (first + second).signum()
}

pub(crate) fn sign_context(horizontal: i8, vertical: i8) -> (u8, u8) {
    match (horizontal, vertical) {
        (1, 1) => (13, 0),
        (1, 0) => (12, 0),
        (1, -1) => (11, 0),
        (0, 1) => (10, 0),
        (0, 0) => (9, 0),
        (0, -1) => (10, 1),
        (-1, 1) => (11, 1),
        (-1, 0) => (12, 1),
        (-1, -1) => (13, 1),
        _ => unreachable!("sign contributions are clamped to -1, 0, or 1"),
    }
}

pub(crate) fn zero_coding_context(subband: Subband, neighbors: Neighborhood) -> u8 {
    let horizontal = neighbors.horizontal_count();
    let vertical = neighbors.vertical_count();
    let diagonal = neighbors.diagonal_count();
    match subband {
        Subband::LowLow | Subband::LowHigh => {
            zero_coding_context_hv(horizontal, vertical, diagonal)
        }
        Subband::HighLow => zero_coding_context_hv(vertical, horizontal, diagonal),
        Subband::HighHigh => match diagonal {
            0 => (horizontal + vertical).min(2),
            1 => 3 + (horizontal + vertical).min(2),
            2 if horizontal + vertical == 0 => 6,
            2 => 7,
            _ => 8,
        },
    }
}

fn zero_coding_context_hv(primary: u8, secondary: u8, diagonal: u8) -> u8 {
    match (primary, secondary, diagonal) {
        (2, _, _) => 8,
        (1, 1..=2, _) => 7,
        (1, 0, 1..=4) => 6,
        (1, 0, 0) => 5,
        (0, 2, _) => 4,
        (0, 1, _) => 3,
        (0, 0, 2..=4) => 2,
        (0, 0, 1) => 1,
        (0, 0, 0) => 0,
        _ => unreachable!("neighbour counts are bounded by Annex D"),
    }
}

impl<'a> MqByteInput<'a> {
    /// Create a new byte input over one code-block segment.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            previous_was_ff: false,
        }
    }

    /// Number of segment bytes consumed so far.
    pub fn bytes_consumed(self) -> usize {
        self.offset
    }

    /// Read the next MQ-coded byte, validating stuffed marker-prefix syntax.
    pub fn read_byte(&mut self) -> Result<u8> {
        let byte = *self
            .bytes
            .get(self.offset)
            .ok_or(Tier1Error::MalformedBitstream {
                reason: "MQ byte input reached the end of the code-block segment",
            })?;
        self.offset += 1;

        if self.previous_was_ff && byte > 0x8f {
            return Err(Tier1Error::MalformedBitstream {
                reason: "marker prefix found inside MQ-coded code-block segment",
            });
        }

        self.previous_was_ff = byte == 0xff;
        Ok(byte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment_descriptors(
        style: CodeBlockStyle,
        byte_lengths: &[usize],
        pass_count: u16,
    ) -> Vec<CodeBlockSegment> {
        let mut coding_pass = 0_u16;
        byte_lengths
            .iter()
            .enumerate()
            .map(|(index, &byte_len)| {
                let coding_passes = if style.terminates_each_pass() {
                    1
                } else if style.uses_selective_arithmetic_bypass() {
                    let remaining = pass_count - coding_pass;
                    bypass_segment_pass_capacity(coding_pass).min(remaining)
                } else {
                    assert_eq!(index, 0);
                    pass_count
                };
                coding_pass += coding_passes;
                CodeBlockSegment {
                    byte_len,
                    coding_passes,
                }
            })
            .collect::<Vec<_>>()
    }

    fn qualification_coefficients(width: usize, height: usize) -> Vec<i32> {
        let mut coefficients = (0..width * height)
            .map(|index| {
                let x = index % width;
                let y = index / width;
                let magnitude = ((x * 37 + y * 53 + x * y * 3) % 253) as i32;
                if (x + 2 * y).is_multiple_of(3) {
                    -magnitude
                } else {
                    magnitude
                }
            })
            .collect::<Vec<_>>();
        coefficients[0] = 255;
        coefficients[width * height - 1] = -254;
        coefficients
    }

    #[test]
    fn annex_d_zero_coding_context_boundaries() {
        let none = Neighborhood::default();
        assert_eq!(zero_coding_context(Subband::LowLow, none), 0);
        assert_eq!(zero_coding_context(Subband::HighHigh, none), 0);

        let horizontal_pair = Neighborhood {
            left: true,
            right: true,
            ..Neighborhood::default()
        };
        assert_eq!(zero_coding_context(Subband::LowHigh, horizontal_pair), 8);
        assert_eq!(zero_coding_context(Subband::HighLow, horizontal_pair), 4);
        assert_eq!(zero_coding_context(Subband::HighHigh, horizontal_pair), 2);

        let diagonal_pair = Neighborhood {
            top_left: true,
            bottom_right: true,
            ..Neighborhood::default()
        };
        assert_eq!(zero_coding_context(Subband::LowLow, diagonal_pair), 2);
        assert_eq!(zero_coding_context(Subband::HighHigh, diagonal_pair), 6);

        for mask in 0_u8..=u8::MAX {
            let neighbors = Neighborhood::from_mask(mask);
            for subband in [
                Subband::LowLow,
                Subband::LowHigh,
                Subband::HighLow,
                Subband::HighHigh,
            ] {
                assert!(zero_coding_context(subband, neighbors) <= 8);
            }
        }
    }

    #[test]
    fn annex_d_sign_contexts_cover_all_contributions() {
        let expected = [
            ((1, 1), (13, 0)),
            ((1, 0), (12, 0)),
            ((1, -1), (11, 0)),
            ((0, 1), (10, 0)),
            ((0, 0), (9, 0)),
            ((0, -1), (10, 1)),
            ((-1, 1), (11, 1)),
            ((-1, 0), (12, 1)),
            ((-1, -1), (13, 1)),
        ];
        for ((horizontal, vertical), context) in expected {
            assert_eq!(sign_context(horizontal, vertical), context);
        }
    }

    #[test]
    fn all_classic_styles_round_trip_through_each_decoder_backend() {
        let width = 17;
        let height = 11;
        let dimensions = CodeBlockDimensions::new(width, height).unwrap();
        let coefficients = qualification_coefficients(width as usize, height as usize);
        let styles = [
            CodeBlockStyle::NONE.bits(),
            CodeBlockStyle::RESET_CONTEXTS,
            CodeBlockStyle::TERMINATE_EACH_PASS,
            CodeBlockStyle::VERTICALLY_CAUSAL,
            CodeBlockStyle::PREDICTABLE_TERMINATION,
            CodeBlockStyle::SEGMENTATION_SYMBOLS,
            CodeBlockStyle::SELECTIVE_ARITHMETIC_BYPASS,
            CodeBlockStyle::SELECTIVE_ARITHMETIC_BYPASS | CodeBlockStyle::PREDICTABLE_TERMINATION,
            CodeBlockStyle::SELECTIVE_ARITHMETIC_BYPASS
                | CodeBlockStyle::RESET_CONTEXTS
                | CodeBlockStyle::VERTICALLY_CAUSAL
                | CodeBlockStyle::SEGMENTATION_SYMBOLS,
            CodeBlockStyle::TERMINATE_EACH_PASS
                | CodeBlockStyle::RESET_CONTEXTS
                | CodeBlockStyle::VERTICALLY_CAUSAL
                | CodeBlockStyle::PREDICTABLE_TERMINATION
                | CodeBlockStyle::SEGMENTATION_SYMBOLS,
        ];

        for subband in [
            Subband::LowLow,
            Subband::LowHigh,
            Subband::HighLow,
            Subband::HighHigh,
        ] {
            for style_bits in styles {
                let encode_spec = CodeBlockEncodeSpec {
                    dimensions,
                    subband,
                    available_bitplanes: 9,
                    code_block_style: style_bits,
                };
                let mut encoded_bytes = Vec::new();
                let mut segment_lengths = Vec::new();
                let mut encode_scratch = CodeBlockEncodeScratch::new();
                let encoded = encode_baseline_code_block_segments_with_scratch(
                    &coefficients,
                    encode_spec,
                    &mut encoded_bytes,
                    &mut segment_lengths,
                    &mut encode_scratch,
                )
                .unwrap();
                assert!(encoded.included);
                assert_eq!(encoded.byte_len, encoded_bytes.len());
                assert_eq!(segment_lengths.iter().sum::<usize>(), encoded.byte_len);

                let style = CodeBlockStyle::from_bits(style_bits);
                let segments = segment_descriptors(style, &segment_lengths, encoded.pass_count);
                assert_eq!(
                    segments
                        .iter()
                        .map(|segment| segment.coding_passes)
                        .sum::<u16>(),
                    encoded.pass_count
                );
                let decode_spec = CodeBlockDecodeSpec {
                    dimensions,
                    available_bitplanes: encode_spec.available_bitplanes,
                    missing_most_significant_bitplanes: encoded.missing_bitplanes,
                    coding_passes: encoded.pass_count,
                    style,
                    subband,
                };

                let mut checked = vec![0; coefficients.len()];
                let mut checked_scratch = CodeBlockDecodeScratch::new();
                decode_baseline_code_block_segments_with_scratch(
                    &encoded_bytes,
                    &segments,
                    decode_spec,
                    &mut checked,
                    &mut checked_scratch,
                )
                .unwrap();
                assert_eq!(
                    checked, coefficients,
                    "checked {subband:?} style {style_bits:#04x}"
                );

                let mut packed = vec![0; coefficients.len()];
                let mut packed_scratch = CodeBlockDecodeScratch::new();
                decode_baseline_code_block_segments_with_packed_scratch(
                    &encoded_bytes,
                    &segments,
                    decode_spec,
                    &mut packed,
                    &mut packed_scratch,
                )
                .unwrap();
                assert_eq!(
                    packed, coefficients,
                    "packed {subband:?} style {style_bits:#04x}"
                );

                let mut sparse = vec![0; coefficients.len()];
                let mut sparse_scratch = CodeBlockDecodeScratch::new();
                decode_baseline_code_block_segments_with_sparse_scratch_outcome(
                    &encoded_bytes,
                    &segments,
                    decode_spec,
                    &mut sparse,
                    &mut sparse_scratch,
                )
                .unwrap();
                assert_eq!(
                    sparse, coefficients,
                    "sparse {subband:?} style {style_bits:#04x}"
                );
            }
        }
    }

    #[test]
    fn zero_code_block_emits_no_segment_and_decodes_without_input() {
        let dimensions = CodeBlockDimensions::new(5, 3).unwrap();
        let encode_spec = CodeBlockEncodeSpec {
            dimensions,
            subband: Subband::LowLow,
            available_bitplanes: 8,
            code_block_style: CodeBlockStyle::NONE.bits(),
        };
        let mut output = Vec::new();
        let encoded = encode_baseline_code_block(&[0; 15], encode_spec, &mut output).unwrap();
        assert_eq!(encoded.pass_count, 0);
        assert!(!encoded.included);
        assert!(output.is_empty());

        let decode_spec = CodeBlockDecodeSpec {
            dimensions,
            available_bitplanes: 8,
            missing_most_significant_bitplanes: 8,
            coding_passes: 0,
            style: CodeBlockStyle::NONE,
            subband: Subband::LowLow,
        };
        let mut coefficients = [1; 15];
        decode_baseline_code_block_segments(&[], &[], decode_spec, &mut coefficients).unwrap();
        assert_eq!(coefficients, [0; 15]);
    }

    #[test]
    fn guard_bit_magnitude_planes_bound_missing_planes_and_coding_passes() {
        let dimensions = CodeBlockDimensions::new(1, 1).unwrap();
        let spec = |available_bitplanes, missing_bitplanes, coding_passes| CodeBlockDecodeSpec {
            dimensions,
            available_bitplanes,
            missing_most_significant_bitplanes: missing_bitplanes,
            coding_passes,
            style: CodeBlockStyle::NONE,
            subband: Subband::LowLow,
        };

        assert_eq!(maximum_coding_passes(37), 109);
        assert!(matches!(
            validate_bitplane_pass_count(spec(37, 0, 110)),
            Err(Tier1Error::UnsupportedCodingPass { .. })
        ));
        assert!(matches!(
            validate_bitplane_pass_count(spec(37, 0, 109)),
            Err(Tier1Error::UnsupportedCodingPass { .. })
        ));
        assert_eq!(validate_bitplane_pass_count(spec(9, 8, 1)).unwrap(), 1);
        assert!(matches!(
            validate_bitplane_pass_count(spec(9, 8, 2)),
            Err(Tier1Error::UnsupportedCodingPass { .. })
        ));
        assert_eq!(validate_bitplane_pass_count(spec(9, 9, 0)).unwrap(), 0);
        assert!(matches!(
            validate_bitplane_pass_count(spec(9, 10, 0)),
            Err(Tier1Error::MalformedBitstream { .. })
        ));
    }

    #[test]
    fn every_classic_backend_rejects_unrepresentable_magnitude_width() {
        let spec = CodeBlockDecodeSpec {
            dimensions: CodeBlockDimensions::new(1, 1).unwrap(),
            available_bitplanes: MAX_RECONSTRUCTED_MAGNITUDE_BITPLANES + 1,
            missing_most_significant_bitplanes: 0,
            coding_passes: 1,
            style: CodeBlockStyle::NONE,
            subband: Subband::LowLow,
        };
        let segment = [0_u8];
        let coding_segments = [CodeBlockSegment {
            byte_len: segment.len(),
            coding_passes: spec.coding_passes,
        }];

        let assert_rejected = |result: Result<_>| {
            assert!(matches!(
                result,
                Err(Tier1Error::UnsupportedCodingPass { .. })
            ));
        };

        let mut checked = [0_i32];
        let mut checked_scratch = CodeBlockDecodeScratch::new();
        assert_rejected(
            decode_baseline_code_block_segments_with_scratch(
                &segment,
                &coding_segments,
                spec,
                &mut checked,
                &mut checked_scratch,
            )
            .map(|_| ()),
        );

        let mut dense = [0_i32];
        let mut dense_scratch = CodeBlockDecodeScratch::new();
        assert_rejected(
            decode_baseline_code_block_segments_with_packed_scratch_outcome(
                &segment,
                &coding_segments,
                spec,
                &mut dense,
                &mut dense_scratch,
            )
            .map(|_| ()),
        );

        let mut sparse = [0_i32];
        let mut sparse_scratch = CodeBlockDecodeScratch::new();
        assert_rejected(
            decode_baseline_code_block_segments_with_sparse_scratch_outcome(
                &segment,
                &coding_segments,
                spec,
                &mut sparse,
                &mut sparse_scratch,
            )
            .map(|_| ()),
        );
    }
}

/// Boundary marker retained for callers that only need to name the crate layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tier1Boundary;
