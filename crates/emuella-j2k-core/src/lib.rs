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
    codestream: codestream::PreparedPart1ComponentDecode<'a>,
}

impl PreparedPart1Decode<'_> {
    pub fn info(&self) -> &ImageInfo {
        &self.info
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
    /// reconstructs every layer in the admitted profile.
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
    max_layers: Option<u16>,
) -> Result<()> {
    if max_layers.is_some() && primary_part1_codestream_bytes(input, metadata)?.is_none() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "maximum quality-layer selection is currently available only for Part 1 J2K and JP2 component decode",
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
    pub region: Option<Region>,
    pub tile: Option<TileSelection>,
    pub resolution: ResolutionLevel,
    pub components: ComponentSelection,
    /// Maximum number of leading quality layers to reconstruct.
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
    validate_max_quality_layer_profile(input, &metadata, options.max_quality_layers)?;
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
    validate_max_quality_layer_profile(input, &metadata, options.max_quality_layers)?;
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
    validate_max_quality_layer_profile(input, &metadata, options.max_quality_layers)?;
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
    validate_max_quality_layer_profile(input, &metadata, options.max_quality_layers)?;
    requested_component_indices(&metadata, &options.requested_components)?;
    if options.allow_best_effort_backend_decode {
        validate_native_best_effort_decode_request(&metadata)?;
    }
    reject_unsupported_rendered_projection(&metadata, options)?;
    reject_unsupported_part1_rendered_sampling(input, &metadata, options)?;

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
        if codestream::is_supported_part1_native_subsampled_component_profile(&parsed) {
            return Err(unsupported(
                UnsupportedFeature::ComponentLayout,
                "rendered output does not implicitly resample unequal native component grids; request planar component mode",
            ));
        }
    }
    Ok(())
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

fn decoded_baseline_to_image(
    decoded: codestream::DecodedImage,
    options: &DecodeOptions,
) -> Result<Image> {
    decoded_baseline_to_image_with_component_info(decoded, options, None)
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
        !codestream::is_supported_part1_native_subsampled_component_profile(&parsed)
            && (codestream::is_owned_baseline_profile(codestream_bytes)
                || codestream::is_supported_part1_bounded_poc_component_profile(
                    codestream_bytes,
                    &parsed,
                )
                || codestream::is_supported_part1_selective_irreversible97_component_profile(
                    codestream_bytes,
                    &parsed,
                ))
    })
}

fn part1_component_info(
    codestream_bytes: &[u8],
    selection: &ComponentSelection,
    region: Option<Region>,
) -> Result<Vec<ComponentInfo>> {
    let codestream = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
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
            let x_separation = u32::from(component.horizontal_separation);
            let y_separation = u32::from(component.vertical_separation);
            let (reference_x0, reference_y0, reference_x1, reference_y1) = match region {
                Some(region) => {
                    let x0 = codestream
                        .siz
                        .image_origin_x
                        .checked_add(region.x)
                        .ok_or_else(sample_size_overflow)?;
                    let y0 = codestream
                        .siz
                        .image_origin_y
                        .checked_add(region.y)
                        .ok_or_else(sample_size_overflow)?;
                    let x1 = x0
                        .checked_add(region.width)
                        .ok_or_else(sample_size_overflow)?;
                    let y1 = y0
                        .checked_add(region.height)
                        .ok_or_else(sample_size_overflow)?;
                    (x0, y0, x1, y1)
                }
                None => (
                    codestream.siz.image_origin_x,
                    codestream.siz.image_origin_y,
                    codestream.siz.reference_grid_width,
                    codestream.siz.reference_grid_height,
                ),
            };
            let x_origin = ceil_div_u32(reference_x0, x_separation)?;
            let y_origin = ceil_div_u32(reference_y0, y_separation)?;
            let x_end = ceil_div_u32(reference_x1, x_separation)?;
            let y_end = ceil_div_u32(reference_y1, y_separation)?;
            let byte_order = (component.bits_per_sample > 8).then_some(SampleEndian::Little);
            Ok(ComponentInfo {
                source_component: Some(component_index),
                width: x_end
                    .checked_sub(x_origin)
                    .ok_or_else(sample_size_overflow)?,
                height: y_end
                    .checked_sub(y_origin)
                    .ok_or_else(sample_size_overflow)?,
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
/// This validates the partial request, performs the supported full decode, then
/// crops regions, selects components, and re-lays out samples in memory. It is
/// not packet-indexed or codestream-level partial decode.
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
    let mut component_info = part1_component_info(codestream_bytes, &options.components, None)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 reduced MCT decode requires image dimensions",
        )
    })?;
    let source_region = options.region.unwrap_or(Region {
        x: 0,
        y: 0,
        width: image.width,
        height: image.height,
    });
    let reduced_region =
        reduced_roi_region(source_region, discard_levels, image.width, image.height)?;
    for component in &mut component_info {
        component.width = reduced_region.width;
        component.height = reduced_region.height;
        component.x_origin = reduced_region.x;
        component.y_origin = reduced_region.y;
    }
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
    let mut component_info = part1_component_info(codestream_bytes, &options.components, None)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 reduced irreversible MCT decode requires image dimensions",
        )
    })?;
    let reduced_region = reduced_roi_region(
        Region {
            x: 0,
            y: 0,
            width: image.width,
            height: image.height,
        },
        discard_levels,
        image.width,
        image.height,
    )?;
    for component in &mut component_info {
        component.width = reduced_region.width;
        component.height = reduced_region.height;
        component.x_origin = reduced_region.x;
        component.y_origin = reduced_region.y;
    }
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
    let mut component_info = part1_component_info(codestream_bytes, &options.components, None)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 reduced heterogeneous component decode requires image dimensions",
        )
    })?;
    let reduced_region = reduced_roi_region(
        Region {
            x: 0,
            y: 0,
            width: image.width,
            height: image.height,
        },
        discard_levels,
        image.width,
        image.height,
    )?;
    for component in &mut component_info {
        component.width = reduced_region.width;
        component.height = reduced_region.height;
        component.x_origin = reduced_region.x;
        component.y_origin = reduced_region.y;
    }
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
    let mut component_info = part1_component_info(codestream_bytes, &options.components, None)?;
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "Profile-0 reduced ROI component decode requires image dimensions",
        )
    })?;
    let reduced_region = reduced_roi_region(
        Region {
            x: 0,
            y: 0,
            width: image.width,
            height: image.height,
        },
        discard_levels,
        image.width,
        image.height,
    )?;
    for component in &mut component_info {
        component.width = reduced_region.width;
        component.height = reduced_region.height;
        component.x_origin = reduced_region.x;
        component.y_origin = reduced_region.y;
    }
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
/// This is the partial-decode counterpart to [`decode_shape`]. Callers that
/// provide their own planes can use the returned dimensions, component count,
/// sample format, and layout to allocate and validate a target before calling
/// [`decode_partial_into_with_workspace`].
pub fn decode_partial_info(input: &[u8], options: &PartialDecodeOptions) -> Result<ImageInfo> {
    if input.is_empty() {
        return Err(J2kError::TruncatedInput {
            needed: 1,
            remaining: 0,
        });
    }
    partial_decode_target_info(input, options)
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
    let mut component_info = part1_component_info(codestream_bytes, &options.components, None)?;
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
    let reduced_region = reduced_roi_region(
        source_region,
        match options.resolution {
            ResolutionLevel::Reduced { discard_levels } => discard_levels,
            ResolutionLevel::Full => 0,
        },
        image.width,
        image.height,
    )?;
    for component in &mut component_info {
        component.width = info.width;
        component.height = info.height;
        component.x_origin = reduced_region.x;
        component.y_origin = reduced_region.y;
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
    if options.resolution != ResolutionLevel::Full || options.tile.is_some() {
        return Ok(None);
    }
    if options.region.is_none() && options.max_quality_layers.is_none() {
        return Ok(None);
    }
    let Some(codestream_bytes) = primary_part1_codestream_bytes(input, metadata)? else {
        return Ok(None);
    };
    if !is_direct_selective_part1_component_profile(codestream_bytes) {
        return Ok(None);
    }
    let parsed = codestream::parse(codestream_bytes).map_err(map_codestream_error)?;
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Ok(None);
    }

    validate_partial_options_without_support(metadata, options)?;
    let region = partial_output_region(metadata, options)?;
    let component_indices = partial_component_indices(metadata, &options.components)?;
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
    let requested_components = if metadata.image.as_ref().is_some_and(|image| {
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
    let Some(info) = selective_part1_discard_target_info(input, metadata, options)? else {
        return Ok(None);
    };
    let ResolutionLevel::Reduced { discard_levels } = options.resolution else {
        return Ok(None);
    };
    let image = metadata.image.as_ref().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "partial discard planning requires image dimensions",
        )
    })?;
    let codestream_bytes = primary_part1_codestream_bytes(input, metadata)?;
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

fn reduced_roi_region(
    region: Region,
    discard_levels: u8,
    image_width: u32,
    image_height: u32,
) -> Result<Region> {
    let scale = 1_u32
        .checked_shl(u32::from(discard_levels))
        .ok_or_else(sample_size_overflow)?;
    let x1 = region
        .x
        .checked_add(region.width)
        .ok_or_else(sample_size_overflow)?;
    let y1 = region
        .y
        .checked_add(region.height)
        .ok_or_else(sample_size_overflow)?;
    if x1 > image_width || y1 > image_height {
        return Err(J2kError::InvalidParameter {
            parameter: "region",
            message: "partial decode region must fit inside the image bounds",
        });
    }

    let rx0 = region.x / scale;
    let ry0 = region.y / scale;
    let rx1 = ceil_div_u32(x1, scale)?;
    let ry1 = ceil_div_u32(y1, scale)?;

    Ok(Region {
        x: rx0,
        y: ry0,
        width: rx1.checked_sub(rx0).ok_or_else(sample_size_overflow)?,
        height: ry1.checked_sub(ry0).ok_or_else(sample_size_overflow)?,
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
    if options.target_layout != ComponentLayout::Planar || options.tile.is_some() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "prepared Part 1 decode requires planar component output and an image/region request",
        ));
    }
    let info = partial_decode_target_info(input, options)?;
    let metadata = inspect(input, &InspectOptions::default())?;
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
    let discard_levels = match options.resolution {
        ResolutionLevel::Full | ResolutionLevel::Reduced { discard_levels: 0 } => 0,
        ResolutionLevel::Reduced { discard_levels } => discard_levels,
    };
    if discard_levels == 0 {
        validate_partial_options_without_support(&metadata, options)?;
    } else if selective_part1_discard_target_info(input, &metadata, options)?.is_none() {
        return Err(unsupported(
            UnsupportedFeature::PartialDecodeMode,
            "input is outside the direct selective Part 1 discard profile",
        ));
    }
    let region = partial_output_region(&metadata, options)?;
    let component_indices = partial_component_indices(&metadata, &options.components)?;
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
    Ok(PreparedPart1Decode { info, codestream })
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
    let (width, height) = codestream.output_dimensions();
    let components = u16::try_from(codestream.component_indices().len()).map_err(|_| {
        unsupported(
            UnsupportedFeature::ComponentLayout,
            "source-backed component selection exceeds the public image model",
        )
    })?;
    let (bits_per_sample, signed) = codestream.selected_sample_format().ok_or_else(|| {
        unsupported(
            UnsupportedFeature::ComponentLayout,
            "source-backed prepared output requires a uniform selected sample representation",
        )
    })?;
    let sample_format = SampleFormat::with_byte_order(
        bits_per_sample,
        signed,
        (bits_per_sample > 8).then_some(SampleEndian::Little),
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
    Ok(PreparedPart1Decode { info, codestream })
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
    validate_decode_target(&prepared.info, target)?;
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
    validate_decode_target(&expected_info, target)?;
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
    if options.tile.is_some() {
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
    if parsed
        .uniform_effective_coding_style()
        .is_some_and(|coding_style| coding_style.multiple_component_transform)
    {
        return Ok(false);
    }
    if discard_levels == 0 {
        validate_partial_options_without_support(&metadata, options)?;
    } else if selective_part1_discard_target_info(input, &metadata, options)?.is_none() {
        return Ok(false);
    }
    let region = partial_output_region(&metadata, options)?;
    let component_indices = partial_component_indices(&metadata, &options.components)?;
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
        && bytes.is_some_and(|bytes| codestream::is_htj2k_lossless_profile(bytes, codestream))
    {
        return SupportStatus::Supported;
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
    decoded_baseline_to_image(decoded, options).map(Some)
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
                .saturating_add(region.x / u32::from(component.horizontal_separation));
            component.y_origin = component
                .y_origin
                .saturating_add(region.y / u32::from(component.vertical_separation));
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
    let reduced_region =
        reduced_roi_region(full_region, discard_levels, image.width, image.height)?;
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
    value
        .checked_add(divisor - 1)
        .map(|value| value / divisor)
        .ok_or_else(sample_size_overflow)
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
}
