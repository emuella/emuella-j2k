use super::*;

fn streams_match_original_columns() {
    for (width, height) in [(4, 5), (17, 19), (65, 67)] {
        for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
            let bytes = usize::from(bits / 8);
            // Disjoint planes, interleaved planes, identical slices, and partially
            // overlapping U16 samples are all legal immutable borrowed inputs.
            for layout in 0..4 {
                let step = if layout == 1 {
                    bytes * components
                } else {
                    bytes + 1
                };
                let stride = width * step + 7;
                let span = height * stride;
                let raw: Vec<u8> = (0..span * components + components * bytes)
                    .map(|n| (n * 193 + n / 7) as u8)
                    .collect();
                let before = raw.clone();
                let planes: Vec<_> = (0..components)
                    .map(|c| LosslessD2Plane {
                        samples: &raw[match layout {
                            0 => c * span,
                            1 => c * bytes,
                            2 => 0,
                            _ => c,
                        }..],
                        stride_bytes: stride,
                        sample_step_bytes: step,
                    })
                    .collect();
                for bypass in [false, true] {
                    let limits = LosslessEncodeLimits::default();
                    let run = |paired: bool| {
                        let mut timings = LosslessEncodeTimings::default();
                        let mut execution = LosslessEncodeExecution::default();
                        match (paired, bypass) {
                            (true, false) => encode_lossless_d2_impl::<false, false, true>(
                                width as u32,
                                height as u32,
                                bits,
                                &planes,
                                limits,
                                &mut timings,
                                &mut execution,
                            ),
                            (false, false) => encode_lossless_d2_impl::<false, false, false>(
                                width as u32,
                                height as u32,
                                bits,
                                &planes,
                                limits,
                                &mut timings,
                                &mut execution,
                            ),
                            (true, true) => encode_lossless_d2_impl::<false, true, true>(
                                width as u32,
                                height as u32,
                                bits,
                                &planes,
                                limits,
                                &mut timings,
                                &mut execution,
                            ),
                            (false, true) => encode_lossless_d2_impl::<false, true, false>(
                                width as u32,
                                height as u32,
                                bits,
                                &planes,
                                limits,
                                &mut timings,
                                &mut execution,
                            ),
                        }
                        .unwrap()
                    };
                    assert_eq!(
                        run(true),
                        run(false),
                        "{width}x{height} c={components} bits={bits} layout={layout} bypass={bypass}"
                    );
                    assert_eq!(raw, before);
                }
            }
        }
    }
}

#[test]
fn paired_columns_preserve_complete_streams_and_borrowed_input() {
    #[cfg(feature = "parallel")]
    for workers in [1, 8] {
        rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .unwrap()
            .install(streams_match_original_columns);
    }
    #[cfg(not(feature = "parallel"))]
    streams_match_original_columns();
}
