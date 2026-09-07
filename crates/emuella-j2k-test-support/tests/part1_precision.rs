use emuella_j2k_core::{
    ColorModel, ComponentLayout, EncodeOptions, ImageInfo, ImageView, ImageViewMut, OutputFormat,
    Part1DecodeWorkspace, PlaneMut, SampleEndian, SampleFormat, TileSize, codestream as c, encode,
    execute_prepared_part1_decode_into_with_workspace, prepare_part1_decode_from_source,
};

fn sample(x: u32, y: u32, band: u16, bits: u8) -> u16 {
    let mask = (1_u32 << bits) - 1;
    match (x + y + u32::from(band)) % 7 {
        0 => 0,
        1 => mask as u16,
        _ => ((x * 997 + y * 617 + x * y * 13 + u32::from(band) * 1237) & mask) as u16,
    }
}

fn fixture(bits: u8, bands: u16, levels: u8, tiled: bool) -> Vec<u8> {
    let format = SampleFormat::with_byte_order(bits, false, Some(SampleEndian::Little)).unwrap();
    let info = ImageInfo::new(
        67,
        53,
        bands,
        format,
        if bands == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        ComponentLayout::Interleaved,
    )
    .unwrap();
    let samples = (0..53)
        .flat_map(|y| {
            (0..67).flat_map(move |x| {
                (0..bands).flat_map(move |band| sample(x, y, band, bits).to_le_bytes())
            })
        })
        .collect::<Vec<_>>();
    encode(
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: 67 * usize::from(bands) * 2,
        },
        &EncodeOptions {
            format: OutputFormat::J2kCodestream,
            decomposition_levels: levels,
            tile_size: tiled.then_some(TileSize {
                width: 32,
                height: 32,
            }),
            ..Default::default()
        },
    )
    .unwrap()
}

#[test]
fn precision_aware_samples_and_regional_rgb_match_arithmetic_oracle() {
    for bits in 9..=16 {
        for bands in [1, 3] {
            if bands == 3 && bits != 16 {
                continue;
            }
            for (levels, tiled) in [(0, false), (1, false), (2, false), (2, true)] {
                let bytes = fixture(bits, bands, levels, tiled);
                let parsed = c::parse(&bytes).unwrap();
                assert!(
                    parsed
                        .siz
                        .components
                        .iter()
                        .all(|component| component.bits_per_sample == bits)
                );
                let source =
                    c::source::InstrumentedSource::new(c::source::SliceSource::new(&bytes));
                for (x, y, width, height) in [(29, 27, 9, 11), (63, 49, 4, 4), (0, 0, 67, 53)] {
                    let components = if bands == 3 { vec![2, 0, 1] } else { vec![0] };
                    let p = prepare_part1_decode_from_source(
                        &source,
                        c::Part1ComponentDecodeRequest {
                            component_indices: &components,
                            region: c::TileRegionRequest {
                                x,
                                y,
                                width,
                                height,
                            },
                            discard_levels: 0,
                            max_layers: None,
                        },
                    )
                    .unwrap();
                    assert_eq!(p.info().sample_format.bits_per_sample, bits);
                    let stride = width as usize * 2 + 7;
                    let mut outputs =
                        vec![vec![0xa5; stride * height as usize + 11]; components.len()];
                    let mut workspace = Part1DecodeWorkspace::new();
                    for _ in 0..2 {
                        let mut planes = outputs
                            .iter_mut()
                            .map(|out| {
                                PlaneMut::new(out, width, height, stride, p.info().sample_format)
                                    .unwrap()
                            })
                            .collect::<Vec<_>>();
                        let mut target = ImageViewMut::Planar {
                            info: p.info(),
                            planes: &mut planes,
                        };
                        let metrics = execute_prepared_part1_decode_into_with_workspace(
                            &p,
                            &mut target,
                            &mut workspace,
                            c::PreparedPart1ExecutionOptions {
                                instrumentation: c::DecodeInstrumentation::WorkCounters,
                                ..Default::default()
                            },
                        )
                        .unwrap();
                        assert_eq!(
                            metrics.caller_output_bytes,
                            u64::from(width * height) * 2 * components.len() as u64
                        );
                    }
                    for (out, band) in outputs.iter().zip(components) {
                        for row in 0..height {
                            for col in 0..width {
                                let offset = row as usize * stride + col as usize * 2;
                                assert_eq!(
                                    u16::from_le_bytes(out[offset..offset + 2].try_into().unwrap()),
                                    sample(x + col, y + row, band, bits),
                                    "bits={bits} bands={bands} levels={levels} tiled={tiled} x={} y={}",
                                    x + col,
                                    y + row
                                );
                            }
                            assert!(
                                out[row as usize * stride + width as usize * 2
                                    ..(row as usize + 1) * stride]
                                    .iter()
                                    .all(|v| *v == 0xa5)
                            );
                        }
                        assert!(out[stride * height as usize..].iter().all(|v| *v == 0xa5));
                    }
                }
            }
        }
    }
}

#[test]
fn precision_encoder_rejects_out_of_range_words() {
    for bits in 9..16 {
        let samples = (1_u16 << bits).to_le_bytes();
        let input = c::GrayscaleU16LeEncode {
            width: 1,
            height: 1,
            samples: &samples,
            stride_bytes: 2,
        };
        assert!(c::encode_grayscale_u16_le_with_precision(input, bits, 0, None).is_err());
    }
}

#[test]
fn mct_region_rejects_mismatched_and_signed_precision() {
    let original = fixture(16, 3, 2, true);
    let siz = original
        .windows(2)
        .position(|bytes| bytes == [0xff, 0x51])
        .unwrap();
    for ssiz in [7, 10, 0x8f] {
        let mut bytes = original.clone();
        // Mutated headers are negative admission neighbours only.
        bytes[siz + 42 + 3] = ssiz;
        let source = c::source::SliceSource::new(&bytes);
        assert!(
            prepare_part1_decode_from_source(
                &source,
                c::Part1ComponentDecodeRequest {
                    component_indices: &[0],
                    region: c::TileRegionRequest {
                        x: 29,
                        y: 27,
                        width: 9,
                        height: 11
                    },
                    discard_levels: 0,
                    max_layers: None,
                }
            )
            .is_err()
        );
    }
}
