# Tiled HT depth and durable sparse index calibration

Status: retain 512-pixel tiles and six levels for composed qualification.
Source baseline: `41a43e388d30a8f495c8262f6e0135dcdb56def4`.
The implementation identities below bind this provisional observation before
coordinator checkpointing. This component record does not claim final real-image,
JPP transport or browser qualification.

## Question and selected boundary

Can one tiled HT codestream provide useful large-scene overview and repeatable
native-precision windows after process restart, without retaining a global
admitted index during preparation or sending it to every client? The smallest
probe compares 256/512/1024 tiles, two/five/six 9/7 levels and authored U16 grey,
RGB8 and RGB16 pixels. It measures compressed and descriptor bytes, admitted
metadata capacity, selected packets/blocks/coefficients, workspace and time.
A separate reference uses complete coefficient-plane reconstruction and full
inverse synthesis at each requested reduction. The codec owns this evidence.
Exit requires sparse import agreement, explicit incomplete-data failure, bounded
streaming preparation and a justified geometry selection; actual JPP delivery,
browser execution and multiple HT coding sets have distinct owning increments.

`TiledLossyHtProfile::decomposition_levels` accepts 2, 5 or 6. All matching
unsigned precisions from 8 through 16 use their declared range and level shift;
9–16-bit input/output uses little-endian words. Out-of-range source words fail.
One/three components, 64×64 blocks, one cleanup pass, one LRCP layer, no MCT and
default precincts retain the existing indexed contract. Component selection
preserves request order. Tile axes may be odd and final axes remain actual image
geometry; no image padding is introduced. Every actual axis remains at least
four samples. Two-level public encoding still calls the same scalar search and
analysis through wrappers fixed at two; its rate/admission contract is unchanged.
The indexed per-tile budget remains the floored rate target plus 128 bytes.

The implementation reuses project-authored transform, quantisation, subband,
packet and block mechanisms already described in `ht-lossy-foundations.md` and
`ht-indexed-foundation.md`. The added envelope below is application metadata,
not new codestream or JPIP syntax. No external implementation or protected input
was consulted or copied for this calibration.

## Persistence and immutable source identity

`encode_tiled` still returns a complete retained `IndexedLossyHt`.
`encode_tiled_to_descriptors` instead consumes synchronous pixel, codestream and
per-tile descriptor callbacks and returns `TiledLossyHtSummary`: profile,
main-header range, complete source length, tile count, descriptor-byte count and
peak admitted per-tile metadata capacity. It retains only one tile's metadata;
image-area-sized vectors are absent in this path. Both sinks can be incomplete
on failure. The caller publishes their immutable manifest only after success.

`IndexedLossyHt::sparse(profile, encoded_bytes, main_header)` creates an empty
index. `export_tile_descriptor(tile)` borrows durable bytes;
`import_tile_descriptor(bytes)` validates and inserts one selected tile.
Descriptors can arrive in any order. Duplicates fail and requests crossing an
absent tile fail. Import never reads the original codestream. Planning after
import never scans its headers or unrelated tiles' block metadata. The
`precincts()` and `tile_headers()` arrays contain admitted tiles only, ordered by
tile; consumers must use precinct tile identities rather than assume a complete
ordinal array. A region's precinct indices refer to its borrowed sparse index.

The version-one envelope begins with `EHTIDX01`, then little-endian tile ordinal
(u16), absolute tile-payload offset (u64), local canonical-header length (u32)
and header bytes. For each resolution then component it contains packet-header
length and body length (u32 each), followed by the actual packet-header bytes.
The declared profile fixes the packet count. There are no entropy bodies,
source samples, raw private structs, trailing extensions or host-sized fields.
The ordinal supplies actual geometry from the manifest's image/tile dimensions.

Imports reject unknown versions, extra/truncated bytes, profile disagreement,
noncanonical marker metadata, unsafe scalar quantisers, invalid packet/pass/
bitplane declarations, inconsistent packet bodies, out-of-source ranges and
inconsistent ordered tile-part ranges. The main header is compared byte-for-byte
against freshly generated canonical metadata for the actual tile. Existing
packet admission derives block geometry and quantisers from that header and
real packet headers in bounded temporary scratch. Synthetic zero-filled body
space is used only to admit metadata and is immediately discarded; decoding
always requests actual entropy bytes from the caller. Failed imports add no
usable tile state. A descriptor is bounded to 1 MiB, synthetic tile payload to
16 MiB, and retained metadata to 256 MiB. Native source offsets stay u64,
including on WASM; local allocations and indexing remain tile-sized.

The application must bind manifest, descriptors and compressed source to one
immutable representation identity, including profile and policy inputs. A valid
but substituted descriptor is not authenticated by grammar validation. Selected
descriptors may be fetched separately; actual compressed delivery remains the
standard JPP path. The codec callback must fill every requested codeword or
return an error. Missing or partial compressed cache never becomes a zero block,
and no partially decoded plane is returned after failure.

## Authored observations

Observed 2026-09-07 on AMD Ryzen 9 9950X3D, Linux 7.1.3, Rust 1.97.1, optimised
build with default features. Timings are individual observations on a shared
host, not exclusive-host benchmarks or performance acceptance thresholds.
Capacity counters exclude allocator overhead, process RSS and caller storage.

`authored` in `ht_indexed/tests.rs` defines the exact sample generator: bounded
smooth/faint background, small bright objects, narrow dark diagonals and strong
edges with component offsets. It clips to the declared precision. All 27 geometry
cells use 1040×1138 pixels at 2 bpp and component-zero 8×8 requests at (500,500).
The dimensions deliberately include a final 16-pixel width at every tile edge
and a final 114-pixel height for edges 256/512. RGB sources encode all planes.
Complete per-cell counters are in [the depth CSV](ht-indexed-depth-calibration.csv).

Retain 512/six levels for the next composition. On RGB16 it uses 1,572,864 source
tile bytes, 5,908 total descriptor bytes across nine tiles, and roughly 34 KiB
for one imported full tile. Default-precinct delivery at that request is about
21 KiB, versus about 5 KiB for 256 and 87 KiB for 1024. The smaller tile remains
better for very small full-resolution delivery, but requires more descriptors
and admitted tile states for whole-scene overview. The streaming descriptor sink
removes the need to retain all those states during preparation. Clients should
import only selected tiles and bound their own index lifetime/cache.

Six levels give 672×672 coarsest geometry at 43008², compared with 1344² at five
and 10752² at two. The six-level selected window adds three small blocks over
five levels in this probe. This is resolution progression; it is not a quality
layer, HT placeholder payload, refinement pass or multiple-set measurement.

The [scaling CSV](ht-indexed-scaling-calibration.csv) compares authored U11 grey
at 2048² and 4096² and RGB16 at 2048², using 512/six levels. U11 source tile
storage remains 524288 bytes, selected sparse index 16445 bytes, selected work
28 blocks/77824 coefficients and workspace 760476 bytes while source area
quadruples. The complete retained index and durable descriptor totals scale
with area; the sparse regional path does not. Scaling encoding writes to a
counting/discarding sink, so no complete encoded byte buffer is required.

## Actual large geometry and streaming observation

The additional flat-field probe encodes every source tile and emits every
descriptor through counting/discarding sinks, retaining only the last descriptor.
Inputs are authored unsigned level-shift values (1024 for U11, 128 for RGB8).
The last descriptor is imported and its 32×32 edge region reconstructs without
entropy reads. This proves actual tile counts, edge geometry and bounded
preparation; flat fields do not measure representative satellite compression or
textured descriptor size. An ordinary regression also requires identical peak
metadata capacity for one, two and eight tiles of the same geometry.

| Image and format | Actual tiles/descriptors | Descriptor bytes total/max | Peak source tile bytes | Peak admitted tile metadata bytes | Sparse final-tile bytes | Encoded bytes | Actual last tile | Encode ms |
|---|---:|---:|---:|---:|---:|---:|---|---:|
| 4096² U11 grey | 64 | 12608 / 197 | 524288 | 784 | 725 | 4210 | 512×512 | 425 |
| 43008² U11 grey | 7056 | 1390032 / 197 | 524288 | 784 | 725 | 451698 | 512×512 | 46864 |
| 43280×19058 RGB8 | 3230 | 1062670 / 329 | 786432 | 1712 | 1529 | 252060 | 272×114 | 65582 |

The first large probe rejected a leftover image-area precinct-capacity reservation
in the streaming encoder. The retained candidate uses one tile's capacity and
the table records the repaired run. These flat measurements preceded the wide
entropy repair below; their no-entropy path and source/descriptor format remain
unchanged. Their implementation SHA-256 identities were:

- `ht_indexed.rs`: `a71c7f1341f1fbec336c70cff1352589060cedd615a99946e8c8976db44924b3`
- `ht_indexed/persistence.rs`: `808a7b4d161f4c26c8fe13a6b4a42abf243fc1c546cf272dc2522a51a898d6b9`
- `ht_indexed/tests.rs`: `b0fc73eeb67a58e3073d200708bcb899848d32632bfeb2db358892720be75d2e`
- `ht_lossy.rs`: `8bd626ba256e6d4a52037c9e0d6d264c55b8063ad32b74b561082c83f271341a`

## Wide direct cleanup repair

A composed authored U16 signal exposed a valid first LL block requiring 17
explicit magnitude/sign bits. The legacy direct output stored that field in
`u16` and rejected it, even though the existing encoder and scalar kernel
support 17-bit transformed magnitudes, which can need 18 explicit bits including
sign. This was independent of HTTP, JPP and odd-origin geometry.

The additive `HtVlcCleanupCoefficientOutput<M = u16>` and direct scratch request
support sealed `u16` and `u32` storage. Existing callers retain their 16-bit
checked boundary. Indexed reconstruction opts into `u32`, admits at most 18
explicit bits, preserves those bits through south predictors, and checks the
midpoint, shift and signed output range before materialisation. Its accounting
includes the wider records. Legacy two-level encoding and regional decoding
retain their original storage choice and accounting. The project-authored
scalar reconstruction in `lib.rs` supplies the wide arithmetic; closed derived
implementation files were neither inspected nor changed for this repair.

Regression inputs include positive/negative 17/18-bit words, legacy rejection
without cursor movement, invalid declared widths, out-of-width values,
insignificant data and overflow without output mutation. The composed signal
is encoded as 2048² U16, 512 tiles, five levels and 2 bpp. Region
(247,117,769,513) reconstructs at every discard level 0–5 and agrees with complete
same-representation output. The service owner separately confirmed its direct
file and actual HTTP/JPP regression after this repair.

## Verification and remaining qualification

Focused tests compare two/five/six levels across RGB8, U11 grey, U16 grey and
RGB16, every discard level, reordered components, a cross-tile window and the
16-pixel edge tile. Bounded synthesis agrees byte-exactly with independently
orchestrated complete coefficient-plane/full-synthesis reconstruction of the
same representation. Precision 9–16 flat fields and out-of-range 9–15-bit words
exercise native sample bounds. Empty packets import and reconstruct without
entropy reads. Every truncated descriptor prefix and selected corrupt version,
ordinal, geometry and range fields fail. Imported selected metadata supports
explicit missing-cache failure. Streaming and retained encoders produce exactly
the same codestream and descriptors; descriptor-sink failures stop preparation.

Reproduce the component tests and measurements with:

```sh
cargo test --release -p emuella-j2k-codestream ht_indexed
cargo test --release -p emuella-j2k-codestream depth_geometry_calibration -- --ignored --nocapture --test-threads=1
cargo test --release -p emuella-j2k-codestream deep_profile_scaling_calibration -- --ignored --nocapture --test-threads=1
cargo test --release -p emuella-j2k-codestream large_streaming_geometry_calibration -- --ignored --nocapture --test-threads=1
```

The service owner verified imported selected descriptors with actual HTTP/JPP
cache reads in its seven-test suite at service revision
`5093ded2ab0e2c9315b262b12a85569b5be8ac76`; detailed transport evidence remains there. Browser floating-point agreement,
authorised real-source compression/detail quality, global application cache
budgets, descriptor authentication/lifetime, transport interruption and multiple
HT coding sets remain unproved by this component increment. Full-scene descriptor
size is content-dependent; do not replace measurement with small-fixture
extrapolation. Canonical clean-commit verification remains coordinator-owned.

Focused verification on the final component candidate passed 241 codestream
unit tests and seven HT unit tests (five optional codestream probes omitted),
all-target Clippy with warnings denied for both affected crates, and the
codestream `wasm32-unknown-unknown` check. The 27-cell depth and three-cell
scaling probes were rerun after the wide scratch-accounting change; their CSVs
contain the current counters. No canonical clean-commit gate is claimed here.

Final implementation SHA-256 identities, relative to `crates/`:

| File | SHA-256 |
|---|---|
| `emuella-j2k-codestream/src/ht_indexed.rs` | `2bc09c4f1d5dbecee39487f9d7f0b9f376623e3f081d18fd42b8242b231d50e4` |
| `emuella-j2k-codestream/src/ht_indexed/persistence.rs` | `808a7b4d161f4c26c8fe13a6b4a42abf243fc1c546cf272dc2522a51a898d6b9` |
| `emuella-j2k-codestream/src/ht_indexed/tests.rs` | `31dba3f99eb7443abb875c643566a6e9279b9684596e4c929c185b887a5f0915` |
| `emuella-j2k-codestream/src/ht_lossy.rs` | `4c4c150ca47b40f2be3ee66b22d066c634d1670914c69913890d42ac0d13d2f3` |
| `emuella-j2k-ht/src/lib.rs` | `cd725eeff18cef680d6ad3a8fde34a410a4e7333d347754c50fc6c198d8e0a54` |
| `emuella-j2k-ht/src/wide_cleanup.rs` | `81e0c26b1871c6eace4d9bb56ebeefab75ddd0092b27cc6e17fae80f90c795de` |
