use emuella_j2k::*;

fn options() -> EncodeOptions {
    EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    }
}
fn info(width: u32, height: u32, components: u16, bits: u8, layout: ComponentLayout) -> ImageInfo {
    ImageInfo::new(
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
    .unwrap()
}
fn samples(width: u32, height: u32, components: u16, bits: u8) -> Vec<u8> {
    let mut state = 0x17b3e92au32;
    (0..width as usize * height as usize * components as usize)
        .flat_map(|i| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let value = if (i / components as usize / 64).is_multiple_of(3) {
                0u16
            } else {
                state as u16
            };
            value.to_le_bytes()[..usize::from(bits / 8)].to_vec()
        })
        .collect()
}
fn verify(stream: &[u8], raw: &[u8]) {
    let decoded = decode(
        stream,
        &DecodeOptions {
            mode: DecodeMode::Components,
            target_layout: ComponentLayout::Interleaved,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(decoded.data, ImageData::Interleaved(raw.to_vec()));
}

#[test]
fn explicit_bypass_retains_padded_layouts_and_default_entry_points() {
    for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
        for (width, height) in [(4, 4), (133, 129)] {
            let raw = samples(width, height, components, bits);
            let bytes = usize::from(bits / 8);
            let packed_info = info(
                width,
                height,
                components,
                bits,
                ComponentLayout::Interleaved,
            );
            let view = ImageView::Interleaved {
                info: &packed_info,
                samples: &raw,
                stride_bytes: width as usize * components as usize * bytes,
            };
            let limits = LosslessEncodeLimits::default();
            let stream = encode_lossless_bypass_with_limits(view, &options(), &limits).unwrap();
            verify(&stream, &raw);
            let default = encode_with_limits(view, &options(), &limits).unwrap();
            assert_eq!(default, encode(view, &options()).unwrap());
            assert_ne!(default, stream);
            let planar_info = info(width, height, components, bits, ComponentLayout::Planar);
            let stride = width as usize * bytes + 3;
            let mut storage = vec![vec![0xa5; stride * height as usize]; components as usize];
            for (c, plane) in storage.iter_mut().enumerate() {
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        let source = ((y * width as usize + x) * components as usize + c) * bytes;
                        plane[y * stride + x * bytes..y * stride + (x + 1) * bytes]
                            .copy_from_slice(&raw[source..source + bytes]);
                    }
                }
            }
            let planes: Vec<_> = storage
                .iter()
                .map(|p| Plane::new(p, width, height, stride, planar_info.sample_format).unwrap())
                .collect();
            let planar = ImageView::Planar {
                info: &planar_info,
                planes: &planes,
            };
            assert_eq!(
                stream,
                encode_lossless_bypass_with_limits(planar, &options(), &limits).unwrap()
            );
            // A rejected output allowance never mutates borrowed sample storage.
            let before = storage.clone();
            if stream.len() > 128 {
                assert!(
                    encode_lossless_bypass_with_limits(
                        planar,
                        &options(),
                        &LosslessEncodeLimits {
                            max_output_bytes: 128,
                            ..limits
                        }
                    )
                    .is_err()
                );
            }
            assert_eq!(storage, before);
        }
    }
}

#[test]
fn bypass_admission_adds_only_bounded_metadata_and_rejects_other_profiles() {
    let image = info(133, 129, 3, 16, ComponentLayout::Interleaved);
    let limits = LosslessEncodeLimits::default();
    let base = lossless_encode_requirements(&image, &options(), &limits).unwrap();
    let bypass = lossless_bypass_encode_requirements(&image, &options(), &limits).unwrap();
    assert_eq!(
        bypass.working_bytes,
        base.working_bytes + 512 * base.code_blocks
    );
    let minimum = bypass.total_component_samples * 4
        + bypass.code_blocks * (4096 + 512)
        + u64::from(image.width.max(image.height)) * 12
        + limits.max_output_bytes * 2
        + (4 << 20);
    assert!(
        lossless_bypass_encode_requirements(
            &image,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: minimum - 1,
                ..limits
            }
        )
        .is_err()
    );
    assert!(
        lossless_bypass_encode_requirements(
            &image,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: minimum,
                ..limits
            }
        )
        .is_ok()
    );
    for altered in [
        EncodeOptions {
            decomposition_levels: 1,
            ..options()
        },
        EncodeOptions {
            format: OutputFormat::Jp2,
            ..options()
        },
        EncodeOptions {
            tile_size: Some(TileSize {
                width: 64,
                height: 64,
            }),
            ..options()
        },
    ] {
        assert!(lossless_bypass_encode_requirements(&image, &altered, &limits).is_err());
    }
    for (w, h, c) in [(3, 129, 3), (133, 3, 3), (133, 129, 2), (32769, 4, 1)] {
        let mut bad = image.clone();
        bad.width = w;
        bad.height = h;
        bad.components = c;
        assert!(lossless_bypass_encode_requirements(&bad, &options(), &limits).is_err());
    }
}

#[cfg(feature = "parallel")]
#[test]
fn bypass_joined_worker_counts_and_tight_budgets_preserve_exact_bytes() {
    let pools: Vec<_> = [1, 2, 4, 8]
        .into_iter()
        .map(|workers| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
        })
        .collect();
    for (components, bits) in [(1, 8), (1, 16), (3, 8), (3, 16), (8, 16)] {
        let (width, height) = (513, 515);
        let raw = samples(width, height, components, bits);
        let image = info(
            width,
            height,
            components,
            bits,
            ComponentLayout::Interleaved,
        );
        let view = ImageView::Interleaved {
            info: &image,
            samples: &raw,
            stride_bytes: width as usize * components as usize * usize::from(bits / 8),
        };
        let limits = LosslessEncodeLimits {
            max_working_bytes: 256 << 20,
            max_output_bytes: 16 << 20,
        };
        let serial = pools[0]
            .install(|| lossless_bypass_encode_requirements(&image, &options(), &limits).unwrap());
        let expected = pools[0]
            .install(|| encode_lossless_bypass_with_limits(view, &options(), &limits).unwrap());
        verify(&expected, &raw);
        for (pool, workers) in pools.iter().zip([1u64, 2, 4, 8]) {
            pool.install(|| {
                let req = lossless_bypass_encode_requirements(&image, &options(), &limits).unwrap();
                assert_eq!(
                    req.working_bytes,
                    serial.working_bytes + (workers - 1) * (4 << 20)
                );
                assert_eq!(
                    expected,
                    encode_lossless_bypass_with_limits(view, &options(), &limits).unwrap()
                );
                for allowed in [1u64, 2] {
                    let tight = LosslessEncodeLimits {
                        max_working_bytes: serial.working_bytes + (allowed - 1) * (4 << 20),
                        ..limits
                    };
                    let req =
                        lossless_bypass_encode_requirements(&image, &options(), &tight).unwrap();
                    assert_eq!(
                        req.working_bytes,
                        serial.working_bytes + (allowed.min(workers) - 1) * (4 << 20)
                    );
                    assert_eq!(
                        expected,
                        encode_lossless_bypass_with_limits(view, &options(), &tight).unwrap()
                    );
                }
                assert!(
                    encode_lossless_bypass_with_limits(
                        view,
                        &options(),
                        &LosslessEncodeLimits {
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
