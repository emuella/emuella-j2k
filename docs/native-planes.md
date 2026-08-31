# Independent unsigned 8-bit native planes

The existing owned Part 1 decoder reconstructs independent non-MCT planes
through its general native component route, including two and four components.
The narrower greyscale/RGB baseline classifier is not the complete native
support boundary. No encoder or codestream-admission expansion is needed for
these planes.

`DecodeMode::Components` returns native samples from raw J2K or a structurally
valid JP2. JP2 palette, component mapping and channel roles do not modify these
values. Existing component metadata conventions remain: all-component one- and
three-plane output is labelled greyscale and RGB, while two/four planes and
explicit selections have an unknown colour model. These labels do not apply a
colour transform or establish presentation roles. Component descriptors retain
the selected source indices, precision and native grid.

## Atomic full-image publication contract

`decode_into` and `decode_into_with_workspace` reconstruct privately before
publishing any samples for the following bounded profile. Both planar and
interleaved targets preserve padding. A failure in a later plane leaves every
caller byte unchanged, including with the `parallel` feature enabled.

| Field | Bound |
|---|---|
| Codestream | Classic Part 1, one tile and one tile-part; first part, declared count one or unspecified |
| Components | One through four independent unsigned 8-bit planes, unit horizontal/vertical sampling |
| Geometry | Zero image/tile origins, positive dimensions at most 32,768 per axis, at most 16 Mi total native samples across all components |
| Tile | Covers the image; nominal tile size may exceed it |
| Coding | LRCP, one layer, zero decompositions, reversible transform identifier, no MCT |
| Blocks | Power-of-two nominal axes from 4 through 64 samples, style zero; at most 1,048,576 blocks across all planes, including smaller edge blocks |
| Quantisation | One main QCD, no quantisation, LL exponent eight, one or two guard bits |
| Markers | SIZ, main COD/QCD, SOT, SOD, EOC and optional COM only (SOC begins the stream) |
| Requests | Existing supported full native component selections and layouts |

These are engineering bounds for this publication contract, not general
Part 1 limits. The crate-private `native_planes::is_atomic_profile` predicate
checks them before packet work and can be reused by JP2 presentation planning.
A separate checked block-count bound applies before packet topology, including
partial edge blocks. The axis bound keeps each plane in one default
precinct, giving at most four packets. Reconstruction uses existing bounded
packet and coefficient machinery; no new entropy coder or transform is added.

Component/tile coding or quantisation overrides, ROI, POC, CRG, packet-header
relocation, packet length tables, SOP/EPH, explicit precincts and other block
styles are outside this atomic contract. Existing supported routes outside it
retain their prior behaviour. The prepared and partial APIs retain their
existing publication contracts; this change does not promise transactional
prepared execution. The reusable workspace is unused when full decode takes
the owned staging adapter.

The fix addresses an observed parallel-route failure: earlier component jobs
could publish their samples before a later component failed entropy decoding.
The scalar one-tile route already staged a tile. Choosing owned reconstruction
for this bounded profile gives the full-image APIs the same publication
behaviour regardless of the feature configuration.

## Independent fixtures and verification

`emuella-j2k-test-support::native_planes` supplies reusable authored builders.
The literal empty-packet family invokes neither an image encoder nor an entropy
encoder and must reconstruct sample value 128. The varied 5×3 family builds its
own SIZ/COD/QCD/SOT and packet headers, calls only the project-owned Tier-1
utility, and compares against explicit native values including 0, 1, 127, 128
and 255. Neither expected native values nor future expected presentation values
are derived from a decoder. The helper accepts 1–4 planes with one code-block
per plane, dimensions 1–64 per axis. Optional JP2 boxes are authored by callers.

The test matrix covers raw and valid JP2 inputs, all four counts, reversed
source selections, both output layouts, source descriptors, shape agreement,
padded caller buffers and both workspace adapters. A separate 65×67 test uses
the existing project fixture encoder to cover multiple blocks and odd edges
against independently specified samples. It supplements the hand-built family.
A late invalid-MQ fixture passes packet preparation and fails reconstruction in
the final plane; sentinel buffers prove atomicity. Malformed lengths and QCD
and an unsupported CRG neighbour also fail without mutation. The existing
Tier-1 error adapter reports an entropy-coder `Unsupported` error for the invalid
MQ payload; this work does not reclassify its diagnostics.

Run both feature configurations; ordinary workspace tests alone do not exercise
the parallel caller route:

```sh
cargo test -p emuella-j2k-core native_planes
cargo test -p emuella-j2k-test-support --test native_planes
cargo test -p emuella-j2k-test-support --features emuella-j2k-core/parallel --test native_planes
```

The fixtures and implementation are project-authored. They contain no external
implementation source, protected payload, or reproduced standards expression.
Their basis is published normative-core ISO/IEC 15444-1:2024 / ITU-T T.800 (V4),
A.6.1 and Tables A.13–A.21 (physical PDF pages 47–49), A.6.4 and Tables
A.28–A.29 (pages 52–53), B.10.1–B.10.7.1 (pages 91–93), and G.1.2/G.2
(pages 153–154, including the transform placement in Figures G.1/G.2), retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`. Those clauses distinguish independent
native reconstruction from a signalled multiple-component transform. The
geometry, resource and atomic-publication restrictions above are project policy.
