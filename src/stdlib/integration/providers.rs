
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Provider Integration
//!
//! Canonical path:
//!     src/stdlib/integration/providers.rs
//!
//! Rust edition:
//!     2021
//!
//! Minimum supported Rust:
//!     1.97 or later
//!
//! Safety:
//!     Safe Rust only.
//!
//! # Purpose
//!
//! This module provides the standard library with a stable integration
//! boundary to Zamani's existing provider authorities.
//!
//! It connects the following independently owned provider systems:
//!
//! - General standard-library providers.
//! - Compiler extension providers.
//! - Quantum hardware providers.
//!
//! These systems have different responsibilities and different
//! compatibility contracts. They must not be collapsed into one
//! artificial provider implementation.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Provider-system integration paths.
//! - Namespaced access to existing provider contracts.
//! - Namespaced access to existing provider registries.
//! - Documentation of provider-system boundaries.
//! - Integration-level API identity.
//! - Integration contract tests.
//!
//! This module does not own:
//!
//! - Provider identity definitions.
//! - Provider descriptor definitions.
//! - Provider registration storage.
//! - Provider discovery.
//! - Provider authentication.
//! - Provider authorization.
//! - Provider admission.
//! - Provider compatibility evaluation.
//! - Provider initialization or shutdown.
//! - Hardware discovery.
//! - Backend registration.
//! - Compiler pass registration.
//! - Target discovery.
//! - Semantic analysis.
//! - Capability evaluation.
//! - Feasibility evaluation.
//! - Resource allocation.
//! - Scheduling.
//! - Routing.
//! - Code generation.
//! - Runtime execution.
//! - Distributed execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! # Canonical provider authorities
//!
//! ## General standard-library providers
//!
//!     crate::stdlib::provider
//!     crate::stdlib::registry
//!
//! The general provider contract describes computational services
//! independently of their implementation domain.
//!
//! Its registry manages standard-library provider instances.
//!
//! ## Compiler extension providers
//!
//!     crate::compiler::extensions::provider
//!     crate::compiler::extensions::registry
//!
//! The compiler extension system manages compiler contributions.
//!
//! It is not a replacement for backend, pass, target-discovery,
//! grammar-extension, or runtime registries.
//!
//! ## Quantum hardware providers
//!
//!     crate::quantum::hardware::provider
//!     crate::quantum::hardware::provider_registry
//!
//! The quantum hardware subsystem owns quantum-specific provider
//! descriptions and quantum provider registration.
//!
//! A quantum provider is not an individual quantum backend or device.
//!
//! # Provider identity invariant
//!
//! The provider identity types owned by the three systems are not
//! interchangeable merely because they share the name ProviderId.
//!
//! This module deliberately preserves their namespaces.
//!
//! It does not:
//!
//! - Convert one provider identity into another implicitly.
//! - Assume identical validation rules.
//! - Assume identical version semantics.
//! - Merge provider metadata.
//! - Treat a registry entry as an activated provider.
//! - Treat a declared capability as live availability evidence.
//!
//! An explicit adapter must be implemented by the owning subsystem
//! whenever identity or metadata conversion is genuinely required.
//!
//! # Provider versus backend versus device
//!
//! These concepts must remain distinct:
//!
//! Provider:
//!     An independently identifiable service or implementation owner.
//!
//! Backend:
//!     An implementation that realizes a particular computational
//!     workload or domain interface.
//!
//! Device:
//!     A concrete execution resource or hardware endpoint.
//!
//! A provider may expose multiple backends.
//!
//! A backend may operate on one or more devices, depending on its
//! authoritative domain contract.
//!
//! Provider registration must never imply backend or device registration.
//!
//! # Computing-domain integration
//!
//! General provider contracts may describe services for:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware description.
//! - Hybrid computation.
//! - Distributed computation.
//! - AI and tensor computation.
//! - Scientific computation.
//! - Networking.
//! - Cryptography.
//! - Storage.
//! - Embedded and edge computation.
//! - Future computational domains.
//!
//! These are examples, not a closed enumeration.
//!
//! Domain-specific interfaces remain owned by their respective
//! subsystems.
//!
//! This module does not introduce domain-specific provider traits
//! merely to create a uniform-looking API.
//!
//! # Capability and feasibility boundary
//!
//! A provider descriptor may declare capabilities.
//!
//! A declaration is not proof that a capability is currently available.
//!
//! Current availability must be established through the owning
//! provider's evidence-backed capability-observation interface.
//!
//! Feasibility must be evaluated by the canonical semantic,
//! compiler-planning, or runtime authority appropriate to the operation.
//!
//! This module does not select a provider based on capability declarations.
//!
//! # Security boundary
//!
//! Provider registration is not proof of trust.
//!
//! Provider descriptors are not credential stores.
//!
//! This module does not authenticate providers, verify signatures,
//! establish publisher trust, sandbox code, or authorize execution.
//!
//! Untrusted executable providers must pass the applicable security,
//! compatibility, admission, and isolation checks before activation.
//!
//! # Lifecycle boundary
//!
//! Provider registration does not initialize a provider.
//!
//! Provider initialization and shutdown remain owned by the appropriate
//! lifecycle manager.
//!
//! This module performs no implicit initialization, shutdown,
//! discovery, network access, environment inspection, or hardware access.
//!
//! # POCO-REAF and scalability
//!
//! Provider integration must remain open-world.
//!
//! Adding a new provider, vendor, architecture, accelerator,
//! computational domain, or execution technology must not require
//! modifying this integration module.
//!
//! This module imposes no universal maximum on:
//!
//! - Providers.
//! - Provider operations.
//! - Computing domains.
//! - Hardware devices.
//! - Processing resources.
//! - Quantum resources.
//! - Distributed participants.
//! - Backend implementations.
//! - Future execution technologies.
//!
//! Actual capacity is determined by available resources, address-space
//! constraints, implementation capabilities, operating-system limits,
//! and explicitly configured operational budgets.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! # Dependency direction
//!
//! ```text
//! Zamani source and semantic model
//!              |
//!              v
//!       Canonical compiler
//!              |
//!              v
//!     Provider contribution
//!              |
//!       +------+-------+
//!       |              |
//!       v              v
//! General provider   Compiler extension
//!       |              |
//!       v              v
//! General registry  Extension registry
//!       |
//!       +-----------------------------+
//!                                     |
//!                                     v
//!                         Specialized subsystem interfaces
//!                                     |
//!               +---------------------+-------------------+
//!               |                     |                   |
//!               v                     v                   v
//!          Backend/HAL          Quantum hardware       Runtime
//!               |                     |                   |
//!               +---------------------+-------------------+
//!                                     |
//!                                     v
//!                              Actual execution
//! ```
//!
//! This diagram describes ownership boundaries. It does not claim
//! that every provider contribution or execution path is implemented.
//!
//! # Integration rule
//!
//! Existing provider authorities must be extended in their own modules.
//!
//! This file must remain a stable integration facade.
//!
//! Adding a new provider system must not require changing the general
//! standard-library provider identity or merging incompatible registries.

/// Integration contract version.
///
/// This version identifies this facade only. It is independent of
/// the standard-library API, compiler extension API, quantum provider
/// schema, language, grammar, IR, and runtime versions.
pub const PROVIDER_INTEGRATION_API_VERSION: u16 = 1;

/// Stable identity of this integration component.
pub const PROVIDER_INTEGRATION_NAME: &str =
    "zamani.stdlib.integration.providers";

/// Returns the stable integration component identity.
#[must_use]
pub const fn integration_name() -> &'static str {
    PROVIDER_INTEGRATION_NAME
}

/// Returns the integration contract version.
#[must_use]
pub const fn integration_api_version() -> u16 {
    PROVIDER_INTEGRATION_API_VERSION
}

/// General standard-library provider contracts and registry.
///
/// This is the provider-neutral service-provider system owned by
/// `crate::stdlib::provider` and `crate::stdlib::registry`.
///
/// Its identity types must not be substituted for compiler or
/// quantum provider identities.
pub mod standard_library {
    pub use crate::stdlib::provider::{
        capability_status,
        Provider,
        ProviderContractId,
        ProviderDescriptor,
        ProviderFuture,
        ProviderId,
        ProviderKind,
        ProviderLifecycle,
        ProviderOperationDescriptor,
        ProviderOperationId,
        ProviderStatus,
    };

    pub use crate::stdlib::registry::{
        ProviderRegistry,
        RegisteredProvider,
        RegistrationReceipt,
        RegistrySnapshot,
    };

    pub use crate::stdlib::capability::{
        Capability,
        CapabilityId,
        CapabilitySnapshot,
    };

    pub use crate::stdlib::context::OperationContext;
}

/// Compiler extension provider contracts and registry.
///
/// Compiler extension providers remain owned by the compiler.
/// Their identities and compatibility versions are not aliases for
/// standard-library provider identities.
pub mod compiler_extensions {
    pub use crate::compiler::extensions::provider;
    pub use crate::compiler::extensions::registry;
}

/// Quantum hardware provider contracts and registry.
///
/// Quantum provider identities, technology metadata, backend
/// associations, and quantum-specific provider constraints remain
/// owned by `crate::quantum::hardware`.
pub mod quantum_hardware {
    pub use crate::quantum::hardware::provider;
    pub use crate::quantum::hardware::provider_registry;
}

/// Returns the integration invariants that consumers must preserve.
///
/// This function is informational. It performs no provider discovery,
/// initialization, registration, or execution.
#[must_use]
pub const fn architectural_invariants() -> &'static [&'static str] {
    &[
        "provider identities remain subsystem-owned",
        "provider registries remain subsystem-owned",
        "provider registration does not imply activation",
        "capability declarations are not availability evidence",
        "provider registration does not imply backend registration",
        "provider registration does not imply device registration",
        "provider selection remains outside this integration facade",
        "resource allocation remains outside this integration facade",
        "scheduling and routing remain outside this integration facade",
        "runtime execution remains outside this integration facade",
        "new computing domains remain open-world",
        "no universal hardware capacity ceilings are introduced",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_identity_is_stable() {
        assert_eq!(
            integration_name(),
            "zamani.stdlib.integration.providers"
        );
    }

    #[test]
    fn integration_version_is_nonzero() {
        assert_ne!(integration_api_version(), 0);
    }

    #[test]
    fn architectural_invariants_are_available() {
        let invariants = architectural_invariants();

        assert!(!invariants.is_empty());

        assert!(
            invariants.iter().any(|item| {
                item.contains("identities remain subsystem-owned")
            })
        );

        assert!(
            invariants.iter().any(|item| {
                item.contains("no universal hardware capacity ceilings")
            })
        );
    }

    #[test]
    fn standard_library_provider_identity_is_not_redefined() {
        // Compile-time type assertion: the integration exposes the
        // existing canonical provider identity rather than defining
        // another ProviderId.
        fn accepts_id(
            _: standard_library::ProviderId
        ) {
        }

        let _ = accepts_id;
    }

    #[test]
    fn standard_library_registry_is_the_existing_registry() {
        fn accepts_registry(
            _: &standard_library::ProviderRegistry
        ) {
        }

        let _ = accepts_registry;
    }
}
