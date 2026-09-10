//! Owned raw classic D2 resource policy.
use super::*;

pub use codestream::{LosslessEncodeLimits, LosslessEncodeRequirements};

pub(super) fn is_scalable_lossless(image: ImageView<'_>, options: &EncodeOptions) -> bool {
    let info = image_info(image);
    options.format == OutputFormat::J2kCodestream
        && options.quality == EncodeQuality::Lossless
        && options.decomposition_levels == 2
        && options.tile_size.is_none()
        && options.metadata.is_empty()
        && matches!(info.sample_format, SampleFormat::U8 | SampleFormat::U16_LE)
        && (is_native_grayscale_u8_encode(info)
            || is_native_rgb_u8_encode(info)
            || is_native_grayscale_u16_le_encode(info)
            || is_native_rgb_u16_le_encode(info)
            || is_native_eight_component_u16(info))
        && (4..=32768).contains(&info.width)
        && (4..=32768).contains(&info.height)
}

/// Check the conservative working allocation envelope before encoding.
///
/// Accepts only raw, single-tile, lossless reversible D2, LRCP, unsigned U8/U16
/// greyscale/RGB, or exactly eight U16_LE components with `ColorModel::Unknown`,
/// and no metadata. Eight components preserve positional native samples without
/// MCT and admit at most 32 Mi pixels (256 Mi aggregate samples).
/// Other profiles return an error; no limit is
/// silently ignored. This validates geometry/options, not caller sample storage.
/// See `docs/scalable-lossless.md` for accounting and runtime output admission.
pub fn lossless_encode_requirements(
    info: &ImageInfo,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    requirements::<false>(info, options, limits)
}

/// Check the D2 selective-bypass working envelope without reading samples.
/// Admission matches `lossless_encode_requirements` with an additional bounded
/// segment-metadata allowance. Query and encode in the same calling pool.
pub fn lossless_bypass_encode_requirements(
    info: &ImageInfo,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    requirements::<true>(info, options, limits)
}

fn requirements<const BYPASS: bool>(
    info: &ImageInfo,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    validate_encode_options(options)?;
    if !is_native_eight_component_u16(info) {
        validate_encode_image_info(info)?;
    }
    let dummy = ImageView::Interleaved {
        info,
        samples: &[],
        stride_bytes: 0,
    };
    if !is_scalable_lossless(dummy, options) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "explicit lossless limits require raw single-tile U8/U16 grey/RGB or eight native U16 components, D2 without metadata",
        ));
    }
    if BYPASS {
        codestream::lossless_d2_bypass_requirements(
            info.width,
            info.height,
            info.components,
            *limits,
        )
    } else {
        codestream::lossless_d2_requirements(info.width, info.height, info.components, *limits)
    }
    .map_err(map_codestream_error)
}

/// Encode owned raw classic lossless D2 with explicit allocation limits.
///
/// The limits cover the encoder invocation, excluding borrowed input, other
/// application allocations and later decoding. Output capacity has its own
/// limit. An insufficient working limit fails before coefficient allocation;
/// an insufficient output limit can fail during coding. This operation returns
/// no partial output. Existing `EncodeOptions` struct literals remain valid.
pub fn encode_with_limits(
    image: ImageView<'_>,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    encode_impl::<false>(image, options, limits)
}

/// Encode owned raw lossless D2 with selective arithmetic bypass (COD style 1).
/// Accepts exactly the explicit-limit D2 sample models and geometry. RGB retains
/// reversible MCT. Existing default encoders continue to write style zero.
/// Admission includes bounded segment metadata; no partial output is returned.
pub fn encode_lossless_bypass_with_limits(
    image: ImageView<'_>,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    encode_impl::<true>(image, options, limits)
}

fn encode_impl<const BYPASS: bool>(
    image: ImageView<'_>,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<Vec<u8>> {
    requirements::<BYPASS>(image_info(image), options, limits)?;
    validate_image_view(&image)?;
    let info = image_info(image);
    if let ImageView::Planar { planes, .. } = image {
        for plane in planes {
            if plane.width != info.width
                || plane.height != info.height
                || plane.sample_format != info.sample_format
            {
                return Err(unsupported(
                    UnsupportedFeature::ComponentLayout,
                    "lossless D2 plane metadata must agree with image metadata",
                ));
            }
        }
    }
    let bytes = usize::from(info.sample_format.bits_per_sample / 8);
    let mut planes = [codestream::LosslessD2Plane {
        samples: &[],
        stride_bytes: 0,
        sample_step_bytes: bytes,
    }; 8];
    for (index, target) in planes
        .iter_mut()
        .enumerate()
        .take(usize::from(info.components))
    {
        *target = match image {
            ImageView::Planar { planes, .. } => codestream::LosslessD2Plane {
                samples: planes[index].samples,
                stride_bytes: planes[index].stride_bytes,
                sample_step_bytes: bytes,
            },
            ImageView::Interleaved {
                samples,
                stride_bytes,
                ..
            } => codestream::LosslessD2Plane {
                samples: &samples[index * bytes..],
                stride_bytes,
                sample_step_bytes: bytes * usize::from(info.components),
            },
        };
    }
    let encode = if BYPASS {
        codestream::encode_lossless_d2_bypass
    } else {
        codestream::encode_lossless_d2
    };
    encode(
        info.width,
        info.height,
        info.sample_format.bits_per_sample,
        &planes[..usize::from(info.components)],
        *limits,
    )
    .map_err(map_codestream_error)
}

// Unknown retains component positions without asserting spectral or colour meaning.
fn is_native_eight_component_u16(info: &ImageInfo) -> bool {
    info.components == 8
        && info.color_model == ColorModel::Unknown
        && info.sample_format == SampleFormat::U16_LE
}
