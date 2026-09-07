# Reversible MCT layer continuation qualification

The regional MCT fixture now qualifies real quality-layer continuation, alongside
the empty-packet traversal covered by its original construction. This closes a
coverage gap; the investigation did not demonstrate a decoder defect or require
a change to runtime admission.

## Project-authored regression

The 256×192 RGB fixture retains five reversible decompositions, 19 LRCP layers,
style-zero MQ coding, default precincts, EPH, TLM and PLT. Blocks first enter in
different layers and contribute further coding passes and bytes later. The
schedule varies across component, resolution and block position. Structural
assertions inspect the resulting packet contributions, independently of the
writer's schedule, and require continued blocks in every component/resolution
pair. Exact full-image and selected-component region comparisons use the
project-authored RGB formula, including boundary and image-edge windows.

The test-only construction measures the bytes consumed while decoding completed
coding passes from the final continuous MQ codeword. It selects conservative
prefixes without synthetic input, avoids trailing `0xff`, and checks prefix
reconstruction against the same pass count decoded from the full codeword.
It does not terminate each pass or treat arbitrary byte splits as pass boundaries.

The standards basis is ISO/IEC 15444-1:2024, B.10.7–B.10.8, PDF pages 93–94,
and D.4–D.4.3, PDF pages 124–125, retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977`. Packet contribution lengths and
per-block header state are normative; D.4.3's length-computation discussion is
informative. The conservative decoder-consumption construction is a
project-authored engineering choice, not a mandated encoder algorithm.

## Independent external input

The opt-in `external_mct_layers` test reads the existing GDAL
`autotest/gdrivers/data/nitf/test_jp2_ecw33.ntf` in place. This independently
produced ECW codestream is the same input used by the existing
[NITF regional qualification](nitf-tile-part-count-calibration.md).

- GDAL source revision: `1af54d99959f3b62ba10451a357a969075374663`.
- NITF file length: 2,525 bytes.
- SHA-256: `cc2e868e1b4bb9878703333090d60a5cfa6101ae231fc3bab4233549c154f5ed`.
- C8 input: the 1,592-byte slice beginning at byte 933, borrowed in memory.
- Profile: 200×100, three matching unsigned U8 components, reversible MCT,
  five decompositions, 19 LRCP layers, default precincts and inline EPH.

The test verifies the input identity before using its fixed NITF/PLT offsets.
It maps parsed code-block fragments back to the 342 PLT packet boundaries and
checks component/resolution identity, increasing layer indices, staggered first
inclusion, continued codewords and differing component schedules. Zero-byte
contributions at a packet boundary belong to the preceding packet; they are not
mistaken for a contribution in the next packet.

Observed structure contains five continued blocks. First inclusion occurs in
layers 0, 1, 2 and 4. Active resolutions are 0, 3 and 4; all three components
continue through layer 18, with distinct activity schedules. This sparse input
complements the authored fixture's activity across every component/resolution.

Four positioned-source requests select component 0, 1, 2 and the ordered set
`[2, 0, 1]` for the interior window `(37,23,61,29)`, with all layers and no
reduction. Each reconstructs all three MCT dependencies and checks exact samples
against the existing GDAL qualification's interior ramp oracle, plus caller
padding. No pixel arrays are printed on failure. This proves the selected
interior outputs, not a new full-image losslessness or general conformance claim.

## Reproduction and data boundary

Ordinary tests remain self-contained; this external test is ignored unless
explicitly selected. Supply the already-authorised input from its approved
store. It does not acquire data, invoke an external codec, extract a file or
write decoded pixels:

```sh
EMUELLA_MCT_NITF_INPUT=/approved/gdal/autotest/gdrivers/data/nitf/test_jp2_ecw33.ntf \
  cargo test -p emuella-j2k-test-support --test external_mct_layers -- \
  --ignored --nocapture
```

Repeat with `--features emuella-j2k-core/parallel` before `--` for the parallel
build. An explicitly selected test fails when its input is missing or differs
from the pinned identity.

This is an opt-in external-data qualification using the existing GDAL-owned
input and handling authority. `emuella-testdata` remains unchanged: its locked
ISO suite has no input matching this exact regional envelope, and its existing
decoded-pixel schema does not describe this NITF regional oracle. No catalogue
membership, new rights grant or redistribution permission is implied. External
source and decoded samples remain outside public source and package artefacts.
Reviewed source revisions, canonical checks and final runs are recorded in the
owning pull request's landing evidence.
