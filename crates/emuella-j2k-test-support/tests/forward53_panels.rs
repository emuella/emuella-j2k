#![cfg(feature = "classic-execution-diagnostics")]
use emuella_j2k_codestream as cs;
use emuella_j2k_core::*;

#[test]
fn all_backends_workers_styles_full_range_components_and_padding_have_exact_bytes() {
    let pools: Vec<_> = [1, 2, 4, 8]
        .map(|workers| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
        })
        .into();
    for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
        let (width, height) = (129usize, 131usize);
        let bytes = bits / 8;
        for pattern in 0..2 {
            let mut rng = 0x97128315u32;
            let packed: Vec<u8> = (0..width * height * components)
                .flat_map(|n| {
                    rng ^= rng << 13;
                    rng ^= rng >> 17;
                    rng ^= rng << 5;
                    let value: u16 = if pattern == 1 {
                        1 << (bits - 1)
                    } else {
                        match n % 11 {
                            0 => 0,
                            1 => u16::MAX,
                            2..=4 => 1 << (bits - 1),
                            _ => rng as u16,
                        }
                    };
                    value.to_le_bytes()[..bytes].to_vec()
                })
                .collect();
            // Planar source has padding and no final trailing row storage.
            let stride = width * bytes + 5;
            let planar: Vec<Vec<u8>> = (0..components)
                .map(|c| {
                    let mut p = vec![0xa5; (height - 1) * stride + width * bytes];
                    for y in 0..height {
                        for x in 0..width {
                            let source = ((y * width + x) * components + c) * bytes;
                            p[y * stride + x * bytes..y * stride + (x + 1) * bytes]
                                .copy_from_slice(&packed[source..source + bytes]);
                        }
                    }
                    p
                })
                .collect();
            let planes: Vec<_> = planar
                .iter()
                .map(|samples| cs::LosslessD2Plane {
                    samples,
                    stride_bytes: stride,
                    sample_step_bytes: bytes,
                })
                .collect();
            for bypass in [false, true] {
                let limits = cs::LosslessEncodeLimits {
                    max_working_bytes: 128 << 20,
                    max_output_bytes: 4 << 20,
                };
                let encode = |limits| {
                    if bypass {
                        cs::encode_lossless_d2_bypass(
                            width as u32,
                            height as u32,
                            bits as u8,
                            &planes,
                            limits,
                        )
                    } else {
                        cs::encode_lossless_d2(
                            width as u32,
                            height as u32,
                            bits as u8,
                            &planes,
                            limits,
                        )
                    }
                };
                let requirements = |limits| {
                    if bypass {
                        cs::lossless_d2_bypass_requirements(
                            width as u32,
                            height as u32,
                            components as u16,
                            limits,
                        )
                    } else {
                        cs::lossless_d2_requirements(
                            width as u32,
                            height as u32,
                            components as u16,
                            limits,
                        )
                    }
                };
                let reference = pools[0].install(|| {
                    cs::with_forward53_diagnostic_policy(
                        cs::Forward53Backend::Reference,
                        16,
                        || encode(limits).unwrap(),
                    )
                });
                let serial = pools[0].install(|| requirements(limits).unwrap());
                let decoded = decode(
                    &reference,
                    &DecodeOptions {
                        mode: DecodeMode::Components,
                        target_layout: ComponentLayout::Interleaved,
                        ..Default::default()
                    },
                )
                .unwrap();
                assert_eq!(decoded.data, ImageData::Interleaved(packed.clone()));
                for pool in &pools {
                    for backend in [
                        cs::Forward53Backend::Reference,
                        cs::Forward53Backend::RowPanelScalar,
                        cs::Forward53Backend::RowPanelParallel,
                    ] {
                        pool.install(|| {
                            cs::with_forward53_diagnostic_policy(backend, 16, || {
                                let (encoded, detail) =
                                    cs::observe_lossless_encode(|| encode(limits));
                                let encoded = encoded.unwrap();
                                assert_eq!(encoded, reference); // Includes exponents, block/packet metadata and payload.
                                let stepped: Vec<_> = (0..components)
                                    .map(|c| cs::LosslessD2Plane {
                                        samples: &packed[c * bytes..],
                                        stride_bytes: width * components * bytes,
                                        sample_step_bytes: components * bytes,
                                    })
                                    .collect();
                                let interleaved = if bypass {
                                    cs::encode_lossless_d2_bypass(
                                        width as u32,
                                        height as u32,
                                        bits as u8,
                                        &stepped,
                                        limits,
                                    )
                                } else {
                                    cs::encode_lossless_d2(
                                        width as u32,
                                        height as u32,
                                        bits as u8,
                                        &stepped,
                                        limits,
                                    )
                                }
                                .unwrap();
                                assert_eq!(interleaved, reference);
                                assert!(
                                    detail.execution.max_batch_blocks
                                        <= detail.execution.effective_workers
                                );
                                assert_eq!(
                                    requirements(limits).unwrap().working_bytes,
                                    serial.working_bytes
                                        + (detail.execution.effective_workers as u64 - 1)
                                            * (4 << 20)
                                );
                                if backend == cs::Forward53Backend::RowPanelScalar
                                    || (backend == cs::Forward53Backend::RowPanelParallel
                                        && detail.execution.effective_workers >= 2)
                                {
                                    assert!(detail.forward53.workspace_capacity_bytes > 0);
                                    assert!(
                                        detail.forward53.logical_slots
                                            <= detail.execution.effective_workers
                                    );
                                }
                            })
                        });
                    }
                }
                pools[3].install(|| {
                    let tight = cs::LosslessEncodeLimits {
                        max_working_bytes: serial.working_bytes,
                        ..limits
                    };
                    let (encoded, detail) = cs::observe_lossless_encode(|| encode(tight));
                    assert_eq!(encoded.unwrap(), reference);
                    assert_eq!(detail.execution.effective_workers, 1);
                    assert_eq!(detail.forward53.logical_slots, 0);
                    assert_eq!(detail.forward53.workspace_capacity_bytes, 0);
                    assert_eq!(detail.forward53_original_helper_calls, components);
                    assert!(detail.forward53_original_scratch_capacity_bytes >= 12 * height);
                    assert!(
                        encode(cs::LosslessEncodeLimits {
                            max_working_bytes: serial.working_bytes - 1,
                            ..limits
                        })
                        .is_err()
                    );
                    if reference.len() > 128 {
                        assert!(
                            encode(cs::LosslessEncodeLimits {
                                max_output_bytes: 128,
                                ..limits
                            })
                            .is_err()
                        );
                    }
                });
            }
        }
    }
}

#[test]
fn ordinary_dispatch_tracks_original_scratch_and_admitted_parallel_storage() {
    // Sequential calls vary geometry and effective budgets in the same pools.
    // Diagnostic observation adds no forcing; this exercises the ordinary rule.
    for requested in [1, 2, 4, 8] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(requested)
            .build()
            .unwrap();
        pool.install(|| {
            for (width, height) in [
                (129usize, 131usize),
                (16, 513),
                (65, 67),
                (7, 9),
                (257, 259),
            ] {
                let samples: Vec<u8> = (0..width * height).map(|n| (n * 71) as u8).collect();
                let planes = [cs::LosslessD2Plane {
                    samples: &samples,
                    stride_bytes: width,
                    sample_step_bytes: 1,
                }];
                let limits = cs::LosslessEncodeLimits {
                    max_working_bytes: 128 << 20,
                    max_output_bytes: 4 << 20,
                };
                let requirement =
                    cs::lossless_d2_requirements(width as u32, height as u32, 1, limits).unwrap();
                let available = cs::parallel_worker_count()
                    .unwrap_or(1)
                    .min(requirement.code_blocks as usize);
                let serial_bytes = requirement.working_bytes - (available as u64 - 1) * (4 << 20);
                let encode = |limits| {
                    cs::encode_lossless_d2(width as u32, height as u32, 8, &planes, limits)
                };
                let reference = cs::with_forward53_diagnostic_policy(
                    cs::Forward53Backend::Reference,
                    16,
                    || encode(limits).unwrap(),
                );
                for admitted in [1usize, 2, 4, 8] {
                    let tight = cs::LosslessEncodeLimits {
                        max_working_bytes: serial_bytes + (admitted as u64 - 1) * (4 << 20),
                        ..limits
                    };
                    let (result, detail) = cs::observe_lossless_encode(|| encode(tight));
                    assert_eq!(result.unwrap(), reference);
                    let effective = available.min(admitted);
                    assert_eq!(detail.execution.effective_workers, effective);
                    let panels = effective >= 2 && width > 16 && width * height >= 4096;
                    if panels {
                        assert_eq!(detail.forward53_original_helper_calls, 0);
                        assert_eq!(detail.forward53_original_scratch_capacity_bytes, 0);
                        assert_eq!(detail.forward53_panel_prepare_attempts, 1);
                        assert_eq!(
                            detail.forward53_panel_initialised_bytes,
                            detail.forward53.workspace_capacity_bytes
                        );
                        assert!(detail.forward53.logical_slots >= 2);
                        assert_eq!(
                            detail.forward53.level_backends[0],
                            Some(cs::Forward53Backend::RowPanelParallel)
                        );
                        let ll = if width.div_ceil(2) * height.div_ceil(2) >= 4096 {
                            cs::Forward53Backend::RowPanelParallel
                        } else {
                            cs::Forward53Backend::Reference
                        };
                        assert_eq!(detail.forward53.level_backends[1], Some(ll));
                    } else {
                        assert_eq!(detail.forward53_original_helper_calls, 1);
                        assert!(
                            detail.forward53_original_scratch_capacity_bytes
                                >= 12 * width.max(height)
                        );
                        assert_eq!(detail.forward53_panel_prepare_attempts, 0);
                        assert_eq!(detail.forward53_panel_initialised_bytes, 0);
                        assert_eq!(detail.forward53.workspace_capacity_bytes, 0);
                        assert_eq!(detail.forward53.logical_slots, 0);
                        assert_eq!(
                            detail.forward53.level_backends,
                            [Some(cs::Forward53Backend::Reference); 2]
                        );
                    }
                }
            }
        });
    }
}

#[test]
fn eight_component_u8_remains_outside_the_profile() {
    let samples = vec![0u8; 129 * 131];
    let planes = [cs::LosslessD2Plane {
        samples: &samples,
        stride_bytes: 129,
        sample_step_bytes: 1,
    }; 8];
    assert!(
        cs::encode_lossless_d2(129, 131, 8, &planes, cs::LosslessEncodeLimits::default()).is_err()
    );
    assert!(
        cs::encode_lossless_d2_bypass(129, 131, 8, &planes, cs::LosslessEncodeLimits::default())
            .is_err()
    );
}
