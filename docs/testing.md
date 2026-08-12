# Testing and corpus boundary

Ordinary tests are Layer 1: small, deterministic, project-authored inputs
created in memory. They require no network, external executable, or adjacent
repository. `emuella-j2k-test-support` contains the shared generator and
round-trip tests.

Optional conformance and interoperability work is Layer 2. A caller may point
a future harness at a materialized `emuella-testdata` pack, but this repository
must not download, vendor, or silently accept terms for that data.

Pinned qualification in CI is Layer 3. It should record the codec revision,
catalogue revision, pack ID and version, and pack digest. Restricted packs must
be pre-provisioned. Missing restricted data is never fetched as a side effect
of `cargo test`.

No Kakadu SDK, OpenJPEG/OpenJPH output, standards insert, PDF, OCR transcript,
or externally sourced fixture is permitted in this repository without a new
file-level provenance and redistribution review.

Library support classification is structural and algorithmic. Exact-payload
classifiers, embedded reference-pixel replay, empty fixture-array placeholders,
and compatibility APIs named after an external codec do not belong in the
runtime crates. Corpus-specific expectations stay in `emuella-testdata` and its
harnesses.
