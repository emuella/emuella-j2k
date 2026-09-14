//! Authored style-zero/bypass encoder replay; no corpus input.
//! Run identical source against reference and candidate builds and compare output.
use emuella_j2k_tier1::{
    CodeBlockDecodeScratch, CodeBlockDecodeSpec, CodeBlockDimensions, CodeBlockEncodeScratch,
    CodeBlockEncodeSpec, CodeBlockSegment, CodeBlockStyle, Subband,
    decode_baseline_code_block_segments,
    decode_baseline_code_block_segments_with_packed_scratch_outcome,
    decode_baseline_code_block_segments_with_sparse_scratch_outcome,
    encode_baseline_code_block_segments_with_scratch,
};
use std::{hint::black_box, time::Instant};

struct Replay {
    coefficients: Vec<i32>,
    encode_spec: CodeBlockEncodeSpec,
    decode_spec: CodeBlockDecodeSpec,
    bytes: Vec<u8>,
    lengths: Vec<usize>,
    segments: Vec<CodeBlockSegment>,
}

fn cases(pattern: usize, style: u8) -> Vec<Replay> {
    let mut cases = Vec::new();
    for (width, height) in [(64_usize, 64_usize), (17, 11)] {
        for bits in [1, 8, 16, 24, 30] {
            for subband in [
                Subband::LowLow,
                Subband::LowHigh,
                Subband::HighLow,
                Subband::HighHigh,
            ] {
                let mask = (1_u32 << bits) - 1;
                let mut state = 0x6c21_7d93_u32;
                let coefficients = (0..width * height)
                    .map(|index| {
                        state ^= state << 13;
                        state ^= state >> 17;
                        state ^= state << 5;
                        let x = index % width;
                        let y = index / width;
                        let active = match pattern {
                            0 => true,
                            1 => index.is_multiple_of(31),
                            2 => (x / 4 + y / 4).is_multiple_of(2),
                            _ => x == y || x + y == width - 1,
                        };
                        let magnitude = if active { (state & mask) as i32 } else { 0 };
                        if state & (1 << 31) == 0 {
                            magnitude
                        } else {
                            -magnitude
                        }
                    })
                    .collect::<Vec<_>>();
                let encode_spec = CodeBlockEncodeSpec {
                    dimensions: CodeBlockDimensions::new(width as u16, height as u16).unwrap(),
                    subband,
                    available_bitplanes: bits,
                    code_block_style: style,
                };
                let mut bytes = Vec::new();
                let mut lengths = Vec::new();
                let encoded = encode_baseline_code_block_segments_with_scratch(
                    &coefficients,
                    encode_spec,
                    &mut bytes,
                    &mut lengths,
                    &mut CodeBlockEncodeScratch::new(),
                )
                .unwrap();
                let decode_spec = CodeBlockDecodeSpec {
                    dimensions: encode_spec.dimensions,
                    subband,
                    available_bitplanes: bits,
                    missing_most_significant_bitplanes: encoded.missing_bitplanes,
                    coding_passes: encoded.pass_count,
                    style: CodeBlockStyle::from_bits(style),
                };
                let mut pass = 0_u16;
                let segments = lengths
                    .iter()
                    .map(|&byte_len| {
                        let remaining = encoded.pass_count - pass;
                        let coding_passes = if style == 0 {
                            remaining
                        } else if pass == 0 {
                            10.min(remaining)
                        } else if pass % 3 == 1 {
                            2.min(remaining)
                        } else {
                            1
                        };
                        pass += coding_passes;
                        CodeBlockSegment {
                            byte_len,
                            coding_passes,
                        }
                    })
                    .collect();
                cases.push(Replay {
                    coefficients,
                    encode_spec,
                    decode_spec,
                    bytes,
                    lengths,
                    segments,
                });
            }
        }
    }
    cases
}

fn decode(case: &Replay, backend: usize, output: &mut [i32], scratch: &mut CodeBlockDecodeScratch) {
    match backend {
        0 => {
            decode_baseline_code_block_segments(
                &case.bytes,
                &case.segments,
                case.decode_spec,
                output,
            )
            .unwrap();
        }
        1 => {
            decode_baseline_code_block_segments_with_packed_scratch_outcome(
                &case.bytes,
                &case.segments,
                case.decode_spec,
                output,
                scratch,
            )
            .unwrap();
        }
        _ => {
            decode_baseline_code_block_segments_with_sparse_scratch_outcome(
                &case.bytes,
                &case.segments,
                case.decode_spec,
                output,
                scratch,
            )
            .unwrap();
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let repeats = args
        .next()
        .map_or(8, |value| value.parse::<usize>().unwrap());
    assert!(repeats > 0);
    let byte_output = args.next();
    assert!(
        args.next().is_none(),
        "usage: encoder_replay [repeats] [authored-byte-output]"
    );
    let mut byte_identity = Vec::new();
    for (pattern, name) in ["dense", "sparse", "patches", "diagonals"]
        .into_iter()
        .enumerate()
    {
        for style in [0, 1] {
            let cases = cases(pattern, style);
            for case in &cases {
                byte_identity.push(style);
                byte_identity.extend_from_slice(&case.decode_spec.coding_passes.to_le_bytes());
                byte_identity.push(case.decode_spec.missing_most_significant_bitplanes);
                byte_identity.extend_from_slice(&(case.lengths.len() as u64).to_le_bytes());
                for &length in &case.lengths {
                    byte_identity.extend_from_slice(&(length as u64).to_le_bytes());
                }
                byte_identity.extend_from_slice(&(case.bytes.len() as u64).to_le_bytes());
                byte_identity.extend_from_slice(&case.bytes);
            }
            let mut encode_scratch = CodeBlockEncodeScratch::new();
            let mut encoded = Vec::new();
            let mut lengths = Vec::new();
            // Warm scratch and verify deterministic bytes before the timed loop.
            for case in &cases {
                encoded.clear();
                encode_baseline_code_block_segments_with_scratch(
                    &case.coefficients,
                    case.encode_spec,
                    &mut encoded,
                    &mut lengths,
                    &mut encode_scratch,
                )
                .unwrap();
                assert_eq!(encoded, case.bytes);
                assert_eq!(lengths, case.lengths);
            }
            let start = Instant::now();
            for _ in 0..repeats {
                for case in &cases {
                    encoded.clear();
                    black_box(
                        encode_baseline_code_block_segments_with_scratch(
                            black_box(&case.coefficients),
                            case.encode_spec,
                            &mut encoded,
                            &mut lengths,
                            &mut encode_scratch,
                        )
                        .unwrap(),
                    );
                    black_box(&encoded);
                }
            }
            println!(
                "pattern={name} style={style} operation=encode blocks={} elapsed_ns={}",
                repeats * cases.len(),
                start.elapsed().as_nanos()
            );
            for (backend_index, backend) in ["checked", "dense", "sparse"].into_iter().enumerate() {
                let mut scratch = CodeBlockDecodeScratch::new();
                let mut decoded = Vec::new();
                for case in &cases {
                    decoded.resize(case.coefficients.len(), 0);
                    decode(case, backend_index, &mut decoded, &mut scratch);
                    assert_eq!(decoded, case.coefficients, "{name} {backend}");
                }
            }
        }
    }
    if let Some(path) = byte_output {
        std::fs::write(path, byte_identity).unwrap();
    }
}
