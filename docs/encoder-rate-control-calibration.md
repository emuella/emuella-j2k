# Encoder rate-control qualification

This project-authored record qualifies the first bounded public JPEG 2000
Part 1 irreversible rate-control profile. It does not qualify HTJ2K
decomposition, JPH writing, multiple tiles, multiple quality layers, arbitrary
component geometry, or release publication.

## Revisions, inputs, and authority

- Codec base revision:
  `ebf3d7011df968b6d4f897f5e2944862682783f1`.
- Review-blocked candidate revision:
  `a7dc10c9194769c83101e9f0ae3278225e2ba9e0`. Its scalar-step search could
  select a quantisation declaration wider than the preserved classic
  component decoder's 30-magnitude-plane representation.
- Candidate identity: the exact reviewed pull-request head recorded in the
  owning pull request and private campaign plan. The tracked record cannot
  contain its own future commit identity without becoming self-referential.
- Probe: `crates/emuella-j2k-test-support/tests/encoder_calibration.rs`.
- Inputs: four in-memory 257 by 193 patterns, greyscale and RGB at unsigned
  8-bit and little-endian unsigned 16-bit precision. The test generates each
  code value from its x coordinate, y coordinate, component index, products,
  an xor term, and two bounded block terms. No fixture, corpus, external codec,
  or third-party output is used.
- Boundary input: one independently project-authored 65 by 65 greyscale
  U16_LE pattern. For each coordinate, indices 1, 2, 4, and 5 are classified
  negative and every other index positive; a pixel is `u16::MAX` when its x
  and y classifications match and zero otherwise. It is generated in memory
  with a 130-byte interleaved stride.
- Normative route: ISO/IEC 15444-1:2024, Annex A, A.6.1–A.6.5; Annex B, B.5
  and B.8; Annex E, E.1; and Annex F, F.2.2. Annex J, J.13 is informative.
  Retrieval revision:
  `34e5d1639b9f121807e620c001893ca9d2c8f977`; reviewed bundle:
  `1a7a03799078b476bf38e91786b979059b4c533d`.

The implementation, test inputs, calculations, constants, control flow, and
this record are independently project-authored. This record identifies routes
without quoting or reproducing ISO content.

## Public contract and supported profile

The existing `EncodeQuality::TargetRate { bits_per_pixel: f32 }` API is
retained. The value is a finite positive target for complete raw-codestream
bits per reference-grid pixel. It excludes JP2 box overhead and is not bits per
component sample. The implementation converts the exact finite `f32` to
`f64`, multiplies by the checked reference-grid pixel count, floors to whole
bits, and divides by eight with floor. It never increases the requested rate.

The supported lossy profile is one tile, LRCP, one quality layer, irreversible
9/7, scalar-expounded quantisation, exactly two decomposition levels, and
native greyscale or RGB unsigned U8 or U16_LE input. Both planar and
interleaved layouts are accepted. RGB uses the irreversible component
transform. Raw J2K and ordinary JP2 wrapping are supported; metadata remains
subject to the existing container boundary. The existing reversible 5/3
`Lossless` paths are unchanged.

The encoder performs deterministic floating-point 9/7 analysis once. It then
searches a finite ordered set of representable scalar steps. Each candidate is
quantised and fully classic Tier-1 coded with one packet per LRCP
layer-resolution-component position. The largest sampled complete codestream
that does not exceed the internal byte budget is retained. No payload is
padded and no codestream is truncated after packet construction.

Before coding a candidate, the encoder resolves every scalar step to the same
guard-bit-plus-exponent magnitude-plane width consumed by ordinary component
decode. A declaration wider than the preserved 30-plane classic coefficient
representation is ineligible, and the ordered search continues towards
coarser quantisation. If the remaining decoder-safe candidates cannot meet the
qualified undershoot tolerance, encoding fails explicitly instead of emitting
a codestream that its ordinary public component path cannot decode.

Invalid non-finite or non-positive rates, a zero-byte conversion, an
irreducible budget, a budget not attainable within the qualified tolerance,
another decomposition count, a tile request, a transform/quality mismatch,
unsupported image metadata, and checked size overflow fail explicitly.

## Empirical result

The probe encoded every cell twice and required byte identity. It decoded raw
J2K and its JP2 wrapper to native interleaved component samples. NMSE is the
integer squared-error sum divided by the integer sample count and
`(2^precision - 1)^2`. Assertions compare this rational quantity by cross
multiplication; the decimals below are diagnostics. Peak error is diagnostic
in the source code-value domain.

| Input | Target bpp | Budget | Raw bytes | Undershoot | NMSE | Peak error |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Grey U8 | 1 | 6,200 | 6,200 | 0 | 0.042136203536 | 188 |
| Grey U8 | 2 | 12,400 | 12,397 | 3 | 0.018372574234 | 147 |
| Grey U8 | 4 | 24,800 | 24,800 | 0 | 0.002226001245 | 49 |
| RGB U8 | 1 | 6,200 | 6,193 | 7 | 0.056045766297 | 255 |
| RGB U8 | 2 | 12,400 | 12,400 | 0 | 0.038679271583 | 254 |
| RGB U8 | 4 | 24,800 | 24,800 | 0 | 0.018230576812 | 207 |
| Grey U16_LE | 1 | 6,200 | 6,195 | 5 | 0.031251786324 | 44,538 |
| Grey U16_LE | 2 | 12,400 | 12,389 | 11 | 0.007524874749 | 25,949 |
| Grey U16_LE | 4 | 24,800 | 24,799 | 1 | 0.000405862895 | 5,789 |
| RGB U16_LE | 1 | 6,200 | 6,200 | 0 | 0.063964035811 | 65,435 |
| RGB U16_LE | 2 | 12,400 | 12,381 | 19 | 0.046477327217 | 65,376 |
| RGB U16_LE | 4 | 24,800 | 24,797 | 3 | 0.022832137801 | 53,350 |

For every input, exact-rational NMSE decreases strictly from 1 to 2 to 4 bpp.
The observed maximum undershoot is 19 bytes. The retained limit is therefore
`max(32 bytes, ceil(0.2% of budget))`, replacing the provisional 5% limit.
The retained rate-wise NMSE ceilings are 0.070, 0.050, and 0.025. They round
the observed worst cases upwards while remaining materially separated across
the three rates; the qualification test enforces them exactly as
70,000/1,000,000, 50,000/1,000,000, and 25,000/1,000,000.

Every JP2 output was the corresponding raw codestream plus 85 deterministic
bytes. Rate compliance is measured only on the raw codestream. A separate
planar-input matrix at 2 bpp covers both accepted layouts. The original 64 by
48 lossless matrix remains byte-repeatable, decodes exactly, and has zero
source-domain distortion.

The 65 by 65 U16_LE boundary probe explores the review transition and requires
every successful raw and JP2 encode to decode through ordinary public
component mode. The repaired results were:

| Target bpp | Budget | Raw bytes | Undershoot | JP2 bytes | Outcome |
| ---: | ---: | ---: | ---: | ---: | --- |
| 1.58 | 834 | 833 | 1 | 918 | decoded |
| 1.59 | 839 | 835 | 4 | 920 | decoded |
| 1.60 | 845 | 835 | 10 | 920 | decoded |
| 1.65 | 871 | 857 | 14 | 942 | decoded |
| 1.70 | 897 | 896 | 1 | 981 | decoded |
| 1.71 | 903 | 898 | 5 | 983 | decoded |
| 8.00 | 4,225 | - | - | - | rejected as unattainable |

The 1.58 bpp lower boundary remains attainable at the same 833 raw bytes. At
1.59 through 1.71 bpp, the repaired search selects close decoder-safe
candidates rather than the finer ineligible declaration. The 8 bpp probe
proves fail-closed behaviour when no safe candidate is attainable within the
retained tolerance. Every successful wrapper still adds exactly 85 bytes.

The focused evidence command was:

```sh
task_build_root=/path/to/assigned-build-root
TMPDIR="$task_build_root/tmp" \
CARGO_TARGET_DIR="$task_build_root" \
  cargo test -p emuella-j2k-test-support --test encoder_calibration -- --nocapture
```

It passed all five tests, including the 12 rate cells, the extreme U16_LE
decoder boundary, lossless baselines, planar inputs, invalid rates, infeasible
rates, and transform/profile mismatch cases. For current canonical repository
verification, commit the candidate and run from a clean checkout:

```sh
EMUELLA_CHECK_TMPDIR=/path/to/assigned-check-scratch sh scripts/check.sh
```

The parent must already exist outside the checkout. The entry point creates an
exact committed-source export and separate disposable build output; no
hosted-CI environment flag is needed. See the
[canonical verification contract](../CONTRIBUTING.md#canonical-verification).
The historical calibration check predates this source-export entry point and
is not evidence for the new wrapper. That check passed public-tree and legal
policy, formatting, all workspace targets and tests, parallel codestream
checking, strict clippy for workspace and fuzz targets, dependency policy,
no-default-features checks, and the locked fuzz workspace check.

## Retain/reject decisions and residual risk

Retain the existing raw bits-per-reference-grid-pixel API, two decomposition
levels, scalar-step search, the 0.2%/32-byte tolerance, and the empirical NMSE
ceilings. Retain the decoder-equivalent 30-plane candidate eligibility bound.
Reject padding, silent rate excess, automatic profile widening, multiple
tiles, multiple layers, reversible target-rate coding, lossy `Lossless`, and
zero- or one-level irreversible output for this increment.

The qualification is deliberately synthetic and bounded. It does not prove
visual quality, interoperability with an external encoder, optimal
rate-distortion allocation across components or subbands, behaviour for every
image dimension and target, or performance suitability. A future increment
may introduce coding-pass rate-distortion truncation, but it must preserve the
public unit and non-exceeding complete-codestream contract and must be
recalibrated independently. HTJ2K decomposition and JPH writing remain
separate unimplemented work packages.
