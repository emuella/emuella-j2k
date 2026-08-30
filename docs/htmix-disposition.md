# HTMIX architecture decision

Status: accepted unsupported boundary.

## Decision

HTMIX decoding is deliberately unsupported. Emuella retains legal mixed
signalling for inspection, rejects structural contradictions it can establish,
and declines mixed packet interpretation and reconstruction. This is a product
boundary, not a statement that mixed codestreams are invalid, a conformance
claim, or a commitment to implement another profile.

The qualification claim remains Profile 0, Class 0, `M_MAGB=18`, HTONLY only.
The locked P0.06 and P0.15 HTMIX points are not applicable to that claim and
are not executed by the derived-set runner. Their not-applicable status is
neither a passing pixel comparison nor evidence of a malformed codestream.
Their HTONLY counterparts have independent qualification.

## Three independent axes

| Axis | Retained representation | Admission consequence |
|---|---|---|
| Code-block population permission | `Part15CodeBlockMode::{HtOnly,HtDeclared,Mixed}` from `Ccap^15` | Native reconstruction requires `HtOnly`; broader population declarations do not establish per-block coder identity. |
| Effective tile-component method | Resolved COD/COC `code_block_style` | Homogeneous classic and homogeneous HT packet grammars are distinct. Mixed signalling needs a packet-dependent code-block decision; bit 6 alone cannot select HT. |
| HT set multiplicity | `multiple_ht_sets_allowed` and parsed `HtCodeBlockCodingSet` records | Declaration-only MULTIHT permission can retain an admitted one-set HTONLY route. Actual additional HT sets remain unsupported by native reconstruction; a proved SINGLEHT contradiction is invalid. |

The catalogue's `HTMIX` label and the Part 15 `MIXED` population permission
must not be used as synonyms for MULTIHT. Population permissions do not prove
which mechanisms actually occur. Even an HTONLY population may use
mixed-capable method signalling; the current decoder still declines that
packet-dependent grammar. Conversely, a broader population declaration with
homogeneous effective HT coding can have safely inspected HT packet metadata
without receiving native decode admission.

Each separately qualified HTONLY reduced, sampled, ROI, high-component and
tile-progression route keeps its own request, transform, effective-state,
resource and presentation envelope. Neither this decision nor a population or
MULTIHT permission combines those envelopes or widens JPH presentation.

## Current enforcement and correction

`parse` retains typed Part 15 capability and validates marker relationships,
reserved method bits and declared restrictions. Its tile-part payload is opaque.
`validate_part15_packet_signalling` proves only bounded packet contradictions;
it deliberately does not apply homogeneous SINGLEHT reasoning to unresolved
mixed code-block grammar. Successful structural inspection is not complete
mixed-packet validity or pixel qualification.

The shared packet walker now rejects effective mixed-capable COD/COC before
reading packet bytes. Previously, the public
`parse_default_precinct_lrcp_packets` route could dispatch on bit 6 alone and
return HT contribution metadata for mixed signalling. That was false metadata,
not implemented mixed-block discovery. The previous architecture already
excluded HTMIX packet-semantic admission and the API documented classic or HT
profiles, not a mixed profile. Returning `UnsupportedConstruct::PacketDecode`
corrects that unintended admission without removing a supported mixed API.

The native profile classifier and each prepared HTONLY branch remain stricter
than structural inspection. Core decode, shape, partial metadata, native and
rendered requests, reusable workspaces, incremental input, best-effort requests
and JPH wrappers cannot obtain mixed reconstruction through a classic or HT
fallback. Optional workspace/probe APIs retain their documented `Ok(None)`
decline contract; ordinary decode returns structured unsupported errors.
Caller-owned output is not published on a decline or structural failure.

## Contracts a different architecture would have to own

These are prerequisites for any separately authorised replacement decision,
not scheduled work or a proposed mixed decoder hidden in the HTONLY path.

- The codestream owner must separate unresolved per-block coding identity from
  a resolved classic or HT contribution. `CodingStyleMarker.entropy_coder` and
  the existing `PacketCodeBlockContribution.ht_coded` boolean are not a mixed
  discovery state machine. Effective main/component/tile method scope must be
  fixed before discovering a block, including unselected components.
- Packet discovery must retain inclusion/tag-tree state, `Lblock`, announced
  pass counts, missing bitplanes and the first non-empty contribution across
  layers and tile parts. The first non-zero segment's length signalling is
  material to mixed HT identification. Absent and placeholder contributions
  cannot be guessed as classic or HT merely to enter an existing length reader.
- A resolved classic block owns its segment continuation, context/reset,
  termination and style semantics. A resolved HT block owns cleanup and
  refinement segment boundaries, placeholder passes, empty/non-empty sets,
  `Z_blk`/`S_blk` and the applicable magnitude/ROI transfer. `DefaultPrecinctSubband`
  currently carries pass and first-HT-set state for an already selected grammar;
  that is not permission to change coder after a failed entropy decode.
- `PendingPacketContribution`, fragment ranges and contribution aggregation
  must preserve the resolved block identity and byte ownership. Neither an HT
  set boundary nor a later quality-layer contribution is a classic/HT switch.
  SINGLEHT validity and native MULTIHT admission must remain separate after
  mixed block discovery. Resource bounds must cover discovery and retained
  state before allocation, not only reconstruction.
- Core prepared plans must consume that typed result and dispatch each block
  to its owning entropy engine before existing coefficient, quantisation, ROI
  and transform seams. Selection cannot skip required validity work, and every
  owned/caller/partial/container route must share admission and atomic output.
  Only independently authored semantic tests and separately authorised exact
  decoded-pixel qualification can establish a new support claim.

## Authority and evidence

The standards basis is ISO/IEC 15444-15:2019, published extension, clauses
8.2–8.3 (PDF page 31), normative Annex A, A.3.2–A.3.3 and A.4, including
Tables A.2–A.4 (PDF pages 36–38), and normative Annex B, B.1–B.3 and Figure B.1
(PDF pages 40–41), reviewed retrieval
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`. The state contracts above are
Emuella engineering requirements inferred from that signalling and its current
implementation, not new normative requirements. This decision does not
reinterpret the referenced Part 1 edition or define a complete mixed decoder.
All explanation and regression fixtures are project-authored; no standards
expression, protected payload or external implementation is reproduced.

The codestream mixed-admission tests cover legal mixed style flags, effective
COC, unsupported-before-payload handling and homogeneous declaration-only
neighbours. The public route matrix covers raw/JPH/JP2 neighbours, full/partial/shape,
workspaces/probes, incremental input, rendered/best-effort requests and padded
caller-buffer atomicity; structural mode contradictions retain invalidity
precedence. Existing HTONLY and SINGLEHT/MULTIHT tests remain regression gates.
The opt-in structural journeys and unchanged derived-set aggregate provide
bounded integration evidence, never a general Part 15 conformance result.
