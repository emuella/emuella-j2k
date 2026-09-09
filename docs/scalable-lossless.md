# Scalable classic lossless encoding

The public owned `encode` route admits raw, single-tile Part 1 lossless D2
images with unsigned U8 or U16_LE greyscale/RGB samples, LRCP and reversible
5/3. Both planar and interleaved input are supported, including row padding.
Each axis is 4–32768 and each component has at most 64 Mi samples. This includes
6650 × 7054 and the larger authored 8001 × 8003 probe. RGB uses the existing
reversible component transform. This is a bounded implementation profile,
not a claim of general JPEG 2000 conformance.

`encode_with_limits(image, &options, &limits)` is additive; existing
`EncodeOptions` struct literals remain valid. `lossless_encode_requirements`
checks geometry, options and the working budget without reading samples.
The explicit API rejects JP2, metadata, tiles, D0/D1, other precisions, lossy
options and other component models. It never silently ignores a requested
limit. Ordinary `encode` retains the previous routes for those cases,
including 9–15-bit greyscale and any previously accepted thin D2 images.
HT APIs and all decoder admission rules remain unchanged. The new D2 writer
runs sequentially even with the `parallel` feature; byte identity does not
imply unchanged throughput.

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
working_bytes = 4*S + 4096*B + 12*A + 2*O + 4 MiB
```

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
- Tier-1 works on one at-most-4096-coefficient block, using existing reusable
  coefficient-state, sign and magnitude buffers. The block's compressed bytes
  are accumulated locally before the checked output append. At most three
  binary decisions per coefficient per bitplane, 31 bitplanes, at most 15 MQ
  renormalisation shifts per decision, seven usable bits per emitted byte and
  termination give less than 1 MiB per block. A growing vector, including
  its old allocation during reallocation, plus padded Tier-1 state and small
  local vectors fit the 4 MiB term. U8/U16 RCT followed by four one-dimensional
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

The new route bypasses none of those decoder or HT checks. Its upper geometry
also fits the existing full native decoder's separate 64 Mi samples/component
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
grey/RGB, both layouts, odd and 64/128/256 processing boundaries, tiny legacy
route agreement, exact byte identity, input metadata/extent checks, checked
geometry/arithmetic, exact working-budget boundaries and runtime output-budget
failure. It also checks that old HT and D1 guards still reject the larger shape.

The opt-in diagnostic is an example in the non-published test-support crate:

```sh
CARGO_TARGET_DIR=/your/authorised/scratch/target \
  cargo run --release -p emuella-j2k-test-support \
  --example lossless_allocations -- 8001 8003 3 16
```

Arguments are width, height, components (1/3), bits (8/16) and optional
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
