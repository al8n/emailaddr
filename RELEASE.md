# Release checklist

This checklist is intentionally repository-local. Publishing still requires a
maintainer to review the final commit, tag, and registry release settings.

## Scope and API

- [ ] Confirm the package version and release notes are finalized.
- [ ] Review the public API and semver report against the previous release.
- [ ] For promotions, compare semver against the latest stable release and the
  exact release-candidate tag being promoted.
- [ ] Confirm parser, storage, serde, IDNA, and no-alloc contracts are documented.
- [ ] Confirm the release candidate has completed its soak period and all
  release-blocking issues are closed or explicitly accepted.
- [ ] Confirm the copyright owner and year in `LICENSE-MIT`: Al Liu, 2026.
- [ ] Confirm no license or API-semantic changes are included accidentally.
- [ ] Confirm the crates.io trusted publisher matches this repository, release
  workflow, and intended publishing environment.
- [ ] Confirm the GitHub `crates-io` environment name and protection rules.
- [ ] Confirm `main` and release-tag branch protection before creating the release tag.

## Verification

- [ ] Run formatting, default/all-feature tests, doctests, and examples.
- [ ] Run no-default, `serde`, `alloc+serde`, arbitrary, and quickcheck matrices.
- [ ] Run MSRV 1.86 checks and the feature-powerset/clippy jobs.
- [ ] Run `cargo deny --all-features check advisories` and review ignored advisories.
- [ ] Run semver checks, package verification, and a publish dry run.
- [ ] Validate the generated package contents and README examples.

## Release and follow-up

- [ ] Review the changelog, README, docs, and package metadata one final time.
- [ ] Before creating the release tag, verify that the target crate version,
  Git tag, and GitHub Release are absent.
- [ ] Run `workflow_dispatch` with `tag=v<package-version>` to validate the
  pipeline; confirm it does not enter the crates-io environment, request OIDC
  credentials, publish, or create a GitHub release.
- [ ] Create the release tag only from the reviewed commit.
- [ ] Publish through the approved registry/release process.
- [ ] Verify the published checksum and OIDC provenance attestation.
- [ ] Verify docs.rs and registry metadata after publication.
- [ ] Create and verify the corresponding GitHub Release object.
- [ ] Announce the release and record any follow-up issues separately.
