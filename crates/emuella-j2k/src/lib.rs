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
//! Specialised integrations also have resource-bounded lossless encode,
//! reusable decode workspaces, prepared Part 1 requests and positioned-source
//! indexes. They are listed explicitly here and are not prerequisites for a
//! first application. Codestream, container and entropy-coding internals,
//! diagnostics and test fixtures are not part of this facade; the workspace's
//! lower-layer crates can change their APIs in any release.
//!
//! # Evolution
//!
//! Error, classification and option types are `#[non_exhaustive]` so that later
//! releases can add variants and fields. Match with a wildcard arm, and build
//! options from `Default` (or [`Htj2kLossyEncodeOptions::new`]) with the
//! chainable `with_*` setters or field assignment, rather than struct literals.
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

// Keep this list explicit. A glob would make every public item added to a
// lower-layer crate part of this facade's compatibility surface.

// Results and structured failures.
pub use emuella_j2k_core::{J2kError, Result, UnsupportedFeature};

// Image description, sample storage and caller-owned views.
pub use emuella_j2k_core::{
    ColorModel, ComponentInfo, ComponentLayout, ImageInfo, ImageView, ImageViewMut, Plane,
    PlaneMut, SampleEndian, SampleFormat,
};

// Inspection.
pub use emuella_j2k_core::{
    CodestreamInfo, CodestreamKind, ContainerInfo, EntropyCoder, InputFormat, InspectOptions,
    Metadata, MetadataKind, MetadataRecord, ProgressionOrder, SupportStatus, TileGrid,
    WaveletTransform, inspect,
};

// Full, caller-output and partial decode.
pub use emuella_j2k_core::{
    ComponentSelection, DecodeMode, DecodeOptions, DecodeShape, Image, ImageData,
    PartialDecodeOptions, Region, ResolutionLevel, TileSelection, decode, decode_into,
    decode_partial, decode_partial_component_info, decode_partial_info, decode_partial_into,
    decode_rendered_partial, decode_rendered_partial_info, decode_rendered_partial_into,
    decode_shape,
};

// Part 1 and HTJ2K encode.
pub use emuella_j2k_core::{
    EncodeOptions, EncodeQuality, Htj2kEncodeOptions, Htj2kLossyEncodeOptions, OutputFormat,
    TileSize, encode, encode_htj2k, encode_htj2k_jph, encode_htj2k_lossy, encode_htj2k_lossy_jph,
    encode_into,
};

// Resource-bounded lossless encode.
#[cfg(feature = "std")]
pub use emuella_j2k_core::{
    LosslessEncodeLimits, LosslessEncodeRequirements, encode_lossless_bypass_with_limits,
    encode_with_limits, lossless_bypass_encode_requirements, lossless_encode_requirements,
};

// Reusable decode workspaces.
#[cfg(feature = "std")]
pub use emuella_j2k_core::{Htj2kDecodeWorkspace, decode_htj2k_with_workspace};
pub use emuella_j2k_core::{
    Part1DecodeWorkspace, decode_into_with_workspace, decode_partial_into_with_workspace,
};

// Prepared Part 1 requests and positioned sources.
pub use emuella_j2k_core::{
    CodestreamSource, DecodeInstrumentation, DecodeStageTimings, Part1ComponentDecodeRequest,
    Part1SourceIndex, Part1SourceIndexLimits, Part1SourceInspection, PreparedPart1Decode,
    PreparedPart1ExecutionOptions, SliceSource, SourceError, SourceErrorKind, TileRegionRequest,
    execute_prepared_part1_decode_into_with_workspace, inspect_part1_source, prepare_part1_decode,
    prepare_part1_decode_from_source,
};

/// Compile-time boundary checks for the facade's compatibility surface.
///
/// Lower-layer modules, diagnostics and test fixtures are unreachable:
///
/// ```compile_fail,E0432
/// use emuella_j2k::codestream;
/// ```
///
/// ```compile_fail,E0432
/// use emuella_j2k::container;
/// ```
///
/// ```compile_fail,E0432
/// use emuella_j2k::decode_native_d2_profiled;
/// ```
///
/// ```compile_fail,E0432
/// use emuella_j2k::decode_htj2k_cleanup_vlc_output_probe_with_workspace;
/// ```
///
/// Option types cannot be built with struct literals, so fields can be added:
///
/// ```compile_fail,E0639
/// let _ = emuella_j2k::InspectOptions {
///     preserve_raw_metadata: true,
///     classify_support: true,
/// };
/// ```
///
/// ```compile_fail,E0639
/// let _ = emuella_j2k::DecodeOptions {
///     mode: emuella_j2k::DecodeMode::Components,
///     ..emuella_j2k::DecodeOptions::default()
/// };
/// ```
///
/// Error and classification enums need a wildcard arm, so variants can be added:
///
/// ```compile_fail,E0004
/// fn status(error: &emuella_j2k::J2kError) -> u8 {
///     use emuella_j2k::J2kError::*;
///     match error {
///         InvalidParameter { .. } => 0,
///         InvalidInput { .. } => 1,
///         Source { .. } => 2,
///         TruncatedInput { .. } => 3,
///         Unsupported { .. } => 4,
///         BufferTooSmall { .. } => 5,
///         InternalInvariant { .. } => 6,
///     }
/// }
/// ```
///
/// ```compile_fail,E0004
/// fn known(format: emuella_j2k::InputFormat) -> bool {
///     use emuella_j2k::InputFormat::*;
///     match format {
///         Jp2 | J2kCodestream | Jph | Htj2kCodestream => true,
///         Unknown => false,
///     }
/// }
/// ```
///
/// The supported spellings of the same requests compile:
///
/// ```
/// use emuella_j2k::{DecodeMode, DecodeOptions, InputFormat, J2kError};
/// let options = DecodeOptions::default().with_mode(DecodeMode::Components);
/// assert_eq!(options.mode, DecodeMode::Components);
/// let mut assigned = DecodeOptions::default();
/// assigned.mode = DecodeMode::Components;
/// assert_eq!(assigned, options);
/// fn known(format: InputFormat) -> bool {
///     match format {
///         InputFormat::Unknown => false,
///         _ => true,
///     }
/// }
/// assert!(known(InputFormat::Jp2));
/// let error = J2kError::TruncatedInput { needed: 1, remaining: 0 };
/// assert!(matches!(error, J2kError::TruncatedInput { needed: 1, .. }));
/// ```
#[cfg(doctest)]
pub struct FacadeBoundary;

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

    #[test]
    fn facade_lists_every_re_export_explicitly() {
        // A glob would silently widen the facade whenever a lower layer adds a
        // public item; compile-fail doctests cover the named internal modules.
        let source = include_str!("lib.rs");
        let items = source.split("#[cfg(test)]").next().unwrap();
        let code: Vec<&str> = items
            .lines()
            .map(str::trim_start)
            .filter(|line| !line.starts_with("//"))
            .collect();
        assert!(
            code.iter()
                .any(|line| line.starts_with("pub use emuella_j2k_core::"))
        );
        assert!(
            !code.iter().any(|line| line.contains("::*")),
            "facade re-exports must be explicit"
        );
        assert!(
            !code.iter().any(|line| line.starts_with("pub mod")),
            "facade must not add public modules"
        );
    }
}
