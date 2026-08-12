#![no_main]

use emuella_j2k_ht::{
    HtCleanupPredestuffBenchScratch, HtCleanupReaderBenchResult,
    checked_magsgn_reader_workload_for_bench, checked_mel_reader_workload_for_bench,
    checked_vlc_reader_workload_for_bench, fast_magsgn_reader_workload_for_bench,
    fast_mel_reader_workload_for_bench, fast_vlc_reader_workload_for_bench,
    predestuffed_magsgn_reader_workload_for_bench, predestuffed_vlc_reader_workload_for_bench,
};
use libfuzzer_sys::fuzz_target;

const MAX_STREAM_BYTES: usize = 4 * 1024;
const MAX_READS: usize = 128;

fuzz_target!(|data: &[u8]| {
    let data = &data[..data.len().min(MAX_STREAM_BYTES * 2 + MAX_READS + 1)];
    let partition = data.len() / 3;
    let vlc = &data[..partition];
    let magsgn = &data[partition..partition * 2];
    let plan = &data[partition * 2..];
    let widths = plan
        .iter()
        .copied()
        .take(MAX_READS)
        .map(|value| value % 17)
        .collect::<Vec<_>>();

    assert_eq!(
        fast_vlc_reader_workload_for_bench(vlc, &widths),
        checked_vlc_reader_workload_for_bench(vlc, &widths),
        "on-demand VLC reader diverged from checked reader"
    );

    let mut predestuff = HtCleanupPredestuffBenchScratch::default();
    let prepared_vlc = predestuffed_vlc_reader_workload_for_bench(&mut predestuff, vlc, &widths);
    let checked_vlc = checked_vlc_reader_workload_for_bench(vlc, &widths);
    if let (Ok(prepared), Ok(checked)) = (prepared_vlc, checked_vlc) {
        assert_same_decoded_reads("VLC", prepared, checked);
    }

    assert_eq!(
        fast_magsgn_reader_workload_for_bench(magsgn, &widths),
        checked_magsgn_reader_workload_for_bench(magsgn, &widths),
        "on-demand MagSgn reader diverged from checked reader"
    );

    let bounded_widths = bounded_magsgn_widths(magsgn, &widths);
    let prepared_magsgn =
        predestuffed_magsgn_reader_workload_for_bench(&mut predestuff, magsgn, &bounded_widths);
    let checked_magsgn = checked_magsgn_reader_workload_for_bench(magsgn, &bounded_widths);
    if let (Ok(prepared), Ok(checked)) = (prepared_magsgn, checked_magsgn) {
        assert_same_decoded_reads("MagSgn", prepared, checked);
    }

    let event_count = plan.first().copied().unwrap_or_default() as usize % (MAX_READS + 1);
    assert_eq!(
        fast_mel_reader_workload_for_bench(vlc, event_count),
        checked_mel_reader_workload_for_bench(vlc, event_count),
        "fast MEL reader diverged from checked reader"
    );
});

fn assert_same_decoded_reads(
    stream: &str,
    prepared: HtCleanupReaderBenchResult,
    checked: HtCleanupReaderBenchResult,
) {
    assert_eq!(
        prepared.checksum, checked.checksum,
        "pre-destuffed {stream} reader decoded different values"
    );
    assert_eq!(
        prepared.read_count, checked.read_count,
        "pre-destuffed {stream} reader completed a different number of reads"
    );
}

fn bounded_magsgn_widths(bytes: &[u8], widths: &[u8]) -> Vec<u8> {
    let mut available = 0_usize;
    let mut previous_was_ff = false;
    for &byte in bytes {
        available += if previous_was_ff { 7 } else { 8 };
        previous_was_ff = byte == 0xff;
    }

    let mut consumed = 0_usize;
    widths
        .iter()
        .copied()
        .take_while(|width| {
            consumed = consumed.saturating_add(usize::from(*width));
            consumed <= available
        })
        .collect()
}
