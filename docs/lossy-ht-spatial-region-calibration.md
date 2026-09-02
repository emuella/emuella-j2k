# Lossy HT spatial-region calibration

## Question

Can the existing packet-contribution metadata and transform-owned bounded 9/7
machinery reconstruct exact regions from the raw greyscale U16_LE output of
`encode_htj2k_lossy`, while entropy-decoding only whole intersecting code
blocks and never retaining a complete coefficient or output plane?

The probe started from codec revision
`31a32971531895ad6cda5e4da583d410c6e94220`. It is an internal calibration;
it does not change partial-decode admission or any public API.

The executable calibration checkpoint is
`fb04df168927e689e0a9ea893b4432b28978f0cf`, tree
`750dfea589d6b7e2e5aea19457c357de2f292a0f`. On that exact source, the seven
release-mode `ht_lossy::tests` passed, including the six-cell probe and checked
negative cases. The public-encoder identity test and the existing discard-1/
discard-2 geometry-and-routes test each ran once and passed. Formatting and
diff checks also passed.

## Probe identity

The project-authored input is the deterministic
`ht_lossy_test_support::source(1024, 1024, 16, 1, 1)` U16 greyscale plane.
Its little-endian byte SHA-256 is
`75855d11fddce88d3377bcaeab864905cef5cc724f0059da81271832010c9054`.
The core companion test feeds those planar bytes, a 2048-byte stride and 4.0
bits per pixel through the public `encode_htj2k_lossy` entry point. The
codestream probe uses the same underlying `ht_lossy::encode_planar` operation
and locks the identical raw codestream SHA-256:
`f376f5c04b13c640fec6b80ba52bfb198cc1832d75f6850411ba801e035da597`.

Two full-resolution image-relative half-open regions are used:

- interior: `(137, 211, 43, 35)`;
- bottom-right edge: `(973, 981, 51, 43)`, ending exactly at `(1024, 1024)`.

Both are projected with checked ceiling division at discard 0, 1 and 2. The
oracle is the byte-exact corresponding crop of Emuella's established full or
direct reduced component reconstruction, never another JPEG 2000
implementation or fixture.

## Standards basis

The planning follows ISO/IEC 15444-1:2024 retrieval
`34e5d1639b9f121807e620c001893ca9d2c8f977`: image, tile-component,
resolution and subband geometry in B.1–B.3 (PDF pages 79–82), subband and code-
block partitioning in B.5–B.9 (pages 85–90), packet completeness in E.1–
E.1.1.2 (pages 129–130), and recursive inverse 9/7 synthesis and boundary
extension in F.1–F.3.8.2.1 (pages 132–143). G.1–G.1.2 (page 153) informs the
existing quantisation handling. Figures B.1, B.9, F.8 and F.15 and the inverse
lifting equations were inspected; no standards text is reproduced here.

HT cleanup interpretation remains the existing implementation of ISO/IEC
15444-15:2019 retrieval
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`: clauses 6.1–6.2 (PDF pages
10–11), 7.6 (page 30), A.1–A.3 (page 35) and B.1–B.3 (pages 40–41).

The finite four-sample transform halo is an engineering use of the existing
`plan_window_synthesis` contract. Cropping that conservative reconstruction to
byte equality with established Emuella output is a product claim proved by the
probe, not a claim quoted from the standard.

## Observed result

All six comparisons are byte-exact. Complete parsing retains 256 contribution
records, but only the following whole intersecting contribution rectangles
reach HT decode. Workspace ceilings include compact and synthesis f32 values,
one maximum 64×64 i32 block, the largest selected segment and exact U16 output.

| Region | Discard | Output | Selected blocks | Selected block coefficients | Compact coefficients | Synthesis ceiling samples | Workspace ceiling bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| interior | 0 | 43×35 | 11 | 45,056 | 4,364 | 5,366 | 66,947 |
| edge | 0 | 51×43 | 7 | 28,672 | 3,632 | 7,158 | 75,338 |
| interior | 1 | 21×17 | 8 | 32,768 | 1,292 | 1,504 | 31,866 |
| edge | 1 | 25×21 | 4 | 16,384 | 1,020 | 1,868 | 33,219 |
| interior | 2 | 10×9 | 1 | 4,096 | 90 | 182 | 18,666 |
| edge | 2 | 12×10 | 1 | 4,096 | 120 | 242 | 19,205 |

The test locks these figures, confirms every selected block intersects its
required subband rectangle, and confirms the legacy complete reduced
coefficient plane and transform scratch retain zero capacity. No compact
coefficient band, synthesis workspace or output reaches 1,048,576 samples.

## Decision and limitations

**RETAIN.** Existing packet metadata, the current HT doubled-half-step
normalisation/dequantisation, and transform-owned bounded 9/7 synthesis are
sufficient for the selected encoder envelope without material redesign.

This result covers one deterministic input, rate, interior region and edge
region. It is not the public admission matrix and does not establish caller-
buffer atomicity, reusable region workspace behaviour, malformed entropy
failure handling, other rates or patterns, one-pixel and strip geometry, full-
image requests, or any excluded container, format, colour, tile or profile.

The retained seam now has a production-private executor for zero, one or two
discarded resolution levels. The discard-two increment started from codec
revision `c7615fa5fcefb9dda5af25a999d4252a8f788fc1`. At both discard 1 and 2,
eighteen full-resolution half-open regions with valid projections use checked
ceiling geometry and match exact crops of the established direct reduced
reconstruction. The matrix covers a full odd non-power-of-two image, all
corners and edges, retained one-pixel and strip results, odd origins and
dimensions, 64-sample block boundaries and crossings, and transform
boundaries. Three representative successful pattern/rate pairs at each discard
also match byte-for-byte.

The unchanged 1024×1024 calibration codestream hash above additionally locks
the following discard-two full and region-dependent results. Both retain
resolution zero, the exact required lowest LL rectangle and whole intersecting
blocks, with no 9/7 synthesis levels.

| Full-resolution request | Projected output | Selected blocks | Compact LL samples | Synthesis ceiling samples | Workspace ceiling bytes |
|---|---:|---:|---:|---:|---:|
| 1024×1024 full image | 256×256 | 16/256 | 65,536 | 131,074 | 1,196,728 |
| `(256, 256, 256, 256)` | 64×64 | 1/256 | 4,096 | 8,194 | 90,773 |

One workspace successfully alternates full-resolution, discard-1 and
discard-2 requests while regions shrink, grow and relocate. Tests mechanically
retain only the required LL/HL/LH/HH windows and whole selected blocks,
entropy-decode only those selected blocks, enforce the checked deterministic
workspace ceiling before reconstruction, and keep the legacy complete reduced
coefficient plane and transform scratch at zero capacity. Empty requests and
empty discard-two projections, out-of-bounds and overflowing geometry, discard
above two, truncated packets, corrupted selected HT payload, malformed selected
contribution metadata and a one-byte-short workspace limit all fail without
returning output.

The discard-two release-mode codec run passed 15/15 focused
`ht_lossy::tests`, including the unchanged six-cell hashes and accounting above
and the new two-cell large-geometry accounting. The release-mode core lossy
matrix passed 8/8, including public partial-request closure and both established
reduced routes; the irreversible odd-plane transform test passed 1/1. Release
clippy for all codestream targets, formatting and diff checks passed. Public
partial-decode admission remains unchanged; this is not a public support claim.

The next safe action is public admission, caller-buffer qualification and
publication of the already calibrated full/discard-1/discard-2 executor.
