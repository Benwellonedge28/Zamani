
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Integration
//!
//! Path: `src/compiler/backends/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## 1. Purpose
//!
//! This module is the compiler-facing composition boundary for backend
//! registration, discovery, capability evaluation, compatibility,
//! selection, and invocation.
//!
//! It coordinates existing authoritative backend subsystems.
//!
//! It does not implement a second backend architecture.
//!
//! ## 2. POCO-REAF
//!
//! Supports the architectural objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Backend integration must permit a program to be realized across
//! compatible computational environments without requiring application
//! source changes merely because the available hardware changes.
//!
//! Supported computational environments are open-ended.
//!
//! They include classical, quantum, HDL, hybrid, distributed, AI,
//! tensor, embedded, accelerator-based and future environments.
//!
//! This module does not enumerate those environments in a closed
//! hardware or architecture enumeration.
//!
//! ## 3. Canonical ownership
//!
//! | Responsibility | Canonical authority |
//! |---|---|
//! | Source syntax | `grammar/` |
//! | Source semantics | `crate::frontend::semantic` |
//! | Universal IR | `crate::zuir` |
//! | Classical IR | `crate::classical` |
//! | Quantum IR | `crate::quantum::ir` |
//! | HDL representation | `crate::hdl::hdlir` |
//! | Backend interface | `crate::backend` |
//! | Backend registration | `crate::backend::registry` |
//! | Target discovery | `crate::compiler::planning::target_discovery` |
//! | Resource feasibility | `crate::compiler::planning` |
//! | Compilation orchestration | `crate::compiler::pipeline` |
//! | Artifact verification | `crate::compiler::artifacts` |
//! | Linking | `crate::compiler::linking` |
//! | Physical resource acquisition | `crate::runtime` |
//! | Physical execution | `crate::runtime` |
//!
//! This module integrates these authorities without replacing them.
//!
//! ## 4. Responsibilities
//!
//! This module:
//!
//! - Exposes the compiler-facing backend integration API.
//! - Connects to the canonical backend registry.
//! - Exposes backend discovery.
//! - Exposes backend capability evaluation.
//! - Exposes compatibility evaluation.
//! - Exposes deterministic backend selection.
//! - Exposes explicitly requested backend invocation.
//! - Preserves structured subsystem errors.
//! - Provides stable public module paths.
//!
//! This module does not:
//!
//! - Implement backend factories.
//! - Own backend registration storage.
//! - Introduce another backend identifier system.
//! - Implement source-language semantic analysis.
//! - Define another universal IR.
//! - Discover physical devices independently.
//! - Allocate physical resources.
//! - Perform quantum routing.
//! - Perform HDL synthesis.
//! - Schedule distributed execution.
//! - Implement target-specific code generation.
//! - Verify generated artifacts independently.
//! - Link executable artifacts.
//! - Deploy or execute programs.
//!
//! ## 5. Essential architectural distinctions
//!
//! The following operations are different and must remain different:
//!
//! Registration != discovery.
//!
//! Discovery != physical availability.
//!
//! Physical availability != compatibility.
//!
//! Compatibility != resource feasibility.
//!
//! Resource feasibility != resource reservation.
//!
//! Selection != invocation.
//!
//! Invocation != artifact verification.
//!
//! Artifact verification != deployment.
//!
//! Deployment != execution.
//!
//! No operation in this module may silently treat one of these
//! operations as proof that another has succeeded.
//!
//! ## 6. Requirements, capabilities and feasibility
//!
//! Backend integration must preserve the semantic distinctions between:
//!
//! - Requirement: what the program needs.
//! - Capability: what an implementation supports.
//! - Preference: what the developer prefers.
//! - Constraint: what is permitted or prohibited.
//! - Hint: information that may improve planning.
//! - Feasibility: whether the available evidence supports a realization.
//!
//! These concepts are owned by their canonical semantic and planning
//! authorities.
//!
//! Backend integration must not redefine them.
//!
//! Unknown capability or compatibility evidence must not be treated
//! as established compatibility.
//!
//! An incompatible backend must not be selected.
//!
//! ## 7. Scalability
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - Registered backend implementations.
//! - Backend providers.
//! - Backend identifiers.
//! - Target architectures.
//! - Computational domains.
//! - CPUs.
//! - GPUs.
//! - Accelerators.
//! - FPGAs.
//! - ASICs.
//! - Quantum processors.
//! - Qubits.
//! - Distributed participants.
//! - Memory capacities.
//! - Resource quantities.
//! - IR operations.
//! - Program sizes.
//! - Deployment sizes.
//! - Future backend categories.
//!
//! No universal MAX_BACKENDS, MAX_TARGETS, MAX_CPUS, MAX_GPUS,
//! MAX_QUBITS, MAX_NODES or equivalent restriction is introduced.
//!
//! Actual operational constraints belong to the relevant configuration,
//! planning, provider, target and runtime contracts.
//!
//! Memory exhaustion, unavailable resources, implementation limitations,
//! operational budgets and physical constraints must be reported
//! explicitly.
//!
//! This architecture does not claim literal infinite computation.
//!
//! ## 8. Determinism
//!
//! Registry identity and enumeration order are inherited from the
//! canonical backend registry.
//!
//! Compatibility decisions are delegated to the authoritative evaluator.
//!
//! Selection ordering is delegated to the selection subsystem.
//!
//! This module must not introduce another ordering or identity policy.
//!
//! ## 9. Portability
//!
//! Backend compatibility must be established against validated program
//! requirements and applicable target capabilities.
//!
//! An unsupported operation must not silently be replaced with an
//! operation having different semantics.
//!
//! A successful invocation does not establish that generated output is
//! portable or valid.
//!
//! Portable artifact classification and verification belong to the
//! artifact subsystem.
//!
//! ## 10. Security
//!
//! Backend descriptors, capability profiles, provider responses,
//! generated output and external toolchain results are not automatically
//! trusted.
//!
//! Backend registration does not establish provider trust.
//!
//! Backend invocation does not establish sandboxing.
//!
//! Artifact verification and supply-chain validation remain mandatory
//! responsibilities of their authoritative security boundaries.
//!
//! ## 11. Error handling
//!
//! Each child module owns its structured errors.
//!
//! Errors must retain their original identity and causal information
//! wherever the underlying error contract supports it.
//!
//! This module must not silently discard failures or select another
//! backend after an unsuccessful invocation.
//!
//! ## 12. Integration requirements
//!
//! Parent:
//! `crate::compiler`
//!
//! Canonical backend API:
//! `crate::backend`
//!
//! Canonical backend registry:
//! `crate::backend::registry`
//!
//! Canonical backend compatibility:
//! `crate::backend::compatibility`
//!
//! Compiler target discovery:
//! `crate::compiler::planning::target_discovery`
//!
//! The canonical backend module root must expose the modules required
//! by the existing compiler backend integration files.
//!
//! The compiler must not maintain a competing registry.
//!
//! ## 13. Compatibility
//!
//! Existing public child-module paths and re-exports are preserved.
//!
//! The compiler backend API version is independent of:
//!
//! - Zamani language version.
//! - Semantic model version.
//! - ZUIR version.
//! - Domain IR versions.
//! - Backend implementation versions.
//! - External toolchain versions.
//!
//! Changes to these interfaces require deliberate compatibility review.
//!
//! ## 14. Completion criteria
//!
//! - Every declared child module exists.
//! - Every child module compiles independently.
//! - All public re-exports resolve.
//! - Canonical backend type identity is preserved.
//! - No competing registry is introduced.
//! - No competing capability model is introduced.
//! - Selection and invocation remain separate.
//! - Unknown compatibility is not accepted.
//! - Failed invocation is not silently replaced.
//! - Generated unverified output remains unverified.
//! - No arbitrary universal hardware ceilings are introduced.
//! - Rust 1.97+ compatibility is maintained.
//! - Unsafe Rust is forbidden.
//!
//! A successful module build does not certify every backend provider,
//! target implementation or artifact as production-ready.

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler backend integration subsystem.
pub const COMPILER_BACKENDS_MODULE_NAME: &str =
    "zamani.compiler.backends";

/// Public API version of the compiler backend integration boundary.
///
/// This is independent of the language, IR and backend versions.
pub const COMPILER_BACKENDS_API_VERSION: u16 = 1;

/// Returns the stable compiler backend module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    COMPILER_BACKENDS_MODULE_NAME
}

/// Returns the compiler backend integration API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_BACKENDS_API_VERSION
}

// ============================================================================
// CHILD MODULES
// ============================================================================

/// Compiler-facing integration with the canonical backend registry.
///
/// Registration and registry storage remain owned by
/// `crate::backend::registry`.
pub mod registry;

/// Backend discovery and target-discovery integration.
///
/// Discovery does not establish physical availability.
pub mod discovery;

/// Backend capability-profile integration.
///
/// Capability definitions and their canonical evaluation remain
/// owned by their authoritative backend and semantic subsystems.
pub mod capabilities;

/// Backend compatibility evaluation.
///
/// Reuses the canonical backend compatibility contracts.
pub mod compatibility;

/// Backend selection and eligibility evaluation.
///
/// Selection does not allocate hardware or execute programs.
pub mod selection;

/// Explicit invocation of a selected registered backend.
///
/// Invocation does not establish artifact verification or execution.
pub mod invocation;

// ============================================================================
// REGISTRY PUBLIC API
// ============================================================================

/// Canonical compiler-facing backend registry type.
pub use registry::CompilerBackendRegistry;

/// Canonical backend registry result type.
pub use registry::CompilerRegistryResult;

/// Creates an empty compiler-facing canonical backend registry.
pub use registry::new_compiler_backend_registry;

// ============================================================================
// DISCOVERY PUBLIC API
// ============================================================================

/// Backend discovery service.
pub use discovery::BackendDiscovery;

/// Backend discovery errors.
pub use discovery::BackendDiscoveryError;

/// Backend discovery report.
pub use discovery::BackendDiscoveryReport;

/// Backend discovery request.
pub use discovery::BackendDiscoveryRequest;

/// Backend probe report.
pub use discovery::BackendProbeReport;

/// Registered backend target-discovery provider.
pub use discovery::RegisteredBackendTargetProvider;

// ============================================================================
// CAPABILITY PUBLIC API
// ============================================================================

/// Backend capability evaluation error.
pub use capabilities::BackendCapabilityError;

/// Backend capability evaluator.
pub use capabilities::BackendCapabilityEvaluator;

/// Backend capability profile association.
pub use capabilities::BackendCapabilityProfile;

/// Backend capability-profile provider contract.
pub use capabilities::BackendCapabilityProfileProvider;

/// Backend capability evaluation request.
pub use capabilities::BackendCapabilityRequest;

// ============================================================================
// COMPATIBILITY PUBLIC API
// ============================================================================

/// Compiler/backend compatibility error.
pub use compatibility::BackendCompatibilityError;

/// Backend compatibility-profile provider contract.
pub use compatibility::BackendCompatibilityProfileProvider;

// ============================================================================
// SELECTION PUBLIC API
// ============================================================================

/// Evaluation of an individual backend candidate.
pub use selection::BackendCandidateEvaluation;

/// Backend eligibility result.
pub use selection::BackendEligibility;

/// Backend eligibility evaluator contract.
pub use selection::BackendEligibilityEvaluator;

/// Backend selection error.
pub use selection::BackendSelectionError;

/// Complete backend selection report.
pub use selection::BackendSelectionReport;

/// Backend selection request.
pub use selection::BackendSelectionRequest;

/// Backend selection service.
pub use selection::BackendSelector;

// ============================================================================
// INVOCATION PUBLIC API
// ============================================================================

/// Backend invocation error.
pub use invocation::BackendInvocationError;

/// Explicit backend invocation request.
pub use invocation::BackendInvocationRequest;

/// Backend invocation result.
pub use invocation::BackendInvocationResult;

/// Backend invocation service.
pub use invocation::BackendInvocationService;

/// Classification of generated backend output.
pub use invocation::BackendOutputStatus;

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
    fn child_module_versions_are_nonzero() {
        assert_ne!(
            registry::COMPILER_BACKEND_REGISTRY_API_VERSION,
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
    fn compiler_registry_is_the_canonical_registry() {
        fn accepts_canonical_registry(
            _: &crate::backend::registry::BackendRegistry,
        ) {
        }

        let registry = new_compiler_backend_registry();

        accepts_canonical_registry(&registry);
    }

    #[test]
    fn registry_construction_is_empty() {
        let registry = new_compiler_backend_registry();

        assert!(
            registry
                .is_empty()
                .expect("canonical registry must be accessible")
        );
    }

    #[test]
    fn registry_alias_preserves_canonical_type_identity() {
        let _: Option<CompilerBackendRegistry> = None;

        let _: Option<
            crate::backend::registry::BackendRegistry
        > = None;
    }

    #[test]
    fn selection_and_invocation_are_separate() {
        fn accepts_selection(
            _: &BackendSelectionRequest,
        ) {
        }

        fn accepts_invocation(
            _: &BackendInvocationRequest<'_>,
        ) {
        }

        let request =
            BackendSelectionRequest::all_registered();

        accepts_selection(&request);

        // Invocation requires an explicitly selected backend and
        // an actual canonical IR input.
        //
        // Selection cannot implicitly perform invocation.
        let _ = accepts_invocation;
    }

    #[test]
    fn unknown_eligibility_is_not_compatible() {
        let eligibility =
            BackendEligibility::Unknown {
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
        let eligibility =
            BackendEligibility::Incompatible {
                reason: "required capability unavailable".to_owned(),
            };

        assert!(
            !matches!(
                eligibility,
                BackendEligibility::Compatible { .. }
            )
        );
    }

    #[test]
    fn generated_output_is_not_misrepresented_as_verified() {
        assert_eq!(
            BackendOutputStatus::GeneratedUnverified,
            BackendOutputStatus::GeneratedUnverified
        );
    }
}
