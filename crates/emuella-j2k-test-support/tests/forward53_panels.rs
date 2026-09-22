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
                                if backend != cs::Forward53Backend::Reference {
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
                    assert_eq!(detail.forward53.logical_slots, 1);
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
