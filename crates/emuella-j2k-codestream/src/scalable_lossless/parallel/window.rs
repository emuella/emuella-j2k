//! Finite dynamically claimed windows, with W scratch objects and bounded results.
use super::*;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
struct ResultSlot {
    bytes: Vec<u8>,
    lengths: Vec<usize>,
    result: Option<Result<EncodedCodeBlock>>,
    worker: Option<usize>,
    #[cfg(feature = "classic-execution-diagnostics")]
    interval: diagnostics::BlockInterval,
}

pub(in crate::scalable_lossless) struct Window {
    scratch: Vec<tier1::CodeBlockEncodeScratch>,
    results: Vec<Mutex<ResultSlot>>,
    participants: Vec<usize>,
}

fn exact_vec<T: Default>(count: usize) -> Result<Vec<T>> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| CodestreamError::SizeOverflow)?;
    if values.capacity() != count {
        return Err(CodestreamError::SizeOverflow);
    }
    values.resize_with(count, T::default);
    Ok(values)
}

// Each claimant owns exactly one reusable scratch. Result slots are never reused
// until this finite window joins. Mutexes provide safe exclusive access, but the
// atomic index gives each slot exactly one claimant and hence no slot contention.
fn run_window<S: Send, R: Send>(
    scratch: &mut [S],
    results: &[Mutex<R>],
    job: impl Fn(usize, &mut S, &mut R) + Sync,
) {
    let next = AtomicUsize::new(0);
    scratch.par_iter_mut().for_each(|scratch| {
        loop {
            let index = next.fetch_add(1, Ordering::Relaxed);
            let Some(slot) = results.get(index) else {
                break;
            };
            let mut slot = slot
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            job(index, scratch, &mut slot);
        }
    });
}

impl Window {
    pub(super) fn new(workers: usize, slots: usize) -> Result<Self> {
        if workers < 2 || slots <= workers || slots > workers.saturating_mul(4) {
            return Err(CodestreamError::SizeOverflow);
        }
        // Admission already charged 4 MiB for each worker and each extra result.
        // Metadata, fixed queue atomics and diagnostic endpoints fit far below
        // that bound; no additional scratch is constructed per result or job.
        const _: () = assert!(core::mem::size_of::<Mutex<ResultSlot>>() < 1024);
        const _: () = assert!(core::mem::size_of::<tier1::CodeBlockEncodeScratch>() < 1024);
        Ok(Self {
            scratch: exact_vec(workers)?,
            results: exact_vec(slots)?,
            participants: Vec::new(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn encode_subband<const PROFILE: bool, const BYPASS: bool>(
        &mut self,
        image_width: u32,
        plane: &[i32],
        spec: DecompSubbandSpec,
        available_bitplanes: u8,
        output: &mut Vec<u8>,
        maximum: usize,
        timings: &mut LosslessEncodeTimings,
        execution: &mut LosslessEncodeExecution,
    ) -> Result<(NativeDecompSubband, Vec<bypass::SegmentLengths>)> {
        if spec.grid_x0 != 0 || spec.grid_y0 != 0 {
            return Err(CodestreamError::SizeOverflow);
        }
        let start = EncodeClock::start::<PROFILE>();
        let cols =
            u16::try_from(spec.width.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
        let rows =
            u16::try_from(spec.height.div_ceil(64)).map_err(|_| CodestreamError::SizeOverflow)?;
        let count = usize::from(cols)
            .checked_mul(usize::from(rows))
            .ok_or(CodestreamError::SizeOverflow)?;
        let mut blocks = Vec::new();
        blocks
            .try_reserve_exact(count)
            .map_err(|_| CodestreamError::SizeOverflow)?;
        if blocks.capacity() != count {
            return Err(CodestreamError::SizeOverflow);
        }
        let mut lengths = bypass::metadata(if BYPASS { count } else { 0 })?;
        if PROFILE {
            timings.block_preparation_ns += start.ns();
        }
        for first in (0..count).step_by(self.results.len()) {
            let active = (count - first).min(self.results.len());
            let start = EncodeClock::start::<PROFILE>();
            #[cfg(feature = "classic-execution-diagnostics")]
            let batch_start = std::time::Instant::now();
            run_window(
                &mut self.scratch,
                &self.results[..active],
                |offset, scratch, slot| {
                    slot.bytes.clear();
                    if PROFILE {
                        slot.worker = rayon::current_thread_index();
                    }
                    #[cfg(feature = "classic-execution-diagnostics")]
                    let block_start = batch_start.elapsed().as_nanos();
                    slot.result = Some(encode_block::<BYPASS>(
                        image_width,
                        plane,
                        spec,
                        available_bitplanes,
                        cols,
                        first + offset,
                        &mut slot.bytes,
                        &mut slot.lengths,
                        scratch,
                    ));
                    #[cfg(feature = "classic-execution-diagnostics")]
                    {
                        slot.interval = diagnostics::BlockInterval {
                            start: block_start,
                            end: batch_start.elapsed().as_nanos(),
                            packed: scratch.diagnostic_storage().1,
                        };
                    }
                },
            );
            #[cfg(feature = "classic-execution-diagnostics")]
            let joined_ns = batch_start.elapsed().as_nanos();
            if PROFILE {
                timings.tier1_ns += start.ns();
                execution.max_batch_blocks = execution.max_batch_blocks.max(active);
            }
            #[cfg(feature = "classic-execution-diagnostics")]
            let metadata = self.results.capacity() * core::mem::size_of::<Mutex<ResultSlot>>()
                + self.scratch.capacity() * core::mem::size_of::<tier1::CodeBlockEncodeScratch>();
            #[cfg(feature = "classic-execution-diagnostics")]
            diagnostics::batch(
                self.results[..active].iter_mut().map(|s| {
                    s.get_mut()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .interval
                }),
                joined_ns,
                self.scratch.iter().map(|s| s.diagnostic_storage().0).sum(),
                // All slots remain live, including a previous longer window.
                0,
                0,
                metadata,
            );
            #[cfg(feature = "classic-execution-diagnostics")]
            {
                let mut bytes = 0;
                let mut collectors = 0;
                for slot in &mut self.results {
                    let slot = slot
                        .get_mut()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    bytes += slot.bytes.capacity();
                    collectors += slot.lengths.capacity() * core::mem::size_of::<usize>();
                }
                diagnostics::retained_results(bytes, collectors);
            }
            let start = EncodeClock::start::<PROFILE>();
            #[cfg(feature = "classic-execution-diagnostics")]
            let append_start = std::time::Instant::now();
            // Joining completed all started jobs, regardless of their Result.
            // Taking in stream order preserves first-error selection and bytes.
            for slot in &mut self.results[..active] {
                let slot = slot
                    .get_mut()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let mut block = slot.result.take().ok_or(CodestreamError::SizeOverflow)??;
                if PROFILE {
                    if let Some(worker) = slot.worker
                        && !self.participants.contains(&worker)
                    {
                        self.participants.push(worker);
                    }
                    execution.participating_workers = self.participants.len().max(1);
                    timings.checked_tier1_blocks += 1;
                    timings.included_tier1_blocks += u64::from(block.included);
                    timings.tier1_coefficients += u64::from(block.width) * u64::from(block.height);
                    timings.tier1_coding_passes += u64::from(block.coding_passes);
                    timings.tier1_codeword_bytes += block.segment_len as u64;
                }
                let segments = if BYPASS {
                    if block.segment_len != slot.bytes.len() {
                        return Err(CodestreamError::SizeOverflow);
                    }
                    Some(bypass::SegmentLengths::checked(
                        block.coding_passes,
                        &slot.bytes,
                        &slot.lengths,
                    )?)
                } else {
                    None
                };
                reserve_output(output, slot.bytes.len(), maximum)?;
                block.segment_offset = output.len();
                output.extend_from_slice(&slot.bytes);
                blocks.push(block);
                if let Some(segments) = segments {
                    lengths.push(segments);
                }
            }
            #[cfg(feature = "classic-execution-diagnostics")]
            diagnostics::append(append_start.elapsed().as_nanos());
            if PROFILE {
                timings.assembly_ns += start.ns();
            }
        }
        Ok((
            NativeDecompSubband {
                index: spec.index,
                resolution: spec.resolution,
                kind: spec.kind,
                x: spec.x,
                y: spec.y,
                width: spec.width,
                height: spec.height,
                code_block_cols: cols,
                code_block_rows: rows,
                available_bitplanes,
                code_blocks: blocks,
            },
            lengths,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Condvar};

    #[test]
    fn finite_claims_bound_slow_first_results_join_faults_and_release_storage() {
        struct Scratch(Arc<AtomicUsize>);
        impl Drop for Scratch {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        struct ResultProbe {
            value: Option<std::result::Result<usize, usize>>,
            drops: Arc<AtomicUsize>,
        }
        impl Drop for ResultProbe {
            fn drop(&mut self) {
                self.drops.fetch_add(1, Ordering::SeqCst);
            }
        }
        for workers in [2, 4, 8] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| {
                    for multiplier in [2, 4] {
                        let count = workers * multiplier;
                        let scratch_drops = Arc::new(AtomicUsize::new(0));
                        let result_drops = Arc::new(AtomicUsize::new(0));
                        let mut scratch: Vec<_> = (0..workers)
                            .map(|_| Scratch(scratch_drops.clone()))
                            .collect();
                        let results: Vec<_> = (0..count)
                            .map(|_| {
                                Mutex::new(ResultProbe {
                                    value: None,
                                    drops: result_drops.clone(),
                                })
                            })
                            .collect();
                        let finished = (Mutex::new(Vec::new()), Condvar::new());
                        let active = AtomicUsize::new(0);
                        let peak = AtomicUsize::new(0);
                        run_window(&mut scratch, &results, |index, _, result| {
                            let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                            peak.fetch_max(now, Ordering::SeqCst);
                            // Only jobs in this finite window are awaited. At least
                            // two admitted scratch lanes make the gate independent
                            // of the delayed first job and no sleep/timing race is used.
                            if index == 0 {
                                let guard = finished.0.lock().unwrap();
                                let guard = finished
                                    .1
                                    .wait_while(guard, |v| v.len() != count - 1)
                                    .unwrap();
                                assert_eq!(guard.len(), results.len() - 1);
                                assert_eq!(scratch_drops.load(Ordering::SeqCst), 0);
                                assert_eq!(result_drops.load(Ordering::SeqCst), 0);
                            }
                            result.value = Some(if index == 0 || index == 3 {
                                Err(index)
                            } else {
                                Ok(index)
                            });
                            finished.0.lock().unwrap().push(index);
                            finished.1.notify_all();
                            active.fetch_sub(1, Ordering::SeqCst);
                        });
                        assert_eq!(active.load(Ordering::SeqCst), 0);
                        assert!(peak.load(Ordering::SeqCst) <= workers);
                        assert_eq!(finished.0.lock().unwrap().last(), Some(&0));
                        let values: Vec<_> = results
                            .iter()
                            .map(|r| r.lock().unwrap().value.unwrap())
                            .collect();
                        assert_eq!(values.len(), count);
                        assert_eq!(values[0], Err(0));
                        assert_eq!(values[3], Err(3));
                        assert!(
                            values
                                .iter()
                                .enumerate()
                                .all(|(i, v)| *v == if i == 0 || i == 3 { Err(i) } else { Ok(i) })
                        );
                        // Reuse has no stale failure and all original scratch/result
                        // storage survives until the explicit owner drops it.
                        run_window(&mut scratch, &results, |i, _, r| r.value = Some(Ok(i)));
                        assert!(
                            results
                                .iter()
                                .enumerate()
                                .all(|(i, r)| r.lock().unwrap().value == Some(Ok(i)))
                        );
                        drop(results);
                        drop(scratch);
                        assert_eq!(scratch_drops.load(Ordering::SeqCst), workers);
                        assert_eq!(result_drops.load(Ordering::SeqCst), count);
                    }
                });
        }
    }

    #[test]
    fn empty_partial_and_one_lane_claims_do_not_require_other_work() {
        for workers in [1, 2, 4, 8] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| {
                    for count in [0, 1, 3, workers * 2 - 1] {
                        let mut scratch = vec![0; workers];
                        let results: Vec<_> = (0..count).map(|_| Mutex::new(None)).collect();
                        run_window(&mut scratch, &results, |i, s, r| {
                            *s += 1;
                            *r = Some(i);
                        });
                        assert_eq!(scratch.iter().sum::<usize>(), count);
                        assert!(
                            results
                                .iter()
                                .enumerate()
                                .all(|(i, r)| *r.lock().unwrap() == Some(i))
                        );
                    }
                });
        }
    }

    #[test]
    fn panic_joins_other_lanes_and_reusable_poisoned_slots_are_replaced() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
            .install(|| {
                let mut scratch = [0; 2];
                let results: Vec<_> = (0..4).map(|_| Mutex::new(None)).collect();
                let completed = AtomicUsize::new(0);
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    run_window(&mut scratch, &results, |i, _, r| {
                        if i == 0 {
                            panic!("authored fault");
                        }
                        *r = Some(i);
                        completed.fetch_add(1, Ordering::SeqCst);
                    });
                }));
                assert!(result.is_err());
                assert_eq!(completed.load(Ordering::SeqCst), 3);
                run_window(&mut scratch, &results, |i, _, r| *r = Some(i));
                assert!(results.iter().enumerate().all(|(i, r)| {
                    *r.lock().unwrap_or_else(std::sync::PoisonError::into_inner) == Some(i)
                }));
            });
    }

    #[test]
    fn windows_preserve_strided_edge_bytes_metadata_fault_order_and_reuse() {
        compare::<false>();
        compare::<true>();
    }
    fn compare<const BYPASS: bool>() {
        for workers in [2, 4, 8] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| {
                    for multiplier in [2, 4] {
                        let mut window = Window::new(workers, workers * multiplier).unwrap();
                        let mut batches = Batches::new(workers).unwrap();
                        for (width, height) in [(0, 0), (1, 1), (63, 65), (129, 131), (513, 67)] {
                            for pattern in 0..3 {
                                let stride = width + 7;
                                let spec = DecompSubbandSpec {
                                    index: 0,
                                    resolution: 0,
                                    kind: PacketSubbandKind::LowLow,
                                    x: 0,
                                    y: 0,
                                    grid_x0: 0,
                                    grid_y0: 0,
                                    width,
                                    height,
                                };
                                let mut plane = vec![0; (stride * height) as usize];
                                for y in 0..height {
                                    for x in 0..width {
                                        plane[(y * stride + x) as usize] = match pattern {
                                            0 => 0,
                                            1 => {
                                                if (x + y).is_multiple_of(71) {
                                                    511
                                                } else {
                                                    0
                                                }
                                            }
                                            _ => ((x * 7919 + y * 3109) % 2048) as i32 - 1024,
                                        };
                                    }
                                }
                                let mut a = vec![0x71; 13];
                                let mut b = a.clone();
                                let mut timings = LosslessEncodeTimings::default();
                                let mut execution = LosslessEncodeExecution::default();
                                let actual = window
                                    .encode_subband::<true, BYPASS>(
                                        stride,
                                        &plane,
                                        spec,
                                        12,
                                        &mut a,
                                        1 << 20,
                                        &mut timings,
                                        &mut execution,
                                    )
                                    .unwrap();
                                let expected = batches
                                    .encode_subband::<true, BYPASS>(
                                        stride,
                                        &plane,
                                        spec,
                                        12,
                                        &mut b,
                                        1 << 20,
                                        &mut LosslessEncodeTimings::default(),
                                        &mut LosslessEncodeExecution::default(),
                                    )
                                    .unwrap();
                                assert_eq!(a, b);
                                assert_eq!(actual, expected);
                                assert_eq!(window.scratch.len(), workers);
                                assert_eq!(window.results.len(), workers * multiplier);
                                assert!(execution.max_batch_blocks <= workers * multiplier);
                                // Output exhaustion occurs only after the current
                                // finite window joins; a later reuse remains exact.
                                if a.len() > 13 {
                                    let failed = window.encode_subband::<false, BYPASS>(
                                        stride,
                                        &plane,
                                        spec,
                                        12,
                                        &mut vec![0x71; 13],
                                        13,
                                        &mut LosslessEncodeTimings::default(),
                                        &mut LosslessEncodeExecution::default(),
                                    );
                                    assert!(failed.is_err());
                                    let mut again = vec![0x71; 13];
                                    let recovered = window
                                        .encode_subband::<false, BYPASS>(
                                            stride,
                                            &plane,
                                            spec,
                                            12,
                                            &mut again,
                                            1 << 20,
                                            &mut LosslessEncodeTimings::default(),
                                            &mut LosslessEncodeExecution::default(),
                                        )
                                        .unwrap();
                                    assert_eq!(again, a);
                                    assert_eq!(recovered, actual);
                                }
                                if width > 64 {
                                    plane[0] = i32::MAX;
                                    let mut a = vec![0x71; 13];
                                    let mut b = a.clone();
                                    let actual = window.encode_subband::<false, BYPASS>(
                                        stride,
                                        &plane,
                                        spec,
                                        1,
                                        &mut a,
                                        1 << 20,
                                        &mut LosslessEncodeTimings::default(),
                                        &mut LosslessEncodeExecution::default(),
                                    );
                                    let expected = batches.encode_subband::<false, BYPASS>(
                                        stride,
                                        &plane,
                                        spec,
                                        1,
                                        &mut b,
                                        1 << 20,
                                        &mut LosslessEncodeTimings::default(),
                                        &mut LosslessEncodeExecution::default(),
                                    );
                                    assert_eq!(actual, expected);
                                    assert!(actual.is_err());
                                    assert_eq!(a, b);
                                    for slot in window
                                        .results
                                        .iter_mut()
                                        .take(
                                            ((width.div_ceil(64) * height.div_ceil(64)) as usize)
                                                .min(workers * multiplier),
                                        )
                                        .skip(1)
                                    {
                                        assert!(slot.get_mut().unwrap().result.is_some());
                                    }
                                }
                            }
                        }
                    }
                });
        }
    }

    #[test]
    fn earlier_output_exhaustion_wins_over_a_later_block_fault() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
            .install(|| {
                let spec = DecompSubbandSpec {
                    index: 0,
                    resolution: 0,
                    kind: PacketSubbandKind::LowLow,
                    x: 0,
                    y: 0,
                    grid_x0: 0,
                    grid_y0: 0,
                    width: 256,
                    height: 64,
                };
                let mut plane = vec![0; 256 * 64];
                plane[0] = 3;
                plane[64] = i32::MAX;
                let mut window = Window::new(2, 4).unwrap();
                let mut batches = Batches::new(2).unwrap();
                let mut a = vec![0x71; 13];
                let mut b = a.clone();
                let actual = window
                    .encode_subband::<false, false>(
                        256,
                        &plane,
                        spec,
                        3,
                        &mut a,
                        13,
                        &mut LosslessEncodeTimings::default(),
                        &mut LosslessEncodeExecution::default(),
                    )
                    .unwrap_err();
                let expected = batches
                    .encode_subband::<false, false>(
                        256,
                        &plane,
                        spec,
                        3,
                        &mut b,
                        13,
                        &mut LosslessEncodeTimings::default(),
                        &mut LosslessEncodeExecution::default(),
                    )
                    .unwrap_err();
                assert_eq!(actual, expected);
                assert_eq!(
                    actual,
                    resource_error("lossless D2 output-capacity budget is insufficient")
                );
                assert_eq!(a, vec![0x71; 13]);
                assert!(
                    window.results[1]
                        .get_mut()
                        .unwrap()
                        .result
                        .as_ref()
                        .unwrap()
                        .is_err()
                );
                assert!(
                    window.results[2..].iter_mut().all(|s| s
                        .get_mut()
                        .unwrap()
                        .result
                        .as_ref()
                        .unwrap()
                        .is_ok())
                );
            });
    }

    #[test]
    fn extra_results_never_consume_the_existing_worker_admission() {
        for workers in [1, 2, 4, 8] {
            for multiplier in [2, 4] {
                let original = WORKER_BYTES * workers as u64 + 12345;
                assert_eq!(
                    capacity_for(multiplier, workers, 100, original, original),
                    workers
                );
                assert_eq!(
                    capacity_for(
                        multiplier,
                        workers,
                        100,
                        original,
                        original + WORKER_BYTES - 1
                    ),
                    workers
                );
                for extra in 0..workers * multiplier {
                    let slots = capacity_for(
                        multiplier,
                        workers,
                        100,
                        original,
                        original + extra as u64 * WORKER_BYTES,
                    );
                    assert!(slots >= workers && slots <= workers * multiplier);
                    assert!((slots - workers) <= extra);
                    if workers == 1 {
                        assert_eq!(slots, 1);
                    }
                }
                assert_eq!(
                    capacity_for(multiplier, workers, workers as u64, original, u64::MAX),
                    workers
                );
            }
        }
    }
}
