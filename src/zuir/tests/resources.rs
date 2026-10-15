//! ZUIR resource-intent conformance and regression tests.
//!
//! Path: `src/zuir/tests/resources.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: safe Rust only; unsafe code is forbidden.
//!
//! PURPOSE
//!
//! Exercise the public resource-intent API implemented in
//! `crate::zuir::ir::resources`.
//!
//! OWNS
//!
//! - Resource-key validation and open-world naming.
//! - Exact decimal parsing, normalization, and comparison.
//! - Quantity and unit-comparison behavior.
//! - Assertion relations and intent categories.
//! - Capability and topology intent validation.
//! - Aggregate intent behavior and obvious contradiction detection.
//! - Determinism and dynamically growing collections.
//!
//! DOES NOT OWN
//!
//! - Resource-intent production implementation.
//! - Parsing Zamani source or interpreting grammar rules.
//! - Target discovery, resource allocation, or feasibility analysis.
//! - Unit conversion, symbolic expression evaluation, or resource costing.
//! - Whole-program verification or backend-specific tests.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/resources.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/config.rs`
//! - `grammar/spec/resources.md`
//! - `grammar/resources/README.md`
//! - `grammar/resources/constraints.g4`
//! - `grammar/resources/budgets.g4`
//! - `grammar/resources/capabilities.g4`
//!
//! INTEGRATION
//!
//! Import the canonical resource types directly. Do not create test-only
//! resource types or infer resource semantics from source text in this file.
//! Register this file with the repository's actual test-module hierarchy.
//!
//! SCALABILITY
//!
//! Test counts are regression workloads, not language or implementation
//! limits. The model must grow subject to available process resources.
//! Literal infinity cannot be promised by a finite implementation.
//!
//! VERIFICATION BOUNDARY
//!
//! Passing these tests does not establish target feasibility, unit conversion,
//! extension authorization, physical allocation, or full-program correctness.
//!
//! FREEZE CRITERIA
//!
//! - Tests use the public API in `ir/resources.rs`.
//! - No fixed hardware capacity is encoded.
//! - Unknown but valid resource names remain representable.
//! - Exact decimal behavior does not rely on floating-point arithmetic.
//! - Mandatory intent remains distinct from preference and hint intent.
//! - Local validation is not confused with target feasibility.
//! - The file is registered, formatted, compiled, and executed in the
//!   repository's real Cargo configuration.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::zuir::ir::resources::{
    CapabilityIntent, DecimalMagnitude, ResourceAssertion, ResourceIntent,
    ResourceIntentKind, ResourceKey, ResourceQuantity, ResourceRelation,
    TopologyIntent, TopologyRelation,
};

fn key(value: &str) -> ResourceKey {
    ResourceKey::new(value).expect("test resource key should be valid")
}

fn quantity(magnitude: &str, unit: &str) -> ResourceQuantity {
    ResourceQuantity::parse(magnitude, unit)
        .expect("test resource quantity should be valid")
}

fn assertion(
    dimension: &str,
    relation: ResourceRelation,
    magnitude: &str,
    unit: &str,
    kind: ResourceIntentKind,
) -> ResourceAssertion {
    ResourceAssertion::new(
        key(dimension),
        relation,
        quantity(magnitude, unit),
        kind,
    )
    .expect("test resource assertion should be valid")
}

#[test]
fn accepts_open_world_resource_names() {
    let names = [
        "memory.capacity",
        "quantum.logical_qubits",
        "processing.units",
        "network.bandwidth",
        "execution.latency",
        "future.compute.resource",
        "future_domain.new_resource",
    ];

    for name in names {
        let resource = key(name);
        assert_eq!(resource.as_str(), name);
        assert!(resource.validate().is_ok());
    }
}

#[test]
fn rejects_empty_or_malformed_resource_names() {
    for invalid in [
        "",
        ".memory",
        "memory.",
        "memory..capacity",
        "-memory.capacity",
        "memory invalid.capacity",
        "memory/physical",
        "mémoire.capacity",
    ] {
        assert!(
            ResourceKey::new(invalid).is_err(),
            "resource key should be rejected: {invalid:?}"
        );
    }
}

#[test]
fn resource_keys_have_value_based_ordering() {
    let alpha = key("future.alpha");
    let beta = key("future.beta");

    assert!(alpha < beta);
    assert_eq!(key("future.alpha"), alpha);
}

#[test]
fn decimal_magnitudes_are_normalized() {
    let cases = [
        ("00012.3400", "12.34"),
        ("00012", "12"),
        ("000.000", "0"),
        ("0.5000", "0.5"),
        ("100.000", "100"),
        ("0000000000000000007.25000", "7.25"),
    ];

    for (input, expected) in cases {
        assert_eq!(
            DecimalMagnitude::parse(input)
                .expect("valid decimal")
                .as_str(),
            expected,
            "incorrect normalization for {input}"
        );
    }
}

#[test]
fn decimal_magnitudes_reject_invalid_numeric_forms() {
    for invalid in [
        "",
        "-1",
        "+1",
        ".5",
        "1.",
        "1.2.3",
        "1e9",
        "NaN",
        "inf",
        "Infinity",
        "1_000",
        "1 000",
    ] {
        assert!(
            DecimalMagnitude::parse(invalid).is_err(),
            "invalid decimal should be rejected: {invalid:?}"
        );
    }
}

#[test]
fn decimal_zero_is_canonical() {
    let zero = DecimalMagnitude::zero();

    assert_eq!(zero.as_str(), "0");
    assert!(zero.is_zero());
    assert_eq!(
        DecimalMagnitude::parse("000.000")
            .expect("zero")
            .as_str(),
        zero.as_str()
    );
}

#[test]
fn decimal_comparison_is_exact_beyond_machine_integer_widths() {
    let smaller = DecimalMagnitude::parse(
        "99999999999999999999999999999999999999999999999999",
    )
    .expect("large decimal");

    let larger = DecimalMagnitude::parse(
        "100000000000000000000000000000000000000000000000000",
    )
    .expect("larger decimal");

    assert_eq!(
        smaller.compare(&larger),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        larger.compare(&smaller),
        std::cmp::Ordering::Greater
    );
}

#[test]
fn decimal_comparison_preserves_fractional_precision() {
    let tenth = DecimalMagnitude::parse("0.1").expect("decimal");
    let slightly_larger =
        DecimalMagnitude::parse("0.1000000000000000000001")
            .expect("decimal");
    let equal_normalized =
        DecimalMagnitude::parse("000.1000").expect("decimal");

    assert_eq!(
        slightly_larger.compare(&tenth),
        std::cmp::Ordering::Greater
    );
    assert_eq!(
        tenth.compare(&equal_normalized),
        std::cmp::Ordering::Equal
    );
}

#[test]
fn quantities_preserve_magnitude_and_unit() {
    let value = quantity("0001024.000", "byte");

    assert_eq!(value.magnitude().as_str(), "1024");
    assert_eq!(value.unit().as_str(), "byte");
    assert!(value.validate().is_ok());
}

#[test]
fn quantities_do_not_silently_convert_units() {
    let required = assertion(
        "memory.capacity",
        ResourceRelation::AtLeast,
        "1024",
        "byte",
        ResourceIntentKind::Requirement,
    );

    assert!(
        required
            .evaluate_same_unit(&quantity("2048", "byte"))
            .expect("same-unit comparison")
    );

    assert!(
        required
            .evaluate_same_unit(&quantity("2", "kibibyte"))
            .is_err(),
        "different units require an explicit conversion contract"
    );
}

#[test]
fn resource_relations_evaluate_exactly() {
    let cases = [
        (ResourceRelation::AtLeast, "10", true),
        (ResourceRelation::AtMost, "10", true),
        (ResourceRelation::Exactly, "10", true),
        (ResourceRelation::GreaterThan, "10", false),
        (ResourceRelation::LessThan, "10", false),
        (ResourceRelation::AtLeast, "10.01", true),
        (ResourceRelation::AtMost, "10.01", false),
        (ResourceRelation::GreaterThan, "10.01", true),
        (ResourceRelation::LessThan, "10.01", false),
        (ResourceRelation::AtLeast, "9.99", false),
        (ResourceRelation::AtMost, "9.99", true),
        (ResourceRelation::GreaterThan, "9.99", false),
        (ResourceRelation::LessThan, "9.99", true),
    ];

    let requirement = assertion(
        "execution.quantity",
        ResourceRelation::Exactly,
        "10",
        "unit",
        ResourceIntentKind::Requirement,
    );

    for (relation, actual, expected) in cases {
        let item = assertion(
            "execution.quantity",
            relation,
            "10",
            "unit",
            ResourceIntentKind::Requirement,
        );

        assert_eq!(
            item.evaluate_same_unit(&quantity(actual, "unit"))
                .expect("same-unit evaluation"),
            expected,
            "unexpected result for {relation:?} and actual {actual}"
        );
    }

    assert_eq!(
        requirement.relation(),
        ResourceRelation::Exactly
    );
}

#[test]
fn intent_categories_remain_distinct() {
    let mut intent = ResourceIntent::new();

    for kind in [
        ResourceIntentKind::Requirement,
        ResourceIntentKind::Constraint,
        ResourceIntentKind::Budget,
        ResourceIntentKind::Preference,
        ResourceIntentKind::Hint,
    ] {
        intent
            .add_assertion(assertion(
                &format!("future.category.{}", format!("{kind:?}").to_lowercase()),
                ResourceRelation::AtLeast,
                "1",
                "unit",
                kind,
            ))
            .expect("valid intent should be accepted");
    }

    assert_eq!(intent.assertions().len(), 5);
    assert_eq!(intent.mandatory_assertions().count(), 2);
    assert!(intent.validate().is_ok());
}

#[test]
fn preferences_and_hints_do_not_become_mandatory_assertions() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "processing.units",
            ResourceRelation::AtLeast,
            "4",
            "unit",
            ResourceIntentKind::Requirement,
        ))
        .expect("requirement");

    intent
        .add_assertion(assertion(
            "processing.units",
            ResourceRelation::AtLeast,
            "16",
            "unit",
            ResourceIntentKind::Preference,
        ))
        .expect("preference");

    intent
        .add_assertion(assertion(
            "energy.consumption",
            ResourceRelation::AtMost,
            "20",
            "joule",
            ResourceIntentKind::Hint,
        ))
        .expect("hint");

    assert_eq!(intent.assertions().len(), 3);
    assert_eq!(intent.mandatory_assertions().count(), 1);
}

#[test]
fn capability_intent_supports_future_namespaces() {
    let capability_name = key("future_domain.specialized_execution");
    let capability = CapabilityIntent::new(
        capability_name.clone(),
        ResourceIntentKind::Requirement,
    )
    .expect("valid capability");

    assert_eq!(capability.name(), &capability_name);
    assert_eq!(capability.kind(), ResourceIntentKind::Requirement);
    assert!(capability.validate().is_ok());
}

#[test]
fn capability_budget_category_is_rejected() {
    let result = CapabilityIntent::new(
        key("quantum.measurement"),
        ResourceIntentKind::Budget,
    );

    assert!(result.is_err());
}

#[test]
fn required_capabilities_include_requirements_and_constraints() {
    let mut intent = ResourceIntent::new();

    for (name, kind) in [
        (
            "quantum.measurement",
            ResourceIntentKind::Requirement,
        ),
        (
            "classical.parallel_execution",
            ResourceIntentKind::Constraint,
        ),
        (
            "tensor.acceleration",
            ResourceIntentKind::Preference,
        ),
        (
            "future.optional_feature",
            ResourceIntentKind::Hint,
        ),
    ] {
        intent
            .add_capability(
                CapabilityIntent::new(key(name), kind)
                    .expect("valid capability"),
            )
            .expect("capability should be added");
    }

    let required: Vec<&str> = intent
        .required_capabilities()
        .map(ResourceKey::as_str)
        .collect();

    assert_eq!(
        required,
        vec![
            "quantum.measurement",
            "classical.parallel_execution"
        ]
    );
}

#[test]
fn topology_intent_uses_logical_names() {
    let topology = TopologyIntent::new(
        TopologyRelation::Connected,
        key("pipeline.stage_a"),
        key("pipeline.stage_b"),
        ResourceIntentKind::Constraint,
    )
    .expect("valid topology");

    assert_eq!(topology.relation(), TopologyRelation::Connected);
    assert_eq!(topology.left().as_str(), "pipeline.stage_a");
    assert_eq!(topology.right().as_str(), "pipeline.stage_b");
    assert_eq!(topology.kind(), ResourceIntentKind::Constraint);
    assert!(topology.validate().is_ok());
}

#[test]
fn topology_rejects_separating_a_group_from_itself() {
    assert!(
        TopologyIntent::new(
            TopologyRelation::Separate,
            key("logical.worker_group"),
            key("logical.worker_group"),
            ResourceIntentKind::Constraint,
        )
        .is_err()
    );
}

#[test]
fn topology_extensions_require_a_named_relation() {
    let relation = key("future_topology.low_contention");

    let topology = TopologyIntent::extension(
        relation.clone(),
        key("logical.group_a"),
        key("logical.group_b"),
        ResourceIntentKind::Hint,
    )
    .expect("valid extension relation");

    assert_eq!(
        topology.relation(),
        TopologyRelation::Extension
    );
    assert_eq!(
        topology.extension_relation(),
        Some(&relation)
    );
    assert!(topology.validate().is_ok());
}

#[test]
fn empty_resource_intent_is_valid() {
    let intent = ResourceIntent::new();

    assert!(intent.is_empty());
    assert!(intent.validate().is_ok());
    assert!(intent.validate_obvious_contradictions().is_ok());
    assert_eq!(intent.assertions().len(), 0);
    assert_eq!(intent.capabilities().len(), 0);
    assert_eq!(intent.topology().len(), 0);
    assert!(intent.metadata().is_empty());
}

#[test]
fn aggregate_intent_validates_all_local_components() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "quantum.logical_qubits",
            ResourceRelation::AtLeast,
            "128",
            "qubit",
            ResourceIntentKind::Requirement,
        ))
        .expect("resource assertion");

    intent
        .add_capability(
            CapabilityIntent::new(
                key("quantum.measurement"),
                ResourceIntentKind::Requirement,
            )
            .expect("capability"),
        )
        .expect("add capability");

    intent
        .add_topology(
            TopologyIntent::new(
                TopologyRelation::Connected,
                key("logical.control"),
                key("logical.target"),
                ResourceIntentKind::Constraint,
            )
            .expect("topology"),
        )
        .expect("add topology");

    intent
        .insert_metadata(
            key("provenance.origin"),
            "resource-conformance-test",
        )
        .expect("insert metadata");

    assert!(!intent.is_empty());
    assert!(intent.validate().is_ok());
    assert_eq!(intent.assertions().len(), 1);
    assert_eq!(intent.capabilities().len(), 1);
    assert_eq!(intent.topology().len(), 1);
    assert_eq!(
        intent.metadata().get(&key("provenance.origin")),
        Some(&"resource-conformance-test".to_owned())
    );
}

#[test]
fn metadata_replacement_is_explicit_and_deterministic() {
    let mut intent = ResourceIntent::new();
    let metadata_key = key("analysis.annotation");

    assert_eq!(
        intent
            .insert_metadata(metadata_key.clone(), "first")
            .expect("first insertion"),
        None
    );

    assert_eq!(
        intent
            .insert_metadata(metadata_key.clone(), "second")
            .expect("replacement"),
        Some("first".to_owned())
    );

    assert_eq!(
        intent.metadata().get(&metadata_key),
        Some(&"second".to_owned())
    );
    assert_eq!(intent.metadata().len(), 1);
}

#[test]
fn assertions_for_a_dimension_preserve_declaration_order() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtLeast,
            "10",
            "byte",
            ResourceIntentKind::Requirement,
        ))
        .expect("lower bound");

    intent
        .add_assertion(assertion(
            "processing.units",
            ResourceRelation::AtLeast,
            "2",
            "unit",
            ResourceIntentKind::Requirement,
        ))
        .expect("unrelated dimension");

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtMost,
            "100",
            "byte",
            ResourceIntentKind::Constraint,
        ))
        .expect("upper bound");

    let memory_assertions: Vec<&ResourceRelation> = intent
        .assertions_for(&key("memory.capacity"))
        .map(ResourceAssertion::relation)
        .collect();

    assert_eq!(
        memory_assertions,
        vec![
            &ResourceRelation::AtLeast,
            &ResourceRelation::AtMost
        ]
    );
}

#[test]
fn obvious_mandatory_contradictions_are_detected() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtLeast,
            "100",
            "byte",
            ResourceIntentKind::Requirement,
        ))
        .expect("lower bound");

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtMost,
            "50",
            "byte",
            ResourceIntentKind::Constraint,
        ))
        .expect("upper bound");

    assert!(intent.validate().is_ok());
    assert!(intent.validate_obvious_contradictions().is_err());
}

#[test]
fn equal_mandatory_lower_and_upper_bounds_are_compatible() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "processing.units",
            ResourceRelation::AtLeast,
            "8",
            "unit",
            ResourceIntentKind::Requirement,
        ))
        .expect("lower bound");

    intent
        .add_assertion(assertion(
            "processing.units",
            ResourceRelation::AtMost,
            "8",
            "unit",
            ResourceIntentKind::Constraint,
        ))
        .expect("upper bound");

    assert!(intent.validate_obvious_contradictions().is_ok());
}

#[test]
fn assertions_with_different_units_are_not_compared_implicitly() {
    let mut intent = ResourceIntent::new();

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtLeast,
            "100",
            "byte",
            ResourceIntentKind::Requirement,
        ))
        .expect("byte requirement");

    intent
        .add_assertion(assertion(
            "memory.capacity",
            ResourceRelation::AtMost,
            "1",
            "kibibyte",
            ResourceIntentKind::Constraint,
        ))
        .expect("kibibyte constraint");

    // Cross-unit reasoning requires a separate, explicit conversion contract.
    assert!(intent.validate_obvious_contradictions().is_ok());
}

#[test]
fn duplicate_assertions_are_preserved_instead_of_silently_collapsed() {
    let mut intent = ResourceIntent::new();
    let requirement = assertion(
        "quantum.logical_qubits",
        ResourceRelation::AtLeast,
        "4",
        "qubit",
        ResourceIntentKind::Requirement,
    );

    intent
        .add_assertion(requirement.clone())
        .expect("first assertion");
    intent
        .add_assertion(requirement.clone())
        .expect("second assertion");

    assert_eq!(intent.assertions().len(), 2);
    assert_eq!(intent.assertions()[0], requirement);
    assert_eq!(intent.assertions()[1], requirement);
}

#[test]
fn dynamically_grows_across_many_distinct_resource_dimensions() {
    // Regression workload only. This is not a maximum supported dimension
    // count, machine capacity, or language-level restriction.
    const TEST_DIMENSION_COUNT: usize = 512;

    let mut intent = ResourceIntent::new();

    for index in 0..TEST_DIMENSION_COUNT {
        let dimension = format!("future_domain.resource_{index}");

        intent
            .add_assertion(assertion(
                &dimension,
                ResourceRelation::AtLeast,
                "1",
                "unit",
                ResourceIntentKind::Requirement,
            ))
            .expect("valid dynamic resource dimension");
    }

    assert_eq!(intent.assertions().len(), TEST_DIMENSION_COUNT);
    assert_eq!(
        intent.mandatory_assertions().count(),
        TEST_DIMENSION_COUNT
    );
    assert!(intent.validate().is_ok());
}

#[test]
fn dynamically_grows_across_many_capabilities_and_topology_edges() {
    // A bounded CI workload catches accidental fixed-capacity storage while
    // remaining practical for routine test runs.
    const TEST_ENTRY_COUNT: usize = 256;

    let mut intent = ResourceIntent::new();

    for index in 0..TEST_ENTRY_COUNT {
        intent
            .add_capability(
                CapabilityIntent::new(
                    key(&format!("future_domain.capability_{index}")),
                    ResourceIntentKind::Requirement,
                )
                .expect("valid dynamic capability"),
            )
            .expect("add capability");

        intent
            .add_topology(
                TopologyIntent::new(
                    TopologyRelation::Connected,
                    key(&format!("logical.group_{index}")),
                    key(&format!("logical.group_{}", index + 1)),
                    ResourceIntentKind::Constraint,
                )
                .expect("valid dynamic topology"),
            )
            .expect("add topology");
    }

    assert_eq!(intent.capabilities().len(), TEST_ENTRY_COUNT);
    assert_eq!(intent.topology().len(), TEST_ENTRY_COUNT);
    assert_eq!(
        intent.required_capabilities().count(),
        TEST_ENTRY_COUNT
    );
    assert!(intent.validate().is_ok());
}

#[test]
fn resource_model_does_not_require_a_finite_known_resource_catalogue() {
    let mut intent = ResourceIntent::new();

    for dimension in [
        "classical.processing.units",
        "quantum.logical_qubits",
        "hdl.signal_width",
        "tensor.shape_dimension",
        "distributed.node_capacity",
        "network.bandwidth",
        "crypto.parallel_operations",
        "future_domain.uninvented_resource",
    ] {
        intent
            .add_assertion(assertion(
                dimension,
                ResourceRelation::AtLeast,
                "1",
                "unit",
                ResourceIntentKind::Requirement,
            ))
            .expect("open-world resource name");
    }

    assert_eq!(intent.assertions().len(), 8);
    assert!(intent.validate().is_ok());
}