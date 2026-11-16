
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZHAL Scalability and POCO-REAF Tests
//!
//! Path: src/zhal/tests/scalability.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Establish scalability properties of the canonical hardware-facing
//! contracts that ZHAL integrates with.
//!
//! These tests verify:
//!
//! - Open-world provider, capability and device identities.
//! - Growth of capability snapshots.
//! - Growth of heterogeneous device descriptions.
//! - Dynamic capability populations.
//! - Long identifiers without arbitrary application-level ceilings.
//! - Large resource quantities without machine-integer conversion.
//! - Deterministic capability iteration.
//! - Explicit handling of unknown and unavailable capabilities.
//! - Device replacement and inventory-observation changes.
//! - Safe concurrent sharing of canonical contract values.
//! - Resource-bounded test execution.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This test module reuses:
//!
//! - crate::stdlib::provider
//! - crate::stdlib::capability
//! - crate::stdlib::io::devices
//!
//! It does not implement another:
//!
//! - Provider registry.
//! - Hardware inventory authority.
//! - Device discovery engine.
//! - Resource allocator.
//! - Resource scheduler.
//! - Semantic requirement model.
//! - Quantum hardware abstraction.
//! - HDL realization system.
//! - Distributed execution engine.
//!
//! ## IMPORTANT LIMITATION
//!
//! The current repository does not yet expose an implemented ZHAL
//! provider registry, inventory, or discovery API.
//!
//! Consequently, these tests verify the existing canonical contracts
//! that ZHAL must reuse. They do not certify physical hardware discovery
//! or a nonexistent ZHAL inventory implementation.
//!
//! Those integration tests must be added when the corresponding
//! production ZHAL interfaces are implemented.
//!
//! ## SCALABILITY MODEL
//!
//! There is no fixed maximum number of devices or capabilities in
//! this test implementation.
//!
//! The stress-test population is configurable through:
//!
//! ZHAL_SCALABILITY_ITEMS
//!
//! If unspecified, a moderate default workload is used.
//!
//! The configured value is a test workload, not a hardware capability
//! limit or a language-level maximum.
//!
//! Actual execution remains subject to available memory, address-space
//! limits, operating-system resources and execution time.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::stdlib::capability::{
    Capability,
    CapabilityEvidence,
    CapabilityId,
    CapabilityKind,
    CapabilitySnapshot,
    CapabilityState,
    CapabilityStatus,
};

use crate::stdlib::io::devices::{
    DeviceCapabilityId,
    DeviceDescriptor,
    DeviceId,
    DeviceKindId,
};

use crate::stdlib::provider::ProviderId;

// ============================================================================
// TEST CONFIGURATION
// ============================================================================

/// Environment variable controlling the requested stress-test population.
const SCALABILITY_ITEMS_ENV: &str = "ZHAL_SCALABILITY_ITEMS";

/// Default test workload.
///
/// This is a test execution parameter, not a supported hardware ceiling.
const DEFAULT_SCALABILITY_ITEMS: usize = 2_048;

/// Returns the requested test population.
///
/// The caller may increase or decrease this value according to available
/// testing resources.
///
/// No artificial upper bound is imposed.
///
/// # Panics
///
/// Panics if the explicitly supplied test population cannot be parsed
/// or is zero.
fn scalability_items() -> usize {
    match std::env::var(SCALABILITY_ITEMS_ENV) {
        Ok(value) => {
            let parsed = value
                .parse::<usize>()
                .expect("ZHAL_SCALABILITY_ITEMS must be a valid usize");

            assert!(
                parsed > 0,
                "ZHAL_SCALABILITY_ITEMS must be greater than zero"
            );

            parsed
        }

        Err(std::env::VarError::NotPresent) => {
            DEFAULT_SCALABILITY_ITEMS
        }

        Err(error) => {
            panic!(
                "Unable to read {SCALABILITY_ITEMS_ENV}: {error}"
            );
        }
    }
}

// ============================================================================
// CANONICAL TEST FIXTURES
// ============================================================================

/// Creates a capability using an open-world identity.
fn make_capability(index: usize) -> Capability {
    let id = CapabilityId::new(
        format!("future.domain.capability-{index}")
    )
    .expect("valid extensible capability identity");

    let kind = CapabilityKind::new("computation")
        .expect("valid capability kind");

    Capability::new(id, kind)
}

/// Creates a capability state with explicit evidence.
fn make_capability_state(index: usize) -> CapabilityState {
    let capability = make_capability(index);

    let status = if index.is_multiple_of(2) {
        CapabilityStatus::Available
    } else {
        CapabilityStatus::Unknown
    };

    let evidence = CapabilityEvidence::new(
        "zhal.scalability.test",
        format!("Observation for capability {index}"),
    )
    .expect("valid capability evidence");

    CapabilityState::new(capability, status)
        .with_evidence(evidence)
}

/// Creates a provider-neutral device description.
///
/// Device kinds are generated dynamically instead of selected from
/// a closed hardware-category enumeration.
fn make_device(index: usize) -> DeviceDescriptor {
    let provider_id = ProviderId::new("test.provider")
        .expect("valid provider identity");

    let device_id = DeviceId::new(
        format!("device-{index}")
    )
    .expect("valid device identity");

    let device_kind = DeviceKindId::new(
        format!("future.domain.{index}")
    )
    .expect("valid open-world device kind");

    let capability = DeviceCapabilityId::new(
        "future.capability.compute"
    )
    .expect("valid device capability");

    let mut properties = BTreeMap::new();

    properties.insert(
        "test.index".to_owned(),
        index.to_string(),
    );

    properties.insert(
        "test.domain".to_owned(),
        format!("future.domain.{index}"),
    );

    DeviceDescriptor::new(
        provider_id,
        device_id,
        device_kind,
        format!("Scalability test device {index}"),
        [capability],
        properties,
    )
    .expect("valid provider-neutral device descriptor")
}

/// Creates a capability snapshot containing a dynamically supplied
/// number of independently identified capabilities.
fn make_snapshot(count: usize, prefix: &str) -> CapabilitySnapshot {
    let mut snapshot = CapabilitySnapshot::new();

    for index in 0..count {
        let capability = Capability::new(
            CapabilityId::new(
                format!("{prefix}.capability.{index}")
            )
            .expect("valid capability identity"),
            CapabilityKind::new("computation")
                .expect("valid capability kind"),
        );

        let state = CapabilityState::new(
            capability,
            CapabilityStatus::Available,
        );

        snapshot
            .insert(state)
            .expect("capability identities must be unique");
    }

    snapshot
}

// ============================================================================
// 1. OPEN-WORLD IDENTITY SCALABILITY
// ============================================================================

/// New provider namespaces must not require modification of a fixed
/// provider catalogue.
#[test]
fn provider_identity_supports_future_namespaces() {
    let identities = [
        "classical.cpu.provider",
        "quantum.qpu.provider",
        "hdl.fpga.provider",
        "distributed.remote.provider",
        "neuromorphic.provider",
        "photonic.provider",
        "future.computing.provider",
    ];

    for identity in identities {
        let provider = ProviderId::new(identity)
            .expect("valid open-world provider identity");

        assert_eq!(provider.as_str(), identity);
    }
}

/// Capability identities must support previously unknown domains.
#[test]
fn capability_identity_supports_future_domains() {
    let identities = [
        "classical.execution",
        "quantum.measurement",
        "hdl.synthesis",
        "distributed.collectives",
        "ai.tensor_acceleration",
        "neuromorphic.events",
        "photonic.computation",
        "future.domain.operation",
    ];

    for identity in identities {
        let capability = CapabilityId::new(identity)
            .expect("valid extensible capability identity");

        assert_eq!(capability.as_str(), identity);
    }
}

/// Device categories must remain extensible.
///
/// Introducing a new device category must not require changing a
/// central device-kind enum.
#[test]
fn device_kinds_support_future_hardware_categories() {
    let kinds = [
        "classical.processor",
        "quantum.processor",
        "hdl.reconfigurable_logic",
        "accelerator.tensor",
        "embedded.controller",
        "neuromorphic.processor",
        "photonic.processor",
        "future.hardware.category",
    ];

    for kind in kinds {
        let device_kind = DeviceKindId::new(kind)
            .expect("valid extensible device kind");

        assert_eq!(device_kind.as_str(), kind);
    }
}

/// Existing identifier authorities must not impose arbitrary
/// application-level maximum lengths.
#[test]
fn identifiers_support_large_valid_namespaces() {
    let long_segment = "x".repeat(65_536);

    let provider_name = format!("future.{long_segment}");

    let provider = ProviderId::new(provider_name.clone())
        .expect("long provider identity must remain representable");

    assert_eq!(provider.as_str(), provider_name);

    let capability_name = format!(
        "future.capability.{long_segment}"
    );

    let capability = CapabilityId::new(
        capability_name.clone()
    )
    .expect("long capability identity must remain representable");

    assert_eq!(
        capability.as_str(),
        capability_name
    );

    let device_name = format!(
        "device-{long_segment}"
    );

    let device = DeviceId::new(device_name.clone())
        .expect("long device identity must remain representable");

    assert_eq!(device.as_str(), device_name);
}

// ============================================================================
// 2. CAPABILITY SNAPSHOT SCALABILITY
// ============================================================================

/// Capability snapshots must grow according to supplied data rather
/// than an arbitrary fixed capability count.
#[test]
fn capability_snapshot_grows_with_available_resources() {
    let count = scalability_items();

    let mut snapshot = CapabilitySnapshot::new();

    for index in 0..count {
        snapshot
            .insert(make_capability_state(index))
            .expect("unique capability must be accepted");
    }

    assert_eq!(snapshot.len(), count);

    assert!(!snapshot.is_empty());

    assert_eq!(
        snapshot.states().count(),
        count
    );

    assert_eq!(
        snapshot.ids().count(),
        count
    );
}

/// Capability lookup must remain correct as the snapshot grows.
#[test]
fn capability_snapshot_supports_dynamic_lookup() {
    let count = scalability_items();

    let mut snapshot = CapabilitySnapshot::new();

    for index in 0..count {
        snapshot
            .insert(make_capability_state(index))
            .expect("unique capability");
    }

    for index in 0..count {
        let identity = format!(
            "future.domain.capability-{index}"
        );

        let state = snapshot
            .get_by_name(&identity)
            .expect("inserted capability must be discoverable");

        assert_eq!(
            state.id().as_str(),
            identity
        );
    }
}

/// Duplicate identities must not silently overwrite existing evidence.
#[test]
fn capability_snapshot_rejects_duplicate_identities() {
    let mut snapshot = CapabilitySnapshot::new();

    snapshot
        .insert(make_capability_state(1))
        .expect("initial insertion");

    let duplicate = snapshot.insert(
        make_capability_state(1)
    );

    assert!(
        duplicate.is_err(),
        "duplicate capability identities must be rejected"
    );

    assert_eq!(snapshot.len(), 1);
}

/// Capability iteration must be deterministic.
#[test]
fn capability_snapshot_iteration_is_deterministic() {
    let count = scalability_items();

    let mut first = make_snapshot(
        count,
        "future.domain",
    );

    let mut second = make_snapshot(
        count,
        "future.domain",
    );

    let first_ids: Vec<String> = first
        .ids()
        .map(|id| id.as_str().to_owned())
        .collect();

    let second_ids: Vec<String> = second
        .ids()
        .map(|id| id.as_str().to_owned())
        .collect();

    assert_eq!(first_ids, second_ids);

    assert_eq!(first.len(), second.len());

    // Keep both snapshots mutable in this test to exercise the
    // canonical removal API without introducing another registry.
    let first_id = first
        .ids()
        .next()
        .expect("non-empty snapshot")
        .clone();

    assert!(first.remove(&first_id).is_some());

    assert_eq!(first.len(), count - 1);
    assert_eq!(second.len(), count);
}

// ============================================================================
// 3. AVAILABILITY SEMANTICS
// ============================================================================

/// Unknown capability information must not become positive availability.
#[test]
fn unknown_capability_is_not_treated_as_available() {
    let mut snapshot = CapabilitySnapshot::new();

    let capability = Capability::new(
        CapabilityId::new("future.hardware.unknown")
            .expect("valid identity"),
        CapabilityKind::new("computation")
            .expect("valid kind"),
    );

    let state = CapabilityState::new(
        capability,
        CapabilityStatus::Unknown,
    );

    snapshot
        .insert(state)
        .expect("valid state");

    let identity = CapabilityId::new(
        "future.hardware.unknown"
    )
    .expect("valid identity");

    assert_eq!(
        snapshot.status(&identity),
        CapabilityStatus::Unknown
    );

    assert!(
        !snapshot.establishes_availability(&identity)
    );
}

/// An absent capability is unknown, not automatically unavailable.
#[test]
fn missing_capability_information_remains_unknown() {
    let snapshot = CapabilitySnapshot::new();

    let missing = CapabilityId::new(
        "future.hardware.not_observed"
    )
    .expect("valid identity");

    assert_eq!(
        snapshot.status(&missing),
        CapabilityStatus::Unknown
    );

    assert!(
        !snapshot.establishes_availability(&missing)
    );
}

/// Known unavailability must remain distinct from unknown availability.
#[test]
fn unavailable_and_unknown_capabilities_remain_distinct() {
    let mut snapshot = CapabilitySnapshot::new();

    let unavailable = CapabilityState::new(
        Capability::new(
            CapabilityId::new("test.unavailable")
                .expect("valid identity"),
            CapabilityKind::new("computation")
                .expect("valid kind"),
        ),
        CapabilityStatus::Unavailable,
    );

    let unknown = CapabilityState::new(
        Capability::new(
            CapabilityId::new("test.unknown")
                .expect("valid identity"),
            CapabilityKind::new("computation")
                .expect("valid kind"),
        ),
        CapabilityStatus::Unknown,
    );

    snapshot
        .insert(unavailable)
        .expect("valid unavailable state");

    snapshot
        .insert(unknown)
        .expect("valid unknown state");

    assert_eq!(
        snapshot.status(
            &CapabilityId::new("test.unavailable")
                .expect("valid identity")
        ),
        CapabilityStatus::Unavailable
    );

    assert_eq!(
        snapshot.status(
            &CapabilityId::new("test.unknown")
                .expect("valid identity")
        ),
        CapabilityStatus::Unknown
    );
}

// ============================================================================
// 4. HETEROGENEOUS DEVICE SCALABILITY
// ============================================================================

/// Device descriptors must support dynamically introduced hardware
/// categories without changing the common descriptor type.
#[test]
fn heterogeneous_device_descriptors_are_extensible() {
    let domains = [
        "classical",
        "quantum",
        "hdl",
        "accelerator",
        "embedded",
        "distributed",
        "neuromorphic",
        "photonic",
        "future",
    ];

    for (index, domain) in domains.iter().enumerate() {
        let device = DeviceDescriptor::new(
            ProviderId::new("test.heterogeneous.provider")
                .expect("provider identity"),
            DeviceId::new(format!("device-{index}"))
                .expect("device identity"),
            DeviceKindId::new(format!("{domain}.hardware"))
                .expect("device kind"),
            format!("{domain} test device"),
            [
                DeviceCapabilityId::new(
                    format!("{domain}.capability")
                )
                .expect("device capability"),
            ],
            BTreeMap::new(),
        )
        .expect("valid heterogeneous device descriptor");

        assert_eq!(
            device.kind().as_str(),
            format!("{domain}.hardware")
        );

        assert_eq!(
            device.capabilities().len(),
            1
        );
    }
}

/// A large collection of provider-neutral device descriptions must
/// be representable without a universal device-count constant.
///
/// This tests descriptor storage, not a physical inventory manager.
#[test]
fn device_descriptor_population_scales_dynamically() {
    let count = scalability_items();

    let mut devices: Vec<DeviceDescriptor> = Vec::new();

    devices
        .try_reserve_exact(count)
        .expect("test environment must have sufficient memory for requested population");

    for index in 0..count {
        devices.push(make_device(index));
    }

    assert_eq!(devices.len(), count);

    assert_eq!(
        devices.first()
            .expect("non-empty inventory")
            .id()
            .as_str(),
        "device-0"
    );

    assert_eq!(
        devices.last()
            .expect("non-empty inventory")
            .id()
            .as_str(),
        format!("device-{}", count - 1)
    );

    // Every generated device has a unique provider-scoped identity.
    for (index, device) in devices.iter().enumerate() {
        assert_eq!(
            device.id().as_str(),
            format!("device-{index}")
        );

        assert_eq!(
            device.provider_id().as_str(),
            "test.provider"
        );
    }
}

// ============================================================================
// 5. DYNAMIC OBSERVATION AND REPLACEMENT
// ============================================================================

/// A new capability snapshot must be capable of representing changed
/// observations without mutating previously retained snapshots.
#[test]
fn refreshed_capability_observations_preserve_snapshot_isolation() {
    let capability = Capability::new(
        CapabilityId::new("future.device.compute")
            .expect("valid identity"),
        CapabilityKind::new("computation")
            .expect("valid kind"),
    );

    let mut original = CapabilitySnapshot::new();

    original
        .insert(
            CapabilityState::new(
                capability.clone(),
                CapabilityStatus::Available,
            )
        )
        .expect("initial observation");

    let mut refreshed = CapabilitySnapshot::new();

    refreshed
        .insert(
            CapabilityState::new(
                capability,
                CapabilityStatus::Unavailable,
            )
        )
        .expect("refreshed observation");

    let identity = CapabilityId::new(
        "future.device.compute"
    )
    .expect("valid identity");

    assert_eq!(
        original.status(&identity),
        CapabilityStatus::Available
    );

    assert_eq!(
        refreshed.status(&identity),
        CapabilityStatus::Unavailable
    );
}

// ============================================================================
// 6. RESOURCE QUANTITY REPRESENTATION
// ============================================================================

/// Exact decimal resource quantities must not depend on conversion to
/// usize, u64, u128 or floating-point values.
///
/// This test uses the canonical semantic resource model directly.
#[test]
fn large_resource_quantities_remain_exact() {
    use crate::frontend::semantic::resources::model::ExactQuantity;

    let large = format!(
        "1{}",
        "0".repeat(4_096)
    );

    let smaller = "9".repeat(4_096);

    let large_quantity = ExactQuantity::new(&large)
        .expect("valid large exact quantity");

    let smaller_quantity = ExactQuantity::new(&smaller)
        .expect("valid smaller exact quantity");

    assert_eq!(
        large_quantity.as_str(),
        large
    );

    assert_eq!(
        large_quantity.numeric_cmp(&smaller_quantity),
        std::cmp::Ordering::Greater
    );
}

/// Zero must remain an exact value rather than being confused with
/// an unspecified quantity.
#[test]
fn exact_zero_is_distinct_from_missing_resource_information() {
    use crate::frontend::semantic::resources::model::{
        ExactQuantity,
        ResourceQuantity,
    };

    let zero = ExactQuantity::new("0")
        .expect("valid zero");

    assert!(zero.is_zero());

    let known_zero = ResourceQuantity::known("0")
        .expect("valid known zero");

    assert!(known_zero.is_known());

    assert_eq!(
        known_zero.known_value()
            .expect("known quantity")
            .as_str(),
        "0"
    );

    let unspecified = ResourceQuantity::Unspecified;

    assert!(!unspecified.is_known());
}

// ============================================================================
// 7. CONCURRENT CONTRACT SAFETY
// ============================================================================

/// Canonical hardware-facing contract types must remain safe to share
/// across concurrent runtime and provider operations.
#[test]
fn hardware_contract_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<ProviderId>();
    assert_send_sync::<CapabilityId>();
    assert_send_sync::<Capability>();
    assert_send_sync::<CapabilityState>();
    assert_send_sync::<CapabilitySnapshot>();
    assert_send_sync::<DeviceId>();
    assert_send_sync::<DeviceKindId>();
    assert_send_sync::<DeviceCapabilityId>();
    assert_send_sync::<DeviceDescriptor>();
}

/// Immutable snapshots can be shared between independent observers.
#[test]
fn capability_snapshots_support_concurrent_readers() {
    let count = scalability_items();

    let snapshot = Arc::new(
        make_snapshot(count, "concurrent.test")
    );

    let first = Arc::clone(&snapshot);
    let second = Arc::clone(&snapshot);

    let first_reader = std::thread::spawn(move || {
        first.states().count()
    });

    let second_reader = std::thread::spawn(move || {
        second.states().count()
    });

    assert_eq!(
        first_reader.join()
            .expect("first reader must complete"),
        count
    );

    assert_eq!(
        second_reader.join()
            .expect("second reader must complete"),
        count
    );

    assert_eq!(snapshot.len(), count);
}

// ============================================================================
// 8. RESOURCE-BUDGET AND FAILURE BEHAVIOR
// ============================================================================

/// A requested stress workload must fail explicitly when its collection
/// cannot reserve the required memory.
///
/// This prevents silently truncating a requested test workload.
#[test]
fn stress_workload_reservation_is_explicit() {
    let count = scalability_items();

    let mut storage: Vec<usize> = Vec::new();

    let reservation = storage.try_reserve_exact(count);

    assert!(
        reservation.is_ok(),
        "requested scalability workload could not be reserved: \
         count={count}, error={reservation:?}"
    );

    for index in 0..count {
        storage.push(index);
    }

    assert_eq!(storage.len(), count);
}

/// Failed or absent capability observations must not fabricate success.
#[test]
fn failed_observation_does_not_create_capability_availability() {
    let snapshot = CapabilitySnapshot::new();

    let capability = CapabilityId::new(
        "future.provider.unobserved"
    )
    .expect("valid identity");

    assert!(
        snapshot.get(&capability).is_none()
    );

    assert_eq!(
        snapshot.status(&capability),
        CapabilityStatus::Unknown
    );

    assert!(
        !snapshot.establishes_availability(&capability)
    );
}

// ============================================================================
// 9. MINIMAL AND LARGE WORKLOAD COMPATIBILITY
// ============================================================================

/// The same canonical contracts must represent a single device and
/// a dynamically populated device collection.
#[test]
fn tiny_and_large_device_populations_use_the_same_contract() {
    let tiny = make_device(0);

    assert_eq!(
        tiny.id().as_str(),
        "device-0"
    );

    let count = scalability_items();

    let mut large: Vec<DeviceDescriptor> = Vec::new();

    large
        .try_reserve_exact(count)
        .expect("sufficient memory for configured workload");

    for index in 0..count {
        large.push(make_device(index));
    }

    assert_eq!(large.len(), count);

    assert_eq!(
        large[0].kind().as_str(),
        "future.domain.0"
    );
}

/// The capability contract must work for the smallest non-empty
/// snapshot and a larger dynamically populated snapshot.
#[test]
fn tiny_and_large_capability_populations_use_the_same_contract() {
    let mut tiny = CapabilitySnapshot::new();

    tiny.insert(make_capability_state(0))
        .expect("single capability");

    assert_eq!(tiny.len(), 1);

    let large = make_snapshot(
        scalability_items(),
        "future.scalability",
    );

    assert_eq!(
        large.len(),
        scalability_items()
    );
}

// ============================================================================
// END OF SCALABILITY CONTRACT TESTS
// ============================================================================
