# Decoder profile contracts

This document records the exact selected Part 1 and HTJ2K inspection and native
decode requests exposed by the public API. Start with the
[supported-profile overview](supported-profiles.md) to choose an operation;
inspection, packet validity, decode admission and a particular output request
are separate claims. These bounds describe implemented routes, not general
JPEG 2000 or Part 15 conformance. Unless a section says otherwise, nearby
formats, requests and output modes do not inherit its support.

## Classic Part 1 regional output with reversible MCT

The positioned-source regional route also admits reversible MCT within the
classic Part 1 envelope: three matching unsigned 8–16-bit
unit-sampled components, reversible 5/3, default-precinct LRCP packets with optional EPH and no SOP, and
exactly one `TPsot=0` part per SIZ tile. `TNsot` may declare one part or leave
the count unspecified; complete sequence validation must still prove the
single part and reconcile SOT, any TLM, `Psot` and terminal EOC. Regional
reconstruction retains all three RCT dependencies but publishes only requested
native RGB components. Other tile-part counts, interleaving, reductions, layer
limits and MCT shapes remain unsupported.

See also the independent [native-plane publication contract](native-planes.md),
[JP2 mapped presentation](jp2-presentation.md), and the mechanism description
in [architecture](architecture.md#classic-reversible-mct-regional-output).

## HTJ2K decoding and support admission

Native HTJ2K decode has a separate staged HTONLY boundary. Structural Part 15
parsing and packet-signalling validity run before support admission. A broader
`Ccap^15` permission does not by itself make a codestream unsupported when the
effective codestream still uses the implemented single-set, ROI-free,
homogeneous, reversible HT path. Actual multiple HT sets, ROI, heterogeneous
state, HTMIX and cleanup magnitude bounds above 18 remain unsupported in
that full-image path. A separate [irreversible HT foundation](ht-lossy-foundations.md)
admits the selected two-level, no-MCT, unsigned grey/RGB U8/U16_LE profile
for full native component output from raw HT and JPH, including the additive
lossy encoder outputs. Use `DecodeMode::Components`, all components, full
resolution and either output layout; the default rendered mode is excluded.
Other irreversible full-image profiles remain unsupported, as does rendered
projection of the new irreversible profile. Supported full inputs continue
through the existing HT packet, entropy, reconstruction and public image path;
this is not general Part 15 or JPEG 2000 conformance.

[HTMIX is deliberately unsupported](htmix-disposition.md), independently
of MULTIHT permission and the HTONLY envelopes below. Legal mixed signalling
remains inspectable; packet-dependent mixed classic/HT block interpretation
does not fall back to either homogeneous decoder. Locked HTMIX points are not
applicable to the HTONLY qualification claim, not decoded-pixel passes.

The [bounded DS0 qualification summary](testing.md#bounded-ds0-result)
records all sixteen selected HTONLY points and their distinct output routes.
Those points do not imply general full-image, rendered or JPH decode support.
Structural inspection also does not prove complete packet validity: the
bounded SINGLEHT validator skips every CAP Mixed declaration, including
homogeneous-effective HT neighbours.

## JPH inspection

JPH inspection enforces the bounded Annex D signature, `jph ` file type and
`jph ` compatibility membership,
inherited `jp2h` structure including optional-box dependencies, complete HTJ2K
`jp2c`, and first-codestream header-consistency boundary before decode
admission. The JPH unknown-colour/no-`colr` form is structurally accepted but
remains unsupported for rendered colour interpretation. Unknown legal boxes
are preserved, while optional presentation, alpha, multiple-codestream
composition, HTMIX and codec profiles outside the documented subset remain
unsupported.

## Full native component grids

### Subsampled full native grids

A separate native-grid full decode route admits one unsigned 8-bit subsampled
component, one tile/part, three effective reversible 5/3 levels, one through
six LRCP layers and one effective precinct per resolution. Main COC/QCC
overrides resolve before reconstruction; component origins must be aligned to
eight native samples. Use component mode, planar output and all components or
component zero. `ImageInfo` and `decode_shape` retain reference-image dimensions;
`Image::component_info` describes the actual native plane and its origin and
sampling. No resampling is performed. JPH, rendered/interleaved output, MCT,
other transform phases, regions, reductions and layer limits are outside this
route.

The same native-grid preparation also admits three matching unsigned 8-bit
sampled components with reversible MCT across one to 64 tiles, one part per
tile. Each native tile-component origin must be aligned to eight samples;
the three-level, one-to-six-layer LRCP and single-precinct limits above still
apply. Explicitly select component zero in planar component mode to obtain
the transformed codestream component before inverse RCT. All-component output,
other selections, RGB presentation and JPH remain unsupported for this branch.
Native tile bounds are assembled without resampling. The aggregate native
component sample count is limited to 16 Mi samples before packet preparation.

### High-component native output

A separate high-component native route qualifies locked P0.13 HTONLY. It
accepts four through 257 unit-sampled 8–16-bit components in one zero-origin
tile/part of at most 64×64 samples. Every effective main COD/COC has one
reversible 5/3 level, one RLCP layer, MCT, 32/64-sample block axes and explicit
128×128/256×256 precincts without SOP/EPH. Main POC partitions all components
into two adjacent complete volumes, first RLCP then CPRL; resolution bounds
are clipped to actual resolutions. Main QCD/QCC resolves reversible quantisers
for every component, with positive exponents and at most 30 ROI-extended
magnitude bits. One main Maxshift assignment of 1–15 must name an unselected
component. The first three MCT component formats match; later native formats
may differ. Every packet is validated, but only component zero is reconstructed
before inverse RCT. Use full `decode` or `decode_htj2k_with_workspace` with
explicit planar component zero in component mode; `decode_shape` shares the
admission. All-component support inspection remains unsupported. Other
selections, partial requests, layer limits, tile overrides, packet relocation,
HTMIX and JPH/rendered output remain outside this route.

## Native reduced and regional requests

### Maxshift native window

A separate raw native ROI window route admits one unit-sampled component with
1–16-bit signed or unsigned precision, zero origins, one reversible 5/3 level,
32/64-sample block axes, 128×128/256×256 precincts and one through eight layers.
Main POC must resolve the PCRL COD to one complete LRCP volume. One tile-zero,
component-zero Maxshift assignment of 1–15 is restored before synthesis;
effective QCD/QCC must be reversible and the ROI-extended magnitude width must
fit 30 bits. Tiles have 32/64/128-sample axes, at most 64 tiles, and one payload
part followed by at most three empty parts. TLM and informational CRG are
validated; inline SOP/EPH is supported. Explicitly select planar component zero
with a full-resolution region inside tile zero. All tiles' packets are checked,
but only tile zero is reconstructed and cropped without resampling. This
qualifies both locked P0.03 and P0.15 full-resolution window alternatives.
Full-image decode, reduced alternatives, other ROI assignments, functional
tile-header overrides, packet relocation, HTMIX and JPH/rendered output remain
outside this route.

### Two-level reduced component output

Native partial component output adds bounded HTONLY reconstruction branches
behind one request shape. A raw origin-aligned single-tile codestream with five
decomposition levels and one-layer LRCP packets on the existing
single-effective-precinct inline-header route may select transformed component
0 at two discarded resolution levels. The reversible 5/3 branch requires three
matching unsigned 8-bit unit-sampled components, MCT and the existing
no-quantisation QCD contract. The irreversible 9/7 branch instead requires
exactly one unsigned 8-bit unit-sampled component, no MCT, exactly one
main-header scalar-expounded QCD and no component or tile overrides. The
planar output is reconstructed at its exact reduced geometry before inverse
colour transformation. JPH, rendered output, other selections or reductions,
regions, tile requests, quality-layer limits, heterogeneous coding or
quantisation, ROI, HTMIX and other irreversible HT shapes remain unsupported by
this route.

### Raw lossy U16 greyscale partial output

The lossy encoder's raw unsigned greyscale `U16_LE` output has an independent
partial component-zero route. A no-region request admits exactly one or two
discarded resolution levels. A spatial request must provide one contained,
non-empty, image-relative half-open full-resolution region and may select full,
discard-one or discard-two output. Each half-open endpoint is projected
independently with ceiling division; `decode_partial_component_info` reports
that projected origin and shape. The route validates the complete packet
stream, entropy-decodes selected whole blocks only and reconstructs bounded
coefficient and 9/7 synthesis windows without an application-visible halo,
full decode/crop, resampling or a full-resolution output plane for small
requests. Owned, reusable-workspace and padded caller output are byte-exact;
caller bytes and padding remain unchanged on every failure. JPH, RGB, U8,
signed, rendered or interleaved output, other component selections, tiles,
layer limits and discard above two remain unsupported.

### Irreversible transformed component at reduction three

A separate reduction-three request selects transformed component zero from a
raw zero-origin single tile/part with three matching unsigned 8-bit unit-sampled
components, MCT, six 9/7 levels, twenty RLCP layers, 64×64 HTONLY blocks and
explicit 128×128 precincts at every resolution. Main QCD and optional QCC
resolve to scalar-expounded quantisation for every component. The shared packet
walker validates all layers, components and precincts; reconstruction retains
component zero through resolution three, before inverse ICT. The reference
image is bounded to 16 Mi samples per component before packet preparation.
Only planar component-zero output without a region, tile or layer limit is
admitted. JPH, rendered output, other reductions, COC, tile-header overrides,
ROI, packed/inline packet markers and HTMIX remain outside this branch.

### Heterogeneous reversible component at reduction five

A heterogeneous reversible reduction-five route selects raw component zero
from one zero-origin tile/part with three unit-sampled components. Each may
have its own signedness, 8–16-bit precision, effective main COD/COC coding and
QCD/QCC no-quantisation exponents. Component zero has six decomposition levels;
the others have six through eight. CPRL uses one through thirty layers,
32/64-sample block axes and explicit 128×128 or 256×256 precincts. Inline
SOP/EPH is supported. All packets and quantisers are validated; only component
zero through resolution one is reconstructed, retaining its native precision
and signedness. The 16 Mi-sample reference-plane bound applies before packet
preparation. Only planar component-zero output without region, tile or layer
limits is admitted; MCT, sampling, tile overrides, ROI, HTMIX, JPH and rendered
output remain outside this route. This qualifies the locked P0.08 HTONLY point.

### Sampled scalar-derived component at reduction three

A separate scalar-derived reduction-three route selects raw component zero
from one zero-origin tile/part with four unsigned 8-bit components sampled
1×1, 1×1, 2×2 and 2×2. Effective main COD/COC resolves 6/3/6/6 levels,
9/7 for the first three components and 5/3 for the fourth, no MCT, one through
seven PCRL layers, 32×32 blocks and explicit square 128/256 precincts.
QCD/QCC resolves scalar-derived component zero, scalar-expounded components
one/two and reversible component three; every resolved exponent is positive
and its guard-adjusted magnitude width is at most 30 bits. All components and
packets are validated, but only component zero through resolution three is
reconstructed. The 16 Mi-sample reference-plane limit applies before packet
preparation. This qualifies the locked P0.05 HTONLY point with planar
component-zero output; other selections, reductions, regions, tile/layer
requests, inline packet markers, tile overrides, ROI, HTMIX, JPH and rendered
resampling remain outside this route.

### Heterogeneous reduced ROI component

A separate heterogeneous reduced ROI route qualifies locked P0.06 HTONLY.
It selects planar native component zero at reduction three, from a zero-origin
single tile/part with four independently signed or unsigned 8–16-bit components
sampled 1×1, 2×1, 1×2 and 2×2. Effective main COD/COC has six levels, 9/7
for components zero through two and 5/3 for three, 64×64 HTONLY blocks, one
through four RPCL layers, no MCT or inline SOP/EPH, and explicit square
128/256 precincts. Main QCD/QCC resolves scalar-expounded zero through two
and reversible three, with positive exponents and at most 30 ROI-extended
magnitude bits. Exactly one component-zero main RGN and one tile RGN have
shifts 1–15; the tile overrides the main and its effective shift must be 1–9.
Every component's packets are validated; only zero through resolution three
is reconstructed. ROI magnitudes are restored before dequantisation and 9/7
synthesis, preserving the native precision and signedness. The 16 Mi-sample
reference-plane preflight applies before packet preparation. Full-image decode,
other selections/reductions, region/tile/layer requests, additional tile
overrides, POC, relocation, HTMIX and JPH/rendered output remain unsupported.

### Tile-progression native window

A separate tile-progression native window route qualifies locked P0.07 HTONLY.
It admits three independently signed or unsigned 8–16-bit unit-sampled
components, zero origins, three reversible 5/3 levels, one through eight RLCP
layers, 32/64-sample block axes, explicit 128×128/256×256 precincts and optional
SOP/EPH. Tiles have 32/64/128-sample axes and the grid has at most 256 tiles.
Tile zero has two parts with successive LRCP POC volumes: a prefix of the
resolutions, then a complete volume whose already-seen packets are skipped.
All other tiles have one part and inherit RLCP. Main QCD supplies bounded
reversible quantisation; no component or tile coding/quantisation override,
ROI, MCT, main POC or packet relocation is admitted. The input is bounded to
64 MiB before packet work. Every tile/component packet is validated, using
tile-local header scopes; only component zero of tile zero is reconstructed.
Explicitly select a full-resolution planar component-zero region inside tile
zero. Native precision and signedness are preserved without resampling.
Full-image decode, other requests, HTMIX and JPH/rendered output remain outside
this route. This bounded qualification is not general Part 15 conformance.
