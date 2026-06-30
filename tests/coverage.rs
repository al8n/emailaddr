use core::borrow::Borrow;

use emailaddr::{
  Buffer, DEFAULT_MAX_LOCAL_PART_LENGTH, DEFAULT_MINIMUM_DNS_LABELS, DomainLiteralPolicy,
  DomainOptions, DomainPart, DomainUnicodePolicy, EmailAddr, Limits, LocalOptions, LocalPart,
  Options, Relax, SmtpUtf8Policy, verify_ascii_dns_domain, verify_ascii_domain_part,
  verify_ascii_email_addr, verify_ascii_local_part, verify_email_addr_with_options,
  verify_local_part,
};

#[cfg(any(feature = "alloc", feature = "std"))]
use emailaddr::verify_email_addr;

#[test]
fn covers_options_and_error_accessors() {
  assert_eq!(DEFAULT_MAX_LOCAL_PART_LENGTH, 64);
  assert_eq!(DEFAULT_MINIMUM_DNS_LABELS, 1);

  let mut options = Options::default();
  assert_eq!(options.local(), LocalOptions::default());
  assert_eq!(options.domain(), DomainOptions::default());
  assert_eq!(options.limits(), Limits::default());

  options
    .set_local(LocalOptions::new().with_smtp_utf8(SmtpUtf8Policy::Forbid))
    .set_domain(
      DomainOptions::new()
        .with_no_minimum_dns_labels()
        .with_domain_literals()
        .with_unicode(DomainUnicodePolicy::AsciiOnly),
    )
    .set_limits(Limits::new().with_max_local_part_len(32));
  assert_eq!(options.local().smtp_utf8(), SmtpUtf8Policy::Forbid);
  assert_eq!(options.domain().minimum_dns_labels(), 0);
  assert_eq!(options.domain().literals(), DomainLiteralPolicy::Allow);
  assert_eq!(options.domain().unicode(), DomainUnicodePolicy::AsciiOnly);
  assert_eq!(options.limits().max_local_part_len(), 32);

  let options = Options::new()
    .with_local(LocalOptions::new().with_smtp_utf8(SmtpUtf8Policy::Allow))
    .with_domain(
      DomainOptions::new()
        .with_required_tld()
        .with_literals(DomainLiteralPolicy::Forbid)
        .without_domain_literals()
        .with_unicode(DomainUnicodePolicy::NonStandardUtf8),
    )
    .with_limits(Limits::new().with_max_local_part_len(128));
  assert_eq!(options.domain().minimum_dns_labels(), 2);
  assert_eq!(options.domain().literals(), DomainLiteralPolicy::Forbid);
  assert_eq!(options.limits().max_local_part_len(), 128);

  assert_eq!(SmtpUtf8Policy::Allow.as_str(), "allow");
  assert!(SmtpUtf8Policy::Allow.is_allow());
  assert!(!SmtpUtf8Policy::Allow.is_forbid());
  assert_eq!(SmtpUtf8Policy::Forbid.as_str(), "forbid");
  assert!(SmtpUtf8Policy::Forbid.is_forbid());

  assert_eq!(DomainLiteralPolicy::Allow.as_str(), "allow");
  assert!(DomainLiteralPolicy::Allow.is_allow());
  assert!(!DomainLiteralPolicy::Allow.is_forbid());
  assert_eq!(DomainLiteralPolicy::Forbid.as_str(), "forbid");
  assert!(DomainLiteralPolicy::Forbid.is_forbid());

  assert_eq!(DomainUnicodePolicy::AsciiOnly.as_str(), "ascii");
  assert!(DomainUnicodePolicy::AsciiOnly.is_ascii_only());
  assert_eq!(DomainUnicodePolicy::Idna.as_str(), "idna");
  assert!(DomainUnicodePolicy::Idna.is_idna());
  assert_eq!(DomainUnicodePolicy::NonStandardUtf8.as_str(), "raw");
  assert!(DomainUnicodePolicy::NonStandardUtf8.is_non_standard_utf8());

  let address_err = EmailAddr::try_from_ascii_str("missing-at").unwrap_err();
  assert_eq!(address_err.as_str(), "invalid email address");
  assert_eq!(address_err.to_string(), "invalid email address");

  let local_addr_err = EmailAddr::try_from_ascii_str(".bad@example.com").unwrap_err();
  assert_eq!(local_addr_err.as_str(), "invalid email local-part");

  let domain_addr_err = EmailAddr::try_from_ascii_str("user@-bad.example").unwrap_err();
  assert_eq!(domain_addr_err.as_str(), "invalid email domain-part");

  let local_err = verify_ascii_local_part(b".bad").unwrap_err();
  assert_eq!(local_err.as_str(), "invalid email local-part");
  assert_eq!(local_err.to_string(), "invalid email local-part");

  let domain_err = verify_ascii_domain_part(b"-bad.example").unwrap_err();
  assert_eq!(domain_err.as_str(), "invalid email domain-part");
  assert_eq!(domain_err.to_string(), "invalid email domain-part");
}

#[test]
fn covers_core_wrappers_and_buffer_traits() {
  let addr = EmailAddr::<Buffer>::try_from("user@[TAG:payload]").unwrap();
  assert_eq!(addr.as_inner().as_str(), "user@[TAG:payload]");
  assert_eq!(addr.to_string(), "user@[TAG:payload]");
  assert_eq!(addr.as_ref().as_inner().as_str(), "user@[TAG:payload]");
  assert_eq!(
    <EmailAddr<Buffer> as Borrow<Buffer>>::borrow(&addr),
    addr.as_inner()
  );
  assert_eq!(
    <EmailAddr<Buffer> as AsRef<str>>::as_ref(&addr),
    "user@[TAG:payload]"
  );
  assert_eq!(
    <EmailAddr<Buffer> as AsRef<[u8]>>::as_ref(&addr),
    b"user@[TAG:payload]"
  );
  assert!(addr.is_domain_literal());

  let buffer = *addr.as_inner();
  assert_eq!(buffer.to_string(), "user@[TAG:payload]");
  assert_eq!(
    <Buffer as Borrow<str>>::borrow(&buffer),
    "user@[TAG:payload]"
  );
  assert_eq!(
    <Buffer as AsRef<str>>::as_ref(&buffer),
    "user@[TAG:payload]"
  );
  assert_eq!(
    <Buffer as AsRef<[u8]>>::as_ref(&buffer),
    b"user@[TAG:payload]"
  );
  let borrowed_str: &str = (&buffer).into();
  let borrowed_bytes: &[u8] = (&buffer).into();
  assert_eq!(borrowed_str, "user@[TAG:payload]");
  assert_eq!(borrowed_bytes, b"user@[TAG:payload]");
  assert_eq!(buffer.cmp(&buffer), core::cmp::Ordering::Equal);
  assert_eq!(
    buffer.partial_cmp(&buffer),
    Some(core::cmp::Ordering::Equal)
  );

  let borrowed: EmailAddr<&str> = EmailAddr::try_from("user@example.com").unwrap();
  assert_eq!(borrowed.parts().0.as_inner(), &"user");
  assert_eq!(borrowed.parts().1.as_inner(), &"example.com");
  assert_eq!(borrowed.parts_ref().0.as_inner(), "user");
  assert_eq!(borrowed.parts_ref().1.as_inner(), "example.com");
  assert_eq!(borrowed.as_ref().copied().as_str(), "user@example.com");
  assert_eq!(borrowed.as_ref().cloned().as_str(), "user@example.com");
  assert_eq!(borrowed.as_deref().as_str(), "user@example.com");
  assert_eq!(
    borrowed.as_bytes_addr().as_str_addr().as_str(),
    "user@example.com"
  );

  let borrowed_bytes: EmailAddr<&[u8]> =
    EmailAddr::try_from(b"user@example.com".as_slice()).unwrap();
  assert_eq!(
    borrowed_bytes.as_str_addr().as_bytes_addr().as_bytes(),
    b"user@example.com"
  );

  let parsed_buffer: EmailAddr<Buffer> = "user@example.com".parse().unwrap();
  assert_eq!(parsed_buffer.as_str(), "user@example.com");

  let local: LocalPart<&str> = LocalPart::try_from("first.last").unwrap();
  assert_eq!(local.into_inner(), "first.last");
  assert_eq!(local.to_string(), "first.last");
  assert_eq!(local.as_ref().copied().as_inner(), &"first.last");
  assert_eq!(local.as_ref().cloned().as_inner(), &"first.last");
  assert_eq!(local.as_bytes().as_str().as_inner(), &"first.last");
  let local_dst = LocalPart::try_from_ascii_str("first.last").unwrap();
  assert_eq!(
    <LocalPart<str> as Borrow<str>>::borrow(local_dst),
    "first.last"
  );
  assert_eq!(
    <LocalPart<str> as AsRef<str>>::as_ref(local_dst),
    "first.last"
  );
  assert_eq!(
    <LocalPart<[u8]> as AsRef<[u8]>>::as_ref(local_dst.as_bytes()),
    b"first.last"
  );
  assert_eq!(
    LocalPart::<&[u8]>::try_from(b"first.last".as_slice())
      .unwrap()
      .as_str()
      .as_inner(),
    &"first.last"
  );

  let domain: DomainPart<&str> = DomainPart::try_from("example.com").unwrap();
  assert_eq!(domain.into_inner(), "example.com");
  assert_eq!(domain.to_string(), "example.com");
  assert_eq!(domain.as_ref().copied().as_inner(), &"example.com");
  assert_eq!(domain.as_ref().cloned().as_inner(), &"example.com");
  assert_eq!(domain.as_bytes().as_str().as_inner(), &"example.com");
  let domain_dst = DomainPart::try_from_ascii_str("example.com").unwrap();
  assert_eq!(
    <DomainPart<str> as Borrow<str>>::borrow(domain_dst),
    "example.com"
  );
  assert_eq!(
    <DomainPart<str> as AsRef<str>>::as_ref(domain_dst),
    "example.com"
  );
  assert_eq!(
    <DomainPart<[u8]> as AsRef<[u8]>>::as_ref(domain_dst.as_bytes()),
    b"example.com"
  );
  assert_eq!(
    DomainPart::<&[u8]>::try_from(b"example.com".as_slice())
      .unwrap()
      .as_str()
      .as_inner(),
    &"example.com"
  );

  assert!(verify_email_addr_with_options(b"user@example.com", Options::new()).is_ok());
  assert!(verify_local_part("\"quoted\\\\pair\"".as_bytes()).is_ok());
  assert!(verify_local_part("\"bad\\\n\"".as_bytes()).is_err());
  assert!(verify_email_addr_with_options(b"", Options::new()).is_err());
}

#[test]
fn covers_local_part_rejection_paths() {
  assert!(verify_ascii_local_part(b"").is_err());
  assert!(verify_ascii_local_part(b".bad").is_err());
  assert!(verify_ascii_local_part(b"bad.").is_err());
  assert!(verify_ascii_local_part(b"bad..local").is_err());
  assert!(verify_ascii_local_part(b"bad local").is_err());
  assert!(verify_ascii_local_part(b"\"unterminated").is_err());
  assert!(verify_ascii_local_part(b"\"bad\\\"").is_err());
  assert!(verify_ascii_local_part(b"\"bad\\\x1f\"").is_err());
  assert!(verify_ascii_local_part(b"\"bad\"quote\"").is_err());
  assert!(verify_ascii_local_part(b"\"bad\x7f\"").is_err());

  assert!(verify_local_part(&[0xff]).is_err());
  assert!(verify_local_part("用户..name".as_bytes()).is_err());
  assert!(verify_local_part("用户.".as_bytes()).is_err());
  assert!(verify_local_part("用户 name".as_bytes()).is_err());
  assert!(verify_local_part("\"用户\\\n\"".as_bytes()).is_err());
  assert!(verify_local_part("\"用户\x7f\"".as_bytes()).is_err());
  assert!(verify_local_part("\"用户".as_bytes()).is_err());

  assert!(LocalPart::<[u8]>::try_from_bytes("用户".as_bytes()).is_ok());
  assert!(LocalPart::<[u8]>::try_from_ascii_bytes(b"bad..local").is_err());
}

#[test]
fn covers_domain_part_rejection_paths() {
  assert!(verify_ascii_domain_part(b"").is_err());
  assert!(verify_ascii_domain_part("例.example".as_bytes()).is_err());
  assert!(verify_ascii_dns_domain(b"").is_err());
  assert!(verify_ascii_dns_domain(&[b'a'; 64]).is_err());
  assert!(verify_ascii_dns_domain(b".example").is_err());
  assert!(verify_ascii_dns_domain(b"example..com").is_err());
  assert!(verify_ascii_dns_domain(b"example-.com").is_err());
  assert!(verify_ascii_dns_domain(b"example.").is_err());
  assert!(verify_ascii_dns_domain(b"_bad.example").is_err());

  assert!(verify_ascii_domain_part(b"[]").is_err());
  assert!(verify_ascii_domain_part(b"[bad").is_err());
  assert!(verify_ascii_domain_part(b"[no-colon]").is_err());
  assert!(verify_ascii_domain_part(b"[:payload]").is_err());
  assert!(verify_ascii_domain_part(b"[-TAG:payload]").is_err());
  assert!(verify_ascii_domain_part(b"[TAG_:payload]").is_err());
  assert!(verify_ascii_domain_part(b"[TAG-:payload]").is_err());
  assert!(verify_ascii_domain_part(b"[TAG:]").is_err());
  assert!(verify_ascii_domain_part(b"[TAG:bad space]").is_err());
  assert!(verify_ascii_domain_part(b"[999.0.0.1]").is_err());
  assert!(verify_ascii_domain_part(b"[IPv6:not-ip]").is_err());

  assert!(verify_ascii_email_addr(b"").is_err());
  assert!(verify_ascii_email_addr(b"user@").is_err());
  assert!(verify_ascii_email_addr(b"@example.com").is_err());
  assert!(verify_ascii_email_addr(b"user@@example.com").is_err());
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn covers_idna_and_relaxed_domain_branches() {
  assert!(verify_email_addr(b"user@example.com").is_ok());
  assert!(verify_email_addr(b"user@xn--0zwm56d.xn--fiqs8s").is_ok());
  assert!(verify_email_addr(b"user@\xff").is_err());
  assert!(EmailAddr::<String>::try_from("user@测试。中国").is_ok());
  assert!(EmailAddr::<String>::try_from("user@.测试").is_err());

  match EmailAddr::<&[u8]>::try_from_bytes(b"user@xn--0zwm56d.xn--fiqs8s".as_slice()).unwrap() {
    either::Either::Left(_) => panic!("ASCII A-label should be normalized before acceptance"),
    either::Either::Right(buf) => assert_eq!(buf.as_str(), "user@xn--0zwm56d.xn--fiqs8s"),
  }

  let non_standard = Options::new()
    .with_domain(DomainOptions::new().with_unicode(DomainUnicodePolicy::NonStandardUtf8));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@é.example", non_standard).is_ok());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@[é]", non_standard).is_err());
  assert!(
    EmailAddr::<Buffer, Relax>::parse_bytes_with_options(b"user@\xff", non_standard).is_err()
  );
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@é..example", non_standard).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@-é.example", non_standard).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@é-.example", non_standard).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@é_.example", non_standard).is_err());
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options("user@é example", non_standard).is_err());
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options("user@é\u{3002}example", non_standard).is_err()
  );
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options("user@\u{7f}.example", non_standard).is_err()
  );
  let too_long_domain = format!("user@{}", "é".repeat(127));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(&too_long_domain, non_standard).is_err());

  let long_local = "a".repeat(254);
  let push_full = format!("{long_local}@a");
  let relaxed_long_local = Options::new().with_limits(Limits::new().with_max_local_part_len(254));
  assert!(EmailAddr::<Buffer, Relax>::parse_with_options(&push_full, relaxed_long_local).is_err());

  let long_local = "a".repeat(250);
  let extend_full = format!("{long_local}@abcd");
  let relaxed_long_local = Options::new().with_limits(Limits::new().with_max_local_part_len(250));
  assert!(
    EmailAddr::<Buffer, Relax>::parse_with_options(&extend_full, relaxed_long_local).is_err()
  );
}

#[test]
fn covers_relaxed_policy_surface() {
  let strict = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let relaxed: EmailAddr<Buffer, Relax> = strict.into();
  assert_eq!(relaxed.local_part(), "user");
  assert_eq!(relaxed.domain_part(), "example.com");
  assert_eq!(relaxed.parts(), ("user", "example.com"));
  assert_eq!(relaxed.as_ref().copied().parts(), ("user", "example.com"));
  assert_eq!(relaxed.as_ref().cloned().parts(), ("user", "example.com"));

  let options = Options::new()
    .with_limits(Limits::new().with_max_local_part_len(96))
    .with_domain(DomainOptions::new().with_required_tld());
  let relaxed =
    EmailAddr::<Buffer, Relax>::parse_with_options("reply+long-local@example.com", options)
      .unwrap();
  assert_eq!(relaxed.parts(), ("reply+long-local", "example.com"));

  let bytes =
    EmailAddr::<Buffer, Relax>::parse_bytes_with_options(b"reply+long-local@example.com", options)
      .unwrap();
  assert_eq!(bytes.as_bytes(), b"reply+long-local@example.com");
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn covers_alloc_storage_conversions() {
  use std::{borrow::Cow, boxed::Box, rc::Rc, str::FromStr, sync::Arc, vec::Vec};

  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();

  assert_eq!(String::from(buffer), "user@example.com");
  assert_eq!(Box::<str>::from(buffer).as_ref(), "user@example.com");
  assert_eq!(Rc::<str>::from(buffer).as_ref(), "user@example.com");
  assert_eq!(Arc::<str>::from(buffer).as_ref(), "user@example.com");
  assert_eq!(Vec::<u8>::from(buffer), b"user@example.com");
  assert_eq!(Box::<[u8]>::from(buffer).as_ref(), b"user@example.com");
  assert_eq!(Rc::<[u8]>::from(buffer).as_ref(), b"user@example.com");
  assert_eq!(Arc::<[u8]>::from(buffer).as_ref(), b"user@example.com");
  assert_eq!(Cow::<str>::from(buffer).as_ref(), "user@example.com");
  assert_eq!(Cow::<[u8]>::from(buffer).as_ref(), b"user@example.com");

  let string = String::from("user@example.com");
  assert_eq!(
    EmailAddr::<String>::try_from(string).unwrap().as_str(),
    "user@example.com"
  );
  assert_eq!(
    EmailAddr::<Vec<u8>>::try_from(b"user@example.com".to_vec())
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  assert_eq!(
    EmailAddr::<String>::from_str("user@example.com")
      .unwrap()
      .as_str(),
    "user@example.com"
  );
  let parsed_buffer: EmailAddr<Buffer> = "user@example.com".parse().unwrap();
  assert_eq!(parsed_buffer.as_str(), "user@example.com");
  assert_eq!(
    EmailAddr::<Vec<u8>>::from_str("user@example.com")
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  assert_eq!(
    EmailAddr::<Buffer>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .domain_part()
      .as_inner(),
    &"xn--0zwm56d.xn--fiqs8s"
  );

  assert_eq!(
    EmailAddr::<Box<str>>::try_from("user@example.com")
      .unwrap()
      .as_str(),
    "user@example.com"
  );
  assert_eq!(
    EmailAddr::<Rc<str>>::try_from("user@example.com")
      .unwrap()
      .as_str(),
    "user@example.com"
  );
  assert_eq!(
    EmailAddr::<Arc<str>>::try_from("user@example.com")
      .unwrap()
      .as_str(),
    "user@example.com"
  );
  assert_eq!(
    EmailAddr::<Box<[u8]>>::try_from(b"user@example.com".as_slice())
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  assert_eq!(
    EmailAddr::<Rc<[u8]>>::try_from(b"user@example.com".as_slice())
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  assert_eq!(
    EmailAddr::<Arc<[u8]>>::try_from(b"user@example.com".as_slice())
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  let _: EmailAddr<Box<str>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<Rc<str>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<Arc<str>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<Box<[u8]>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<Rc<[u8]>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<Arc<[u8]>> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<String>::try_from(String::from("user@测试.中国"))
      .unwrap()
      .as_str(),
    "user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<Box<str>>::try_from("user@测试.中国")
      .unwrap()
      .as_str(),
    "user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<Box<[u8]>>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<Vec<u8>>::try_from("user@测试.中国")
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<Vec<u8>>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<Vec<u8>>::try_from("user@测试.中国".as_bytes().to_vec())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );

  let borrowed_cow: EmailAddr<Cow<'_, str>> = EmailAddr::try_from("user@example.com").unwrap();
  assert!(matches!(borrowed_cow.into_inner(), Cow::Borrowed(_)));
  let borrowed_bytes: EmailAddr<Cow<'_, [u8]>> =
    EmailAddr::try_from(b"user@example.com".as_slice()).unwrap();
  assert!(matches!(borrowed_bytes.into_inner(), Cow::Borrowed(_)));
  let borrowed_bytes_from_str: EmailAddr<Cow<'_, [u8]>> =
    EmailAddr::try_from("user@example.com").unwrap();
  assert!(matches!(
    borrowed_bytes_from_str.into_inner(),
    Cow::Borrowed(_)
  ));
  let owned_cow_str: EmailAddr<Cow<'_, str>> = EmailAddr::try_from("user@测试.中国").unwrap();
  assert!(matches!(owned_cow_str.into_inner(), Cow::Owned(_)));
  let owned_cow_bytes: EmailAddr<Cow<'_, [u8]>> =
    EmailAddr::try_from("user@测试.中国".as_bytes()).unwrap();
  assert!(matches!(owned_cow_bytes.into_inner(), Cow::Owned(_)));
  let owned_cow_bytes_from_str: EmailAddr<Cow<'_, [u8]>> =
    EmailAddr::try_from("user@测试.中国").unwrap();
  assert!(matches!(
    owned_cow_bytes_from_str.into_inner(),
    Cow::Owned(_)
  ));

  let options = Options::new();
  match EmailAddr::<String, Relax>::try_from_str_with_options(
    String::from("user@example.com"),
    options,
  )
  .unwrap()
  {
    either::Either::Left(addr) => assert_eq!(addr.as_str(), "user@example.com"),
    either::Either::Right(_) => panic!("ASCII address should preserve storage"),
  }
  match EmailAddr::<Vec<u8>, Relax>::try_from_bytes_with_options(
    "user@测试.中国".as_bytes().to_vec(),
    options,
  )
  .unwrap()
  {
    either::Either::Left(_) => panic!("Unicode domain should normalize"),
    either::Either::Right(addr) => assert_eq!(addr.domain_part(), "xn--0zwm56d.xn--fiqs8s"),
  }
  match EmailAddr::<String, Relax>::try_from_str_with_options(
    String::from("user@测试.中国"),
    options,
  )
  .unwrap()
  {
    either::Either::Left(_) => panic!("Unicode domain should normalize"),
    either::Either::Right(addr) => assert_eq!(addr.domain_part(), "xn--0zwm56d.xn--fiqs8s"),
  }
}

#[cfg(feature = "bytes_1")]
#[test]
fn covers_bytes_storage() {
  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();
  assert_eq!(bytes_1::Bytes::from(buffer).as_ref(), b"user@example.com");

  let bytes = bytes_1::Bytes::copy_from_slice(b"user@example.com");
  assert_eq!(
    EmailAddr::<bytes_1::Bytes>::try_from(bytes)
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  assert_eq!(
    EmailAddr::<bytes_1::Bytes>::try_from(b"user@example.com".as_slice())
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  let _: EmailAddr<bytes_1::Bytes> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<bytes_1::Bytes>::try_from("user@测试.中国")
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<bytes_1::Bytes>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<bytes_1::Bytes>::try_from(bytes_1::Bytes::copy_from_slice(
      "user@测试.中国".as_bytes()
    ))
    .unwrap()
    .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
}

#[cfg(feature = "tinyvec_1")]
#[test]
fn covers_tinyvec_storage() {
  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();
  let tiny: tinyvec_1::TinyVec<[u8; 32]> = buffer.into();
  assert_eq!(tiny.as_slice(), b"user@example.com");

  let owned = tinyvec_1::TinyVec::<[u8; 32]>::from(b"user@example.com".as_slice());
  assert_eq!(
    EmailAddr::<tinyvec_1::TinyVec<[u8; 32]>>::try_from(owned)
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  let _: EmailAddr<tinyvec_1::TinyVec<[u8; 32]>> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<tinyvec_1::TinyVec<[u8; 64]>>::try_from("user@测试.中国")
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<tinyvec_1::TinyVec<[u8; 64]>>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  let owned = tinyvec_1::TinyVec::<[u8; 64]>::from("user@测试.中国".as_bytes());
  assert_eq!(
    EmailAddr::<tinyvec_1::TinyVec<[u8; 64]>>::try_from(owned)
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
}

#[cfg(feature = "smallvec_1")]
#[test]
fn covers_smallvec_storage() {
  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();
  let small: smallvec_1::SmallVec<[u8; 32]> = buffer.into();
  assert_eq!(small.as_slice(), b"user@example.com");

  let owned = smallvec_1::SmallVec::<[u8; 32]>::from_slice(b"user@example.com");
  assert_eq!(
    EmailAddr::<smallvec_1::SmallVec<[u8; 32]>>::try_from(owned)
      .unwrap()
      .as_bytes(),
    b"user@example.com"
  );
  let _: EmailAddr<smallvec_1::SmallVec<[u8; 32]>> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<smallvec_1::SmallVec<[u8; 64]>>::try_from("user@测试.中国")
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<smallvec_1::SmallVec<[u8; 64]>>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
  let owned = smallvec_1::SmallVec::<[u8; 64]>::from_slice("user@测试.中国".as_bytes());
  assert_eq!(
    EmailAddr::<smallvec_1::SmallVec<[u8; 64]>>::try_from(owned)
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
}

#[cfg(feature = "smol_str_0_3")]
#[test]
fn covers_smol_str_storage() {
  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();
  assert_eq!(
    smol_str_0_3::SmolStr::from(buffer).as_str(),
    "user@example.com"
  );
  assert_eq!(
    EmailAddr::<smol_str_0_3::SmolStr>::try_from("user@example.com")
      .unwrap()
      .as_str(),
    "user@example.com"
  );
  let _: EmailAddr<smol_str_0_3::SmolStr> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<smol_str_0_3::SmolStr>::try_from("user@测试.中国")
      .unwrap()
      .as_str(),
    "user@xn--0zwm56d.xn--fiqs8s"
  );
}

#[cfg(feature = "triomphe_0_1")]
#[test]
fn covers_triomphe_storage() {
  let addr = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  let buffer = *addr.as_inner();
  assert_eq!(
    triomphe_0_1::Arc::<str>::from(buffer).as_ref(),
    "user@example.com"
  );
  assert_eq!(
    triomphe_0_1::Arc::<[u8]>::from(buffer).as_ref(),
    b"user@example.com"
  );
  let _: EmailAddr<triomphe_0_1::Arc<str>> = "user@example.com".parse().unwrap();
  let _: EmailAddr<triomphe_0_1::Arc<[u8]>> = "user@example.com".parse().unwrap();
  assert_eq!(
    EmailAddr::<triomphe_0_1::Arc<str>>::try_from("user@测试.中国")
      .unwrap()
      .as_str(),
    "user@xn--0zwm56d.xn--fiqs8s"
  );
  assert_eq!(
    EmailAddr::<triomphe_0_1::Arc<[u8]>>::try_from("user@测试.中国".as_bytes())
      .unwrap()
      .as_bytes(),
    b"user@xn--0zwm56d.xn--fiqs8s"
  );
}

#[cfg(all(feature = "arbitrary", any(feature = "alloc", feature = "std")))]
#[test]
fn covers_arbitrary_generators() {
  use arbitrary::{Arbitrary, Unstructured};
  use std::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc, vec::Vec};

  fn with_generated<T>()
  where
    for<'a> T: Arbitrary<'a>,
  {
    for seed in 0u8..=u8::MAX {
      let data = [seed; 256];
      let mut unstructured = Unstructured::new(&data);
      if T::arbitrary(&mut unstructured).is_ok() {
        return;
      }
    }
    panic!("failed to generate arbitrary value");
  }

  with_generated::<EmailAddr<Buffer>>();
  with_generated::<EmailAddr<String>>();
  with_generated::<EmailAddr<Vec<u8>>>();
  with_generated::<EmailAddr<Box<str>>>();
  with_generated::<EmailAddr<Box<[u8]>>>();
  with_generated::<EmailAddr<Rc<str>>>();
  with_generated::<EmailAddr<Rc<[u8]>>>();
  with_generated::<EmailAddr<Arc<str>>>();
  with_generated::<EmailAddr<Arc<[u8]>>>();
  for seed in 0u32..512 {
    let mut state = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    let mut data = [0u8; 512];
    for byte in &mut data {
      state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      *byte = (state >> 24) as u8;
    }
    let mut unstructured = Unstructured::new(&data);
    let _ = EmailAddr::<Buffer>::arbitrary(&mut unstructured);
  }
  for seed in 0u8..=u8::MAX {
    let data = [seed; 256];
    let mut unstructured = Unstructured::new(&data);
    if EmailAddr::<Cow<'_, str>>::arbitrary(&mut unstructured).is_ok() {
      break;
    }
    assert_ne!(seed, u8::MAX, "failed to generate arbitrary Cow<str>");
  }
  for seed in 0u8..=u8::MAX {
    let data = [seed; 256];
    let mut unstructured = Unstructured::new(&data);
    if EmailAddr::<Cow<'_, [u8]>>::arbitrary(&mut unstructured).is_ok() {
      break;
    }
    assert_ne!(seed, u8::MAX, "failed to generate arbitrary Cow<[u8]>");
  }

  #[cfg(feature = "bytes_1")]
  with_generated::<EmailAddr<bytes_1::Bytes>>();
  #[cfg(feature = "tinyvec_1")]
  with_generated::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>();
  #[cfg(feature = "smallvec_1")]
  with_generated::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>();
  #[cfg(feature = "smol_str_0_3")]
  with_generated::<EmailAddr<smol_str_0_3::SmolStr>>();
  #[cfg(feature = "triomphe_0_1")]
  {
    with_generated::<EmailAddr<triomphe_0_1::Arc<str>>>();
    with_generated::<EmailAddr<triomphe_0_1::Arc<[u8]>>>();
  }
}

#[cfg(all(feature = "quickcheck", any(feature = "alloc", feature = "std")))]
#[test]
fn covers_quickcheck_generators() {
  use quickcheck::{Arbitrary, Gen};
  use std::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc, vec::Vec};

  fn generated<T>() -> T
  where
    T: Arbitrary,
  {
    T::arbitrary(&mut Gen::new(32))
  }

  assert!(generated::<EmailAddr<Buffer>>().as_str().contains('@'));
  assert!(generated::<EmailAddr<String>>().as_str().contains('@'));
  assert!(generated::<EmailAddr<Vec<u8>>>().as_bytes().contains(&b'@'));
  assert!(generated::<EmailAddr<Box<str>>>().as_str().contains('@'));
  assert!(
    generated::<EmailAddr<Box<[u8]>>>()
      .as_bytes()
      .contains(&b'@')
  );
  assert!(generated::<EmailAddr<Rc<str>>>().as_str().contains('@'));
  assert!(
    generated::<EmailAddr<Rc<[u8]>>>()
      .as_bytes()
      .contains(&b'@')
  );
  assert!(generated::<EmailAddr<Arc<str>>>().as_str().contains('@'));
  assert!(
    generated::<EmailAddr<Arc<[u8]>>>()
      .as_bytes()
      .contains(&b'@')
  );
  assert!(
    generated::<EmailAddr<Cow<'static, str>>>()
      .as_str()
      .contains('@')
  );
  assert!(
    generated::<EmailAddr<Cow<'static, [u8]>>>()
      .as_bytes()
      .contains(&b'@')
  );

  #[cfg(feature = "bytes_1")]
  assert!(
    generated::<EmailAddr<bytes_1::Bytes>>()
      .as_bytes()
      .contains(&b'@')
  );
  #[cfg(feature = "tinyvec_1")]
  assert!(
    generated::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>()
      .as_bytes()
      .contains(&b'@')
  );
  #[cfg(feature = "smallvec_1")]
  assert!(
    generated::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>()
      .as_bytes()
      .contains(&b'@')
  );
  #[cfg(feature = "smol_str_0_3")]
  assert!(
    generated::<EmailAddr<smol_str_0_3::SmolStr>>()
      .as_str()
      .contains('@')
  );
  #[cfg(feature = "triomphe_0_1")]
  {
    assert!(
      generated::<EmailAddr<triomphe_0_1::Arc<str>>>()
        .as_str()
        .contains('@')
    );
    assert!(
      generated::<EmailAddr<triomphe_0_1::Arc<[u8]>>>()
        .as_bytes()
        .contains(&b'@')
    );
  }
}

#[cfg(feature = "serde")]
mod serde_coverage {
  use core::fmt;

  use emailaddr::{Buffer, DomainPart, EmailAddr, LocalPart};
  use serde_core::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{
      self, Unexpected, Visitor,
      value::{Error as DeError, StrDeserializer},
    },
    ser::{Error as SerErrorTrait, Impossible},
  };

  #[cfg(any(feature = "alloc", feature = "std"))]
  use serde_core::de::value::BorrowedStrDeserializer;

  #[derive(Debug)]
  struct SerError(String);

  impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str(&self.0)
    }
  }

  impl std::error::Error for SerError {}

  impl de::Error for SerError {
    fn custom<T>(msg: T) -> Self
    where
      T: fmt::Display,
    {
      Self(msg.to_string())
    }
  }

  impl SerErrorTrait for SerError {
    fn custom<T>(msg: T) -> Self
    where
      T: fmt::Display,
    {
      Self(msg.to_string())
    }
  }

  struct StrSerializer;

  struct ExpectingDeserializer;

  impl<'de> Deserializer<'de> for ExpectingDeserializer {
    type Error = SerError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      Err(de::Error::invalid_type(Unexpected::Unit, &visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
      V: Visitor<'de>,
    {
      Err(de::Error::invalid_type(Unexpected::Unit, &visitor))
    }

    serde_core::forward_to_deserialize_any! {
      bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf
      option unit unit_struct newtype_struct seq tuple tuple_struct map struct
      enum identifier ignored_any
    }
  }

  macro_rules! reject {
    ($($name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {
      $(
        fn $name(self, $($arg: $ty),*) -> Result<$ret, Self::Error> {
          let _ = ($($arg),*);
          Err(<Self::Error as SerErrorTrait>::custom("expected string"))
        }
      )*
    };
  }

  impl Serializer for StrSerializer {
    type Ok = String;
    type Error = SerError;
    type SerializeSeq = Impossible<String, SerError>;
    type SerializeTuple = Impossible<String, SerError>;
    type SerializeTupleStruct = Impossible<String, SerError>;
    type SerializeTupleVariant = Impossible<String, SerError>;
    type SerializeMap = Impossible<String, SerError>;
    type SerializeStruct = Impossible<String, SerError>;
    type SerializeStructVariant = Impossible<String, SerError>;

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
      Ok(value.to_owned())
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + fmt::Display,
    {
      Ok(value.to_string())
    }

    reject! {
      serialize_bool(value: bool) -> String;
      serialize_i8(value: i8) -> String;
      serialize_i16(value: i16) -> String;
      serialize_i32(value: i32) -> String;
      serialize_i64(value: i64) -> String;
      serialize_u8(value: u8) -> String;
      serialize_u16(value: u16) -> String;
      serialize_u32(value: u32) -> String;
      serialize_u64(value: u64) -> String;
      serialize_f32(value: f32) -> String;
      serialize_f64(value: f64) -> String;
      serialize_char(value: char) -> String;
      serialize_bytes(value: &[u8]) -> String;
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize,
    {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_unit_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_newtype_struct<T>(
      self,
      _name: &'static str,
      _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
      T: ?Sized + Serialize,
    {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
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
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_tuple_struct(
      self,
      _name: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_tuple_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_struct(
      self,
      _name: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }

    fn serialize_struct_variant(
      self,
      _name: &'static str,
      _variant_index: u32,
      _variant: &'static str,
      _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
      Err(<Self::Error as SerErrorTrait>::custom("expected string"))
    }
  }

  fn serialize_to_string<T>(value: &T) -> String
  where
    T: ?Sized + Serialize,
  {
    value.serialize(StrSerializer).unwrap()
  }

  #[test]
  fn covers_serde_serialization() {
    let stack = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
    assert_eq!(serialize_to_string(&stack), "user@example.com");
    assert_eq!(serialize_to_string(&stack.as_ref()), "user@example.com");
    assert_eq!(
      serialize_to_string(EmailAddr::<[u8]>::try_from_ascii_bytes(b"user@example.com").unwrap()),
      "user@example.com"
    );

    let local = LocalPart::try_from_ascii_str("user").unwrap();
    assert_eq!(serialize_to_string(local), "user");
    assert_eq!(serialize_to_string(&local.as_ref()), "user");
    assert_eq!(serialize_to_string(local.as_bytes()), "user");

    let domain = DomainPart::try_from_ascii_str("example.com").unwrap();
    assert_eq!(serialize_to_string(domain), "example.com");
    assert_eq!(serialize_to_string(&domain.as_ref()), "example.com");
    assert_eq!(serialize_to_string(domain.as_bytes()), "example.com");

    #[cfg(any(feature = "alloc", feature = "std"))]
    {
      use std::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc, vec::Vec};

      assert_eq!(
        serialize_to_string(&EmailAddr::<String>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Vec<u8>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Box<str>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Box<[u8]>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Rc<str>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Rc<[u8]>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Arc<str>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Arc<[u8]>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Cow<'_, str>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(&EmailAddr::<Cow<'_, [u8]>>::try_from("user@example.com").unwrap()),
        "user@example.com"
      );
    }

    #[cfg(feature = "bytes_1")]
    assert_eq!(
      serialize_to_string(&EmailAddr::<bytes_1::Bytes>::try_from("user@example.com").unwrap()),
      "user@example.com"
    );
    #[cfg(feature = "tinyvec_1")]
    assert_eq!(
      serialize_to_string(
        &EmailAddr::<tinyvec_1::TinyVec<[u8; 32]>>::try_from("user@example.com").unwrap()
      ),
      "user@example.com"
    );
    #[cfg(feature = "smallvec_1")]
    assert_eq!(
      serialize_to_string(
        &EmailAddr::<smallvec_1::SmallVec<[u8; 32]>>::try_from("user@example.com").unwrap()
      ),
      "user@example.com"
    );
    #[cfg(feature = "smol_str_0_3")]
    assert_eq!(
      serialize_to_string(
        &EmailAddr::<smol_str_0_3::SmolStr>::try_from("user@example.com").unwrap()
      ),
      "user@example.com"
    );
    #[cfg(feature = "triomphe_0_1")]
    {
      assert_eq!(
        serialize_to_string(
          &EmailAddr::<triomphe_0_1::Arc<str>>::try_from("user@example.com").unwrap()
        ),
        "user@example.com"
      );
      assert_eq!(
        serialize_to_string(
          &EmailAddr::<triomphe_0_1::Arc<[u8]>>::try_from("user@example.com").unwrap()
        ),
        "user@example.com"
      );
    }
  }

  #[test]
  fn covers_serde_deserialization() {
    fn from_str<'de, T>(value: &'de str) -> T
    where
      T: Deserialize<'de>,
    {
      T::deserialize(StrDeserializer::<DeError>::new(value)).unwrap()
    }

    let stack: EmailAddr<Buffer> = from_str("user@example.com");
    assert_eq!(stack.as_str(), "user@example.com");
    assert!(
      EmailAddr::<Buffer>::deserialize(ExpectingDeserializer)
        .unwrap_err()
        .to_string()
        .contains("a valid email address string")
    );

    #[cfg(any(feature = "alloc", feature = "std"))]
    {
      use std::{borrow::Cow, boxed::Box, rc::Rc, sync::Arc, vec::Vec};

      assert_eq!(
        from_str::<EmailAddr<String>>("user@example.com").as_str(),
        "user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Vec<u8>>>("user@example.com").as_bytes(),
        b"user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Box<str>>>("user@example.com").as_str(),
        "user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Box<[u8]>>>("user@example.com").as_bytes(),
        b"user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Rc<str>>>("user@example.com").as_str(),
        "user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Rc<[u8]>>>("user@example.com").as_bytes(),
        b"user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Arc<str>>>("user@example.com").as_str(),
        "user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<Arc<[u8]>>>("user@example.com").as_bytes(),
        b"user@example.com"
      );

      let borrowed = BorrowedStrDeserializer::<DeError>::new("user@example.com");
      let cow_str = EmailAddr::<Cow<'_, str>>::deserialize(borrowed).unwrap();
      assert!(matches!(cow_str.into_inner(), Cow::Borrowed(_)));

      let borrowed = BorrowedStrDeserializer::<DeError>::new("user@example.com");
      let cow_bytes = EmailAddr::<Cow<'_, [u8]>>::deserialize(borrowed).unwrap();
      assert!(matches!(cow_bytes.into_inner(), Cow::Borrowed(_)));

      let owned = StrDeserializer::<DeError>::new("user@example.com");
      let cow_str = EmailAddr::<Cow<'_, str>>::deserialize(owned).unwrap();
      assert!(matches!(cow_str.into_inner(), Cow::Owned(_)));

      let owned = StrDeserializer::<DeError>::new("user@example.com");
      let cow_bytes = EmailAddr::<Cow<'_, [u8]>>::deserialize(owned).unwrap();
      assert!(matches!(cow_bytes.into_inner(), Cow::Owned(_)));

      assert!(
        EmailAddr::<Cow<'_, str>>::deserialize(ExpectingDeserializer)
          .unwrap_err()
          .to_string()
          .contains("a valid email address string")
      );
      assert!(
        EmailAddr::<Cow<'_, [u8]>>::deserialize(ExpectingDeserializer)
          .unwrap_err()
          .to_string()
          .contains("a valid email address string")
      );
    }

    #[cfg(feature = "bytes_1")]
    assert_eq!(
      from_str::<EmailAddr<bytes_1::Bytes>>("user@example.com").as_bytes(),
      b"user@example.com"
    );
    #[cfg(feature = "tinyvec_1")]
    assert_eq!(
      from_str::<EmailAddr<tinyvec_1::TinyVec<[u8; 32]>>>("user@example.com").as_bytes(),
      b"user@example.com"
    );
    #[cfg(feature = "smallvec_1")]
    assert_eq!(
      from_str::<EmailAddr<smallvec_1::SmallVec<[u8; 32]>>>("user@example.com").as_bytes(),
      b"user@example.com"
    );
    #[cfg(feature = "smol_str_0_3")]
    assert_eq!(
      from_str::<EmailAddr<smol_str_0_3::SmolStr>>("user@example.com").as_str(),
      "user@example.com"
    );
    #[cfg(feature = "triomphe_0_1")]
    {
      assert_eq!(
        from_str::<EmailAddr<triomphe_0_1::Arc<str>>>("user@example.com").as_str(),
        "user@example.com"
      );
      assert_eq!(
        from_str::<EmailAddr<triomphe_0_1::Arc<[u8]>>>("user@example.com").as_bytes(),
        b"user@example.com"
      );
    }
  }
}
