# Release checklist

This checklist is intentionally repository-local. Publishing still requires a
maintainer to review the final commit, tag, and registry release settings.

## Scope and API

- [ ] Confirm the `1.0.0` version and release notes are finalized.
- [ ] Review the public API and semver report against the previous release.
- [ ] Confirm parser, storage, serde, IDNA, and no-alloc contracts are documented.
- [ ] Confirm no license or API-semantic changes are included accidentally.

## Verification

- [ ] Run formatting, default/all-feature tests, doctests, and examples.
- [ ] Run no-default, `serde`, `alloc+serde`, arbitrary, and quickcheck matrices.
- [ ] Run MSRV 1.86 checks and the feature-powerset/clippy jobs.
- [ ] Run `cargo deny --all-features check advisories` and review ignored advisories.
- [ ] Run semver checks, package verification, and a publish dry run.
- [ ] Validate the generated package contents and README examples.

## Release and follow-up

- [ ] Review the changelog, README, docs, and package metadata one final time.
- [ ] Create the release tag only from the reviewed commit.
- [ ] Publish through the approved registry/release process.
- [ ] Verify docs.rs and registry metadata after publication.
- [ ] Announce the release and record any follow-up issues separately.
