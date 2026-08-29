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
continues to fail closed except for the bounded full-frame sYCC projection
below. Existing resolution reductions continue to describe native component
reconstruction; no rendered-reduction interpretation is selected by
common-grid or JP2 default-image planning.

The neutral SIZ arithmetic follows the image, component, registration and
reduced-grid navigation in ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.9.1,
and Annex B, B.1–B.5. JP2 default-image selection follows Annex I,
I.5.3.1.1. The reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The description and deterministic
tests are project-authored and do not reproduce standards prose, equations or
tables.

## Bounded full-frame sYCC projection

The core admits one direct, unsigned 8-bit JP2 sYCC projection. The first
component is Y on a 1 × 1 SIZ grid and the second and third components are Cb
and Cr on matching 2 × 2 grids. The input must contain exactly one contiguous
codestream and one enumerated sYCC colour specification, no multiple-component
transform, and no palette, component mapping or channel definition. This also
excludes alpha, ICC interpretation and additional colour specifications. The
native codestream must satisfy the existing single-tile subsampled-component
decoder gate, including zero image and tile origins. CRG may be absent or may
contain exactly one all-zero registration pair for each of the three
components. Every nearby profile remains unsupported, including signed or
higher-precision samples, different sampling, non-zero registration, multiple
codestreams and indirect component mappings.

With zero registration, chroma sample `(x / 2, y / 2)` is selected for output
sample `(x, y)`, using integer division. This is a nearest-neighbour,
zero-order hold over each 2 × 2 block; the last chroma sample is extended to an
odd right or bottom edge. The selected Y, Cb and Cr values are converted using
binary64 arithmetic and the sYCC coefficients `1.402`, `0.34413`, `0.71414`
and `1.772`, with a chroma centre of 128. Each result is rounded to the nearest
integer, with halfway cases away from zero, then clipped to `[0, 255]`. Output
is three unsigned 8-bit sRGB channels at the full SIZ image width and height,
stored in the caller-selected planar or interleaved layout.

Only ordinary full-frame rendered decode and shape discovery select this
policy. Raw component and partial decode continue to return native component
grids without colour conversion or resampling, and raw J2K rendered decode
continues to fail closed. The component locations and sYCC interpretation
follow ISO/IEC 15444-1:2024, Annex B, B.1–B.2, Annex I, I.5.3.3 and Table I.10,
and J.14, PDF pages 79–80, 171–173 and 216. The full-resolution sRGB
qualification boundary follows ISO/IEC 15444-4:2024, Annex G, G.1–G.4 and
Table G.1, PDF pages 47–49. The reviewed retrieval revisions were
`34e5d1639b9f121807e620c001893ca9d2c8f977` and
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7` respectively. The resampling
policy and its synthetic fixtures are project-authored because Part 1 does not
select an interpolation kernel.

## Baseline JP2 header admission

JP2 parsing admits one structural header before the first contiguous
codestream. The image header is first and singular, colour specification boxes
form one non-empty contiguous sequence, and bits-per-component metadata is
present exactly when component precision or signedness varies. Legal field
ranges and component cardinality are checked at the container boundary.

The core then compares image width, height, component count, precision and
signedness with SIZ in the first contiguous codestream. A mismatch is invalid
metadata, not a request to prefer one source, and is rejected before decode can
write caller-owned output. Unknown boxes retain the existing byte-preserving
behaviour unless they interrupt a required sequence.

This admission establishes container and codestream consistency only. It does
not produce presentation pixels or select palette application, component
mapping, channel or alpha interpretation, ICC transforms, colour conversion,
resampling, resolution handling or multiple-codestream composition. The known
unimplemented presentation metadata is reported by support classification
without making an otherwise well-formed container invalid. Component-mode
decode may still expose admitted raw codestream planes and applies no
presentation transform. Component output metadata is inferred from the raw
selection: all one- and three-component outputs are labelled grayscale and RGB
respectively, while other counts and explicit subsets remain unknown. JP2
colour metadata does not alter that inference. Except for the bounded sYCC
profile above, rendered requests fail closed before output allocation or
mutation for palette, component mapping, channel definition, sYCC, ICC,
vendor, reserved or unrecognised colour metadata.
Partial decode requests are native component selections; their Part 1
full-decode compatibility fallback therefore also uses component mode.

The authority is ISO/IEC 15444-1:2024, Annex A, A.5.1, PDF pages 41–44, and
Annex I, I.2.2, I.5.3–I.5.4, PDF pages 160–171 and 181. The reviewed retrieval
revision was `34e5d1639b9f121807e620c001893ca9d2c8f977`.
