#![cfg_attr(not(feature = "std"), no_std)]
//! JP2 and JPH container boundary.
//!
//! This crate owns box parsing, brand checks, codestream box discovery, raw
//! metadata preservation, and small writer primitives. APIs operate on byte
//! slices or caller-owned byte buffers so higher layers do not need path-only
//! IO.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// JPEG 2000 family container kind identified from the file signature and brand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerKind {
    /// JP2 Part 1 container.
    Jp2,
    /// JPH container for HTJ2K codestreams.
    Jph,
}

impl ContainerKind {
    fn brand(self) -> FourCc {
        match self {
            Self::Jp2 => boxes::BRAND_JP2,
            Self::Jph => boxes::BRAND_JPH,
        }
    }
}

/// Raw metadata box family preserved for higher-level callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataBoxKind {
    /// XML box content.
    Xml,
    /// UUID box content.
    Uuid,
    /// Unknown box content that should be preserved byte-for-byte.
    Unknown,
}

/// Parsed JP2/JPH document summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub kind: ContainerKind,
    pub file_type: FileTypeBox,
    pub image_header: Option<ImageHeaderBox>,
    pub bits_per_component: Option<BitsPerComponentBox>,
    pub color_specification: Option<ColorSpecificationBox>,
    pub codestreams: Vec<CodestreamBox>,
    pub metadata: Vec<MetadataBox>,
    pub boxes: Vec<BoxRecord>,
}

impl Container {
    /// Return the first codestream payload, if present.
    pub fn primary_codestream<'a>(&self, input: &'a [u8]) -> Result<Option<&'a [u8]>> {
        match self.codestreams.first() {
            Some(codestream) => Ok(Some(checked_slice(
                input,
                codestream.data_offset,
                codestream.data_len,
            )?)),
            None => Ok(None),
        }
    }

    /// Return sample precision from `bpcc` when present, otherwise from `ihdr`.
    pub fn component_sample_formats(&self) -> Option<Vec<ComponentSampleFormat>> {
        if let Some(bits_per_component) = &self.bits_per_component {
            return Some(bits_per_component.components.clone());
        }

        let image_header = self.image_header?;
        if image_header.bits_per_component == 255 {
            return None;
        }

        let sample_format = ComponentSampleFormat::from_bpc_byte(image_header.bits_per_component);
        Some(alloc::vec![sample_format; usize::from(image_header.components)])
    }
}

/// File Type box (`ftyp`) contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTypeBox {
    pub brand: FourCc,
    pub minor_version: u32,
    pub compatible_brands: Vec<FourCc>,
}

/// Image Header box (`ihdr`) contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageHeaderBox {
    pub width: u32,
    pub height: u32,
    pub components: u16,
    pub bits_per_component: u8,
    pub compression_type: u8,
    pub unknown_color_space: bool,
    pub intellectual_property: bool,
}

impl ImageHeaderBox {
    pub fn sample_format(self) -> Option<ComponentSampleFormat> {
        if self.bits_per_component == 255 {
            None
        } else {
            Some(ComponentSampleFormat::from_bpc_byte(
                self.bits_per_component,
            ))
        }
    }
}

/// Component sample precision and signedness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentSampleFormat {
    pub bits_per_sample: u8,
    pub signed: bool,
}

impl ComponentSampleFormat {
    pub fn from_bpc_byte(value: u8) -> Self {
        Self {
            bits_per_sample: (value & 0x7f) + 1,
            signed: (value & 0x80) != 0,
        }
    }

    pub fn to_bpc_byte(self) -> Result<u8> {
        if !(1..=38).contains(&self.bits_per_sample) {
            return Err(ContainerError::InvalidBox {
                offset: None,
                box_type: None,
                message: "component precision must be in 1..=38".to_string(),
            });
        }

        Ok((self.bits_per_sample - 1) | if self.signed { 0x80 } else { 0 })
    }
}

/// Bits Per Component box (`bpcc`) contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitsPerComponentBox {
    pub components: Vec<ComponentSampleFormat>,
}

/// Colour Specification box (`colr`) contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorSpecificationBox {
    pub method: ColorSpecificationMethod,
    pub precedence: u8,
    pub approximation: u8,
    pub enumerated_color_space: Option<EnumeratedColorSpace>,
}

/// JP2 colour specification method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpecificationMethod {
    Enumerated,
    RestrictedIccProfile,
    AnyIccProfile,
    Vendor(u8),
}

/// JP2 enumerated colour spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumeratedColorSpace {
    SRgb,
    Greyscale,
    SYcc,
    Unknown(u32),
}

/// Contiguous Codestream box (`jp2c`) location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodestreamBox {
    pub data_offset: usize,
    pub data_len: usize,
}

/// Raw metadata box preserved from the container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataBox {
    pub kind: MetadataBoxKind,
    pub box_type: FourCc,
    pub data_offset: usize,
    pub data: Vec<u8>,
}

/// Parsed top-level or nested box record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxRecord {
    pub box_type: FourCc,
    pub header_offset: usize,
    pub data_offset: usize,
    pub data_len: usize,
    pub total_len: usize,
}

/// Four-byte JPEG 2000 box type or brand code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCc(pub [u8; 4]);

impl FourCc {
    pub const fn new(value: [u8; 4]) -> Self {
        Self(value)
    }

    pub fn as_bytes(self) -> [u8; 4] {
        self.0
    }

    pub fn as_ascii_lossy(self) -> String {
        self.0
            .iter()
            .map(|byte| {
                if byte.is_ascii_graphic() || *byte == b' ' {
                    char::from(*byte)
                } else {
                    '.'
                }
            })
            .collect()
    }
}

impl fmt::Display for FourCc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_ascii_lossy())
    }
}

/// Well-known JP2/JPH box types and brands.
pub mod boxes {
    use super::FourCc;

    pub const SIGNATURE: FourCc = FourCc::new(*b"jP  ");
    pub const FILE_TYPE: FourCc = FourCc::new(*b"ftyp");
    pub const JP2_HEADER: FourCc = FourCc::new(*b"jp2h");
    pub const IMAGE_HEADER: FourCc = FourCc::new(*b"ihdr");
    pub const BITS_PER_COMPONENT: FourCc = FourCc::new(*b"bpcc");
    pub const COLOR_SPECIFICATION: FourCc = FourCc::new(*b"colr");
    pub const PALETTE: FourCc = FourCc::new(*b"pclr");
    pub const COMPONENT_MAPPING: FourCc = FourCc::new(*b"cmap");
    pub const CHANNEL_DEFINITION: FourCc = FourCc::new(*b"cdef");
    pub const CONTIGUOUS_CODESTREAM: FourCc = FourCc::new(*b"jp2c");
    pub const XML: FourCc = FourCc::new(*b"xml ");
    pub const UUID: FourCc = FourCc::new(*b"uuid");

    pub const BRAND_JP2: FourCc = FourCc::new(*b"jp2 ");
    pub const BRAND_JPH: FourCc = FourCc::new(*b"jph ");
}

/// Container parser and writer error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerError {
    TruncatedInput {
        offset: usize,
        needed: usize,
        remaining: usize,
    },
    InvalidBox {
        offset: Option<usize>,
        box_type: Option<FourCc>,
        message: String,
    },
    Unsupported {
        offset: Option<usize>,
        box_type: Option<FourCc>,
        message: String,
    },
    SizeOverflow,
}

impl fmt::Display for ContainerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedInput {
                offset,
                needed,
                remaining,
            } => write!(
                f,
                "truncated container input at byte {offset}: needed {needed} bytes, had {remaining}"
            ),
            Self::InvalidBox {
                offset,
                box_type,
                message,
            } => match (offset, box_type) {
                (Some(offset), Some(box_type)) => {
                    write!(f, "invalid `{box_type}` box at byte {offset}: {message}")
                }
                (Some(offset), None) => write!(f, "invalid box at byte {offset}: {message}"),
                (None, Some(box_type)) => write!(f, "invalid `{box_type}` box: {message}"),
                (None, None) => write!(f, "invalid box: {message}"),
            },
            Self::Unsupported {
                offset,
                box_type,
                message,
            } => match (offset, box_type) {
                (Some(offset), Some(box_type)) => {
                    write!(
                        f,
                        "unsupported `{box_type}` box at byte {offset}: {message}"
                    )
                }
                (Some(offset), None) => write!(f, "unsupported box at byte {offset}: {message}"),
                (None, Some(box_type)) => write!(f, "unsupported `{box_type}` box: {message}"),
                (None, None) => write!(f, "unsupported box: {message}"),
            },
            Self::SizeOverflow => f.write_str("container size overflowed usize or u64"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ContainerError {}

/// Convenient result alias for container operations.
pub type Result<T> = core::result::Result<T, ContainerError>;

/// Boundary marker for container parser state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerBoundary {
    pub kind: ContainerKind,
}

/// Parse a JP2 or JPH container from a byte slice.
pub fn parse(input: &[u8]) -> Result<Container> {
    if input.is_empty() {
        return Err(ContainerError::TruncatedInput {
            offset: 0,
            needed: 12,
            remaining: 0,
        });
    }

    let top_level = parse_box_range(input, 0, input.len())?;
    let signature = top_level
        .first()
        .ok_or_else(|| invalid(None, None, "container must contain a signature box"))?;
    if signature.box_type != boxes::SIGNATURE {
        return Err(invalid(
            Some(signature.header_offset),
            Some(signature.box_type),
            "first box must be the JPEG 2000 signature box",
        ));
    }
    if checked_slice(input, signature.data_offset, signature.data_len)? != [0x0d, 0x0a, 0x87, 0x0a]
    {
        return Err(invalid(
            Some(signature.header_offset),
            Some(signature.box_type),
            "signature payload is not the JPEG 2000 magic value",
        ));
    }

    let file_type_record = top_level
        .iter()
        .find(|record| record.box_type == boxes::FILE_TYPE)
        .ok_or_else(|| invalid(None, None, "container must contain a file type box"))?;
    let file_type = parse_file_type(input, file_type_record)?;
    let kind = container_kind(&file_type)?;
    if kind == ContainerKind::Jp2 {
        validate_jp2_top_level_structure(&top_level)?;
    }

    let mut image_header = None;
    let mut bits_per_component = None;
    let mut color_specification = None;
    let mut codestreams = Vec::new();
    let mut metadata = Vec::new();
    let mut boxes = top_level.clone();

    for record in &top_level {
        match record.box_type {
            boxes::JP2_HEADER => {
                let children = parse_box_range(input, record.data_offset, record.end_offset()?)?;
                if kind == ContainerKind::Jp2 {
                    validate_jp2_header_structure(&children, record)?;
                }
                for child in &children {
                    match child.box_type {
                        boxes::IMAGE_HEADER => {
                            image_header = Some(parse_image_header(
                                input,
                                child,
                                kind == ContainerKind::Jp2,
                            )?);
                        }
                        boxes::BITS_PER_COMPONENT => {
                            bits_per_component = Some(parse_bits_per_component(
                                input,
                                child,
                                kind == ContainerKind::Jp2,
                            )?);
                        }
                        boxes::COLOR_SPECIFICATION => {
                            let parsed = parse_color_specification(input, child)?;
                            if color_specification.is_none() || kind == ContainerKind::Jph {
                                color_specification = Some(parsed);
                            }
                        }
                        boxes::XML | boxes::UUID => {
                            metadata.push(preserve_metadata(input, child)?);
                        }
                        _ => {
                            metadata.push(preserve_unknown_metadata(input, child)?);
                        }
                    }
                }
                boxes.extend(children);
            }
            boxes::CONTIGUOUS_CODESTREAM => codestreams.push(CodestreamBox {
                data_offset: record.data_offset,
                data_len: record.data_len,
            }),
            boxes::XML | boxes::UUID => metadata.push(preserve_metadata(input, record)?),
            box_type
                if box_type != boxes::SIGNATURE
                    && box_type != boxes::FILE_TYPE
                    && box_type != boxes::JP2_HEADER =>
            {
                metadata.push(preserve_unknown_metadata(input, record)?);
            }
            _ => {}
        }
    }

    if kind == ContainerKind::Jp2 {
        validate_jp2_header_fields(image_header, bits_per_component.as_ref(), &boxes)?;
    } else {
        validate_header_consistency(image_header, bits_per_component.as_ref())?;
    }

    Ok(Container {
        kind,
        file_type,
        image_header,
        bits_per_component,
        color_specification,
        codestreams,
        metadata,
        boxes,
    })
}

/// Write a complete JP2/JPH box to a caller-owned output buffer.
pub fn write_box(output: &mut Vec<u8>, box_type: FourCc, contents: &[u8]) -> Result<()> {
    let total_len = contents
        .len()
        .checked_add(8)
        .ok_or(ContainerError::SizeOverflow)?;
    if total_len <= u32::MAX as usize {
        output.extend_from_slice(&(total_len as u32).to_be_bytes());
        output.extend_from_slice(&box_type.as_bytes());
    } else {
        output.extend_from_slice(&1_u32.to_be_bytes());
        output.extend_from_slice(&box_type.as_bytes());
        let extended_len = contents
            .len()
            .checked_add(16)
            .ok_or(ContainerError::SizeOverflow)?;
        output.extend_from_slice(&(extended_len as u64).to_be_bytes());
    }
    output.extend_from_slice(contents);
    Ok(())
}

/// Append the fixed JPEG 2000 signature box.
pub fn write_signature_box(output: &mut Vec<u8>) -> Result<()> {
    write_box(output, boxes::SIGNATURE, &[0x0d, 0x0a, 0x87, 0x0a])
}

/// Append a JP2/JPH file type box.
pub fn write_file_type_box(
    output: &mut Vec<u8>,
    kind: ContainerKind,
    minor_version: u32,
    compatible_brands: &[FourCc],
) -> Result<()> {
    let mut contents = Vec::new();
    contents.extend_from_slice(&kind.brand().as_bytes());
    contents.extend_from_slice(&minor_version.to_be_bytes());
    if compatible_brands.is_empty() {
        contents.extend_from_slice(&kind.brand().as_bytes());
    } else {
        for brand in compatible_brands {
            contents.extend_from_slice(&brand.as_bytes());
        }
    }
    write_box(output, boxes::FILE_TYPE, &contents)
}

/// Append an image header box.
pub fn write_image_header_box(output: &mut Vec<u8>, image_header: ImageHeaderBox) -> Result<()> {
    if image_header.width == 0 || image_header.height == 0 || image_header.components == 0 {
        return Err(invalid(
            None,
            Some(boxes::IMAGE_HEADER),
            "image dimensions and component count must be non-zero",
        ));
    }

    let mut contents = Vec::new();
    contents.extend_from_slice(&image_header.height.to_be_bytes());
    contents.extend_from_slice(&image_header.width.to_be_bytes());
    contents.extend_from_slice(&image_header.components.to_be_bytes());
    contents.push(image_header.bits_per_component);
    contents.push(image_header.compression_type);
    contents.push(u8::from(image_header.unknown_color_space));
    contents.push(u8::from(image_header.intellectual_property));
    write_box(output, boxes::IMAGE_HEADER, &contents)
}

/// Append a bits-per-component box.
pub fn write_bits_per_component_box(
    output: &mut Vec<u8>,
    bits_per_component: &BitsPerComponentBox,
) -> Result<()> {
    let mut contents = Vec::with_capacity(bits_per_component.components.len());
    for component in &bits_per_component.components {
        contents.push(component.to_bpc_byte()?);
    }
    write_box(output, boxes::BITS_PER_COMPONENT, &contents)
}

/// Append an enumerated colour specification box.
pub fn write_color_specification_box(
    output: &mut Vec<u8>,
    color_specification: ColorSpecificationBox,
) -> Result<()> {
    let mut contents = Vec::new();
    contents.push(match color_specification.method {
        ColorSpecificationMethod::Enumerated => 1,
        ColorSpecificationMethod::RestrictedIccProfile => 2,
        ColorSpecificationMethod::AnyIccProfile => 3,
        ColorSpecificationMethod::Vendor(value) => value,
    });
    contents.push(color_specification.precedence);
    contents.push(color_specification.approximation);
    if let Some(color_space) = color_specification.enumerated_color_space {
        contents.extend_from_slice(
            &match color_space {
                EnumeratedColorSpace::SRgb => 16,
                EnumeratedColorSpace::Greyscale => 17,
                EnumeratedColorSpace::SYcc => 18,
                EnumeratedColorSpace::Unknown(value) => value,
            }
            .to_be_bytes(),
        );
    }
    write_box(output, boxes::COLOR_SPECIFICATION, &contents)
}

/// Append a JP2 Header superbox from child box bytes.
pub fn write_jp2_header_box(output: &mut Vec<u8>, child_boxes: &[u8]) -> Result<()> {
    write_box(output, boxes::JP2_HEADER, child_boxes)
}

/// Append a contiguous codestream box.
pub fn write_contiguous_codestream_box(output: &mut Vec<u8>, codestream: &[u8]) -> Result<()> {
    write_box(output, boxes::CONTIGUOUS_CODESTREAM, codestream)
}

fn parse_box_range(input: &[u8], start: usize, end: usize) -> Result<Vec<BoxRecord>> {
    if start > end || end > input.len() {
        return Err(ContainerError::SizeOverflow);
    }

    let mut offset = start;
    let mut records = Vec::new();
    while offset < end {
        let header = parse_box_header(input, offset, end)?;
        if header.total_len == 0 {
            return Err(invalid(
                Some(offset),
                Some(header.box_type),
                "zero-length boxes are only valid at top level and are not used by this parser",
            ));
        }
        offset = offset
            .checked_add(header.total_len)
            .ok_or(ContainerError::SizeOverflow)?;
        if offset > end {
            return Err(invalid(
                Some(header.header_offset),
                Some(header.box_type),
                "box length exceeds containing box bounds",
            ));
        }
        records.push(header);
    }
    Ok(records)
}

fn parse_box_header(input: &[u8], offset: usize, range_end: usize) -> Result<BoxRecord> {
    require(input, offset, 8)?;
    let length = read_u32(input, offset)?;
    let box_type = FourCc::new(read_array(input, offset + 4)?);

    let (header_len, total_len) = match length {
        0 => {
            if range_end != input.len() {
                return Err(invalid(
                    Some(offset),
                    Some(box_type),
                    "length-to-end boxes are only valid at top level",
                ));
            }
            (
                8,
                range_end
                    .checked_sub(offset)
                    .ok_or(ContainerError::SizeOverflow)?,
            )
        }
        1 => {
            require(input, offset, 16)?;
            let extended = read_u64(input, offset + 8)?;
            let total_len = usize::try_from(extended).map_err(|_| ContainerError::SizeOverflow)?;
            (16, total_len)
        }
        value => (8, value as usize),
    };

    if total_len < header_len {
        return Err(invalid(
            Some(offset),
            Some(box_type),
            "box length is smaller than its header",
        ));
    }

    let data_offset = offset
        .checked_add(header_len)
        .ok_or(ContainerError::SizeOverflow)?;
    let data_len = total_len
        .checked_sub(header_len)
        .ok_or(ContainerError::SizeOverflow)?;
    let box_end = offset
        .checked_add(total_len)
        .ok_or(ContainerError::SizeOverflow)?;
    if box_end > range_end {
        return Err(ContainerError::TruncatedInput {
            offset,
            needed: box_end - offset,
            remaining: range_end - offset,
        });
    }

    Ok(BoxRecord {
        box_type,
        header_offset: offset,
        data_offset,
        data_len,
        total_len,
    })
}

fn parse_file_type(input: &[u8], record: &BoxRecord) -> Result<FileTypeBox> {
    if record.data_len < 8 || !(record.data_len - 8).is_multiple_of(4) {
        return Err(invalid(
            Some(record.header_offset),
            Some(record.box_type),
            "file type box must contain brand, minor version, and four-byte compatible brands",
        ));
    }

    let brand = FourCc::new(read_array(input, record.data_offset)?);
    let minor_version = read_u32(input, record.data_offset + 4)?;
    let mut compatible_brands = Vec::new();
    let mut offset = record.data_offset + 8;
    let end = record.end_offset()?;
    while offset < end {
        compatible_brands.push(FourCc::new(read_array(input, offset)?));
        offset += 4;
    }

    Ok(FileTypeBox {
        brand,
        minor_version,
        compatible_brands,
    })
}

fn parse_image_header(
    input: &[u8],
    record: &BoxRecord,
    strict_jp2: bool,
) -> Result<ImageHeaderBox> {
    if record.data_len != 14 {
        return Err(invalid(
            Some(record.header_offset),
            Some(record.box_type),
            "image header box must be exactly 14 bytes",
        ));
    }

    let height = read_u32(input, record.data_offset)?;
    let width = read_u32(input, record.data_offset + 4)?;
    let components = read_u16(input, record.data_offset + 8)?;
    if width == 0 || height == 0 || components == 0 {
        return Err(invalid(
            Some(record.header_offset),
            Some(record.box_type),
            "image dimensions and component count must be non-zero",
        ));
    }
    if strict_jp2 && components > 16_384 {
        return Err(invalid(
            Some(record.data_offset + 8),
            Some(record.box_type),
            "JP2 image header component count exceeds 16384",
        ));
    }

    let bits_per_component = input[record.data_offset + 10];
    if strict_jp2 && bits_per_component != 255 && (bits_per_component & 0x7f) > 37 {
        return Err(invalid(
            Some(record.data_offset + 10),
            Some(record.box_type),
            "image header component precision is reserved",
        ));
    }
    let compression_type = input[record.data_offset + 11];
    if strict_jp2 && compression_type != 7 {
        return Err(invalid(
            Some(record.data_offset + 11),
            Some(record.box_type),
            "JP2 image header compression type must be 7",
        ));
    }
    let unknown_color_space = input[record.data_offset + 12];
    if strict_jp2 && unknown_color_space > 1 {
        return Err(invalid(
            Some(record.data_offset + 12),
            Some(record.box_type),
            "image header colourspace-known flag is reserved",
        ));
    }
    let intellectual_property = input[record.data_offset + 13];
    if strict_jp2 && intellectual_property > 1 {
        return Err(invalid(
            Some(record.data_offset + 13),
            Some(record.box_type),
            "image header intellectual-property flag is reserved",
        ));
    }

    Ok(ImageHeaderBox {
        height,
        width,
        components,
        bits_per_component,
        compression_type,
        unknown_color_space: unknown_color_space != 0,
        intellectual_property: intellectual_property != 0,
    })
}

fn parse_bits_per_component(
    input: &[u8],
    record: &BoxRecord,
    strict_jp2: bool,
) -> Result<BitsPerComponentBox> {
    let bytes = checked_slice(input, record.data_offset, record.data_len)?;
    if strict_jp2 && let Some(index) = bytes.iter().position(|byte| (*byte & 0x7f) > 37) {
        return Err(invalid(
            Some(record.data_offset + index),
            Some(record.box_type),
            "bits-per-component precision is reserved",
        ));
    }
    Ok(BitsPerComponentBox {
        components: bytes
            .iter()
            .map(|byte| ComponentSampleFormat::from_bpc_byte(*byte))
            .collect(),
    })
}

fn parse_color_specification(input: &[u8], record: &BoxRecord) -> Result<ColorSpecificationBox> {
    if record.data_len < 3 {
        return Err(invalid(
            Some(record.header_offset),
            Some(record.box_type),
            "colour specification box must contain method, precedence, and approximation",
        ));
    }

    let method_byte = input[record.data_offset];
    let method = match method_byte {
        1 => ColorSpecificationMethod::Enumerated,
        2 => ColorSpecificationMethod::RestrictedIccProfile,
        3 => ColorSpecificationMethod::AnyIccProfile,
        value => ColorSpecificationMethod::Vendor(value),
    };
    let enumerated_color_space = if method == ColorSpecificationMethod::Enumerated {
        if record.data_len != 7 {
            return Err(invalid(
                Some(record.header_offset),
                Some(record.box_type),
                "enumerated colour specification box must contain a four-byte colour space",
            ));
        }
        Some(match read_u32(input, record.data_offset + 3)? {
            16 => EnumeratedColorSpace::SRgb,
            17 => EnumeratedColorSpace::Greyscale,
            18 => EnumeratedColorSpace::SYcc,
            value => EnumeratedColorSpace::Unknown(value),
        })
    } else {
        None
    };

    Ok(ColorSpecificationBox {
        method,
        precedence: input[record.data_offset + 1],
        approximation: input[record.data_offset + 2],
        enumerated_color_space,
    })
}

fn preserve_metadata(input: &[u8], record: &BoxRecord) -> Result<MetadataBox> {
    Ok(MetadataBox {
        kind: match record.box_type {
            boxes::XML => MetadataBoxKind::Xml,
            boxes::UUID => MetadataBoxKind::Uuid,
            _ => MetadataBoxKind::Unknown,
        },
        box_type: record.box_type,
        data_offset: record.data_offset,
        data: checked_slice(input, record.data_offset, record.data_len)?.to_vec(),
    })
}

fn preserve_unknown_metadata(input: &[u8], record: &BoxRecord) -> Result<MetadataBox> {
    Ok(MetadataBox {
        kind: MetadataBoxKind::Unknown,
        box_type: record.box_type,
        data_offset: record.data_offset,
        data: checked_slice(input, record.data_offset, record.data_len)?.to_vec(),
    })
}

fn container_kind(file_type: &FileTypeBox) -> Result<ContainerKind> {
    if file_type.brand == boxes::BRAND_JP2
        || file_type.compatible_brands.contains(&boxes::BRAND_JP2)
    {
        return Ok(ContainerKind::Jp2);
    }
    if file_type.brand == boxes::BRAND_JPH
        || file_type.compatible_brands.contains(&boxes::BRAND_JPH)
    {
        return Ok(ContainerKind::Jph);
    }
    Err(ContainerError::Unsupported {
        offset: None,
        box_type: Some(boxes::FILE_TYPE),
        message: "file type box does not declare jp2 or jph compatibility".to_string(),
    })
}

fn validate_header_consistency(
    image_header: Option<ImageHeaderBox>,
    bits_per_component: Option<&BitsPerComponentBox>,
) -> Result<()> {
    let Some(image_header) = image_header else {
        return Ok(());
    };

    if image_header.compression_type != 7 {
        return Err(ContainerError::Unsupported {
            offset: None,
            box_type: Some(boxes::IMAGE_HEADER),
            message: "only JPEG 2000 compression type 7 is supported".to_string(),
        });
    }

    if let Some(bits_per_component) = bits_per_component
        && bits_per_component.components.len() != usize::from(image_header.components)
    {
        return Err(invalid(
            None,
            Some(boxes::BITS_PER_COMPONENT),
            "bits-per-component entry count must match image header component count",
        ));
    }

    Ok(())
}

fn validate_jp2_top_level_structure(top_level: &[BoxRecord]) -> Result<()> {
    let Some(first_codestream_index) = top_level
        .iter()
        .position(|record| record.box_type == boxes::CONTIGUOUS_CODESTREAM)
    else {
        let offset = top_level
            .iter()
            .find(|record| record.box_type == boxes::JP2_HEADER)
            .map(|record| record.header_offset);
        return Err(invalid(
            offset,
            Some(boxes::JP2_HEADER),
            "JP2 must contain a contiguous codestream box",
        ));
    };

    let header_indices = top_level
        .iter()
        .enumerate()
        .filter_map(|(index, record)| (record.box_type == boxes::JP2_HEADER).then_some(index))
        .collect::<Vec<_>>();
    let Some(&header_index) = header_indices.first() else {
        return Err(invalid(
            Some(top_level[first_codestream_index].header_offset),
            Some(boxes::CONTIGUOUS_CODESTREAM),
            "JP2 header box must precede the first contiguous codestream box",
        ));
    };
    if let Some(&duplicate_index) = header_indices.get(1) {
        let duplicate = &top_level[duplicate_index];
        return Err(invalid(
            Some(duplicate.header_offset),
            Some(duplicate.box_type),
            "JP2 must contain exactly one JP2 header box",
        ));
    }
    if header_index > first_codestream_index {
        let header = &top_level[header_index];
        return Err(invalid(
            Some(header.header_offset),
            Some(header.box_type),
            "JP2 header box must precede the first contiguous codestream box",
        ));
    }
    let file_type_index = top_level
        .iter()
        .position(|record| record.box_type == boxes::FILE_TYPE)
        .ok_or_else(|| invalid(None, None, "container must contain a file type box"))?;
    if header_index < file_type_index {
        let header = &top_level[header_index];
        return Err(invalid(
            Some(header.header_offset),
            Some(header.box_type),
            "JP2 header box must follow the file type box",
        ));
    }
    Ok(())
}

fn validate_jp2_header_structure(children: &[BoxRecord], header: &BoxRecord) -> Result<()> {
    let Some(first) = children.first() else {
        return Err(invalid(
            Some(header.header_offset),
            Some(header.box_type),
            "JP2 header box must begin with an image header box",
        ));
    };
    if first.box_type != boxes::IMAGE_HEADER {
        return Err(invalid(
            Some(first.header_offset),
            Some(first.box_type),
            "image header box must be first in the JP2 header box",
        ));
    }
    if let Some(duplicate) = children
        .iter()
        .skip(1)
        .find(|record| record.box_type == boxes::IMAGE_HEADER)
    {
        return Err(invalid(
            Some(duplicate.header_offset),
            Some(duplicate.box_type),
            "JP2 header box must contain exactly one image header box",
        ));
    }

    let bits_per_component = children
        .iter()
        .filter(|record| record.box_type == boxes::BITS_PER_COMPONENT)
        .collect::<Vec<_>>();
    if let Some(duplicate) = bits_per_component.get(1) {
        return Err(invalid(
            Some(duplicate.header_offset),
            Some(duplicate.box_type),
            "JP2 header box must contain at most one bits-per-component box",
        ));
    }

    let colour_indices = children
        .iter()
        .enumerate()
        .filter_map(|(index, record)| {
            (record.box_type == boxes::COLOR_SPECIFICATION).then_some(index)
        })
        .collect::<Vec<_>>();
    let Some(&first_colour) = colour_indices.first() else {
        return Err(invalid(
            Some(header.header_offset),
            Some(header.box_type),
            "JP2 header box must contain at least one colour specification box",
        ));
    };
    for (&previous, &current) in colour_indices.iter().zip(colour_indices.iter().skip(1)) {
        if current != previous + 1 {
            let record = &children[current];
            return Err(invalid(
                Some(record.header_offset),
                Some(record.box_type),
                "colour specification boxes must form one contiguous sequence",
            ));
        }
    }
    debug_assert!(first_colour > 0);
    Ok(())
}

fn validate_jp2_header_fields(
    image_header: Option<ImageHeaderBox>,
    bits_per_component: Option<&BitsPerComponentBox>,
    records: &[BoxRecord],
) -> Result<()> {
    let image_header = image_header.ok_or_else(|| {
        invalid(
            None,
            Some(boxes::IMAGE_HEADER),
            "JP2 header box must contain an image header box",
        )
    })?;
    let header_record = records
        .iter()
        .find(|record| record.box_type == boxes::JP2_HEADER)
        .ok_or(ContainerError::SizeOverflow)?;
    let bits_record = records.iter().find(|record| {
        record.box_type == boxes::BITS_PER_COMPONENT
            && record.header_offset >= header_record.data_offset
            && record.header_offset < header_record.data_offset + header_record.data_len
    });

    match (image_header.bits_per_component == 255, bits_per_component) {
        (true, None) => {
            return Err(invalid(
                Some(header_record.header_offset),
                Some(header_record.box_type),
                "varying component precision requires a bits-per-component box",
            ));
        }
        (false, Some(_)) => {
            return Err(invalid(
                bits_record.map(|record| record.header_offset),
                Some(boxes::BITS_PER_COMPONENT),
                "uniform component precision forbids a bits-per-component box",
            ));
        }
        _ => {}
    }

    if let Some(bits_per_component) = bits_per_component
        && bits_per_component.components.len() != usize::from(image_header.components)
    {
        return Err(invalid(
            bits_record.map(|record| record.header_offset),
            Some(boxes::BITS_PER_COMPONENT),
            "bits-per-component entry count must match image header component count",
        ));
    }
    Ok(())
}

fn checked_slice(input: &[u8], offset: usize, len: usize) -> Result<&[u8]> {
    let end = offset
        .checked_add(len)
        .ok_or(ContainerError::SizeOverflow)?;
    input
        .get(offset..end)
        .ok_or(ContainerError::TruncatedInput {
            offset,
            needed: len,
            remaining: input.len().saturating_sub(offset),
        })
}

fn require(input: &[u8], offset: usize, needed: usize) -> Result<()> {
    checked_slice(input, offset, needed).map(|_| ())
}

fn read_array(input: &[u8], offset: usize) -> Result<[u8; 4]> {
    checked_slice(input, offset, 4)?
        .try_into()
        .map_err(|_| ContainerError::SizeOverflow)
}

fn read_u16(input: &[u8], offset: usize) -> Result<u16> {
    Ok(u16::from_be_bytes(
        checked_slice(input, offset, 2)?
            .try_into()
            .map_err(|_| ContainerError::SizeOverflow)?,
    ))
}

fn read_u32(input: &[u8], offset: usize) -> Result<u32> {
    Ok(u32::from_be_bytes(
        checked_slice(input, offset, 4)?
            .try_into()
            .map_err(|_| ContainerError::SizeOverflow)?,
    ))
}

fn read_u64(input: &[u8], offset: usize) -> Result<u64> {
    Ok(u64::from_be_bytes(
        checked_slice(input, offset, 8)?
            .try_into()
            .map_err(|_| ContainerError::SizeOverflow)?,
    ))
}

fn invalid(
    offset: Option<usize>,
    box_type: Option<FourCc>,
    message: impl Into<String>,
) -> ContainerError {
    ContainerError::InvalidBox {
        offset,
        box_type,
        message: message.into(),
    }
}

impl BoxRecord {
    fn end_offset(&self) -> Result<usize> {
        self.data_offset
            .checked_add(self.data_len)
            .ok_or(ContainerError::SizeOverflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxed(box_type: FourCc, payload: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_box(&mut bytes, box_type, payload).unwrap();
        bytes
    }

    fn image_header(components: u16, bpc: u8) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&3_u32.to_be_bytes());
        payload.extend_from_slice(&5_u32.to_be_bytes());
        payload.extend_from_slice(&components.to_be_bytes());
        payload.extend_from_slice(&[bpc, 7, 0, 0]);
        boxed(boxes::IMAGE_HEADER, &payload)
    }

    fn colour() -> Vec<u8> {
        boxed(boxes::COLOR_SPECIFICATION, &[1, 0, 0, 0, 0, 0, 17])
    }

    fn jp2_header(children: &[Vec<u8>]) -> Vec<u8> {
        boxed(
            boxes::JP2_HEADER,
            &children.iter().flatten().copied().collect::<Vec<_>>(),
        )
    }

    fn file(kind: ContainerKind, top_level: &[Vec<u8>]) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_signature_box(&mut bytes).unwrap();
        write_file_type_box(&mut bytes, kind, 0, &[]).unwrap();
        bytes.extend(top_level.iter().flatten().copied());
        bytes
    }

    fn codestream_box() -> Vec<u8> {
        boxed(boxes::CONTIGUOUS_CODESTREAM, &[0xff, 0x4f, 0xff, 0xd9])
    }

    fn valid_uniform_file() -> Vec<u8> {
        file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[image_header(1, 7), colour()]),
                codestream_box(),
            ],
        )
    }

    fn box_offset(input: &[u8], box_type: FourCc, occurrence: usize) -> usize {
        input
            .windows(4)
            .enumerate()
            .filter(|(_, bytes)| *bytes == box_type.as_bytes())
            .map(|(offset, _)| offset - 4)
            .nth(occurrence)
            .unwrap()
    }

    #[test]
    fn accepts_uniform_and_varying_precision_jp2_headers() {
        let uniform = parse(&valid_uniform_file()).unwrap();
        assert_eq!(
            uniform.component_sample_formats().unwrap(),
            vec![ComponentSampleFormat {
                bits_per_sample: 8,
                signed: false,
            }]
        );

        let varying = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(2, 255),
                    boxed(boxes::BITS_PER_COMPONENT, &[7, 0x89]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        assert_eq!(
            parse(&varying).unwrap().component_sample_formats().unwrap(),
            vec![
                ComponentSampleFormat {
                    bits_per_sample: 8,
                    signed: false,
                },
                ComponentSampleFormat {
                    bits_per_sample: 10,
                    signed: true,
                },
            ]
        );
    }

    #[test]
    fn rejects_missing_duplicate_and_misordered_image_headers() {
        let missing = file(
            ContainerKind::Jp2,
            &[jp2_header(&[colour()]), codestream_box()],
        );
        assert!(matches!(
            parse(&missing),
            Err(ContainerError::InvalidBox {
                box_type: Some(boxes::COLOR_SPECIFICATION),
                ..
            })
        ));

        let duplicate = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[image_header(1, 7), colour(), image_header(1, 7)]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&duplicate, boxes::IMAGE_HEADER, 1);
        assert!(matches!(
            parse(&duplicate),
            Err(ContainerError::InvalidBox { offset: Some(offset), box_type: Some(boxes::IMAGE_HEADER), .. })
                if offset == expected
        ));

        let misordered = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[colour(), image_header(1, 7)]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&misordered, boxes::COLOR_SPECIFICATION, 0);
        assert!(matches!(
            parse(&misordered),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));
    }

    #[test]
    fn rejects_missing_late_and_duplicate_jp2_headers() {
        let codestream = codestream_box();
        let missing = file(ContainerKind::Jp2, core::slice::from_ref(&codestream));
        let expected = box_offset(&missing, boxes::CONTIGUOUS_CODESTREAM, 0);
        assert!(matches!(
            parse(&missing),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));

        let header = jp2_header(&[image_header(1, 7), colour()]);
        let late = file(ContainerKind::Jp2, &[codestream.clone(), header.clone()]);
        let expected = box_offset(&late, boxes::JP2_HEADER, 0);
        assert!(matches!(
            parse(&late),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));

        let duplicate = file(ContainerKind::Jp2, &[header.clone(), codestream, header]);
        let expected = box_offset(&duplicate, boxes::JP2_HEADER, 1);
        assert!(matches!(
            parse(&duplicate),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));

        let missing_codestream = file(
            ContainerKind::Jp2,
            &[jp2_header(&[image_header(1, 7), colour()])],
        );
        let expected = box_offset(&missing_codestream, boxes::JP2_HEADER, 0);
        assert!(matches!(
            parse(&missing_codestream),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));
    }

    #[test]
    fn rejects_absent_and_noncontiguous_colour_specifications() {
        let absent = file(
            ContainerKind::Jp2,
            &[jp2_header(&[image_header(1, 7)]), codestream_box()],
        );
        let expected = box_offset(&absent, boxes::JP2_HEADER, 0);
        assert!(matches!(
            parse(&absent),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));

        let interrupted = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(1, 7),
                    colour(),
                    boxed(FourCc::new(*b"free"), &[1]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&interrupted, boxes::COLOR_SPECIFICATION, 1);
        assert!(matches!(
            parse(&interrupted),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));
    }

    #[test]
    fn accepts_contiguous_colours_and_preserves_unknown_boxes() {
        let unknown = FourCc::new(*b"free");
        let input = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(1, 7),
                    boxed(unknown, &[1, 2, 3]),
                    colour(),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        let parsed = parse(&input).unwrap();
        assert!(parsed.metadata.iter().any(|record| {
            record.kind == MetadataBoxKind::Unknown
                && record.box_type == unknown
                && record.data == [1, 2, 3]
        }));
    }

    #[test]
    fn enforces_conditional_singular_bits_per_component() {
        let absent = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[image_header(2, 255), colour()]),
                codestream_box(),
            ],
        );
        assert!(matches!(
            parse(&absent),
            Err(ContainerError::InvalidBox { .. })
        ));

        let unexpected = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(1, 7),
                    boxed(boxes::BITS_PER_COMPONENT, &[7]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        assert!(matches!(
            parse(&unexpected),
            Err(ContainerError::InvalidBox { .. })
        ));

        let duplicate = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(1, 255),
                    boxed(boxes::BITS_PER_COMPONENT, &[7]),
                    boxed(boxes::BITS_PER_COMPONENT, &[7]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&duplicate, boxes::BITS_PER_COMPONENT, 1);
        assert!(matches!(
            parse(&duplicate),
            Err(ContainerError::InvalidBox { offset: Some(offset), .. }) if offset == expected
        ));
    }

    #[test]
    fn rejects_bits_per_component_count_and_reserved_entries() {
        let wrong_count = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(2, 255),
                    boxed(boxes::BITS_PER_COMPONENT, &[7]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        assert!(matches!(
            parse(&wrong_count),
            Err(ContainerError::InvalidBox { .. })
        ));

        let reserved = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[
                    image_header(2, 255),
                    boxed(boxes::BITS_PER_COMPONENT, &[7, 38]),
                    colour(),
                ]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&reserved, boxes::BITS_PER_COMPONENT, 0) + 9;
        assert!(matches!(
            parse(&reserved),
            Err(ContainerError::InvalidBox { offset: Some(offset), box_type: Some(boxes::BITS_PER_COMPONENT), .. })
                if offset == expected
        ));
    }

    #[test]
    fn rejects_reserved_image_header_values_at_exact_fields() {
        for (field, value) in [(10_usize, 38_u8), (11, 6), (12, 2), (13, 2)] {
            let mut header = image_header(1, 7);
            header[8 + field] = value;
            let input = file(
                ContainerKind::Jp2,
                &[jp2_header(&[header, colour()]), codestream_box()],
            );
            let expected = box_offset(&input, boxes::IMAGE_HEADER, 0) + 8 + field;
            assert!(matches!(
                parse(&input),
                Err(ContainerError::InvalidBox { offset: Some(offset), box_type: Some(boxes::IMAGE_HEADER), .. })
                    if offset == expected
            ));
        }

        let too_many_components = file(
            ContainerKind::Jp2,
            &[
                jp2_header(&[image_header(16_385, 7), colour()]),
                codestream_box(),
            ],
        );
        let expected = box_offset(&too_many_components, boxes::IMAGE_HEADER, 0) + 16;
        assert!(matches!(
            parse(&too_many_components),
            Err(ContainerError::InvalidBox { offset: Some(offset), box_type: Some(boxes::IMAGE_HEADER), .. })
                if offset == expected
        ));
    }

    #[test]
    fn reports_truncated_box_at_its_header_offset() {
        let mut input = valid_uniform_file();
        let expected = box_offset(&input, boxes::CONTIGUOUS_CODESTREAM, 0);
        input.pop();
        assert!(matches!(
            parse(&input),
            Err(ContainerError::TruncatedInput { offset, needed: 12, remaining: 11 })
                if offset == expected
        ));
    }

    #[test]
    fn leaves_jph_header_cardinality_behaviour_unchanged() {
        let input = file(ContainerKind::Jph, &[codestream_box()]);
        let parsed = parse(&input).unwrap();
        assert_eq!(parsed.kind, ContainerKind::Jph);
        assert!(parsed.image_header.is_none());
        assert_eq!(parsed.codestreams.len(), 1);
    }
}
