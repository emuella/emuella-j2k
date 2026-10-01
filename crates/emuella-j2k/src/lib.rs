//! Primary public Rust API for the Emuella JPEG 2000 and HTJ2K codec.
//!
//! This facade keeps the application-facing package and crate names independent
//! from the workspace's internal layering. The pre-release API is implemented by
//! `emuella-j2k-core` and re-exported here.
//!
//! # Choose an operation
//!
//! | Application task | Entry points | Principal contracts |
//! |---|---|---|
//! | Inspect structure without reconstructing samples | [`inspect`] | [`InspectOptions`], [`Metadata`], [`SupportStatus`] |
//! | Decode an owned full image | [`decode`] | [`DecodeOptions`], [`DecodeMode`], [`Image`], [`ImageData`] |
//! | Discover and fill caller-owned storage | [`decode_shape`], [`decode_into`] | [`DecodeShape`], [`ImageInfo`], [`ImageViewMut`], [`PlaneMut`] |
//! | Select native components, a region or reduction | [`decode_partial`], [`decode_partial_info`], [`decode_partial_component_info`], [`decode_partial_into`] | [`PartialDecodeOptions`], [`ComponentInfo`] |
//! | Request bounded rendered partial output | [`decode_rendered_partial`], [`decode_rendered_partial_info`], [`decode_rendered_partial_into`] | [`PartialDecodeOptions`] and the sYCC profile |
//! | Encode Part 1 lossless or target-rate output | [`encode`], [`encode_into`] | [`ImageView`], [`EncodeOptions`], [`EncodeQuality`] |
//! | Encode lossless HTJ2K raw/JPH | [`encode_htj2k`], [`encode_htj2k_jph`] | [`Htj2kEncodeOptions`] |
//! | Encode target-rate HTJ2K raw/JPH | [`encode_htj2k_lossy`], [`encode_htj2k_lossy_jph`] | [`Htj2kLossyEncodeOptions`] |
//!
//! [`DecodeMode::Components`] preserves native sample meaning; rendered output
//! has separate colour, mapping and alpha admission. Inspection success and
//! [`SupportStatus::Supported`] do not approve every later request. Shape
//! discovery can succeed before packet or sample reconstruction fails. Match
//! structured [`J2kError`] variants rather than diagnostic strings.
//!
//! Begin with the [application API map](https://github.com/emuella/emuella-j2k/blob/main/docs/rust-api.md),
//! [caller-output journey](https://github.com/emuella/emuella-j2k/blob/main/docs/caller-owned-output.md)
//! and [structured failures](https://github.com/emuella/emuella-j2k/blob/main/docs/error-handling.md).
//! The [supported-profile guide](https://github.com/emuella/emuella-j2k/blob/main/docs/supported-profiles.md)
//! and [decoder contracts](https://github.com/emuella/emuella-j2k/blob/main/docs/decoding-profiles.md)
//! own exact limits. Selected support is not general JPEG 2000/HTJ2K conformance.
//!
//! # Features and specialised layers
//!
//! Ordinary application examples use default `std` on the host target.
//! `parallel` and `simd` enable `std` and retain serial/scalar fallbacks.
//! Library compilation without default features does not establish availability
//! of every algorithmic route; some resource and source APIs are `std`-gated.
//! See the API map for local generation commands and feature boundaries.
//!
//! The facade deliberately also exposes [`codestream`], [`container`], prepared
//! requests, indexes, reusable workspaces and diagnostic hooks. Their specialised
//! contracts are available without making them prerequisites for a first
//! application. This facade retains all defining-core exports.
//!
//! # In-memory start
//!
//! A small project-authored image can be encoded and inspected without a file:
//!
//! ```
//! use emuella_j2k::{encode, inspect, ColorModel, ComponentLayout, EncodeOptions,
//!     ImageInfo, ImageView, InspectOptions, SampleFormat};
//! let samples = [42_u8; 64];
//! let info = ImageInfo::new(8, 8, 1, SampleFormat::U8,
//!     ColorModel::Grayscale, ComponentLayout::Interleaved)?;
//! let bytes = encode(ImageView::Interleaved {
//!     info: &info, samples: &samples, stride_bytes: 8,
//! }, &EncodeOptions::default())?;
//! let metadata = inspect(&bytes, &InspectOptions::default())?;
//! assert_eq!(metadata.image.as_ref().map(|image| (image.width, image.height)), Some((8, 8)));
//! # Ok::<(), emuella_j2k::J2kError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use emuella_j2k_core::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_exposes_the_primary_api_types() {
        let _ = InspectOptions::default();
        let _ = DecodeOptions::default();
        let _ = EncodeOptions::default();
        let _jph_encoder: fn(ImageView<'_>, &Htj2kEncodeOptions) -> Result<Vec<u8>> =
            encode_htj2k_jph;
        let _lossy: fn(ImageView<'_>, &Htj2kLossyEncodeOptions) -> Result<Vec<u8>> =
            encode_htj2k_lossy;
        let _lossy_jph: fn(ImageView<'_>, &Htj2kLossyEncodeOptions) -> Result<Vec<u8>> =
            encode_htj2k_lossy_jph;
    }
}
