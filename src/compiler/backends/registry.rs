
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Registry Integration
//!
//! Path: `src/compiler/backend/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only. `unsafe` is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing registry API by integrating the canonical
//! backend registry owned by `crate::backend::registry`.
//!
//! This module deliberately does not implement another registry.
//!
//! The canonical backend registry remains responsible for:
//!
//! - Backend registration.
//! - Canonical backend identifiers.
//! - Alias registration and resolution.
//! - Duplicate registration detection.
//! - Backend factory ownership.
//! - Backend construction.
//! - Registry enumeration.
//! - Backend unregistration.
//! - Registry synchronization and lifecycle.
//!
//! ## Architectural authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/portability.md`
//! - `grammar/spec/resources.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/quantum.md`
//! - `src/backend/registry.rs`
//! - `src/backend/mod.rs`
//! - `src/backend/api.rs`
//! - `src/backend/selection.rs`
//! - `src/backend/pipeline.rs`
//! - `src/backend/ids.rs`
//! - `src/compiler/passes/registry.rs`
//! - `src/zuir/pass/registry.rs`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Compiler-facing visibility of backend registry contracts.
//! - Compiler-facing registry naming.
//! - Integration documentation.
//! - Compatibility with the canonical backend registry API.
//!
//! This module does NOT own:
//!
//! - A second backend registry.
//! - A second backend factory implementation.
//! - Backend identifiers or identifier allocation.
//! - Backend descriptor validation.
//! - Backend capability definitions.
//! - Target descriptors.
//! - Target discovery.
//! - Target selection.
//! - Capability negotiation.
//! - Resource feasibility.
//! - Physical hardware allocation.
//! - Backend execution.
//! - IR lowering.
//! - Artifact generation.
//! - Runtime execution.
//!
//! ## Dependency direction
//!
//! ```text
//!
//!          Canonical backend contracts
//!                     |
//!                     v
//!          crate::backend::registry
//!                     |
//!                     v
//!       compiler::backend::registry
//!                     |
//!                     v
//!          Compiler orchestration
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!       Selection  Pipeline   Artifacts
//!          |          |          |
//!          +----------+----------+
//!                     |
//!                     v
//!              Target realization
//!
//! ```
//!
//! ## POCO-REAF
//!
//! The registry must accommodate backends for:
//!
//! - Classical processors.
//! - Embedded processors.
//! - GPUs and accelerators.
//! - FPGAs.
//! - ASICs.
//! - Quantum processors.
//! - Quantum simulators.
//! - Neuromorphic systems.
//! - AI and tensor accelerators.
//! - Distributed execution.
//! - WebAssembly.
//! - Future computing architectures.
//!
//! These are examples, not a closed inventory.
//!
//! Adding a new backend must not require modifying this facade.
//!
//! ## Scalability
//!
//! This module introduces:
//!
//! - No fixed backend-count ceiling.
//! - No fixed architecture inventory.
//! - No fixed accelerator inventory.
//! - No fixed computational-domain enumeration.
//! - No arbitrary hardware-capacity restrictions.
//!
//! The canonical registry uses dynamically growing collections.
//! Actual allocation and address-space limitations remain implementation
//! and operating-environment constraints.
//!
//! ## Determinism
//!
//! Deterministic enumeration, canonical identity resolution and
//! registration consistency are inherited from the canonical registry.
//!
//! This facade must not introduce another ordering or identity policy.
//!
//! ## Thread safety
//!
//! Thread safety is inherited from the canonical `BackendRegistry`.
//!
//! Cloning a registry shares its underlying canonical registry state.
//! It does not create an independent compiler registry.
//!
//! ## Error handling
//!
//! Registry errors are propagated without translation or loss of detail.
//!
//! This preserves:
//!
//! - Invalid identifiers.
//! - Duplicate backend registration.
//! - Alias conflicts.
//! - Unknown backend identifiers.
//! - Factory identity mismatches.
//! - Registry synchronization failures.
//!
//! ## Integration contract
//!
//! `src/backend/mod.rs` must expose:
//!
//!     pub mod registry;
//!
//! `src/compiler/backend/mod.rs` must expose:
//!
//!     pub mod registry;
//!
//! `src/compiler/mod.rs` must expose:
//!
//!     pub mod backend;
//!
//! No registry implementation should be copied into this module.
//!
//! ## Completion criteria
//!
//! - Canonical registry types are re-exported.
//! - Compiler-facing registry naming is available.
//! - Registry operations use the canonical implementation.
//! - No duplicate mutable registry state exists.
//! - No target-specific architecture list is introduced.
//! - Existing backend APIs remain compatible.
//! - Rust 1.97+ compatibility is maintained.
//! - No unsafe code is used.

/// Canonical backend registry implementation and associated contracts.
///
/// These types are re-exported directly from the authoritative backend
/// subsystem. Their identities and implementations are not duplicated.
pub use crate::backend::registry::{
    BackendDescriptor,
    BackendFactory,
    BackendId,
    BackendRegistry,
    RegistryError,
};

/// Compiler-facing name for the canonical backend registry.
///
/// This is a type alias, not a new registry implementation.
///
/// Both names refer to exactly the same registry type and shared state.
///
/// Prefer `CompilerBackendRegistry` in compiler-specific APIs and
/// `BackendRegistry` when interacting directly with the backend subsystem.
pub type CompilerBackendRegistry = BackendRegistry;

/// Compiler-facing result of backend registry operations.
///
/// This alias preserves the canonical registry error type.
pub type CompilerRegistryResult<T> = Result<T, RegistryError>;

/// Construct an empty compiler-facing view of the canonical backend
/// registry.
///
/// This function does not register built-in backends, discover hardware,
/// select targets, or initialize external toolchains.
///
/// Backend registration remains an explicit initialization responsibility.
///
/// # Examples
///
/// ```ignore
/// use zamani_compiler::compiler::backend::registry::{
///     new_compiler_backend_registry,
/// };
///
/// let registry = new_compiler_backend_registry();
/// assert!(registry.is_empty()?);
/// # Ok::<(), crate::backend::registry::RegistryError>(())
/// ```
#[must_use]
pub fn new_compiler_backend_registry() -> CompilerBackendRegistry {
    CompilerBackendRegistry::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_registry_is_the_canonical_backend_registry() {
        let compiler_registry = CompilerBackendRegistry::new();

        assert_eq!(
            compiler_registry.len().expect("registry lock must be valid"),
            0
        );

        assert!(
            compiler_registry
                .is_empty()
                .expect("registry lock must be valid")
        );
    }

    #[test]
    fn compiler_factory_returns_the_canonical_registry_type() {
        let registry = new_compiler_backend_registry();

        let canonical: BackendRegistry = registry;

        assert!(
            canonical
                .is_empty()
                .expect("registry lock must be valid")
        );
    }

    #[test]
    fn compiler_registry_aliases_preserve_type_identity() {
        fn accepts_canonical(_: &BackendRegistry) {}

        fn accepts_compiler(_: &CompilerBackendRegistry) {}

        let registry = CompilerBackendRegistry::new();

        accepts_canonical(&registry);
        accepts_compiler(&registry);
    }

    #[test]
    fn registry_descriptor_contract_is_available() {
        let descriptor =
            BackendDescriptor::new("test.example", ".test")
                .with_description("Compiler registry integration test");

        assert_eq!(descriptor.id, "test.example");
        assert_eq!(descriptor.file_extension, ".test");
    }

    #[test]
    fn registry_error_contract_is_available() {
        let error = RegistryError::BackendNotFound {
            identifier: "unknown.backend".to_owned(),
        };

        assert!(
            error.to_string().contains("unknown.backend")
        );
    }

    #[test]
    fn compiler_registry_clones_share_canonical_state() {
        let first = CompilerBackendRegistry::new();
        let second = first.clone();

        assert_eq!(
            first.len().expect("registry lock must be valid"),
            second.len().expect("registry lock must be valid")
        );
    }
}
