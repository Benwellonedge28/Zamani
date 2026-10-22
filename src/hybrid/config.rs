
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Configuration and Operational Policies
//!
//! Path: `src/hybrid/config.rs`
//!
//! ## Purpose
//!
//! Defines explicit policies for validating, representing, and exchanging
//! hybrid computations across classical, quantum, accelerator, tensor,
//! AI/ML, HDL, distributed, networking, and future computation domains.
//!
//! Configuration governs how the implementation operates. It does not
//! redefine what a Zamani program means or which hardware capacities exist.
//!
//! ## Ownership
//!
//! This module owns:
//! - Explicit operational budgets.
//! - Hybrid graph-validation policy.
//! - Determinism and canonical-output policy.
//! - Interchange decoding and unknown-field policy.
//! - Older-artifact compatibility policy.
//! - Budget lookup and budget-exhaustion checks.
//!
//! This module does not own:
//! - Language semantics, type checking, or source-language configuration.
//! - Domain-specific IRs or a universal computational IR.
//! - Physical resource discovery or device allocation.
//! - Target selection, scheduling, routing, QEC, or code generation.
//! - Serialization implementation or schema migration.
//! - Runtime sessions, vendor SDKs, or hardware handles.
//!
//! ## Scalability
//!
//! All operational limits are optional. `None` means that this configuration
//! imposes no limit for that category. `Some(limit)` is an explicit policy
//! supplied by the caller or deployment environment.
//!
//! A limit is an operational safeguard, not a limit on valid Zamani programs,
//! the number of hardware devices, or the capacities of a target.
//!
//! No finite computer has infinite resources. Allocation, address-space,
//! operating-system, and target constraints remain real and must be reported
//! explicitly rather than hidden in language semantics.
//!
//! ## Safety
//!
//! This module uses safe Rust only. Unsafe code is forbidden.
//!
//! ## Integration contract
//!
//! - Uses `super::errors::HybridError` as the canonical hybrid error.
//! - Is consumed by graph builders, graph verification, interchange, and
//!   planning through explicit references or owned configuration values.
//! - Must not depend on compiler pipelines, runtime engines, domain IRs,
//!   hardware discovery, or vendor-specific APIs.
//! - Existing compiler and profiler limits must be migrated deliberately;
//!   defining a budget here does not automatically override other modules.

use serde::{Deserialize, Serialize};

use super::errors::HybridError;

/// Top-level configuration for hybrid-computation operations.
///
/// All nested policies have defaults. Missing fields in a deserialized
/// configuration therefore receive documented defaults, allowing compatible
/// configurations to omit fields introduced in later releases.
///
/// Unknown fields are not rejected by this structure. This permits a newer
/// producer to attach fields that an older reader does not understand.
/// Whether unknown fields may be preserved or ignored semantically is governed
/// by the explicit serialization policy and the concrete decoder.
///
/// This configuration never permits an implementation to silently change
/// required program semantics to satisfy a budget or target limitation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct HybridConfig {
    /// Explicit operational limits. All limits default to unspecified.
    pub budgets: OperationalBudgets,

    /// Policy for checking hybrid structures and their references.
    pub validation: ValidationPolicy,

    /// Policy for deterministic representation and observable output.
    pub determinism: DeterminismPolicy,

    /// Policy for decoding and encoding hybrid artifacts.
    pub serialization: SerializationPolicy,

    /// Policy for artifacts produced by older schema versions.
    pub compatibility: ArtifactCompatibilityPolicy,
}

impl HybridConfig {
    /// Returns the configured limit for a budget category.
    ///
    /// `None` means that no limit for this category has been configured.
    #[must_use]
    pub const fn budget_limit(&self, kind: BudgetKind) -> Option<u64> {
        self.budgets.limit(kind)
    }

    /// Checks an observed amount against its explicitly configured budget.
    ///
    /// The observed value is `u128` so that callers can report measured
    /// quantities without first narrowing them to a platform-sized integer.
    ///
    /// The check is inclusive: an observation equal to the configured limit
    /// succeeds; an observation greater than it fails.
    ///
    /// An absent limit never causes a budget-exhaustion error. It does not
    /// guarantee that the operating system or allocator can satisfy a request.
    pub fn check_budget(
        &self,
        kind: BudgetKind,
        observed: u128,
    ) -> Result<(), HybridError> {
        let Some(limit) = self.budget_limit(kind) else {
            return Ok(());
        };

        if observed <= u128::from(limit) {
            return Ok(());
        }

        Err(HybridError::budget_exhausted(
            kind.as_str(),
            format!(
                "observed {observed}; configured limit is {limit}"
            ),
        ))
    }
}

/// The category of an explicitly configurable operational budget.
///
/// These limits concern work performed by the implementation. They are not
/// universal limits on the expressiveness of Zamani or on hardware capacity.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BudgetKind {
    /// Source bytes accepted by an operation.
    SourceBytes,

    /// Bytes emitted by an interchange encoder.
    EncodedBytes,

    /// Bytes consumed by an interchange decoder.
    DecodedBytes,

    /// Number of nodes in one hybrid composition graph.
    GraphNodes,

    /// Number of edges in one hybrid composition graph.
    GraphEdges,

    /// Number of domain instances in one composition.
    DomainInstances,

    /// Nesting depth processed by an operation.
    NestingDepth,

    /// Number of validation work units performed.
    ValidationSteps,

    /// Number of planning work units performed.
    PlanningSteps,

    /// Number of diagnostics retained by an operation.
    DiagnosticCount,

    /// Bytes retained for diagnostic messages and metadata.
    MetadataBytes,

    /// Total operational work units, when a caller defines such accounting.
    WorkUnits,
}

impl BudgetKind {
    /// Stable identifier for diagnostics and configuration integrations.
    ///
    /// These identifiers are not source-language keywords.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceBytes => "source_bytes",
            Self::EncodedBytes => "encoded_bytes",
            Self::DecodedBytes => "decoded_bytes",
            Self::GraphNodes => "graph_nodes",
            Self::GraphEdges => "graph_edges",
            Self::DomainInstances => "domain_instances",
            Self::NestingDepth => "nesting_depth",
            Self::ValidationSteps => "validation_steps",
            Self::PlanningSteps => "planning_steps",
            Self::DiagnosticCount => "diagnostic_count",
            Self::MetadataBytes => "metadata_bytes",
            Self::WorkUnits => "work_units",
        }
    }
}

/// Optional operational budgets.
///
/// Every field defaults to `None`, meaning that this policy does not impose
/// a finite limit for that category. Applications processing untrusted input
/// should explicitly configure suitable limits for their environment.
///
/// A value of `Some(0)` is valid and means that no amount of that operation
/// is permitted under the configured policy.
///
/// Budgets are checked by the operation that consumes the corresponding
/// resource. Merely storing a budget here does not enforce it automatically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OperationalBudgets {
    /// Maximum accepted source size, in bytes.
    pub max_source_bytes: Option<u64>,

    /// Maximum encoded artifact size, in bytes.
    pub max_encoded_bytes: Option<u64>,

    /// Maximum decoded input size, in bytes.
    pub max_decoded_bytes: Option<u64>,

    /// Maximum number of graph nodes.
    pub max_graph_nodes: Option<u64>,

    /// Maximum number of graph edges.
    pub max_graph_edges: Option<u64>,

    /// Maximum number of domain instances.
    pub max_domain_instances: Option<u64>,

    /// Maximum nesting depth accepted by a configured operation.
    pub max_nesting_depth: Option<u64>,

    /// Maximum validation work units.
    pub max_validation_steps: Option<u64>,

    /// Maximum planning work units.
    pub max_planning_steps: Option<u64>,

    /// Maximum number of diagnostics retained by an operation.
    pub max_diagnostic_count: Option<u64>,

    /// Maximum bytes retained for metadata and diagnostic content.
    pub max_metadata_bytes: Option<u64>,

    /// Maximum total work units under the caller's accounting model.
    pub max_work_units: Option<u64>,
}

impl OperationalBudgets {
    /// Returns the configured limit for a budget category.
    #[must_use]
    pub const fn limit(&self, kind: BudgetKind) -> Option<u64> {
        match kind {
            BudgetKind::SourceBytes => self.max_source_bytes,
            BudgetKind::EncodedBytes => self.max_encoded_bytes,
            BudgetKind::DecodedBytes => self.max_decoded_bytes,
            BudgetKind::GraphNodes => self.max_graph_nodes,
            BudgetKind::GraphEdges => self.max_graph_edges,
            BudgetKind::DomainInstances => self.max_domain_instances,
            BudgetKind::NestingDepth => self.max_nesting_depth,
            BudgetKind::ValidationSteps => self.max_validation_steps,
            BudgetKind::PlanningSteps => self.max_planning_steps,
            BudgetKind::DiagnosticCount => self.max_diagnostic_count,
            BudgetKind::MetadataBytes => self.max_metadata_bytes,
            BudgetKind::WorkUnits => self.max_work_units,
        }
    }

    /// Returns whether every budget is unspecified.
    ///
    /// This is informational; it does not indicate whether the host has
    /// sufficient physical resources for a particular computation.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.max_source_bytes.is_none()
            && self.max_encoded_bytes.is_none()
            && self.max_decoded_bytes.is_none()
            && self.max_graph_nodes.is_none()
            && self.max_graph_edges.is_none()
            && self.max_domain_instances.is_none()
            && self.max_nesting_depth.is_none()
            && self.max_validation_steps.is_none()
            && self.max_planning_steps.is_none()
            && self.max_diagnostic_count.is_none()
            && self.max_metadata_bytes.is_none()
            && self.max_work_units.is_none()
    }
}

/// Validation policy for hybrid structures.
///
/// Required semantic checks must not be disabled through this configuration.
/// This policy controls when structural validation occurs and how explicitly
/// represented extension information is handled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidationPolicy {
    /// Validate graph structure during construction when the builder supports
    /// validation at that stage.
    pub validate_graph_on_build: bool,

    /// Require referenced domain contracts to be resolved before a structure
    /// is accepted as fully validated.
    pub require_resolved_domain_contracts: bool,

    /// Permit explicitly optional, namespaced extension metadata to survive
    /// validation when its semantics are not required for execution.
    ///
    /// This does not permit ignoring an unknown required operation, capability,
    /// constraint, or semantic guarantee.
    pub preserve_unknown_optional_extensions: bool,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            validate_graph_on_build: true,
            require_resolved_domain_contracts: true,
            preserve_unknown_optional_extensions: true,
        }
    }
}

/// Determinism policy for canonical hybrid representations.
///
/// These settings concern reproducibility of representations and diagnostics.
/// They do not claim that all hardware executions are physically deterministic:
/// quantum outcomes, distributed timing, and explicitly nondeterministic
/// language operations retain their specified semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeterminismPolicy {
    /// Require stable ordering wherever the hybrid API promises canonical
    /// ordering of graph entities, dependencies, or metadata.
    pub require_stable_ordering: bool,

    /// Require reproducible diagnostic ordering for equivalent inputs and
    /// the same explicit configuration.
    pub require_reproducible_diagnostics: bool,

    /// Require canonical encoding when the selected interchange format and
    /// serializer support a canonical representation.
    pub require_canonical_serialization: bool,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self {
            require_stable_ordering: true,
            require_reproducible_diagnostics: true,
            require_canonical_serialization: true,
        }
    }
}

/// Handling policy for optional fields unknown to the current implementation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum UnknownOptionalFieldPolicy {
    /// Preserve unknown optional fields when the interchange representation
    /// and implementation support lossless preservation.
    #[default]
    Preserve,

    /// Ignore unknown optional fields that do not affect required semantics.
    Ignore,
}

/// Serialization and interchange policy.
///
/// This configuration describes policy; the concrete encoder or decoder
/// remains responsible for implementing it and reporting errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SerializationPolicy {
    /// Policy for unknown optional fields.
    pub unknown_optional_fields: UnknownOptionalFieldPolicy,

    /// Reject unknown fields when the schema or producer marks them as
    /// required for interpretation or semantic preservation.
    pub reject_unknown_required_fields: bool,
}

impl Default for SerializationPolicy {
    fn default() -> Self {
        Self {
            unknown_optional_fields: UnknownOptionalFieldPolicy::Preserve,
            reject_unknown_required_fields: true,
        }
    }
}

/// Policy for reading artifacts produced by older schema versions.
///
/// Version identity and the actual compatibility decision belong to
/// `version.rs`; migration implementations belong to the compatibility and
/// interchange layers. This enum states the configured policy only.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default,
)]
#[serde(rename_all = "snake_case")]
pub enum OlderArtifactPolicy {
    /// Reject an older artifact unless a compatible migration is explicitly
    /// selected by the caller.
    Reject,

    /// Require an explicit migration before interpreting the artifact.
    #[default]
    RequireMigration,

    /// Permit reading without migration only when the version compatibility
    /// contract explicitly certifies that behavior as safe.
    AllowIfCompatible,
}

/// Compatibility policy for persisted hybrid artifacts.
///
/// This does not govern source-language versions, compiler package versions,
/// quantum IR versions, hardware firmware versions, or target capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtifactCompatibilityPolicy {
    /// Behavior when an artifact uses an older schema or format version.
    pub older_artifacts: OlderArtifactPolicy,
}

impl Default for ArtifactCompatibilityPolicy {
    fn default() -> Self {
        Self {
            older_artifacts: OlderArtifactPolicy::RequireMigration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_has_no_artificial_operational_limits() {
        let config = HybridConfig::default();

        assert!(config.budgets.is_unbounded());

        for kind in [
            BudgetKind::SourceBytes,
            BudgetKind::EncodedBytes,
            BudgetKind::DecodedBytes,
            BudgetKind::GraphNodes,
            BudgetKind::GraphEdges,
            BudgetKind::DomainInstances,
            BudgetKind::NestingDepth,
            BudgetKind::ValidationSteps,
            BudgetKind::PlanningSteps,
            BudgetKind::DiagnosticCount,
            BudgetKind::MetadataBytes,
            BudgetKind::WorkUnits,
        ] {
            assert_eq!(config.budget_limit(kind), None);
            assert_eq!(config.check_budget(kind, u128::MAX), Ok(()));
        }
    }

    #[test]
    fn explicitly_configured_budget_accepts_values_at_the_limit() {
        let config = HybridConfig {
            budgets: OperationalBudgets {
                max_graph_nodes: Some(100),
                ..OperationalBudgets::default()
            },
            ..HybridConfig::default()
        };

        assert_eq!(config.check_budget(BudgetKind::GraphNodes, 0), Ok(()));
        assert_eq!(
            config.check_budget(BudgetKind::GraphNodes, 100),
            Ok(())
        );
    }

    #[test]
    fn exceeding_a_configured_budget_returns_the_canonical_error() {
        let config = HybridConfig {
            budgets: OperationalBudgets {
                max_graph_nodes: Some(100),
                ..OperationalBudgets::default()
            },
            ..HybridConfig::default()
        };

        let error = config
            .check_budget(BudgetKind::GraphNodes, 101)
            .expect_err("the configured budget must be enforced");

        assert_eq!(error.code_str(), "HYB024");
        assert!(error.message().contains("graph_nodes"));
        assert!(error.message().contains("101"));
        assert!(error.message().contains("100"));
    }

    #[test]
    fn zero_is_a_valid_explicit_budget() {
        let config = HybridConfig {
            budgets: OperationalBudgets {
                max_planning_steps: Some(0),
                ..OperationalBudgets::default()
            },
            ..HybridConfig::default()
        };

        assert_eq!(
            config.check_budget(BudgetKind::PlanningSteps, 0),
            Ok(())
        );

        let error = config
            .check_budget(BudgetKind::PlanningSteps, 1)
            .expect_err("zero budget must reject positive work");

        assert_eq!(error.code_str(), "HYB024");
    }

    #[test]
    fn budget_categories_have_stable_identifiers() {
        assert_eq!(BudgetKind::GraphNodes.as_str(), "graph_nodes");
        assert_eq!(BudgetKind::DecodedBytes.as_str(), "decoded_bytes");
        assert_eq!(BudgetKind::WorkUnits.as_str(), "work_units");
    }

    #[test]
    fn strict_semantic_validation_is_enabled_by_default() {
        let config = HybridConfig::default();

        assert!(config.validation.validate_graph_on_build);
        assert!(config.validation.require_resolved_domain_contracts);
        assert!(config.determinism.require_stable_ordering);
        assert!(config.determinism.require_reproducible_diagnostics);
        assert!(config.determinism.require_canonical_serialization);
        assert!(config.serialization.reject_unknown_required_fields);
        assert_eq!(
            config.compatibility.older_artifacts,
            OlderArtifactPolicy::RequireMigration
        );
    }

    #[test]
    fn configuration_round_trips_through_serde_json() {
        let config = HybridConfig {
            budgets: OperationalBudgets {
                max_graph_nodes: Some(500),
                max_decoded_bytes: Some(1_000_000),
                ..OperationalBudgets::default()
            },
            compatibility: ArtifactCompatibilityPolicy {
                older_artifacts: OlderArtifactPolicy::AllowIfCompatible,
            },
            ..HybridConfig::default()
        };

        let encoded =
            serde_json::to_string(&config).expect("configuration serializes");
        let decoded: HybridConfig =
            serde_json::from_str(&encoded).expect("configuration deserializes");

        assert_eq!(decoded, config);
    }

    #[test]
    fn omitted_fields_receive_defaults() {
        let config: HybridConfig =
            serde_json::from_str("{}").expect("empty config is valid");

        assert_eq!(config, HybridConfig::default());
        assert!(config.budgets.is_unbounded());
    }

    #[test]
    fn unknown_configuration_fields_do_not_break_forward_compatible_reading() {
        let config: HybridConfig = serde_json::from_str(
            r#"{"future_policy":{"enabled":true},"budgets":{"max_graph_nodes":12}}"#,
        )
        .expect("unknown fields should not prevent reading known fields");

        assert_eq!(config.budgets.max_graph_nodes, Some(12));
    }
}
