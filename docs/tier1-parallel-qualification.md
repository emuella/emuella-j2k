# Bounded Tier-1 encoding qualification

The fixed-slot D2 encoder retains the serial writer for one admitted worker
and uses joined batches with deterministic raster-order assembly otherwise.
The resource contract and calibration question are in
[scalable lossless encoding](scalable-lossless.md). This record owns the codec's
development selection evidence; the larger acquisition cohort and delivery
reconciliation belong to the system integration campaign.

## Exact development identities

- Baseline codec: `1998b59d42cfa5c61bd893991a949de6a606a648`.
- Measured candidate: `abf03555cd700b42f1edecb06aa0bce11067cc00`.
- Benchmark: `f78c9c4edc2c606a0037b1445753830781726646`.
- Both worker builds: tuned `perf`, default features and `parallel`, SIMD off,
  Rust 1.97.1; compiler observations, binary hashes and exact source trees are
  retained in the build sidecars.
- Prepared input manifest SHA-256:
  `6c37b54bf75af0c17c1b67c5ad7bd2d56b5d34d45de9737ddb50d0fc1fe10e7b`.
- Development acquisition: Mansfield `94_104001000B823500`, with all PAN16,
  MS16, derived RGB16 and RGB8 products kept together. PAN/MSI use no MCT;
  RGB encode uses the existing reversible MCT. The geometry, D2, single tile,
  layer, LRCP, 64×64 blocks and style-zero boundary is unchanged.

The testdata-approved store retains `tier1-execution-b-mansfield-v1/result.json`
(SHA-256 `9b0888fb49a0df0edc21747c4ebe573a30e93701bbf30632da9ccd52d5e38e19`),
its exact bounded driver and build identities, requests, responses, raw batch
observations and per-case comparisons. Input and compressed pixels stay in that
store. The input hashes are rechecked against the prepared record before use.

## Ordinary encode timing

The baseline and candidate run twenty paired AB/BA rounds at the **same**
requested thread budget, separately at one and eight workers. Each batch uses
a fresh process and warm input; the measured interval is codec encoding, with
exact sample verification afterwards. The existing gate uses conservative 99%
per-case ratio bounds, a 5% practical threshold and no outlier removal.

| Product | Baseline 1, s | Candidate 1, s | Candidate 2, s | Candidate 4, s | Baseline 8, s | Candidate 8, s |
|---|---:|---:|---:|---:|---:|---:|
| PAN16 | 2.0639 | 2.0647 | 1.1076 | 0.6620 | 2.0695 | 0.3698 |
| MS16 | 1.0969 | 1.0924 | 0.6152 | 0.3531 | 1.1050 | 0.2090 |
| RGB16 | 0.3897 | 0.3885 | 0.2181 | 0.1335 | 0.3901 | 0.0743 |
| RGB8 | 2.6157 | 2.6073 | 1.4833 | 0.9623 | 2.6167 | 0.5995 |

Two/four-worker cells are separately named candidate-only five-round means.
They describe scaling; no cross-thread formal comparison is claimed. Baseline
encoding remains serial even when its enclosing pool has eight workers.

| Product | Candidate relative change at 1, 99% interval | At 8, 99% interval | Verdicts 1 / 8 |
|---|---:|---:|---|
| PAN16 | −0.62% to +0.70% | −82.85% to −81.40% | equivalent / improved |
| MS16 | −1.52% to +0.71% | −82.28% to −79.84% | equivalent / improved |
| RGB16 | −1.88% to +1.27% | −81.92% to −79.97% | equivalent / improved |
| RGB8 | −0.85% to +0.22% | −77.86% to −76.32% | equivalent / improved |

All four cases have the same successful coverage. The equal-case geometric mean
speedup is 1.00253× at one worker and 5.10888× at eight. Aggregate component
throughput at eight is 10.124 to 49.961 million samples/s, a different weighting
from the equal-case mean. These are separate operating points; no speedups are
multiplied across packages or build configurations.

## Diagnostic resource and participation observations

The separate `lossless_parallel` example uses the existing forwarding allocator
and `/proc` per-task CPU deltas at 100 ticks/s on this host. All four requested
pool sizes observed exactly 1/2/4/8 distinct Tier-1 workers in its separate
profiled encoder invocation. Diagnostic timings include allocator overhead and
do not supply the headline comparison above. CPU snapshots include their small
collection overhead and only resolve activity at the scheduler tick scale.

| Product | Encode CPU 1 / 8, s | Encoder peak requested allocation 1 / 8, bytes | Ordinary decode wall across 1/2/4/8, s |
|---|---:|---:|---:|
| PAN16 | 2.08 / 2.14 | 88,237,176 / 88,539,736 | 1.327–1.344 |
| MS16 | 1.08 / 1.17 | 42,856,080 / 43,176,592 | 0.696–0.702 |
| RGB16 | 0.38 / 0.38 | 14,268,424 / 14,593,032 | 0.249–0.253 |
| RGB8 | 2.61 / 2.79 | 197,316,296 / 197,592,232 | 2.079–2.125 |

Every observed allocation peak fits its checked working allowance. Requested
allocations exclude input storage and pool startup and include conservative
output reallocation overlap; they are not process RSS. The default output
allowance makes the admission ceiling substantially larger than these observed
peaks. A tighter budget reduces simultaneous slots, preserving serial admission.
Distinct participating threads over the whole operation can exceed the slot
count when slots migrate between pool threads.

Ordinary decode is measured inside `pool.install`, with no prepared or stage
profile route substituted. Its full-image default-precinct Tier-1 parallel gate
requires execution outside a Rayon worker, so the enclosing local pool prevents
that branch. Some transform/component work uses other pool threads, but these
observations show no useful wall scaling. Decoder scheduling remains unchanged.
The diagnostic aggregate is
`tier1-execution-b-mansfield-diagnostic-v1/provenance-and-summary.json`, SHA-256
`ae8282e1a8fb82a5b550e48f410946350bf421082983d0694d465aba4f12424c`.

## Correctness and remaining boundary

The authored matrix covers five sample/component models, both layouts, odd
513×515 geometry, alternating sparse/dense stripes, 1/2/4/8 pools, exact ordinary
and profiled stream parity, native sample equality, one/two-slot memory fallback,
minimum-budget rejection and runtime output-capacity failure. A separate batch
fault test makes the first Tier-1 job fail while three successful jobs still
finish, then checks safe slot reuse. Private owned output is published only after
complete success; `encode_into` retains its existing staged append contract.

All Mansfield streams have identical hash and length at 1/2/4/8. Independent
OpenJPEG decoding of the one-worker export is exact for every product; the
retained mapping binds each other count's identical bytes to that same result,
without manufacturing duplicate payload copies. This mapping is
`tier1-execution-b-mansfield-v1/parallel-stream-identity.json`, SHA-256
`c2126b72b33b88aebf4311ea33b4f60cf786321c9d74cfe3d2565ce7d3106fa6`.

Retain this bounded design for the next qualification gate. These development
results do not establish the fixed twelve-acquisition result or broader JPEG
2000 support. Reserved acquisitions, Tok regression and final current/merged
owner qualification remain system campaign work. No geometry, decoder,
DWT/SIMD, selective-bypass or strict decoder policy change is included here.
