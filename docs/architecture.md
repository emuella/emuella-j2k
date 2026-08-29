# Architecture

The workspace separates public policy from codec mechanisms:

```text
emuella-j2k
  └── emuella-j2k-core
        ├── emuella-j2k-container
        └── emuella-j2k-codestream
              ├── emuella-j2k-tier1
              ├── emuella-j2k-ht
              └── emuella-j2k-transform
                    └── emuella-j2k-accel
```

`emuella-j2k` is the stable public facade and re-exports the application API
implemented by `emuella-j2k-core`. The core crate owns caller-visible images,
parameters, errors, support classification, and high-level
inspect/decode/encode entry points. Lower-level crates parse boxes and markers,
walk packets, decode or encode code blocks, and apply transforms. Optional
parallel and SIMD paths retain deterministic scalar fallbacks.

Classic block coding in `emuella-j2k-tier1` uses a project-authored Annex C MQ
coder and Annex D coefficient-context model. Its standards basis, design
boundary, and qualification record are documented in
[`tier1-implementation.md`](tier1-implementation.md).

The CLI and Python crates are adapters; they must not become alternate codec
implementations. The test-support crate creates deterministic inputs through
project-owned algorithms. External reference codecs and corpora are never
runtime dependencies.

## Common-grid and JP2 default-image geometry

The codestream crate provides semantically neutral SIZ common-grid arithmetic.
It derives one scalar spacing as the greatest common divisor of the combined
set of every non-zero horizontal and vertical component separation, then maps
absolute, non-empty reference-grid bounds through checked ceiling division by
that same spacing on both axes. The plan retains the absolute common-grid
origin, dimensions, spacing and each component's checked native-grid bounds.
It does not assign presentation meaning or resample samples.

The core crate selects that neutral arithmetic as JP2 default-image geometry
only after the container boundary has identified a JP2 input. Raw J2K remains
a codestream reconstruction input and does not acquire JP2 presentation
semantics. JPH and HTJ2K admission are unchanged.

Public native component output remains unchanged. Unequal-grid rendered decode
continues to fail closed while CRG registration, interpolation phase and
kernel, edge extension, arithmetic precision, rounding, clipping and rendered
sample storage remain deliberately unselected. Existing resolution reductions
continue to describe native component reconstruction; no rendered-reduction
interpretation is selected by common-grid or JP2 default-image planning.

The neutral SIZ arithmetic follows the image, component, registration and
reduced-grid navigation in ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.9.1,
and Annex B, B.1–B.5. JP2 default-image selection follows Annex I,
I.5.3.1.1. The reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The description and deterministic
tests are project-authored and do not reproduce standards prose, equations or
tables.
