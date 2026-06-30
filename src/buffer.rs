use core::{borrow::Borrow, fmt, str};

/// A stack-allocated buffer for a validated email address.
///
/// The buffer stores up to 254 bytes of address data plus one byte for the
/// current length. This matches the SMTP mailbox length limit used by
/// [`EmailAddr`](crate::EmailAddr).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Buffer {
  buf: [u8; 255],
}

impl PartialOrd for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.as_str().cmp(other.as_str())
  }
}

impl fmt::Display for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl Borrow<str> for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn borrow(&self) -> &str {
    self.as_str()
  }
}

impl AsRef<str> for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl AsRef<[u8]> for Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn as_ref(&self) -> &[u8] {
    self.as_bytes()
  }
}

impl<'a> From<&'a Buffer> for &'a str {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: &'a Buffer) -> Self {
    value.as_str()
  }
}

impl<'a> From<&'a Buffer> for &'a [u8] {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: &'a Buffer) -> Self {
    value.as_bytes()
  }
}

#[cfg(feature = "smol_str_0_3")]
impl From<Buffer> for smol_str_0_3::SmolStr {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: Buffer) -> Self {
    value.as_str().into()
  }
}

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  impl From<Buffer> for triomphe_0_1::Arc<str> {
    #[cfg_attr(not(tarpaulin), inline(always))]
    fn from(value: Buffer) -> Self {
      value.as_str().into()
    }
  }

  impl From<Buffer> for triomphe_0_1::Arc<[u8]> {
    #[cfg_attr(not(tarpaulin), inline(always))]
    fn from(value: Buffer) -> Self {
      value.as_bytes().into()
    }
  }
};

#[cfg(feature = "bytes_1")]
impl From<Buffer> for bytes_1::Bytes {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: Buffer) -> Self {
    Self::copy_from_slice(value.as_bytes())
  }
}

#[cfg(feature = "tinyvec_1")]
impl<const N: usize> From<Buffer> for tinyvec_1::TinyVec<[u8; N]> {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: Buffer) -> Self {
    Self::from(value.as_bytes())
  }
}

#[cfg(feature = "smallvec_1")]
impl<const N: usize> From<Buffer> for smallvec_1::SmallVec<[u8; N]> {
  #[cfg_attr(not(tarpaulin), inline(always))]
  fn from(value: Buffer) -> Self {
    Self::from_slice(value.as_bytes())
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  use std::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    rc::Rc,
    string::String,
    sync::Arc,
    vec::Vec,
  };

  impl From<Buffer> for String {
    #[inline]
    fn from(value: Buffer) -> Self {
      value.as_str().to_owned()
    }
  }

  impl From<Buffer> for Box<str> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Box::from(value.as_str())
    }
  }

  impl From<Buffer> for Rc<str> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Rc::from(value.as_str())
    }
  }

  impl From<Buffer> for Arc<str> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Arc::from(value.as_str())
    }
  }

  impl From<Buffer> for Vec<u8> {
    #[inline]
    fn from(value: Buffer) -> Self {
      value.as_bytes().to_owned()
    }
  }

  impl From<Buffer> for Box<[u8]> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Box::from(value.as_bytes())
    }
  }

  impl From<Buffer> for Rc<[u8]> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Rc::from(value.as_bytes())
    }
  }

  impl From<Buffer> for Arc<[u8]> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Arc::from(value.as_bytes())
    }
  }

  impl From<Buffer> for Cow<'_, str> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Cow::Owned(value.as_str().to_owned())
    }
  }

  impl From<Buffer> for Cow<'_, [u8]> {
    #[inline]
    fn from(value: Buffer) -> Self {
      Cow::Owned(value.as_bytes().to_owned())
    }
  }
};

impl Buffer {
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub(crate) const fn new() -> Self {
    Self { buf: [0; 255] }
  }

  /// Returns the email address as a string slice.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_str(&self) -> &str {
    str::from_utf8(self.as_bytes()).expect("validated email addresses are valid UTF-8")
  }

  /// Returns the email address as bytes.
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub fn as_bytes(&self) -> &[u8] {
    &self.buf[..self.len()]
  }

  #[cfg(any(feature = "alloc", feature = "std"))]
  #[cfg_attr(not(tarpaulin), inline(always))]
  pub(crate) const fn push(&mut self, byte: u8) -> Result<(), u8> {
    let len = self.len();
    if len == 254 {
      return Err(byte);
    }

    self.buf[len] = byte;
    self.buf[254] += 1;
    Ok(())
  }

  #[cfg_attr(not(tarpaulin), inline(always))]
  pub(crate) fn extend_from_slice(&mut self, input: &[u8]) -> Result<(), ()> {
    let len = self.len();
    let Some(end) = len.checked_add(input.len()) else {
      return Err(());
    };

    if end > 254 {
      return Err(());
    }

    self.buf[len..end].copy_from_slice(input);
    self.buf[254] = end as u8;
    Ok(())
  }

  #[cfg_attr(not(tarpaulin), inline(always))]
  pub(crate) fn copy_from_slice(input: &[u8]) -> Self {
    assert!(input.len() <= 254, "email address too long");

    let mut buf = Self::new();
    buf
      .extend_from_slice(input)
      .expect("input length was checked before copying");
    buf
  }

  #[cfg_attr(not(tarpaulin), inline(always))]
  const fn len(&self) -> usize {
    self.buf[254] as usize
  }
}
