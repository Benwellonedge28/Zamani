#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Compatibility Contract
//!
//! Path: `src/zuir/spec/compatibility.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the normative, machine-readable compatibility contract for ZUIR.
//!
//! Compatibility is multidimensional. Equal version numbers alone do not
//! establish semantic compatibility, and successful deserialization does
//! not establish that a representation may safely be interpreted or run.
//!
//! This module classifies compatibility, defines explicit policy decisions,
//! and records migration requirements. It does not perform migrations,
//! decode serialized payloads, verify concrete IR, or execute operations.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `grammar/DESIGN.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/spec/invariants.rs`
//!
//! The normative specifications own meaning. This module defines reusable
//! compatibility contracts; it does not override their authority.
//!
//! ## OWNS
//!
//! - Compatibility dimensions and decision outcomes.
//! - Change classifications and compatibility requirements.
//! - Explicit migration requirements and policy evaluation.
//! - Compatibility reports and stable identifiers for dimensions.
//! - Unit tests for deterministic policy behavior.
//!
//! ## DOES_NOT_OWN
//!
//! - Version identity or version-number comparison algorithms.
//! - Source grammar, tokens, AST structure, or semantic analysis.
//! - IR nodes, dialect definitions, or operation implementations.
//! - Migration execution, serialization, or deserialization.
//! - Target discovery, hardware allocation, routing, or scheduling.
//! - Claims that an implementation or migration has been tested.
//!
//! ## DEPENDENCIES
//!
//! Uses `crate::zuir::version::VersionTriple` for version identity.
//! Does not depend on concrete IR nodes, builders, parsers, or backends.
//!
//! ## SCALABILITY AND PORTABILITY
//!
//! Compatibility policy imposes no universal hardware or computation-size
//! ceilings. Collections grow dynamically. Allocation and execution still
//! require finite resources; literal infinity is not promised.
//!
//! Unknown required semantics, unsupported versions, and missing migrations
//! must never be silently accepted.
//!
//! ## FREEZE CRITERIA
//!
//! - Every compatibility dimension has a stable identifier.
//! - Compatibility is not reduced to a single version comparison.
//! - Migration requirements are explicit.
//! - Unknown required features fail closed.
//! - Resource exhaustion is not reported as semantic incompatibility.
//! - Tests cover policy boundaries and deterministic results.
//! - No unsafe code or hardware-specific assumptions are introduced.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

use crate::zuir::version::VersionTriple;

// ============================================================================
// Stable compatibility dimensions
// ============================================================================

/// A dimension along which two artifacts or implementations may be compared.
///
/// The serialized names of these variants are part of the compatibility
/// reporting contract. Additive variants are permitted; existing meanings
/// must not be silently changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CompatibilityDimension {
    Lexical,
    Grammar,
    SourceLanguage,
    Ast,
    TypeSystem,
    Semantics,
    Effects,
    Capabilities,
    Resources,
    Ownership,
    ControlFlow,
    NumericalBehavior,
    Concurrency,
    QuantumSemantics,
    HdlSemantics,
    DialectSchema,
    OperationSemantics,
    ZuirSchema,
    SerializationFormat,
    PublicApi,
    SymbolLinkage,
    Provenance,
    Diagnostics,
    Transformations,
    Lowering,
    RuntimeInterface,
    TargetRealization,
    Security,
    Determinism,
    Extensions,
}

impl CompatibilityDimension {
    /// Stable identifier suitable for reports, logs, and machine interfaces.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lexical => "lexical",
            Self::Grammar => "grammar",
            Self::SourceLanguage => "source_language",
            Self::Ast => "ast",
            Self::TypeSystem => "type_system",
            Self::Semantics => "semantics",
            Self::Effects => "effects",
            Self::Capabilities => "capabilities",
            Self::Resources => "resources",
            Self::Ownership => "ownership",
            Self::ControlFlow => "control_flow",
            Self::NumericalBehavior => "numerical_behavior",
            Self::Concurrency => "concurrency",
            Self::QuantumSemantics => "quantum_semantics",
            Self::HdlSemantics => "hdl_semantics",
            Self::DialectSchema => "dialect_schema",
            Self::OperationSemantics => "operation_semantics",
            Self::ZuirSchema => "zuir_schema",
            Self::SerializationFormat => "serialization_format",
            Self::PublicApi => "public_api",
            Self::SymbolLinkage => "symbol_linkage",
            Self::Provenance => "provenance",
            Self::Diagnostics => "diagnostics",
            Self::Transformations => "transformations",
            Self::Lowering => "lowering",
            Self::RuntimeInterface => "runtime_interface",
            Self::TargetRealization => "target_realization",
            Self::Security => "security",
            Self::Determinism => "determinism",
            Self::Extensions => "extensions",
        }
    }
}

impl fmt::Display for CompatibilityDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Change classification
// ============================================================================

/// The declared nature of a proposed change.
///
/// Classification is evidence for a decision, not proof that a change
/// actually preserves behavior. The compatibility owner must evaluate
/// the affected guarantees and available conformance evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeClass {
    /// Adds functionality without invalidating existing guarantees.
    Additive,

    /// Corrects behavior that violated an existing normative contract.
    BugFix,

    /// Changes internal representation without changing promised behavior.
    Internal,

    /// Requires a controlled conversion from an earlier representation.
    Migratable,

    /// Invalidates at least one previously guaranteed compatibility property.
    Breaking,

    /// Changes whose compatibility effects have not yet been established.
    Unknown,
}

/// The compatibility effect expected for an existing consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeImpact {
    Preserved,
    RequiresMigration,
    Breaking,
    Undetermined,
}

impl ChangeClass {
    /// Returns the conservative default impact of the classification.
    ///
    /// This does not replace an evidence-based review of the change.
    #[must_use]
    pub const fn default_impact(self) -> ChangeImpact {
        match self {
            Self::Additive | Self::Internal => ChangeImpact::Preserved,
            Self::BugFix => ChangeImpact::Undetermined,
            Self::Migratable => ChangeImpact::RequiresMigration,
            Self::Breaking => ChangeImpact::Breaking,
            Self::Unknown => ChangeImpact::Undetermined,
        }
    }
}

// ============================================================================
// Decision outcomes
// ============================================================================

/// Result of evaluating a compatibility requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityStatus {
    /// The evaluated contract is satisfied without migration.
    Compatible,

    /// An explicit migration is required before normal interpretation.
    RequiresMigration,

    /// A required contract is violated or unsupported.
    Incompatible,

    /// Insufficient evidence exists to establish compatibility.
    Unknown,

    /// Evaluation could not complete because operational resources ran out.
    ResourceExhausted,
}

/// Stable reason category for a compatibility decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityReason {
    ExactMatch,
    SupportedOlderVersion,
    MigrationRequired,
    FutureVersion,
    MajorVersionMismatch,
    BelowMinimumSupported,
    UnsupportedRequiredFeature,
    UnknownRequiredSemantics,
    ContractViolation,
    BreakingChange,
    MissingMigration,
    MigrationUnavailable,
    MigrationFailed,
    InsufficientEvidence,
    ResourceBudgetExceeded,
    PolicyRejected,
    NotEvaluated,
}

/// A migration requirement. This describes a migration; it does not execute it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRequirement {
    /// Stable, namespaced identifier of the migration.
    pub migration_id: String,

    /// Version of the input representation.
    pub from: VersionTriple,

    /// Version expected after a successful migration.
    pub to: VersionTriple,

    /// Human-readable explanation for tooling and diagnostics.
    pub rationale: String,

    /// Whether migration is required before semantic interpretation.
    pub required_before_interpretation: bool,
}

/// Decision for one compatibility dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionDecision {
    pub dimension: CompatibilityDimension,
    pub status: CompatibilityStatus,
    pub reason: CompatibilityReason,
    pub explanation: String,
    pub migration: Option<MigrationRequirement>,
}

impl DimensionDecision {
    /// Construct a decision without a migration requirement.
    #[must_use]
    pub fn new(
        dimension: CompatibilityDimension,
        status: CompatibilityStatus,
        reason: CompatibilityReason,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            dimension,
            status,
            reason,
            explanation: explanation.into(),
            migration: None,
        }
    }

    /// Construct a decision that requires a named migration.
    #[must_use]
    pub fn requiring_migration(
        dimension: CompatibilityDimension,
        explanation: impl Into<String>,
        migration: MigrationRequirement,
    ) -> Self {
        Self {
            dimension,
            status: CompatibilityStatus::RequiresMigration,
            reason: CompatibilityReason::MigrationRequired,
            explanation: explanation.into(),
            migration: Some(migration),
        }
    }

    /// Whether this dimension is safe to interpret without migration.
    #[must_use]
    pub const fn is_compatible_without_migration(&self) -> bool {
        matches!(self.status, CompatibilityStatus::Compatible)
    }
}

// ============================================================================
// Aggregate report
// ============================================================================

/// A report covering the dimensions examined by one compatibility evaluation.
///
/// Reports preserve all individual decisions so callers can explain failures
/// instead of collapsing distinct causes into an uninformative Boolean.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub decisions: Vec<DimensionDecision>,
}

impl CompatibilityReport {
    /// Construct an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            decisions: Vec::new(),
        }
    }

    /// Append one decision.
    pub fn push(&mut self, decision: DimensionDecision) {
        self.decisions.push(decision);
    }

    /// Return the aggregate status.
    ///
    /// Precedence is conservative:
    /// incompatible > resource exhausted > unknown >
    /// migration required > compatible.
    ///
    /// Resource exhaustion is kept distinct from semantic incompatibility.
    #[must_use]
    pub fn status(&self) -> CompatibilityStatus {
        if self.decisions.iter().any(|d| {
            d.status == CompatibilityStatus::Incompatible
        }) {
            return CompatibilityStatus::Incompatible;
        }

        if self.decisions.iter().any(|d| {
            d.status == CompatibilityStatus::ResourceExhausted
        }) {
            return CompatibilityStatus::ResourceExhausted;
        }

        if self.decisions.iter().any(|d| {
            d.status == CompatibilityStatus::Unknown
        }) {
            return CompatibilityStatus::Unknown;
        }

        if self.decisions.iter().any(|d| {
            d.status == CompatibilityStatus::RequiresMigration
        }) {
            return CompatibilityStatus::RequiresMigration;
        }

        if self.decisions.iter().all(|d| {
            d.status == CompatibilityStatus::Compatible
        }) {
            return CompatibilityStatus::Compatible;
        }

        CompatibilityStatus::Unknown
    }

    /// Whether every recorded dimension is compatible without migration.
    #[must_use]
    pub fn is_compatible_without_migration(&self) -> bool {
        !self.decisions.is_empty()
            && self.decisions.iter().all(|d| {
                d.status == CompatibilityStatus::Compatible
            })
    }

    /// Whether interpretation must be rejected pending migration or review.
    #[must_use]
    pub fn must_reject_without_resolution(&self) -> bool {
        !matches!(
            self.status(),
            CompatibilityStatus::Compatible
        )
    }

    /// Return the migration requirements in report order.
    #[must_use]
    pub fn migrations(&self) -> impl Iterator<Item = &MigrationRequirement> {
        self.decisions.iter().filter_map(|d| d.migration.as_ref())
    }

    /// Find the decision for a dimension.
    #[must_use]
    pub fn decision(
        &self,
        dimension: CompatibilityDimension,
    ) -> Option<&DimensionDecision> {
        self.decisions.iter().find(|d| d.dimension == dimension)
    }

    /// Sort decisions into a deterministic dimension order.
    ///
    /// If duplicate dimensions exist, their relative order is preserved by
    /// stable sorting. Producers should normally emit one decision per
    /// dimension; this method does not silently discard duplicates.
    pub fn sort_deterministically(&mut self) {
        self.decisions.sort_by_key(|d| d.dimension);
    }
}

impl Default for CompatibilityReport {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Policy
// ============================================================================

/// Policy for deciding which compatibility dimensions must be evaluated.
///
/// Empty required-dimension sets are allowed for incremental policy assembly,
/// but an empty report never proves compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityPolicy {
    /// Dimensions that must be evaluated before acceptance.
    pub required_dimensions: BTreeSet<CompatibilityDimension>,

    /// Whether explicitly named migrations may be used.
    pub allow_migrations: bool,

    /// Whether unknown optional extensions may be preserved as opaque data.
    ///
    /// This never authorizes execution, optimization, or lowering of unknown
    /// operations as if their semantics were known.
    pub allow_opaque_optional_extensions: bool,

    /// Whether insufficient evidence may be treated as compatible.
    ///
    /// Production execution policies should leave this false.
    pub allow_unknown_for_non_executable_artifacts: bool,
}

impl CompatibilityPolicy {
    /// Conservative policy for production compilation or execution.
    #[must_use]
    pub fn strict() -> Self {
        Self {
            required_dimensions: [
                CompatibilityDimension::Semantics,
                CompatibilityDimension::TypeSystem,
                CompatibilityDimension::Effects,
                CompatibilityDimension::Capabilities,
                CompatibilityDimension::Resources,
                CompatibilityDimension::Ownership,
                CompatibilityDimension::OperationSemantics,
                CompatibilityDimension::DialectSchema,
                CompatibilityDimension::ZuirSchema,
                CompatibilityDimension::SerializationFormat,
                CompatibilityDimension::Extensions,
                CompatibilityDimension::Lowering,
                CompatibilityDimension::Security,
            ]
            .into_iter()
            .collect(),
            allow_migrations: true,
            allow_opaque_optional_extensions: true,
            allow_unknown_for_non_executable_artifacts: false,
        }
    }

    /// Policy that requires exact compatibility evidence and disallows
    /// migration-dependent interpretation.
    #[must_use]
    pub fn exact() -> Self {
        let mut policy = Self::strict();
        policy.allow_migrations = false;
        policy.allow_opaque_optional_extensions = false;
        policy
    }

    /// Add a dimension that must be evaluated.
    pub fn require(&mut self, dimension: CompatibilityDimension) {
        self.required_dimensions.insert(dimension);
    }

    /// Remove a requirement from the policy.
    ///
    /// This only changes policy configuration. It does not assert that the
    /// corresponding semantic guarantee is unnecessary for a given task.
    pub fn make_optional(&mut self, dimension: CompatibilityDimension) {
        self.required_dimensions.remove(&dimension);
    }

    /// Check whether a report satisfies this policy.
    ///
    /// Every required dimension must be present and compatible. Missing
    /// required dimensions are unknown, not implicitly compatible.
    #[must_use]
    pub fn evaluate(&self, report: &CompatibilityReport) -> CompatibilityStatus {
        let mut aggregate = CompatibilityStatus::Compatible;

        if self.required_dimensions.is_empty() {
            return CompatibilityStatus::Unknown;
        }

        for required in &self.required_dimensions {
            let Some(decision) = report.decision(*required) else {
                return CompatibilityStatus::Unknown;
            };

            match decision.status {
                CompatibilityStatus::Incompatible => {
                    return CompatibilityStatus::Incompatible;
                }
                CompatibilityStatus::ResourceExhausted => {
                    aggregate = CompatibilityStatus::ResourceExhausted;
                }
                CompatibilityStatus::Unknown => {
                    if !self.allow_unknown_for_non_executable_artifacts {
                        aggregate = CompatibilityStatus::Unknown;
                    }
                }
                CompatibilityStatus::RequiresMigration => {
                    if !self.allow_migrations {
                        return CompatibilityStatus::Incompatible;
                    }
                    if aggregate == CompatibilityStatus::Compatible {
                        aggregate = CompatibilityStatus::RequiresMigration;
                    }
                }
                CompatibilityStatus::Compatible => {}
            }
        }

        aggregate
    }
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// Version-level helper
// ============================================================================

/// Check whether a source version can be interpreted as the requested
/// target version without assuming that a migration exists.
///
/// Version compatibility is only one dimension of compatibility. Callers
/// must separately evaluate semantic, dialect, extension, and target
/// compatibility.
#[must_use]
pub fn compare_version_triples(
    source: VersionTriple,
    target: VersionTriple,
    minimum_supported: VersionTriple,
    allow_older_same_major_without_migration: bool,
) -> DimensionDecision {
    let dimension = CompatibilityDimension::ZuirSchema;

    if source == target {
        return DimensionDecision::new(
            dimension,
            CompatibilityStatus::Compatible,
            CompatibilityReason::ExactMatch,
            "Source and target versions match exactly.",
        );
    }

    if source.major() != target.major() {
        return DimensionDecision::new(
            dimension,
            CompatibilityStatus::Incompatible,
            CompatibilityReason::MajorVersionMismatch,
            "Major version mismatch requires an explicitly supported migration.",
        );
    }

    if source > target {
        return DimensionDecision::new(
            dimension,
            CompatibilityStatus::Incompatible,
            CompatibilityReason::FutureVersion,
            "The source version is newer than the version understood by this reader.",
        );
    }

    if source < minimum_supported {
        return DimensionDecision::new(
            dimension,
            CompatibilityStatus::Incompatible,
            CompatibilityReason::BelowMinimumSupported,
            "The source version is older than the declared supported range.",
        );
    }

    if allow_older_same_major_without_migration {
        return DimensionDecision::new(
            dimension,
            CompatibilityStatus::Compatible,
            CompatibilityReason::SupportedOlderVersion,
            "The configured policy explicitly permits this older version.",
        );
    }

    DimensionDecision::new(
        dimension,
        CompatibilityStatus::RequiresMigration,
        CompatibilityReason::MigrationRequired,
        "An explicit migration is required before interpretation.",
    )
}

// ============================================================================
// Conformance contract
// ============================================================================

/// Requirements that a compatibility implementation must satisfy.
///
/// This is descriptive conformance metadata, not proof that an implementation
/// has passed the corresponding tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityConformance {
    pub requires_semantic_preservation: bool,
    pub requires_explicit_migrations: bool,
    pub rejects_unknown_required_semantics: bool,
    pub separates_resource_exhaustion: bool,
    pub requires_deterministic_reports: bool,
    pub forbids_unsafe_code: bool,
}

/// Normative minimum conformance requirements.
pub const REQUIRED_CONFORMANCE: CompatibilityConformance =
    CompatibilityConformance {
        requires_semantic_preservation: true,
        requires_explicit_migrations: true,
        rejects_unknown_required_semantics: true,
        separates_resource_exhaustion: true,
        requires_deterministic_reports: true,
        forbids_unsafe_code: true,
    };

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn compatible(dimension: CompatibilityDimension) -> DimensionDecision {
        DimensionDecision::new(
            dimension,
            CompatibilityStatus::Compatible,
            CompatibilityReason::ExactMatch,
            "Test contract satisfied.",
        )
    }

    #[test]
    fn stable_dimension_identifiers_are_unique() {
        let dimensions = [
            CompatibilityDimension::Lexical,
            CompatibilityDimension::Grammar,
            CompatibilityDimension::SourceLanguage,
            CompatibilityDimension::Ast,
            CompatibilityDimension::TypeSystem,
            CompatibilityDimension::Semantics,
            CompatibilityDimension::Effects,
            CompatibilityDimension::Capabilities,
            CompatibilityDimension::Resources,
            CompatibilityDimension::Ownership,
            CompatibilityDimension::ControlFlow,
            CompatibilityDimension::NumericalBehavior,
            CompatibilityDimension::Concurrency,
            CompatibilityDimension::QuantumSemantics,
            CompatibilityDimension::HdlSemantics,
            CompatibilityDimension::DialectSchema,
            CompatibilityDimension::OperationSemantics,
            CompatibilityDimension::ZuirSchema,
            CompatibilityDimension::SerializationFormat,
            CompatibilityDimension::PublicApi,
            CompatibilityDimension::SymbolLinkage,
            CompatibilityDimension::Provenance,
            CompatibilityDimension::Diagnostics,
            CompatibilityDimension::Transformations,
            CompatibilityDimension::Lowering,
            CompatibilityDimension::RuntimeInterface,
            CompatibilityDimension::TargetRealization,
            CompatibilityDimension::Security,
            CompatibilityDimension::Determinism,
            CompatibilityDimension::Extensions,
        ];

        let unique: BTreeSet<_> = dimensions.iter().map(|d| d.as_str()).collect();

        assert_eq!(unique.len(), dimensions.len());
    }

    #[test]
    fn exact_version_is_compatible() {
        let version = VersionTriple::new(1, 2, 3);

        let decision = compare_version_triples(
            version,
            version,
            version,
            false,
        );

        assert_eq!(decision.status, CompatibilityStatus::Compatible);
        assert_eq!(decision.reason, CompatibilityReason::ExactMatch);
    }

    #[test]
    fn future_version_is_rejected() {
        let decision = compare_version_triples(
            VersionTriple::new(2, 0, 0),
            VersionTriple::new(1, 9, 0),
            VersionTriple::new(1, 0, 0),
            true,
        );

        assert_eq!(decision.status, CompatibilityStatus::Incompatible);
        assert_eq!(decision.reason, CompatibilityReason::FutureVersion);
    }

    #[test]
    fn older_version_requires_migration_by_default() {
        let decision = compare_version_triples(
            VersionTriple::new(1, 2, 0),
            VersionTriple::new(1, 4, 0),
            VersionTriple::new(1, 0, 0),
            false,
        );

        assert_eq!(decision.status, CompatibilityStatus::RequiresMigration);
        assert_eq!(decision.reason, CompatibilityReason::MigrationRequired);
    }

    #[test]
    fn unsupported_old_version_is_rejected() {
        let decision = compare_version_triples(
            VersionTriple::new(1, 0, 0),
            VersionTriple::new(2, 0, 0),
            VersionTriple::new(1, 1, 0),
            true,
        );

        assert_eq!(decision.status, CompatibilityStatus::Incompatible);
        assert_eq!(decision.reason, CompatibilityReason::BelowMinimumSupported);
    }

    #[test]
    fn empty_report_does_not_prove_compatibility() {
        let report = CompatibilityReport::new();

        assert_eq!(report.status(), CompatibilityStatus::Unknown);
        assert!(!report.is_compatible_without_migration());
        assert!(report.must_reject_without_resolution());
    }

    #[test]
    fn incompatible_dimension_takes_precedence() {
        let mut report = CompatibilityReport::new();

        report.push(DimensionDecision::new(
            CompatibilityDimension::Semantics,
            CompatibilityStatus::Unknown,
            CompatibilityReason::InsufficientEvidence,
            "Semantic equivalence was not established.",
        ));

        report.push(DimensionDecision::new(
            CompatibilityDimension::Security,
            CompatibilityStatus::Incompatible,
            CompatibilityReason::ContractViolation,
            "A required security contract is violated.",
        ));

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn resource_exhaustion_is_not_semantic_incompatibility() {
        let mut report = CompatibilityReport::new();

        report.push(DimensionDecision::new(
            CompatibilityDimension::Semantics,
            CompatibilityStatus::ResourceExhausted,
            CompatibilityReason::ResourceBudgetExceeded,
            "The configured verification budget was exhausted.",
        ));

        assert_eq!(report.status(), CompatibilityStatus::ResourceExhausted);
        assert_ne!(report.status(), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn strict_policy_rejects_missing_required_dimensions() {
        let policy = CompatibilityPolicy::strict();
        let mut report = CompatibilityReport::new();

        report.push(compatible(CompatibilityDimension::Semantics));

        assert_eq!(policy.evaluate(&report), CompatibilityStatus::Unknown);
    }

    #[test]
    fn policy_accepts_complete_required_evidence() {
        let policy = CompatibilityPolicy {
            required_dimensions: [
                CompatibilityDimension::Semantics,
                CompatibilityDimension::TypeSystem,
            ]
            .into_iter()
            .collect(),
            allow_migrations: false,
            allow_opaque_optional_extensions: false,
            allow_unknown_for_non_executable_artifacts: false,
        };

        let mut report = CompatibilityReport::new();
        report.push(compatible(CompatibilityDimension::Semantics));
        report.push(compatible(CompatibilityDimension::TypeSystem));

        assert_eq!(policy.evaluate(&report), CompatibilityStatus::Compatible);
    }

    #[test]
    fn policy_rejects_migration_when_migrations_are_disabled() {
        let policy = CompatibilityPolicy {
            required_dimensions: [
                CompatibilityDimension::Semantics,
            ]
            .into_iter()
            .collect(),
            allow_migrations: false,
            allow_opaque_optional_extensions: false,
            allow_unknown_for_non_executable_artifacts: false,
        };

        let mut report = CompatibilityReport::new();
        report.push(DimensionDecision::new(
            CompatibilityDimension::Semantics,
            CompatibilityStatus::RequiresMigration,
            CompatibilityReason::MigrationRequired,
            "A semantic migration is needed.",
        ));

        assert_eq!(policy.evaluate(&report), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn migration_requirements_are_preserved() {
        let migration = MigrationRequirement {
            migration_id: "zuir.schema.v1_to_v2".to_owned(),
            from: VersionTriple::new(1, 0, 0),
            to: VersionTriple::new(2, 0, 0),
            rationale: "Explicit schema conversion.".to_owned(),
            required_before_interpretation: true,
        };

        let decision = DimensionDecision::requiring_migration(
            CompatibilityDimension::ZuirSchema,
            "Schema conversion is required.",
            migration.clone(),
        );

        assert_eq!(decision.status, CompatibilityStatus::RequiresMigration);
        assert_eq!(decision.migration, Some(migration));
    }

    #[test]
    fn report_order_can_be_canonicalized() {
        let mut report = CompatibilityReport::new();

        report.push(compatible(CompatibilityDimension::Security));
        report.push(compatible(CompatibilityDimension::Semantics));
        report.sort_deterministically();

        assert_eq!(
            report.decisions[0].dimension,
            CompatibilityDimension::Semantics
        );
        assert_eq!(
            report.decisions[1].dimension,
            CompatibilityDimension::Security
        );
    }

    #[test]
    fn policy_with_no_required_dimensions_is_not_accepted_as_proof() {
        let policy = CompatibilityPolicy {
            required_dimensions: BTreeSet::new(),
            allow_migrations: false,
            allow_opaque_optional_extensions: false,
            allow_unknown_for_non_executable_artifacts: false,
        };

        assert_eq!(
            policy.evaluate(&CompatibilityReport::new()),
            CompatibilityStatus::Unknown
        );
    }

    #[test]
    fn conformance_requirements_are_explicit() {
        assert!(REQUIRED_CONFORMANCE.requires_semantic_preservation);
        assert!(REQUIRED_CONFORMANCE.requires_explicit_migrations);
        assert!(REQUIRED_CONFORMANCE.rejects_unknown_required_semantics);
        assert!(REQUIRED_CONFORMANCE.separates_resource_exhaustion);
        assert!(REQUIRED_CONFORMANCE.requires_deterministic_reports);
        assert!(REQUIRED_CONFORMANCE.forbids_unsafe_code);
    }
}