# Getting started with the Rust facade

Use `emuella-j2k` as the application-facing package. The
[root README](../README.md#get-started) shows how to depend on the source tree
and inspect a file you supply. The package is preparing its first release;
the API and supported profiles may change.

## Create, inspect and decode a small image

This complete example creates an 8 × 8 unsigned greyscale image in memory,
encodes it as lossless JP2, inspects the declared geometry and decodes the
stored component samples. It requires no external image or codec.
Documentation verification executes the assertions.

```rust
use emuella_j2k::{decode, encode, inspect, ColorModel, ComponentLayout,
    DecodeMode, DecodeOptions, EncodeOptions, ImageData, ImageInfo, ImageView,
    InspectOptions, SampleFormat};

fn main() -> Result<(), emuella_j2k::J2kError> {
    let samples: [u8; 64] = std::array::from_fn(|index| index as u8);
    let info = ImageInfo::new(8, 8, 1, SampleFormat::U8,
        ColorModel::Grayscale, ComponentLayout::Interleaved)?;
    let bytes = encode(ImageView::Interleaved {
        info: &info, samples: &samples, stride_bytes: 8,
    }, &EncodeOptions::default())?;
    let metadata = inspect(&bytes, &InspectOptions::default())?;
    let image = metadata.image.expect("encoded image metadata");
    assert_eq!((image.width, image.height, image.components), (8, 8, 1));

    let decoded = decode(&bytes, &DecodeOptions {
        mode: DecodeMode::Components,
        target_layout: ComponentLayout::Interleaved,
        ..DecodeOptions::default()
    })?;
    assert_eq!(decoded.info.sample_format, SampleFormat::U8);
    assert_eq!(decoded.data, ImageData::Interleaved(samples.to_vec()));
    Ok(())
}
```

`ImageView` borrows the input samples for encoding; `encode` returns owned
JP2 bytes. Here each pixel has one U8 sample and each row occupies eight bytes,
so `stride_bytes` is 8. The default encode options select lossless Part 1
with no wavelet decomposition.

`DecodeMode::Components` requests native sample values. The explicit
interleaved layout returns them in one owned byte buffer, which the example
compares with the original samples. Rendered output has separate presentation
rules; a successful native decode does not establish support for another mode,
profile or output request.

Inspection and pixel decoding have different admission rules. Choose an input
and output request from the [supported profiles](supported-profiles.md) and
[decoder contracts](decoding-profiles.md) before assuming a decode is supported.
The [documentation index](README.md) links to detailed integration contracts.
