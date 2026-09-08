# C ABI design and safety contract

Status: implemented experimental major-zero boundary. Its generated header and
Linux x86-64 native checks do not yet constitute an ABI-major-one compatibility
promise.

## Purpose

`emuella-j2k` needs a native boundary suitable for independently built C and
C++ consumers, initially a read-only GDAL JPEG 2000 driver. The boundary must
preserve the codec's safe-Rust architecture, support GDAL virtual I/O without
materialising an entire NITF image segment, and remain independently packageable
from any one consumer.

This record fixes the ownership, threading, callback, error, panic, versioning
and unsafe-code rules that govern the implementation. The original design did
not fix function names, complete C structure layouts or build artefacts; the
implemented major-zero header now records those experimental details. They
become public compatibility commitments only after the major-one gates below.

## Decision

The project exposes a small experimental C ABI from a separate
`emuella-j2k-capi` crate. The ABI will call safe public facade operations and
does not duplicate codec behaviour. `emuella-j2k` provides the safe
source-backed inspection operation that reports raw Part 1 image geometry and
component descriptors without requiring a decode request. That operation is
codec-owned and reuses the codec's parser, admission and error behaviour; the C
API does not parse codestream markers itself. Pinned `cbindgen` produces the
public C header deterministically from explicitly C-compatible Rust
declarations. The installed C ABI, rather than Rust or C++ ABI, will be the
binary contract for consumers after major-one qualification.

The initial implementation is read-only and source-backed. Its intended
capabilities are:

- create a decoder over an immutable positioned-read source;
- discover raw Part 1 image and component properties;
- decode one admitted region/resolution request selecting one or several
  components with a reusable workspace;
- retain decoded samples in Rust-owned immutable image storage;
- copy a selected plane into a caller buffer; and
- return structured status and diagnostic information.

The implemented names and layouts are generated in
`crates/emuella-j2k-capi/include/emuella_j2k.h`. Opaque decoder, inspection,
workspace, image and error handles have distinct nominal types. Source and
decode-request inputs use `struct_size`, experimental ABI version zero and
zeroed reserved fields. Image and component descriptors carry their own size
and ABI version. `emuella_j2k_abi_version` and
`emuella_j2k_package_version` provide the minimal version queries. The source
callback, inspection, reusable-workspace component-region decode, immutable
image copy, opt-in work observations and owned diagnostic lifecycles comprise
the current surface.

One admitted reversible-MCT regional profile reconstructs the three required
component dependencies privately and returns only the single component named
by the existing request. The image continues to contain exactly one plane and
its descriptor retains that requested source-component identity. No ABI field,
ownership rule or publication sequence changes: the handle is published only
after all dependency reads, reconstruction and inverse RCT succeed.

The additive `EmuellaJ2kDecodeComponentsRequestV0` selects one through four
distinct indices in an inline array. The array avoids a new foreign-slice
operation. Count bounds are checked before slicing, unused slots and reserved
fields must be zero, and `collect_work` is zero or one. The four-component bound
is a C request limit, not new codec admission. One existing safe prepared plan
produces owned planar output in request order, reconstructing reversible-MCT
dependencies once. Indexed descriptor and bounded-copy operations address
output positions; the existing single-component request remains unchanged and
legacy image accessors address position zero. Failure never publishes a partial
image; rejected copies preserve destination bytes and row padding.

Optional per-image work observations use `DecodeInstrumentation::WorkCounters`
and immutable scalar snapshots. They include unrequested reconstruction
dependencies. Output allocation observations count only successful C ABI
output-plane reservation requests and their logical bytes and actual
capacities. They exclude all other allocations and allocator metadata.
Workspace fields report retained capacities after the call, including earlier
reuse, rather than allocation counts or growth. Consumer callbacks own source
byte attribution. A reused decoder preserves source identity and a reused
workspace retains scratch. The explicit indexed constructor also retains
validated header and tile-part metadata; selected packet plans are rebuilt for
each request in both constructor modes. Observations are local to
one execution and do not use shared counter deltas that concurrent calls could
contaminate. Disabled collection returns `UNSUPPORTED` from the work accessor
without changing the caller's output descriptor.

Raw J2K is the first input route because it is a payload form delegated by a
NITF `IC=C8` image segment. JP2 container traversal, HTJ2K/JPH exposure,
encoding, direct decode into foreign storage, asynchronous work and custom
allocators are later additive decisions. The C ABI must not describe an input
as supported merely because the Rust inspector can parse it.

## Boundary and dependency direction

The safe Rust facade remains the semantic owner:

```text
C or C++ consumer
        |
        v
emuella-j2k-capi     C layout, ownership and failure translation only
        |
        v
emuella-j2k          safe public Rust facade
        |
        v
core/codestream/container/transform/entropy crates
```

The C API must not expose Rust layout, generics, trait objects, references,
slices, `String`, `Vec`, panics or allocator obligations. The codec crates must
not depend on the C API crate or GDAL. A GDAL adapter owns `VSILFILE` and GDAL
object semantics; the C API sees only the positioned source contract.

Source inspection is an additive safe Rust facade operation, not C ABI code. It
is implemented and verified in `emuella-j2k`; the C discovery operation
delegates to it so both operations share the same codec-owned interpretation of
image properties. The C API may not read markers or reconstruct private parser
behaviour in a later extension.

## Rust calling contract

The crate also produces an `rlib`. Every pointer-dependent export is therefore
an `unsafe extern "C" fn`, with an individual `# Safety` section stating the
caller obligations. Only the pointer-free ABI and package version queries
remain safe. Rust consumers must acknowledge storage validity, handle lifetime
and ownership, output non-aliasing, callback lifetime and concurrent-use rules
at each call. This corrects the Rust API's safety contract without changing C
symbol names, calling convention or structure layouts.

Null, alignment, capacity and structure-size checks detect some invalid
arguments; they cannot establish allocation provenance or prevent use after
destruction. The private pointer helpers and panic boundary are also unsafe,
so a safe helper cannot hide these obligations. `unsafe_op_in_unsafe_fn` is
denied and each unsafe call has an explicit justification.

## C representation

Only fixed-width integers, `size_t`, raw pointers, function pointers and
`#[repr(C)]` plain-data structures may cross the boundary. In particular:

- status, mode and option values use fixed-width integer typedefs and named
  constants, not a Rust enum;
- every incoming integer is validated before conversion to a Rust enum or
  bounded type;
- C booleans are fixed-width integers with only zero and one accepted;
- offsets and source lengths are unsigned 64-bit values;
- byte spans always carry an explicit capacity or length;
- arithmetic converting offsets, dimensions, strides and capacities is checked;
  and
- structures never contain a Rust-owned field or a pointer whose target layout
  is not itself part of the C contract.

Extensible input structures begin with `struct_size` and `abi_version` fields.
The implementation checks the minimum size before reading a field, ignores
known-compatible trailing storage, and rejects an incompatible ABI major.
Reserved fields must be zero and are not an invitation for consumers to store
private state.

Opaque handle declarations give C nominal type separation between decoder,
workspace, image and error values. Their pointees remain private Rust types.

## Ownership and lifetimes

The allocator that creates storage must release it. C and C++ code must never
`free` or `delete` a Rust allocation, and Rust must never release consumer-owned
storage or a callback context.

The initial handle rules are:

| Value | Owner | Lifetime and release |
|---|---|---|
| Source context | Consumer | Must remain valid and immutable in identity until every decoder using it is destroyed and all calls have returned; Rust never releases it |
| Decoder | Rust library | Created through the ABI and destroyed exactly once after its operations have quiesced |
| Workspace | Rust library | Created through the ABI; exclusively borrowed by one active call and destroyed exactly once when idle |
| Image | Rust library | Immutable after publication; destroyed exactly once after all borrowed observations and copies finish |
| Error | Rust library | Immutable after publication; destroyed exactly once after diagnostic copies finish |
| Destination buffer | Consumer | Valid and exclusively writable for the complete synchronous copy call |

A null handle passed to a destroy operation is a no-op. Reusing a non-null
handle after destruction, destroying a copied handle twice, or destroying a
handle during an active call violates the C contract. The implementation must
not publish a partially initialised output handle on failure.

The first decode API returns Rust-owned image storage. Accessors may lend a
read-only pointer only for a documented synchronous lifetime tied to that image
handle; the baseline GDAL adapter should use an explicit bounded copy. Direct
decode into foreign storage is deferred until profiling justifies its larger
aliasing and failure-atomicity contract.

## Positioned-read callbacks

The source callback family mirrors `CodestreamSource`: report a stable length
and fill an exact range at a logical unsigned 64-bit offset. The implementation
passes Rust-owned writable storage to the read callback. A callback must either
fill the complete requested range or return a failure; a short successful read
is not representable.

For the decoder's complete lifetime, the consumer guarantees that:

- the context pointer remains valid;
- source length and bytes do not change;
- callbacks use the platform C calling convention and return normally;
- callbacks do not retain the temporary destination pointer;
- callbacks do not re-enter Emuella using a related handle;
- callbacks do not destroy or mutate an Emuella handle; and
- callbacks are safe for concurrent invocation.

The C++ consumer must catch all exceptions within its callback and translate
them to the callback's I/O status. No C++ exception may enter Rust. Callback
failure is reported as an I/O error with logical range provenance where
available; the ABI does not depend on ambient `errno`.

The additive `emuella_j2k_decoder_create_indexed` constructor takes explicit
positive byte/count ceilings and lazily retains one successful codec-owned
[Part 1 source index](part1-source-index.md) across inspection and different
regional requests. It fails over-budget requests without fallback; the original
`emuella_j2k_decoder_create` retains its prior one-shot source admission. Only
index construction is serialised; published metadata is
immutable and independent workspaces can decode concurrently. Failed construction
publishes no state and can retry transient source failures. Index budgets bound
retained headers and metadata; compressed bodies stay with the callback source.

The first implementation may serialise a cursor-based source internally or in
the consumer adapter, but that serialisation must not weaken the public
concurrent-callback contract. Asynchronous callbacks, callback unregistration
and callback-owned context release are outside the initial ABI.

## Threading contract

The initial ABI has synchronous calls only. Thread safety is defined per
handle, not inferred from C pointer constness:

- an initialised decoder is immutable and may serve concurrent operations;
- every concurrent decode uses a distinct workspace and distinct output;
- a workspace has at most one active operation, though it may move between
  threads while idle;
- an image and an error are immutable and may be observed concurrently;
- creating or destroying a handle is not concurrent with another operation on
  that handle; and
- source callbacks may run concurrently, including from internal codec workers.

The implementation must enforce these claims with compile-time `Send`/`Sync`
assertions for the private state and mixed-language concurrency tests. It must
not add an `unsafe impl Send` or `unsafe impl Sync` to obtain a desired result.
If the private types do not naturally satisfy the contract, the implementation
must add safe synchronisation or revise this record before publishing the ABI.

## Errors and diagnostics

Every fallible operation returns a fixed-width status code. The initial stable
categories are conceptually:

- success;
- invalid argument or ABI structure;
- invalid or truncated input;
- unsupported input or request;
- source I/O failure;
- allocation or resource-limit failure;
- internal invariant failure; and
- contained Rust panic.

Exact numeric values and names are fixed by the generated version-one header,
not by this design-only record. Unknown status values must remain representable
by consumers. Expected invalid, unsupported, I/O and resource outcomes are
ordinary errors and must never be implemented with panic.

Detailed diagnostics use an optional immutable Rust-owned error handle returned
for the failed call. Error messages are UTF-8, informational and not a stable
matching interface. Consumers obtain the required byte count and copy the
message into their own explicitly sized buffer. There is no process-global or
thread-local last-error slot, so nested and concurrent calls cannot overwrite
one another's diagnostics.

On success, output handles and scalar outputs are fully initialised and the
error output is null. On failure, ordinary outputs retain their documented
input value or are set to a defined null/zero state before return. Plane-copy
failure does not publish a partial logical result; the implementation validates
all dimensions, products, strides and capacity before the first write.

## Panic and exception containment

No unwind may cross the ABI in either direction. Exports use the non-unwinding
`extern "C"` ABI. Every export capable of reaching codec or allocation logic
contains an outer `catch_unwind` barrier and translates an unwinding Rust panic
to the panic status. The C API library is built with an unwinding panic strategy
so that barrier can operate; `panic=abort` is not an acceptable release profile
for an in-process GDAL dependency.

`catch_unwind` is containment for a library defect, not normal error handling
or proof that state remains usable. A panic must not publish a new handle or
image. Any mutable handle involved in the call is poisoned and permits only
destruction. Destruction and diagnostic fallback paths must be non-panicking.
Tests deliberately exercise the containment path without relying on a panic
for ordinary validation.

The callback contract prohibits foreign exceptions. The C++ GDAL adapter must
mark callback shims non-throwing, catch every exception internally and return an
I/O or internal-error status. The project will not use `C-unwind`.

## ABI versioning and packaging

The first implementation will become ABI major version 1 only when its header,
library and tests are accepted together. Before that point, generated artefacts
are experimental and carry no binary compatibility promise.

Once published:

- every exported symbol uses the `emuella_j2k_` prefix;
- the library exposes its ABI major and package version;
- the shared-library soname carries the ABI major on platforms that support it;
- compatible releases do not remove or change an existing symbol or reinterpret
  an existing field;
- new functions and named constants may be added compatibly;
- structures grow only by appending fields guarded by `struct_size`;
- a breaking ownership, layout, calling, threading or semantic change requires
  a new ABI major; and
- static and shared builds implement the same public contract.

The generated C header is a release artefact and the source of truth for native
consumers. CI regenerates it with a pinned `cbindgen`, fails on drift, compiles
it as both C and C++, and compares exported symbols with an allow-list. A C++
RAII convenience wrapper may be provided as header-only source, but it is not a
separate binary ABI.

## Initial unsafe budget

All existing codec, parser, container, transform, entropy, facade and CLI crates
retain `unsafe_code = "deny"`. Only `emuella-j2k-capi` may contain
project-authored unsafe code. Generated code and dependency unsafe code are
reported separately and never represented as eliminating risk.

The initial C API permits only these unsafe categories:

1. one explicit unsafe export-name attribute for each allow-listed
   `emuella_j2k_` symbol;
2. conversion between an opaque handle pointer and its private Rust allocation,
   including the exactly-once destruction path;
3. checked reads from C plain-data input pointers and checked writes to scalar
   or handle output pointers;
4. a checked copy between Rust storage and a caller-provided byte range; and
5. invocation of a validated foreign positioned-read callback.

Categories 2 through 5 retain six raw-operation sites: callback invocation,
plain-data read, output write, opaque-handle borrow, allocation reconstruction
and byte copy. `scripts/check-c-api.sh` checks a reviewed inventory of every
production unsafe block, including helper delegation, through
`scripts/audit-c-api-unsafe.py`. Changes to an operation or delegation expression
require updating that inventory after review. Its deliberately narrow source
recognition rejects unfamiliar block shapes; it complements the compiler lint
and independent safety review rather than claiming to parse arbitrary Rust.
Regression probes reject added operations before and after the decode test
hook, within a delegation block, and inside nested blocks. Each
raw operation must remain local, carry a `// SAFETY:` comment explaining its
validity, alignment, aliasing, length, lifetime and concurrency preconditions,
and immediately return to safe types.

Explicit unsafe calls to these helpers, the panic boundary and the public
exports forward the documented caller obligations and need their own local
safety justification. These calls are not additional raw operations. This
replaces the original eight-block ceiling: counting delegation blocks would
penalise enforcing the Rust calling contract and encourage safe helpers that
conceal unchecked preconditions. Adding a raw operation or a prohibited
construct still requires an amended design record and independent safety
review; neither larger blocks nor macros may conceal such a change.

The initial implementation prohibits:

- `unsafe trait` or `unsafe impl`, including manual `Send`/`Sync`;
- `static mut`, mutable foreign globals or unsynchronised global handle state;
- `transmute`, unions, unchecked indexing or uninitialised typed values;
- borrowing a foreign buffer beyond the synchronous call;
- reconstructing a Rust `Vec`, `String` or `CString` from foreign ownership;
- cross-language unwinding or `C-unwind`;
- callbacks that return borrowed foreign storage; and
- allocator interposition or consumer release of Rust storage.

An implementation that needs another raw operation or a prohibited operation
must stop and amend this design record with the motivating evidence and a new
independent safety review. Unsafe reduction is not allowed to weaken checks,
failure atomicity, source stability or resource bounds.

## Verification gates for the first implementation

The current Linux x86-64 gate checks header drift, the shared-library symbol
allow-list, layout assertions, and C11/C++17 compile, shared/static link and
runtime source-inspect-decode-copy journeys. Rust tests cover ordinary pointer
validation, callback failure and provenance, malformed input, bounds,
workspace reuse, natural `Send`/`Sync`, panic containment and poisoning.
The additive multi-component request, indexed copy/descriptor and work accessor
retain these six raw-operation sites and extend the helper-delegation inventory.
Native C11/C++17 shared and static tests also check their full layouts, reordered
MCT samples, bounds, padding, reuse and dependency work counts.
Compile-fail Rust consumer tests independently require unsafe calls for each
pointer-dependent export, with safe version queries as positive controls. The
reversible-MCT regional regression also checks exact one-plane output through
the existing request, image-descriptor and copy calls. The
remaining major-one gates include the full Miri, sanitiser, concurrent native
callback and operation-sequence fuzzing campaigns below; until those are
complete the version remains experimental major zero.

Publishing ABI major 1 requires all of the following:

- ordinary Rust tests for every safe conversion and state transition;
- compile-time assertions for intended `Send`/`Sync` properties;
- Miri coverage of the project-authored pointer and handle helpers where Miri
  can model them;
- plain C and C++ compile, link and runtime consumers of the generated header;
- C/C++ size, alignment and field-offset assertions for every shared structure;
- null, undersized, misaligned where testable, overflow, double-use and
  callback-failure cases without deliberately dereferencing an invalid address;
- panic-containment and poisoned-handle tests;
- concurrent decoder calls with distinct workspaces and a callback that detects
  invalid overlap;
- AddressSanitizer and UndefinedBehaviorSanitizer coverage of the complete
  native harness on supported toolchains, plus ThreadSanitizer where supported;
- fuzzing of valid ABI operation sequences and malformed codestream input;
- generated-header drift and exported-symbol allow-list checks;
- shared and static library smoke tests on every claimed platform; and
- the repository's canonical verification.

Sanitisers, Miri and fuzzing are complementary evidence, not proofs. Tests must
not invoke undefined behaviour merely to demonstrate that an invalid pointer
cannot be detected. The C contract remains responsible for pointer provenance
that the callee cannot establish.

## Consequences and deferred decisions

This design preserves a zero-unsafe codec core while acknowledging the real
trust boundary introduced by C. Rust-owned first-version output costs one copy
in a typical GDAL block read; that is accepted until measurement demonstrates a
need for a direct-output contract. Concurrent source callbacks place a clear
thread-safety obligation on a GDAL VSI adapter, which can initially meet it with
safe serialisation.

The experimental implementation adds and verifies the safe source-backed
inspection operation, including mixed-language concurrent decoder calls with
distinct workspaces and outputs. It resolves the current symbol names,
plain-data layouts, status values, ABI queries and package targets in its
generated header. Major one still requires the remaining gates above and
explicit supported-platform export controls. A later increment must not add
JP2 traversal, encoding, direct foreign output, asynchronous work, allocation
callbacks or new codec admission under the authority of this record alone.

This contract is based on the Rust Reference's
[C ABI](https://doc.rust-lang.org/reference/items/external-blocks.html),
[representation](https://doc.rust-lang.org/reference/type-layout.html) and
[unwinding](https://doc.rust-lang.org/reference/panic.html#unwinding-across-ffi-boundaries)
guarantees and the Rustonomicon's
[FFI guidance](https://doc.rust-lang.org/nomicon/ffi.html).
[`cbindgen`](https://github.com/mozilla/cbindgen) is a header generator, not a
soundness verifier; correctness remains owned by the boundary design,
implementation review and verification above.
