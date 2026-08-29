use emuella_j2k_core::{
    ColorModel, ComponentLayout, ComponentSelection, DecodeMode, DecodeOptions, ImageData,
    InputFormat, InspectOptions, PartialDecodeOptions, Region, ResolutionLevel, SampleEndian,
    SampleFormat, decode, decode_partial, inspect,
};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_INPUT_BYTES: u64 = 512 * 1024 * 1024;
const MAX_REFERENCE_BYTES: u64 = 512 * 1024 * 1024;
const MAX_COMPARISON_SAMPLES: u64 = 100_000_000;
const RENDERED_LIMIT_EXCEEDED: &str = "rendered samples exceed the comparison limit";

fn usage() -> &'static str {
    "usage:\n  emuella-j2k inspect INPUT\n  emuella-j2k compare-pgx INPUT REFERENCE --component N (--full-component|--output-window) --resolution-reduction N --output-origin-x N --output-origin-y N --width N --height N --bits-per-sample N (--signed|--unsigned) --peak-error-limit N --mean-squared-error-limit N\n  emuella-j2k compare-rendered-tiff-rgb INPUT REFERENCE --width N --height N --components 3 --peak-error-limit N"
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RenderedComparisonContract {
    width: u32,
    height: u32,
    components: u16,
    peak_error_limit: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RenderedErrorAggregates {
    samples: u64,
    peak_error: u64,
}

impl RenderedErrorAggregates {
    fn passes(self, contract: RenderedComparisonContract) -> bool {
        self.peak_error <= contract.peak_error_limit
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TiffEndian {
    Little,
    Big,
}

#[derive(Debug, PartialEq, Eq)]
struct TiffRgbImage {
    width: u32,
    height: u32,
    samples: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ComparisonContract {
    component: u16,
    output_window: bool,
    resolution_reduction: u8,
    output_origin_x: u32,
    output_origin_y: u32,
    width: u32,
    height: u32,
    bits_per_sample: u8,
    signed: bool,
    peak_error_limit: u64,
    mean_squared_error_limit: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ErrorAggregates {
    samples: u64,
    peak_error: u64,
    mean_squared_error: f64,
}

impl ErrorAggregates {
    fn passes(self, contract: ComparisonContract) -> bool {
        self.peak_error <= contract.peak_error_limit
            && self.mean_squared_error <= contract.mean_squared_error_limit
    }
}

#[derive(Debug, PartialEq)]
struct PgxImage {
    width: u32,
    height: u32,
    bits_per_sample: u8,
    signed: bool,
    samples: Vec<i64>,
}

fn parse_number<T>(value: &str, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    value
        .parse::<T>()
        .map_err(|_| format!("{label} is not a valid number"))
}

fn take_flag_value(arguments: &[String], index: &mut usize, flag: &str) -> Result<String, String> {
    *index += 1;
    arguments
        .get(*index)
        .cloned()
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn parse_comparison_arguments(
    arguments: Vec<OsString>,
) -> Result<(PathBuf, PathBuf, ComparisonContract), String> {
    if arguments.len() < 2 {
        return Err(usage().to_owned());
    }
    let input = PathBuf::from(&arguments[0]);
    let reference = PathBuf::from(&arguments[1]);
    let arguments = arguments[2..]
        .iter()
        .map(|argument| {
            argument
                .clone()
                .into_string()
                .map_err(|_| "comparison options must be valid UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut component = None;
    let mut output_window = None;
    let mut resolution_reduction = None;
    let mut output_origin_x = None;
    let mut output_origin_y = None;
    let mut width = None;
    let mut height = None;
    let mut bits_per_sample = None;
    let mut signed = None;
    let mut peak_error_limit = None;
    let mut mean_squared_error_limit = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--component" if component.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--component")?;
                component = Some(parse_number(&value, "component")?);
            }
            "--full-component" if output_window.is_none() => output_window = Some(false),
            "--output-window" if output_window.is_none() => output_window = Some(true),
            "--resolution-reduction" if resolution_reduction.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--resolution-reduction")?;
                resolution_reduction = Some(parse_number(&value, "resolution reduction")?);
            }
            "--output-origin-x" if output_origin_x.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--output-origin-x")?;
                output_origin_x = Some(parse_number(&value, "output origin x")?);
            }
            "--output-origin-y" if output_origin_y.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--output-origin-y")?;
                output_origin_y = Some(parse_number(&value, "output origin y")?);
            }
            "--width" if width.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--width")?;
                width = Some(parse_number(&value, "width")?);
            }
            "--height" if height.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--height")?;
                height = Some(parse_number(&value, "height")?);
            }
            "--bits-per-sample" if bits_per_sample.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--bits-per-sample")?;
                bits_per_sample = Some(parse_number(&value, "bits per sample")?);
            }
            "--signed" if signed.is_none() => signed = Some(true),
            "--unsigned" if signed.is_none() => signed = Some(false),
            "--peak-error-limit" if peak_error_limit.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--peak-error-limit")?;
                peak_error_limit = Some(parse_number(&value, "peak-error limit")?);
            }
            "--mean-squared-error-limit" if mean_squared_error_limit.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--mean-squared-error-limit")?;
                mean_squared_error_limit = Some(parse_number(&value, "mean-squared-error limit")?);
            }
            _ => return Err(usage().to_owned()),
        }
        index += 1;
    }
    let contract = ComparisonContract {
        component: component.ok_or_else(|| usage().to_owned())?,
        output_window: output_window.ok_or_else(|| usage().to_owned())?,
        resolution_reduction: resolution_reduction.ok_or_else(|| usage().to_owned())?,
        output_origin_x: output_origin_x.ok_or_else(|| usage().to_owned())?,
        output_origin_y: output_origin_y.ok_or_else(|| usage().to_owned())?,
        width: width.ok_or_else(|| usage().to_owned())?,
        height: height.ok_or_else(|| usage().to_owned())?,
        bits_per_sample: bits_per_sample.ok_or_else(|| usage().to_owned())?,
        signed: signed.ok_or_else(|| usage().to_owned())?,
        peak_error_limit: peak_error_limit.ok_or_else(|| usage().to_owned())?,
        mean_squared_error_limit: mean_squared_error_limit.ok_or_else(|| usage().to_owned())?,
    };
    validate_contract(contract)?;
    Ok((input, reference, contract))
}

fn parse_rendered_comparison_arguments(
    arguments: Vec<OsString>,
) -> Result<(PathBuf, PathBuf, RenderedComparisonContract), String> {
    if arguments.len() < 2 {
        return Err(usage().to_owned());
    }
    let input = PathBuf::from(&arguments[0]);
    let reference = PathBuf::from(&arguments[1]);
    let arguments = arguments[2..]
        .iter()
        .map(|argument| {
            argument
                .clone()
                .into_string()
                .map_err(|_| "rendered comparison options must be valid UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut width = None;
    let mut height = None;
    let mut components = None;
    let mut peak_error_limit = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--width" if width.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--width")?;
                width = Some(parse_number(&value, "width")?);
            }
            "--height" if height.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--height")?;
                height = Some(parse_number(&value, "height")?);
            }
            "--components" if components.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--components")?;
                components = Some(parse_number(&value, "components")?);
            }
            "--peak-error-limit" if peak_error_limit.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--peak-error-limit")?;
                peak_error_limit = Some(parse_number(&value, "peak-error limit")?);
            }
            _ => return Err(usage().to_owned()),
        }
        index += 1;
    }
    let contract = RenderedComparisonContract {
        width: width.ok_or_else(|| usage().to_owned())?,
        height: height.ok_or_else(|| usage().to_owned())?,
        components: components.ok_or_else(|| usage().to_owned())?,
        peak_error_limit: peak_error_limit.ok_or_else(|| usage().to_owned())?,
    };
    validate_rendered_contract(contract)?;
    Ok((input, reference, contract))
}

fn validate_rendered_contract(contract: RenderedComparisonContract) -> Result<(), String> {
    if contract.components != 3 {
        return Err("rendered comparison requires exactly three components".to_owned());
    }
    let samples = u64::from(contract.width)
        .checked_mul(u64::from(contract.height))
        .and_then(|pixels| pixels.checked_mul(u64::from(contract.components)))
        .ok_or_else(|| "rendered comparison dimensions overflow".to_owned())?;
    if samples == 0 || samples > MAX_COMPARISON_SAMPLES {
        return Err(
            "rendered comparison sample count is zero or exceeds the worker bound".to_owned(),
        );
    }
    if contract.peak_error_limit > u64::from(u8::MAX) {
        return Err("rendered peak-error limit exceeds the 8-bit sample range".to_owned());
    }
    Ok(())
}

fn validate_contract(contract: ComparisonContract) -> Result<(), String> {
    let samples = u64::from(contract.width)
        .checked_mul(u64::from(contract.height))
        .ok_or_else(|| "comparison dimensions overflow".to_owned())?;
    if samples == 0 || samples > MAX_COMPARISON_SAMPLES {
        return Err("comparison sample count is zero or exceeds the runner bound".to_owned());
    }
    let maximum_reduction = if contract.output_window { 1 } else { 5 };
    if contract.resolution_reduction > maximum_reduction {
        return Err(if contract.output_window {
            "output-window comparison resolution reduction must be zero or one".to_owned()
        } else {
            "full-component comparison resolution reduction must be between zero and five"
                .to_owned()
        });
    }
    if !contract.output_window && (contract.output_origin_x != 0 || contract.output_origin_y != 0) {
        return Err("full-component comparison cannot select an output origin".to_owned());
    }
    if contract.output_window {
        comparison_source_region(contract)?;
    }
    if !(1..=32).contains(&contract.bits_per_sample) {
        return Err("comparison precision must be in 1..=32".to_owned());
    }
    if !contract.mean_squared_error_limit.is_finite() || contract.mean_squared_error_limit < 0.0 {
        return Err("mean-squared-error limit must be finite and non-negative".to_owned());
    }
    Ok(())
}

fn comparison_source_region(contract: ComparisonContract) -> Result<Region, String> {
    let scale = 1_u32
        .checked_shl(u32::from(contract.resolution_reduction))
        .ok_or_else(|| "comparison resolution scale overflow".to_owned())?;
    Ok(Region {
        x: contract
            .output_origin_x
            .checked_mul(scale)
            .ok_or_else(|| "comparison output origin overflow".to_owned())?,
        y: contract
            .output_origin_y
            .checked_mul(scale)
            .ok_or_else(|| "comparison output origin overflow".to_owned())?,
        width: contract
            .width
            .checked_mul(scale)
            .ok_or_else(|| "comparison output dimensions overflow".to_owned())?,
        height: contract
            .height
            .checked_mul(scale)
            .ok_or_else(|| "comparison output dimensions overflow".to_owned())?,
    })
}

fn read_bounded(path: &Path, maximum: u64, label: &str) -> Result<Vec<u8>, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("failed to inspect {label} {}: {error}", path.display()))?;
    if !metadata.is_file() || metadata.len() > maximum {
        return Err(format!(
            "{label} is not a regular file within the runner size bound"
        ));
    }
    fs::read(path).map_err(|error| format!("failed to read {label} {}: {error}", path.display()))
}

fn read_bounded_scrubbed(path: &Path, maximum: u64, label: &str) -> Result<Vec<u8>, String> {
    let file = fs::File::open(path).map_err(|_| format!("cannot open {label}"))?;
    let metadata = file
        .metadata()
        .map_err(|_| format!("cannot inspect {label}"))?;
    if !metadata.is_file() || metadata.len() > maximum {
        return Err(format!(
            "{label} is not a regular file within the worker size bound"
        ));
    }
    let capacity = usize::try_from(metadata.len())
        .map_err(|_| format!("{label} size exceeds the host range"))?;
    let mut bytes = Vec::with_capacity(capacity);
    file.take(maximum.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|_| format!("cannot read {label}"))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > maximum {
        return Err(format!(
            "{label} exceeds the worker size bound while reading"
        ));
    }
    Ok(bytes)
}

fn parse_pgx(bytes: &[u8]) -> Result<PgxImage, String> {
    let newline = bytes
        .iter()
        .take(256)
        .position(|byte| *byte == b'\n')
        .ok_or_else(|| "PGX header is absent or exceeds 255 bytes".to_owned())?;
    let header = std::str::from_utf8(&bytes[..newline])
        .map_err(|_| "PGX header is not UTF-8 text".to_owned())?
        .trim_end_matches('\r');
    if header
        .bytes()
        .any(|byte| byte.is_ascii_whitespace() && byte != b' ')
    {
        return Err("PGX header fields must use spaces".to_owned());
    }
    let fields = header.split(' ').collect::<Vec<_>>();
    if !(5..=6).contains(&fields.len()) || fields[0] != "PG" {
        return Err("PGX header has an unsupported field structure".to_owned());
    }
    let little_endian = match fields[1] {
        "ML" => false,
        "LM" => true,
        _ => return Err("PGX byte order must be ML or LM".to_owned()),
    };
    let (depth_field, width_field, height_field) = match fields.as_slice() {
        [_, _, depth, width, height] => (*depth, *width, *height),
        [_, _, "", depth, width, height]
            if depth.as_bytes().first().is_some_and(u8::is_ascii_digit) =>
        {
            (*depth, *width, *height)
        }
        [_, _, sign @ ("+" | "-"), depth, width, height] => {
            let signed_depth = format!("{sign}{depth}");
            return parse_pgx_fields(bytes, newline, little_endian, &signed_depth, width, height);
        }
        _ => return Err("PGX header has an unsupported sign and precision form".to_owned()),
    };
    parse_pgx_fields(
        bytes,
        newline,
        little_endian,
        depth_field,
        width_field,
        height_field,
    )
}

fn parse_pgx_fields(
    bytes: &[u8],
    newline: usize,
    little_endian: bool,
    depth_field: &str,
    width_field: &str,
    height_field: &str,
) -> Result<PgxImage, String> {
    let (signed, depth_digits) = match depth_field.as_bytes().first() {
        Some(b'-') => (true, &depth_field[1..]),
        Some(b'+') => (false, &depth_field[1..]),
        Some(byte) if byte.is_ascii_digit() => (false, depth_field),
        _ => return Err("PGX precision has an unsupported sign form".to_owned()),
    };
    if depth_digits.is_empty() || !depth_digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("PGX precision must contain decimal digits".to_owned());
    }
    let bits_per_sample = parse_number::<u8>(depth_digits, "PGX bits per sample")?;
    let width = parse_number::<u32>(width_field, "PGX width")?;
    let height = parse_number::<u32>(height_field, "PGX height")?;
    if !(1..=32).contains(&bits_per_sample) || width == 0 || height == 0 {
        return Err("PGX dimensions or precision are outside the supported bounds".to_owned());
    }
    let sample_count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "PGX sample count overflow".to_owned())?;
    if sample_count > MAX_COMPARISON_SAMPLES {
        return Err("PGX sample count exceeds the runner bound".to_owned());
    }
    let bytes_per_sample = match bits_per_sample {
        1..=8 => 1,
        9..=16 => 2,
        17..=32 => 4,
        _ => unreachable!("PGX precision was validated"),
    };
    let payload_length = usize::try_from(sample_count)
        .ok()
        .and_then(|count| count.checked_mul(bytes_per_sample))
        .ok_or_else(|| "PGX payload length overflow".to_owned())?;
    let payload = &bytes[newline + 1..];
    if payload.len() != payload_length {
        return Err("PGX payload length disagrees with its header".to_owned());
    }
    let samples = payload
        .chunks_exact(bytes_per_sample)
        .map(|sample| logical_sample(sample, bits_per_sample, signed, little_endian))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PgxImage {
        width,
        height,
        bits_per_sample,
        signed,
        samples,
    })
}

fn logical_sample(
    bytes: &[u8],
    bits_per_sample: u8,
    signed: bool,
    little_endian: bool,
) -> Result<i64, String> {
    if bytes.is_empty() || bytes.len() > 4 || !(1..=32).contains(&bits_per_sample) {
        return Err("sample storage is outside the supported bounds".to_owned());
    }
    let mut raw = 0_u64;
    if little_endian {
        for (shift, byte) in bytes.iter().enumerate() {
            raw |= u64::from(*byte) << (shift * 8);
        }
    } else {
        for byte in bytes {
            raw = (raw << 8) | u64::from(*byte);
        }
    }
    let storage_bits = u8::try_from(bytes.len() * 8).expect("at most four sample bytes");
    if bits_per_sample < storage_bits {
        let extension_bits = storage_bits - bits_per_sample;
        let actual_extension = raw >> bits_per_sample;
        let sign_bit_set = raw & (1_u64 << (bits_per_sample - 1)) != 0;
        let expected_extension = if signed && sign_bit_set {
            (1_u64 << extension_bits) - 1
        } else {
            0
        };
        if actual_extension != expected_extension {
            return Err("sample storage does not extend its logical precision".to_owned());
        }
    }
    let mask = (1_u64 << bits_per_sample) - 1;
    raw &= mask;
    if signed && raw & (1_u64 << (bits_per_sample - 1)) != 0 {
        Ok(i64::try_from(raw).expect("32-bit sample fits i64") - (1_i64 << bits_per_sample))
    } else {
        i64::try_from(raw).map_err(|_| "sample value exceeds i64".to_owned())
    }
}

fn tiff_u16(bytes: &[u8], offset: usize, endian: TiffEndian) -> Result<u16, String> {
    let value = bytes
        .get(
            offset
                ..offset
                    .checked_add(2)
                    .ok_or_else(|| "TIFF offset overflow".to_owned())?,
        )
        .ok_or_else(|| "TIFF field is truncated".to_owned())?;
    Ok(match endian {
        TiffEndian::Little => u16::from_le_bytes([value[0], value[1]]),
        TiffEndian::Big => u16::from_be_bytes([value[0], value[1]]),
    })
}

fn tiff_u32(bytes: &[u8], offset: usize, endian: TiffEndian) -> Result<u32, String> {
    let value = bytes
        .get(
            offset
                ..offset
                    .checked_add(4)
                    .ok_or_else(|| "TIFF offset overflow".to_owned())?,
        )
        .ok_or_else(|| "TIFF field is truncated".to_owned())?;
    Ok(match endian {
        TiffEndian::Little => u32::from_le_bytes([value[0], value[1], value[2], value[3]]),
        TiffEndian::Big => u32::from_be_bytes([value[0], value[1], value[2], value[3]]),
    })
}

fn register_tiff_range(
    ranges: &mut Vec<(usize, usize)>,
    start: usize,
    length: usize,
    file_length: usize,
) -> Result<(), String> {
    if length == 0 {
        return Err("TIFF contains an empty referenced range".to_owned());
    }
    let end = start
        .checked_add(length)
        .ok_or_else(|| "TIFF referenced range overflows".to_owned())?;
    if end > file_length {
        return Err("TIFF referenced range is truncated".to_owned());
    }
    if ranges
        .iter()
        .any(|(other_start, other_end)| start < *other_end && *other_start < end)
    {
        return Err("TIFF referenced ranges overlap".to_owned());
    }
    ranges.push((start, end));
    Ok(())
}

fn tiff_type_size(field_type: u16) -> Result<usize, String> {
    match field_type {
        1 | 7 => Ok(1),
        3 => Ok(2),
        4 => Ok(4),
        5 => Ok(8),
        _ => Err("TIFF field uses an unsupported type".to_owned()),
    }
}

fn tiff_entry_data<'a>(
    bytes: &'a [u8],
    endian: TiffEndian,
    field_type: u16,
    count: u32,
    value_field_offset: usize,
    ranges: &mut Vec<(usize, usize)>,
) -> Result<&'a [u8], String> {
    if count == 0 {
        return Err("TIFF field has an empty value count".to_owned());
    }
    let length = usize::try_from(count)
        .ok()
        .and_then(|value_count| value_count.checked_mul(tiff_type_size(field_type).ok()?))
        .ok_or_else(|| "TIFF field length overflows".to_owned())?;
    let start = if length <= 4 {
        value_field_offset
    } else {
        usize::try_from(tiff_u32(bytes, value_field_offset, endian)?)
            .map_err(|_| "TIFF value offset exceeds the host range".to_owned())?
    };
    if length > 4 {
        register_tiff_range(ranges, start, length, bytes.len())?;
    }
    let end = start
        .checked_add(length)
        .ok_or_else(|| "TIFF field range overflows".to_owned())?;
    bytes
        .get(start..end)
        .ok_or_else(|| "TIFF field value is truncated".to_owned())
}

fn tiff_unsigned_values(
    bytes: &[u8],
    endian: TiffEndian,
    field_type: u16,
    count: u32,
    value_field_offset: usize,
    ranges: &mut Vec<(usize, usize)>,
) -> Result<Vec<u32>, String> {
    if !matches!(field_type, 3 | 4) {
        return Err("TIFF integer field has an unsupported type".to_owned());
    }
    let data = tiff_entry_data(bytes, endian, field_type, count, value_field_offset, ranges)?;
    let width = tiff_type_size(field_type)?;
    (0..usize::try_from(count).map_err(|_| "TIFF value count is too large".to_owned())?)
        .map(|index| {
            let offset = index
                .checked_mul(width)
                .ok_or_else(|| "TIFF value offset overflows".to_owned())?;
            if field_type == 3 {
                tiff_u16(data, offset, endian).map(u32::from)
            } else {
                tiff_u32(data, offset, endian)
            }
        })
        .collect()
}

fn tiff_scalar(
    bytes: &[u8],
    endian: TiffEndian,
    field_type: u16,
    count: u32,
    value_field_offset: usize,
    ranges: &mut Vec<(usize, usize)>,
) -> Result<u32, String> {
    if count != 1 {
        return Err("TIFF scalar field has a conflicting value count".to_owned());
    }
    tiff_unsigned_values(bytes, endian, field_type, count, value_field_offset, ranges)?
        .into_iter()
        .next()
        .ok_or_else(|| "TIFF scalar field has no value".to_owned())
}

fn validate_tiff_rational(
    bytes: &[u8],
    endian: TiffEndian,
    field_type: u16,
    count: u32,
    value_field_offset: usize,
    ranges: &mut Vec<(usize, usize)>,
) -> Result<(), String> {
    if field_type != 5 || count != 1 {
        return Err("TIFF resolution field has an unsupported shape".to_owned());
    }
    let data = tiff_entry_data(bytes, endian, field_type, count, value_field_offset, ranges)?;
    if tiff_u32(data, 4, endian)? == 0 {
        return Err("TIFF resolution denominator is zero".to_owned());
    }
    Ok(())
}

fn validate_tiff_inert_bytes(
    bytes: &[u8],
    endian: TiffEndian,
    field_type: u16,
    expected_type: u16,
    count: u32,
    value_field_offset: usize,
    ranges: &mut Vec<(usize, usize)>,
) -> Result<(), String> {
    if field_type != expected_type {
        return Err("TIFF inert metadata field has an unsupported type".to_owned());
    }
    tiff_entry_data(bytes, endian, field_type, count, value_field_offset, ranges)?;
    Ok(())
}

struct TiffLzwBitReader<'a> {
    bytes: &'a [u8],
    bit_offset: usize,
}

impl<'a> TiffLzwBitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            bit_offset: 0,
        }
    }

    fn read(&mut self, width: u8) -> Result<u16, String> {
        let end = self
            .bit_offset
            .checked_add(usize::from(width))
            .ok_or_else(|| "TIFF LZW bit offset overflows".to_owned())?;
        let available = self
            .bytes
            .len()
            .checked_mul(8)
            .ok_or_else(|| "TIFF LZW input length overflows".to_owned())?;
        if end > available {
            return Err("TIFF LZW code is truncated".to_owned());
        }
        let mut value = 0_u16;
        for bit in self.bit_offset..end {
            value = (value << 1) | u16::from((self.bytes[bit / 8] >> (7 - bit % 8)) & 1);
        }
        self.bit_offset = end;
        Ok(value)
    }

    fn require_zero_padding(self) -> Result<(), String> {
        let available = self
            .bytes
            .len()
            .checked_mul(8)
            .ok_or_else(|| "TIFF LZW input length overflows".to_owned())?;
        let remaining = available
            .checked_sub(self.bit_offset)
            .ok_or_else(|| "TIFF LZW bit accounting underflows".to_owned())?;
        if remaining >= 8 {
            return Err("TIFF LZW stream has trailing bytes".to_owned());
        }
        for bit in self.bit_offset..available {
            if (self.bytes[bit / 8] >> (7 - bit % 8)) & 1 != 0 {
                return Err("TIFF LZW stream has non-zero padding".to_owned());
            }
        }
        Ok(())
    }
}

fn expand_tiff_lzw_code(
    code: u16,
    next_code: u16,
    prefixes: &[u16; 4096],
    suffixes: &[u8; 4096],
    lengths: &[usize; 4096],
    scratch: &mut Vec<u8>,
    output_bound: usize,
) -> Result<u8, String> {
    if code >= next_code || code == 256 || code == 257 {
        return Err("TIFF LZW code is outside the current dictionary".to_owned());
    }
    let expected_length = if code < 256 {
        1
    } else {
        lengths[usize::from(code)]
    };
    if expected_length == 0 || expected_length > output_bound {
        return Err("TIFF LZW expansion exceeds its output bound".to_owned());
    }
    scratch.clear();
    scratch.reserve(expected_length);
    let mut current = code;
    let mut steps = 0_usize;
    while current >= 258 {
        if current >= next_code || steps >= 4096 {
            return Err("TIFF LZW dictionary chain is invalid".to_owned());
        }
        scratch.push(suffixes[usize::from(current)]);
        current = prefixes[usize::from(current)];
        steps += 1;
    }
    if current >= 256 {
        return Err("TIFF LZW dictionary chain reaches a control code".to_owned());
    }
    let first = u8::try_from(current).expect("literal LZW code fits u8");
    scratch.push(first);
    if scratch.len() != expected_length {
        return Err("TIFF LZW dictionary length is inconsistent".to_owned());
    }
    scratch.reverse();
    Ok(first)
}

fn decode_tiff_lzw(bytes: &[u8], expected_output: usize) -> Result<Vec<u8>, String> {
    if bytes.is_empty() || expected_output == 0 {
        return Err("TIFF LZW input or output bound is empty".to_owned());
    }
    let mut reader = TiffLzwBitReader::new(bytes);
    let mut prefixes = [0_u16; 4096];
    let mut suffixes = [0_u8; 4096];
    let mut lengths = [0_usize; 4096];
    let mut scratch = Vec::new();
    let mut output = Vec::with_capacity(expected_output);
    let mut next_code = 258_u16;
    let mut code_width = 9_u8;
    let mut previous = None;
    let mut require_literal_after_clear = true;
    if reader.read(code_width)? != 256 {
        return Err("TIFF LZW strip does not begin with a clear code".to_owned());
    }
    loop {
        let code = reader.read(code_width)?;
        match code {
            256 => {
                if require_literal_after_clear {
                    return Err("TIFF LZW strip repeats a clear code".to_owned());
                }
                next_code = 258;
                code_width = 9;
                previous = None;
                require_literal_after_clear = true;
            }
            257 => {
                if output.len() != expected_output {
                    return Err("TIFF LZW output length disagrees with its strip".to_owned());
                }
                reader.require_zero_padding()?;
                return Ok(output);
            }
            _ => {
                let remaining = expected_output
                    .checked_sub(output.len())
                    .ok_or_else(|| "TIFF LZW output length exceeds its bound".to_owned())?;
                let first = if code == next_code {
                    let previous_code = previous.ok_or_else(|| {
                        "TIFF LZW special code has no previous dictionary entry".to_owned()
                    })?;
                    let first = expand_tiff_lzw_code(
                        previous_code,
                        next_code,
                        &prefixes,
                        &suffixes,
                        &lengths,
                        &mut scratch,
                        remaining,
                    )?;
                    let expanded = scratch
                        .len()
                        .checked_add(1)
                        .ok_or_else(|| "TIFF LZW expansion length overflows".to_owned())?;
                    if expanded > remaining {
                        return Err("TIFF LZW expansion exceeds its output bound".to_owned());
                    }
                    output.extend_from_slice(&scratch);
                    output.push(first);
                    first
                } else {
                    let first = expand_tiff_lzw_code(
                        code,
                        next_code,
                        &prefixes,
                        &suffixes,
                        &lengths,
                        &mut scratch,
                        remaining,
                    )?;
                    if scratch.len() > remaining {
                        return Err("TIFF LZW expansion exceeds its output bound".to_owned());
                    }
                    output.extend_from_slice(&scratch);
                    first
                };
                if let Some(previous_code) = previous
                    && next_code < 4096
                {
                    let previous_length = if previous_code < 256 {
                        1
                    } else {
                        lengths[usize::from(previous_code)]
                    };
                    let new_length = previous_length
                        .checked_add(1)
                        .ok_or_else(|| "TIFF LZW dictionary length overflows".to_owned())?;
                    if new_length > expected_output {
                        return Err("TIFF LZW dictionary entry exceeds its output bound".to_owned());
                    }
                    prefixes[usize::from(next_code)] = previous_code;
                    suffixes[usize::from(next_code)] = first;
                    lengths[usize::from(next_code)] = new_length;
                    next_code += 1;
                    if code_width < 12 && next_code == (1_u16 << code_width) - 1 {
                        code_width += 1;
                    }
                }
                previous = Some(code);
                require_literal_after_clear = false;
            }
        }
    }
}

fn reverse_tiff_horizontal_predictor(samples: &mut [u8], row_bytes: usize) -> Result<(), String> {
    if row_bytes == 0 || !row_bytes.is_multiple_of(3) || !samples.len().is_multiple_of(row_bytes) {
        return Err("TIFF horizontal predictor geometry is inconsistent".to_owned());
    }
    for row in samples.chunks_exact_mut(row_bytes) {
        for index in 3..row.len() {
            row[index] = row[index].wrapping_add(row[index - 3]);
        }
    }
    Ok(())
}

fn parse_tiff_rgb_u8_contiguous(bytes: &[u8]) -> Result<TiffRgbImage, String> {
    if bytes.len() < 8 {
        return Err("TIFF header is truncated".to_owned());
    }
    let endian = match &bytes[..2] {
        b"II" => TiffEndian::Little,
        b"MM" => TiffEndian::Big,
        _ => return Err("TIFF byte-order marker is invalid".to_owned()),
    };
    if tiff_u16(bytes, 2, endian)? != 42 {
        return Err("TIFF classic-file marker is invalid".to_owned());
    }
    let ifd_offset = usize::try_from(tiff_u32(bytes, 4, endian)?)
        .map_err(|_| "TIFF IFD offset exceeds the host range".to_owned())?;
    let entry_count = usize::from(tiff_u16(bytes, ifd_offset, endian)?);
    if entry_count == 0 || entry_count > 64 {
        return Err("TIFF IFD entry count is outside the worker bound".to_owned());
    }
    let table_length = entry_count
        .checked_mul(12)
        .and_then(|entries| entries.checked_add(6))
        .ok_or_else(|| "TIFF IFD length overflows".to_owned())?;
    let mut ranges = Vec::new();
    register_tiff_range(&mut ranges, 0, 8, bytes.len())?;
    register_tiff_range(&mut ranges, ifd_offset, table_length, bytes.len())?;
    let next_ifd_offset = ifd_offset
        .checked_add(2)
        .and_then(|start| start.checked_add(entry_count.checked_mul(12)?))
        .ok_or_else(|| "TIFF next-IFD offset overflows".to_owned())?;
    if tiff_u32(bytes, next_ifd_offset, endian)? != 0 {
        return Err("TIFF contains an unsupported additional IFD".to_owned());
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut width = None;
    let mut height = None;
    let mut bits_per_sample = None;
    let mut compression = None;
    let mut photometric = None;
    let mut strip_offsets = None;
    let mut samples_per_pixel = None;
    let mut rows_per_strip = None;
    let mut strip_byte_counts = None;
    let mut planar_configuration = None;
    let mut predictor = None;
    for index in 0..entry_count {
        let entry = ifd_offset
            .checked_add(2)
            .and_then(|start| start.checked_add(index.checked_mul(12)?))
            .ok_or_else(|| "TIFF entry offset overflows".to_owned())?;
        let tag = tiff_u16(bytes, entry, endian)?;
        let field_type = tiff_u16(bytes, entry + 2, endian)?;
        let count = tiff_u32(bytes, entry + 4, endian)?;
        let value_field_offset = entry + 8;
        if !seen.insert(tag) {
            return Err("TIFF IFD repeats a tag".to_owned());
        }
        match tag {
            254 => {
                if tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )? != 0
                {
                    return Err("TIFF subfile type is unsupported".to_owned());
                }
            }
            256 => {
                width = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            257 => {
                height = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            258 => {
                if field_type != 3 || count != 3 {
                    return Err("TIFF BitsPerSample has an unsupported shape".to_owned());
                }
                bits_per_sample = Some(tiff_unsigned_values(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            259 => {
                compression = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            262 => {
                photometric = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            266 => {
                if tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )? != 1
                {
                    return Err("TIFF fill order is unsupported".to_owned());
                }
            }
            273 => {
                strip_offsets = Some(tiff_unsigned_values(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            274 => {
                if tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )? != 1
                {
                    return Err("TIFF orientation is unsupported".to_owned());
                }
            }
            277 => {
                samples_per_pixel = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            278 => {
                rows_per_strip = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            279 => {
                strip_byte_counts = Some(tiff_unsigned_values(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            282 | 283 => validate_tiff_rational(
                bytes,
                endian,
                field_type,
                count,
                value_field_offset,
                &mut ranges,
            )?,
            284 => {
                planar_configuration = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            296 => {
                let unit = tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?;
                if !(1..=3).contains(&unit) {
                    return Err("TIFF resolution unit is unsupported".to_owned());
                }
            }
            317 => {
                predictor = Some(tiff_scalar(
                    bytes,
                    endian,
                    field_type,
                    count,
                    value_field_offset,
                    &mut ranges,
                )?);
            }
            320 => return Err("TIFF palette colour is unsupported".to_owned()),
            338 => return Err("TIFF extra samples are unsupported".to_owned()),
            34377 => validate_tiff_inert_bytes(
                bytes,
                endian,
                field_type,
                1,
                count,
                value_field_offset,
                &mut ranges,
            )?,
            37724 => validate_tiff_inert_bytes(
                bytes,
                endian,
                field_type,
                7,
                count,
                value_field_offset,
                &mut ranges,
            )?,
            _ => return Err("TIFF IFD contains an unsupported tag".to_owned()),
        }
    }

    let width = width.ok_or_else(|| "TIFF lacks ImageWidth".to_owned())?;
    let height = height.ok_or_else(|| "TIFF lacks ImageLength".to_owned())?;
    if width == 0 || height == 0 {
        return Err("TIFF dimensions must be positive".to_owned());
    }
    if bits_per_sample.as_deref() != Some(&[8, 8, 8]) {
        return Err("TIFF requires three unsigned 8-bit samples".to_owned());
    }
    let compression = compression.ok_or_else(|| "TIFF lacks Compression".to_owned())?;
    match (compression, predictor) {
        (1, None | Some(1)) | (5, Some(2)) => {}
        (5, _) => {
            return Err("TIFF LZW compression requires horizontal Predictor 2".to_owned());
        }
        (1, Some(_)) => {
            return Err("TIFF uncompressed data has an unsupported predictor".to_owned());
        }
        _ => return Err("TIFF compression is unsupported".to_owned()),
    }
    if photometric != Some(2) {
        return Err("TIFF photometric interpretation is not RGB".to_owned());
    }
    if samples_per_pixel != Some(3) {
        return Err("TIFF SamplesPerPixel must be three".to_owned());
    }
    if planar_configuration != Some(1) {
        return Err("TIFF requires contiguous sample layout".to_owned());
    }
    let rows_per_strip = rows_per_strip
        .filter(|rows| *rows != 0)
        .ok_or_else(|| "TIFF RowsPerStrip is absent or zero".to_owned())?;
    let strip_offsets = strip_offsets.ok_or_else(|| "TIFF lacks StripOffsets".to_owned())?;
    let strip_byte_counts =
        strip_byte_counts.ok_or_else(|| "TIFF lacks StripByteCounts".to_owned())?;
    let expected_strips = height.div_ceil(rows_per_strip);
    let expected_strips = usize::try_from(expected_strips)
        .map_err(|_| "TIFF strip count exceeds the host range".to_owned())?;
    if strip_offsets.len() != expected_strips || strip_byte_counts.len() != expected_strips {
        return Err("TIFF strip count disagrees with RowsPerStrip".to_owned());
    }
    let row_bytes = u64::from(width)
        .checked_mul(3)
        .ok_or_else(|| "TIFF row length overflows".to_owned())?;
    let sample_count = row_bytes
        .checked_mul(u64::from(height))
        .ok_or_else(|| "TIFF sample count overflows".to_owned())?;
    if sample_count == 0 || sample_count > MAX_COMPARISON_SAMPLES {
        return Err("TIFF sample count exceeds the worker bound".to_owned());
    }
    let mut samples = Vec::with_capacity(
        usize::try_from(sample_count)
            .map_err(|_| "TIFF sample count exceeds the host range".to_owned())?,
    );
    for (index, (&offset, &byte_count)) in strip_offsets.iter().zip(&strip_byte_counts).enumerate()
    {
        let first_row = u32::try_from(index)
            .ok()
            .and_then(|strip| strip.checked_mul(rows_per_strip))
            .ok_or_else(|| "TIFF strip row offset overflows".to_owned())?;
        let rows = rows_per_strip.min(height - first_row);
        let expected_bytes = row_bytes
            .checked_mul(u64::from(rows))
            .ok_or_else(|| "TIFF strip byte count overflows".to_owned())?;
        let start = usize::try_from(offset)
            .map_err(|_| "TIFF strip offset exceeds the host range".to_owned())?;
        let length = usize::try_from(byte_count)
            .map_err(|_| "TIFF strip length exceeds the host range".to_owned())?;
        register_tiff_range(&mut ranges, start, length, bytes.len())?;
        let expected_bytes = usize::try_from(expected_bytes)
            .map_err(|_| "TIFF strip output length exceeds the host range".to_owned())?;
        if compression == 1 {
            if length != expected_bytes {
                return Err("TIFF strip contains missing or trailing sample bytes".to_owned());
            }
            samples.extend_from_slice(&bytes[start..start + length]);
        } else {
            let mut strip = decode_tiff_lzw(&bytes[start..start + length], expected_bytes)?;
            reverse_tiff_horizontal_predictor(
                &mut strip,
                usize::try_from(row_bytes)
                    .map_err(|_| "TIFF row length exceeds the host range".to_owned())?,
            )?;
            samples.extend_from_slice(&strip);
        }
    }
    if ranges.iter().map(|(_, end)| *end).max() != Some(bytes.len()) {
        return Err("TIFF contains an unreferenced trailing range".to_owned());
    }
    if samples.len()
        != usize::try_from(sample_count)
            .map_err(|_| "TIFF sample count exceeds the host range".to_owned())?
    {
        return Err("TIFF logical sample length is inconsistent".to_owned());
    }
    Ok(TiffRgbImage {
        width,
        height,
        samples,
    })
}

fn decoded_logical_samples(samples: &[u8], format: SampleFormat) -> Result<Vec<i64>, String> {
    let bytes_per_sample = usize::from(format.bits_per_sample).div_ceil(8);
    if !samples.len().is_multiple_of(bytes_per_sample) {
        return Err("decoded plane length is not sample-aligned".to_owned());
    }
    let little_endian = match format.byte_order {
        None => false,
        Some(SampleEndian::Little) => true,
        Some(SampleEndian::Big) => false,
    };
    samples
        .chunks_exact(bytes_per_sample)
        .map(|sample| logical_sample(sample, format.bits_per_sample, format.signed, little_endian))
        .collect()
}

/// Apply the arithmetic bit-depth scaling required by ISO/IEC 15444-4:2024,
/// B.2.3.1.5 after decode and clipping, without changing codec output.
fn scale_decoded_samples_to_reference_precision(
    samples: Vec<i64>,
    decoded_bits_per_sample: u8,
    reference_bits_per_sample: u8,
) -> Result<Vec<i64>, String> {
    let shift = decoded_bits_per_sample
        .checked_sub(reference_bits_per_sample)
        .ok_or_else(|| "decoded precision is lower than the comparison reference".to_owned())?;
    if shift == 0 {
        return Ok(samples);
    }
    Ok(samples.into_iter().map(|sample| sample >> shift).collect())
}

fn compare_samples(decoded: &[i64], reference: &[i64]) -> Result<ErrorAggregates, String> {
    if decoded.len() != reference.len() || decoded.is_empty() {
        return Err("decoded and reference sample counts differ or are empty".to_owned());
    }
    let mut peak_error = 0_u64;
    let mut squared_error_sum = 0_u128;
    for (actual, expected) in decoded.iter().zip(reference) {
        let difference = actual.abs_diff(*expected);
        peak_error = peak_error.max(difference);
        squared_error_sum = squared_error_sum
            .checked_add(u128::from(difference) * u128::from(difference))
            .ok_or_else(|| "squared-error aggregate overflow".to_owned())?;
    }
    let samples = u64::try_from(decoded.len()).map_err(|_| "sample count overflow".to_owned())?;
    Ok(ErrorAggregates {
        samples,
        peak_error,
        mean_squared_error: squared_error_sum as f64 / samples as f64,
    })
}

fn compare_j2k_to_pgx(
    codestream: &[u8],
    pgx_bytes: &[u8],
    contract: ComparisonContract,
) -> Result<ErrorAggregates, String> {
    validate_contract(contract)?;
    let reference = parse_pgx(pgx_bytes)?;
    if reference.width != contract.width
        || reference.height != contract.height
        || reference.bits_per_sample != contract.bits_per_sample
        || reference.signed != contract.signed
    {
        return Err("PGX metadata disagrees with the comparison contract".to_owned());
    }
    let decoded = if contract.output_window || contract.resolution_reduction != 0 {
        let options = PartialDecodeOptions {
            region: contract
                .output_window
                .then(|| comparison_source_region(contract))
                .transpose()?,
            resolution: if contract.resolution_reduction == 0 {
                ResolutionLevel::Full
            } else {
                ResolutionLevel::Reduced {
                    discard_levels: contract.resolution_reduction,
                }
            },
            components: ComponentSelection::Indices(vec![contract.component]),
            target_layout: ComponentLayout::Planar,
            ..PartialDecodeOptions::default()
        };
        decode_partial(codestream, &options)
    } else {
        let options = DecodeOptions {
            mode: DecodeMode::Components,
            requested_components: ComponentSelection::Indices(vec![contract.component]),
            target_layout: ComponentLayout::Planar,
            ..DecodeOptions::default()
        };
        decode(codestream, &options)
    };
    let decoded = decoded.map_err(|error| format!("decode failed: {error}"))?;
    if decoded.component_info.len() != 1
        || decoded.component_info[0].source_component != Some(contract.component)
        || decoded.component_info[0].width != contract.width
        || decoded.component_info[0].height != contract.height
        || decoded.component_info[0].sample_format.bits_per_sample < contract.bits_per_sample
        || decoded.component_info[0].sample_format.signed != contract.signed
    {
        return Err("decoded component metadata disagrees with the comparison contract".to_owned());
    }
    let planes = match decoded.data {
        ImageData::Planes(planes) if planes.len() == 1 => planes,
        _ => return Err("component decode did not produce exactly one planar buffer".to_owned()),
    };
    let decoded_format = decoded.component_info[0].sample_format;
    let samples = decoded_logical_samples(&planes[0], decoded_format)?;
    let samples = scale_decoded_samples_to_reference_precision(
        samples,
        decoded_format.bits_per_sample,
        contract.bits_per_sample,
    )?;
    compare_samples(&samples, &reference.samples)
}

fn compare_rendered_jp2_to_tiff(
    jp2: &[u8],
    tiff_bytes: &[u8],
    contract: RenderedComparisonContract,
) -> Result<RenderedErrorAggregates, String> {
    validate_rendered_contract(contract)?;
    let metadata = inspect(jp2, &InspectOptions::default())
        .map_err(|error| format!("input inspection failed: {error}"))?;
    if metadata.format != InputFormat::Jp2 {
        return Err("rendered comparison input is not JP2".to_owned());
    }
    let reference = parse_tiff_rgb_u8_contiguous(tiff_bytes)?;
    if reference.width != contract.width || reference.height != contract.height {
        return Err("TIFF metadata disagrees with the comparison contract".to_owned());
    }
    let decoded = decode(
        jp2,
        &DecodeOptions {
            allow_best_effort_backend_decode: false,
            mode: DecodeMode::Rendered,
            requested_components: ComponentSelection::All,
            max_quality_layers: None,
            target_layout: ComponentLayout::Interleaved,
        },
    )
    .map_err(|error| format!("rendered decode failed: {error}"))?;
    if decoded.info.width != contract.width
        || decoded.info.height != contract.height
        || decoded.info.components != contract.components
        || decoded.info.sample_format != SampleFormat::U8
        || decoded.info.color_model != ColorModel::Rgb
        || decoded.info.layout != ComponentLayout::Interleaved
        || decoded.component_info.len() != usize::from(contract.components)
        || decoded.component_info.iter().any(|component| {
            component.width != contract.width
                || component.height != contract.height
                || component.sample_format != SampleFormat::U8
                || component.source_component.is_some()
        })
    {
        return Err("rendered decode metadata disagrees with the comparison contract".to_owned());
    }
    let samples = match decoded.data {
        ImageData::Interleaved(samples) => samples,
        _ => return Err("rendered decode did not produce one interleaved buffer".to_owned()),
    };
    if samples.len() != reference.samples.len() || samples.is_empty() {
        return Err("rendered and reference sample counts differ or are empty".to_owned());
    }
    let peak_error = samples
        .iter()
        .zip(&reference.samples)
        .map(|(actual, expected)| u64::from(actual.abs_diff(*expected)))
        .max()
        .ok_or_else(|| "rendered comparison has no samples".to_owned())?;
    Ok(RenderedErrorAggregates {
        samples: u64::try_from(samples.len())
            .map_err(|_| "rendered sample count overflow".to_owned())?,
        peak_error,
    })
}

fn run_inspect(input: &Path) -> Result<(), String> {
    let bytes =
        fs::read(input).map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let metadata = inspect(&bytes, &InspectOptions::default())
        .map_err(|error| format!("failed to inspect {}: {error}", input.display()))?;
    println!("{metadata:#?}");
    Ok(())
}

fn run_compare(arguments: Vec<OsString>) -> Result<(), String> {
    let (input, reference, contract) = parse_comparison_arguments(arguments)?;
    let codestream = read_bounded(&input, MAX_INPUT_BYTES, "input")?;
    let pgx = read_bounded(&reference, MAX_REFERENCE_BYTES, "reference")?;
    let aggregates = compare_j2k_to_pgx(&codestream, &pgx, contract)?;
    let passed = aggregates.passes(contract);
    println!(
        "component={} width={} height={} samples={} peak_error={} mean_squared_error={:.17} peak_error_limit={} mean_squared_error_limit={:.17} passed={}",
        contract.component,
        contract.width,
        contract.height,
        aggregates.samples,
        aggregates.peak_error,
        aggregates.mean_squared_error,
        contract.peak_error_limit,
        contract.mean_squared_error_limit,
        passed
    );
    if passed {
        Ok(())
    } else {
        Err("decoded samples exceed the comparison limits".to_owned())
    }
}

fn run_rendered_compare(arguments: Vec<OsString>) -> Result<(), String> {
    let (input, reference, contract) = parse_rendered_comparison_arguments(arguments)?;
    let jp2 = read_bounded_scrubbed(&input, MAX_INPUT_BYTES, "input")?;
    let tiff = read_bounded_scrubbed(&reference, MAX_REFERENCE_BYTES, "reference")?;
    let aggregates = compare_rendered_jp2_to_tiff(&jp2, &tiff, contract)?;
    let passed = aggregates.passes(contract);
    println!(
        "components={} width={} height={} samples={} peak={} limit={} passed={}",
        contract.components,
        contract.width,
        contract.height,
        aggregates.samples,
        aggregates.peak_error,
        contract.peak_error_limit,
        passed
    );
    if passed {
        Ok(())
    } else {
        Err(RENDERED_LIMIT_EXCEEDED.to_owned())
    }
}

fn run_arguments(arguments: Vec<OsString>) -> Result<(), String> {
    let mut arguments = arguments.into_iter();
    let command = arguments.next().ok_or_else(|| usage().to_owned())?;
    let remaining = arguments.collect::<Vec<_>>();
    match command.to_str() {
        Some("inspect") if remaining.len() == 1 => run_inspect(Path::new(&remaining[0])),
        Some("compare-pgx") => run_compare(remaining),
        Some("compare-rendered-tiff-rgb") => run_rendered_compare(remaining),
        _ => Err(usage().to_owned()),
    }
}

fn run() -> Result<(), String> {
    run_arguments(env::args_os().skip(1).collect())
}

fn main() {
    if let Err(error) = run() {
        if error != RENDERED_LIMIT_EXCEEDED {
            eprintln!("emuella-j2k: {error}");
        }
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use emuella_j2k_core::{EncodeOptions, ImageInfo, ImageView, OutputFormat, encode};

    fn grayscale_gradient(width: u32, height: u32) -> Vec<u8> {
        (0..height)
            .flat_map(|y| {
                (0..width).map(move |x| {
                    ((x.wrapping_mul(11) + y.wrapping_mul(17) + x.wrapping_mul(y) * 5) & 0xff) as u8
                })
            })
            .collect()
    }

    fn generate_grayscale_j2k(width: u32, height: u32) -> Result<Vec<u8>, String> {
        let samples = grayscale_gradient(width, height);
        let info = ImageInfo::new(
            width,
            height,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Interleaved,
        )
        .map_err(|error| error.to_string())?;
        encode(
            ImageView::Interleaved {
                info: &info,
                samples: &samples,
                stride_bytes: width as usize,
            },
            &EncodeOptions {
                format: OutputFormat::J2kCodestream,
                ..EncodeOptions::default()
            },
        )
        .map_err(|error| error.to_string())
    }

    fn rgb_gradient(width: u32, height: u32) -> Vec<u8> {
        (0..height)
            .flat_map(|y| {
                (0..width).flat_map(move |x| {
                    [
                        ((x * 19 + y * 7) & 0xff) as u8,
                        ((x * 3 + y * 23 + 41) & 0xff) as u8,
                        ((x * 11 + y * 5 + 97) & 0xff) as u8,
                    ]
                })
            })
            .collect()
    }

    fn generate_rgb(width: u32, height: u32, format: OutputFormat) -> Result<Vec<u8>, String> {
        let samples = rgb_gradient(width, height);
        let info = ImageInfo::new(
            width,
            height,
            3,
            SampleFormat::U8,
            ColorModel::Rgb,
            ComponentLayout::Interleaved,
        )
        .map_err(|error| error.to_string())?;
        encode(
            ImageView::Interleaved {
                info: &info,
                samples: &samples,
                stride_bytes: width as usize * 3,
            },
            &EncodeOptions {
                format,
                ..EncodeOptions::default()
            },
        )
        .map_err(|error| error.to_string())
    }

    fn append_tiff_u16(bytes: &mut Vec<u8>, value: u16, endian: TiffEndian) {
        let encoded = match endian {
            TiffEndian::Little => value.to_le_bytes(),
            TiffEndian::Big => value.to_be_bytes(),
        };
        bytes.extend_from_slice(&encoded);
    }

    fn append_tiff_u32(bytes: &mut Vec<u8>, value: u32, endian: TiffEndian) {
        let encoded = match endian {
            TiffEndian::Little => value.to_le_bytes(),
            TiffEndian::Big => value.to_be_bytes(),
        };
        bytes.extend_from_slice(&encoded);
    }

    fn write_tiff_u16(bytes: &mut [u8], offset: usize, value: u16, endian: TiffEndian) {
        let value = match endian {
            TiffEndian::Little => value.to_le_bytes(),
            TiffEndian::Big => value.to_be_bytes(),
        };
        bytes[offset..offset + 2].copy_from_slice(&value);
    }

    fn write_tiff_u32(bytes: &mut [u8], offset: usize, value: u32, endian: TiffEndian) {
        let value = match endian {
            TiffEndian::Little => value.to_le_bytes(),
            TiffEndian::Big => value.to_be_bytes(),
        };
        bytes[offset..offset + 4].copy_from_slice(&value);
    }

    fn tiff_entry_offset(bytes: &[u8], endian: TiffEndian, wanted: u16) -> usize {
        let ifd = tiff_u32(bytes, 4, endian).unwrap() as usize;
        let count = tiff_u16(bytes, ifd, endian).unwrap() as usize;
        (0..count)
            .map(|index| ifd + 2 + index * 12)
            .find(|offset| tiff_u16(bytes, *offset, endian).unwrap() == wanted)
            .expect("synthetic TIFF contains requested tag")
    }

    fn tiff_rgb(
        width: u32,
        height: u32,
        samples: &[u8],
        endian: TiffEndian,
        rows_per_strip: u32,
        reverse_physical_strips: bool,
    ) -> Vec<u8> {
        assert_eq!(samples.len(), width as usize * height as usize * 3);
        assert!(rows_per_strip > 0);
        let strip_count = height.div_ceil(rows_per_strip) as usize;
        let ifd_offset = 8_u32;
        let ifd_length = 2 + 10 * 12 + 4;
        let bits_offset = ifd_offset as usize + ifd_length;
        let offsets_array_length = if strip_count > 1 { strip_count * 4 } else { 0 };
        let counts_array_length = if strip_count > 1 { strip_count * 4 } else { 0 };
        let offsets_offset = bits_offset + 6;
        let counts_offset = offsets_offset + offsets_array_length;
        let pixel_offset = counts_offset + counts_array_length;
        let row_bytes = width as usize * 3;
        let strip_lengths = (0..strip_count)
            .map(|index| {
                let first_row = index as u32 * rows_per_strip;
                rows_per_strip.min(height - first_row) as usize * row_bytes
            })
            .collect::<Vec<_>>();
        let physical_order = if reverse_physical_strips {
            (0..strip_count).rev().collect::<Vec<_>>()
        } else {
            (0..strip_count).collect::<Vec<_>>()
        };
        let mut strip_offsets = vec![0_u32; strip_count];
        let mut next_offset = pixel_offset;
        for &logical_index in &physical_order {
            strip_offsets[logical_index] = next_offset as u32;
            next_offset += strip_lengths[logical_index];
        }

        let mut bytes = Vec::with_capacity(next_offset);
        bytes.extend_from_slice(match endian {
            TiffEndian::Little => b"II",
            TiffEndian::Big => b"MM",
        });
        append_tiff_u16(&mut bytes, 42, endian);
        append_tiff_u32(&mut bytes, ifd_offset, endian);
        append_tiff_u16(&mut bytes, 10, endian);
        let mut entry = |tag: u16, field_type: u16, count: u32, value: u32| {
            append_tiff_u16(&mut bytes, tag, endian);
            append_tiff_u16(&mut bytes, field_type, endian);
            append_tiff_u32(&mut bytes, count, endian);
            if field_type == 3 && count == 1 {
                append_tiff_u16(&mut bytes, value as u16, endian);
                append_tiff_u16(&mut bytes, 0, endian);
            } else {
                append_tiff_u32(&mut bytes, value, endian);
            }
        };
        entry(256, 4, 1, width);
        entry(257, 4, 1, height);
        entry(258, 3, 3, bits_offset as u32);
        entry(259, 3, 1, 1);
        entry(262, 3, 1, 2);
        entry(
            273,
            4,
            strip_count as u32,
            if strip_count == 1 {
                strip_offsets[0]
            } else {
                offsets_offset as u32
            },
        );
        entry(277, 3, 1, 3);
        entry(278, 4, 1, rows_per_strip);
        entry(
            279,
            4,
            strip_count as u32,
            if strip_count == 1 {
                strip_lengths[0] as u32
            } else {
                counts_offset as u32
            },
        );
        entry(284, 3, 1, 1);
        append_tiff_u32(&mut bytes, 0, endian);
        for _ in 0..3 {
            append_tiff_u16(&mut bytes, 8, endian);
        }
        if strip_count > 1 {
            for &offset in &strip_offsets {
                append_tiff_u32(&mut bytes, offset, endian);
            }
            for &length in &strip_lengths {
                append_tiff_u32(&mut bytes, length as u32, endian);
            }
        }
        for logical_index in physical_order {
            let first_row = logical_index as u32 * rows_per_strip;
            let start = first_row as usize * row_bytes;
            bytes.extend_from_slice(&samples[start..start + strip_lengths[logical_index]]);
        }
        bytes
    }

    fn pack_tiff_lzw_codes(codes: &[u16]) -> (Vec<u8>, u8) {
        let mut bytes = Vec::new();
        let mut current = 0_u8;
        let mut used = 0_u8;
        let mut width = 9_u8;
        let mut maximum_width = width;
        let mut next_code = 258_u16;
        let mut previous = None;
        for &code in codes {
            assert!(code < (1_u16 << width));
            for shift in (0..width).rev() {
                current = (current << 1) | ((code >> shift) & 1) as u8;
                used += 1;
                if used == 8 {
                    bytes.push(current);
                    current = 0;
                    used = 0;
                }
            }
            match code {
                256 => {
                    width = 9;
                    next_code = 258;
                    previous = None;
                }
                257 => {}
                _ => {
                    if previous.is_some() && next_code < 4096 {
                        next_code += 1;
                        if width < 12 && next_code == (1_u16 << width) - 1 {
                            width += 1;
                            maximum_width = maximum_width.max(width);
                        }
                    }
                    previous = Some(code);
                }
            }
        }
        if used != 0 {
            bytes.push(current << (8 - used));
        }
        (bytes, maximum_width)
    }

    fn encode_tiff_lzw(samples: &[u8], reset_at: Option<u16>) -> (Vec<u8>, u8, usize) {
        assert!(!samples.is_empty());
        let mut dictionary = std::collections::BTreeMap::<Vec<u8>, u16>::new();
        let initialise = |dictionary: &mut std::collections::BTreeMap<Vec<u8>, u16>| {
            dictionary.clear();
            for value in 0_u16..=255 {
                dictionary.insert(vec![value as u8], value);
            }
        };
        initialise(&mut dictionary);
        let mut codes = vec![256];
        let mut clears = 1;
        let mut next_code = 258_u16;
        let mut current = vec![samples[0]];
        for &sample in &samples[1..] {
            let mut extended = current.clone();
            extended.push(sample);
            if dictionary.contains_key(&extended) {
                current = extended;
                continue;
            }
            codes.push(dictionary[&current]);
            if reset_at.is_some_and(|limit| next_code >= limit) || next_code >= 4096 {
                codes.push(256);
                clears += 1;
                initialise(&mut dictionary);
                next_code = 258;
            } else {
                dictionary.insert(extended, next_code);
                next_code += 1;
            }
            current = vec![sample];
        }
        codes.push(dictionary[&current]);
        codes.push(257);
        let (bytes, maximum_width) = pack_tiff_lzw_codes(&codes);
        (bytes, maximum_width, clears)
    }

    fn apply_horizontal_predictor(samples: &[u8], row_bytes: usize) -> Vec<u8> {
        let mut predicted = samples.to_vec();
        for row in predicted.chunks_exact_mut(row_bytes) {
            for index in (3..row.len()).rev() {
                row[index] = row[index].wrapping_sub(row[index - 3]);
            }
        }
        predicted
    }

    fn tiff_rgb_lzw(
        width: u32,
        height: u32,
        samples: &[u8],
        endian: TiffEndian,
        rows_per_strip: u32,
        reverse_physical_strips: bool,
        reset_at: Option<u16>,
    ) -> (Vec<u8>, u8, usize) {
        assert_eq!(samples.len(), width as usize * height as usize * 3);
        let strip_count = height.div_ceil(rows_per_strip) as usize;
        let row_bytes = width as usize * 3;
        let mut maximum_width = 9_u8;
        let mut clear_count = 0_usize;
        let compressed = (0..strip_count)
            .map(|index| {
                let first_row = index as u32 * rows_per_strip;
                let rows = rows_per_strip.min(height - first_row) as usize;
                let start = first_row as usize * row_bytes;
                let predicted = apply_horizontal_predictor(
                    &samples[start..start + rows * row_bytes],
                    row_bytes,
                );
                let (encoded, width, clears) = encode_tiff_lzw(&predicted, reset_at);
                maximum_width = maximum_width.max(width);
                clear_count += clears;
                encoded
            })
            .collect::<Vec<_>>();

        let ifd_offset = 8_u32;
        let ifd_length = 2 + 13 * 12 + 4;
        let bits_offset = ifd_offset as usize + ifd_length;
        let offsets_array_length = if strip_count > 1 { strip_count * 4 } else { 0 };
        let counts_array_length = if strip_count > 1 { strip_count * 4 } else { 0 };
        let offsets_offset = bits_offset + 6;
        let counts_offset = offsets_offset + offsets_array_length;
        let photoshop_offset = counts_offset + counts_array_length;
        let photoshop = b"photo";
        let document_offset = photoshop_offset + photoshop.len();
        let document = b"docblk";
        let pixel_offset = document_offset + document.len();
        let physical_order = if reverse_physical_strips {
            (0..strip_count).rev().collect::<Vec<_>>()
        } else {
            (0..strip_count).collect::<Vec<_>>()
        };
        let mut strip_offsets = vec![0_u32; strip_count];
        let mut next_offset = pixel_offset;
        for &logical_index in &physical_order {
            strip_offsets[logical_index] = next_offset as u32;
            next_offset += compressed[logical_index].len();
        }

        let mut bytes = Vec::with_capacity(next_offset);
        bytes.extend_from_slice(match endian {
            TiffEndian::Little => b"II",
            TiffEndian::Big => b"MM",
        });
        append_tiff_u16(&mut bytes, 42, endian);
        append_tiff_u32(&mut bytes, ifd_offset, endian);
        append_tiff_u16(&mut bytes, 13, endian);
        let mut entry = |tag: u16, field_type: u16, count: u32, value: u32| {
            append_tiff_u16(&mut bytes, tag, endian);
            append_tiff_u16(&mut bytes, field_type, endian);
            append_tiff_u32(&mut bytes, count, endian);
            if field_type == 3 && count == 1 {
                append_tiff_u16(&mut bytes, value as u16, endian);
                append_tiff_u16(&mut bytes, 0, endian);
            } else {
                append_tiff_u32(&mut bytes, value, endian);
            }
        };
        entry(256, 4, 1, width);
        entry(257, 4, 1, height);
        entry(258, 3, 3, bits_offset as u32);
        entry(259, 3, 1, 5);
        entry(262, 3, 1, 2);
        entry(
            273,
            4,
            strip_count as u32,
            if strip_count == 1 {
                strip_offsets[0]
            } else {
                offsets_offset as u32
            },
        );
        entry(277, 3, 1, 3);
        entry(278, 4, 1, rows_per_strip);
        entry(
            279,
            4,
            strip_count as u32,
            if strip_count == 1 {
                compressed[0].len() as u32
            } else {
                counts_offset as u32
            },
        );
        entry(284, 3, 1, 1);
        entry(317, 3, 1, 2);
        entry(34377, 1, photoshop.len() as u32, photoshop_offset as u32);
        entry(37724, 7, document.len() as u32, document_offset as u32);
        append_tiff_u32(&mut bytes, 0, endian);
        for _ in 0..3 {
            append_tiff_u16(&mut bytes, 8, endian);
        }
        if strip_count > 1 {
            for &offset in &strip_offsets {
                append_tiff_u32(&mut bytes, offset, endian);
            }
            for strip in &compressed {
                append_tiff_u32(&mut bytes, strip.len() as u32, endian);
            }
        }
        bytes.extend_from_slice(photoshop);
        bytes.extend_from_slice(document);
        for logical_index in physical_order {
            bytes.extend_from_slice(&compressed[logical_index]);
        }
        (bytes, maximum_width, clear_count)
    }

    fn rendered_contract(width: u32, height: u32, limit: u64) -> RenderedComparisonContract {
        RenderedComparisonContract {
            width,
            height,
            components: 3,
            peak_error_limit: limit,
        }
    }

    fn pgx(width: u32, height: u32, samples: &[u8]) -> Vec<u8> {
        let mut bytes = format!("PG ML + 8 {width} {height}\n").into_bytes();
        bytes.extend_from_slice(samples);
        bytes
    }

    fn contract(width: u32, height: u32) -> ComparisonContract {
        ComparisonContract {
            component: 0,
            output_window: false,
            resolution_reduction: 0,
            output_origin_x: 0,
            output_origin_y: 0,
            width,
            height,
            bits_per_sample: 8,
            signed: false,
            peak_error_limit: 0,
            mean_squared_error_limit: 0.0,
        }
    }

    #[test]
    fn parses_pgx_byte_order_and_signed_samples() {
        let parsed =
            parse_pgx(b"PG LM -12 2 1\n\xff\xff\x00\xf8").expect("project-authored PGX parses");
        assert_eq!(parsed.width, 2);
        assert_eq!(parsed.height, 1);
        assert_eq!(parsed.bits_per_sample, 12);
        assert!(parsed.signed);
        assert_eq!(parsed.samples, [-1, -2048]);

        let spaced =
            parse_pgx(b"PG ML - 4 2 1\n\xff\xf8").expect("separated project-authored sign parses");
        assert_eq!(spaced.samples, [-1, -8]);

        let joined_unsigned =
            parse_pgx(b"PG ML +4 1 1\r\n\x07").expect("joined unsigned precision parses");
        assert_eq!(joined_unsigned.samples, [7]);
        let unsigned_without_sign =
            parse_pgx(b"PG ML 4 1 1\n\x07").expect("unsigned precision parses");
        assert_eq!(unsigned_without_sign.samples, [7]);
        let blank_separated_sign =
            parse_pgx(b"PG ML  8 1 1\n\x07").expect("blank sign position parses as unsigned");
        assert_eq!(blank_separated_sign.samples, [7]);

        parse_pgx(b"PG ML +17 1 1\n\x00\x00\x00\x01").expect("17-bit PGX uses four-byte storage");
    }

    #[test]
    fn scales_decoded_precision_with_arithmetic_shifts() {
        assert_eq!(
            scale_decoded_samples_to_reference_precision(vec![0, 15, 16, 4095], 12, 8).unwrap(),
            [0, 0, 1, 255]
        );
        assert_eq!(
            scale_decoded_samples_to_reference_precision(vec![-16, -1, 0, 15], 12, 8).unwrap(),
            [-1, -1, 0, 0]
        );
        assert!(scale_decoded_samples_to_reference_precision(vec![0], 8, 12).is_err());
    }

    #[test]
    fn compares_project_authored_decode_with_pgx_in_memory() {
        let width = 4;
        let height = 3;
        let samples = grayscale_gradient(width, height);
        let codestream = generate_grayscale_j2k(width, height).expect("synthetic J2K encodes");
        let exact = compare_j2k_to_pgx(
            &codestream,
            &pgx(width, height, &samples),
            contract(width, height),
        )
        .expect("synthetic comparison succeeds");
        assert_eq!(exact.peak_error, 0);
        assert_eq!(exact.mean_squared_error, 0.0);

        let mut changed = samples;
        changed[0] = changed[0].saturating_add(1);
        let mismatch = compare_j2k_to_pgx(
            &codestream,
            &pgx(width, height, &changed),
            contract(width, height),
        )
        .expect("synthetic mismatch produces aggregates");
        assert_eq!(mismatch.peak_error, 1);
        assert_eq!(mismatch.mean_squared_error, 1.0 / 12.0);
    }

    #[test]
    fn parses_little_and_big_endian_tiff_with_odd_multi_strip_geometry() {
        let width = 3;
        let height = 5;
        let samples = rgb_gradient(width, height);
        for endian in [TiffEndian::Little, TiffEndian::Big] {
            let parsed =
                parse_tiff_rgb_u8_contiguous(&tiff_rgb(width, height, &samples, endian, 2, true))
                    .expect("project-authored multi-strip TIFF parses");
            assert_eq!(parsed.width, width);
            assert_eq!(parsed.height, height);
            assert_eq!(parsed.samples, samples);
        }
    }

    #[test]
    fn decodes_tiff_lzw_controls_width_transitions_and_dictionary_resets() {
        let (special, _) = pack_tiff_lzw_codes(&[256, 65, 258, 257]);
        assert_eq!(decode_tiff_lzw(&special, 3).unwrap(), b"AAA");

        let mut seed = 0x1234_5678_u32;
        let varied = (0..2_000)
            .map(|_| {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (seed >> 24) as u8
            })
            .collect::<Vec<_>>();
        let (transitioned, maximum_width, clears) = encode_tiff_lzw(&varied, None);
        assert_eq!(
            maximum_width, 12,
            "fixture must cross every TIFF LZW code-width boundary"
        );
        assert_eq!(clears, 1);
        assert_eq!(
            decode_tiff_lzw(&transitioned, varied.len()).unwrap(),
            varied
        );

        let (reset, reset_width, clears) = encode_tiff_lzw(&varied, Some(300));
        assert!(reset_width >= 9);
        assert!(
            clears > 2,
            "fixture must exercise repeated dictionary reset"
        );
        assert_eq!(decode_tiff_lzw(&reset, varied.len()).unwrap(), varied);
    }

    #[test]
    fn rejects_invalid_truncated_and_over_bound_tiff_lzw_streams() {
        for codes in [[256, 258, 257].as_slice(), [256, 65, 300, 257].as_slice()] {
            let (encoded, _) = pack_tiff_lzw_codes(codes);
            assert!(decode_tiff_lzw(&encoded, 1).is_err());
        }

        let (valid, _) = pack_tiff_lzw_codes(&[256, 65, 258, 257]);
        assert!(decode_tiff_lzw(&valid, 2).is_err());
        let mut truncated = valid.clone();
        truncated.pop();
        assert!(decode_tiff_lzw(&truncated, 3).is_err());
        let mut trailing = valid.clone();
        trailing.push(0);
        assert!(decode_tiff_lzw(&trailing, 3).is_err());
        let mut non_zero_padding = valid;
        let last = non_zero_padding.last_mut().unwrap();
        *last |= 1;
        assert!(decode_tiff_lzw(&non_zero_padding, 3).is_err());

        let (reset, _) = pack_tiff_lzw_codes(&[256, 65, 66, 256, 67, 68, 257]);
        assert_eq!(decode_tiff_lzw(&reset, 4).unwrap(), b"ABCD");
    }

    #[test]
    fn parses_lzw_predictor_strips_and_inert_adobe_metadata() {
        let width = 5;
        let height = 7;
        let samples = rgb_gradient(width, height);
        for endian in [TiffEndian::Little, TiffEndian::Big] {
            let (tiff, _, clears) =
                tiff_rgb_lzw(width, height, &samples, endian, 2, true, Some(270));
            assert!(clears > height.div_ceil(2) as usize);
            let parsed = parse_tiff_rgb_u8_contiguous(&tiff)
                .expect("project-authored LZW predictor TIFF parses");
            assert_eq!(parsed.width, width);
            assert_eq!(parsed.height, height);
            assert_eq!(parsed.samples, samples);
        }
    }

    #[test]
    fn compares_project_authored_rendered_jp2_with_lzw_tiff() {
        let width = 7;
        let height = 5;
        let samples = rgb_gradient(width, height);
        let jp2 = generate_rgb(width, height, OutputFormat::Jp2).expect("synthetic JP2 encodes");
        let (tiff, _, _) = tiff_rgb_lzw(width, height, &samples, TiffEndian::Little, 2, true, None);
        let aggregates =
            compare_rendered_jp2_to_tiff(&jp2, &tiff, rendered_contract(width, height, 0))
                .expect("synthetic LZW rendered comparison succeeds");
        assert_eq!(aggregates.peak_error, 0);
        assert_eq!(aggregates.samples, u64::from(width) * u64::from(height) * 3);
    }

    #[test]
    fn compares_project_authored_rendered_jp2_and_tiff_in_memory() {
        let width = 3;
        let height = 5;
        let samples = rgb_gradient(width, height);
        let jp2 = generate_rgb(width, height, OutputFormat::Jp2).expect("synthetic JP2 encodes");
        let exact = compare_rendered_jp2_to_tiff(
            &jp2,
            &tiff_rgb(width, height, &samples, TiffEndian::Little, 2, false),
            rendered_contract(width, height, 0),
        )
        .expect("synthetic rendered comparison succeeds");
        assert_eq!(exact.samples, u64::from(width) * u64::from(height) * 3);
        assert_eq!(exact.peak_error, 0);

        let mut changed = samples;
        changed[7] = changed[7].saturating_add(4);
        let mismatch = compare_rendered_jp2_to_tiff(
            &jp2,
            &tiff_rgb(width, height, &changed, TiffEndian::Big, height, false),
            rendered_contract(width, height, 4),
        )
        .expect("synthetic rendered mismatch produces aggregates");
        assert_eq!(mismatch.peak_error, 4);
        assert!(mismatch.passes(rendered_contract(width, height, 4)));
        assert!(!mismatch.passes(rendered_contract(width, height, 3)));
    }

    #[test]
    fn rendered_worker_requires_the_jp2_rendered_route_and_declared_shape() {
        let width = 2;
        let height = 3;
        let samples = rgb_gradient(width, height);
        let tiff = tiff_rgb(width, height, &samples, TiffEndian::Little, height, false);
        let jp2 = generate_rgb(width, height, OutputFormat::Jp2).expect("synthetic JP2 encodes");
        compare_rendered_jp2_to_tiff(&jp2, &tiff, rendered_contract(width, height, 0))
            .expect("JP2 rendered route succeeds");
        let j2k = generate_rgb(width, height, OutputFormat::J2kCodestream)
            .expect("synthetic J2K encodes");
        assert!(
            compare_rendered_jp2_to_tiff(&j2k, &tiff, rendered_contract(width, height, 0))
                .unwrap_err()
                .contains("not JP2")
        );
        assert!(
            compare_rendered_jp2_to_tiff(&jp2, &tiff, rendered_contract(width + 1, height, 0))
                .unwrap_err()
                .contains("TIFF metadata")
        );
        assert!(
            validate_rendered_contract(RenderedComparisonContract {
                components: 4,
                ..rendered_contract(width, height, 0)
            })
            .is_err()
        );
        assert!(validate_rendered_contract(rendered_contract(width, height, 256)).is_err());
    }

    #[test]
    fn rejects_malformed_tiff_headers_ifds_and_ranges() {
        let endian = TiffEndian::Little;
        let samples = rgb_gradient(2, 3);
        let valid = tiff_rgb(2, 3, &samples, endian, 2, false);

        for malformed in [Vec::new(), b"ZZ\x2a\0\x08\0\0\0".to_vec()] {
            assert!(parse_tiff_rgb_u8_contiguous(&malformed).is_err());
        }
        let mut bad_magic = valid.clone();
        write_tiff_u16(&mut bad_magic, 2, 43, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&bad_magic).is_err());

        let mut duplicate = valid.clone();
        let planar = tiff_entry_offset(&duplicate, endian, 284);
        write_tiff_u16(&mut duplicate, planar, 256, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&duplicate).is_err());

        let mut additional_ifd = valid.clone();
        let ifd = tiff_u32(&additional_ifd, 4, endian).unwrap() as usize;
        let count = tiff_u16(&additional_ifd, ifd, endian).unwrap() as usize;
        write_tiff_u32(&mut additional_ifd, ifd + 2 + count * 12, 8, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&additional_ifd).is_err());

        let mut bad_type = valid.clone();
        let width_entry = tiff_entry_offset(&bad_type, endian, 256);
        write_tiff_u16(&mut bad_type, width_entry + 2, 5, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&bad_type).is_err());

        let mut bad_count = valid.clone();
        let bits = tiff_entry_offset(&bad_count, endian, 258);
        write_tiff_u32(&mut bad_count, bits + 4, 2, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&bad_count).is_err());

        let mut bad_offset = valid.clone();
        let bits = tiff_entry_offset(&bad_offset, endian, 258);
        write_tiff_u32(&mut bad_offset, bits + 8, u32::MAX, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&bad_offset).is_err());

        let mut duplicate_strips = valid.clone();
        let offsets_entry = tiff_entry_offset(&duplicate_strips, endian, 273);
        let offsets = tiff_u32(&duplicate_strips, offsets_entry + 8, endian).unwrap() as usize;
        let first_offset = tiff_u32(&duplicate_strips, offsets, endian).unwrap();
        write_tiff_u32(&mut duplicate_strips, offsets + 4, first_offset, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&duplicate_strips).is_err());

        let mut trailing_strip = valid.clone();
        let counts_entry = tiff_entry_offset(&trailing_strip, endian, 279);
        let counts = tiff_u32(&trailing_strip, counts_entry + 8, endian).unwrap() as usize;
        let first_count = tiff_u32(&trailing_strip, counts, endian).unwrap();
        write_tiff_u32(&mut trailing_strip, counts, first_count + 1, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&trailing_strip).is_err());

        let mut truncated = valid.clone();
        truncated.pop();
        assert!(parse_tiff_rgb_u8_contiguous(&truncated).is_err());
        let mut overflow = valid.clone();
        let width_entry = tiff_entry_offset(&overflow, endian, 256);
        let height_entry = tiff_entry_offset(&overflow, endian, 257);
        write_tiff_u32(&mut overflow, width_entry + 8, u32::MAX, endian);
        write_tiff_u32(&mut overflow, height_entry + 8, u32::MAX, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&overflow).is_err());

        let mut trailing = valid;
        trailing.push(0);
        assert!(parse_tiff_rgb_u8_contiguous(&trailing).is_err());
    }

    #[test]
    fn rejects_unsupported_tiff_pixel_models() {
        let endian = TiffEndian::Big;
        let samples = rgb_gradient(2, 2);
        let valid = tiff_rgb(2, 2, &samples, endian, 2, false);
        for (tag, value) in [(259, 5), (262, 3), (277, 4), (284, 2)] {
            let mut malformed = valid.clone();
            let entry = tiff_entry_offset(&malformed, endian, tag);
            write_tiff_u16(&mut malformed, entry + 8, value, endian);
            assert!(
                parse_tiff_rgb_u8_contiguous(&malformed).is_err(),
                "tag {tag} value {value} must be rejected"
            );
        }
        let mut non_u8 = valid;
        let bits_entry = tiff_entry_offset(&non_u8, endian, 258);
        let bits = tiff_u32(&non_u8, bits_entry + 8, endian).unwrap() as usize;
        write_tiff_u16(&mut non_u8, bits, 16, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&non_u8).is_err());

        for unsupported_tag in [320, 338] {
            let mut tagged = tiff_rgb(2, 2, &samples, endian, 2, false);
            let planar = tiff_entry_offset(&tagged, endian, 284);
            write_tiff_u16(&mut tagged, planar, unsupported_tag, endian);
            assert!(parse_tiff_rgb_u8_contiguous(&tagged).is_err());
        }
    }

    #[test]
    fn rejects_invalid_lzw_predictor_and_inert_metadata_shapes() {
        let endian = TiffEndian::Little;
        let samples = rgb_gradient(4, 3);
        let (valid, _, _) = tiff_rgb_lzw(4, 3, &samples, endian, 2, false, None);
        let predictor = tiff_entry_offset(&valid, endian, 317);
        let compression = tiff_entry_offset(&valid, endian, 259);

        let mut wrong_predictor = valid.clone();
        write_tiff_u16(&mut wrong_predictor, predictor + 8, 1, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&wrong_predictor).is_err());

        let mut missing_predictor = valid.clone();
        write_tiff_u16(&mut missing_predictor, predictor, 274, endian);
        write_tiff_u16(&mut missing_predictor, predictor + 8, 1, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&missing_predictor).is_err());

        let mut illegal_uncompressed_predictor = valid.clone();
        write_tiff_u16(
            &mut illegal_uncompressed_predictor,
            compression + 8,
            1,
            endian,
        );
        assert!(parse_tiff_rgb_u8_contiguous(&illegal_uncompressed_predictor).is_err());

        let photoshop = tiff_entry_offset(&valid, endian, 34377);
        let document = tiff_entry_offset(&valid, endian, 37724);
        let mut wrong_type = valid.clone();
        write_tiff_u16(&mut wrong_type, photoshop + 2, 7, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&wrong_type).is_err());

        let mut empty_metadata = valid.clone();
        write_tiff_u32(&mut empty_metadata, photoshop + 4, 0, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&empty_metadata).is_err());

        let mut truncated_metadata = valid.clone();
        write_tiff_u32(&mut truncated_metadata, document + 8, u32::MAX, endian);
        assert!(parse_tiff_rgb_u8_contiguous(&truncated_metadata).is_err());

        let document_offset = tiff_u32(&valid, document + 8, endian).unwrap();
        let mut overlapping_metadata = valid;
        write_tiff_u32(
            &mut overlapping_metadata,
            photoshop + 8,
            document_offset,
            endian,
        );
        assert!(parse_tiff_rgb_u8_contiguous(&overlapping_metadata).is_err());
    }

    #[test]
    fn rendered_worker_rejects_decoded_model_mismatch_and_scrubs_open_paths() {
        let width = 2;
        let height = 2;
        let reference = tiff_rgb(
            width,
            height,
            &rgb_gradient(width, height),
            TiffEndian::Little,
            height,
            false,
        );
        let greyscale = grayscale_gradient(width, height);
        let info = ImageInfo::new(
            width,
            height,
            1,
            SampleFormat::U8,
            ColorModel::Grayscale,
            ComponentLayout::Interleaved,
        )
        .unwrap();
        let jp2 = encode(
            ImageView::Interleaved {
                info: &info,
                samples: &greyscale,
                stride_bytes: width as usize,
            },
            &EncodeOptions::default(),
        )
        .unwrap();
        assert!(
            compare_rendered_jp2_to_tiff(&jp2, &reference, rendered_contract(width, height, 0))
                .unwrap_err()
                .contains("metadata")
        );

        let absent = Path::new("/project-authored/secret/absent.jp2");
        let diagnostic = read_bounded_scrubbed(absent, MAX_INPUT_BYTES, "input").unwrap_err();
        assert_eq!(diagnostic, "cannot open input");
        assert!(!diagnostic.contains("secret"));
    }

    #[test]
    fn compares_an_explicit_project_authored_output_window() {
        let source_width = 4;
        let source_height = 3;
        let samples = grayscale_gradient(source_width, source_height);
        let codestream =
            generate_grayscale_j2k(source_width, source_height).expect("synthetic J2K encodes");
        let expected = vec![samples[5], samples[6], samples[9], samples[10]];
        let aggregates = compare_j2k_to_pgx(
            &codestream,
            &pgx(2, 2, &expected),
            ComparisonContract {
                output_window: true,
                output_origin_x: 1,
                output_origin_y: 1,
                ..contract(2, 2)
            },
        )
        .expect("synthetic output-window comparison succeeds");
        assert_eq!(aggregates.peak_error, 0);
        assert_eq!(aggregates.mean_squared_error, 0.0);
    }

    #[test]
    fn rejects_malformed_or_unbounded_comparison_inputs() {
        assert!(parse_pgx(b"PG ML + 8 1 1\n").is_err());
        assert!(parse_pgx(b"PG ML -4 1 1\n\x0f").is_err());
        assert!(parse_pgx(b"PG\tML -4 1 1\n\xff").is_err());
        assert!(parse_pgx(b"PG ML - 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML +-4 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML --4 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML +x 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG  ML 8 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML 8  1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML   8 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML  +8 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML  -8 1 1\n\x00").is_err());
        assert!(parse_pgx(b"PG ML +17 1 1\n\x00\x00\x01").is_err());
        assert!(
            validate_contract(ComparisonContract {
                width: 0,
                ..contract(1, 1)
            })
            .is_err()
        );
        assert!(
            validate_contract(ComparisonContract {
                resolution_reduction: 2,
                output_window: true,
                ..contract(1, 1)
            })
            .is_err()
        );
        validate_contract(ComparisonContract {
            resolution_reduction: 5,
            ..contract(1, 1)
        })
        .expect("full-component comparison admits five reduced levels");
        assert!(
            validate_contract(ComparisonContract {
                resolution_reduction: 6,
                ..contract(1, 1)
            })
            .is_err()
        );
        assert!(
            validate_contract(ComparisonContract {
                resolution_reduction: 1,
                output_window: true,
                output_origin_x: u32::MAX,
                ..contract(1, 1)
            })
            .is_err()
        );
        assert!(
            validate_contract(ComparisonContract {
                mean_squared_error_limit: f64::INFINITY,
                ..contract(1, 1)
            })
            .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn inspect_preserves_non_utf8_unix_paths() {
        use std::os::unix::ffi::OsStringExt;

        let mut filename = format!("emuella-j2k-cli-{}-", std::process::id()).into_bytes();
        filename.push(0xff);
        filename.extend_from_slice(b".j2k");
        let path = std::env::temp_dir().join(OsString::from_vec(filename));
        let codestream = generate_grayscale_j2k(2, 2).expect("synthetic J2K encodes");
        fs::write(&path, codestream).expect("synthetic J2K fixture is written");
        let result = run_arguments(vec![
            OsString::from("inspect"),
            path.clone().into_os_string(),
        ]);
        fs::remove_file(&path).expect("synthetic J2K fixture is removed");
        result.expect("inspect accepts a native non-UTF-8 path");
    }
}
