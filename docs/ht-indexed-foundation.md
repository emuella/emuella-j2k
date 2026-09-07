# Indexed tiled HT exploration

Status: historical two-level foundation. The current experimental depth and
durable sparse index contract is in [ht-indexed-persistence.md](ht-indexed-persistence.md);
its next composed candidate uses 512-pixel tiles and six levels. The observations
below retain the original two-level calibration context.
Source baseline: `aa7090c23cce62437cefe5b441e971b1bd4320b5`.

The question is whether the existing project-authored lossy HT algorithms can
prepare a large single codestream from bounded tile input and service repeated
small windows using retained packet/block metadata. The smallest probe compares
256, 512 and 1024 sample tiles on the same deterministic U16 greyscale texture,
measuring source tile bytes, complete encoded size, metadata capacity, packet
and selected entropy bytes, selected blocks/coefficients and window workspace.
The codec owns these measurements. Exit requires a selected geometry justified
by those observations, a cross-tile regional comparison with established full
tile reconstruction, and explicit remaining limitations.

`emuella_j2k_codestream::ht_indexed` owns the experimental boundary. It reuses
existing two-level 9/7 analysis, scalar search, quantisation, HT cleanup coding,
packet writing and bounded inverse synthesis. One main header describes the
complete image. Each tile has one part with its own QCD; no independent image
codestream is written as a delivery chunk. A bounded temporary local envelope
allows existing packet admission to validate generated packets once during
preparation. The index retains metadata and no encoded payload or source pixels.

The profile is unsigned U8/U16 little-endian, one or three matching unit-sampled
components, zero origins, no MCT, two decomposition levels, 64×64 blocks, one
cleanup set/pass, one LRCP layer and default precincts. Supported tile edges are 256/512/1024; at most 65535 tiles. Every
actual tile axis, including the last row/column, must have at least four
samples. Tile origins are multiples of four; final tile axes may be odd. There is one
actual precinct per tile/component/resolution. This granularity has a delivery
cost for small windows even though entropy and synthesis select smaller blocks.
Configurable precincts, deeper pyramids and genuine quality layers remain
separate calibration questions. Resolution stages do not claim quality-layer
refinement or multiple HT sets.

The rate is a per-tile upper search target plus 128 bytes per tile. It uses the
existing bounded scalar search; it can undershoot without a tight-fill promise.
This differs deliberately from the existing public single-image rate contract,
which remains unchanged. A successful stream uses no more than the sum of
floored tile rate budgets plus 128 bytes per tile plus 19 bytes. Search can fail
when even the coarsest candidate is too large.

Tile reading and output writing are synchronous callbacks. They can wrap files,
WASM memory or application storage without HTTP or thread assumptions. Failure
may leave an incomplete sink: publication is the caller's responsibility after
success. Planning reads no source bytes and visits only intersecting tiles'
retained block metadata. Decoding reads exact selected block ranges, performs
bounded window synthesis, and returns planar requested-component bytes only
after all parts succeed. Missing data must produce a callback error. The caller
owns immutable representation identity and ensuring bytes match the index.

The index exposes main-header bytes including SOC, tile-header marker ranges
excluding SOT/SOD, and actual packet header/body ranges with tile, component,
resolution, precinct and layer identities. These are codec facts; constructing
JPIP databins and requests belongs to the protocol owner. The index currently
has no serialisation or arbitrary-file reconstruction API. It must live for the
representation's process lifetime; persistence and restart are not proved.

Output is limited to 64 MiB per request. Active synthesis workspace has the
existing explicit caller limit; the returned request planes are an additional
bounded allocation. Source tile planes, float analysis, integer quantisation,
candidate packets and admission scratch scale with tile geometry, not image
area. Metadata scales with encoded block/tile count and is limited to 256 MiB of
retained dynamic capacity during encoding. Capacity accounting excludes
allocator overhead and is not an RSS claim. No cache, asynchronous cancellation,
decoded-block reuse, browser execution or final real-image qualification is
claimed by this component increment.

Standards authority for the reused algorithms and marker signalling is recorded
in `ht-lossy-foundations.md`. The tiled SIZ/SOT and tile QCD construction follows
those existing Part 1 contracts. No external implementation source or protected
image data was consulted or copied.

The two-level profile is not sufficient for efficient whole-image overview of
a 43008×43008 image: its coarsest output is 10752×10752 samples. Five or six
levels need a separate candidate and calibration. Analysis, subband generation,
quantiser count and packet writing currently select two levels explicitly; the
shared bounded synthesis planner already accepts a level count. A deeper profile
must retain phase, border, quantisation and reduced-reference checks.

The first full-stream structural probe rejected the inherited homogeneous CAP
declaration with tile-dependent QCD. Retain the existing parser check and use
`Ccap^15=0x082a`, setting bit 11 while retaining the single-set declaration.
Authority: ISO/IEC 15444-15:2019, A.3.5/Table A.2, physical PDF page 36, and
clause 8.5; reviewed retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`. This is a signalling repair, not a
relaxation of packet admission.

## Calibration observations

Observed on 2026-09-07, AMD Ryzen 9 9950X3D, Linux 7.1.3, Rust 1.97.1,
optimised default-feature build. The project-authored `fill` function in
`ht_indexed/tests.rs` defines the input exactly: native U16 sample
`(977*x + 1393*y + 9973*c + 41*x*y) mod 65536`, with unsigned wrapping
arithmetic. All images use 2 bpp per reference pixel. The request is component
zero, full resolution, `(901,901)` with extent `32×32`. RGB rows encode all
three planes but request only component zero. Bytes below are exact; timings
are one host observation without a performance acceptance threshold.

| Image | Components | Tile edge | Peak source tile bytes | Stream bytes | Index capacity bytes | Selected packet bytes | Entropy bytes | Blocks | Coefficients | Workspace ceiling bytes | Prepare/validate ms | 20 requests ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 2048² | 1 | 256 | 131072 | 1051762 | 150608 | 16407 | 16359 | 16 | 65536 | 687524 | 691 | 53 |
| 2048² | 1 | 512 | 524288 | 1048643 | 135600 | 65248 | 16206 | 16 | 65536 | 687443 | 679 | 52 |
| 2048² | 1 | 1024 | 2097152 | 1048529 | 131856 | 262154 | 16015 | 16 | 65536 | 687441 | 704 | 51 |
| 2048² | 3 | 256 | 393216 | 1048412 | 440016 | 5461 | 5437 | 9 | 36864 | 686479 | 1409 | 28 |
| 2048² | 3 | 512 | 1572864 | 1047861 | 393296 | 21750 | 5421 | 5 | 20480 | 686494 | 1462 | 16 |
| 2048² | 3 | 1024 | 6291456 | 1047868 | 321200 | 87366 | 5767 | 5 | 20480 | 686545 | 1616 | 16 |
| 4096² | 1 | 256 | 131072 | 4205609 | 601872 | 16407 | 16359 | 16 | 65536 | 687524 | 2703 | 52 |

Retain 256 as the initial two-level default-precinct candidate: packet delivery
is about four/sixteen times smaller than 512/1024 and source tile storage has
the same reduction. Greyscale index capacity is about 11%/14% larger. RGB
shows a real opposing cost: the selected window uses nine blocks at 256 versus
five at the larger geometries. Reopen this selection for configurable precincts,
deeper decomposition, real images and all-component presentation. The 4096²
probe preserves source-tile storage and regional work while index/storage grows
with image area. No whole-image source plane is constructed by the generator.
The test sink buffers encoded bytes intentionally; a file sink can consume the
same chunks synchronously.

Preparation timing includes encoding, index creation and a separate verification
of the generated global packets against effective per-tile state. Repeated
request timing includes planning, fresh workspace allocation and in-memory
selected-body reads. It is not disk, network, warm-cache, browser or RSS evidence.
The established full-image facade keeps its 1 Mi-pixel resource ceiling; the
large stream uses this separate indexed API. Global structure and each tile's
packet grammar are checked without expanding that older facade admission.

The 8×8 RGB U8 boundary probe exposed unconditional seven-bit reverse VLC
lookahead. The shared project-authored reader now resolves a short final prefix
only when every possible remaining lookup suffix selects the same word and its
actual consumed syntax fits. MEL-suppressed zero-context quads require no VLC
lookup. No absent syntax is consumed or replaced. Regression tests reject
ambiguous/missing prefixes and verify four sample families against established
full tile reconstruction, including tiny and odd edges. Empty packets retain
precinct identities and reconstruct the level shift without entropy reads.

Focused verification passed: 58 codestream HT tests (two optional probes omitted),
the explicit seven-row geometry probe, two short-prefix HT reader tests,
Clippy with warnings denied for both affected crates/all targets, and the
codestream `wasm32-unknown-unknown` check. The canonical clean-commit repository
gate remains coordinator-owned after checkpointing; no canonical-gate pass or
browser execution is claimed here.

Reproduce geometry with:

```sh
cargo test --release -p emuella-j2k-codestream geometry_calibration -- --ignored --nocapture
```

The baseline revision above plus these implementation file SHA-256 identities
bind this provisional observation before the coordinator creates its checkpoint:

| File under `crates/` | SHA-256 |
|---|---|
| `emuella-j2k-codestream/src/ht_indexed.rs` | `3f925c01aacc97d85b1c84f24e860695aaf43e7649671bd8d7a169d8acd345e7` |
| `emuella-j2k-codestream/src/ht_indexed/tests.rs` | `091ce6286a053c9996eb8776331cb061284c510567f6ce0bf608967959081762` |
| `emuella-j2k-codestream/src/ht_lossy.rs` | `b706920676bdea37b41870a5a9e691cbc199abd4de02aa668bde94ceb61c032d` |
| `emuella-j2k-codestream/src/lib.rs` | `9a84810965cbea974ccf4c9d02efeb81ffb20ec8511ba0fd13d14fb64d0fd96b` |
| `emuella-j2k-ht/src/lib.rs` | `3d3a0e5b48fd831f4f0a9175048124ac74667c2c569c2c2cdef03f9e9e02d8b5` |
