use emailaddr::EmailAddr;

fn main() {
  let addr: EmailAddr<String> = "user@example.com".parse().unwrap();
  println!("{} at {}", addr.local_part(), addr.domain_part());
}
