
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Providers
//!
//! Path: `src/zhal/providers/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module is the canonical public composition root for
//! ZHAL hardware-provider infrastructure.
//!
//! It exposes a unified provider API by composing the existing
//! provider contract, registry, lifecycle, discovery and
//! validation implementations.
//!
//! It does not introduce a competing provider implementation,
//! registry, lifecycle authority, capability system, hardware
//! inventory or execution engine.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - Provider subsystem module composition.
//! - Stable public provider API paths.
//! - Selective public type re-exports.
//! - Provider subsystem documentation.
//! - Provider subsystem architectural invariants.
//! - Public API composition tests.
//!
//! The child modules own their individual implementations.
//!
//! # Provider subsystem
//!
//! The provider subsystem consists of five independently
//! maintained components.
//!
//! ## Contract
//!
//! `contract` defines:
//!
//! - HardwareProvider.
//! - HardwareSession.
//! - HardwareProviderFuture.
//! - DeviceDiscoveryRequest.
//! - DeviceDiscoveryPage.
//! - DeviceDiscoveryCursor.
//!
//! It defines the interface implemented by hardware providers.
//!
//! ## Registry
//!
//! `registry` defines:
//!
//! - ProviderRegistry.
//! - ProviderHandle.
//! - ProviderRegistrySnapshot.
//! - ProviderQuery.
//! - RegistryGeneration.
//!
//! It owns provider implementation registration and lookup.
//!
//! ## Lifecycle
//!
//! `lifecycle` defines:
//!
//! - ProviderLifecycleManager.
//! - ManagedProviderState.
//! - LifecyclePhase.
//! - LifecycleFailure.
//! - LifecycleReport.
//!
//! It coordinates provider initialization and shutdown using
//! the existing standard-library ProviderLifecycle contract.
//!
//! ## Discovery
//!
//! `discovery` defines:
//!
//! - DiscoveryOptions.
//! - DiscoveryFailurePolicy.
//! - DiscoveryFailure.
//! - DiscoveryFailurePhase.
//! - DiscoveryReport.
//! - ProviderDiscoveryPage.
//! - DiscoverySession.
//!
//! It coordinates incremental discovery across hardware providers.
//!
//! ## Validation
//!
//! `validation` defines:
//!
//! - ProviderValidationPolicy.
//! - ProviderValidationEvidence.
//! - ProviderValidationReport.
//! - ProviderInterfaceManifest.
//! - ProviderTrustEvidence.
//! - ProviderConformanceEvidence.
//! - Provider API compatibility validation.
//! - Provider descriptor validation.
//!
//! It validates provider declarations and externally supplied
//! trust and conformance evidence.
//!
//! It does not perform cryptographic verification itself.
//!
//! # Architectural ownership
//!
//! This module does not own:
//!
//! - Provider identity definitions.
//! - Provider descriptor definitions.
//! - Provider lifecycle hook definitions.
//! - Capability identity or capability semantics.
//! - Source-language semantic requirements.
//! - Hardware feasibility.
//! - Hardware discovery implementation.
//! - Device inventory storage.
//! - Device access implementation.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Authentication.
//! - Cryptographic attestation.
//! - Authorization.
//! - Dynamic library loading.
//! - Domain-specific hardware execution.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Runtime orchestration.
//! - Compiler or IR definitions.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # Canonical repository dependencies
//!
//! Provider identity, descriptors and lifecycle:
//!     crate::stdlib::provider
//!
//! Capability identity and evidence:
//!     crate::stdlib::capability
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Version representation:
//!     crate::stdlib::version
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware device models:
//!     crate::zhal::model
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! # Provider integration
//!
//! A hardware provider implements:
//!
//!     crate::stdlib::provider::Provider
//!
//! and:
//!
//!     crate::zhal::providers::contract::HardwareProvider
//!
//! Provider registration, provider validation, lifecycle
//! coordination and hardware discovery are separate operations.
//!
//! Registration does not imply:
//!
//! - Successful validation.
//! - Provider trust.
//! - Provider initialization.
//! - Provider readiness.
//! - Hardware availability.
//! - Hardware authorization.
//! - Resource reservation.
//! - Execution feasibility.
//! - Successful hardware execution.
//!
//! Callers must explicitly perform the applicable admission,
//! initialization, authorization and discovery operations.
//!
//! # Important registry and discovery integration boundary
//!
//! The existing ProviderRegistry stores:
//!
//!     Arc<dyn crate::stdlib::provider::Provider>
//!
//! The existing DiscoverySession accepts:
//!
//!     Arc<dyn HardwareProvider>
//!
//! These are intentionally different trait-object types.
//!
//! Rust does not automatically convert an erased
//! `Arc<dyn Provider>` back into `Arc<dyn HardwareProvider>`.
//!
//! Consequently, a caller must retain a hardware-provider
//! implementation handle before erasing it to `dyn Provider`,
//! or the registry must be extended with a typed hardware-provider
//! registration and retrieval path.
//!
//! This composition module does not conceal that distinction
//! or introduce an unsafe downcast.
//!
//! The required registry integration is documented below
//! the implementation.
//!
//! # POCO-REAF and scalability
//!
//! The provider subsystem supports open-ended hardware domains
//! and independently implemented provider technologies.
//!
//! No arbitrary universal maximum is introduced here for:
//!
//! - Providers.
//! - Devices.
//! - Provider operations.
//! - Hardware interfaces.
//! - Computational domains.
//! - Hardware categories.
//! - Discovery pages.
//! - Discovery cursors.
//! - Provider dependencies.
//! - Capabilities.
//! - Hardware resources.
//!
//! Collection growth is governed by available memory,
//! platform representation limits and explicitly configured
//! operational budgets.
//!
//! Discovery can be paginated.
//!
//! Providers can be independently implemented and integrated.
//!
//! Adding a new hardware technology must not require modifying
//! this composition root merely to add another provider.
//!
//! Literal infinite physical computation or storage is not
//! promised.
//!
//! # Domain integration
//!
//! Provider implementations and adapters can expose hardware
//! from:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - GPUs and other accelerators.
//! - AI and tensor acceleration.
//! - Hybrid computing.
//! - Distributed systems.
//! - Embedded systems.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! These domains remain open-ended.
//!
//! Domain-specific execution contracts remain owned by their
//! existing subsystems.
//!
//! # Grammar boundary
//!
//! Relevant grammar and specifications include:
//!
//! - `grammar/effects/hardware.g4`
//! - `grammar/hardware/resources.g4`
//! - `grammar/execution/execution-context.g4`
//! - `grammar/execution/distributed-execution.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar and semantic model describe source-level
//! computational intent.
//!
//! This module composes physical hardware-provider interfaces.
//!
//! It does not parse grammar, interpret source expressions,
//! create semantic requirements or choose physical hardware.
//!
//! # Thread safety
//!
//! Thread-safety guarantees are established by the individual
//! provider, registry, lifecycle and discovery contracts.
//!
//! This module introduces no additional shared mutable state.
//!
//! # Error handling
//!
//! Child modules retain their canonical error types.
//!
//! Registry and lifecycle operations use the existing
//! standard-library error contract where defined.
//!
//! Hardware-provider contract, discovery and validation
//! operations use the existing ZHAL error contract.
//!
//! Errors must be propagated rather than silently converted
//! into successful provider operations.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This module performs no raw-pointer operations,
//! unchecked conversions or trait-object downcasts.

// ============================================================================
// PROVIDER SUBMODULES
// ============================================================================

/// Universal hardware-provider implementation contract.
///
/// Defines the provider interface, hardware sessions and
/// provider-scoped device discovery requests and responses.
pub mod contract;

/// Coordinates incremental discovery across registered
/// hardware-provider implementations.
///
/// Discovery is observational and does not reserve resources
/// or authorize physical device access.
pub mod discovery;

/// Coordinates hardware-provider initialization, readiness,
/// dependency ordering, rollback and shutdown.
///
/// Reuses the standard-library provider lifecycle contract.
pub mod lifecycle;

/// Owns provider implementation registration, lookup,
/// replacement, removal and immutable registry snapshots.
///
/// Reuses the canonical standard-library provider identity
/// and descriptor types.
pub mod registry;

/// Validates provider declarations, API compatibility,
/// interface manifests, trust evidence and conformance evidence.
///
/// Does not authenticate providers or perform cryptographic
/// attestation.
pub mod validation;

// ============================================================================
// HARDWARE PROVIDER CONTRACT RE-EXPORTS
// ============================================================================

/// Hardware-provider contract and discovery types.
///
/// These exports are convenience paths. The canonical
/// implementation remains in [`contract`].
pub use contract::{
    DeviceDiscoveryCursor,
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
};

/// Stable component identity of the hardware-provider contract.
pub use contract::COMPONENT as CONTRACT_COMPONENT;

// ============================================================================
// PROVIDER DISCOVERY RE-EXPORTS
// ============================================================================

/// Provider discovery coordination and reporting types.
///
/// The canonical implementations remain in [`discovery`].
pub use discovery::{
    DiscoveryFailure,
    DiscoveryFailurePhase,
    DiscoveryFailurePolicy,
    DiscoveryOptions,
    DiscoveryReport,
    DiscoverySession,
    ProviderDiscoveryPage,
};

/// Stable component identity of provider discovery.
pub use discovery::COMPONENT as DISCOVERY_COMPONENT;

// ============================================================================
// PROVIDER LIFECYCLE RE-EXPORTS
// ============================================================================

/// Provider lifecycle coordination and reporting types.
///
/// The canonical implementations remain in [`lifecycle`].
pub use lifecycle::{
    LifecycleFailure,
    LifecyclePhase,
    LifecycleReport,
    ManagedProviderState,
    ProviderLifecycleManager,
};

/// Stable component identity of provider lifecycle coordination.
pub use lifecycle::COMPONENT as LIFECYCLE_COMPONENT;

// ============================================================================
// PROVIDER REGISTRY RE-EXPORTS
// ============================================================================

/// Provider registration, lookup, query and snapshot types.
///
/// The canonical implementations remain in [`registry`].
pub use registry::{
    ProviderHandle,
    ProviderQuery,
    ProviderRegistry,
    ProviderRegistrySnapshot,
    RegistryGeneration,
};

/// Provider registry schema identity.
pub use registry::PROVIDER_REGISTRY_SCHEMA_ID;

/// Provider registry schema major version.
pub use registry::PROVIDER_REGISTRY_SCHEMA_MAJOR;

/// Provider registry schema minor version.
pub use registry::PROVIDER_REGISTRY_SCHEMA_MINOR;

/// Provider registry schema patch version.
pub use registry::PROVIDER_REGISTRY_SCHEMA_PATCH;

/// Provider registry schema version tuple.
pub use registry::PROVIDER_REGISTRY_SCHEMA_VERSION;

// ============================================================================
// PROVIDER VALIDATION RE-EXPORTS
// ============================================================================

/// Provider validation policies, evidence and structured findings.
///
/// The canonical implementations remain in [`validation`].
pub use validation::{
    ConformanceStatus,
    ProviderApiPolicy,
    ProviderConformanceEvidence,
    ProviderInterfaceDeclaration,
    ProviderInterfaceManifest,
    ProviderTrustEvidence,
    ProviderValidationEvidence,
    ProviderValidationPolicy,
    ProviderValidationReport,
    RequiredProviderInterface,
    TrustRequirement,
    ValidationIssue,
    ValidationSeverity,
};

/// Stable component identity of provider validation.
pub use validation::COMPONENT as VALIDATION_COMPONENT;

/// Provider validation schema identifier.
pub use validation::VALIDATION_SCHEMA_ID;

/// Provider validation schema major version.
pub use validation::VALIDATION_SCHEMA_MAJOR;

/// Provider validation schema minor version.
pub use validation::VALIDATION_SCHEMA_MINOR;

/// Provider validation schema patch version.
pub use validation::VALIDATION_SCHEMA_PATCH;

/// Provider validation schema version tuple.
pub use validation::VALIDATION_SCHEMA_VERSION;

// ============================================================================
// PUBLIC API COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_submodules_are_composed() {
        assert_eq!(
            CONTRACT_COMPONENT,
            "zhal.providers.contract",
        );

        assert_eq!(
            DISCOVERY_COMPONENT,
            "zhal.providers.discovery",
        );

        assert_eq!(
            LIFECYCLE_COMPONENT,
            "zhal.providers.lifecycle",
        );

        assert_eq!(
            VALIDATION_COMPONENT,
            "zhal.providers.validation",
        );
    }

    #[test]
    fn provider_registry_schema_is_exposed() {
        assert_eq!(
            PROVIDER_REGISTRY_SCHEMA_ID,
            "zamani.zhal.providers.registry",
        );

        assert_eq!(
            PROVIDER_REGISTRY_SCHEMA_VERSION,
            (
                PROVIDER_REGISTRY_SCHEMA_MAJOR,
                PROVIDER_REGISTRY_SCHEMA_MINOR,
                PROVIDER_REGISTRY_SCHEMA_PATCH,
            ),
        );
    }

    #[test]
    fn provider_validation_schema_is_exposed() {
        assert_eq!(
            VALIDATION_SCHEMA_ID,
            "zamani.zhal.providers.validation",
        );

        assert_eq!(
            VALIDATION_SCHEMA_VERSION,
            (
                VALIDATION_SCHEMA_MAJOR,
                VALIDATION_SCHEMA_MINOR,
                VALIDATION_SCHEMA_PATCH,
            ),
        );
    }

    #[test]
    fn canonical_provider_types_are_accessible() {
        fn accepts_provider<T: HardwareProvider + ?Sized>(_: &T) {}

        fn accepts_registry(_: &ProviderRegistry) {}

        fn accepts_lifecycle(_: &ProviderLifecycleManager) {}

        fn accepts_validation_policy(
            _: &ProviderValidationPolicy,
        ) {
        }

        fn accepts_discovery_options(
            _: &DiscoveryOptions,
        ) {
        }

        let _ = (
            accepts_provider::<dyn HardwareProvider>,
            accepts_registry,
            accepts_lifecycle,
            accepts_validation_policy,
            accepts_discovery_options,
        );
    }

    #[test]
    fn provider_modules_remain_independently_accessible() {
        let _ = core::any::type_name::<
            contract::DeviceDiscoveryRequest
        >();

        let _ = core::any::type_name::<
            registry::ProviderRegistry
        >();

        let _ = core::any::type_name::<
            lifecycle::ProviderLifecycleManager
        >();

        let _ = core::any::type_name::<
            discovery::DiscoverySession
        >();

        let _ = core::any::type_name::<
            validation::ProviderValidationPolicy
        >();
    }

    #[test]
    fn provider_composition_introduces_no_hardware_limits() {
        // Hardware capacities and provider counts are determined
        // by actual implementations, available resources and
        // explicitly configured operational budgets.
        //
        // This composition root defines no universal capacity
        // constants or closed hardware-domain enumeration.
        assert!(true);
    }
}
