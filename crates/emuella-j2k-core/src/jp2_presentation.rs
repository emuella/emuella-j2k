//! Bounded JP2 channel projection after independent native reconstruction.
//!
//! See docs/jp2-presentation.md for the standards basis and engineering bounds.

use crate::*;
use alloc::vec;

#[derive(Clone, Copy)]
struct Channel {
    source: usize,
    column: Option<usize>,
    format: container::ComponentSampleFormat,
}

pub(crate) struct Plan<'a> {
    codestream: &'a [u8],
    width: u32,
    height: u32,
    native_count: u16,
    channels: Vec<Channel>,
    colours: Vec<usize>,
    alpha: Option<usize>,
    palette: &'a [u8],
    palette_columns: usize,
    palette_entries: usize,
}

fn declined(detail: &'static str) -> J2kError {
    unsupported(UnsupportedFeature::ContainerBox, detail)
}

fn invalid(detail: &'static str) -> J2kError {
    J2kError::InvalidInput {
        offset: None,
        message: detail.into(),
    }
}

fn payload<'a>(input: &'a [u8], record: &container::BoxRecord) -> Result<&'a [u8]> {
    input
        .get(
            record.data_offset
                ..record
                    .data_offset
                    .checked_add(record.data_len)
                    .ok_or_else(sample_size_overflow)?,
        )
        .ok_or_else(|| invalid("JP2 presentation box exceeds input"))
}

fn word(bytes: &[u8]) -> usize {
    usize::from(u16::from_be_bytes([bytes[0], bytes[1]]))
}

/// Consume structurally validated metadata only. Native packet samples are not
/// examined during planning, so out-of-table indices can still fail at decode.
pub(crate) fn prepare<'a>(
    input: &'a [u8],
    container: &container::Container,
    primary: Option<(&'a [u8], &codestream::Codestream)>,
) -> Result<Option<Plan<'a>>> {
    if container.kind != container::ContainerKind::Jp2 {
        return Ok(None);
    }
    let Some(header) = container
        .boxes
        .iter()
        .find(|b| b.box_type == container::boxes::JP2_HEADER)
    else {
        return Ok(None);
    };
    let end = header
        .data_offset
        .checked_add(header.data_len)
        .ok_or_else(sample_size_overflow)?;
    let children = || {
        container
            .boxes
            .iter()
            .filter(|b| b.header_offset >= header.data_offset && b.header_offset < end)
    };
    let palette_box = children().find(|b| b.box_type == container::boxes::PALETTE);
    let mapping_box = children().find(|b| b.box_type == container::boxes::COMPONENT_MAPPING);
    let definition_box = children().find(|b| b.box_type == container::boxes::CHANNEL_DEFINITION);
    if palette_box.is_none() && mapping_box.is_none() && definition_box.is_none() {
        return Ok(None);
    }
    if container.codestreams.len() != 1 {
        return Err(declined(
            "mapped JP2 presentation requires exactly one codestream",
        ));
    }
    let colour_count = match container.color_specification {
        Some(container::ColorSpecificationBox {
            method: container::ColorSpecificationMethod::Enumerated,
            enumerated_color_space: Some(container::EnumeratedColorSpace::Greyscale),
            ..
        }) => 1,
        Some(container::ColorSpecificationBox {
            method: container::ColorSpecificationMethod::Enumerated,
            enumerated_color_space: Some(container::EnumeratedColorSpace::SRgb),
            ..
        }) => 3,
        _ => {
            return Err(declined(
                "mapped JP2 presentation requires enumerated greyscale or sRGB; ICC and other colourspaces are unsupported",
            ));
        }
    };
    if children()
        .filter(|b| b.box_type == container::boxes::COLOR_SPECIFICATION)
        .count()
        != 1
    {
        return Err(declined(
            "mapped JP2 presentation admits one colour description only; later descriptions never replace the first",
        ));
    }
    let formats = container
        .component_sample_formats()
        .ok_or_else(sample_size_overflow)?;
    let mut palette = &[][..];
    let mut palette_entries = 0;
    let mut palette_columns = 0;
    let mut palette_formats = &[][..];
    if let Some(record) = palette_box {
        let bytes = payload(input, record)?;
        palette_entries = word(bytes);
        palette_columns = usize::from(bytes[2]);
        palette_formats = &bytes[3..3 + palette_columns];
        palette = &bytes[3 + palette_columns..];
    }
    let channels = if let Some(record) = mapping_box {
        if record.data_len / 4 > 4 {
            return Err(declined(
                "mapped JP2 presentation admits at most four logical channels",
            ));
        }
        payload(input, record)?
            .chunks_exact(4)
            .map(|entry| {
                let source = word(entry);
                let column = (entry[2] == 1).then_some(usize::from(entry[3]));
                let format = if let Some(column) = column {
                    let value = palette_formats[column];
                    container::ComponentSampleFormat {
                        bits_per_sample: (value & 127) + 1,
                        signed: value & 128 != 0,
                    }
                } else {
                    formats[source]
                };
                Channel {
                    source,
                    column,
                    format,
                }
            })
            .collect::<Vec<_>>()
    } else {
        if formats.len() > 4 {
            return Err(declined(
                "mapped JP2 presentation admits at most four logical channels",
            ));
        }
        formats
            .iter()
            .enumerate()
            .map(|(source, format)| Channel {
                source,
                column: None,
                format: *format,
            })
            .collect()
    };
    let mut colours = vec![None; colour_count];
    let mut alpha = None;
    if let Some(record) = definition_box {
        for entry in payload(input, record)?[2..].chunks_exact(6) {
            let channel = word(entry);
            match (word(&entry[2..]), word(&entry[4..])) {
                (0, association) => {
                    if channels[channel].format.signed {
                        return Err(invalid(
                            "mapped greyscale and sRGB colour channels require unsigned samples",
                        ));
                    }
                    colours[association - 1] = Some(channel);
                }
                (1, 0) => alpha = Some(channel),
                _ => {
                    return Err(declined(
                        "mapped JP2 presentation supports colour roles and one whole-image straight opacity channel only",
                    ));
                }
            }
        }
    } else {
        for (index, channel) in channels.iter().enumerate() {
            if channel.format.signed {
                return Err(invalid(
                    "mapped greyscale and sRGB colour channels require unsigned samples",
                ));
            }
            colours[index] = Some(index);
        }
    }
    let colours = colours
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| invalid("mapped JP2 presentation is missing a required colour"))?;
    if channels
        .iter()
        .any(|channel| channel.format.signed || channel.format.bits_per_sample != 8)
        || palette_formats.iter().any(|format| *format != 7)
    {
        return Err(declined(
            "mapped JP2 presentation requires unsigned 8-bit channels and palette columns",
        ));
    }
    let (bytes, parsed) =
        primary.ok_or_else(|| declined("mapped JP2 presentation needs a Part 1 codestream"))?;
    if !native_planes::is_atomic_profile(bytes, parsed) {
        return Err(declined(
            "mapped JP2 presentation requires the bounded independent U8 zero-decomposition Part 1 profile",
        ));
    }
    let output_count = if alpha.is_some() { 4 } else { colour_count };
    // Expanded output is bounded separately from the native-index input. This
    // preflight runs before packet reconstruction or image-sample allocation.
    if u64::from(parsed.image_width()) * u64::from(parsed.image_height()) * output_count as u64
        > 16 * 1024 * 1024
    {
        return Err(declined(
            "mapped JP2 presentation exceeds 16 Mi expanded output samples",
        ));
    }
    Ok(Some(Plan {
        codestream: bytes,
        width: parsed.image_width(),
        height: parsed.image_height(),
        native_count: parsed.siz.component_count(),
        channels,
        colours,
        alpha,
        palette,
        palette_entries,
        palette_columns,
    }))
}

pub(crate) fn for_request<'a>(
    input: &'a [u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<Plan<'a>>> {
    if metadata.format != InputFormat::Jp2 || options.mode != DecodeMode::Rendered {
        return Ok(None);
    }
    let container = container::parse(input).map_err(map_container_error)?;
    let primary = container
        .primary_codestream(input)
        .map_err(map_container_error)?;
    let parsed = primary
        .map(codestream::parse)
        .transpose()
        .map_err(map_codestream_error)?;
    prepare(input, &container, primary.zip(parsed.as_ref()))
}

impl Plan<'_> {
    pub(crate) fn shape(&self, layout: ComponentLayout) -> DecodeShape {
        let count = if self.alpha.is_some() {
            4
        } else {
            self.colours.len() as u16
        };
        DecodeShape {
            width: self.width,
            height: self.height,
            codestream_components: self.native_count,
            colour_channels: count,
            output_components: count,
            sample_format: SampleFormat::U8,
            layout,
            byte_order: None,
            color_model: if self.alpha.is_some() {
                ColorModel::Rgba
            } else if count == 1 {
                ColorModel::Grayscale
            } else {
                ColorModel::Rgb
            },
            mode: DecodeMode::Rendered,
        }
    }

    pub(crate) fn decode(&self, options: &DecodeOptions) -> Result<Image> {
        let native = codestream::decode_baseline_owned_components(self.codestream)
            .map_err(map_codestream_error)?;
        let pixels = usize::try_from(u64::from(self.width) * u64::from(self.height))
            .map_err(|_| sample_size_overflow())?;
        if native.bits_per_sample != 8
            || native.signed
            || native.width != self.width
            || native.height != self.height
            || native.components.len() != usize::from(self.native_count)
            || native
                .components
                .iter()
                .any(|plane| plane.samples.len() != pixels)
        {
            return Err(J2kError::InternalInvariant {
                message: "mapped JP2 native reconstruction disagrees with its plan".into(),
            });
        }
        let mut outputs = self.colours.clone();
        if let Some(alpha) = self.alpha {
            if outputs.len() == 1 {
                outputs.resize(3, outputs[0]);
            }
            outputs.push(alpha);
        }
        let mut planes = Vec::with_capacity(outputs.len());
        for channel_index in outputs {
            let channel = self.channels[channel_index];
            let source = &native.components[channel.source].samples;
            let plane = if let Some(column) = channel.column {
                source
                    .iter()
                    .map(|index| {
                        let row = usize::from(*index);
                        if row >= self.palette_entries {
                            return Err(declined(
                                "palette index is outside the table; rendered sample is indeterminate (no clamping or substitution)",
                            ));
                        }
                        Ok(self.palette[row * self.palette_columns + column])
                    })
                    .collect::<Result<Vec<_>>>()?
            } else {
                source.clone()
            };
            planes.push(plane);
        }
        let info = self.shape(options.target_layout).image_info()?;
        let component_info = uniform_component_info(&info, false);
        let data = match options.target_layout {
            ComponentLayout::Planar => ImageData::Planes(planes),
            ComponentLayout::Interleaved => ImageData::Interleaved(interleave_planes(
                &planes,
                self.width,
                self.height,
                SampleFormat::U8,
            )?),
        };
        Ok(Image {
            info,
            component_info,
            data,
        })
    }
}
