//! Planned bounded forward 5/3 analysis for the classic two-level writer.
//!
//! Panels retain interleaved rows and adjacent columns. No column transform or
//! extension occurs at a panel boundary. See `docs/forward53-parallel-dispatch.md` in the
//! repository for the arithmetic and live-allocation proof.
use alloc::vec::Vec;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::{
    Reversible53Config, TransformBand, TransformError, forward_reversible_5_3_bounded,
    transform_line_forward_bounded, validate_axis_edges,
};

/// Implementation selected before any coefficient mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Forward53Backend {
    Reference,
    RowPanelScalar,
    RowPanelParallel,
}

/// A narrow, validated full-resolution-to-LL plan (at most two levels).
/// Geometry and capacities are immutable after construction.
#[derive(Debug, Clone)]
pub struct Forward53Plan {
    levels: [Option<Reversible53Config>; 2],
    extent: usize,
    axis: usize,
    panel_width: usize,
    panel_elements: usize,
    slot_elements: usize,
    slots: usize,
    elements: usize,
    backend: Forward53Backend,
}

impl Forward53Plan {
    /// Bind explicit levels, original stride, panel width and logical slots.
    /// High-first phases and small levels use the unchanged scalar reference.
    pub fn new(
        levels: &[Reversible53Config],
        backend: Forward53Backend,
        panel_width: usize,
        slots: usize,
    ) -> Result<Self, TransformError> {
        if levels.is_empty()
            || levels.len() > 2
            || panel_width == 0
            || panel_width > 32
            || slots == 0
            || slots > 8
        {
            return Err(TransformError::InvalidWindow);
        }
        let first = levels[0];
        let mut previous: Option<Reversible53Config> = None;
        for &level in levels {
            if level.width == 0 || level.height == 0 {
                return Err(TransformError::EmptyPlane);
            }
            if level.stride < level.width {
                return Err(TransformError::StrideTooSmall);
            }
            if let Some(prior) = previous
                && (level.width != prior.edges.horizontal_low_samples
                    || level.height != prior.edges.vertical_low_samples
                    || level.stride != first.stride)
            {
                return Err(TransformError::InvalidWindow);
            }
            validate_axis_edges(
                level.width,
                level.edges.horizontal_low_samples,
                level.edges.horizontal_first,
            )?;
            validate_axis_edges(
                level.height,
                level.edges.vertical_low_samples,
                level.edges.vertical_first,
            )?;
            // Validate sample precision even when execution takes the panel path.
            level.sample_range.bounds()?;
            if previous.is_some() && level.sample_range != super::ComponentSampleRange::signed(32) {
                return Err(TransformError::InvalidSampleRange);
            }
            previous = Some(level);
        }
        let extent = (first.height - 1)
            .checked_mul(first.stride)
            .and_then(|n| n.checked_add(first.width))
            .ok_or(TransformError::SizeOverflow)?;
        let axis = first.width.max(first.height);
        let slots = if backend == Forward53Backend::RowPanelParallel && cfg!(feature = "parallel") {
            slots
                .min(first.width.div_ceil(panel_width))
                .min(first.height)
        } else {
            1
        };
        let backend = if backend == Forward53Backend::RowPanelParallel && slots == 1 {
            Forward53Backend::Reference
        } else {
            backend
        };
        let backend = if !levels.iter().any(|level| {
            level.edges.horizontal_first == TransformBand::Low
                && level.edges.vertical_first == TransformBand::Low
                && level.width.saturating_mul(level.height) >= 4096
                && (backend == Forward53Backend::RowPanelScalar
                    || (level.width.div_ceil(panel_width) >= 2 && level.height >= 2))
        }) {
            Forward53Backend::Reference
        } else {
            backend
        };
        let panel_elements = first
            .height
            .checked_mul(panel_width)
            .ok_or(TransformError::SizeOverflow)?;
        let slot_elements = panel_elements
            .checked_add(axis)
            .ok_or(TransformError::SizeOverflow)?;
        let reference_elements = axis.checked_mul(3).ok_or(TransformError::SizeOverflow)?;
        let elements = if backend == Forward53Backend::Reference {
            reference_elements
        } else {
            slot_elements
                .checked_mul(slots)
                .ok_or(TransformError::SizeOverflow)?
                .max(reference_elements)
        };
        elements
            .checked_mul(core::mem::size_of::<i32>())
            .filter(|&n| n <= isize::MAX as usize)
            .ok_or(TransformError::SizeOverflow)?;
        let mut bound = [None; 2];
        for (destination, &level) in bound.iter_mut().zip(levels) {
            *destination = Some(level);
        }
        Ok(Self {
            levels: bound,
            extent,
            axis,
            panel_width,
            panel_elements,
            slot_elements,
            slots,
            elements,
            backend,
        })
    }

    pub fn backend(&self) -> Forward53Backend {
        self.backend
    }
    pub fn panel_width(&self) -> usize {
        self.panel_width
    }
    pub fn slots(&self) -> usize {
        self.slots
    }
    pub fn workspace_bytes(&self) -> usize {
        self.elements * core::mem::size_of::<i32>()
    }

    fn uses_panels(&self, level: Reversible53Config) -> bool {
        self.backend != Forward53Backend::Reference
            && level.edges.horizontal_first == TransformBand::Low
            && level.edges.vertical_first == TransformBand::Low
            && level.width.saturating_mul(level.height) >= 4096
            && (self.backend == Forward53Backend::RowPanelScalar
                || (level.width.div_ceil(self.panel_width) >= 2 && level.height >= 2))
    }
}

/// One flat allocation, explicitly divided into bounded logical slots.
#[derive(Debug, Default)]
pub struct Forward53Workspace {
    samples: Vec<i32>,
}

impl Forward53Workspace {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn capacity_bytes(&self) -> usize {
        self.samples.capacity() * core::mem::size_of::<i32>()
    }

    /// Prepare before mutation. Retained storage is released before growth, so
    /// old and new allocations never overlap. An allocation/capacity failure
    /// leaves an empty workspace and is safe to retry with a smaller plan.
    pub fn prepare(
        &mut self,
        plan: &Forward53Plan,
        maximum_bytes: usize,
    ) -> Result<(), TransformError> {
        if plan.workspace_bytes() > maximum_bytes {
            return Err(TransformError::ScratchTooSmall {
                required: plan.workspace_bytes(),
                actual: maximum_bytes,
            });
        }
        if self.samples.capacity() < plan.elements || self.capacity_bytes() > maximum_bytes {
            self.samples = Vec::new();
            let mut samples = Vec::new();
            samples
                .try_reserve_exact(plan.elements)
                .map_err(|_| TransformError::SizeOverflow)?;
            if samples.capacity() != plan.elements {
                return Err(TransformError::SizeOverflow);
            }
            self.samples = samples;
        }
        self.samples.resize(plan.elements, 0);
        Ok(())
    }
}

/// Execute a prepared bounded plan, reusing storage across components.
///
/// As with `forward_reversible_5_3_bounded`, the caller must prove arithmetic
/// bounds. Classic unsigned U8/U16 (optionally RCT), at most two levels, meets
/// this obligation; this is not an arbitrary-i32 transform. Geometry and every
/// configured input range are validated before any plane mutation.
pub fn forward_reversible_5_3_planned_bounded(
    plane: &mut [i32],
    plan: &Forward53Plan,
    workspace: &mut Forward53Workspace,
) -> Result<(), TransformError> {
    if plane.len() < plan.extent {
        return Err(TransformError::PlaneTooSmall);
    }
    if workspace.samples.len() < plan.elements {
        return Err(TransformError::ScratchTooSmall {
            required: plan.elements,
            actual: workspace.samples.len(),
        });
    }
    // The first level describes original samples. Later levels describe LL
    // intermediates and must use signed(32), just as the existing writer does.
    for config in plan.levels.iter().flatten() {
        if config.sample_range != super::ComponentSampleRange::signed(32) {
            super::validate_samples_in_range(plane, *config)?;
        }
    }
    #[cfg(feature = "classic-execution-diagnostics")]
    super::forward_diagnostics::update(|d| {
        d.level_backends = plan.levels.map(|level| {
            level.map(|c| {
                if plan.uses_panels(c) {
                    plan.backend
                } else {
                    Forward53Backend::Reference
                }
            })
        });
        d.panel_width = plan.panel_width;
        d.logical_slots = plan.slots;
        d.workspace_capacity_bytes = workspace.capacity_bytes();
    });
    for config in plan.levels.iter().flatten().copied() {
        if plan.uses_panels(config) {
            vertical(plane, config, plan, &mut workspace.samples);
            horizontal(plane, config, plan, &mut workspace.samples);
        } else {
            #[cfg(feature = "classic-execution-diagnostics")]
            let jobs = super::forward_diagnostics::Jobs::new();
            #[cfg(feature = "classic-execution-diagnostics")]
            let job = jobs.enter();
            forward_reversible_5_3_bounded(plane, config, &mut workspace.samples[..3 * plan.axis])?;
            #[cfg(feature = "classic-execution-diagnostics")]
            {
                drop(job);
                jobs.finish();
            }
        }
    }
    Ok(())
}

fn gather_lift(
    source: &[i32],
    config: Reversible53Config,
    x: usize,
    lanes: usize,
    panel_stride: usize,
    panel: &mut [i32],
) {
    for y in 0..config.height {
        panel[y * panel_stride..y * panel_stride + lanes]
            .copy_from_slice(&source[y * config.stride + x..y * config.stride + x + lanes]);
    }
    lift_panel(panel, config.height, lanes, panel_stride);
}

fn lift_panel(panel: &mut [i32], height: usize, lanes: usize, stride: usize) {
    if height == 1 {
        return;
    }
    // Predict uses only original even rows. Row-major lanes are independent.
    for y in (1..height).step_by(2) {
        let above = (y - 1) * stride;
        let below = if y + 1 < height {
            (y + 1) * stride
        } else {
            above
        };
        for lane in 0..lanes {
            panel[y * stride + lane] -= (panel[above + lane] + panel[below + lane]) >> 1;
        }
    }
    // All odd rows are complete before any low row is updated.
    for y in (0..height).step_by(2) {
        let above = if y == 0 { stride } else { (y - 1) * stride };
        let below = if y + 1 < height {
            (y + 1) * stride
        } else {
            above
        };
        for lane in 0..lanes {
            panel[y * stride + lane] += (panel[above + lane] + panel[below + lane] + 2) >> 2;
        }
    }
}

fn vertical(
    plane: &mut [i32],
    config: Reversible53Config,
    plan: &Forward53Plan,
    samples: &mut [i32],
) {
    vertical_with_hook(plane, config, plan, samples, |_| {});
}

// The no-op monomorphisation has no hook state or dispatch in ordinary builds.
// Tests inject a deterministic panic while another job remains in flight.
fn vertical_with_hook(
    plane: &mut [i32],
    config: Reversible53Config,
    plan: &Forward53Plan,
    samples: &mut [i32],
    after_gather: impl Fn(usize) + Sync,
) {
    let extent = (config.height - 1) * config.stride + config.width;
    for start_x in (0..config.width).step_by(plan.slots * plan.panel_width) {
        let count = (config.width - start_x)
            .div_ceil(plan.panel_width)
            .min(plan.slots);
        let slots = &mut samples[..count * plan.slot_elements];
        #[cfg(feature = "classic-execution-diagnostics")]
        let phase = super::forward_diagnostics::Clock::start();
        #[cfg(feature = "classic-execution-diagnostics")]
        let jobs = super::forward_diagnostics::Jobs::new();
        let gather = |(slot, storage): (usize, &mut [i32])| {
            #[cfg(feature = "classic-execution-diagnostics")]
            let _job = jobs.enter();
            let x = start_x + slot * plan.panel_width;
            gather_lift(
                plane,
                config,
                x,
                (config.width - x).min(plan.panel_width),
                plan.panel_width,
                &mut storage[..plan.panel_elements],
            );
            after_gather(slot);
        };
        #[cfg(feature = "parallel")]
        if count > 1 {
            slots
                .par_chunks_mut(plan.slot_elements)
                .enumerate()
                .for_each(gather);
        } else {
            slots
                .chunks_mut(plan.slot_elements)
                .enumerate()
                .for_each(gather);
        }
        #[cfg(not(feature = "parallel"))]
        slots
            .chunks_mut(plan.slot_elements)
            .enumerate()
            .for_each(gather);
        #[cfg(feature = "classic-execution-diagnostics")]
        {
            phase.finish(|d| &mut d.gather_lift_join_ns);
            jobs.finish();
        }
        // All immutable plane borrows end above. Parallel scatter owns disjoint
        // destination row groups, including a potentially short final row.
        let panels: &[i32] = slots;
        let rows_per_group = config.height.div_ceil(plan.slots);
        let group_elements = rows_per_group.checked_mul(config.stride).unwrap_or(extent);
        #[cfg(feature = "classic-execution-diagnostics")]
        let phase = super::forward_diagnostics::Clock::start();
        #[cfg(feature = "classic-execution-diagnostics")]
        let jobs = super::forward_diagnostics::Jobs::new();
        let scatter = |(group, rows): (usize, &mut [i32])| {
            #[cfg(feature = "classic-execution-diagnostics")]
            let _job = jobs.enter();
            for (local_y, row) in rows.chunks_mut(config.stride).enumerate() {
                let y = group * rows_per_group + local_y;
                let source_y = if y < config.edges.vertical_low_samples {
                    2 * y
                } else {
                    2 * (y - config.edges.vertical_low_samples) + 1
                };
                for slot in 0..count {
                    let x = start_x + slot * plan.panel_width;
                    let lanes = (config.width - x).min(plan.panel_width);
                    let offset = slot * plan.slot_elements + source_y * plan.panel_width;
                    row[x..x + lanes].copy_from_slice(&panels[offset..offset + lanes]);
                }
            }
        };
        #[cfg(feature = "parallel")]
        if plan.slots > 1 {
            plane[..extent]
                .par_chunks_mut(group_elements)
                .enumerate()
                .for_each(scatter);
        } else {
            plane[..extent]
                .chunks_mut(group_elements)
                .enumerate()
                .for_each(scatter);
        }
        #[cfg(not(feature = "parallel"))]
        plane[..extent]
            .chunks_mut(group_elements)
            .enumerate()
            .for_each(scatter);
        #[cfg(feature = "classic-execution-diagnostics")]
        {
            phase.finish(|d| &mut d.scatter_join_ns);
            jobs.finish();
        }
    }
}

fn horizontal(
    plane: &mut [i32],
    config: Reversible53Config,
    plan: &Forward53Plan,
    samples: &mut [i32],
) {
    let extent = (config.height - 1) * config.stride + config.width;
    let rows_per_group = config.height.div_ceil(plan.slots);
    let group_elements = rows_per_group.checked_mul(config.stride).unwrap_or(extent);
    let slots = &mut samples[..plan.slots * plan.slot_elements];
    #[cfg(feature = "classic-execution-diagnostics")]
    let phase = super::forward_diagnostics::Clock::start();
    #[cfg(feature = "classic-execution-diagnostics")]
    let jobs = super::forward_diagnostics::Jobs::new();
    let lift = |(rows, storage): (&mut [i32], &mut [i32])| {
        #[cfg(feature = "classic-execution-diagnostics")]
        let _job = jobs.enter();
        let line = &mut storage[plan.panel_elements..plan.panel_elements + plan.axis];
        for row in rows.chunks_mut(config.stride) {
            transform_line_forward_bounded(
                &mut row[..config.width],
                config.edges.horizontal_low_samples,
                config.edges.horizontal_first,
                line,
            );
        }
    };
    #[cfg(feature = "parallel")]
    if plan.slots > 1 {
        plane[..extent]
            .par_chunks_mut(group_elements)
            .zip(slots.par_chunks_mut(plan.slot_elements))
            .for_each(lift);
    } else {
        plane[..extent]
            .chunks_mut(group_elements)
            .zip(slots.chunks_mut(plan.slot_elements))
            .for_each(lift);
    }
    #[cfg(not(feature = "parallel"))]
    plane[..extent]
        .chunks_mut(group_elements)
        .zip(slots.chunks_mut(plan.slot_elements))
        .for_each(lift);
    #[cfg(feature = "classic-execution-diagnostics")]
    {
        phase.finish(|d| &mut d.horizontal_join_ns);
        jobs.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComponentSampleRange, Reversible53Edges};
    use alloc::vec;

    fn config(
        width: usize,
        height: usize,
        stride: usize,
        x: usize,
        y: usize,
    ) -> Reversible53Config {
        Reversible53Config {
            width,
            height,
            stride,
            edges: Reversible53Edges::from_tile_origin(x, y, width, height),
            sample_range: ComponentSampleRange::signed(32),
        }
    }

    // Independently checked original line arithmetic; neither gather/scatter
    // indexing nor panel lifting is shared with the implementation under test.
    fn reference_axis(plane: &mut [i32], c: Reversible53Config, vertical_axis: bool) {
        let mut line = vec![0; c.width.max(c.height)];
        let mut read = line.clone();
        let mut coefficients = line.clone();
        if vertical_axis {
            for x in 0..c.width {
                for y in 0..c.height {
                    line[y] = plane[y * c.stride + x];
                }
                crate::transform_line_forward(
                    &mut line[..c.height],
                    c.edges.vertical_low_samples,
                    c.edges.vertical_first,
                    &mut read,
                    &mut coefficients,
                )
                .unwrap();
                for y in 0..c.height {
                    plane[y * c.stride + x] = line[y];
                }
            }
        } else {
            for y in 0..c.height {
                crate::transform_line_forward(
                    &mut plane[y * c.stride..y * c.stride + c.width],
                    c.edges.horizontal_low_samples,
                    c.edges.horizontal_first,
                    &mut read,
                    &mut coefficients,
                )
                .unwrap();
            }
        }
    }

    fn phase_cases() {
        for (width, height) in [
            (1, 1),
            (1, 19),
            (18, 1),
            (2, 2),
            (7, 9),
            (32, 33),
            (33, 32),
            (65, 67),
            (131, 129),
        ] {
            let first = config(width, height, width + 7, 0, 0);
            let second = config(width.div_ceil(2), height.div_ceil(2), first.stride, 0, 0);
            for panel_width in [8, 16, 32] {
                for slots in [1, 2, 4, 8] {
                    for pattern in 0..3 {
                        // Exact extent deliberately omits the final row's padding.
                        let mut source = vec![0x1234567; (height - 1) * first.stride + width];
                        for y in 0..height {
                            for x in 0..width {
                                source[y * first.stride + x] = match pattern {
                                    0 => {
                                        if (x + y) % 2 == 0 {
                                            -65535
                                        } else {
                                            65535
                                        }
                                    }
                                    1 => ((x * 7919 + y * 3571) % 131071) as i32 - 65535,
                                    _ => {
                                        if x == width / 2 && y == height / 2 {
                                            -65535
                                        } else {
                                            0
                                        }
                                    }
                                };
                            }
                        }
                        let mut plan = Forward53Plan::new(
                            &[first, second],
                            Forward53Backend::RowPanelParallel,
                            panel_width,
                            slots,
                        )
                        .unwrap();
                        // Force mechanical storage only for the independent axis
                        // oracle: policy assertions use unmodified plans below.
                        plan.elements = (plan.slot_elements * plan.slots).max(3 * plan.axis);
                        let mut workspace = Forward53Workspace::new();
                        workspace.prepare(&plan, plan.workspace_bytes()).unwrap();
                        let capacity = workspace.capacity_bytes();
                        let address = workspace.samples.as_ptr();
                        let mut candidate = source.clone();
                        let mut expected = source.clone();
                        for level in [first, second] {
                            vertical(&mut candidate, level, &plan, &mut workspace.samples);
                            reference_axis(&mut expected, level, true);
                            assert_eq!(
                                candidate, expected,
                                "vertical {width}x{height}, b{panel_width}, q{slots}"
                            );
                            horizontal(&mut candidate, level, &plan, &mut workspace.samples);
                            reference_axis(&mut expected, level, false);
                            assert_eq!(
                                candidate, expected,
                                "horizontal {width}x{height}, b{panel_width}, q{slots}"
                            );
                        }
                        // Reuse across components resets no capacity or pointer.
                        workspace.prepare(&plan, capacity).unwrap();
                        assert_eq!(address, workspace.samples.as_ptr());
                        assert_eq!(capacity, workspace.capacity_bytes());
                        let mut reused = source;
                        forward_reversible_5_3_planned_bounded(&mut reused, &plan, &mut workspace)
                            .unwrap();
                        assert_eq!(reused, expected);
                        assert_eq!(address, workspace.samples.as_ptr());
                    }
                }
            }
        }
    }

    #[test]
    fn phases_levels_padding_partial_panels_and_reuse_match_checked_reference() {
        #[cfg(feature = "parallel")]
        for workers in [1, 2, 4, 8] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(phase_cases);
        }
        #[cfg(not(feature = "parallel"))]
        phase_cases();
    }

    #[test]
    fn high_phases_and_small_work_use_unchanged_reference() {
        for (width, height) in [(1, 1), (1, 17), (18, 1), (65, 67)] {
            for (x, y) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                let config = config(width, height, width + 3, x, y);
                let plan = Forward53Plan::new(&[config], Forward53Backend::RowPanelParallel, 16, 8)
                    .unwrap();
                assert_eq!(
                    plan.uses_panels(config),
                    cfg!(feature = "parallel") && width * height >= 4096 && x == 0 && y == 0
                );
                if !plan.uses_panels(config) {
                    assert_eq!(plan.backend(), Forward53Backend::Reference);
                    assert_eq!(plan.workspace_bytes(), 12 * width.max(height));
                }
                let source: Vec<i32> = (0..(height - 1) * config.stride + width)
                    .map(|n| (n % 131071) as i32 - 65535)
                    .collect();
                let mut reference = source.clone();
                let mut candidate = source;
                crate::forward_reversible_5_3(
                    &mut reference,
                    config,
                    &mut vec![0; config.scratch_len()],
                )
                .unwrap();
                let mut workspace = Forward53Workspace::new();
                workspace.prepare(&plan, plan.workspace_bytes()).unwrap();
                forward_reversible_5_3_planned_bounded(&mut candidate, &plan, &mut workspace)
                    .unwrap();
                assert_eq!(candidate, reference);
            }
        }
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn mixed_level_keeps_storage_but_rejects_one_slot_ll() {
        let first = config(17, 1024, 20, 0, 0);
        let second = config(9, 512, 20, 0, 0);
        let plan = Forward53Plan::new(&[first, second], Forward53Backend::RowPanelParallel, 16, 8)
            .unwrap();
        assert_eq!(plan.slots(), 2);
        assert!(plan.uses_panels(first));
        assert!(!plan.uses_panels(second));
        let mut workspace = Forward53Workspace::new();
        workspace.prepare(&plan, plan.workspace_bytes()).unwrap();
        let address = workspace.samples.as_ptr();
        let mut plane: Vec<i32> = (0..(first.height - 1) * first.stride + first.width)
            .map(|n| ((n * 71) % 131071) as i32 - 65535)
            .collect();
        let mut expected = plane.clone();
        for level in [first, second] {
            reference_axis(&mut expected, level, true);
            reference_axis(&mut expected, level, false);
        }
        forward_reversible_5_3_planned_bounded(&mut plane, &plan, &mut workspace).unwrap();
        assert_eq!(plane, expected);
        assert_eq!(workspace.samples.as_ptr(), address);
        assert_eq!(workspace.capacity_bytes(), plan.workspace_bytes());
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn panic_joins_every_gather_before_return_and_workspace_can_be_reused() {
        use std::sync::{
            Barrier,
            atomic::{AtomicUsize, Ordering},
        };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let c = config(65, 67, 70, 0, 0);
        let p = Forward53Plan::new(&[c], Forward53Backend::RowPanelParallel, 16, 2).unwrap();
        let mut w = Forward53Workspace::new();
        w.prepare(&p, p.workspace_bytes()).unwrap();
        let address = w.samples.as_ptr();
        let mut plane = vec![17; (c.height - 1) * c.stride + c.width];
        let original = plane.clone();
        let barrier = Barrier::new(2);
        let completed = AtomicUsize::new(0);
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pool.install(|| {
                vertical_with_hook(&mut plane, c, &p, &mut w.samples, |slot| {
                    barrier.wait();
                    if slot == 0 {
                        panic!("authored gather fault");
                    }
                    completed.fetch_add(1, Ordering::SeqCst);
                });
            })
        }));
        assert!(failed.is_err());
        assert_eq!(completed.load(Ordering::SeqCst), 1);
        assert_eq!(plane, original); // No scatter can run before a successful join.
        assert_eq!(w.samples.as_ptr(), address);
        pool.install(|| forward_reversible_5_3_planned_bounded(&mut plane, &p, &mut w))
            .unwrap();
        let mut expected = original;
        crate::forward_reversible_5_3(&mut expected, c, &mut vec![0; c.scratch_len()]).unwrap();
        assert_eq!(plane, expected);
    }

    #[test]
    fn malformed_plan_and_workspace_fail_before_mutation() {
        let c = config(65, 67, 70, 0, 0);
        assert!(Forward53Plan::new(&[], Forward53Backend::Reference, 16, 1).is_err());
        for bad in [
            Reversible53Config { width: 0, ..c },
            Reversible53Config { stride: 64, ..c },
            Reversible53Config {
                sample_range: ComponentSampleRange::signed(0),
                ..c
            },
            Reversible53Config {
                edges: Reversible53Edges {
                    vertical_low_samples: 1,
                    ..c.edges
                },
                ..c
            },
        ] {
            assert!(Forward53Plan::new(&[bad], Forward53Backend::RowPanelScalar, 16, 1).is_err());
        }
        assert!(Forward53Plan::new(&[c, c], Forward53Backend::Reference, 16, 1).is_err());
        let p = Forward53Plan::new(&[c], Forward53Backend::RowPanelParallel, 16, 8).unwrap();
        let mut w = Forward53Workspace::new();
        assert!(w.prepare(&p, p.workspace_bytes() - 1).is_err());
        assert_eq!(w.capacity_bytes(), 0);
        let mut plane = vec![123; (c.height - 1) * c.stride + c.width];
        let source = plane.clone();
        assert!(forward_reversible_5_3_planned_bounded(&mut plane, &p, &mut w).is_err());
        assert_eq!(plane, source);
        w.prepare(&p, p.workspace_bytes()).unwrap();
        assert!(
            forward_reversible_5_3_planned_bounded(&mut plane[..source.len() - 1], &p, &mut w)
                .is_err()
        );
        assert_eq!(plane, source);
        // Growth releases the earlier allocation before requesting a replacement.
        let larger = Forward53Plan::new(
            &[config(129, 131, 129, 0, 0)],
            Forward53Backend::RowPanelParallel,
            32,
            8,
        )
        .unwrap();
        w.prepare(&larger, larger.workspace_bytes()).unwrap();
        assert_eq!(w.capacity_bytes(), larger.workspace_bytes());
        let scalar = Forward53Plan::new(&[c], Forward53Backend::Reference, 1, 1).unwrap();
        w.prepare(&scalar, scalar.workspace_bytes()).unwrap();
        assert_eq!(w.capacity_bytes(), scalar.workspace_bytes());
    }

    #[cfg(feature = "classic-execution-diagnostics")]
    #[test]
    fn diagnostic_wall_intervals_and_actual_concurrency_are_separate() {
        let run = || {
            let c = config(257, 259, 260, 0, 0);
            let p = Forward53Plan::new(&[c], Forward53Backend::RowPanelParallel, 16, 4).unwrap();
            let mut w = Forward53Workspace::new();
            w.prepare(&p, p.workspace_bytes()).unwrap();
            let mut plane = vec![0; (c.height - 1) * c.stride + c.width];
            let start = std::time::Instant::now();
            let (result, d) = crate::observe_forward_transform(|| {
                forward_reversible_5_3_planned_bounded(&mut plane, &p, &mut w)
            });
            result.unwrap();
            assert!(
                d.gather_lift_join_ns + d.scatter_join_ns + d.horizontal_join_ns
                    <= start.elapsed().as_nanos()
            );
            assert_eq!(d.workspace_capacity_bytes, p.workspace_bytes());
            assert!((1..=p.slots).contains(&d.peak_active_slots));
            assert!(d.participating_workers.unwrap() >= d.peak_active_slots);
            assert_eq!(d.level_backends[0], Some(p.backend));
        };
        #[cfg(feature = "parallel")]
        rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
            .install(run);
        #[cfg(not(feature = "parallel"))]
        run();
    }
}
