//! Native high-component HT presentation before inverse RCT.
use super::*;

pub(super) fn prepare<'a>(
    input: &'a [u8],
    options: &DecodeOptions,
) -> Result<Option<codestream::PreparedHtj2kReducedComponentDecode<'a>>> {
    if !input.starts_with(&[0xff, 0x4f]) || !is_p0_13_decode_request(options) {
        return Ok(None);
    }
    codestream::prepare_htj2k_high_component_decode(input).map_err(map_codestream_error)
}

pub(super) fn decode(
    input: &[u8],
    options: &DecodeOptions,
    workspace: &mut Htj2kDecodeWorkspace,
) -> Result<Option<Image>> {
    let Some(prepared) = prepare(input, options)? else {
        return Ok(None);
    };
    let decoded = codestream::decode_prepared_htj2k_reduced_component_owned_with_workspace(
        &prepared,
        &mut workspace.codestream,
    )
    .map_err(map_codestream_error)?;
    let components = part1_component_info(input, &options.requested_components, None)?;
    decoded_baseline_to_image_with_component_info(decoded, options, Some(components)).map(Some)
}

pub(super) fn shape(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<DecodeShape>> {
    if prepare(input, options)?.is_none() {
        return Ok(None);
    }
    let mut shape = decode_shape_from_metadata(metadata, options)?;
    shape.color_model = ColorModel::Unknown;
    Ok(Some(shape))
}
