//! Differential checks against the independent, pre-change arithmetic snapshot.
use super::*;
use crate::{encode_trace::Event, mq_reference as reference};
use alloc::vec;

fn checkpoint(decoder: &Decoder<'_>) -> (u32, u32, usize, u32, u8) {
    (
        decoder.interval,
        decoder.code,
        decoder.cursor,
        decoder.bits_available,
        decoder.synthetic_marker_reads,
    )
}

fn contexts_equal(
    actual: &[Context; CONTEXT_COUNT],
    expected: &[reference::Context; CONTEXT_COUNT],
) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!((actual.state, actual.mps), expected.checkpoint());
    }
}

fn compare_decision(
    decoder: &mut Decoder<'_>,
    oracle: &mut reference::Decoder<'_>,
    contexts: &mut [Context; CONTEXT_COUNT],
    expected: &mut [reference::Context; CONTEXT_COUNT],
    label: usize,
    by_value: bool,
) -> u32 {
    let (bit, original) = if by_value {
        let (bit, next) = decoder.read_packed_bit_value(contexts[label]);
        contexts[label] = next;
        let (original, next) = oracle.read_bit_value(expected[label]);
        expected[label] = next;
        (bit, original)
    } else {
        (
            decoder.read_packed_bit(&mut contexts[label]),
            oracle.read_bit(&mut expected[label]),
        )
    };
    assert_eq!(bit, original);
    contexts_equal(contexts, expected);
    assert_eq!(checkpoint(decoder), oracle.checkpoint());
    assert_eq!(decoder.consumed_prefix_len(), oracle.consumed_prefix_len());
    assert_eq!(
        decoder.validate_predictable_termination(),
        oracle.validate_predictable_termination()
    );
    bit
}

#[test]
fn all_two_byte_inputs_preserve_decisions_registers_and_synthetic_boundaries() {
    for word in 0..=u16::MAX {
        let bytes = word.to_be_bytes();
        let mut actual = Decoder::new(&bytes);
        let mut expected = reference::Decoder::new(&bytes);
        let mut contexts = initial_contexts();
        let mut original = reference::initial_contexts();
        assert_eq!(checkpoint(&actual), expected.checkpoint());
        for step in 0..38 {
            compare_decision(
                &mut actual,
                &mut expected,
                &mut contexts,
                &mut original,
                step % CONTEXT_COUNT,
                step % 2 == 0,
            );
        }
    }
}

#[test]
fn authored_reachable_sequences_preserve_every_checkpoint_and_prefix() {
    let mut bytes = Vec::new();
    let mut decisions = Vec::new();
    let mut encoder = reference::Encoder::new(&mut bytes);
    let mut random = 0x9637_41a5_u32;
    for _ in 0..100_000 {
        encoder.write_bit(0, 0);
        decisions.push((0, 0));
    }
    // Deliberately reach the secondary probability branch before the broad
    // distribution sweep; long MPS runs alone bypass these states.
    for _ in 0..16 {
        if encoder.context_checkpoint(1).0 == 1 {
            break;
        }
        let bit = u32::from(encoder.context_checkpoint(1).1);
        encoder.write_bit(1, bit);
        decisions.push((1, bit));
    }
    assert_eq!(encoder.context_checkpoint(1).0, 1);
    let bit = u32::from(encoder.context_checkpoint(1).1 ^ 1);
    encoder.write_bit(1, bit);
    decisions.push((1, bit));
    assert_eq!(encoder.context_checkpoint(1).0, 6);
    for _ in 0..4096 {
        if encoder.context_checkpoint(1).0 == 13 {
            break;
        }
        let bit = u32::from(encoder.context_checkpoint(1).1);
        encoder.write_bit(1, bit);
        decisions.push((1, bit));
    }
    assert_eq!(encoder.context_checkpoint(1).0, 13);
    for phase in 0..32 {
        for step in 0..2048 {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            let label = (step % CONTEXT_COUNT) as u8;
            let bit = u32::from((random & 255) < (phase * 8));
            encoder.write_bit(label, bit);
            decisions.push((label, bit));
        }
    }
    encoder.finish();
    let mut seen = [[false; 2]; 47];
    let mut actual = Decoder::new(&bytes);
    let mut expected = reference::Decoder::new(&bytes);
    let mut contexts = initial_contexts();
    let mut original = reference::initial_contexts();
    for (step, &(label, bit)) in decisions.iter().enumerate() {
        assert_eq!(
            compare_decision(
                &mut actual,
                &mut expected,
                &mut contexts,
                &mut original,
                usize::from(label),
                step % 2 == 0
            ),
            bit
        );
        for context in contexts {
            seen[usize::from(context.state)][usize::from(context.mps)] = true;
        }
    }
    // Uniform context 46 has no transition that changes its initial MPS.
    assert!(
        seen.iter().all(|mps| mps[0] || mps[1]),
        "every probability state must be reached; missing {:?}",
        seen.iter()
            .enumerate()
            .filter(|(_, mps)| !mps[0] && !mps[1])
            .map(|(state, _)| state)
            .collect::<Vec<_>>()
    );
    for end in [
        0,
        1,
        2,
        3,
        bytes.len() / 2,
        bytes.len() - 2,
        bytes.len() - 1,
        bytes.len(),
    ] {
        let mut actual = Decoder::new(&bytes[..end]);
        let mut expected = reference::Decoder::new(&bytes[..end]);
        let mut contexts = initial_contexts();
        let mut original = reference::initial_contexts();
        for step in 0..4096 {
            compare_decision(
                &mut actual,
                &mut expected,
                &mut contexts,
                &mut original,
                step % CONTEXT_COUNT,
                step % 2 == 0,
            );
        }
    }
}

/// Replay authored block decisions through the independent writer and both
/// decoders, retaining each segment boundary and context reset.
pub(crate) fn assert_trace(trace: &[Event], bytes: &[u8]) {
    let mut original_bytes = Vec::new();
    let mut encoder = reference::Encoder::new(&mut original_bytes);
    let mut contexts = initial_contexts();
    let mut original = reference::initial_contexts();
    let mut offset = 0;
    let mut pending = Vec::new();
    for event in trace {
        match *event {
            Event::Mq { label, bit, .. } => {
                encoder.write_bit(label, bit);
                pending.push(event);
            }
            Event::Raw { bit, .. } => {
                encoder.write_raw_bit(bit);
                pending.push(event);
            }
            Event::Reset => {
                encoder.reset_contexts();
                pending.push(event);
            }
            Event::Restart { raw } => {
                if raw {
                    encoder.restart_raw_segment();
                } else {
                    encoder.restart_segment();
                }
            }
            Event::Pass(..) => {}
            Event::Finish {
                raw,
                predictable,
                length,
            } => {
                if raw {
                    encoder.finish_raw(predictable);
                } else if predictable {
                    encoder.finish_predictable();
                } else {
                    encoder.finish();
                }
                assert_eq!(encoder.current_segment_len(), length);
                let segment = &bytes[offset..offset + length];
                let mut decoder = Decoder::new(segment);
                let mut oracle = reference::Decoder::new(segment);
                let mut raw_decoder = RawDecoder::new(segment);
                let mut raw_oracle = reference::RawDecoder::new(segment);
                for (step, event) in pending.drain(..).enumerate() {
                    match *event {
                        Event::Mq { label, bit, .. } => assert_eq!(
                            compare_decision(
                                &mut decoder,
                                &mut oracle,
                                &mut contexts,
                                &mut original,
                                usize::from(label),
                                step % 2 == 0
                            ),
                            bit
                        ),
                        Event::Raw { bit, .. } => {
                            assert_eq!(raw_decoder.read_bit(), bit);
                            assert_eq!(raw_oracle.read_bit(), bit);
                            assert_eq!(
                                (
                                    raw_decoder.cursor,
                                    raw_decoder.current,
                                    raw_decoder.bits_available
                                ),
                                raw_oracle.checkpoint()
                            );
                        }
                        Event::Reset => {
                            reset_contexts(&mut contexts);
                            reference::reset_contexts(&mut original);
                        }
                        _ => unreachable!(),
                    }
                }
                offset += length;
            }
        }
    }
    assert!(pending.is_empty());
    assert_eq!(offset, bytes.len());
    assert_eq!(original_bytes, bytes);
}

#[test]
fn raw_reader_preserves_stuffing_marker_and_empty_boundaries() {
    for bytes in [
        vec![],
        vec![0xff],
        vec![0xff, 0x8f],
        vec![0xff, 0x90],
        vec![0xff, 0xff],
        vec![0, 0xff, 0x7f, 0xff, 0x90],
    ] {
        let mut actual = RawDecoder::new(&bytes);
        let mut expected = reference::RawDecoder::new(&bytes);
        for _ in 0..80 {
            assert_eq!(actual.read_bit(), expected.read_bit());
            assert_eq!(
                (actual.cursor, actual.current, actual.bits_available),
                expected.checkpoint()
            );
        }
    }
}
