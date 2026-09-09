use emuella_j2k_core::*;

fn options() -> EncodeOptions {
    EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    }
}
fn info(w: u32, h: u32, c: u16, bits: u8, layout: ComponentLayout) -> ImageInfo {
    ImageInfo::new(
        w,
        h,
        c,
        if bits == 8 {
            SampleFormat::U8
        } else {
            SampleFormat::U16_LE
        },
        if c == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        layout,
    )
    .unwrap()
}
fn input(w: u32, h: u32, c: u16, bits: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut rng = 0x713b9d21_u32;
    for i in 0..u64::from(w) * u64::from(h) * u64::from(c) {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        let value = match i % 7 {
            0 => 0,
            1 => u16::MAX,
            _ => rng as u16,
        };
        bytes.extend_from_slice(&value.to_le_bytes()[..usize::from(bits / 8)]);
    }
    bytes
}

#[test]
fn odd_dwt_and_code_block_boundaries_roundtrip_both_layouts_and_formats() {
    for (w, h) in [
        (4, 4),
        (63, 65),
        (64, 64),
        (65, 63),
        (127, 129),
        (128, 128),
        (129, 127),
        (255, 257),
    ] {
        for c in [1, 3] {
            for bits in [8, 16] {
                let input = input(w, h, c, bits);
                let bytes = usize::from(bits / 8);
                let interleaved = info(w, h, c, bits, ComponentLayout::Interleaved);
                let view = ImageView::Interleaved {
                    info: &interleaved,
                    samples: &input,
                    stride_bytes: w as usize * c as usize * bytes,
                };
                let encoded = encode(view, &options()).unwrap();
                let explicit =
                    encode_with_limits(view, &options(), &LosslessEncodeLimits::default()).unwrap();
                assert_eq!(encoded, explicit);
                let decoded = decode(
                    &encoded,
                    &DecodeOptions {
                        mode: DecodeMode::Components,
                        target_layout: ComponentLayout::Interleaved,
                        ..Default::default()
                    },
                )
                .unwrap();
                assert_eq!(decoded.data, ImageData::Interleaved(input.clone()));
                let planar = info(w, h, c, bits, ComponentLayout::Planar);
                let stride = w as usize * bytes + 3;
                let mut storage = vec![vec![0xa5; stride * h as usize]; c as usize];
                for y in 0..h as usize {
                    for x in 0..w as usize {
                        for (band, component) in storage.iter_mut().enumerate() {
                            let source = ((y * w as usize + x) * c as usize + band) * bytes;
                            component[y * stride + x * bytes..y * stride + (x + 1) * bytes]
                                .copy_from_slice(&input[source..source + bytes]);
                        }
                    }
                }
                let planes = storage
                    .iter()
                    .map(|s| Plane::new(s, w, h, stride, planar.sample_format).unwrap())
                    .collect::<Vec<_>>();
                let packed = encode_with_limits(
                    ImageView::Planar {
                        info: &planar,
                        planes: &planes,
                    },
                    &options(),
                    &LosslessEncodeLimits::default(),
                )
                .unwrap();
                assert_eq!(
                    packed, encoded,
                    "layout changed bytes at {w}x{h}, c={c}, bits={bits}"
                );
                let mut destination = vec![0xa5; 11];
                encode_into(view, &mut destination, &options()).unwrap();
                assert_eq!(&destination[..11], &[0xa5; 11]);
                assert_eq!(&destination[11..], encoded);
            }
        }
    }
}

#[test]
fn checked_requirements_reject_arithmetic_geometry_and_working_budgets() {
    let base = info(65, 67, 3, 16, ComponentLayout::Interleaved);
    let limits = LosslessEncodeLimits::default();
    let req = lossless_encode_requirements(&base, &options(), &limits).unwrap();
    let tight = LosslessEncodeLimits {
        max_working_bytes: req.working_bytes,
        max_output_bytes: limits.max_output_bytes,
    };
    assert_eq!(
        lossless_encode_requirements(&base, &options(), &tight).unwrap(),
        req
    );
    assert!(
        lossless_encode_requirements(
            &base,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: req.working_bytes - 1,
                ..tight
            }
        )
        .is_err()
    );
    for (w, h) in [
        (u32::MAX, u32::MAX),
        (0, 1),
        (3, 4),
        (32769, 4),
        (8193, 8193),
    ] {
        let mut bad = base.clone();
        bad.width = w;
        bad.height = h;
        assert!(lossless_encode_requirements(&bad, &options(), &limits).is_err());
    }
    assert!(
        lossless_encode_requirements(
            &base,
            &options(),
            &LosslessEncodeLimits {
                max_output_bytes: u64::MAX,
                ..limits
            }
        )
        .is_err()
    );
    for (w, h) in [(6650, 7054), (8001, 8003), (8192, 8192)] {
        assert!(
            lossless_encode_requirements(
                &info(w, h, 3, 16, ComponentLayout::Interleaved),
                &options(),
                &limits
            )
            .is_ok()
        );
    }
    // An invalid empty source with a deficient work allowance must report budget
    // admission first, without reading or allocating a coefficient plane.
    let error = encode_with_limits(
        ImageView::Interleaved {
            info: &base,
            samples: &[],
            stride_bytes: 0,
        },
        &options(),
        &LosslessEncodeLimits {
            max_working_bytes: 0,
            ..limits
        },
    )
    .unwrap_err();
    assert!(error.to_string().contains("working-memory budget"));
}

#[test]
fn output_limit_is_enforced_and_explicit_limits_never_fall_through() {
    let base = info(65, 67, 3, 16, ComponentLayout::Interleaved);
    let samples = input(65, 67, 3, 16);
    let view = ImageView::Interleaved {
        info: &base,
        samples: &samples,
        stride_bytes: 65 * 3 * 2,
    };
    let limits = LosslessEncodeLimits::default();
    let encoded = encode(view, &options()).unwrap();
    let tight = LosslessEncodeLimits {
        max_output_bytes: encoded.len() as u64,
        ..limits
    };
    let bounded = encode_with_limits(view, &options(), &tight).unwrap();
    assert_eq!(bounded, encoded);
    assert_eq!(bounded.capacity(), encoded.len());
    assert!(
        encode_with_limits(
            view,
            &options(),
            &LosslessEncodeLimits {
                max_output_bytes: encoded.len() as u64 - 1,
                ..limits
            }
        )
        .is_err()
    );
    assert!(
        encode_with_limits(
            view,
            &options(),
            &LosslessEncodeLimits {
                max_output_bytes: 0,
                ..limits
            }
        )
        .is_err()
    );
    let mut variants = vec![options(); 5];
    variants[0].format = OutputFormat::Jp2;
    variants[1].decomposition_levels = 0;
    variants[2].decomposition_levels = 1;
    variants[3].tile_size = Some(TileSize {
        width: 32,
        height: 32,
    });
    variants[4].quality = EncodeQuality::TargetRate {
        bits_per_pixel: 1.0,
    };
    for variant in variants {
        assert!(encode_with_limits(view, &variant, &limits).is_err());
    }
}

#[test]
fn planar_metadata_and_storage_are_checked_before_coding() {
    let base = info(65, 67, 1, 16, ComponentLayout::Planar);
    let samples = input(65, 67, 1, 16);
    for plane in [
        Plane {
            samples: &samples,
            width: 64,
            height: 67,
            stride_bytes: 130,
            sample_format: SampleFormat::U16_LE,
        },
        Plane {
            samples: &samples,
            width: 65,
            height: 67,
            stride_bytes: 130,
            sample_format: SampleFormat::U8,
        },
        Plane {
            samples: &samples[..10],
            width: 65,
            height: 67,
            stride_bytes: 130,
            sample_format: SampleFormat::U16_LE,
        },
        Plane {
            samples: &samples,
            width: 65,
            height: 67,
            stride_bytes: usize::MAX,
            sample_format: SampleFormat::U16_LE,
        },
    ] {
        assert!(
            encode_with_limits(
                ImageView::Planar {
                    info: &base,
                    planes: &[plane]
                },
                &options(),
                &LosslessEncodeLimits::default()
            )
            .is_err()
        );
    }
}

#[test]
fn previous_d2_bytes_and_tiny_precision_routes_are_preserved() {
    for (w, h) in [(1, 1), (2, 3), (3, 7), (67, 65)] {
        for c in [1, 3] {
            for bits in [8, 16] {
                let base = info(w, h, c, bits, ComponentLayout::Interleaved);
                let samples = input(w, h, c, bits);
                let stride = w as usize * c as usize * usize::from(bits / 8);
                let current = encode(
                    ImageView::Interleaved {
                        info: &base,
                        samples: &samples,
                        stride_bytes: stride,
                    },
                    &options(),
                );
                let old = match (c, bits) {
                    (1, 8) => {
                        codestream::encode_grayscale_u8_two_decomp(codestream::GrayscaleU8Encode {
                            width: w,
                            height: h,
                            samples: &samples,
                            stride_bytes: stride,
                        })
                    }
                    (1, 16) => codestream::encode_grayscale_u16_le_two_decomp(
                        codestream::GrayscaleU16LeEncode {
                            width: w,
                            height: h,
                            samples: &samples,
                            stride_bytes: stride,
                        },
                    ),
                    (3, 8) => codestream::encode_rgb_u8_two_decomp(codestream::RgbU8Encode {
                        width: w,
                        height: h,
                        samples: &samples,
                        stride_bytes: stride,
                    }),
                    (3, 16) => {
                        codestream::encode_rgb_u16_le_two_decomp(codestream::RgbU16LeEncode {
                            width: w,
                            height: h,
                            samples: &samples,
                            stride_bytes: stride,
                        })
                    }
                    _ => unreachable!(),
                };
                match (current, old) {
                    (Ok(a), Ok(b)) => assert_eq!(a, b),
                    (Err(_), Err(_)) => {}
                    _ => panic!("changed tiny or existing D2 acceptance"),
                }
            }
        }
    }
}

#[test]
fn old_shared_guards_still_reject_large_ht_and_d1_inputs() {
    let empty = codestream::GrayscaleU16LeEncode {
        width: 6650,
        height: 7054,
        samples: &[],
        stride_bytes: 6650 * 2,
    };
    for result in [
        codestream::encode_grayscale_u16_le_one_decomp(empty),
        codestream::encode_htj2k_grayscale_u16_le_no_decomp(empty),
        codestream::encode_grayscale_u16_le_two_decomp(empty),
    ] {
        assert!(result.unwrap_err().to_string().contains("16 Mi"));
    }
}
