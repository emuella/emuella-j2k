//! Project-authored sample patterns shared by calibration and public qualification.
use alloc::vec::Vec;

pub fn source(width: u32, height: u32, bits: u8, components: u16, pattern: u32) -> Vec<Vec<u16>> {
    let max = (1_u32 << bits) - 1;
    (0..u32::from(components))
        .map(|c| {
            (0..height)
                .flat_map(|y| {
                    (0..width).map(move |x| {
                        let mixed = x
                            .wrapping_mul(977)
                            .wrapping_add(y.wrapping_mul(1393))
                            .wrapping_add(c.wrapping_mul(9973))
                            .wrapping_add(x.wrapping_mul(y).wrapping_mul(41))
                            .wrapping_add((x ^ y).wrapping_mul(271))
                            .wrapping_add((x / 7).wrapping_mul((y / 5) + 1).wrapping_mul(613));
                        let mut noise =
                            (x + y * width + c * width * height + 1).wrapping_mul(0x9e3779b9);
                        noise ^= noise >> 16;
                        noise = noise.wrapping_mul(0x85ebca6b);
                        noise ^= noise >> 13;
                        noise = noise.wrapping_mul(0xc2b2ae35);
                        noise ^= noise >> 16;
                        let value = match pattern {
                            0 => mixed & max,
                            1 => noise & max,
                            2 => {
                                let base = if ((x / 17) + (y / 13) + c) % 2 == 0 {
                                    0
                                } else {
                                    max * 3 / 4
                                };
                                base + (noise & (max / 4))
                            }
                            3 => {
                                let base = if (x + y + c) % 2 == 0 { 0 } else { max / 2 };
                                base + (noise & (max / 2))
                            }
                            4 => {
                                if (x + y + c) % 2 == 0 {
                                    0
                                } else {
                                    max
                                }
                            }
                            5 => {
                                if x == width / 2 && y == height / 2 {
                                    max
                                } else {
                                    0
                                }
                            }
                            6 => max / 2,
                            8 => {
                                let negative = |v: u32| matches!(v % 8, 1 | 2 | 4 | 5);
                                if negative(x) == negative(y) { max } else { 0 }
                            }
                            _ => ((x + y + c * 17) * max / (width + height + 34)).min(max),
                        };
                        value as u16
                    })
                })
                .collect()
        })
        .collect()
}
