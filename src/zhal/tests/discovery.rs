
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZHAL — Hardware Discovery Tests
//!
//! Path: src/zhal/tests/discovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! Tests the existing provider-discovery and device-discovery
//! implementations. This file does not define another discovery API.
//!
//! Canonical authorities:
//! - stdlib::provider: provider identity, descriptor, status and trait.
//! - stdlib::capability: capability snapshots.
//! - stdlib::context: operation context and cancellation.
//! - stdlib::io::devices: DeviceId and DeviceKindId.
//! - zhal::providers::contract: HardwareProvider and page contracts.
//! - zhal::providers::discovery: provider pagination and coordination.
//! - zhal::devices::discovery: device filtering and discovery sessions.
//! - zhal::model::device: canonical device descriptors.
//! - zhal::identity: provider-scoped hardware device identity.
//!
//! Discovery is observational. These tests do not treat discovery as
//! authorization, allocation, reservation, or an execution guarantee.
//!
//! Finite test workloads are regression samples, not universal limits.
//! Growth is bounded by actual memory and representational resources.
//! Literal infinite physical storage or computation is not promised.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

use crate::stdlib::capability::CapabilitySnapshot;
use crate::stdlib::context::OperationContext;
use crate::stdlib::error::Error;
use crate::stdlib::io::devices::{DeviceId, DeviceKindId};
use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderKind,
    ProviderStatus,
};
use crate::stdlib::version::Version;

use crate::zhal::context::ZhalContext;
use crate::zhal::errors::{ZhalError, ZhalErrorKind, ZhalResult};
use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceIdentity,
    HardwareSessionIdentity,
};
use crate::zhal::model::device::{
    DeviceAvailability,
    HardwareDeviceDescriptor,
};
use crate::zhal::model::interface::HardwareInterfaceDescriptor;
use crate::zhal::providers::contract::{
    DeviceDiscoveryCursor,
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
};
use crate::zhal::providers::discovery::{
    DiscoveryFailurePhase,
    DiscoveryFailurePolicy,
    DiscoveryOptions,
    DiscoverySession,
};
use crate::zhal::devices::discovery::{
    DeviceAvailabilityFilter,
    DeviceDiscoveryFilter,
    DeviceDiscoveryOptions,
    DeviceDiscoverySession,
};

// ============================================================================
// SAFE, MINIMAL FUTURE EXECUTOR
// ============================================================================

/// A safe no-op waker for immediately completing test futures.
///
/// The test provider futures are deliberately non-blocking and do not
/// depend on an external executor. If a future returns Pending, block_on
/// yields the current thread and polls again.
struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {}
}

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
// FIXTURE CONSTRUCTION
// ============================================================================

fn version(value: &str) -> Version {
    Version::parse(value).expect("test version must be valid")
}

fn provider_id(value: &str) -> ProviderId {
    ProviderId::new(value).expect("test provider identity must be valid")
}

fn provider_descriptor(id: &str) -> ProviderDescriptor {
    ProviderDescriptor::new(
        provider_id(id),
        ProviderKind::new("hardware.provider")
            .expect("valid open-world provider kind"),
        format!("Test provider {id}"),
        version("1.0.0"),
        version("1.0.0"),
    )
    .expect("valid provider descriptor")
}

fn device(
    provider: &str,
    local_id: &str,
    category: &str,
    availability: DeviceAvailability,
) -> HardwareDeviceDescriptor {
    let identity = HardwareDeviceIdentity::new(
        provider_id(provider),
        DeviceId::new(local_id).expect("valid device identity"),
    );

    HardwareDeviceDescriptor::new(
        identity,
        DeviceKindId::new(category).expect("valid open-world category"),
        format!("Test device {local_id}"),
    )
    .expect("valid device descriptor")
    .with_availability(availability)
    .expect("valid availability observation")
}

fn available_device(
    provider: &str,
    local_id: &str,
    category: &str,
) -> HardwareDeviceDescriptor {
    device(
        provider,
        local_id,
        category,
        DeviceAvailability::available(),
    )
}

fn unknown_device(
    provider: &str,
    local_id: &str,
    category: &str,
) -> HardwareDeviceDescriptor {
    device(
        provider,
        local_id,
        category,
        DeviceAvailability::unknown(),
    )
}

fn page(
    devices: Vec<HardwareDeviceDescriptor>,
    cursor: Option<&str>,
    revision: Option<&str>,
) -> DeviceDiscoveryPage {
    DeviceDiscoveryPage::new(
        devices,
        cursor.map(|value| {
            DeviceDiscoveryCursor::new(value)
                .expect("valid provider-owned cursor")
        }),
        revision.map(str::to_owned),
    )
    .expect("valid provider discovery page")
}

fn context() -> ZhalContext {
    ZhalContext::builder()
        .with_component("zhal.tests.discovery")
        .expect("valid component")
        .with_operation("hardware.discovery")
        .expect("valid operation")
        .build()
}

fn provider_error(message: &str) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::ProviderFailure,
        message,
    )
    .in_component("zhal.tests.discovery")
}

// ============================================================================
// TEST PROVIDER
// ============================================================================

/// A deterministic, in-memory hardware provider used only by these tests.
///
/// Pages are supplied by the test. The fixture does not represent
/// physical hardware or claim that simulated observations are real.
struct TestHardwareProvider {
    descriptor: ProviderDescriptor,
    status: ProviderStatus,
    pages: Mutex<VecDeque<ZhalResult<DeviceDiscoveryPage>>>,
    requests: Mutex<Vec<DeviceDiscoveryRequest>>,
    discovery_calls: AtomicUsize,
}

impl TestHardwareProvider {
    fn new(
        id: &str,
        pages: impl IntoIterator<Item = ZhalResult<DeviceDiscoveryPage>>,
    ) -> Self {
        Self {
            descriptor: provider_descriptor(id),
            status: ProviderStatus::Ready,
            pages: Mutex::new(pages.into_iter().collect()),
            requests: Mutex::new(Vec::new()),
            discovery_calls: AtomicUsize::new(0),
        }
    }

    fn with_status(mut self, status: ProviderStatus) -> Self {
        self.status = status;
        self
    }

    fn calls(&self) -> usize {
        self.discovery_calls.load(Ordering::SeqCst)
    }

    fn requests(&self) -> Vec<DeviceDiscoveryRequest> {
        self.requests
            .lock()
            .expect("request lock must not be poisoned")
            .clone()
    }
}

impl Provider for TestHardwareProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    fn status(&self) -> ProviderStatus {
        self.status
    }

    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            context.check_cancelled()?;
            Ok(CapabilitySnapshot::new())
        })
    }
}

impl HardwareProvider for TestHardwareProvider {
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        _context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        Box::pin(async move {
            self.discovery_calls.fetch_add(1, Ordering::SeqCst);

            self.requests
                .lock()
                .map_err(|_| {
                    ZhalError::new(
                        ZhalErrorKind::Internal,
                        "test request lock is poisoned",
                    )
                })?
                .push(request.clone());

            let mut pages = self.pages.lock().map_err(|_| {
                ZhalError::new(
                    ZhalErrorKind::Internal,
                    "test page lock is poisoned",
                )
            })?;

            match pages.pop_front() {
                Some(result) => result,
                None => Ok(page(Vec::new(), None, Some("empty"))),
            }
        })
    }

    fn inspect_device<'a>(
        &'a self,
        _identity: &'a HardwareDeviceIdentity,
        _context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor> {
        Box::pin(async move {
            Err(ZhalError::new(
                ZhalErrorKind::DeviceNotFound,
                "inspection is not implemented by this discovery fixture",
            ))
        })
    }

    fn describe_interface<'a>(
        &'a self,
        _identity: &'a HardwareInterfaceIdentity,
        _context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor> {
        Box::pin(async move {
            Err(ZhalError::new(
                ZhalErrorKind::Other,
                "interface description is not implemented by this discovery fixture",
            ))
        })
    }

    fn open_session<'a>(
        &'a self,
        _device: &'a HardwareDeviceIdentity,
        _context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        Box::pin(async move {
            Err(ZhalError::new(
                ZhalErrorKind::Other,
                "session creation is not implemented by this discovery fixture",
            ))
        })
    }
}

fn provider_handle(
    provider: TestHardwareProvider,
) -> Arc<dyn HardwareProvider> {
    Arc::new(provider)
}

fn successful_page(
    devices: Vec<HardwareDeviceDescriptor>,
) -> ZhalResult<DeviceDiscoveryPage> {
    Ok(page(devices, None, Some("revision.1")))
}

// ============================================================================
// PROVIDER DISCOVERY — EMPTY AND BASIC CONTRACTS
// ============================================================================

#[test]
fn empty_provider_set_is_a_valid_completed_discovery() {
    let mut session = DiscoverySession::new(
        Vec::<Arc<dyn HardwareProvider>>::new(),
        DiscoveryOptions::default(),
    )
    .expect("empty provider set is valid");

    assert_eq!(session.report().provider_count(), 0);
    assert!(session.is_complete());
    assert!(session.report().is_successful());

    let result = block_on(session.next_page(&context()))
        .expect("empty discovery must succeed");

    assert!(result.is_none());
}

#[test]
fn one_provider_returns_one_page_incrementally() {
    let provider = TestHardwareProvider::new(
        "tests.provider.one",
        [successful_page(vec![
            available_device(
                "tests.provider.one",
                "device.one",
                "classical.processor",
            ),
        ])],
    );

    let provider = provider_handle(provider);

    let mut session = DiscoverySession::new(
        [Arc::clone(&provider)],
        DiscoveryOptions::default(),
    )
    .expect("session creation succeeds");

    let first = block_on(session.next_page(&context()))
        .expect("discovery succeeds")
        .expect("one page is available");

    assert_eq!(first.provider_id().as_str(), "tests.provider.one");
    assert_eq!(first.page_index(), 0);
    assert_eq!(first.devices().len(), 1);
    assert!(first.is_complete());

    assert_eq!(session.report().pages_observed(), 1);
    assert_eq!(session.report().devices_observed(), 1);
    assert!(session.is_complete());

    let second = block_on(session.next_page(&context()))
        .expect("completed discovery remains successful");

    assert!(second.is_none());
}

#[test]
fn provider_page_size_is_an_optional_operational_request() {
    use std::num::NonZeroUsize;

    let requested = NonZeroUsize::new(7).expect("nonzero");

    let provider = TestHardwareProvider::new(
        "tests.provider.page-size",
        [successful_page(Vec::new())],
    );

    let provider = Arc::new(provider);

    let mut session = DiscoverySession::new(
        [provider.clone() as Arc<dyn HardwareProvider>],
        DiscoveryOptions::new().with_page_size(Some(requested)),
    )
    .expect("session creation succeeds");

    let _ = block_on(session.next_page(&context()))
        .expect("discovery succeeds");

    let requests = provider.requests();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].page_size(), Some(requested));
}

// ============================================================================
// PROVIDER DISCOVERY — DETERMINISM AND FAIR PROGRESSION
// ============================================================================

#[test]
fn providers_are_enumerated_in_canonical_identity_order() {
    let alpha = provider_handle(TestHardwareProvider::new(
        "tests.provider.alpha",
        [successful_page(vec![
            available_device(
                "tests.provider.alpha",
                "alpha.device",
                "classical.processor",
            ),
        ])],
    ));

    let zeta = provider_handle(TestHardwareProvider::new(
        "tests.provider.zeta",
        [successful_page(vec![
            available_device(
                "tests.provider.zeta",
                "zeta.device",
                "quantum.processor",
            ),
        ])],
    ));

    // Deliberately provide the handles in reverse order.
    let mut session = DiscoverySession::new(
        [zeta, alpha],
        DiscoveryOptions::default(),
    )
    .expect("unique providers");

    let first = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    let second = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(first.provider_id().as_str(), "tests.provider.alpha");
    assert_eq!(second.provider_id().as_str(), "tests.provider.zeta");
}

#[test]
fn discovery_progresses_round_robin_across_paginated_providers() {
    let alpha = Arc::new(TestHardwareProvider::new(
        "tests.provider.alpha",
        [
            Ok(page(
                vec![available_device(
                    "tests.provider.alpha",
                    "alpha.1",
                    "classical.processor",
                )],
                Some("alpha.cursor.1"),
                Some("alpha.revision"),
            )),
            successful_page(vec![available_device(
                "tests.provider.alpha",
                "alpha.2",
                "classical.processor",
            )]),
        ],
    ));

    let beta = Arc::new(TestHardwareProvider::new(
        "tests.provider.beta",
        [
            Ok(page(
                vec![available_device(
                    "tests.provider.beta",
                    "beta.1",
                    "accelerator.gpu",
                )],
                Some("beta.cursor.1"),
                Some("beta.revision"),
            )),
            successful_page(vec![available_device(
                "tests.provider.beta",
                "beta.2",
                "accelerator.gpu",
            )]),
        ],
    ));

    let mut session = DiscoverySession::new(
        [
            alpha.clone() as Arc<dyn HardwareProvider>,
            beta.clone() as Arc<dyn HardwareProvider>,
        ],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let first = block_on(session.next_page(&context())).unwrap().unwrap();
    let second = block_on(session.next_page(&context())).unwrap().unwrap();
    let third = block_on(session.next_page(&context())).unwrap().unwrap();
    let fourth = block_on(session.next_page(&context())).unwrap().unwrap();

    assert_eq!(first.provider_id().as_str(), "tests.provider.alpha");
    assert_eq!(second.provider_id().as_str(), "tests.provider.beta");
    assert_eq!(third.provider_id().as_str(), "tests.provider.alpha");
    assert_eq!(fourth.provider_id().as_str(), "tests.provider.beta");

    assert_eq!(first.page_index(), 0);
    assert_eq!(second.page_index(), 0);
    assert_eq!(third.page_index(), 1);
    assert_eq!(fourth.page_index(), 1);

    assert_eq!(alpha.calls(), 2);
    assert_eq!(beta.calls(), 2);
    assert!(session.is_complete());
}

#[test]
fn device_order_within_a_page_is_canonical() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.order",
        [successful_page(vec![
            available_device(
                "tests.provider.order",
                "device.z",
                "classical.processor",
            ),
            available_device(
                "tests.provider.order",
                "device.a",
                "classical.processor",
            ),
            available_device(
                "tests.provider.order",
                "device.m",
                "classical.processor",
            ),
        ])],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    let ids: Vec<_> = result
        .devices()
        .iter()
        .map(|device| device.device_id().as_str())
        .collect();

    assert_eq!(ids, vec!["device.a", "device.m", "device.z"]);
}

// ============================================================================
// PROVIDER DISCOVERY — FAILURE ISOLATION AND STRICTNESS
// ============================================================================

#[test]
fn best_effort_records_a_provider_failure_and_continues() {
    let failing = provider_handle(TestHardwareProvider::new(
        "tests.provider.failing",
        [Err(provider_error("simulated discovery failure"))],
    ));

    let healthy = provider_handle(TestHardwareProvider::new(
        "tests.provider.healthy",
        [successful_page(vec![available_device(
            "tests.provider.healthy",
            "healthy.device",
            "classical.processor",
        )])],
    ));

    let mut session = DiscoverySession::new(
        [failing, healthy],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .expect("best-effort mode continues")
        .expect("healthy provider returns a page");

    assert_eq!(result.provider_id().as_str(), "tests.provider.healthy");

    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(session.report().providers_completed(), 1);
    assert!(session.report().has_failures());
    assert!(!session.report().is_successful());
}

#[test]
fn best_effort_preserves_provider_failure_identity_and_phase() {
    let failing = provider_handle(TestHardwareProvider::new(
        "tests.provider.failure-detail",
        [Err(provider_error("failure detail"))],
    ));

    let mut session = DiscoverySession::new(
        [failing],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .expect("best-effort provider failure is recorded");

    assert!(result.is_none());

    let failures = session.report().failures();

    assert_eq!(failures.len(), 1);
    assert_eq!(
        failures[0].provider_id().as_str(),
        "tests.provider.failure-detail",
    );
    assert_eq!(
        failures[0].phase(),
        DiscoveryFailurePhase::PageDiscovery,
    );
    assert_eq!(
        failures[0].error().kind(),
        ZhalErrorKind::ProviderFailure,
    );
}

#[test]
fn strict_mode_terminates_on_the_first_provider_failure() {
    let failing = Arc::new(TestHardwareProvider::new(
        "tests.provider.strict-failure",
        [Err(provider_error("strict failure"))],
    ));

    let later = Arc::new(TestHardwareProvider::new(
        "tests.provider.strict-later",
        [successful_page(vec![available_device(
            "tests.provider.strict-later",
            "must.not.be.queried",
            "classical.processor",
        )])],
    ));

    let mut session = DiscoverySession::new(
        [
            failing.clone() as Arc<dyn HardwareProvider>,
            later.clone() as Arc<dyn HardwareProvider>,
        ],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::Strict),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()));

    assert!(result.is_err());
    assert!(session.is_terminated());
    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(failing.calls(), 1);
    assert_eq!(later.calls(), 0);

    // A terminated session must not continue issuing provider calls.
    assert!(block_on(session.next_page(&context()))
        .expect("terminated session returns no further page")
        .is_none());

    assert_eq!(later.calls(), 0);
}

#[test]
fn unavailable_provider_is_not_treated_as_discoverable() {
    let provider = provider_handle(
        TestHardwareProvider::new(
            "tests.provider.unavailable",
            [successful_page(Vec::new())],
        )
        .with_status(ProviderStatus::Unavailable),
    );

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .expect("best-effort unavailable provider is recorded");

    assert!(result.is_none());
    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(
        session.report().failures()[0].phase(),
        DiscoveryFailurePhase::ProviderReadiness,
    );
}

// ============================================================================
// PROVIDER DISCOVERY — PAGINATION INTEGRITY
// ============================================================================

#[test]
fn continuation_cursor_is_passed_back_to_its_originating_provider() {
    let provider = Arc::new(TestHardwareProvider::new(
        "tests.provider.cursor",
        [
            Ok(page(
                vec![available_device(
                    "tests.provider.cursor",
                    "first",
                    "classical.processor",
                )],
                Some("cursor.next"),
                Some("revision.7"),
            )),
            successful_page(vec![available_device(
                "tests.provider.cursor",
                "second",
                "classical.processor",
            )]),
        ],
    ));

    let mut session = DiscoverySession::new(
        [provider.clone() as Arc<dyn HardwareProvider>],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let first = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(first.next_cursor().map(|c| c.as_str()), Some("cursor.next"));
    assert_eq!(first.snapshot_revision(), Some("revision.7"));

    let second = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert!(second.is_complete());

    let requests = provider.requests();

    assert_eq!(requests.len(), 2);
    assert!(requests[0].cursor().is_none());
    assert_eq!(
        requests[1].cursor().map(|cursor| cursor.as_str()),
        Some("cursor.next"),
    );
}

#[test]
fn repeated_continuation_cursor_is_rejected_as_a_pagination_cycle() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.cursor-cycle",
        [
            Ok(page(Vec::new(), Some("cursor.a"), None)),
            Ok(page(Vec::new(), Some("cursor.b"), None)),
            Ok(page(Vec::new(), Some("cursor.a"), None)),
        ],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    assert!(block_on(session.next_page(&context())).unwrap().is_some());
    assert!(block_on(session.next_page(&context())).unwrap().is_some());

    // The third page repeats a cursor already seen in this provider stream.
    let result = block_on(session.next_page(&context()))
        .expect("best-effort cycle is reported rather than propagated");

    assert!(result.is_none());
    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(
        session.report().failures()[0].phase(),
        DiscoveryFailurePhase::PaginationValidation,
    );
}

#[test]
fn duplicate_device_across_pages_is_rejected() {
    let repeated = available_device(
        "tests.provider.duplicate",
        "same.device",
        "classical.processor",
    );

    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.duplicate",
        [
            Ok(page(
                vec![repeated.clone()],
                Some("cursor.next"),
                None,
            )),
            successful_page(vec![repeated]),
        ],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    assert!(block_on(session.next_page(&context())).unwrap().is_some());

    let result = block_on(session.next_page(&context()))
        .expect("duplicate is recorded in best-effort mode");

    assert!(result.is_none());
    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(
        session.report().failures()[0].phase(),
        DiscoveryFailurePhase::DeviceValidation,
    );
}

#[test]
fn duplicate_provider_identity_is_rejected_before_discovery() {
    let first = provider_handle(TestHardwareProvider::new(
        "tests.provider.duplicate-id",
        [successful_page(Vec::new())],
    ));

    let second = provider_handle(TestHardwareProvider::new(
        "tests.provider.duplicate-id",
        [successful_page(Vec::new())],
    ));

    let result = DiscoverySession::new(
        [first, second],
        DiscoveryOptions::default(),
    );

    assert!(result.is_err());
}

// ============================================================================
// PROVIDER DISCOVERY — INVALID PROVIDER OBSERVATIONS
// ============================================================================

#[test]
fn provider_cannot_return_a_device_owned_by_another_provider() {
    let foreign_device = available_device(
        "tests.provider.foreign",
        "foreign.device",
        "classical.processor",
    );

    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.owner",
        [successful_page(vec![foreign_device])],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .expect("foreign device is handled as a provider contract failure");

    assert!(result.is_none());
    assert_eq!(session.report().providers_failed(), 1);
    assert_eq!(
        session.report().failures()[0].phase(),
        DiscoveryFailurePhase::PageDiscovery,
    );
}

#[test]
fn unknown_device_availability_is_preserved() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.unknown",
        [successful_page(vec![unknown_device(
            "tests.provider.unknown",
            "unknown.device",
            "future.photonic.processor",
        )])],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(result.devices().len(), 1);
    assert!(result.devices()[0].availability().is_unknown());
    assert!(!result.devices()[0].is_available());
}

// ============================================================================
// DEVICE DISCOVERY — FILTERING AND OBSERVATION
// ============================================================================

#[test]
fn device_filter_combines_provider_category_and_availability() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.filter",
        [successful_page(vec![
            available_device(
                "tests.provider.filter",
                "cpu",
                "classical.processor",
            ),
            available_device(
                "tests.provider.filter",
                "gpu",
                "accelerator.gpu",
            ),
            unknown_device(
                "tests.provider.filter",
                "unknown.cpu",
                "classical.processor",
            ),
        ])],
    ));

    let filter = DeviceDiscoveryFilter::new()
        .with_provider(provider_id("tests.provider.filter"))
        .with_category(
            DeviceKindId::new("classical.processor").unwrap(),
        )
        .with_availability(DeviceAvailabilityFilter::Available);

    let options = DeviceDiscoveryOptions::new().with_filter(filter);

    let mut session = DeviceDiscoverySession::new(
        [provider],
        options,
    )
    .expect("valid device discovery options");

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result.observed_devices(), 3);
    assert_eq!(result.filtered_devices(), 2);
    assert_eq!(
        result.devices()[0].device_id().as_str(),
        "cpu",
    );
}

#[test]
fn unknown_availability_is_not_included_by_available_filter() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.unknown-filter",
        [successful_page(vec![
            unknown_device(
                "tests.provider.unknown-filter",
                "unknown",
                "future.photonic.processor",
            ),
        ])],
    ));

    let filter = DeviceDiscoveryFilter::new()
        .with_availability(DeviceAvailabilityFilter::Available);

    let mut session = DeviceDiscoverySession::new(
        [provider],
        DeviceDiscoveryOptions::new().with_filter(filter),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert!(result.is_empty());
    assert_eq!(result.observed_devices(), 1);
    assert_eq!(result.filtered_devices(), 1);
}

#[test]
fn provider_defined_availability_states_are_filterable() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.custom-state",
        [successful_page(vec![
            device(
                "tests.provider.custom-state",
                "maintenance.device",
                "future.optical.processor",
                DeviceAvailability::new(
                    "provider_specific.maintenance",
                    Some("scheduled maintenance".to_owned()),
                )
                .unwrap(),
            ),
            available_device(
                "tests.provider.custom-state",
                "available.device",
                "future.optical.processor",
            ),
        ])],
    ));

    let filter = DeviceDiscoveryFilter::new()
        .with_availability(DeviceAvailabilityFilter::State(
            "provider_specific.maintenance".to_owned(),
        ));

    let mut session = DeviceDiscoverySession::new(
        [provider],
        DeviceDiscoveryOptions::new().with_filter(filter),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.devices()[0].availability().state(),
        "provider_specific.maintenance",
    );
}

#[test]
fn invalid_custom_availability_filter_is_rejected() {
    let filter = DeviceDiscoveryFilter::new()
        .with_availability(DeviceAvailabilityFilter::State(
            " ".to_owned(),
        ));

    let result = DeviceDiscoverySession::new(
        Vec::<Arc<dyn HardwareProvider>>::new(),
        DeviceDiscoveryOptions::new().with_filter(filter),
    );

    assert!(result.is_err());
}

#[test]
fn empty_filtered_page_can_still_have_a_continuation_cursor() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.empty-filtered-page",
        [
            Ok(page(
                vec![available_device(
                    "tests.provider.empty-filtered-page",
                    "gpu",
                    "accelerator.gpu",
                )],
                Some("cursor.more"),
                Some("revision.2"),
            )),
            successful_page(vec![available_device(
                "tests.provider.empty-filtered-page",
                "cpu",
                "classical.processor",
            )]),
        ],
    ));

    let filter = DeviceDiscoveryFilter::new()
        .with_category(
            DeviceKindId::new("classical.processor").unwrap(),
        );

    let mut session = DeviceDiscoverySession::new(
        [provider],
        DeviceDiscoveryOptions::new().with_filter(filter),
    )
    .unwrap();

    let first = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert!(first.is_empty());
    assert_eq!(first.observed_devices(), 1);
    assert_eq!(first.filtered_devices(), 1);
    assert_eq!(
        first.next_cursor().map(|cursor| cursor.as_str()),
        Some("cursor.more"),
    );
    assert!(!first.is_complete());

    let second = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(second.len(), 1);
    assert!(second.is_complete());
}

#[test]
fn device_discovery_preserves_provider_failure_reporting() {
    let failing = provider_handle(TestHardwareProvider::new(
        "tests.provider.device-failure",
        [Err(provider_error("device discovery failure"))],
    ));

    let mut session = DeviceDiscoverySession::new(
        [failing],
        DeviceDiscoveryOptions::new()
            .with_failure_policy(DiscoveryFailurePolicy::BestEffort),
    )
    .unwrap();

    let result = block_on(session.next_page(&context()))
        .expect("best-effort failure is reported");

    assert!(result.is_none());
    assert!(session.has_failures());
    assert_eq!(session.failures().len(), 1);
}

// ============================================================================
// SCALABILITY — DYNAMIC, PAGINATED INVENTORY
// ============================================================================

#[test]
fn discovery_handles_a_growing_paginated_inventory_without_a_global_cap() {
    // A finite stress workload used to exercise incremental growth.
    // This number is not a maximum supported device count.
    let workload_size = 513_usize;
    let page_width = 37_usize;

    let provider_name = "tests.provider.scalability";
    let mut pages = Vec::new();

    let mut start = 0_usize;

    while start < workload_size {
        let end = start.saturating_add(page_width).min(workload_size);

        let devices = (start..end)
            .map(|index| {
                available_device(
                    provider_name,
                    &format!("device.{index}"),
                    "future.reconfigurable.compute",
                )
            })
            .collect::<Vec<_>>();

        let next_cursor = if end < workload_size {
            Some(format!("cursor.{end}"))
        } else {
            None
        };

        pages.push(Ok(page(
            devices,
            next_cursor.as_deref(),
            Some("inventory.revision.1"),
        )));

        start = end;
    }

    let provider = provider_handle(TestHardwareProvider::new(
        provider_name,
        pages,
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let mut identities = Vec::new();

    loop {
        match block_on(session.next_page(&context()))
            .expect("each discovery page must be valid")
        {
            Some(page) => {
                identities.extend(
                    page.devices()
                        .iter()
                        .map(|device| device.identity().clone()),
                );
            }
            None => break,
        }
    }

    assert_eq!(identities.len(), workload_size);
    assert_eq!(session.report().devices_observed(), workload_size);
    assert_eq!(
        session.report().pages_observed(),
        workload_size.div_ceil(page_width),
    );
    assert!(session.is_complete());
    assert!(session.report().is_successful());
}

#[test]
fn discovery_does_not_accumulate_all_device_descriptors_in_its_report() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.incremental",
        [
            Ok(page(
                vec![available_device(
                    "tests.provider.incremental",
                    "first",
                    "classical.processor",
                )],
                Some("cursor.next"),
                None,
            )),
            successful_page(vec![available_device(
                "tests.provider.incremental",
                "second",
                "classical.processor",
            )]),
        ],
    ));

    let mut session = DiscoverySession::new(
        [provider],
        DiscoveryOptions::default(),
    )
    .unwrap();

    let first = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(first.devices().len(), 1);
    assert_eq!(session.report().devices_observed(), 1);

    // The caller owns the returned page. The report retains accounting
    // and failure information, not a second copy of the inventory.
    drop(first);

    let second = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(second.devices().len(), 1);
    assert_eq!(session.report().devices_observed(), 2);
}

// ============================================================================
// PROVIDER HANDLE LIFETIME
// ============================================================================

#[test]
fn discovery_session_retains_provider_handle_after_external_handle_drop() {
    let provider = provider_handle(TestHardwareProvider::new(
        "tests.provider.retained",
        [successful_page(vec![available_device(
            "tests.provider.retained",
            "retained.device",
            "classical.processor",
        )])],
    ));

    let mut session = DiscoverySession::new(
        [Arc::clone(&provider)],
        DiscoveryOptions::default(),
    )
    .unwrap();

    drop(provider);

    let result = block_on(session.next_page(&context()))
        .unwrap()
        .unwrap();

    assert_eq!(result.devices().len(), 1);
    assert!(session.is_complete());
}

// ============================================================================
// DOMAIN EXTENSIBILITY
// ============================================================================

#[test]
fn discovery_accepts_future_open_world_hardware_categories() {
    let categories = [
        "classical.processor",
        "quantum.processor",
        "hdl.fpga",
        "hdl.asic",
        "accelerator.gpu",
        "embedded.microcontroller",
        "distributed.remote_device",
        "future.photonic.processor",
        "future.neuromorphic.device",
        "future.molecular.computer",
    ];

    for (index, category) in categories.iter().enumerate() {
        let provider_name = format!("tests.provider.domain.{index}");

        let provider = provider_handle(TestHardwareProvider::new(
            &provider_name,
            [successful_page(vec![available_device(
                &provider_name,
                "device",
                category,
            )])],
        ));

        let mut session = DeviceDiscoverySession::new(
            [provider],
            DeviceDiscoveryOptions::new().with_filter(
                DeviceDiscoveryFilter::new().with_category(
                    DeviceKindId::new(*category).unwrap(),
                ),
            ),
        )
        .unwrap();

        let result = block_on(session.next_page(&context()))
            .unwrap()
            .unwrap();

        assert_eq!(result.len(), 1, "category: {category}");
        assert_eq!(result.devices()[0].category().as_str(), *category);
    }
}
