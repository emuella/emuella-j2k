# Contributing

Contributions are welcome while the public API is being stabilized.

Before submitting a change:

1. Confirm that every added source, fixture, generated table, and document has
   clear redistribution rights.
2. Keep third-party and restricted test data out of this repository; integrate
   it through `emuella-testdata` instead.
3. Run `scripts/check.sh`.
4. Include provenance and licence updates whenever generated or third-party
   material changes.

Unless explicitly stated otherwise, intentionally submitted contributions are
provided under Apache-2.0 as described by section 5 of the licence.

## Standards and implementation provenance

The ISO/IEC standards are normative authorities, not sources of publishable
repository content. Cite an applicable standard by identifier, edition,
clause, annex, table, figure, physical page, and reviewed retrieval revision as
needed, then explain its requirements in project-authored prose. Exact
standard-defined identifiers, field names, symbols, marker values, and
normative terminology may be preserved where interoperability requires them.

Do not reproduce or closely transcribe specification prose, page
transcriptions, tables, figures, diagrams, examples, images, or rendered
equations in tracked files, source comments, tests, commit messages,
pull-request or issue text, release notes, packages, or other public project
artefacts. If exact quotation appears necessary, obtain the copyright holder's
redistribution authority and prior explicit human maintainer approval before
publishing it.

Except for the exact OpenJPH-derived files enumerated in `THIRD_PARTY.md`, all
codec source, tests, tables, constants, comments, documentation, and generated
material must be project-authored from applicable standards and independently
observed behaviour. Do not copy, translate, port, adapt, transcribe, or
structurally reproduce source code, control flow, lookup tables, constants,
comments, tests, reference output, API layouts, or generated material from
Kakadu, OpenJPEG, Grok, JasPer, JJ2000, another reference implementation, or
any other JPEG 2000 implementation.

Running another implementation as a black-box interoperability oracle is
permitted when its terms and applicable project policy authorise the exact use.
Only project-authored summaries of observed behaviour may enter public project
artefacts; do not reproduce its source, binaries, fixtures, output payloads,
verbatim diagnostics, generated files, or other artefacts. Source inspection
of another implementation must not become an input to project-authored code.

The OpenJPH exception is closed to the paths and pinned upstream inputs already
listed in `THIRD_PARTY.md`. Adding, removing, or renaming an allowlisted file,
changing its upstream source or revision, or introducing any other
source-derived implementation requires prior explicit human maintainer
approval and a new file-level copyright, licence, provenance, and architectural
review.

Reviewers must treat copied or closely transcribed standards expression,
non-allowlisted implementation-derived material, an unexplained provenance
change, or disagreement between source headers and `THIRD_PARTY.md` as
blocking.
