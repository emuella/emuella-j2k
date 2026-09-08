//! Optional source interoperability; inputs remain with the testdata owner.
use emuella_j2k_core::{
    ImageViewMut, Part1DecodeWorkspace, Part1SourceIndex, PlaneMut, PreparedPart1Decode,
    codestream as c, execute_prepared_part1_decode_into_with_workspace,
    prepare_part1_decode_from_source,
};
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn decode(
    source: &dyn c::source::CodestreamSource,
    bands: u16,
    bits: u8,
    region: c::TileRegionRequest,
    discard: u8,
) -> (u32, u32, Vec<Vec<u8>>) {
    let components = (0..bands).collect::<Vec<_>>();
    let prepared = prepare_part1_decode_from_source(
        source,
        c::Part1ComponentDecodeRequest {
            component_indices: &components,
            region,
            discard_levels: discard,
            max_layers: None,
        },
    )
    .expect("prepare independent source");
    execute(prepared, bands, bits)
}

fn execute(prepared: PreparedPart1Decode<'_>, bands: u16, bits: u8) -> (u32, u32, Vec<Vec<u8>>) {
    assert_eq!(prepared.info().sample_format.bits_per_sample, bits);
    let width = prepared.info().width;
    let height = prepared.info().height;
    let stride = width as usize * usize::from(bits).div_ceil(8);
    let mut output = vec![vec![0; stride * height as usize]; usize::from(bands)];
    let mut planes = output
        .iter_mut()
        .map(|bytes| {
            PlaneMut::new(bytes, width, height, stride, prepared.info().sample_format).unwrap()
        })
        .collect::<Vec<_>>();
    execute_prepared_part1_decode_into_with_workspace(
        &prepared,
        &mut ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        },
        &mut Part1DecodeWorkspace::new(),
        c::PreparedPart1ExecutionOptions::default(),
    )
    .expect("execute independent source");
    (width, height, output)
}

#[test]
#[ignore = "optional testdata pack via EMUELLA_INDEPENDENT_NITF_INPUT"]
fn independent_source_full_pixels_and_regions_agree() {
    let Some(root) = std::env::var_os("EMUELLA_INDEPENDENT_NITF_INPUT") else {
        eprintln!("SKIP: optional independent NITF pack is absent");
        return;
    };
    let root = std::path::PathBuf::from(root);
    // Testdata 2d519ddaf019f10b9e409ea3338d395438486647, independent-nitf v1.
    // Decoded hashes use the owner's tile-major, interleaved native sample order.
    let cases = [
        (
            "pan11-lossless",
            1057,
            65,
            1_u16,
            11_u8,
            "eab53affe5e3f715c225b770d611fe0adaf025d143f7a7b4315e66111514300f",
            "cc36989aa2a4d571d7a156fa003881523853a44799d094e4a116bf5976a65a25",
        ),
        (
            "pan11-lossy",
            1057,
            65,
            1_u16,
            11_u8,
            "fc10d857ccd82852806212cc3a8aa0728b2f5a38afd277f0f3aba54d3ff117f8",
            "e9f2a6eb484710dc023a788c1dbe93fda8e948835c38c954db60ca559a62fca8",
        ),
        (
            "grey16-lossless",
            97,
            65,
            1_u16,
            16_u8,
            "0f78a2246143b2264e36b2aa363b9eff5bdc8d5590d370a5b7888359ea012b52",
            "f9a8fd4e128710203f423d13afded56564e045165f849a9b12d8017566b41239",
        ),
        (
            "rgb8-lossless",
            97,
            65,
            3_u16,
            8_u8,
            "74e5f370cf0bed49b36d9d58dff4e625027f551ef224bedb1363fe65910f0f33",
            "f8befd320fce0751848d8cc0cd696229294904b5428cc6d868403d0a79e838c6",
        ),
        (
            "rgb16-lossless",
            97,
            65,
            3_u16,
            16_u8,
            "0a5044c40e69a8970b615255461529196a29d74ce1c25d58d9b75b2f49cb64cc",
            "dbf4011c409056171a91c0e069ff285eb1587e11d1c93d7c9e14215743459008",
        ),
    ];
    for (name, width, height, bands, bits, input_hash, pixel_hash) in cases {
        let input = std::fs::read(root.join(format!("{name}.j2k"))).expect("read locked input");
        assert_eq!(digest(&input), input_hash, "input identity: {name}");
        let source = c::source::SliceSource::new(&input);
        let index = Part1SourceIndex::new(&source).expect("index independent source once");
        let retained_headers = (index.header_bytes(), index.tile_part_count());
        let indexed_components = (0..bands).collect::<Vec<_>>();
        let decode_indexed = |region, discard| {
            let prepared = index
                .prepare(c::Part1ComponentDecodeRequest {
                    component_indices: &indexed_components,
                    region,
                    discard_levels: discard,
                    max_layers: None,
                })
                .expect("prepare retained independent source index");
            execute(prepared, bands, bits)
        };
        let full = c::TileRegionRequest {
            x: 0,
            y: 0,
            width,
            height,
        };
        let sample_bytes = usize::from(bits).div_ceil(8);
        for discard in 0..=2 {
            let legacy_full = decode(&source, bands, bits, full, discard);
            let indexed_full = decode_indexed(full, discard);
            assert!(
                legacy_full == indexed_full,
                "indexed/legacy complete samples: {name}, discard {discard}"
            );
            let (full_width, _, reference) = legacy_full;
            if discard == 0 {
                let mut interleaved = Vec::new();
                for tile_x in (0..width).step_by(1024) {
                    for y in 0..height {
                        for x in tile_x..(tile_x + 1024).min(width) {
                            let offset = (y * width + x) as usize * sample_bytes;
                            for plane in &reference {
                                interleaved
                                    .extend_from_slice(&plane[offset..offset + sample_bytes]);
                            }
                        }
                    }
                }
                if name == "pan11-lossy" {
                    let reference_path = std::env::var_os(
                        "EMUELLA_INDEPENDENT_NITF_LOSSY_REFERENCE",
                    )
                    .expect("supply the independent OpenJPEG PGM reference for lossy comparison");
                    let reference_file =
                        std::fs::read(reference_path).expect("read independent reference in place");
                    let mut cursor = 0;
                    let mut tokens = Vec::new();
                    while tokens.len() < 4 {
                        while reference_file
                            .get(cursor)
                            .is_some_and(u8::is_ascii_whitespace)
                        {
                            cursor += 1;
                        }
                        if reference_file.get(cursor) == Some(&b'#') {
                            while reference_file
                                .get(cursor)
                                .is_some_and(|byte| *byte != b'\n')
                            {
                                cursor += 1;
                            }
                            continue;
                        }
                        let start = cursor;
                        while reference_file
                            .get(cursor)
                            .is_some_and(|byte| !byte.is_ascii_whitespace())
                        {
                            cursor += 1;
                        }
                        assert!(cursor > start, "truncated PGM header");
                        tokens.push(&reference_file[start..cursor]);
                    }
                    assert_eq!(tokens, [b"P5".as_slice(), b"1057", b"65", b"2047"]);
                    cursor += 1;
                    let payload = &reference_file[cursor..];
                    assert_eq!(payload.len(), width as usize * height as usize * 2);
                    let mut reference_ordered = Vec::with_capacity(payload.len());
                    for tile_x in (0..width).step_by(1024) {
                        for y in 0..height {
                            for x in tile_x..(tile_x + 1024).min(width) {
                                let offset = (y * width + x) as usize * 2;
                                reference_ordered
                                    .extend_from_slice(&[payload[offset + 1], payload[offset]]);
                            }
                        }
                    }
                    assert_eq!(
                        digest(&reference_ordered),
                        pixel_hash,
                        "independent reference identity"
                    );
                    let mut peak = 0;
                    let mut squared = 0_u64;
                    let mut differing = 0;
                    for (actual, expected) in interleaved
                        .chunks_exact(2)
                        .zip(reference_ordered.chunks_exact(2))
                    {
                        let error = i32::from(u16::from_le_bytes(actual.try_into().unwrap()))
                            - i32::from(u16::from_le_bytes(expected.try_into().unwrap()));
                        peak = peak.max(error.unsigned_abs());
                        squared += u64::from(error.unsigned_abs()).pow(2);
                        differing += usize::from(error != 0);
                    }
                    // The existing irreversible qualification uses one native
                    // code value between independent floating-point decoders.
                    assert!(
                        peak <= 1,
                        "lossy independent peak error exceeds one code value"
                    );
                    println!(
                        "lossy independent samples: peak {peak}, squared error {squared}, differing {differing}"
                    );
                } else {
                    assert_eq!(
                        digest(&interleaved),
                        pixel_hash,
                        "independent native pixels: {name}"
                    );
                }
            }
            for (x, y, rw, rh) in [
                (3, 5, 31, 27),
                (width - 17, height - 13, 17, 13),
                (
                    width.min(1024) - 9,
                    19,
                    (width - width.min(1024) + 9).min(25),
                    23,
                ),
            ] {
                let region = c::TileRegionRequest {
                    x,
                    y,
                    width: rw,
                    height: rh,
                };
                let legacy_region = decode(&source, bands, bits, region, discard);
                let indexed_region = decode_indexed(region, discard);
                assert!(
                    legacy_region == indexed_region,
                    "indexed/legacy regional samples: {name}, discard {discard}, region {region:?}"
                );
                let (regional_width, regional_height, actual) = legacy_region;
                let step = 1_u32 << discard;
                let ox = x.div_ceil(step);
                let oy = y.div_ceil(step);
                assert_eq!(regional_width, (x + rw).div_ceil(step) - ox);
                assert_eq!(regional_height, (y + rh).div_ceil(step) - oy);
                for (plane, expected) in actual.iter().zip(&reference) {
                    for row in 0..regional_height {
                        let begin = ((oy + row) * full_width + ox) as usize * sample_bytes;
                        let len = regional_width as usize * sample_bytes;
                        assert!(
                            plane[row as usize * len..(row as usize + 1) * len]
                                == expected[begin..begin + len],
                            "regional samples: {name}, discard {discard}, region {region:?}"
                        );
                    }
                }
            }
            assert!(
                decode_indexed(full, discard) == indexed_full,
                "indexed complete revisit after regions: {name}, discard {discard}"
            );
            assert_eq!(
                (index.header_bytes(), index.tile_part_count()),
                retained_headers
            );
            println!(
                "PASS {name}: discard {discard}, legacy/indexed complete and three exact regions, indexed complete revisit"
            );
        }
    }
}
