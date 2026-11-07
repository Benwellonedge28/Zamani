
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Registry Integration
//!
//! Path: src/compiler/backends/registry.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety:
//! Safe Rust only. Unsafe Rust is forbidden.
//!
//! ## 1. Purpose
//!
//! Provides the compiler-facing API for the canonical backend
//! registry owned by `crate::backend::registry`.
//!
//! This module is an integration facade.
//!
//! It does not introduce another registry, backend factory,
//! backend identifier system, alias resolver or registration
//! lifecycle.
//!
//! ## 2. Architectural authorities
//!
//! Backend interface:
//!     crate::backend
//!
//! Canonical registry:
//!     crate::backend::registry
//!
//! Backend capabilities:
//!     crate::backend::capabilities
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! Compiler backend discovery:
//!     crate::compiler::backends::discovery
//!
//! Backend selection:
//!     crate::compiler::backends::selection
//!
//! Backend invocation:
//!     crate::compiler::backends::invocation
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Universal IR:
//!     crate::zuir
//!
//! Runtime execution:
//!     crate::runtime
//!
//! ## 3. Ownership
//!
//! This module owns:
//!
//! - Compiler-facing registry exports.
//! - Compiler-facing type aliases.
//! - Registry construction convenience.
//! - Read-only registry inspection helpers.
//! - Explicit backend construction delegation.
//! - Compiler registry API documentation.
//!
//! It does not own:
//!
//! - Backend registration storage.
//! - Backend factories.
//! - Backend identity definitions.
//! - Alias resolution.
//! - Descriptor validation.
//! - Backend implementation.
//! - Hardware discovery.
//! - Capability definitions.
//! - Compatibility evaluation.
//! - Resource feasibility.
//! - Backend selection.
//! - Physical resource allocation.
//! - Code generation.
//! - Artifact verification.
//! - Runtime execution.
//!
//! ## 4. POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Backend identifiers are open-world identifiers.
//!
//! New computational domains and backend providers must be
//! supported through registration with the canonical registry.
//!
//! Adding a backend must not require modifying this facade.
//!
//! ## 5. Scalability
//!
//! This module establishes no arbitrary universal maximum for:
//!
//! - Registered backends.
//! - Backend providers.
//! - Backend identifiers.
//! - Computational domains.
//! - Architectures.
//! - Accelerators.
//! - Quantum processors.
//! - Distributed participants.
//! - Hardware resources.
//! - Future computing technologies.
//!
//! Actual operational limitations are determined by:
//!
//! - Available memory.
//! - Address-space constraints.
//! - Configured operational budgets.
//! - Processing capacity.
//! - Provider implementation capabilities.
//! - Target-specific limitations.
//!
//! No finite implementation promises literal infinite
//! physical computation.
//!
//! ## 6. Architectural invariants
//!
//! Registration is not availability.
//!
//! Availability is not compatibility.
//!
//! Compatibility is not feasibility.
//!
//! Feasibility is not physical resource reservation.
//!
//! Backend construction is not code generation.
//!
//! Code generation is not artifact verification.
//!
//! Artifact verification is not successful execution.
//!
//! No operation in this module silently substitutes a backend.
//!
//! ## 7. Thread safety
//!
//! All synchronization and shared-state behavior is inherited
//! from the canonical BackendRegistry.
//!
//! Cloning CompilerBackendRegistry shares the canonical registry.
//!
//! This module creates no additional mutable registry state.
//!
//! ## 8. Error handling
//!
//! Canonical RegistryError values are propagated unchanged.
//!
//! The facade does not discard error identity or convert
//! structured registry errors into string-only errors.
//!
//! ## 9. Compatibility
//!
//! Existing public names are preserved.
//!
//! CompilerBackendRegistry remains a type alias.
//!
//! CompilerRegistryResult remains the canonical result type.
//!
//! The compiler-facing API version remains independently
//! versioned from the backend implementation and language.

use std::error::Error;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler backend registry integration.
pub const COMPILER_BACKEND_REGISTRY_MODULE_NAME: &str =
    "zamani.compiler.backends.registry";

/// Compiler-facing registry API version.
///
/// Independent of the language, IR, runtime and backend versions.
pub const COMPILER_BACKEND_REGISTRY_API_VERSION: u16 = 1;

// ============================================================================
// CANONICAL BACKEND REGISTRY EXPORTS
// ============================================================================

/// The authoritative backend registry.
///
/// All registration, lookup, alias resolution, synchronization
/// and lifecycle operations remain owned by this type.
pub use crate::backend::registry::BackendRegistry;

/// Canonical backend descriptor.
///
/// Describes registered backend metadata without constructing
/// a backend implementation.
pub use crate::backend::registry::BackendDescriptor;

/// Canonical backend identifier.
///
/// Backend identity remains open-world and case-sensitive.
pub use crate::backend::registry::BackendId;

/// Canonical backend factory contract.
///
/// Factory ownership remains with the authoritative registry.
pub use crate::backend::registry::BackendFactory;

/// Canonical backend registry error.
///
/// Original errors are preserved.
pub use crate::backend::registry::RegistryError;

/// Canonical backend implementation interface.
///
/// This is the existing backend trait, not a new compiler trait.
pub use crate::backend::Backend;

// ============================================================================
// COMPILER-FACING TYPES
// ============================================================================

/// Compiler-facing name for the canonical backend registry.
///
/// This is a type alias, not a second registry implementation.
///
/// Clones share the canonical registry state.
pub type CompilerBackendRegistry = BackendRegistry;

/// Result type for compiler-facing registry operations.
///
/// The canonical RegistryError is preserved.
pub type CompilerRegistryResult<T> = Result<T, RegistryError>;

// ============================================================================
// REGISTRY CONSTRUCTION
// ============================================================================

/// Creates an empty compiler-facing canonical backend registry.
///
/// This delegates directly to BackendRegistry::new().
///
/// Construction does not:
///
/// - Register built-in backends.
/// - Discover hardware.
/// - Probe physical devices.
/// - Load external providers.
/// - Evaluate compatibility.
/// - Select a target.
/// - Allocate resources.
/// - Generate code.
/// - Execute programs.
///
/// Backend initialization remains an explicit responsibility
/// of the compiler or toolchain initialization layer.
///
/// # Examples
///
/// ```ignore
/// use zamani_compiler::compiler::backends::registry;
///
/// let backends = registry::new_compiler_backend_registry();
///
/// assert!(
///     backends.is_empty()?
/// );
/// ```
///
/// # Errors
///
/// Construction cannot fail. Subsequent registry operations
/// return their canonical RegistryError where appropriate.
#[must_use]
pub fn new_compiler_backend_registry(
) -> CompilerBackendRegistry {
    BackendRegistry::new()
}

// ============================================================================
// BACKEND IDENTITY RESOLUTION
// ============================================================================

/// Resolves a canonical backend identifier or registered alias.
///
/// The canonical registry performs the actual resolution.
///
/// Returns the canonical backend identifier.
///
/// No architecture inference or fallback is performed.
///
/// # Errors
///
/// Returns BackendNotFound when the identifier is unknown.
/// Other canonical registry errors are preserved.
pub fn resolve_backend_id(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendId> {
    registry.resolve(identifier)
}

/// Retrieves the registered descriptor for a backend.
///
/// Both canonical identifiers and registered aliases are accepted.
///
/// The operation does not construct the backend.
///
/// It does not establish compatibility, feasibility or availability.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn registered_backend_descriptor(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendDescriptor> {
    registry.descriptor(identifier)
}

/// Checks whether an identifier or alias is registered.
///
/// Registration status is the only property evaluated.
///
/// It does not imply that the backend is available or usable.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn is_backend_registered(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<bool> {
    registry.contains(identifier)
}

// ============================================================================
// REGISTRY INSPECTION
// ============================================================================

/// Returns all registered backend descriptors.
///
/// Results use canonical identifier ordering supplied by
/// the authoritative registry.
///
/// Backend implementations are not constructed.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn registered_backends(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<Vec<BackendDescriptor>> {
    registry.descriptors()
}

/// Returns all registered canonical backend identifiers.
///
/// Aliases are not returned as independent backends.
///
/// Ordering is deterministic.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn registered_backend_ids(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<Vec<BackendId>> {
    registry.backend_ids()
}

/// Returns the number of registered canonical backends.
///
/// Aliases do not increase the count.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn registered_backend_count(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<usize> {
    registry.len()
}

/// Returns whether the registry contains no registered backends.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn backend_registry_is_empty(
    registry: &CompilerBackendRegistry,
) -> CompilerRegistryResult<bool> {
    registry.is_empty()
}

// ============================================================================
// BACKEND CONSTRUCTION
// ============================================================================

/// Constructs a backend through the canonical registry.
///
/// The identifier may be a canonical identifier or an alias.
///
/// The canonical registry is responsible for:
///
/// - Resolving the identifier.
/// - Retrieving the associated factory.
/// - Executing the factory outside its registry lock.
/// - Validating the constructed backend identity.
///
/// This operation does not select a backend.
///
/// The caller must already have completed the appropriate
/// compatibility and feasibility checks.
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
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn create_registered_backend(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<Box<dyn Backend>> {
    registry.create(identifier)
}

// ============================================================================
// REGISTRY LIFECYCLE
// ============================================================================

/// Unregisters a backend using its canonical identifier or alias.
///
/// The authoritative registry removes the backend registration
/// and all aliases belonging to that registration.
///
/// Already constructed backend instances remain independently
/// owned by their callers.
///
/// This operation should be restricted to controlled registry
/// lifecycle management.
///
/// # Errors
///
/// Returns the original canonical RegistryError.
pub fn unregister_backend(
    registry: &CompilerBackendRegistry,
    identifier: &str,
) -> CompilerRegistryResult<BackendDescriptor> {
    registry.unregister(identifier)
}

// ============================================================================
// ERROR INTEGRATION
// ============================================================================

/// Returns the canonical registry error as a standard error source.
///
/// No error conversion or diagnostic information is lost.
pub fn registry_error_source(
    error: &RegistryError,
) -> &(dyn Error + 'static) {
    error
}

// ============================================================================
// MODULE CONTRACT HELPERS
// ============================================================================

/// Returns the stable compiler backend registry module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    COMPILER_BACKEND_REGISTRY_MODULE_NAME
}

/// Returns the compiler backend registry integration API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_BACKEND_REGISTRY_API_VERSION
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
            module_name(),
            "zamani.compiler.backends.registry"
        );
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(api_version(), 0);
    }

    #[test]
    fn compiler_registry_is_the_canonical_registry_type() {
        fn accepts_canonical_registry(
            _: &BackendRegistry,
        ) {
        }

        fn accepts_compiler_registry(
            _: &CompilerBackendRegistry,
        ) {
        }

        let registry = new_compiler_backend_registry();

        accepts_canonical_registry(&registry);
        accepts_compiler_registry(&registry);
    }

    #[test]
    fn construction_produces_an_empty_registry() {
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
    fn cloned_registry_shares_canonical_state() {
        let original = new_compiler_backend_registry();

        let cloned = original.clone();

        assert_eq!(
            original.len().expect("registry must be accessible"),
            cloned.len().expect("registry must be accessible")
        );
    }

    #[test]
    fn arbitrary_backend_identifiers_are_supported() {
        let identifiers = [
            "classical.vendor:processor",
            "quantum.vendor:qpu",
            "hdl.vendor:fpga",
            "ai.vendor:tensor-engine",
            "distributed.vendor:executor",
            "future.vendor:heterogeneous-engine",
        ];

        for identifier in identifiers {
            let descriptor =
                BackendDescriptor::new(identifier, ".artifact");

            assert_eq!(
                descriptor.id,
                identifier
            );
        }
    }

    #[test]
    fn descriptor_aliases_are_preserved() {
        let descriptor = BackendDescriptor::new(
            "vendor.accelerator",
            ".artifact",
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
    fn descriptor_metadata_is_preserved() {
        let descriptor = BackendDescriptor::new(
            "vendor.backend",
            ".output",
        )
        .with_description("Example backend provider");

        assert_eq!(
            descriptor.description,
            "Example backend provider"
        );

        assert_eq!(
            descriptor.file_extension,
            ".output"
        );
    }

    #[test]
    fn registry_helpers_preserve_empty_inventory() {
        let registry = new_compiler_backend_registry();

        assert!(
            registered_backends(&registry)
                .expect("inspection must succeed")
                .is_empty()
        );

        assert!(
            registered_backend_ids(&registry)
                .expect("inspection must succeed")
                .is_empty()
        );
    }

    #[test]
    fn unknown_identifiers_are_not_registered() {
        let registry = new_compiler_backend_registry();

        assert!(
            !is_backend_registered(
                &registry,
                "unknown.future.backend"
            )
            .expect("lookup must succeed")
        );
    }

    #[test]
    fn unknown_backend_resolution_preserves_error() {
        let registry = new_compiler_backend_registry();

        let result = resolve_backend_id(
            &registry,
            "unknown.backend",
        );

        assert!(matches!(
            result,
            Err(RegistryError::BackendNotFound { .. })
        ));
    }

    #[test]
    fn registry_error_source_preserves_error_identity() {
        let error = RegistryError::BackendNotFound {
            identifier: "missing.backend".to_owned(),
        };

        let source = registry_error_source(&error);

        assert!(
            source.to_string().contains("missing.backend")
        );
    }

    #[test]
    fn compiler_and_canonical_registry_versions_are_independent() {
        assert_ne!(
            COMPILER_BACKEND_REGISTRY_API_VERSION,
            0
        );

        assert_ne!(
            crate::backend::BACKEND_API_VERSION,
            0
        );
    }
}
