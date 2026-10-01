# Documentation verification

The canonical gate computes `full` or `docs` from Git objects and the reviewed
policy in `scripts/documentation-policy.json`. Eligible paths are `README.md`,
`docs/README.md` and `docs/getting-started.md`. Every Rust edit, including a doc
comment, stays full. Examples, manifests, locks, features, toolchain, helpers,
policy, checks, CI, agent instructions, legal/provenance, product contracts and
qualification evidence stay full. Mixed or uncertain scope is full. Invalid
candidate identity and source integrity fail.

## Commands and binding

Run `sh scripts/check-docs.sh` while editing. It accepts dirty Rust doc comments
and Markdown and claims no committed-tree/delivery pass. Keep Cargo output in
an ignored target or set `CARGO_TARGET_DIR` outside the source tree.
After committing, run `sh scripts/check.sh --base FULL_COMPARISON_COMMIT` from a
clean checkout. The base is a full local commit ID. Missing, shallow, unrelated
or unsupported comparison evidence selects full. Both routes retain exact
Git-object export, working-byte/mode checks, dirty refusal, before/after
original/export integrity and owned output/cleanup. Unpacked archives run full;
no environment, JSON verdict, label or message selects docs.

CI independently binds a PR head or recognised synthetic merge to its event
base. Push binds `after` to actual checkout and uses its own `before`. Missing
or unsupported event/history evidence selects full; identity disagreement
fails. Each phase recomputes the original candidate/tree/base, policy/helper
and event binding. The required job remains `test`; missing, skipped, failed
or cancelled selected results fail.

## Fixed documentation obligations

Docs and full run public-tree, package-legal and workspace-dependency policy
tests, provenance/public-content audit, binary dependency notice checks,
routing/helper regressions and the fixed group below. Rustdoc rendering uses
`--lib --no-deps --locked`; doctests are collected separately with `--doc
--locked`. The feature/target scope is default `std`, libraries with explicit
`--target host-tuple`.
Broken intra-doc links are errors; selected facade/core results must be nonzero,
with no ignored examples.

| Surface | Coverage and boundary |
|---|---|
| Facade `emuella-j2k` | At least one in-memory encode/inspect doctest executes |
| Defining `emuella-j2k-core` | At least one inspection-error doctest executes; facade re-exports do not collect it |
| Codestream/container re-exported boundaries | Documentation and doctest collection; currently zero authored doctests, explicitly reported |
| README | Exactly one `rust,no_run` block compiles; it reads a caller-supplied external image |
| Getting-started guide | Exactly one `rust` block executes project-authored facade assertions |
| Three eligible files | Deterministic local destinations, heading anchors and explicit references; same-repository `blob/main` URLs use current source |
| Eight inherited README package consumers | Locked Cargo source inventory and exact legal source bytes for facade/CLI/core/codestream/container/HT/tier1/transform |

Explicit temporary Rustdoc consumers read the exact Markdown blocks. Unexpected
counts/attributes fail. Eligible syntax is ATX headings, prose, simple inline
links, explicit reference definitions, and `rust`, `rust,no_run`, `toml` or `sh`
fences. HTML/images/wiki/shortcut links, unsupported attributes, indented code,
malformed references, local queries and escaping the source tree fail. Shell
and TOML fences are never executed. Remote availability is outside the check.
This is a bounded surface validator, not a general Markdown parser.

Source inventory does not qualify `.crate` archives. Actual archive/distribution
qualification stays in [release preparation](releasing.md). JPIP/C API use their
own README; acceleration/Python/test-support do not inherit the root README.
The existing all-member legal-source audit covers them. No publication runs.

## Full gate, cache and ownership

Common source obligations are in `scripts/check-source.sh`; full runtime
obligations are in `scripts/check-full-runtime.sh`. The full local/CI superset
retains rendered Layer 2 runner tests, the bypass-batch example, all feature
and fuzz checks, the complete 264-cell lossy HT matrix, strict clippy, dependency
checks and clean C API consumers. Both Cargo target/build-directory variables
are cleared for the fresh C API target. Parallel/SIMD/no-default-feature and
cross-target documentation are outside this bounded documentation group.

CI creates an owned `ci-verification` sibling of `checkout`, audits its source
before cache restore and runs all selected commands against that export.
Pre-existing/symlinked scratch is refused and preserved. Only owned export/temp
output is removed. The existing action/runner cache targets remain: original
checkout and fuzz workspace metadata survive for post-job save; exported builds
use their respective ignored targets. The layout prefix is bumped. Docs may
restore; only a successful full push on `main` saves, with failure caching off.
Both Docker dependency actions use exported manifest/config paths and pinned
Rust. The compiler, runner and cache action remain unchanged.

The vendored standard-library helper is version 1.0.0. Its scoped Apache-2.0
licence, immutable source revision and seven payload SHA-256 values are in
`scripts/documentation-route/` and `.documentation-route.json`. It observes Git
layers/transitions without GitHub or command policy. Explicit reviewed
adopter dry-run/apply updates stop on byte/hash drift and never edit the product
policy/wrappers/workflow. Every update takes full verification and review.

## Forthcoming API documentation coverage

| Area | Foundation proof | Future explicit work |
|---|---|---|
| Facade/core | Builds and selected executed examples | Systematic entrypoint/options/errors/resource examples and completeness review |
| Codestream/container | Builds and explicit current zero-example counts | Parsing/metadata/regional/source/persistence examples |
| Transform/tier1/HT/acceleration | Existing full gate and source/legal checks | Select public low-level guides and executable examples |
| JPIP | Existing full gate | Separate protocol/request/cache integration guides |
| CLI/C API/Python | Full gate and distribution contracts | Separate consumer guides and feature/target examples |
| Profiles/contracts/evidence | Stay full and remain linked | Maintain bounded claims through separately authorised work |

This proves routing and bounded examples, not complete API documentation,
a speed benchmark or broader product qualification.
