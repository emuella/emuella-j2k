# Classic Tier-1 implementation basis

The classic JPEG 2000 Tier-1 implementation in `emuella-j2k-tier1` is
project-authored Rust licensed under Apache-2.0. Its arithmetic coder and
coefficient-context logic are expressed directly from the normative model;
they are not ports of, or table transcriptions from, another codec.

## Standards authority

The implementation decisions in this module use ISO/IEC 15444-1:2024,
published, role `normative-core`. The controlled local standards record was:

- document ID: `iso-iec-15444-1-2024`
- retrieval root: `15444-1-retrieval`
- reviewed bundle commit:
  `1a7a03799078b476bf38e91786b979059b4c533d`
- retrieval commit: `34e5d1639b9f121807e620c001893ca9d2c8f977`

The arithmetic-coding implementation is based on Annex C, especially C.2,
Table C.2 on physical PDF pages 105-106, BYTEOUT in C.2.7 on pages 108-109,
and the decoder procedures in C.3.2-C.3.5 on pages 113-118. The coding-pass
scan, coefficient contexts, initialization, and termination are based on
Annex D on pages 119-126, especially D.1, D.3, Tables D.1-D.5, D.4,
Table D.7, D.4.1, D.4.2, D.5, and D.6.

The standards corpus and its transcription are engineering inputs and are not
redistributed by this repository.

## Implementation shape

The MQ coder keeps the probability-state index and most-probable symbol as
separate fields. The 47 probability estimates are the normative values from
Table C.2. Encoder and decoder control flow follows the Annex C register
procedures, including byte stuffing, carry propagation, ordinary flushing,
and predictable termination. Output buffering owns an explicit pending byte,
so termination never depends on inspecting or removing a byte already placed
in the caller's output vector.

Raw selective-bypass coding uses a separate MSB-first bit writer and reader.
It applies the seven-bit capacity after `0xff` and finishes partial bytes with
the termination fill required by the selected style. Segment boundaries are
represented explicitly by `CodeBlockSegment` rather than inferred from a
lookahead byte in adjacent codestream storage.

Reference coefficient state uses three named booleans: significant, visited in the
current significance-propagation pass, and magnitude-refined. Neighbourhoods
use eight named directions from Figure D.2. Sign-coding and magnitude-refinement
labels use the project-authored formulas.
Encoder and packed-decoder zero-coding labels use a compile-time lookup generated solely from this module's
existing count formula. Each of four subband rows contains 45 entries indexed by
horizontal, vertical and diagonal significance counts; it occupies 180 bytes
without dynamic allocation. The original formula remains the generator and
oracle, and the checked decoder continues to evaluate it directly. Exhaustive tests check all 256 neighbourhood masks in all four subbands,
direction-to-count mapping and collision-free coverage of the count index.
The reference encoder and checked decoder do not use packed coefficient-state words.

The dense and sparse packed decoders remain Emuella performance backends, but
they call the same Annex C arithmetic decoder and use equivalent context
labels; the checked backend retains the branch-based zero-coding formula.

## Qualification

Unit qualification covers the Table D.7 initial contexts, MQ state evolution
through stuffed bytes, raw termination, sign and zero-context boundaries, and
zero blocks. Code-block round trips cover all four subbands and combinations
of selective arithmetic bypass, context reset, per-pass termination,
vertical-causal context formation, predictable termination, and segmentation
symbols through the checked, dense-packed, and sparse-packed decoders.

The repository's deterministic `gray-gradient-17x19.j2k` generator was also
run before and after the rewrite. Both complete codestreams were 402 bytes and
had SHA-256
`348a05f5696a49320b584ced0576df7cc5d612e4dcb72514ff79521e139675ca`.

## Authored context replay

The opt-in `context_replay` example exercises dense, sparse, patch and diagonal
coefficient patterns, both 64×64 and partial-stripe 17×11 blocks, all four
subbands and 1-, 8-, 16-, 24- and 30-bit magnitudes. It verifies exact
reconstruction through the checked, packed-dense and packed-sparse backends
before timing repeated encode and decode with reusable scratch. It contains no
third-party or corpus material.

```sh
cargo run --profile perf -p emuella-j2k-tier1 --example context_replay -- 32
```

An optional second argument names an output file for the length-prefixed,
project-authored encoded blocks. Run the unchanged replay source against both
revisions and compare these files byte for byte to check encoder parity across
builds. The replay is a local mechanism probe; its timings do not establish
full-image or satellite performance. End-to-end qualification belongs with the
paired codec-consumer measurements.

The initial count-lookup probe also routed checked decode through the lookup.
Four alternating authored replay pairs showed a roughly 5–7% regression for
checked decode on dense coefficient patterns, so that variant was rejected.
The narrower candidate retains direct formula evaluation for checked decode;
it passed all 160 authored reconstructions through all three backends and
matched all 304,926 length-prefixed encoded bytes from baseline
`e703c4d2e59623c91f1ef824c7aff7a13029628c` (the same source tree later merged as
`54b561bc8b70f556d508b730c6f3914247f50f19`). The authored byte sequence has SHA-256
`adda2a1cc7eb3c353e016fdcee6b0ca10920224e038de60e86ee4a0159eb3b44`.
These are mechanism and exactness observations, not a full-image speed claim.

A subsequent [dense first-refinement calibration](tier1-refinement-calibration.md)
measured the existing neighbour bitboard as a replacement for the first-refinement
presence query. The bounded probe preserved exactness but did not meet the
full-image 5% practical improvement gate, so the production implementation was
retained.

## Packed classic encoder

The default encoder stores eight directional neighbour-significance bits
and the significant flag in one `u16` per padded cell. First significance
updates the surrounding cells. Magnitude remains a full
`u32` and sign remains separate, preserving the low-level encoder's accepted
`i32::MIN` magnitude. The implementation retains stripe/column/row traversal,
the existing context formulas and the unchanged MQ/raw writer. The reference
encoder keeps its own state preparation, neighbourhood gathering and pass loops.

Ordered traversal groups four rows and sixteen columns
into each 64-bit word. Separate words track significance, neighbour presence,
visitation, refinement and valid positions; at most 64 such groups cover an
eligible block. Bit order follows stripe/column/row coding order. Significance
propagation recomputes its live candidate mask after each decision and masks
the consumed prefix, including the last-bit case. Newly enabled earlier
positions remain for a later pass. Refinement visits only significant positions
not visited during significance propagation, and cleanup retains four-row run
decisions while skipping unavailable positions. Visit clearing touches the
bounded word array. Authored checks exhaust every eligible geometry's validity
mask and explicitly exercise live propagation across bit 63, word boundaries,
partial stripes and already-consumed positions.

The packed backend is the default for style bytes 0 and 1 with each code-block
axis at most 64. Other accepted styles and geometries use the independent
reference encoder. `EMUELLA_TIER1_ENCODER=reference` forces reference encoding
**at compile time**; `EMUELLA_TIER1_ENCODER=packed` explicitly selects the default
policy, including its fallback. An unset value selects that same default.
Any other value fails compilation. There is no runtime environment access or
change to public APIs, profile admission, entropy coding or scheduling. Record
the build-time selector alongside source and binary identities for comparisons.
An authored test calls every scratch-based entry point and checks which scratch
buffers actually receive state, under both the default and forced-reference
builds, while checking exact bytes against independent reference encoding.

Authored unit tests compare MQ context/decision and raw decision traces, scan
positions, pass boundaries, segment termination lengths, complete bytes and
encode metadata. Coverage includes all four subbands, 1–32 magnitude planes,
missing planes, partial stripes, wide-shape/style fallback, strided input,
output prefixes, known-maximum zero handling and reuse after errors. The trace
wrapper delegates to the same entropy writer and is compiled only for unit
tests. It limits recorded events to a block's decision and boundary envelope;
its fields, allocation and calls are absent from ordinary libraries, including
builds with `test-fixtures` enabled.

The separate `encoder_replay` example uses authored dense, sparse, patch and
diagonal patterns in 64×64 and 17×11 blocks, all subbands, both styles, and
1-, 8-, 16-, 24- and 30-bit magnitudes. It verifies reconstruction with the
checked, dense and sparse decoders outside the timed encoder loop:

```sh
EMUELLA_TIER1_ENCODER=packed cargo run --profile perf \
  -p emuella-j2k-tier1 --example encoder_replay -- 32
```

Its optional second argument writes project-authored bytes together with
length-prefixed segment lengths, coding-pass counts and missing-plane counts.
Run identical example source against both builds and compare that output
byte for byte. Replay timing remains a mechanism probe; it does not substitute
for matched full-image facade measurements.

The retained backend was selected using development inputs before the frozen
confirmation cohort was measured. Confirmation used twenty alternating
fresh-process pairs, conservative 99% per-case intervals and a 5% practical
threshold, with the existing lossless D2 profile, parallel enabled and SIMD
disabled. All 1,920 RarePlanes confirmation invocations were exact. All six
primary one-worker contrasts improved; across the nine-product cohort, 35 of
36 encode comparisons improved. Eight-worker Boca Raton RGB8 bypass remained
inconclusive. The six one-worker decode comparisons were equivalent and the
six eight-worker decode comparisons remained inconclusive; no comparison
established a practical regression against reference encoding.

Separate SpaceNet RGB16 regression coverage passed all 960 exact invocations
and improved all 24 encode comparisons. It covered the twelve fixed Vegas,
Paris and Shanghai chips; Khartoum remained excluded. Independent exact-stream
OpenJPEG evidence was reused only where the stream identity and execution
conditions matched. The broader matched OpenJPEG anchor remained mixed and
was not the promotion criterion. Detailed timing tables, estimator provenance
and external-anchor results belong to the benchmark component. These results
qualify the measured profile and cohorts, not unmeasured profiles, input
families or thread counts.
