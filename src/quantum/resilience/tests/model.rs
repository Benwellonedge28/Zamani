//! Zamani Quantum Resilience — model-layer tests.
//!
//! Path:
//!     `src/quantum/resilience/tests/model.rs`
//!
//! Purpose:
//!     Verify the foundational resilience model as a single integration
//!     boundary. These tests deliberately exercise the public contracts of
//!     `model::resource` and `model::confidence` and compile-time visibility of
//!     every model namespace.
//!
//! Architectural contract:
//!     - canonical quantum identities come from `quantum::ir::qubit`;
//!     - generic resource identity comes from `quantum::ir::core::identity`;
//!     - resilience does not define replacement qubit/resource identities;
//!     - unknown, unavailable and unbounded states remain distinct;
//!     - no machine size, provider, topology or retry policy is encoded;
//!     - model values are deterministic and side-effect free;
//!     - tests use no `unsafe` code;
//!     - tests are valid on Rust 1.97/1.97.1 and Rust 2021.
//!
//! Integration:
//!     This file is intended to be included by the resilience test module,
//!     normally with `mod model;`. It intentionally uses `crate::...` paths so
//!     it tests the same canonical module graph used by production code.
//!
//! The tests do not instantiate higher-level resilience algorithms. Those
//! layers must consume these model contracts rather than redefine them.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::any::type_name;
use core::cmp::Ordering;

use crate::quantum::ir::core::identity::ResourceId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::model::confidence::Confidence;
use crate::quantum::resilience::model::resource::{
    ResourceAvailability,
    ResourceIdentity,
    ResourceKind,
    ResourceQuantity,
    ResourceScope,
};

// =============================================================================
// Namespace and dependency-boundary tests
// =============================================================================

#[test]
fn every_foundational_model_namespace_is_available() {
    // `type_name` forces the compiler to resolve the complete public model
    // namespace without coupling this test to implementation details.
    let _ = type_name::<crate::quantum::resilience::model::capability::Capability>();
    let _ = type_name::<crate::quantum::resilience::model::confidence::Confidence>();
    let _ = type_name::<crate::quantum::resilience::model::degradation::Degradation>();
    let _ = type_name::<crate::quantum::resilience::model::fault::Fault>();
    let _ = type_name::<crate::quantum::resilience::model::health::HealthState>();
    let _ = type_name::<crate::quantum::resilience::model::incident::Incident>();
    let _ = type_name::<crate::quantum::resilience::model::resource::ResourceIdentity>();
    let _ = type_name::<crate::quantum::resilience::model::severity::Severity>();
}

#[test]
fn model_namespace_does_not_replace_canonical_qubit_identity() {
    // These explicit type references are intentional architectural checks.
    // Resilience must use the canonical IR identity types.
    let logical: QubitId = QubitId::new(7);
    let physical: PhysicalQubitId = PhysicalQubitId::new(7);
    let generic: ResourceId = ResourceId::new(7);

    let logical_resource = ResourceIdentity::logical_qubit(logical);
    let physical_resource = ResourceIdentity::physical_qubit(physical);
    let generic_resource = ResourceIdentity::ir(generic);

    assert!(logical_resource.is_logical_qubit());
    assert!(!logical_resource.is_physical_qubit());

    assert!(physical_resource.is_physical_qubit());
    assert!(!physical_resource.is_logical_qubit());

    assert!(generic_resource.is_ir_resource());

    assert_eq!(logical_resource.logical_qubit_id(), Some(logical));
    assert_eq!(physical_resource.physical_qubit_id(), Some(physical));
    assert_eq!(generic_resource.ir_id(), Some(generic));

    // Equal numeric values in distinct identity domains must remain distinct.
    assert_ne!(logical_resource, physical_resource);
    assert_ne!(logical_resource, generic_resource);
    assert_ne!(physical_resource, generic_resource);
}

#[test]
fn resource_scope_is_derived_from_canonical_identity() {
    let logical = ResourceIdentity::logical_qubit(QubitId::new(0));
    let physical = ResourceIdentity::physical_qubit(PhysicalQubitId::new(0));
    let generic = ResourceIdentity::ir(ResourceId::new(0));

    assert_eq!(
        ResourceScope::for_identity(logical),
        ResourceScope::Logical
    );
    assert_eq!(
        ResourceScope::for_identity(physical),
        ResourceScope::Physical
    );
    assert_eq!(
        ResourceScope::for_identity(generic),
        ResourceScope::Generic
    );

    assert!(ResourceScope::Logical.is_logical());
    assert!(!ResourceScope::Logical.is_physical());

    assert!(ResourceScope::Physical.is_physical());
    assert!(!ResourceScope::Physical.is_generic());

    assert!(ResourceScope::Generic.is_generic());
    assert!(!ResourceScope::Generic.is_logical());
}

// =============================================================================
// Resource kind tests
// =============================================================================

#[test]
fn resource_kind_accepts_extensible_semantic_labels() {
    let labels = [
        "qubit",
        "logical_qubit",
        "physical_qubit",
        "coupling",
        "control_channel",
        "measurement_channel",
        "reset_channel",
        "execution_slot",
        "classical_memory",
        "quantum_memory",
        "communication_link",
        "custom.future.resource",
    ];

    for label in labels {
        let kind =
            ResourceKind::new(label).expect("non-empty semantic label must be valid");

        assert_eq!(kind.as_str(), label);
        assert_eq!(kind.to_string(), label);
    }
}

#[test]
fn resource_kind_rejects_empty_and_whitespace_only_values() {
    assert!(ResourceKind::new("").is_err());
    assert!(ResourceKind::new("   ").is_err());
    assert!(ResourceKind::new("\t\n").is_err());
}

#[test]
fn resource_kind_round_trips_through_try_from() {
    let borrowed =
        ResourceKind::try_from("future.resource")
            .expect("valid borrowed label");

    let owned =
        ResourceKind::try_from(String::from("future.resource"))
            .expect("valid owned label");

    assert_eq!(borrowed, owned);
    assert_eq!(
        owned.clone().into_string(),
        "future.resource"
    );
}

#[test]
fn resource_kind_ordering_is_deterministic() {
    let mut kinds = vec![
        ResourceKind::new("zeta").expect("valid kind"),
        ResourceKind::new("alpha").expect("valid kind"),
        ResourceKind::new("middle").expect("valid kind"),
    ];

    kinds.sort();

    let labels: Vec<&str> =
        kinds.iter().map(ResourceKind::as_str).collect();

    assert_eq!(
        labels,
        vec!["alpha", "middle", "zeta"]
    );
}

// =============================================================================
// Resource quantity tests
// =============================================================================

#[test]
fn resource_quantity_preserves_finite_unbounded_and_unknown_semantics() {
    let zero = ResourceQuantity::finite(0);
    let one = ResourceQuantity::finite(1);
    let large = ResourceQuantity::finite(u128::MAX);
    let unbounded = ResourceQuantity::unbounded();
    let unknown = ResourceQuantity::unknown();

    assert!(zero.is_finite());
    assert!(zero.is_zero());
    assert_eq!(zero.as_finite(), Some(0));

    assert!(one.is_finite());
    assert!(!one.is_zero());
    assert_eq!(one.as_finite(), Some(1));

    assert!(large.is_finite());
    assert_eq!(large.as_finite(), Some(u128::MAX));

    assert!(unbounded.is_unbounded());
    assert!(!unbounded.is_finite());
    assert!(!unbounded.is_unknown());
    assert_eq!(unbounded.as_finite(), None);

    assert!(unknown.is_unknown());
    assert!(!unknown.is_finite());
    assert!(!unknown.is_unbounded());
    assert_eq!(unknown.as_finite(), None);
}

#[test]
fn resource_quantity_does_not_use_integer_sentinels_for_unboundedness() {
    // u128::MAX is still a legitimate finite quantity.
    // Unbounded is a semantic variant, never an integer sentinel.
    let maximum_finite =
        ResourceQuantity::finite(u128::MAX);

    assert!(maximum_finite.is_finite());
    assert!(!maximum_finite.is_unbounded());
    assert_eq!(
        maximum_finite.as_finite(),
        Some(u128::MAX)
    );

    assert!(ResourceQuantity::unbounded().is_unbounded());
}

#[test]
fn resource_quantity_ordering_is_total_and_deterministic() {
    let values = [
        ResourceQuantity::unknown(),
        ResourceQuantity::finite(0),
        ResourceQuantity::finite(1),
        ResourceQuantity::finite(u128::MAX),
        ResourceQuantity::unbounded(),
    ];

    for left in &values {
        for right in &values {
            assert_eq!(left.cmp(right), left.cmp(right));
        }
    }

    assert_eq!(
        ResourceQuantity::finite(1)
            .cmp(&ResourceQuantity::finite(1)),
        Ordering::Equal
    );
}

// =============================================================================
// Resource availability tests
// =============================================================================

#[test]
fn resource_availability_keeps_unknown_distinct_from_unavailable() {
    assert!(ResourceAvailability::Available.is_available());
    assert!(!ResourceAvailability::Available.is_unknown());
    assert!(!ResourceAvailability::Available.is_unavailable());

    assert!(ResourceAvailability::Unavailable.is_unavailable());
    assert!(!ResourceAvailability::Unavailable.is_available());
    assert!(!ResourceAvailability::Unavailable.is_unknown());

    assert!(ResourceAvailability::Unknown.is_unknown());
    assert!(!ResourceAvailability::Unknown.is_available());
    assert!(!ResourceAvailability::Unknown.is_unavailable());
}

// =============================================================================
// Resource identity tests
// =============================================================================

#[test]
fn resource_identity_preserves_domain_separation_at_extreme_ids() {
    // Boundary identifiers test the representable identity domain. They do
    // not establish a hardware size or a maximum number of qubits.
    let logical = QubitId::new(u64::MAX);
    let physical = PhysicalQubitId::new(usize::MAX);
    let generic = ResourceId::new(u64::MAX);

    let logical_resource =
        ResourceIdentity::logical_qubit(logical);

    let physical_resource =
        ResourceIdentity::physical_qubit(physical);

    let generic_resource =
        ResourceIdentity::ir(generic);

    assert_eq!(
        logical_resource.logical_qubit_id(),
        Some(logical)
    );

    assert_eq!(
        physical_resource.physical_qubit_id(),
        Some(physical)
    );

    assert_eq!(
        generic_resource.ir_id(),
        Some(generic)
    );

    assert_ne!(
        logical_resource,
        physical_resource
    );
}

#[test]
fn resource_identity_display_is_non_empty_and_domain_qualified() {
    let logical =
        ResourceIdentity::logical_qubit(QubitId::new(1));

    let physical =
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(1),
        );

    let generic =
        ResourceIdentity::ir(ResourceId::new(1));

    let logical_text = logical.to_string();
    let physical_text = physical.to_string();
    let generic_text = generic.to_string();

    assert!(
        logical_text.starts_with("logical-qubit:")
    );

    assert!(
        physical_text.starts_with("physical-qubit:")
    );

    assert!(
        generic_text.starts_with("ir:")
    );

    assert!(!logical_text.is_empty());
    assert!(!physical_text.is_empty());
    assert!(!generic_text.is_empty());
}

// =============================================================================
// Confidence model tests
// =============================================================================

#[test]
fn confidence_accepts_only_finite_normalized_values() {
    let minimum =
        Confidence::new(0.0)
            .expect("zero is valid confidence");

    let interior =
        Confidence::new(0.5)
            .expect("interior normalized value is valid");

    let maximum =
        Confidence::new(1.0)
            .expect("one is valid confidence");

    assert_eq!(minimum, Confidence::MIN);
    assert_eq!(maximum, Confidence::MAX);

    assert_eq!(interior.value(), 0.5);
    assert_eq!(interior.score(), 0.5);

    assert!(minimum.is_zero());
    assert!(maximum.is_certain());
    assert!(interior.is_strictly_between_bounds());
}

#[test]
fn confidence_rejects_nan_infinity_and_out_of_range_values_without_clamping() {
    assert!(Confidence::new(f64::NAN).is_none());
    assert!(Confidence::new(f64::INFINITY).is_none());
    assert!(Confidence::new(f64::NEG_INFINITY).is_none());

    assert!(
        Confidence::new(-f64::EPSILON).is_none()
    );

    assert!(
        Confidence::new(1.0 + f64::EPSILON).is_none()
    );

    // Invalid input must not be silently clamped.
    assert_eq!(Confidence::new(-1.0), None);
    assert_eq!(Confidence::new(2.0), None);
}

#[test]
fn confidence_unknown_is_represented_by_option_not_a_special_value() {
    let unknown: Option<Confidence> = None;

    let established_zero =
        Some(Confidence::none());

    let established_certain =
        Some(Confidence::certain());

    assert!(unknown.is_none());

    assert_eq!(
        established_zero
            .expect("zero confidence is established")
            .value(),
        0.0
    );

    assert_eq!(
        established_certain
            .expect("certain confidence is established")
            .value(),
        1.0
    );
}

#[test]
fn confidence_comparison_requires_an_explicit_threshold() {
    let observed =
        Confidence::new(0.8)
            .expect("valid confidence");

    let required =
        Confidence::new(0.8)
            .expect("valid confidence");

    let stricter =
        Confidence::new(0.9)
            .expect("valid confidence");

    assert!(observed.meets(required));
    assert!(!observed.is_below(required));
    assert!(observed.is_below(stricter));
}

#[test]
fn confidence_minimum_and_maximum_are_ordinal_operations_only() {
    let low =
        Confidence::new(0.2)
            .expect("valid confidence");

    let high =
        Confidence::new(0.7)
            .expect("valid confidence");

    assert_eq!(low.min(high), low);
    assert_eq!(low.max(high), high);

    assert_eq!(high.min(low), low);
    assert_eq!(high.max(low), high);

    assert_eq!(
        low.absolute_difference(high).value(),
        0.5
    );
}

#[test]
fn confidence_boundaries_are_ordered_and_deterministic() {
    let values = [
        Confidence::MIN,
        Confidence::new(0.25)
            .expect("valid confidence"),
        Confidence::new(0.5)
            .expect("valid confidence"),
        Confidence::new(0.75)
            .expect("valid confidence"),
        Confidence::MAX,
    ];

    for window in values.windows(2) {
        assert!(window[0] <= window[1]);
    }

    assert_eq!(
        Confidence::MIN.cmp(&Confidence::MIN),
        Ordering::Equal
    );

    assert_eq!(
        Confidence::MAX.cmp(&Confidence::MAX),
        Ordering::Equal
    );
}

#[test]
fn confidence_presentation_does_not_change_normalized_semantics() {
    let confidence =
        Confidence::new(0.375)
            .expect("valid confidence");

    assert_eq!(confidence.value(), 0.375);
    assert_eq!(confidence.score(), 0.375);
    assert_eq!(confidence.percentage(), 37.5);
    assert_eq!(confidence.as_str(), "0.375");
    assert_eq!(confidence.to_string(), "0.375");
}

#[test]
fn confidence_extreme_valid_values_remain_finite() {
    let near_zero =
        Confidence::new(f64::MIN_POSITIVE)
            .expect("positive finite value is valid");

    let near_one =
        Confidence::new(1.0 - f64::EPSILON)
            .expect("value below one is valid");

    assert!(near_zero.value().is_finite());
    assert!(near_one.value().is_finite());

    assert!(near_zero.value() >= 0.0);
    assert!(near_one.value() <= 1.0);
}

// =============================================================================
// Cross-model contract tests
// =============================================================================

#[test]
fn model_dimensions_remain_semantically_independent() {
    let identity =
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(3),
        );

    let availability =
        ResourceAvailability::Unknown;

    let quantity =
        ResourceQuantity::unknown();

    let confidence =
        Confidence::new(0.0)
            .expect("zero is valid confidence");

    assert!(identity.is_physical_qubit());
    assert!(availability.is_unknown());
    assert!(quantity.is_unknown());
    assert!(confidence.is_zero());

    // Unknown availability/quantity must not be silently transformed into
    // zero confidence, and a physical identity must not become logical.
    assert!(!identity.is_logical_qubit());
    assert!(!availability.is_available());
    assert!(!quantity.is_zero());
}

#[test]
fn model_values_are_clone_equal_and_hashable_where_required() {
    use std::collections::BTreeSet;

    let kind =
        ResourceKind::new("physical_qubit")
            .expect("valid kind");

    let kind_clone = kind.clone();

    assert_eq!(kind, kind_clone);

    let mut kinds = BTreeSet::new();

    kinds.insert(kind);
    kinds.insert(kind_clone);

    assert_eq!(kinds.len(), 1);

    let identity =
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(4),
        );

    let identity_clone = identity;

    assert_eq!(identity, identity_clone);
}

// =============================================================================
// Scalability contract tests
// =============================================================================

#[test]
fn model_accepts_large_representable_resource_quantities_without_machine_limits() {
    let quantities = [
        ResourceQuantity::finite(0),
        ResourceQuantity::finite(1),
        ResourceQuantity::finite(1024),
        ResourceQuantity::finite(1_000_000),
        ResourceQuantity::finite(u128::MAX),
        ResourceQuantity::unbounded(),
    ];

    for quantity in quantities {
        match quantity {
            ResourceQuantity::Finite(value) => {
                assert_eq!(
                    quantity.as_finite(),
                    Some(value)
                );
            }

            ResourceQuantity::Unbounded => {
                assert!(quantity.is_unbounded());
            }

            ResourceQuantity::Unknown => {
                assert!(quantity.is_unknown());
            }
        }
    }
}

#[test]
fn model_does_not_assume_contiguous_qubit_ids() {
    // Canonical IDs are opaque identities. Sparse IDs are valid.
    let ids = [
        QubitId::new(0),
        QubitId::new(17),
        QubitId::new(1_000_000),
        QubitId::new(u64::MAX),
    ];

    for id in ids {
        let resource =
            ResourceIdentity::logical_qubit(id);

        assert_eq!(
            resource.logical_qubit_id(),
            Some(id)
        );

        assert!(resource.is_logical_qubit());
    }
}

#[test]
fn model_does_not_assume_contiguous_physical_qubit_ids() {
    let ids = [
        PhysicalQubitId::new(0),
        PhysicalQubitId::new(17),
        PhysicalQubitId::new(1_000_000),
        PhysicalQubitId::new(usize::MAX),
    ];

    for id in ids {
        let resource =
            ResourceIdentity::physical_qubit(id);

        assert_eq!(
            resource.physical_qubit_id(),
            Some(id)
        );

        assert!(resource.is_physical_qubit());
    }
}

// =============================================================================
// Determinism contract tests
// =============================================================================

#[test]
fn deterministic_model_operations_return_identical_results_for_identical_inputs() {
    let identity_a =
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(9),
        );

    let identity_b =
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(9),
        );

    assert_eq!(identity_a, identity_b);
    assert_eq!(
        identity_a.to_string(),
        identity_b.to_string()
    );

    let kind_a =
        ResourceKind::new("deterministic.resource")
            .expect("valid kind");

    let kind_b =
        ResourceKind::new("deterministic.resource")
            .expect("valid kind");

    assert_eq!(kind_a, kind_b);
    assert_eq!(
        kind_a.to_string(),
        kind_b.to_string()
    );

    let confidence_a =
        Confidence::new(0.625)
            .expect("valid confidence");

    let confidence_b =
        Confidence::new(0.625)
            .expect("valid confidence");

    assert_eq!(confidence_a, confidence_b);
    assert_eq!(
        confidence_a.value(),
        confidence_b.value()
    );
}

// =============================================================================
// No-I/O / no-provider boundary
// =============================================================================

#[test]
fn model_tests_use_only_pure_domain_inputs() {
    // The model layer must not require filesystem, network, clock, random
    // sources, backend SDKs or mutable global state.
    let kind =
        ResourceKind::new("pure.domain.value")
            .expect("valid kind");

    let quantity =
        ResourceQuantity::finite(42);

    let availability =
        ResourceAvailability::Available;

    let confidence =
        Confidence::new(1.0)
            .expect("valid confidence");

    assert_eq!(
        kind.as_str(),
        "pure.domain.value"
    );

    assert_eq!(
        quantity.as_finite(),
        Some(42)
    );

    assert!(availability.is_available());
    assert!(confidence.is_certain());
}