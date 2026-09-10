# Dense first-refinement calibration

This bounded experiment asks whether the existing neighbour-presence bitboard
can reduce work in high-bit-depth classic Tier-1 decoding. The semantic owner is
the codec. Full-image measurements use the benchmark harness and testdata-owned
RarePlanes preparation; no imagery or third-party codec material is included here.

## Frozen probe

- Baseline: `1998b59d42cfa5c61bd893991a949de6a606a648`.
- Experimental source: `f396dd456d2a09325dfb8d4ee422021c4f303658`.
- Benchmark: `f78c9c4edc2c606a0037b1445753830781726646`.
- Build: tuned `perf`, optional SIMD disabled, one worker, Rust 1.97.1,
  AMD Ryzen 9 9950X3D, unchanged lossless D2 and 64×64 code-block profile.
- Development acquisition: Mansfield `94_104001000B823500`, retaining PAN16,
  MS16, RGB8 and RGB16 together. Reserved acquisitions do not inform selection.
- Prepared manifest SHA-256:
  `6c37b54bf75af0c17c1b67c5ad7bd2d56b5d34d45de9737ddb50d0fc1fe10e7b`.

The exit condition is a full-image decision under the existing conservative
99% per-case interval and 5% practical threshold, with exact bytes, samples,
profiles and failure behaviour preserved. Authored replay can reject a mechanism
cheaply; it cannot establish satellite performance.

## Production mechanism

Hardware counters and 499 Hz DWARF stack samples were collected from the actual
packed-dense worker. Whole-process PAN16 decode used 7.875 billion user cycles,
16.350 billion instructions, 148.520 million branch misses and 1.049 million
cache misses. MS16 used 4.118 billion cycles, 8.956 billion instructions,
78.318 million branch misses and 0.540 million cache misses. These include worker
setup and correctness checks and are mechanism observations, not headline codec
operation timings.

| Sampled symbol | PAN16 | MS16 |
|---|---:|---:|
| Magnitude refinement | 40.94% | 35.23% |
| Significance propagation | 37.28% | 41.65% |
| Cleanup | 14.83% | 12.56% |

The compiled full-word loop already inlines MQ decoding and keeps the main
interval and code registers resident, with state stores after decisions. The
repeat-only specialisation is present. Mixed first/repeat words still calculate
row-major coordinates and read eight neighbour flags to answer the first
refinement's presence query. They also retain probability-table traffic and
arithmetic branches. This evidence does not establish memory bandwidth as the
bottleneck or justify a new arithmetic representation.

For a fresh dense 64×64 scratch, coefficient storage is 16,384 bytes, the four
bitboards occupy 2,048 bytes, and each haloed neighbourhood/sign array occupies
34,848 bytes: 88,128 bytes total. The probe changes neither this footprint nor
resource admission. It replaces only the first-refinement presence query with
the existing stripe neighbour word and coefficient mask; other context labels,
scan order, arithmetic and output reconstruction remain the same.

The focused tests passed all 14 Tier-1 unit tests and strict Clippy. An added
exhaustive probe compared neighbour presence for every isolated source and target
position at 16×8, 17×11 and 64×64, both with and without vertical-causal mode.
The unchanged checked decoder remains the coefficient-context oracle.

Four alternating authored replay pairs each used 64 repetitions of the 160
cases. All three decode backends reconstructed every case exactly. Baseline and
candidate encoded bytes were identical: 304,926 bytes, SHA-256
`adda2a1cc7eb3c353e016fdcee6b0ca10920224e038de60e86ee4a0159eb3b44`.
Dense decode's descriptive geometric speed ratios were 1.009 for dense patterns,
1.012 for sparse patterns, 1.043 for patches and 1.033 for diagonals. They do not
satisfy the full-image selection gate by themselves.

## Full-image decision

The probe was rejected. The fixed 20-round comparison finds PAN16 and MS16 decode
improvements within the existing ±5% equivalence band. Those results do not meet
the practical selection gate, even though their intervals exclude zero. The baseline implementation was restored; the experimental source and invariant test are not
part of the resulting production diff. No reserved acquisition was used to tune
or select the probe, and a rejected candidate needs no holdout qualification.

The completed comparison reports no failures. All four exports have identical
baseline/candidate bytes and passed complete sample checks through both Emuella
workers and the independent OpenJPEG worker. Twenty fresh-process rounds per
case retain the original 99% interval, 5% threshold and no-outlier-removal policy.
All ten cases are equivalent; negative relative time favours the candidate.

| Family | Product | Candidate relative-time 99% interval |
|---|---|---:|
| Encode | PAN16 | −0.405% to +0.959% |
| Encode | MS16 | −1.113% to +1.252% |
| Encode | RGB8 | −0.106% to +1.192% |
| Encode | RGB16 | −1.933% to +1.981% |
| Common no-MCT decode | PAN16 | −3.309% to −1.920% |
| Common no-MCT decode | MS16 | −3.731% to −1.570% |
| Common no-MCT decode | RGB8 | −1.120% to +0.467% |
| Common no-MCT decode | RGB16 | −4.024% to −0.491% |
| RGB-RCT decode | RGB8 | −0.761% to +0.941% |
| RGB-RCT decode | RGB16 | −4.131% to −0.926% |

| Family | Same-success cases | Equal-case geometric speed ratio | Sample-weighted throughput ratio |
|---|---:|---:|---:|
| Encode | 4/4 | 0.997795 | 0.996683 |
| Common no-MCT decode | 4/4 | 1.020139 | 1.015341 |
| RGB-RCT decode | 2/2 | 1.012518 | 1.002445 |

Ratios above one favour the candidate. These descriptive aggregates have no
confidence interval and do not override the per-case selection decision. They
are not compounded with historical gains.

The result is limited to this rejected presence-query probe and this development
acquisition. It does not reject compact state, a different MQ design or PGO as
future hypotheses. No supported profile, error, resource bound or production
implementation changes as a result; subsequent work starts from the unchanged
codec behaviour.

Store-local evidence is retained under `tier1-execution-a-profile` and
`tier1-execution-a-mansfield-v1` within the approved RarePlanes store. It includes
exact input hashes, build sidecars and compiler commands, the runner, raw worker
requests/responses, annotated assembly and a recoverable source patch. The patch
SHA-256 is `564076a45b931d861560189dcfcd0406c228beaba2046785a23bf3e5415f3042`.

The completed report SHA-256 is
`bf8209f24e16b94fac6a0169fb2d6a2b7cf4cfdbf8bd306a503f53ee3d926349`.
The retained runner SHA-256 is
`57b50bdf6dc2ac7ce28fbd8e0087ed847a87489282f29297ccd2b5e8efd99685`;
it reuses the benchmark's existing qualification helpers with the current
harness location. Baseline/candidate build-provenance SHA-256 values are
`79fcd8fba9ca44e1bfbd33207e82c7e2ba72c84a0d2c2429c9e3bc510aeb76d2` and
`dcc4eab5c30b91fff53e5e206fe7465853aa3f647886000fc26358fef37cde0a`.
