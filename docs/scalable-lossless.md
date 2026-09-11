# Scalable classic lossless encoding

The public owned `encode` route admits raw, single-tile Part 1 lossless D2
images with unsigned U8 or U16_LE greyscale/RGB samples, LRCP and reversible
5/3. Both planar and interleaved input are supported, including row padding.
Each axis is 4–32768 and each component has at most 64 Mi samples. This includes
6650 × 7054 and the larger authored 8001 × 8003 probe. RGB uses the existing
reversible component transform. This is a bounded implementation profile,
not a claim of general JPEG 2000 conformance.

An additive [eight-component native U16 route](native-eight-components.md) uses
the same writer without MCT, with `ColorModel::Unknown` and at most 32 Mi
pixels (256 Mi aggregate samples). Its supplied positions have no inferred
colour or spectral interpretation; both layouts retain every band.

`encode_with_limits(image, &options, &limits)` is additive; existing
`EncodeOptions` struct literals remain valid. `lossless_encode_requirements`
checks geometry, options and the working budget without reading samples.
The explicit API rejects JP2, metadata, tiles, D0/D1, other precisions, lossy
options and other component models. It never silently ignores a requested
limit. Ordinary `encode` retains the previous routes for those cases,
including 9–15-bit greyscale and previously accepted thin D2 images with
axes below four or above 32768. Explicit limits continue to reject these shapes.
HT APIs and all decoder admission rules remain unchanged. Eight-component full
caller decode now stages complete output to preserve destination bytes on failure. The D2 writer uses bounded code-block batches with the `parallel` feature,
using the current Rayon pool and the explicit working-memory allowance.
The one-worker path keeps the serial writer.

## Opt-in selective arithmetic bypass

`encode_lossless_bypass_with_limits(image, &options, &limits)` selects COD
style 1 on the same explicit-limit raw D2 sample models and geometry.
`lossless_bypass_encode_requirements` checks its working envelope before
samples are read. Neither API changes `EncodeOptions`, the existing limits or
requirements types, or the style-zero defaults. RGB retains reversible MCT;
eight native U16 components retain their positional no-MCT interpretation.
The lower-level codestream equivalents are `encode_lossless_d2_bypass` and
`lossless_d2_bypass_requirements`.

The bypass admission adds `512*B` to the shared working terms below before
choosing W. Actual segment lengths come from the production Tier-1 collector.
A checked fixed record holds at most 55 lengths per block: the 31-magnitude-plane
D2 bound permits 91 complete passes, comprising one ten-pass MQ segment and
27 subsequent raw-pair/cleanup pairs. An authored full-magnitude test reaches
that bound. The record's size is mechanically limited to 512 bytes; per-packet
metadata vectors reserve their exact block capacity and do not grow. All
metadata for the current packet, including earlier subbands, is covered by B.
The additional raw length fields remain within the existing packet-header
allowance. Each serial or parallel worker's temporary collector and codeword
coexist with its fixed result record inside the existing 4 MiB local allowance.

Parallel bypass follows the same bounded joined batches and stream-order
append policy. Each slot owns its collector; actual lengths follow the encoded
block into packet assembly. Errors publish no owned output and every started
batch is joined before an error returns. Reduced working budgets lower worker
admission after the extra shared metadata term; the ordinary style-zero serial
minimum and requirements values remain unchanged.

The [development operating point](tier1-bypass-feasibility.md) records the
selection evidence and distinguishes the codestream-only exploratory clock
from subsequent facade encode/decode qualification, which includes output
packing in the requested layout. This opt-in adds no broader style, geometry
or decoder rejection policy.

## Admission and allocation contract

`LosslessEncodeLimits` defaults to 4 GiB `max_working_bytes` and 1 GiB
`max_output_bytes`. The latter bounds returned vector **capacity**, not just
codestream length. It must be at least 128 bytes and no greater than `u32::MAX`
or `isize::MAX`, whichever is smaller. Tile-part length remains checked.
An insufficient working budget fails before coefficient storage is allocated.
The output budget is checked at each actual append; a working-admitted image
can still fail during encoding if its compressed output exceeds that budget.
No partial codestream is returned by the owned API.

Let P = width × height, C = component count, S = P × C, A = the larger axis,
O = the output capacity allowance, and B = the sum of the 64 × 64 block-grid
sizes across all seven D2 subbands and components. Checked arithmetic computes:

```text
working_bytes = 4*S + 4096*B + 12*A + 2*O + (4 MiB)*W
```

W is the lesser of the current pool size, B and the number of 4 MiB worker
allowances that fit after the shared terms. Without `parallel`, W is one.
A budget below the serial minimum fails; a tighter admitted budget reduces W.
Requirements are resolved in the calling pool, so query and encode should run
in the same pool to observe the same allowance. The existing options, limits
and requirements struct shapes are unchanged. This is an adaptive pool policy;
it uses the calling or global Rayon pool and does not construct a dedicated
pool or promise an exact count of participating threads. The global pool may
initialise on first use.
A batch contains at most W results, and at most W Tier-1 calls execute at once.
Thread stacks and Rayon pool infrastructure belong to the caller's pool.

`total_component_samples` is S, not P. The bound is a conservative admission
ceiling; the encoder does not reserve that amount in advance. Its terms follow
actual allocation lifetimes:

- Exact-capacity i32 coefficient vectors use 4*S bytes. Input components are
  read directly at their byte stride/step; planar RGB has no packed input copy.
- One reusable DWT vector contains three i32 lines, at most 12*A bytes. It is
  released before entropy coding.
- Descriptors for one packet, two encoding tag trees for one subband and the
  packet header are bounded by 4096*B. A descriptor is below 64 bytes. Each
  tree has fewer than three times its leaf count in nodes (including thin
  grids); a node is below 32 bytes. There are at most 32 tree levels. Even
  allowing 32 missing-bitplane decisions per node, block pass/length fields,
  bit stuffing and vector growth/reallocation, a header consumes less than
  1024 bytes per leaf. The 4096-byte combined allowance covers descriptors,
  both trees, header growth and their vector bookkeeping. Small fixed vector
  metadata also fits the separate local allowance.
- Each exclusive worker slot handles one at-most-4096-coefficient block, using existing reusable
  coefficient-state, sign and magnitude buffers. The block's compressed bytes
  are accumulated locally before the checked output append. At most three
  binary decisions per coefficient per bitplane, 31 bitplanes, at most 15 MQ
  renormalisation shifts per decision, seven usable bits per emitted byte and
  termination give less than 1 MiB per block. A growing vector, including
  its old allocation during reallocation, plus padded Tier-1 state and small
  local vectors fit the 4 MiB per-slot term, including the slot-array allocation. Scratch and
  codeword vector capacities are retained and reused across batches and subbands. U8/U16 RCT followed by four one-dimensional
  lifting stages stays below this bitplane bound; coding remains the existing
  baseline Tier-1 implementation.
- 2*O deliberately covers both old and new output allocations during growth,
  even though one current output allocation is excluded from the definition
  of additional working memory. This extra conservatism also means the
  diagnostic can compare its total encoder allocation peak directly with W.
  `try_reserve_exact` requests bounded capacity, and reported capacity is
  checked against O. Allocation failure returns an error at these large
  explicit reservations. This is not a promise that every internal small
  allocation is fallible or that the operating system will supply the budget.

The owned API excludes caller-owned input, unrelated application allocations,
allocator bookkeeping, thread stacks, mapped libraries and later verification.
Requested allocation bytes are not process RSS. The allowance includes
conservative output growth overlap; it does not infer a smaller peak by
subtracting final output capacity from a peak observed at a different time.

`encode_into` keeps its append semantics and uses the default owned encoder as
staging. The caller's existing output and its growth are outside this owned
invocation's limit. Callers requiring the explicit allocation contract should
use `encode_with_limits` and manage their destination separately. Failure of
encoding or the final fallible destination reserve preserves existing bytes.

## Implementation and preserved boundaries

The new writer reuses existing RCT, DWT, subband geometry, Tier-1 and packet
header mechanisms. For each LRCP packet it encodes blocks, builds the header
from their lengths, and inserts that header before the packet body in the final
vector. It does not retain complete segment, packet and codestream copies at
once. Inserting a header moves only the current packet body. Existing small
D2 streams remain byte-identical in the boundary tests.

The original 16 Mi-sample limit dates to initial public revision `7cfa186`;
there is no checked-in derivation that establishes it as a format limit.
The constant and every old use are retained. Its inventory comprises:

- U8/U16 greyscale validators, RGB shape validation and
  `checked_component_sample_count`, including legacy encoder helpers;
- `unsupported_construct`, no-decomposition/decomposition compatibility,
  irreversible default-precinct, high-bit-depth, multitile greyscale,
  irreversible selective and native D2 multitile profile gates;
- HT native-grid classification/preparation, heterogeneous reversible,
  scalar-derived and six-level reduced envelopes, and lossless multitile decode;
- the alias `MAX_HTJ2K_REDUCED_COMPONENT_SAMPLES` and its independent users.

The new route bypasses none of those decoder or HT checks. Grey/RGB upper geometry
and the eight-component route’s separate 32 Mi-pixel ceiling fit the existing full native decoder's separate 64 Mi samples/component
and 256 Mi aggregate sample ceilings. Decode capability is independently
exercised by roundtrips; encoder admission is not itself decode proof.

## Calibration and reproducible probes

The initial question was whether component-size rejection could be replaced by
checked resource admission without copying whole compressed buffers. The
small representative probe used 67 × 65 grey U16, D2 and xorshift32 seed
`0x713b9d21`, taking low little-endian bytes from each successive state.
Baseline source was `2568f1c40c83a40f527c7ee8f1600af511e046d0`.

- Input SHA-256: `932510b44ee74badd7c8248c70bde08760b30d7f5ac36ac2a20c301e29aa839b`.
- Output SHA-256: `0ffc2bc3f1dceb6c19fcc0372dd970790d6a47bef6606a206abf43cb71ab870a`.
- Baseline: 9422 encoded bytes/capacity, 53,858 peak requested encoder bytes,
  exact independent invocation of Emuella decode; 0.001070 s observed encode.
- First retained candidate: identical encoded bytes/hash, capacity 10,448,
  peak 47,524 requested bytes, exact decode; 0.001387 s observed encode.

These single small timings are not performance conclusions. The retain decision
is based on exact bytes, the allocation/lifetime model and successful explicit
budget tests. Larger qualification binds the measured source revision and
build to its evidence; this baseline checkpoint is not final large-image proof.

The self-contained `scalable_lossless` integration test covers U8/U16,
grey/RGB, both layouts, odd and 64/128/256 processing boundaries, tiny and long/thin legacy
route agreement, exact byte identity, input metadata/extent checks, checked
geometry/arithmetic, exact working-budget boundaries and runtime output-budget
failure. It also checks that old HT and D1 guards still reject the larger shape.

The opt-in diagnostic is an example in the non-published test-support crate:

```sh
CARGO_TARGET_DIR=/your/authorised/scratch/target \
  cargo run --release -p emuella-j2k-test-support \
  --example lossless_allocations -- 8001 8003 3 16
```

Arguments are width, height, components (1/3/8), bits (8/16, with 16 required
for eight components) and optional
`planar`/`interleaved` layout (default interleaved). It prints input
and output SHA-256, encode duration, output length/capacity, requested encoder
allocation peak, admission bound and exact decode result. Inputs and decoded
pixels remain in memory. Run serially at roughly 1, 4, 16, 47 and more than
47 million pixels, covering all four sample/component models. Large probes are
excluded from ordinary tests. The example's diagnostic-only `GlobalAlloc`
wrapper delegates unchanged pointers/layouts to `System`; it is never linked
into codec libraries and is the sole local unsafe exception in that example.
Atomic counters count all requested live bytes and conservatively count old
and new requests during reallocation. Measurement resets after input and
requirements creation, stops immediately after encode, and excludes decode
verification. It reports total encoder peak as a conservative upper bound on
additional working memory, because final output capacity alone cannot identify
the output allocation at every earlier peak. It asserts that even this total
peak fits the working bound, and separately checks retained output capacity.

## Opt-in production-path stage diagnostics

`encode_lossless_d2_profiled` in the low-level codestream crate measures the
same validated scalable writer. Its ordinary entry point instantiates the
shared implementation with profiling disabled at compile time. It preserves
existing sample, arithmetic, budget and output checks. This is an additive
`std` diagnostic API; it changes neither facade options nor encode admission.

The non-published test-support example reads an already-authorised packed RAW
input and its matching raw codestream once, then reports aggregate JSON only:

```sh
CARGO_TARGET_DIR=/your/authorised/scratch/target \
  cargo build --profile perf -p emuella-j2k-test-support \
  --features parallel,simd --example lossless_diagnostics
EMUELLA_DIAGNOSTIC_REVISION="$(git rev-parse HEAD)" RAYON_NUM_THREADS=1 \
  /your/authorised/scratch/target/perf/examples/lossless_diagnostics \
  /your/approved-store/input.raw /your/approved-store/input.j2k \
  4851 2752 1 16 interleaved
```

Arguments are RAW path, codestream path, width, height, components (1/3/8),
bits (8/16, eight components require 16), and `planar` or `interleaved`.
RAW storage must be packed in the supplied layout, with U16 words little-endian.
The bounded driver accepts matching unsigned unit-sampled classic D2, reversible
5/3, LRCP, one layer/tile, default precincts, style-zero 64×64 blocks, zero
origins, and no SOP/EPH. RGB decode permits existing no-MCT or RCT streams;
encoding retains the existing RGB RCT policy. Eight-band decode uses the full
owned native component route. No prepared selective route is substituted.
`decode_native_d2_profiled` shares the ordinary core `decode` implementation,
including inspection, component metadata and native output construction. The
one-worker gate is checked against the current Rayon pool when enabled; multiple
workers are explicitly unsupported by this diagnostic. Component coding or
quantisation overrides, tile COD/QCD overrides, ROI and POC are outside the
frozen diagnostic profile; ordinary decoder admission remains unchanged.
Invalid, mismatched or unsupported input returns an error JSON and nonzero exit.
A parity or measured allocation failure also exits nonzero. Inputs, generated
streams and reconstructed pixels remain in memory; the driver never saves them.
Its only file reads are the two explicitly supplied paths.

For the corresponding RGB8 dimensions use `4851 2752 3 8`; for native eight-band
U16 use `1213 688 8 16`. The example does not acquire inputs or grant rights to
use a path. Keep any redirected JSON with its authorised evidence owner.
The revision environment field is explicitly a caller-supplied label, not
proof of binary provenance. Record source/tree, toolchain, build command and
binary hash externally. Feature booleans reflect this example's feature
selection; they do not assert that a particular SIMD kernel executed.

Encode reports disjoint conversion/level-shift/RCT, forward DWT, subband and
block preparation, checked Tier-1, packet header and assembly intervals.
Assembly includes output appends, moving each current packet body to insert its
header, and closure. Total also contains validation, accounting, local
allocation/destruction and loop overhead; the remainder is reported explicitly.
The Tier-1 interval includes its internal block preparation. Completed calls,
included blocks, coefficient slots, coding passes and codeword bytes are actual
work counters. Inner entropy-operation counts are unavailable and reported as
`null`, not invented zeros. The writer uses the same checked baseline Tier-1 in bounded batches when
multiple workers are admitted. Its Tier-1 stage is elapsed batch wall time, not
the sum of concurrent worker times; preparation within each job is included.
The additive `encode_lossless_d2_execution_profiled` returns the encoded bytes,
the original `LosslessEncodeTimings`, and a separate non-exhaustive
`LosslessEncodeExecution`. The existing timing struct and
`encode_lossless_d2_profiled` signature retain their original shapes.
`LosslessEncodeExecution` fields `effective_workers`, `participating_workers`
and `max_batch_blocks` describe the
admitted slots, observed distinct Tier-1 threads and largest batch. Distinct participants
can exceed W when memory limits the slots below the pool size: a slot can run
on different pool threads in successive batches. W bounds simultaneous work. Participant
tracking exists only in the profiled instantiation, has at most B entries and
fits the existing descriptor allowance. Serial execution reports one worker.

Decode's `production-one-worker` identity retains the ordinary adaptive Tier-1
call and records the backend that actually ran, including packed dense blocks.
It preserves the ordinary fused inverse-RCT/sample-conversion routine for RGB8.
That interval is combined; its separate RCT and conversion fields are `null`.
Without that fused route, the existing separate intervals remain measurable.
Inner Tier-1 operation counters and segment-acquisition attribution are unavailable
in this collector and explicitly `null`. The existing detailed checked profiler
is unchanged and may be run separately for reference work counts; it must have a
separate identity and its timings must not supply production stage fractions.

The core output builder is shared, including PAN's move-only output,
specialised RGB U8/U16 interleaving, generic native-band interleaving and planar
moves. The recorded packing route is set in that actual branch. Core output
construction includes packing and metadata, so its nested packing interval must
not be added again. The outer interval includes full core decode; an explicit
remainder retains work not attributed to the named intervals. Input reads,
hashes and verification are outside both measured operations. The example
verifies native samples against RAW, ordinary decode against RAW, its encode
round trip, and profiled versus ordinary core encode bytes. The supplied decode
stream need not equal the generated stream: RGB no-MCT and RCT differ.

Requested encoder allocation accounting resets immediately before profiled
encode and is captured immediately after it, before decode or verification.
It excludes already-loaded inputs; the total peak conservatively includes
output reallocation overlap. This is requested allocation traffic, not RSS.
The example-only allocator follows the existing allocation probe's forwarding
contract and is never linked into codec libraries.

Clock reads and the allocation meter perturb execution. Production collection
is qualified only at one worker, where the ordinary tile, block and component
parallel gates are also closed. Its full-D2 dispatch keeps the ordinary adaptive
backend, fused conversion and packing branches. It is not proof of parallel
fidelity or an uninstrumented throughput result. Use a separate ordinary process
for headline timing, with matched inputs/build settings and explicit thread
policy. The retained checked detailed collector intentionally changes Tier-1
and fused-conversion routes; sample equality does not make its stage fractions
representative of ordinary decode. Zero observed work, such as inverse RCT
without MCT, remains zero rather than implying an unavailable measurement.

Focused authored checks cover grey/RGB U8/U16 and eight-band U16, full-range
words, both layouts, block boundaries, ordinary byte identity, native decode
parity, actual packed-dense dispatch, fused RGB8 conversion, shared PAN output
allocation identity, separate checked-reference counters, no-MCT RGB D2 and
unchanged resource/error gates. Parallel-feature tests use a local one-worker
pool and separately verify rejection at two workers:

```sh
cargo test -p emuella-j2k-test-support --test lossless_diagnostics
cargo test -p emuella-j2k-test-support --test lossless_diagnostics --features parallel,simd
cargo test -p emuella-j2k-test-support --example lossless_diagnostics
cargo test -p emuella-j2k-core shared_pan_output_moves_the_original_plane
```

## Bounded parallel encoding calibration

The design question is whether fixed batches of independent 64×64 code-blocks
provide useful full-image scaling while retaining serial byte order, reusable
Tier-1 storage and explicit admission. The smallest probe is an authored odd
513×515 U16 image with contrasting sparse and dense blocks, run in local pools
of 1, 2, 4 and 8 workers. The codec owns scheduling, resource and failure
observations. Full-image development measurements use an authorised existing
input and keep diagnostic allocation/CPU collection separate from headline
wall timings. Exit requires exact streams and decoded samples at every worker
count, bounded live results and scratch, joined failures, useful scaling and
no meaningful one-worker regression; otherwise reject or revise this candidate.

The candidate uses a fixed array of worker slots, each owning reusable Tier-1
scratch and one compressed result. A batch contains at most one block per slot.
All jobs finish before serial raster-order append and the next batch. Shared
coefficient planes are immutable during coding. Output-budget errors after a
join drop the private owned output; no job outlives its borrowed input or error.
There is no channel, unbounded completed-result collection or per-image job
queue. Slot ownership is exclusive during each job; slots can migrate between
Rayon threads across batches.


The separate Linux `lossless_parallel` example accepts the same seven input
arguments as `lossless_diagnostics`, followed by the requested worker count.
Build with `--profile perf --features parallel --example lossless_parallel`.
It measures ordinary core encode and decode inside the requested local pool,
using the existing example-only forwarding allocator. Each operation reports
peak requested bytes, wall time and per-task CPU deltas from `/proc/self/task`;
`getconf CLK_TCK` supplies their resolution. Threads with a nonzero CPU delta
are observed participants at that resolution, not an exact inner-kernel worker
count. Very short operations can have zero observed CPU ticks. Encoder Tier-1
participation is additionally collected in a separate profiled invocation and
its bytes must equal ordinary encode. Input loading, hashes, pool startup and
sample verification are outside measurement. CPU snapshots bracket the wall
interval and include their small collection overhead. Diagnostic allocation
and clock overhead perturb these runs: use ordinary benchmark processes for
headline comparisons. No decoder scheduling policy changes in this candidate.
The ordinary decoder measurement does not substitute prepared reconstruction.


The initial bounded candidate passed authored parity, budget fallback and
joined-error checks, and its Mansfield development gate retained serial
performance while improving eight-worker encoding. The source identities,
per-case intervals, allocation and CPU observations, and remaining qualification
boundary are recorded in [the parallel qualification](tier1-parallel-qualification.md).

## Explicit consumer contexts and bypass execution observations

The test-support `lossless_bypass_batch` request accepts `execution_context` as
`direct_global` or `nested_pool` (the historical default). The former configures
the process-global pool and invokes the facade from an ordinary application
thread. The latter uses a local `ThreadPool::install`, preserving the decoder's
existing nested-call guard. Requirements are queried in the same context as
encoding. Optional `max_working_bytes` and `max_output_bytes` set explicit
application limits; omitted values preserve the previous production defaults.
Neither pool width nor an admission query is proof of actual decode participants.

Its separate `profile` operation emits no headline samples. It records native
encoder stages, effective slots and observed distinct Tier-1 participants,
checks ordinary facade byte parity and reconstructs every sample. The
`test-fixtures`-gated `encode_lossless_d2_bypass_execution_profiled` observes the
existing bypass writer without changing ordinary scheduling or coding. Serial
bypass's Tier-1 interval includes subband preparation and output appends; this
combined boundary is labelled in the example output. Parallel bypass retains
separate preparation and assembly intervals. Parallel decoder stage and worker
telemetry is unavailable in this collector and remains explicitly null.

The allocation-only example accepts optional `WORKING_BYTES OUTPUT_BYTES` after
its original arguments so resource probes can use the same application limits.
Requested allocation peaks, whole-process RSS and ordinary operation timings
remain different observations. The authored `lossless_bypass` integration test
checks direct/nested sample parity, same-context requirements and unchanged
encoder bytes across one/eight-worker pools. No public struct or default changes.
