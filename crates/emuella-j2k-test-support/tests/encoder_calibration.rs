//! Project-authored lossless baselines and bounded irreversible rate-control
//! qualification.

use emuella_j2k_core::{
    ColorModel, ComponentLayout, DecodeMode, DecodeOptions, EncodeOptions, EncodeQuality,
    ImageData, ImageInfo, ImageView, OutputFormat, Plane, SampleFormat, WaveletTransform, decode,
    encode,
};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 48;
const QUALIFICATION_WIDTH: u32 = 257;
const QUALIFICATION_HEIGHT: u32 = 193;
const EXTREME_WIDTH: u32 = 65;
const EXTREME_HEIGHT: u32 = 65;

#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    components: u16,
    format: SampleFormat,
    colour: ColorModel,
}

fn source(case: Case) -> Vec<u8> {
    let samples = usize::try_from(WIDTH * HEIGHT).expect("calibration dimensions fit usize")
        * usize::from(case.components);
    match case.format.bits_per_sample {
        8 => (0..samples)
            .map(|index| {
                let x = index % WIDTH as usize;
                let y = index / WIDTH as usize / usize::from(case.components);
                let component = index % usize::from(case.components);
                ((x * 17 + y * 31 + component * 73 + x * y * 3) & 0xff) as u8
            })
            .collect(),
        16 => (0..samples)
            .flat_map(|index| {
                let x = index % WIDTH as usize;
                let y = index / WIDTH as usize / usize::from(case.components);
                let component = index % usize::from(case.components);
                let value =
                    ((x * 977 + y * 1_393 + component * 9_973 + x * y * 41) & 0xffff) as u16;
                value.to_le_bytes()
            })
            .collect(),
        _ => unreachable!("the calibration matrix only has byte-addressable samples"),
    }
}

fn qualification_source(case: Case) -> Vec<u8> {
    let mut output = Vec::with_capacity(
        usize::try_from(QUALIFICATION_WIDTH * QUALIFICATION_HEIGHT).unwrap()
            * usize::from(case.components)
            * usize::from(case.format.bits_per_sample / 8),
    );
    for y in 0..QUALIFICATION_HEIGHT {
        for x in 0..QUALIFICATION_WIDTH {
            for component in 0..u32::from(case.components) {
                let mixed = x
                    .wrapping_mul(977)
                    .wrapping_add(y.wrapping_mul(1_393))
                    .wrapping_add(component.wrapping_mul(9_973))
                    .wrapping_add(x.wrapping_mul(y).wrapping_mul(41))
                    .wrapping_add((x ^ y).wrapping_mul(271))
                    .wrapping_add((x / 7).wrapping_mul((y / 5) + 1).wrapping_mul(613));
                match case.format.bits_per_sample {
                    8 => output.push((mixed & 0xff) as u8),
                    16 => output.extend_from_slice(&((mixed & 0xffff) as u16).to_le_bytes()),
                    _ => unreachable!(),
                }
            }
        }
    }
    output
}

fn extreme_u16_source() -> Vec<u8> {
    let mut output = Vec::with_capacity((EXTREME_WIDTH * EXTREME_HEIGHT * 2) as usize);
    let positive = |index| !matches!(index, 1 | 2 | 4 | 5);
    for y in 0..EXTREME_HEIGHT {
        for x in 0..EXTREME_WIDTH {
            let value = if positive(x) == positive(y) {
                u16::MAX
            } else {
                0
            };
            output.extend_from_slice(&value.to_le_bytes());
        }
    }
    output
}

#[derive(Clone, Copy)]
struct SquaredError {
    total: u128,
    sample_count: usize,
    peak_code: u64,
}

fn qualification_cases() -> [Case; 4] {
    [
        Case {
            name: "grey-u8",
            components: 1,
            format: SampleFormat::U8,
            colour: ColorModel::Grayscale,
        },
        Case {
            name: "rgb-u8",
            components: 3,
            format: SampleFormat::U8,
            colour: ColorModel::Rgb,
        },
        Case {
            name: "grey-u16-le",
            components: 1,
            format: SampleFormat::U16_LE,
            colour: ColorModel::Grayscale,
        },
        Case {
            name: "rgb-u16-le",
            components: 3,
            format: SampleFormat::U16_LE,
            colour: ColorModel::Rgb,
        },
    ]
}

fn squared_error(source: &[u8], decoded: &[u8], format: SampleFormat) -> SquaredError {
    assert_eq!(source.len(), decoded.len());
    let total: u128 = match format.bits_per_sample {
        8 => source
            .iter()
            .zip(decoded)
            .map(|(&expected, &actual)| u128::from(expected.abs_diff(actual)).pow(2))
            .sum(),
        16 => source
            .chunks_exact(2)
            .zip(decoded.chunks_exact(2))
            .map(|(expected, actual)| {
                let expected = u16::from_le_bytes(expected.try_into().expect("two bytes"));
                let actual = u16::from_le_bytes(actual.try_into().expect("two bytes"));
                u128::from(expected.abs_diff(actual)).pow(2)
            })
            .sum(),
        _ => unreachable!("the calibration matrix only has byte-addressable samples"),
    };
    SquaredError {
        total,
        sample_count: source.len() / usize::from(format.bits_per_sample / 8),
        peak_code: (1_u64 << format.bits_per_sample) - 1,
    }
}

fn normalised_mean_squared_error_diagnostic(error: SquaredError) -> f64 {
    error.total as f64 / (error.sample_count as f64 * (error.peak_code as f64).powi(2))
}

fn assert_nmse_at_most(error: SquaredError, numerator: u128, denominator: u128, context: &str) {
    let peak_squared = u128::from(error.peak_code).pow(2);
    assert!(
        error.total * denominator <= error.sample_count as u128 * peak_squared * numerator,
        "{context}: NMSE exceeded {numerator}/{denominator}"
    );
}

#[test]
fn lossless_baselines_are_repeatable_and_define_zero_distortion() {
    for case in qualification_cases() {
        let source = source(case);
        let info = ImageInfo::new(
            WIDTH,
            HEIGHT,
            case.components,
            case.format,
            case.colour,
            ComponentLayout::Interleaved,
        )
        .expect("calibration image is valid");
        let image = ImageView::Interleaved {
            info: &info,
            samples: &source,
            stride_bytes: WIDTH as usize
                * usize::from(case.components)
                * usize::from(case.format.bits_per_sample / 8),
        };
        let options = EncodeOptions {
            format: OutputFormat::J2kCodestream,
            ..EncodeOptions::default()
        };
        let first = encode(image, &options).expect("lossless baseline encodes");
        let second = encode(image, &options).expect("lossless baseline re-encodes");
        assert_eq!(
            first,
            second,
            "{name} must be byte-repeatable",
            name = case.name
        );

        let decoded = decode(
            &first,
            &DecodeOptions {
                mode: DecodeMode::Components,
                target_layout: ComponentLayout::Interleaved,
                ..DecodeOptions::default()
            },
        )
        .expect("lossless baseline decodes");
        let ImageData::Interleaved(decoded) = decoded.data else {
            panic!(
                "{name} did not decode to an interleaved image",
                name = case.name
            );
        };
        assert_eq!(
            decoded,
            source,
            "{name} must remain lossless",
            name = case.name
        );
        let error = squared_error(&source, &decoded, case.format);
        assert_eq!(
            error.total,
            0,
            "{name} must have zero squared error",
            name = case.name
        );
        let bits_per_sample = (first.len() as f64 * 8.0)
            / (f64::from(WIDTH) * f64::from(HEIGHT) * f64::from(case.components));
        println!(
            "encoder-calibration case={} bytes={} bits-per-sample={bits_per_sample:.6}",
            case.name,
            first.len(),
        );
        println!(
            "encoder-calibration case={} nmse={:.12}",
            case.name,
            normalised_mean_squared_error_diagnostic(error),
        );
    }
}

#[test]
fn irreversible_target_rate_qualifies_bounded_matrix() {
    for case in qualification_cases() {
        let source = qualification_source(case);
        let info = ImageInfo::new(
            QUALIFICATION_WIDTH,
            QUALIFICATION_HEIGHT,
            case.components,
            case.format,
            case.colour,
            ComponentLayout::Interleaved,
        )
        .unwrap();
        let image = ImageView::Interleaved {
            info: &info,
            samples: &source,
            stride_bytes: QUALIFICATION_WIDTH as usize
                * usize::from(case.components)
                * usize::from(case.format.bits_per_sample / 8),
        };
        let mut previous_error: Option<SquaredError> = None;
        for (bits_per_pixel, budget) in [(1.0, 6_200), (2.0, 12_400), (4.0, 24_800)] {
            let raw_options = EncodeOptions {
                format: OutputFormat::J2kCodestream,
                transform: WaveletTransform::Irreversible97,
                quality: EncodeQuality::TargetRate { bits_per_pixel },
                decomposition_levels: 2,
                ..EncodeOptions::default()
            };
            let first = encode(image, &raw_options).expect("target-rate raw encode succeeds");
            let second = encode(image, &raw_options).expect("target-rate raw re-encode succeeds");
            assert_eq!(first, second, "{} {bits_per_pixel} bpp", case.name);
            assert!(first.len() <= budget, "{} {bits_per_pixel} bpp", case.name);
            let allowed_undershoot = 32_usize.max(budget.div_ceil(500));
            assert!(
                budget - first.len() <= allowed_undershoot,
                "{} {bits_per_pixel} bpp undershot by more than {allowed_undershoot} bytes",
                case.name
            );

            let decoded = decode(
                &first,
                &DecodeOptions {
                    mode: DecodeMode::Components,
                    target_layout: ComponentLayout::Interleaved,
                    ..DecodeOptions::default()
                },
            )
            .expect("target-rate raw codestream decodes");
            let ImageData::Interleaved(decoded) = decoded.data else {
                panic!("target-rate decode was not interleaved")
            };
            let error = squared_error(&source, &decoded, case.format);
            let ceiling_numerator = match bits_per_pixel as u8 {
                1 => 70_000,
                2 => 50_000,
                4 => 25_000,
                _ => unreachable!(),
            };
            assert_nmse_at_most(
                error,
                ceiling_numerator,
                1_000_000,
                &format!("{} {bits_per_pixel} bpp", case.name),
            );
            if let Some(previous) = previous_error {
                assert!(
                    error.total * (previous.sample_count as u128)
                        < previous.total * (error.sample_count as u128),
                    "{} NMSE must improve at {bits_per_pixel} bpp",
                    case.name
                );
            }
            previous_error = Some(error);

            let jp2 = encode(
                image,
                &EncodeOptions {
                    format: OutputFormat::Jp2,
                    ..raw_options.clone()
                },
            )
            .expect("target-rate JP2 encode succeeds");
            let jp2_decoded = decode(
                &jp2,
                &DecodeOptions {
                    mode: DecodeMode::Components,
                    target_layout: ComponentLayout::Interleaved,
                    ..DecodeOptions::default()
                },
            )
            .expect("target-rate JP2 decodes");
            assert_eq!(jp2_decoded.data, ImageData::Interleaved(decoded));
            assert_eq!(jp2.len() - first.len(), 85, "{} JP2 overhead", case.name);
            println!(
                "encoder-rate-control case={} target-bpp={bits_per_pixel:.1} budget={} raw-bytes={} undershoot={} nmse={:.12} peak-error={} jp2-bytes={} jp2-overhead={}",
                case.name,
                budget,
                first.len(),
                budget - first.len(),
                normalised_mean_squared_error_diagnostic(error),
                peak_error(&source, &jp2_decoded.data, case.format),
                jp2.len(),
                jp2.len() - first.len(),
            );
        }
    }
}

#[test]
fn irreversible_target_rate_fails_closed_outside_profile() {
    let case = qualification_cases()[0];
    let source = qualification_source(case);
    let info = ImageInfo::new(
        QUALIFICATION_WIDTH,
        QUALIFICATION_HEIGHT,
        case.components,
        case.format,
        case.colour,
        ComponentLayout::Interleaved,
    )
    .unwrap();
    let image = ImageView::Interleaved {
        info: &info,
        samples: &source,
        stride_bytes: QUALIFICATION_WIDTH as usize,
    };
    let options = |bits_per_pixel| EncodeOptions {
        format: OutputFormat::J2kCodestream,
        transform: WaveletTransform::Irreversible97,
        quality: EncodeQuality::TargetRate { bits_per_pixel },
        decomposition_levels: 2,
        ..EncodeOptions::default()
    };
    for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert!(encode(image, &options(invalid)).is_err());
    }
    assert!(encode(image, &options(0.001)).is_err());
    let unknown_info = ImageInfo::new(
        QUALIFICATION_WIDTH,
        QUALIFICATION_HEIGHT,
        1,
        SampleFormat::U8,
        ColorModel::Unknown,
        ComponentLayout::Interleaved,
    )
    .unwrap();
    assert!(
        encode(
            ImageView::Interleaved {
                info: &unknown_info,
                samples: &source,
                stride_bytes: QUALIFICATION_WIDTH as usize,
            },
            &options(2.0)
        )
        .is_err()
    );
    assert!(
        encode(
            image,
            &EncodeOptions {
                decomposition_levels: 1,
                ..options(2.0)
            }
        )
        .is_err()
    );
    assert!(
        encode(
            image,
            &EncodeOptions {
                transform: WaveletTransform::Reversible53,
                ..options(2.0)
            }
        )
        .is_err()
    );
    assert!(
        encode(
            image,
            &EncodeOptions {
                transform: WaveletTransform::Irreversible97,
                quality: EncodeQuality::Lossless,
                ..EncodeOptions::default()
            }
        )
        .is_err()
    );
}

#[test]
fn extreme_u16_target_rate_only_returns_classic_decodable_outputs() {
    let source = extreme_u16_source();
    let info = ImageInfo::new(
        EXTREME_WIDTH,
        EXTREME_HEIGHT,
        1,
        SampleFormat::U16_LE,
        ColorModel::Grayscale,
        ComponentLayout::Interleaved,
    )
    .unwrap();
    let image = ImageView::Interleaved {
        info: &info,
        samples: &source,
        stride_bytes: EXTREME_WIDTH as usize * 2,
    };
    let mut lower_boundary_succeeded = false;
    let mut unattainable_target_rejected = false;

    for bits_per_pixel in [1.58, 1.59, 1.60, 1.65, 1.70, 1.71, 8.00] {
        let raw_options = EncodeOptions {
            format: OutputFormat::J2kCodestream,
            transform: WaveletTransform::Irreversible97,
            quality: EncodeQuality::TargetRate { bits_per_pixel },
            decomposition_levels: 2,
            ..EncodeOptions::default()
        };
        let raw = encode(image, &raw_options);
        let jp2 = encode(
            image,
            &EncodeOptions {
                format: OutputFormat::Jp2,
                ..raw_options.clone()
            },
        );

        match (raw, jp2) {
            (Ok(raw), Ok(jp2)) => {
                lower_boundary_succeeded |= bits_per_pixel == 1.58;
                let budget = ((f64::from(bits_per_pixel)
                    * f64::from(EXTREME_WIDTH * EXTREME_HEIGHT))
                .floor() as usize)
                    / 8;
                assert!(
                    raw.len() <= budget,
                    "{bits_per_pixel} bpp exceeded its budget"
                );
                assert_eq!(
                    jp2.len() - raw.len(),
                    85,
                    "{bits_per_pixel} bpp JP2 overhead"
                );
                for (label, encoded) in [("raw", raw.as_slice()), ("JP2", jp2.as_slice())] {
                    let decoded = decode(
                        encoded,
                        &DecodeOptions {
                            mode: DecodeMode::Components,
                            target_layout: ComponentLayout::Interleaved,
                            ..DecodeOptions::default()
                        },
                    )
                    .unwrap_or_else(|error| {
                        panic!("{label} {bits_per_pixel} bpp did not component-decode: {error}")
                    });
                    assert_eq!(
                        decoded.info.width, EXTREME_WIDTH,
                        "{label} {bits_per_pixel} bpp width"
                    );
                    assert_eq!(
                        decoded.info.height, EXTREME_HEIGHT,
                        "{label} {bits_per_pixel} bpp height"
                    );
                }
                println!(
                    "encoder-rate-control-extreme target-bpp={bits_per_pixel:.2} budget={budget} raw-bytes={} jp2-bytes={} outcome=decoded",
                    raw.len(),
                    jp2.len(),
                );
            }
            (Err(raw_error), Err(jp2_error)) => {
                unattainable_target_rejected |= bits_per_pixel == 8.00;
                if bits_per_pixel == 8.00 {
                    for (label, error) in [("raw", raw_error), ("JP2", jp2_error)] {
                        assert!(
                            error.to_string().contains(
                                "not attainable within the qualified non-padding tolerance"
                            ),
                            "{label} failed for an unexpected reason: {error}"
                        );
                    }
                }
                println!(
                    "encoder-rate-control-extreme target-bpp={bits_per_pixel:.2} outcome=rejected"
                );
            }
            (raw, jp2) => panic!(
                "raw and JP2 target-rate results diverged at {bits_per_pixel} bpp: raw={raw:?}, jp2={jp2:?}"
            ),
        }
    }

    assert!(
        lower_boundary_succeeded,
        "the 1.58 bpp lower boundary must remain attainable and decodable"
    );
    assert!(
        unattainable_target_rejected,
        "an unattainable safe-candidate target must be rejected explicitly"
    );
}

fn peak_error(source: &[u8], decoded: &ImageData, format: SampleFormat) -> u16 {
    let ImageData::Interleaved(decoded) = decoded else {
        panic!("expected interleaved diagnostic samples")
    };
    match format.bits_per_sample {
        8 => source
            .iter()
            .zip(decoded)
            .map(|(&a, &b)| u16::from(a.abs_diff(b)))
            .max()
            .unwrap_or(0),
        16 => source
            .chunks_exact(2)
            .zip(decoded.chunks_exact(2))
            .map(|(a, b)| {
                u16::from_le_bytes(a.try_into().unwrap())
                    .abs_diff(u16::from_le_bytes(b.try_into().unwrap()))
            })
            .max()
            .unwrap_or(0),
        _ => unreachable!(),
    }
}

#[test]
fn irreversible_target_rate_accepts_planar_inputs() {
    for case in qualification_cases() {
        let interleaved = qualification_source(case);
        let bytes_per_sample = usize::from(case.format.bits_per_sample / 8);
        let pixel_count = usize::try_from(QUALIFICATION_WIDTH * QUALIFICATION_HEIGHT).unwrap();
        let mut plane_storage = (0..case.components)
            .map(|_| Vec::with_capacity(pixel_count * bytes_per_sample))
            .collect::<Vec<_>>();
        for pixel in interleaved.chunks_exact(usize::from(case.components) * bytes_per_sample) {
            for (component, plane) in plane_storage.iter_mut().enumerate() {
                let offset = component * bytes_per_sample;
                plane.extend_from_slice(&pixel[offset..offset + bytes_per_sample]);
            }
        }
        let info = ImageInfo::new(
            QUALIFICATION_WIDTH,
            QUALIFICATION_HEIGHT,
            case.components,
            case.format,
            case.colour,
            ComponentLayout::Planar,
        )
        .unwrap();
        let planes = plane_storage
            .iter()
            .map(|samples| {
                Plane::new(
                    samples,
                    QUALIFICATION_WIDTH,
                    QUALIFICATION_HEIGHT,
                    QUALIFICATION_WIDTH as usize * bytes_per_sample,
                    case.format,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let encoded = encode(
            ImageView::Planar {
                info: &info,
                planes: &planes,
            },
            &EncodeOptions {
                format: OutputFormat::J2kCodestream,
                transform: WaveletTransform::Irreversible97,
                quality: EncodeQuality::TargetRate {
                    bits_per_pixel: 2.0,
                },
                decomposition_levels: 2,
                ..EncodeOptions::default()
            },
        )
        .expect("planar target-rate input encodes");
        assert!(encoded.len() <= 12_400, "{}", case.name);
    }
}
