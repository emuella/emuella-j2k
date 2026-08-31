# Architecture

The workspace separates public policy from codec mechanisms:

```text
emuella-j2k
  └── emuella-j2k-core
        ├── emuella-j2k-container
        └── emuella-j2k-codestream
              ├── emuella-j2k-tier1
              ├── emuella-j2k-ht
              └── emuella-j2k-transform
                    └── emuella-j2k-accel
```

`emuella-j2k` is the stable public facade and re-exports the application API
implemented by `emuella-j2k-core`. The core crate owns caller-visible images,
parameters, errors, support classification, and high-level
inspect/decode/encode entry points. Lower-level crates parse boxes and markers,
walk packets, decode or encode code blocks, and apply transforms. Optional
parallel and SIMD paths retain deterministic scalar fallbacks.

Classic block coding in `emuella-j2k-tier1` uses a project-authored Annex C MQ
coder and Annex D coefficient-context model. Its standards basis, design
boundary, and qualification record are documented in
[`tier1-implementation.md`](tier1-implementation.md).

The CLI and Python crates are adapters; they must not become alternate codec
implementations. The test-support crate creates deterministic inputs through
project-owned algorithms. External reference codecs and corpora are never
runtime dependencies.

The [independent U8 native-plane contract](native-planes.md) selects a bounded
subset of existing Part 1 component decode for atomic full-image publication.
Core uses a crate-private predicate to keep those caller requests on the owned
staging adapter, avoiding partial publication by parallel component jobs.
Codestream admission, prepared/partial execution and encoder APIs are unchanged.
The independently authored native fixture builder belongs to test support and
is reusable by JP2 presentation tests without assigning display roles to native
planes.

## Part 15 signalling and admission

The [HTMIX architecture decision](htmix-disposition.md) records the accepted
unsupported boundary, the mixed-packet admission correction and the distinct
block-discovery/state contracts required by any separately authorised change.

The codestream parser retains SIZ, CAP, PRF and CPF as separate structural
state. CPF identifies the profile of the corresponding Part 1 codestream used
for reversible transcoding; it is not required to equal the profile field of
the current HTJ2K codestream. Part 15 kind identification requires the mapped
Rsiz and Pcap declaration, while native decode admission remains a later,
narrower decision.

Packet-level SINGLEHT checking is bounded: the validator skips every CAP
`Mixed` declaration, even when effective COD/COC is homogeneous HT (`0x40`).
It does not establish SINGLEHT contradictions for those declarations.
For admitted homogeneous grammar, placeholder passes before the
first non-empty cleanup are not HT sets; once that first set has appeared, a
later empty or non-empty set contradicts SINGLEHT. HTMIX syntax remains
structurally inspectable but outside native decode and packet-semantic
admission, because a mixed code-block must first be distinguished from a
classic Part 1 code-block.

Native HTONLY admission is deliberately later than both stages above. The
profile classifier uses effective main-header COD/COC state and parsed packet
contributions, rather than treating every `Ccap^15` permission bit as proof
that its mechanism occurs. Declaration-only permission for multiple HT sets,
ROI, heterogeneous state or irreversible coding can therefore retain the
existing native route when the effective mechanisms are still one HT coding
set per code-block, no RGN use, one supported coding style and reversible 5/3.
The decoder and support classification share the effective packet-mechanism
predicate for the common full-image route. Actual multiple sets, RGN use,
heterogeneous state and irreversible coding are not admitted by that route.
The independently prepared native-grid, reduced, ROI, high-component and
tile-progression routes below have their own bounded effective-state and
request contracts; their envelopes are not interchangeable. HTMIX/HT-declared
population modes and a cleanup magnitude bound above 18 fail native admission
across these routes. A later zero-byte Cleanup-set
announcement counts as an actual second set once the first non-empty set has
appeared; leading zero-byte placeholders do not. Native admission walks the
complete packet source but retains only the first and latest set once this
unsupported multiplicity is known. Packet contradictions are
still validity errors before unsupported-mechanism admission.

The native full-resolution partial API also has an independent tile-progression
window route. It admits only the bounded three-level, three-component,
ROI-free reversible envelope documented in the README, with two tile-zero
LRCP POC volumes and inherited RLCP elsewhere. Main coding and quantisation
are uniform; this does not grant heterogeneous COD/COC/QCD/QCC or MCT.
Resource preflight precedes packet work. One linear partition creates small
tile-local header scopes, so the existing effective POC resolver and packet
walker do not repeatedly scan the global marker list. All components and all
tiles are packet-validated, with unsupported native mechanisms deferred until
later tile validity is known. Only tile-zero/component-zero contributions are
retained. Selected payload spans are joined in memory for the existing HT
coefficient-transfer and three-level 5/3 seam, then cropped privately before
publication. Metadata, descriptors and owned/caller decode share the prepared
plan. No raw main-header inference replaces effective POC scope or availability.

The native partial API has an HT-owned reduction-two transformed-component
request with reversible and irreversible reconstruction branches.
After the same structural and effective-mechanism stages, it admits a raw,
origin-aligned single tile with one-layer LRCP packets and five decomposition
levels. The reversible branch requires three matching unsigned 8-bit,
unit-sampled components, MCT, 5/3 and the established no-quantisation QCD
contract. The irreversible branch instead requires exactly one unsigned 8-bit,
unit-sampled component, no MCT, 9/7 and exactly one main-header scalar-expounded
QCD; QCC and tile-header quantisation state remain excluded. A
request must select component 0, discard exactly two levels and use planar
output without a region, tile or layer limit.
Packet walking reuses the existing single-effective-precinct inline-header
route, including signalled SOP and EPH; packed headers remain excluded. Packet
parsing still validates the complete source, while entropy and
coefficient work is retained only through resolution 3 for component 0. Three
inverse-DWT levels reconstruct the checked reduced geometry. Reversible HT
coefficients retain the existing transfer and 5/3 synthesis. Irreversible HT
coefficients remain raw signed `i32` values. Placement normalises their HT
bitplane alignment to doubled half-step `f32` values from the resolved subband
magnitude-bitplane count, applies the resolved subband gain and
`0.5 × Delta_b`, then uses the existing 9/7 synthesis and finite, ties-to-even,
level-shift and clamp conversion.
Publication occurs before inverse colour transformation. Ordinary full HT
decode retains its existing inverse-DWT and inverse-RCT behaviour. One HT-owned
prepared plan supplies the checked geometry to the public metadata and
component-descriptor APIs and retains the same packet contributions for owned
decode; no geometry is published before complete structural and
effective-mechanism admission. Full-image support classification remains
unsupported for the irreversible profile; only the exact partial request owns
this admission. This mechanism is owned by the HT decomposed path and does not
reuse the raw Part 1 selective planner.
Before packet-state construction, checked reduced geometry is limited to 16 Mi
component samples, matching the native profile component bound. The retained
`i32` or `f32` plane and transform scratch use fallible reservation, so
oversized SIZ declarations and allocation failure remain structured decoder
errors rather than process-aborting allocation attempts.

This boundary follows ISO/IEC 15444-15:2019, clauses 6.1 and 8.3, normative
Annex A, A.1–A.4, and Annex B, B.3, PDF pages 10, 31 and 35–41, at retrieval
revision `10baf9472429d52f5d6b5f9b7a892dbed395b1db`, together with the effective
COD/COC precedence in ISO/IEC 15444-1:2024, Annex A, A.6.1 and Table A.15, PDF
pages 46–48, at retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The admission logic and synthetic
tests are project-authored and reproduce no protected standards text or
payload.

## Native HTONLY component grids

### High-component native output

The independently granted HT high-component permission checks the README's
component, geometry, coding and header envelope before packet topology. Its
structural envelope allows up to two layers so SINGLEHT contradictions can be
reported before the one-layer native decline. It resolves the two complete
adjacent RLCP/CPRL volumes, every effective COD/COC and QCD/QCC, and one main
RGN with normal one/two-byte selectors. No Part 1 permission is granted and no
functional tile override is admitted. Marker scanning is linear; component
lookups are bounded by 257, geometry by 64×64 and packet scheduling by 1,028
packets before the native one-layer limit reduces that to 514.

The ROI's extended quantiser width applies to its unselected component during
packet validation. This permission retains header magnitude metadata beyond
the classic coefficient-store limit, so every legal shift through 37 remains
structurally checkable. Native shift and coefficient-width limits still apply
afterwards; other packet permissions keep their existing bounds.
All packet headers, lengths, magnitude declarations and
HT-set signalling are validated before selected-plane retention. Discarded
components are not entropy-decoded or reconstructed. Component zero reuses
the prepared reversible executor with zero discarded levels and no inverse
RCT. The full planar component-zero request and metadata shape share that
plan; reconstruction completes in private storage before caller publication.
General all-component inspection remains unsupported for this shape.

This follows Part 1:2024 A.6.1–A.6.6 and B.12 (physical pages 46, 49–55,
96 and 98–99), retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`;
Part 15:2019 A.5 (page 38), retrieval
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`; and Part 4:2024 B.2.3/B.2.5
(pages 25–26), retrieval `725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`.
The code and synthetic fixtures are project-authored; no external payload,
pixels or standards expression enters the public tree.

### Native ROI window

The HT-owned prepared ROI plan is independent of the Part 1 selective planner.
It validates the one-component, one-level, unit-grid envelope documented in the
README, resolves main QCC over QCD and the tile-zero Maxshift, and checks one
full-domain main POC LRCP schedule. TLM reconciliation remains structural;
empty trailing parts do not become new packet sources. Every tile's complete
packets are validated with first/latest-set retention before output geometry
is exposed. Actual multiple sets still fail closed, with SINGLEHT
contradictions remaining invalid. The maximum grid is 64 tiles of at most
128×128 samples; that bound precedes packet topology and retained allocation.

The block-local ROI candidate admits native precision below eight without
changing the ordinary HT candidate classifier. HT placement first normalises
decoded coefficients to the ROI-extended integer magnitude domain using the
resolved subband exponent and guard bits. Maxshift restores above-threshold
ROI magnitudes while preserving signs and background values, before the
existing checked reversible synthesis and native clipping/byte conversion.
Only tile zero reaches entropy decoding; a region within that tile is cropped
from private storage. Metadata, descriptors and caller-owned publication use
the same prepared route. Full-image support classification remains unsupported.

The basis is Part 1:2024 A.6.3/A.6.6, A.7.1, B.12.3 and H.1, physical pages
51–52, 54–57, 99 and 156, retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`; Part 15:2019 A.5, page 38,
retrieval `10baf9472429d52f5d6b5f9b7a892dbed395b1db`; and Part 4:2024
B.2.3/B.2.5, pages 25–26, retrieval
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. This independently authored
mechanism contains no standards expression or protected payload/pixels.

### Six-level irreversible reduced component

The additional reduction-three branch owns its admission independently of the
Part 1 selective profiles and the five-level HT branch. It requires one
zero-origin tile/part, three matching unsigned 8-bit unit-sampled components,
MCT, six 9/7 levels, twenty RLCP layers, 64×64 HTONLY blocks, and explicit
128×128 precincts at all resolutions. Only main-header scalar-expounded QCD
and optional QCC are admitted; quantisation resolves for every component,
including unselected components. COC, tile overrides, ROI, progression changes,
packet markers/relocation and registration remain excluded.

An HT-owned packet permission is independently rechecked by the shared walker;
it does not grant the classic P0.04 capability or broaden full-image HT support.
The block coder is precinct-neutral only after that envelope is established.
All packets are validated, with at most the first and latest HT sets retained
during admission; actual multiple sets remain unsupported. Reconstruction
retains only component zero through resolution three and reuses the existing
half-step dequantisation and three-level 9/7 seam. No inverse ICT is performed.
Metadata and caller-owned routes share the prepared plan, with owned
reconstruction completing before caller publication. The full reference plane
is limited to 16 Mi samples before packet topology; the existing packet-count,
precinct-state, reduced-plane and fallible-allocation limits also apply.

The changed packet and quantisation envelope follows ISO/IEC 15444-1:2024,
A.6.1/A.6.4/A.6.5 and B.12.1.2 (physical pages 46–47, 52–53 and 96), at
retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`. Matching-grid ICT
conditions are in G.3 (pages 154–155). Transformed component-zero qualification
is distinct from display RGB, following ISO/IEC 15444-4:2024 B.2.3.1.2 and
B.2.5 (pages 25–26), retrieval
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. All implementation and synthetic
fixtures are project-authored; no protected payload or pixels are embedded.

### Heterogeneous reversible reduced component

The reduction-five HT-owned plan resolves each component's effective main
coding and quantisation before packet topology. It admits three unit-sampled
8–16-bit components with independently signed or unsigned formats, one
zero-origin tile/part, no MCT, reversible 5/3, six levels for component zero
and six through eight for the others. The packet boundary is CPRL with one
through thirty layers, 32/64-sample code-block axes, explicit square 128/256
precincts and optional inline SOP/EPH. Only main COD/COC/QCD/QCC are functional
overrides; ROI, POC, packet relocation and tile-header state fail closed.

An independently rechecked HT permission grants heterogeneous multi-precinct
packet traversal, without granting any classic Profile-0 permission. Every
component's quantiser must resolve to no quantisation with positive exponents
and a magnitude transfer bounded to 31 bits. When all components override QCD,
the unused default is validated against COD's own decomposition count.
The full reference plane is bounded to 16 Mi samples before packet preparation;
existing packet/precinct-state limits and fallible retained-plane allocation
remain in force. Admission retains at most first/latest HT sets while scanning
all packets, then rejects effective multiplicity. Only component zero through
resolution one reaches entropy and reversible synthesis. The block candidate
describes this selected plane after heterogeneous packet admission, not a
fictionally homogeneous source image. Public metadata and owned/caller output
share the prepared plan and preserve the selected native signedness/precision.

This separation follows ISO/IEC 15444-1:2024 A.6.1/A.6.2, A.6.4/A.6.5 and
B.12.1.5 (physical pages 46, 49–54 and 98), retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`. Part 4:2024 B.2.3/B.2.5
(pages 25–26), retrieval `725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`,
separates native reconstruction from Class-0 signed arithmetic scaling.
Project-authored fixtures and implementation contain no protected payloads.

### Scalar-derived sampled reduced component

The independent reduction-three sampled-PCRL permission admits four unsigned
8-bit components with sampling 1×1/1×1/2×2/2×2, effective decomposition counts
6/3/6/6 and transforms 9/7, 9/7, 9/7 and 5/3. One zero-origin tile/part,
no MCT, one through seven layers, 32×32 blocks and explicit square 128/256
precincts bound the packet shape. Only main COD/COC/QCD/QCC overrides are
functional; inline SOP/EPH, ROI, POC, relocation and tile-header state remain
excluded. The existing 16 Mi-sample reference-plane cap is checked before
packet topology; packet/precinct caps and fallible retained allocations remain.

The shared packet walker independently rechecks this HT-owned permission and
resolves each native component grid, coding style and quantiser. Component zero
must use scalar-derived steps, components one/two scalar-expounded steps and
component three reversible exponents. Positive resolved exponents and at most
30 guard-adjusted magnitude bits bound every effective quantiser, including
unselected components. Scalar-derived expansion remains relative to the full
component's subband sequence, not a renumbered reduced image. Each retained
contribution carries its resolved step and magnitude width into the existing
HT half-step transfer and 9/7 synthesis. The entropy candidate represents only
the selected unit-sampled plane after complete packet admission. Only component
zero through resolution three reaches reconstruction; metadata and caller
output reuse that prepared plan. This does not grant full-image, resampled or
container output, other selectors/reductions, ROI or HTMIX support.

The quantisation and sampled-PCRL basis is ISO/IEC 15444-1:2024 A.6.1/A.6.2,
A.6.4/A.6.5, B.12.1.4 and E.1.1 (physical pages 46, 49–50, 52–54, 97 and
129–130), retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`.
Class-0 component-zero reduced comparison follows Part 4:2024 B.2.3/B.2.5
(pages 25–26), retrieval `725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`.
All implementation and synthetic evidence is project-authored; no standards
expression, protected payload or decoded pixels enter public artefacts.

### Heterogeneous reduced ROI component

The independent sampled-RPCL HT permission resolves the four native sampling
grids and main coding/quantisation overrides documented in the README. A
linear marker scan admits one main and one tile component-zero RGN; the tile
assignment overrides the main without applying both shifts. No Part 1 profile
permission is granted. Geometry, tile/component counts and six-level topology
are bounded before structural packet validation. Within that envelope,
SINGLEHT contradictions precede native cleanup-bound, effective-shift and
request declines. Every component and packet is validated before retaining
component zero through resolution three.

Irreversible HT transfer first tests the ROI threshold in the ROI-extended
coefficient domain. It restores the doubled ROI coefficient using integer
alignment before floating-point conversion, with zero ROI reconstruction bias;
background values retain the existing HT half-step. Each contribution keeps
its original full-component subband quantiser and magnitude width. Existing
three-level 9/7 synthesis and native precision/sign conversion then execute
the prepared plan. This is not WP-09's reversible integer restoration, although
the sign and threshold ownership are shared. Metadata, descriptors and owned
or caller-planar output share admission; caller publication remains atomic.

The basis is Part 1:2024 A.6.3–A.6.5, B.12.1.3, E.1 and H.1 (physical
pages 51–54, 97, 129–130 and 156), retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`, Part 15:2019 A.5 (page 38),
retrieval `10baf9472429d52f5d6b5f9b7a892dbed395b1db`, and Part 4:2024
B.2.3/B.2.5 (pages 25–26), retrieval
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`. Public code and synthetic
fixtures are project-authored without protected payloads or standards expression.

### Full native grids

The full native component-grid route has its own prepared admission plan. It
requires one unsigned 8-bit non-unit-sampled component, one tile and tile-part,
three effective reversible 5/3 levels, one through six LRCP layers, one
effective precinct per resolution and inline headers (including SOP/EPH).
Main COD/COC and QCD/QCC resolve before block dimensions, transform and
coefficient-transfer guard bits are selected. Unused irreversible defaults do
not change an effective reversible component into an irreversible one.

Absolute SIZ endpoints determine the native plane. The 16 Mi-sample bound is
checked before packet-state construction. Component origins aligned to eight
samples preserve the existing low-pass-first phase through all three synthesis
levels, including non-zero and odd reference-image origins. Other phases fail
closed. The plan retains native dimensions alongside admitted contributions;
the HT block coder receives a grid-neutral block candidate only after this
ownership boundary. Reversible coefficient transfer and integer synthesis use
the native dimensions, not the reference-image dimensions.

The public route accepts only raw codestreams, component mode and planar all-
or component-zero selection. Reference-image `ImageInfo`/`decode_shape` and
native `ComponentInfo` remain distinct. Owned reconstruction completes before
copying validated native rows into caller storage; row padding is untouched.
No rendered resampling, JPH projection, MCT, multi-tile composition, ROI,
tile-header overrides, reductions or layer limits are admitted. The existing
unit-sampled lossless and reduced classifiers retain their boundaries.

A second shape uses this prepared grid mechanism for three matching unsigned
8-bit sampled components with reversible MCT and one to 64 tiles. It admits
exactly one complete part per tile, the same homogeneous three-level coding
style and effective reversible quantisation, and one native precinct per
component/resolution/tile. Every absolute tile-component origin must align to
eight samples; image-edge clipping may produce smaller final tiles. Aggregate
native component samples are bounded to 16 Mi before packet preparation.
Tile payloads and admitted contributions remain paired in the prepared plan.
Only component zero undergoes HT entropy decoding and inverse DWT; the checked
native tile rectangles place its samples into a privately allocated image
plane. The other components' packet grammar is still validated.

This branch requires explicit planar component-zero selection and publishes
the transformed codestream component before inverse RCT. It does not decode
display RGB and then attempt to recover luminance after clipping. All-component
output, other selectors, rendered resampling, JPH, ROI, heterogeneous tile
state, reductions and layer limits remain excluded. The original one-component
shape is still single-tile and permits either all or component-zero selection.
Geometry follows ISO/IEC 15444-1:2024, B.2/B.3 and F.1–F.3 (physical pages
80–82 and 132–133); the matching-grid RCT requirements are in G.2 (page 154),
retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`. The native transformed
component comparison is distinct from full image reconstruction, as described
by ISO/IEC 15444-4:2024, B.2.3.1.2 and B.2.5 (pages 25–26), retrieval
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7`.

Leading placeholder-only layered HT contributions consume a complete
zero-length field covering their announced passes before the next block's
header. This repairs packet alignment without treating placeholders as actual
HT sets or relaxing SINGLEHT validity. The geometry and override basis is
ISO/IEC 15444-1:2024, A.6.1/A.6.2, A.6.4/A.6.5, B.2/B.3 and F.1–F.3,
physical pages 46–53, 79–82 and 132–133, retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`; the placeholder basis is
ISO/IEC 15444-15:2019, B.1–B.3, physical pages 40–41, retrieval
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`.

## Bounded reversible HTJ2K encode and JPH output

`encode_htj2k` writes raw HTJ2K codestreams only. Its
`Htj2kEncodeOptions::decomposition_levels` field admits zero or one: zero keeps
the established cleanup-only byte path, while one applies one reversible 5/3
level before the same repo-owned HT cleanup block boundary. Two or more levels
fail explicitly. The additive `encode_htj2k_jph` entry point accepts the same
options and input matrix. It calls the raw encoder once, then places those exact
bytes in one contiguous codestream box behind deterministic JPH file-type,
image-header and enumerated greyscale or sRGB colour boxes. Container writing
does not introduce a second HT encoding route.

The JPH reader and writer share one bounded Annex D container contract. A JPH
file has exactly one JPEG 2000 signature, followed immediately by exactly one
file-type box, and exactly one `jp2h` after that file type and before the first
`jp2c`. The file-type brand is `jph `, its minor version is zero, and its
compatibility list contains `jph ` membership; repeated compatible-brand
membership is harmless. The inherited `jp2h`
boundary requires `ihdr` first, conditionally admits one `bpcc`, keeps `colr`
boxes contiguous, and validates the cardinality and payload structure of
`pclr`, `cmap`, `cdef` and `res `. Palette and component-mapping dependencies,
component and palette selectors, channel-definition domains, and resolution
children are checked before optional presentation is classified. For
enumerated greyscale, sRGB and sYCC, an explicit channel definition must cover
every channel and required colour; the default ordered colour channels must
omit that box. Unsupported ICC and vendor methods do not acquire an inferred
colour count. When both resolution children occur, capture resolution precedes
default display resolution. JP2 still
requires `colr`; JPH may omit it only for an `ihdr` whose colourspace is
unknown, and such a header cannot describe a type-0 colour channel. The JPH
single-alpha and application-specified channel rules are applied without
changing JP2 rules. At least one `jp2c` is required, every `jp2c` must contain a
structurally parsed HTJ2K codestream with EOC at the end of the box payload,
and the first codestream SIZ dimensions, component count, precision and
signedness must agree with `ihdr` and `bpcc`. Checked box arithmetic, field
domains and exact containing-box bounds precede metadata allocation. Legal
unknown boxes remain byte-preserved.

This validation is container and codestream admission, not presentation or
decode expansion. Palette, component mapping, channel definition, alpha, ICC,
unrecognised colour interpretation, multiple-codestream composition, HTMIX and
codec profiles outside the existing decoder remain unsupported. A structurally
valid file can therefore inspect as unsupported; structural contradictions are
rejected first. Caller-owned decode routes inspect and size the complete input
before publication, so invalid JPH input cannot partially change output. The
deterministic writer emits one codestream and preserves the raw
`encode_htj2k` payload byte-for-byte.

The one-level matrix is one tile, one layer, LRCP, default precincts, 64 × 64
code-blocks, HT-only cleanup coding, zero origins, no component subsampling and
no multiple-component transform. It accepts greyscale or RGB, planar or
interleaved input, and unsigned `U8` or 16-bit `U16_LE` storage. The RGB route
transforms each native component independently, so component decode returns the
original native red, green and blue samples without an inverse colour
transform. The one-level `U16_LE` route admits the complete unsigned native
range. A reversible 5/3 level can produce 17-bit transformed magnitudes: the
two polarities of the 2 × 2 full-range checkerboard produce HH coefficients of
−131070 and 131070 and signal a QCD exponent of 17.

The project-authored direct scalar cleanup materialiser holds per-quad MagSgn
values and masks in `u32`. A 17-bit transformed magnitude can use up to 18
explicit bits once the interleaved sign representation is included. Potential
16- or 17-bit subbands bypass the full-octet 16-bit MagSgn materialiser; lower
depths retain the prepared full-octet and acceleration routes. This fallback is
lossless but may be slower for high-dynamic-range blocks. The full-octet VLC
caller also retains the general initial-nibble reader until a physical VLC
nibble has been loaded, then resumes the existing steady reader. Empty high
subbands in very small one-level images are represented without included code
blocks. Byte-aligned cleanup termination treats an empty partial MEL byte
explicitly rather than shifting an eight-bit value by eight.

The marker and transform route follows ISO/IEC 15444-15:2019, clauses
6.1–6.3 and normative Annex A, A.1 and A.4, PDF pages 10–11 and 35–38, and
ISO/IEC 15444-1:2024, Annex A, A.6.1–A.6.2, Annex B, B.5 and B.8, and Annex F,
F.2.2, PDF pages 46–51, 85–86, 88–89 and 132. The Part 15 retrieval revision is
`10baf9472429d52f5d6b5f9b7a892dbed395b1db` (reviewed bundle
`e7d1936131227fae2d3f8315309de4dedc83eb3f`); the Part 1 retrieval revision is
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). Part 15:2019 refers to the 2019
Part 1 edition, whereas the locally reviewed normative core is Part 1:2024.
This bounded profile relies only on the established COD decomposition count,
reversible-transform signalling, resolution/sub-band organisation and
one-layer packet model; it does not rely on a post-2019 Part 1 feature or use
the later edition to reinterpret Part 15.

The JPH container calibration additionally follows ISO/IEC 15444-15:2019,
normative Annex D, PDF pages 62–64, at retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`, together with its mapped inherited
ISO/IEC 15444-1:2024 Annex I structure, PDF pages 160–181, at retrieval
revision `34e5d1639b9f121807e620c001893ca9d2c8f977`. The implementation and
synthetic tests are project-authored and reproduce no protected standards
text, table or payload.

The decomposition orchestration, transform use, packet construction, scalar
fallback, termination repair, JPH composition, tests and this description are project-authored
from those standards routes. The closed OpenJPH-derived file set, pinned source
revision, headers, attribution, tables and provenance inputs are unchanged.
The independently designed termination repair is contained in the already
allowlisted cleanup encoder file under the approved repository authority; no
external implementation source, test, constant, table, payload or diagnostic
informed it.

## Common-grid and JP2 default-image geometry

The codestream crate provides semantically neutral SIZ common-grid arithmetic.
It derives one scalar spacing as the greatest common divisor of the combined
set of every non-zero horizontal and vertical component separation, then maps
absolute, non-empty reference-grid bounds through checked ceiling division by
that same spacing on both axes. The plan retains the absolute common-grid
origin, dimensions, spacing and each component's checked native-grid bounds.
It does not assign presentation meaning or resample samples.

The core crate selects that neutral arithmetic as JP2 default-image geometry
only after the container boundary has identified a JP2 input. Raw J2K remains
a codestream reconstruction input and does not acquire JP2 presentation
semantics. JPH and HTJ2K admission are unchanged.

Public native component output remains unchanged. Unequal-grid rendered decode
continues to fail closed except for the bounded full-frame sYCC projection
below. Direct high-precision greyscale rendering has a separate unit-grid
boundary. Existing resolution reductions continue to describe native component
reconstruction; no rendered-reduction interpretation is selected by
common-grid or JP2 default-image planning.

The neutral SIZ arithmetic follows the image, component, registration and
reduced-grid navigation in ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.9.1,
and Annex B, B.1–B.5. JP2 default-image selection follows Annex I,
I.5.3.1.1. The reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The description and deterministic
tests are project-authored and do not reproduce standards prose, equations or
tables.

## Direct high-precision JP2 greyscale projection

The core admits one direct unsigned 9–16-bit JP2 greyscale rendered profile.
The first and only contiguous codestream has one unit-sampled component, zero
image and tile origins, one tile, reversible 5/3 coding, no decomposition and
no multiple-component transform or CRG. The JP2 header has exactly one colour
specification, whose first method is enumerated greyscale; image-header and SIZ
precision and signedness agree. Palette, component mapping, channel definition,
ICC interpretation and additional colour specifications are absent.

Rendered output reuses the native reconstructed plane unchanged. Each code
value occupies one little-endian `u16` word; `SampleFormat.bits_per_sample`
retains the declared precision, `signed` is false, the colour model is
greyscale, and rendered component descriptors do not claim a source component.
There is no second level shift, scaling, clipping, rounding or narrowing at the
container boundary. Shape discovery, owned planar and interleaved decode, and
caller-owned decode all select the same contract before output mutation.

Signed greyscale, precision above 16 bits, RGB, sYCC or mixed-precision
high-depth images, unequal or registered grids, multiple tiles, non-zero
origins, decomposition, indirect mapping, alpha and ICC remain unsupported.
Raw J2K does not acquire JP2 colour semantics, and JPH remains outside this
Part 1 profile. Native component decode is unchanged.

The precision, component and container boundary follows ISO/IEC 15444-1:2024,
Annex A, A.5.1 and Tables A.9 and A.11, Annex B, B.1–B.2, Annex G,
G.1–G.1.2, and Annex I, I.3.5 and I.5.3.1–I.5.3.3, I.5.3.5–I.5.3.6. The
reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`, source SHA-256
`3b15e13add906b67e6528f13dc69dde999abc9c0d089afa7eccea7075806f5a1`). The
description and synthetic fixtures are project-authored.

## Bounded full-frame and partial sYCC projection

The core admits one direct, unsigned 8-bit JP2 sYCC projection. The first
component is Y on a 1 × 1 SIZ grid and the second and third components are Cb
and Cr on matching 2 × 2 grids. The input must contain exactly one contiguous
codestream and one enumerated sYCC colour specification, no multiple-component
transform, and no palette, component mapping or channel definition. This also
excludes alpha, ICC interpretation and additional colour specifications. The
native codestream must satisfy the existing single-tile subsampled-component
decoder gate, including zero image and tile origins. CRG may be absent or may
contain exactly one all-zero registration pair for each of the three
components. Every nearby profile remains unsupported, including signed or
higher-precision samples, different sampling, non-zero registration, multiple
codestreams and indirect component mappings.

With zero registration, chroma sample `(x / 2, y / 2)` is selected for output
sample `(x, y)`, using integer division. This is a nearest-neighbour,
zero-order hold over each 2 × 2 block; the last chroma sample is extended to an
odd right or bottom edge. The selected Y, Cb and Cr values are converted using
binary64 arithmetic and the sYCC coefficients `1.402`, `0.34413`, `0.71414`
and `1.772`, with a chroma centre of 128. Each result is rounded to the nearest
integer, with halfway cases away from zero, then clipped to `[0, 255]`. Output
is three unsigned 8-bit sRGB channels at the full SIZ image width and height,
stored in the caller-selected planar or interleaved layout.

The additive `decode_rendered_partial`, `decode_rendered_partial_info` and
`decode_rendered_partial_into` APIs reuse `PartialDecodeOptions` without
changing the native partial API. They admit a non-empty, full-resolution,
image-relative rectangle wholly inside this same direct JP2 profile. The
request must select all channels, no tile and no quality-layer limit. Partial
decode additionally requires reversible 5/3 coding and the existing direct
selective codestream profile; it never selects compatibility or
full-image-decode-and-crop fallback.

For a requested half-open rectangle, the selective source rectangle starts at
the preceding even x and y coordinates and ends at the requested right and
bottom edges. This retains the co-sited chroma needed by odd starts. The
prepared codestream plan reconstructs only that source rectangle into owned
native staging, then the shared projector selects chroma from the absolute
output coordinate and crops surplus luma. Caller-owned publication occurs only
after reconstruction and projection both succeed. Rendered descriptors have
no source-component identity; their origin is the absolute requested x and y,
their separation is one, and their dimensions are the requested dimensions.
The ordinary reversible full-frame sYCC renderer uses this same checked plan
and projector while retaining its existing acceptance of inert top-level
metadata. The partial API applies the stricter direct-metadata boundary above.
The existing irreversible full-frame renderer remains available, but partial
9/7 requests fail closed.

| Boundary | Supported now | Deferred |
| --- | --- | --- |
| Container and colour | One JP2, one Part 1 codestream, one enumerated sYCC specification, direct Y/Cb/Cr | raw rendered output, JPH/HT, indirect mapping, ICC and alpha |
| Samples and grids | unsigned u8, Y 1×1, Cb/Cr 2×2, zero image/tile origins, absent or all-zero CRG | high-depth or direct greyscale partial, other sampling, non-zero origins or CRG |
| Coding and selection | one tile, reversible 5/3, full resolution, all channels, no quality limit | multiple tiles, reductions, quality-layer limits and partial 9/7 |
| Output | planar or RGB-interleaved u8, checked owned staging before caller publication | compatibility fallback and raw component presentation |

Native component decode, including the existing native partial APIs, continues
to return native component grids without colour conversion or resampling. Raw
J2K rendered decode continues to fail closed. The project authority is
ISO/IEC 15444-1:2024, clauses A.5.1, A.9.1, B.1–B.3, I.5.3.1.1,
I.5.3.3 Table I.10 and J.14, reviewed at retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` in bundle
`1a7a03799078b476bf38e91786b979059b4c533d`. The rendered comparison authority
is ISO/IEC 15444-4:2024, Annex G, reviewed at retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7` in bundle
`7b3d8d60cd4d4f6c056cd108d928b7f99f492aa9`. This description, the
nearest-neighbour hold policy and the synthetic fixtures are project-authored;
no ISO expression is reproduced.

## Baseline JP2 header admission

JP2 parsing admits one structural header before the first contiguous
codestream. The image header is first and singular, colour specification boxes
form one non-empty contiguous sequence, and bits-per-component metadata is
present exactly when component precision or signedness varies. Legal field
ranges and component cardinality are checked at the container boundary.

The core then compares image width, height, component count, precision and
signedness with SIZ in the first contiguous codestream. A mismatch is invalid
metadata, not a request to prefer one source, and is rejected before decode can
write caller-owned output. Unknown boxes retain the existing byte-preserving
behaviour unless they interrupt a required sequence.

This admission establishes container and codestream consistency only. It does
not produce presentation pixels or select palette application, component
mapping, channel or alpha interpretation, ICC transforms, colour conversion,
resampling, resolution handling or multiple-codestream composition. The known
unimplemented presentation metadata is reported by support classification
without making an otherwise well-formed container invalid. Component-mode
decode may still expose admitted raw codestream planes and applies no
presentation transform. Component output metadata is inferred from the raw
selection: all one- and three-component outputs are labelled grayscale and RGB
respectively, while other counts and explicit subsets remain unknown. JP2
colour metadata does not alter that inference. Except for the bounded direct
greyscale and sYCC profiles above, rendered requests fail closed before output
allocation or mutation for high-depth or signed samples, palette, component
mapping, channel definition, sYCC, ICC, vendor, reserved or unrecognised colour
metadata.
Partial decode requests are native component selections; their Part 1
full-decode compatibility fallback therefore also uses component mode.

The authority is ISO/IEC 15444-1:2024, Annex A, A.5.1, PDF pages 41–44, and
Annex I, I.2.2, I.5.3–I.5.4, PDF pages 160–171 and 181. The reviewed retrieval
revision was `34e5d1639b9f121807e620c001893ca9d2c8f977`.
