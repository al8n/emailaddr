use crate::MAX_LOCAL_PART_LENGTH;

/// Default local-part length limit used by [`Limits`].
pub const DEFAULT_MAX_LOCAL_PART_LENGTH: usize = MAX_LOCAL_PART_LENGTH;

/// Default minimum DNS label count used by [`DomainOptions`].
pub const DEFAULT_MINIMUM_DNS_LABELS: usize = 1;

/// Email address parsing options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Options {
  local: LocalOptions,
  domain: DomainOptions,
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
pub struct LocalOptions {
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
pub struct DomainOptions {
  minimum_dns_labels: usize,
  literals: DomainLiteralPolicy,
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
pub struct Limits {
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
#[non_exhaustive]
pub enum SmtpUtf8Policy {
  /// Accept UTF-8 local-parts.
  #[default]
  Allow,
  /// Reject non-ASCII local-parts.
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
}

/// Policy for address-literal domain-parts.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DomainLiteralPolicy {
  /// Accept address-literal domains.
  #[default]
  Allow,
  /// Reject address-literal domains.
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
}

/// Policy for Unicode domain input.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DomainUnicodePolicy {
  /// Reject non-ASCII domain input.
  AsciiOnly,
  /// Accept Unicode domain input only through IDNA normalization.
  #[default]
  Idna,
  /// Preserve non-ASCII UTF-8 DNS labels as non-standard application data.
  NonStandardUtf8,
}

impl DomainUnicodePolicy {
  /// Returns the stable policy name.
  #[cfg_attr(not(coverage), inline(always))]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::AsciiOnly => "ascii_only",
      Self::Idna => "idna",
      Self::NonStandardUtf8 => "non_standard_utf8",
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
}
