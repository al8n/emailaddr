use emailaddr::{Buffer, EmailAddr};

fn main() {
  let addr: EmailAddr<Buffer> = "user@example.com".parse().unwrap();
  println!("{} at {}", addr.local_part(), addr.domain_part());
}
