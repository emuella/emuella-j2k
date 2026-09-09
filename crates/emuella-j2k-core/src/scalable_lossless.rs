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
            || is_native_rgb_u16_le_encode(info))
        && info.width >= 4
        && info.height >= 4
}

/// Check the conservative working allocation envelope before encoding.
///
/// Accepts only raw, single-tile, lossless reversible D2, LRCP, unsigned U8/U16
/// greyscale/RGB and no metadata. Other profiles return an error; no limit is
/// silently ignored. This validates geometry/options, not caller sample storage.
/// See `docs/scalable-lossless.md` for accounting and runtime output admission.
pub fn lossless_encode_requirements(
    info: &ImageInfo,
    options: &EncodeOptions,
    limits: &LosslessEncodeLimits,
) -> Result<LosslessEncodeRequirements> {
    validate_encode_options(options)?;
    validate_encode_image_info(info)?;
    let dummy = ImageView::Interleaved {
        info,
        samples: &[],
        stride_bytes: 0,
    };
    if !is_scalable_lossless(dummy, options) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "explicit lossless limits require raw single-tile U8/U16 grey/RGB D2 without metadata",
        ));
    }
    codestream::lossless_d2_requirements(info.width, info.height, info.components, *limits)
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
    lossless_encode_requirements(image_info(image), options, limits)?;
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
    }; 3];
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
    codestream::encode_lossless_d2(
        info.width,
        info.height,
        info.sample_format.bits_per_sample,
        &planes[..usize::from(info.components)],
        *limits,
    )
    .map_err(map_codestream_error)
}
