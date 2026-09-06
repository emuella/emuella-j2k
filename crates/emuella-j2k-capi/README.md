# emuella-j2k-capi

Experimental, read-only C ABI for raw JPEG 2000 Part 1 positioned sources.
The generated header in `include/emuella_j2k.h` is not yet an ABI-major-one
compatibility promise. See the repository C ABI safety contract for ownership,
threading and callback requirements.

The current surface creates a decoder from a consumer-owned exact positioned
read callback, inspects reference-image and component geometry, reuses an
explicit workspace for single- or multi-component region decode, retains decoded bytes in an
immutable Rust-owned image, and copies rows into an explicitly sized consumer
buffer. Error status values are stable within this experimental header;
optional Rust-owned error handles provide a NUL-terminated UTF-8 diagnostic.
`emuella_j2k_error_message_size` includes that terminating NUL byte.

All calls are synchronous. Output pointer locations must be writable and
mutually disjoint. Consumers must destroy every successfully returned decoder,
inspection, workspace, image and error exactly once, and must not use or destroy
a handle concurrently with its destruction. A decoder may serve concurrent
calls, but every active decode requires a different workspace. The source
context and bytes remain consumer-owned, stable and valid until decoder
destruction; its callback must be concurrency-safe and must not unwind or
retain the temporary destination.

`sh scripts/check-c-api.sh` regenerates and compares the header, checks the
shared-library symbol allow-list, and compiles, links and runs the C11 and C++17
consumers against both shared and static Linux x86-64 libraries. Each consumer
also drives one decoder concurrently from two threads with distinct workspaces
and outputs; its positioned callback proves that the operations overlap.

The additive `emuella_j2k_decode_components_region` request selects one through
four distinct component indices in caller order. All selected components share
one region, reduction and layer limit. Unused inline indices and reserved
fields must be zero. This four-component limit belongs to the ABI request,
not to the codec's admission rules. Existing single-component requests retain
their layout and behaviour. `emuella_j2k_image_component_info_at` and
`emuella_j2k_image_copy_component` index output positions; the legacy descriptor
and copy functions select output position zero. Image metadata reports the
selected component count. All output planes remain private until the complete
decode succeeds, including inverse RCT dependencies for reversible MCT.

Set `collect_work = 1` to retain an immutable `EmuellaJ2kDecodeWorkV0` snapshot
and obtain it through `emuella_j2k_image_decode_work`. Disabled collection
returns `UNSUPPORTED` without modifying the output descriptor. Work counters
include all reconstructed dependencies, including unrequested MCT components;
they use the codec's inexpensive `WorkCounters` mode, without detailed
per-block timing. Each successful call prepares one fresh regional plan.
Decoder reuse preserves source identity, and workspace reuse retains scratch;
neither means that a packet index or regional plan is cached across calls.

`output_allocation_count` counts successful new C ABI output-plane buffer
reservation requests only. `output_allocation_bytes` is their total logical
length and `output_capacity_bytes` is their actual combined capacity. These
exclude descriptor vectors, plans, workspace, allocator metadata and consumer
copy buffers. They are not total allocator-call counts or process memory.
Workspace fields observe retained capacities after execution, including previous
calls, with sample-slot or byte units documented in the header. The complete
`workspace_retained_heap_bytes` includes private worker scratch; it is not
per-call allocation growth. Source callback bytes are deliberately measured by
the consumer, where overlapping calls can be attributed correctly.

The native gate generates its reversible-MCT codestream from the existing
project-authored `native_planes::reversible_mct_region_fixture` and checks each
requested RGB sample against its independent arithmetic oracle. Both native
languages test reordered planes, row padding, invalid bounds, reuse and MCT
work counts. Rust tests also compare combined work with all three individual
component calls and contain an injected panic through the new entry point.
