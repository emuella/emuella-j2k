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

This foundation does not add rendered projection, component selection, partial
or reduced requests, or container presentation features. Existing JPH header
consistency and presentation checks remain in force. The independent DS0
reduced-component profiles, HTMIX dispositions and existing lossless full-image
classifier are unchanged.

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
