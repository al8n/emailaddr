use quickcheck::{Arbitrary, Gen};

use std::{borrow::Cow, boxed::Box, rc::Rc, string::String, sync::Arc, vec::Vec};

use crate::{Buffer, EmailAddr};

const ATEXT: &[u8] =
  b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'*+-/=?^_`{|}~";

fn arbitrary_atext(g: &mut Gen) -> char {
  ATEXT[usize::arbitrary(g) % ATEXT.len()] as char
}

fn arbitrary_alnum(g: &mut Gen) -> char {
  let idx = usize::arbitrary(g) % 62;
  match idx {
    0..=25 => (b'a' + idx as u8) as char,
    26..=51 => (b'A' + (idx as u8 - 26)) as char,
    _ => (b'0' + (idx as u8 - 52)) as char,
  }
}

fn arbitrary_label_char(g: &mut Gen) -> char {
  if bool::arbitrary(g) {
    arbitrary_alnum(g)
  } else {
    '-'
  }
}

fn arbitrary_local(g: &mut Gen) -> String {
  let segment_count = (usize::arbitrary(g) % 3) + 1;
  let mut local = String::new();

  for segment in 0..segment_count {
    if segment > 0 {
      local.push('.');
    }

    let len = (usize::arbitrary(g) % 12) + 1;
    for _ in 0..len {
      local.push(arbitrary_atext(g));
    }
  }

  local
}

fn arbitrary_domain(g: &mut Gen) -> String {
  let label_count = (usize::arbitrary(g) % 3) + 1;
  let mut domain = String::new();

  for label in 0..label_count {
    if label > 0 {
      domain.push('.');
    }

    let len = (usize::arbitrary(g) % 20) + 1;
    domain.push(arbitrary_alnum(g));

    for _ in 1..len.saturating_sub(1) {
      domain.push(arbitrary_label_char(g));
    }

    if len > 1 {
      domain.push(arbitrary_alnum(g));
    }
  }

  domain
}

fn arbitrary_email(g: &mut Gen) -> EmailAddr<Buffer> {
  let mut email = arbitrary_local(g);
  email.push('@');
  email.push_str(&arbitrary_domain(g));
  EmailAddr::<Buffer>::try_from(email.as_str())
    .unwrap_or_else(|_| EmailAddr::<Buffer>::try_from("user@example.com").unwrap())
}

macro_rules! impl_arbitrary_from_buffer {
  ($($ty:ty),+ $(,)?) => {
    $(
      impl Arbitrary for EmailAddr<$ty> {
        fn arbitrary(g: &mut Gen) -> Self {
          let addr = arbitrary_email(g);
          EmailAddr(addr.0.into())
        }
      }
    )+
  };
}

impl Arbitrary for EmailAddr<Buffer> {
  fn arbitrary(g: &mut Gen) -> Self {
    arbitrary_email(g)
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
  Cow<'static, str>,
  Cow<'static, [u8]>,
);

#[cfg(feature = "smol_str_0_3")]
impl_arbitrary_from_buffer!(smol_str_0_3::SmolStr);

#[cfg(feature = "bytes_1")]
impl_arbitrary_from_buffer!(bytes_1::Bytes);

#[cfg(feature = "tinyvec_1")]
impl<const N: usize> Arbitrary for EmailAddr<tinyvec_1::TinyVec<[u8; N]>> {
  fn arbitrary(g: &mut Gen) -> Self {
    let addr = arbitrary_email(g);
    EmailAddr(addr.0.into())
  }
}

#[cfg(feature = "smallvec_1")]
impl<const N: usize> Arbitrary for EmailAddr<smallvec_1::SmallVec<[u8; N]>> {
  fn arbitrary(g: &mut Gen) -> Self {
    let addr = arbitrary_email(g);
    EmailAddr(addr.0.into())
  }
}

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  impl_arbitrary_from_buffer!(Arc<str>, Arc<[u8]>);
};
