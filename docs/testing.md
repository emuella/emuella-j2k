# Testing and corpus boundary

Ordinary tests are Layer 1: small, deterministic, project-authored inputs
created in memory. They require no network, external executable, or adjacent
repository. `emuella-j2k-test-support` contains the shared generator and
round-trip tests.

Optional conformance and interoperability work is Layer 2. The first inspection
smoke runner consumes the pinned `layer2/conformance-jpeg-2000` suite and its
locally materialised `jpeg-2000/conformance` pack:

```sh
cargo build -p emuella-j2k-cli
python3 scripts/run-layer2-conformance-inspection.py \
  --testdata /path/to/emuella-testdata
```

The explicit command is the opt-in boundary. It does not acquire data or
accept terms. Before invoking the codec, it requires the locked catalogue
commit and a clean tracked checkout, checks the suite revision and pack
version, verifies every inventory file and the authoritative archive digest,
and recomputes the complete materialised-tree digest. It then selects every
`.j2k`, `.htj2k`, `.jp2`, and `.jph` inventory candidate, requires exactly one
catalogue-owned format/cohort classification, and inspects each file in its own
process with a per-file timeout. Integrity failures, empty or unclassified
selections, crashes, timeouts, unexpected acceptance or rejection, and
rejection-diagnostic mismatches make the run fail.

The normal invocation binds its evidence to this codec checkout. Run the
documented `cargo build` immediately beforehand; the runner then requires a
clean tracked checkout, reports its full `HEAD`, verifies that the executable
is this worktree's canonical `target/debug/emuella-j2k`, and copies it to one
private execution snapshot. Every candidate runs through that snapshot; its
SHA-256 is reported and rechecked after inspection, alongside the canonical
source path and checkout. This identifies the tested checkout and bytes but is
not a cryptographic build-provenance attestation.

An arbitrary executable may be exercised explicitly with
`--unbound-codec --codec /path/to/executable`. That mode still records the
executable digest, but labels its source revision unbound and produces
diagnostic evidence rather than pinned qualification evidence. It is intended
for project-authored orchestration tests and deliberate external comparisons,
not release qualification.

The deterministic summary separates J2K, HTJ2K, JP2, and JPH results by their
Annex C, Annex G, or informative Annex H cohort. Successful metadata inspection
does not mean native decoding is supported; those are distinct API outcomes.
Ordinary `cargo test` runs only project-authored runner and parser cases and
never invokes this live corpus journey.

Pinned qualification in CI is Layer 3. It should record the codec revision,
catalogue revision, pack ID and version, and pack digest. Restricted packs must
be pre-provisioned. Missing restricted data is never fetched as a side effect
of `cargo test`.

No Kakadu SDK, external codec binary or source, OpenJPEG/OpenJPH output,
standards insert, PDF, OCR transcript, or externally sourced fixture is
permitted in this repository. Another implementation may be executed as an
authorised black-box interoperability oracle, but only project-authored
summaries of observed behaviour may enter public project artefacts; its source,
binaries, fixtures, output payloads, verbatim diagnostics, generated files, and
other artefacts remain outside them. The only source-derived codec
exception is the closed set of OpenJPH-derived files and pinned inputs already
enumerated in `THIRD_PARTY.md`; changing that set requires prior maintainer
approval through an explicit human copyright, licence, provenance, and
architectural review.

Library support classification is structural and algorithmic. Exact-payload
classifiers, embedded reference-pixel replay, empty fixture-array placeholders,
and compatibility APIs named after an external codec do not belong in the
runtime crates. Corpus-specific expectations stay in `emuella-testdata` and its
harnesses.

### Decoded-pixel canary

The decoded-pixel Layer 2 journey consumes a catalogue-owned
`decoded_pixel_comparison` scalar case or choice group. Scalar cases bind one
codestream to one PGX component reference. A choice group binds one codestream
to alternative output contracts and declares how many alternatives must pass.

```sh
cargo build -p emuella-j2k-cli
python3 scripts/run-layer2-decoded-pixel-canary.py \
  --testdata /path/to/emuella-testdata
```

The outer runner applies the same pinned checkout, inventory, complete-tree,
and executable-snapshot checks as the inspection runner. It invokes one Rust
worker with a finite timeout for each comparison it needs to satisfy. The
worker decodes the selected component and output window at the declared zero-
or one-level resolution reduction, parses the PGX reference with
project-authored code, and compares logical samples in memory. Output-window
origins are expressed in the selected output resolution and converted through
checked arithmetic at the worker boundary. Its output is restricted to case
and alternative identities, dimensions, sample count, peak absolute error,
mean squared error, limits, and pass/fail state; input, reference, and decoded
pixel payloads are neither persisted nor printed.

ISO/IEC 15444-4:2024, published conformance-testing authority, Annex B, B.2,
PDF pages 23–28, defines the logical-output preparation and the inclusive peak
and mean-squared-error comparison. Annex C, C.2.1.1–C.2.1.3 and Table C.1, PDF
page 31, identify this Class-0/Profile-0 mapping and assign zero to both limits.
The reviewed transcription used for the contract was retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7` (reviewed bundle
`7b3d8d60cd4d4f6c056cd108d928b7f99f492aa9`). Exactness therefore applies to
corresponding logical samples for this case, not to file bytes.

The P0.03 choice group records two alternative component-0 outputs. One uses
an upper-left 128 × 128 window at full resolution; the other uses a 128 × 128
output after one resolution reduction. Both are signed 4-bit comparisons with
inclusive limits 0/0, and the group requires at least one alternative to pass.
Alternative outputs are choices rather than cumulative requirements.

### Component coding-style overrides

The main-header COC parser and its project-authored synthetic fixtures follow
ISO/IEC 15444-1:2024, Annex A, A.6.1 and A.6.2, including Tables A.15 and
A.18–A.23, PDF pages 46–51. COD supplies the default coding style and COC
replaces the component-local fields for its named component. Components with
no override retain the COD fields. The canonical transcription consulted for
this implementation was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose and
fixtures are project-authored and do not reproduce ISO expression.

The parser rejects malformed lengths, duplicate or out-of-range component
selectors, reserved forms, unsupported explicit precinct forms, and tile-part
COD or COC precedence that the current decoder cannot safely resolve. Decoder
classification, packet parsing, code-block reconstruction, quantisation, and
synthesis all consume the same resolved style; a parsed COC is therefore
exposed only when its effective component style is unambiguous within the
supported main-header boundary.

### Profile-0 component quantization overrides

The native component multi-tile decoder resolves a main-header QCC over the
main QCD only for the component selected by QCC. Components without an
override retain the QCD default. The resolver derives the selector width from
the SIZ component count and applies the same checked quantization-style,
guard-bit and step-size parsing to QCD and QCC. Selective decode validates
wavelet compatibility after QCC precedence for each requested component;
unrequested defaults are still parsed structurally but do not widen the
selected-component contract.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.6.4–A.6.5,
including Tables A.9 and A.28–A.31, and Annex A, A.10, including Table A.45,
PDF pages 41–42, 52–54 and 63–64. The canonical transcription consulted for
this implementation was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose and
fixtures are project-authored and do not reproduce ISO expression.

Synthetic regressions cover QCC before QCD, QCD fallback for unmentioned
components, effective selected-component compatibility, the one-byte/two-byte
selector boundary, end-to-end native multi-tile decode, and fail-closed
handling of truncated, duplicate, out-of-range, reserved or
transform-incompatible forms. Tile-header QCC precedence and encoder-side QCC
production remain outside this decoder increment.

### Bounded Profile-0 progression override

The native component decoder accepts one main-header POC record when its
effective intersection with the codestream covers every component, resolution
and quality layer in LRCP order. Legal component and resolution end values may
extend beyond the actual codestream domain; the decoder intersects those ends
with the SIZ and COD domains. The POC order replaces COD progression for packet
walking without changing COD's layer or decomposition counts.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.6.6 including Figure
A.15 and Table A.32, and Annex B, B.12–B.12.3 including Figures B.14–B.15,
PDF pages 54–55 and 96–99. The canonical transcription consulted for this
implementation was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose and
fixtures are project-authored and do not reproduce ISO expression.

Synthetic regressions cover COD override, broad legal end bounds, successful
native multi-tile reconstruction, inline SOP/EPH combinations, malformed
lengths, duplicate markers, multiple or partial volumes, reserved orders and
tile-header precedence. Multiple progression volumes, other POC orders,
tile-part POC precedence and encoder-side POC production remain outside this
decoder increment.

## Inline packet markers

The native subsampled-component path and the byte-verified bounded-POC
selective component path consume inline SOP and EPH syntax at structurally
known packet boundaries. SOP remains optional for each packet when COD permits
it; an observed `Nsop` must equal the packet's tile-scoped 16-bit sequence
value, including packets that omit SOP. EPH is required immediately after
every inline packet header when COD signals it. Marker length, sequence,
placement, truncation, signalling, packet-body bounds, and PLT agreement are
checked before Tier-1 reconstruction.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.6.1 and A.8.1–A.8.2,
Tables A.12–A.13 and A.40–A.41, and Annex B, B.10.8, PDF pages 46–49, 60–61,
and 93–95. The canonical transcription consulted for this implementation was
retrieval revision `34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed
bundle `1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose
and synthetic fixtures are project-authored and do not reproduce ISO
expression.

Packed packet headers, multiple tile parts in this native profile, explicit
precincts, and encoder-side SOP/EPH production remain outside this increment.
Synthetic bounded-POC regressions exercise SOP-only, EPH-only and combined
signalling, fail-closed sequence and placement errors, and the requirement for
a valid POC admission. Other decoder profiles retain their existing packet-
marker support boundary.

## Bounded tile-header Maxshift

The byte-verified bounded-POC selective component path accepts the single
tile-header RGN assignment required by P0.03: tile 0, component 0, Maxshift
style `Srgn=0`, and `SPrgn=7`. It increases that tile-component's Tier-1
available magnitude-plane count by seven, then realigns each signed decoded
coefficient exactly once before coefficient placement and inverse reversible
5/3 synthesis. Magnitudes at or above `2^7` are shifted right while their sign
is preserved; smaller background magnitudes remain unchanged.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.6.3 and Tables
A.24–A.26; Annex A, Table A.45; Annex D, D.2.1–D.2.2; Annex E, E.1; and Annex
H, H.1, PDF pages 51–52, 63, 119–120, 129–130, and 156. The canonical
transcription consulted for this implementation was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose and
synthetic fixtures are project-authored and do not reproduce ISO expression.

The same bounded path recognises and validates the optional main-header CRG
component-pair syntax as informational metadata. ISO/IEC 15444-1:2024, Annex
A, A.9–A.9.1 and Table A.42, PDF pages 61–62, states that CRG has no effect on
codestream decoding. The selected raw component samples therefore remain
unchanged; rendered registration, component placement and resampling are not
implemented.

Synthetic POC-plus-SOP regressions cover signed coefficient realignment,
windowed and full selected output, unaffected tiles, exact reconstruction, and
fail-closed malformed lengths, selectors, reserved styles, duplicate or
main-header assignments, shifts outside the locked path, missing POC, and
unrepresentable plane widths. General ROI masks, main-header or multi-part RGN
precedence, other tile/component/shift assignments, irreversible or HT paths,
encoder-side ROI production, and other decoder profiles remain outside this
increment.

## Reversible QCD magnitude planes

The classic decoder resolves every LL, HL, LH, and HH sub-band through one
quantisation calculation. For reversible no-quantisation, the available
magnitude-plane count combines the QCD guard-bit field and the sub-band
exponent, with one deducted from their sum. Packet-header missing-MSB state is
then deducted by the shared Tier-1 contract. Impossible subtraction and coding
pass counts above the resulting cumulative bit-plane capacity remain errors;
valid layer truncation may contribute fewer passes. A non-empty contribution
with more than 30 reconstructed magnitude planes is also rejected because the
current checked and packed classic coefficient stores cannot represent a wider
result without colliding with their signed 32-bit boundary.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.6.4 and Tables
A.28–A.29; Annex B, B.10.5–B.10.6; Annex D, D.2–D.3; and Annex E, E.1 and
E.1.2, PDF pages 52–53, 92–93, 119–120, and 129–130. The canonical
transcription consulted for this implementation was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose and
synthetic fixtures are independently project-authored and do not reproduce ISO
expression.

The synthetic regressions qualify guard-bit values below, at, and above the
historical two-bit case, the maximum syntactic magnitude width, consistent
application across resolved sub-bands, exact and excessive coding-pass bounds,
impossible missing-MSB or zero-width claims, and fail-closed dispatch across
the checked and both packed decoder backends when the current coefficient
representation is too narrow. Other packet or quantisation extensions remain
outside this increment.

## Reserved parameterless marker basis

The project-authored parser regression for `0xFF30`–`0xFF3F` records the rule
behind the `p0_02.j2k` repair without copying conformance bytes. ISO/IEC
15444-1:2024, published normative core, Annex A, A.1.2 and A.1.3, PDF pages
32–33, classifies that range as parameterless markers and requires decoders to
skip them. The canonical transcription used for the change was retrieval
revision `34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`).

ISO/IEC 15444-4:2024, published conformance-testing authority, Annex C,
C.2.1.1 and Table C.1, PDF page 31, identifies `p0_02.j2k` as a positive
Profile-0 decoder test with a reference result. Informative C.2.1.4 and Table
C.2, PDF page 32, identify the reserved marker among the syntax it exercises.
That transcription was retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7` (reviewed bundle
`7b3d8d60cd4d4f6c056cd108d928b7f99f492aa9`). The smoke result proves only
that metadata inspection accepts this valid syntax; it does not prove decoded
pixels or every other feature in that conformance input.
