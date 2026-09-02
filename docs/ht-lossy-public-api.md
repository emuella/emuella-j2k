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
greyscale `U16_LE` encoder output and planar component zero. Without a region,
exactly one or two resolution levels must be discarded. With a region, full,
discard-one and discard-two output are admitted.

`PartialDecodeOptions::region` is one non-empty, contained, image-relative
half-open rectangle on the full-resolution reference grid. For a discard count
`d`, its left, top, right and bottom endpoints are independently projected as
`ceil(endpoint / 2^d)`. A non-empty full-resolution request whose projected
endpoints coincide is rejected rather than published as an empty image.
`decode_partial_info` reports the projected width and height;
`decode_partial_component_info` additionally reports the projected origin.
A full-image region agrees byte-for-byte and geometrically with the established
full-image or reduced route.

Preparation uses the exact encoder envelope and complete packet walker before
retaining selected work. Only whole code blocks intersecting the required
coefficient windows reach HT entropy decode. Compact LL/HL/LH/HH windows and
transform-owned bounded 9/7 synthesis reconstruct conservative internal
support before returning exactly the requested rectangle; callers see no halo.
Small requests allocate neither complete coefficient/synthesis planes nor a
full-tile or full-resolution output plane. This is not full decode followed by
crop or resampling.

Owned output, shape, component descriptors, `Part1DecodeWorkspace` reuse and
padded `decode_partial_into` agree byte-exactly. The reusable workspace retains
private segment, coefficient-window and synthesis storage;
`retained_heap_bytes` includes those capacities. Per-block HT entropy scratch
is local to one call and dropped afterwards. The
`set_lossy_ht_spatial_region_memory_limit` value bounds deterministic active
work for each region, including a conservative checked ceiling for that local
entropy scratch. Retained capacity may exceed a later request's active need and
is not treated as current use. Target geometry, format, stride, final extent
and padding are validated before execution. Complete selected entropy decode,
synthesis, finite ties-to-even U16_LE conversion, output allocation and the
active-use limit all succeed in staging storage before any caller row is
copied. Every failure therefore preserves active bytes, row padding and
trailing storage; success preserves padding.

JPH and other wrappers, RGB, U8, signed, rendered or interleaved output, other
selections, tiles, layer limits, discard above two and any altered coding,
quantisation, decomposition, origin, sampling or packet profile remain
unsupported. Existing reversible full-image and separately bounded native-grid,
reduced, ROI and selected DS0 routes keep their own admission contracts. This
finite profile does not claim general JPEG 2000 or Part 15 conformance.

The spatial planning basis is ISO/IEC 15444-1:2024 retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`: B.1–B.3 and B.5–B.9 for image,
resolution, subband and code-block geometry; E.1–E.1.1.2 for packet
completeness; F.1–F.3.8.2.1 for recursive inverse 9/7 synthesis and boundary
extension; and G.1–G.1.2 for the existing quantisation handling. HT cleanup
interpretation remains based on ISO/IEC 15444-15:2019 retrieval
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`, clauses 6.1–6.2 and 7.6,
Annex A.1–A.3 and B.1–B.3. These references inform independently authored
code and generated tests; no protected standards expression or payload is
reproduced.

## Partial U16 qualification

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
plane equals an exact crop of Emuella's established full or direct reduced
decode. One workspace is reused across dimensions, rates, all three admitted
resolutions and growing, shrinking and relocated regions. The matrix covers
interior, corners, every edge, one-pixel and narrow strips, odd geometry,
64-sample alignment and crossings, transform boundaries, full-image requests,
representative smooth/structured/high-entropy encoder successes, a 1024 × 1024
image with a 256 × 256 full-grid request, and a 1024 × 1024 full request
projecting to 256 × 256 at discard two. Invalid requests, malformed packets,
truncated input, corrupt selected entropy and one-byte-short active workspace
limits leave caller storage unchanged; a failed workspace decode can be
followed by a successful decode.
Neighbour tests reject empty, out-of-bounds, endpoint-overflow and empty-
projection regions, discard above two, simultaneous region and tile, tiles,
layer limits, JPH, RGB, U8, signed input, default rendered mode, unsupported
component selections, interleaved output and altered geometry, decomposition,
coding, quantisation, profile or layer state. Full-resolution raw/JPH
reconstruction and repeated raw encoder bytes are checked alongside the
partial results.

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
