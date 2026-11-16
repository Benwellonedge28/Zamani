
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Public API
//!
//! Path: `src/toolchain/api/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the canonical public API composition boundary for the
//! Zamani toolchain.
//!
//! It provides a stable interface for:
//!
//! - Human developers.
//! - Command-line applications.
//! - IDEs and language-server integrations.
//! - Build automation.
//! - Continuous integration systems.
//! - AI agents.
//! - Enterprise integrations.
//! - Third-party developer tools.
//! - Local and remote toolchain clients.
//!
//! Every client uses the same request, response, capability,
//! compatibility and version-negotiation contracts.
//!
//! ## Architecture
//!
//! The public API is divided into five independently maintained contracts:
//!
//! - `request`: caller intent and validated operation requests.
//! - `response`: structured operation outcomes and artifact references.
//! - `capabilities`: advertised toolchain API functionality.
//! - `compatibility`: cross-contract compatibility decisions.
//! - `versioning`: API version negotiation and dispatch protection.
//!
//! This module composes and exposes those contracts.
//! It does not implement their internal behavior.
//!
//! ## Ownership boundaries
//!
//! This module does NOT own:
//!
//! - Source grammar, tokens, AST or semantic analysis.
//! - Language-level requirements, capabilities, preferences,
//!   constraints or hints.
//! - Classical, quantum, HDL, AI or other domain IRs.
//! - Compiler passes or compilation.
//! - Target registration or hardware discovery.
//! - Physical resource allocation.
//! - Backend code generation.
//! - External process execution.
//! - Dependency resolution.
//! - Package archives or registry implementation.
//! - Runtime scheduling or application execution.
//! - Deployment execution.
//! - Authentication or credential storage.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! The API coordinates requests to those subsystems through the
//! corresponding toolchain services.
//!
//! ## Repository integration
//!
//! The API consumes the following existing authorities:
//!
//! - `crate::toolchain::ids`
//! - `crate::toolchain::version`
//! - `crate::toolchain::errors`
//! - `crate::toolchain::result`
//! - `crate::zamani_project_config`
//! - `crate::frontend`
//! - `crate::compiler`
//! - `crate::zuir`
//! - `crate::backend`
//! - `crate::zhal`
//! - `crate::zur`
//! - `crate::distributed`
//!
//! These are integration boundaries, not competing implementations.
//!
//! ## Grammar and semantic integration
//!
//! The API must remain consistent with:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//!
//! The grammar defines valid source syntax.
//!
//! The semantic model defines program meaning.
//!
//! The compiler determines valid compilation and realization strategies.
//!
//! Backend, hardware and runtime authorities determine whether a
//! requested realization can actually be produced and executed.
//!
//! The public API must not collapse these distinct responsibilities.
//!
//! ## POCO-REAF
//!
//! One logical project and request may produce multiple compatible
//! target-specific realizations.
//!
//! The API must preserve:
//!
//! - Project identity.
//! - Request identity.
//! - Source and dependency identity.
//! - API contract version.
//! - Target-selection intent.
//! - Operational budgets.
//! - Artifact identities.
//! - Compatibility decisions.
//! - Structured diagnostics.
//! - Execution and operation outcomes.
//!
//! A requested target is not proof of target feasibility.
//!
//! A successful request submission is not proof of successful compilation.
//!
//! A produced artifact is not automatically verified, portable,
//! executable or deployable.
//!
//! These facts must be established by the appropriate authorities.
//!
//! Literal infinite computation, storage or physical capacity is not
//! promised. Scaling is governed by semantic requirements, implementation
//! capabilities, representation constraints, policies and available
//! resources.
//!
//! ## Extensibility
//!
//! Public identifiers and capability catalogues are open-ended where
//! their underlying contracts support extension.
//!
//! Third-party integrations must not require modifications to this
//! composition module merely to introduce a new provider, computing
//! domain, target family, API operation or feature.
//!
//! New first-party API contract modules must be explicitly registered
//! here and documented as part of the API compatibility policy.
//!
//! ## Versioning
//!
//! `version.rs` is the authoritative owner of toolchain API versions.
//!
//! `versioning.rs` negotiates those versions.
//!
//! `compatibility.rs` evaluates compatibility across independently
//! versioned contracts.
//!
//! This module must not introduce another API version constant,
//! manifest version, package version or provider version authority.
//!
//! ## Security
//!
//! All externally received requests must pass structural validation
//! before dispatch.
//!
//! API version compatibility must be checked before dispatch.
//!
//! Authorization must be enforced by the owning service.
//!
//! Capability advertisements and extension metadata are untrusted
//! descriptive data, not executable instructions.
//!
//! Credentials must not be embedded in request or response metadata.
//!
//! Unknown or incompatible contracts must not silently become trusted.
//!
//! ## Concurrency
//!
//! This module contains no global mutable state, singleton service,
//! hidden worker pool or implicit execution environment.
//!
//! API implementations may be shared between concurrent clients,
//! subject to the thread-safety contracts of their owning services.
//!
//! Request isolation, cancellation, idempotency and transaction
//! guarantees belong to the operation services.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum on:
//!
//! - API clients.
//! - Projects or workspaces.
//! - Requests.
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - Artifacts.
//! - Dependencies.
//! - Distributed participants.
//! - Hardware resources.
//!
//! Collection and transport budgets, where needed, must be explicit,
//! configurable operational policies.
//!
//! Finite process memory, address space and transport capacity remain
//! physical implementation constraints.

// ============================================================================
// API CONTRACT MODULES
// ============================================================================

/// Request contracts.
///
/// Defines caller identity, request context, project scope, target intent,
/// operational budgets, operation types, request envelopes and structural
/// request validation.
///
/// This module does not execute operations.
pub mod request;

/// Response contracts.
///
/// Defines operation lifecycle states, structured errors, diagnostics,
/// artifact references, dependency changes, execution summaries and
/// response envelopes.
///
/// This module does not manufacture or verify compiler artifacts.
pub mod response;

/// API capability discovery.
///
/// Defines open-ended operation and feature identifiers, advertisement
/// statuses, capability descriptors and capability catalogues.
///
/// API capabilities must not be confused with language-semantic or
/// physical hardware capabilities.
pub mod capabilities;

/// Cross-contract compatibility.
///
/// Coordinates compatibility decisions for independently versioned
/// toolchain contracts without taking ownership of their version types
/// or performing migrations.
pub mod compatibility;

/// API version negotiation.
///
/// Defines client/server API version offers, negotiation results and
/// compatibility enforcement before request dispatch.
pub mod versioning;

// ============================================================================
// REQUEST CONTRACT RE-EXPORTS
// ============================================================================

/// Request validation error.
pub use request::RequestValidationError;

/// Request validation result.
pub use request::RequestValidationResult;

/// Calling-client identity descriptor.
pub use request::ClientIdentity;

/// Shared request metadata and correlation context.
pub use request::RequestContext;

/// Project location reference.
pub use request::ProjectLocator;

/// Project and workspace scope.
pub use request::ProjectScope;

/// Open-ended target-selection identifier.
pub use request::TargetSelector;

/// Target-selection intent.
pub use request::TargetSelection;

/// Operational budget identifier.
pub use request::BudgetKey;

/// Configurable operational budgets.
pub use request::OperationalBudgets;

/// Project build request.
pub use request::BuildRequest;

/// Workspace discovery request.
pub use request::WorkspaceDiscoveryRequest;

/// Project validation request.
pub use request::CheckRequest;

/// Application execution request.
pub use request::RunRequest;

/// Project testing request.
pub use request::TestRequest;

/// Package creation request.
pub use request::PackageRequest;

/// Package source reference.
pub use request::PackageSource;

/// Package installation request.
pub use request::InstallRequest;

/// Package publication request.
pub use request::PublishRequest;

/// Deployment request.
pub use request::DeployRequest;

/// Toolchain operation descriptor.
pub use request::ToolchainOperation;

/// Canonical request envelope.
pub use request::RequestEnvelope;

// ============================================================================
// RESPONSE CONTRACT RE-EXPORTS
// ============================================================================

/// Response structural validation error.
pub use response::ResponseValidationError;

/// Response validation result.
pub use response::ResponseValidationResult;

/// Operation lifecycle status.
pub use response::ResponseStatus;

/// Structured toolchain response error.
pub use response::ResponseError;

/// Diagnostic source-location information.
pub use response::DiagnosticLocation;

/// Structured response diagnostic.
pub use response::ResponseDiagnostic;

/// Artifact payload classification.
pub use response::ArtifactPayloadKind;

/// Artifact digest verification state.
pub use response::ArtifactDigestState;

/// Reference to a canonical compiler artifact.
pub use response::ArtifactReference;

/// Dependency change classification.
pub use response::DependencyChangeKind;

/// Dependency change information.
pub use response::DependencyChange;

/// Execution lifecycle state.
pub use response::ExecutionState;

/// Serialized elapsed-time representation.
pub use response::ElapsedTime;

/// Execution summary.
pub use response::ExecutionSummary;

/// Structured response data.
pub use response::ResponseData;

/// Canonical response envelope.
pub use response::ResponseEnvelope;

// ============================================================================
// CAPABILITY CONTRACT RE-EXPORTS
// ============================================================================

/// Capability catalogue validation error.
pub use capabilities::CapabilityValidationError;

/// Capability catalogue validation result.
pub use capabilities::CapabilityValidationResult;

/// API capability advertisement status.
pub use capabilities::ApiCapabilityStatus;

/// API operation descriptor.
pub use capabilities::ApiOperationDescriptor;

/// API feature descriptor.
pub use capabilities::ApiFeatureDescriptor;

/// Complete toolchain API capability catalogue.
pub use capabilities::ToolchainApiCapabilities;

// ============================================================================
// COMPATIBILITY CONTRACT RE-EXPORTS
// ============================================================================

/// Kind of independently versioned contract.
pub use compatibility::ContractKind;

/// Version value associated with a contract.
pub use compatibility::ContractVersion;

/// Contract compatibility classification.
pub use compatibility::CompatibilityStatus;

/// Reason for a compatibility decision.
pub use compatibility::CompatibilityReason;

/// Contract identity and version information.
pub use compatibility::CompatibilityContract;

/// Supported contract-version range.
pub use compatibility::SupportedVersionRange;

/// Individual contract compatibility result.
pub use compatibility::CompatibilityCheck;

/// Aggregated compatibility report.
pub use compatibility::CompatibilityReport;

/// Configured compatibility policies.
pub use compatibility::CompatibilityPolicies;

/// Compatibility evaluation service.
pub use compatibility::CompatibilityChecker;

// ============================================================================
// API VERSIONING CONTRACT RE-EXPORTS
// ============================================================================

/// API versioning validation error.
pub use versioning::ApiVersioningError;

/// API versioning validation result.
pub use versioning::ApiVersioningResult;

/// Client API version offer.
pub use versioning::ApiVersionOffer;

/// API version compatibility check.
pub use versioning::ApiVersionCheck;

/// API negotiation outcome classification.
pub use versioning::ApiNegotiationStatus;

/// Client/server API negotiation result.
pub use versioning::ApiVersionNegotiation;

/// API version negotiation service.
pub use versioning::ApiVersionNegotiator;

// ============================================================================
// PUBLIC API CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that the public request facade exposes the canonical
    /// request-envelope type.
    #[test]
    fn request_envelope_is_exposed() {
        fn accepts_request(_: &RequestEnvelope) {}

        let _ = accepts_request;
    }

    /// Verify that the public response facade exposes the canonical
    /// response-envelope type.
    #[test]
    fn response_envelope_is_exposed() {
        fn accepts_response(_: &ResponseEnvelope) {}

        let _ = accepts_response;
    }

    /// Verify that capability discovery is exposed through the
    /// public API facade.
    #[test]
    fn capability_catalogue_is_exposed() {
        fn accepts_capabilities(_: &ToolchainApiCapabilities) {}

        let _ = accepts_capabilities;
    }

    /// Verify that compatibility evaluation is exposed through
    /// the public API facade.
    #[test]
    fn compatibility_checker_is_exposed() {
        fn accepts_checker(_: &CompatibilityChecker) {}

        let _ = accepts_checker;
    }

    /// Verify that API version negotiation is exposed through
    /// the public API facade.
    #[test]
    fn version_negotiator_is_exposed() {
        fn accepts_negotiator(_: &ApiVersionNegotiator) {}

        let _ = accepts_negotiator;
    }

    /// Verify that request and response lifecycle classifications
    /// remain distinct.
    #[test]
    fn request_and_response_contracts_are_distinct() {
        assert_eq!(
            ResponseStatus::Succeeded.is_success(),
            true
        );

        assert_eq!(
            ResponseStatus::Running.is_terminal(),
            false
        );
    }
}
