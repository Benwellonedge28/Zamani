
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Canonical Backend Registry Integration
//!
//! Path: src/compiler/backends/registry.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides the compiler-facing interface to the authoritative
//! backend registry implemented by `crate::backend::registry`.
//!
//! This module is an integration facade.
//!
//! It MUST NOT implement a second backend registry, backend
//! factory system, backend identity system, or independent
//! registration lifecycle.
//!
//! # Architectural authorities
//!
//! Canonical backend implementation:
//!     crate::backend
//!
//! Canonical backend registration:
//!     crate::backend::registry
//!
//! Backend capability information:
//!     crate::backend::capabilities
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! Backend target information:
//!     crate::backend::target
//!
//! Compiler backend discovery:
//!     crate::compiler::backends::discovery
//!
//! Compiler backend selection:
//!     crate::compiler::backends::selection
//!
//! Compiler backend invocation:
//!     crate::compiler::backends::invocation
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Source semantics:
//!     crate::frontend::semantic
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Physical execution:
//!     crate::runtime
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Compiler-facing registry exports.
//! - Compiler-facing registry naming.
//! - Registry construction convenience.
//! - Compiler integration result aliases.
//! - Public registry integration documentation.
//!
//! It does NOT own:
//!
//! - Backend implementations.
//! - Backend registration storage.
//! - Backend factories.
//! - Backend identifiers.
//! - Alias resolution algorithms.
//! - Backend descriptor validation.
//! - Target descriptors.
//! - Capability definitions.
//! - Capability negotiation.
//! - Compatibility algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - IR construction.
//! - IR lowering.
//! - Code generation.
//! - Artifact verification.
//! - Linking.
//! - Runtime execution.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The registry supports an open-world backend ecosystem.
//!
//! New backend implementations must be registrable without modifying
//! this facade or adding new architecture-specific variants here.
//!
//! Supported backend categories are not a closed enumeration.
//!
//! # Scalability
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Registered backends.
//! - Backend providers.
//! - Target architectures.
//! - Computational domains.
//! - Accelerators.
//! - Hardware devices.
//! - CPU counts.
//! - GPU counts.
//! - FPGA counts.
//! - QPU counts.
//! - Distributed participants.
//! - Memory capacity.
//! - Quantum resources.
//! - Future computing technologies.
//!
//! Actual memory, address-space, processing-time and operational
//! constraints remain properties of the executing environment.
//!
//! Configurable operational budgets belong to compiler configuration.
//!
//! Resource feasibility belongs to compiler planning.
//!
//! Physical resource acquisition belongs to runtime providers.
//!
//! Literal infinite physical computation is not claimed.
//!
//! # Registration invariants
//!
//! The canonical backend registry is responsible for:
//!
//! - Canonical identifier uniqueness.
//! - Alias uniqueness.
//! - Duplicate registration rejection.
//! - Atomic registration.
//! - Deterministic enumeration.
//! - Canonical identifier resolution.
//! - Backend factory ownership.
//! - Backend construction.
//! - Factory identity validation.
//! - Registry synchronization.
//! - Registry lifecycle.
//!
//! This module must preserve those guarantees without duplicating them.
//!
//! # Integration invariants
//!
//! Registration does not imply compatibility.
//!
//! Compatibility does not imply resource feasibility.
//!
//! Resource feasibility does not imply physical availability.
//!
//! Physical availability does not imply successful code generation.
//!
//! Code generation does not imply artifact verification.
//!
//! Artifact verification does not imply successful runtime execution.
//!
//! No operation in this module silently changes the requested backend.
//!
//! # Thread safety
//!
//! Thread safety is inherited from the canonical BackendRegistry.
//!
//! Cloning the compiler-facing registry shares the canonical registry
//! state. It never creates a second independent registry.
//!
//! # Error handling
//!
//! Registry errors are propagated using the canonical RegistryError.
//!
//! This preserves the original error information and avoids
//! incompatible compiler-specific error translations.
//!
//! # Compatibility
//!
//! Public types are direct re-exports of the canonical backend
//! registry types.
//!
//! CompilerBackendRegistry is a type alias, not a new registry.
//!
//! # Completion criteria
//!
//! - Canonical registry types are re-exported.
//! - No independent mutable registry exists.
//! - No duplicate backend factory exists.
//! - No architecture-specific closed enumeration exists.
//! - Registry operations retain canonical error information.
//! - Registry clones share canonical state.
//! - The compiler-facing module path is stable.
//! - Rust 1.97+ is supported.
//! - No unsafe Rust is introduced.

use std::error::Error;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of this compiler registry integration module.
pub const COMPILER_BACKEND_REGISTRY_MODULE_NAME: &str =
    "zamani.compiler.backends.registry";

/// Version of the compiler-facing registry integration API.
///
/// Independent of the language, IR, backend implementation and
/// runtime versions.
pub const COMPILER_BACKEND_REGISTRY_API_VERSION: u16 = 1;

// ============================================================================
// CANONICAL REGISTRY EXPORTS
// ============================================================================

/// Canonical backend registry implementation.
///
/// This is the authoritative registry owned by `crate::backend`.
///
/// No additional registry state is introduced here.
pub use crate::backend::registry::BackendRegistry;

/// Canonical backend descriptor.
///
/// Contains backend identity and registration metadata.
pub use crate::backend::registry::BackendDescriptor;

/// Canonical backend identifier.
///
/// This type is owned by the backend subsystem.
pub use crate::backend::registry::BackendId;

/// Canonical backend factory contract.
///
/// Factories remain owned and managed by the canonical registry.
pub use crate::backend::registry::BackendFactory;

/// Canonical backend registration and lookup errors.
///
/// No compiler-specific error conversion is performed.
pub use crate::backend::registry::RegistryError;

// ============================================================================
// COMPILER-FACING ALIASES
// ============================================================================

/// Compiler-facing name for the canonical backend registry.
///
/// This is exactly the same type as `BackendRegistry`.
///
/// Clones share the same underlying canonical registry state.
///
/// No independent registration storage is created.
pub type CompilerBackendRegistry = BackendRegistry;

/// Result type for compiler-facing registry operations.
///
/// The original canonical RegistryError is preserved.
pub type CompilerRegistryResult<T> = Result<T, RegistryError>;

// ============================================================================
// REGISTRY CONSTRUCTION
// ============================================================================

/// Construct an empty compiler-facing canonical backend registry.
///
/// This delegates directly to the canonical backend registry.
///
/// It does not:
///
/// - Register built-in backends.
/// - Discover physical hardware.
/// - Inspect the host machine.
/// - Load external plugins.
/// - Invoke backend factories.
/// - Select a target.
/// - Allocate resources.
/// - Execute programs.
///
/// Backend registration is an explicit initialization responsibility.
///
/// # Examples
///
/// ```ignore
/// use zamani::compiler::backends::registry::new_compiler_backend_registry;
///
/// let registry = new_compiler_backend_registry();
///
/// assert!(registry.is_empty()?);
///
/// # Ok::<(), zamani::backend::registry::RegistryError>(())
/// ```
///
/// # Errors
///
/// Construction itself cannot fail. Subsequent registry operations
/// return their canonical RegistryError where appropriate.
#[must_use]
pub fn new_compiler_backend_registry() -> CompilerBackendRegistry {
    BackendRegistry::new()
}

// ============================================================================
// CANONICAL IDENTITY HELPERS
// ============================================================================

/// Resolve a canonical backend identifier or registered alias.
///
/// Resolution delegates to the authoritative registry.
///
/// The returned identifier is canonical.
///
/// No architecture inference or fallback is performed.
///
/// # Errors
///
/// Returns BackendNotFound when the identifier cannot be resolved,
/// or the original registry error when registry access fails.
pub fn resolve_backend_id(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendId> {
    registry.resolve(identifier)
}

/// Retrieve registered backend metadata.
///
/// This operation does not instantiate the backend.
///
/// It does not establish target compatibility or physical availability.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn registered_backend_descriptor(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendDescriptor> {
    registry.descriptor(identifier)
}

/// Determine whether a canonical backend identifier or alias
/// is registered.
///
/// This operation only inspects registry metadata.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn is_backend_registered(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<bool> {
    registry.contains(identifier)
}

// ============================================================================
// REGISTRY INSPECTION
// ============================================================================

/// Return registered backend descriptors in canonical identifier order.
///
/// Enumeration ordering is provided by the canonical registry.
///
/// This function does not instantiate backends or discover hardware.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn registered_backends(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<Vec<BackendDescriptor>> {
    registry.descriptors()
}

/// Return registered canonical backend identifiers.
///
/// The returned identifiers are independent of registration order.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn registered_backend_ids(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<Vec<BackendId>> {
    registry.backend_ids()
}

/// Return the number of registered canonical backends.
///
/// Aliases are not counted as separate backend implementations.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn registered_backend_count(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<usize> {
    registry.len()
}

/// Return whether the canonical registry contains no registered backends.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn backend_registry_is_empty(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<bool> {
    registry.is_empty()
}

// ============================================================================
// BACKEND CONSTRUCTION
// ============================================================================

/// Construct a fresh backend implementation through the canonical registry.
///
/// The requested identifier may be a canonical backend identifier
/// or a registered alias.
///
/// The canonical registry:
///
/// - Resolves the requested identifier.
/// - Retrieves its registered factory.
/// - Executes the factory outside its registry lock.
/// - Checks the constructed implementation's canonical identity.
///
/// This function does not perform target selection.
///
/// It must only be called after the compiler has established that
/// invoking the explicitly requested backend is appropriate.
///
/// # Errors
///
/// Returns the original canonical registry error, including:
///
/// - BackendNotFound.
/// - FactoryIdentityMismatch.
/// - LockPoisoned.
///
/// # Important
///
/// Successful construction does not establish:
///
/// - Physical hardware availability.
/// - Toolchain availability.
/// - Capability compatibility.
/// - Resource feasibility.
/// - Successful code generation.
/// - Artifact verification.
/// - Runtime execution.
pub fn create_registered_backend(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<Box<dyn crate::backend::Backend>> {
    registry.create(identifier)
}

// ============================================================================
// REGISTRY LIFECYCLE
// ============================================================================

/// Unregister a backend by canonical identifier or registered alias.
///
/// Unregistration delegates to the canonical registry.
///
/// The canonical registry removes the registration and its aliases.
///
/// Already constructed backend instances remain independently owned
/// by their callers.
///
/// Production initialization should normally establish the required
/// registrations before compilation requests begin.
///
/// Dynamic extension lifecycle management must additionally respect
/// provider isolation and security policies.
///
/// # Errors
///
/// Returns the original canonical registry error.
pub fn unregister_backend(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendDescriptor> {
    registry.unregister(identifier)
}

// ============================================================================
// ERROR INTEGRATION
// ============================================================================

/// Return the canonical registry error as a standard error reference.
///
/// This helper preserves the original error type and does not
/// transform or discard its diagnostic information.
pub fn registry_error_source(
    error: &RegistryError,
) -> &(dyn Error + 'static) {
    error
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            COMPILER_BACKEND_REGISTRY_MODULE_NAME,
            "zamani.compiler.backends.registry"
        );

        assert_ne!(
            COMPILER_BACKEND_REGISTRY_API_VERSION,
            0
        );
    }

    #[test]
    fn compiler_registry_is_the_canonical_registry_type() {
        fn accepts_canonical(_: &BackendRegistry) {}

        fn accepts_compiler(_: &CompilerBackendRegistry) {}

        let registry = new_compiler_backend_registry();

        accepts_canonical(&registry);
        accepts_compiler(&registry);
    }

    #[test]
    fn registry_construction_creates_empty_registry() {
        let registry = new_compiler_backend_registry();

        assert!(
            backend_registry_is_empty(&registry)
                .expect("registry inspection must succeed")
        );

        assert_eq!(
            registered_backend_count(&registry)
                .expect("registry inspection must succeed"),
            0
        );
    }

    #[test]
    fn registry_clone_preserves_canonical_type_identity() {
        let original = new_compiler_backend_registry();
        let cloned: CompilerBackendRegistry = original.clone();

        let canonical: BackendRegistry = cloned;

        assert_eq!(
            original.len().expect("registry must be accessible"),
            canonical.len().expect("registry must be accessible")
        );
    }

    #[test]
    fn arbitrary_backend_descriptors_are_representable() {
        let descriptor = BackendDescriptor::new(
            "future.vendor:heterogeneous-accelerator",
            ".future",
        )
        .with_description("Future heterogeneous computing provider");

        assert_eq!(
            descriptor.id,
            "future.vendor:heterogeneous-accelerator"
        );

        assert_eq!(
            descriptor.file_extension,
            ".future"
        );
    }

    #[test]
    fn backend_descriptor_aliases_are_preserved() {
        let descriptor = BackendDescriptor::new(
            "vendor.accelerator",
            ".acc",
        )
        .with_alias("accelerator")
        .with_alias("vendor.acc");

        assert_eq!(
            descriptor.aliases,
            vec![
                "accelerator".to_owned(),
                "vendor.acc".to_owned()
            ]
        );
    }

    #[test]
    fn backend_descriptor_metadata_is_available() {
        let descriptor = BackendDescriptor::new(
            "vendor.backend",
            ".out",
        )
        .with_description("Example backend");

        assert_eq!(
            descriptor.description,
            "Example backend"
        );
    }

    #[test]
    fn unknown_backend_errors_are_preserved() {
        let error = RegistryError::BackendNotFound {
            identifier: "unknown.backend".to_owned(),
        };

        assert!(
            error.to_string().contains("unknown.backend")
        );
    }

    #[test]
    fn registry_helpers_preserve_empty_inventory() {
        let registry = new_compiler_backend_registry();

        assert!(
            registered_backends(&registry)
                .expect("discovery must succeed")
                .is_empty()
        );

        assert!(
            registered_backend_ids(&registry)
                .expect("identifier enumeration must succeed")
                .is_empty()
        );

        assert!(
            !is_backend_registered(&registry, "unknown.backend")
                .expect("registry lookup must succeed")
        );
    }

    #[test]
    fn arbitrary_backend_identifiers_are_not_hardcoded() {
        let identifiers = [
            "classical.vendor:processor",
            "quantum.vendor:qpu",
            "hdl.vendor:fpga",
            "ai.vendor:tensor-engine",
            "distributed.vendor:executor",
            "future.vendor:unknown-computing-model",
        ];

        for identifier in identifiers {
            let descriptor = BackendDescriptor::new(
                identifier,
                ".artifact",
            );

            assert_eq!(descriptor.id, identifier);
        }
    }
}
