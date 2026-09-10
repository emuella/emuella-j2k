# Native D2 selective-bypass exploration

Retain this provisional codec experiment: authored native and independent
synthetic decoding establish initial feasibility. No full-image performance
point or selective-bypass public encode profile has been accepted. The starting codec revision is
`e1df5a8353c11bcc5e812ff3d3a2a0e46e77341e`.

The question is whether the existing authored Tier-1 selective-bypass engine
can provide a useful native D2 encode/decode/size operating point when its
actual terminated segment lengths reach packet assembly. The first probe is
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

## Remaining decision

Retain the probe for a separate quiet-host development comparison
of style 0 and style 1. This checkpoint makes no full-image, CPU, allocation,
latency or throughput claim; no full-image probe or performance measurement has
run. A useful measured encode/decode/size point must precede selection of a
stable public API or a parallel style-1 profile. Bounded metadata propagation,
resource qualification, failure atomicity, full-image correctness and the
existing performance acceptance gate remain outstanding.

The prepared `lossless_bypass_batch` example provides one fresh-process batch
for that comparison. It requires the tuned `perf` build with `parallel`
enabled, SIMD disabled and a local one-worker pool for both styles. The
protocol retains zero warmups, one sample per process, twenty AB/BA rounds and
a 120-second timeout. Input loading, identity checks, generated-stream profile
and encoder-syntax validation, and exact native sample verification occur
outside the operation clock. Both arms measure ordinary codestream encoding
or ordinary native decoding to owned component planes; neither includes core
facade output packing or substitutes a prepared decoder. Complete codestream
sizes and both actual stream hashes remain separate.

The prepared external numeric wrapper reuses the benchmark's exact interval
function and classification expressions at
`f78c9c4edc2c606a0037b1445753830781726646`; the owning `src/compare.rs` SHA-256
is `12fafbd13bd6a17a1aba1ae86c2cf474ad0b86b87b3b5078e4bd40109be653d2`.
It retains conservative 99.5% marginal Student t intervals on independent
batch means, Bonferroni 99% ratio bounds per case and the 5% practical threshold.
AB/BA describes acquisition order, not paired-t estimation. No outliers are
removed and no identical stream identities are invented to compare different
profiles. This adapter and its positive encoder-syntax checks are prepared but
have not yet been compiled, tested or used for image measurements.
