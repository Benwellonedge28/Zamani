//! Zamani Quantum Hardware — Instruction Set Tests
//!
//! File:
//!     src/quantum/hardware/tests/instruction_set.rs
//!
//! # Purpose
//!
//! Production conformance and regression tests for:
//!
//!     quantum::hardware::instruction_set
//!
//! This test module verifies the complete provider-independent instruction
//! contract without depending on any concrete provider, backend, topology,
//! calibration, scheduler, router, benchmark, or Danga implementation.
//!
//! # Responsibility
//!
//! This file verifies:
//!
//! - canonical instruction identifiers;
//! - identifier validation;
//! - deterministic ordering;
//! - instruction kinds;
//! - operand schemas;
//! - parameter schemas;
//! - parameter domains;
//! - capability requirements;
//! - instruction semantic invariants;
//! - instruction validation;
//! - instruction-set registration;
//! - duplicate detection;
//! - alias registration and resolution;
//! - instruction removal;
//! - compatibility projection through `canonical_names()`;
//! - standard gate-model vocabulary;
//! - measurement/reset definitions;
//! - instruction filtering;
//! - complete instruction-set validation;
//! - serialization round trips;
//! - deterministic serialization;
//! - error classification;
//! - regression protection for malformed instruction metadata.
//!
//! # Non-responsibility
//!
//! This file does NOT test:
//!
//! - physical topology;
//! - calibration;
//! - backend execution;
//! - provider authentication;
//! - provider APIs;
//! - network transport;
//! - routing algorithms;
//! - scheduling algorithms;
//! - quantum IR semantics;
//! - benchmark protocols;
//! - pulse waveform generation;
//! - analog execution;
//! - annealing execution;
//! - Danga.
//!
//! Those concerns have their own contracts and tests.
//!
//! # Architectural integration
//!
//! The test target is intentionally limited to the public instruction-set API.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::hardware::instruction_set
//!                 ▲
//!                 │
//!                 │ tested by
//!                 │
//! hardware/tests/instruction_set.rs
//! ```
//!
//! Future modules consume the instruction model rather than changing these
//! tests merely to integrate with another hardware subsystem.
//!
//! # Integration into `hardware/mod.rs`
//!
//! This file is intentionally self-contained. The parent module should include
//! it exactly once through a test-only module declaration, for example:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "tests/instruction_set.rs"]
//! mod instruction_set_tests;
//! ```
//!
//! That declaration is test composition only; it does not alter the production
//! hardware API.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are used.
//!
//! # Test philosophy
//!
//! These tests verify observable contracts rather than implementation details.
//! In particular, tests do not depend on the internal representation of
//! `InstructionSet` beyond behavior guaranteed by its public API.
//!
//! A change from `BTreeMap` to another deterministic representation should not
//! require rewriting these tests as long as the public contract remains valid.
//!
//! # Production acceptance rule
//!
//! The instruction-set implementation is not considered complete if any test
//! in this file fails.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use super::super::instruction_set::{
    cx_instruction,
    h_instruction,
    x_instruction,
    y_instruction,
    z_instruction,
    Instruction,
    InstructionCapability,
    InstructionError,
    InstructionId,
    InstructionIdError,
    InstructionKind,
    InstructionSet,
    InstructionSetMetadata,
    InteroperabilityNames,
    OperandKind,
    OperandSpec,
    ParameterDomain,
    ParameterDomainError,
    ParameterSpec,
    ParameterType,
    INSTRUCTION_MODEL_VERSION,
    INSTRUCTION_SET_SCHEMA_VERSION,
};

use pretty_assertions::assert_eq;
use std::collections::BTreeSet;

// =============================================================================
// Test helpers
// =============================================================================

fn instruction_id(value: &str) -> InstructionId {
    InstructionId::new(value)
        .unwrap_or_else(|error| panic!("expected valid instruction ID: {}", error))
}

fn standard_set() -> InstructionSet {
    InstructionSet::standard_gate_model()
        .expect("standard gate-model instruction set must be valid")
}

fn simple_custom_instruction(
    id: &str,
) -> Instruction {
    Instruction::new(
        instruction_id(id),
        InstructionKind::Custom,
        id,
        Vec::new(),
        Vec::new(),
    )
    .expect("custom instruction must be valid")
}

fn qubit_instruction(
    id: &str,
) -> Instruction {
    Instruction::new(
        instruction_id(id),
        InstructionKind::Custom,
        id,
        vec![OperandSpec::required(0, OperandKind::Qubit)],
        Vec::new(),
    )
    .expect("qubit instruction must be valid")
}

// =============================================================================
// InstructionId
// =============================================================================

#[test]
fn instruction_id_accepts_valid_identifier() {
    let id = InstructionId::new("cx")
        .expect("cx must be a valid instruction ID");

    assert_eq!(id.as_str(), "cx");
    assert_eq!(id.to_string(), "cx");
}

#[test]
fn instruction_id_supports_documented_identifier_namespace_characters() {
    let valid_ids = [
        "x",
        "rz",
        "pulse.play",
        "analog.evolve",
        "anneal.run",
        "custom:vendor.operation",
        "vendor/custom.operation",
        "gate-v1",
        "gate_v1",
        "u3",
    ];

    for value in valid_ids {
        assert!(
            InstructionId::new(value).is_ok(),
            "expected '{}' to be accepted",
            value
        );
    }
}

#[test]
fn instruction_id_rejects_empty_identifier() {
    let error = InstructionId::new("")
        .expect_err("empty identifier must be rejected");

    assert_eq!(error, InstructionIdError::Empty);
}

#[test]
fn instruction_id_rejects_leading_whitespace() {
    let error = InstructionId::new(" cx")
        .expect_err("leading whitespace must be rejected");

    assert_eq!(error, InstructionIdError::Whitespace);
}

#[test]
fn instruction_id_rejects_trailing_whitespace() {
    let error = InstructionId::new("cx ")
        .expect_err("trailing whitespace must be rejected");

    assert_eq!(error, InstructionIdError::Whitespace);
}

#[test]
fn instruction_id_rejects_internal_whitespace() {
    let error = InstructionId::new("c x")
        .expect_err("internal whitespace must be rejected");

    assert_eq!(error, InstructionIdError::Whitespace);
}

#[test]
fn instruction_id_rejects_non_ascii() {
    let error = InstructionId::new("méasure")
        .expect_err("non-ASCII identifiers must be rejected");

    assert_eq!(error, InstructionIdError::NonAscii);
}

#[test]
fn instruction_id_rejects_unsupported_characters() {
    let invalid_ids = [
        "cx!",
        "cx?",
        "cx()",
        "cx@provider",
        "cx#1",
        "cx$1",
        "cx%1",
        "cx=1",
    ];

    for value in invalid_ids {
        assert!(
            matches!(
                InstructionId::new(value),
                Err(InstructionIdError::InvalidCharacters { .. })
            ),
            "expected '{}' to be rejected for invalid characters",
            value
        );
    }
}

#[test]
fn instruction_id_rejects_uppercase_input_under_current_canonical_contract() {
    // The implementation validates the identifier alphabet before applying
    // lowercase canonicalization. This test locks the current stable contract
    // so that accidental acceptance of a new spelling does not silently alter
    // identity semantics.
    assert!(
        InstructionId::new("CX").is_err(),
        "uppercase identifiers must not silently create a second spelling"
    );
}

#[test]
fn instruction_id_rejects_identifier_over_maximum_length() {
    let value = "a".repeat(129);

    let error = InstructionId::new(value)
        .expect_err("overlong instruction IDs must be rejected");

    assert!(matches!(
        error,
        InstructionIdError::TooLong {
            length: 129,
            maximum: 128
        }
    ));
}

#[test]
fn instruction_id_from_str_matches_constructor() {
    let direct = InstructionId::new("rz")
        .expect("rz must be valid");

    let parsed = "rz"
        .parse::<InstructionId>()
        .expect("FromStr must accept rz");

    assert_eq!(direct, parsed);
}

#[test]
fn instruction_id_is_orderable_and_hashable() {
    let mut ids = BTreeSet::new();

    ids.insert(instruction_id("z"));
    ids.insert(instruction_id("x"));
    ids.insert(instruction_id("h"));

    let ordered: Vec<&str> = ids.iter().map(InstructionId::as_str).collect();

    assert_eq!(ordered, vec!["h", "x", "z"]);
}

// =============================================================================
// InstructionKind
// =============================================================================

#[test]
fn instruction_kind_strings_are_stable() {
    assert_eq!(
        InstructionKind::SingleQubitGate.as_str(),
        "single_qubit_gate"
    );
    assert_eq!(
        InstructionKind::MultiQubitGate.as_str(),
        "multi_qubit_gate"
    );
    assert_eq!(
        InstructionKind::Measurement.as_str(),
        "measurement"
    );
    assert_eq!(
        InstructionKind::MidCircuitMeasurement.as_str(),
        "mid_circuit_measurement"
    );
    assert_eq!(
        InstructionKind::Reset.as_str(),
        "reset"
    );
    assert_eq!(
        InstructionKind::Pulse.as_str(),
        "pulse"
    );
    assert_eq!(
        InstructionKind::Analog.as_str(),
        "analog"
    );
    assert_eq!(
        InstructionKind::Annealing.as_str(),
        "annealing"
    );
    assert_eq!(
        InstructionKind::Logical.as_str(),
        "logical"
    );
    assert_eq!(
        InstructionKind::Photonic.as_str(),
        "photonic"
    );
    assert_eq!(
        InstructionKind::ContinuousVariable.as_str(),
        "continuous_variable"
    );
    assert_eq!(
        InstructionKind::Qudit.as_str(),
        "qudit"
    );
    assert_eq!(
        InstructionKind::Custom.as_str(),
        "custom"
    );
}

#[test]
fn instruction_kind_display_matches_stable_string() {
    assert_eq!(
        InstructionKind::Measurement.to_string(),
        "measurement"
    );

    assert_eq!(
        InstructionKind::Pulse.to_string(),
        "pulse"
    );
}

// =============================================================================
// Operand model
// =============================================================================

#[test]
fn required_operand_constructor_sets_expected_fields() {
    let operand = OperandSpec::required(0, OperandKind::Qubit);

    assert_eq!(operand.position, 0);
    assert_eq!(operand.kind, OperandKind::Qubit);
    assert!(!operand.optional);
    assert_eq!(operand.role, None);
}

#[test]
fn optional_operand_constructor_sets_expected_fields() {
    let operand = OperandSpec::optional(1, OperandKind::ClassicalBit);

    assert_eq!(operand.position, 1);
    assert_eq!(operand.kind, OperandKind::ClassicalBit);
    assert!(operand.optional);
    assert_eq!(operand.role, None);
}

#[test]
fn operand_role_is_preserved() {
    let operand = OperandSpec::required(0, OperandKind::Qubit)
        .with_role("control");

    assert_eq!(
        operand.role.as_deref(),
        Some("control")
    );
}

#[test]
fn all_major_operand_kinds_have_stable_names() {
    let operands = [
        (OperandKind::Qubit, "qubit"),
        (OperandKind::Qudit, "qudit"),
        (OperandKind::Mode, "mode"),
        (OperandKind::BosonicMode, "bosonic_mode"),
        (
            OperandKind::ContinuousVariable,
            "continuous_variable",
        ),
        (OperandKind::ClassicalBit, "classical_bit"),
        (
            OperandKind::ClassicalRegister,
            "classical_register",
        ),
        (
            OperandKind::MeasurementResult,
            "measurement_result",
        ),
        (
            OperandKind::ControlChannel,
            "control_channel",
        ),
        (OperandKind::DriveChannel, "drive_channel"),
        (
            OperandKind::MeasureChannel,
            "measure_channel",
        ),
        (
            OperandKind::AcquireChannel,
            "acquire_channel",
        ),
        (OperandKind::Frame, "frame"),
        (OperandKind::Waveform, "waveform"),
        (
            OperandKind::SpatialField,
            "spatial_field",
        ),
        (
            OperandKind::TemporalField,
            "temporal_field",
        ),
        (OperandKind::Observable, "observable"),
        (OperandKind::LogicalQubit, "logical_qubit"),
        (
            OperandKind::SyndromeRegister,
            "syndrome_register",
        ),
        (
            OperandKind::NetworkEndpoint,
            "network_endpoint",
        ),
        (OperandKind::Resource, "resource"),
        (OperandKind::Custom, "custom"),
    ];

    for (kind, expected) in operands {
        assert_eq!(kind.as_str(), expected);
        assert_eq!(kind.to_string(), expected);
    }
}

// =============================================================================
// Parameter model
// =============================================================================

#[test]
fn real_parameter_constructor_is_correct() {
    let parameter = ParameterSpec::real(0, "value");

    assert_eq!(parameter.position, 0);
    assert_eq!(parameter.name, "value");
    assert_eq!(parameter.parameter_type, ParameterType::Real);
    assert_eq!(
        parameter.domain,
        ParameterDomain::AnyFinite
    );
    assert!(!parameter.optional);
    assert_eq!(parameter.description, None);
}

#[test]
fn angle_parameter_constructor_is_correct() {
    let parameter = ParameterSpec::angle(0, "theta");

    assert_eq!(parameter.position, 0);
    assert_eq!(parameter.name, "theta");
    assert_eq!(
        parameter.parameter_type,
        ParameterType::Angle
    );
    assert_eq!(
        parameter.domain,
        ParameterDomain::AnyFinite
    );
}

#[test]
fn probability_parameter_constructor_is_correct() {
    let parameter = ParameterSpec::probability(0, "p");

    assert_eq!(
        parameter.parameter_type,
        ParameterType::Probability
    );
    assert_eq!(
        parameter.domain,
        ParameterDomain::Probability
    );
}

#[test]
fn integer_parameter_constructor_is_correct() {
    let parameter = ParameterSpec::integer(0, "n");

    assert_eq!(
        parameter.parameter_type,
        ParameterType::Integer
    );
    assert_eq!(
        parameter.domain,
        ParameterDomain::AnyInteger
    );
}

#[test]
fn parameter_builder_methods_are_composable() {
    let parameter = ParameterSpec::angle(0, "theta")
        .with_domain(ParameterDomain::InclusiveRange {
            min: -std::f64::consts::PI,
            max: std::f64::consts::PI,
        })
        .optional()
        .with_description("Rotation angle in radians.");

    assert_eq!(parameter.position, 0);
    assert_eq!(parameter.name, "theta");
    assert!(parameter.optional);
    assert_eq!(
        parameter.description.as_deref(),
        Some("Rotation angle in radians.")
    );
}

// =============================================================================
// Parameter domains
// =============================================================================

#[test]
fn any_finite_accepts_finite_values() {
    assert!(
        ParameterDomain::AnyFinite
            .validate_f64(0.0)
            .is_ok()
    );

    assert!(
        ParameterDomain::AnyFinite
            .validate_f64(-123.5)
            .is_ok()
    );

    assert!(
        ParameterDomain::AnyFinite
            .validate_f64(123.5)
            .is_ok()
    );
}

#[test]
fn any_finite_rejects_nan() {
    let error = ParameterDomain::AnyFinite
        .validate_f64(f64::NAN)
        .expect_err("NaN must be rejected");

    assert!(matches!(
        error,
        ParameterDomainError::NonFinite { .. }
    ));
}

#[test]
fn any_finite_rejects_positive_infinity() {
    let error = ParameterDomain::AnyFinite
        .validate_f64(f64::INFINITY)
        .expect_err("positive infinity must be rejected");

    assert!(matches!(
        error,
        ParameterDomainError::NonFinite { .. }
    ));
}

#[test]
fn any_finite_rejects_negative_infinity() {
    let error = ParameterDomain::AnyFinite
        .validate_f64(f64::NEG_INFINITY)
        .expect_err("negative infinity must be rejected");

    assert!(matches!(
        error,
        ParameterDomainError::NonFinite { .. }
    ));
}

#[test]
fn inclusive_range_accepts_boundaries() {
    let domain = ParameterDomain::InclusiveRange {
        min: -1.0,
        max: 1.0,
    };

    assert!(domain.validate_f64(-1.0).is_ok());
    assert!(domain.validate_f64(0.0).is_ok());
    assert!(domain.validate_f64(1.0).is_ok());
}

#[test]
fn inclusive_range_rejects_outside_values() {
    let domain = ParameterDomain::InclusiveRange {
        min: -1.0,
        max: 1.0,
    };

    assert!(domain.validate_f64(-1.0001).is_err());
    assert!(domain.validate_f64(1.0001).is_err());
}

#[test]
fn exclusive_range_rejects_boundaries() {
    let domain = ParameterDomain::ExclusiveRange {
        min: -1.0,
        max: 1.0,
    };

    assert!(domain.validate_f64(-1.0).is_err());
    assert!(domain.validate_f64(1.0).is_err());
    assert!(domain.validate_f64(0.0).is_ok());
}

#[test]
fn inclusive_exclusive_range_has_correct_boundary_semantics() {
    let domain = ParameterDomain::InclusiveExclusiveRange {
        min: 0.0,
        max: 1.0,
    };

    assert!(domain.validate_f64(0.0).is_ok());
    assert!(domain.validate_f64(0.999999).is_ok());
    assert!(domain.validate_f64(1.0).is_err());
}

#[test]
fn exclusive_inclusive_range_has_correct_boundary_semantics() {
    let domain = ParameterDomain::ExclusiveInclusiveRange {
        min: 0.0,
        max: 1.0,
    };

    assert!(domain.validate_f64(0.0).is_err());
    assert!(domain.validate_f64(0.000001).is_ok());
    assert!(domain.validate_f64(1.0).is_ok());
}

#[test]
fn non_negative_domain_rejects_negative_values() {
    let domain = ParameterDomain::NonNegativeFinite;

    assert!(domain.validate_f64(0.0).is_ok());
    assert!(domain.validate_f64(1.0).is_ok());
    assert!(domain.validate_f64(-0.000001).is_err());
}

#[test]
fn positive_domain_rejects_zero_and_negative_values() {
    let domain = ParameterDomain::PositiveFinite;

    assert!(domain.validate_f64(1.0).is_ok());
    assert!(domain.validate_f64(0.0).is_err());
    assert!(domain.validate_f64(-1.0).is_err());
}

#[test]
fn probability_domain_accepts_only_unit_interval() {
    let domain = ParameterDomain::Probability;

    assert!(domain.validate_f64(0.0).is_ok());
    assert!(domain.validate_f64(0.5).is_ok());
    assert!(domain.validate_f64(1.0).is_ok());

    assert!(domain.validate_f64(-0.000001).is_err());
    assert!(domain.validate_f64(1.000001).is_err());
}

#[test]
fn integer_range_accepts_boundaries() {
    let domain = ParameterDomain::IntegerRange {
        min: -2,
        max: 2,
    };

    assert!(domain.validate_i64(-2).is_ok());
    assert!(domain.validate_i64(0).is_ok());
    assert!(domain.validate_i64(2).is_ok());
}

#[test]
fn integer_range_rejects_outside_values() {
    let domain = ParameterDomain::IntegerRange {
        min: -2,
        max: 2,
    };

    assert!(domain.validate_i64(-3).is_err());
    assert!(domain.validate_i64(3).is_err());
}

#[test]
fn integer_domain_rejects_floating_validation() {
    let error = ParameterDomain::AnyInteger
        .validate_f64(1.0)
        .expect_err("integer domain must reject floating validation");

    assert_eq!(
        error,
        ParameterDomainError::WrongValueKind
    );
}

#[test]
fn floating_domain_rejects_integer_validation() {
    let error = ParameterDomain::AnyFinite
        .validate_i64(1)
        .expect_err("floating domain must reject integer validation");

    assert_eq!(
        error,
        ParameterDomainError::WrongValueKind
    );
}

#[test]
fn invalid_real_bounds_are_rejected() {
    let domain = ParameterDomain::InclusiveRange {
        min: 2.0,
        max: 1.0,
    };

    let error = domain
        .validate_f64(1.5)
        .expect_err("invalid bounds must be rejected");

    assert_eq!(
        error,
        ParameterDomainError::InvalidBounds
    );
}

#[test]
fn invalid_integer_bounds_are_rejected() {
    let domain = ParameterDomain::IntegerRange {
        min: 2,
        max: 1,
    };

    let error = domain
        .validate_i64(1)
        .expect_err("invalid integer bounds must be rejected");

    assert_eq!(
        error,
        ParameterDomainError::InvalidBounds
    );
}

// =============================================================================
// Interoperability metadata
// =============================================================================

#[test]
fn interoperability_defaults_are_empty() {
    let names = InteroperabilityNames::new();

    assert_eq!(names.openqasm, None);
    assert_eq!(names.qir, None);
    assert_eq!(names.quil, None);
    assert!(names.provider_aliases.is_empty());
}

#[test]
fn interoperability_builder_preserves_standard_names() {
    let names = InteroperabilityNames::new()
        .with_openqasm("cx")
        .with_qir("llvm.cx")
        .with_quil("CNOT")
        .with_provider_alias("CX")
        .with_provider_alias("vendor.cx");

    assert_eq!(names.openqasm.as_deref(), Some("cx"));
    assert_eq!(names.qir.as_deref(), Some("llvm.cx"));
    assert_eq!(names.quil.as_deref(), Some("CNOT"));

    assert!(
        names.provider_aliases.contains("cx"),
        "provider aliases are normalized to lowercase"
    );

    assert!(
        names.provider_aliases.contains("vendor.cx")
    );
}

// =============================================================================
// Instruction construction
// =============================================================================

#[test]
fn simple_custom_instruction_is_valid() {
    let instruction = simple_custom_instruction("custom.test");

    assert_eq!(
        instruction.id.as_str(),
        "custom.test"
    );
    assert_eq!(
        instruction.kind,
        InstructionKind::Custom
    );
    assert_eq!(instruction.name, "custom.test");
    assert!(instruction.operands.is_empty());
    assert!(instruction.parameters.is_empty());
    assert!(instruction.required_capabilities.is_empty());
    assert!(!instruction.unitary);
    assert!(!instruction.reversible);
    assert!(!instruction.adjoint);
    assert!(!instruction.controllable);
    assert!(!instruction.classically_controllable);
    assert!(!instruction.dynamic);
}

#[test]
fn single_qubit_gate_constructor_creates_one_qubit_operand() {
    let instruction =
        Instruction::single_qubit_gate("u", "U")
            .expect("single-qubit gate must be constructible");

    assert_eq!(
        instruction.kind,
        InstructionKind::SingleQubitGate
    );

    assert_eq!(
        instruction.operands.len(),
        1
    );

    assert_eq!(
        instruction.operands[0].position,
        0
    );

    assert_eq!(
        instruction.operands[0].kind,
        OperandKind::Qubit
    );
}

#[test]
fn two_qubit_gate_constructor_creates_control_and_target() {
    let instruction =
        Instruction::two_qubit_gate("cx", "Controlled-X")
            .expect("two-qubit gate must be constructible");

    assert_eq!(
        instruction.kind,
        InstructionKind::MultiQubitGate
    );

    assert_eq!(
        instruction.operands.len(),
        2
    );

    assert_eq!(
        instruction.operands[0].role.as_deref(),
        Some("control")
    );

    assert_eq!(
        instruction.operands[1].role.as_deref(),
        Some("target")
    );
}

#[test]
fn measurement_constructor_requires_measurement_capability() {
    let instruction =
        Instruction::measurement("measure")
            .expect("measurement must be constructible");

    assert_eq!(
        instruction.kind,
        InstructionKind::Measurement
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Measurement)
    );

    assert_eq!(
        instruction.required_operand_count(),
        2
    );
}

#[test]
fn reset_constructor_requires_reset_capability() {
    let instruction =
        Instruction::reset("reset")
            .expect("reset must be constructible");

    assert_eq!(
        instruction.kind,
        InstructionKind::Reset
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Reset)
    );
}

// =============================================================================
// Instruction semantics
// =============================================================================

#[test]
fn unitary_builder_marks_instruction_unitary() {
    let instruction =
        simple_custom_instruction("unitary")
            .unitary();

    assert!(instruction.unitary);
    assert!(instruction.validate().is_ok());
}

#[test]
fn reversible_requires_unitary() {
    let instruction =
        simple_custom_instruction("reversible")
            .reversible();

    let error = instruction
        .validate()
        .expect_err("reversible non-unitary instruction must fail");

    assert!(matches!(
        error,
        InstructionError::InvalidSemantics { .. }
    ));
}

#[test]
fn adjoint_requires_unitary() {
    let instruction =
        simple_custom_instruction("adjoint")
            .with_adjoint();

    let error = instruction
        .validate()
        .expect_err("adjoint non-unitary instruction must fail");

    assert!(matches!(
        error,
        InstructionError::InvalidSemantics { .. }
    ));
}

#[test]
fn measurement_cannot_be_unitary() {
    let instruction =
        Instruction::measurement("measure")
            .expect("measurement must be valid")
            .unitary();

    let error = instruction
        .validate()
        .expect_err("measurement cannot be unitary");

    assert!(matches!(
        error,
        InstructionError::InvalidSemantics { .. }
    ));
}

#[test]
fn reset_cannot_be_unitary() {
    let instruction =
        Instruction::reset("reset")
            .expect("reset must be valid")
            .unitary();

    let error = instruction
        .validate()
        .expect_err("reset cannot be unitary");

    assert!(matches!(
        error,
        InstructionError::InvalidSemantics { .. }
    ));
}

#[test]
fn dynamic_instruction_requires_dynamic_circuit_capability() {
    let instruction =
        simple_custom_instruction("dynamic")
            .dynamic();

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::DynamicCircuits)
    );

    assert!(instruction.validate().is_ok());
}

#[test]
fn requiring_adds_one_capability() {
    let instruction =
        simple_custom_instruction("measurement_like")
            .requiring(InstructionCapability::Measurement);

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Measurement)
    );
}

#[test]
fn requiring_all_adds_all_capabilities() {
    let instruction =
        simple_custom_instruction("dynamic_measure")
            .requiring_all([
                InstructionCapability::Measurement,
                InstructionCapability::MidCircuitMeasurement,
                InstructionCapability::DynamicCircuits,
            ]);

    assert_eq!(
        instruction.required_capabilities.len(),
        3
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Measurement)
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::MidCircuitMeasurement)
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::DynamicCircuits)
    );
}

#[test]
fn controllability_flags_are_independent() {
    let instruction =
        simple_custom_instruction("controlled")
            .controllable()
            .classically_controllable();

    assert!(instruction.controllable);
    assert!(instruction.classically_controllable);
}

// =============================================================================
// Operand validation
// =============================================================================

#[test]
fn duplicate_operand_positions_are_rejected() {
    let result = Instruction::new(
        instruction_id("bad.operands"),
        InstructionKind::Custom,
        "Bad operands",
        vec![
            OperandSpec::required(0, OperandKind::Qubit),
            OperandSpec::required(0, OperandKind::Qubit),
        ],
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(InstructionError::DuplicateOperandPosition {
            position: 0
        })
    ));
}

#[test]
fn non_contiguous_operand_positions_are_rejected() {
    let result = Instruction::new(
        instruction_id("bad.operands"),
        InstructionKind::Custom,
        "Bad operands",
        vec![
            OperandSpec::required(0, OperandKind::Qubit),
            OperandSpec::required(2, OperandKind::Qubit),
        ],
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(InstructionError::NonContiguousOperandPositions)
    ));
}

#[test]
fn required_operand_after_optional_operand_is_rejected() {
    let result = Instruction::new(
        instruction_id("bad.operands"),
        InstructionKind::Custom,
        "Bad operands",
        vec![
            OperandSpec::optional(0, OperandKind::Qubit),
            OperandSpec::required(1, OperandKind::Qubit),
        ],
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(InstructionError::RequiredOperandAfterOptional)
    ));
}

#[test]
fn empty_operand_role_is_rejected() {
    let result = Instruction::new(
        instruction_id("bad.role"),
        InstructionKind::Custom,
        "Bad role",
        vec![
            OperandSpec::required(0, OperandKind::Qubit)
                .with_role("   "),
        ],
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(InstructionError::EmptyOperandRole {
            position: 0
        })
    ));
}

// =============================================================================
// Parameter validation
// =============================================================================

#[test]
fn duplicate_parameter_positions_are_rejected() {
    let result = Instruction::new(
        instruction_id("bad.parameters"),
        InstructionKind::Custom,
        "Bad parameters",
        Vec::new(),
        vec![
            ParameterSpec::real(0, "a"),
            ParameterSpec::real(0, "b"),
        ],
    );

    assert!(matches!(
        result,
        Err(InstructionError::DuplicateParameterPosition {
            position: 0
        })
    ));
}

#[test]
fn non_contiguous_parameter_positions_are_rejected() {
    let result = Instruction::new(
        instruction_id("bad.parameters"),
        InstructionKind::Custom,
        "Bad parameters",
        Vec::new(),
        vec![
            ParameterSpec::real(0, "a"),
            ParameterSpec::real(2, "b"),
        ],
    );

    assert!(matches!(
        result,
        Err(InstructionError::NonContiguousParameterPositions)
    ));
}

#[test]
fn required_parameter_after_optional_parameter_is_rejected() {
    let result = Instruction::new(
        instruction_id("bad.parameters"),
        InstructionKind::Custom,
        "Bad parameters",
        Vec::new(),
        vec![
            ParameterSpec::real(0, "optional").optional(),
            ParameterSpec::real(1, "required"),
        ],
    );

    assert!(matches!(
        result,
        Err(InstructionError::RequiredParameterAfterOptional)
    ));
}

#[test]
fn empty_parameter_name_is_rejected() {
    let result = Instruction::new(
        instruction_id("bad.parameter.name"),
        InstructionKind::Custom,
        "Bad parameter",
        Vec::new(),
        vec![ParameterSpec::real(0, "   ")],
    );

    assert!(matches!(
        result,
        Err(InstructionError::EmptyParameterName {
            position: 0
        })
    ));
}

#[test]
fn invalid_parameter_domain_is_rejected() {
    let result = Instruction::new(
        instruction_id("bad.domain"),
        InstructionKind::Custom,
        "Bad domain",
        Vec::new(),
        vec![
            ParameterSpec::real(0, "x")
                .with_domain(ParameterDomain::InclusiveRange {
                    min: 10.0,
                    max: 1.0,
                }),
        ],
    );

    assert!(matches!(
        result,
        Err(InstructionError::InvalidParameterDomain {
            position: 0
        })
    ));
}

// =============================================================================
// Operand/parameter cardinality
// =============================================================================

#[test]
fn required_operand_count_is_correct() {
    let instruction = Instruction::new(
        instruction_id("cardinality"),
        InstructionKind::Custom,
        "Cardinality",
        vec![
            OperandSpec::required(0, OperandKind::Qubit),
            OperandSpec::required(1, OperandKind::Qubit),
            OperandSpec::optional(2, OperandKind::ClassicalBit),
        ],
        Vec::new(),
    )
    .expect("instruction must be valid");

    assert_eq!(
        instruction.required_operand_count(),
        2
    );

    assert_eq!(
        instruction.maximum_operand_count(),
        3
    );
}

#[test]
fn operand_count_acceptance_respects_optional_operands() {
    let instruction = Instruction::new(
        instruction_id("cardinality"),
        InstructionKind::Custom,
        "Cardinality",
        vec![
            OperandSpec::required(0, OperandKind::Qubit),
            OperandSpec::optional(1, OperandKind::ClassicalBit),
        ],
        Vec::new(),
    )
    .expect("instruction must be valid");

    assert!(!instruction.accepts_operand_count(0));
    assert!(instruction.accepts_operand_count(1));
    assert!(instruction.accepts_operand_count(2));
    assert!(!instruction.accepts_operand_count(3));
}

#[test]
fn parameter_count_acceptance_respects_optional_parameters() {
    let instruction = Instruction::new(
        instruction_id("parameters"),
        InstructionKind::Custom,
        "Parameters",
        Vec::new(),
        vec![
            ParameterSpec::real(0, "theta"),
            ParameterSpec::real(1, "phase").optional(),
        ],
    )
    .expect("instruction must be valid");

    assert!(!instruction.accepts_parameter_count(0));
    assert!(instruction.accepts_parameter_count(1));
    assert!(instruction.accepts_parameter_count(2));
    assert!(!instruction.accepts_parameter_count(3));
}

// =============================================================================
// InstructionSet construction
// =============================================================================

#[test]
fn new_instruction_set_is_empty() {
    let set = InstructionSet::new();

    assert_eq!(set.len(), 0);
    assert_eq!(set.alias_count(), 0);
    assert!(set.is_empty());
}

#[test]
fn default_instruction_set_matches_new() {
    let set = InstructionSet::default();

    assert_eq!(set.len(), 0);
    assert_eq!(set.alias_count(), 0);
    assert!(set.is_empty());
}

#[test]
fn default_metadata_contains_current_schema_versions() {
    let metadata = InstructionSetMetadata::default();

    assert_eq!(
        metadata.schema_version,
        INSTRUCTION_SET_SCHEMA_VERSION
    );

    assert_eq!(
        metadata.model_version,
        INSTRUCTION_MODEL_VERSION
    );

    assert_eq!(
        metadata.implementation_version,
        None
    );

    assert_eq!(metadata.source, None);
}

#[test]
fn metadata_builder_preserves_versions() {
    let metadata = InstructionSetMetadata::new()
        .with_implementation_version("provider-v7")
        .with_source("test-provider");

    assert_eq!(
        metadata.schema_version,
        INSTRUCTION_SET_SCHEMA_VERSION
    );

    assert_eq!(
        metadata.model_version,
        INSTRUCTION_MODEL_VERSION
    );

    assert_eq!(
        metadata.implementation_version.as_deref(),
        Some("provider-v7")
    );

    assert_eq!(
        metadata.source.as_deref(),
        Some("test-provider")
    );
}

// =============================================================================
// InstructionSet registration
// =============================================================================

#[test]
fn registration_adds_instruction() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("custom.test"))
        .expect("registration must succeed");

    assert_eq!(set.len(), 1);
    assert!(!set.is_empty());

    let id = instruction_id("custom.test");

    assert!(
        set.get(&id).is_some(),
        "registered instruction must be retrievable"
    );
}

#[test]
fn duplicate_instruction_registration_is_rejected() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("duplicate"))
        .expect("first registration must succeed");

    let error = set
        .register(simple_custom_instruction("duplicate"))
        .expect_err("duplicate registration must fail");

    assert!(matches!(
        error,
        InstructionError::DuplicateInstruction { .. }
    ));

    assert_eq!(
        set.len(),
        1,
        "failed registration must not mutate the set"
    );
}

#[test]
fn invalid_instruction_is_rejected_before_mutation() {
    let mut set = InstructionSet::new();

    let invalid = Instruction::new(
        instruction_id("invalid"),
        InstructionKind::Custom,
        "Invalid",
        vec![
            OperandSpec::optional(0, OperandKind::Qubit),
            OperandSpec::required(1, OperandKind::Qubit),
        ],
        Vec::new(),
    );

    assert!(invalid.is_err());

    assert_eq!(
        set.len(),
        0,
        "invalid construction must not affect the set"
    );
}

#[test]
fn get_returns_none_for_unknown_instruction() {
    let set = InstructionSet::new();

    assert!(
        set.get(&instruction_id("missing")).is_none()
    );
}

#[test]
fn iter_is_deterministically_ordered() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("z"))
        .unwrap();
    set.register(simple_custom_instruction("a"))
        .unwrap();
    set.register(simple_custom_instruction("m"))
        .unwrap();

    let ids: Vec<&str> = set
        .iter()
        .map(|(id, _)| id.as_str())
        .collect();

    assert_eq!(
        ids,
        vec!["a", "m", "z"]
    );
}

#[test]
fn ids_are_deterministically_ordered() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("z"))
        .unwrap();
    set.register(simple_custom_instruction("a"))
        .unwrap();
    set.register(simple_custom_instruction("m"))
        .unwrap();

    let ids: Vec<&str> = set
        .ids()
        .map(InstructionId::as_str)
        .collect();

    assert_eq!(
        ids,
        vec!["a", "m", "z"]
    );
}

// =============================================================================
// Alias handling
// =============================================================================

#[test]
fn alias_can_be_registered_for_existing_instruction() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("controlled_x"))
        .unwrap();

    set.register_alias(
        "cx",
        &instruction_id("controlled_x"),
    )
    .expect("alias registration must succeed");

    assert_eq!(set.alias_count(), 1);
}

#[test]
fn alias_resolves_to_target_instruction() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("controlled_x"))
        .unwrap();

    set.register_alias(
        "cx",
        &instruction_id("controlled_x"),
    )
    .unwrap();

    let instruction = set
        .resolve("cx")
        .expect("alias must resolve");

    assert_eq!(
        instruction.id.as_str(),
        "controlled_x"
    );
}

#[test]
fn canonical_name_resolves_without_alias() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("cx"))
        .unwrap();

    let instruction = set
        .resolve("cx")
        .expect("canonical ID must resolve");

    assert_eq!(instruction.id.as_str(), "cx");
}

#[test]
fn unknown_instruction_resolution_fails() {
    let set = InstructionSet::new();

    let error = set
        .resolve("missing")
        .expect_err("unknown instruction must fail");

    assert!(matches!(
        error,
        InstructionError::InstructionNotFound { .. }
    ));
}

#[test]
fn alias_to_unknown_instruction_is_rejected() {
    let mut set = InstructionSet::new();

    let error = set
        .register_alias(
            "cx",
            &instruction_id("missing"),
        )
        .expect_err("unknown alias target must fail");

    assert!(matches!(
        error,
        InstructionError::AliasTargetNotFound { .. }
    ));

    assert_eq!(
        set.alias_count(),
        0,
        "failed alias registration must not mutate the set"
    );
}

#[test]
fn duplicate_alias_is_rejected() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("target"))
        .unwrap();

    let target = instruction_id("target");

    set.register_alias("cx", &target)
        .unwrap();

    let error = set
        .register_alias("cx", &target)
        .expect_err("duplicate alias must fail");

    assert!(matches!(
        error,
        InstructionError::DuplicateAlias { .. }
    ));

    assert_eq!(set.alias_count(), 1);
}

#[test]
fn alias_cannot_conflict_with_canonical_instruction_id() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("cx"))
        .unwrap();

    let error = set
        .register_alias(
            "cx",
            &instruction_id("cx"),
        )
        .expect_err("alias must not shadow canonical ID");

    assert!(matches!(
        error,
        InstructionError::AliasConflictsWithInstruction { .. }
    ));
}

#[test]
fn removing_instruction_removes_its_aliases() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("controlled_x"))
        .unwrap();

    let target = instruction_id("controlled_x");

    set.register_alias("cx", &target)
        .unwrap();

    assert_eq!(set.alias_count(), 1);

    let removed = set
        .remove(&target)
        .expect("existing instruction must be removable");

    assert_eq!(
        removed.id.as_str(),
        "controlled_x"
    );

    assert_eq!(set.len(), 0);
    assert_eq!(
        set.alias_count(),
        0,
        "aliases to removed instructions must be removed"
    );
}

#[test]
fn removing_unknown_instruction_returns_structured_error() {
    let mut set = InstructionSet::new();

    let error = set
        .remove(&instruction_id("missing"))
        .expect_err("unknown instruction must fail");

    assert!(matches!(
        error,
        InstructionError::InstructionNotFound { .. }
    ));
}

// =============================================================================
// Capability filtering
// =============================================================================

#[test]
fn requiring_capability_returns_matching_instructions() {
    let mut set = InstructionSet::new();

    set.register(
        simple_custom_instruction("measurement")
            .requiring(InstructionCapability::Measurement),
    )
    .unwrap();

    set.register(
        simple_custom_instruction("reset")
            .requiring(InstructionCapability::Reset),
    )
    .unwrap();

    let matching = set.requiring_capability(
        InstructionCapability::Measurement,
    );

    assert_eq!(matching.len(), 1);
    assert_eq!(
        matching[0].id.as_str(),
        "measurement"
    );
}

#[test]
fn requiring_capability_returns_empty_for_missing_capability() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("x"))
        .unwrap();

    let matching = set.requiring_capability(
        InstructionCapability::PulseControl,
    );

    assert!(matching.is_empty());
}

#[test]
fn of_kind_returns_only_matching_kind() {
    let mut set = InstructionSet::new();

    set.register(
        Instruction::single_qubit_gate("x", "X")
            .unwrap(),
    )
    .unwrap();

    set.register(
        Instruction::measurement("measure")
            .unwrap(),
    )
    .unwrap();

    let gates = set.of_kind(
        InstructionKind::SingleQubitGate
    );

    assert_eq!(gates.len(), 1);
    assert_eq!(gates[0].id.as_str(), "x");
}

// =============================================================================
// Compatibility projection
// =============================================================================

#[test]
fn canonical_names_contains_only_canonical_instruction_ids() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("x"))
        .unwrap();

    set.register(simple_custom_instruction("cx"))
        .unwrap();

    set.register_alias(
        "cnot",
        &instruction_id("cx"),
    )
    .unwrap();

    let names = set.canonical_names();

    assert_eq!(
        names,
        BTreeSet::from([
            "cx".to_owned(),
            "x".to_owned(),
        ])
    );

    assert!(
        !names.contains("cnot"),
        "aliases must not leak into canonical names"
    );
}

#[test]
fn from_canonical_names_creates_valid_custom_instructions() {
    let set = InstructionSet::from_canonical_names([
        "x",
        "cx",
        "vendor.custom",
    ])
    .expect("canonical-name construction must succeed");

    assert_eq!(set.len(), 3);

    assert!(set.contains("x"));
    assert!(set.contains("cx"));
    assert!(set.contains("vendor.custom"));
}

#[test]
fn from_canonical_names_rejects_invalid_names() {
    let result = InstructionSet::from_canonical_names([
        "x",
        "invalid name",
    ]);

    assert!(matches!(
        result,
        Err(InstructionError::InvalidId(
            InstructionIdError::Whitespace
        ))
    ));
}

// =============================================================================
// Standard instruction vocabulary
// =============================================================================

#[test]
fn standard_gate_model_is_valid() {
    let set = standard_set();

    set.validate()
        .expect("standard gate-model set must validate");
}

#[test]
fn standard_gate_model_contains_required_core_instructions() {
    let set = standard_set();

    for name in [
        "x",
        "y",
        "z",
        "h",
        "cx",
        "rz",
        "measure",
        "reset",
    ] {
        assert!(
            set.contains(name),
            "standard set must contain '{}'",
            name
        );
    }
}

#[test]
fn standard_gate_model_has_expected_instruction_count() {
    let set = standard_set();

    assert_eq!(
        set.len(),
        8,
        "the current standard vocabulary contains x, y, z, h, cx, rz, measure and reset"
    );
}

#[test]
fn standard_single_qubit_pauli_gates_are_unitary_and_reversible() {
    let set = standard_set();

    for name in ["x", "y", "z"] {
        let instruction = set
            .resolve(name)
            .expect("standard instruction must exist");

        assert!(instruction.unitary);
        assert!(instruction.reversible);
        assert!(instruction.adjoint);
    }
}

#[test]
fn standard_hadamard_gate_is_unitary_and_reversible() {
    let set = standard_set();

    let instruction = set
        .resolve("h")
        .expect("Hadamard must exist");

    assert!(instruction.unitary);
    assert!(instruction.reversible);
    assert!(instruction.adjoint);
}

#[test]
fn standard_cx_has_two_qubit_schema() {
    let set = standard_set();

    let instruction = set
        .resolve("cx")
        .expect("cx must exist");

    assert_eq!(
        instruction.kind,
        InstructionKind::MultiQubitGate
    );

    assert_eq!(
        instruction.operands.len(),
        2
    );

    assert_eq!(
        instruction.operands[0].kind,
        OperandKind::Qubit
    );

    assert_eq!(
        instruction.operands[1].kind,
        OperandKind::Qubit
    );

    assert_eq!(
        instruction.operands[0].role.as_deref(),
        Some("control")
    );

    assert_eq!(
        instruction.operands[1].role.as_deref(),
        Some("target")
    );
}

#[test]
fn standard_rz_has_one_angle_parameter() {
    let set = standard_set();

    let instruction = set
        .resolve("rz")
        .expect("rz must exist");

    assert_eq!(
        instruction.parameters.len(),
        1
    );

    let parameter = &instruction.parameters[0];

    assert_eq!(
        parameter.name,
        "theta"
    );

    assert_eq!(
        parameter.parameter_type,
        ParameterType::Angle
    );

    assert_eq!(
        parameter.domain,
        ParameterDomain::AnyFinite
    );
}

#[test]
fn standard_rz_requires_parameterized_circuit_capability() {
    let set = standard_set();

    let instruction = set
        .resolve("rz")
        .expect("rz must exist");

    assert!(
        instruction
            .required_capabilities
            .contains(
                &InstructionCapability::ParameterizedCircuits
            )
    );
}

#[test]
fn standard_measurement_has_measurement_capability() {
    let set = standard_set();

    let instruction = set
        .resolve("measure")
        .expect("measure must exist");

    assert_eq!(
        instruction.kind,
        InstructionKind::Measurement
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Measurement)
    );

    assert!(!instruction.unitary);
}

#[test]
fn standard_reset_has_reset_capability() {
    let set = standard_set();

    let instruction = set
        .resolve("reset")
        .expect("reset must exist");

    assert_eq!(
        instruction.kind,
        InstructionKind::Reset
    );

    assert!(
        instruction
            .required_capabilities
            .contains(&InstructionCapability::Reset)
    );

    assert!(!instruction.unitary);
}

#[test]
fn standard_instructions_have_openqasm_names() {
    let set = standard_set();

    for name in [
        "x",
        "y",
        "z",
        "h",
        "cx",
        "measure",
        "reset",
    ] {
        let instruction = set
            .resolve(name)
            .expect("instruction must exist");

        assert_eq!(
            instruction.interoperability.openqasm.as_deref(),
            Some(name)
        );
    }
}

// =============================================================================
// Free standard constructors
// =============================================================================

#[test]
fn x_instruction_matches_standard_set() {
    let instruction =
        x_instruction().expect("x constructor must succeed");

    let set = standard_set();

    let standard = set
        .resolve("x")
        .expect("standard x must exist");

    assert_eq!(
        instruction,
        standard.clone()
    );
}

#[test]
fn y_instruction_matches_standard_set() {
    let instruction =
        y_instruction().expect("y constructor must succeed");

    let set = standard_set();

    let standard = set
        .resolve("y")
        .expect("standard y must exist");

    assert_eq!(
        instruction,
        standard.clone()
    );
}

#[test]
fn z_instruction_matches_standard_set() {
    let instruction =
        z_instruction().expect("z constructor must succeed");

    let set = standard_set();

    let standard = set
        .resolve("z")
        .expect("standard z must exist");

    assert_eq!(
        instruction,
        standard.clone()
    );
}

#[test]
fn h_instruction_matches_standard_set() {
    let instruction =
        h_instruction().expect("h constructor must succeed");

    let set = standard_set();

    let standard = set
        .resolve("h")
        .expect("standard h must exist");

    assert_eq!(
        instruction,
        standard.clone()
    );
}

#[test]
fn cx_instruction_matches_standard_set() {
    let instruction =
        cx_instruction().expect("cx constructor must succeed");

    let set = standard_set();

    let standard = set
        .resolve("cx")
        .expect("standard cx must exist");

    assert_eq!(
        instruction,
        standard.clone()
    );
}

// =============================================================================
// InstructionSet validation
// =============================================================================

#[test]
fn empty_instruction_set_is_valid() {
    let set = InstructionSet::new();

    set.validate()
        .expect("empty instruction set is a valid set");
}

#[test]
fn instruction_set_with_registered_valid_instruction_is_valid() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("valid"))
        .unwrap();

    set.validate()
        .expect("valid instruction set must validate");
}

#[test]
fn zero_schema_version_is_rejected() {
    let mut metadata = InstructionSetMetadata::default();
    metadata.schema_version = 0;

    let set = InstructionSet::with_metadata(metadata);

    let error = set
        .validate()
        .expect_err("zero schema version must fail");

    assert!(matches!(
        error,
        InstructionError::InvalidInstructionSet { .. }
    ));
}

#[test]
fn zero_model_version_is_rejected() {
    let mut metadata = InstructionSetMetadata::default();
    metadata.model_version = 0;

    let set = InstructionSet::with_metadata(metadata);

    let error = set
        .validate()
        .expect_err("zero model version must fail");

    assert!(matches!(
        error,
        InstructionError::InvalidInstructionSet { .. }
    ));
}

// =============================================================================
// Serialization
// =============================================================================

#[test]
fn instruction_id_json_round_trip_is_lossless() {
    let id = instruction_id("custom.vendor.operation");

    let serialized =
        serde_json::to_string(&id)
            .expect("InstructionId must serialize");

    let deserialized =
        serde_json::from_str::<InstructionId>(&serialized)
            .expect("InstructionId must deserialize");

    assert_eq!(id, deserialized);
}

#[test]
fn instruction_json_round_trip_is_lossless() {
    let instruction =
        Instruction::two_qubit_gate("cx", "Controlled-X")
            .unwrap()
            .unitary()
            .reversible()
            .with_adjoint()
            .with_interoperability(
                InteroperabilityNames::new()
                    .with_openqasm("cx")
                    .with_qir("cx"),
            );

    let serialized =
        serde_json::to_string(&instruction)
            .expect("instruction must serialize");

    let deserialized =
        serde_json::from_str::<Instruction>(&serialized)
            .expect("instruction must deserialize");

    assert_eq!(
        instruction,
        deserialized
    );
}

#[test]
fn instruction_set_json_round_trip_is_lossless() {
    let mut set = standard_set();

    set.register_alias(
        "cnot",
        &instruction_id("cx"),
    )
    .expect("alias must register");

    let serialized =
        serde_json::to_string(&set)
            .expect("instruction set must serialize");

    let deserialized =
        serde_json::from_str::<InstructionSet>(&serialized)
            .expect("instruction set must deserialize");

    assert_eq!(
        set,
        deserialized
    );

    deserialized
        .validate()
        .expect("deserialized instruction set must validate");
}

#[test]
fn instruction_set_serialization_is_deterministic() {
    let mut first = InstructionSet::new();

    first
        .register(simple_custom_instruction("z"))
        .unwrap();

    first
        .register(simple_custom_instruction("a"))
        .unwrap();

    first
        .register(simple_custom_instruction("m"))
        .unwrap();

    let mut second = InstructionSet::new();

    second
        .register(simple_custom_instruction("m"))
        .unwrap();

    second
        .register(simple_custom_instruction("z"))
        .unwrap();

    second
        .register(simple_custom_instruction("a"))
        .unwrap();

    let first_json =
        serde_json::to_string(&first)
            .expect("first set must serialize");

    let second_json =
        serde_json::to_string(&second)
            .expect("second set must serialize");

    assert_eq!(
        first_json,
        second_json,
        "instruction-set serialization must not depend on registration order"
    );
}

#[test]
fn aliases_survive_serialization() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("controlled_x"))
        .unwrap();

    set.register_alias(
        "cx",
        &instruction_id("controlled_x"),
    )
    .unwrap();

    let json =
        serde_json::to_string(&set)
            .expect("set must serialize");

    let restored =
        serde_json::from_str::<InstructionSet>(&json)
            .expect("set must deserialize");

    let instruction = restored
        .resolve("cx")
        .expect("alias must survive round trip");

    assert_eq!(
        instruction.id.as_str(),
        "controlled_x"
    );
}

// =============================================================================
// Regression tests for compatibility and safety
// =============================================================================

#[test]
fn aliases_do_not_create_duplicate_canonical_instructions() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("cx"))
        .unwrap();

    set.register_alias(
        "cnot",
        &instruction_id("cx"),
    )
    .unwrap();

    assert_eq!(set.len(), 1);
    assert_eq!(set.alias_count(), 1);

    let canonical_names = set.canonical_names();

    assert_eq!(
        canonical_names.len(),
        1
    );

    assert!(
        canonical_names.contains("cx")
    );
}

#[test]
fn failed_duplicate_registration_does_not_replace_original_instruction() {
    let mut set = InstructionSet::new();

    let original =
        Instruction::single_qubit_gate(
            "x",
            "Original X",
        )
        .unwrap();

    set.register(original)
        .unwrap();

    let replacement =
        Instruction::single_qubit_gate(
            "x",
            "Replacement X",
        )
        .unwrap();

    let error = set
        .register(replacement)
        .expect_err("duplicate registration must fail");

    assert!(matches!(
        error,
        InstructionError::DuplicateInstruction { .. }
    ));

    let stored = set
        .resolve("x")
        .expect("original instruction must remain");

    assert_eq!(
        stored.name,
        "Original X"
    );
}

#[test]
fn failed_alias_registration_does_not_mutate_alias_count() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("x"))
        .unwrap();

    let error = set
        .register_alias(
            "missing-target",
            &instruction_id("does-not-exist"),
        )
        .expect_err("invalid target must fail");

    assert!(matches!(
        error,
        InstructionError::AliasTargetNotFound { .. }
    ));

    assert_eq!(
        set.alias_count(),
        0
    );
}

#[test]
fn remove_preserves_other_instructions_and_aliases() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("a"))
        .unwrap();

    set.register(simple_custom_instruction("b"))
        .unwrap();

    set.register_alias(
        "bee",
        &instruction_id("b"),
    )
    .unwrap();

    set.remove(&instruction_id("a"))
        .unwrap();

    assert_eq!(set.len(), 1);
    assert_eq!(set.alias_count(), 1);

    assert!(
        set.resolve("b").is_ok()
    );

    assert!(
        set.resolve("bee").is_ok()
    );
}

#[test]
fn contains_supports_canonical_names_and_aliases() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("controlled_x"))
        .unwrap();

    set.register_alias(
        "cx",
        &instruction_id("controlled_x"),
    )
    .unwrap();

    assert!(set.contains("controlled_x"));
    assert!(set.contains("cx"));
    assert!(!set.contains("missing"));
}

#[test]
fn capabilities_iterator_is_deterministic() {
    let instruction =
        simple_custom_instruction("capabilities")
            .requiring_all([
                InstructionCapability::Reset,
                InstructionCapability::Measurement,
                InstructionCapability::PulseControl,
            ]);

    let names: Vec<&str> = instruction
        .capabilities()
        .map(|capability| capability.as_str())
        .collect();

    assert_eq!(
        names,
        vec![
            "measurement",
            "pulse_control",
            "reset",
        ]
    );
}

#[test]
fn instruction_capability_strings_are_stable() {
    let capabilities = [
        (
            InstructionCapability::Measurement,
            "measurement",
        ),
        (
            InstructionCapability::MidCircuitMeasurement,
            "mid_circuit_measurement",
        ),
        (
            InstructionCapability::Reset,
            "reset",
        ),
        (
            InstructionCapability::ClassicalControl,
            "classical_control",
        ),
        (
            InstructionCapability::DynamicCircuits,
            "dynamic_circuits",
        ),
        (
            InstructionCapability::ParameterizedCircuits,
            "parameterized_circuits",
        ),
        (
            InstructionCapability::PulseControl,
            "pulse_control",
        ),
        (
            InstructionCapability::AnalogControl,
            "analog_control",
        ),
        (
            InstructionCapability::Annealing,
            "annealing",
        ),
        (
            InstructionCapability::LogicalQubits,
            "logical_qubits",
        ),
        (
            InstructionCapability::FaultTolerantExecution,
            "fault_tolerant_execution",
        ),
        (
            InstructionCapability::SyndromeMeasurement,
            "syndrome_measurement",
        ),
        (
            InstructionCapability::Photonic,
            "photonic",
        ),
        (
            InstructionCapability::ContinuousVariable,
            "continuous_variable",
        ),
        (
            InstructionCapability::Qudit,
            "qudit",
        ),
        (
            InstructionCapability::QuantumNetworking,
            "quantum_networking",
        ),
        (
            InstructionCapability::Custom,
            "custom",
        ),
    ];

    for (capability, expected) in capabilities {
        assert_eq!(
            capability.as_str(),
            expected
        );
    }
}

// =============================================================================
// Cross-contract production tests
// =============================================================================

#[test]
fn standard_instruction_set_has_no_duplicate_canonical_ids() {
    let set = standard_set();

    let ids: Vec<&InstructionId> =
        set.ids().collect();

    let unique: BTreeSet<&InstructionId> =
        ids.iter().copied().collect();

    assert_eq!(
        ids.len(),
        unique.len(),
        "canonical instruction IDs must be unique"
    );
}

#[test]
fn every_standard_instruction_validates_individually() {
    let set = standard_set();

    for (id, instruction) in set.iter() {
        instruction
            .validate()
            .unwrap_or_else(|error| {
                panic!(
                    "standard instruction '{}' failed validation: {}",
                    id,
                    error
                )
            });
    }
}

#[test]
fn every_standard_instruction_has_a_non_empty_display_name() {
    let set = standard_set();

    for (id, instruction) in set.iter() {
        assert!(
            !instruction.name.trim().is_empty(),
            "instruction '{}' must have a display name",
            id
        );
    }
}

#[test]
fn every_standard_instruction_has_contiguous_operands() {
    let set = standard_set();

    for (id, instruction) in set.iter() {
        for (expected, operand) in
            instruction.operands.iter().enumerate()
        {
            assert_eq!(
                operand.position,
                expected,
                "instruction '{}' has invalid operand position",
                id
            );
        }
    }
}

#[test]
fn every_standard_instruction_has_contiguous_parameters() {
    let set = standard_set();

    for (id, instruction) in set.iter() {
        for (expected, parameter) in
            instruction.parameters.iter().enumerate()
        {
            assert_eq!(
                parameter.position,
                expected,
                "instruction '{}' has invalid parameter position",
                id
            );
        }
    }
}

#[test]
fn standard_measurement_is_not_marked_reversible() {
    let set = standard_set();

    let measure = set
        .resolve("measure")
        .expect("measure must exist");

    assert!(!measure.reversible);
    assert!(!measure.adjoint);
}

#[test]
fn standard_reset_is_not_marked_reversible() {
    let set = standard_set();

    let reset = set
        .resolve("reset")
        .expect("reset must exist");

    assert!(!reset.reversible);
    assert!(!reset.adjoint);
}

#[test]
fn standard_rz_parameter_domain_rejects_non_finite_values() {
    let set = standard_set();

    let rz = set
        .resolve("rz")
        .expect("rz must exist");

    let parameter = &rz.parameters[0];

    assert!(
        parameter
            .domain
            .validate_f64(f64::NAN)
            .is_err()
    );

    assert!(
        parameter
            .domain
            .validate_f64(f64::INFINITY)
            .is_err()
    );
}

#[test]
fn standard_instruction_set_can_be_used_as_backend_name_projection() {
    let set = standard_set();

    let names = set.canonical_names();

    assert!(names.contains("x"));
    assert!(names.contains("y"));
    assert!(names.contains("z"));
    assert!(names.contains("h"));
    assert!(names.contains("cx"));
    assert!(names.contains("rz"));
    assert!(names.contains("measure"));
    assert!(names.contains("reset"));

    assert_eq!(
        names.len(),
        set.len(),
        "canonical projection must preserve one name per instruction"
    );
}

// =============================================================================
// Error display regression tests
// =============================================================================

#[test]
fn instruction_id_error_has_human_readable_message() {
    let error = InstructionId::new("")
        .expect_err("empty ID must fail");

    let message = error.to_string();

    assert!(
        message.contains("cannot be empty")
    );
}

#[test]
fn instruction_error_has_human_readable_duplicate_message() {
    let mut set = InstructionSet::new();

    set.register(simple_custom_instruction("duplicate"))
        .unwrap();

    let error = set
        .register(simple_custom_instruction("duplicate"))
        .expect_err("duplicate must fail");

    let message = error.to_string();

    assert!(
        message.contains("duplicate")
    );
}

#[test]
fn parameter_domain_error_has_human_readable_message() {
    let error = ParameterDomain::Probability
        .validate_f64(2.0)
        .expect_err("2.0 is not a probability");

    let message = error.to_string();

    assert!(
        message.contains("outside")
    );
}

// =============================================================================
// Provider-neutrality regression tests
// =============================================================================

#[test]
fn instruction_set_contains_no_provider_specific_runtime_state() {
    // This is intentionally a compile-time/API-shape test expressed through
    // the public model. Provider names may appear only as optional metadata or
    // aliases; the instruction itself has no credential, transport, or client
    // object.
    let instruction =
        simple_custom_instruction("provider.custom");

    assert!(
        instruction.description.is_none()
    );

    assert!(
        instruction
            .required_capabilities
            .is_empty()
    );
}

#[test]
fn provider_aliases_do_not_change_canonical_identity() {
    let instruction =
        Instruction::single_qubit_gate(
            "vendor.x",
            "Vendor X",
        )
        .unwrap()
        .with_interoperability(
            InteroperabilityNames::new()
                .with_openqasm("x")
                .with_provider_alias("X")
                .with_provider_alias("vendor_x"),
        );

    assert_eq!(
        instruction.id.as_str(),
        "vendor.x"
    );

    assert_eq!(
        instruction.interoperability.openqasm.as_deref(),
        Some("x")
    );
}

// =============================================================================
// Final conformance gate
// =============================================================================

#[test]
fn instruction_set_production_conformance_gate() {
    let mut set = standard_set();

    set.register_alias(
        "cnot",
        &instruction_id("cx"),
    )
    .expect("standard alias registration must succeed");

    set.validate()
        .expect("complete instruction set must validate");

    assert_eq!(set.len(), 8);
    assert_eq!(set.alias_count(), 1);

    assert!(
        set.resolve("cnot")
            .expect("alias must resolve")
            .id
            .as_str()
            == "cx"
    );

    let json =
        serde_json::to_string(&set)
            .expect("production instruction set must serialize");

    let restored =
        serde_json::from_str::<InstructionSet>(&json)
            .expect("production instruction set must deserialize");

    restored
        .validate()
        .expect("round-tripped production instruction set must validate");

    assert_eq!(
        set,
        restored,
        "serialization must preserve the complete instruction contract"
    );

    let canonical_names =
        restored.canonical_names();

    assert_eq!(
        canonical_names.len(),
        restored.len()
    );

    assert!(
        canonical_names.contains("x")
    );

    assert!(
        canonical_names.contains("cx")
    );

    assert!(
        canonical_names.contains("rz")
    );

    assert!(
        canonical_names.contains("measure")
    );

    assert!(
        canonical_names.contains("reset")
    );
}