# emuella-j2k-capi

Experimental, read-only C ABI for raw JPEG 2000 Part 1 positioned sources.
The generated header in `include/emuella_j2k.h` is not yet an ABI-major-one
compatibility promise. See the repository C ABI safety contract for ownership,
threading and callback requirements.

The current surface creates a decoder from a consumer-owned exact positioned
read callback, inspects reference-image and component geometry, reuses an
explicit workspace for one-component region decode, retains decoded bytes in an
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
