# Rust application API

Start applications with the `emuella-j2k` package and `emuella_j2k` imports.
The [getting-started example](getting-started.md) creates, inspects and decodes
an image entirely in memory. The API is preparing its first release and may
change. Choose a supported input **and request** from the
[profile guide](supported-profiles.md); inspection success alone does not
establish pixel decode support or general JPEG 2000/HTJ2K conformance.

## Choose an operation

| Consumer task | Public entry points and principal types | Contract and example |
|---|---|---|
| Examine bytes without reconstructing samples | `inspect`, `InspectOptions`, `Metadata`, `InputFormat`, `SupportStatus` | [Inspection and failure distinctions](error-handling.md), [profiles](supported-profiles.md#i-want-to-inspect-an-input) |
| Obtain owned full-image output | `decode`, `DecodeOptions`, `DecodeMode`, `Image`, `ImageData`, `ComponentInfo` | [In-memory round trip](getting-started.md), [native planes](native-planes.md), [JP2 presentation](jp2-presentation.md) |
| Discover shape and provide output storage | `decode_shape`, `DecodeShape`, `decode_into`, `ImageInfo`, `ImageViewMut`, `PlaneMut` | [Sizing and padded caller output](caller-owned-output.md) |
| Select native components, a region, tile or reduction | `decode_partial`, `decode_partial_info`, `decode_partial_component_info`, `decode_partial_into`, `PartialDecodeOptions` | [Exact decoder request profiles](decoding-profiles.md); discovery must use the same request as execution |
| Request bounded rendered regional output | `decode_rendered_partial`, `decode_rendered_partial_info`, `decode_rendered_partial_into` | [sYCC presentation boundary](architecture.md#bounded-full-frame-and-partial-sycc-projection); this is separate from native partial output |
| Encode lossless or target-rate Part 1 | `encode`, `encode_into`, `ImageView`, `Plane`, `EncodeOptions`, `EncodeQuality`, `OutputFormat` | [Getting started](getting-started.md), [precision](part1-precision.md), [rate control](encoder-rate-control-calibration.md) |
| Budget the selected raw D2 lossless encoder | `lossless_encode_requirements`, `encode_with_limits`, `LosslessEncodeLimits`, `LosslessEncodeRequirements` | [Scalable lossless resource contract](scalable-lossless.md), [eight positional bands](native-eight-components.md) |
| Encode lossless HTJ2K raw or JPH | `encode_htj2k`, `encode_htj2k_jph`, `Htj2kEncodeOptions` | [Zero/one-level reversible HT profile](architecture.md#bounded-reversible-htj2k-encode-and-jph-output) |
| Encode target-rate HTJ2K raw or JPH | `encode_htj2k_lossy`, `encode_htj2k_lossy_jph`, `Htj2kLossyEncodeOptions` | [Two-level irreversible HT profile](ht-lossy-public-api.md); invalid and unattainable rates fail |

Native `DecodeMode::Components` preserves codestream component meaning.
Rendered mode applies only admitted colour/palette/channel/alpha projections.
A component may have a different grid, precision or signedness from the
reference image; use `Image::component_info` and partial component discovery
for those routes. Rendered channel descriptors have no source component index.
See [native component grids](decoding-profiles.md#full-native-component-grids).

## Full decode and inspection options

`DecodeOptions::default()` requests rendered output, all components, all
quality layers, planar layout and no best-effort attempt.

| Field | Meaning and combinations |
|---|---|
| `mode` | `Rendered` requests presentation; `Components` requests native samples. Supported profiles differ. |
| `requested_components` | `All`, or `Indices(Vec<u16>)` of source component indices. Explicit selection requires component mode; indices must be non-empty, unique and in range. Output follows the requested order. |
| `max_quality_layers` | `None` reconstructs all layers. `Some(0)` is invalid. Positive limits require component mode and the admitted profile; genuine truncation is bounded to the raw full-image planar single-component two-layer LRCP route. |
| `target_layout` | `Planar` returns one byte vector per component; `Interleaved` returns one pixel-interleaved byte vector. Native grids that differ cannot be assumed interleavable. Caller-output execution derives this choice from its target variant. |
| `allow_best_effort_backend_decode` | Default `false`. Legacy compatibility attempt for selected native Part 1 inputs classified unsupported; it may still fail. It enables no external codec and does not admit JPH, raw HTJ2K or unknown formats. |

`InspectOptions` has two independent booleans, both defaulting to `true`.
`preserve_raw_metadata` retains selected raw metadata records; turning it off
does not skip structural parsing. `classify_support` requests the implemented
decode-support classification; disabling it yields `SupportStatus::Unknown`.
`Supported`, `Unsupported { feature, detail }` and `Unknown { detail }` describe
inspection's classification, not a promise for every later option combination.
`permits_decode()` is true only for `Supported`.

`Metadata` reports detected `format`, optional image/codestream/container
descriptions, `support` and preserved `records`. `InputFormat` distinguishes
`Jp2`, `J2kCodestream`, `Jph`, `Htj2kCodestream` and `Unknown`. Optional fields
must be handled as optional rather than invented when absent. Image metadata
is distinct from the resolved output of `decode_shape`.

## Partial requests

`PartialDecodeOptions::default()` selects no region or tile, full resolution,
all components and layers, and planar output. It describes native component
output; rendered partial output uses its explicitly named functions above.

| Field or type | Meaning |
|---|---|
| `region: Option<Region>` | Image-relative, full-resolution reference-grid origin `x`, `y` plus positive `width`, `height`; a contained half-open rectangle. Mutually exclusive with `tile`. |
| `tile: Option<TileSelection>` | Zero-based SIZ grid `tile_x`, `tile_y`, resolved to a clipped image-relative tile rectangle. Mutually exclusive with `region`. |
| `resolution` | `Full`, or `Reduced { discard_levels }`; permitted levels and endpoint projection depend on the route. |
| `components` | `ComponentSelection::All` or explicit source indices; profile-specific selections remain enforced. |
| `max_quality_layers` | `None` or a positive leading-layer limit in admitted profiles. Region/reduction requests do not inherit the genuine two-layer truncation profile. |
| `target_layout` | `Planar` by default; interleaved admission depends on the profile. |

Use [decoder profiles](decoding-profiles.md#native-reduced-and-regional-requests)
for coordinate projection and supported neighbours, and
[positioned-source decoding](part1-source-index.md) for preparation, reuse,
source lifetime, execution limits and workspace contracts. A full-image shape
does not size a partial request, and `ImageInfo` alone does not size unequal
native planes. These advanced paths require their own discovery APIs.

## Encoding choices

`EncodeOptions::default()` chooses JP2, LRCP, reversible 5/3, lossless coding,
zero decomposition levels, one untiled image (`tile_size: None`) and no added
metadata. `format` is `OutputFormat::Jp2` or `J2kCodestream`.
`progression_order` names `Lrcp`, `Rlcp`, `Rpcl`, `Pcrl` or `Cprl`; exposed
values do not imply every encoder accepts them. `transform` names
`Reversible53`, `Irreversible97` or `Unknown`. `quality` is `Lossless` or
`TargetRate { bits_per_pixel }`. Rate measures complete raw-codestream bits per
reference-grid pixel, excluding container overhead. The Part 1 target-rate
profile requires LRCP, irreversible 9/7 and two levels; invalid, unsupported
or unattainable combinations return errors.

`decomposition_levels` and optional `TileSize { width, height }` are bounded by
the selected encoder. `metadata` contains `MetadataRecord { kind, label,
bytes }`; `MetadataKind` includes XML, UUID, unknown boxes and unknown markers,
but baseline encoding does not admit arbitrary supplied records. Consult the
[profile guide](supported-profiles.md#i-want-to-encode-an-image) and linked
contracts before changing these fields. `encode_into` appends to a caller-owned
`Vec<u8>`; it is separate from pixel `decode_into` and is not a fixed-capacity
output-slice API.

The separate HT functions choose raw/JPH output by function name.
`Htj2kEncodeOptions::default()` selects zero reversible levels; only zero or one
is admitted. `Htj2kLossyEncodeOptions { bits_per_pixel }` selects the separate
two-level, no-MCT irreversible profile. Its checked budget and finite search
can return an unattainable-rate result. Neither HT type is a way to request
arbitrary Part 1 `EncodeOptions` combinations.

## Features, layers and local API reference

The facade defaults to `std`; these application examples use that configuration
on the host target. `parallel` and `simd` also enable `std` and retain serial
or scalar fallbacks. `--no-default-features` is a library compilation contract,
not a claim that every documented codec route works without `std`: the selected
algorithmic encode/decode and resource APIs have narrower availability.
No browser or cross-target execution is established by these host examples.

Generate the local reference from the source checkout:

```sh
cargo doc --locked --lib --no-deps -p emuella-j2k -p emuella-j2k-core -p emuella-j2k-codestream -p emuella-j2k-container
```

Open `target/doc/emuella_j2k/index.html` (or the equivalent beneath
`CARGO_TARGET_DIR`). Its application links lead to re-exported defining-core
contracts. The facade deliberately also exposes `codestream` and `container`
modules, prepared/source-index/workspace APIs and diagnostic hooks. They serve
lower-layer or specialised integration tasks; ordinary applications can begin
with the operation table. Their exports remain available, and their presence
does not broaden supported profiles. [Architecture](architecture.md) describes
crate ownership. This local generation route makes no hosted-docs availability
claim.
