
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZHAL — Canonical Hardware Contract Tests
//!
//! Path: `src/zhal/tests/contracts.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.1
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Establish regression tests for the existing canonical contracts
//! that the Zamani Universal Hardware Abstraction Layer (ZHAL)
//! must consume and preserve.
//!
//! These tests deliberately avoid inventing ZHAL APIs that have
//! not yet been implemented.
//!
//! They establish the foundational compatibility requirements
//! against which subsequent ZHAL implementation modules must be
//! integrated.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Capability descriptions and evidence:
//!     crate::stdlib::capability
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Structured errors:
//!     crate::stdlib::error
//!
//! Versioning:
//!     crate::stdlib::version
//!
//! Generic byte-oriented device identity:
//!     crate::stdlib::io::devices
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Runtime orchestration:
//!     crate::zur
//!
//! ## OWNERSHIP BOUNDARIES
//!
//! These tests do not create:
//!
//! - A second provider identity.
//! - A second capability registry.
//! - A second error system.
//! - A second cancellation system.
//! - A second versioning system.
//! - A second generic device-I/O system.
//! - A second quantum hardware abstraction.
//! - A second HDL realization system.
//! - A second runtime resource allocator.
//!
//! ## SCALABILITY
//!
//! Test data uses configurable, finite workloads.
//!
//! Finite test workloads are verification samples, not universal
//! hardware or language capacity limits.
//!
//! No universal maximum is established for:
//!
//! - Providers.
//! - Devices.
//! - Capabilities.
//! - Operations.
//! - Hardware resources.
//! - Computational domains.
//! - Namespace depth.
//! - Identifier length.
//! - Distributed participants.
//! - Memory.
//! - Processing capacity.
//!
//! Actual operation is bounded by available resources,
//! representable values and explicitly configured budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## COMPLETION STATUS
//!
//! This file validates existing shared contracts.
//!
//! Direct ZHAL API conformance tests must be added when the
//! corresponding ZHAL public interfaces actually exist.
//!
//! No test in this file should be interpreted as proof that
//! the entire ZHAL subsystem is already implemented.

use std::cmp::Ordering;
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

use crate::stdlib::context::{
    OperationContext,
};

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Retryability,
};

use crate::stdlib::io::devices::DeviceId;

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderId,
    ProviderKind,
    ProviderOperationDescriptor,
    ProviderOperationId,
    ProviderStatus,
    capability_status,
};

use crate::stdlib::version::Version;

// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Parses a valid semantic version for contract tests.
fn version(value: &str) -> Version {
    Version::parse(value)
        .expect("test version must be valid")
}

/// Constructs an extensible capability.
fn make_capability(id: &str) -> Capability {
    Capability::new(
        CapabilityId::new(id)
            .expect("test capability identity must be valid"),
        CapabilityKind::new("computation")
            .expect("test capability kind must be valid"),
    )
}

/// Constructs an empty operation context.
fn operation_context() -> OperationContext {
    OperationContext::builder()
        .with_component("zhal.tests")
        .expect("valid component")
        .with_operation("contract.validation")
        .expect("valid operation")
        .build()
}

/// Constructs a provider descriptor.
fn provider_descriptor() -> ProviderDescriptor {
    ProviderDescriptor::new(
        ProviderId::new("zamani.tests.hardware")
            .expect("valid provider identity"),
        ProviderKind::new("hardware.provider")
            .expect("valid provider kind"),
        "ZHAL Contract Test Provider",
        version("1.0.0"),
        version("1.0.0"),
    )
    .expect("valid provider descriptor")
}

// ============================================================================
// 1. PROVIDER IDENTITY CONTRACTS
// ============================================================================

#[test]
fn provider_identity_is_open_world() {
    let identities = [
        "classical.cpu",
        "quantum.qpu",
        "hdl.fpga",
        "accelerator.gpu",
        "distributed.remote",
        "embedded.microcontroller",
        "future.photonic.processor",
        "future.molecular.computing.provider",
    ];

    for identity in identities {
        let provider = ProviderId::new(identity)
            .expect("valid future provider identity");

        assert_eq!(provider.as_str(), identity);
    }
}

#[test]
fn provider_identity_preserves_exact_value() {
    let identity = "future.vendor_42.special-provider";

    let provider = ProviderId::new(identity)
        .expect("valid provider identity");

    assert_eq!(provider.as_str(), identity);
}

#[test]
fn provider_identity_rejects_empty_identifier() {
    assert!(ProviderId::new("").is_err());
}

#[test]
fn provider_identity_rejects_malformed_namespaces() {
    let invalid = [
        ".provider",
        "provider.",
        "provider..hardware",
        " provider.hardware",
        "provider.hardware ",
        "provider hardware",
    ];

    for identity in invalid {
        assert!(
            ProviderId::new(identity).is_err(),
            "identity should be rejected: {identity}"
        );
    }
}

#[test]
fn provider_identity_has_no_arbitrary_short_identifier_ceiling() {
    // This is a finite stress sample, not a universal maximum.
    let segment = "x".repeat(16_384);

    let identity = format!("future.{segment}");

    let provider = ProviderId::new(identity.clone())
        .expect("long valid identity must be accepted");

    assert_eq!(provider.as_str(), identity);
}

#[test]
fn provider_identity_supports_ordering_and_hashing() {
    use std::collections::BTreeSet;
    use std::collections::HashSet;

    let first = ProviderId::new("future.provider.alpha")
        .unwrap();

    let second = ProviderId::new("future.provider.beta")
        .unwrap();

    let mut ordered = BTreeSet::new();
    ordered.insert(first.clone());
    ordered.insert(second.clone());

    assert_eq!(ordered.len(), 2);

    let mut hashed = HashSet::new();
    hashed.insert(first);
    hashed.insert(second);

    assert_eq!(hashed.len(), 2);
}

// ============================================================================
// 2. CAPABILITY IDENTITY CONTRACTS
// ============================================================================

#[test]
fn capability_identity_is_extensible() {
    let identities = [
        "classical.computation",
        "quantum.computation",
        "quantum.error_correction",
        "hdl.synthesis",
        "accelerator.tensor_execution",
        "distributed.execution",
        "embedded.device_access",
        "future.photonic.execution",
        "future.neuromorphic.execution",
    ];

    for identity in identities {
        let capability = CapabilityId::new(identity)
            .expect("valid capability identity");

        assert_eq!(capability.as_str(), identity);
    }
}

#[test]
fn capability_identity_preserves_namespace() {
    let capability = CapabilityId::new(
        "future.photonic.optical_computation",
    )
    .expect("valid capability");

    assert_eq!(
        capability.namespace(),
        Some("future.photonic")
    );

    assert_eq!(
        capability.name(),
        "optical_computation"
    );
}

#[test]
fn capability_identity_rejects_invalid_segments() {
    let invalid = [
        "",
        ".quantum",
        "quantum.",
        "quantum..computation",
        "Quantum.computation",
        "quantum computation",
        "quantum.computation ",
    ];

    for identity in invalid {
        assert!(
            CapabilityId::new(identity).is_err(),
            "invalid capability identity: {identity}"
        );
    }
}

#[test]
fn capability_identity_supports_deep_namespaces() {
    // Namespace depth is a test input, not a language limit.
    let namespace = std::iter::repeat_n("future", 128)
        .collect::<Vec<_>>()
        .join(".");

    let identity = format!("{namespace}.computation");

    let capability = CapabilityId::new(identity.clone())
        .expect("valid deep namespace");

    assert_eq!(capability.as_str(), identity);
}

// ============================================================================
// 3. CAPABILITY AVAILABILITY CONTRACTS
// ============================================================================

#[test]
fn unknown_capability_is_not_available() {
    let snapshot = CapabilitySnapshot::new();

    let id = CapabilityId::new("quantum.computation")
        .unwrap();

    assert_eq!(
        snapshot.status(&id),
        CapabilityStatus::Unknown
    );

    assert!(!snapshot.establishes_availability(&id));

    assert_eq!(
        snapshot.get(&id),
        None
    );
}

#[test]
fn capability_availability_states_remain_distinct() {
    let available = CapabilityStatus::Available;
    let unavailable = CapabilityStatus::Unavailable;
    let conditional = CapabilityStatus::Conditional;
    let unknown = CapabilityStatus::Unknown;

    assert_eq!(available.availability(), Some(true));

    assert_eq!(unavailable.availability(), Some(false));

    assert_eq!(conditional.availability(), None);

    assert_eq!(unknown.availability(), None);

    assert!(available.is_available());
    assert!(unavailable.is_unavailable());
    assert!(conditional.is_conditional());
    assert!(unknown.is_unknown());
}

#[test]
fn capability_snapshot_preserves_all_availability_states() {
    let mut snapshot = CapabilitySnapshot::new();

    let cases = [
        (
            "classical.execution",
            CapabilityStatus::Available,
        ),
        (
            "quantum.execution",
            CapabilityStatus::Conditional,
        ),
        (
            "hdl.synthesis",
            CapabilityStatus::Unavailable,
        ),
        (
            "future.photonic.execution",
            CapabilityStatus::Unknown,
        ),
    ];

    for (id, status) in cases {
        snapshot.insert(
            CapabilityState::new(
                make_capability(id),
                status,
            ),
        )
        .expect("unique capability identity");
    }

    for (id, expected) in cases {
        let id = CapabilityId::new(id).unwrap();

        assert_eq!(
            snapshot.status(&id),
            expected
        );
    }

    assert_eq!(snapshot.len(), cases.len());
}

#[test]
fn capability_snapshot_rejects_duplicate_identity() {
    let mut snapshot = CapabilitySnapshot::new();

    let first = CapabilityState::new(
        make_capability("quantum.execution"),
        CapabilityStatus::Available,
    );

    let second = CapabilityState::new(
        make_capability("quantum.execution"),
        CapabilityStatus::Unavailable,
    );

    snapshot.insert(first)
        .expect("first insertion must succeed");

    assert!(snapshot.insert(second).is_err());

    assert_eq!(
        snapshot.status(
            &CapabilityId::new("quantum.execution").unwrap()
        ),
        CapabilityStatus::Available
    );
}

#[test]
fn capability_snapshot_remove_returns_previous_state() {
    let mut snapshot = CapabilitySnapshot::new();

    let id = CapabilityId::new("classical.execution")
        .unwrap();

    snapshot.insert(
        CapabilityState::new(
            make_capability("classical.execution"),
            CapabilityStatus::Available,
        ),
    )
    .unwrap();

    let removed = snapshot.remove(&id);

    assert!(removed.is_some());

    assert_eq!(
        snapshot.status(&id),
        CapabilityStatus::Unknown
    );
}

#[test]
fn capability_snapshot_iteration_is_deterministic() {
    let mut snapshot = CapabilitySnapshot::new();

    let identities = [
        "future.zeta",
        "classical.execution",
        "quantum.execution",
        "future.alpha",
        "hdl.synthesis",
    ];

    for identity in identities {
        snapshot.insert(
            CapabilityState::new(
                make_capability(identity),
                CapabilityStatus::Unknown,
            ),
        )
        .unwrap();
    }

    let actual = snapshot
        .ids()
        .map(|id| id.as_str())
        .collect::<Vec<_>>();

    let mut expected = identities.to_vec();
    expected.sort_unstable();

    assert_eq!(actual, expected);
}

#[test]
fn capability_snapshot_scales_with_inserted_entries() {
    let mut snapshot = CapabilitySnapshot::new();

    // A finite regression workload only.
    // This does not establish a supported maximum.
    let workload_size = 257_usize;

    for index in 0..workload_size {
        let identity = format!("stress.capability.{index}");

        snapshot.insert(
            CapabilityState::new(
                make_capability(&identity),
                CapabilityStatus::Available,
            ),
        )
        .expect("unique stress-test capability");
    }

    assert_eq!(snapshot.len(), workload_size);

    assert_eq!(
        snapshot.ids().count(),
        workload_size
    );
}

// ============================================================================
// 4. CAPABILITY EVIDENCE CONTRACTS
// ============================================================================

#[test]
fn capability_evidence_preserves_origin() {
    let evidence = CapabilityEvidence::new(
        "hardware.provider.observation",
        "Capability reported by the provider",
    )
    .expect("valid evidence");

    assert_eq!(
        evidence.source(),
        "hardware.provider.observation"
    );

    assert_eq!(
        evidence.description(),
        "Capability reported by the provider"
    );
}

#[test]
fn capability_evidence_rejects_empty_source() {
    assert!(
        CapabilityEvidence::new(
            "",
            "Observed capability"
        )
        .is_err()
    );
}

#[test]
fn capability_evidence_rejects_empty_description() {
    assert!(
        CapabilityEvidence::new(
            "hardware.provider",
            ""
        )
        .is_err()
    );
}

#[test]
fn capability_state_preserves_evidence() {
    let evidence = CapabilityEvidence::new(
        "provider.observation",
        "Provider reported availability",
    )
    .unwrap();

    let state = CapabilityState::new(
        make_capability("quantum.execution"),
        CapabilityStatus::Available,
    )
    .with_evidence(evidence);

    assert_eq!(state.evidence().len(), 1);

    assert_eq!(
        state.evidence()[0].source(),
        "provider.observation"
    );

    assert!(state.is_available());
}

#[test]
fn provider_capability_status_preserves_unknown() {
    let snapshot = CapabilitySnapshot::new();

    let id = CapabilityId::new("future.execution")
        .unwrap();

    assert_eq!(
        capability_status(&snapshot, &id),
        None
    );
}

// ============================================================================
// 5. PROVIDER DESCRIPTOR CONTRACTS
// ============================================================================

#[test]
fn provider_descriptor_preserves_identity_and_versions() {
    let descriptor = provider_descriptor();

    assert_eq!(
        descriptor.id().as_str(),
        "zamani.tests.hardware"
    );

    assert_eq!(
        descriptor.kind().as_str(),
        "hardware.provider"
    );

    assert_eq!(
        descriptor.display_name(),
        "ZHAL Contract Test Provider"
    );

    assert_eq!(
        descriptor.implementation_version().core_version(),
        "1.0.0"
    );

    assert_eq!(
        descriptor.api_version().core_version(),
        "1.0.0"
    );
}

#[test]
fn provider_implementation_and_api_versions_are_independent() {
    let descriptor = ProviderDescriptor::new(
        ProviderId::new("future.provider")
            .unwrap(),
        ProviderKind::new("future.hardware")
            .unwrap(),
        "Future Provider",
        version("8.4.2"),
        version("2.1.0"),
    )
    .unwrap();

    assert_eq!(
        descriptor.implementation_version().core_version(),
        "8.4.2"
    );

    assert_eq!(
        descriptor.api_version().core_version(),
        "2.1.0"
    );
}

#[test]
fn provider_descriptor_supports_extensible_capabilities() {
    let mut descriptor = provider_descriptor();

    let capability = make_capability(
        "future.photonic.execution",
    );

    descriptor
        .insert_capability(capability)
        .expect("new capability must be accepted");

    assert_eq!(
        descriptor.capabilities().len(),
        1
    );

    assert!(
        descriptor
            .capability(
                &CapabilityId::new(
                    "future.photonic.execution"
                )
                .unwrap()
            )
            .is_some()
    );
}

#[test]
fn provider_descriptor_rejects_duplicate_capabilities() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_capability(
            make_capability("quantum.execution")
        )
        .unwrap();

    let result = descriptor.insert_capability(
        make_capability("quantum.execution")
    );

    assert!(result.is_err());

    assert_eq!(
        descriptor.capabilities().len(),
        1
    );
}

#[test]
fn provider_descriptor_supports_explicit_capability_replacement() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_capability(
            make_capability("quantum.execution")
        )
        .unwrap();

    descriptor
        .replace_capability(
            make_capability("quantum.execution")
                .with_description(
                    "Updated capability description"
                )
        )
        .expect("existing capability must be replaceable");

    assert_eq!(
        descriptor.capabilities().len(),
        1
    );

    assert_eq!(
        descriptor.capabilities()[0].description(),
        Some("Updated capability description")
    );
}

#[test]
fn provider_descriptor_supports_open_world_operations() {
    let mut descriptor = provider_descriptor();

    let operation = ProviderOperationDescriptor::from_id(
        "future.photonic.execute",
        version("1.0.0"),
    )
    .expect("valid operation")
    .with_description(
        "Future photonic execution operation"
    )
    .expect("valid description");

    descriptor
        .insert_operation(operation)
        .expect("operation registration");

    assert_eq!(
        descriptor.operations().len(),
        1
    );

    assert!(
        descriptor
            .operation(
                &ProviderOperationId::new(
                    "future.photonic.execute"
                )
                .unwrap()
            )
            .is_some()
    );
}

#[test]
fn provider_descriptor_rejects_duplicate_operations() {
    let mut descriptor = provider_descriptor();

    let operation = ProviderOperationDescriptor::from_id(
        "hardware.execute",
        version("1.0.0"),
    )
    .unwrap();

    descriptor
        .insert_operation(operation.clone())
        .unwrap();

    assert!(
        descriptor.insert_operation(operation).is_err()
    );

    assert_eq!(
        descriptor.operations().len(),
        1
    );
}

#[test]
fn provider_metadata_has_deterministic_order() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_attribute("vendor", "example")
        .unwrap();

    descriptor
        .insert_attribute("architecture", "future")
        .unwrap();

    let keys = descriptor
        .attributes()
        .keys()
        .map(String::as_str)
        .collect::<Vec<_>>();

    assert_eq!(
        keys,
        vec!["architecture", "vendor"]
    );
}

#[test]
fn provider_metadata_rejects_duplicate_attributes() {
    let mut descriptor = provider_descriptor();

    descriptor
        .insert_attribute("architecture", "first")
        .unwrap();

    assert!(
        descriptor
            .insert_attribute("architecture", "second")
            .is_err()
    );

    assert_eq!(
        descriptor.attribute("architecture"),
        Some("first")
    );
}

// ============================================================================
// 6. PROVIDER STATUS CONTRACTS
// ============================================================================

#[test]
fn provider_status_distinguishes_readiness() {
    assert!(
        ProviderStatus::Ready.may_accept_operations()
    );

    assert!(
        ProviderStatus::Degraded.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Unknown.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Initializing.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Unavailable.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Stopping.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Stopped.may_accept_operations()
    );

    assert!(
        !ProviderStatus::Failed.may_accept_operations()
    );
}

// ============================================================================
// 7. PROVIDER TRAIT CONTRACTS
// ============================================================================

/// Minimal provider implementation used to verify the existing
/// object-safe provider boundary.
///
/// This is a test fixture, not a production hardware provider.
struct ContractTestProvider {
    descriptor: ProviderDescriptor,
    snapshot: CapabilitySnapshot,
}

impl Provider for ContractTestProvider {
    fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    fn status(&self) -> ProviderStatus {
        ProviderStatus::Ready
    }

    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            context.check_cancelled()?;

            Ok(self.snapshot.clone())
        })
    }
}

#[test]
fn provider_trait_supports_independent_implementations() {
    let provider = ContractTestProvider {
        descriptor: provider_descriptor(),
        snapshot: CapabilitySnapshot::new(),
    };

    assert_eq!(
        provider.status(),
        ProviderStatus::Ready
    );

    assert_eq!(
        provider.descriptor().id().as_str(),
        "zamani.tests.hardware"
    );
}

#[test]
fn provider_trait_supports_shared_trait_objects() {
    let provider = ContractTestProvider {
        descriptor: provider_descriptor(),
        snapshot: CapabilitySnapshot::new(),
    };

    let provider: Arc<dyn Provider> = Arc::new(provider);

    assert_eq!(
        provider.status(),
        ProviderStatus::Ready
    );
}

#[test]
fn provider_snapshot_operation_is_asynchronous() {
    let provider = ContractTestProvider {
        descriptor: provider_descriptor(),
        snapshot: CapabilitySnapshot::new(),
    };

    let context = operation_context();

    let future = provider.capability_snapshot(&context);

    // Creating and owning the future must not require the caller
    // to introduce a second executor.
    drop(future);
}

#[test]
fn provider_snapshot_preserves_capability_observations() {
    let mut snapshot = CapabilitySnapshot::new();

    snapshot
        .insert(
            CapabilityState::new(
                make_capability("classical.execution"),
                CapabilityStatus::Available,
            )
        )
        .unwrap();

    let provider = ContractTestProvider {
        descriptor: provider_descriptor(),
        snapshot,
    };

    assert_eq!(
        provider.snapshot.len(),
        1
    );

    assert_eq!(
        provider.snapshot.status(
            &CapabilityId::new(
                "classical.execution"
            )
            .unwrap()
        ),
        CapabilityStatus::Available
    );
}

// ============================================================================
// 8. OPERATION CONTEXT CONTRACTS
// ============================================================================

#[test]
fn operation_context_preserves_operation_identity() {
    let context = OperationContext::builder()
        .with_component("zhal")
        .unwrap()
        .with_operation("hardware.discovery")
        .unwrap()
        .with_scope("logical.execution.scope")
        .unwrap()
        .build();

    assert_eq!(context.component(), Some("zhal"));

    assert_eq!(
        context.operation(),
        Some("hardware.discovery")
    );

    assert_eq!(
        context.scope(),
        Some("logical.execution.scope")
    );

    assert!(!context.is_cancelled());
}

#[test]
fn operation_context_supports_child_contexts() {
    let parent = OperationContext::builder()
        .with_component("zhal")
        .unwrap()
        .with_operation("hardware.discovery")
        .unwrap()
        .with_scope("logical.scope")
        .unwrap()
        .build();

    let child = parent
        .child_builder()
        .with_operation("hardware.device.observe")
        .unwrap()
        .build();

    assert_eq!(child.component(), Some("zhal"));

    assert_eq!(
        child.operation(),
        Some("hardware.device.observe")
    );

    assert_eq!(
        child.scope(),
        Some("logical.scope")
    );
}

#[test]
fn operation_context_supports_typed_extensions() {
    #[derive(Debug, PartialEq, Eq)]
    struct ProviderContext {
        provider_namespace: String,
    }

    let context = OperationContext::builder()
        .with_extension(ProviderContext {
            provider_namespace: "future.hardware".to_owned(),
        })
        .unwrap()
        .build();

    let extension = context
        .extension::<ProviderContext>()
        .expect("extension must be available");

    assert_eq!(
        extension.provider_namespace,
        "future.hardware"
    );
}

#[test]
fn operation_context_rejects_duplicate_extension_types() {
    let builder = OperationContext::builder()
        .with_extension(42_u64)
        .unwrap();

    let result = builder.with_extension(84_u64);

    assert!(result.is_err());
}

#[test]
fn operation_context_does_not_assume_physical_location() {
    let context = OperationContext::builder()
        .with_scope("logical.application.scope")
        .unwrap()
        .build();

    assert_eq!(
        context.scope(),
        Some("logical.application.scope")
    );
}

// ============================================================================
// 9. DEVICE IDENTITY INTEGRATION
// ============================================================================

#[test]
fn generic_device_identity_is_provider_neutral() {
    let device = DeviceId::new(
        "provider-defined-device-001"
    )
    .expect("valid device identity");

    assert_eq!(
        device.as_str(),
        "provider-defined-device-001"
    );
}

#[test]
fn generic_device_identity_accepts_future_devices() {
    let identities = [
        "cpu-instance-alpha",
        "gpu-instance-beta",
        "qpu-instance-gamma",
        "fpga-instance-delta",
        "embedded-instance-epsilon",
        "future-photonic-device",
        "remote-heterogeneous-node",
    ];

    for identity in identities {
        let device = DeviceId::new(identity)
            .expect("valid device identity");

        assert_eq!(device.as_str(), identity);
    }
}

#[test]
fn generic_device_identity_rejects_empty_values() {
    assert!(DeviceId::new("").is_err());
}

#[test]
fn generic_device_identity_rejects_whitespace() {
    assert!(DeviceId::new("device with spaces").is_err());
}

// ============================================================================
// 10. VERSIONING CONTRACTS
// ============================================================================

#[test]
fn provider_versions_follow_semantic_versioning() {
    let stable = version("1.0.0");
    let prerelease = version("1.0.0-alpha.1");

    assert_eq!(
        stable.precedence_cmp(&prerelease),
        Ordering::Greater
    );
}

#[test]
fn provider_version_build_metadata_does_not_change_precedence() {
    let first = version("1.2.3+build.1");
    let second = version("1.2.3+build.2");

    assert_eq!(
        first.precedence_cmp(&second),
        Ordering::Equal
    );
}

#[test]
fn version_comparison_does_not_require_fixed_width_integers() {
    let first = version(
        "999999999999999999999999999999999999.0.0"
    );

    let second = version(
        "1000000000000000000000000000000000000.0.0"
    );

    assert_eq!(
        first.precedence_cmp(&second),
        Ordering::Less
    );
}

#[test]
fn provider_api_and_implementation_versions_remain_distinct() {
    let descriptor = ProviderDescriptor::new(
        ProviderId::new("future.provider.versioned")
            .unwrap(),
        ProviderKind::new("hardware.provider")
            .unwrap(),
        "Versioned Provider",
        version("12.4.1"),
        version("3.0.0"),
    )
    .unwrap();

    assert_ne!(
        descriptor.implementation_version().core_version(),
        descriptor.api_version().core_version()
    );
}

// ============================================================================
// 11. STRUCTURED ERROR CONTRACTS
// ============================================================================

#[test]
fn hardware_error_categories_remain_distinct() {
    let cases = [
        (ErrorKind::Unsupported, "unsupported"),
        (
            ErrorKind::CapabilityUnavailable,
            "capability_unavailable",
        ),
        (ErrorKind::Infeasible, "infeasible"),
        (
            ErrorKind::ResourceUnavailable,
            "resource_unavailable",
        ),
        (
            ErrorKind::BudgetExhausted,
            "budget_exhausted",
        ),
        (
            ErrorKind::PermissionDenied,
            "permission_denied",
        ),
        (
            ErrorKind::Communication,
            "communication",
        ),
        (ErrorKind::Provider, "provider"),
        (ErrorKind::Cancelled, "cancelled"),
        (ErrorKind::TimedOut, "timed_out"),
        (
            ErrorKind::VersionMismatch,
            "version_mismatch",
        ),
        (
            ErrorKind::ContractViolation,
            "contract_violation",
        ),
    ];

    for (kind, expected) in cases {
        assert_eq!(kind.as_str(), expected);
    }
}

#[test]
fn structured_error_preserves_component_and_operation() {
    let error = Error::new(
        ErrorKind::Communication,
        "hardware communication failed",
    )
    .in_component("zhal")
    .during_operation("device.access");

    assert_eq!(
        error.kind(),
        ErrorKind::Communication
    );

    assert_eq!(error.component(), Some("zhal"));

    assert_eq!(
        error.operation(),
        Some("device.access")
    );
}

#[test]
fn structured_error_preserves_retryability() {
    let error = Error::new(
        ErrorKind::Communication,
        "temporary communication failure",
    )
    .with_retryability(Retryability::Retryable);

    assert_eq!(
        error.retryability(),
        Retryability::Retryable
    );
}

#[test]
fn budget_exhaustion_is_not_unsupported_capability() {
    let budget_error = Error::new(
        ErrorKind::BudgetExhausted,
        "configured operational budget exhausted",
    );

    let capability_error = Error::new(
        ErrorKind::CapabilityUnavailable,
        "required capability unavailable",
    );

    assert_ne!(
        budget_error.kind(),
        capability_error.kind()
    );
}

// ============================================================================
// 12. DOMAIN OWNERSHIP CONTRACTS
// ============================================================================

#[test]
fn classical_and_future_domains_use_open_capability_identifiers() {
    let domains = [
        "classical.computation",
        "quantum.computation",
        "hdl.realization",
        "accelerator.computation",
        "distributed.computation",
        "embedded.computation",
        "future.neuromorphic.computation",
        "future.photonic.computation",
        "future.molecular.computation",
    ];

    for domain in domains {
        assert!(
            CapabilityId::new(domain).is_ok(),
            "domain capability should be extensible: {domain}"
        );
    }
}

#[test]
fn quantum_and_hdl_capabilities_are_not_hardware_identity() {
    let quantum = CapabilityId::new(
        "quantum.computation"
    )
    .unwrap();

    let hdl = CapabilityId::new(
        "hdl.realization"
    )
    .unwrap();

    assert_ne!(quantum, hdl);

    // Capability identity describes functionality.
    // It does not identify a physical device or provider.
}

#[test]
fn hardware_observations_do_not_imply_resource_reservation() {
    let mut snapshot = CapabilitySnapshot::new();

    snapshot
        .insert(
            CapabilityState::new(
                make_capability("classical.execution"),
                CapabilityStatus::Available,
            )
        )
        .unwrap();

    // The snapshot establishes only an observation.
    // It exposes no resource lease or reservation operation.
    assert!(
        snapshot.establishes_availability(
            &CapabilityId::new("classical.execution").unwrap()
        )
    );
}

// ============================================================================
// 13. THREAD-SAFETY CONTRACTS
// ============================================================================

#[test]
fn shared_provider_contract_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<ProviderId>();
    assert_send_sync::<CapabilityId>();
    assert_send_sync::<CapabilitySnapshot>();
    assert_send_sync::<OperationContext>();
    assert_send_sync::<ProviderDescriptor>();
}

#[test]
fn provider_descriptor_can_be_shared_across_threads() {
    let descriptor = Arc::new(provider_descriptor());

    let cloned = Arc::clone(&descriptor);

    let handle = std::thread::spawn(move || {
        cloned.id().as_str().to_owned()
    });

    assert_eq!(
        handle.join().expect("thread must complete"),
        "zamani.tests.hardware"
    );
}
