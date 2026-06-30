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
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    "invalid email domain-part"
  }
}

/// A validated email domain-part.
///
/// The domain-part may be either an RFC 5321 `Domain` such as `example.com`, or
/// an address literal such as `[127.0.0.1]` or `[IPv6:::1]`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DomainPart<S>(pub(crate) S);

impl<S> fmt::Display for DomainPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.0.as_ref())
  }
}

impl<S> DomainPart<S> {
  /// Returns the inner storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn into_inner(self) -> S {
    self.0
  }

  /// Returns a reference to the inner storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_inner(&self) -> &S {
    &self.0
  }

  /// Converts from `&DomainPart<S>` to `DomainPart<&S>`.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_ref(&self) -> DomainPart<&S> {
    DomainPart(&self.0)
  }
}

impl<S> DomainPart<&S> {
  /// Copies the referenced domain-part storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn copied(self) -> DomainPart<S>
  where
    S: Copy,
  {
    DomainPart(*self.0)
  }

  /// Clones the referenced domain-part storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn cloned(self) -> DomainPart<S>
  where
    S: Clone,
  {
    DomainPart(self.0.clone())
  }
}

impl<S> AsRef<str> for DomainPart<S>
where
  S: AsRef<str>,
{
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl<S> AsRef<[u8]> for DomainPart<S>
where
  S: AsRef<[u8]>,
{
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<'a> DomainPart<&'a str> {
  /// Validates an ASCII domain-part and returns it as borrowed storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_ascii_str(input: &'a str) -> Result<Self, ParseDomainPartError> {
    verify_ascii_domain_part(input.as_bytes())?;
    Ok(Self(input))
  }

  /// Converts the domain-part to borrowed bytes.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_bytes(&self) -> DomainPart<&'a [u8]> {
    DomainPart(self.0.as_bytes())
  }
}

impl<'a> DomainPart<&'a [u8]> {
  /// Validates an ASCII domain-part and returns it as borrowed bytes.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_ascii_bytes(input: &'a [u8]) -> Result<Self, ParseDomainPartError> {
    verify_ascii_domain_part(input)?;
    Ok(Self(input))
  }

  /// Converts the domain-part to a borrowed string.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_str(&self) -> DomainPart<&'a str> {
    let input = str::from_utf8(self.0).expect("validated domain-parts are valid UTF-8");
    DomainPart(input)
  }
}

/// Verifies that `input` is a valid ASCII email domain-part.
///
/// DNS names are validated as RFC 5321 `Domain` values: dot-separated LDH
/// labels without a trailing root dot. Address literals accept bracketed IPv4,
/// bracketed IPv6, and syntactically valid general address literal forms.
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
pub const fn verify_ascii_dns_domain(input: &[u8]) -> Result<(), ParseDomainPartError> {
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
  if let Some(ipv6) = literal.strip_prefix("IPv6:") {
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
  if input.is_ascii() {
    verify_ascii_domain_part(input)?;
    return output
      .extend_from_slice(input)
      .map_err(|_| ParseDomainPartError(()));
  }

  let mut normalized = Buffer::new();
  domain_to_ascii(input, &mut normalized)?;
  verify_ascii_dns_domain(normalized.as_bytes())?;
  output
    .extend_from_slice(normalized.as_bytes())
    .map_err(|_| ParseDomainPartError(()))
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn domain_to_ascii(input: &[u8], output: &mut Buffer) -> Result<(), ParseDomainPartError> {
  use idna::{
    uts46::{ErrorPolicy, Hyphens, ProcessingSuccess, Uts46},
    AsciiDenyList,
  };

  let result = Uts46::new().process(
    input,
    AsciiDenyList::URL,
    Hyphens::Allow,
    ErrorPolicy::FailFast,
    |_, _, _| false,
    output,
    None,
  );

  match result {
    Ok(ProcessingSuccess::WroteToSink) => Ok(()),
    Ok(ProcessingSuccess::Passthrough) => output
      .extend_from_slice(input)
      .map_err(|_| ParseDomainPartError(())),
    Err(_) => Err(ParseDomainPartError(())),
  }
}

impl fmt::Write for Buffer {
  #[inline]
  fn write_str(&mut self, value: &str) -> fmt::Result {
    self
      .extend_from_slice(value.as_bytes())
      .map_err(|_| fmt::Error)
  }
}
