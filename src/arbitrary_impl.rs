use arbitrary::{Arbitrary, Result, Unstructured};

use std::{borrow::Cow, boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};

use crate::{Buffer, EmailAddr};

const ATEXT: &[u8] =
  b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-/=?^_`{|}~";

fn arbitrary_atext(u: &mut Unstructured<'_>) -> Result<char> {
  let idx = u.int_in_range(0..=ATEXT.len() - 1)?;
  Ok(ATEXT[idx] as char)
}

fn arbitrary_alnum(u: &mut Unstructured<'_>) -> Result<char> {
  let idx = u.int_in_range(0..=61)?;
  Ok(match idx {
    0..=25 => (b'a' + idx as u8) as char,
    26..=51 => (b'A' + (idx as u8 - 26)) as char,
    _ => (b'0' + (idx as u8 - 52)) as char,
  })
}

fn arbitrary_label_char(u: &mut Unstructured<'_>) -> Result<char> {
  if u.arbitrary()? {
    arbitrary_alnum(u)
  } else {
    Ok('-')
  }
}

fn arbitrary_local(u: &mut Unstructured<'_>) -> Result<String> {
  let segment_count: usize = u.int_in_range(1..=3)?;
  let mut local = String::new();

  for segment in 0..segment_count {
    if segment > 0 {
      local.push('.');
    }

    let len: usize = u.int_in_range(1..=12)?;
    for _ in 0..len {
      local.push(arbitrary_atext(u)?);
    }
  }

  Ok(local)
}

fn arbitrary_domain(u: &mut Unstructured<'_>) -> Result<String> {
  let label_count: usize = u.int_in_range(1..=3)?;
  let mut domain = String::new();

  for label in 0..label_count {
    if label > 0 {
      domain.push('.');
    }

    let len: usize = u.int_in_range(1..=20)?;
    domain.push(arbitrary_alnum(u)?);

    for _ in 1..len.saturating_sub(1) {
      domain.push(arbitrary_label_char(u)?);
    }

    if len > 1 {
      domain.push(arbitrary_alnum(u)?);
    }
  }

  Ok(domain)
}

fn arbitrary_email(u: &mut Unstructured<'_>) -> Result<EmailAddr<Buffer>> {
  let mut email = arbitrary_local(u)?;
  email.push('@');
  email.push_str(&arbitrary_domain(u)?);
  EmailAddr::<Buffer>::try_from(email.as_str()).map_err(|_| arbitrary::Error::IncorrectFormat)
}

macro_rules! impl_arbitrary_from_buffer {
  ($($ty:ty),+ $(,)?) => {
    $(
      impl<'a> Arbitrary<'a> for EmailAddr<$ty> {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
          arbitrary_email(u).map(|addr| EmailAddr(addr.0.into()))
        }
      }
    )+
  };
}

impl<'a> Arbitrary<'a> for EmailAddr<Buffer> {
  fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
    arbitrary_email(u)
  }
}

impl_arbitrary_from_buffer!(
  String,
  Box<str>,
  Rc<str>,
  Arc<str>,
  Vec<u8>,
  Box<[u8]>,
  Rc<[u8]>,
  Arc<[u8]>,
  Cow<'a, str>,
  Cow<'a, [u8]>,
);

#[cfg(feature = "smol_str_0_3")]
impl_arbitrary_from_buffer!(smol_str_0_3::SmolStr);

#[cfg(feature = "bytes_1")]
impl_arbitrary_from_buffer!(bytes_1::Bytes);

#[cfg(feature = "tinyvec_1")]
impl<'a, const N: usize> Arbitrary<'a> for EmailAddr<tinyvec_1::TinyVec<[u8; N]>> {
  fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
    arbitrary_email(u).map(|addr| EmailAddr(addr.0.into()))
  }
}

#[cfg(feature = "smallvec_1")]
impl<'a, const N: usize> Arbitrary<'a> for EmailAddr<smallvec_1::SmallVec<[u8; N]>> {
  fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
    arbitrary_email(u).map(|addr| EmailAddr(addr.0.into()))
  }
}

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  impl_arbitrary_from_buffer!(Arc<str>, Arc<[u8]>);
};
