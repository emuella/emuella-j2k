//! Bounded native plane contract shared with future JP2 presentation planning.
//!
//! This selects atomic full-image publication from already supported decoding;
//! it does not grant or remove any codestream admission. See
//! `docs/native-planes.md` for bounds and independently authored evidence.

use crate::codestream::{
    Codestream, CodestreamKind, EntropyCoder, Marker, ProgressionOrder, WaveletTransform,
};

pub(crate) fn is_atomic_profile(input: &[u8], parsed: &Codestream) -> bool {
    let siz = &parsed.siz;
    let width = parsed.image_width();
    let height = parsed.image_height();
    if parsed.kind != CodestreamKind::J2k
        || !(1..=4).contains(&siz.component_count())
        || siz.image_origin_x != 0
        || siz.image_origin_y != 0
        || siz.tile_origin_x != 0
        || siz.tile_origin_y != 0
        || !(1..=32768).contains(&width)
        || !(1..=32768).contains(&height)
        || siz.tile_width < width
        || siz.tile_height < height
        || u64::from(width) * u64::from(height) * u64::from(siz.component_count())
            > 16 * 1024 * 1024
        || siz.components.iter().any(|component| {
            component.signed
                || component.bits_per_sample != 8
                || component.horizontal_separation != 1
                || component.vertical_separation != 1
        })
    {
        return false;
    }
    let [tile] = parsed.tiles.as_slice() else {
        return false;
    };
    if tile.tile_index != 0
        || tile.tile_part_index != 0
        || !matches!(tile.tile_part_count, None | Some(1))
    {
        return false;
    }
    let Some(style) = parsed.uniform_effective_coding_style() else {
        return false;
    };
    if style.entropy_coder != EntropyCoder::ClassicTier1
        || style.progression_order != ProgressionOrder::Lrcp
        || style.layers != 1
        || style.multiple_component_transform
        || style.decomposition_levels != 0
        || style.transform != WaveletTransform::Reversible53
        || style.code_block_style != 0
        || !(2..=6).contains(&style.code_block_width_exponent)
        || !(2..=6).contains(&style.code_block_height_exponent)
        || style.precincts_declared
        || style.sop_markers
        || style.eph_markers
    {
        return false;
    }
    let blocks = u64::from(width.div_ceil(1 << style.code_block_width_exponent))
        * u64::from(height.div_ceil(1 << style.code_block_height_exponent))
        * u64::from(siz.component_count());
    if blocks > 1_048_576 {
        return false;
    }
    let mut in_tile = false;
    let mut quantisation = false;
    for segment in &parsed.markers {
        match segment.marker {
            Marker::Sot => in_tile = true,
            Marker::Soc | Marker::Siz | Marker::Sod | Marker::Eoc | Marker::Com => {}
            Marker::Cod if !in_tile => {}
            Marker::Qcd if !in_tile => {
                let Some(end) = segment.data_offset.checked_add(segment.data_len) else {
                    return false;
                };
                let Some([guard, exponent]) = input.get(segment.data_offset..end) else {
                    return false;
                };
                // No quantisation, one/two guard bits, LL exponent eight.
                // Low exponent bits are reserved and do not change its value.
                if !matches!(*guard, 0x20 | 0x40) || exponent >> 3 != 8 {
                    return false;
                }
                quantisation = true;
            }
            _ => return false,
        }
    }
    quantisation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codestream;

    fn fixture(components: usize) -> Vec<u8> {
        codestream::encode_planar_u8_no_decomp_test_fixture(5, 3, &vec![&[17; 15][..]; components])
            .unwrap()
    }

    #[test]
    fn atomic_contract_is_independent_of_broader_native_support() {
        for count in 1..=4 {
            let raw = fixture(count);
            let parsed = codestream::parse(&raw).unwrap();
            assert!(is_atomic_profile(&raw, &parsed));
        }
        let five = fixture(5);
        let parsed = codestream::parse(&five).unwrap();
        assert!(!is_atomic_profile(&five, &parsed));
        assert!(codestream::decode_baseline_owned_components(&five).is_ok());

        let mut rlcp = fixture(4);
        let cod = codestream::parse(&rlcp)
            .unwrap()
            .markers
            .into_iter()
            .find(|s| s.marker == Marker::Cod)
            .unwrap();
        rlcp[cod.data_offset + 1] = 1;
        assert!(!is_atomic_profile(
            &rlcp,
            &codestream::parse(&rlcp).unwrap()
        ));
        assert!(codestream::decode_baseline_owned_components(&rlcp).is_ok());
    }

    #[test]
    fn atomic_contract_checks_markers_and_quantisation() {
        let raw = fixture(4);
        let parsed = codestream::parse(&raw).unwrap();
        let sot = parsed
            .markers
            .iter()
            .find(|s| s.marker == Marker::Sot)
            .unwrap()
            .offset;
        let cod = parsed
            .markers
            .iter()
            .find(|s| s.marker == Marker::Cod)
            .unwrap();
        let qcd = parsed
            .markers
            .iter()
            .find(|s| s.marker == Marker::Qcd)
            .unwrap();
        // Valid marker mechanisms outside this publication contract. The
        // predicate is a policy gate, not a replacement structural parser.
        for segment in [
            vec![255, 100, 0, 5, 0, 1, b'x'], // COM is harmless.
            vec![
                255, 99, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            vec![255, 83, 0, 9, 0, 0, 0, 4, 4, 0, 1], // Main COC.
            vec![255, 93, 0, 5, 0, 0x40, 0x40],       // Main QCC.
        ] {
            let mut bytes = raw.clone();
            bytes.splice(sot..sot, segment.clone());
            let parsed = codestream::parse(&bytes).unwrap();
            assert_eq!(is_atomic_profile(&bytes, &parsed), segment[1] == 100);
        }
        for (offset, values) in [
            (cod.data_offset + 8, &[1, 2, 4, 8, 16, 32][..]),
            (qcd.data_offset, &[0, 0x60, 0xe0][..]),
            (qcd.data_offset + 1, &[0x38, 0x48][..]),
        ] {
            for value in values {
                let mut bytes = raw.clone();
                bytes[offset] = *value;
                let parsed = codestream::parse(&bytes).unwrap();
                assert!(!is_atomic_profile(&bytes, &parsed));
            }
        }
        // Additional declarations cannot quietly bypass the exact allowlist.
        for marker in [
            Marker::Poc,
            Marker::Rgn,
            Marker::Ppm,
            Marker::Ppt,
            Marker::Plt,
            Marker::Tlm,
            Marker::Cap,
            Marker::Unknown(0xff70),
        ] {
            let mut changed = parsed.clone();
            let mut segment = *cod;
            segment.marker = marker;
            changed.markers.push(segment);
            assert!(!is_atomic_profile(&raw, &changed));
        }
    }

    #[test]
    fn atomic_contract_bounds_geometry_before_packet_allocation() {
        let raw = fixture(4);
        let parsed = codestream::parse(&raw).unwrap();
        for (width, height, admitted) in [
            (2048, 2048, true),
            (2049, 2048, false),
            (32768, 1, true),
            (32769, 1, false),
            (u32::MAX, u32::MAX, false),
        ] {
            let mut changed = parsed.clone();
            changed.siz.reference_grid_width = width;
            changed.siz.reference_grid_height = height;
            changed.siz.tile_width = width;
            changed.siz.tile_height = height;
            assert_eq!(is_atomic_profile(&raw, &changed), admitted);
        }
        for width in 2..=7 {
            for height in 2..=7 {
                let mut bytes = raw.clone();
                let cod = parsed
                    .markers
                    .iter()
                    .find(|s| s.marker == Marker::Cod)
                    .unwrap();
                bytes[cod.data_offset + 6] = width - 2;
                bytes[cod.data_offset + 7] = height - 2;
                if width + height > 12 {
                    continue;
                }
                let parsed = codestream::parse(&bytes).unwrap();
                assert_eq!(
                    is_atomic_profile(&bytes, &parsed),
                    width <= 6 && height <= 6
                );
            }
        }
        let mut small_blocks = raw.clone();
        let cod = parsed
            .markers
            .iter()
            .find(|s| s.marker == Marker::Cod)
            .unwrap();
        small_blocks[cod.data_offset + 6..cod.data_offset + 8].fill(0);
        let mut edge = codestream::parse(&small_blocks).unwrap();
        edge.siz.components.truncate(1);
        for (width, height, admitted) in [(4096, 4096, true), (4097, 4095, false)] {
            edge.siz.reference_grid_width = width;
            edge.siz.reference_grid_height = height;
            edge.siz.tile_width = width;
            edge.siz.tile_height = height;
            // Both fit the total sample bound; partial edge blocks must also
            // fit the independent topology bound.
            assert_eq!(is_atomic_profile(&small_blocks, &edge), admitted);
        }
    }
}
