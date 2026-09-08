use std::ops::Range;
use std::sync::Mutex;

use c::source::{CodestreamSource, SliceSource, SourceError, SourceErrorKind};
use emuella_j2k_core::{
    ColorModel, ComponentLayout, EncodeOptions, ImageInfo, ImageView, ImageViewMut, J2kError,
    OutputFormat, Part1DecodeWorkspace, Part1SourceIndex, PlaneMut, PreparedPart1Decode,
    SampleFormat, TileSize, codestream as c, encode,
    execute_prepared_part1_decode_into_with_workspace, prepare_part1_decode_from_source,
};

struct RecordingSource {
    bytes: Vec<u8>,
    reads: Mutex<Vec<Range<usize>>>,
    allowed: Mutex<Option<Vec<Range<usize>>>>,
}

impl RecordingSource {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            reads: Mutex::new(Vec::new()),
            allowed: Mutex::new(None),
        }
    }

    fn restrict(&self, allowed: Vec<Range<usize>>) {
        self.reads.lock().unwrap().clear();
        *self.allowed.lock().unwrap() = Some(allowed);
    }
}

impl CodestreamSource for RecordingSource {
    fn len(&self) -> Result<u64, SourceError> {
        Ok(self.bytes.len() as u64)
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        let range = offset as usize..offset as usize + destination.len();
        self.reads.lock().unwrap().push(range.clone());
        if self
            .allowed
            .lock()
            .unwrap()
            .as_ref()
            .is_some_and(|allowed| {
                !allowed
                    .iter()
                    .any(|span| span.start <= range.start && span.end >= range.end)
            })
        {
            return Err(SourceError {
                kind: SourceErrorKind::Io,
                offset,
                requested: destination.len() as u64,
                available: self.bytes.len() as u64 - offset,
                message: "source-index probe denied an unselected range".into(),
            });
        }
        SliceSource::new(&self.bytes).read_exact_at(offset, destination)
    }
}

fn fixture() -> (Vec<u8>, Vec<u8>) {
    sized_fixture(137, 73)
}

fn sized_fixture(width: u32, height: u32) -> (Vec<u8>, Vec<u8>) {
    let samples = (0..height)
        .flat_map(|y| (0..width).map(move |x| ((x * 17 + y * 31 + x * y) % 256) as u8))
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
    let bytes = encode(
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: width as usize,
        },
        &EncodeOptions {
            format: OutputFormat::J2kCodestream,
            decomposition_levels: 2,
            tile_size: Some(TileSize {
                width: 32,
                height: 32,
            }),
            ..Default::default()
        },
    )
    .unwrap();
    (bytes, samples)
}

fn request(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    discard_levels: u8,
) -> c::Part1ComponentDecodeRequest<'static> {
    c::Part1ComponentDecodeRequest {
        component_indices: &[0],
        region: c::TileRegionRequest {
            x,
            y,
            width,
            height,
        },
        discard_levels,
        max_layers: None,
    }
}

fn execute(prepared: &PreparedPart1Decode<'_>) -> Vec<u8> {
    let info = prepared.info();
    let stride = info.width as usize + 7;
    let mut bytes = vec![0xa5; stride * info.height as usize + 11];
    let mut planes = [PlaneMut::new(
        &mut bytes,
        info.width,
        info.height,
        stride,
        SampleFormat::U8,
    )
    .unwrap()];
    let mut target = ImageViewMut::Planar {
        info,
        planes: &mut planes,
    };
    let metrics = execute_prepared_part1_decode_into_with_workspace(
        prepared,
        &mut target,
        &mut Part1DecodeWorkspace::new(),
        c::PreparedPart1ExecutionOptions {
            instrumentation: c::DecodeInstrumentation::WorkCounters,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        metrics.caller_output_bytes,
        u64::from(info.width) * u64::from(info.height)
    );
    for row in bytes[..stride * info.height as usize].chunks(stride) {
        assert!(row[info.width as usize..].iter().all(|byte| *byte == 0xa5));
    }
    assert!(
        bytes[stride * info.height as usize..]
            .iter()
            .all(|byte| *byte == 0xa5)
    );
    bytes
        .chunks(stride)
        .take(info.height as usize)
        .flat_map(|row| row[..info.width as usize].iter().copied())
        .collect()
}

#[test]
fn different_windows_reuse_headers_and_read_only_their_selected_tile_payload() {
    let (original, samples) = fixture();
    for tlm in [true, false] {
        let mut bytes = original.clone();
        if tlm {
            let parsed = c::parse(&bytes).unwrap();
            let sot = parsed
                .markers
                .iter()
                .find(|marker| marker.marker == c::Marker::Sot)
                .unwrap()
                .offset;
            let mut marker = vec![0xff, 0x55];
            marker.extend_from_slice(&(4_u16 + 6 * parsed.tiles.len() as u16).to_be_bytes());
            marker.extend_from_slice(&[0, 0x60]);
            for tile in &parsed.tiles {
                marker.extend_from_slice(&tile.tile_index.to_be_bytes());
                marker.extend_from_slice(&tile.tile_part_length.unwrap().to_be_bytes());
            }
            bytes.splice(sot..sot, marker);
        }
        let parsed = c::parse(&bytes).unwrap();
        let payloads = parsed
            .tiles
            .iter()
            .map(|tile| {
                let start = tile.payload_offset.unwrap();
                start..start + tile.payload_len.unwrap()
            })
            .collect::<Vec<_>>();
        let source = RecordingSource::new(bytes);
        let index = Part1SourceIndex::new(&source).unwrap();
        assert_eq!(index.tile_part_count(), 15);
        assert!(index.header_bytes() < 2048);
        let initial_reads = source.reads.lock().unwrap().len();
        // Construction must not touch compressed bodies, even for selected tiles.
        for read in source.reads.lock().unwrap().iter() {
            assert!(
                payloads
                    .iter()
                    .all(|payload| read.end <= payload.start || read.start >= payload.end)
            );
        }
        assert_eq!(index.inspect().unwrap().image.width, 137);
        assert_eq!(initial_reads, source.reads.lock().unwrap().len());
        for (tile, req) in [(0, request(3, 5, 9, 7, 0)), (8, request(101, 39, 9, 7, 0))] {
            source.restrict(vec![payloads[tile].clone()]);
            let got = execute(&index.prepare(req).unwrap());
            let sample_plane = &samples;
            let expected = (req.region.y..req.region.y + req.region.height)
                .flat_map(|y| {
                    (req.region.x..req.region.x + req.region.width)
                        .map(move |x| sample_plane[(y * 137 + x) as usize])
                })
                .collect::<Vec<_>>();
            assert_eq!(got, expected);
            assert!(!source.reads.lock().unwrap().is_empty());
            let reads = source.reads.lock().unwrap();
            eprintln!(
                "tlm={tlm} tile={tile} indexed_reads={} indexed_bytes={} initial_header_reads={initial_reads} retained_header_bytes={}",
                reads.len(),
                reads.iter().map(|read| read.len()).sum::<usize>(),
                index.header_bytes()
            );
        }
        // This same range guard rejects the legacy whole-header traversal.
        assert!(matches!(
            prepare_part1_decode_from_source(&source, request(101, 39, 9, 7, 0)),
            Err(J2kError::Source { offset: 0, .. })
        ));
    }
}

#[test]
fn indexed_full_reduced_and_cross_tile_windows_match_one_shot_and_recover_after_io_failure() {
    let (bytes, samples) = fixture();
    let source = RecordingSource::new(bytes);
    let index = Part1SourceIndex::new(&source).unwrap();
    for req in [
        request(0, 0, 137, 73, 0),
        request(29, 27, 39, 21, 0),
        request(0, 0, 137, 73, 1),
        request(32, 32, 32, 32, 2),
    ] {
        let expected = execute(
            &prepare_part1_decode_from_source(&source, req)
                .unwrap_or_else(|error| panic!("request {req:?}: {error:?}")),
        );
        assert_eq!(execute(&index.prepare(req).unwrap()), expected);
        if req.region.width == 137 && req.discard_levels == 0 {
            assert_eq!(expected, samples);
        }
    }
    source.restrict(Vec::new());
    assert!(
        matches!(index.prepare(request(3, 5, 9, 7, 0)), Err(J2kError::Source { requested, .. }) if requested > 0)
    );
    *source.allowed.lock().unwrap() = None;
    assert_eq!(
        execute(&index.prepare(request(0, 0, 137, 73, 0)).unwrap()),
        samples
    );
    assert!(index.prepare(request(0, 0, u32::MAX, u32::MAX, 0)).is_err());
}

#[test]
#[ignore = "manual source-index scaling calibration"]
fn index_geometry_scaling_calibration() {
    for (width, height) in [(137, 73), (1024, 1024)] {
        let (bytes, _) = sized_fixture(width, height);
        let source = RecordingSource::new(bytes);
        let started = std::time::Instant::now();
        let index = Part1SourceIndex::new(&source).unwrap();
        let construction_ns = started.elapsed().as_nanos();
        let initial_reads = source.reads.lock().unwrap().len();
        let mut metadata_ns = Vec::new();
        let mut preparation_ns = Vec::new();
        for iteration in 0..21 {
            let req = if iteration % 2 == 0 {
                request(3, 5, 9, 7, 0)
            } else {
                request(101, 39, 9, 7, 0)
            };
            source.reads.lock().unwrap().clear();
            let prepared = index.prepare(req).unwrap();
            metadata_ns.push(prepared.preparation_timings().marker_parse_ns);
            preparation_ns.push(prepared.preparation_timings().prepare_ns);
            execute(&prepared);
            assert_eq!(source.reads.lock().unwrap().len(), 8);
        }
        metadata_ns.sort_unstable();
        preparation_ns.sort_unstable();
        eprintln!(
            "tiles={} retained_header_bytes={} construction_ns={construction_ns} initial_header_reads={initial_reads} reuse_metadata_median_ns={} reuse_prepare_median_ns={}",
            index.tile_part_count(),
            index.header_bytes(),
            metadata_ns[10],
            preparation_ns[10]
        );
    }
}
