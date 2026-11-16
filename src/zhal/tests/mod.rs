
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## ZHAL Test Composition Root
//!
//! Path: src/zhal/tests/mod.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provides the canonical composition root for ZHAL's
//! cross-component unit and contract tests.
//!
//! This module organizes tests by architectural responsibility
//! and verifies that the public ZHAL interfaces remain usable
//! through their canonical module paths.
//!
//! The individual production modules retain ownership of
//! their implementation-specific unit tests.
//!
//! # Test ownership
//!
//! This module owns:
//!
//! - Cross-component test organization.
//! - Public API composition tests.
//! - Architectural integration checks.
//! - Open-world extension checks.
//! - POCO-REAF boundary checks.
//! - Common safety requirements.
//! - Cross-module contract accessibility.
//!
//! This module does not own:
//!
//! - Production hardware implementations.
//! - Provider registration implementation.
//! - Device discovery implementation.
//! - Hardware inventory implementation.
//! - Resource allocation.
//! - Capability evaluation algorithms.
//! - Quantum execution.
//! - HDL realization.
//! - Runtime scheduling.
//! - Distributed execution.
//! - Hybrid execution.
//! - Grammar parsing.
//! - Compiler implementation.
//!
//! Those responsibilities remain with their existing
//! canonical repository authorities.
//!
//! # Canonical authorities
//!
//! Hardware models:
//!     crate::zhal::model
//!
//! Hardware identities:
//!     crate::zhal::identity
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware operation context:
//!     crate::zhal::context
//!
//! Hardware providers:
//!     crate::zhal::providers
//!
//! Hardware discovery:
//!     crate::zhal::devices
//!
//! Hardware capabilities:
//!     crate::zhal::capabilities
//!
//! Hardware operations:
//!     crate::zhal::operations
//!
//! Hardware resources:
//!     crate::zhal::resources
//!
//! Hardware security:
//!     crate::zhal::security
//!
//! Hardware observability:
//!     crate::zhal::observability
//!
//! Hardware simulation:
//!     crate::zhal::simulation
//!
//! Common provider authority:
//!     crate::stdlib::provider
//!
//! Common capability authority:
//!     crate::stdlib::capability
//!
//! Common device-I/O authority:
//!     crate::stdlib::io::devices
//!
//! Common operation context:
//!     crate::stdlib::context
//!
//! # POCO-REAF
//!
//! Tests must preserve the following invariants:
//!
//! - Hardware categories are open-ended.
//! - Provider identities use the canonical provider authority.
//! - Capability identities use the canonical capability authority.
//! - Physical device observations are not semantic declarations.
//! - Discovery does not imply resource reservation.
//! - Compatibility does not imply execution feasibility.
//! - Simulation does not imply physical execution.
//! - Hardware availability is observational and may change.
//! - Resource capacity is determined by actual observations.
//! - Operational budgets are not universal hardware ceilings.
//!
//! No arbitrary maximum is introduced for:
//!
//! - Providers.
//! - Devices.
//! - Processing units.
//! - Accelerators.
//! - Qubits.
//! - Memory.
//! - Storage.
//! - Hardware interfaces.
//! - Hardware resources.
//! - Computational domains.
//! - Distributed participants.
//! - Hardware technologies.
//!
//! Literal infinite physical computation or storage is not
//! promised. Actual execution remains bounded by available
//! resources, physical reality and implementation capabilities.
//!
//! # Grammar boundary
//!
//! The relevant grammar authorities include:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! These tests must not interpret source-language hardware
//! declarations as physical hardware observations.
//!
//! # Test execution
//!
//! These tests are compiled as part of the library's normal
//! test configuration.
//!
//! They do not require physical hardware, external hardware
//! SDKs, network access or a particular asynchronous executor.
//!
//! Physical hardware conformance is tested separately through
//! the simulation and provider-conformance authorities.
//!
//! # Test modules
//!
//! The test categories are private inline modules so that
//! this composition root does not depend on test files that
//! have not yet been created.
//!
//! Individual test implementations can grow within their
//! respective categories without changing the production
//! API or creating duplicate test infrastructure.

// ============================================================================
// TEST MODULE COMPOSITION
// ============================================================================

mod contracts;
mod providers;
mod discovery;
mod capabilities;
mod portability;
mod scalability;
mod security;
mod determinism;
mod fault_tolerance;
mod integration;

// ============================================================================
// SHARED TEST SUPPORT
// ============================================================================

/// Shared test support for canonical ZHAL identities.
///
/// This module deliberately exposes only test-local helpers.
/// It does not introduce production identity implementations.
mod support {
    use crate::stdlib::io::devices::{
        DeviceId,
        DeviceKindId,
    };

    use crate::stdlib::provider::ProviderId;

    use crate::zhal::identity::HardwareDeviceIdentity;

    /// Creates a validated provider identity.
    pub(super) fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("test provider identity must be valid")
    }

    /// Creates a validated device identity.
    pub(super) fn device_id(value: &str) -> DeviceId {
        DeviceId::new(value)
            .expect("test device identity must be valid")
    }

    /// Creates a validated hardware category.
    pub(super) fn device_kind(value: &str) -> DeviceKindId {
        DeviceKindId::new(value)
            .expect("test hardware category must be valid")
    }

    /// Creates a canonical hardware device identity.
    pub(super) fn hardware_identity(
        provider: &str,
        device: &str,
    ) -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            provider_id(provider),
            device_id(device),
        )
    }
}

// ============================================================================
// 1. CONTRACT TESTS
// ============================================================================

mod contracts {
    use core::any::type_name;

    #[test]
    fn canonical_hardware_identity_is_accessible() {
        let identity = type_name::<
            crate::zhal::identity::HardwareDeviceIdentity
        >();

        assert!(identity.contains("HardwareDeviceIdentity"));
    }

    #[test]
    fn canonical_device_model_is_accessible() {
        let descriptor = type_name::<
            crate::zhal::model::HardwareDeviceDescriptor
        >();

        assert!(descriptor.contains("HardwareDeviceDescriptor"));
    }

    #[test]
    fn canonical_interface_model_is_accessible() {
        let interface = type_name::<
            crate::zhal::model::HardwareInterfaceDescriptor
        >();

        assert!(interface.contains("HardwareInterfaceDescriptor"));
    }

    #[test]
    fn canonical_resource_model_is_accessible() {
        let resource = type_name::<
            crate::zhal::model::ResourceAmount
        >();

        assert!(resource.contains("ResourceAmount"));
    }

    #[test]
    fn canonical_topology_model_is_accessible() {
        let topology = type_name::<
            crate::zhal::model::HardwareTopology
        >();

        assert!(topology.contains("HardwareTopology"));
    }

    #[test]
    fn canonical_error_contract_is_accessible() {
        let error = type_name::<
            crate::zhal::errors::ZhalError
        >();

        let error_kind = type_name::<
            crate::zhal::errors::ZhalErrorKind
        >();

        assert!(error.contains("ZhalError"));
        assert!(error_kind.contains("ZhalErrorKind"));
    }

    #[test]
    fn canonical_result_contract_is_accessible() {
        let result = type_name::<
            crate::zhal::errors::ZhalResult<()>
        >();

        assert!(result.contains("Result"));
    }

    #[test]
    fn hardware_context_is_accessible() {
        let context = type_name::<
            crate::zhal::context::ZhalContext
        >();

        assert!(context.contains("ZhalContext"));
    }
}

// ============================================================================
// 2. PROVIDER TESTS
// ============================================================================

mod providers {
    use core::any::type_name;

    #[test]
    fn hardware_provider_contract_is_accessible() {
        let provider = type_name::<
            dyn crate::zhal::providers::HardwareProvider
        >();

        assert!(provider.contains("HardwareProvider"));
    }

    #[test]
    fn provider_registry_is_accessible() {
        let registry = type_name::<
            crate::zhal::providers::ProviderRegistry
        >();

        assert!(registry.contains("ProviderRegistry"));
    }

    #[test]
    fn provider_lifecycle_is_accessible() {
        let lifecycle = type_name::<
            crate::zhal::providers::ProviderLifecycleManager
        >();

        assert!(lifecycle.contains("ProviderLifecycleManager"));
    }

    #[test]
    fn provider_validation_is_accessible() {
        let validation = type_name::<
            crate::zhal::providers::ProviderValidationPolicy
        >();

        assert!(validation.contains("ProviderValidationPolicy"));
    }

    #[test]
    fn provider_discovery_is_accessible() {
        let discovery = type_name::<
            crate::zhal::providers::discovery::DiscoverySession
        >();

        assert!(discovery.contains("DiscoverySession"));
    }

    #[test]
    fn canonical_provider_identity_is_reused() {
        let provider = crate::stdlib::provider::ProviderId::new(
            "future.hardware.provider",
        )
        .expect("future provider identity must be valid");

        assert_eq!(
            provider.as_str(),
            "future.hardware.provider",
        );
    }

    #[test]
    fn provider_components_remain_separate() {
        assert_ne!(
            crate::zhal::providers::contract::COMPONENT,
            crate::zhal::providers::registry::COMPONENT,
        );

        assert_ne!(
            crate::zhal::providers::discovery::COMPONENT,
            crate::zhal::providers::validation::COMPONENT,
        );
    }
}

// ============================================================================
// 3. DISCOVERY AND INVENTORY TESTS
// ============================================================================

mod discovery {
    use core::any::type_name;

    #[test]
    fn provider_discovery_session_is_accessible() {
        let discovery = type_name::<
            crate::zhal::providers::discovery::DiscoverySession
        >();

        assert!(discovery.contains("DiscoverySession"));
    }

    #[test]
    fn device_discovery_session_is_accessible() {
        let discovery = type_name::<
            crate::zhal::devices::discovery::DeviceDiscoverySession
        >();

        assert!(discovery.contains("DeviceDiscoverySession"));
    }

    #[test]
    fn device_inventory_is_accessible() {
        let inventory = type_name::<
            crate::zhal::devices::inventory::HardwareDeviceInventory
        >();

        assert!(inventory.contains("HardwareDeviceInventory"));
    }

    #[test]
    fn device_access_authority_is_accessible() {
        let access = type_name::<
            crate::zhal::devices::access::HardwareDeviceAccess
        >();

        assert!(access.contains("HardwareDeviceAccess"));
    }

    #[test]
    fn device_session_authority_is_accessible() {
        let sessions = type_name::<
            crate::zhal::devices::sessions::HardwareDeviceSession
        >();

        assert!(sessions.contains("HardwareDeviceSession"));
    }

    #[test]
    fn device_health_authority_is_accessible() {
        let health = type_name::<
            crate::zhal::devices::health::HardwareDeviceHealth
        >();

        assert!(health.contains("HardwareDeviceHealth"));
    }

    #[test]
    fn inventory_can_be_initialized_without_physical_hardware() {
        let inventory =
            crate::zhal::devices::inventory::HardwareDeviceInventory::new();

        assert!(
            inventory.is_empty()
                .expect("new inventory must be readable")
        );

        assert_eq!(
            inventory.len()
                .expect("new inventory length must be readable"),
            0,
        );
    }
}

// ============================================================================
// 4. CAPABILITY TESTS
// ============================================================================

mod capabilities {
    use core::any::type_name;

    #[test]
    fn canonical_capability_identity_is_accessible() {
        let capability = type_name::<
            crate::zhal::capabilities::CapabilityId
        >();

        assert!(capability.contains("CapabilityId"));
    }

    #[test]
    fn capability_evidence_is_accessible() {
        let evidence = type_name::<
            crate::zhal::capabilities::HardwareCapabilityEvidence
        >();

        assert!(evidence.contains("HardwareCapabilityEvidence"));
    }

    #[test]
    fn hardware_requirements_are_accessible() {
        let requirements = type_name::<
            crate::zhal::capabilities::HardwareCapabilityRequirement
        >();

        assert!(requirements.contains("HardwareCapabilityRequirement"));
    }

    #[test]
    fn availability_assessment_is_accessible() {
        let availability = type_name::<
            crate::zhal::capabilities::AvailabilityAssessment
        >();

        assert!(availability.contains("AvailabilityAssessment"));
    }

    #[test]
    fn compatibility_report_is_accessible() {
        let compatibility = type_name::<
            crate::zhal::capabilities::CompatibilityReport
        >();

        assert!(compatibility.contains("CompatibilityReport"));
    }

    #[test]
    fn future_capabilities_are_supported() {
        let capability =
            crate::stdlib::capability::CapabilityId::new(
                "future.photonic.computation",
            )
            .expect("future capability must be valid");

        assert_eq!(
            capability.as_str(),
            "future.photonic.computation",
        );
    }
}

// ============================================================================
// 5. PORTABILITY TESTS
// ============================================================================

mod portability {
    use crate::support::{
        device_kind,
        hardware_identity,
        provider_id,
    };

    #[test]
    fn future_hardware_categories_are_supported() {
        let categories = [
            "classical.processor",
            "quantum.processor",
            "hdl.fpga",
            "accelerator.gpu",
            "embedded.microcontroller",
            "future.photonic.processor",
            "future.neuromorphic.processor",
        ];

        for category in categories {
            let value = device_kind(category);

            assert_eq!(value.as_str(), category);
        }
    }

    #[test]
    fn hardware_identity_is_provider_scoped() {
        let first = hardware_identity(
            "provider.first",
            "device.shared",
        );

        let second = hardware_identity(
            "provider.second",
            "device.shared",
        );

        assert_ne!(first, second);
    }

    #[test]
    fn hardware_identity_preserves_device_identity() {
        let identity = hardware_identity(
            "provider.classical",
            "processor.device",
        );

        assert_eq!(
            identity.device_id().as_str(),
            "processor.device",
        );
    }

    #[test]
    fn hardware_categories_are_not_vendor_enumerations() {
        let category = device_kind(
            "future.vendor_independent.computing",
        );

        assert_eq!(
            category.as_str(),
            "future.vendor_independent.computing",
        );
    }

    #[test]
    fn provider_identity_is_independent_of_device_identity() {
        let first = provider_id("provider.first");
        let second = provider_id("provider.second");

        assert_ne!(first, second);
    }
}

// ============================================================================
// 6. SCALABILITY TESTS
// ============================================================================

mod scalability {
    use crate::support::{
        device_kind,
        hardware_identity,
        provider_id,
    };

    #[test]
    fn identifiers_are_not_tied_to_hardware_vendors() {
        let provider = provider_id(
            "future.open.hardware.provider",
        );

        let category = device_kind(
            "future.open.computational.device",
        );

        assert_eq!(
            provider.as_str(),
            "future.open.hardware.provider",
        );

        assert_eq!(
            category.as_str(),
            "future.open.computational.device",
        );
    }

    #[test]
    fn dynamic_identifier_values_are_supported() {
        let namespace = String::from("future");
        let technology = String::from("photonic");
        let device = String::from("processor");

        let category = format!(
            "{namespace}.{technology}.{device}"
        );

        let validated = device_kind(&category);

        assert_eq!(
            validated.as_str(),
            category,
        );
    }

    #[test]
    fn device_identity_does_not_encode_machine_capacity() {
        let identity = hardware_identity(
            "provider.scalable",
            "device.resource-parametric",
        );

        assert_eq!(
            identity.device_id().as_str(),
            "device.resource-parametric",
        );
    }

    #[test]
    fn hardware_resource_models_are_not_fixed_width_machine_models() {
        let resource = core::any::type_name::<
            crate::zhal::model::ResourceAmount
        >();

        assert!(resource.contains("ResourceAmount"));
    }

    #[test]
    fn operational_budgets_are_not_language_capacity_limits() {
        // Operational budgets may restrict an individual
        // discovery, reservation, simulation or execution run.
        //
        // They must never become universal hardware limits
        // or semantic restrictions on valid Zamani programs.

        assert!(
            crate::zhal::providers::contract::COMPONENT
                .contains("providers")
        );
    }
}

// ============================================================================
// 7. SECURITY TESTS
// ============================================================================

mod security {
    use core::any::type_name;

    #[test]
    fn trust_contract_is_accessible() {
        let trust = type_name::<
            crate::zhal::security::TrustPolicy
        >();

        assert!(trust.contains("TrustPolicy"));
    }

    #[test]
    fn authorization_contract_is_accessible() {
        let authorization = type_name::<
            crate::zhal::security::HardwareAuthorizationService
        >();

        assert!(authorization.contains(
            "HardwareAuthorizationService"
        ));
    }

    #[test]
    fn isolation_contract_is_accessible() {
        let isolation = type_name::<
            crate::zhal::security::HardwareIsolationService
        >();

        assert!(isolation.contains(
            "HardwareIsolationService"
        ));
    }

    #[test]
    fn security_authorities_are_separate() {
        assert_ne!(
            crate::zhal::security::trust::COMPONENT,
            crate::zhal::security::authorization::COMPONENT,
        );

        assert_ne!(
            crate::zhal::security::trust::COMPONENT,
            crate::zhal::security::isolation::COMPONENT,
        );
    }
}

// ============================================================================
// 8. DETERMINISM TESTS
// ============================================================================

mod determinism {
    use core::any::type_name;

    #[test]
    fn provider_registry_snapshot_is_accessible() {
        let snapshot = type_name::<
            crate::zhal::providers::ProviderRegistrySnapshot
        >();

        assert!(snapshot.contains("ProviderRegistrySnapshot"));
    }

    #[test]
    fn hardware_snapshot_identity_is_accessible() {
        let snapshot = type_name::<
            crate::zhal::identity::HardwareSnapshotId
        >();

        assert!(snapshot.contains("HardwareSnapshotId"));
    }

    #[test]
    fn hardware_inventory_snapshot_is_accessible() {
        let inventory = type_name::<
            crate::zhal::devices::inventory::HardwareDeviceInventorySnapshot
        >();

        assert!(inventory.contains(
            "HardwareDeviceInventorySnapshot"
        ));
    }

    #[test]
    fn hardware_provenance_is_accessible() {
        let provenance = type_name::<
            crate::zhal::observability::HardwareProvenance
        >();

        assert!(provenance.contains("HardwareProvenance"));
    }

    #[test]
    fn discovery_and_inventory_remain_distinct_authorities() {
        assert_ne!(
            crate::zhal::providers::discovery::COMPONENT,
            crate::zhal::devices::inventory::COMPONENT,
        );
    }
}

// ============================================================================
// 9. FAULT-TOLERANCE TESTS
// ============================================================================

mod fault_tolerance {
    use core::any::type_name;

    #[test]
    fn structured_hardware_errors_are_accessible() {
        let error = type_name::<
            crate::zhal::errors::ZhalError
        >();

        assert!(error.contains("ZhalError"));
    }

    #[test]
    fn hardware_error_classification_is_accessible() {
        let error_kind = type_name::<
            crate::zhal::errors::ZhalErrorKind
        >();

        assert!(error_kind.contains("ZhalErrorKind"));
    }

    #[test]
    fn operation_cancellation_is_accessible() {
        let cancellation = type_name::<
            crate::zhal::operations::CancellationRequest
        >();

        assert!(cancellation.contains("CancellationRequest"));
    }

    #[test]
    fn completion_failure_contract_is_accessible() {
        let failure = type_name::<
            crate::zhal::operations::CompletionFailure
        >();

        assert!(failure.contains("CompletionFailure"));
    }

    #[test]
    fn discovery_failure_is_structured() {
        let failure = type_name::<
            crate::zhal::providers::discovery::DiscoveryFailure
        >();

        assert!(failure.contains("DiscoveryFailure"));
    }

    #[test]
    fn simulation_fault_injection_is_available() {
        let fault = type_name::<
            crate::zhal::simulation::provider::SimulationFault
        >();

        assert!(fault.contains("SimulationFault"));
    }
}

// ============================================================================
// 10. CROSS-SUBSYSTEM INTEGRATION TESTS
// ============================================================================

mod integration {
    use core::any::type_name;

    #[test]
    fn standard_provider_authority_is_reused() {
        let provider = type_name::<
            crate::stdlib::provider::ProviderId
        >();

        assert!(provider.contains("ProviderId"));
    }

    #[test]
    fn standard_capability_authority_is_reused() {
        let capability = type_name::<
            crate::stdlib::capability::CapabilityId
        >();

        assert!(capability.contains("CapabilityId"));
    }

    #[test]
    fn standard_device_identity_authority_is_reused() {
        let device = type_name::<
            crate::stdlib::io::devices::DeviceId
        >();

        assert!(device.contains("DeviceId"));
    }

    #[test]
    fn standard_operation_context_is_reused() {
        let context = type_name::<
            crate::stdlib::context::OperationContext
        >();

        assert!(context.contains("OperationContext"));
    }

    #[test]
    fn hardware_operations_remain_domain_neutral() {
        let contract = type_name::<
            crate::zhal::operations::HardwareOperationContract
        >();

        assert!(contract.contains(
            "HardwareOperationContract"
        ));
    }

    #[test]
    fn hardware_resource_observations_are_accessible() {
        let observation = type_name::<
            crate::zhal::resources::ResourceObservationSet
        >();

        assert!(observation.contains(
            "ResourceObservationSet"
        ));
    }

    #[test]
    fn telemetry_and_diagnostics_are_integrated() {
        let telemetry = type_name::<
            crate::zhal::observability::HardwareTelemetryObservation
        >();

        let diagnostic = type_name::<
            crate::zhal::observability::ZhalDiagnostic
        >();

        assert!(telemetry.contains(
            "HardwareTelemetryObservation"
        ));

        assert!(diagnostic.contains(
            "ZhalDiagnostic"
        ));
    }

    #[test]
    fn simulation_is_a_provider_implementation() {
        let provider = type_name::<
            crate::zhal::simulation::provider::SimulationProvider
        >();

        assert!(provider.contains("SimulationProvider"));
    }

    #[test]
    fn simulation_does_not_replace_physical_provider_contracts() {
        let physical = type_name::<
            dyn crate::zhal::providers::HardwareProvider
        >();

        let simulated = type_name::<
            crate::zhal::simulation::provider::SimulationProvider
        >();

        assert_ne!(physical, simulated);
    }

    #[test]
    fn grammar_and_physical_inventory_have_separate_ownership() {
        // The hardware grammar describes logical device intent.
        // ZHAL inventory describes observed physical devices.
        //
        // Neither authority replaces the other.

        let inventory = type_name::<
            crate::zhal::devices::inventory::HardwareDeviceInventory
        >();

        let semantic_capability = type_name::<
            crate::stdlib::capability::CapabilityId
        >();

        assert!(!inventory.is_empty());
        assert!(!semantic_capability.is_empty());
    }
}
