//! Backend compatibility contract tests for Zamani.
//!
//! Path: src/backend/tests/compatibility.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Verify the public compatibility contract implemented by
//! `src/backend/compatibility.rs`.
//!
//! These tests cover:
//! - required capabilities and resources;
//! - unsupported, partial, experimental, and unknown support;
//! - complete versus incomplete capability inventories;
//! - preference and hint semantics;
//! - arbitrary-length decimal resource quantities;
//! - malformed and duplicate metadata;
//! - deterministic compatibility reports;
//! - extensible capability and resource namespaces;
//! - combinations of requirements across computing domains.
//!
//! # Ownership boundary
//!
//! These tests do not define source-language semantics, select physical
//! devices, allocate resources, perform quantum routing or QEC, synthesize
//! HDL, or generate target code. Those responsibilities remain with their
//! existing authoritative subsystems.
//!
//! Grammar and semantic specifications define program meaning.
//! Canonical domain IRs preserve domain-specific meaning.
//! This compatibility contract assesses declared requirements against
//! target evidence.
//!
//! # Integration
//!
//! The existing repository has both `src/backend.rs` and
//! `src/backend/mod.rs`. Until that module-ownership conflict is resolved,
//! `src/backend.rs` remains the current implementation root.
//!
//! The compatibility module must be declared from that root, and this test
//! module must be wired there as described after this file.
//!
//! # Resource scalability
//!
//! These tests intentionally avoid universal hardware ceilings.
//! Large decimal quantities and long names test representation behavior;
//! they do not claim that physical resources or compilation time are
//! unlimited. Actual resource exhaustion remains an explicit operational
//! concern, not a change to source-language semantics.

#![forbid(unsafe_code)]

use crate::backend::compatibility::{
    CapabilityId,
    CapabilityRequirement,
    CapabilitySupport,
    CompatibilityError,
    CompatibilityEvaluator,
    CompatibilityIssueCode,
    CompatibilityPolicy,
    CompatibilityRequest,
    CompatibilitySeverity,
    CompatibilityStatus,
    DecimalInteger,
    InventoryCompleteness,
    RequirementStrength,
    ResourceAvailability,
    ResourceId,
    ResourceRequirement,
    SupportLevel,
    TargetCapabilityProfile,
};

fn capability(value: &str) -> CapabilityId {
    CapabilityId::new(value).expect("test capability identifier must be valid")
}

fn resource(value: &str) -> ResourceId {
    ResourceId::new(value).expect("test resource identifier must be valid")
}

fn quantity(value: &str) -> DecimalInteger {
    DecimalInteger::new(value).expect("test quantity must be valid")
}

fn target() -> TargetCapabilityProfile {
    TargetCapabilityProfile::new("test.target")
        .expect("test target identifier must be valid")
}

fn report(
    request: &CompatibilityRequest,
    target: &TargetCapabilityProfile,
) -> crate::backend::compatibility::CompatibilityReport {
    CompatibilityEvaluator.evaluate(request, target)
}

fn declare_capability(
    target: &mut TargetCapabilityProfile,
    name: &str,
    level: SupportLevel,
) {
    target
        .declare_capability(CapabilitySupport::new(capability(name), level))
        .expect("capability should be declared only once in this fixture");
}

fn declare_resource(
    target: &mut TargetCapabilityProfile,
    name: &str,
    availability: ResourceAvailability,
) {
    target
        .declare_resource(resource(name), availability)
        .expect("resource should be declared only once in this fixture");
}

// ---------------------------------------------------------------------------
// Capability requirements
// ---------------------------------------------------------------------------

#[test]
fn supported_hard_capability_is_compatible() {
    let mut profile = target();
    declare_capability(&mut profile, "cpu.integer", SupportLevel::Supported);

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability("cpu.integer")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.is_compatible());
    assert!(result.is_fully_compatible());
    assert!(!result.has_errors());
    assert!(result.issues.is_empty());
}

#[test]
fn explicitly_unsupported_hard_capability_is_incompatible() {
    let mut profile = target();
    declare_capability(
        &mut profile,
        "quantum.mid_circuit_measurement",
        SupportLevel::Unsupported,
    );

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability(
            "quantum.mid_circuit_measurement",
        )),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(!result.is_compatible());
    assert!(result.has_errors());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityUnsupported
            && issue.severity == CompatibilitySeverity::Error
            && issue.subject == "quantum.mid_circuit_measurement"
    }));
}

#[test]
fn partial_support_never_satisfies_a_hard_requirement() {
    let mut profile = target();

    profile
        .declare_capability(
            CapabilitySupport::new(
                capability("quantum.measurement"),
                SupportLevel::Partial,
            )
            .with_evidence("Only a documented subset is implemented")
            .with_limitation("Mid-circuit measurement is not supported"),
        )
        .expect("capability declaration should succeed");

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability("quantum.measurement")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityPartiallySupported
            && issue.severity == CompatibilitySeverity::Error
            && issue.evidence.as_deref().is_some_and(|evidence| {
                evidence.contains("Mid-circuit measurement")
            })
    }));
}

#[test]
fn unknown_capability_evidence_does_not_count_as_success() {
    let profile = target();

    // A newly created profile has a partial inventory. An absent entry
    // therefore means "unknown", not "definitely unsupported".
    assert_eq!(
        profile.inventory_completeness,
        InventoryCompleteness::Partial
    );

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability("future.domain.operation")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Indeterminate);
    assert!(!result.is_compatible());
    assert!(result.has_errors());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityUnknown
            && issue.severity == CompatibilitySeverity::Error
    }));
}

#[test]
fn complete_capability_inventory_makes_absent_hard_capability_unsupported() {
    let mut profile = target();
    profile.mark_capability_inventory_complete();

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability("quantum.error_correction")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityUnsupported
            && issue.severity == CompatibilitySeverity::Error
    }));
}

#[test]
fn experimental_hard_capability_is_rejected_by_default() {
    let mut profile = target();
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Experimental,
    );

    let request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityExperimental
            && issue.severity == CompatibilitySeverity::Error
    }));
}

#[test]
fn experimental_hard_capability_requires_explicit_policy_permission() {
    let mut profile = target();
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Experimental,
    );

    let request = CompatibilityRequest::new()
        .with_policy(CompatibilityPolicy {
            allow_experimental_for_hard_requirements: true,
        })
        .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

    let result = report(&request, &profile);

    assert_eq!(
        result.status,
        CompatibilityStatus::CompatibleWithWarnings
    );
    assert!(result.is_compatible());
    assert!(!result.is_fully_compatible());
    assert!(result.has_warnings());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityExperimental
            && issue.severity == CompatibilitySeverity::Warning
    }));
}

#[test]
fn adding_a_future_capability_does_not_require_a_closed_domain_enum() {
    let future_capability =
        "future.accelerator.sparse_tensor_operation:version+2";

    let mut profile = target();
    declare_capability(
        &mut profile,
        future_capability,
        SupportLevel::Supported,
    );

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::required(capability(future_capability)),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.issues.is_empty());
}

// ---------------------------------------------------------------------------
// Preferences and hints
// ---------------------------------------------------------------------------

#[test]
fn unsupported_preference_warns_without_blocking_compatibility() {
    let mut profile = target();
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Unsupported,
    );

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::new(
            capability("gpu.compute"),
            RequirementStrength::Preference,
        ),
    );

    let result = report(&request, &profile);

    assert_eq!(
        result.status,
        CompatibilityStatus::CompatibleWithWarnings
    );
    assert!(result.is_compatible());
    assert!(!result.has_errors());
    assert!(result.has_warnings());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityPreferenceUnavailable
            && issue.severity == CompatibilitySeverity::Warning
            && issue.strength == RequirementStrength::Preference
    }));
}

#[test]
fn unsupported_hint_does_not_become_a_hard_requirement() {
    let mut profile = target();
    declare_capability(
        &mut profile,
        "cpu.vector_acceleration",
        SupportLevel::Unsupported,
    );

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::new(
            capability("cpu.vector_acceleration"),
            RequirementStrength::Hint,
        ),
    );

    let result = report(&request, &profile);

    assert!(result.is_compatible());
    assert!(!result.has_errors());
    assert_eq!(
        result.status,
        CompatibilityStatus::CompatibleWithWarnings
    );
}

#[test]
fn unknown_preference_is_informational_not_a_blocker() {
    let profile = target();

    let request = CompatibilityRequest::new().with_capability(
        CapabilityRequirement::new(
            capability("future.optional.optimization"),
            RequirementStrength::Preference,
        ),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.is_compatible());
    assert!(!result.has_errors());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityUnknown
            && issue.severity == CompatibilitySeverity::Info
    }));
}

// ---------------------------------------------------------------------------
// Resource requirements and availability
// ---------------------------------------------------------------------------

#[test]
fn exact_resource_capacity_satisfies_a_hard_minimum() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "quantum.logical_qubits",
        ResourceAvailability::known(quantity("256")),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(
            resource("quantum.logical_qubits"),
            quantity("256"),
        ),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.issues.is_empty());
}

#[test]
fn resource_capacity_above_the_minimum_satisfies_a_hard_requirement() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "memory.bytes",
        ResourceAvailability::known(quantity("1048577")),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("memory.bytes"), quantity("1048576")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.issues.is_empty());
}

#[test]
fn insufficient_known_resource_capacity_blocks_a_hard_requirement() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "cpu.cores",
        ResourceAvailability::known(quantity("7")),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("cpu.cores"), quantity("8")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceInsufficient
            && issue.severity == CompatibilitySeverity::Error
            && issue.subject == "cpu.cores"
    }));
}

#[test]
fn unavailable_resource_blocks_a_hard_requirement() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "hdl.synthesis",
        ResourceAvailability::unavailable("No compatible synthesis toolchain is available"),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("hdl.synthesis"), quantity("1")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceUnavailable
            && issue.severity == CompatibilitySeverity::Error
            && issue.evidence.as_deref().is_some_and(|evidence| {
                evidence.contains("synthesis toolchain")
            })
    }));
}

#[test]
fn unknown_resource_capacity_is_indeterminate_for_a_hard_requirement() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "distributed.nodes",
        ResourceAvailability::unknown(),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("distributed.nodes"), quantity("100")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Indeterminate);
    assert!(!result.is_compatible());
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceUnknown
            && issue.severity == CompatibilitySeverity::Error
    }));
}

#[test]
fn missing_resource_in_a_partial_inventory_is_not_proven_unavailable() {
    let profile = target();

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("memory.bytes"), quantity("1")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Indeterminate);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceUnknown
    }));
}

#[test]
fn missing_resource_in_a_complete_inventory_blocks_a_hard_requirement() {
    let mut profile = target();
    profile.mark_capability_inventory_complete();

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::required(resource("quantum.physical_qubits"), quantity("1")),
    );

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceUnavailable
            && issue.severity == CompatibilitySeverity::Error
    }));
}

#[test]
fn insufficient_preferred_resource_warns_without_blocking_compatibility() {
    let mut profile = target();
    declare_resource(
        &mut profile,
        "network.bandwidth",
        ResourceAvailability::known(quantity("9")),
    );

    let request = CompatibilityRequest::new().with_resource(
        ResourceRequirement::new(
            resource("network.bandwidth"),
            quantity("10"),
            RequirementStrength::Preference,
        ),
    );

    let result = report(&request, &profile);

    assert_eq!(
        result.status,
        CompatibilityStatus::CompatibleWithWarnings
    );
    assert!(result.is_compatible());
    assert!(!result.has_errors());

    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourcePreferenceUnavailable
            && issue.severity == CompatibilitySeverity::Warning
    }));
}

// ---------------------------------------------------------------------------
// Arbitrary-length quantities and extensibility
// ---------------------------------------------------------------------------

#[test]
fn decimal_quantities_support_values_beyond_machine_integer_widths() {
    let huge_value = format!("9{}", "0".repeat(4096));
    let huge = quantity(&huge_value);
    let smaller = quantity(&format!("8{}", "9".repeat(4096)));

    assert!(huge > smaller);
    assert_eq!(huge.as_str(), huge_value);
}

#[test]
fn leading_zeroes_do_not_change_resource_quantity_identity() {
    assert_eq!(quantity("00000000000042"), quantity("42"));
    assert_eq!(quantity("00000000000000"), DecimalInteger::zero());
}

#[test]
fn decimal_quantity_rejects_negative_fractional_and_exponential_forms() {
    assert!(DecimalInteger::new("-1").is_err());
    assert!(DecimalInteger::new("+1").is_err());
    assert!(DecimalInteger::new("1.25").is_err());
    assert!(DecimalInteger::new("1e9").is_err());
    assert!(DecimalInteger::new("").is_err());
}

#[test]
fn extensible_identifiers_do_not_have_an_arbitrary_short_length_limit() {
    let long_capability = format!("future.{}", "x".repeat(4096));
    let long_resource = format!("resource.{}", "y".repeat(4096));

    let capability_id = capability(&long_capability);
    let resource_id = resource(&long_resource);

    assert_eq!(capability_id.as_str(), long_capability);
    assert_eq!(resource_id.as_str(), long_resource);
}

// ---------------------------------------------------------------------------
// Invalid and duplicate metadata
// ---------------------------------------------------------------------------

#[test]
fn malformed_capability_and_resource_identifiers_are_rejected() {
    assert!(matches!(
        CapabilityId::new(""),
        Err(CompatibilityError::EmptyIdentifier { .. })
    ));

    assert!(matches!(
        CapabilityId::new("quantum measurement"),
        Err(CompatibilityError::InvalidIdentifier { .. })
    ));

    assert!(matches!(
        ResourceId::new("memory bytes"),
        Err(CompatibilityError::InvalidIdentifier { .. })
    ));

    assert!(TargetCapabilityProfile::new("   ").is_err());
}

#[test]
fn duplicate_capability_declarations_are_rejected() {
    let mut profile = target();

    profile
        .declare_capability(CapabilitySupport::new(
            capability("cpu.integer"),
            SupportLevel::Supported,
        ))
        .expect("first declaration should succeed");

    let duplicate = profile.declare_capability(CapabilitySupport::new(
        capability("cpu.integer"),
        SupportLevel::Unsupported,
    ));

    assert!(matches!(
        duplicate,
        Err(CompatibilityError::DuplicateCapability { .. })
    ));

    // A rejected duplicate must not silently overwrite the original evidence.
    assert_eq!(
        profile
            .capability(&capability("cpu.integer"))
            .expect("original declaration must remain present")
            .level,
        SupportLevel::Supported
    );
}

#[test]
fn duplicate_resource_declarations_are_rejected() {
    let mut profile = target();

    profile
        .declare_resource(
            resource("memory.bytes"),
            ResourceAvailability::known(quantity("100")),
        )
        .expect("first declaration should succeed");

    let duplicate = profile.declare_resource(
        resource("memory.bytes"),
        ResourceAvailability::known(quantity("0")),
    );

    assert!(matches!(
        duplicate,
        Err(CompatibilityError::DuplicateResource { .. })
    ));

    // The duplicate must not replace the original capacity.
    match profile
        .resource(&resource("memory.bytes"))
        .expect("original resource declaration must remain present")
    {
        ResourceAvailability::Known { capacity, .. } => {
            assert_eq!(capacity, &quantity("100"));
        }
        other => panic!("unexpected resource availability: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Determinism and combined requirements
// ---------------------------------------------------------------------------

#[test]
fn compatibility_report_is_deterministic_for_identical_inputs() {
    let mut profile = target();

    declare_capability(
        &mut profile,
        "quantum.measurement",
        SupportLevel::Unsupported,
    );
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Partial,
    );
    declare_resource(
        &mut profile,
        "memory.bytes",
        ResourceAvailability::known(quantity("4")),
    );

    let request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("gpu.compute")))
        .with_capability(CapabilityRequirement::required(capability("quantum.measurement")))
        .with_resource(ResourceRequirement::required(
            resource("memory.bytes"),
            quantity("8"),
        ));

    let first = report(&request, &profile);
    let second = report(&request, &profile);

    assert_eq!(first, second);
}

#[test]
fn compatibility_report_order_does_not_depend_on_requirement_insertion_order() {
    let mut profile = target();

    declare_capability(
        &mut profile,
        "quantum.measurement",
        SupportLevel::Unsupported,
    );
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Partial,
    );
    declare_resource(
        &mut profile,
        "memory.bytes",
        ResourceAvailability::known(quantity("4")),
    );

    let first_request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("gpu.compute")))
        .with_capability(CapabilityRequirement::required(capability("quantum.measurement")))
        .with_resource(ResourceRequirement::required(
            resource("memory.bytes"),
            quantity("8"),
        ));

    let second_request = CompatibilityRequest::new()
        .with_resource(ResourceRequirement::required(
            resource("memory.bytes"),
            quantity("8"),
        ))
        .with_capability(CapabilityRequirement::required(capability("quantum.measurement")))
        .with_capability(CapabilityRequirement::required(capability("gpu.compute")));

    assert_eq!(
        report(&first_request, &profile),
        report(&second_request, &profile)
    );
}

#[test]
fn one_unsupported_hard_requirement_blocks_an_otherwise_supported_request() {
    let mut profile = target();

    declare_capability(
        &mut profile,
        "cpu.integer",
        SupportLevel::Supported,
    );
    declare_capability(
        &mut profile,
        "distributed.execution",
        SupportLevel::Unsupported,
    );
    declare_resource(
        &mut profile,
        "memory.bytes",
        ResourceAvailability::known(quantity("1048576")),
    );

    let request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("cpu.integer")))
        .with_capability(CapabilityRequirement::required(capability("distributed.execution")))
        .with_resource(ResourceRequirement::required(
            resource("memory.bytes"),
            quantity("1024"),
        ));

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert!(result.has_errors());
    assert!(result.issues.iter().any(|issue| {
        issue.subject == "distributed.execution"
            && issue.code == CompatibilityIssueCode::CapabilityUnsupported
    }));
}

#[test]
fn unknown_evidence_is_distinguished_from_proven_incompatibility() {
    let mut profile = target();

    // The CPU capability is explicitly supported, while the resource
    // inventory has no evidence for the requested memory capacity.
    declare_capability(&mut profile, "cpu.integer", SupportLevel::Supported);

    let request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("cpu.integer")))
        .with_resource(ResourceRequirement::required(
            resource("memory.bytes"),
            quantity("4096"),
        ));

    let result = report(&request, &profile);

    assert_eq!(result.status, CompatibilityStatus::Indeterminate);
    assert!(result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::ResourceUnknown
            && issue.severity == CompatibilitySeverity::Error
    }));
    assert!(!result.issues.iter().any(|issue| {
        issue.code == CompatibilityIssueCode::CapabilityUnsupported
    }));
}

#[test]
fn report_identifies_the_target_that_was_evaluated() {
    let profile = TargetCapabilityProfile::new("vendor.target/revision:2")
        .expect("target identifiers should support namespaced target identities");

    let result = report(&CompatibilityRequest::new(), &profile);

    assert_eq!(result.target_id, "vendor.target/revision:2");
    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert!(result.issues.is_empty());
}

#[test]
fn report_error_and_warning_iterators_match_their_severities() {
    let mut profile = target();

    declare_capability(
        &mut profile,
        "quantum.measurement",
        SupportLevel::Unsupported,
    );
    declare_capability(
        &mut profile,
        "gpu.compute",
        SupportLevel::Unsupported,
    );

    let request = CompatibilityRequest::new()
        .with_capability(CapabilityRequirement::required(capability("quantum.measurement")))
        .with_capability(CapabilityRequirement::new(
            capability("gpu.compute"),
            RequirementStrength::Preference,
        ));

    let result = report(&request, &profile);

    assert!(result.errors().all(|issue| {
        issue.severity == CompatibilitySeverity::Error
    }));
    assert!(result.warnings().all(|issue| {
        issue.severity == CompatibilitySeverity::Warning
    }));
    assert_eq!(result.errors().count(), 1);
    assert_eq!(result.warnings().count(), 1);
}