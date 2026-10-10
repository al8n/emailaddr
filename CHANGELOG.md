# UNRELEASED

- Replace the `Either` parse result with the crate-owned
  `ParseEmailAddrOutput::{Original,Normalized}` enum. Strict parsing uses
  `StrictParseOutput`; relaxed parsing keeps `RelaxedParseOutput` as an alias.
  Results preserve the original storage whenever normalization leaves the
  emitted bytes unchanged, including canonical A-labels and SMTPUTF8 local-
  parts; changed representations carry an already validated
  `EmailAddr<Buffer, P>`.
- Document the crate-owned parse output migration and the dual-license
  attribution update for the 1.0 API surface.
- Define the Serde compatibility boundary: human-readable address and policy
  names are stable, binary representations are non-stable and supported only
  for current-version round trips, and relaxed addresses plus parts are
  serialize-only.

## 1.0 readiness

- Remove the zeroize integration because wiping a validated wrapper leaves its
  value in a state that violates the type's invariant. Drop or reassign the
  value instead of mutating it into an invalid state.
- Make IDNA activation explicit through the `alloc` and `std` features and make
  `arbitrary` include `alloc` so its invariant-preserving generators are
  available whenever that feature is selected.
- Pin the bincode development dependency to `=1.3.3` as an unmaintained,
  dev-only roundtrip codec for the non-stable binary Serde fixture; the
  corresponding advisory rationale is recorded in `deny.toml`.

# 0.1.0

FEATURES

- Implement validated `EmailAddr`, `LocalPart`, `DomainPart`, and stack `Buffer`.
