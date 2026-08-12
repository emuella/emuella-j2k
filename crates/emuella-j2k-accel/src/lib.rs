#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unsafe_code)]
//! Private, safe acceleration boundary for codec kernels.

use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::sync::OnceLock;

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
mod openjph_ht_cleanup;

/// Invalid geometry supplied to a checked block-transpose kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransposeError {
    Empty,
    StrideTooSmall,
    SourceTooSmall,
    DestinationTooSmall,
    SizeOverflow,
}

/// Invalid storage supplied to the even, low-first reversible 5/3 kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reversible53Error {
    Empty,
    InvalidGeometry,
    SizeOverflow,
    InputTooSmall,
    OutputTooSmall,
}

/// Invalid storage supplied to direct unsigned-8 component packing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackU8Error {
    LengthMismatch,
    InvalidGeometry,
}

/// Invalid geometry supplied to a disjoint strided output rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputRectError {
    InvalidGeometry,
}

/// Exclusive ownership of one rectangular byte region in padded row-major
/// output.
///
/// Consuming row/column splits are the only way to derive child rectangles,
/// so separate tokens may be sent to parallel jobs without creating aliasing
/// mutable slices. The token exposes one short, non-overlapping row borrow at a
/// time and never exposes stride padding or a bounding slice that overlaps an
/// adjacent rectangle.
pub struct OutputRect<'a> {
    pointer: *mut u8,
    output_len: usize,
    stride: usize,
    first_row: usize,
    first_column: usize,
    width_bytes: usize,
    height: usize,
    marker: PhantomData<&'a mut [u8]>,
}

// SAFETY: construction owns the complete output and every split consumes its
// parent, producing geometrically non-overlapping children. `row_mut` exposes
// only this token's columns in one owned row.
unsafe impl Send for OutputRect<'_> {}

impl<'a> OutputRect<'a> {
    pub fn new(
        output: &'a mut [u8],
        stride: usize,
        width_bytes: usize,
        height: usize,
    ) -> Result<Self, OutputRectError> {
        if width_bytes == 0 || height == 0 || stride < width_bytes {
            return Err(OutputRectError::InvalidGeometry);
        }
        let required = (height - 1)
            .checked_mul(stride)
            .and_then(|offset| offset.checked_add(width_bytes))
            .ok_or(OutputRectError::InvalidGeometry)?;
        if output.len() < required {
            return Err(OutputRectError::InvalidGeometry);
        }
        Ok(Self {
            pointer: output.as_mut_ptr(),
            output_len: output.len(),
            stride,
            first_row: 0,
            first_column: 0,
            width_bytes,
            height,
            marker: PhantomData,
        })
    }

    pub fn width_bytes(&self) -> usize {
        self.width_bytes
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn split_rows(self, top_rows: usize) -> Result<(Self, Self), OutputRectError> {
        if top_rows == 0 || top_rows >= self.height {
            return Err(OutputRectError::InvalidGeometry);
        }
        let bottom_first_row = self
            .first_row
            .checked_add(top_rows)
            .ok_or(OutputRectError::InvalidGeometry)?;
        let top = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            first_row: self.first_row,
            first_column: self.first_column,
            width_bytes: self.width_bytes,
            height: top_rows,
            marker: PhantomData,
        };
        let bottom = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            first_row: bottom_first_row,
            first_column: self.first_column,
            width_bytes: self.width_bytes,
            height: self.height - top_rows,
            marker: PhantomData,
        };
        Ok((top, bottom))
    }

    pub fn split_columns(self, left_bytes: usize) -> Result<(Self, Self), OutputRectError> {
        if left_bytes == 0 || left_bytes >= self.width_bytes {
            return Err(OutputRectError::InvalidGeometry);
        }
        let right_first_column = self
            .first_column
            .checked_add(left_bytes)
            .ok_or(OutputRectError::InvalidGeometry)?;
        let left = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            first_row: self.first_row,
            first_column: self.first_column,
            width_bytes: left_bytes,
            height: self.height,
            marker: PhantomData,
        };
        let right = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            first_row: self.first_row,
            first_column: right_first_column,
            width_bytes: self.width_bytes - left_bytes,
            height: self.height,
            marker: PhantomData,
        };
        Ok((left, right))
    }

    pub fn row_mut(&mut self, row: usize) -> Result<&mut [u8], OutputRectError> {
        if row >= self.height {
            return Err(OutputRectError::InvalidGeometry);
        }
        let offset = self
            .first_row
            .checked_add(row)
            .and_then(|row| row.checked_mul(self.stride))
            .and_then(|offset| offset.checked_add(self.first_column))
            .ok_or(OutputRectError::InvalidGeometry)?;
        let end = offset
            .checked_add(self.width_bytes)
            .ok_or(OutputRectError::InvalidGeometry)?;
        if end > self.output_len {
            return Err(OutputRectError::InvalidGeometry);
        }
        // SAFETY: construction validates the root allocation and consuming
        // splits prove this token exclusively owns the returned row interval.
        Ok(unsafe { core::slice::from_raw_parts_mut(self.pointer.add(offset), self.width_bytes) })
    }
}

/// Exclusive ownership of a vertical byte-column stripe in padded row-major
/// output. Tokens can only be created by consuming or splitting an exclusive
/// output borrow, so parallel writers remain disjoint without exposing raw
/// pointers outside this acceleration boundary.
pub struct U8OutputColumns<'a> {
    pointer: *mut u8,
    output_len: usize,
    stride: usize,
    height: usize,
    first_column: usize,
    columns: usize,
    marker: PhantomData<&'a mut [u8]>,
}

// SAFETY: each token exclusively owns its horizontal byte interval in every
// row; splitting consumes the parent and creates non-overlapping children.
unsafe impl Send for U8OutputColumns<'_> {}

impl<'a> U8OutputColumns<'a> {
    pub fn new(
        output: &'a mut [u8],
        stride: usize,
        width: usize,
        height: usize,
    ) -> Result<Self, PackU8Error> {
        if width == 0 || height == 0 || stride < width {
            return Err(PackU8Error::InvalidGeometry);
        }
        let required = (height - 1)
            .checked_mul(stride)
            .and_then(|offset| offset.checked_add(width))
            .ok_or(PackU8Error::InvalidGeometry)?;
        if output.len() < required {
            return Err(PackU8Error::InvalidGeometry);
        }
        Ok(Self {
            pointer: output.as_mut_ptr(),
            output_len: output.len(),
            stride,
            height,
            first_column: 0,
            columns: width,
            marker: PhantomData,
        })
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn split_at(self, left_columns: usize) -> Result<(Self, Self), PackU8Error> {
        if left_columns == 0 || left_columns >= self.columns {
            return Err(PackU8Error::InvalidGeometry);
        }
        let right_column = self
            .first_column
            .checked_add(left_columns)
            .ok_or(PackU8Error::InvalidGeometry)?;
        let left = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            height: self.height,
            first_column: self.first_column,
            columns: left_columns,
            marker: PhantomData,
        };
        let right = Self {
            pointer: self.pointer,
            output_len: self.output_len,
            stride: self.stride,
            height: self.height,
            first_column: right_column,
            columns: self.columns - left_columns,
            marker: PhantomData,
        };
        Ok((left, right))
    }

    /// Pack a column-major block into a subrange owned by this token.
    pub fn pack_transposed_block(
        &mut self,
        local_column: usize,
        source: &[i32],
        columns: usize,
    ) -> Result<(), PackU8Error> {
        if columns == 0
            || local_column
                .checked_add(columns)
                .is_none_or(|end| end > self.columns)
            || source.len() < columns.saturating_mul(self.height)
        {
            return Err(PackU8Error::InvalidGeometry);
        }
        let destination_offset = self
            .first_column
            .checked_add(local_column)
            .ok_or(PackU8Error::InvalidGeometry)?;
        // SAFETY: token construction and consuming splits prove exclusive
        // ownership of these columns. Geometry checks cover the source block
        // and the original output length covers every strided destination.
        unsafe {
            match transpose_dispatch().backend {
                #[cfg(all(feature = "simd", target_arch = "x86_64"))]
                TransposeBackend::Avx2 => transpose_pack_i32_unsigned_u8_avx2(
                    source.as_ptr(),
                    self.height,
                    0,
                    0,
                    columns,
                    self.height,
                    self.pointer.add(destination_offset),
                    self.stride,
                ),
                _ => transpose_pack_i32_unsigned_u8_scalar(
                    source.as_ptr(),
                    self.height,
                    0,
                    0,
                    columns,
                    self.height,
                    self.pointer.add(destination_offset),
                    self.stride,
                ),
            }
        }
        Ok(())
    }
}

/// Exclusive ownership of a vertical column interval in an `i32` coefficient
/// plane. Consuming splits prove that parallel vertical lifting jobs mutate
/// disjoint columns even though the rows share one padded allocation.
pub struct I32PlaneColumns<'a> {
    pointer: *mut i32,
    plane_len: usize,
    stride: usize,
    source_width: usize,
    source_height: usize,
    first_column: usize,
    columns: usize,
    marker: PhantomData<&'a mut [i32]>,
}

// SAFETY: construction exclusively borrows the complete plane and consuming
// splits create non-overlapping column intervals in every active row.
unsafe impl Send for I32PlaneColumns<'_> {}

impl<'a> I32PlaneColumns<'a> {
    pub fn new(
        plane: &'a mut [i32],
        stride: usize,
        source_width: usize,
        source_height: usize,
        first_column: usize,
        columns: usize,
    ) -> Result<Self, Reversible53Error> {
        if source_width == 0
            || source_height == 0
            || columns == 0
            || stride < source_width
            || first_column
                .checked_add(columns)
                .is_none_or(|end| end > source_width)
        {
            return Err(Reversible53Error::InvalidGeometry);
        }
        let required = (source_height - 1)
            .checked_mul(stride)
            .and_then(|offset| offset.checked_add(source_width))
            .ok_or(Reversible53Error::SizeOverflow)?;
        if plane.len() < required {
            return Err(Reversible53Error::InputTooSmall);
        }
        Ok(Self {
            pointer: plane.as_mut_ptr(),
            plane_len: plane.len(),
            stride,
            source_width,
            source_height,
            first_column,
            columns,
            marker: PhantomData,
        })
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn split_at(self, left_columns: usize) -> Result<(Self, Self), Reversible53Error> {
        if left_columns == 0 || left_columns >= self.columns {
            return Err(Reversible53Error::InvalidGeometry);
        }
        let right_first_column = self
            .first_column
            .checked_add(left_columns)
            .ok_or(Reversible53Error::SizeOverflow)?;
        let left = Self {
            pointer: self.pointer,
            plane_len: self.plane_len,
            stride: self.stride,
            source_width: self.source_width,
            source_height: self.source_height,
            first_column: self.first_column,
            columns: left_columns,
            marker: PhantomData,
        };
        let right = Self {
            pointer: self.pointer,
            plane_len: self.plane_len,
            stride: self.stride,
            source_width: self.source_width,
            source_height: self.source_height,
            first_column: right_first_column,
            columns: self.columns - left_columns,
            marker: PhantomData,
        };
        Ok((left, right))
    }

    /// Apply final even, low-first reversible vertical lifting to this
    /// token's columns and pack the requested row interval into a matching
    /// exclusive output-column token.
    pub fn inverse_reversible_5_3_even_first_low_to_unsigned_u8(
        &mut self,
        output_y: usize,
        output_height: usize,
        output: &mut U8OutputColumns<'_>,
    ) -> Result<(), Reversible53Error> {
        if self.source_height < 2
            || !self.source_height.is_multiple_of(2)
            || output_height == 0
            || output_y
                .checked_add(output_height)
                .is_none_or(|end| end > self.source_height)
            || output.columns != self.columns
            || output.height != output_height
        {
            return Err(Reversible53Error::InvalidGeometry);
        }
        let output_offset = output.first_column;
        let output_end = (output_height - 1)
            .checked_mul(output.stride)
            .and_then(|offset| offset.checked_add(output_offset))
            .and_then(|offset| offset.checked_add(self.columns))
            .ok_or(Reversible53Error::SizeOverflow)?;
        if output_end > output.output_len {
            return Err(Reversible53Error::OutputTooSmall);
        }
        // SAFETY: this token owns the selected coefficient columns in every
        // active row, and `output` owns a matching non-overlapping byte-column
        // interval. Constructor bounds cover all kernel loads and stores.
        unsafe {
            (reversible_53_vertical_u8_dispatch().kernel)(
                self.pointer,
                self.stride,
                self.source_width,
                self.source_height,
                self.first_column,
                output_y,
                self.columns,
                output_height,
                output.pointer.add(output_offset),
                output.stride,
            );
        }
        Ok(())
    }

    /// Apply final even, low-first reversible vertical lifting to this
    /// token's columns and store the reconstructed rows in a matching
    /// row-major `i32` column token.
    ///
    /// This is used between inverse-DWT levels: consuming splits give each
    /// worker disjoint input and output columns, so the following horizontal
    /// phase can consume row-major low-band rows without a transpose round
    /// trip.
    pub fn inverse_reversible_5_3_even_first_low_to_row_major_i32(
        &mut self,
        output_y: usize,
        output_height: usize,
        output: &mut I32PlaneColumns<'_>,
    ) -> Result<(), Reversible53Error> {
        if self.source_height < 2
            || !self.source_height.is_multiple_of(2)
            || output.source_width != self.source_width
            || output.source_height != output_height
            || output.first_column != self.first_column
            || output.columns != self.columns
            || output_height == 0
            || output_y
                .checked_add(output_height)
                .is_none_or(|end| end > self.source_height)
        {
            return Err(Reversible53Error::InvalidGeometry);
        }
        let output_end = (output_height - 1)
            .checked_mul(output.stride)
            .and_then(|offset| offset.checked_add(output.first_column))
            .and_then(|offset| offset.checked_add(self.columns))
            .ok_or(Reversible53Error::SizeOverflow)?;
        if output_end > output.plane_len {
            return Err(Reversible53Error::OutputTooSmall);
        }
        // SAFETY: both tokens exclusively own matching, non-overlapping
        // column intervals. Constructor bounds cover every active source and
        // destination row touched by the selected kernel.
        unsafe {
            (reversible_53_vertical_i32_dispatch().kernel)(
                self.pointer,
                self.stride,
                self.source_width,
                self.source_height,
                self.first_column,
                output_y,
                self.columns,
                output_height,
                output.pointer.add(output.first_column),
                output.stride,
            );
        }
        Ok(())
    }
}

type PackU8Kernel = unsafe fn(*const i32, *mut u8, usize);

#[derive(Clone, Copy)]
struct PackU8Dispatch {
    kernel: PackU8Kernel,
}

const SCALAR_PACK_U8_DISPATCH: PackU8Dispatch = PackU8Dispatch {
    kernel: pack_unsigned_u8_level_shift_scalar,
};

#[cfg(feature = "std")]
static PACK_U8_DISPATCH: OnceLock<PackU8Dispatch> = OnceLock::new();

#[cfg(feature = "std")]
fn pack_u8_dispatch() -> &'static PackU8Dispatch {
    PACK_U8_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return PackU8Dispatch {
                kernel: pack_unsigned_u8_level_shift_avx2,
            };
        }
        SCALAR_PACK_U8_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn pack_u8_dispatch() -> &'static PackU8Dispatch {
    &SCALAR_PACK_U8_DISPATCH
}

/// Apply the unsigned 8-bit JPEG 2000 level shift, clamp, and pack exactly.
pub fn pack_unsigned_u8_level_shift(
    coefficients: &[i32],
    output: &mut [u8],
) -> Result<(), PackU8Error> {
    if coefficients.len() != output.len() {
        return Err(PackU8Error::LengthMismatch);
    }
    // SAFETY: equal checked lengths cover every load and store and the shared
    // input/mutable output borrows cannot alias.
    unsafe {
        (pack_u8_dispatch().kernel)(coefficients.as_ptr(), output.as_mut_ptr(), output.len());
    }
    Ok(())
}

unsafe fn pack_unsigned_u8_level_shift_scalar(
    coefficients: *const i32,
    output: *mut u8,
    len: usize,
) {
    for index in 0..len {
        let coefficient = unsafe { *coefficients.add(index) };
        unsafe { *output.add(index) = coefficient.saturating_add(128).clamp(0, 255) as u8 };
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn pack_unsigned_u8_level_shift_avx2(coefficients: *const i32, output: *mut u8, len: usize) {
    use core::arch::x86_64::*;

    let minimum = _mm256_set1_epi32(-128);
    let maximum = _mm256_set1_epi32(127);
    let shift = _mm256_set1_epi32(128);
    let mut index = 0;
    while index + 16 <= len {
        let first = unsafe { _mm256_loadu_si256(coefficients.add(index).cast()) };
        let second = unsafe { _mm256_loadu_si256(coefficients.add(index + 8).cast()) };
        let first = _mm256_add_epi32(
            _mm256_min_epi32(_mm256_max_epi32(first, minimum), maximum),
            shift,
        );
        let second = _mm256_add_epi32(
            _mm256_min_epi32(_mm256_max_epi32(second, minimum), maximum),
            shift,
        );
        let packed = _mm256_permute4x64_epi64(_mm256_packus_epi32(first, second), 0xd8);
        let low = _mm256_castsi256_si128(packed);
        let high = _mm256_extracti128_si256(packed, 1);
        let bytes = _mm_packus_epi16(low, high);
        unsafe { _mm_storeu_si128(output.add(index).cast(), bytes) };
        index += 16;
    }
    while index < len {
        let coefficient = unsafe { *coefficients.add(index) };
        unsafe { *output.add(index) = coefficient.saturating_add(128).clamp(0, 255) as u8 };
        index += 1;
    }
}

type Reversible53EvenKernel = unsafe fn(*mut i32, *mut i32, usize);
type Reversible53SplitKernel = unsafe fn(*mut i32, *mut i32, usize);

#[derive(Clone, Copy)]
struct Reversible53Dispatch {
    kernel: Reversible53EvenKernel,
    split_kernel: Reversible53SplitKernel,
}

const SCALAR_REVERSIBLE_53_DISPATCH: Reversible53Dispatch = Reversible53Dispatch {
    kernel: inverse_reversible_5_3_even_first_low_scalar,
    split_kernel: inverse_reversible_5_3_even_first_low_split_scalar,
};

#[cfg(feature = "std")]
static REVERSIBLE_53_DISPATCH: OnceLock<Reversible53Dispatch> = OnceLock::new();

#[cfg(feature = "std")]
fn reversible_53_dispatch() -> &'static Reversible53Dispatch {
    REVERSIBLE_53_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return Reversible53Dispatch {
                kernel: inverse_reversible_5_3_even_first_low_avx2,
                split_kernel: inverse_reversible_5_3_even_first_low_split_avx2,
            };
        }
        #[cfg(all(feature = "simd", target_arch = "aarch64"))]
        if std::arch::is_aarch64_feature_detected!("neon") {
            return Reversible53Dispatch {
                kernel: inverse_reversible_5_3_even_first_low_neon,
                split_kernel: inverse_reversible_5_3_even_first_low_split_scalar,
            };
        }
        SCALAR_REVERSIBLE_53_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn reversible_53_dispatch() -> &'static Reversible53Dispatch {
    &SCALAR_REVERSIBLE_53_DISPATCH
}

/// Reconstruct an even, low-first reversible 5/3 line exactly.
///
/// `read` contains `band_samples` low coefficients followed by the same
/// number of high coefficients and is reusable scratch. `output` receives
/// interleaved spatial samples. The selected architecture kernel preserves
/// Rust wrapping addition/subtraction and signed floor shifts exactly.
pub fn inverse_reversible_5_3_even_first_low(
    read: &mut [i32],
    output: &mut [i32],
    band_samples: usize,
) -> Result<(), Reversible53Error> {
    if band_samples == 0 {
        return Err(Reversible53Error::Empty);
    }
    let len = band_samples
        .checked_mul(2)
        .ok_or(Reversible53Error::SizeOverflow)?;
    if read.len() < len {
        return Err(Reversible53Error::InputTooSmall);
    }
    if output.len() < len {
        return Err(Reversible53Error::OutputTooSmall);
    }
    // SAFETY: checked lengths cover every kernel load/store, and the mutable
    // borrows prove input scratch and output do not alias.
    unsafe {
        (reversible_53_dispatch().kernel)(read.as_mut_ptr(), output.as_mut_ptr(), band_samples);
    }
    Ok(())
}

/// Reconstruct an even, low-first reversible 5/3 line whose low band is in
/// separate storage and whose high band occupies the second half of `output`.
///
/// Both inputs are reusable scratch. The selected kernel updates the two
/// bands and interleaves them into `output` without first copying them into a
/// combined line buffer.
pub fn inverse_reversible_5_3_even_first_low_split(
    low: &mut [i32],
    output: &mut [i32],
    band_samples: usize,
) -> Result<(), Reversible53Error> {
    if band_samples == 0 {
        return Err(Reversible53Error::Empty);
    }
    let len = band_samples
        .checked_mul(2)
        .ok_or(Reversible53Error::SizeOverflow)?;
    if low.len() < band_samples {
        return Err(Reversible53Error::InputTooSmall);
    }
    if output.len() < len {
        return Err(Reversible53Error::OutputTooSmall);
    }
    // SAFETY: checked lengths cover the separate low band and both halves of
    // output; independent mutable borrows prove the low and output storage do
    // not alias.
    unsafe {
        (reversible_53_dispatch().split_kernel)(
            low.as_mut_ptr(),
            output.as_mut_ptr(),
            band_samples,
        );
    }
    Ok(())
}

type Reversible53VerticalU8Kernel =
    unsafe fn(*mut i32, usize, usize, usize, usize, usize, usize, usize, *mut u8, usize);

#[derive(Clone, Copy)]
struct Reversible53VerticalU8Dispatch {
    kernel: Reversible53VerticalU8Kernel,
}

const SCALAR_REVERSIBLE_53_VERTICAL_U8_DISPATCH: Reversible53VerticalU8Dispatch =
    Reversible53VerticalU8Dispatch {
        kernel: inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_scalar,
    };

#[cfg(feature = "std")]
static REVERSIBLE_53_VERTICAL_U8_DISPATCH: OnceLock<Reversible53VerticalU8Dispatch> =
    OnceLock::new();

#[cfg(feature = "std")]
fn reversible_53_vertical_u8_dispatch() -> &'static Reversible53VerticalU8Dispatch {
    REVERSIBLE_53_VERTICAL_U8_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return Reversible53VerticalU8Dispatch {
                kernel: inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_avx2,
            };
        }
        SCALAR_REVERSIBLE_53_VERTICAL_U8_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn reversible_53_vertical_u8_dispatch() -> &'static Reversible53VerticalU8Dispatch {
    &SCALAR_REVERSIBLE_53_VERTICAL_U8_DISPATCH
}

/// Reconstruct a final even, low-first reversible vertical level directly
/// from row-major coefficients into unsigned-8 caller rows.
///
/// The active coefficient rectangle is mutated as lifting steps are undone.
/// Output row padding is preserved. This final-level kernel avoids both the
/// column-major transpose and the transpose-pack pass used by the general
/// full-synthesis route.
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8(
    plane: &mut [i32],
    stride: usize,
    width: usize,
    height: usize,
    output: &mut [u8],
    output_stride: usize,
) -> Result<(), Reversible53Error> {
    inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_region(
        plane,
        stride,
        width,
        height,
        0,
        0,
        width,
        height,
        output,
        output_stride,
    )
}

/// Reconstruct a selected rectangle of a final even, low-first reversible
/// vertical level directly into unsigned-8 caller rows.
///
/// Vertical lifting is performed only for the selected columns, while all
/// source rows remain available for the lifting dependencies. Output row
/// padding is preserved.
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_region(
    plane: &mut [i32],
    stride: usize,
    source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: &mut [u8],
    output_stride: usize,
) -> Result<(), Reversible53Error> {
    if source_width == 0 || source_height == 0 || output_width == 0 || output_height == 0 {
        return Err(Reversible53Error::Empty);
    }
    if source_height < 2
        || !source_height.is_multiple_of(2)
        || stride < source_width
        || output_stride < output_width
        || output_x
            .checked_add(output_width)
            .is_none_or(|end| end > source_width)
        || output_y
            .checked_add(output_height)
            .is_none_or(|end| end > source_height)
    {
        return Err(Reversible53Error::InvalidGeometry);
    }
    let plane_required = (source_height - 1)
        .checked_mul(stride)
        .and_then(|offset| offset.checked_add(source_width))
        .ok_or(Reversible53Error::SizeOverflow)?;
    if plane.len() < plane_required {
        return Err(Reversible53Error::InputTooSmall);
    }
    let output_required = (output_height - 1)
        .checked_mul(output_stride)
        .and_then(|offset| offset.checked_add(output_width))
        .ok_or(Reversible53Error::SizeOverflow)?;
    if output.len() < output_required {
        return Err(Reversible53Error::OutputTooSmall);
    }
    // SAFETY: checked active rectangles cover every kernel load and store;
    // the independent mutable borrows prove coefficient and output storage do
    // not alias.
    unsafe {
        (reversible_53_vertical_u8_dispatch().kernel)(
            plane.as_mut_ptr(),
            stride,
            source_width,
            source_height,
            output_x,
            output_y,
            output_width,
            output_height,
            output.as_mut_ptr(),
            output_stride,
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_scalar(
    plane: *mut i32,
    stride: usize,
    _source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: *mut u8,
    output_stride: usize,
) {
    let band_samples = source_height / 2;
    let output_end_x = output_x + output_width;
    for x in output_x..output_end_x {
        let first_high = unsafe { *plane.add(band_samples * stride + x) };
        let low = unsafe { plane.add(x) };
        unsafe {
            *low = (*low).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2)
        };
    }
    for low_row in 1..band_samples {
        for x in output_x..output_end_x {
            let low = unsafe { plane.add(low_row * stride + x) };
            let left = unsafe { *plane.add((band_samples + low_row - 1) * stride + x) };
            let right = unsafe { *plane.add((band_samples + low_row) * stride + x) };
            unsafe { *low = (*low).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2) };
        }
    }
    for high_row in 0..band_samples - 1 {
        for x in output_x..output_end_x {
            let high = unsafe { plane.add((band_samples + high_row) * stride + x) };
            let left = unsafe { *plane.add(high_row * stride + x) };
            let right = unsafe { *plane.add((high_row + 1) * stride + x) };
            unsafe { *high = (*high).wrapping_add(left.wrapping_add(right) >> 1) };
        }
    }
    let last = band_samples - 1;
    for x in output_x..output_end_x {
        let high = unsafe { plane.add((band_samples + last) * stride + x) };
        let low = unsafe { *plane.add(last * stride + x) };
        unsafe { *high = (*high).wrapping_add(low.wrapping_add(low) >> 1) };
    }
    for local_row in 0..output_height {
        let source_row = output_y + local_row;
        let band_row = source_row / 2 + usize::from(!source_row.is_multiple_of(2)) * band_samples;
        for local_x in 0..output_width {
            let sample = unsafe { *plane.add(band_row * stride + output_x + local_x) };
            unsafe {
                *output.add(local_row * output_stride + local_x) =
                    sample.saturating_add(128).clamp(0, 255) as u8;
            }
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
unsafe fn inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_avx2(
    plane: *mut i32,
    stride: usize,
    _source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: *mut u8,
    output_stride: usize,
) {
    use core::arch::x86_64::*;

    let band_samples = source_height / 2;
    let vector_end = output_x + output_width / 8 * 8;
    let output_end_x = output_x + output_width;
    let two = _mm256_set1_epi32(2);
    let first_high = unsafe { plane.add(band_samples * stride) };
    let mut x = output_x;
    while x < vector_end {
        let low = unsafe { _mm256_loadu_si256(plane.add(x).cast()) };
        let high = unsafe { _mm256_loadu_si256(first_high.add(x).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(high, high), two), 2);
        unsafe { _mm256_storeu_si256(plane.add(x).cast(), _mm256_sub_epi32(low, update)) };
        x += 8;
    }
    for x in vector_end..output_end_x {
        let high = unsafe { *first_high.add(x) };
        let low = unsafe { plane.add(x) };
        unsafe { *low = (*low).wrapping_sub(high.wrapping_add(high).wrapping_add(2) >> 2) };
    }
    for low_row in 1..band_samples {
        let low = unsafe { plane.add(low_row * stride) };
        let left = unsafe { plane.add((band_samples + low_row - 1) * stride) };
        let right = unsafe { plane.add((band_samples + low_row) * stride) };
        let mut x = output_x;
        while x < vector_end {
            let lows = unsafe { _mm256_loadu_si256(low.add(x).cast()) };
            let lefts = unsafe { _mm256_loadu_si256(left.add(x).cast()) };
            let rights = unsafe { _mm256_loadu_si256(right.add(x).cast()) };
            let update =
                _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(lefts, rights), two), 2);
            unsafe { _mm256_storeu_si256(low.add(x).cast(), _mm256_sub_epi32(lows, update)) };
            x += 8;
        }
        for x in vector_end..output_end_x {
            unsafe {
                *low.add(x) = (*low.add(x))
                    .wrapping_sub((*left.add(x)).wrapping_add(*right.add(x)).wrapping_add(2) >> 2)
            };
        }
    }
    for high_row in 0..band_samples - 1 {
        let high = unsafe { plane.add((band_samples + high_row) * stride) };
        let left = unsafe { plane.add(high_row * stride) };
        let right = unsafe { plane.add((high_row + 1) * stride) };
        let mut x = output_x;
        while x < vector_end {
            let highs = unsafe { _mm256_loadu_si256(high.add(x).cast()) };
            let lefts = unsafe { _mm256_loadu_si256(left.add(x).cast()) };
            let rights = unsafe { _mm256_loadu_si256(right.add(x).cast()) };
            let update = _mm256_srai_epi32(_mm256_add_epi32(lefts, rights), 1);
            unsafe { _mm256_storeu_si256(high.add(x).cast(), _mm256_add_epi32(highs, update)) };
            x += 8;
        }
        for x in vector_end..output_end_x {
            unsafe {
                *high.add(x) =
                    (*high.add(x)).wrapping_add((*left.add(x)).wrapping_add(*right.add(x)) >> 1)
            };
        }
    }
    let last = band_samples - 1;
    let high = unsafe { plane.add((band_samples + last) * stride) };
    let low = unsafe { plane.add(last * stride) };
    let mut x = output_x;
    while x < vector_end {
        let highs = unsafe { _mm256_loadu_si256(high.add(x).cast()) };
        let lows = unsafe { _mm256_loadu_si256(low.add(x).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(lows, lows), 1);
        unsafe { _mm256_storeu_si256(high.add(x).cast(), _mm256_add_epi32(highs, update)) };
        x += 8;
    }
    for x in vector_end..output_end_x {
        unsafe {
            *high.add(x) = (*high.add(x)).wrapping_add((*low.add(x)).wrapping_add(*low.add(x)) >> 1)
        };
    }

    let minimum = _mm256_set1_epi32(-128);
    let maximum = _mm256_set1_epi32(127);
    let shift = _mm256_set1_epi32(128);
    let zero = _mm256_setzero_si256();
    let pack_vector_width = output_width / 16 * 16;
    for local_row in 0..output_height {
        let source_row = output_y + local_row;
        let band_row = source_row / 2 + usize::from(!source_row.is_multiple_of(2)) * band_samples;
        let source = unsafe { plane.add(band_row * stride + output_x) };
        let destination = unsafe { output.add(local_row * output_stride) };
        let mut local_x = 0;
        while local_x < pack_vector_width {
            let values_0 = unsafe { _mm256_loadu_si256(source.add(local_x).cast()) };
            let values_1 = unsafe { _mm256_loadu_si256(source.add(local_x + 8).cast()) };
            let values_0 = _mm256_add_epi32(
                _mm256_min_epi32(_mm256_max_epi32(values_0, minimum), maximum),
                shift,
            );
            let values_1 = _mm256_add_epi32(
                _mm256_min_epi32(_mm256_max_epi32(values_1, minimum), maximum),
                shift,
            );
            let words = _mm256_permute4x64_epi64(_mm256_packus_epi32(values_0, values_1), 0xd8);
            let bytes = _mm256_permute4x64_epi64(_mm256_packus_epi16(words, zero), 0xd8);
            unsafe {
                _mm_storeu_si128(
                    destination.add(local_x).cast(),
                    _mm256_castsi256_si128(bytes),
                );
            }
            local_x += 16;
        }
        for local_x in pack_vector_width..output_width {
            unsafe {
                *destination.add(local_x) =
                    (*source.add(local_x)).saturating_add(128).clamp(0, 255) as u8;
            }
        }
    }
}

type Reversible53VerticalI32Kernel =
    unsafe fn(*mut i32, usize, usize, usize, usize, usize, usize, usize, *mut i32, usize);

#[derive(Clone, Copy)]
struct Reversible53VerticalI32Dispatch {
    kernel: Reversible53VerticalI32Kernel,
}

const SCALAR_REVERSIBLE_53_VERTICAL_I32_DISPATCH: Reversible53VerticalI32Dispatch =
    Reversible53VerticalI32Dispatch {
        kernel: inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_scalar,
    };

#[cfg(feature = "std")]
static REVERSIBLE_53_VERTICAL_I32_DISPATCH: OnceLock<Reversible53VerticalI32Dispatch> =
    OnceLock::new();

#[cfg(feature = "std")]
fn reversible_53_vertical_i32_dispatch() -> &'static Reversible53VerticalI32Dispatch {
    REVERSIBLE_53_VERTICAL_I32_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return Reversible53VerticalI32Dispatch {
                kernel: inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_avx2,
            };
        }
        SCALAR_REVERSIBLE_53_VERTICAL_I32_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn reversible_53_vertical_i32_dispatch() -> &'static Reversible53VerticalI32Dispatch {
    &SCALAR_REVERSIBLE_53_VERTICAL_I32_DISPATCH
}

/// Reconstruct an even, low-first reversible vertical level into separate
/// row-major signed-sample storage.
///
/// The active coefficient rectangle is mutated as lifting steps are undone.
/// Output row padding is preserved. This lets a following horizontal level
/// consume reconstructed low-band rows without a column-major transpose round
/// trip.
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32(
    plane: &mut [i32],
    stride: usize,
    width: usize,
    height: usize,
    output: &mut [i32],
    output_stride: usize,
) -> Result<(), Reversible53Error> {
    inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_region(
        plane,
        stride,
        width,
        height,
        0,
        0,
        width,
        height,
        output,
        output_stride,
    )
}

/// Reconstruct a selected rectangle of an even, low-first reversible vertical
/// level into separate row-major signed-sample storage.
///
/// Vertical lifting is performed only for the selected columns, while all
/// source rows remain available for the lifting dependencies. Output row
/// padding is preserved.
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_region(
    plane: &mut [i32],
    stride: usize,
    source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: &mut [i32],
    output_stride: usize,
) -> Result<(), Reversible53Error> {
    if source_width == 0 || source_height == 0 || output_width == 0 || output_height == 0 {
        return Err(Reversible53Error::Empty);
    }
    if source_height < 2
        || !source_height.is_multiple_of(2)
        || stride < source_width
        || output_stride < output_width
        || output_x
            .checked_add(output_width)
            .is_none_or(|end| end > source_width)
        || output_y
            .checked_add(output_height)
            .is_none_or(|end| end > source_height)
    {
        return Err(Reversible53Error::InvalidGeometry);
    }
    let plane_required = (source_height - 1)
        .checked_mul(stride)
        .and_then(|offset| offset.checked_add(source_width))
        .ok_or(Reversible53Error::SizeOverflow)?;
    if plane.len() < plane_required {
        return Err(Reversible53Error::InputTooSmall);
    }
    let output_required = (output_height - 1)
        .checked_mul(output_stride)
        .and_then(|offset| offset.checked_add(output_width))
        .ok_or(Reversible53Error::SizeOverflow)?;
    if output.len() < output_required {
        return Err(Reversible53Error::OutputTooSmall);
    }
    // SAFETY: checked active rectangles cover every kernel load and store;
    // the independent mutable borrows prove coefficient and output storage do
    // not alias.
    unsafe {
        (reversible_53_vertical_i32_dispatch().kernel)(
            plane.as_mut_ptr(),
            stride,
            source_width,
            source_height,
            output_x,
            output_y,
            output_width,
            output_height,
            output.as_mut_ptr(),
            output_stride,
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_scalar(
    plane: *mut i32,
    stride: usize,
    _source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: *mut i32,
    output_stride: usize,
) {
    let band_samples = source_height / 2;
    let output_end_x = output_x + output_width;
    for x in output_x..output_end_x {
        let first_high = unsafe { *plane.add(band_samples * stride + x) };
        let low = unsafe { plane.add(x) };
        unsafe {
            *low = (*low).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2)
        };
    }
    for low_row in 1..band_samples {
        for x in output_x..output_end_x {
            let low = unsafe { plane.add(low_row * stride + x) };
            let left = unsafe { *plane.add((band_samples + low_row - 1) * stride + x) };
            let right = unsafe { *plane.add((band_samples + low_row) * stride + x) };
            unsafe { *low = (*low).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2) };
        }
    }
    for high_row in 0..band_samples - 1 {
        for x in output_x..output_end_x {
            let high = unsafe { plane.add((band_samples + high_row) * stride + x) };
            let left = unsafe { *plane.add(high_row * stride + x) };
            let right = unsafe { *plane.add((high_row + 1) * stride + x) };
            unsafe { *high = (*high).wrapping_add(left.wrapping_add(right) >> 1) };
        }
    }
    let last = band_samples - 1;
    for x in output_x..output_end_x {
        let high = unsafe { plane.add((band_samples + last) * stride + x) };
        let low = unsafe { *plane.add(last * stride + x) };
        unsafe { *high = (*high).wrapping_add(low.wrapping_add(low) >> 1) };
    }
    for local_row in 0..output_height {
        let source_row = output_y + local_row;
        let band_row = source_row / 2 + usize::from(!source_row.is_multiple_of(2)) * band_samples;
        unsafe {
            core::ptr::copy_nonoverlapping(
                plane.add(band_row * stride + output_x),
                output.add(local_row * output_stride),
                output_width,
            );
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
unsafe fn inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_avx2(
    plane: *mut i32,
    stride: usize,
    _source_width: usize,
    source_height: usize,
    output_x: usize,
    output_y: usize,
    output_width: usize,
    output_height: usize,
    output: *mut i32,
    output_stride: usize,
) {
    use core::arch::x86_64::*;

    let band_samples = source_height / 2;
    let vector_end = output_x + output_width / 8 * 8;
    let output_end_x = output_x + output_width;
    let two = _mm256_set1_epi32(2);
    let first_high = unsafe { plane.add(band_samples * stride) };
    let mut x = output_x;
    while x < vector_end {
        let low = unsafe { _mm256_loadu_si256(plane.add(x).cast()) };
        let high = unsafe { _mm256_loadu_si256(first_high.add(x).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(high, high), two), 2);
        unsafe { _mm256_storeu_si256(plane.add(x).cast(), _mm256_sub_epi32(low, update)) };
        x += 8;
    }
    for x in vector_end..output_end_x {
        let high = unsafe { *first_high.add(x) };
        let low = unsafe { plane.add(x) };
        unsafe { *low = (*low).wrapping_sub(high.wrapping_add(high).wrapping_add(2) >> 2) };
    }
    for low_row in 1..band_samples {
        let low = unsafe { plane.add(low_row * stride) };
        let left = unsafe { plane.add((band_samples + low_row - 1) * stride) };
        let right = unsafe { plane.add((band_samples + low_row) * stride) };
        let mut x = output_x;
        while x < vector_end {
            let lows = unsafe { _mm256_loadu_si256(low.add(x).cast()) };
            let lefts = unsafe { _mm256_loadu_si256(left.add(x).cast()) };
            let rights = unsafe { _mm256_loadu_si256(right.add(x).cast()) };
            let update =
                _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(lefts, rights), two), 2);
            unsafe { _mm256_storeu_si256(low.add(x).cast(), _mm256_sub_epi32(lows, update)) };
            x += 8;
        }
        for x in vector_end..output_end_x {
            unsafe {
                *low.add(x) = (*low.add(x))
                    .wrapping_sub((*left.add(x)).wrapping_add(*right.add(x)).wrapping_add(2) >> 2)
            };
        }
    }
    for high_row in 0..band_samples - 1 {
        let high = unsafe { plane.add((band_samples + high_row) * stride) };
        let left = unsafe { plane.add(high_row * stride) };
        let right = unsafe { plane.add((high_row + 1) * stride) };
        let mut x = output_x;
        while x < vector_end {
            let highs = unsafe { _mm256_loadu_si256(high.add(x).cast()) };
            let lefts = unsafe { _mm256_loadu_si256(left.add(x).cast()) };
            let rights = unsafe { _mm256_loadu_si256(right.add(x).cast()) };
            let update = _mm256_srai_epi32(_mm256_add_epi32(lefts, rights), 1);
            unsafe { _mm256_storeu_si256(high.add(x).cast(), _mm256_add_epi32(highs, update)) };
            x += 8;
        }
        for x in vector_end..output_end_x {
            unsafe {
                *high.add(x) =
                    (*high.add(x)).wrapping_add((*left.add(x)).wrapping_add(*right.add(x)) >> 1)
            };
        }
    }
    let last = band_samples - 1;
    let high = unsafe { plane.add((band_samples + last) * stride) };
    let low = unsafe { plane.add(last * stride) };
    let mut x = output_x;
    while x < vector_end {
        let highs = unsafe { _mm256_loadu_si256(high.add(x).cast()) };
        let lows = unsafe { _mm256_loadu_si256(low.add(x).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(lows, lows), 1);
        unsafe { _mm256_storeu_si256(high.add(x).cast(), _mm256_add_epi32(highs, update)) };
        x += 8;
    }
    for x in vector_end..output_end_x {
        unsafe {
            *high.add(x) = (*high.add(x)).wrapping_add((*low.add(x)).wrapping_add(*low.add(x)) >> 1)
        };
    }

    let vector_width = output_width / 8 * 8;
    for local_row in 0..output_height {
        let source_row = output_y + local_row;
        let band_row = source_row / 2 + usize::from(!source_row.is_multiple_of(2)) * band_samples;
        let source = unsafe { plane.add(band_row * stride + output_x) };
        let destination = unsafe { output.add(local_row * output_stride) };
        let mut local_x = 0;
        while local_x < vector_width {
            let values = unsafe { _mm256_loadu_si256(source.add(local_x).cast()) };
            unsafe { _mm256_storeu_si256(destination.add(local_x).cast(), values) };
            local_x += 8;
        }
        for local_x in vector_width..output_width {
            unsafe { *destination.add(local_x) = *source.add(local_x) };
        }
    }
}

type Reversible53OddKernel = unsafe fn(*mut i32, *mut i32, usize);

#[derive(Clone, Copy)]
struct Reversible53OddDispatch {
    kernel: Reversible53OddKernel,
}

const SCALAR_REVERSIBLE_53_ODD_DISPATCH: Reversible53OddDispatch = Reversible53OddDispatch {
    kernel: inverse_reversible_5_3_odd_first_low_scalar,
};

#[cfg(feature = "std")]
static REVERSIBLE_53_ODD_DISPATCH: OnceLock<Reversible53OddDispatch> = OnceLock::new();

#[cfg(feature = "std")]
fn reversible_53_odd_dispatch() -> &'static Reversible53OddDispatch {
    REVERSIBLE_53_ODD_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return Reversible53OddDispatch {
                kernel: inverse_reversible_5_3_odd_first_low_avx2,
            };
        }
        SCALAR_REVERSIBLE_53_ODD_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn reversible_53_odd_dispatch() -> &'static Reversible53OddDispatch {
    &SCALAR_REVERSIBLE_53_ODD_DISPATCH
}

/// Reconstruct an odd, low-first reversible 5/3 line exactly.
///
/// `read` contains `low_samples` low coefficients followed by
/// `low_samples - 1` high coefficients. `output` receives the odd-length
/// interleaved spatial line. The architecture kernel preserves the scalar
/// wrapping and signed-floor semantics.
pub fn inverse_reversible_5_3_odd_first_low(
    read: &mut [i32],
    output: &mut [i32],
    low_samples: usize,
) -> Result<(), Reversible53Error> {
    if low_samples < 2 {
        return Err(Reversible53Error::Empty);
    }
    let len = low_samples
        .checked_mul(2)
        .and_then(|value| value.checked_sub(1))
        .ok_or(Reversible53Error::SizeOverflow)?;
    if read.len() < len {
        return Err(Reversible53Error::InputTooSmall);
    }
    if output.len() < len {
        return Err(Reversible53Error::OutputTooSmall);
    }
    // SAFETY: checked lengths cover every kernel load/store, and the mutable
    // borrows prove input scratch and output do not alias.
    unsafe {
        (reversible_53_odd_dispatch().kernel)(read.as_mut_ptr(), output.as_mut_ptr(), low_samples);
    }
    Ok(())
}

unsafe fn inverse_reversible_5_3_odd_first_low_scalar(
    read: *mut i32,
    output: *mut i32,
    low_samples: usize,
) {
    let high_samples = low_samples - 1;
    // SAFETY: the checked wrapper validates both odd-length slices.
    let read = unsafe { core::slice::from_raw_parts_mut(read, low_samples + high_samples) };
    // SAFETY: the checked wrapper validates both odd-length slices.
    let output = unsafe { core::slice::from_raw_parts_mut(output, low_samples + high_samples) };

    let first_high = read[low_samples];
    read[0] = read[0].wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2);
    for low in 1..high_samples {
        read[low] = read[low].wrapping_sub(
            read[low_samples + low - 1]
                .wrapping_add(read[low_samples + low])
                .wrapping_add(2)
                >> 2,
        );
    }
    let last_high = read[low_samples + high_samples - 1];
    read[high_samples] =
        read[high_samples].wrapping_sub(last_high.wrapping_add(last_high).wrapping_add(2) >> 2);

    for high in 0..high_samples {
        read[low_samples + high] =
            read[low_samples + high].wrapping_add(read[high].wrapping_add(read[high + 1]) >> 1);
        output[2 * high] = read[high];
        output[2 * high + 1] = read[low_samples + high];
    }
    output[2 * high_samples] = read[high_samples];
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn inverse_reversible_5_3_odd_first_low_avx2(
    read: *mut i32,
    output: *mut i32,
    low_samples: usize,
) {
    use core::arch::x86_64::*;

    let high_samples = low_samples - 1;
    let first_high = unsafe { *read.add(low_samples) };
    unsafe {
        *read = (*read).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2)
    };
    let two = _mm256_set1_epi32(2);
    let mut low = 1;
    while low + 8 <= high_samples {
        let lows = unsafe { _mm256_loadu_si256(read.add(low).cast()) };
        let left = unsafe { _mm256_loadu_si256(read.add(low_samples + low - 1).cast()) };
        let right = unsafe { _mm256_loadu_si256(read.add(low_samples + low).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(left, right), two), 2);
        unsafe { _mm256_storeu_si256(read.add(low).cast(), _mm256_sub_epi32(lows, update)) };
        low += 8;
    }
    while low < high_samples {
        let left = unsafe { *read.add(low_samples + low - 1) };
        let right = unsafe { *read.add(low_samples + low) };
        unsafe {
            *read.add(low) =
                (*read.add(low)).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2)
        };
        low += 1;
    }
    let last_high = unsafe { *read.add(low_samples + high_samples - 1) };
    unsafe {
        *read.add(high_samples) = (*read.add(high_samples))
            .wrapping_sub(last_high.wrapping_add(last_high).wrapping_add(2) >> 2)
    };

    let mut high = 0;
    while high + 8 <= high_samples {
        let highs = unsafe { _mm256_loadu_si256(read.add(low_samples + high).cast()) };
        let left = unsafe { _mm256_loadu_si256(read.add(high).cast()) };
        let right = unsafe { _mm256_loadu_si256(read.add(high + 1).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(left, right), 1);
        unsafe {
            _mm256_storeu_si256(
                read.add(low_samples + high).cast(),
                _mm256_add_epi32(highs, update),
            )
        };
        high += 8;
    }
    while high < high_samples {
        let left = unsafe { *read.add(high) };
        let right = unsafe { *read.add(high + 1) };
        unsafe {
            *read.add(low_samples + high) =
                (*read.add(low_samples + high)).wrapping_add(left.wrapping_add(right) >> 1)
        };
        high += 1;
    }

    let mut index = 0;
    while index + 8 <= high_samples {
        let lows = unsafe { _mm256_loadu_si256(read.add(index).cast()) };
        let highs = unsafe { _mm256_loadu_si256(read.add(low_samples + index).cast()) };
        let low_pairs = _mm256_unpacklo_epi32(lows, highs);
        let high_pairs = _mm256_unpackhi_epi32(lows, highs);
        let first = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x20);
        let second = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x31);
        unsafe {
            _mm256_storeu_si256(output.add(2 * index).cast(), first);
            _mm256_storeu_si256(output.add(2 * index + 8).cast(), second);
        }
        index += 8;
    }
    while index < high_samples {
        unsafe {
            *output.add(2 * index) = *read.add(index);
            *output.add(2 * index + 1) = *read.add(low_samples + index);
        }
        index += 1;
    }
    unsafe { *output.add(2 * high_samples) = *read.add(high_samples) };
}

unsafe fn inverse_reversible_5_3_even_first_low_split_scalar(
    low: *mut i32,
    output: *mut i32,
    band_samples: usize,
) {
    let high = unsafe { output.add(band_samples) };
    let first_high = unsafe { *high };
    unsafe { *low = (*low).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2) };
    for index in 1..band_samples {
        let left = unsafe { *high.add(index - 1) };
        let right = unsafe { *high.add(index) };
        unsafe {
            *low.add(index) =
                (*low.add(index)).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2)
        };
    }
    for index in 0..band_samples - 1 {
        let left = unsafe { *low.add(index) };
        let right = unsafe { *low.add(index + 1) };
        unsafe {
            *high.add(index) = (*high.add(index)).wrapping_add(left.wrapping_add(right) >> 1)
        };
    }
    let last = band_samples - 1;
    let last_low = unsafe { *low.add(last) };
    unsafe {
        *high.add(last) = (*high.add(last)).wrapping_add(last_low.wrapping_add(last_low) >> 1)
    };
    // Every destination precedes or equals its high-band source. Forward
    // interleaving therefore overwrites only high values already consumed.
    for index in 0..band_samples {
        let low_value = unsafe { *low.add(index) };
        let high_value = unsafe { *high.add(index) };
        unsafe {
            *output.add(2 * index) = low_value;
            *output.add(2 * index + 1) = high_value;
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn inverse_reversible_5_3_even_first_low_split_avx2(
    low: *mut i32,
    output: *mut i32,
    band_samples: usize,
) {
    use core::arch::x86_64::*;

    let high = unsafe { output.add(band_samples) };
    let first_high = unsafe { *high };
    unsafe { *low = (*low).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2) };
    let two = _mm256_set1_epi32(2);
    let mut index = 1;
    while index + 8 <= band_samples {
        let lows = unsafe { _mm256_loadu_si256(low.add(index).cast()) };
        let left = unsafe { _mm256_loadu_si256(high.add(index - 1).cast()) };
        let right = unsafe { _mm256_loadu_si256(high.add(index).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(left, right), two), 2);
        unsafe { _mm256_storeu_si256(low.add(index).cast(), _mm256_sub_epi32(lows, update)) };
        index += 8;
    }
    while index < band_samples {
        let left = unsafe { *high.add(index - 1) };
        let right = unsafe { *high.add(index) };
        unsafe {
            *low.add(index) =
                (*low.add(index)).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2)
        };
        index += 1;
    }

    index = 0;
    while index + 8 < band_samples {
        let highs = unsafe { _mm256_loadu_si256(high.add(index).cast()) };
        let left = unsafe { _mm256_loadu_si256(low.add(index).cast()) };
        let right = unsafe { _mm256_loadu_si256(low.add(index + 1).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(left, right), 1);
        unsafe { _mm256_storeu_si256(high.add(index).cast(), _mm256_add_epi32(highs, update)) };
        index += 8;
    }
    while index < band_samples - 1 {
        let left = unsafe { *low.add(index) };
        let right = unsafe { *low.add(index + 1) };
        unsafe {
            *high.add(index) = (*high.add(index)).wrapping_add(left.wrapping_add(right) >> 1)
        };
        index += 1;
    }
    let last = band_samples - 1;
    let last_low = unsafe { *low.add(last) };
    unsafe {
        *high.add(last) = (*high.add(last)).wrapping_add(last_low.wrapping_add(last_low) >> 1)
    };

    index = 0;
    while index + 8 <= band_samples {
        let lows = unsafe { _mm256_loadu_si256(low.add(index).cast()) };
        let highs = unsafe { _mm256_loadu_si256(high.add(index).cast()) };
        let low_pairs = _mm256_unpacklo_epi32(lows, highs);
        let high_pairs = _mm256_unpackhi_epi32(lows, highs);
        let first = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x20);
        let second = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x31);
        unsafe {
            _mm256_storeu_si256(output.add(2 * index).cast(), first);
            _mm256_storeu_si256(output.add(2 * index + 8).cast(), second);
        }
        index += 8;
    }
    while index < band_samples {
        let low_value = unsafe { *low.add(index) };
        let high_value = unsafe { *high.add(index) };
        unsafe {
            *output.add(2 * index) = low_value;
            *output.add(2 * index + 1) = high_value;
        }
        index += 1;
    }
}

unsafe fn inverse_reversible_5_3_even_first_low_scalar(
    read: *mut i32,
    output: *mut i32,
    band_samples: usize,
) {
    // SAFETY: the checked wrapper validates both 2*band slices.
    let read = unsafe { core::slice::from_raw_parts_mut(read, band_samples * 2) };
    // SAFETY: the checked wrapper validates both 2*band slices.
    let output = unsafe { core::slice::from_raw_parts_mut(output, band_samples * 2) };
    let first_high = read[band_samples];
    read[0] = read[0].wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2);
    for low in 1..band_samples {
        read[low] = read[low].wrapping_sub(
            read[band_samples + low - 1]
                .wrapping_add(read[band_samples + low])
                .wrapping_add(2)
                >> 2,
        );
    }
    for high in 0..band_samples - 1 {
        read[band_samples + high] =
            read[band_samples + high].wrapping_add(read[high].wrapping_add(read[high + 1]) >> 1);
    }
    let last = band_samples - 1;
    read[band_samples + last] =
        read[band_samples + last].wrapping_add(read[last].wrapping_add(read[last]) >> 1);
    for index in 0..band_samples {
        output[2 * index] = read[index];
        output[2 * index + 1] = read[band_samples + index];
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn inverse_reversible_5_3_even_first_low_avx2(
    read: *mut i32,
    output: *mut i32,
    band_samples: usize,
) {
    use core::arch::x86_64::*;

    let first_high = unsafe { *read.add(band_samples) };
    unsafe {
        *read = (*read).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2)
    };
    let two = _mm256_set1_epi32(2);
    let mut low = 1;
    while low + 8 <= band_samples {
        let lows = unsafe { _mm256_loadu_si256(read.add(low).cast()) };
        let left = unsafe { _mm256_loadu_si256(read.add(band_samples + low - 1).cast()) };
        let right = unsafe { _mm256_loadu_si256(read.add(band_samples + low).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(_mm256_add_epi32(left, right), two), 2);
        unsafe { _mm256_storeu_si256(read.add(low).cast(), _mm256_sub_epi32(lows, update)) };
        low += 8;
    }
    while low < band_samples {
        let left = unsafe { *read.add(band_samples + low - 1) };
        let right = unsafe { *read.add(band_samples + low) };
        unsafe {
            *read.add(low) =
                (*read.add(low)).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2)
        };
        low += 1;
    }

    let mut high = 0;
    while high + 8 < band_samples {
        let highs = unsafe { _mm256_loadu_si256(read.add(band_samples + high).cast()) };
        let left = unsafe { _mm256_loadu_si256(read.add(high).cast()) };
        let right = unsafe { _mm256_loadu_si256(read.add(high + 1).cast()) };
        let update = _mm256_srai_epi32(_mm256_add_epi32(left, right), 1);
        unsafe {
            _mm256_storeu_si256(
                read.add(band_samples + high).cast(),
                _mm256_add_epi32(highs, update),
            )
        };
        high += 8;
    }
    while high < band_samples - 1 {
        let left = unsafe { *read.add(high) };
        let right = unsafe { *read.add(high + 1) };
        unsafe {
            *read.add(band_samples + high) =
                (*read.add(band_samples + high)).wrapping_add(left.wrapping_add(right) >> 1)
        };
        high += 1;
    }
    let last = band_samples - 1;
    let last_low = unsafe { *read.add(last) };
    unsafe {
        *read.add(band_samples + last) =
            (*read.add(band_samples + last)).wrapping_add(last_low.wrapping_add(last_low) >> 1)
    };

    let mut index = 0;
    while index + 8 <= band_samples {
        let lows = unsafe { _mm256_loadu_si256(read.add(index).cast()) };
        let highs = unsafe { _mm256_loadu_si256(read.add(band_samples + index).cast()) };
        let low_pairs = _mm256_unpacklo_epi32(lows, highs);
        let high_pairs = _mm256_unpackhi_epi32(lows, highs);
        let first = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x20);
        let second = _mm256_permute2x128_si256(low_pairs, high_pairs, 0x31);
        unsafe {
            _mm256_storeu_si256(output.add(2 * index).cast(), first);
            _mm256_storeu_si256(output.add(2 * index + 8).cast(), second);
        }
        index += 8;
    }
    while index < band_samples {
        unsafe {
            *output.add(2 * index) = *read.add(index);
            *output.add(2 * index + 1) = *read.add(band_samples + index);
        }
        index += 1;
    }
}

#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[target_feature(enable = "neon")]
unsafe fn inverse_reversible_5_3_even_first_low_neon(
    read: *mut i32,
    output: *mut i32,
    band_samples: usize,
) {
    use core::arch::aarch64::*;

    let first_high = unsafe { *read.add(band_samples) };
    unsafe {
        *read = (*read).wrapping_sub(first_high.wrapping_add(first_high).wrapping_add(2) >> 2)
    };
    let mut low = 1;
    while low + 4 <= band_samples {
        let lows = unsafe { vld1q_s32(read.add(low)) };
        let left = unsafe { vld1q_s32(read.add(band_samples + low - 1)) };
        let right = unsafe { vld1q_s32(read.add(band_samples + low)) };
        let update = vshrq_n_s32(vaddq_s32(vaddq_s32(left, right), vdupq_n_s32(2)), 2);
        unsafe { vst1q_s32(read.add(low), vsubq_s32(lows, update)) };
        low += 4;
    }
    while low < band_samples {
        let left = unsafe { *read.add(band_samples + low - 1) };
        let right = unsafe { *read.add(band_samples + low) };
        unsafe {
            *read.add(low) =
                (*read.add(low)).wrapping_sub(left.wrapping_add(right).wrapping_add(2) >> 2)
        };
        low += 1;
    }
    let mut high = 0;
    while high + 4 <= band_samples - 1 {
        let highs = unsafe { vld1q_s32(read.add(band_samples + high)) };
        let left = unsafe { vld1q_s32(read.add(high)) };
        let right = unsafe { vld1q_s32(read.add(high + 1)) };
        let update = vshrq_n_s32(vaddq_s32(left, right), 1);
        unsafe { vst1q_s32(read.add(band_samples + high), vaddq_s32(highs, update)) };
        high += 4;
    }
    while high < band_samples - 1 {
        let left = unsafe { *read.add(high) };
        let right = unsafe { *read.add(high + 1) };
        unsafe {
            *read.add(band_samples + high) =
                (*read.add(band_samples + high)).wrapping_add(left.wrapping_add(right) >> 1)
        };
        high += 1;
    }
    let last = band_samples - 1;
    let last_low = unsafe { *read.add(last) };
    unsafe {
        *read.add(band_samples + last) =
            (*read.add(band_samples + last)).wrapping_add(last_low.wrapping_add(last_low) >> 1)
    };
    let mut index = 0;
    while index + 4 <= band_samples {
        let lows = unsafe { vld1q_s32(read.add(index)) };
        let highs = unsafe { vld1q_s32(read.add(band_samples + index)) };
        unsafe {
            vst1q_s32(output.add(2 * index), vzip1q_s32(lows, highs));
            vst1q_s32(output.add(2 * index + 4), vzip2q_s32(lows, highs));
        }
        index += 4;
    }
    while index < band_samples {
        unsafe {
            *output.add(2 * index) = *read.add(index);
            *output.add(2 * index + 1) = *read.add(band_samples + index);
        }
        index += 1;
    }
}

/// Process-selected 32-bit block-transpose implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransposeBackend {
    Scalar,
    Avx2,
    Neon,
}

impl TransposeBackend {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Avx2 => "avx2",
            Self::Neon => "neon",
        }
    }
}

type Transpose32Kernel = unsafe fn(*const u32, usize, usize, usize, usize, *mut u32, usize);

#[derive(Clone, Copy)]
struct TransposeDispatch {
    backend: TransposeBackend,
    kernel: Transpose32Kernel,
}

const SCALAR_TRANSPOSE_DISPATCH: TransposeDispatch = TransposeDispatch {
    backend: TransposeBackend::Scalar,
    kernel: transpose_32bit_scalar,
};

#[cfg(feature = "std")]
static TRANSPOSE_DISPATCH: OnceLock<TransposeDispatch> = OnceLock::new();

#[cfg(feature = "std")]
fn transpose_dispatch() -> &'static TransposeDispatch {
    TRANSPOSE_DISPATCH.get_or_init(|| {
        #[cfg(all(feature = "simd", target_arch = "x86_64"))]
        if std::arch::is_x86_feature_detected!("avx2") {
            return TransposeDispatch {
                backend: TransposeBackend::Avx2,
                kernel: transpose_32bit_avx2,
            };
        }
        #[cfg(all(feature = "simd", target_arch = "aarch64"))]
        if std::arch::is_aarch64_feature_detected!("neon") {
            return TransposeDispatch {
                backend: TransposeBackend::Neon,
                kernel: transpose_32bit_neon,
            };
        }
        SCALAR_TRANSPOSE_DISPATCH
    })
}

#[cfg(not(feature = "std"))]
fn transpose_dispatch() -> &'static TransposeDispatch {
    &SCALAR_TRANSPOSE_DISPATCH
}

/// Return the cached block-transpose backend used by safe transpose APIs.
pub fn transpose_backend() -> TransposeBackend {
    transpose_dispatch().backend
}

#[allow(clippy::too_many_arguments)]
fn validate_transpose_stripe<T>(
    source: &[T],
    source_stride: usize,
    source_width: usize,
    source_height: usize,
    source_x: usize,
    columns: usize,
    destination: &[T],
    destination_stride: usize,
) -> Result<(), TransposeError> {
    if source_width == 0 || source_height == 0 || columns == 0 {
        return Err(TransposeError::Empty);
    }
    if source_stride < source_width || destination_stride < source_height {
        return Err(TransposeError::StrideTooSmall);
    }
    if source_x
        .checked_add(columns)
        .ok_or(TransposeError::SizeOverflow)?
        > source_width
    {
        return Err(TransposeError::SourceTooSmall);
    }
    let source_required = (source_height - 1)
        .checked_mul(source_stride)
        .and_then(|offset| offset.checked_add(source_width))
        .ok_or(TransposeError::SizeOverflow)?;
    if source.len() < source_required {
        return Err(TransposeError::SourceTooSmall);
    }
    let destination_required = (columns - 1)
        .checked_mul(destination_stride)
        .and_then(|offset| offset.checked_add(source_height))
        .ok_or(TransposeError::SizeOverflow)?;
    if destination.len() < destination_required {
        return Err(TransposeError::DestinationTooSmall);
    }
    Ok(())
}

macro_rules! define_transpose_stripe {
    ($safe_name:ident, $sample:ty) => {
        /// Transpose a validated source-column stripe into destination rows.
        ///
        /// `destination` contains `columns` rows of `source_height` samples.
        /// Padding beyond each active destination row is not modified.
        #[allow(clippy::too_many_arguments)]
        pub fn $safe_name(
            source: &[$sample],
            source_stride: usize,
            source_width: usize,
            source_height: usize,
            source_x: usize,
            columns: usize,
            destination: &mut [$sample],
            destination_stride: usize,
        ) -> Result<(), TransposeError> {
            validate_transpose_stripe(
                source,
                source_stride,
                source_width,
                source_height,
                source_x,
                columns,
                destination,
                destination_stride,
            )?;
            // SAFETY: validation proves every source and destination address
            // used by the kernel is inside its non-aliasing borrowed slice.
            unsafe {
                (transpose_dispatch().kernel)(
                    source.as_ptr().cast::<u32>(),
                    source_stride,
                    source_height,
                    source_x,
                    columns,
                    destination.as_mut_ptr().cast::<u32>(),
                    destination_stride,
                );
            }
            Ok(())
        }
    };
}

define_transpose_stripe!(transpose_i32_stripe, i32);
define_transpose_stripe!(transpose_f32_stripe, f32);

/// Transpose a column-major signed-coefficient rectangle while applying the
/// exact unsigned-8 JPEG 2000 level shift and packing into padded output rows.
#[allow(clippy::too_many_arguments)]
pub fn transpose_pack_i32_unsigned_u8(
    source: &[i32],
    source_width: usize,
    source_height: usize,
    source_x: usize,
    source_y: usize,
    columns: usize,
    rows: usize,
    destination: &mut [u8],
    destination_stride: usize,
) -> Result<(), TransposeError> {
    if source_width == 0 || source_height == 0 || columns == 0 || rows == 0 {
        return Err(TransposeError::Empty);
    }
    if destination_stride < columns {
        return Err(TransposeError::StrideTooSmall);
    }
    if source_x
        .checked_add(columns)
        .ok_or(TransposeError::SizeOverflow)?
        > source_width
        || source_y
            .checked_add(rows)
            .ok_or(TransposeError::SizeOverflow)?
            > source_height
    {
        return Err(TransposeError::SourceTooSmall);
    }
    let source_required = source_width
        .checked_mul(source_height)
        .ok_or(TransposeError::SizeOverflow)?;
    if source.len() < source_required {
        return Err(TransposeError::SourceTooSmall);
    }
    let destination_required = (rows - 1)
        .checked_mul(destination_stride)
        .and_then(|offset| offset.checked_add(columns))
        .ok_or(TransposeError::SizeOverflow)?;
    if destination.len() < destination_required {
        return Err(TransposeError::DestinationTooSmall);
    }
    // SAFETY: the checked active rectangles cover every kernel load/store;
    // shared source and mutable destination borrows cannot alias.
    unsafe {
        match transpose_dispatch().backend {
            #[cfg(all(feature = "simd", target_arch = "x86_64"))]
            TransposeBackend::Avx2 => transpose_pack_i32_unsigned_u8_avx2(
                source.as_ptr(),
                source_height,
                source_x,
                source_y,
                columns,
                rows,
                destination.as_mut_ptr(),
                destination_stride,
            ),
            _ => transpose_pack_i32_unsigned_u8_scalar(
                source.as_ptr(),
                source_height,
                source_x,
                source_y,
                columns,
                rows,
                destination.as_mut_ptr(),
                destination_stride,
            ),
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn transpose_pack_i32_unsigned_u8_scalar(
    source: *const i32,
    source_height: usize,
    source_x: usize,
    source_y: usize,
    columns: usize,
    rows: usize,
    destination: *mut u8,
    destination_stride: usize,
) {
    for row in 0..rows {
        for column in 0..columns {
            let coefficient =
                unsafe { *source.add((source_x + column) * source_height + source_y + row) };
            unsafe {
                *destination.add(row * destination_stride + column) =
                    coefficient.saturating_add(128).clamp(0, 255) as u8
            };
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::too_many_arguments)]
unsafe fn transpose_pack_i32_unsigned_u8_avx2(
    source: *const i32,
    source_height: usize,
    source_x: usize,
    source_y: usize,
    columns: usize,
    rows: usize,
    destination: *mut u8,
    destination_stride: usize,
) {
    use core::arch::x86_64::*;

    let vector_rows = rows / 8 * 8;
    let wide_vector_columns = columns / 16 * 16;
    let vector_columns = columns / 8 * 8;
    let minimum = _mm256_set1_epi32(-128);
    let maximum = _mm256_set1_epi32(127);
    let shift = _mm256_set1_epi32(128);
    for row in (0..vector_rows).step_by(8) {
        for column in (0..wide_vector_columns).step_by(16) {
            let first = unsafe {
                transpose_8x8_i32_avx2(
                    source.add((source_x + column) * source_height + source_y + row),
                    source_height,
                )
            };
            let second = unsafe {
                transpose_8x8_i32_avx2(
                    source.add((source_x + column + 8) * source_height + source_y + row),
                    source_height,
                )
            };
            for local_row in 0..8 {
                let first = _mm256_add_epi32(
                    _mm256_min_epi32(_mm256_max_epi32(first[local_row], minimum), maximum),
                    shift,
                );
                let second = _mm256_add_epi32(
                    _mm256_min_epi32(_mm256_max_epi32(second[local_row], minimum), maximum),
                    shift,
                );
                let words = _mm256_permute4x64_epi64(_mm256_packus_epi32(first, second), 0xd8);
                let bytes = _mm_packus_epi16(
                    _mm256_castsi256_si128(words),
                    _mm256_extracti128_si256(words, 1),
                );
                unsafe {
                    _mm_storeu_si128(
                        destination
                            .add((row + local_row) * destination_stride + column)
                            .cast(),
                        bytes,
                    )
                };
            }
        }
        if vector_columns > wide_vector_columns {
            let column = wide_vector_columns;
            let values = unsafe {
                transpose_8x8_i32_avx2(
                    source.add((source_x + column) * source_height + source_y + row),
                    source_height,
                )
            };
            let zero = _mm_setzero_si128();
            for local_row in 0..8 {
                let values = _mm256_add_epi32(
                    _mm256_min_epi32(_mm256_max_epi32(values[local_row], minimum), maximum),
                    shift,
                );
                let words = _mm_packus_epi32(
                    _mm256_castsi256_si128(values),
                    _mm256_extracti128_si256(values, 1),
                );
                let bytes = _mm_packus_epi16(words, zero);
                unsafe {
                    _mm_storel_epi64(
                        destination
                            .add((row + local_row) * destination_stride + column)
                            .cast(),
                        bytes,
                    )
                };
            }
        }
    }
    for row in 0..rows {
        let first_scalar_column = if row < vector_rows { vector_columns } else { 0 };
        for column in first_scalar_column..columns {
            let coefficient =
                unsafe { *source.add((source_x + column) * source_height + source_y + row) };
            unsafe {
                *destination.add(row * destination_stride + column) =
                    coefficient.saturating_add(128).clamp(0, 255) as u8
            };
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn transpose_8x8_i32_avx2(
    source: *const i32,
    source_stride: usize,
) -> [core::arch::x86_64::__m256i; 8] {
    use core::arch::x86_64::*;

    let mut input = [_mm256_setzero_si256(); 8];
    for (row, value) in input.iter_mut().enumerate() {
        *value = unsafe { _mm256_loadu_si256(source.add(row * source_stride).cast()) };
    }
    let t0 = _mm256_unpacklo_epi32(input[0], input[1]);
    let t1 = _mm256_unpackhi_epi32(input[0], input[1]);
    let t2 = _mm256_unpacklo_epi32(input[2], input[3]);
    let t3 = _mm256_unpackhi_epi32(input[2], input[3]);
    let t4 = _mm256_unpacklo_epi32(input[4], input[5]);
    let t5 = _mm256_unpackhi_epi32(input[4], input[5]);
    let t6 = _mm256_unpacklo_epi32(input[6], input[7]);
    let t7 = _mm256_unpackhi_epi32(input[6], input[7]);
    let u0 = _mm256_unpacklo_epi64(t0, t2);
    let u1 = _mm256_unpackhi_epi64(t0, t2);
    let u2 = _mm256_unpacklo_epi64(t1, t3);
    let u3 = _mm256_unpackhi_epi64(t1, t3);
    let u4 = _mm256_unpacklo_epi64(t4, t6);
    let u5 = _mm256_unpackhi_epi64(t4, t6);
    let u6 = _mm256_unpacklo_epi64(t5, t7);
    let u7 = _mm256_unpackhi_epi64(t5, t7);
    [
        _mm256_permute2x128_si256(u0, u4, 0x20),
        _mm256_permute2x128_si256(u1, u5, 0x20),
        _mm256_permute2x128_si256(u2, u6, 0x20),
        _mm256_permute2x128_si256(u3, u7, 0x20),
        _mm256_permute2x128_si256(u0, u4, 0x31),
        _mm256_permute2x128_si256(u1, u5, 0x31),
        _mm256_permute2x128_si256(u2, u6, 0x31),
        _mm256_permute2x128_si256(u3, u7, 0x31),
    ]
}

unsafe fn transpose_32bit_scalar(
    source: *const u32,
    source_stride: usize,
    source_height: usize,
    source_x: usize,
    columns: usize,
    destination: *mut u32,
    destination_stride: usize,
) {
    const BLOCK: usize = 32;
    for first_row in (0..source_height).step_by(BLOCK) {
        let rows = (source_height - first_row).min(BLOCK);
        for local_row in 0..rows {
            // SAFETY: the checked wrapper validates the active source.
            let source_row =
                unsafe { source.add((first_row + local_row) * source_stride + source_x) };
            for column in 0..columns {
                // SAFETY: the checked wrapper validates the destination.
                unsafe {
                    *destination.add(column * destination_stride + first_row + local_row) =
                        *source_row.add(column);
                }
            }
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn transpose_32bit_avx2(
    source: *const u32,
    source_stride: usize,
    source_height: usize,
    source_x: usize,
    columns: usize,
    destination: *mut u32,
    destination_stride: usize,
) {
    use core::arch::x86_64::*;

    let vector_rows = source_height / 8 * 8;
    let vector_columns = columns / 8 * 8;
    for row in (0..vector_rows).step_by(8) {
        for column in (0..vector_columns).step_by(8) {
            let mut input = [_mm256_setzero_si256(); 8];
            for (local_row, value) in input.iter_mut().enumerate() {
                // SAFETY: the checked wrapper validates every 8-sample load.
                *value = unsafe {
                    _mm256_loadu_si256(
                        source
                            .add((row + local_row) * source_stride + source_x + column)
                            .cast::<__m256i>(),
                    )
                };
            }
            let t0 = _mm256_unpacklo_epi32(input[0], input[1]);
            let t1 = _mm256_unpackhi_epi32(input[0], input[1]);
            let t2 = _mm256_unpacklo_epi32(input[2], input[3]);
            let t3 = _mm256_unpackhi_epi32(input[2], input[3]);
            let t4 = _mm256_unpacklo_epi32(input[4], input[5]);
            let t5 = _mm256_unpackhi_epi32(input[4], input[5]);
            let t6 = _mm256_unpacklo_epi32(input[6], input[7]);
            let t7 = _mm256_unpackhi_epi32(input[6], input[7]);
            let u0 = _mm256_unpacklo_epi64(t0, t2);
            let u1 = _mm256_unpackhi_epi64(t0, t2);
            let u2 = _mm256_unpacklo_epi64(t1, t3);
            let u3 = _mm256_unpackhi_epi64(t1, t3);
            let u4 = _mm256_unpacklo_epi64(t4, t6);
            let u5 = _mm256_unpackhi_epi64(t4, t6);
            let u6 = _mm256_unpacklo_epi64(t5, t7);
            let u7 = _mm256_unpackhi_epi64(t5, t7);
            let output = [
                _mm256_permute2x128_si256(u0, u4, 0x20),
                _mm256_permute2x128_si256(u1, u5, 0x20),
                _mm256_permute2x128_si256(u2, u6, 0x20),
                _mm256_permute2x128_si256(u3, u7, 0x20),
                _mm256_permute2x128_si256(u0, u4, 0x31),
                _mm256_permute2x128_si256(u1, u5, 0x31),
                _mm256_permute2x128_si256(u2, u6, 0x31),
                _mm256_permute2x128_si256(u3, u7, 0x31),
            ];
            for (local_column, value) in output.into_iter().enumerate() {
                // SAFETY: the checked wrapper validates every 8-sample store.
                unsafe {
                    _mm256_storeu_si256(
                        destination
                            .add((column + local_column) * destination_stride + row)
                            .cast::<__m256i>(),
                        value,
                    );
                }
            }
        }
    }
    for column in 0..vector_columns {
        for row in vector_rows..source_height {
            // SAFETY: scalar tails remain inside checked active rectangles.
            unsafe {
                *destination.add(column * destination_stride + row) =
                    *source.add(row * source_stride + source_x + column);
            }
        }
    }
    for column in vector_columns..columns {
        for row in 0..source_height {
            // SAFETY: scalar tails remain inside checked active rectangles.
            unsafe {
                *destination.add(column * destination_stride + row) =
                    *source.add(row * source_stride + source_x + column);
            }
        }
    }
}

#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[target_feature(enable = "neon")]
unsafe fn transpose_32bit_neon(
    source: *const u32,
    source_stride: usize,
    source_height: usize,
    source_x: usize,
    columns: usize,
    destination: *mut u32,
    destination_stride: usize,
) {
    use core::arch::aarch64::*;

    let vector_rows = source_height / 4 * 4;
    let vector_columns = columns / 4 * 4;
    for row in (0..vector_rows).step_by(4) {
        for column in (0..vector_columns).step_by(4) {
            // SAFETY: the checked wrapper validates every 4-sample load.
            let r0 = unsafe { vld1q_u32(source.add(row * source_stride + source_x + column)) };
            let r1 =
                unsafe { vld1q_u32(source.add((row + 1) * source_stride + source_x + column)) };
            let r2 =
                unsafe { vld1q_u32(source.add((row + 2) * source_stride + source_x + column)) };
            let r3 =
                unsafe { vld1q_u32(source.add((row + 3) * source_stride + source_x + column)) };
            let t0 = vtrn1q_u32(r0, r1);
            let t1 = vtrn2q_u32(r0, r1);
            let t2 = vtrn1q_u32(r2, r3);
            let t3 = vtrn2q_u32(r2, r3);
            let output = [
                vreinterpretq_u32_u64(vtrn1q_u64(
                    vreinterpretq_u64_u32(t0),
                    vreinterpretq_u64_u32(t2),
                )),
                vreinterpretq_u32_u64(vtrn1q_u64(
                    vreinterpretq_u64_u32(t1),
                    vreinterpretq_u64_u32(t3),
                )),
                vreinterpretq_u32_u64(vtrn2q_u64(
                    vreinterpretq_u64_u32(t0),
                    vreinterpretq_u64_u32(t2),
                )),
                vreinterpretq_u32_u64(vtrn2q_u64(
                    vreinterpretq_u64_u32(t1),
                    vreinterpretq_u64_u32(t3),
                )),
            ];
            for (local_column, value) in output.into_iter().enumerate() {
                // SAFETY: the checked wrapper validates every 4-sample store.
                unsafe {
                    vst1q_u32(
                        destination.add((column + local_column) * destination_stride + row),
                        value,
                    );
                }
            }
        }
    }
    for column in 0..vector_columns {
        for row in vector_rows..source_height {
            // SAFETY: scalar tails remain inside checked active rectangles.
            unsafe {
                *destination.add(column * destination_stride + row) =
                    *source.add(row * source_stride + source_x + column);
            }
        }
    }
    for column in vector_columns..columns {
        for row in 0..source_height {
            // SAFETY: scalar tails remain inside checked active rectangles.
            unsafe {
                *destination.add(column * destination_stride + row) =
                    *source.add(row * source_stride + source_x + column);
            }
        }
    }
}

/// Process-selected implementation of the HT cleanup octet kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtCleanupBackend {
    /// Checked scalar conformance implementation.
    Reference,
    /// Portable fast scalar implementation.
    Scalar,
    /// x86-64 AVX2 extraction, reconstruction, and stores.
    Avx2,
    /// x86-64 AVX2 reconstruction with BMI2 bit extraction.
    Avx2Bmi2,
    /// AArch64 NEON reconstruction and stores.
    Neon,
}

impl HtCleanupBackend {
    /// Stable backend label used by benchmark provenance.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Scalar => "scalar",
            Self::Avx2 => "avx2",
            Self::Avx2Bmi2 => "avx2-bmi2",
            Self::Neon => "neon",
        }
    }
}

/// Failure to honor a forced HT backend selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtCleanupBackendError {
    /// `EMUELLA_J2K_HT_BACKEND` named an unknown backend.
    UnknownOverride,
    /// The requested backend is not available on this CPU or build target.
    UnavailableOverride,
}

/// Invalid input to the prepared MagSgn octet kernel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtCleanupOctetError {
    /// An insignificant sample declares embedded magnitude or reduction data.
    InvalidCodewordFlags,
    /// A sample requested more bits than an HT cleanup sample can contain.
    InvalidBitLength,
    /// The prepared input lacks the eight-byte sentinel required by the kernel.
    InputTooShort,
}

/// Coefficients and south-predictor magnitudes produced by one 4x2 octet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtCleanupOctetOutput {
    /// Slot-order coefficients: TL, BL, TR, BR for each adjacent quad.
    pub coefficients: [i32; 8],
    /// Magnitude codes used to update south-neighbour predictors.
    pub predictors: [u32; 8],
    /// Number of MagSgn bits consumed by the octet.
    pub consumed_bits: u16,
}

type PreparedOctetKernel = fn(&[u8], usize, [u8; 8], u8, u8, u32) -> HtCleanupOctetOutput;
type PreparedCodewordOctetKernel =
    fn(&[u8], usize, u16, u16, u16, u16, u32) -> HtCleanupOctetOutput;

/// Cached HT octet implementation and its benchmark provenance.
#[derive(Debug, Clone, Copy)]
pub struct HtCleanupDispatch {
    backend: HtCleanupBackend,
    reconstruct: fn([u32; 8], u8, u32) -> [i32; 8],
    prepared_octet: PreparedOctetKernel,
    prepared_dense_octet: PreparedOctetKernel,
}

impl HtCleanupDispatch {
    /// Selected backend.
    pub const fn backend(self) -> HtCleanupBackend {
        self.backend
    }

    /// Reconstruct eight sign-magnitude cleanup samples.
    #[inline(always)]
    pub fn reconstruct(self, magnitude_codes: [u32; 8], negative_mask: u8, shift: u32) -> [i32; 8] {
        (self.reconstruct)(magnitude_codes, negative_mask, shift)
    }

    /// Extract and reconstruct an octet from a flat, LSB-first, de-stuffed
    /// MagSgn stream. `prepared` must include an eight-byte zero sentinel.
    #[inline]
    pub fn decode_prepared_octet(
        self,
        prepared: &[u8],
        bit_offset: usize,
        bit_lengths: [u8; 8],
        significance_mask: u8,
        embedded_mask: u8,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        self.decode_prepared_octet_with(
            self.prepared_octet,
            prepared,
            bit_offset,
            bit_lengths,
            significance_mask,
            embedded_mask,
            shift,
        )
    }

    /// Dense-stream companion to [`Self::decode_prepared_octet`]. Backends may
    /// use a higher-throughput extraction kernel when block-level profiling
    /// proves that preparation density amortizes it.
    #[inline]
    pub fn decode_prepared_dense_octet(
        self,
        prepared: &[u8],
        bit_offset: usize,
        bit_lengths: [u8; 8],
        significance_mask: u8,
        embedded_mask: u8,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        self.decode_prepared_octet_with(
            self.prepared_dense_octet,
            prepared,
            bit_offset,
            bit_lengths,
            significance_mask,
            embedded_mask,
            shift,
        )
    }

    /// Decode one octet directly from two packed VLC table words and their
    /// UVLC-derived `u` values. This avoids materializing eight scalar lane
    /// lengths before entering an architecture kernel.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn decode_prepared_codeword_octet(
        self,
        prepared: &[u8],
        bit_offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        self.decode_prepared_codeword_octet_with(
            false,
            prepared,
            bit_offset,
            first_codeword,
            second_codeword,
            first_u,
            second_u,
            shift,
        )
    }

    /// Dense-stream companion to [`Self::decode_prepared_codeword_octet`].
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn decode_prepared_dense_codeword_octet(
        self,
        prepared: &[u8],
        bit_offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        self.decode_prepared_codeword_octet_with(
            true,
            prepared,
            bit_offset,
            first_codeword,
            second_codeword,
            first_u,
            second_u,
            shift,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn decode_prepared_octet_with(
        self,
        kernel: PreparedOctetKernel,
        prepared: &[u8],
        bit_offset: usize,
        bit_lengths: [u8; 8],
        significance_mask: u8,
        embedded_mask: u8,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        if bit_lengths.iter().any(|&length| length > 16) {
            return Err(HtCleanupOctetError::InvalidBitLength);
        }
        let consumed = bit_lengths
            .iter()
            .map(|&length| usize::from(length))
            .sum::<usize>();
        let last_byte = (bit_offset + consumed).div_ceil(8);
        if prepared.len() < last_byte.saturating_add(8) {
            return Err(HtCleanupOctetError::InputTooShort);
        }
        Ok(kernel(
            prepared,
            bit_offset,
            bit_lengths,
            significance_mask,
            embedded_mask,
            shift,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn decode_prepared_codeword_octet_with(
        self,
        dense: bool,
        prepared: &[u8],
        bit_offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> Result<HtCleanupOctetOutput, HtCleanupOctetError> {
        let consumed =
            codeword_octet_consumed_bits(first_codeword, second_codeword, first_u, second_u)?;
        let last_byte = (bit_offset + consumed).div_ceil(8);
        if prepared.len() < last_byte.saturating_add(8) {
            return Err(HtCleanupOctetError::InputTooShort);
        }
        let _ = dense;
        let kernel: PreparedCodewordOctetKernel = match self.backend {
            HtCleanupBackend::Reference => decode_prepared_codeword_octet_reference,
            HtCleanupBackend::Scalar => decode_prepared_codeword_octet_scalar,
            #[cfg(all(feature = "simd", target_arch = "x86_64"))]
            HtCleanupBackend::Avx2 => {
                if dense {
                    x86::decode_prepared_codeword_octet_avx2_dense_safe
                } else {
                    x86::decode_prepared_codeword_octet_avx2_safe
                }
            }
            #[cfg(all(feature = "simd", target_arch = "x86_64"))]
            HtCleanupBackend::Avx2Bmi2 => {
                if dense {
                    x86::decode_prepared_codeword_octet_avx2_dense_safe
                } else {
                    x86::decode_prepared_codeword_octet_avx2_bmi2_safe
                }
            }
            #[cfg(all(feature = "simd", target_arch = "aarch64"))]
            HtCleanupBackend::Neon => aarch64::decode_prepared_codeword_octet_neon_safe,
            _ => decode_prepared_codeword_octet_scalar,
        };
        Ok(kernel(
            prepared,
            bit_offset,
            first_codeword,
            second_codeword,
            first_u,
            second_u,
            shift,
        ))
    }
}

const REFERENCE_DISPATCH: HtCleanupDispatch = HtCleanupDispatch {
    backend: HtCleanupBackend::Reference,
    reconstruct: reconstruct_reference,
    prepared_octet: decode_prepared_octet_reference,
    prepared_dense_octet: decode_prepared_octet_reference,
};

const SCALAR_DISPATCH: HtCleanupDispatch = HtCleanupDispatch {
    backend: HtCleanupBackend::Scalar,
    reconstruct: reconstruct_scalar,
    prepared_octet: decode_prepared_octet_scalar,
    prepared_dense_octet: decode_prepared_octet_scalar,
};

#[inline(always)]
fn codeword_quad_consumed_bits(codeword: u16, u_value: u16) -> Result<usize, HtCleanupOctetError> {
    let significance = ((codeword >> 4) & 0xf) as u8;
    let reductions = ((codeword >> 12) & 0xf) as u8;
    if significance == 0 {
        return Ok(0);
    }
    let reduced_count = (significance & reductions).count_ones() as usize;
    let unreduced_count = significance.count_ones() as usize - reduced_count;
    if (reduced_count != 0 && u_value == 0)
        || (unreduced_count != 0 && u_value > 16)
        || (reduced_count != 0 && u_value > 17)
    {
        return Err(HtCleanupOctetError::InvalidBitLength);
    }
    Ok(usize::from(u_value) * significance.count_ones() as usize - reduced_count)
}

#[inline(always)]
fn codeword_octet_consumed_bits(
    first_codeword: u16,
    second_codeword: u16,
    first_u: u16,
    second_u: u16,
) -> Result<usize, HtCleanupOctetError> {
    let first_significance = ((first_codeword >> 4) & 0xf) as u8;
    let first_side_flags =
        ((first_codeword >> 8) & 0xf) as u8 | ((first_codeword >> 12) & 0xf) as u8;
    let second_significance = ((second_codeword >> 4) & 0xf) as u8;
    let second_side_flags =
        ((second_codeword >> 8) & 0xf) as u8 | ((second_codeword >> 12) & 0xf) as u8;
    if first_side_flags & !first_significance != 0 || second_side_flags & !second_significance != 0
    {
        return Err(HtCleanupOctetError::InvalidCodewordFlags);
    }
    let first = codeword_quad_consumed_bits(first_codeword, first_u)?;
    let second = codeword_quad_consumed_bits(second_codeword, second_u)?;
    Ok(first + second)
}

#[inline(always)]
fn codeword_octet_metadata(
    first_codeword: u16,
    second_codeword: u16,
    first_u: u16,
    second_u: u16,
) -> ([u8; 8], u8, u8) {
    let significance =
        ((first_codeword >> 4) & 0xf) as u8 | (((second_codeword >> 4) & 0xf) as u8) << 4;
    let embedded =
        ((first_codeword >> 8) & 0xf) as u8 | (((second_codeword >> 8) & 0xf) as u8) << 4;
    let reductions =
        ((first_codeword >> 12) & 0xf) as u8 | (((second_codeword >> 12) & 0xf) as u8) << 4;
    let mut lengths = [0_u8; 8];
    for (lane, length) in lengths.iter_mut().enumerate() {
        if significance & (1 << lane) != 0 {
            let u_value = if lane < 4 { first_u } else { second_u };
            *length = (u_value - u16::from(reductions >> lane & 1)) as u8;
        }
    }
    (lengths, significance, embedded)
}

/// Resolve the process-wide HT cleanup backend once.
#[cfg(feature = "std")]
pub fn ht_cleanup_dispatch() -> Result<&'static HtCleanupDispatch, HtCleanupBackendError> {
    static DISPATCH: OnceLock<Result<HtCleanupDispatch, HtCleanupBackendError>> = OnceLock::new();
    DISPATCH
        .get_or_init(resolve_dispatch)
        .as_ref()
        .map_err(|&error| error)
}

/// Resolve one explicitly requested backend without changing cached process
/// selection. Intended for differential tests and focused benchmarks.
pub fn ht_cleanup_dispatch_for_backend(
    backend: HtCleanupBackend,
) -> Result<HtCleanupDispatch, HtCleanupBackendError> {
    match backend {
        HtCleanupBackend::Reference => Ok(REFERENCE_DISPATCH),
        HtCleanupBackend::Scalar => Ok(SCALAR_DISPATCH),
        HtCleanupBackend::Avx2 => avx2_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride),
        HtCleanupBackend::Avx2Bmi2 => {
            avx2_bmi2_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride)
        }
        HtCleanupBackend::Neon => neon_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride),
    }
}

/// Return the portable dispatch in `no_std` builds.
#[cfg(not(feature = "std"))]
pub fn ht_cleanup_dispatch() -> Result<&'static HtCleanupDispatch, HtCleanupBackendError> {
    Ok(&SCALAR_DISPATCH)
}

#[cfg(feature = "std")]
fn resolve_dispatch() -> Result<HtCleanupDispatch, HtCleanupBackendError> {
    let backend = std::env::var("EMUELLA_J2K_HT_BACKEND").ok();
    resolve_dispatch_for_override(backend.as_deref())
}

#[cfg(feature = "std")]
fn resolve_dispatch_for_override(
    backend: Option<&str>,
) -> Result<HtCleanupDispatch, HtCleanupBackendError> {
    match backend {
        None | Some("") | Some("auto") => Ok(detect_dispatch()),
        Some("reference") => Ok(REFERENCE_DISPATCH),
        Some("scalar") => Ok(SCALAR_DISPATCH),
        Some("avx2") => avx2_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride),
        Some("avx2-bmi2") => avx2_bmi2_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride),
        Some("neon") => neon_dispatch().ok_or(HtCleanupBackendError::UnavailableOverride),
        Some(_) => Err(HtCleanupBackendError::UnknownOverride),
    }
}

#[cfg(feature = "std")]
fn detect_dispatch() -> HtCleanupDispatch {
    // Auto remains scalar until the complete extraction/store benchmark and
    // end-to-end corpus guardrail both prove an architecture backend wins.
    SCALAR_DISPATCH
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
fn avx2_dispatch() -> Option<HtCleanupDispatch> {
    std::arch::is_x86_feature_detected!("avx2").then_some(HtCleanupDispatch {
        backend: HtCleanupBackend::Avx2,
        reconstruct: x86::reconstruct_avx2_safe,
        prepared_octet: x86::decode_prepared_octet_avx2_safe,
        prepared_dense_octet: x86::decode_prepared_octet_avx2_dense_safe,
    })
}

#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
fn avx2_dispatch() -> Option<HtCleanupDispatch> {
    None
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
fn avx2_bmi2_dispatch() -> Option<HtCleanupDispatch> {
    (std::arch::is_x86_feature_detected!("avx2") && std::arch::is_x86_feature_detected!("bmi2"))
        .then_some(HtCleanupDispatch {
            backend: HtCleanupBackend::Avx2Bmi2,
            reconstruct: x86::reconstruct_avx2_safe,
            prepared_octet: x86::decode_prepared_octet_avx2_bmi2_safe,
            prepared_dense_octet: x86::decode_prepared_octet_avx2_bmi2_safe,
        })
}

#[cfg(not(all(feature = "simd", target_arch = "x86_64")))]
fn avx2_bmi2_dispatch() -> Option<HtCleanupDispatch> {
    None
}

#[cfg(all(feature = "simd", target_arch = "aarch64"))]
fn neon_dispatch() -> Option<HtCleanupDispatch> {
    Some(HtCleanupDispatch {
        backend: HtCleanupBackend::Neon,
        reconstruct: aarch64::reconstruct_neon_safe,
        prepared_octet: aarch64::decode_prepared_octet_neon_safe,
        prepared_dense_octet: aarch64::decode_prepared_octet_neon_safe,
    })
}

#[cfg(not(all(feature = "simd", target_arch = "aarch64")))]
fn neon_dispatch() -> Option<HtCleanupDispatch> {
    None
}

#[inline(always)]
fn reconstruct_reference(magnitude_codes: [u32; 8], negative_mask: u8, shift: u32) -> [i32; 8] {
    let mut output = [0_i32; 8];
    for index in 0..8 {
        let code = magnitude_codes[index];
        if code != 0 {
            let scaled = (code + 2).wrapping_shl(shift);
            let magnitude = (scaled & 0x7fff_ffff) as i32;
            output[index] = if negative_mask & (1 << index) != 0 || scaled & 0x8000_0000 != 0 {
                -magnitude
            } else {
                magnitude
            };
        }
    }
    output
}

#[inline(always)]
fn reconstruct_scalar(magnitude_codes: [u32; 8], negative_mask: u8, shift: u32) -> [i32; 8] {
    reconstruct_reference(magnitude_codes, negative_mask, shift)
}

#[inline(always)]
fn load_window(prepared: &[u8], bit_offset: usize) -> u64 {
    let byte_offset = bit_offset / 8;
    let bytes: [u8; 8] = prepared[byte_offset..byte_offset + 8].try_into().unwrap();
    u64::from_le_bytes(bytes) >> (bit_offset % 8)
}

#[inline(always)]
fn extract_scalar(prepared: &[u8], bit_offset: usize, length: u8) -> u32 {
    if length == 0 {
        return 0;
    }
    let mask = (1_u64 << length) - 1;
    (load_window(prepared, bit_offset) & mask) as u32
}

fn decode_prepared_octet_reference(
    prepared: &[u8],
    bit_offset: usize,
    lengths: [u8; 8],
    significance: u8,
    embedded: u8,
    shift: u32,
) -> HtCleanupOctetOutput {
    decode_prepared_octet_with(
        prepared,
        bit_offset,
        lengths,
        significance,
        embedded,
        shift,
        extract_scalar,
        reconstruct_reference,
    )
}

fn decode_prepared_octet_scalar(
    prepared: &[u8],
    bit_offset: usize,
    lengths: [u8; 8],
    significance: u8,
    embedded: u8,
    shift: u32,
) -> HtCleanupOctetOutput {
    decode_prepared_octet_with(
        prepared,
        bit_offset,
        lengths,
        significance,
        embedded,
        shift,
        extract_scalar,
        reconstruct_scalar,
    )
}

fn decode_prepared_codeword_octet_reference(
    prepared: &[u8],
    bit_offset: usize,
    first_codeword: u16,
    second_codeword: u16,
    first_u: u16,
    second_u: u16,
    shift: u32,
) -> HtCleanupOctetOutput {
    let (lengths, significance, embedded) =
        codeword_octet_metadata(first_codeword, second_codeword, first_u, second_u);
    decode_prepared_octet_reference(prepared, bit_offset, lengths, significance, embedded, shift)
}

fn decode_prepared_codeword_octet_scalar(
    prepared: &[u8],
    bit_offset: usize,
    first_codeword: u16,
    second_codeword: u16,
    first_u: u16,
    second_u: u16,
    shift: u32,
) -> HtCleanupOctetOutput {
    let (lengths, significance, embedded) =
        codeword_octet_metadata(first_codeword, second_codeword, first_u, second_u);
    decode_prepared_octet_scalar(prepared, bit_offset, lengths, significance, embedded, shift)
}

#[allow(clippy::too_many_arguments)]
fn decode_prepared_octet_with(
    prepared: &[u8],
    bit_offset: usize,
    lengths: [u8; 8],
    significance: u8,
    embedded: u8,
    shift: u32,
    extract: fn(&[u8], usize, u8) -> u32,
    reconstruct: fn([u32; 8], u8, u32) -> [i32; 8],
) -> HtCleanupOctetOutput {
    let mut predictors = [0_u32; 8];
    let mut negative = 0_u8;
    let mut offset = bit_offset;
    for lane in 0..8 {
        let length = lengths[lane];
        let raw = extract(prepared, offset, length);
        offset += usize::from(length);
        if significance & (1 << lane) != 0 {
            predictors[lane] = raw | (u32::from(embedded >> lane & 1) << length) | 1;
            negative |= ((raw & 1) as u8) << lane;
        }
    }
    HtCleanupOctetOutput {
        coefficients: reconstruct(predictors, negative, shift),
        predictors,
        consumed_bits: (offset - bit_offset) as u16,
    }
}

#[cfg(all(feature = "simd", target_arch = "x86_64"))]
mod x86 {
    use super::*;
    use core::arch::x86_64::*;

    pub(super) fn reconstruct_avx2_safe(codes: [u32; 8], negative: u8, shift: u32) -> [i32; 8] {
        // SAFETY: installed only after AVX2 runtime detection.
        unsafe { reconstruct_avx2(codes, negative, shift) }
    }

    pub(super) fn decode_prepared_octet_avx2_safe(
        prepared: &[u8],
        offset: usize,
        lengths: [u8; 8],
        significance: u8,
        embedded: u8,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        // SAFETY: dispatch verifies AVX2. The public wrapper proves the padded
        // source range, and this function uses only validated slice loads.
        unsafe {
            decode_prepared_octet_avx2(
                prepared,
                offset,
                lengths,
                significance,
                embedded,
                shift,
                false,
            )
        }
    }

    pub(super) fn decode_prepared_octet_avx2_dense_safe(
        prepared: &[u8],
        offset: usize,
        lengths: [u8; 8],
        significance: u8,
        embedded: u8,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        // SAFETY: dispatch verifies AVX2. The public wrapper proves the padded
        // source range, and gather indices are relative to that checked base.
        unsafe {
            crate::openjph_ht_cleanup::decode_prepared_octet_avx2_gather(
                prepared,
                offset,
                lengths,
                significance,
                embedded,
                shift,
            )
        }
    }

    pub(super) fn decode_prepared_codeword_octet_avx2_safe(
        prepared: &[u8],
        offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        let (lengths, significance, embedded) =
            codeword_octet_metadata(first_codeword, second_codeword, first_u, second_u);
        decode_prepared_octet_avx2_safe(prepared, offset, lengths, significance, embedded, shift)
    }

    pub(super) fn decode_prepared_codeword_octet_avx2_dense_safe(
        prepared: &[u8],
        offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        // SAFETY: dispatch verifies AVX2. The public wrapper validates packed
        // flags, bit lengths, and the padded source range used by the gather.
        unsafe {
            crate::openjph_ht_cleanup::decode_prepared_codeword_octet_avx2_gather(
                prepared,
                offset,
                first_codeword,
                second_codeword,
                first_u,
                second_u,
                shift,
            )
        }
    }

    pub(super) fn decode_prepared_octet_avx2_bmi2_safe(
        prepared: &[u8],
        offset: usize,
        lengths: [u8; 8],
        significance: u8,
        embedded: u8,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        // SAFETY: dispatch verifies AVX2 and BMI2; source bounds were checked.
        unsafe {
            decode_prepared_octet_avx2(
                prepared,
                offset,
                lengths,
                significance,
                embedded,
                shift,
                true,
            )
        }
    }

    pub(super) fn decode_prepared_codeword_octet_avx2_bmi2_safe(
        prepared: &[u8],
        offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        let (lengths, significance, embedded) =
            codeword_octet_metadata(first_codeword, second_codeword, first_u, second_u);
        decode_prepared_octet_avx2_bmi2_safe(
            prepared,
            offset,
            lengths,
            significance,
            embedded,
            shift,
        )
    }

    #[target_feature(enable = "avx2")]
    unsafe fn reconstruct_avx2(codes: [u32; 8], negative: u8, shift: u32) -> [i32; 8] {
        let mut output = [0_i32; 8];
        let codes = unsafe { _mm256_loadu_si256(codes.as_ptr().cast()) };
        let nonzero = _mm256_cmpgt_epi32(codes, _mm256_setzero_si256());
        let shifted = _mm256_sllv_epi32(
            _mm256_add_epi32(codes, _mm256_set1_epi32(2)),
            _mm256_set1_epi32(shift as i32),
        );
        let magnitude = _mm256_and_si256(shifted, _mm256_set1_epi32(0x7fff_ffff));
        let sign_bits = _mm256_setr_epi32(1, 2, 4, 8, 16, 32, 64, 128);
        let explicit = _mm256_cmpeq_epi32(
            _mm256_and_si256(_mm256_set1_epi32(i32::from(negative)), sign_bits),
            sign_bits,
        );
        let sign = _mm256_or_si256(explicit, _mm256_srai_epi32(shifted, 31));
        let signed = _mm256_sub_epi32(_mm256_xor_si256(magnitude, sign), sign);
        unsafe {
            _mm256_storeu_si256(
                output.as_mut_ptr().cast(),
                _mm256_and_si256(signed, nonzero),
            )
        };
        output
    }

    #[target_feature(enable = "avx2")]
    unsafe fn decode_prepared_octet_avx2(
        prepared: &[u8],
        bit_offset: usize,
        lengths: [u8; 8],
        significance: u8,
        embedded: u8,
        shift: u32,
        bmi2: bool,
    ) -> HtCleanupOctetOutput {
        let mut predictors = [0_u32; 8];
        let mut negative = 0_u8;
        let mut offset = bit_offset;
        for lane in 0..8 {
            let length = lengths[lane];
            let raw = if bmi2 {
                unsafe { extract_bmi2(prepared, offset, length) }
            } else {
                extract_scalar(prepared, offset, length)
            };
            offset += usize::from(length);
            if significance & (1 << lane) != 0 {
                predictors[lane] = raw | (u32::from(embedded >> lane & 1) << length) | 1;
                negative |= ((raw & 1) as u8) << lane;
            }
        }
        HtCleanupOctetOutput {
            coefficients: unsafe { reconstruct_avx2(predictors, negative, shift) },
            predictors,
            consumed_bits: (offset - bit_offset) as u16,
        }
    }

    #[target_feature(enable = "bmi2")]
    unsafe fn extract_bmi2(prepared: &[u8], bit_offset: usize, length: u8) -> u32 {
        if length == 0 {
            return 0;
        }
        let window = load_window(prepared, bit_offset);
        let mask = (1_u64 << length) - 1;
        _pext_u64(window, mask) as u32
    }
}

#[cfg(all(feature = "simd", target_arch = "aarch64"))]
mod aarch64 {
    use super::*;
    use core::arch::aarch64::*;

    pub(super) fn reconstruct_neon_safe(codes: [u32; 8], negative: u8, shift: u32) -> [i32; 8] {
        // SAFETY: AArch64 guarantees NEON support.
        unsafe { reconstruct_neon(codes, negative, shift) }
    }

    pub(super) fn decode_prepared_octet_neon_safe(
        prepared: &[u8],
        offset: usize,
        lengths: [u8; 8],
        significance: u8,
        embedded: u8,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        let mut output = decode_prepared_octet_with(
            prepared,
            offset,
            lengths,
            significance,
            embedded,
            shift,
            extract_scalar,
            reconstruct_scalar,
        );
        output.coefficients = reconstruct_neon_safe(
            output.predictors,
            output
                .coefficients
                .iter()
                .enumerate()
                .fold(0_u8, |mask, (lane, &value)| {
                    mask | (u8::from(value < 0) << lane)
                }),
            shift,
        );
        output
    }

    pub(super) fn decode_prepared_codeword_octet_neon_safe(
        prepared: &[u8],
        offset: usize,
        first_codeword: u16,
        second_codeword: u16,
        first_u: u16,
        second_u: u16,
        shift: u32,
    ) -> HtCleanupOctetOutput {
        let (lengths, significance, embedded) =
            codeword_octet_metadata(first_codeword, second_codeword, first_u, second_u);
        decode_prepared_octet_neon_safe(prepared, offset, lengths, significance, embedded, shift)
    }

    #[target_feature(enable = "neon")]
    unsafe fn reconstruct_neon(codes: [u32; 8], negative: u8, shift: u32) -> [i32; 8] {
        let mut output = [0_i32; 8];
        for half in 0..2 {
            let base = half * 4;
            let values = unsafe { vld1q_u32(codes.as_ptr().add(base)) };
            let adjusted = vaddq_u32(values, vdupq_n_u32(2));
            let shifted = vshlq_u32(adjusted, vdupq_n_s32(shift as i32));
            let magnitude = vandq_u32(shifted, vdupq_n_u32(0x7fff_ffff));
            let mut lanes = [0_u32; 4];
            let mut shifted_lanes = [0_u32; 4];
            unsafe { vst1q_u32(lanes.as_mut_ptr(), magnitude) };
            unsafe { vst1q_u32(shifted_lanes.as_mut_ptr(), shifted) };
            for lane in 0..4 {
                if codes[base + lane] != 0 {
                    let is_negative = negative & (1 << (base + lane)) != 0
                        || shifted_lanes[lane] & 0x8000_0000 != 0;
                    output[base + lane] = if is_negative {
                        -(lanes[lane] as i32)
                    } else {
                        lanes[lane] as i32
                    };
                }
            }
        }
        output
    }
}
