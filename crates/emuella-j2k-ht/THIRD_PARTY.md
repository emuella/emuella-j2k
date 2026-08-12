# Third-party notices and provenance

Most of this package is project-authored and licensed under Apache-2.0. The
isolated files below retain their upstream BSD-2-Clause licence. No OpenJPH
binaries or test fixtures are included.

## OpenJPH-derived HT block coding

Upstream project: `https://github.com/aous72/OpenJPH`

Pinned upstream commit: `2d0a033a135fb58dab87ea9551db8870e5b68548`

### Package files

| Package path | OpenJPH-derived or aligned material | Emuella modifications |
| --- | --- | --- |
| `src/block_encoder.rs` | Scalar HT cleanup encoder structure and coding logic | Safe Rust translation, checked coefficient representation, structured errors |
| `src/ht_vlc_tables.rs` | Initial and non-initial HT cleanup VLC entries and packing logic | Typed Rust source entries and `const fn` lookup/encoder-table construction |
| `src/openjph_decoder.rs` | 32-bit cleanup bitplane bound and sign-magnitude coefficient reconstruction | Checked arithmetic, structured errors, typed cleanup-coefficient input |
| `src/openjph_fast_cleanup.rs` | MEL, VLC, and MagSgn reader architecture and prepared de-stuffing | Checked typed cursors, reusable prepared storage, backend dispatch, scalar fallbacks |

### Upstream files consulted

| Upstream path | SHA-256 at the pinned commit | Emuella use |
| --- | --- | --- |
| `src/core/coding/ojph_block_encoder.cpp` | `de2eabe213073eff7fd49dbde4f282e3db0f5c0315092a07258d13bdceedc3d2` | Scalar HT cleanup encoder |
| `src/core/coding/table0.h` | `72d1ebd3ec1822c3d11aabe9e48c4101593ba7a255e3399a986c8dd6117ce26b` | Initial-line HT cleanup VLC entries |
| `src/core/coding/table1.h` | `7cfc9a69ac11f37fe6c58f6f474333b3e0c4fd74e24b7bef3418b32141793717` | Non-initial-line HT cleanup VLC entries |
| `src/core/coding/ojph_block_common.cpp` | `4bb114b652c1214eb23cc604bddeed11f1b8e21398c5344d5aad0d82de4b9429` | VLC lookup-table packing logic |
| `src/core/coding/ojph_block_decoder32.cpp` | `6cc2ef065143c4fdc2e285010b5ebd51d082be695b7272b1d29af33b412abb49` | Scalar 32-bit cleanup reconstruction and bitstream readers |
| `src/core/coding/ojph_block_decoder64.cpp` | `e4a480b4fece9b3b9969f9cb27d4af717886defb229a383536e1c7239c7c3870` | 64-bit reservoir reader structure |
| `src/core/coding/ojph_block_decoder_avx2.cpp` | `11d9099ccd5a6b4a86a5f71f54b6dc417b9169ad7931b73d61940b7ccad71fd3` | Prepared de-stuffing architecture |
| `LICENSE` | `5ddf5177863dfc9ab65fa129d587db651241f00e21ed2427b218bea997591f98` | BSD 2-Clause licence text |

The complete BSD 2-Clause text and applicable notices are reproduced in
`LICENSE-BSD-2-CLAUSE`. This package declares
`Apache-2.0 AND BSD-2-Clause`; the files named above carry BSD-2-Clause SPDX
headers, upstream copyright notices, pinned sources, and modification
summaries.
