#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::bool_assert_comparison,
    clippy::cloned_ref_to_slice_refs,
    clippy::too_many_arguments
)]
//! Public Rust API for `emuella-j2k`.
//!
//! This crate owns the stable, wrapper-ready surface for JPEG 2000 and HTJ2K
//! callers. Entrypoints accept byte slices or caller-owned buffers, route the
//! current profile-scoped Part 1 decode rows, structurally admitted
//! encode-compatible rows, and native encode
//! profiles through repo-owned Rust codec code. Profiles outside the implemented
//! milestone subset return structured `Unsupported` errors. Bounded HTJ2K/JPH
//! and prepared selective Part 1 profiles are algorithmic; narrower compatibility
//! rows remain explicitly profile-scoped or adapters.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

pub use emuella_j2k_codestream as codestream;
pub use emuella_j2k_container as container;

pub const PROJECT_NAME: &str = "emuella-j2k";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectSummary {
    pub name: &'static str,
    pub summary: &'static str,
}

#[cfg(test)]
mod jp2_header_validation_tests {
    use super::*;

    fn codestream(components: usize) -> Vec<u8> {
        let samples = (0..15)
            .map(|index| u8::try_from(index * 13 + 7).unwrap())
            .collect::<Vec<_>>();
        let planes = (0..components)
            .map(|_| samples.as_slice())
            .collect::<Vec<_>>();
        codestream::encode_planar_u8_no_decomp_test_fixture(5, 3, &planes).unwrap()
    }

    fn wrap_jp2(
        codestream: &[u8],
        width: u32,
        height: u32,
        components: u16,
        bpc: u8,
        bpcc: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut output = Vec::new();
        container::write_signature_box(&mut output).unwrap();
        container::write_file_type_box(&mut output, container::ContainerKind::Jp2, 0, &[]).unwrap();
        let mut children = Vec::new();
        container::write_image_header_box(
            &mut children,
            container::ImageHeaderBox {
                width,
                height,
                components,
                bits_per_component: bpc,
                compression_type: 7,
                unknown_color_space: false,
                intellectual_property: false,
            },
        )
        .unwrap();
        if let Some(entries) = bpcc {
            container::write_box(&mut children, container::boxes::BITS_PER_COMPONENT, entries)
                .unwrap();
        }
        container::write_color_specification_box(
            &mut children,
            container::ColorSpecificationBox {
                method: container::ColorSpecificationMethod::Enumerated,
                precedence: 0,
                approximation: 0,
                enumerated_color_space: Some(if components == 1 {
                    container::EnumeratedColorSpace::Greyscale
                } else {
                    container::EnumeratedColorSpace::SRgb
                }),
            },
        )
        .unwrap();
        container::write_jp2_header_box(&mut output, &children).unwrap();
        container::write_contiguous_codestream_box(&mut output, codestream).unwrap();
        output
    }

    fn marker_offset(input: &[u8], marker: [u8; 2]) -> usize {
        input.windows(2).position(|bytes| bytes == marker).unwrap()
    }

    fn box_offset(input: &[u8], box_type: container::FourCc) -> usize {
        input
            .windows(4)
            .position(|bytes| bytes == box_type.as_bytes())
            .unwrap()
            - 4
    }

    #[test]
    fn validates_uniform_and_varying_jp2_header_precision_against_siz() {
        let uniform = codestream(1);
        let uniform_jp2 = wrap_jp2(&uniform, 5, 3, 1, 7, None);
        let metadata = inspect(&uniform_jp2, &InspectOptions::default()).unwrap();
        assert_eq!(metadata.format, InputFormat::Jp2);
        assert_eq!(metadata.image.unwrap().sample_format, SampleFormat::U8);

        let mut first_codestream_wins = uniform_jp2.clone();
        let different =
            codestream::encode_planar_u8_no_decomp_test_fixture(1, 1, &[&[0x2a_u8][..]]).unwrap();
        container::write_contiguous_codestream_box(&mut first_codestream_wins, &different).unwrap();
        assert_eq!(
            inspect(&first_codestream_wins, &InspectOptions::default())
                .unwrap()
                .image
                .unwrap()
                .width,
            5
        );

        let mut varying = codestream(2);
        let siz = marker_offset(&varying, [0xff, 0x51]);
        varying[siz + 43] = 0x89;
        let varying_jp2 = wrap_jp2(&varying, 5, 3, 2, 255, Some(&[7, 0x89]));
        let metadata = inspect(&varying_jp2, &InspectOptions::default()).unwrap();
        assert_eq!(metadata.format, InputFormat::Jp2);
        assert_eq!(
            metadata.codestream.unwrap().kind,
            codestream::CodestreamKind::J2k
        );
    }

    #[test]
    fn rejects_ihdr_geometry_and_component_mismatches_at_exact_fields() {
        let raw = codestream(1);
        for (width, height, components, field_offset, message_fragment) in [
            (5, 4, 1, 0_u64, "height"),
            (6, 3, 1, 4, "width"),
            (5, 3, 2, 8, "component count"),
        ] {
            let input = wrap_jp2(&raw, width, height, components, 7, None);
            let expected =
                (box_offset(&input, container::boxes::IMAGE_HEADER) + 8) as u64 + field_offset;
            assert!(matches!(
                inspect(&input, &InspectOptions::default()),
                Err(J2kError::InvalidInput { offset: Some(offset), message })
                    if offset == expected && message.contains(message_fragment)
            ));
        }
    }

    #[test]
    fn rejects_uniform_and_varying_siz_sample_mismatches_at_exact_fields() {
        let mut uniform = codestream(2);
        let siz = marker_offset(&uniform, [0xff, 0x51]);
        uniform[siz + 43] = 8;
        let uniform_jp2 = wrap_jp2(&uniform, 5, 3, 2, 7, None);
        let expected = (box_offset(&uniform_jp2, container::boxes::IMAGE_HEADER) + 18) as u64;
        assert!(matches!(
            inspect(&uniform_jp2, &InspectOptions::default()),
            Err(J2kError::InvalidInput { offset: Some(offset), .. }) if offset == expected
        ));

        let mut varying = codestream(2);
        let siz = marker_offset(&varying, [0xff, 0x51]);
        varying[siz + 43] = 0x89;
        let varying_jp2 = wrap_jp2(&varying, 5, 3, 2, 255, Some(&[7, 9]));
        let expected = (box_offset(&varying_jp2, container::boxes::BITS_PER_COMPONENT) + 9) as u64;
        assert!(matches!(
            inspect(&varying_jp2, &InspectOptions::default()),
            Err(J2kError::InvalidInput { offset: Some(offset), message })
                if offset == expected && message.contains("entry 1")
        ));
    }

    #[test]
    fn invalid_jp2_metadata_does_not_mutate_caller_output() {
        let raw = codestream(1);
        let input = wrap_jp2(&raw, 6, 3, 1, 7, None);
        let info = ImageInfo::new(
            5,
            3,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Planar,
        )
        .unwrap();
        let mut samples = vec![0x6d; 15];
        {
            let plane = PlaneMut::new(&mut samples, 5, 3, 5, SampleFormat::U8).unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(matches!(
                decode_into(&input, &mut target, &DecodeOptions::default()),
                Err(J2kError::InvalidInput { .. })
            ));
        }
        assert!(samples.iter().all(|sample| *sample == 0x6d));
    }

    #[test]
    fn raw_j2k_and_jph_paths_do_not_acquire_jp2_header_validation() {
        let raw = codestream(1);
        assert_eq!(
            inspect(&raw, &InspectOptions::default()).unwrap().format,
            InputFormat::J2kCodestream
        );

        let mut jph = Vec::new();
        container::write_signature_box(&mut jph).unwrap();
        container::write_file_type_box(&mut jph, container::ContainerKind::Jph, 0, &[]).unwrap();
        container::write_contiguous_codestream_box(&mut jph, &raw).unwrap();
        let metadata = inspect(&jph, &InspectOptions::default()).unwrap();
        assert_eq!(metadata.format, InputFormat::Jph);
        assert!(matches!(
            metadata.support,
            SupportStatus::Unsupported { .. }
        ));
    }

    #[test]
    fn unimplemented_jp2_presentation_mechanisms_fail_closed() {
        let raw = codestream(1);
        let mut palette = wrap_jp2(&raw, 5, 3, 1, 7, None);
        let jp2c = box_offset(&palette, container::boxes::CONTIGUOUS_CODESTREAM);
        let mut palette_box = Vec::new();
        container::write_box(
            &mut palette_box,
            container::boxes::PALETTE,
            &[0, 1, 1, 7, 0],
        )
        .unwrap();
        let jp2h = box_offset(&palette, container::boxes::JP2_HEADER);
        let old_len = u32::from_be_bytes(palette[jp2h..jp2h + 4].try_into().unwrap());
        palette[jp2h..jp2h + 4]
            .copy_from_slice(&(old_len + palette_box.len() as u32).to_be_bytes());
        palette.splice(jp2c..jp2c, palette_box);
        assert!(matches!(
            inspect(&palette, &InspectOptions::default()),
            Err(J2kError::Unsupported {
                feature: UnsupportedFeature::ContainerBox,
                ..
            })
        ));

        let mut icc = wrap_jp2(&raw, 5, 3, 1, 7, None);
        let colr = box_offset(&icc, container::boxes::COLOR_SPECIFICATION);
        icc[colr + 8] = 2;
        assert!(matches!(
            inspect(&icc, &InspectOptions::default()),
            Err(J2kError::Unsupported {
                feature: UnsupportedFeature::ColorModel,
                ..
            })
        ));
    }
}

pub fn bootstrap_summary() -> ProjectSummary {
    ProjectSummary {
        name: PROJECT_NAME,
        summary: "A pure-Rust JPEG 2000 and HTJ2K codec with a native Rust library, thin CLI, and future WASM and language-binding paths.",
    }
}

/// Convenient result alias for all public core operations.
pub type Result<T> = core::result::Result<T, J2kError>;

/// High-level container or codestream family detected from input bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    /// JP2 container with Part 1 codestream payload.
    Jp2,
    /// Raw JPEG 2000 Part 1 codestream.
    J2kCodestream,
    /// JPH container with HTJ2K payload.
    Jph,
    /// Raw HTJ2K codestream.
    Htj2kCodestream,
    /// Input family has not been classified yet.
    Unknown,
}

/// Output family requested by encode calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// JP2 container with deterministic baseline metadata.
    Jp2,
    /// Raw JPEG 2000 Part 1 codestream.
    J2kCodestream,
}

/// Byte order used by multi-byte samples in caller-owned buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleEndian {
    Little,
    Big,
}

/// Pixel sample interpretation for one component plane.
///
/// `byte_order` is `None` for one-byte samples and must be `Some` for
/// multi-byte caller-owned sample buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleFormat {
    pub bits_per_sample: u8,
    pub signed: bool,
    pub byte_order: Option<SampleEndian>,
}

impl SampleFormat {
    pub const U8: Self = Self {
        bits_per_sample: 8,
        signed: false,
        byte_order: None,
    };

    pub const U16_LE: Self = Self {
        bits_per_sample: 16,
        signed: false,
        byte_order: Some(SampleEndian::Little),
    };

    pub const U16_BE: Self = Self {
        bits_per_sample: 16,
        signed: false,
        byte_order: Some(SampleEndian::Big),
    };

    pub const I16_LE: Self = Self {
        bits_per_sample: 16,
        signed: true,
        byte_order: Some(SampleEndian::Little),
    };

    pub const I16_BE: Self = Self {
        bits_per_sample: 16,
        signed: true,
        byte_order: Some(SampleEndian::Big),
    };

    pub fn new(bits_per_sample: u8, signed: bool) -> Result<Self> {
        if bits_per_sample > 8 {
            return Err(J2kError::InvalidParameter {
                parameter: "bits_per_sample",
                message: "multi-byte sample formats require explicit byte order",
            });
        }

        Self::with_byte_order(bits_per_sample, signed, None)
    }

    pub fn with_byte_order(
        bits_per_sample: u8,
        signed: bool,
        byte_order: Option<SampleEndian>,
    ) -> Result<Self> {
        if !(1..=38).contains(&bits_per_sample) {
            return Err(J2kError::InvalidParameter {
                parameter: "bits_per_sample",
                message: "JPEG 2000 component precision must be in 1..=38",
            });
        }
        if bits_per_sample <= 8 && byte_order.is_some() {
            return Err(J2kError::InvalidParameter {
                parameter: "byte_order",
                message: "one-byte sample formats must not declare byte order",
            });
        }
        if bits_per_sample > 8 && byte_order.is_none() {
            return Err(J2kError::InvalidParameter {
                parameter: "byte_order",
                message: "multi-byte sample formats require explicit byte order",
            });
        }

        Ok(Self {
            bits_per_sample,
            signed,
            byte_order,
        })
    }
}

/// Color model declared or inferred for an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorModel {
    Grayscale,
    Rgb,
    Rgba,
    YCbCr,
    Unknown,
}

/// Memory layout used by decoded or caller-supplied image samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentLayout {
    /// One contiguous plane per component.
    Planar,
    /// One pixel-interleaved buffer, such as RGBRGB.
    Interleaved,
}

/// Semantic decode mode selected by callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeMode {
    /// Decode pixels as rendered/display channels for the declared image color
    /// model.
    Rendered,
    /// Decode raw codestream component planes without applying display-only
    /// projection such as palette expansion, alpha handling, or color-managed
    /// rendering.
    Components,
}

/// Image geometry and sample model shared by metadata, decode, and encode APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub components: u16,
    pub sample_format: SampleFormat,
    pub color_model: ColorModel,
    pub layout: ComponentLayout,
}

/// Geometry and sample model for one caller-visible output component.
///
/// `ImageInfo` remains the packed-image convenience description. Component
/// mode callers should use these descriptors when source components may have
/// different precision, signedness, origins, or sampling factors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentInfo {
    /// Source codestream component index, or `None` for a rendered channel
    /// produced by a transform or container projection.
    pub source_component: Option<u16>,
    pub width: u32,
    pub height: u32,
    pub x_origin: u32,
    pub y_origin: u32,
    pub horizontal_separation: u8,
    pub vertical_separation: u8,
    pub sample_format: SampleFormat,
}

impl ImageInfo {
    pub fn new(
        width: u32,
        height: u32,
        components: u16,
        sample_format: SampleFormat,
        color_model: ColorModel,
        layout: ComponentLayout,
    ) -> Result<Self> {
        if width == 0 {
            return Err(J2kError::InvalidParameter {
                parameter: "width",
                message: "image width must be greater than zero",
            });
        }
        if height == 0 {
            return Err(J2kError::InvalidParameter {
                parameter: "height",
                message: "image height must be greater than zero",
            });
        }
        if components == 0 {
            return Err(J2kError::InvalidParameter {
                parameter: "components",
                message: "image must contain at least one component",
            });
        }

        Ok(Self {
            width,
            height,
            components,
            sample_format,
            color_model,
            layout,
        })
    }
}

/// Caller-owned immutable component plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plane<'a> {
    pub samples: &'a [u8],
    pub width: u32,
    pub height: u32,
    pub stride_bytes: usize,
    pub sample_format: SampleFormat,
}

impl<'a> Plane<'a> {
    pub fn new(
        samples: &'a [u8],
        width: u32,
        height: u32,
        stride_bytes: usize,
        sample_format: SampleFormat,
    ) -> Result<Self> {
        validate_plane(
            "plane",
            samples.len(),
            width,
            height,
            stride_bytes,
            sample_format,
        )?;
        Ok(Self {
            samples,
            width,
            height,
            stride_bytes,
            sample_format,
        })
    }
}

/// Caller-owned mutable component plane used by decode-into paths.
#[derive(Debug, PartialEq, Eq)]
pub struct PlaneMut<'a> {
    pub samples: &'a mut [u8],
    pub width: u32,
    pub height: u32,
    pub stride_bytes: usize,
    pub sample_format: SampleFormat,
}

impl<'a> PlaneMut<'a> {
    pub fn new(
        samples: &'a mut [u8],
        width: u32,
        height: u32,
        stride_bytes: usize,
        sample_format: SampleFormat,
    ) -> Result<Self> {
        validate_plane(
            "plane",
            samples.len(),
            width,
            height,
            stride_bytes,
            sample_format,
        )?;
        Ok(Self {
            samples,
            width,
            height,
            stride_bytes,
            sample_format,
        })
    }
}

/// Borrowed image view for caller-owned encode inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageView<'a> {
    Planar {
        info: &'a ImageInfo,
        planes: &'a [Plane<'a>],
    },
    Interleaved {
        info: &'a ImageInfo,
        samples: &'a [u8],
        stride_bytes: usize,
    },
}

/// Mutable caller-owned decode target.
#[derive(Debug, PartialEq, Eq)]
pub enum ImageViewMut<'a> {
    Planar {
        info: &'a ImageInfo,
        planes: &'a mut [PlaneMut<'a>],
    },
    Interleaved {
        info: &'a ImageInfo,
        samples: &'a mut [u8],
        stride_bytes: usize,
    },
}

/// Owned image returned by convenience full-decode APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub info: ImageInfo,
    /// Per-plane/component descriptors in the same order as the decoded
    /// output. For interleaved rendered output these describe the interleaved
    /// channels.
    pub component_info: Vec<ComponentInfo>,
    pub data: ImageData,
}

/// Reusable scratch for selective Part 1 component decode into caller-owned
/// planar storage.
#[derive(Default)]
pub struct Part1DecodeWorkspace {
    codestream: codestream::Part1ComponentDecodeWorkspace,
}

impl Part1DecodeWorkspace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Coefficient slots retained for the largest selected code block.
    pub fn coefficient_capacity(&self) -> usize {
        self.codestream.coefficient_capacity()
    }

    /// Bytes retained for fragmented code-block segment assembly.
    pub fn segment_capacity(&self) -> usize {
        self.codestream.segment_capacity()
    }

    /// Transform scratch slots retained for the largest selected tile axis.
    pub fn transform_capacity(&self) -> usize {
        self.codestream.transform_capacity()
    }

    /// Largest retained full coefficient-plane capacity, in samples.
    pub fn full_coefficient_plane_capacity(&self) -> usize {
        self.codestream.full_coefficient_plane_capacity()
    }

    /// Largest retained full-transform workspace capacity, in samples.
    pub fn full_transform_scratch_capacity(&self) -> usize {
        self.codestream.full_transform_scratch_capacity()
    }

    /// Private worker workspaces retained by prepared parallel execution.
    pub fn parallel_worker_capacity(&self) -> usize {
        self.codestream.parallel_worker_capacity()
    }

    /// Capacity-based heap bytes retained by the complete workspace,
    /// including private parallel worker scratch.
    pub fn retained_heap_bytes(&self) -> u64 {
        self.codestream.retained_heap_bytes()
    }

    /// Clear logical scratch lengths while retaining allocation capacity.
    pub fn clear(&mut self) {
        self.codestream.clear();
    }
}

/// Reusable, structurally validated selective Part 1 decode plan.
///
/// The plan borrows its codestream bytes, retains packet topology and selected
/// code-block ranges, and may be executed repeatedly into different validated
/// planar targets without reparsing packet headers.
pub struct PreparedPart1Decode<'a> {
    info: ImageInfo,
    component_info: Vec<ComponentInfo>,
    codestream: codestream::PreparedPart1ComponentDecode<'a>,
}

impl PreparedPart1Decode<'_> {
    pub fn info(&self) -> &ImageInfo {
        &self.info
    }

    /// Exact native output descriptors in caller plane order.
    pub fn component_info(&self) -> &[ComponentInfo] {
        &self.component_info
    }

    pub fn preparation_timings(&self) -> codestream::DecodeStageTimings {
        self.codestream.preparation_timings()
    }

    pub fn memory_accounting(&self) -> codestream::PreparedPart1PlanMemory {
        self.codestream.memory_accounting()
    }

    pub fn execution_parallelism(&self) -> (codestream::DecodeParallelAxis, usize) {
        self.codestream.execution_parallelism()
    }
}

/// Reusable std workspace for algorithmic HTJ2K decode.
///
/// This retains codestream-level HT scratch and coefficient storage across
/// calls to [`decode_htj2k_with_workspace`].
#[cfg(feature = "std")]
#[derive(Debug, Default)]
pub struct Htj2kDecodeWorkspace {
    codestream: codestream::HtCodestreamDecodeWorkspace,
}

/// Diagnostic HTJ2K cleanup-output prefix traversal result.
///
/// This is a benchmark/provenance surface for the in-progress real HT parser.
/// It reports cleanup-prefix parser progress only; it is not a decoded image and
/// does not imply general HTJ2K decode support.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htj2kCleanupVlcOutputProbe {
    pub output_count: usize,
    pub significant_output_count: usize,
    pub significant_refinement_slot_mask_low64: u64,
    pub first_significant_output: Option<Htj2kCleanupVlcSignificantOutput>,
    pub coding_passes: u16,
    pub packet_missing_most_significant_bitplanes: u8,
    pub cleanup_bitplane: Option<u8>,
    pub materialized_coefficient_count: usize,
    pub materialized_coefficient_prefix: [i32; 4],
    /// Unsigned HT cleanup sign-magnitude coefficient prefix.
    ///
    /// Bit 31 carries the sign and bits 0..30 carry the centered magnitude.
    pub ht_sign_magnitude_coefficient_prefix: [u32; 4],
    pub reversible_transfer_qcd_guard_bits: Option<u8>,
    pub reversible_transfer_qcd_exponent: Option<u8>,
    pub reversible_transfer_k_max: Option<u8>,
    pub reversible_transfer_shift: Option<u8>,
    pub reversible_transfer_coefficient_prefix: Option<[i32; 4]>,
    pub reversible_transfer_sign_magnitude_coefficient_prefix: Option<[i32; 4]>,
    pub reversible_transfer_sample_prefix: Option<[u8; 4]>,
    pub reversible_transfer_nonzero_coefficient_slot_mask_low64: Option<u64>,
    pub first_vlc_lookup: Htj2kCleanupVlcFirstLookup,
    pub first_vlc_group: Htj2kCleanupVlcFirstGroup,
    pub scratch_words: usize,
    pub cleanup_progress: codestream::HtCodestreamVlcCleanupProgressSnapshot,
    pub segment_bit_progress: codestream::HtCodestreamVlcCleanupSegmentBitProgressSnapshot,
}

/// First standard-table VLC lookup made by the current HTJ2K cleanup-output
/// parser probe.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htj2kCleanupVlcFirstLookup {
    pub context: u8,
    pub zero_context_mel_event: Option<bool>,
    pub prefix_bits_lsb: u8,
    pub table_word: u16,
    pub gated_table_word: u16,
    pub codeword_vlc_bits: u8,
    pub significance_bits: u8,
    pub embedded_magnitude_bits: u8,
    pub magnitude_exponent_reduction_bits: u8,
    pub u_offset: bool,
    pub next_initial_context: u8,
}

/// First cleanup VLC quad-group step made by the current HTJ2K cleanup-output
/// parser probe.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htj2kCleanupVlcFirstGroup {
    pub first_quad_present_count: usize,
    pub first_quad_present_mask: u16,
    pub second_quad_present: bool,
    pub second_quad_present_count: usize,
    pub first_context: u8,
    pub first_zero_context_mel_event: Option<bool>,
    pub first_prefix_bits_lsb: u8,
    pub first_table_word: u16,
    pub first_gated_table_word: u16,
    pub first_codeword_vlc_bits: u8,
    pub first_significance_bits: u8,
    pub first_embedded_magnitude_bits: u8,
    pub first_magnitude_exponent_reduction_bits: u8,
    pub first_u_offset: bool,
    pub second_context: Option<u8>,
    pub second_zero_context_mel_event: Option<bool>,
    pub second_prefix_bits_lsb: Option<u8>,
    pub second_table_word: Option<u16>,
    pub second_gated_table_word: Option<u16>,
    pub second_codeword_vlc_bits: Option<u8>,
    pub second_significance_bits: Option<u8>,
    pub second_embedded_magnitude_bits: Option<u8>,
    pub second_magnitude_exponent_reduction_bits: Option<u8>,
    pub second_u_offset: Option<bool>,
    pub paired_uvlc_both_offsets_mel_event: Option<bool>,
    pub paired_uvlc_first: Option<u16>,
    pub paired_uvlc_second: Option<u16>,
    pub paired_uvlc_consumed_bits: Option<u8>,
    pub single_tail_u_value: Option<u16>,
}

/// Compact description of one significant HTJ2K cleanup-output record.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Htj2kCleanupVlcSignificantOutput {
    pub refinement_slot: usize,
    pub quad_slot: u8,
    pub magnitude_sign_bits: u16,
    pub magnitude_sign_value: u16,
    pub embedded_magnitude_bit: bool,
    pub magnitude_exponent_reduction: bool,
    /// Unsigned HT cleanup sign-magnitude coefficient, when materialized.
    ///
    /// Bit 31 carries the sign and bits 0..30 carry the centered magnitude.
    pub ht_sign_magnitude_coefficient: Option<u32>,
    pub reversible_transfer_coefficient: Option<i32>,
    pub reversible_transfer_sample: Option<u8>,
}

/// Owned sample buffers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageData {
    Planes(Vec<Vec<u8>>),
    Interleaved(Vec<u8>),
}

/// Container and codestream metadata available without image allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub format: InputFormat,
    pub image: Option<ImageInfo>,
    pub codestream: Option<CodestreamInfo>,
    pub container: Option<ContainerInfo>,
    pub support: SupportStatus,
    pub records: Vec<MetadataRecord>,
}

/// Resolved output shape for a full-image decode request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeShape {
    pub width: u32,
    pub height: u32,
    pub codestream_components: u16,
    pub colour_channels: u16,
    pub output_components: u16,
    pub sample_format: SampleFormat,
    pub layout: ComponentLayout,
    pub byte_order: Option<SampleEndian>,
    pub color_model: ColorModel,
    pub mode: DecodeMode,
}

impl DecodeShape {
    fn image_info(&self) -> Result<ImageInfo> {
        ImageInfo::new(
            self.width,
            self.height,
            self.output_components,
            self.sample_format,
            self.color_model,
            self.layout,
        )
    }
}

/// Codestream-level fields that can be reported before full decode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodestreamInfo {
    pub kind: codestream::CodestreamKind,
    pub tile_grid: Option<TileGrid>,
    pub progression_order: Option<ProgressionOrder>,
    pub transform: Option<WaveletTransform>,
    pub entropy_coder: Option<EntropyCoder>,
}

/// JP2/JPH container fields relevant to support classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerInfo {
    pub brand: Option<String>,
    pub compatible_brands: Vec<String>,
    pub codestream_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileGrid {
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_origin_x: u32,
    pub tile_origin_y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressionOrder {
    Lrcp,
    Rlcp,
    Rpcl,
    Pcrl,
    Cprl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveletTransform {
    Reversible53,
    Irreversible97,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntropyCoder {
    ClassicTier1,
    HtBlockCoding,
}

/// Preserved metadata blocks whose semantics may be owned by later features.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataRecord {
    pub kind: MetadataKind,
    pub label: Option<String>,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataKind {
    Xml,
    Uuid,
    UnknownBox,
    UnknownMarker,
}

/// Whether the parsed input is in the implemented milestone subset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportStatus {
    /// Algorithmic repo-owned decode path for the current milestone.
    Supported,
    Unsupported {
        feature: UnsupportedFeature,
        detail: String,
    },
    Unknown {
        detail: String,
    },
}

impl SupportStatus {
    /// True when `decode` may attempt the input with default support gating.
    pub fn permits_decode(&self) -> bool {
        matches!(self, Self::Supported)
    }
}

/// Named unsupported features used by errors and metadata classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedFeature {
    InputFormat,
    OutputFormat,
    ContainerBox,
    MarkerSegment,
    ProgressionOrder,
    WaveletTransform,
    EntropyCoder,
    ColorModel,
    ComponentLayout,
    PartialDecodeMode,
    IncrementalInput,
}

/// Full-image decode parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeOptions {
    /// Permit legacy callers to request a native best-effort decode attempt
    /// after metadata inspection has classified an input as outside the
    /// supported `emuella-j2k` contract.
    ///
    /// This is a compatibility flag only. It does not route to a third-party
    /// codec, may still fail with `Unsupported`, and does not make the input
    /// part of the supported decode matrix. JPH, raw HTJ2K, and unknown formats
    /// are not enabled by this option.
    pub allow_best_effort_backend_decode: bool,
    pub mode: DecodeMode,
    pub requested_components: ComponentSelection,
    /// Maximum number of leading quality layers to reconstruct. `None`
    /// reconstructs every layer. Existing admitted one-layer profiles clamp
    /// any positive limit to their complete output. Genuine truncation is
    /// currently bounded to the raw, full-image, planar, single-component
    /// two-layer LRCP profile, where values at or above two clamp to complete.
    pub max_quality_layers: Option<u16>,
    pub target_layout: ComponentLayout,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self {
            allow_best_effort_backend_decode: false,
            mode: DecodeMode::Rendered,
            requested_components: ComponentSelection::All,
            max_quality_layers: None,
            target_layout: ComponentLayout::Planar,
        }
    }
}

fn validate_max_quality_layers(mode: DecodeMode, max_layers: Option<u16>) -> Result<()> {
    if max_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }
    if max_layers.is_some() && mode != DecodeMode::Components {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "maximum quality-layer selection is available only in component mode",
        ));
    }
    Ok(())
}

fn validate_max_quality_layer_profile(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<()> {
    if options.max_quality_layers.is_none() {
        return Ok(());
    }
    let codestream_bytes = primary_part1_codestream_bytes(input, metadata)?.ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "maximum quality-layer selection is currently available only for Part 1 J2K and JP2 component decode",
        )
    })?;
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|style| style.layers == 1)
    {
        return Ok(());
    }
    if metadata.format != InputFormat::J2kCodestream
        || options.target_layout != ComponentLayout::Planar
        || !(matches!(&options.requested_components, ComponentSelection::All)
            || matches!(&options.requested_components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16]))
        || !codestream::is_supported_part1_native_quality_layer_component_profile(&parsed)
    {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "maximum quality-layer selection requires the bounded raw single-component two-layer LRCP profile with full planar output",
        ));
    }
    Ok(())
}

fn validate_partial_quality_layer_profile(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<()> {
    if options.max_quality_layers.is_none() {
        return Ok(());
    }
    if options.max_quality_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(());
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|style| style.layers == 1)
    {
        return Ok(());
    }
    let exact_component = matches!(&options.components, ComponentSelection::All)
        || matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16]);
    if metadata.format != InputFormat::J2kCodestream
        || options.region.is_some()
        || options.tile.is_some()
        || options.resolution != ResolutionLevel::Full
        || options.target_layout != ComponentLayout::Planar
        || !exact_component
        || !codestream::is_supported_part1_native_quality_layer_component_profile(&parsed)
    {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "maximum quality-layer selection requires the bounded raw single-component two-layer LRCP profile with full planar output",
        ));
    }
    Ok(())
}

#[cfg(feature = "std")]
impl Htj2kDecodeWorkspace {
    /// Create an empty reusable algorithmic HTJ2K decode workspace.
    pub fn new() -> Self {
        Self {
            codestream: codestream::HtCodestreamDecodeWorkspace::new(),
        }
    }

    /// Number of reusable HT coefficient slots currently retained.
    pub fn coefficient_len(&self) -> usize {
        self.codestream.coefficient_len()
    }

    /// Number of reusable HT side-buffer scratch words currently retained.
    pub fn scratch_len(&self) -> usize {
        self.codestream.scratch_len()
    }

    /// Number of reusable Part 15 VLC quad side-bit slots currently retained.
    pub fn vlc_quad_side_bit_len(&self) -> usize {
        self.codestream.vlc_quad_side_bit_len()
    }

    /// Number of reusable Part 15 VLC odd-tail `u` slots currently retained.
    pub fn vlc_odd_tail_u_value_len(&self) -> usize {
        self.codestream.vlc_odd_tail_u_value_len()
    }

    /// Number of reusable direct VLC cleanup-output slots currently retained.
    pub fn vlc_cleanup_output_len(&self) -> usize {
        self.codestream.vlc_cleanup_output_len()
    }

    /// Number of reusable VLC context-progression slots currently retained.
    pub fn vlc_context_state_len(&self) -> usize {
        self.codestream.vlc_context_state_len()
    }

    /// Dispatch provenance from the most recent reusable HT code-block decode,
    /// if any.
    pub fn last_code_block_dispatch_progress(
        &self,
    ) -> Option<codestream::HtCodestreamCodeBlockDispatchProgressSnapshot> {
        self.codestream.last_code_block_dispatch_progress()
    }
}

/// Metadata parse parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectOptions {
    pub preserve_raw_metadata: bool,
    pub classify_support: bool,
}

impl Default for InspectOptions {
    fn default() -> Self {
        Self {
            preserve_raw_metadata: true,
            classify_support: true,
        }
    }
}

/// Encode parameters for the initial Part 1 encoder surface.
#[derive(Debug, Clone, PartialEq)]
pub struct EncodeOptions {
    pub format: OutputFormat,
    pub progression_order: ProgressionOrder,
    pub transform: WaveletTransform,
    pub quality: EncodeQuality,
    pub decomposition_levels: u8,
    pub tile_size: Option<TileSize>,
    pub metadata: Vec<MetadataRecord>,
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Jp2,
            progression_order: ProgressionOrder::Lrcp,
            transform: WaveletTransform::Reversible53,
            quality: EncodeQuality::Lossless,
            decomposition_levels: 0,
            tile_size: None,
            metadata: Vec::new(),
        }
    }
}

/// Encode parameters for raw lossless HTJ2K output.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Htj2kEncodeOptions {
    /// Reversible 5/3 decomposition levels. The initial algorithmic surface
    /// supports cleanup-only no-decomposition codestreams.
    pub decomposition_levels: u8,
}

/// Optional tile dimensions for the narrow native multi-tile encode surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncodeQuality {
    Lossless,
    TargetRate { bits_per_pixel: f32 },
}

/// Component selection shared by full and partial decode paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentSelection {
    All,
    Indices(Vec<u16>),
}

/// Scoped partial decode request. Unsupported combinations must fail explicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialDecodeOptions {
    /// Non-empty full-resolution image-relative reference-grid rectangle.
    /// Mutually exclusive with [`Self::tile`].
    pub region: Option<Region>,
    /// SIZ tile-grid coordinate resolved to the clipped image-relative tile
    /// rectangle. Mutually exclusive with [`Self::region`].
    pub tile: Option<TileSelection>,
    pub resolution: ResolutionLevel,
    pub components: ComponentSelection,
    /// Maximum number of leading quality layers to reconstruct. Existing
    /// admitted one-layer profiles preserve their positive-limit behaviour.
    /// Genuine truncation is currently bounded to the raw, full-image, planar,
    /// single-component two-layer LRCP profile; spatial selection and
    /// resolution reduction remain outside that two-layer profile.
    pub max_quality_layers: Option<u16>,
    pub target_layout: ComponentLayout,
}

impl Default for PartialDecodeOptions {
    fn default() -> Self {
        Self {
            region: None,
            tile: None,
            resolution: ResolutionLevel::Full,
            components: ComponentSelection::All,
            max_quality_layers: None,
            target_layout: ComponentLayout::Planar,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Zero-based horizontal and vertical coordinates in the SIZ tile grid.
pub struct TileSelection {
    pub tile_x: u32,
    pub tile_y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionLevel {
    Full,
    Reduced { discard_levels: u8 },
}

/// Internal/test-oriented descriptor for partial-decode work planning.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PartialDecodeWorkPlan {
    pub request: PartialDecodeOptions,
    pub selected_resolution: PlannedResolution,
    pub full_image_full_resolution_fallback: bool,
    pub selected_tiles: Vec<PlannedPartialTile>,
    pub selected_components: Vec<u16>,
    pub work_units: PlannedPartialWorkUnits,
    pub evidence: PartialDecodePlanEvidence,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlannedResolution {
    pub discard_levels: u8,
    pub codestream_resolution_level: Option<u8>,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlannedPartialTile {
    pub tile_index: u16,
    pub tile_x: u32,
    pub tile_y: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlannedPartialWorkUnits {
    pub packet_detail: WorkUnitDetail,
    pub code_block_detail: WorkUnitDetail,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WorkUnitDetail {
    NotAvailableYet { status: &'static str },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PartialDecodePlanEvidence {
    TrueCodestreamPartialCandidate,
    FullDecodeBackedAdapter,
}

impl PartialDecodeWorkPlan {
    #[allow(dead_code)]
    pub(crate) fn satisfies_true_partial_assertions(&self) -> bool {
        self.evidence == PartialDecodePlanEvidence::TrueCodestreamPartialCandidate
            && !self.full_image_full_resolution_fallback
            && (self.selected_resolution.discard_levels > 0
                || self.request.region.is_some()
                || self.request.tile.is_some()
                || !matches!(self.request.components, ComponentSelection::All))
    }
}

/// Contiguous-prefix input feeder for callers that receive bytes over time.
///
/// This deliberately buffers a growing byte slice and reuses the normal inspect
/// and decode paths. It is not a packet index, byte-range cache, or arbitrary
/// random-access streaming contract.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IncrementalDecoder {
    buffer: Vec<u8>,
}

impl IncrementalDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append the next contiguous bytes for this image.
    pub fn feed(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn buffered_bytes(&self) -> &[u8] {
        &self.buffer
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Try metadata inspection against the bytes fed so far.
    pub fn inspect(&self, options: &InspectOptions) -> Result<Metadata> {
        inspect(&self.buffer, options)
    }

    /// Decode only when the buffered bytes already contain a complete input.
    pub fn decode(&self, options: &DecodeOptions) -> Result<Image> {
        decode(&self.buffer, options)
    }

    /// Run the conservative partial-decode prototype against buffered bytes.
    pub fn decode_partial(&self, options: &PartialDecodeOptions) -> Result<Image> {
        decode_partial(&self.buffer, options)
    }
}

/// Inspect container and codestream metadata without allocating image samples.
pub fn inspect(input: &[u8], options: &InspectOptions) -> Result<Metadata> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    if input.starts_with(&[0xff, 0x4f]) {
        let codestream = codestream::parse(input).map_err(map_codestream_error)?;
        return Ok(metadata_from_codestream(input, codestream, options));
    }

    let container = container::parse(input).map_err(map_container_error)?;
    metadata_from_container(input, container, options)
}

/// Convenience full decode that owns the returned image buffers.
pub fn decode(input: &[u8], options: &DecodeOptions) -> Result<Image> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    if !matches!(options.requested_components, ComponentSelection::All)
        && options.mode != DecodeMode::Components
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "component-subset full decode is available only in component mode",
        ));
    }
    validate_max_quality_layers(options.mode, options.max_quality_layers)?;

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_max_quality_layer_profile(input, &metadata, options)?;
    requested_component_indices(&metadata, &options.requested_components)?;
    if options.allow_best_effort_backend_decode {
        validate_native_best_effort_decode_request(&metadata)?;
    }
    reject_unsupported_rendered_projection(&metadata, options)?;
    reject_unsupported_part1_rendered_sampling(input, &metadata, options)?;
    #[cfg(feature = "std")]
    if let Some(image) = decode_algorithmic_htj2k(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) =
        decode_owned_part1_p0_13_high_component_progression(input, &metadata, options)?
    {
        return Ok(image);
    }
    if let Some(image) =
        decode_owned_part1_p0_10_subsampled_reversible_mct(input, &metadata, options)?
    {
        return Ok(image);
    }
    if let Some(image) = decode_owned_baseline(input, &metadata, options)? {
        return Ok(image);
    }

    require_supported_metadata(&metadata)?;
    Err(native_decode_unsupported(&metadata, options))
}

/// Decode the supported algorithmic HTJ2K profile with caller-retained block
/// workspace.
///
/// Returns `Ok(None)` for non-HTJ2K input or HTJ2K outside the admitted
/// lossless no-decomposition profile.
#[cfg(feature = "std")]
pub fn decode_htj2k_with_workspace(
    input: &[u8],
    options: &DecodeOptions,
    workspace: &mut Htj2kDecodeWorkspace,
) -> Result<Option<Image>> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    if !matches!(options.requested_components, ComponentSelection::All) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "component-subset full decode is not enabled for HTJ2K",
        ));
    }

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_max_quality_layers(options.mode, options.max_quality_layers)?;
    validate_max_quality_layer_profile(input, &metadata, options)?;
    reject_unsupported_rendered_projection(&metadata, options)?;
    decode_algorithmic_htj2k_with_workspace(input, &metadata, options, workspace)
}

/// Run the real HTJ2K cleanup-output prefix parser for an admitted algorithmic
/// HTJ2K profile using caller-retained workspace.
///
/// Returns `Ok(None)` when the input is outside that profile. This is diagnostic
/// instrumentation for implementation and benchmark work, not a full-image
/// decode path.
#[cfg(feature = "std")]
pub fn decode_htj2k_cleanup_vlc_output_probe_with_workspace(
    input: &[u8],
    options: &DecodeOptions,
    workspace: &mut Htj2kDecodeWorkspace,
) -> Result<Option<Htj2kCleanupVlcOutputProbe>> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    if !matches!(options.requested_components, ComponentSelection::All) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "component-subset HTJ2K cleanup probing is not implemented",
        ));
    }

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_max_quality_layers(options.mode, options.max_quality_layers)?;
    validate_max_quality_layer_profile(input, &metadata, options)?;
    if options.allow_best_effort_backend_decode {
        validate_native_best_effort_decode_request(&metadata)?;
    }
    reject_unsupported_rendered_projection(&metadata, options)?;
    decode_htj2k_cleanup_vlc_output_probe_from_metadata(input, &metadata, workspace)
}

/// Resolve the full-image output shape for a decode request without allocating
/// image samples.
pub fn decode_shape(input: &[u8], options: &DecodeOptions) -> Result<DecodeShape> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    if !matches!(options.requested_components, ComponentSelection::All)
        && options.mode != DecodeMode::Components
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "component-subset full decode is available only in component mode",
        ));
    }
    validate_max_quality_layers(options.mode, options.max_quality_layers)?;

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_max_quality_layer_profile(input, &metadata, options)?;
    requested_component_indices(&metadata, &options.requested_components)?;
    if options.allow_best_effort_backend_decode {
        validate_native_best_effort_decode_request(&metadata)?;
    }
    reject_unsupported_rendered_projection(&metadata, options)?;
    reject_unsupported_part1_rendered_sampling(input, &metadata, options)?;

    if let Some(shape) = p0_13_high_component_progression_decode_shape(input, &metadata, options)? {
        return Ok(shape);
    }
    if let Some(shape) = p0_10_subsampled_reversible_mct_decode_shape(input, &metadata, options)? {
        return Ok(shape);
    }

    require_native_full_decode_coverage(input, &metadata, options)?;

    decode_shape_from_metadata(&metadata, options)
}

fn decode_owned_baseline(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<Image>> {
    if !matches!(metadata.support, SupportStatus::Supported) {
        return Ok(None);
    }

    let codestream_bytes = primary_part1_codestream_bytes(input, metadata)?;
    let Some(codestream_bytes) = codestream_bytes else {
        return Ok(None);
    };
    if !codestream::is_owned_baseline_profile(codestream_bytes) {
        return Ok(None);
    }

    let decoded = if options.max_quality_layers.is_some() {
        let indices = requested_component_indices(metadata, &options.requested_components)?;
        codestream::decode_baseline_owned_components_selected_with_max_layers(
            codestream_bytes,
            &indices,
            options.max_quality_layers,
        )
    } else {
        match (&options.mode, &options.requested_components) {
            (DecodeMode::Rendered, _) => {
                codestream::decode_baseline_owned_rendered(codestream_bytes)
            }
            (DecodeMode::Components, ComponentSelection::All) => {
                codestream::decode_baseline_owned_components(codestream_bytes)
            }
            (DecodeMode::Components, ComponentSelection::Indices(indices)) => {
                codestream::decode_baseline_owned_components_selected(codestream_bytes, indices)
            }
        }
    }
    .map_err(map_codestream_error)?;
    let component_info = if options.mode == DecodeMode::Components {
        Some(part1_component_info(
            codestream_bytes,
            &options.requested_components,
            None,
        )?)
    } else {
        None
    };
    decoded_baseline_to_image_with_component_info(decoded, options, component_info).map(Some)
}

fn is_p0_10_decode_request(options: &DecodeOptions) -> bool {
    !options.allow_best_effort_backend_decode
        && options.mode == DecodeMode::Components
        && matches!(&options.requested_components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
        && options.max_quality_layers.is_none()
        && options.target_layout == ComponentLayout::Planar
}

fn decode_owned_part1_p0_10_subsampled_reversible_mct(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<Image>> {
    if !is_p0_10_decode_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_10_subsampled_reversible_mct_component_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let decoded =
        codestream::decode_part1_p0_10_subsampled_reversible_mct_component_zero(codestream_bytes)
            .map_err(map_codestream_error)?;
    let component_info =
        part1_component_info(codestream_bytes, &options.requested_components, None)?;
    decoded_baseline_to_image_with_component_info(decoded, options, Some(component_info)).map(Some)
}

fn p0_10_subsampled_reversible_mct_decode_shape(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<DecodeShape>> {
    if !is_p0_10_decode_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_10_subsampled_reversible_mct_component_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let sample_format = metadata
        .image
        .as_ref()
        .map(|image| image.sample_format)
        .ok_or_else(sample_size_overflow)?;
    Ok(Some(DecodeShape {
        width: 64,
        height: 64,
        codestream_components: 3,
        colour_channels: 3,
        output_components: 1,
        sample_format,
        layout: ComponentLayout::Planar,
        byte_order: sample_format.byte_order,
        color_model: ColorModel::Unknown,
        mode: DecodeMode::Components,
    }))
}

fn is_p0_13_decode_request(options: &DecodeOptions) -> bool {
    !options.allow_best_effort_backend_decode
        && options.mode == DecodeMode::Components
        && matches!(&options.requested_components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
        && options.max_quality_layers.is_none()
        && options.target_layout == ComponentLayout::Planar
}

fn decode_owned_part1_p0_13_high_component_progression(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<Image>> {
    if !is_p0_13_decode_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_13_high_component_progression_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let decoded =
        codestream::decode_part1_p0_13_high_component_progression_component_zero(codestream_bytes)
            .map_err(map_codestream_error)?;
    let component_info =
        part1_component_info(codestream_bytes, &options.requested_components, None)?;
    decoded_baseline_to_image_with_component_info(decoded, options, Some(component_info)).map(Some)
}

fn p0_13_high_component_progression_decode_shape(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<DecodeShape>> {
    if !is_p0_13_decode_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_13_high_component_progression_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let sample_format = metadata
        .image
        .as_ref()
        .map(|image| image.sample_format)
        .ok_or_else(sample_size_overflow)?;
    Ok(Some(DecodeShape {
        width: 1,
        height: 1,
        codestream_components: 257,
        colour_channels: 257,
        output_components: 1,
        sample_format,
        layout: ComponentLayout::Planar,
        byte_order: sample_format.byte_order,
        color_model: ColorModel::Unknown,
        mode: DecodeMode::Components,
    }))
}

fn primary_part1_codestream_bytes<'a>(
    input: &'a [u8],
    metadata: &Metadata,
) -> Result<Option<&'a [u8]>> {
    match metadata.format {
        InputFormat::J2kCodestream => Ok(Some(input)),
        InputFormat::Jp2 => {
            let container = container::parse(input).map_err(map_container_error)?;
            container
                .primary_codestream(input)
                .map_err(map_container_error)
        }
        _ => Ok(None),
    }
}

#[cfg(feature = "std")]
fn primary_htj2k_codestream_bytes<'a>(
    input: &'a [u8],
    metadata: &Metadata,
) -> Result<Option<&'a [u8]>> {
    match metadata.format {
        InputFormat::Htj2kCodestream => Ok(Some(input)),
        InputFormat::Jph => {
            let container = container::parse(input).map_err(map_container_error)?;
            container
                .primary_codestream(input)
                .map_err(map_container_error)
        }
        _ => Ok(None),
    }
}

fn reject_unsupported_rendered_projection(
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<()> {
    if options.mode == DecodeMode::Rendered
        && metadata.image.as_ref().is_some_and(|image| {
            image.sample_format.bits_per_sample > 8 || image.sample_format.signed
        })
    {
        return Err(unsupported(
            UnsupportedFeature::ColorModel,
            "rendered projection from high-bit-depth or signed component samples is not implemented",
        ));
    }

    Ok(())
}

fn reject_unsupported_part1_rendered_sampling(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<()> {
    if options.mode != DecodeMode::Rendered {
        return Ok(());
    }
    if let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? {
        let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
        if metadata.format == InputFormat::Jp2 {
            // Select JP2 default-image geometry only at this container
            // presentation boundary. Pixel projection remains held.
            let _default_image_geometry = jp2_default_image_geometry(&parsed)?;
        }
        if codestream::is_supported_part1_native_subsampled_component_profile(&parsed) {
            return Err(unsupported(
                UnsupportedFeature::ComponentLayout,
                "rendered output does not implicitly resample unequal native component grids; request planar component mode",
            ));
        }
    }
    Ok(())
}

fn jp2_default_image_geometry(
    codestream: &codestream::Codestream,
) -> Result<codestream::geometry::CommonGridPlan> {
    codestream::geometry::CommonGridPlan::new(
        codestream
            .siz
            .image_reference_rect()
            .map_err(map_codestream_error)?,
        &codestream.siz.components,
    )
    .map_err(map_codestream_error)
}

fn validate_native_best_effort_decode_request(metadata: &Metadata) -> Result<()> {
    if matches!(
        metadata.format,
        InputFormat::Jph | InputFormat::Htj2kCodestream | InputFormat::Unknown
    ) {
        return Err(unsupported(
            UnsupportedFeature::InputFormat,
            "native best-effort decode is limited to JP2 and raw J2K Part 1 inputs",
        ));
    }
    Ok(())
}

fn require_native_full_decode_coverage(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<()> {
    if native_full_decode_is_available(input, metadata)? {
        return Ok(());
    }

    require_supported_metadata(metadata)?;
    Err(native_decode_unsupported(metadata, options))
}

fn native_full_decode_is_available(input: &[u8], metadata: &Metadata) -> Result<bool> {
    if matches!(
        metadata.format,
        InputFormat::Htj2kCodestream | InputFormat::Jph
    ) {
        let codestream_bytes = match metadata.format {
            InputFormat::Htj2kCodestream => Some(input),
            InputFormat::Jph => {
                let container = container::parse(input).map_err(map_container_error)?;
                container
                    .primary_codestream(input)
                    .map_err(map_container_error)?
            }
            _ => None,
        };
        let Some(codestream_bytes) = codestream_bytes else {
            return Ok(false);
        };
        #[cfg(feature = "std")]
        {
            let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
            return Ok(codestream::is_htj2k_lossless_profile(
                codestream_bytes,
                &parsed,
            ));
        }
        #[cfg(not(feature = "std"))]
        {
            let _ = codestream_bytes;
            return Ok(false);
        }
    }

    Ok(primary_part1_codestream_bytes(input, metadata)?
        .is_some_and(codestream::is_owned_baseline_profile))
}

fn native_decode_unsupported(metadata: &Metadata, options: &DecodeOptions) -> J2kError {
    let feature = match metadata.format {
        InputFormat::Jp2 | InputFormat::J2kCodestream => match options.mode {
            DecodeMode::Rendered => UnsupportedFeature::EntropyCoder,
            DecodeMode::Components => UnsupportedFeature::ComponentLayout,
        },
        _ => UnsupportedFeature::InputFormat,
    };
    unsupported(
        feature,
        "native decode coverage is limited to structurally admitted algorithmic Part 1 and HTJ2K profiles; unsupported inputs are not routed to a third-party codec",
    )
}

fn decode_shape_from_metadata(metadata: &Metadata, options: &DecodeOptions) -> Result<DecodeShape> {
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::ComponentLayout,
            "decode shape requires image metadata before decode support can be enabled",
        )
    })?;
    let colour_channels = match options.mode {
        DecodeMode::Rendered => colour_channel_count(image.color_model, image.components)?,
        DecodeMode::Components => {
            colour_channel_count(image.color_model, image.components).unwrap_or(image.components)
        }
    };
    let output_components = match (&options.mode, &options.requested_components) {
        (DecodeMode::Rendered, _) => colour_channels,
        (DecodeMode::Components, ComponentSelection::All) => image.components,
        (DecodeMode::Components, ComponentSelection::Indices(indices)) => {
            u16::try_from(indices.len()).map_err(|_| sample_size_overflow())?
        }
    };

    Ok(DecodeShape {
        width: image.width,
        height: image.height,
        codestream_components: image.components,
        colour_channels,
        output_components,
        sample_format: image.sample_format,
        layout: options.target_layout,
        byte_order: image.sample_format.byte_order,
        color_model: match options.mode {
            DecodeMode::Rendered => image.color_model,
            DecodeMode::Components => {
                component_decode_color_model(image.color_model, image.components)
            }
        },
        mode: options.mode,
    })
}

fn colour_channel_count(color_model: ColorModel, components: u16) -> Result<u16> {
    match color_model {
        ColorModel::Grayscale => Ok(1),
        ColorModel::Rgb | ColorModel::YCbCr => Ok(3),
        ColorModel::Rgba => Ok(4),
        ColorModel::Unknown if (1..=4).contains(&components) => Ok(components),
        ColorModel::Unknown => Err(unsupported(
            UnsupportedFeature::ColorModel,
            "rendered channel count cannot be resolved for unknown color models",
        )),
    }
}

fn component_decode_color_model(color_model: ColorModel, components: u16) -> ColorModel {
    match (color_model, components) {
        (ColorModel::Grayscale, 1) => ColorModel::Grayscale,
        (ColorModel::Rgb, 3) => ColorModel::Rgb,
        _ => ColorModel::Unknown,
    }
}

fn decoded_baseline_to_image_with_component_info(
    decoded: codestream::DecodedImage,
    options: &DecodeOptions,
    component_info: Option<Vec<ComponentInfo>>,
) -> Result<Image> {
    let component_len = decoded.components.len();
    if component_len == 0 || component_len > usize::from(u16::MAX) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "owned baseline decode returned an invalid component-plane count",
        ));
    }
    if options.mode == DecodeMode::Rendered && !matches!(component_len, 1 | 3) {
        return Err(unsupported(
            UnsupportedFeature::ColorModel,
            "rendered baseline decode currently returns grayscale or RGB output",
        ));
    }
    let component_count = u16::try_from(component_len).map_err(|_| sample_size_overflow())?;
    let color_model = match (options.mode, &options.requested_components, component_len) {
        (DecodeMode::Components, ComponentSelection::Indices(_), _) => ColorModel::Unknown,
        (_, _, 1) => ColorModel::Grayscale,
        (_, _, 3) => ColorModel::Rgb,
        (DecodeMode::Components, _, _) => ColorModel::Unknown,
        (DecodeMode::Rendered, _, _) => {
            unreachable!("rendered component count was checked above")
        }
    };

    let info = ImageInfo::new(
        decoded.width,
        decoded.height,
        component_count,
        decoded_sample_format(&decoded)?,
        color_model,
        options.target_layout,
    )?;
    let planes = decoded
        .components
        .into_iter()
        .map(|component| component.samples)
        .collect::<Vec<_>>();
    let component_info = component_info
        .unwrap_or_else(|| uniform_component_info(&info, options.mode == DecodeMode::Components));
    if component_info.len() != component_len {
        return Err(J2kError::InternalInvariant {
            message: "decoded component metadata count did not match output planes".into(),
        });
    }
    if options.target_layout == ComponentLayout::Interleaved
        && component_info.iter().any(|component| {
            component.width != decoded.width
                || component.height != decoded.height
                || component.sample_format != info.sample_format
        })
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "interleaved component output requires identical dimensions and sample formats; use planar output for heterogeneous components",
        ));
    }

    match options.target_layout {
        ComponentLayout::Planar => Ok(Image {
            component_info,
            info,
            data: ImageData::Planes(planes),
        }),
        ComponentLayout::Interleaved => {
            let samples = if planes.len() == 1 {
                planes.into_iter().next().ok_or_else(sample_size_overflow)?
            } else {
                interleave_planes(&planes, decoded.width, decoded.height, info.sample_format)?
            };
            Ok(Image {
                data: ImageData::Interleaved(samples),
                component_info,
                info,
            })
        }
    }
}

fn is_direct_selective_part1_component_profile(codestream_bytes: &[u8]) -> bool {
    codestream::parse(codestream_bytes).is_ok_and(|parsed| {
        let native_subsampled =
            codestream::is_supported_part1_native_subsampled_component_profile(&parsed);
        (native_subsampled
            && parsed
                .uniform_effective_coding_style()
                .is_some_and(|style| style.transform == codestream::WaveletTransform::Reversible53))
            || (!native_subsampled
                && (codestream::is_owned_baseline_profile(codestream_bytes)
                    || codestream::is_supported_part1_bounded_poc_component_profile(
                        codestream_bytes,
                        &parsed,
                    )
                    || codestream::is_supported_part1_selective_irreversible97_component_profile(
                        codestream_bytes,
                        &parsed,
                    )))
    })
}

fn is_native_multitile_partial_profile(
    metadata: &Metadata,
    codestream_bytes: &[u8],
    codestream: &codestream::Codestream,
) -> bool {
    metadata.format == InputFormat::J2kCodestream
        && codestream::is_supported_part1_native_multitile_partial_profile(
            codestream_bytes,
            codestream,
        )
}

fn has_multiple_part1_tiles(codestream: &codestream::Codestream) -> bool {
    codestream
        .siz
        .tile_count_x()
        .ok()
        .zip(codestream.siz.tile_count_y().ok())
        .is_some_and(|(x, y)| x > 1 || y > 1)
}

fn part1_component_info(
    codestream_bytes: &[u8],
    selection: &ComponentSelection,
    region: Option<Region>,
) -> Result<Vec<ComponentInfo>> {
    part1_component_info_at_resolution(codestream_bytes, selection, region, 0)
}

fn part1_component_info_at_resolution(
    codestream_bytes: &[u8],
    selection: &ComponentSelection,
    region: Option<Region>,
    discard_levels: u8,
) -> Result<Vec<ComponentInfo>> {
    let codestream = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    let image = codestream
        .siz
        .image_reference_rect()
        .map_err(map_codestream_error)?;
    let reference_region = match region {
        Some(region) => codestream::geometry::ReferenceGridRect::from_image_relative(
            image,
            region.x,
            region.y,
            region.width,
            region.height,
        )
        .map_err(map_codestream_error)?,
        None => image,
    };
    let component_indices = match selection {
        ComponentSelection::All => (0..codestream.siz.component_count()).collect::<Vec<_>>(),
        ComponentSelection::Indices(indices) => indices.clone(),
    };
    component_indices
        .into_iter()
        .map(|component_index| {
            let component = codestream
                .siz
                .components
                .get(usize::from(component_index))
                .ok_or_else(sample_size_overflow)?;
            let component_region = reference_region
                .to_component_grid(
                    component.horizontal_separation,
                    component.vertical_separation,
                )
                .map_err(|_| J2kError::InvalidParameter {
                    parameter: "region",
                    message:
                        "non-empty reference region maps to an empty native component rectangle",
                })?;
            let (x_origin, y_origin, width, height) = if discard_levels == 0 {
                (
                    component_region.x0(),
                    component_region.y0(),
                    component_region.width(),
                    component_region.height(),
                )
            } else {
                let reduced = component_region
                    .reduce(discard_levels)
                    .map_err(map_codestream_error)?;
                (
                    reduced.x0(),
                    reduced.y0(),
                    reduced.width(),
                    reduced.height(),
                )
            };
            let byte_order = (component.bits_per_sample > 8).then_some(SampleEndian::Little);
            Ok(ComponentInfo {
                source_component: Some(component_index),
                width,
                height,
                x_origin,
                y_origin,
                horizontal_separation: component.horizontal_separation,
                vertical_separation: component.vertical_separation,
                sample_format: SampleFormat::with_byte_order(
                    component.bits_per_sample,
                    component.signed,
                    byte_order,
                )?,
            })
        })
        .collect()
}

fn direct_part1_region(
    codestream: &codestream::Codestream,
    options: &PartialDecodeOptions,
) -> Result<Region> {
    if options.tile.is_some() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "direct selective Part 1 tile decode requires an admitted high-level raw-codestream route",
        ));
    }
    direct_part1_discard_levels(codestream, options)?;
    if options.max_quality_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }
    let region = options.region.unwrap_or(Region {
        x: 0,
        y: 0,
        width: codestream.image_width(),
        height: codestream.image_height(),
    });
    if region.width == 0 || region.height == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "region",
            message: "partial decode region dimensions must be greater than zero",
        });
    }
    let end_x = region
        .x
        .checked_add(region.width)
        .ok_or_else(sample_size_overflow)?;
    let end_y = region
        .y
        .checked_add(region.height)
        .ok_or_else(sample_size_overflow)?;
    if end_x > codestream.image_width() || end_y > codestream.image_height() {
        return Err(J2kError::InvalidParameter {
            parameter: "region",
            message: "partial decode region must fit inside the image bounds",
        });
    }
    Ok(region)
}

fn direct_part1_tile_region(siz: &codestream::SizMarker, tile: TileSelection) -> Result<Region> {
    let tile_count_x = siz.tile_count_x().map_err(map_codestream_error)?;
    let tile_count_y = siz.tile_count_y().map_err(map_codestream_error)?;
    if tile.tile_x >= tile_count_x || tile.tile_y >= tile_count_y {
        return Err(J2kError::InvalidParameter {
            parameter: "tile",
            message: "requested tile is outside the codestream tile grid",
        });
    }
    let nominal_x = tile
        .tile_x
        .checked_mul(siz.tile_width)
        .and_then(|offset| siz.tile_origin_x.checked_add(offset))
        .ok_or_else(sample_size_overflow)?;
    let nominal_y = tile
        .tile_y
        .checked_mul(siz.tile_height)
        .and_then(|offset| siz.tile_origin_y.checked_add(offset))
        .ok_or_else(sample_size_overflow)?;
    let nominal_end_x = nominal_x
        .checked_add(siz.tile_width)
        .ok_or_else(sample_size_overflow)?;
    let nominal_end_y = nominal_y
        .checked_add(siz.tile_height)
        .ok_or_else(sample_size_overflow)?;
    let x0 = nominal_x.max(siz.image_origin_x);
    let y0 = nominal_y.max(siz.image_origin_y);
    let x1 = nominal_end_x.min(siz.reference_grid_width);
    let y1 = nominal_end_y.min(siz.reference_grid_height);
    let width =
        x1.checked_sub(x0)
            .filter(|width| *width != 0)
            .ok_or(J2kError::InvalidParameter {
                parameter: "tile",
                message: "requested tile does not intersect the codestream image",
            })?;
    let height =
        y1.checked_sub(y0)
            .filter(|height| *height != 0)
            .ok_or(J2kError::InvalidParameter {
                parameter: "tile",
                message: "requested tile does not intersect the codestream image",
            })?;
    Ok(Region {
        x: x0
            .checked_sub(siz.image_origin_x)
            .ok_or_else(sample_size_overflow)?,
        y: y0
            .checked_sub(siz.image_origin_y)
            .ok_or_else(sample_size_overflow)?,
        width,
        height,
    })
}

fn validate_native_multitile_partial_options(
    metadata: &Metadata,
    codestream_bytes: &[u8],
    codestream: &codestream::Codestream,
    options: &PartialDecodeOptions,
) -> Result<(Region, Vec<u16>)> {
    if !is_native_multitile_partial_profile(metadata, codestream_bytes, codestream) {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "input is outside the bounded native multi-tile partial profile",
        ));
    }
    if options.target_layout != ComponentLayout::Planar {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "native multi-tile partial decode requires planar output",
        ));
    }
    if !matches!(
        options.resolution,
        ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 }
    ) {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "native multi-tile partial decode supports full resolution only",
        ));
    }
    if !matches!(options.max_quality_layers, None | Some(1)) {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "native multi-tile partial decode has one quality layer and does not admit truncation claims",
        ));
    }
    let component_indices = direct_part1_component_indices(codestream, &options.components)?;
    if component_indices.as_slice() != [0] {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "native multi-tile partial decode selects its sole component",
        ));
    }
    let region = match (options.region, options.tile) {
        (Some(_), Some(_)) => {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "a native partial request must select either a region or a tile, not both",
            ));
        }
        (None, Some(tile)) => direct_part1_tile_region(&codestream.siz, tile)?,
        (_, None) => direct_part1_region(codestream, options)?,
    };
    Ok((region, component_indices))
}

fn direct_part1_discard_levels(
    codestream: &codestream::Codestream,
    options: &PartialDecodeOptions,
) -> Result<u8> {
    match options.resolution {
        ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 } => Ok(0),
        ResolutionLevel::Reduced {
            discard_levels: discard_levels @ (1 | 2),
        } => {
            let coding_style = codestream.uniform_effective_coding_style().ok_or_else(|| {
                unsupported(
                    UnsupportedFeature::PartialDecodeMode,
                    "reduced subsampled selective decode requires one uniform coding style",
                )
            })?;
            if coding_style.transform != codestream::WaveletTransform::Reversible53
                || coding_style.decomposition_levels < discard_levels
                || coding_style.multiple_component_transform
            {
                return Err(unsupported(
                    UnsupportedFeature::PartialDecodeMode,
                    "reduced subsampled selective decode requires reversible 5/3 coding with at least the requested decomposition count and no MCT",
                ));
            }
            Ok(discard_levels)
        }
        ResolutionLevel::Reduced { .. } => Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "subsampled selective Part 1 component decode supports at most two discarded resolution levels",
        )),
    }
}

fn direct_part1_component_indices(
    codestream: &codestream::Codestream,
    selection: &ComponentSelection,
) -> Result<Vec<u16>> {
    match selection {
        ComponentSelection::All => Ok((0..codestream.siz.component_count()).collect()),
        ComponentSelection::Indices(indices) => {
            if indices.is_empty() {
                return Err(J2kError::InvalidParameter {
                    parameter: "components",
                    message: "component subset must contain at least one component index",
                });
            }
            let mut selected = Vec::with_capacity(indices.len());
            for index in indices {
                if *index >= codestream.siz.component_count() {
                    return Err(J2kError::InvalidParameter {
                        parameter: "components",
                        message: "component index is outside the decoded component range",
                    });
                }
                if selected.contains(index) {
                    return Err(J2kError::InvalidParameter {
                        parameter: "components",
                        message: "component subset must not contain duplicate indices",
                    });
                }
                selected.push(*index);
            }
            Ok(selected)
        }
    }
}

fn uniform_component_info(info: &ImageInfo, source_components: bool) -> Vec<ComponentInfo> {
    (0..info.components)
        .map(|component_index| ComponentInfo {
            source_component: source_components.then_some(component_index),
            width: info.width,
            height: info.height,
            x_origin: 0,
            y_origin: 0,
            horizontal_separation: 1,
            vertical_separation: 1,
            sample_format: info.sample_format,
        })
        .collect()
}

fn decoded_sample_format(decoded: &codestream::DecodedImage) -> Result<SampleFormat> {
    let byte_order = if decoded.bits_per_sample <= 8 {
        None
    } else {
        Some(SampleEndian::Little)
    };
    SampleFormat::with_byte_order(decoded.bits_per_sample, decoded.signed, byte_order)
}

/// Full decode into caller-owned buffers.
///
/// Supported non-MCT Part 1 component requests write selected planar samples
/// directly into the provided rows, including padded strides, without first
/// allocating a second full output image. Other profiles remain conservative
/// caller-owned-buffer adapters over [`decode`].
pub fn decode_into(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &DecodeOptions,
) -> Result<()> {
    let mut workspace = Part1DecodeWorkspace::new();
    decode_into_with_workspace(input, target, options, &mut workspace)
}

/// Full decode into caller-owned buffers with reusable selective Part 1
/// reconstruction scratch.
///
/// Profiles that do not use the direct Part 1 component route retain their
/// existing behavior and leave this workspace unused.
pub fn decode_into_with_workspace(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &DecodeOptions,
    workspace: &mut Part1DecodeWorkspace,
) -> Result<()> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    validate_image_view_mut(target)?;

    let mut owned_options = options.clone();
    owned_options.target_layout = match target {
        ImageViewMut::Planar { .. } => ComponentLayout::Planar,
        ImageViewMut::Interleaved { .. } => ComponentLayout::Interleaved,
    };
    let expected_shape = decode_shape(input, &owned_options)?;
    let expected_info = expected_shape.image_info()?;
    validate_decode_target(&expected_info, target)?;
    if decode_part1_components_into_direct(input, target, &owned_options, workspace)? {
        return Ok(());
    }
    let decoded = decode(input, &owned_options)?;
    copy_image_into_target(&decoded, target)
}

fn decode_part1_components_into_direct(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &DecodeOptions,
    workspace: &mut Part1DecodeWorkspace,
) -> Result<bool> {
    if options.mode != DecodeMode::Components {
        return Ok(false);
    }
    let ImageViewMut::Planar { planes, .. } = target else {
        return Ok(false);
    };
    let metadata = inspect(input, &InspectOptions::default())?;
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, &metadata)? else {
        return Ok(false);
    };
    if !is_direct_selective_part1_component_profile(codestream_bytes) {
        return Ok(false);
    }
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if codestream::is_supported_part1_native_subsampled_component_profile(&parsed) {
        return Ok(false);
    }
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Ok(false);
    }
    let component_indices = requested_component_indices(&metadata, &options.requested_components)?;
    let mut output_planes = planes
        .iter_mut()
        .map(|plane| codestream::ComponentPlaneMut {
            samples: &mut *plane.samples,
            stride_bytes: plane.stride_bytes,
        })
        .collect::<Vec<_>>();
    codestream::decode_part1_component_request_into_with_workspace(
        codestream_bytes,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &component_indices,
            region: codestream::TileRegionRequest {
                x: 0,
                y: 0,
                width: parsed.image_width(),
                height: parsed.image_height(),
            },
            discard_levels: 0,
            max_layers: options.max_quality_layers,
        },
        &mut output_planes,
        &mut workspace.codestream,
    )
    .map_err(map_codestream_error)?;
    Ok(true)
}

/// Scoped partial decode that owns its returned buffers.
///
/// Admitted direct Part 1 profiles execute their selected packet/code-block and
/// synthesis work into the requested output geometry. Other supported inputs
/// retain the compatibility full-decode-and-crop route.
pub fn decode_partial(input: &[u8], options: &PartialDecodeOptions) -> Result<Image> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    if options.max_quality_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_partial_quality_layer_profile(input, &metadata, options)?;
    if let Some(image) =
        decode_owned_part1_p0_08_heterogeneous_reversible(input, &metadata, options)?
    {
        return Ok(image);
    }
    if let Some(image) = decode_owned_part1_p0_07_progression_change(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) = decode_owned_part1_reduced_reversible_mct(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) = decode_owned_part1_reduced_irreversible_mct(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) =
        decode_owned_part1_reduced_heterogeneous_irreversible(input, &metadata, options)?
    {
        return Ok(image);
    }
    if let Some(image) = decode_owned_part1_reduced_roi_irreversible(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) = decode_owned_selective_part1_discard(input, &metadata, options)? {
        return Ok(image);
    }
    if let Some(image) = decode_owned_selective_part1_partial(input, &metadata, options)? {
        return Ok(image);
    }
    if options.max_quality_layers.is_some() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "maximum quality-layer selection requires an admitted non-MCT Part 1 component request",
        ));
    }
    if let Some(image) = decode_owned_multitile_partial_region(input, &metadata, options)? {
        return Ok(image);
    }
    let selective_component_profile = options.tile.is_none()
        && options.resolution == ResolutionLevel::Full
        && primary_part1_codestream_bytes(input, &metadata)?
            .is_some_and(is_direct_selective_part1_component_profile);
    if selective_component_profile {
        validate_partial_options_without_support(&metadata, options)?;
    } else {
        validate_partial_options(&metadata, options)?;
    }
    let region = partial_output_region(&metadata, options)?;
    let component_indices = partial_component_indices(&metadata, &options.components)?;
    let mut decode_options = DecodeOptions {
        target_layout: ComponentLayout::Planar,
        ..DecodeOptions::default()
    };
    decode_options.requested_components = ComponentSelection::All;
    let decoded = decode(input, &decode_options)?;
    apply_partial_selection(decoded, region, &component_indices, options.target_layout)
}

fn is_p0_08_output_request(options: &PartialDecodeOptions) -> bool {
    options.region.is_none()
        && options.tile.is_none()
        && options.resolution == ResolutionLevel::Reduced { discard_levels: 5 }
        && matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
        && options.max_quality_layers.is_none()
        && options.target_layout == ComponentLayout::Planar
}

fn decode_owned_part1_p0_08_heterogeneous_reversible(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    if !is_p0_08_output_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_08_heterogeneous_reversible_component_profile(
        codestream_bytes,
        &parsed,
        5,
    ) {
        return Ok(None);
    }
    let decoded =
        codestream::decode_part1_p0_08_heterogeneous_reversible_component_zero(codestream_bytes, 5)
            .map_err(map_codestream_error)?;
    let component_info =
        part1_component_info_at_resolution(codestream_bytes, &options.components, None, 5)?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn is_p0_07_output_request(options: &PartialDecodeOptions) -> bool {
    options.region
        == Some(Region {
            x: 0,
            y: 0,
            width: 128,
            height: 128,
        })
        && options.tile.is_none()
        && options.resolution == ResolutionLevel::Full
        && matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
        && options.max_quality_layers.is_none()
        && options.target_layout == ComponentLayout::Planar
}

fn decode_owned_part1_p0_07_progression_change(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    if !is_p0_07_output_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_07_progression_change_component_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let decoded =
        codestream::decode_part1_p0_07_progression_change_component_zero(codestream_bytes)
            .map_err(map_codestream_error)?;
    let component_info =
        part1_component_info(codestream_bytes, &options.components, options.region)?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn decode_owned_part1_reduced_reversible_mct(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    if options.region.is_some()
        || options.tile.is_some()
        || options.target_layout != ComponentLayout::Planar
        || options.max_quality_layers.is_some()
    {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_reduced_reversible_mct_component_profile(
        codestream_bytes,
        &parsed,
        discard_levels,
    ) {
        return Ok(None);
    }
    let component_indices = partial_component_indices(metadata, &options.components)?;
    let decoded = codestream::decode_part1_reduced_reversible_mct_components_selected(
        codestream_bytes,
        &component_indices,
        discard_levels,
    )
    .map_err(map_codestream_error)?;
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &options.components,
        options.region,
        discard_levels,
    )?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn decode_owned_part1_reduced_irreversible_mct(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    if options.region.is_some()
        || options.tile.is_some()
        || options.target_layout != ComponentLayout::Planar
        || options.max_quality_layers.is_some()
        || !matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
    {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_reduced_irreversible_mct_component_profile(
        codestream_bytes,
        &parsed,
        discard_levels,
    ) {
        return Ok(None);
    }
    let decoded = codestream::decode_part1_reduced_irreversible_mct_component_zero(
        codestream_bytes,
        discard_levels,
    )
    .map_err(map_codestream_error)?;
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &options.components,
        None,
        discard_levels,
    )?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn decode_owned_part1_reduced_heterogeneous_irreversible(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    if options.region.is_some()
        || options.tile.is_some()
        || options.target_layout != ComponentLayout::Planar
        || options.max_quality_layers.is_some()
        || !matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
    {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_reduced_heterogeneous_irreversible_component_profile(
        codestream_bytes,
        &parsed,
        discard_levels,
    ) {
        return Ok(None);
    }
    let decoded = codestream::decode_part1_reduced_heterogeneous_irreversible_component_zero(
        codestream_bytes,
        discard_levels,
    )
    .map_err(map_codestream_error)?;
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &options.components,
        None,
        discard_levels,
    )?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn decode_owned_part1_reduced_roi_irreversible(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    if options.region.is_some()
        || options.tile.is_some()
        || options.target_layout != ComponentLayout::Planar
        || options.max_quality_layers.is_some()
        || !matches!(&options.components, ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16])
    {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_reduced_roi_irreversible_component_profile(
        codestream_bytes,
        &parsed,
        discard_levels,
    ) {
        return Ok(None);
    }
    let decoded = codestream::decode_part1_reduced_roi_irreversible_component_zero(
        codestream_bytes,
        discard_levels,
    )
    .map_err(map_codestream_error)?;
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &options.components,
        None,
        discard_levels,
    )?;
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: options.components.clone(),
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

/// Resolve the exact output image description for a partial decode request
/// without allocating image samples.
///
/// This is the partial-decode counterpart to [`decode_shape`]. Its dimensions
/// describe the requested reference-grid envelope. For planar component mode,
/// callers must use [`decode_partial_component_info`] to allocate each plane
/// when selected components may have unequal native geometry or sample format.
pub fn decode_partial_info(input: &[u8], options: &PartialDecodeOptions) -> Result<ImageInfo> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    partial_decode_target_info(input, options)
}

/// Resolve exact native plane descriptors for a planar partial component
/// request without allocating sample storage.
///
/// `PartialDecodeOptions::region` remains a full-resolution image-relative
/// reference-grid rectangle. Each returned descriptor is its checked mapping
/// to the selected component's native grid. Callers must allocate every plane
/// from its own descriptor rather than treating [`decode_partial_info`] as a
/// common component canvas.
pub fn decode_partial_component_info(
    input: &[u8],
    options: &PartialDecodeOptions,
) -> Result<Vec<ComponentInfo>> {
    if options.target_layout != ComponentLayout::Planar {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "native component descriptors require planar output",
        ));
    }
    Ok(prepare_part1_decode(input, options)?.component_info)
}

fn decode_owned_selective_part1_discard(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let Some(info) = selective_part1_discard_target_info(input, metadata, options)? else {
        return Ok(None);
    };
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let component_indices = partial_component_indices(metadata, &options.components)?;
    let bytes_per_sample = public_bytes_per_sample("sample_format", info.sample_format)?;
    let row_bytes = usize::try_from(info.width)
        .map_err(|_| sample_size_overflow())?
        .checked_mul(bytes_per_sample)
        .ok_or_else(sample_size_overflow)?;
    let plane_len = row_bytes
        .checked_mul(usize::try_from(info.height).map_err(|_| sample_size_overflow())?)
        .ok_or_else(sample_size_overflow)?;
    let mut output = (0..component_indices.len())
        .map(|_| alloc::vec![0_u8; plane_len])
        .collect::<Vec<_>>();
    {
        let mut planes = output
            .iter_mut()
            .map(|samples| {
                PlaneMut::new(
                    samples,
                    info.width,
                    info.height,
                    row_bytes,
                    info.sample_format,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let mut target = ImageViewMut::Planar {
            info: &info,
            planes: &mut planes,
        };
        let mut workspace = Part1DecodeWorkspace::new();
        if !decode_partial_part1_components_into_direct(
            input,
            &mut target,
            options,
            &mut workspace,
        )? {
            return Ok(None);
        }
    }
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial discard decode requires image dimensions",
        )
    })?;
    let source_region = options.region.unwrap_or(Region {
        x: 0,
        y: 0,
        width: image.width,
        height: image.height,
    });
    let discard_levels = match options.resolution {
        ResolutionLevel::Reduced { discard_levels } => discard_levels,
        ResolutionLevel::Full => 0,
    };
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &options.components,
        Some(source_region),
        discard_levels,
    )?;
    if component_info
        .iter()
        .any(|component| component.width != info.width || component.height != info.height)
    {
        return Err(J2kError::InternalInvariant {
            message: "planned native component geometry did not match decoded plane dimensions"
                .into(),
        });
    }
    Ok(Some(Image {
        info,
        component_info,
        data: ImageData::Planes(output),
    }))
}

fn decode_owned_selective_part1_partial(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    if !is_direct_selective_part1_component_profile(codestream_bytes) {
        return Ok(None);
    }
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if is_native_multitile_partial_profile(metadata, codestream_bytes, &parsed) {
        return decode_owned_prepared_part1(input, options).map(Some);
    }
    if has_multiple_part1_tiles(&parsed) && (options.region.is_some() || options.tile.is_some()) {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "spatial multi-tile decode requires the bounded native two-decomposition grayscale profile",
        ));
    }
    if options.tile.is_some() {
        return Ok(None);
    }
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Ok(None);
    }
    let native_subsampled =
        codestream::is_supported_part1_native_subsampled_component_profile(&parsed);
    if native_subsampled && options.target_layout != ComponentLayout::Planar {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "subsampled native components require planar output without implicit resampling",
        ));
    }
    if !native_subsampled
        && !matches!(
            options.resolution,
            ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 }
        )
    {
        return Ok(None);
    }
    if !native_subsampled
        && options.region.is_none()
        && options.max_quality_layers.is_none()
        && matches!(options.components, ComponentSelection::All)
    {
        return Ok(None);
    }

    let (region, component_indices) = if native_subsampled {
        (
            direct_part1_region(&parsed, options)?,
            direct_part1_component_indices(&parsed, &options.components)?,
        )
    } else {
        validate_partial_options_without_support(metadata, options)?;
        (
            partial_output_region(metadata, options)?,
            partial_component_indices(metadata, &options.components)?,
        )
    };
    let discard_levels = direct_part1_discard_levels(&parsed, options)?;
    if native_subsampled && discard_levels != 0 {
        let prepared = prepare_part1_decode(input, options)?;
        let mut output = prepared
            .component_info()
            .iter()
            .map(|component| {
                let bytes_per_sample =
                    public_bytes_per_sample("sample_format", component.sample_format)?;
                let len = usize::try_from(component.width)
                    .map_err(|_| sample_size_overflow())?
                    .checked_mul(
                        usize::try_from(component.height).map_err(|_| sample_size_overflow())?,
                    )
                    .and_then(|samples| samples.checked_mul(bytes_per_sample))
                    .ok_or_else(sample_size_overflow)?;
                Ok(alloc::vec![0_u8; len])
            })
            .collect::<Result<Vec<_>>>()?;
        {
            let mut planes = output
                .iter_mut()
                .zip(prepared.component_info())
                .map(|(samples, component)| {
                    let bytes_per_sample =
                        public_bytes_per_sample("sample_format", component.sample_format)?;
                    let stride = usize::try_from(component.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(bytes_per_sample)
                        .ok_or_else(sample_size_overflow)?;
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        stride,
                        component.sample_format,
                    )
                })
                .collect::<Result<Vec<_>>>()?;
            let mut target = ImageViewMut::Planar {
                info: prepared.info(),
                planes: &mut planes,
            };
            execute_prepared_part1_decode_into_with_workspace(
                &prepared,
                &mut target,
                &mut Part1DecodeWorkspace::new(),
                codestream::PreparedPart1ExecutionOptions::default(),
            )?;
        }
        return Ok(Some(Image {
            info: prepared.info().clone(),
            component_info: prepared.component_info().to_vec(),
            data: ImageData::Planes(output),
        }));
    }
    let decoded = codestream::decode_baseline_owned_component_region_selected_with_max_layers(
        codestream_bytes,
        &component_indices,
        codestream::TileRegionRequest {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        },
        options.max_quality_layers,
    )
    .map_err(map_codestream_error)?;
    let requested_components = if !native_subsampled
        && metadata.image.as_ref().is_some_and(|image| {
            component_indices.len() == usize::from(image.components)
                && component_indices.iter().copied().eq(0..image.components)
        }) {
        ComponentSelection::All
    } else {
        ComponentSelection::Indices(component_indices)
    };
    let decode_options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components,
        target_layout: options.target_layout,
        ..DecodeOptions::default()
    };
    let component_info = part1_component_info(
        codestream_bytes,
        &decode_options.requested_components,
        options.region,
    )?;
    decoded_baseline_to_image_with_component_info(decoded, &decode_options, Some(component_info))
        .map(Some)
}

fn decode_owned_prepared_part1(input: &[u8], options: &PartialDecodeOptions) -> Result<Image> {
    let prepared = prepare_part1_decode(input, options)?;
    let mut output = prepared
        .component_info()
        .iter()
        .map(|component| {
            let bytes_per_sample =
                public_bytes_per_sample("sample_format", component.sample_format)?;
            let len = usize::try_from(component.width)
                .map_err(|_| sample_size_overflow())?
                .checked_mul(usize::try_from(component.height).map_err(|_| sample_size_overflow())?)
                .and_then(|samples| samples.checked_mul(bytes_per_sample))
                .ok_or_else(sample_size_overflow)?;
            Ok(alloc::vec![0_u8; len])
        })
        .collect::<Result<Vec<_>>>()?;
    {
        let mut planes = output
            .iter_mut()
            .zip(prepared.component_info())
            .map(|(samples, component)| {
                let bytes_per_sample =
                    public_bytes_per_sample("sample_format", component.sample_format)?;
                let stride = usize::try_from(component.width)
                    .map_err(|_| sample_size_overflow())?
                    .checked_mul(bytes_per_sample)
                    .ok_or_else(sample_size_overflow)?;
                PlaneMut::new(
                    samples,
                    component.width,
                    component.height,
                    stride,
                    component.sample_format,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        let mut target = ImageViewMut::Planar {
            info: prepared.info(),
            planes: &mut planes,
        };
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut Part1DecodeWorkspace::new(),
            codestream::PreparedPart1ExecutionOptions::default(),
        )?;
    }
    Ok(Image {
        info: prepared.info().clone(),
        component_info: prepared.component_info().to_vec(),
        data: ImageData::Planes(output),
    })
}

#[allow(dead_code)]
pub(crate) fn plan_partial_decode_work(
    input: &[u8],
    options: &PartialDecodeOptions,
) -> Result<PartialDecodeWorkPlan> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    let metadata = inspect(input, &InspectOptions::default())?;
    let codestream_bytes = primary_part1_codestream_bytes(input, &metadata)?;
    if let Some((codestream_bytes, parsed)) = codestream_bytes.and_then(|bytes| {
        codestream::parse(bytes).ok().and_then(|parsed| {
            is_native_multitile_partial_profile(&metadata, bytes, &parsed)
                .then_some((bytes, parsed))
        })
    }) {
        let (region, selected_components) = validate_native_multitile_partial_options(
            &metadata,
            codestream_bytes,
            &parsed,
            options,
        )?;
        return Ok(PartialDecodeWorkPlan {
            request: options.clone(),
            selected_resolution: PlannedResolution {
                discard_levels: 0,
                codestream_resolution_level: codestream_resolution_level(Some(codestream_bytes), 0),
                width: region.width,
                height: region.height,
            },
            full_image_full_resolution_fallback: false,
            selected_tiles: planned_tiles_for_region(Some(codestream_bytes), region)?,
            selected_components,
            work_units: unavailable_partial_work_units(
                "prepared native multi-tile decode retains only intersecting tile, packet and code-block work",
            ),
            evidence: PartialDecodePlanEvidence::TrueCodestreamPartialCandidate,
        });
    }
    let unsupported_spatial_multitile = if options.region.is_some() || options.tile.is_some() {
        codestream_bytes
            .filter(|bytes| is_direct_selective_part1_component_profile(bytes))
            .map(codestream::parse)
            .transpose()
            .map_err(map_codestream_error)?
            .is_some_and(|parsed| has_multiple_part1_tiles(&parsed))
    } else {
        false
    };
    if unsupported_spatial_multitile {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "spatial multi-tile decode requires the bounded native two-decomposition grayscale profile",
        ));
    }
    if let Some(plan) = plan_selective_part1_discard(input, &metadata, options)? {
        return Ok(plan);
    }
    let selective_part1_region = options.region.is_some()
        && options.tile.is_none()
        && options.resolution == ResolutionLevel::Full
        && codestream_bytes.is_some_and(|bytes| {
            is_direct_selective_part1_component_profile(bytes)
                && codestream::parse(bytes).is_ok_and(|parsed| {
                    parsed
                        .uniform_effective_coding_style()
                        .is_some_and(|coding_style| !coding_style.multiple_component_transform)
                })
        });
    if selective_part1_region {
        validate_partial_options_without_support(&metadata, options)?;
    } else {
        validate_partial_options(&metadata, options)?;
    }
    let region = partial_output_region(&metadata, options)?;
    let requested_components = partial_component_indices(&metadata, &options.components)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial decode planning requires image dimensions from metadata inspection",
        )
    })?;
    if selective_part1_region {
        return Ok(PartialDecodeWorkPlan {
            request: options.clone(),
            selected_resolution: PlannedResolution {
                discard_levels: 0,
                codestream_resolution_level: codestream_resolution_level(codestream_bytes, 0),
                width: region.width,
                height: region.height,
            },
            full_image_full_resolution_fallback: false,
            selected_tiles: planned_tiles_for_region(codestream_bytes, region)?,
            selected_components: requested_components,
            work_units: unavailable_partial_work_units(
                "selective Part 1 region decode does not yet expose packet/code-block jobs through the core work plan",
            ),
            evidence: PartialDecodePlanEvidence::TrueCodestreamPartialCandidate,
        });
    }
    let decoded_components = (0..image.components).collect();

    Ok(PartialDecodeWorkPlan {
        request: options.clone(),
        selected_resolution: PlannedResolution {
            discard_levels: 0,
            codestream_resolution_level: metadata
                .codestream
                .as_ref()
                .and_then(|_| codestream_resolution_level(codestream_bytes, 0)),
            width: image.width,
            height: image.height,
        },
        full_image_full_resolution_fallback: true,
        selected_tiles: planned_tiles_for_region(codestream_bytes, region)?,
        selected_components: decoded_components,
        work_units: unavailable_partial_work_units(
            "full-decode-backed adapter does not expose packet or code-block work units",
        ),
        evidence: PartialDecodePlanEvidence::FullDecodeBackedAdapter,
    })
}

fn plan_selective_part1_discard(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<PartialDecodeWorkPlan>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    let codestream_bytes = primary_part1_codestream_bytes(input, metadata)?;
    let native_subsampled = matches!(discard_levels, 1 | 2)
        && codestream_bytes
            .and_then(|bytes| codestream::parse(bytes).ok())
            .is_some_and(|parsed| {
                codestream::is_supported_part1_native_subsampled_component_profile(&parsed)
            });
    let info = if native_subsampled {
        partial_decode_target_info(input, options)?
    } else {
        let Some(info) = selective_part1_discard_target_info(input, metadata, options)? else {
            return Ok(None);
        };
        info
    };
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial discard planning requires image dimensions",
        )
    })?;
    let selected_components = partial_component_indices(metadata, &options.components)?;
    let selected_region = options.region.unwrap_or(Region {
        x: 0,
        y: 0,
        width: image.width,
        height: image.height,
    });
    Ok(Some(PartialDecodeWorkPlan {
        request: options.clone(),
        selected_resolution: PlannedResolution {
            discard_levels,
            codestream_resolution_level: codestream_resolution_level(
                codestream_bytes,
                discard_levels,
            ),
            width: info.width,
            height: info.height,
        },
        full_image_full_resolution_fallback: false,
        selected_tiles: planned_tiles_for_region(codestream_bytes, selected_region)?,
        selected_components,
        work_units: unavailable_partial_work_units(
            "selective Part 1 discard parses all packet headers but excludes higher-resolution code-block jobs before Tier-1",
        ),
        evidence: PartialDecodePlanEvidence::TrueCodestreamPartialCandidate,
    }))
}

fn reduced_part1_region(
    siz: &codestream::SizMarker,
    region: Region,
    discard_levels: u8,
) -> Result<Region> {
    let reduced = siz
        .absolute_reference_region(codestream::TileRegionRequest {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        })
        .and_then(|region| region.to_component_grid(1, 1))
        .and_then(|region| region.reduce(discard_levels))
        .map_err(map_codestream_error)?;
    Ok(Region {
        x: reduced.x0(),
        y: reduced.y0(),
        width: reduced.width(),
        height: reduced.height(),
    })
}

fn codestream_resolution_level(codestream_bytes: Option<&[u8]>, discard_levels: u8) -> Option<u8> {
    let codestream = codestream::parse(codestream_bytes?).ok()?;
    let decomposition_levels = codestream
        .uniform_effective_coding_style()?
        .decomposition_levels;
    decomposition_levels.checked_sub(discard_levels)
}

fn planned_tiles_for_region(
    codestream_bytes: Option<&[u8]>,
    region: Region,
) -> Result<Vec<PlannedPartialTile>> {
    let Some(codestream_bytes) = codestream_bytes else {
        return Ok(Vec::new());
    };
    let codestream = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    let tile_plan = codestream::plan_tile_region_decode(
        &codestream,
        codestream::TileRegionRequest {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        },
    )
    .map_err(map_codestream_error)?;
    Ok(tile_plan
        .tiles
        .into_iter()
        .map(|planned| PlannedPartialTile {
            tile_index: planned.tile.tile_index,
            tile_x: planned.tile.tile_x,
            tile_y: planned.tile.tile_y,
            x: planned.tile.x,
            y: planned.tile.y,
            width: planned.tile.width,
            height: planned.tile.height,
        })
        .collect())
}

fn unavailable_partial_work_units(status: &'static str) -> PlannedPartialWorkUnits {
    PlannedPartialWorkUnits {
        packet_detail: WorkUnitDetail::NotAvailableYet { status },
        code_block_detail: WorkUnitDetail::NotAvailableYet { status },
    }
}

fn decode_owned_multitile_partial_region(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<Image>> {
    if metadata.format != InputFormat::J2kCodestream
        || !matches!(
            metadata.support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::PartialDecodeMode,
                ..
            }
        )
        || options.resolution != ResolutionLevel::Full
        || options.tile.is_some()
        || options.components != ComponentSelection::All
    {
        return Ok(None);
    }
    let Some(region) = options.region else {
        return Ok(None);
    };
    if region.x != 2 || region.y != 0 || region.width != 2 || region.height != 2 {
        return Ok(None);
    }

    let decoded = codestream::decode_multitile_grayscale_region_owned(
        input,
        codestream::TileRegionRequest {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        },
    )
    .map_err(map_codestream_error)?;
    let info = ImageInfo::new(
        decoded.width,
        decoded.height,
        1,
        SampleFormat::U8,
        ColorModel::Grayscale,
        options.target_layout,
    )?;
    let plane = decoded
        .components
        .first()
        .ok_or_else(|| J2kError::InternalInvariant {
            message: "owned partial decode returned no component planes".into(),
        })?
        .samples
        .clone();

    match options.target_layout {
        ComponentLayout::Planar => Ok(Some(Image {
            component_info: uniform_component_info(&info, true),
            info,
            data: ImageData::Planes(alloc::vec![plane]),
        })),
        ComponentLayout::Interleaved => Ok(Some(Image {
            component_info: uniform_component_info(&info, true),
            info,
            data: ImageData::Interleaved(plane),
        })),
    }
}

pub fn decode_partial_into(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &PartialDecodeOptions,
) -> Result<()> {
    let mut workspace = Part1DecodeWorkspace::new();
    decode_partial_into_with_workspace(input, target, options, &mut workspace)
}

/// Prepare the direct selective Part 1 route once for repeated execution.
///
/// This API deliberately rejects compatibility/fallback decode profiles: a
/// returned value always represents the packet-indexed, caller-planar route.
pub fn prepare_part1_decode<'a>(
    input: &'a [u8],
    options: &PartialDecodeOptions,
) -> Result<PreparedPart1Decode<'a>> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    if options.target_layout != ComponentLayout::Planar {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "prepared Part 1 decode requires planar component output",
        ));
    }
    let metadata = inspect(input, &InspectOptions::default())?;
    validate_partial_quality_layer_profile(input, &metadata, options)?;
    let codestream_bytes = primary_part1_codestream_bytes(input, &metadata)?.ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "prepared Part 1 decode requires a Part 1 codestream",
        )
    })?;
    if !is_direct_selective_part1_component_profile(codestream_bytes) {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "input is outside the direct selective Part 1 component profile",
        ));
    }
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "prepared selective component decode does not split MCT inputs",
        ));
    }
    let native_multitile_partial =
        is_native_multitile_partial_profile(&metadata, codestream_bytes, &parsed);
    if has_multiple_part1_tiles(&parsed)
        && (options.region.is_some() || options.tile.is_some())
        && !native_multitile_partial
    {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "prepared spatial multi-tile decode requires the bounded native two-decomposition grayscale profile",
        ));
    }
    if options.tile.is_some() && !native_multitile_partial {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "prepared Part 1 tile decode requires the bounded native multi-tile profile",
        ));
    }
    let info = partial_decode_target_info(input, options)?;
    let discard_levels = match options.resolution {
        ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 } => 0,
        ResolutionLevel::Reduced { discard_levels } => discard_levels,
    };
    let native_subsampled =
        codestream::is_supported_part1_native_subsampled_component_profile(&parsed);
    if native_subsampled {
        direct_part1_discard_levels(&parsed, options)?;
    } else if discard_levels == 0 {
        validate_partial_options_without_support(&metadata, options)?;
    } else if selective_part1_discard_target_info(input, &metadata, options)?.is_none() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "input is outside the direct selective Part 1 discard profile",
        ));
    }
    let (region, component_indices) = if native_multitile_partial {
        validate_native_multitile_partial_options(&metadata, codestream_bytes, &parsed, options)?
    } else if native_subsampled {
        (
            direct_part1_region(&parsed, options)?,
            direct_part1_component_indices(&parsed, &options.components)?,
        )
    } else {
        (
            partial_output_region(&metadata, options)?,
            partial_component_indices(&metadata, &options.components)?,
        )
    };
    let component_info = part1_component_info_at_resolution(
        codestream_bytes,
        &ComponentSelection::Indices(component_indices.clone()),
        Some(region),
        discard_levels,
    )?;
    let codestream = codestream::prepare_part1_component_decode(
        codestream_bytes,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &component_indices,
            region: codestream::TileRegionRequest {
                x: region.x,
                y: region.y,
                width: region.width,
                height: region.height,
            },
            discard_levels,
            max_layers: options.max_quality_layers,
        },
    )
    .map_err(map_codestream_error)?;
    Ok(PreparedPart1Decode {
        info,
        component_info,
        codestream,
    })
}

/// Prepare a raw Part 1 codestream from an immutable positioned-read source.
///
/// This is the application boundary for large files and container image
/// segments: bind a [`codestream::source::FileSource`] directly, or wrap it in
/// [`codestream::source::SubrangeSource`] so logical byte zero is the start of
/// a NITF/JP2 codestream subrange. The returned plan borrows and is permanently
/// bound to that source. Container parsing is intentionally outside this raw
/// codestream entry point.
pub fn prepare_part1_decode_from_source<'a>(
    source: &'a dyn codestream::source::CodestreamSource,
    request: codestream::Part1ComponentDecodeRequest<'_>,
) -> Result<PreparedPart1Decode<'a>> {
    let codestream = codestream::prepare_part1_component_decode_from_source(source, request)
        .map_err(map_codestream_error)?;
    if request.max_layers.is_some() && codestream.codestream_declared_quality_layers() != Some(1) {
        let (image_width, image_height) = codestream.codestream_image_dimensions();
        if request.max_layers == Some(0) {
            return Err(J2kError::InvalidParameter {
                parameter: "max_quality_layers",
                message: "maximum quality layers must be at least one",
            });
        }
        if !codestream.codestream_supports_native_quality_layers()
            || request.component_indices != [0]
            || request.region.x != 0
            || request.region.y != 0
            || request.region.width != image_width
            || request.region.height != image_height
            || request.discard_levels != 0
        {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "source-backed quality-layer selection requires the bounded full-image single-component two-layer LRCP profile",
            ));
        }
    }
    if codestream.codestream_has_subsampled_components() {
        if request.discard_levels > 2 {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "source-backed subsampled selective decode supports at most two discarded resolution levels",
            ));
        }
        if request.discard_levels == 2
            && !codestream.codestream_supports_native_subsampled_discard(2)
        {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "two-level source-backed subsampled selective decode requires the bounded reversible native subsampled profile with at least two decompositions",
            ));
        }
    }
    let (width, height) = codestream.output_dimensions();
    let components = u16::try_from(codestream.component_indices().len()).map_err(|_| {
        unsupported(
            UnsupportedFeature::ComponentLayout,
            "source-backed component selection exceeds the public image model",
        )
    })?;
    let first_output = codestream
        .component_outputs()
        .first()
        .ok_or_else(sample_size_overflow)?;
    let sample_format = SampleFormat::with_byte_order(
        first_output.bits_per_sample,
        first_output.signed,
        (first_output.bits_per_sample > 8).then_some(SampleEndian::Little),
    )?;
    let color_model = if components == 1 {
        ColorModel::Grayscale
    } else {
        ColorModel::Unknown
    };
    let info = ImageInfo::new(
        width,
        height,
        components,
        sample_format,
        color_model,
        ComponentLayout::Planar,
    )?;
    let component_info = codestream
        .component_outputs()
        .iter()
        .map(|output| {
            Ok(ComponentInfo {
                source_component: Some(output.component_index),
                width: output.width,
                height: output.height,
                x_origin: output.x_origin,
                y_origin: output.y_origin,
                horizontal_separation: output.horizontal_separation,
                vertical_separation: output.vertical_separation,
                sample_format: SampleFormat::with_byte_order(
                    output.bits_per_sample,
                    output.signed,
                    (output.bits_per_sample > 8).then_some(SampleEndian::Little),
                )?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(PreparedPart1Decode {
        info,
        component_info,
        codestream,
    })
}

/// Execute a prepared Part 1 plan into caller-owned planar rows.
///
/// Output is unspecified after an execution-time entropy decode failure. The
/// target is fully validated before execution, but independent work may already
/// have committed rows when a later block fails. Decode into staging storage
/// when an application requires transactional publication.
pub fn execute_prepared_part1_decode_into_with_workspace(
    prepared: &PreparedPart1Decode<'_>,
    target: &mut ImageViewMut<'_>,
    workspace: &mut Part1DecodeWorkspace,
    options: codestream::PreparedPart1ExecutionOptions,
) -> Result<codestream::DecodeStageTimings> {
    let full_synthesis_options = codestream::FullSynthesisExecutionOptions::from(options);
    execute_prepared_part1_decode_into_with_workspace_and_full_synthesis_options(
        prepared,
        target,
        workspace,
        options,
        full_synthesis_options,
    )
}

/// Execute a prepared Part 1 plan with explicit large full-synthesis policy.
pub fn execute_prepared_part1_decode_into_with_workspace_and_full_synthesis_options(
    prepared: &PreparedPart1Decode<'_>,
    target: &mut ImageViewMut<'_>,
    workspace: &mut Part1DecodeWorkspace,
    options: codestream::PreparedPart1ExecutionOptions,
    full_synthesis_options: codestream::FullSynthesisExecutionOptions,
) -> Result<codestream::DecodeStageTimings> {
    validate_image_view_mut(target)?;
    validate_decode_target_components(&prepared.info, &prepared.component_info, target)?;
    let ImageViewMut::Planar { planes, .. } = target else {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "prepared Part 1 decode requires planar caller output",
        ));
    };
    let mut output_planes = planes
        .iter_mut()
        .map(|plane| codestream::ComponentPlaneMut {
            samples: &mut *plane.samples,
            stride_bytes: plane.stride_bytes,
        })
        .collect::<Vec<_>>();
    codestream::execute_prepared_part1_component_decode_into_with_workspace_and_full_synthesis_options(
        &prepared.codestream,
        &mut output_planes,
        &mut workspace.codestream,
        options,
        full_synthesis_options,
    )
    .map_err(map_codestream_error)
}

/// Partial decode into caller-owned buffers with reusable selective Part 1
/// reconstruction scratch.
pub fn decode_partial_into_with_workspace(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &PartialDecodeOptions,
    workspace: &mut Part1DecodeWorkspace,
) -> Result<()> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    validate_image_view_mut(target)?;

    let mut owned_options = options.clone();
    owned_options.target_layout = match target {
        ImageViewMut::Planar { .. } => ComponentLayout::Planar,
        ImageViewMut::Interleaved { .. } => ComponentLayout::Interleaved,
    };
    let expected_info = partial_decode_target_info(input, &owned_options)?;
    let direct_component_info = if matches!(target, ImageViewMut::Planar { .. }) {
        let metadata = inspect(input, &InspectOptions::default())?;
        primary_part1_codestream_bytes(input, &metadata)?
            .filter(|bytes| is_direct_selective_part1_component_profile(bytes))
            .map(|_| decode_partial_component_info(input, &owned_options))
            .transpose()?
    } else {
        None
    };
    if let Some(component_info) = direct_component_info.as_deref() {
        validate_decode_target_components(&expected_info, component_info, target)?;
    } else {
        validate_decode_target(&expected_info, target)?;
    }
    if decode_partial_part1_components_into_direct(input, target, &owned_options, workspace)? {
        return Ok(());
    }
    let decoded = decode_partial(input, &owned_options)?;
    copy_image_into_target(&decoded, target)
}

fn decode_partial_part1_components_into_direct(
    input: &[u8],
    target: &mut ImageViewMut<'_>,
    options: &PartialDecodeOptions,
    workspace: &mut Part1DecodeWorkspace,
) -> Result<bool> {
    let discard_levels = match options.resolution {
        ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 } => 0,
        ResolutionLevel::Reduced { discard_levels } => discard_levels,
    };
    if !matches!(target, ImageViewMut::Planar { .. }) {
        return Ok(false);
    }
    let metadata = inspect(input, &InspectOptions::default())?;
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, &metadata)? else {
        return Ok(false);
    };
    if !is_direct_selective_part1_component_profile(codestream_bytes) {
        return Ok(false);
    }
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if is_native_multitile_partial_profile(&metadata, codestream_bytes, &parsed) {
        let prepared = prepare_part1_decode(input, options)?;
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            target,
            workspace,
            codestream::PreparedPart1ExecutionOptions::default(),
        )?;
        return Ok(true);
    }
    if options.tile.is_some() {
        return Ok(false);
    }
    let ImageViewMut::Planar { planes, .. } = target else {
        return Ok(false);
    };
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Ok(false);
    }
    let native_subsampled =
        codestream::is_supported_part1_native_subsampled_component_profile(&parsed);
    if native_subsampled {
        direct_part1_discard_levels(&parsed, options)?;
    } else if discard_levels == 0 {
        validate_partial_options_without_support(&metadata, options)?;
    } else if selective_part1_discard_target_info(input, &metadata, options)?.is_none() {
        return Ok(false);
    }
    let region = if native_subsampled {
        direct_part1_region(&parsed, options)?
    } else {
        partial_output_region(&metadata, options)?
    };
    let component_indices = if native_subsampled {
        direct_part1_component_indices(&parsed, &options.components)?
    } else {
        partial_component_indices(&metadata, &options.components)?
    };
    let mut output_planes = planes
        .iter_mut()
        .map(|plane| codestream::ComponentPlaneMut {
            samples: &mut *plane.samples,
            stride_bytes: plane.stride_bytes,
        })
        .collect::<Vec<_>>();
    codestream::decode_part1_component_request_into_with_workspace(
        codestream_bytes,
        codestream::Part1ComponentDecodeRequest {
            component_indices: &component_indices,
            region: codestream::TileRegionRequest {
                x: region.x,
                y: region.y,
                width: region.width,
                height: region.height,
            },
            discard_levels,
            max_layers: options.max_quality_layers,
        },
        &mut output_planes,
        &mut workspace.codestream,
    )
    .map_err(map_codestream_error)?;
    Ok(true)
}

/// Convenience encode that owns the returned codestream or container bytes.
pub fn encode(image: ImageView<'_>, options: &EncodeOptions) -> Result<Vec<u8>> {
    validate_image_view(&image)?;

    let mut output = Vec::new();
    encode_into(image, &mut output, options)?;
    Ok(output)
}

/// Encode a raw lossless HTJ2K codestream through the repo-owned HT block
/// coder.
pub fn encode_htj2k(image: ImageView<'_>, options: &Htj2kEncodeOptions) -> Result<Vec<u8>> {
    validate_image_view(&image)?;
    if options.decomposition_levels != 0 {
        return Err(unsupported(
            UnsupportedFeature::WaveletTransform,
            "HTJ2K encode currently supports no wavelet decomposition",
        ));
    }
    #[cfg(feature = "std")]
    {
        let info = image_info(image);
        validate_htj2k_encode_image_info(info)?;
        if is_native_grayscale_u8_encode(info) {
            return encode_native_htj2k_grayscale_u8_no_decomp(image);
        }
        if is_native_rgb_u8_encode(info) {
            return encode_native_htj2k_rgb_u8_no_decomp(image);
        }
        if is_native_grayscale_u16_le_ht_encode(info) {
            return encode_native_htj2k_grayscale_u16_le_no_decomp(image);
        }
        if is_native_rgb_u16_le_ht_encode(info) {
            return encode_native_htj2k_rgb_u16_le_no_decomp(image);
        }
        Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "HTJ2K encode supports native grayscale/RGB u8 and u16_le input",
        ))
    }

    #[cfg(not(feature = "std"))]
    {
        let _ = image;
        Err(unsupported(
            UnsupportedFeature::EntropyCoder,
            "HTJ2K encode requires the std feature",
        ))
    }
}

/// Encode into a caller-owned output buffer.
pub fn encode_into(
    image: ImageView<'_>,
    output: &mut Vec<u8>,
    options: &EncodeOptions,
) -> Result<()> {
    validate_image_view(&image)?;
    encode_part1_lossless_into(image, output, options)
}

#[cfg(feature = "std")]
fn encode_part1_lossless_into(
    image: ImageView<'_>,
    output: &mut Vec<u8>,
    options: &EncodeOptions,
) -> Result<()> {
    validate_encode_options(options)?;
    let info = image_info(image);
    validate_encode_image_info(info)?;
    if options.tile_size.is_some()
        && !((is_native_grayscale_u8_encode(info)
            || is_native_rgb_u8_encode(info)
            || is_native_grayscale_u16_le_encode(info)
            || is_native_rgb_u16_le_encode(info))
            && options.decomposition_levels == 2)
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "tile-size encode is currently limited to grayscale/RGB u8 or u16_le with exactly two decomposition levels",
        ));
    }
    if matches!(options.decomposition_levels, 1 | 2) {
        if is_native_grayscale_u8_encode(info) {
            let codestream = if let Some(tile_size) = options.tile_size {
                encode_native_grayscale_u8_decomp_multitile(image, tile_size)?
            } else {
                encode_native_grayscale_u8_decomp(image, options.decomposition_levels)?
            };
            match options.format {
                OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
                OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
            }
            return Ok(());
        }
        if is_native_rgb_u8_encode(info) {
            let codestream = if let Some(tile_size) = options.tile_size {
                encode_native_rgb_u8_decomp_multitile(image, tile_size)?
            } else {
                encode_native_rgb_u8_decomp(image, options.decomposition_levels)?
            };
            match options.format {
                OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
                OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
            }
            return Ok(());
        }
        if is_native_grayscale_u16_le_encode(info) {
            let codestream = if let Some(tile_size) = options.tile_size {
                encode_native_grayscale_u16_le_decomp_multitile(image, tile_size)?
            } else {
                encode_native_grayscale_u16_le_decomp(image, options.decomposition_levels)?
            };
            match options.format {
                OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
                OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
            }
            return Ok(());
        }
        if is_native_rgb_u16_le_encode(info) {
            let codestream = if let Some(tile_size) = options.tile_size {
                encode_native_rgb_u16_le_decomp_multitile(image, tile_size)?
            } else {
                encode_native_rgb_u16_le_decomp(image, options.decomposition_levels)?
            };
            match options.format {
                OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
                OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
            }
            return Ok(());
        }
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "decomposition encode currently supports native grayscale/RGB u8 and u16_le input for levels 1 or 2",
        ));
    }
    if is_native_grayscale_u8_encode(info) {
        let codestream = encode_native_grayscale_u8_no_decomp(image)?;
        match options.format {
            OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
            OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
        }
        return Ok(());
    }
    if is_native_rgb_u8_encode(info) {
        let codestream = encode_native_rgb_u8_no_decomp(image)?;
        match options.format {
            OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
            OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
        }
        return Ok(());
    }
    if is_native_grayscale_u16_le_encode(info) {
        let codestream = encode_native_grayscale_u16_le_no_decomp(image)?;
        match options.format {
            OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
            OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
        }
        return Ok(());
    }
    if is_native_rgb_u16_le_encode(info) {
        let codestream = encode_native_rgb_u16_le_no_decomp(image)?;
        match options.format {
            OutputFormat::J2kCodestream => output.extend_from_slice(&codestream),
            OutputFormat::Jp2 => write_jp2_encode_output(info, &codestream, options, output)?,
        }
        return Ok(());
    }

    Err(unsupported(
        UnsupportedFeature::ComponentLayout,
        "baseline encode supports only native grayscale/RGB u8 and u16_le no-decomposition profiles plus grayscale/RGB u8 and u16_le one- and two-decomposition profiles",
    ))
}

#[cfg(feature = "std")]
fn is_native_grayscale_u8_encode(info: &ImageInfo) -> bool {
    info.components == 1
        && matches!(
            info.color_model,
            ColorModel::Grayscale | ColorModel::Unknown
        )
        && info.sample_format == SampleFormat::U8
}

#[cfg(feature = "std")]
fn is_native_rgb_u8_encode(info: &ImageInfo) -> bool {
    info.components == 3
        && matches!(info.color_model, ColorModel::Rgb | ColorModel::Unknown)
        && info.sample_format == SampleFormat::U8
}

#[cfg(feature = "std")]
fn is_native_grayscale_u16_le_encode(info: &ImageInfo) -> bool {
    info.components == 1
        && matches!(
            info.color_model,
            ColorModel::Grayscale | ColorModel::Unknown
        )
        && info.sample_format == SampleFormat::U16_LE
}

#[cfg(feature = "std")]
fn is_native_rgb_u16_le_encode(info: &ImageInfo) -> bool {
    info.components == 3
        && matches!(info.color_model, ColorModel::Rgb | ColorModel::Unknown)
        && info.sample_format == SampleFormat::U16_LE
}

#[cfg(feature = "std")]
fn is_native_grayscale_u16_le_ht_encode(info: &ImageInfo) -> bool {
    info.components == 1
        && matches!(
            info.color_model,
            ColorModel::Grayscale | ColorModel::Unknown
        )
        && is_unsigned_u16_le_precision(info.sample_format)
}

#[cfg(feature = "std")]
fn is_native_rgb_u16_le_ht_encode(info: &ImageInfo) -> bool {
    info.components == 3
        && matches!(info.color_model, ColorModel::Rgb | ColorModel::Unknown)
        && is_unsigned_u16_le_precision(info.sample_format)
}

#[cfg(feature = "std")]
fn is_unsigned_u16_le_precision(format: SampleFormat) -> bool {
    (9..=16).contains(&format.bits_per_sample)
        && !format.signed
        && format.byte_order == Some(SampleEndian::Little)
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u8_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::encode_grayscale_u8_no_decomp(codestream::GrayscaleU8Encode {
                width: info.width,
                height: info.height,
                samples: plane.samples,
                stride_bytes: plane.stride_bytes,
            })
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_grayscale_u8_no_decomp(codestream::GrayscaleU8Encode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        })
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_htj2k_grayscale_u8_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    let input = match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::GrayscaleU8Encode {
                width: info.width,
                height: info.height,
                samples: plane.samples,
                stride_bytes: plane.stride_bytes,
            }
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::GrayscaleU8Encode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        },
    };
    codestream::encode_htj2k_grayscale_u8_no_decomp(input).map_err(map_codestream_error)
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u8_decomp(
    image: ImageView<'_>,
    decomposition_levels: u8,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            encode_grayscale_u8_decomp_codestream(
                codestream::GrayscaleU8Encode {
                    width: info.width,
                    height: info.height,
                    samples: plane.samples,
                    stride_bytes: plane.stride_bytes,
                },
                decomposition_levels,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => encode_grayscale_u8_decomp_codestream(
            codestream::GrayscaleU8Encode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            decomposition_levels,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_grayscale_u8_decomp_codestream(
    input: codestream::GrayscaleU8Encode<'_>,
    decomposition_levels: u8,
) -> codestream::Result<Vec<u8>> {
    match decomposition_levels {
        1 => codestream::encode_grayscale_u8_one_decomp(input),
        2 => codestream::encode_grayscale_u8_two_decomp(input),
        _ => Err(codestream::CodestreamError::Unsupported {
            offset: None,
            marker: None,
            construct: codestream::UnsupportedConstruct::WaveletTransform,
            message: "native grayscale u8 decomposition encode supports exactly one or two levels",
        }),
    }
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u8_decomp_multitile(
    image: ImageView<'_>,
    tile_size: TileSize,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    let codestream_tile_size = codestream::TileSize {
        width: tile_size.width,
        height: tile_size.height,
    };
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::encode_grayscale_u8_two_decomp_multitile(
                codestream::GrayscaleU8Encode {
                    width: info.width,
                    height: info.height,
                    samples: plane.samples,
                    stride_bytes: plane.stride_bytes,
                },
                codestream_tile_size,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_grayscale_u8_two_decomp_multitile(
            codestream::GrayscaleU8Encode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            codestream_tile_size,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u8_decomp(image: ImageView<'_>, decomposition_levels: u8) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U8)?;
            encode_rgb_u8_decomp_codestream(
                codestream::RgbU8Encode {
                    width: info.width,
                    height: info.height,
                    samples: &samples,
                    stride_bytes: usize::try_from(info.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(3)
                        .ok_or_else(sample_size_overflow)?,
                },
                decomposition_levels,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => encode_rgb_u8_decomp_codestream(
            codestream::RgbU8Encode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            decomposition_levels,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u8_decomp_multitile(
    image: ImageView<'_>,
    tile_size: TileSize,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    let codestream_tile_size = codestream::TileSize {
        width: tile_size.width,
        height: tile_size.height,
    };
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U8)?;
            codestream::encode_rgb_u8_two_decomp_multitile(
                codestream::RgbU8Encode {
                    width: info.width,
                    height: info.height,
                    samples: &samples,
                    stride_bytes: usize::try_from(info.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(3)
                        .ok_or_else(sample_size_overflow)?,
                },
                codestream_tile_size,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_rgb_u8_two_decomp_multitile(
            codestream::RgbU8Encode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            codestream_tile_size,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_rgb_u8_decomp_codestream(
    input: codestream::RgbU8Encode<'_>,
    decomposition_levels: u8,
) -> codestream::Result<Vec<u8>> {
    match decomposition_levels {
        1 => codestream::encode_rgb_u8_one_decomp(input),
        2 => codestream::encode_rgb_u8_two_decomp(input),
        _ => Err(codestream::CodestreamError::Unsupported {
            offset: None,
            marker: None,
            construct: codestream::UnsupportedConstruct::WaveletTransform,
            message: "native RGB u8 decomposition encode supports exactly one or two levels",
        }),
    }
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u16_le_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::encode_grayscale_u16_le_no_decomp(codestream::GrayscaleU16LeEncode {
                width: info.width,
                height: info.height,
                samples: plane.samples,
                stride_bytes: plane.stride_bytes,
            })
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_grayscale_u16_le_no_decomp(codestream::GrayscaleU16LeEncode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        })
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_htj2k_grayscale_u16_le_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    let input = match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::GrayscaleU16LeEncode {
                width: info.width,
                height: info.height,
                samples: plane.samples,
                stride_bytes: plane.stride_bytes,
            }
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::GrayscaleU16LeEncode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        },
    };
    codestream::encode_htj2k_grayscale_u16_le_no_decomp_with_precision(
        input,
        info.sample_format.bits_per_sample,
    )
    .map_err(map_codestream_error)
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u16_le_decomp(
    image: ImageView<'_>,
    decomposition_levels: u8,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            encode_grayscale_u16_le_decomp_codestream(
                codestream::GrayscaleU16LeEncode {
                    width: info.width,
                    height: info.height,
                    samples: plane.samples,
                    stride_bytes: plane.stride_bytes,
                },
                decomposition_levels,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => encode_grayscale_u16_le_decomp_codestream(
            codestream::GrayscaleU16LeEncode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            decomposition_levels,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_grayscale_u16_le_decomp_multitile(
    image: ImageView<'_>,
    tile_size: TileSize,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    let codestream_tile_size = codestream::TileSize {
        width: tile_size.width,
        height: tile_size.height,
    };
    match image {
        ImageView::Planar { planes, .. } => {
            let plane = planes.first().ok_or(J2kError::InvalidParameter {
                parameter: "planes",
                message: "grayscale encode requires one input plane",
            })?;
            codestream::encode_grayscale_u16_le_two_decomp_multitile(
                codestream::GrayscaleU16LeEncode {
                    width: info.width,
                    height: info.height,
                    samples: plane.samples,
                    stride_bytes: plane.stride_bytes,
                },
                codestream_tile_size,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_grayscale_u16_le_two_decomp_multitile(
            codestream::GrayscaleU16LeEncode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            codestream_tile_size,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_grayscale_u16_le_decomp_codestream(
    input: codestream::GrayscaleU16LeEncode<'_>,
    decomposition_levels: u8,
) -> codestream::Result<Vec<u8>> {
    match decomposition_levels {
        1 => codestream::encode_grayscale_u16_le_one_decomp(input),
        2 => codestream::encode_grayscale_u16_le_two_decomp(input),
        _ => Err(codestream::CodestreamError::Unsupported {
            offset: None,
            marker: None,
            construct: codestream::UnsupportedConstruct::WaveletTransform,
            message: "native grayscale u16 decomposition encode supports exactly one or two levels",
        }),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u8_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U8)?;
            codestream::encode_rgb_u8_no_decomp(codestream::RgbU8Encode {
                width: info.width,
                height: info.height,
                samples: &samples,
                stride_bytes: usize::try_from(info.width)
                    .map_err(|_| sample_size_overflow())?
                    .checked_mul(3)
                    .ok_or_else(sample_size_overflow)?,
            })
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_rgb_u8_no_decomp(codestream::RgbU8Encode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        })
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_htj2k_rgb_u8_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U8)?;
            codestream::encode_htj2k_rgb_u8_no_decomp(codestream::RgbU8Encode {
                width: info.width,
                height: info.height,
                samples: &samples,
                stride_bytes: usize::try_from(info.width)
                    .map_err(|_| sample_size_overflow())?
                    .checked_mul(3)
                    .ok_or_else(sample_size_overflow)?,
            })
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_htj2k_rgb_u8_no_decomp(codestream::RgbU8Encode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        })
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u16_le_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U16_LE)?;
            codestream::encode_rgb_u16_le_no_decomp(codestream::RgbU16LeEncode {
                width: info.width,
                height: info.height,
                samples: &samples,
                stride_bytes: usize::try_from(info.width)
                    .map_err(|_| sample_size_overflow())?
                    .checked_mul(6)
                    .ok_or_else(sample_size_overflow)?,
            })
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_rgb_u16_le_no_decomp(codestream::RgbU16LeEncode {
            width: info.width,
            height: info.height,
            samples,
            stride_bytes,
        })
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_htj2k_rgb_u16_le_no_decomp(image: ImageView<'_>) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, info.sample_format)?;
            codestream::encode_htj2k_rgb_u16_le_no_decomp_with_precision(
                codestream::RgbU16LeEncode {
                    width: info.width,
                    height: info.height,
                    samples: &samples,
                    stride_bytes: usize::try_from(info.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(6)
                        .ok_or_else(sample_size_overflow)?,
                },
                info.sample_format.bits_per_sample,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_htj2k_rgb_u16_le_no_decomp_with_precision(
            codestream::RgbU16LeEncode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            info.sample_format.bits_per_sample,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u16_le_decomp(
    image: ImageView<'_>,
    decomposition_levels: u8,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U16_LE)?;
            encode_rgb_u16_le_decomp_codestream(
                codestream::RgbU16LeEncode {
                    width: info.width,
                    height: info.height,
                    samples: &samples,
                    stride_bytes: usize::try_from(info.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(6)
                        .ok_or_else(sample_size_overflow)?,
                },
                decomposition_levels,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => encode_rgb_u16_le_decomp_codestream(
            codestream::RgbU16LeEncode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            decomposition_levels,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_native_rgb_u16_le_decomp_multitile(
    image: ImageView<'_>,
    tile_size: TileSize,
) -> Result<Vec<u8>> {
    let info = image_info(image);
    let codestream_tile_size = codestream::TileSize {
        width: tile_size.width,
        height: tile_size.height,
    };
    match image {
        ImageView::Planar { planes, .. } => {
            let samples = interleaved_rgb_from_planes(info, planes, SampleFormat::U16_LE)?;
            codestream::encode_rgb_u16_le_two_decomp_multitile(
                codestream::RgbU16LeEncode {
                    width: info.width,
                    height: info.height,
                    samples: &samples,
                    stride_bytes: usize::try_from(info.width)
                        .map_err(|_| sample_size_overflow())?
                        .checked_mul(6)
                        .ok_or_else(sample_size_overflow)?,
                },
                codestream_tile_size,
            )
            .map_err(map_codestream_error)
        }
        ImageView::Interleaved {
            samples,
            stride_bytes,
            ..
        } => codestream::encode_rgb_u16_le_two_decomp_multitile(
            codestream::RgbU16LeEncode {
                width: info.width,
                height: info.height,
                samples,
                stride_bytes,
            },
            codestream_tile_size,
        )
        .map_err(map_codestream_error),
    }
}

#[cfg(feature = "std")]
fn encode_rgb_u16_le_decomp_codestream(
    input: codestream::RgbU16LeEncode<'_>,
    decomposition_levels: u8,
) -> codestream::Result<Vec<u8>> {
    match decomposition_levels {
        1 => codestream::encode_rgb_u16_le_one_decomp(input),
        2 => codestream::encode_rgb_u16_le_two_decomp(input),
        _ => Err(codestream::CodestreamError::Unsupported {
            offset: None,
            marker: None,
            construct: codestream::UnsupportedConstruct::WaveletTransform,
            message: "native RGB u16 decomposition encode supports exactly one or two levels",
        }),
    }
}

#[cfg(feature = "std")]
fn interleaved_rgb_from_planes(
    info: &ImageInfo,
    planes: &[Plane<'_>],
    sample_format: SampleFormat,
) -> Result<Vec<u8>> {
    if planes.len() != 3 {
        return Err(J2kError::InvalidParameter {
            parameter: "planes",
            message: "RGB encode requires exactly three input planes",
        });
    }
    let bytes_per_sample = bytes_per_sample(sample_format)?;
    let row_bytes = checked_row_bytes(info.width, 1, bytes_per_sample)?;
    for plane in planes {
        if plane.width != info.width || plane.height != info.height {
            return Err(J2kError::InvalidParameter {
                parameter: "planes",
                message: "encode plane dimensions must match image info",
            });
        }
        if plane.sample_format != sample_format {
            return Err(J2kError::InvalidParameter {
                parameter: "planes",
                message: "RGB native encode planes must match the image sample format",
            });
        }
        if plane.stride_bytes < row_bytes {
            return Err(J2kError::InvalidParameter {
                parameter: "plane.stride_bytes",
                message: "encode plane stride must be at least one packed row",
            });
        }
    }

    let capacity = pixel_count(info.width, info.height)?
        .checked_mul(3)
        .and_then(|value| value.checked_mul(bytes_per_sample))
        .ok_or_else(sample_size_overflow)?;
    let mut interleaved = Vec::with_capacity(capacity);
    let width = info.width as usize;
    for y in 0..info.height as usize {
        let red_row = plane_row(planes[0], y, row_bytes)?;
        let green_row = plane_row(planes[1], y, row_bytes)?;
        let blue_row = plane_row(planes[2], y, row_bytes)?;
        match bytes_per_sample {
            1 => {
                for x in 0..width {
                    interleaved.push(red_row[x]);
                    interleaved.push(green_row[x]);
                    interleaved.push(blue_row[x]);
                }
            }
            2 => {
                for x in 0..width {
                    let sample_offset = x * 2;
                    interleaved.extend_from_slice(&red_row[sample_offset..sample_offset + 2]);
                    interleaved.extend_from_slice(&green_row[sample_offset..sample_offset + 2]);
                    interleaved.extend_from_slice(&blue_row[sample_offset..sample_offset + 2]);
                }
            }
            _ => {
                for x in 0..width {
                    let sample_offset = x
                        .checked_mul(bytes_per_sample)
                        .ok_or_else(sample_size_overflow)?;
                    interleaved.extend_from_slice(
                        &red_row[sample_offset..sample_offset + bytes_per_sample],
                    );
                    interleaved.extend_from_slice(
                        &green_row[sample_offset..sample_offset + bytes_per_sample],
                    );
                    interleaved.extend_from_slice(
                        &blue_row[sample_offset..sample_offset + bytes_per_sample],
                    );
                }
            }
        }
    }
    Ok(interleaved)
}

/// Convert planar RGB input through the runtime encode layout adapter used by
/// conformance benchmarks.
#[cfg(feature = "std")]
pub fn interleaved_rgb_from_planes_for_bench(
    info: &ImageInfo,
    planes: &[Plane<'_>],
    sample_format: SampleFormat,
) -> Result<Vec<u8>> {
    interleaved_rgb_from_planes(info, planes, sample_format)
}

#[cfg(feature = "std")]
fn plane_row(plane: Plane<'_>, y: usize, row_bytes: usize) -> Result<&[u8]> {
    let row_start = y
        .checked_mul(plane.stride_bytes)
        .ok_or_else(sample_size_overflow)?;
    checked_byte_slice(plane.samples, row_start, row_bytes)
}

#[cfg(not(feature = "std"))]
fn encode_part1_lossless_into(
    image: ImageView<'_>,
    output: &mut Vec<u8>,
    options: &EncodeOptions,
) -> Result<()> {
    let _ = (image, output, options);
    Err(unsupported(
        UnsupportedFeature::OutputFormat,
        "baseline encode requires the std feature in this implementation slice",
    ))
}

#[cfg(feature = "std")]
fn validate_encode_options(options: &EncodeOptions) -> Result<()> {
    if options.progression_order != ProgressionOrder::Lrcp {
        return Err(unsupported(
            UnsupportedFeature::ProgressionOrder,
            "baseline encode currently emits deterministic LRCP progression only",
        ));
    }
    if options.transform != WaveletTransform::Reversible53 {
        return Err(unsupported(
            UnsupportedFeature::WaveletTransform,
            "baseline encode currently emits reversible 5/3 lossless codestreams only",
        ));
    }
    if !matches!(options.quality, EncodeQuality::Lossless) {
        return Err(unsupported(
            UnsupportedFeature::OutputFormat,
            "rate-targeted lossy encode requires an implemented and qualified rate-control design",
        ));
    }
    if options.decomposition_levels > 2 {
        return Err(unsupported(
            UnsupportedFeature::WaveletTransform,
            "baseline encode currently supports decomposition level 0 and grayscale/RGB u8 or u16_le decomposition levels 1 or 2 only",
        ));
    }
    if options.format == OutputFormat::J2kCodestream && !options.metadata.is_empty() {
        return Err(unsupported(
            UnsupportedFeature::ContainerBox,
            "raw J2K codestream encode cannot carry JP2 metadata records",
        ));
    }

    Ok(())
}

#[cfg(feature = "std")]
fn validate_encode_image_info(info: &ImageInfo) -> Result<()> {
    if !matches!(info.components, 1 | 3) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "baseline encode supports grayscale and RGB images only",
        ));
    }
    if !matches!(
        (info.components, info.color_model),
        (1, ColorModel::Grayscale | ColorModel::Unknown)
            | (3, ColorModel::Rgb | ColorModel::Unknown)
    ) {
        return Err(unsupported(
            UnsupportedFeature::ColorModel,
            "baseline encode supports grayscale or RGB color models only",
        ));
    }
    if info.sample_format.signed {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "baseline encode supports unsigned samples only",
        ));
    }
    if info.sample_format.bits_per_sample == 16
        && info.sample_format.byte_order != Some(SampleEndian::Little)
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "baseline encode accepts 16-bit sample buffers as SampleFormat::U16_LE only",
        ));
    }
    if !matches!(info.sample_format.bits_per_sample, 8 | 16) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "baseline encode currently supports 8-bit and 16-bit unsigned samples only",
        ));
    }

    Ok(())
}

#[cfg(feature = "std")]
fn validate_htj2k_encode_image_info(info: &ImageInfo) -> Result<()> {
    if !matches!(info.components, 1 | 3) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "HTJ2K encode supports grayscale and RGB images only",
        ));
    }
    if !matches!(
        (info.components, info.color_model),
        (1, ColorModel::Grayscale | ColorModel::Unknown)
            | (3, ColorModel::Rgb | ColorModel::Unknown)
    ) {
        return Err(unsupported(
            UnsupportedFeature::ColorModel,
            "HTJ2K encode supports grayscale or RGB color models only",
        ));
    }
    if info.sample_format.signed || !(8..=16).contains(&info.sample_format.bits_per_sample) {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "HTJ2K encode supports unsigned precision in 8..=16",
        ));
    }
    if info.sample_format.bits_per_sample > 8
        && info.sample_format.byte_order != Some(SampleEndian::Little)
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "HTJ2K multi-byte input requires little-endian u16 storage",
        ));
    }
    Ok(())
}

#[cfg(feature = "std")]
fn image_info(image: ImageView<'_>) -> &ImageInfo {
    match image {
        ImageView::Planar { info, .. } | ImageView::Interleaved { info, .. } => info,
    }
}

#[cfg(feature = "std")]
fn bytes_per_sample(sample_format: SampleFormat) -> Result<usize> {
    match (sample_format.bits_per_sample, sample_format.signed) {
        (1..=8, false) => Ok(1),
        (9..=16, false) => Ok(2),
        _ => Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "baseline encode accepts unsigned byte-addressable samples up to 16 bits",
        )),
    }
}

#[cfg(feature = "std")]
fn checked_row_bytes(width: u32, components: u16, bytes_per_sample: usize) -> Result<usize> {
    (width as usize)
        .checked_mul(usize::from(components))
        .and_then(|value| value.checked_mul(bytes_per_sample))
        .ok_or_else(sample_size_overflow)
}

#[cfg(feature = "std")]
fn checked_byte_slice(input: &[u8], offset: usize, len: usize) -> Result<&[u8]> {
    let end = offset.checked_add(len).ok_or_else(sample_size_overflow)?;
    input.get(offset..end).ok_or(J2kError::BufferTooSmall {
        required: end,
        provided: input.len(),
    })
}

#[cfg(feature = "std")]
fn write_jp2_encode_output(
    info: &ImageInfo,
    codestream: &[u8],
    options: &EncodeOptions,
    output: &mut Vec<u8>,
) -> Result<()> {
    container::write_signature_box(output).map_err(map_container_error)?;
    container::write_file_type_box(output, container::ContainerKind::Jp2, 0, &[])
        .map_err(map_container_error)?;

    let mut header_children = Vec::new();
    container::write_image_header_box(
        &mut header_children,
        container::ImageHeaderBox {
            width: info.width,
            height: info.height,
            components: info.components,
            bits_per_component: info.sample_format.bits_per_sample - 1,
            compression_type: 7,
            unknown_color_space: false,
            intellectual_property: false,
        },
    )
    .map_err(map_container_error)?;
    container::write_color_specification_box(
        &mut header_children,
        container::ColorSpecificationBox {
            method: container::ColorSpecificationMethod::Enumerated,
            precedence: 0,
            approximation: 0,
            enumerated_color_space: Some(match info.components {
                1 => container::EnumeratedColorSpace::Greyscale,
                3 => container::EnumeratedColorSpace::SRgb,
                _ => container::EnumeratedColorSpace::Unknown(0),
            }),
        },
    )
    .map_err(map_container_error)?;
    container::write_jp2_header_box(output, &header_children).map_err(map_container_error)?;
    write_jp2_metadata_records(output, &options.metadata)?;
    container::write_contiguous_codestream_box(output, codestream).map_err(map_container_error)
}

#[cfg(feature = "std")]
fn write_jp2_metadata_records(output: &mut Vec<u8>, records: &[MetadataRecord]) -> Result<()> {
    for record in records {
        match record.kind {
            MetadataKind::Xml => {
                container::write_box(output, container::boxes::XML, &record.bytes)
                    .map_err(map_container_error)?;
            }
            MetadataKind::Uuid => {
                container::write_box(output, container::boxes::UUID, &record.bytes)
                    .map_err(map_container_error)?;
            }
            MetadataKind::UnknownBox => {
                let box_type = record
                    .label
                    .as_deref()
                    .and_then(fourcc_from_label)
                    .ok_or_else(|| {
                        unsupported(
                            UnsupportedFeature::ContainerBox,
                            "unknown JP2 metadata records require a four-byte box label",
                        )
                    })?;
                container::write_box(output, box_type, &record.bytes)
                    .map_err(map_container_error)?;
            }
            MetadataKind::UnknownMarker => {
                return Err(unsupported(
                    UnsupportedFeature::MarkerSegment,
                    "codestream marker metadata cannot be written through baseline JP2 encode",
                ));
            }
        }
    }

    Ok(())
}

#[cfg(feature = "std")]
fn fourcc_from_label(label: &str) -> Option<container::FourCc> {
    let bytes = label.as_bytes();
    if bytes.len() != 4 {
        return None;
    }
    let mut value = [0_u8; 4];
    value.copy_from_slice(bytes);
    Some(container::FourCc::new(value))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum J2kError {
    InvalidParameter {
        parameter: &'static str,
        message: &'static str,
    },
    InvalidInput {
        offset: Option<u64>,
        message: String,
    },
    TruncatedInput {
        needed: usize,
        remaining: usize,
    },
    Unsupported {
        feature: UnsupportedFeature,
        detail: String,
    },
    BufferTooSmall {
        required: usize,
        provided: usize,
    },
    InternalInvariant {
        message: String,
    },
}

impl fmt::Display for J2kError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { parameter, message } => {
                write!(f, "invalid parameter `{parameter}`: {message}")
            }
            Self::InvalidInput { offset, message } => match offset {
                Some(offset) => write!(f, "invalid input at byte {offset}: {message}"),
                None => write!(f, "invalid input: {message}"),
            },
            Self::TruncatedInput { needed, remaining } => write!(
                f,
                "truncated input: needed at least {needed} more bytes, had {remaining}"
            ),
            Self::Unsupported { feature, detail } => {
                write!(f, "unsupported {feature:?}: {detail}")
            }
            Self::BufferTooSmall { required, provided } => write!(
                f,
                "buffer too small: required at least {required} bytes, provided {provided}"
            ),
            Self::InternalInvariant { message } => {
                write!(f, "internal invariant failed: {message}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for J2kError {}

fn unsupported(feature: UnsupportedFeature, detail: impl Into<String>) -> J2kError {
    J2kError::Unsupported {
        feature,
        detail: detail.into(),
    }
}

fn map_container_error(error: container::ContainerError) -> J2kError {
    match error {
        container::ContainerError::TruncatedInput {
            needed, remaining, ..
        } => J2kError::TruncatedInput {
            needed: needed.saturating_sub(remaining),
            remaining,
        },
        container::ContainerError::Unsupported { message, .. } => {
            unsupported(UnsupportedFeature::InputFormat, message)
        }
        container::ContainerError::InvalidBox {
            offset, message, ..
        } => J2kError::InvalidInput {
            offset: offset.map(|value| value as u64),
            message,
        },
        container::ContainerError::SizeOverflow => J2kError::InvalidInput {
            offset: None,
            message: "container size overflowed parser limits".into(),
        },
    }
}

fn map_codestream_error(error: codestream::CodestreamError) -> J2kError {
    match error {
        codestream::CodestreamError::Source {
            offset, message, ..
        } => J2kError::InvalidInput {
            offset: Some(offset),
            message,
        },
        codestream::CodestreamError::TruncatedInput {
            needed, remaining, ..
        } => J2kError::TruncatedInput {
            needed: needed.saturating_sub(remaining),
            remaining,
        },
        codestream::CodestreamError::InvalidMarker {
            offset, message, ..
        } => J2kError::InvalidInput {
            offset: offset.map(|value| value as u64),
            message: message.into(),
        },
        codestream::CodestreamError::Unsupported {
            construct, message, ..
        } => unsupported(unsupported_feature_from_construct(construct), message),
        codestream::CodestreamError::SizeOverflow => J2kError::InvalidInput {
            offset: None,
            message: "codestream size overflowed parser limits".into(),
        },
    }
}

fn metadata_from_container(
    input: &[u8],
    container: container::Container,
    options: &InspectOptions,
) -> Result<Metadata> {
    let image = image_info_from_container(&container);
    let format = match container.kind {
        container::ContainerKind::Jp2 => InputFormat::Jp2,
        container::ContainerKind::Jph => InputFormat::Jph,
    };
    let primary_codestream = container
        .primary_codestream(input)
        .map_err(map_container_error)?;
    let parsed_codestream = match primary_codestream {
        Some(bytes) => Some(codestream::parse(bytes).map_err(map_codestream_error)?),
        None => None,
    };
    if container.kind == container::ContainerKind::Jp2
        && let Some(codestream) = &parsed_codestream
    {
        validate_jp2_header_against_siz(&container, codestream)?;
    }
    reject_unimplemented_jp2_presentation(&container)?;
    let support = if !options.classify_support {
        SupportStatus::Unknown {
            detail: "support classification was not requested".into(),
        }
    } else {
        match (&container.kind, &parsed_codestream) {
            (container::ContainerKind::Jph, Some(codestream))
                if codestream.kind != codestream::CodestreamKind::Htj2k =>
            {
                SupportStatus::Unsupported {
                    feature: UnsupportedFeature::InputFormat,
                    detail: "JPH containers must carry an HTJ2K codestream for the current decode subset"
                        .into(),
                }
            }
            (_, Some(codestream)) => support_from_codestream(codestream, primary_codestream),
            (_, None) => SupportStatus::Unknown {
                detail: "container parsed without a contiguous codestream box".into(),
            },
        }
    };
    let records = if options.preserve_raw_metadata {
        container
            .metadata
            .iter()
            .map(|record| MetadataRecord {
                kind: match record.kind {
                    container::MetadataBoxKind::Xml => MetadataKind::Xml,
                    container::MetadataBoxKind::Uuid => MetadataKind::Uuid,
                    container::MetadataBoxKind::Unknown => MetadataKind::UnknownBox,
                },
                label: Some(record.box_type.as_ascii_lossy()),
                bytes: record.data.clone(),
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(Metadata {
        format,
        image,
        codestream: parsed_codestream
            .as_ref()
            .map(codestream_info_from_codestream),
        container: Some(ContainerInfo {
            brand: Some(container.file_type.brand.as_ascii_lossy()),
            compatible_brands: container
                .file_type
                .compatible_brands
                .iter()
                .map(|brand| brand.as_ascii_lossy())
                .collect(),
            codestream_count: container.codestreams.len() as u32,
        }),
        support,
        records,
    })
}

fn reject_unimplemented_jp2_presentation(container: &container::Container) -> Result<()> {
    if container.kind != container::ContainerKind::Jp2 {
        return Ok(());
    }
    let Some(header) = container
        .boxes
        .iter()
        .find(|record| record.box_type == container::boxes::JP2_HEADER)
    else {
        return Ok(());
    };
    let header_end = header
        .data_offset
        .checked_add(header.data_len)
        .ok_or_else(|| J2kError::InvalidInput {
            offset: None,
            message: "container size overflowed parser limits".into(),
        })?;
    if let Some(record) = container.boxes.iter().find(|record| {
        record.header_offset >= header.data_offset
            && record.header_offset < header_end
            && matches!(
                record.box_type,
                container::boxes::PALETTE
                    | container::boxes::COMPONENT_MAPPING
                    | container::boxes::CHANNEL_DEFINITION
            )
    }) {
        return Err(unsupported(
            UnsupportedFeature::ContainerBox,
            alloc::format!(
                "JP2 `{}` presentation is not implemented; palette, component mapping, and channel definition remain fail-closed",
                record.box_type
            ),
        ));
    }
    if let Some(colour) = container.color_specification
        && (colour.method != container::ColorSpecificationMethod::Enumerated
            || !matches!(
                colour.enumerated_color_space,
                Some(
                    container::EnumeratedColorSpace::SRgb
                        | container::EnumeratedColorSpace::Greyscale
                        | container::EnumeratedColorSpace::SYcc
                )
            ))
    {
        return Err(unsupported(
            UnsupportedFeature::ColorModel,
            "JP2 ICC, vendor, and unrecognised colour transforms remain fail-closed",
        ));
    }
    Ok(())
}

fn validate_jp2_header_against_siz(
    container: &container::Container,
    codestream: &codestream::Codestream,
) -> Result<()> {
    let image_header = container
        .image_header
        .ok_or_else(|| J2kError::InvalidInput {
            offset: None,
            message: "JP2 header box is missing its image header".into(),
        })?;
    let image_record =
        jp2_child_record(container, container::boxes::IMAGE_HEADER).ok_or_else(|| {
            J2kError::InvalidInput {
                offset: None,
                message: "JP2 image header location is unavailable".into(),
            }
        })?;
    let mismatch = |field_offset: usize, message: &'static str| J2kError::InvalidInput {
        offset: Some((image_record.data_offset + field_offset) as u64),
        message: message.into(),
    };

    if image_header.height != codestream.image_height() {
        return Err(mismatch(
            0,
            "JP2 image header height does not match the first codestream SIZ marker",
        ));
    }
    if image_header.width != codestream.image_width() {
        return Err(mismatch(
            4,
            "JP2 image header width does not match the first codestream SIZ marker",
        ));
    }
    if image_header.components != codestream.siz.component_count() {
        return Err(mismatch(
            8,
            "JP2 image header component count does not match the first codestream SIZ marker",
        ));
    }

    if image_header.bits_per_component == 255 {
        let bits = container
            .bits_per_component
            .as_ref()
            .ok_or_else(|| mismatch(10, "JP2 varying precision is missing component entries"))?;
        let bits_record = jp2_child_record(container, container::boxes::BITS_PER_COMPONENT)
            .ok_or_else(|| mismatch(10, "JP2 bits-per-component location is unavailable"))?;
        for (index, (header_component, siz_component)) in bits
            .components
            .iter()
            .zip(&codestream.siz.components)
            .enumerate()
        {
            if header_component.bits_per_sample != siz_component.bits_per_sample
                || header_component.signed != siz_component.signed
            {
                return Err(J2kError::InvalidInput {
                    offset: Some((bits_record.data_offset + index) as u64),
                    message: alloc::format!(
                        "JP2 bits-per-component entry {index} does not match the first codestream SIZ marker"
                    ),
                });
            }
        }
    } else {
        let header_format = image_header
            .sample_format()
            .ok_or_else(|| mismatch(10, "JP2 image header precision is invalid"))?;
        if codestream.siz.components.iter().any(|component| {
            component.bits_per_sample != header_format.bits_per_sample
                || component.signed != header_format.signed
        }) {
            return Err(mismatch(
                10,
                "JP2 image header precision and signedness do not match every first-codestream SIZ component",
            ));
        }
    }
    Ok(())
}

fn jp2_child_record(
    parsed: &container::Container,
    box_type: container::FourCc,
) -> Option<&container::BoxRecord> {
    let header = parsed
        .boxes
        .iter()
        .find(|record| record.box_type == container::boxes::JP2_HEADER)?;
    let header_end = header.data_offset.checked_add(header.data_len)?;
    parsed.boxes.iter().find(|record| {
        record.box_type == box_type
            && record.header_offset >= header.data_offset
            && record.header_offset < header_end
    })
}

fn metadata_from_codestream(
    input: &[u8],
    codestream: codestream::Codestream,
    options: &InspectOptions,
) -> Metadata {
    let format = match codestream.kind {
        codestream::CodestreamKind::J2k => InputFormat::J2kCodestream,
        codestream::CodestreamKind::Htj2k => InputFormat::Htj2kCodestream,
    };
    let image = image_info_from_codestream(&codestream);
    let support = if options.classify_support {
        support_from_codestream(&codestream, Some(input))
    } else {
        SupportStatus::Unknown {
            detail: "support classification was not requested".into(),
        }
    };

    Metadata {
        format,
        image,
        codestream: Some(codestream_info_from_codestream(&codestream)),
        container: None,
        support,
        records: Vec::new(),
    }
}

fn codestream_info_from_codestream(codestream: &codestream::Codestream) -> CodestreamInfo {
    let coding_style = codestream.uniform_effective_coding_style();
    CodestreamInfo {
        kind: codestream.kind,
        tile_grid: Some(TileGrid {
            tile_width: codestream.siz.tile_width,
            tile_height: codestream.siz.tile_height,
            tile_origin_x: codestream.siz.tile_origin_x,
            tile_origin_y: codestream.siz.tile_origin_y,
        }),
        progression_order: coding_style
            .map(|coding_style| progression_order_from_codestream(coding_style.progression_order)),
        transform: coding_style
            .map(|coding_style| transform_from_codestream(coding_style.transform)),
        entropy_coder: Some(match codestream.kind {
            codestream::CodestreamKind::J2k => EntropyCoder::ClassicTier1,
            codestream::CodestreamKind::Htj2k => EntropyCoder::HtBlockCoding,
        }),
    }
}

fn support_from_codestream(
    codestream: &codestream::Codestream,
    bytes: Option<&[u8]>,
) -> SupportStatus {
    #[cfg(feature = "std")]
    if codestream.kind == codestream::CodestreamKind::Htj2k
        && let Some(bytes) = bytes
    {
        return match codestream::htj2k_lossless_profile_unsupported_construct(bytes, codestream) {
            None => SupportStatus::Supported,
            Some((construct, detail)) => SupportStatus::Unsupported {
                feature: unsupported_feature_from_construct(construct),
                detail,
            },
        };
    }

    if codestream.kind == codestream::CodestreamKind::J2k
        && bytes.is_some_and(codestream::is_algorithmic_baseline_profile)
    {
        return SupportStatus::Supported;
    }

    match codestream::unsupported_construct(codestream) {
        Some((construct, detail)) => SupportStatus::Unsupported {
            feature: unsupported_feature_from_construct(construct),
            detail,
        },
        None if codestream.kind == codestream::CodestreamKind::J2k => SupportStatus::Unsupported {
            feature: UnsupportedFeature::EntropyCoder,
            detail:
                "native Part 1 decode is limited to the structurally admitted algorithmic profiles"
                    .into(),
        },
        None => SupportStatus::Unsupported {
            feature: UnsupportedFeature::EntropyCoder,
            detail: "native HTJ2K decode is limited to the structurally admitted lossless profiles"
                .into(),
        },
    }
}

#[cfg(feature = "std")]
fn decode_algorithmic_htj2k(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
) -> Result<Option<Image>> {
    let mut workspace = Htj2kDecodeWorkspace::new();
    decode_algorithmic_htj2k_with_workspace(input, metadata, options, &mut workspace)
}

#[cfg(feature = "std")]
fn decode_algorithmic_htj2k_with_workspace(
    input: &[u8],
    metadata: &Metadata,
    options: &DecodeOptions,
    workspace: &mut Htj2kDecodeWorkspace,
) -> Result<Option<Image>> {
    if !matches!(metadata.support, SupportStatus::Supported) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_htj2k_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let Some(decoded) = codestream::decode_htj2k_lossless_owned_with_workspace(
        codestream_bytes,
        &mut workspace.codestream,
    )
    .map_err(map_codestream_error)?
    else {
        return Ok(None);
    };
    decoded_baseline_to_image_with_component_info(decoded, options, None).map(Some)
}

#[cfg(feature = "std")]
fn decode_htj2k_cleanup_vlc_output_probe_from_metadata(
    input: &[u8],
    metadata: &Metadata,
    workspace: &mut Htj2kDecodeWorkspace,
) -> Result<Option<Htj2kCleanupVlcOutputProbe>> {
    let Some(codestream_bytes) = primary_htj2k_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    if !metadata.support.permits_decode() {
        return Ok(None);
    }

    workspace
        .codestream
        .decode_cleanup_vlc_output_probe(codestream_bytes)
        .map_err(map_codestream_error)
        .map(|outcome| {
            outcome.map(|probe| Htj2kCleanupVlcOutputProbe {
                output_count: probe.output_count,
                significant_output_count: probe.significant_output_count,
                significant_refinement_slot_mask_low64: probe
                    .significant_refinement_slot_mask_low64,
                first_significant_output: probe.first_significant_output.map(|output| {
                    Htj2kCleanupVlcSignificantOutput {
                        refinement_slot: output.refinement_slot,
                        quad_slot: output.quad_slot,
                        magnitude_sign_bits: output.magnitude_sign_bits,
                        magnitude_sign_value: output.magnitude_sign_value,
                        embedded_magnitude_bit: output.embedded_magnitude_bit,
                        magnitude_exponent_reduction: output.magnitude_exponent_reduction,
                        ht_sign_magnitude_coefficient: output.ht_sign_magnitude_coefficient,
                        reversible_transfer_coefficient: output.reversible_transfer_coefficient,
                        reversible_transfer_sample: output.reversible_transfer_sample,
                    }
                }),
                coding_passes: probe.coding_passes,
                packet_missing_most_significant_bitplanes: probe
                    .packet_missing_most_significant_bitplanes,
                cleanup_bitplane: probe.cleanup_bitplane,
                materialized_coefficient_count: probe.materialized_coefficient_count,
                materialized_coefficient_prefix: probe.materialized_coefficient_prefix,
                ht_sign_magnitude_coefficient_prefix: probe.ht_sign_magnitude_coefficient_prefix,
                reversible_transfer_qcd_guard_bits: probe.reversible_transfer_qcd_guard_bits,
                reversible_transfer_qcd_exponent: probe.reversible_transfer_qcd_exponent,
                reversible_transfer_k_max: probe.reversible_transfer_k_max,
                reversible_transfer_shift: probe.reversible_transfer_shift,
                reversible_transfer_coefficient_prefix: probe
                    .reversible_transfer_coefficient_prefix,
                reversible_transfer_sign_magnitude_coefficient_prefix: probe
                    .reversible_transfer_sign_magnitude_coefficient_prefix,
                reversible_transfer_sample_prefix: probe.reversible_transfer_sample_prefix,
                reversible_transfer_nonzero_coefficient_slot_mask_low64: probe
                    .reversible_transfer_nonzero_coefficient_slot_mask_low64,
                first_vlc_lookup: Htj2kCleanupVlcFirstLookup {
                    context: probe.first_vlc_lookup.context,
                    zero_context_mel_event: probe.first_vlc_lookup.zero_context_mel_event,
                    prefix_bits_lsb: probe.first_vlc_lookup.prefix_bits_lsb,
                    table_word: probe.first_vlc_lookup.table_word,
                    gated_table_word: probe.first_vlc_lookup.gated_table_word,
                    codeword_vlc_bits: probe.first_vlc_lookup.codeword_vlc_bits,
                    significance_bits: probe.first_vlc_lookup.significance_bits,
                    embedded_magnitude_bits: probe.first_vlc_lookup.embedded_magnitude_bits,
                    magnitude_exponent_reduction_bits: probe
                        .first_vlc_lookup
                        .magnitude_exponent_reduction_bits,
                    u_offset: probe.first_vlc_lookup.u_offset,
                    next_initial_context: probe.first_vlc_lookup.next_initial_context,
                },
                first_vlc_group: Htj2kCleanupVlcFirstGroup {
                    first_quad_present_count: probe.first_vlc_group.first_quad_present_count,
                    first_quad_present_mask: probe.first_vlc_group.first_quad_present_mask,
                    second_quad_present: probe.first_vlc_group.second_quad_present,
                    second_quad_present_count: probe.first_vlc_group.second_quad_present_count,
                    first_context: probe.first_vlc_group.first_context,
                    first_zero_context_mel_event: probe
                        .first_vlc_group
                        .first_zero_context_mel_event,
                    first_prefix_bits_lsb: probe.first_vlc_group.first_prefix_bits_lsb,
                    first_table_word: probe.first_vlc_group.first_table_word,
                    first_gated_table_word: probe.first_vlc_group.first_gated_table_word,
                    first_codeword_vlc_bits: probe.first_vlc_group.first_codeword_vlc_bits,
                    first_significance_bits: probe.first_vlc_group.first_significance_bits,
                    first_embedded_magnitude_bits: probe
                        .first_vlc_group
                        .first_embedded_magnitude_bits,
                    first_magnitude_exponent_reduction_bits: probe
                        .first_vlc_group
                        .first_magnitude_exponent_reduction_bits,
                    first_u_offset: probe.first_vlc_group.first_u_offset,
                    second_context: probe.first_vlc_group.second_context,
                    second_zero_context_mel_event: probe
                        .first_vlc_group
                        .second_zero_context_mel_event,
                    second_prefix_bits_lsb: probe.first_vlc_group.second_prefix_bits_lsb,
                    second_table_word: probe.first_vlc_group.second_table_word,
                    second_gated_table_word: probe.first_vlc_group.second_gated_table_word,
                    second_codeword_vlc_bits: probe.first_vlc_group.second_codeword_vlc_bits,
                    second_significance_bits: probe.first_vlc_group.second_significance_bits,
                    second_embedded_magnitude_bits: probe
                        .first_vlc_group
                        .second_embedded_magnitude_bits,
                    second_magnitude_exponent_reduction_bits: probe
                        .first_vlc_group
                        .second_magnitude_exponent_reduction_bits,
                    second_u_offset: probe.first_vlc_group.second_u_offset,
                    paired_uvlc_both_offsets_mel_event: probe
                        .first_vlc_group
                        .paired_uvlc_both_offsets_mel_event,
                    paired_uvlc_first: probe.first_vlc_group.paired_uvlc_first,
                    paired_uvlc_second: probe.first_vlc_group.paired_uvlc_second,
                    paired_uvlc_consumed_bits: probe.first_vlc_group.paired_uvlc_consumed_bits,
                    single_tail_u_value: probe.first_vlc_group.single_tail_u_value,
                },
                scratch_words: probe.scratch_words,
                cleanup_progress: probe.cleanup_progress,
                segment_bit_progress: probe.segment_bit_progress,
            })
        })
}

fn unsupported_feature_from_construct(
    construct: codestream::UnsupportedConstruct,
) -> UnsupportedFeature {
    match construct {
        codestream::UnsupportedConstruct::MarkerSegment
        | codestream::UnsupportedConstruct::Part2Capabilities => UnsupportedFeature::MarkerSegment,
        codestream::UnsupportedConstruct::ProgressionOrder => UnsupportedFeature::ProgressionOrder,
        codestream::UnsupportedConstruct::WaveletTransform => UnsupportedFeature::WaveletTransform,
        codestream::UnsupportedConstruct::EntropyCoder
        | codestream::UnsupportedConstruct::HtBlockDecode => UnsupportedFeature::EntropyCoder,
        codestream::UnsupportedConstruct::SamplePrecision
        | codestream::UnsupportedConstruct::ComponentCount
        | codestream::UnsupportedConstruct::ComponentSampling => {
            UnsupportedFeature::ComponentLayout
        }
        codestream::UnsupportedConstruct::MultipleTiles => UnsupportedFeature::PartialDecodeMode,
        codestream::UnsupportedConstruct::PacketDecode => UnsupportedFeature::MarkerSegment,
        codestream::UnsupportedConstruct::Tier1Decode => UnsupportedFeature::EntropyCoder,
        codestream::UnsupportedConstruct::Transform => UnsupportedFeature::WaveletTransform,
    }
}

fn image_info_from_codestream(codestream: &codestream::Codestream) -> Option<ImageInfo> {
    let sample_format = sample_format_from_codestream(codestream)?;
    let color_model = match codestream.siz.component_count() {
        1 => ColorModel::Grayscale,
        3 => ColorModel::Rgb,
        _ => ColorModel::Unknown,
    };
    ImageInfo::new(
        codestream.image_width(),
        codestream.image_height(),
        codestream.siz.component_count(),
        sample_format,
        color_model,
        ComponentLayout::Planar,
    )
    .ok()
}

fn sample_format_from_codestream(codestream: &codestream::Codestream) -> Option<SampleFormat> {
    let first = *codestream.siz.components.first()?;
    if codestream.siz.components.iter().any(|component| {
        component.bits_per_sample != first.bits_per_sample || component.signed != first.signed
    }) {
        return None;
    }
    let byte_order = if first.bits_per_sample <= 8 {
        None
    } else {
        Some(SampleEndian::Little)
    };
    SampleFormat::with_byte_order(first.bits_per_sample, first.signed, byte_order).ok()
}

fn progression_order_from_codestream(
    progression_order: codestream::ProgressionOrder,
) -> ProgressionOrder {
    match progression_order {
        codestream::ProgressionOrder::Lrcp => ProgressionOrder::Lrcp,
        codestream::ProgressionOrder::Rlcp => ProgressionOrder::Rlcp,
        codestream::ProgressionOrder::Rpcl => ProgressionOrder::Rpcl,
        codestream::ProgressionOrder::Pcrl => ProgressionOrder::Pcrl,
        codestream::ProgressionOrder::Cprl => ProgressionOrder::Cprl,
    }
}

fn transform_from_codestream(transform: codestream::WaveletTransform) -> WaveletTransform {
    match transform {
        codestream::WaveletTransform::Reversible53 => WaveletTransform::Reversible53,
        codestream::WaveletTransform::Irreversible97 => WaveletTransform::Irreversible97,
    }
}

fn image_info_from_container(container: &container::Container) -> Option<ImageInfo> {
    let image_header = container.image_header?;
    let sample_format = sample_format_from_container(container)?;
    ImageInfo::new(
        image_header.width,
        image_header.height,
        image_header.components,
        sample_format,
        color_model_from_container(container),
        ComponentLayout::Planar,
    )
    .ok()
}

fn sample_format_from_container(container: &container::Container) -> Option<SampleFormat> {
    let components = container.component_sample_formats()?;
    let first = *components.first()?;
    if components.iter().any(|component| *component != first) {
        return None;
    }
    let byte_order = if first.bits_per_sample <= 8 {
        None
    } else {
        Some(SampleEndian::Little)
    };
    SampleFormat::with_byte_order(first.bits_per_sample, first.signed, byte_order).ok()
}

fn color_model_from_container(container: &container::Container) -> ColorModel {
    let Some(color_specification) = container.color_specification else {
        return ColorModel::Unknown;
    };

    match color_specification.enumerated_color_space {
        Some(container::EnumeratedColorSpace::SRgb) => ColorModel::Rgb,
        Some(container::EnumeratedColorSpace::Greyscale) => ColorModel::Grayscale,
        Some(container::EnumeratedColorSpace::SYcc) => ColorModel::YCbCr,
        Some(container::EnumeratedColorSpace::Unknown(_)) | None => ColorModel::Unknown,
    }
}

fn require_supported_metadata(metadata: &Metadata) -> Result<()> {
    match &metadata.support {
        SupportStatus::Supported => Ok(()),
        SupportStatus::Unsupported { feature, detail } => {
            Err(unsupported(*feature, detail.clone()))
        }
        SupportStatus::Unknown { detail } => Err(unsupported(
            UnsupportedFeature::InputFormat,
            alloc::format!("input support is unknown: {detail}"),
        )),
    }
}

fn validate_partial_options(metadata: &Metadata, options: &PartialDecodeOptions) -> Result<()> {
    validate_partial_options_without_support(metadata, options)?;
    require_supported_metadata(metadata)
}

fn validate_partial_options_without_support(
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<()> {
    if options.max_quality_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }
    match options.resolution {
        ResolutionLevel::Full => {}
        ResolutionLevel::Reduced { discard_levels: 0 } => {}
        ResolutionLevel::Reduced { .. } => {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "reduced-resolution decode is not implemented for this profile",
            ));
        }
    }

    if options.region.is_some() && options.tile.is_some() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "combining tile and region decode requires packet-index semantics before support is enabled",
        ));
    }

    if let Some(region) = options.region {
        if region.width == 0 || region.height == 0 {
            return Err(J2kError::InvalidParameter {
                parameter: "region",
                message: "partial decode region dimensions must be greater than zero",
            });
        }
        let image = metadata.image.as_ref().ok_or_else(|| {
            unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "partial decode requires image dimensions from metadata inspection",
            )
        })?;
        let end_x = region
            .x
            .checked_add(region.width)
            .ok_or_else(sample_size_overflow)?;
        let end_y = region
            .y
            .checked_add(region.height)
            .ok_or_else(sample_size_overflow)?;
        if end_x > image.width || end_y > image.height {
            return Err(J2kError::InvalidParameter {
                parameter: "region",
                message: "partial decode region must fit inside the image bounds",
            });
        }
    }

    Ok(())
}

fn partial_output_region(metadata: &Metadata, options: &PartialDecodeOptions) -> Result<Region> {
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial decode requires image dimensions from metadata inspection",
        )
    })?;

    if let Some(region) = options.region {
        return Ok(region);
    }

    if let Some(tile) = options.tile {
        let grid = metadata
            .codestream
            .as_ref()
            .and_then(|codestream| codestream.tile_grid)
            .ok_or_else(|| {
                unsupported(
                    UnsupportedFeature::PartialDecodeMode,
                    "tile decode requires codestream tile-grid metadata",
                )
            })?;
        if grid.tile_width == 0 || grid.tile_height == 0 {
            return Err(J2kError::InvalidInput {
                offset: None,
                message: "codestream tile grid reported zero tile dimensions".into(),
            });
        }

        let tile_count_x = ceil_div_u32(image.width, grid.tile_width)?;
        let tile_count_y = ceil_div_u32(image.height, grid.tile_height)?;
        if tile.tile_x >= tile_count_x || tile.tile_y >= tile_count_y {
            return Err(J2kError::InvalidParameter {
                parameter: "tile",
                message: "requested tile is outside the codestream tile grid",
            });
        }
        if tile_count_x > 1 || tile_count_y > 1 {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "tile decode currently uses the full-decode adapter and is enabled only for single-tile inputs",
            ));
        }

        return Ok(Region {
            x: 0,
            y: 0,
            width: image.width,
            height: image.height,
        });
    }

    Ok(Region {
        x: 0,
        y: 0,
        width: image.width,
        height: image.height,
    })
}

fn partial_component_indices(
    metadata: &Metadata,
    components: &ComponentSelection,
) -> Result<Vec<u16>> {
    requested_component_indices(metadata, components)
}

fn requested_component_indices(
    metadata: &Metadata,
    components: &ComponentSelection,
) -> Result<Vec<u16>> {
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::ComponentLayout,
            "component selection requires image component metadata",
        )
    })?;

    match components {
        ComponentSelection::All => Ok((0..image.components).collect()),
        ComponentSelection::Indices(indices) => {
            if indices.is_empty() {
                return Err(J2kError::InvalidParameter {
                    parameter: "components",
                    message: "component subset must contain at least one component index",
                });
            }
            let mut seen = Vec::new();
            for index in indices {
                if *index >= image.components {
                    return Err(J2kError::InvalidParameter {
                        parameter: "components",
                        message: "component index is outside the decoded component range",
                    });
                }
                if seen.contains(index) {
                    return Err(J2kError::InvalidParameter {
                        parameter: "components",
                        message: "component subset must not contain duplicate indices",
                    });
                }
                seen.push(*index);
            }
            Ok(indices.clone())
        }
    }
}

fn apply_partial_selection(
    decoded: Image,
    region: Region,
    component_indices: &[u16],
    target_layout: ComponentLayout,
) -> Result<Image> {
    let source_component_info = decoded.component_info;
    let source_planes = match decoded.data {
        ImageData::Planes(planes) => planes,
        ImageData::Interleaved(samples) => split_interleaved_to_planes(
            &samples,
            decoded.info.width,
            decoded.info.height,
            u8::try_from(decoded.info.components).map_err(|_| {
                unsupported(
                    UnsupportedFeature::ComponentLayout,
                    "decoded component count exceeds the public image model",
                )
            })?,
            decoded.info.sample_format,
        )?,
    };

    let selected_planes =
        crop_selected_planes(&source_planes, &decoded.info, region, component_indices)?;
    let info = ImageInfo::new(
        region.width,
        region.height,
        u16::try_from(component_indices.len()).map_err(|_| {
            unsupported(
                UnsupportedFeature::ComponentLayout,
                "component subset exceeds the public image model",
            )
        })?,
        decoded.info.sample_format,
        partial_color_model(&decoded.info, component_indices),
        target_layout,
    )?;
    let component_info = component_indices
        .iter()
        .map(|component_index| {
            let mut component = source_component_info
                .get(usize::from(*component_index))
                .cloned()
                .ok_or_else(sample_size_overflow)?;
            component.width = region.width;
            component.height = region.height;
            component.x_origin = component
                .x_origin
                .checked_add(region.x / u32::from(component.horizontal_separation))
                .ok_or_else(sample_size_overflow)?;
            component.y_origin = component
                .y_origin
                .checked_add(region.y / u32::from(component.vertical_separation))
                .ok_or_else(sample_size_overflow)?;
            Ok(component)
        })
        .collect::<Result<Vec<_>>>()?;

    match target_layout {
        ComponentLayout::Planar => Ok(Image {
            component_info,
            info,
            data: ImageData::Planes(selected_planes),
        }),
        ComponentLayout::Interleaved => Ok(Image {
            data: ImageData::Interleaved(interleave_planes(
                &selected_planes,
                region.width,
                region.height,
                decoded.info.sample_format,
            )?),
            component_info,
            info,
        }),
    }
}

fn crop_selected_planes(
    source_planes: &[Vec<u8>],
    info: &ImageInfo,
    region: Region,
    component_indices: &[u16],
) -> Result<Vec<Vec<u8>>> {
    let bytes_per_sample = public_bytes_per_sample("sample_format", info.sample_format)?;
    let row_bytes = checked_public_row_bytes("sample_format", info.width, 1, bytes_per_sample)?;
    let output_row_bytes =
        checked_public_row_bytes("sample_format", region.width, 1, bytes_per_sample)?;
    let capacity = output_row_bytes
        .checked_mul(region.height as usize)
        .ok_or_else(sample_size_overflow)?;
    let mut output = Vec::with_capacity(component_indices.len());

    for component in component_indices {
        let plane =
            source_planes
                .get(usize::from(*component))
                .ok_or_else(|| J2kError::InvalidInput {
                    offset: None,
                    message: "decoded plane count was smaller than metadata requires".into(),
                })?;
        let mut cropped = Vec::with_capacity(capacity);
        for y in region.y..region.y + region.height {
            let row_start = (y as usize)
                .checked_mul(row_bytes)
                .and_then(|value| {
                    value.checked_add((region.x as usize).checked_mul(bytes_per_sample)?)
                })
                .ok_or_else(sample_size_overflow)?;
            let row_end = row_start
                .checked_add(output_row_bytes)
                .ok_or_else(sample_size_overflow)?;
            cropped.extend_from_slice(plane.get(row_start..row_end).ok_or_else(|| {
                J2kError::InvalidInput {
                    offset: None,
                    message: "decoded plane was smaller than image metadata requires".into(),
                }
            })?);
        }
        output.push(cropped);
    }

    Ok(output)
}

fn interleave_planes(
    planes: &[Vec<u8>],
    width: u32,
    height: u32,
    sample_format: SampleFormat,
) -> Result<Vec<u8>> {
    let pixels = pixel_count(width, height)?;
    let bytes_per_sample = public_bytes_per_sample("sample_format", sample_format)?;
    let component_count = planes.len();
    let plane_bytes = pixels
        .checked_mul(bytes_per_sample)
        .ok_or_else(sample_size_overflow)?;
    if planes.iter().any(|plane| plane.len() < plane_bytes) {
        return Err(J2kError::InvalidInput {
            offset: None,
            message: "decoded plane was smaller than image metadata requires".into(),
        });
    }
    let capacity = pixels
        .checked_mul(component_count)
        .and_then(|value| value.checked_mul(bytes_per_sample))
        .ok_or_else(sample_size_overflow)?;
    let mut output = alloc::vec![0_u8; capacity];

    if let [red, green, blue] = planes {
        match bytes_per_sample {
            1 => {
                for (((pixel, red), green), blue) in output
                    .chunks_exact_mut(3)
                    .zip(red.iter())
                    .zip(green.iter())
                    .zip(blue.iter())
                {
                    pixel[0] = *red;
                    pixel[1] = *green;
                    pixel[2] = *blue;
                }
                return Ok(output);
            }
            2 => {
                for (((pixel, red), green), blue) in output
                    .chunks_exact_mut(6)
                    .zip(red.chunks_exact(2))
                    .zip(green.chunks_exact(2))
                    .zip(blue.chunks_exact(2))
                {
                    pixel[0] = red[0];
                    pixel[1] = red[1];
                    pixel[2] = green[0];
                    pixel[3] = green[1];
                    pixel[4] = blue[0];
                    pixel[5] = blue[1];
                }
                return Ok(output);
            }
            _ => {}
        }
    }

    for pixel in 0..pixels {
        let sample_offset = pixel
            .checked_mul(bytes_per_sample)
            .ok_or_else(sample_size_overflow)?;
        let pixel_output_offset = pixel
            .checked_mul(component_count)
            .and_then(|value| value.checked_mul(bytes_per_sample))
            .ok_or_else(sample_size_overflow)?;
        for (component, plane) in planes.iter().enumerate() {
            let sample = plane
                .get(sample_offset..sample_offset + bytes_per_sample)
                .ok_or_else(|| J2kError::InvalidInput {
                    offset: None,
                    message: "decoded plane was smaller than image metadata requires".into(),
                })?;
            let output_offset = pixel_output_offset
                .checked_add(
                    component
                        .checked_mul(bytes_per_sample)
                        .ok_or_else(sample_size_overflow)?,
                )
                .ok_or_else(sample_size_overflow)?;
            output[output_offset..output_offset + bytes_per_sample].copy_from_slice(sample);
        }
    }

    Ok(output)
}

fn partial_color_model(info: &ImageInfo, component_indices: &[u16]) -> ColorModel {
    if component_indices.len() == usize::from(info.components)
        && component_indices.iter().copied().eq(0..info.components)
    {
        return info.color_model;
    }

    match component_indices {
        [0] if info.components == 1 => ColorModel::Grayscale,
        _ => ColorModel::Unknown,
    }
}

fn partial_decode_target_info(input: &[u8], options: &PartialDecodeOptions) -> Result<ImageInfo> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }

    let metadata = inspect(input, &InspectOptions::default())?;
    validate_partial_quality_layer_profile(input, &metadata, options)?;
    if let Some(codestream_bytes) = primary_part1_codestream_bytes(input, &metadata)? {
        let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
        if is_native_multitile_partial_profile(&metadata, codestream_bytes, &parsed) {
            let (region, component_indices) = validate_native_multitile_partial_options(
                &metadata,
                codestream_bytes,
                &parsed,
                options,
            )?;
            return ImageInfo::new(
                region.width,
                region.height,
                u16::try_from(component_indices.len()).map_err(|_| sample_size_overflow())?,
                SampleFormat::U8,
                ColorModel::Grayscale,
                ComponentLayout::Planar,
            );
        }
        if has_multiple_part1_tiles(&parsed) && (options.region.is_some() || options.tile.is_some())
        {
            return Err(unsupported(
                UnsupportedFeature::PartialDecodeMode,
                "spatial multi-tile decode requires the bounded native two-decomposition grayscale profile",
            ));
        }
        if codestream::is_supported_part1_native_subsampled_component_profile(&parsed) {
            if options.target_layout != ComponentLayout::Planar {
                return Err(unsupported(
                    UnsupportedFeature::ComponentLayout,
                    "subsampled native components require planar output without implicit resampling",
                ));
            }
            let region = direct_part1_region(&parsed, options)?;
            let component_indices = direct_part1_component_indices(&parsed, &options.components)?;
            let discard_levels = direct_part1_discard_levels(&parsed, options)?;
            let descriptors = part1_component_info_at_resolution(
                codestream_bytes,
                &ComponentSelection::Indices(component_indices.clone()),
                Some(region),
                discard_levels,
            )?;
            let sample_format = descriptors
                .first()
                .map(|component| component.sample_format)
                .ok_or_else(sample_size_overflow)?;
            let output_region = reduced_part1_region(&parsed.siz, region, discard_levels)?;
            return ImageInfo::new(
                output_region.width,
                output_region.height,
                u16::try_from(component_indices.len()).map_err(|_| sample_size_overflow())?,
                sample_format,
                ColorModel::Unknown,
                ComponentLayout::Planar,
            );
        }
    }
    if let Some(info) = p0_08_heterogeneous_reversible_target_info(input, &metadata, options)? {
        return Ok(info);
    }
    if let Some(info) = p0_07_progression_change_target_info(input, &metadata, options)? {
        return Ok(info);
    }
    if let Some(info) = selective_part1_discard_target_info(input, &metadata, options)? {
        return Ok(info);
    }
    let selective_component_profile = options.tile.is_none()
        && options.resolution == ResolutionLevel::Full
        && primary_part1_codestream_bytes(input, &metadata)?
            .is_some_and(is_direct_selective_part1_component_profile);
    if selective_component_profile {
        validate_partial_options_without_support(&metadata, options)?;
    } else {
        validate_partial_options(&metadata, options)?;
    }
    let region = partial_output_region(&metadata, options)?;
    let component_indices = partial_component_indices(&metadata, &options.components)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial decode requires image dimensions from metadata inspection",
        )
    })?;
    ImageInfo::new(
        region.width,
        region.height,
        u16::try_from(component_indices.len()).map_err(|_| {
            unsupported(
                UnsupportedFeature::ComponentLayout,
                "component subset exceeds the public image model",
            )
        })?,
        image.sample_format,
        partial_color_model(image, &component_indices),
        options.target_layout,
    )
}

fn p0_08_heterogeneous_reversible_target_info(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<ImageInfo>> {
    if !is_p0_08_output_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_08_heterogeneous_reversible_component_profile(
        codestream_bytes,
        &parsed,
        5,
    ) {
        return Ok(None);
    }
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 P0.08 decode requires image sample metadata",
        )
    })?;
    ImageInfo::new(
        17,
        96,
        1,
        image.sample_format,
        ColorModel::Unknown,
        ComponentLayout::Planar,
    )
    .map(Some)
}

fn p0_07_progression_change_target_info(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<ImageInfo>> {
    if !is_p0_07_output_request(options) {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if !codestream::is_supported_part1_p0_07_progression_change_component_profile(
        codestream_bytes,
        &parsed,
    ) {
        return Ok(None);
    }
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 P0.07 decode requires image sample metadata",
        )
    })?;
    ImageInfo::new(
        128,
        128,
        1,
        image.sample_format,
        ColorModel::Unknown,
        ComponentLayout::Planar,
    )
    .map(Some)
}

fn selective_part1_discard_target_info(
    input: &[u8],
    metadata: &Metadata,
    options: &PartialDecodeOptions,
) -> Result<Option<ImageInfo>> {
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    if discard_levels == 0 {
        return Ok(None);
    }
    if options.tile.is_some() || options.target_layout != ComponentLayout::Planar {
        return Ok(None);
    }
    if options.max_quality_layers == Some(0) {
        return Err(J2kError::InvalidParameter {
            parameter: "max_quality_layers",
            message: "maximum quality layers must be at least one",
        });
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if codestream::is_supported_part1_native_subsampled_component_profile(&parsed) {
        // Native subsampled reductions retain per-component output geometry
        // and are handled by the direct planar route rather than this uniform
        // reduced-image compatibility path.
        return Ok(None);
    }
    let reduced_mct_component_zero = matches!(
        &options.components,
        ComponentSelection::Indices(indices) if indices.as_slice() == [0_u16]
    );
    let reduced_heterogeneous_profile = options.region.is_none()
        && options.max_quality_layers.is_none()
        && reduced_mct_component_zero
        && codestream::is_supported_part1_reduced_heterogeneous_irreversible_component_profile(
            codestream_bytes,
            &parsed,
            discard_levels,
        );
    let reduced_roi_profile = options.region.is_none()
        && options.max_quality_layers.is_none()
        && reduced_mct_component_zero
        && codestream::is_supported_part1_reduced_roi_irreversible_component_profile(
            codestream_bytes,
            &parsed,
            discard_levels,
        );
    let Some(coding_style) = parsed.uniform_effective_coding_style().or_else(|| {
        (reduced_heterogeneous_profile || reduced_roi_profile)
            .then(|| parsed.effective_coding_style(0))
            .flatten()
    }) else {
        return Ok(None);
    };
    let reduced_mct_profile = options.region.is_none()
        && options.max_quality_layers.is_none()
        && reduced_mct_component_zero
        && coding_style.multiple_component_transform
        && (codestream::is_supported_part1_reduced_reversible_mct_component_profile(
            codestream_bytes,
            &parsed,
            discard_levels,
        ) || codestream::is_supported_part1_reduced_irreversible_mct_component_profile(
            codestream_bytes,
            &parsed,
            discard_levels,
        ));
    let reduced_profile =
        reduced_mct_profile || reduced_heterogeneous_profile || reduced_roi_profile;
    if (!is_direct_selective_part1_component_profile(codestream_bytes) && !reduced_profile)
        || (coding_style.multiple_component_transform && !reduced_mct_profile)
        || discard_levels > coding_style.decomposition_levels
    {
        return Ok(None);
    }
    let scale = 1_u32
        .checked_shl(u32::from(discard_levels))
        .ok_or_else(sample_size_overflow)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial discard decode requires image dimensions",
        )
    })?;
    let full_region = options.region.unwrap_or(Region {
        x: 0,
        y: 0,
        width: image.width,
        height: image.height,
    });
    let tile_plan = codestream::plan_tile_region_decode(
        &parsed,
        codestream::TileRegionRequest {
            x: full_region.x,
            y: full_region.y,
            width: full_region.width,
            height: full_region.height,
        },
    )
    .map_err(map_codestream_error)?;
    if tile_plan
        .tiles
        .iter()
        .any(|planned| planned.tile.x % scale != 0 || planned.tile.y % scale != 0)
    {
        return Ok(None);
    }
    let component_indices = partial_component_indices(metadata, &options.components)?;
    let reduced_region = reduced_part1_region(&parsed.siz, full_region, discard_levels)?;
    ImageInfo::new(
        reduced_region.width,
        reduced_region.height,
        u16::try_from(component_indices.len()).map_err(|_| sample_size_overflow())?,
        image.sample_format,
        partial_color_model(image, &component_indices),
        options.target_layout,
    )
    .map(Some)
}
fn ceil_div_u32(value: u32, divisor: u32) -> Result<u32> {
    if divisor == 0 {
        return Err(J2kError::InvalidInput {
            offset: None,
            message: "division by zero while deriving tile grid".into(),
        });
    }
    Ok(value.div_ceil(divisor))
}

fn split_interleaved_to_planes(
    samples: &[u8],
    width: u32,
    height: u32,
    components: u8,
    sample_format: SampleFormat,
) -> Result<Vec<Vec<u8>>> {
    let pixels = pixel_count(width, height)?;
    let bytes_per_sample = public_bytes_per_sample("sample_format", sample_format)?;
    let component_count = usize::from(components);
    let required = pixels
        .checked_mul(component_count)
        .and_then(|value| value.checked_mul(bytes_per_sample))
        .ok_or_else(sample_size_overflow)?;
    if samples.len() < required {
        return Err(J2kError::InvalidInput {
            offset: None,
            message: "decoded sample buffer was smaller than image metadata requires".into(),
        });
    }

    let mut planes = (0..component_count)
        .map(|_| Vec::with_capacity(pixels * bytes_per_sample))
        .collect::<Vec<_>>();
    let pixel_bytes = component_count
        .checked_mul(bytes_per_sample)
        .ok_or_else(sample_size_overflow)?;
    for pixel in samples[..required].chunks_exact(pixel_bytes) {
        for (component, plane) in planes.iter_mut().enumerate() {
            let start = component
                .checked_mul(bytes_per_sample)
                .ok_or_else(sample_size_overflow)?;
            plane.extend_from_slice(&pixel[start..start + bytes_per_sample]);
        }
    }

    Ok(planes)
}

fn copy_image_into_target(image: &Image, target: &mut ImageViewMut<'_>) -> Result<()> {
    match (&image.data, target) {
        (
            ImageData::Planes(source_planes),
            ImageViewMut::Planar {
                info,
                planes: target_planes,
            },
        ) => {
            validate_decode_target_info(&image.info, info)?;
            if source_planes.len() != target_planes.len() {
                return Err(J2kError::InvalidParameter {
                    parameter: "planes",
                    message: "target plane count must match decoded component count",
                });
            }

            for (source, target) in source_planes.iter().zip(target_planes.iter_mut()) {
                let row_bytes = checked_public_row_bytes(
                    "target.info",
                    image.info.width,
                    1,
                    public_bytes_per_sample("target.info", image.info.sample_format)?,
                )?;
                if target.width != image.info.width || target.height != image.info.height {
                    return Err(J2kError::InvalidParameter {
                        parameter: "plane",
                        message: "target plane dimensions must match decoded image dimensions",
                    });
                }
                if target.sample_format != image.info.sample_format {
                    return Err(J2kError::InvalidParameter {
                        parameter: "plane.sample_format",
                        message: "target plane sample format must match decoded image sample format",
                    });
                }
                copy_rows(
                    source,
                    row_bytes,
                    target.samples,
                    target.stride_bytes,
                    image.info.height,
                )?;
            }

            Ok(())
        }
        (
            ImageData::Interleaved(source),
            ImageViewMut::Interleaved {
                info,
                samples,
                stride_bytes,
            },
        ) => {
            validate_decode_target_info(&image.info, info)?;
            let row_bytes = checked_public_row_bytes(
                "target.info",
                image.info.width,
                image.info.components,
                public_bytes_per_sample("target.info", image.info.sample_format)?,
            )?;
            copy_rows(source, row_bytes, samples, *stride_bytes, image.info.height)
        }
        _ => Err(J2kError::InvalidParameter {
            parameter: "target",
            message: "target layout must match decode layout",
        }),
    }
}

fn validate_decode_target_info(decoded: &ImageInfo, target: &ImageInfo) -> Result<()> {
    if target.width != decoded.width
        || target.height != decoded.height
        || target.components != decoded.components
        || target.sample_format != decoded.sample_format
        || target.color_model != decoded.color_model
        || target.layout != decoded.layout
    {
        return Err(J2kError::InvalidParameter {
            parameter: "target.info",
            message: "target image info must match decoded image info",
        });
    }

    Ok(())
}

fn validate_decode_target(expected: &ImageInfo, target: &ImageViewMut<'_>) -> Result<()> {
    match target {
        ImageViewMut::Planar {
            info,
            planes: target_planes,
        } => {
            validate_decode_target_info(expected, info)?;
            if target_planes.len() != usize::from(expected.components) {
                return Err(J2kError::InvalidParameter {
                    parameter: "planes",
                    message: "target plane count must match decoded component count",
                });
            }
            for plane in target_planes.iter() {
                if plane.width != expected.width || plane.height != expected.height {
                    return Err(J2kError::InvalidParameter {
                        parameter: "plane",
                        message: "target plane dimensions must match decoded image dimensions",
                    });
                }
                if plane.sample_format != expected.sample_format {
                    return Err(J2kError::InvalidParameter {
                        parameter: "plane.sample_format",
                        message: "target plane sample format must match decoded image sample format",
                    });
                }
                let row_bytes = checked_public_row_bytes(
                    "target.info",
                    expected.width,
                    1,
                    public_bytes_per_sample("target.info", expected.sample_format)?,
                )?;
                if plane.stride_bytes < row_bytes {
                    return Err(J2kError::InvalidParameter {
                        parameter: "stride_bytes",
                        message: "target stride must be at least one decoded row",
                    });
                }
                let required = plane
                    .stride_bytes
                    .checked_mul(expected.height as usize)
                    .ok_or_else(sample_size_overflow)?;
                if plane.samples.len() < required {
                    return Err(J2kError::BufferTooSmall {
                        required,
                        provided: plane.samples.len(),
                    });
                }
            }
            Ok(())
        }
        ImageViewMut::Interleaved {
            info,
            samples,
            stride_bytes,
        } => {
            validate_decode_target_info(expected, info)?;
            let row_bytes = checked_public_row_bytes(
                "target.info",
                expected.width,
                expected.components,
                public_bytes_per_sample("target.info", expected.sample_format)?,
            )?;
            if *stride_bytes < row_bytes {
                return Err(J2kError::InvalidParameter {
                    parameter: "stride_bytes",
                    message: "target stride must be at least one decoded row",
                });
            }
            let required = stride_bytes
                .checked_mul(expected.height as usize)
                .ok_or_else(sample_size_overflow)?;
            if samples.len() < required {
                return Err(J2kError::BufferTooSmall {
                    required,
                    provided: samples.len(),
                });
            }
            Ok(())
        }
    }
}

fn validate_decode_target_components(
    expected: &ImageInfo,
    components: &[ComponentInfo],
    target: &ImageViewMut<'_>,
) -> Result<()> {
    let ImageViewMut::Planar {
        info,
        planes: target_planes,
    } = target
    else {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "native component descriptors require planar caller output",
        ));
    };
    validate_decode_target_info(expected, info)?;
    if target_planes.len() != components.len() {
        return Err(J2kError::InvalidParameter {
            parameter: "planes",
            message: "target plane count must match selected component descriptors",
        });
    }
    for (plane, component) in target_planes.iter().zip(components) {
        if plane.width != component.width || plane.height != component.height {
            return Err(J2kError::InvalidParameter {
                parameter: "plane",
                message: "target plane dimensions must match its native component descriptor",
            });
        }
        if plane.sample_format != component.sample_format {
            return Err(J2kError::InvalidParameter {
                parameter: "plane.sample_format",
                message: "target plane sample format must match its component descriptor",
            });
        }
        let row_bytes = checked_public_row_bytes(
            "plane.sample_format",
            component.width,
            1,
            public_bytes_per_sample("plane.sample_format", component.sample_format)?,
        )?;
        if plane.stride_bytes < row_bytes {
            return Err(J2kError::InvalidParameter {
                parameter: "stride_bytes",
                message: "target stride must hold one native component row",
            });
        }
        let rows = usize::try_from(component.height).map_err(|_| sample_size_overflow())?;
        let required = if rows == 0 {
            0
        } else {
            (rows - 1)
                .checked_mul(plane.stride_bytes)
                .and_then(|offset| offset.checked_add(row_bytes))
                .ok_or_else(sample_size_overflow)?
        };
        if plane.samples.len() < required {
            return Err(J2kError::BufferTooSmall {
                required,
                provided: plane.samples.len(),
            });
        }
    }
    Ok(())
}

fn copy_rows(
    source: &[u8],
    row_bytes: usize,
    target: &mut [u8],
    target_stride: usize,
    height: u32,
) -> Result<()> {
    if target_stride < row_bytes {
        return Err(J2kError::InvalidParameter {
            parameter: "stride_bytes",
            message: "target stride must be at least one decoded row",
        });
    }

    let source_required = row_bytes
        .checked_mul(height as usize)
        .ok_or_else(sample_size_overflow)?;
    if source.len() < source_required {
        return Err(J2kError::InvalidInput {
            offset: None,
            message: "decoded sample buffer was smaller than image metadata requires".into(),
        });
    }

    let target_required = target_stride
        .checked_mul(height as usize)
        .ok_or_else(sample_size_overflow)?;
    if target.len() < target_required {
        return Err(J2kError::BufferTooSmall {
            required: target_required,
            provided: target.len(),
        });
    }

    for row in 0..height as usize {
        let source_start = row * row_bytes;
        let target_start = row * target_stride;
        target[target_start..target_start + row_bytes]
            .copy_from_slice(&source[source_start..source_start + row_bytes]);
    }

    Ok(())
}

fn pixel_count(width: u32, height: u32) -> Result<usize> {
    width
        .checked_mul(height)
        .map(|value| value as usize)
        .ok_or_else(sample_size_overflow)
}

fn sample_size_overflow() -> J2kError {
    J2kError::InvalidInput {
        offset: None,
        message: "decoded sample size overflowed usize".into(),
    }
}

fn validate_plane(
    parameter: &'static str,
    len: usize,
    width: u32,
    height: u32,
    stride_bytes: usize,
    sample_format: SampleFormat,
) -> Result<()> {
    if width == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "width",
            message: "plane width must be greater than zero",
        });
    }
    if height == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "height",
            message: "plane height must be greater than zero",
        });
    }

    let row_bytes = checked_public_row_bytes(
        parameter,
        width,
        1,
        public_bytes_per_sample(parameter, sample_format)?,
    )?;
    if stride_bytes < row_bytes {
        return Err(J2kError::InvalidParameter {
            parameter,
            message: "plane stride must be at least one packed row",
        });
    }

    let required = stride_bytes
        .checked_mul(height as usize)
        .ok_or(J2kError::InvalidParameter {
            parameter,
            message: "plane byte size overflowed usize",
        })?;
    if len < required {
        return Err(J2kError::BufferTooSmall {
            required,
            provided: len,
        });
    }

    Ok(())
}

fn validate_image_view(image: &ImageView<'_>) -> Result<()> {
    match image {
        ImageView::Planar { info, planes } => {
            if info.layout != ComponentLayout::Planar {
                return Err(J2kError::InvalidParameter {
                    parameter: "info.layout",
                    message: "planar image view requires planar image info",
                });
            }
            if planes.len() != usize::from(info.components) {
                return Err(J2kError::InvalidParameter {
                    parameter: "planes",
                    message: "plane count must match image component count",
                });
            }
        }
        ImageView::Interleaved {
            info,
            samples,
            stride_bytes,
        } => {
            if info.layout != ComponentLayout::Interleaved {
                return Err(J2kError::InvalidParameter {
                    parameter: "info.layout",
                    message: "interleaved image view requires interleaved image info",
                });
            }
            validate_interleaved_view(
                "samples",
                samples.len(),
                info.width,
                info.height,
                info.components,
                *stride_bytes,
                info.sample_format,
            )?;
        }
    }

    Ok(())
}

fn validate_image_view_mut(image: &ImageViewMut<'_>) -> Result<()> {
    match image {
        ImageViewMut::Planar { info, planes } => {
            if info.layout != ComponentLayout::Planar {
                return Err(J2kError::InvalidParameter {
                    parameter: "info.layout",
                    message: "planar image view requires planar image info",
                });
            }
            if planes.len() != usize::from(info.components) {
                return Err(J2kError::InvalidParameter {
                    parameter: "planes",
                    message: "plane count must match image component count",
                });
            }
            for plane in planes.iter() {
                validate_plane(
                    "plane",
                    plane.samples.len(),
                    plane.width,
                    plane.height,
                    plane.stride_bytes,
                    plane.sample_format,
                )?;
            }
        }
        ImageViewMut::Interleaved {
            info,
            samples,
            stride_bytes,
        } => {
            if info.layout != ComponentLayout::Interleaved {
                return Err(J2kError::InvalidParameter {
                    parameter: "info.layout",
                    message: "interleaved image view requires interleaved image info",
                });
            }
            validate_interleaved_view(
                "samples",
                samples.len(),
                info.width,
                info.height,
                info.components,
                *stride_bytes,
                info.sample_format,
            )?;
        }
    }

    Ok(())
}

fn validate_interleaved_view(
    parameter: &'static str,
    len: usize,
    width: u32,
    height: u32,
    components: u16,
    stride_bytes: usize,
    sample_format: SampleFormat,
) -> Result<()> {
    if width == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "width",
            message: "image width must be greater than zero",
        });
    }
    if height == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "height",
            message: "image height must be greater than zero",
        });
    }
    if components == 0 {
        return Err(J2kError::InvalidParameter {
            parameter: "components",
            message: "image must contain at least one component",
        });
    }

    let row_bytes = checked_public_row_bytes(
        parameter,
        width,
        components,
        public_bytes_per_sample(parameter, sample_format)?,
    )?;
    if stride_bytes < row_bytes {
        return Err(J2kError::InvalidParameter {
            parameter,
            message: "interleaved stride must be at least one packed row",
        });
    }

    let required = stride_bytes
        .checked_mul(height as usize)
        .ok_or(J2kError::InvalidParameter {
            parameter,
            message: "image byte size overflowed usize",
        })?;
    if len < required {
        return Err(J2kError::BufferTooSmall {
            required,
            provided: len,
        });
    }

    Ok(())
}

fn public_bytes_per_sample(parameter: &'static str, sample_format: SampleFormat) -> Result<usize> {
    if sample_format.bits_per_sample == 0 {
        return Err(J2kError::InvalidParameter {
            parameter,
            message: "sample precision must be greater than zero",
        });
    }
    if sample_format.bits_per_sample <= 8 && sample_format.byte_order.is_some() {
        return Err(J2kError::InvalidParameter {
            parameter,
            message: "one-byte sample formats must not declare byte order",
        });
    }
    if sample_format.bits_per_sample > 8 && sample_format.byte_order.is_none() {
        return Err(J2kError::InvalidParameter {
            parameter,
            message: "multi-byte sample formats require explicit byte order",
        });
    }

    Ok(usize::from(sample_format.bits_per_sample).saturating_add(7) / 8)
}

fn checked_public_row_bytes(
    parameter: &'static str,
    width: u32,
    components: u16,
    bytes_per_sample: usize,
) -> Result<usize> {
    (width as usize)
        .checked_mul(usize::from(components))
        .and_then(|value| value.checked_mul(bytes_per_sample))
        .ok_or(J2kError::InvalidParameter {
            parameter,
            message: "packed row byte size overflowed usize",
        })
}

#[cfg(test)]
mod effective_coding_style_tests {
    use super::*;

    fn subsampled_fixture(width: u32, height: u32) -> (Vec<u8>, Vec<Vec<u8>>) {
        let sampling = [(1_u8, 1_u8), (2, 1), (2, 2)];
        let planes = sampling
            .iter()
            .enumerate()
            .map(|(component, (horizontal, vertical))| {
                let native_width = width.div_ceil(u32::from(*horizontal));
                let native_height = height.div_ceil(u32::from(*vertical));
                (0..native_width * native_height)
                    .map(|sample| {
                        u8::try_from(
                            (sample * (17 + component as u32 * 6)
                                + sample / native_width * 11
                                + component as u32 * 41)
                                % 251,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let components = planes
            .iter()
            .zip(sampling)
            .map(|(samples, (horizontal_separation, vertical_separation))| {
                codestream::SubsampledU8TestComponent {
                    horizontal_separation,
                    vertical_separation,
                    samples,
                }
            })
            .collect::<Vec<_>>();
        let codestream = codestream::encode_planar_u8_subsampled_no_decomp_test_fixture(
            width,
            height,
            &components,
        )
        .unwrap();
        (codestream, planes)
    }

    fn reduced_subsampled_fixture(width: u32, height: u32) -> Vec<u8> {
        let sampling = [(1_u8, 1_u8), (2, 1), (2, 2)];
        let planes = sampling
            .iter()
            .enumerate()
            .map(|(component, (horizontal, vertical))| {
                let native_width = width.div_ceil(u32::from(*horizontal));
                let native_height = height.div_ceil(u32::from(*vertical));
                (0..native_width * native_height)
                    .map(|sample| {
                        u8::try_from(
                            (sample * (13 + component as u32 * 10)
                                + sample / native_width * 7
                                + component as u32 * 53)
                                % 251,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let components = planes
            .iter()
            .zip(sampling)
            .map(|(samples, (horizontal_separation, vertical_separation))| {
                codestream::SubsampledU8TestComponent {
                    horizontal_separation,
                    vertical_separation,
                    samples,
                }
            })
            .collect::<Vec<_>>();
        codestream::encode_planar_u8_subsampled_one_decomp_test_fixture(width, height, &components)
            .unwrap()
    }

    fn two_level_reduced_subsampled_fixture(width: u32, height: u32) -> Vec<u8> {
        let sampling = [(1_u8, 1_u8), (2, 1), (2, 2)];
        let planes = sampling
            .iter()
            .enumerate()
            .map(|(component, (horizontal, vertical))| {
                let native_width = width.div_ceil(u32::from(*horizontal));
                let native_height = height.div_ceil(u32::from(*vertical));
                (0..native_width * native_height)
                    .map(|sample| {
                        u8::try_from(
                            (sample * (19 + component as u32 * 12)
                                + sample / native_width * 9
                                + component as u32 * 47)
                                % 251,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let components = planes
            .iter()
            .zip(sampling)
            .map(|(samples, (horizontal_separation, vertical_separation))| {
                codestream::SubsampledU8TestComponent {
                    horizontal_separation,
                    vertical_separation,
                    samples,
                }
            })
            .collect::<Vec<_>>();
        codestream::encode_planar_u8_subsampled_two_decomp_test_fixture(width, height, &components)
            .unwrap()
    }

    fn planar_bytes(image: &Image) -> &[Vec<u8>] {
        let ImageData::Planes(planes) = &image.data else {
            panic!("native component decode returned interleaved samples");
        };
        planes
    }

    fn native_multitile_fixture() -> (Vec<u8>, Vec<u8>) {
        let width = 131_u32;
        let height = 99_u32;
        let samples = (0..width * height)
            .map(|sample| u8::try_from((sample * 37 + sample / width * 19 + 11) % 251).unwrap())
            .collect::<Vec<_>>();
        let fixture = codestream::encode_grayscale_u8_two_decomp_multitile(
            codestream::GrayscaleU8Encode {
                width,
                height,
                samples: &samples,
                stride_bytes: usize::try_from(width).unwrap(),
            },
            codestream::TileSize {
                width: 64,
                height: 48,
            },
        )
        .unwrap();
        (fixture, samples)
    }

    fn native_origin_multitile_fixture() -> (Vec<u8>, Vec<u8>) {
        let width = 131_u32;
        let height = 99_u32;
        let samples = (0..width * height)
            .map(|sample| u8::try_from((sample * 43 + sample / width * 23 + 17) % 251).unwrap())
            .collect::<Vec<_>>();
        let fixture = codestream::encode_grayscale_u8_two_decomp_multitile_test_fixture(
            codestream::GrayscaleU8Encode {
                width,
                height,
                samples: &samples,
                stride_bytes: usize::try_from(width).unwrap(),
            },
            codestream::NativeMultitileTestGeometry {
                image_origin_x: 8,
                image_origin_y: 12,
                tile_origin_x: 4,
                tile_origin_y: 4,
                tile_width: 64,
                tile_height: 48,
            },
        )
        .unwrap();
        (fixture, samples)
    }

    fn native_quality_layer_fixture() -> (codestream::NativeQualityLayerTestFixture, Vec<u8>) {
        let width = 32_u32;
        let height = 32_u32;
        let samples = (0..width * height)
            .map(|index| {
                let x = index % width;
                let y = index / width;
                ((x * 29 + y * 47 + x * y * 7 + (x ^ y) * 11) & 0xff) as u8
            })
            .collect::<Vec<_>>();
        let fixture = codestream::encode_grayscale_u8_two_decomp_quality_layer_test_fixture(
            codestream::GrayscaleU8Encode {
                width,
                height,
                samples: &samples,
                stride_bytes: width as usize,
            },
        )
        .unwrap();
        (fixture, samples)
    }

    fn component_decode_options(max_quality_layers: Option<u16>) -> DecodeOptions {
        DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::All,
            max_quality_layers,
            target_layout: ComponentLayout::Planar,
            ..DecodeOptions::default()
        }
    }

    fn wrap_grayscale_jp2(codestream: &[u8], width: u32, height: u32) -> Vec<u8> {
        let info = ImageInfo::new(
            width,
            height,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Planar,
        )
        .unwrap();
        let mut output = Vec::new();
        write_jp2_encode_output(
            &info,
            codestream,
            &EncodeOptions {
                format: OutputFormat::Jp2,
                decomposition_levels: 2,
                ..EncodeOptions::default()
            },
            &mut output,
        )
        .unwrap();
        output
    }

    fn wrap_rgb_jp2(codestream: &[u8], width: u32, height: u32) -> Vec<u8> {
        let info = ImageInfo::new(
            width,
            height,
            3,
            SampleFormat::U8,
            ColorModel::Rgb,
            ComponentLayout::Planar,
        )
        .unwrap();
        let mut output = Vec::new();
        write_jp2_encode_output(
            &info,
            codestream,
            &EncodeOptions {
                format: OutputFormat::Jp2,
                decomposition_levels: 0,
                ..EncodeOptions::default()
            },
            &mut output,
        )
        .unwrap();
        output
    }

    fn jp2_wrapped_native_multitile_fixture(codestream: &[u8]) -> Vec<u8> {
        let info = ImageInfo::new(
            131,
            99,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Planar,
        )
        .unwrap();
        let mut output = Vec::new();
        write_jp2_encode_output(
            &info,
            codestream,
            &EncodeOptions {
                format: OutputFormat::Jp2,
                decomposition_levels: 2,
                tile_size: Some(TileSize {
                    width: 64,
                    height: 48,
                }),
                ..EncodeOptions::default()
            },
            &mut output,
        )
        .unwrap();
        output
    }

    fn crop_u8(samples: &[u8], image_width: u32, region: Region) -> Vec<u8> {
        let image_width = usize::try_from(image_width).unwrap();
        let x = usize::try_from(region.x).unwrap();
        let y = usize::try_from(region.y).unwrap();
        let width = usize::try_from(region.width).unwrap();
        (0..usize::try_from(region.height).unwrap())
            .flat_map(|row| {
                let start = (y + row) * image_width + x;
                samples[start..start + width].iter().copied()
            })
            .collect()
    }

    fn execute_prepared_u8(
        prepared: &PreparedPart1Decode<'_>,
        route: Option<codestream::SynthesisCrossoverRoute>,
    ) -> (Vec<u8>, codestream::DecodeStageTimings) {
        let component = &prepared.component_info()[0];
        let stride = usize::try_from(component.width).unwrap();
        let mut samples = vec![0xa5; stride * usize::try_from(component.height).unwrap()];
        let timings = {
            let plane = PlaneMut::new(
                &mut samples,
                component.width,
                component.height,
                stride,
                component.sample_format,
            )
            .unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: prepared.info(),
                planes: &mut planes,
            };
            execute_prepared_part1_decode_into_with_workspace(
                prepared,
                &mut target,
                &mut Part1DecodeWorkspace::new(),
                codestream::PreparedPart1ExecutionOptions {
                    instrumentation: codestream::DecodeInstrumentation::DetailedProfile,
                    collect_tier1_work_counters: true,
                    parallelism: codestream::DecodeExecutionParallelism::Serial,
                    synthesis_crossover_route: route,
                    ..codestream::PreparedPart1ExecutionOptions::default()
                },
            )
            .unwrap()
        };
        (samples, timings)
    }

    #[test]
    fn genuine_quality_layers_are_exact_across_owned_caller_info_and_prepared_routes() {
        let (fixture, samples) = native_quality_layer_fixture();
        let full_options = component_decode_options(None);
        let limited_options = component_decode_options(Some(1));
        let oracle = decode(&fixture.one_layer_oracle, &full_options).unwrap();
        let full = decode(&fixture.two_layer_codestream, &full_options).unwrap();
        let limited = decode(&fixture.two_layer_codestream, &limited_options).unwrap();
        assert_eq!(planar_bytes(&limited), planar_bytes(&oracle));
        assert_ne!(planar_bytes(&limited), planar_bytes(&full));
        assert_eq!(planar_bytes(&full)[0], samples);
        for limit in [Some(2), Some(9)] {
            assert_eq!(
                decode(
                    &fixture.two_layer_codestream,
                    &component_decode_options(limit)
                )
                .unwrap(),
                full
            );
        }
        assert_eq!(
            decode_shape(&fixture.two_layer_codestream, &limited_options).unwrap(),
            decode_shape(&fixture.two_layer_codestream, &full_options).unwrap()
        );

        let mut caller = vec![0xa5; samples.len()];
        let info = limited.info.clone();
        {
            let plane = PlaneMut::new(&mut caller, 32, 32, 32, SampleFormat::U8).unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_into(&fixture.two_layer_codestream, &mut target, &limited_options).unwrap();
        }
        assert_eq!(caller, planar_bytes(&oracle)[0]);

        let limited_partial = PartialDecodeOptions {
            max_quality_layers: Some(1),
            ..PartialDecodeOptions::default()
        };
        let full_partial = PartialDecodeOptions::default();
        let partial = decode_partial(&fixture.two_layer_codestream, &limited_partial).unwrap();
        assert_eq!(planar_bytes(&partial), planar_bytes(&oracle));
        assert_eq!(
            decode_partial_info(&fixture.two_layer_codestream, &limited_partial).unwrap(),
            limited.info
        );
        assert_eq!(
            decode_partial_component_info(&fixture.two_layer_codestream, &limited_partial).unwrap(),
            limited.component_info
        );
        for limit in [Some(2), Some(9)] {
            let clamped = PartialDecodeOptions {
                max_quality_layers: limit,
                ..PartialDecodeOptions::default()
            };
            assert_eq!(
                decode_partial(&fixture.two_layer_codestream, &clamped).unwrap(),
                full
            );
            let prepared = prepare_part1_decode(&fixture.two_layer_codestream, &clamped).unwrap();
            assert_eq!(execute_prepared_u8(&prepared, None).0, samples);
            let source = codestream::source::SliceSource::new(&fixture.two_layer_codestream);
            let source_prepared = prepare_part1_decode_from_source(
                &source,
                codestream::Part1ComponentDecodeRequest {
                    component_indices: &[0],
                    region: codestream::TileRegionRequest {
                        x: 0,
                        y: 0,
                        width: 32,
                        height: 32,
                    },
                    discard_levels: 0,
                    max_layers: limit,
                },
            )
            .unwrap();
            assert_eq!(execute_prepared_u8(&source_prepared, None).0, samples);
        }

        let limited_prepared =
            prepare_part1_decode(&fixture.two_layer_codestream, &limited_partial).unwrap();
        let full_prepared =
            prepare_part1_decode(&fixture.two_layer_codestream, &full_partial).unwrap();
        let (limited_prepared_samples, limited_work) = execute_prepared_u8(&limited_prepared, None);
        let (full_prepared_samples, full_work) = execute_prepared_u8(&full_prepared, None);
        assert_eq!(limited_prepared_samples, planar_bytes(&oracle)[0]);
        assert_eq!(full_prepared_samples, samples);
        assert_eq!(
            limited_prepared.preparation_timings().packet_headers_parsed,
            full_prepared.preparation_timings().packet_headers_parsed
        );
        assert_eq!(
            limited_prepared.preparation_timings().packet_headers_parsed,
            6
        );
        assert!(
            limited_prepared
                .preparation_timings()
                .packet_body_bytes_skipped
                > 0
        );
        assert!(limited_work.tier1_codeword_bytes < full_work.tier1_codeword_bytes);
        let tier1_positions = |work: &codestream::DecodeStageTimings| {
            work.tier1_work_counters
                .cleanup_positions_visited
                .saturating_add(work.tier1_work_counters.significance_positions_visited)
                .saturating_add(work.tier1_work_counters.magnitude_positions_visited)
        };
        assert!(tier1_positions(&limited_work) < tier1_positions(&full_work));

        let limited_source = codestream::source::InstrumentedSource::new(
            codestream::source::SliceSource::new(&fixture.two_layer_codestream),
        );
        let limited_source_prepared = prepare_part1_decode_from_source(
            &limited_source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: codestream::TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 32,
                    height: 32,
                },
                discard_levels: 0,
                max_layers: Some(1),
            },
        )
        .unwrap();
        let (source_samples, source_work) = execute_prepared_u8(&limited_source_prepared, None);
        assert_eq!(source_samples, planar_bytes(&oracle)[0]);
        assert_eq!(
            source_work.tier1_codeword_bytes,
            limited_work.tier1_codeword_bytes
        );
        let skipped_second_layer_bytes = fixture
            .second_layer_body_ranges
            .iter()
            .map(|range| u64::try_from(range.len()).unwrap())
            .sum::<u64>();
        assert!(limited_source.metrics().packet_body_bytes_not_read >= skipped_second_layer_bytes);

        let full_source = codestream::source::InstrumentedSource::new(
            codestream::source::SliceSource::new(&fixture.two_layer_codestream),
        );
        let full_source_prepared = prepare_part1_decode_from_source(
            &full_source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: codestream::TileRegionRequest {
                    x: 0,
                    y: 0,
                    width: 32,
                    height: 32,
                },
                discard_levels: 0,
                max_layers: None,
            },
        )
        .unwrap();
        assert_eq!(execute_prepared_u8(&full_source_prepared, None).0, samples);
        assert!(
            limited_source.metrics().source_bytes_returned
                < full_source.metrics().source_bytes_returned
        );
    }

    #[test]
    fn quality_layer_corruption_and_exclusions_fail_closed() {
        let (fixture, _) = native_quality_layer_fixture();
        let limited = component_decode_options(Some(1));
        let full = component_decode_options(None);
        let oracle = decode(&fixture.one_layer_oracle, &full).unwrap();

        let mut ignored_second_layer_corruption = None;
        'second: for range in &fixture.second_layer_body_ranges {
            for offset in range.start..range.end.saturating_sub(1) {
                let mut candidate = fixture.two_layer_codestream.clone();
                candidate[offset] = 0xff;
                candidate[offset + 1] = 0x90;
                if decode(&candidate, &full).is_err()
                    && decode(&candidate, &limited).is_ok_and(|image| image == oracle)
                {
                    ignored_second_layer_corruption = Some(candidate);
                    break 'second;
                }
            }
        }
        assert!(ignored_second_layer_corruption.is_some());

        let mut malformed_second_layer_header = false;
        'header: for offset in &fixture.second_layer_header_offsets {
            for mask in [0x80, 0x40, 0x20, 0x10, 0xff] {
                let mut candidate = fixture.two_layer_codestream.clone();
                candidate[*offset] ^= mask;
                if decode(&candidate, &limited).is_err() {
                    malformed_second_layer_header = true;
                    break 'header;
                }
            }
        }
        assert!(malformed_second_layer_header);

        let mut failing_first_layer = None;
        'first: for range in &fixture.first_layer_body_ranges {
            for offset in range.start..range.end.saturating_sub(1) {
                let mut candidate = fixture.two_layer_codestream.clone();
                candidate[offset] = 0xff;
                candidate[offset + 1] = 0x90;
                if decode(&candidate, &limited).is_err() {
                    failing_first_layer = Some(candidate);
                    break 'first;
                }
            }
        }
        let failing_first_layer = failing_first_layer.expect("first-layer entropy corruption");
        let mut caller = vec![0xa5; 32 * 32];
        let info = oracle.info.clone();
        {
            let plane = PlaneMut::new(&mut caller, 32, 32, 32, SampleFormat::U8).unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_into(&failing_first_layer, &mut target, &limited).is_err());
        }
        assert!(caller.iter().all(|sample| *sample == 0xa5));

        let assert_rejected_across_routes = |candidate: &[u8], width: u32, height: u32| {
            let partial = PartialDecodeOptions {
                max_quality_layers: Some(1),
                ..PartialDecodeOptions::default()
            };
            assert!(decode(candidate, &limited).is_err());
            assert!(decode_shape(candidate, &limited).is_err());
            assert!(decode_partial(candidate, &partial).is_err());
            assert!(decode_partial_info(candidate, &partial).is_err());
            assert!(decode_partial_component_info(candidate, &partial).is_err());
            assert!(prepare_part1_decode(candidate, &partial).is_err());
            assert!(
                prepare_part1_decode_from_source(
                    &codestream::source::SliceSource::new(candidate),
                    codestream::Part1ComponentDecodeRequest {
                        component_indices: &[0],
                        region: codestream::TileRegionRequest {
                            x: 0,
                            y: 0,
                            width,
                            height,
                        },
                        discard_levels: 0,
                        max_layers: Some(1),
                    },
                )
                .is_err()
            );
        };
        let zero = component_decode_options(Some(0));
        assert!(decode(&fixture.two_layer_codestream, &zero).is_err());
        assert!(decode_shape(&fixture.two_layer_codestream, &zero).is_err());
        let zero_partial = PartialDecodeOptions {
            max_quality_layers: Some(0),
            ..PartialDecodeOptions::default()
        };
        assert!(decode_partial(&fixture.two_layer_codestream, &zero_partial).is_err());
        assert!(decode_partial_info(&fixture.two_layer_codestream, &zero_partial).is_err());
        assert!(
            decode_partial_component_info(&fixture.two_layer_codestream, &zero_partial).is_err()
        );
        assert!(prepare_part1_decode(&fixture.two_layer_codestream, &zero_partial).is_err());
        assert!(
            prepare_part1_decode_from_source(
                &codestream::source::SliceSource::new(&fixture.two_layer_codestream),
                codestream::Part1ComponentDecodeRequest {
                    component_indices: &[0],
                    region: codestream::TileRegionRequest {
                        x: 0,
                        y: 0,
                        width: 32,
                        height: 32,
                    },
                    discard_levels: 0,
                    max_layers: Some(0),
                },
            )
            .is_err()
        );
        let container = wrap_grayscale_jp2(&fixture.two_layer_codestream, 32, 32);
        assert_rejected_across_routes(&container, 32, 32);

        let partial_exclusions = [
            PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 16,
                    height: 32,
                }),
                max_quality_layers: Some(1),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                tile: Some(TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                }),
                max_quality_layers: Some(1),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 0 },
                max_quality_layers: Some(1),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                max_quality_layers: Some(1),
                ..PartialDecodeOptions::default()
            },
        ];
        assert!(partial_exclusions.iter().all(|options| {
            decode_partial(&fixture.two_layer_codestream, options).is_err()
                && decode_partial_info(&fixture.two_layer_codestream, options).is_err()
                && prepare_part1_decode(&fixture.two_layer_codestream, options).is_err()
        }));

        let marker = |bytes: &[u8], marker| marker_offset(bytes, marker, 0);
        let mut structural = Vec::new();
        let mut changed = fixture.two_layer_codestream.clone();
        let siz = marker(&changed, codestream::Marker::Siz);
        changed[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let siz = marker(&changed, codestream::Marker::Siz);
        changed[siz + 40] |= 0x80;
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let siz = marker(&changed, codestream::Marker::Siz);
        changed[siz + 40] = 8;
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let siz = marker(&changed, codestream::Marker::Siz);
        changed[siz + 41] = 2;
        structural.push(changed);
        for (offset, value) in [
            (4, 2),
            (4, 4),
            (5, 1),
            (8, 1),
            (9, 1),
            (10, 3),
            (11, 3),
            (12, 0),
            (12, 1),
            (13, 0),
        ] {
            let mut changed = fixture.two_layer_codestream.clone();
            let cod = marker(&changed, codestream::Marker::Cod);
            changed[cod + offset] = value;
            structural.push(changed);
        }
        let mut changed = fixture.two_layer_codestream.clone();
        let cod = marker(&changed, codestream::Marker::Cod);
        let cod_len = u16::from_be_bytes(changed[cod + 2..cod + 4].try_into().unwrap());
        changed[cod + 4] |= 1;
        changed[cod + 2..cod + 4].copy_from_slice(&(cod_len + 3).to_be_bytes());
        changed.splice(
            cod + 2 + usize::from(cod_len)..cod + 2 + usize::from(cod_len),
            [0x44; 3],
        );
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let cod = marker(&changed, codestream::Marker::Cod);
        changed[cod + 7] = 3;
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let sot = marker(&changed, codestream::Marker::Sot);
        changed.splice(sot..sot, [0xff, 0x53, 0, 9, 0, 0, 2, 4, 4, 0, 1]);
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let qcd = marker(&changed, codestream::Marker::Qcd);
        let qcd_len = usize::from(u16::from_be_bytes(
            changed[qcd + 2..qcd + 4].try_into().unwrap(),
        ));
        let mut qcc = vec![0xff, 0x5d];
        qcc.extend_from_slice(&u16::try_from(qcd_len + 1).unwrap().to_be_bytes());
        qcc.push(0);
        qcc.extend_from_slice(&changed[qcd + 4..qcd + 2 + qcd_len]);
        let sot = marker(&changed, codestream::Marker::Sot);
        changed.splice(sot..sot, qcc);
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let sot = marker(&changed, codestream::Marker::Sot);
        changed.splice(sot..sot, [0xff, 0x50, 0, 6, 0, 0, 0, 0]);
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let sot = marker(&changed, codestream::Marker::Sot);
        changed.splice(sot..sot, [0xff, 0x64, 0, 3, 0]);
        structural.push(changed);
        let mut changed = fixture.two_layer_codestream.clone();
        let sot = marker(&changed, codestream::Marker::Sot);
        changed[sot + 11] = 0;
        structural.push(changed);
        for candidate in &structural {
            assert_rejected_across_routes(candidate, 32, 32);
        }

        let rgb = vec![91_u8; 16 * 16 * 3];
        let multiple_components = codestream::encode_rgb_u8_two_decomp(codestream::RgbU8Encode {
            width: 16,
            height: 16,
            samples: &rgb,
            stride_bytes: 16 * 3,
        })
        .unwrap();
        let declare_two_layers = |mut candidate: Vec<u8>| {
            let cod = marker(&candidate, codestream::Marker::Cod);
            candidate[cod + 6..cod + 8].copy_from_slice(&2_u16.to_be_bytes());
            candidate
        };
        assert_rejected_across_routes(&declare_two_layers(multiple_components), 16, 16);
        let ht = codestream::encode_htj2k_rgb_u8_no_decomp(codestream::RgbU8Encode {
            width: 16,
            height: 16,
            samples: &rgb,
            stride_bytes: 16 * 3,
        })
        .unwrap();
        assert_rejected_across_routes(&declare_two_layers(ht), 16, 16);

        let (multitile, _) = native_multitile_fixture();
        assert_rejected_across_routes(&declare_two_layers(multitile), 131, 99);
        let rendered = DecodeOptions {
            mode: DecodeMode::Rendered,
            max_quality_layers: Some(1),
            ..DecodeOptions::default()
        };
        assert!(decode(&fixture.two_layer_codestream, &rendered).is_err());
    }

    #[test]
    fn native_multitile_partial_regions_tiles_routes_and_work_are_exact() {
        let (fixture, samples) = native_multitile_fixture();
        let full_options = PartialDecodeOptions::default();
        let full = decode_partial(&fixture, &full_options).unwrap();
        assert_eq!((full.info.width, full.info.height), (131, 99));
        assert_eq!(planar_bytes(&full)[0], samples);

        let requests = [
            Region {
                x: 8,
                y: 9,
                width: 20,
                height: 17,
            },
            Region {
                x: 60,
                y: 8,
                width: 10,
                height: 21,
            },
            Region {
                x: 7,
                y: 44,
                width: 23,
                height: 11,
            },
            Region {
                x: 50,
                y: 38,
                width: 30,
                height: 25,
            },
        ];
        for region in requests {
            let decoded = decode_partial(
                &fixture,
                &PartialDecodeOptions {
                    region: Some(region),
                    ..PartialDecodeOptions::default()
                },
            )
            .unwrap();
            assert_eq!(planar_bytes(&decoded)[0], crop_u8(&samples, 131, region));
        }

        for (tile, region) in [
            (
                TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                },
                Region {
                    x: 0,
                    y: 0,
                    width: 64,
                    height: 48,
                },
            ),
            (
                TileSelection {
                    tile_x: 2,
                    tile_y: 0,
                },
                Region {
                    x: 128,
                    y: 0,
                    width: 3,
                    height: 48,
                },
            ),
            (
                TileSelection {
                    tile_x: 0,
                    tile_y: 2,
                },
                Region {
                    x: 0,
                    y: 96,
                    width: 64,
                    height: 3,
                },
            ),
            (
                TileSelection {
                    tile_x: 2,
                    tile_y: 2,
                },
                Region {
                    x: 128,
                    y: 96,
                    width: 3,
                    height: 3,
                },
            ),
        ] {
            let options = PartialDecodeOptions {
                tile: Some(tile),
                ..PartialDecodeOptions::default()
            };
            let decoded = decode_partial(&fixture, &options).unwrap();
            assert_eq!(
                (decoded.info.width, decoded.info.height),
                (region.width, region.height)
            );
            assert_eq!(planar_bytes(&decoded)[0], crop_u8(&samples, 131, region));
            assert_eq!(
                plan_partial_decode_work(&fixture, &options)
                    .unwrap()
                    .selected_tiles
                    .len(),
                1
            );
        }

        let crossing = Region {
            x: 50,
            y: 38,
            width: 30,
            height: 25,
        };
        let crossing_options = PartialDecodeOptions {
            region: Some(crossing),
            ..PartialDecodeOptions::default()
        };
        let full_plan = plan_partial_decode_work(&fixture, &full_options).unwrap();
        let crossing_plan = plan_partial_decode_work(&fixture, &crossing_options).unwrap();
        assert_eq!(full_plan.selected_tiles.len(), 9);
        assert_eq!(crossing_plan.selected_tiles.len(), 4);
        assert!(!full_plan.full_image_full_resolution_fallback);
        assert!(!crossing_plan.full_image_full_resolution_fallback);
        assert_eq!(
            crossing_plan.evidence,
            PartialDecodePlanEvidence::TrueCodestreamPartialCandidate
        );

        let full_prepared = prepare_part1_decode(&fixture, &full_options).unwrap();
        let crossing_prepared = prepare_part1_decode(&fixture, &crossing_options).unwrap();
        assert_eq!(full_prepared.codestream.selected_tile_count(), 9);
        assert_eq!(crossing_prepared.codestream.selected_tile_count(), 4);
        let (full_prepared_samples, full_work) = execute_prepared_u8(&full_prepared, None);
        let (windowed_samples, windowed_work) = execute_prepared_u8(&crossing_prepared, None);
        let crossing_crop = crop_u8(&samples, 131, crossing);
        assert_eq!(full_prepared_samples, samples);
        assert_eq!(windowed_samples, crossing_crop);
        assert!(windowed_work.windowed_synthesis_component_tiles > 0);
        assert!(windowed_work.executed_code_blocks < full_work.executed_code_blocks);
        assert!(windowed_work.output_samples < full_work.output_samples);
        assert!(
            crossing_prepared.preparation_timings().prepared_code_blocks
                < full_prepared.preparation_timings().prepared_code_blocks
        );
        let whole_tile = PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 1,
                tile_y: 0,
            }),
            ..PartialDecodeOptions::default()
        };
        let whole_tile_prepared = prepare_part1_decode(&fixture, &whole_tile).unwrap();
        let (whole_tile_samples, whole_tile_work) = execute_prepared_u8(&whole_tile_prepared, None);
        assert_eq!(
            whole_tile_samples,
            crop_u8(
                &samples,
                131,
                Region {
                    x: 64,
                    y: 0,
                    width: 64,
                    height: 48,
                }
            )
        );
        assert!(whole_tile_work.full_synthesis_component_tiles > 0);
        assert_eq!(whole_tile_work.windowed_synthesis_component_tiles, 0);

        let all = decode_partial(&fixture, &crossing_options).unwrap();
        let selected = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![0]),
                max_quality_layers: Some(1),
                resolution: ResolutionLevel::Reduced { discard_levels: 0 },
                ..crossing_options.clone()
            },
        )
        .unwrap();
        assert_eq!(selected, all);

        let descriptors = decode_partial_component_info(&fixture, &crossing_options).unwrap();
        let stride = usize::try_from(crossing.width + 9).unwrap();
        let mut padded = vec![0x6d; stride * usize::try_from(crossing.height).unwrap()];
        {
            let plane = PlaneMut::new(
                &mut padded,
                crossing.width,
                crossing.height,
                stride,
                descriptors[0].sample_format,
            )
            .unwrap();
            let mut planes = [plane];
            let info = decode_partial_info(&fixture, &crossing_options).unwrap();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_partial_into(&fixture, &mut target, &crossing_options).unwrap();
        }
        for (row, expected) in padded
            .chunks_exact(stride)
            .zip(crossing_crop.chunks_exact(usize::try_from(crossing.width).unwrap()))
        {
            assert_eq!(&row[..usize::try_from(crossing.width).unwrap()], expected);
            assert!(
                row[usize::try_from(crossing.width).unwrap()..]
                    .iter()
                    .all(|sample| *sample == 0x6d)
            );
        }

        let source = codestream::source::SliceSource::new(&fixture);
        let source_prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: codestream::TileRegionRequest {
                    x: crossing.x,
                    y: crossing.y,
                    width: crossing.width,
                    height: crossing.height,
                },
                discard_levels: 0,
                max_layers: Some(1),
            },
        )
        .unwrap();
        assert_eq!(source_prepared.codestream.selected_tile_count(), 4);
        assert_eq!(execute_prepared_u8(&source_prepared, None).0, crossing_crop);
    }

    #[test]
    fn phase_aligned_native_origins_are_exact_across_partial_routes() {
        let (fixture, samples) = native_origin_multitile_fixture();
        let parsed = codestream::parse(&fixture).unwrap();
        assert_eq!(
            (
                parsed.siz.reference_grid_width,
                parsed.siz.reference_grid_height,
                parsed.siz.image_origin_x,
                parsed.siz.image_origin_y,
                parsed.siz.tile_origin_x,
                parsed.siz.tile_origin_y,
                parsed.siz.tile_width,
                parsed.siz.tile_height,
            ),
            (139, 111, 8, 12, 4, 4, 64, 48)
        );
        assert!(codestream::is_supported_part1_native_multitile_partial_profile(&fixture, &parsed));

        let partial_full = decode_partial(&fixture, &PartialDecodeOptions::default()).unwrap();
        assert_eq!(
            (partial_full.info.width, partial_full.info.height),
            (131, 99)
        );
        assert_eq!(planar_bytes(&partial_full)[0], samples);

        let full = decode(
            &fixture,
            &DecodeOptions {
                mode: DecodeMode::Components,
                ..DecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!((full.info.width, full.info.height), (131, 99));
        assert_eq!(planar_bytes(&full)[0], samples);
        assert_eq!(
            (
                full.component_info[0].x_origin,
                full.component_info[0].y_origin,
                full.component_info[0].width,
                full.component_info[0].height,
            ),
            (8, 12, 131, 99)
        );

        for region in [
            Region {
                x: 8,
                y: 9,
                width: 20,
                height: 17,
            },
            Region {
                x: 54,
                y: 8,
                width: 12,
                height: 21,
            },
            Region {
                x: 7,
                y: 35,
                width: 23,
                height: 11,
            },
            Region {
                x: 50,
                y: 32,
                width: 30,
                height: 25,
            },
        ] {
            let options = PartialDecodeOptions {
                region: Some(region),
                ..PartialDecodeOptions::default()
            };
            let decoded = decode_partial(&fixture, &options).unwrap();
            assert_eq!(planar_bytes(&decoded)[0], crop_u8(&samples, 131, region));
            let descriptor = decode_partial_component_info(&fixture, &options).unwrap()[0].clone();
            assert_eq!(
                (
                    descriptor.x_origin,
                    descriptor.y_origin,
                    descriptor.width,
                    descriptor.height,
                ),
                (8 + region.x, 12 + region.y, region.width, region.height)
            );
        }

        for (tile, region) in [
            (
                TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                },
                Region {
                    x: 0,
                    y: 0,
                    width: 60,
                    height: 40,
                },
            ),
            (
                TileSelection {
                    tile_x: 2,
                    tile_y: 0,
                },
                Region {
                    x: 124,
                    y: 0,
                    width: 7,
                    height: 40,
                },
            ),
            (
                TileSelection {
                    tile_x: 0,
                    tile_y: 2,
                },
                Region {
                    x: 0,
                    y: 88,
                    width: 60,
                    height: 11,
                },
            ),
            (
                TileSelection {
                    tile_x: 2,
                    tile_y: 2,
                },
                Region {
                    x: 124,
                    y: 88,
                    width: 7,
                    height: 11,
                },
            ),
        ] {
            let options = PartialDecodeOptions {
                tile: Some(tile),
                ..PartialDecodeOptions::default()
            };
            let decoded = decode_partial(&fixture, &options).unwrap();
            assert_eq!(planar_bytes(&decoded)[0], crop_u8(&samples, 131, region));
            assert_eq!(
                plan_partial_decode_work(&fixture, &options)
                    .unwrap()
                    .selected_tiles
                    .len(),
                1
            );
        }

        let crossing = Region {
            x: 50,
            y: 32,
            width: 30,
            height: 25,
        };
        let options = PartialDecodeOptions {
            region: Some(crossing),
            ..PartialDecodeOptions::default()
        };
        assert_eq!(
            decode_partial_info(&fixture, &options).unwrap().width,
            crossing.width
        );
        let plan = plan_partial_decode_work(&fixture, &options).unwrap();
        assert_eq!(plan.selected_tiles.len(), 4);
        assert_eq!(
            (
                plan.selected_resolution.width,
                plan.selected_resolution.height
            ),
            (crossing.width, crossing.height)
        );
        assert!(!plan.full_image_full_resolution_fallback);
        let prepared = prepare_part1_decode(&fixture, &options).unwrap();
        assert_eq!(prepared.codestream.selected_tile_count(), 4);
        assert_eq!(
            (
                prepared.component_info()[0].x_origin,
                prepared.component_info()[0].y_origin,
            ),
            (58, 44)
        );
        let (prepared_samples, work) = execute_prepared_u8(&prepared, None);
        let expected = crop_u8(&samples, 131, crossing);
        assert_eq!(prepared_samples, expected);
        assert!(work.executed_code_blocks > 0);
        assert!(work.windowed_synthesis_component_tiles > 0);

        let stride = usize::try_from(crossing.width + 9).unwrap();
        let mut padded = vec![0x6d; stride * usize::try_from(crossing.height).unwrap()];
        {
            let plane = PlaneMut::new(
                &mut padded,
                crossing.width,
                crossing.height,
                stride,
                SampleFormat::U8,
            )
            .unwrap();
            let mut planes = [plane];
            let info = decode_partial_info(&fixture, &options).unwrap();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            decode_partial_into(&fixture, &mut target, &options).unwrap();
        }
        for (actual, expected) in padded
            .chunks_exact(stride)
            .zip(expected.chunks_exact(usize::try_from(crossing.width).unwrap()))
        {
            assert_eq!(&actual[..expected.len()], expected);
            assert!(
                actual[expected.len()..]
                    .iter()
                    .all(|sample| *sample == 0x6d)
            );
        }

        let source = codestream::source::SliceSource::new(&fixture);
        let source_prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: codestream::TileRegionRequest {
                    x: crossing.x,
                    y: crossing.y,
                    width: crossing.width,
                    height: crossing.height,
                },
                discard_levels: 0,
                max_layers: Some(1),
            },
        )
        .unwrap();
        assert_eq!(source_prepared.codestream.selected_tile_count(), 4);
        assert_eq!(
            (
                source_prepared.component_info()[0].x_origin,
                source_prepared.component_info()[0].y_origin,
            ),
            (58, 44)
        );
        assert_eq!(execute_prepared_u8(&source_prepared, None).0, expected);

        let clipped_tile_options = PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 2,
                tile_y: 2,
            }),
            ..PartialDecodeOptions::default()
        };
        let clipped_tile_prepared = prepare_part1_decode(&fixture, &clipped_tile_options).unwrap();
        let (clipped_tile_samples, clipped_tile_work) =
            execute_prepared_u8(&clipped_tile_prepared, None);
        assert_eq!(
            clipped_tile_samples,
            crop_u8(
                &samples,
                131,
                Region {
                    x: 124,
                    y: 88,
                    width: 7,
                    height: 11,
                }
            )
        );
        assert!(clipped_tile_work.full_synthesis_component_tiles > 0);
        assert_eq!(clipped_tile_work.windowed_synthesis_component_tiles, 0);
    }

    #[test]
    fn native_multitile_partial_partition_stitches_to_one_call() {
        let (fixture, samples) = native_multitile_fixture();
        let whole = Region {
            x: 50,
            y: 38,
            width: 30,
            height: 25,
        };
        let mut stitched = vec![0_u8; usize::try_from(whole.width * whole.height).unwrap()];
        for part in [
            Region {
                x: 50,
                y: 38,
                width: 14,
                height: 10,
            },
            Region {
                x: 64,
                y: 38,
                width: 16,
                height: 10,
            },
            Region {
                x: 50,
                y: 48,
                width: 14,
                height: 15,
            },
            Region {
                x: 64,
                y: 48,
                width: 16,
                height: 15,
            },
        ] {
            let decoded = decode_partial(
                &fixture,
                &PartialDecodeOptions {
                    region: Some(part),
                    ..PartialDecodeOptions::default()
                },
            )
            .unwrap();
            let local_x = usize::try_from(part.x - whole.x).unwrap();
            let local_y = usize::try_from(part.y - whole.y).unwrap();
            let width = usize::try_from(part.width).unwrap();
            for (row, source) in planar_bytes(&decoded)[0].chunks_exact(width).enumerate() {
                let start = (local_y + row) * usize::try_from(whole.width).unwrap() + local_x;
                stitched[start..start + width].copy_from_slice(source);
            }
        }
        assert_eq!(stitched, crop_u8(&samples, 131, whole));
        let one_call = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(whole),
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!(stitched, planar_bytes(&one_call)[0]);
    }

    #[test]
    fn phase_aligned_native_origin_partition_stitches_to_one_call() {
        let (fixture, samples) = native_origin_multitile_fixture();
        let whole = Region {
            x: 50,
            y: 32,
            width: 30,
            height: 25,
        };
        let mut stitched = vec![0_u8; usize::try_from(whole.width * whole.height).unwrap()];
        for part in [
            Region {
                x: 50,
                y: 32,
                width: 10,
                height: 8,
            },
            Region {
                x: 60,
                y: 32,
                width: 20,
                height: 8,
            },
            Region {
                x: 50,
                y: 40,
                width: 10,
                height: 17,
            },
            Region {
                x: 60,
                y: 40,
                width: 20,
                height: 17,
            },
        ] {
            let decoded = decode_partial(
                &fixture,
                &PartialDecodeOptions {
                    region: Some(part),
                    ..PartialDecodeOptions::default()
                },
            )
            .unwrap();
            let local_x = usize::try_from(part.x - whole.x).unwrap();
            let local_y = usize::try_from(part.y - whole.y).unwrap();
            let width = usize::try_from(part.width).unwrap();
            for (row, source) in planar_bytes(&decoded)[0].chunks_exact(width).enumerate() {
                let start = (local_y + row) * usize::try_from(whole.width).unwrap() + local_x;
                stitched[start..start + width].copy_from_slice(source);
            }
        }
        assert_eq!(stitched, crop_u8(&samples, 131, whole));
        let one_call = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(whole),
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!(stitched, planar_bytes(&one_call)[0]);
    }

    fn marker_offset(input: &[u8], marker: codestream::Marker, occurrence: usize) -> usize {
        codestream::parse(input)
            .unwrap()
            .markers
            .iter()
            .filter(|segment| segment.marker == marker)
            .nth(occurrence)
            .unwrap()
            .offset
    }

    fn insert_before_first_sot(mut input: Vec<u8>, segment: &[u8]) -> Vec<u8> {
        let sot = marker_offset(&input, codestream::Marker::Sot, 0);
        input.splice(sot..sot, segment.iter().copied());
        input
    }

    fn insert_first_tile_header_segment(mut input: Vec<u8>, segment: &[u8]) -> Vec<u8> {
        let sot = marker_offset(&input, codestream::Marker::Sot, 0);
        let sod = marker_offset(&input, codestream::Marker::Sod, 0);
        let psot = u32::from_be_bytes(input[sot + 6..sot + 10].try_into().unwrap());
        input[sot + 6..sot + 10].copy_from_slice(
            &psot
                .checked_add(u32::try_from(segment.len()).unwrap())
                .unwrap()
                .to_be_bytes(),
        );
        input.splice(sod..sod, segment.iter().copied());
        input
    }

    fn assert_partial_rejected_without_mutation(
        input: &[u8],
        options: &PartialDecodeOptions,
        target_info: &ImageInfo,
    ) {
        let stride = usize::try_from(target_info.width).unwrap() + 7;
        let mut samples = vec![0x9b; stride * usize::try_from(target_info.height).unwrap()];
        {
            let plane = PlaneMut::new(
                &mut samples,
                target_info.width,
                target_info.height,
                stride,
                target_info.sample_format,
            )
            .unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: target_info,
                planes: &mut planes,
            };
            assert!(
                decode_partial_into(input, &mut target, options).is_err(),
                "request unexpectedly succeeded: {options:?}"
            );
        }
        assert!(samples.iter().all(|sample| *sample == 0x9b));
    }

    fn assert_native_partial_rejected_across_routes(
        input: &[u8],
        options: &PartialDecodeOptions,
        target_info: &ImageInfo,
    ) {
        assert!(decode_partial(input, options).is_err());
        assert!(decode_partial_info(input, options).is_err());
        assert!(decode_partial_component_info(input, options).is_err());
        assert!(prepare_part1_decode(input, options).is_err());
        assert!(plan_partial_decode_work(input, options).is_err());
        assert_partial_rejected_without_mutation(input, options, target_info);
    }

    fn assert_native_origin_source_rejected(input: &[u8], region: Region) {
        let source = codestream::source::SliceSource::new(input);
        assert!(
            prepare_part1_decode_from_source(
                &source,
                codestream::Part1ComponentDecodeRequest {
                    component_indices: &[0],
                    region: codestream::TileRegionRequest {
                        x: region.x,
                        y: region.y,
                        width: region.width,
                        height: region.height,
                    },
                    discard_levels: 0,
                    max_layers: Some(1),
                },
            )
            .is_err()
        );
    }

    fn set_siz_u32(input: &mut [u8], field_offset: usize, value: u32) {
        let siz = marker_offset(input, codestream::Marker::Siz, 0);
        input[siz + field_offset..siz + field_offset + 4].copy_from_slice(&value.to_be_bytes());
    }

    #[test]
    fn native_origin_partial_rejects_unaligned_geometry_and_prior_exclusions_across_routes() {
        let (fixture, _) = native_origin_multitile_fixture();
        let region = Region {
            x: 50,
            y: 32,
            width: 30,
            height: 25,
        };
        let options = PartialDecodeOptions {
            region: Some(region),
            ..PartialDecodeOptions::default()
        };
        let target_info = decode_partial_info(&fixture, &options).unwrap();

        let mut candidates = Vec::new();
        for (field, end_field, origin, end) in [
            (14, 6, 9, 140),
            (14, 6, 10, 141),
            (18, 10, 13, 112),
            (18, 10, 14, 113),
        ] {
            let mut candidate = fixture.clone();
            set_siz_u32(&mut candidate, field, origin);
            set_siz_u32(&mut candidate, end_field, end);
            candidates.push(candidate);
        }
        for (field, origin) in [(30, 5), (30, 6), (34, 5), (34, 6)] {
            let mut candidate = fixture.clone();
            set_siz_u32(&mut candidate, field, origin);
            candidates.push(candidate);
        }
        for (field, extent) in [(22, 62), (26, 46)] {
            let mut candidate = fixture.clone();
            set_siz_u32(&mut candidate, field, extent);
            candidates.push(candidate);
        }
        for (field, origin) in [(30, 12), (34, 16)] {
            let mut candidate = fixture.clone();
            set_siz_u32(&mut candidate, field, origin);
            candidates.push(candidate);
        }

        let siz = marker_offset(&fixture, codestream::Marker::Siz, 0);
        let cod = marker_offset(&fixture, codestream::Marker::Cod, 0);
        let mut signed = fixture.clone();
        signed[siz + 40] |= 0x80;
        candidates.push(signed);
        let mut subsampled = fixture.clone();
        subsampled[siz + 41] = 2;
        candidates.push(subsampled);
        let mut irreversible = fixture.clone();
        irreversible[cod + 13] = 0;
        candidates.push(irreversible);
        let mut two_layers = fixture.clone();
        two_layers[cod + 6..cod + 8].copy_from_slice(&2_u16.to_be_bytes());
        candidates.push(two_layers);
        candidates.push(insert_before_first_sot(
            fixture.clone(),
            &[0xff, 0x60, 0, 3, 0],
        ));
        candidates.push(insert_before_first_sot(
            fixture.clone(),
            &[0xff, 0x64, 0, 2],
        ));
        let mut bytes_after_eoc = fixture.clone();
        bytes_after_eoc.extend_from_slice(&[0, 1]);
        candidates.push(bytes_after_eoc);

        for candidate in candidates {
            if let Ok(parsed) = codestream::parse(&candidate) {
                assert!(
                    !codestream::is_supported_part1_native_multitile_partial_profile(
                        &candidate, &parsed,
                    )
                );
            }
            assert_native_partial_rejected_across_routes(&candidate, &options, &target_info);
            assert_native_origin_source_rejected(&candidate, region);
        }

        let jp2 = jp2_wrapped_native_multitile_fixture(&fixture);
        assert_native_partial_rejected_across_routes(&jp2, &options, &target_info);
        assert_native_origin_source_rejected(&jp2, region);
        for excluded in [
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..options.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(2),
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![1]),
                ..options.clone()
            },
        ] {
            assert_partial_rejected_without_mutation(&fixture, &excluded, &target_info);
        }
    }

    #[test]
    fn native_origin_partial_selected_corruption_is_atomic_and_unselected_is_ignored() {
        let (fixture, _) = native_origin_multitile_fixture();
        let tile_options = PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 2,
                tile_y: 2,
            }),
            ..PartialDecodeOptions::default()
        };
        let target_info = decode_partial_info(&fixture, &tile_options).unwrap();

        let mut bad_psot = fixture.clone();
        let selected_sot = marker_offset(&bad_psot, codestream::Marker::Sot, 8);
        bad_psot[selected_sot + 6..selected_sot + 10].copy_from_slice(&13_u32.to_be_bytes());
        assert_partial_rejected_without_mutation(&bad_psot, &tile_options, &target_info);

        let mut bad_selected_packet = fixture.clone();
        let selected_payload = codestream::parse(&bad_selected_packet)
            .unwrap()
            .tiles
            .iter()
            .find(|tile| tile.tile_index == 8)
            .and_then(|tile| tile.payload_offset)
            .unwrap();
        bad_selected_packet[selected_payload] = 0xff;
        assert_partial_rejected_without_mutation(&bad_selected_packet, &tile_options, &target_info);

        let expected = decode_partial(&fixture, &tile_options).unwrap();
        let mut bad_unselected_packet = fixture.clone();
        let unselected_payload = codestream::parse(&bad_unselected_packet)
            .unwrap()
            .tiles
            .iter()
            .find(|tile| tile.tile_index == 0)
            .and_then(|tile| tile.payload_offset)
            .unwrap();
        bad_unselected_packet[unselected_payload] = 0xff;
        assert_eq!(
            decode_partial(&bad_unselected_packet, &tile_options).unwrap(),
            expected
        );

        let selected_region = codestream::TileRegionRequest {
            x: 124,
            y: 88,
            width: 7,
            height: 11,
        };
        let source = codestream::source::SliceSource::new(&bad_unselected_packet);
        let source_prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: selected_region,
                discard_levels: 0,
                max_layers: Some(1),
            },
        )
        .unwrap();
        assert_eq!(source_prepared.codestream.selected_tile_count(), 1);
        assert_eq!(
            execute_prepared_u8(&source_prepared, None).0,
            planar_bytes(&expected)[0]
        );
    }

    #[test]
    fn native_multitile_partial_preflight_is_atomic_and_profile_is_narrow() {
        let (fixture, _) = native_multitile_fixture();
        let region = Region {
            x: 50,
            y: 38,
            width: 30,
            height: 25,
        };
        let options = PartialDecodeOptions {
            region: Some(region),
            ..PartialDecodeOptions::default()
        };
        let target_info = decode_partial_info(&fixture, &options).unwrap();

        let jp2 = jp2_wrapped_native_multitile_fixture(&fixture);
        assert_eq!(
            inspect(&jp2, &InspectOptions::default()).unwrap().format,
            InputFormat::Jp2
        );
        assert_native_partial_rejected_across_routes(&jp2, &options, &target_info);

        for excluded in [
            PartialDecodeOptions {
                region: Some(region),
                tile: Some(TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                }),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 1,
                }),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 130,
                    y: 98,
                    width: 2,
                    height: 2,
                }),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                region: None,
                tile: Some(TileSelection {
                    tile_x: 3,
                    tile_y: 0,
                }),
                ..PartialDecodeOptions::default()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(0),
                ..options.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(2),
                ..options.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![]),
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![0, 0]),
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![1]),
                ..options.clone()
            },
        ] {
            assert_partial_rejected_without_mutation(&fixture, &excluded, &target_info);
        }
        let interleaved_info = ImageInfo::new(
            region.width,
            region.height,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Interleaved,
        )
        .unwrap();
        let interleaved_stride = usize::try_from(region.width).unwrap() + 7;
        let mut interleaved =
            vec![0x9b; interleaved_stride * usize::try_from(region.height).unwrap()];
        {
            let mut target = ImageViewMut::Interleaved {
                info: &interleaved_info,
                samples: &mut interleaved,
                stride_bytes: interleaved_stride,
            };
            assert!(decode_partial_into(&fixture, &mut target, &options).is_err());
        }
        assert!(interleaved.iter().all(|sample| *sample == 0x9b));

        let tile_options = PartialDecodeOptions {
            tile: Some(TileSelection {
                tile_x: 2,
                tile_y: 2,
            }),
            ..PartialDecodeOptions::default()
        };
        let tile_info = decode_partial_info(&fixture, &tile_options).unwrap();
        for occurrence in [0_usize, 8] {
            let mut bad_psot = fixture.clone();
            let sot = marker_offset(&bad_psot, codestream::Marker::Sot, occurrence);
            bad_psot[sot + 6..sot + 10].copy_from_slice(&13_u32.to_be_bytes());
            assert_partial_rejected_without_mutation(&bad_psot, &tile_options, &tile_info);
        }
        let mut bad_selected_packet = fixture.clone();
        let payload = codestream::parse(&bad_selected_packet)
            .unwrap()
            .tiles
            .iter()
            .find(|tile| tile.tile_index == 8)
            .and_then(|tile| tile.payload_offset)
            .unwrap();
        bad_selected_packet[payload] = 0xff;
        assert_partial_rejected_without_mutation(&bad_selected_packet, &tile_options, &tile_info);

        let expected_selected_tile = decode_partial(&fixture, &tile_options).unwrap();
        let mut bad_unselected_packet = fixture.clone();
        let unselected_payload = codestream::parse(&bad_unselected_packet)
            .unwrap()
            .tiles
            .iter()
            .find(|tile| tile.tile_index == 0)
            .and_then(|tile| tile.payload_offset)
            .unwrap();
        bad_unselected_packet[unselected_payload] = 0xff;
        assert_eq!(
            decode_partial(&bad_unselected_packet, &tile_options).unwrap(),
            expected_selected_tile
        );

        let siz = marker_offset(&fixture, codestream::Marker::Siz, 0);
        let cod = marker_offset(&fixture, codestream::Marker::Cod, 0);
        let mut nearby = Vec::new();

        let lsiz = usize::from(u16::from_be_bytes(
            fixture[siz + 2..siz + 4].try_into().unwrap(),
        ));
        let siz_segment = fixture[siz..siz + 2 + lsiz].to_vec();
        let mut duplicate_main_siz = fixture.clone();
        duplicate_main_siz.splice(cod..cod, siz_segment.iter().copied());
        let parsed_duplicate_main_siz = codestream::parse(&duplicate_main_siz).unwrap();
        assert!(
            !codestream::is_supported_part1_native_multitile_partial_profile(
                &duplicate_main_siz,
                &parsed_duplicate_main_siz,
            )
        );
        nearby.push(duplicate_main_siz);
        let tile_header_siz = insert_first_tile_header_segment(fixture.clone(), &siz_segment);
        let parsed_tile_header_siz = codestream::parse(&tile_header_siz).unwrap();
        assert!(
            !codestream::is_supported_part1_native_multitile_partial_profile(
                &tile_header_siz,
                &parsed_tile_header_siz,
            )
        );
        nearby.push(tile_header_siz);
        let mut bytes_after_eoc = fixture.clone();
        bytes_after_eoc.extend_from_slice(&[0, 1]);
        let parsed_bytes_after_eoc = codestream::parse(&bytes_after_eoc).unwrap();
        assert!(
            !codestream::is_supported_part1_native_multitile_partial_profile(
                &bytes_after_eoc,
                &parsed_bytes_after_eoc,
            )
        );
        nearby.push(bytes_after_eoc);

        let mut nonzero_origin = fixture.clone();
        nonzero_origin[siz + 6..siz + 10].copy_from_slice(&132_u32.to_be_bytes());
        nonzero_origin[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        nonzero_origin[siz + 30..siz + 34].copy_from_slice(&1_u32.to_be_bytes());
        nearby.push(nonzero_origin);

        let mut signed = fixture.clone();
        signed[siz + 42] |= 0x80;
        nearby.push(signed);
        let mut high_precision = fixture.clone();
        high_precision[siz + 42] = 8;
        nearby.push(high_precision);
        let mut subsampled = fixture.clone();
        subsampled[siz + 43] = 2;
        nearby.push(subsampled);
        let mut mct = fixture.clone();
        mct[cod + 8] = 1;
        nearby.push(mct);
        let mut irreversible = fixture.clone();
        irreversible[cod + 13] = 0;
        nearby.push(irreversible);
        let mut two_layers = fixture.clone();
        two_layers[cod + 6..cod + 8].copy_from_slice(&2_u16.to_be_bytes());
        nearby.push(two_layers);
        let mut rlcp = fixture.clone();
        rlcp[cod + 5] = 1;
        nearby.push(rlcp);
        let mut sop = fixture.clone();
        sop[cod + 4] |= 0x02;
        nearby.push(sop);
        let mut fragmented = fixture.clone();
        let first_sot = marker_offset(&fragmented, codestream::Marker::Sot, 0);
        fragmented[first_sot + 11] = 2;
        nearby.push(fragmented);

        nearby.push(insert_main_coc(fixture.clone(), [2, 4, 4, 0, 1]));
        let qcd = marker_offset(&fixture, codestream::Marker::Qcd, 0);
        let lqcd = usize::from(u16::from_be_bytes(
            fixture[qcd + 2..qcd + 4].try_into().unwrap(),
        ));
        let qcd_payload = &fixture[qcd + 4..qcd + 2 + lqcd];
        let mut qcc = vec![0xff, 0x5d];
        qcc.extend_from_slice(&u16::try_from(qcd_payload.len() + 3).unwrap().to_be_bytes());
        qcc.push(0);
        qcc.extend_from_slice(qcd_payload);
        nearby.push(insert_before_first_sot(fixture.clone(), &qcc));
        nearby.push(insert_before_first_sot(
            fixture.clone(),
            &[0xff, 0x60, 0, 3, 0],
        ));
        nearby.push(insert_first_tile_header_segment(
            fixture.clone(),
            &[0xff, 0x53, 0, 9, 0, 0, 2, 4, 4, 0, 1],
        ));

        let rgb_samples = (0..131_u32 * 99)
            .flat_map(|sample| {
                let value = u8::try_from((sample * 17 + 3) % 251).unwrap();
                [value, value.wrapping_add(1), value.wrapping_add(2)]
            })
            .collect::<Vec<_>>();
        nearby.push(
            codestream::encode_rgb_u8_two_decomp_multitile(
                codestream::RgbU8Encode {
                    width: 131,
                    height: 99,
                    samples: &rgb_samples,
                    stride_bytes: 131 * 3,
                },
                codestream::TileSize {
                    width: 64,
                    height: 48,
                },
            )
            .unwrap(),
        );

        for candidate in nearby {
            if let Ok(parsed) = codestream::parse(&candidate) {
                assert!(
                    !codestream::is_supported_part1_native_multitile_partial_profile(
                        &candidate, &parsed,
                    )
                );
            }
            assert_native_partial_rejected_across_routes(&candidate, &options, &target_info);
        }
    }

    #[test]
    fn full_decode_into_rejects_subsampled_native_planes_without_mutation() {
        let (fixture, _) = subsampled_fixture(9, 5);
        let options = DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::Indices(vec![1]),
            target_layout: ComponentLayout::Planar,
            ..DecodeOptions::default()
        };
        let info = decode_shape(&fixture, &options)
            .unwrap()
            .image_info()
            .unwrap();
        assert_eq!((info.width, info.height, info.components), (9, 5, 1));
        let mut samples = vec![0x6d; 45];
        {
            let plane = PlaneMut::new(&mut samples, 9, 5, 9, info.sample_format).unwrap();
            let mut planes = [plane];
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_into(&fixture, &mut target, &options).is_err());
        }
        assert!(samples.iter().all(|sample| *sample == 0x6d));
    }

    #[test]
    fn jp2_default_image_planning_preserves_native_output_and_rejects_projection_atomically() {
        let (fixture, expected) = subsampled_fixture(9, 5);
        let native = decode_partial(&fixture, &PartialDecodeOptions::default()).unwrap();
        assert_eq!(planar_bytes(&native), expected.as_slice());

        let rendered = DecodeOptions::default();
        assert!(matches!(
            decode(&fixture, &rendered),
            Err(J2kError::Unsupported {
                feature: UnsupportedFeature::ComponentLayout,
                ..
            })
        ));

        let jp2 = wrap_rgb_jp2(&fixture, 9, 5);
        let metadata = inspect(&jp2, &InspectOptions::default()).unwrap();
        assert_eq!(metadata.format, InputFormat::Jp2);
        let codestream_bytes = primary_part1_codestream_bytes(&jp2, &metadata)
            .unwrap()
            .unwrap();
        let parsed = codestream::parse(codestream_bytes).unwrap();
        let selected = jp2_default_image_geometry(&parsed).unwrap();
        assert_eq!(selected.spacing(), 1);
        assert_eq!(
            (
                selected.bounds().x0(),
                selected.bounds().y0(),
                selected.width(),
                selected.height(),
            ),
            (0, 0, 9, 5)
        );
        assert!(matches!(
            decode(&jp2, &rendered),
            Err(J2kError::Unsupported {
                feature: UnsupportedFeature::ComponentLayout,
                ..
            })
        ));

        let info = ImageInfo::new(
            9,
            5,
            3,
            SampleFormat::U8,
            ColorModel::Rgb,
            ComponentLayout::Planar,
        )
        .unwrap();
        let mut buffers = [vec![0x6d; 45], vec![0x6d; 45], vec![0x6d; 45]];
        {
            let mut planes = buffers
                .iter_mut()
                .map(|samples| PlaneMut::new(samples, 9, 5, 9, SampleFormat::U8).unwrap())
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(matches!(
                decode_into(&jp2, &mut target, &rendered),
                Err(J2kError::Unsupported {
                    feature: UnsupportedFeature::ComponentLayout,
                    ..
                })
            ));
        }
        assert!(buffers.iter().flatten().all(|sample| *sample == 0x6d));
    }

    #[test]
    fn raw_part1_does_not_select_jp2_default_image_geometry_or_change_error_precedence() {
        let sample = [0x2a];
        let mut fixture = codestream::encode_planar_u8_subsampled_no_decomp_test_fixture(
            1,
            1,
            &[codestream::SubsampledU8TestComponent {
                horizontal_separation: 2,
                vertical_separation: 2,
                samples: &sample,
            }],
        )
        .unwrap();
        let siz = marker_offset(&fixture, codestream::Marker::Siz, 0);
        fixture[siz + 6..siz + 10].copy_from_slice(&2_u32.to_be_bytes());
        fixture[siz + 10..siz + 14].copy_from_slice(&2_u32.to_be_bytes());
        fixture[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        fixture[siz + 18..siz + 22].copy_from_slice(&1_u32.to_be_bytes());
        fixture[siz + 22..siz + 26].copy_from_slice(&2_u32.to_be_bytes());
        fixture[siz + 26..siz + 30].copy_from_slice(&2_u32.to_be_bytes());

        let parsed = codestream::parse(&fixture).unwrap();
        assert!(
            codestream::geometry::CommonGridPlan::new(
                parsed.siz.image_reference_rect().unwrap(),
                &parsed.siz.components,
            )
            .is_err()
        );
        let raw_error = decode(&fixture, &DecodeOptions::default()).unwrap_err();
        assert!(
            matches!(
                raw_error,
                J2kError::Unsupported {
                    feature: UnsupportedFeature::ComponentLayout,
                    ref detail,
                } if detail == "native subsampled component decode currently requires zero image and tile origins"
            ),
            "{raw_error:?}"
        );

        let jp2 = wrap_grayscale_jp2(&fixture, 1, 1);
        assert!(matches!(
            decode(&jp2, &DecodeOptions::default()),
            Err(J2kError::InvalidInput {
                offset: None,
                message,
            }) if message == "codestream size overflowed parser limits"
        ));
    }

    #[test]
    fn subsampled_zero_discard_matches_full_owned_caller_and_info() {
        let (fixture, _) = subsampled_fixture(9, 5);
        let full_options = PartialDecodeOptions::default();
        let zero_discard_options = PartialDecodeOptions {
            resolution: ResolutionLevel::Reduced { discard_levels: 0 },
            ..PartialDecodeOptions::default()
        };
        let full_info = decode_partial_info(&fixture, &full_options).unwrap();
        let zero_discard_info = decode_partial_info(&fixture, &zero_discard_options).unwrap();
        assert_eq!(zero_discard_info, full_info);
        let full_components = decode_partial_component_info(&fixture, &full_options).unwrap();
        let zero_discard_components =
            decode_partial_component_info(&fixture, &zero_discard_options).unwrap();
        assert_eq!(zero_discard_components, full_components);

        let full = decode_partial(&fixture, &full_options).unwrap();
        let zero_discard = decode_partial(&fixture, &zero_discard_options).unwrap();
        assert_eq!(zero_discard, full);

        let mut caller_buffers = zero_discard_components
            .iter()
            .map(|component| {
                vec![0xa5; usize::try_from(component.width * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut caller_planes = caller_buffers
                .iter_mut()
                .zip(&zero_discard_components)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &zero_discard_info,
                planes: &mut caller_planes,
            };
            decode_partial_into(&fixture, &mut target, &zero_discard_options).unwrap();
        }
        assert_eq!(caller_buffers, planar_bytes(&full));
    }

    #[test]
    fn reduced_subsampled_full_request_preserves_per_component_geometry() {
        let fixture = reduced_subsampled_fixture(129, 67);
        let options = PartialDecodeOptions {
            resolution: ResolutionLevel::Reduced { discard_levels: 1 },
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&fixture, &options).unwrap();
        assert_eq!((info.width, info.height, info.components), (65, 34, 3));
        let descriptors = decode_partial_component_info(&fixture, &options).unwrap();
        assert_eq!(
            descriptors
                .iter()
                .map(|component| (
                    component.source_component,
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                    component.horizontal_separation,
                    component.vertical_separation,
                ))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), 0, 0, 65, 34, 1, 1),
                (Some(1), 0, 0, 33, 34, 2, 1),
                (Some(2), 0, 0, 33, 17, 2, 2),
            ]
        );
        let decoded = decode_partial(&fixture, &options).unwrap();
        assert_eq!(decoded.info, info);
        assert_eq!(decoded.component_info, descriptors);
        assert_eq!(planar_bytes(&decoded)[0].len(), 65 * 34);
        assert_eq!(planar_bytes(&decoded)[1].len(), 33 * 34);
        assert_eq!(planar_bytes(&decoded)[2].len(), 33 * 17);
    }

    #[test]
    fn reduced_subsampled_region_crop_stitch_routes_and_work_are_consistent() {
        let fixture = reduced_subsampled_fixture(257, 131);
        let reduced = ResolutionLevel::Reduced { discard_levels: 1 };
        let full_options = PartialDecodeOptions {
            resolution: reduced,
            ..PartialDecodeOptions::default()
        };
        let region = Region {
            x: 64,
            y: 32,
            width: 128,
            height: 64,
        };
        let region_options = PartialDecodeOptions {
            region: Some(region),
            resolution: reduced,
            ..PartialDecodeOptions::default()
        };
        let full = decode_partial(&fixture, &full_options).unwrap();
        let strict = decode_partial(&fixture, &region_options).unwrap();
        let work_plan = plan_partial_decode_work(&fixture, &region_options).unwrap();
        assert!(!work_plan.full_image_full_resolution_fallback);
        assert_eq!(work_plan.selected_resolution.discard_levels, 1);
        assert_eq!(
            (
                work_plan.selected_resolution.width,
                work_plan.selected_resolution.height
            ),
            (64, 32)
        );
        assert_eq!((strict.info.width, strict.info.height), (64, 32));
        assert_eq!(
            strict
                .component_info
                .iter()
                .map(|component| (
                    component.source_component,
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                    component.horizontal_separation,
                    component.vertical_separation,
                ))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), 32, 16, 64, 32, 1, 1),
                (Some(1), 16, 16, 32, 32, 2, 1),
                (Some(2), 16, 8, 32, 16, 2, 2),
            ]
        );
        for ((strict_plane, strict_info), (full_plane, full_info)) in planar_bytes(&strict)
            .iter()
            .zip(&strict.component_info)
            .zip(planar_bytes(&full).iter().zip(&full.component_info))
        {
            let x = usize::try_from(strict_info.x_origin - full_info.x_origin).unwrap();
            let y = usize::try_from(strict_info.y_origin - full_info.y_origin).unwrap();
            let width = usize::try_from(strict_info.width).unwrap();
            let full_width = usize::try_from(full_info.width).unwrap();
            let expected = (0..usize::try_from(strict_info.height).unwrap())
                .flat_map(|row| {
                    let start = (y + row) * full_width + x;
                    full_plane[start..start + width].iter().copied()
                })
                .collect::<Vec<_>>();
            assert_eq!(strict_plane, &expected);
        }

        let left = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 128,
                    height: 131,
                }),
                resolution: reduced,
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let right = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 128,
                    y: 0,
                    width: 129,
                    height: 131,
                }),
                resolution: reduced,
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        for component in 0..3 {
            let full_info = &full.component_info[component];
            let left_info = &left.component_info[component];
            let right_info = &right.component_info[component];
            assert_eq!(left_info.x_origin + left_info.width, right_info.x_origin);
            assert_eq!(left_info.width + right_info.width, full_info.width);
            let left_width = usize::try_from(left_info.width).unwrap();
            let right_width = usize::try_from(right_info.width).unwrap();
            let stitched = (0..usize::try_from(full_info.height).unwrap())
                .flat_map(|row| {
                    planar_bytes(&left)[component][row * left_width..(row + 1) * left_width]
                        .iter()
                        .chain(
                            &planar_bytes(&right)[component]
                                [row * right_width..(row + 1) * right_width],
                        )
                        .copied()
                })
                .collect::<Vec<_>>();
            assert_eq!(stitched, planar_bytes(&full)[component]);
        }

        let descriptors = strict.component_info.clone();
        let mut padded = descriptors
            .iter()
            .map(|component| {
                vec![0xa5; usize::try_from((component.width + 7) * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut planes = padded
                .iter_mut()
                .zip(&descriptors)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width + 7).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &strict.info,
                planes: &mut planes,
            };
            decode_partial_into(&fixture, &mut target, &region_options).unwrap();
        }
        for ((buffer, descriptor), expected) in
            padded.iter().zip(&descriptors).zip(planar_bytes(&strict))
        {
            let width = usize::try_from(descriptor.width).unwrap();
            let stride = width + 7;
            let actual = buffer
                .chunks_exact(stride)
                .flat_map(|row| row[..width].iter().copied())
                .collect::<Vec<_>>();
            assert_eq!(&actual, expected);
            assert!(
                buffer
                    .chunks_exact(stride)
                    .all(|row| row[width..].iter().all(|sample| *sample == 0xa5))
            );
        }

        let execute = |prepared: &PreparedPart1Decode<'_>| {
            let mut buffers = prepared
                .component_info()
                .iter()
                .map(|component| {
                    vec![0_u8; usize::try_from(component.width * component.height).unwrap()]
                })
                .collect::<Vec<_>>();
            let timings = {
                let mut planes = buffers
                    .iter_mut()
                    .zip(prepared.component_info())
                    .map(|(samples, component)| {
                        PlaneMut::new(
                            samples,
                            component.width,
                            component.height,
                            usize::try_from(component.width).unwrap(),
                            component.sample_format,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();
                let mut target = ImageViewMut::Planar {
                    info: prepared.info(),
                    planes: &mut planes,
                };
                execute_prepared_part1_decode_into_with_workspace(
                    prepared,
                    &mut target,
                    &mut Part1DecodeWorkspace::new(),
                    codestream::PreparedPart1ExecutionOptions {
                        instrumentation: codestream::DecodeInstrumentation::WorkCounters,
                        parallelism: codestream::DecodeExecutionParallelism::Serial,
                        ..codestream::PreparedPart1ExecutionOptions::default()
                    },
                )
                .unwrap()
            };
            (buffers, timings)
        };
        let full_prepared = prepare_part1_decode(&fixture, &full_options).unwrap();
        let strict_prepared = prepare_part1_decode(&fixture, &region_options).unwrap();
        let selected_options = PartialDecodeOptions {
            components: ComponentSelection::Indices(vec![2]),
            ..region_options.clone()
        };
        let selected_prepared = prepare_part1_decode(&fixture, &selected_options).unwrap();
        let (full_buffers, full_work) = execute(&full_prepared);
        let (strict_buffers, strict_work) = execute(&strict_prepared);
        let (selected_buffers, selected_work) = execute(&selected_prepared);
        assert_eq!(full_buffers, planar_bytes(&full));
        assert_eq!(strict_buffers, planar_bytes(&strict));
        assert_eq!(selected_buffers, planar_bytes(&strict)[2..3]);
        assert!(strict_work.executed_code_blocks < full_work.executed_code_blocks);
        assert!(strict_work.output_samples < full_work.output_samples);
        assert!(selected_work.executed_code_blocks < strict_work.executed_code_blocks);
        assert!(selected_work.output_samples < strict_work.output_samples);
        assert!(full_work.packet_body_bytes_skipped > 0);
        assert!(strict_work.packet_body_bytes_skipped > full_work.packet_body_bytes_skipped);
        assert!(selected_work.packet_body_bytes_skipped > strict_work.packet_body_bytes_skipped);
        assert_eq!(full_work.full_output_allocation_bytes, 0);
        assert_eq!(strict_work.full_output_allocation_bytes, 0);
        assert_eq!(selected_work.full_output_allocation_bytes, 0);

        let source = codestream::source::SliceSource::new(&fixture);
        let components = [0_u16, 1, 2];
        let source_prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &components,
                region: codestream::TileRegionRequest {
                    x: region.x,
                    y: region.y,
                    width: region.width,
                    height: region.height,
                },
                discard_levels: 1,
                max_layers: Some(1),
            },
        )
        .unwrap();
        let (source_buffers, source_work) = execute(&source_prepared);
        assert_eq!(source_buffers, planar_bytes(&strict));
        assert_eq!(source_work.full_output_allocation_bytes, 0);

        for max_quality_layers in [Some(1), Some(2)] {
            let limited = decode_partial(
                &fixture,
                &PartialDecodeOptions {
                    max_quality_layers,
                    ..region_options.clone()
                },
            )
            .unwrap();
            assert_eq!(limited, strict);
        }
    }

    #[test]
    fn two_level_reduced_subsampled_geometry_routes_and_selective_work_are_consistent() {
        let fixture = two_level_reduced_subsampled_fixture(257, 131);
        let reduced = ResolutionLevel::Reduced { discard_levels: 2 };
        let full_options = PartialDecodeOptions {
            resolution: reduced,
            ..PartialDecodeOptions::default()
        };
        let full = decode_partial(&fixture, &full_options).unwrap();
        assert_eq!((full.info.width, full.info.height), (65, 33));
        assert_eq!(
            full.component_info
                .iter()
                .map(|component| (
                    component.source_component,
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                    component.horizontal_separation,
                    component.vertical_separation,
                ))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), 0, 0, 65, 33, 1, 1),
                (Some(1), 0, 0, 33, 33, 2, 1),
                (Some(2), 0, 0, 33, 17, 2, 2),
            ]
        );

        let region = Region {
            x: 64,
            y: 32,
            width: 128,
            height: 64,
        };
        let region_options = PartialDecodeOptions {
            region: Some(region),
            resolution: reduced,
            ..PartialDecodeOptions::default()
        };
        let strict = decode_partial(&fixture, &region_options).unwrap();
        assert_eq!((strict.info.width, strict.info.height), (32, 16));
        assert_eq!(
            strict
                .component_info
                .iter()
                .map(|component| (
                    component.source_component,
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                    component.horizontal_separation,
                    component.vertical_separation,
                ))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), 16, 8, 32, 16, 1, 1),
                (Some(1), 8, 8, 16, 16, 2, 1),
                (Some(2), 8, 4, 16, 8, 2, 2),
            ]
        );
        let work_plan = plan_partial_decode_work(&fixture, &region_options).unwrap();
        assert!(!work_plan.full_image_full_resolution_fallback);
        assert_eq!(work_plan.selected_resolution.discard_levels, 2);
        assert_eq!(
            (
                work_plan.selected_resolution.width,
                work_plan.selected_resolution.height,
            ),
            (32, 16)
        );

        let assert_matches_full_crop = |cropped: &Image| {
            for ((cropped_plane, cropped_info), (full_plane, full_info)) in planar_bytes(cropped)
                .iter()
                .zip(&cropped.component_info)
                .zip(planar_bytes(&full).iter().zip(&full.component_info))
            {
                let x = usize::try_from(cropped_info.x_origin - full_info.x_origin).unwrap();
                let y = usize::try_from(cropped_info.y_origin - full_info.y_origin).unwrap();
                let width = usize::try_from(cropped_info.width).unwrap();
                let full_width = usize::try_from(full_info.width).unwrap();
                let expected = (0..usize::try_from(cropped_info.height).unwrap())
                    .flat_map(|row| {
                        let start = (y + row) * full_width + x;
                        full_plane[start..start + width].iter().copied()
                    })
                    .collect::<Vec<_>>();
                assert_eq!(cropped_plane, &expected);
            }
        };
        assert_matches_full_crop(&strict);

        let odd = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 1,
                    y: 1,
                    width: 30,
                    height: 30,
                }),
                resolution: reduced,
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!((odd.info.width, odd.info.height), (7, 7));
        assert_eq!(
            odd.component_info
                .iter()
                .map(|component| (
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                ))
                .collect::<Vec<_>>(),
            vec![(1, 1, 7, 7), (1, 1, 3, 7), (1, 1, 3, 3)]
        );
        assert_matches_full_crop(&odd);

        let left = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 128,
                    height: 131,
                }),
                resolution: reduced,
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let right = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 128,
                    y: 0,
                    width: 129,
                    height: 131,
                }),
                resolution: reduced,
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        for component in 0..3 {
            let full_info = &full.component_info[component];
            let left_info = &left.component_info[component];
            let right_info = &right.component_info[component];
            assert_eq!(left_info.x_origin + left_info.width, right_info.x_origin);
            assert_eq!(left_info.width + right_info.width, full_info.width);
            let left_width = usize::try_from(left_info.width).unwrap();
            let right_width = usize::try_from(right_info.width).unwrap();
            let stitched = (0..usize::try_from(full_info.height).unwrap())
                .flat_map(|row| {
                    planar_bytes(&left)[component][row * left_width..(row + 1) * left_width]
                        .iter()
                        .chain(
                            &planar_bytes(&right)[component]
                                [row * right_width..(row + 1) * right_width],
                        )
                        .copied()
                })
                .collect::<Vec<_>>();
            assert_eq!(stitched, planar_bytes(&full)[component]);
        }

        let descriptors = strict.component_info.clone();
        let mut padded = descriptors
            .iter()
            .map(|component| {
                vec![0xa5; usize::try_from((component.width + 7) * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut planes = padded
                .iter_mut()
                .zip(&descriptors)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width + 7).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &strict.info,
                planes: &mut planes,
            };
            decode_partial_into(&fixture, &mut target, &region_options).unwrap();
        }
        for ((buffer, descriptor), expected) in
            padded.iter().zip(&descriptors).zip(planar_bytes(&strict))
        {
            let width = usize::try_from(descriptor.width).unwrap();
            let stride = width + 7;
            let actual = buffer
                .chunks_exact(stride)
                .flat_map(|row| row[..width].iter().copied())
                .collect::<Vec<_>>();
            assert_eq!(&actual, expected);
            assert!(
                buffer
                    .chunks_exact(stride)
                    .all(|row| row[width..].iter().all(|sample| *sample == 0xa5))
            );
        }

        let execute = |prepared: &PreparedPart1Decode<'_>| {
            let mut buffers = prepared
                .component_info()
                .iter()
                .map(|component| {
                    vec![0_u8; usize::try_from(component.width * component.height).unwrap()]
                })
                .collect::<Vec<_>>();
            let work = {
                let mut planes = buffers
                    .iter_mut()
                    .zip(prepared.component_info())
                    .map(|(samples, component)| {
                        PlaneMut::new(
                            samples,
                            component.width,
                            component.height,
                            usize::try_from(component.width).unwrap(),
                            component.sample_format,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();
                let mut target = ImageViewMut::Planar {
                    info: prepared.info(),
                    planes: &mut planes,
                };
                execute_prepared_part1_decode_into_with_workspace(
                    prepared,
                    &mut target,
                    &mut Part1DecodeWorkspace::new(),
                    codestream::PreparedPart1ExecutionOptions {
                        instrumentation: codestream::DecodeInstrumentation::WorkCounters,
                        parallelism: codestream::DecodeExecutionParallelism::Serial,
                        ..codestream::PreparedPart1ExecutionOptions::default()
                    },
                )
                .unwrap()
            };
            (buffers, work)
        };
        let full_prepared = prepare_part1_decode(&fixture, &full_options).unwrap();
        let strict_prepared = prepare_part1_decode(&fixture, &region_options).unwrap();
        let selected_options = PartialDecodeOptions {
            components: ComponentSelection::Indices(vec![2]),
            ..region_options.clone()
        };
        let selected_prepared = prepare_part1_decode(&fixture, &selected_options).unwrap();
        let (full_buffers, full_work) = execute(&full_prepared);
        let (strict_buffers, strict_work) = execute(&strict_prepared);
        let (selected_buffers, selected_work) = execute(&selected_prepared);
        assert_eq!(full_buffers, planar_bytes(&full));
        assert_eq!(strict_buffers, planar_bytes(&strict));
        assert_eq!(selected_buffers, planar_bytes(&strict)[2..3]);
        assert!(strict_work.executed_code_blocks < full_work.executed_code_blocks);
        assert!(strict_work.output_samples < full_work.output_samples);
        assert!(selected_work.executed_code_blocks < strict_work.executed_code_blocks);
        assert!(selected_work.output_samples < strict_work.output_samples);
        assert!(full_work.packet_body_bytes_skipped > 0);
        assert!(strict_work.packet_body_bytes_skipped > 0);
        assert!(selected_work.packet_body_bytes_skipped > 0);
        assert_eq!(full_work.full_output_allocation_bytes, 0);
        assert_eq!(strict_work.full_output_allocation_bytes, 0);
        assert_eq!(selected_work.full_output_allocation_bytes, 0);

        let source = codestream::source::SliceSource::new(&fixture);
        let components = [0_u16, 1, 2];
        let source_prepared = prepare_part1_decode_from_source(
            &source,
            codestream::Part1ComponentDecodeRequest {
                component_indices: &components,
                region: codestream::TileRegionRequest {
                    x: region.x,
                    y: region.y,
                    width: region.width,
                    height: region.height,
                },
                discard_levels: 2,
                max_layers: Some(1),
            },
        )
        .unwrap();
        let (source_buffers, source_work) = execute(&source_prepared);
        assert_eq!(source_buffers, planar_bytes(&strict));
        assert_eq!(source_work.full_output_allocation_bytes, 0);

        for max_quality_layers in [Some(1), Some(2)] {
            let limited = decode_partial(
                &fixture,
                &PartialDecodeOptions {
                    max_quality_layers,
                    ..region_options.clone()
                },
            )
            .unwrap();
            assert_eq!(limited, strict);
        }
    }

    #[test]
    fn two_level_reduced_subsampled_exclusions_fail_before_caller_mutation() {
        let fixture = two_level_reduced_subsampled_fixture(129, 67);
        let options = PartialDecodeOptions {
            region: Some(Region {
                x: 32,
                y: 16,
                width: 64,
                height: 32,
            }),
            resolution: ResolutionLevel::Reduced { discard_levels: 2 },
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&fixture, &options).unwrap();
        let descriptors = decode_partial_component_info(&fixture, &options).unwrap();
        let assert_rejected_without_mutation =
            |label: &str, input: &[u8], rejected_options: &PartialDecodeOptions| {
                let mut buffers = descriptors
                    .iter()
                    .map(|component| {
                        vec![0x6d; usize::try_from(component.width * component.height).unwrap()]
                    })
                    .collect::<Vec<_>>();
                {
                    let mut planes = buffers
                        .iter_mut()
                        .zip(&descriptors)
                        .map(|(samples, component)| {
                            PlaneMut::new(
                                samples,
                                component.width,
                                component.height,
                                usize::try_from(component.width).unwrap(),
                                component.sample_format,
                            )
                            .unwrap()
                        })
                        .collect::<Vec<_>>();
                    let mut target = ImageViewMut::Planar {
                        info: &info,
                        planes: &mut planes,
                    };
                    assert!(
                        decode_partial_into(input, &mut target, rejected_options).is_err(),
                        "{label} was unexpectedly admitted"
                    );
                }
                assert!(buffers.iter().flatten().all(|sample| *sample == 0x6d));
            };

        let mut malformed = fixture.clone();
        let sot = malformed
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x90])
            .unwrap();
        let psot = u32::from_be_bytes(malformed[sot + 6..sot + 10].try_into().unwrap());
        malformed[sot + 6..sot + 10].copy_from_slice(&(psot + 32).to_be_bytes());
        assert_rejected_without_mutation("malformed Psot", &malformed, &options);

        let one_decomposition = reduced_subsampled_fixture(129, 67);
        assert_rejected_without_mutation("one decomposition", &one_decomposition, &options);
        let selected_component = [0_u16];
        let source_request = |discard_levels| codestream::Part1ComponentDecodeRequest {
            component_indices: &selected_component,
            region: codestream::TileRegionRequest {
                x: 32,
                y: 16,
                width: 64,
                height: 32,
            },
            discard_levels,
            max_layers: None,
        };
        assert!(
            prepare_part1_decode_from_source(
                &codestream::source::SliceSource::new(&one_decomposition),
                source_request(2),
            )
            .is_err()
        );
        assert!(
            prepare_part1_decode_from_source(
                &codestream::source::SliceSource::new(&fixture),
                source_request(3),
            )
            .is_err()
        );

        let interleaved_info = ImageInfo::new(
            info.width,
            info.height,
            info.components,
            info.sample_format,
            info.color_model,
            ComponentLayout::Interleaved,
        )
        .unwrap();
        let interleaved_stride = usize::try_from(info.width * u32::from(info.components)).unwrap();
        let mut interleaved_samples =
            vec![0x6d; interleaved_stride * usize::try_from(info.height).unwrap()];
        {
            let mut target = ImageViewMut::Interleaved {
                info: &interleaved_info,
                samples: &mut interleaved_samples,
                stride_bytes: interleaved_stride,
            };
            assert!(decode_partial_into(&fixture, &mut target, &options).is_err());
        }
        assert!(interleaved_samples.iter().all(|sample| *sample == 0x6d));

        for (label, excluded) in [
            (
                "discard three",
                PartialDecodeOptions {
                    resolution: ResolutionLevel::Reduced { discard_levels: 3 },
                    ..options.clone()
                },
            ),
            (
                "tile request",
                PartialDecodeOptions {
                    tile: Some(TileSelection {
                        tile_x: 0,
                        tile_y: 0,
                    }),
                    region: None,
                    ..options.clone()
                },
            ),
            (
                "zero layers",
                PartialDecodeOptions {
                    max_quality_layers: Some(0),
                    ..options.clone()
                },
            ),
            (
                "empty reduced output",
                PartialDecodeOptions {
                    region: Some(Region {
                        x: 1,
                        y: 0,
                        width: 1,
                        height: 1,
                    }),
                    components: ComponentSelection::Indices(vec![0]),
                    ..options.clone()
                },
            ),
            (
                "out-of-bounds region",
                PartialDecodeOptions {
                    region: Some(Region {
                        x: 128,
                        y: 0,
                        width: 2,
                        height: 1,
                    }),
                    ..options.clone()
                },
            ),
            (
                "empty selection",
                PartialDecodeOptions {
                    components: ComponentSelection::Indices(vec![]),
                    ..options.clone()
                },
            ),
            (
                "out-of-range selection",
                PartialDecodeOptions {
                    components: ComponentSelection::Indices(vec![3]),
                    ..options.clone()
                },
            ),
            (
                "duplicate selection",
                PartialDecodeOptions {
                    components: ComponentSelection::Indices(vec![0, 0]),
                    ..options.clone()
                },
            ),
        ] {
            assert_rejected_without_mutation(label, &fixture, &excluded);
        }

        let cod = fixture
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x52])
            .unwrap();
        let siz = fixture
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x51])
            .unwrap();
        let mut mutations = Vec::new();
        let mut irreversible = fixture.clone();
        irreversible[cod + 13] = 0;
        mutations.push(("9/7", irreversible));
        let mut mct = fixture.clone();
        mct[cod + 8] = 1;
        mutations.push(("MCT", mct));
        let mut precinct = fixture.clone();
        precinct[cod + 4] = 1;
        mutations.push(("explicit precinct", precinct));
        let mut image_origin = fixture.clone();
        image_origin[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        mutations.push(("image origin", image_origin));
        let mut tile_origin = fixture.clone();
        tile_origin[siz + 30..siz + 34].copy_from_slice(&1_u32.to_be_bytes());
        mutations.push(("tile origin", tile_origin));
        let mut multiple_tiles = fixture.clone();
        multiple_tiles[siz + 22..siz + 26].copy_from_slice(&64_u32.to_be_bytes());
        mutations.push(("multiple tiles", multiple_tiles));
        for (label, mutation) in mutations {
            assert_rejected_without_mutation(label, &mutation, &options);
            assert!(
                prepare_part1_decode_from_source(
                    &codestream::source::SliceSource::new(&mutation),
                    source_request(2),
                )
                .is_err(),
                "source-backed {label} mutation was unexpectedly admitted"
            );
        }
    }

    #[test]
    fn reduced_subsampled_rejections_and_malformed_preflight_fail_closed() {
        let fixture = reduced_subsampled_fixture(129, 67);
        let options = PartialDecodeOptions {
            region: Some(Region {
                x: 32,
                y: 16,
                width: 64,
                height: 32,
            }),
            resolution: ResolutionLevel::Reduced { discard_levels: 1 },
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&fixture, &options).unwrap();
        let descriptors = decode_partial_component_info(&fixture, &options).unwrap();

        let mut malformed = fixture.clone();
        let sot = malformed
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x90])
            .unwrap();
        let psot = u32::from_be_bytes(malformed[sot + 6..sot + 10].try_into().unwrap());
        malformed[sot + 6..sot + 10].copy_from_slice(&(psot + 32).to_be_bytes());
        let mut buffers = descriptors
            .iter()
            .map(|component| {
                vec![0x6d; usize::try_from(component.width * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut planes = buffers
                .iter_mut()
                .zip(&descriptors)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_partial_into(&malformed, &mut target, &options).is_err());
        }
        assert!(buffers.iter().flatten().all(|sample| *sample == 0x6d));

        for excluded in [
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 2 },
                ..options.clone()
            },
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..options.clone()
            },
            PartialDecodeOptions {
                tile: Some(TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                }),
                region: None,
                ..options.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(0),
                ..options.clone()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 1,
                    y: 0,
                    width: 1,
                    height: 1,
                }),
                components: ComponentSelection::Indices(vec![0]),
                ..options.clone()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    x: 128,
                    y: 0,
                    width: 2,
                    height: 1,
                }),
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![]),
                ..options.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::Indices(vec![3]),
                ..options.clone()
            },
        ] {
            assert!(decode_partial(&fixture, &excluded).is_err());
        }

        let cod = fixture
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x52])
            .unwrap();
        let siz = fixture
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x51])
            .unwrap();
        let mut mutations = Vec::new();
        let mut irreversible = fixture.clone();
        irreversible[cod + 13] = 0;
        mutations.push(irreversible);
        let mut mct = fixture.clone();
        mct[cod + 8] = 1;
        mutations.push(mct);
        let mut precinct = fixture.clone();
        precinct[cod + 4] = 1;
        mutations.push(precinct);
        let mut non_zero_origin = fixture.clone();
        non_zero_origin[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        mutations.push(non_zero_origin);
        let mut second_tile = fixture.clone();
        second_tile[siz + 22..siz + 26].copy_from_slice(&64_u32.to_be_bytes());
        mutations.push(second_tile);
        for mutation in mutations {
            assert!(decode_partial(&mutation, &options).is_err());
        }
    }

    #[test]
    fn source_backed_discard_classifies_complete_sampling_before_selection() {
        let width = 129_u32;
        let height = 67_u32;
        let sampling = [(1_u8, 1_u8), (2, 2)];
        let planes = sampling
            .iter()
            .enumerate()
            .map(|(component, (horizontal, vertical))| {
                let native_width = width.div_ceil(u32::from(*horizontal));
                let native_height = height.div_ceil(u32::from(*vertical));
                (0..native_width * native_height)
                    .map(|sample| ((sample * (19 + component as u32 * 12) + 23) % 251) as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let components = planes
            .iter()
            .zip(sampling)
            .map(|(samples, (horizontal_separation, vertical_separation))| {
                codestream::SubsampledU8TestComponent {
                    horizontal_separation,
                    vertical_separation,
                    samples,
                }
            })
            .collect::<Vec<_>>();
        let mixed = codestream::encode_planar_u8_subsampled_two_decomp_test_fixture(
            width,
            height,
            &components,
        )
        .unwrap();
        let mixed_source = codestream::source::SliceSource::new(&mixed);
        let selected_unit_component = [0_u16];
        let request = codestream::Part1ComponentDecodeRequest {
            component_indices: &selected_unit_component,
            region: codestream::TileRegionRequest {
                x: 0,
                y: 0,
                width,
                height,
            },
            discard_levels: 2,
            max_layers: None,
        };
        let mixed_prepared = prepare_part1_decode_from_source(&mixed_source, request).unwrap();
        assert_eq!(
            (
                mixed_prepared.component_info()[0].width,
                mixed_prepared.component_info()[0].height,
                mixed_prepared.component_info()[0].horizontal_separation,
                mixed_prepared.component_info()[0].vertical_separation,
            ),
            (33, 17, 1, 1)
        );

        let unit_samples = (0..width * height)
            .map(|sample| ((sample * 29 + 11) % 251) as u8)
            .collect::<Vec<_>>();
        let unit = codestream::encode_grayscale_u8_two_decomp(codestream::GrayscaleU8Encode {
            width,
            height,
            samples: &unit_samples,
            stride_bytes: usize::try_from(width).unwrap(),
        })
        .unwrap();
        let unit_source = codestream::source::SliceSource::new(&unit);
        let prepared = prepare_part1_decode_from_source(&unit_source, request).unwrap();
        assert_eq!((prepared.info().width, prepared.info().height), (33, 17));
        assert_eq!(prepared.component_info().len(), 1);
        assert_eq!(
            (
                prepared.component_info()[0].width,
                prepared.component_info()[0].height,
                prepared.component_info()[0].horizontal_separation,
                prepared.component_info()[0].vertical_separation,
            ),
            (33, 17, 1, 1)
        );
    }

    #[test]
    fn subsampled_partial_decode_preserves_native_geometry_bytes_and_selective_work() {
        let (fixture, full_expected) = subsampled_fixture(129, 65);
        let options = PartialDecodeOptions {
            region: Some(Region {
                x: 1,
                y: 1,
                width: 31,
                height: 31,
            }),
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&fixture, &options).unwrap();
        assert_eq!((info.width, info.height, info.components), (31, 31, 3));
        let descriptors = decode_partial_component_info(&fixture, &options).unwrap();
        assert_eq!(
            descriptors
                .iter()
                .map(|component| (
                    component.source_component,
                    component.x_origin,
                    component.y_origin,
                    component.width,
                    component.height,
                    component.horizontal_separation,
                    component.vertical_separation,
                ))
                .collect::<Vec<_>>(),
            vec![
                (Some(0), 1, 1, 31, 31, 1, 1),
                (Some(1), 1, 1, 15, 31, 2, 1),
                (Some(2), 1, 1, 15, 15, 2, 2),
            ]
        );

        let owned = decode_partial(&fixture, &options).unwrap();
        assert_eq!(owned.info, info);
        assert_eq!(owned.component_info, descriptors);
        let owned_planes = planar_bytes(&owned);
        for (plane, descriptor) in owned_planes.iter().zip(&descriptors) {
            let source = &full_expected[usize::from(descriptor.source_component.unwrap())];
            let full_width = 129_u32.div_ceil(u32::from(descriptor.horizontal_separation));
            let expected = (descriptor.y_origin..descriptor.y_origin + descriptor.height)
                .flat_map(|y| {
                    let start = usize::try_from(y * full_width + descriptor.x_origin).unwrap();
                    source[start..start + usize::try_from(descriptor.width).unwrap()]
                        .iter()
                        .copied()
                })
                .collect::<Vec<_>>();
            assert_eq!(plane, &expected);
        }

        let mut caller_buffers = descriptors
            .iter()
            .map(|component| {
                vec![0xa5; usize::try_from(component.width * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut caller_planes = caller_buffers
                .iter_mut()
                .zip(&descriptors)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut caller_planes,
            };
            decode_partial_into(&fixture, &mut target, &options).unwrap();
        }
        assert_eq!(caller_buffers, owned_planes);

        let one_options = PartialDecodeOptions {
            components: ComponentSelection::Indices(vec![2]),
            ..options.clone()
        };
        let one = decode_partial(&fixture, &one_options).unwrap();
        assert_eq!(one.component_info, vec![descriptors[2].clone()]);
        assert_eq!(planar_bytes(&one), &owned_planes[2..3]);

        let all_prepared = prepare_part1_decode(&fixture, &options).unwrap();
        let one_prepared = prepare_part1_decode(&fixture, &one_options).unwrap();
        let execute = |prepared: &PreparedPart1Decode<'_>| {
            let mut buffers = prepared
                .component_info()
                .iter()
                .map(|component| {
                    vec![0_u8; usize::try_from(component.width * component.height).unwrap()]
                })
                .collect::<Vec<_>>();
            let timings = {
                let mut planes = buffers
                    .iter_mut()
                    .zip(prepared.component_info())
                    .map(|(samples, component)| {
                        PlaneMut::new(
                            samples,
                            component.width,
                            component.height,
                            usize::try_from(component.width).unwrap(),
                            component.sample_format,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();
                let mut target = ImageViewMut::Planar {
                    info: prepared.info(),
                    planes: &mut planes,
                };
                execute_prepared_part1_decode_into_with_workspace(
                    prepared,
                    &mut target,
                    &mut Part1DecodeWorkspace::new(),
                    codestream::PreparedPart1ExecutionOptions {
                        instrumentation: codestream::DecodeInstrumentation::WorkCounters,
                        parallelism: codestream::DecodeExecutionParallelism::Serial,
                        ..codestream::PreparedPart1ExecutionOptions::default()
                    },
                )
                .unwrap()
            };
            (buffers, timings)
        };
        let (all_buffers, all_work) = execute(&all_prepared);
        let (one_buffers, one_work) = execute(&one_prepared);
        assert_eq!(all_buffers, owned_planes);
        assert_eq!(one_buffers, &owned_planes[2..3]);
        assert!(one_work.executed_code_blocks < all_work.executed_code_blocks);
        assert!(one_work.output_samples < all_work.output_samples);
        assert!(all_work.packet_body_bytes_skipped > 0);
        assert!(one_work.packet_body_bytes_skipped > all_work.packet_body_bytes_skipped);

        let limited = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                max_quality_layers: Some(1),
                ..options
            },
        )
        .unwrap();
        assert_eq!(planar_bytes(&limited), owned_planes);
    }

    #[test]
    fn subsampled_partial_partitions_stitch_on_native_boundaries() {
        let (fixture, expected) = subsampled_fixture(129, 65);
        let full = decode_partial(&fixture, &PartialDecodeOptions::default()).unwrap();
        assert_eq!(planar_bytes(&full), expected.as_slice());

        let left = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 64,
                    height: 65,
                }),
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let right = decode_partial(
            &fixture,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 64,
                    y: 0,
                    width: 65,
                    height: 65,
                }),
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let left_planes = planar_bytes(&left);
        let right_planes = planar_bytes(&right);
        for component in 0..3 {
            let full_info = &full.component_info[component];
            let left_info = &left.component_info[component];
            let right_info = &right.component_info[component];
            assert_eq!(left_info.height, full_info.height);
            assert_eq!(right_info.height, full_info.height);
            assert_eq!(left_info.width + right_info.width, full_info.width);
            assert_eq!(left_info.x_origin + left_info.width, right_info.x_origin);
            let stitched = (0..usize::try_from(full_info.height).unwrap())
                .flat_map(|row| {
                    let left_width = usize::try_from(left_info.width).unwrap();
                    let right_width = usize::try_from(right_info.width).unwrap();
                    left_planes[component][row * left_width..(row + 1) * left_width]
                        .iter()
                        .chain(&right_planes[component][row * right_width..(row + 1) * right_width])
                        .copied()
                })
                .collect::<Vec<_>>();
            assert_eq!(stitched, expected[component]);
        }
    }

    #[test]
    fn subsampled_partial_preflight_is_transactional_and_exclusions_fail_closed() {
        let (fixture, _) = subsampled_fixture(129, 65);
        let options = PartialDecodeOptions {
            region: Some(Region {
                x: 1,
                y: 1,
                width: 31,
                height: 31,
            }),
            ..PartialDecodeOptions::default()
        };
        let info = decode_partial_info(&fixture, &options).unwrap();
        let descriptors = decode_partial_component_info(&fixture, &options).unwrap();
        let mut malformed = fixture.clone();
        let sot = malformed
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x90])
            .unwrap();
        let psot = u32::from_be_bytes(malformed[sot + 6..sot + 10].try_into().unwrap());
        malformed[sot + 6..sot + 10].copy_from_slice(&(psot + 32).to_be_bytes());
        let mut buffers = descriptors
            .iter()
            .map(|component| {
                vec![0x6d; usize::try_from(component.width * component.height).unwrap()]
            })
            .collect::<Vec<_>>();
        {
            let mut planes = buffers
                .iter_mut()
                .zip(&descriptors)
                .map(|(samples, component)| {
                    PlaneMut::new(
                        samples,
                        component.width,
                        component.height,
                        usize::try_from(component.width).unwrap(),
                        component.sample_format,
                    )
                    .unwrap()
                })
                .collect::<Vec<_>>();
            let mut target = ImageViewMut::Planar {
                info: &info,
                planes: &mut planes,
            };
            assert!(decode_partial_into(&malformed, &mut target, &options).is_err());
        }
        assert!(buffers.iter().flatten().all(|sample| *sample == 0x6d));

        let empty_native = PartialDecodeOptions {
            region: Some(Region {
                x: 1,
                y: 0,
                width: 1,
                height: 1,
            }),
            components: ComponentSelection::Indices(vec![1]),
            ..PartialDecodeOptions::default()
        };
        assert!(matches!(
            decode_partial_component_info(&fixture, &empty_native),
            Err(J2kError::InvalidParameter {
                parameter: "region",
                ..
            })
        ));

        for excluded in [
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..options.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..options.clone()
            },
            PartialDecodeOptions {
                tile: Some(TileSelection {
                    tile_x: 0,
                    tile_y: 0,
                }),
                region: None,
                ..options.clone()
            },
        ] {
            assert!(decode_partial(&fixture, &excluded).is_err());
        }

        let mut irreversible = fixture.clone();
        let cod = irreversible
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x52])
            .unwrap();
        irreversible[cod + 13] = 0;
        assert!(!is_direct_selective_part1_component_profile(&irreversible));
        assert!(decode_partial(&irreversible, &options).is_err());

        let mut non_zero_origin = fixture;
        let siz = non_zero_origin
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x51])
            .unwrap();
        non_zero_origin[siz + 6..siz + 10].copy_from_slice(&130_u32.to_be_bytes());
        non_zero_origin[siz + 14..siz + 18].copy_from_slice(&1_u32.to_be_bytes());
        non_zero_origin[siz + 30..siz + 34].copy_from_slice(&1_u32.to_be_bytes());
        assert!(decode_partial(&non_zero_origin, &options).is_err());
    }

    fn insert_main_coc(codestream: Vec<u8>, parameters: [u8; 5]) -> Vec<u8> {
        insert_main_coc_with_scoc(codestream, 0, &parameters)
    }

    fn insert_main_coc_with_scoc(mut codestream: Vec<u8>, scoc: u8, parameters: &[u8]) -> Vec<u8> {
        let sot = codestream
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x90])
            .unwrap();
        let lcoc = u16::try_from(parameters.len() + 4).unwrap();
        let mut coc = vec![0xff, 0x53];
        coc.extend_from_slice(&lcoc.to_be_bytes());
        coc.extend_from_slice(&[0, scoc]);
        coc.extend_from_slice(parameters);
        codestream.splice(sot..sot, coc);
        codestream
    }

    #[test]
    fn component_info_retains_checked_absolute_native_geometry() {
        let samples = (0_u8..16).collect::<Vec<_>>();
        let mut fixture =
            codestream::encode_planar_u8_no_decomp_test_fixture(4, 4, &[&samples, &samples])
                .unwrap();
        let siz = fixture
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x51])
            .unwrap();
        fixture[siz + 6..siz + 10].copy_from_slice(&20_u32.to_be_bytes());
        fixture[siz + 10..siz + 14].copy_from_slice(&18_u32.to_be_bytes());
        fixture[siz + 14..siz + 18].copy_from_slice(&3_u32.to_be_bytes());
        fixture[siz + 18..siz + 22].copy_from_slice(&5_u32.to_be_bytes());
        fixture[siz + 22..siz + 26].copy_from_slice(&20_u32.to_be_bytes());
        fixture[siz + 26..siz + 30].copy_from_slice(&18_u32.to_be_bytes());
        fixture[siz + 30..siz + 34].copy_from_slice(&1_u32.to_be_bytes());
        fixture[siz + 34..siz + 38].copy_from_slice(&2_u32.to_be_bytes());
        fixture[siz + 43] = 0x80 | 10;
        fixture[siz + 44] = 2;
        fixture[siz + 45] = 3;

        let info = part1_component_info_at_resolution(
            &fixture,
            &ComponentSelection::All,
            Some(Region {
                x: 2,
                y: 1,
                width: 9,
                height: 8,
            }),
            1,
        )
        .unwrap();

        assert_eq!(
            info[0],
            ComponentInfo {
                source_component: Some(0),
                width: 4,
                height: 4,
                x_origin: 3,
                y_origin: 3,
                horizontal_separation: 1,
                vertical_separation: 1,
                sample_format: SampleFormat::U8,
            }
        );
        assert_eq!(
            info[1],
            ComponentInfo {
                source_component: Some(1),
                width: 2,
                height: 2,
                x_origin: 2,
                y_origin: 1,
                horizontal_separation: 2,
                vertical_separation: 3,
                sample_format: SampleFormat::with_byte_order(11, true, Some(SampleEndian::Little),)
                    .unwrap(),
            }
        );

        let parsed = codestream::parse(&fixture).unwrap();
        let plan = codestream::plan_tile_region_decode(
            &parsed,
            codestream::TileRegionRequest {
                x: 2,
                y: 1,
                width: 9,
                height: 8,
            },
        )
        .unwrap();
        assert_eq!(plan.tiles.len(), 1);
        assert_eq!(plan.tiles[0].tile.x, 0);
        assert_eq!(plan.tiles[0].tile.y, 0);
        assert_eq!(plan.tiles[0].intersection, plan.request);
    }

    #[test]
    fn reduced_partial_decode_uses_coc_decomposition_over_cod() {
        let samples = (0..64 * 64)
            .map(|sample| ((sample * 17 + sample / 64) & 0xff) as u8)
            .collect::<Vec<_>>();
        let mut codestream =
            codestream::encode_grayscale_u8_one_decomp(codestream::GrayscaleU8Encode {
                width: 64,
                height: 64,
                samples: &samples,
                stride_bytes: 64,
            })
            .unwrap();
        let original = codestream::parse(&codestream)
            .unwrap()
            .coding_style
            .unwrap();
        let transform = match original.transform {
            codestream::WaveletTransform::Irreversible97 => 0,
            codestream::WaveletTransform::Reversible53 => 1,
        };
        let parameters = [
            original.decomposition_levels,
            original.code_block_width_exponent - 2,
            original.code_block_height_exponent - 2,
            original.code_block_style,
            transform,
        ];
        let cod = codestream
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x52])
            .unwrap();
        codestream[cod + 9] = 0;
        let codestream = insert_main_coc(codestream, parameters);

        let parsed = codestream::parse(&codestream).unwrap();
        assert_eq!(parsed.coding_style.unwrap().decomposition_levels, 0);
        assert_eq!(
            parsed
                .uniform_effective_coding_style()
                .unwrap()
                .decomposition_levels,
            1
        );

        let decoded = decode_partial(
            &codestream,
            &PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!(decoded.info.width, 32);
        assert_eq!(decoded.info.height, 32);
    }

    #[test]
    fn odd_origin_reduced_region_matches_info_owned_caller_and_stitch_paths() {
        let samples = (0..64 * 64)
            .map(|sample| ((sample * 29 + sample / 64 * 7) & 0xff) as u8)
            .collect::<Vec<_>>();
        let codestream =
            codestream::encode_grayscale_u8_one_decomp(codestream::GrayscaleU8Encode {
                width: 64,
                height: 64,
                samples: &samples,
                stride_bytes: 64,
            })
            .unwrap();
        let options = PartialDecodeOptions {
            region: Some(Region {
                x: 1,
                y: 3,
                width: 15,
                height: 13,
            }),
            resolution: ResolutionLevel::Reduced { discard_levels: 1 },
            ..PartialDecodeOptions::default()
        };

        let info = decode_partial_info(&codestream, &options).unwrap();
        assert_eq!((info.width, info.height), (7, 6));
        let owned = decode_partial(&codestream, &options).unwrap();
        assert_eq!(owned.info, info);
        assert_eq!(
            owned.component_info[0],
            ComponentInfo {
                source_component: Some(0),
                width: 7,
                height: 6,
                x_origin: 1,
                y_origin: 2,
                horizontal_separation: 1,
                vertical_separation: 1,
                sample_format: SampleFormat::U8,
            }
        );
        let ImageData::Planes(owned_planes) = &owned.data else {
            panic!("planar partial decode returned interleaved data");
        };
        let owned_plane = &owned_planes[0];

        let mut caller_samples = vec![0xa5; usize::try_from(info.width * info.height).unwrap()];
        let mut caller_planes = [PlaneMut::new(
            &mut caller_samples,
            info.width,
            info.height,
            usize::try_from(info.width).unwrap(),
            info.sample_format,
        )
        .unwrap()];
        let mut target = ImageViewMut::Planar {
            info: &info,
            planes: &mut caller_planes,
        };
        decode_partial_into(&codestream, &mut target, &options).unwrap();
        assert_eq!(&caller_samples, owned_plane);

        let full = decode_partial(
            &codestream,
            &PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let ImageData::Planes(full_planes) = full.data else {
            panic!("planar full reduced decode returned interleaved data");
        };
        let expected_from_full = (2_usize..8)
            .flat_map(|y| {
                let row = &full_planes[0][y * 32..(y + 1) * 32];
                row[1..8].iter().copied()
            })
            .collect::<Vec<_>>();
        assert_eq!(owned_plane, &expected_from_full);

        let left = decode_partial(
            &codestream,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 1,
                    y: 3,
                    width: 7,
                    height: 13,
                }),
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        let right = decode_partial(
            &codestream,
            &PartialDecodeOptions {
                region: Some(Region {
                    x: 8,
                    y: 3,
                    width: 8,
                    height: 13,
                }),
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..PartialDecodeOptions::default()
            },
        )
        .unwrap();
        assert_eq!((left.info.width, right.info.width), (3, 4));
        let (ImageData::Planes(left_planes), ImageData::Planes(right_planes)) =
            (left.data, right.data)
        else {
            panic!("partitioned planar decode returned interleaved data");
        };
        let stitched = (0..6_usize)
            .flat_map(|row| {
                left_planes[0][row * 3..(row + 1) * 3]
                    .iter()
                    .chain(&right_planes[0][row * 4..(row + 1) * 4])
                    .copied()
            })
            .collect::<Vec<_>>();
        assert_eq!(stitched, *owned_plane);
    }

    #[test]
    fn inspect_classifies_structural_coc_precincts_before_decode_admission() {
        let samples = (0..16).map(|sample| sample as u8).collect::<Vec<_>>();
        let codestream =
            codestream::encode_planar_u8_no_decomp_test_fixture(4, 4, &[&samples]).unwrap();
        let codestream = insert_main_coc_with_scoc(codestream, 1, &[0, 2, 2, 0, 1, 0x11]);

        let metadata = inspect(&codestream, &InspectOptions::default()).unwrap();
        assert!(matches!(
            metadata.support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::MarkerSegment,
                ref detail,
            } if detail.contains("explicit precinct tables")
        ));
        assert!(matches!(
            decode(&codestream, &DecodeOptions::default()),
            Err(J2kError::Unsupported {
                feature: UnsupportedFeature::MarkerSegment,
                ..
            })
        ));
    }

    #[test]
    fn inspect_reports_the_complete_htj2k_lossless_admission_diagnostic() {
        let samples = (0..64)
            .map(|sample| u8::try_from((sample * 19 + 5) % 251).unwrap())
            .collect::<Vec<_>>();
        let fixture = || {
            codestream::encode_htj2k_grayscale_u8_no_decomp(codestream::GrayscaleU8Encode {
                width: 8,
                height: 8,
                samples: &samples,
                stride_bytes: 8,
            })
            .unwrap()
        };
        let multitile_fixture = |decomposition_levels: u8, declared_tile_parts: u8| {
            let mut codestream = fixture();
            if decomposition_levels != 0 {
                let cod = codestream
                    .windows(2)
                    .position(|bytes| bytes == [0xff, 0x52])
                    .unwrap();
                codestream[cod + 9] = decomposition_levels;
                let qcd = codestream
                    .windows(2)
                    .position(|bytes| bytes == [0xff, 0x5c])
                    .unwrap();
                let old_length = usize::from(u16::from_be_bytes(
                    codestream[qcd + 2..qcd + 4].try_into().unwrap(),
                ));
                let exponent = codestream[qcd + 5];
                let added = usize::from(decomposition_levels) * 3;
                codestream.splice(
                    qcd + 2 + old_length..qcd + 2 + old_length,
                    vec![exponent; added],
                );
                codestream[qcd + 2..qcd + 4]
                    .copy_from_slice(&u16::try_from(old_length + added).unwrap().to_be_bytes());
            }

            let siz = codestream
                .windows(2)
                .position(|bytes| bytes == [0xff, 0x51])
                .unwrap();
            codestream[siz + 22..siz + 26].copy_from_slice(&4_u32.to_be_bytes());
            let sot = codestream
                .windows(2)
                .position(|bytes| bytes == [0xff, 0x90])
                .unwrap();
            codestream[sot + 11] = declared_tile_parts;
            let tile_part_length = usize::try_from(u32::from_be_bytes(
                codestream[sot + 6..sot + 10].try_into().unwrap(),
            ))
            .unwrap();
            let tile_part_end = sot + tile_part_length;
            let mut second_tile = codestream[sot..tile_part_end].to_vec();
            second_tile[4..6].copy_from_slice(&1_u16.to_be_bytes());
            second_tile[11] = declared_tile_parts;
            codestream.splice(tile_part_end..tile_part_end, second_tile);
            codestream
        };

        assert_eq!(
            inspect(&fixture(), &InspectOptions::default())
                .unwrap()
                .support,
            SupportStatus::Supported
        );

        let mut invalid_qcd = fixture();
        let qcd = invalid_qcd
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x5c])
            .unwrap();
        invalid_qcd[qcd + 4] = 0xe0;
        invalid_qcd[qcd + 5] = 0xf8;
        assert!(matches!(
            inspect(&invalid_qcd, &InspectOptions::default())
                .unwrap()
                .support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::MarkerSegment,
                ref detail,
            } if detail.contains("reversible scalar QCD")
        ));

        let mut unsupported_final_set = fixture();
        let cod = unsupported_final_set
            .windows(2)
            .position(|bytes| bytes == [0xff, 0x52])
            .unwrap();
        unsupported_final_set[cod + 12] = 0;
        assert!(matches!(
            inspect(&unsupported_final_set, &InspectOptions::default())
                .unwrap()
                .support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::EntropyCoder,
                ref detail,
            } if detail.contains("HT final coding set")
        ));

        let multitile_decomposition = multitile_fixture(1, 1);
        assert!(matches!(
            inspect(&multitile_decomposition, &InspectOptions::default())
                .unwrap()
                .support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::WaveletTransform,
                ref detail,
            } if detail.contains("multi-tile HTJ2K lossless decode requires zero decomposition")
        ));

        let unspecified_tile_part_count = multitile_fixture(0, 0);
        assert!(matches!(
            inspect(&unspecified_tile_part_count, &InspectOptions::default())
                .unwrap()
                .support,
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::MarkerSegment,
                ref detail,
            } if detail.contains("exactly one tile-part for every tile")
        ));

        let retained_payload_failure = multitile_fixture(0, 1);
        let mut parsed = codestream::parse(&retained_payload_failure).unwrap();
        parsed.tiles[0].payload_offset = Some(usize::MAX);
        assert!(matches!(
            support_from_codestream(&parsed, Some(&retained_payload_failure)),
            SupportStatus::Unsupported {
                feature: UnsupportedFeature::MarkerSegment,
                ref detail,
            } if detail.contains("retained payload state")
        ));
    }

    #[test]
    fn p0_07_route_admits_only_its_exact_bounded_output_request() {
        let admitted = PartialDecodeOptions {
            region: Some(Region {
                x: 0,
                y: 0,
                width: 128,
                height: 128,
            }),
            components: ComponentSelection::Indices(vec![0]),
            ..PartialDecodeOptions::default()
        };
        assert!(is_p0_07_output_request(&admitted));

        let mutations = [
            PartialDecodeOptions {
                region: None,
                ..admitted.clone()
            },
            PartialDecodeOptions {
                region: Some(Region {
                    width: 127,
                    ..admitted.region.unwrap()
                }),
                ..admitted.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::All,
                ..admitted.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 1 },
                ..admitted.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(8),
                ..admitted.clone()
            },
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..admitted.clone()
            },
        ];
        assert!(
            mutations
                .iter()
                .all(|request| !is_p0_07_output_request(request))
        );
    }

    #[test]
    fn p0_08_route_admits_only_its_exact_reduction_five_request() {
        let admitted = PartialDecodeOptions {
            resolution: ResolutionLevel::Reduced { discard_levels: 5 },
            components: ComponentSelection::Indices(vec![0]),
            ..PartialDecodeOptions::default()
        };
        assert!(is_p0_08_output_request(&admitted));

        let mutations = [
            PartialDecodeOptions {
                region: Some(Region {
                    x: 0,
                    y: 0,
                    width: 17,
                    height: 96,
                }),
                ..admitted.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 4 },
                ..admitted.clone()
            },
            PartialDecodeOptions {
                resolution: ResolutionLevel::Reduced { discard_levels: 6 },
                ..admitted.clone()
            },
            PartialDecodeOptions {
                components: ComponentSelection::All,
                ..admitted.clone()
            },
            PartialDecodeOptions {
                max_quality_layers: Some(30),
                ..admitted.clone()
            },
            PartialDecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..admitted.clone()
            },
        ];
        assert!(
            mutations
                .iter()
                .all(|request| !is_p0_08_output_request(request))
        );
    }

    #[test]
    fn p0_10_route_admits_only_component_zero_planar_full_decode() {
        let admitted = DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::Indices(vec![0]),
            ..DecodeOptions::default()
        };
        assert!(is_p0_10_decode_request(&admitted));

        let mutations = [
            DecodeOptions {
                allow_best_effort_backend_decode: true,
                ..admitted.clone()
            },
            DecodeOptions {
                mode: DecodeMode::Rendered,
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::All,
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::Indices(vec![1]),
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::Indices(vec![0, 1]),
                ..admitted.clone()
            },
            DecodeOptions {
                max_quality_layers: Some(2),
                ..admitted.clone()
            },
            DecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..admitted.clone()
            },
        ];
        assert!(
            mutations
                .iter()
                .all(|request| !is_p0_10_decode_request(request))
        );
    }

    #[test]
    fn p0_13_route_admits_only_component_zero_planar_full_decode() {
        let admitted = DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::Indices(vec![0]),
            ..DecodeOptions::default()
        };
        assert!(is_p0_13_decode_request(&admitted));

        let mutations = [
            DecodeOptions {
                allow_best_effort_backend_decode: true,
                ..admitted.clone()
            },
            DecodeOptions {
                mode: DecodeMode::Rendered,
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::All,
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::Indices(vec![1]),
                ..admitted.clone()
            },
            DecodeOptions {
                requested_components: ComponentSelection::Indices(vec![0, 1]),
                ..admitted.clone()
            },
            DecodeOptions {
                max_quality_layers: Some(1),
                ..admitted.clone()
            },
            DecodeOptions {
                target_layout: ComponentLayout::Interleaved,
                ..admitted.clone()
            },
        ];
        assert!(
            mutations
                .iter()
                .all(|request| !is_p0_13_decode_request(request))
        );
    }
}
