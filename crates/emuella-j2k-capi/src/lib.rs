//! Experimental C ABI for raw JPEG 2000 Part 1 positioned sources.
#![doc = include_str!("rust-safety.md")]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(panic = "abort")]
compile_error!("emuella-j2k-capi requires panic=unwind for ABI containment");

use std::ffi::{c_char, c_void};
use std::mem::{align_of, size_of};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use emuella_j2k::codestream::source::{CodestreamSource, SourceError, SourceErrorKind};
use emuella_j2k::{
    ComponentInfo, Image, ImageData, ImageInfo, ImageViewMut, J2kError, Part1DecodeWorkspace,
    Part1SourceIndex, Part1SourceIndexLimits, PlaneMut, SampleEndian, SampleFormat,
    execute_prepared_part1_decode_into_with_workspace, inspect_part1_source,
    prepare_part1_decode_from_source,
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
/// Explicit positive ceilings for required retained Part 1 source indexing.
/// Marker bytes exclude metadata overhead; counts bound retained descriptors.
/// Zero ceilings or a non-zero reserved field are invalid arguments.
pub struct EmuellaJ2kSourceIndexOptionsV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub max_header_bytes: u64,
    pub max_markers: u32,
    pub max_tile_parts: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// Reference-image properties or decoded image properties.
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
#[derive(Clone, Copy, Default)]
/// One to four distinct components in caller order, sharing one regional plan.
/// The four-component bound belongs to this ABI, not codec admission. Unused
/// component slots and reserved fields must be zero. Collection is opt-in.
pub struct EmuellaJ2kDecodeComponentsRequestV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    pub component_count: u16,
    /// Zero means all quality layers; otherwise the leading layer count.
    pub max_quality_layers: u16,
    pub components: [u16; 4],
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub discard_levels: u8,
    /// Zero disables observations; one collects execution work counters.
    pub collect_work: u8,
    pub reserved_bytes: [u8; 6],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
/// Immutable observations for one successful decode with collect_work enabled.
/// Work includes unrequested MCT dependencies. Capacities are retained storage,
/// not allocation counts or per-call growth. Source callback bytes are excluded.
pub struct EmuellaJ2kDecodeWorkV0 {
    pub struct_size: usize,
    pub abi_version: u32,
    pub reserved: u32,
    /// Exactly one preparation per successful call; plans are not cached.
    pub preparation_count: u64,
    pub code_blocks_decoded: u64,
    pub tier1_coefficients: u64,
    pub dwt_samples: u64,
    pub synthesis_coefficients_loaded: u64,
    pub synthesis_horizontal_values: u64,
    pub synthesis_vertical_values: u64,
    pub synthesis_lifting_updates: u64,
    pub synthesis_output_samples: u64,
    pub windowed_synthesis_component_tiles: u64,
    pub full_synthesis_component_tiles: u64,
    /// Logical bytes requested for the new C ABI owned output-plane buffers.
    /// Excludes descriptors, plans, workspace, allocator metadata and copies.
    pub output_allocation_bytes: u64,
    /// Actual combined capacity in bytes of those output-plane buffers.
    pub output_capacity_bytes: u64,
    /// Largest retained code-block coefficient capacity, in sample slots.
    pub coefficient_capacity: u64,
    /// Retained fragmented codeword assembly capacity, in bytes.
    pub segment_capacity: u64,
    /// Largest retained tile-axis transform scratch capacity, in sample slots.
    pub transform_capacity: u64,
    /// Largest retained full coefficient-plane capacity, in sample slots.
    pub full_coefficient_plane_capacity: u64,
    /// Largest retained full-transform scratch capacity, in sample slots.
    pub full_transform_scratch_capacity: u64,
    /// Successful output-plane buffer reservation requests in this C ABI call.
    /// Excludes all other allocations; this is not a process allocator count.
    pub output_allocation_count: u64,
    /// Capacity-based heap bytes retained by the complete workspace after this
    /// execution, including private workers; excludes allocator metadata.
    pub workspace_retained_heap_bytes: u64,
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
    index_limits: Option<Part1SourceIndexLimits>,
    index: OnceLock<Part1SourceIndex<CallbackSource>>,
    index_initialisation: Mutex<()>,
}

impl DecoderState {
    fn index(&self) -> Result<&Part1SourceIndex<CallbackSource>, AbiFailure> {
        if let Some(index) = self.index.get() {
            return Ok(index);
        }
        // Only construction is serialised. A failed attempt publishes nothing;
        // transient callback failures can be retried. A contained construction panic
        // also leaves no partial index, so a poisoned construction lock is safe
        // to recover. Successful reads and decodes share immutable metadata.
        let _guard = self
            .index_initialisation
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        if let Some(index) = self.index.get() {
            return Ok(index);
        }
        let limits = self.index_limits.ok_or_else(|| {
            AbiFailure::invalid("decoder did not request retained source indexing")
        })?;
        let index =
            Part1SourceIndex::new_with_limits(self.source, limits).map_err(AbiFailure::from)?;
        Ok(self.index.get_or_init(|| index))
    }
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
    work: Option<EmuellaJ2kDecodeWorkV0>,
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

/// # Safety
/// For non-null aligned pointers, the caller supplies a readable, initialised T, immutable
/// during the read.
unsafe fn checked_read<T: Copy>(pointer: *const T, name: &'static str) -> Result<T, AbiFailure> {
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

/// # Safety
/// For non-null aligned pointers, the caller supplies exclusive writable storage for one T,
/// disjoint from all inputs.
unsafe fn checked_write<T>(
    pointer: *mut T,
    value: T,
    name: &'static str,
) -> Result<(), AbiFailure> {
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

/// # Safety
/// The caller supplies a null/rejected pointer or the exact live Box<S> handle, with no
/// destruction or incompatible access for the returned borrow's lifetime.
unsafe fn handle_ref<'a, O, S>(pointer: *const O, name: &'static str) -> Result<&'a S, AbiFailure> {
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

/// # Safety
/// The caller transfers the exact live Box<S> allocation once, after all uses quiesce; null is
/// permitted.
unsafe fn destroy_handle<O, S>(pointer: *mut O) {
    if pointer.is_null() {
        return;
    }
    // SAFETY: the opaque ownership contract requires this to be the exact live
    // pointer returned by `Box<S>`, destroyed once after all calls quiesce. The
    // pointee layout is private, no foreign allocation is reconstructed, and
    // `Box::from_raw` immediately restores Rust ownership for one drop.
    unsafe { drop(Box::from_raw(pointer.cast::<S>())) };
}

/// # Safety
/// The caller supplies exclusive writable storage for source.len() bytes, disjoint from source;
/// null is rejected for a non-empty copy.
unsafe fn checked_copy(destination: *mut u8, source: &[u8]) -> Result<(), AbiFailure> {
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

/// # Safety
/// The caller supplies null or exclusive writable storage for one error pointer, disjoint from
/// inputs and other outputs.
unsafe fn write_error(
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
    // SAFETY: The export contract supplies exclusive, disjoint output storage for this value.
    unsafe { checked_write(output, value, "error_output") }
}

/// # Safety
/// The caller supplies a valid optional error output and a null or live workspace kept alive
/// through panic recovery; operation preserves these obligations.
unsafe fn boundary<F>(
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
        // SAFETY: The caller reserves this optional, disjoint error slot for the complete
        // boundary call.
        unsafe { write_error(error_output, None) }?;
        operation()
    }));
    match result {
        Ok(Ok(())) => EMUELLA_J2K_STATUS_OK,
        Ok(Err(failure)) => {
            let status = failure.status;
            match catch_unwind(AssertUnwindSafe(|| {
                // SAFETY: The caller reserves this optional, disjoint error slot for the
                // complete boundary call.
                unsafe { write_error(error_output, Some(&failure)) }
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
                // SAFETY: The caller keeps the workspace live through panic recovery.
                if let Ok(workspace) = unsafe {
                    handle_ref::<EmuellaJ2kWorkspace, WorkspaceState>(poison_workspace, "workspace")
                } {
                    workspace.poisoned.store(true, Ordering::Release);
                }
                let failure = AbiFailure {
                    status: EMUELLA_J2K_STATUS_PANIC,
                    message: "contained Rust panic".into(),
                };
                // SAFETY: The caller reserves this optional, disjoint error slot for the
                // complete boundary call.
                let _ = unsafe { write_error(error_output, Some(&failure)) };
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
///
/// # Safety
/// `source` must contain a readable, initialised size/version prefix; if it
/// advertises the supported full size, the complete source structure must be
/// readable and initialised. The descriptor is copied. Its callback, context,
/// stable length and immutable source bytes must remain valid until the returned
/// decoder is destroyed and all its operations finish. The callback must support
/// concurrent calls, fill each successful requested range, retain no destination,
/// neither re-enter related handles nor destroy or mutate any Emuella handle,
/// and return normally without unwinding. `output` must provide writable storage for one value
/// of its declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_decoder_create(
    source: *const EmuellaJ2kSourceV0,
    output: *mut *mut EmuellaJ2kDecoder,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, ptr::null_mut(), "decoder_output") }?;
        // SAFETY: The export supplies the complete source descriptor and callback lifetime.
        let decoder = unsafe { decoder_from_source(source, None) }?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe {
            checked_write(
                output,
                Box::into_raw(decoder).cast::<EmuellaJ2kDecoder>(),
                "decoder_output",
            )
        }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

// SAFETY: The caller supplies a readable source prefix and, after size
// validation, a complete source descriptor; the callback contract remains valid
// for the lifetime of the returned decoder. This helper publishes no raw handle.
unsafe fn decoder_from_source(
    source: *const EmuellaJ2kSourceV0,
    index_limits: Option<Part1SourceIndexLimits>,
) -> Result<Box<DecoderState>, AbiFailure> {
    // SAFETY: The export contract supplies initialised input storage; size validation
    // precedes full-structure reads.
    let header = unsafe { checked_read(source.cast::<AbiHeader>(), "source") }?;
    validate_header(
        header.struct_size,
        header.abi_version,
        size_of::<EmuellaJ2kSourceV0>(),
    )?;
    // SAFETY: The export contract supplies initialised input storage; size validation
    // precedes full-structure reads.
    let source = unsafe { checked_read(source, "source") }?;
    if source.reserved != 0 {
        return Err(AbiFailure::invalid("source reserved field must be zero"));
    }
    let read_at = source
        .read_at
        .ok_or_else(|| AbiFailure::invalid("source read_at callback is required"))?;
    let decoder = Box::new(DecoderState {
        index_limits,
        index: OnceLock::new(),
        index_initialisation: Mutex::new(()),
        source: CallbackSource {
            context_address: source.context.expose_provenance(),
            length: source.length,
            read_at,
        },
    });
    Ok(decoder)
}

#[unsafe(no_mangle)]
/// Create a decoder that requires a retained Part 1 source index with explicit limits.
///
/// Creation validates and copies descriptors without reading source bytes. The
/// first inspection or decode builds the index; exceeding any ceiling fails as
/// unsupported input, without falling back to repeated header traversal. A
/// successful index is shared by subsequent inspection and regional requests.
/// Existing `emuella_j2k_decoder_create` retains its one-shot source semantics.
///
/// # Safety
/// All source, callback, context, output and error-output obligations of
/// `emuella_j2k_decoder_create` apply. `options` must provide a readable,
/// initialised size/version prefix and, when the full size is advertised, the
/// complete initialised options structure. Its storage must remain valid and
/// correctly aligned for this call, and disjoint from writable outputs.
pub unsafe extern "C" fn emuella_j2k_decoder_create_indexed(
    source: *const EmuellaJ2kSourceV0,
    options: *const EmuellaJ2kSourceIndexOptionsV0,
    output: *mut *mut EmuellaJ2kDecoder,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export supplies exclusive, disjoint output storage.
        unsafe { checked_write(output, ptr::null_mut(), "decoder_output") }?;
        // SAFETY: The export supplies a readable options prefix before full-size validation.
        let header = unsafe { checked_read(options.cast::<AbiHeader>(), "index_options") }?;
        validate_header(
            header.struct_size,
            header.abi_version,
            size_of::<EmuellaJ2kSourceIndexOptionsV0>(),
        )?;
        // SAFETY: The export supplies the full readable options structure after size validation.
        let options = unsafe { checked_read(options, "index_options") }?;
        if options.reserved != 0
            || options.max_header_bytes == 0
            || options.max_markers == 0
            || options.max_tile_parts == 0
        {
            return Err(AbiFailure::invalid(
                "index ceilings must be positive and reserved must be zero",
            ));
        }
        let limits = Part1SourceIndexLimits {
            header_bytes: usize::try_from(options.max_header_bytes).map_err(|_| {
                AbiFailure::invalid("index header ceiling exceeds addressable memory")
            })?,
            markers: usize::try_from(options.max_markers).map_err(|_| {
                AbiFailure::invalid("index marker ceiling exceeds addressable memory")
            })?,
            tile_parts: usize::try_from(options.max_tile_parts).map_err(|_| {
                AbiFailure::invalid("index tile-part ceiling exceeds addressable memory")
            })?,
        };
        // SAFETY: The export supplies the complete source descriptor and callback lifetime.
        let decoder = unsafe { decoder_from_source(source, Some(limits)) }?;
        // SAFETY: The export supplies exclusive, disjoint output storage for the new handle.
        unsafe {
            checked_write(
                output,
                Box::into_raw(decoder).cast::<EmuellaJ2kDecoder>(),
                "decoder_output",
            )
        }
    };
    // SAFETY: The export reserves the error slot; the closure upholds pointer obligations.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Destroy a decoder after all calls and callbacks using it have quiesced.
///
/// # Safety
/// `decoder` must be null or the exact live decoder handle returned by this
/// library. A non-null handle transfers ownership back exactly once; all calls,
/// callbacks and borrowed observations using it must have quiesced. It must not
/// be used again or destroyed concurrently.
pub unsafe extern "C" fn emuella_j2k_decoder_destroy(decoder: *mut EmuellaJ2kDecoder) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller transfers this matching handle exactly once after all uses have
        // quiesced.
        unsafe { destroy_handle::<EmuellaJ2kDecoder, DecoderState>(decoder) }
    }));
}

#[unsafe(no_mangle)]
/// Inspect raw Part 1 geometry without decoding packet bodies.
///
/// # Safety
/// `decoder` must be null or an exact live decoder from this library, kept
/// alive for the call. Its creation-time source and callback obligations still
/// apply, including concurrent callback safety. `output` must provide writable storage for one
/// value of its declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_decoder_inspect(
    decoder: *const EmuellaJ2kDecoder,
    output: *mut *mut EmuellaJ2kInspection,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, ptr::null_mut(), "inspection_output") }?;
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let decoder = unsafe { handle_ref::<EmuellaJ2kDecoder, DecoderState>(decoder, "decoder") }?;
        let inspected = if decoder.index_limits.is_some() {
            decoder.index()?.inspect()
        } else {
            inspect_part1_source(&decoder.source)
        }
        .map_err(AbiFailure::from)?;
        let inspection = Box::new(InspectionState {
            image: inspected.image,
            components: inspected.components,
        });
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe {
            checked_write(
                output,
                Box::into_raw(inspection).cast::<EmuellaJ2kInspection>(),
                "inspection_output",
            )
        }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Destroy an immutable inspection handle.
///
/// # Safety
/// `inspection` must be null or the exact live inspection handle returned by this
/// library. A non-null handle transfers ownership back exactly once; all calls,
/// callbacks and borrowed observations using it must have quiesced. It must not
/// be used again or destroyed concurrently.
pub unsafe extern "C" fn emuella_j2k_inspection_destroy(inspection: *mut EmuellaJ2kInspection) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller transfers this matching handle exactly once after all uses have
        // quiesced.
        unsafe { destroy_handle::<EmuellaJ2kInspection, InspectionState>(inspection) }
    }));
}

#[unsafe(no_mangle)]
/// Copy reference-image properties into caller-owned storage.
///
/// # Safety
/// `inspection` must be null or an exact live inspection handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_inspection_image_info(
    inspection: *const EmuellaJ2kInspection,
    output: *mut EmuellaJ2kImageInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let inspection = unsafe {
            handle_ref::<EmuellaJ2kInspection, InspectionState>(inspection, "inspection")
        }?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, image_info(&inspection.image), "image_info_output") }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Copy one inspected component descriptor into caller-owned storage.
///
/// # Safety
/// `inspection` must be null or an exact live inspection handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_inspection_component_info(
    inspection: *const EmuellaJ2kInspection,
    component: u16,
    output: *mut EmuellaJ2kComponentInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let inspection = unsafe {
            handle_ref::<EmuellaJ2kInspection, InspectionState>(inspection, "inspection")
        }?;
        let component = inspection
            .components
            .get(usize::from(component))
            .ok_or_else(|| AbiFailure::invalid("component index is out of bounds"))?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, component_info(component)?, "component_info_output") }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Create an exclusively used, reusable decode workspace.
///
/// # Safety
/// `output` must provide writable storage for one value of its declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_workspace_create(
    output: *mut *mut EmuellaJ2kWorkspace,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, ptr::null_mut(), "workspace_output") }?;
        let workspace = Box::new(WorkspaceState {
            poisoned: AtomicBool::new(false),
            inner: Mutex::new(Part1DecodeWorkspace::new()),
        });
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe {
            checked_write(
                output,
                Box::into_raw(workspace).cast::<EmuellaJ2kWorkspace>(),
                "workspace_output",
            )
        }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Destroy an idle workspace, including a workspace poisoned by panic.
///
/// # Safety
/// `workspace` must be null or the exact live workspace handle returned by this
/// library. A non-null handle transfers ownership back exactly once; all calls,
/// callbacks and borrowed observations using it must have quiesced. It must not
/// be used again or destroyed concurrently.
pub unsafe extern "C" fn emuella_j2k_workspace_destroy(workspace: *mut EmuellaJ2kWorkspace) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller transfers this matching handle exactly once after all uses have
        // quiesced.
        unsafe { destroy_handle::<EmuellaJ2kWorkspace, WorkspaceState>(workspace) }
    }));
}

#[unsafe(no_mangle)]
/// Decode one component region into a new immutable Rust-owned image.
///
/// # Safety
/// `decoder` and `workspace` must be null or exact live handles of their
/// respective types from this library, kept alive for the call. No other active
/// operation may use this workspace. The decoder creation-time source and
/// callback obligations still apply, including concurrent callback safety.
/// `request` must contain a readable, initialised size/version prefix and, when
/// it advertises the supported full size, the complete initialised request. `output` must
/// provide writable storage for one value of its declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_decode_component_region(
    decoder: *const EmuellaJ2kDecoder,
    workspace: *const EmuellaJ2kWorkspace,
    request: *const EmuellaJ2kDecodeRequestV0,
    output: *mut *mut EmuellaJ2kImage,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, ptr::null_mut(), "image_output") }?;
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let decoder = unsafe { handle_ref::<EmuellaJ2kDecoder, DecoderState>(decoder, "decoder") }?;
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let workspace =
            unsafe { handle_ref::<EmuellaJ2kWorkspace, WorkspaceState>(workspace, "workspace") }?;
        if workspace.poisoned.load(Ordering::Acquire) {
            return Err(AbiFailure::invalid(
                "workspace is poisoned and may only be destroyed",
            ));
        }
        // SAFETY: The export contract supplies initialised input storage; size validation
        // precedes full-structure reads.
        let header = unsafe { checked_read(request.cast::<AbiHeader>(), "request") }?;
        validate_header(
            header.struct_size,
            header.abi_version,
            size_of::<EmuellaJ2kDecodeRequestV0>(),
        )?;
        // SAFETY: The export contract supplies initialised input storage; size validation
        // precedes full-structure reads.
        let request = unsafe { checked_read(request, "request") }?;
        if request.reserved != 0 || request.reserved_bytes != [0; 7] {
            return Err(AbiFailure::invalid("request reserved fields must be zero"));
        }
        if request.width == 0 || request.height == 0 {
            return Err(AbiFailure::invalid(
                "request width and height must be non-zero",
            ));
        }
        let image = decode_components(
            decoder,
            workspace,
            emuella_j2k::codestream::Part1ComponentDecodeRequest {
                component_indices: &[request.component],
                region: emuella_j2k::codestream::TileRegionRequest {
                    x: request.x,
                    y: request.y,
                    width: request.width,
                    height: request.height,
                },
                discard_levels: request.discard_levels,
                max_layers: (request.max_quality_layers != 0).then_some(request.max_quality_layers),
            },
            false,
        )?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe {
            checked_write(
                output,
                Box::into_raw(image).cast::<EmuellaJ2kImage>(),
                "image_output",
            )
        }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, workspace, operation) }
}

#[unsafe(no_mangle)]
/// Decode selected components of one region into a new immutable Rust-owned image.
/// Planes follow request order. Failure clears output to null; workspace scratch
/// may grow on failure. No partially decoded image is published.
///
/// # Safety
/// `decoder` and `workspace` must be null or exact live handles of their
/// respective types from this library, kept alive for the call. No other active
/// operation may use this workspace. The decoder creation-time source and
/// callback obligations still apply, including concurrent callback safety.
/// `request` must contain a readable, initialised size/version prefix and, when
/// it advertises the supported full size, the complete initialised request. `output` must
/// provide writable storage for one value of its declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_decode_components_region(
    decoder: *const EmuellaJ2kDecoder,
    workspace: *const EmuellaJ2kWorkspace,
    request: *const EmuellaJ2kDecodeComponentsRequestV0,
    output: *mut *mut EmuellaJ2kImage,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, ptr::null_mut(), "image_output") }?;
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let decoder = unsafe { handle_ref::<EmuellaJ2kDecoder, DecoderState>(decoder, "decoder") }?;
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let workspace =
            unsafe { handle_ref::<EmuellaJ2kWorkspace, WorkspaceState>(workspace, "workspace") }?;
        if workspace.poisoned.load(Ordering::Acquire) {
            return Err(AbiFailure::invalid(
                "workspace is poisoned and may only be destroyed",
            ));
        }
        // SAFETY: The export contract supplies initialised input storage; size validation
        // precedes full-structure reads.
        let header = unsafe { checked_read(request.cast::<AbiHeader>(), "request") }?;
        validate_header(
            header.struct_size,
            header.abi_version,
            size_of::<EmuellaJ2kDecodeComponentsRequestV0>(),
        )?;
        // SAFETY: The export contract supplies initialised input storage; size validation
        // precedes full-structure reads.
        let request = unsafe { checked_read(request, "request") }?;
        if request.reserved != 0 || request.reserved_bytes != [0; 6] {
            return Err(AbiFailure::invalid("request reserved fields must be zero"));
        }
        if request.width == 0 || request.height == 0 {
            return Err(AbiFailure::invalid(
                "request width and height must be non-zero",
            ));
        }
        if request.collect_work > 1 {
            return Err(AbiFailure::invalid("collect_work must be zero or one"));
        }
        let count = usize::from(request.component_count);
        if !(1..=4).contains(&count) {
            return Err(AbiFailure::invalid(
                "component count must be between one and four",
            ));
        }
        if request.components[count..]
            .iter()
            .any(|&component| component != 0)
        {
            return Err(AbiFailure::invalid("unused component slots must be zero"));
        }
        let indices = &request.components[..count];
        if indices
            .iter()
            .enumerate()
            .any(|(index, component)| indices[..index].contains(component))
        {
            return Err(AbiFailure::invalid("component indices must be distinct"));
        }
        let image = decode_components(
            decoder,
            workspace,
            emuella_j2k::codestream::Part1ComponentDecodeRequest {
                component_indices: indices,
                region: emuella_j2k::codestream::TileRegionRequest {
                    x: request.x,
                    y: request.y,
                    width: request.width,
                    height: request.height,
                },
                discard_levels: request.discard_levels,
                max_layers: (request.max_quality_layers != 0).then_some(request.max_quality_layers),
            },
            request.collect_work == 1,
        )?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe {
            checked_write(
                output,
                Box::into_raw(image).cast::<EmuellaJ2kImage>(),
                "image_output",
            )
        }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, workspace, operation) }
}

fn decode_components(
    decoder: &DecoderState,
    workspace: &WorkspaceState,
    request: emuella_j2k::codestream::Part1ComponentDecodeRequest<'_>,
    collect_work: bool,
) -> Result<Box<ImageState>, AbiFailure> {
    let prepared = if decoder.index_limits.is_some() {
        decoder.index()?.prepare(request)
    } else {
        prepare_part1_decode_from_source(&decoder.source, request)
    }
    .map_err(AbiFailure::from)?;
    let info = prepared.info().clone();
    let components = prepared.component_info().to_vec();
    let mut samples = Vec::with_capacity(components.len());
    let mut strides = Vec::with_capacity(components.len());
    for component in &components {
        let sample_bytes = usize::from(component.sample_format.bits_per_sample).div_ceil(8);
        let stride = usize::try_from(component.width)
            .ok()
            .and_then(|width| width.checked_mul(sample_bytes))
            .ok_or_else(|| AbiFailure {
                status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
                message: "decoded row byte size overflowed".into(),
            })?;
        let length = usize::try_from(component.height)
            .ok()
            .and_then(|height| stride.checked_mul(height))
            .ok_or_else(|| AbiFailure {
                status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
                message: "decoded image byte size overflowed".into(),
            })?;
        let mut plane = Vec::new();
        plane.try_reserve_exact(length).map_err(|_| AbiFailure {
            status: EMUELLA_J2K_STATUS_RESOURCE_LIMIT,
            message: "decoded image allocation failed".into(),
        })?;
        plane.resize(length, 0);
        samples.push(plane);
        strides.push(stride);
    }
    let mut planes = samples
        .iter_mut()
        .zip(&components)
        .zip(strides)
        .map(|((samples, component), stride)| {
            PlaneMut::new(
                samples,
                component.width,
                component.height,
                stride,
                component.sample_format,
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(AbiFailure::from)?;
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
    let timings = execute_prepared_part1_decode_into_with_workspace(
        &prepared,
        &mut target,
        &mut workspace,
        emuella_j2k::codestream::PreparedPart1ExecutionOptions {
            instrumentation: if collect_work {
                emuella_j2k::codestream::DecodeInstrumentation::WorkCounters
            } else {
                emuella_j2k::codestream::DecodeInstrumentation::None
            },
            ..Default::default()
        },
    )
    .map_err(AbiFailure::from)?;
    let work = collect_work.then(|| EmuellaJ2kDecodeWorkV0 {
        struct_size: size_of::<EmuellaJ2kDecodeWorkV0>(),
        abi_version: EMUELLA_J2K_ABI_VERSION,
        reserved: 0,
        preparation_count: 1,
        code_blocks_decoded: timings.code_blocks_decoded,
        tier1_coefficients: timings.tier1_coefficients,
        dwt_samples: timings.dwt_samples,
        synthesis_coefficients_loaded: timings.synthesis_coefficients_loaded,
        synthesis_horizontal_values: timings.synthesis_horizontal_values,
        synthesis_vertical_values: timings.synthesis_vertical_values,
        synthesis_lifting_updates: timings.synthesis_lifting_updates,
        synthesis_output_samples: timings.synthesis_output_samples,
        windowed_synthesis_component_tiles: timings.windowed_synthesis_component_tiles,
        full_synthesis_component_tiles: timings.full_synthesis_component_tiles,
        output_allocation_bytes: samples.iter().map(|plane| plane.len() as u64).sum(),
        output_capacity_bytes: samples.iter().map(|plane| plane.capacity() as u64).sum(),
        coefficient_capacity: workspace.coefficient_capacity() as u64,
        segment_capacity: workspace.segment_capacity() as u64,
        transform_capacity: workspace.transform_capacity() as u64,
        full_coefficient_plane_capacity: workspace.full_coefficient_plane_capacity() as u64,
        full_transform_scratch_capacity: workspace.full_transform_scratch_capacity() as u64,
        output_allocation_count: samples.len() as u64,
        workspace_retained_heap_bytes: workspace.retained_heap_bytes(),
    });
    Ok(Box::new(ImageState {
        image: Image {
            info,
            component_info: components,
            data: ImageData::Planes(samples),
        },
        work,
    }))
}

#[unsafe(no_mangle)]
/// Destroy an immutable decoded image.
///
/// # Safety
/// `image` must be null or the exact live image handle returned by this
/// library. A non-null handle transfers ownership back exactly once; all calls,
/// callbacks and borrowed observations using it must have quiesced. It must not
/// be used again or destroyed concurrently.
pub unsafe extern "C" fn emuella_j2k_image_destroy(image: *mut EmuellaJ2kImage) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller transfers this matching handle exactly once after all uses have
        // quiesced.
        unsafe { destroy_handle::<EmuellaJ2kImage, ImageState>(image) }
    }));
}

#[unsafe(no_mangle)]
/// Copy decoded image properties into caller-owned storage.
///
/// # Safety
/// `image` must be null or an exact live image handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_info(
    image: *const EmuellaJ2kImage,
    output: *mut EmuellaJ2kImageInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let image = unsafe { handle_ref::<EmuellaJ2kImage, ImageState>(image, "image") }?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, image_info(&image.image.info), "image_info_output") }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Copy opt-in work observations; return UNSUPPORTED without modifying output
/// when collection was disabled for this image.
///
/// # Safety
/// `image` must be null or an exact live image handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_decode_work(
    image: *const EmuellaJ2kImage,
    output: *mut EmuellaJ2kDecodeWorkV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let image = unsafe { handle_ref::<EmuellaJ2kImage, ImageState>(image, "image") }?;
        let work = image.work.ok_or_else(|| AbiFailure {
            status: EMUELLA_J2K_STATUS_UNSUPPORTED,
            message: "decode work collection was disabled".into(),
        })?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, work, "decode_work_output") }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

/// Copy the first decoded component descriptor into caller-owned storage.
#[unsafe(no_mangle)]
///
/// # Safety
/// `image` must be null or an exact live image handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_component_info(
    image: *const EmuellaJ2kImage,
    output: *mut EmuellaJ2kComponentInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    // SAFETY: This legacy entry point forwards the same handle and storage
    // obligations to the indexed operation for the first output component.
    unsafe { emuella_j2k_image_component_info_at(image, 0, output, error_output) }
}

#[unsafe(no_mangle)]
/// Copy the descriptor at a zero-based output position in request order.
///
/// # Safety
/// `image` must be null or an exact live image handle from this library,
/// kept alive without destruction throughout the call. Concurrent immutable
/// observations are allowed. `output` must provide writable storage for one value of its
/// declared type.
/// A non-null `error_output` must provide writable storage for one error pointer.
/// Returned handles belong to the caller and must be released exactly once with
/// their matching destroy function; output slots do not release previous handles.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_component_info_at(
    image: *const EmuellaJ2kImage,
    output_index: u16,
    output: *mut EmuellaJ2kComponentInfoV0,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let image = unsafe { handle_ref::<EmuellaJ2kImage, ImageState>(image, "image") }?;
        let component = image
            .image
            .component_info
            .get(usize::from(output_index))
            .ok_or_else(|| AbiFailure::invalid("output component index is out of bounds"))?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, component_info(component)?, "component_info_output") }
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Copy the first output component into a bounded buffer with explicit row stride.
///
/// # Safety
/// `image` must be null or an exact live image from this library, kept alive
/// throughout the copy. `destination` must provide `capacity` exclusively writable
/// bytes, disjoint from the image and all other inputs and outputs. The checked
/// stride and capacity determine the rows written. A non-null `error_output`
/// must provide writable storage for one error pointer, owned by the caller on
/// return and released once with `emuella_j2k_error_destroy`.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_copy(
    image: *const EmuellaJ2kImage,
    destination: *mut u8,
    capacity: usize,
    stride_bytes: usize,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    // SAFETY: This legacy entry point forwards the same handle and storage
    // obligations to the indexed operation for the first output component.
    unsafe {
        emuella_j2k_image_copy_component(
            image,
            0,
            destination,
            capacity,
            stride_bytes,
            error_output,
        )
    }
}

#[unsafe(no_mangle)]
/// Copy rows at a zero-based output position in request order with explicit stride.
/// Bounds failure leaves destination bytes unchanged; successful copies preserve
/// row padding. This operation does not decode directly into foreign storage.
///
/// # Safety
/// `image` must be null or an exact live image from this library, kept alive
/// throughout the copy. `destination` must provide `capacity` exclusively writable
/// bytes, disjoint from the image and all other inputs and outputs. The checked
/// stride and capacity determine the rows written. A non-null `error_output`
/// must provide writable storage for one error pointer, owned by the caller on
/// return and released once with `emuella_j2k_error_destroy`.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_image_copy_component(
    image: *const EmuellaJ2kImage,
    output_index: u16,
    destination: *mut u8,
    capacity: usize,
    stride_bytes: usize,
    error_output: *mut *mut EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    let operation = || {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let image = unsafe { handle_ref::<EmuellaJ2kImage, ImageState>(image, "image") }?;
        let ImageData::Planes(planes) = &image.image.data else {
            return Err(AbiFailure::invalid("image is not planar"));
        };
        let source = planes
            .get(usize::from(output_index))
            .ok_or_else(|| AbiFailure::invalid("output component index is out of bounds"))?;
        let component = image
            .image
            .component_info
            .get(usize::from(output_index))
            .ok_or_else(|| AbiFailure::invalid("output component index is out of bounds"))?;
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
            // SAFETY: Capacity and offsets were checked; the caller supplies exclusive
            // destination storage disjoint from the source.
            unsafe {
                checked_copy(
                    destination_row,
                    &source[source_start..source_start + row_bytes],
                )
            }?;
        }
        Ok(())
    };
    // SAFETY: The export contract reserves the error slot and keeps any workspace alive
    // through panic recovery. The closure upholds each individual pointer obligation.
    unsafe { boundary(error_output, ptr::null(), operation) }
}

#[unsafe(no_mangle)]
/// Destroy an immutable diagnostic handle.
///
/// # Safety
/// `error` must be null or the exact live error handle returned by this
/// library. A non-null handle transfers ownership back exactly once; all calls,
/// callbacks and borrowed observations using it must have quiesced. It must not
/// be used again or destroyed concurrently.
pub unsafe extern "C" fn emuella_j2k_error_destroy(error: *mut EmuellaJ2kError) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller transfers this matching handle exactly once after all uses have
        // quiesced.
        unsafe { destroy_handle::<EmuellaJ2kError, ErrorState>(error) }
    }));
}

#[unsafe(no_mangle)]
/// Return the status retained by an immutable diagnostic handle.
///
/// # Safety
/// `error` must be null or an exact live diagnostic from this library, with
/// no destruction during the call. Concurrent immutable observations are allowed.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_error_status(
    error: *const EmuellaJ2kError,
) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        unsafe { handle_ref::<EmuellaJ2kError, ErrorState>(error, "error") }
            .map_or(EMUELLA_J2K_STATUS_INVALID_ARGUMENT, |error| error.status)
    }))
    .unwrap_or(EMUELLA_J2K_STATUS_PANIC)
}

#[unsafe(no_mangle)]
/// Return the diagnostic byte count, including its terminating NUL byte.
///
/// # Safety
/// `error` must be null or an exact live diagnostic from this library, with
/// no destruction during the call. `output` must provide exclusive writable
/// storage for one usize, disjoint from the diagnostic allocation.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_error_message_size(
    error: *const EmuellaJ2kError,
    output: *mut usize,
) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let error = unsafe { handle_ref::<EmuellaJ2kError, ErrorState>(error, "error") }?;
        // SAFETY: The export contract supplies exclusive, disjoint output storage for this
        // value.
        unsafe { checked_write(output, error.message.len(), "message_size_output") }
    }))
    .map_or(EMUELLA_J2K_STATUS_PANIC, |result| {
        result.map_or_else(|failure| failure.status, |()| EMUELLA_J2K_STATUS_OK)
    })
}

#[unsafe(no_mangle)]
/// Copy the complete NUL-terminated UTF-8 diagnostic into caller storage.
///
/// # Safety
/// `error` must be null or an exact live diagnostic from this library, with
/// no destruction during the call. `destination` must provide `capacity`
/// exclusively writable bytes disjoint from the diagnostic allocation.
/// All non-null storage pointers must be valid for the accessed extent and
/// correctly aligned throughout this synchronous call. Writable storage must be
/// exclusively accessible and disjoint from inputs, other outputs, live handle
/// allocations and callback storage. Null or misaligned arguments are rejected
/// where checked; these checks do not establish allocation validity.
pub unsafe extern "C" fn emuella_j2k_error_message_copy(
    error: *const EmuellaJ2kError,
    destination: *mut u8,
    capacity: usize,
) -> EmuellaJ2kStatus {
    catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The caller guarantees the matching live handle remains valid for this call;
        // null and alignment are checked.
        let error = unsafe { handle_ref::<EmuellaJ2kError, ErrorState>(error, "error") }?;
        if capacity < error.message.len() {
            return Err(AbiFailure::invalid("diagnostic destination is too small"));
        }
        // SAFETY: Capacity and offsets were checked; the caller supplies exclusive destination
        // storage disjoint from the source.
        unsafe { checked_copy(destination, &error.message) }
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
        reads: Mutex<Vec<(u64, usize)>>,
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
            source.reads.lock().unwrap().push((offset, length));
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
                reads: Mutex::new(Vec::new()),
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
            reads: Mutex::new(Vec::new()),
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

    // SAFETY: the caller must keep the source allocation and bytes stable until
    // the returned decoder is destroyed and all its callbacks have finished.
    unsafe fn decoder(source: &mut TestSource) -> *mut EmuellaJ2kDecoder {
        let descriptor = source_descriptor(source);
        let mut decoder = ptr::null_mut();
        assert_eq!(
            // SAFETY: The caller keeps the source stable; the descriptor and output are
            // disjoint local storage.
            unsafe { emuella_j2k_decoder_create(&descriptor, &mut decoder, ptr::null_mut()) },
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
    fn decoder_retains_successful_index_retries_io_failure_and_reuses_it_for_new_windows() {
        let (mut source, _) = fixture();
        let payload_start = emuella_j2k::codestream::parse(&source.bytes).unwrap().tiles[0]
            .payload_offset
            .unwrap() as u64;
        let descriptor = source_descriptor(&mut source);
        let decoder = DecoderState {
            source: CallbackSource {
                context_address: descriptor.context.expose_provenance(),
                length: descriptor.length,
                read_at: descriptor.read_at.unwrap(),
            },
            index_limits: Some(Part1SourceIndexLimits::default()),
            index: OnceLock::new(),
            index_initialisation: Mutex::new(()),
        };
        source.fail_reads = true;
        assert!(matches!(
            decoder.index(),
            Err(AbiFailure {
                status: EMUELLA_J2K_STATUS_SOURCE_IO,
                ..
            })
        ));
        assert!(decoder.index.get().is_none());
        source.fail_reads = false;
        let index = decoder
            .index()
            .unwrap_or_else(|error| panic!("{}", error.message));
        assert_eq!(index.inspect().unwrap().image.width, 4);
        source.reads.lock().unwrap().clear();
        source.fail_reads = true;
        // Inspection is served entirely by immutable metadata after construction.
        assert!(
            decoder
                .index()
                .unwrap_or_else(|error| panic!("{}", error.message))
                .inspect()
                .is_ok()
        );
        assert!(source.reads.lock().unwrap().is_empty());
        source.fail_reads = false;
        let workspace = WorkspaceState {
            poisoned: AtomicBool::new(false),
            inner: Mutex::new(Part1DecodeWorkspace::new()),
        };
        for x in [0, 2] {
            let request = emuella_j2k::codestream::Part1ComponentDecodeRequest {
                component_indices: &[0],
                region: emuella_j2k::codestream::TileRegionRequest {
                    x,
                    y: 0,
                    width: 2,
                    height: 4,
                },
                discard_levels: 0,
                max_layers: None,
            };
            let image = decode_components(&decoder, &workspace, request, true)
                .unwrap_or_else(|error| panic!("{}", error.message));
            assert_eq!(image.image.info.width, 2);
        }
        assert!(!source.reads.lock().unwrap().is_empty());
        assert!(
            source
                .reads
                .lock()
                .unwrap()
                .iter()
                .all(|(offset, _)| *offset >= payload_start)
        );
    }

    #[test]
    fn indexed_constructor_validates_options_and_keeps_legacy_admission_separate() {
        let (mut source, _) = fixture();
        let descriptor = source_descriptor(&mut source);
        let valid = EmuellaJ2kSourceIndexOptionsV0 {
            struct_size: size_of::<EmuellaJ2kSourceIndexOptionsV0>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
            reserved: 0,
            max_header_bytes: 8,
            max_markers: 65_536,
            max_tile_parts: 65_536,
        };
        for options in [
            EmuellaJ2kSourceIndexOptionsV0 {
                struct_size: 0,
                ..valid
            },
            EmuellaJ2kSourceIndexOptionsV0 {
                abi_version: u32::MAX,
                ..valid
            },
            EmuellaJ2kSourceIndexOptionsV0 {
                reserved: 1,
                ..valid
            },
            EmuellaJ2kSourceIndexOptionsV0 {
                max_header_bytes: 0,
                ..valid
            },
            EmuellaJ2kSourceIndexOptionsV0 {
                max_markers: 0,
                ..valid
            },
            EmuellaJ2kSourceIndexOptionsV0 {
                max_tile_parts: 0,
                ..valid
            },
        ] {
            let mut decoder = ptr::null_mut();
            // SAFETY: Complete descriptors and disjoint outputs remain live for this call.
            assert_eq!(
                unsafe {
                    emuella_j2k_decoder_create_indexed(
                        &descriptor,
                        &options,
                        &mut decoder,
                        ptr::null_mut(),
                    )
                },
                EMUELLA_J2K_STATUS_INVALID_ARGUMENT
            );
            assert!(decoder.is_null());
        }
        assert!(source.reads.lock().unwrap().is_empty());
        let mut indexed = ptr::null_mut();
        // SAFETY: The boxed immutable bytes and callback remain live through decoder destruction.
        assert_eq!(
            unsafe {
                emuella_j2k_decoder_create_indexed(
                    &descriptor,
                    &valid,
                    &mut indexed,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_OK
        );
        assert!(source.reads.lock().unwrap().is_empty());
        let mut inspection = ptr::null_mut();
        // SAFETY: The decoder is live and the output is disjoint writable storage.
        assert_eq!(
            unsafe { emuella_j2k_decoder_inspect(indexed, &mut inspection, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_UNSUPPORTED
        );
        assert!(inspection.is_null());
        // SAFETY: No operation remains active and this transfers the handle exactly once.
        unsafe { emuella_j2k_decoder_destroy(indexed) };
        // SAFETY: The same source remains stable and valid through decoder destruction.
        let legacy = unsafe { decoder(&mut source) };
        // SAFETY: The legacy decoder is live and the output is disjoint writable storage.
        assert_eq!(
            unsafe { emuella_j2k_decoder_inspect(legacy, &mut inspection, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_OK
        );
        // SAFETY: Both handles are live and quiescent and are released exactly once.
        unsafe {
            emuella_j2k_inspection_destroy(inspection);
            emuella_j2k_decoder_destroy(legacy);
        }
    }

    #[test]
    fn invalid_null_outputs_are_rejected_without_ub() {
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(ptr::null_mut(), ptr::null_mut()) },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_error_message_size(ptr::null(), ptr::null_mut()) },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        let mut aligned_storage = [0_usize; 2];
        let misaligned_output = aligned_storage
            .as_mut_ptr()
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<*mut EmuellaJ2kWorkspace>();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(misaligned_output, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );

        let mut workspace = ptr::null_mut();
        let misaligned_error = aligned_storage
            .as_mut_ptr()
            .cast::<u8>()
            .wrapping_add(1)
            .cast::<*mut EmuellaJ2kError>();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(&mut workspace, misaligned_error) },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert!(workspace.is_null());
    }

    #[test]
    fn inspect_decode_copy_and_workspace_reuse_are_failure_atomic() {
        let (mut source, expected) = fixture();
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut source) };
        let mut inspection = ptr::null_mut();
        assert_eq!(
            // SAFETY: The source and decoder remain live; the local output slots are exclusive
            // and disjoint.
            unsafe { emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_OK
        );
        let mut info = EmuellaJ2kImageInfoV0::default();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_inspection_image_info(inspection, &mut info, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((info.width, info.height, info.component_count), (4, 4, 1));
        let mut component = EmuellaJ2kComponentInfoV0::default();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe {
                emuella_j2k_inspection_component_info(
                    inspection,
                    0,
                    &mut component,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((component.width, component.height), (4, 4));
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe {
                emuella_j2k_inspection_component_info(
                    inspection,
                    1,
                    &mut component,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );

        let mut workspace = ptr::null_mut();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()) },
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
                // SAFETY: The source and handles remain live; this call exclusively uses the
                // workspace and local outputs.
                unsafe {
                    emuella_j2k_decode_component_region(
                        decoder,
                        workspace,
                        &request,
                        &mut image,
                        ptr::null_mut(),
                    )
                },
                EMUELLA_J2K_STATUS_OK
            );
            let mut decoded_component = EmuellaJ2kComponentInfoV0::default();
            assert_eq!(
                // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
                // pointers are rejected before access.
                unsafe {
                    emuella_j2k_image_component_info(image, &mut decoded_component, ptr::null_mut())
                },
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
                // SAFETY: The handle is live and the disjoint local buffer has the stated
                // capacity.
                unsafe {
                    emuella_j2k_image_copy(
                        image,
                        too_small.as_mut_ptr(),
                        too_small.len(),
                        2,
                        ptr::null_mut(),
                    )
                },
                EMUELLA_J2K_STATUS_INVALID_ARGUMENT
            );
            assert_eq!(too_small, [0xa5; 3]);
            let mut actual = [0_u8; 4];
            assert_eq!(
                // SAFETY: The handle is live and the disjoint local buffer has the stated
                // capacity.
                unsafe {
                    emuella_j2k_image_copy(
                        image,
                        actual.as_mut_ptr(),
                        actual.len(),
                        2,
                        ptr::null_mut(),
                    )
                },
                EMUELLA_J2K_STATUS_OK
            );
            assert_eq!(
                actual,
                [expected[5], expected[6], expected[9], expected[10]]
            );
            // SAFETY: This matching handle is released once, after its last synchronous use.
            unsafe { emuella_j2k_image_destroy(image) };
        }
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_inspection_destroy(inspection) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_workspace_destroy(workspace) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }

    #[test]
    fn satellite_precision_and_rgb16_cross_tile_regions_preserve_native_words() {
        use emuella_j2k::{
            ColorModel, ComponentLayout, EncodeOptions, ImageInfo, ImageView, OutputFormat,
            SampleEndian, SampleFormat, TileSize, encode,
        };
        for (bits, bands) in [(11, 1_u16), (16, 3)] {
            let format =
                SampleFormat::with_byte_order(bits, false, Some(SampleEndian::Little)).unwrap();
            let info = ImageInfo::new(
                67,
                53,
                bands,
                format,
                if bands == 1 {
                    ColorModel::Grayscale
                } else {
                    ColorModel::Rgb
                },
                ComponentLayout::Interleaved,
            )
            .unwrap();
            let value = |x: u32, y: u32, c: u16| {
                ((x * 997 + y * 617 + x * y * 13 + u32::from(c) * 1237) & ((1_u32 << bits) - 1))
                    as u16
            };
            let mut samples = Vec::new();
            for y in 0..53 {
                for x in 0..67 {
                    for c in 0..bands {
                        samples.extend_from_slice(&value(x, y, c).to_le_bytes());
                    }
                }
            }
            let bytes = encode(
                ImageView::Interleaved {
                    info: &info,
                    samples: &samples,
                    stride_bytes: 67 * usize::from(bands) * 2,
                },
                &EncodeOptions {
                    format: OutputFormat::J2kCodestream,
                    decomposition_levels: 2,
                    tile_size: Some(TileSize {
                        width: 32,
                        height: 32,
                    }),
                    ..Default::default()
                },
            )
            .unwrap();
            let mut source = Box::new(TestSource {
                bytes,
                reads: Mutex::new(Vec::new()),
                fail_reads: false,
            });
            // SAFETY: Source storage and handles remain live; all destinations are disjoint local buffers.
            unsafe {
                let decoder = decoder(&mut source);
                let mut workspace = ptr::null_mut();
                assert_eq!(
                    emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                let request = EmuellaJ2kDecodeComponentsRequestV0 {
                    struct_size: size_of::<EmuellaJ2kDecodeComponentsRequestV0>(),
                    component_count: bands,
                    components: if bands == 3 { [2, 0, 1, 0] } else { [0; 4] },
                    x: 29,
                    y: 27,
                    width: 9,
                    height: 11,
                    collect_work: 1,
                    ..Default::default()
                };
                for _ in 0..2 {
                    let mut image = ptr::null_mut();
                    assert_eq!(
                        emuella_j2k_decode_components_region(
                            decoder,
                            workspace,
                            &request,
                            &mut image,
                            ptr::null_mut()
                        ),
                        EMUELLA_J2K_STATUS_OK
                    );
                    for index in 0..bands {
                        let mut component = EmuellaJ2kComponentInfoV0::default();
                        assert_eq!(
                            emuella_j2k_image_component_info_at(
                                image,
                                index,
                                &mut component,
                                ptr::null_mut()
                            ),
                            EMUELLA_J2K_STATUS_OK
                        );
                        assert_eq!(component.bits_per_sample, bits);
                        assert_eq!(component.is_signed, 0);
                        assert_eq!((component.width, component.height), (9, 11));

                        let mut output = [0xa5; 25 * 11 + 7];
                        assert_eq!(
                            emuella_j2k_image_copy_component(
                                image,
                                index,
                                output.as_mut_ptr(),
                                output.len(),
                                25,
                                ptr::null_mut()
                            ),
                            EMUELLA_J2K_STATUS_OK
                        );
                        for y in 0..11 {
                            for x in 0..9 {
                                let offset = y * 25 + x * 2;
                                assert_eq!(
                                    u16::from_le_bytes([output[offset], output[offset + 1]]),
                                    value(
                                        29 + x as u32,
                                        27 + y as u32,
                                        request.components[index as usize]
                                    )
                                );
                            }
                            assert!(output[y * 25 + 18..(y + 1) * 25].iter().all(|v| *v == 0xa5));
                        }
                        assert!(output[25 * 11..].iter().all(|v| *v == 0xa5));
                    }
                    let mut work = EmuellaJ2kDecodeWorkV0::default();
                    assert_eq!(
                        emuella_j2k_image_decode_work(image, &mut work, ptr::null_mut()),
                        EMUELLA_J2K_STATUS_OK
                    );
                    assert_eq!(work.output_allocation_bytes, 9 * 11 * 2 * u64::from(bands));
                    emuella_j2k_image_destroy(image);
                }
                emuella_j2k_workspace_destroy(workspace);
                emuella_j2k_decoder_destroy(decoder);
            }
        }
    }

    #[test]
    fn four_independent_components_use_all_inline_slots() {
        let originals = [vec![11; 16], vec![23; 16], vec![37; 16], vec![49; 16]];
        let bytes = emuella_j2k::codestream::encode_planar_u8_no_decomp_test_fixture(
            4,
            4,
            &[&originals[0], &originals[1], &originals[2], &originals[3]],
        )
        .unwrap();
        let mut source = Box::new(TestSource {
            bytes,
            reads: Mutex::new(Vec::new()),
            fail_reads: false,
        });
        // SAFETY: The source and handles remain live; every output is disjoint local storage.
        unsafe {
            let decoder = decoder(&mut source);
            let mut workspace = ptr::null_mut();
            assert_eq!(
                emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
                EMUELLA_J2K_STATUS_OK
            );
            let request = EmuellaJ2kDecodeComponentsRequestV0 {
                struct_size: size_of::<EmuellaJ2kDecodeComponentsRequestV0>(),
                component_count: 4,
                components: [3, 1, 0, 2],
                width: 4,
                height: 4,
                collect_work: 1,
                ..Default::default()
            };
            let mut image = ptr::null_mut();
            assert_eq!(
                emuella_j2k_decode_components_region(
                    decoder,
                    workspace,
                    &request,
                    &mut image,
                    ptr::null_mut()
                ),
                EMUELLA_J2K_STATUS_OK
            );
            for (index, source_component) in request.components.into_iter().enumerate() {
                let mut samples = [0; 16];
                assert_eq!(
                    emuella_j2k_image_copy_component(
                        image,
                        index as u16,
                        samples.as_mut_ptr(),
                        16,
                        4,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(samples.as_slice(), originals[source_component as usize]);
            }
            let mut work = EmuellaJ2kDecodeWorkV0::default();
            assert_eq!(
                emuella_j2k_image_decode_work(image, &mut work, ptr::null_mut()),
                EMUELLA_J2K_STATUS_OK
            );
            assert_eq!(work.output_allocation_count, 4);
            assert_eq!(work.output_allocation_bytes, 64);
            emuella_j2k_image_destroy(image);
            emuella_j2k_workspace_destroy(workspace);
            emuella_j2k_decoder_destroy(decoder);
        }
    }

    #[test]
    fn combined_mct_preserves_order_and_reconstructs_dependencies_once() {
        let fixture = emuella_j2k_test_support::native_planes::reversible_mct_region_fixture();
        let mut source = Box::new(TestSource {
            bytes: fixture.tnsot_one,
            reads: Mutex::new(Vec::new()),
            fail_reads: false,
        });
        // SAFETY: Source storage remains live and immutable through decoder destruction.
        let decoder = unsafe { decoder(&mut source) };
        let mut workspace = ptr::null_mut();
        // SAFETY: All handles remain live and all outputs are disjoint local storage.
        unsafe {
            assert_eq!(
                emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
                EMUELLA_J2K_STATUS_OK
            );
            let mut request = EmuellaJ2kDecodeComponentsRequestV0 {
                struct_size: size_of::<EmuellaJ2kDecodeComponentsRequestV0>(),
                component_count: 3,
                components: [2, 0, 1, 0],
                x: 61,
                y: 63,
                width: 7,
                height: 5,
                collect_work: 1,
                ..Default::default()
            };
            let mut combined = EmuellaJ2kDecodeWorkV0::default();
            for repetition in 0..2 {
                let mut image = ptr::null_mut();
                assert_eq!(
                    emuella_j2k_decode_components_region(
                        decoder,
                        workspace,
                        &request,
                        &mut image,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_OK
                );
                let mut info = EmuellaJ2kImageInfoV0::default();
                assert_eq!(
                    emuella_j2k_image_info(image, &mut info, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!((info.width, info.height, info.component_count), (7, 5, 3));
                let mut work = EmuellaJ2kDecodeWorkV0::default();
                assert_eq!(
                    emuella_j2k_image_decode_work(image, &mut work, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(work.preparation_count, 1);
                assert!(work.code_blocks_decoded > 0 && work.tier1_coefficients > 0);
                assert!(work.synthesis_lifting_updates > 0);
                assert_eq!(work.output_allocation_bytes, 105);
                assert_eq!(work.output_allocation_count, 3);
                assert!(work.output_capacity_bytes >= 105);
                if repetition == 0 {
                    combined = work;
                } else {
                    assert_eq!(
                        work.workspace_retained_heap_bytes,
                        combined.workspace_retained_heap_bytes
                    );
                    assert_eq!(work.code_blocks_decoded, combined.code_blocks_decoded);
                }
                for (index, source_index) in [2_usize, 0, 1].into_iter().enumerate() {
                    let mut component = EmuellaJ2kComponentInfoV0::default();
                    assert_eq!(
                        emuella_j2k_image_component_info_at(
                            image,
                            index as u16,
                            &mut component,
                            ptr::null_mut()
                        ),
                        EMUELLA_J2K_STATUS_OK
                    );
                    assert_eq!(component.source_component as usize, source_index);
                    assert_eq!(
                        (
                            component.x_origin,
                            component.y_origin,
                            component.width,
                            component.height
                        ),
                        (61, 63, 7, 5)
                    );
                    let mut actual = [0xa5_u8; 45];
                    assert_eq!(
                        emuella_j2k_image_copy_component(
                            image,
                            index as u16,
                            actual.as_mut_ptr(),
                            actual.len(),
                            9,
                            ptr::null_mut()
                        ),
                        EMUELLA_J2K_STATUS_OK
                    );
                    for row in 0..5 {
                        let start = (row + 63) * fixture.width as usize + 61;
                        assert_eq!(
                            &actual[row * 9..row * 9 + 7],
                            &fixture.planes[source_index][start..start + 7]
                        );
                        assert_eq!(&actual[row * 9 + 7..row * 9 + 9], &[0xa5; 2]);
                    }
                    let sentinel = actual;
                    for (bad_index, capacity, stride) in
                        [(3, 45, 9), (0, 42, 9), (0, 45, 6), (0, 45, usize::MAX)]
                    {
                        assert_eq!(
                            emuella_j2k_image_copy_component(
                                image,
                                bad_index,
                                actual.as_mut_ptr(),
                                capacity,
                                stride,
                                ptr::null_mut()
                            ),
                            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
                        );
                        assert_eq!(actual, sentinel);
                    }
                    let sentinel_component = component.source_component;
                    assert_eq!(
                        emuella_j2k_image_component_info_at(
                            image,
                            3,
                            &mut component,
                            ptr::null_mut()
                        ),
                        EMUELLA_J2K_STATUS_INVALID_ARGUMENT
                    );
                    assert_eq!(component.source_component, sentinel_component);
                }
                let mut first = [0; 35];
                let mut legacy = [0; 35];
                assert_eq!(
                    emuella_j2k_image_copy_component(
                        image,
                        0,
                        first.as_mut_ptr(),
                        35,
                        7,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(
                    emuella_j2k_image_copy(image, legacy.as_mut_ptr(), 35, 7, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(first, legacy);
                let mut component = EmuellaJ2kComponentInfoV0::default();
                assert_eq!(
                    emuella_j2k_image_component_info(image, &mut component, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(component.source_component, 2);
                emuella_j2k_image_destroy(image);
            }
            for index in 0..3 {
                request.component_count = 1;
                request.components = [index, 0, 0, 0];
                let mut image = ptr::null_mut();
                assert_eq!(
                    emuella_j2k_decode_components_region(
                        decoder,
                        workspace,
                        &request,
                        &mut image,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_OK
                );
                let mut work = EmuellaJ2kDecodeWorkV0::default();
                assert_eq!(
                    emuella_j2k_image_decode_work(image, &mut work, ptr::null_mut()),
                    EMUELLA_J2K_STATUS_OK
                );
                assert_eq!(work.code_blocks_decoded, combined.code_blocks_decoded);
                assert_eq!(work.tier1_coefficients, combined.tier1_coefficients);
                assert_eq!(
                    work.synthesis_lifting_updates,
                    combined.synthesis_lifting_updates
                );
                assert_eq!(
                    work.synthesis_output_samples,
                    combined.synthesis_output_samples
                );
                assert_eq!(
                    work.windowed_synthesis_component_tiles,
                    combined.windowed_synthesis_component_tiles
                );
                assert_eq!(
                    work.full_synthesis_component_tiles,
                    combined.full_synthesis_component_tiles
                );
                assert_eq!(work.output_allocation_bytes, 35);
                emuella_j2k_image_destroy(image);
            }
            request.collect_work = 0;
            let mut image = ptr::null_mut();
            assert_eq!(
                emuella_j2k_decode_components_region(
                    decoder,
                    workspace,
                    &request,
                    &mut image,
                    ptr::null_mut()
                ),
                EMUELLA_J2K_STATUS_OK
            );
            let mut work = EmuellaJ2kDecodeWorkV0 {
                preparation_count: 99,
                ..Default::default()
            };
            assert_eq!(
                emuella_j2k_image_decode_work(image, &mut work, ptr::null_mut()),
                EMUELLA_J2K_STATUS_UNSUPPORTED
            );
            assert_eq!(work.preparation_count, 99);
            emuella_j2k_image_destroy(image);
            emuella_j2k_workspace_destroy(workspace);
            emuella_j2k_decoder_destroy(decoder);
        }
    }

    #[test]
    fn combined_requests_validate_before_publication_and_contain_panics() {
        let (mut source, _) = fixture();
        // SAFETY: Source storage remains live and immutable through decoder destruction.
        let decoder = unsafe { decoder(&mut source) };
        // SAFETY: All handles remain live, request storage is initialised and outputs are disjoint.
        unsafe {
            let mut workspace = ptr::null_mut();
            assert_eq!(
                emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()),
                EMUELLA_J2K_STATUS_OK
            );
            let valid = EmuellaJ2kDecodeComponentsRequestV0 {
                struct_size: size_of::<EmuellaJ2kDecodeComponentsRequestV0>(),
                component_count: 1,
                width: 2,
                height: 2,
                ..Default::default()
            };
            let invalid = [
                EmuellaJ2kDecodeComponentsRequestV0 {
                    struct_size: size_of::<AbiHeader>(),
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    abi_version: 1,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    component_count: 0,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    component_count: 5,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    component_count: 2,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    components: [0, 1, 0, 0],
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    collect_work: 2,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    reserved: 1,
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    reserved_bytes: [1; 6],
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 { width: 0, ..valid },
            ];
            for request in invalid {
                let mut image = ptr::dangling_mut();
                assert_eq!(
                    emuella_j2k_decode_components_region(
                        decoder,
                        workspace,
                        &request,
                        &mut image,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_INVALID_ARGUMENT
                );
                assert!(image.is_null());
            }
            for request in [
                EmuellaJ2kDecodeComponentsRequestV0 {
                    components: [1, 0, 0, 0],
                    ..valid
                },
                EmuellaJ2kDecodeComponentsRequestV0 {
                    x: u32::MAX,
                    ..valid
                },
            ] {
                let mut image = ptr::dangling_mut();
                assert_ne!(
                    emuella_j2k_decode_components_region(
                        decoder,
                        workspace,
                        &request,
                        &mut image,
                        ptr::null_mut()
                    ),
                    EMUELLA_J2K_STATUS_OK
                );
                assert!(image.is_null());
            }
            let mut image = ptr::null_mut();
            assert_eq!(
                emuella_j2k_decode_components_region(
                    decoder,
                    workspace,
                    &valid,
                    &mut image,
                    ptr::null_mut()
                ),
                EMUELLA_J2K_STATUS_OK
            );
            emuella_j2k_image_destroy(image);
            FORCE_DECODE_PANIC.with(|force| force.set(true));
            assert_eq!(
                emuella_j2k_decode_components_region(
                    decoder,
                    workspace,
                    &valid,
                    &mut image,
                    ptr::null_mut()
                ),
                EMUELLA_J2K_STATUS_PANIC
            );
            assert!(image.is_null());
            assert_eq!(
                emuella_j2k_decode_components_region(
                    decoder,
                    workspace,
                    &valid,
                    &mut image,
                    ptr::null_mut()
                ),
                EMUELLA_J2K_STATUS_INVALID_ARGUMENT
            );
            emuella_j2k_workspace_destroy(workspace);
            emuella_j2k_decoder_destroy(decoder);
        }
    }

    #[test]
    fn reversible_mct_region_preserves_the_one_plane_abi() {
        let fixture = emuella_j2k_test_support::native_planes::reversible_mct_region_fixture();
        let mut source = Box::new(TestSource {
            bytes: fixture.tnsot_zero,
            reads: Mutex::new(Vec::new()),
            fail_reads: false,
        });
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut source) };
        let mut workspace = ptr::null_mut();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()) },
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
            // SAFETY: The source and handles remain live; this call exclusively uses the
            // workspace and local outputs.
            unsafe {
                emuella_j2k_decode_component_region(
                    decoder,
                    workspace,
                    &request,
                    &mut image,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_OK
        );
        let mut info = EmuellaJ2kImageInfoV0::default();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_image_info(image, &mut info, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_OK
        );
        assert_eq!((info.width, info.height, info.component_count), (7, 5, 1));
        let mut component = EmuellaJ2kComponentInfoV0::default();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_image_component_info(image, &mut component, ptr::null_mut()) },
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
            // SAFETY: The handle is live and the disjoint local buffer has the stated capacity.
            unsafe {
                emuella_j2k_image_copy(image, actual.as_mut_ptr(), actual.len(), 7, ptr::null_mut())
            },
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
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_image_destroy(image) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_workspace_destroy(workspace) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }

    #[test]
    fn inspection_exposes_heterogeneous_component_metadata() {
        let mut source = heterogeneous_fixture();
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut source) };
        let mut inspection = ptr::null_mut();
        assert_eq!(
            // SAFETY: The source and decoder remain live; the local output slots are exclusive
            // and disjoint.
            unsafe { emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_OK
        );
        let expected = [(8, 0, 1, 1, 4, 4), (12, 1, 2, 1, 2, 4), (16, 0, 2, 2, 2, 2)];
        for (index, expected) in expected.into_iter().enumerate() {
            let mut component = EmuellaJ2kComponentInfoV0::default();
            assert_eq!(
                // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
                // pointers are rejected before access.
                unsafe {
                    emuella_j2k_inspection_component_info(
                        inspection,
                        u16::try_from(index).unwrap(),
                        &mut component,
                        ptr::null_mut(),
                    )
                },
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
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_inspection_destroy(inspection) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }

    #[test]
    fn callback_failure_is_source_io_with_owned_diagnostic() {
        let (mut source, _) = fixture();
        source.fail_reads = true;
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut source) };
        let mut inspection = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert_eq!(
            // SAFETY: The source and decoder remain live; the local output slots are exclusive
            // and disjoint.
            unsafe { emuella_j2k_decoder_inspect(decoder, &mut inspection, &mut error) },
            EMUELLA_J2K_STATUS_SOURCE_IO
        );
        assert!(inspection.is_null());
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_error_status(error) },
            EMUELLA_J2K_STATUS_SOURCE_IO
        );
        let mut required = 0;
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_error_message_size(error, &mut required) },
            EMUELLA_J2K_STATUS_OK
        );
        let mut message = vec![0; required];
        assert_eq!(
            // SAFETY: The handle is live and the disjoint local buffer has the stated capacity.
            unsafe { emuella_j2k_error_message_copy(error, message.as_mut_ptr(), message.len()) },
            EMUELLA_J2K_STATUS_OK
        );
        let message = std::ffi::CStr::from_bytes_with_nul(&message).unwrap();
        assert!(message.to_string_lossy().contains("byte 0"));
        assert!(message.to_string_lossy().contains("status 91"));
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_error_destroy(error) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }

    #[test]
    fn malformed_later_marker_is_invalid_input_not_source_io() {
        let (mut malformed, _) = fixture();
        let eoc_prefix = malformed.bytes.len() - 2;
        assert_eq!(&malformed.bytes[eoc_prefix..], &[0xff, 0xd9]);
        malformed.bytes[eoc_prefix] = 0xfe;
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut malformed) };
        let mut inspection = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert_eq!(
            // SAFETY: The source and decoder remain live; the local output slots are exclusive
            // and disjoint.
            unsafe { emuella_j2k_decoder_inspect(decoder, &mut inspection, &mut error) },
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        assert!(inspection.is_null());
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_error_status(error) },
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_error_destroy(error) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }

    #[test]
    fn malformed_input_and_undersized_structures_are_rejected() {
        let (mut malformed, _) = fixture();
        malformed.bytes[1] = 0x50;
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut malformed) };
        let mut inspection = ptr::null_mut();
        assert_eq!(
            // SAFETY: The source and decoder remain live; the local output slots are exclusive
            // and disjoint.
            unsafe { emuella_j2k_decoder_inspect(decoder, &mut inspection, ptr::null_mut()) },
            EMUELLA_J2K_STATUS_INVALID_INPUT
        );
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };

        let header = AbiHeader {
            struct_size: size_of::<AbiHeader>(),
            abi_version: EMUELLA_J2K_ABI_VERSION,
        };
        let sentinel = ptr::dangling_mut::<EmuellaJ2kDecoder>();
        let mut output = sentinel;
        assert_eq!(
            // SAFETY: The caller keeps the source stable; the descriptor and output are
            // disjoint local storage.
            unsafe {
                emuella_j2k_decoder_create(
                    ptr::from_ref(&header).cast::<EmuellaJ2kSourceV0>(),
                    &mut output,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        assert!(output.is_null());
    }

    #[test]
    fn panic_is_contained_and_workspace_is_poisoned() {
        let (mut source, _) = fixture();
        let decoder = // SAFETY: The boxed source stays live and unchanged until the decoder is destroyed.
unsafe { decoder(&mut source) };
        let mut workspace = ptr::null_mut();
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_workspace_create(&mut workspace, ptr::null_mut()) },
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
            // SAFETY: The source and handles remain live; this call exclusively uses the
            // workspace and local outputs.
            unsafe {
                emuella_j2k_decode_component_region(
                    decoder, workspace, &request, &mut image, &mut error,
                )
            },
            EMUELLA_J2K_STATUS_PANIC
        );
        assert!(image.is_null());
        assert!(!error.is_null());
        assert_eq!(
            // SAFETY: Handles remain live and outputs are disjoint local storage; invalid
            // pointers are rejected before access.
            unsafe { emuella_j2k_error_status(error) },
            EMUELLA_J2K_STATUS_PANIC
        );
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_error_destroy(error) };
        assert_eq!(
            // SAFETY: The source and handles remain live; this call exclusively uses the
            // workspace and local outputs.
            unsafe {
                emuella_j2k_decode_component_region(
                    decoder,
                    workspace,
                    &request,
                    &mut image,
                    ptr::null_mut(),
                )
            },
            EMUELLA_J2K_STATUS_INVALID_ARGUMENT
        );
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_workspace_destroy(workspace) };
        // SAFETY: This matching handle is released once, after its last synchronous use.
        unsafe { emuella_j2k_decoder_destroy(decoder) };
    }
}
