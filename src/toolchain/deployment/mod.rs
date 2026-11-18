
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Subsystem
//!
//! Path: `src/toolchain/deployment/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! This module is the composition boundary for deployment orchestration
//! within the Zamani toolchain.
//!
//! It brings together the existing deployment request, profile,
//! validation, planning, provider-registry and reporting contracts.
//!
//! It does not implement another deployment engine.
//!
//! ## Canonical ownership
//!
//! This module composes the following existing authorities:
//!
//! - `request`: canonical public deployment request adapter.
//! - `profiles`: deployment-specific settings over the generic profile
//!   registry.
//! - `validation`: evidence-based deployment validation.
//! - `planning`: side-effect-free deployment planning.
//! - `providers`: extensible deployment-provider registration.
//! - `reporting`: structured deployment outcomes and evidence.
//!
//! The command-level plan, evidence, receipt, provider execution and
//! command-result contracts remain owned by:
//!
//!     crate::toolchain::commands::deploy
//!
//! The public request contract remains owned by:
//!
//!     crate::toolchain::api::request
//!
//! ## Integration boundaries
//!
//! This subsystem coordinates with:
//!
//! - `crate::toolchain::api`: public deployment requests.
//! - `crate::toolchain::configuration`: configuration and generic profiles.
//! - `crate::toolchain::environments`: environment discovery and validation.
//! - `crate::toolchain::commands::deploy`: deployment command lifecycle.
//! - `crate::toolchain::targets`: target profiles and compatibility.
//! - `crate::compiler`: compilation and realization contracts.
//! - `crate::backend`: target-specific artifacts and backend contracts.
//! - `crate::zhal`: physical hardware and resource authorities.
//! - `crate::zur`: runtime handoff and execution.
//! - `crate::distributed`: distributed-computation infrastructure.
//! - `crate::stdlib::feasibility`: canonical feasibility model.
//!
//! These integrations must use the owning subsystem's public contracts.
//! This module must not duplicate their implementations or data models.
//!
//! ## Deployment lifecycle
//!
//! The intended lifecycle is:
//!
//! ```text
//! DeployRequest
//!      |
//!      v
//! Request validation
//!      |
//!      v
//! Deployment profile resolution
//!      |
//!      v
//! Evidence-based validation
//!      |
//!      v
//! Deployment planning
//!      |
//!      v
//! DeploymentPlan
//!      |
//!      v
//! Explicit provider selection
//!      |
//!      v
//! Apply-time revalidation and authorization
//!      |
//!      v
//! DeploymentProvider
//!      |
//!      v
//! DeploymentReceipt
//!      |
//!      v
//! DeploymentCommandResult
//!      |
//!      v
//! DeploymentReport
//! ```
//!
//! Planning and reporting do not perform deployment side effects.
//! Physical resource acquisition, infrastructure changes and application
//! execution remain with their authorized providers and runtime services.
//!
//! ## Evidence and outcome invariants
//!
//! The deployment subsystem must preserve these distinctions:
//!
//! - Request validity is not feasibility.
//! - Provider registration is not provider trust.
//! - Provider trust is not authorization.
//! - Capability declaration is not resource availability.
//! - Feasibility is not resource reservation.
//! - Resource reservation is not deployment completion.
//! - Deployment acceptance is not successful completion.
//! - Partial completion is not complete success.
//! - Missing evidence is not positive evidence.
//! - Unknown must never be silently promoted to success.
//!
//! Planning evidence and application evidence remain distinguishable.
//! Target-specific evidence retains its target identity.
//! Provider and artifact provenance must not be fabricated.
//!
//! ## Semantic and grammar boundaries
//!
//! Source-level deployment intent is defined by:
//!
//! - `grammar/execution/deployment.g4`
//! - `grammar/compile/deployment.g4`
//! - `grammar/policies/deployment.g4`
//! - `grammar/resources/requirements.g4`
//!
//! The normative portability and scalability contracts are defined by:
//!
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//!
//! This module consumes validated deployment intent. It does not parse
//! grammar productions or redefine semantic meaning.
//!
//! Requirements, capabilities, preferences, constraints, hints and
//! feasibility must remain separate concepts.
//!
//! In particular, a requested capability is not proof that a target,
//! provider or physical device actually supplies that capability.
//!
//! ## POCO-REAF and scalability
//!
//! One logical project and deployment request may have multiple
//! compatible target-specific realizations.
//!
//! The deployment architecture is open to classical, quantum, HDL,
//! AI/tensor, hybrid, distributed and future computing domains.
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - Deployment requests.
//! - Targets or target realizations.
//! - Providers.
//! - Artifacts.
//! - Computational domains.
//! - Hardware devices.
//! - CPUs or accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Deployment environments.
//! - Evidence records.
//!
//! Collection growth is subject to available memory, address space,
//! implementation capabilities and explicitly configured operational
//! budgets.
//!
//! Literal infinite computation, storage or physical capacity is not
//! promised.
//!
//! ## Security
//!
//! This composition module does not establish provider trust,
//! authentication, authorization or artifact integrity.
//!
//! Those checks must be performed by the canonical security,
//! credential, artifact and provider authorities.
//!
//! Secrets must not be embedded in deployment identifiers, evidence,
//! provider descriptors or serialized reports.
//!
//! Apply-time authorization and validation remain mandatory even when
//! a plan was previously validated.
//!
//! ## Compatibility
//!
//! Each child module owns its own versioned contract where required.
//! This module must not introduce a competing language, compiler,
//! provider, deployment protocol or report schema version.
//!
//! New deployment modules must be explicitly declared here and must
//! preserve the ownership boundaries documented above.

// ============================================================================
// DEPLOYMENT SUBMODULES
// ============================================================================

/// Side-effect-free deployment planning and readiness assessment.
///
/// Consumes validated requests and authoritative validation evidence.
/// Produces deployment plans without allocating physical resources or
/// starting applications.
pub mod planning;

/// Deployment profile settings and integration with the generic
/// toolchain profile registry.
///
/// Stores deployment intent and provider-scoped configuration without
/// claiming that a target or provider is feasible.
pub mod profiles;

/// Extensible deployment-provider identity and registry.
///
/// Registers and resolves implementations of the canonical
/// `commands::deploy::DeploymentProvider` contract.
pub mod providers;

/// Structured deployment reports, evidence snapshots and serialization.
///
/// Reports actual command outcomes without manufacturing evidence,
/// provenance or successful completion.
pub mod reporting;

/// Canonical deployment request adapter.
///
/// Reuses the public toolchain API request and its validation contract.
pub mod request;

/// Evidence-based deployment validation.
///
/// Coordinates authoritative validation providers and preserves
/// unknown, unsupported, infeasible and feasible outcomes.
pub mod validation;

// ============================================================================
// PUBLIC DEPLOYMENT API
// ============================================================================

// Request boundary

/// Canonical deployment request alias.
pub use request::DeploymentRequest;

/// Canonical target-selection intent.
pub use request::{TargetSelection, TargetSelector};

/// Canonical deployment request validation result.
pub use request::{
    DeploymentRequestValidationError,
    DeploymentRequestValidationResult,
};

// Profile boundary

/// Deployment-specific profile settings.
pub use profiles::DeploymentProfileSettings;

/// Deployment profile definition.
pub use profiles::DeploymentProfileDefinition;

/// Resolved deployment profile.
pub use profiles::ResolvedDeploymentProfile;

/// Adapter over the canonical generic profile registry.
pub use profiles::DeploymentProfileRegistry;

// Validation boundary

/// Deployment validation scope.
pub use validation::DeploymentValidationScope;

/// Deployment validation request.
pub use validation::DeploymentValidationRequest;

/// Deployment validation context.
pub use validation::DeploymentValidationContext;

/// Deployment validation provider contract.
pub use validation::DeploymentValidationProvider;

/// Structured deployment validation report.
pub use validation::DeploymentValidationReport;

/// Evidence-based deployment validator.
pub use validation::DeploymentValidator;

// Planning boundary

/// Deployment planning input.
pub use planning::DeploymentPlanningInput;

/// Deployment planner.
pub use planning::DeploymentPlanner;

/// Deployment readiness assessment.
pub use planning::DeploymentReadinessAssessment;

/// Deployment planning issue.
pub use planning::DeploymentPlanningIssue;

/// Deployment planning issue classification.
pub use planning::DeploymentPlanningIssueKind;

// Provider boundary

/// Deployment provider identity.
pub use providers::DeploymentProviderId;

/// Deployment provider descriptor.
pub use providers::DeploymentProviderDescriptor;

/// Registered deployment provider.
pub use providers::RegisteredDeploymentProvider;

/// Extensible deployment provider registry.
pub use providers::DeploymentProviderRegistry;

// Reporting boundary

/// Deployment report.
pub use reporting::DeploymentReport;

/// Deployment evidence record.
pub use reporting::DeploymentEvidenceRecord;

/// Deployment evidence phase.
pub use reporting::DeploymentEvidencePhase;

/// Deployment evidence counts.
pub use reporting::DeploymentEvidenceCounts;

// ============================================================================
// SUBSYSTEM CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the deployment composition exposes its child modules.
    #[test]
    fn deployment_submodules_are_accessible() {
        let _ = planning::DEPLOYMENT_PLANNING_MODULE_NAME;
        let _ = providers::DEPLOYMENT_PROVIDERS_MODULE_NAME;
        let _ = reporting::DEPLOYMENT_REPORTING_MODULE_NAME;
        let _ = profiles::DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION;
        let _ = validation::SCOPE_ARTIFACTS;
    }

    /// Verify that the deployment report schema has a stable version.
    #[test]
    fn deployment_report_schema_is_versioned() {
        assert!(reporting::DEPLOYMENT_REPORT_SCHEMA_VERSION > 0);
    }

    /// Verify that the deployment-provider API has a stable version.
    #[test]
    fn deployment_provider_api_is_versioned() {
        assert!(providers::DEPLOYMENT_PROVIDER_API_VERSION > 0);
    }

    /// Verify that the deployment-planning API has a stable version.
    #[test]
    fn deployment_planning_api_is_versioned() {
        assert!(planning::DEPLOYMENT_PLANNING_API_VERSION > 0);
    }

    /// Verify that the deployment profile schema has a stable version.
    #[test]
    fn deployment_profile_schema_is_versioned() {
        assert!(profiles::DEPLOYMENT_PROFILE_SETTINGS_SCHEMA_VERSION > 0);
    }

    /// Verify that the public deployment request is the canonical API type.
    #[test]
    fn deployment_request_reuses_the_public_api_contract() {
        fn accepts_request(_: &crate::toolchain::api::request::DeployRequest) {}

        let adapter: fn(&DeploymentRequest) = accepts_request;
        let _ = adapter;
    }
}
