//! Native partial ROI presentation, independent of the Part 1 request planner.
use super::*;

#[allow(clippy::type_complexity)]
pub(super) fn prepare<'a>(
    input: &'a [u8],
    options: &PartialDecodeOptions,
) -> Result<
    Option<(
        codestream::PreparedHtj2kRoiWindowDecode<'a>,
        ImageInfo,
        Vec<ComponentInfo>,
    )>,
> {
    let Some(region) = options.region else {
        return Ok(None);
    };
    if !input.starts_with(&[0xff, 0x4f])
        || options.resolution != ResolutionLevel::Full
        || options.tile.is_some()
        || options.max_quality_layers.is_some()
        || options.target_layout != ComponentLayout::Planar
        || !matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
    {
        return Ok(None);
    }
    let Some(prepared) = codestream::prepare_htj2k_roi_window_decode(
        input,
        codestream::TileRegionRequest {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        },
    )
    .map_err(map_codestream_error)?
    else {
        return Ok(None);
    };
    let sample_format = SampleFormat::with_byte_order(
        prepared.bits_per_sample(),
        prepared.signed(),
        (prepared.bits_per_sample() > 8).then_some(SampleEndian::Little),
    )?;
    let info = ImageInfo::new(
        region.width,
        region.height,
        1,
        sample_format,
        ColorModel::Unknown,
        ComponentLayout::Planar,
    )?;
    let components = alloc::vec![ComponentInfo {
        source_component: Some(0),
        width: region.width,
        height: region.height,
        x_origin: region.x,
        y_origin: region.y,
        horizontal_separation: 1,
        vertical_separation: 1,
        sample_format
    }];
    Ok(Some((prepared, info, components)))
}

pub(super) fn decode(input: &[u8], options: &PartialDecodeOptions) -> Result<Option<Image>> {
    let Some((prepared, _, components)) = prepare(input, options)? else {
        return Ok(None);
    };
    let decoded = codestream::decode_prepared_htj2k_roi_window_owned(&prepared)
        .map_err(map_codestream_error)?;
    decoded_baseline_to_image_with_component_info(
        decoded,
        &DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: options.components.clone(),
            target_layout: ComponentLayout::Planar,
            ..DecodeOptions::default()
        },
        Some(components),
    )
    .map(Some)
}
