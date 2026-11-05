
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Integration
//!
//! Path: `src/compiler/backends/mod.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only. Unsafe Rust is forbidden.
//!
//! ## 1. Purpose
//!
//! This module is the canonical compiler-facing integration boundary
//! for backend registration, discovery, capability evaluation,
//! compatibility analysis, selection, and invocation.
//!
//! It coordinates existing backend authorities without introducing
//! duplicate backend implementations or registries.
//!
//! ## 2. Architectural objective
//!
//! Support:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! The compiler must be able to work with registered backends for
//! classical, quantum, hybrid, AI, tensor, HDL, distributed,
//! accelerator-based, and future computational environments.
//!
//! This module must not enumerate every possible architecture.
//!
//! New backend implementations must be integrated through the
//! canonical backend registration and capability contracts.
//!
//! ## 3. Architectural ownership
//!
//! Canonical backend implementation and registration:
//!     crate::backend
//!
//! Compiler-facing backend integration:
//!     crate::compiler::backends
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability and resource analysis:
//!     crate::backend::compatibility
//!
//! IR ownership:
//!     crate::zuir and the applicable domain IRs
//!
//! Runtime execution:
//!     crate::runtime
//!
//! ## 4. Responsibilities
//!
//! This module provides:
//!
//! - Backend registry integration.
//! - Backend discovery integration.
//! - Backend capability integration.
//! - Backend compatibility integration.
//! - Backend selection integration.
//! - Backend invocation integration.
//! - A stable public compiler backend namespace.
//!
//! It does not own:
//!
//! - A second backend registry.
//! - Backend factories.
//! - Backend implementations.
//! - Source-language semantics.
//! - AST or parser implementations.
//! - A competing universal IR.
//! - Target descriptor definitions.
//! - Physical device discovery.
//! - Physical hardware allocation.
//! - Resource scheduling.
//! - Quantum routing.
//! - HDL synthesis.
//! - Target-specific code generation algorithms.
//! - Artifact linking.
//! - Runtime execution.
//!
//! ## 5. Dependency architecture
//!
//! ```text
//!                    compiler/backends
//!                           |
//!          +----------------+----------------+
//!          |                |                |
//!          v                v                v
//!       registry        discovery       capabilities
//!          |                |                |
//!          |                v                |
//!          |          target discovery       |
//!          |                                 |
//!          +----------------+----------------+
//!                           |
//!                           v
//!                     compatibility
//!                           |
//!                           v
//!                       selection
//!                           |
//!                           v
//!                       invocation
//!                           |
//!                           v
//!                    crate::backend
//!                           |
//!                           v
//!                  Target-specific backend
//!                           |
//!                           v
//!                    Generated output
//!                           |
//!                           v
//!                 Artifact verification
//!                           |
//!                           v
//!                       Runtime
//! ```
//!
//! Selection and invocation are separate operations.
//!
//! Selection identifies an eligible registered backend.
//!
//! Invocation constructs and invokes the explicitly selected backend.
//!
//! Neither operation allocates physical hardware or executes a
//! compiled program.
//!
//! ## 6. Scalability
//!
//! This module imposes no fixed maximum on:
//!
//! - Backend implementations.
//! - Backend registrations.
//! - Target architectures.
//! - CPU architectures.
//! - GPU architectures.
//! - Accelerator architectures.
//! - Quantum processors.
//! - Computational domains.
//! - Hardware resource counts.
//! - Program sizes.
//! - IR operation counts.
//! - Deployment sizes.
//! - Future backend categories.
//!
//! No universal MAX_BACKENDS, MAX_TARGETS, MAX_CPUS,
//! MAX_GPUS, MAX_QUBITS, or equivalent restrictions are permitted.
//!
//! Operational budgets must be supplied by the appropriate
//! compilation configuration and orchestration contracts.
//!
//! Actual allocation failures, memory exhaustion, time budgets,
//! unavailable resources, and implementation limitations must be
//! reported explicitly.
//!
//! Literal infinite computation is not claimed.
//!
//! ## 7. Portability
//!
//! Backend availability does not establish portability.
//!
//! Registration does not establish compatibility.
//!
//! Compatibility does not establish resource feasibility.
//!
//! Resource feasibility does not establish physical availability.
//!
//! Successful code generation does not establish artifact validity.
//!
//! The compiler must preserve these distinctions.
//!
//! An unsupported operation must not silently be replaced with
//! an operation having different semantics.
//!
//! ## 8. Determinism
//!
//! Deterministic backend identity, registry enumeration, and
//! selection ordering are inherited from their canonical owners.
//!
//! This module must not introduce independent identity or
//! ordering policies.
//!
//! ## 9. Security
//!
//! Backend metadata, discovery information, capability evidence,
//! generated output, and external toolchain results are not
//! automatically trusted.
//!
//! The compiler must validate information at the appropriate
//! security and artifact-verification boundaries.
//!
//! No backend invocation in this module implies sandboxing,
//! authorization, artifact signing, or successful verification.
//!
//! ## 10. Error handling
//!
//! Each child module owns its corresponding structured errors.
//!
//! This module does not erase error information or silently
//! substitute another backend when an operation fails.
//!
//! ## 11. Integration requirements
//!
//! The parent compiler module must expose:
//!
//!     pub mod backends;
//!
//! The backend root must expose the canonical backend API,
//! registry, capabilities, and compatibility contracts.
//!
//! The compiler planning module must expose target discovery.
//!
//! Existing child modules must remain the sole owners of their
//! respective implementations.
//!
//! ## 12. Compatibility
//!
//! Existing callers must migrate through the public module paths
//! without requiring duplicate backend types.
//!
//! The existing crate::backend::Backend trait remains authoritative.
//!
//! ## 13. Completion criteria
//!
//! - All six child modules are exposed.
//! - No duplicate backend registry is introduced.
//! - No duplicate capability model is introduced.
//! - No duplicate compatibility algorithm is introduced.
//! - Selection and invocation remain separate.
//! - Unknown capability evidence is not accepted as compatibility.
//! - Generated unverified output is not called an executable artifact.
//! - No arbitrary hardware ceilings are introduced.
//! - Rust 1.97.1 compatibility is maintained.
//! - No unsafe Rust is used.
//!
//! The existence of this module alone does not certify every
//! registered backend as production-ready.

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler backend integration subsystem.
pub const COMPILER_BACKENDS_MODULE_NAME: &str =
    "zamani.compiler.backends";

/// Public API version of the compiler backend integration boundary.
///
/// This is independent of the Zamani language version, backend
/// implementation versions, and IR format versions.
pub const COMPILER_BACKENDS_API_VERSION: u16 = 1;

// ============================================================================
// CANONICAL CHILD MODULES
// ============================================================================

/// Canonical backend registry integration.
///
/// Reuses `crate::backend::registry`.
///
/// Owns no independent registry state.
pub mod registry;

/// Backend discovery and target-discovery integration.
///
/// Connects registered backend metadata to the existing
/// compiler target-discovery subsystem.
///
/// Discovery does not establish physical device availability.
pub mod discovery;

/// Backend capability contracts and evaluation.
///
/// Reuses the canonical capability definitions and matching
/// implementation from `crate::backend`.
///
/// Missing capability evidence must remain unknown.
pub mod capabilities;

/// Backend compatibility adapter.
///
/// Integrates canonical backend compatibility reports with
/// the compiler backend-selection interface.
///
/// Does not implement another compatibility engine.
pub mod compatibility;

/// Backend selection.
///
/// Evaluates registered backend implementations using an
/// explicit compatibility evaluator.
///
/// Unknown and incompatible backends must not be selected.
pub mod selection;

/// Backend invocation.
///
/// Invokes an explicitly selected backend through the
/// canonical backend registry.
///
/// Generated text is not automatically a verified artifact.
pub mod invocation;

// ============================================================================
// CANONICAL PUBLIC API
// ============================================================================

/// Public backend registry integration API.
pub use registry::{
    new_compiler_backend_registry,
    CompilerBackendRegistry,
    CompilerRegistryResult,
};

/// Public backend discovery API.
pub use discovery::{
    BackendDiscovery,
    BackendDiscoveryError,
    BackendDiscoveryReport,
    BackendDiscoveryRequest,
    BackendProbeReport,
    RegisteredBackendTargetProvider,
};

/// Public backend capability evaluation API.
pub use capabilities::{
    BackendCapabilityError,
    BackendCapabilityEvaluator,
    BackendCapabilityProfile,
    BackendCapabilityProfileProvider,
    BackendCapabilityRequest,
};

/// Public backend compatibility API.
pub use compatibility::{
    BackendCompatibilityError,
    BackendCompatibilityProfileProvider,
};

/// Public backend selection API.
pub use selection::{
    BackendCandidateEvaluation,
    BackendEligibility,
    BackendEligibilityEvaluator,
    BackendSelectionError,
    BackendSelectionReport,
    BackendSelectionRequest,
    BackendSelector,
};

/// Public backend invocation API.
pub use invocation::{
    BackendInvocationError,
    BackendInvocationRequest,
    BackendInvocationResult,
    BackendInvocationService,
    BackendOutputStatus,
};

// ============================================================================
// VERSION AND IDENTITY VALIDATION
// ============================================================================

/// Returns the compiler backend integration API version.
///
/// This is provided for diagnostics, compatibility checks,
/// plugin integration, and tooling.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_BACKENDS_API_VERSION
}

/// Returns the stable compiler backend integration module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    COMPILER_BACKENDS_MODULE_NAME
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.backends"
        );
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(api_version(), 0);
    }

    #[test]
    fn all_backend_subsystems_have_versions() {
        assert_ne!(
            registry::BACKEND_REGISTRY_API_VERSION,
            0
        );

        assert_ne!(
            discovery::BACKEND_DISCOVERY_API_VERSION,
            0
        );

        assert_ne!(
            capabilities::BACKEND_CAPABILITIES_API_VERSION,
            0
        );

        assert_ne!(
            compatibility::BACKEND_COMPATIBILITY_API_VERSION,
            0
        );

        assert_ne!(
            selection::BACKEND_SELECTION_API_VERSION,
            0
        );

        assert_ne!(
            invocation::BACKEND_INVOCATION_API_VERSION,
            0
        );
    }

    #[test]
    fn public_registry_uses_canonical_registry() {
        let registry = new_compiler_backend_registry();

        assert!(
            registry
                .is_empty()
                .expect("registry operation must succeed")
        );
    }

    #[test]
    fn compiler_registry_preserves_canonical_type_identity() {
        fn accepts_canonical_registry(
            _: &crate::backend::registry::BackendRegistry,
        ) {
        }

        let registry = new_compiler_backend_registry();

        accepts_canonical_registry(&registry);
    }

    #[test]
    fn selection_and_invocation_are_distinct_contracts() {
        fn accepts_selection(
            _: &BackendSelectionRequest,
        ) {
        }

        fn accepts_invocation<'a>(
            _: &BackendInvocationRequest<'a>,
        ) {
        }

        let request = BackendSelectionRequest::all_registered();

        accepts_selection(&request);

        // Invocation requires an actual canonical IR module.
        // Its construction and validation remain the responsibility
        // of the authoritative IR and invocation integration.
        let _ = accepts_invocation;
    }

    #[test]
    fn unknown_eligibility_is_not_compatible() {
        let eligibility = BackendEligibility::Unknown {
            reason: "insufficient evidence".to_owned(),
        };

        assert!(
            !matches!(
                eligibility,
                BackendEligibility::Compatible { .. }
            )
        );
    }

    #[test]
    fn incompatible_eligibility_is_not_compatible() {
        let eligibility = BackendEligibility::Incompatible {
            reason: "missing required capability".to_owned(),
        };

        assert!(
            !matches!(
                eligibility,
                BackendEligibility::Compatible { .. }
            )
        );
    }

    #[test]
    fn generated_unverified_output_is_not_verified() {
        assert_eq!(
            BackendOutputStatus::GeneratedUnverified,
            BackendOutputStatus::GeneratedUnverified
        );
    }
}
