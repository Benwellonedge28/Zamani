"src/hybrid/spec/compatibility.rs"

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Compatibility Contracts
//!
//! Path: `src/hybrid/spec/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the canonical hybrid-specific policy and assessment layer for
//! compatibility between persisted hybrid artifacts, hybrid schemas,
//! interchange formats, domain contracts, and semantic extensions.
//!
//! Generic numeric version comparison remains owned by
//! `crate::zuir::version`; hybrid version identities remain owned by
//! `crate::hybrid::version`; operational policies remain owned by
//! `crate::hybrid::config`.
//!
//! This module combines those contracts into an explicit decision about
//! whether a hybrid artifact can be interpreted directly, requires a
//! registered migration, is incompatible, or cannot yet be assessed.
//!
//! ## Architectural boundaries
//!
//! This module does not own:
//! - Zamani source-language syntax or language-version acceptance;
//! - the canonical semantic model, AST, classical IR, quantum IR, or ZUIR;
//! - hardware discovery, target selection, physical allocation, routing,
//!   scheduling, calibration, QEC, or vendor-specific execution;
//! - serialization and deserialization implementations;
//! - migration execution;
//! - compiler or runtime orchestration.
//!
//! A compatible artifact version does not prove that every domain operation,
//! capability, target, or execution environment is supported.
//!
//! ## Compatibility rules
//!
//! 1. Schema and encoding versions are checked independently.
//! 2. Future versions are never silently accepted.
//! 3. A registered migration is not evidence that its implementation exists
//!    or that the migration preserves semantics.
//! 4. Required unknown extensions are rejected.
//! 5. Optional unknown extensions may be preserved or ignored only under an
//!    explicit policy and only when they do not affect required semantics.
//! 6. Domain contract compatibility is decided using the policy supplied by
//!    the authoritative domain registry or adapter.
//! 7. Compatibility checks never silently weaken a program requirement.
//! 8. Reports use deterministic ordering independent of map iteration order.
//!
//! ## Scalability
//!
//! This module defines no universal limits on domains, nodes, values,
//! operations, extensions, or hardware resources.
//!
//! Callers may impose operational budgets through `HybridConfig` and the
//! interchange implementation. Budget exhaustion must be reported explicitly.
//!
//! ## Integration
//!
//! - `super::super::version` provides hybrid version identities and current
//!   version constants.
//! - `super::super::config` provides older-artifact policy.
//! - `super::super::errors` provides canonical hybrid errors.
//! - `super::invariants` should enforce the semantic invariants associated
//!   with compatibility decisions.
//! - `hybrid/interchange/decode.rs` should assess compatibility before
//!   interpreting artifact contents.
//! - `hybrid/interchange/encode.rs` should emit canonical current versions.
//! - `hybrid/domain/registry.rs` should supply authoritative domain policies.
//! - `compiler/hybrid_pipeline.rs` should use the canonical format identifier.
//!
//! This module assesses compatibility; it does not decode artifacts, execute
//! migrations, or claim that an artifact has passed semantic verification.

use core::cmp::Ordering;
use core::fmt;
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::super::config::OlderArtifactPolicy;
use super::super::errors::{HybridError, HybridErrorCode, HybridResult};
use super::super::version::{
    check_domain_contract_compatibility,
    check_hybrid_format_compatibility,
    check_hybrid_schema_compatibility,
    DomainContractVersion,
    HybridFormatVersion,
    HybridSchemaVersion,
};
use crate::zuir::version::{
    Compatibility,
    CompatibilityDecision,
    CompatibilityPolicy,
    CompatibilityReason,
    VersionTriple,
};

/// Category of versioned object being assessed.
///
/// These categories deliberately remain distinct. A matching encoding
/// version does not establish schema compatibility, and neither establishes
/// compatibility with a participating domain contract.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilitySubject {
    /// Logical hybrid representation schema.
    HybridSchema,

    /// Serialized hybrid interchange format.
    HybridFormat,

    /// A participating domain's declared contract.
    DomainContract,

    /// An optional or required semantic extension.
    Extension,
}

/// Outcome of a compatibility assessment.
///
/// `Compatible` means only that the assessed subject passes the supplied
/// compatibility policy. It does not mean that the full program is executable
/// on a particular target.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityStatus {
    /// The subject can be interpreted under the selected policy without
    /// migration.
    Compatible,

    /// A migration is required before interpretation.
    MigrationRequired,

    /// A mandatory contract or compatibility condition is not satisfied.
    Incompatible,

    /// Available information is insufficient to establish compatibility.
    Unknown,
}

impl CompatibilityStatus {
    /// Returns whether this status permits interpretation without migration.
    #[must_use]
    pub const fn permits_direct_interpretation(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether this status is a definitive rejection.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether additional evidence or implementation is needed.
    #[must_use]
    pub const fn requires_follow_up(self) -> bool {
        matches!(self, Self::MigrationRequired | Self::Unknown)
    }
}

/// Machine-readable reason for a hybrid compatibility result.
///
/// Keep this enum independent of human-readable error messages. Consumers
/// should branch on variants or stable codes, never parse display strings.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityIssue {
    /// The supplied version matches the current version.
    ExactVersionMatch,

    /// The supplied version is a supported older version.
    SupportedOlderVersion,

    /// A version migration must run before interpretation.
    MigrationRequired,

    /// No registered migration describes the required version transition.
    MigrationNotRegistered,

    /// A migration is described but its implementation has not been verified.
    MigrationImplementationUnverified,

    /// The supplied version is newer than the reader's current version.
    FutureVersion,

    /// The version belongs to an incompatible major-version line.
    MajorVersionMismatch,

    /// The supplied version is outside the supported historical range.
    BelowMinimumSupported,

    /// The source and target domain contracts are incompatible.
    DomainContractMismatch,

    /// The required domain contract could not be resolved.
    DomainContractUnknown,

    /// An extension is required but is not recognized.
    UnknownRequiredExtension,

    /// An optional extension is unknown to the current implementation.
    UnknownOptionalExtension,

    /// An optional extension is ignored under explicit policy.
    OptionalExtensionIgnored,

    /// The compatibility policy does not provide enough evidence.
    InsufficientEvidence,

    /// A semantic preservation guarantee could not be established.
    SemanticPreservationUnproven,

    /// A required feature or guarantee is unsupported.
    RequiredSemanticsUnsupported,
}

impl CompatibilityIssue {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ExactVersionMatch => "HYBC001",
            Self::SupportedOlderVersion => "HYBC002",
            Self::MigrationRequired => "HYBC003",
            Self::MigrationNotRegistered => "HYBC004",
            Self::MigrationImplementationUnverified => "HYBC005",
            Self::FutureVersion => "HYBC006",
            Self::MajorVersionMismatch => "HYBC007",
            Self::BelowMinimumSupported => "HYBC008",
            Self::DomainContractMismatch => "HYBC009",
            Self::DomainContractUnknown => "HYBC010",
            Self::UnknownRequiredExtension => "HYBC011",
            Self::UnknownOptionalExtension => "HYBC012",
            Self::OptionalExtensionIgnored => "HYBC013",
            Self::InsufficientEvidence => "HYBC014",
            Self::SemanticPreservationUnproven => "HYBC015",
            Self::RequiredSemanticsUnsupported => "HYBC016",
        }
    }

    /// Returns whether this issue must prevent direct interpretation.
    #[must_use]
    pub const fn blocks_direct_interpretation(&self) -> bool {
        matches!(
            self,
            Self::MigrationRequired
                | Self::MigrationNotRegistered
                | Self::MigrationImplementationUnverified
                | Self::FutureVersion
                | Self::MajorVersionMismatch
                | Self::BelowMinimumSupported
                | Self::DomainContractMismatch
                | Self::DomainContractUnknown
                | Self::UnknownRequiredExtension
                | Self::InsufficientEvidence
                | Self::SemanticPreservationUnproven
                | Self::RequiredSemanticsUnsupported
        )
    }
}

impl fmt::Display for CompatibilityIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ExactVersionMatch => "version matches exactly",
            Self::SupportedOlderVersion => "older version is supported directly",
            Self::MigrationRequired => "migration is required",
            Self::MigrationNotRegistered => "required migration is not registered",
            Self::MigrationImplementationUnverified => {
                "migration implementation has not been verified"
            }
            Self::FutureVersion => "artifact uses a future version",
            Self::MajorVersionMismatch => "major versions are incompatible",
            Self::BelowMinimumSupported => {
                "version is below the supported historical range"
            }
            Self::DomainContractMismatch => "domain contracts are incompatible",
            Self::DomainContractUnknown => "domain contract compatibility is unknown",
            Self::UnknownRequiredExtension => {
                "required extension is not recognized"
            }
            Self::UnknownOptionalExtension => {
                "optional extension is not recognized"
            }
            Self::OptionalExtensionIgnored => {
                "unknown optional extension is ignored by policy"
            }
            Self::InsufficientEvidence => {
                "insufficient evidence to establish compatibility"
            }
            Self::SemanticPreservationUnproven => {
                "semantic preservation has not been established"
            }
            Self::RequiredSemanticsUnsupported => {
                "required program semantics are unsupported"
            }
        })
    }
}

/// A single structured finding in a compatibility report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFinding {
    /// Subject category.
    pub subject: CompatibilitySubject,

    /// Stable issue category.
    pub issue: CompatibilityIssue,

    /// Stable machine-readable issue code.
    pub code: String,

    /// Whether this finding prevents direct interpretation.
    pub blocking: bool,

    /// Optional stable subject identifier, such as a domain or extension ID.
    ///
    /// This is descriptive metadata, not a replacement for canonical IDs
    /// owned by other Zamani subsystems.
    pub subject_id: Option<String>,

    /// Explanatory detail intended for diagnostics and logs.
    ///
    /// Consumers must not parse this field to determine compatibility.
    pub detail: String,
}

impl CompatibilityFinding {
    /// Creates a finding using the issue's stable code and blocking rule.
    #[must_use]
    pub fn new(
        subject: CompatibilitySubject,
        issue: CompatibilityIssue,
        subject_id: Option<String>,
        detail: impl Into<String>,
    ) -> Self {
        let code = issue.code().to_owned();
        let blocking = issue.blocks_direct_interpretation();

        Self {
            subject,
            issue,
            code,
            blocking,
            subject_id,
            detail: detail.into(),
        }
    }
}

/// Complete report for a compatibility assessment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    /// Final status after applying all required checks.
    pub status: CompatibilityStatus,

    /// Findings sorted into deterministic order.
    pub findings: Vec<CompatibilityFinding>,

    /// Whether every mandatory check has sufficient evidence.
    pub evidence_complete: bool,

    /// Whether interpretation without migration is allowed.
    pub direct_interpretation_allowed: bool,
}

impl CompatibilityReport {
    /// Creates a report and derives its status from its findings.
    ///
    /// Findings are sorted deterministically by subject, subject identifier,
    /// issue, and stable code. Human-readable detail is not used for semantic
    /// classification.
    #[must_use]
    pub fn from_findings(
        mut findings: Vec<CompatibilityFinding>,
        evidence_complete: bool,
    ) -> Self {
        findings.sort_by(|left, right| {
            left.subject
                .cmp(&right.subject)
                .then_with(|| left.subject_id.cmp(&right.subject_id))
                .then_with(|| left.issue.cmp(&right.issue))
                .then_with(|| left.code.cmp(&right.code))
                .then_with(|| left.detail.cmp(&right.detail))
        });

        let has_incompatible = findings.iter().any(|finding| {
            matches!(
                finding.issue,
                CompatibilityIssue::FutureVersion
                    | CompatibilityIssue::MajorVersionMismatch
                    | CompatibilityIssue::BelowMinimumSupported
                    | CompatibilityIssue::DomainContractMismatch
                    | CompatibilityIssue::UnknownRequiredExtension
                    | CompatibilityIssue::RequiredSemanticsUnsupported
            )
        });

        let has_unknown = !evidence_complete
            || findings.iter().any(|finding| {
                matches!(
                    finding.issue,
                    CompatibilityIssue::DomainContractUnknown
                        | CompatibilityIssue::InsufficientEvidence
                        | CompatibilityIssue::SemanticPreservationUnproven
                )
            });

        let has_migration = findings.iter().any(|finding| {
            matches!(
                finding.issue,
                CompatibilityIssue::MigrationRequired
                    | CompatibilityIssue::MigrationNotRegistered
                    | CompatibilityIssue::MigrationImplementationUnverified
            )
        });

        let has_blocking = findings.iter().any(|finding| finding.blocking);

        let status = if has_incompatible {
            CompatibilityStatus::Incompatible
        } else if has_unknown {
            CompatibilityStatus::Unknown
        } else if has_migration {
            CompatibilityStatus::MigrationRequired
        } else if has_blocking {
            CompatibilityStatus::Incompatible
        } else {
            CompatibilityStatus::Compatible
        };

        let direct_interpretation_allowed =
            matches!(status, CompatibilityStatus::Compatible)
                && evidence_complete;

        Self {
            status,
            findings,
            evidence_complete,
            direct_interpretation_allowed,
        }
    }

    /// Returns `true` only if the report permits direct interpretation.
    #[must_use]
    pub const fn allows_direct_interpretation(&self) -> bool {
        self.direct_interpretation_allowed
    }

    /// Returns the number of findings that block direct interpretation.
    #[must_use]
    pub fn blocking_finding_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.blocking)
            .count()
    }

    /// Returns findings with the supplied stable issue code.
    #[must_use]
    pub fn findings_with_code<'a>(
        &'a self,
        code: &'a str,
    ) -> impl Iterator<Item = &'a CompatibilityFinding> + 'a {
        self.findings
            .iter()
            .filter(move |finding| finding.code == code)
    }
}

/// Explicit policy for hybrid schema and format compatibility.
///
/// The generic policy type and version comparison rules are owned by
/// `crate::zuir::version`. This structure binds those rules to the distinct
/// hybrid schema and format identities and to the configured historical
/// artifact policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridCompatibilityPolicy {
    /// Policy for logical schema versions.
    pub schema: CompatibilityPolicy,

    /// Policy for serialized format versions.
    pub format: CompatibilityPolicy,

    /// Handling of artifacts older than the current version.
    pub older_artifacts: OlderArtifactPolicy,
}

impl Default for HybridCompatibilityPolicy {
    fn default() -> Self {
        Self {
            schema: CompatibilityPolicy::exact(
                super::super::version::CURRENT_HYBRID_SCHEMA_VERSION.components(),
            ),
            format: CompatibilityPolicy::exact(
                super::super::version::CURRENT_HYBRID_FORMAT_VERSION.components(),
            ),
            older_artifacts: OlderArtifactPolicy::RequireMigration,
        }
    }
}

impl HybridCompatibilityPolicy {
    /// Constructs an explicit policy.
    #[must_use]
    pub const fn new(
        schema: CompatibilityPolicy,
        format: CompatibilityPolicy,
        older_artifacts: OlderArtifactPolicy,
    ) -> Self {
        Self {
            schema,
            format,
            older_artifacts,
        }
    }

    /// Assesses a hybrid schema version.
    #[must_use]
    pub fn assess_schema(
        &self,
        supplied: HybridSchemaVersion,
    ) -> CompatibilityFinding {
        let decision = self.schema.check(supplied.components());

        finding_from_version_decision(
            CompatibilitySubject::HybridSchema,
            decision,
            self.older_artifacts,
            None,
        )
    }

    /// Assesses a hybrid interchange-format version.
    #[must_use]
    pub fn assess_format(
        &self,
        supplied: HybridFormatVersion,
    ) -> CompatibilityFinding {
        let decision = self.format.check(supplied.components());

        finding_from_version_decision(
            CompatibilitySubject::HybridFormat,
            decision,
            self.older_artifacts,
            None,
        )
    }

    /// Assesses schema and format versions independently.
    ///
    /// Both checks are required. A matching schema does not compensate for
    /// an incompatible encoding, and a matching encoding does not prove that
    /// the logical schema is compatible.
    #[must_use]
    pub fn assess_artifact_versions(
        &self,
        schema: HybridSchemaVersion,
        format: HybridFormatVersion,
    ) -> CompatibilityReport {
        let findings = vec![
            self.assess_schema(schema),
            self.assess_format(format),
        ];

        CompatibilityReport::from_findings(findings, true)
    }
}

fn finding_from_version_decision(
    subject: CompatibilitySubject,
    decision: CompatibilityDecision,
    older_policy: OlderArtifactPolicy,
    subject_id: Option<String>,
) -> CompatibilityFinding {
    let (issue, detail) = match decision.reason() {
        CompatibilityReason::ExactMatch => (
            CompatibilityIssue::ExactVersionMatch,
            format!(
                "supplied version {} matches current version {}",
                decision.source(),
                decision.target()
            ),
        ),

        CompatibilityReason::SupportedOlderVersion => {
            match older_policy {
                OlderArtifactPolicy::AllowIfCompatible => (
                    CompatibilityIssue::SupportedOlderVersion,
                    format!(
                        "version {} is explicitly allowed by the compatibility policy",
                        decision.source()
                    ),
                ),
                OlderArtifactPolicy::Reject
                | OlderArtifactPolicy::RequireMigration => (
                    CompatibilityIssue::MigrationRequired,
                    format!(
                        "older version {} is not accepted directly under the configured policy",
                        decision.source()
                    ),
                ),
            }
        }

        CompatibilityReason::MigrationRequired => (
            CompatibilityIssue::MigrationRequired,
            format!(
                "version {} requires migration to {}",
                decision.source(),
                decision.target()
            ),
        ),

        CompatibilityReason::FutureVersion => (
            CompatibilityIssue::FutureVersion,
            format!(
                "version {} is newer than supported version {}",
                decision.source(),
                decision.target()
            ),
        ),

        CompatibilityReason::MajorVersionMismatch => (
            CompatibilityIssue::MajorVersionMismatch,
            format!(
                "version {} has a major version incompatible with {}",
                decision.source(),
                decision.target()
            ),
        ),

        CompatibilityReason::BelowMinimumSupported => (
            CompatibilityIssue::BelowMinimumSupported,
            format!(
                "version {} is below the supported historical range",
                decision.source()
            ),
        ),

        CompatibilityReason::OutsideAllowedRange => (
            CompatibilityIssue::InsufficientEvidence,
            format!(
                "version {} is outside the configured compatibility range",
                decision.source()
            ),
        ),
    };

    let mut finding = CompatibilityFinding::new(
        subject,
        issue,
        subject_id,
        detail,
    );

    // A version policy may classify a version as directly compatible, but
    // an explicit artifact policy can still require migration or reject it.
    if matches!(
        decision.compatibility(),
        Compatibility::Incompatible
    ) {
        finding.blocking = true;
    }

    if matches!(
        decision.compatibility(),
        Compatibility::RequiresMigration
    ) {
        finding.blocking = true;
    }

    if matches!(older_policy, OlderArtifactPolicy::Reject)
        && !matches!(decision.reason(), CompatibilityReason::ExactMatch)
    {
        finding.blocking = true;
    }

    finding
}

/// Describes a version transition that a migration implementation claims
/// to support.
///
/// Registering this descriptor does not implement or execute the migration.
/// The interchange layer must separately resolve and invoke a trusted
/// implementation and verify its semantic-preservation contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MigrationDescriptor {
    /// Version category to which the migration applies.
    pub subject: CompatibilitySubject,

    /// Source version.
    pub from: VersionTriple,

    /// Destination version.
    pub to: VersionTriple,

    /// Stable migration identifier.
    pub migration_id: String,

    /// Whether the implementation has been independently verified.
    ///
    /// `true` is an assertion supplied by the registering implementation,
    /// not proof produced by this module.
    pub implementation_verified: bool,

    /// Whether the migration declares preservation of required semantics.
    pub preserves_required_semantics: bool,
}

impl MigrationDescriptor {
    /// Creates a migration descriptor.
    ///
    /// Invalid identifiers and no-op transitions are rejected. Versions must
    /// be ordered explicitly by the caller; this constructor does not assume
    /// that every older version can be upgraded directly to every newer one.
    pub fn new(
        subject: CompatibilitySubject,
        from: VersionTriple,
        to: VersionTriple,
        migration_id: impl Into<String>,
        implementation_verified: bool,
        preserves_required_semantics: bool,
    ) -> HybridResult<Self> {
        let migration_id = migration_id.into();

        if migration_id.trim().is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "migration identifier must not be empty",
            ));
        }

        if from == to {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "migration source and destination versions must differ",
            ));
        }

        if !is_valid_stable_identifier(&migration_id) {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "migration identifier contains unsupported characters",
            ));
        }

        Ok(Self {
            subject,
            from,
            to,
            migration_id,
            implementation_verified,
            preserves_required_semantics,
        })
    }

    /// Returns whether this descriptor claims a safe, verified migration.
    #[must_use]
    pub const fn is_verified_semantics_preserving(&self) -> bool {
        self.implementation_verified && self.preserves_required_semantics
    }
}

/// Validates a portable stable identifier.
///
/// The grammar intentionally avoids assuming a maximum identifier length.
/// Input-size and decoding budgets belong to the caller's explicit policy.
fn is_valid_stable_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(byte, b'-' | b'_' | b'.' | b':')
        })
}

/// A deterministic collection of declared migration transitions.
///
/// This is a metadata registry, not an executable migration engine.
/// Duplicate transitions with different migration IDs are rejected to avoid
/// silently choosing one implementation based on registration order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRegistry {
    entries: Vec<MigrationDescriptor>,
}

impl MigrationRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Registers a migration descriptor.
    ///
    /// An identical descriptor may be registered repeatedly without changing
    /// the registry. Conflicting descriptors for the same transition fail.
    pub fn register(
        &mut self,
        descriptor: MigrationDescriptor,
    ) -> HybridResult<()> {
        if let Some(existing) = self.entries.iter().find(|existing| {
            existing.subject == descriptor.subject
                && existing.from == descriptor.from
                && existing.to == descriptor.to
        }) {
            if existing == &descriptor {
                return Ok(());
            }

            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!(
                    "conflicting migration descriptors for {} transition {} -> {}",
                    descriptor.subject,
                    descriptor.from,
                    descriptor.to
                ),
            ));
        }

        self.entries.push(descriptor);
        self.entries.sort_by(|left, right| {
            left.subject
                .cmp(&right.subject)
                .then_with(|| left.from.cmp(&right.from))
                .then_with(|| left.to.cmp(&right.to))
                .then_with(|| left.migration_id.cmp(&right.migration_id))
        });

        Ok(())
    }

    /// Returns the registered descriptor for an exact transition.
    #[must_use]
    pub fn find(
        &self,
        subject: CompatibilitySubject,
        from: VersionTriple,
        to: VersionTriple,
    ) -> Option<&MigrationDescriptor> {
        self.entries.iter().find(|entry| {
            entry.subject == subject && entry.from == from && entry.to == to
        })
    }

    /// Returns all registered descriptors in deterministic order.
    #[must_use]
    pub fn entries(&self) -> &[MigrationDescriptor] {
        &self.entries
    }

    /// Assesses whether an explicit migration is available and sufficiently
    /// declared for a version transition.
    #[must_use]
    pub fn assess_transition(
        &self,
        subject: CompatibilitySubject,
        from: VersionTriple,
        to: VersionTriple,
    ) -> CompatibilityFinding {
        match self.find(subject, from, to) {
            None => CompatibilityFinding::new(
                subject,
                CompatibilityIssue::MigrationNotRegistered,
                None,
                format!(
                    "no migration is registered for transition {} -> {}",
                    from, to
                ),
            ),

            Some(descriptor)
                if !descriptor.implementation_verified =>
            {
                CompatibilityFinding::new(
                    subject,
                    CompatibilityIssue::MigrationImplementationUnverified,
                    Some(descriptor.migration_id.clone()),
                    "the migration implementation is not verified",
                )
            }

            Some(descriptor)
                if !descriptor.preserves_required_semantics =>
            {
                CompatibilityFinding::new(
                    subject,
                    CompatibilityIssue::SemanticPreservationUnproven,
                    Some(descriptor.migration_id.clone()),
                    "the migration does not declare preservation of required semantics",
                )
            }

            Some(descriptor) => CompatibilityFinding::new(
                subject,
                CompatibilityIssue::MigrationRequired,
                Some(descriptor.migration_id.clone()),
                "a verified migration is registered; it must still be executed before interpretation",
            ),
        }
    }
}

/// Whether an unknown optional extension is retained or ignored.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OptionalExtensionPolicy {
    /// Preserve the extension as opaque data where the interchange format
    /// and decoder support lossless preservation.
    Preserve,

    /// Ignore the extension only when it is explicitly non-semantic and
    /// cannot affect required behavior.
    IgnoreNonSemantic,
}

/// Declared extension encountered during compatibility assessment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDeclaration {
    /// Stable extension identifier.
    pub id: String,

    /// Whether the producer marks this extension as mandatory.
    pub required: bool,

    /// Whether this extension can affect required program semantics.
    pub affects_required_semantics: bool,

    /// Whether the current implementation recognizes this extension.
    pub recognized: bool,
}

impl ExtensionDeclaration {
    /// Creates and validates an extension declaration.
    pub fn new(
        id: impl Into<String>,
        required: bool,
        affects_required_semantics: bool,
        recognized: bool,
    ) -> HybridResult<Self> {
        let id = id.into();

        if !is_valid_stable_identifier(&id) {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "extension identifier is empty or invalid",
            ));
        }

        if required && !recognized {
            // Preserve the declaration so that assessment can produce the
            // appropriate structured incompatibility result.
        }

        Ok(Self {
            id,
            required,
            affects_required_semantics,
            recognized,
        })
    }
}

/// Assesses declared extensions under an explicit optional-extension policy.
///
/// Required unknown extensions always block interpretation. Optional unknown
/// extensions that affect required semantics also block interpretation.
/// Preservation is allowed only as an explicit representation policy; it
/// never establishes that the current implementation understands the data.
#[must_use]
pub fn assess_extensions(
    extensions: &[ExtensionDeclaration],
    optional_policy: OptionalExtensionPolicy,
) -> CompatibilityReport {
    let mut findings = Vec::new();

    for extension in extensions {
        if extension.recognized {
            continue;
        }

        if extension.required {
            findings.push(CompatibilityFinding::new(
                CompatibilitySubject::Extension,
                CompatibilityIssue::UnknownRequiredExtension,
                Some(extension.id.clone()),
                "the artifact requires an extension that this implementation does not recognize",
            ));
            continue;
        }

        if extension.affects_required_semantics {
            findings.push(CompatibilityFinding::new(
                CompatibilitySubject::Extension,
                CompatibilityIssue::RequiredSemanticsUnsupported,
                Some(extension.id.clone()),
                "an unknown optional extension affects required semantics and cannot be ignored safely",
            ));
            continue;
        }

        match optional_policy {
            OptionalExtensionPolicy::Preserve => {
                findings.push(CompatibilityFinding::new(
                    CompatibilitySubject::Extension,
                    CompatibilityIssue::UnknownOptionalExtension,
                    Some(extension.id.clone()),
                    "the extension is unknown and must be preserved opaquely by the interchange implementation",
                ));
            }

            OptionalExtensionPolicy::IgnoreNonSemantic => {
                findings.push(CompatibilityFinding::new(
                    CompatibilitySubject::Extension,
                    CompatibilityIssue::OptionalExtensionIgnored,
                    Some(extension.id.clone()),
                    "the unknown extension is declared non-semantic and is ignored by explicit policy",
                ));
            }
        }
    }

    CompatibilityReport::from_findings(findings, true)
}

/// Assesses a participating domain contract using the authoritative policy
/// supplied by that domain's registry or adapter.
///
/// This function deliberately does not define a universal current version
/// for every computational domain.
#[must_use]
pub fn assess_domain_contract(
    domain_id: impl Into<String>,
    supplied: DomainContractVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityFinding {
    let domain_id = domain_id.into();
    let decision = check_domain_contract_compatibility(supplied, policy);

    let issue = match decision.reason() {
        CompatibilityReason::ExactMatch => CompatibilityIssue::ExactVersionMatch,
        CompatibilityReason::SupportedOlderVersion => {
            CompatibilityIssue::SupportedOlderVersion
        }
        CompatibilityReason::MigrationRequired => {
            CompatibilityIssue::MigrationRequired
        }
        CompatibilityReason::FutureVersion => CompatibilityIssue::FutureVersion,
        CompatibilityReason::MajorVersionMismatch => {
            CompatibilityIssue::DomainContractMismatch
        }
        CompatibilityReason::BelowMinimumSupported => {
            CompatibilityIssue::DomainContractMismatch
        }
        CompatibilityReason::OutsideAllowedRange => {
            CompatibilityIssue::InsufficientEvidence
        }
    };

    CompatibilityFinding::new(
        CompatibilitySubject::DomainContract,
        issue,
        Some(domain_id.clone()),
        format!(
            "domain `{domain_id}` supplied contract version {}; policy target is {}",
            decision.source(),
            decision.target()
        ),
    )
}

/// Assesses the current hybrid schema version using the canonical version
/// policy. This is a convenience function for callers that use the default
/// compatibility behavior.
#[must_use]
pub fn assess_current_schema(
    supplied: HybridSchemaVersion,
) -> CompatibilityFinding {
    let decision = check_hybrid_schema_compatibility(supplied);

    finding_from_version_decision(
        CompatibilitySubject::HybridSchema,
        decision,
        OlderArtifactPolicy::RequireMigration,
        None,
    )
}

/// Assesses the current hybrid interchange-format version.
#[must_use]
pub fn assess_current_format(
    supplied: HybridFormatVersion,
) -> CompatibilityFinding {
    let decision = check_hybrid_format_compatibility(supplied);

    finding_from_version_decision(
        CompatibilitySubject::HybridFormat,
        decision,
        OlderArtifactPolicy::RequireMigration,
        None,
    )
}

/// Combines schema, format, domain, and extension findings into one report.
///
/// The caller is responsible for providing a finding for every required
/// domain contract and mandatory compatibility condition. If required checks
/// were skipped or evidence is missing, set `evidence_complete` to `false`.
#[must_use]
pub fn combine_assessments(
    findings: impl IntoIterator<Item = CompatibilityFinding>,
    evidence_complete: bool,
) -> CompatibilityReport {
    CompatibilityReport::from_findings(
        findings.into_iter().collect(),
        evidence_complete,
    )
}

/// Converts a version decision to the generic status vocabulary.
///
/// This helper intentionally does not execute migrations or assume that a
/// compatible version proves semantic or target compatibility.
#[must_use]
pub const fn status_from_version_decision(
    decision: CompatibilityDecision,
) -> CompatibilityStatus {
    match decision.compatibility() {
        Compatibility::Compatible => CompatibilityStatus::Compatible,
        Compatibility::RequiresMigration => CompatibilityStatus::MigrationRequired,
        Compatibility::Incompatible => CompatibilityStatus::Incompatible,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn current_policy() -> HybridCompatibilityPolicy {
        HybridCompatibilityPolicy::default()
    }

    #[test]
    fn current_schema_and_format_are_compatible() {
        let report = current_policy().assess_artifact_versions(
            super::super::super::version::CURRENT_HYBRID_SCHEMA_VERSION,
            super::super::super::version::CURRENT_HYBRID_FORMAT_VERSION,
        );

        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert!(report.allows_direct_interpretation());
        assert!(report.evidence_complete);
    }

    #[test]
    fn schema_and_format_are_checked_independently() {
        let policy = current_policy();

        let schema = policy.assess_schema(
            super::super::super::version::CURRENT_HYBRID_SCHEMA_VERSION,
        );
        let format = policy.assess_format(
            super::super::super::version::HybridFormatVersion::new(2, 0, 0),
        );

        assert_eq!(schema.issue, CompatibilityIssue::ExactVersionMatch);
        assert_eq!(format.issue, CompatibilityIssue::MajorVersionMismatch);
        assert!(format.blocking);
    }

    #[test]
    fn future_versions_are_not_silently_accepted() {
        let finding = current_policy().assess_schema(
            HybridSchemaVersion::new(u32::MAX, u32::MAX, u32::MAX),
        );

        assert_eq!(finding.issue, CompatibilityIssue::FutureVersion);
        assert!(finding.blocking);
    }

    #[test]
    fn migration_registration_does_not_execute_migration() {
        let from = VersionTriple::new(1, 0, 0);
        let to = VersionTriple::new(1, 1, 0);

        let descriptor = MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            from,
            to,
            "schema-1-to-1",
            true,
            true,
        )
        .expect("valid migration descriptor");

        let mut registry = MigrationRegistry::new();

        registry
            .register(descriptor)
            .expect("registration should succeed");

        let finding = registry.assess_transition(
            CompatibilitySubject::HybridSchema,
            from,
            to,
        );

        assert_eq!(finding.issue, CompatibilityIssue::MigrationRequired);
        assert!(finding.blocking);
        assert_eq!(registry.entries().len(), 1);
    }

    #[test]
    fn missing_migration_is_explicit() {
        let registry = MigrationRegistry::new();

        let finding = registry.assess_transition(
            CompatibilitySubject::HybridSchema,
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(1, 1, 0),
        );

        assert_eq!(finding.issue, CompatibilityIssue::MigrationNotRegistered);
        assert!(finding.blocking);
    }

    #[test]
    fn conflicting_migration_descriptors_are_rejected() {
        let from = VersionTriple::new(1, 0, 0);
        let to = VersionTriple::new(1, 1, 0);

        let first = MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            from,
            to,
            "migration-a",
            true,
            true,
        )
        .unwrap();

        let second = MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            from,
            to,
            "migration-b",
            true,
            true,
        )
        .unwrap();

        let mut registry = MigrationRegistry::new();
        registry.register(first).unwrap();

        assert_eq!(
            registry.register(second).unwrap_err().code(),
            HybridErrorCode::DuplicateIdentity
        );
    }

    #[test]
    fn required_unknown_extensions_block_compatibility() {
        let extensions = vec![
            ExtensionDeclaration::new("future.required", true, true, false)
                .unwrap(),
        ];

        let report = assess_extensions(
            &extensions,
            OptionalExtensionPolicy::Preserve,
        );

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert!(!report.allows_direct_interpretation());
        assert_eq!(
            report.findings[0].issue,
            CompatibilityIssue::UnknownRequiredExtension
        );
    }

    #[test]
    fn unknown_optional_nonsemantic_extensions_can_be_preserved() {
        let extensions = vec![
            ExtensionDeclaration::new("future.optional", false, false, false)
                .unwrap(),
        ];

        let report = assess_extensions(
            &extensions,
            OptionalExtensionPolicy::Preserve,
        );

        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert!(report.allows_direct_interpretation());
        assert_eq!(
            report.findings[0].issue,
            CompatibilityIssue::UnknownOptionalExtension
        );
    }

    #[test]
    fn unknown_optional_extensions_affecting_semantics_are_rejected() {
        let extensions = vec![
            ExtensionDeclaration::new("future.behavior", false, true, false)
                .unwrap(),
        ];

        let report = assess_extensions(
            &extensions,
            OptionalExtensionPolicy::IgnoreNonSemantic,
        );

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert!(!report.allows_direct_interpretation());
    }

    #[test]
    fn ignored_extensions_require_nonsemantic_declaration() {
        let extensions = vec![
            ExtensionDeclaration::new("future.annotation", false, false, false)
                .unwrap(),
        ];

        let report = assess_extensions(
            &extensions,
            OptionalExtensionPolicy::IgnoreNonSemantic,
        );

        assert_eq!(report.status, CompatibilityStatus::Compatible);
        assert_eq!(
            report.findings[0].issue,
            CompatibilityIssue::OptionalExtensionIgnored
        );
    }

    #[test]
    fn incomplete_evidence_never_reports_direct_compatibility() {
        let report = combine_assessments(Vec::new(), false);

        assert_eq!(report.status, CompatibilityStatus::Unknown);
        assert!(!report.allows_direct_interpretation());
    }

    #[test]
    fn report_order_is_deterministic() {
        let first = CompatibilityFinding::new(
            CompatibilitySubject::Extension,
            CompatibilityIssue::UnknownOptionalExtension,
            Some("z.extension".to_owned()),
            "optional",
        );

        let second = CompatibilityFinding::new(
            CompatibilitySubject::Extension,
            CompatibilityIssue::UnknownOptionalExtension,
            Some("a.extension".to_owned()),
            "optional",
        );

        let forward = combine_assessments(
            vec![first.clone(), second.clone()],
            true,
        );
        let reverse = combine_assessments(vec![second, first], true);

        assert_eq!(forward, reverse);
        assert_eq!(
            forward.findings[0].subject_id.as_deref(),
            Some("a.extension")
        );
    }

    #[test]
    fn migration_identifiers_are_validated() {
        assert!(MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(1, 1, 0),
            "",
            true,
            true,
        )
        .is_err());

        assert!(MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(1, 1, 0),
            "invalid identifier!",
            true,
            true,
        )
        .is_err());
    }

    #[test]
    fn no_op_migrations_are_rejected() {
        let version = VersionTriple::new(1, 0, 0);

        assert!(MigrationDescriptor::new(
            CompatibilitySubject::HybridSchema,
            version,
            version,
            "no-op",
            true,
            true,
        )
        .is_err());
    }

    #[test]
    fn issue_codes_are_stable_and_unique() {
        let issues = [
            CompatibilityIssue::ExactVersionMatch,
            CompatibilityIssue::SupportedOlderVersion,
            CompatibilityIssue::MigrationRequired,
            CompatibilityIssue::MigrationNotRegistered,
            CompatibilityIssue::MigrationImplementationUnverified,
            CompatibilityIssue::FutureVersion,
            CompatibilityIssue::MajorVersionMismatch,
            CompatibilityIssue::BelowMinimumSupported,
            CompatibilityIssue::DomainContractMismatch,
            CompatibilityIssue::DomainContractUnknown,
            CompatibilityIssue::UnknownRequiredExtension,
            CompatibilityIssue::UnknownOptionalExtension,
            CompatibilityIssue::OptionalExtensionIgnored,
            CompatibilityIssue::InsufficientEvidence,
            CompatibilityIssue::SemanticPreservationUnproven,
            CompatibilityIssue::RequiredSemanticsUnsupported,
        ];

        let codes = issues
            .iter()
            .map(CompatibilityIssue::code)
            .collect::<BTreeSet<_>>();

        assert_eq!(codes.len(), issues.len());
    }

    #[test]
    fn domain_compatibility_uses_the_callers_policy() {
        let version = DomainContractVersion::new(1, 0, 0);
        let policy = CompatibilityPolicy::exact(version.components());

        let finding = assess_domain_contract(
            "quantum",
            version,
            policy,
        );

        assert_eq!(finding.issue, CompatibilityIssue::ExactVersionMatch);
        assert!(!finding.blocking);
    }

    #[test]
    fn future_or_incompatible_versions_do_not_become_compatible() {
        let policy = current_policy();

        let report = policy.assess_artifact_versions(
            HybridSchemaVersion::new(1, 0, 0),
            HybridFormatVersion::new(9, 0, 0),
        );

        assert_eq!(report.status, CompatibilityStatus::Incompatible);
        assert!(!report.allows_direct_interpretation());
    }
}