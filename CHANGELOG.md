# UNRELEASED

- Use maintained `serde_test2` readable/compact token coverage for the
  parsing-options Serde contracts and remove the prior advisory exception.

# 1.0.0-rc.1

This is an unpublished release candidate for the 1.0 API. It is not yet
available on crates.io.

## Highlights

- Replace the `Either` parse result with the crate-owned
  `ParseEmailAddrOutput::{Original,Normalized}` enum. Strict parsing uses
  `StrictParseOutput`; relaxed parsing keeps `RelaxedParseOutput` as an alias.
  Results preserve the original storage whenever normalization leaves the
  emitted bytes unchanged, including canonical A-labels and SMTPUTF8
  local-parts; changed representations carry an already validated
  `EmailAddr<Buffer, P>`.
- Validate IDNA A-labels through the allocating `alloc`/`std` paths while
  retaining structural-only ASCII helpers for no-alloc callers.
- Make email length errors follow the validated component that failed, with
  a final address-length error after successful local/domain validation.
- Define representation-sensitive equality, ordering, and hashing for the
  built-in string, byte, and `Buffer` storage forms.
- Define the Serde compatibility boundary: human-readable address and policy
  names are stable, binary representations are non-stable and supported only
  for current-version round trips, and relaxed addresses plus parts are
  serialize-only.
- Remove the undocumented implicit Cargo `idna` feature; downstream users
  needing IDNA should select `alloc` or `std` explicitly.
- The Cargo feature `arbitrary` now implies `alloc`, so invariant-preserving
  generators are available whenever `arbitrary` is selected.

## Migration from 0.1.x

- Code matching `EmailAddr::try_from_str` or `try_from_bytes` results must
  migrate from `Either` to `ParseEmailAddrOutput::{Original,Normalized}`.
  `Original` preserves the caller's storage when the emitted bytes are
  unchanged; `Normalized` carries `EmailAddr<Buffer, P>`.
- `EmailAddr<S, Relax>` remains the explicit policy type for custom parsing
  options. Relaxed values and `LocalPart`/`DomainPart` values are
  serialize-only under Serde; strict supported address storage remains
  deserializable where the selected features permit validation.
- Select `alloc` or `std` when deserializing IDNA-normalized addresses. A
  serde-only/no-alloc build accepts structural ASCII forms and rejects A-label
  domains that require IDNA validation.
- The published `zeroize` feature and API from 0.1.1 are removed in RC1.
  In-place wiping leaves validated wrappers invalid, and safe accessors can
  panic afterward. Stop enabling `zeroize`; if owned raw storage must be
  wiped, extract it with `into_inner` only after the value reaches end of use,
  or use separate drop-time secret storage.
- The 1.0 API keeps the dual MIT/Apache-2.0 licensing choice and documents
  storage-defined trait semantics explicitly.

# 0.1.1 - 2026-06-30

- Publish `zeroize` support for `Buffer`, `EmailAddr`, `LocalPart`, and
  `DomainPart` through the opt-in feature and storage bounds.
- Move the crate to edition 2024 while retaining the minimum supported Rust
  version of 1.86.
- Add crates.io categories and keywords, and expand the documentation of the
  zero-copy storage model.
- Loosen the `smol_str` dependency requirement from exact `0.3.2` to the
  compatible `0.3` series.
- Refresh documentation, feature metadata, coverage tests, and regression
  tests.

# 0.1.0

- Initial release of validated `EmailAddr`, `LocalPart`, `DomainPart`, and
  stack `Buffer` types.
- Support RFC-oriented mailbox parsing, ASCII validators, IDNA domain
  normalization, address literals, generic storage, and no-std/no-alloc use.
- Add `serde_core` support for validated addresses and supported storage forms.
- Add policy-driven parsing through `Options` and the explicit `Relax` type,
  including SMTPUTF8, domain-literal, Unicode-domain, and length policies.
- Add `clap` support for parsing options and policy values.
- Add optional storage backends and the associated feature, CI, coverage, and
  validation-test matrix.
