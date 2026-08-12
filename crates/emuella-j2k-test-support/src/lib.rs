use emuella_j2k_core::{
    ColorModel, ComponentLayout, EncodeOptions, ImageInfo, ImageView, OutputFormat, SampleFormat,
    encode,
};

/// Generate deterministic grayscale samples without an external image source.
pub fn grayscale_gradient(width: u32, height: u32) -> Vec<u8> {
    (0..height)
        .flat_map(|y| {
            (0..width).map(move |x| {
                ((x.wrapping_mul(13) + y.wrapping_mul(29) + x.wrapping_mul(y) * 3) & 0xff) as u8
            })
        })
        .collect()
}

/// Encode a deterministic project-authored grayscale image as raw J2K.
pub fn generate_grayscale_j2k(width: u32, height: u32) -> Result<Vec<u8>, String> {
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
