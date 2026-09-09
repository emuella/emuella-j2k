//! Bounded production-route diagnostics; ordinary decode owns all behaviour.
use super::*;

/// Actual native packing branch executed by the shared core output builder.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NativePackingRoute {
    #[default]
    Unrecorded,
    PlanarMove,
    SinglePlaneMove,
    RgbU8,
    RgbU16,
    GenericInterleaved,
}

/// One-worker full-core stage observation. Detailed checked Tier-1 operations
/// are unavailable here; the codestream's separate checked profiler retains them.
#[derive(Debug, Default)]
pub struct NativeDecodeTimings {
    pub total_ns: u128,
    pub core_preparation_ns: u128,
    /// Component metadata and the shared output builder, including packing.
    pub output_construction_ns: u128,
    pub packing_ns: u128,
    pub packing_route: NativePackingRoute,
    pub codestream: codestream::DecodeStageTimings,
}

/// Profile ordinary full core decode for raw native lossless D2 at one worker.
///
/// Requires all unsigned U8/U16 grey/RGB components or eight U16 bands, native
/// component mode, no layer limit and the documented frozen D2 coding profile.
/// The original full decode, admission, adaptive Tier-1, fused conversion and
/// native packing implementation is shared. Unsupported profiles fail explicitly.
/// Timings perturb execution and are not an uninstrumented throughput result.
#[cfg(feature = "std")]
pub fn decode_native_d2_profiled(
    input: &[u8],
    options: &DecodeOptions,
) -> Result<(Image, NativeDecodeTimings)> {
    if options.mode != DecodeMode::Components
        || options.requested_components != ComponentSelection::All
        || options.max_quality_layers.is_some()
        || options.allow_best_effort_backend_decode
        || !input.starts_with(&[0xff, 0x4f])
        || codestream::parallel_worker_count().is_some_and(|workers| workers != 1)
    {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "production diagnostics require raw full native component decode and exactly one worker",
        ));
    }
    let start = std::time::Instant::now();
    let mut timings = NativeDecodeTimings::default();
    let image = decode_impl(input, options, Some(&mut timings))?;
    if !timings.codestream.production_one_worker {
        return Err(unsupported(
            UnsupportedFeature::ComponentLayout,
            "production diagnostics require the bounded classic lossless D2 route",
        ));
    }
    timings.total_ns = start.elapsed().as_nanos();
    Ok((image, timings))
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn shared_pan_output_moves_the_original_plane() {
        let samples = vec![17; 64];
        let pointer = samples.as_ptr();
        let decoded = codestream::DecodedImage {
            width: 8,
            height: 8,
            bits_per_sample: 8,
            signed: false,
            components: vec![codestream::DecodedComponent { samples }],
        };
        let options = DecodeOptions {
            mode: DecodeMode::Components,
            target_layout: ComponentLayout::Interleaved,
            ..Default::default()
        };
        let mut timings = NativeDecodeTimings::default();
        let image = decoded_baseline_to_image_with_component_info_impl(
            decoded,
            &options,
            None,
            Some(&mut timings),
        )
        .unwrap();
        let ImageData::Interleaved(samples) = image.data else {
            panic!("interleaved PAN expected")
        };
        assert_eq!(samples.as_ptr(), pointer);
        assert_eq!(timings.packing_route, NativePackingRoute::SinglePlaneMove);
    }
}
