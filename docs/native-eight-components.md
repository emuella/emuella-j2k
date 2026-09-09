# Native eight-component lossless coding

The raw classic Part 1 D2 encoder accepts exactly eight positional unsigned
16-bit components as `ColorModel::Unknown`, `SampleFormat::U16_LE`. Each
component is independent: source index 0 through 7, dimensions and every sample
value survive a native round trip. `Unknown` explicitly supplies no colour or
spectral interpretation. The application owns band names, wavelengths and any
rendering decisions. No RGB projection, normalisation, byte swapping, band
selection or spectral transform occurs.

This is an additive encode profile; established greyscale/RGB APIs and
codestream bytes retain their behaviour. Raw codestream metadata does not
identify arbitrary eight-component data as multispectral, so the API uses the
existing `Unknown` model rather than inferring a spectral colour model.

## Public use and profile

```rust
use emuella_j2k::{
    ColorModel, ComponentLayout, DecodeMode, DecodeOptions, EncodeOptions,
    ImageInfo, ImageView, OutputFormat, SampleFormat, decode, encode,
};

# fn round_trip(samples: &[u8]) -> emuella_j2k::Result<()> {
let info = ImageInfo::new(
    67, 65, 8, SampleFormat::U16_LE, ColorModel::Unknown,
    ComponentLayout::Interleaved,
)?;
let stream = encode(
    ImageView::Interleaved { info: &info, samples, stride_bytes: 67 * 8 * 2 },
    &EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    },
)?;
let image = decode(&stream, &DecodeOptions {
    mode: DecodeMode::Components,
    target_layout: ComponentLayout::Interleaved,
    ..Default::default()
})?;
assert_eq!(image.info.components, 8);
assert_eq!(image.info.color_model, ColorModel::Unknown);
# Ok(())
# }
```

For planar input, provide eight `Plane` values in source order and set the
image layout to `Planar`. Each plane has the same width, height and U16_LE
format. Planar rows contain width × 2 meaningful bytes; interleaved rows contain
width × 8 × 2 meaningful bytes. Both accept row padding. Interleaved pixels
store components 0 through 7, with each unsigned word in little-endian order.
Decode accepts both layouts; `Image::component_info` retains each source index.

The new encoder profile requires lossless quality, raw J2K output, exactly two
reversible 5/3 decompositions, one full-image tile, LRCP, one layer, unit sampling,
zero origins, no metadata and no MCT. Each axis is 4–32768 and the image has at
most 32 Mi spatial pixels. Other component counts, eight-component U8/signed or
intermediate precision, RGB-labelled eight-band input, HT, lossy coding,
containers, tiling and other decomposition levels are outside this new encode
profile. The encoder never silently chooses a display interpretation.

The qualified round trip uses full-image `DecodeMode::Components`, all eight
components and full resolution. Default rendered output is unsupported.
The existing decoder already admits broader native component profiles; this
change neither widens nor narrows those admission rules and does not qualify
additional selective, reduced, tiled, container or HT decode profiles.

## Resources and caller buffers

`lossless_encode_requirements` and `encode_with_limits` use the existing
[checked allocation model](scalable-lossless.md). C is eight, S is 8 × width ×
height, and the code-block count includes all eight planes. The 32 Mi-pixel
limit reconciles the existing decoder ceiling of 256 Mi aggregate samples;
no former 64 Mi-pixel per-component promise is applied to eight bands. Working
and output limits remain independent. Passing geometry admission does not
promise that a caller's output budget or available system memory is sufficient.
Geometry/resource tests exercise the maximum without allocating maximum images;
ordinary authored round trips cover bounded representative inputs.

`encode_into` appends a complete stream after the caller's existing prefix.
An encode error preserves that prefix. `decode_into` and
`decode_into_with_workspace` stage all eight components before publishing to
caller buffers, including planar builds with the parallel feature. A malformed
later band's entropy data cannot expose earlier decoded bands. Invalid shape,
short storage and malformed stream errors preserve all destination bytes;
success preserves row padding and trailing storage. This staging adds owned
output memory during caller decode and does not change the encode allocation
contract. Partial decode retains its existing contracts.

## Standards authority

Project-authored implementation and tests follow ISO/IEC 15444-1:2024 / ITU-T
T.800 V4 (July 2024), reviewed retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`:

- A.5.1, Tables A.9 and A.11, PDF pages 41–44: SIZ records component count,
  precision, signedness and sampling. Eight unsigned 16-bit unit-sampled
  components use `Csiz=8`, eight `Ssiz=15, XRsiz=1, YRsiz=1` records and `Lsiz=62`.
- A.6.1, Tables A.14, A.17 and A.20, PDF pages 47–49: `MCT=0` disables the
  component transform independently of `SPcod.Transformation=1` reversible
  wavelet coding and the decomposition-count field of two.
- A.6.2, Table A.22, PDF page 51, and A.6.5, Table A.31, PDF page 54:
  component indices in COC/QCC remain one byte at eight components. Homogeneous
  output uses main coding and quantisation defaults without component overrides.
- G.2–G.3, PDF pages 154–155: the defined component transforms act on the first
  three components when enabled. Eight components do not create a requirement
  to enable MCT. This implementation codes every supplied band independently.

These are implementation boundaries and authored explanations, not a claim of
general JPEG 2000 conformance or spectral metadata support.

## Exploration and authored evidence

The question was whether existing scalable coefficient, packet and allocation
machinery could preserve eight positional U16 planes without fixed RGB storage.
The small probe at source `66185cf7b8e268206b942eadee54d4c4f75487be` used the
`lossless_allocations` example with 67 × 65 × 8, 16-bit input and planar layout.
The deterministic xorshift input identity was
`6c05e4206c2e77eacd4d73ab879cb59054e05603e9aa46fafd4fa970fb23175e`;
output identity was
`b845069458502dbb9c59e76b0d42c113f6359cc9f4274e59a9da6e500e58e572`.
The probe produced 74,876 bytes with capacity 87,040, measured requested
encoder peak 284,632 bytes and conservative working allowance 2,152,047,492
bytes. Native decode was exact. The retain decision follows exact positional
bytes and checked resource admission; the small timing is not a performance
claim. This checkpoint identifies the probe and does not claim final delivery.

`native_eight_components` exercises full-range, band-distinct authored values,
asymmetric byte order, odd dimensions, 64/128/256 processing boundaries, both
padded input/output layouts, explicit/ordinary encoder identity, header fields,
source component metadata, append semantics, unsupported encode declarations,
checked arithmetic/aggregate limits and late-payload failure atomicity.
The opt-in allocation diagnostic accepts `67 65 8 16 planar` or
`67 65 8 16 interleaved`, and keeps generated pixels and streams in memory.
Real imagery and independent-codec measurements belong to external authorised
qualification evidence, not the self-contained source tree.
