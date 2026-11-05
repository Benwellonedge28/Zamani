
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Pass Registry
//!
//! Path: src/compiler/passes/registry.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide the compiler-facing registration and lookup boundary for
//! transformation and analysis passes.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! The canonical pass infrastructure is:
//!
//! - crate::zuir::pass::contracts
//! - crate::zuir::pass::registry
//! - crate::zuir::pass::manager
//!
//! This module deliberately reuses that infrastructure.
//! It does not introduce a second pass registry.
//!
//! OWNS
//!
//! - Compiler-facing access to the canonical pass registry.
//! - Stable compiler namespace for pass registration.
//! - Explicit aliases for the existing operational pass contract.
//! - Compiler-facing documentation of registry ownership.
//!
//! DOES NOT OWN
//!
//! - Pass identity or pass descriptor definitions.
//! - Pass versioning.
//! - Pass execution.
//! - Optimization algorithms.
//! - Pass scheduling or pipeline ordering.
//! - Analysis computation or invalidation.
//! - Semantic preservation proofs.
//! - IR verification algorithms.
//! - Hardware discovery or selection.
//! - Device allocation.
//! - Routing or physical scheduling.
//! - Backend invocation.
//!
//! CANONICAL OWNERSHIP
//!
//! Pass contracts:
//!     crate::zuir::pass::contracts
//!
//! Pass registration:
//!     crate::zuir::pass::registry
//!
//! Pass execution:
//!     crate::zuir::pass::manager
//!
//! Quantum-specific optimization:
//!     crate::quantum::optimization
//!
//! Classical optimization:
//!     crate::optimizer
//!
//! COMPILER INTEGRATION
//!
//! The compiler accesses the canonical registry through this module.
//! New compiler subsystems must not introduce independent definitions
//! of PassId, PassDescriptor, PassVersion, or PassError.
//!
//! DOMAIN EXTENSIBILITY
//!
//! The registry is generic over its IR representation.
//!
//! Different IR owners may instantiate the canonical registry for
//! their respective representations.
//!
//! New computational domains do not require modifications here.
//!
//! SCALABILITY
//!
//! No fixed pass-count ceiling is introduced.
//!
//! No fixed domain count is introduced.
//!
//! No hardware population or capacity is encoded here.
//!
//! Registry capacity is constrained only by the actual implementation,
//! address space, and available resources.
//!
//! REGISTRATION ORDER
//!
//! Registration order is not a persistent identity.
//!
//! Persisted pipeline descriptions must use stable pass IDs and versions.
//!
//! REGISTRY THREAD SAFETY
//!
//! Registration requires exclusive mutable access.
//! Concurrent registration must be coordinated by the owner.
//!
//! The registry does not introduce global mutable state.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! FREEZE CRITERIA
//!
//! - No duplicate pass registry exists.
//! - The canonical ZUIR registry is reused.
//! - Existing registration and lookup behavior is preserved.
//! - Compiler-facing types remain compatible with the canonical registry.
//! - No hardware or domain-specific ceilings are introduced.
//! - Unit and integration tests pass.

/// Canonical registry implementation.
///
/// This is a re-export, not a second registry.
///
/// Registration, duplicate detection, descriptor validation,
/// registration order, and implementation lookup remain owned by
/// `crate::zuir::pass::registry`.
pub use crate::zuir::pass::registry::PassRegistry;

/// Existing operational pass contract used by the canonical registry.
///
/// These aliases intentionally identify the manager's legacy contract
/// types. They must not be confused with the canonical contract types
/// exported directly by `crate::zuir::pass`.
///
/// A subsequent migration must make manager, registry, and contracts
/// share the canonical definitions before removing these aliases.
pub use crate::zuir::pass::manager::{
    Pass as RegisteredPass,
    PassDescriptor as RegisteredPassDescriptor,
    PassError as RegistryError,
    PassErrorKind as RegistryErrorKind,
    PassId as RegistryPassId,
    PassOutcome as RegisteredPassOutcome,
    PassVersion as RegistryPassVersion,
};

/// Canonical, implementation-independent pass identity.
///
/// This is the authoritative identity used by the contract subsystem.
pub use crate::zuir::pass::contracts::PassId;

/// Canonical pass descriptor.
pub use crate::zuir::pass::contracts::PassDescriptor;

/// Canonical pass version.
pub use crate::zuir::pass::contracts::PassVersion;

/// Canonical structured pass error.
pub use crate::zuir::pass::contracts::PassError;

/// Canonical pass error category.
pub use crate::zuir::pass::contracts::PassErrorKind;

/// Canonical completed-pass outcome.
pub use crate::zuir::pass::contracts::PassOutcome;

/// Compiler-facing registry alias.
///
/// This alias does not create a new registry implementation.
///
/// The IR type determines which pass implementations can be registered.
pub type CompilerPassRegistry<I> = PassRegistry<I>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::pass::manager::{
        Pass as OperationalPass,
        PassContext,
        PassDescriptor as OperationalDescriptor,
        PassError as OperationalError,
        PassOutcome as OperationalOutcome,
        PassVersion as OperationalVersion,
    };

    #[derive(Debug)]
    struct TestPass {
        descriptor: OperationalDescriptor,
    }

    impl OperationalPass<u64> for TestPass {
        fn descriptor(&self) -> OperationalDescriptor {
            self.descriptor.clone()
        }

        fn run(
            &mut self,
            _ir: &mut u64,
            _context: &mut PassContext,
        ) -> Result<OperationalOutcome, OperationalError> {
            Ok(OperationalOutcome::unchanged())
        }
    }

    fn test_pass(name: &str) -> TestPass {
        let id = RegistryPassId::new(name)
            .expect("test pass ID must be valid");

        TestPass {
            descriptor: OperationalDescriptor::new(
                id,
                OperationalVersion::new(1, 0, 0),
                "Compiler registry integration test pass",
            ),
        }
    }

    #[test]
    fn compiler_registry_uses_canonical_registry() {
        let registry: CompilerPassRegistry<u64> =
            CompilerPassRegistry::new();

        assert!(registry.is_empty());
    }

    #[test]
    fn registers_pass_by_stable_identity() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        registry
            .register(test_pass("classical.test"))
            .expect("registration should succeed");

        let id = RegistryPassId::new("classical.test")
            .expect("ID must be valid");

        assert!(registry.contains(&id));
        assert_eq!(registry.len(), 1);

        registry
            .validate()
            .expect("registry invariants must hold");
    }

    #[test]
    fn rejects_duplicate_registration() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        registry
            .register(test_pass("classical.test"))
            .expect("first registration should succeed");

        let result = registry.register(
            test_pass("classical.test")
        );

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);

        registry
            .validate()
            .expect("registry must remain consistent");
    }

    #[test]
    fn preserves_explicit_registration_order() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        for name in [
            "quantum.optimize",
            "classical.optimize",
            "hdl.optimize",
        ] {
            registry
                .register(test_pass(name))
                .expect("registration should succeed");
        }

        let registered: Vec<&str> = registry
            .ordered_ids()
            .map(|id| id.as_str())
            .collect();

        assert_eq!(
            registered,
            vec![
                "quantum.optimize",
                "classical.optimize",
                "hdl.optimize",
            ]
        );
    }

    #[test]
    fn descriptor_lookup_uses_registered_identity() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        registry
            .register(test_pass("tensor.optimize"))
            .expect("registration should succeed");

        let id = RegistryPassId::new("tensor.optimize")
            .expect("ID must be valid");

        let descriptor = registry
            .descriptor(&id)
            .expect("registered descriptor must exist");

        assert_eq!(
            descriptor.id.as_str(),
            "tensor.optimize"
        );

        assert_eq!(
            descriptor.version,
            OperationalVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn failed_registration_does_not_replace_existing_pass() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        registry
            .register(test_pass("core.existing"))
            .expect("initial registration should succeed");

        let result = registry.register(
            test_pass("core.existing")
        );

        assert!(result.is_err());
        assert_eq!(registry.len(), 1);

        let id = RegistryPassId::new("core.existing")
            .expect("ID must be valid");

        assert!(registry.get(&id).is_some());
    }

    #[test]
    fn registry_lookup_and_order_are_independent() {
        let mut registry = CompilerPassRegistry::<u64>::new();

        registry
            .register(test_pass("z_domain.last"))
            .expect("registration should succeed");

        registry
            .register(test_pass("a_domain.first"))
            .expect("registration should succeed");

        let ordered: Vec<&str> = registry
            .ordered_ids()
            .map(|id| id.as_str())
            .collect();

        assert_eq!(
            ordered,
            vec![
                "z_domain.last",
                "a_domain.first",
            ]
        );

        let lexical: Vec<&str> = registry
            .registered_ids()
            .map(|id| id.as_str())
            .collect();

        assert_eq!(
            lexical,
            vec![
                "a_domain.first",
                "z_domain.last",
            ]
        );
    }
}
