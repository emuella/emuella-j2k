# NITF tile-part count and RCT calibration

- Status: selected candidate implemented and focused verification complete
- Calibration date: 2026-09-04
- Evidence owner: `emuella-j2k`

## Question and bounded input

The question was whether GDAL's `test_jp2_ecw33.ntf` fails because its JPEG
2000 image uses multiple tile-parts, an incomplete count, interleaving, or a
different supported-profile boundary. The probe inspected only the NITF C8
image segment. It did not invoke another JPEG 2000 implementation, alter the
fixture, or retain a derived codestream.

The exact source state was:

- `emuella-j2k` revision
  `75083930b6e533053b7f5c3c4dda7b4a2c0a0c38`;
- GDAL revision `1af54d99959f3b62ba10451a357a969075374663`;
- `gdal-jp2emuella` revision
  `5c43d355491ca0e895b93011159e7321315c4347`, used only to trace the
  consumer call path; and
- GDAL input `autotest/gdrivers/data/nitf/test_jp2_ecw33.ntf`, 2,525 bytes,
  SHA-256
  `cc2e868e1b4bb9878703333090d60a5cfa6101ae231fc3bab4233549c154f5ed`.

The NITF file header declares a 404-byte file header, one 529-byte image
subheader and 1,592 bytes of image data. GDAL therefore presents the C8 image
as `/vsisubfile/933_1592,...`; relative byte zero of that range is the JPEG
2000 SOC marker and relative byte 1,590 is EOC.

## Observed codestream organisation

The bounded marker walk produced this exact sequence, with relative byte
offset and total marker or marker-segment size:

| Marker | Offset | Bytes |
| --- | ---: | ---: |
| SOC | 0 | 2 |
| SIZ | 2 | 49 |
| COD | 51 | 14 |
| QCD | 65 | 21 |
| COM | 86 | 34 |
| TLM | 120 | 12 |
| SOT | 132 | 12 |
| PLT | 144 | 347 |
| SOD | 491 | 2 |
| EOC | 1,590 | 2 |

SIZ declares a 200×100 reference grid at origin `(0,0)`, a 1024×1024 tile
grid at origin `(0,0)`, and three unsigned eight-bit components with unit
horizontal and vertical separation. The clipped image therefore contains
exactly one tile, index 0.

COD declares LRCP progression, 19 layers, the reversible 5/3 transform, five
decomposition levels, 64×64 code-blocks, default precincts, the reversible
multiple-component transform and EPH markers. With one precinct at each of six
resolution levels, the expected packet count is `19 × 6 × 3 = 342`.

There is exactly one observed tile-part. Its SOT fields are `Isot=0`,
`Psot=1458`, `TPsot=0` and `TNsot=0`. The single complete TLM entry explicitly
names tile 0 and length 1,458; it agrees with SOT and reaches EOC. The PLT has
one ordered series containing 342 packet lengths. Its 342 encoded bytes sum to
1,097, exactly the bytes between SOD and EOC.

Consequently, the fixture does **not** contain multiple tile-parts, an omitted
continuation or interleaving. `TNsot=0` leaves the count unstated in SOT; it
does not mean zero parts or announce another part. The complete marker walk
and TLM independently establish one part.

## Standards basis

The authority consulted was published normative-core ISO/IEC 15444-1:2024,
*Core coding system* (also ITU-T T.800 (V4), 07/2024). Catalogue identity:
source SHA-256
`3b15e13add906b67e6528f13dc69dde999abc9c0d089afa7eccea7075806f5a1`,
reviewed bundle commit
`1a7a03799078b476bf38e91786b979059b4c533d`, retrieval commit
`34e5d1639b9f121807e620c001893ca9d2c8f977`.

Direct requirements and definitions used were:

- A.4.2, Tables A.5 and A.6, PDF pages 39–40 (printed 19–20): tile-parts
  for a tile use ordered indices starting at zero; one part has index zero;
  TNsot may be either the actual total or zero, with zero leaving the total
  unspecified in that header.
- A.5.1, PDF pages 41–44 (printed 21–24): SIZ defines the reference grid,
  tile grid, component precision and sample separation used above.
- A.6.1, Tables A.13–A.17, PDF pages 46–48 (printed 26–28): COD value zero is
  LRCP, the layer count is explicit, EPH use is signalled, and MCT value one
  applies a reversible component transform to components 0–2 when paired with
  the 5/3 filter.
- A.7.1, Tables A.33 and A.34, PDF pages 56–57 (printed 36–37): a TLM list is
  in codestream order, can identify every tile-part and must agree with Psot;
  its entries or non-zero TNsot can establish per-tile counts.
- A.7.3 and Table A.36, PDF pages 58–59 (printed 38–39): PLT lengths describe
  the following packets and use a most-significant-first series of seven-bit
  groups.
- B.11, PDF pages 95–96 (printed 75–76): packet sequences may be divided into
  tile-parts only at packet boundaries, with same-tile part order preserved;
  parts from different tiles may be interleaved.
- B.12.1.1, PDF page 96 (printed 76): LRCP orders packets by layer, resolution,
  component and precinct.
- G.2 and G.2.2, PDF page 154 (printed 134): reversible MCT requires matching
  geometry and depth for the first three components and inverse RCT follows
  inverse wavelet reconstruction.

The conclusion that this particular source has one complete part is an
engineering inference from the complete observed sequence, matching TLM/Psot,
dense `TPsot=0` and terminal EOC. It is not inferred from `TNsot=0` alone.

## Rejection trace

GDAL's NITF driver constructs the bounded `/vsisubfile/` name and opens the
image through `JP2Emuella`. The plugin's `DecodeRegion` requests one band and
region through `emuella_j2k_decode_component_region`. The C boundary creates a
single-component `Part1ComponentDecodeRequest`, then calls
`prepare_part1_decode_from_source` before allocating or executing output.

The source scanner reads the main header, reconciles the complete TLM with
each SOT `Isot`/`Psot`, scans selected tile headers and records payload spans.
`parse_sot` correctly represents raw `TNsot=0` as an unspecified count. The
complete `validate_indexed_tile_part_sequence` correctly accepts it while
requiring dense indices, one observed part for every SIZ tile, consistent
non-zero declarations and any declared total to equal the observed count.

Preparation then classifies the native component profile. Because COD enables
EPH, `validate_supported_native_component_multitile_profile_with_sample_guard`
uses `validate_one_tile_part_per_tile`. That narrower helper rejects any count
other than `Some(1)`, producing:

```text
Unsupported { feature: MarkerSegment, detail: "multi-tile decode requires each tile to declare exactly one tile-part" }
```

This occurs before `plan_tile_region_decode`, the PLT/packet-header walk,
code-block selection, Tier-1 work, synthesis or output. The core diagnostic
`plan_partial_decode_work` is a separate slice-backed path and is not called by
the plugin/C boundary. A read-only counterfactual source overlay that changes
only TNsot from 0 to 1 passes the first gate and exposes the next existing
boundary:

```text
Unsupported { feature: WaveletTransform, detail: "direct selective component output does not split multiple-component-transform inputs" }
```

## Selected implementation candidate

Retain a bounded two-part codec-owned change:

1. For the existing one-part-per-tile EPH profile, accept `TNsot` as either
   unspecified or one **only after** a complete parse has proved exactly one
   retained part per SIZ tile, `TPsot=0`, bounded payloads and consistent
   SOT/EOC structure and any present TLM. Continue rejecting continuations and non-one
   declared totals in this profile.
2. Separate requested output components from transform dependencies in the
   prepared regional path. For the already admitted three-component,
   equal-geometry, equal-depth reversible profile, a request for one rendered
   component must retain packet/code-block work for components 0–2, reconstruct
   their common bounded region, reuse the existing inverse RCT, then publish
   only the requested component. Preserve the one-plane C ABI and existing
   source-component descriptor convention.

Regional planning must remain tile- and synthesis-window-bounded; packet body
reads and work counters must include the three necessary transform inputs,
while output accounting covers only requested planes. All source ranges and
caller geometry must be preflighted before execution. The C boundary must
continue publishing its Rust-owned image only after successful execution.

Rejected alternatives are fixture-specific admission, treating zero as a
literal tile-part count, requiring encoders to emit a non-zero TNsot, changing
the C ABI, or broadly enabling multi-part continuations/interleaving. None is
needed by this input, and the last option would enlarge packet-state and inline
marker qualification without evidence from the fixture.

## Focused probes and exit decision

Commands were run with generated material confined to the registered campaign
scratch directory:

```console
gdalinfo --config NITF_OPEN_UNDERLYING_DS NO -mdd all test_jp2_ecw33.ntf
python3 "$CAMPAIGN_SCRATCH/calibration/structure_probe.py"
CARGO_TARGET_DIR="$CAMPAIGN_SCRATCH/calibration/cargo-target" \
  cargo run --manifest-path "$CAMPAIGN_SCRATCH/calibration/source-probe/Cargo.toml" --quiet
```

The first command identified one 200×100 C8 image without opening an
underlying JPEG 2000 driver. The structural probe produced the marker and
field results recorded above. Codec inspection succeeded after 26 positioned
reads totalling 509 bytes. Full-image and `(37,19,41,23)` preparations both
failed at the TNsot gate with the same metrics; the TNsot-only overlay reached
the MCT gate, also before packet bodies were fetched.

Exploration exits with the candidate above. Implementation should begin with a
small project-authored reversible three-component fixture carrying TLM, PLT,
EPH, LRCP layers and one `TPsot=0/TNsot=0` part, paired with an exact pixel
oracle and a `TNsot=1` equivalence case. Negative probes should preserve dense
index/count/TLM/Psot/truncation checks. General multi-part and interleaving
support remains a separate capability unless a new input demonstrates it is
required.

## Implementation evidence

The selected candidate is implemented without fixture-specific dimensions,
offsets, checksums or payloads. The complete indexed sequence validator remains
authoritative; only the admitted one-part EPH profile treats `TNsot=0` and one
as equivalent after parsing proves one dense `TPsot=0` part for each SIZ tile
and reconciles SOT, TLM, `Psot`, bounded payloads and EOC. Other profiles retain
their previous declared-count requirements, and no continuation or interleaved
packet state was added.

The prepared regional plan now records requested outputs separately from the
three reversible-MCT reconstruction dependencies. It retains only region-owned
packet bodies and code blocks for components 0–2, reconstructs their common
bounded window, applies inverse RCT, then copies only requested planes. Source,
work and synthesis counters include dependencies; caller output counters do
not. Execution stages every dependency and transformed tile before caller
publication.

The project-authored fixture described in `docs/testing.md` supplies exact
full-plane and regional RGB oracles and equivalent `TNsot=0`/`TNsot=1`
codestreams. Focused tests cover marker shape, 19-layer packet progression,
full and boundary-region pixels, positioned-read bounds, retained-work
accounting, padding and failure atomicity, the one-plane C ABI, and malformed
count/index/duplicate/missing/TLM/`Psot`/truncation cases. This evidence
qualifies the narrow candidate only; the external GDAL journey and canonical
clean-tree gate remain coordination-owned integration work.
