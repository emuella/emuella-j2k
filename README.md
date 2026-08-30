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

The ordinary test suite is self-contained. It generates its inputs
algorithmically and does not download or invoke OpenJPEG, OpenJPH, Kakadu, or
standards conformance material.

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

Native HTJ2K decode has a separate staged HTONLY boundary. Structural Part 15
parsing and packet-signalling validity run before support admission. A broader
`Ccap^15` permission does not by itself make a codestream unsupported when the
effective codestream still uses the implemented single-set, ROI-free,
homogeneous, reversible HT path. Actual multiple HT sets, ROI, heterogeneous
state, irreversible HT coding in the full-image path, HTMIX and cleanup
magnitude bounds above 18 remain unsupported. Supported full inputs continue
through the existing HT packet, entropy, reconstruction and public image path;
this is not general Part 15 or JPEG 2000 conformance.

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

Native partial component output adds two bounded HTONLY reconstruction branches
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

JPH inspection enforces the bounded Annex D signature, `jph ` file type and
`jph ` compatibility membership,
inherited `jp2h` structure including optional-box dependencies, complete HTJ2K
`jp2c`, and first-codestream header-consistency boundary before decode
admission. The JPH unknown-colour/no-`colr` form is structurally accepted but
remains unsupported for rendered colour interpretation. Unknown legal boxes
are preserved, while optional presentation, alpha, multiple-codestream
composition, HTMIX and codec profiles outside the documented subset remain
unsupported.

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
