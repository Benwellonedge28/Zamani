
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Compatibility Contracts
//!
//! Path: src/classical/compatibility.rs
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Assess whether a classical artifact can interoperate with a consumer
//! without silently changing its declared semantics.
//!
//! This module provides:
//! - explicit, independently versioned compatibility contracts;
//! - semantic-version comparison with conservative compatibility rules;
//! - extensible, namespaced feature identifiers;
//! - deterministic compatibility reports and stable issue codes;
//! - separation between incompatibility and migration requirements;
//! - a pure assessment function with no I/O or global state.
//!
//! # Authority and ownership
//!
//! This module owns compatibility descriptors, consumer profiles, assessment
//! results, compatibility issue codes, and the rules implemented here.
//!
//! It does not own:
//! - Zamani source syntax or language-version policy;
//! - canonical AST, semantic types, or classical/quantum IR;
//! - serialization formats or their wire encoders/decoders;
//! - canonical IDs, source spans, or diagnostic rendering;
//! - migration execution, compiler selection, or runtime scheduling;
//! - hardware discovery, resource allocation, or backend capabilities.
//!
//! # Conservative compatibility policy
//!
//! Compatibility is accepted only when:
//! - the artifact and consumer identify the same contract domain;
//! - the artifact's contract version is supported by the consumer;
//! - the declared semantic version is supported by the consumer;
//! - every required artifact feature is supported by the consumer.
//!
//! Contract and semantic versions use a conservative rule: the major
//! versions must match, and the artifact's minor version must not exceed
//! the consumer's minor version. Patch versions do not change compatibility
//! under this policy.
//!
//! A compatible major version is necessary, but is not by itself proof that
//! two arbitrary programs are equivalent. Callers must also validate
//! serialization integrity, IR invariants, type/effect constraints, and any
//! domain-specific contracts.
//!
//! This module never performs migrations and never treats an unknown required
//! feature as optional. A future migration system must be explicit, loss-aware,
//! separately versioned, and must report when semantics cannot be preserved.
//!
//! # POCO-REAF and scalability
//!
//! There are no universal limits on hardware counts, source sizes, operation
//! counts, tensor ranks, feature counts, or collection sizes. Collections are
//! dynamically sized. Actual allocation failure and representational limits
//! remain operational constraints.
//!
//! Compatibility is independent of the target's size. Target capabilities
//! and resource feasibility must be assessed by their respective owners.
//!
//! # Integration
//!
//! - `classical/mod.rs`: declare `pub mod compatibility;`.
//! - `classical/error.rs`: callers may map an incompatible report to the
//!   existing `ClassicalErrorKind::VersionMismatch`; this module deliberately
//!   does not depend on that not-yet-integrated public module tree.
//! - serialization modules: construct a descriptor only after validating
//!   the artifact's envelope and metadata.
//! - canonical IR verification: validate the artifact's actual invariants
//!   separately; compatibility is not IR verification.
//! - `classical/validation/`: use the report as one input to validation.
//! - `classical/lowering/` and `classical/execution/`: reject incompatible
//!   inputs or request an explicit, separately implemented migration.
//! - grammar/specification and `grammar/grammar.md`: language and semantic
//!   version authorities must be documented there; do not infer them from
//!   the Rust package version.
//!
//! This file intentionally uses only the standard library. It does not
//! introduce a duplicate ID, span, IR, or general error type.
//!
//! # Freeze status
//!
//! Contract-first implementation. Repository integration, actual checkout
//! compilation, and downstream compatibility tests are still required.

use core::cmp::Ordering;
use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

/// Stable, three-component compatibility version.
///
/// This is a compatibility-contract version, not necessarily the crate
/// version, language release, or serialization format version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl CompatibilityVersion {
    /// Constructs a version from its numeric components.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }

    /// Compares the full version numerically.
    #[must_use]
    pub const fn compare(self, other: Self) -> Ordering {
        if self.major < other.major {
            Ordering::Less
        } else if self.major > other.major {
            Ordering::Greater
        } else if self.minor < other.minor {
            Ordering::Less
        } else if self.minor > other.minor {
            Ordering::Greater
        } else if self.patch < other.patch {
            Ordering::Less
        } else if self.patch > other.patch {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Applies the conservative artifact-to-consumer compatibility rule.
    ///
    /// An artifact can be read by a consumer when both use the same major
    /// version and the artifact's minor version is no newer than the
    /// consumer's minor version. Patch differences are accepted.
    ///
    /// This is a policy rule, not a general claim that all semantic-versioned
    /// systems are backward compatible.
    #[must_use]
    pub const fn is_supported_by(self, consumer: Self) -> bool {
        self.major == consumer.major && self.minor <= consumer.minor
    }
}

impl Ord for CompatibilityVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(*other)
    }
}

impl PartialOrd for CompatibilityVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for CompatibilityVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Errors raised when constructing an invalid compatibility descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityDescriptorError {
    /// The domain is empty or malformed.
    InvalidDomain(String),

    /// A feature identifier is empty or malformed.
    InvalidFeatureIdentifier(String),

    /// A feature was declared both required and optional.
    FeatureDeclaredAsRequiredAndOptional(String),
}

impl fmt::Display for CompatibilityDescriptorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDomain(value) => {
                write!(formatter, "invalid compatibility domain: {value:?}")
            }
            Self::InvalidFeatureIdentifier(value) => {
                write!(formatter, "invalid feature identifier: {value:?}")
            }
            Self::FeatureDeclaredAsRequiredAndOptional(value) => {
                write!(
                    formatter,
                    "feature is both required and optional: {value}"
                )
            }
        }
    }
}

impl Error for CompatibilityDescriptorError {}

/// A validated, namespaced compatibility descriptor.
///
/// Feature identifiers are extensible strings rather than a closed enum.
/// For example, `classical.numeric.arbitrary_precision` or
/// `classical.parallel.data_parallel`. Feature names have no meaning to this
/// module beyond identity and set membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityDescriptor {
    domain: String,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    required_features: BTreeSet<String>,
    optional_features: BTreeSet<String>,
}

impl CompatibilityDescriptor {
    /// Creates a descriptor and validates its domain and feature identifiers.
    pub fn new(
        domain: impl Into<String>,
        contract_version: CompatibilityVersion,
        semantic_version: CompatibilityVersion,
        required_features: impl IntoIterator<Item = String>,
        optional_features: impl IntoIterator<Item = String>,
    ) -> Result<Self, CompatibilityDescriptorError> {
        let domain = domain.into();

        if !is_valid_identifier(&domain) {
            return Err(CompatibilityDescriptorError::InvalidDomain(domain));
        }

        let required_features =
            collect_valid_features(required_features)?;
        let optional_features =
            collect_valid_features(optional_features)?;

        if let Some(feature) = required_features
            .intersection(&optional_features)
            .next()
        {
            return Err(
                CompatibilityDescriptorError::FeatureDeclaredAsRequiredAndOptional(
                    feature.clone(),
                ),
            );
        }

        Ok(Self {
            domain,
            contract_version,
            semantic_version,
            required_features,
            optional_features,
        })
    }

    /// Returns the descriptor's contract domain.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the descriptor's contract version.
    #[must_use]
    pub const fn contract_version(&self) -> CompatibilityVersion {
        self.contract_version
    }

    /// Returns the descriptor's semantic version.
    #[must_use]
    pub const fn semantic_version(&self) -> CompatibilityVersion {
        self.semantic_version
    }

    /// Returns required features in deterministic order.
    #[must_use]
    pub fn required_features(&self) -> &BTreeSet<String> {
        &self.required_features
    }

    /// Returns optional features in deterministic order.
    #[must_use]
    pub fn optional_features(&self) -> &BTreeSet<String> {
        &self.optional_features
    }
}

/// The versions and capabilities a consumer promises to support.
///
/// A consumer profile is an explicit declaration, not a probe of the current
/// machine. Hardware and runtime availability must be checked separately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityProfile {
    domain: String,
    contract_version: CompatibilityVersion,
    semantic_version: CompatibilityVersion,
    supported_features: BTreeSet<String>,
}

impl CompatibilityProfile {
    /// Creates a consumer profile with validated feature identifiers.
    pub fn new(
        domain: impl Into<String>,
        contract_version: CompatibilityVersion,
        semantic_version: CompatibilityVersion,
        supported_features: impl IntoIterator<Item = String>,
    ) -> Result<Self, CompatibilityDescriptorError> {
        let domain = domain.into();

        if !is_valid_identifier(&domain) {
            return Err(CompatibilityDescriptorError::InvalidDomain(domain));
        }

        let supported_features = collect_valid_features(supported_features)?;

        Ok(Self {
            domain,
            contract_version,
            semantic_version,
            supported_features,
        })
    }

    /// Returns the consumer's contract domain.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the consumer's contract version.
    #[must_use]
    pub const fn contract_version(&self) -> CompatibilityVersion {
        self.contract_version
    }

    /// Returns the consumer's semantic version.
    #[must_use]
    pub const fn semantic_version(&self) -> CompatibilityVersion {
        self.semantic_version
    }

    /// Returns supported features in deterministic order.
    #[must_use]
    pub fn supported_features(&self) -> &BTreeSet<String> {
        &self.supported_features
    }
}

/// Overall result of compatibility assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityStatus {
    /// All declared compatibility requirements are satisfied.
    Compatible,

    /// Required compatibility conditions hold, but optional features are
    /// unavailable or the artifact declares optional functionality unknown
    /// to this consumer.
    CompatibleWithWarnings,

    /// One or more required conditions are not satisfied.
    Incompatible,
}

impl CompatibilityStatus {
    /// Returns whether the artifact may proceed without migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Compatible | Self::CompatibleWithWarnings
        )
    }
}

/// Stable, machine-readable reason for a compatibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityIssueCode {
    /// Artifact and consumer identify different domains.
    DomainMismatch,

    /// Contract major versions differ.
    ContractMajorMismatch,

    /// Artifact contract minor version is newer than the consumer supports.
    ContractVersionTooNew,

    /// Semantic major versions differ.
    SemanticMajorMismatch,

    /// Artifact semantic minor version is newer than the consumer supports.
    SemanticVersionTooNew,

    /// The consumer does not implement a required artifact feature.
    MissingRequiredFeature,

    /// An optional artifact feature is unavailable.
    MissingOptionalFeature,
}

impl CompatibilityIssueCode {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DomainMismatch => "classical.compatibility.domain_mismatch",
            Self::ContractMajorMismatch => {
                "classical.compatibility.contract_major_mismatch"
            }
            Self::ContractVersionTooNew => {
                "classical.compatibility.contract_version_too_new"
            }
            Self::SemanticMajorMismatch => {
                "classical.compatibility.semantic_major_mismatch"
            }
            Self::SemanticVersionTooNew => {
                "classical.compatibility.semantic_version_too_new"
            }
            Self::MissingRequiredFeature => {
                "classical.compatibility.missing_required_feature"
            }
            Self::MissingOptionalFeature => {
                "classical.compatibility.missing_optional_feature"
            }
        }
    }

    /// Whether this issue makes the assessment incompatible.
    #[must_use]
    pub const fn is_blocking(self) -> bool {
        !matches!(self, Self::MissingOptionalFeature)
    }
}

impl fmt::Display for CompatibilityIssueCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// One machine-readable compatibility finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityIssue {
    code: CompatibilityIssueCode,
    subject: String,
    message: String,
}

impl CompatibilityIssue {
    fn new(
        code: CompatibilityIssueCode,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Returns the stable issue code.
    #[must_use]
    pub const fn code(&self) -> CompatibilityIssueCode {
        self.code
    }

    /// Returns the domain, version, or feature associated with the issue.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns explanatory text for humans.
    ///
    /// Consumers must use `code()` for programmatic decisions rather than
    /// parsing this message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Deterministic result of comparing an artifact with a consumer profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    status: CompatibilityStatus,
    issues: Vec<CompatibilityIssue>,
}

impl CompatibilityReport {
    fn from_issues(issues: Vec<CompatibilityIssue>) -> Self {
        let has_blocking_issue = issues.iter().any(|issue| issue.code.is_blocking());

        let status = if has_blocking_issue {
            CompatibilityStatus::Incompatible
        } else if issues.is_empty() {
            CompatibilityStatus::Compatible
        } else {
            CompatibilityStatus::CompatibleWithWarnings
        };

        Self { status, issues }
    }

    /// Returns the overall status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Returns all findings in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[CompatibilityIssue] {
        &self.issues
    }

    /// Returns whether the artifact satisfies required compatibility checks.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns whether any finding requires explicit remediation.
    ///
    /// Warnings do not automatically imply that remediation is necessary.
    #[must_use]
    pub fn has_blocking_issues(&self) -> bool {
        self.issues.iter().any(|issue| issue.code.is_blocking())
    }
}

/// Assesses an artifact against a consumer's declared compatibility profile.
///
/// The function is deterministic and side-effect free. It does not load,
/// execute, deserialize, migrate, or modify the artifact.
///
/// Domain and version failures are reported alongside feature failures so a
/// caller can inspect the full report. A caller must not proceed merely
/// because the report contains some non-blocking findings: it must check
/// `is_compatible()`.
#[must_use]
pub fn assess_compatibility(
    artifact: &CompatibilityDescriptor,
    consumer: &CompatibilityProfile,
) -> CompatibilityReport {
    let mut issues = Vec::new();

    if artifact.domain != consumer.domain {
        issues.push(CompatibilityIssue::new(
            CompatibilityIssueCode::DomainMismatch,
            artifact.domain.clone(),
            format!(
                "artifact domain {:?} does not match consumer domain {:?}",
                artifact.domain, consumer.domain
            ),
        ));
    }

    assess_version(
        artifact.contract_version,
        consumer.contract_version,
        true,
        &mut issues,
    );

    assess_version(
        artifact.semantic_version,
        consumer.semantic_version,
        false,
        &mut issues,
    );

    for feature in artifact
        .required_features
        .difference(&consumer.supported_features)
    {
        issues.push(CompatibilityIssue::new(
            CompatibilityIssueCode::MissingRequiredFeature,
            feature.clone(),
            format!("required feature {feature:?} is not supported"),
        ));
    }

    for feature in artifact
        .optional_features
        .difference(&consumer.supported_features)
    {
        issues.push(CompatibilityIssue::new(
            CompatibilityIssueCode::MissingOptionalFeature,
            feature.clone(),
            format!("optional feature {feature:?} is not supported"),
        ));
    }

    // BTreeSet iteration is stable; this final sort also makes report ordering
    // independent of the order in which different categories were assessed.
    issues.sort_by(|left, right| {
        left.code
            .cmp(&right.code)
            .then_with(|| left.subject.cmp(&right.subject))
            .then_with(|| left.message.cmp(&right.message))
    });

    CompatibilityReport::from_issues(issues)
}

fn assess_version(
    artifact: CompatibilityVersion,
    consumer: CompatibilityVersion,
    contract_version: bool,
    issues: &mut Vec<CompatibilityIssue>,
) {
    let (major_code, newer_code, label) = if contract_version {
        (
            CompatibilityIssueCode::ContractMajorMismatch,
            CompatibilityIssueCode::ContractVersionTooNew,
            "contract",
        )
    } else {
        (
            CompatibilityIssueCode::SemanticMajorMismatch,
            CompatibilityIssueCode::SemanticVersionTooNew,
            "semantic",
        )
    };

    if artifact.major != consumer.major {
        issues.push(CompatibilityIssue::new(
            major_code,
            label,
            format!(
                "{label} major version {} is incompatible with consumer version {}",
                artifact, consumer
            ),
        ));
    } else if artifact.minor > consumer.minor {
        issues.push(CompatibilityIssue::new(
            newer_code,
            label,
            format!(
                "artifact {label} version {} is newer than consumer version {}",
                artifact, consumer
            ),
        ));
    }
}

fn collect_valid_features(
    features: impl IntoIterator<Item = String>,
) -> Result<BTreeSet<String>, CompatibilityDescriptorError> {
    let mut result = BTreeSet::new();

    for feature in features {
        if !is_valid_identifier(&feature) {
            return Err(
                CompatibilityDescriptorError::InvalidFeatureIdentifier(feature),
            );
        }

        result.insert(feature);
    }

    Ok(result)
}

/// Validates an extensible ASCII identifier.
///
/// Components are separated by dots. Each component starts with an ASCII
/// letter or underscore and continues with ASCII letters, digits, underscores,
/// or hyphens. This permits namespaced identifiers without a closed registry.
///
/// Examples:
/// - `classical.numeric.arbitrary_precision`
/// - `classical.parallel.data-parallel`
/// - `vendor_extension.feature_v2`
fn is_valid_identifier(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    value.split('.').all(|component| {
        let mut chars = component.chars();

        matches!(
            chars.next(),
            Some('a'..='z' | 'A'..='Z' | '_')
        ) && chars.all(|character| {
            matches!(
                character,
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-'
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
        CompatibilityVersion::new(major, minor, patch)
    }

    fn descriptor(
        contract: CompatibilityVersion,
        semantics: CompatibilityVersion,
        required: &[&str],
        optional: &[&str],
    ) -> CompatibilityDescriptor {
        CompatibilityDescriptor::new(
            "classical.ir",
            contract,
            semantics,
            required.iter().map(|value| (*value).to_owned()),
            optional.iter().map(|value| (*value).to_owned()),
        )
        .expect("valid descriptor")
    }

    fn profile(
        contract: CompatibilityVersion,
        semantics: CompatibilityVersion,
        features: &[&str],
    ) -> CompatibilityProfile {
        CompatibilityProfile::new(
            "classical.ir",
            contract,
            semantics,
            features.iter().map(|value| (*value).to_owned()),
        )
        .expect("valid profile")
    }

    #[test]
    fn accepts_equal_versions_and_supported_features() {
        let artifact = descriptor(
            version(1, 2, 0),
            version(3, 1, 4),
            &["classical.numeric.checked_arithmetic"],
            &[],
        );
        let consumer = profile(
            version(1, 2, 9),
            version(3, 1, 0),
            &["classical.numeric.checked_arithmetic"],
        );

        let report = assess_compatibility(&artifact, &consumer);

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(report.issues().is_empty());
    }

    #[test]
    fn accepts_older_minor_contract_and_semantics_versions() {
        let artifact = descriptor(
            version(2, 3, 9),
            version(4, 1, 0),
            &[],
            &[],
        );
        let consumer = profile(
            version(2, 5, 0),
            version(4, 2, 0),
            &[],
        );

        assert!(assess_compatibility(&artifact, &consumer).is_compatible());
    }

    #[test]
    fn rejects_contract_major_mismatch() {
        let artifact = descriptor(version(1, 0, 0), version(1, 0, 0), &[], &[]);
        let consumer = profile(version(2, 0, 0), version(1, 0, 0), &[]);

        let report = assess_compatibility(&artifact, &consumer);

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
        assert!(report.issues().iter().any(|issue| {
            issue.code() == CompatibilityIssueCode::ContractMajorMismatch
        }));
    }

    #[test]
    fn rejects_artifact_minor_version_newer_than_consumer() {
        let artifact = descriptor(version(1, 4, 0), version(1, 0, 0), &[], &[]);
        let consumer = profile(version(1, 3, 99), version(1, 0, 0), &[]);

        let report = assess_compatibility(&artifact, &consumer);

        assert!(!report.is_compatible());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == CompatibilityIssueCode::ContractVersionTooNew
        }));
    }

    #[test]
    fn rejects_semantic_major_mismatch() {
        let artifact = descriptor(version(1, 0, 0), version(2, 0, 0), &[], &[]);
        let consumer = profile(version(1, 0, 0), version(3, 0, 0), &[]);

        let report = assess_compatibility(&artifact, &consumer);

        assert!(!report.is_compatible());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == CompatibilityIssueCode::SemanticMajorMismatch
        }));
    }

    #[test]
    fn rejects_missing_required_features() {
        let artifact = descriptor(
            version(1, 0, 0),
            version(1, 0, 0),
            &["classical.numeric.arbitrary_precision"],
            &[],
        );
        let consumer = profile(version(1, 0, 0), version(1, 0, 0), &[]);

        let report = assess_compatibility(&artifact, &consumer);

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
        assert!(report.issues().iter().any(|issue| {
            issue.code() == CompatibilityIssueCode::MissingRequiredFeature
        }));
    }

    #[test]
    fn missing_optional_features_produce_warnings_not_acceptance_silence() {
        let artifact = descriptor(
            version(1, 0, 0),
            version(1, 0, 0),
            &[],
            &["classical.numeric.fast_math"],
        );
        let consumer = profile(version(1, 0, 0), version(1, 0, 0), &[]);

        let report = assess_compatibility(&artifact, &consumer);

        assert_eq!(
            report.status(),
            CompatibilityStatus::CompatibleWithWarnings
        );
        assert!(report.is_compatible());
        assert_eq!(report.issues().len(), 1);
        assert_eq!(
            report.issues()[0].code(),
            CompatibilityIssueCode::MissingOptionalFeature
        );
    }

    #[test]
    fn rejects_domain_mismatch() {
        let artifact = descriptor(version(1, 0, 0), version(1, 0, 0), &[], &[]);
        let consumer = CompatibilityProfile::new(
            "classical.runtime",
            version(1, 0, 0),
            version(1, 0, 0),
            Vec::<String>::new(),
        )
        .expect("valid profile");

        let report = assess_compatibility(&artifact, &consumer);

        assert!(!report.is_compatible());
        assert!(report.issues().iter().any(|issue| {
            issue.code() == CompatibilityIssueCode::DomainMismatch
        }));
    }

    #[test]
    fn rejects_required_and_optional_overlap() {
        let result = CompatibilityDescriptor::new(
            "classical.ir",
            version(1, 0, 0),
            version(1, 0, 0),
            vec!["classical.feature".to_owned()],
            vec!["classical.feature".to_owned()],
        );

        assert!(matches!(
            result,
            Err(
                CompatibilityDescriptorError::FeatureDeclaredAsRequiredAndOptional(_)
            )
        ));
    }

    #[test]
    fn rejects_malformed_domain_and_feature_identifiers() {
        assert!(CompatibilityProfile::new(
            "",
            version(1, 0, 0),
            version(1, 0, 0),
            Vec::<String>::new(),
        )
        .is_err());

        assert!(CompatibilityProfile::new(
            "classical.ir",
            version(1, 0, 0),
            version(1, 0, 0),
            vec!["not valid".to_owned()],
        )
        .is_err());

        assert!(CompatibilityProfile::new(
            "classical..ir",
            version(1, 0, 0),
            version(1, 0, 0),
            Vec::<String>::new(),
        )
        .is_err());
    }

    #[test]
    fn duplicate_feature_declarations_are_deduplicated() {
        let profile = profile(
            version(1, 0, 0),
            version(1, 0, 0),
            &["classical.feature", "classical.feature"],
        );

        assert_eq!(profile.supported_features().len(), 1);
    }

    #[test]
    fn report_order_is_deterministic() {
        let artifact = descriptor(
            version(2, 1, 0),
            version(3, 1, 0),
            &["classical.zeta", "classical.alpha"],
            &["classical.optional"],
        );
        let consumer = profile(version(1, 0, 0), version(1, 0, 0), &[]);

        let first = assess_compatibility(&artifact, &consumer);
        let second = assess_compatibility(&artifact, &consumer);

        assert_eq!(first, second);
        assert!(first.has_blocking_issues());
    }

    #[test]
    fn very_large_version_components_do_not_overflow() {
        let high = version(u64::MAX, u64::MAX, u64::MAX);
        let lower = version(u64::MAX, u64::MAX - 1, u64::MAX);

        assert_eq!(high.compare(lower), Ordering::Greater);
        assert!(!high.is_supported_by(lower));
    }

    #[test]
    fn empty_feature_sets_are_valid() {
        let artifact = descriptor(version(1, 0, 0), version(1, 0, 0), &[], &[]);
        let consumer = profile(version(1, 0, 0), version(1, 0, 0), &[]);

        assert_eq!(
            assess_compatibility(&artifact, &consumer).status(),
            CompatibilityStatus::Compatible
        );
    }

    #[test]
    fn compatibility_assessment_does_not_mutate_inputs() {
        let artifact = descriptor(
            version(1, 0, 0),
            version(1, 0, 0),
            &["classical.required"],
            &[],
        );
        let consumer = profile(version(1, 0, 0), version(1, 0, 0), &[]);
        let artifact_before = artifact.clone();
        let consumer_before = consumer.clone();

        let _ = assess_compatibility(&artifact, &consumer);

        assert_eq!(artifact, artifact_before);
        assert_eq!(consumer, consumer_before);
    }
}
