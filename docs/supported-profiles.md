# Supported profiles

Emuella inspects JPEG 2000 and HTJ2K structures and implements selected encode
and decode requests. Choose the operation and output you need before treating an
input as supported. [Inspection](decoding-profiles.md#htj2k-decoding-and-support-admission),
packet validity, decode admission and reconstructed pixels are distinct results.
The Rust facade reports unsupported features rather than claiming general
JPEG 2000, HTJ2K or JP2 conformance.

## I want to inspect an input

`inspect` accepts raw J2K/HTJ2K codestreams and JP2/JPH containers within its
parser bounds. It returns metadata and a support classification without
allocating image samples. A structurally accepted input may still be outside
the implemented decode route. In particular, JPH colour interpretation and
mixed classic/HT packet semantics have narrower bounds than structural
inspection; see [JPH inspection](decoding-profiles.md#jph-inspection) and the
[HTMIX disposition](htmix-disposition.md).

## I want to decode pixels

| Input and request | Supported output to start with | Exact contract |
|---|---|---|
| Classic Part 1 raw J2K or JP2, selected full-image requests | Rendered greyscale/RGB or native components, according to format and admission | [Native planes](native-planes.md), [precision](part1-precision.md), [decoder profiles](decoding-profiles.md) |
| JP2 with selected palette, mapping, channel order or straight alpha | Full rendered U8 greyscale, RGB or RGBA in either layout | [JP2 presentation](jp2-presentation.md) |
| JP2 with selected direct higher-precision greyscale or direct sYCC | Bounded rendered projection with format-specific geometry and output limits | [High-precision greyscale](architecture.md#direct-high-precision-jp2-greyscale-projection), [sYCC](architecture.md#bounded-full-frame-and-partial-sycc-projection) |
| Classic Part 1 positioned source with selected regions, including a bounded reversible MCT route | Requested native components with their source geometry; no general rendered region promise | [Decoder profiles](decoding-profiles.md#classic-part-1-regional-output-with-reversible-mct), [source index](part1-source-index.md) |
| Raw HTJ2K in selected full-image HTONLY profiles; JPH in the two-level irreversible route | Full native components with profile-specific admission | [Decoder profiles](decoding-profiles.md#htj2k-decoding-and-support-admission), [irreversible foundation](ht-lossy-foundations.md) |
| Raw HTJ2K in selected component, region or reduced-resolution profiles | Explicit planar native component request, with profile-specific selection and geometry | [Decoder profiles](decoding-profiles.md#native-reduced-and-regional-requests), [testing matrix](testing.md#bounded-ds0-result) |

Native components are codestream samples, not necessarily display channels.
`ImageInfo` describes the reference image; component descriptors carry native
sampling and origin where they differ. `DecodeMode::Rendered` may apply a
bounded JP2 presentation, while `DecodeMode::Components` preserves native
component meaning. A successful selected component request does not imply
all-component, full-image, rendered, interleaved, JPH or another reduction
request. [Exact decoder profiles](decoding-profiles.md) state those neighbours.

Some selected regional paths reconstruct only the required component or
spatial window; their request and resource limits differ. Use the
[decoder profiles](decoding-profiles.md) and
[lossy HT public contract](ht-lossy-public-api.md#native-decoding) before
relying on region, caller-buffer or workspace behaviour.

## I want to encode an image

| Input and output | Significant boundary | Exact contract |
|---|---|---|
| Unsigned U8/U16_LE greyscale or RGB to lossless Part 1 raw J2K | Selected single-tile D2 route, with checked resource limits for explicit-limit APIs | [Scalable lossless](scalable-lossless.md) |
| Eight positional U16_LE components to lossless raw J2K | Native components, no inferred colour roles or MCT | [Eight-component coding](native-eight-components.md) |
| Intermediate 9–15-bit unsigned greyscale to lossless Part 1 | Two-byte storage and bounded levels/tiling; not RGB or target-rate at these precisions | [Part 1 precision](part1-precision.md) |
| Greyscale/RGB to target-rate Part 1 J2K or JP2 | Bounded irreversible profile; rate and distortion evidence are scoped | [Rate control](encoder-rate-control-calibration.md) |
| U8/U16_LE greyscale or RGB to lossless HTJ2K raw or JPH | Zero or one reversible decomposition in the selected profile | [Architecture contract](architecture.md#bounded-reversible-htj2k-encode-and-jph-output) |
| U8/U16_LE greyscale or RGB to lossy HTJ2K raw or JPH | Exactly two irreversible levels, checked rate and resource envelope; unattainable rates fail | [Lossy HT public API](ht-lossy-public-api.md) |

These rows are independent profiles. Container availability for one encoder
does not make it available for another. Input layout, precision, dimensions,
rate and output-capacity restrictions belong to the linked contracts.

## Where is the evidence?

[Testing](testing.md) records self-contained verification, opt-in corpus
qualification, selected HTONLY points and the conformance-worker procedure.
Focused contracts retain their own resource bounds and qualification links.
Those observations are scoped to their named revisions and inputs; they do
not widen the supported operation table above.
