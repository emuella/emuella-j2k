use bytemuck::cast_slice;
#[cfg(target_endian = "little")]
use bytemuck::cast_slice_mut;
use emuella_j2k_core::{
    ColorModel, ComponentLayout, DecodeMode, DecodeOptions, Htj2kDecodeWorkspace,
    Htj2kEncodeOptions, ImageData, ImageInfo, ImageView, Plane, SampleFormat,
    decode_htj2k_with_workspace, encode_htj2k,
};
use numpy::{IntoPyArray, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    Hwc,
    Chw,
}

impl Layout {
    fn parse(value: &str) -> PyResult<Self> {
        match value {
            "HWC" => Ok(Self::Hwc),
            "CHW" => Ok(Self::Chw),
            _ => Err(PyValueError::new_err("layout must be 'HWC' or 'CHW'")),
        }
    }

    fn component_layout(self) -> ComponentLayout {
        match self {
            Self::Hwc => ComponentLayout::Interleaved,
            Self::Chw => ComponentLayout::Planar,
        }
    }
}

#[pyclass(unsendable)]
struct Htj2kCodec {
    layout: Layout,
    bit_depth: Option<u8>,
    decode_workspace: Htj2kDecodeWorkspace,
}

#[pymethods]
impl Htj2kCodec {
    #[new]
    #[pyo3(signature = (layout="HWC", bit_depth=None))]
    fn new(layout: &str, bit_depth: Option<u8>) -> PyResult<Self> {
        if bit_depth.is_some_and(|bits| !(8..=16).contains(&bits)) {
            return Err(PyValueError::new_err("bit_depth must be in 8..=16"));
        }
        Ok(Self {
            layout: Layout::parse(layout)?,
            bit_depth,
            decode_workspace: Htj2kDecodeWorkspace::new(),
        })
    }

    fn encode(&self, image: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
        if let Ok(array) = image.cast::<PyArrayDyn<u8>>() {
            let array = array.readonly();
            let samples = array
                .as_slice()
                .map_err(|_| PyValueError::new_err("image must be C-contiguous"))?;
            if self.bit_depth.is_some_and(|bits| bits != 8) {
                return Err(PyValueError::new_err("uint8 input requires bit_depth=8"));
            }
            return encode_array(samples, array.shape(), SampleFormat::U8, self.layout);
        }
        if let Ok(array) = image.cast::<PyArrayDyn<u16>>() {
            let array = array.readonly();
            let samples = array
                .as_slice()
                .map_err(|_| PyValueError::new_err("image must be C-contiguous"))?;
            let bits_per_sample = self.bit_depth.unwrap_or(16);
            let sample_format = SampleFormat::with_byte_order(
                bits_per_sample,
                false,
                Some(emuella_j2k_core::SampleEndian::Little),
            )
            .map_err(core_error)?;
            return encode_array(
                cast_slice(samples),
                array.shape(),
                sample_format,
                self.layout,
            );
        }
        Err(PyValueError::new_err(
            "image dtype must be uint8 or native-endian uint16",
        ))
    }

    #[pyo3(signature = (payload, shape, dtype))]
    fn decode<'py>(
        &mut self,
        py: Python<'py>,
        payload: &[u8],
        shape: Vec<usize>,
        dtype: &str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let expected = ArrayShape::parse(&shape, self.layout)?;
        let options = DecodeOptions {
            mode: DecodeMode::Components,
            target_layout: self.layout.component_layout(),
            ..DecodeOptions::default()
        };
        let image = decode_htj2k_with_workspace(payload, &options, &mut self.decode_workspace)
            .map_err(core_error)?
            .ok_or_else(|| {
                PyValueError::new_err("payload is outside the supported HTJ2K profile")
            })?;
        if image.info.width as usize != expected.width
            || image.info.height as usize != expected.height
            || image.info.components as usize != expected.components
        {
            return Err(PyValueError::new_err(format!(
                "decoded image {}x{}x{} does not match expected shape {shape:?}",
                image.info.width, image.info.height, image.info.components
            )));
        }
        let packed = packed_image_data(image.data)?;
        match dtype {
            "uint8" if image.info.sample_format == SampleFormat::U8 => packed
                .into_pyarray(py)
                .reshape(shape)
                .map(Bound::into_any)
                .map_err(|error| PyValueError::new_err(error.to_string())),
            "uint16"
                if image.info.sample_format.bits_per_sample > 8
                    && !image.info.sample_format.signed
                    && image.info.sample_format.byte_order
                        == Some(emuella_j2k_core::SampleEndian::Little) =>
            {
                let samples = unpack_little_endian_u16(&packed)?;
                samples
                    .into_pyarray(py)
                    .reshape(shape)
                    .map(Bound::into_any)
                    .map_err(|error| PyValueError::new_err(error.to_string()))
            }
            "int8"
                if image.info.sample_format.bits_per_sample <= 8
                    && image.info.sample_format.signed =>
            {
                packed
                    .into_iter()
                    .map(|sample| sample as i8)
                    .collect::<Vec<_>>()
                    .into_pyarray(py)
                    .reshape(shape)
                    .map(Bound::into_any)
                    .map_err(|error| PyValueError::new_err(error.to_string()))
            }
            "int16"
                if image.info.sample_format.bits_per_sample > 8
                    && image.info.sample_format.signed
                    && image.info.sample_format.byte_order
                        == Some(emuella_j2k_core::SampleEndian::Little) =>
            {
                unpack_little_endian_u16(&packed)?
                    .into_iter()
                    .map(|sample| sample as i16)
                    .collect::<Vec<_>>()
                    .into_pyarray(py)
                    .reshape(shape)
                    .map(Bound::into_any)
                    .map_err(|error| PyValueError::new_err(error.to_string()))
            }
            "uint8" | "uint16" | "int8" | "int16" => Err(PyValueError::new_err(format!(
                "decoded precision does not match requested dtype {dtype}"
            ))),
            _ => Err(PyValueError::new_err(
                "dtype must be 'uint8', 'uint16', 'int8', or 'int16'",
            )),
        }
    }

    fn workload_profile<'py>(
        &self,
        py: Python<'py>,
        payload: &[u8],
    ) -> PyResult<Bound<'py, PyDict>> {
        let profile =
            emuella_j2k_core::codestream::htj2k_lossless_no_decomp_workload_profile(payload)
                .map_err(|error| PyRuntimeError::new_err(error.to_string()))?
                .ok_or_else(|| {
                    PyValueError::new_err(
                        "payload is outside the profiled lossless HTJ2K no-decomposition profile",
                    )
                })?;
        let result = PyDict::new(py);
        result.set_item(
            "configured_code_block_width",
            profile.configured_code_block_width,
        )?;
        result.set_item(
            "configured_code_block_height",
            profile.configured_code_block_height,
        )?;
        result.set_item("total_code_blocks", profile.total_code_block_count)?;
        result.set_item("included_code_blocks", profile.included_code_block_count)?;
        result.set_item("omitted_code_blocks", profile.omitted_code_block_count)?;
        result.set_item(
            "cleanup_only_code_blocks",
            profile.cleanup_only_code_block_count,
        )?;
        result.set_item(
            "refinement_code_blocks",
            profile.refinement_code_block_count,
        )?;
        let cleanup_octets = PyDict::new(py);
        cleanup_octets.set_item("total", profile.cleanup_octets.octet_count)?;
        cleanup_octets.set_item(
            "embedded_magnitude_coefficients",
            profile.cleanup_octets.embedded_magnitude_coefficient_count,
        )?;
        cleanup_octets.set_item(
            "exponent_reduction_coefficients",
            profile.cleanup_octets.exponent_reduction_coefficient_count,
        )?;
        let initial_vlc_contexts = PyDict::new(py);
        for (context, quads) in profile
            .cleanup_octets
            .initial_vlc_context_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if quads != 0 {
                initial_vlc_contexts.set_item(context, quads)?;
            }
        }
        cleanup_octets.set_item("initial_vlc_context_histogram", initial_vlc_contexts)?;
        let noninitial_vlc_contexts = PyDict::new(py);
        for (context, quads) in profile
            .cleanup_octets
            .noninitial_vlc_context_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if quads != 0 {
                noninitial_vlc_contexts.set_item(context, quads)?;
            }
        }
        cleanup_octets.set_item("noninitial_vlc_context_histogram", noninitial_vlc_contexts)?;
        let uvlc_mode_names = ["neither", "first", "second", "both"];
        let initial_uvlc_modes = PyDict::new(py);
        for (mode, octets) in profile
            .cleanup_octets
            .initial_uvlc_mode_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if octets != 0 {
                initial_uvlc_modes.set_item(uvlc_mode_names[mode], octets)?;
            }
        }
        cleanup_octets.set_item("initial_uvlc_mode_histogram", initial_uvlc_modes)?;
        let noninitial_uvlc_modes = PyDict::new(py);
        for (mode, octets) in profile
            .cleanup_octets
            .noninitial_uvlc_mode_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if octets != 0 {
                noninitial_uvlc_modes.set_item(uvlc_mode_names[mode], octets)?;
            }
        }
        cleanup_octets.set_item("noninitial_uvlc_mode_histogram", noninitial_uvlc_modes)?;
        let significance_masks = PyDict::new(py);
        for (mask, count) in profile
            .cleanup_octets
            .significance_mask_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if count != 0 {
                significance_masks.set_item(format!("{mask:02x}"), count)?;
            }
        }
        cleanup_octets.set_item("significance_mask_histogram", significance_masks)?;
        let significant_counts = PyDict::new(py);
        for (count, octets) in profile
            .cleanup_octets
            .significant_coefficient_count_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if octets != 0 {
                significant_counts.set_item(count, octets)?;
            }
        }
        cleanup_octets.set_item(
            "significant_coefficient_count_histogram",
            significant_counts,
        )?;
        let magnitude_sign_bits = PyDict::new(py);
        for (bits, octets) in profile
            .cleanup_octets
            .magnitude_sign_bit_count_histogram
            .iter()
            .copied()
            .enumerate()
        {
            if octets != 0 {
                magnitude_sign_bits.set_item(bits, octets)?;
            }
        }
        cleanup_octets.set_item("magnitude_sign_bit_count_histogram", magnitude_sign_bits)?;
        let magnitude_sign_width_patterns = PyDict::new(py);
        for (&pattern, &octets) in &profile
            .cleanup_octets
            .magnitude_sign_width_pattern_histogram
        {
            magnitude_sign_width_patterns.set_item(format!("{pattern:010x}"), octets)?;
        }
        cleanup_octets.set_item(
            "magnitude_sign_width_pattern_histogram",
            magnitude_sign_width_patterns,
        )?;
        result.set_item("cleanup_octets", cleanup_octets)?;
        let blocks = PyList::empty(py);
        for block in profile.code_blocks {
            let item = PyDict::new(py);
            item.set_item("component", block.component_index)?;
            item.set_item("width", block.width)?;
            item.set_item("height", block.height)?;
            item.set_item("coefficients", block.coefficient_count)?;
            item.set_item("segment_bytes", block.segment_bytes)?;
            item.set_item(
                "cleanup_magnitude_sign_bytes",
                block.cleanup_magnitude_sign_bytes,
            )?;
            item.set_item(
                "cleanup_magnitude_sign_ff_bytes",
                block.cleanup_magnitude_sign_ff_bytes,
            )?;
            item.set_item(
                "cleanup_magnitude_sign_longest_non_ff_run",
                block.cleanup_magnitude_sign_longest_non_ff_run,
            )?;
            item.set_item("cleanup_mel_vlc_bytes", block.cleanup_mel_vlc_bytes)?;
            item.set_item("coding_passes", block.coding_passes)?;
            item.set_item(
                "missing_most_significant_bitplanes",
                block.missing_most_significant_bitplanes,
            )?;
            item.set_item("full_configured_block", block.full_configured_block)?;
            item.set_item("full_octets", block.full_octets)?;
            blocks.append(item)?;
        }
        result.set_item("code_blocks", blocks)?;
        Ok(result)
    }
}

fn encode_array(
    samples: &[u8],
    shape: &[usize],
    sample_format: SampleFormat,
    layout: Layout,
) -> PyResult<Vec<u8>> {
    let shape = ArrayShape::parse(shape, layout)?;
    let info = ImageInfo {
        width: shape.width as u32,
        height: shape.height as u32,
        components: shape.components as u16,
        sample_format,
        color_model: if shape.components == 1 {
            ColorModel::Grayscale
        } else {
            ColorModel::Rgb
        },
        layout: layout.component_layout(),
    };
    let bytes_per_sample = usize::from(sample_format.bits_per_sample > 8) + 1;
    let row_bytes = shape
        .width
        .checked_mul(bytes_per_sample)
        .ok_or_else(|| PyValueError::new_err("image size overflows address space"))?;

    match (layout, shape.components) {
        (Layout::Hwc, 1 | 3) => encode_htj2k(
            ImageView::Interleaved {
                info: &info,
                samples,
                stride_bytes: row_bytes * shape.components,
            },
            &Htj2kEncodeOptions::default(),
        )
        .map_err(core_error),
        (Layout::Chw, 1 | 3) => {
            let plane_len = row_bytes
                .checked_mul(shape.height)
                .ok_or_else(|| PyValueError::new_err("image size overflows address space"))?;
            let planes = samples
                .chunks_exact(plane_len)
                .map(|plane| {
                    Plane::new(
                        plane,
                        shape.width as u32,
                        shape.height as u32,
                        row_bytes,
                        sample_format,
                    )
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(core_error)?;
            encode_htj2k(
                ImageView::Planar {
                    info: &info,
                    planes: &planes,
                },
                &Htj2kEncodeOptions::default(),
            )
            .map_err(core_error)
        }
        _ => unreachable!("ArrayShape accepts one or three components"),
    }
}

#[derive(Debug, Clone, Copy)]
struct ArrayShape {
    width: usize,
    height: usize,
    components: usize,
}

impl ArrayShape {
    fn parse(shape: &[usize], layout: Layout) -> PyResult<Self> {
        let (height, width, components) = match shape {
            [height, width] => (*height, *width, 1),
            [height, width, components] if layout == Layout::Hwc => (*height, *width, *components),
            [components, height, width] if layout == Layout::Chw => (*height, *width, *components),
            _ => {
                return Err(PyValueError::new_err(
                    "image shape must be HW, HWC, or CHW for the configured layout",
                ));
            }
        };
        if height == 0 || width == 0 {
            return Err(PyValueError::new_err("image dimensions must be nonzero"));
        }
        if !matches!(components, 1 | 3) {
            return Err(PyValueError::new_err(
                "HTJ2K benchmark binding supports one or three components",
            ));
        }
        Ok(Self {
            width,
            height,
            components,
        })
    }
}

fn packed_image_data(data: ImageData) -> PyResult<Vec<u8>> {
    match data {
        ImageData::Interleaved(samples) => Ok(samples),
        ImageData::Planes(planes) if planes.len() == 1 => planes
            .into_iter()
            .next()
            .ok_or_else(|| PyRuntimeError::new_err("decoded image had no component plane")),
        ImageData::Planes(planes) => Ok(planes.into_iter().flatten().collect()),
    }
}

fn unpack_little_endian_u16(packed: &[u8]) -> PyResult<Vec<u16>> {
    if !packed.len().is_multiple_of(2) {
        return Err(PyRuntimeError::new_err(
            "decoded uint16 storage has an odd byte length",
        ));
    }

    let mut samples = vec![0_u16; packed.len() / 2];
    #[cfg(target_endian = "little")]
    cast_slice_mut(&mut samples).copy_from_slice(packed);
    #[cfg(target_endian = "big")]
    for (sample, bytes) in samples.iter_mut().zip(packed.chunks_exact(2)) {
        *sample = u16::from_le_bytes([bytes[0], bytes[1]]);
    }
    Ok(samples)
}

fn core_error(error: emuella_j2k_core::J2kError) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn emuella_j2k(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Htj2kCodec>()?;
    module.add_function(wrap_pyfunction!(version, module)?)?;
    Ok(())
}
