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

## Bounded reversible HTJ2K encode and JPH output

`encode_htj2k` writes raw HTJ2K codestreams only. Its
`Htj2kEncodeOptions::decomposition_levels` field admits zero or one: zero keeps
the established cleanup-only byte path, while one applies one reversible 5/3
level before the same repo-owned HT cleanup block boundary. Two or more levels
fail explicitly. The additive `encode_htj2k_jph` entry point accepts the same
options and input matrix. It calls the raw encoder once, then places those exact
bytes in one contiguous codestream box behind deterministic JPH file-type,
image-header and enumerated greyscale or sRGB colour boxes. Container writing
does not introduce a second HT encoding route.

The JPH reader and writer share one bounded Annex D container contract. A JPH
file has exactly one JPEG 2000 signature, followed immediately by exactly one
file-type box, and exactly one `jp2h` after that file type and before the first
`jp2c`. The file-type brand is `jph `, its minor version is zero, and its
compatibility list contains both `jph ` and inherited `jp2 ` membership;
repeated compatible-brand membership is harmless. The inherited `jp2h`
boundary requires `ihdr` first, conditionally admits one `bpcc`, keeps `colr`
boxes contiguous, and validates the cardinality and payload structure of
`pclr`, `cmap`, `cdef` and `res `. Palette and component-mapping dependencies,
component and palette selectors, channel-definition domains, and resolution
children are checked before optional presentation is classified. JP2 still
requires `colr`; JPH may omit it only for an `ihdr` whose colourspace is
unknown, and such a header cannot describe a type-0 colour channel. The JPH
single-alpha and application-specified channel rules are applied without
changing JP2 rules. At least one `jp2c` is required, every `jp2c` must contain a
structurally parsed HTJ2K codestream with EOC at the end of the box payload,
and the first codestream SIZ dimensions, component count, precision and
signedness must agree with `ihdr` and `bpcc`. Checked box arithmetic, field
domains and exact containing-box bounds precede metadata allocation. Legal
unknown boxes remain byte-preserved.

This validation is container and codestream admission, not presentation or
decode expansion. Palette, component mapping, channel definition, alpha, ICC,
unrecognised colour interpretation, multiple-codestream composition, HTMIX and
codec profiles outside the existing decoder remain unsupported. A structurally
valid file can therefore inspect as unsupported; structural contradictions are
rejected first. Caller-owned decode routes inspect and size the complete input
before publication, so invalid JPH input cannot partially change output. The
deterministic writer emits one codestream and preserves the raw
`encode_htj2k` payload byte-for-byte.

The one-level matrix is one tile, one layer, LRCP, default precincts, 64 × 64
code-blocks, HT-only cleanup coding, zero origins, no component subsampling and
no multiple-component transform. It accepts greyscale or RGB, planar or
interleaved input, and unsigned `U8` or 16-bit `U16_LE` storage. The RGB route
transforms each native component independently, so component decode returns the
original native red, green and blue samples without an inverse colour
transform. The one-level `U16_LE` route admits the complete unsigned native
range. A reversible 5/3 level can produce 17-bit transformed magnitudes: the
two polarities of the 2 × 2 full-range checkerboard produce HH coefficients of
−131070 and 131070 and signal a QCD exponent of 17.

The project-authored direct scalar cleanup materialiser holds per-quad MagSgn
values and masks in `u32`. A 17-bit transformed magnitude can use up to 18
explicit bits once the interleaved sign representation is included. Potential
16- or 17-bit subbands bypass the full-octet 16-bit MagSgn materialiser; lower
depths retain the prepared full-octet and acceleration routes. This fallback is
lossless but may be slower for high-dynamic-range blocks. The full-octet VLC
caller also retains the general initial-nibble reader until a physical VLC
nibble has been loaded, then resumes the existing steady reader. Empty high
subbands in very small one-level images are represented without included code
blocks. Byte-aligned cleanup termination treats an empty partial MEL byte
explicitly rather than shifting an eight-bit value by eight.

The marker and transform route follows ISO/IEC 15444-15:2019, clauses
6.1–6.3 and normative Annex A, A.1 and A.4, PDF pages 10–11 and 35–38, and
ISO/IEC 15444-1:2024, Annex A, A.6.1–A.6.2, Annex B, B.5 and B.8, and Annex F,
F.2.2, PDF pages 46–51, 85–86, 88–89 and 132. The Part 15 retrieval revision is
`10baf9472429d52f5d6b5f9b7a892dbed395b1db` (reviewed bundle
`e7d1936131227fae2d3f8315309de4dedc83eb3f`); the Part 1 retrieval revision is
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`). Part 15:2019 refers to the 2019
Part 1 edition, whereas the locally reviewed normative core is Part 1:2024.
This bounded profile relies only on the established COD decomposition count,
reversible-transform signalling, resolution/sub-band organisation and
one-layer packet model; it does not rely on a post-2019 Part 1 feature or use
the later edition to reinterpret Part 15.

The JPH container calibration additionally follows ISO/IEC 15444-15:2019,
normative Annex D, PDF pages 62–64, at retrieval revision
`10baf9472429d52f5d6b5f9b7a892dbed395b1db`, together with its mapped inherited
ISO/IEC 15444-1:2024 Annex I structure, PDF pages 160–181, at retrieval
revision `34e5d1639b9f121807e620c001893ca9d2c8f977`. The implementation and
synthetic tests are project-authored and reproduce no protected standards
text, table or payload.

The decomposition orchestration, transform use, packet construction, scalar
fallback, termination repair, JPH composition, tests and this description are project-authored
from those standards routes. The closed OpenJPH-derived file set, pinned source
revision, headers, attribution, tables and provenance inputs are unchanged.
The independently designed termination repair is contained in the already
allowlisted cleanup encoder file under the approved repository authority; no
external implementation source, test, constant, table, payload or diagnostic
informed it.

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
below. Direct high-precision greyscale rendering has a separate unit-grid
boundary. Existing resolution reductions continue to describe native component
reconstruction; no rendered-reduction interpretation is selected by
common-grid or JP2 default-image planning.

The neutral SIZ arithmetic follows the image, component, registration and
reduced-grid navigation in ISO/IEC 15444-1:2024, Annex A, A.5.1 and A.9.1,
and Annex B, B.1–B.5. JP2 default-image selection follows Annex I,
I.5.3.1.1. The reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977`. The description and deterministic
tests are project-authored and do not reproduce standards prose, equations or
tables.

## Direct high-precision JP2 greyscale projection

The core admits one direct unsigned 9–16-bit JP2 greyscale rendered profile.
The first and only contiguous codestream has one unit-sampled component, zero
image and tile origins, one tile, reversible 5/3 coding, no decomposition and
no multiple-component transform or CRG. The JP2 header has exactly one colour
specification, whose first method is enumerated greyscale; image-header and SIZ
precision and signedness agree. Palette, component mapping, channel definition,
ICC interpretation and additional colour specifications are absent.

Rendered output reuses the native reconstructed plane unchanged. Each code
value occupies one little-endian `u16` word; `SampleFormat.bits_per_sample`
retains the declared precision, `signed` is false, the colour model is
greyscale, and rendered component descriptors do not claim a source component.
There is no second level shift, scaling, clipping, rounding or narrowing at the
container boundary. Shape discovery, owned planar and interleaved decode, and
caller-owned decode all select the same contract before output mutation.

Signed greyscale, precision above 16 bits, RGB, sYCC or mixed-precision
high-depth images, unequal or registered grids, multiple tiles, non-zero
origins, decomposition, indirect mapping, alpha and ICC remain unsupported.
Raw J2K does not acquire JP2 colour semantics, and JPH remains outside this
Part 1 profile. Native component decode is unchanged.

The precision, component and container boundary follows ISO/IEC 15444-1:2024,
Annex A, A.5.1 and Tables A.9 and A.11, Annex B, B.1–B.2, Annex G,
G.1–G.1.2, and Annex I, I.3.5 and I.5.3.1–I.5.3.3, I.5.3.5–I.5.3.6. The
reviewed retrieval revision was
`34e5d1639b9f121807e620c001893ca9d2c8f977` (reviewed bundle
`1a7a03799078b476bf38e91786b979059b4c533d`, source SHA-256
`3b15e13add906b67e6528f13dc69dde999abc9c0d089afa7eccea7075806f5a1`). The
description and synthetic fixtures are project-authored.

## Bounded full-frame and partial sYCC projection

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

The additive `decode_rendered_partial`, `decode_rendered_partial_info` and
`decode_rendered_partial_into` APIs reuse `PartialDecodeOptions` without
changing the native partial API. They admit a non-empty, full-resolution,
image-relative rectangle wholly inside this same direct JP2 profile. The
request must select all channels, no tile and no quality-layer limit. Partial
decode additionally requires reversible 5/3 coding and the existing direct
selective codestream profile; it never selects compatibility or
full-image-decode-and-crop fallback.

For a requested half-open rectangle, the selective source rectangle starts at
the preceding even x and y coordinates and ends at the requested right and
bottom edges. This retains the co-sited chroma needed by odd starts. The
prepared codestream plan reconstructs only that source rectangle into owned
native staging, then the shared projector selects chroma from the absolute
output coordinate and crops surplus luma. Caller-owned publication occurs only
after reconstruction and projection both succeed. Rendered descriptors have
no source-component identity; their origin is the absolute requested x and y,
their separation is one, and their dimensions are the requested dimensions.
The ordinary reversible full-frame sYCC renderer uses this same checked plan
and projector while retaining its existing acceptance of inert top-level
metadata. The partial API applies the stricter direct-metadata boundary above.
The existing irreversible full-frame renderer remains available, but partial
9/7 requests fail closed.

| Boundary | Supported now | Deferred |
| --- | --- | --- |
| Container and colour | One JP2, one Part 1 codestream, one enumerated sYCC specification, direct Y/Cb/Cr | raw rendered output, JPH/HT, indirect mapping, ICC and alpha |
| Samples and grids | unsigned u8, Y 1×1, Cb/Cr 2×2, zero image/tile origins, absent or all-zero CRG | high-depth or direct greyscale partial, other sampling, non-zero origins or CRG |
| Coding and selection | one tile, reversible 5/3, full resolution, all channels, no quality limit | multiple tiles, reductions, quality-layer limits and partial 9/7 |
| Output | planar or RGB-interleaved u8, checked owned staging before caller publication | compatibility fallback and raw component presentation |

Native component decode, including the existing native partial APIs, continues
to return native component grids without colour conversion or resampling. Raw
J2K rendered decode continues to fail closed. The project authority is
ISO/IEC 15444-1:2024, clauses A.5.1, A.9.1, B.1–B.3, I.5.3.1.1,
I.5.3.3 Table I.10 and J.14, reviewed at retrieval revision
`34e5d1639b9f121807e620c001893ca9d2c8f977` in bundle
`1a7a03799078b476bf38e91786b979059b4c533d`. The rendered comparison authority
is ISO/IEC 15444-4:2024, Annex G, reviewed at retrieval revision
`725ecba70e5d03eff3f6ce9626bb9cb08dd4e0c7` in bundle
`7b3d8d60cd4d4f6c056cd108d928b7f99f492aa9`. This description, the
nearest-neighbour hold policy and the synthetic fixtures are project-authored;
no ISO expression is reproduced.

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
colour metadata does not alter that inference. Except for the bounded direct
greyscale and sYCC profiles above, rendered requests fail closed before output
allocation or mutation for high-depth or signed samples, palette, component
mapping, channel definition, sYCC, ICC, vendor, reserved or unrecognised colour
metadata.
Partial decode requests are native component selections; their Part 1
full-decode compatibility fallback therefore also uses component mode.

The authority is ISO/IEC 15444-1:2024, Annex A, A.5.1, PDF pages 41–44, and
Annex I, I.2.2, I.5.3–I.5.4, PDF pages 160–171 and 181. The reviewed retrieval
revision was `34e5d1639b9f121807e620c001893ca9d2c8f977`.
