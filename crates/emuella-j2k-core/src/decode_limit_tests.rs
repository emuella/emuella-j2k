//! Caller decode budgets: header-only measurement and early rejection.
use super::*;

const LARGE_SIDE: u32 = 8192;
const LARGE_COMPONENTS: u64 = 4;
const LARGE_PLANE_SAMPLES: u64 = LARGE_SIDE as u64 * LARGE_SIDE as u64;

/// Tiny raw Part 1 codestream whose SIZ declares a four-component
/// 8192 × 8192 image and whose single tile carries one empty packet per
/// component. Default limits would allocate every declared plane.
fn large_siz_empty_packet_fixture() -> Vec<u8> {
    let planes = [[0_u8]; 4];
    let planes = planes
        .iter()
        .map(|plane| plane.as_slice())
        .collect::<Vec<_>>();
    let mut input = codestream::encode_planar_u8_no_decomp_test_fixture(1, 1, &planes).unwrap();
    let tile = codestream::parse(&input).unwrap().tiles[0];
    let payload_offset = tile.payload_offset.unwrap();
    let payload_len = tile.payload_len.unwrap();
    input.splice(payload_offset..payload_offset + payload_len, [0_u8; 4]);
    let siz = marker(&input, &[0xff, 0x51]);
    for field in [6, 10, 22, 26] {
        input[siz + field..siz + field + 4].copy_from_slice(&LARGE_SIDE.to_be_bytes());
    }
    let sot = marker(&input, &[0xff, 0x90, 0, 10]);
    let tile_part_len = tile.tile_part_length.unwrap() - u32::try_from(payload_len).unwrap() + 4;
    input[sot + 6..sot + 10].copy_from_slice(&tile_part_len.to_be_bytes());
    let image = inspect(&input, &InspectOptions::default())
        .unwrap()
        .image
        .unwrap();
    assert_eq!(
        (image.width, image.height, image.components),
        (8192, 8192, 4)
    );
    input
}

fn marker(input: &[u8], code: &[u8]) -> usize {
    input
        .windows(code.len())
        .position(|bytes| bytes == code)
        .unwrap()
}

/// Small decodable three-component image with (1,1), (2,1) and (2,2)
/// sampling and one decomposition level, so selection, regions and
/// reduction each change the per-component output geometry.
fn subsampled_fixture(width: u32, height: u32) -> Vec<u8> {
    let sampling = [(1_u8, 1_u8), (2, 1), (2, 2)];
    let planes = sampling
        .iter()
        .map(|(horizontal, vertical)| {
            let samples =
                width.div_ceil(u32::from(*horizontal)) * height.div_ceil(u32::from(*vertical));
            (0..samples)
                .map(|sample| u8::try_from((sample * 29 + 7) % 251).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let components = planes
        .iter()
        .zip(sampling)
        .map(|(samples, (horizontal_separation, vertical_separation))| {
            codestream::SubsampledU8TestComponent {
                horizontal_separation,
                vertical_separation,
                samples,
            }
        })
        .collect::<Vec<_>>();
    codestream::encode_planar_u8_subsampled_one_decomp_test_fixture(width, height, &components)
        .unwrap()
}

fn limits(max_output_samples: u64) -> DecodeLimits {
    DecodeLimits::default().with_max_output_samples(max_output_samples)
}

fn components_options(max_output_samples: u64) -> DecodeOptions {
    DecodeOptions::default()
        .with_mode(DecodeMode::Components)
        .with_limits(limits(max_output_samples))
}

fn exceeded(requested: u64, maximum: u64) -> J2kError {
    J2kError::LimitExceeded {
        limit: "max_output_samples",
        requested,
        maximum,
    }
}

fn output_samples(image: &Image) -> u64 {
    image
        .component_info
        .iter()
        .map(|component| u64::from(component.width) * u64::from(component.height))
        .sum()
}

fn with_one_sample_target<R>(
    caller: &mut [u8; 1],
    run: impl FnOnce(&mut ImageViewMut<'_>) -> R,
) -> R {
    let info = ImageInfo::new(
        1,
        1,
        1,
        SampleFormat::U8,
        ColorModel::Unknown,
        ComponentLayout::Planar,
    )
    .unwrap();
    let mut planes = [PlaneMut::new(caller, 1, 1, 1, SampleFormat::U8).unwrap()];
    let mut target = ImageViewMut::Planar {
        info: &info,
        planes: &mut planes,
    };
    run(&mut target)
}

#[test]
fn defaults_keep_the_built_in_total_cap_and_reject_zero() {
    assert_eq!(
        DecodeLimits::default().max_output_samples,
        DecodeLimits::BUILT_IN_MAX_OUTPUT_SAMPLES
    );
    assert_eq!(DecodeLimits::BUILT_IN_MAX_OUTPUT_SAMPLES, 256 * 1024 * 1024);
    assert_eq!(DecodeOptions::default().limits, DecodeLimits::default());
    assert_eq!(
        PartialDecodeOptions::default().limits,
        DecodeLimits::default()
    );

    let input = subsampled_fixture(9, 7);
    let zero = J2kError::InvalidParameter {
        parameter: "max_output_samples",
        message: "maximum output samples must be at least one",
    };
    assert_eq!(decode(&input, &components_options(0)), Err(zero.clone()));
    assert_eq!(
        decode_partial(
            &input,
            &PartialDecodeOptions::default().with_limits(limits(0))
        ),
        Err(zero)
    );

    // A budget above the built-in cap is accepted; the built-in caps remain.
    let unlimited = decode(&input, &components_options(u64::MAX)).unwrap();
    assert_eq!(
        unlimited,
        decode(
            &input,
            &DecodeOptions::default().with_mode(DecodeMode::Components)
        )
        .unwrap()
    );
}

#[test]
fn limit_exceeded_reports_the_budget_and_both_counts() {
    assert_eq!(
        exceeded(268_435_456, 1_048_576).to_string(),
        "decode limit `max_output_samples` exceeded: request needs 268435456, maximum 1048576"
    );
}

#[test]
fn large_declared_geometry_fails_from_headers_on_every_decode_route() {
    let input = large_siz_empty_packet_fixture();
    assert!(input.len() < 128, "fixture is {} bytes", input.len());
    let maximum = 1 << 20;
    let all = LARGE_COMPONENTS * LARGE_PLANE_SAMPLES;
    let error = exceeded(all, maximum);

    // The reported count is the declared geometry, so the request was measured
    // from SIZ alone; reconstruction would have needed 256 Mi samples first.
    let full = components_options(maximum);
    assert_eq!(decode(&input, &full), Err(error.clone()));
    assert_eq!(decode_shape(&input, &full).map(|_| ()), Err(error.clone()));
    let rendered = DecodeOptions::default().with_limits(limits(maximum));
    assert_eq!(decode(&input, &rendered), Err(error.clone()));
    let mut caller = [0x5a_u8];
    assert_eq!(
        with_one_sample_target(&mut caller, |target| decode_into(&input, target, &full)),
        Err(error.clone())
    );

    let partial = PartialDecodeOptions::default().with_limits(limits(maximum));
    assert_eq!(decode_partial(&input, &partial), Err(error.clone()));
    assert_eq!(decode_partial_info(&input, &partial), Err(error.clone()));
    assert_eq!(
        decode_partial_component_info(&input, &partial),
        Err(error.clone())
    );
    assert_eq!(
        prepare_part1_decode(&input, &partial).map(|_| ()),
        Err(error.clone())
    );
    assert_eq!(
        with_one_sample_target(&mut caller, |target| {
            decode_partial_into(&input, target, &partial)
        }),
        Err(error.clone())
    );
    assert_eq!(caller, [0x5a]);

    // Rendered partial output counts channels on the reference grid; the
    // budget fails before the route's own JP2 admission.
    assert_eq!(
        decode_rendered_partial(&input, &partial).map(|_| ()),
        Err(error.clone())
    );
    assert_eq!(
        decode_rendered_partial_info(&input, &partial).map(|_| ()),
        Err(error)
    );

    // Selection and windows are counted, not the whole declared image.
    let one_component = partial
        .clone()
        .with_components(ComponentSelection::Indices(vec![2]))
        .with_limits(limits(LARGE_PLANE_SAMPLES - 1));
    assert_eq!(
        decode_partial_info(&input, &one_component),
        Err(exceeded(LARGE_PLANE_SAMPLES, LARGE_PLANE_SAMPLES - 1))
    );
    let region = partial.with_region(Some(Region {
        x: 4000,
        y: 100,
        width: 1025,
        height: 1024,
    }));
    assert_eq!(
        decode_partial_info(&input, &region),
        Err(exceeded(LARGE_COMPONENTS * 1025 * 1024, maximum))
    );
}

#[cfg(feature = "std")]
#[test]
fn large_declared_htj2k_geometry_fails_before_workspace_decode() {
    let side = 32_768_u64;
    let input =
        codestream::encode_htj2k_reduced_component_empty_packet_test_fixture(32_768, 32_768)
            .unwrap();
    let options = components_options(1 << 20);
    let mut workspace = Htj2kDecodeWorkspace::new();
    assert_eq!(
        decode_htj2k_with_workspace(&input, &options, &mut workspace).map(|_| ()),
        Err(exceeded(3 * side * side, 1 << 20))
    );
    assert_eq!(workspace.coefficient_len(), 0);
    let reduced = PartialDecodeOptions::default()
        .with_resolution(ResolutionLevel::Reduced { discard_levels: 5 })
        .with_limits(limits(3 * 1024 * 1024 - 1));
    assert_eq!(
        decode_partial(&input, &reduced).map(|_| ()),
        Err(exceeded(3 * 1024 * 1024, 3 * 1024 * 1024 - 1))
    );
}

#[test]
fn full_decode_succeeds_exactly_at_the_limit_and_fails_one_below() {
    let input = subsampled_fixture(129, 67);
    let requested = 129 * 67 + 65 * 67 + 65 * 34;
    let image = decode(&input, &components_options(requested)).unwrap();
    assert_eq!(output_samples(&image), requested);
    assert_eq!(
        decode(&input, &components_options(requested - 1)),
        Err(exceeded(requested, requested - 1))
    );
    assert_eq!(
        decode_shape(&input, &components_options(requested - 1)).map(|_| ()),
        Err(exceeded(requested, requested - 1))
    );

    let selected = components_options(129 * 67)
        .with_requested_components(ComponentSelection::Indices(vec![0]));
    assert_eq!(
        output_samples(&decode(&input, &selected).unwrap()),
        129 * 67
    );
}

#[test]
fn caller_output_full_decode_is_bounded_before_publication() {
    let width = 23_u32;
    let height = 11_u32;
    let source = (0..width * height)
        .map(|sample| u8::try_from(sample % 251).unwrap())
        .collect::<Vec<_>>();
    let input =
        codestream::encode_planar_u8_no_decomp_test_fixture(width, height, &[&source]).unwrap();
    let requested = u64::from(width * height);
    let info = decode_shape(&input, &components_options(requested))
        .unwrap()
        .image_info()
        .unwrap();
    let decode_with = |maximum: u64, caller: &mut Vec<u8>| {
        let mut planes =
            [PlaneMut::new(caller, width, height, width as usize, SampleFormat::U8).unwrap()];
        let mut target = ImageViewMut::Planar {
            info: &info,
            planes: &mut planes,
        };
        decode_into(&input, &mut target, &components_options(maximum))
    };
    let mut caller = vec![0xa5_u8; source.len()];
    assert_eq!(
        decode_with(requested - 1, &mut caller),
        Err(exceeded(requested, requested - 1))
    );
    assert!(caller.iter().all(|sample| *sample == 0xa5));
    decode_with(requested, &mut caller).unwrap();
    assert_eq!(caller, source);
}

#[test]
fn reduced_requests_count_only_their_output() {
    let input = subsampled_fixture(129, 67);
    let full_image = 129 * 67 + 65 * 67 + 65 * 34;
    let cases = [
        (
            "discard one level",
            PartialDecodeOptions::default()
                .with_resolution(ResolutionLevel::Reduced { discard_levels: 1 }),
            65 * 34 + 33 * 34 + 33 * 17,
        ),
        (
            "one subsampled component",
            PartialDecodeOptions::default().with_components(ComponentSelection::Indices(vec![2])),
            65 * 34,
        ),
        (
            "odd region",
            PartialDecodeOptions::default().with_region(Some(Region {
                x: 3,
                y: 5,
                width: 41,
                height: 20,
            })),
            // Each endpoint is ceiling-projected to its component grid.
            41 * 20 + 20 * 20 + 20 * 10,
        ),
        (
            "region at reduced resolution",
            PartialDecodeOptions::default()
                .with_region(Some(Region {
                    x: 64,
                    y: 32,
                    width: 64,
                    height: 34,
                }))
                .with_resolution(ResolutionLevel::Reduced { discard_levels: 1 }),
            32 * 17 + 16 * 17 + 16 * 9,
        ),
    ];
    for (label, options, requested) in cases {
        assert!(requested < full_image, "{label}");
        let at_limit = options.clone().with_limits(limits(requested));
        let image =
            decode_partial(&input, &at_limit).unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(output_samples(&image), requested, "{label}");
        let components = decode_partial_component_info(&input, &at_limit).unwrap();
        assert_eq!(components, image.component_info, "{label}");

        let below = options.with_limits(limits(requested - 1));
        assert_eq!(
            decode_partial(&input, &below).map(|_| ()),
            Err(exceeded(requested, requested - 1)),
            "{label}"
        );
        assert_eq!(
            decode_partial_info(&input, &below).map(|_| ()),
            Err(exceeded(requested, requested - 1)),
            "{label}"
        );
    }
    assert_eq!(
        decode_partial(
            &input,
            &PartialDecodeOptions::default().with_limits(limits(full_image - 1))
        )
        .map(|_| ()),
        Err(exceeded(full_image, full_image - 1))
    );
}

#[test]
fn caller_output_partial_decode_is_bounded_before_publication() {
    let input = subsampled_fixture(129, 67);
    let options = PartialDecodeOptions::default()
        .with_components(ComponentSelection::Indices(vec![1]))
        .with_region(Some(Region {
            x: 10,
            y: 10,
            width: 30,
            height: 12,
        }));
    let requested = 15 * 12;
    let info = decode_partial_info(&input, &options).unwrap();
    let component = decode_partial_component_info(&input, &options).unwrap()[0].clone();
    assert_eq!(u64::from(component.width * component.height), requested);
    let run = |maximum: u64, caller: &mut Vec<u8>| {
        let mut planes = [PlaneMut::new(
            caller,
            component.width,
            component.height,
            component.width as usize,
            component.sample_format,
        )
        .unwrap()];
        let mut target = ImageViewMut::Planar {
            info: &info,
            planes: &mut planes,
        };
        decode_partial_into(
            &input,
            &mut target,
            &options.clone().with_limits(limits(maximum)),
        )
    };
    let mut caller = vec![0xc3_u8; requested as usize];
    assert_eq!(
        run(requested - 1, &mut caller),
        Err(exceeded(requested, requested - 1))
    );
    assert!(caller.iter().all(|sample| *sample == 0xc3));
    run(requested, &mut caller).unwrap();
    assert_eq!(
        caller,
        planar(&decode_partial(&input, &options).unwrap())[0]
    );

    let prepared_below = options.clone().with_limits(limits(requested - 1));
    assert_eq!(
        prepare_part1_decode(&input, &prepared_below).map(|_| ()),
        Err(exceeded(requested, requested - 1))
    );
    prepare_part1_decode(&input, &options.with_limits(limits(requested))).unwrap();
}

fn planar(image: &Image) -> &[Vec<u8>] {
    let ImageData::Planes(planes) = &image.data else {
        panic!("native component decode returned interleaved samples");
    };
    planes
}

#[cfg(feature = "std")]
#[test]
fn crop_backed_partial_routes_also_bound_their_full_decode() {
    let width = 40_u32;
    let height = 30_u32;
    let samples = (0..width * height)
        .map(|sample| u8::try_from(sample % 200).unwrap())
        .collect::<Vec<_>>();
    let info = ImageInfo::new(
        width,
        height,
        1,
        SampleFormat::U8,
        ColorModel::Grayscale,
        ComponentLayout::Interleaved,
    )
    .unwrap();
    let input = encode_htj2k(
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: width as usize,
        },
        &Htj2kEncodeOptions::default(),
    )
    .unwrap();
    // This HTJ2K region request is served by reconstructing the full image and
    // cropping it, so that internal full decode is what the budget must cover.
    let region = PartialDecodeOptions::default().with_region(Some(Region {
        x: 2,
        y: 3,
        width: 10,
        height: 10,
    }));
    let full_image = u64::from(width * height);
    assert_eq!(
        decode_partial(&input, &region.clone().with_limits(limits(100))).map(|_| ()),
        Err(exceeded(full_image, 100))
    );
    let image = decode_partial(&input, &region.with_limits(limits(full_image))).unwrap();
    assert_eq!(output_samples(&image), 100);
}
