//! Separate-build observations of the existing facade writer, never headline timing.
use super::{LosslessEncodeExecution, LosslessEncodeTimings};
use std::{cell::RefCell, time::Instant};

/// Fixed logarithmic duration distribution: bin i contains [2^(i-1), 2^i) ns,
/// with zero in bin zero and the last bin saturating. Sum/min/max remain exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Durations {
    pub bins: [u64; 64],
    pub count: u64,
    pub sum_ns: u128,
    pub min_ns: u128,
    pub max_ns: u128,
}
impl Default for Durations {
    fn default() -> Self {
        Self {
            bins: [0; 64],
            count: 0,
            sum_ns: 0,
            min_ns: 0,
            max_ns: 0,
        }
    }
}
impl Durations {
    fn add(&mut self, ns: u128) {
        let bin = (128 - ns.leading_zeros()).min(63) as usize;
        self.bins[bin] += 1;
        self.min_ns = if self.count == 0 {
            ns
        } else {
            self.min_ns.min(ns)
        };
        self.max_ns = self.max_ns.max(ns);
        self.count += 1;
        self.sum_ns += ns;
    }
}

/// Aggregated observations. Storage fields are retained requested capacities,
/// not growth peaks, allocator overhead or RSS. The separate allocation meter
/// observes transient live allocations. No coefficient or codeword is retained.
#[derive(Debug, Clone, Default)]
pub struct LosslessExecutionDiagnostic {
    pub timings: LosslessEncodeTimings,
    pub execution: LosslessEncodeExecution,
    /// Front-end subintervals nested within the existing stage totals.
    pub conversion_level_shift_ns: u128,
    pub forward_rct_ns: u128,
    pub dwt_scratch_resize_ns: u128,
    pub dwt_scratch_drop_ns: u128,
    pub dwt_validation_ns: u128,
    pub dwt_vertical_gather_ns: u128,
    pub dwt_vertical_lifting_ns: u128,
    pub dwt_vertical_store_ns: u128,
    pub dwt_horizontal_lifting_ns: u128,
    pub dwt_horizontal_copy_ns: u128,
    pub blocks: Durations,
    pub batch_tails: Durations,
    pub batches: u64,
    pub peak_active_blocks: usize,
    pub active_block_ns: u128,
    pub any_block_active_ns: u128,
    pub dispatch_to_first_start_ns: u128,
    pub last_finish_to_join_ns: u128,
    pub batch_dispatch_join_ns: u128,
    pub ordered_append_ns: u128,
    pub aggregation_ns: u128,
    pub retained_scratch_bytes: usize,
    pub retained_result_capacity_bytes: usize,
    pub retained_collector_capacity_bytes: usize,
    pub slot_metadata_bytes: usize,
    pub endpoint_capacity_bytes: usize,
    pub blocks_with_retained_packed_storage: u64,
    pub blocks_without_packed_storage: u64,
}

thread_local! {
    static CURRENT: RefCell<Option<LosslessExecutionDiagnostic>> = const { RefCell::new(None) };
}

/// Observe one synchronous facade call in a separate feature-enabled build.
/// Nested observation is rejected. The observer is removed even during unwinding.
/// All asynchronous codec jobs must join before the closure returns.
pub fn observe_lossless_encode<T>(call: impl FnOnce() -> T) -> (T, LosslessExecutionDiagnostic) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            CURRENT.with(|c| {
                c.borrow_mut().take();
            });
        }
    }
    CURRENT.with(|c| {
        assert!(
            c.borrow().is_none(),
            "nested lossless execution observation"
        );
        *c.borrow_mut() = Some(LosslessExecutionDiagnostic::default());
    });
    let reset = Reset;
    let result = call();
    let observation = CURRENT.with(|c| c.borrow_mut().take().expect("active observation"));
    drop(reset);
    (result, observation)
}

pub(super) fn enabled() -> bool {
    CURRENT.with(|c| c.borrow().is_some())
}
pub(crate) fn update(f: impl FnOnce(&mut LosslessExecutionDiagnostic)) {
    CURRENT.with(|c| {
        if let Some(d) = c.borrow_mut().as_mut() {
            f(d);
        }
    });
}
pub(super) fn finish(timings: LosslessEncodeTimings, execution: LosslessEncodeExecution) {
    update(|d| {
        d.timings = timings;
        d.execution = execution;
    });
}
pub(crate) fn append(ns: u128) {
    update(|d| d.ordered_append_ns += ns);
}

pub(crate) fn serial_block(ns: u128, storage: (usize, bool), result: usize, collector: usize) {
    let start = Instant::now();
    update(|d| {
        d.blocks.add(ns);
        d.peak_active_blocks = 1;
        d.active_block_ns += ns;
        d.any_block_active_ns += ns;
        d.retained_scratch_bytes = d.retained_scratch_bytes.max(storage.0);
        d.retained_result_capacity_bytes = d.retained_result_capacity_bytes.max(result);
        d.retained_collector_capacity_bytes = d.retained_collector_capacity_bytes.max(collector);
        d.blocks_with_retained_packed_storage += u64::from(storage.1);
        d.blocks_without_packed_storage += u64::from(!storage.1);
    });
    update(|d| d.aggregation_ns += start.elapsed().as_nanos());
}

#[cfg(feature = "parallel")]
#[derive(Clone, Copy, Default)]
pub(super) struct BlockInterval {
    pub start: u128,
    pub end: u128,
    pub packed: bool,
}

#[cfg(feature = "parallel")]
#[allow(clippy::too_many_arguments)]
pub(super) fn batch(
    intervals: impl Iterator<Item = BlockInterval>,
    joined_ns: u128,
    scratch: usize,
    results: usize,
    collectors: usize,
    metadata: usize,
) {
    if !enabled() {
        return;
    }
    let aggregation = Instant::now();
    update(|d| {
        let intervals = intervals.collect::<Vec<_>>();
        let mut endpoints = Vec::with_capacity(intervals.len() * 2);
        let mut first_start = joined_ns;
        let mut first_finish = joined_ns;
        let mut last_finish = 0;
        for interval in &intervals {
            d.blocks.add(interval.end - interval.start);
            d.active_block_ns += interval.end - interval.start;
            d.blocks_with_retained_packed_storage += u64::from(interval.packed);
            d.blocks_without_packed_storage += u64::from(!interval.packed);
            first_start = first_start.min(interval.start);
            first_finish = first_finish.min(interval.end);
            last_finish = last_finish.max(interval.end);
            if interval.end > interval.start {
                endpoints.push((interval.start, 1_i32));
                endpoints.push((interval.end, -1_i32));
            }
        }
        // End before start at ties: touching intervals do not overlap.
        endpoints.sort_unstable();
        let mut active = 0;
        let mut previous = 0;
        for &(at, delta) in &endpoints {
            if active > 0 {
                d.any_block_active_ns += at - previous;
            }
            active += delta;
            d.peak_active_blocks = d.peak_active_blocks.max(active as usize);
            previous = at;
        }
        assert_eq!(active, 0);
        if !intervals.is_empty() {
            d.batch_tails.add(last_finish - first_finish);
            d.dispatch_to_first_start_ns += first_start;
            d.last_finish_to_join_ns += joined_ns - last_finish;
            d.batch_dispatch_join_ns += joined_ns;
            d.batches += 1;
        }
        d.retained_scratch_bytes = d.retained_scratch_bytes.max(scratch);
        d.retained_result_capacity_bytes = d.retained_result_capacity_bytes.max(results);
        d.retained_collector_capacity_bytes = d.retained_collector_capacity_bytes.max(collectors);
        d.slot_metadata_bytes = d.slot_metadata_bytes.max(metadata);
        d.endpoint_capacity_bytes = d.endpoint_capacity_bytes.max(
            endpoints.capacity() * core::mem::size_of::<(u128, i32)>()
                + intervals.capacity() * core::mem::size_of::<BlockInterval>(),
        );
    });
    update(|d| d.aggregation_ns += aggregation.elapsed().as_nanos());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn front_end_observation_preserves_models_layouts_and_styles() {
        use super::super::*;
        let (width, height) = (17, 19);
        for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
            let bytes = usize::from(bits / 8);
            for interleaved in [false, true] {
                let step = bytes * if interleaved { components } else { 1 };
                let stride = width * step + 7;
                let span = height * stride;
                let mut raw = vec![0; if interleaved { span } else { span * components }];
                for c in 0..components {
                    for y in 0..height {
                        for x in 0..width {
                            let offset = if interleaved { c * bytes } else { c * span }
                                + y * stride
                                + x * step;
                            let value = ((x * 7919 + y * 3301 + c * 65521) as u16).to_le_bytes();
                            raw[offset..offset + bytes].copy_from_slice(&value[..bytes]);
                        }
                    }
                }
                let planes: Vec<_> = (0..components)
                    .map(|c| LosslessD2Plane {
                        samples: &raw[if interleaved { c * bytes } else { c * span }..],
                        stride_bytes: stride,
                        sample_step_bytes: step,
                    })
                    .collect();
                for bypass in [false, true] {
                    let call = || {
                        let encode = if bypass {
                            encode_lossless_d2_bypass
                        } else {
                            encode_lossless_d2
                        };
                        encode(
                            width as u32,
                            height as u32,
                            bits,
                            &planes,
                            LosslessEncodeLimits::default(),
                        )
                    };
                    let plain = call().unwrap();
                    let (observed, d) = observe_lossless_encode(call);
                    assert_eq!(observed.unwrap(), plain);
                    assert!(
                        d.conversion_level_shift_ns + d.forward_rct_ns
                            <= d.timings.conversion_level_shift_rct_ns
                    );
                    if components != 3 {
                        assert_eq!(d.forward_rct_ns, 0);
                    }
                    let detailed_dwt = d.dwt_scratch_resize_ns
                        + d.dwt_scratch_drop_ns
                        + d.dwt_validation_ns
                        + d.dwt_vertical_gather_ns
                        + d.dwt_vertical_lifting_ns
                        + d.dwt_vertical_store_ns
                        + d.dwt_horizontal_lifting_ns
                        + d.dwt_horizontal_copy_ns;
                    assert!(detailed_dwt <= d.timings.forward_dwt_ns);
                    assert!(d.dwt_vertical_gather_ns > 0);
                    assert!(d.dwt_vertical_lifting_ns > 0);
                    assert!(d.dwt_horizontal_lifting_ns > 0);
                }
            }
        }
    }

    #[test]
    fn fixed_distribution_preserves_counts_and_bounds() {
        let mut d = Durations::default();
        for ns in [0, 1, 2, 3, 4, 7, 8] {
            d.add(ns);
        }
        assert_eq!(&d.bins[..5], &[1, 1, 2, 2, 1]);
        assert_eq!((d.count, d.sum_ns, d.min_ns, d.max_ns), (7, 25, 0, 8));
    }
    #[cfg(feature = "parallel")]
    #[test]
    fn activity_uses_overlap_and_integral_not_participants() {
        let (_, d) = observe_lossless_encode(|| {
            batch(
                [
                    BlockInterval {
                        start: 2,
                        end: 12,
                        packed: true,
                    },
                    BlockInterval {
                        start: 3,
                        end: 5,
                        packed: true,
                    },
                    BlockInterval {
                        start: 5,
                        end: 8,
                        packed: true,
                    },
                ]
                .into_iter(),
                15,
                30,
                60,
                90,
                10,
            );
        });
        assert_eq!(d.peak_active_blocks, 2);
        assert_eq!(d.active_block_ns, 15);
        assert_eq!(d.any_block_active_ns, 10);
        assert_eq!(d.batch_tails.sum_ns, 7);
        assert_eq!(d.dispatch_to_first_start_ns, 2);
        assert_eq!(d.last_finish_to_join_ns, 3);
        assert!(!enabled());
    }
    #[cfg(feature = "parallel")]
    #[test]
    fn facade_writer_observation_preserves_stream_and_serial_route() {
        use super::super::*;
        let raw: Vec<u8> = (0..129 * 131)
            .flat_map(|n| ((n * 7919 + n / 7) as u16).to_le_bytes())
            .collect();
        for workers in [1, 2, 4, 8] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| {
                    let planes = [LosslessD2Plane {
                        samples: &raw,
                        stride_bytes: 129 * 2,
                        sample_step_bytes: 2,
                    }];
                    for bypass in [false, true] {
                        let call = || {
                            if bypass {
                                encode_lossless_d2_bypass(
                                    129,
                                    131,
                                    16,
                                    &planes,
                                    LosslessEncodeLimits::default(),
                                )
                            } else {
                                encode_lossless_d2(
                                    129,
                                    131,
                                    16,
                                    &planes,
                                    LosslessEncodeLimits::default(),
                                )
                            }
                        };
                        let plain = call().unwrap();
                        let (observed, d) = observe_lossless_encode(call);
                        assert_eq!(observed.unwrap(), plain);
                        assert_eq!(d.blocks.count, d.timings.checked_tier1_blocks);
                        assert_eq!(d.execution.effective_workers, workers);
                        assert!(d.peak_active_blocks > 0 && d.peak_active_blocks <= workers);
                        assert!(d.active_block_ns >= d.any_block_active_ns);
                        assert!(d.retained_scratch_bytes > 0);
                        assert!(d.blocks_with_retained_packed_storage > 0);
                        if workers == 1 {
                            assert_eq!(d.batches, 0);
                            assert_eq!(d.execution.max_batch_blocks, 1);
                            assert_eq!(d.peak_active_blocks, 1);
                        } else {
                            assert!(d.batches > 0);
                            assert_eq!(d.batch_tails.count, d.batches);
                        }
                    }
                });
        }
    }
    #[test]
    fn observer_releases_after_unwind() {
        assert!(
            std::panic::catch_unwind(|| observe_lossless_encode(|| panic!("authored fault")))
                .is_err()
        );
        assert!(!enabled());
        let (_, d) = observe_lossless_encode(|| ());
        assert_eq!(d.blocks.count, 0);
    }
}
