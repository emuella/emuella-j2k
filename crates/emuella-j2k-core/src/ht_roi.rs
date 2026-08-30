//! Native partial ROI presentation, independent of the Part 1 request planner.
use super::*;

pub(super) enum NativeWindow<'a> {
    Roi(codestream::PreparedHtj2kRoiWindowDecode<'a>),
    Tile(codestream::PreparedHtj2kTileWindowDecode<'a>),
}
impl NativeWindow<'_> {
    fn bits_per_sample(&self) -> u8 {
        match self {
            Self::Roi(p) => p.bits_per_sample(),
            Self::Tile(p) => p.bits_per_sample(),
        }
    }
    fn signed(&self) -> bool {
        match self {
            Self::Roi(p) => p.signed(),
            Self::Tile(p) => p.signed(),
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn prepare<'a>(
    input: &'a [u8],
    options: &PartialDecodeOptions,
) -> Result<Option<(NativeWindow<'a>, ImageInfo, Vec<ComponentInfo>)>> {
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
    let window = codestream::TileRegionRequest {
        x: region.x,
        y: region.y,
        width: region.width,
        height: region.height,
    };
    let prepared = if let Some(p) =
        codestream::prepare_htj2k_tile_window_decode(input, window).map_err(map_codestream_error)?
    {
        NativeWindow::Tile(p)
    } else if let Some(p) =
        codestream::prepare_htj2k_roi_window_decode(input, window).map_err(map_codestream_error)?
    {
        NativeWindow::Roi(p)
    } else {
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
    let decoded = match &prepared {
        NativeWindow::Roi(p) => codestream::decode_prepared_htj2k_roi_window_owned(p),
        NativeWindow::Tile(p) => codestream::decode_prepared_htj2k_tile_window_owned(p),
    }
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
    fn ht_tile_window_public_routes_preserve_native_metadata_and_atomicity() {
        let bytes = codestream::encode_htj2k_tile_window_test_fixture(3, false).unwrap();
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
        let image = decode_partial(&bytes, &request).unwrap();
        assert_eq!(image.info, info);
        assert_eq!(
            image.component_info,
            decode_partial_component_info(&bytes, &request).unwrap()
        );
        assert_eq!(
            info.sample_format,
            SampleFormat::with_byte_order(12, true, Some(SampleEndian::Little)).unwrap()
        );
        let ImageData::Planes(expected) = image.data else {
            panic!("planar")
        };
        let write = |input: &[u8], options: &PartialDecodeOptions, caller: &mut [u8]| {
            let mut planes = [PlaneMut::new(caller, 17, 19, 40, info.sample_format).unwrap()];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_partial_into(input, &mut target, options)
        };
        let mut caller = vec![0xa6; 40 * 19];
        write(&bytes, &request, &mut caller).unwrap();
        for (row, pixels) in expected[0].chunks_exact(34).enumerate() {
            assert_eq!(&caller[row * 40..row * 40 + 34], pixels);
            assert!(
                caller[row * 40 + 34..(row + 1) * 40]
                    .iter()
                    .all(|b| *b == 0xa6)
            );
        }
        let metadata = inspect(&bytes, &InspectOptions::default()).unwrap();
        assert!(matches!(
            metadata.support,
            SupportStatus::Unsupported { .. }
        ));
        let mut jph = Vec::new();
        write_jph_encode_output(metadata.image.as_ref().unwrap(), &bytes, &mut jph).unwrap();
        let mut rejected = vec![(jph, request.clone())];
        for options in [
            PartialDecodeOptions {
                components: ComponentSelection::All,
                ..request.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..request.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(1),
                ..request.clone()
            },
            PartialDecodeOptions {
                tile: Some(TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                }),
                ..request.clone()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 31,
                    y: 0,
                    width: 17,
                    height: 19,
                }),
                ..request.clone()
            },
        ] {
            rejected.push((bytes.clone(), options));
        }
        let c = codestream::parse(&bytes).unwrap();
        let last = c.tiles.iter().find(|p| p.tile_index == 2).unwrap();
        let mut malformed = bytes.clone();
        malformed[last.payload_offset.unwrap() + last.payload_len.unwrap() - 1] = 0xff;
        rejected.push((malformed, request.clone()));
        let mut entropy = bytes.clone();
        let first = c.tiles[0].payload_offset.unwrap();
        let body = first
            + bytes[first..]
                .windows(2)
                .position(|w| w == [0xff, 0x92])
                .unwrap()
            + 2;
        let next = body
            + bytes[body..]
                .windows(2)
                .position(|w| w == [0xff, 0x91])
                .unwrap();
        entropy[next - 2..next].copy_from_slice(&[0xff, 0xff]);
        assert!(decode_partial_info(&entropy, &request).is_ok());
        assert!(decode_partial(&entropy, &request).is_err());
        let mut caller = vec![0xa6; 40 * 19];
        assert!(write(&entropy, &request, &mut caller).is_err());
        assert!(caller.iter().all(|b| *b == 0xa6));
        for (input, options) in rejected {
            assert!(decode_partial_info(&input, &options).is_err());
            assert!(decode_partial_component_info(&input, &options).is_err());
            assert!(decode_partial(&input, &options).is_err());
            caller.fill(0xa6);
            assert!(write(&input, &options, &mut caller).is_err());
            assert!(caller.iter().all(|b| *b == 0xa6));
        }
        let invalid = codestream::encode_htj2k_tile_window_test_fixture(3, true).unwrap();
        assert!(matches!(
            inspect(&invalid, &InspectOptions::default()),
            Err(J2kError::InvalidInput { .. })
        ));
        for x in [3, 31] {
            let options = PartialDecodeOptions {
                region: Some(Region {
                    x,
                    y: 5,
                    width: 17,
                    height: 19,
                }),
                ..request.clone()
            };
            assert!(matches!(
                decode_partial_info(&invalid, &options),
                Err(J2kError::InvalidInput { .. })
            ));
            assert!(matches!(
                decode_partial_component_info(&invalid, &options),
                Err(J2kError::InvalidInput { .. })
            ));
            assert!(matches!(
                decode_partial(&invalid, &options),
                Err(J2kError::InvalidInput { .. })
            ));
            caller.fill(0xa6);
            assert!(matches!(
                write(&invalid, &options, &mut caller),
                Err(J2kError::InvalidInput { .. })
            ));
            assert!(caller.iter().all(|b| *b == 0xa6));
        }
    }

    #[test]
    fn ht_roi_singleht_invalidity_precedes_native_bounds_and_window_admission() {
        // Project-authored multiple-set packets, with the bounded ROI marker
        // topology. The SINGLEHT declaration deliberately contradicts them.
        let mut bytes =
            codestream::encode_htj2k_one_decomp_two_layer_multiple_set_test_fixture().unwrap();
        let at = |input: &[u8], marker| {
            codestream::parse(input)
                .unwrap()
                .markers
                .iter()
                .find(|m| m.marker == marker)
                .unwrap()
                .offset
        };
        let cap = at(&bytes, codestream::Marker::Cap);
        bytes[cap + 8] = 0x18;
        let siz = at(&bytes, codestream::Marker::Siz);
        bytes[siz + 22..siz + 26].copy_from_slice(&128_u32.to_be_bytes());
        bytes[siz + 26..siz + 30].copy_from_slice(&128_u32.to_be_bytes());
        let cod = at(&bytes, codestream::Marker::Cod);
        bytes[cod + 2..cod + 4].copy_from_slice(&14_u16.to_be_bytes());
        bytes[cod + 4] = 1;
        bytes[cod + 5] = 3;
        bytes.splice(cod + 14..cod + 14, [0x77, 0x88]);
        let sot = at(&bytes, codestream::Marker::Sot);
        bytes.splice(sot..sot, [0xff, 0x5f, 0, 9, 0, 0, 0, 2, 33, 255, 0]);
        let sot = at(&bytes, codestream::Marker::Sot);
        let len = u32::from_be_bytes(bytes[sot + 6..sot + 10].try_into().unwrap());
        bytes[sot + 6..sot + 10].copy_from_slice(&(len + 7).to_be_bytes());
        bytes.splice(sot + 12..sot + 12, [0xff, 0x5e, 0, 5, 0, 0, 7]);
        let format = SampleFormat::with_byte_order(8, false, None).unwrap();
        let info = ImageInfo::new(
            8,
            8,
            1,
            format,
            ColorModel::Unknown,
            ComponentLayout::Planar,
        )
        .unwrap();
        for bound in [10, 11] {
            bytes[cap + 9] = bound; // Cleanup magnitude bounds eighteen/nineteen.
            for x in [0, 127] {
                let request = PartialDecodeOptions {
                    region: Some(Region {
                        x,
                        y: 0,
                        width: 8,
                        height: 8,
                    }),
                    components: ComponentSelection::Indices(vec![0]),
                    target_layout: ComponentLayout::Planar,
                    ..PartialDecodeOptions::default()
                };
                assert!(matches!(
                    inspect(&bytes, &InspectOptions::default()),
                    Err(J2kError::InvalidInput { .. })
                ));
                assert!(matches!(
                    decode_partial_info(&bytes, &request),
                    Err(J2kError::InvalidInput { .. })
                ));
                assert!(matches!(
                    decode_partial_component_info(&bytes, &request),
                    Err(J2kError::InvalidInput { .. })
                ));
                assert!(matches!(
                    decode_partial(&bytes, &request),
                    Err(J2kError::InvalidInput { .. })
                ));
                let mut caller = vec![0xa6; 64];
                {
                    let mut planes = [PlaneMut::new(&mut caller, 8, 8, 8, format).unwrap()];
                    let mut target = ImageViewMut::Planar {
                        info: &info,
                        planes: &mut planes,
                    };
                    assert!(matches!(
                        decode_partial_into(&bytes, &mut target, &request),
                        Err(J2kError::InvalidInput { .. })
                    ));
                }
                assert!(caller.iter().all(|b| *b == 0xa6));
            }
        }
    }

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
