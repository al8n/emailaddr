<div align="center">
<h1>emailaddr</h1>
</div>
<div align="center">

面向 Rust 的类型安全、已验证电子邮件地址类型。

[<img alt="github" src="https://img.shields.io/badge/github-al8n/emailaddr-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
[<img alt="Build" src="https://img.shields.io/github/actions/workflow/status/al8n/emailaddr/ci.yml?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-emailaddr-66c2a5?style=for-the-badge" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/emailaddr?style=for-the-badge" height="22">][crates-url]

[English][en-url] | 简体中文

</div>

## 概览

`emailaddr` 提供与 `hostaddr` 类似的存储架构：

- `EmailAddr<S>` 使用调用方选择的存储类型保存完整地址。
- `LocalPart<S>` 验证 dot-atom 和 quoted-string local-part。
- `DomainPart<S>` 验证 RFC 5321 domain-part 和带方括号的 address literal。
- `Buffer` 在栈上保存完整地址，适合 `no_std` / no-allocation 场景。

默认 `std` feature 支持 SMTPUTF8 local-part，并会把 Unicode domain-part
规范化为 IDNA/punycode。`verify_ascii_*` 辅助函数仍保持严格 ASCII 验证。

可选 `serde` feature 使用 `serde_core` 为已验证的地址值提供字符串序列化
和反序列化支持；与 `alloc` 或 `std` 组合使用时，反序列化会执行 IDNA A-label
验证。

## 安装

```toml
[dependencies]
emailaddr = "0.1"
```

## 快速开始

```rust
use emailaddr::EmailAddr;

let addr: EmailAddr<String> = "user.name@example.com".parse().unwrap();
assert_eq!(addr.local_part().as_inner(), &"user.name");
assert_eq!(addr.domain_part().as_inner(), &"example.com");

let idn: EmailAddr<String> = "user@测试.中国".parse().unwrap();
assert_eq!(idn.as_str(), "user@xn--0zwm56d.xn--fiqs8s");

let smtp_utf8: EmailAddr<String> = "用户@example.com".parse().unwrap();
assert_eq!(smtp_utf8.local_part().as_inner(), &"用户");
```

## License

`emailaddr` 使用 MIT OR Apache-2.0 双许可证。

[Github-url]: https://github.com/al8n/emailaddr/
[CI-url]: https://github.com/al8n/emailaddr/actions/workflows/ci.yml
[doc-url]: https://docs.rs/emailaddr
[crates-url]: https://crates.io/crates/emailaddr
[en-url]: https://github.com/al8n/emailaddr/tree/main/README.md
