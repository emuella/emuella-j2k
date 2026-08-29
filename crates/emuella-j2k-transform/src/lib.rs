#![cfg_attr(not(feature = "std"), no_std)]
//! Wavelet transform and quantization boundary.
//!
//! This crate owns reversible 5/3, irreversible 9/7, component transform, and
//! quantization code. SIMD and parallel paths must remain feature-gated and keep
//! deterministic scalar fallbacks.

extern crate alloc;

use core::fmt;

mod full;
mod window;

pub use full::{
    FullSynthesisBackend, FullSynthesisEstimate, FullSynthesisLevel, FullSynthesisPlan,
    FullSynthesisReport, FullSynthesisWorkspace, FullWorkspaceCapacities,
    full_transpose_backend_name,
};
#[cfg(feature = "parallel")]
pub use full::{
    inverse_irreversible_9_7_full_parallel,
    inverse_irreversible_9_7_full_parallel_deferred_transpose,
    inverse_irreversible_9_7_full_parallel_deferred_transpose_with_workers,
    inverse_irreversible_9_7_full_parallel_with_workers, inverse_reversible_5_3_full_parallel,
    inverse_reversible_5_3_full_parallel_deferred_transpose,
    inverse_reversible_5_3_full_parallel_deferred_transpose_with_workers,
    inverse_reversible_5_3_full_parallel_direct_unsigned_u8,
    inverse_reversible_5_3_full_parallel_direct_unsigned_u8_region,
    inverse_reversible_5_3_full_parallel_direct_unsigned_u8_region_with_workers,
    inverse_reversible_5_3_full_parallel_with_workers,
};
pub use window::{
    AxisAlignedRegion, ResolutionWindowPlan, WindowCoefficientBand, WindowCoefficientPlane,
    WindowSubband, WindowSynthesisPlan, WindowSynthesisReport, WindowSynthesisStageTimings,
    WindowSynthesisWork, WindowSynthesisWorkspace, WindowWorkspaceCapacities,
    inverse_irreversible_9_7_window, inverse_reversible_5_3_window, plan_window_synthesis,
};
#[cfg(feature = "parallel")]
pub use window::{
    inverse_irreversible_9_7_window_parallel,
    inverse_irreversible_9_7_window_parallel_with_workers, inverse_reversible_5_3_window_parallel,
    inverse_reversible_5_3_window_parallel_with_workers,
};

#[cfg(test)]
extern crate std;

/// Wavelet transform family selected by coding style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveletTransform {
    /// Reversible 5/3 integer transform.
    Reversible53,
    /// Irreversible 9/7 floating-point transform.
    Irreversible97,
}

/// Boundary marker for future scalar transform state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransformBoundary {
    pub transform: WaveletTransform,
}

/// Low-pass or high-pass band placement for one transform axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformBand {
    /// The first sample along the axis belongs to the low-pass band.
    Low,
    /// The first sample along the axis belongs to the high-pass band.
    High,
}

/// Original component sample range used for reversible-transform validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentSampleRange {
    /// Number of significant bits in the original component samples.
    pub precision_bits: u8,
    /// Whether the original component samples are signed.
    pub signed: bool,
}

impl ComponentSampleRange {
    /// Construct a signed component range with JPEG 2000 sample precision.
    pub const fn signed(precision_bits: u8) -> Self {
        Self {
            precision_bits,
            signed: true,
        }
    }

    /// Construct an unsigned component range with JPEG 2000 sample precision.
    pub const fn unsigned(precision_bits: u8) -> Self {
        Self {
            precision_bits,
            signed: false,
        }
    }

    fn bounds(self) -> Result<(i32, i32), TransformError> {
        if self.signed {
            if self.precision_bits == 0 || self.precision_bits > 32 {
                return Err(TransformError::InvalidSampleRange);
            }
            if self.precision_bits == 32 {
                Ok((i32::MIN, i32::MAX))
            } else {
                let magnitude_bits = u32::from(self.precision_bits - 1);
                let min = -(1_i32 << magnitude_bits);
                let max = (1_i32 << magnitude_bits) - 1;
                Ok((min, max))
            }
        } else {
            if self.precision_bits == 0 || self.precision_bits > 31 {
                return Err(TransformError::InvalidSampleRange);
            }
            Ok((0, (1_i32 << u32::from(self.precision_bits)) - 1))
        }
    }

    fn contains(self, value: i32) -> Result<bool, TransformError> {
        let (min, max) = self.bounds()?;
        Ok((min..=max).contains(&value))
    }
}

/// Low-band sizes and first-sample parity for the two reversible 5/3 axes.
///
/// JPEG 2000 subband parity comes from the tile-component origin. Use
/// [`Reversible53Edges::from_tile_origin`] when the original coordinate origin
/// is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reversible53Edges {
    /// Number of low-pass samples produced by each transformed row.
    pub horizontal_low_samples: usize,
    /// Number of low-pass samples produced by each transformed column.
    pub vertical_low_samples: usize,
    /// Band assignment for the first sample in every row.
    pub horizontal_first: TransformBand,
    /// Band assignment for the first sample in every column.
    pub vertical_first: TransformBand,
}

impl Reversible53Edges {
    /// Return JPEG 2000 Part 1 edge sizes for a tile-component origin.
    ///
    /// A sample at an even reference-grid coordinate starts in the low-pass
    /// band; a sample at an odd coordinate starts in the high-pass band.
    pub const fn from_tile_origin(x0: usize, y0: usize, width: usize, height: usize) -> Self {
        let horizontal_first = if x0.is_multiple_of(2) {
            TransformBand::Low
        } else {
            TransformBand::High
        };
        let vertical_first = if y0.is_multiple_of(2) {
            TransformBand::Low
        } else {
            TransformBand::High
        };

        Self {
            horizontal_low_samples: low_sample_count(width, horizontal_first),
            vertical_low_samples: low_sample_count(height, vertical_first),
            horizontal_first,
            vertical_first,
        }
    }
}

/// Geometry and validation parameters for an in-place reversible 5/3 transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reversible53Config {
    /// Active component-plane width in samples.
    pub width: usize,
    /// Active component-plane height in samples.
    pub height: usize,
    /// Distance between the first samples of adjacent rows in `plane`.
    pub stride: usize,
    /// Low-band edge sizes and first-sample parity.
    pub edges: Reversible53Edges,
    /// Original component sample range.
    pub sample_range: ComponentSampleRange,
}

/// JPEG 2000 scalar quantization style declared by QCD or QCC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantizationStyle {
    /// One exponent byte per subband; valid only with the reversible transform.
    NoQuantization,
    /// One exponent/mantissa pair from which every subband step is derived.
    ScalarDerived,
    /// One explicit exponent/mantissa pair per subband.
    ScalarExpounded,
}

/// One Part 1 irreversible scalar quantization step-size entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrreversibleQuantizationStep {
    /// Five-bit exponent epsilon_b from the marker segment.
    pub exponent: u8,
    /// Eleven-bit mantissa mu_b from the marker segment.
    pub mantissa: u16,
}

impl IrreversibleQuantizationStep {
    /// Construct a validated Part 1 irreversible quantization step.
    pub const fn new(exponent: u8, mantissa: u16) -> Result<Self, TransformError> {
        if exponent > 31 || mantissa > 0x07ff {
            return Err(TransformError::InvalidQuantizationStep);
        }
        Ok(Self { exponent, mantissa })
    }

    /// Compute Delta_b from Part 1 equation E-3.
    ///
    /// `component_precision` is the SIZ component precision and
    /// `subband_gain` is zero for LL, one for HL/LH, and two for HH.
    pub fn delta(self, component_precision: u8, subband_gain: u8) -> Result<f32, TransformError> {
        if component_precision == 0 || component_precision > 31 || subband_gain > 2 {
            return Err(TransformError::InvalidQuantizationStep);
        }
        let power =
            i16::from(component_precision) + i16::from(subband_gain) - i16::from(self.exponent);
        let scale = power_of_two_f32(power).ok_or(TransformError::InvalidQuantizationStep)?;
        Ok((1.0 + f32::from(self.mantissa) / 2048.0) * scale)
    }
}

/// Low-band sizes and first-sample parity for the irreversible 9/7 axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Irreversible97Edges {
    pub horizontal_low_samples: usize,
    pub vertical_low_samples: usize,
    pub horizontal_first: TransformBand,
    pub vertical_first: TransformBand,
}

impl Irreversible97Edges {
    /// Return JPEG 2000 Part 1 edge sizes for a tile-component origin.
    pub const fn from_tile_origin(x0: usize, y0: usize, width: usize, height: usize) -> Self {
        let horizontal_first = if x0.is_multiple_of(2) {
            TransformBand::Low
        } else {
            TransformBand::High
        };
        let vertical_first = if y0.is_multiple_of(2) {
            TransformBand::Low
        } else {
            TransformBand::High
        };
        Self {
            horizontal_low_samples: low_sample_count(width, horizontal_first),
            vertical_low_samples: low_sample_count(height, vertical_first),
            horizontal_first,
            vertical_first,
        }
    }
}

/// Geometry for an allocation-free in-place inverse irreversible 9/7 transform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Irreversible97Config {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub edges: Irreversible97Edges,
}

impl Irreversible97Config {
    /// Scratch elements required by [`forward_irreversible_9_7`] and
    /// [`inverse_irreversible_9_7`].
    pub const fn scratch_len(self) -> usize {
        2 * max_usize(self.width, self.height)
    }
}

impl Reversible53Config {
    /// Scratch elements required by [`forward_reversible_5_3`] and
    /// [`inverse_reversible_5_3`].
    ///
    /// The transform is allocation-free: callers provide a column line buffer,
    /// a read buffer, and a coefficient buffer, each sized for the longer plane
    /// axis.
    pub const fn scratch_len(self) -> usize {
        3 * max_usize(self.width, self.height)
    }
}

/// Errors reported by reversible transform APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformError {
    /// Width and height must both be greater than zero.
    EmptyPlane,
    /// Stride must be at least the active width.
    StrideTooSmall,
    /// Plane length cannot contain the configured width, height, and stride.
    PlaneTooSmall,
    /// Scratch length is smaller than [`Reversible53Config::scratch_len`].
    ScratchTooSmall { required: usize, actual: usize },
    /// Low-band edge sizes or first-band parity do not match plane dimensions.
    InvalidEdges,
    /// Component precision cannot be represented by this `i32` transform API.
    InvalidSampleRange,
    /// A source or reconstructed sample is outside the configured range.
    SampleOutOfRange { index: usize, value: i32 },
    /// Component planes supplied to a multi-component transform have different lengths.
    ComponentLengthMismatch,
    /// Intermediate arithmetic overflowed `i32`.
    ArithmeticOverflow,
    /// A quantization exponent, mantissa, precision, or gain is outside Part 1 bounds.
    InvalidQuantizationStep,
    /// Irreversible reconstruction produced a non-finite floating-point value.
    NonFiniteSample { index: usize },
    /// A bounded synthesis region, level, or coefficient layout is invalid.
    InvalidWindow,
    /// Bounded synthesis dimensions or storage requirements overflowed.
    SizeOverflow,
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPlane => f.write_str("transform plane dimensions must be greater than zero"),
            Self::StrideTooSmall => {
                f.write_str("transform stride must be at least the plane width")
            }
            Self::PlaneTooSmall => {
                f.write_str("transform plane buffer is too small for width, height, and stride")
            }
            Self::ScratchTooSmall { required, actual } => write!(
                f,
                "transform scratch buffer is too small: required {required}, got {actual}"
            ),
            Self::InvalidEdges => {
                f.write_str("transform edge sizes do not match dimensions and first-band parity")
            }
            Self::InvalidSampleRange => {
                f.write_str("component sample range is not representable by i32 samples")
            }
            Self::SampleOutOfRange { index, value } => {
                write!(
                    f,
                    "component sample {index} with value {value} is out of range"
                )
            }
            Self::ComponentLengthMismatch => {
                f.write_str("component transform planes must have matching lengths")
            }
            Self::ArithmeticOverflow => {
                f.write_str("reversible 5/3 transform arithmetic overflowed i32")
            }
            Self::InvalidQuantizationStep => {
                f.write_str("irreversible quantization step is outside Part 1 bounds")
            }
            Self::NonFiniteSample { index } => {
                write!(f, "irreversible transform sample {index} is not finite")
            }
            Self::InvalidWindow => f.write_str("bounded synthesis window is invalid"),
            Self::SizeOverflow => f.write_str("bounded synthesis storage size overflowed"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TransformError {}

/// Apply an in-place two-dimensional reversible 5/3 forward transform.
///
/// Coefficients are stored with low-pass samples first along each transformed
/// axis. After the transform, the top-left region is LL, the top-right region is
/// HL, the bottom-left region is LH, and the bottom-right region is HH. The
/// scalar lifting path is deterministic and does not allocate.
pub fn forward_reversible_5_3(
    plane: &mut [i32],
    config: Reversible53Config,
    scratch: &mut [i32],
) -> Result<(), TransformError> {
    validate_config(plane, config, scratch)?;
    validate_samples_in_range(plane, config)?;

    let max_axis = max_usize(config.width, config.height);
    let (line, rest) = scratch.split_at_mut(max_axis);
    let (read, coeffs) = rest.split_at_mut(max_axis);
    for x in 0..config.width {
        copy_strided_column_to_line(plane, config.stride, x, &mut line[..config.height]);
        transform_line_forward(
            &mut line[..config.height],
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &mut read[..config.height],
            &mut coeffs[..config.height],
        )?;
        write_line_to_strided_column(plane, config.stride, x, &line[..config.height]);
    }

    for y in 0..config.height {
        let start = y * config.stride;
        transform_line_forward(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut read[..config.width],
            &mut coeffs[..config.width],
        )?;
    }

    Ok(())
}

/// Apply an in-place reversible 5/3 forward transform for prevalidated bounded data.
///
/// This fast path preserves the same coefficient layout as
/// [`forward_reversible_5_3`] but skips per-lift overflow checks. Callers must
/// prove the configured profile cannot overflow `i32`; use the checked API for
/// untrusted or general-purpose transform input.
pub fn forward_reversible_5_3_bounded(
    plane: &mut [i32],
    config: Reversible53Config,
    scratch: &mut [i32],
) -> Result<(), TransformError> {
    validate_config(plane, config, scratch)?;
    validate_samples_in_range(plane, config)?;

    let max_axis = max_usize(config.width, config.height);
    let (_line, rest) = scratch.split_at_mut(max_axis);
    let (read, coeffs) = rest.split_at_mut(max_axis);
    for x in 0..config.width {
        copy_strided_column_to_line(plane, config.stride, x, &mut read[..config.height]);
        transform_line_forward_from_read_to_strided_bounded(
            config.height,
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &read[..config.height],
            &mut coeffs[..config.height],
            plane,
            config.stride,
            x,
        );
    }

    for y in 0..config.height {
        let start = y * config.stride;
        transform_line_forward_bounded(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut coeffs[..config.width],
        );
    }

    Ok(())
}

/// Apply an in-place two-dimensional reversible 5/3 inverse transform.
///
/// The input coefficient layout must match [`forward_reversible_5_3`]. The
/// reconstructed samples are validated against the configured signedness and
/// precision range before the function returns.
pub fn inverse_reversible_5_3(
    plane: &mut [i32],
    config: Reversible53Config,
    scratch: &mut [i32],
) -> Result<(), TransformError> {
    validate_config(plane, config, scratch)?;

    let max_axis = max_usize(config.width, config.height);
    let (line, rest) = scratch.split_at_mut(max_axis);
    let (read, _coeffs) = rest.split_at_mut(max_axis);
    for y in 0..config.height {
        let start = y * config.stride;
        transform_line_inverse(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut read[..config.width],
        )?;
    }

    for x in 0..config.width {
        copy_strided_column_to_line(plane, config.stride, x, &mut line[..config.height]);
        transform_line_inverse(
            &mut line[..config.height],
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &mut read[..config.height],
        )?;
        write_line_to_strided_column(plane, config.stride, x, &line[..config.height]);
    }

    validate_samples_in_range(plane, config)?;
    Ok(())
}

/// Apply an in-place reversible 5/3 inverse transform for prevalidated bounded data.
///
/// This fast path preserves the checked API's coefficient layout but skips
/// per-lift overflow checks. Callers must prove the transform coefficients are
/// bounded enough that reconstruction cannot overflow `i32`.
pub fn inverse_reversible_5_3_bounded(
    plane: &mut [i32],
    config: Reversible53Config,
    scratch: &mut [i32],
) -> Result<(), TransformError> {
    inverse_reversible_5_3_bounded_with_stage_timings(plane, config, scratch, false).map(|_| ())
}

/// Profiled form of [`inverse_reversible_5_3_bounded`].
///
/// Timing is opt-in and leaves the non-profiled path free of clock reads.
#[cfg_attr(not(feature = "std"), allow(unused_mut, unused_variables))]
pub fn inverse_reversible_5_3_bounded_with_stage_timings(
    plane: &mut [i32],
    config: Reversible53Config,
    scratch: &mut [i32],
    collect_stage_timings: bool,
) -> Result<WindowSynthesisStageTimings, TransformError> {
    #[cfg(feature = "std")]
    let preparation_started = collect_stage_timings.then(std::time::Instant::now);
    validate_config(plane, config, scratch)?;

    let max_axis = max_usize(config.width, config.height);
    let (line, rest) = scratch.split_at_mut(max_axis);
    let (read, _coeffs) = rest.split_at_mut(max_axis);
    let mut timings = WindowSynthesisStageTimings::default();
    #[cfg(feature = "std")]
    if let Some(preparation_started) = preparation_started {
        timings.level_preparation_ns = preparation_started.elapsed().as_nanos();
    }
    #[cfg(feature = "std")]
    let horizontal_started = collect_stage_timings.then(std::time::Instant::now);
    for y in 0..config.height {
        let start = y * config.stride;
        transform_line_inverse_bounded(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut read[..config.width],
        );
    }
    #[cfg(feature = "std")]
    if let Some(horizontal_started) = horizontal_started {
        timings.horizontal_ns = horizontal_started.elapsed().as_nanos();
    }

    #[cfg(feature = "std")]
    let vertical_started = collect_stage_timings.then(std::time::Instant::now);
    for x in 0..config.width {
        copy_strided_column_to_line(plane, config.stride, x, &mut read[..config.height]);
        transform_line_inverse_from_read_bounded(
            config.height,
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &read[..config.height],
            &mut line[..config.height],
        );
        write_line_to_strided_column(plane, config.stride, x, &line[..config.height]);
    }
    #[cfg(feature = "std")]
    if let Some(vertical_started) = vertical_started {
        timings.vertical_ns = vertical_started.elapsed().as_nanos();
    }

    validate_samples_in_range(plane, config)?;
    Ok(timings)
}

const IRREVERSIBLE_97_ALPHA: f32 = -1.586_134_3;
const IRREVERSIBLE_97_BETA: f32 = -0.052_980_118;
const IRREVERSIBLE_97_GAMMA: f32 = 0.882_911_1;
const IRREVERSIBLE_97_DELTA: f32 = 0.443_506_87;
const IRREVERSIBLE_97_K: f32 = 1.230_174_1;
const IRREVERSIBLE_97_INV_K: f32 = 0.812_893_1;

/// Apply an allocation-free in-place two-dimensional forward irreversible 9/7
/// transform to level-shifted `f32` samples.
///
/// Coefficients are stored low-pass first along each transformed axis. Callers
/// apply one level at a time from the full tile-component towards its lowest
/// resolution.
pub fn forward_irreversible_9_7(
    plane: &mut [f32],
    config: Irreversible97Config,
    scratch: &mut [f32],
) -> Result<(), TransformError> {
    validate_irreversible_config(plane, config, scratch)?;
    let max_axis = max_usize(config.width, config.height);
    let (line, work) = scratch.split_at_mut(max_axis);

    for x in 0..config.width {
        copy_strided_f32_column_to_line(plane, config.stride, x, &mut line[..config.height]);
        forward_irreversible_9_7_line(
            &mut line[..config.height],
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &mut work[..config.height],
        );
        write_f32_line_to_strided_column(plane, config.stride, x, &line[..config.height]);
    }

    for y in 0..config.height {
        let start = y * config.stride;
        forward_irreversible_9_7_line(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut work[..config.width],
        );
    }

    validate_finite_irreversible_samples(plane, config)
}

/// Apply an allocation-free in-place two-dimensional inverse irreversible 9/7
/// transform to dequantized `f32` coefficients.
///
/// Coefficients use the same low-first rectangular subband layout as the
/// reversible transform. Horizontal reconstruction precedes vertical
/// reconstruction, matching JPEG 2000 synthesis order. Callers apply one
/// level at a time from the lowest retained resolution upward.
pub fn inverse_irreversible_9_7(
    plane: &mut [f32],
    config: Irreversible97Config,
    scratch: &mut [f32],
) -> Result<(), TransformError> {
    inverse_irreversible_9_7_with_stage_timings(plane, config, scratch, false).map(|_| ())
}

/// Profiled form of [`inverse_irreversible_9_7`].
///
/// Timing is opt-in and leaves the non-profiled path free of clock reads.
#[cfg_attr(not(feature = "std"), allow(unused_mut, unused_variables))]
pub fn inverse_irreversible_9_7_with_stage_timings(
    plane: &mut [f32],
    config: Irreversible97Config,
    scratch: &mut [f32],
    collect_stage_timings: bool,
) -> Result<WindowSynthesisStageTimings, TransformError> {
    #[cfg(feature = "std")]
    let preparation_started = collect_stage_timings.then(std::time::Instant::now);
    validate_irreversible_config(plane, config, scratch)?;
    let max_axis = max_usize(config.width, config.height);
    let (line, work) = scratch.split_at_mut(max_axis);
    let mut timings = WindowSynthesisStageTimings::default();
    #[cfg(feature = "std")]
    if let Some(preparation_started) = preparation_started {
        timings.level_preparation_ns = preparation_started.elapsed().as_nanos();
    }

    #[cfg(feature = "std")]
    let horizontal_started = collect_stage_timings.then(std::time::Instant::now);
    for y in 0..config.height {
        let start = y * config.stride;
        inverse_irreversible_9_7_line(
            &mut plane[start..start + config.width],
            config.edges.horizontal_low_samples,
            config.edges.horizontal_first,
            &mut work[..config.width],
        );
    }
    #[cfg(feature = "std")]
    if let Some(horizontal_started) = horizontal_started {
        timings.horizontal_ns = horizontal_started.elapsed().as_nanos();
    }

    #[cfg(feature = "std")]
    let vertical_started = collect_stage_timings.then(std::time::Instant::now);
    for x in 0..config.width {
        copy_strided_f32_column_to_line(plane, config.stride, x, &mut line[..config.height]);
        inverse_irreversible_9_7_line(
            &mut line[..config.height],
            config.edges.vertical_low_samples,
            config.edges.vertical_first,
            &mut work[..config.height],
        );
        write_f32_line_to_strided_column(plane, config.stride, x, &line[..config.height]);
    }
    #[cfg(feature = "std")]
    if let Some(vertical_started) = vertical_started {
        timings.vertical_ns = vertical_started.elapsed().as_nanos();
    }

    validate_finite_irreversible_samples(plane, config)?;
    Ok(timings)
}

/// Apply the inverse JPEG 2000 irreversible component transform in place.
///
/// Inputs are `Y`, `Cb`, and `Cr` transform-component planes. They are replaced
/// by level-shifted `R`, `G`, and `B` planes. Rounding and clamping belong at
/// the sample-output boundary.
pub fn inverse_irreversible_color_transform(
    y_plane: &mut [f32],
    cb_plane: &mut [f32],
    cr_plane: &mut [f32],
) -> Result<(), TransformError> {
    if y_plane.len() != cb_plane.len() || y_plane.len() != cr_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }
    for index in 0..y_plane.len() {
        let y = y_plane[index];
        let cb = cb_plane[index];
        let cr = cr_plane[index];
        let red = y + 1.402 * cr;
        let green = y - 0.344_13 * cb - 0.714_14 * cr;
        let blue = y + 1.772 * cb;
        if !red.is_finite() || !green.is_finite() || !blue.is_finite() {
            return Err(TransformError::NonFiniteSample { index });
        }
        y_plane[index] = red;
        cb_plane[index] = green;
        cr_plane[index] = blue;
    }
    Ok(())
}

/// Apply the forward JPEG 2000 irreversible component transform in place.
///
/// Inputs are level-shifted `R`, `G`, and `B` planes. They are replaced by
/// `Y`, `Cb`, and `Cr` transform-component planes.
pub fn forward_irreversible_color_transform(
    r_plane: &mut [f32],
    g_plane: &mut [f32],
    b_plane: &mut [f32],
) -> Result<(), TransformError> {
    if r_plane.len() != g_plane.len() || r_plane.len() != b_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }
    for index in 0..r_plane.len() {
        let red = r_plane[index];
        let green = g_plane[index];
        let blue = b_plane[index];
        let y = 0.299 * red + 0.587 * green + 0.114 * blue;
        let cb = -0.168_75 * red - 0.331_26 * green + 0.5 * blue;
        let cr = 0.5 * red - 0.418_69 * green - 0.081_31 * blue;
        if !y.is_finite() || !cb.is_finite() || !cr.is_finite() {
            return Err(TransformError::NonFiniteSample { index });
        }
        r_plane[index] = y;
        g_plane[index] = cb;
        b_plane[index] = cr;
    }
    Ok(())
}

fn validate_irreversible_config(
    plane: &[f32],
    config: Irreversible97Config,
    scratch: &[f32],
) -> Result<(), TransformError> {
    if config.width == 0 || config.height == 0 {
        return Err(TransformError::EmptyPlane);
    }
    if config.stride < config.width {
        return Err(TransformError::StrideTooSmall);
    }
    let required_len = (config.height - 1)
        .checked_mul(config.stride)
        .and_then(|offset| offset.checked_add(config.width))
        .ok_or(TransformError::PlaneTooSmall)?;
    if plane.len() < required_len {
        return Err(TransformError::PlaneTooSmall);
    }
    if scratch.len() < config.scratch_len() {
        return Err(TransformError::ScratchTooSmall {
            required: config.scratch_len(),
            actual: scratch.len(),
        });
    }
    validate_axis_edges(
        config.width,
        config.edges.horizontal_low_samples,
        config.edges.horizontal_first,
    )?;
    validate_axis_edges(
        config.height,
        config.edges.vertical_low_samples,
        config.edges.vertical_first,
    )?;
    validate_finite_irreversible_samples(plane, config)
}

fn validate_finite_irreversible_samples(
    plane: &[f32],
    config: Irreversible97Config,
) -> Result<(), TransformError> {
    for y in 0..config.height {
        for x in 0..config.width {
            let index = y * config.stride + x;
            if !plane[index].is_finite() {
                return Err(TransformError::NonFiniteSample { index });
            }
        }
    }
    Ok(())
}

fn inverse_irreversible_9_7_line(
    line: &mut [f32],
    low_samples: usize,
    first: TransformBand,
    work: &mut [f32],
) {
    if line.len() <= 1 {
        return;
    }
    work[..line.len()].copy_from_slice(line);
    inverse_irreversible_9_7_line_from_read(&mut work[..line.len()], line, low_samples, first);
}

fn forward_irreversible_9_7_line(
    line: &mut [f32],
    low_samples: usize,
    first: TransformBand,
    work: &mut [f32],
) {
    if line.len() <= 1 {
        return;
    }
    let high_samples = line.len() - low_samples;
    for index in 0..low_samples {
        work[index] = match first {
            TransformBand::Low => line[2 * index],
            TransformBand::High => line[2 * index + 1],
        };
    }
    for index in 0..high_samples {
        work[low_samples + index] = match first {
            TransformBand::Low => line[2 * index + 1],
            TransformBand::High => line[2 * index],
        };
    }
    let (low, high) = work[..line.len()].split_at_mut(low_samples);
    update_irreversible_high(low, high, first, IRREVERSIBLE_97_ALPHA);
    update_irreversible_low(low, high, first, IRREVERSIBLE_97_BETA);
    update_irreversible_high(low, high, first, IRREVERSIBLE_97_GAMMA);
    update_irreversible_low(low, high, first, IRREVERSIBLE_97_DELTA);
    for value in low.iter_mut() {
        *value *= IRREVERSIBLE_97_INV_K;
    }
    for value in high.iter_mut() {
        *value *= IRREVERSIBLE_97_K;
    }
    line.copy_from_slice(&work[..line.len()]);
}

fn inverse_irreversible_9_7_line_from_read(
    read: &mut [f32],
    line: &mut [f32],
    low_samples: usize,
    first: TransformBand,
) {
    if line.len() <= 1 {
        if let (Some(output), Some(input)) = (line.first_mut(), read.first()) {
            *output = *input;
        }
        return;
    }
    let high_samples = line.len() - low_samples;
    let (low, high) = read[..line.len()].split_at_mut(low_samples);
    for value in low.iter_mut() {
        *value *= IRREVERSIBLE_97_K;
    }
    for value in high.iter_mut() {
        *value *= IRREVERSIBLE_97_INV_K;
    }
    update_irreversible_low(low, high, first, -IRREVERSIBLE_97_DELTA);
    update_irreversible_high(low, high, first, -IRREVERSIBLE_97_GAMMA);
    update_irreversible_low(low, high, first, -IRREVERSIBLE_97_BETA);
    update_irreversible_high(low, high, first, -IRREVERSIBLE_97_ALPHA);

    match first {
        TransformBand::Low => {
            for index in 0..low_samples {
                line[2 * index] = low[index];
                if index < high_samples {
                    line[2 * index + 1] = high[index];
                }
            }
        }
        TransformBand::High => {
            for index in 0..high_samples {
                line[2 * index] = high[index];
                if index < low_samples {
                    line[2 * index + 1] = low[index];
                }
            }
        }
    }
}

fn update_irreversible_low(low: &mut [f32], high: &[f32], first: TransformBand, factor: f32) {
    if high.is_empty() {
        return;
    }
    let last_high = high.len() - 1;
    for index in 0..low.len() {
        let (left, right) = match first {
            TransformBand::Low => (
                high[if index == 0 { 0 } else { index - 1 }],
                high[index.min(last_high)],
            ),
            TransformBand::High => (high[index.min(last_high)], high[(index + 1).min(last_high)]),
        };
        low[index] += factor * (left + right);
    }
}

fn update_irreversible_high(low: &[f32], high: &mut [f32], first: TransformBand, factor: f32) {
    if low.is_empty() {
        return;
    }
    let last_low = low.len() - 1;
    for index in 0..high.len() {
        let (left, right) = match first {
            TransformBand::Low => (low[index.min(last_low)], low[(index + 1).min(last_low)]),
            TransformBand::High => (
                low[if index == 0 { 0 } else { index - 1 }],
                low[index.min(last_low)],
            ),
        };
        high[index] += factor * (left + right);
    }
}

fn copy_strided_f32_column_to_line(plane: &[f32], stride: usize, x: usize, line: &mut [f32]) {
    let mut offset = x;
    for value in line {
        *value = plane[offset];
        offset += stride;
    }
}

fn write_f32_line_to_strided_column(plane: &mut [f32], stride: usize, x: usize, line: &[f32]) {
    let mut offset = x;
    for value in line {
        plane[offset] = *value;
        offset += stride;
    }
}

fn power_of_two_f32(power: i16) -> Option<f32> {
    if !(-126..=127).contains(&power) {
        return None;
    }
    let biased = u32::try_from(power + 127).ok()?;
    Some(f32::from_bits(biased << 23))
}

const fn max_usize(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

const fn low_sample_count(len: usize, first: TransformBand) -> usize {
    match first {
        TransformBand::Low => len.div_ceil(2),
        TransformBand::High => len / 2,
    }
}

fn copy_strided_column_to_line(plane: &[i32], stride: usize, x: usize, line: &mut [i32]) {
    let mut offset = x;
    for value in line {
        *value = plane[offset];
        offset += stride;
    }
}

fn write_line_to_strided_column(plane: &mut [i32], stride: usize, x: usize, line: &[i32]) {
    let mut offset = x;
    for value in line {
        plane[offset] = *value;
        offset += stride;
    }
}

/// Apply the inverse JPEG 2000 reversible color transform in place.
///
/// Inputs are the three reconstructed transform-component planes in `Y`, `Db`,
/// `Dr` order. On success they are replaced with `R`, `G`, `B` planes in that
/// same slice order. Public sample level shifting belongs outside this function.
pub fn inverse_reversible_color_transform(
    y_plane: &mut [i32],
    db_plane: &mut [i32],
    dr_plane: &mut [i32],
) -> Result<(), TransformError> {
    if y_plane.len() != db_plane.len() || y_plane.len() != dr_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }

    for index in 0..y_plane.len() {
        let y = y_plane[index];
        let db = db_plane[index];
        let dr = dr_plane[index];
        let chroma_sum = db
            .checked_add(dr)
            .ok_or(TransformError::ArithmeticOverflow)?;
        let green = y
            .checked_sub(chroma_sum.div_euclid(4))
            .ok_or(TransformError::ArithmeticOverflow)?;
        let red = dr
            .checked_add(green)
            .ok_or(TransformError::ArithmeticOverflow)?;
        let blue = db
            .checked_add(green)
            .ok_or(TransformError::ArithmeticOverflow)?;
        y_plane[index] = red;
        db_plane[index] = green;
        dr_plane[index] = blue;
    }

    Ok(())
}

/// Apply the inverse JPEG 2000 reversible color transform for prevalidated
/// bounded data.
///
/// This fast path preserves [`inverse_reversible_color_transform`]'s component
/// layout but skips per-sample overflow checks. Callers must prove the
/// transform components are bounded enough that reconstructed samples cannot
/// overflow `i32`.
pub fn inverse_reversible_color_transform_bounded(
    y_plane: &mut [i32],
    db_plane: &mut [i32],
    dr_plane: &mut [i32],
) -> Result<(), TransformError> {
    if y_plane.len() != db_plane.len() || y_plane.len() != dr_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }

    for index in 0..y_plane.len() {
        let y = y_plane[index];
        let db = db_plane[index];
        let dr = dr_plane[index];
        let green = y.wrapping_sub(db.wrapping_add(dr) >> 2);
        y_plane[index] = dr.wrapping_add(green);
        db_plane[index] = green;
        dr_plane[index] = db.wrapping_add(green);
    }

    Ok(())
}

/// Apply the forward JPEG 2000 reversible color transform in place.
///
/// Inputs are signed, level-shifted `R`, `G`, `B` planes. On success they are
/// replaced with transform-component planes in `Y`, `Db`, `Dr` order. Public
/// sample level shifting belongs outside this function.
pub fn forward_reversible_color_transform(
    r_plane: &mut [i32],
    g_plane: &mut [i32],
    b_plane: &mut [i32],
) -> Result<(), TransformError> {
    if r_plane.len() != g_plane.len() || r_plane.len() != b_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }

    for index in 0..r_plane.len() {
        let red = r_plane[index];
        let green = g_plane[index];
        let blue = b_plane[index];
        let doubled_green = green
            .checked_mul(2)
            .ok_or(TransformError::ArithmeticOverflow)?;
        let luma_sum = red
            .checked_add(doubled_green)
            .and_then(|value| value.checked_add(blue))
            .ok_or(TransformError::ArithmeticOverflow)?;
        let y = luma_sum.div_euclid(4);
        let db = blue
            .checked_sub(green)
            .ok_or(TransformError::ArithmeticOverflow)?;
        let dr = red
            .checked_sub(green)
            .ok_or(TransformError::ArithmeticOverflow)?;
        r_plane[index] = y;
        g_plane[index] = db;
        b_plane[index] = dr;
    }

    Ok(())
}

/// Apply the forward JPEG 2000 reversible color transform for prevalidated
/// bounded data.
///
/// This fast path preserves [`forward_reversible_color_transform`]'s component
/// layout but skips per-sample overflow checks. Callers must prove the input
/// component samples are bounded enough that transform components cannot
/// overflow `i32`.
pub fn forward_reversible_color_transform_bounded(
    r_plane: &mut [i32],
    g_plane: &mut [i32],
    b_plane: &mut [i32],
) -> Result<(), TransformError> {
    if r_plane.len() != g_plane.len() || r_plane.len() != b_plane.len() {
        return Err(TransformError::ComponentLengthMismatch);
    }

    for index in 0..r_plane.len() {
        let red = r_plane[index];
        let green = g_plane[index];
        let blue = b_plane[index];
        r_plane[index] = red.wrapping_add(green.wrapping_shl(1)).wrapping_add(blue) >> 2;
        g_plane[index] = blue.wrapping_sub(green);
        b_plane[index] = red.wrapping_sub(green);
    }

    Ok(())
}

fn validate_config(
    plane: &[i32],
    config: Reversible53Config,
    scratch: &[i32],
) -> Result<(), TransformError> {
    if config.width == 0 || config.height == 0 {
        return Err(TransformError::EmptyPlane);
    }
    if config.stride < config.width {
        return Err(TransformError::StrideTooSmall);
    }
    let required_len = (config.height - 1)
        .checked_mul(config.stride)
        .and_then(|offset| offset.checked_add(config.width))
        .ok_or(TransformError::PlaneTooSmall)?;
    if plane.len() < required_len {
        return Err(TransformError::PlaneTooSmall);
    }
    let required_scratch = config.scratch_len();
    if scratch.len() < required_scratch {
        return Err(TransformError::ScratchTooSmall {
            required: required_scratch,
            actual: scratch.len(),
        });
    }
    validate_axis_edges(
        config.width,
        config.edges.horizontal_low_samples,
        config.edges.horizontal_first,
    )?;
    validate_axis_edges(
        config.height,
        config.edges.vertical_low_samples,
        config.edges.vertical_first,
    )?;
    config.sample_range.bounds()?;
    Ok(())
}

fn validate_axis_edges(
    len: usize,
    low_samples: usize,
    first: TransformBand,
) -> Result<(), TransformError> {
    if low_samples != low_sample_count(len, first) {
        return Err(TransformError::InvalidEdges);
    }
    Ok(())
}

fn validate_samples_in_range(
    plane: &[i32],
    config: Reversible53Config,
) -> Result<(), TransformError> {
    if config.sample_range.signed && config.sample_range.precision_bits == 32 {
        return Ok(());
    }

    for y in 0..config.height {
        let row_offset = y * config.stride;
        for x in 0..config.width {
            let index = row_offset + x;
            let value = plane[index];
            if !config.sample_range.contains(value)? {
                return Err(TransformError::SampleOutOfRange { index, value });
            }
        }
    }
    Ok(())
}

fn transform_line_forward(
    line: &mut [i32],
    low_samples: usize,
    first: TransformBand,
    read: &mut [i32],
    coeffs: &mut [i32],
) -> Result<(), TransformError> {
    let len = line.len();
    if len == 1 {
        return Ok(());
    }
    read[..len].copy_from_slice(line);

    match first {
        TransformBand::Low => {
            transform_line_forward_first_low(len, low_samples, read, coeffs, line)?
        }
        TransformBand::High => {
            transform_line_forward_first_high(len, low_samples, read, coeffs, line)?
        }
    }

    Ok(())
}

fn transform_line_forward_first_low(
    len: usize,
    low_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
    line: &mut [i32],
) -> Result<(), TransformError> {
    let high_samples = len - low_samples;
    for high in 0..high_samples {
        let pos = 2 * high + 1;
        let left = read[2 * high];
        let right_low = if high + 1 < low_samples {
            high + 1
        } else {
            high
        };
        let right = read[2 * right_low];
        coeffs[low_samples + high] = read[pos]
            .checked_sub(floor_div2_sum(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    for low in 0..low_samples {
        let pos = 2 * low;
        let left = if low == 0 {
            coeffs[low_samples]
        } else {
            coeffs[low_samples + low - 1]
        };
        let right = if low < high_samples {
            coeffs[low_samples + low]
        } else {
            coeffs[low_samples + high_samples - 1]
        };
        coeffs[low] = read[pos]
            .checked_add(floor_div4_sum_plus_two(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    line.copy_from_slice(&coeffs[..len]);
    Ok(())
}

fn transform_line_forward_first_high(
    len: usize,
    low_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
    line: &mut [i32],
) -> Result<(), TransformError> {
    let high_samples = len - low_samples;
    for high in 0..high_samples {
        let pos = 2 * high;
        let left = if high == 0 {
            read[1]
        } else {
            read[2 * high - 1]
        };
        let right = if high < low_samples {
            read[2 * high + 1]
        } else {
            left
        };
        coeffs[low_samples + high] = read[pos]
            .checked_sub(floor_div2_sum(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    for low in 0..low_samples {
        let pos = 2 * low + 1;
        let left = coeffs[low_samples + low];
        let right = if low + 1 < high_samples {
            coeffs[low_samples + low + 1]
        } else {
            left
        };
        coeffs[low] = read[pos]
            .checked_add(floor_div4_sum_plus_two(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    line.copy_from_slice(&coeffs[..len]);
    Ok(())
}

fn transform_line_inverse(
    line: &mut [i32],
    low_samples: usize,
    first: TransformBand,
    read: &mut [i32],
) -> Result<(), TransformError> {
    let len = line.len();
    if len == 1 {
        return Ok(());
    }
    read[..len].copy_from_slice(line);

    match first {
        TransformBand::Low => transform_line_inverse_first_low(len, low_samples, read, line)?,
        TransformBand::High => transform_line_inverse_first_high(len, low_samples, read, line)?,
    }

    Ok(())
}

fn transform_line_inverse_first_low(
    len: usize,
    low_samples: usize,
    read: &[i32],
    line: &mut [i32],
) -> Result<(), TransformError> {
    let high_samples = len - low_samples;
    for low in 0..low_samples {
        let left = if low == 0 {
            read[low_samples]
        } else {
            read[low_samples + low - 1]
        };
        let right = if low < high_samples {
            read[low_samples + low]
        } else {
            read[low_samples + high_samples - 1]
        };
        let original = read[low]
            .checked_sub(floor_div4_sum_plus_two(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
        line[2 * low] = original;
    }

    for high in 0..high_samples {
        let left = line[2 * high];
        let right_low = if high + 1 < low_samples {
            high + 1
        } else {
            high
        };
        let right = line[2 * right_low];
        line[2 * high + 1] = read[low_samples + high]
            .checked_add(floor_div2_sum(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    Ok(())
}

fn transform_line_inverse_first_high(
    len: usize,
    low_samples: usize,
    read: &[i32],
    line: &mut [i32],
) -> Result<(), TransformError> {
    let high_samples = len - low_samples;
    for low in 0..low_samples {
        let left = read[low_samples + low];
        let right = if low + 1 < high_samples {
            read[low_samples + low + 1]
        } else {
            left
        };
        let original = read[low]
            .checked_sub(floor_div4_sum_plus_two(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
        line[2 * low + 1] = original;
    }

    for high in 0..high_samples {
        let left = if high == 0 {
            line[1]
        } else {
            line[2 * (high - 1) + 1]
        };
        let right = if high < low_samples {
            line[2 * high + 1]
        } else {
            left
        };
        line[2 * high] = read[low_samples + high]
            .checked_add(floor_div2_sum(left, right)?)
            .ok_or(TransformError::ArithmeticOverflow)?;
    }

    Ok(())
}

fn floor_div2_sum(a: i32, b: i32) -> Result<i32, TransformError> {
    if let Some(sum) = a.checked_add(b) {
        return Ok(sum >> 1);
    }
    let sum = i64::from(a) + i64::from(b);
    i32::try_from(sum.div_euclid(2)).map_err(|_| TransformError::ArithmeticOverflow)
}

fn floor_div4_sum_plus_two(a: i32, b: i32) -> Result<i32, TransformError> {
    if let Some(sum) = a.checked_add(b).and_then(|sum| sum.checked_add(2)) {
        return Ok(sum >> 2);
    }
    let sum = i64::from(a) + i64::from(b) + 2;
    i32::try_from(sum.div_euclid(4)).map_err(|_| TransformError::ArithmeticOverflow)
}

fn transform_line_forward_bounded(
    line: &mut [i32],
    low_samples: usize,
    first: TransformBand,
    coeffs: &mut [i32],
) {
    let len = line.len();
    if len == 1 {
        return;
    }

    match first {
        TransformBand::Low => {
            transform_line_forward_first_low_bounded(len, low_samples, line, coeffs)
        }
        TransformBand::High => {
            transform_line_forward_first_high_bounded(len, low_samples, line, coeffs)
        }
    }
    line.copy_from_slice(&coeffs[..len]);
}

#[allow(clippy::too_many_arguments)]
fn transform_line_forward_from_read_to_strided_bounded(
    len: usize,
    low_samples: usize,
    first: TransformBand,
    read: &[i32],
    coeffs: &mut [i32],
    plane: &mut [i32],
    stride: usize,
    x: usize,
) {
    if len == 1 {
        plane[x] = read[0];
        return;
    }

    match first {
        TransformBand::Low => {
            transform_line_forward_first_low_bounded(len, low_samples, read, coeffs)
        }
        TransformBand::High => {
            transform_line_forward_first_high_bounded(len, low_samples, read, coeffs)
        }
    }
    write_line_to_strided_column(plane, stride, x, &coeffs[..len]);
}

fn transform_line_forward_first_low_bounded(
    len: usize,
    low_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
) {
    let high_samples = len - low_samples;
    if low_samples == high_samples {
        transform_line_forward_first_low_even_bounded(low_samples, read, coeffs);
        return;
    }

    for high in 0..high_samples {
        let pos = 2 * high + 1;
        let left = read[2 * high];
        let right_low = if high + 1 < low_samples {
            high + 1
        } else {
            high
        };
        let right = read[2 * right_low];
        coeffs[low_samples + high] = read[pos].wrapping_sub(floor_div2_sum_bounded(left, right));
    }

    for low in 0..low_samples {
        let pos = 2 * low;
        let left = if low == 0 {
            coeffs[low_samples]
        } else {
            coeffs[low_samples + low - 1]
        };
        let right = if low < high_samples {
            coeffs[low_samples + low]
        } else {
            coeffs[low_samples + high_samples - 1]
        };
        coeffs[low] = read[pos].wrapping_add(floor_div4_sum_plus_two_bounded(left, right));
    }
}

fn transform_line_forward_first_low_even_bounded(
    band_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
) {
    for high in 0..band_samples.saturating_sub(1) {
        let pos = 2 * high + 1;
        let left = read[2 * high];
        let right = read[2 * high + 2];
        coeffs[band_samples + high] = read[pos].wrapping_sub(floor_div2_sum_bounded(left, right));
    }
    let last = band_samples - 1;
    let pos = 2 * last + 1;
    let left = read[2 * last];
    coeffs[band_samples + last] = read[pos].wrapping_sub(floor_div2_sum_bounded(left, left));

    let first_high = coeffs[band_samples];
    coeffs[0] = read[0].wrapping_add(floor_div4_sum_plus_two_bounded(first_high, first_high));
    for low in 1..band_samples {
        let pos = 2 * low;
        let left = coeffs[band_samples + low - 1];
        let right = coeffs[band_samples + low];
        coeffs[low] = read[pos].wrapping_add(floor_div4_sum_plus_two_bounded(left, right));
    }
}

fn transform_line_forward_first_high_bounded(
    len: usize,
    low_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
) {
    let high_samples = len - low_samples;
    if low_samples == high_samples {
        transform_line_forward_first_high_even_bounded(low_samples, read, coeffs);
        return;
    }

    for high in 0..high_samples {
        let pos = 2 * high;
        let left = if high == 0 {
            read[1]
        } else {
            read[2 * high - 1]
        };
        let right = if high < low_samples {
            read[2 * high + 1]
        } else {
            left
        };
        coeffs[low_samples + high] = read[pos].wrapping_sub(floor_div2_sum_bounded(left, right));
    }

    for low in 0..low_samples {
        let pos = 2 * low + 1;
        let left = coeffs[low_samples + low];
        let right = if low + 1 < high_samples {
            coeffs[low_samples + low + 1]
        } else {
            left
        };
        coeffs[low] = read[pos].wrapping_add(floor_div4_sum_plus_two_bounded(left, right));
    }
}

fn transform_line_forward_first_high_even_bounded(
    band_samples: usize,
    read: &[i32],
    coeffs: &mut [i32],
) {
    let first_low = read[1];
    coeffs[band_samples] = read[0].wrapping_sub(floor_div2_sum_bounded(first_low, first_low));
    for high in 1..band_samples {
        let pos = 2 * high;
        let left = read[pos - 1];
        let right = read[pos + 1];
        coeffs[band_samples + high] = read[pos].wrapping_sub(floor_div2_sum_bounded(left, right));
    }

    for low in 0..band_samples.saturating_sub(1) {
        let pos = 2 * low + 1;
        let left = coeffs[band_samples + low];
        let right = coeffs[band_samples + low + 1];
        coeffs[low] = read[pos].wrapping_add(floor_div4_sum_plus_two_bounded(left, right));
    }
    let last = band_samples - 1;
    let pos = 2 * last + 1;
    let last_high = coeffs[band_samples + last];
    coeffs[last] = read[pos].wrapping_add(floor_div4_sum_plus_two_bounded(last_high, last_high));
}

fn transform_line_inverse_bounded(
    line: &mut [i32],
    low_samples: usize,
    first: TransformBand,
    read: &mut [i32],
) {
    let len = line.len();
    if len == 1 {
        return;
    }
    read[..len].copy_from_slice(line);

    match first {
        TransformBand::Low => {
            transform_line_inverse_first_low_bounded(len, low_samples, read, line)
        }
        TransformBand::High => {
            transform_line_inverse_first_high_bounded(len, low_samples, read, line)
        }
    }
}

fn transform_line_inverse_first_low_bounded(
    len: usize,
    low_samples: usize,
    read: &[i32],
    line: &mut [i32],
) {
    let high_samples = len - low_samples;
    if low_samples == high_samples {
        transform_line_inverse_first_low_even_bounded(low_samples, read, line);
        return;
    }

    for low in 0..low_samples {
        let left = if low == 0 {
            read[low_samples]
        } else {
            read[low_samples + low - 1]
        };
        let right = if low < high_samples {
            read[low_samples + low]
        } else {
            read[low_samples + high_samples - 1]
        };
        line[2 * low] = read[low].wrapping_sub(floor_div4_sum_plus_two_bounded(left, right));
    }

    for high in 0..high_samples {
        let left = line[2 * high];
        let right_low = if high + 1 < low_samples {
            high + 1
        } else {
            high
        };
        let right = line[2 * right_low];
        line[2 * high + 1] =
            read[low_samples + high].wrapping_add(floor_div2_sum_bounded(left, right));
    }
}

fn transform_line_inverse_first_low_even_bounded(
    band_samples: usize,
    read: &[i32],
    line: &mut [i32],
) {
    let first_high = read[band_samples];
    line[0] = read[0].wrapping_sub(floor_div4_sum_plus_two_bounded(first_high, first_high));
    for low in 1..band_samples {
        let left = read[band_samples + low - 1];
        let right = read[band_samples + low];
        line[2 * low] = read[low].wrapping_sub(floor_div4_sum_plus_two_bounded(left, right));
    }

    for high in 0..band_samples.saturating_sub(1) {
        let left = line[2 * high];
        let right = line[2 * high + 2];
        line[2 * high + 1] =
            read[band_samples + high].wrapping_add(floor_div2_sum_bounded(left, right));
    }
    let last = band_samples - 1;
    let left = line[2 * last];
    line[2 * last + 1] = read[band_samples + last].wrapping_add(floor_div2_sum_bounded(left, left));
}

fn transform_line_inverse_from_read_bounded(
    len: usize,
    low_samples: usize,
    first: TransformBand,
    read: &[i32],
    line: &mut [i32],
) {
    if len == 1 {
        line[0] = read[0];
        return;
    }

    match first {
        TransformBand::Low => {
            transform_line_inverse_first_low_bounded(len, low_samples, read, line)
        }
        TransformBand::High => {
            transform_line_inverse_first_high_bounded(len, low_samples, read, line)
        }
    }
}

fn transform_line_inverse_first_high_bounded(
    len: usize,
    low_samples: usize,
    read: &[i32],
    line: &mut [i32],
) {
    let high_samples = len - low_samples;
    if low_samples == high_samples {
        transform_line_inverse_first_high_even_bounded(low_samples, read, line);
        return;
    }

    for low in 0..low_samples {
        let left = read[low_samples + low];
        let right = if low + 1 < high_samples {
            read[low_samples + low + 1]
        } else {
            left
        };
        line[2 * low + 1] = read[low].wrapping_sub(floor_div4_sum_plus_two_bounded(left, right));
    }

    for high in 0..high_samples {
        let left = if high == 0 {
            line[1]
        } else {
            line[2 * (high - 1) + 1]
        };
        let right = if high < low_samples {
            line[2 * high + 1]
        } else {
            left
        };
        line[2 * high] = read[low_samples + high].wrapping_add(floor_div2_sum_bounded(left, right));
    }
}

fn transform_line_inverse_first_high_even_bounded(
    band_samples: usize,
    read: &[i32],
    line: &mut [i32],
) {
    for low in 0..band_samples.saturating_sub(1) {
        let left = read[band_samples + low];
        let right = read[band_samples + low + 1];
        line[2 * low + 1] = read[low].wrapping_sub(floor_div4_sum_plus_two_bounded(left, right));
    }
    let last = band_samples - 1;
    let last_high = read[band_samples + last];
    line[2 * last + 1] =
        read[last].wrapping_sub(floor_div4_sum_plus_two_bounded(last_high, last_high));

    let first_low = line[1];
    line[0] = read[band_samples].wrapping_add(floor_div2_sum_bounded(first_low, first_low));
    for high in 1..band_samples {
        let left = line[2 * high - 1];
        let right = line[2 * high + 1];
        line[2 * high] =
            read[band_samples + high].wrapping_add(floor_div2_sum_bounded(left, right));
    }
}

fn floor_div2_sum_bounded(a: i32, b: i32) -> i32 {
    a.wrapping_add(b) >> 1
}

fn floor_div4_sum_plus_two_bounded(a: i32, b: i32) -> i32 {
    a.wrapping_add(b).wrapping_add(2) >> 2
}

#[cfg(test)]
mod irreversible_encode_tests {
    use super::*;

    #[test]
    fn forward_and_inverse_irreversible_9_7_round_trip_odd_plane() {
        let width = 17;
        let height = 13;
        let mut plane = (0..width * height)
            .map(|index| ((index * 37 + index / width * 19) % 511) as f32 - 255.0)
            .collect::<Vec<_>>();
        let source = plane.clone();
        let config = Irreversible97Config {
            width,
            height,
            stride: width,
            edges: Irreversible97Edges::from_tile_origin(0, 0, width, height),
        };
        let mut scratch = vec![0.0; config.scratch_len()];
        forward_irreversible_9_7(&mut plane, config, &mut scratch).unwrap();
        inverse_irreversible_9_7(&mut plane, config, &mut scratch).unwrap();
        for (expected, actual) in source.iter().zip(&plane) {
            assert!((expected - actual).abs() < 0.001, "{expected} != {actual}");
        }
    }
}
