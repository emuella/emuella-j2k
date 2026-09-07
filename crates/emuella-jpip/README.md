# emuella-jpip

This dependency-free Rust package implements a selected stateless HTTP JPP
profile. It builds with `std` or `no_std` plus `alloc`; it contains no HTTP
client/server, operating-system calls, codec dependency, image renderer or
whole-codestream buffer. Applications own HTTP transport and immutable target
resolution. Codec adapters own window selection, dependency planning and bin
assembly from positioned source ranges.

The implementation is a protocol/cache foundation. Package tests do not claim a
composed viewer, general JPIP conformance or external-server interoperability.

## Selected profile

- One immutable target and codestream zero; no channels or persistent server
  cache state. Requests require `target`, `tid`, `fsiz`, `roff`, `rsiz`, `comps`,
  `len` and `type`. Target names use ASCII letters, digits, `-`, `.`, `_` and
  are resolved by an application catalogue, never directly as filesystem paths.
- The frame uses the default `round-down` rule. Explicit rounding tokens are
  outside this parser subset. Offset and size are explicit positive in-frame
  requests. Components are strictly increasing comma-separated indices before
  inverse component transforms; ranges and wildcards are excluded. Optional
  `layers` is a positive layer count. The codec adapter must reject a requested
  layer/window profile it cannot implement; parsing is not codec admission.
- `type=jpp-stream` or `type=jpp-stream;ptype=ext`. Query formatting percent-
  encodes reserved bytes. Parsing rejects duplicate/unknown fields, malformed
  escapes, form-style `+`, channel controls and unsupported variants. The
  caller supplies maximum query bytes and model descriptors.
- Explicit byte-prefix cache descriptors use `Hm`, `H<tile>`, `P<id>` and
  `M<id>`, with optional `:<bytes>`; no suffix means complete. Layer-based,
  subtractive, wildcard and multiple-codestream models are excluded.
- Supported cached classes are precinct 0/1 (one normalised identity), tile
  header 2, main header 6 with identifier zero, and metadata 8. The incremental
  reader skips correctly framed other classes and nonzero codestreams; metadata
  ignores the codestream number. Unknown extended classes still consume their
  auxiliary field. A fresh response resets inherited class/codestream values.
- EOR framing and arbitrary bounded EOR bodies are consumed. The writer emits
  window completion (2), byte limit (4) or response limit (7). `len` counts
  normal message headers and bodies; EOR is outside this limit. Every complete
  response requires EOR and no trailing bytes. A transport interruption is
  explicitly truncated, while already emitted data fragments remain useful.
- `ResponseFields` parses/formats `JPIP-tid`, `JPIP-fsiz`, `JPIP-roff` and
  `JPIP-rsiz` values in the selected syntax. The transport must parse HTTP
  headers case-insensitively and reject duplicate conflicting response fields.
  Target identifiers in this subset are 1–255 printable non-space ASCII bytes.
  Applications send the required changed effective window fields and identity.

Main-header bins contain SOC and the main marker segments, excluding SOT, SOD
and EOC. Tile-header bins concatenate non-SOT tile-part marker segments; the
adapter may omit SOD. It must deliver empty tile-header completion when there
are no such markers. Raw codestream delivery also supplies empty metadata bin
zero. Absence of a bin does not establish that it is empty. Precincts concatenate
inline packet headers and bodies in layer order. Packed packet headers are
outside this initial profile. `precinct_id` computes the standard bin identifier
from tile, component and resolution-ordered precinct sequence; it does not
infer precinct geometry from byte ranges.

## Safe composition

1. Resolve a target to one immutable representation identity, including encoding
   and policy inputs. Start with an empty server model each request and call
   `Request::model_for_identity(actual_tid)` to discard stale assertions. Do not
   trust a model from a different target identity.
2. Build ordered `Demand` entries with the codec's dependency and quality
   planner. Include mandatory main/tile/metadata bins. A `BinSource` adapter
   exposes bin length and positioned reads without materialising the source.
   A requested byte prefix is not inherently a decodable quality boundary.
3. `deliver` reads at most one configured payload chunk at a time and emits a
   bounded response. Its transient payload storage is at most twice the message
   payload cap plus a small header; the output sink must enforce its own queue
   budget. Source packet-count hints are either absent for every prefix of a
   bin or present and monotonic; exact counts are required for extended messages.
4. Before parsing any response body, call `Cache::bind_identity` with the
   validated response identity. An identity change clears cached bins. Identity
   `0` resets on every bind and never produces reusable cache declarations.
   Gate late responses by application request/representation generation before
   rebinding, so a stale response cannot replace the active representation.
5. Feed body fragments to `Decoder::push`, passing data events to `Cache::insert`.
   Discard the decoder after any error and create a new one for the next HTTP
   response. `finish` detects missing EOR and partial headers/bodies. The cache
   can retain bytes already received during an interrupted response.
6. Replay `Cache::model` on each stateless retry or reconnect. Only the contiguous
   prefix starting at byte zero is declared. A final fragment records the bin's
   length; holes prevent complete status. Duplicate bytes must agree. Conflicting
   bytes/lengths, overflow and insertion limits fail before cache mutation or
   eviction. Previously accepted fragments remain present after a later failure.

Cache payload bytes, bin count, intervals per bin and maximum bin endpoint are
separately bounded. Whole-bin LRU eviction removes the corresponding model
assertion. An insertion can temporarily allocate one merged interval up to the
payload cap in addition to retained payloads; metadata is bounded by the bin
and interval caps. Reads use caller-owned buffers and fail without partial
writes when a requested interval is missing. The decoder stores only a bounded
header (64 bytes), forwards borrowed payload fragments and enforces a configured
maximum message/EOR body length; it never buffers a full response.

## Authority and provenance

Implementation, tests and explanations are project-authored from independently
reviewed normative requirements. No external codec implementation, SDK, sample,
source or generated fixture was used as an implementation input.

The selected normative target is ISO/IEC 15444-9:2023 and its corresponding
ITU-T T.808 edition 2 (12/2022), reviewed retrieval
`22382b710222cc98b16da9afde6421abf07c39ef`. Relevant references are Annex A.2.1–3
(message framing), A.3.2.1 (precinct identity), A.3.3–6 (bin contents), C.2
(target identity), C.4 (window fields), C.8.1.2 (explicit cache descriptors),
D.2.1–2 (response fields) and D.3 (EOR and response length). Canonical physical
PDF pages 27–28, 39, 45, 65, 79 and 84 supplied the additional bin, target,
rounding, cache, response and EOR facts. Part 9 edition 3 (June 2026) exists as
prepublished material; differences have not been verified and this package does
not claim conformance to that edition. This document does not reproduce the
standards' expression or examples.

## Verification

```sh
cargo test -p emuella-jpip
cargo test -p emuella-jpip --no-default-features
cargo clippy -p emuella-jpip --all-targets -- -D warnings
cargo check -p emuella-jpip --no-default-features --target wasm32-unknown-unknown
```

The authored tests exercise every fragmentation size across integer boundaries,
independent small wire layouts, inherited and skipped classes, extended fields,
EOR bodies, truncation at every byte, invalid/overflowing headers, body limits,
sparse/final holes, agreeing/conflicting duplicates, empty mandatory bins,
range/bin/byte limits, LRU eviction, target invalidation, strict request/response
round trips, bounded delivery, source/write failure, interrupted receipt,
stateless retry, reconnect and warm-cache reuse. Whole-workspace canonical
verification remains the repository coordinator's delivery gate.
