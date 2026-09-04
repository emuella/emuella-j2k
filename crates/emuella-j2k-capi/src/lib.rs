//! Experimental C ABI for raw JPEG 2000 Part 1 positioned sources.

#[cfg(panic = "abort")]
compile_error!("emuella-j2k-capi requires panic=unwind for ABI containment");

use std::ffi::{c_char, c_void};
use std::mem::{align_of, size_of};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use emuella_j2k::codestream::source::{CodestreamSource, SourceError, SourceErrorKind};
use emuella_j2k::{
    ComponentInfo, Image, ImageData, ImageInfo, ImageViewMut, J2kError, Part1DecodeWorkspace,
    PlaneMut, SampleEndian, SampleFormat, execute_prepared_part1_decode_into_with_workspace,
    inspect_part1_source, prepare_part1_decode_from_source,
};

pub type EmuellaJ2kStatus = u32;
pub const EMUELLA_J2K_STATUS_OK: EmuellaJ2kStatus = 0;
pub const EMUELLA_J2K_STATUS_INVALID_ARGUMENT: EmuellaJ2kStatus = 1;
pub const EMUELLA_J2K_STATUS_INVALID_INPUT: EmuellaJ2kStatus = 2;
pub const EMUELLA_J2K_STATUS_UNSUPPORTED: EmuellaJ2kStatus = 3;
pub const EMUELLA_J2K_STATUS_SOURCE_IO: EmuellaJ2kStatus = 4;
pub const EMUELLA_J2K_STATUS_RESOURCE_LIMIT: EmuellaJ2kStatus = 5;
pub const EMUELLA_J2K_STATUS_INTERNAL: EmuellaJ2kStatus = 6;
pub const EMUELLA_J2K_STATUS_PANIC: EmuellaJ2kStatus = 7;

/// Experimental ABI version. Major zero carries no compatibility promise.
pub const EMUELLA_J2K_ABI_VERSION: u32 = 0;
pub const EMUELLA_J2K_ENDIAN_NONE: u8 = 0;
pub const EMUELLA_J2K_ENDIAN_LITTLE: u8 = 1;
pub const EMUELLA_J2K_ENDIAN_BIG: u8 = 2;

/// A positioned-read callback fills the complete requested destination range.
/// Any non-zero return is translated to source I/O failure.
pub type EmuellaJ2kReadAtFn = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        offset: u64,
        destination: *mut u8,
        length: usize,
    ) -> EmuellaJ2kStatus,
>;

#[repr(C)]
#[derive(Clone, Copy)]
/// Consumer-owned positioned source descriptor borrowed by a decoder.
pub struct EmuellaJ2kSourceV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub length: u64,
    pub context: *mut c_void,
    pub read_at: EmuellaJ2kReadAtFn,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// Reference-image properties or decoded single-plane properties.
pub struct EmuellaJ2kImageInfoV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub width: u32,
    pub height: u32,
    pub component_count: u16,
    pub bits_per_sample: u8,
    pub is_signed: u8,
    pub byte_order: u8,
    pub reserved_bytes: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// Native raw-component geometry and sample representation.
pub struct EmuellaJ2kComponentInfoV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub source_component: u16,
    pub bits_per_sample: u8,
    pub is_signed: u8,
    pub byte_order: u8,
    pub horizontal_separation: u8,
    pub vertical_separation: u8,
    pub reserved_byte: u8,
    pub width: u32,
    pub height: u32,
    pub x_origin: u32,
    pub y_origin: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
/// One-component decode request in full-resolution image-relative coordinates.
pub struct EmuellaJ2kDecodeRequestV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub component: u16,
    /// Zero means all quality layers; otherwise the leading layer count.
    pub max_quality_layers: u16,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub discard_levels: u8,
    pub reserved_bytes: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AbiHeader {
    struct_size: usize,
    abi_version: u32,
}

pub enum EmuellaJ2kDecoder {}
pub enum EmuellaJ2kInspection {}
pub enum EmuellaJ2kWorkspace {}
pub enum EmuellaJ2kImage {}
pub enum EmuellaJ2kError {}

#[derive(Clone, Copy)]
struct CallbackSource {
    context_address: usize,
    length: u64,
    read_at: unsafe extern "C" fn(*mut c_void, u64, *mut u8, usize) -> EmuellaJ2kStatus,
}

impl CodestreamSource for CallbackSource {
    fn len(&self) -> Result<u64, SourceError> {
        Ok(self.length)
    }

    fn read_exact_at(&self, offset: u64, destination: &mut [u8]) -> Result<(), SourceError> {
        let requested = destination.len() as u64;
        let available = self.length.saturating_sub(offset);
        let end = offset.checked_add(requested).ok_or_else(|| SourceError {
            kind: SourceErrorKind::OutOfRange,
            offset,
            requested,
            available,
            message: "positioned callback range overflowed u64".into(),
        })?;
        if end > self.length {
            return Err(SourceError {
                kind: SourceErrorKind::ShortRead,
                offset,
                requested,
                available,
                message: "positioned callback range exceeds the declared source length".into(),
            });
        }
        // SAFETY: source creation validated the callback pointer and retained
        // the context's exposed-provenance address. The consumer contract keeps
        // that allocation valid and the callback non-unwinding for the decoder
        // lifetime. `destination` is valid, aligned byte storage, exclusively
        // borrowed for this synchronous call, and the callback may neither
        // retain it nor write beyond `destination.len()`.
        let status = unsafe {
            (self.read_at)(
                ptr::with_exposed_provenance_mut(self.context_address),
                offset,
                destination.as_mut_ptr(),
                destination.len(),
            )
        };
        if status == EMUELLA_J2K_STATUS_OK {
            Ok(())
        } else {
            Err(SourceError {
                kind: SourceErrorKind::Io,
                offset,
                requested,
                available,
                message: format!("positioned callback returned status {status}"),
            })
        }
    }
}

struct DecoderState {
    source: CallbackSource,
}

struct InspectionState {
    image: ImageInfo,
    components: Vec<ComponentInfo>,
}

struct WorkspaceState {
    poisoned: AtomicBool,
    inner: Mutex<Part1DecodeWorkspace>,
}

struct ImageState {
    image: Image,
}

struct ErrorState {
    status: EmuellaJ2kStatus,
    message: Vec<u8>,
}

struct AbiFailure {
    status: EmuellaJ2kStatus,
    message: String,
}

impl AbiFailure {
    fn invalid(message: impl Into<String>) -> Self {
        Self {
            status: EMUELLA_J2K_STATUS_INVALID_ARGUMENT,
            message: message.into(),
        }
    }
}

impl From<J2kError> for AbiFailure {
    fn from(error: J2kError) -> Self {
        let status = match error {
            J2kError::InvalidParameter { .. } | J2kError::BufferTooSmall { .. } => {
                EMUELLA_J2K_STATUS_INVALID_ARGUMENT
            }
            J2kError::InvalidInput { .. } | J2kError::TruncatedInput { .. } => {
                EMUELLA_J2K_STATUS_INVALID_INPUT
            }
            J2kError::Unsupported { .. } => EMUELLA_J2K_STATUS_UNSUPPORTED,
            J2kError::Source { .. } => EMUELLA_J2K_STATUS_SOURCE_IO,
            J2kError::InternalInvariant { .. } => EMUELLA_J2K_STATUS_INTERNAL,
        };
        Self {
            status,
            message: error.to_string(),
        }
    }
}

fn checked_read<T: Copy>(pointer: *const T, name: &'static str) -> Result<T, AbiFailure> {
    if pointer.is_null() || !(pointer as usize).is_multiple_of(align_of::<T>()) {
        return Err(AbiFailure::invalid(format!(
            "{name} must be non-null and aligned"
        )));
    }
    // SAFETY: the caller contract requires `pointer` to name an initialised,
    // readable `T` for this synchronous call. Null and alignment were checked;
    // `T: Copy` prevents a foreign read from transferring Rust ownership, and
    // the returned value no longer borrows foreign storage.
    Ok(unsafe { pointer.read() })
}

fn checked_write<T>(pointer: *mut T, value: T, name: &'static str) -> Result<(), AbiFailure> {
    if pointer.is_null() || !(pointer as usize).is_multiple_of(align_of::<T>()) {
        return Err(AbiFailure::invalid(format!(
            "{name} must be non-null and aligned"
        )));
    }
    // SAFETY: the caller contract requires `pointer` to name initialised or
    // writable storage for one `T`, exclusively writable for this synchronous
    // call. Null and alignment were checked. `write` publishes exactly one
    // value and does not retain a foreign borrow.
    unsafe { pointer.write(value) };
    Ok(())
}

fn handle_ref<'a, O, S>(pointer: *const O, name: &'static str) -> Result<&'a S, AbiFailure> {
    if pointer.is_null() || !(pointer as usize).is_multiple_of(align_of::<S>()) {
        return Err(AbiFailure::invalid(format!(
            "{name} must be a non-null aligned live handle"
        )));
    }
    // SAFETY: the nominal opaque handle contract requires `pointer` to be the
    // unchanged pointer returned for an allocation of `S`, still live for this
    // call and not concurrently destroyed. Allocation alignment was checked;
    // the shared reference is limited to the call and no mutable reference to
    // immutable handle state is created.
    Ok(unsafe { &*(pointer.cast::<S>()) })
}

fn destroy_handle<O, S>(pointer: *mut O) {
    if pointer.is_null() {
        return;
    }
    // SAFETY: the opaque ownership contract requires this to be the exact live
    // pointer returned by `Box<S>`, destroyed once after all calls quiesce. The
    // pointee layout is private, no foreign allocation is reconstructed, and
    // `Box::from_raw` immediately restores Rust ownership for one drop.
    unsafe { drop(Box::from_raw(pointer.cast::<S>())) };
}

fn checked_copy(destination: *mut u8, source: &[u8]) -> Result<(), AbiFailure> {
    if source.is_empty() {
        return Ok(());
    }
    if destination.is_null() {
        return Err(AbiFailure::invalid("destination must be non-null"));
    }
    // SAFETY: the caller contract provides a writable range of at least
    // `source.len()` bytes, exclusively writable for this synchronous call and
    // disjoint from Rust-owned immutable image/error storage. The source slice
    // is valid, byte alignment is one, and no foreign borrow is retained.
    unsafe { ptr::copy_nonoverlapping(source.as_ptr(), destination, source.len()) };
    Ok(())
}

fn make_error(failure: &AbiFailure) -> Box<ErrorState> {
    let mut message = failure.message.as_bytes().to_vec();
    message.push(0);
    Box::new(ErrorState {
        status: failure.status,
        message,
    })
}

fn write_error(
    output: *mut *mut EmuellaJ2kError,
    failure: Option<&AbiFailure>,
) -> Result<(), AbiFailure> {
    if output.is_null() {
        return Ok(());
    }
    if !(output as usize).is_multiple_of(align_of::<*mut EmuellaJ2kError>()) {
        return Err(AbiFailure::invalid(
            "error_output must be null or aligned writable storage",
        ));
    }
    let value = failure.map_or(ptr::null_mut(), |failure| {
        Box::into_raw(make_error(failure)).cast::<EmuellaJ2kError>()
    });
    checked_write(output, value, "error_output")
}

fn boundary<F>(
    error_output: *mut *mut EmuellaJ2kError,
    poison_workspace: *const EmuellaJ2kWorkspace,
    operation: F,
) -> EmuellaJ2kStatus
where
    F: FnOnce() -> Result<(), AbiFailure>,
{
    // Validate and clear the optional diagnostic output before the operation
    // can allocate or publish any other output. A detectable bad diagnostic
    // pointer therefore cannot turn a successful operation into a leaking
    // failure after the fact.
    let result = catch_unwind(AssertUnwindSafe(|| {
        write_error(error_output, None)?;
        operation()
    }));
    match result {
        Ok(Ok(())) => EMUELLA_J2K_STATUS_OK,
        Ok(Err(failure)) => {
            let status = failure.status;
            match catch_unwind(AssertUnwindSafe(|| {
                write_error(error_output, Some(&failure))
            })) {
                Ok(Ok(())) => status,
                Ok(Err(_)) => EMUELLA_J2K_STATUS_INVALID_ARGUMENT,
                Err(_) => EMUELLA_J2K_STATUS_PANIC,
            }
        }
        Err(_) => {
            // Diagnostic construction is itself inside a final containment
            // barrier. If fallback diagnostics fail, the panic status remains
            // usable and no unwind reaches the foreign caller.
            let _ = catch_unwind(AssertUnwindSafe(|| {
                if let Ok(workspace) =
                    handle_ref::<EmuellaJ2kWorkspace, WorkspaceState>(poison_workspace, "workspace")
                {
                    workspace.poisoned.store(true, Ordering::Release);
                }
                let failure = AbiFailure {
                    status: EMUELLA_J2K_STATUS_PANIC,
                    message: "contained Rust panic".into(),
                };
                let _ = write_error(error_output, Some(&failure));
            }));
            EMUELLA_J2K_STATUS_PANIC
        }
    }
}

fn validate_header(struct_size: usize, abi_version: u32, minimum: usize) -> Result<(), AbiFailure> {
    if struct_size < minimum {
        return Err(AbiFailure::invalid(
            "structure is smaller than the required version",
        ));
    }
    if abi_version != EMUELLA_J2K_ABI_VERSION {
        return Err(AbiFailure::invalid("structure ABI version is incompatible"));
    }
    Ok(())
}

fn byte_order(format: SampleFormat) -> u8 {
    match format.byte_order {
        None => EMUELLA_J2K_ENDIAN_NONE,
        Some(SampleEndian::Little) => EMUELLA_J2K_ENDIAN_LITTLE,
        Some(SampleEndian::Big) => EMUELLA_J2K_ENDIAN_BIG,
    }
}

fn image_info(info: &ImageInfo) -> EmuellaJ2kImageInfoV0 {
    EmuellaJ2kImageInfoV0 {
        struct_size: size_of::<EmuellaJ2kImageInfoV0>(),
        abi_version: EMUELLA_J2K_ABI_VERSION,
        reserved: 0,
        width: info.width,
        height: info.height,
        component_count: info.components,
        bits_per_sample: info.sample_format.bits_per_sample,
        is_signed: u8::from(info.sample_format.signed),
        byte_order: byte_order(info.sample_format),
        reserved_bytes: [0; 7],
    }
}

fn component_info(info: &ComponentInfo) -> Result<EmuellaJ2kComponentInfoV0, AbiFailure> {
    Ok(EmuellaJ2kComponentInfoV0 {
        struct_size: size_of::<EmuellaJ2kComponentInfoV0>(),
        abi_version: EMUELLA_J2K_ABI_VERSION,
        reserved: 0,
        source_component: info
            .source_component
            .ok_or_else(|| AbiFailure::invalid("component has no raw source index"))?,
        bits_per_sample: info.sample_format.bits_per_sample,
        is_signed: u8::from(info.sample_format.signed),
        byte_order: byte_order(info.sample_format),
        horizontal_separation: info.horizontal_separation,
        vertical_separation: info.vertical_separation,
        reserved_byte: 0,
        width: info.width,
        height: info.height,
        x_origin: info.x_origin,
        y_origin: info.y_origin,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn emuella_j2k_abi_version() -> u32 {
    EMUELLA_J2K_ABI_VERSION
}

static PACKAGE_VERSION: &[u8] = concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes();

/// Return a library-owned NUL-terminated package version string.
#[unsafe(no_mangle)]
pub extern "C" fn emuella_j2k_package_version() -> *const c_char {
    PACKAGE_VERSION.as_ptr().cast::<c_char>()
}

#[unsafe(no_mangle)]
/// Create a decoder that borrows the source descriptor's context and callback.
pub extern "C" fn emuella_j2k_decoder_create(
    source: *const EmuellaJ2kSourceV0,
    output: *mut *mut EmuellaJ2kDecoder,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        checked_write(output, ptr::null_mut(), "decoder_output")?;
        let header = checked_read(source.cast::<AbiHeader>(), "source")?;
        validate_header(
            header.struct_size,
            header.abi_version,
            size_of::<EmuellaJ2kSourceV0>(),
        )?;
        let source = checked_read(source, "source")?;
        if source.reserved != 0 {
            return Err(AbiFailure::invalid("source reserved field must be zero"));
        }
        let read_at = source
            .read_at
            .ok_or_else(|| AbiFailure::invalid("source read_at callback is required"))?;
        let decoder = Box::new(DecoderState {
            source: CallbackSource {
                context_address: source.context.expose_provenance(),
                length: source.length,
                read_at,
            },
        });
        checked_write(
            output,
            Box::into_raw(decoder).cast::<EmuellaJ2kDecoder>(),
            "decoder_output",
        )
    })
}

#[unsafe(no_mangle)]
/// Destroy a decoder after all calls and callbacks using it have quiesced.
pub extern "C" fn emuella_j2k_decoder_destroy(decoder: *mut EmuellaJ2kDecoder) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        destroy_handle::<EmuellaJ2kDecoder, DecoderState>(decoder)
    }));
}

#[unsafe(no_mangle)]
/// Inspect raw Part 1 geometry without decoding packet bodies.
pub extern "C" fn emuella_j2k_decoder_inspect(
    decoder: *const EmuellaJ2kDecoder,
    output: *mut *mut EmuellaJ2kInspection,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        checked_write(output, ptr::null_mut(), "inspection_output")?;
        let decoder = handle_ref::<EmuellaJ2kDecoder, DecoderState>(decoder, "decoder")?;
        let inspected = inspect_part1_source(&decoder.source).map_err(AbiFailure::from)?;
        let inspection = Box::new(InspectionState {
            image: inspected.image,
            components: inspected.components,
        });
        checked_write(
            output,
            Box::into_raw(inspection).cast::<EmuellaJ2kInspection>(),
            "inspection_output",
        )
    })
}

#[unsafe(no_mangle)]
/// Destroy an immutable inspection handle.
pub extern "C" fn emuella_j2k_inspection_destroy(inspection: *mut EmuellaJ2kInspection) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        destroy_handle::<EmuellaJ2kInspection, InspectionState>(inspection)
    }));
}

#[unsafe(no_mangle)]
/// Copy reference-image properties into caller-owned storage.
pub extern "C" fn emuella_j2k_inspection_image_info(
    inspection: *const EmuellaJ2kInspection,
    output: *mut EmuellaJ2kImageInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        let inspection =
            handle_ref::<EmuellaJ2kInspection, InspectionState>(inspection, "inspection")?;
        checked_write(output, image_info(&inspection.image), "image_info_output")
    })
}

#[unsafe(no_mangle)]
/// Copy one inspected component descriptor into caller-owned storage.
pub extern "C" fn emuella_j2k_inspection_component_info(
    inspection: *const EmuellaJ2kInspection,
    component: u16,
    output: *mut EmuellaJ2kComponentInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        let inspection =
            handle_ref::<EmuellaJ2kInspection, InspectionState>(inspection, "inspection")?;
        let component = inspection
            .components
            .get(usize::from(component))
            .ok_or_else(|| AbiFailure::invalid("component index is out of bounds"))?;
        checked_write(output, component_info(component)?, "component_info_output")
    })
}

#[unsafe(no_mangle)]
/// Create an exclusively used, reusable decode workspace.
pub extern "C" fn emuella_j2k_workspace_create(
    output: *mut *mut EmuellaJ2kWorkspace,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        checked_write(output, ptr::null_mut(), "workspace_output")?;
        let workspace = Box::new(WorkspaceState {
            poisoned: AtomicBool::new(false),
            inner: Mutex::new(Part1DecodeWorkspace::new()),
        });
        checked_write(
            output,
            Box::into_raw(workspace).cast::<EmuellaJ2kWorkspace>(),
            "workspace_output",
        )
    })
}

#[unsafe(no_mangle)]
/// Destroy an idle workspace, including a workspace poisoned by panic.
pub extern "C" fn emuella_j2k_workspace_destroy(workspace: *mut EmuellaJ2kWorkspace) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        destroy_handle::<EmuellaJ2kWorkspace, WorkspaceState>(workspace)
    }));
}

#[unsafe(no_mangle)]
/// Decode one component region into a new immutable Rust-owned image.
pub extern "C" fn emuella_j2k_decode_component_region(
    decoder: *const EmuellaJ2kDecoder,
    workspace: *const EmuellaJ2kWorkspace,
    request: *const EmuellaJ2kDecodeRequestV0,
    output: *mut *mut EmuellaJ2kImage,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, workspace, || {
        checked_write(output, ptr::null_mut(), "image_output")?;
        let decoder = handle_ref::<EmuellaJ2kDecoder, DecoderState>(decoder, "decoder")?;
        let workspace = handle_ref::<EmuellaJ2kWorkspace, WorkspaceState>(workspace, "workspace")?;
        if workspace.poisoned.load(Ordering::Acquire) {
            return Err(AbiFailure::invalid(
                "workspace is poisoned and may only be destroyed",
            ));
        }
        let header = checked_read(request.cast::<AbiHeader>(), "request")?;
        validate_header(
            header.struct_size,
            header.abi_version,
            size_of::<EmuellaJ2kDecodeRequestV0>(),
        )?;
        let request = checked_read(request, "request")?;
        if request.reserved != 0 || request.reserved_bytes != [0; 7] {
            return Err(AbiFailure::invalid("request reserved fields must be zero"));
        }
        if request.width == 0 || request.height == 0 {
            return Err(AbiFailure::invalid(
                "request width and height must be non-zero",
            ));
        }
        let component_indices = [request.component];
        let prepared = prepare_part1_decode_from_source(
            &decoder.source,
            emuella_j2k::codestream::Part1ComponentDecodeRequest {
                component_indices: &component_indices,
                region: emuella_j2k::codestream::TileRegionRequest {
                    x: request.x,
                    y: request.y,
                    width: request.width,
                    height: request.height,
                },
                discard_levels: request.discard_levels,
                max_layers: (request.max_quality_layers != 0).then_some(request.max_quality_layers),
            },
        )
        .map_err(AbiFailure::from)?;
        let info = prepared.info().clone();
        let components = prepared.component_info().to_vec();
        let component = components
            .first()
            .ok_or_else(|| AbiFailure::invalid("decode produced no component descriptor"))?;
        let sample_bytes = usize::from(component.sample_format.bits_per_sample).div_ceil(8);
        let stride = usize::try_from(component.width)
            .ok()
            .and_then(|width| width.checked_mul(sample_bytes))
            .ok_or_else(|| AbiFailure {
                status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
                message: "decoded row byte size overflowed".into(),
            })?;
        let length = stride
            .checked_mul(usize::try_from(component.height).map_err(|_| AbiFailure {
                status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
                message: "decoded height exceeds addressable storage".into(),
            })?)
            .ok_or_else(|| AbiFailure {
                status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
                message: "decoded image byte size overflowed".into(),
            })?;
        let mut samples = Vec::new();
        samples.try_reserve_exact(length).map_err(|_| AbiFailure {
            status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
            message: "decoded image allocation failed".into(),
        })?;
        samples.resize(length, 0);
        let plane = PlaneMut::new(
            &mut samples,
            component.width,
            component.height,
            stride,
            component.sample_format,
        )
        .map_err(AbiFailure::from)?;
        let mut planes = [plane];
        let mut target = ImageViewMut::Planar {
            info: &info,
            planes: &mut planes,
        };
        let mut workspace = workspace.inner.lock().map_err(|_| AbiFailure {
            status: EMUELLA_J2K_STATUS_PANIC,
            message: "workspace mutex was poisoned".into(),
        })?;
        #[cfg(test)]
        if FORCE_DECODE_PANIC.with(|force| force.replace(false)) {
            panic!("C ABI panic containment test");
        }
        execute_prepared_part1_decode_into_with_workspace(
            &prepared,
            &mut target,
            &mut workspace,
            emuella_j2k::codestream::PreparedPart1ExecutionOptions::default(),
        )
        .map_err(AbiFailure::from)?;
        let image = Box::new(ImageState {
            image: Image {
                info,
                component_info: components,
                data: ImageData::Planes(vec![samples]),
            },
        });
        checked_write(
            output,
            Box::into_raw(image).cast::<EmuellaJ2kImage>(),
            "image_output",
        )
    })
}

#[unsafe(no_mangle)]
/// Destroy an immutable decoded image.
pub extern "C" fn emuella_j2k_image_destroy(image: *mut EmuellaJ2kImage) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        destroy_handle::<EmuellaJ2kImage, ImageState>(image)
    }));
}

#[unsafe(no_mangle)]
/// Copy decoded image properties into caller-owned storage.
pub extern "C" fn emuella_j2k_image_info(
    image: *const EmuellaJ2kImage,
    output: *mut EmuellaJ2kImageInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        let image = handle_ref::<EmuellaJ2kImage, ImageState>(image, "image")?;
        checked_write(output, image_info(&image.image.info), "image_info_output")
    })
}

/// Copy the single decoded component descriptor into caller-owned storage.
#[unsafe(no_mangle)]
pub extern "C" fn emuella_j2k_image_component_info(
    image: *const EmuellaJ2kImage,
    output: *mut EmuellaJ2kComponentInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        let image = handle_ref::<EmuellaJ2kImage, ImageState>(image, "image")?;
        let component = image
            .image
            .component_info
            .first()
            .ok_or_else(|| AbiFailure::invalid("image has no component descriptor"))?;
        checked_write(output, component_info(component)?, "component_info_output")
    })
}

#[unsafe(no_mangle)]
/// Copy decoded rows into a bounded caller-owned buffer with explicit stride.
pub extern "C" fn emuella_j2k_image_copy(
    image: *const EmuellaJ2kImage,
    destination: *mut u8,
    capacity: usize,
    stride_bytes: usize,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    boundary(error_output, ptr::null(), || {
        let image = handle_ref::<EmuellaJ2kImage, ImageState>(image, "image")?;
        let ImageData::Planes(planes) = &image.image.data else {
            return Err(AbiFailure::invalid("image is not planar"));
        };
        let source = planes
            .first()
            .ok_or_else(|| AbiFailure::invalid("image has no component plane"))?;
        let component = image
            .image
            .component_info
            .first()
            .ok_or_else(|| AbiFailure::invalid("image has no component descriptor"))?;
        let sample_bytes = usize::from(component.sample_format.bits_per_sample).div_ceil(8);
        let row_bytes = usize::try_from(component.width)
            .ok()
            .and_then(|width| width.checked_mul(sample_bytes))
            .ok_or_else(|| AbiFailure::invalid("image row byte size overflowed"))?;
        if stride_bytes < row_bytes {
            return Err(AbiFailure::invalid(
                "destination stride is smaller than one row",
            ));
        }
        let height = usize::try_from(component.height)
            .map_err(|_| AbiFailure::invalid("image height exceeds usize"))?;
        let required = if height == 0 {
            0
        } else {
            (height - 1)
                .checked_mul(stride_bytes)
                .and_then(|prefix| prefix.checked_add(row_bytes))
                .ok_or_else(|| AbiFailure::invalid("destination extent overflowed"))?
        };
        if capacity < required {
            return Err(AbiFailure::invalid(format!(
                "destination capacity {capacity} is smaller than required {required}"
            )));
        }
        if required != 0 && destination.is_null() {
            return Err(AbiFailure::invalid("destination must be non-null"));
        }
        for row in 0..height {
            let source_start = row
                .checked_mul(row_bytes)
                .ok_or_else(|| AbiFailure::invalid("source row offset overflowed"))?;
            let destination_start = row
                .checked_mul(stride_bytes)
                .ok_or_else(|| AbiFailure::invalid("destination row offset overflowed"))?;
            let destination_row = destination.wrapping_add(destination_start);
            checked_copy(
                destination_row,
                &source[source_start..source_start + row_bytes],
            )?;
        }
        Ok(())
    })
}

#[unsafe(no_mangle)]
/// Destroy an immutable diagnostic handle.
pub extern "C" fn emuella_j2k_error_destroy(error: *mut EmuellaJ2kError) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        destroy_handle::<EmuellaJ2kError, ErrorState>(error)
    }));
}

#[unsafe(no_mangle)]
/// Return the status retained by an immutable diagnostic handle.
pub extern "C" fn emuella_j2k_error_status(error: *const EmuellaJ2kError) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        handle_ref::<EmuellaJ2kError, ErrorState>(error, "error")
            .map_or(EMUELLA_J2K_STATUS_INVALID_ARGUMENT, |error| error.status)
    }))
    .unwrap_or(EMUELLA_J2K_STATUS_PANIC)
}

#[unsafe(no_mangle)]
/// Return the diagnostic byte count, including its terminating NUL byte.
pub extern "C" fn emuella_j2k_error_message_size(
    error: *const EmuellaJ2kError,
    output: *mut usize,
) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        let error = handle_ref::<EmuellaJ2kError, ErrorState>(error, "error")?;
        checked_write(output, error.message.len(), "message_size_output")
    }))
    .map_or(EMUELLA_J2K_STATUS_PANIC, |result| {
        result.map_or_else(|failure| failure.status, |()| EMUELLA_J2K_STATUS_OK)
    })
}

#[unsafe(no_mangle)]
/// Copy the complete NUL-terminated UTF-8 diagnostic into caller storage.
pub extern "C" fn emuella_j2k_error_message_copy(
    error: *const EmuellaJ2kError,
    destination: *mut u8,
    capacity: usize,
) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        let error = handle_ref::<EmuellaJ2kError, ErrorState>(error, "error")?;
        if capacity < error.message.len() {
            return Err(AbiFailure::invalid("diagnostic destination is too small"));
        }
        checked_copy(destination, &error.message)
    }))
    .map_or(EMUELLA_J2K_STATUS_PANIC, |result| {
        result.map_or_else(|failure| failure.status, |()| EMUELLA_J2K_STATUS_OK)
    })
}

#[cfg(test)]
std::thread_local! {
    static FORCE_DECODE_PANIC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    struct TestSource {
        bytes: Vec<u8>,
        fail_reads: bool,
    }

    unsafe extern "C" fn test_read_at(
        context: *mut c_void,
        offset: u64,
        destination: *mut u8,
        length: usize,
    ) -> EmuellaJ2kStatus {
        // SAFETY: every test call keeps its boxed `TestSource` alive through
        // decoder destruction. Production code passes an exclusively borrowed
        // Rust destination valid for `length` bytes. Bounds are checked before
        // the copy, byte alignment is one, ranges do not overlap, and neither
        // reference escapes this synchronous callback.
        unsafe {
            let source = &*(context.cast::<TestSource>());
            if source.fail_reads {
                return 91;
            }
            let Ok(start) = usize::try_from(offset) else {
                return 92;
            };
            let Some(end) = start.checked_add(length) else {
                return 93;
            };
            let Some(bytes) = source.bytes.get(start..end) else {
                return 94;
            };
            ptr::copy_nonoverlapping(bytes.as_ptr(), destination, length);
        }
        EMUELLA_J2K_STATUS_OK
    }

    fn fixture() -> (Box<TestSource>, Vec<u8>) {
        let samples = (0_u8..16).collect::<Vec<_>>();
        let bytes =
            emuella_j2k::codestream::encode_planar_u8_no_decomp_test_fixture(4, 4, &[&samples])
                .unwrap();
        (
            Box::new(TestSource {
                bytes,
                fail_reads: false,
            }),
            samples,
        )
    }

    fn heterogeneous_fixture() -> Box<TestSource> {
        let planes = [vec![0_u8; 16], vec![0_u8; 8], vec![0_u8; 4]];
        let mut bytes =
            emuella_j2k::codestream::encode_planar_u8_subsampled_no_decomp_test_fixture(
                4,
                4,
                &[
                    emuella_j2k::codestream::SubsampledU8TestComponent {
                        horizontal_separation: 1,
                        vertical_separation: 1,
                        samples: &planes[0],
                    },
                    emuella_j2k::codestream::SubsampledU8TestComponent {
                        horizontal_separation: 2,
                        vertical_separation: 1,
                        samples: &planes[1],
                    },
                    emuella_j2k::codestream::SubsampledU8TestComponent {
                        horizontal_separation: 2,
                        vertical_separation: 2,
                        samples: &planes[2],
                    },
                ],
            )
            .unwrap();
        assert_eq!(&bytes[..4], &[0xff, 0x4f, 0xff, 0x51]);
        bytes[45] = 0x8b;
        bytes[48] = 0x0f;
        Box::new(TestSource {
            bytes,
            fail_reads: false,
        })
    }

    fn source_descriptor(source: &mut TestSource) -> EmuellaJ2kSourceV0 {
        EmuellaJ2kSourceV0 {
            struct_size: size_of::<EmuellaJ2kSourceV0>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
            reserved: 0,
            length: source.bytes.len() as u64,
            context: ptr::from_mut(source).cast::<c_void>(),
            read_at: Some(test_read_at),
        }
    }

    fn decoder(source: &mut TestSource) -> *mut EmuellaJ2kDecoder {
        let descriptor = source_descriptor(source);
        let mut decoder = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_create(&descriptor, &mut decoder, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        assert!(!decoder.is_null());
        decoder
    }

    assert_impl_all!(CallbackSource: Send, Sync);
    assert_impl_all!(DecoderState: Send, Sync);
    assert_impl_all!(WorkspaceState: Send, Sync);
    assert_impl_all!(InspectionState: Send, Sync);
    assert_impl_all!(ImageState: Send, Sync);
    assert_impl_all!(ErrorState: Send, Sync);

    #[test]
    fn invalid_null_outputs_are_rejected_without_ub() {
        assert_eq!(
            emuella_j2k_workspace_create(ptr::null_mut(), ptr::null_mut()),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert_eq!(
            emuella_j2k_error_message_size(ptr::null(), ptr::null_mut()),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        let mut aligned_storage = [0_usize; 2];
        let misaligned_output = aligned_storage
            .as_mut_ptr()
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<*mut EmuellaJ2kWorkspace>();
        assert_eq!(
            emuella_j2k_workspace_create(misaligned_output, ptr::null_mut()),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );

        let mut workspace = ptr::null_mut();
        let misaligned_error = aligned_storage
            .as_mut_ptr()
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<*mut EmuellaJ2kError>();
        assert_eq!(
            emuella_j2k_workspace_create(&mut workspace, misaligned_error),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert!(workspace.is_null());
    }

    #[test]
    fn inspect_decode_copy_and_workspace_reuse_are_failure_atomic() {
        let (mut source, expected) = fixture();
        let decoder = decoder(&mut source);
        let mut inspection = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        let mut info = EmuellaJ2kImageInfoV0::default();
        assert_eq!(
            emuella_j2k_inspection_image_info(inspection, &mut info, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((info.width, info.height, info.component_count), (4, 4, 1));
        let mut component = EmuellaJ2kComponentInfoV0::default();
        assert_eq!(
            emuella_j2k_inspection_component_info(inspection, 0, &mut component, ptr::null_mut(),),
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((component.width, component.height), (4, 4));
        assert_eq!(
            emuella_j2k_inspection_component_info(inspection, 1, &mut component, ptr::null_mut(),),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );

        let mut workspace = ptr::null_mut();
        assert_eq!(
            emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        let request = EmuellaJ2kDecodeRequestV0 {
            struct_size: size_of::<EmuellaJ2kDecodeRequestV0>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
            reserved: 0,
            component: 0,
            max_quality_layers: 0,
            x: 1,
            y: 1,
            width: 2,
            height: 2,
            discard_levels: 0,
            reserved_bytes: [0; 7],
        };
        for _ in 0..2 {
            let mut image = ptr::null_mut();
            assert_eq!(
                emuella_j2k_decode_component_region(
                    decoder,
                    workspace,
                    &request,
                    &mut image,
                    ptr::null_mut(),
                ),
                EMUELLA_J2K_STATUS_OK
            );
            let mut decoded_component = EmuellaJ2kComponentInfoV0::default();
            assert_eq!(
                emuella_j2k_image_component_info(image, &mut decoded_component, ptr::null_mut(),),
                EMUELLA_J2K_STATUS_OK
            );
            assert_eq!(
                (
                    decoded_component.width,
                    decoded_component.height,
                    decoded_component.x_origin,
                    decoded_component.y_origin,
                ),
                (2, 2, 1, 1)
            );
            let mut too_small = [0xa5; 3];
            assert_eq!(
                emuella_j2k_image_copy(
                    image,
                    too_small.as_mut_ptr(),
                    too_small.len(),
                    2,
                    ptr::null_mut(),
                ),
                EMUELLA_J2K_STATUS_INVALID_ARGUMENT
            );
            assert_eq!(too_small, [0xa5; 3]);
            let mut actual = [0_u8; 4];
            assert_eq!(
                emuella_j2k_image_copy(
                    image,
                    actual.as_mut_ptr(),
                    actual.len(),
                    2,
                    ptr::null_mut(),
                ),
                EMUELLA_J2K_STATUS_OK
            );
            assert_eq!(
                actual,
                [expected[5], expected[6], expected[9], expected[10]]
            );
            emuella_j2k_image_destroy(image);
        }
        emuella_j2k_inspection_destroy(inspection);
        emuella_j2k_workspace_destroy(workspace);
        emuella_j2k_decoder_destroy(decoder);
    }

    #[test]
    fn reversible_mct_region_preserves_the_one_plane_abi() {
        let fixture = emuella_j2k_test_support::native_planes::reversible_mct_region_fixture();
        let mut source = Box::new(TestSource {
            bytes: fixture.tnsot_zero,
            fail_reads: false,
        });
        let decoder = decoder(&mut source);
        let mut workspace = ptr::null_mut();
        assert_eq!(
            emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        let request = EmuellaJ2kDecodeRequestV0 {
            struct_size: size_of::<EmuellaJ2kDecodeRequestV0>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
            reserved: 0,
            component: 2,
            max_quality_layers: 0,
            x: 61,
            y: 63,
            width: 7,
            height: 5,
            discard_levels: 0,
            reserved_bytes: [0; 7],
        };
        let mut image = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decode_component_region(
                decoder,
                workspace,
                &request,
                &mut image,
                ptr::null_mut(),
            ),
            EMUELLA_J2K_STATUS_OK
        );
        let mut info = EmuellaJ2kImageInfoV0::default();
        assert_eq!(
            emuella_j2k_image_info(image, &mut info, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((info.width, info.height, info.component_count), (7, 5, 1));
        let mut component = EmuellaJ2kComponentInfoV0::default();
        assert_eq!(
            emuella_j2k_image_component_info(image, &mut component, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!(
            (
                component.source_component,
                component.width,
                component.height
            ),
            (2, 7, 5)
        );
        let mut actual = [0_u8; 35];
        assert_eq!(
            emuella_j2k_image_copy(image, actual.as_mut_ptr(), actual.len(), 7, ptr::null_mut(),),
            EMUELLA_J2K_STATUS_OK
        );
        let mut expected = Vec::new();
        for y in 63_usize..68 {
            expected.extend_from_slice(
                &fixture.planes[2]
                    [y * fixture.width as usize + 61..y * fixture.width as usize + 68],
            );
        }
        assert_eq!(actual.as_slice(), expected);
        emuella_j2k_image_destroy(image);
        emuella_j2k_workspace_destroy(workspace);
        emuella_j2k_decoder_destroy(decoder);
    }

    #[test]
    fn inspection_exposes_heterogeneous_component_metadata() {
        let mut source = heterogeneous_fixture();
        let decoder = decoder(&mut source);
        let mut inspection = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        let expected = [(8, 0, 1, 1, 4, 4), (12, 1, 2, 1, 2, 4), (16, 0, 2, 2, 2, 2)];
        for (index, expected) in expected.into_iter().enumerate() {
            let mut component = EmuellaJ2kComponentInfoV0::default();
            assert_eq!(
                emuella_j2k_inspection_component_info(
                    inspection,
                    u16::try_from(index).unwrap(),
                    &mut component,
                    ptr::null_mut(),
                ),
                EMUELLA_J2K_STATUS_OK
            );
            assert_eq!(
                (
                    component.bits_per_sample,
                    component.is_signed,
                    component.horizontal_separation,
                    component.vertical_separation,
                    component.width,
                    component.height,
                ),
                expected
            );
        }
        emuella_j2k_inspection_destroy(inspection);
        emuella_j2k_decoder_destroy(decoder);
    }

    #[test]
    fn callback_failure_is_source_io_with_owned_diagnostic() {
        let (mut source, _) = fixture();
        source.fail_reads = true;
        let decoder = decoder(&mut source);
        let mut inspection = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_inspect(decoder, &mut inspection, &mut error),
            EMUELLA_J2K_STATUS_SOURCE_IO
        );
        assert!(inspection.is_null());
        assert_eq!(
            emuella_j2k_error_status(error),
            EMUELLA_J2K_STATUS_SOURCE_IO
        );
        let mut required = 0;
        assert_eq!(
            emuella_j2k_error_message_size(error, &mut required),
            EMUELLA_J2K_STATUS_OK
        );
        let mut message = vec![0; required];
        assert_eq!(
            emuella_j2k_error_message_copy(error, message.as_mut_ptr(), message.len()),
            EMUELLA_J2K_STATUS_OK
        );
        let message = std::ffi::CStr::from_bytes_with_nul(&message).unwrap();
        assert!(message.to_string_lossy().contains("byte 0"));
        assert!(message.to_string_lossy().contains("status 91"));
        emuella_j2k_error_destroy(error);
        emuella_j2k_decoder_destroy(decoder);
    }

    #[test]
    fn malformed_later_marker_is_invalid_input_not_source_io() {
        let (mut malformed, _) = fixture();
        let eoc_prefix = malformed.bytes.len() - 2;
        assert_eq!(&malformed.bytes[eoc_prefix..], &[0xff, 0xd9]);
        malformed.bytes[eoc_prefix] = 0xfe;
        let decoder = decoder(&mut malformed);
        let mut inspection = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_inspect(decoder, &mut inspection, &mut error),
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        assert!(inspection.is_null());
        assert_eq!(
            emuella_j2k_error_status(error),
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        emuella_j2k_error_destroy(error);
        emuella_j2k_decoder_destroy(decoder);
    }

    #[test]
    fn malformed_input_and_undersized_structures_are_rejected() {
        let (mut malformed, _) = fixture();
        malformed.bytes[1] = 0x50;
        let decoder = decoder(&mut malformed);
        let mut inspection = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()),
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        emuella_j2k_decoder_destroy(decoder);

        let header = AbiHeader {
            struct_size: size_of::<AbiHeader>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
        };
        let sentinel = ptr::dangling_mut::<EmuellaJ2kDecoder>();
        let mut output = sentinel;
        assert_eq!(
            emuella_j2k_decoder_create(
                ptr::from_ref(&header).cast::<EmuellaJ2kSourceV0>(),
                &mut output,
                ptr::null_mut(),
            ),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert!(output.is_null());
    }

    #[test]
    fn panic_is_contained_and_workspace_is_poisoned() {
        let (mut source, _) = fixture();
        let decoder = decoder(&mut source);
        let mut workspace = ptr::null_mut();
        assert_eq!(
            emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
            EMUELLA_J2K_STATUS_OK
        );
        let request = EmuellaJ2kDecodeRequestV0 {
            struct_size: size_of::<EmuellaJ2kDecodeRequestV0>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
            reserved: 0,
            component: 0,
            max_quality_layers: 0,
            x: 0,
            y: 0,
            width: 4,
            height: 4,
            discard_levels: 0,
            reserved_bytes: [0; 7],
        };
        FORCE_DECODE_PANIC.with(|force| force.set(true));
        let mut image = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert_eq!(
            emuella_j2k_decode_component_region(
                decoder, workspace, &request, &mut image, &mut error,
            ),
            EMUELLA_J2K_STATUS_PANIC
        );
        assert!(image.is_null());
        assert!(!error.is_null());
        assert_eq!(emuella_j2k_error_status(error), EMUELLA_J2K_STATUS_PANIC);
        emuella_j2k_error_destroy(error);
        assert_eq!(
            emuella_j2k_decode_component_region(
                decoder,
                workspace,
                &request,
                &mut image,
                ptr::null_mut(),
            ),
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        emuella_j2k_workspace_destroy(workspace);
        emuella_j2k_decoder_destroy(decoder);
    }
}
