use emailaddr::{
  verify_ascii_dns_domain, verify_ascii_domain_part, verify_ascii_local_part,
  verify_email_addr_with_options, verify_local_part, Buffer, DomainLiteralPolicy, DomainOptions,
  DomainPart, DomainUnicodePolicy, EmailAddr, Limits, LocalOptions, LocalPart, Options, Relax,
  SmtpUtf8Policy,
};

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

#[cfg(all(feature = "serde", any(feature = "alloc", feature = "std")))]
fn assert_deserialize<'de, T>()
where
  T: serde_core::Deserialize<'de>,
{
}

#[cfg(all(feature = "arbitrary", any(feature = "alloc", feature = "std")))]
fn assert_arbitrary<'a, T>()
where
  T: arbitrary::Arbitrary<'a>,
{
}

#[cfg(all(feature = "quickcheck", any(feature = "alloc", feature = "std")))]
fn assert_quickcheck<T>()
where
  T: quickcheck::Arbitrary,
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

  let display_like =
    EmailAddr::try_from_ascii_str("\"User <user@example.com>\"@example.com").unwrap();
  assert_eq!(
    display_like.local_part().as_inner(),
    &"\"User <user@example.com>\""
  );
}

#[test]
fn rejects_rfc5322_display_names_and_address_lists() {
  for input in [
    "User <user@example.com>",
    "With, Comma <a@example.net>",
    "a@example.net, b@example.net",
  ] {
    assert!(EmailAddr::try_from_ascii_str(input).is_err(), "{input}");
  }
}

#[test]
fn supports_unsized_borrowed_wrappers() {
  let addr: &EmailAddr<str> = EmailAddr::try_from_ascii_str("user.name+tag@example.com").unwrap();
  assert_eq!(addr.as_inner(), "user.name+tag@example.com");
  assert_eq!(addr.local_part_ref().as_inner(), "user.name+tag");
  assert_eq!(addr.domain_part_ref().as_inner(), "example.com");
  assert_eq!(addr.parts_ref().0.as_inner(), "user.name+tag");
  assert_eq!(addr.parts_ref().1.as_inner(), "example.com");

  let addr_value: EmailAddr<&str> = addr.as_ref();
  assert_eq!(addr_value.as_inner(), &"user.name+tag@example.com");

  let bytes: &EmailAddr<[u8]> = addr.as_bytes_addr();
  assert_eq!(bytes.as_inner(), b"user.name+tag@example.com");
  assert_eq!(bytes.as_str_addr().as_inner(), "user.name+tag@example.com");

  let bytes = EmailAddr::<[u8]>::try_from_ascii_bytes(b"user@example.com").unwrap();
  assert_eq!(bytes.as_str_addr().as_inner(), "user@example.com");

  let local: &LocalPart<str> = LocalPart::try_from_ascii_str("first.last").unwrap();
  assert_eq!(local.as_inner(), "first.last");
  assert_eq!(local.as_bytes().as_inner(), b"first.last");
  assert_eq!(
    LocalPart::<[u8]>::try_from_ascii_bytes(b"first.last")
      .unwrap()
      .as_str()
      .as_inner(),
    "first.last"
  );

  let domain: &DomainPart<str> = DomainPart::try_from_ascii_str("example.com").unwrap();
  assert_eq!(domain.as_inner(), "example.com");
  assert_eq!(domain.as_bytes().as_inner(), b"example.com");
  assert_eq!(
    DomainPart::<[u8]>::try_from_ascii_bytes(b"example.com")
      .unwrap()
      .as_str()
      .as_inner(),
    "example.com"
  );
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

#[test]
fn supports_parse_options_for_domain_literals() {
  let literal = "user@[127.0.0.1]";
  let default = EmailAddr::<Buffer, Relax>::parse_with_options(literal, Options::new()).unwrap();
  assert!(default.is_domain_literal());

  let forbidden = Options::new().with_domain(DomainOptions::new().without_domain_literals());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(literal, forbidden).is_err());
  assert!(verify_email_addr_with_options(literal.as_bytes(), forbidden).is_err());

  let explicit =
    Options::new().with_domain(DomainOptions::new().with_literals(DomainLiteralPolicy::Forbid));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(literal, explicit).is_err());
}

#[test]
fn converts_strict_email_addr_into_relaxed_email_addr() {
  let strict = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let relaxed: EmailAddr<Buffer, Relax> = strict.into();
  assert_eq!(relaxed.as_str(), "user@example.com");
  assert_eq!(relaxed.parts(), ("user", "example.com"));
}

#[test]
fn supports_parse_options_for_minimum_dns_labels() {
  let single = "ted.backer@gmail";
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(single, Options::new()).is_ok());

  let require_tld = Options::new().with_domain(DomainOptions::new().with_required_tld());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(single, require_tld).is_err());
  assert!(verify_email_addr_with_options(single.as_bytes(), require_tld).is_err());

  let multi = "ted.backer@gmail.com";
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(multi, require_tld).is_ok());

  let three_labels = Options::new().with_domain(DomainOptions::new().with_minimum_dns_labels(3));
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options("user@example.com", three_labels).is_err()
  );
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options("user@mail.example.com", three_labels).is_ok()
  );
}

#[test]
fn supports_parse_options_for_smtp_utf8_local_parts() {
  let input = "用户@example.com";
  let allowed = EmailAddr::<Buffer, Relax>::parse_with_options(input, Options::new()).unwrap();
  assert_eq!(allowed.local_part(), "用户");

  let forbidden =
    Options::new().with_local(LocalOptions::new().with_smtp_utf8(SmtpUtf8Policy::Forbid));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(input, forbidden).is_err());
  assert!(verify_email_addr_with_options(input.as_bytes(), forbidden).is_err());
}

#[test]
fn supports_parse_options_for_non_standard_utf8_domains() {
  let input = "👋@💌.kz";
  let default = EmailAddr::<Buffer, Relax>::parse_with_options(input, Options::new());
  #[cfg(any(feature = "alloc", feature = "std"))]
  assert_ne!(default.unwrap().as_str(), input);
  #[cfg(not(any(feature = "alloc", feature = "std")))]
  assert!(default.is_err());

  let options = Options::new()
    .with_domain(DomainOptions::new().with_unicode(DomainUnicodePolicy::NonStandardUtf8));
  let addr = EmailAddr::<Buffer, Relax>::parse_with_options(input, options).unwrap();
  assert_eq!(addr.as_str(), input);
  assert_eq!(addr.local_part(), "👋");
  assert_eq!(addr.domain_part(), "💌.kz");
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@xn--55555577.💌", options).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@xn--é.example", options).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@ab--é.example", options).is_err());
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options("user@é.123.xn--9dbne9b", options).is_err()
  );

  let ascii_only =
    Options::new().with_domain(DomainOptions::new().with_unicode(DomainUnicodePolicy::AsciiOnly));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@测试.中国", ascii_only).is_err());
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
  assert!(DomainPart::<str>::try_from_ascii_str(valid).is_ok());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(valid.as_bytes()).is_ok());
  assert!(DomainPart::<&str>::try_from(valid).is_ok());
  assert!(DomainPart::<&[u8]>::try_from(valid.as_bytes()).is_ok());
  assert!(verify_ascii_dns_domain(valid.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(valid.as_bytes()).is_ok());

  let invalid = "xn--55555577.com";
  assert!(DomainPart::<str>::try_from_ascii_str(invalid).is_err());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(invalid.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from(invalid).is_err());
  assert!(DomainPart::<&[u8]>::try_from(invalid.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid.as_bytes()).is_err());

  let literal = "[TAG:a.xn--payload]";
  assert!(DomainPart::<str>::try_from_ascii_str(literal).is_ok());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(literal.as_bytes()).is_ok());
  assert!(DomainPart::<&str>::try_from(literal).is_ok());
  assert!(DomainPart::<&[u8]>::try_from(literal.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(literal.as_bytes()).is_ok());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn preserves_ordinary_ascii_labels_next_to_idna_labels() {
  let mixed = "ab--cd.xn--0zwm56d";
  assert!(verify_ascii_dns_domain(mixed.as_bytes()).is_ok());
  assert!(verify_ascii_domain_part(mixed.as_bytes()).is_ok());
  assert!(DomainPart::<str>::try_from_ascii_str(mixed).is_ok());
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
  assert!(DomainPart::<str>::try_from_ascii_str(invalid_alabel_domain).is_err());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(invalid_alabel_domain.as_bytes()).is_err());

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
  assert!(DomainPart::<str>::try_from_ascii_str(rtl_alabel_domain).is_err());

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
  assert!(DomainPart::<str>::try_from_ascii_str(valid_alabel).is_err());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(valid_alabel.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from(valid_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from(valid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(valid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(valid_alabel.as_bytes()).is_err());

  let invalid_alabel = "xn--55555577.com";
  assert!(DomainPart::<str>::try_from_ascii_str(invalid_alabel).is_err());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(invalid_alabel.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from(invalid_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from(invalid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid_alabel.as_bytes()).is_err());

  let invalid_hyphen_alabel = "xn----bga.com";
  assert!(DomainPart::<str>::try_from_ascii_str(invalid_hyphen_alabel).is_err());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(invalid_hyphen_alabel.as_bytes()).is_err());
  assert!(DomainPart::<&str>::try_from(invalid_hyphen_alabel).is_err());
  assert!(DomainPart::<&[u8]>::try_from(invalid_hyphen_alabel.as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(invalid_hyphen_alabel.as_bytes()).is_err());
  assert!(verify_ascii_domain_part(invalid_hyphen_alabel.as_bytes()).is_err());

  let literal = "[TAG:a.xn--payload]";
  assert!(DomainPart::<str>::try_from_ascii_str(literal).is_ok());
  assert!(DomainPart::<[u8]>::try_from_ascii_bytes(literal.as_bytes()).is_ok());
  assert!(DomainPart::<&str>::try_from(literal).is_ok());
  assert!(DomainPart::<&[u8]>::try_from(literal.as_bytes()).is_ok());
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

  let emoji = EmailAddr::<String>::try_from("I❤️CHOCOLATE@example.com").unwrap();
  assert_eq!(emoji.local_part().as_inner(), &"I❤️CHOCOLATE");

  let lower = "İⅢ@example.com".to_lowercase();
  let combining = EmailAddr::<String>::try_from(lower.as_str()).unwrap();
  assert_eq!(combining.as_str(), lower);

  let quoted = EmailAddr::<String>::try_from("\"用户 name\"@example.com").unwrap();
  assert_eq!(quoted.local_part().as_inner(), &"\"用户 name\"");

  assert!(verify_email_addr("用户@example.com".as_bytes()).is_ok());
  assert!(verify_ascii_email_addr("用户@example.com".as_bytes()).is_err());
}

#[test]
fn validates_parts_directly() {
  let local = LocalPart::try_from_ascii_str("first.last").unwrap();
  assert_eq!(local.as_inner(), "first.last");
  assert!(verify_ascii_local_part(b"first..last").is_err());
  assert!(verify_local_part("用户.name".as_bytes()).is_ok());

  let domain = DomainPart::try_from_ascii_str("example.com").unwrap();
  assert_eq!(domain.as_inner(), "example.com");
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

#[cfg(any(
  feature = "bytes_1",
  feature = "smallvec_1",
  feature = "smol_str_0_3",
  feature = "tinyvec_1",
  feature = "triomphe_0_1",
))]
#[test]
fn supports_optional_storage_backends() {
  #[cfg(feature = "smol_str_0_3")]
  {
    let addr: EmailAddr<smol_str_0_3::SmolStr> = EmailAddr::try_from("user@example.com").unwrap();
    assert_eq!(addr.as_str(), "user@example.com");
  }

  #[cfg(feature = "triomphe_0_1")]
  {
    let text: EmailAddr<triomphe_0_1::Arc<str>> = EmailAddr::try_from("user@example.com").unwrap();
    let bytes: EmailAddr<triomphe_0_1::Arc<[u8]>> =
      EmailAddr::try_from(b"user@example.com".as_slice()).unwrap();
    assert_eq!(text.as_str(), "user@example.com");
    assert_eq!(bytes.as_bytes(), b"user@example.com");
  }

  #[cfg(feature = "bytes_1")]
  {
    let addr: EmailAddr<bytes_1::Bytes> = EmailAddr::try_from("user@example.com").unwrap();
    assert_eq!(addr.as_bytes(), b"user@example.com");

    let owned = bytes_1::Bytes::copy_from_slice(b"user@example.com");
    let addr = EmailAddr::<bytes_1::Bytes>::try_from(owned).unwrap();
    assert_eq!(addr.as_bytes(), b"user@example.com");
  }

  #[cfg(feature = "tinyvec_1")]
  {
    let addr: EmailAddr<tinyvec_1::TinyVec<[u8; 32]>> =
      EmailAddr::try_from("user@example.com").unwrap();
    assert_eq!(addr.as_bytes(), b"user@example.com");
  }

  #[cfg(feature = "smallvec_1")]
  {
    let addr: EmailAddr<smallvec_1::SmallVec<[u8; 32]>> =
      EmailAddr::try_from("user@example.com").unwrap();
    assert_eq!(addr.as_bytes(), b"user@example.com");
  }
}

#[cfg(all(feature = "arbitrary", any(feature = "alloc", feature = "std")))]
#[test]
fn supports_arbitrary_for_storage_backends() {
  assert_arbitrary::<EmailAddr<Buffer>>();
  assert_arbitrary::<EmailAddr<String>>();
  assert_arbitrary::<EmailAddr<Vec<u8>>>();
  assert_arbitrary::<EmailAddr<Box<str>>>();
  assert_arbitrary::<EmailAddr<Box<[u8]>>>();
  assert_arbitrary::<EmailAddr<Rc<str>>>();
  assert_arbitrary::<EmailAddr<Rc<[u8]>>>();
  assert_arbitrary::<EmailAddr<Arc<str>>>();
  assert_arbitrary::<EmailAddr<Arc<[u8]>>>();
  assert_arbitrary::<EmailAddr<Cow<'_, str>>>();
  assert_arbitrary::<EmailAddr<Cow<'_, [u8]>>>();

  #[cfg(feature = "smol_str_0_3")]
  assert_arbitrary::<EmailAddr<smol_str_0_3::SmolStr>>();

  #[cfg(feature = "triomphe_0_1")]
  {
    assert_arbitrary::<EmailAddr<triomphe_0_1::Arc<str>>>();
    assert_arbitrary::<EmailAddr<triomphe_0_1::Arc<[u8]>>>();
  }

  #[cfg(feature = "bytes_1")]
  assert_arbitrary::<EmailAddr<bytes_1::Bytes>>();

  #[cfg(feature = "tinyvec_1")]
  assert_arbitrary::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>();

  #[cfg(feature = "smallvec_1")]
  assert_arbitrary::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>();
}

#[cfg(all(feature = "quickcheck", any(feature = "alloc", feature = "std")))]
#[test]
fn supports_quickcheck_for_storage_backends() {
  assert_quickcheck::<EmailAddr<Buffer>>();
  assert_quickcheck::<EmailAddr<String>>();
  assert_quickcheck::<EmailAddr<Vec<u8>>>();
  assert_quickcheck::<EmailAddr<Box<str>>>();
  assert_quickcheck::<EmailAddr<Box<[u8]>>>();
  assert_quickcheck::<EmailAddr<Rc<str>>>();
  assert_quickcheck::<EmailAddr<Rc<[u8]>>>();
  assert_quickcheck::<EmailAddr<Arc<str>>>();
  assert_quickcheck::<EmailAddr<Arc<[u8]>>>();
  assert_quickcheck::<EmailAddr<Cow<'static, str>>>();
  assert_quickcheck::<EmailAddr<Cow<'static, [u8]>>>();

  #[cfg(feature = "smol_str_0_3")]
  assert_quickcheck::<EmailAddr<smol_str_0_3::SmolStr>>();

  #[cfg(feature = "triomphe_0_1")]
  {
    assert_quickcheck::<EmailAddr<triomphe_0_1::Arc<str>>>();
    assert_quickcheck::<EmailAddr<triomphe_0_1::Arc<[u8]>>>();
  }

  #[cfg(feature = "bytes_1")]
  assert_quickcheck::<EmailAddr<bytes_1::Bytes>>();

  #[cfg(feature = "tinyvec_1")]
  assert_quickcheck::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>();

  #[cfg(feature = "smallvec_1")]
  assert_quickcheck::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>();
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

#[test]
fn supports_parse_options_for_long_local_parts() {
  let input = "reply+2a907e&3uofr1&&99cd5c22c2ca5b23655799316a8d8eb2dd83c3c487612cb9b9a00bf13f13afe2@mg1.substack.com";
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(input, Options::new()).is_err());

  let options = Options::new().with_limits(Limits::new().with_max_local_part_len(128));
  let addr: EmailAddr<Buffer, Relax> =
    EmailAddr::<Buffer, Relax>::parse_with_options(input, options).unwrap();
  assert_eq!(
    addr.local_part(),
    "reply+2a907e&3uofr1&&99cd5c22c2ca5b23655799316a8d8eb2dd83c3c487612cb9b9a00bf13f13afe2"
  );
  assert_eq!(addr.domain_part(), "mg1.substack.com");
  assert!(LocalPart::try_from_str(addr.local_part()).is_err());

  #[cfg(any(feature = "alloc", feature = "std"))]
  {
    let string = EmailAddr::<String, Relax>::parse_with_options(input, options).unwrap();
    assert_eq!(string.as_str(), input);
  }
}

#[test]
fn parse_options_do_not_launder_relaxed_values_as_strict_parts() {
  let options = Options::new()
    .with_domain(DomainOptions::new().with_unicode(DomainUnicodePolicy::NonStandardUtf8));
  let addr = EmailAddr::<Buffer, Relax>::parse_with_options("👋@💌.kz", options).unwrap();

  assert_eq!(addr.parts(), ("👋", "💌.kz"));
  assert!(DomainPart::try_from_ascii_str(addr.domain_part()).is_err());
}

#[cfg(feature = "serde")]
#[test]
fn supports_serde_core_for_stack_storage() {
  assert_serde::<EmailAddr<Buffer>>();
  assert_serialize::<LocalPart<&str>>();
  assert_serialize::<DomainPart<&str>>();
  assert_serde::<Options>();
  assert_serde::<LocalOptions>();
  assert_serde::<DomainOptions>();
  assert_serde::<Limits>();
  assert_serde::<SmtpUtf8Policy>();
  assert_serde::<DomainLiteralPolicy>();
  assert_serde::<DomainUnicodePolicy>();
}

#[cfg(feature = "serde")]
#[test]
fn supports_serde_core_for_options() {
  use core::fmt;

  use serde_core::{
    de::{
      value::{Error as DeError, MapDeserializer, StrDeserializer},
      DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor,
    },
    ser::{Error as SerErrorTrait, Impossible, SerializeStruct},
    Deserialize, Serialize, Serializer,
  };
  use std::{string::String, vec::Vec};

  #[derive(Debug, PartialEq, Eq)]
  enum Token {
    Str(String),
    U8(u8),
    U64(u64),
    Struct {
      name: &'static str,
      fields: Vec<(&'static str, Token)>,
    },
  }

  #[derive(Debug)]
  struct SerError(String);

  impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str(&self.0)
    }
  }

  impl std::error::Error for SerError {}

  impl SerErrorTrait for SerError {
    fn custom<T>(msg: T) -> Self
    where
      T: fmt::Display,
    {
      Self(msg.to_string())
    }
  }

  struct TokenSerializer {
    human: bool,
  }

  struct TokenStructSerializer {
    human: bool,
    name: &'static str,
    fields: Vec<(&'static str, Token)>,
  }

  macro_rules! reject_serialize {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
      $(
        fn $name(self, $($arg: $ty),*) -> Result<$ret, Self::Error> {
          let _ = ($($arg),*);
          Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
        }
      )*
    };
  }

  impl Serializer for TokenSerializer {
    type Ok = Token;
    type Error = SerError;
    type SerializeSeq = Impossible<Token, SerError>;
    type SerializeTuple = Impossible<Token, SerError>;
    type SerializeTupleStruct = Impossible<Token, SerError>;
    type SerializeTupleVariant = Impossible<Token, SerError>;
    type SerializeMap = Impossible<Token, SerError>;
    type SerializeStruct = TokenStructSerializer;
    type SerializeStructVariant = Impossible<Token, SerError>;

    fn is_human_readable(&self) -> bool {
      self.human
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
      Ok(Token::Str(value.to_owned()))
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
      Ok(Token::U8(value))
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
      Ok(Token::U64(value))
    }

    fn serialize_struct(
      self,
      name: &'static str,
      len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
      Ok(TokenStructSerializer {
        human: self.human,
        name,
        fields: Vec::with_capacity(len),
      })
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + fmt::Display,
    {
      Ok(Token::Str(value.to_string()))
    }

    reject_serialize! {
      serialize_bool(value: bool) -> Token;
      serialize_i8(value: i8) -> Token;
      serialize_i16(value: i16) -> Token;
      serialize_i32(value: i32) -> Token;
      serialize_i64(value: i64) -> Token;
      serialize_u16(value: u16) -> Token;
      serialize_u32(value: u32) -> Token;
      serialize_f32(value: f32) -> Token;
      serialize_f64(value: f64) -> Token;
      serialize_char(value: char) -> Token;
      serialize_bytes(value: &[u8]) -> Token;
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize,
    {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_unit_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_newtype_struct<T>(
      self,
      _name: &'static str,
      _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize,
    {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_newtype_variant<T>(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
      _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize,
    {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_tuple_struct(
      self,
      _name: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_tuple_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }

    fn serialize_struct_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("unsupported token"))
    }
  }

  impl SerializeStruct for TokenStructSerializer {
    type Ok = Token;
    type Error = SerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
      T: ?Sized + Serialize,
    {
      let value = value.serialize(TokenSerializer { human: self.human })?;
      self.fields.push((key, value));
      Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(Token::Struct {
        name: self.name,
        fields: self.fields,
      })
    }
  }

  struct BinaryU8Deserializer(u8);

  impl<'de> Deserializer<'de> for BinaryU8Deserializer {
    type Error = DeError;

    fn is_human_readable(&self) -> bool {
      false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_u8(self.0)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_u8(self.0)
    }

    serde_core::forward_to_deserialize_any! {
      bool i8 i16 i32 i64 u16 u32 u64 f32 f64 char str string bytes byte_buf
      option unit unit_struct newtype_struct seq tuple tuple_struct map struct
      enum identifier ignored_any
    }
  }

  struct BinaryU64Deserializer(u64);

  impl<'de> Deserializer<'de> for BinaryU64Deserializer {
    type Error = DeError;

    fn is_human_readable(&self) -> bool {
      false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_u64(self.0)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_u64(self.0)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_u64(self.0)
    }

    serde_core::forward_to_deserialize_any! {
      bool i8 i16 i32 i64 u16 u32 f32 f64 char str string bytes byte_buf
      option unit unit_struct newtype_struct seq tuple tuple_struct map struct
      enum identifier ignored_any
    }
  }

  struct HumanBytesDeserializer<'a>(&'a [u8]);

  impl<'de> Deserializer<'de> for HumanBytesDeserializer<'_> {
    type Error = DeError;

    fn is_human_readable(&self) -> bool {
      true
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_bytes(self.0)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_bytes(self.0)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      visitor.visit_bytes(self.0)
    }

    serde_core::forward_to_deserialize_any! {
      bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string byte_buf
      option unit unit_struct newtype_struct seq tuple tuple_struct map struct
      enum identifier ignored_any
    }
  }

  enum BinaryValue {
    U8(u8),
    U64(u64),
    Map(Vec<(&'static str, BinaryValue)>),
    Seq(Vec<BinaryValue>),
  }

  struct BinaryMapAccess {
    entries: std::vec::IntoIter<(&'static str, BinaryValue)>,
    value: Option<BinaryValue>,
  }

  impl<'de> MapAccess<'de> for BinaryMapAccess {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
      K: DeserializeSeed<'de>,
    {
      let Some((key, value)) = self.entries.next() else {
        return Ok(None);
      };
      self.value = Some(value);
      seed
        .deserialize(StrDeserializer::<DeError>::new(key))
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
      V: DeserializeSeed<'de>,
    {
      seed.deserialize(self.value.take().expect("map value follows map key"))
    }
  }

  struct BinarySeqAccess {
    values: std::vec::IntoIter<BinaryValue>,
  }

  impl<'de> SeqAccess<'de> for BinarySeqAccess {
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
      T: DeserializeSeed<'de>,
    {
      self
        .values
        .next()
        .map(|value| seed.deserialize(value))
        .transpose()
    }
  }

  impl<'de> Deserializer<'de> for BinaryValue {
    type Error = DeError;

    fn is_human_readable(&self) -> bool {
      false
    }

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      match self {
        Self::U8(value) => visitor.visit_u8(value),
        Self::U64(value) => visitor.visit_u64(value),
        Self::Map(entries) => visitor.visit_map(BinaryMapAccess {
          entries: entries.into_iter(),
          value: None,
        }),
        Self::Seq(values) => visitor.visit_seq(BinarySeqAccess {
          values: values.into_iter(),
        }),
      }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      self.deserialize_any(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      self.deserialize_any(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
      self,
      _name: &'static str,
      _fields: &'static [&'static str],
      visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      self.deserialize_any(visitor)
    }

    serde_core::forward_to_deserialize_any! {
      bool i8 i16 i32 i64 u16 u32 f32 f64 char str string bytes byte_buf
      option unit unit_struct newtype_struct seq tuple tuple_struct enum
      identifier ignored_any
    }
  }

  fn binary_local() -> BinaryValue {
    BinaryValue::Map(vec![("smtp_utf8", BinaryValue::U8(1))])
  }

  fn binary_domain() -> BinaryValue {
    BinaryValue::Map(vec![
      ("minimum_dns_labels", BinaryValue::U64(2)),
      ("literals", BinaryValue::U8(0)),
      ("unicode", BinaryValue::U8(2)),
    ])
  }

  fn binary_limits() -> BinaryValue {
    BinaryValue::Map(vec![("max_local_part_len", BinaryValue::U64(128))])
  }

  assert_eq!(
    SmtpUtf8Policy::Forbid
      .serialize(TokenSerializer { human: true })
      .unwrap(),
    Token::Str("forbid".to_owned())
  );
  assert_eq!(
    SmtpUtf8Policy::Allow
      .serialize(TokenSerializer { human: false })
      .unwrap(),
    Token::U8(0)
  );
  assert_eq!(
    DomainLiteralPolicy::Forbid
      .serialize(TokenSerializer { human: false })
      .unwrap(),
    Token::U8(1)
  );
  assert_eq!(
    DomainUnicodePolicy::NonStandardUtf8
      .serialize(TokenSerializer { human: true })
      .unwrap(),
    Token::Str("raw".to_owned())
  );
  assert_eq!(
    DomainUnicodePolicy::NonStandardUtf8
      .serialize(TokenSerializer { human: false })
      .unwrap(),
    Token::U8(2)
  );

  let human = DomainUnicodePolicy::deserialize(StrDeserializer::<DeError>::new("ascii")).unwrap();
  assert_eq!(human, DomainUnicodePolicy::AsciiOnly);
  let human_alias =
    DomainUnicodePolicy::deserialize(StrDeserializer::<DeError>::new("ascii_only")).unwrap();
  assert_eq!(human_alias, DomainUnicodePolicy::AsciiOnly);
  let raw_alias =
    DomainUnicodePolicy::deserialize(StrDeserializer::<DeError>::new("non_standard_utf8")).unwrap();
  assert_eq!(raw_alias, DomainUnicodePolicy::NonStandardUtf8);
  let binary = DomainUnicodePolicy::deserialize(BinaryU8Deserializer(1)).unwrap();
  assert_eq!(binary, DomainUnicodePolicy::Idna);
  let smtp_binary = SmtpUtf8Policy::deserialize(BinaryU64Deserializer(1)).unwrap();
  assert_eq!(smtp_binary, SmtpUtf8Policy::Forbid);
  let literal_bytes = DomainLiteralPolicy::deserialize(HumanBytesDeserializer(b"forbid")).unwrap();
  assert_eq!(literal_bytes, DomainLiteralPolicy::Forbid);
  assert!(DomainLiteralPolicy::deserialize(HumanBytesDeserializer(b"\xff")).is_err());
  assert!(DomainUnicodePolicy::deserialize(BinaryU64Deserializer(300)).is_err());
  assert!(DomainLiteralPolicy::deserialize(BinaryU8Deserializer(9)).is_err());

  let options = Options::new()
    .with_local(LocalOptions::new().with_smtp_utf8(SmtpUtf8Policy::Forbid))
    .with_domain(
      DomainOptions::new()
        .with_minimum_dns_labels(2)
        .with_unicode(DomainUnicodePolicy::NonStandardUtf8),
    )
    .with_limits(Limits::new().with_max_local_part_len(128));
  let token = options.serialize(TokenSerializer { human: true }).unwrap();
  assert!(matches!(
    token,
    Token::Struct {
      name: "Options",
      ..
    }
  ));

  let options_binary = bincode::serialize(&options).unwrap();
  assert_eq!(
    bincode::deserialize::<Options>(&options_binary).unwrap(),
    options
  );
  let local_binary = bincode::serialize(&options.local()).unwrap();
  assert_eq!(
    bincode::deserialize::<LocalOptions>(&local_binary).unwrap(),
    options.local()
  );
  let domain_binary = bincode::serialize(&options.domain()).unwrap();
  assert_eq!(
    bincode::deserialize::<DomainOptions>(&domain_binary).unwrap(),
    options.domain()
  );
  let limits_binary = bincode::serialize(&options.limits()).unwrap();
  assert_eq!(
    bincode::deserialize::<Limits>(&limits_binary).unwrap(),
    options.limits()
  );
  let sequence_options = Options::deserialize(BinaryValue::Seq(vec![
    BinaryValue::Seq(vec![BinaryValue::U8(1)]),
    BinaryValue::Seq(vec![
      BinaryValue::U64(2),
      BinaryValue::U8(0),
      BinaryValue::U8(2),
    ]),
    BinaryValue::Seq(vec![BinaryValue::U64(128)]),
  ]))
  .unwrap();
  assert_eq!(sequence_options, options);
  assert!(Options::deserialize(BinaryValue::Seq(vec![])).is_err());
  assert!(
    Options::deserialize(BinaryValue::Seq(vec![BinaryValue::Seq(vec![
      BinaryValue::U8(1)
    ])]))
    .is_err()
  );
  assert!(LocalOptions::deserialize(BinaryValue::Seq(vec![])).is_err());
  assert!(DomainOptions::deserialize(BinaryValue::Seq(vec![BinaryValue::U64(2)])).is_err());
  assert!(DomainOptions::deserialize(BinaryValue::Seq(vec![
    BinaryValue::U64(2),
    BinaryValue::U8(0)
  ]))
  .is_err());
  assert!(Limits::deserialize(BinaryValue::Seq(vec![])).is_err());
  let map_options = Options::deserialize(BinaryValue::Map(vec![
    ("local", binary_local()),
    ("domain", binary_domain()),
    ("limits", binary_limits()),
  ]))
  .unwrap();
  assert_eq!(map_options, options);
  assert_eq!(
    LocalOptions::deserialize(binary_local()).unwrap(),
    options.local()
  );
  assert_eq!(
    DomainOptions::deserialize(binary_domain()).unwrap(),
    options.domain()
  );
  assert_eq!(
    Limits::deserialize(binary_limits()).unwrap(),
    options.limits()
  );
  assert!(Options::deserialize(BinaryValue::Map(vec![])).is_err());
  assert!(Options::deserialize(BinaryValue::Map(vec![("local", binary_local())])).is_err());
  assert!(LocalOptions::deserialize(BinaryValue::Map(vec![])).is_err());
  assert!(DomainOptions::deserialize(BinaryValue::Map(vec![
    ("minimum_dns_labels", BinaryValue::U64(2)),
    ("literals", BinaryValue::U8(0)),
  ]))
  .is_err());
  assert!(Limits::deserialize(BinaryValue::Map(vec![])).is_err());

  let empty = core::iter::empty::<(&str, StrDeserializer<'_, DeError>)>();
  let default_options = Options::deserialize(MapDeserializer::new(empty)).unwrap();
  assert_eq!(default_options, Options::new());

  let domain = DomainOptions::deserialize(MapDeserializer::new(
    [("unicode", StrDeserializer::<DeError>::new("raw"))].into_iter(),
  ))
  .unwrap();
  assert_eq!(domain.minimum_dns_labels(), 1);
  assert_eq!(domain.literals(), DomainLiteralPolicy::Allow);
  assert_eq!(domain.unicode(), DomainUnicodePolicy::NonStandardUtf8);

  let default_local = LocalOptions::deserialize(MapDeserializer::new(core::iter::empty::<(
    &str,
    StrDeserializer<'_, DeError>,
  )>()))
  .unwrap();
  assert_eq!(default_local, LocalOptions::new());
  let default_limits = Limits::deserialize(MapDeserializer::new(core::iter::empty::<(
    &str,
    StrDeserializer<'_, DeError>,
  )>()))
  .unwrap();
  assert_eq!(default_limits, Limits::new());
}

#[cfg(feature = "serde")]
#[test]
fn supports_human_readable_config_formats_for_options() {
  let options = Options::new()
    .with_local(LocalOptions::new().with_smtp_utf8(SmtpUtf8Policy::Forbid))
    .with_domain(
      DomainOptions::new()
        .with_minimum_dns_labels(2)
        .with_literals(DomainLiteralPolicy::Forbid)
        .with_unicode(DomainUnicodePolicy::NonStandardUtf8),
    )
    .with_limits(Limits::new().with_max_local_part_len(128));

  let json = serde_json::to_string(&options).unwrap();
  assert!(json.contains("\"smtp_utf8\":\"forbid\""));
  assert!(json.contains("\"unicode\":\"raw\""));
  assert_eq!(serde_json::from_str::<Options>(&json).unwrap(), options);
  assert_eq!(
    serde_json::from_str::<DomainOptions>(r#"{"unicode":"non_standard_utf8"}"#)
      .unwrap()
      .unicode(),
    DomainUnicodePolicy::NonStandardUtf8
  );

  let toml_doc = toml::to_string(&options).unwrap();
  assert!(toml_doc.contains("smtp_utf8 = \"forbid\""));
  assert!(toml_doc.contains("unicode = \"raw\""));
  assert_eq!(toml::from_str::<Options>(&toml_doc).unwrap(), options);
  assert_eq!(
    toml::from_str::<DomainOptions>("unicode = \"ascii_only\"")
      .unwrap()
      .unicode(),
    DomainUnicodePolicy::AsciiOnly
  );

  let yaml = yaml_serde::to_string(&options).unwrap();
  assert!(yaml.contains("smtp_utf8: forbid"));
  assert!(yaml.contains("unicode: raw"));
  assert_eq!(yaml_serde::from_str::<Options>(&yaml).unwrap(), options);
  assert_eq!(
    yaml_serde::from_str::<DomainOptions>("unicode: raw_utf8")
      .unwrap()
      .unicode(),
    DomainUnicodePolicy::NonStandardUtf8
  );
}

#[cfg(feature = "serde")]
#[test]
fn rejects_invalid_options_config_fields() {
  for input in [
    "[]",
    r#"{"unknown":{}}"#,
    r#"{"local":{},"local":{}}"#,
    r#"{"domain":{},"domain":{}}"#,
    r#"{"limits":{},"limits":{}}"#,
  ] {
    assert!(serde_json::from_str::<Options>(input).is_err(), "{input}");
  }

  for input in [
    "[]",
    r#"{"unknown":"allow"}"#,
    r#"{"smtp_utf8":"allow","smtp_utf8":"forbid"}"#,
  ] {
    assert!(
      serde_json::from_str::<LocalOptions>(input).is_err(),
      "{input}"
    );
  }

  for input in [
    "[]",
    r#"{"unknown":"allow"}"#,
    r#"{"minimum_dns_labels":1,"minimum_dns_labels":2}"#,
    r#"{"literals":"allow","literals":"forbid"}"#,
    r#"{"unicode":"idna","unicode":"raw"}"#,
  ] {
    assert!(
      serde_json::from_str::<DomainOptions>(input).is_err(),
      "{input}"
    );
  }

  for input in [
    "[]",
    r#"{"unknown":64}"#,
    r#"{"max_local_part_len":64,"max_local_part_len":128}"#,
  ] {
    assert!(serde_json::from_str::<Limits>(input).is_err(), "{input}");
  }
}

#[cfg(feature = "clap")]
#[test]
fn supports_clap_for_options() {
  use clap::Parser;

  #[derive(Debug, Parser)]
  struct Cli {
    #[command(flatten)]
    options: Options,
  }

  let default = Cli::try_parse_from(["emailaddr-test"]).unwrap();
  assert_eq!(default.options, Options::new());

  let cli = Cli::try_parse_from([
    "emailaddr-test",
    "--email-local-smtp-utf8",
    "forbid",
    "--email-domain-minimum-dns-labels",
    "2",
    "--email-domain-literals",
    "forbid",
    "--email-domain-unicode",
    "raw",
    "--email-limits-max-local-part-len",
    "128",
  ])
  .unwrap();
  assert_eq!(cli.options.local().smtp_utf8(), SmtpUtf8Policy::Forbid);
  assert_eq!(cli.options.domain().minimum_dns_labels(), 2);
  assert_eq!(cli.options.domain().literals(), DomainLiteralPolicy::Forbid);
  assert_eq!(
    cli.options.domain().unicode(),
    DomainUnicodePolicy::NonStandardUtf8
  );
  assert_eq!(cli.options.limits().max_local_part_len(), 128);

  let alias_cli = Cli::try_parse_from([
    "emailaddr-test",
    "--email-domain-unicode",
    "non_standard_utf8",
  ])
  .unwrap();
  assert_eq!(
    alias_cli.options.domain().unicode(),
    DomainUnicodePolicy::NonStandardUtf8
  );
}

#[cfg(feature = "serde")]
#[test]
fn supports_serde_core_for_custom_string_storage() {
  struct CustomStr(&'static str);

  impl AsRef<str> for CustomStr {
    fn as_ref(&self) -> &str {
      self.0
    }
  }

  impl emailaddr::EmailAddrSerdeStorage for CustomStr {
    fn as_valid_str(&self) -> &str {
      self.as_ref()
    }
  }

  assert_serialize::<EmailAddr<CustomStr>>();
  assert_serialize::<LocalPart<CustomStr>>();
  assert_serialize::<DomainPart<CustomStr>>();
}

#[cfg(all(feature = "serde", any(feature = "alloc", feature = "std")))]
#[test]
fn supports_serde_core_for_owned_storage() {
  assert_serde::<EmailAddr<String>>();
  assert_serde::<EmailAddr<Vec<u8>>>();
  assert_serde::<EmailAddr<Box<str>>>();
  assert_serde::<EmailAddr<Box<[u8]>>>();
  assert_serde::<EmailAddr<Rc<str>>>();
  assert_serde::<EmailAddr<Rc<[u8]>>>();
  assert_serde::<EmailAddr<Arc<str>>>();
  assert_serde::<EmailAddr<Arc<[u8]>>>();
  assert_serialize::<EmailAddr<Cow<'static, str>>>();
  assert_serialize::<EmailAddr<Cow<'static, [u8]>>>();
  assert_deserialize::<'static, EmailAddr<Cow<'static, [u8]>>>();

  #[cfg(feature = "smol_str_0_3")]
  assert_serde::<EmailAddr<smol_str_0_3::SmolStr>>();

  #[cfg(feature = "triomphe_0_1")]
  {
    assert_serde::<EmailAddr<triomphe_0_1::Arc<str>>>();
    assert_serde::<EmailAddr<triomphe_0_1::Arc<[u8]>>>();
  }

  #[cfg(feature = "bytes_1")]
  assert_serde::<EmailAddr<bytes_1::Bytes>>();

  #[cfg(feature = "tinyvec_1")]
  assert_serde::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>();

  #[cfg(feature = "smallvec_1")]
  assert_serde::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>();
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

  let valid_box_bytes = StrDeserializer::<Error>::new("user@example.com");
  assert!(EmailAddr::<Box<[u8]>>::deserialize(valid_box_bytes).is_ok());

  let valid_cow_bytes = StrDeserializer::<Error>::new("user@example.com");
  assert!(EmailAddr::<Cow<'_, [u8]>>::deserialize(valid_cow_bytes).is_ok());

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
