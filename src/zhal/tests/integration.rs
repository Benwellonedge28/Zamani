
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Cross-subsystem integration tests
//!
//! Path: src/zhal/tests/integration.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verifies integration between the existing ZHAL hardware contracts,
//! provider registry, simulation provider, device discovery and
//! canonical hardware-device inventory.
//!
//! These tests verify actual ZHAL contracts. They do not introduce
//! competing provider registries, device inventories, resource
//! allocators, execution engines or domain-specific hardware semantics.
//!
//! # Architectural boundaries
//!
//! - Provider identity: stdlib::provider.
//! - Provider capabilities: stdlib::capability.
//! - Provider implementation: zhal::providers.
//! - Hardware identity: zhal::identity.
//! - Hardware description: zhal::model::device.
//! - Hardware discovery: zhal::devices::discovery.
//! - Hardware inventory: zhal::devices::inventory.
//! - Simulation: zhal::simulation.
//! - Runtime allocation: ZUR.
//! - Quantum execution: quantum::hardware.
//! - HDL realization: hdl::realization.
//! - Distributed execution: distributed.
//! - Hybrid execution: hybrid.
//!
//! # POCO-REAF
//!
//! Tests must not introduce universal hardware-capacity limits.
//!
//! Operational test fixture sizes are test inputs, not architectural
//! capacity restrictions.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Grammar boundary
//!
//! The grammar describes logical computational intent.
//!
//! ZHAL describes observed physical hardware.
//!
//! These tests must never equate a logical hardware declaration
//! with a discovered physical device.
//!
//! # Safety
//!
//! Unsafe Rust is prohibited.

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use std::sync::Arc;
    use std::time::Duration;

    use crate::stdlib::capability::CapabilitySnapshot;

    use crate::stdlib::io::devices::{
        DeviceId,
        DeviceKindId,
    };

    use crate::stdlib::provider::{
        Provider,
        ProviderDescriptor,
        ProviderId,
        ProviderKind,
        ProviderStatus,
    };

    use crate::stdlib::version::Version;

    use crate::zhal::devices::discovery::{
        DeviceDiscoveryOptions,
        DeviceDiscoverySession,
    };

    use crate::zhal::devices::inventory::{
        HardwareDeviceInventory,
    };

    use crate::zhal::identity::{
        HardwareDeviceId,
        HardwareDeviceIdentity,
    };

    use crate::zhal::model::device::{
        DeviceAvailability,
        HardwareDeviceDescriptor,
    };

    use crate::zhal::providers::contract::{
        HardwareProvider,
    };

    use crate::zhal::providers::discovery::{
        DiscoveryOptions,
        DiscoverySession,
    };

    use crate::zhal::providers::registry::{
        ProviderRegistry,
        RegistryGeneration,
    };

    use crate::zhal::simulation::{
        SimulatedDevice,
        SimulationProvider,
        SimulationProviderConfig,
    };

    // ========================================================================
    // TEST FIXTURES
    // ========================================================================

    /// Creates a canonical provider identity.
    ///
    /// Provider identity remains owned by stdlib::provider.
    fn provider_id() -> ProviderId {
        ProviderId::new("integration.simulation")
            .expect("valid provider identity")
    }

    /// Creates a canonical hardware-device identity.
    ///
    /// Hardware identity is scoped by its provider.
    fn device_identity(
        provider: ProviderId,
        device: &str,
    ) -> HardwareDeviceIdentity {
        let device_id: HardwareDeviceId =
            DeviceId::new(device)
                .expect("valid device identity");

        HardwareDeviceIdentity::new(
            provider,
            device_id,
        )
    }

    /// Constructs a validated hardware-device descriptor.
    ///
    /// New descriptors retain unknown availability unless
    /// an explicit observation is supplied.
    fn device_descriptor(
        provider: ProviderId,
        device: &str,
    ) -> HardwareDeviceDescriptor {
        let identity = device_identity(
            provider,
            device,
        );

        HardwareDeviceDescriptor::new(
            identity,
            DeviceKindId::new("integration.test_device")
                .expect("valid open-world device category"),
            format!("Integration device {device}"),
        )
        .expect("valid hardware-device descriptor")
    }

    /// Constructs a canonical provider descriptor.
    ///
    /// Both implementation and provider API versions are
    /// supplied explicitly.
    fn provider_descriptor() -> ProviderDescriptor {
        ProviderDescriptor::new(
            provider_id(),
            ProviderKind::new("integration.simulation")
                .expect("valid provider kind"),
            "ZHAL integration simulation provider",
            Version::parse("1.0.0")
                .expect("valid implementation version"),
            Version::parse("1.0.0")
                .expect("valid provider API version"),
        )
        .expect("valid provider descriptor")
    }

    /// Creates an independent simulation-provider implementation.
    ///
    /// The provider begins stopped.
    ///
    /// Construction does not initialize a provider, discover
    /// physical hardware or claim that simulated hardware exists.
    fn simulation_provider() -> Arc<SimulationProvider> {
        let descriptor = provider_descriptor();

        let config = SimulationProviderConfig::new(
            descriptor,
            CapabilitySnapshot::new(),
            Vec::<SimulatedDevice>::new(),
            NonZeroUsize::new(1)
                .expect("nonzero operational page size"),
            Duration::ZERO,
        );

        Arc::new(
            SimulationProvider::new(config)
                .expect("valid simulation-provider configuration"),
        )
    }

    // ========================================================================
    // PROVIDER REGISTRY INTEGRATION
    // ========================================================================

    /// An independent provider registry begins empty.
    ///
    /// No global registry or hidden application state is required.
    #[test]
    fn provider_registry_starts_empty() {
        let registry = ProviderRegistry::new();

        let snapshot = registry
            .snapshot()
            .expect("valid registry snapshot");

        assert!(snapshot.is_empty());

        assert_eq!(
            snapshot.len(),
            0,
        );

        assert_eq!(
            snapshot.generation(),
            RegistryGeneration::INITIAL,
        );

        registry
            .verify_integrity()
            .expect("empty registry is structurally valid");
    }

    /// Registers a simulated provider through the existing
    /// standard-library Provider contract.
    ///
    /// Registration must not imply provider initialization,
    /// hardware availability or execution readiness.
    #[test]
    fn simulation_provider_integrates_with_registry() {
        let registry = ProviderRegistry::new();

        let provider = simulation_provider();

        assert_eq!(
            provider.status(),
            ProviderStatus::Stopped,
        );

        let implementation: Arc<dyn Provider> =
            provider.clone();

        let generation = registry
            .register(implementation)
            .expect("provider registration succeeds");

        assert_eq!(
            generation.get(),
            1,
        );

        let snapshot = registry
            .snapshot()
            .expect("provider snapshot succeeds");

        assert_eq!(
            snapshot.len(),
            1,
        );

        assert!(
            snapshot.get(&provider_id()).is_some(),
            "registered provider must be discoverable by canonical identity",
        );

        assert_eq!(
            snapshot.generation(),
            generation,
        );

        registry
            .verify_integrity()
            .expect("registered provider identity remains consistent");

        // Registration is not initialization.
        assert_eq!(
            provider.status(),
            ProviderStatus::Stopped,
        );
    }

    /// Provider snapshots remain independently readable.
    ///
    /// Later registry changes must not silently mutate
    /// previously captured metadata snapshots.
    #[test]
    fn provider_registry_snapshot_is_immutable() {
        let registry = ProviderRegistry::new();

        let original = registry
            .snapshot()
            .expect("initial snapshot succeeds");

        let provider = simulation_provider();

        let implementation: Arc<dyn Provider> =
            provider;

        registry
            .register(implementation)
            .expect("registration succeeds");

        let updated = registry
            .snapshot()
            .expect("updated snapshot succeeds");

        assert_eq!(original.len(), 0);
        assert_eq!(updated.len(), 1);

        assert!(
            updated.generation() > original.generation(),
        );
    }

    // ========================================================================
    // HARDWARE IDENTITY AND MODEL INTEGRATION
    // ========================================================================

    /// Hardware identity must retain its canonical provider scope.
    ///
    /// Two providers may expose the same provider-defined device
    /// identifier without creating an identity collision.
    #[test]
    fn hardware_identity_is_provider_scoped() {
        let first_provider = ProviderId::new(
            "integration.first",
        )
        .expect("valid provider");

        let second_provider = ProviderId::new(
            "integration.second",
        )
        .expect("valid provider");

        let first = device_identity(
            first_provider,
            "device-1",
        );

        let second = device_identity(
            second_provider,
            "device-1",
        );

        assert_ne!(
            first,
            second,
            "device identity must include provider identity",
        );
    }

    /// Device descriptions must not fabricate availability.
    ///
    /// The default state must remain unknown until a provider
    /// supplies an actual availability observation.
    #[test]
    fn hardware_model_preserves_unknown_availability() {
        let descriptor = device_descriptor(
            provider_id(),
            "unknown-device",
        );

        assert_eq!(
            descriptor.availability().state(),
            DeviceAvailability::unknown().state(),
        );

        assert!(
            !descriptor.availability().is_available(),
            "unknown availability must not imply availability",
        );

        descriptor
            .validate()
            .expect("device descriptor is valid");
    }

    /// Explicit availability observations must survive model
    /// construction and validation.
    #[test]
    fn hardware_model_preserves_explicit_availability() {
        let descriptor = device_descriptor(
            provider_id(),
            "available-device",
        )
        .with_availability(
            DeviceAvailability::available(),
        )
        .expect("valid availability observation");

        assert!(
            descriptor.availability().is_available(),
        );

        descriptor
            .validate()
            .expect("available device descriptor is valid");
    }

    // ========================================================================
    // DEVICE INVENTORY INTEGRATION
    // ========================================================================

    /// The canonical device inventory starts empty.
    #[test]
    fn hardware_inventory_starts_empty() {
        let inventory = HardwareDeviceInventory::new();

        assert!(
            inventory.is_empty()
                .expect("inventory observation succeeds"),
        );

        assert_eq!(
            inventory.len().expect("inventory length succeeds"),
            0,
        );

        assert_eq!(
            inventory.generation()
                .expect("inventory generation succeeds"),
            0,
        );
    }

    /// A complete provider snapshot must be published atomically.
    ///
    /// Inventory must retain the canonical device descriptor
    /// and provider-scoped identity.
    #[test]
    fn inventory_accepts_complete_provider_snapshot() {
        let inventory = HardwareDeviceInventory::new();

        let provider = provider_id();

        let descriptor = device_descriptor(
            provider.clone(),
            "device-a",
        );

        let identity = descriptor.identity().clone();

        inventory
            .replace_provider_snapshot(
                provider.clone(),
                vec![descriptor],
            )
            .expect("valid provider snapshot publication");

        assert_eq!(
            inventory.len().expect("inventory length succeeds"),
            1,
        );

        assert!(
            inventory.get(&identity)
                .expect("device lookup succeeds")
                .is_some(),
        );

        assert_eq!(
            inventory.provider_count()
                .expect("provider count succeeds"),
            1,
        );

        assert!(
            inventory.generation()
                .expect("generation succeeds") > 0,
        );
    }

    /// A provider's failed snapshot replacement must not destroy
    /// previously published inventory.
    ///
    /// This verifies transactional publication and provider isolation.
    #[test]
    fn invalid_provider_snapshot_preserves_existing_inventory() {
        let inventory = HardwareDeviceInventory::new();

        let provider = provider_id();

        let existing = device_descriptor(
            provider.clone(),
            "existing-device",
        );

        let existing_identity = existing.identity().clone();

        inventory
            .replace_provider_snapshot(
                provider.clone(),
                vec![existing],
            )
            .expect("initial publication succeeds");

        let previous_generation = inventory
            .generation()
            .expect("generation succeeds");

        let foreign_provider = ProviderId::new(
            "integration.foreign",
        )
        .expect("valid foreign provider identity");

        let foreign_device = device_descriptor(
            foreign_provider,
            "foreign-device",
        );

        let result = inventory.replace_provider_snapshot(
            provider,
            vec![foreign_device],
        );

        assert!(
            result.is_err(),
            "foreign-provider devices must be rejected",
        );

        assert_eq!(
            inventory.len().expect("inventory length succeeds"),
            1,
        );

        assert!(
            inventory.get(&existing_identity)
                .expect("existing device lookup succeeds")
                .is_some(),
            "previously published device must remain available",
        );

        assert_eq!(
            inventory.generation()
                .expect("generation succeeds"),
            previous_generation,
            "failed publication must not advance inventory generation",
        );
    }

    /// An aborted refresh must preserve the previously published
    /// provider inventory.
    ///
    /// Beginning a refresh does not replace existing observations.
    #[test]
    fn aborted_provider_refresh_preserves_published_inventory() {
        let inventory = HardwareDeviceInventory::new();

        let provider = provider_id();

        let existing = device_descriptor(
            provider.clone(),
            "stable-device",
        );

        let identity = existing.identity().clone();

        inventory
            .replace_provider_snapshot(
                provider.clone(),
                vec![existing],
            )
            .expect("initial publication succeeds");

        let previous_generation = inventory
            .generation()
            .expect("generation succeeds");

        let refresh = inventory
            .begin_provider_refresh(provider)
            .expect("provider refresh begins");

        assert_eq!(
            inventory.pending_refresh_count()
                .expect("pending refresh count succeeds"),
            1,
        );

        // The old published snapshot remains visible.
        assert!(
            inventory.get(&identity)
                .expect("device lookup succeeds")
                .is_some(),
        );

        inventory
            .abort_provider_refresh(&refresh)
            .expect("refresh abort succeeds");

        assert_eq!(
            inventory.pending_refresh_count()
                .expect("pending refresh count succeeds"),
            0,
        );

        assert_eq!(
            inventory.len().expect("inventory length succeeds"),
            1,
        );

        assert_eq!(
            inventory.generation()
                .expect("generation succeeds"),
            previous_generation,
        );

        assert!(
            inventory.get(&identity)
                .expect("device lookup succeeds")
                .is_some(),
        );
    }

    // ========================================================================
    // DISCOVERY INTEGRATION
    // ========================================================================

    /// Empty provider discovery must complete successfully.
    ///
    /// An environment with no registered providers is a valid
    /// observation, not an internal failure.
    #[test]
    fn empty_provider_discovery_completes() {
        let session = DiscoverySession::new(
            Vec::<Arc<dyn HardwareProvider>>::new(),
            DiscoveryOptions::new(),
        )
        .expect("empty discovery session is valid");

        assert!(
            session.is_complete(),
            "discovery with no providers must be complete",
        );

        assert!(
            !session.is_terminated(),
            "empty discovery must not be treated as failure",
        );

        assert_eq!(
            session.report().provider_count(),
            0,
        );
    }

    /// Device discovery must compose correctly with the
    /// provider-discovery authority.
    ///
    /// It must not require a physical device or provider.
    #[test]
    fn empty_device_discovery_completes() {
        let session = DeviceDiscoverySession::new(
            Vec::<Arc<dyn HardwareProvider>>::new(),
            DeviceDiscoveryOptions::default(),
        )
        .expect("empty device discovery session is valid");

        assert!(
            session.is_complete(),
        );

        assert!(
            !session.has_failures(),
        );

        assert!(
            !session.is_terminated(),
        );
    }

    /// An empty inventory and an empty discovery result must
    /// remain distinct, independently owned observations.
    #[test]
    fn discovery_does_not_implicitly_publish_inventory() {
        let inventory = HardwareDeviceInventory::new();

        let session = DiscoverySession::new(
            Vec::<Arc<dyn HardwareProvider>>::new(),
            DiscoveryOptions::new(),
        )
        .expect("empty discovery session is valid");

        assert!(session.is_complete());

        assert!(
            inventory.is_empty()
                .expect("inventory query succeeds"),
        );

        assert_eq!(
            inventory.generation()
                .expect("generation succeeds"),
            0,
        );
    }

    // ========================================================================
    // PROVIDER/DISCOVERY TYPE INTEGRATION
    // ========================================================================

    /// The same simulation provider must satisfy both the canonical
    /// standard-library provider contract and the specialized
    /// ZHAL hardware-provider contract.
    #[test]
    fn simulation_provider_implements_both_provider_boundaries() {
        let provider = simulation_provider();

        let _: &dyn Provider = provider.as_ref();

        let _: &dyn HardwareProvider = provider.as_ref();

        assert_eq!(
            provider.provider_descriptor().id(),
            &provider_id(),
        );
    }

    /// Discovery configuration must remain resource-parametric.
    ///
    /// The page size is optional and is an operational request,
    /// not a universal hardware-capacity limit.
    #[test]
    fn discovery_options_do_not_impose_global_page_limits() {
        let default_options = DiscoveryOptions::new();

        assert!(
            default_options.page_size().is_none(),
        );

        let configured_size = NonZeroUsize::new(257)
            .expect("nonzero page size");

        let configured = default_options
            .with_page_size(Some(configured_size));

        assert_eq!(
            configured.page_size(),
            Some(configured_size),
        );
    }

    // ========================================================================
    // OPEN-WORLD IDENTITY INTEGRATION
    // ========================================================================

    /// Provider and device identifiers must not be restricted
    /// to a small fixed identifier length.
    ///
    /// This verifies that identity construction delegates to
    /// canonical open-world identifier contracts.
    #[test]
    fn long_provider_and_device_identities_are_supported() {
        let provider_value = format!(
            "integration.{}",
            "p".repeat(4096),
        );

        let device_value = format!(
            "device.{}",
            "d".repeat(4096),
        );

        let provider = ProviderId::new(
            provider_value.clone(),
        )
        .expect("long provider identity is valid");

        let device = DeviceId::new(
            device_value.clone(),
        )
        .expect("long device identity is valid");

        assert_eq!(
            provider.as_str(),
            provider_value,
        );

        assert_eq!(
            device.as_str(),
            device_value,
        );
    }

    // ========================================================================
    // CONCURRENT INVENTORY ACCESS
    // ========================================================================

    /// Cloned inventory handles must refer to the same canonical
    /// underlying inventory.
    ///
    /// This verifies the documented shared-state clone semantics.
    #[test]
    fn cloned_inventory_handles_share_published_state() {
        let original = HardwareDeviceInventory::new();

        let observer = original.clone();

        let provider = provider_id();

        let descriptor = device_descriptor(
            provider.clone(),
            "shared-device",
        );

        let identity = descriptor.identity().clone();

        original
            .replace_provider_snapshot(
                provider,
                vec![descriptor],
            )
            .expect("publication succeeds");

        assert_eq!(
            observer.len().expect("observer length succeeds"),
            1,
        );

        assert!(
            observer.get(&identity)
                .expect("observer lookup succeeds")
                .is_some(),
        );

        assert_eq!(
            original.generation().expect("generation succeeds"),
            observer.generation().expect("generation succeeds"),
        );
    }
}
