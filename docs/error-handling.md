# Structured errors and inspection boundaries

Application operations return `Result<T, J2kError>`. Match the error variant
and relevant structured fields; diagnostic strings are for people and are not
a stable parsing contract. Inputs and options with several defects have no
promised error precedence. Start at the [API map](rust-api.md) and choose a
[supported input and request](supported-profiles.md).

## Invalid input and unsupported requests

This complete program creates a supported image in memory. It distinguishes
empty/truncated input, malformed container bytes, readable metadata followed by
an unsupported requested operation, invalid caller geometry and insufficient
caller storage. Paste it into `src/main.rs` with the
[source dependency](../README.md#get-started) and run `cargo run`.
Documentation verification executes its variant assertions.

```rust
use emuella_j2k::{decode, encode, inspect, ColorModel, ComponentLayout,
    ComponentSelection, DecodeOptions, EncodeOptions, ImageInfo, ImageView,
    InspectOptions, J2kError, PlaneMut, SampleFormat};

fn main() -> Result<(), J2kError> {
    assert!(matches!(inspect(&[], &InspectOptions::default()),
        Err(J2kError::TruncatedInput { .. })));
    let samples = [42_u8; 64];
    let info = ImageInfo::new(8, 8, 1, SampleFormat::U8,
        ColorModel::Grayscale, ComponentLayout::Interleaved)?;
    let bytes = encode(ImageView::Interleaved {
        info: &info, samples: &samples, stride_bytes: 8,
    }, &EncodeOptions::default())?;
    let metadata = inspect(&bytes, &InspectOptions::default())?;
    assert!(metadata.image.is_some());
    // Default mode is rendered: explicit source-component selection is unsupported.
    let request = DecodeOptions {
        requested_components: ComponentSelection::Indices(vec![0]),
        ..DecodeOptions::default()
    };
    assert!(matches!(decode(&bytes, &request),
        Err(J2kError::Unsupported { .. })));
    let mut malformed = bytes.clone();
    malformed[8] ^= 1; // Corrupt the project-created JP2 signature payload.
    assert!(matches!(inspect(&malformed, &InspectOptions::default()),
        Err(J2kError::InvalidInput { .. })));
    assert!(matches!(ImageInfo::new(0, 8, 1, SampleFormat::U8,
        ColorModel::Grayscale, ComponentLayout::Planar),
        Err(J2kError::InvalidParameter { .. })));
    let mut short = [0_u8; 63];
    assert!(matches!(PlaneMut::new(&mut short, 8, 8, 8, SampleFormat::U8),
        Err(J2kError::BufferTooSmall { required: 64, provided: 63 })));
    Ok(())
}
```

Readable structure and supported pixel requests are different observations.
The original bytes are structurally inspectable; the explicit source selection
in rendered mode is rejected without making those bytes malformed. The corrupted
signature is an input defect. A successful `inspect`, `SupportStatus::Supported`
or `decode_shape` does not prove every requested mode, component selection,
packet reconstruction or sample value will succeed.

## Error variants

| Variant | Structured context and response |
|---|---|
| `InvalidParameter { parameter, message }` | Caller description or option is invalid. Check dimensions, sample format, stride and option combinations. |
| `InvalidInput { offset, message }` | Input fails structural or reconstructed-data validation; offset is optional. Do not retry it as a supported image based only on earlier inspection. |
| `Source { offset, requested, available, message }` | Positioned source read failed with byte-range context. Source-based integrations own their I/O policy. |
| `TruncatedInput { needed, remaining }` | Input is incomplete for the attempted operation. `needed` reports additional required bytes at that failure, not a full-file length predictor. |
| `Unsupported { feature, detail }` | Input feature or requested combination is outside the implemented route. Choose another documented request or report the missing capability. |
| `BufferTooSmall { required, provided }` | Declared byte storage is insufficient. Discover and validate the intended output, then size its rows and capacity. |
| `InternalInvariant { message }` | An internal consistency check failed. Report a reproducible case rather than treating it as normal support admission. |

`UnsupportedFeature` names `InputFormat`, `OutputFormat`, `ContainerBox`,
`MarkerSegment`, `ProgressionOrder`, `WaveletTransform`, `EntropyCoder`,
`ColorModel`, `ComponentLayout`, `PartialDecodeMode` and `IncrementalInput`.
These are broad categories, not a complete feature-support matrix. Some
unsupported option combinations use a category such as `ComponentLayout`;
derive permitted combinations from their maintained contracts.

`SupportStatus::Unsupported` and `Unknown` are metadata values returned by a
successful inspection, whereas `J2kError::Unsupported` is an operation failure.
Disabling `InspectOptions::classify_support` produces `Unknown` rather than an
approval. Best-effort compatibility does not remove these distinctions.

For caller output, validation failure and reconstruction failure have different
implications. Use the [caller-output guide](caller-owned-output.md#validation-failures-and-reuse)
and the selected profile's publication contract; no general unchanged-buffer
or transactional decode guarantee follows from the error type.
