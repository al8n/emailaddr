use std::{borrow::Cow, rc::Rc, sync::Arc};

use emailaddr::{
  verify_ascii_domain_part, verify_ascii_email_addr, verify_ascii_local_part, verify_email_addr,
  verify_local_part, Buffer, DomainPart, EmailAddr, LocalPart, MAX_EMAIL_ADDR_LENGTH,
};

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

  let general = EmailAddr::try_from_ascii_str("user@[TAG:payload]").unwrap();
  assert!(general.is_domain_literal());
  assert_eq!(general.domain_part().as_inner(), &"[TAG:payload]");
}

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

#[test]
fn enforces_length_after_domain_normalization() {
  let unicode_label = "ａ".repeat(63);
  let input = format!("u@{unicode_label}.{unicode_label}");
  assert!(input.len() > MAX_EMAIL_ADDR_LENGTH);

  let addr = EmailAddr::<String>::try_from(input.as_str()).unwrap();
  let ascii_label = "a".repeat(63);
  assert_eq!(addr.as_str(), format!("u@{ascii_label}.{ascii_label}"));
}

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
  assert!(verify_ascii_domain_part(b"example_com").is_err());
  assert!(verify_ascii_domain_part(b"example.com.").is_err());
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
    "user@[-TAG:payload]",
    "user@[TAG-:payload]",
  ] {
    assert!(EmailAddr::try_from_ascii_str(input).is_err(), "{input}");
  }
}

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
