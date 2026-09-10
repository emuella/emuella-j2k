# Native D2 selective-bypass exploration

This is a provisional codec experiment. No selective-bypass public encode
profile has been accepted. The starting codec revision is
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
codestream against the supplied raw samples. Only use inputs and destinations with applicable rights.

Codec-owned experiment records must capture exact source/tree and binary
identities, authored or authorised input identities, observed native and
independent-codec results, and a retain/reject decision. External codec outputs
remain outside the public source tree. No external codec source is an
implementation input.

Exit feasibility only after actual native and independently observed streams
work in both directions, including raw and packet-header stuffing. Qualify
malformed segment boundaries, truncated/inflated lengths, Lblock overflow,
non-zero stuffed bits and forbidden contribution endings. Structural admission
alone does not establish entropy support. A useful operating point must then
justify a narrow opt-in API, bounded serial/parallel metadata propagation,
resource accounting, failure atomicity and full-image qualification before
ordinary reviewed delivery. Existing public options, timing and resource
struct shapes, default bytes and geometry admission remain acceptance gates.

The applicable normative authority is ISO/IEC 15444-1:2024 / ITU-T T.800 V4
(07/2024), A.6.1, B.10.1, B.10.6–B.10.7 and D.6, including Table D.9,
reviewed retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`.
Implementation and explanations are project-authored.

Execution remains a provisional checkpoint. Focused authored correctness and
authorised black-box interoperability may proceed in a shared-host interval;
performance observations require a separately released quiet host.
