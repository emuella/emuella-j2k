# Third-party notices and provenance

The Python extension statically links the Emuella codec crates into one native
extension module. Its distribution metadata therefore describes the combined
licensing obligations of the embedded implementation, not only the
project-authored Python binding layer.

## OpenJPH-derived HTJ2K implementation

Upstream project: `https://github.com/aous72/OpenJPH`

Pinned upstream commit: `2d0a033a135fb58dab87ea9551db8870e5b68548`

The extension embeds the OpenJPH-derived files enumerated in the repository's
root `THIRD_PARTY.md`, including the HT VLC tables, scalar encoder and decoder
paths, prepared MEL/VLC/MagSgn readers, AVX2 cleanup kernel, and reversible
code-block transfer. Those sources were translated and adapted to checked Rust
interfaces and isolated into BSD-2-Clause provenance-bearing modules.

Upstream paths and SHA-256 values at the pinned commit:

| Upstream path | SHA-256 |
| --- | --- |
| `src/core/coding/ojph_block_encoder.cpp` | `de2eabe213073eff7fd49dbde4f282e3db0f5c0315092a07258d13bdceedc3d2` |
| `src/core/coding/table0.h` | `72d1ebd3ec1822c3d11aabe9e48c4101593ba7a255e3399a986c8dd6117ce26b` |
| `src/core/coding/table1.h` | `7cfc9a69ac11f37fe6c58f6f474333b3e0c4fd74e24b7bef3418b32141793717` |
| `src/core/coding/ojph_block_common.cpp` | `4bb114b652c1214eb23cc604bddeed11f1b8e21398c5344d5aad0d82de4b9429` |
| `src/core/coding/ojph_block_decoder32.cpp` | `6cc2ef065143c4fdc2e285010b5ebd51d082be695b7272b1d29af33b412abb49` |
| `src/core/coding/ojph_block_decoder64.cpp` | `e4a480b4fece9b3b9969f9cb27d4af717886defb229a383536e1c7239c7c3870` |
| `src/core/coding/ojph_block_decoder_avx2.cpp` | `11d9099ccd5a6b4a86a5f71f54b6dc417b9169ad7931b73d61940b7ccad71fd3` |
| `src/core/codestream/ojph_codestream_gen.cpp` | `7a0a33d9cc5f9d52404ba4be3104c1c6a4b28d9f7e267e889d58450ba40afeb9` |
| `LICENSE` | `5ddf5177863dfc9ab65fa129d587db651241f00e21ed2427b218bea997591f98` |

The applicable licence text and notices are reproduced in
`LICENSE-OPENJPH-BSD-2-CLAUSE`.

## Statically linked Rust dependencies

The native extension statically links third-party Rust crates resolved from
the repository's exact `Cargo.lock`. `THIRD_PARTY_DEPENDENCIES.md` records each
external package's version, locked source and checksum, SPDX licence
expression, binary role, and bundled notice paths. Applicable runtime licence
and notice texts are reproduced under `THIRD_PARTY_LICENSES/`. Build-only and
development-only dependencies are inventoried but are not represented as code
in the installed extension.

## Project-authored classic Tier-1 implementation

The extension also embeds `emuella-j2k-tier1`. That crate was rewritten as
project-authored Rust directly from the normative ISO/IEC 15444-1:2024 model
and is licensed under Apache-2.0. It does not contribute an additional
third-party licence obligation. Its implementation basis and qualification
record are documented in the repository's `PROVENANCE.md` and
`docs/tier1-implementation.md`.

## Distribution metadata

The Python project declares `Apache-2.0 AND BSD-2-Clause` because the native
extension combines project-authored Apache-2.0 code with the OpenJPH-derived
BSD-2-Clause modules. Its explicit `license-files` list includes the Apache
licence, the OpenJPH BSD licence, this provenance record, `NOTICE`, the locked
dependency inventory, and all applicable runtime dependency notices; PEP
639-aware build tools must include those files in source and binary
distributions and record each through Core Metadata `License-File` fields.
