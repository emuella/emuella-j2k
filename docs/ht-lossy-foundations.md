# Irreversible HT internal foundation

The implementation-facing `emuella-j2k-codestream::ht_lossy::encode_planar`
boundary encodes the calibrated two-level irreversible HT profile. It accepts
one or three unsigned planar byte views, matching U8 or U16_LE precision,
explicit byte strides and a finite positive `f32` rate. The additive
[public API](ht-lossy-public-api.md) now exposes this same boundary through
`Htj2kLossyEncodeOptions`, `encode_htj2k_lossy` and `encode_htj2k_lossy_jph`.
`Htj2kEncodeOptions`, `encode_htj2k` and `encode_htj2k_jph` retain their existing
lossless contracts.
No public scalar-step option or alternative fixed-step encoder is introduced.

## Encoding and resources

The [calibration record](ht-lossy-calibration.md) owns standards citations,
selected signalling, historical source identities and measured rate/distortion
acceptance. The production encoder uses that scalar family unchanged: two 9/7
levels, no MCT, one tile/part/layer, LRCP, 64×64 blocks, default precincts,
zero origins and matching unit component grids. It writes scalar-expounded QCD
with three guard bits and seven steps, `Rsiz=0x4000`, `Pcap=0x00020000`,
`Ccap^15=0x002a`, and HT code-block style `0x40`.

Rate counts complete raw-codestream bits per reference-grid pixel, excluding
JPH overhead. The exact input `f32` is widened to `f64`, multiplied by checked
pixel count, floored to whole bits, then floored to whole bytes. Invalid,
unrepresentable or zero budgets fail explicitly. Search visits the coarsest
candidate and at most sixteen binary-search positions. It retains the largest
sampled complete stream within budget, with the first encounter winning a tie.
Success undershoots by at most `max(32 bytes, ceil(budget / 500))`. Otherwise the
request fails as unattainable by this bounded search. It never pads, truncates
or returns an oversized trial. The finite search is not exhaustive and does
not promise universal rate/distortion behaviour.

Each axis is 4–8192 samples, with at most 1048576 reference pixels and 3145728
component samples. Plane strides and final row extents are checked before
analysis. Image-sized analysis, quantisation, packet and output storage uses
fallible reservation; quantisation storage and transform scratch are reused.
Only the current complete trial and best stream are retained during search.
Generated and admitted raw codestreams are limited to 32 MiB. QCD-derived
magnitude widths must be at most 30, and quantised magnitudes must be below
131072. Ineligible candidates skip entropy coding. Existing bounded per-block
entropy scratch remains owned by the HT block implementation; this contract
does not promise recovery from every process allocator failure or a fixed RSS.

## Native decoding

Ordinary `inspect`, `decode_shape`, `decode`, `decode_htj2k_with_workspace` and
full `decode_into` agree on this new native profile for raw HT and JPH. Select
`DecodeMode::Components`, all components, and either planar or interleaved
output. Width, height, precision and component descriptors describe the full
native image. Owned and caller output round ties to even and clip to the native
unsigned range through the shared irreversible reconstruction helper.

Admission requires the exact encoding envelope and equal absolute scalar step
across the seven orientations, before allocating image samples. Only the main
SIZ/CAP/COD/QCD and single SOT/SOD/EOC sequence is admitted. Component or tile
overrides, optional markers, ROI, MCT, sampling, additional parts/layers,
non-default precincts, other progression, mixed coding, refinement passes,
other precision and larger magnitude widths do not enter this route. The new
shape's resource preflight also precedes the generic SINGLEHT packet validator.
Structural invalidity and unsupported profiles remain distinct from entropy
failures: support and shape inspect packet grammar, but complete entropy
validity is established during decode.

Full caller decode reconstructs all components into owned storage before
publishing any samples. Early target/admission failures and late entropy
failures leave every caller byte unchanged. Successful padded targets preserve
row padding and trailing storage. Workspace reuse after a failed decode does
not publish stale coefficients.

Rendered projection and container presentation remain outside this foundation.
One independent raw partial route selects component zero from the unsigned
greyscale U16 encoder profile at exactly one or two discarded resolution
levels. It reuses the complete envelope and packet admission above, retains
only the required resolutions, and passes the checked reduced geometry to the
existing prepared reduced executor. The reusable HT workspace and private
atomic-publication path are shared; there is no second decoder,
full-resolution decode-and-resample fallback or full-resolution output plane.
RGB, U8, signed, interleaved, JPH, spatial, tile, layer, zero-discard and
discard-above-two requests remain outside this boundary. Existing JPH header
consistency and presentation checks, independent DS0 reduced-component
profiles, HTMIX dispositions and the lossless full-image classifier are
unchanged.

## Reduced reconstruction authority record

The reviewed Part 1 authority is ISO/IEC 15444-1:2024, local file
`15444-1.pdf`, SHA-256
`3b15e13add906b67e6528f13dc69dde999abc9c0d089afa7eccea7075806f5a1`,
248 physical PDF pages, reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`, at retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The bounded implementation review
covered Annex A, A.5.1, physical pages 41–44; A.6.1, pages 46–51; A.6.4–A.6.5,
pages 52–54; Annex B, B.5–B.6, pages 85–87; B.9, pages 89–90; B.12.1.1,
pages 96–98; Annex E, E.1.1.1–E.1.1.2, pages 129–130; Annex F, F.3.1 and
F.3.8.2, pages 132 and 142–143; and Annex G, G.1.2, pages 153–154, and G.3,
pages 154–155. These locations bound image/component geometry, coding and
quantisation state, packet ordering, subband reconstruction and native sample
conversion for the admitted route.

The reviewed HT authority is ISO/IEC 15444-15:2019, local source SHA-256
`b161fa1bbd1adbacbe484ef32dfa74d107468a33f8a5bf441dc38f403e7f092b`,
82 physical PDF pages, reviewed bundle
`e7d1936131227fae2d3f8315309de4dedc83eb3f`, at retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`. The bounded review covered
clauses 6.1–6.2, physical pages 10–11; clause 7, pages 11–31; clauses 8.2–8.3,
page 31; clauses 8.7.1–8.7.3, pages 32–34; Annex A, A.1–A.4, pages 35–38;
and Annex B, B.1–B.3, pages 40–42. These locations bound the HT codestream,
block-coding and signalling requirements used by the existing packet and
entropy machinery.

Part 15:2019 references Part 1:2019. Applying the project's reviewed Part
1:2024 authority to this fixed route is the already documented bounded
engineering inference; it is not a claim that the editions are clause-for-
clause equivalent. Part 1 Annex G does not mandate Emuella's final ties-to-even
rounding and clipping. Those operations remain the established, directly
tested product convention. This record and the implementation are
project-authored; they reproduce no standards equations, tables, figures or
prose.

## Evidence boundary

Ordinary tests cover all four sample families at 257×193 and 1/2/4 bpp, using
the authored calibration texture. They check deterministic padded planar
encoding, raw/JPH payload identity, support/shape/owned/workspace agreement,
both output layouts, padded caller output, selected NMSE ceilings and
non-increasing distortion. Separate tests exercise rate, stride, precision,
marker, quantisation, geometry and resource neighbours, and final-component
entropy failure without caller mutation.

The historical optional probe now shares production analysis, candidate search
and ordinary full-image reconstruction; it no longer contains parallel encoder
or test-only decoder admission implementations. It intentionally records even
unsuccessful complete rate trials for historical comparison. Such observation
trials are not successful results of `encode_planar`. The immutable
CSV-formatted text remains unchanged. The subsequent [public qualification](ht-lossy-public-api.md) covers
the complete input-layout matrix. Independent and locked-corpus qualification
from the final merged source remain separate delivery gates; this foundation
alone makes no final product or general conformance claim.
