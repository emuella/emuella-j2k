# Indexed HT representation efficiency exploration

This finite investigation preserves the existing encoder defaults, per-tile
byte allowance, descriptor syntax, decoder, component order and native precision.
No external codec implementation source is an input. Temporary experimental
instrumentation and policies are removed unless separately qualified as an
explicit additive policy. Historical viewer quality and runtime failures remain.

The baseline measures every tile search index, candidate byte length, final
index and visit count. Each candidate records component/subband entropy-body
bytes, quantisation step, sample count, transformed-domain squared reconstruction
error and zero population. These are numerical aggregates, never coefficient or
packet payloads. Transformed-domain error is diagnostic, not spatial or display
quality and not a surrogate for the frozen acceptance score.

## Frozen mechanism and budget

The authored encoder truncates absolute coefficients to a scalar magnitude index.
Its decoder reconstructs zero at zero and a nonzero index q at (q + 0.5) times
the step. The first decision boundary is currently one step, although the
midpoint between the zero and first nonzero reconstruction values is 0.75 steps.
The related mechanism family changes only the first zero-bin boundary to 0.875,
0.75 or 0.5 steps. All later bins and decoder reconstruction remain unchanged.
The last threshold deliberately explores beyond the nearest-midpoint boundary;
it may increase distortion. Rate search remains independently bounded per tile,
so extra nonzero coefficients may require a coarser selected step.

Before any probe, baseline diagnostics estimate each threshold's fixed-step
error and changed population. No weighting, component budget transfer, cross-tile
borrowing, extra rates or display-coordinate optimisation is part of this family.
One baseline and at most these three probes run once each on Mansfield RGB16 at
12 total bpp, Mansfield PAN16 at 4 and Tok RGB8 at 4: at most twelve serial screen
encodes, no measured retries, thirty minutes per invocation and sixteen GiB of
new discovery output. Existing admission and workspace limits remain unchanged.
The original TIFFs, selected bands, tile support, views, stretches and scale-one
and scale-four quality functions are bound by the consumer's frozen protocol.

No untouched validation is claimed for Mansfield, Boca or Tok. A promoted policy
requires the separately bound reserved cases, full-scene ANY-valid quality,
independent decoder reconstruction, regional/native/WASM and merged-owner proof.
Missing mandatory evidence blocks promotion. At most one policy can be frozen;
reserved results cannot drive retuning. Rejection retains all outcomes and removes
unsuccessful production experiments. Final findings and evidence follow below.

Selection was bound before any probe encode or result: at least one RGB product
must newly pass every existing frozen screen quality and actual-byte gate, and
PAN16 must retain its pass and resource contract. A PAN16-only gain is insufficient.
Rank eligible policies by number of newly passing RGB products, then lowest
worst RGB16 frozen-cell RMSE, then lower actual image payload, then the threshold
closest to baseline. Fixed-step transformed error cannot select a policy.
