# Third-party notices and provenance

Most of this package is project-authored and licensed under Apache-2.0. The
isolated file below retains its upstream BSD-2-Clause licence.

## OpenJPH-derived HT cleanup acceleration

Package file: `src/openjph_ht_cleanup.rs`

Upstream project: `https://github.com/aous72/OpenJPH`

Pinned upstream commit: `2d0a033a135fb58dab87ea9551db8870e5b68548`

Upstream file: `src/core/coding/ojph_block_decoder_avx2.cpp`

SHA-256 at the pinned commit:
`11d9099ccd5a6b4a86a5f71f54b6dc417b9169ad7931b73d61940b7ccad71fd3`

Emuella uses the AVX2 octet gather, MagSgn extraction, and coefficient
reconstruction structure. It adapts that structure to checked prepared Rust
slices, Rust intrinsics, typed packed-codeword inputs, Emuella backend
dispatch, and a typed coefficient/predictor result.

The complete BSD 2-Clause text and applicable notices are reproduced in
`LICENSE-BSD-2-CLAUSE`. This package declares
`Apache-2.0 AND BSD-2-Clause`; the named file carries a BSD-2-Clause SPDX
header, upstream copyright notices, the pinned source, and a modification
summary.
