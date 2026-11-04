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
quality layers, planar layout, no best-effort attempt and the built-in decode
limits. Each field in the
tables below has a chainable `with_<field>` setter on its options type; see
[compatibility](#compatibility-and-specialised-api).

| Field | Meaning and combinations |
|---|---|
| `mode` | `Rendered` requests presentation; `Components` requests native samples. Supported profiles differ. |
| `requested_components` | `All`, or `Indices(Vec<u16>)` of source component indices. Explicit selection requires component mode; indices must be non-empty, unique and in range. Output follows the requested order. |
| `max_quality_layers` | `None` reconstructs all layers. `Some(0)` is invalid. Positive limits require component mode and the admitted profile; genuine truncation is bounded to the raw full-image planar single-component two-layer LRCP route. |
| `target_layout` | `Planar` returns one byte vector per component; `Interleaved` returns one pixel-interleaved byte vector. Native grids that differ cannot be assumed interleavable. Caller-output execution derives this choice from its target variant. |
| `allow_best_effort_backend_decode` | Default `false`. Legacy compatibility attempt for selected native Part 1 inputs classified unsupported; it may still fail. It enables no external codec and does not admit JPH, raw HTJ2K or unknown formats. |
| `limits` | `DecodeLimits` budgets; see [caller decode limits](#caller-decode-limits). `decode_shape` and caller-output execution apply them too. |

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
| `limits` | `DecodeLimits` budgets for the requested window, applied by every partial, rendered-partial, discovery and prepared entry point. |

Use [decoder profiles](decoding-profiles.md#native-reduced-and-regional-requests)
for coordinate projection and supported neighbours, and
[positioned-source decoding](part1-source-index.md) for preparation, reuse,
source lifetime, execution limits and workspace contracts. A full-image shape
does not size a partial request, and `ImageInfo` alone does not size unequal
native planes. These advanced paths require their own discovery APIs.

## Caller decode limits

Decoding untrusted input allocates output and working storage from declared
header geometry. `DecodeLimits` lets a caller lower that bound below the
built-in caps. `max_output_samples` is the total number of output samples a
request produces: each selected native component's width × height after
region, tile and resolution reduction, summed. Rendered requests count each
rendered channel on the reference grid, and never fewer channels than the
codestream has components. The default,
`DecodeLimits::BUILT_IN_MAX_OUTPUT_SAMPLES` (256 Mi samples), keeps the
built-in behaviour. A larger value is accepted, but the built-in caps,
including narrower per-component caps on some routes, still apply. Zero is
`InvalidParameter`. Build a budget with
`DecodeLimits::default().with_max_output_samples(n)` and pass it with
`DecodeOptions::with_limits` or `PartialDecodeOptions::with_limits`.

The count comes from inspected headers and the request before any packet
preparation, decode work or output allocation, on every entry point that takes
`DecodeOptions` or `PartialDecodeOptions`, including discovery and the `*_into`
variants. A request over budget fails with `J2kError::LimitExceeded { limit,
requested, maximum }`; caller-owned output is untouched. A partial request
served by reconstructing the full image and cropping it also checks that full
decode, so its budget must cover the full image. `InspectOptions` has no
limits because inspection produces no samples. Positioned-source preparation
takes a `Part1ComponentDecodeRequest` rather than these options and keeps its
existing `Part1SourceIndexLimits` and built-in bounds.

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
is admitted. `Htj2kLossyEncodeOptions::new(bits_per_pixel)` selects the separate
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
contracts. [Architecture](architecture.md) describes crate ownership. This
local generation route makes no hosted-docs availability claim.

## Compatibility and specialised API

The facade re-exports an explicit list; nothing reaches it by glob. Besides the
operation table, it exports these specialised groups. None of them broadens
supported profiles, and ordinary applications do not need them.

| Group | Exports |
|---|---|
| Caller decode budgets | `DecodeLimits`, carried by `DecodeOptions::limits` and `PartialDecodeOptions::limits` |
| Resource-bounded lossless encode (`std`) | `lossless_encode_requirements`, `lossless_bypass_encode_requirements`, `encode_with_limits`, `encode_lossless_bypass_with_limits`, `LosslessEncodeLimits`, `LosslessEncodeRequirements` |
| Reusable decode workspaces | `Part1DecodeWorkspace`, `decode_into_with_workspace`, `decode_partial_into_with_workspace`; with `std`, `Htj2kDecodeWorkspace` and `decode_htj2k_with_workspace` |
| Prepared Part 1 requests and positioned sources | `prepare_part1_decode`, `prepare_part1_decode_from_source`, `PreparedPart1Decode`, `execute_prepared_part1_decode`, `Part1ExecutionOptions`, `Part1DecodeWork`, `Part1ComponentDecodeRequest`, `TileRegionRequest`, `inspect_part1_source`, `Part1SourceInspection`, `Part1SourceIndex`, `Part1SourceIndexLimits`, `CodestreamSource`, `SliceSource`, `SourceError`, `SourceErrorKind`, `SourceMetrics` |

The facade does not export the `codestream` or `container` modules, the
entropy-coding and transform crates or their types, profiling diagnostics,
stage timings, execution-tuning policies or test fixtures. Prepared execution
uses the default policy and reports only `Part1DecodeWork` counters.
`emuella-j2k-core`, `emuella-j2k-codestream` and the other lower-layer crates
are implementation crates whose APIs follow the facade's needs and can change
in any release. Depend on them directly only for lower-layer work, pinned to
an exact version.

Types expected to grow are `#[non_exhaustive]`:

- enums `J2kError`, `UnsupportedFeature`, `SupportStatus`, `InputFormat`,
  `CodestreamKind`, `OutputFormat`, `ColorModel`, `WaveletTransform`,
  `EntropyCoder`, `MetadataKind`, `EncodeQuality` and `SourceErrorKind`: give
  an exhaustive `match` a wildcard arm;
- options and limits `DecodeOptions`, `InspectOptions`, `PartialDecodeOptions`,
  `DecodeLimits`, `EncodeOptions`, `Htj2kEncodeOptions`, `Htj2kLossyEncodeOptions`,
  `Part1ExecutionOptions`, `LosslessEncodeLimits` and `Part1SourceIndexLimits`:
  start from `Default::default()` or `Htj2kLossyEncodeOptions::new`, then use
  the `with_<field>` setters or assign the public fields;
- `SourceError`: source implementations construct it with `SourceError::new`;
- returned records `Metadata`, `CodestreamInfo`, `ContainerInfo`,
  `DecodeShape`, `ComponentInfo`, `Part1SourceInspection`, `Part1DecodeWork`,
  `LosslessEncodeRequirements` and `SourceMetrics`: read their fields; callers
  do not construct them, except that a source implementation may start
  `SourceMetrics` from `Default::default()`.

`ImageInfo`, `SampleFormat`, `Region`, `TileSize`, `TileSelection`, `TileGrid`,
`Part1ComponentDecodeRequest`, `TileRegionRequest` and the remaining enums are
fixed value types.
