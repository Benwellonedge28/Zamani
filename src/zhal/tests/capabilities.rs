
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! ZHAL capability integration and production conformance tests.
//!
//! Path: src/zhal/tests/capabilities.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify the actual ZHAL capability architecture across:
//!
//! - Canonical standard-library capability identities.
//! - Hardware capability evidence.
//! - Evidence provenance and verification.
//! - Evidence validity and freshness.
//! - Hardware capability requirements.
//! - Availability assessment.
//! - Structured compatibility evaluation.
//! - Provider and device isolation.
//! - Heterogeneous computational domains.
//! - Dynamic hardware observations.
//! - Large evidence collections.
//! - POCO-REAF portability contracts.
//!
//! # Architectural ownership
//!
//! These tests exercise existing authorities:
//!
//! - crate::stdlib::capability
//! - crate::stdlib::provider
//! - crate::stdlib::io::devices
//! - crate::zhal::identity
//! - crate::zhal::capabilities::evidence
//! - crate::zhal::capabilities::requirements
//! - crate::zhal::capabilities::availability
//! - crate::zhal::capabilities::compatibility
//!
//! Domain-specific capabilities remain owned by their respective
//! computational subsystems.
//!
//! These tests do not implement:
//!
//! - Hardware discovery.
//! - Provider registration.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Backend selection.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//!
//! # Scalability
//!
//! No universal hardware maximum is introduced.
//!
//! Stress-test sizes represent test workloads, not supported
//! hardware ceilings.
//!
//! Actual execution remains bounded by available resources,
//! implementation capabilities and explicitly configured budgets.
//!
//! Literal infinite computation is not promised.
//!
//! # Important
//!
//! Passing these tests establishes correctness of the tested
//! contracts. It does not independently certify physical hardware,
//! guarantee execution or establish resource feasibility.

// =============================================================================
// IMPORTS
// =============================================================================

use crate::stdlib::capability::{
    CapabilityId,
    CapabilityStatus,
};

use crate::stdlib::io::devices::DeviceId;

use crate::stdlib::provider::ProviderId;

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::capabilities::evidence::{
    EvidenceConfidence,
    EvidenceFreshness,
    EvidenceOrigin,
    EvidenceTimestamp,
    EvidenceVerification,
    HardwareCapabilityEvidence,
    HardwareCapabilityEvidenceSet,
};

use crate::zhal::capabilities::requirements::{
    HardwareCapabilityRequirement,
    HardwareCapabilityRequirementSet,
    RequirementOrigin,
};

use crate::zhal::capabilities::availability::{
    AccessObservation,
    AvailabilityState,
    assess_capability,
    assess_requirement,
};

use crate::zhal::capabilities::compatibility::{
    CompatibilityFindingKind,
    CompatibilityPolicy,
    CompatibilityScope,
    CompatibilityStatus,
    evaluate_compatibility,
};

// =============================================================================
// SHARED TEST FIXTURES
// =============================================================================

/// Creates a canonical standard-library capability identity.
///
/// Capability identifiers are open-world strings.
///
/// These examples cover different computational domains, but
/// do not establish an exhaustive list.
fn capability(value: &str) -> CapabilityId {
    CapabilityId::new(value.to_owned())
        .expect("test capability identity must be valid")
}

/// Creates a canonical provider identity.
fn provider(value: &str) -> ProviderId {
    ProviderId::new(value.to_owned())
        .expect("test provider identity must be valid")
}

/// Creates a provider-scoped physical hardware identity.
fn device(
    provider_name: &str,
    device_name: &str,
) -> HardwareDeviceIdentity {
    HardwareDeviceIdentity::new(
        provider(provider_name),
        DeviceId::new(device_name.to_owned())
            .expect("test device identity must be valid"),
    )
}

/// Creates an explicit evidence timestamp.
///
/// Test timestamps are deterministic and do not depend on
/// the machine clock.
fn timestamp(value: i128) -> EvidenceTimestamp {
    EvidenceTimestamp::from_unix_nanos(value)
}

/// Creates a validated evidence origin.
fn origin(name: &str) -> EvidenceOrigin {
    EvidenceOrigin::new(
        name.to_owned(),
        Some("test.evidence.reference".to_owned()),
    )
    .expect("test evidence origin must be valid")
}

/// Creates a hardware capability observation.
///
/// Provider and device attribution are optional so that the
/// tests can exercise both attributed and unattributed evidence.
fn evidence(
    capability_name: &str,
    status: CapabilityStatus,
    observed_at: i128,
    valid_until: Option<i128>,
    provider_name: Option<&str>,
    device_name: Option<&str>,
) -> HardwareCapabilityEvidence {
    let mut record = HardwareCapabilityEvidence::new(
        capability(capability_name),
        status,
        origin("hardware.provider.report"),
        timestamp(observed_at),
        "test hardware provider",
        "test physical hardware capability observation",
    )
    .expect("test evidence must be valid");

    if let Some(provider_name) = provider_name {
        record = record
            .with_provider(provider(provider_name))
            .expect("provider attribution must be valid");
    }

    if let Some(device_name) = device_name {
        let provider_name = provider_name
            .expect("device attribution requires a provider");

        record = record
            .with_device(device(provider_name, device_name))
            .expect("device attribution must be valid");
    }

    if let Some(expiration) = valid_until {
        record = record
            .with_valid_until(timestamp(expiration))
            .expect("evidence validity interval must be valid");
    }

    record
}

/// Creates evidence with an explicit verification classification.
fn verified_evidence(
    capability_name: &str,
    verification: EvidenceVerification,
    verifier: Option<&str>,
) -> HardwareCapabilityEvidence {
    evidence(
        capability_name,
        CapabilityStatus::Available,
        50,
        Some(150),
        Some("test.provider"),
        Some("test.device"),
    )
    .with_verification(
        verification,
        verifier.map(str::to_owned),
    )
    .expect("verification metadata must be valid")
}

/// Creates a mandatory hardware capability requirement.
fn requirement(
    capability_name: &str,
) -> HardwareCapabilityRequirement {
    HardwareCapabilityRequirement::new(
        capability(capability_name),
    )
    .with_origin(
        RequirementOrigin::new(
            "frontend.semantic",
            Some("test.source.reference".to_owned()),
        )
        .expect("requirement origin must be valid"),
    )
}

/// Evaluates one or more requirements against evidence using
/// the standard compatibility policy.
fn evaluate(
    requirements: &HardwareCapabilityRequirementSet,
    records: &[HardwareCapabilityEvidence],
    scope: CompatibilityScope,
) -> crate::zhal::errors::ZhalResult<
    crate::zhal::capabilities::compatibility::CompatibilityReport,
> {
    evaluate_compatibility(
        requirements,
        records,
        timestamp(100),
        scope,
        CompatibilityPolicy::new(),
    )
}

// =============================================================================
// 1. CANONICAL CAPABILITY IDENTITY
// =============================================================================

#[test]
fn zhal_uses_the_canonical_standard_library_capability_identity() {
    let identity = capability("classical.parallel_execution");

    let requirement = requirement(
        "classical.parallel_execution",
    );

    let observation = evidence(
        "classical.parallel_execution",
        CapabilityStatus::Available,
        50,
        Some(150),
        None,
        None,
    );

    // All three boundaries must preserve the exact same
    // canonical identity representation.
    fn accepts_standard_identity(_: &CapabilityId) {}

    accepts_standard_identity(&identity);
    accepts_standard_identity(requirement.capability());
    accepts_standard_identity(observation.capability());

    assert_eq!(
        identity,
        *requirement.capability(),
    );

    assert_eq!(
        identity,
        *observation.capability(),
    );
}

#[test]
fn capability_identifiers_support_future_computational_domains() {
    let names = [
        "classical.parallel_execution",
        "quantum.dynamic_control",
        "hdl.synthesis",
        "accelerator.tensor_execution",
        "distributed.collective_execution",
        "embedded.sensor_io",
        "hybrid.cross_domain_execution",
        "future.photonic.computation",
        "future.neuromorphic.computation",
        "future.molecular.computation",
    ];

    for name in names {
        let identity = capability(name);

        assert_eq!(
            identity.as_str(),
            name,
            "capability identity must preserve its exact name",
        );
    }
}

#[test]
fn unknown_future_capabilities_are_not_rejected_by_zhal() {
    let identity = capability(
        "future.unregistered.computational_capability",
    );

    let requirement =
        HardwareCapabilityRequirement::new(identity.clone());

    assert_eq!(
        requirement.capability(),
        &identity,
    );
}

// =============================================================================
// 2. EVIDENCE CONSTRUCTION AND VALIDATION
// =============================================================================

#[test]
fn valid_hardware_evidence_is_constructed_and_validated() {
    let record = evidence(
        "classical.parallel_execution",
        CapabilityStatus::Available,
        50,
        Some(150),
        Some("classical.provider"),
        Some("processor.primary"),
    );

    assert!(record.validate().is_ok());

    assert_eq!(
        record.status(),
        CapabilityStatus::Available,
    );

    assert_eq!(
        record.provider_id(),
        Some(&provider("classical.provider")),
    );

    assert_eq!(
        record.device(),
        Some(&device(
            "classical.provider",
            "processor.primary",
        )),
    );
}

#[test]
fn evidence_preserves_its_origin() {
    let record = evidence(
        "quantum.dynamic_control",
        CapabilityStatus::Available,
        50,
        Some(150),
        None,
        None,
    );

    assert_eq!(
        record.origin().kind(),
        "hardware.provider.report",
    );

    assert_eq!(
        record.origin().reference(),
        Some("test.evidence.reference"),
    );
}

#[test]
fn evidence_rejects_invalid_validity_intervals() {
    let record = evidence(
        "classical.compute",
        CapabilityStatus::Available,
        100,
        None,
        None,
        None,
    );

    assert!(
        record
            .clone()
            .with_valid_until(timestamp(100))
            .is_err()
    );

    assert!(
        record
            .with_valid_until(timestamp(50))
            .is_err()
    );
}

#[test]
fn evidence_confidence_must_be_finite_and_bounded() {
    assert!(EvidenceConfidence::new(0.0).is_ok());
    assert!(EvidenceConfidence::new(0.5).is_ok());
    assert!(EvidenceConfidence::new(1.0).is_ok());

    assert!(EvidenceConfidence::new(-0.01).is_err());
    assert!(EvidenceConfidence::new(1.01).is_err());
    assert!(EvidenceConfidence::new(f64::NAN).is_err());
    assert!(EvidenceConfidence::new(f64::INFINITY).is_err());
    assert!(EvidenceConfidence::new(f64::NEG_INFINITY).is_err());
}

#[test]
fn evidence_verification_requires_a_verifier_reference() {
    let unverified = verified_evidence(
        "quantum.measurement",
        EvidenceVerification::Unverified,
        None,
    );

    assert!(unverified.validate().is_ok());

    let independently_verified_without_authority =
        evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        )
        .with_verification(
            EvidenceVerification::IndependentlyVerified,
            None,
        );

    assert!(
        independently_verified_without_authority.is_err()
    );
}

#[test]
fn independent_verification_provenance_is_preserved() {
    let record = verified_evidence(
        "quantum.measurement",
        EvidenceVerification::IndependentlyVerified,
        Some("independent.attestation.authority"),
    );

    assert!(record.validate().is_ok());

    assert_eq!(
        record.verification(),
        EvidenceVerification::IndependentlyVerified,
    );

    assert_eq!(
        record.verified_by(),
        Some("independent.attestation.authority"),
    );
}

#[test]
fn provider_attribution_is_preserved() {
    let record = evidence(
        "accelerator.tensor_execution",
        CapabilityStatus::Available,
        50,
        Some(150),
        Some("accelerator.provider"),
        None,
    );

    assert_eq!(
        record.provider_id(),
        Some(&provider("accelerator.provider")),
    );
}

#[test]
fn device_attribution_establishes_provider_attribution() {
    let record = evidence(
        "hdl.synthesis",
        CapabilityStatus::Available,
        50,
        Some(150),
        Some("hdl.provider"),
        Some("fpga.board"),
    );

    assert_eq!(
        record.provider_id(),
        Some(&provider("hdl.provider")),
    );

    assert_eq!(
        record.device(),
        Some(&device("hdl.provider", "fpga.board")),
    );
}

#[test]
fn conflicting_device_and_provider_attribution_is_rejected() {
    let result = HardwareCapabilityEvidence::new(
        capability("quantum.measurement"),
        CapabilityStatus::Available,
        origin("provider.report"),
        timestamp(50),
        "test provider",
        "test observation",
    )
    .expect("valid initial evidence")
    .with_provider(provider("provider.one"))
    .expect("valid initial provider")
    .with_device(device(
        "provider.two",
        "quantum.device",
    ));

    assert!(result.is_err());
}

// =============================================================================
// 3. EVIDENCE FRESHNESS
// =============================================================================

#[test]
fn evidence_is_current_inside_its_validity_interval() {
    let record = evidence(
        "classical.compute",
        CapabilityStatus::Available,
        50,
        Some(150),
        None,
        None,
    );

    assert_eq!(
        record.freshness_at(timestamp(100)),
        EvidenceFreshness::Current,
    );
}

#[test]
fn evidence_expires_at_the_declared_expiration_instant() {
    let record = evidence(
        "classical.compute",
        CapabilityStatus::Available,
        50,
        Some(150),
        None,
        None,
    );

    assert_eq!(
        record.freshness_at(timestamp(150)),
        EvidenceFreshness::Expired,
    );
}

#[test]
fn future_evidence_is_not_treated_as_current() {
    let record = evidence(
        "quantum.dynamic_control",
        CapabilityStatus::Available,
        200,
        Some(300),
        None,
        None,
    );

    assert_eq!(
        record.freshness_at(timestamp(100)),
        EvidenceFreshness::NotYetObserved,
    );

    assert!(!record.is_current_at(timestamp(100)));
}

#[test]
fn evidence_without_expiration_has_no_declared_time_expiry() {
    let record = evidence(
        "classical.compute",
        CapabilityStatus::Available,
        50,
        None,
        None,
        None,
    );

    assert_eq!(
        record.freshness_at(timestamp(100)),
        EvidenceFreshness::Current,
    );

    assert_eq!(
        record.valid_until(),
        None,
    );
}

// =============================================================================
// 4. EVIDENCE COLLECTION
// =============================================================================

#[test]
fn evidence_collection_preserves_insertion_order() {
    let mut collection =
        HardwareCapabilityEvidenceSet::new();

    let first = evidence(
        "classical.compute",
        CapabilityStatus::Available,
        50,
        None,
        None,
        None,
    );

    let second = evidence(
        "quantum.measurement",
        CapabilityStatus::Conditional,
        60,
        None,
        None,
        None,
    );

    collection
        .insert(first.clone())
        .expect("first record must be accepted");

    collection
        .insert(second.clone())
        .expect("second record must be accepted");

    let records: Vec<_> = collection.iter().collect();

    assert_eq!(records.len(), 2);
    assert_eq!(records[0].capability(), first.capability());
    assert_eq!(records[1].capability(), second.capability());
}

#[test]
fn evidence_collection_preserves_provider_filtering() {
    let mut collection =
        HardwareCapabilityEvidenceSet::new();

    collection
        .insert(evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            None,
            Some("provider.one"),
            None,
        ))
        .expect("first record");

    collection
        .insert(evidence(
            "classical.compute",
            CapabilityStatus::Available,
            60,
            None,
            Some("provider.two"),
            None,
        ))
        .expect("second record");

    let selected: Vec<_> = collection
        .for_provider(&provider("provider.one"))
        .collect();

    assert_eq!(selected.len(), 1);

    assert_eq!(
        selected[0].provider_id(),
        Some(&provider("provider.one")),
    );
}

#[test]
fn evidence_collection_preserves_capability_filtering() {
    let mut collection =
        HardwareCapabilityEvidenceSet::new();

    collection
        .insert(evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            None,
            None,
            None,
        ))
        .expect("first record");

    collection
        .insert(evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            60,
            None,
            None,
            None,
        ))
        .expect("second record");

    let selected: Vec<_> = collection
        .for_capability(&capability("quantum.measurement"))
        .collect();

    assert_eq!(selected.len(), 1);

    assert_eq!(
        selected[0].capability().as_str(),
        "quantum.measurement",
    );
}

#[test]
fn evidence_collection_extension_is_transactional() {
    let mut collection =
        HardwareCapabilityEvidenceSet::new();

    collection
        .insert(evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            None,
            None,
            None,
        ))
        .expect("initial record");

    let original_length = collection.len();

    let invalid = evidence(
        "quantum.measurement",
        CapabilityStatus::Available,
        50,
        None,
        None,
        None,
    )
    .with_verification(
        EvidenceVerification::IndependentlyVerified,
        None,
    )
    .expect_err("missing verifier must be rejected");

    assert!(collection.extend(vec![invalid]).is_err());

    assert_eq!(
        collection.len(),
        original_length,
        "failed extension must not partially mutate the collection",
    );
}

// =============================================================================
// 5. AVAILABILITY ASSESSMENT
// =============================================================================

#[test]
fn missing_evidence_produces_unknown_availability() {
    let result = assess_capability(
        &capability("classical.compute"),
        &[],
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Unknown,
    );

    assert!(!result.is_available());
}

#[test]
fn available_evidence_establishes_availability() {
    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Available,
    );

    assert!(result.is_available());
}

#[test]
fn unavailable_evidence_establishes_unavailability() {
    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Unavailable,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Unavailable,
    );

    assert!(!result.is_available());
}

#[test]
fn conditional_evidence_does_not_establish_unconditional_availability() {
    let records = vec![
        evidence(
            "quantum.dynamic_control",
            CapabilityStatus::Conditional,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("quantum.dynamic_control"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Conditional,
    );

    assert!(!result.is_available());
}

#[test]
fn unknown_evidence_never_means_available() {
    let records = vec![
        evidence(
            "accelerator.tensor_execution",
            CapabilityStatus::Unknown,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("accelerator.tensor_execution"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Unknown,
    );

    assert!(!result.is_available());
}

#[test]
fn expired_evidence_does_not_establish_current_availability() {
    let records = vec![
        evidence(
            "hdl.synthesis",
            CapabilityStatus::Available,
            10,
            Some(50),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("hdl.synthesis"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Stale,
    );

    assert!(!result.is_available());
}

#[test]
fn temporary_inaccessibility_is_not_physical_unavailability() {
    let records = vec![
        evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("quantum.measurement"),
        &records,
        timestamp(100),
        AccessObservation::TemporarilyInaccessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::TemporarilyInaccessible,
    );

    assert!(!result.is_available());
    assert!(!result.state().is_unavailable());
}

#[test]
fn contradictory_current_evidence_is_not_silently_accepted() {
    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
        evidence(
            "classical.compute",
            CapabilityStatus::Unavailable,
            60,
            Some(160),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Conflicting,
    );

    assert!(!result.is_available());
}

#[test]
fn unrelated_capability_evidence_is_ignored() {
    let records = vec![
        evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        result.state(),
        AvailabilityState::Unknown,
    );
}

#[test]
fn single_requirement_availability_preserves_identity() {
    let required = requirement("hdl.synthesis");

    let records = vec![
        evidence(
            "hdl.synthesis",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let result = assess_requirement(
        &required,
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("requirement assessment must succeed");

    assert_eq!(
        result.capability(),
        required.capability(),
    );

    assert_eq!(
        result.state(),
        AvailabilityState::Available,
    );
}

// =============================================================================
// 6. REQUIREMENT PRESERVATION
// =============================================================================

#[test]
fn hardware_requirements_preserve_origin_and_rationale() {
    let required = HardwareCapabilityRequirement::new(
        capability("quantum.dynamic_control"),
    )
    .with_origin(
        RequirementOrigin::new(
            "quantum.semantic",
            Some("source.quantum.operation".to_owned()),
        )
        .expect("valid origin"),
    )
    .with_rationale(
        "The workload requires dynamic quantum control.",
    )
    .expect("valid rationale");

    assert_eq!(
        required.origin()
            .map(RequirementOrigin::identifier),
        Some("quantum.semantic"),
    );

    assert_eq!(
        required.rationale(),
        Some("The workload requires dynamic quantum control."),
    );
}

#[test]
fn requirement_collections_preserve_duplicates_and_order() {
    let first = requirement("classical.compute");
    let second = requirement("quantum.measurement");
    let third = requirement("classical.compute");

    let requirements =
        HardwareCapabilityRequirementSet::from_requirements(
            vec![
                first,
                second,
                third,
            ],
        );

    assert_eq!(requirements.len(), 3);

    assert_eq!(
        requirements
            .get(0)
            .expect("first requirement")
            .capability()
            .as_str(),
        "classical.compute",
    );

    assert_eq!(
        requirements
            .get(1)
            .expect("second requirement")
            .capability()
            .as_str(),
        "quantum.measurement",
    );

    assert_eq!(
        requirements
            .get(2)
            .expect("third requirement")
            .capability()
            .as_str(),
        "classical.compute",
    );
}

#[test]
fn empty_requirement_collection_is_valid() {
    let requirements =
        HardwareCapabilityRequirementSet::new();

    assert!(requirements.is_empty());
    assert!(requirements.validate().is_ok());
}

// =============================================================================
// 7. STRUCTURED COMPATIBILITY
// =============================================================================

#[test]
fn compatible_evidence_produces_a_structured_report() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"));

    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("classical.provider"),
            Some("processor.primary"),
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("compatibility evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );

    assert_eq!(report.len(), 1);
    assert_eq!(report.satisfied_count(), 1);
    assert_eq!(report.unavailable_count(), 0);
    assert_eq!(report.indeterminate_count(), 0);

    assert_eq!(
        report.findings()[0].kind(),
        CompatibilityFindingKind::Satisfied,
    );
}

#[test]
fn explicitly_unavailable_evidence_produces_incompatibility() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("hdl.synthesis"));

    let records = vec![
        evidence(
            "hdl.synthesis",
            CapabilityStatus::Unavailable,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Incompatible,
    );

    assert_eq!(report.unavailable_count(), 1);
}

#[test]
fn missing_evidence_produces_indeterminate_compatibility() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("future.photonic.computation"));

    let report = evaluate(
        &requirements,
        &[],
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );

    assert_eq!(report.indeterminate_count(), 1);
}

#[test]
fn empty_requirements_do_not_establish_compatibility() {
    let requirements =
        HardwareCapabilityRequirementSet::new();

    let report = evaluate(
        &requirements,
        &[],
        CompatibilityScope::AllEvidence,
    )
    .expect("empty evaluation must succeed");

    assert!(report.is_empty());

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );
}

#[test]
fn compatibility_preserves_duplicate_requirements() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"))
            .with(requirement("classical.compute"));

    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(report.len(), 2);
    assert_eq!(report.satisfied_count(), 2);

    assert_eq!(
        report.findings()[0].requirement_index(),
        0,
    );

    assert_eq!(
        report.findings()[1].requirement_index(),
        1,
    );
}

#[test]
fn compatibility_preserves_the_evaluation_timestamp() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"));

    let report = evaluate(
        &requirements,
        &[],
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.evaluated_at(),
        timestamp(100),
    );
}

#[test]
fn compatibility_does_not_treat_conditional_evidence_as_satisfied() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("quantum.dynamic_control"));

    let records = vec![
        evidence(
            "quantum.dynamic_control",
            CapabilityStatus::Conditional,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );

    assert_eq!(
        report.findings()[0].kind(),
        CompatibilityFindingKind::Conditional,
    );
}

// =============================================================================
// 8. PROVIDER AND DEVICE ISOLATION
// =============================================================================

#[test]
fn provider_scope_excludes_other_providers() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"));

    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("provider.one"),
            None,
        ),
        evidence(
            "classical.compute",
            CapabilityStatus::Unavailable,
            60,
            Some(160),
            Some("provider.two"),
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::Provider(
            provider("provider.one"),
        ),
    )
    .expect("provider-scoped evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );
}

#[test]
fn provider_scope_does_not_accept_unattributed_evidence() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("quantum.measurement"));

    let records = vec![
        evidence(
            "quantum.measurement",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::Provider(
            provider("quantum.provider"),
        ),
    )
    .expect("scoped evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );
}

#[test]
fn device_scope_excludes_evidence_from_other_devices() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("accelerator.tensor_execution"));

    let records = vec![
        evidence(
            "accelerator.tensor_execution",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("accelerator.provider"),
            Some("accelerator.device.one"),
        ),
        evidence(
            "accelerator.tensor_execution",
            CapabilityStatus::Unavailable,
            60,
            Some(160),
            Some("accelerator.provider"),
            Some("accelerator.device.two"),
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::Device(
            device(
                "accelerator.provider",
                "accelerator.device.one",
            ),
        ),
    )
    .expect("device-scoped evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );
}

#[test]
fn one_provider_cannot_satisfy_another_providers_scoped_requirement() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("hdl.synthesis"));

    let records = vec![
        evidence(
            "hdl.synthesis",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("provider.alpha"),
            Some("fpga.alpha"),
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::Provider(
            provider("provider.beta"),
        ),
    )
    .expect("scoped evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );
}

// =============================================================================
// 9. VERIFICATION POLICY
// =============================================================================

#[test]
fn default_policy_preserves_provider_reported_evidence() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("quantum.measurement"));

    let records = vec![
        verified_evidence(
            "quantum.measurement",
            EvidenceVerification::ProviderReported,
            None,
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );
}

#[test]
fn strict_verification_policy_rejects_unverified_evidence() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("quantum.measurement"));

    let records = vec![
        verified_evidence(
            "quantum.measurement",
            EvidenceVerification::Unverified,
            None,
        ),
    ];

    let report = evaluate_compatibility(
        &requirements,
        &records,
        timestamp(100),
        CompatibilityScope::AllEvidence,
        CompatibilityPolicy::new()
            .independently_verified_only(),
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Indeterminate,
    );

    assert_eq!(
        report.findings()[0].kind(),
        CompatibilityFindingKind::InsufficientVerification,
    );
}

#[test]
fn independently_verified_evidence_satisfies_strict_policy() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("quantum.measurement"));

    let records = vec![
        verified_evidence(
            "quantum.measurement",
            EvidenceVerification::IndependentlyVerified,
            Some("independent.verification.authority"),
        ),
    ];

    let report = evaluate_compatibility(
        &requirements,
        &records,
        timestamp(100),
        CompatibilityScope::AllEvidence,
        CompatibilityPolicy::new()
            .independently_verified_only(),
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );
}

// =============================================================================
// 10. DOMAIN PORTABILITY
// =============================================================================

#[test]
fn heterogeneous_domain_requirements_use_one_common_contract() {
    let names = [
        "classical.parallel_execution",
        "quantum.measurement",
        "hdl.synthesis",
        "accelerator.tensor_execution",
        "distributed.collective_execution",
        "embedded.sensor_io",
        "hybrid.cross_domain_execution",
        "future.photonic.computation",
    ];

    let requirements =
        HardwareCapabilityRequirementSet::from_requirements(
            names
                .iter()
                .map(|name| requirement(name)),
        );

    assert_eq!(
        requirements.len(),
        names.len(),
    );

    for (index, name) in names.iter().enumerate() {
        assert_eq!(
            requirements
                .get(index)
                .expect("requirement must exist")
                .capability()
                .as_str(),
            *name,
        );
    }
}

#[test]
fn domain_specific_capabilities_keep_distinct_identities() {
    let classical = capability("classical.parallel_execution");
    let quantum = capability("quantum.measurement");
    let hdl = capability("hdl.synthesis");
    let accelerator = capability("accelerator.tensor_execution");

    assert_ne!(classical, quantum);
    assert_ne!(quantum, hdl);
    assert_ne!(hdl, accelerator);
    assert_ne!(classical, accelerator);
}

// =============================================================================
// 11. LARGE AND DYNAMIC EVIDENCE COLLECTIONS
// =============================================================================

#[test]
fn large_evidence_collections_do_not_introduce_hardware_ceilings() {
    // This is a representative stress workload.
    //
    // It is not a maximum supported provider count, device count,
    // capability count or evidence count.
    //
    // The actual collection grows according to available resources.

    let record_count = 2048usize;

    let mut records =
        Vec::with_capacity(record_count);

    for index in 0..record_count {
        records.push(
            evidence(
                "classical.parallel_execution",
                CapabilityStatus::Available,
                50,
                Some(150),
                Some(&format!("provider.{index}")),
                None,
            ),
        );
    }

    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.parallel_execution"));

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("large evidence evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );

    assert_eq!(
        report.findings()[0].evidence_counts().total(),
        record_count,
    );
}

#[test]
fn additional_hardware_observations_can_be_added_dynamically() {
    let mut records =
        HardwareCapabilityEvidenceSet::new();

    records
        .insert(evidence(
            "accelerator.tensor_execution",
            CapabilityStatus::Unknown,
            50,
            Some(150),
            Some("accelerator.provider"),
            Some("accelerator.device"),
        ))
        .expect("initial observation");

    let initial = assess_capability(
        &capability("accelerator.tensor_execution"),
        &records.iter().cloned().collect::<Vec<_>>(),
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("initial assessment");

    assert_eq!(
        initial.state(),
        AvailabilityState::Unknown,
    );

    records
        .insert(evidence(
            "accelerator.tensor_execution",
            CapabilityStatus::Available,
            75,
            Some(175),
            Some("accelerator.provider"),
            Some("accelerator.device"),
        ))
        .expect("updated observation");

    let updated = assess_capability(
        &capability("accelerator.tensor_execution"),
        &records.iter().cloned().collect::<Vec<_>>(),
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("updated assessment");

    assert_eq!(
        updated.state(),
        AvailabilityState::Available,
    );
}

#[test]
fn large_requirement_collections_preserve_every_declaration() {
    // The count is a representative test workload, not a semantic
    // or architectural requirement limit.

    let requirement_count = 1024usize;

    let requirements =
        HardwareCapabilityRequirementSet::from_requirements(
            (0..requirement_count).map(|index| {
                requirement(&format!(
                    "future.domain.capability_{index}"
                ))
            }),
        );

    assert_eq!(
        requirements.len(),
        requirement_count,
    );

    assert_eq!(
        requirements.iter().count(),
        requirement_count,
    );

    assert!(requirements.validate().is_ok());
}

// =============================================================================
// 12. DETERMINISTIC ASSESSMENT
// =============================================================================

#[test]
fn identical_inputs_produce_identical_availability_assessments() {
    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ),
    ];

    let first = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("first assessment");

    let second = assess_capability(
        &capability("classical.compute"),
        &records,
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("second assessment");

    assert_eq!(first, second);
}

#[test]
fn identical_inputs_produce_identical_compatibility_reports() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"));

    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("classical.provider"),
            Some("processor.primary"),
        ),
    ];

    let first = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("first evaluation");

    let second = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("second evaluation");

    assert_eq!(first, second);
}

// =============================================================================
// 13. CAPABILITY AUTHORITY BOUNDARIES
// =============================================================================

#[test]
fn capability_requirement_does_not_create_physical_evidence() {
    let required = requirement("quantum.measurement");

    assert_eq!(
        required.capability().as_str(),
        "quantum.measurement",
    );

    // Creating a requirement must not fabricate an observation.
    let assessment = assess_capability(
        required.capability(),
        &[],
        timestamp(100),
        AccessObservation::Accessible,
    )
    .expect("assessment must succeed");

    assert_eq!(
        assessment.state(),
        AvailabilityState::Unknown,
    );
}

#[test]
fn compatibility_does_not_claim_resource_reservation() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("classical.compute"));

    let records = vec![
        evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("classical.provider"),
            Some("processor.primary"),
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );

    // Compatibility is an evidence-based assessment.
    // It does not reserve a processor or allocate resources.
    // Such actions belong to the canonical runtime authority.
}

#[test]
fn compatibility_does_not_claim_physical_execution() {
    let requirements =
        HardwareCapabilityRequirementSet::new()
            .with(requirement("hdl.synthesis"));

    let records = vec![
        evidence(
            "hdl.synthesis",
            CapabilityStatus::Available,
            50,
            Some(150),
            Some("hdl.provider"),
            Some("fpga.device"),
        ),
    ];

    let report = evaluate(
        &requirements,
        &records,
        CompatibilityScope::AllEvidence,
    )
    .expect("evaluation must succeed");

    assert_eq!(
        report.status(),
        CompatibilityStatus::Compatible,
    );

    // A compatibility report is not a synthesis result,
    // FPGA programming receipt or realization report.
    // Those remain owned by src/hdl/realization.
}

// =============================================================================
// 14. CANONICAL STANDARD-LIBRARY INTEGRATION
// =============================================================================

#[test]
fn zhal_evidence_adapts_to_standard_library_evidence() {
    let record = evidence(
        "classical.parallel_execution",
        CapabilityStatus::Available,
        50,
        Some(150),
        Some("classical.provider"),
        None,
    );

    let standard = record
        .to_standard_evidence()
        .expect("standard evidence conversion must succeed");

    assert_eq!(
        standard.source(),
        "test hardware provider",
    );

    assert_eq!(
        standard.description(),
        "test physical hardware capability observation",
    );
}

#[test]
fn evidence_collection_validates_all_inserted_records() {
    let mut records =
        HardwareCapabilityEvidenceSet::new();

    records
        .insert(evidence(
            "classical.compute",
            CapabilityStatus::Available,
            50,
            Some(150),
            None,
            None,
        ))
        .expect("valid evidence");

    records
        .insert(evidence(
            "quantum.measurement",
            CapabilityStatus::Conditional,
            60,
            Some(160),
            None,
            None,
        ))
        .expect("valid evidence");

    assert!(records.validate().is_ok());
    assert_eq!(records.len(), 2);
}

// =============================================================================
// END OF ZHAL CAPABILITY CONFORMANCE TESTS
// =============================================================================
