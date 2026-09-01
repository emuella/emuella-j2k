//! Public API qualification using the same authored patterns as calibration.
use super::ht_lossy_tests::{assert_caller, options, sse};
use super::*;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::path::Path;

#[derive(Clone, Copy)]
struct Row {
    width: u32,
    height: u32,
    bits: u8,
    components: u16,
    pattern: u32,
}
impl Row {
    fn info(self, layout: ComponentLayout) -> ImageInfo {
        ImageInfo::new(
            self.width,
            self.height,
            self.components,
            if self.bits == 8 {
                SampleFormat::U8
            } else {
                SampleFormat::U16_LE
            },
            if self.components == 1 {
                ColorModel::Grayscale
            } else {
                ColorModel::Rgb
            },
            layout,
        )
        .unwrap()
    }
    fn expected_success(self, rate: u32) -> bool {
        match (self.width, self.height, self.pattern) {
            (4, 4, _) => false,
            (65, 65, 8) => self.components == 1 && rate == 1,
            (257, 193, 0..=3) | (1024, 1024, 1) | (8192, 128, 1) => true,
            (257, 193, 7) => rate == 1 || (self.bits == 16 && self.components == 3 && rate == 2),
            (257, 193, 4..=8) => false,
            _ => panic!("unselected public matrix row"),
        }
    }
}
fn rows(group: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for bits in [8, 16] {
        for components in [1, 3] {
            let patterns = match group {
                "main" => 0..4,
                "boundary" => 4..9,
                _ => 0..0,
            };
            for pattern in patterns {
                rows.push(Row {
                    width: 257,
                    height: 193,
                    bits,
                    components,
                    pattern,
                });
            }
            if group == "minimum" {
                rows.push(Row {
                    width: 4,
                    height: 4,
                    bits,
                    components,
                    pattern: 0,
                });
            }
        }
    }
    if group == "extreme" {
        for components in [1, 3] {
            rows.push(Row {
                width: 65,
                height: 65,
                bits: 16,
                components,
                pattern: 8,
            });
        }
    }
    if group == "resource" {
        for (width, height) in [(1024, 1024), (8192, 128)] {
            rows.push(Row {
                width,
                height,
                bits: 16,
                components: 3,
                pattern: 1,
            });
        }
    }
    rows
}
fn packed(image: &Image) -> Vec<u8> {
    match &image.data {
        ImageData::Interleaved(bytes) => bytes.clone(),
        ImageData::Planes(planes) => interleave_planes(
            planes,
            image.info.width,
            image.info.height,
            image.info.sample_format,
        )
        .unwrap(),
    }
}
fn padded(bytes: &[u8], row: usize, height: usize, padding: usize) -> Vec<u8> {
    // Deliberately omit padding after the final row: the last active byte is enough.
    let mut result = vec![0xa5; (row + padding) * (height - 1) + row];
    for y in 0..height {
        result[y * (row + padding)..y * (row + padding) + row]
            .copy_from_slice(&bytes[y * row..(y + 1) * row]);
    }
    result
}
fn peak(source: &[u8], native: &[u8], bits: u8) -> u16 {
    let value = |bytes: &[u8]| {
        if bits == 8 {
            u16::from(bytes[0])
        } else {
            u16::from_le_bytes([bytes[0], bytes[1]])
        }
    };
    source
        .chunks_exact(usize::from(bits / 8))
        .zip(native.chunks_exact(usize::from(bits / 8)))
        .map(|(a, b)| value(a).abs_diff(value(b)))
        .max()
        .unwrap()
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Default)]
struct Report {
    cells: usize,
    successes: usize,
    max_nmse: [f64; 3],
    max_undershoot: usize,
    manifest: String,
}
impl Report {
    fn new() -> Self {
        Self {
            manifest: "case\twidth\theight\tbits\tcomponents\tinput_layout\tpattern\trate\tdisposition\tbudget\ttolerance\traw_bytes\tjph_bytes\tsse\tdenominator\tnmse\tpeak\tsource_file\tsource_sha256\traw_file\traw_sha256\tjph_file\tjph_sha256\tnative_file\tnative_sha256\n".into(),
            ..Self::default()
        }
    }
    fn run(
        &mut self,
        group: &str,
        selected_rows: impl IntoIterator<Item = Row>,
        output: Option<&Path>,
    ) {
        for row in selected_rows {
            let source = codestream::ht_lossy_test_support::source(
                row.width,
                row.height,
                row.bits,
                row.components,
                row.pattern,
            );
            let planes = source
                .iter()
                .map(|p| {
                    p.iter()
                        .flat_map(|&v| {
                            if row.bits == 8 {
                                vec![v as u8]
                            } else {
                                v.to_le_bytes().to_vec()
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let canonical = interleave_planes(
                &planes,
                row.width,
                row.height,
                row.info(ComponentLayout::Planar).sample_format,
            )
            .unwrap();
            let source_hash = hash(&canonical);
            let plane_row = row.width as usize * usize::from(row.bits / 8);
            let plane_padding = 7;
            let padded_planes = planes
                .iter()
                .map(|p| padded(p, plane_row, row.height as usize, plane_padding))
                .collect::<Vec<_>>();
            let pixel_row = plane_row * usize::from(row.components);
            let pixel_padding = 11;
            let padded_pixels = padded(&canonical, pixel_row, row.height as usize, pixel_padding);
            let mut layout_baselines = Vec::new();
            for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
                let info = row.info(layout);
                // Public fields allow callers to provide a valid final-row extent
                // even though Plane::new currently requires final-row padding too.
                let views = padded_planes
                    .iter()
                    .map(|p| Plane {
                        samples: p,
                        width: row.width,
                        height: row.height,
                        stride_bytes: plane_row + plane_padding,
                        sample_format: info.sample_format,
                    })
                    .collect::<Vec<_>>();
                let view = match layout {
                    ComponentLayout::Planar => ImageView::Planar {
                        info: &info,
                        planes: &views,
                    },
                    ComponentLayout::Interleaved => ImageView::Interleaved {
                        info: &info,
                        samples: &padded_pixels,
                        stride_bytes: pixel_row + pixel_padding,
                    },
                };
                let mut previous = u128::MAX;
                for (rate_index, rate) in [1, 2, 4].into_iter().enumerate() {
                    let name = format!(
                        "w{}-h{}-b{}-c{}-p{}-{}-r{rate}",
                        row.width,
                        row.height,
                        row.bits,
                        row.components,
                        row.pattern,
                        if layout == ComponentLayout::Planar {
                            "planar"
                        } else {
                            "interleaved"
                        }
                    );
                    let opts = Htj2kLossyEncodeOptions {
                        bits_per_pixel: rate as f32,
                    };
                    let first = encode_htj2k_lossy(view, &opts);
                    assert_eq!(
                        first.is_ok(),
                        row.expected_success(rate),
                        "{name}: {first:?}"
                    );
                    let budget = row.width as usize * row.height as usize * rate as usize / 8;
                    let tolerance = 32.max(budget.div_ceil(500));
                    self.cells += 1;
                    assert_eq!(encode_htj2k_lossy(view, &opts), first, "repeat {name}");
                    let jph_result = encode_htj2k_lossy_jph(view, &opts);
                    assert_eq!(
                        encode_htj2k_lossy_jph(view, &opts),
                        jph_result,
                        "JPH repeat {name}"
                    );
                    let layout_name = if layout == ComponentLayout::Planar {
                        "planar"
                    } else {
                        "interleaved"
                    };
                    if let Err(error) = first {
                        assert_eq!(jph_result, Err(error.clone()), "{name}");
                        assert!(
                            format!("{error:?}").contains("unattainable"),
                            "{name}: {error:?}"
                        );
                        writeln!(self.manifest, "{name}\t{}\t{}\t{}\t{}\t{layout_name}\t{}\t{rate}\tunattainable\t{budget}\t{tolerance}\t-\t-\t-\t-\t-\t-\t-\t{source_hash}\t-\t-\t-\t-\t-\t-", row.width,row.height,row.bits,row.components,row.pattern).unwrap();
                        continue;
                    }
                    let raw = first.unwrap();
                    let jph = jph_result.unwrap();
                    assert!(
                        raw.len() <= budget && budget - raw.len() <= tolerance,
                        "{name}"
                    );
                    assert_eq!(jph.len(), raw.len() + 85);
                    assert_eq!(
                        container::parse(&jph)
                            .unwrap()
                            .primary_codestream(&jph)
                            .unwrap(),
                        Some(raw.as_slice())
                    );
                    if layout == ComponentLayout::Planar {
                        layout_baselines.push((rate, raw.clone()));
                    } else {
                        assert_eq!(
                            raw,
                            layout_baselines.iter().find(|(r, _)| *r == rate).unwrap().1,
                            "cross-layout {name}"
                        );
                    }
                    let native = decode(&raw, &options(ComponentLayout::Planar)).unwrap();
                    let canonical_native = packed(&native);
                    assert_eq!(canonical_native.len(), canonical.len());
                    assert_eq!(
                        (
                            native.info.width,
                            native.info.height,
                            native.info.components,
                            native.info.sample_format
                        ),
                        (row.width, row.height, row.components, info.sample_format)
                    );
                    assert_eq!(native.component_info.len(), usize::from(row.components));
                    let error = sse(&planes, &native, row.bits);
                    let denominator = u128::from(row.width)
                        * u128::from(row.height)
                        * u128::from(row.components)
                        * ((1_u128 << row.bits) - 1).pow(2);
                    let nmse = error as f64 / denominator as f64;
                    assert!(
                        error <= previous,
                        "distortion reversal {name}: {error} > {previous}"
                    );
                    previous = error;
                    if group == "main" {
                        assert!(
                            error * 1000 <= denominator * [125, 60, 40][rate_index],
                            "NMSE {name}: {nmse}"
                        );
                        self.max_nmse[rate_index] = self.max_nmse[rate_index].max(nmse);
                    }
                    for input in [&raw, &jph] {
                        assert_eq!(
                            inspect(input, &InspectOptions::default()).unwrap().support,
                            SupportStatus::Supported
                        );
                        for output_layout in [ComponentLayout::Planar, ComponentLayout::Interleaved]
                        {
                            let decode_options = options(output_layout);
                            let image = decode(input, &decode_options).unwrap();
                            assert_eq!(
                                packed(&image),
                                canonical_native,
                                "native {name} {output_layout:?}"
                            );
                            assert_eq!(
                                decode_shape(input, &decode_options)
                                    .unwrap()
                                    .image_info()
                                    .unwrap(),
                                image.info
                            );
                            assert_eq!(
                                decode_htj2k_with_workspace(
                                    input,
                                    &decode_options,
                                    &mut Htj2kDecodeWorkspace::new()
                                )
                                .unwrap(),
                                Some(image.clone())
                            );
                            assert_caller(input, &image, false);
                            // An early truncation failure must leave all active and
                            // padding bytes untouched for every selected success.
                            assert_caller(&input[..input.len() - 1], &image, true);
                        }
                    }
                    self.successes += 1;
                    self.max_undershoot = self.max_undershoot.max(budget - raw.len());
                    let source_file = format!("{name}.source.bin");
                    let raw_file = format!("{name}.j2c");
                    let jph_file = format!("{name}.jph");
                    let native_file = format!("{name}.native.bin");
                    if let Some(output) = output {
                        for (file, bytes) in [
                            (&source_file, &canonical),
                            (&raw_file, &raw),
                            (&jph_file, &jph),
                            (&native_file, &canonical_native),
                        ] {
                            std::fs::write(output.join(file), bytes).unwrap();
                        }
                    }
                    writeln!(self.manifest, "{name}\t{}\t{}\t{}\t{}\t{layout_name}\t{}\t{rate}\tsuccess\t{budget}\t{tolerance}\t{}\t{}\t{error}\t{denominator}\t{nmse:.15}\t{}\t{source_file}\t{source_hash}\t{raw_file}\t{}\t{jph_file}\t{}\t{native_file}\t{}",row.width,row.height,row.bits,row.components,row.pattern,raw.len(),jph.len(),peak(&canonical,&canonical_native,row.bits),hash(&raw),hash(&jph),hash(&canonical_native)).unwrap();
                }
            }
        }
    }
    fn summary(&self) {
        println!(
            "public HT matrix: cells={} success={} unattainable={} max_main_nmse={:?} max_undershoot={}",
            self.cells,
            self.successes,
            self.cells - self.successes,
            self.max_nmse,
            self.max_undershoot
        );
    }
}
#[test]
fn lossy_ht_public_smoke() {
    let representative = Row {
        width: 257,
        height: 193,
        bits: 8,
        components: 1,
        pattern: 0,
    };
    let mut report = Report::new();
    report.run("main", [representative], None);
    report.summary();
    assert_eq!((report.cells, report.successes), (6, 6));
}

fn complete_matrix(output: Option<&Path>) -> Report {
    let mut report = Report::new();
    for (group, expected_cells, expected_successes) in [
        ("main", 96, 96),
        ("boundary", 120, 10),
        ("extreme", 12, 2),
        ("resource", 12, 12),
        ("minimum", 24, 0),
    ] {
        let cells_before = report.cells;
        let successes_before = report.successes;
        report.run(group, rows(group), output);
        assert_eq!(report.cells - cells_before, expected_cells);
        assert_eq!(report.successes - successes_before, expected_successes);
    }
    report.summary();
    assert_eq!((report.cells, report.successes), (264, 120));
    report
}

#[test]
#[ignore = "mandatory optimised qualification; invoked by scripts/check-lossy-ht-public-matrix.sh"]
fn lossy_ht_public_complete_matrix() {
    complete_matrix(None);
}

#[test]
#[ignore = "opt-in project-authored export; requires an empty directory outside source"]
fn lossy_ht_export_public_matrix() {
    let path = std::env::var_os("EMUELLA_HT_PUBLIC_OUTPUT").expect("set EMUELLA_HT_PUBLIC_OUTPUT");
    let output = Path::new(&path).canonicalize().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    assert!(
        !output.starts_with(&source),
        "exports must stay outside source"
    );
    assert!(
        output.is_dir() && std::fs::read_dir(&output).unwrap().next().is_none(),
        "export directory must exist and be empty"
    );
    let report = complete_matrix(Some(&output));
    std::fs::write(output.join("manifest.txt"), report.manifest).unwrap();
}

fn both_reject(view: ImageView<'_>, rate: f32) -> J2kError {
    let options = Htj2kLossyEncodeOptions {
        bits_per_pixel: rate,
    };
    let error = encode_htj2k_lossy(view, &options).unwrap_err();
    assert_eq!(encode_htj2k_lossy_jph(view, &options), Err(error.clone()));
    error
}
#[test]
fn lossy_ht_public_preflight_rejects_immediate_neighbours_before_conversion() {
    let row = Row {
        width: 257,
        height: 193,
        bits: 16,
        components: 3,
        pattern: 0,
    };
    for layout in [ComponentLayout::Planar, ComponentLayout::Interleaved] {
        // Missing backing storage makes it observable whether semantic checks run
        // before the adapter attempts to read, convert or allocate image samples.
        let rejected = |info: &ImageInfo, rate| {
            let empty_planes = [Plane {
                samples: &[],
                width: info.width,
                height: info.height,
                stride_bytes: 0,
                sample_format: info.sample_format,
            }; 3];
            let view = match layout {
                ComponentLayout::Planar => ImageView::Planar {
                    info,
                    planes: &empty_planes,
                },
                ComponentLayout::Interleaved => ImageView::Interleaved {
                    info,
                    samples: &[],
                    stride_bytes: 0,
                },
            };
            let error = both_reject(view, rate);
            assert!(!matches!(error, J2kError::BufferTooSmall { .. }));
            assert!(!format!("{error:?}").contains("stride"), "{error:?}");
        };
        for (width, height) in [
            (0, 4),
            (3, 4),
            (4, 3),
            (8193, 4),
            (4, 8193),
            (1024, 1025),
            (8192, 129),
            (u32::MAX, u32::MAX),
        ] {
            rejected(
                &ImageInfo {
                    width,
                    height,
                    ..row.info(layout)
                },
                1.0,
            );
        }
        for rate in [
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            0.0,
            -0.0,
            -1.0,
            f32::MIN_POSITIVE,
            f32::MAX,
        ] {
            rejected(&row.info(layout), rate);
        }
        for format in [
            SampleFormat {
                bits_per_sample: 7,
                signed: false,
                byte_order: None,
            },
            SampleFormat {
                bits_per_sample: 9,
                signed: false,
                byte_order: Some(SampleEndian::Little),
            },
            SampleFormat {
                bits_per_sample: 15,
                signed: false,
                byte_order: Some(SampleEndian::Little),
            },
            SampleFormat {
                bits_per_sample: 17,
                signed: false,
                byte_order: Some(SampleEndian::Little),
            },
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
                bits_per_sample: 8,
                signed: false,
                byte_order: Some(SampleEndian::Little),
            },
        ] {
            rejected(
                &ImageInfo {
                    sample_format: format,
                    ..row.info(layout)
                },
                1.0,
            );
        }
        for components in [0, 2, 4, u16::MAX] {
            rejected(
                &ImageInfo {
                    components,
                    ..row.info(layout)
                },
                1.0,
            );
        }
        for color_model in [
            ColorModel::Unknown,
            ColorModel::Grayscale,
            ColorModel::YCbCr,
            ColorModel::Rgba,
        ] {
            rejected(
                &ImageInfo {
                    color_model,
                    ..row.info(layout)
                },
                1.0,
            );
        }
    }
    // Exact accepted resource boundaries reach extent validation with no image
    // allocation. Their selected large-image success journeys run in the matrix.
    for (width, height) in [
        (4, 4),
        (8192, 4),
        (4, 8192),
        (1024, 1024),
        (8192, 128),
        (128, 8192),
    ] {
        let info = ImageInfo {
            width,
            height,
            ..row.info(ComponentLayout::Interleaved)
        };
        assert!(matches!(
            both_reject(
                ImageView::Interleaved {
                    info: &info,
                    samples: &[],
                    stride_bytes: width as usize * 6
                },
                1.0
            ),
            J2kError::BufferTooSmall { .. }
        ));
    }
    let info = row.info(ComponentLayout::Interleaved);
    let samples = vec![0; 257 * 193 * 6];
    for (len, stride) in [
        (samples.len(), 257 * 6 - 1),
        (samples.len() - 1, 257 * 6),
        (samples.len(), usize::MAX),
    ] {
        both_reject(
            ImageView::Interleaved {
                info: &info,
                samples: &samples[..len],
                stride_bytes: stride,
            },
            1.0,
        );
    }
    let wrong_layout = row.info(ComponentLayout::Planar);
    both_reject(
        ImageView::Interleaved {
            info: &wrong_layout,
            samples: &samples,
            stride_bytes: 257 * 6,
        },
        1.0,
    );
    let plane = Plane {
        samples: &samples[..257 * 193 * 2],
        width: 257,
        height: 193,
        stride_bytes: 257 * 2,
        sample_format: SampleFormat::U16_LE,
    };
    let info = row.info(ComponentLayout::Planar);
    for bad in [
        Plane {
            width: 256,
            ..plane
        },
        Plane {
            height: 192,
            ..plane
        },
        Plane {
            sample_format: SampleFormat::U8,
            ..plane
        },
        Plane {
            stride_bytes: 513,
            ..plane
        },
        Plane {
            stride_bytes: usize::MAX,
            ..plane
        },
        Plane {
            samples: &plane.samples[..plane.samples.len() - 1],
            ..plane
        },
    ] {
        both_reject(
            ImageView::Planar {
                info: &info,
                planes: &[plane, plane, bad],
            },
            1.0,
        );
    }
    both_reject(
        ImageView::Planar {
            info: &info,
            planes: &[plane, plane],
        },
        1.0,
    );
    both_reject(
        ImageView::Planar {
            info: &row.info(ComponentLayout::Interleaved),
            planes: &[plane; 3],
        },
        1.0,
    );
}

#[test]
fn lossy_ht_public_fractional_budget_uses_exact_f32_and_byte_floor() {
    let row = Row {
        width: 257,
        height: 193,
        bits: 8,
        components: 1,
        pattern: 0,
    };
    let info = row.info(ComponentLayout::Interleaved);
    let source = codestream::ht_lossy_test_support::source(257, 193, 8, 1, 0)[0]
        .iter()
        .map(|&v| v as u8)
        .collect::<Vec<_>>();
    for rate in [1.001_f32, 1.1, 2.003, 4.0001] {
        let budget = ((f64::from(rate) * 257.0 * 193.0).floor() / 8.0).floor() as usize;
        let opts = Htj2kLossyEncodeOptions {
            bits_per_pixel: rate,
        };
        let view = ImageView::Interleaved {
            info: &info,
            samples: &source,
            stride_bytes: 257,
        };
        let raw = encode_htj2k_lossy(view, &opts).unwrap();
        assert!(raw.len() <= budget && budget - raw.len() <= 32.max(budget.div_ceil(500)));
        assert_eq!(encode_htj2k_lossy(view, &opts).unwrap(), raw);
    }
    let info = ImageInfo {
        width: 4,
        height: 4,
        ..info
    };
    for rate in [f32::from_bits(0.5_f32.to_bits() - 1), 0.5] {
        let error = both_reject(
            ImageView::Interleaved {
                info: &info,
                samples: &source[..16],
                stride_bytes: 4,
            },
            rate,
        );
        assert_eq!(format!("{error:?}").contains("unattainable"), rate == 0.5);
    }
}
