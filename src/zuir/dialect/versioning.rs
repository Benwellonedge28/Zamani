
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Dialect Versioning
//!
//! Path: `src/zuir/dialect/versioning.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: `unsafe` code is forbidden.
//!
//! ## PURPOSE
//!
//! Defines the version-compatibility policy for ZUIR dialects.
//!
//! This module evaluates:
//! - Dialect dependency version requirements.
//! - Dialect version compatibility.
//! - Minimum ZUIR schema requirements.
//! - Minimum ZUIR serialization-format requirements.
//! - Explicit migration requirements.
//! - Compatibility results suitable for diagnostics and registration policy.
//!
//! It does not execute migrations, register dialects, load extension code,
//! authorize executable implementations, or lower operations to hardware.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/version.rs`
//! - `src/zuir/dialect/definition.rs`
//! - `src/zuir/spec/compatibility.rs`
//!
//! The specifications own normative meaning. This module implements the
//! dialect-versioning portion of those contracts.
//!
//! ## OWNS
//!
//! - Evaluation of `VersionRequirement`.
//! - Explicit compatibility outcomes for dialect requirements.
//! - Compatibility reports for dialect-declared ZUIR requirements.
//! - Migration-required versus compatible versus incompatible decisions.
//!
//! ## DOES_NOT_OWN
//!
//! - Canonical version types or global version constants.
//! - Dialect manifests or dialect registration.
//! - Operation schemas, type schemas, or executable implementations.
//! - Migration algorithms or migration execution.
//! - Serialization and deserialization.
//! - Target selection, hardware discovery, or physical resource allocation.
//! - Source-language parsing or semantic analysis.
//!
//! ## SCALABILITY AND PORTABILITY
//!
//! Versioning imposes no universal limits on program size, operation count,
//! dialect count, hardware capacity, qubit count, tensor rank, or topology.
//! The version components are defined by `version.rs`.
//!
//! Collections and input sizes remain subject to available process resources.
//! This module does not promise literal infinity.
//!
//! Unknown or future versions are never silently accepted.
//! A compatible version does not imply that an operation is implemented,
//! that executable extension code is trusted, or that a target can execute it.
//!
//! ## INTEGRATION
//!
//! `definition.rs` owns `DialectDefinition`, `DialectCompatibility`,
//! `VersionRequirement`, and the manifest-local validation rules.
//!
//! `version.rs` owns version identities and general version policy.
//!
//! The dialect registry owns installation state, dependency graph resolution,
//! namespace uniqueness, and registration transactions.
//!
//! The migration subsystem, when implemented, owns actual migration logic.
//! It must consult these decisions before interpreting or transforming data.
//!
//! No parser or grammar module should depend on this implementation.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::zuir::dialect::definition::{
    DialectCompatibility,
    DialectDefinition,
    DialectDefinitionError,
    DialectName,
    VersionRequirement,
};
use crate::zuir::version::{
    check_format_compatibility,
    check_schema_compatibility,
    Compatibility,
    CompatibilityDecision,
    CompatibilityPolicy,
    DialectVersion,
    VersionTriple,
    CURRENT_ZUIR_FORMAT_VERSION,
    CURRENT_ZUIR_SCHEMA_VERSION,
};

// =============================================================================
// Compatibility status
// =============================================================================

/// Result of evaluating a version requirement.
///
/// `Compatible` means the requirement is satisfied under the selected policy.
/// `RequiresMigration` means a migration must be found and applied before
/// normal interpretation.
/// `Incompatible` means the requirement cannot be satisfied under that policy.
///
/// None of these results establishes trust, implementation availability,
/// or target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    Compatible,
    RequiresMigration,
    Incompatible,
}

impl VersionStatus {
    /// Whether the version can be used without a migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Whether a migration must be resolved before interpretation.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::RequiresMigration)
    }

    /// Whether the version must be rejected under the selected policy.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

impl From<Compatibility> for VersionStatus {
    fn from(value: Compatibility) -> Self {
        match value {
            Compatibility::Compatible => Self::Compatible,
            Compatibility::RequiresMigration => Self::RequiresMigration,
            Compatibility::Incompatible => Self::Incompatible,
        }
    }
}

// =============================================================================
// Requirement evaluation
// =============================================================================

/// The result of checking one declared version requirement.
///
/// The original requirement and supplied version are retained so callers can
/// produce deterministic diagnostics without reconstructing the decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementDecision {
    requirement: VersionRequirement,
    actual: VersionTriple,
    status: VersionStatus,
}

impl RequirementDecision {
    /// Construct a requirement decision.
    #[must_use]
    pub const fn new(
        requirement: VersionRequirement,
        actual: VersionTriple,
        status: VersionStatus,
    ) -> Self {
        Self {
            requirement,
            actual,
            status,
        }
    }

    /// The declared requirement.
    #[must_use]
    pub const fn requirement(self) -> VersionRequirement {
        self.requirement
    }

    /// The version that was evaluated.
    #[must_use]
    pub const fn actual(self) -> VersionTriple {
        self.actual
    }

    /// The compatibility outcome.
    #[must_use]
    pub const fn status(self) -> VersionStatus {
        self.status
    }

    /// Whether this requirement is satisfied without migration.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        self.status.is_compatible()
    }
}

/// Explicit policy used to interpret registry-policy requirements.
///
/// `VersionRequirement::RegistryPolicy` deliberately delegates to a supplied
/// compatibility policy. It does not mean "accept every version".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VersioningPolicy {
    registry_policy: CompatibilityPolicy,
}

impl VersioningPolicy {
    /// Construct a versioning policy with an explicit registry policy.
    #[must_use]
    pub const fn new(registry_policy: CompatibilityPolicy) -> Self {
        Self { registry_policy }
    }

    /// Return the registry compatibility policy.
    #[must_use]
    pub const fn registry_policy(self) -> CompatibilityPolicy {
        self.registry_policy
    }

    /// Evaluate a manifest or dependency requirement against an actual version.
    ///
    /// Rules:
    /// - `Exact` requires exact equality.
    /// - `SameMajorAtLeast` requires the declared major and a version no lower
    ///   than the declared minimum.
    /// - `RegistryPolicy` delegates to the explicitly supplied policy.
    ///
    /// A major-version mismatch never becomes compatible merely because the
    /// numeric version is greater.
    pub fn evaluate_requirement(
        self,
        requirement: VersionRequirement,
        actual: VersionTriple,
    ) -> RequirementDecision {
        let status = match requirement {
            VersionRequirement::Exact {
                major,
                minor,
                patch,
            } => {
                let required = VersionTriple::new(major, minor, patch);

                if actual == required {
                    VersionStatus::Compatible
                } else {
                    VersionStatus::Incompatible
                }
            }

            VersionRequirement::SameMajorAtLeast {
                major,
                minor,
                patch,
            } => {
                let minimum = VersionTriple::new(major, minor, patch);

                if actual.major() != major || actual < minimum {
                    VersionStatus::Incompatible
                } else {
                    VersionStatus::Compatible
                }
            }

            VersionRequirement::RegistryPolicy => {
                self.registry_policy.check(actual).compatibility().into()
            }
        };

        RequirementDecision::new(requirement, actual, status)
    }

    /// Evaluate a dialect version against a declared requirement.
    #[must_use]
    pub fn evaluate_dialect_requirement(
        self,
        requirement: VersionRequirement,
        actual: DialectVersion,
    ) -> RequirementDecision {
        self.evaluate_requirement(requirement, actual.components())
    }
}

// =============================================================================
// Compatibility reports
// =============================================================================

/// Compatibility outcome for one dialect's declared ZUIR requirements.
///
/// Schema compatibility and serialization-format compatibility are separate:
/// being able to read an encoding does not prove that the encoded semantics
/// are supported, and schema compatibility does not prove decoding is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialectCompatibilityReport {
    schema: RequirementDecision,
    format: RequirementDecision,
}

impl DialectCompatibilityReport {
    /// Create a report from the two independent decisions.
    #[must_use]
    pub const fn new(
        schema: RequirementDecision,
        format: RequirementDecision,
    ) -> Self {
        Self { schema, format }
    }

    /// Result for the ZUIR logical schema requirement.
    #[must_use]
    pub const fn schema(self) -> RequirementDecision {
        self.schema
    }

    /// Result for the serialized format requirement.
    #[must_use]
    pub const fn format(self) -> RequirementDecision {
        self.format
    }

    /// Whether both dimensions are compatible without migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        self.schema.is_satisfied() && self.format.is_satisfied()
    }

    /// Whether at least one dimension requires migration.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        self.schema.status().requires_migration()
            || self.format.status().requires_migration()
    }

    /// Whether either dimension is incompatible.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        self.schema.status().is_incompatible()
            || self.format.status().is_incompatible()
    }
}

/// Evaluate a dialect's minimum schema and format requirements.
///
/// The caller supplies the policy for `RegistryPolicy` requirements. No
/// migration is performed here.
///
/// This function checks the currently declared ZUIR versions from `version.rs`.
/// If the application supports reading other versions, it must pass those
/// versions to `VersioningPolicy::evaluate_requirement` directly.
#[must_use]
pub fn evaluate_dialect_compatibility(
    compatibility: &DialectCompatibility,
    policy: VersioningPolicy,
) -> DialectCompatibilityReport {
    let schema = policy.evaluate_requirement(
        *compatibility.minimum_zuir_schema(),
        CURRENT_ZUIR_SCHEMA_VERSION.components(),
    );

    let format = policy.evaluate_requirement(
        *compatibility.minimum_zuir_format(),
        CURRENT_ZUIR_FORMAT_VERSION.components(),
    );

    DialectCompatibilityReport::new(schema, format)
}

/// Validate a complete manifest and evaluate its ZUIR compatibility.
///
/// This checks manifest-local invariants first. It does not check installed
/// dependencies, executable trust, operation-schema availability, or target
/// capability support; those belong to the registry and downstream stages.
pub fn evaluate_definition_compatibility(
    definition: &DialectDefinition,
    policy: VersioningPolicy,
) -> Result<DialectCompatibilityReport, VersioningError> {
    definition.validate()?;

    Ok(evaluate_dialect_compatibility(
        definition.compatibility(),
        policy,
    ))
}

// =============================================================================
// Migration decisions
// =============================================================================

/// Explicit action required before an artifact can be interpreted.
///
/// This is a planning result, not a migration implementation. A migration
/// executor must resolve and validate the exact source and target versions
/// and preserve the artifact's declared semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionAction {
    Interpret,
    MigrateBeforeInterpretation,
    Reject,
}

impl From<VersionStatus> for VersionAction {
    fn from(status: VersionStatus) -> Self {
        match status {
            VersionStatus::Compatible => Self::Interpret,
            VersionStatus::RequiresMigration => Self::MigrateBeforeInterpretation,
            VersionStatus::Incompatible => Self::Reject,
        }
    }
}

/// A structured plan for handling a version decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionPlan {
    action: VersionAction,
    decision: RequirementDecision,
}

impl VersionPlan {
    /// Build a plan from a compatibility decision.
    #[must_use]
    pub const fn new(decision: RequirementDecision) -> Self {
        Self {
            action: VersionAction::from(decision.status()),
            decision,
        }
    }

    /// Required handling action.
    #[must_use]
    pub const fn action(self) -> VersionAction {
        self.action
    }

    /// Underlying version decision.
    #[must_use]
    pub const fn decision(self) -> RequirementDecision {
        self.decision
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Failures encountered while validating or evaluating dialect versioning.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum VersioningError {
    /// The dialect manifest is locally invalid.
    #[error("invalid dialect definition: {0}")]
    InvalidDefinition(#[from] DialectDefinitionError),

    /// The manifest has incompatible ZUIR schema or format requirements.
    #[error(
        "dialect {dialect} is not compatible with the active ZUIR schema/format policy"
    )]
    IncompatibleZuirVersion {
        dialect: DialectName,
        report: DialectCompatibilityReport,
    },

    /// A required dependency is not present in the registry.
    ///
    /// This variant is provided for registry integration. The versioning
    /// module does not inspect registry state or produce it independently.
    #[error("required dialect dependency is missing: {dependency}")]
    MissingDependency { dependency: DialectName },

    /// An installed dependency does not satisfy its version requirement.
    #[error(
        "dialect dependency {dependency} has version {actual}, which does not satisfy its requirement"
    )]
    DependencyVersionMismatch {
        dependency: DialectName,
        actual: VersionTriple,
        decision: RequirementDecision,
    },
}

impl VersioningError {
    /// Stable machine-readable category.
    ///
    /// These strings are for diagnostics and tooling. They are not substitutes
    /// for matching the typed error variants inside Rust code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidDefinition(_) => "ZMN-IR-DIALECT-VERSION-INVALID-DEFINITION",
            Self::IncompatibleZuirVersion { .. } => {
                "ZMN-IR-DIALECT-VERSION-ZUIR-INCOMPATIBLE"
            }
            Self::MissingDependency { .. } => {
                "ZMN-IR-DIALECT-VERSION-MISSING-DEPENDENCY"
            }
            Self::DependencyVersionMismatch { .. } => {
                "ZMN-IR-DIALECT-VERSION-DEPENDENCY-MISMATCH"
            }
        }
    }
}

impl fmt::Display for VersionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Interpret => "interpret",
            Self::MigrateBeforeInterpretation => "migrate_before_interpretation",
            Self::Reject => "reject",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::dialect::definition::{
        DialectCompatibility,
        DialectDefinition,
        DialectImplementation,
        DialectName,
        DialectTrust,
        VersionRequirement,
    };
    use crate::zuir::version::{
        CompatibilityPolicy,
        DialectVersion,
        VersionTriple,
        INITIAL_DIALECT_VERSION,
    };

    fn exact_policy() -> VersioningPolicy {
        VersioningPolicy::new(CompatibilityPolicy::exact(
            CURRENT_ZUIR_SCHEMA_VERSION.components(),
        ))
    }

    fn definition(
        name: &str,
        compatibility: DialectCompatibility,
    ) -> DialectDefinition {
        DialectDefinition::new(
            DialectName::parse(name).expect("valid dialect name"),
            INITIAL_DIALECT_VERSION,
            compatibility,
            DialectImplementation::new(DialectTrust::DeclarativeOnly),
        )
        .expect("valid dialect definition")
    }

    fn compatibility(
        schema: VersionRequirement,
        format: VersionRequirement,
    ) -> DialectCompatibility {
        DialectCompatibility::new(schema, format, false)
            .expect("valid compatibility declaration")
    }

    #[test]
    fn exact_requirement_accepts_only_the_exact_version() {
        let policy = exact_policy();
        let required = VersionRequirement::exact(2, 3, 4);

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(2, 3, 4))
                .status(),
            VersionStatus::Compatible
        );

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(2, 3, 5))
                .status(),
            VersionStatus::Incompatible
        );
    }

    #[test]
    fn minimum_requirement_rejects_a_different_major() {
        let policy = exact_policy();
        let required = VersionRequirement::same_major_at_least(2, 1, 0);

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(2, 1, 0))
                .status(),
            VersionStatus::Compatible
        );

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(2, 8, 0))
                .status(),
            VersionStatus::Compatible
        );

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(3, 0, 0))
                .status(),
            VersionStatus::Incompatible
        );

        assert_eq!(
            policy
                .evaluate_requirement(required, VersionTriple::new(2, 0, 9))
                .status(),
            VersionStatus::Incompatible
        );
    }

    #[test]
    fn registry_policy_does_not_automatically_accept_future_versions() {
        let current = VersionTriple::new(4, 2, 0);
        let policy = VersioningPolicy::new(CompatibilityPolicy::exact(current));

        let decision = policy.evaluate_requirement(
            VersionRequirement::registry_policy(),
            VersionTriple::new(4, 2, 1),
        );

        assert_eq!(decision.status(), VersionStatus::Incompatible);
    }

    #[test]
    fn registry_policy_can_require_migration_for_older_versions() {
        let current = VersionTriple::new(2, 1, 0);
        let minimum = VersionTriple::new(2, 0, 0);

        let compatibility_policy =
            CompatibilityPolicy::new(current, minimum, false)
                .expect("valid version range");

        let policy = VersioningPolicy::new(compatibility_policy);

        let decision = policy.evaluate_requirement(
            VersionRequirement::registry_policy(),
            VersionTriple::new(2, 0, 3),
        );

        assert_eq!(decision.status(), VersionStatus::RequiresMigration);
        assert_eq!(
            VersionPlan::new(decision).action(),
            VersionAction::MigrateBeforeInterpretation
        );
    }

    #[test]
    fn schema_and_format_are_evaluated_independently() {
        let declaration = compatibility(
            VersionRequirement::registry_policy(),
            VersionRequirement::exact(99, 0, 0),
        );

        let report = evaluate_dialect_compatibility(
            &declaration,
            exact_policy(),
        );

        assert!(report.schema().is_satisfied());
        assert_eq!(report.format().status(), VersionStatus::Incompatible);
        assert!(report.is_incompatible());
        assert!(!report.is_compatible());
    }

    #[test]
    fn a_valid_manifest_can_be_checked_without_registering_it() {
        let declaration = compatibility(
            VersionRequirement::registry_policy(),
            VersionRequirement::registry_policy(),
        );

        let dialect = definition("example::dialect", declaration);
        let report = evaluate_definition_compatibility(
            &dialect,
            exact_policy(),
        )
        .expect("valid definition");

        assert!(report.is_compatible());
    }

    #[test]
    fn invalid_manifest_is_rejected_before_compatibility_evaluation() {
        let declaration = compatibility(
            VersionRequirement::registry_policy(),
            VersionRequirement::registry_policy(),
        );

        let mut dialect = definition("example::dialect", declaration);

        // Simulate an invalid state originating from deserialization or a
        // future internal construction path. Validation remains authoritative.
        dialect = dialect.with_description("   ");

        assert!(matches!(
            evaluate_definition_compatibility(&dialect, exact_policy()),
            Err(VersioningError::InvalidDefinition(_))
        ));
    }

    #[test]
    fn dialect_requirement_uses_the_canonical_version_type() {
        let policy = exact_policy();

        let decision = policy.evaluate_dialect_requirement(
            VersionRequirement::same_major_at_least(1, 0, 0),
            DialectVersion::new(1, 4, 2),
        );

        assert_eq!(decision.status(), VersionStatus::Compatible);
    }

    #[test]
    fn incompatible_versions_produce_a_reject_action() {
        let decision = RequirementDecision::new(
            VersionRequirement::exact(1, 0, 0),
            VersionTriple::new(2, 0, 0),
            VersionStatus::Incompatible,
        );

        assert_eq!(VersionPlan::new(decision).action(), VersionAction::Reject);
    }

    #[test]
    fn error_codes_are_stable_non_empty_identifiers() {
        let error = VersioningError::MissingDependency {
            dependency: DialectName::parse("example::dependency")
                .expect("valid name"),
        };

        assert_eq!(
            error.code(),
            "ZMN-IR-DIALECT-VERSION-MISSING-DEPENDENCY"
        );
    }
}
