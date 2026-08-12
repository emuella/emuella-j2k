use emuella_j2k_core::{
    ComponentLayout, ComponentSelection, DecodeMode, DecodeOptions, ImageData, InspectOptions,
    SampleEndian, SampleFormat, decode, inspect,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_INPUT_BYTES: u64 = 512 * 1024 * 1024;
const MAX_REFERENCE_BYTES: u64 = 512 * 1024 * 1024;
const MAX_COMPARISON_SAMPLES: u64 = 100_000_000;

fn usage() -> &'static str {
    "usage:\n  emuella-j2k inspect INPUT\n  emuella-j2k compare-pgx INPUT REFERENCE --component N --width N --height N --bits-per-sample N (--signed|--unsigned) --peak-error-limit N --mean-squared-error-limit N"
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ComparisonContract {
    component: u16,
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
    arguments: Vec<String>,
) -> Result<(PathBuf, PathBuf, ComparisonContract), String> {
    if arguments.len() < 2 {
        return Err(usage().to_owned());
    }
    let input = PathBuf::from(&arguments[0]);
    let reference = PathBuf::from(&arguments[1]);
    let mut component = None;
    let mut width = None;
    let mut height = None;
    let mut bits_per_sample = None;
    let mut signed = None;
    let mut peak_error_limit = None;
    let mut mean_squared_error_limit = None;
    let mut index = 2;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--component" if component.is_none() => {
                let value = take_flag_value(&arguments, &mut index, "--component")?;
                component = Some(parse_number(&value, "component")?);
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

fn validate_contract(contract: ComparisonContract) -> Result<(), String> {
    let samples = u64::from(contract.width)
        .checked_mul(u64::from(contract.height))
        .ok_or_else(|| "comparison dimensions overflow".to_owned())?;
    if samples == 0 || samples > MAX_COMPARISON_SAMPLES {
        return Err("comparison sample count is zero or exceeds the runner bound".to_owned());
    }
    if !(1..=32).contains(&contract.bits_per_sample) {
        return Err("comparison precision must be in 1..=32".to_owned());
    }
    if !contract.mean_squared_error_limit.is_finite() || contract.mean_squared_error_limit < 0.0 {
        return Err("mean-squared-error limit must be finite and non-negative".to_owned());
    }
    Ok(())
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

fn parse_pgx(bytes: &[u8]) -> Result<PgxImage, String> {
    let newline = bytes
        .iter()
        .take(256)
        .position(|byte| *byte == b'\n')
        .ok_or_else(|| "PGX header is absent or exceeds 255 bytes".to_owned())?;
    let header = std::str::from_utf8(&bytes[..newline])
        .map_err(|_| "PGX header is not UTF-8 text".to_owned())?
        .trim_end_matches('\r');
    let fields = header.split_ascii_whitespace().collect::<Vec<_>>();
    if fields.len() != 6 || fields[0] != "PG" {
        return Err("PGX header must contain the six required fields".to_owned());
    }
    let little_endian = match fields[1] {
        "ML" => false,
        "LM" => true,
        _ => return Err("PGX byte order must be ML or LM".to_owned()),
    };
    let signed = match fields[2] {
        "+" => false,
        "-" => true,
        _ => return Err("PGX sign field must be + or -".to_owned()),
    };
    let bits_per_sample = parse_number::<u8>(fields[3], "PGX bits per sample")?;
    let width = parse_number::<u32>(fields[4], "PGX width")?;
    let height = parse_number::<u32>(fields[5], "PGX height")?;
    if !(1..=32).contains(&bits_per_sample) || width == 0 || height == 0 {
        return Err("PGX dimensions or precision are outside the supported bounds".to_owned());
    }
    let sample_count = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or_else(|| "PGX sample count overflow".to_owned())?;
    if sample_count > MAX_COMPARISON_SAMPLES {
        return Err("PGX sample count exceeds the runner bound".to_owned());
    }
    let bytes_per_sample = usize::from(bits_per_sample).div_ceil(8);
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
    let mask = (1_u64 << bits_per_sample) - 1;
    raw &= mask;
    if signed && raw & (1_u64 << (bits_per_sample - 1)) != 0 {
        Ok(i64::try_from(raw).expect("32-bit sample fits i64") - (1_i64 << bits_per_sample))
    } else {
        i64::try_from(raw).map_err(|_| "sample value exceeds i64".to_owned())
    }
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
    let options = DecodeOptions {
        mode: DecodeMode::Components,
        requested_components: ComponentSelection::Indices(vec![contract.component]),
        target_layout: ComponentLayout::Planar,
        ..DecodeOptions::default()
    };
    let decoded =
        decode(codestream, &options).map_err(|error| format!("decode failed: {error}"))?;
    if decoded.component_info.len() != 1
        || decoded.component_info[0].source_component != Some(contract.component)
        || decoded.component_info[0].width != contract.width
        || decoded.component_info[0].height != contract.height
        || decoded.component_info[0].sample_format.bits_per_sample != contract.bits_per_sample
        || decoded.component_info[0].sample_format.signed != contract.signed
    {
        return Err("decoded component metadata disagrees with the comparison contract".to_owned());
    }
    let planes = match decoded.data {
        ImageData::Planes(planes) if planes.len() == 1 => planes,
        _ => return Err("component decode did not produce exactly one planar buffer".to_owned()),
    };
    let samples = decoded_logical_samples(&planes[0], decoded.component_info[0].sample_format)?;
    compare_samples(&samples, &reference.samples)
}

fn run_inspect(input: &Path) -> Result<(), String> {
    let bytes =
        fs::read(input).map_err(|error| format!("failed to read {}: {error}", input.display()))?;
    let metadata = inspect(&bytes, &InspectOptions::default())
        .map_err(|error| format!("failed to inspect {}: {error}", input.display()))?;
    println!("{metadata:#?}");
    Ok(())
}

fn run_compare(arguments: Vec<String>) -> Result<(), String> {
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

fn run() -> Result<(), String> {
    let mut arguments = env::args_os().skip(1);
    let command = arguments.next().ok_or_else(|| usage().to_owned())?;
    let remaining = arguments
        .map(|argument| {
            argument
                .into_string()
                .map_err(|_| "arguments must be valid UTF-8".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    match command.to_str() {
        Some("inspect") if remaining.len() == 1 => run_inspect(Path::new(&remaining[0])),
        Some("compare-pgx") => run_compare(remaining),
        _ => Err(usage().to_owned()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("emuella-j2k: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use emuella_j2k_core::{ColorModel, EncodeOptions, ImageInfo, ImageView, OutputFormat, encode};

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

    fn pgx(width: u32, height: u32, samples: &[u8]) -> Vec<u8> {
        let mut bytes = format!("PG ML + 8 {width} {height}\n").into_bytes();
        bytes.extend_from_slice(samples);
        bytes
    }

    fn contract(width: u32, height: u32) -> ComparisonContract {
        ComparisonContract {
            component: 0,
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
            parse_pgx(b"PG LM - 12 2 1\n\xff\x0f\x00\x08").expect("project-authored PGX parses");
        assert_eq!(parsed.width, 2);
        assert_eq!(parsed.height, 1);
        assert_eq!(parsed.bits_per_sample, 12);
        assert!(parsed.signed);
        assert_eq!(parsed.samples, [-1, -2048]);
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
    fn rejects_malformed_or_unbounded_comparison_inputs() {
        assert!(parse_pgx(b"PG ML + 8 1 1\n").is_err());
        assert!(
            validate_contract(ComparisonContract {
                width: 0,
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
}
