use core::{fmt, marker::PhantomData};

#[cfg(any(feature = "alloc", feature = "std"))]
use std::{borrow::Cow, boxed::Box, rc::Rc, string::String, sync::Arc};

use serde_core::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{Buffer, DomainPart, EmailAddr, LocalPart};

impl<S> Serialize for EmailAddr<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl<S> Serialize for LocalPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_inner().as_ref())
  }
}

impl<S> Serialize for DomainPart<S>
where
  S: AsRef<str>,
{
  #[inline]
  fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
  where
    Ser: Serializer,
  {
    serializer.serialize_str(self.as_inner().as_ref())
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

struct EmailAddrVisitor<T>(PhantomData<fn() -> T>);

impl<T> EmailAddrVisitor<T> {
  #[cfg_attr(not(tarpaulin), inline(always))]
  const fn new() -> Self {
    Self(PhantomData)
  }
}

impl<T> Visitor<'_> for EmailAddrVisitor<T>
where
  for<'a> T: TryFrom<&'a str>,
  for<'a> <T as TryFrom<&'a str>>::Error: fmt::Display,
{
  type Value = T;

  fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("a valid email address string")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    T::try_from(value).map_err(E::custom)
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
struct EmailAddrCowStrVisitor;

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
