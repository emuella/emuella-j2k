//! Direct bounded inverse synthesis owned by the transform crate.
//!
//! Planning expands every requested level by a transform-specific halo. The
//! production kernels assemble those compact low/high intervals into dense
//! lines, invoke the same scalar lifting loops as full-plane reconstruction,
//! and retain only the rectangle consumed by the next level.

use alloc::vec::Vec;
use core::mem::size_of;

use super::{
    Irreversible97Edges, Reversible53Edges, TransformBand, TransformError, WaveletTransform,
    inverse_irreversible_9_7_line, transform_line_inverse_bounded,
};

/// Checked half-open rectangle in one component or subband coordinate grid.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AxisAlignedRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl AxisAlignedRegion {
    pub fn end_x(self) -> Result<u32, TransformError> {
        self.x
            .checked_add(self.width)
            .ok_or(TransformError::SizeOverflow)
    }

    pub fn end_y(self) -> Result<u32, TransformError> {
        self.y
            .checked_add(self.height)
            .ok_or(TransformError::SizeOverflow)
    }

    pub fn sample_count(self) -> u64 {
        u64::from(self.width).saturating_mul(u64::from(self.height))
    }

    pub fn intersects(self, other: Self) -> Result<bool, TransformError> {
        Ok(self.x < other.end_x()?
            && other.x < self.end_x()?
            && self.y < other.end_y()?
            && other.y < self.end_y()?)
    }
}

/// Transform-owned subband identity for one packed coefficient plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowSubband {
    LowLow,
    HighLow,
    LowHigh,
    HighHigh,
}

/// Required coefficient rectangles for one inverse synthesis level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolutionWindowPlan {
    pub resolution: u8,
    pub output: AxisAlignedRegion,
    pub low_low: AxisAlignedRegion,
    pub high_low: AxisAlignedRegion,
    pub low_high: AxisAlignedRegion,
    pub high_high: AxisAlignedRegion,
}

/// Immutable transform-owned dependency plan for one retained output window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSynthesisPlan {
    pub width: u32,
    pub height: u32,
    pub decomposition_levels: u8,
    pub transform: WaveletTransform,
    pub output_region: AxisAlignedRegion,
    pub lowest_low_low: AxisAlignedRegion,
    /// Whether the measured small-window policy selects bounded synthesis.
    pub use_windowed_synthesis: bool,
    /// Resolution records in ascending synthesis order, starting at one.
    pub levels: Vec<ResolutionWindowPlan>,
}

impl WindowSynthesisPlan {
    pub fn required_subband_region(
        &self,
        resolution: u8,
        subband: WindowSubband,
    ) -> Option<AxisAlignedRegion> {
        if resolution == 0 {
            return (subband == WindowSubband::LowLow).then_some(self.lowest_low_low);
        }
        let level = self.levels.get(usize::from(resolution.checked_sub(1)?))?;
        (level.resolution == resolution).then_some(match subband {
            WindowSubband::LowLow => level.low_low,
            WindowSubband::HighLow => level.high_low,
            WindowSubband::LowHigh => level.low_high,
            WindowSubband::HighHigh => level.high_high,
        })
    }
}

fn ceil_div(value: u32, divisor: u32) -> Result<u32, TransformError> {
    if divisor == 0 {
        return Err(TransformError::InvalidWindow);
    }
    Ok(value / divisor + u32::from(!value.is_multiple_of(divisor)))
}

fn resolution_dimensions(
    width: u32,
    height: u32,
    decomposition_levels: u8,
    resolution: u8,
) -> Result<(u32, u32), TransformError> {
    if resolution > decomposition_levels {
        return Err(TransformError::InvalidWindow);
    }
    let scale = 1_u32
        .checked_shl(u32::from(decomposition_levels - resolution))
        .ok_or(TransformError::SizeOverflow)?;
    Ok((ceil_div(width, scale)?, ceil_div(height, scale)?))
}

fn synthesis_band_axis(
    output_start: u32,
    output_end: u32,
    band_len: u32,
    support_radius: u32,
) -> Result<(u32, u32), TransformError> {
    if band_len == 0 {
        return Ok((0, 0));
    }
    let start = (output_start / 2).saturating_sub(support_radius);
    let end = ceil_div(output_end, 2)?
        .saturating_add(support_radius)
        .min(band_len);
    let start = start.min(end);
    Ok((start, end - start))
}

/// Build an exact checked bounded-synthesis dependency plan.
pub fn plan_window_synthesis(
    width: u32,
    height: u32,
    decomposition_levels: u8,
    output_region: AxisAlignedRegion,
    transform: WaveletTransform,
) -> Result<WindowSynthesisPlan, TransformError> {
    if width == 0
        || height == 0
        || output_region.width == 0
        || output_region.height == 0
        || output_region.end_x()? > width
        || output_region.end_y()? > height
    {
        return Err(TransformError::InvalidWindow);
    }
    let support_radius = match transform {
        WaveletTransform::Reversible53 => 2,
        WaveletTransform::Irreversible97 => 4,
    };
    let mut desired = output_region;
    let mut descending = Vec::with_capacity(usize::from(decomposition_levels));
    for resolution in (1..=decomposition_levels).rev() {
        let (level_width, level_height) =
            resolution_dimensions(width, height, decomposition_levels, resolution)?;
        if desired.end_x()? > level_width || desired.end_y()? > level_height {
            return Err(TransformError::InvalidWindow);
        }
        let (low_width, low_height) =
            resolution_dimensions(width, height, decomposition_levels, resolution - 1)?;
        let high_width = level_width
            .checked_sub(low_width)
            .ok_or(TransformError::SizeOverflow)?;
        let high_height = level_height
            .checked_sub(low_height)
            .ok_or(TransformError::SizeOverflow)?;
        let desired_end_x = desired.end_x()?;
        let desired_end_y = desired.end_y()?;
        let (low_x, low_width_needed) =
            synthesis_band_axis(desired.x, desired_end_x, low_width, support_radius)?;
        let (high_x, high_width_needed) =
            synthesis_band_axis(desired.x, desired_end_x, high_width, support_radius)?;
        let (low_y, low_height_needed) =
            synthesis_band_axis(desired.y, desired_end_y, low_height, support_radius)?;
        let (high_y, high_height_needed) =
            synthesis_band_axis(desired.y, desired_end_y, high_height, support_radius)?;
        let low_low = AxisAlignedRegion {
            x: low_x,
            y: low_y,
            width: low_width_needed,
            height: low_height_needed,
        };
        descending.push(ResolutionWindowPlan {
            resolution,
            output: desired,
            low_low,
            high_low: AxisAlignedRegion {
                x: low_width
                    .checked_add(high_x)
                    .ok_or(TransformError::SizeOverflow)?,
                y: low_y,
                width: high_width_needed,
                height: low_height_needed,
            },
            low_high: AxisAlignedRegion {
                x: low_x,
                y: low_height
                    .checked_add(high_y)
                    .ok_or(TransformError::SizeOverflow)?,
                width: low_width_needed,
                height: high_height_needed,
            },
            high_high: AxisAlignedRegion {
                x: low_width
                    .checked_add(high_x)
                    .ok_or(TransformError::SizeOverflow)?,
                y: low_height
                    .checked_add(high_y)
                    .ok_or(TransformError::SizeOverflow)?,
                width: high_width_needed,
                height: high_height_needed,
            },
        });
        desired = low_low;
    }
    descending.reverse();
    let output_area = output_region.sample_count();
    let estimated_window_work = descending
        .iter()
        .fold(output_area.saturating_mul(2), |total, level| {
            total.saturating_add(level.low_low.sample_count())
        });
    let full_area = u64::from(width).saturating_mul(u64::from(height));
    Ok(WindowSynthesisPlan {
        width,
        height,
        decomposition_levels,
        transform,
        output_region,
        lowest_low_low: desired,
        // This compatibility hint intentionally depends on decomposition and
        // halo-expanded work, not on a fixed requested-area percentage. The
        // codestream scheduler applies the richer workspace/lifting/copy
        // crossover model when both routes are forceable.
        use_windowed_synthesis: estimated_window_work < full_area,
        levels: descending,
    })
}

fn region_len(region: AxisAlignedRegion) -> Result<usize, TransformError> {
    usize::try_from(region.sample_count()).map_err(|_| TransformError::SizeOverflow)
}

/// One compact dense coefficient band.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowCoefficientBand<T> {
    pub region: AxisAlignedRegion,
    pub values: Vec<T>,
}

impl<T: Copy + Default> WindowCoefficientBand<T> {
    fn new(region: AxisAlignedRegion) -> Result<Self, TransformError> {
        Ok(Self {
            region,
            values: alloc::vec![T::default(); region_len(region)?],
        })
    }

    fn reset(&mut self, region: AxisAlignedRegion) -> Result<(), TransformError> {
        self.region = region;
        self.values.resize(region_len(region)?, T::default());
        self.values.fill(T::default());
        Ok(())
    }

    fn clear(&mut self) {
        self.region = AxisAlignedRegion::default();
        self.values.clear();
    }

    fn row(&self, y: u32) -> Result<&[T], TransformError> {
        dense_row(&self.values, self.region, y)
    }

    fn get(&self, x: u32, y: u32) -> Result<T, TransformError> {
        let column = x
            .checked_sub(self.region.x)
            .filter(|column| *column < self.region.width)
            .ok_or(TransformError::InvalidWindow)?;
        let row = y
            .checked_sub(self.region.y)
            .filter(|row| *row < self.region.height)
            .ok_or(TransformError::InvalidWindow)?;
        let index = usize::try_from(row)
            .ok()
            .and_then(|row| row.checked_mul(self.region.width as usize))
            .and_then(|offset| {
                usize::try_from(column)
                    .ok()
                    .and_then(|column| offset.checked_add(column))
            })
            .ok_or(TransformError::SizeOverflow)?;
        self.values
            .get(index)
            .copied()
            .ok_or(TransformError::InvalidWindow)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WindowLevelCoefficients<T> {
    resolution: u8,
    high_low: WindowCoefficientBand<T>,
    low_high: WindowCoefficientBand<T>,
    high_high: WindowCoefficientBand<T>,
}

/// Compact transform-owned coefficient rectangles for one window plan.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowCoefficientPlane<T> {
    lowest_low_low: WindowCoefficientBand<T>,
    levels: Vec<WindowLevelCoefficients<T>>,
}

impl<T: Copy + Default> WindowCoefficientPlane<T> {
    pub fn new(plan: &WindowSynthesisPlan) -> Result<Self, TransformError> {
        let lowest_low_low = WindowCoefficientBand::new(plan.lowest_low_low)?;
        let levels = plan
            .levels
            .iter()
            .map(|level| {
                Ok(WindowLevelCoefficients {
                    resolution: level.resolution,
                    high_low: WindowCoefficientBand::new(level.high_low)?,
                    low_high: WindowCoefficientBand::new(level.low_high)?,
                    high_high: WindowCoefficientBand::new(level.high_high)?,
                })
            })
            .collect::<Result<Vec<_>, TransformError>>()?;
        Ok(Self {
            lowest_low_low,
            levels,
        })
    }

    /// Reconfigure and zero this compact plane while retaining every usable
    /// allocation from prior plans.
    pub fn reset_for_plan(&mut self, plan: &WindowSynthesisPlan) -> Result<(), TransformError> {
        self.lowest_low_low.reset(plan.lowest_low_low)?;
        self.levels.truncate(plan.levels.len());
        for (index, planned) in plan.levels.iter().enumerate() {
            if let Some(level) = self.levels.get_mut(index) {
                level.resolution = planned.resolution;
                level.high_low.reset(planned.high_low)?;
                level.low_high.reset(planned.low_high)?;
                level.high_high.reset(planned.high_high)?;
            } else {
                self.levels.push(WindowLevelCoefficients {
                    resolution: planned.resolution,
                    high_low: WindowCoefficientBand::new(planned.high_low)?,
                    low_high: WindowCoefficientBand::new(planned.low_high)?,
                    high_high: WindowCoefficientBand::new(planned.high_high)?,
                });
            }
        }
        Ok(())
    }

    /// Clear logical coefficient contents while preserving backing capacity.
    pub fn clear(&mut self) {
        self.lowest_low_low.clear();
        for level in &mut self.levels {
            level.high_low.clear();
            level.low_high.clear();
            level.high_high.clear();
        }
    }

    /// Capacity-based retained bytes, excluding allocator metadata.
    pub fn retained_heap_bytes(&self) -> u64 {
        let coefficient_values =
            self.lowest_low_low
                .values
                .capacity()
                .saturating_add(self.levels.iter().fold(0_usize, |total, level| {
                    total
                        .saturating_add(level.high_low.values.capacity())
                        .saturating_add(level.low_high.values.capacity())
                        .saturating_add(level.high_high.values.capacity())
                }));
        (coefficient_values as u64)
            .saturating_mul(size_of::<T>() as u64)
            .saturating_add(
                (self.levels.capacity() as u64)
                    .saturating_mul(size_of::<WindowLevelCoefficients<T>>() as u64),
            )
    }

    pub fn sample_count(&self) -> u64 {
        self.lowest_low_low.values.len() as u64
            + self.levels.iter().fold(0_u64, |total, level| {
                total
                    .saturating_add(level.high_low.values.len() as u64)
                    .saturating_add(level.low_high.values.len() as u64)
                    .saturating_add(level.high_high.values.len() as u64)
            })
    }

    pub fn band(
        &self,
        resolution: u8,
        subband: WindowSubband,
    ) -> Result<&WindowCoefficientBand<T>, TransformError> {
        if resolution == 0 && subband == WindowSubband::LowLow {
            return Ok(&self.lowest_low_low);
        }
        let level = self
            .levels
            .get(usize::from(
                resolution
                    .checked_sub(1)
                    .ok_or(TransformError::InvalidWindow)?,
            ))
            .filter(|level| level.resolution == resolution)
            .ok_or(TransformError::InvalidWindow)?;
        match subband {
            WindowSubband::HighLow => Ok(&level.high_low),
            WindowSubband::LowHigh => Ok(&level.low_high),
            WindowSubband::HighHigh => Ok(&level.high_high),
            WindowSubband::LowLow => Err(TransformError::InvalidWindow),
        }
    }

    pub fn band_mut(
        &mut self,
        resolution: u8,
        subband: WindowSubband,
    ) -> Result<&mut WindowCoefficientBand<T>, TransformError> {
        if resolution == 0 && subband == WindowSubband::LowLow {
            return Ok(&mut self.lowest_low_low);
        }
        let level = self
            .levels
            .get_mut(usize::from(
                resolution
                    .checked_sub(1)
                    .ok_or(TransformError::InvalidWindow)?,
            ))
            .filter(|level| level.resolution == resolution)
            .ok_or(TransformError::InvalidWindow)?;
        match subband {
            WindowSubband::HighLow => Ok(&mut level.high_low),
            WindowSubband::LowHigh => Ok(&mut level.low_high),
            WindowSubband::HighHigh => Ok(&mut level.high_high),
            WindowSubband::LowLow => Err(TransformError::InvalidWindow),
        }
    }

    pub fn get(
        &self,
        resolution: u8,
        subband: WindowSubband,
        x: u32,
        y: u32,
    ) -> Result<T, TransformError> {
        self.band(resolution, subband)?.get(x, y)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowSynthesisWork {
    pub coefficients_loaded: u64,
    pub horizontal_values: u64,
    pub vertical_values: u64,
    pub lifting_updates: u64,
    pub output_samples: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowSynthesisStageTimings {
    pub horizontal_ns: u128,
    pub vertical_ns: u128,
    pub level_preparation_ns: u128,
    pub horizontal_kernel_ns: u128,
    pub vertical_kernel_ns: u128,
    pub horizontal_startup_and_barrier_ns: u128,
    pub vertical_startup_and_barrier_ns: u128,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowSynthesisReport {
    pub work: WindowSynthesisWork,
    pub stages: WindowSynthesisStageTimings,
    pub peak_value_bytes: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct WindowWorkspaceCapacities {
    pub current: usize,
    pub next: usize,
    pub horizontal_low: usize,
    pub horizontal_high: usize,
    pub line: usize,
    pub line_work: usize,
    pub phase_lines: usize,
    pub phase_line_work: usize,
    pub vertical_columns: usize,
}

/// Reusable direct-window synthesis storage.
#[derive(Debug, Clone)]
pub struct WindowSynthesisWorkspace<T> {
    current_region: AxisAlignedRegion,
    current: Vec<T>,
    next: Vec<T>,
    horizontal_low: Vec<T>,
    horizontal_high: Vec<T>,
    line: Vec<T>,
    line_work: Vec<T>,
    phase_lines: Vec<T>,
    phase_line_work: Vec<T>,
    vertical_columns: Vec<T>,
}

impl<T: Copy + Default> Default for WindowSynthesisWorkspace<T> {
    fn default() -> Self {
        Self {
            current_region: AxisAlignedRegion::default(),
            current: Vec::new(),
            next: Vec::new(),
            horizontal_low: Vec::new(),
            horizontal_high: Vec::new(),
            line: Vec::new(),
            line_work: Vec::new(),
            phase_lines: Vec::new(),
            phase_line_work: Vec::new(),
            vertical_columns: Vec::new(),
        }
    }
}

impl<T: Copy + Default> WindowSynthesisWorkspace<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn capacities(&self) -> WindowWorkspaceCapacities {
        WindowWorkspaceCapacities {
            current: self.current.capacity(),
            next: self.next.capacity(),
            horizontal_low: self.horizontal_low.capacity(),
            horizontal_high: self.horizontal_high.capacity(),
            line: self.line.capacity(),
            line_work: self.line_work.capacity(),
            phase_lines: self.phase_lines.capacity(),
            phase_line_work: self.phase_line_work.capacity(),
            vertical_columns: self.vertical_columns.capacity(),
        }
    }

    pub fn retained_heap_bytes(&self) -> u64 {
        let values = self
            .current
            .capacity()
            .saturating_add(self.next.capacity())
            .saturating_add(self.horizontal_low.capacity())
            .saturating_add(self.horizontal_high.capacity())
            .saturating_add(self.line.capacity())
            .saturating_add(self.line_work.capacity())
            .saturating_add(self.phase_lines.capacity())
            .saturating_add(self.phase_line_work.capacity())
            .saturating_add(self.vertical_columns.capacity());
        (values as u64).saturating_mul(size_of::<T>() as u64)
    }

    pub fn reserve_for_plan(&mut self, plan: &WindowSynthesisPlan) -> Result<(), TransformError> {
        let mut max_plane = region_len(plan.lowest_low_low)?;
        let mut max_horizontal_low = 0_usize;
        let mut max_horizontal_high = 0_usize;
        let mut max_line = 1_usize;
        for level in &plan.levels {
            max_plane = max_plane.max(region_len(level.output)?);
            let width =
                usize::try_from(level.output.width).map_err(|_| TransformError::SizeOverflow)?;
            max_horizontal_low = max_horizontal_low.max(
                width
                    .checked_mul(
                        usize::try_from(level.low_low.height)
                            .map_err(|_| TransformError::SizeOverflow)?,
                    )
                    .ok_or(TransformError::SizeOverflow)?,
            );
            max_horizontal_high = max_horizontal_high.max(
                width
                    .checked_mul(
                        usize::try_from(level.low_high.height)
                            .map_err(|_| TransformError::SizeOverflow)?,
                    )
                    .ok_or(TransformError::SizeOverflow)?,
            );
            for (low, high) in [
                (level.low_low, level.high_low),
                (level.low_high, level.high_high),
            ] {
                max_line = max_line.max(
                    usize::try_from(low.width)
                        .ok()
                        .and_then(|low| {
                            usize::try_from(high.width)
                                .ok()
                                .and_then(|high| low.checked_add(high))
                        })
                        .ok_or(TransformError::SizeOverflow)?,
                );
            }
            max_line = max_line.max(
                usize::try_from(level.low_low.height)
                    .ok()
                    .and_then(|low| {
                        usize::try_from(level.low_high.height)
                            .ok()
                            .and_then(|high| low.checked_add(high))
                    })
                    .ok_or(TransformError::SizeOverflow)?,
            );
        }
        reserve_to(&mut self.current, max_plane);
        reserve_to(&mut self.next, max_plane);
        reserve_to(&mut self.horizontal_low, max_horizontal_low);
        reserve_to(&mut self.horizontal_high, max_horizontal_high);
        reserve_to(&mut self.line, max_line);
        reserve_to(&mut self.line_work, max_line);
        Ok(())
    }

    pub fn output_region(&self) -> AxisAlignedRegion {
        self.current_region
    }

    pub fn output(&self) -> &[T] {
        &self.current
    }

    pub fn clear(&mut self) {
        self.current_region = AxisAlignedRegion::default();
        self.current.clear();
        self.next.clear();
        self.horizontal_low.clear();
        self.horizontal_high.clear();
        self.line.clear();
        self.line_work.clear();
        self.phase_lines.clear();
        self.phase_line_work.clear();
        self.vertical_columns.clear();
    }

    #[cfg(feature = "parallel")]
    fn reserve_parallel_for_plan(
        &mut self,
        plan: &WindowSynthesisPlan,
    ) -> Result<(), TransformError> {
        let mut max_lines = 0_usize;
        for level in &plan.levels {
            let output_width =
                usize::try_from(level.output.width).map_err(|_| TransformError::SizeOverflow)?;
            let horizontal_low_line = usize::try_from(level.low_low.width)
                .ok()
                .and_then(|low| {
                    usize::try_from(level.high_low.width)
                        .ok()
                        .and_then(|high| low.checked_add(high))
                })
                .ok_or(TransformError::SizeOverflow)?;
            let horizontal_high_line = usize::try_from(level.low_high.width)
                .ok()
                .and_then(|low| {
                    usize::try_from(level.high_high.width)
                        .ok()
                        .and_then(|high| low.checked_add(high))
                })
                .ok_or(TransformError::SizeOverflow)?;
            let vertical_line = usize::try_from(level.low_low.height)
                .ok()
                .and_then(|low| {
                    usize::try_from(level.low_high.height)
                        .ok()
                        .and_then(|high| low.checked_add(high))
                })
                .ok_or(TransformError::SizeOverflow)?;
            max_lines = max_lines
                .max(
                    usize::try_from(level.low_low.height)
                        .ok()
                        .and_then(|rows| rows.checked_mul(horizontal_low_line))
                        .ok_or(TransformError::SizeOverflow)?,
                )
                .max(
                    usize::try_from(level.low_high.height)
                        .ok()
                        .and_then(|rows| rows.checked_mul(horizontal_high_line))
                        .ok_or(TransformError::SizeOverflow)?,
                )
                .max(
                    output_width
                        .checked_mul(vertical_line)
                        .ok_or(TransformError::SizeOverflow)?,
                );
        }
        reserve_to(&mut self.phase_lines, max_lines);
        reserve_to(&mut self.phase_line_work, max_lines);
        Ok(())
    }
}

fn reserve_to<T>(values: &mut Vec<T>, capacity: usize) {
    if values.capacity() < capacity {
        values.reserve_exact(capacity - values.len());
    }
}

fn validate_plan<T>(
    coefficients: &WindowCoefficientPlane<T>,
    plan: &WindowSynthesisPlan,
    transform: WaveletTransform,
) -> Result<(), TransformError> {
    if plan.transform != transform
        || plan.levels.len() != usize::from(plan.decomposition_levels)
        || coefficients.levels.len() != plan.levels.len()
        || coefficients.lowest_low_low.region != plan.lowest_low_low
    {
        return Err(TransformError::InvalidWindow);
    }
    for (planned, actual) in plan.levels.iter().zip(&coefficients.levels) {
        if planned.resolution != actual.resolution
            || planned.high_low != actual.high_low.region
            || planned.low_high != actual.low_high.region
            || planned.high_high != actual.high_high.region
        {
            return Err(TransformError::InvalidWindow);
        }
    }
    Ok(())
}

fn dense_row<T>(values: &[T], region: AxisAlignedRegion, y: u32) -> Result<&[T], TransformError> {
    let row = y
        .checked_sub(region.y)
        .filter(|row| *row < region.height)
        .ok_or(TransformError::InvalidWindow)?;
    let width = usize::try_from(region.width).map_err(|_| TransformError::SizeOverflow)?;
    let start = usize::try_from(row)
        .ok()
        .and_then(|row| row.checked_mul(width))
        .ok_or(TransformError::SizeOverflow)?;
    values
        .get(start..start + width)
        .ok_or(TransformError::InvalidWindow)
}

fn prepare_horizontal_line<T: Copy>(
    low: &[T],
    high: &[T],
    low_start: u32,
    high_start: u32,
    requested_start: u32,
    requested_width: u32,
    line: &mut Vec<T>,
) -> Result<(usize, usize), TransformError> {
    if low_start != high_start {
        return Err(TransformError::InvalidWindow);
    }
    let line_len = low
        .len()
        .checked_add(high.len())
        .ok_or(TransformError::SizeOverflow)?;
    line.clear();
    line.extend_from_slice(low);
    line.extend_from_slice(high);
    let reconstructed_start = low_start
        .checked_mul(2)
        .ok_or(TransformError::SizeOverflow)?;
    let offset = usize::try_from(
        requested_start
            .checked_sub(reconstructed_start)
            .ok_or(TransformError::InvalidWindow)?,
    )
    .map_err(|_| TransformError::SizeOverflow)?;
    let width = usize::try_from(requested_width).map_err(|_| TransformError::SizeOverflow)?;
    if offset
        .checked_add(width)
        .ok_or(TransformError::SizeOverflow)?
        > line_len
    {
        return Err(TransformError::InvalidWindow);
    }
    Ok((low.len(), offset))
}

fn peak_value_bytes<T>(workspace: &WindowSynthesisWorkspace<T>) -> u64 {
    let values = workspace
        .current
        .len()
        .saturating_add(workspace.next.len())
        .saturating_add(workspace.horizontal_low.len())
        .saturating_add(workspace.horizontal_high.len())
        .saturating_add(workspace.line.len())
        .saturating_add(workspace.line_work.len())
        .saturating_add(workspace.phase_lines.len())
        .saturating_add(workspace.phase_line_work.len())
        .saturating_add(workspace.vertical_columns.len());
    (values as u64).saturating_mul(size_of::<T>() as u64)
}

#[cfg(feature = "std")]
struct StageInstant(std::time::Instant);

#[cfg(not(feature = "std"))]
struct StageInstant;

fn stage_start(enabled: bool) -> Option<StageInstant> {
    #[cfg(feature = "std")]
    {
        enabled.then(|| StageInstant(std::time::Instant::now()))
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = enabled;
        None
    }
}

fn elapsed_ns(start: Option<StageInstant>) -> u128 {
    #[cfg(feature = "std")]
    {
        start.map_or(0, |start| start.0.elapsed().as_nanos())
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = start;
        0
    }
}

#[cfg(feature = "parallel")]
fn record_maximum_task_ns(maximum: &core::sync::atomic::AtomicU64, started: Option<StageInstant>) {
    use core::sync::atomic::Ordering;

    let elapsed = u64::try_from(elapsed_ns(started)).unwrap_or(u64::MAX);
    maximum.fetch_max(elapsed, Ordering::Relaxed);
}

#[cfg(feature = "parallel")]
fn maximum_task_ns(maximum: &core::sync::atomic::AtomicU64) -> u128 {
    use core::sync::atomic::Ordering;

    u128::from(maximum.load(Ordering::Relaxed))
}

#[cfg(feature = "parallel")]
trait ParallelWindowSample: Copy + Default + Send + Sync {
    const TRANSFORM: WaveletTransform;
    const UPDATES_PER_VALUE: u64;
    const SPLIT_LOW_INPUT: bool;
    const DIRECT_VERTICAL: bool;

    fn inverse_line(line: &mut [Self], low_len: usize, work: &mut [Self]);

    fn inverse_line_from_separate_low(
        low: &mut [Self],
        output: &mut [Self],
        low_len: usize,
    ) -> bool;

    // Geometry and output stride are explicit so implementations can write the
    // selected rows directly without constructing an intermediate descriptor.
    #[allow(clippy::too_many_arguments)]
    fn inverse_vertical_to_row_major(
        plane: &mut [Self],
        stride: usize,
        width: usize,
        height: usize,
        output_y: usize,
        output_height: usize,
        output: &mut [Self],
        output_stride: usize,
    ) -> bool;

    #[allow(clippy::too_many_arguments)]
    fn inverse_vertical_to_row_major_parallel(
        plane: &mut [Self],
        stride: usize,
        width: usize,
        height: usize,
        output_y: usize,
        output_height: usize,
        output: &mut [Self],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>>;

    #[allow(clippy::too_many_arguments)]
    fn transpose_stripe(
        source: &[Self],
        source_stride: usize,
        source_width: usize,
        source_height: usize,
        source_x: usize,
        columns: usize,
        destination: &mut [Self],
        destination_stride: usize,
    ) -> Result<(), TransformError>;

    fn is_valid(self) -> bool;
}

#[cfg(feature = "parallel")]
impl ParallelWindowSample for i32 {
    const TRANSFORM: WaveletTransform = WaveletTransform::Reversible53;
    const UPDATES_PER_VALUE: u64 = 1;
    const SPLIT_LOW_INPUT: bool = true;
    const DIRECT_VERTICAL: bool = true;

    fn inverse_line(line: &mut [Self], low_len: usize, work: &mut [Self]) {
        if line.len() > 1 && low_len.checked_mul(2) == Some(line.len()) {
            work[..low_len].copy_from_slice(&line[..low_len]);
            if emuella_j2k_accel::inverse_reversible_5_3_even_first_low_split(
                &mut work[..low_len],
                line,
                low_len,
            )
            .is_ok()
            {
                return;
            }
        } else if line.len() > 1
            && low_len
                .checked_mul(2)
                .and_then(|value| value.checked_sub(1))
                == Some(line.len())
        {
            work[..line.len()].copy_from_slice(line);
            if emuella_j2k_accel::inverse_reversible_5_3_odd_first_low(
                &mut work[..line.len()],
                line,
                low_len,
            )
            .is_ok()
            {
                return;
            }
        }
        transform_line_inverse_bounded(line, low_len, TransformBand::Low, work);
    }

    fn inverse_line_from_separate_low(
        low: &mut [Self],
        output: &mut [Self],
        low_len: usize,
    ) -> bool {
        emuella_j2k_accel::inverse_reversible_5_3_even_first_low_split(low, output, low_len).is_ok()
    }

    fn inverse_vertical_to_row_major(
        plane: &mut [Self],
        stride: usize,
        width: usize,
        height: usize,
        output_y: usize,
        output_height: usize,
        output: &mut [Self],
        output_stride: usize,
    ) -> bool {
        emuella_j2k_accel::inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32_region(
            plane,
            stride,
            width,
            height,
            0,
            output_y,
            width,
            output_height,
            output,
            output_stride,
        )
        .is_ok()
    }

    fn inverse_vertical_to_row_major_parallel(
        plane: &mut [Self],
        stride: usize,
        width: usize,
        height: usize,
        output_y: usize,
        output_height: usize,
        output: &mut [Self],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        use rayon::prelude::*;

        Some((|| {
            let desired_workers = worker_count.min(width).max(1);
            let chunk_columns = width.div_ceil(desired_workers);
            let active_workers = width.div_ceil(chunk_columns);
            let plane_root =
                emuella_j2k_accel::I32PlaneColumns::new(plane, stride, width, height, 0, width)
                    .map_err(|_| TransformError::SizeOverflow)?;
            let output_root = emuella_j2k_accel::I32PlaneColumns::new(
                output,
                output_stride,
                width,
                output_height,
                0,
                width,
            )
            .map_err(|_| TransformError::SizeOverflow)?;
            let mut plane_remainder = Some(plane_root);
            let mut output_remainder = Some(output_root);
            let mut jobs = Vec::with_capacity(active_workers);
            for worker_index in 0..active_workers {
                let plane_token = plane_remainder.take().ok_or(TransformError::SizeOverflow)?;
                let output_token = output_remainder
                    .take()
                    .ok_or(TransformError::SizeOverflow)?;
                if worker_index + 1 == active_workers {
                    jobs.push((plane_token, output_token));
                } else {
                    let (plane_job, plane_tail) = plane_token
                        .split_at(chunk_columns)
                        .map_err(|_| TransformError::SizeOverflow)?;
                    let (output_job, output_tail) = output_token
                        .split_at(chunk_columns)
                        .map_err(|_| TransformError::SizeOverflow)?;
                    jobs.push((plane_job, output_job));
                    plane_remainder = Some(plane_tail);
                    output_remainder = Some(output_tail);
                }
            }
            let maximum = core::sync::atomic::AtomicU64::new(0);
            jobs.into_par_iter()
                .try_for_each(|(mut plane_job, mut output_job)| {
                    let started = stage_start(collect_stage_timings);
                    plane_job
                        .inverse_reversible_5_3_even_first_low_to_row_major_i32(
                            output_y,
                            output_height,
                            &mut output_job,
                        )
                        .map_err(|_| TransformError::SizeOverflow)?;
                    record_maximum_task_ns(&maximum, started);
                    Ok(())
                })?;
            Ok(maximum_task_ns(&maximum))
        })())
    }

    fn transpose_stripe(
        source: &[Self],
        source_stride: usize,
        source_width: usize,
        source_height: usize,
        source_x: usize,
        columns: usize,
        destination: &mut [Self],
        destination_stride: usize,
    ) -> Result<(), TransformError> {
        emuella_j2k_accel::transpose_i32_stripe(
            source,
            source_stride,
            source_width,
            source_height,
            source_x,
            columns,
            destination,
            destination_stride,
        )
        .map_err(|_| TransformError::SizeOverflow)
    }

    fn is_valid(self) -> bool {
        true
    }
}

#[cfg(feature = "parallel")]
impl ParallelWindowSample for f32 {
    const TRANSFORM: WaveletTransform = WaveletTransform::Irreversible97;
    const UPDATES_PER_VALUE: u64 = 2;
    const SPLIT_LOW_INPUT: bool = false;
    const DIRECT_VERTICAL: bool = false;

    fn inverse_line(line: &mut [Self], low_len: usize, work: &mut [Self]) {
        inverse_irreversible_9_7_line(line, low_len, TransformBand::Low, work);
    }

    fn inverse_line_from_separate_low(
        _low: &mut [Self],
        _output: &mut [Self],
        _low_len: usize,
    ) -> bool {
        false
    }

    fn inverse_vertical_to_row_major(
        _plane: &mut [Self],
        _stride: usize,
        _width: usize,
        _height: usize,
        _output_y: usize,
        _output_height: usize,
        _output: &mut [Self],
        _output_stride: usize,
    ) -> bool {
        false
    }

    fn inverse_vertical_to_row_major_parallel(
        _plane: &mut [Self],
        _stride: usize,
        _width: usize,
        _height: usize,
        _output_y: usize,
        _output_height: usize,
        _output: &mut [Self],
        _output_stride: usize,
        _worker_count: usize,
        _collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        None
    }

    fn transpose_stripe(
        source: &[Self],
        source_stride: usize,
        source_width: usize,
        source_height: usize,
        source_x: usize,
        columns: usize,
        destination: &mut [Self],
        destination_stride: usize,
    ) -> Result<(), TransformError> {
        emuella_j2k_accel::transpose_f32_stripe(
            source,
            source_stride,
            source_width,
            source_height,
            source_x,
            columns,
            destination,
            destination_stride,
        )
        .map_err(|_| TransformError::SizeOverflow)
    }

    fn is_valid(self) -> bool {
        self.is_finite()
    }
}

#[cfg(feature = "parallel")]
fn bounded_phase_task_count(worker_count: usize) -> usize {
    // Retained two-worker measurements show that two millisecond-scale tasks
    // can be consumed by one freshly woken Rayon worker. Eight disjoint tasks
    // make stealing reliable. Larger worker budgets already have enough
    // independent tasks and retain one task per effective worker.
    if worker_count == 2 { 8 } else { worker_count }
}

#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
fn parallel_horizontal_phase<T: ParallelWindowSample>(
    low_values: &[T],
    low_region: AxisAlignedRegion,
    high_values: &[T],
    high_region: AxisAlignedRegion,
    packed_high_x_offset: u32,
    output_region: AxisAlignedRegion,
    output: &mut Vec<T>,
    phase_lines: &mut Vec<T>,
    phase_line_work: &mut Vec<T>,
    worker_count: usize,
    collect_stage_timings: bool,
) -> Result<(u64, u64, u128), TransformError> {
    use rayon::prelude::*;

    if low_region.y != high_region.y || low_region.height != high_region.height {
        return Err(TransformError::InvalidWindow);
    }
    let high_start = high_region
        .x
        .checked_sub(packed_high_x_offset)
        .ok_or(TransformError::InvalidWindow)?;
    if low_region.x != high_start {
        return Err(TransformError::InvalidWindow);
    }
    let low_len = usize::try_from(low_region.width).map_err(|_| TransformError::SizeOverflow)?;
    let high_len = usize::try_from(high_region.width).map_err(|_| TransformError::SizeOverflow)?;
    let line_len = low_len
        .checked_add(high_len)
        .ok_or(TransformError::SizeOverflow)?;
    let row_count = usize::try_from(low_region.height).map_err(|_| TransformError::SizeOverflow)?;
    let output_width =
        usize::try_from(output_region.width).map_err(|_| TransformError::SizeOverflow)?;
    let reconstructed_start = low_region
        .x
        .checked_mul(2)
        .ok_or(TransformError::SizeOverflow)?;
    let output_offset = usize::try_from(
        output_region
            .x
            .checked_sub(reconstructed_start)
            .ok_or(TransformError::InvalidWindow)?,
    )
    .map_err(|_| TransformError::SizeOverflow)?;
    if output_offset
        .checked_add(output_width)
        .ok_or(TransformError::SizeOverflow)?
        > line_len
    {
        return Err(TransformError::InvalidWindow);
    }
    output.resize(
        row_count
            .checked_mul(output_width)
            .ok_or(TransformError::SizeOverflow)?,
        T::default(),
    );
    let phase_len = row_count
        .checked_mul(line_len)
        .ok_or(TransformError::SizeOverflow)?;
    if phase_lines.len() < phase_len {
        phase_lines.resize(phase_len, T::default());
    }
    if phase_line_work.len() < phase_len {
        phase_line_work.resize(phase_len, T::default());
    }
    if row_count == 0 {
        return Ok((0, 0, 0));
    }
    let task_count = bounded_phase_task_count(worker_count);
    let chunk_rows = row_count.div_ceil(task_count.max(1).min(row_count));
    let maximum = core::sync::atomic::AtomicU64::new(0);
    output
        .par_chunks_mut(chunk_rows * output_width)
        .zip(phase_lines[..phase_len].par_chunks_mut(chunk_rows * line_len))
        .zip(phase_line_work[..phase_len].par_chunks_mut(chunk_rows * line_len))
        .enumerate()
        .try_for_each(|(chunk, ((destinations, lines), works))| {
            let started = stage_start(collect_stage_timings);
            let first_row = chunk * chunk_rows;
            let rows = destinations.len() / output_width;
            for local_row in 0..rows {
                let row = first_row + local_row;
                let y = low_region
                    .y
                    .checked_add(u32::try_from(row).map_err(|_| TransformError::SizeOverflow)?)
                    .ok_or(TransformError::SizeOverflow)?;
                let low = dense_row(low_values, low_region, y)?;
                let high = dense_row(high_values, high_region, y)?;
                let line = &mut lines[local_row * line_len..(local_row + 1) * line_len];
                let work = &mut works[local_row * line_len..(local_row + 1) * line_len];
                if T::SPLIT_LOW_INPUT && low_len.checked_mul(2) == Some(line_len) {
                    work[..low_len].copy_from_slice(low);
                    line[low_len..].copy_from_slice(high);
                    if !T::inverse_line_from_separate_low(&mut work[..low_len], line, low_len) {
                        return Err(TransformError::InvalidWindow);
                    }
                } else {
                    line[..low_len].copy_from_slice(low);
                    line[low_len..].copy_from_slice(high);
                    T::inverse_line(line, low_len, work);
                }
                let destination =
                    &mut destinations[local_row * output_width..(local_row + 1) * output_width];
                destination.copy_from_slice(&line[output_offset..output_offset + output_width]);
            }
            record_maximum_task_ns(&maximum, started);
            Ok::<(), TransformError>(())
        })?;
    let values = (row_count as u64).saturating_mul(output_width as u64);
    let updates = (row_count as u64)
        .saturating_mul(line_len as u64)
        .saturating_mul(T::UPDATES_PER_VALUE);
    Ok((values, updates, maximum_task_ns(&maximum)))
}

#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
fn parallel_vertical_phase<T: ParallelWindowSample>(
    horizontal_low: &[T],
    low_region: AxisAlignedRegion,
    horizontal_high: &[T],
    high_region: AxisAlignedRegion,
    packed_high_y_offset: u32,
    output_region: AxisAlignedRegion,
    next: &mut Vec<T>,
    phase_lines: &mut Vec<T>,
    phase_line_work: &mut Vec<T>,
    worker_count: usize,
    collect_stage_timings: bool,
) -> Result<(u64, u64, u128), TransformError> {
    use rayon::prelude::*;

    let output_width =
        usize::try_from(output_region.width).map_err(|_| TransformError::SizeOverflow)?;
    let output_height =
        usize::try_from(output_region.height).map_err(|_| TransformError::SizeOverflow)?;
    if low_region.x != output_region.x
        || high_region.x != output_region.x
        || low_region.width != output_region.width
        || high_region.width != output_region.width
    {
        return Err(TransformError::InvalidWindow);
    }
    let high_start = high_region
        .y
        .checked_sub(packed_high_y_offset)
        .ok_or(TransformError::InvalidWindow)?;
    if low_region.y != high_start {
        return Err(TransformError::InvalidWindow);
    }
    let low_rows = usize::try_from(low_region.height).map_err(|_| TransformError::SizeOverflow)?;
    let high_rows =
        usize::try_from(high_region.height).map_err(|_| TransformError::SizeOverflow)?;
    let line_len = low_rows
        .checked_add(high_rows)
        .ok_or(TransformError::SizeOverflow)?;
    let reconstructed_start = low_region
        .y
        .checked_mul(2)
        .ok_or(TransformError::SizeOverflow)?;
    let output_offset = usize::try_from(
        output_region
            .y
            .checked_sub(reconstructed_start)
            .ok_or(TransformError::InvalidWindow)?,
    )
    .map_err(|_| TransformError::SizeOverflow)?;
    if output_offset
        .checked_add(output_height)
        .ok_or(TransformError::SizeOverflow)?
        > line_len
    {
        return Err(TransformError::InvalidWindow);
    }
    let phase_len = output_width
        .checked_mul(line_len)
        .ok_or(TransformError::SizeOverflow)?;
    if phase_lines.len() < phase_len {
        phase_lines.resize(phase_len, T::default());
    }
    if output_width == 0 {
        return Ok((0, 0, 0));
    }
    if T::DIRECT_VERTICAL && low_rows.checked_mul(2) == Some(line_len) {
        let low_samples = low_rows
            .checked_mul(output_width)
            .ok_or(TransformError::SizeOverflow)?;
        let high_samples = high_rows
            .checked_mul(output_width)
            .ok_or(TransformError::SizeOverflow)?;
        if horizontal_low.len() != low_samples
            || horizontal_high.len() != high_samples
            || low_samples
                .checked_add(high_samples)
                .ok_or(TransformError::SizeOverflow)?
                != phase_len
        {
            return Err(TransformError::InvalidWindow);
        }
        phase_lines[..low_samples].copy_from_slice(horizontal_low);
        phase_lines[low_samples..phase_len].copy_from_slice(horizontal_high);
        let output_samples = output_width
            .checked_mul(output_height)
            .ok_or(TransformError::SizeOverflow)?;
        next.resize(output_samples, T::default());
        let direct_kernel_ns = if worker_count == 1 {
            let started = stage_start(collect_stage_timings);
            T::inverse_vertical_to_row_major(
                &mut phase_lines[..phase_len],
                output_width,
                output_width,
                line_len,
                output_offset,
                output_height,
                next,
                output_width,
            )
            .then(|| elapsed_ns(started))
        } else if let Some(result) = T::inverse_vertical_to_row_major_parallel(
            &mut phase_lines[..phase_len],
            output_width,
            output_width,
            line_len,
            output_offset,
            output_height,
            next,
            output_width,
            bounded_phase_task_count(worker_count),
            collect_stage_timings,
        ) {
            Some(result?)
        } else {
            None
        };
        if let Some(kernel_ns) = direct_kernel_ns {
            let values = output_samples as u64;
            let updates = (output_width as u64)
                .saturating_mul(line_len as u64)
                .saturating_mul(T::UPDATES_PER_VALUE);
            return Ok((values, updates, kernel_ns));
        }
    }
    if phase_line_work.len() < phase_len {
        phase_line_work.resize(phase_len, T::default());
    }
    let task_count = bounded_phase_task_count(worker_count);
    let chunk_columns = output_width.div_ceil(task_count.max(1).min(output_width));
    let lifting_maximum = core::sync::atomic::AtomicU64::new(0);
    phase_lines[..phase_len]
        .par_chunks_mut(chunk_columns * line_len)
        .zip(phase_line_work[..phase_len].par_chunks_mut(chunk_columns * line_len))
        .enumerate()
        .try_for_each(|(chunk, (lines, works))| {
            let started = stage_start(collect_stage_timings);
            let first_column = chunk * chunk_columns;
            let columns = lines.len() / line_len;
            let split_low = T::SPLIT_LOW_INPUT && low_rows.checked_mul(2) == Some(line_len);
            let low_destination = if split_low { &mut *works } else { &mut *lines };
            T::transpose_stripe(
                horizontal_low,
                output_width,
                output_width,
                low_rows,
                first_column,
                columns,
                low_destination,
                line_len,
            )?;
            T::transpose_stripe(
                horizontal_high,
                output_width,
                output_width,
                high_rows,
                first_column,
                columns,
                &mut lines[low_rows..],
                line_len,
            )?;
            for local_column in 0..columns {
                let line = &mut lines[local_column * line_len..(local_column + 1) * line_len];
                let work = &mut works[local_column * line_len..(local_column + 1) * line_len];
                if split_low {
                    if !T::inverse_line_from_separate_low(&mut work[..low_rows], line, low_rows) {
                        return Err(TransformError::InvalidWindow);
                    }
                } else {
                    T::inverse_line(line, low_rows, work);
                }
            }
            record_maximum_task_ns(&lifting_maximum, started);
            Ok::<(), TransformError>(())
        })?;
    next.resize(
        output_width
            .checked_mul(output_height)
            .ok_or(TransformError::SizeOverflow)?,
        T::default(),
    );
    let task_count = bounded_phase_task_count(worker_count);
    let chunk_rows = output_height.div_ceil(task_count.max(1).min(output_height));
    let output_maximum = core::sync::atomic::AtomicU64::new(0);
    next.par_chunks_mut(chunk_rows * output_width)
        .enumerate()
        .try_for_each(|(chunk, destination)| {
            let started = stage_start(collect_stage_timings);
            let first_row = chunk * chunk_rows;
            let rows = destination.len() / output_width;
            T::transpose_stripe(
                &phase_lines[..phase_len],
                line_len,
                line_len,
                output_width,
                output_offset + first_row,
                rows,
                destination,
                output_width,
            )?;
            record_maximum_task_ns(&output_maximum, started);
            Ok::<(), TransformError>(())
        })?;
    let values = (output_width as u64).saturating_mul(output_height as u64);
    let updates = (output_width as u64)
        .saturating_mul(line_len as u64)
        .saturating_mul(T::UPDATES_PER_VALUE);
    Ok((
        values,
        updates,
        maximum_task_ns(&lifting_maximum).saturating_add(maximum_task_ns(&output_maximum)),
    ))
}

#[cfg(feature = "parallel")]
fn inverse_window_parallel<T: ParallelWindowSample>(
    coefficients: &WindowCoefficientPlane<T>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<T>,
    collect_stage_timings: bool,
    horizontal_worker_count: usize,
    vertical_worker_count: usize,
) -> Result<WindowSynthesisReport, TransformError> {
    if horizontal_worker_count == 0 || vertical_worker_count == 0 {
        return Err(TransformError::InvalidWindow);
    }
    validate_plan(coefficients, plan, T::TRANSFORM)?;
    workspace.reserve_for_plan(plan)?;
    workspace.reserve_parallel_for_plan(plan)?;
    let mut report = WindowSynthesisReport::default();
    report.work.coefficients_loaded = coefficients.sample_count();

    let prepare_started = stage_start(collect_stage_timings);
    workspace.current_region = plan.lowest_low_low;
    workspace.current.clear();
    workspace
        .current
        .extend_from_slice(&coefficients.lowest_low_low.values);
    report.stages.level_preparation_ns += elapsed_ns(prepare_started);

    for (level_index, level) in plan.levels.iter().enumerate() {
        let bands = coefficients
            .levels
            .get(level_index)
            .ok_or(TransformError::InvalidWindow)?;
        if workspace.current_region != level.low_low {
            return Err(TransformError::InvalidWindow);
        }
        let (level_width, level_height) = resolution_dimensions(
            plan.width,
            plan.height,
            plan.decomposition_levels,
            level.resolution,
        )?;
        let low_width = level_width.div_ceil(2);
        let low_height = level_height.div_ceil(2);

        let horizontal_started = stage_start(collect_stage_timings);
        let horizontal_low_region = AxisAlignedRegion {
            x: level.output.x,
            y: level.low_low.y,
            width: level.output.width,
            height: level.low_low.height,
        };
        let (values, updates, horizontal_low_kernel_ns) = parallel_horizontal_phase(
            &workspace.current,
            workspace.current_region,
            &bands.high_low.values,
            bands.high_low.region,
            low_width,
            level.output,
            &mut workspace.horizontal_low,
            &mut workspace.phase_lines,
            &mut workspace.phase_line_work,
            horizontal_worker_count,
            collect_stage_timings,
        )?;
        report.work.horizontal_values = report.work.horizontal_values.saturating_add(values);
        report.work.lifting_updates = report.work.lifting_updates.saturating_add(updates);
        let horizontal_high_region = AxisAlignedRegion {
            x: level.output.x,
            y: level.low_high.y,
            width: level.output.width,
            height: level.low_high.height,
        };
        let (values, updates, horizontal_high_kernel_ns) = parallel_horizontal_phase(
            &bands.low_high.values,
            bands.low_high.region,
            &bands.high_high.values,
            bands.high_high.region,
            low_width,
            level.output,
            &mut workspace.horizontal_high,
            &mut workspace.phase_lines,
            &mut workspace.phase_line_work,
            horizontal_worker_count,
            collect_stage_timings,
        )?;
        report.work.horizontal_values = report.work.horizontal_values.saturating_add(values);
        report.work.lifting_updates = report.work.lifting_updates.saturating_add(updates);
        let horizontal_ns = elapsed_ns(horizontal_started);
        let horizontal_kernel_ns =
            horizontal_low_kernel_ns.saturating_add(horizontal_high_kernel_ns);
        report.stages.horizontal_ns = report.stages.horizontal_ns.saturating_add(horizontal_ns);
        report.stages.horizontal_kernel_ns = report
            .stages
            .horizontal_kernel_ns
            .saturating_add(horizontal_kernel_ns);
        report.stages.horizontal_startup_and_barrier_ns = report
            .stages
            .horizontal_startup_and_barrier_ns
            .saturating_add(horizontal_ns.saturating_sub(horizontal_kernel_ns));

        let vertical_started = stage_start(collect_stage_timings);
        let (values, updates, vertical_kernel_ns) = parallel_vertical_phase(
            &workspace.horizontal_low,
            horizontal_low_region,
            &workspace.horizontal_high,
            horizontal_high_region,
            low_height,
            level.output,
            &mut workspace.next,
            &mut workspace.phase_lines,
            &mut workspace.phase_line_work,
            vertical_worker_count,
            collect_stage_timings,
        )?;
        report.work.vertical_values = report.work.vertical_values.saturating_add(values);
        report.work.lifting_updates = report.work.lifting_updates.saturating_add(updates);
        let vertical_ns = elapsed_ns(vertical_started);
        report.stages.vertical_ns = report.stages.vertical_ns.saturating_add(vertical_ns);
        report.stages.vertical_kernel_ns = report
            .stages
            .vertical_kernel_ns
            .saturating_add(vertical_kernel_ns);
        report.stages.vertical_startup_and_barrier_ns = report
            .stages
            .vertical_startup_and_barrier_ns
            .saturating_add(vertical_ns.saturating_sub(vertical_kernel_ns));
        report.peak_value_bytes = report.peak_value_bytes.max(peak_value_bytes(workspace));

        let prepare_started = stage_start(collect_stage_timings);
        core::mem::swap(&mut workspace.current, &mut workspace.next);
        workspace.current_region = level.output;
        workspace.next.clear();
        report.stages.level_preparation_ns += elapsed_ns(prepare_started);
    }
    if workspace.current_region != plan.output_region
        || workspace
            .current
            .iter()
            .copied()
            .any(|sample| !sample.is_valid())
    {
        return Err(TransformError::InvalidWindow);
    }
    report.work.output_samples = plan.output_region.sample_count();
    Ok(report)
}

/// Phase-parallel reversible 5/3 bounded synthesis in the current Rayon pool.
#[cfg(feature = "parallel")]
pub fn inverse_reversible_5_3_window_parallel(
    coefficients: &WindowCoefficientPlane<i32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<i32>,
    collect_stage_timings: bool,
    worker_count: usize,
) -> Result<WindowSynthesisReport, TransformError> {
    inverse_window_parallel(
        coefficients,
        plan,
        workspace,
        collect_stage_timings,
        worker_count,
        worker_count,
    )
}

/// Phase-parallel reversible 5/3 bounded synthesis with independent
/// horizontal and vertical worker budgets in the current Rayon pool.
#[cfg(feature = "parallel")]
pub fn inverse_reversible_5_3_window_parallel_with_workers(
    coefficients: &WindowCoefficientPlane<i32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<i32>,
    collect_stage_timings: bool,
    horizontal_workers: usize,
    vertical_workers: usize,
) -> Result<WindowSynthesisReport, TransformError> {
    inverse_window_parallel(
        coefficients,
        plan,
        workspace,
        collect_stage_timings,
        horizontal_workers,
        vertical_workers,
    )
}

/// Phase-parallel irreversible 9/7 bounded synthesis in the current Rayon pool.
#[cfg(feature = "parallel")]
pub fn inverse_irreversible_9_7_window_parallel(
    coefficients: &WindowCoefficientPlane<f32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<f32>,
    collect_stage_timings: bool,
    worker_count: usize,
) -> Result<WindowSynthesisReport, TransformError> {
    inverse_window_parallel(
        coefficients,
        plan,
        workspace,
        collect_stage_timings,
        worker_count,
        worker_count,
    )
}

/// Phase-parallel irreversible 9/7 bounded synthesis with independent
/// horizontal and vertical worker budgets in the current Rayon pool.
#[cfg(feature = "parallel")]
pub fn inverse_irreversible_9_7_window_parallel_with_workers(
    coefficients: &WindowCoefficientPlane<f32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<f32>,
    collect_stage_timings: bool,
    horizontal_workers: usize,
    vertical_workers: usize,
) -> Result<WindowSynthesisReport, TransformError> {
    inverse_window_parallel(
        coefficients,
        plan,
        workspace,
        collect_stage_timings,
        horizontal_workers,
        vertical_workers,
    )
}

/// Direct scalar reversible 5/3 bounded synthesis.
pub fn inverse_reversible_5_3_window(
    coefficients: &WindowCoefficientPlane<i32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<i32>,
    collect_stage_timings: bool,
) -> Result<WindowSynthesisReport, TransformError> {
    validate_plan(coefficients, plan, WaveletTransform::Reversible53)?;
    workspace.reserve_for_plan(plan)?;
    let mut report = WindowSynthesisReport::default();
    report.work.coefficients_loaded = coefficients.sample_count();

    let prepare_started = stage_start(collect_stage_timings);
    workspace.current_region = plan.lowest_low_low;
    workspace.current.clear();
    workspace
        .current
        .extend_from_slice(&coefficients.lowest_low_low.values);
    report.stages.level_preparation_ns += elapsed_ns(prepare_started);

    for (level_index, level) in plan.levels.iter().enumerate() {
        let bands = coefficients
            .levels
            .get(level_index)
            .ok_or(TransformError::InvalidWindow)?;
        let (level_width, level_height) = resolution_dimensions(
            plan.width,
            plan.height,
            plan.decomposition_levels,
            level.resolution,
        )?;
        let edges = Reversible53Edges::from_tile_origin(
            0,
            0,
            usize::try_from(level_width).map_err(|_| TransformError::SizeOverflow)?,
            usize::try_from(level_height).map_err(|_| TransformError::SizeOverflow)?,
        );
        let low_width = u32::try_from(edges.horizontal_low_samples)
            .map_err(|_| TransformError::SizeOverflow)?;
        let low_height =
            u32::try_from(edges.vertical_low_samples).map_err(|_| TransformError::SizeOverflow)?;
        if workspace.current_region != level.low_low {
            return Err(TransformError::InvalidWindow);
        }

        let horizontal_started = stage_start(collect_stage_timings);
        let output_width =
            usize::try_from(level.output.width).map_err(|_| TransformError::SizeOverflow)?;
        workspace.horizontal_low.resize(
            output_width
                .checked_mul(
                    usize::try_from(level.low_low.height)
                        .map_err(|_| TransformError::SizeOverflow)?,
                )
                .ok_or(TransformError::SizeOverflow)?,
            0,
        );
        for y in level.low_low.y..level.low_low.end_y()? {
            let low = dense_row(&workspace.current, workspace.current_region, y)?;
            let high = bands.high_low.row(y)?;
            let (low_len, offset) = prepare_horizontal_line(
                low,
                high,
                level.low_low.x,
                level
                    .high_low
                    .x
                    .checked_sub(low_width)
                    .ok_or(TransformError::InvalidWindow)?,
                level.output.x,
                level.output.width,
                &mut workspace.line,
            )?;
            workspace.line_work.resize(workspace.line.len(), 0);
            transform_line_inverse_bounded(
                &mut workspace.line,
                low_len,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            let destination_row =
                usize::try_from(y - level.low_low.y).map_err(|_| TransformError::SizeOverflow)?;
            let destination_start = destination_row
                .checked_mul(output_width)
                .ok_or(TransformError::SizeOverflow)?;
            workspace.horizontal_low[destination_start..destination_start + output_width]
                .copy_from_slice(&workspace.line[offset..offset + output_width]);
            report.work.horizontal_values = report
                .work
                .horizontal_values
                .saturating_add(output_width as u64);
            report.work.lifting_updates = report
                .work
                .lifting_updates
                .saturating_add(workspace.line.len() as u64);
        }
        workspace.horizontal_high.resize(
            output_width
                .checked_mul(
                    usize::try_from(level.low_high.height)
                        .map_err(|_| TransformError::SizeOverflow)?,
                )
                .ok_or(TransformError::SizeOverflow)?,
            0,
        );
        for packed_y in level.low_high.y..level.low_high.end_y()? {
            let low = bands.low_high.row(packed_y)?;
            let high = bands.high_high.row(packed_y)?;
            let (low_len, offset) = prepare_horizontal_line(
                low,
                high,
                level.low_high.x,
                level
                    .high_high
                    .x
                    .checked_sub(low_width)
                    .ok_or(TransformError::InvalidWindow)?,
                level.output.x,
                level.output.width,
                &mut workspace.line,
            )?;
            workspace.line_work.resize(workspace.line.len(), 0);
            transform_line_inverse_bounded(
                &mut workspace.line,
                low_len,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            let destination_row = usize::try_from(packed_y - level.low_high.y)
                .map_err(|_| TransformError::SizeOverflow)?;
            let destination_start = destination_row
                .checked_mul(output_width)
                .ok_or(TransformError::SizeOverflow)?;
            workspace.horizontal_high[destination_start..destination_start + output_width]
                .copy_from_slice(&workspace.line[offset..offset + output_width]);
            report.work.horizontal_values = report
                .work
                .horizontal_values
                .saturating_add(output_width as u64);
            report.work.lifting_updates = report
                .work
                .lifting_updates
                .saturating_add(workspace.line.len() as u64);
        }
        report.stages.horizontal_ns += elapsed_ns(horizontal_started);

        let vertical_started = stage_start(collect_stage_timings);
        workspace.next.resize(region_len(level.output)?, 0);
        let low_rows =
            usize::try_from(level.low_low.height).map_err(|_| TransformError::SizeOverflow)?;
        let high_rows =
            usize::try_from(level.low_high.height).map_err(|_| TransformError::SizeOverflow)?;
        let line_len = low_rows
            .checked_add(high_rows)
            .ok_or(TransformError::SizeOverflow)?;
        let reconstructed_y = level
            .low_low
            .y
            .checked_mul(2)
            .ok_or(TransformError::SizeOverflow)?;
        let output_y_offset = usize::try_from(
            level
                .output
                .y
                .checked_sub(reconstructed_y)
                .ok_or(TransformError::InvalidWindow)?,
        )
        .map_err(|_| TransformError::SizeOverflow)?;
        let output_height =
            usize::try_from(level.output.height).map_err(|_| TransformError::SizeOverflow)?;
        if level
            .low_high
            .y
            .checked_sub(low_height)
            .ok_or(TransformError::InvalidWindow)?
            != level.low_low.y
            || output_y_offset
                .checked_add(output_height)
                .ok_or(TransformError::SizeOverflow)?
                > line_len
        {
            return Err(TransformError::InvalidWindow);
        }
        for x in 0..output_width {
            workspace.line.resize(line_len, 0);
            for row in 0..low_rows {
                workspace.line[row] = workspace.horizontal_low[row * output_width + x];
            }
            for row in 0..high_rows {
                workspace.line[low_rows + row] = workspace.horizontal_high[row * output_width + x];
            }
            workspace.line_work.resize(line_len, 0);
            transform_line_inverse_bounded(
                &mut workspace.line,
                low_rows,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            for row in 0..output_height {
                workspace.next[row * output_width + x] = workspace.line[output_y_offset + row];
            }
            report.work.vertical_values = report
                .work
                .vertical_values
                .saturating_add(output_height as u64);
            report.work.lifting_updates =
                report.work.lifting_updates.saturating_add(line_len as u64);
        }
        report.stages.vertical_ns += elapsed_ns(vertical_started);
        report.peak_value_bytes = report.peak_value_bytes.max(peak_value_bytes(workspace));

        let prepare_started = stage_start(collect_stage_timings);
        core::mem::swap(&mut workspace.current, &mut workspace.next);
        workspace.current_region = level.output;
        workspace.next.clear();
        report.stages.level_preparation_ns += elapsed_ns(prepare_started);
    }
    if workspace.current_region != plan.output_region {
        return Err(TransformError::InvalidWindow);
    }
    report.work.output_samples = plan.output_region.sample_count();
    Ok(report)
}

/// Direct scalar irreversible 9/7 bounded synthesis.
pub fn inverse_irreversible_9_7_window(
    coefficients: &WindowCoefficientPlane<f32>,
    plan: &WindowSynthesisPlan,
    workspace: &mut WindowSynthesisWorkspace<f32>,
    collect_stage_timings: bool,
) -> Result<WindowSynthesisReport, TransformError> {
    validate_plan(coefficients, plan, WaveletTransform::Irreversible97)?;
    workspace.reserve_for_plan(plan)?;
    let mut report = WindowSynthesisReport::default();
    report.work.coefficients_loaded = coefficients.sample_count();

    let prepare_started = stage_start(collect_stage_timings);
    workspace.current_region = plan.lowest_low_low;
    workspace.current.clear();
    workspace
        .current
        .extend_from_slice(&coefficients.lowest_low_low.values);
    report.stages.level_preparation_ns += elapsed_ns(prepare_started);

    for (level_index, level) in plan.levels.iter().enumerate() {
        let bands = coefficients
            .levels
            .get(level_index)
            .ok_or(TransformError::InvalidWindow)?;
        let (level_width, level_height) = resolution_dimensions(
            plan.width,
            plan.height,
            plan.decomposition_levels,
            level.resolution,
        )?;
        let edges = Irreversible97Edges::from_tile_origin(
            0,
            0,
            usize::try_from(level_width).map_err(|_| TransformError::SizeOverflow)?,
            usize::try_from(level_height).map_err(|_| TransformError::SizeOverflow)?,
        );
        let low_width = u32::try_from(edges.horizontal_low_samples)
            .map_err(|_| TransformError::SizeOverflow)?;
        let low_height =
            u32::try_from(edges.vertical_low_samples).map_err(|_| TransformError::SizeOverflow)?;
        if workspace.current_region != level.low_low {
            return Err(TransformError::InvalidWindow);
        }

        let horizontal_started = stage_start(collect_stage_timings);
        let output_width =
            usize::try_from(level.output.width).map_err(|_| TransformError::SizeOverflow)?;
        workspace.horizontal_low.resize(
            output_width
                .checked_mul(
                    usize::try_from(level.low_low.height)
                        .map_err(|_| TransformError::SizeOverflow)?,
                )
                .ok_or(TransformError::SizeOverflow)?,
            0.0,
        );
        for y in level.low_low.y..level.low_low.end_y()? {
            let low = dense_row(&workspace.current, workspace.current_region, y)?;
            let high = bands.high_low.row(y)?;
            let (low_len, offset) = prepare_horizontal_line(
                low,
                high,
                level.low_low.x,
                level
                    .high_low
                    .x
                    .checked_sub(low_width)
                    .ok_or(TransformError::InvalidWindow)?,
                level.output.x,
                level.output.width,
                &mut workspace.line,
            )?;
            workspace.line_work.resize(workspace.line.len(), 0.0);
            inverse_irreversible_9_7_line(
                &mut workspace.line,
                low_len,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            let destination_row =
                usize::try_from(y - level.low_low.y).map_err(|_| TransformError::SizeOverflow)?;
            let destination_start = destination_row
                .checked_mul(output_width)
                .ok_or(TransformError::SizeOverflow)?;
            workspace.horizontal_low[destination_start..destination_start + output_width]
                .copy_from_slice(&workspace.line[offset..offset + output_width]);
            report.work.horizontal_values = report
                .work
                .horizontal_values
                .saturating_add(output_width as u64);
            report.work.lifting_updates = report
                .work
                .lifting_updates
                .saturating_add((workspace.line.len() as u64).saturating_mul(2));
        }
        workspace.horizontal_high.resize(
            output_width
                .checked_mul(
                    usize::try_from(level.low_high.height)
                        .map_err(|_| TransformError::SizeOverflow)?,
                )
                .ok_or(TransformError::SizeOverflow)?,
            0.0,
        );
        for packed_y in level.low_high.y..level.low_high.end_y()? {
            let low = bands.low_high.row(packed_y)?;
            let high = bands.high_high.row(packed_y)?;
            let (low_len, offset) = prepare_horizontal_line(
                low,
                high,
                level.low_high.x,
                level
                    .high_high
                    .x
                    .checked_sub(low_width)
                    .ok_or(TransformError::InvalidWindow)?,
                level.output.x,
                level.output.width,
                &mut workspace.line,
            )?;
            workspace.line_work.resize(workspace.line.len(), 0.0);
            inverse_irreversible_9_7_line(
                &mut workspace.line,
                low_len,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            let destination_row = usize::try_from(packed_y - level.low_high.y)
                .map_err(|_| TransformError::SizeOverflow)?;
            let destination_start = destination_row
                .checked_mul(output_width)
                .ok_or(TransformError::SizeOverflow)?;
            workspace.horizontal_high[destination_start..destination_start + output_width]
                .copy_from_slice(&workspace.line[offset..offset + output_width]);
            report.work.horizontal_values = report
                .work
                .horizontal_values
                .saturating_add(output_width as u64);
            report.work.lifting_updates = report
                .work
                .lifting_updates
                .saturating_add((workspace.line.len() as u64).saturating_mul(2));
        }
        report.stages.horizontal_ns += elapsed_ns(horizontal_started);

        let vertical_started = stage_start(collect_stage_timings);
        workspace.next.resize(region_len(level.output)?, 0.0);
        let low_rows =
            usize::try_from(level.low_low.height).map_err(|_| TransformError::SizeOverflow)?;
        let high_rows =
            usize::try_from(level.low_high.height).map_err(|_| TransformError::SizeOverflow)?;
        let line_len = low_rows
            .checked_add(high_rows)
            .ok_or(TransformError::SizeOverflow)?;
        let reconstructed_y = level
            .low_low
            .y
            .checked_mul(2)
            .ok_or(TransformError::SizeOverflow)?;
        let output_y_offset = usize::try_from(
            level
                .output
                .y
                .checked_sub(reconstructed_y)
                .ok_or(TransformError::InvalidWindow)?,
        )
        .map_err(|_| TransformError::SizeOverflow)?;
        let output_height =
            usize::try_from(level.output.height).map_err(|_| TransformError::SizeOverflow)?;
        if level
            .low_high
            .y
            .checked_sub(low_height)
            .ok_or(TransformError::InvalidWindow)?
            != level.low_low.y
            || output_y_offset
                .checked_add(output_height)
                .ok_or(TransformError::SizeOverflow)?
                > line_len
        {
            return Err(TransformError::InvalidWindow);
        }
        for x in 0..output_width {
            workspace.line.resize(line_len, 0.0);
            for row in 0..low_rows {
                workspace.line[row] = workspace.horizontal_low[row * output_width + x];
            }
            for row in 0..high_rows {
                workspace.line[low_rows + row] = workspace.horizontal_high[row * output_width + x];
            }
            workspace.line_work.resize(line_len, 0.0);
            inverse_irreversible_9_7_line(
                &mut workspace.line,
                low_rows,
                TransformBand::Low,
                &mut workspace.line_work,
            );
            for row in 0..output_height {
                workspace.next[row * output_width + x] = workspace.line[output_y_offset + row];
            }
            report.work.vertical_values = report
                .work
                .vertical_values
                .saturating_add(output_height as u64);
            report.work.lifting_updates = report
                .work
                .lifting_updates
                .saturating_add((line_len as u64).saturating_mul(2));
        }
        report.stages.vertical_ns += elapsed_ns(vertical_started);
        report.peak_value_bytes = report.peak_value_bytes.max(peak_value_bytes(workspace));

        let prepare_started = stage_start(collect_stage_timings);
        core::mem::swap(&mut workspace.current, &mut workspace.next);
        workspace.current_region = level.output;
        workspace.next.clear();
        report.stages.level_preparation_ns += elapsed_ns(prepare_started);
    }
    if workspace.current_region != plan.output_region
        || workspace.current.iter().any(|sample| !sample.is_finite())
    {
        return Err(TransformError::InvalidWindow);
    }
    report.work.output_samples = plan.output_region.sample_count();
    Ok(report)
}
