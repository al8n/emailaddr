# UNRELEASED

## 1.0 readiness

- Remove the zeroize integration because wiping a validated wrapper leaves its
  value in a state that violates the type's invariant. Drop or reassign the
  value instead of mutating it into an invalid state.

# 0.1.0

FEATURES

- Implement validated `EmailAddr`, `LocalPart`, `DomainPart`, and stack `Buffer`.
