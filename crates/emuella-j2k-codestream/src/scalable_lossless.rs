//! Resource admission for the project-authored single-tile classic D2 writer.
use super::*;

#[cfg(feature = "parallel")]
mod parallel;

mod bypass;
#[cfg(feature = "classic-execution-diagnostics")]
pub(super) mod diagnostics;
#[cfg(feature = "test-fixtures")]
pub use bypass::encode_lossless_d2_bypass_test_fixture;

const WORKER_BYTES: u64 = 4 * 1024 * 1024;

// Source-bound development policy. Promotion requires independent confirmation.
const FORWARD53_BACKEND: transform::Forward53Backend =
    transform::Forward53Backend::RowPanelParallel;
const FORWARD53_PANEL_WIDTH: usize = 16;

type PreparedForward53 = (transform::Forward53Plan, transform::Forward53Workspace);

fn prepare_forward53(
    width: usize,
    height: usize,
    workers: usize,
) -> Result<Option<PreparedForward53>> {
    let backend = FORWARD53_BACKEND;
    let panel_width = FORWARD53_PANEL_WIDTH;
    #[cfg(feature = "classic-execution-diagnostics")]
    let (backend, panel_width) = transform::forward53_diagnostic_policy(backend, panel_width);
    #[cfg(feature = "classic-execution-diagnostics")]
    if !(1..=32).contains(&panel_width) {
        return Err(CodestreamError::SizeOverflow);
    }
    // Admission already reflects the existing pool and Tier-1 memory budget.
    // Do not construct a plan or initialise panel storage for serial calls.
    if backend == transform::Forward53Backend::Reference
        || width.saturating_mul(height) < 4096
        || (backend == transform::Forward53Backend::RowPanelParallel
            && (workers < 2 || width.div_ceil(panel_width) < 2 || height < 2))
    {
        return Ok(None);
    }
    // Only the DWT and Tier-1 local terms are reused. Descriptors, output and
    // coefficient allowances remain untouched; 4 KiB preserves fixed metadata.
    let maximum = workers
        .min(8)
        .checked_mul(WORKER_BYTES as usize)
        .and_then(|n| n.checked_add(width.max(height).checked_mul(12)?))
        .and_then(|n| n.checked_sub(4096))
        .ok_or(CodestreamError::SizeOverflow)?;
    prepare_forward53_with_maximum(
        width,
        height,
        workers,
        maximum,
        backend,
        panel_width,
        |workspace, plan, maximum| {
            #[cfg(feature = "classic-execution-diagnostics")]
            diagnostics::update(|d| d.forward53_panel_prepare_attempts += 1);
            let result = workspace.prepare(plan, maximum);
            #[cfg(feature = "classic-execution-diagnostics")]
            if result.is_ok() {
                diagnostics::update(|d| {
                    d.forward53_panel_initialised_bytes += plan.workspace_bytes()
                });
            }
            result
        },
    )
}

fn prepare_forward53_with_maximum(
    width: usize,
    height: usize,
    workers: usize,
    maximum: usize,
    backend: transform::Forward53Backend,
    panel_width: usize,
    mut prepare: impl FnMut(
        &mut transform::Forward53Workspace,
        &transform::Forward53Plan,
        usize,
    ) -> core::result::Result<(), transform::TransformError>,
) -> Result<Option<PreparedForward53>> {
    let config = |w, h| transform::Reversible53Config {
        width: w,
        height: h,
        stride: width,
        edges: transform::Reversible53Edges::from_tile_origin(0, 0, w, h),
        sample_range: transform::ComponentSampleRange::signed(32),
    };
    let levels = [
        config(width, height),
        config(width.div_ceil(2), height.div_ceil(2)),
    ];
    let minimum_slots = if backend == transform::Forward53Backend::RowPanelScalar {
        1
    } else {
        2
    };
    let mut slots = workers.min(8);
    while slots >= minimum_slots {
        let plan = transform::Forward53Plan::new(&levels, backend, panel_width, slots)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if plan.backend() == transform::Forward53Backend::Reference {
            return Ok(None);
        }
        // A capacity-ineligible plan never requests or initialises storage.
        if plan.workspace_bytes() <= maximum {
            let mut workspace = transform::Forward53Workspace::new();
            if prepare(&mut workspace, &plan, maximum).is_ok() {
                return Ok(Some((plan, workspace)));
            }
            // Drop every failed optional reservation before retry or fallback.
        }
        if slots == minimum_slots {
            break;
        }
        slots = slots.div_ceil(2);
    }
    Ok(None)
}

/// Byte limits for owned raw classic lossless D2 encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LosslessEncodeLimits {
    /// Additional requested live allocation allowance, excluding caller input
    /// and the retained output allocation. Includes conservative realloc overlap.
    pub max_working_bytes: u64,
    /// Maximum capacity of the returned codestream allocation.
    pub max_output_bytes: u64,
}

impl Default for LosslessEncodeLimits {
    fn default() -> Self {
        Self {
            max_working_bytes: 4 * 1024 * 1024 * 1024,
            max_output_bytes: 1024 * 1024 * 1024,
        }
    }
}

/// Checked conservative admission accounting, not measured RSS or a reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LosslessEncodeRequirements {
    /// Total samples across all components (width × height × components).
    pub total_component_samples: u64,
    pub code_blocks: u64,
    pub working_bytes: u64,
    pub output_capacity_limit: u64,
}

/// Borrowed native component with an explicit byte step between samples.
#[derive(Clone, Copy)]
pub struct LosslessD2Plane<'a> {
    pub samples: &'a [u8],
    pub stride_bytes: usize,
    pub sample_step_bytes: usize,
}

/// Geometry-only admission for raw D2 grey/RGB or eight native components.
/// Eight-component encoding requires U16 at the sample-bearing entry point.
pub fn lossless_d2_requirements(
    width: u32,
    height: u32,
    components: u16,
    limits: LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    execution_requirements::<false>(width, height, components, limits)
        .map(|(requirements, _)| requirements)
}

/// Geometry-only admission for the opt-in D2 selective-bypass writer.
/// Includes bounded segment metadata before choosing the worker allowance.
pub fn lossless_d2_bypass_requirements(
    width: u32,
    height: u32,
    components: u16,
    limits: LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    execution_requirements::<true>(width, height, components, limits)
        .map(|(requirements, _)| requirements)
}

fn execution_requirements<const BYPASS: bool>(
    width: u32,
    height: u32,
    components: u16,
    limits: LosslessEncodeLimits,
) -> Result<(LosslessEncodeRequirements, usize)> {
    let pixels = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(CodestreamError::SizeOverflow)?;
    if width < 4
        || height < 4
        || width > 32768
        || height > 32768
        || pixels > 64 * 1024 * 1024
        || !matches!(components, 1 | 3 | 8)
        || (components == 8 && pixels > 32 * 1024 * 1024)
    {
        return Err(resource_error(
            "lossless D2 requires 4..=32768 axes, at most 64 Mi pixels for grey/RGB or 32 Mi pixels for eight components",
        ));
    }
    if limits.max_output_bytes < 128 {
        return Err(resource_error(
            "lossless D2 output-capacity budget must allow at least 128 bytes",
        ));
    }
    if limits.max_output_bytes > u64::from(u32::MAX) || limits.max_output_bytes > isize::MAX as u64
    {
        return Err(CodestreamError::SizeOverflow);
    }
    let blocks = decomp_subband_specs(width, height, 2)?
        .iter()
        .try_fold(0_u64, |sum, band| {
            sum.checked_add(
                u64::from(band.width)
                    .div_ceil(64)
                    .checked_mul(u64::from(band.height).div_ceil(64))
                    .ok_or(CodestreamError::SizeOverflow)?,
            )
            .ok_or(CodestreamError::SizeOverflow)
        })?
        .checked_mul(u64::from(components))
        .ok_or(CodestreamError::SizeOverflow)?;
    let samples = pixels
        .checked_mul(u64::from(components))
        .ok_or(CodestreamError::SizeOverflow)?;
    // See docs/scalable-lossless.md: planes, descriptors/trees/header growth,
    // DWT lines, output reallocation overlap, and bounded Tier-1/local storage.
    let working = samples
        .checked_mul(4)
        .and_then(|n| n.checked_add(blocks.checked_mul(4096)?))
        .and_then(|n| n.checked_add(u64::from(width.max(height)).checked_mul(12)?))
        .and_then(|n| n.checked_add(limits.max_output_bytes.checked_mul(2)?))
        .ok_or(CodestreamError::SizeOverflow)?;
    let working = if BYPASS {
        working
            .checked_add(
                blocks
                    .checked_mul(bypass::METADATA_BYTES_PER_BLOCK)
                    .ok_or(CodestreamError::SizeOverflow)?,
            )
            .ok_or(CodestreamError::SizeOverflow)?
    } else {
        working
    };
    let affordable = limits.max_working_bytes.saturating_sub(working) / WORKER_BYTES;
    if affordable == 0 {
        return Err(resource_error(
            "lossless D2 working-memory budget is insufficient",
        ));
    }
    let workers = parallel_worker_count()
        .unwrap_or(1)
        .min(usize::try_from(affordable.min(blocks)).map_err(|_| CodestreamError::SizeOverflow)?);
    let working = working
        .checked_add(
            (workers as u64)
                .checked_mul(WORKER_BYTES)
                .ok_or(CodestreamError::SizeOverflow)?,
        )
        .ok_or(CodestreamError::SizeOverflow)?;
    Ok((
        LosslessEncodeRequirements {
            total_component_samples: samples,
            code_blocks: blocks,
            working_bytes: working,
            output_capacity_limit: limits.max_output_bytes,
        },
        workers,
    ))
}

pub(super) fn resource_error(message: &'static str) -> CodestreamError {
    unsupported(
        None,
        Some(Marker::Siz),
        UnsupportedConstruct::PacketDecode,
        message,
    )
}

pub(super) fn reserve_output(
    output: &mut Vec<u8>,
    additional: usize,
    maximum: usize,
) -> Result<()> {
    let needed = output
        .len()
        .checked_add(additional)
        .ok_or(CodestreamError::SizeOverflow)?;
    if needed > maximum {
        return Err(resource_error(
            "lossless D2 output-capacity budget is insufficient",
        ));
    }
    if needed > output.capacity() {
        let target = needed.max(output.capacity().saturating_mul(2).min(maximum));
        output
            .try_reserve_exact(target - output.len())
            .map_err(|_| CodestreamError::SizeOverflow)?;
        // Capacity is part of the contract, even on an allocator/library that
        // chooses a larger capacity for an exact request.
        if output.capacity() > maximum {
            return Err(resource_error(
                "lossless D2 allocation exceeds output-capacity budget",
            ));
        }
    }
    Ok(())
}

/// Opt-in timings for the unchanged scalable D2 writer. Stage intervals are
/// disjoint; total also includes validation, accounting and local destruction.
/// Counters describe completed successful calls, not an allocation or RSS meter.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LosslessEncodeTimings {
    pub total_ns: u128,
    pub conversion_level_shift_rct_ns: u128,
    pub forward_dwt_ns: u128,
    /// Subband exponent scans and block geometry/descriptor preparation.
    pub block_preparation_ns: u128,
    /// Existing checked baseline Tier-1 call, including its internal preparation.
    pub tier1_ns: u128,
    pub packet_headers_ns: u128,
    /// Main header, output appends, packet insertion and codestream closure.
    pub assembly_ns: u128,
    pub component_samples: u64,
    pub rct_pixels: u64,
    pub checked_tier1_blocks: u64,
    pub included_tier1_blocks: u64,
    pub tier1_coefficients: u64,
    pub tier1_coding_passes: u64,
    pub tier1_codeword_bytes: u64,
    pub packets: u64,
    pub packet_header_bytes: u64,
    pub packet_body_bytes_moved: u64,
}

/// Observed scheduling for a successful profiled D2 encode.
/// Slot count bounds simultaneous work; distinct participants can exceed slots
/// when the memory allowance is tighter than the calling pool size.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct LosslessEncodeExecution {
    /// Maximum concurrent slots admitted from the current pool and byte budget.
    pub effective_workers: usize,
    /// Distinct Rayon workers observed executing Tier-1 (one for serial calls).
    pub participating_workers: usize,
    /// Largest joined batch, including excluded zero blocks.
    pub max_batch_blocks: usize,
}

// A false const parameter removes clock reads and accounting from ordinary
// instantiations, including no-std builds. It does not select a different codec.
pub(super) struct EncodeClock {
    #[cfg(feature = "std")]
    started: Option<std::time::Instant>,
}
impl EncodeClock {
    #[inline]
    pub(super) fn start<const PROFILE: bool>() -> Self {
        Self {
            #[cfg(feature = "std")]
            started: if PROFILE {
                Some(std::time::Instant::now())
            } else {
                None
            },
        }
    }
    #[inline]
    pub(super) fn ns(self) -> u128 {
        #[cfg(feature = "std")]
        {
            self.started.map_or(0, |s| s.elapsed().as_nanos())
        }
        #[cfg(not(feature = "std"))]
        {
            0
        }
    }
}

/// Profile the same admitted D2 writer, with the same validation and bytes.
/// This diagnostic adds clock/accounting overhead; ordinary encoding remains
/// authoritative for throughput. No Tier-1 inner-operation counts are inferred.
#[cfg(feature = "std")]
pub fn encode_lossless_d2_profiled(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<(Vec<u8>, LosslessEncodeTimings)> {
    let (bytes, timings, _) =
        encode_lossless_d2_execution_profiled(width, height, bits, planes, limits)?;
    Ok((bytes, timings))
}

/// Profile D2 encoding with stage timings and separate scheduling observations.
/// The existing timing type and profiled entry point retain their original
/// shape; new execution observations do not extend that public timing struct.
#[cfg(feature = "std")]
pub fn encode_lossless_d2_execution_profiled(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<(Vec<u8>, LosslessEncodeTimings, LosslessEncodeExecution)> {
    let start = EncodeClock::start::<true>();
    let mut timings = LosslessEncodeTimings::default();
    let mut execution = LosslessEncodeExecution::default();
    let bytes = encode_lossless_d2_impl::<true, false>(
        width,
        height,
        bits,
        planes,
        limits,
        &mut timings,
        &mut execution,
    )?;
    timings.total_ns = start.ns();
    Ok((bytes, timings, execution))
}

/// Test-support observation of the existing bypass writer; not headline timing.
#[cfg(all(feature = "std", feature = "test-fixtures"))]
pub fn encode_lossless_d2_bypass_execution_profiled(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<(Vec<u8>, LosslessEncodeTimings, LosslessEncodeExecution)> {
    let start = EncodeClock::start::<true>();
    let mut timings = LosslessEncodeTimings::default();
    let mut execution = LosslessEncodeExecution::default();
    let bytes = encode_lossless_d2_impl::<true, true>(
        width,
        height,
        bits,
        planes,
        limits,
        &mut timings,
        &mut execution,
    )?;
    timings.total_ns = start.ns();
    Ok((bytes, timings, execution))
}

/// Encode the bounded profile without packed input or complete packet copies.
/// Eight unsigned U16 components are coded independently without MCT.
pub fn encode_lossless_d2(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    #[cfg(feature = "classic-execution-diagnostics")]
    if diagnostics::enabled() {
        let start = EncodeClock::start::<true>();
        let mut timings = LosslessEncodeTimings::default();
        let mut execution = LosslessEncodeExecution::default();
        let result = encode_lossless_d2_impl::<true, false>(
            width,
            height,
            bits,
            planes,
            limits,
            &mut timings,
            &mut execution,
        );
        timings.total_ns = start.ns();
        diagnostics::finish(timings, execution);
        return result;
    }
    encode_lossless_d2_impl::<false, false>(
        width,
        height,
        bits,
        planes,
        limits,
        &mut LosslessEncodeTimings::default(),
        &mut LosslessEncodeExecution::default(),
    )
}

/// Encode bounded raw D2 with selective arithmetic bypass (COD style 1).
/// Geometry and sample models match `encode_lossless_d2`; segment metadata is
/// included in `lossless_d2_bypass_requirements`. No partial output is returned.
pub fn encode_lossless_d2_bypass(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    #[cfg(feature = "classic-execution-diagnostics")]
    if diagnostics::enabled() {
        let start = EncodeClock::start::<true>();
        let mut timings = LosslessEncodeTimings::default();
        let mut execution = LosslessEncodeExecution::default();
        let result = encode_lossless_d2_impl::<true, true>(
            width,
            height,
            bits,
            planes,
            limits,
            &mut timings,
            &mut execution,
        );
        timings.total_ns = start.ns();
        diagnostics::finish(timings, execution);
        return result;
    }
    encode_lossless_d2_impl::<false, true>(
        width,
        height,
        bits,
        planes,
        limits,
        &mut LosslessEncodeTimings::default(),
        &mut LosslessEncodeExecution::default(),
    )
}

fn encode_lossless_d2_impl<const PROFILE: bool, const BYPASS: bool>(
    width: u32,
    height: u32,
    bits: u8,
    planes: &[LosslessD2Plane<'_>],
    limits: LosslessEncodeLimits,
    timings: &mut LosslessEncodeTimings,
    execution: &mut LosslessEncodeExecution,
) -> Result<Vec<u8>> {
    let (_, workers) = execution_requirements::<BYPASS>(
        width,
        height,
        u16::try_from(planes.len()).map_err(|_| CodestreamError::SizeOverflow)?,
        limits,
    )?;
    if !matches!(bits, 8 | 16) || (planes.len() == 8 && bits != 16) {
        return Err(resource_error(
            "lossless D2 requires unsigned 8-bit or 16-bit input",
        ));
    }
    let bytes = usize::from(bits / 8);
    let width_usize = width as usize;
    let count = width_usize
        .checked_mul(height as usize)
        .ok_or(CodestreamError::SizeOverflow)?;
    // Validate every borrowed component before allocating coefficient storage.
    for plane in planes {
        let row = (width_usize - 1)
            .checked_mul(plane.sample_step_bytes)
            .and_then(|n| n.checked_add(bytes))
            .ok_or(CodestreamError::SizeOverflow)?;
        let required = (height as usize - 1)
            .checked_mul(plane.stride_bytes)
            .and_then(|n| n.checked_add(row))
            .ok_or(CodestreamError::SizeOverflow)?;
        if plane.sample_step_bytes < bytes
            || plane.stride_bytes < row
            || plane.samples.len() < required
        {
            return Err(resource_error(
                "lossless D2 component storage is too short or has an invalid stride",
            ));
        }
    }
    let start = EncodeClock::start::<PROFILE>();
    let mut coefficients = Vec::with_capacity(planes.len());
    for plane in planes {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if values.capacity() != count {
            return Err(CodestreamError::SizeOverflow);
        }
        for y in 0..height as usize {
            for x in 0..width_usize {
                let offset = y * plane.stride_bytes + x * plane.sample_step_bytes;
                let sample = if bits == 8 {
                    i32::from(plane.samples[offset])
                } else {
                    i32::from(u16::from_le_bytes([
                        plane.samples[offset],
                        plane.samples[offset + 1],
                    ]))
                };
                values.push(sample - (1_i32 << (bits - 1)));
            }
        }
        coefficients.push(values);
    }
    #[cfg(feature = "classic-execution-diagnostics")]
    if PROFILE {
        let ns = start.started.map_or(0, |s| s.elapsed().as_nanos());
        diagnostics::update(|d| d.conversion_level_shift_ns = ns);
    }
    if let [red, green, blue] = coefficients.as_mut_slice() {
        #[cfg(feature = "classic-execution-diagnostics")]
        let rct = EncodeClock::start::<PROFILE>();
        transform::forward_reversible_color_transform_bounded(red, green, blue)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        #[cfg(feature = "classic-execution-diagnostics")]
        if PROFILE {
            let ns = rct.ns();
            diagnostics::update(|d| d.forward_rct_ns = ns);
        }
    }
    if PROFILE {
        timings.conversion_level_shift_rct_ns = start.ns();
        timings.component_samples = count as u64 * planes.len() as u64;
        timings.rct_pixels = if planes.len() == 3 { count as u64 } else { 0 };
    }
    let start = EncodeClock::start::<PROFILE>();
    #[cfg(feature = "classic-execution-diagnostics")]
    let preparation = EncodeClock::start::<PROFILE>();
    let mut prepared_transform = prepare_forward53(width_usize, height as usize, workers)?;
    let mut transform_scratch = Vec::new();
    #[cfg(feature = "classic-execution-diagnostics")]
    if PROFILE {
        diagnostics::update(|d| d.dwt_scratch_resize_ns += preparation.ns());
    }
    let mut transform_components = || -> Result<()> {
        for plane in &mut coefficients {
            if let Some((plan, workspace)) = prepared_transform.as_mut() {
                transform::forward_reversible_5_3_planned_bounded(plane, plan, workspace)
                    .map_err(|_| CodestreamError::SizeOverflow)?;
            } else {
                forward_reversible_5_3_levels_with_scratch(
                    width,
                    height,
                    plane,
                    2,
                    "lossless D2 transform failed",
                    &mut transform_scratch,
                )?;
                #[cfg(feature = "classic-execution-diagnostics")]
                if PROFILE {
                    diagnostics::update(|d| {
                        d.forward53_original_helper_calls += 1;
                        d.forward53_original_scratch_capacity_bytes =
                            transform_scratch.capacity() * 4;
                    });
                }
            }
        }
        Ok(())
    };
    #[cfg(feature = "classic-execution-diagnostics")]
    if PROFILE && diagnostics::enabled() {
        let (result, detail) = transform::observe_forward_transform(transform_components);
        result?;
        diagnostics::update(|d| {
            d.dwt_validation_ns = detail.validation_ns;
            d.dwt_vertical_gather_ns = detail.vertical_gather_ns;
            d.dwt_vertical_lifting_ns = detail.vertical_lifting_ns;
            d.dwt_vertical_store_ns = detail.vertical_store_ns;
            d.dwt_horizontal_lifting_ns = detail.horizontal_lifting_ns;
            d.dwt_horizontal_copy_ns = detail.horizontal_copy_ns;
            d.forward53 = detail;
            if prepared_transform.is_none() {
                d.forward53.level_backends = [Some(transform::Forward53Backend::Reference); 2];
            }
        });
    } else {
        transform_components()?;
    }
    #[cfg(not(feature = "classic-execution-diagnostics"))]
    transform_components()?;
    #[cfg(feature = "classic-execution-diagnostics")]
    let dropping = EncodeClock::start::<PROFILE>();
    drop(prepared_transform);
    drop(transform_scratch);
    #[cfg(feature = "classic-execution-diagnostics")]
    if PROFILE {
        let ns = dropping.ns();
        diagnostics::update(|d| d.dwt_scratch_drop_ns += ns);
    }
    if PROFILE {
        timings.forward_dwt_ns = start.ns();
    }
    let start = EncodeClock::start::<PROFILE>();
    let specs = decomp_subband_specs(width, height, 2)?;
    let refs = coefficients.iter().map(Vec::as_slice).collect::<Vec<_>>();
    let exponents = specs
        .iter()
        .map(|spec| max_component_subband_available_bitplanes(width, &refs, *spec))
        .collect::<Result<Vec<_>>>()?;
    if BYPASS && exponents.iter().any(|&planes| planes > 31) {
        return Err(CodestreamError::SizeOverflow);
    }
    if PROFILE {
        timings.block_preparation_ns += start.ns();
    }
    let start = EncodeClock::start::<PROFILE>();
    let maximum =
        usize::try_from(limits.max_output_bytes).map_err(|_| CodestreamError::SizeOverflow)?;
    let mut output = Vec::new();
    reserve_output(&mut output, 128, maximum)?;
    write_native_main_header(
        &mut output,
        width,
        height,
        width,
        height,
        bits,
        planes.len() as u16,
        planes.len() == 3,
        2,
        &exponents,
        false,
        u8::from(BYPASS),
        1,
    )?;
    let sot = output.len();
    output.extend_from_slice(&[0xff, 0x90, 0, 10, 0, 0, 0, 0, 0, 0, 0, 1, 0xff, 0x93]);
    let mut scratch = tier1::CodeBlockEncodeScratch::new();
    #[cfg(feature = "parallel")]
    let mut parallel = parallel::BlockWorkers::new(if workers > 1 { workers } else { 0 })?;
    if PROFILE {
        execution.effective_workers = workers;
        execution.participating_workers = 1;
        execution.max_batch_blocks = 1;
    }
    if PROFILE {
        timings.assembly_ns += start.ns();
    }
    for resolution in 0..=2 {
        for plane in &coefficients {
            let body_start = output.len();
            let mut bands = Vec::with_capacity(3);
            let mut bypass_lengths = Vec::with_capacity(if BYPASS { 3 } else { 0 });
            for (spec, exponent) in specs
                .iter()
                .zip(&exponents)
                .filter(|(spec, _)| spec.resolution == resolution)
            {
                #[cfg(feature = "parallel")]
                if workers > 1 {
                    let (band, lengths) = parallel.encode_subband::<PROFILE, BYPASS>(
                        width,
                        plane,
                        *spec,
                        *exponent,
                        &mut output,
                        maximum,
                        timings,
                        execution,
                    )?;
                    bands.push(band);
                    if BYPASS {
                        bypass_lengths.push(lengths);
                    }
                    continue;
                }
                if BYPASS {
                    let start = EncodeClock::start::<PROFILE>();
                    let (band, lengths) = bypass::encode_subband(
                        width,
                        plane,
                        *spec,
                        *exponent,
                        &mut output,
                        maximum,
                        &mut scratch,
                    )?;
                    if PROFILE {
                        // Serial bypass includes its subband preparation and
                        // output appends in this interval, unlike the parallel
                        // collector's separate assembly interval.
                        timings.tier1_ns += start.ns();
                        for block in &band.code_blocks {
                            timings.checked_tier1_blocks += 1;
                            timings.included_tier1_blocks += u64::from(block.included);
                            timings.tier1_coefficients +=
                                u64::from(block.width) * u64::from(block.height);
                            timings.tier1_coding_passes += u64::from(block.coding_passes);
                            timings.tier1_codeword_bytes += block.segment_len as u64;
                        }
                    }
                    bands.push(band);
                    bypass_lengths.push(lengths);
                    continue;
                }
                bands.push(encode_decomp_subband_with_output_limit::<PROFILE>(
                    width,
                    plane,
                    *spec,
                    *exponent,
                    &mut output,
                    &mut scratch,
                    Some(maximum),
                    timings,
                )?);
            }
            let start = EncodeClock::start::<PROFILE>();
            let mut header = PacketBitWriter::new();
            let present = bands
                .iter()
                .any(|band| band.code_blocks.iter().any(|block| block.included));
            header.write_bit(u32::from(present))?;
            if present {
                for (index, band) in bands.iter().enumerate() {
                    if BYPASS {
                        bypass::write_packet_header(&mut header, band, &bypass_lengths[index])?;
                        continue;
                    }
                    write_component_packet_header(
                        &mut header,
                        band.code_block_cols,
                        band.code_block_rows,
                        &band.code_blocks,
                    )?;
                }
            }
            header.align();
            let header = header.bytes();
            if PROFILE {
                timings.packet_headers_ns += start.ns();
                timings.packets += 1;
                timings.packet_header_bytes += header.len() as u64;
                timings.packet_body_bytes_moved += (output.len() - body_start) as u64;
            }
            let start = EncodeClock::start::<PROFILE>();
            reserve_output(&mut output, header.len(), maximum)?;
            let old_len = output.len();
            output.resize(old_len + header.len(), 0);
            output.copy_within(body_start..old_len, body_start + header.len());
            output[body_start..body_start + header.len()].copy_from_slice(header);
            if PROFILE {
                timings.assembly_ns += start.ns();
            }
        }
    }
    let start = EncodeClock::start::<PROFILE>();
    let tile_len = u32::try_from(output.len() - sot).map_err(|_| CodestreamError::SizeOverflow)?;
    output[sot + 6..sot + 10].copy_from_slice(&tile_len.to_be_bytes());
    reserve_output(&mut output, 2, maximum)?;
    output.extend_from_slice(&[0xff, 0xd9]);
    if PROFILE {
        timings.assembly_ns += start.ns();
    }
    Ok(output)
}

#[cfg(test)]
mod forward53_tests {
    use super::*;

    #[test]
    fn optional_workspace_shrinks_or_falls_back_before_allocation() {
        for (width, height) in [
            (4, 32768),
            (32768, 4),
            (129, 131),
            (32768, 2048),
            (2048, 32768),
        ] {
            for workers in [1, 2, 4, 8] {
                let prepared = prepare_forward53(width, height, workers).unwrap();
                if workers == 1 || width <= 16 || !cfg!(feature = "parallel") {
                    assert!(prepared.is_none());
                }
                if let Some((plan, workspace)) = prepared {
                    assert!(plan.slots() >= 2 && plan.slots() <= workers);
                    assert_eq!(workspace.capacity_bytes(), plan.workspace_bytes());
                    assert!(
                        workspace.capacity_bytes()
                            <= 12 * width.max(height) + WORKER_BYTES as usize * workers - 4096
                    );
                }
            }
        }
        let mut attempts = 0;
        let mut prepare =
            |w: &mut transform::Forward53Workspace, p: &transform::Forward53Plan, m| {
                attempts += 1;
                w.prepare(p, m)
            };
        let absent = prepare_forward53_with_maximum(
            513,
            515,
            8,
            12 * 515,
            FORWARD53_BACKEND,
            16,
            &mut prepare,
        )
        .unwrap();
        assert!(absent.is_none());
        assert_eq!(attempts, 0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn failed_optional_reservations_are_released_and_never_become_scalar_panels() {
        let mut attempts = Vec::new();
        let absent = prepare_forward53_with_maximum(
            513,
            515,
            8,
            32 << 20,
            FORWARD53_BACKEND,
            16,
            |workspace, plan, maximum| {
                assert_eq!(workspace.capacity_bytes(), 0);
                attempts.push(plan.slots());
                workspace.prepare(plan, maximum)?;
                Err(transform::TransformError::SizeOverflow)
            },
        )
        .unwrap();
        assert!(absent.is_none());
        assert_eq!(attempts, [8, 4, 2]);
        let (plan, workspace) = prepare_forward53_with_maximum(
            513,
            515,
            8,
            2 * 4 * (16 * 515 + 515),
            FORWARD53_BACKEND,
            16,
            |w, p, m| w.prepare(p, m),
        )
        .unwrap()
        .unwrap();
        assert_eq!(plan.slots(), 2);
        assert_eq!(workspace.capacity_bytes(), plan.workspace_bytes());
        let (wide, _) = prepare_forward53(513, 515, usize::MAX).unwrap().unwrap();
        let (eight, _) = prepare_forward53(513, 515, 8).unwrap().unwrap();
        assert_eq!(wide.workspace_bytes(), eight.workspace_bytes());
    }
}
