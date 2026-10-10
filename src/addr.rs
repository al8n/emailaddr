use core::{fmt, marker::PhantomData, str};

#[cfg(any(feature = "alloc", feature = "std"))]
use std::{
  borrow::Cow,
  boxed::Box,
  rc::Rc,
  string::{String, ToString},
  sync::Arc,
  vec::Vec,
};

#[cfg(any(feature = "alloc", feature = "std"))]
use either::Either;

use crate::{
  Buffer, DomainPart, LocalPart, MAX_LOCAL_PART_LENGTH, Options, ParseDomainPartError,
  ParseLocalPartError,
};

use crate::{
  domain::{contains_ascii_alabel, write_domain_part_with_options},
  local::verify_local_part_with_limit,
};

#[cfg(any(feature = "alloc", feature = "std"))]
use crate::domain::write_normalized_domain_part;

/// The maximum email address length accepted by this crate.
pub const MAX_EMAIL_ADDR_LENGTH: usize = 254;

/// Output from relaxed parsing that may preserve the original storage or return a stack buffer.
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub type RelaxedParseOutput<S> = Either<EmailAddr<S, Relax>, EmailAddr<Buffer, Relax>>;

/// The provided input is not a syntactically valid email address.
#[derive(Debug, Clone, Eq, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum ParseEmailAddrError {
  /// The address is structurally invalid.
  #[error("invalid email address")]
  Address,
  /// The local-part is invalid.
  #[error(transparent)]
  LocalPart(#[from] ParseLocalPartError),
  /// The domain-part is invalid.
  #[error(transparent)]
  DomainPart(#[from] ParseDomainPartError),
}

impl ParseEmailAddrError {
  /// Returns the high-level error message.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Address => "invalid email address",
      Self::LocalPart(err) => err.as_str(),
      Self::DomainPart(err) => err.as_str(),
    }
  }
}

/// A type-safe, validated email address.
///
/// `EmailAddr<S>` is the strict default form and stores the whole email address
/// in the caller-selected storage type `S`. Borrowed DST storage such as
/// `EmailAddr<str>` and `EmailAddr<[u8]>` is zero-copy. Owned storage such as
/// `String`, `Arc<str>`, `Vec<u8>`, or [`Buffer`] is available through
/// `TryFrom` and `FromStr` implementations.
///
/// `EmailAddr<S, Relax>` is the opt-in relaxed policy form returned by
/// `parse_with_options`. Strict addresses can be widened into relaxed addresses,
/// but relaxed addresses must be revalidated before they can become strict.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct EmailAddr<S: ?Sized = str, P = Strict> {
  marker: PhantomData<fn() -> P>,
  pub(crate) storage: S,
}

/// Strict email address policy.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Strict;

/// Relaxed email address policy for custom parsing options.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Relax;

impl<S, P> EmailAddr<S, P> {
  #[cfg_attr(not(coverage), inline(always))]
  pub(crate) const fn from_inner(storage: S) -> Self {
    Self {
      marker: PhantomData,
      storage,
    }
  }
}

impl<S: ?Sized, P> fmt::Display for EmailAddr<S, P>
where
  S: AsRef<str>,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.storage.as_ref())
  }
}

impl<S: ?Sized, P> EmailAddr<S, P> {
  /// Returns a reference to the inner storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_inner(&self) -> &S {
    &self.storage
  }

  /// Returns the inner storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn into_inner(self) -> S
  where
    S: Sized,
  {
    self.storage
  }

  /// Converts from `&EmailAddr<S, P>` to `EmailAddr<&S, P>`.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_ref(&self) -> EmailAddr<&S, P> {
    EmailAddr::from_inner(&self.storage)
  }

  /// Converts from `EmailAddr<S, P>` to `EmailAddr<&S::Target, P>`.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_deref(&self) -> EmailAddr<&S::Target, P>
  where
    S: core::ops::Deref,
  {
    EmailAddr::from_inner(core::ops::Deref::deref(&self.storage))
  }

  /// Returns the full email address as a string slice.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str(&self) -> &str
  where
    S: AsRef<str>,
  {
    self.storage.as_ref()
  }

  /// Returns the full email address as bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_bytes(&self) -> &[u8]
  where
    S: AsRef<[u8]>,
  {
    self.storage.as_ref()
  }

  /// Returns `true` if the domain-part is an address literal.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn is_domain_literal(&self) -> bool
  where
    S: AsRef<[u8]>,
  {
    let bytes = self.storage.as_ref();
    let at = find_at(bytes).expect("validated email addresses contain @");
    bytes[at + 1..].starts_with(b"[")
  }

  #[cfg_attr(not(coverage), inline(always))]
  const fn ref_cast(input: &S) -> &Self {
    // SAFETY: EmailAddr<S, P> is #[repr(transparent)] over S, so references to
    // S and EmailAddr<S, P> have the same layout and metadata, including for DSTs.
    unsafe { &*(input as *const S as *const Self) }
  }
}

impl<S: ?Sized> EmailAddr<S, Strict> {
  /// Returns the validated local-part.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn local_part(&self) -> LocalPart<&str>
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    LocalPart::<str>::from_valid_str(&input[..at]).as_ref()
  }

  /// Returns the validated domain-part.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn domain_part(&self) -> DomainPart<&str>
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    DomainPart::<str>::from_valid_str(&input[at + 1..]).as_ref()
  }

  /// Returns the local-part and domain-part as validated borrowed values.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn parts(&self) -> (LocalPart<&str>, DomainPart<&str>)
  where
    S: AsRef<str>,
  {
    (self.local_part(), self.domain_part())
  }

  /// Returns the validated local-part as a borrowed DST wrapper.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn local_part_ref(&self) -> &LocalPart<str>
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    LocalPart::<str>::from_valid_str(&input[..at])
  }

  /// Returns the validated domain-part as a borrowed DST wrapper.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn domain_part_ref(&self) -> &DomainPart<str>
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    DomainPart::<str>::from_valid_str(&input[at + 1..])
  }

  /// Returns the local-part and domain-part as borrowed DST wrappers.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn parts_ref(&self) -> (&LocalPart<str>, &DomainPart<str>)
  where
    S: AsRef<str>,
  {
    (self.local_part_ref(), self.domain_part_ref())
  }
}

impl<S: ?Sized> EmailAddr<S, Relax> {
  /// Returns the local-part as validated by the relaxed policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn local_part(&self) -> &str
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    &input[..at]
  }

  /// Returns the domain-part as validated by the relaxed policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn domain_part(&self) -> &str
  where
    S: AsRef<str>,
  {
    let input = self.storage.as_ref();
    let at = find_at(input.as_bytes()).expect("validated email addresses contain @");
    &input[at + 1..]
  }

  /// Returns the local-part and domain-part as relaxed-policy string slices.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn parts(&self) -> (&str, &str)
  where
    S: AsRef<str>,
  {
    (self.local_part(), self.domain_part())
  }
}

impl<S> From<EmailAddr<S, Strict>> for EmailAddr<S, Relax> {
  #[cfg_attr(not(coverage), inline(always))]
  fn from(value: EmailAddr<S, Strict>) -> Self {
    Self::from_inner(value.storage)
  }
}

impl<S: ?Sized, P> core::borrow::Borrow<S> for EmailAddr<S, P> {
  #[cfg_attr(not(coverage), inline(always))]
  fn borrow(&self) -> &S {
    &self.storage
  }
}

impl<S: ?Sized, P> AsRef<str> for EmailAddr<S, P>
where
  S: AsRef<str>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &str {
    self.storage.as_ref()
  }
}

impl<S: ?Sized, P> AsRef<[u8]> for EmailAddr<S, P>
where
  S: AsRef<[u8]>,
{
  #[cfg_attr(not(coverage), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.storage.as_ref()
  }
}

impl<S: ?Sized, P> EmailAddr<&S, P> {
  /// Copies the referenced address storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn copied(self) -> EmailAddr<S, P>
  where
    S: Copy,
  {
    EmailAddr::from_inner(*self.storage)
  }

  /// Clones the referenced address storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn cloned(self) -> EmailAddr<S, P>
  where
    S: Clone,
  {
    EmailAddr::from_inner(self.storage.clone())
  }
}

impl EmailAddr<str> {
  /// Validates an ASCII email address and returns it as a borrowed DST.
  ///
  /// This method does not perform IDNA normalization. ASCII A-labels are
  /// IDNA-validated when `alloc` or `std` is enabled and rejected otherwise.
  /// Use `TryFrom<&str>` for owned storage when Unicode domain names should be
  /// converted to punycode.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_ascii_str(input: &str) -> Result<&Self, ParseEmailAddrError> {
    verify_borrowed_ascii_email_addr(input.as_bytes())?;
    Ok(Self::ref_cast(input))
  }

  /// Converts the address to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes_addr(&self) -> &EmailAddr<[u8]> {
    EmailAddr::<[u8]>::ref_cast(self.storage.as_bytes())
  }
}

impl EmailAddr<[u8]> {
  /// Validates an ASCII email address and returns it as borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn try_from_ascii_bytes(input: &[u8]) -> Result<&Self, ParseEmailAddrError> {
    verify_borrowed_ascii_email_addr(input)?;
    Ok(Self::ref_cast(input))
  }

  /// Converts the address to borrowed string storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str_addr(&self) -> &EmailAddr<str> {
    let input = str::from_utf8(&self.storage).expect("validated email addresses are valid UTF-8");
    EmailAddr::<str>::ref_cast(input)
  }
}

impl<'a> EmailAddr<&'a str> {
  /// Converts the address to borrowed bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_bytes_addr(&self) -> EmailAddr<&'a [u8]> {
    EmailAddr::from_inner(self.storage.as_bytes())
  }
}

impl<'a> EmailAddr<&'a [u8]> {
  /// Converts the address to borrowed string storage.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn as_str_addr(&self) -> EmailAddr<&'a str> {
    let input = str::from_utf8(self.storage).expect("validated email addresses are valid UTF-8");
    EmailAddr::from_inner(input)
  }
}

impl core::str::FromStr for EmailAddr<Buffer> {
  type Err = ParseEmailAddrError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Self::try_from(input)
  }
}

impl<'a> TryFrom<&'a str> for EmailAddr<&'a str> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    EmailAddr::<str>::try_from_ascii_str(input)?;
    Ok(Self::from_inner(input))
  }
}

impl<'a> TryFrom<&'a [u8]> for EmailAddr<&'a [u8]> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
    EmailAddr::<[u8]>::try_from_ascii_bytes(input)?;
    Ok(Self::from_inner(input))
  }
}

impl TryFrom<&str> for EmailAddr<Buffer> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &str) -> Result<Self, Self::Error> {
    #[cfg(any(feature = "alloc", feature = "std"))]
    {
      match EmailAddr::try_from_str(input)? {
        Either::Left(addr) => Ok(Self::from_inner(Buffer::copy_from_slice(
          addr.storage.as_bytes(),
        ))),
        Either::Right(buf) => Ok(Self::from_inner(buf)),
      }
    }

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    {
      verify_ascii_email_addr(input.as_bytes())?;
      let (_, domain) = split_email_addr(input.as_bytes())?;
      if crate::domain::contains_ascii_alabel(domain) {
        return Err(ParseEmailAddrError::DomainPart(ParseDomainPartError(())));
      }

      Ok(Self::from_inner(Buffer::copy_from_slice(input.as_bytes())))
    }
  }
}

impl TryFrom<&[u8]> for EmailAddr<Buffer> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
    #[cfg(any(feature = "alloc", feature = "std"))]
    {
      match EmailAddr::try_from_bytes(input)? {
        Either::Left(addr) => Ok(Self::from_inner(Buffer::copy_from_slice(addr.storage))),
        Either::Right(buf) => Ok(Self::from_inner(buf)),
      }
    }

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    {
      verify_ascii_email_addr(input)?;
      let (_, domain) = split_email_addr(input)?;
      if crate::domain::contains_ascii_alabel(domain) {
        return Err(ParseEmailAddrError::DomainPart(ParseDomainPartError(())));
      }

      Ok(Self::from_inner(Buffer::copy_from_slice(input)))
    }
  }
}

impl EmailAddr<Buffer, Relax> {
  /// Parses an email address with custom parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn parse_with_options(input: &str, options: Options) -> Result<Self, ParseEmailAddrError> {
    Self::parse_bytes_with_options(input.as_bytes(), options)
  }

  /// Parses an email address from bytes with custom parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub fn parse_bytes_with_options(
    input: &[u8],
    options: Options,
  ) -> Result<Self, ParseEmailAddrError> {
    let mut output = Buffer::new();
    write_email_addr_with_options(input, &mut output, options)?;
    Ok(Self::from_inner(output))
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<S> EmailAddr<S> {
  /// Parses an email address from bytes.
  ///
  /// ASCII addresses are returned using the original storage. If the domain-part
  /// contains non-ASCII Unicode, it is normalized to IDNA/punycode and returned
  /// as [`Buffer`]. SMTPUTF8 local-parts are preserved as UTF-8.
  #[inline]
  pub fn try_from_bytes(input: S) -> Result<Either<Self, Buffer>, ParseEmailAddrError>
  where
    S: AsRef<[u8]>,
  {
    let bytes = input.as_ref();
    if bytes.is_ascii() {
      verify_ascii_email_addr(bytes)?;
      let (_, domain) = split_email_addr(bytes)?;
      if !contains_ascii_alabel(domain) {
        return Ok(Either::Left(Self::from_inner(input)));
      }
    }

    let mut output = Buffer::new();
    write_normalized_email_addr(bytes, &mut output)?;
    Ok(Either::Right(output))
  }

  /// Parses an email address from a string.
  ///
  /// ASCII addresses are returned using the original storage. If the domain-part
  /// contains non-ASCII Unicode, it is normalized to IDNA/punycode and returned
  /// as [`Buffer`]. SMTPUTF8 local-parts are preserved as UTF-8.
  #[inline]
  pub fn try_from_str(input: S) -> Result<Either<Self, Buffer>, ParseEmailAddrError>
  where
    S: AsRef<str>,
  {
    let bytes = input.as_ref().as_bytes();
    if bytes.is_ascii() {
      verify_ascii_email_addr(bytes)?;
      let (_, domain) = split_email_addr(bytes)?;
      if !contains_ascii_alabel(domain) {
        return Ok(Either::Left(Self::from_inner(input)));
      }
    }

    let mut output = Buffer::new();
    write_normalized_email_addr(bytes, &mut output)?;
    Ok(Either::Right(output))
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<S> EmailAddr<S, Relax> {
  /// Parses an email address from bytes with custom parsing options.
  ///
  /// The returned [`Either::Left`] value preserves the original storage when no
  /// normalization was needed. [`Either::Right`] contains the normalized stack
  /// buffer otherwise.
  #[inline]
  pub fn try_from_bytes_with_options(
    input: S,
    options: Options,
  ) -> Result<RelaxedParseOutput<S>, ParseEmailAddrError>
  where
    S: AsRef<[u8]>,
  {
    let bytes = input.as_ref();
    let mut output = Buffer::new();
    write_email_addr_with_options(bytes, &mut output, options)?;
    if output.as_bytes() == bytes {
      Ok(Either::Left(EmailAddr::from_inner(input)))
    } else {
      Ok(Either::Right(EmailAddr::from_inner(output)))
    }
  }

  /// Parses an email address from a string with custom parsing options.
  ///
  /// The returned [`Either::Left`] value preserves the original storage when no
  /// normalization was needed. [`Either::Right`] contains the normalized stack
  /// buffer otherwise.
  #[inline]
  pub fn try_from_str_with_options(
    input: S,
    options: Options,
  ) -> Result<RelaxedParseOutput<S>, ParseEmailAddrError>
  where
    S: AsRef<str>,
  {
    let bytes = input.as_ref().as_bytes();
    let mut output = Buffer::new();
    write_email_addr_with_options(bytes, &mut output, options)?;
    if output.as_bytes() == bytes {
      Ok(Either::Left(EmailAddr::from_inner(input)))
    } else {
      Ok(Either::Right(EmailAddr::from_inner(output)))
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl EmailAddr<String, Relax> {
  /// Parses an email address with custom parsing options.
  #[inline]
  pub fn parse_with_options(input: &str, options: Options) -> Result<Self, ParseEmailAddrError> {
    let mut output = Buffer::new();
    write_email_addr_with_options(input.as_bytes(), &mut output, options)?;
    Ok(Self::from_inner(output.into()))
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl core::str::FromStr for EmailAddr<String> {
  type Err = ParseEmailAddrError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Self::try_from(input)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a str> for EmailAddr<String> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_str(input)? {
      Either::Left(addr) => Ok(Self::from_inner(addr.storage.to_string())),
      Either::Right(buf) => Ok(Self::from_inner(buf.into())),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl TryFrom<String> for EmailAddr<String> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: String) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_str(input.as_str())? {
      Either::Left(_) => Ok(Self::from_inner(input)),
      Either::Right(buf) => Ok(Self::from_inner(buf.into())),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! impl_str_storage {
  ($($ty:ty), +$(,)?) => {
    $(
      impl core::str::FromStr for EmailAddr<$ty> {
        type Err = ParseEmailAddrError;

        #[inline]
        fn from_str(input: &str) -> Result<Self, Self::Err> {
          Self::try_from(input)
        }
      }

      impl<'a> TryFrom<&'a str> for EmailAddr<$ty> {
        type Error = ParseEmailAddrError;

        #[inline]
        fn try_from(input: &'a str) -> Result<Self, Self::Error> {
          match EmailAddr::try_from_str(input)? {
            Either::Left(addr) => Ok(Self::from_inner(<$ty>::from(addr.storage))),
            Either::Right(buf) => Ok(Self::from_inner(<$ty>::from(buf.as_str()))),
          }
        }
      }
    )*
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! impl_byte_storage {
  ($($ty:ty), +$(,)?) => {
    $(
      impl core::str::FromStr for EmailAddr<$ty> {
        type Err = ParseEmailAddrError;

        #[inline]
        fn from_str(input: &str) -> Result<Self, Self::Err> {
          Self::try_from(input)
        }
      }

      impl<'a> TryFrom<&'a str> for EmailAddr<$ty> {
        type Error = ParseEmailAddrError;

        #[inline]
        fn try_from(input: &'a str) -> Result<Self, Self::Error> {
          Self::try_from(input.as_bytes())
        }
      }

      impl<'a> TryFrom<&'a [u8]> for EmailAddr<$ty> {
        type Error = ParseEmailAddrError;

        #[inline]
        fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
          match EmailAddr::try_from_bytes(input)? {
            Either::Left(addr) => Ok(Self::from_inner(<$ty>::from(addr.storage))),
            Either::Right(buf) => Ok(Self::from_inner(<$ty>::from(buf.as_bytes()))),
          }
        }
      }
    )*
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl_str_storage!(Box<str>, Rc<str>, Arc<str>);

#[cfg(feature = "smol_str_0_3")]
impl_str_storage!(smol_str_0_3::SmolStr);

#[cfg(feature = "triomphe_0_1")]
impl_str_storage!(triomphe_0_1::Arc<str>);

#[cfg(any(feature = "alloc", feature = "std"))]
impl_byte_storage!(Box<[u8]>, Rc<[u8]>, Arc<[u8]>);

#[cfg(feature = "triomphe_0_1")]
impl_byte_storage!(triomphe_0_1::Arc<[u8]>);

#[cfg(any(feature = "alloc", feature = "std"))]
impl core::str::FromStr for EmailAddr<Vec<u8>> {
  type Err = ParseEmailAddrError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Self::try_from(input)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a str> for EmailAddr<Vec<u8>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_str(input)? {
      Either::Left(addr) => Ok(Self::from_inner(addr.storage.as_bytes().to_vec())),
      Either::Right(buf) => Ok(Self::from_inner(buf.into())),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a [u8]> for EmailAddr<Vec<u8>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_bytes(input)? {
      Either::Left(addr) => Ok(Self::from_inner(addr.storage.to_vec())),
      Either::Right(buf) => Ok(Self::from_inner(buf.into())),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl TryFrom<Vec<u8>> for EmailAddr<Vec<u8>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: Vec<u8>) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_bytes(input.as_slice())? {
      Either::Left(_) => Ok(Self::from_inner(input)),
      Either::Right(buf) => Ok(Self::from_inner(buf.into())),
    }
  }
}

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  impl core::str::FromStr for EmailAddr<Bytes> {
    type Err = ParseEmailAddrError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
      Self::try_from(input)
    }
  }

  impl<'a> TryFrom<&'a str> for EmailAddr<Bytes> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_str(input)? {
        Either::Left(addr) => Ok(Self::from_inner(Bytes::copy_from_slice(
          addr.storage.as_bytes(),
        ))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl<'a> TryFrom<&'a [u8]> for EmailAddr<Bytes> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input)? {
        Either::Left(addr) => Ok(Self::from_inner(Bytes::copy_from_slice(addr.storage))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl TryFrom<Bytes> for EmailAddr<Bytes> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: Bytes) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input.as_ref())? {
        Either::Left(_) => Ok(Self::from_inner(input)),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  use tinyvec_1::TinyVec;

  impl<const N: usize> core::str::FromStr for EmailAddr<TinyVec<[u8; N]>> {
    type Err = ParseEmailAddrError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
      Self::try_from(input)
    }
  }

  impl<'a, const N: usize> TryFrom<&'a str> for EmailAddr<TinyVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_str(input)? {
        Either::Left(addr) => Ok(Self::from_inner(TinyVec::from(addr.storage.as_bytes()))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl<'a, const N: usize> TryFrom<&'a [u8]> for EmailAddr<TinyVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input)? {
        Either::Left(addr) => Ok(Self::from_inner(TinyVec::from(addr.storage))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl<const N: usize> TryFrom<TinyVec<[u8; N]>> for EmailAddr<TinyVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: TinyVec<[u8; N]>) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input.as_ref())? {
        Either::Left(_) => Ok(Self::from_inner(input)),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }
};

#[cfg(all(feature = "smallvec_1", any(feature = "alloc", feature = "std")))]
const _: () = {
  use smallvec_1::SmallVec;

  impl<const N: usize> core::str::FromStr for EmailAddr<SmallVec<[u8; N]>> {
    type Err = ParseEmailAddrError;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
      Self::try_from(input)
    }
  }

  impl<'a, const N: usize> TryFrom<&'a str> for EmailAddr<SmallVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_str(input)? {
        Either::Left(addr) => Ok(Self::from_inner(SmallVec::from_slice(
          addr.storage.as_bytes(),
        ))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl<'a, const N: usize> TryFrom<&'a [u8]> for EmailAddr<SmallVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input)? {
        Either::Left(addr) => Ok(Self::from_inner(SmallVec::from_slice(addr.storage))),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }

  impl<const N: usize> TryFrom<SmallVec<[u8; N]>> for EmailAddr<SmallVec<[u8; N]>> {
    type Error = ParseEmailAddrError;

    #[inline]
    fn try_from(input: SmallVec<[u8; N]>) -> Result<Self, Self::Error> {
      match EmailAddr::try_from_bytes(input.as_ref())? {
        Either::Left(_) => Ok(Self::from_inner(input)),
        Either::Right(buf) => Ok(Self::from_inner(buf.into())),
      }
    }
  }
};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a str> for EmailAddr<Cow<'a, str>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_str(input)? {
      Either::Left(addr) => Ok(Self::from_inner(Cow::Borrowed(addr.storage))),
      Either::Right(buf) => Ok(Self::from_inner(Cow::Owned(buf.into()))),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a [u8]> for EmailAddr<Cow<'a, [u8]>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a [u8]) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_bytes(input)? {
      Either::Left(addr) => Ok(Self::from_inner(Cow::Borrowed(addr.storage))),
      Either::Right(buf) => Ok(Self::from_inner(Cow::Owned(buf.into()))),
    }
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'a> TryFrom<&'a str> for EmailAddr<Cow<'a, [u8]>> {
  type Error = ParseEmailAddrError;

  #[inline]
  fn try_from(input: &'a str) -> Result<Self, Self::Error> {
    match EmailAddr::try_from_str(input)? {
      Either::Left(addr) => Ok(Self::from_inner(Cow::Borrowed(addr.storage.as_bytes()))),
      Either::Right(buf) => Ok(Self::from_inner(Cow::Owned(buf.into()))),
    }
  }
}

/// Verifies that `input` is a valid email address.
///
/// ASCII addresses are validated directly. SMTPUTF8 local-parts must be valid
/// UTF-8. If the domain-part contains Unicode, it must be valid after
/// IDNA/punycode normalization.
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
pub fn verify_email_addr(input: &[u8]) -> Result<(), ParseEmailAddrError> {
  if input.is_ascii() {
    verify_ascii_email_addr(input)?;
    let (_, domain) = split_email_addr(input)?;
    if !contains_ascii_alabel(domain) {
      return Ok(());
    }
  }

  let mut output = Buffer::new();
  write_normalized_email_addr(input, &mut output)
}

/// Verifies that `input` is a valid email address under custom parsing options.
pub fn verify_email_addr_with_options(
  input: &[u8],
  options: Options,
) -> Result<(), ParseEmailAddrError> {
  let mut output = Buffer::new();
  write_email_addr_with_options(input, &mut output, options)
}

/// Verifies that `input` is a valid ASCII email address.
///
/// This accepts dot-atom and quoted-string local-parts, DNS domain-parts, and
/// bracketed IPv4/IPv6 domain literals. It enforces the 254-byte email address
/// limit and 64-byte local-part limit. Use `verify_email_addr` when SMTPUTF8
/// local-parts or IDNA domain-parts should be accepted.
pub fn verify_ascii_email_addr(input: &[u8]) -> Result<(), ParseEmailAddrError> {
  if input.is_empty() || input.len() > MAX_EMAIL_ADDR_LENGTH || !input.is_ascii() {
    return Err(ParseEmailAddrError::Address);
  }

  let (local, domain) = split_email_addr(input)?;
  if local.len() > MAX_LOCAL_PART_LENGTH {
    return Err(ParseEmailAddrError::LocalPart(ParseLocalPartError(())));
  }

  crate::verify_ascii_local_part(local)?;
  crate::verify_ascii_domain_part(domain)?;
  Ok(())
}

fn verify_borrowed_ascii_email_addr(input: &[u8]) -> Result<(), ParseEmailAddrError> {
  verify_ascii_email_addr(input)?;
  let (_, domain) = split_email_addr(input)?;
  if !contains_ascii_alabel(domain) {
    return Ok(());
  }

  #[cfg(any(feature = "alloc", feature = "std"))]
  {
    let mut output = Buffer::new();
    write_normalized_email_addr(input, &mut output)
  }

  #[cfg(not(any(feature = "alloc", feature = "std")))]
  {
    Err(ParseEmailAddrError::DomainPart(ParseDomainPartError(())))
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
fn write_normalized_email_addr(
  input: &[u8],
  output: &mut Buffer,
) -> Result<(), ParseEmailAddrError> {
  if input.is_empty() {
    return Err(ParseEmailAddrError::Address);
  }

  let (local, domain) = split_email_addr(input)?;
  crate::verify_local_part(local)?;
  output
    .extend_from_slice(local)
    .map_err(|_| ParseEmailAddrError::Address)?;
  output
    .push(b'@')
    .map_err(|_| ParseEmailAddrError::Address)?;
  write_normalized_domain_part(domain, output)?;
  Ok(())
}

fn write_email_addr_with_options(
  input: &[u8],
  output: &mut Buffer,
  options: Options,
) -> Result<(), ParseEmailAddrError> {
  if input.is_empty() {
    return Err(ParseEmailAddrError::Address);
  }

  let (local, domain) = split_email_addr(input)?;
  verify_local_part_with_limit(
    local,
    options.limits().max_local_part_len(),
    options.local().smtp_utf8().is_allow(),
  )?;
  output
    .extend_from_slice(local)
    .map_err(|_| ParseEmailAddrError::Address)?;
  output
    .push(b'@')
    .map_err(|_| ParseEmailAddrError::Address)?;
  write_domain_part_with_options(domain, output, options.domain())?;
  Ok(())
}

fn split_email_addr(input: &[u8]) -> Result<(&[u8], &[u8]), ParseEmailAddrError> {
  let Some(at) = find_at(input) else {
    return Err(ParseEmailAddrError::Address);
  };

  if at == 0 || at + 1 == input.len() {
    return Err(ParseEmailAddrError::Address);
  }

  Ok((&input[..at], &input[at + 1..]))
}

fn find_at(input: &[u8]) -> Option<usize> {
  let mut i = 0;
  let mut in_quote = false;
  let mut escaped = false;

  while i < input.len() {
    let byte = input[i];
    if escaped {
      escaped = false;
    } else if in_quote {
      match byte {
        b'\\' => escaped = true,
        b'"' => in_quote = false,
        _ => {}
      }
    } else {
      match byte {
        b'"' if i == 0 => in_quote = true,
        b'@' => return Some(i),
        _ => {}
      }
    }

    i += 1;
  }

  None
}
