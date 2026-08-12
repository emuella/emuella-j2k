# Source provenance

This repository was assembled as a new public snapshot from an explicit
file-level allowlist. It does not contain the private development repository's
Git history, Git objects, refs, reflogs, submodules, ignored working files, or
historical test corpus.

Project-authored Rust source was selected from a privately retained frozen
snapshot, renamed to the Emuella project namespace, and reviewed for embedded
fixture payloads and private-path dependencies. Public documentation,
automation, legal files, the CLI adapter, and the self-contained test and
fixture-generation crate were created for this public snapshot.

The classic Tier-1 arithmetic coder and coefficient-context implementation in
`emuella-j2k-tier1` was subsequently rewritten as project-authored Rust from
the normative ISO/IEC 15444-1:2024 model. It does not retain codec-derived MQ
encoder/decoder control flow, packed probability contexts, coefficient-state
bit layouts, or context lookup tables. The exact standard clauses, controlled
retrieval revisions, implementation choices, and qualification evidence are
recorded in `docs/tier1-implementation.md`.

OpenJPH-derived material is not relicensed as Apache-2.0. Its exact files,
upstream revision, hashes, licence, and modification summary are recorded in
`THIRD_PARTY.md` and `LICENSES/OpenJPH-BSD-2-Clause.txt`.

That OpenJPH file set is the repository's complete approved source-derived JPEG
2000 implementation exception. All other JPEG 2000 implementation material is
governed as project-authored under `CONTRIBUTING.md`. Any proposal to expand or
change the OpenJPH-derived file set or its pinned inputs requires prior explicit
human approval and a new copyright, licence, provenance, and architectural
review.

Standards citations identify the normative authority behind implementation
decisions without redistributing specification content. Public project
artefacts may preserve exact interoperability identifiers and cite standards
locations, but do not reproduce or closely transcribe specification prose,
page transcriptions, tables, figures, diagrams, examples, images, or rendered
equations.

Historical conformance, interoperability, and benchmark data is intentionally
absent. Optional external corpus integration belongs in the separately
versioned `emuella-testdata` catalogue and must retain each pack's own terms.
