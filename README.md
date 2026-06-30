<div align="center">
<h1>emailaddr</h1>
</div>
<div align="center">

Type-safe, validated email addresses for Rust.

[<img alt="github" src="https://img.shields.io/badge/github-al8n/emailaddr-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
[<img alt="Build" src="https://img.shields.io/github/actions/workflow/status/al8n/emailaddr/ci.yml?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-emailaddr-66c2a5?style=for-the-badge" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/emailaddr?style=for-the-badge" height="22">][crates-url]

</div>

## Overview

`emailaddr` provides validated email address types with a storage model similar
to `hostaddr`:

- `EmailAddr<S>` stores a validated full address using caller-chosen storage.
- `LocalPart<S>` validates dot-atom and quoted-string local-parts.
- `DomainPart<S>` validates RFC 5321 domain-parts and bracketed address
  literals.
- `Buffer` stores a full validated address on the stack for `no_std` /
  no-allocation use.

With the default `std` feature, SMTPUTF8 local-parts are accepted and Unicode
domain-parts are normalized to IDNA/punycode. The `verify_ascii_*` helpers
remain strict ASCII validators.

## Installation

```toml
[dependencies]
emailaddr = "0.1"
```

## Quick Start

```rust
use emailaddr::EmailAddr;

let addr: EmailAddr<String> = "user.name@example.com".parse().unwrap();
assert_eq!(addr.local_part().as_inner(), &"user.name");
assert_eq!(addr.domain_part().as_inner(), &"example.com");

let quoted = EmailAddr::try_from_ascii_str("\"user name\"@example.com").unwrap();
assert_eq!(quoted.local_part().as_inner(), &"\"user name\"");

let idn: EmailAddr<String> = "user@测试.中国".parse().unwrap();
assert_eq!(idn.as_str(), "user@xn--0zwm56d.xn--fiqs8s");

let smtp_utf8: EmailAddr<String> = "用户@example.com".parse().unwrap();
assert_eq!(smtp_utf8.local_part().as_inner(), &"用户");
```

## Storage

```rust
use emailaddr::{Buffer, EmailAddr};
use std::sync::Arc;

let owned: EmailAddr<String> = "user@example.com".parse().unwrap();
let shared: EmailAddr<Arc<str>> = EmailAddr::try_from("user@example.com").unwrap();
let bytes: EmailAddr<Vec<u8>> = EmailAddr::try_from(b"user@example.com".as_slice()).unwrap();
let stack: EmailAddr<Buffer> = EmailAddr::try_from("user@example.com").unwrap();

assert_eq!(owned.as_str(), shared.as_str());
assert_eq!(bytes.as_bytes(), stack.as_bytes());
```

## Validation Helpers

```rust
use emailaddr::{
    verify_ascii_domain_part,
    verify_ascii_email_addr,
    verify_ascii_local_part,
    verify_email_addr,
};

assert!(verify_ascii_email_addr(b"user@example.com").is_ok());
assert!(verify_email_addr("user@测试.中国".as_bytes()).is_ok());
assert!(verify_email_addr("用户@example.com".as_bytes()).is_ok());
assert!(verify_ascii_local_part(b"user.name").is_ok());
assert!(verify_ascii_domain_part(b"[IPv6:::1]").is_ok());

assert!(verify_ascii_email_addr(b"user..name@example.com").is_err());
assert!(verify_ascii_email_addr(b"user@example_com").is_err());
assert!(verify_ascii_email_addr(b"user@example.com.").is_err());
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` (default) | Standard library support and IDNA domain normalization |
| `alloc` | Allocation support without `std`, including IDNA domain normalization |
| `loom` | CI compatibility feature |
| `tarpaulin` | Coverage-build compatibility feature |

## RFC Coverage

`emailaddr` validates the address forms relevant to an address value:

- RFC 5321 SMTP mailbox syntax and length limits
- RFC 5322 `addr-spec` local-part/domain token grammar where it overlaps with
  mailbox addresses
- RFC 6531 SMTPUTF8 local-parts
- RFC 6532 UTF-8 header character model for address tokens
- RFC 1123 host label digit relaxation for domains
- RFC 4291 IPv6 address literals
- RFC 5890 IDNA domain normalization
- RFC 3629 UTF-8 validity for SMTPUTF8 input
- RFC 5234 ABNF-derived parser behavior
- RFC 3696 application-level email length guidance

This crate does not parse full message headers, display names, groups, comments,
or SMTP commands; it validates and normalizes address values.

#### License

`emailaddr` is under the terms of both the MIT license and the Apache License
(Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2026 Al Liu.

[Github-url]: https://github.com/al8n/emailaddr/
[CI-url]: https://github.com/al8n/emailaddr/actions/workflows/ci.yml
[doc-url]: https://docs.rs/emailaddr
[crates-url]: https://crates.io/crates/emailaddr
