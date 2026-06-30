use crate::MAX_LOCAL_PART_LENGTH;

/// Default local-part length limit used by [`Limits`].
pub const DEFAULT_MAX_LOCAL_PART_LENGTH: usize = MAX_LOCAL_PART_LENGTH;

/// Default minimum DNS label count used by [`DomainOptions`].
pub const DEFAULT_MINIMUM_DNS_LABELS: usize = 1;

/// Email address parsing options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Options {
  #[cfg_attr(feature = "clap", command(flatten))]
  local: LocalOptions,
  #[cfg_attr(feature = "clap", command(flatten))]
  domain: DomainOptions,
  #[cfg_attr(feature = "clap", command(flatten))]
  limits: Limits,
}

impl Default for Options {
  #[cfg_attr(not(coverage), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl Options {
  /// Default RFC-compatible parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn new() -> Self {
    Self {
      local: LocalOptions::new(),
      domain: DomainOptions::new(),
      limits: Limits::new(),
    }
  }

  /// Returns local-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn local(&self) -> LocalOptions {
    self.local
  }

  /// Sets local-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_local(&mut self, local: LocalOptions) -> &mut Self {
    self.local = local;
    self
  }

  /// Returns these options with local-part parsing options changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_local(mut self, local: LocalOptions) -> Self {
    self.set_local(local);
    self
  }

  /// Returns domain-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn domain(&self) -> DomainOptions {
    self.domain
  }

  /// Sets domain-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_domain(&mut self, domain: DomainOptions) -> &mut Self {
    self.domain = domain;
    self
  }

  /// Returns these options with domain-part parsing options changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_domain(mut self, domain: DomainOptions) -> Self {
    self.set_domain(domain);
    self
  }

  /// Returns parsing limits.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn limits(&self) -> Limits {
    self.limits
  }

  /// Sets parsing limits.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_limits(&mut self, limits: Limits) -> &mut Self {
    self.limits = limits;
    self
  }

  /// Returns these options with parsing limits changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_limits(mut self, limits: Limits) -> Self {
    self.set_limits(limits);
    self
  }
}

/// Local-part parsing options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct LocalOptions {
  #[cfg_attr(feature = "clap", arg(
    id = "email-local-smtp-utf8",
    long = "email-local-smtp-utf8",
    value_enum,
    default_value_t = SmtpUtf8Policy::Allow,
  ))]
  smtp_utf8: SmtpUtf8Policy,
}

impl Default for LocalOptions {
  #[cfg_attr(not(coverage), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl LocalOptions {
  /// Default local-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn new() -> Self {
    Self {
      smtp_utf8: SmtpUtf8Policy::Allow,
    }
  }

  /// Returns the SMTPUTF8 policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn smtp_utf8(&self) -> SmtpUtf8Policy {
    self.smtp_utf8
  }

  /// Sets the SMTPUTF8 policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_smtp_utf8(&mut self, smtp_utf8: SmtpUtf8Policy) -> &mut Self {
    self.smtp_utf8 = smtp_utf8;
    self
  }

  /// Returns these options with the SMTPUTF8 policy changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_smtp_utf8(mut self, smtp_utf8: SmtpUtf8Policy) -> Self {
    self.set_smtp_utf8(smtp_utf8);
    self
  }
}

/// Domain-part parsing options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct DomainOptions {
  #[cfg_attr(feature = "clap", arg(
    id = "email-domain-minimum-dns-labels",
    long = "email-domain-minimum-dns-labels",
    default_value_t = DEFAULT_MINIMUM_DNS_LABELS,
  ))]
  minimum_dns_labels: usize,
  #[cfg_attr(feature = "clap", arg(
    id = "email-domain-literals",
    long = "email-domain-literals",
    value_enum,
    default_value_t = DomainLiteralPolicy::Allow,
  ))]
  literals: DomainLiteralPolicy,
  #[cfg_attr(feature = "clap", arg(
    id = "email-domain-unicode",
    long = "email-domain-unicode",
    value_enum,
    default_value_t = DomainUnicodePolicy::Idna,
  ))]
  unicode: DomainUnicodePolicy,
}

impl Default for DomainOptions {
  #[cfg_attr(not(coverage), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl DomainOptions {
  /// Default domain-part parsing options.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn new() -> Self {
    Self {
      minimum_dns_labels: DEFAULT_MINIMUM_DNS_LABELS,
      literals: DomainLiteralPolicy::Allow,
      unicode: DomainUnicodePolicy::Idna,
    }
  }

  /// Returns the minimum DNS label count.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn minimum_dns_labels(&self) -> usize {
    self.minimum_dns_labels
  }

  /// Sets the minimum DNS label count.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_minimum_dns_labels(&mut self, minimum_dns_labels: usize) -> &mut Self {
    self.minimum_dns_labels = minimum_dns_labels;
    self
  }

  /// Returns these options with the minimum DNS label count changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_minimum_dns_labels(mut self, minimum_dns_labels: usize) -> Self {
    self.set_minimum_dns_labels(minimum_dns_labels);
    self
  }

  /// Returns these options with no extra DNS label count requirement.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_no_minimum_dns_labels(mut self) -> Self {
    self.set_minimum_dns_labels(0);
    self
  }

  /// Returns these options requiring at least two DNS labels.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_required_tld(mut self) -> Self {
    self.set_minimum_dns_labels(2);
    self
  }

  /// Returns the domain literal policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn literals(&self) -> DomainLiteralPolicy {
    self.literals
  }

  /// Sets the domain literal policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_literals(&mut self, literals: DomainLiteralPolicy) -> &mut Self {
    self.literals = literals;
    self
  }

  /// Returns these options with the domain literal policy changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_literals(mut self, literals: DomainLiteralPolicy) -> Self {
    self.set_literals(literals);
    self
  }

  /// Returns these options allowing address-literal domains.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_domain_literals(mut self) -> Self {
    self.set_literals(DomainLiteralPolicy::Allow);
    self
  }

  /// Returns these options rejecting address-literal domains.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn without_domain_literals(mut self) -> Self {
    self.set_literals(DomainLiteralPolicy::Forbid);
    self
  }

  /// Returns the Unicode domain policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn unicode(&self) -> DomainUnicodePolicy {
    self.unicode
  }

  /// Sets the Unicode domain policy.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_unicode(&mut self, unicode: DomainUnicodePolicy) -> &mut Self {
    self.unicode = unicode;
    self
  }

  /// Returns these options with the Unicode domain policy changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_unicode(mut self, unicode: DomainUnicodePolicy) -> Self {
    self.set_unicode(unicode);
    self
  }
}

/// Parsing limits.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Limits {
  #[cfg_attr(feature = "clap", arg(
    id = "email-limits-max-local-part-len",
    long = "email-limits-max-local-part-len",
    default_value_t = DEFAULT_MAX_LOCAL_PART_LENGTH,
  ))]
  max_local_part_len: usize,
}

impl Default for Limits {
  #[cfg_attr(not(coverage), inline(always))]
  fn default() -> Self {
    Self::new()
  }
}

impl Limits {
  /// Default RFC-compatible parsing limits.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn new() -> Self {
    Self {
      max_local_part_len: DEFAULT_MAX_LOCAL_PART_LENGTH,
    }
  }

  /// Returns the maximum local-part length in bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn max_local_part_len(&self) -> usize {
    self.max_local_part_len
  }

  /// Sets the maximum local-part length in bytes.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn set_max_local_part_len(&mut self, max_local_part_len: usize) -> &mut Self {
    self.max_local_part_len = max_local_part_len;
    self
  }

  /// Returns these limits with the maximum local-part length changed.
  #[must_use]
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn with_max_local_part_len(mut self, max_local_part_len: usize) -> Self {
    self.set_max_local_part_len(max_local_part_len);
    self
  }
}

/// Policy for SMTPUTF8 local-parts.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[non_exhaustive]
pub enum SmtpUtf8Policy {
  /// Accept UTF-8 local-parts.
  #[default]
  #[cfg_attr(feature = "clap", value(name = "allow"))]
  Allow,
  /// Reject non-ASCII local-parts.
  #[cfg_attr(feature = "clap", value(name = "forbid"))]
  Forbid,
}

impl SmtpUtf8Policy {
  /// Returns the stable policy name.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Allow => "allow",
      Self::Forbid => "forbid",
    }
  }

  /// Returns `true` if this policy accepts UTF-8 local-parts.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_allow(&self) -> bool {
    matches!(self, Self::Allow)
  }

  /// Returns `true` if this policy rejects non-ASCII local-parts.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_forbid(&self) -> bool {
    matches!(self, Self::Forbid)
  }

  #[cfg(feature = "serde")]
  const fn as_u8(&self) -> u8 {
    match self {
      Self::Allow => 0,
      Self::Forbid => 1,
    }
  }

  #[cfg(feature = "serde")]
  const fn from_u8(value: u8) -> Option<Self> {
    match value {
      0 => Some(Self::Allow),
      1 => Some(Self::Forbid),
      _ => None,
    }
  }

  #[cfg(feature = "serde")]
  fn from_str_name(value: &str) -> Option<Self> {
    match value {
      "allow" => Some(Self::Allow),
      "forbid" => Some(Self::Forbid),
      _ => None,
    }
  }
}

impl core::fmt::Display for SmtpUtf8Policy {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Policy for address-literal domain-parts.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[non_exhaustive]
pub enum DomainLiteralPolicy {
  /// Accept address-literal domains.
  #[default]
  #[cfg_attr(feature = "clap", value(name = "allow"))]
  Allow,
  /// Reject address-literal domains.
  #[cfg_attr(feature = "clap", value(name = "forbid"))]
  Forbid,
}

impl DomainLiteralPolicy {
  /// Returns the stable policy name.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Allow => "allow",
      Self::Forbid => "forbid",
    }
  }

  /// Returns `true` if address-literal domains are accepted.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_allow(&self) -> bool {
    matches!(self, Self::Allow)
  }

  /// Returns `true` if address-literal domains are rejected.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_forbid(&self) -> bool {
    matches!(self, Self::Forbid)
  }

  #[cfg(feature = "serde")]
  const fn as_u8(&self) -> u8 {
    match self {
      Self::Allow => 0,
      Self::Forbid => 1,
    }
  }

  #[cfg(feature = "serde")]
  const fn from_u8(value: u8) -> Option<Self> {
    match value {
      0 => Some(Self::Allow),
      1 => Some(Self::Forbid),
      _ => None,
    }
  }

  #[cfg(feature = "serde")]
  fn from_str_name(value: &str) -> Option<Self> {
    match value {
      "allow" => Some(Self::Allow),
      "forbid" => Some(Self::Forbid),
      _ => None,
    }
  }
}

impl core::fmt::Display for DomainLiteralPolicy {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Policy for Unicode domain input.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[non_exhaustive]
pub enum DomainUnicodePolicy {
  /// Reject non-ASCII domain input.
  #[cfg_attr(feature = "clap", value(name = "ascii", alias("ascii_only")))]
  AsciiOnly,
  /// Accept Unicode domain input only through IDNA normalization.
  #[default]
  #[cfg_attr(feature = "clap", value(name = "idna"))]
  Idna,
  /// Preserve non-ASCII UTF-8 DNS labels as non-standard application data.
  #[cfg_attr(
    feature = "clap",
    value(name = "raw", alias("raw_utf8"), alias("non_standard_utf8"))
  )]
  NonStandardUtf8,
}

impl DomainUnicodePolicy {
  /// Returns the stable policy name.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::AsciiOnly => "ascii",
      Self::Idna => "idna",
      Self::NonStandardUtf8 => "raw",
    }
  }

  /// Returns `true` if non-ASCII domain input is rejected.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_ascii_only(&self) -> bool {
    matches!(self, Self::AsciiOnly)
  }

  /// Returns `true` if Unicode domain input uses IDNA normalization.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_idna(&self) -> bool {
    matches!(self, Self::Idna)
  }

  /// Returns `true` if non-ASCII UTF-8 DNS labels are preserved.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn is_non_standard_utf8(&self) -> bool {
    matches!(self, Self::NonStandardUtf8)
  }

  #[cfg(feature = "serde")]
  const fn as_u8(&self) -> u8 {
    match self {
      Self::AsciiOnly => 0,
      Self::Idna => 1,
      Self::NonStandardUtf8 => 2,
    }
  }

  #[cfg(feature = "serde")]
  const fn from_u8(value: u8) -> Option<Self> {
    match value {
      0 => Some(Self::AsciiOnly),
      1 => Some(Self::Idna),
      2 => Some(Self::NonStandardUtf8),
      _ => None,
    }
  }

  #[cfg(feature = "serde")]
  fn from_str_name(value: &str) -> Option<Self> {
    match value {
      "ascii" | "ascii_only" => Some(Self::AsciiOnly),
      "idna" => Some(Self::Idna),
      "raw" | "raw_utf8" | "non_standard_utf8" => Some(Self::NonStandardUtf8),
      _ => None,
    }
  }
}

impl core::fmt::Display for DomainUnicodePolicy {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

#[cfg(feature = "serde")]
mod serde {
  use core::{fmt, marker::PhantomData};

  use serde_core::{
    de::{self, IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
  };

  use super::{
    DomainLiteralPolicy, DomainOptions, DomainUnicodePolicy, Limits, LocalOptions, Options,
    SmtpUtf8Policy,
  };

  trait UnitPolicy: Copy {
    const EXPECTING: &'static str;
    const VARIANTS: &'static [&'static str];

    fn as_name(&self) -> &'static str;
    fn as_discriminant(&self) -> u8;
    fn from_name(value: &str) -> Option<Self>;
    fn from_discriminant(value: u8) -> Option<Self>;
  }

  struct UnitPolicyVisitor<P>(PhantomData<fn() -> P>);

  impl<P> UnitPolicyVisitor<P> {
    const fn new() -> Self {
      Self(PhantomData)
    }
  }

  impl<P> Visitor<'_> for UnitPolicyVisitor<P>
  where
    P: UnitPolicy,
  {
    type Value = P;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str(P::EXPECTING)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      P::from_name(value).ok_or_else(|| E::unknown_variant(value, P::VARIANTS))
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      let value = core::str::from_utf8(value)
        .map_err(|_| E::invalid_value(Unexpected::Bytes(value), &self))?;
      self.visit_str(value)
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      P::from_discriminant(value)
        .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(u64::from(value)), &self))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      let Ok(value) = u8::try_from(value) else {
        return Err(E::invalid_value(Unexpected::Unsigned(value), &self));
      };
      self.visit_u8(value)
    }
  }

  fn serialize_unit_policy<P, S>(value: &P, serializer: S) -> Result<S::Ok, S::Error>
  where
    P: UnitPolicy,
    S: Serializer,
  {
    if serializer.is_human_readable() {
      serializer.serialize_str(value.as_name())
    } else {
      serializer.serialize_u8(value.as_discriminant())
    }
  }

  fn deserialize_unit_policy<'de, P, D>(deserializer: D) -> Result<P, D::Error>
  where
    P: UnitPolicy,
    D: Deserializer<'de>,
  {
    if deserializer.is_human_readable() {
      deserializer.deserialize_str(UnitPolicyVisitor::<P>::new())
    } else {
      deserializer.deserialize_u8(UnitPolicyVisitor::<P>::new())
    }
  }

  impl UnitPolicy for SmtpUtf8Policy {
    const EXPECTING: &'static str = "an SMTPUTF8 policy";
    const VARIANTS: &'static [&'static str] = &["allow", "forbid"];

    fn as_name(&self) -> &'static str {
      self.as_str()
    }

    fn as_discriminant(&self) -> u8 {
      self.as_u8()
    }

    fn from_name(value: &str) -> Option<Self> {
      Self::from_str_name(value)
    }

    fn from_discriminant(value: u8) -> Option<Self> {
      Self::from_u8(value)
    }
  }

  impl Serialize for SmtpUtf8Policy {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serialize_unit_policy(self, serializer)
    }
  }

  impl<'de> Deserialize<'de> for SmtpUtf8Policy {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserialize_unit_policy(deserializer)
    }
  }

  impl UnitPolicy for DomainLiteralPolicy {
    const EXPECTING: &'static str = "a domain literal policy";
    const VARIANTS: &'static [&'static str] = &["allow", "forbid"];

    fn as_name(&self) -> &'static str {
      self.as_str()
    }

    fn as_discriminant(&self) -> u8 {
      self.as_u8()
    }

    fn from_name(value: &str) -> Option<Self> {
      Self::from_str_name(value)
    }

    fn from_discriminant(value: u8) -> Option<Self> {
      Self::from_u8(value)
    }
  }

  impl Serialize for DomainLiteralPolicy {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serialize_unit_policy(self, serializer)
    }
  }

  impl<'de> Deserialize<'de> for DomainLiteralPolicy {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserialize_unit_policy(deserializer)
    }
  }

  impl UnitPolicy for DomainUnicodePolicy {
    const EXPECTING: &'static str = "a domain Unicode policy";
    const VARIANTS: &'static [&'static str] = &["ascii", "idna", "raw"];

    fn as_name(&self) -> &'static str {
      self.as_str()
    }

    fn as_discriminant(&self) -> u8 {
      self.as_u8()
    }

    fn from_name(value: &str) -> Option<Self> {
      Self::from_str_name(value)
    }

    fn from_discriminant(value: u8) -> Option<Self> {
      Self::from_u8(value)
    }
  }

  impl Serialize for DomainUnicodePolicy {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serialize_unit_policy(self, serializer)
    }
  }

  impl<'de> Deserialize<'de> for DomainUnicodePolicy {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserialize_unit_policy(deserializer)
    }
  }

  impl Serialize for Options {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut state = serializer.serialize_struct("Options", 3)?;
      state.serialize_field("local", &self.local())?;
      state.serialize_field("domain", &self.domain())?;
      state.serialize_field("limits", &self.limits())?;
      state.end()
    }
  }

  impl<'de> Deserialize<'de> for Options {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let human_readable = deserializer.is_human_readable();
      deserializer.deserialize_struct(
        "Options",
        &["local", "domain", "limits"],
        OptionsVisitor { human_readable },
      )
    }
  }

  enum OptionsField {
    Local,
    Domain,
    Limits,
  }

  struct OptionsFieldVisitor;

  impl Visitor<'_> for OptionsFieldVisitor {
    type Value = OptionsField;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("an Options field")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match value {
        "local" => Ok(OptionsField::Local),
        "domain" => Ok(OptionsField::Domain),
        "limits" => Ok(OptionsField::Limits),
        _ => Err(E::unknown_field(value, &["local", "domain", "limits"])),
      }
    }
  }

  impl<'de> Deserialize<'de> for OptionsField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_identifier(OptionsFieldVisitor)
    }
  }

  struct OptionsVisitor {
    human_readable: bool,
  }

  impl<'de> Visitor<'de> for OptionsVisitor {
    type Value = Options;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("email address parsing options")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      let mut local = None;
      let mut domain = None;
      let mut limits = None;
      while let Some(field) = access.next_key()? {
        match field {
          OptionsField::Local => {
            if local.is_some() {
              return Err(de::Error::duplicate_field("local"));
            }
            local = Some(access.next_value()?);
          }
          OptionsField::Domain => {
            if domain.is_some() {
              return Err(de::Error::duplicate_field("domain"));
            }
            domain = Some(access.next_value()?);
          }
          OptionsField::Limits => {
            if limits.is_some() {
              return Err(de::Error::duplicate_field("limits"));
            }
            limits = Some(access.next_value()?);
          }
        }
      }

      let (local, domain, limits) = if self.human_readable {
        (
          local.unwrap_or_default(),
          domain.unwrap_or_default(),
          limits.unwrap_or_default(),
        )
      } else {
        (
          local.ok_or_else(|| de::Error::missing_field("local"))?,
          domain.ok_or_else(|| de::Error::missing_field("domain"))?,
          limits.ok_or_else(|| de::Error::missing_field("limits"))?,
        )
      };

      Ok(
        Options::new()
          .with_local(local)
          .with_domain(domain)
          .with_limits(limits),
      )
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      if self.human_readable {
        return Err(de::Error::invalid_type(Unexpected::Seq, &self));
      }

      let local = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
      let domain = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
      let limits = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(2, &self))?;
      if access.next_element::<IgnoredAny>()?.is_some() {
        return Err(de::Error::invalid_length(4, &self));
      }

      Ok(
        Options::new()
          .with_local(local)
          .with_domain(domain)
          .with_limits(limits),
      )
    }
  }

  impl Serialize for LocalOptions {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut state = serializer.serialize_struct("LocalOptions", 1)?;
      state.serialize_field("smtp_utf8", &self.smtp_utf8())?;
      state.end()
    }
  }

  impl<'de> Deserialize<'de> for LocalOptions {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let human_readable = deserializer.is_human_readable();
      deserializer.deserialize_struct(
        "LocalOptions",
        &["smtp_utf8"],
        LocalOptionsVisitor { human_readable },
      )
    }
  }

  enum LocalOptionsField {
    SmtpUtf8,
  }

  struct LocalOptionsFieldVisitor;

  impl Visitor<'_> for LocalOptionsFieldVisitor {
    type Value = LocalOptionsField;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("a LocalOptions field")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match value {
        "smtp_utf8" => Ok(LocalOptionsField::SmtpUtf8),
        _ => Err(E::unknown_field(value, &["smtp_utf8"])),
      }
    }
  }

  impl<'de> Deserialize<'de> for LocalOptionsField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_identifier(LocalOptionsFieldVisitor)
    }
  }

  struct LocalOptionsVisitor {
    human_readable: bool,
  }

  impl<'de> Visitor<'de> for LocalOptionsVisitor {
    type Value = LocalOptions;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("local-part parsing options")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      let mut smtp_utf8 = None;
      while let Some(field) = access.next_key()? {
        match field {
          LocalOptionsField::SmtpUtf8 => {
            if smtp_utf8.is_some() {
              return Err(de::Error::duplicate_field("smtp_utf8"));
            }
            smtp_utf8 = Some(access.next_value()?);
          }
        }
      }

      let smtp_utf8 = if self.human_readable {
        smtp_utf8.unwrap_or_default()
      } else {
        smtp_utf8.ok_or_else(|| de::Error::missing_field("smtp_utf8"))?
      };

      Ok(LocalOptions::new().with_smtp_utf8(smtp_utf8))
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      if self.human_readable {
        return Err(de::Error::invalid_type(Unexpected::Seq, &self));
      }

      let smtp_utf8 = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
      if access.next_element::<IgnoredAny>()?.is_some() {
        return Err(de::Error::invalid_length(2, &self));
      }

      Ok(LocalOptions::new().with_smtp_utf8(smtp_utf8))
    }
  }

  impl Serialize for DomainOptions {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut state = serializer.serialize_struct("DomainOptions", 3)?;
      state.serialize_field("minimum_dns_labels", &self.minimum_dns_labels())?;
      state.serialize_field("literals", &self.literals())?;
      state.serialize_field("unicode", &self.unicode())?;
      state.end()
    }
  }

  impl<'de> Deserialize<'de> for DomainOptions {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let human_readable = deserializer.is_human_readable();
      deserializer.deserialize_struct(
        "DomainOptions",
        &["minimum_dns_labels", "literals", "unicode"],
        DomainOptionsVisitor { human_readable },
      )
    }
  }

  enum DomainOptionsField {
    MinimumDnsLabels,
    Literals,
    Unicode,
  }

  struct DomainOptionsFieldVisitor;

  impl Visitor<'_> for DomainOptionsFieldVisitor {
    type Value = DomainOptionsField;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("a DomainOptions field")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match value {
        "minimum_dns_labels" => Ok(DomainOptionsField::MinimumDnsLabels),
        "literals" => Ok(DomainOptionsField::Literals),
        "unicode" => Ok(DomainOptionsField::Unicode),
        _ => Err(E::unknown_field(
          value,
          &["minimum_dns_labels", "literals", "unicode"],
        )),
      }
    }
  }

  impl<'de> Deserialize<'de> for DomainOptionsField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_identifier(DomainOptionsFieldVisitor)
    }
  }

  struct DomainOptionsVisitor {
    human_readable: bool,
  }

  impl<'de> Visitor<'de> for DomainOptionsVisitor {
    type Value = DomainOptions;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("domain-part parsing options")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      let mut minimum_dns_labels = None;
      let mut literals = None;
      let mut unicode = None;
      while let Some(field) = access.next_key()? {
        match field {
          DomainOptionsField::MinimumDnsLabels => {
            if minimum_dns_labels.is_some() {
              return Err(de::Error::duplicate_field("minimum_dns_labels"));
            }
            minimum_dns_labels = Some(access.next_value()?);
          }
          DomainOptionsField::Literals => {
            if literals.is_some() {
              return Err(de::Error::duplicate_field("literals"));
            }
            literals = Some(access.next_value()?);
          }
          DomainOptionsField::Unicode => {
            if unicode.is_some() {
              return Err(de::Error::duplicate_field("unicode"));
            }
            unicode = Some(access.next_value()?);
          }
        }
      }

      let (minimum_dns_labels, literals, unicode) = if self.human_readable {
        (
          minimum_dns_labels.unwrap_or_else(|| DomainOptions::new().minimum_dns_labels()),
          literals.unwrap_or_default(),
          unicode.unwrap_or_default(),
        )
      } else {
        (
          minimum_dns_labels.ok_or_else(|| de::Error::missing_field("minimum_dns_labels"))?,
          literals.ok_or_else(|| de::Error::missing_field("literals"))?,
          unicode.ok_or_else(|| de::Error::missing_field("unicode"))?,
        )
      };

      Ok(
        DomainOptions::new()
          .with_minimum_dns_labels(minimum_dns_labels)
          .with_literals(literals)
          .with_unicode(unicode),
      )
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      if self.human_readable {
        return Err(de::Error::invalid_type(Unexpected::Seq, &self));
      }

      let minimum_dns_labels = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
      let literals = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
      let unicode = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(2, &self))?;
      if access.next_element::<IgnoredAny>()?.is_some() {
        return Err(de::Error::invalid_length(4, &self));
      }

      Ok(
        DomainOptions::new()
          .with_minimum_dns_labels(minimum_dns_labels)
          .with_literals(literals)
          .with_unicode(unicode),
      )
    }
  }

  impl Serialize for Limits {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      let mut state = serializer.serialize_struct("Limits", 1)?;
      state.serialize_field("max_local_part_len", &self.max_local_part_len())?;
      state.end()
    }
  }

  impl<'de> Deserialize<'de> for Limits {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let human_readable = deserializer.is_human_readable();
      deserializer.deserialize_struct(
        "Limits",
        &["max_local_part_len"],
        LimitsVisitor { human_readable },
      )
    }
  }

  enum LimitsField {
    MaxLocalPartLen,
  }

  struct LimitsFieldVisitor;

  impl Visitor<'_> for LimitsFieldVisitor {
    type Value = LimitsField;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("a Limits field")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      match value {
        "max_local_part_len" => Ok(LimitsField::MaxLocalPartLen),
        _ => Err(E::unknown_field(value, &["max_local_part_len"])),
      }
    }
  }

  impl<'de> Deserialize<'de> for LimitsField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      deserializer.deserialize_identifier(LimitsFieldVisitor)
    }
  }

  struct LimitsVisitor {
    human_readable: bool,
  }

  impl<'de> Visitor<'de> for LimitsVisitor {
    type Value = Limits;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.write_str("parsing limits")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      let mut max_local_part_len = None;
      while let Some(field) = access.next_key()? {
        match field {
          LimitsField::MaxLocalPartLen => {
            if max_local_part_len.is_some() {
              return Err(de::Error::duplicate_field("max_local_part_len"));
            }
            max_local_part_len = Some(access.next_value()?);
          }
        }
      }

      let max_local_part_len = if self.human_readable {
        max_local_part_len.unwrap_or_else(|| Limits::new().max_local_part_len())
      } else {
        max_local_part_len.ok_or_else(|| de::Error::missing_field("max_local_part_len"))?
      };

      Ok(Limits::new().with_max_local_part_len(max_local_part_len))
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
      A: SeqAccess<'de>,
    {
      if self.human_readable {
        return Err(de::Error::invalid_type(Unexpected::Seq, &self));
      }

      let max_local_part_len = access
        .next_element()?
        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
      if access.next_element::<IgnoredAny>()?.is_some() {
        return Err(de::Error::invalid_length(2, &self));
      }

      Ok(Limits::new().with_max_local_part_len(max_local_part_len))
    }
  }
}
