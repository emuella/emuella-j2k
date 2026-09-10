#![cfg(feature = "parallel")]
use emuella_j2k_codestream as cs;
use emuella_j2k_core::*;

fn pool(workers: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build()
        .unwrap()
}

#[test]
fn ordered_bounded_batches_preserve_bytes_layouts_samples_and_budgets() {
    let pools: Vec<_> = [1, 2, 4, 8].into_iter().map(pool).collect();
    for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
        let (width, height) = (513, 515);
        let bytes = usize::from(bits / 8);
        let mut rng = 0x713b9d21u32;
        let packed: Vec<u8> = (0..width as usize * height as usize * components as usize)
            .flat_map(|i| {
                rng ^= rng << 13;
                rng ^= rng >> 17;
                rng ^= rng << 5;
                // Dense first stripes followed by zero/constant stripes make
                // neighbouring blocks finish at materially different times.
                let value = if (i / components as usize / 64).is_multiple_of(3) {
                    rng as u16
                } else {
                    32768
                };
                value.to_le_bytes()[..bytes].to_vec()
            })
            .collect();
        let mut reference = None;
        for planar in [false, true] {
            let storage: Vec<Vec<u8>> = (0..components as usize)
                .map(|c| {
                    packed
                        .chunks_exact(components as usize * bytes)
                        .flat_map(|p| p[c * bytes..(c + 1) * bytes].iter().copied())
                        .collect()
                })
                .collect();
            let planes: Vec<_> = (0..components as usize)
                .map(|c| cs::LosslessD2Plane {
                    samples: if planar {
                        &storage[c]
                    } else {
                        &packed[c * bytes..]
                    },
                    stride_bytes: width as usize
                        * bytes
                        * if planar { 1 } else { components as usize },
                    sample_step_bytes: bytes * if planar { 1 } else { components as usize },
                })
                .collect();
            let limits = cs::LosslessEncodeLimits {
                max_working_bytes: 256 << 20,
                max_output_bytes: 16 << 20,
            };
            let serial = pools[0].install(|| {
                cs::lossless_d2_requirements(width, height, components, limits).unwrap()
            });
            for (index, pool) in pools.iter().enumerate() {
                pool.install(|| {
                    let expected_workers = [1, 2, 4, 8][index];
                    let requirements =
                        cs::lossless_d2_requirements(width, height, components, limits).unwrap();
                    assert_eq!(
                        requirements.working_bytes,
                        serial.working_bytes + (expected_workers - 1) as u64 * (4 << 20)
                    );
                    let (stream, timings, execution) = cs::encode_lossless_d2_execution_profiled(
                        width, height, bits, &planes, limits,
                    )
                    .unwrap();
                    assert_eq!(execution.effective_workers, expected_workers);
                    assert!(
                        execution.participating_workers >= 1
                            && execution.participating_workers <= expected_workers
                    );
                    assert!(execution.max_batch_blocks <= expected_workers);
                    assert_eq!(timings.checked_tier1_blocks, requirements.code_blocks);
                    assert_eq!(
                        timings.tier1_coefficients,
                        requirements.total_component_samples
                    );
                    assert_eq!(
                        stream,
                        cs::encode_lossless_d2(width, height, bits, &planes, limits).unwrap()
                    );
                    if let Some(ref bytes) = reference {
                        assert_eq!(&stream, bytes);
                    } else {
                        reference = Some(stream.clone());
                    }
                    let decoded = decode(
                        &stream,
                        &DecodeOptions {
                            mode: DecodeMode::Components,
                            target_layout: ComponentLayout::Interleaved,
                            ..Default::default()
                        },
                    )
                    .unwrap();
                    assert_eq!(decoded.data, ImageData::Interleaved(packed.clone()));
                    // Memory pressure reduces concurrency without rejecting a
                    // geometry that fits the previous serial working bound.
                    for admitted in [1, 2] {
                        let tight = cs::LosslessEncodeLimits {
                            max_working_bytes: serial.working_bytes + (admitted - 1) * (4 << 20),
                            ..limits
                        };
                        let (limited, _, t) = cs::encode_lossless_d2_execution_profiled(
                            width, height, bits, &planes, tight,
                        )
                        .unwrap();
                        assert_eq!(limited, stream);
                        assert_eq!(t.effective_workers, expected_workers.min(admitted as usize));
                    }
                    assert!(
                        cs::encode_lossless_d2(
                            width,
                            height,
                            bits,
                            &planes,
                            cs::LosslessEncodeLimits {
                                max_working_bytes: serial.working_bytes - 1,
                                ..limits
                            }
                        )
                        .is_err()
                    );
                    // Output capacity fails after joined entropy work; owned
                    // publication cannot expose a prefix from the private vector.
                    assert!(
                        cs::encode_lossless_d2(
                            width,
                            height,
                            bits,
                            &planes,
                            cs::LosslessEncodeLimits {
                                max_output_bytes: 128,
                                ..limits
                            }
                        )
                        .is_err()
                    );
                });
            }
        }
    }
}
