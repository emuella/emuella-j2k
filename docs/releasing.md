# Release preparation

A release candidate must be built from one reviewed commit. Before publishing:

1. Run `sh scripts/check.sh` from the clean candidate checkout and retain its
   reported commit/tree identities. The [canonical gate](../CONTRIBUTING.md#canonical-verification)
   verifies a disposable committed-source export. The public-tree audit accepts only reviewed UTF-8
   source/text formats and named legal files; it rejects unknown suffixes,
   binary content, links, and non-regular files by default.
2. Confirm package versions, included-file lists, licences, notices, and
   third-party provenance.
3. Run `cargo package --workspace --locked`, then run
   `python3 scripts/check-package-contents.py`. The checker applies the same
   fail-closed content policy to every regular member of every completed
   `.crate` archive. Do not add `--no-verify`: Cargo's default verification
   extracts and builds the generated package, checking that it can be built
   from the packaged source rather than the workspace checkout.
4. Run `sh scripts/check-python-distributions.sh` and
   `sh scripts/package-cli-binary.sh`. The Python and CLI inspectors permit
   only their reviewed compiled payload paths, and only when the archived
   bytes have the same SHA-256 as the independently retained build output.
5. Exercise the packaged libraries in a clean temporary consumer.
6. Preserve artifact hashes, member manifests, binary-member hashes, and
   metadata evidence, then obtain explicit publication authority.

## Cargo archive staging versus release qualification

The current release process has only the verified route above. Neither the
release checklist nor `.github/workflows/release-dry-run.yml` invokes
`cargo package --no-verify`, and no local-registry staging tool is currently
part of this repository. The flag appears below only to define the restricted
status of a possible future staging step.

If interdependent workspace packages later require bootstrapping through a
temporary local registry, keep these as two visibly separate stages:

1. **Local-registry archive generation only.** `cargo package --no-verify` may
   be used only to produce inputs for the temporary registry. Put those
   archives in a staging-only directory; do not copy them into release evidence
   and do not publish them.
2. **Mandatory release qualification.** Package each crate in dependency order
   against the temporary registry without `--no-verify`. For every resulting
   archive, inspect its member list and exact legal-file inventory, unpack it,
   build it, run its applicable tests or clean packaged-consumer tests, and
   retain the qualified archive's hash as release evidence.

Success in stage 1 establishes only that an archive was generated. It does not
establish that the packaged source is self-contained, builds, passes tests, or
has the required legal files, and is never sufficient authority to publish.

An otherwise rejected source-tree payload requires an exact repository-relative
entry and expected SHA-256 in `PUBLIC_TREE_HASH_EXCEPTIONS` in
`scripts/audit-public-tree.py`. Adding or changing an exception is a policy
change that must be reviewed with the payload. Do not broaden the suffix or
basename allowlists to admit one exceptional binary.

This repository contains automation for validation and package dry runs. It
does not grant authority to publish to crates.io, PyPI, or another registry.
