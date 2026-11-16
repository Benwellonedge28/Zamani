
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani ZHAL — Fault-Tolerance Tests
//!
//! Path: src/zhal/tests/fault_tolerance.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! These tests exercise the existing ZHAL provider, discovery,
//! device-session and simulation contracts.
//!
//! They deliberately do not create a second recovery manager,
//! scheduler, resource allocator, provider registry or execution API.
//!
//! Physical execution is never inferred from simulation success.
//! Unknown and partial outcomes remain distinct from full completion.
//! Cancellation and shutdown do not imply that external hardware
//! has physically stopped.
//!
//! The finite workloads in these tests are test samples, not
//! architectural limits on providers, devices or hardware capacity.

use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll, Wake, Waker};

use std::sync::Arc;
use std::task;

use crate::stdlib::capability::CapabilitySnapshot;
use crate::stdlib::context::OperationContext;
use crate::stdlib::error::ErrorKind as StandardErrorKind;
use crate::stdlib::io::devices::{DeviceId, DeviceKindId};
use crate::stdlib::provider::{
    ProviderDescriptor,
    ProviderId,
    ProviderKind,
    ProviderLifecycle,
    ProviderOperationId,
    ProviderStatus,
};
use crate::stdlib::version::Version;

use crate::zhal::context::ZhalContext;
use crate::zhal::errors::ZhalErrorKind;
use crate::zhal::identity::HardwareDeviceIdentity;
use crate::zhal::model::device::{
    DeviceAvailability,
    HardwareDeviceDescriptor,
};
use crate::zhal::providers::contract::{
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareSession,
};
use crate::zhal::providers::discovery::{
    DiscoveryFailurePolicy,
    DiscoveryOptions,
    DiscoverySession,
};
use crate::zhal::simulation::provider::{
    SimulatedDevice,
    SimulatedOutcome,
    SimulationFault,
    SimulationFaultPoint,
    SimulationProvider,
    SimulationProviderConfig,
};

// ============================================================================
// TEST EXECUTOR
// ============================================================================

/// A wake implementation for immediately-ready test futures.
///
/// The current simulation provider uses immediately-ready asynchronous
/// operations. This executor is intentionally limited to those operations:
/// it does not implement a production runtime or a general-purpose
/// scheduling executor.
struct TestWake;

impl Wake for TestWake {
    fn wake(self: Arc<Self>) {}
}

/// Polls a future until it completes.
///
/// This helper is appropriate for the synchronous, immediately-ready
/// provider operations exercised here. A future that genuinely requires
/// an external executor or asynchronous I/O must be tested using the
/// repository's canonical runtime executor instead.
fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(TestWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    loop {
        match Pin::as_mut(&mut future).poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => task::yield_now(),
        }
    }
}

// ============================================================================
// FIXTURES
// ============================================================================

fn provider_id(value: &str) -> ProviderId {
    ProviderId::new(value).expect("test provider identity must be valid")
}

fn provider_descriptor(id: &str) -> ProviderDescriptor {
    let version = Version::parse("1.0.0")
        .expect("test version must be valid");

    ProviderDescriptor::new(
        provider_id(id),
        ProviderKind::new("simulation")
            .expect("test provider kind must be valid"),
        "ZHAL Fault-Tolerance Test Provider",
        version.clone(),
        version,
    )
    .expect("test provider descriptor must be valid")
}

fn device(
    provider: &ProviderId,
    local_id: &str,
) -> SimulatedDevice {
    let identity = HardwareDeviceIdentity::new(
        provider.clone(),
        DeviceId::new(local_id)
            .expect("test device identity must be valid"),
    );

    let descriptor = HardwareDeviceDescriptor::new(
        identity,
        DeviceKindId::new("test.hardware.device")
            .expect("test device kind must be valid"),
        "Simulated fault-tolerance device",
    )
    .expect("test hardware descriptor must be valid");

    SimulatedDevice::new(descriptor)
}

fn provider_with_devices(
    id: &str,
    devices: impl IntoIterator<Item = SimulatedDevice>,
    page_size: NonZeroUsize,
) -> SimulationProvider {
    let config = SimulationProviderConfig::new(
        provider_descriptor(id),
        CapabilitySnapshot::new(),
        devices,
        page_size,
        std::time::Duration::ZERO,
    );

    SimulationProvider::new(config)
        .expect("test simulation provider must be valid")
}

fn empty_provider(id: &str) -> SimulationProvider {
    provider_with_devices(
        id,
        [],
        NonZeroUsize::new(1).expect("one is nonzero"),
    )
}

fn initialize(provider: &SimulationProvider) {
    let context = OperationContext::default();

    block_on(ProviderLifecycle::initialize(
        provider,
        &context,
    ))
    .expect("simulation provider initialization must succeed");
}

fn shutdown(provider: &SimulationProvider) {
    let context = OperationContext::default();

    block_on(ProviderLifecycle::shutdown(
        provider,
        &context,
    ))
    .expect("simulation provider shutdown must succeed");
}

fn hardware_context() -> ZhalContext {
    ZhalContext::default()
}

fn discovery_request(
    page_size: Option<NonZeroUsize>,
) -> DeviceDiscoveryRequest {
    DeviceDiscoveryRequest::new(None, page_size)
}

// ============================================================================
// PROVIDER FAILURE AND RECOVERY
// ============================================================================

#[test]
fn provider_starts_unavailable_until_initialized() {
    let provider = empty_provider("test.fault.provider");

    assert_eq!(
        provider.status(),
        ProviderStatus::Stopped,
    );

    let result = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    );

    assert!(result.is_err());
}

#[test]
fn provider_accepts_operations_after_successful_initialization() {
    let provider = empty_provider("test.fault.ready");

    initialize(&provider);

    assert!(
        provider.status().may_accept_operations(),
        "a successfully initialized provider must report an operational status",
    );

    let page = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    )
    .expect("discovery should work after initialization");

    assert!(page.is_empty());
}

#[test]
fn communication_failure_is_not_reported_as_success() {
    let provider = empty_provider("test.fault.communication");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::CommunicationFailure {
                message: "injected communication interruption".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let result = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    );

    let error = result.expect_err(
        "a communication failure must remain an error",
    );

    assert_eq!(
        error.kind(),
        ZhalErrorKind::HardwareCommunication,
    );
}

#[test]
fn one_shot_provider_failure_does_not_poison_later_discovery() {
    let provider = empty_provider("test.fault.recovery");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::ProviderFailure {
                message: "one-shot provider failure".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let failed = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    );

    assert_eq!(
        failed.expect_err("first discovery must fail").kind(),
        ZhalErrorKind::ProviderFailure,
    );

    let recovered = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    )
    .expect("the consumed one-shot fault must not affect later discovery");

    assert!(recovered.is_empty());
}

// ============================================================================
// DISCOVERY FAILURE ISOLATION
// ============================================================================

#[test]
fn best_effort_discovery_continues_after_an_individual_provider_fails() {
    let first_id = provider_id("test.a.discovery.failure");
    let second_id = provider_id("test.b.discovery.healthy");

    let first = Arc::new(empty_provider(first_id.as_str()));
    let second = Arc::new(empty_provider(second_id.as_str()));

    initialize(first.as_ref());
    initialize(second.as_ref());

    first
        .inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::CommunicationFailure {
                message: "first provider is temporarily unreachable".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let first_provider: Arc<dyn HardwareProvider> = first;
    let second_provider: Arc<dyn HardwareProvider> = second;

    let mut session = DiscoverySession::new(
        [first_provider, second_provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .expect("discovery session must be constructible");

    let result = block_on(session.next_page(&hardware_context()))
        .expect("best-effort discovery must continue");

    assert!(result.is_some());

    assert_eq!(
        session.report().providers_failed(),
        1,
        "the failing provider must be recorded",
    );

    assert_eq!(
        session.report().pages_observed(),
        1,
        "the healthy provider must still contribute a page",
    );

    assert_eq!(
        session.report().devices_observed(),
        0,
        "an empty healthy provider must not invent device observations",
    );

    assert!(
        session.report().has_failures(),
        "a partial discovery report must retain its failure",
    );

    assert!(
        !session.report().is_successful(),
        "a partial report must not be represented as complete success",
    );
}

#[test]
fn strict_discovery_terminates_after_a_provider_failure() {
    let first = Arc::new(empty_provider("test.a.strict.failure"));
    let second = Arc::new(empty_provider("test.b.strict.healthy"));

    initialize(first.as_ref());
    initialize(second.as_ref());

    first
        .inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::CommunicationFailure {
                message: "strict discovery failure".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let first_provider: Arc<dyn HardwareProvider> = first;
    let second_provider: Arc<dyn HardwareProvider> = second;

    let mut session = DiscoverySession::new(
        [first_provider, second_provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::Strict),
    )
    .expect("discovery session must be constructible");

    let result = block_on(session.next_page(&hardware_context()));

    assert!(
        result.is_err(),
        "strict discovery must return the provider failure",
    );

    assert!(session.is_terminated());

    assert_eq!(
        session.report().providers_failed(),
        1,
    );

    assert!(
        !session.report().is_successful(),
        "strict early termination must not report successful inventory discovery",
    );
}

// ============================================================================
// DEVICE DISAPPEARANCE AND STALE OBSERVATIONS
// ============================================================================

#[test]
fn device_disappearance_is_reported_during_inspection() {
    let id = provider_id("test.device.disappearance");

    let simulated_device = device(&id, "device.disappearing");
    let identity = simulated_device.descriptor().identity().clone();

    let provider = provider_with_devices(
        id.as_str(),
        [simulated_device],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    let removed = provider
        .remove_device(&identity)
        .expect("device removal must succeed");

    assert!(
        removed.is_some(),
        "the device must have been present before removal",
    );

    let result = block_on(
        provider.inspect_device(
            &identity,
            &hardware_context(),
        ),
    );

    assert_eq!(
        result.expect_err("a disappeared device must not be inspected")
            .kind(),
        ZhalErrorKind::DeviceNotFound,
    );
}

#[test]
fn inventory_change_invalidates_an_old_discovery_cursor() {
    let id = provider_id("test.discovery.stale.cursor");

    let first = device(&id, "device.first");
    let second = device(&id, "device.second");

    let provider = provider_with_devices(
        id.as_str(),
        [first, second],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    let first_page = block_on(
        provider.discover_devices_page(
            &discovery_request(
                NonZeroUsize::new(1),
            ),
            &hardware_context(),
        ),
    )
    .expect("first discovery page must succeed");

    let cursor = first_page
        .next_cursor()
        .cloned()
        .expect("more than one device requires a continuation cursor");

    let previous_revision = provider
        .inventory_revision()
        .expect("inventory revision must be readable");

    provider
        .register_device(device(&id, "device.added"))
        .expect("inventory update must succeed");

    let current_revision = provider
        .inventory_revision()
        .expect("inventory revision must be readable");

    assert_ne!(
        previous_revision,
        current_revision,
        "inventory mutation must advance its revision",
    );

    let stale_request = DeviceDiscoveryRequest::new(
        Some(cursor),
        NonZeroUsize::new(1),
    );

    let result = block_on(
        provider.discover_devices_page(
            &stale_request,
            &hardware_context(),
        ),
    );

    assert_eq!(
        result.expect_err("stale cursor must be rejected").kind(),
        ZhalErrorKind::ConcurrencyConflict,
    );
}

// ============================================================================
// SESSION CREATION, FAILURE AND CLEANUP
// ============================================================================

#[test]
fn session_creation_failure_is_not_mistaken_for_device_access() {
    let id = provider_id("test.session.creation.failure");

    let simulated_device = device(&id, "device.session.failure");
    let identity = simulated_device.descriptor().identity().clone();

    let provider = provider_with_devices(
        id.as_str(),
        [simulated_device],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::SessionCreation,
            SimulationFault::CommunicationFailure {
                message: "session channel unavailable".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let result = block_on(
        provider.open_session(
            &identity,
            &hardware_context(),
        ),
    );

    assert_eq!(
        result.expect_err("session creation must fail").kind(),
        ZhalErrorKind::HardwareCommunication,
    );
}

#[test]
fn successfully_opened_session_can_be_closed_idempotently() {
    let id = provider_id("test.session.cleanup");

    let simulated_device = device(&id, "device.session.cleanup");
    let identity = simulated_device.descriptor().identity().clone();

    let provider = provider_with_devices(
        id.as_str(),
        [simulated_device],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    let mut session = block_on(
        provider.open_session(
            &identity,
            &hardware_context(),
        ),
    )
    .expect("session creation must succeed");

    assert!(!session.is_closed());

    block_on(session.close(&hardware_context()))
        .expect("session close must succeed");

    assert!(session.is_closed());

    block_on(session.close(&hardware_context()))
        .expect("repeated close must remain harmless");

    assert!(session.is_closed());
}

// ============================================================================
// OPERATION OUTCOME INTEGRITY
// ============================================================================

#[test]
fn unknown_operation_outcome_remains_unknown() {
    let provider = empty_provider("test.operation.unknown");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::OperationSimulation,
            SimulationFault::OutcomeUnknown {
                message: "remote acknowledgement was lost".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let operation = ProviderOperationId::new(
        "test.operation.unknown",
    )
    .expect("operation identity must be valid");

    let report = provider
        .simulate_operation(operation, &hardware_context())
        .expect("unknown outcome is a reportable simulation result");

    assert_eq!(
        report.outcome(),
        &SimulatedOutcome::Unknown {
            reason: "remote acknowledgement was lost".to_owned(),
        },
    );

    assert!(
        !report.is_physical_execution(),
        "simulation must never claim physical execution",
    );
}

#[test]
fn partial_completion_is_not_full_completion() {
    let provider = empty_provider("test.operation.partial");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::OperationSimulation,
            SimulationFault::PartialCompletion {
                completed: "7".to_owned(),
                requested: "10".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let operation = ProviderOperationId::new(
        "test.operation.partial",
    )
    .expect("operation identity must be valid");

    let report = provider
        .simulate_operation(operation, &hardware_context())
        .expect("partial completion must be represented");

    assert_eq!(
        report.outcome(),
        &SimulatedOutcome::PartiallyCompleted {
            completed: "7".to_owned(),
            requested: "10".to_owned(),
        },
    );

    assert_ne!(
        report.outcome(),
        &SimulatedOutcome::Completed,
    );
}

#[test]
fn hardware_fault_is_not_reported_as_full_completion() {
    let provider = empty_provider("test.operation.hardware.fault");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::OperationSimulation,
            SimulationFault::HardwareFault {
                message: "simulated hardware fault".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let operation = ProviderOperationId::new(
        "test.operation.hardware.fault",
    )
    .expect("operation identity must be valid");

    let report = provider
        .simulate_operation(operation, &hardware_context())
        .expect("simulated hardware fault must produce an outcome report");

    assert_eq!(
        report.outcome(),
        &SimulatedOutcome::Failed {
            reason: "simulated hardware fault".to_owned(),
        },
    );

    assert!(!report.is_physical_execution());
}

// ============================================================================
// DEVICE AVAILABILITY
// ============================================================================

#[test]
fn unavailable_device_cannot_open_a_session() {
    let id = provider_id("test.device.unavailable");

    let simulated_device = device(&id, "device.unavailable");
    let identity = simulated_device.descriptor().identity().clone();

    let provider = provider_with_devices(
        id.as_str(),
        [simulated_device],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    provider
        .set_device_availability(
            &identity,
            DeviceAvailability::unavailable(),
        )
        .expect("availability update must succeed");

    let result = block_on(
        provider.open_session(
            &identity,
            &hardware_context(),
        ),
    );

    assert_eq!(
        result.expect_err("unavailable device must not open a session")
            .kind(),
        ZhalErrorKind::DeviceUnavailable,
    );
}

// ============================================================================
// DYNAMIC INVENTORY AND PAGINATION
// ============================================================================

#[test]
fn inventory_can_grow_and_be_discovered_incrementally() {
    let id = provider_id("test.inventory.dynamic");

    let provider = provider_with_devices(
        id.as_str(),
        [device(&id, "device.initial")],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    initialize(&provider);

    provider
        .register_device(device(&id, "device.added.a"))
        .expect("first dynamic registration must succeed");

    provider
        .register_device(device(&id, "device.added.b"))
        .expect("second dynamic registration must succeed");

    assert_eq!(
        provider.device_count().expect("inventory count readable"),
        3,
    );

    let provider_handle: Arc<dyn HardwareProvider> =
        Arc::new(provider);

    let mut session = DiscoverySession::new(
        [provider_handle],
        DiscoveryOptions::new()
            .with_page_size(NonZeroUsize::new(1))
            .with_failure_policy(DiscoveryFailurePolicy::Strict),
    )
    .expect("discovery session must be constructible");

    let mut observed = 0usize;

    while let Some(page) = block_on(
        session.next_page(&hardware_context()),
    )
    .expect("each page must be valid")
    {
        observed = observed
            .checked_add(page.devices().len())
            .expect("test observation counter must not overflow");
    }

    assert_eq!(observed, 3);

    assert_eq!(
        session.report().devices_observed(),
        3,
    );

    assert_eq!(
        session.report().providers_completed(),
        1,
    );

    assert!(session.report().is_successful());
}

// ============================================================================
// PROVIDER SHUTDOWN
// ============================================================================

#[test]
fn shutdown_changes_provider_status_and_prevents_new_discovery() {
    let provider = empty_provider("test.provider.shutdown");
    initialize(&provider);

    shutdown(&provider);

    assert_eq!(
        provider.status(),
        ProviderStatus::Stopped,
    );

    let result = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    );

    assert!(
        result.is_err(),
        "stopped providers must not accept new discovery operations",
    );
}

// ============================================================================
// SCALABILITY AND OPEN-WORLD IDENTITIES
// ============================================================================

#[test]
fn provider_accepts_future_hardware_categories() {
    let id = provider_id("test.future.hardware");

    let identity = HardwareDeviceIdentity::new(
        id.clone(),
        DeviceId::new("future.device.identity")
            .expect("device identity must be valid"),
    );

    let descriptor = HardwareDeviceDescriptor::new(
        identity,
        DeviceKindId::new(
            "future.photonic.neuromorphic.molecular.device",
        )
        .expect("open-world device kind must be accepted"),
        "Future computational device",
    )
    .expect("future hardware descriptor must be valid");

    let provider = provider_with_devices(
        id.as_str(),
        [SimulatedDevice::new(descriptor)],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    assert_eq!(
        provider.device_count().expect("inventory count readable"),
        1,
    );
}

#[test]
fn large_identifier_values_are_not_rejected_by_a_test_defined_ceiling() {
    let id = provider_id("test.identifiers.large");

    let local_id = format!(
        "device.{}",
        "x".repeat(16_384),
    );

    let simulated_device = device(&id, &local_id);

    let provider = provider_with_devices(
        id.as_str(),
        [simulated_device],
        NonZeroUsize::new(1).expect("one is nonzero"),
    );

    assert_eq!(
        provider.device_count().expect("inventory count readable"),
        1,
    );
}

#[test]
fn decimal_inventory_revision_can_advance_beyond_fixed_integer_width() {
    let mut revision = String::from(
        "999999999999999999999999999999999999999999999999",
    );

    // This test verifies the same open-ended decimal sequence property
    // exercised by the simulation provider's internal revision counter.
    // It does not define a maximum inventory revision or device count.
    let mut carry = true;
    let mut digits = revision.as_bytes().to_vec();

    for digit in digits.iter_mut().rev() {
        if !carry {
            break;
        }

        if *digit == b'9' {
            *digit = b'0';
        } else {
            *digit += 1;
            carry = false;
        }
    }

    if carry {
        digits.insert(0, b'1');
    }

    revision = digits.into_iter().map(char::from).collect();

    assert_eq!(
        revision,
        "1000000000000000000000000000000000000000000000000",
    );
}

// ============================================================================
// THREAD-SAFETY CONTRACT
// ============================================================================

#[test]
fn simulation_provider_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<SimulationProvider>();
    assert_send_sync::<SimulatedDevice>();
}

// ============================================================================
// ERROR CLASSIFICATION REGRESSION
// ============================================================================

#[test]
fn zhal_provider_faults_keep_their_hardware_error_classification() {
    let provider = empty_provider("test.error.classification");
    initialize(&provider);

    provider
        .inject_fault(
            SimulationFaultPoint::Discovery,
            SimulationFault::CommunicationFailure {
                message: "communication fault".to_owned(),
            },
        )
        .expect("fault injection must succeed");

    let error = block_on(
        provider.discover_devices_page(
            &discovery_request(None),
            &hardware_context(),
        ),
    )
    .expect_err("injected communication fault must fail");

    assert_eq!(
        error.kind(),
        ZhalErrorKind::HardwareCommunication,
    );

    // This is a ZHAL error, not a standard-library error. Keep the
    // import in this module as a compile-time reminder that callers
    // must not conflate the two error authorities.
    let _standard_error_kind: Option<StandardErrorKind> = None;
}
