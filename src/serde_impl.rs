use core::{fmt, marker::PhantomData, str};

#[cfg(any(feature = "alloc", feature = "std"))]
use std::{borrow::Cow, boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};

use serde_core::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{Buffer, DomainPart, EmailAddr, LocalPart};

/// Storage that can expose a validated email address, local-part, or
/// domain-part as a UTF-8 string for serde serialization.
///
/// The built-in string and byte storage backends implement this trait. Custom
/// storage backends can implement it to opt into `Serialize` for
/// [`EmailAddr`], [`LocalPart`], and [`DomainPart`].
///
/// This trait is separate from `AsRef<str>` so byte-backed storage can also
/// serialize without overlapping Rust trait implementations.
pub trait EmailAddrSerdeStorage {
  /// Returns the validated storage contents as a string slice.
  fn as_valid_str(&self) -> &str;
}

#[cfg_attr(not(coverage), inline(always))]
fn valid_utf8(bytes: &[u8]) -> &str {
  str::from_utf8(bytes).expect("validated email addresses are valid UTF-8")
}

impl EmailAddrSerdeStorage for str {
  #[inline]
  fn as_valid_str(&self) -> &str {
    self
  }
}

impl EmailAddrSerdeStorage for [u8] {
  #[inline]
  fn as_valid_str(&self) -> &str {
    valid_utf8(self)
  }
}

impl<T: ?Sized> EmailAddrSerdeStorage for &T
where
  T: EmailAddrSerdeStorage,
{
  #[inline]
  fn as_valid_str(&self) -> &str {
    (*self).as_valid_str()
  }
}

impl EmailAddrSerdeStorage for Buffer {
  #[inline]
  fn as_valid_str(&self) -> &str {
    self.as_str()
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  impl EmailAddrSerdeStorage for String {
    #[inline]
    fn as_valid_str(&self) -> &str {
      self.as_str()
    }
  }

  impl EmailAddrSerdeStorage for Vec<u8> {
    #[inline]
    fn as_valid_str(&self) -> &str {
      valid_utf8(self.as_slice())
    }
  }

  impl<T: ?Sized> EmailAddrSerdeStorage for Box<T>
  where
    T: EmailAddrSerdeStorage,
  {
    #[inline]
    fn as_valid_str(&self) -> &str {
      self.as_ref().as_valid_str()
    }
  }

  impl<T: ?Sized> EmailAddrSerdeStorage for Rc<T>
  where
    T: EmailAddrSerdeStorage,
  {
    #[inline]
    fn as_valid_str(&self) -> &str {
      self.as_ref().as_valid_str()
    }
  }

  impl<T: ?Sized> EmailAddrSerdeStorage for Arc<T>
  where
    T: EmailAddrSerdeStorage,
  {
    #[inline]
    fn as_valid_str(&self) -> &str {
      self.as_ref().as_valid_str()
    }
  }

  impl EmailAddrSerdeStorage for Cow<'_, str> {
    #[inline]
    fn as_valid_str(&self) -> &str {
      self.as_ref()
    }
  }

  impl EmailAddrSerdeStorage for Cow<'_, [u8]> {
    #[inline]
    fn as_valid_str(&self) -> &str {
      valid_utf8(self.as_ref())
    }
  }
};

#[cfg(feature = "smol_str_0_3")]
impl EmailAddrSerdeStorage for smol_str_0_3::SmolStr {
  #[inline]
  fn as_valid_str(&self) -> &str {
    self.as_str()
  }
}

#[cfg(feature = "triomphe_0_1")]
impl<T: ?Sized> EmailAddrSerdeStorage for triomphe_0_1::Arc<T>
where
  T: EmailAddrSerdeStorage,
{
  #[inline]
  fn as_valid_str(&self) -> &str {
    self.as_ref().as_valid_str()
  }
}

#[cfg(feature = "bytes_1")]
impl EmailAddrSerdeStorage for bytes_1::Bytes {
  #[inline]
  fn as_valid_str(&self) -> &str {
    valid_utf8(self.as_ref())
  }
}

#[cfg(feature = "tinyvec_1")]
impl<const N: usize> EmailAddrSerdeStorage for tinyvec_1::TinyVec<[u8; N]> {
  #[inline]
  fn as_valid_str(&self) -> &str {
    valid_utf8(self.as_ref())
  }
}

#[cfg(feature = "smallvec_1")]
impl<const N: usize> EmailAddrSerdeStorage for smallvec_1::SmallVec<[u8; N]> {
  #[inline]
  fn as_valid_str(&self) -> &str {
    valid_utf8(self.as_ref())
  }
}

impl<S: ?Sized> Serialize for EmailAddr<S>
where
  S: EmailAddrSerdeStorage,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_inner().as_valid_str())
  }
}

impl<S: ?Sized> Serialize for LocalPart<S>
where
  S: EmailAddrSerdeStorage,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_inner().as_valid_str())
  }
}

impl<S: ?Sized> Serialize for DomainPart<S>
where
  S: EmailAddrSerdeStorage,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_inner().as_valid_str())
  }
}

impl<'de> Deserialize<'de> for EmailAddr<Buffer> {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(EmailAddrVisitor::<Self>::new())
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
macro_rules! impl_email_addr_str_storage_deserialize {
  ($($ty:ty),+ $(,)?) => {
    $(
      impl<'de> Deserialize<'de> for EmailAddr<$ty> {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
          D: Deserializer<'de>,
        {
          deserializer.deserialize_str(EmailAddrVisitor::<Self>::new())
        }
      }
    )+
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl_email_addr_str_storage_deserialize!(String, Box<str>, Rc<str>, Arc<str>);

#[cfg(feature = "smol_str_0_3")]
impl_email_addr_str_storage_deserialize!(smol_str_0_3::SmolStr);

#[cfg(feature = "triomphe_0_1")]
impl_email_addr_str_storage_deserialize!(triomphe_0_1::Arc<str>);

#[cfg(any(feature = "alloc", feature = "std", feature = "bytes_1"))]
macro_rules! impl_email_addr_bytes_storage_deserialize {
  ($($ty:ty),+ $(,)?) => {
    $(
      impl<'de> Deserialize<'de> for EmailAddr<$ty> {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
          D: Deserializer<'de>,
        {
          deserializer.deserialize_str(EmailAddrVisitor::<Self>::new())
        }
      }
    )+
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl_email_addr_bytes_storage_deserialize!(std::vec::Vec<u8>, Box<[u8]>, Rc<[u8]>, Arc<[u8]>,);

#[cfg(feature = "bytes_1")]
impl_email_addr_bytes_storage_deserialize!(bytes_1::Bytes);

#[cfg(feature = "triomphe_0_1")]
impl_email_addr_bytes_storage_deserialize!(triomphe_0_1::Arc<[u8]>);

#[cfg(feature = "tinyvec_1")]
const _: () = {
  impl<'de, const N: usize> Deserialize<'de> for EmailAddr<tinyvec_1::TinyVec<[u8; N]>> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_str(EmailAddrVisitor::<Self>::new())
    }
  }
};

#[cfg(all(feature = "smallvec_1", any(feature = "alloc", feature = "std")))]
const _: () = {
  impl<'de, const N: usize> Deserialize<'de> for EmailAddr<smallvec_1::SmallVec<[u8; N]>> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_str(EmailAddrVisitor::<Self>::new())
    }
  }
};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'de> Deserialize<'de> for EmailAddr<Cow<'de, str>> {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(EmailAddrCowStrVisitor)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'de> Deserialize<'de> for EmailAddr<Cow<'de, [u8]>> {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(EmailAddrCowBytesVisitor)
  }
}

struct EmailAddrVisitor<T>(PhantomData<fn() -> T>);

impl<T> EmailAddrVisitor<T> {
  #[cfg_attr(not(coverage), inline(always))]
  const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<T> Visitor<'_> for EmailAddrVisitor<T>
where
  for<'a> T: TryFrom<&'a str>,
{
  type Value = T;

  fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("a valid email address string")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    T::try_from(value).map_err(|_| E::custom("invalid email address"))
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
struct EmailAddrCowStrVisitor;

#[cfg(any(feature = "alloc", feature = "std"))]
struct EmailAddrCowBytesVisitor;

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'de> Visitor<'de> for EmailAddrCowStrVisitor {
  type Value = EmailAddr<Cow<'de, str>>;

  fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("a valid email address string")
  }

  fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    EmailAddr::try_from(value).map_err(E::custom)
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    EmailAddr::<String>::try_from(value)
      .map(|addr| EmailAddr(Cow::Owned(addr.into_inner())))
      .map_err(E::custom)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<'de> Visitor<'de> for EmailAddrCowBytesVisitor {
  type Value = EmailAddr<Cow<'de, [u8]>>;

  fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("a valid email address string")
  }

  fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    EmailAddr::try_from(value).map_err(E::custom)
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    EmailAddr::<Vec<u8>>::try_from(value)
      .map(|addr| EmailAddr(Cow::Owned(addr.into_inner())))
      .map_err(E::custom)
  }
}
