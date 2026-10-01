# Caller-owned full-image output

Use `decode` for owned `Image` storage. Use `decode_shape` with the intended
`DecodeOptions` before constructing an `ImageViewMut` for `decode_into`.
The [API map](rust-api.md) links both to their defining Rust contracts.
Discovery does not reconstruct image samples and cannot prove that later
packet or sample decoding will succeed.

## Discover, size and decode

This complete program uses the same supported 8 × 8 unsigned greyscale
lossless JP2 profile as [getting started](getting-started.md). It discovers
native interleaved output, adds three padding bytes per row, compares decoded
rows with owned output and checks padding. It creates every input in memory.
Paste it into `src/main.rs` of the application configured by the
[source dependency instructions](../README.md#get-started), then run `cargo run`.
Documentation verification executes it.

```rust
use emuella_j2k::{decode, decode_into, decode_shape, encode, ColorModel,
    ComponentLayout, DecodeMode, DecodeOptions, EncodeOptions, ImageData,
    ImageInfo, ImageView, ImageViewMut, SampleFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples: [u8; 64] = std::array::from_fn(|index| index as u8);
    let input_info = ImageInfo::new(8, 8, 1, SampleFormat::U8,
        ColorModel::Grayscale, ComponentLayout::Interleaved)?;
    let bytes = encode(ImageView::Interleaved {
        info: &input_info, samples: &samples, stride_bytes: 8,
    }, &EncodeOptions::default())?;
    let options = DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: ComponentLayout::Interleaved,
        ..DecodeOptions::default()
    };
    let shape = decode_shape(&bytes, &options)?;
    // This example is deliberately a uniform, unit-sampled U8 profile.
    assert_eq!(shape.sample_format, SampleFormat::U8);
    assert_eq!(shape.byte_order, None);
    assert_eq!((shape.width, shape.height, shape.output_components), (8, 8, 1));
    let target_info = ImageInfo::new(shape.width, shape.height,
        shape.output_components, shape.sample_format, shape.color_model,
        shape.layout)?;
    let row_bytes = usize::try_from(shape.width)?
        .checked_mul(usize::from(shape.output_components)).ok_or("row overflow")?;
    let stride_bytes = row_bytes.checked_add(3).ok_or("stride overflow")?;
    let capacity = stride_bytes.checked_mul(usize::try_from(shape.height)?)
        .ok_or("capacity overflow")?;
    let mut storage = vec![0xa5; capacity];
    {
        let mut target = ImageViewMut::Interleaved {
            info: &target_info, samples: &mut storage, stride_bytes,
        };
        decode_into(&bytes, &mut target, &options)?;
    } // End the mutable borrow before reading or reusing storage.
    let owned = decode(&bytes, &options)?;
    let ImageData::Interleaved(compact) = owned.data else {
        panic!("requested interleaved output");
    };
    assert_eq!(compact, samples);
    for (padded, expected) in storage.chunks_exact(stride_bytes)
        .zip(compact.chunks_exact(row_bytes)) {
        assert_eq!(&padded[..row_bytes], expected);
        assert!(padded[row_bytes..].iter().all(|&byte| byte == 0xa5));
    }
    Ok(())
}
```

Change the three padding bytes to five for a simple storage variation.
The active row width remains eight bytes. `decode_into` takes output layout
from `ImageViewMut::Planar` or `Interleaved`, overriding `options.target_layout`
for execution; use matching options during discovery so the description agrees.
The target's `ImageInfo` must match resolved dimensions, output component count,
sample format, colour model and layout. Metadata from `inspect` describes the
input and is not a substitute for output discovery, especially for mapped JP2.

## Storage and descriptors

| Type or field | Caller-visible contract |
|---|---|
| `ImageInfo` | Reference-image `width`, `height`, `components`, `sample_format`, `color_model`, `layout`. `new` checks nonzero geometry/count; it does not establish encoder or decoder support. |
| `SampleFormat` | Precision `bits_per_sample`, `signed` and `byte_order`. One-byte samples have no byte order; multi-byte samples require one. Constructors admit 1–38-bit descriptors, which is wider than any individual codec profile. |
| `ComponentLayout` | `Planar`: one byte plane per output component. `Interleaved`: samples for each pixel stored in output-component order. |
| `DecodeShape` | Resolved `width`, `height`, original `codestream_components`, `colour_channels` (rendered count or native count before selection), actual `output_components`, `sample_format`, `layout`, `byte_order`, `color_model`, `mode`. Use actual output count to size output. |
| `Image` / `ImageData` | Owned `info`, ordered `component_info`, and `Planes(Vec<Vec<u8>>)` or `Interleaved(Vec<u8>)`. Uniform output rows are tightly packed; native plane size follows its component descriptor. |
| `ComponentInfo` | Ordered source index (or `None` for rendered channels), native dimensions/origins, sampling separations and sample format. Reference-image geometry need not be native-plane geometry. |
| `Plane` / `PlaneMut` | Borrowed immutable/mutable byte samples with per-plane width, height, byte stride and format. `new` validates the plane's declared storage; it does not validate a whole decode request. |
| `ImageView` / `ImageViewMut` | Borrowed whole-image inputs/outputs, with matching info and either a plane slice or interleaved bytes/stride. Buffers remain caller-owned; Rust borrows govern when they may be accessed or reused. |

For the selected U8 interleaved path, `row_bytes = width × output_components`.
For uniform planar output, each row contains `width` samples; a planar target
needs one matching `PlaneMut` per component. Strides are **bytes**, may exceed
active row bytes, and must be at least the active row size. Each buffer needs
at least `stride_bytes × height` bytes, including padding after the last row.
Use checked arithmetic and an application allocation policy before allocating
from untrusted dimensions. Extra row padding is preserved on successful
decode in the selected profile.

U8 is unsigned one-byte storage. `U16_LE`, `U16_BE`, `I16_LE` and `I16_BE`
name two-byte formats with explicit signedness and byte order. Intermediate
9–16-bit native precision uses two-byte storage; it is not bit-packed.
Output must use the discovered format; `decode_into` does not accept an
arbitrary endianness or precision conversion. See
[Part 1 precision](part1-precision.md) for exact admitted formats and
[native grids](decoding-profiles.md#full-native-component-grids) when precision,
signedness or sampling differs across components. The uniform sizing formula
above is deliberately scoped to this example.

## Validation, failures and reuse

Invalid declared geometry, layout, format, stride or target-info agreement
returns `InvalidParameter`. An otherwise declared buffer shorter than required
returns `BufferTooSmall { required, provided }`. Malformed/truncated input and
unsupported requests have separate structured errors; see
[error handling](error-handling.md). Multiple defects have no promised error
precedence or stable diagnostic wording.

Successful shape discovery can be followed by decode failure. There is no
general atomic-on-error promise for all `decode_into` profiles: direct Part 1
component paths may publish rows during reconstruction, while particular
profiles stage privately. The selected independent U8 full-image profile has
the maintained [native-plane publication contract](native-planes.md#atomic-full-image-publication-contract).
[JP2 mapped presentation](jp2-presentation.md) and
[eight-component coding](native-eight-components.md) own their distinct
publication guarantees. Do not extend those guarantees to prepared, partial
or neighbouring profiles.

Caller-owned output alone does not imply zero allocation, zero copy or a
performance advantage. `decode_into_with_workspace` can reuse
`Part1DecodeWorkspace` for supported direct selective Part 1 routes; profiles
using owned staging leave that workspace unused. This example needs no prepared
state or shared workspace. Keep any reuse, execution limits and source lifetime
assumptions within the [positioned-source contract](part1-source-index.md).
