# Classic forward 5/3 parallel dispatch v1 (qualification pending)

This source treatment restores the archived panel engine with a new dispatch and
scratch-lifetime policy for the bounded classic lossless D2 encoder. Production
selection requires fresh independent confirmation. Source-route equality and
correctness checks supply no speed guarantee. Historical serial-panel RGB8 gains
are deliberately foregone; no claim attributes historical RGB16 cost to allocation.
The closed original experiment remains in [the archive](performance-candidates.md).
No decoding, input conversion, level shift, RCT, exponent scan, header, Tier-1, MQ,
packet or W-batch algorithm changes.

## Plan and execution

`analysis53.rs` owns an immutable plan with one or two explicit active geometries,
validated edge counts, the original plane stride, checked true allocation extent,
full-resolution-to-LL order, panel width and at most eight logical slots. The
bounded transform API retains the caller's arithmetic-proof obligation; use the existing
checked transform for arbitrary input. Following levels must describe exactly
the previous LL dimensions and signed-32 intermediate storage.

The frozen ordinary policy uses width 16. Before panel planning, allocation or
zero initialisation, the encoder selects the original multi-level scalar helper
and its original reusable `Vec` scratch lifecycle whenever effective admission
is one worker, first-level geometry has fewer than two useful panel slots, or
all work is small. Effective admission already incorporates the existing pool
and unchanged Tier-1 memory budget; this dispatch creates and probes no pool.
No-parallel, no-std and ordinary WASM builds therefore use the original helper.

Logical slots are bounded by admitted W, eight, first-level panel count and row
count. Plans are tried at the existing halving sequence, but stop before one
slot. A memory-ineligible plan requests no storage. Failed optional multi-slot
reservations are dropped before retry or original-helper fallback, all before
coefficient mutation. A scalar-panel backend is never an ordinary fallback.
A level with fewer than 4096 active samples, a high-first axis or fewer than two
useful panel slots uses the original scalar level transform. A mixed plan retains
its admitted storage and reference region through both levels. Partial-window
tails retain the donor execution. The encoder's admitted phases are low-first;
transform tests independently exercise high-first fallback.

Widths 8/16/32 and explicitly forced scalar panels remain available only to the
separate diagnostic feature and authored tests. Ordinary timing builds contain
no policy state, route counters or instrumentation. There is no new facade
option, profile or bit-depth exception.

Each slot contains a complete-height row-major panel and one reusable horizontal
line. A window of at most q panels first gathers contiguous row segments and
lifts them from an immutable source plane. Prediction leaves original even rows
available; update starts only after all odd rows in that panel are complete.
Adjacent columns form the inner loop. Rows remain interleaved in scratch.
After all jobs join, disjoint destination row groups copy contiguous segments
into the existing low-first vertical layout. A second join ends every scratch
borrow before reuse. Panel boundaries never extend signals or split horizontal
wavelet lines. After the final vertical window, disjoint complete-width row
groups use explicitly assigned line scratch and join before LL recursion.

Components are sequential and share the same workspace. There is no nested
component pool, per-column line lifting in a panel, full transpose, per-job
scratch allocation or `map_init`. One-slot work schedules no Rayon jobs. Partial
panels use only their valid lanes. Destination ownership ends at
`(height - 1) * stride + width`, including a short final row; neither padding nor
coefficients outside the active LL region are changed.

## Bounded arithmetic

This proof applies to the existing admitted unsigned U8/U16 sample models and
two levels, not arbitrary i32 values. Level shift bounds unsigned 16-bit samples
by absolute value 32768. The unchanged RCT keeps luminance in that interval and
bounds each difference by 65535. Its sums also fit i32. Eight native components
have no RCT, so the same conservative initial bound M=65535 covers all admitted
components and both precisions.

Given input absolute bound M, prediction has absolute bound at most 2M. The
update's two predicted neighbours and rounding bias have absolute sum at most
4M+2; the updated low sample is bounded by 2M+1. Replacing M by 2M+1 after each
axis therefore bounds all coefficients after four axes by 1048575. Every
intermediate neighbour sum, bias, subtraction and addition fits i32. A singleton
axis is unchanged. Repeated end neighbours preserve those bounds. Arithmetic
right shifts match the reference's floor rounding for negative values. The
panel implementation independently implements the vertical arithmetic; the
unchanged checked line routine supplies the test oracle after each axis.

## Whole-operation live-allocation proof

Let S be all component samples, B the existing total block count, A the larger
original axis, O the output capacity allowance and W the unchanged admitted
Tier-1 worker count. Existing queries remain byte-for-byte unchanged:

```text
Q = 4*S + 4096*B + 12*A + 2*O + (4 MiB)*W
```

Selective bypass additionally retains its existing 512*B term. No descriptor,
output or bypass allowance is treated as spare transform capacity.

For panel width b, full height H and logical slots q, the sole workspace request
is exactly:

```text
D = 4 * max(3*A, q*(b*H + A))
```

The 3*A minimum allows any level to use the original scalar routine in place.
Reference-only transform plans request exactly 12*A; ordinary wholly serial
encoding bypasses such plans and uses the original helper's scratch instead. Each retained slot's panel has b*H
elements and its horizontal line has A elements, even at later smaller levels.
Plan/config metadata is fixed stack storage: two optional configurations, scalar
capacity fields and no allocated level/job arrays. Iterators borrow slices;
the existing caller-owned Rayon pool supplies scheduling infrastructure and
stacks. This does not create a second pool or charge an assumed OS-thread count.

Preparation checks `D <= 12*A + (4 MiB)*min(W, 8) - 4096`. Capping the
optional allowance before usize arithmetic keeps unusually wide admitted pools
safe on 32-bit hosts; it uses a conservative subset of the existing W term and
does not cap Tier-1 workers. The 4096-byte reserve keeps
fixed allocation bookkeeping within the original local allowance; the outer
coefficient vector holds at most eight Vec descriptors (192 bytes on 64-bit
hosts), with no transform job metadata allocation. The new plan and workspace
headers and diagnostic identity tables themselves live on the stack. During
conversion/RCT, only the original exact-capacity coefficient vectors and their
outer descriptor vector exist. During transform, those same vectors coexist
with D. There is no codestream output, packet descriptor/tag-tree/header
allocation or Tier-1 worker storage yet. Thus this phase fits the unchanged
coefficient plus local/DWT terms. D is explicitly dropped before exponent/spec
vectors, output construction, serial Tier-1 scratch or parallel slots are
constructed. Every subsequent phase retains the original allocation proof and
worker admission. This is a maximum across disjoint lifetimes, not a sum that
counts the same allowance twice.

Panel `try_reserve_exact` is followed by an actual capacity check. Zero
initialisation is part of successful optional preparation. Workspace growth
discards the old allocation before requesting a larger one; old/new requests
never overlap. Reuse of the same panel plan requires no allocation or
initialisation. On optional failure, no panel allocation survives entry to the
original helper. That helper's existing `Vec::resize` lifecycle and allocation
behaviour remain unchanged, including reuse across components and resizing for
successive LL levels. Its original allocation semantics are not converted into
a new recoverable-allocation guarantee. No panel workspace is requested or
initialised for ordinary single-worker calls.

Conversion/RCT, serial transformation and panel transformation all precede
entropy and output construction. Both optional panel storage and original scalar
scratch are dropped before exponent/spec vectors, packet state, output growth
and Tier-1 worker slots. Thus the earlier Q proof remains a maximum over disjoint
phases. Returned queries, the serial minimum, Tier-1 admission and output bounds
are unchanged. There is no restart after partial mutation. Panic unwinding joins
started Rayon jobs before borrowed planes/workspace can be released.

The proof concerns requested allocations, not allocator implementation overhead,
process RSS, thread stacks or CPU time. The existing allocation-only example
measures fresh whole-operation peaks, including output/growth, against the
unchanged query; those observations belong with the benchmark record.

## Diagnostics and reproducible tests

The separate `classic-execution-diagnostics` build adds scoped
`with_forward53_diagnostic_policy` forcing. Ordinary code has no forcing state,
clock, atomic or participant bookkeeping. Existing `observe_lossless_encode`
returns a `forward53` record with per-level backend, width, logical slots,
workspace capacity, gather/lift/join wall time, scatter/join wall time and
horizontal/join wall time. These are elapsed stage times including barriers,
not useful-lifting durations or sums of worker time. Existing scalar fields keep
their original meanings. The encode observer separately reports actual original-helper calls and retained
original scratch capacity, plus panel preparation attempts and initialised bytes.
Wholly serial calls record two reference levels and zero panel storage/attempts.
The ordinary total DWT interval includes preparation,
zero initialisation and workspace destruction. Stage fields nest inside it.

Actual overlapping jobs and distinct thread participants are separate fields.
The fixed 64-identity observer reports `None` on overflow instead of inventing
an exact count. Jobs can migrate across windows and participant count can exceed
logical slots. Instrumentation perturbs execution and supplies no headline
speed measurement.

Focused tests compare entire arrays after each axis and level, including
padding and untouched subbands, to independent checked scalar line arithmetic.
They cover singleton/thin, odd/even, both phases, widths 8/16/32, partial
panels/windows, high-range alternating/dense/sparse samples, short final rows,
workspace reuse and malformed geometry. A barrier-driven injected gather panic
proves every started job has completed, no scatter occurred, and the workspace
can be reused. No sleeps or scheduling-dependent allocation counts are asserted.

The authored encode matrix compares full streams for every backend with
1/2/4/8 workers, both styles, full-range U8/U16, colour extremes, zero coefficients,
one/three U8/U16 components and eight U16 components, padded planar input and
stepped interleaved input. Eight U8 components remain rejected. Equal full bytes include exponent,
block and packet metadata. Native decoding checks original packed samples.
Tight budgets preserve serial admission and output exhaustion returns no stream.
Separate preparation tests exercise slot reduction, memory rejection before
allocation, and injected failure after optional reservation, with release before
retry/fallback. Ordinary-policy tests assert backend and scratch paths at
1/2/4/8 workers, reduced effective budgets, one-slot geometry, small and mixed
LL levels, and changing geometries/budgets. Independent transform tests retain
high-first phases, partial windows and joined panic/reuse. None of these tests
uses a corpus input or establishes performance. Canonical checks run these tests explicitly.

```sh
cargo test -p emuella-j2k-transform --features parallel,classic-execution-diagnostics analysis53
cargo test -p emuella-j2k-codestream --features parallel scalable_lossless::forward53_tests
cargo test --release -p emuella-j2k-test-support --features parallel,classic-execution-diagnostics --test forward53_panels
cargo check -p emuella-j2k --no-default-features --target wasm32-unknown-unknown
```

Independent OpenJPEG reconstruction, fresh allocation observations, ordinary
end-to-end exploration and the qualification disposition remain external evidence
requirements. This candidate does not claim they passed merely because authored
checks or compilation pass.

The codec-owned aggregate diagnostic driver uses the same feature observer:

```sh
cargo build --profile perf -p emuella-j2k-test-support --features parallel,classic-execution-diagnostics --example forward53_diagnostics
# Arguments: authorised packed RAW, width, height, components, bits, workers,
# style (0/1), reference|scalar|parallel, and panel width (8/16/32).
```

It emits hashes and counters only; it does not persist source samples,
coefficients or codestream payloads. File access supplies no new corpus rights.
