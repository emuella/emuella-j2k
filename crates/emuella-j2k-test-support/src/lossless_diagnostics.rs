//! Opt-in helpers for already-loaded native D2 diagnostics. No file I/O.
use emuella_j2k_codestream as codestream;
use emuella_j2k_core::{ComponentLayout, ImageData};
use std::time::Instant;

/// Native output and metadata from one measured full owned invocation.
pub struct NativeDecodeDiagnostic {
    pub width: u32,
    pub height: u32,
    pub bits_per_sample: u8,
    pub signed: bool,
    pub components: usize,
    pub data: ImageData,
    pub timings: codestream::DecodeStageTimings,
    pub packing_ns: u128,
}

/// Full owned component reconstruction followed by native layout packing.
/// Timings may disable parallel routes; use ordinary core decode for throughput.
/// Unsupported codestream diagnostics return the existing decoder error.
pub fn decode_native_profiled(
    stream: &[u8],
    layout: ComponentLayout,
    collect_work: bool,
) -> Result<NativeDecodeDiagnostic, codestream::CodestreamError> {
    let (mut image, timings) =
        codestream::decode_baseline_owned_components_profiled_with_work_counters(
            stream,
            collect_work,
        )?;
    let start = Instant::now();
    let data = match layout {
        ComponentLayout::Planar => ImageData::Planes(
            image
                .components
                .iter_mut()
                .map(|c| std::mem::take(&mut c.samples))
                .collect(),
        ),
        ComponentLayout::Interleaved => {
            let bytes = usize::from(image.bits_per_sample.div_ceil(8));
            let pixels = (image.width as usize)
                .checked_mul(image.height as usize)
                .ok_or(codestream::CodestreamError::SizeOverflow)?;
            let length = pixels
                .checked_mul(image.components.len())
                .and_then(|n| n.checked_mul(bytes))
                .ok_or(codestream::CodestreamError::SizeOverflow)?;
            if image
                .components
                .iter()
                .any(|c| c.samples.len() != pixels * bytes)
            {
                return Err(codestream::CodestreamError::SizeOverflow);
            }
            let mut output = vec![0; length];
            for (pixel, target) in output
                .chunks_exact_mut(image.components.len() * bytes)
                .enumerate()
            {
                for (component, destination) in
                    image.components.iter().zip(target.chunks_exact_mut(bytes))
                {
                    destination
                        .copy_from_slice(&component.samples[pixel * bytes..(pixel + 1) * bytes]);
                }
            }
            ImageData::Interleaved(output)
        }
    };
    // Release intermediate native planes inside the measured call. The result
    // retains metadata and the requested final layout, without duplicate pixels.
    let result = NativeDecodeDiagnostic {
        width: image.width,
        height: image.height,
        bits_per_sample: image.bits_per_sample,
        signed: image.signed,
        components: image.components.len(),
        data,
        timings,
        packing_ns: 0,
    };
    drop(image);
    Ok(NativeDecodeDiagnostic {
        packing_ns: start.elapsed().as_nanos(),
        ..result
    })
}
