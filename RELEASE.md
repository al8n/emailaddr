# Release checklist

This checklist is intentionally repository-local. Publishing still requires a
maintainer to review the final commit, tag, and registry release settings.

## Scope and API

- [ ] Confirm the `1.0.0` version and release notes are finalized.
- [ ] Review the public API and semver report against the previous release.
- [ ] Confirm parser, storage, serde, IDNA, and no-alloc contracts are documented.
- [ ] Confirm no license or API-semantic changes are included accidentally.
- [ ] Confirm the crates.io trusted publisher matches this repository, release
  workflow, and intended publishing environment.
- [ ] Confirm the GitHub `crates-io` environment name and protection rules.
- [ ] Confirm `main` and release-tag branch protection before the first `1.0.0`
  release.

## Verification

- [ ] Run formatting, default/all-feature tests, doctests, and examples.
- [ ] Run no-default, `serde`, `alloc+serde`, arbitrary, and quickcheck matrices.
- [ ] Run MSRV 1.86 checks and the feature-powerset/clippy jobs.
- [ ] Run `cargo deny --all-features check advisories` and review ignored advisories.
- [ ] Run semver checks, package verification, and a publish dry run.
- [ ] Validate the generated package contents and README examples.

## Release and follow-up

- [ ] Review the changelog, README, docs, and package metadata one final time.
- [ ] Run `workflow_dispatch` in dry-run mode and confirm publish and release
  steps are skipped.
- [ ] Create the release tag only from the reviewed commit.
- [ ] Publish through the approved registry/release process.
- [ ] Verify the published checksum and OIDC provenance attestation.
- [ ] Verify docs.rs and registry metadata after publication.
- [ ] Create and verify the corresponding GitHub Release object.
- [ ] Announce the release and record any follow-up issues separately.
