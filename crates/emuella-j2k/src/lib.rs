//! Primary public Rust API for the Emuella JPEG 2000 and HTJ2K codec.
//!
//! This facade keeps the stable package and crate names independent from the
//! workspace's internal layering. The API is implemented by
//! `emuella-j2k-core` and re-exported here.

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
