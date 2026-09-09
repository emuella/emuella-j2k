use emuella_j2k_codestream as cs;
use emuella_j2k_core::*;
use emuella_j2k_test_support::lossless_diagnostics::decode_native_profiled;

#[test]
fn stage_accounting_and_native_parity() {
    for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
        for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
            let (width, height) = (67, 65);
            let bytes = usize::from(bits / 8);
            let n = width as usize * height as usize;
            let mut rng = 0x7193c621_u32;
            let packed: Vec<u8> = (0..n * components as usize)
                .flat_map(|i| {
                    rng ^= rng << 13;
                    rng ^= rng >> 17;
                    rng ^= rng << 5;
                    let word = match i % 17 {
                        0 => 0,
                        1 => 65535,
                        _ => rng as u16,
                    };
                    word.to_le_bytes()[..bytes].to_vec()
                })
                .collect();
            let planar: Vec<Vec<u8>> = (0..components as usize)
                .map(|c| {
                    packed
                        .chunks_exact(components as usize * bytes)
                        .flat_map(|pixel| pixel[c * bytes..(c + 1) * bytes].iter().copied())
                        .collect()
                })
                .collect();
            let info = ImageInfo::new(
                width,
                height,
                components,
                if bits == 8 {
                    SampleFormat::U8
                } else {
                    SampleFormat::U16_LE
                },
                match components {
                    1 => ColorModel::Grayscale,
                    3 => ColorModel::Rgb,
                    _ => ColorModel::Unknown,
                },
                layout,
            )
            .unwrap();
            let core_planes: Vec<_> = planar
                .iter()
                .map(|p| {
                    Plane::new(p, width, height, width as usize * bytes, info.sample_format)
                        .unwrap()
                })
                .collect();
            let view = if layout == ComponentLayout::Planar {
                ImageView::Planar {
                    info: &info,
                    planes: &core_planes,
                }
            } else {
                ImageView::Interleaved {
                    info: &info,
                    samples: &packed,
                    stride_bytes: width as usize * components as usize * bytes,
                }
            };
            let planes: Vec<_> = (0..components as usize)
                .map(|c| cs::LosslessD2Plane {
                    samples: if layout == ComponentLayout::Planar {
                        &planar[c]
                    } else {
                        &packed[c * bytes..]
                    },
                    stride_bytes: width as usize
                        * bytes
                        * if layout == ComponentLayout::Planar {
                            1
                        } else {
                            components as usize
                        },
                    sample_step_bytes: bytes
                        * if layout == ComponentLayout::Planar {
                            1
                        } else {
                            components as usize
                        },
                })
                .collect();
            let options = EncodeOptions {
                format: OutputFormat::J2kCodestream,
                decomposition_levels: 2,
                ..Default::default()
            };
            let limits = LosslessEncodeLimits::default();
            let (stream, e) =
                cs::encode_lossless_d2_profiled(width, height, bits, &planes, limits).unwrap();
            assert_eq!(stream, encode(view, &options).unwrap());
            assert_eq!(
                stream,
                cs::encode_lossless_d2(width, height, bits, &planes, limits).unwrap()
            );
            let requirements =
                cs::lossless_d2_requirements(width, height, components, limits).unwrap();
            assert_eq!(e.checked_tier1_blocks, requirements.code_blocks);
            assert_eq!(e.tier1_coefficients, requirements.total_component_samples);
            assert_eq!(e.component_samples, requirements.total_component_samples);
            assert_eq!(e.rct_pixels, if components == 3 { n as u64 } else { 0 });
            assert_eq!(e.packets, u64::from(components) * 3);
            assert_eq!(e.packet_body_bytes_moved, e.tier1_codeword_bytes);
            assert!(e.included_tier1_blocks > 0 && e.tier1_coding_passes > 0);
            assert!(
                e.total_ns
                    >= e.conversion_level_shift_rct_ns
                        + e.forward_dwt_ns
                        + e.block_preparation_ns
                        + e.tier1_ns
                        + e.packet_headers_ns
                        + e.assembly_ns
            );
            let expected = if layout == ComponentLayout::Planar {
                ImageData::Planes(planar)
            } else {
                ImageData::Interleaved(packed)
            };
            for counters in [false, true] {
                let native = decode_native_profiled(&stream, layout, counters).unwrap();
                assert_eq!(native.components, components as usize);
                assert_eq!(native.data, expected);
                let d = native.timings;
                let routes = d.tier1_routes;
                assert!(
                    routes.executed_checked.blocks
                        + routes.executed_packed_dense.blocks
                        + routes.executed_packed_sparse.blocks
                        > 0
                );
                assert_eq!(
                    d.tier1_work_counters.cleanup_positions_visited > 0,
                    counters
                );
            }
            assert_eq!(
                decode(
                    &stream,
                    &DecodeOptions {
                        mode: DecodeMode::Components,
                        target_layout: layout,
                        ..Default::default()
                    }
                )
                .unwrap()
                .data,
                expected
            );
        }
    }
}

#[test]
fn errors_preserve_profile_and_resource_gates() {
    let samples = vec![0; 67 * 65 * 2];
    let plane = cs::LosslessD2Plane {
        samples: &samples,
        stride_bytes: 134,
        sample_step_bytes: 2,
    };
    let limits = LosslessEncodeLimits::default();
    for (width, bits, count) in [(3, 16, 1), (67, 12, 1), (67, 8, 8), (67, 16, 4)] {
        let planes = vec![plane; count];
        assert_eq!(
            cs::encode_lossless_d2_profiled(width, 65, bits, &planes, limits).unwrap_err(),
            cs::encode_lossless_d2(width, 65, bits, &planes, limits).unwrap_err()
        );
    }
    let short = cs::LosslessD2Plane {
        samples: &[],
        ..plane
    };
    assert!(cs::encode_lossless_d2_profiled(67, 65, 16, &[short], limits).is_err());
    let tiny = LosslessEncodeLimits {
        max_output_bytes: 128,
        ..limits
    };
    let noisy: Vec<_> = (0..67 * 65 * 2).map(|i| (i * 73 + i / 19) as u8).collect();
    assert!(
        cs::encode_lossless_d2_profiled(
            67,
            65,
            16,
            &[cs::LosslessD2Plane {
                samples: &noisy,
                ..plane
            }],
            tiny
        )
        .is_err()
    );
    assert!(
        cs::encode_lossless_d2_profiled(
            67,
            65,
            16,
            &[plane],
            LosslessEncodeLimits {
                max_working_bytes: 1,
                ..limits
            }
        )
        .is_err()
    );
    assert!(decode_native_profiled(&[0, 1, 2], ComponentLayout::Planar, true).is_err());
}

#[test]
fn no_mct_rgb_d2_uses_full_owned_diagnostics() {
    // Reinterpret the project's existing RCT coefficient stream as independent
    // components. The authored oracle is the forward RCT result plus level shift;
    // changing COD's MCT flag therefore requires no entropy-data modification.
    let mut samples = Vec::new();
    let mut expected = Vec::new();
    for y in 0..8_u16 {
        for x in 0..8_u16 {
            let red = 128 + x;
            let green = 128;
            let blue = 128 + y;
            samples.extend_from_slice(&[red as u8, green as u8, blue as u8]);
            expected.extend_from_slice(&[
                ((red + 2 * green + blue) / 4) as u8,
                (blue - green + 128) as u8,
                (red - green + 128) as u8,
            ]);
        }
    }
    let planes: Vec<_> = (0..3)
        .map(|c| cs::LosslessD2Plane {
            samples: &samples[c..],
            stride_bytes: 24,
            sample_step_bytes: 3,
        })
        .collect();
    let mut stream =
        cs::encode_lossless_d2(8, 8, 8, &planes, LosslessEncodeLimits::default()).unwrap();
    let cod = stream.windows(2).position(|w| w == [0xff, 0x52]).unwrap();
    assert_eq!(stream[cod + 8], 1);
    stream[cod + 8] = 0;
    assert!(
        !cs::parse(&stream)
            .unwrap()
            .coding_style
            .unwrap()
            .multiple_component_transform
    );
    let native = decode_native_profiled(&stream, ComponentLayout::Interleaved, true).unwrap();
    assert_eq!(native.data, ImageData::Interleaved(expected));
    assert_eq!(native.timings.inverse_rct_ns, 0);
    assert_eq!(
        decode(
            &stream,
            &DecodeOptions {
                mode: DecodeMode::Components,
                target_layout: ComponentLayout::Interleaved,
                ..Default::default()
            }
        )
        .unwrap()
        .data,
        native.data
    );
}
