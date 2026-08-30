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
state, irreversible HT coding, HTMIX and cleanup magnitude bounds above 18
remain unsupported. Supported inputs continue through the existing HT packet,
entropy, reconstruction and public image path; this is not general Part 15 or
JPEG 2000 conformance.

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
