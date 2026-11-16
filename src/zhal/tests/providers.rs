
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! ZHAL provider integration contract tests.
//!
//! These tests exercise the existing standard-library provider contracts
//! that ZHAL must reuse. They intentionally do not introduce a second
//! provider trait, provider identity, lifecycle manager, capability
//! authority, cancellation mechanism, or resource allocator.
//!
//! Canonical dependencies:
//! - crate::stdlib::provider
//! - crate::stdlib::capability
//! - crate::stdlib::context
//! - crate::stdlib::version
//!
//! Domain-specific hardware authorities remain in their existing modules:
//! - crate::quantum::hardware
//! - crate::hdl::realization
//! - crate::stdlib::io::devices
//! - crate::zur
//! - crate::distributed
//!
//! Scalability:
//! Finite test workloads are verification samples, not universal limits.
//! This module defines no maximum provider count, device count, capability
//! count, operation count, hardware count, or resource quantity.
//!
//! Physical infinity is not promised. Supported scale is bounded by actual
//! resources, representational limits, and explicitly configured budgets.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use crate::stdlib::capability::{
    Capability,
    CapabilityEvidence,
    CapabilityId,
    CapabilityKind,
    CapabilitySnapshot,
    CapabilityState,
    CapabilityStatus,
};

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
};

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderKind,
    ProviderLifecycle,
    ProviderOperationDescriptor,
    ProviderOperationId,
    ProviderStatus,
};

use crate::stdlib::version::Version;

// ============================================================================
// TEST SUPPORT
// ============================================================================

fn version(value: &str) -> Version {
    Version::parse(value).expect("test version must be valid")
}

fn operation_context() -> OperationContext {
    OperationContext::builder()
        .with_component("zhal.tests.providers")
        .expect("valid component")
        .with_operation("provider.contract_test")
        .expect("valid operation")
        .build()
}

fn capability(identity: &str) -> Capability {
    Capability::new(
        CapabilityId::new(identity).expect("valid capability identity"),
        CapabilityKind::new("computation").expect("valid capability kind"),
    )
}

fn provider_descriptor() -> ProviderDescriptor {
    ProviderDescriptor::new(
        ProviderId::new("zamani.tests.hardware")
            .expect("valid provider identity"),
        ProviderKind::new("hardware.provider")
            .expect("valid provider kind"),
        "ZHAL Provider Contract Test",
        version("1.0.0"),
        version("1.0.0"),
    )
    .expect("valid provider descriptor")
}

/// A no-op waker for immediately-ready test futures.
struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

/// Polls a future expected to complete immediately and successfully.
///
/// This is deliberately not a general-purpose executor. A Pending result
/// indicates that the test fixture violated its immediate-completion
/// contract.
fn poll_ready<T>(mut future: ProviderFuture<'_, T>) -> T {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    match Future::poll(Pin::as_mut(&mut future), &mut context) {
        Poll::Ready(Ok(value)) => value,
        Poll::Ready(Err(error)) => {
            panic!("provider operation unexpectedly failed: {error}");
        }
        Poll::Pending => {
            panic!("test provider future unexpectedly returned Pending");
        }
    }
}

#[derive(Debug, Default)]
struct TestCancellation {
    cancelled: AtomicBool,
}

impl TestCancellation {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

impl CancellationSignal for TestCancellation {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

fn cancellable_context(
    signal: Arc<TestCancellation>,
) -> OperationContext {
    let cancellation: Arc<dyn CancellationSignal> = signal;

    OperationContext::builder()
        .with_component("zhal.tests.providers")
        .expect("valid component")
        .with_operation("provider.cancellation_test")
        .expect("valid operation")
        .with_cancellation(cancellation)
        .build()
}

/// Minimal fixture implementing the canonical provider contract.
///
/// This is test-only infrastructure, not a second production provider API.
struct TestProvider {
    descriptor: ProviderDescriptor,
    status: ProviderStatus,
    capabilities: CapabilitySnapshot,
}

impl TestProvider {
    fn new(
        descriptor: ProviderDescriptor,
        status: ProviderStatus,
        capabilities: CapabilitySnapshot,
    ) -> Self {
        Self {
            descriptor,
            status,
            capabilities,
        }
    }
}

impl Provider for TestProvider {
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
            Ok(self.capabilities.clone())
        })
    }
}

impl ProviderLifecycle for TestProvider {
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        Box::pin(async move {
            context.check_cancelled()?;
            Ok(())
        })
    }

    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        Box::pin(async move {
            context.check_cancelled()?;
            Ok(())
        })
    }
}

fn empty_provider() -> TestProvider {
    TestProvider::new(
        provider_descriptor(),
        ProviderStatus::Ready,
        CapabilitySnapshot::new(),
    )
}

/// Polls a future once and returns its result without assuming success.
fn poll_once<T>(
    mut future: ProviderFuture<'_, T>,
) -> Poll<crate::stdlib::error::Result<T>> {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);

    Future::poll(Pin::as_mut(&mut future), &mut context)
}

// ============================================================================
// 1. PROVIDER CONTRACT AND OBJECT SAFETY
// ============================================================================

#[test]
fn provider_contract_is_object_safe_and_shareable() {
    fn accepts_provider(_: &dyn Provider) {}
    fn accepts_send_sync<T: Send + Sync>() {}

    accepts_send_sync::<TestProvider>();

    let provider: Arc<dyn Provider> = Arc::new(empty_provider());

    accepts_provider(provider.as_ref());

    assert_eq!(
        provider.descriptor().id().as_str(),
        "zamani.tests.hardware"
    );
}

#[test]
fn provider_descriptor_is_stable_for_provider_instance() {
    let provider = empty_provider();

    assert_eq!(provider.descriptor(), provider.descriptor());

    assert_eq!(
        provider.descriptor().implementation_version(),
        &version("1.0.0")
    );

    assert_eq!(
        provider.descriptor().api_version(),
        &version("1.0.0")
    );
}

#[test]
fn provider_identity_and_kind_are_extensible() {
    let identities = [
        "classical.cpu",
        "quantum.qpu",
        "hdl.fpga",
        "accelerator.gpu",
        "distributed.remote",
        "embedded.microcontroller",
        "future.photonic.processor",
        "future.neuromorphic.system",
        "future.molecular.computing.provider",
    ];

    for identity in identities {
        let provider_id =
            ProviderId::new(identity).expect("valid provider identity");

        assert_eq!(provider_id.as_str(), identity);
    }

    let future_kind = ProviderKind::new("future.optical.compute")
        .expect("future provider kinds must remain extensible");

    assert_eq!(future_kind.as_str(), "future.optical.compute");
}

#[test]
fn malformed_provider_identities_are_rejected() {
    let invalid = [
        "",
        ".provider",
        "provider.",
        "provider..hardware",
        "provider hardware",
        " provider.hardware",
        "provider.hardware ",
    ];

    for identity in invalid {
        assert!(
            ProviderId::new(identity).is_err(),
            "invalid provider identity accepted: {identity}"
        );
    }
}

// ============================================================================
// 2. PROVIDER STATUS AND CAPABILITY AVAILABILITY
// ============================================================================

#[test]
fn provider_status_distinguishes_operational_states() {
    assert!(ProviderStatus::Ready.may_accept_operations());
    assert!(ProviderStatus::Degraded.may_accept_operations());

    assert!(!ProviderStatus::Unknown.may_accept_operations());
    assert!(!ProviderStatus::Initializing.may_accept_operations());
    assert!(!ProviderStatus::Unavailable.may_accept_operations());
    assert!(!ProviderStatus::Stopping.may_accept_operations());
    assert!(!ProviderStatus::Stopped.may_accept_operations());
    assert!(!ProviderStatus::Failed.may_accept_operations());
}

#[test]
fn ready_provider_does_not_imply_all_capabilities_are_available() {
    let provider = empty_provider();

    assert_eq!(provider.status(), ProviderStatus::Ready);

    let quantum = CapabilityId::new("quantum.execution").unwrap();

    let snapshot = poll_ready(
        provider.capability_snapshot(&operation_context())
    );

    assert_eq!(
        snapshot.status(&quantum),
        CapabilityStatus::Unknown
    );

    assert!(!snapshot.establishes_availability(&quantum));
}

#[test]
fn degraded_provider_can_report_capability_specific_states() {
    let mut snapshot = CapabilitySnapshot::new();

    snapshot
        .insert(CapabilityState::new(
            capability("classical.execution"),
            CapabilityStatus::Available,
        ))
        .expect("unique capability");

    snapshot
        .insert(CapabilityState::new(
            capability("quantum.execution"),
            CapabilityStatus::Conditional,
        ))
        .expect("unique capability");

    let provider = TestProvider::new(
        provider_descriptor(),
        ProviderStatus::Degraded,
        snapshot,
    );

    let observed = poll_ready(
        provider.capability_snapshot(&operation_context())
    );

    let classical =
        CapabilityId::new("classical.execution").unwrap();

    let quantum =
        CapabilityId::new("quantum.execution").unwrap();

    assert_eq!(
        observed.status(&classical),
        CapabilityStatus::Available
    );

    assert_eq!(
        observed.status(&quantum),
        CapabilityStatus::Conditional
    );
}

#[test]
fn capability_states_preserve_unknown_and_unavailable() {
    let mut snapshot = CapabilitySnapshot::new();

    let states = [
        ("classical.execution", CapabilityStatus::Available),
        ("quantum.execution", CapabilityStatus::Conditional),
        ("hdl.synthesis", CapabilityStatus::Unavailable),
        ("future.photonic.execution", CapabilityStatus::Unknown),
    ];

    for (identity, status) in states {
        snapshot
            .insert(CapabilityState::new(
                capability(identity),
                status,
            ))
            .expect("unique capability");
    }

    let provider = TestProvider::new(
        provider_descriptor(),
        ProviderStatus::Ready,
        snapshot,
    );

    let observed = poll_ready(
        provider.capability_snapshot(&operation_context())
    );

    for (identity, expected) in states {
        let id = CapabilityId::new(identity).unwrap();

        assert_eq!(observed.status(&id), expected);

        assert_eq!(
            observed.establishes_availability(&id),
            expected == CapabilityStatus::Available
        );
    }
}

// ============================================================================
// 3. DECLARED CAPABILITIES VERSUS OBSERVED CAPABILITIES
// ============================================================================

#[test]
fn provider_declaration_does_not_establish_live_availability() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_capability(capability("quantum.execution"))
        .expect("capability declaration");

    assert!(
        descriptor
            .capability(
                &CapabilityId::new("quantum.execution").unwrap()
            )
            .is_some()
    );

    let provider = TestProvider::new(
        descriptor,
        ProviderStatus::Ready,
        CapabilitySnapshot::new(),
    );

    let observed = poll_ready(
        provider.capability_snapshot(&operation_context())
    );

    let quantum =
        CapabilityId::new("quantum.execution").unwrap();

    assert_eq!(
        observed.status(&quantum),
        CapabilityStatus::Unknown
    );

    assert!(!observed.establishes_availability(&quantum));
}

#[test]
fn capability_evidence_is_preserved() {
    let evidence = CapabilityEvidence::new(
        "provider.probe",
        "Provider reported the capability during this observation",
    )
    .expect("valid evidence");

    let state = CapabilityState::new(
        capability("accelerator.tensor_execution"),
        CapabilityStatus::Available,
    )
    .with_evidence(evidence.clone());

    assert_eq!(state.evidence().len(), 1);
    assert_eq!(state.evidence()[0], evidence);
    assert_eq!(state.status(), CapabilityStatus::Available);
}

// ============================================================================
// 4. OPEN-WORLD PROVIDER OPERATIONS
// ============================================================================

#[test]
fn provider_operation_identities_are_extensible() {
    let identities = [
        "classical.matrix_multiplication",
        "quantum.circuit_execution",
        "hdl.synthesis",
        "distributed.collective_operation",
        "ai.model_inference",
        "future.photonic.transduction",
        "future.molecular.compute",
    ];

    for identity in identities {
        let operation = ProviderOperationId::new(identity)
            .expect("valid extensible operation identity");

        assert_eq!(operation.as_str(), identity);
    }
}

#[test]
fn provider_descriptor_supports_multiple_domain_operations() {
    let mut descriptor = provider_descriptor();

    let identities = [
        "classical.execution",
        "quantum.circuit_execution",
        "hdl.synthesis",
        "distributed.execution",
        "future.photonic.execution",
    ];

    for identity in identities {
        descriptor
            .insert_operation(
                ProviderOperationDescriptor::from_id(
                    identity,
                    version("1.0.0"),
                )
                .expect("valid operation descriptor"),
            )
            .expect("unique operation");
    }

    assert_eq!(descriptor.operations().len(), identities.len());

    for identity in identities {
        let id = ProviderOperationId::new(identity).unwrap();

        assert!(descriptor.operation(&id).is_some());
    }
}

#[test]
fn duplicate_operation_declarations_are_rejected() {
    let mut descriptor = provider_descriptor();

    let first = ProviderOperationDescriptor::from_id(
        "quantum.circuit_execution",
        version("1.0.0"),
    )
    .unwrap();

    let duplicate = ProviderOperationDescriptor::from_id(
        "quantum.circuit_execution",
        version("2.0.0"),
    )
    .unwrap();

    descriptor
        .insert_operation(first)
        .expect("first declaration");

    assert!(descriptor.insert_operation(duplicate).is_err());
    assert_eq!(descriptor.operations().len(), 1);
}

#[test]
fn common_operation_contract_does_not_replace_domain_payloads() {
    let operation = ProviderOperationDescriptor::from_id(
        "quantum.circuit_execution",
        version("1.0.0"),
    )
    .unwrap();

    assert_eq!(
        operation.id().as_str(),
        "quantum.circuit_execution"
    );

    // The common provider contract identifies the operation.
    // Quantum circuit payloads and results remain owned by the
    // existing quantum hardware execution interfaces.
    assert!(operation.request_contract().is_none());
    assert!(operation.response_contract().is_none());
}

// ============================================================================
// 5. VERSIONING AND PROVIDER METADATA
// ============================================================================

#[test]
fn provider_implementation_and_api_versions_are_distinct() {
    let descriptor = ProviderDescriptor::new(
        ProviderId::new("future.provider").unwrap(),
        ProviderKind::new("future.computation").unwrap(),
        "Future Provider",
        version("7.4.2"),
        version("2.1.0"),
    )
    .unwrap();

    assert_eq!(
        descriptor.implementation_version(),
        &version("7.4.2")
    );

    assert_eq!(
        descriptor.api_version(),
        &version("2.1.0")
    );
}

#[test]
fn duplicate_provider_metadata_is_rejected() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_attribute("hardware.family", "future-family")
        .expect("first attribute");

    assert!(
        descriptor
            .insert_attribute("hardware.family", "replacement")
            .is_err()
    );

    assert_eq!(
        descriptor.attribute("hardware.family"),
        Some("future-family")
    );
}

#[test]
fn provider_metadata_can_be_explicitly_replaced() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_attribute("hardware.revision", "revision-a")
        .expect("initial metadata");

    descriptor
        .replace_attribute("hardware.revision", "revision-b")
        .expect("explicit replacement");

    assert_eq!(
        descriptor.attribute("hardware.revision"),
        Some("revision-b")
    );
}

// ============================================================================
// 6. CANCELLATION AND ERROR PROPAGATION
// ============================================================================

#[test]
fn capability_observation_propagates_cancellation() {
    let provider = empty_provider();
    let signal = Arc::new(TestCancellation::default());
    let context = cancellable_context(Arc::clone(&signal));

    signal.cancel();

    match poll_once(provider.capability_snapshot(&context)) {
        Poll::Ready(Err(_)) => {}
        Poll::Ready(Ok(_)) => {
            panic!("provider ignored a cancellation request");
        }
        Poll::Pending => {
            panic!("test provider should complete immediately");
        }
    }
}

#[test]
fn provider_initialization_and_shutdown_propagate_cancellation() {
    let provider = empty_provider();
    let signal = Arc::new(TestCancellation::default());
    let context = cancellable_context(Arc::clone(&signal));

    signal.cancel();

    match poll_once(provider.initialize(&context)) {
        Poll::Ready(Err(_)) => {}
        Poll::Ready(Ok(())) => {
            panic!("provider initialization ignored cancellation");
        }
        Poll::Pending => {
            panic!("test initialization should complete immediately");
        }
    }

    match poll_once(provider.shutdown(&context)) {
        Poll::Ready(Err(_)) => {}
        Poll::Ready(Ok(())) => {
            panic!("provider shutdown ignored cancellation");
        }
        Poll::Pending => {
            panic!("test shutdown should complete immediately");
        }
    }
}

#[test]
fn provider_lifecycle_succeeds_without_cancellation() {
    let provider = empty_provider();
    let context = operation_context();

    poll_ready(provider.initialize(&context));
    poll_ready(provider.shutdown(&context));
}

// ============================================================================
// 7. CONCURRENT READ-ONLY OBSERVATION
// ============================================================================

#[test]
fn provider_supports_concurrent_read_only_observation() {
    let mut snapshot = CapabilitySnapshot::new();

    snapshot
        .insert(CapabilityState::new(
            capability("classical.execution"),
            CapabilityStatus::Available,
        ))
        .expect("unique capability");

    let provider = Arc::new(TestProvider::new(
        provider_descriptor(),
        ProviderStatus::Ready,
        snapshot,
    ));

    let handles = (0..8)
        .map(|_| {
            let provider = Arc::clone(&provider);

            std::thread::spawn(move || {
                let context = operation_context();

                let observed = poll_ready(
                    provider.capability_snapshot(&context)
                );

                assert_eq!(observed.len(), 1);
                assert_eq!(provider.status(), ProviderStatus::Ready);
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().expect("provider observation thread");
    }
}

// ============================================================================
// 8. EXTENSIBILITY AND SCALABILITY
// ============================================================================

#[test]
fn provider_contract_accepts_new_hardware_domains() {
    let kinds = [
        "classical",
        "quantum",
        "hdl",
        "hybrid",
        "distributed",
        "accelerator",
        "embedded",
        "photonic",
        "neuromorphic",
        "molecular",
        "future.domain",
    ];

    for kind in kinds {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new(format!("future.{kind}"))
                .expect("valid provider identity"),
            ProviderKind::new(kind)
                .expect("valid provider kind"),
            format!("Provider for {kind}"),
            version("1.0.0"),
            version("1.0.0"),
        );

        assert!(
            descriptor.is_ok(),
            "provider kind must remain extensible: {kind}"
        );
    }
}

#[test]
fn provider_identity_accepts_large_finite_namespaces() {
    // This is a finite stress sample, not a maximum supported identifier
    // length or a universal scalability guarantee.
    let segment = "future".repeat(2_048);
    let identity = format!("provider.{segment}");

    let provider_id = ProviderId::new(identity.clone())
        .expect("valid long provider identity");

    assert_eq!(provider_id.as_str(), identity);
}
