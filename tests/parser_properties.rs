#![cfg(feature = "quickcheck")]

use std::{format, string::String, vec::Vec};

use emailaddr::{
  Buffer, DomainPart, EmailAddr, LocalPart, verify_ascii_dns_domain, verify_ascii_domain_part,
  verify_ascii_email_addr, verify_ascii_local_part, verify_email_addr, verify_local_part,
};
use quickcheck::{QuickCheck, Testable};

fn run_property<T: Testable>(property: T) {
  let tests = std::env::var("QUICKCHECK_TESTS")
    .ok()
    .and_then(|value| value.parse().ok())
    .unwrap_or(5_000);
  QuickCheck::new().tests(tests).quickcheck(property);
}

fn arbitrary_bytes_are_panic_free(input: Vec<u8>) -> bool {
  let _ = verify_ascii_email_addr(&input);
  let _ = verify_email_addr(&input);
  let _ = verify_ascii_local_part(&input);
  let _ = verify_local_part(&input);
  let _ = verify_ascii_domain_part(&input);
  let _ = verify_ascii_dns_domain(&input);
  let _ = EmailAddr::<Buffer>::try_from(input.as_slice());
  let _ = EmailAddr::<[u8]>::try_from_ascii_bytes(input.as_slice());
  let _ = LocalPart::<&[u8]>::try_from(input.as_slice());
  let _ = DomainPart::<&[u8]>::try_from(input.as_slice());
  true
}

fn generated_buffer_addresses_roundtrip(address: EmailAddr<Buffer>) -> bool {
  let text = address.as_str().to_owned();
  let reparsed = EmailAddr::<Buffer>::try_from(text.as_str()).unwrap();
  reparsed == address && reparsed.as_str() == text
}

#[test]
fn arbitrary_bytes_do_not_panic() {
  run_property(arbitrary_bytes_are_panic_free as fn(Vec<u8>) -> bool);
}

#[test]
fn generated_buffer_addresses_roundtrip_and_are_idempotent() {
  run_property(generated_buffer_addresses_roundtrip as fn(EmailAddr<Buffer>) -> bool);
}

#[test]
fn normalization_corpus_is_deterministic_and_idempotent() {
  for (input, expected) in [
    ("user@测试.中国", "user@xn--0zwm56d.xn--fiqs8s"),
    ("用户@example.com", "用户@example.com"),
    ("a@bücher.example", "a@xn--bcher-kva.example"),
  ] {
    let address: EmailAddr<String> = EmailAddr::try_from(input).unwrap();
    assert_eq!(address.as_str(), expected);

    let reparsed: EmailAddr<String> = EmailAddr::try_from(address.as_str()).unwrap();
    assert_eq!(reparsed.as_str(), expected);
  }
}

fn domain_with_len(length: usize) -> String {
  assert!((193..=254).contains(&length));
  let tail_len = length - 192;
  format!(
    "{}.{}.{}.{}",
    "a".repeat(63),
    "b".repeat(63),
    "c".repeat(63),
    "d".repeat(tail_len),
  )
}

#[test]
fn parser_boundaries_cover_lengths_and_utf8_bytes() {
  let local64 = "a".repeat(64);
  let local65 = "a".repeat(65);
  assert!(verify_ascii_local_part(local64.as_bytes()).is_ok());
  assert!(verify_ascii_local_part(local65.as_bytes()).is_err());

  let label63 = format!("{}.com", "a".repeat(63));
  let label64 = format!("{}.com", "a".repeat(64));
  assert!(verify_ascii_dns_domain(label63.as_bytes()).is_ok());
  assert!(verify_ascii_dns_domain(label64.as_bytes()).is_err());

  let domain253 = domain_with_len(253);
  let domain254 = domain_with_len(254);
  assert!(verify_ascii_dns_domain(domain253.as_bytes()).is_ok());
  assert!(verify_ascii_dns_domain(domain254.as_bytes()).is_err());

  let address254 = format!("a@{}", domain_with_len(252));
  let address255 = format!("a@{}", domain_with_len(253));
  assert_eq!(address254.len(), 254);
  assert_eq!(address255.len(), 255);
  assert!(EmailAddr::<Buffer>::try_from(address254.as_str()).is_ok());
  assert!(EmailAddr::<Buffer>::try_from(address255.as_str()).is_err());

  let utf8_64 = "é".repeat(32);
  let utf8_65 = format!("{}a", utf8_64);
  assert_eq!(utf8_64.len(), 64);
  assert_eq!(utf8_65.len(), 65);
  assert!(verify_local_part(utf8_64.as_bytes()).is_ok());
  assert!(verify_local_part(utf8_65.as_bytes()).is_err());
  assert!(verify_local_part(&[0xff]).is_err());
}
