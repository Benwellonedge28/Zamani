//! Zamani Quantum IR — Analog Model Integration Tests
//!
//! Path:
//!     src/quantum/ir/tests/analog.rs
//!
//! # Purpose
//!
//! Cross-module integration tests for the canonical analog quantum IR.
//!
//! These tests verify the public semantic contract of:
//!
//! - `quantum::ir::model::analog`;
//! - `quantum::ir::qubit::QubitId`;
//! - `quantum::ir::parameter::Parameter`;
//!
//! without coupling the tests to:
//!
//! - a particular QPU;
//! - a particular vendor;
//! - a particular topology;
//! - a particular simulator;
//! - a particular compiler pass;
//! - a particular resource limit;
//! - a particular physical qubit count.
//!
//! # Architectural contract
//!
//! The analog model represents semantic quantum computation:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! quantum::ir::model::analog
//!      │
//!      ├── resources
//!      ├── positions
//!      ├── controls
//!      ├── Hamiltonian terms
//!      ├── evolution semantics
//!      └── observables
//!      │
//!      ▼
//! target capabilities
//!      │
//!      ▼
//! mapping / scheduling / lowering
//!      │
//!      ▼
//! hardware / backend
//! ```
//!
//! These tests therefore verify that the semantic layer does not accidentally
//! become a hardware-specific implementation.
//!
//! # Scaling principle
//!
//! The tests intentionally use generated collections rather than a semantic
//! maximum. Test sizes are test workloads only; they are NOT architectural
//! limits.
//!
//! A test such as:
//!
//! ```text
//! 10_000 resources
//! ```
//!
//! means only that the test suite exercises a large finite workload. It does
//! NOT mean that Zamani supports only 10,000 resources.
//!
//! # Canonical qubit identity
//!
//! All qubit-aware tests use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! No duplicate qubit identity type is introduced here.
//!
//! # Safety
//!
//! This test module forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! The parent test module should register this file with:
//!
//! ```rust
//! #[cfg(test)]
//! mod analog;
//! ```
//!
//! The implementation under test remains:
//!
//! ```text
//! quantum::ir::model::analog
//! ```
//!
//! This file must not modify the production implementation merely to make a
//! test compile. If a public semantic contract changes, the implementation
//! and its public API must be changed deliberately and versioned appropriately.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;

use crate::quantum::ir::model::analog::{
    AnalogControl,
    AnalogError,
    AnalogHamiltonian,
    AnalogResource,
    ControlProfile,
    HamiltonianTerm,
    Interpolation,
    OperatorKind,
    Position,
    StandardOperator,
    TargetSet,
    TimeSample,
    ANALOG_MODEL_ID,
    ANALOG_MODEL_VERSION,
};
use crate::quantum::ir::parameter::Parameter;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Test helpers
// =============================================================================

/// Constructs a finite constant parameter.
///
/// Keeping construction behind one helper makes the test suite independent of
/// whether callers choose the checked constructor or the enum constructor in
/// production code.
fn constant(value: f64) -> Parameter {
    Parameter::constant(value).expect("test fixture must use a finite parameter")
}

/// Constructs a canonical logical qubit identifier.
fn qubit(index: u64) -> QubitId {
    QubitId::new(index)
}

/// Creates a deterministic one-dimensional resource set.
///
/// The IDs are generated through `quantum::ir::qubit::QubitId`, ensuring that
/// the test exercises the same identity domain as production code.
fn resources(count: usize) -> Vec<AnalogResource> {
    (0..count)
        .map(|index| AnalogResource::new(qubit(index as u64)))
        .collect()
}

/// Creates a simple one-dimensional time sample.
fn sample(time: f64, value: f64) -> TimeSample {
    TimeSample::new(time, constant(value)).expect("test sample must be valid")
}

/// Creates a minimal valid control profile.
fn profile() -> ControlProfile {
    ControlProfile::new(
        vec![sample(0.0, 0.0), sample(1.0, 1.0)],
        Interpolation::PiecewiseLinear,
    )
    .expect("test control profile must be valid")
}

/// Creates a minimal valid Pauli-Z Hamiltonian term.
fn z_term(target: QubitId, coefficient: f64) -> HamiltonianTerm {
    HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        vec![target],
        constant(coefficient),
    )
    .expect("test Hamiltonian term must be valid")
}

// =============================================================================
// Schema contract
// =============================================================================

#[test]
fn analog_schema_identity_is_stable() {
    assert_eq!(
        ANALOG_MODEL_ID,
        "zamani.quantum.ir.model.analog",
        "the analog model identifier is a schema-level compatibility contract"
    );

    assert!(
        ANALOG_MODEL_VERSION >= 1,
        "the analog model must have a non-zero semantic version"
    );
}

// =============================================================================
// Canonical qubit identity
// =============================================================================

#[test]
fn analog_resources_use_canonical_qubit_identity() {
    let id = qubit(17);
    let resource = AnalogResource::new(id);

    assert_eq!(resource.id(), id);
    assert_eq!(resource.id(), QubitId::new(17));
}

#[test]
fn analog_resource_identity_is_not_tied_to_position() {
    let id = qubit(9);

    let without_position = AnalogResource::new(id);

    let position = Position::one_dimensional(3.5).expect("position must be valid");
    let with_position = AnalogResource::with_position(id, position);

    assert_eq!(without_position.id(), with_position.id());

    assert!(without_position.position().is_none());
    assert!(with_position.position().is_some());
}

// =============================================================================
// Position semantics
// =============================================================================

#[test]
fn position_rejects_empty_coordinate_vectors() {
    let result = Position::new(Vec::new());

    assert!(matches!(result, Err(AnalogError::EmptyPosition)));
}

#[test]
fn position_rejects_non_finite_coordinates() {
    let nan = Position::new(vec![f64::NAN]);

    assert!(matches!(
        nan,
        Err(AnalogError::NonFiniteValue {
            field: "position.coordinate",
            ..
        })
    ));

    let positive_infinity = Position::new(vec![f64::INFINITY]);

    assert!(matches!(
        positive_infinity,
        Err(AnalogError::NonFiniteValue {
            field: "position.coordinate",
            ..
        })
    ));

    let negative_infinity = Position::new(vec![f64::NEG_INFINITY]);

    assert!(matches!(
        negative_infinity,
        Err(AnalogError::NonFiniteValue {
            field: "position.coordinate",
            ..
        })
    ));
}

#[test]
fn position_supports_arbitrary_dimensions() {
    let coordinates: Vec<f64> = (0..32).map(|value| value as f64).collect();

    let position =
        Position::new(coordinates.clone()).expect("arbitrary finite dimensional positions work");

    assert_eq!(position.dimension(), coordinates.len());
    assert_eq!(position.coordinates(), coordinates.as_slice());
}

#[test]
fn position_convenience_constructors_preserve_dimension() {
    let one = Position::one_dimensional(1.0).expect("1D position");
    let two = Position::two_dimensional(1.0, 2.0).expect("2D position");
    let three = Position::three_dimensional(1.0, 2.0, 3.0).expect("3D position");

    assert_eq!(one.dimension(), 1);
    assert_eq!(two.dimension(), 2);
    assert_eq!(three.dimension(), 3);
}

#[test]
fn position_squared_distance_is_semantically_correct() {
    let left =
        Position::two_dimensional(1.0, 2.0).expect("left position must be valid");

    let right =
        Position::two_dimensional(4.0, 6.0).expect("right position must be valid");

    let distance = left
        .squared_distance(&right)
        .expect("compatible finite positions must produce a finite distance");

    assert_eq!(distance, 25.0);
}

#[test]
fn position_squared_distance_rejects_dimension_mismatch() {
    let left =
        Position::one_dimensional(1.0).expect("left position must be valid");

    let right =
        Position::two_dimensional(1.0, 2.0).expect("right position must be valid");

    let result = left.squared_distance(&right);

    assert!(matches!(
        result,
        Err(AnalogError::DimensionMismatch {
            expected: 1,
            actual: 2
        })
    ));
}

#[test]
fn position_access_is_bounds_safe() {
    let position =
        Position::three_dimensional(10.0, 20.0, 30.0).expect("position must be valid");

    assert_eq!(position.get(0), Some(10.0));
    assert_eq!(position.get(1), Some(20.0));
    assert_eq!(position.get(2), Some(30.0));
    assert_eq!(position.get(3), None);
}

// =============================================================================
// Resource semantics
// =============================================================================

#[test]
fn resource_position_can_be_added_without_changing_identity() {
    let id = qubit(42);
    let mut resource = AnalogResource::new(id);

    assert_eq!(resource.id(), id);
    assert!(resource.position().is_none());

    let position =
        Position::two_dimensional(4.0, 8.0).expect("position must be valid");

    resource.set_position(position);

    assert_eq!(resource.id(), id);
    assert_eq!(
        resource
            .position()
            .expect("resource must contain a position")
            .coordinates(),
        &[4.0, 8.0]
    );
}

#[test]
fn resource_labels_are_metadata_not_identity() {
    let id = qubit(3);
    let mut resource = AnalogResource::new(id);

    resource
        .set_label("atom-3")
        .expect("non-empty labels are valid");

    assert_eq!(resource.id(), id);
    assert_eq!(resource.label(), Some("atom-3"));
}

#[test]
fn resource_rejects_empty_labels() {
    let mut resource = AnalogResource::new(qubit(1));

    let result = resource.set_label("");

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "resource.label"
        })
    ));
}

// =============================================================================
// Time sample semantics
// =============================================================================

#[test]
fn time_sample_accepts_zero_and_positive_time() {
    let zero = TimeSample::new(0.0, constant(1.0))
        .expect("zero is a valid origin time");

    let positive = TimeSample::new(12.5, constant(2.0))
        .expect("positive time is valid");

    assert_eq!(zero.time(), 0.0);
    assert_eq!(positive.time(), 12.5);
}

#[test]
fn time_sample_rejects_negative_time() {
    let result = TimeSample::new(-0.0001, constant(1.0));

    assert!(matches!(
        result,
        Err(AnalogError::ValueOutOfRange {
            field: "time_sample.time",
            ..
        })
    ));
}

#[test]
fn time_sample_rejects_non_finite_time() {
    let nan = TimeSample::new(f64::NAN, constant(1.0));

    assert!(matches!(
        nan,
        Err(AnalogError::NonFiniteValue {
            field: "time_sample.time",
            ..
        })
    ));

    let infinity = TimeSample::new(f64::INFINITY, constant(1.0));

    assert!(matches!(
        infinity,
        Err(AnalogError::NonFiniteValue {
            field: "time_sample.time",
            ..
        })
    ));
}

#[test]
fn symbolic_parameters_are_preserved_in_time_samples() {
    let parameter = Parameter::Symbol("omega".to_owned());

    let sample =
        TimeSample::new(2.0, parameter).expect("symbolic parameters are valid");

    assert!(sample.value().is_symbolic());
}

// =============================================================================
// Control profile semantics
// =============================================================================

#[test]
fn control_profile_rejects_empty_sample_sequences() {
    let result =
        ControlProfile::new(Vec::new(), Interpolation::PiecewiseConstant);

    assert!(matches!(
        result,
        Err(AnalogError::EmptyField {
            field: "control_profile.samples"
        })
    ));
}

#[test]
fn control_profile_accepts_non_decreasing_time() {
    let result = ControlProfile::new(
        vec![
            sample(0.0, 0.0),
            sample(1.0, 1.0),
            sample(1.0, 2.0),
            sample(3.0, 3.0),
        ],
        Interpolation::PiecewiseConstant,
    );

    assert!(result.is_ok());
}

#[test]
fn control_profile_rejects_decreasing_time() {
    let result = ControlProfile::new(
        vec![sample(2.0, 1.0), sample(1.0, 2.0)],
        Interpolation::PiecewiseLinear,
    );

    assert!(matches!(
        result,
        Err(AnalogError::NonMonotonicTime {
            previous: 2.0,
            current: 1.0
        })
    ));
}

#[test]
fn control_profile_preserves_interpolation_semantics() {
    for interpolation in [
        Interpolation::PiecewiseConstant,
        Interpolation::PiecewiseLinear,
        Interpolation::TargetDefined,
    ] {
        let profile = ControlProfile::new(
            vec![sample(0.0, 0.0), sample(5.0, 1.0)],
            interpolation,
        )
        .expect("valid interpolation mode must be accepted");

        assert_eq!(profile.interpolation(), interpolation);
        assert_eq!(profile.end_time(), Some(5.0));
        assert_eq!(profile.samples().len(), 2);
    }
}

#[test]
fn control_profile_detects_symbolic_values_without_evaluating_them() {
    let symbolic = TimeSample::new(
        0.0,
        Parameter::Symbol("omega_max".to_owned()),
    )
    .expect("symbolic sample must be valid");

    let profile = ControlProfile::new(
        vec![symbolic],
        Interpolation::TargetDefined,
    )
    .expect("symbolic profile must be valid");

    assert!(profile.is_symbolic());
}

// =============================================================================
// Target-set semantics
// =============================================================================

#[test]
fn global_target_set_is_explicitly_global() {
    let targets = TargetSet::All;

    assert!(targets.is_global());
    assert!(targets.explicit_targets().is_none());
}

#[test]
fn explicit_target_set_accepts_arbitrary_cardinality() {
    let target_ids: Vec<QubitId> = (0..128).map(qubit).collect();

    let targets =
        TargetSet::explicit(target_ids.clone()).expect("unique explicit targets are valid");

    assert!(!targets.is_global());

    assert_eq!(
        targets
            .explicit_targets()
            .expect("explicit target set must expose its targets"),
        target_ids.as_slice()
    );
}

#[test]
fn explicit_target_set_rejects_empty_targets() {
    let result = TargetSet::explicit(Vec::new());

    assert!(matches!(result, Err(AnalogError::InvalidTargetSet)));
}

#[test]
fn explicit_target_set_rejects_duplicate_qubits() {
    let result =
        TargetSet::explicit(vec![qubit(1), qubit(2), qubit(1)]);

    assert!(matches!(
        result,
        Err(AnalogError::DuplicateOperatorTarget {
            resource
        }) if resource == qubit(1)
    ));
}

// =============================================================================
// Analog control semantics
// =============================================================================

#[test]
fn analog_control_requires_a_name() {
    let result =
        AnalogControl::new("", TargetSet::All, profile());

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "analog_control.name"
        })
    ));
}

#[test]
fn analog_control_preserves_global_semantics() {
    let control =
        AnalogControl::new("global_detuning", TargetSet::All, profile())
            .expect("valid analog control");

    assert_eq!(control.name(), "global_detuning");
    assert!(control.targets().is_global());
    assert_eq!(control.profile().end_time(), Some(1.0));
}

#[test]
fn analog_control_preserves_local_target_semantics() {
    let targets =
        TargetSet::explicit(vec![qubit(2), qubit(7)])
            .expect("unique targets");

    let control =
        AnalogControl::new("local_drive", targets, profile())
            .expect("valid analog control");

    assert_eq!(
        control
            .targets()
            .explicit_targets()
            .expect("control must preserve explicit targets"),
        &[qubit(2), qubit(7)]
    );
}

#[test]
fn analog_control_supports_explicit_units() {
    let control =
        AnalogControl::new("detuning", TargetSet::All, profile())
            .expect("valid control")
            .with_units("Hz")
            .expect("non-empty units");

    assert_eq!(control.units(), Some("Hz"));
}

#[test]
fn analog_control_rejects_empty_units() {
    let result =
        AnalogControl::new("detuning", TargetSet::All, profile())
            .expect("valid control")
            .with_units("");

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "analog_control.units"
        })
    ));
}

// =============================================================================
// Operator semantics
// =============================================================================

#[test]
fn standard_operator_families_are_distinct() {
    let operators = [
        StandardOperator::Identity,
        StandardOperator::PauliX,
        StandardOperator::PauliY,
        StandardOperator::PauliZ,
        StandardOperator::Number,
        StandardOperator::Raising,
        StandardOperator::Lowering,
        StandardOperator::Spin,
        StandardOperator::Creation,
        StandardOperator::Annihilation,
    ];

    let unique: BTreeSet<StandardOperator> =
        operators.into_iter().collect();

    assert_eq!(unique.len(), 10);
}

#[test]
fn custom_operator_namespace_is_extensible() {
    let operator =
        OperatorKind::custom("zamani.future", "custom_operator")
            .expect("non-empty custom operator identifiers are valid");

    assert_eq!(
        operator,
        OperatorKind::Custom {
            namespace: "zamani.future".to_owned(),
            name: "custom_operator".to_owned(),
        }
    );
}

#[test]
fn custom_operator_rejects_empty_namespace() {
    let result = OperatorKind::custom("", "operator");

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "operator.namespace"
        })
    ));
}

#[test]
fn custom_operator_rejects_empty_name() {
    let result = OperatorKind::custom("zamani.test", "");

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "operator.name"
        })
    ));
}

// =============================================================================
// Hamiltonian-term semantics
// =============================================================================

#[test]
fn Hamiltonian_term_requires_at_least_one_target() {
    let result = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        Vec::new(),
        constant(1.0),
    );

    assert!(matches!(
        result,
        Err(AnalogError::EmptyOperatorTargets)
    ));
}

#[test]
fn Hamiltonian_term_rejects_duplicate_targets() {
    let result = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        vec![qubit(3), qubit(3)],
        constant(1.0),
    );

    assert!(matches!(
        result,
        Err(AnalogError::DuplicateOperatorTarget {
            resource
        }) if resource == qubit(3)
    ));
}

#[test]
fn Hamiltonian_term_preserves_targets_and_coefficient() {
    let targets = vec![qubit(4), qubit(8)];

    let term = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        targets.clone(),
        constant(0.75),
    )
    .expect("valid Hamiltonian term");

    assert_eq!(term.targets(), targets.as_slice());

    assert_eq!(
        term.coefficient(),
        &constant(0.75)
    );

    assert_eq!(
        term.operator(),
        &OperatorKind::Standard(StandardOperator::PauliZ)
    );
}

#[test]
fn Hamiltonian_term_supports_time_dependent_control_reference() {
    let term = z_term(qubit(5), 1.0)
        .with_control("detuning")
        .expect("non-empty control name");

    assert_eq!(term.control(), Some("detuning"));
}

#[test]
fn Hamiltonian_term_rejects_empty_control_reference() {
    let result = z_term(qubit(5), 1.0).and_then(|term| {
        term.with_control("")
    });

    assert!(matches!(
        result,
        Err(AnalogError::EmptyString {
            field: "hamiltonian_term.control"
        })
    ));
}

#[test]
fn Hamiltonian_term_metadata_is_deterministic() {
    let mut term = z_term(qubit(0), 1.0);

    term.insert_metadata("unit", "Hz")
        .expect("first metadata entry");

    term.insert_metadata("role", "interaction")
        .expect("second metadata entry");

    let metadata = term.metadata();

    assert_eq!(metadata.get("unit"), Some(&"Hz".to_owned()));
    assert_eq!(
        metadata.get("role"),
        Some(&"interaction".to_owned())
    );
}

#[test]
fn Hamiltonian_term_rejects_duplicate_metadata_keys() {
    let mut term = z_term(qubit(0), 1.0);

    term.insert_metadata("unit", "Hz")
        .expect("first metadata entry");

    let result = term.insert_metadata("unit", "rad/s");

    assert!(matches!(
        result,
        Err(AnalogError::DuplicateMetadataKey { key })
            if key == "unit"
    ));
}

#[test]
fn Hamiltonian_term_rejects_empty_metadata_keys() {
    let mut term = z_term(qubit(0), 1.0);

    let result = term.insert_metadata("", "value");

    assert!(matches!(
        result,
        Err(AnalogError::EmptyMetadataKey)
    ));
}

// =============================================================================
// Hamiltonian semantics
// =============================================================================

#[test]
fn empty_hamiltonian_is_valid_as_a_constructible_container() {
    let hamiltonian = AnalogHamiltonian::new();

    assert!(hamiltonian.terms().is_empty());
}

#[test]
fn Hamiltonian_preserves_term_order() {
    let first = z_term(qubit(0), 1.0);

    let second = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliX),
        vec![qubit(1)],
        constant(2.0),
    )
    .expect("valid second term");

    let hamiltonian =
        AnalogHamiltonian::from_terms(vec![first.clone(), second.clone()]);

    assert_eq!(hamiltonian.terms(), &[first, second]);
}

#[test]
fn Hamiltonian_detects_time_dependent_terms() {
    let static_term = z_term(qubit(0), 1.0);

    let static_hamiltonian =
        AnalogHamiltonian::from_terms(vec![static_term]);

    assert!(!static_hamiltonian.is_time_dependent());

    let controlled_term = z_term(qubit(1), 1.0)
        .with_control("detuning")
        .expect("valid control reference");

    let dynamic_hamiltonian =
        AnalogHamiltonian::from_terms(vec![controlled_term]);

    assert!(dynamic_hamiltonian.is_time_dependent());
}

#[test]
fn Hamiltonian_push_preserves_dynamic_state() {
    let mut hamiltonian = AnalogHamiltonian::new();

    hamiltonian.push_term(z_term(qubit(0), 1.0));

    assert_eq!(hamiltonian.terms().len(), 1);
    assert!(!hamiltonian.is_time_dependent());

    let dynamic =
        z_term(qubit(1), 2.0)
            .with_control("omega")
            .expect("valid control");

    hamiltonian.push_term(dynamic);

    assert_eq!(hamiltonian.terms().len(), 2);
    assert!(hamiltonian.is_time_dependent());
}

// =============================================================================
// Symbolic analog semantics
// =============================================================================

#[test]
fn symbolic_Hamiltonian_coefficients_are_not_forced_to_numeric_values() {
    let coefficient = Parameter::Symbol("J".to_owned());

    let term = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliZ),
        vec![qubit(0), qubit(1)],
        coefficient,
    )
    .expect("symbolic coefficient must be accepted");

    assert!(term.coefficient().is_symbolic());
}

#[test]
fn symbolic_control_profile_and_symbolic_Hamiltonian_can_coexist() {
    let omega = TimeSample::new(
        0.0,
        Parameter::Symbol("omega_max".to_owned()),
    )
    .expect("symbolic sample");

    let profile =
        ControlProfile::new(
            vec![omega],
            Interpolation::TargetDefined,
        )
        .expect("symbolic control profile");

    let term = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::PauliX),
        vec![qubit(0)],
        Parameter::Symbol("amplitude".to_owned()),
    )
    .expect("symbolic Hamiltonian coefficient");

    assert!(profile.is_symbolic());
    assert!(term.coefficient().is_symbolic());
}

// =============================================================================
// Global/local analog control model
// =============================================================================

#[test]
fn global_and_local_controls_are_semantically_distinct() {
    let global =
        AnalogControl::new("global", TargetSet::All, profile())
            .expect("global control");

    let local_targets =
        TargetSet::explicit(vec![qubit(1), qubit(4)])
            .expect("local targets");

    let local =
        AnalogControl::new("local", local_targets, profile())
            .expect("local control");

    assert!(global.targets().is_global());
    assert!(!local.targets().is_global());

    assert!(local.targets().explicit_targets().is_some());
}

// =============================================================================
// Large-workload / scalability tests
// =============================================================================

#[test]
fn resource_collections_scale_without_a_semantic_qubit_limit() {
    // This is a workload size, NOT a supported-machine limit.
    //
    // The production model stores resources as data. This test merely verifies
    // that the semantic representation does not contain a small fixed-size
    // assumption.
    let count = 10_000usize;

    let resources = resources(count);

    assert_eq!(resources.len(), count);

    for (index, resource) in resources.iter().enumerate() {
        assert_eq!(resource.id(), qubit(index as u64));
    }
}

#[test]
fn explicit_targets_scale_without_fixed_gate_arity() {
    // The universal analog target set is not restricted to one-, two-, or
    // three-resource operations.
    let count = 1_024usize;

    let targets: Vec<QubitId> =
        (0..count).map(|index| qubit(index as u64)).collect();

    let target_set =
        TargetSet::explicit(targets.clone())
            .expect("large explicit target sets must remain representable");

    assert_eq!(
        target_set
            .explicit_targets()
            .expect("explicit target set"),
        targets.as_slice()
    );
}

#[test]
fn Hamiltonian_terms_can_target_many_resources() {
    let count = 512usize;

    let targets: Vec<QubitId> =
        (0..count).map(|index| qubit(index as u64)).collect();

    let term = HamiltonianTerm::new(
        OperatorKind::Standard(StandardOperator::Spin),
        targets.clone(),
        constant(1.0),
    )
    .expect("many-body semantic terms must be representable");

    assert_eq!(term.targets(), targets.as_slice());
}

#[test]
fn many_Hamiltonian_terms_remain_ordered_and_deterministic() {
    let count = 1_024usize;

    let mut hamiltonian = AnalogHamiltonian::new();

    for index in 0..count {
        hamiltonian.push_term(
            z_term(qubit(index as u64), 1.0),
        );
    }

    assert_eq!(hamiltonian.terms().len(), count);

    for (index, term) in hamiltonian.terms().iter().enumerate() {
        assert_eq!(term.targets(), &[qubit(index as u64)]);
    }
}

#[test]
fn large_symbolic_control_profiles_remain_symbolic() {
    let count = 4_096usize;

    let samples: Vec<TimeSample> = (0..count)
        .map(|index| {
            TimeSample::new(
                index as f64,
                Parameter::Symbol(format!("control_{index}")),
            )
            .expect("generated symbolic sample must be valid")
        })
        .collect();

    let profile =
        ControlProfile::new(samples, Interpolation::TargetDefined)
            .expect("large symbolic profile must be valid");

    assert_eq!(profile.samples().len(), count);
    assert!(profile.is_symbolic());
    assert_eq!(profile.end_time(), Some((count - 1) as f64));
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_positions_have_identical_semantic_values() {
    let first =
        Position::three_dimensional(1.0, 2.0, 3.0)
            .expect("first position");

    let second =
        Position::three_dimensional(1.0, 2.0, 3.0)
            .expect("second position");

    assert_eq!(first, second);
}

#[test]
fn identical_resources_have_identical_semantic_values() {
    let position =
        Position::two_dimensional(1.0, 2.0)
            .expect("position");

    let first =
        AnalogResource::with_position(qubit(7), position.clone());

    let second =
        AnalogResource::with_position(qubit(7), position);

    assert_eq!(first, second);
}

#[test]
fn identical_control_profiles_are_deterministically_equal() {
    let first = profile();
    let second = profile();

    assert_eq!(first, second);
}

#[test]
fn identical_Hamiltonians_are_deterministically_equal() {
    let first =
        AnalogHamiltonian::from_terms(vec![
            z_term(qubit(0), 1.0),
            z_term(qubit(1), 2.0),
        ]);

    let second =
        AnalogHamiltonian::from_terms(vec![
            z_term(qubit(0), 1.0),
            z_term(qubit(1), 2.0),
        ]);

    assert_eq!(first, second);
}

// =============================================================================
// Error-contract tests
// =============================================================================

#[test]
fn error_display_is_non_empty_for_every_public_error_variant_used_here() {
    let errors = [
        AnalogError::EmptyPosition,
        AnalogError::EmptyField {
            field: "test",
        },
        AnalogError::EmptyString {
            field: "test",
        },
        AnalogError::NonFiniteValue {
            field: "test",
            value: f64::NAN,
        },
        AnalogError::ValueOutOfRange {
            field: "test",
            value: -1.0,
            minimum: Some(0.0),
            maximum: None,
        },
        AnalogError::InvalidTargetSet,
        AnalogError::EmptyOperatorTargets,
        AnalogError::EmptyMetadataKey,
    ];

    for error in errors {
        assert!(
            !error.to_string().is_empty(),
            "public analog errors must have useful diagnostics"
        );
    }
}

#[test]
fn non_finite_values_never_enter_core_analog_value_paths() {
    assert!(Position::new(vec![f64::NAN]).is_err());
    assert!(Position::new(vec![f64::INFINITY]).is_err());
    assert!(Position::new(vec![f64::NEG_INFINITY]).is_err());

    assert!(TimeSample::new(f64::NAN, constant(1.0)).is_err());
    assert!(TimeSample::new(f64::INFINITY, constant(1.0)).is_err());
    assert!(TimeSample::new(f64::NEG_INFINITY, constant(1.0)).is_err());

    assert!(Parameter::constant(f64::NAN).is_err());
    assert!(Parameter::constant(f64::INFINITY).is_err());
    assert!(Parameter::constant(f64::NEG_INFINITY).is_err());
}

// =============================================================================
// Architectural boundary tests
// =============================================================================

#[test]
fn analog_resource_does_not_require_a_physical_qubit_identifier() {
    let resource =
        AnalogResource::new(qubit(123_456));

    // The semantic resource is identified entirely by the canonical logical
    // QubitId. Physical allocation is deliberately outside this model.
    assert_eq!(resource.id(), qubit(123_456));
    assert!(resource.position().is_none());
}

#[test]
fn analog_targets_are_logical_resource_references() {
    let logical_ids = vec![
        qubit(10),
        qubit(20),
        qubit(30),
    ];

    let targets =
        TargetSet::explicit(logical_ids.clone())
            .expect("logical target references must be valid");

    assert_eq!(
        targets
            .explicit_targets()
            .expect("explicit logical targets"),
        logical_ids.as_slice()
    );
}

#[test]
fn custom_operators_do_not_require_vendor_specific_hardware_types() {
    let operator =
        OperatorKind::custom(
            "zamani.semantic",
            "future_many_body_operator",
        )
        .expect("semantic custom operators must be supported");

    let term =
        HamiltonianTerm::new(
            operator,
            vec![
                qubit(0),
                qubit(1),
                qubit(2),
                qubit(3),
            ],
            Parameter::Symbol("lambda".to_owned()),
        )
        .expect("future semantic operator must be representable");

    assert!(term.coefficient().is_symbolic());
    assert_eq!(term.targets().len(), 4);
}

// =============================================================================
// Cross-model construction contract
// =============================================================================

#[test]
fn complete_minimal_analog_workload_can_be_constructed_from_public_primitives() {
    let resources = vec![
        AnalogResource::with_position(
            qubit(0),
            Position::two_dimensional(0.0, 0.0)
                .expect("valid position"),
        ),
        AnalogResource::with_position(
            qubit(1),
            Position::two_dimensional(1.0, 0.0)
                .expect("valid position"),
        ),
    ];

    let control =
        AnalogControl::new(
            "detuning",
            TargetSet::All,
            ControlProfile::new(
                vec![
                    TimeSample::new(0.0, constant(0.0))
                        .expect("valid sample"),
                    TimeSample::new(10.0, constant(1.0))
                        .expect("valid sample"),
                ],
                Interpolation::PiecewiseLinear,
            )
            .expect("valid profile"),
        )
        .expect("valid control");

    let interaction =
        HamiltonianTerm::new(
            OperatorKind::Standard(StandardOperator::PauliZ),
            vec![qubit(0), qubit(1)],
            constant(0.5),
        )
        .expect("valid interaction")
        .with_control("detuning")
        .expect("valid control reference");

    let hamiltonian =
        AnalogHamiltonian::from_terms(vec![interaction]);

    assert_eq!(resources.len(), 2);
    assert_eq!(control.name(), "detuning");
    assert!(control.targets().is_global());
    assert!(hamiltonian.is_time_dependent());
    assert_eq!(hamiltonian.terms().len(), 1);
}

// =============================================================================
// Regression tests for semantic invariants
// =============================================================================

#[test]
fn duplicate_resource_ids_are_detectable_by_consuming_layers() {
    let first = AnalogResource::new(qubit(1));
    let second = AnalogResource::new(qubit(1));

    let ids: Vec<QubitId> = vec![first.id(), second.id()];

    let unique: BTreeSet<QubitId> =
        ids.iter().copied().collect();

    assert_eq!(ids.len(), 2);
    assert_eq!(unique.len(), 1);

    // The important contract is that the canonical identity is stable and
    // comparable. Whole-program resource validation can therefore reject
    // duplicates without this model inventing another identity domain.
}

#[test]
fn positions_with_different_dimensions_are_not_silently_compared() {
    let one =
        Position::one_dimensional(1.0)
            .expect("valid position");

    let three =
        Position::three_dimensional(1.0, 2.0, 3.0)
            .expect("valid position");

    let result = one.squared_distance(&three);

    assert!(matches!(
        result,
        Err(AnalogError::DimensionMismatch {
            expected: 1,
            actual: 3
        })
    ));
}

#[test]
fn symbolic_semantics_are_preserved_until_downstream_binding() {
    let coefficient =
        Parameter::Symbol("J_max".to_owned());

    let term =
        HamiltonianTerm::new(
            OperatorKind::Standard(StandardOperator::PauliZ),
            vec![qubit(0)],
            coefficient,
        )
        .expect("symbolic coefficient must be accepted");

    // The IR must not resolve symbolic values merely because a test or target
    // happens to know a numerical value today.
    assert!(term.coefficient().is_symbolic());
}

#[test]
fn analog_model_has_no_fixed_small_qubit_universe() {
    let ids = [
        qubit(0),
        qubit(1),
        qubit(63),
        qubit(64),
        qubit(127),
        qubit(128),
        qubit(1_024),
        qubit(65_536),
        qubit(1_000_000),
    ];

    for id in ids {
        let resource = AnalogResource::new(id);
        assert_eq!(resource.id(), id);
    }
}