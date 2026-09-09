//! Opt-in authored allocation diagnostic; never linked into codec libraries.
#![allow(unsafe_code)]
use emuella_j2k_core::*;
use sha2::{Digest, Sha256};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
struct Meter;
static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
fn add(n: usize) {
    let live = LIVE.fetch_add(n, SeqCst) + n;
    PEAK.fetch_max(live, SeqCst);
}
// Diagnostic-only allocator. Every pointer and layout is forwarded unchanged to
// System. Reallocation conservatively counts old and new requests concurrently.
unsafe impl GlobalAlloc for Meter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(l) };
        if !p.is_null() {
            add(l.size());
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(l) };
        if !p.is_null() {
            add(l.size());
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) };
        LIVE.fetch_sub(l.size(), SeqCst);
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        add(n);
        let q = unsafe { System.realloc(p, l, n) };
        LIVE.fetch_sub(if q.is_null() { n } else { l.size() }, SeqCst);
        q
    }
}
#[global_allocator]
static ALLOC: Meter = Meter;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let w: u32 = a.get(1).map_or(67, |s| s.parse().unwrap());
    let h: u32 = a.get(2).map_or(65, |s| s.parse().unwrap());
    let c: u16 = a.get(3).map_or(1, |s| s.parse().unwrap());
    let bits: u8 = a.get(4).map_or(16, |s| s.parse().unwrap());
    let b = usize::from(bits / 8);
    assert!(matches!(c, 1 | 3 | 8) && matches!(bits, 8 | 16) && (c != 8 || bits == 16));
    let planar = a.get(5).is_some_and(|s| s == "planar");
    assert!(
        a.get(5)
            .is_none_or(|s| matches!(s.as_str(), "planar" | "interleaved"))
    );
    let info = ImageInfo::new(
        w,
        h,
        c,
        if bits == 8 {
            SampleFormat::U8
        } else {
            SampleFormat::U16_LE
        },
        if c == 1 {
            ColorModel::Grayscale
        } else if c == 3 {
            ColorModel::Rgb
        } else {
            ColorModel::Unknown
        },
        ComponentLayout::Interleaved,
    )
    .unwrap();
    let mut samples = Vec::with_capacity(w as usize * h as usize * c as usize * b);
    let mut rng = 0x713b9d21u32;
    for _ in 0..w as usize * h as usize * c as usize {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        samples.extend_from_slice(&rng.to_le_bytes()[..b]);
    }
    let input_hash = Sha256::digest(&samples)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let options = EncodeOptions {
        format: OutputFormat::J2kCodestream,
        decomposition_levels: 2,
        ..Default::default()
    };
    let requirements =
        lossless_encode_requirements(&info, &options, &LosslessEncodeLimits::default()).unwrap();
    let mut planar_info = info.clone();
    planar_info.layout = ComponentLayout::Planar;
    let mut planar_storage = Vec::new();
    if planar {
        for band in 0..usize::from(c) {
            let mut component = Vec::with_capacity(w as usize * h as usize * b);
            for pixel in samples.chunks_exact(usize::from(c) * b) {
                component.extend_from_slice(&pixel[band * b..(band + 1) * b]);
            }
            planar_storage.push(component);
        }
    }
    let planes = planar_storage
        .iter()
        .map(|s| Plane::new(s, w, h, w as usize * b, info.sample_format).unwrap())
        .collect::<Vec<_>>();
    let view = if planar {
        ImageView::Planar {
            info: &planar_info,
            planes: &planes,
        }
    } else {
        ImageView::Interleaved {
            info: &info,
            samples: &samples,
            stride_bytes: w as usize * c as usize * b,
        }
    };
    let baseline = LIVE.load(SeqCst);
    PEAK.store(baseline, SeqCst);
    let start = std::time::Instant::now();
    let stream = encode(view, &options).unwrap();
    let seconds = start.elapsed().as_secs_f64();
    let peak = PEAK.load(SeqCst) - baseline;
    let retained = stream.capacity();
    assert!(
        peak as u64 <= requirements.working_bytes,
        "total measured peak must also bound additional encoder allocations"
    );
    assert!(retained as u64 <= requirements.output_capacity_limit);
    let output_hash = Sha256::digest(&stream)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let decoded = decode(
        &stream,
        &DecodeOptions {
            mode: DecodeMode::Components,
            target_layout: ComponentLayout::Interleaved,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(decoded.data, ImageData::Interleaved(samples));
    let spatial_pixels = u64::from(w) * u64::from(h);
    let aggregate_samples = requirements.total_component_samples;
    let max_working_bytes = LosslessEncodeLimits::default().max_working_bytes;
    println!(
        "spatial_pixels={spatial_pixels} aggregate_samples={aggregate_samples} max_working_bytes={max_working_bytes} width={w} height={h} components={c} bits={bits} planar={planar} input_sha256={input_hash} output_sha256={output_hash} seconds={seconds:.6} bytes={} retained_capacity={retained} peak_requested_encoder_bytes={peak} working_bound={} output_limit={} exact=true",
        stream.len(),
        requirements.working_bytes,
        requirements.output_capacity_limit
    );
}
