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

Coefficient state uses three named booleans: significant, visited in the
current significance-propagation pass, and magnitude-refined. Neighbourhoods
use eight named directions from Figure D.2. Zero-coding, sign-coding, and
magnitude-refinement labels are calculated from Tables D.1-D.5; there are no
precomputed 256-entry context-label tables or packed context-state words in
the checked implementation.

The dense and sparse packed decoders remain Emuella performance backends, but
they call the same Annex C arithmetic decoder and the same Annex D context
formulas as the checked backend.

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
