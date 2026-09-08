# Independent NITF ingestion calibration

Status: prefix repair selected and independently qualified; reusable source index
is a separate bounded implementation under `docs/part1-source-index.md`.
Evidence owner: `emuella-j2k`.

The first question is why a project-authored 1057×65 unsigned 11-bit signal,
independently encoded with five decompositions and twenty LRCP layers, fails
regional preparation. The smallest probe separates structural parsing, packet
preparation and execution on each of its two tiles. External encoded inputs
remain with the testdata recipe owner; no external payload enters this repository.
Exit requires a diagnosed mechanism and exact full and regional lossless samples,
with precision preserved. Lossy and other fixture profiles are separately measured.

The second question is how an immutable positioned source can reuse validated
main-header and tile-part metadata across different regional requests. A two-tile
authored regression will count positioned reads on first and second regions,
including unrelated tile headers, and exercise failure atomicity. The retained
index must have an explicit memory bound and source lifetime/identity contract.
Exit requires no repeated complete source-header scan, exact regional samples and
bounded retained metadata. Existing HT indexed viewing is outside this change.

Starting codec revision: `3afcfabb24282645c3e101ab3495810d28212dfd`.

## Diagnosed mechanism and selected repair

The raw 8294-byte U11 lossless input has SHA-256
`eab53affe5e3f715c225b770d611fe0adaf025d143f7a7b4315e66111514300f`.
Both tiles and the complete image prepared successfully through the slice route
at the starting revision. Positioned preparation failed at logical packet offsets
7608 and 217. Each failure occurs in the final layer's resolution-zero packet.

The coding-pass-count reader speculatively inspected nine bits before trying
shorter prefixes. A valid short continuation packet can contain another coding
pass and zero additional codeword bytes in a one-byte header. The speculative
read crosses its PLT boundary even though the actual count is only one bit.
Slice reads concealed the defect by looking into the following packet. Source
reads correctly honoured the declared packet boundary and rejected the peek.

The selected repair consumes only enough prefix bits to identify the count.
It preserves the existing codeword mapping, stuffing checks and truncation
errors, without widening source windows or bypassing PLT checks. The ordinary
regression includes a complete one-byte continuation header, a one-bit count at
the final available bit, and an actually truncated longer prefix.

Normative authority is published normative-core ISO/IEC 15444-1:2024,
B.10.6 and Table B.4, physical PDF page 92, with the continuation on page 93;
B.10.7.1 and B.10.8 on page 93 establish contribution lengths and header order.
Canonical retrieval revision: `34e5d1639b9f121807e620c001893ca9d2c8f977`.
The incremental reader is project-authored from that syntax and observed source
behaviour; no independent implementation source was consulted.

## Independent fixture qualification

The optional runtime harness consumes testdata `jpeg-2000/independent-nitf`
version 1 at merged owner revision
`2d519ddaf019f10b9e409ea3338d395438486647`. Each input and complete independent
decoded reference is integrity-bound to the owner's `PROVENANCE.json`.
It does not invoke a generator, embed payloads, acquire material or use the
Emuella encoder. Testdata retains fixture/recipe/provenance ownership.

Four lossless profiles (U11 PAN, U16 grey, U8 RGB and U16 RGB) match the complete
independent OpenJPEG native-sample hash exactly. The U11 lossy PAN initially
failed an exploratory exact-digest comparison. Comparing all 68705 samples
against the locked independent decoder output finds 56 differing samples:
29 at +1 and 27 at −1, peak 1, squared error 56 and RMSE approximately 0.02855.
All differences lie in tile zero; only one is on an image or tile edge.
This is consistent with the existing floating-point irreversible reconstruction
boundary. No entropy or transform modification was selected from this probe.

The selected inter-decoder limit is one native code value, as already used by
`docs/ht-lossy-calibration.md` for the shared irreversible reconstruction. This
is an engineering interoperability limit, not a general Part 1 conformance
claim or permission to alter lossless samples. It is separate from the fixture's
source-to-independent-decode distortion (peak 1, squared error 1995).
The lossy test reads an independently generated PGM in place, verifies its
normalised sample hash against the owner and enforces the peak limit.

Every profile passes complete reconstruction and three interior, edge and
(where present) tile-crossing regions at discard levels zero, one and two.
The one-shot route performs 15 complete and 45 regional requests. A single
retained `Part1SourceIndex` is constructed for each fixture and reused across
all resolutions and windows; its 15 complete, 45 regional and 15 complete-revisit
requests equal one-shot samples and geometry exactly. Complete revisits run
after the regional requests, and retained header-byte/tile-part counts remain
unchanged. Both routes preserve declared sample precision.
Regional samples equal the corresponding complete same-source reconstruction
exactly at all three resolutions. Only full resolution has an independent
sample oracle; reduced results establish regional/full consistency, not a new
independent reduced-decoder claim. The harness allocates complete planes only
for these locked small qualification inputs, not the large ingestion path.

Reproduce after separately generating the independent PGM with the testdata
recipe's OpenJPEG CLI check:

```sh
EMUELLA_INDEPENDENT_NITF_INPUT=/approved/testdata/generated/jpeg-2000/independent-nitf \
EMUELLA_INDEPENDENT_NITF_LOSSY_REFERENCE=/approved/check/pan11-lossy.pgm \
  cargo test -p emuella-j2k-test-support --test independent_nitf -- --ignored --nocapture
```

The test is ignored by ordinary self-contained runs. An explicitly selected run
without the optional pack reports a skip; a supplied pack must contain the exact
inputs and the independently decoded lossy reference or the run fails.
Repeat with `--features emuella-j2k-core/parallel` before `--` for that build.
GDAL composition, the reusable decoder index, large-source bounds, canonical
verification and reviewed source identity remain integration-owned gates.

Focused checks pass: 245 codestream tests (six explicitly ignored), three
precision tests, six reversible-MCT regional tests, the optional five-profile
qualification in default and parallel builds, and strict clippy for that harness.
The retained-index extension also passes both builds on all five independent
TLM/PLT fixtures, preserving the same external lossy limit and exact
indexed/one-shot comparisons. Source-header read exclusion and construction
failure behaviour are covered separately by the source-index regressions.
The composed provisional GDAL/plugin C ABI probe also reports zero mismatches
for all four complete lossless NITF rasters. That observation precedes an exact
candidate commit; final committed integration evidence belongs to the owning PR.
