# Documentation route helper

Copyright 2026 Rob Christie. The newly authored files in this directory are
licensed under [Apache-2.0](LICENSE). This licence applies only to this directory;
it grants no licence to other material in the source repository.

This optional Python standard-library helper observes Git scope and returns
`full` or `docs`. It runs no product checks and interprets no event, environment,
label or commit message as permission to select documentation verification.
Full verification remains a valid setup without adopting this helper. Require
Python 3.11+ and Git supporting `--no-lazy-fetch`.

## Product integration

Author and review a small product policy before enabling a reduced route:

```json
{"schema_version": 1, "eligible_paths": ["README.md", "docs/getting-started.md"]}
```

Every entry is an exact UTF-8 relative filename ending in `.md`, `.rst`, `.txt`
or `.adoc`. There are no extension-wide rules or globs. No policy, or an empty
policy, selects full. Products must keep sensitive contracts, legal/evidence
records, instruction files, generated content, source examples and verification
wiring out of this allowlist, and validate their supported prose surfaces.
The policy does not identify commands or verification obligations.

Import `documentation_route` from the vendored directory and call:

```python
verdict = documentation_route.classify(
    repository_root,
    candidate_commit=actual_head_commit,
    base_commit=validated_local_base_commit,
    policy_path="tools/documentation-policy.json",
)
record = verdict.to_dict()
```

`candidate_commit` must be a full lowercase hexadecimal commit ID equal to
actual HEAD. Invalid candidate binding, malformed current policy or unavailable
helper integrity raises `RouteError`; callers fail the operation. An absent,
invalid, missing, unrelated or shallow comparison selects full. Only a locally
available full commit ID that is an ancestor is a usable comparison. The policy
blob must exist at that base and be byte-identical to the candidate policy.
A policy addition or widening therefore cannot qualify its own edit as docs.
Staged and working policy changes also select full; malformed current policy
fails even when another change already requires full.

The frozen `Verdict` exposes the same keys as `to_dict()`, whose schema version
is 1. It records `route`, useful `reasons`, exact `candidate_commit` and
`candidate_tree`, resolved `base_commit` and `base_tree` (nullable),
`helper_version`, `helper_sha256`, `policy` and `changes`. Policy provenance has
`schema_version`, `path`, `sha256`, `base_sha256` and `eligible_paths`. Each
change has `layer`, `status`, `old_path`, `new_path`, `old_mode` and `new_mode`;
the absent side of additions/deletions is null. Layers are `committed`,
`staged`, `unstaged` and `untracked`. An incomplete scope observation is reported
in the reasons and selects full; available changes are diagnostic, not a claim
of complete observation. A resolved base may still be unusable; read the reasons.

The helper reads NUL-delimited raw diffs without rename inference from base to
candidate, candidate to index, and index to working tree, plus non-ignored
untracked files. It checks each independently, including staged edits reversed
in the working tree. Transitions inspect both sides. Only regular non-executable
files and add/modify/delete observations can select docs. Symlinks, modes,
hidden index flags, sparse checkout, unmerged entries, incomplete observations
and unsupported filename encoding select full. Odd valid UTF-8 filenames retain
their bytes through decoding and JSON escaping. Every `.rs` change selects full,
including inline comments. Policy, workflow and the runtime helper directory
are excluded. When vendored within the repository, helper and VERSION bytes
must match the candidate. Git redirect environment variables are removed,
replace objects, optional writes and lazy fetching are disabled. This path
helper is not a security sandbox or an atomic filesystem snapshot.

The product wrapper computes the verdict in process and runs its fixed selected
group. It fails if that group fails. Canonical delivery still refuses dirty
source, binds the verdict before exact export, validates source stability and
performs its existing integrity checks. Persisted contexts require product-owned
validation or recomputation against the exact source; an unbound archive defaults
to full. Focused dirty authoring feedback makes no canonical delivery claim.

The product wrapper owns event semantics. A PR comparison must bind actual head
checkout or recognise the synthetic merge and its head/base topology; passing
an event head SHA while a different commit is checked out fails identity binding.
A push compares the actual merged revision against the event's prior revision.
Missing or unsupported event evidence selects full. Every local, PR and actual
post-merge environment makes a new independent decision, with its own available
history and candidate binding. No prior verdict authorises a later environment.

## Explicit adoption and update

Select a reviewed, immutable source commit available in a local source checkout.
No runtime network or live source dependency is introduced. The operation copies
only these files: `documentation_route.py`, `adopt.py`, `VERSION`, `LICENSE`,
`README.md`, `test_documentation_route.py` and `test_adopt.py`. It records the
owner commit, payload version and per-file SHA-256 in `.documentation-route.json`.
Use the adopter from that reviewed source version. Inspect a dry run first:

```sh
python3 SOURCE/public-safe/documentation-route/adopt.py \
  --source SOURCE --revision FULL_REVIEWED_COMMIT --version 1.0.0 \
  --target PRODUCT/tools/documentation-route --dry-run > /tmp/doc-route-plan.json
cat /tmp/doc-route-plan.json
python3 SOURCE/public-safe/documentation-route/adopt.py \
  --source SOURCE --revision FULL_REVIEWED_COMMIT --version 1.0.0 \
  --target PRODUCT/tools/documentation-route --apply /tmp/doc-route-plan.json
```

The same operation updates a managed helper. A receipt must exactly match a new
write-free plan, including source revision/version, destination and prior hashes.
Initial files may be absent or already match the requested content. Updates
require the current bytes to match the requested or prior managed hash. Managed
deletions, mode drift, unknown conflicts, malformed or altered manifests and
symlinked destinations stop before writes. Prior manifest hashes are checked
against their immutable source revision, which must also be locally available.
Files use mode 0644. Product policy, wrappers, workflows and unknown authored
files are untouched. There is no replacement flag, fleet update, generated
workflow or implicit adoption. Concurrent filesystem mutation is outside this
small local operation's boundary; stop other writers before applying. Files are
written atomically one at a time, so interrupted I/O can require inspection and
a new dry run.

Commit the explicit adoption diff through the product's reviewed mechanism and
retain the manifest. Do not pin provisional or unreviewed source revisions.
Retain the licence and tests when copying to a public product; no other private
source material is included in this directory's allowlist.

Run all shipped regressions without a source checkout or personal tooling:

```sh
python3 -m unittest discover -s PRODUCT/tools/documentation-route -p 'test_*.py' -v
```
