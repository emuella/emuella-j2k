//! Memory-bounded full-plane inverse synthesis backends.

use alloc::vec::Vec;
use core::mem::size_of;

#[cfg(feature = "parallel")]
use super::{
    AxisAlignedRegion, inverse_irreversible_9_7_line, inverse_irreversible_9_7_line_from_read,
    transform_line_inverse_from_read_bounded,
};
use super::{
    Irreversible97Edges, Reversible53Edges, TransformBand, TransformError, WaveletTransform,
};

const DEFAULT_STRIPE_WIDTH: usize = 128;
const TRANSPOSE_BLOCK: usize = 16;

/// Full-plane inverse-DWT implementation that actually executed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FullSynthesisBackend {
    /// Checked-compatible serial row and strided-column reconstruction.
    #[default]
    LegacyScalar,
    /// Parallel rows plus a bounded reusable column-major stripe.
    ParallelStriped,
    /// Parallel rows plus one reusable full transposed plane.
    ParallelTranspose,
}

impl FullSynthesisBackend {
    pub const fn name(self) -> &'static str {
        match self {
            Self::LegacyScalar => "legacy-scalar",
            Self::ParallelStriped => "parallel-striped",
            Self::ParallelTranspose => "parallel-transpose",
        }
    }
}

/// Cached architecture backend used by the transpose route.
pub fn full_transpose_backend_name() -> &'static str {
    emuella_j2k_accel::transpose_backend().name()
}

/// One active resolution in ascending inverse-synthesis order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FullSynthesisLevel {
    pub width: usize,
    pub height: usize,
    pub horizontal_low_samples: usize,
    pub vertical_low_samples: usize,
    pub horizontal_first: TransformBand,
    pub vertical_first: TransformBand,
}

impl FullSynthesisLevel {
    pub const fn reversible(width: usize, height: usize, edges: Reversible53Edges) -> Self {
        Self {
            width,
            height,
            horizontal_low_samples: edges.horizontal_low_samples,
            vertical_low_samples: edges.vertical_low_samples,
            horizontal_first: edges.horizontal_first,
            vertical_first: edges.vertical_first,
        }
    }

    pub const fn irreversible(width: usize, height: usize, edges: Irreversible97Edges) -> Self {
        Self {
            width,
            height,
            horizontal_low_samples: edges.horizontal_low_samples,
            vertical_low_samples: edges.vertical_low_samples,
            horizontal_first: edges.horizontal_first,
            vertical_first: edges.vertical_first,
        }
    }
}

/// Validated geometry shared by all full-plane synthesis backends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullSynthesisPlan {
    pub transform: WaveletTransform,
    pub stride: usize,
    pub levels: Vec<FullSynthesisLevel>,
}

impl FullSynthesisPlan {
    pub fn new(
        transform: WaveletTransform,
        stride: usize,
        levels: Vec<FullSynthesisLevel>,
    ) -> Result<Self, TransformError> {
        let plan = Self {
            transform,
            stride,
            levels,
        };
        plan.validate_geometry()?;
        Ok(plan)
    }

    pub fn active_samples(&self) -> u64 {
        self.levels.last().map_or(0, |level| {
            (level.width as u64).saturating_mul(level.height as u64)
        })
    }

    pub fn max_axis(&self) -> usize {
        self.levels
            .iter()
            .map(|level| level.width.max(level.height))
            .max()
            .unwrap_or(0)
    }

    pub fn estimate<T>(&self, workers: usize) -> Result<FullSynthesisEstimate, TransformError> {
        self.validate_geometry()?;
        let sample_bytes = size_of::<T>() as u64;
        let active_samples = self.active_samples();
        let max_axis = self.max_axis() as u64;
        let worker_count = workers.max(1) as u64;
        let worker_scratch = worker_count
            .checked_mul(max_axis)
            .and_then(|value| value.checked_mul(2))
            .and_then(|value| value.checked_mul(sample_bytes))
            .ok_or(TransformError::SizeOverflow)?;
        let transpose_tasks = worker_count;
        let transpose_worker_scratch = transpose_tasks
            .checked_mul(max_axis)
            .and_then(|value| value.checked_mul(TRANSPOSE_BLOCK as u64 + 1))
            .and_then(|value| value.checked_mul(sample_bytes))
            .ok_or(TransformError::SizeOverflow)?;
        let coefficient_bytes = active_samples
            .checked_mul(sample_bytes)
            .ok_or(TransformError::SizeOverflow)?;
        let stripe_width =
            self.levels
                .last()
                .map_or(0, |level| level.width.min(DEFAULT_STRIPE_WIDTH)) as u64;
        let stripe_bytes = stripe_width
            .checked_mul(self.levels.last().map_or(0, |level| level.height) as u64)
            .and_then(|value| value.checked_mul(sample_bytes))
            .ok_or(TransformError::SizeOverflow)?;
        let estimated_parallel_work = self.levels.iter().try_fold(0_u64, |total, level| {
            let samples = (level.width as u64)
                .checked_mul(level.height as u64)
                .ok_or(TransformError::SizeOverflow)?;
            total
                .checked_add(samples.saturating_mul(2))
                .ok_or(TransformError::SizeOverflow)
        })?;
        Ok(FullSynthesisEstimate {
            legacy_bytes: coefficient_bytes
                .checked_add(max_axis.saturating_mul(2).saturating_mul(sample_bytes))
                .ok_or(TransformError::SizeOverflow)?,
            striped_bytes: coefficient_bytes
                .checked_add(worker_scratch)
                .and_then(|value| value.checked_add(stripe_bytes))
                .ok_or(TransformError::SizeOverflow)?,
            transpose_bytes: coefficient_bytes
                .checked_mul(2)
                .and_then(|value| value.checked_add(transpose_worker_scratch))
                .ok_or(TransformError::SizeOverflow)?,
            active_samples,
            estimated_parallel_work,
        })
    }

    fn validate_geometry(&self) -> Result<(), TransformError> {
        if self.levels.is_empty() {
            return Err(TransformError::EmptyPlane);
        }
        let mut previous = None;
        for level in &self.levels {
            if level.width == 0 || level.height == 0 {
                return Err(TransformError::EmptyPlane);
            }
            if self.stride < level.width {
                return Err(TransformError::StrideTooSmall);
            }
            validate_axis(
                level.width,
                level.horizontal_low_samples,
                level.horizontal_first,
            )?;
            validate_axis(
                level.height,
                level.vertical_low_samples,
                level.vertical_first,
            )?;
            if previous.is_some_and(|(width, height)| level.width < width || level.height < height)
            {
                return Err(TransformError::InvalidEdges);
            }
            previous = Some((level.width, level.height));
        }
        Ok(())
    }
}

fn validate_axis(
    len: usize,
    low_samples: usize,
    first: TransformBand,
) -> Result<(), TransformError> {
    let expected = match first {
        TransformBand::Low => len.div_ceil(2),
        TransformBand::High => len / 2,
    };
    if low_samples != expected {
        return Err(TransformError::InvalidEdges);
    }
    Ok(())
}

/// Retained-memory estimate including the caller's coefficient plane.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FullSynthesisEstimate {
    pub legacy_bytes: u64,
    pub striped_bytes: u64,
    pub transpose_bytes: u64,
    pub active_samples: u64,
    pub estimated_parallel_work: u64,
}

/// Capacity accounting for one typed full-synthesis workspace.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FullWorkspaceCapacities {
    pub stripe: usize,
    pub transpose: usize,
    pub worker_lines: usize,
    pub worker_work: usize,
}

#[derive(Debug, Clone, Default)]
struct FullWorkerWorkspace<T> {
    line: Vec<T>,
    work: Vec<T>,
}

/// Reusable scratch for legacy, striped, and transpose full synthesis.
#[derive(Debug, Clone, Default)]
pub struct FullSynthesisWorkspace<T> {
    stripe: Vec<T>,
    transpose: Vec<T>,
    workers: Vec<FullWorkerWorkspace<T>>,
}

impl<T> FullSynthesisWorkspace<T> {
    pub fn capacities(&self) -> FullWorkspaceCapacities {
        FullWorkspaceCapacities {
            stripe: self.stripe.capacity(),
            transpose: self.transpose.capacity(),
            worker_lines: self
                .workers
                .iter()
                .map(|worker| worker.line.capacity())
                .sum(),
            worker_work: self
                .workers
                .iter()
                .map(|worker| worker.work.capacity())
                .sum(),
        }
    }

    pub fn capacity_samples(&self) -> usize {
        let capacities = self.capacities();
        capacities
            .stripe
            .saturating_add(capacities.transpose)
            .saturating_add(capacities.worker_lines)
            .saturating_add(capacities.worker_work)
    }

    pub fn retained_heap_bytes(&self) -> u64 {
        (self.capacity_samples() as u64).saturating_mul(size_of::<T>() as u64)
    }

    /// Final column-major output retained by a deferred-transpose execution.
    ///
    /// The slice is empty for legacy, striped, and ordinary transpose runs.
    pub fn deferred_transposed_output(&self) -> &[T] {
        &self.transpose
    }

    pub fn clear(&mut self) {
        self.stripe.clear();
        self.transpose.clear();
        for worker in &mut self.workers {
            worker.line.clear();
            worker.work.clear();
        }
    }
}

#[cfg(feature = "parallel")]
impl<T: Copy + Default> FullSynthesisWorkspace<T> {
    fn reserve(
        &mut self,
        plan: &FullSynthesisPlan,
        backend: FullSynthesisBackend,
        workers: usize,
    ) -> Result<(), TransformError> {
        let worker_count = match backend {
            FullSynthesisBackend::LegacyScalar => 1,
            FullSynthesisBackend::ParallelStriped => workers.max(1),
            FullSynthesisBackend::ParallelTranspose => workers.max(1),
        };
        self.workers.truncate(worker_count);
        self.workers
            .resize_with(worker_count, FullWorkerWorkspace::default);
        let max_axis = plan.max_axis();
        for worker in &mut self.workers[..worker_count] {
            let line_samples = if backend == FullSynthesisBackend::ParallelTranspose {
                max_axis
                    .checked_mul(TRANSPOSE_BLOCK)
                    .ok_or(TransformError::SizeOverflow)?
            } else {
                max_axis
            };
            if worker.line.capacity() > line_samples {
                worker.line = Vec::new();
            }
            if worker.work.capacity() > max_axis {
                worker.work = Vec::new();
            }
            worker.line.resize(line_samples, T::default());
            worker.work.resize(max_axis, T::default());
        }
        let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
        match backend {
            FullSynthesisBackend::LegacyScalar => {
                self.stripe = Vec::new();
                self.transpose = Vec::new();
            }
            FullSynthesisBackend::ParallelStriped => {
                let stripe_width = final_level.width.min(DEFAULT_STRIPE_WIDTH);
                let stripe_samples = stripe_width
                    .checked_mul(final_level.height)
                    .ok_or(TransformError::SizeOverflow)?;
                if self.stripe.capacity() > stripe_samples {
                    self.stripe = Vec::new();
                }
                self.stripe.resize(stripe_samples, T::default());
                self.transpose = Vec::new();
            }
            FullSynthesisBackend::ParallelTranspose => {
                let transpose_samples = final_level
                    .width
                    .checked_mul(final_level.height)
                    .ok_or(TransformError::SizeOverflow)?;
                if self.transpose.capacity() > transpose_samples {
                    self.transpose = Vec::new();
                }
                self.transpose.resize(transpose_samples, T::default());
                self.stripe = Vec::new();
            }
        }
        Ok(())
    }

    /// Materialize the exact storage required by one execution and return its
    /// allocator-observed retained byte capacity.
    ///
    /// Callers with a hard memory ceiling use this before synthesis begins so
    /// ordinary `Vec` over-allocation is part of admission rather than merely
    /// post-execution telemetry. Calling the synthesis API afterwards with the
    /// same plan, backend, and worker count does not grow these buffers again.
    pub fn prepare_for_execution(
        &mut self,
        plan: &FullSynthesisPlan,
        backend: FullSynthesisBackend,
        workers: usize,
    ) -> Result<u64, TransformError> {
        if workers == 0 {
            return Err(TransformError::InvalidWindow);
        }
        self.reserve(plan, backend, workers)?;
        Ok(self.retained_heap_bytes())
    }
}

/// Work and stage provenance returned by full synthesis.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct FullSynthesisReport {
    pub backend: FullSynthesisBackend,
    pub horizontal_workers: usize,
    pub vertical_workers: usize,
    pub horizontal_ns: u128,
    pub vertical_ns: u128,
    pub level_preparation_ns: u128,
    pub horizontal_kernel_ns: u128,
    pub vertical_kernel_ns: u128,
    pub horizontal_startup_and_barrier_ns: u128,
    pub vertical_startup_and_barrier_ns: u128,
    pub horizontal_values: u64,
    pub vertical_values: u64,
    pub lifting_updates: u64,
    pub output_samples: u64,
    pub peak_workspace_bytes: u64,
    /// The final transpose-back was deliberately deferred so the caller can
    /// fuse it with sample packing from `FullSynthesisWorkspace`.
    pub output_transposed: bool,
    /// Final unsigned-8 samples were packed directly into caller rows.
    pub output_direct: bool,
}

#[cfg(feature = "parallel")]
trait FullSample: Copy + Default + Send + Sync {
    const TRANSFORM: WaveletTransform;
    const UPDATES_PER_VALUE: u64;

    fn inverse_line(line: &mut [Self], low_samples: usize, first: TransformBand, work: &mut [Self]);

    fn inverse_line_from_read(
        read: &mut [Self],
        output: &mut [Self],
        low_samples: usize,
        first: TransformBand,
    );

    fn inverse_line_from_separate_low(
        low: &mut [Self],
        output: &mut [Self],
        low_samples: usize,
        first: TransformBand,
    ) -> Option<Result<(), TransformError>>;

    fn inverse_vertical_direct_unsigned_u8(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output_region: AxisAlignedRegion,
        output: &mut [u8],
        output_stride: usize,
    ) -> Option<Result<(), TransformError>>;

    #[allow(clippy::too_many_arguments)]
    fn inverse_vertical_direct_unsigned_u8_parallel(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output_region: AxisAlignedRegion,
        output: &mut [u8],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>>;

    fn inverse_vertical_to_row_major(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output: &mut [Self],
        output_stride: usize,
    ) -> Option<Result<(), TransformError>>;

    fn inverse_vertical_to_row_major_parallel(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output: &mut [Self],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>>;

    fn validate_output(
        plane: &[Self],
        stride: usize,
        width: usize,
        height: usize,
    ) -> Result<(), TransformError>;

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

    fn pack_unsigned_u8_block(
        output: &mut emuella_j2k_accel::U8OutputColumns<'_>,
        local_column: usize,
        source: &[Self],
        columns: usize,
    ) -> Result<(), TransformError>;
}

#[cfg(feature = "parallel")]
impl FullSample for i32 {
    const TRANSFORM: WaveletTransform = WaveletTransform::Reversible53;
    const UPDATES_PER_VALUE: u64 = 2;

    fn inverse_line(
        line: &mut [Self],
        low_samples: usize,
        first: TransformBand,
        work: &mut [Self],
    ) {
        if line.len() <= 1 {
            return;
        }
        work[..line.len()].copy_from_slice(line);
        Self::inverse_line_from_read(&mut work[..line.len()], line, low_samples, first);
    }

    fn inverse_line_from_read(
        read: &mut [Self],
        output: &mut [Self],
        low_samples: usize,
        first: TransformBand,
    ) {
        if output.len() == 1 {
            output[0] = read[0];
        } else if first == TransformBand::Low && low_samples.checked_mul(2) == Some(output.len()) {
            if emuella_j2k_accel::inverse_reversible_5_3_even_first_low(read, output, low_samples)
                .is_err()
            {
                transform_line_inverse_from_read_bounded(
                    output.len(),
                    low_samples,
                    first,
                    read,
                    output,
                );
            }
        } else {
            transform_line_inverse_from_read_bounded(
                output.len(),
                low_samples,
                first,
                read,
                output,
            );
        }
    }

    fn inverse_line_from_separate_low(
        low: &mut [Self],
        output: &mut [Self],
        low_samples: usize,
        first: TransformBand,
    ) -> Option<Result<(), TransformError>> {
        if first != TransformBand::Low || low_samples.checked_mul(2) != Some(output.len()) {
            return None;
        }
        Some(
            emuella_j2k_accel::inverse_reversible_5_3_even_first_low_split(
                low,
                output,
                low_samples,
            )
            .map_err(|_| TransformError::SizeOverflow),
        )
    }

    fn inverse_vertical_direct_unsigned_u8(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output_region: AxisAlignedRegion,
        output: &mut [u8],
        output_stride: usize,
    ) -> Option<Result<(), TransformError>> {
        if level.vertical_first != TransformBand::Low
            || level.vertical_low_samples.checked_mul(2) != Some(level.height)
        {
            return None;
        }
        Some(
            emuella_j2k_accel::inverse_reversible_5_3_vertical_even_first_low_to_unsigned_u8_region(
                plane,
                stride,
                level.width,
                level.height,
                output_region.x as usize,
                output_region.y as usize,
                output_region.width as usize,
                output_region.height as usize,
                output,
                output_stride,
            )
            .map_err(|_| TransformError::SizeOverflow),
        )
    }

    fn inverse_vertical_direct_unsigned_u8_parallel(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output_region: AxisAlignedRegion,
        output: &mut [u8],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        use rayon::prelude::*;

        if level.vertical_first != TransformBand::Low
            || level.vertical_low_samples.checked_mul(2) != Some(level.height)
        {
            return None;
        }
        Some((|| {
            let output_width =
                usize::try_from(output_region.width).map_err(|_| TransformError::SizeOverflow)?;
            let output_height =
                usize::try_from(output_region.height).map_err(|_| TransformError::SizeOverflow)?;
            let output_x =
                usize::try_from(output_region.x).map_err(|_| TransformError::SizeOverflow)?;
            let output_y =
                usize::try_from(output_region.y).map_err(|_| TransformError::SizeOverflow)?;
            let desired_workers = worker_count.min(output_width).max(1);
            let chunk_columns = output_width.div_ceil(desired_workers);
            let active_workers = output_width.div_ceil(chunk_columns);
            let plane_root = emuella_j2k_accel::I32PlaneColumns::new(
                plane,
                stride,
                level.width,
                level.height,
                output_x,
                output_width,
            )
            .map_err(|_| TransformError::SizeOverflow)?;
            let output_root = emuella_j2k_accel::U8OutputColumns::new(
                output,
                output_stride,
                output_width,
                output_height,
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
                        .inverse_reversible_5_3_even_first_low_to_unsigned_u8(
                            output_y,
                            output_height,
                            &mut output_job,
                        )
                        .map_err(|_| TransformError::SizeOverflow)?;
                    record_maximum_task_ns(&maximum, started, elapsed_ns);
                    Ok(())
                })?;
            Ok(maximum_task_ns(&maximum))
        })())
    }

    fn inverse_vertical_to_row_major(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output: &mut [Self],
        output_stride: usize,
    ) -> Option<Result<(), TransformError>> {
        if level.vertical_first != TransformBand::Low
            || level.vertical_low_samples.checked_mul(2) != Some(level.height)
        {
            return None;
        }
        Some(
            emuella_j2k_accel::inverse_reversible_5_3_vertical_even_first_low_to_row_major_i32(
                plane,
                stride,
                level.width,
                level.height,
                output,
                output_stride,
            )
            .map_err(|_| TransformError::SizeOverflow),
        )
    }

    fn inverse_vertical_to_row_major_parallel(
        plane: &mut [Self],
        stride: usize,
        level: FullSynthesisLevel,
        output: &mut [Self],
        output_stride: usize,
        worker_count: usize,
        collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        use rayon::prelude::*;

        if level.vertical_first != TransformBand::Low
            || level.vertical_low_samples.checked_mul(2) != Some(level.height)
        {
            return None;
        }
        Some((|| {
            let desired_workers = worker_count.min(level.width).max(1);
            let chunk_columns = level.width.div_ceil(desired_workers);
            let active_workers = level.width.div_ceil(chunk_columns);
            let plane_root = emuella_j2k_accel::I32PlaneColumns::new(
                plane,
                stride,
                level.width,
                level.height,
                0,
                level.width,
            )
            .map_err(|_| TransformError::SizeOverflow)?;
            let output_root = emuella_j2k_accel::I32PlaneColumns::new(
                output,
                output_stride,
                level.width,
                level.height,
                0,
                level.width,
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
                            0,
                            level.height,
                            &mut output_job,
                        )
                        .map_err(|_| TransformError::SizeOverflow)?;
                    record_maximum_task_ns(&maximum, started, elapsed_ns);
                    Ok(())
                })?;
            Ok(maximum_task_ns(&maximum))
        })())
    }

    fn validate_output(
        _plane: &[Self],
        _stride: usize,
        _width: usize,
        _height: usize,
    ) -> Result<(), TransformError> {
        Ok(())
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

    fn pack_unsigned_u8_block(
        output: &mut emuella_j2k_accel::U8OutputColumns<'_>,
        local_column: usize,
        source: &[Self],
        columns: usize,
    ) -> Result<(), TransformError> {
        output
            .pack_transposed_block(local_column, source, columns)
            .map_err(|_| TransformError::SizeOverflow)
    }
}

#[cfg(feature = "parallel")]
impl FullSample for f32 {
    const TRANSFORM: WaveletTransform = WaveletTransform::Irreversible97;
    const UPDATES_PER_VALUE: u64 = 4;

    fn inverse_line(
        line: &mut [Self],
        low_samples: usize,
        first: TransformBand,
        work: &mut [Self],
    ) {
        inverse_irreversible_9_7_line(line, low_samples, first, work);
    }

    fn inverse_line_from_read(
        read: &mut [Self],
        output: &mut [Self],
        low_samples: usize,
        first: TransformBand,
    ) {
        inverse_irreversible_9_7_line_from_read(read, output, low_samples, first);
    }

    fn inverse_line_from_separate_low(
        _low: &mut [Self],
        _output: &mut [Self],
        _low_samples: usize,
        _first: TransformBand,
    ) -> Option<Result<(), TransformError>> {
        None
    }

    fn inverse_vertical_direct_unsigned_u8(
        _plane: &mut [Self],
        _stride: usize,
        _level: FullSynthesisLevel,
        _output_region: AxisAlignedRegion,
        _output: &mut [u8],
        _output_stride: usize,
    ) -> Option<Result<(), TransformError>> {
        None
    }

    fn inverse_vertical_direct_unsigned_u8_parallel(
        _plane: &mut [Self],
        _stride: usize,
        _level: FullSynthesisLevel,
        _output_region: AxisAlignedRegion,
        _output: &mut [u8],
        _output_stride: usize,
        _worker_count: usize,
        _collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        None
    }

    fn inverse_vertical_to_row_major(
        _plane: &mut [Self],
        _stride: usize,
        _level: FullSynthesisLevel,
        _output: &mut [Self],
        _output_stride: usize,
    ) -> Option<Result<(), TransformError>> {
        None
    }

    fn inverse_vertical_to_row_major_parallel(
        _plane: &mut [Self],
        _stride: usize,
        _level: FullSynthesisLevel,
        _output: &mut [Self],
        _output_stride: usize,
        _worker_count: usize,
        _collect_stage_timings: bool,
    ) -> Option<Result<u128, TransformError>> {
        None
    }

    fn validate_output(
        plane: &[Self],
        stride: usize,
        width: usize,
        height: usize,
    ) -> Result<(), TransformError> {
        for y in 0..height {
            for x in 0..width {
                let index = y * stride + x;
                if !plane[index].is_finite() {
                    return Err(TransformError::NonFiniteSample { index });
                }
            }
        }
        Ok(())
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

    fn pack_unsigned_u8_block(
        _output: &mut emuella_j2k_accel::U8OutputColumns<'_>,
        _local_column: usize,
        _source: &[Self],
        _columns: usize,
    ) -> Result<(), TransformError> {
        Err(TransformError::InvalidWindow)
    }
}

#[cfg(feature = "std")]
#[cfg(feature = "parallel")]
fn stage_start(enabled: bool) -> Option<std::time::Instant> {
    enabled.then(std::time::Instant::now)
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "parallel")]
fn stage_start(_enabled: bool) -> Option<()> {
    None
}

#[cfg(feature = "std")]
#[cfg(feature = "parallel")]
fn elapsed_ns(started: Option<std::time::Instant>) -> u128 {
    started.map_or(0, |started| started.elapsed().as_nanos())
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "parallel")]
fn elapsed_ns(_started: Option<()>) -> u128 {
    0
}

#[cfg(feature = "parallel")]
fn record_maximum_task_ns<S>(
    maximum: &core::sync::atomic::AtomicU64,
    started: Option<S>,
    elapsed: impl FnOnce(Option<S>) -> u128,
) {
    use core::sync::atomic::Ordering;

    let elapsed = u64::try_from(elapsed(started)).unwrap_or(u64::MAX);
    maximum.fetch_max(elapsed, Ordering::Relaxed);
}

#[cfg(feature = "parallel")]
fn maximum_task_ns(maximum: &core::sync::atomic::AtomicU64) -> u128 {
    use core::sync::atomic::Ordering;

    u128::from(maximum.load(Ordering::Relaxed))
}

#[cfg(feature = "parallel")]
fn validate_plane<T>(plane: &[T], plan: &FullSynthesisPlan) -> Result<(), TransformError> {
    plan.validate_geometry()?;
    let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
    let required = (final_level.height - 1)
        .checked_mul(plan.stride)
        .and_then(|offset| offset.checked_add(final_level.width))
        .ok_or(TransformError::SizeOverflow)?;
    if plane.len() < required {
        return Err(TransformError::PlaneTooSmall);
    }
    Ok(())
}

#[cfg(feature = "parallel")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum PendingVerticalLayout {
    Transposed,
    RowMajor,
    Direct,
}

#[cfg(feature = "parallel")]
#[derive(Clone, Copy)]
struct PendingVertical {
    level: FullSynthesisLevel,
    layout: PendingVerticalLayout,
}

#[cfg(feature = "parallel")]
fn parallel_horizontal<T: FullSample>(
    plane: &mut [T],
    stride: usize,
    level: FullSynthesisLevel,
    workers: &mut [FullWorkerWorkspace<T>],
    mut pending_low: Option<(&mut [T], PendingVertical)>,
    collect_stage_timings: bool,
) -> Result<u128, TransformError> {
    use rayon::prelude::*;

    if pending_low.as_ref().is_some_and(|(_, pending)| {
        pending.layout == PendingVerticalLayout::RowMajor
            && pending.level.width != level.horizontal_low_samples
    }) {
        return Err(TransformError::InvalidEdges);
    }

    if workers.len() == 1
        && pending_low
            .as_ref()
            .is_some_and(|(_, pending)| pending.layout == PendingVerticalLayout::RowMajor)
    {
        let started = stage_start(collect_stage_timings);
        let (row_major, pending) = pending_low.take().ok_or(TransformError::InvalidWindow)?;
        let previous = pending.level;
        let worker = &mut workers[0];
        for (row_index, row) in plane[..level.height * stride]
            .chunks_mut(stride)
            .enumerate()
        {
            if row_index < previous.height {
                let source_start = row_index
                    .checked_mul(previous.width)
                    .ok_or(TransformError::SizeOverflow)?;
                let source = row_major
                    .get_mut(source_start..source_start + previous.width)
                    .ok_or(TransformError::PlaneTooSmall)?;
                if let Some(result) = T::inverse_line_from_separate_low(
                    source,
                    &mut row[..level.width],
                    level.horizontal_low_samples,
                    level.horizontal_first,
                ) {
                    result?;
                } else {
                    let read = &mut worker.work[..level.width];
                    read[..previous.width].copy_from_slice(source);
                    read[previous.width..].copy_from_slice(&row[previous.width..level.width]);
                    T::inverse_line_from_read(
                        read,
                        &mut row[..level.width],
                        level.horizontal_low_samples,
                        level.horizontal_first,
                    );
                }
            } else {
                T::inverse_line(
                    &mut row[..level.width],
                    level.horizontal_low_samples,
                    level.horizontal_first,
                    &mut worker.work[..level.width],
                );
            }
        }
        return Ok(elapsed_ns(started));
    }

    let pending_low = pending_low.map(|(samples, pending)| (&*samples, pending));

    let worker_count = workers.len().min(level.height).max(1);
    let chunk_rows = level.height.div_ceil(worker_count);
    let maximum = core::sync::atomic::AtomicU64::new(0);
    plane[..level.height * stride]
        .par_chunks_mut(chunk_rows * stride)
        .zip(workers[..worker_count].par_iter_mut())
        .enumerate()
        .try_for_each(|(chunk_index, (rows, worker))| {
            let started = stage_start(collect_stage_timings);
            if let Some((transposed, pending)) = pending_low
                && pending.layout == PendingVerticalLayout::Transposed
            {
                let previous = pending.level;
                let first_row = chunk_index * chunk_rows;
                if first_row < previous.height {
                    let rows_in_chunk = (rows.len() / stride).min(previous.height - first_row);
                    T::transpose_stripe(
                        transposed,
                        previous.height,
                        previous.height,
                        previous.width,
                        first_row,
                        rows_in_chunk,
                        rows,
                        stride,
                    )?;
                }
            }
            for (local_row, row) in rows.chunks_mut(stride).enumerate() {
                let row_index = chunk_index * chunk_rows + local_row;
                let used_pending_row = if let Some((row_major, pending)) = pending_low {
                    if pending.layout == PendingVerticalLayout::RowMajor
                        && row_index < pending.level.height
                    {
                        let previous = pending.level;
                        let source_start = row_index
                            .checked_mul(previous.width)
                            .ok_or(TransformError::SizeOverflow)?;
                        let source = row_major
                            .get(source_start..source_start + previous.width)
                            .ok_or(TransformError::PlaneTooSmall)?;
                        let read = &mut worker.work[..level.width];
                        read[..previous.width].copy_from_slice(source);
                        read[previous.width..].copy_from_slice(&row[previous.width..level.width]);
                        T::inverse_line_from_read(
                            read,
                            &mut row[..level.width],
                            level.horizontal_low_samples,
                            level.horizontal_first,
                        );
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                if !used_pending_row {
                    T::inverse_line(
                        &mut row[..level.width],
                        level.horizontal_low_samples,
                        level.horizontal_first,
                        &mut worker.work[..level.width],
                    );
                }
            }
            record_maximum_task_ns(&maximum, started, elapsed_ns);
            Ok(())
        })?;
    Ok(maximum_task_ns(&maximum))
}

#[cfg(feature = "parallel")]
fn legacy_vertical<T: FullSample>(
    plane: &mut [T],
    stride: usize,
    level: FullSynthesisLevel,
    worker: &mut FullWorkerWorkspace<T>,
) {
    for x in 0..level.width {
        for y in 0..level.height {
            worker.line[y] = plane[y * stride + x];
        }
        T::inverse_line(
            &mut worker.line[..level.height],
            level.vertical_low_samples,
            level.vertical_first,
            &mut worker.work[..level.height],
        );
        for y in 0..level.height {
            plane[y * stride + x] = worker.line[y];
        }
    }
}

#[cfg(feature = "parallel")]
fn striped_vertical<T: FullSample>(
    plane: &mut [T],
    stride: usize,
    level: FullSynthesisLevel,
    workspace: &mut FullSynthesisWorkspace<T>,
    worker_count: usize,
    collect_stage_timings: bool,
) -> u128 {
    use rayon::prelude::*;

    let stripe_width = level.width.min(DEFAULT_STRIPE_WIDTH);
    let mut critical_ns = 0_u128;
    for first_column in (0..level.width).step_by(stripe_width) {
        let columns = (level.width - first_column).min(stripe_width);
        let stripe = &mut workspace.stripe[..columns * level.height];
        for column in 0..columns {
            let destination = &mut stripe[column * level.height..(column + 1) * level.height];
            for (row, sample) in destination.iter_mut().enumerate() {
                *sample = plane[row * stride + first_column + column];
            }
        }
        let active_workers = worker_count.min(columns).max(1);
        let chunk_columns = columns.div_ceil(active_workers);
        let maximum = core::sync::atomic::AtomicU64::new(0);
        stripe
            .par_chunks_mut(chunk_columns * level.height)
            .zip(workspace.workers[..active_workers].par_iter_mut())
            .for_each(|(column_chunk, worker)| {
                let started = stage_start(collect_stage_timings);
                for column in column_chunk.chunks_mut(level.height) {
                    T::inverse_line(
                        column,
                        level.vertical_low_samples,
                        level.vertical_first,
                        &mut worker.work[..level.height],
                    );
                }
                record_maximum_task_ns(&maximum, started, elapsed_ns);
            });
        critical_ns = critical_ns.saturating_add(maximum_task_ns(&maximum));
        for column in 0..columns {
            let source = &stripe[column * level.height..(column + 1) * level.height];
            for (row, sample) in source.iter().copied().enumerate() {
                plane[row * stride + first_column + column] = sample;
            }
        }
    }
    critical_ns
}

#[cfg(feature = "parallel")]
// This internal phase boundary keeps geometry, workspace, scheduling, and
// optional direct-output state explicit; bundling them would obscure ownership.
#[allow(clippy::too_many_arguments)]
fn transpose_vertical<T: FullSample>(
    plane: &mut [T],
    stride: usize,
    level: FullSynthesisLevel,
    workspace: &mut FullSynthesisWorkspace<T>,
    worker_count: usize,
    allow_row_major_intermediate: bool,
    intermediate_level: bool,
    scatter_back: bool,
    mut direct_unsigned_u8: Option<(&mut [u8], usize, AxisAlignedRegion)>,
    collect_stage_timings: bool,
) -> Result<(PendingVerticalLayout, u128), TransformError> {
    use rayon::prelude::*;

    if worker_count > 1
        && let Some((output, output_stride, output_region)) = direct_unsigned_u8.as_mut()
        && let Some(result) = T::inverse_vertical_direct_unsigned_u8_parallel(
            plane,
            stride,
            level,
            *output_region,
            output,
            *output_stride,
            worker_count,
            collect_stage_timings,
        )
    {
        return Ok((PendingVerticalLayout::Direct, result?));
    }
    if worker_count > 1 && intermediate_level && allow_row_major_intermediate {
        let row_major = &mut workspace.transpose[..level.width * level.height];
        if let Some(result) = T::inverse_vertical_to_row_major_parallel(
            plane,
            stride,
            level,
            row_major,
            level.width,
            worker_count,
            collect_stage_timings,
        ) {
            return Ok((PendingVerticalLayout::RowMajor, result?));
        }
    }
    if worker_count == 1 {
        if let Some((output, output_stride, output_region)) = direct_unsigned_u8.as_mut() {
            let started = stage_start(collect_stage_timings);
            if let Some(result) = T::inverse_vertical_direct_unsigned_u8(
                plane,
                stride,
                level,
                *output_region,
                output,
                *output_stride,
            ) {
                result?;
                return Ok((PendingVerticalLayout::Direct, elapsed_ns(started)));
            }
        }
        if intermediate_level && allow_row_major_intermediate {
            let row_major = &mut workspace.transpose[..level.width * level.height];
            let started = stage_start(collect_stage_timings);
            if let Some(result) =
                T::inverse_vertical_to_row_major(plane, stride, level, row_major, level.width)
            {
                result?;
                return Ok((PendingVerticalLayout::RowMajor, elapsed_ns(started)));
            }
        }
    }

    let desired_tasks = worker_count
        .min(workspace.workers.len())
        .min(level.width)
        .max(1);
    let chunk_columns = level.width.div_ceil(desired_tasks);
    let active_workers = level.width.div_ceil(chunk_columns);
    let (transposed_storage, workers) = (&mut workspace.transpose, &mut workspace.workers);
    let transposed = &mut transposed_storage[..level.width * level.height];
    if let Some((output, output_stride, output_region)) = direct_unsigned_u8 {
        if output_region.x != 0
            || output_region.y != 0
            || output_region.width as usize != level.width
            || output_region.height as usize != level.height
        {
            return Err(TransformError::InvalidWindow);
        }
        let root = emuella_j2k_accel::U8OutputColumns::new(
            output,
            output_stride,
            level.width,
            level.height,
        )
        .map_err(|_| TransformError::SizeOverflow)?;
        let mut output_stripes = Vec::with_capacity(active_workers);
        let mut remainder = Some(root);
        for worker_index in 0..active_workers {
            let token = remainder.take().ok_or(TransformError::SizeOverflow)?;
            if worker_index + 1 == active_workers {
                output_stripes.push(token);
            } else {
                let (left, right) = token
                    .split_at(chunk_columns)
                    .map_err(|_| TransformError::SizeOverflow)?;
                output_stripes.push(left);
                remainder = Some(right);
            }
        }
        let mut scratch_stripes = Vec::with_capacity(active_workers);
        let mut scratch_remainder = &mut transposed[..];
        for worker_index in 0..active_workers {
            let first_column = worker_index * chunk_columns;
            let worker_columns = (level.width - first_column).min(chunk_columns);
            let scratch_columns = worker_columns.min(TRANSPOSE_BLOCK);
            let scratch_samples = scratch_columns
                .checked_mul(level.height)
                .ok_or(TransformError::SizeOverflow)?;
            if scratch_samples > scratch_remainder.len() {
                return Err(TransformError::SizeOverflow);
            }
            let (scratch, remaining) = scratch_remainder.split_at_mut(scratch_samples);
            scratch_stripes.push(scratch);
            scratch_remainder = remaining;
        }
        let maximum = core::sync::atomic::AtomicU64::new(0);
        scratch_stripes
            .into_par_iter()
            .zip(workers[..active_workers].par_iter_mut())
            .zip(output_stripes.into_par_iter())
            .enumerate()
            .try_for_each(
                |(worker_index, ((column_chunk, worker), mut output_stripe))| {
                    let started = stage_start(collect_stage_timings);
                    let worker_first_column = worker_index * chunk_columns;
                    let worker_columns = (level.width - worker_first_column).min(chunk_columns);
                    for local_first_column in (0..worker_columns).step_by(TRANSPOSE_BLOCK) {
                        let columns = (worker_columns - local_first_column).min(TRANSPOSE_BLOCK);
                        let first_column = worker_first_column + local_first_column;
                        // Direct output consumes this block immediately, so
                        // reuse one hot destination block instead of dirtying
                        // the entire retained transpose plane at the final
                        // resolution.
                        let output_block = &mut column_chunk[..columns * level.height];
                        let input_block = &mut worker.line[..columns * level.height];
                        T::transpose_stripe(
                            plane,
                            stride,
                            level.width,
                            level.height,
                            first_column,
                            columns,
                            input_block,
                            level.height,
                        )?;
                        for (read, output) in input_block
                            .chunks_mut(level.height)
                            .zip(output_block.chunks_mut(level.height))
                        {
                            T::inverse_line_from_read(
                                read,
                                output,
                                level.vertical_low_samples,
                                level.vertical_first,
                            );
                        }
                        T::pack_unsigned_u8_block(
                            &mut output_stripe,
                            local_first_column,
                            output_block,
                            columns,
                        )?;
                    }
                    record_maximum_task_ns(&maximum, started, elapsed_ns);
                    Ok(())
                },
            )?;
        return Ok((PendingVerticalLayout::Direct, maximum_task_ns(&maximum)));
    }
    let transform_maximum = core::sync::atomic::AtomicU64::new(0);
    transposed
        .par_chunks_mut(chunk_columns * level.height)
        .zip(workers[..active_workers].par_iter_mut())
        .enumerate()
        .try_for_each(|(worker_index, (column_chunk, worker))| {
            let started = stage_start(collect_stage_timings);
            let worker_first_column = worker_index * chunk_columns;
            let worker_columns = column_chunk.len() / level.height;
            for local_first_column in (0..worker_columns).step_by(TRANSPOSE_BLOCK) {
                let columns = (worker_columns - local_first_column).min(TRANSPOSE_BLOCK);
                let first_column = worker_first_column + local_first_column;
                let first_sample = local_first_column * level.height;
                let output_block =
                    &mut column_chunk[first_sample..first_sample + columns * level.height];
                let input_block = &mut worker.line[..columns * level.height];
                T::transpose_stripe(
                    plane,
                    stride,
                    level.width,
                    level.height,
                    first_column,
                    columns,
                    input_block,
                    level.height,
                )?;
                for (read, output) in input_block
                    .chunks_mut(level.height)
                    .zip(output_block.chunks_mut(level.height))
                {
                    T::inverse_line_from_read(
                        read,
                        output,
                        level.vertical_low_samples,
                        level.vertical_first,
                    );
                }
            }
            record_maximum_task_ns(&transform_maximum, started, elapsed_ns);
            Ok(())
        })?;
    let mut critical_ns = maximum_task_ns(&transform_maximum);
    if scatter_back {
        let scatter_maximum = core::sync::atomic::AtomicU64::new(0);
        plane[..level.height * stride]
            .par_chunks_mut(TRANSPOSE_BLOCK * stride)
            .enumerate()
            .try_for_each(|(block, row_block)| {
                let started = stage_start(collect_stage_timings);
                let first_row = block * TRANSPOSE_BLOCK;
                let rows = row_block.len() / stride;
                T::transpose_stripe(
                    transposed,
                    level.height,
                    level.height,
                    level.width,
                    first_row,
                    rows,
                    row_block,
                    stride,
                )?;
                record_maximum_task_ns(&scatter_maximum, started, elapsed_ns);
                Ok(())
            })?;
        critical_ns = critical_ns.saturating_add(maximum_task_ns(&scatter_maximum));
    }
    Ok((PendingVerticalLayout::Transposed, critical_ns))
}

#[cfg(feature = "parallel")]
// This internal orchestration boundary intentionally exposes independently
// forceable phase and output controls used by differential qualification.
#[allow(clippy::too_many_arguments)]
fn inverse_full_parallel<T: FullSample>(
    plane: &mut [T],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<T>,
    backend: FullSynthesisBackend,
    horizontal_worker_count: usize,
    vertical_worker_count: usize,
    collect_stage_timings: bool,
    defer_final_transpose: bool,
    mut direct_unsigned_u8: Option<(&mut [u8], usize, AxisAlignedRegion)>,
) -> Result<FullSynthesisReport, TransformError> {
    if horizontal_worker_count == 0
        || vertical_worker_count == 0
        || plan.transform != T::TRANSFORM
        || ((defer_final_transpose || direct_unsigned_u8.is_some())
            && backend != FullSynthesisBackend::ParallelTranspose)
        || (defer_final_transpose && direct_unsigned_u8.is_some())
    {
        return Err(TransformError::InvalidWindow);
    }
    validate_plane(plane, plan)?;
    workspace.reserve(
        plan,
        backend,
        horizontal_worker_count.max(vertical_worker_count),
    )?;
    let mut report = FullSynthesisReport {
        backend,
        horizontal_workers: if backend == FullSynthesisBackend::LegacyScalar {
            1
        } else {
            horizontal_worker_count
        },
        vertical_workers: if backend == FullSynthesisBackend::LegacyScalar {
            1
        } else {
            vertical_worker_count
        },
        ..FullSynthesisReport::default()
    };
    let mut pending_vertical = None;
    for (level_index, level) in plan.levels.iter().enumerate() {
        let preparation_started = stage_start(collect_stage_timings);
        report.level_preparation_ns = report
            .level_preparation_ns
            .saturating_add(elapsed_ns(preparation_started));

        let horizontal_started = stage_start(collect_stage_timings);
        let horizontal_critical_ns = match backend {
            FullSynthesisBackend::LegacyScalar => {
                let worker = &mut workspace.workers[0];
                for y in 0..level.height {
                    let start = y * plan.stride;
                    T::inverse_line(
                        &mut plane[start..start + level.width],
                        level.horizontal_low_samples,
                        level.horizontal_first,
                        &mut worker.work[..level.width],
                    );
                }
                0
            }
            FullSynthesisBackend::ParallelStriped | FullSynthesisBackend::ParallelTranspose => {
                let pending_low = pending_vertical
                    .take()
                    .map(|previous| (&mut workspace.transpose[..], previous));
                parallel_horizontal(
                    plane,
                    plan.stride,
                    *level,
                    &mut workspace.workers[..horizontal_worker_count],
                    pending_low,
                    collect_stage_timings,
                )?
            }
        };
        let horizontal_wall_ns = elapsed_ns(horizontal_started);
        let horizontal_kernel_ns = if backend == FullSynthesisBackend::LegacyScalar {
            horizontal_wall_ns
        } else {
            horizontal_critical_ns
        };
        report.horizontal_ns = report.horizontal_ns.saturating_add(horizontal_wall_ns);
        report.horizontal_kernel_ns = report
            .horizontal_kernel_ns
            .saturating_add(horizontal_kernel_ns);
        report.horizontal_startup_and_barrier_ns = report
            .horizontal_startup_and_barrier_ns
            .saturating_add(horizontal_wall_ns.saturating_sub(horizontal_kernel_ns));

        let vertical_started = stage_start(collect_stage_timings);
        let vertical_critical_ns = match backend {
            FullSynthesisBackend::LegacyScalar => {
                legacy_vertical(plane, plan.stride, *level, &mut workspace.workers[0]);
                0
            }
            FullSynthesisBackend::ParallelStriped => striped_vertical(
                plane,
                plan.stride,
                *level,
                workspace,
                vertical_worker_count,
                collect_stage_timings,
            ),
            FullSynthesisBackend::ParallelTranspose => {
                let final_level = level_index + 1 == plan.levels.len();
                let level_vertical_workers = if !final_level && horizontal_worker_count == 1 {
                    1
                } else {
                    vertical_worker_count
                };
                let (layout, critical_ns) = transpose_vertical(
                    plane,
                    plan.stride,
                    *level,
                    workspace,
                    level_vertical_workers,
                    true,
                    !final_level,
                    final_level && !defer_final_transpose,
                    if final_level {
                        direct_unsigned_u8
                            .as_mut()
                            .map(|(output, stride, region)| (&mut **output, *stride, *region))
                    } else {
                        None
                    },
                    collect_stage_timings,
                )?;
                if !final_level {
                    if layout == PendingVerticalLayout::Direct {
                        return Err(TransformError::InvalidWindow);
                    }
                    pending_vertical = Some(PendingVertical {
                        level: *level,
                        layout,
                    });
                }
                critical_ns
            }
        };
        let vertical_wall_ns = elapsed_ns(vertical_started);
        let vertical_kernel_ns = if backend == FullSynthesisBackend::LegacyScalar {
            vertical_wall_ns
        } else {
            vertical_critical_ns
        };
        report.vertical_ns = report.vertical_ns.saturating_add(vertical_wall_ns);
        report.vertical_kernel_ns = report.vertical_kernel_ns.saturating_add(vertical_kernel_ns);
        report.vertical_startup_and_barrier_ns = report
            .vertical_startup_and_barrier_ns
            .saturating_add(vertical_wall_ns.saturating_sub(vertical_kernel_ns));
        let samples = (level.width as u64).saturating_mul(level.height as u64);
        report.horizontal_values = report.horizontal_values.saturating_add(samples);
        report.vertical_values = report.vertical_values.saturating_add(samples);
        report.lifting_updates = report
            .lifting_updates
            .saturating_add(samples.saturating_mul(T::UPDATES_PER_VALUE));
        report.peak_workspace_bytes = report
            .peak_workspace_bytes
            .max(workspace.retained_heap_bytes());
    }
    let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
    if defer_final_transpose || direct_unsigned_u8.is_some() {
        T::validate_output(
            &workspace.transpose,
            final_level.height,
            final_level.height,
            final_level.width,
        )?;
    } else {
        T::validate_output(plane, plan.stride, final_level.width, final_level.height)?;
    }
    report.output_samples = plan.active_samples();
    report.output_transposed = defer_final_transpose;
    report.output_direct = direct_unsigned_u8.is_some();
    Ok(report)
}

/// Execute a selected full-plane reversible 5/3 backend in the current pool.
#[cfg(feature = "parallel")]
pub fn inverse_reversible_5_3_full_parallel(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    backend: FullSynthesisBackend,
    workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        backend,
        workers,
        workers,
        collect_stage_timings,
        false,
        None,
    )
}

/// Execute a selected full-plane reversible 5/3 backend with independent
/// horizontal and vertical worker budgets in the current pool.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_full_parallel_with_workers(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    backend: FullSynthesisBackend,
    horizontal_workers: usize,
    vertical_workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        backend,
        horizontal_workers,
        vertical_workers,
        collect_stage_timings,
        false,
        None,
    )
}

/// Execute transpose-backed reversible synthesis while leaving the final
/// column-major result in the reusable workspace for fused caller packing.
#[cfg(feature = "parallel")]
pub fn inverse_reversible_5_3_full_parallel_deferred_transpose(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        workers,
        workers,
        collect_stage_timings,
        true,
        None,
    )
}

/// Execute deferred-transpose reversible synthesis with independent
/// horizontal and vertical worker budgets in the current pool.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_full_parallel_deferred_transpose_with_workers(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    horizontal_workers: usize,
    vertical_workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        horizontal_workers,
        vertical_workers,
        collect_stage_timings,
        true,
        None,
    )
}

/// Execute transpose-backed reversible synthesis and pack unsigned-8 samples
/// into caller-owned padded rows while each final column block is still hot.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_full_parallel_direct_unsigned_u8(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    workers: usize,
    collect_stage_timings: bool,
    output: &mut [u8],
    output_stride: usize,
) -> Result<FullSynthesisReport, TransformError> {
    let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
    inverse_reversible_5_3_full_parallel_direct_unsigned_u8_region(
        plane,
        plan,
        workspace,
        workers,
        collect_stage_timings,
        AxisAlignedRegion {
            x: 0,
            y: 0,
            width: u32::try_from(final_level.width).map_err(|_| TransformError::SizeOverflow)?,
            height: u32::try_from(final_level.height).map_err(|_| TransformError::SizeOverflow)?,
        },
        output,
        output_stride,
    )
}

/// Execute transpose-backed reversible synthesis and pack one selected
/// unsigned-8 rectangle directly into caller-owned padded rows.
///
/// Parallel execution partitions the requested rectangle into disjoint
/// coefficient and caller-output column intervals while preserving the full
/// vertical lifting dependencies in every interval.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_full_parallel_direct_unsigned_u8_region(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    workers: usize,
    collect_stage_timings: bool,
    output_region: AxisAlignedRegion,
    output: &mut [u8],
    output_stride: usize,
) -> Result<FullSynthesisReport, TransformError> {
    let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
    if output_region.width == 0
        || output_region.height == 0
        || output_region
            .end_x()
            .map_err(|_| TransformError::SizeOverflow)?
            > u32::try_from(final_level.width).map_err(|_| TransformError::SizeOverflow)?
        || output_region
            .end_y()
            .map_err(|_| TransformError::SizeOverflow)?
            > u32::try_from(final_level.height).map_err(|_| TransformError::SizeOverflow)?
    {
        return Err(TransformError::InvalidWindow);
    }
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        workers,
        workers,
        collect_stage_timings,
        false,
        Some((output, output_stride, output_region)),
    )
}

/// Execute transpose-backed reversible synthesis and direct unsigned-8
/// packing with independent horizontal and vertical worker budgets.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_reversible_5_3_full_parallel_direct_unsigned_u8_region_with_workers(
    plane: &mut [i32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<i32>,
    horizontal_workers: usize,
    vertical_workers: usize,
    collect_stage_timings: bool,
    output_region: AxisAlignedRegion,
    output: &mut [u8],
    output_stride: usize,
) -> Result<FullSynthesisReport, TransformError> {
    let final_level = plan.levels.last().ok_or(TransformError::EmptyPlane)?;
    if output_region.width == 0
        || output_region.height == 0
        || output_region
            .end_x()
            .map_err(|_| TransformError::SizeOverflow)?
            > u32::try_from(final_level.width).map_err(|_| TransformError::SizeOverflow)?
        || output_region
            .end_y()
            .map_err(|_| TransformError::SizeOverflow)?
            > u32::try_from(final_level.height).map_err(|_| TransformError::SizeOverflow)?
    {
        return Err(TransformError::InvalidWindow);
    }
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        horizontal_workers,
        vertical_workers,
        collect_stage_timings,
        false,
        Some((output, output_stride, output_region)),
    )
}

/// Execute a selected full-plane irreversible 9/7 backend in the current pool.
#[cfg(feature = "parallel")]
pub fn inverse_irreversible_9_7_full_parallel(
    plane: &mut [f32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<f32>,
    backend: FullSynthesisBackend,
    workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        backend,
        workers,
        workers,
        collect_stage_timings,
        false,
        None,
    )
}

/// Execute a selected full-plane irreversible 9/7 backend with independent
/// horizontal and vertical worker budgets in the current pool.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_irreversible_9_7_full_parallel_with_workers(
    plane: &mut [f32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<f32>,
    backend: FullSynthesisBackend,
    horizontal_workers: usize,
    vertical_workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        backend,
        horizontal_workers,
        vertical_workers,
        collect_stage_timings,
        false,
        None,
    )
}

/// Execute transpose-backed irreversible synthesis while leaving the final
/// column-major result in the reusable workspace for fused caller packing.
#[cfg(feature = "parallel")]
pub fn inverse_irreversible_9_7_full_parallel_deferred_transpose(
    plane: &mut [f32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<f32>,
    workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        workers,
        workers,
        collect_stage_timings,
        true,
        None,
    )
}

/// Execute deferred-transpose irreversible synthesis with independent
/// horizontal and vertical worker budgets in the current pool.
#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub fn inverse_irreversible_9_7_full_parallel_deferred_transpose_with_workers(
    plane: &mut [f32],
    plan: &FullSynthesisPlan,
    workspace: &mut FullSynthesisWorkspace<f32>,
    horizontal_workers: usize,
    vertical_workers: usize,
    collect_stage_timings: bool,
) -> Result<FullSynthesisReport, TransformError> {
    inverse_full_parallel(
        plane,
        plan,
        workspace,
        FullSynthesisBackend::ParallelTranspose,
        horizontal_workers,
        vertical_workers,
        collect_stage_timings,
        true,
        None,
    )
}

#[cfg(all(test, feature = "parallel"))]
mod tests {
    use super::*;

    fn plan(
        transform: WaveletTransform,
        width: usize,
        height: usize,
        stride: usize,
    ) -> FullSynthesisPlan {
        FullSynthesisPlan::new(
            transform,
            stride,
            [(width.div_ceil(2), height.div_ceil(2)), (width, height)]
                .into_iter()
                .map(|(width, height)| {
                    let edges = Reversible53Edges::from_tile_origin(0, 0, width, height);
                    FullSynthesisLevel::reversible(width, height, edges)
                })
                .collect(),
        )
        .unwrap()
    }

    fn ceil_div_pow2(value: usize, shift: usize) -> usize {
        value.div_ceil(1_usize << shift)
    }

    fn parity_plan(
        transform: WaveletTransform,
        x0: usize,
        y0: usize,
        width: usize,
        height: usize,
        levels: usize,
        stride: usize,
    ) -> FullSynthesisPlan {
        let x1 = x0 + width;
        let y1 = y0 + height;
        let synthesis_levels = (1..=levels)
            .map(|resolution| {
                let shift = levels - resolution;
                let level_x0 = ceil_div_pow2(x0, shift);
                let level_y0 = ceil_div_pow2(y0, shift);
                let level_x1 = ceil_div_pow2(x1, shift);
                let level_y1 = ceil_div_pow2(y1, shift);
                let level_width = level_x1 - level_x0;
                let level_height = level_y1 - level_y0;
                match transform {
                    WaveletTransform::Reversible53 => FullSynthesisLevel::reversible(
                        level_width,
                        level_height,
                        Reversible53Edges::from_tile_origin(
                            level_x0,
                            level_y0,
                            level_width,
                            level_height,
                        ),
                    ),
                    WaveletTransform::Irreversible97 => FullSynthesisLevel::irreversible(
                        level_width,
                        level_height,
                        Irreversible97Edges::from_tile_origin(
                            level_x0,
                            level_y0,
                            level_width,
                            level_height,
                        ),
                    ),
                }
            })
            .collect();
        FullSynthesisPlan::new(transform, stride, synthesis_levels).unwrap()
    }

    #[test]
    fn reversible_backends_match_legacy_with_padding_and_odd_dimensions() {
        let plan = plan(WaveletTransform::Reversible53, 31, 23, 37);
        let mut coefficients = vec![0_i32; 37 * 23];
        for y in 0..23 {
            for x in 0..31 {
                coefficients[y * 37 + x] = ((x * 17 + y * 31) % 127) as i32 - 63;
            }
            coefficients[y * 37 + 31..y * 37 + 37].fill(0x5151);
        }
        let mut expected = coefficients.clone();
        inverse_reversible_5_3_full_parallel(
            &mut expected,
            &plan,
            &mut FullSynthesisWorkspace::default(),
            FullSynthesisBackend::LegacyScalar,
            1,
            false,
        )
        .unwrap();
        for backend in [
            FullSynthesisBackend::ParallelStriped,
            FullSynthesisBackend::ParallelTranspose,
        ] {
            for workers in [1, 2, 4, 8, 16] {
                let mut actual = coefficients.clone();
                let report = inverse_reversible_5_3_full_parallel(
                    &mut actual,
                    &plan,
                    &mut FullSynthesisWorkspace::default(),
                    backend,
                    workers,
                    true,
                )
                .unwrap();
                assert_eq!(actual, expected, "backend={backend:?} workers={workers}");
                assert_eq!(report.backend, backend);
                assert_eq!(report.output_samples, 31 * 23);
                assert_eq!(
                    report.horizontal_ns,
                    report
                        .horizontal_kernel_ns
                        .saturating_add(report.horizontal_startup_and_barrier_ns),
                    "horizontal timing reconciliation backend={backend:?} workers={workers}"
                );
                assert_eq!(
                    report.vertical_ns,
                    report
                        .vertical_kernel_ns
                        .saturating_add(report.vertical_startup_and_barrier_ns),
                    "vertical timing reconciliation backend={backend:?} workers={workers}"
                );
                assert!(report.horizontal_kernel_ns > 0);
                assert!(report.vertical_kernel_ns > 0);
            }
        }
        let mut phase_specific = coefficients.clone();
        let report = inverse_reversible_5_3_full_parallel_with_workers(
            &mut phase_specific,
            &plan,
            &mut FullSynthesisWorkspace::default(),
            FullSynthesisBackend::ParallelTranspose,
            1,
            4,
            true,
        )
        .unwrap();
        assert_eq!(phase_specific, expected);
        assert_eq!(report.horizontal_workers, 1);
        assert_eq!(report.vertical_workers, 4);

        let mut horizontal_phase_specific = coefficients.clone();
        let report = inverse_reversible_5_3_full_parallel_with_workers(
            &mut horizontal_phase_specific,
            &plan,
            &mut FullSynthesisWorkspace::default(),
            FullSynthesisBackend::ParallelTranspose,
            4,
            1,
            true,
        )
        .unwrap();
        assert_eq!(horizontal_phase_specific, expected);
        assert_eq!(report.horizontal_workers, 4);
        assert_eq!(report.vertical_workers, 1);
    }

    #[test]
    fn irreversible_backends_are_worker_deterministic() {
        let mut plan = plan(WaveletTransform::Irreversible97, 29, 19, 32);
        for level in &mut plan.levels {
            *level = FullSynthesisLevel::irreversible(
                level.width,
                level.height,
                Irreversible97Edges::from_tile_origin(0, 0, level.width, level.height),
            );
        }
        let mut coefficients = vec![0.0_f32; 32 * 19];
        for y in 0..19 {
            for x in 0..29 {
                coefficients[y * 32 + x] = ((x * 11 + y * 7) % 97) as f32 * 0.125 - 4.0;
            }
        }
        let mut expected = coefficients.clone();
        inverse_irreversible_9_7_full_parallel(
            &mut expected,
            &plan,
            &mut FullSynthesisWorkspace::default(),
            FullSynthesisBackend::LegacyScalar,
            1,
            false,
        )
        .unwrap();
        for backend in [
            FullSynthesisBackend::ParallelStriped,
            FullSynthesisBackend::ParallelTranspose,
        ] {
            for workers in [1, 2, 4, 8, 16] {
                let mut actual = coefficients.clone();
                inverse_irreversible_9_7_full_parallel(
                    &mut actual,
                    &plan,
                    &mut FullSynthesisWorkspace::default(),
                    backend,
                    workers,
                    false,
                )
                .unwrap();
                assert_eq!(actual, expected, "backend={backend:?} workers={workers}");
            }
        }
    }

    #[test]
    fn backends_match_every_origin_parity_and_multiple_level_counts() {
        for transform in [
            WaveletTransform::Reversible53,
            WaveletTransform::Irreversible97,
        ] {
            for (x0, y0) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                for (width, height, levels) in [(17, 15, 1), (31, 24, 2), (34, 29, 3)] {
                    let stride = width + 5;
                    let plan = parity_plan(transform, x0, y0, width, height, levels, stride);
                    match transform {
                        WaveletTransform::Reversible53 => {
                            let mut coefficients = vec![0_i32; stride * height];
                            for y in 0..height {
                                for x in 0..width {
                                    coefficients[y * stride + x] =
                                        ((x * 17 + y * 31 + x * y * 3) % 251) as i32 - 125;
                                }
                                coefficients[y * stride + width..(y + 1) * stride].fill(0x5151);
                            }
                            let mut expected = coefficients.clone();
                            inverse_reversible_5_3_full_parallel(
                                &mut expected,
                                &plan,
                                &mut FullSynthesisWorkspace::default(),
                                FullSynthesisBackend::LegacyScalar,
                                1,
                                false,
                            )
                            .unwrap();
                            for backend in [
                                FullSynthesisBackend::ParallelStriped,
                                FullSynthesisBackend::ParallelTranspose,
                            ] {
                                for workers in [1, 2, 4, 8, 16] {
                                    let mut actual = coefficients.clone();
                                    inverse_reversible_5_3_full_parallel(
                                        &mut actual,
                                        &plan,
                                        &mut FullSynthesisWorkspace::default(),
                                        backend,
                                        workers,
                                        false,
                                    )
                                    .unwrap_or_else(|error| {
                                        panic!(
                                            "transform={transform:?} origin=({x0},{y0}) size={width}x{height} levels={levels} backend={backend:?} workers={workers}: {error:?}"
                                        )
                                    });
                                    assert_eq!(
                                        actual, expected,
                                        "transform={transform:?} origin=({x0},{y0}) size={width}x{height} levels={levels} backend={backend:?} workers={workers}"
                                    );
                                }
                            }
                        }
                        WaveletTransform::Irreversible97 => {
                            let mut coefficients = vec![0.0_f32; stride * height];
                            for y in 0..height {
                                for x in 0..width {
                                    coefficients[y * stride + x] =
                                        ((x * 11 + y * 7 + x * y) % 193) as f32 * 0.125 - 9.0;
                                }
                                coefficients[y * stride + width..(y + 1) * stride].fill(1234.5);
                            }
                            let mut expected = coefficients.clone();
                            inverse_irreversible_9_7_full_parallel(
                                &mut expected,
                                &plan,
                                &mut FullSynthesisWorkspace::default(),
                                FullSynthesisBackend::LegacyScalar,
                                1,
                                false,
                            )
                            .unwrap();
                            for backend in [
                                FullSynthesisBackend::ParallelStriped,
                                FullSynthesisBackend::ParallelTranspose,
                            ] {
                                for workers in [1, 2, 4, 8, 16] {
                                    let mut actual = coefficients.clone();
                                    inverse_irreversible_9_7_full_parallel(
                                        &mut actual,
                                        &plan,
                                        &mut FullSynthesisWorkspace::default(),
                                        backend,
                                        workers,
                                        false,
                                    )
                                    .unwrap();
                                    assert_eq!(
                                        actual, expected,
                                        "transform={transform:?} origin=({x0},{y0}) size={width}x{height} levels={levels} backend={backend:?} workers={workers}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn estimates_and_capacities_distinguish_striped_and_transpose() {
        let plan = plan(WaveletTransform::Reversible53, 4096, 4096, 4096);
        let estimate = plan.estimate::<i32>(16).unwrap();
        assert!(estimate.legacy_bytes < estimate.striped_bytes);
        assert!(estimate.striped_bytes < estimate.transpose_bytes);
        assert_eq!(estimate.active_samples, 4096 * 4096);

        let coefficients = vec![0_i32; 4096 * 4096];
        let mut workspace = FullSynthesisWorkspace::default();
        let mut first = coefficients.clone();
        inverse_reversible_5_3_full_parallel(
            &mut first,
            &plan,
            &mut workspace,
            FullSynthesisBackend::ParallelStriped,
            4,
            false,
        )
        .unwrap();
        let capacities = workspace.capacities();
        let mut second = coefficients;
        inverse_reversible_5_3_full_parallel(
            &mut second,
            &plan,
            &mut workspace,
            FullSynthesisBackend::ParallelStriped,
            4,
            false,
        )
        .unwrap();
        assert_eq!(workspace.capacities(), capacities);
        assert!(capacities.stripe <= DEFAULT_STRIPE_WIDTH * 4096);
        assert_eq!(capacities.transpose, 0);

        let mut transpose_workspace = FullSynthesisWorkspace::default();
        let mut first = vec![0_i32; 4096 * 4096];
        inverse_reversible_5_3_full_parallel(
            &mut first,
            &plan,
            &mut transpose_workspace,
            FullSynthesisBackend::ParallelTranspose,
            4,
            false,
        )
        .unwrap();
        let transpose_capacities = transpose_workspace.capacities();
        let mut second = vec![0_i32; 4096 * 4096];
        inverse_reversible_5_3_full_parallel(
            &mut second,
            &plan,
            &mut transpose_workspace,
            FullSynthesisBackend::ParallelTranspose,
            4,
            false,
        )
        .unwrap();
        assert_eq!(transpose_workspace.capacities(), transpose_capacities);
        assert_eq!(transpose_capacities.stripe, 0);
        assert_eq!(transpose_capacities.transpose, 4096 * 4096);

        // A tighter later policy must not keep the inactive full transpose
        // plane alive behind a nominally striped/legacy memory estimate.
        let mut switched = vec![0_i32; 4096 * 4096];
        inverse_reversible_5_3_full_parallel(
            &mut switched,
            &plan,
            &mut transpose_workspace,
            FullSynthesisBackend::ParallelStriped,
            4,
            false,
        )
        .unwrap();
        let switched_capacities = transpose_workspace.capacities();
        assert_eq!(switched_capacities.transpose, 0);
        assert!(switched_capacities.stripe <= DEFAULT_STRIPE_WIDTH * 4096);
        let mut legacy = vec![0_i32; 4096 * 4096];
        inverse_reversible_5_3_full_parallel(
            &mut legacy,
            &plan,
            &mut transpose_workspace,
            FullSynthesisBackend::LegacyScalar,
            1,
            false,
        )
        .unwrap();
        let legacy_capacities = transpose_workspace.capacities();
        assert_eq!(legacy_capacities.transpose, 0);
        assert_eq!(legacy_capacities.stripe, 0);
    }
}
