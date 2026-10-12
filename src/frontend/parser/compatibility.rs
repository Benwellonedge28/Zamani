
#![forbid(unsafe_code)]

//! Language-version and syntax-feature compatibility for the Zamani parser.
//!
//! # Purpose
//!
//! Apply an explicitly declared compatibility contract while parsing Zamani
//! source. This module determines whether a declared syntax feature is
//! available, deprecated, experimental, or disallowed for the selected
//! language version.
//!
//! # Ownership
//!
//! This module owns:
//! - language-version representation and parsing;
//! - stable identifiers for compatibility features;
//! - feature lifecycle metadata;
//! - deterministic feature registration;
//! - parser-facing compatibility policy;
//! - structured compatibility findings.
//!
//! This module does not own:
//! - tokenization or token-kind definitions;
//! - grammar productions or AST construction;
//! - AST representation or AST validation;
//! - name resolution, types, effects, capabilities, or resource analysis;
//! - compiler IR, target selection, hardware discovery, or execution;
//! - a second normative language specification.
//!
//! # Integration contract
//!
//! The parser calls `CompatibilityRegistry::check` when a grammar production
//! identifies a feature whose availability or lifecycle is versioned.
//!
//! The selected `LanguageVersion` comes from the parser configuration or
//! explicit source-version declaration after that declaration has been
//! parsed. This module does not guess a default language version.
//!
//! The registry is built from approved compatibility metadata. This file
//! intentionally contains no hard-coded list of keywords, quantum
//! operations, HDL primitives, dialect names, or hardware targets.
//!
//! Grammar handlers should report findings through the parser's diagnostic
//! adapter. They should not convert these findings into unrelated parser
//! errors or silently rewrite the source.
//!
//! # Version semantics
//!
//! `introduced_in` is inclusive: a feature is available starting at that
//! version, subject to its lifecycle policy.
//!
//! `removed_in` is exclusive: a feature is unavailable at that version and
//! later.
//!
//! `deprecated_in` is inclusive: the configured deprecation policy applies
//! at that version and later.
//!
//! All version comparisons are numeric, not lexicographic.
//!
//! # Scalability and safety
//!
//! Registries use dynamically sized ordered collections. No language-level
//! limit is imposed on feature count or computational capacity. Actual
//! allocations remain subject to host resources and caller policy.
//!
//! The implementation uses safe Rust only and does not execute extensions,
//! evaluate source code, access the filesystem, or inspect hardware.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Numeric Zamani language version.
///
/// This is the language-contract version, not the compiler release, AST
/// schema version, IR version, dialect version, or target version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LanguageVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl LanguageVersion {
    /// Construct a language version from numeric components.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for LanguageVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A malformed or unsupported textual language version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionParseError {
    /// The input was empty or contained surrounding whitespace.
    InvalidFormat(String),

    /// A version component was not an unsigned decimal integer.
    InvalidComponent(String),

    /// A component contains a leading zero that is not the number zero.
    LeadingZero(String),
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(value) => {
                write!(
                    f,
                    "invalid language version {value:?}; expected MAJOR.MINOR.PATCH"
                )
            }
            Self::InvalidComponent(value) => {
                write!(f, "invalid numeric language-version component {value:?}")
            }
            Self::LeadingZero(value) => {
                write!(
                    f,
                    "language-version component {value:?} must not contain leading zeros"
                )
            }
        }
    }
}

impl Error for VersionParseError {}

impl FromStr for LanguageVersion {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() || input.trim() != input {
            return Err(VersionParseError::InvalidFormat(input.to_owned()));
        }

        let mut parts = input.split('.');
        let major = parse_version_component(parts.next(), input)?;
        let minor = parse_version_component(parts.next(), input)?;
        let patch = parse_version_component(parts.next(), input)?;

        if parts.next().is_some() {
            return Err(VersionParseError::InvalidFormat(input.to_owned()));
        }

        Ok(Self::new(major, minor, patch))
    }
}

fn parse_version_component(
    component: Option<&str>,
    whole_input: &str,
) -> Result<u64, VersionParseError> {
    let value = component.ok_or_else(|| {
        VersionParseError::InvalidFormat(whole_input.to_owned())
    })?;

    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionParseError::InvalidComponent(value.to_owned()));
    }

    if value.len() > 1 && value.starts_with('0') {
        return Err(VersionParseError::LeadingZero(value.to_owned()));
    }

    value
        .parse::<u64>()
        .map_err(|_| VersionParseError::InvalidComponent(value.to_owned()))
}

/// Validated identifier for a compatibility feature.
///
/// Feature IDs are opaque, case-sensitive metadata keys. Namespaced values
/// such as `core.patterns`, `quantum.operation_syntax`, or
/// `vendor.example.feature` are permitted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureId(String);

impl FeatureId {
    /// Create a feature identifier.
    ///
    /// Identifiers must be non-empty, have no surrounding whitespace, and
    /// contain no control characters. The naming convention beyond these
    /// basic checks is owned by the compatibility metadata contract.
    pub fn new(value: impl Into<String>) -> Result<Self, FeatureIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeatureIdError::Empty);
        }

        if value.trim() != value {
            return Err(FeatureIdError::SurroundingWhitespace);
        }

        if value.chars().any(char::is_control) {
            return Err(FeatureIdError::ControlCharacter);
        }

        Ok(Self(value))
    }

    /// Return the exact feature identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Why a feature identifier could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureIdError {
    Empty,
    SurroundingWhitespace,
    ControlCharacter,
}

impl fmt::Display for FeatureIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("feature identifier must not be empty"),
            Self::SurroundingWhitespace => {
                f.write_str("feature identifier must not have surrounding whitespace")
            }
            Self::ControlCharacter => {
                f.write_str("feature identifier must not contain control characters")
            }
        }
    }
}

impl Error for FeatureIdError {}

/// Lifecycle status vocabulary aligned with the repository's grammar
/// conformance model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FeatureStatus {
    /// Normative, implemented, tested, and compatibility-controlled.
    Stable,

    /// Implemented but not yet stabilized.
    Implemented,

    /// Some implementation exists, but the integration contract is
    /// incomplete.
    Partial,

    /// Designed but not accepted into the normative language.
    Proposed,

    /// Available only through an explicit experimental boundary.
    Experimental,

    /// Planned but not implemented.
    Planned,

    /// Existing syntax retained for compatibility but discouraged.
    Deprecated,

    /// Historical or explanatory material only.
    Historical,

    /// Explicitly excluded from the language.
    Rejected,
}

/// Metadata describing the compatibility lifecycle of a syntax feature.
///
/// This is declarative metadata. It does not define the feature's grammar
/// or semantic meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureRule {
    id: FeatureId,
    status: FeatureStatus,
    introduced_in: Option<LanguageVersion>,
    deprecated_in: Option<LanguageVersion>,
    removed_in: Option<LanguageVersion>,
    replacement: Option<String>,
    description: Option<String>,
}

impl FeatureRule {
    /// Create a rule with a required identifier and lifecycle status.
    #[must_use]
    pub fn new(id: FeatureId, status: FeatureStatus) -> Self {
        Self {
            id,
            status,
            introduced_in: None,
            deprecated_in: None,
            removed_in: None,
            replacement: None,
            description: None,
        }
    }

    /// Set the first language version in which this feature is available.
    #[must_use]
    pub fn introduced_in(mut self, version: LanguageVersion) -> Self {
        self.introduced_in = Some(version);
        self
    }

    /// Set the version from which deprecation policy applies.
    #[must_use]
    pub fn deprecated_in(mut self, version: LanguageVersion) -> Self {
        self.deprecated_in = Some(version);
        self
    }

    /// Set the first version in which this feature is unavailable.
    #[must_use]
    pub fn removed_in(mut self, version: LanguageVersion) -> Self {
        self.removed_in = Some(version);
        self
    }

    /// Set the identifier of the documented replacement, if any.
    #[must_use]
    pub fn replacement(mut self, replacement: impl Into<String>) -> Self {
        self.replacement = Some(replacement.into());
        self
    }

    /// Set a short explanation for diagnostics and tooling.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn id(&self) -> &FeatureId {
        &self.id
    }

    #[must_use]
    pub const fn status(&self) -> FeatureStatus {
        self.status
    }

    #[must_use]
    pub const fn introduced_version(&self) -> Option<LanguageVersion> {
        self.introduced_in
    }

    #[must_use]
    pub const fn deprecated_version(&self) -> Option<LanguageVersion> {
        self.deprecated_in
    }

    #[must_use]
    pub const fn removed_version(&self) -> Option<LanguageVersion> {
        self.removed_in
    }

    #[must_use]
    pub fn replacement_id(&self) -> Option<&str> {
        self.replacement.as_deref()
    }

    #[must_use]
    pub fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Policy action for a feature lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureDisposition {
    /// Accept without emitting a finding.
    Allow,

    /// Accept parsing but emit a compatibility warning.
    Warn,

    /// Reject the feature for the selected compatibility policy.
    Reject,
}

/// Policy for syntax whose deprecation version has been reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeprecationPolicy {
    Allow,
    Warn,
    Reject,
}

/// Policy for a feature that has no registered compatibility metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnknownFeaturePolicy {
    /// Reject unknown feature IDs. Recommended for normative parsing.
    Reject,

    /// Emit a warning but permit parsing to continue.
    Warn,

    /// Permit parsing without a finding.
    ///
    /// Use only when a caller has independently established that unknown
    /// feature metadata is acceptable for its workflow.
    Allow,
}

/// Explicit policy applied to a selected language version.
///
/// No global or guessed "current version" is embedded here. Callers must
/// select the language version and choose their lifecycle policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityPolicy {
    language_version: LanguageVersion,
    deprecated: DeprecationPolicy,
    unknown_feature: UnknownFeaturePolicy,
    lifecycle: BTreeMap<FeatureStatus, FeatureDisposition>,
}

impl CompatibilityPolicy {
    /// Create a policy for a specific language version with conservative
    /// lifecycle defaults.
    ///
    /// Stable and implemented features are accepted. Partial and
    /// experimental features warn. Proposed, planned, historical, and
    /// rejected features fail. Deprecation warns by default, and unknown
    /// feature IDs fail by default.
    #[must_use]
    pub fn conservative(language_version: LanguageVersion) -> Self {
        let mut lifecycle = BTreeMap::new();
        lifecycle.insert(FeatureStatus::Stable, FeatureDisposition::Allow);
        lifecycle.insert(FeatureStatus::Implemented, FeatureDisposition::Allow);
        lifecycle.insert(FeatureStatus::Partial, FeatureDisposition::Warn);
        lifecycle.insert(FeatureStatus::Proposed, FeatureDisposition::Reject);
        lifecycle.insert(FeatureStatus::Experimental, FeatureDisposition::Warn);
        lifecycle.insert(FeatureStatus::Planned, FeatureDisposition::Reject);
        lifecycle.insert(FeatureStatus::Deprecated, FeatureDisposition::Warn);
        lifecycle.insert(FeatureStatus::Historical, FeatureDisposition::Reject);
        lifecycle.insert(FeatureStatus::Rejected, FeatureDisposition::Reject);

        Self {
            language_version,
            deprecated: DeprecationPolicy::Warn,
            unknown_feature: UnknownFeaturePolicy::Reject,
            lifecycle,
        }
    }

    /// Return the selected language version.
    #[must_use]
    pub const fn language_version(&self) -> LanguageVersion {
        self.language_version
    }

    /// Configure treatment of deprecated syntax.
    #[must_use]
    pub fn with_deprecation_policy(mut self, policy: DeprecationPolicy) -> Self {
        self.deprecated = policy;
        self
    }

    /// Configure treatment of feature IDs absent from the registry.
    #[must_use]
    pub fn with_unknown_feature_policy(
        mut self,
        policy: UnknownFeaturePolicy,
    ) -> Self {
        self.unknown_feature = policy;
        self
    }

    /// Override the disposition of a lifecycle status.
    #[must_use]
    pub fn with_status_disposition(
        mut self,
        status: FeatureStatus,
        disposition: FeatureDisposition,
    ) -> Self {
        self.lifecycle.insert(status, disposition);
        self
    }

    #[must_use]
    pub const fn deprecation_policy(&self) -> DeprecationPolicy {
        self.deprecated
    }

    #[must_use]
    pub const fn unknown_feature_policy(&self) -> UnknownFeaturePolicy {
        self.unknown_feature
    }

    #[must_use]
    pub fn status_disposition(
        &self,
        status: FeatureStatus,
    ) -> FeatureDisposition {
        self.lifecycle
            .get(&status)
            .copied()
            .unwrap_or(FeatureDisposition::Reject)
    }
}

/// A syntax feature encountered by a parser production.
///
/// `source_range` is an optional half-open byte range in the current source.
/// It intentionally uses the standard-library range type so this module
/// does not introduce another source-span representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureUse {
    id: FeatureId,
    source_range: Option<std::ops::Range<usize>>,
}

impl FeatureUse {
    #[must_use]
    pub fn new(id: FeatureId) -> Self {
        Self {
            id,
            source_range: None,
        }
    }

    #[must_use]
    pub fn at(id: FeatureId, source_range: std::ops::Range<usize>) -> Self {
        Self {
            id,
            source_range: Some(source_range),
        }
    }

    #[must_use]
    pub fn id(&self) -> &FeatureId {
        &self.id
    }

    #[must_use]
    pub fn source_range(&self) -> Option<std::ops::Range<usize>> {
        self.source_range.clone()
    }
}

/// Severity of a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatibilitySeverity {
    Warning,
    Error,
}

/// Machine-readable compatibility finding code.
///
/// Codes are stable identifiers; diagnostic wording may evolve without
/// requiring tools to parse human-readable messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompatibilityCode {
    UnknownFeature,
    FeatureNotIntroduced,
    FeatureRemoved,
    FeatureLifecycleWarning,
    FeatureLifecycleRejected,
    DeprecatedSyntaxWarning,
    DeprecatedSyntaxRejected,
    InvalidRule,
}

/// A compatibility finding associated with a feature use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityIssue {
    code: CompatibilityCode,
    severity: CompatibilitySeverity,
    feature: FeatureId,
    language_version: LanguageVersion,
    source_range: Option<std::ops::Range<usize>>,
    message: String,
    replacement: Option<String>,
}

impl CompatibilityIssue {
    #[must_use]
    pub const fn code(&self) -> CompatibilityCode {
        self.code
    }

    #[must_use]
    pub const fn severity(&self) -> CompatibilitySeverity {
        self.severity
    }

    #[must_use]
    pub fn feature(&self) -> &FeatureId {
        &self.feature
    }

    #[must_use]
    pub const fn language_version(&self) -> LanguageVersion {
        self.language_version
    }

    #[must_use]
    pub fn source_range(&self) -> Option<std::ops::Range<usize>> {
        self.source_range.clone()
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn replacement(&self) -> Option<&str> {
        self.replacement.as_deref()
    }
}

/// Result of checking one or more syntax features.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompatibilityReport {
    issues: Vec<CompatibilityIssue>,
}

impl CompatibilityReport {
    #[must_use]
    pub fn issues(&self) -> &[CompatibilityIssue] {
        &self.issues
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues
            .iter()
            .any(|issue| issue.severity == CompatibilitySeverity::Error)
    }

    #[must_use]
    pub fn is_compatible(&self) -> bool {
        !self.has_errors()
    }

    #[must_use]
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == CompatibilitySeverity::Error)
            .count()
    }

    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|issue| issue.severity == CompatibilitySeverity::Warning)
            .count()
    }

    fn push(&mut self, issue: CompatibilityIssue) {
        self.issues.push(issue);
    }

    fn sort_deterministically(&mut self) {
        self.issues.sort_by(|left, right| {
            left.source_range
                .as_ref()
                .map(|range| (range.start, range.end))
                .cmp(
                    &right
                        .source_range
                        .as_ref()
                        .map(|range| (range.start, range.end)),
                )
                .then_with(|| left.feature.cmp(&right.feature))
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.message.cmp(&right.message))
        });
    }
}

/// Error returned when compatibility metadata cannot be registered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityRegistryError {
    DuplicateFeature(FeatureId),
    InvalidVersionOrder {
        feature: FeatureId,
        introduced: Option<LanguageVersion>,
        deprecated: Option<LanguageVersion>,
        removed: Option<LanguageVersion>,
    },
    EmptyReplacement {
        feature: FeatureId,
    },
}

impl fmt::Display for CompatibilityRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateFeature(feature) => {
                write!(f, "compatibility feature {feature} is already registered")
            }
            Self::InvalidVersionOrder {
                feature,
                introduced,
                deprecated,
                removed,
            } => write!(
                f,
                "invalid version ordering for feature {feature}: \
                 introduced={introduced:?}, deprecated={deprecated:?}, removed={removed:?}"
            ),
            Self::EmptyReplacement { feature } => {
                write!(f, "replacement identifier for feature {feature} is empty")
            }
        }
    }
}

impl Error for CompatibilityRegistryError {}

/// Deterministic registry of syntax compatibility rules.
///
/// Registration is explicit. This module ships no assumed catalogue of
/// language features because the normative specifications and feature
/// lifecycle metadata remain the source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompatibilityRegistry {
    rules: BTreeMap<FeatureId, FeatureRule>,
}

impl CompatibilityRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a feature rule.
    ///
    /// Duplicate IDs are rejected rather than silently overwritten. This
    /// prevents extension registration order from changing compatibility
    /// behavior.
    pub fn register(
        &mut self,
        rule: FeatureRule,
    ) -> Result<(), CompatibilityRegistryError> {
        validate_rule(&rule)?;

        if self.rules.contains_key(&rule.id) {
            return Err(CompatibilityRegistryError::DuplicateFeature(rule.id));
        }

        self.rules.insert(rule.id.clone(), rule);
        Ok(())
    }

    /// Look up a registered feature rule.
    #[must_use]
    pub fn get(&self, id: &FeatureId) -> Option<&FeatureRule> {
        self.rules.get(id)
    }

    /// Return the number of registered rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Evaluate one feature use against an explicit policy.
    #[must_use]
    pub fn check(
        &self,
        feature_use: &FeatureUse,
        policy: &CompatibilityPolicy,
    ) -> CompatibilityReport {
        let mut report = CompatibilityReport::default();
        self.check_into(feature_use, policy, &mut report);
        report.sort_deterministically();
        report
    }

    /// Evaluate multiple feature uses.
    ///
    /// Findings are sorted deterministically after evaluation. Repeated uses
    /// remain separate findings so diagnostics can point to each occurrence.
    #[must_use]
    pub fn check_all<'a>(
        &self,
        feature_uses: impl IntoIterator<Item = &'a FeatureUse>,
        policy: &CompatibilityPolicy,
    ) -> CompatibilityReport {
        let mut report = CompatibilityReport::default();

        for feature_use in feature_uses {
            self.check_into(feature_use, policy, &mut report);
        }

        report.sort_deterministically();
        report
    }

    fn check_into(
        &self,
        feature_use: &FeatureUse,
        policy: &CompatibilityPolicy,
        report: &mut CompatibilityReport,
    ) {
        let version = policy.language_version();
        let Some(rule) = self.rules.get(&feature_use.id) else {
            let (severity, message) = match policy.unknown_feature {
                UnknownFeaturePolicy::Allow => return,
                UnknownFeaturePolicy::Warn => (
                    CompatibilitySeverity::Warning,
                    format!(
                        "feature {} has no registered compatibility rule",
                        feature_use.id
                    ),
                ),
                UnknownFeaturePolicy::Reject => (
                    CompatibilitySeverity::Error,
                    format!(
                        "feature {} has no registered compatibility rule",
                        feature_use.id
                    ),
                ),
            };

            report.push(issue(
                CompatibilityCode::UnknownFeature,
                severity,
                feature_use,
                version,
                message,
                None,
            ));
            return;
        };

        if let Some(introduced) = rule.introduced_in {
            if version < introduced {
                report.push(issue(
                    CompatibilityCode::FeatureNotIntroduced,
                    CompatibilitySeverity::Error,
                    feature_use,
                    version,
                    format!(
                        "feature {} is introduced in language version {}, \
                         but the selected version is {}",
                        rule.id, introduced, version
                    ),
                    rule.replacement.clone(),
                ));
                return;
            }
        }

        if let Some(removed) = rule.removed_in {
            if version >= removed {
                report.push(issue(
                    CompatibilityCode::FeatureRemoved,
                    CompatibilitySeverity::Error,
                    feature_use,
                    version,
                    format!(
                        "feature {} was removed in language version {}",
                        rule.id, removed
                    ),
                    rule.replacement.clone(),
                ));
                return;
            }
        }

        let disposition = policy.status_disposition(rule.status);
        match disposition {
            FeatureDisposition::Allow => {}
            FeatureDisposition::Warn => report.push(issue(
                CompatibilityCode::FeatureLifecycleWarning,
                CompatibilitySeverity::Warning,
                feature_use,
                version,
                lifecycle_message(rule),
                rule.replacement.clone(),
            )),
            FeatureDisposition::Reject => {
                report.push(issue(
                    CompatibilityCode::FeatureLifecycleRejected,
                    CompatibilitySeverity::Error,
                    feature_use,
                    version,
                    lifecycle_message(rule),
                    rule.replacement.clone(),
                ));
                return;
            }
        }

        let deprecated_by_status = rule.status == FeatureStatus::Deprecated;
        let deprecated_by_version = rule
            .deprecated_in
            .is_some_and(|deprecated| version >= deprecated);

        if deprecated_by_status || deprecated_by_version {
            match policy.deprecated {
                DeprecationPolicy::Allow => {}
                DeprecationPolicy::Warn => report.push(issue(
                    CompatibilityCode::DeprecatedSyntaxWarning,
                    CompatibilitySeverity::Warning,
                    feature_use,
                    version,
                    deprecation_message(rule),
                    rule.replacement.clone(),
                )),
                DeprecationPolicy::Reject => report.push(issue(
                    CompatibilityCode::DeprecatedSyntaxRejected,
                    CompatibilitySeverity::Error,
                    feature_use,
                    version,
                    deprecation_message(rule),
                    rule.replacement.clone(),
                )),
            }
        }
    }
}

fn validate_rule(rule: &FeatureRule) -> Result<(), CompatibilityRegistryError> {
    let introduced = rule.introduced_in;
    let deprecated = rule.deprecated_in;
    let removed = rule.removed_in;

    let invalid = match (introduced, deprecated, removed) {
        (Some(i), Some(d), Some(r)) => i > d || d >= r,
        (Some(i), Some(d), None) => i > d,
        (Some(i), None, Some(r)) => i >= r,
        (None, Some(d), Some(r)) => d >= r,
        _ => false,
    };

    if invalid {
        return Err(CompatibilityRegistryError::InvalidVersionOrder {
            feature: rule.id.clone(),
            introduced,
            deprecated,
            removed,
        });
    }

    if rule
        .replacement
        .as_deref()
        .is_some_and(|replacement| replacement.trim().is_empty())
    {
        return Err(CompatibilityRegistryError::EmptyReplacement {
            feature: rule.id.clone(),
        });
    }

    Ok(())
}

fn lifecycle_message(rule: &FeatureRule) -> String {
    if let Some(description) = rule.description.as_deref() {
        return format!(
            "feature {} has lifecycle status {:?}: {}",
            rule.id, rule.status, description
        );
    }

    format!(
        "feature {} has lifecycle status {:?}",
        rule.id, rule.status
    )
}

fn deprecation_message(rule: &FeatureRule) -> String {
    let mut message = format!("feature {} is deprecated", rule.id);

    if let Some(version) = rule.deprecated_in {
        message.push_str(&format!(" since language version {version}"));
    }

    if let Some(replacement) = rule.replacement.as_deref() {
        message.push_str(&format!("; use {replacement} instead"));
    }

    message
}

fn issue(
    code: CompatibilityCode,
    severity: CompatibilitySeverity,
    feature_use: &FeatureUse,
    language_version: LanguageVersion,
    message: String,
    replacement: Option<String>,
) -> CompatibilityIssue {
    CompatibilityIssue {
        code,
        severity,
        feature: feature_use.id.clone(),
        language_version,
        source_range: feature_use.source_range.clone(),
        message,
        replacement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> FeatureId {
        FeatureId::new(value).expect("valid test feature ID")
    }

    fn version(value: &str) -> LanguageVersion {
        value.parse().expect("valid test language version")
    }

    fn stable_rule(feature: &str) -> FeatureRule {
        FeatureRule::new(id(feature), FeatureStatus::Stable)
    }

    #[test]
    fn parses_and_orders_numeric_versions() {
        assert_eq!(version("1.10.0"), LanguageVersion::new(1, 10, 0));
        assert!(version("1.10.0") > version("1.9.99"));
        assert_eq!(version("2.0.0").to_string(), "2.0.0");
    }

    #[test]
    fn rejects_malformed_versions() {
        for value in [
            "",
            "1",
            "1.2",
            "1.2.3.4",
            " 1.2.3",
            "1.2.3 ",
            "1..3",
            "1.-2.3",
            "01.2.3",
            "1.02.3",
            "1.2.03",
            "18446744073709551616.0.0",
        ] {
            assert!(value.parse::<LanguageVersion>().is_err(), "{value:?}");
        }
    }

    #[test]
    fn rejects_empty_and_invalid_feature_ids() {
        assert!(FeatureId::new("").is_err());
        assert!(FeatureId::new("   ").is_err());
        assert!(FeatureId::new(" feature").is_err());
        assert!(FeatureId::new("feature ").is_err());
        assert!(FeatureId::new("feature\nname").is_err());
        assert_eq!(id("vendor.quantum.operation").as_str(), "vendor.quantum.operation");
    }

    #[test]
    fn registration_is_explicit_and_duplicate_ids_fail() {
        let mut registry = CompatibilityRegistry::new();
        registry.register(stable_rule("core.expressions")).unwrap();

        assert_eq!(registry.len(), 1);
        assert!(matches!(
            registry.register(stable_rule("core.expressions")),
            Err(CompatibilityRegistryError::DuplicateFeature(_))
        ));
    }

    #[test]
    fn version_boundaries_are_inclusive_and_exclusive_as_documented() {
        let mut registry = CompatibilityRegistry::new();
        registry
            .register(
                stable_rule("core.new_syntax")
                    .introduced_in(version("1.2.0"))
                    .removed_in(version("2.0.0")),
            )
            .unwrap();

        let before = CompatibilityPolicy::conservative(version("1.1.9"));
        let at_intro = CompatibilityPolicy::conservative(version("1.2.0"));
        let before_removal = CompatibilityPolicy::conservative(version("1.99.0"));
        let at_removal = CompatibilityPolicy::conservative(version("2.0.0"));

        assert!(registry
            .check(&FeatureUse::new(id("core.new_syntax")), &before)
            .has_errors());
        assert!(registry
            .check(&FeatureUse::new(id("core.new_syntax")), &at_intro)
            .is_compatible());
        assert!(registry
            .check(&FeatureUse::new(id("core.new_syntax")), &before_removal)
            .is_compatible());
        assert!(registry
            .check(&FeatureUse::new(id("core.new_syntax")), &at_removal)
            .has_errors());
    }

    #[test]
    fn unknown_features_fail_under_conservative_policy() {
        let registry = CompatibilityRegistry::new();
        let policy = CompatibilityPolicy::conservative(version("1.0.0"));
        let report = registry.check(&FeatureUse::new(id("unregistered.feature")), &policy);

        assert!(report.has_errors());
        assert_eq!(report.issues()[0].code(), CompatibilityCode::UnknownFeature);
    }

    #[test]
    fn deprecated_features_warn_by_default_and_can_be_rejected() {
        let mut registry = CompatibilityRegistry::new();
        registry
            .register(
                FeatureRule::new(id("legacy.syntax"), FeatureStatus::Deprecated)
                    .deprecated_in(version("1.1.0"))
                    .replacement("core.replacement"),
            )
            .unwrap();

        let use_site = FeatureUse::at(id("legacy.syntax"), 4..10);
        let warning_policy = CompatibilityPolicy::conservative(version("1.2.0"));
        let warning = registry.check(&use_site, &warning_policy);

        assert!(!warning.has_errors());
        assert!(warning.warning_count() > 0);
        assert_eq!(warning.issues()[0].source_range(), Some(4..10));
        assert_eq!(warning.issues()[0].replacement(), Some("core.replacement"));

        let rejecting_policy = CompatibilityPolicy::conservative(version("1.2.0"))
            .with_deprecation_policy(DeprecationPolicy::Reject);
        let rejected = registry.check(&use_site, &rejecting_policy);

        assert!(rejected.has_errors());
        assert!(rejected
            .issues()
            .iter()
            .any(|issue| issue.code() == CompatibilityCode::DeprecatedSyntaxRejected));
    }

    #[test]
    fn proposed_and_historical_features_are_rejected_by_default() {
        let mut registry = CompatibilityRegistry::new();
        registry
            .register(FeatureRule::new(id("future.syntax"), FeatureStatus::Proposed))
            .unwrap();
        registry
            .register(FeatureRule::new(id("old.syntax"), FeatureStatus::Historical))
            .unwrap();

        let policy = CompatibilityPolicy::conservative(version("1.0.0"));
        let uses = [
            FeatureUse::new(id("old.syntax")),
            FeatureUse::new(id("future.syntax")),
        ];
        let report = registry.check_all(uses.iter(), &policy);

        assert!(report.has_errors());
        assert_eq!(report.error_count(), 2);
    }

    #[test]
    fn invalid_rule_version_order_is_rejected() {
        let mut registry = CompatibilityRegistry::new();
        let rule = stable_rule("core.bad_rule")
            .introduced_in(version("2.0.0"))
            .deprecated_in(version("1.9.0"));

        assert!(matches!(
            registry.register(rule),
            Err(CompatibilityRegistryError::InvalidVersionOrder { .. })
        ));
    }

    #[test]
    fn findings_are_sorted_deterministically_by_source_range() {
        let mut registry = CompatibilityRegistry::new();
        registry
            .register(FeatureRule::new(id("experimental.z"), FeatureStatus::Experimental))
            .unwrap();
        registry
            .register(FeatureRule::new(id("experimental.a"), FeatureStatus::Experimental))
            .unwrap();

        let policy = CompatibilityPolicy::conservative(version("1.0.0"));
        let uses = [
            FeatureUse::at(id("experimental.z"), 20..30),
            FeatureUse::at(id("experimental.a"), 1..5),
        ];
        let report = registry.check_all(uses.iter(), &policy);

        assert_eq!(report.issues()[0].source_range(), Some(1..5));
        assert_eq!(report.issues()[1].source_range(), Some(20..30));
    }

    #[test]
    fn callers_can_explicitly_allow_registered_experimental_features() {
        let mut registry = CompatibilityRegistry::new();
        registry
            .register(FeatureRule::new(
                id("vendor.experimental"),
                FeatureStatus::Experimental,
            ))
            .unwrap();

        let policy = CompatibilityPolicy::conservative(version("1.0.0"))
            .with_status_disposition(FeatureStatus::Experimental, FeatureDisposition::Allow);

        let report = registry.check(&FeatureUse::new(id("vendor.experimental")), &policy);
        assert!(report.is_compatible());
        assert_eq!(report.issues().len(), 0);
    }
}
