# Genuine HT quality calibration

Status: measured calibration; retain one cleanup set and one quality layer with
resolution progression for the first composed local HTTP proof. No production
profile extension or public API change is needed.
The exact implementation baseline is
`7923acd621f3f3687206f5e3721d15755e886e1b`.

The question is whether a small number of actual HT precision endpoints improves
first useful native-resolution detail enough to justify additional stored bytes
and repeated reconstruction for a workload containing many mostly unviewed
images on fast storage. The smallest probe uses a 256×256 tile, five 9/7 levels,
64×64 blocks, unsigned native U11/U16 greyscale/RGB, no MCT, default precincts,
and one fixed scalar-expounded unit quantiser. The codec owns the measurements.
Exit requires actual compressed-prefix decodes, a layered Part 1 reference,
byte/work/error measurements, opened visual evidence and the composed workload's
transport observation. Those observations now justify the selected local profile.

## Semantics and authority

ISO/IEC 15444-15:2019 / ITU-T T.814 (06/2019), published extension,
B.1–B.3, physical PDF pages 40–41, and 7.1.1, page 11, are the authority for
this experiment. A single HT set permits cleanup followed by SigProp and
MagRef; these are a small number of coding-pass endpoints, not Part 1's dense
sequence of separately refinable magnitude planes. Multiple sets allow further
precision choices, but each non-empty cleanup carries a fresh block description.
The experiment retains those earlier bytes in the same block's packet prefix.
It does not encode complete independent images and call them progressive.

The probe uses genuine cleanup-only sets. Empty refinement segments do not
cause the decoder to perform a refinement pass. Initial placeholders contribute
no payload; each preceding set still advances the effective skipped-plane count.
Empty sets can bridge a precision gap without adding entropy bytes. Later
non-empty cleanup segments are at least two bytes. The first non-empty set
occurs alone within its first packet contribution; subsequent packets advance
the pass count through explicitly empty refinement and cleanup contributions
before carrying the next selected cleanup. The parser derives the
selected set's `S_blk` from the original packet missing-plane value and the set
position, including gaps. `Ccap^15` bit 13 permits MULTIHT; A.3.3/Table A.2,
physical PDF page 36, is the signalling authority. All these Part 15 decisions
use reviewed retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`.

ISO/IEC 15444-1:2024 / ITU-T T.800 (07/2024), published normative core,
B.9–B.10, physical PDF pages 89–91, and B.10.7–B.10.8, pages 93–94,
provide the packet, inclusion, continuation-length and segment-boundary rules.
Reviewed retrieval revision is
`34e5d1639b9f121807e620c001893ca9d2c8f977`.
The existing transform, quantisation and reconstruction contracts are recorded
in [the irreversible foundation](ht-lossy-foundations.md). Applying the locally
reviewed Part 1:2024 to the Part 15:2019 reference remains that foundation's
bounded engineering inference. No newer Part 9 text or conformance claim is
used here. Transport can schedule available bytes; it cannot create missing
coefficient precision in a single-cleanup representation.

The implementation and explanation are project-authored, reusing the existing
repository coders and their declared provenance. No new external implementation
source, restricted images, standards prose, equations, tables or examples are
copied into these artefacts.

## Probe and measurement contract

[The runnable probe](../crates/emuella-j2k-codestream/src/ht_quality_calibration.rs)
defines each source sample deterministically:
sloping background, bounded texture, a step edge, a faint 60×60 patch, a 3×3
object and thin lines. U16 includes non-zero lower five bits rather than merely
rescaling U11. All input and reconstruction metrics use the native integers.
Input identity is SHA-256 of component-major little-endian words, without a
container. The CSV records all four identities. No external input is used.

Three-set HT selects cleanup low-bit cutoffs 4/2/0 for U11 and 9/7/0 for U16.
The two-set candidate retains the first and final endpoints. The larger native
U16 cutoff makes the early visible precision comparable while preserving the
native unit-step final endpoint. The gap contains empty sets. Cleanup and empty refinement segments use separate
packet contributions, so useful U11 quality endpoints fall at packet layers
1/5/9 and useful U16 endpoints at 1/5/19. The two-endpoint candidates stop at
1/9 and 1/19 respectively. These extra packet layers have zero entropy payload
and are not labelled additional quality. Every selected prefix retains earlier
cleanup bytes. At decode, only the latest non-empty set for
each included block reaches entropy coding; older sets remain a storage and
progressive-delivery cost. The experiment exercises no SigProp/MagRef encoder;
its actual quality stages are multiple cleanup sets with empty refinements.

Part 1 uses precisely the same transformed and quantised coefficients. Its
three layers select matching low-bit cutoffs from one continuous MQ codeword
per block. Conservative prefix lengths include required arithmetic lookahead;
each selected truncated prefix is separately decoded and compared with the
same pass count decoded from the complete codeword. The ordinary regression
then requires native image equality with the corresponding HT quality stage
for all four sample families. Thus the comparison measures actual layers and
coding passes, rather than labels or reference-only coefficient truncation.

The resolution comparison contains one final cleanup set, delivered through
quarter, half and full linear resolution. A local decode envelope reuses the
unchanged delivered packet prefix and synthesises empty headers for omitted
higher resolutions. It changes only envelope bookkeeping, never entropy
payload. The regression requires reduced output equality with the complete
single-set source. Delivered-byte counts exclude those locally generated empty
headers. Quality-prefix envelopes likewise only adjust COD layer count and
SOT/EOC bookkeeping around the original packet prefix. This is codec prefix
qualification, not a second JPIP wire implementation.

The CSV distinguishes useful stage number from actual packet layer count.
Stored bytes include the whole representation; delivered bytes are cumulative
and include the bounded main/tile header allowance. Entropy bytes count only
segments actually decoded at the selected endpoint. Coefficient and pass counts
are actual dispatch work. Synthesis samples sum the image-grid areas processed
at each inverse level. These are logical work counts, not allocation or RSS
measurements. Decode time includes packet parsing, entropy, coefficient
placement, synthesis and native output conversion. Entropy time includes block
allocation and coefficient placement. Each timing row is the observation at
the median total decode time of seven consecutive in-memory runs on this host;
its component times belong to that same run. Encoding time is not measured.

RMSE and peak compare with the original native samples. Object and faint-patch
RMSE cover the exact authored regions. Reduced-resolution error first uses
nearest-neighbour display at original size; this describes displayed preview
loss, not an error claim about the native reduced grid. Error metrics and
reduced native equality are separate checks. Final unit quantisation is lossy;
none of these rows promises lossless image reconstruction.

## Observations and disposition

Measured on 2026-09-07, AMD Ryzen 9 9950X3D, Linux, Rust 1.97.1,
optimised default-feature build. [Complete measurements](ht-quality-calibration.csv)
include every stage's bytes, work, native error, timing and input identity.

| Input | Single-set stored bytes | Two-set stored bytes | Premium | Coarse full-resolution delivered bytes | Half-resolution delivered bytes |
|---|---:|---:|---:|---:|---:|
| U11 grey | 57287 | 75652 | 32.06% | 18242 | 12523 |
| U11 RGB | 171454 | 226711 | 32.23% | 54650 | 37518 |
| U16 grey | 100154 | 118901 | 18.72% | 18230 | 23442 |
| U16 RGB | 300225 | 356644 | 18.79% | 54630 | 70330 |

U11 grey illustrates the distinction. Three-set quality delivers
18242 → 56668 → 113924 bytes with image RMSE 12.48 → 3.20 → 0.75 and
object RMSE 28.85 → 4.69 → 0.47. Part 1 delivers
16554 → 35810 → 53863 bytes and produces identical stage pixels. Its decoded
pass totals are 187/331/481, compared with HT's 22/25/25 latest-set cleanup
passes. Its entropy work is substantially slower on this authored input.
The two-set HT candidate delivers 18242 → 75652 bytes, omitting the middle
stage. The single-set quarter/half/full journey delivers
2830 → 12523 → 57287 bytes. Quarter and half previews have object RMSE
123.01 and 100.23; their synthesis work is 5376 and 21760 samples, compared
with 87296 for every full-resolution quality endpoint.

Opened U11 contact sheets show the tiny object loses its shape in quarter and
half previews; the coarse HT endpoint retains its small footprint and the later
endpoints improve it. Thin lines and the step edge also sharpen with native
resolution. Faint-patch RMSE is 9.17 at coarse HT, exceeding the authored
8-unit patch contrast, and falls to 3.42/0.69 at the next/final endpoints.
Early full-resolution quality is a preview, not sufficient evidence for every
faint feature. The source's texture can mask low-contrast features. Visual
inspection used both full native-range display and one common linear window,
29.3%–53.7% of the native range, plus an eightfold nearest-neighbour object crop.
No metric or transport claim is inferred solely from the contact sheet.

Reject three cleanup endpoints for the current candidate: stored redundancy is
large, and the U11 middle endpoint's cumulative size is almost the single-set
final size. Retain two endpoints as the smallest demonstrated genuine quality
alternative for a bandwidth-limited workload, outside this first local proof. Its approximately 19–32% storage premium applies
to every prepared image; proceeding from coarse to final also performs full
synthesis twice. Resolution previews preserve much smaller work and storage,
with a measured early small-object penalty.

Retain one cleanup set and one quality layer with resolution progression for the
primary fast-storage, mostly-unviewed-image proof. The composition owner measured
one 512×512 final-detail ROI across four tiles in a 43008×43008 U11 representation.
Five independent processes each issued five requests. Cold compressed delivery
was 253143 bytes; descriptor, HTTP, planning and cache readiness together took
0.901–1.497 ms, while decode took 22.039–30.524 ms. Reusing compressed bytes
required zero network bytes and still took 21.671–28.477 ms to decode.
These are supplied composition-owner observations, not measurements performed
by this codec probe. Their exact source identities are codec
`7923acd621f3f3687206f5e3721d15755e886e1b` and instrumented service
`9e5c94c00c3a478a353add0426c995912040ffc9`. The Polyorama composition owner
owns the detailed input identities and reports in
`docs/emuella-viewer-calibration.md`.

For this measured local path, at most 1.5 ms of final-detail delivery does not
justify storing approximately 19–32% more bytes for every image and adding another
full-resolution synthesis stage. The measured tiny-object preview penalty is
accepted with resolution fallback followed by the final native detail. This is
a local HTTP profile decision, not a claim about remote links. The two-set
first-stage saving relative to single-set final delivery is 39045–245595 bytes
across the four synthetic inputs. As an explicitly idealised transfer
calculation, that is 0.31–1.96 ms at 1 Gbit/s or 31–196 ms at 10 Mbit/s,
excluding transport and CPU overheads. Those transfer calculations are not
measured network results. Slow links can therefore justify reopening the
quality alternative using the retained genuine two-set probe.

This is one small synthetic tile with fixed unit quantisation. It is not a
corpus rate-distortion study, an arbitrary-image rate policy, a benchmark of the
selected 2 bpp tiled encoder, measured HTTP latency, browser execution, or
interoperability with another codec. The actual quality-stage experiment and the first local-proof profile decision
are complete; remote-workload profile selection remains separate.

An initial provisional packet writer aggregated intervening empty segments into
the next cleanup length. Reject that grammar: segment boundaries must remain
explicit after the first real cleanup; initial placeholders are a different
case. The current probe separates those contributions and remeasures every row.
The rejected probe source SHA-256 was
`0897cc1ef937e559812d2a2ec9ed400fbdab977e25294b387940513a6ff6340b`
on the same baseline; its measurements have been superseded. No production
parser acceptance rule was changed to accommodate either experiment.

## Reproduction and verification

Set `CARGO_TARGET_DIR` to the registered campaign scratch's `codec-quality/target`.
Run the ordinary prefix regression and optional measurement:

```sh
cargo test --release -p emuella-j2k-codestream genuine_ht_quality_prefixes
EMUELLA_HT_QUALITY_OUTPUT=/registered/campaign/codec-quality \
  cargo test --release -p emuella-j2k-codestream genuine_ht_quality_calibration \
  -- --ignored --nocapture
```

The optional directory must already exist. The probe emits only authored PPM
previews there; it prints CSV rows to stdout. The ordinary regression exercises
all four sample families, packet validation, precise HT/Part 1 stage equality,
two/three-set final equality, reduced-prefix equality and truncated envelopes.
Focused regression and codestream all-target Clippy with warnings denied pass.
The coordinator owns the canonical clean-commit gate and independent review.

Opened visual artefact identities (generated scratch, not public image fixtures):

| Artefact | SHA-256 |
|---|---|
| `quality-contact.png` | `f9d9b2352f4cad215c370e99623496a72b62f3a59f10feb03c8087f1eede6a12` |
| `quality-contact-window.png` | `3d9d0198666cc48d5dedc38eece5e2e12c6d33313f0a307b39ed0a565b0d267b` |

Both sheets show quarter resolution, half resolution, coarse HT, middle HT and
final HT from left to right; the lower row crops `(108,132)` through `(134,158)`
and enlarges it eightfold. The upper row uses nearest-neighbour display at
256×256. The windowed sheet applies the common linear window before enlargement.
The coarse and final columns also represent the two-set candidate because the
regression proves those same native reconstructions.

Provisional source binding before the coordinator creates the checkpoint:

| File | SHA-256 |
|---|---|
| `crates/emuella-j2k-codestream/src/ht_quality_calibration.rs` | `d842664034a0a5489decd77a65ec94fd768d15b6a2aa35060817c8fdcd8bf8ae` |
| `crates/emuella-j2k-codestream/src/lib.rs` | `f6066e1c2c0ebfc0983b2aea0a8415d0edd43472fe8321c1494e6cda1815d788` |
| `docs/ht-quality-calibration.csv` | `cfa7cc8d29c5a578efd607ecab1429cc46a5a69a20925711bbd7af8e7477b35e` |
