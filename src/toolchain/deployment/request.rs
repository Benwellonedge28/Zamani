
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deployment Requests
//!
//! Path: src/toolchain/deployment/request.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: No unsafe Rust.
//!
//! ## Purpose
//!
//! Provides the deployment subsystem's canonical request boundary.
//!
//! The canonical request data structure remains owned by:
//!
//!     crate::toolchain::api::request::DeployRequest
//!
//! This module exposes that request under the deployment-domain name
//! `DeploymentRequest` without introducing a competing representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The deployment-domain request API alias.
//! - Deployment request validation entry points.
//! - Deployment request integration documentation.
//!
//! It does not own:
//!
//! - Source grammar or AST.
//! - Semantic deployment declarations.
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Compilation.
//! - Artifact creation or storage.
//! - Target registration.
//! - Capability discovery.
//! - Feasibility analysis.
//! - Hardware discovery.
//! - Resource allocation.
//! - Physical placement.
//! - Infrastructure provisioning.
//! - Distributed scheduling.
//! - Runtime execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! API request:
//!
//!     crate::toolchain::api::request::DeployRequest
//!
//! Deployment command:
//!
//!     crate::toolchain::commands::deploy
//!
//! Deployment profiles:
//!
//!     crate::toolchain::deployment::profiles
//!
//! Deployment validation:
//!
//!     crate::toolchain::deployment::validation
//!
//! Deployment planning:
//!
//!     crate::toolchain::deployment::planning
//!
//! Deployment providers:
//!
//!     crate::toolchain::deployment::providers
//!
//! Deployment reporting:
//!
//!     crate::toolchain::deployment::reporting
//!
//! Compiler:
//!
//!     crate::compiler
//!
//! Backend:
//!
//!     crate::backend
//!
//! Hardware:
//!
//!     crate::zhal
//!
//! Runtime:
//!
//!     crate::zur
//!
//! Distributed computation:
//!
//!     crate::distributed
//!
//! ## POCO-REAF
//!
//! A deployment request describes one logical deployment intent.
//!
//! Its target selection may describe one or multiple target realizations.
//!
//! A request does not guarantee that those realizations are feasible.
//!
//! The relevant compiler, backend, domain, hardware and runtime
//! authorities must establish actual compatibility and feasibility.
//!
//! Different realizations may produce different artifacts while
//! preserving the applicable program semantics.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary limits on:
//!
//! - Deployment targets.
//! - Computational domains.
//! - Hardware devices.
//! - CPUs or accelerators.
//! - Quantum resources.
//! - Distributed participants.
//! - Deployment environments.
//! - Projects.
//! - Artifacts.
//! - Providers.
//!
//! Actual capacity is determined by available resources, implementation
//! capabilities and explicitly configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Security
//!
//! A structurally valid request is not an authorization.
//!
//! Authentication, authorization, filesystem validation, provider
//! trust, artifact verification and deployment policy are enforced
//! by their respective authorities.
//!
//! Requests must not carry raw credentials.
//!
//! ## Failure semantics
//!
//! Invalid requests must be rejected.
//!
//! Unknown feasibility must remain unknown.
//!
//! Unsupported targets must not be silently substituted.
//!
//! A planning request must not cause deployment side effects.
//!
//! A successful request validation must never be interpreted as
//! proof of successful deployment.

// ============================================================================
// CANONICAL DEPLOYMENT REQUEST
// ============================================================================

/// Canonical deployment request.
///
/// This is an alias of the public toolchain API request rather than
/// a second request structure.
///
/// The canonical representation owns:
///
/// - Project and workspace scope.
/// - Target-selection intent.
/// - Optional environment identity.
/// - Optional deployment profile.
/// - Plan-only intent.
/// - Operational budgets.
///
/// Keeping one underlying type guarantees that the deployment
/// subsystem and public toolchain API cannot silently disagree
/// about the meaning or representation of a deployment request.
///
/// Every request must be structurally validated before dispatch.
///
/// Filesystem state, authentication, authorization, target feasibility,
/// provider readiness and resource availability are validated by
/// their respective authoritative services.
pub use crate::toolchain::api::request::DeployRequest as DeploymentRequest;

// ============================================================================
// CANONICAL TARGET-SELECTION CONTRACT
// ============================================================================

/// Canonical target-selection intent used by deployment requests.
///
/// Target selectors are opaque identifiers resolved through the
/// authoritative target registry.
///
/// They are not physical hardware identities.
///
/// Explicit targets, capability-based resolution and target
/// preferences retain the semantics defined by the public API.
pub use crate::toolchain::api::request::{
    TargetSelection,
    TargetSelector,
};

// ============================================================================
// REQUEST VALIDATION CONTRACT
// ============================================================================

/// Canonical structural validation error for deployment requests.
///
/// This is re-exported rather than duplicated so API clients and
/// deployment services use the same validation-error representation.
pub use crate::toolchain::api::request::{
    RequestValidationError as DeploymentRequestValidationError,
    RequestValidationResult as DeploymentRequestValidationResult,
};

/// Validate a deployment request before deployment orchestration.
///
/// This function deliberately delegates to the canonical request
/// validator.
///
/// It does not perform:
///
/// - Project discovery.
/// - Manifest loading.
/// - Authentication.
/// - Authorization.
/// - Compilation.
/// - Artifact verification.
/// - Target discovery.
/// - Hardware discovery.
/// - Feasibility analysis.
/// - Resource allocation.
/// - Infrastructure provisioning.
/// - Runtime execution.
///
/// Those operations must be performed by their owning services.
///
/// # Arguments
///
/// * `request` - The canonical deployment request.
///
/// # Returns
///
/// `Ok(())` when the request satisfies its structural contract.
///
/// Returns the canonical request-validation error otherwise.
///
/// # Important
///
/// Structural validity does not establish deployment feasibility.
pub fn validate_deployment_request(
    request: &DeploymentRequest,
) -> DeploymentRequestValidationResult<()> {
    request.validate()
}

// ============================================================================
// INTEGRATION CONTRACT
// ============================================================================

/// Identifies the canonical deployment request API version.
///
/// This delegates to the toolchain API versioning authority.
///
/// It does not introduce an independent deployment API version.
#[must_use]
pub const fn deployment_request_api_version() -> u32 {
    crate::toolchain::version::INITIAL_TOOLCHAIN_API_VERSION
        .get()
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::request::DeployRequest;

    #[test]
    fn deployment_request_is_the_canonical_api_request() {
        fn assert_same_type(_: &DeploymentRequest) {}

        let _ = assert_same_type as fn(&DeployRequest);
    }

    #[test]
    fn deployment_request_validator_is_available() {
        fn validator(
            request: &DeploymentRequest,
        ) -> DeploymentRequestValidationResult<()> {
            validate_deployment_request(request)
        }

        let _ = validator;
    }

    #[test]
    fn deployment_target_contract_is_shared_with_public_api() {
        fn accept_selection(_: TargetSelection) {}

        let _ = accept_selection as fn(TargetSelection);
    }

    #[test]
    fn deployment_selector_contract_is_shared_with_public_api() {
        fn accept_selector(_: TargetSelector) {}

        let _ = accept_selector as fn(TargetSelector);
    }

    #[test]
    fn deployment_validation_error_is_shared_with_public_api() {
        fn accept_error(
            _: DeploymentRequestValidationError,
        ) {
        }

        let _ = accept_error as fn(
            DeploymentRequestValidationError,
        );
    }
}
