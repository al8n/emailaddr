use emailaddr::{
  verify_ascii_dns_domain, verify_ascii_domain_part, verify_ascii_local_part, verify_local_part,
  DomainPart, EmailAddr, LocalPart,
};

#[cfg(any(feature = "alloc", feature = "serde", feature = "std"))]
use emailaddr::Buffer;

#[cfg(any(feature = "alloc", feature = "std"))]
use emailaddr::{verify_ascii_email_addr, verify_email_addr, MAX_EMAIL_ADDR_LENGTH};

#[cfg(any(feature = "alloc", feature = "std"))]
use std::{borrow::Cow, rc::Rc, sync::Arc};

#[cfg(feature = "serde")]
fn assert_serde<T>()
where
  T: serde_core::Serialize + for<'de> serde_core::Deserialize<'de>,
{
}

#[cfg(feature = "serde")]
fn assert_serialize<T>()
where
  T: serde_core::Serialize,
{
}

#[test]
fn validates_common_addresses() {
  let addr = EmailAddr::try_from_ascii_str("user.name+tag@example.com").unwrap();
  assert_eq!(addr.as_str(), "user.name+tag@example.com");
  assert_eq!(addr.local_part().as_inner(), &"user.name+tag");
  assert_eq!(addr.domain_part().as_inner(), &"example.com");
  assert!(!addr.is_domain_literal());

  let quoted = EmailAddr::try_from_ascii_str("\"user name\"@example.com").unwrap();
  assert_eq!(quoted.local_part().as_inner(), &"\"user name\"");

  let quoted_at = EmailAddr::try_from_ascii_str("\"a@b\"@example.com").unwrap();
  assert_eq!(quoted_at.local_part().as_inner(), &"\"a@b\"");
}

#[test]
fn validates_domain_literals() {
  let ipv4 = EmailAddr::try_from_ascii_str("user@[127.0.0.1]").unwrap();
  assert!(ipv4.is_domain_literal());
  assert_eq!(ipv4.domain_part().as_inner(), &"[127.0.0.1]");

  let ipv6 = EmailAddr::try_from_ascii_str("user@[IPv6:::1]").unwrap();
  assert!(ipv6.is_domain_literal());
  assert_eq!(ipv6.domain_part().as_inner(), &"[IPv6:::1]");

  let ipv6_lower = EmailAddr::try_from_ascii_str("user@[ipv6:::1]").unwrap();
  assert!(ipv6_lower.is_domain_literal());
  assert_eq!(ipv6_lower.domain_part().as_inner(), &"[ipv6:::1]");

  let ipv6_mixed = EmailAddr::try_from_ascii_str("user@[IpV6:::1]").unwrap();
  assert!(ipv6_mixed.is_domain_literal());
  assert_eq!(ipv6_mixed.domain_part().as_inner(), &"[IpV6:::1]");

  let general = EmailAddr::try_from_ascii_str("user@[TAG:payload]").unwrap();
  assert!(general.is_domain_literal());
  assert_eq!(general.domain_part().as_inner(), &"[TAG:payload]");
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn normalizes_unicode_domains_for_owned_storage() {
  let addr: EmailAddr<String> = "user@测试.中国".parse().unwrap();
  assert_eq!(addr.as_str(), "user@xn--0zwm56d.xn--fiqs8s");
  assert_eq!(addr.local_part().as_inner(), &"user");
  assert_eq!(addr.domain_part().as_inner(), &"xn--0zwm56d.xn--fiqs8s");

  let stack = EmailAddr::<Buffer>::try_from("user@测试.中国").unwrap();
  assert_eq!(stack.as_str(), "user@xn--0zwm56d.xn--fiqs8s");

  assert!(verify_email_addr("user@测试.中国".as_bytes()).is_ok());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn validates_ascii_alabels_through_idna() {
  let valid_input = "user@xn--0zwm56d.xn--fiqs8s";
  let valid = EmailAddr::<String>::try_from(valid_input).unwrap();
  assert_eq!(valid.as_str(), "user@xn--0zwm56d.xn--fiqs8s");
  assert!(EmailAddr::<&str>::try_from(valid_input).is_ok());
  assert!(EmailAddr::<&[u8]>::try_from(valid_input.as_bytes()).is_ok());
  assert!(EmailAddr::try_from_ascii_str(valid_input).is_ok());
  assert!(EmailAddr::try_from_ascii_bytes(valid_input.as_bytes()).is_ok());
  assert!(verify_email_addr(b"user@xn--0zwm56d.xn--fiqs8s").is_ok());

  let invalid = "user@xn--55555577.com";
  assert!(EmailAddr::<&str>::try_from(invalid).is_err());
  assert!(EmailAddr::<&[u8]>::try_from(invalid.as_bytes()).is_err());
  assert!(EmailAddr::try_from_ascii_str(invalid).is_err());
  assert!(EmailAddr::try_from_ascii_bytes(invalid.as_bytes()).is_err());
  assert!(EmailAddr::<String>::try_from(invalid).is_err());
  assert!(EmailAddr::<Buffer>::try_from(invalid).is_err());
  assert!(verify_email_addr(invalid.as_bytes()).is_err());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn ignores_alabel_like_text_inside_address_literals() {
  let literal = "user@[TAG:a.xn--payload]";

  let string = EmailAddr::<String>::try_from(literal).unwrap();
  assert_eq!(string.as_str(), literal);
  assert!(EmailAddr::<&str>::try_from(literal).is_ok());
  assert!(EmailAddr::<&[u8]>::try_from(literal.as_bytes()).is_ok());
  assert!(EmailAddr::<Buffer>::try_from(literal).is_ok());
  assert!(verify_email_addr(literal.as_bytes()).is_ok());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn validates_domain_part_ascii_alabels_through_idna() {
  let valid = "xn--0zwm56d.xn--fiqs8s";
  assert!(DomainPart::<&str>::try_from_ascii_str(valid).is_ok());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(valid.as_bytes()).is_ok());
  assert!(verify_ascii_dns_domain(valid.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(valid.as_bytes()).is_ok());

  let invalid = "xn--55555577.com";
  assert!(DomainPart::<&str>::try_from_ascii_str(invalid).is_err());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(invalid.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid.as_bytes()).is_err());

  let literal = "[TAG:a.xn--payload]";
  assert!(DomainPart::<&str>::try_from_ascii_str(literal).is_ok());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(literal.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(literal.as_bytes()).is_ok());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn preserves_ordinary_ascii_labels_next_to_idna_labels() {
  let mixed = "ab--cd.xn--0zwm56d";
  assert!(verify_ascii_dns_domain(mixed.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(mixed.as_bytes()).is_ok());
  assert!(DomainPart::<&str>::try_from_ascii_str(mixed).is_ok());
  assert!(EmailAddr::<&str>::try_from("user@ab--cd.xn--0zwm56d").is_ok());

  let ascii: EmailAddr<String> = EmailAddr::try_from("user@ab--cd.xn--0zwm56d").unwrap();
  assert_eq!(ascii.as_str(), "user@ab--cd.xn--0zwm56d");

  let unicode: EmailAddr<String> = EmailAddr::try_from("user@ab--cd.测试").unwrap();
  assert_eq!(unicode.as_str(), "user@ab--cd.xn--0zwm56d");
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn rejects_idna_labels_with_decoded_hyphen_violations() {
  let invalid_alabel_domain = "xn----bga.com";
  assert!(verify_ascii_dns_domain(invalid_alabel_domain.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid_alabel_domain.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from_ascii_str(invalid_alabel_domain).is_err());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(invalid_alabel_domain.as_bytes()).is_err());

  let invalid_alabel_addr = "user@xn----bga.com";
  assert!(EmailAddr::<&str>::try_from(invalid_alabel_addr).is_err());
  assert!(EmailAddr::<&[u8]>::try_from(invalid_alabel_addr.as_bytes()).is_err());
  assert!(EmailAddr::<String>::try_from(invalid_alabel_addr).is_err());
  assert!(EmailAddr::<Cow<'_, str>>::try_from(invalid_alabel_addr).is_err());
  assert!(EmailAddr::<Vec<u8>>::try_from(invalid_alabel_addr.as_bytes()).is_err());
  assert!(EmailAddr::<Buffer>::try_from(invalid_alabel_addr).is_err());
  assert!(verify_email_addr(invalid_alabel_addr.as_bytes()).is_err());

  for input in ["user@-é.com", "user@é-.com"] {
    assert!(EmailAddr::<String>::try_from(input).is_err(), "{input}");
    assert!(
      EmailAddr::<Cow<'_, str>>::try_from(input).is_err(),
      "{input}"
    );
    assert!(EmailAddr::<Buffer>::try_from(input).is_err(), "{input}");
    assert!(verify_email_addr(input.as_bytes()).is_err(), "{input}");
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn rejects_mixed_bidi_domains_after_idna_normalization() {
  let rtl_alabel_domain = "123.xn--9dbne9b";
  assert!(verify_ascii_dns_domain(rtl_alabel_domain.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(rtl_alabel_domain.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from_ascii_str(rtl_alabel_domain).is_err());

  for input in ["user@123.שלום", "user@123.xn--9dbne9b"] {
    assert!(EmailAddr::<String>::try_from(input).is_err(), "{input}");
    assert!(
      EmailAddr::<Cow<'_, str>>::try_from(input).is_err(),
      "{input}"
    );
    assert!(EmailAddr::<Buffer>::try_from(input).is_err(), "{input}");
    assert!(verify_email_addr(input.as_bytes()).is_err(), "{input}");
  }
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[test]
fn borrowed_constructors_reject_ascii_alabels_without_idna() {
  let valid_alabel = "user@xn--0zwm56d.xn--fiqs8s";
  assert!(EmailAddr::<&str>::try_from(valid_alabel).is_err());
  assert!(EmailAddr::<&[u8]>::try_from(valid_alabel.as_bytes()).is_err());
  assert!(EmailAddr::try_from_ascii_str(valid_alabel).is_err());
  assert!(EmailAddr::try_from_ascii_bytes(valid_alabel.as_bytes()).is_err());

  let literal = "user@[TAG:a.xn--payload]";
  assert!(EmailAddr::<&str>::try_from(literal).is_ok());
  assert!(EmailAddr::<&[u8]>::try_from(literal.as_bytes()).is_ok());
  assert!(EmailAddr::try_from_ascii_str(literal).is_ok());
  assert!(EmailAddr::try_from_ascii_bytes(literal.as_bytes()).is_ok());
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[test]
fn domain_part_rejects_ascii_alabels_without_idna() {
  let valid_alabel = "xn--0zwm56d.xn--fiqs8s";
  assert!(DomainPart::<&str>::try_from_ascii_str(valid_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(valid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(valid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(valid_alabel.as_bytes()).is_err());

  let invalid_alabel = "xn--55555577.com";
  assert!(DomainPart::<&str>::try_from_ascii_str(invalid_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(invalid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid_alabel.as_bytes()).is_err());

  let invalid_hyphen_alabel = "xn----bga.com";
  assert!(DomainPart::<&str>::try_from_ascii_str(invalid_hyphen_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(invalid_hyphen_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid_hyphen_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid_hyphen_alabel.as_bytes()).is_err());

  let literal = "[TAG:a.xn--payload]";
  assert!(DomainPart::<&str>::try_from_ascii_str(literal).is_ok());
  assert!(DomainPart::<&[u8]>::try_from_ascii_bytes(literal.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(literal.as_bytes()).is_ok());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn enforces_length_after_domain_normalization() {
  let unicode_label = "ａ".repeat(63);
  let input = format!("u@{unicode_label}.{unicode_label}");
  assert!(input.len() > MAX_EMAIL_ADDR_LENGTH);

  let addr = EmailAddr::<String>::try_from(input.as_str()).unwrap();
  let ascii_label = "a".repeat(63);
  assert_eq!(addr.as_str(), format!("u@{ascii_label}.{ascii_label}"));
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn supports_smtp_utf8_local_parts() {
  let addr = EmailAddr::<String>::try_from("用户@example.com").unwrap();
  assert_eq!(addr.as_str(), "用户@example.com");
  assert_eq!(addr.local_part().as_inner(), &"用户");

  let quoted = EmailAddr::<String>::try_from("\"用户 name\"@example.com").unwrap();
  assert_eq!(quoted.local_part().as_inner(), &"\"用户 name\"");

  assert!(verify_email_addr("用户@example.com".as_bytes()).is_ok());
  assert!(verify_ascii_email_addr("用户@example.com".as_bytes()).is_err());
}

#[test]
fn validates_parts_directly() {
  let local = LocalPart::try_from_ascii_str("first.last").unwrap();
  assert_eq!(local.as_inner(), &"first.last");
  assert!(verify_ascii_local_part(b"first..last").is_err());
  assert!(verify_local_part("用户.name".as_bytes()).is_ok());

  let domain = DomainPart::try_from_ascii_str("example.com").unwrap();
  assert_eq!(domain.as_inner(), &"example.com");
  assert!(verify_ascii_domain_part(b"example.123").is_ok());
  assert!(verify_ascii_dns_domain(b"example.123").is_ok());
  assert!(verify_ascii_domain_part(b"example_com").is_err());
  assert!(verify_ascii_dns_domain(b"example_com").is_err());
  assert!(verify_ascii_domain_part(b"example.com.").is_err());
  assert!(verify_ascii_domain_part(b"[ipv6:::1]").is_ok());
  assert!(verify_ascii_domain_part(b"[IpV6:not-ip]").is_err());
}

#[test]
fn rejects_invalid_addresses() {
  for input in [
    "",
    "missing-at.example.com",
    "@example.com",
    "user@",
    "user..name@example.com",
    ".user@example.com",
    "user.@example.com",
    "user@example..com",
    "user@example_com",
    "user@example.com.",
    "user@-example.com",
    "user@example-.com",
    "\"unterminated@example.com",
    "\"bad\\\n\"@example.com",
    "user@[999.0.0.1]",
    "user@[IPv6:not-ip]",
    "user@[ipv6:not-ip]",
    "user@[IpV6:not-ip]",
    "user@[-TAG:payload]",
    "user@[TAG-:payload]",
  ] {
    assert!(EmailAddr::try_from_ascii_str(input).is_err(), "{input}");
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn supports_storage_backends() {
  let string: EmailAddr<String> = "user@example.com".parse().unwrap();
  let arc: EmailAddr<Arc<str>> = EmailAddr::try_from("user@example.com").unwrap();
  let rc: EmailAddr<Rc<str>> = EmailAddr::try_from("user@example.com").unwrap();
  let boxed: EmailAddr<Box<str>> = EmailAddr::try_from("user@example.com").unwrap();
  let cow: EmailAddr<Cow<'_, str>> = EmailAddr::try_from("user@example.com").unwrap();
  let bytes: EmailAddr<Vec<u8>> = EmailAddr::try_from(b"user@example.com".as_slice()).unwrap();
  let stack: EmailAddr<Buffer> = EmailAddr::try_from("user@example.com").unwrap();

  assert_eq!(string.as_str(), arc.as_str());
  assert_eq!(arc.as_str(), rc.as_str());
  assert_eq!(rc.as_str(), boxed.as_str());
  assert_eq!(boxed.as_str(), cow.as_str());
  assert_eq!(bytes.as_bytes(), stack.as_bytes());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn enforces_length_limits() {
  let local = "a".repeat(64);
  let valid = format!("{local}@example.com");
  assert!(EmailAddr::<String>::try_from(valid).is_ok());

  let local = "a".repeat(65);
  let invalid = format!("{local}@example.com");
  assert!(EmailAddr::<String>::try_from(invalid).is_err());

  let long = format!("{}@{}", "a".repeat(64), "b".repeat(MAX_EMAIL_ADDR_LENGTH));
  assert!(EmailAddr::<String>::try_from(long).is_err());

  let long_utf8 = format!("{}@example.com", "用".repeat(90));
  assert!(long_utf8.len() > MAX_EMAIL_ADDR_LENGTH);
  assert!(EmailAddr::<String>::try_from(long_utf8).is_err());
}

#[cfg(feature = "serde")]
#[test]
fn supports_serde_core_for_stack_storage() {
  assert_serde::<EmailAddr<Buffer>>();
  assert_serialize::<LocalPart<&str>>();
  assert_serialize::<DomainPart<&str>>();
}

#[cfg(all(feature = "serde", any(feature = "alloc", feature = "std")))]
#[test]
fn supports_serde_core_for_owned_storage() {
  assert_serde::<EmailAddr<String>>();
  assert_serde::<EmailAddr<Box<str>>>();
  assert_serde::<EmailAddr<Rc<str>>>();
  assert_serde::<EmailAddr<Arc<str>>>();
  assert_serialize::<EmailAddr<Cow<'static, str>>>();
}

#[cfg(all(feature = "serde", any(feature = "alloc", feature = "std")))]
#[test]
fn serde_deserialization_rejects_malformed_ascii_alabels() {
  use serde_core::{
    de::value::{Error, StrDeserializer},
    Deserialize,
  };

  let valid = StrDeserializer::<Error>::new("user@xn--0zwm56d.xn--fiqs8s");
  assert!(EmailAddr::<String>::deserialize(valid).is_ok());

  let invalid = StrDeserializer::<Error>::new("user@xn--55555577.com");
  assert!(EmailAddr::<String>::deserialize(invalid).is_err());

  let invalid_buffer = StrDeserializer::<Error>::new("user@xn--55555577.com");
  assert!(EmailAddr::<Buffer>::deserialize(invalid_buffer).is_err());

  let invalid_hyphen = StrDeserializer::<Error>::new("user@xn----bga.com");
  assert!(EmailAddr::<String>::deserialize(invalid_hyphen).is_err());

  let invalid_hyphen_buffer = StrDeserializer::<Error>::new("user@xn----bga.com");
  assert!(EmailAddr::<Buffer>::deserialize(invalid_hyphen_buffer).is_err());

  let invalid_bidi = StrDeserializer::<Error>::new("user@123.xn--9dbne9b");
  assert!(EmailAddr::<String>::deserialize(invalid_bidi).is_err());

  let invalid_bidi_buffer = StrDeserializer::<Error>::new("user@123.xn--9dbne9b");
  assert!(EmailAddr::<Buffer>::deserialize(invalid_bidi_buffer).is_err());

  let valid_buffer = StrDeserializer::<Error>::new("user@xn--0zwm56d.xn--fiqs8s");
  assert!(EmailAddr::<Buffer>::deserialize(valid_buffer).is_ok());

  let literal = StrDeserializer::<Error>::new("user@[TAG:a.xn--payload]");
  assert!(EmailAddr::<String>::deserialize(literal).is_ok());
}

#[cfg(all(feature = "serde", not(any(feature = "alloc", feature = "std"))))]
#[test]
fn serde_only_buffer_deserialization_rejects_ascii_alabels() {
  use serde_core::{
    de::value::{Error, StrDeserializer},
    Deserialize,
  };

  let invalid = StrDeserializer::<Error>::new("user@xn--55555577.com");
  assert!(EmailAddr::<Buffer>::deserialize(invalid).is_err());

  let valid_alabel = StrDeserializer::<Error>::new("user@xn--0zwm56d.xn--fiqs8s");
  assert!(EmailAddr::<Buffer>::deserialize(valid_alabel).is_err());

  let literal = StrDeserializer::<Error>::new("user@[TAG:a.xn--payload]");
  assert!(EmailAddr::<Buffer>::deserialize(literal).is_ok());
}
