# emuella-j2k

`emuella-j2k` is a pure-Rust JPEG 2000 and HTJ2K codec workspace. It provides
native Rust libraries for JP2/JPH containers, JPEG 2000 and HTJ2K codestreams,
wavelet transforms, classic Tier-1 and HT block coding, a small command-line
adapter, and an experimental Python binding.

The project is preparing for its first public release. APIs and the supported
profile boundaries may still change.

## Build and test

The repository pins Rust 1.97.1:

```sh
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

These commands are useful while editing. After committing the candidate, run
`sh scripts/check.sh` for the canonical gate: it verifies the exact clean Git
tree in a disposable export, including the focused parallel native-plane and
JP2-presentation regressions. See [contributor instructions](CONTRIBUTING.md#canonical-verification)
for prerequisites, scratch placement and the clean-checkout contract.

The ordinary test suite is self-contained. It generates its inputs
algorithmically and does not download or invoke OpenJPEG, OpenJPH, Kakadu, or
standards conformance material.

Native component decode includes independent one-through-four-plane unsigned
8-bit Part 1 inputs without MCT. The bounded zero-decomposition profile in
[`docs/native-planes.md`](docs/native-planes.md) preserves caller buffers on
failure in full-image decode, including parallel builds. Native output remains
separate from the bounded
[JP2 mapped presentation](docs/jp2-presentation.md) route, which expands U8
grey/RGB palettes, direct/palette/mixed mappings and channel-defined order into
greyscale, RGB or straight RGBA. Full shape, owned and padded caller decode
agree in both layouts; alpha preserves colour samples even when zero.

Applications should normally depend on the facade package and import its
underscore-form Rust crate name:

```sh
cargo add emuella-j2k
```

```rust
use emuella_j2k::{DecodeOptions, decode};
```

Lossless HTJ2K callers can choose raw codestream bytes with `encode_htj2k` or
a deterministic JPH container with `encode_htj2k_jph`. Both entry points use
`Htj2kEncodeOptions` and the same bounded greyscale/RGB, `U8`/`U16_LE`,
planar/interleaved, zero-or-one-decomposition profile; the JPH payload is the
unchanged raw encoder output.

Lossy callers use the additive `Htj2kLossyEncodeOptions { bits_per_pixel }`,
`encode_htj2k_lossy` and `encode_htj2k_lossy_jph`. They accept explicit greyscale
or RGB, unsigned `U8`/`U16_LE`, planar or interleaved input and exactly two
irreversible 9/7 levels with no MCT. Rate counts complete raw-codestream bits
per reference pixel, excluding the 85-byte JPH wrapper. Successful output fits
the floored byte budget within `max(32 bytes, ceil(budget / 500))`; invalid or
unattainable rates fail, without padding or truncating a stream. Each axis is
4–8192 samples and the image has at most 1,048,576 pixels. See the
[public lossy HT contract and qualification](docs/ht-lossy-public-api.md) for
resource bounds, exact error metrics, measured limits and the export recipe.

Native HTJ2K decode has a separate staged HTONLY boundary. Structural Part 15
parsing and packet-signalling validity run before support admission. A broader
`Ccap^15` permission does not by itself make a codestream unsupported when the
effective codestream still uses the implemented single-set, ROI-free,
homogeneous, reversible HT path. Actual multiple HT sets, ROI, heterogeneous
state, HTMIX and cleanup magnitude bounds above 18 remain unsupported in
that full-image path. A separate [irreversible HT foundation](docs/ht-lossy-foundations.md)
admits the selected two-level, no-MCT, unsigned grey/RGB U8/U16_LE profile
for full native component output from raw HT and JPH, including the additive
lossy encoder outputs. Use `DecodeMode::Components`, all components, full
resolution and either output layout; the default rendered mode is excluded.
Other irreversible full-image profiles remain unsupported, as does rendered
projection of the new irreversible profile. Supported full inputs continue
through the existing HT packet, entropy, reconstruction and public image path;
this is not general Part 15 or JPEG 2000 conformance.

[HTMIX is deliberately unsupported](docs/htmix-disposition.md), independently
of MULTIHT permission and the HTONLY envelopes below. Legal mixed signalling
remains inspectable; packet-dependent mixed classic/HT block interpretation
does not fall back to either homogeneous decoder. Locked HTMIX points are not
applicable to the HTONLY qualification claim, not decoded-pixel passes.

The [bounded DS0 qualification summary](docs/testing.md#bounded-ds0-result)
records all sixteen selected HTONLY points and their distinct output routes.
Those points do not imply general full-image, rendered or JPH decode support.
Structural inspection also does not prove complete packet validity: the
bounded SINGLEHT validator skips every CAP Mixed declaration, including
homogeneous-effective HT neighbours.

A separate raw native ROI window route admits one unit-sampled component with
1–16-bit signed or unsigned precision, zero origins, one reversible 5/3 level,
32/64-sample block axes, 128×128/256×256 precincts and one through eight layers.
Main POC must resolve the PCRL COD to one complete LRCP volume. One tile-zero,
component-zero Maxshift assignment of 1–15 is restored before synthesis;
effective QCD/QCC must be reversible and the ROI-extended magnitude width must
fit 30 bits. Tiles have 32/64/128-sample axes, at most 64 tiles, and one payload
part followed by at most three empty parts. TLM and informational CRG are
validated; inline SOP/EPH is supported. Explicitly select planar component zero
with a full-resolution region inside tile zero. All tiles' packets are checked,
but only tile zero is reconstructed and cropped without resampling. This
qualifies both locked P0.03 and P0.15 full-resolution window alternatives.
Full-image decode, reduced alternatives, other ROI assignments, functional
tile-header overrides, packet relocation, HTMIX and JPH/rendered output remain
outside this route.

A separate native-grid full decode route admits one unsigned 8-bit subsampled
component, one tile/part, three effective reversible 5/3 levels, one through
six LRCP layers and one effective precinct per resolution. Main COC/QCC
overrides resolve before reconstruction; component origins must be aligned to
eight native samples. Use component mode, planar output and all components or
component zero. `ImageInfo` and `decode_shape` retain reference-image dimensions;
`Image::component_info` describes the actual native plane and its origin and
sampling. No resampling is performed. JPH, rendered/interleaved output, MCT,
other transform phases, regions, reductions and layer limits are outside this
route.

The same native-grid preparation also admits three matching unsigned 8-bit
sampled components with reversible MCT across one to 64 tiles, one part per
tile. Each native tile-component origin must be aligned to eight samples;
the three-level, one-to-six-layer LRCP and single-precinct limits above still
apply. Explicitly select component zero in planar component mode to obtain
the transformed codestream component before inverse RCT. All-component output,
other selections, RGB presentation and JPH remain unsupported for this branch.
Native tile bounds are assembled without resampling. The aggregate native
component sample count is limited to 16 Mi samples before packet preparation.

Native partial component output adds bounded HTONLY reconstruction branches
behind one request shape. A raw origin-aligned single-tile codestream with five
decomposition levels and one-layer LRCP packets on the existing
single-effective-precinct inline-header route may select transformed component
0 at two discarded resolution levels. The reversible 5/3 branch requires three
matching unsigned 8-bit unit-sampled components, MCT and the existing
no-quantisation QCD contract. The irreversible 9/7 branch instead requires
exactly one unsigned 8-bit unit-sampled component, no MCT, exactly one
main-header scalar-expounded QCD and no component or tile overrides. The
planar output is reconstructed at its exact reduced geometry before inverse
colour transformation. JPH, rendered output, other selections or reductions,
regions, tile requests, quality-layer limits, heterogeneous coding or
quantisation, ROI, HTMIX and other irreversible HT shapes remain unsupported by
this route.

The lossy encoder's raw unsigned greyscale `U16_LE` output has an independent
partial component-zero route at exactly one discarded resolution level. It
retains the encoder's exact two-level irreversible envelope, validates the
complete packet stream, and reconstructs the checked half-resolution planar
geometry directly. JPH, RGB, U8, signed, rendered or interleaved output, other
component selections, regions, tiles, layer limits and every other discard
count remain unsupported in this first bounded increment.

A separate reduction-three request selects transformed component zero from a
raw zero-origin single tile/part with three matching unsigned 8-bit unit-sampled
components, MCT, six 9/7 levels, twenty RLCP layers, 64×64 HTONLY blocks and
explicit 128×128 precincts at every resolution. Main QCD and optional QCC
resolve to scalar-expounded quantisation for every component. The shared packet
walker validates all layers, components and precincts; reconstruction retains
component zero through resolution three, before inverse ICT. The reference
image is bounded to 16 Mi samples per component before packet preparation.
Only planar component-zero output without a region, tile or layer limit is
admitted. JPH, rendered output, other reductions, COC, tile-header overrides,
ROI, packed/inline packet markers and HTMIX remain outside this branch.

A heterogeneous reversible reduction-five route selects raw component zero
from one zero-origin tile/part with three unit-sampled components. Each may
have its own signedness, 8–16-bit precision, effective main COD/COC coding and
QCD/QCC no-quantisation exponents. Component zero has six decomposition levels;
the others have six through eight. CPRL uses one through thirty layers,
32/64-sample block axes and explicit 128×128 or 256×256 precincts. Inline
SOP/EPH is supported. All packets and quantisers are validated; only component
zero through resolution one is reconstructed, retaining its native precision
and signedness. The 16 Mi-sample reference-plane bound applies before packet
preparation. Only planar component-zero output without region, tile or layer
limits is admitted; MCT, sampling, tile overrides, ROI, HTMIX, JPH and rendered
output remain outside this route. This qualifies the locked P0.08 HTONLY point.

A separate scalar-derived reduction-three route selects raw component zero
from one zero-origin tile/part with four unsigned 8-bit components sampled
1×1, 1×1, 2×2 and 2×2. Effective main COD/COC resolves 6/3/6/6 levels,
9/7 for the first three components and 5/3 for the fourth, no MCT, one through
seven PCRL layers, 32×32 blocks and explicit square 128/256 precincts.
QCD/QCC resolves scalar-derived component zero, scalar-expounded components
one/two and reversible component three; every resolved exponent is positive
and its guard-adjusted magnitude width is at most 30 bits. All components and
packets are validated, but only component zero through resolution three is
reconstructed. The 16 Mi-sample reference-plane limit applies before packet
preparation. This qualifies the locked P0.05 HTONLY point with planar
component-zero output; other selections, reductions, regions, tile/layer
requests, inline packet markers, tile overrides, ROI, HTMIX, JPH and rendered
resampling remain outside this route.

JPH inspection enforces the bounded Annex D signature, `jph ` file type and
`jph ` compatibility membership,
inherited `jp2h` structure including optional-box dependencies, complete HTJ2K
`jp2c`, and first-codestream header-consistency boundary before decode
admission. The JPH unknown-colour/no-`colr` form is structurally accepted but
remains unsupported for rendered colour interpretation. Unknown legal boxes
are preserved, while optional presentation, alpha, multiple-codestream
composition, HTMIX and codec profiles outside the documented subset remain
unsupported.

A separate heterogeneous reduced ROI route qualifies locked P0.06 HTONLY.
It selects planar native component zero at reduction three, from a zero-origin
single tile/part with four independently signed or unsigned 8–16-bit components
sampled 1×1, 2×1, 1×2 and 2×2. Effective main COD/COC has six levels, 9/7
for components zero through two and 5/3 for three, 64×64 HTONLY blocks, one
through four RPCL layers, no MCT or inline SOP/EPH, and explicit square
128/256 precincts. Main QCD/QCC resolves scalar-expounded zero through two
and reversible three, with positive exponents and at most 30 ROI-extended
magnitude bits. Exactly one component-zero main RGN and one tile RGN have
shifts 1–15; the tile overrides the main and its effective shift must be 1–9.
Every component's packets are validated; only zero through resolution three
is reconstructed. ROI magnitudes are restored before dequantisation and 9/7
synthesis, preserving the native precision and signedness. The 16 Mi-sample
reference-plane preflight applies before packet preparation. Full-image decode,
other selections/reductions, region/tile/layer requests, additional tile
overrides, POC, relocation, HTMIX and JPH/rendered output remain unsupported.

A separate high-component native route qualifies locked P0.13 HTONLY. It
accepts four through 257 unit-sampled 8–16-bit components in one zero-origin
tile/part of at most 64×64 samples. Every effective main COD/COC has one
reversible 5/3 level, one RLCP layer, MCT, 32/64-sample block axes and explicit
128×128/256×256 precincts without SOP/EPH. Main POC partitions all components
into two adjacent complete volumes, first RLCP then CPRL; resolution bounds
are clipped to actual resolutions. Main QCD/QCC resolves reversible quantisers
for every component, with positive exponents and at most 30 ROI-extended
magnitude bits. One main Maxshift assignment of 1–15 must name an unselected
component. The first three MCT component formats match; later native formats
may differ. Every packet is validated, but only component zero is reconstructed
before inverse RCT. Use full `decode` or `decode_htj2k_with_workspace` with
explicit planar component zero in component mode; `decode_shape` shares the
admission. All-component support inspection remains unsupported. Other
selections, partial requests, layer limits, tile overrides, packet relocation,
HTMIX and JPH/rendered output remain outside this route.

A separate tile-progression native window route qualifies locked P0.07 HTONLY.
It admits three independently signed or unsigned 8–16-bit unit-sampled
components, zero origins, three reversible 5/3 levels, one through eight RLCP
layers, 32/64-sample block axes, explicit 128×128/256×256 precincts and optional
SOP/EPH. Tiles have 32/64/128-sample axes and the grid has at most 256 tiles.
Tile zero has two parts with successive LRCP POC volumes: a prefix of the
resolutions, then a complete volume whose already-seen packets are skipped.
All other tiles have one part and inherit RLCP. Main QCD supplies bounded
reversible quantisation; no component or tile coding/quantisation override,
ROI, MCT, main POC or packet relocation is admitted. The input is bounded to
64 MiB before packet work. Every tile/component packet is validated, using
tile-local header scopes; only component zero of tile zero is reconstructed.
Explicitly select a full-resolution planar component-zero region inside tile
zero. Native precision and signedness are preserved without resampling.
Full-image decode, other requests, HTMIX and JPH/rendered output remain outside
this route. This bounded qualification is not general Part 15 conformance.

The command-line adapter installs the `emuella-j2k` executable:

```sh
emuella-j2k inspect image.jp2
```

Its dedicated rendered conformance worker compares one bounded full-frame JP2
rendered decode with one bounded baseline RGB TIFF entirely in memory:

```sh
emuella-j2k compare-rendered-tiff-rgb image.jp2 reference.tif \
  --width 480 --height 640 --components 3 --peak-error-limit 4
```

This low-level worker is intended for the verified opt-in Layer 2 runner. It
prints aggregate fields only and does not acquire, copy or persist pixels.

## Workspace

- `emuella-j2k`: stable public facade for application users.
- `emuella-j2k-core`: implementation of the high-level inspect, decode, and
  encode API.
- `emuella-j2k-container`: JP2 and JPH container parsing.
- `emuella-j2k-codestream`: J2K and HTJ2K codestream parsing and coding.
- `emuella-j2k-tier1`: classic JPEG 2000 Tier-1 block coding.
- `emuella-j2k-ht`: HTJ2K block coding.
- `emuella-j2k-transform`: wavelet and component transforms.
- `emuella-j2k-accel`: safe architecture-acceleration boundary.
- `emuella-j2k-cli`: package for the `emuella-j2k` command-line inspection
  adapter.
- `emuella-j2k-python`: experimental PyO3 binding.
- `emuella-j2k-test-support`: deterministic public test and fixture generator.

See `docs/architecture.md` and `docs/testing.md` for the public development
boundary.

## Test data

Large, third-party, conformance, interoperability, and benchmark corpora do not
belong in this repository. They are catalogued separately by
`emuella-testdata`, with stable pack identities and per-pack licensing.
Runtime support is classified from parsed codec structure, not by matching or
replaying corpus payloads.

## Licensing

Project-authored material is licensed under Apache-2.0. Small, isolated HTJ2K
modules contain OpenJPH-derived code or table data under BSD-2-Clause. Read
`NOTICE`, `THIRD_PARTY.md`, and `LICENSES/OpenJPH-BSD-2-Clause.txt` before
redistribution.
