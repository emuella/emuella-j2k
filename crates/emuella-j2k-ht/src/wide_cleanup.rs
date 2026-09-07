//! Additive caller-owned magnitude storage for the direct cleanup path.
use super::*;
mod sealed {
    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}
/// Explicit magnitude/sign storage supported by direct cleanup decoding.
/// Existing `u16` callers retain their checked 16-bit boundary. `u32` storage
/// admits up to 18 explicit bits for the existing 17-bit transformed-magnitude
/// profile. Implementations are sealed to preserve these admission bounds.
pub trait HtCleanupMagnitude: sealed::Sealed + Copy + Default + Into<u32> {
    const MAX_BITS: u16;
    fn from_u32(value: u32) -> Result<Self, HtLayoutError>;
    fn reconstruct(
        output: HtVlcCleanupCoefficientOutput<Self>,
        missing_msbs: u8,
    ) -> Result<i32, HtLayoutError>;
}
impl HtCleanupMagnitude for u16 {
    const MAX_BITS: u16 = 16;
    fn from_u32(value: u32) -> Result<Self, HtLayoutError> {
        Self::try_from(value).map_err(|_| HtLayoutError::SizeOverflow)
    }
    fn reconstruct(
        output: HtVlcCleanupCoefficientOutput<Self>,
        missing_msbs: u8,
    ) -> Result<i32, HtLayoutError> {
        output.reconstruct_cleanup_coefficient(missing_msbs)
    }
}
impl HtCleanupMagnitude for u32 {
    const MAX_BITS: u16 = 18;
    fn from_u32(value: u32) -> Result<Self, HtLayoutError> {
        Ok(value)
    }
    fn reconstruct(
        output: HtVlcCleanupCoefficientOutput<Self>,
        missing_msbs: u8,
    ) -> Result<i32, HtLayoutError> {
        // This follows the project-authored scalar direct reconstruction in
        // lib.rs, with the same checked midpoint, shift and signed range.
        let shift = cleanup_bitplane_from_missing_msbs(missing_msbs)?
            .checked_sub(1)
            .ok_or(HtLayoutError::SizeOverflow)?;
        if output.magnitude_sign_bits > Self::MAX_BITS {
            return Err(HtLayoutError::SizeOverflow);
        }
        let mask = (1_u32 << output.magnitude_sign_bits) - 1;
        if output.magnitude_sign_value & !mask != 0 {
            return Err(HtLayoutError::InvalidVlcCleanupOutput {
                reason: "magnitude/sign value exceeds its declared width",
            });
        }
        if !output.significant {
            if output.magnitude_sign_bits != 0
                || output.magnitude_sign_value != 0
                || output.embedded_magnitude_bit
                || output.magnitude_exponent_reduction
            {
                return Err(HtLayoutError::InvalidVlcCleanupOutput {
                    reason: "insignificant coefficient declares magnitude/sign data",
                });
            }
            return Ok(0);
        }
        let code = output.magnitude_sign_value
            | (u32::from(output.embedded_magnitude_bit) << output.magnitude_sign_bits)
            | 1;
        let magnitude = u64::from(code)
            .checked_add(2)
            .and_then(|v| v.checked_shl(u32::from(shift)))
            .and_then(|v| i32::try_from(v).ok())
            .ok_or(HtLayoutError::SizeOverflow)?;
        if output.magnitude_sign_value & 1 != 0 {
            magnitude.checked_neg().ok_or(HtLayoutError::SizeOverflow)
        } else {
            Ok(magnitude)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wide_reads_preserve_high_bits_and_legacy_width_checks() {
        let block = HtBlockLayout::new(HtCodeBlockDimensions::new(1, 1).unwrap());
        let position = block.coefficient_position(0, 0).unwrap();
        for bits in [17, 18] {
            for bytes in [[0x54, 0x76, 0x03], [0x55, 0x76, 0x03]] {
                let plan = HtVlcQuadCoefficientDecodePlan {
                    position,
                    significant: true,
                    magnitude_sign_bits: bits,
                    embedded_magnitude_bit: false,
                    magnitude_exponent_reduction: false,
                };
                let mut legacy = HtForwardBitCursor::new(&bytes);
                assert!(
                    read_required_vlc_cleanup_coefficient_from_segment_magnitude_sign::<u16>(
                        &mut legacy,
                        plan
                    )
                    .is_err()
                );
                assert_eq!(legacy.consumed_bits(), 0);
                let mut wide = HtForwardBitCursor::new(&bytes);
                let output =
                    read_required_vlc_cleanup_coefficient_from_segment_magnitude_sign::<u32>(
                        &mut wide, plan,
                    )
                    .unwrap();
                let packed =
                    u32::from(bytes[0]) | (u32::from(bytes[1]) << 8) | (u32::from(bytes[2]) << 16);
                assert_eq!(output.magnitude_sign_value, packed & ((1 << bits) - 1));
                assert!(output.magnitude_sign_value > u16::MAX as u32);
                assert_eq!(wide.consumed_bits(), bits as usize);
                let missing = 29;
                let shift = cleanup_bitplane_from_missing_msbs(missing).unwrap() - 1;
                let magnitude = ((u64::from(output.magnitude_sign_value | 1) + 2) << shift) as i32;
                assert_eq!(
                    u32::reconstruct(output, missing).unwrap(),
                    if bytes[0] & 1 != 0 {
                        -magnitude
                    } else {
                        magnitude
                    }
                );
            }
        }
    }
    #[test]
    fn wide_materialisation_checks_ranges_before_mutating_output() {
        let block = HtBlockLayout::new(HtCodeBlockDimensions::new(1, 1).unwrap());
        let seed = HtVlcCleanupCoefficientOutput::<u32> {
            position: block.coefficient_position(0, 0).unwrap(),
            significant: true,
            magnitude_sign_bits: 18,
            magnitude_sign_value: (1 << 18) - 1,
            embedded_magnitude_bit: true,
            magnitude_exponent_reduction: false,
        };
        for (record, missing) in [
            (seed, 0),
            (
                HtVlcCleanupCoefficientOutput {
                    magnitude_sign_bits: 19,
                    ..seed
                },
                29,
            ),
            (
                HtVlcCleanupCoefficientOutput {
                    magnitude_sign_value: 1 << 18,
                    ..seed
                },
                29,
            ),
            (
                HtVlcCleanupCoefficientOutput {
                    significant: false,
                    ..seed
                },
                29,
            ),
        ] {
            let mut output = [123_i32];
            assert!(
                block
                    .materialize_vlc_cleanup_coefficient_outputs(&[record], missing, &mut output)
                    .is_err()
            );
            assert_eq!(output, [123]);
        }
    }
}
