# Contributing

Contributions are welcome while the public API is being stabilized.

Before submitting a change:

1. Confirm that every added source, fixture, generated table, and document has
   clear redistribution rights.
2. Keep third-party and restricted test data out of this repository; integrate
   it through `emuella-testdata` instead.
3. Commit the candidate source, then run `sh scripts/check.sh` from a clean
   primary or linked Git checkout.
4. Include provenance and licence updates whenever generated or third-party
   material changes.

Unless explicitly stated otherwise, intentionally submitted contributions are
provided under Apache-2.0 as described by section 5 of the licence.

## Canonical verification

Use focused Cargo or Python tests while editing. The complete local gate,
`sh scripts/check.sh`, verifies committed source: it refuses staged or unstaged
tracked changes and non-ignored untracked files instead of silently checking an
older revision. Ignored private overlays and build caches may remain in the
checkout; they are not exported.

The entry point reports the full commit and tree identities, reads the complete
tree directly from Git objects, and runs all checks in a disposable source
export without Git metadata. `export-ignore`, `export-subst` and checkout
filters cannot silently change that source. Working source must match the
committed bytes and executable modes, including files marked assume-unchanged
or skip-worktree. Links, submodules and unsafe paths are refused.

Python 3.11 or later, Git, the pinned Rust toolchain and `cargo-deny` must be
available. By default, the wrapper allocates a disposable child in the system
temporary directory. To choose an existing parent outside the checkout:

```sh
EMUELLA_CHECK_TMPDIR=/path/to/check-scratch sh scripts/check.sh
```

The child contains separate source, Cargo build and temporary-output
directories. The wrapper overrides `CARGO_TARGET_DIR` for this run; existing
checkout build caches are neither copied nor reused. Ordinary Cargo download
caches remain available outside the export. Only the allocated child is
removed on normal completion or a handled failure; its parent and unrelated
files are preserved. An abrupt process kill may leave that reported child for
manual inspection and cleanup.

A pass requires unchanged exported source and an unchanged, clean original
checkout after the checks. Keep the checkout idle for the run; this guards
against observed changes, not a hostile process changing and restoring bytes
between checks. The public-tree audit is unchanged and no hosted-CI environment
flag is needed. Direct invocation from an already unpacked source tree runs
the same checks without claiming a Git identity or discovering a parent
repository; keep its build output outside that source tree yourself.

Both the complete local gate and hosted CI run the existing native-plane and
JP2-presentation tests with the core parallel feature. See
[`docs/testing.md`](docs/testing.md#canonical-local-gate) for the focused command.

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
