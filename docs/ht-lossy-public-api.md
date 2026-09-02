# Bounded lossy HTJ2K public API

`Htj2kLossyEncodeOptions { bits_per_pixel: f32 }`, `encode_htj2k_lossy` and
`encode_htj2k_lossy_jph` expose the calibrated irreversible HT encoder through
`ImageView`. The existing `Htj2kEncodeOptions`, `encode_htj2k` and
`encode_htj2k_jph` retain their lossless API and byte paths. There is no public
fixed-step setting or configurable decomposition count for lossy HT.

## Input, rate and resources

Input is explicitly greyscale or RGB, unsigned `SampleFormat::U8` or
`SampleFormat::U16_LE`, planar or interleaved. Unknown colour models, signed
samples, other precisions and byte orders are excluded. Plane dimensions and
formats must match `ImageInfo`, and layout and plane count must agree with the
view. Strides are bytes and must contain one active row. The buffer must reach
`stride * (height - 1) + active_row_bytes`; row padding and trailing storage do
not affect encoding. All extent arithmetic is checked. This API can accept a
public `Plane` value whose final row omits padding; `Plane::new` continues its
existing stricter whole-stride validation.

The profile is one tile/part/layer, LRCP, default precincts, zero origins,
matching unit component grids, 64×64 HT cleanup blocks, no MCT and exactly two
irreversible 9/7 decomposition levels. The scalar family, signalling and
magnitude bounds remain those of the [foundation](ht-lossy-foundations.md) and
[historical calibration](ht-lossy-calibration.md).

Rate counts the complete raw codestream in bits per reference-grid pixel. The
exact finite positive `f32` is widened to `f64`, multiplied by the checked
pixel count, floored to whole bits, then floored to whole bytes. A successful
stream does not exceed that budget and undershoots by no more than
`max(32 bytes, ceil(budget / 500))`. Invalid, zero, unrepresentable or
unattainable budgets return errors. At most seventeen scalar candidates are
visited; the largest sampled stream within budget wins, with the first
encounter breaking equal-size ties. The search is approximate, not exhaustive.
No stream is padded to simulate rate attainment or truncated to fit a budget.
JPH adds exactly 85 bytes, excluded from the budget, and contains the raw
encoder output byte-for-byte.

Each axis is 4–8192 samples, with at most 1,048,576 reference pixels and
3,145,728 component samples. Shared codestream-owned geometry, precision and
rate validation precedes image-sized layout conversion. Planar and greyscale
views borrow source rows; interleaved RGB is converted into fallibly reserved
planes only after all extents are valid. Foundation working allocations and
the complete JPH output reserve fallibly. Generated/admitted raw streams are
bounded to 32 MiB. QCD-derived magnitude widths remain at most 30 and quantised
magnitudes remain below 131072. This is neither a fixed RSS guarantee nor a
promise to recover from every allocator failure in bounded entropy scratch.

## Native decoding

Use `DecodeMode::Components`, all components and full resolution. Ordinary
`inspect`, `decode_shape`, `decode`, `decode_htj2k_with_workspace` and
`decode_into` agree for raw output and its JPH wrapper, in planar and
interleaved output layouts. Reconstruction rounds ties to even and clips to
the original unsigned precision. Full caller decode stages reconstruction
before publication: invalid targets, truncated inputs and late entropy
failures leave all caller bytes untouched; successful output preserves padding.

The default rendered mode and additional JPH presentation remain outside this
full-image profile. One independent partial route admits only raw, unsigned
greyscale `U16_LE` encoder output, planar component zero and exactly one or two
discarded resolution levels. It uses the same envelope, complete packet walker,
checked reduced geometry, prepared reduced executor and atomic publication
path. The public reusable Part 1 workspace route does not retain HT scratch,
preserving its existing accounting and clear contract. It does not allocate a
full-resolution output plane.
JPH, RGB, U8, signed, rendered or interleaved output, other selections, regions,
tiles, layer limits, zero discard and discard above two remain unsupported.
Existing reversible full-image and separately bounded native-grid, reduced,
ROI and selected DS0 routes keep their own admission contracts. The new partial
route does not widen them or claim general conformance.

## Reduced U16 qualification

The project-authored reduced matrix exercises smooth ramp, structured
zero/maximum checkerboard, structured texture and high-entropy inputs at the
representative 1, 2 and 4 bpp targets. Every cell asserts its selected success
or the matrix's explicit `EncoderRateUnattainable` disposition; no successful
row is chosen after observing the result. Successful rows decode at both
reductions and cover even, odd and non-power-of-two dimensions. The 1024 × 1024
noise case at two discarded levels produces exactly 256 × 256 samples.

Shape discovery, component descriptors, deterministic owned output, the
reusable-workspace route and padded caller output agree. Output is planar
unsigned `U16_LE`, caller row padding is preserved, and every successful public
plane equals the internal prepared reduced executor byte-for-byte. One
workspace is reused across dimensions, rates and both reductions. Invalid
requests, truncated input and corrupt retained entropy leave caller storage
unchanged; a failed workspace decode can be followed by a successful decode.
Neighbour tests reject discard above two, regions, tiles, layer limits, JPH,
RGB, U8, signed input, default rendered mode, unsupported component selections,
interleaved output and altered geometry, decomposition, coding, quantisation,
profile or layer state. Full-resolution raw/JPH reconstruction and repeated raw
encoder bytes are checked alongside the reduced results.

## Public qualification

`ht_lossy_public_tests` exercises the matrix selected before implementation in
the calibration record. It shares the project-authored
`ht_lossy_test_support::source` generator with the historical probe; no source
fixtures or external output bytes are checked in.

| Group | Public cells | Successful | Explicit unattainable |
| --- | ---: | ---: | ---: |
| 257×193, patterns 0–3, four sample families, two layouts, 1/2/4 bpp | 96 | 96 | 0 |
| 257×193, patterns 4–8, same families/layouts/rates | 120 | 10 | 110 |
| 65×65 U16 pattern 8, grey/RGB, both layouts, 1/2/4 bpp | 12 | 2 | 10 |
| 1024×1024 and 8192×128 RGB U16 noise, both layouts, 1/2/4 bpp | 12 | 12 | 0 |
| 4×4 grey/RGB U8/U16, both layouts, 1/2/4 bpp | 24 | 0 | 24 |
| Total | 264 | 120 | 144 |

Every cell repeats both raw and JPH requests. Successful raw bytes agree
across input layouts and with the extracted JPH payload. Each successful cell
checks raw and JPH native decoding in both output layouts: 480 format/layout
owned checks, plus corresponding shape, workspace and padded caller paths.
Every success also exercises truncation without caller mutation. Separate
publicly generated RGB U8/U16 cases retain the final-component entropy failure
and workspace-reuse regression, proving late failure atomicity.

Main-cell NMSE is the exact integer sum of squared source/native differences
divided by `width * height * components * (2^bits - 1)^2`. Tests enforce the
rational ceilings by cross multiplication; decimal values are diagnostic.

| Main rate | Raw budget | Greatest native NMSE | Ceiling |
| --- | ---: | ---: | ---: |
| 1 bpp | 6200 | 0.117352306900382 | 0.125 |
| 2 bpp | 12400 | 0.057098846643195 | 0.060 |
| 4 bpp | 24800 | 0.035057619808636 | 0.040 |

Main undershoot is at most 11 bytes. Every successful rate sequence is
non-increasing in source distortion, including boundary and resource rows;
no successful cell is filtered from that assertion. Ramp succeeds at 1 bpp
in all four sample families and at 2 bpp in RGB U16, in both layouts. Other
257×193 boundary requests fail explicitly. The 65×65 grey U16 extreme succeeds
only at 1 bpp, producing 515 bytes for a 528-byte budget, in both layouts.
The maximum-resource matrix succeeds at all three rates in both layouts;
greatest undershoot is 177 bytes, within the scaled tolerance. All minimum
requests fail rather than returning a padded nominal success.

Public negative tests exercise immediate axis/pixel neighbours, checked stride
and final-row overflow/shortage, mismatched plane metadata, component count,
precision, signedness, byte order, colour model and layout, invalid rates and
the zero-byte-budget transition. Fractional rates check exact `f32` conversion
and byte flooring. Existing foundation tests retain marker, transform,
decomposition, quantisation, sampling and entropy neighbours. The ordinary
lossless and locked Part 1/HTONLY regression gates remain independent.

These finite results do not establish universal rate attainability, monotonic
rate/distortion, visual quality, performance parity or general standards
conformance. Native qualification is not independent interoperability proof.
Independent full-matrix and locked-corpus execution from final merged source
remain separate authorised delivery gates; this document makes no advance
claim about those results.

## Reproducible export for independent qualification

Focused native qualification, with no external executable or assets:

```sh
CARGO_TARGET_DIR=/outside/source/build cargo test -p emuella-j2k-core \
  --release lossy_ht -- --nocapture
CARGO_TARGET_DIR=/outside/source/build-parallel cargo test -p emuella-j2k-core \
  --release --features parallel lossy_ht -- --nocapture
```

To export the same complete matrix, first commit the intended source and record
`git rev-parse HEAD` and `git rev-parse HEAD^{tree}` alongside the run log. Create
an empty output directory outside the source checkout, then run:

```sh
EMUELLA_HT_PUBLIC_OUTPUT=/outside/source/public-matrix \
CARGO_TARGET_DIR=/outside/source/build cargo test -p emuella-j2k-core \
  --release lossy_ht_export_public_matrix -- --ignored --nocapture
```

The ignored test refuses a source-contained or non-empty output directory.
`manifest.txt` is tab-separated plain text, one row per public cell. It records
case identity, dimensions, bits, components, original input layout, pattern,
rate, success/unattainable disposition, budget, tolerance, raw/JPH sizes, exact
SSE and denominator, diagnostic NMSE and peak error, and filenames with SHA-256
identities. Successful cells emit `.source.bin`, `.j2c`, `.jph` and
`.native.bin`. Both `.bin` files are packed pixel-interleaved samples, row-major
without padding; multi-byte samples are unsigned little-endian U16. Source
files describe active samples even when the public input view was planar and
padded. Unattainable rows record the reproducible source hash but no artefact
filenames or metric values; `-` denotes fields that do not apply.

Only project-authored input and Emuella output are exported. The test does not
invoke or inspect an external implementation. An authorised independent run
can decode every emitted raw/JPH pair, compare its packed samples with the
manifest's native samples using the separately selected one-code-value
per-sample tolerance, and evaluate source distortion with the exact recorded
denominator. External binaries, payloads and diagnostics must remain outside
public source; only project-authored factual summaries may be retained here.
