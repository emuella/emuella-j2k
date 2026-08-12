# Third-party notices and provenance

Most of this package is project-authored and licensed under Apache-2.0. The
isolated file below retains its upstream BSD-2-Clause licence.

## OpenJPH-derived 32-bit cleanup representation and transfer

Package file: `src/openjph_transfer.rs`

Upstream project: `https://github.com/aous72/OpenJPH`

Pinned upstream commit: `2d0a033a135fb58dab87ea9551db8870e5b68548`

Upstream files:

- `src/core/coding/ojph_block_decoder32.cpp`, SHA-256
  `6cc2ef065143c4fdc2e285010b5ebd51d082be695b7272b1d29af33b412abb49`;
- `src/core/codestream/ojph_codestream_gen.cpp`, SHA-256
  `7a0a33d9cc5f9d52404ba4be3104c1c6a4b28d9f7e267e889d58450ba40afeb9`.

Emuella uses the 32-bit cleanup bitplane bound and reversible sign-magnitude
code-block transfer. It expresses these as scalar Rust helpers operating on
typed transfer metadata and signed or sign-magnitude coefficient inputs.

The complete BSD 2-Clause text and applicable notices are reproduced in
`LICENSE-BSD-2-CLAUSE`. This package declares
`Apache-2.0 AND BSD-2-Clause`; the named file carries a BSD-2-Clause SPDX
header, upstream copyright notices, the pinned source, and a modification
summary.
