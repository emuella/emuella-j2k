use std::sync::atomic::{AtomicBool, Ordering};

use emuella_j2k_core::{
    ImageViewMut, Part1DecodeWorkspace, PlaneMut, Region, SampleFormat, codestream,
    execute_prepared_part1_decode_into_with_workspace, prepare_part1_decode_from_source,
};
use emuella_j2k_test_support::native_planes::{
    ReversibleMctRegionFixture, reversible_mct_region_fixture,
};

fn marker(bytes: &[u8], code: [u8; 2]) -> usize {
    bytes.windows(2).position(|window| window == code).unwrap()
}

fn crop(samples: &[u8], image_width: u32, region: Region) -> Vec<u8> {
    let image_width = usize::try_from(image_width).unwrap();
    let x = usize::try_from(region.x).unwrap();
    let y = usize::try_from(region.y).unwrap();
    let width = usize::try_from(region.width).unwrap();
    (0..usize::try_from(region.height).unwrap())
        .flat_map(|row| {
            let start = (y + row) * image_width + x;
            samples[start..start + width].iter().copied()
        })
        .collect()
}

fn execute_source(
    fixture: &ReversibleMctRegionFixture,
    bytes: &[u8],
    components: &[u16],
    region: Region,
) -> (
    Vec<Vec<u8>>,
    codestream::DecodeStageTimings,
    codestream::source::SourceMetrics,
) {
    let source =
        codestream::source::InstrumentedSource::new(codestream::source::SliceSource::new(bytes));
    let prepared = prepare_part1_decode_from_source(
        &source,
        codestream::Part1ComponentDecodeRequest {
            component_indices: components,
            region: codestream::TileRegionRequest {
                x: region.x,
                y: region.y,
                width: region.width,
                height: region.height,
            },
            discard_levels: 0,
            max_layers: None,
        },
    )
    .unwrap();
    assert_eq!(prepared.reconstruction_component_indices(), [0, 1, 2]);
    assert_eq!(prepared.info().width, region.width);
    assert_eq!(prepared.info().height, region.height);
    assert_eq!(prepared.info().components as usize, components.len());

    let stride = usize::try_from(region.width).unwrap() + 7;
    let height = usize::try_from(region.height).unwrap();
    let mut padded = components
        .iter()
        .map(|_| vec![0xa5; stride * height + 11])
        .collect::<Vec<_>>();
    let timings = {
        let mut planes = padded
            .iter_mut()
            .map(|samples| {
                PlaneMut::new(
                    samples,
                    region.width,
                    region.height,
                    stride,
                    SampleFormat::U8,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let mut target = ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        };
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions {
                instrumentation: codestream::DecodeInstrumentation::DetailedProfile,
                collect_tier1_work_counters: true,
                parallelism: codestream::DecodeExecutionParallelism::Serial,
                ..codestream::PreparedPart1ExecutionOptions::default()
            },
        )
        .unwrap()
    };
    let active = padded
        .iter()
        .map(|plane| {
            assert!(plane.chunks(stride).take(height).all(|row| {
                row[usize::try_from(region.width).unwrap()..]
                    .iter()
                    .all(|byte| *byte == 0xa5)
            }));
            assert!(plane[stride * height..].iter().all(|byte| *byte == 0xa5));
            plane
                .chunks(stride)
                .take(height)
                .flat_map(|row| {
                    row[..usize::try_from(region.width).unwrap()]
                        .iter()
                        .copied()
                })
                .collect()
        })
        .collect();
    let metrics = source.metrics();
    assert!(metrics.source_read_operations > 0);
    assert!(metrics.largest_source_read < bytes.len() as u64);
    assert_eq!(
        timings.output_samples,
        u64::from(region.width) * u64::from(region.height) * components.len() as u64
    );
    assert_eq!(fixture.width, 256);
    (active, timings, metrics)
}

#[test]
fn tnsot_zero_and_one_are_exact_for_full_regions_and_boundary_windows() {
    let fixture = reversible_mct_region_fixture();
    for bytes in [&fixture.tnsot_zero, &fixture.tnsot_one] {
        let parsed = codestream::parse(bytes).unwrap();
        let style = parsed.uniform_effective_coding_style().unwrap();
        assert_eq!(style.layers, 19);
        assert_eq!(style.decomposition_levels, 5);
        assert!(style.multiple_component_transform && style.eph_markers);
        assert!(
            parsed
                .markers
                .iter()
                .any(|entry| entry.marker == codestream::Marker::Tlm)
        );
        assert!(
            parsed
                .markers
                .iter()
                .any(|entry| entry.marker == codestream::Marker::Plt)
        );

        let full = Region {
            x: 0,
            y: 0,
            width: fixture.width,
            height: fixture.height,
        };
        let (planes, work, _) = execute_source(&fixture, bytes, &[0, 1, 2], full);
        assert_eq!(planes, fixture.planes);
        assert_eq!(
            work.caller_output_bytes,
            u64::from(fixture.width * fixture.height * 3)
        );

        for region in [
            Region {
                x: 63,
                y: 47,
                width: 19,
                height: 21,
            },
            Region {
                x: 251,
                y: 187,
                width: 5,
                height: 5,
            },
        ] {
            for component in 0..3_u16 {
                let (actual, _, _) = execute_source(&fixture, bytes, &[component], region);
                assert_eq!(
                    actual[0],
                    crop(
                        &fixture.planes[usize::from(component)],
                        fixture.width,
                        region
                    )
                );
            }
        }
    }
}

#[test]
fn regional_planning_retains_mct_dependencies_but_not_full_image_work() {
    let fixture = reversible_mct_region_fixture();
    let full = Region {
        x: 0,
        y: 0,
        width: fixture.width,
        height: fixture.height,
    };
    let region = Region {
        x: 7,
        y: 11,
        width: 13,
        height: 17,
    };
    let (_, full_one, full_source) = execute_source(&fixture, &fixture.tnsot_zero, &[0], full);
    let (_, full_three, _) = execute_source(&fixture, &fixture.tnsot_zero, &[0, 1, 2], full);
    let (_, regional, regional_source) =
        execute_source(&fixture, &fixture.tnsot_zero, &[1], region);

    assert_eq!(full_one.code_blocks_planned, full_three.code_blocks_planned);
    assert_eq!(
        full_one.tier1_codeword_bytes,
        full_three.tier1_codeword_bytes
    );
    assert!(regional.code_blocks_planned < full_one.code_blocks_planned);
    assert!(regional.tier1_codeword_bytes < full_one.tier1_codeword_bytes);
    assert!(regional_source.source_bytes_returned < full_source.source_bytes_returned);
    assert_eq!(
        regional.caller_output_bytes,
        u64::from(region.width * region.height)
    );
    assert_eq!(
        regional.synthesis_output_samples,
        u64::from(region.width * region.height) * 3
    );
}

#[test]
fn full_region_preflights_aggregate_mct_storage_and_forced_options() {
    let fixture = reversible_mct_region_fixture();
    let source = codestream::source::InstrumentedSource::new(codestream::source::SliceSource::new(
        &fixture.tnsot_zero,
    ));
    let component_indices = [0_u16];
    let prepared = prepare_part1_decode_from_source(
        &source,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &component_indices,
            region: codestream::TileRegionRequest {
                x: 0,
                y: 0,
                width: fixture.width,
                height: fixture.height,
            },
            discard_levels: 0,
            max_layers: None,
        },
    )
    .unwrap();
    let sample_count = usize::try_from(fixture.width * fixture.height).unwrap();
    let mut output = vec![0_u8; sample_count];
    let timings = {
        let plane = PlaneMut::new(
            &mut output,
            fixture.width,
            fixture.height,
            fixture.width as usize,
            SampleFormat::U8,
        )
        .unwrap();
        let mut planes = [plane];
        let mut target = ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        };
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions {
                instrumentation: codestream::DecodeInstrumentation::WorkCounters,
                parallelism: codestream::DecodeExecutionParallelism::Serial,
                ..codestream::PreparedPart1ExecutionOptions::default()
            },
        )
        .unwrap()
    };
    assert_eq!(output, fixture.planes[0]);
    assert!(
        timings.full_synthesis_request_admitted_bytes
            > (sample_count as u64) * (3 * std::mem::size_of::<i32>() as u64 + 1)
    );
    assert_eq!(
        timings.full_synthesis_request_admitted_bytes,
        timings.full_synthesis_admitted_bytes
    );
    assert!(timings.full_synthesis_admitted_bytes >= timings.full_synthesis_estimated_bytes);
    assert_eq!(timings.full_intermediate_output_bytes, sample_count as u64);
    assert!(timings.full_coefficient_plane_capacity >= sample_count as u64);
    assert!(timings.full_transform_scratch_capacity > 0);
    assert!(timings.peak_scratch_bytes >= timings.full_synthesis_request_admitted_bytes);
    assert_eq!(
        timings.full_synthesis_backend,
        Some(codestream::FullSynthesisBackend::LegacyScalar)
    );

    let metrics_before_rejection = source.metrics();
    let mut limited = vec![0x6d_u8; sample_count];
    let before = limited.clone();
    let error = {
        let plane = PlaneMut::new(
            &mut limited,
            fixture.width,
            fixture.height,
            fixture.width as usize,
            SampleFormat::U8,
        )
        .unwrap();
        let mut planes = [plane];
        let mut target = ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        };
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions {
                instrumentation: codestream::DecodeInstrumentation::WorkCounters,
                parallelism: codestream::DecodeExecutionParallelism::Serial,
                retained_memory_limit: Some(timings.full_synthesis_request_admitted_bytes - 1),
                ..codestream::PreparedPart1ExecutionOptions::default()
            },
        )
        .unwrap_err()
    };
    assert!(format!("{error:?}").contains("retained-memory limit"));
    assert_eq!(limited, before);
    assert_eq!(source.metrics(), metrics_before_rejection);

    for forced in [
        codestream::PreparedPart1ExecutionOptions {
            full_synthesis_backend: Some(codestream::FullSynthesisBackend::LegacyScalar),
            ..codestream::PreparedPart1ExecutionOptions::default()
        },
        codestream::PreparedPart1ExecutionOptions {
            synthesis_crossover_route: Some(codestream::SynthesisCrossoverRoute::Full),
            ..codestream::PreparedPart1ExecutionOptions::default()
        },
    ] {
        let mut caller = vec![0x4b_u8; sample_count];
        let before = caller.clone();
        let plane = PlaneMut::new(
            &mut caller,
            fixture.width,
            fixture.height,
            fixture.width as usize,
            SampleFormat::U8,
        )
        .unwrap();
        let mut planes = [plane];
        let mut target = ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        };
        assert!(
            execute_prepared_part1_decode_into_with_workspace(
                &prepared,
                &mut target,
                &mut Part1DecodeWorkspace::new(),
                forced,
            )
            .is_err()
        );
        assert_eq!(caller, before);
        assert_eq!(source.metrics(), metrics_before_rejection);
    }
}

struct SwitchFailSource<'a> {
    bytes: &'a [u8],
    fail: AtomicBool,
}

impl codestream::source::CodestreamSource for SwitchFailSource<'_> {
    fn len(&self) -> Result<u64, codestream::source::SourceError> {
        Ok(self.bytes.len() as u64)
    }

    fn read_exact_at(
        &self,
        offset: u64,
        destination: &mut [u8],
    ) -> Result<(), codestream::source::SourceError> {
        if self.fail.load(Ordering::Relaxed) {
            return Err(codestream::source::SourceError {
                kind: codestream::source::SourceErrorKind::Io,
                offset,
                requested: destination.len() as u64,
                available: self.bytes.len() as u64,
                message: "injected execution read failure".into(),
            });
        }
        let start = usize::try_from(offset).unwrap();
        let source = self.bytes.get(start..start + destination.len()).unwrap();
        destination.copy_from_slice(source);
        Ok(())
    }
}

#[test]
fn source_execution_failure_preserves_the_complete_caller_plane() {
    let fixture = reversible_mct_region_fixture();
    let source = SwitchFailSource {
        bytes: &fixture.tnsot_zero,
        fail: AtomicBool::new(false),
    };
    let component_indices = [2];
    let prepared = prepare_part1_decode_from_source(
        &source,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &component_indices,
            region: codestream::TileRegionRequest {
                x: 9,
                y: 13,
                width: 23,
                height: 19,
            },
            discard_levels: 0,
            max_layers: None,
        },
    )
    .unwrap();
    source.fail.store(true, Ordering::Relaxed);
    let mut caller = vec![0x6d; 29 * 19 + 13];
    let before = caller.clone();
    let plane = PlaneMut::new(&mut caller, 23, 19, 29, SampleFormat::U8).unwrap();
    let mut planes = [plane];
    let mut target = ImageViewMut::Planar {
        info: prepared.info(),
        planes: &mut planes,
    };
    assert!(
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions::default(),
        )
        .is_err()
    );
    assert_eq!(caller, before);
}

fn without_tlm(mut bytes: Vec<u8>) -> Vec<u8> {
    let offset = marker(&bytes, codestream::Marker::Tlm.code().to_be_bytes());
    let length = usize::from(u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]));
    bytes.drain(offset..offset + 2 + length);
    bytes
}

fn two_parts(bytes: &[u8], second_index: u8, first_count: u8, second_count: u8) -> Vec<u8> {
    let mut bytes = without_tlm(bytes.to_vec());
    let sot = marker(&bytes, codestream::Marker::Sot.code().to_be_bytes());
    let psot = usize::try_from(u32::from_be_bytes(
        bytes[sot + 6..sot + 10].try_into().unwrap(),
    ))
    .unwrap();
    bytes[sot + 11] = first_count;
    let mut second = bytes[sot..sot + psot].to_vec();
    second[10] = second_index;
    second[11] = second_count;
    let eoc = marker(
        &bytes[sot + psot..],
        codestream::Marker::Eoc.code().to_be_bytes(),
    ) + sot
        + psot;
    bytes.splice(eoc..eoc, second);
    bytes
}

fn prepare_component_zero(bytes: &[u8]) -> Result<(), String> {
    codestream::prepare_part1_component_decode(
        bytes,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &[0],
            region: codestream::TileRegionRequest {
                x: 0,
                y: 0,
                width: 256,
                height: 192,
            },
            discard_levels: 0,
            max_layers: None,
        },
    )
    .map(|_| ())
    .map_err(|error| format!("{error:?}"))
}

#[test]
fn malformed_tile_part_sequences_lengths_and_truncation_fail_closed() {
    let fixture = reversible_mct_region_fixture();
    let original = &fixture.tnsot_zero;
    let sot = marker(original, codestream::Marker::Sot.code().to_be_bytes());
    let tlm = marker(original, codestream::Marker::Tlm.code().to_be_bytes());

    let mut out_of_range_tile = original.clone();
    out_of_range_tile[sot + 5] = 1;
    let mut non_zero_first_index = original.clone();
    non_zero_first_index[sot + 10] = 1;
    let mut non_one_declared_count = original.clone();
    non_one_declared_count[sot + 11] = 2;
    let mut tlm_mismatch = original.clone();
    tlm_mismatch[tlm + 11] ^= 1;
    let mut psot_mismatch = original.clone();
    psot_mismatch[sot + 9] ^= 1;
    let duplicate_zero = two_parts(original, 0, 0, 0);
    let inconsistent = two_parts(original, 1, 2, 1);
    let unsupported_two_parts = two_parts(original, 1, 2, 2);
    let mut trailing = original.clone();
    trailing.extend_from_slice(&[0, 0]);
    let mut missing = without_tlm(original.clone());
    let missing_sot = marker(&missing, codestream::Marker::Sot.code().to_be_bytes());
    let missing_psot = usize::try_from(u32::from_be_bytes(
        missing[missing_sot + 6..missing_sot + 10]
            .try_into()
            .unwrap(),
    ))
    .unwrap();
    missing.drain(missing_sot..missing_sot + missing_psot);

    for (name, candidate) in [
        ("out-of-range tile", out_of_range_tile),
        ("non-zero first index", non_zero_first_index),
        ("non-one declared count", non_one_declared_count),
        ("TLM mismatch", tlm_mismatch),
        ("Psot mismatch", psot_mismatch),
        ("duplicate", duplicate_zero),
        ("inconsistent", inconsistent),
        ("two parts", unsupported_two_parts),
        ("missing", missing),
        ("truncated", original[..original.len() - 1].to_vec()),
        ("bytes after EOC", trailing),
    ] {
        assert!(
            prepare_component_zero(&candidate).is_err(),
            "admitted {name}"
        );
    }
}
