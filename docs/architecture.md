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

## Rendered-canvas geometry boundary

The codestream crate can plan the full-resolution default rendered canvas
without rendering pixels. It derives horizontal and vertical canvas spacing
independently as the greatest common divisor of the corresponding non-zero SIZ
component separations, then maps the absolute, non-empty image bounds to that
grid with checked ceiling division. The plan retains the absolute rendered
origin, dimensions, axis spacing and each component's checked native-grid
bounds. It is a geometry contract, not a resampling operation.

Public native component output remains unchanged. Unequal-grid rendered decode
continues to fail closed while CRG registration, interpolation phase and
kernel, edge extension, arithmetic precision, rounding, clipping and rendered
sample storage remain deliberately unselected. Existing resolution reductions
continue to describe native component reconstruction; no rendered-reduction
interpretation is selected by canvas planning.

This boundary follows the image, component, registration and reduced-grid
navigation in ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.9.1; Annex B,
B.1–B.5; and Annex I, I.5.3.1.1. The reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The description and deterministic
tests are project-authored and do not reproduce standards prose, equations or
tables.
