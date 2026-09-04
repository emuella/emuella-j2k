//! Small project-authored Part 1 fixtures for native and mapped JP2 tests.
//!
//! The compact native-plane marker builder is independent of the production
//! image encoder. The regional reversible-MCT fixture instead delegates to a
//! dedicated project-owned test encoder in the codestream crate. Expected
//! samples are authored here rather than recovered from a decoder. See
//! `docs/native-planes.md` for the broader contract and standards basis.

use emuella_j2k_tier1::{
    CodeBlockDimensions, CodeBlockEncodeSpec, Subband, encode_baseline_code_block,
};

/// Deterministic classic Part 1 reversible-MCT regional fixture and its
/// independently authored RGB plane oracle.
pub struct ReversibleMctRegionFixture {
    pub width: u32,
    pub height: u32,
    pub planes: [Vec<u8>; 3],
    pub tnsot_zero: Vec<u8>,
    pub tnsot_one: Vec<u8>,
}

/// Build a five-level, 19-layer LRCP fixture with TLM, PLT and inline EPH.
///
/// Samples are generated here rather than recovered from either decoder
/// output. The dimensions cross 64-sample code-block boundaries on both axes,
/// so a non-trivial region can exclude real packet-body work.
pub fn reversible_mct_region_fixture() -> ReversibleMctRegionFixture {
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 192;
    let mut planes = [Vec::new(), Vec::new(), Vec::new()];
    for plane in &mut planes {
        plane.reserve_exact(usize::try_from(WIDTH * HEIGHT).unwrap());
    }
    let mut interleaved = Vec::with_capacity(usize::try_from(WIDTH * HEIGHT * 3).unwrap());
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let red = ((x * 13 + y * 7 + x * y * 3 + 17) & 0xff) as u8;
            let green = ((x * 5 + y * 19 + (x ^ y) * 11 + 29) & 0xff) as u8;
            let blue = ((x * 23 + y * 3 + x * y * 5 + 41) & 0xff) as u8;
            planes[0].push(red);
            planes[1].push(green);
            planes[2].push(blue);
            interleaved.extend_from_slice(&[red, green, blue]);
        }
    }
    let encode = |unspecified_tile_part_count| {
        emuella_j2k_core::codestream::encode_part1_reversible_mct_region_test_fixture(
            emuella_j2k_core::codestream::RgbU8Encode {
                width: WIDTH,
                height: HEIGHT,
                samples: &interleaved,
                stride_bytes: usize::try_from(WIDTH * 3).unwrap(),
            },
            unspecified_tile_part_count,
        )
        .unwrap()
    };
    ReversibleMctRegionFixture {
        width: WIDTH,
        height: HEIGHT,
        planes,
        tnsot_zero: encode(true),
        tnsot_one: encode(false),
    }
}

/// Build a single-block-per-component, no-MCT codestream from authored planes.
///
/// Dimensions are 1–64 on each axis and there are 1–4 unsigned 8-bit planes.
/// This fixture helper deliberately does not extend any production encode API.
pub fn codestream(width: u16, height: u16, planes: &[&[u8]]) -> Vec<u8> {
    assert!((1..=64).contains(&width) && (1..=64).contains(&height));
    assert!((1..=4).contains(&planes.len()));
    let mut packets = Vec::new();
    for plane in planes {
        assert_eq!(plane.len(), usize::from(width) * usize::from(height));
        if plane.iter().all(|sample| *sample == 128) {
            packets.push(0); // Empty packet: all LL coefficients are zero.
            continue;
        }
        let coefficients = plane
            .iter()
            .map(|v| i32::from(*v) - 128)
            .collect::<Vec<_>>();
        let mut body = Vec::new();
        let block = encode_baseline_code_block(
            &coefficients,
            CodeBlockEncodeSpec {
                dimensions: CodeBlockDimensions::new(width, height).unwrap(),
                subband: Subband::LowLow,
                available_bitplanes: 8,
                code_block_style: 0,
            },
            &mut body,
        )
        .unwrap();
        let mut bits = vec![true, true]; // Non-empty packet, included sole leaf.
        bits.extend(std::iter::repeat_n(
            false,
            usize::from(block.missing_bitplanes),
        ));
        bits.push(true);
        match block.pass_count {
            1 => bits.push(false),
            2 => bits.extend([true, false]),
            3..=5 => push_bits(&mut bits, u32::from(block.pass_count + 9), 4),
            6..=36 => {
                push_bits(&mut bits, 15, 4);
                push_bits(&mut bits, u32::from(block.pass_count - 6), 5);
            }
            _ => panic!("8-bit fixture needs at most 22 coding passes"),
        }
        let mut length_bits = 3 + block.pass_count.ilog2();
        while body.len() >= 1 << length_bits {
            bits.push(true);
            length_bits += 1;
        }
        bits.push(false);
        push_bits(&mut bits, body.len().try_into().unwrap(), length_bits);
        packets.extend(pack_bits(&bits));
        packets.extend(body);
    }
    from_packets(
        u32::from(width),
        u32::from(height),
        planes.len() as u16,
        &packets,
    )
}

/// Build literal empty packets. No image or entropy encoder is invoked.
pub fn empty_codestream(width: u32, height: u32, components: u16) -> Vec<u8> {
    assert!((1..=4).contains(&components));
    from_packets(width, height, components, &vec![0; usize::from(components)])
}

/// Append a final packet with an invalid MQ segment after valid earlier planes.
/// Earlier packets remain valid, so reconstruction can fail after earlier
/// planes have completed. This is deliberately malformed Tier-1 input.
pub fn late_invalid_segment(width: u16, height: u16, preceding: &[&[u8]]) -> Vec<u8> {
    assert!(preceding.len() < 4);
    let raw = codestream(width, height, preceding);
    let sod = raw
        .windows(2)
        .position(|bytes| bytes == [255, 147])
        .unwrap();
    let mut packets = raw[sod + 2..raw.len() - 2].to_vec();
    packets.extend([0xe2, 0xff, 0x90]); // One pass, two bytes with an invalid MQ marker.
    from_packets(
        u32::from(width),
        u32::from(height),
        preceding.len() as u16 + 1,
        &packets,
    )
}

fn from_packets(width: u32, height: u32, components: u16, packets: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0xff, 0x4f];
    let mut siz = vec![0, 0];
    for field in [width, height, 0, 0, width, height, 0, 0] {
        siz.extend(field.to_be_bytes());
    }
    siz.extend(components.to_be_bytes());
    for _ in 0..components {
        siz.extend([7, 1, 1]);
    }
    marker(&mut bytes, 0x51, &siz);
    marker(&mut bytes, 0x52, &[0, 0, 0, 1, 0, 0, 4, 4, 0, 1]);
    marker(&mut bytes, 0x5c, &[0x20, 0x40]); // One guard bit; LL exponent eight.
    let mut sot = vec![0, 0];
    sot.extend(u32::try_from(14 + packets.len()).unwrap().to_be_bytes());
    sot.extend([0, 1]);
    marker(&mut bytes, 0x90, &sot);
    bytes.extend([0xff, 0x93]);
    bytes.extend(packets);
    bytes.extend([0xff, 0xd9]);
    bytes
}

fn marker(bytes: &mut Vec<u8>, code: u8, payload: &[u8]) {
    bytes.extend([0xff, code]);
    bytes.extend(u16::try_from(payload.len() + 2).unwrap().to_be_bytes());
    bytes.extend(payload);
}

fn push_bits(bits: &mut Vec<bool>, value: u32, count: u32) {
    bits.extend((0..count).rev().map(|shift| value & (1 << shift) != 0));
}

fn pack_bits(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut remaining = bits;
    loop {
        let capacity = if bytes.last() == Some(&255) { 7 } else { 8 };
        let count = remaining.len().min(capacity);
        let mut byte = 0;
        for (index, bit) in remaining[..count].iter().enumerate() {
            byte |= u8::from(*bit) << (capacity - 1 - index);
        }
        bytes.push(byte);
        remaining = &remaining[count..];
        if remaining.is_empty() && byte != 255 {
            return bytes;
        }
    }
}

/// Wrap a native fixture in a JP2 with caller-authored optional header boxes.
/// `colour` is an enumerated colourspace (16 for sRGB, 17 for greyscale).
pub fn jp2(
    raw: &[u8],
    width: u32,
    height: u32,
    components: u16,
    colour: u32,
    extra: &[u8],
) -> Vec<u8> {
    let mut bytes = jp2_box(*b"jP  ", &[13, 10, 135, 10]);
    bytes.extend(jp2_box(*b"ftyp", b"jp2 \0\0\0\0jp2 "));
    let mut ihdr = Vec::new();
    ihdr.extend(height.to_be_bytes());
    ihdr.extend(width.to_be_bytes());
    ihdr.extend(components.to_be_bytes());
    ihdr.extend([7, 7, 0, 0]);
    let mut header = jp2_box(*b"ihdr", &ihdr);
    let mut colr = vec![1, 0, 0];
    colr.extend(colour.to_be_bytes());
    header.extend(jp2_box(*b"colr", &colr));
    header.extend(extra);
    bytes.extend(jp2_box(*b"jp2h", &header));
    bytes.extend(jp2_box(*b"jp2c", raw));
    bytes
}

/// Build one ordinary JP2 box for authored metadata fixtures.
pub fn jp2_box(kind: [u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut bytes = u32::try_from(payload.len() + 8)
        .unwrap()
        .to_be_bytes()
        .to_vec();
    bytes.extend(kind);
    bytes.extend(payload);
    bytes
}
