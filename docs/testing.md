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

### Reversible HTJ2K encode and JPH qualification

The public one-level qualification is algorithmic and self-contained. It uses
project-authored 257 × 193 odd-sized inputs for the eight greyscale/RGB ×
`U8`/`U16_LE` × planar/interleaved cells. Every cell is encoded twice and must
produce identical bytes. Structural parsing must identify an HTJ2K codestream,
the Part 15 SIZ and CAP signalling, HT-only COD block style, exactly one
decomposition level, the reversible 5/3 transform and no multiple-component
transform. Ordinary public component decode must reproduce each native plane
byte-for-byte. The U16 generator spans the complete unsigned native range.

Full-range U16 boundary qualification covers constant zero and maximum planes,
both checkerboard polarities, alternating rows and columns, and isolated
minimum or maximum samples at the first, middle and last positions. It runs at
1–4, 23, 53 and 63–65 pixels, across 64-sample code-block boundaries, cropped
63 × 65, 65 × 63 and 129 × 65 blocks, and odd 257 × 193 geometry. A codestream
test proves that the two 2 × 2 checkerboards transform to HH −131070 and 131070
and signal exponent 17. The scalar MagSgn fallback, empty-small-image
subbands, byte-aligned MEL termination and delayed VLC initial-nibble
consumption are therefore exercised by ordinary encode/decode rather than a
payload classifier. The exact low-level MEL regression is a 23 × 1 block with
coefficients `[1, 0 × 22]` at depth 2; the public regression is a 53 × 2 U8
image whose two rows contain values 0 through 52.

The same tests compare the public zero-level output with the pre-existing
codestream encoder for an algorithmic input, so dispatch widening cannot alter
that byte path. Negative cases cover decomposition counts above one, component
counts outside greyscale/RGB, signed or wrong-endian input, one-level stored
precision outside `U8`/`U16_LE`, and checked geometry overflow. These cases
must return structured errors rather than emit a partially supported stream.
Zero-level bytes are compared directly with the established encoder path, and
the canonical suite retains the existing U8, classic Part 1 and non-HT
behaviour checks.

The JPH writer repeats the complete greyscale/RGB × `U8`/`U16_LE` ×
planar/interleaved matrix at decomposition levels zero and one. Each cell is
written twice, parsed as one JPH container carrying one complete HTJ2K
codestream, and decoded through the ordinary public component route. The
extracted container payload must equal the corresponding `encode_htj2k`
output byte-for-byte. These deterministic Layer 1 checks establish container
structure, repeatability, exact native reconstruction and single ownership of
the codestream bytes; external decoder interoperability remains a separate
authorised black-box qualification.

The JPH admission tests also cover the Annex D boundary independently of
decoder success. Positive cases exercise the required signature, file type,
header and one-or-more codestream ordering; exact `jph ` brand, zero minor
version, required `jph ` compatibility membership, optional additional brands
and harmless duplicate membership; inherited uniform and varying `ihdr`/`bpcc`
forms; legal unknown box preservation; the JPH unknown-colour form without
`colr`; structurally
valid palette/mapping, channel-definition and resolution metadata; multiple
complete HTJ2K codestreams; and writer payload identity. Negative cases cover
missing, misplaced and duplicate structural boxes; conflicting file-type
fields; palette/mapping dependency and selector conflicts; malformed palette
tables and padding; channel-definition count, index, type, association, alpha
and unknown-colour contradictions; missing required colours, incomplete channel
lists and redundant default channel definitions; malformed, duplicate or
reversed resolution children;
absent codestreams; non-HT and incomplete primary or later codestreams;
trailing bytes after EOC; SIZ disagreement in dimensions, component count,
precision or signedness; reserved header fields; short, truncated, undersized
and overflowing box lengths; unsupported presentation versus structural-error
precedence; and caller-buffer atomicity.
A bounded deterministic mutation matrix rejects every strict prefix of a valid
JPH file and one-bit changes across signature magic and protected file-type
fields. Focused tests run through container parsing, inspection, shape
discovery, owned decode, incremental inspection and caller-owned decode.

These tests do not claim optional colour, palette, alpha or composition
support, multiple-codestream presentation, HTMIX support, general encoder or
decoder coverage, or external conformance. Structurally valid but unimplemented
presentation and decode profiles remain explicit support-classification
failures rather than container-invalidity results.

The standards and provenance basis is the bounded route recorded under
"Bounded reversible HTJ2K encode and JPH output" in `architecture.md`. All fixtures,
patterns and assertions are project-authored. The ordinary suite neither
invokes an external codec nor retains external payloads or diagnostics.
Private authorised black-box qualification under registered campaign scratch
passed the project-authored U8 PGM in both directions (2/2 aggregate) and the
full-range U16 PGM/PPM greyscale/RGB cases in both directions (4/4 aggregate)
between raw internal codestreams and installed OpenJPH tools. Only these
aggregate results are public; external payloads and diagnostics remain outside
the repository.

### HTJ2K DS0 qualification

The native HTONLY admission foundation is qualified separately from structural
syntax and packet validity. Project-authored 8 × 8 inputs mutate only the
structured `Ccap^15` value and decode through the ordinary public native API.
The `0x2000` declaration-only multiple-set permission reproduces every expected
sample exactly; declaration-only ROI, heterogeneous and irreversible
permissions exercise the same effective-mechanism boundary in codestream
tests. This calibration selected permission widening because the unchanged
packet contribution contains one effective HT coding set and the existing HT
entropy and reconstruction route already produces exact pixels.

Negative synthetic cases keep the stages distinct. A SINGLEHT declaration over
an actual second HT set is an invalid CAP contradiction before support
admission, including when the sets span tile parts. With multiple sets
permitted, the same actual mechanism is structurally valid but remains an
unsupported native mechanism and cannot modify caller-owned output. This
includes a later zero-byte second Cleanup-set announcement after the first
non-empty set; leading zero-byte placeholders do not count as actual sets. A maximum
checked-pass-count sequence proves native admission consumes every packet but
retains only two set records before rejecting multiplicity. Actual RGN use,
heterogeneous tile-header coding, irreversible coding, HTMIX/HT-declared
population modes and excessive cleanup magnitude bounds remain fail closed.
These tests qualify only admission to the existing bounded lossless HT decode;
they do not qualify other Part 15 mechanisms or claim general conformance.

Project-authored native-grid tests cover horizontal, vertical and two-axis
subsampling, odd non-zero reference origins, native-plane dimensions and exact
three-level reversible reconstruction. Effective COC/QCC tests deliberately
replace the raw irreversible defaults with reversible component state and vary
guard bits while preserving the coefficient-transfer bound.
Six-layer fixtures cover all 24 packets with SOP/EPH and reject a late EPH
contradiction. Adjacent progression, layer, MCT, precision, decomposition,
block-style and multi-tile shapes remain outside the single-component profile.
Explicit precinct tests distinguish one absolute native precinct from the
larger reference grid and reject actual multi-precinct grids. Valid JPH wrappers
remain unsupported by inspection, shape and decode, without publishing samples.

The matching three-component MCT grid branch has separate project-authored
fixtures with two LRCP layers and four tiles. Non-zero image/tile origins,
asymmetric sampling and clipped right/bottom edges reconstruct exact native
component-zero samples before inverse RCT. Packet validation covers all three
components while entropy reconstruction selects only zero. Tests reject
non-matching grids and formats, non-aligned tile phases, wrong MCT/progression/
decomposition/layer state, duplicate parts, tile-header overrides, excessive
sample and tile counts, excluded presentation and selection requests, and JPH.
Owned, shape, retained-workspace and padded caller-planar results agree;
truncation and a last-tile entropy error leave caller storage untouched.
The locked DS0 P0.10 representative comparison qualifies 4,096 native samples
at peak error 3 and MSE 0.459716796875 within 10 and 2.84. This is bounded
component-output evidence, not full RGB support or a general conformance claim.
Public inspection,
shape, owned, caller-retained HT workspace and caller-planar routes agree on
the admitted request; padding is preserved. Wrong phase, empty or oversized
grids, excluded requests, undersized target geometry, truncated input and a
late entropy failure reject without partially publishing caller samples.
A 3,600-case synthetic placeholder-length matrix varies the first pass and
announced pass count, then checks the next header bit exactly. No protected
input or reference sample participates in these ordinary tests.

Project-authored Layer 1 coverage also qualifies the bounded reduced HT
component route independently of protected DS0 material. A 49 × 49 RGB fixture
uses reversible RCT and five reversible 5/3 levels; the component-0 request at
two discarded levels must produce the independently reconstructed 13 × 13
transformed plane. A separate odd 17 × 37 fixture uses five irreversible 9/7
levels in exactly one unsigned 8-bit unit-sampled component without MCT, one
main-header scalar-expounded QCD with independently varied legal per-subband
steps, and varied retained LL/HL/LH/HH coefficient signs and magnitudes. The
same request must produce its exact 5 × 10 component-zero bytes. The
irreversible branch retains raw signed HT
coefficients, normalises their HT bitplane alignment to doubled half-step
`f32` values from each resolved subband magnitude-bitplane count, applies the
resolved subband gain and `0.5 × Delta_b`, performs three 9/7 synthesis levels,
and uses the established finite, ties-to-even, level-shift and clamp
conversion. Metadata, component descriptors, owned planar output and
caller-owned planar output must agree for both branches. Full-image
support classification remains unsupported for the irreversible fixture, and
the full one-level reversible HT MCT regression still reconstructs RGB after
inverse RCT.

Negative cases cover all-components and other-component requests, full and
nearby reduced resolutions, regions, tiles, layer limits, interleaved output,
scalar-derived, reserved or short-step quantisation, missing irreversible
permission, cross-branch component/MCT envelopes, ROI, HTMIX and other
irreversible shapes. Metadata and component-descriptor
queries reject a packet-header or QCD contradiction even when other header
declarations match the bounded shape. Caller sentinels remain unchanged after
target validation failure, malformed irreversible QCD and entropy
reconstruction failure. Numerical boundary cases cover non-finite rejection,
ties-to-even rounding, unsigned level shift and clamp.
An otherwise matching project-authored empty-packet codestream declares
32,768 × 32,768 SIZ geometry and proves that the 16 Mi reduced-sample limit is
enforced before metadata packet state or output allocation. Arithmetic tests
cover the exact limit, the first over-limit row, maximum `u32` dimensions and
discard-level underflow.

The standards route is ISO/IEC 15444-15:2019, clauses 6.1 and 8.3, Annex A,
A.1–A.4, and Annex B, B.3, PDF pages 10, 31 and 35–41, at retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`, plus ISO/IEC 15444-1:2024, Annex
A, A.6.1 and Table A.15, PDF pages 46–48, at retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. All mutations, samples and
assertions are project-authored.

The six-level irreversible HT branch has independent synthetic probes at
17×37, 529×401 and 1025×769, including clipped precinct edges and a retained
output wider than one precinct. Varied signed LL/HL/LH/HH coefficients are
compared with an independently assembled coefficient-domain oracle through
three 9/7 synthesis levels. Twenty-layer RLCP packets delay first non-empty
contributions by resolution; other components carry non-zero coefficients.
Main QCC overrides component zero as well as the two unselected components.
Changing overridden QCD or unselected QCC mantissas must not change the selected
output; changing component-zero QCC must change it. Malformed unselected
quantisation and a malformed final packet fail before metadata publication.
Public tests cover exact geometry, descriptors, padded caller rows, full-image
classifier separation, JPH, neighbouring requests/styles, and oversized SIZ
rejection without caller mutation. The five-level numerical and atomicity
regressions remain unchanged.

The heterogeneous reversible HT reduction-five tests generate three distinct
effective coding and sample-format states (signed 12-bit, unsigned 8-bit and
signed 16-bit), six/seven/eight levels, 64/32/64 blocks and distinct QCC guard
bits. Independent CPRL lattice enumeration writes thirty-layer SOP/EPH packets
with delayed contributions and non-zero unselected/discarded subbands. Odd
65×97, 513×257 and 4129×65 grids cover clipping and retained/discarded
multi-precinct topology. An independently assembled one-level coefficient
oracle verifies exact signed output. QCD/QCC precedence, malformed unselected
quantisation, oversized SIZ and counted linear marker traversal are checked.
Public tests cover metadata/descriptor/executor agreement, signed padded caller
rows, late packet and entropy failure atomicity, full-image separation, JPH and
neighbouring requests. Locked P0.08 compares native signed 12-bit component zero
at reduction five after the existing arithmetic scale to its signed 8-bit
reference; no protected samples enter ordinary tests.

Scalar-derived reduction-three tests independently enumerate actual sampled
PCRL precinct origins for four mixed-transform components. Generated odd grids
65×97, 529×401, 2057×65 and 1033×65 exercise one/four/seven layers, both
128/256 precinct sizes, and retained/discarded precinct crossings. Explicit
subband exponent/mantissa expectations and a coefficient-domain oracle with
independent scales check step expansion, HT half-step transfer and three-level
9/7 synthesis. Every component contributes non-zero packets. Tests check
selected/unselected quantiser behaviour, malformed late packets, linear marker
visits, pre-topology size rejection, and invalid SINGLEHT versus unsupported
MULTIHT. Public metadata, descriptors, owned and padded caller routes agree;
neighbouring requests, JPH, ROI, tile overrides, malformed quantisers and
selected entropy failures preserve caller storage. Locked P0.05 uses its
unchanged component-zero reduction-three and error-bound contract. Other ROI
shapes, tile-header heterogeneity and HTMIX need separate admission evidence.

The heterogeneous reduced ROI branch separately qualifies P0.06. Synthetic
RPCL fixtures enumerate actual reference-grid precinct origins for all four
sampled components, with distinct quantiser guards, per-subband exponents and
mantissas, and main/tile Maxshift precedence. Odd 65×97, 529×401 and 2057×65
grids, one/three/four layers, 128/256 precincts, native 8/12/16-bit signed and
unsigned output and effective shifts 1/3/9 are compared with an independently
assembled coefficient-domain oracle through three 9/7 levels. Threshold
neighbours, signs and alignment are checked directly; the synthetic 529×145
grid counts exactly 144 packets with all unselected components present.
Unselected quantisers and late packets, linear marker visits, pre-structural
resource rejection, invalid SINGLEHT versus unsupported MULTIHT and public
metadata/descriptor/owned/padded-caller parity are covered. Entropy failure and
excluded requests/containers preserve caller sentinels. The locked P0.06
comparison keeps its unsigned 12-bit native plane until the established
Class-0 arithmetic scale to the 65×17 unsigned 8-bit reference; limits remain
109/743. No protected sample enters the ordinary tests.

High-component HT fixtures independently enumerate adjacent RLCP/CPRL volumes
for 4/9/255/256/257 components, including the one-byte end-256 POC sentinel and
two-byte component-256 COC/QCC/RGN selectors. Every component contributes
nonzero data. Per-component blocks, guards and exponents, empty high subbands,
odd grids through 64×64, signed and unsigned 8/12/16-bit output and unselected
ROI shifts 1/3/7/11/15 exercise complete packet validation. A separately
assembled coefficient plane and 5/3 oracle prove selected native output.
ROI-extended magnitude widths are checked on every contribution; existing
sign/threshold restoration is checked separately, not applied to component
zero. Corrupt discarded entropy and changed discarded formats cannot alter
selected output. Malformed unselected quantisers/packets, counted linear marker
visits and zero structural calls for oversized grids guard failure behaviour.
Two-layer contradictory SINGLEHT fixtures remain invalid before unsupported
native layer, cleanup-bound or ROI-selection/shift declines, including shifts
21/22 across the header width thirty boundary, the legal maximum shift 37 and
maximum reversible exponent 31. The same packets with MULTIHT signalling are
structurally valid but still fail native admission. Public inspection,
shape, owned/workspace decode and padded caller output agree; bad packets,
selected entropy failures, excluded requests and JPH preserve caller storage.
The locked P0.13 full native component-zero comparison remains exact at 0/0.

HT ROI window tests construct coefficients and native packet schedules without
external imagery. Signed/unsigned 1/4/8/12/16-bit outputs and shifts 1/3/7/15
exercise background and ROI coefficients, signs, threshold neighbours, guard
alignment, clipping and cropped native output. Odd grids, 32/64 blocks,
one/eight layers, inline SOP/EPH, QCC precedence and empty trailing parts with
TLM exercise complete packet traversal. A maximum 64-tile grid counts exactly
1,024 packets with bounded HT-set retention. Malformed RGN/POC/CRG, quantisation,
oversized grids, late unselected packets, SINGLEHT contradictions and actual
permitted multiplicity fail closed. Public metadata, descriptors, owned and
padded caller output agree; entropy failures leave caller storage unchanged.
JPH, full decode, reduced output and neighbouring requests remain unsupported.
The locked P0.03 and P0.15 full-resolution windows each compare 16,384 samples
at peak/MSE 0/0 within unchanged 0/0 bounds. Their alternative reduced outputs
are not claimed by this route.

The independent tile-progression window route qualifies the locked P0.07
HTONLY native component-zero window: 16,384 samples at peak/MSE 0/0 within
the unchanged 10/0.34 limits. Project-authored coefficient planes cover
8/12/16-bit signed/unsigned formats, 32/64/128-sample tiles, one/two/eight
layers, all three resolution-prefix splits and optional SOP/EPH. The expected
window is synthesised directly from authored coefficients, without external
pixels. A 256-tile probe checks exact packet counts and bounded tile-local
marker work; excessive grids stop before packets. Unselected multiple sets,
late malformed packets, premature POC use and neighbouring coding/quantiser,
ROI/MCT and main POC headers fail closed. SINGLEHT contradictions precede
native cleanup/window declines. Public tests compare metadata, native
descriptors, owned output and padded caller output, including entropy-failure
atomicity and unsupported JPH/request shapes. This is neither HTMIX support
nor a general conformance claim.

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

The [HTMIX architecture decision](htmix-disposition.md) makes this a deliberate
support boundary. Project-authored mixed signalling, packet admission, public
route and caller-atomicity tests preserve it independently of actual MULTIHT
set multiplicity and the individually qualified HTONLY request envelopes.

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

### Rendered partial sYCC

Layer 1 qualifies rendered partial decode with one project-authored in-memory
129 × 65 JP2 fixture. It has reversible 5/3 coding with two decomposition
levels, one tile, Y sampling at 1×1 and Cb/Cr sampling at 2×2. Non-linear
algorithmic planes include periodic 0, 255 and 128 anchors. An independent test
oracle applies the selected binary64 conversion, rounding and clipping without
calling the production renderer.

The x partitions 0, 31, 64 and 129 and y partitions 0, 17, 32 and 65 form nine
requests. Their planar and interleaved results stitch to the full rendered
image, and each request equals the corresponding full-frame crop. Focused
65,33 + 17×9, edge 127,63 + 2×2 and full-frame requests cover preceding
co-sited chroma at odd starts and held chroma at odd image edges. Shape, owned
decode and caller-owned decode agree; output descriptors carry absolute
origins, unit separation and no source-component identity. Absent and all-zero
CRG forms are equivalent.

Prepared-plan evidence checks that the focused request expands to the source
rectangle 64,32 + 18×10, invokes only the selective route, skips positive
packet-body bytes, allocates no full output, and executes fewer code blocks and
source-output samples than the full prepared request. The fail-closed matrix
covers invalid regions and option combinations, raw inputs, incompatible SIZ
and coding shapes, CRG variants, MCT, multiple codestreams, indirect or
additional colour metadata, unrecognised metadata, malformed container and
codestream boundaries, and partial 9/7. Caller-owned sentinels prove that
preflight, target-validation and selected reconstruction failures do not
publish partial output. Native JP2 and raw J2K component results remain the
unchanged control.

The project authority is ISO/IEC 15444-1:2024, clauses A.5.1, A.9.1, B.1–B.3,
I.5.3.1.1, I.5.3.3 Table I.10 and J.14, retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`, reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`. ISO/IEC 15444-4:2024, Annex G,
is the rendered comparison authority, retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`, reviewed bundle
`7b3d8d60cd4d4f6c056cd108d928b7f99f492aa9`. Tests and prose are
project-authored and do not reproduce standards expression.

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

Part 15 signalling tests independently retain CPF values that identify a
corresponding Part 1 profile, distinguish initial placeholder passes from the
first HT set, and reject later zero-length as well as non-empty sets under a
SINGLEHT declaration. HTMIX remains a structurally retained, unsupported
boundary rather than being passed through the homogeneous HT packet validator.

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
