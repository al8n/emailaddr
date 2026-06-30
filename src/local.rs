/// The maximum local-part length, in bytes.
pub const MAX_LOCAL_PART_LENGTH: usize = 64;

/// The provided input is not a syntactically valid email local-part.
#[derive(Debug, Clone, Copy, Eq, PartialEq, thiserror::Error)]
#[error("{}", self.as_str())]
pub struct ParseLocalPartError(pub(crate) ());

impl ParseLocalPartError {
  /// Returns the error message.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    "invalid email local-part"
  }
}

/// A validated email local-part.
///
/// This type validates the local-part grammar used by [`EmailAddr`](crate::EmailAddr):
/// dot-atom local-parts such as `user.name`, and quoted strings such as
/// `"user name"`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalPart<S>(pub(crate) S);

impl<S> core::fmt::Display for LocalPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.0.as_ref())
  }
}

impl<S> LocalPart<S> {
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

  /// Converts from `&LocalPart<S>` to `LocalPart<&S>`.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_ref(&self) -> LocalPart<&S> {
    LocalPart(&self.0)
  }
}

impl<S> LocalPart<&S> {
  /// Copies the referenced local-part storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn copied(self) -> LocalPart<S>
  where
    S: Copy,
  {
    LocalPart(*self.0)
  }

  /// Clones the referenced local-part storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn cloned(self) -> LocalPart<S>
  where
    S: Clone,
  {
    LocalPart(self.0.clone())
  }
}

impl<S> AsRef<str> for LocalPart<S>
where
  S: AsRef<str>,
{
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl<S> AsRef<[u8]> for LocalPart<S>
where
  S: AsRef<[u8]>,
{
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl<'a> LocalPart<&'a str> {
  /// Validates an email local-part and returns it as borrowed storage.
  ///
  /// This accepts the ASCII local-part syntax from RFC 5321 and the SMTPUTF8
  /// extensions from RFC 6531.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_str(input: &'a str) -> Result<Self, ParseLocalPartError> {
    verify_local_part(input.as_bytes())?;
    Ok(Self(input))
  }

  /// Validates an ASCII local-part and returns it as borrowed storage.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_ascii_str(input: &'a str) -> Result<Self, ParseLocalPartError> {
    verify_ascii_local_part(input.as_bytes())?;
    Ok(Self(input))
  }

  /// Converts the local-part to borrowed bytes.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub const fn as_bytes(&self) -> LocalPart<&'a [u8]> {
    LocalPart(self.0.as_bytes())
  }
}

impl<'a> LocalPart<&'a [u8]> {
  /// Validates an email local-part and returns it as borrowed bytes.
  ///
  /// This accepts the ASCII local-part syntax from RFC 5321 and the SMTPUTF8
  /// extensions from RFC 6531.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_bytes(input: &'a [u8]) -> Result<Self, ParseLocalPartError> {
    verify_local_part(input)?;
    Ok(Self(input))
  }

  /// Validates an ASCII local-part and returns it as borrowed bytes.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn try_from_ascii_bytes(input: &'a [u8]) -> Result<Self, ParseLocalPartError> {
    verify_ascii_local_part(input)?;
    Ok(Self(input))
  }

  /// Converts the local-part to a borrowed string.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_str(&self) -> LocalPart<&'a str> {
    let input = core::str::from_utf8(self.0).expect("validated local-parts are valid UTF-8");
    LocalPart(input)
  }
}

/// Returns `true` if `byte` is valid in an unquoted email atom.
#[cfg_attr(not(tarpaulin), inline(always))]
pub const fn is_atext(byte: u8) -> bool {
  matches!(
    byte,
    b'a'..=b'z'
      | b'A'..=b'Z'
      | b'0'..=b'9'
      | b'!'
      | b'#'
      | b'$'
      | b'%'
      | b'&'
      | b'\''
      | b'*'
      | b'+'
      | b'-'
      | b'/'
      | b'='
      | b'?'
      | b'^'
      | b'_'
      | b'`'
      | b'{'
      | b'|'
      | b'}'
      | b'~'
  )
}

/// Verifies that `input` is a valid ASCII email local-part.
///
/// This accepts dot-atom local-parts and quoted-string local-parts. It rejects
/// empty input, non-ASCII bytes, leading/trailing/consecutive dots in dot-atoms,
/// invalid quoted-pairs, and local-parts longer than 64 bytes.
pub const fn verify_ascii_local_part(input: &[u8]) -> Result<(), ParseLocalPartError> {
  let len = input.len();
  if len == 0 || len > MAX_LOCAL_PART_LENGTH {
    return Err(ParseLocalPartError(()));
  }

  if input[0] == b'"' {
    return verify_quoted(input);
  }

  verify_dot_atom(input)
}

/// Verifies that `input` is a valid email local-part.
///
/// This accepts the ASCII local-part syntax from RFC 5321 and the SMTPUTF8
/// extensions from RFC 6531. Non-ASCII input must be valid UTF-8.
pub fn verify_local_part(input: &[u8]) -> Result<(), ParseLocalPartError> {
  if input.is_ascii() {
    return verify_ascii_local_part(input);
  }

  let input = core::str::from_utf8(input).map_err(|_| ParseLocalPartError(()))?;
  let bytes = input.as_bytes();
  let len = bytes.len();
  if len == 0 || len > MAX_LOCAL_PART_LENGTH {
    return Err(ParseLocalPartError(()));
  }

  if bytes[0] == b'"' {
    return verify_quoted_utf8(bytes);
  }

  verify_dot_atom_utf8(input)
}

const fn verify_dot_atom(input: &[u8]) -> Result<(), ParseLocalPartError> {
  let len = input.len();
  let mut i = 0;
  let mut previous_dot = true;

  while i < len {
    let byte = input[i];
    if byte == b'.' {
      if previous_dot {
        return Err(ParseLocalPartError(()));
      }

      previous_dot = true;
    } else if is_atext(byte) {
      previous_dot = false;
    } else {
      return Err(ParseLocalPartError(()));
    }

    i += 1;
  }

  if previous_dot {
    return Err(ParseLocalPartError(()));
  }

  Ok(())
}

fn verify_dot_atom_utf8(input: &str) -> Result<(), ParseLocalPartError> {
  let mut previous_dot = true;

  for ch in input.chars() {
    if ch == '.' {
      if previous_dot {
        return Err(ParseLocalPartError(()));
      }

      previous_dot = true;
    } else if ch.is_ascii() {
      if !is_atext(ch as u8) {
        return Err(ParseLocalPartError(()));
      }

      previous_dot = false;
    } else {
      previous_dot = false;
    }
  }

  if previous_dot {
    return Err(ParseLocalPartError(()));
  }

  Ok(())
}

const fn verify_quoted(input: &[u8]) -> Result<(), ParseLocalPartError> {
  let len = input.len();
  if len < 2 || input[len - 1] != b'"' {
    return Err(ParseLocalPartError(()));
  }

  let mut i = 1;
  while i < len - 1 {
    let byte = input[i];
    if byte == b'\\' {
      i += 1;
      if i >= len - 1 {
        return Err(ParseLocalPartError(()));
      }

      let escaped = input[i];
      if escaped < 32 || escaped > 126 {
        return Err(ParseLocalPartError(()));
      }
    } else if byte == b'"' || byte < 32 || byte > 126 {
      return Err(ParseLocalPartError(()));
    }

    i += 1;
  }

  Ok(())
}

fn verify_quoted_utf8(input: &[u8]) -> Result<(), ParseLocalPartError> {
  let len = input.len();
  if len < 2 || input[len - 1] != b'"' {
    return Err(ParseLocalPartError(()));
  }

  let mut i = 1;
  while i < len - 1 {
    let byte = input[i];
    if byte == b'\\' {
      i += 1;
      if i >= len - 1 {
        return Err(ParseLocalPartError(()));
      }

      let escaped = input[i];
      if !(32..=126).contains(&escaped) {
        return Err(ParseLocalPartError(()));
      }
    } else if byte == b'"' || byte < 32 || byte == 127 {
      return Err(ParseLocalPartError(()));
    }

    i += 1;
  }

  Ok(())
}
