# Indexed HT representation efficiency exploration

This finite investigation preserves the existing encoder defaults, per-tile
byte allowance, descriptor syntax, decoder, component order and native precision.
No external codec implementation source is an input. Temporary experimental
instrumentation and policies are removed unless separately qualified as an
explicit additive policy. Historical viewer quality and runtime failures remain.

The baseline measures every tile search index, candidate byte length, final
index and visit count. Each candidate records component/subband entropy-body
bytes, quantisation step, sample count, transformed-domain squared reconstruction
error and zero population. These are numerical aggregates, never coefficient or
packet payloads. Transformed-domain error is diagnostic, not spatial or display
quality and not a surrogate for the frozen acceptance score.

## Frozen mechanism and budget

The authored encoder truncates absolute coefficients to a scalar magnitude index.
Its decoder reconstructs zero at zero and a nonzero index q at (q + 0.5) times
the step. The first decision boundary is currently one step, although the
midpoint between the zero and first nonzero reconstruction values is 0.75 steps.
The related mechanism family changes only the first zero-bin boundary to 0.875,
0.75 or 0.5 steps. All later bins and decoder reconstruction remain unchanged.
The last threshold deliberately explores beyond the nearest-midpoint boundary;
it may increase distortion. Rate search remains independently bounded per tile,
so extra nonzero coefficients may require a coarser selected step.

Before any probe, baseline diagnostics estimate each threshold's fixed-step
error and changed population. No weighting, component budget transfer, cross-tile
borrowing, extra rates or display-coordinate optimisation is part of this family.
One baseline and at most these three probes run once each on Mansfield RGB16 at
12 total bpp, Mansfield PAN16 at 4 and Tok RGB8 at 4: at most twelve serial screen
encodes, no measured retries, thirty minutes per invocation and sixteen GiB of
new discovery output. Existing admission and workspace limits remain unchanged.
The original TIFFs, selected bands, tile support, views, stretches and scale-one
and scale-four quality functions are bound by the consumer's frozen protocol.

No untouched validation is claimed for Mansfield, Boca or Tok. A promoted policy
requires separately bound confirmation cases, full-scene ANY-valid quality,
independent decoder reconstruction, regional/native/WASM and merged-owner proof.
Missing mandatory evidence blocks promotion. At most one policy can be frozen;
reserved results cannot drive retuning. Rejection retains all outcomes and removes
unsuccessful production experiments. Final findings and evidence follow below.

Selection was bound before any probe encode or result: at least one RGB product
must newly pass every existing frozen screen quality and actual-byte gate, and
PAN16 must retain its pass and resource contract. A PAN16-only gain is insufficient.
Rank eligible policies by number of newly passing RGB products, then lowest
worst RGB16 frozen-cell RMSE, then lower actual image payload, then the threshold
closest to baseline. Fixed-step transformed error cannot select a policy.

## Completed bounded outcome

Reject this first-bin mechanism family for the frozen screen acceptance. Twelve
serial invocations completed with no setup failures, process failures or measured
retries. No RGB screen passed, so the predeclared selection rule selected no
policy. All experimental production changes, including diagnostics, are restored
to the exact starting source. This deliverable changes documentation only.

The [aggregate record](representation-efficiency-results.json) binds every
experimental source revision/tree, immutable consumer/tool build, source hash,
selected source bands, crop lineage, search utilisation, final quantisers,
component bytes/distortion, source-precision error, display score and resource
diagnostic. Detailed per-candidate component/subband data remain in the approved
RarePlanes store's attributed `representation-efficiency-a-*-execution` groups.
Source patches, build/run/analysis scripts and all twelve successful or rejected
numerical outcomes are retained there. No protected payload, sample, display image
or third-party implementation material enters this source repository.

| First-bin threshold | Product / total bpp | Image payload bytes | Failed cells | Worst display RMSE | Worst p99 |
|---|---|---:|---:|---:|---:|
| 1.0, unchanged | RGB16 / 12 | 1,056,940 | 6 / 48 | 3.531609 | 9 |
| 0.875 | RGB16 / 12 | 1,057,057 | 5 / 48 | 3.456510 | 9 |
| 0.75 | RGB16 / 12 | 1,056,965 | 6 / 48 | 3.463809 | 9 |
| 0.5 | RGB16 / 12 | 1,056,997 | 11 / 48 | 4.066907 | 11 |
| 1.0, unchanged | PAN16 / 4 | 1,966,903 | 0 / 16 | 2.747602 | 7 |
| 0.875 | PAN16 / 4 | 1,966,876 | 0 / 16 | 2.765909 | 7 |
| 0.75 | PAN16 / 4 | 1,966,888 | 0 / 16 | 2.695284 | 7 |
| 0.5 | PAN16 / 4 | 1,966,902 | 2 / 16 | 3.071905 | 8 |
| 1.0, unchanged | RGB8 / 4 | 1,573,332 | 24 / 48 | 9.636513 | 25 |
| 0.875 | RGB8 / 4 | 1,573,377 | 24 / 48 | 9.633056 | 25 |
| 0.75 | RGB8 / 4 | 1,573,243 | 24 / 48 | 9.856020 | 25 |
| 0.5 | RGB8 / 4 | 1,573,317 | 29 / 48 | 14.576867 | 37 |

Every candidate satisfies the existing absolute image ceiling: 1,057,400 bytes
for the RGB16 screen, 1,968,114 for PAN16 and 1,574,520 for RGB8. These ceilings
use each actual tile's floored target plus 128 bytes, and exact shared framing.
Each descriptor and manifest also fits its existing 1 MiB bound. The JSON records
payload, descriptor and manifest sizes separately. These quality-only screens
have no mask sidecars and establish no mask, delivery or total viewer eligibility.
Timing and observed peak RSS are retained diagnostics on a shared host; neither
is a new latency, scheduling or RSS acceptance claim.

## What the diagnostics establish

The instrumented baseline produces the exact historical image payload hashes,
raw reconstruction hashes and numerical quality values for all three products.
Reference timing and process counters are excluded from that equality check.
The baseline's 31 local tile searches visit 17 candidates each except one RGB8
tile with 16 visits. Summed unused bytes inside the actual search envelopes are
152 for RGB16, 146 for PAN16 and 264 for RGB8. These local envelopes differ from
the conservative complete-image ceiling because the writer shares framing.
The observations show little unspent local budget; they do not establish global
rate-distortion optimality or make tile-budget transfers part of this contract.

At the baseline's selected steps, changing only the first boundary to 0.75 would
reduce aggregate transformed-domain squared error from 80.843 to 67.700 million
for RGB16, 140.651 to 118.023 million for PAN16, and 6.787 to 5.655 million for
RGB8. Those estimates justified testing the mechanism; they held the steps fixed
and did not charge the additional nonzero coefficients against compressed bytes.
Actual bounded rate search chose coarser steps. The 0.75 experiment's resulting
transformed error was 77.828, 134.188 and 7.649 million respectively. None of its
RGB products passed the unchanged spatial display gate. The 0.5 overshoot control
increased distortion and broke the previously passing PAN16 control.

All first-bin thresholds and the selection rule were frozen before any probe
result. No additional rate, weighted subband, component allocation, cross-tile
borrowing, source stretch or scored coordinate was tuned. Component/subband body
bytes and transformed errors are observations; this investigation does not
attribute all remaining error to one component or frequency band.

## Qualification limits and verification

This is a bounded screen rejection of the declared family, not rejection of every
possible encoder policy. The small RGB16 improvement and PAN16 control gain do
not establish product acceptance. No policy was selected, so no changed-stream
full-scene, independent-decoder, regional/native/WASM or merged-policy confirmation
was triggered. Those remain mandatory before promoting a future eligible policy.
Mansfield, Boca and Tok are development/regression cases; no untouched validation
was claimed, and no new acquisition, crop, rate or stretch was introduced for
confirmation. Historical full-scene quality, pressure, latency and scheduling
failures are unchanged. The production decoder, descriptors and classic defaults
are byte-for-byte unchanged from the starting source.

Four clean archived consumer builds used recorded local codec dependency and
protocol-identity substitutions; the scoring and population functions were
unchanged. Each build record binds the resulting lockfile, binary and GDAL hashes.
The existing preparation runner rechecked original source identities and verified
selected crop samples against original tile support before each encode, then
rechecked original hashes after scoring. Build inputs, invocations, numerical
outputs and authored experimental patches are referenced by SHA-256 in the
aggregate record. Generated build resources have their lifecycle in the system
campaign; protected evidence remains in its approved store.

The final documentation candidate requires the repository's canonical clean-tree
check and independent exact-head review before delivery. These delivery checks
do not rerun or expand the exhausted twelve-invocation discovery budget.
