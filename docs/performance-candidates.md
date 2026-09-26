# Retained performance candidates

This codec-owned inventory retains project-authored source experiments
for possible future investigation. It changes no production execution or
historical verdict. Recoverable source, correctness observations, timing evidence
and engineering disposition are separate. Retention is not qualification, an
implementation request or a commitment to maintain a branch, rebase, build or
periodically retest a candidate.

The patches are complete text-only changes from reachable merged bases, including
the original authored tests, configuration and documentation where changed.
They contain no binaries, protected inputs, generated stream payloads, raw
measurement transcripts or copied standards expression. Historical documentation
inside a patch describes that experiment, not the current production contract.
The [Tier-1 provenance and oracle record](tier1-implementation.md) remains the
implementation authority; timing and experiment design belong to the benchmark
component's pinned [entropy protocol][entropy-protocol], [entropy result][entropy],
[entropy evidence][entropy-json], [parallel protocol][parallel-protocol],
[parallel result][parallel] and [parallel evidence][parallel-json].

## Owner inventory

| Candidate / owner | Mechanism and callers | Historical evidence and disposition | Complexity, compatibility and reuse boundary |
|---|---|---|---|
| MQ D1 / Tier-1 | `mq::Decoder::read_bit` keeps interval, code and context local and delays successor access while retaining exchange branches. Shared by checked, dense-packed and sparse-packed classic block decode. | Correctness checks passed within the recorded scope. Three-pair development means show small, unconfirmed effects; the high-bit-depth style-zero screen failed. No confirmation or allocation-only qualification. | Local arithmetic refactor with no new buffer, API, scheduler or encoder change. An alternative to D2, not an additive stack member. Fresh review must preserve fast MPS return, context/register state, byte IO, raw transitions and termination. |
| MQ D2 / Tier-1 | Same decision-local family, with Boolean-selected exchange and successor writeback in the same decoder and callers. | Correctness checks passed within the recorded scope. Three-pair development means are unconfirmed; the same high-bit-depth screen failed. A descriptive eight-worker follow-up did not promote it. | More factored control flow than D1; no new buffer or public API. Alternative to D1. Neither is currently eligible for adoption on the historical evidence alone. |
| 4W scheduler / codestream scalable lossless writer | W reusable scratch objects claim at most 4W finite result slots through an atomic index; per-slot mutexes protect storage, then results join and append in stream order. Affects parallel D2 style-zero/bypass encode. | Completed frozen confirmation failed its original promotion gate. Correctness and resource observations passed their recorded checks, but the required high-bit-depth improvements did not. A decoder non-regression bound also remained unresolved. | Separate, higher-complexity scheduler experiment, excluded from the low-cost incremental lane. Extra result lifetime, joining/fault/panic behaviour, admission and up to `3W × 4 MiB` extra working charge require dedicated justification. Not bundled with MQ candidates by this record. |

All three production experiments were removed. Original arithmetic and W joined
batches remain in production. Future reuse needs fresh source review against an
explicit contemporary baseline and a new frozen protocol with appropriate
correctness, resource and regression gates. Historical baseline applicability
must be established rather than assumed. D1/D2 effects cannot be summed, and
results across the entropy and scheduler experiments cannot establish a combined
effect. Retire a record explicitly when its mechanism is superseded, no longer
applicable, cannot be recovered, or its maintenance cost has no justified benefit;
retain the historical result and reason. There is no recurring retest obligation.

## Exact source and recovery

Each patch applies independently to its own base. The source commits identify the
measured experiments; durable recovery uses the tracked patch and merged base,
not the continued reachability of an abandoned source commit.

| Candidate | Base commit / tree | Historical source commit / recovered tree | Source patch / SHA-256 |
|---|---|---|---|
| D1 | `bc747f86907aff09e09278e4444ad5932ef4669d` / `138634fec5d3b7818b95c85d34881dc0807214a7` | `1e0e920c1096671b04863f81a254720df1d66866` / `da49733b249e75be730b38c30bf27303a15d74f1` | [mq-d1.patch.txt](performance-candidates/mq-d1.patch.txt) / `d47f58453ee11a77e736586da9c6f0abcd60980881a4dacb1a9d4418e3c1c0ae` |
| D2 | `bc747f86907aff09e09278e4444ad5932ef4669d` / `138634fec5d3b7818b95c85d34881dc0807214a7` | `78534e6ebad9ad5bbec35529ff315d3900f69b8a` / `ada31becf548c53773ae66f893000526dac9426f` | [mq-d2.patch.txt](performance-candidates/mq-d2.patch.txt) / `0db1e4038effac83457d533cfae827ef10252518511995f1e164d0a834cc9ae5` |
| 4W | `94ba19589b0710192c295478c1f9ad284ea2abd5` / `d9f73ef09ea3bf3cbc50b9a3a250156bee537bec` | `8aca0cde9031c9e81f5cec33b3f09a8d2a330128` / `b9bb3aec96ed86ca5ee2b6db461ad9daee8e1ad5` | [scheduler-4w.patch.txt](performance-candidates/scheduler-4w.patch.txt) / `8dc23bc68ac96ac09afacded8930efc33fa5d6de139f61f3806b713fbc277909` |

Recovery was checked without building or executing a candidate: load the base
into an isolated Git index, apply the patch with `git apply --cached`, and require
`git write-tree` to equal the recorded historical source tree. This compares all
tracked bytes, paths and modes, including unchanged base files. All three matched.
Both bases are ancestors of inventory starting revision
`5eed30d5a2da1cc7b291e19cdfbe031c6ea97584`. The patches include five changed files
each for D1/D2 and fifteen for 4W; full-index blob identities bind every hunk.
They are archive artefacts under `docs/`, not compiled modules or build inputs.

For example, from a full codec clone the following checks D1 in a temporary index
and prints the recovered tree without modifying the checkout or executing it.
Use the matching base, patch and expected tree above for each other candidate:

```sh
(
  candidate_index_dir=$(mktemp -d)
  trap 'rm -f "$candidate_index_dir/index"; rmdir "$candidate_index_dir"' EXIT
  export GIT_INDEX_FILE="$candidate_index_dir/index"
  git read-tree bc747f86907aff09e09278e4444ad5932ef4669d
  git apply --cached --check docs/performance-candidates/mq-d1.patch.txt
  git apply --cached docs/performance-candidates/mq-d1.patch.txt
  git write-tree
)
```

## MQ evidence and missing gates

The benchmark's `descriptive_results.d1_screen`, `d2_screen` and
`d2_eight_worker` own the following factual means. Each uses three adjacent
alternating fresh-process pairs, no warmups, one decode operation per process,
complete Mansfield inputs and common OpenJPEG-origin streams. Both arms use
Rust 1.97.1, perf/optimisation 3, ThinLTO, one codegen unit, parallel enabled,
SIMD disabled, packed-default encode and the original W scheduler on the same
AMD Ryzen 9 9950X3D host. One worker uses CPU 0; eight use CPUs 0–7. Decode is
direct/global, lossless D2, interleaved, with RGB RCT and no PAN/MSI MCT. Limits
are 768 MiB working, 64 MiB output, 120 seconds and an 8 GiB process ceiling.
D1 and D2 have separate adjacent controls; D1 must not borrow D2's baseline.

Times below are baseline → candidate milliseconds; saving is baseline minus
candidate, computed before rounding. A positive saving favours the candidate.
There are **no confidence intervals or confirmed improvement claims** for these
means; their precision does not measure uncertainty.

| Product / style | D1 one worker: ms (saving) | D2 one worker: ms (saving) | D2 eight workers: ms (saving) |
|---|---|---|---|
| PAN16 / 0 | 1338.066 → 1306.615 (31.452) | 1356.276 → 1306.450 (49.826) | 205.567 → 199.691 (5.876) |
| PAN16 / bypass | 684.026 → 656.489 (27.537) | 681.078 → 675.202 (5.876) | 122.342 → 117.507 (4.835) |
| MS16 / 0 | 713.528 → 697.446 (16.081) | 718.359 → 698.857 (19.502) | 104.121 → 104.373 (−0.252) |
| MS16 / bypass | 331.074 → 321.367 (9.708) | 330.229 → 322.198 (8.032) | 55.494 → 53.761 (1.733) |
| RGB8 / 0 | 1749.502 → 1654.140 (95.362) | 1745.706 → 1672.134 (73.572) | 277.802 → 263.614 (14.188) |
| RGB8 / bypass | 1563.907 → 1469.184 (94.723) | 1548.116 → 1493.749 (54.368) | 248.578 → 236.016 (12.562) |

Neither variant achieved a PAN16/MS16 style-zero development mean ratio below
0.95; all six one-worker ratios in each variant were at most 1.05. The RGB8
means cannot satisfy that high-bit-depth predicate. Primary confirmation,
internal regression, candidate allocation-only measurements, expanded both-origin
1/2/4/8-worker corpus parity, SpaceNet performance and the external anchor were
not started. D1 has no eight-worker timing result. Whole-process RSS/CPU is not
allocation-bound qualification. Timed authored replay pairs numbered zero.

Across the experiment, 108 ordinary and eighteen sampled facade calls produced
exact native samples and expected stream hashes. The retained independent
entropy oracle covered 31 Tier-1 tests, exhaustive two-byte inputs, all 47
reachable probability states, decisions/contexts/registers/byte state, consumed
prefixes, predictable termination and raw stuffing. Authored block replay
checked independent writer bytes and segment lengths alongside pass/missing-plane
metadata and reconstruction across styles, magnitudes, partial/wide shapes and
malformed/truncated inputs. Both variants passed explicit no-std and WASM checks.
These historical checks do not discharge any new baseline's correctness gates.

## Contemporary D2 incremental confirmation

A separate fresh confirmation on the contemporary baseline applied
`incremental-performance-policy/v1`, Route A. D2 is **not selected**: its primary
PAN16 result missed the 2% estimated reduction, strictly negative 99% upper
change bound and 20 ms absolute saving requirements. This disposition is not
insufficient precision alone because the estimated and absolute savings also
missed their gates. The historical development verdicts above remain unchanged.
Production MQ arithmetic was restored; the independent arithmetic oracle and a
new decoder scratch recovery regression remain.

The measured adaptation changed only the decision path in `Decoder::read_bit`,
removed its now-unused exchange helpers and added that regression. It restored
no historical harness, build configuration, scheduler or runtime selector.
The test exercises checked, dense, sparse and adaptive decoder scratch after an
entropy-stage segmentation-symbol failure, with and without selective bypass,
and verifies recovery across geometry changes without retained-capacity growth.

| Contemporary recovery identity | Value |
|---|---|
| Base commit / tree | `e0971bc82d18d787a47c53ff27ed51a81e70b94c` / `b4745457f2065fd17ab67b66d385d17f1e651110` |
| Measured commit / recovered tree | `07c85ecac1fbe1c477c05a8f6c8d605d1039d9ad` / `98206d5e870b1601815508796b83aac0ffbe24df` |
| Complete source patch | [mq-d2-incremental.patch.txt](performance-candidates/mq-d2-incremental.patch.txt) |
| Patch SHA-256 | `ebce0dd612f5d6f86bbd82d9722cc70b2b1d0da65e7f6651f3cf3ac534dc590a` |

The full-index patch includes the complete two-file measured diff, including the
scratch regression. Isolated-index application to its stated base reproduced
the recorded measured tree exactly. Use the recovery procedure above with these
identities; do not apply it on top of the final retained-test tree. The patch is
source-only archival material, with no compiled or dormant production route.

Twenty alternating fresh-process pairs per contrast used the common
OpenJPEG-origin stream, full Boca Raton inputs, style zero and one-worker decode.
The primary and mandatory MSI16 corroboration produced:

| Contrast | Baseline → D2 mean ms | Saving ms | Relative change | 99% interval |
|---|---:|---:|---:|---:|
| PAN16 primary | 1714.338752 → 1708.867047 | 5.471704 | −0.319173% | [−0.949967%, +0.314787%] |
| MSI16 corroboration | 937.691784 → 936.122374 | 1.569410 | −0.167370% | [−1.046925%, +0.719901%] |

MSI16 passed its upper-bound tolerance of +1%. Both contrasts were equivalent
under the legacy ±5% classification. All 80 timed calls passed exactness and
absolute limits. The primary failed, so the conditional 560 timing and 80
resource calls were not started; no extra sampling was performed.

Before timing, 80 correctness/resource calls checked exact bytes and native
samples across both stream origins and 1/2/4/8 workers. Maximum observed
additional requested allocation was 233,441,260 bytes. Allocation evidence is
separate from process RSS. The initial literal allocation-request-count gate
failed at four and eight workers. An independently reviewed pre-timing setup
correction preserved that failure and reported schedule-dependent request
counts alongside source review showing no new allocation or buffer mechanism;
all allocation, retained-capacity and admission bounds remained unchanged.
These observations do not claim that request counts were identical.

The measured candidate passed the contemporary independent oracle, no-std and
WASM checks, focused native/failure/resource tests and the complete canonical
check. The benchmark owns the frozen protocol, setup correction, statistical
method and detailed [confirmation result][incremental-result] and
[machine-readable evidence][incremental-evidence]. Recovery does not confer
qualification: any future reconsideration requires a newly justified protocol
and fresh eligibility review, with no recurring retest obligation.

## Separate 4W evidence and costs

The frozen twenty-pair confirmation retained the original conservative 99%
per-contrast intervals and 5% practical gate. All 1,920 RarePlanes calls were
exact, all eighteen one-worker encode contrasts were equivalent, and every encode
upper change bound was below +5%. Required Boca Raton high-bit-depth eight-worker
encode contrasts nevertheless remained inconclusive against the improvement gate:

| Product / style | Baseline → 4W ms | Saving ms | Relative change, 99% interval |
|---|---|---:|---|
| PAN16 / 0 | 390.447 → 361.921 | 28.526 | [−12.21%, −1.96%] |
| PAN16 / bypass | 282.333 → 267.406 | 14.927 | [−11.25%, +1.20%] |
| MS16 / 0 | 199.253 → 181.393 | 17.860 | [−13.71%, −4.02%] |
| MS16 / bypass | 131.896 → 116.573 | 15.323 | [−18.12%, −4.32%] |

Savings above are differences of the displayed rounded means. Other encode
improvements did not rescue the failed gate. Of twelve targeted decode contrasts,
nine were equivalent and three inconclusive; unchanged MS16/style-zero decode at
eight workers was 105.398 → 107.315 ms with [−2.40%, +6.23%], leaving practical
non-regression unproved for that contrast. The fixed 480-call SpaceNet and
1,440-call matched OpenJPEG encode cohorts completed; they did not change the
historical rejection. This is not the MQ experiment's unstarted confirmation.

The 144 separate allocation-only calls fitted each arm's working queries; maximum
requested peak/query was 0.644825. Peak requested allocation rose from 366,357,664
to 366,461,984 bytes. Parallel queries added exactly `3W × 4 MiB` in the measured
cohort (96 MiB at eight workers); the largest query was 702,852,604 bytes, below
768 MiB. One-worker queries were unchanged. Tight budgets reduced extra slots
without changing original worker admission. These are separate from RSS and
headline timings and do not make extra scheduler state a low-cost change.

Historical authored tests covered stalled-first/out-of-order completion,
faults, bounded residency, joining, release counts, panic/reuse, stream/segment
metadata and admission; the frozen candidate's canonical checks passed. Any
reuse must revisit those invariants and resource costs on its new baseline.
The archive does not reinstate the removed implementation or its test machinery.

[entropy-protocol]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/classic-entropy-hot-loop.md
[entropy]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/classic-entropy-hot-loop-results.md
[entropy-json]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/evidence/classic-entropy-hot-loop.json
[parallel-protocol]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/classic-parallel-execution.md
[parallel]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/classic-parallel-execution-results.md
[parallel-json]: https://github.com/emuella/emuella-benchmark/blob/876aefb9fc53d3e856de4c77ba253c043fdc3805/docs/evidence/classic-parallel-execution.json

[incremental-result]: https://github.com/emuella/emuella-benchmark/blob/main/docs/mq-d2-incremental-confirmation-results.md
[incremental-evidence]: https://github.com/emuella/emuella-benchmark/blob/main/docs/evidence/mq-d2-incremental-confirmation.json


## Classic encode front-end locality experiment

The contemporary `classic-encode-front-end/v1` experiment is **not selected**.
Its Route A primary passed, but required non-regression remained unproved in
22 of 24 conditional contrasts. None of those contrasts established a slowdown;
insufficient precision is not zero effect. No additional timing rounds were
run. The conditional OpenJPEG anchor and selected-stage re-profile were not
started. Production uses the original single-column traversal, original MQ,
packed encoder and W batches; only feature-gated stage diagnostics remain.

The one development variant gathered two adjacent DWT columns into two existing
scratch lines before applying the unchanged bounded lifting and scattering each
column. The third scratch line remained lifting output; odd columns used the
original route. There was no new allocation, concurrency, ISA, runtime selector
or resource charge. Independent pre-timing review classified this compact
locality change as Route A. Candidate tests compared intermediate coefficients
against original traversal and checked arithmetic, and complete streams against
an original-path encoder across layouts and shapes. Those candidate tests are
retained with the source archive, not compiled as dormant production machinery.

| Recovery identity | Value |
|---|---|
| Reachable production base / tree | `b02b6ab1ddaecefd25e18d4cd610eba09e8a6627` / `ccebe92485183eb016c25bb7eeb01d4fd00533ef` |
| Measured source / recovered tree | `21033ea890b4393e8c7f6791cf4a89d866d2dfd3` / `19c26fe7f04f88ac8125fdd736916827e41c18a5` |
| Complete source patch | [classic-front-end-columns.patch.txt](performance-candidates/classic-front-end-columns.patch.txt) |
| Patch SHA-256 | `08fdf2912df2ed5003c6a9365a3d35db2f4d32e5e179a7954a98f27d38407e08` |

Isolated-index application to the stated merged base reproduced the measured
tree exactly. The full-index patch includes the diagnostic prerequisite,
implementation, tests and documentation. Apply it to that base, not to the
retained-diagnostics tree. It contains only project-authored source.

The sole primary, full Boca Raton RGB8/bypass at eight workers, improved from
735.469171 to 701.548489 ms: 33.920682 ms saving, estimated −4.612115%, with
99% interval [−7.931653%, −1.167162%]. This passed the frozen Route A estimated
2%, strictly negative upper bound and 10 ms absolute gates. Its legacy 5%
classification remained inconclusive. Style-zero and both one-worker initial
contrasts passed their +1% upper-bound non-regression gate. The subsequent
high-bit-depth, Tok, SpaceNet and targeted unchanged-decoder matrix left 22
upper bounds above +1%; primary success did not waive that requirement.

All 1,288 experimental calls passed their applicable exactness and absolute
resource gates. The separate 112 allocation-only observations retained the same
measured peaks, counts, queries and output capacities in paired arms; equality
of allocation counts is an observation, not a scheduling contract. Maximum
requested peak was 366,357,664 bytes against its 602,189,308-byte query. Frozen
candidate canonical native, no-std and WASM verification passed. Independent
decoder receipts were reused only for identical stream hashes with provenance.

The benchmark owns the [frozen protocol][front-end-protocol],
[complete results][front-end-result] and [factual evidence][front-end-evidence].
Any future proposal needs its own contemporary baseline, justified fixed
precision budget and acceptance route. This archive creates no implementation
request or recurring retest obligation.

[front-end-protocol]: https://github.com/emuella/emuella-benchmark/blob/main/docs/classic-encode-front-end.md
[front-end-result]: https://github.com/emuella/emuella-benchmark/blob/main/docs/classic-encode-front-end-results.md
[front-end-evidence]: https://github.com/emuella/emuella-benchmark/blob/main/docs/evidence/classic-encode-front-end.json

## Classic forward 5/3 bounded panels

The finite `classic-forward53-finite-confirmation/v1` attempt is **DECLINED BEFORE
LAUNCH**. A fresh authorised balanced exclusive CPU reservation was unavailable:
non-interactive administrative access required authentication, and the previous
single-use reservation had already been restored and removed. There were zero
confirmation worker starts and no measured candidate rejection. Production retains
the original scalar forward traversal, original MQ, packed-default encoding and W.
The implementation attempt is closed; future reuse needs a separately authorised
objective or evidence plan, with no ongoing worktree, rebase or retest obligation.

The engineering owner prospectively authorised one finite attempt despite incomplete
A/A precision. That amendment changes only this panel attempt's feasibility-based
launch prohibition and indefinite pending disposition; it does not relax acceptance,
claim sufficient power, revise history or establish product-wide policy. The full
Boca RGB8/bypass/eight-worker primary still requires its 99% upper change bound
strictly below -5% and mean saving at least 10 ms, with every critical upper bound
at most +1% and all exactness, interoperability, resource and failure gates.

| Recovery identity | Value |
|---|---|
| Reachable baseline / tree | `975a5e734773578f61abf76d5fddfbd837f3bd7d` / `cea1ed5dd73b11bfaf3ee1260629fab6d3e4919e` |
| Frozen candidate / recovered tree | `d60859a8595554be52c8748a8e8c85b69614fea5` / `ea121ffa15497d10b4657897b0f9d54f93c3588b` |
| Complete source patch | [classic-forward53-panels.patch.txt](performance-candidates/classic-forward53-panels.patch.txt) |
| Patch SHA-256 | `8f0092d19ba607f76ec6c27dda2b652c3e39b9d85f91a1ca7a9c549bea88b47a` |

Isolated-index application to the exact baseline reproduces the complete candidate
tree, including all thirteen changed source, test, build and documentation files.
No candidate code is compiled by this archive. The snapshot preserves the width-16
bounded parallel-panel policy, one-worker behaviour and independent scalar fallback;
it is not a newer algorithm. The original proof and tests are in the patch and
[original source document](https://github.com/emuella/emuella-j2k/blob/d60859a8595554be52c8748a8e8c85b69614fea5/docs/forward53-panels.md).

The mechanism adds an immutable geometry plan, explicitly owned reusable panel/line
slots, parallel gather/lift, joined disjoint-row scatter and horizontal work.
Components remain sequential, workspace is prepared before mutation and released
before entropy/output allocation, and existing queries and admitted workers remain
unchanged. Scheduling, allocation/fallback and join/failure complexity require the
larger-change policy; no runtime option, decoder change or other optimisation is
part of the treatment. Any future integration must inspect conflicts and preserve
newer independent tests rather than restore this tree over contemporary mainline.

Historical source-readiness review, canonical/native/no-std/WASM checks, independent
axis/level/stream parity, tight-budget/reuse and joined-failure checks remain scoped
to the recorded candidate. All 168 development calls are retained: 84 descriptive
ordinary, 68 allocation, twelve diagnostics and four unchanged-decoder calls.
All 164 encode calls matched complete bytes/native samples and applicable independent
decode receipts; all 68 allocation calls passed unchanged query/output gates.
These counts overlap by role and are not additional confirmation samples.
The single RGB16 bypass/one-worker +6.585% descriptive observation remains an adverse
observation to retain, not a demonstrated regression or a discarded tail.
No development mean establishes a confidence interval or production qualification.
See the benchmark's [development evidence](https://github.com/emuella/emuella-benchmark/blob/162373e881c517bdfb1f9527dd0d0c589a7c9a3c/docs/classic-forward53-panels-results.md)
and its finite confirmation record for complete observations and unstarted gates.


## Classic forward 5/3 parallel dispatch v1

The separate historical `classic-forward53-parallel-dispatch/v1` treatment is
**NOT QUALIFIED WITHIN BUDGET**. At its closure it remained source-only, with no
authorised retest or production promotion under that protocol. Earlier
finite-confirmation v1/v2 outcomes and their evidence are unchanged. The later
`classic-forward53-integration-assessment/v1` separately selected a
byte-congruent B implementation for the ordinary eligible parallel route;
[its current source and proof](forward53-parallel-dispatch.md) retain the
original one-worker helper and bounded fallback.

| Recovery identity | Value |
|---|---|
| Reachable baseline / tree | `a7576ad03486e097ac923b8e49cac39a1cbef5d2` / `5e7dfa48280c6d1d0abfe376d69af243e879359f` |
| Frozen candidate / recovered tree | `4f5bfb39f02e043c9a1f594a8159d3cf86d52c3f` / `259066112bda675e67a8d3eab3dbf9088d18fe91` |
| Complete source patch | [classic-forward53-parallel-dispatch.patch.txt](performance-candidates/classic-forward53-parallel-dispatch.patch.txt) |
| Patch SHA-256 | `ad5333e4e5ea285e3ed1123a65e449cfab6066e74f98a10abe68790b336b4c3c` |

The thirteen-file full-index patch reconstructs the exact frozen tree from its
base in an isolated Git index. It includes the panel engine, independent authored
arithmetic/stream tests, backend/scratch routing tests, diagnostics, build checks
and proof. None was a production build input at the historical archive closure.
The [source record](forward53-parallel-dispatch.md) retains the dispatch truth
table and phase-aware allocation proof for the later byte-congruent integration.

Ordinary single-worker execution kept the original multi-level helper and original
scratch lifecycle, with no panel allocation or initialisation. Panels required at
least two useful slots under existing effective admission, pool, geometry and
resource eligibility. Optional failure released reservations before original-helper
fallback; mixed levels retained admitted workspace. Historical serial-panel RGB8
gains were explicitly foregone. This does not assert universal serial superiority
or attribute historical RGB16 cost to allocation.

The primary and eight-worker style-zero endpoint passed. The serial Boca RGB8
style-zero endpoint's 40-pair 99% interval was [+0.769086%, +1.131692%], leaving the
mandatory +1% upper bound unresolved without demonstrating a greater-than-1%
regression. The stopping rule left 25 endpoints unstarted. The retained report
accounts for 278 starts (240 ordinary, six preflight and 32 allocation), verifies
all applicable report checks and independent restoration, and records no issues.
Its SHA-256 is `e3ffa775cf723379a449e2a8a07086a7f32333dea193137096e2be277f3f9211`.
The benchmark owns the [complete factual result](https://github.com/emuella/emuella-benchmark/pull/31); any future reuse requires a new
explicit objective and contemporary qualification, not a continuation of this run.

## Classic forward 5/3 caller boundary v1

The prospective [caller-boundary experiment](forward53-caller-boundary.md)
compared the then-production reference A, the archived parallel-dispatch B, and B plus
one owning-helper refactoring C. Its additional full-index patch applies after
recovering B and retains the exact C source and authored tests. The helper owns
panel preparation, component execution and destruction; the effective
single-worker writer skips it and uses its original reusable scalar scratch.
The scoped `inline(never)` decision precedes the measured build. This archive
introduces no enabled codec change, option or automatic qualification campaign.
The source record retains the recovery identities, compiled-code interpretation
and the scope of the prospective evidence separately from historical outcomes.

The complete 126-start prospective acquisition found **no worthwhile supported
repair at this budget**. C/B changed mean time by +2.421219 ms, with a 99%
relative interval [−0.120744%, +0.318239%], leaving benefit and harm unresolved.
C/A satisfied the serial bound for this endpoint, but that does not prove a
boundary benefit. B/A's present result did not reproduce the historical
slowdown. C remains recoverable source only; no repair selection, production
promotion, full-matrix qualification or automatic next experiment follows.
