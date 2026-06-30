/// The maximum local-part length, in bytes.
pub const MAX_LOCAL_PART_LENGTH: usize = 64;

/// The provided input is not a syntactically valid email local-part.
#[derive(Debug, Clone, Copy, Eq, PartialEq, thiserror::Error)]
#[error("{}", self.as_str())]
pub struct ParseLocalPartError(pub(crate) ());

impl ParseLocalPartError {
  /// Returns the error message.
  #[cfg_attr(not(coverage), inline(always))]
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
#[repr(transparent)]
pub struct LocalPart<S: ?Sized = str>(pub(crate) S);

impl<S: ?Sized> core::fmt::Display for LocalPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.0.as_ref())
  }
}

#[cfg(feature = "zeroize")]
#[cfg_attr(docsrs, doc(cfg(feature = "zeroize")))]
impl<S: ?Sized> zeroize::Zeroize for LocalPart<S>
where
  S: zeroize::Zeroize,
{
  #[inline]
  fn zeroize(&mut self) {
    zeroize::Zeroize::zeroize(&mut self.0);
  }
}

impl<S: ?Sized> LocalPart<S> {
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

  /// Converts from `&LocalPart<S>` to `LocalPart<&S>`.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_ref(&self) -> LocalPart<&S> {
    LocalPart(&self.0)
  }

  #[cfg_attr(not(coverage), inline(always))]
  const fn ref_cast(input: &S) -> &Self {
    // SAFETY: LocalPart<S> is #[repr(transparent)] over S, so references to
    // S and LocalPart<S> have the same layout and metadata, including for DSTs.
    unsafe { &*(input as *const S as *const Self) }
  }
}

impl<S> LocalPart<&S> {
  /// Copies the referenced local-part storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn copied(self) -> LocalPart<S>
  where
    S: Copy,
  {
    LocalPart(*self.0)
  }

  /// Clones the referenced local-part storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn cloned(self) -> LocalPart<S>
  where
    S: Clone,
  {
    LocalPart(self.0.clone())
  }
}

impl<S: ?Sized> core::borrow::Borrow<S> for LocalPart<S> {
  #[cfg_attr(not(coverage), inline(always))]
  fn borrow(&self) -> &S {
    &self.0
  }
}

impl<S: ?Sized> AsRef<str> for LocalPart<S>
where
  S: AsRef<str>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl<S: ?Sized> AsRef<[u8]> for LocalPart<S>
where
  S: AsRef<[u8]>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl LocalPart<str> {
  /// Validates an email local-part and returns it as a borrowed DST.
  ///
  /// This accepts the ASCII local-part syntax from RFC 5321 and the SMTPUTF8
  /// extensions from RFC 6531.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_str(input: &str) -> Result<&Self, ParseLocalPartError> {
    verify_local_part(input.as_bytes())?;
    Ok(Self::ref_cast(input))
  }

  #[cfg_attr(not(coverage), inline(always))]
  pub(crate) const fn from_valid_str(input: &str) -> &Self {
    Self::ref_cast(input)
  }

  /// Validates an ASCII local-part and returns it as a borrowed DST.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn try_from_ascii_str(input: &str) -> Result<&Self, ParseLocalPartError> {
    match verify_ascii_local_part(input.as_bytes()) {
      Ok(()) => Ok(Self::ref_cast(input)),
      Err(err) => Err(err),
    }
  }

  /// Converts the local-part to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes(&self) -> &LocalPart<[u8]> {
    LocalPart::<[u8]>::ref_cast(self.0.as_bytes())
  }
}

impl LocalPart<[u8]> {
  /// Validates an email local-part and returns it as borrowed bytes.
  ///
  /// This accepts the ASCII local-part syntax from RFC 5321 and the SMTPUTF8
  /// extensions from RFC 6531.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_bytes(input: &[u8]) -> Result<&Self, ParseLocalPartError> {
    verify_local_part(input)?;
    Ok(Self::ref_cast(input))
  }

  /// Validates an ASCII local-part and returns it as borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn try_from_ascii_bytes(input: &[u8]) -> Result<&Self, ParseLocalPartError> {
    match verify_ascii_local_part(input) {
      Ok(()) => Ok(Self::ref_cast(input)),
      Err(err) => Err(err),
    }
  }

  /// Converts the local-part to a borrowed string.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str(&self) -> &LocalPart<str> {
    let input = core::str::from_utf8(&self.0).expect("validated local-parts are valid UTF-8");
    LocalPart::<str>::ref_cast(input)
  }
}

impl<'a> LocalPart<&'a str> {
  /// Converts the local-part to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes(&self) -> LocalPart<&'a [u8]> {
    LocalPart(self.0.as_bytes())
  }
}

impl<'a> LocalPart<&'a [u8]> {
  /// Converts the local-part to a borrowed string.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str(&self) -> LocalPart<&'a str> {
    let input = core::str::from_utf8(self.0).expect("validated local-parts are valid UTF-8");
    LocalPart(input)
  }
}

impl<'a> TryFrom<&'a str> for LocalPart<&'a str> {
  type Error = ParseLocalPartError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    LocalPart::<str>::try_from_str(input)?;
    Ok(Self(input))
  }
}

impl<'a> TryFrom<&'a [u8]> for LocalPart<&'a [u8]> {
  type Error = ParseLocalPartError;

  #[inline]
  fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
    LocalPart::<[u8]>::try_from_bytes(input)?;
    Ok(Self(input))
  }
}

/// Returns `true` if `byte` is valid in an unquoted email atom.
#[cfg_attr(not(coverage), inline(always))]
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
  verify_ascii_local_part_with_limit(input, MAX_LOCAL_PART_LENGTH)
}

pub(crate) const fn verify_ascii_local_part_with_limit(
  input: &[u8],
  max_len: usize,
) -> Result<(), ParseLocalPartError> {
  let len = input.len();
  if len == 0 || len > max_len {
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
  verify_local_part_with_limit(input, MAX_LOCAL_PART_LENGTH, true)
}

pub(crate) fn verify_local_part_with_limit(
  input: &[u8],
  max_len: usize,
  smtp_utf8: bool,
) -> Result<(), ParseLocalPartError> {
  if input.is_ascii() {
    return verify_ascii_local_part_with_limit(input, max_len);
  }

  if !smtp_utf8 {
    return Err(ParseLocalPartError(()));
  }

  let input = core::str::from_utf8(input).map_err(|_| ParseLocalPartError(()))?;
  let bytes = input.as_bytes();
  let len = bytes.len();
  if len == 0 || len > max_len {
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
