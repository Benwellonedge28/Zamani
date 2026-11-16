
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Portability Tests
//!
//! Path: src/zhal/tests/portability.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify that the canonical ZHAL hardware abstraction contracts
//! preserve hardware portability across heterogeneous providers,
//! devices, computational domains and changing hardware inventories.
//!
//! These tests exercise the existing ZHAL implementation rather
//! than introducing a second hardware abstraction architecture.
//!
//! # Architectural authorities
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Device identity:
//!     crate::stdlib::io::devices
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Hardware provider contract:
//!     crate::zhal::providers::contract
//!
//! Hardware device model:
//!     crate::zhal::model::device
//!
//! Hardware discovery:
//!     crate::zhal::providers::discovery
//!
//! Capability authority:
//!     crate::stdlib::capability
//!
//! Hardware capability compatibility:
//!     crate::zhal::capabilities
//!
//! Simulation:
//!     crate::zhal::simulation::provider
//!
//! Domain-specific authorities:
//!     crate::classical
//!     crate::quantum::hardware
//!     crate::hdl
//!     crate::ai
//!     crate::distributed
//!     crate::hybrid
//!
//! Runtime orchestration:
//!     crate::zur
//!
//! Source-language semantics:
//!     crate::frontend::semantic
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The tests establish that generic hardware discovery and identity
//! contracts do not require a particular:
//!
//! - Hardware vendor.
//! - Processor architecture.
//! - Operating system.
//! - Computational domain.
//! - Hardware category.
//! - Physical device identity.
//! - Provider implementation.
//! - Device inventory size.
//! - Hardware deployment topology.
//!
//! They verify that multiple providers can expose hardware through
//! the same canonical ZHAL interfaces.
//!
//! These tests do not claim that every workload is executable on
//! every device. Semantic requirements, capability evidence,
//! compatibility, resource feasibility and domain-specific
//! execution contracts remain authoritative.
//!
//! # Scalability
//!
//! No universal maximum is introduced for:
//!
//! - Hardware providers.
//! - Hardware devices.
//! - Discovery pages.
//! - Hardware categories.
//! - Hardware interfaces.
//! - Device identifiers.
//! - Computational domains.
//! - Hardware technologies.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Finite test workloads are verification samples, not hardware
//! capacity limits.
//!
//! Actual scalability is governed by available resources,
//! representational limits, provider implementations and
//! explicitly configured operational budgets.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! # Important boundaries
//!
//! These tests do not implement:
//!
//! - Source-language semantic analysis.
//! - Compiler target selection.
//! - Backend selection.
//! - Hardware scheduling.
//! - Quantum routing.
//! - HDL synthesis.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Distributed transport.
//! - Hybrid execution.
//! - Hardware drivers.
//! - Hardware instruction execution.
//!
//! Discovery is observational.
//!
//! Device identity is not authorization.
//!
//! Capability availability is not resource reservation.
//!
//! Provider readiness is not proof that every device is available.
//!
//! Successful simulation is not physical hardware execution.

use std::collections::BTreeSet;
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

use crate::stdlib::capability::CapabilitySnapshot;

use crate::stdlib::context::OperationContext;

use crate::stdlib::io::devices::{
    DeviceId,
    DeviceKindId,
};

use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderId,
    ProviderKind,
    ProviderLifecycle,
};

use crate::stdlib::version::Version;

use crate::zhal::context::ZhalContext;

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::model::device::{
    DeviceAvailability,
    HardwareDeviceDescriptor,
};

use crate::zhal::providers::contract::{
    DeviceDiscoveryRequest,
    HardwareProvider,
};

use crate::zhal::simulation::provider::{
    SimulatedDevice,
    SimulationProvider,
    SimulationProviderConfig,
};

// ============================================================================
// TEST EXECUTION SUPPORT
// ============================================================================

/// Safe no-op waker for immediately completing test futures.
///
/// The simulation provider performs its test operations without
/// requiring an external asynchronous executor.
struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {}
}

/// Runs a future to completion using safe standard-library APIs.
///
/// This executor is intentionally limited to test execution.
/// It is not a production runtime or an asynchronous scheduling
/// implementation.
///
/// A pending future yields the current thread before being polled
/// again. Production asynchronous scheduling remains owned by ZUR.
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Pin::as_mut(&mut future).poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

// ============================================================================
// CANONICAL TEST CONTEXT
// ============================================================================

/// Creates an independent hardware operation context.
fn hardware_context() -> ZhalContext {
    ZhalContext::builder()
        .with_component("zhal.tests.portability")
        .expect("valid test component")
        .with_operation("hardware.portability")
        .expect("valid test operation")
        .build()
}

/// Creates a valid standard-library provider identity.
fn provider_id(value: &str) -> ProviderId {
    ProviderId::new(value)
        .expect("test provider identity must be valid")
}

/// Creates a valid canonical hardware device identity.
fn device_identity(
    provider: &str,
    device: &str,
) -> HardwareDeviceIdentity {
    HardwareDeviceIdentity::new(
        provider_id(provider),
        DeviceId::new(device)
            .expect("test device identity must be valid"),
    )
}

/// Creates a canonical provider descriptor.
fn provider_descriptor(
    identity: &str,
) -> ProviderDescriptor {
    ProviderDescriptor::new(
        provider_id(identity),
        ProviderKind::new("hardware.provider")
            .expect("valid open-world provider kind"),
        format!("ZHAL portability provider {identity}"),
        Version::parse("1.0.0")
            .expect("valid implementation version"),
        Version::parse("1.0.0")
            .expect("valid provider API version"),
    )
    .expect("valid provider descriptor")
}

// ============================================================================
// CANONICAL DEVICE CONSTRUCTION
// ============================================================================

/// Creates a canonical hardware descriptor.
///
/// The category is deliberately open-ended.
///
/// It may identify an existing or future computational technology.
fn device_descriptor(
    provider: &str,
    local_identity: &str,
    category: &str,
) -> HardwareDeviceDescriptor {
    let identity = device_identity(
        provider,
        local_identity,
    );

    HardwareDeviceDescriptor::new(
        identity,
        DeviceKindId::new(category)
            .expect("valid open-world hardware category"),
        format!("Portability test device {local_identity}"),
    )
    .expect("valid hardware device descriptor")
    .with_availability(DeviceAvailability::available())
    .expect("valid availability observation")
}

/// Wraps an existing canonical hardware descriptor in the
/// repository's actual simulated-device implementation.
fn simulated_device(
    provider: &str,
    local_identity: &str,
    category: &str,
) -> SimulatedDevice {
    SimulatedDevice::new(
        device_descriptor(
            provider,
            local_identity,
            category,
        ),
    )
}

// ============================================================================
// SIMULATION PROVIDER CONSTRUCTION
// ============================================================================

/// Constructs a simulation provider using the canonical ZHAL
/// provider implementation.
///
/// The provider is initialized through the existing
/// standard-library ProviderLifecycle contract.
///
/// No alternate provider abstraction is introduced.
fn initialized_provider(
    identity: &str,
    devices: impl IntoIterator<Item = SimulatedDevice>,
    page_size: NonZeroUsize,
) -> SimulationProvider {
    let descriptor = provider_descriptor(identity);

    let config = SimulationProviderConfig::new(
        descriptor,
        CapabilitySnapshot::new(),
        devices,
        page_size,
        Duration::ZERO,
    );

    let provider = SimulationProvider::new(config)
        .expect("simulation provider configuration must be valid");

    let context = hardware_context();

    block_on(
        provider.initialize(context.operation_context()),
    )
    .expect("simulation provider initialization must succeed");

    provider
}

/// Discovers the initial page through the canonical validated
/// HardwareProvider discovery interface.
fn discover(
    provider: &dyn HardwareProvider,
    context: &ZhalContext,
) -> crate::zhal::providers::contract::DeviceDiscoveryPage {
    block_on(
        provider.discover_devices(
            &DeviceDiscoveryRequest::initial(),
            context,
        ),
    )
    .expect("hardware discovery must succeed")
}

// ============================================================================
// 1. PROVIDER-INDEPENDENT HARDWARE IDENTITIES
// ============================================================================

#[test]
fn device_identity_is_scoped_by_provider() {
    let first = device_identity(
        "classical.provider",
        "device-001",
    );

    let second = device_identity(
        "accelerator.provider",
        "device-001",
    );

    // The local device identity is the same.
    assert_eq!(
        first.device_id(),
        second.device_id(),
    );

    // The provider identities differ.
    assert_ne!(
        first.provider_id(),
        second.provider_id(),
    );

    // The composite hardware identities must remain distinct.
    assert_ne!(first, second);
}

#[test]
fn hardware_identity_is_independent_of_device_category() {
    let first = device_descriptor(
        "portable.provider",
        "device-001",
        "classical.cpu",
    );

    let second = device_descriptor(
        "portable.provider",
        "device-001",
        "future.photonic.processor",
    );

    assert_eq!(
        first.identity(),
        second.identity(),
    );

    // A category change does not silently manufacture a new
    // physical device identity.
}

#[test]
fn provider_identity_is_extensible_across_domains() {
    let identities = [
        "classical.cpu",
        "quantum.qpu",
        "hdl.fpga",
        "accelerator.gpu",
        "distributed.remote",
        "embedded.microcontroller",
        "future.photonic.provider",
        "future.neuromorphic.provider",
        "future.molecular.provider",
    ];

    for identity in identities {
        let id = provider_id(identity);

        assert_eq!(
            id.as_str(),
            identity,
        );
    }
}

// ============================================================================
// 2. OPEN-WORLD HARDWARE CATEGORIES
// ============================================================================

#[test]
fn heterogeneous_hardware_categories_are_supported() {
    let categories = [
        "classical.cpu",
        "classical.memory",
        "quantum.qpu",
        "quantum.simulator",
        "hdl.fpga",
        "hdl.asic",
        "accelerator.gpu",
        "accelerator.tpu",
        "accelerator.npu",
        "embedded.microcontroller",
        "embedded.sensor",
        "embedded.actuator",
        "distributed.remote",
        "future.photonic.processor",
        "future.neuromorphic.processor",
        "future.analog.processor",
        "future.optical.processor",
    ];

    for (index, category) in categories.iter().enumerate() {
        let local_identity = format!("device-{index}");

        let descriptor = device_descriptor(
            "heterogeneous.provider",
            &local_identity,
            category,
        );

        assert_eq!(
            descriptor.identity().provider_id().as_str(),
            "heterogeneous.provider",
        );

        assert_eq!(
            descriptor.identity().device_id().as_str(),
            local_identity,
        );

        assert!(
            descriptor.validate().is_ok(),
            "valid hardware category was rejected: {category}",
        );
    }
}

#[test]
fn future_hardware_categories_do_not_require_core_changes() {
    let categories = [
        "future.photonic.quantum.processor",
        "future.neuromorphic.accelerator",
        "future.molecular.computing.system",
        "future.optical.interconnect",
        "future.analog.in_memory.processor",
        "future.reconfigurable.computing.device",
    ];

    for category in categories {
        let descriptor = device_descriptor(
            "future.hardware.provider",
            "future-device",
            category,
        );

        assert!(
            descriptor.validate().is_ok(),
            "future hardware category was rejected: {category}",
        );
    }
}

// ============================================================================
// 3. HETEROGENEOUS PROVIDER PORTABILITY
// ============================================================================

#[test]
fn different_hardware_providers_use_the_same_discovery_contract() {
    let scenarios = [
        (
            "classical.provider",
            "classical.cpu",
        ),
        (
            "quantum.provider",
            "quantum.qpu",
        ),
        (
            "hdl.provider",
            "hdl.fpga",
        ),
        (
            "accelerator.provider",
            "accelerator.gpu",
        ),
        (
            "distributed.provider",
            "distributed.remote",
        ),
        (
            "embedded.provider",
            "embedded.microcontroller",
        ),
        (
            "future.provider",
            "future.photonic.processor",
        ),
    ];

    for (provider_name, category) in scenarios {
        let provider = initialized_provider(
            provider_name,
            [
                simulated_device(
                    provider_name,
                    "device-001",
                    category,
                ),
            ],
            NonZeroUsize::new(7)
                .expect("nonzero page size"),
        );

        let context = hardware_context();

        let page = discover(
            &provider,
            &context,
        );

        assert_eq!(
            page.len(),
            1,
            "provider {provider_name} must expose its device",
        );

        assert_eq!(
            page.devices()[0]
                .identity()
                .provider_id()
                .as_str(),
            provider_name,
        );

        assert!(
            page.is_complete(),
            "single-device inventory should be complete",
        );
    }
}

#[test]
fn provider_trait_objects_preserve_the_common_hardware_boundary() {
    let provider = initialized_provider(
        "portable.trait.object.provider",
        [
            simulated_device(
                "portable.trait.object.provider",
                "device-001",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    // The concrete provider can be consumed through the canonical
    // hardware-provider interface without domain-specific downcasting.
    let erased: Arc<dyn HardwareProvider> = Arc::new(provider);

    let context = hardware_context();

    let page = discover(
        erased.as_ref(),
        &context,
    );

    assert_eq!(page.len(), 1);
}

// ============================================================================
// 4. PORTABLE DEVICE INVENTORY
// ============================================================================

#[test]
fn device_inventory_is_not_bound_to_a_specific_provider() {
    let first = initialized_provider(
        "portable.provider.first",
        [
            simulated_device(
                "portable.provider.first",
                "local-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(3)
            .expect("nonzero page size"),
    );

    let second = initialized_provider(
        "portable.provider.second",
        [
            simulated_device(
                "portable.provider.second",
                "local-device",
                "accelerator.gpu",
            ),
        ],
        NonZeroUsize::new(3)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let first_page = discover(
        &first,
        &context,
    );

    let second_page = discover(
        &second,
        &context,
    );

    assert_eq!(first_page.len(), 1);
    assert_eq!(second_page.len(), 1);

    assert_eq!(
        first_page.devices()[0]
            .identity()
            .device_id()
            .as_str(),
        "local-device",
    );

    assert_eq!(
        second_page.devices()[0]
            .identity()
            .device_id()
            .as_str(),
        "local-device",
    );

    assert_ne!(
        first_page.devices()[0].identity(),
        second_page.devices()[0].identity(),
    );
}

#[test]
fn discovery_does_not_change_provider_inventory() {
    let provider = initialized_provider(
        "observational.provider",
        [
            simulated_device(
                "observational.provider",
                "device-a",
                "classical.cpu",
            ),
            simulated_device(
                "observational.provider",
                "device-b",
                "accelerator.gpu",
            ),
        ],
        NonZeroUsize::new(10)
            .expect("nonzero page size"),
    );

    let before = provider
        .device_count()
        .expect("inventory observation");

    let context = hardware_context();

    let _page = discover(
        &provider,
        &context,
    );

    let after = provider
        .device_count()
        .expect("inventory observation");

    assert_eq!(before, after);

    // Discovery must not allocate, reserve or remove hardware.
}

// ============================================================================
// 5. DYNAMIC HARDWARE DISCOVERY
// ============================================================================

#[test]
fn newly_registered_hardware_becomes_discoverable() {
    let provider = initialized_provider(
        "dynamic.provider",
        [
            simulated_device(
                "dynamic.provider",
                "initial-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(10)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let initial = discover(
        &provider,
        &context,
    );

    assert_eq!(initial.len(), 1);

    provider
        .register_device(
            simulated_device(
                "dynamic.provider",
                "new-device",
                "future.photonic.processor",
            ),
        )
        .expect("device registration must succeed");

    let updated = discover(
        &provider,
        &context,
    );

    assert_eq!(updated.len(), 2);

    let identities: BTreeSet<_> = updated
        .devices()
        .iter()
        .map(|device| {
            device.identity().device_id().as_str()
        })
        .collect();

    assert!(identities.contains("initial-device"));
    assert!(identities.contains("new-device"));
}

#[test]
fn removed_hardware_disappears_from_subsequent_discovery() {
    let provider = initialized_provider(
        "removal.provider",
        [
            simulated_device(
                "removal.provider",
                "device-a",
                "classical.cpu",
            ),
            simulated_device(
                "removal.provider",
                "device-b",
                "accelerator.gpu",
            ),
        ],
        NonZeroUsize::new(10)
            .expect("nonzero page size"),
    );

    let identity = device_identity(
        "removal.provider",
        "device-a",
    );

    let context = hardware_context();

    assert_eq!(
        discover(&provider, &context).len(),
        2,
    );

    let removed = provider
        .remove_device(&identity)
        .expect("device removal must succeed");

    assert!(
        removed.is_some(),
        "existing device must be removed",
    );

    let updated = discover(
        &provider,
        &context,
    );

    assert_eq!(updated.len(), 1);

    assert_eq!(
        updated.devices()[0]
            .identity()
            .device_id()
            .as_str(),
        "device-b",
    );
}

#[test]
fn removing_unknown_hardware_does_not_corrupt_inventory() {
    let provider = initialized_provider(
        "safe.removal.provider",
        [
            simulated_device(
                "safe.removal.provider",
                "existing-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let unknown = device_identity(
        "safe.removal.provider",
        "unknown-device",
    );

    let removed = provider
        .remove_device(&unknown)
        .expect("removing an unknown device is a valid no-op");

    assert!(removed.is_none());

    assert_eq!(
        provider.device_count().expect("inventory count"),
        1,
    );
}

// ============================================================================
// 6. PAGINATED DISCOVERY AND SCALABILITY
// ============================================================================

#[test]
fn paginated_discovery_preserves_unique_device_identities() {
    // This is a finite regression workload.
    //
    // It is NOT a maximum supported inventory size.
    //
    // The provider and discovery contracts do not derive their
    // supported capacity from this test sample.
    const INVENTORY_SAMPLE_SIZE: usize = 257;

    let devices = (0..INVENTORY_SAMPLE_SIZE).map(|index| {
        simulated_device(
            "pagination.provider",
            &format!("device-{index}"),
            "classical.cpu",
        )
    });

    let provider = initialized_provider(
        "pagination.provider",
        devices,
        NonZeroUsize::new(11)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let mut cursor = None;
    let mut identities = BTreeSet::new();
    let mut pages_examined: usize = 0;

    loop {
        let request = DeviceDiscoveryRequest::new(
            cursor.clone(),
            NonZeroUsize::new(11),
        );

        let page = block_on(
            provider.discover_devices(
                &request,
                &context,
            ),
        )
        .expect("validated discovery page must succeed");

        pages_examined = pages_examined
            .checked_add(1)
            .expect("page count must remain representable");

        assert!(
            page.len() <= 11,
            "provider exceeded the requested page size",
        );

        for descriptor in page.devices() {
            assert!(
                identities.insert(
                    descriptor.identity().clone()
                ),
                "duplicate device identity across discovery pages",
            );
        }

        match page.next_cursor() {
            Some(next) => {
                let next = next.clone();

                assert_ne!(
                    cursor.as_ref(),
                    Some(&next),
                    "discovery cursor must make progress",
                );

                cursor = Some(next);
            }

            None => {
                assert!(
                    page.is_complete(),
                    "final discovery page must be complete",
                );

                break;
            }
        }
    }

    assert_eq!(
        identities.len(),
        INVENTORY_SAMPLE_SIZE,
    );

    assert!(
        pages_examined > 1,
        "the test must exercise multiple discovery pages",
    );
}

#[test]
fn empty_hardware_inventory_is_a_valid_discovery_result() {
    let provider = initialized_provider(
        "empty.inventory.provider",
        std::iter::empty(),
        NonZeroUsize::new(4)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let page = discover(
        &provider,
        &context,
    );

    assert!(page.is_empty());
    assert!(page.is_complete());
}

#[test]
fn inventory_growth_does_not_change_provider_identity() {
    let provider = initialized_provider(
        "stable.identity.provider",
        [
            simulated_device(
                "stable.identity.provider",
                "device-a",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(10)
            .expect("nonzero page size"),
    );

    let original_identity = provider
        .descriptor()
        .id()
        .clone();

    provider
        .register_device(
            simulated_device(
                "stable.identity.provider",
                "device-b",
                "future.neuromorphic.processor",
            ),
        )
        .expect("registration must succeed");

    assert_eq!(
        provider.descriptor().id(),
        &original_identity,
    );
}

// ============================================================================
// 7. HARDWARE PORTABILITY AND DOMAIN OWNERSHIP
// ============================================================================

#[test]
fn classical_quantum_and_hdl_categories_remain_distinct() {
    let descriptors = [
        device_descriptor(
            "heterogeneous.system",
            "device-classical",
            "classical.cpu",
        ),
        device_descriptor(
            "heterogeneous.system",
            "device-quantum",
            "quantum.qpu",
        ),
        device_descriptor(
            "heterogeneous.system",
            "device-hdl",
            "hdl.fpga",
        ),
    ];

    let identities: BTreeSet<_> = descriptors
        .iter()
        .map(|descriptor| {
            descriptor.identity().clone()
        })
        .collect();

    assert_eq!(identities.len(), descriptors.len());

    for descriptor in &descriptors {
        assert!(
            descriptor.validate().is_ok(),
        );
    }

    // ZHAL represents the common hardware identity boundary.
    //
    // Quantum gates, quantum connectivity, FPGA signals,
    // HDL realization and classical instructions remain
    // under their respective domain authorities.
}

#[test]
fn hardware_categories_do_not_define_execution_semantics() {
    let first = device_descriptor(
        "portable.execution.provider",
        "device-classical",
        "classical.cpu",
    );

    let second = device_descriptor(
        "portable.execution.provider",
        "device-quantum",
        "quantum.qpu",
    );

    let third = device_descriptor(
        "portable.execution.provider",
        "device-hdl",
        "hdl.fpga",
    );

    // The generic descriptor identifies hardware.
    //
    // It does not create a classical instruction set,
    // quantum circuit representation or HDL program.
    //
    // Each descriptor remains independently valid.
    assert!(first.validate().is_ok());
    assert!(second.validate().is_ok());
    assert!(third.validate().is_ok());

    assert_ne!(first.identity(), second.identity());
    assert_ne!(second.identity(), third.identity());
    assert_ne!(first.identity(), third.identity());
}

// ============================================================================
// 8. PROVIDER-SCOPED ACCESS AND VALIDATION
// ============================================================================

#[test]
fn foreign_provider_identity_is_not_reinterpreted_as_local_identity() {
    let provider = initialized_provider(
        "identity.owner",
        [
            simulated_device(
                "identity.owner",
                "device-a",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let foreign_identity = device_identity(
        "another.provider",
        "device-a",
    );

    let context = hardware_context();

    let result = block_on(
        provider.inspect_device_validated(
            &foreign_identity,
            &context,
        ),
    );

    assert!(
        result.is_err(),
        "a provider must not inspect another provider's device",
    );
}

#[test]
fn device_identity_survives_independent_provider_observations() {
    let provider = initialized_provider(
        "observation.provider",
        [
            simulated_device(
                "observation.provider",
                "stable-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let first = discover(
        &provider,
        &context,
    );

    let second = discover(
        &provider,
        &context,
    );

    assert_eq!(
        first.devices()[0].identity(),
        second.devices()[0].identity(),
    );
}

// ============================================================================
// 9. PORTABILITY DOES NOT IMPLY UNIVERSAL FEASIBILITY
// ============================================================================

#[test]
fn unsupported_hardware_is_not_fabricated_by_discovery() {
    let provider = initialized_provider(
        "honest.discovery.provider",
        [
            simulated_device(
                "honest.discovery.provider",
                "classical-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let page = discover(
        &provider,
        &context,
    );

    assert_eq!(page.len(), 1);

    assert_eq!(
        page.devices()[0]
            .identity()
            .device_id()
            .as_str(),
        "classical-device",
    );

    // No quantum or HDL device is manufactured merely because
    // the provider participates in the universal ZHAL contract.
}

#[test]
fn provider_readiness_does_not_imply_universal_capability() {
    let provider = initialized_provider(
        "limited.capability.provider",
        [
            simulated_device(
                "limited.capability.provider",
                "device-a",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    // A ready provider only establishes its operational provider
    // status. It does not establish every possible hardware
    // capability or workload compatibility.
    assert!(
        provider.status().may_accept_operations(),
    );

    let context = hardware_context();

    let snapshot = block_on(
        provider.capability_snapshot(
            context.operation_context(),
        ),
    )
    .expect("capability observation must succeed");

    assert!(
        snapshot.iter().next().is_none(),
        "an empty snapshot must not manufacture capabilities",
    );
}

// ============================================================================
// 10. INVENTORY AND OBSERVATION CONSISTENCY
// ============================================================================

#[test]
fn device_registration_is_explicit() {
    let provider = initialized_provider(
        "explicit.registration.provider",
        std::iter::empty(),
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    assert_eq!(
        provider.device_count().expect("inventory count"),
        0,
    );

    provider
        .register_device(
            simulated_device(
                "explicit.registration.provider",
                "new-device",
                "future.photonic.processor",
            ),
        )
        .expect("explicit registration must succeed");

    assert_eq!(
        provider.device_count().expect("inventory count"),
        1,
    );
}

#[test]
fn device_removal_is_explicit() {
    let provider = initialized_provider(
        "explicit.removal.provider",
        [
            simulated_device(
                "explicit.removal.provider",
                "existing-device",
                "classical.cpu",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let identity = device_identity(
        "explicit.removal.provider",
        "existing-device",
    );

    let before = provider
        .device_count()
        .expect("inventory count");

    assert_eq!(before, 1);

    provider
        .remove_device(&identity)
        .expect("removal must succeed");

    let after = provider
        .device_count()
        .expect("inventory count");

    assert_eq!(after, 0);
}

// ============================================================================
// 11. FUTURE TECHNOLOGY EXTENSIBILITY
// ============================================================================

#[test]
fn future_provider_names_do_not_require_closed_enumerations() {
    let providers = [
        "future.photonic.provider",
        "future.neuromorphic.provider",
        "future.molecular.provider",
        "future.optical.provider",
        "future.biological.provider",
        "future.nanocomputing.provider",
        "future.analog.provider",
        "future.hybrid.provider",
    ];

    for name in providers {
        let descriptor = provider_descriptor(name);

        assert_eq!(
            descriptor.id().as_str(),
            name,
        );
    }
}

#[test]
fn future_hardware_can_use_existing_provider_and_device_contracts() {
    let provider = initialized_provider(
        "future.photonic.provider",
        [
            simulated_device(
                "future.photonic.provider",
                "photonic-device",
                "future.photonic.processor",
            ),
        ],
        NonZeroUsize::new(5)
            .expect("nonzero page size"),
    );

    let context = hardware_context();

    let page = discover(
        &provider,
        &context,
    );

    assert_eq!(page.len(), 1);

    assert_eq!(
        page.devices()[0]
            .identity()
            .provider_id()
            .as_str(),
        "future.photonic.provider",
    );
}

// ============================================================================
// 12. PORTABILITY CONTRACT REGRESSION
// ============================================================================

#[test]
fn provider_and_device_identities_are_deterministic() {
    let first = device_identity(
        "deterministic.provider",
        "device-001",
    );

    let second = device_identity(
        "deterministic.provider",
        "device-001",
    );

    assert_eq!(first, second);

    assert_eq!(
        first.provider_id(),
        second.provider_id(),
    );

    assert_eq!(
        first.device_id(),
        second.device_id(),
    );
}

#[test]
fn different_domains_do_not_require_different_generic_discovery_apis() {
    let providers = [
        (
            "portable.classical",
            "classical.cpu",
        ),
        (
            "portable.quantum",
            "quantum.qpu",
        ),
        (
            "portable.hdl",
            "hdl.fpga",
        ),
        (
            "portable.accelerator",
            "accelerator.gpu",
        ),
        (
            "portable.distributed",
            "distributed.remote",
        ),
        (
            "portable.embedded",
            "embedded.microcontroller",
        ),
    ];

    for (provider_name, category) in providers {
        let provider = initialized_provider(
            provider_name,
            [
                simulated_device(
                    provider_name,
                    "device-001",
                    category,
                ),
            ],
            NonZeroUsize::new(5)
                .expect("nonzero page size"),
        );

        let context = hardware_context();

        let page = discover(
            &provider,
            &context,
        );

        assert_eq!(
            page.len(),
            1,
        );

        assert_eq!(
            page.devices()[0]
                .identity()
                .provider_id()
                .as_str(),
            provider_name,
        );
    }
}
