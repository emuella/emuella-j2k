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
is this worktree's canonical Cargo debug executable, and copies it to one
private execution snapshot. The canonical path is `target/debug/emuella-j2k`
unless `CARGO_TARGET_DIR` names an explicit build root. Every candidate runs
through that snapshot; its SHA-256 is reported and rechecked after inspection,
alongside the canonical source path and checkout. This identifies the tested
checkout and bytes but is not a cryptographic build-provenance attestation.

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

Run structural inspection as the independent command shown above. It proves
only that the codec can parse and report the selected structures; it neither
decodes nor compares pixels and is not decoded-pixel qualification evidence.

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

### HTJ2K DS0 qualification

The codec-owned derived-set runner consumes the pinned catalogue DS0 contract
and the capability claim in `testdata.lock.toml`:

```sh
cargo build -p emuella-j2k-cli
python3 scripts/run-layer2-derived-set.py \
  --testdata /path/to/emuella-testdata
```

The claim is Profile 0, Class 0, `M_MAGB=18`, with `HTONLY` as the only
supported coding mode. `HTMIX` is deliberately unsupported. For every DS0 case
in a claimed mode, the runner selects the variant with the greatest `B_MAGB`
not exceeding `M_MAGB`; a case with no such variant is not applicable. It
resolves the selected point to exactly one scalar or choice reference contract
and applies the variant's final inclusive error limits. Choice alternatives
remain alternatives and their catalogue minimum determines qualification.

The ordinary invocation executes only applicable `HTONLY` cases. Every such
case must execute within its finite worker timeout and produce an in-limit
native `compare-pgx` result; a decode error, malformed aggregate, timeout or
out-of-limit comparison rejects the qualification run. `--report-all` also
lists `HTMIX` points as `not-applicable` with a deliberately-unsupported
diagnostic, without invoking the worker for them. It does not turn unsupported
points into passes or required failures.

The runner applies the same clean catalogue, exact lock, inventory, archive,
complete-tree, codec checkout and executable-snapshot boundaries as the
inspection journey. Its deterministic output contains only case, coding-mode,
input and `B_MAGB` identities, factual aggregate errors, diagnostics and final
qualified/rejected/not-applicable counts. Inputs, PGX references and decoded
pixels remain in the authorised store and process memory and are never printed
or persisted. The self-contained orchestration tests create only temporary,
project-authored fixtures and do not open the protected pack.

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
worker decodes the selected component and output window at the declared
resolution reduction, parses the PGX reference with
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

The P0.03 and P0.15 choice groups each record two alternative component-0
outputs. One uses an upper-left 128 × 128 window at full resolution; the other
uses a 128 × 128 output after one resolution reduction. Both are signed 4-bit
comparisons with inclusive limits 0/0, and each group requires at least one
alternative to pass. Alternative outputs are choices rather than cumulative
requirements. P0.15 currently qualifies through its one-level-reduced
alternative; its unsupported full-resolution multi-tile form continues to
fail closed.

Scalar cases admit zero through three discarded resolution levels; choice-group
alternatives remain bounded to zero or one. P0.14 exercises the two-level
boundary: the single-tile unsigned 8-bit codestream signals a reversible MCT
and five reversible 5/3 decomposition levels, while the component-mode oracle
selects transformed component 0 before inverse RCT. The decoder reconstructs
only the selected component at 13 × 13 and leaves rendered reduced-MCT output,
other decomposition counts, progression changes, component overrides, packet
relocation, ROI, registration, and inline packet markers outside this bounded
profile. Annex C, C.2.1 and Table C.1 assign the component-0 reference and
inclusive limits 0/0.

P0.04 exercises the scalar three-level boundary: an origin-aligned, one-tile,
three-component unsigned 8-bit J2K codestream uses irreversible 9/7 coding,
MCT, six decomposition levels, 20-layer RLCP, 128 × 128 precincts, termination
on every classic Tier-1 coding pass, and main-header QCD/QCC. Its component-0
comparison remains before inverse ICT. The admitted path rejects other
precinct, layer, code-block-style, component-coding, tile-header quantisation,
ROI, packet-relocation, inline-marker, sampling, component-selection,
reduction, tile, and coding-style shapes.

P0.05 exercises a separate scalar three-level boundary with heterogeneous
component coding styles. Its one origin-aligned tile has four unsigned 8-bit
components, bounded 1×1/2×2 sampling, seven-layer PCRL progression, default
precincts, no MCT, and main-header COC plus QCD/QCC. Packet traversal resolves
each component's effective decomposition count and quantisation while still
reconstructing only irreversible component 0. The qualified shape contains
exactly 175 packet headers; other COC patterns, multiple precincts, POC, ROI,
tile-header overrides, regions, quality-layer limits, component selections,
and reductions remain excluded. Its 128 × 128 oracle uses the inclusive lossy
limits from Table C.1: peak error 54 and mean-squared error 68.

P0.06 extends that scalar three-level boundary to the exact four-component
RPCL shape carrying both heterogeneous coding styles and ROI precedence. Its
one 513 × 129 tile has unsigned 12-bit components with 1×1/2×1/1×2/2×2
sampling, four layers, six decomposition levels, default precincts, no MCT,
main-header COC plus QCD/QCC, and a component-0 Maxshift assignment of eleven
overridden by tile zero with the effective shift nine. Packet traversal uses
the component-specific styles and quantisation across exactly 112 packet
headers, and component-0 coefficients are realigned before irreversible
dequantisation. The decoded native 12-bit plane is reduced to the 8-bit PGX
comparison precision with the simple arithmetic bit-depth scaling required by
ISO/IEC 15444-4:2024, B.2.3.1.5. Other RGN assignments or precedence, COC/QCC
patterns, packet shapes, component selections, reductions and quality-layer
limits remain excluded. Its 65 × 17 oracle uses the inclusive Table C.1 limits:
peak error 109 and mean-squared error 743.

P0.09 exercises the same two-level scalar boundary with an irreversible 9/7
codestream. Its unsigned 8-bit component-0 oracle is 5 × 10 samples and uses
the inclusive lossy limits from Table C.1: peak error 4 and mean-squared error
1.47. The existing reduced irreversible component path satisfies that contract
without widening decoder admission.

The comparison worker also accepts the unsigned PGX reference-header spacing
variant used by the Profile-0 ETS: exactly one blank sign position may appear
between the byte-order field and an unsigned decimal precision. Other empty
fields, repeated separators, tabs and malformed numeric fields remain invalid;
this is a bounded ETS compatibility rule rather than general whitespace
normalisation. The canonical PGX grammar is ISO/IEC 15444-4:2024, Annex B,
B.2.6–B.2.6.1, PDF pages 26–27. P0.16's component-0 reference and exact limits
are defined by Annex C, C.2.1 and Table C.1, PDF page 31. The reviewed
transcription was retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`.

### Direct high-precision greyscale

The direct high-precision JP2 greyscale projection is qualified in Layer 1 by
one project-authored in-memory 7 × 3 fixture parameterised over 9, 12 and 16
bits. Its odd plane includes `0`, `1`, the values immediately below, at and
above the unsigned midpoint, and the two greatest values at each precision.
The test compares logical values and little-endian bytes between native
component and rendered output, checks declared precision, unsigned storage,
greyscale presentation, unused high bits and rendered component provenance,
and exercises owned decode, shape discovery, planar and one-channel
interleaved layouts, and caller-owned decode.

The companion fail-closed matrix covers JP2/SIZ precision and sign mismatch,
missing or contradictory varying-precision metadata, signed greyscale,
precision above 16 bits, wrong or mixed component shapes, unequal sampling,
CRG, MCT, non-zero origins, multiple tiles, decomposition, palette, component
mapping, channel definition, ICC, unrecognised or additional colour
specifications and multiple codestreams. Raw J2K and JPH rendered requests also
remain rejected. Caller-owned sentinels prove that every negative case fails
before mutation. No protected corpus or external codec participates in these
tests.

### Rendered Annex G comparison

The rendered-pixel Layer 2 journey consumes only the catalogue-owned
`rendered_pixel_comparison` plan and its exact locked inventory:

```sh
cargo build -p emuella-j2k-cli
python3 scripts/run-layer2-rendered-pixel.py \
  --testdata /path/to/emuella-testdata
```

The runner applies the same exact catalogue lock, clean tracked checkout,
complete inventory, archive, materialised-tree, canonical executable and
private execution-snapshot checks as the inspection journey. It resolves and
verifies both inventory paths before invoking the dedicated
`compare-rendered-tiff-rgb` worker with a finite per-case timeout. Bound mode
does not accept an arbitrary executable. Missing data follows the locked
suite's failure policy; neither runner nor worker acquires, copies or falls
back to other inputs or decoders.

The worker reads bounded JP2 and TIFF files into process memory and calls only
the ordinary full-frame rendered decode API with best-effort fallback disabled.
It requires the decoded result to be exactly three interleaved unsigned 8-bit
sRGB channels at the declared positive dimensions. The reference parser is a
narrow classic-TIFF reader: `II` and `MM` byte orders are admitted; there is
exactly one IFD; and RGB, chunky, three-sample, 8/8/8 data is stored in one or
more whole-row strips whose logical order may differ from physical file order.
Strips may be uncompressed with an absent/default Predictor 1, or use TIFF LZW
compression with horizontal differencing Predictor 2. LZW decoding is bounded
by each strip's declared logical byte count, uses the TIFF early code-width
transition, requires explicit clear and end codes, and reverses horizontal
differences independently on every row and RGB channel. Required pixel tags are
`ImageWidth`, `ImageLength`,
`BitsPerSample`, `Compression`, `PhotometricInterpretation`, `StripOffsets`,
`SamplesPerPixel`, `RowsPerStrip`, `StripByteCounts` and
`PlanarConfiguration`. The inert baseline tags `NewSubfileType`, `FillOrder`,
`Orientation`, `XResolution`, `YResolution` and `ResolutionUnit` are accepted
only in their narrow non-transforming forms. `Predictor` is accepted only in
the compression combinations above. The byte payloads of `Photoshop` and
`ImageSourceData` (`Adobe Photoshop Document Data Block`) are accepted as
inert metadata with their exact BYTE and UNDEFINED TIFF types; their counts,
offsets and ranges are checked, but their contents are neither interpreted nor
reported. All other tags or compression, palette colour, planar samples,
alpha, non-8-bit samples, additional IFDs, duplicate metadata, inconsistent
counts, invalid LZW codes or expansion, overlapping ranges and trailing sample
bytes fail closed. At most four zero bytes of terminal file padding are inert;
non-zero or longer unreferenced trailing data fails closed.

Every logical RGB byte is compared in memory and the inclusive peak-error
limit decides the result. Worker output is exactly
`components,width,height,samples,peak,limit,passed`; the outer report adds only
the locked pack and case identities and final pass/fail counts. Paths, input or
reference hashes, pixel values and derived images are never reported or
persisted. Case IDs must also match the catalogue's report-safe lowercase ASCII
path-token grammar before they can be printed. Ordinary Rust and Python tests
use only project-authored synthetic JP2/TIFF bytes and do not open a
materialised corpus.

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
selectors, reserved forms, and tile-part COD or COC precedence that it cannot
resolve unambiguously. It retains structurally valid explicit precincts and the
HT coding-method bit so metadata inspection can report them independently of
native decode admission. Decoder classification, packet parsing, code-block
reconstruction, quantisation, and synthesis all consume the same resolved
style and continue to reject combinations outside their supported profiles.

HT COC coding-method signalling follows ISO/IEC 15444-15:2019, Annex A,
A.3.2 and A.4, including Tables A.3–A.4, PDF pages 36–38. Bit 6 identifies an
HT-capable tile-component; bit 7 in combination with bit 6 admits mixed classic
and HT code-blocks. The parser retains both forms, while the current native HT
decoder continues to admit only the homogeneous HT form without additional
style flags. The canonical transcription consulted for this boundary was
retrieval revision `10baf9472429d52f5d6b5f9b7a892dbed395b1db`.

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

### Profile-0 P0.07 tile-part progression schedule

P0.07 has a separate, exact admission path; it does not widen the bounded
main-header POC contract above. The qualified path accepts the locked
256-tile, signed 12-bit reversible shape with eight COD layers, SOP/EPH, one
PLT segment per tile part, and precisely two tile-header POC records for tile
zero. The first tile part contributes the 72 LRCP packets for resolutions zero
through two. Tile zero's final tile part contributes the remaining 24 LRCP
packets for resolution three. Packet-header state and SOP numbering continue
across that tile-part boundary. Only component zero of the upper-left
128-by-128 tile is exposed through the public bounded output-window route;
other regions, components, layer limits and reductions remain unsupported.

The POC syntax, precedence and packet-volume rules follow ISO/IEC
15444-1:2024, Annex A, A.6.6 and Annex B, B.12–B.12.3, PDF pages 54–55 and
96–99. The canonical transcription consulted was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. ISO/IEC 15444-4:2024, Annex C,
C.2.1.1 and Table C.1, PDF page 31, supplies the scalar P0.07 component,
window, precision and error limits; its consulted transcription was retrieval
revision `725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. Project-owned packet-order
and request-shape regressions exercise the 72+24 boundary without embedding
protected conformance bytes.

### Profile-0 P0.08 heterogeneous reversible component

P0.08 has a named reduction-five component route. It admits one origin-aligned
513-by-3072 tile with three signed 12-bit unit-sampled components, exactly
three COC assignments, QCC overrides for components zero and two, thirty-layer
CPRL reversible coding, default precincts, and required SOP/EPH packet
markers. The effective component resolution counts are seven, eight and nine,
forming one exact 720-packet CPRL sequence. The public route exposes only the
full reduced component zero as planar signed 12-bit output at 17 by 96; the
comparison runner applies the existing signed arithmetic conversion to the
signed 8-bit oracle. Other reductions, regions, components, layouts, layer
limits, tile parts, progression changes and marker overrides remain excluded.

The coding-style and component-override rules follow ISO/IEC 15444-1:2024,
Annex A, A.6.1–A.6.2 and Tables A.13–A.20, and the packet progression rules in
Annex B. The canonical transcription consulted was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. ISO/IEC 15444-4:2024, Annex C,
C.2.1 and Table C.1, PDF page 31, supplies the component-zero reduction-five
geometry and error limits; its consulted transcription was retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. Project-owned regressions cover
the packet-order boundaries, exact quantisation payloads and the narrow public
request shape without embedding protected conformance bytes.

### Profile-0 P0.10 subsampled reversible MCT component

P0.10 has a named full-resolution component-zero route. It admits one exact
256-by-256 reference grid split into four 128-by-128 tiles, with three unsigned
8-bit components sampled 4 by 4. The main header contains only the exact COD
and QCD defaults: two-layer LRCP, reversible 5/3, three decompositions, MCT
signalling, default precincts and no inline packet markers. Nine empty-header
tile parts form dense logical payloads for the four tiles, and each logical
tile contains exactly 24 packets. The public output is only the transformed
component-zero plane before inverse RCT, stitched on its native 64-by-64
component grid. Rendered output, other component selections, quality-layer
limits, regions, reductions, tile selection, marker overrides and other
tile-part topologies remain excluded.

The component sampling, MCT and coding-style rules follow ISO/IEC
15444-1:2024, Annex A, A.5–A.6 and Tables A.9–A.20; packet progression and
Profile-0 tile-part ordering follow Annex B and Annex C. The canonical
transcription consulted was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. ISO/IEC 15444-4:2024, Annex C,
C.2.1 and Table C.1, PDF page 31, supplies the component-zero geometry and
error limits; its consulted transcription was retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. Project-owned regressions cover
the exact raw coding and quantisation payloads, per-tile packet order, native
component-grid stitching and the narrow public request shape without
embedding protected conformance bytes.

### Profile-0 P0.13 high-component progression and ROI component

P0.13 has a named full-resolution component-zero route. It admits one exact
1-by-1 raw codestream with 257 unsigned 8-bit unit-sampled components, thereby
requiring the two-byte component-selector forms of COC, QCC, RGN and POC. The
main header contains the exact component-two coding-style override,
component-one and component-two quantisation overrides, component-three
Maxshift assignment, and two POC records. Those records cover components
0–127 in one RLCP volume and components 128–256 in one CPRL volume, for 514
single-precinct packets. The public output is only transformed component zero
before inverse RCT. Rendered output, other components, reductions, regions,
tiles, quality-layer limits, altered marker placement and wider progression
or ROI forms remain excluded.

The two-byte selector boundary and marker fields follow ISO/IEC
15444-1:2024, Annex A, A.5–A.6 and Tables A.9–A.32; POC precedence and packet
progression follow Annex B. The canonical transcription consulted was
retrieval revision `34e5d1639b9f121807e620c001893ca9d2c8f977`. ISO/IEC
15444-4:2024, Annex C, C.2.1 and Table C.1, PDF page 31, supplies the exact
component-zero 1-by-1 output contract; its consulted transcription was
retrieval revision `725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`.
Project-owned regressions cover exact two-byte marker payloads, the 514-packet
two-volume order and the narrow public request shape without embedding
protected conformance bytes.

### Native multi-tile spatial component decode

The public partial API admits one origin-aligned raw Part 1 shape for spatial
decode across tiles: one unsigned 8-bit unit-sampled component, reversible
5/3 coding with exactly two decompositions, one LRCP quality layer, default
precincts and one complete tile part per SIZ tile. Output remains planar at
full resolution. A non-empty image-relative region may intersect any number of
tiles, while `TileSelection` resolves to exactly one checked SIZ tile rectangle
clipped to the image. Region and tile selection are mutually exclusive;
`All` and component zero, full and zero-discard resolution, and unlimited and
one-layer requests are equivalent within this one-component, one-layer shape.

Prepared planning retains only intersecting tile payloads and assembles their
cropped output directly into caller rows. Synthetic 3-by-3-tile regressions
cover one-, two- and four-tile regions, clipped edge and corner tiles,
partition-and-stitch equivalence, positioned sources, caller padding,
selective-work counters and both bounded-window and full-tile synthesis copy
paths. The same profile admits non-zero image and tile origins only when both
origin pairs and both nominal tile extents are multiples of four. This keeps
every clipped tile-component start on the current two-level local synthesis
phase. Public requests remain image-relative while component descriptors
retain their absolute native origins.

The project-authored non-zero-origin fixture encoder writes coherent SIZ
geometry and derives clipped packet and absolute code-block topology before
encoding. Its odd 131-by-99 image uses distinct image and tile origins, clipped
first and final tiles, and exercises the same slice and positioned-source
routes. Unaligned origins or extents, tile origins beyond the image origin,
reduction, signed or higher-precision samples, subsampling, MCT, irreversible
9/7, extra layers, progression changes, coding or quantisation overrides, ROI,
packet relocation, inline markers, fragmented tile parts and interleaved
output remain fail closed at this boundary.

The checked image, tile and component geometry follows ISO/IEC 15444-1:2024,
Annex A, A.5.1 and Annex B, B.1–B.5, PDF pages 41–44 and 79–85. Coding-style
and tile-part structure follow Annex A, A.6 and A.4.2, and packet progression
follows Annex B. The canonical transcription consulted was retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). Tests use only the
project-authored multi-tile encoder fixture.

### Native quality-layer truncation

The public component APIs admit leading-quality-layer selection for one narrow
raw Part 1 profile: an origin-aligned single tile with one unsigned 8-bit,
unit-sampled grayscale component, reversible 5/3 coding with exactly two
decompositions, default precincts, coding-pass termination, LRCP progression,
exactly two declared layers and one complete tile part. Coding-pass termination
provides a standards-valid independently decodable boundary for the layer-one
pass prefix; an unterminated style-zero MQ prefix is not treated as a byte
boundary. Output is the complete image in one planar component. `None`,
`Some(2)` and limits above two produce the complete decode; `Some(1)`
reconstructs only the first contribution and `Some(0)` is invalid.

This two-layer gate is additive to the existing one-layer component profiles.
Their previously admitted spatial, reduced, subsampled, caller-owned and
positioned-source routes continue to treat every positive layer limit as the
complete one-layer output.

The feature-gated project-authored fixture retains the production Tier-1
terminated-segment boundaries. It places a non-empty set of complete pass
segments in layer one and the remaining segments in layer two, while carrying
inclusion, missing-MSB tag-tree and Lblock state through genuine layer-major
packet headers. A separate one-layer codestream is written from exactly the
prefix pass set and serves as an independently parsed pixel oracle. A Tier-1
property matrix separately re-encodes each proposed pass prefix and requires
its bytes and decoded coefficients to match the same leading segments of the
complete emitted codeword. Tests require the limited two-layer decode to equal
that oracle exactly and to differ deliberately from the complete two-layer
image.

Prepared slice and positioned-source tests visit the same packet-header count
for limited and complete requests, while proving non-zero excluded layer-two
body bytes and lower retained Tier-1 byte and coefficient-position work for
the limited request. Positioned-source accounting also proves that excluded
body bytes are not physically read. Malformed later packet headers still fail
closed because every header is parsed; corrupt later bodies may be skipped,
whereas selected-body corruption is rejected before a directly staged public
caller buffer is published. Prepared execution retains its documented
non-transactional boundary after entropy work begins.

Regions, tile selection, every reduction form, non-zero origins, multiple,
subsampled, signed or higher-precision components, MCT, irreversible 9/7,
HT block coding, containers, non-LRCP progression, coding or quantisation
overrides, explicit precincts, inline markers, extra mechanisms and fragmented
tile parts remain excluded from this profile. These exclusions are exercised
through owned, information, retained-plan and positioned-source admission
routes without widening the separate multi-tile spatial predicate.

## Inline packet markers

The native subsampled and unit-sampled component paths consume inline SOP and
EPH syntax at structurally known packet boundaries. The unit-sampled profile
admits the same validated default or explicit precinct geometry with and
without a bounded POC override. SOP remains optional for each packet when COD
permits it; an observed `Nsop` must equal the packet's tile-scoped 16-bit
sequence value, including packets that omit SOP. EPH is required immediately
after every inline packet header when COD signals it. Marker length, sequence,
placement, truncation, signalling, packet-body bounds, and PLT agreement are
checked before Tier-1 reconstruction.

An explicit precinct dimension may be nominally smaller than the declared
code-block dimension only when the actual sub-band boundary clips the
code-block to one precinct in that axis. Geometry that would let a code-block
cross an actual precinct boundary remains unsupported. Empty high-frequency
sub-bands produced when a decomposition reaches a very small component extent
have no code-block or packet-header state.

This behaviour follows ISO/IEC 15444-1:2024, Annex A, A.6.1 and A.8.1–A.8.2,
Tables A.12–A.13 and A.40–A.41, and Annex B, B.10.8, PDF pages 46–49, 60–61,
and 93–95. The canonical transcription consulted for this implementation was
retrieval revision `34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed
bundle `1a7a03799078b476bf38e91786b979059b4c533d`). The implementation prose
and synthetic fixtures are project-authored and do not reproduce ISO
expression.

Packed packet headers, SOP with explicit precincts, multiple tile parts in this
inline-marker profile, and encoder-side SOP/EPH production remain outside this
increment. Synthetic default-precinct regressions exercise SOP-only, EPH-only
and combined signalling both with and without the bounded POC path. A separate
decoded regression covers EPH with explicit precincts. The suite also
preserves fail-closed handling of malformed lengths, sequence and placement
errors, duplicate or unsignalled markers, missing required EPH, unqualified
SOP/precinct combinations, and unsupported tile-part topology. Other decoder
profiles retain their existing packet-marker support boundary.

The native `HTONLY` lossless path reuses this same inline packet walker for
SOP-only, EPH-only and combined packets within its existing single-effective-
precinct profile. Marker admission does not widen its reversible-transform,
tile, component, decomposition, progression or precinct-topology boundaries.
Project-authored HT fixtures cover zero and three decompositions, malformed and
unsignalled markers, sequence and duplication errors, and PLT packet-boundary
crossings. The bounded profile requires coding style to come from the main
header: any tile-header COD is rejected before packed-header or SOP/EPH
validation, including an otherwise identical override with unchanged marker
flags.

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
unchanged. Rendered registration, component placement and resampling remain
unsupported except for the separately bounded full-frame sYCC policy in
[`architecture.md`](architecture.md#bounded-full-frame-sycc-projection).

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
