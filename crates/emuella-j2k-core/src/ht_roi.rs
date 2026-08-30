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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ht_roi_public_metadata_executor_container_and_atomicity_agree() {
        let bytes =
            codestream::encode_htj2k_roi_window_test_fixture(259, 263, 4, true, 7, 4).unwrap();
        let request = PartialDecodeOptions {
            region: Some(Region {
                x: 3,
                y: 5,
                width: 17,
                height: 19,
            }),
            components: ComponentSelection::Indices(vec![0]),
            target_layout: ComponentLayout::Planar,
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&bytes, &request).unwrap();
        let owned = decode_partial(&bytes, &request).unwrap();
        assert_eq!(owned.info, info);
        assert_eq!(
            decode_partial_component_info(&bytes, &request).unwrap(),
            owned.component_info
        );
        assert_eq!(
            info.sample_format,
            SampleFormat::with_byte_order(4, true, None).unwrap()
        );
        let ImageData::Planes(expected) = owned.data else {
            panic!("planar");
        };
        let mut caller = vec![0xa6; 23 * 19];
        {
            let mut planes = [PlaneMut::new(&mut caller, 17, 19, 23, info.sample_format).unwrap()];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_partial_into(&bytes, &mut target, &request).unwrap();
        }
        for (row, pixels) in expected[0].chunks_exact(17).enumerate() {
            assert_eq!(&caller[row * 23..row * 23 + 17], pixels);
            assert!(
                caller[row * 23 + 17..(row + 1) * 23]
                    .iter()
                    .all(|b| *b == 0xa6)
            );
        }
        let metadata = inspect(&bytes, &InspectOptions::default()).unwrap();
        assert!(matches!(
            metadata.support,
            SupportStatus::Unsupported { .. }
        ));
        assert!(super::super::decode(&bytes, &DecodeOptions::default()).is_err());
        let mut rejects = Vec::new();
        let mut jph = Vec::new();
        write_jph_encode_output(metadata.image.as_ref().unwrap(), &bytes, &mut jph).unwrap();
        rejects.push((jph, request.clone()));
        let parsed = codestream::parse(&bytes).unwrap();
        let tile = parsed
            .tiles
            .iter()
            .rfind(|p| p.tile_part_index == 0)
            .unwrap();
        let mut late = bytes.clone();
        late[tile.payload_offset.unwrap() + tile.payload_len.unwrap() - 1] = 0xff;
        rejects.push((late, request.clone()));
        // Corrupt entropy after legal packet preparation: caller publication
        // must remain atomic even when metadata preparation succeeds.
        let tile = parsed.tiles[0];
        let mut entropy = bytes.clone();
        let start = tile.payload_offset.unwrap();
        let end = start + tile.payload_len.unwrap();
        // The first packet's EPH locates its synthetic body.
        let body = start
            + bytes[start..end]
                .windows(2)
                .position(|p| p == [0xff, 0x92])
                .unwrap()
            + 2;
        let next_packet = body
            + bytes[body..end]
                .windows(2)
                .position(|p| p == [0xff, 0x91])
                .unwrap();
        entropy[next_packet - 2..next_packet].copy_from_slice(&[0xff, 0xff]);
        assert!(decode_partial_info(&entropy, &request).is_ok());
        assert!(decode_partial(&entropy, &request).is_err());
        let mut untouched = vec![0xa6; 23 * 19];
        {
            let mut planes =
                [PlaneMut::new(&mut untouched, 17, 19, 23, info.sample_format).unwrap()];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_partial_into(&entropy, &mut target, &request).is_err());
        }
        assert!(untouched.iter().all(|b| *b == 0xa6));
        // A different request shape cannot inherit the native window route.
        for changed in [
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..request.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::All,
                ..request.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(1),
                ..request.clone()
            },
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..request.clone()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 127,
                    y: 0,
                    width: 17,
                    height: 19,
                }),
                ..request.clone()
            },
        ] {
            rejects.push((bytes.clone(), changed));
        }
        for (input, options) in rejects {
            assert!(decode_partial_info(&input, &options).is_err());
            assert!(decode_partial_component_info(&input, &options).is_err());
            assert!(decode_partial(&input, &options).is_err());
            // Into chooses layout from its target; skip the interleaved option
            // here because the planar target deliberately overrides it.
            if options.target_layout == ComponentLayout::Interleaved {
                continue;
            }
            let mut caller = vec![0xa6; 23 * 19];
            {
                let mut planes =
                    [PlaneMut::new(&mut caller, 17, 19, 23, info.sample_format).unwrap()];
                let mut target = ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                };
                assert!(decode_partial_into(&input, &mut target, &options).is_err());
            }
            assert!(caller.iter().all(|b| *b == 0xa6));
        }
    }
}
