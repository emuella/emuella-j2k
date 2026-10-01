//! Primary public Rust API for the Emuella JPEG 2000 and HTJ2K codec.
//!
//! This facade keeps the application-facing package and crate names independent
//! from the workspace's internal layering. The pre-release API is implemented by
//! `emuella-j2k-core` and re-exported here.
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
