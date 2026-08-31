# Bounded JP2 mapped presentation

Full `decode`, `decode_shape`, `decode_into` and `decode_into_with_workspace`
accept a bounded unsigned 8-bit JP2 presentation in `DecodeMode::Rendered`.
Palette expansion, direct and palette component mappings, mixed mappings and
channel definitions produce greyscale, RGB or straight RGBA samples. These are
general metadata operations: source indices, palette columns and logical
channel order are resolved from the input, without recognising fixture bytes.
No public struct or function signature changes are required.

## Admission and output

The codestream must satisfy the [independent native-plane contract](native-planes.md):
classic Part 1, one tile/part, one LRCP layer, zero decompositions, no MCT,
1–4 unsigned 8-bit unit-sampled components and zero origins. Its complete marker,
block, quantisation and resource bounds apply. In particular, native samples
are limited to 16 Mi in total, each image axis to 32,768 and total code-blocks
to 1,048,576. Existing reconstruction is reused without widening that profile.

| Presentation field | Admitted interpretation |
|---|---|
| Container | One JP2 codestream, matching `ihdr`/SIZ |
| Colour | Exactly one enumerated greyscale or sRGB `colr` |
| Palette | 1–1024 rows, 1–255 columns, every column unsigned 8-bit |
| Mapping | At most four logical channels; direct, palette or mixed; repeated native sources and palette columns allowed |
| Channel definitions | Colour associations select grey or R/G/B order; redundant descriptions of the same channel are allowed |
| Opacity | Exactly one whole-image straight channel (`Typ=1`, `Asoc=0`), or none; direct or palette source |
| Output | Unsigned 8-bit greyscale, RGB or RGBA, planar or interleaved, at most 16 Mi expanded samples |
| Requests | All rendered channels, full image and resolution, no quality-layer limit |

`pclr` and `cmap` occur together, including direct-only mappings whose palette
is unused. With no mapping box, native component order establishes logical
channels. Mapping records establish logical channel order independently of
native component order. A `cdef` then assigns those logical channels to display
roles. It must cover all logical channels and required colours; default ordered
colour-only channels omit it. One channel may supply more than one colour.
Distinct channels cannot claim the same defined role. A redundant unspecified
description adds no role and cannot justify a `cdef` for default ordered
colour-only channels. Whole-image and colour-specific opacity assignments
cannot conflict. Straight and premultiplied opacity with association 65535
have no colour association and do not conflict with each other; their
presentation remains unsupported.

RGB is emitted in R/G/B order. With opacity, the output is always R/G/B/A and
`ColorModel::Rgba`; greyscale is repeated into R/G/B. Alpha values are preserved
without premultiplication, unpremultiplication or background compositing.
Nonzero colour samples beneath alpha zero are retained. Rendered component
descriptors use output order, the full unit-sampled image grid and
`source_component: None`, including directly mapped channels.

The crate-private `jp2_presentation::Plan` borrows palette bytes and holds the
resolved source/column/role assignments. Inspection support, shape discovery and
owned decoding use the same admission logic. `Metadata::image` retains its
existing native header summary, including native component count; use
`decode_shape` for the resolved rendered count and colour model. Its
`colour_channels` and `output_components` include alpha for RGBA, consistently
with the existing public shape convention. Shape discovery does not reconstruct
samples and therefore cannot detect a bad packet or an out-of-table index.

Reconstruction and projection complete in private owned buffers before either
caller-owned adapter copies anything. Both layouts preserve row padding and
trailing storage, and caller target layout governs over the layout option.
The 16 Mi expanded-output preflight is independent of the native sample bound
and runs before packet reconstruction or image-sample allocation. Temporary
native, projected and interleaved buffers remain separate bounded allocations;
the output bound is not a claim that peak memory equals output size.

## Errors and neighbouring contracts

Structural errors remain `InvalidInput` (or `TruncatedInput` for incomplete
input): invalid box lengths, absent palette/map dependencies, invalid selectors,
nonzero direct `PCOL`, incomplete definitions, conflicting roles/opacity and
invalid colour associations. Signed resulting greyscale/sRGB channels in
optional mapped/channel-defined metadata are invalid; signed opacity is invalid
under the existing parser contract. Unsigned higher precision and otherwise
valid unsupported mechanisms return `Unsupported`.

This resulting-channel validation deliberately applies at the optional
presentation boundary. It does not tighten the pre-existing direct/native
container-admission contract: direct signed JP2 without optional presentation
remains inspectable with its previous rendered rejection. Signed native palette
indices can still describe unsigned resulting channels, but lie outside this
package's native profile. Native component decode never applies mapping,
channel ordering, palette expansion or alpha; existing native support outside
the new envelope is unchanged.

No palette-index recovery rule is assumed. A decoded index outside `[0, NE)`
returns `Unsupported` with an indeterminate-palette diagnostic. It is neither
clamped nor substituted, and is not labelled a normative malformed-sample
condition. Native access remains available; caller bytes are unchanged.

ICC, sYCC with optional presentation, premultiplied or colour-specific alpha,
unknown/unassociated channel semantics, additional mapped channels, higher
precision, resampling, registration, rendered partial/reduced output,
JPH/HT presentation and multiple codestreams remain outside this route.
Additional colour descriptions are conservatively unsupported; the first
`colr` is never replaced by a later supported description. Unused palette
columns also remain subject to the U8 table restriction. Existing direct
high-precision greyscale, direct sYCC, JPH/HT and native-grid routes retain their
separate boundaries. This is not general JP2 conformance or encoder expansion.

## Independent acceptance evidence

`crates/emuella-j2k-test-support/tests/jp2_presentation.rs` supplies independently
stated native planes and display vectors. Its varied packets use the authored
native fixture builder and project-owned Tier-1 utility, not the production
image encoder. The literal empty-packet RGBA family invokes no encoder at all:
index 128 selects an explicitly authored nonzero RGB row with alpha zero.
Expected display samples never come from production decode or projection.

The matrix covers odd 5×3 images; native counts 1–4; grey/RGB palettes; first
and last accessible palette rows; 1/256/1024-row tables; 255-column tables; direct-only, palette-only
and mixed mappings; all 24 RGBA source/channel orderings and all six RGB
orderings; independent source and column permutations; repeated sources and
columns; direct/palette alpha at 0, 128 and 255; grey-to-RGBA; redundant and
shared channel definitions; shape, descriptor and pixel parity; both layouts;
padded ordinary/workspace callers; and a supplemental 65×67 multiple-block
image built by the existing project fixture encoder. Negative cases cover structural errors,
unsupported metadata/coding/requests, output preflight and late packet/index
failure with sentinel atomicity. Existing native tests and the canonical suite
cover the neighbouring reconstruction contracts.

Run both scalar and parallel configurations:

```sh
cargo test -p emuella-j2k-test-support --test jp2_presentation
cargo test -p emuella-j2k-test-support --features emuella-j2k-core/parallel --test jp2_presentation
```

The shared Tier-1 adapter preserves an explicit `MalformedBitstream` variant as
public `InvalidInput`, including the late invalid-MQ fixture. It previously lost
that distinction. Unsupported coding-style/pass variants remain `Unsupported`;
no diagnostic-string matching or blanket entropy-error reclassification is used.
This fixes a known category loss without claiming detection of every possible
malformed bitstream or changing successful native reconstruction.
Locked external qualification remains a separate source-bound regression
journey; public fixtures contain no protected data or external codec output.

Authority: published normative-core ISO/IEC 15444-1:2024 / ITU-T T.800 (V4),
I.5.3.3 and Tables I.9–I.11 (physical PDF pages 172–173), I.5.3.4–I.5.3.5 and
Tables I.12–I.15 (pages 174–175), I.5.3.6 and Tables I.16–I.19 (pages 176–178),
reviewed retrieval `34e5d1639b9f121807e620c001893ca9d2c8f977`.
The implementation, fixtures and explanations are project-authored; none
reproduce standards expression or derive from another codec implementation.
Engineering admission bounds and out-of-table policy are stated separately
from the normative metadata semantics.
