//! Project-authored positional native U16 qualification; no spectral assumptions.
use emuella_j2k_codestream as codestream;
use emuella_j2k_core::*;

fn options() -> EncodeOptions {
    EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    }
}
fn info(w: u32, h: u32, layout: ComponentLayout) -> ImageInfo {
    ImageInfo::new(w, h, 8, SampleFormat::U16_LE, ColorModel::Unknown, layout).unwrap()
}
fn decode_options(layout: ComponentLayout) -> DecodeOptions {
    DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: layout,
        ..Default::default()
    }
}
fn samples(w: u32, h: u32) -> Vec<u8> {
    (0..w * h)
        .flat_map(|pixel| {
            (0..8_u16).flat_map(move |band| {
                // Each band sees both endpoints and asymmetric bytes at distinct positions.
                let value = match (pixel + u32::from(band)) % 11 {
                    0 => 0,
                    1 => 65535,
                    2 => 0x0102,
                    3 => 0xff00,
                    _ => (pixel as u16)
                        .wrapping_mul(257)
                        .wrapping_add(band.wrapping_mul(7919)),
                };
                value.to_le_bytes()
            })
        })
        .collect()
}
fn encoded(w: u32, h: u32) -> Vec<u8> {
    encode(
        ImageView::Interleaved {
            info: &info(w, h, ComponentLayout::Interleaved),
            samples: &samples(w, h),
            stride_bytes: w as usize * 16,
        },
        &options(),
    )
    .unwrap()
}
fn check_header(bytes: &[u8], w: u32, h: u32) {
    let parsed = codestream::parse(bytes).unwrap();
    assert_eq!(
        (
            parsed.image_width(),
            parsed.image_height(),
            parsed.siz.component_count()
        ),
        (w, h, 8)
    );
    assert!(parsed.siz.components.iter().all(|c| !c.signed
        && c.bits_per_sample == 16
        && c.horizontal_separation == 1
        && c.vertical_separation == 1));
    assert_eq!(
        (
            parsed.siz.tile_width,
            parsed.siz.tile_height,
            parsed.tiles.len()
        ),
        (w, h, 1)
    );
    let style = parsed.uniform_effective_coding_style().unwrap();
    assert!(!style.multiple_component_transform);
    assert_eq!(style.decomposition_levels, 2);
    assert_eq!(style.transform, codestream::WaveletTransform::Reversible53);
    assert_eq!(style.progression_order, codestream::ProgressionOrder::Lrcp);
    assert_eq!(style.layers, 1);
    // SIZ: Lsiz62, Csiz8 and eight 16-bit unsigned/unit-sampled declarations.
    assert_eq!(&bytes[4..6], &62_u16.to_be_bytes());
    assert_eq!(&bytes[40..42], &8_u16.to_be_bytes());
    assert!(bytes[42..66].chunks_exact(3).all(|c| c == [15, 1, 1]));
}
fn check_target(bytes: &[u8], w: u32, h: u32, layout: ComponentLayout, fail: bool) {
    let metadata = info(w, h, layout);
    let expected = samples(w, h);
    let stride = w as usize
        * if layout == ComponentLayout::Planar {
            2
        } else {
            16
        }
        + 5;
    let mut storage = vec![
        vec![0xa5; stride * h as usize + 7];
        if layout == ComponentLayout::Planar {
            8
        } else {
            1
        }
    ];
    let result = if layout == ComponentLayout::Planar {
        let mut planes = storage
            .iter_mut()
            .map(|s| PlaneMut::new(s, w, h, stride, SampleFormat::U16_LE).unwrap())
            .collect::<Vec<_>>();
        decode_into(
            bytes,
            &mut ImageViewMut::Planar {
                info: &metadata,
                planes: &mut planes,
            },
            &decode_options(layout),
        )
    } else {
        decode_into(
            bytes,
            &mut ImageViewMut::Interleaved {
                info: &metadata,
                samples: &mut storage[0],
                stride_bytes: stride,
            },
            &decode_options(layout),
        )
    };
    assert_eq!(result.is_err(), fail, "{result:?}");
    for (band, plane) in storage.iter().enumerate() {
        for (offset, &actual) in plane.iter().enumerate() {
            let row = offset / stride;
            let x = offset % stride;
            let row_bytes = w as usize
                * if layout == ComponentLayout::Planar {
                    2
                } else {
                    16
                };
            let value = if fail || row >= h as usize || x >= row_bytes {
                0xa5
            } else if layout == ComponentLayout::Planar {
                expected[(row * w as usize + x / 2) * 16 + band * 2 + x % 2]
            } else {
                expected[row * w as usize * 16 + x]
            };
            assert_eq!(actual, value, "band={band}, offset={offset}");
        }
    }
}
#[test]
fn native_bands_roundtrip_padded_layouts_odd_and_processing_boundaries() {
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
        let raw = samples(w, h);
        let stream = encoded(w, h);
        check_header(&stream, w, h);
        for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
            let metadata = info(w, h, layout);
            let stride = w as usize
                * if layout == ComponentLayout::Planar {
                    2
                } else {
                    16
                }
                + 3;
            let mut storage = vec![
                vec![0x5a; stride * h as usize];
                if layout == ComponentLayout::Planar {
                    8
                } else {
                    1
                }
            ];
            for y in 0..h as usize {
                for x in 0..w as usize {
                    for band in 0..8 {
                        let source = (y * w as usize + x) * 16 + band * 2;
                        let (plane, destination) = if layout == ComponentLayout::Planar {
                            (band, y * stride + x * 2)
                        } else {
                            (0, y * stride + x * 16 + band * 2)
                        };
                        storage[plane][destination..destination + 2]
                            .copy_from_slice(&raw[source..source + 2]);
                    }
                }
            }
            let planes = storage
                .iter()
                .map(|s| Plane::new(s, w, h, stride, SampleFormat::U16_LE).unwrap())
                .collect::<Vec<_>>();
            let view = if layout == ComponentLayout::Planar {
                ImageView::Planar {
                    info: &metadata,
                    planes: &planes,
                }
            } else {
                ImageView::Interleaved {
                    info: &metadata,
                    samples: &storage[0],
                    stride_bytes: stride,
                }
            };
            assert_eq!(
                encode_with_limits(view, &options(), &LosslessEncodeLimits::default()).unwrap(),
                stream
            );
            let mut appended = vec![0xde, 0xad];
            encode_into(view, &mut appended, &options()).unwrap();
            assert_eq!(&appended[..2], &[0xde, 0xad]);
            assert_eq!(&appended[2..], stream);
            let decoded = decode(&stream, &decode_options(layout)).unwrap();
            assert_eq!(decoded.info, metadata);
            assert_eq!(
                decode_shape(&stream, &decode_options(layout))
                    .unwrap()
                    .output_components,
                8
            );
            for (band, component) in decoded.component_info.iter().enumerate() {
                assert_eq!(component.source_component, Some(band as u16));
                assert_eq!((component.width, component.height), (w, h));
            }
            check_target(&stream, w, h, layout, false);
        }
        assert!(decode(&stream, &DecodeOptions::default()).is_err());
    }
}
#[test]
fn rejects_unsupported_encode_contract_and_preserves_append_destination() {
    let base = info(4, 4, ComponentLayout::Interleaved);
    let raw = samples(4, 4);
    let mut bad_infos = Vec::new();
    for c in [0, 2, 4, 7, 9, u16::MAX] {
        let mut b = base.clone();
        b.components = c;
        bad_infos.push(b);
    }
    for format in [
        SampleFormat::U8,
        SampleFormat {
            bits_per_sample: 16,
            signed: true,
            byte_order: Some(SampleEndian::Little),
        },
        SampleFormat {
            bits_per_sample: 16,
            signed: false,
            byte_order: Some(SampleEndian::Big),
        },
        SampleFormat {
            bits_per_sample: 15,
            signed: false,
            byte_order: Some(SampleEndian::Little),
        },
    ] {
        let mut b = base.clone();
        b.sample_format = format;
        bad_infos.push(b);
    }
    for colour in [ColorModel::Rgb, ColorModel::Grayscale, ColorModel::Rgba] {
        let mut b = base.clone();
        b.color_model = colour;
        bad_infos.push(b);
    }
    for metadata in &bad_infos {
        assert!(
            lossless_encode_requirements(metadata, &options(), &LosslessEncodeLimits::default())
                .is_err()
        );
        let mut out = vec![1, 2, 3];
        assert!(
            encode_into(
                ImageView::Interleaved {
                    info: metadata,
                    samples: &raw,
                    stride_bytes: 64
                },
                &mut out,
                &options()
            )
            .is_err()
        );
        assert_eq!(out, [1, 2, 3]);
    }
    let mut bad_options = vec![
        EncodeOptions {
            format: OutputFormat::Jp2,
            ..options()
        },
        EncodeOptions {
            quality: EncodeQuality::TargetRate {
                bits_per_pixel: 1.0,
            },
            ..options()
        },
    ];
    for level in [0, 1, 3] {
        bad_options.push(EncodeOptions {
            decomposition_levels: level,
            ..options()
        });
    }
    bad_options.push(EncodeOptions {
        tile_size: Some(TileSize {
            width: 4,
            height: 4,
        }),
        ..options()
    });
    for o in bad_options {
        assert!(
            encode(
                ImageView::Interleaved {
                    info: &base,
                    samples: &raw,
                    stride_bytes: 64
                },
                &o
            )
            .is_err()
        );
    }
    let mut out = vec![1, 2, 3];
    assert!(
        encode_into(
            ImageView::Interleaved {
                info: &base,
                samples: &raw[..raw.len() - 1],
                stride_bytes: 64
            },
            &mut out,
            &options()
        )
        .is_err()
    );
    assert_eq!(out, [1, 2, 3]);
}
#[test]
fn checked_eight_component_resources_match_aggregate_decode_ceiling() {
    let limits = LosslessEncodeLimits {
        max_working_bytes: u64::MAX,
        max_output_bytes: 128,
    };
    let good = info(8192, 4096, ComponentLayout::Planar);
    let req = lossless_encode_requirements(&good, &options(), &limits).unwrap();
    assert_eq!(req.total_component_samples, 256 * 1024 * 1024);
    for (w, h) in [
        (8192, 4097),
        (32769, 4),
        (u32::MAX, u32::MAX),
        (3, 4),
        (4, 0),
    ] {
        let mut b = good.clone();
        b.width = w;
        b.height = h;
        assert!(lossless_encode_requirements(&b, &options(), &limits).is_err());
    }
    let small = info(65, 67, ComponentLayout::Interleaved);
    let req = lossless_encode_requirements(&small, &options(), &limits).unwrap();
    assert!(
        lossless_encode_requirements(
            &small,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: req.working_bytes,
                ..limits
            }
        )
        .is_ok()
    );
    // A tighter budget can reduce parallelism. Only the serial minimum is
    // a rejection boundary shared by every pool size.
    let minimum = req.total_component_samples * 4
        + req.code_blocks * 4096
        + u64::from(small.width.max(small.height)) * 12
        + limits.max_output_bytes * 2
        + (4 << 20);
    assert_eq!(
        lossless_encode_requirements(
            &small,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: minimum,
                ..limits
            }
        )
        .unwrap()
        .working_bytes,
        minimum
    );
    assert!(
        lossless_encode_requirements(
            &small,
            &options(),
            &LosslessEncodeLimits {
                max_working_bytes: minimum - 1,
                ..limits
            }
        )
        .is_err()
    );
    assert!(
        lossless_encode_requirements(
            &small,
            &options(),
            &LosslessEncodeLimits {
                max_output_bytes: u64::MAX,
                ..limits
            }
        )
        .is_err()
    );
    assert!(
        encode_with_limits(
            ImageView::Interleaved {
                info: &small,
                samples: &samples(65, 67),
                stride_bytes: 65 * 16
            },
            &options(),
            &limits
        )
        .is_err()
    );
}
#[test]
fn malformed_headers_and_late_payload_failures_are_atomic() {
    let stream = encoded(67, 65);
    let parsed = codestream::parse(&stream).unwrap();
    let cod = parsed
        .markers
        .iter()
        .find(|m| m.marker == codestream::Marker::Cod)
        .unwrap()
        .data_offset;
    let mut cases = Vec::new();
    for (offset, value) in [(41, 7), (42, 0xff), (43, 0), (cod + 4, 1)] {
        let mut b = stream.clone();
        b[offset] = value;
        cases.push(b);
    }
    // Keep shape and packet lengths inspectable, but damage the final band's
    // entropy contribution so failure happens after earlier bands reconstruct.
    let mut late = stream.clone();
    let end = late.len() - 2;
    late[end - 12..end].fill(0xff);
    assert!(decode(&late, &decode_options(ComponentLayout::Planar)).is_err());
    assert!(decode_shape(&late, &decode_options(ComponentLayout::Planar)).is_ok());
    cases.push(late);
    cases.push(stream[..stream.len() - 3].to_vec());
    for bad in cases {
        for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
            check_target(&bad, 67, 65, layout, true);
        }
    }
}

#[test]
fn rejects_inconsistent_planar_metadata_and_short_caller_storage() {
    let metadata = info(4, 4, ComponentLayout::Planar);
    let storage = vec![vec![0_u8; 32]; 8];
    let mut planes = storage
        .iter()
        .map(|p| Plane::new(p, 4, 4, 8, SampleFormat::U16_LE).unwrap())
        .collect::<Vec<_>>();
    planes[7].width = 3;
    assert!(
        encode(
            ImageView::Planar {
                info: &metadata,
                planes: &planes
            },
            &options()
        )
        .is_err()
    );
    planes[7].width = 4;
    planes[7].sample_format = SampleFormat::U8;
    assert!(
        encode(
            ImageView::Planar {
                info: &metadata,
                planes: &planes
            },
            &options()
        )
        .is_err()
    );
    planes[7].sample_format = SampleFormat::U16_LE;
    planes[7].stride_bytes = usize::MAX;
    assert!(
        encode(
            ImageView::Planar {
                info: &metadata,
                planes: &planes
            },
            &options()
        )
        .is_err()
    );
    assert!(
        encode(
            ImageView::Planar {
                info: &metadata,
                planes: &planes[..7]
            },
            &options()
        )
        .is_err()
    );
    let stream = encoded(4, 4);
    let metadata = info(4, 4, ComponentLayout::Interleaved);
    let mut destination = vec![0x6d; 255];
    assert!(
        decode_into(
            &stream,
            &mut ImageViewMut::Interleaved {
                info: &metadata,
                samples: &mut destination,
                stride_bytes: 64
            },
            &decode_options(ComponentLayout::Interleaved)
        )
        .is_err()
    );
    assert!(destination.iter().all(|b| *b == 0x6d));
    let metadata = info(4, 4, ComponentLayout::Planar);
    let mut destinations = vec![vec![0x6d; 32]; 8];
    destinations[7].truncate(31);
    let mut targets = destinations
        .iter_mut()
        .map(|samples| PlaneMut {
            samples,
            width: 4,
            height: 4,
            stride_bytes: 8,
            sample_format: SampleFormat::U16_LE,
        })
        .collect::<Vec<_>>();
    assert!(
        decode_into(
            &stream,
            &mut ImageViewMut::Planar {
                info: &metadata,
                planes: &mut targets
            },
            &decode_options(ComponentLayout::Planar)
        )
        .is_err()
    );
    assert!(destinations.iter().flatten().all(|b| *b == 0x6d));
    let borrowed = storage
        .iter()
        .map(|s| codestream::LosslessD2Plane {
            samples: s,
            stride_bytes: 8,
            sample_step_bytes: 2,
        })
        .collect::<Vec<_>>();
    assert!(
        codestream::encode_lossless_d2(4, 4, 8, &borrowed, LosslessEncodeLimits::default())
            .is_err()
    );
}
