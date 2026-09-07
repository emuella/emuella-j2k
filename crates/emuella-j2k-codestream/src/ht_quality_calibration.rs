//! Authored packet-prefix experiment; see docs/ht-quality-calibration.md.
use super::*;
use sha2::{Digest, Sha256};
use std::time::Instant;

const EDGE: u32 = 256;
const LEVELS: u8 = 5;
fn shifts(bits: u8) -> [u8; 3] {
    [bits - 7, bits - 9, 0]
}

fn source(bits: u8, components: usize) -> Vec<Vec<u16>> {
    let unit = 1_i32 << (bits - 11);
    (0..components)
        .map(|c| {
            (0..EDGE * EDGE)
                .map(|i| {
                    let (x, y) = (i % EDGE, i / EDGE);
                    let texture =
                        ((x * 977 + y * 1393 + x * y * 41 + c as u32 * 53) % 65) as i32 - 32;
                    let edge = if x > 133 + (y / 31) { 380 } else { 0 };
                    let faint = if (35..95).contains(&x) && (55..115).contains(&y) {
                        8
                    } else {
                        0
                    };
                    let object = if (119..122).contains(&x) && (143..146).contains(&y) {
                        160
                    } else {
                        0
                    };
                    let lines = if (180..228).contains(&x) && y % 9 == 0 {
                        32
                    } else {
                        0
                    };
                    ((560
                        + x as i32
                        + y as i32 / 2
                        + texture
                        + edge
                        + faint
                        + object
                        + lines
                        + c as i32 * 41)
                        * unit
                        + ((x * 17 + y * 3 + c as u32 * 11) % unit as u32) as i32)
                        as u16
                })
                .collect()
        })
        .collect()
}

struct BlockStages {
    bytes: Vec<Vec<u8>>,
    passes: Vec<u16>,
    missing: u8,
}
fn stage_bytes(b: &BlockStages, stage: Option<usize>) -> &[u8] {
    stage.map_or(&[], |stage| b.bytes[stage].as_slice())
}
struct BandStages {
    spec: DecompSubbandSpec,
    cols: u16,
    rows: u16,
    blocks: Vec<BlockStages>,
}
struct Representation {
    header: Vec<u8>,
    packets: Vec<u8>,
    ends: Vec<usize>,
    stage_layers: Vec<u16>,
    resolution_ends: Vec<usize>,
}
impl Representation {
    fn prefix(&self, stage: usize) -> Vec<u8> {
        let mut raw = self.header.clone();
        let cod = find_marker(&raw, 0, Marker::Cod).unwrap();
        raw[cod + 6..cod + 8].copy_from_slice(&self.stage_layers[stage].to_be_bytes());
        write_tile_part(&mut raw, 0, &self.packets[..self.ends[stage]], true).unwrap();
        raw
    }
    fn resolution_prefix(&self, discard: u8) -> Vec<u8> {
        let parsed = parse(&self.prefix(0)).unwrap();
        let mut packets =
            self.packets[..self.resolution_ends[usize::from(LEVELS - discard)]].to_vec();
        // Local decode envelope: absent higher-resolution packets are empty.
        // Every delivered packet byte is an unchanged prefix of the one source.
        packets.resize(
            packets.len() + usize::from(discard) * usize::from(parsed.siz.component_count()),
            0,
        );
        let mut raw = self.header.clone();
        write_tile_part(&mut raw, 0, &packets, true).unwrap();
        raw
    }
    fn stored(&self) -> usize {
        self.header.len() + self.packets.len() + 16
    }
}

fn prepare(source: &[Vec<u16>], bits: u8, ht: bool, shifts: &[u8]) -> Representation {
    let mut planes: Vec<Vec<f32>> = source
        .iter()
        .map(|p| {
            p.iter()
                .map(|&v| f32::from(v) - (1_u32 << (bits - 1)) as f32)
                .collect()
        })
        .collect();
    ht_lossy::analyse_levels(EDGE, EDGE, &mut planes, LEVELS).unwrap();
    let specs = decomp_subband_specs(EDGE, EDGE, LEVELS).unwrap();
    // One fixed unit quantiser at native precision; all endpoints share it.
    let steps: Vec<_> = specs
        .iter()
        .map(|s| transform::IrreversibleQuantizationStep::new(bits + gain(s.kind), 0).unwrap())
        .collect();
    let mut components = Vec::new();
    for plane in planes {
        let quantised: Vec<i32> = plane.iter().map(|v| *v as i32).collect();
        let mut bands = Vec::new();
        for spec in &specs {
            let depth = bits + gain(spec.kind) + if ht { 2 } else { 1 };
            let mut scratch = tier1::CodeBlockEncodeScratch::new();
            let mut segments = Vec::new();
            let metadata =
                encode_decomp_subband(EDGE, &quantised, *spec, depth, &mut segments, &mut scratch)
                    .unwrap();
            let mut blocks = Vec::new();
            for b in &metadata.code_blocks {
                if ht {
                    let mut bytes = Vec::new();
                    for &shift in shifts {
                        let values: Vec<i32> = (0..u32::from(b.height))
                            .flat_map(|y| {
                                let q = &quantised;
                                (0..u32::from(b.width)).map(move |x| {
                                    let v = q[((spec.y + u32::from(b.y) * 64 + y) * EDGE
                                        + spec.x
                                        + u32::from(b.x) * 64
                                        + x)
                                        as usize];
                                    let m = (v.unsigned_abs() >> shift) as i32;
                                    if v < 0 { -m } else { m }
                                })
                            })
                            .collect();
                        bytes.push(
                            ht::encode_ht_cleanup_block(
                                &values,
                                b.width,
                                b.height,
                                usize::from(b.width),
                                depth,
                            )
                            .unwrap()
                            .map_or_else(Vec::new, |v| v.segment),
                        );
                    }
                    let first = bytes.iter().position(|b| !b.is_empty());
                    let missing = first.map_or(0, |f| depth - 1 - shifts[f]);
                    let passes = shifts
                        .iter()
                        .enumerate()
                        .map(|(i, &shift)| {
                            first
                                .filter(|&f| i >= f)
                                .map_or(0, |f| 1 + 3 * u16::from(shifts[f] - shift))
                        })
                        .collect();
                    blocks.push(BlockStages {
                        bytes,
                        passes,
                        missing,
                    });
                } else {
                    let full = &segments[b.segment_offset..b.segment_offset + b.segment_len];
                    if !b.included {
                        blocks.push(BlockStages {
                            bytes: vec![Vec::new(); shifts.len()],
                            passes: vec![0; shifts.len()],
                            missing: 0,
                        });
                        continue;
                    }
                    let spec = tier1::CodeBlockDecodeSpec {
                        dimensions: tier1::CodeBlockDimensions::new(b.width, b.height).unwrap(),
                        available_bitplanes: depth,
                        missing_most_significant_bitplanes: b.missing_bitplanes - 1,
                        coding_passes: b.coding_passes,
                        style: tier1::CodeBlockStyle::NONE,
                        subband: spec.kind.tier1_subband(),
                    };
                    let lengths =
                        tier1::baseline_pass_prefix_lengths_test_fixture(full, spec).unwrap();
                    let mut bytes = Vec::new();
                    let mut passes = Vec::new();
                    let mut previous = (0, 0);
                    for &shift in shifts {
                        let wanted = b.coding_passes.saturating_sub(3 * u16::from(shift));
                        let endpoint = if shift == 0 {
                            (b.coding_passes, full.len())
                        } else {
                            lengths
                                .iter()
                                .enumerate()
                                .filter(|(i, n)| {
                                    *i < usize::from(wanted) && **n < full.len() && **n > 0
                                })
                                .map(|(i, &n)| (i as u16 + 1, n))
                                .next_back()
                                .unwrap_or(previous)
                        };
                        let ds = tier1::CodeBlockDecodeSpec {
                            coding_passes: endpoint.0,
                            ..spec
                        };
                        if endpoint.0 > 0 {
                            let mut actual = vec![0; usize::from(b.width) * usize::from(b.height)];
                            let mut reference = actual.clone();
                            tier1::decode_baseline_code_block(&full[..endpoint.1], ds, &mut actual)
                                .unwrap();
                            tier1::decode_baseline_code_block(full, ds, &mut reference).unwrap();
                            assert_eq!(actual, reference, "actual MQ prefix differs");
                        }
                        bytes.push(full[previous.1..endpoint.1].to_vec());
                        passes.push(endpoint.0);
                        previous = endpoint;
                    }
                    blocks.push(BlockStages {
                        bytes,
                        passes,
                        missing: b.missing_bitplanes - 1,
                    });
                }
            }
            bands.push(BandStages {
                spec: *spec,
                cols: metadata.code_block_cols,
                rows: metadata.code_block_rows,
                blocks,
            });
        }
        components.push(bands);
    }
    let mut header = Vec::new();
    write_irreversible_main_header(
        &mut header,
        EDGE,
        EDGE,
        bits,
        source.len() as u16,
        false,
        LEVELS,
        &steps,
        ht,
    )
    .unwrap();
    if ht && shifts.len() > 1 {
        let cap = find_marker(&header, 0, Marker::Cap).unwrap();
        header[cap + 8..cap + 10].copy_from_slice(&0x202a_u16.to_be_bytes());
    }
    // Put each HT segment in its own packet contribution. In particular,
    // a later empty refinement or cleanup is a zero-length segment, not an
    // initial placeholder folded into the following non-empty cleanup.
    // Part 15:2019 B.2–B.3, PDF40–41; Part 1:2024 B.10.7.2, PDF93.
    let rounds = if ht {
        2 * usize::from(shifts[0]) + 1
    } else {
        shifts.len()
    };
    let first_round = |b: &BlockStages| {
        b.bytes
            .iter()
            .position(|v| !v.is_empty())
            .map_or(rounds, |stage| {
                if ht {
                    2 * usize::from(shifts[0] - shifts[stage])
                } else {
                    stage
                }
            })
    };
    let mut states: Vec<Vec<_>> = components
        .iter()
        .map(|bands| {
            bands
                .iter()
                .map(|band| {
                    (
                        EncTagTree::new(
                            band.cols,
                            band.rows,
                            band.blocks.iter().map(|b| first_round(b) as u32),
                        )
                        .unwrap(),
                        EncTagTree::new(
                            band.cols,
                            band.rows,
                            band.blocks.iter().map(|b| u32::from(b.missing)),
                        )
                        .unwrap(),
                        vec![3_u8; band.blocks.len()],
                    )
                })
                .collect()
        })
        .collect();
    let mut packets = Vec::new();
    let mut ends = Vec::new();
    let mut stage_layers = Vec::new();
    let mut resolution_ends = Vec::new();
    for round in 0..rounds {
        let stage = if ht {
            if round.is_multiple_of(2) {
                shifts
                    .iter()
                    .position(|&v| v == shifts[0] - (round / 2) as u8)
            } else {
                None
            }
        } else {
            Some(round)
        };

        let contributes = |b: &BlockStages| {
            if ht {
                first_round(b) <= round
            } else {
                !stage_bytes(b, stage).is_empty()
            }
        };
        for resolution in 0..=LEVELS {
            for (c, bands) in components.iter().enumerate() {
                let mut writer = PacketBitWriter::new();
                let present = bands
                    .iter()
                    .filter(|b| b.spec.resolution == resolution)
                    .any(|b| b.blocks.iter().any(contributes));
                writer.write_bit(u32::from(present)).unwrap();
                let mut body = Vec::new();
                if present {
                    for (bi, band) in bands
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| b.spec.resolution == resolution)
                    {
                        let (inclusion, missing, lblocks) = &mut states[c][bi];
                        for (i, b) in band.blocks.iter().enumerate() {
                            let first = first_round(b);
                            let present = contributes(b);
                            let (x, y) = (i as u16 % band.cols, i as u16 / band.cols);
                            if round <= first {
                                inclusion
                                    .encode(&mut writer, x, y, round as u32 + 1)
                                    .unwrap();
                            } else {
                                writer.write_bit(u32::from(present)).unwrap();
                            }
                            if !present {
                                continue;
                            }
                            if round == first {
                                missing.encode(&mut writer, x, y, u32::MAX).unwrap();
                            }
                            let passes = if ht {
                                if round.is_multiple_of(2) { 1 } else { 2 }
                            } else {
                                b.passes[round] - if round == 0 { 0 } else { b.passes[round - 1] }
                            };
                            let bytes = stage_bytes(b, stage);
                            write_coding_pass_count(&mut writer, passes).unwrap();
                            let extra = passes.ilog2() as u8;
                            while bytes.len() >= (1_usize << (lblocks[i] + extra)) {
                                writer.write_bit(1).unwrap();
                                lblocks[i] += 1;
                            }
                            writer.write_bit(0).unwrap();
                            writer
                                .write_bits(bytes.len() as u32, lblocks[i] + extra)
                                .unwrap();
                            body.extend_from_slice(bytes);
                        }
                    }
                }
                writer.align();
                packets.extend_from_slice(writer.bytes());
                packets.extend_from_slice(&body);
            }
            if round == 0 {
                resolution_ends.push(packets.len());
            }
        }
        if stage.is_some() {
            ends.push(packets.len());
            stage_layers.push(round as u16 + 1);
        }
    }
    Representation {
        header,
        packets,
        ends,
        stage_layers,
        resolution_ends,
    }
}
fn gain(kind: PacketSubbandKind) -> u8 {
    match kind {
        PacketSubbandKind::LowLow => 0,
        PacketSubbandKind::HighHigh => 2,
        _ => 1,
    }
}

struct Observation {
    planes: Vec<Vec<u16>>,
    entropy: usize,
    coefficients: usize,
    passes: usize,
    entropy_us: u128,
    synthesis_us: u128,
    synthesis_samples: usize,
    decode_us: u128,
}
fn decode(raw: &[u8], ht: bool, discard: u8) -> Observation {
    let total = Instant::now();
    let parsed = parse(raw).unwrap();
    let tile = tile_rects(&parsed).unwrap()[0];
    let payload = tile_payload(raw, &parsed.tiles[0]).unwrap();
    let contributions = parse_default_precinct_lrcp_packets(raw, &parsed, tile, payload).unwrap();
    let side = EDGE >> discard;
    let reduced_specs = decomp_subband_specs(side, side, LEVELS - discard).unwrap();
    let mut planes =
        vec![vec![0.0; (side * side) as usize]; usize::from(parsed.siz.component_count())];
    let candidate = ht.then(|| {
        ht_decode_candidate_with_transform_permission(&parsed, true)
            .unwrap()
            .unwrap()
    });
    let mut ws = HtCodestreamBlockDecodeWorkspace::default();
    let mut tws = tier1::CodeBlockDecodeScratch::new();
    let mut segment_scratch = Vec::new();
    let mut entropy = 0;
    let mut coefficients = 0;
    let mut passes = 0;
    let started = Instant::now();
    for original in contributions
        .iter()
        .filter(|c| c.resolution <= LEVELS - discard)
    {
        let mut c = original.clone();
        let spec = &reduced_specs[usize::from(c.subband_index)];
        c.x = spec.x + u32::from(c.code_block_x) * 64;
        c.y = spec.y + u32::from(c.code_block_y) * 64;
        let segment =
            code_block_segment_for_decode(payload, original, &mut segment_scratch).unwrap();
        let mut output = vec![0; usize::from(c.width) * usize::from(c.height)];
        coefficients += output.len();
        let delta = c
            .irreversible_quantization_step
            .unwrap()
            .delta(parsed.siz.components[0].bits_per_sample, gain(c.subband))
            .unwrap();
        if ht {
            let set = c
                .ht_coding_sets()
                .filter(|s| s.byte_len > 0)
                .last()
                .unwrap();
            let bytes = &segment[set.byte_offset..set.byte_offset + set.byte_len];
            entropy += bytes.len();
            passes += usize::from(set.coding_passes);
            let input = HtCodestreamCodeBlockInput {
                candidate: candidate.unwrap(),
                active_dimensions: ht::HtCodeBlockDimensions::new(c.width, c.height).unwrap(),
                missing_most_significant_bitplanes: set.missing_most_significant_bitplanes,
                coding_passes: set.coding_passes,
                segment_layout: ht::HtCleanupPassSegmentLayout::from_cleanup_pass_bytes(bytes)
                    .unwrap(),
                segment: bytes,
            };
            assert!(
                ws.decode_code_block_input_into_with_progress(input, &mut output)
                    .unwrap()
                    .0
                    .is_some()
            );
            place_ht_irreversible_code_block_coefficients(
                &mut planes[usize::from(c.component_index)],
                side as usize,
                &c,
                &output,
                0.5 * delta,
                None,
            )
            .unwrap();
        } else {
            entropy += segment.len();
            passes += usize::from(c.coding_passes);
            let spec = code_block_decode_spec(parsed.uniform_effective_coding_style().unwrap(), &c)
                .unwrap();
            tier1::decode_irreversible_code_block_with_scratch(
                segment,
                spec,
                &mut output,
                &mut tws,
            )
            .unwrap();
            place_irreversible_code_block_coefficients(
                &mut planes[usize::from(c.component_index)],
                side as usize,
                &c,
                &output,
                0.5 * delta,
            )
            .unwrap();
        }
    }
    let entropy_us = started.elapsed().as_micros();
    let start = Instant::now();
    let mut scratch = vec![0.0; side as usize * 2];
    for p in &mut planes {
        inverse_irreversible_9_7_levels_with_scratch(
            p,
            side as usize,
            side,
            side,
            LEVELS - discard,
            &mut scratch,
        )
        .unwrap();
    }
    let synthesis_us = start.elapsed().as_micros();
    let synthesis_samples = (1..=LEVELS - discard)
        .map(|r| {
            let s = side >> (LEVELS - discard - r);
            (s * s) as usize
        })
        .sum::<usize>()
        * planes.len();
    let planes = planes
        .iter()
        .enumerate()
        .map(|(c, p)| {
            irreversible_component_samples_to_bytes(&parsed.siz.components[c], p)
                .unwrap()
                .chunks_exact(2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect()
        })
        .collect();
    Observation {
        planes,
        entropy,
        coefficients,
        passes,
        entropy_us,
        synthesis_us,
        synthesis_samples,
        decode_us: total.elapsed().as_micros(),
    }
}

fn metrics(source: &[Vec<u16>], o: &Observation, discard: u8) -> (f64, u16, f64, f64) {
    let side = EDGE >> discard;
    let mut sse = 0_u128;
    let mut peak = 0;
    let mut object = 0_u128;
    let mut faint = 0_u128;
    for (p, q) in source.iter().zip(&o.planes) {
        for y in 0..EDGE {
            for x in 0..EDGE {
                let d = p[(y * EDGE + x) as usize]
                    .abs_diff(q[((y >> discard) * side + (x >> discard)) as usize]);
                sse += u128::from(d).pow(2);
                peak = peak.max(d);
                if (35..95).contains(&x) && (55..115).contains(&y) {
                    faint += u128::from(d).pow(2);
                }
                if (119..122).contains(&x) && (143..146).contains(&y) {
                    object += u128::from(d).pow(2);
                }
            }
        }
    }
    (
        (sse as f64 / (EDGE * EDGE) as f64 / source.len() as f64).sqrt(),
        peak,
        (object as f64 / (9 * source.len()) as f64).sqrt(),
        (faint as f64 / (3600 * source.len()) as f64).sqrt(),
    )
}

#[test]
fn genuine_ht_quality_prefixes_preserve_final_native_reconstruction() {
    for bits in [11, 16] {
        for components in [1, 3] {
            let source = source(bits, components);
            let shifts = shifts(bits);
            let multi = prepare(&source, bits, true, &shifts);
            let part1 = prepare(&source, bits, false, &shifts);
            let single = prepare(&source, bits, true, &[0]);
            let mut previous = f64::INFINITY;
            for stage in 0..3 {
                let raw = multi.prefix(stage);
                validate_part15_packet_signalling(&raw, &parse(&raw).unwrap()).unwrap();
                let actual = decode(&raw, true, 0);
                assert_eq!(actual.planes, decode(&part1.prefix(stage), false, 0).planes);
                let error = metrics(&source, &actual, 0).0;
                assert!(error < previous);
                previous = error;
                if stage == 2 {
                    assert_eq!(actual.planes, decode(&single.prefix(0), true, 0).planes);
                }
                let mut truncated = raw.clone();
                truncated.truncate(truncated.len() - 3);
                assert!(parse(&truncated).is_err());
            }
            let two = prepare(&source, bits, true, &[shifts[0], 0]);
            assert_eq!(
                decode(&two.prefix(1), true, 0).planes,
                decode(&single.prefix(0), true, 0).planes
            );
            for discard in [1, 2] {
                assert_eq!(
                    decode(&single.resolution_prefix(discard), true, discard).planes,
                    decode(&single.prefix(0), true, discard).planes
                );
            }
        }
    }
}

#[test]
#[ignore = "quality calibration; optional authored images belong in registered scratch"]
fn genuine_ht_quality_calibration() {
    let output = std::env::var_os("EMUELLA_HT_QUALITY_OUTPUT").map(std::path::PathBuf::from);
    println!(
        "bits,components,kind,stage,packet_layers,discard,stored_bytes,delivered_bytes,entropy_bytes,coefficients,passes,synthesis_samples,entropy_us,synthesis_us,decode_us,rmse,peak,object_rmse,faint_rmse,input_sha256"
    );
    for bits in [11, 16] {
        for components in [1, 3] {
            let source = source(bits, components);
            let identity: Vec<u8> = source
                .iter()
                .flatten()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let hash = Sha256::digest(&identity)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect::<String>();
            let shifts = shifts(bits);
            let two = [shifts[0], 0];
            for (kind, ht, shifts) in [
                ("multi_ht", true, shifts.as_slice()),
                ("two_ht", true, two.as_slice()),
                ("part1", false, shifts.as_slice()),
                ("resolution", true, &[0][..]),
            ] {
                let encoded = prepare(&source, bits, ht, shifts);
                for stage in 0..if kind == "resolution" {
                    3
                } else {
                    shifts.len()
                } {
                    let discard = if kind == "resolution" {
                        2 - stage as u8
                    } else {
                        0
                    };
                    let raw = if kind == "resolution" {
                        encoded.resolution_prefix(discard)
                    } else {
                        encoded.prefix(stage)
                    };
                    let packet_layers = if kind == "resolution" {
                        1
                    } else {
                        encoded.stage_layers[stage]
                    };
                    let mut observations: Vec<_> =
                        (0..7).map(|_| decode(&raw, ht, discard)).collect();
                    observations.sort_by_key(|o| o.decode_us);
                    let o = observations.swap_remove(3);
                    let (rmse, peak, object, faint) = metrics(&source, &o, discard);
                    let delivered = if kind == "resolution" {
                        encoded.header.len()
                            + 16
                            + encoded.resolution_ends[usize::from(LEVELS - discard)]
                    } else {
                        raw.len()
                    };
                    println!(
                        "{bits},{components},{kind},{stage},{packet_layers},{discard},{},{delivered},{},{},{},{},{},{},{},{rmse:.6},{peak},{object:.6},{faint:.6},{hash}",
                        encoded.stored(),
                        o.entropy,
                        o.coefficients,
                        o.passes,
                        o.synthesis_samples,
                        o.entropy_us,
                        o.synthesis_us,
                        o.decode_us
                    );
                    if let Some(dir) = &output {
                        let side = EDGE >> discard;
                        let mut ppm = format!("P6\n{side} {side}\n255\n").into_bytes();
                        for i in 0..(side * side) as usize {
                            for c in 0..3 {
                                ppm.push(
                                    (u32::from(o.planes[c.min(components - 1)][i]) * 255
                                        / ((1 << bits) - 1))
                                        as u8,
                                );
                            }
                        }
                        std::fs::write(
                            dir.join(format!("u{bits}-c{components}-{kind}-{stage}.ppm")),
                            ppm,
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}
