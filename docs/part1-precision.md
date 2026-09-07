# Part 1 native precision and regional qualification

## Selected change

Lossless greyscale Part 1 `encode` accepts unsigned 9–15-bit precision stored
in little-endian words. The existing unsigned 8-bit and 16-bit routes remain.
The new route supports zero, one or two reversible 5/3 levels; explicit tiles
require two levels and the existing tile geometry validation. Every word is
checked against its declared precision before entropy coding. Level shifting,
SIZ precision and quantisation precision derive from that declaration; this
is not header relabelling. RGB and target-rate encoding at intermediate
precisions remain excluded. JP2 wrapping retains the same precision metadata.

The source-backed reversible MCT region profile now admits three matching
unsigned 8–16-bit unit-sampled components. It retains full-resolution/all-layer
requests, default precincts, LRCP, no SOP and the validated single-part-per-tile
sequence. EPH is optional: the shared packet reader already handles both
forms. Reconstruction retains all three dependencies even when output selects
one plane. The inverse RCT conversion uses `i64` intermediates, adds the
declared level shift, clips to that unsigned range and writes one or two bytes.
Even arbitrary reconstructed `i32` values cannot overflow these `i64` colour
operations. The new greyscale encoder starts with at most 16-bit centred words
and uses the existing checked reversible transform implementation; no transform
or entropy magnitude admission is weakened.

## Authored verification

`crates/emuella-j2k-test-support/tests/part1_precision.rs` generates every input
from an arithmetic formula, including zero, maximum words, low-bit detail and
contrasting channels. It checks greyscale precision 9 through 16 at zero, one
and two levels and two-level 32×32 tiling, plus RGB16 through the same geometry
matrix. Images are 67×53. Full-image regions, a 9×11 tile-crossing window at
(29,27), and a 4×4 edge window at (63,49) agree exactly with that formula.
Padded caller planes and repeated retained-plan execution preserve guards and
native precision. Out-of-range input words fail. The existing six layered MCT
regressions retain EPH, continued MQ segments, malformed sequencing and failure
atomicity coverage.

The C ABI test `satellite_precision_and_rgb16_cross_tile_regions_preserve_native_words`
independently constructs greyscale11 and RGB16 samples, crosses four tiles,
checks reordered RGB output with unaligned padded rows, and repeats with one
workspace. Its output allocation is exactly 198 bytes per selected component.
The ABI continues to prepare each region; workspace reuse does not cache a plan.

Focused commands:

```sh
cargo test --release -p emuella-j2k-test-support --test part1_precision --test reversible_mct_region
cargo test --release -p emuella-j2k-capi satellite_precision_and_rgb16
```

## Authorised in-place satellite observation

The private integration campaign supplied a CORE3D Jacksonville WV3 PAN NITF,
SHA-256 `61c1ba16ff0c7b1788e912cc143ecf966566cd7c092eb14a111e75a185547e4a`.
No source bytes or pixel outputs were copied or retained outside its authorised
store; positioned reads and sample comparisons used memory only. These are
project-authored aggregate observations, not redistribution of the input.

The 822,909,368-byte embedded codestream begins at NITF offset 4192. Header
observations establish 43008×43008, one unsigned 11-bit component, 1024×1024
tiles (1764 total), six parts per tile (10584 total), five irreversible 9/7
levels, 19 RLCP layers, 64×64 classic blocks, code-block style zero, default
precincts, no SOP/EPH and scalar-expounded QCD with two guard bits. TLM indexes
all parts. These findings do not infer rights or claim standards conformance.

At codec baseline `aa7090c23cce62437cefe5b441e971b1bd4320b5`, a release-mode
positioned-source probe successfully reconstructed a 32×32 region at
(20000,20000). Two executions of each of two independently prepared plans were
byte-identical in memory. Output sample range was 160–342. This establishes
repeatability, not independent external-decoder agreement or source losslessness.

| Stage | Logical source bytes | Read operations |
|---|---:|---:|
| Inspection | 731915 | 84687 |
| Each regional preparation | 642821 | 10755 |
| Each execution of a retained plan | 72660 | 222 |

Largest read was 65536 bytes. Each preparation skipped 822207527 tile-part
bytes and 434896 packet-body bytes. Execution selected 25 code-blocks and 6028
coefficient samples, synthesised 1024 samples, wrote 2048 output bytes and
reported 41136 scratch bytes with zero full coefficient-plane capacity.
Reported counters are codec work/capacity observations, not total process RSS
or physical storage operations. No full-image allocation or full payload scan
was observed. Inspection and new regional plans still traverse tile-part
metadata; repeated execution of a retained plan reads selected codewords only.
A reusable header index across different regions is a separate optimisation.

The real source already worked at the baseline codec revision. Its plugin
admission, GDAL routing, comparison against an independent decoder and larger
preparation journeys belong to composed qualification; this component record
does not claim those outcomes from the codec-only probe.
