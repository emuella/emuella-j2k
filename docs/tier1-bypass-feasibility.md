# Native D2 selective-bypass exploration

Native D2 selective bypass is implemented through the additive
`encode_lossless_bypass_with_limits` and `lossless_bypass_encode_requirements`
APIs. Authored tests, independent full-image interoperability, bounded resource
observations and actual-facade reserved measurements pass for the candidate.
Existing encode entry points retain style zero. Final qualification of the
fixed 48-product cohort from merged owner revisions remains outstanding.

## Initial exploration

The starting codec revision is
`e1df5a8353c11bcc5e812ff3d3a2a0e46e77341e`.

The initial question was whether the existing authored Tier-1 selective-bypass engine
can provide a useful native D2 encode/decode/size operating point when its
actual terminated segment lengths reach packet assembly. The first probe was
the fixture-only serial bridge in `scalable_lossless/bypass_probe.rs`, using
the ordinary coefficient conversion, reversible colour transform, two-level
5/3 transform and native decoder. RGB retains reversible MCT; grey and eight
native components retain no MCT. Style zero remains the default.

The smallest authored checks cover actual five- and six-magnitude-plane block
groups, independently specified packet length bits and odd-sized native D2
round trips. The opt-in `lossless_bypass` example accepts packed raw input,
writes one generated codestream after native sample verification and reports
identities. It emits no operation timings and provides no performance
acceptance evidence. The `decode` mode verifies an independently supplied
codestream against the supplied raw samples. Only use inputs and destinations
with applicable rights.

Codec-owned experiment records must capture exact source/tree and binary
identities, authored or authorised input identities, observed native and
independent-codec results, and a retain/reject decision. External codec outputs
remain outside the public source tree. No external codec source is an
implementation input.

The first synthetic checkpoint exercises actual native and independently
observed streams in both directions, actual raw stuffing, authored packet
length bits and malformed-input observations. Structural admission alone does
not establish entropy support. A useful full-image operating point must then
justify a narrow opt-in API, bounded serial/parallel metadata propagation,
resource accounting, failure atomicity and full-image qualification before
ordinary reviewed delivery. Existing public options, timing and resource
struct shapes, default bytes and geometry admission remain acceptance gates.

The applicable normative authority is ISO/IEC 15444-1:2024 / ITU-T T.800 V4
(07/2024), A.6.1, B.10.1, B.10.6–B.10.7 and D.6, including Table D.9,
reviewed retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`.
Implementation and explanations are project-authored.

## Completed synthetic checkpoint

All four focused `selective_bypass` tests pass at revision
`813b7872392fd1f92bfcb1ca760d7be4738a393a`, tree
`285b8652e1d78419b76bf7fc361c7ae093f37404`. They exercise actual five- and
six-magnitude-plane segment groups, independently specified packet-length
bits, odd 133×129 native round trips and malformed-input observations. The
observation test records decoder behaviour; a passing test does not mean every
mutation was rejected.

All fifteen synthetic interoperability comparisons are sample-exact. The five
sample models are grey U8/U16, RGB RCT U8/U16 and eight-component U16. Each
133×129 authored full-range input was checked through native style 0 to
OpenJPEG, native style 1 to OpenJPEG and OpenJPEG style 1 to native. All fifteen
streams' COD fields agree with D2, reversible 5/3, 64×64 blocks, one layer,
LRCP and the intended MCT policy. These are black-box observations using
OpenJPEG 2.5.4; no external codec source informed the implementation.

The interoperability executable was built with `--profile perf`, without SIMD
or parallel features, from revision
`87141c13950f0de02ba4266d901391b3d719a072`, tree
`c3f83bcc3fa7e3fe747a262f8b027835315b1d41`. Its SHA-256 is
`9369bfb6b224925b386cb8929e2295cc1e9c2e04e38e8e8ea2cf08237df0fa2a`.
The later focused-test revision only adds opt-in capture of the already
authored malformed block bytes and partition; production encoder, decoder and
example code are unchanged. The executable remains attributed to its actual
build revision.

The external codec-owned `feasibility.json` record has SHA-256
`90e543c794686eb535b2e5dd291c21abb4fab2d7168f4cee35177d2cbab6b618`.
It binds source and tool identities, input and stream hashes, observed COD
fields, focused-test results, exact malformed bytes and their partition, and
the normative interpretation. Synthetic inputs, external codec outputs and
detailed logs remain outside the public source tree.

## Malformed observations and interpretation

The retained authored 64×64 LL block alternates coefficients 31 and −17.
It has five magnitude planes and thirteen passes: initial MQ bytes 0..1543,
raw bytes 1543..2090 and final cleanup bytes 2090..2092. The raw segment contains
actual `FF7F` pairs. Replacing the pair at offsets 1543/1544 with `FF80` was
accepted with changed coefficients. Replacing its final byte at offset 2089
with `FF` was accepted and retained exact coefficients in this case.

D.6 specifies zero stuffing for the encoder after `FF` and excludes `FF` as
the completed raw segment's final byte; the decoder discards the stuffed bit.
These encoder-syntax violations do not establish a normative requirement for
decoder rejection. D.6 also permits truncated raw input without predictable
termination through synthetic `FF` input. Requiring every decision bit to be
physically present would therefore be an unjustified restriction. B.10.7's
whole-block packet-contribution ending rule is a separate boundary from an
internal raw segment ending.

Declared length truncation/inflation, an Lblock u8 overflow and non-zero
packet-header stuffing were rejected. A one-byte MQ/raw boundary shift was
accepted with changed coefficients, and a 33-bit length field was accepted.
Neither observation alone proves invalid input: a different partition can
describe different coefficients, and B.10.7.1 permits non-minimal length-field
widths within the implementation's value and resource bounds. Segmentation
symbols and predictable termination remain outside this experiment. No
checksum-like detection guarantee or shared decoder acceptance repair follows
from these observations. Canonical follow-up used D.6, physical PDF pages
125–126, and B.10.7/B.10.7.1, physical PDF page 93, at the retrieval above.

## Development operating point and decision

Retain D2 style 1 for narrow opt-in production integration. On the fixed
Mansfield acquisition `94_104001000B823500`, all four products improve both
encode and decode latency under the unchanged 5% practical threshold and
99% interval gate. RGB8 costs 0.373% more complete codestream bytes; each U16
product is slightly smaller. This is development selection evidence, not
reserved-acquisition or stable API qualification.

The exact codec source is `d214ecd64c3d60d5f0294acc7383db60c764d1bf`, tree
`0fa2240c02976608401673771ca2915e097ddda0`. The Rust 1.97.1 tuned `perf`
build enables `parallel`, omits SIMD and creates one local Rayon worker.
Its `lossless_bypass_batch` executable SHA-256 is
`78b39171dc2a7e2cb7c27bdb592a4b8a1966679a78a1cd5cc403442b01b8cf45`.
Three example tests and four focused codec tests pass. Authored zero and
unsigned-midpoint constants cover grey U8/U16, RGB U8/U16 and eight-component
U16 with both styles; no pass-count admission change was needed.

All twelve full-image interoperability comparisons are sample-exact: native
styles 0 and 1 decoded by OpenJPEG 2.5.4, and OpenJPEG style 1 decoded natively,
for each of the four products. The existing prepared inputs retain their
original interleaved layout and hashes. Explicit planar derivatives support
OpenJPEG raw I/O; exact layout-aware byte comparisons bind those derivatives
to the original samples. Each actual stream's SIZ/COD/QCD fields are observed
independently and agree with full dimensions, unsigned unit sampling, D2,
LRCP, 64×64 blocks, one layer, reversible 5/3 and RGB-only reversible MCT.
Pixel-bearing inputs, derivatives and external codec outputs remain in the
approved store; no external implementation source informed this work.

The complete acquisition contains 320 successful fresh-process batches:
twenty AB/BA rounds, zero warmups, one sample per process, a 120-second timeout
and warm loaded input. No batch failed, was retried or was discarded. Hashing,
profile and positive encoder-syntax checks, and exact native reconstruction
occur outside the clock. Both arms time ordinary codestream encode or native
decode to owned component planes. Core facade output packing is excluded from
both arms and must be included at the eventual exposed API boundary.

Relative changes below are style 1 versus style 0 elapsed time; negative is
faster. Brackets give conservative 99% ratio intervals for each comparison.
Sizes count the complete actual codestream, with separate hashes per style.

| Product | Encode change, 99% interval | Decode change, 99% interval | Style 0 bytes | Style 1 bytes | Size change |
| --- | --- | --- | ---: | ---: | ---: |
| RGB16 | −39.63% [−41.12%, −38.11%] | −51.46% [−52.38%, −50.52%] | 2,480,209 | 2,467,937 | −0.495% |
| MS16 | −41.41% [−42.36%, −40.46%] | −53.89% [−54.60%, −53.18%] | 7,414,382 | 7,399,817 | −0.196% |
| PAN16 | −38.25% [−38.86%, −37.63%] | −48.02% [−48.58%, −47.45%] | 13,860,814 | 13,801,207 | −0.430% |
| RGB8 | −8.04% [−8.64%, −7.42%] | −10.59% [−11.41%, −9.77%] | 12,399,691 | 12,445,920 | +0.373% |

The numeric wrapper uses the benchmark owner's exact interval function and
classification expressions at `f78c9c4edc2c606a0037b1445753830781726646`, whose
`src/compare.rs` SHA-256 is
`12fafbd13bd6a17a1aba1ae86c2cf474ad0b86b87b3b5078e4bd40109be653d2`.
It retains conservative 99.5% marginal Student t intervals on independent
batch means and Bonferroni 99% ratio bounds per comparison. AB/BA describes
acquisition order, not paired-t estimation. These intervals do not assert
simultaneous coverage across every product and operation. No multiplied
speedup or aggregate performance claim follows.

The external comparison record SHA-256 is
`c1e6803e0243ab639e922f45aa56cdbc2e660649e8fefa92c602f375a27d46f9`;
the complete batch record SHA-256 is
`936bdeb0f0bc9e41c924c0932309d57b35cd5c3bd83cc2ed9d8c30d422d0d627`.
The config SHA-256 is
`655b7ba6d31a7d4e2b068b83be6355d2e6e2fff54c997e162d1f7234079f2291`.
It binds the exact build, authoritative input identities, both native streams
and per-product interoperability evidence. That evidence binds all three
actual streams, observed profiles, installed executable hashes and command
results, reconstructed output hashes and native decode response identities.

## Production candidate and reserved facade qualification

The production implementation at `4f925a084b9660f7c0fc061028c424358701fb47`,
tree `23f7756f8eafdaf94c38f328bcd3c6fec674df2d`, adds the explicit bypass
entry points, bounded segment metadata and ordered worker-slot propagation.
The fixed 55-length record has a mechanically checked 512-byte maximum;
bypass admission adds `512*B` before choosing worker count. Existing public
struct shapes and style-zero defaults remain unchanged. The
[scalable lossless contract](scalable-lossless.md) owns the API and allocation
rules. Twelve focused tests cover facade layouts/admission/worker budgets,
constant inputs, segment topology, the full 31-plane bound and joined failure
recovery. Canonical verification and independent review passed at
`f41534254a185f1d17818e07fda3fd83e7cf2508`, whose intervening changes only
wire the new parallel test into CI and refresh generated notice identities.

The retained `perf`, parallel, no-SIMD facade batch executable has SHA-256
`5e438f42d50a62fabbf76307fd15a17af404b3c434e0bdc977e5eff0e81056d8`
and remains attributed to its actual `4f925a0` source above. It times public
facade encode and ordinary facade decode, including conversion and packed
output in the requested interleaved layout. It does not substitute a prepared
decoder. Input loading/hashing, profile checks and sample comparisons remain
outside the clock.

Roskilde `6_1040010009874E00`, Billings `100_1040010046CD1500` and Boca Raton
`106_10400100413CDF00` were reserved by acquisition before selection; all twelve
products were retained. Eight products are admitted. Roskilde and Billings
PAN16/RGB8 remain unsupported under both styles' unchanged geometry admission.
For every admitted product, both native styles decode exactly through
OpenJPEG; OpenJPEG style 1 decodes exactly through the facade at one and eight
workers. Native one/eight-worker streams are byte-identical per style. Style
zero also matches the exact merged `e1df5a8` exports. All sixteen successful
allocation observations remain below their recorded queried bounds.

The allocation diagnostic initially rejected Billings MS16 because its local
64 MiB output allowance was below the valid 67,454,525-byte bypass stream.
Diagnostic-only revision `a148a7d0c0472287b0734dc0d42af8df33568d3e` uses
production default limits. The failed invocation and completed prefix were
retained; only unfinished checks resumed. Timing kept the unchanged immutable
facade binary. The separate numeric wrapper also corrected its development
cardinality assertion from twenty to five without changing the benchmark
owner's interval function, critical values or classification expressions.
The initial analysis rejection is retained; no measurement was repeated.

Reserved timing was fixed at five AB/BA rounds, zero warmups, one sample per
fresh process, one local worker and the same 120-second timeout and 5%/99%
gate. All 160 batches succeeded without timing retries or discarded samples.
All sixteen comparisons classify as improved; none is inconclusive. Negative
changes below mean less elapsed time; brackets are per-comparison 99% bounds.
The larger Boca streams are an explicit cost of selecting this opt-in profile.

| Reserved product | Encode change, 99% interval | Decode change, 99% interval | Style 0 bytes | Style 1 bytes | Size change |
| --- | --- | --- | ---: | ---: | ---: |
| Roskilde MS16 | −39.24% [−40.30%, −38.17%] | −51.01% [−52.17%, −49.84%] | 40,318,204 | 40,666,524 | +0.864% |
| Roskilde RGB16 | −36.16% [−38.76%, −33.49%] | −47.06% [−49.34%, −44.71%] | 13,177,125 | 13,201,775 | +0.187% |
| Billings MS16 | −41.37% [−42.18%, −40.55%] | −53.66% [−54.32%, −53.00%] | 67,342,491 | 67,454,525 | +0.166% |
| Billings RGB16 | −39.11% [−40.14%, −38.06%] | −50.98% [−52.18%, −49.76%] | 22,733,626 | 22,616,124 | −0.517% |
| Boca Raton MS16 | −38.75% [−41.38%, −36.02%] | −50.59% [−52.24%, −48.86%] | 9,103,938 | 9,721,574 | +6.784% |
| Boca Raton PAN16 | −39.34% [−40.84%, −37.81%] | −50.76% [−52.31%, −49.18%] | 16,469,500 | 17,734,505 | +7.681% |
| Boca Raton RGB16 | −38.67% [−42.98%, −34.00%] | −51.79% [−54.18%, −49.26%] | 3,054,801 | 3,122,192 | +2.206% |
| Boca Raton RGB8 | −13.39% [−15.39%, −11.36%] | −18.67% [−20.29%, −17.02%] | 14,339,292 | 14,937,382 | +4.171% |

The reserved result SHA-256 is
`112fa07120a362ce6ed275d5da61ed8c99f2fb76204d7fac44370060d5f01b6f`.
It binds the exact source/build, independent stream/profile identities,
complete byte counts, retained batch record, corrected numeric wrapper,
separate diagnostic provenance and preserved unsupported outcomes. The
ordinary and diagnostic binary revisions are not relabelled as later heads.

## Remaining qualification

The opt-in production candidate and bounded reserved operating point are now
implemented and verified. Final delivery still requires gates against the
completed candidate documentation revision, followed by qualification of all
48 products across the fixed twelve acquisitions from final merged owner
revisions. Forty supported and eight geometry-excluded outcomes must remain
visible. This final merged cohort is pending, not inferred from the smaller
reserved set. No broader coding style, geometry, decoder rejection policy,
release or general conformance claim follows from these results.
