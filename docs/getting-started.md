# Getting started with the Rust facade

Use `emuella-j2k` as the application-facing package. The
[root README](../README.md#get-started) shows how to depend on the source tree
and inspect a file you supply. The package is preparing its first release;
the API and supported profiles may change.

## Create and inspect a small image

This complete example creates an 8 × 8 unsigned greyscale image in memory,
encodes it as lossless JP2 and inspects the declared geometry. It requires no
external image or codec. Documentation verification executes the assertions.

```rust
use emuella_j2k::{encode, inspect, ColorModel, ComponentLayout, EncodeOptions,
    ImageInfo, ImageView, InspectOptions, SampleFormat};

fn main() -> Result<(), emuella_j2k::J2kError> {
    let samples = [42_u8; 64];
    let info = ImageInfo::new(8, 8, 1, SampleFormat::U8,
        ColorModel::Grayscale, ComponentLayout::Interleaved)?;
    let bytes = encode(ImageView::Interleaved {
        info: &info, samples: &samples, stride_bytes: 8,
    }, &EncodeOptions::default())?;
    let metadata = inspect(&bytes, &InspectOptions::default())?;
    let image = metadata.image.expect("encoded image metadata");
    assert_eq!((image.width, image.height, image.components), (8, 8, 1));
    Ok(())
}
```

Inspection and pixel decoding have different admission rules. Choose an input
and output request from the [supported profiles](supported-profiles.md) and
[decoder contracts](decoding-profiles.md) before assuming a decode is supported.
The [documentation index](README.md) links to detailed integration contracts.
