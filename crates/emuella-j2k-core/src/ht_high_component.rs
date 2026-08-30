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

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> DecodeOptions {
        DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::Indices(vec![0]),
            target_layout: ComponentLayout::Planar,
            ..DecodeOptions::default()
        }
    }

    #[test]
    fn high_component_public_metadata_execution_and_atomicity_agree() {
        let bytes = codestream::encode_htj2k_high_component_test_fixture(17, 29, 257).unwrap();
        let options = request();
        let metadata = inspect(&bytes, &InspectOptions::default()).unwrap();
        assert!(matches!(
            metadata.support,
            SupportStatus::Unsupported { .. }
        ));
        let shape = decode_shape(&bytes, &options).unwrap();
        assert_eq!(
            (shape.codestream_components, shape.output_components),
            (257, 1)
        );
        let info = shape.image_info().unwrap();
        let image = super::super::decode(&bytes, &options).unwrap();
        assert_eq!(image.info, info);
        assert_eq!(image.component_info.len(), 1);
        assert_eq!(image.component_info[0].source_component, Some(0));
        let with_workspace =
            decode_htj2k_with_workspace(&bytes, &options, &mut Htj2kDecodeWorkspace::new())
                .unwrap()
                .unwrap();
        assert_eq!(with_workspace, image);
        let ImageData::Planes(expected) = image.data else {
            panic!("planar");
        };
        let mut caller = vec![0xa6; 23 * 29];
        {
            let mut planes = [PlaneMut::new(&mut caller, 17, 29, 23, info.sample_format).unwrap()];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_into(&bytes, &mut target, &options).unwrap();
        }
        for (r, row) in expected[0].chunks_exact(17).enumerate() {
            assert_eq!(&caller[r * 23..r * 23 + 17], row);
            assert!(caller[r * 23 + 17..(r + 1) * 23].iter().all(|b| *b == 0xa6));
        }
        let mut rejects = Vec::new();
        for resolution in [
            ResolutionLevel::Full,
            ResolutionLevel::Reduced { discard_levels: 1 },
        ] {
            let partial = PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![0]),
                resolution,
                ..PartialDecodeOptions::default()
            };
            assert!(decode_partial_info(&bytes, &partial).is_err());
            assert!(decode_partial_component_info(&bytes, &partial).is_err());
            assert!(decode_partial(&bytes, &partial).is_err());
        }
        for other in [
            DecodeOptions::default(),
            DecodeOptions {
                requested_components: ComponentSelection::Indices(vec![256]),
                ..options.clone()
            },
            DecodeOptions {
                max_quality_layers: Some(1),
                ..options.clone()
            },
            DecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..options.clone()
            },
            DecodeOptions {
                allow_best_effort_backend_decode: true,
                ..options.clone()
            },
        ] {
            assert!(decode_shape(&bytes, &other).is_err());
            assert!(super::super::decode(&bytes, &other).is_err());
        }
        let mut jph = Vec::new();
        write_jph_encode_output(metadata.image.as_ref().unwrap(), &bytes, &mut jph).unwrap();
        rejects.push(jph);
        let parsed = codestream::parse(&bytes).unwrap();
        let qcc = parsed
            .markers
            .iter()
            .rfind(|m| m.marker == codestream::Marker::Qcc)
            .unwrap();
        let mut quantiser = bytes.clone();
        quantiser[qcc.data_offset + 2] = 3;
        rejects.push(quantiser);
        let mut late = bytes.clone();
        late.remove(
            parsed.tiles[0].payload_offset.unwrap() + parsed.tiles[0].payload_len.unwrap() - 1,
        );
        let sot = parsed
            .markers
            .iter()
            .find(|m| m.marker == codestream::Marker::Sot)
            .unwrap()
            .data_offset;
        let length = u32::from_be_bytes(bytes[sot + 2..sot + 6].try_into().unwrap()) - 1;
        late[sot + 2..sot + 6].copy_from_slice(&length.to_be_bytes());
        rejects.push(late);
        for bad in rejects {
            assert!(decode_shape(&bad, &options).is_err());
            assert!(super::super::decode(&bad, &options).is_err());
            let mut untouched = vec![0xa6; 23 * 29];
            {
                let mut planes =
                    [PlaneMut::new(&mut untouched, 17, 29, 23, info.sample_format).unwrap()];
                let mut target = ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                };
                assert!(decode_into(&bad, &mut target, &options).is_err());
            }
            assert!(untouched.iter().all(|b| *b == 0xa6));
        }
        let entropy =
            codestream::encode_htj2k_high_component_entropy_failure_test_fixture().unwrap();
        let info = decode_shape(&entropy, &options)
            .unwrap()
            .image_info()
            .unwrap();
        assert!(super::super::decode(&entropy, &options).is_err());
        let mut untouched = [0xa6; 3];
        {
            let mut planes = [PlaneMut::new(&mut untouched, 1, 1, 3, info.sample_format).unwrap()];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_into(&entropy, &mut target, &options).is_err());
        }
        assert_eq!(untouched, [0xa6; 3]);
    }

    #[test]
    fn high_component_singleht_invalidity_has_public_route_parity() {
        let bytes = codestream::encode_htj2k_high_component_multiple_set_test_fixture().unwrap();
        let p = codestream::parse(&bytes).unwrap();
        let cap = p
            .markers
            .iter()
            .find(|m| m.marker == codestream::Marker::Cap)
            .unwrap()
            .data_offset;
        let rgn = p
            .markers
            .iter()
            .find(|m| m.marker == codestream::Marker::Rgn)
            .unwrap()
            .data_offset;
        let qcc = p
            .markers
            .iter()
            .rfind(|m| m.marker == codestream::Marker::Qcc)
            .unwrap()
            .data_offset;
        let valid = codestream::encode_htj2k_high_component_test_fixture(1, 1, 257).unwrap();
        let info = decode_shape(&valid, &request())
            .unwrap()
            .image_info()
            .unwrap();
        for (ccap, shift, exponent) in [
            (0x100a_u16, 3, 8),
            (0x100b, 3, 8),
            (0x100a, 21, 8),
            (0x100a, 22, 8),
            (0x100a, 37, 8),
            (0x100a, 3, 31),
            (0x100a, 37, 31),
        ] {
            let mut bad = bytes.clone();
            bad[cap + 4..cap + 6].copy_from_slice(&ccap.to_be_bytes());
            bad[rgn + 3] = shift;
            bad[qcc + 3] = exponent << 3;
            for result in [
                inspect(&bad, &InspectOptions::default()).map(|_| ()),
                decode_shape(&bad, &request()).map(|_| ()),
                super::super::decode(&bad, &request()).map(|_| ()),
                decode_htj2k_with_workspace(&bad, &request(), &mut Htj2kDecodeWorkspace::new())
                    .map(|_| ()),
            ] {
                assert!(
                    matches!(result, Err(J2kError::InvalidInput { .. })),
                    "{result:?}"
                );
            }
            let mut untouched = [0xa6; 3];
            {
                let mut planes =
                    [PlaneMut::new(&mut untouched, 1, 1, 3, info.sample_format).unwrap()];
                let mut target = ImageViewMut::Planar {
                    info: &info,
                    planes: &mut planes,
                };
                assert!(matches!(
                    decode_into(&bad, &mut target, &request()),
                    Err(J2kError::InvalidInput { .. })
                ));
            }
            assert_eq!(untouched, [0xa6; 3]);
        }
    }
}
