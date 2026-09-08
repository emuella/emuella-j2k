# Reusable Part 1 source headers

`Part1SourceIndex<S>` owns an immutable positioned source and a validated header
index. The codestream crate owns indexing, retained marker ranges and physical
payload spans. Core and the public facade expose `new`, `inspect`, `prepare`,
`source`, `header_bytes` and `tile_part_count` with the existing application
errors, component descriptors and prepared decode type. The one-shot source
APIs and the original C constructor remain available with their previous behaviour.

Construction makes one complete header traversal, reconciles the SOT sequence
with any TLM and terminal EOC using the existing scanner, and validates effective
main/tile header state. It stores marker bytes and tile-part metadata, not
compressed packet bodies or image pixels. Index construction is structural
inspection, not a claim that every packet or decode request is supported.
The existing profile, packet and reconstruction validators still admit each
request. An index reads all tile headers during construction, so a malformed
header in an initially unrequested tile fails construction.

Preparation looks up selected tiles' retained marker ranges and payload spans.
It borrows the retained marker-byte buffer without cloning it, assembles only
selected tile markers in physical order, and prepares packet topology only for
the selected tile payloads. Execution reads selected codewords through the same
source. Main headers and unrelated tile headers are not reread or reparsed.
Different windows reuse the same index; packet plans and decoded pixels are not
cached. Source bytes and length must remain stable for the whole index lifetime.
A prepared plan borrows the index and cannot outlive its source binding.

## Resource and failure contract

Construction enforces all three caller-selected ceilings before the corresponding
retained structure grows. Rust `new` defaults to 16 MiB of compact marker bytes,
65,536 markers and 65,536 tile parts; `new_with_limits` accepts an explicit
`Part1SourceIndexLimits`. The byte ceiling excludes metadata and allocator overhead. Stored metadata
is linear in marker, component and tile-part counts, with per-tile marker ranges
and payload spans in bounded maps. The complete tile sequence and rectangle
catalogue are retained; without TLM, a second metadata-only tile-state vector
preserves the existing complete-coverage validator contract. No allocation is
proportional to full-image pixel count. Exceeding an index ceiling is reported
as unsupported input, including through the C ABI.

Per request, main-header descriptors, the physical tile-part sequence, and
complete coverage metadata are copied within those construction bounds. Geometry
selection and existing profile/coverage validation still perform work
proportional to total tile count. Selected tile descriptors and payload spans
are copied for the prepared plan. This implementation removes repeated source
header traversal; it does not promise constant-time regional preparation or
constant metadata memory as tile count grows. Existing selected-output,
coefficient, packet and reconstruction bounds remain in force.

Source I/O failures preserve logical offsets, requested lengths and available
bytes, with index construction identified in the diagnostic. Malformed header
and sequence errors retain parser classification. Failed construction returns
no partial index. Packet or execution failure does not modify the retained
index. Later requests can retry transient source failures, provided the bytes
remain immutable.

## C ABI integration

The additive `emuella_j2k_decoder_create_indexed` constructor requires an explicit
`EmuellaJ2kSourceIndexOptionsV0` with positive `max_header_bytes`, `max_markers`
and `max_tile_parts`. It creates and retains its first successful index lazily on
inspection or decode. Creation validates the copied descriptors and limits but
does not read the source. Indexed operation fails when any ceiling is exceeded;
it never falls back to repeated header traversal.

The original `emuella_j2k_decoder_create` retains its one-shot source semantics
and prior admission, including sources exceeding the new index's default limits.
These are distinct modes: only the explicit indexed constructor promises reuse.
The experimental application probe selects 16 MiB, 65,536 markers and 65,536 tile
parts explicitly; other callers choose limits appropriate to their resource
budget. These ceilings can exclude otherwise admitted inputs, for example a
large collection of COM segments or more markers than the selected cap. This
is explicit indexed-mode admission, not a change to the legacy constructor.

A construction-only mutex prevents duplicate concurrent builds; failed builds
publish nothing and can be retried. Once published, immutable metadata serves
concurrent operations with distinct workspaces. The construction lock is not
held during decode. Existing C symbols and structures retain their layout and contract. The new
constructor and options structure preserve the source callback lifetime rules.
Inspection after construction needs no further callback reads.

This index is for raw Part 1 positioned sources, including an application-owned
NITF image-segment subrange. It adds no container parser, HTTP access, whole-image
pixel cache or change to the separate HT viewing index.

## Calibration and regression evidence

The project-authored probe lives in
`crates/emuella-j2k-test-support/tests/part1_source_index.rs`. Its fixture is an
arithmetic U8 plane encoded by Emuella with two reversible levels and 32×32
tiles. Its initial implementation base is
`3afcfabb24282645c3e101ab3495810d28212dfd`; the delivery pull request records the
exact verified revision containing this index and its tests. No external codec
source, fixture bytes or protected material is required by these probes.

For the 137×73, 15-tile image, the first window is `(3, 5, 9, 7)` in tile zero
and the second is `(101, 39, 9, 7)` in tile eight. After construction, a source
guard rejects every read outside the selected tile payload. Both indexed
requests pass and match the arithmetic pixels, including caller-buffer padding.
The same guard rejects the one-shot route at logical byte zero. Construction
reads no compressed bodies; repeated inspection performs no reads.

| Header organisation | Construction read operations | Retained marker bytes | First window reads / bytes | Second window reads / bytes |
| --- | ---: | ---: | ---: | ---: |
| TLM | 90 | 379 | 8 / 1,414 | 8 / 1,273 |
| Psot, no TLM | 102 | 283 | 8 / 1,414 | 8 / 1,273 |

The opt-in optimised calibration alternates the two windows 21 times. One local
run measured the following medians; timings are observations, not performance
thresholds. Each window retained exactly eight payload/codeword read operations
at both sizes.

| Tiles | Retained marker bytes | Construction | In-memory selection/bookkeeping | Complete preparation |
| ---: | ---: | ---: | ---: | ---: |
| 15 | 283 | 18.9 µs | 0.22 µs | 6.17 µs |
| 1,024 | 14,409 | 452 µs | 1.89 µs | 449 µs |

The complete preparation increase exposes the remaining global geometry and
profile-validation cost. These small authored probes establish the mechanism;
large application inputs still require their own memory and latency calibration.
Retain this design for source-read reuse, without claiming that it removes all
work proportional to the image's tile catalogue.

Ordinary regressions additionally compare full, cross-tile and admitted reduced
requests against one-shot decode; verify full pixels independently; reject
malformed EOC and over-budget construction; preserve source-range diagnostics;
and recover after source I/O failure. C ABI tests cover explicit option
validation, separate legacy admission, lazy index retention,
failed-build retry, metadata-only reinspection, new windows and existing MCT,
precision, panic, failure-atomicity and natural `Send`/`Sync` checks.

Focused commands:

```sh
cargo test -p emuella-j2k-test-support --test part1_source_index
cargo test -p emuella-j2k-codestream part1_source_index_tests
cargo test -p emuella-j2k-capi --lib
cargo test -p emuella-j2k-test-support --release --test part1_source_index \
  index_geometry_scaling_calibration -- --ignored --nocapture
```
