use core::{
  fmt,
  net::{Ipv4Addr, Ipv6Addr},
  str::{self, FromStr},
};

use crate::Buffer;

/// The maximum DNS domain length accepted for email domains.
pub const MAX_DOMAIN_PART_LENGTH: usize = 253;

/// The provided input is not a syntactically valid email domain-part.
#[derive(Debug, Clone, Copy, Eq, PartialEq, thiserror::Error)]
#[error("{}", self.as_str())]
pub struct ParseDomainPartError(pub(crate) ());

impl ParseDomainPartError {
  /// Returns the error message.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    "invalid email domain-part"
  }
}

/// A validated email domain-part.
///
/// The domain-part may be either an RFC 5321 `Domain` such as `example.com`, or
/// an address literal such as `[127.0.0.1]` or `[IPv6:::1]`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct DomainPart<S: ?Sized = str>(pub(crate) S);

impl<S: ?Sized> fmt::Display for DomainPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.0.as_ref())
  }
}

impl<S: ?Sized> DomainPart<S> {
  /// Returns the inner storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn into_inner(self) -> S
  where
    S: Sized,
  {
    self.0
  }

  /// Returns a reference to the inner storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_inner(&self) -> &S {
    &self.0
  }

  /// Converts from `&DomainPart<S>` to `DomainPart<&S>`.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_ref(&self) -> DomainPart<&S> {
    DomainPart(&self.0)
  }

  #[cfg_attr(not(coverage), inline(always))]
  const fn ref_cast(input: &S) -> &Self {
    // SAFETY: DomainPart<S> is #[repr(transparent)] over S, so references to
    // S and DomainPart<S> have the same layout and metadata, including for DSTs.
    unsafe { &*(input as *const S as *const Self) }
  }
}

impl<S> DomainPart<&S> {
  /// Copies the referenced domain-part storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn copied(self) -> DomainPart<S>
  where
    S: Copy,
  {
    DomainPart(*self.0)
  }

  /// Clones the referenced domain-part storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn cloned(self) -> DomainPart<S>
  where
    S: Clone,
  {
    DomainPart(self.0.clone())
  }
}

impl<S: ?Sized> core::borrow::Borrow<S> for DomainPart<S> {
  #[cfg_attr(not(coverage), inline(always))]
  fn borrow(&self) -> &S {
    &self.0
  }
}

impl<S: ?Sized> AsRef<str> for DomainPart<S>
where
  S: AsRef<str>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl<S: ?Sized> AsRef<[u8]> for DomainPart<S>
where
  S: AsRef<[u8]>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl DomainPart<str> {
  /// Validates an ASCII domain-part and returns it as a borrowed DST.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_ascii_str(input: &str) -> Result<&Self, ParseDomainPartError> {
    verify_ascii_domain_part(input.as_bytes())?;
    Ok(Self::ref_cast(input))
  }

  /// Converts the domain-part to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes(&self) -> &DomainPart<[u8]> {
    DomainPart::<[u8]>::ref_cast(self.0.as_bytes())
  }
}

impl DomainPart<[u8]> {
  /// Validates an ASCII domain-part and returns it as borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_ascii_bytes(input: &[u8]) -> Result<&Self, ParseDomainPartError> {
    verify_ascii_domain_part(input)?;
    Ok(Self::ref_cast(input))
  }

  /// Converts the domain-part to a borrowed string.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str(&self) -> &DomainPart<str> {
    let input = str::from_utf8(&self.0).expect("validated domain-parts are valid UTF-8");
    DomainPart::<str>::ref_cast(input)
  }
}

impl<'a> DomainPart<&'a str> {
  /// Converts the domain-part to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes(&self) -> DomainPart<&'a [u8]> {
    DomainPart(self.0.as_bytes())
  }
}

impl<'a> DomainPart<&'a [u8]> {
  /// Converts the domain-part to a borrowed string.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str(&self) -> DomainPart<&'a str> {
    let input = str::from_utf8(self.0).expect("validated domain-parts are valid UTF-8");
    DomainPart(input)
  }
}

impl<'a> TryFrom<&'a str> for DomainPart<&'a str> {
  type Error = ParseDomainPartError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    DomainPart::<str>::try_from_ascii_str(input)?;
    Ok(Self(input))
  }
}

impl<'a> TryFrom<&'a [u8]> for DomainPart<&'a [u8]> {
  type Error = ParseDomainPartError;

  #[inline]
  fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
    DomainPart::<[u8]>::try_from_ascii_bytes(input)?;
    Ok(Self(input))
  }
}

/// Verifies that `input` is a valid ASCII email domain-part.
///
/// DNS names are validated as RFC 5321 `Domain` values: dot-separated LDH
/// labels without a trailing root dot. DNS A-labels are IDNA-validated when
/// `alloc` or `std` is enabled and rejected otherwise. Address literals accept
/// bracketed IPv4, bracketed IPv6, and syntactically valid general address
/// literal forms.
pub fn verify_ascii_domain_part(input: &[u8]) -> Result<(), ParseDomainPartError> {
  if input.is_empty() || !input.is_ascii() {
    return Err(ParseDomainPartError(()));
  }

  if input[0] == b'[' {
    verify_domain_literal(input)
  } else {
    verify_ascii_dns_domain(input)
  }
}

/// Verifies that `input` is a valid ASCII DNS domain name.
///
/// DNS A-labels are IDNA-validated when `alloc` or `std` is enabled and
/// rejected otherwise.
pub fn verify_ascii_dns_domain(input: &[u8]) -> Result<(), ParseDomainPartError> {
  verify_ascii_dns_domain_syntax(input)?;
  verify_ascii_dns_domain_alabel_policy(input)
}

const fn verify_ascii_dns_domain_syntax(input: &[u8]) -> Result<(), ParseDomainPartError> {
  const MAX_LABEL_LENGTH: usize = 63;

  let len = input.len();
  if len == 0 || len > MAX_DOMAIN_PART_LENGTH {
    return Err(ParseDomainPartError(()));
  }

  let mut i = 0;
  let mut label_len = 0;
  let mut last_was_hyphen = false;

  while i < len {
    let ch = input[i];
    if ch == b'.' {
      if label_len == 0 || last_was_hyphen {
        return Err(ParseDomainPartError(()));
      }

      label_len = 0;
      last_was_hyphen = false;
      i += 1;
      continue;
    }

    if label_len == 0 {
      if !is_ascii_alnum(ch) {
        return Err(ParseDomainPartError(()));
      }
    } else if !is_ascii_alnum(ch) && ch != b'-' {
      return Err(ParseDomainPartError(()));
    }

    label_len += 1;
    if label_len > MAX_LABEL_LENGTH {
      return Err(ParseDomainPartError(()));
    }

    last_was_hyphen = ch == b'-';
    i += 1;
  }

  if label_len == 0 || last_was_hyphen {
    return Err(ParseDomainPartError(()));
  }

  Ok(())
}

pub(crate) fn contains_ascii_alabel(input: &[u8]) -> bool {
  if input.starts_with(b"[") {
    return false;
  }

  for label in input.split(|byte| *byte == b'.') {
    if is_ascii_alabel(label) {
      return true;
    }
  }

  false
}

fn is_ascii_alabel(label: &[u8]) -> bool {
  label.len() >= 4 && label[..4].eq_ignore_ascii_case(b"xn--")
}

fn verify_ascii_dns_domain_alabel_policy(input: &[u8]) -> Result<(), ParseDomainPartError> {
  if !contains_ascii_alabel(input) {
    return Ok(());
  }

  #[cfg(any(feature = "alloc", feature = "std"))]
  {
    let mut normalized = Buffer::new();
    domain_to_ascii(input, &mut normalized)?;
    verify_ascii_dns_domain_syntax(normalized.as_bytes())
  }

  #[cfg(not(any(feature = "alloc", feature = "std")))]
  {
    Err(ParseDomainPartError(()))
  }
}

const fn is_ascii_alnum(byte: u8) -> bool {
  matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
}

const fn verify_literal_tag(input: &[u8]) -> Result<(), ParseDomainPartError> {
  let len = input.len();
  if len == 0 {
    return Err(ParseDomainPartError(()));
  }

  if !is_ascii_alnum(input[0]) {
    return Err(ParseDomainPartError(()));
  }

  let mut i = 0;
  while i < len {
    let byte = input[i];
    if !is_ascii_alnum(byte) && byte != b'-' {
      return Err(ParseDomainPartError(()));
    }

    i += 1;
  }

  if !is_ascii_alnum(input[len - 1]) {
    return Err(ParseDomainPartError(()));
  }

  Ok(())
}

const fn verify_literal_content(input: &[u8]) -> Result<(), ParseDomainPartError> {
  let len = input.len();
  if len == 0 {
    return Err(ParseDomainPartError(()));
  }

  let mut i = 0;
  while i < len {
    let byte = input[i];
    match byte {
      33..=90 | 94..=126 => i += 1,
      _ => return Err(ParseDomainPartError(())),
    }
  }

  Ok(())
}

fn verify_general_address_literal(input: &str) -> Result<(), ParseDomainPartError> {
  let Some((tag, content)) = input.split_once(':') else {
    return Err(ParseDomainPartError(()));
  };

  verify_literal_tag(tag.as_bytes())?;
  verify_literal_content(content.as_bytes())
}

fn verify_domain_literal(input: &[u8]) -> Result<(), ParseDomainPartError> {
  if input.len() < 3 || input[input.len() - 1] != b']' {
    return Err(ParseDomainPartError(()));
  }

  let literal = str::from_utf8(&input[1..input.len() - 1]).map_err(|_| ParseDomainPartError(()))?;
  if literal.len() >= 5 && literal.as_bytes()[..5].eq_ignore_ascii_case(b"IPv6:") {
    let ipv6 = &literal[5..];
    Ipv6Addr::from_str(ipv6)
      .map(|_| ())
      .map_err(|_| ParseDomainPartError(()))
  } else if Ipv4Addr::from_str(literal).is_ok() {
    Ok(())
  } else if literal
    .as_bytes()
    .iter()
    .all(|byte| byte.is_ascii_digit() || *byte == b'.')
  {
    Ipv4Addr::from_str(literal)
      .map(|_| ())
      .map_err(|_| ParseDomainPartError(()))
  } else {
    verify_general_address_literal(literal)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
pub(crate) fn write_normalized_domain_part(
  input: &[u8],
  output: &mut Buffer,
) -> Result<(), ParseDomainPartError> {
  if input.is_ascii() && !contains_ascii_alabel(input) {
    verify_ascii_domain_part(input)?;
    return output
      .extend_from_slice(input)
      .map_err(|_| ParseDomainPartError(()));
  }

  let mut normalized = Buffer::new();
  domain_to_ascii(input, &mut normalized)?;
  verify_ascii_dns_domain_syntax(normalized.as_bytes())?;
  output
    .extend_from_slice(normalized.as_bytes())
    .map_err(|_| ParseDomainPartError(()))
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn domain_to_ascii(input: &[u8], output: &mut Buffer) -> Result<(), ParseDomainPartError> {
  let domain = str::from_utf8(input).map_err(|_| ParseDomainPartError(()))?;
  let mut wrote_label = false;

  for label in domain.split(['.', '\u{3002}', '\u{ff0e}', '\u{ff61}']) {
    if label.is_empty() {
      return Err(ParseDomainPartError(()));
    }

    if wrote_label {
      output
        .extend_from_slice(b".")
        .map_err(|_| ParseDomainPartError(()))?;
    }
    write_normalized_dns_label(label.as_bytes(), output)?;
    wrote_label = true;
  }

  if !wrote_label {
    return Err(ParseDomainPartError(()));
  }

  verify_ascii_dns_domain_syntax(output.as_bytes())?;
  verify_normalized_idna_domain(output.as_bytes())
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn write_normalized_dns_label(
  input: &[u8],
  output: &mut Buffer,
) -> Result<(), ParseDomainPartError> {
  if input.is_ascii() && !is_ascii_alabel(input) {
    verify_ascii_dns_domain_syntax(input)?;
    return output
      .extend_from_slice(input)
      .map_err(|_| ParseDomainPartError(()));
  }

  let ascii = idna::uts46::Uts46::new()
    .to_ascii(
      input,
      idna::uts46::AsciiDenyList::STD3,
      idna::uts46::Hyphens::Check,
      idna::uts46::DnsLength::Verify,
    )
    .map_err(|_| ParseDomainPartError(()))?;
  verify_ascii_dns_domain_syntax(ascii.as_bytes())?;
  output
    .extend_from_slice(ascii.as_bytes())
    .map_err(|_| ParseDomainPartError(()))
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn verify_normalized_idna_domain(input: &[u8]) -> Result<(), ParseDomainPartError> {
  let ascii = idna::uts46::Uts46::new()
    .to_ascii(
      input,
      idna::uts46::AsciiDenyList::STD3,
      idna::uts46::Hyphens::CheckFirstLast,
      idna::uts46::DnsLength::Verify,
    )
    .map_err(|_| ParseDomainPartError(()))?;
  verify_ascii_dns_domain_syntax(ascii.as_bytes())
}

impl fmt::Write for Buffer {
  #[inline]
  fn write_str(&mut self, value: &str) -> fmt::Result {
    self
      .extend_from_slice(value.as_bytes())
      .map_err(|_| fmt::Error)
  }
}
