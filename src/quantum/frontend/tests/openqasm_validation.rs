//! Zamani Quantum Frontend — OpenQASM semantic-validation integration tests.
//!
//! This file is intentionally an integration-style contract suite for:
//!
//!     OpenQASM source
//!          |
//!          v
//!     OpenQasmParser
//!          |
//!          v
//!     OpenQASM AST
//!          |
//!          v
//!     validate_program_with_config
//!          |
//!          v
//!     ValidationResult
//!
//! The tests do not depend on Validator's private implementation details.
//! They exercise the public parser + validation contracts.
//!
//! # Production contract
//!
//! These tests verify that semantic validation:
//!
//! - accepts semantically valid OpenQASM;
//! - rejects invalid programs deterministically;
//! - reports stable machine-readable error codes;
//! - enforces declaration and scope rules;
//! - enforces quantum/classical type boundaries;
//! - validates indexes and register sizes;
//! - validates gate arity and parameters;
//! - validates broadcasting;
//! - validates gate modifiers;
//! - validates control flow;
//! - validates measurement;
//! - validates include policy;
//! - validates timing/calibration/extension policies;
//! - enforces expression and statement limits;
//! - enforces symbol/resource limits;
//! - never performs external I/O;
//! - does not execute source-level constructs;
//! - remains deterministic;
//! - remains compatible with Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! # Architectural boundary
//!
//! This test suite intentionally does NOT test:
//!
//! - Quantum IR construction;
//! - optimization;
//! - routing;
//! - hardware mapping;
//! - scheduling;
//! - backend execution.
//!
//! Those belong to downstream layers.
//!
//! # OpenQASM authority
//!
//! OpenQASM 3.1 specification:
//! <https://openqasm.com/versions/3.1/>
//!
//! Rust compatibility:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No additional dependencies are required.

use std::collections::BTreeSet;

use crate::quantum::frontend::core::limits::FrontendLimits;
use crate::quantum::frontend::core::source::SourceId;
use crate::quantum::frontend::formats::openqasm::parser::{
    OpenQasmParser,
    ParserConfig,
    ParserLimits,
};
use crate::quantum::frontend::formats::openqasm::validation::{
    validate_program,
    validate_program_with_config,
    ValidationConfig,
    ValidationErrorCode,
    ValidationResult,
};

// ============================================================================
// Test infrastructure
// ============================================================================

const TEST_SOURCE_ID: u64 = 0x5141_534D_5445_5354;

/// Parse source using the same parser contract used by the production
/// OpenQASM importer.
fn parse(source: &str) -> crate::quantum::frontend::formats::openqasm::ast::Program {
    let config = ParserConfig {
        source_id: SourceId::from_raw(TEST_SOURCE_ID),
        limits: ParserLimits::default(),
    };

    OpenQasmParser::parse(source, config)
        .unwrap_or_else(|error| {
            panic!(
                "test source must be syntactically valid OpenQASM:\n\
                 source:\n{}\n\
                 parser error: {}",
                source,
                error
            )
        })
}

/// Parse source and return the validation result under production policy.
fn validate(source: &str) -> ValidationResult {
    let program = parse(source);

    validate_program(
        &program,
        &FrontendLimits::production(),
    )
}

/// Parse source and validate with an explicit validation configuration.
fn validate_with_config(
    source: &str,
    config: ValidationConfig,
) -> ValidationResult {
    let program = parse(source);

    validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    )
}

/// Assert that a program validates successfully.
fn assert_valid(source: &str) {
    let result = validate(source);

    assert!(
        result.is_valid(),
        "expected valid OpenQASM, got validation errors:\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

/// Assert that a program is rejected with a particular semantic error code.
fn assert_has_code(
    source: &str,
    expected: ValidationErrorCode,
) {
    let result = validate(source);

    assert!(
        result.is_invalid(),
        "expected validation failure `{}` but program was accepted:\n{}",
        expected,
        source
    );

    assert!(
        result
            .errors()
            .iter()
            .any(|error| error.code() == expected),
        "expected validation error `{}`.\n\
         source:\n{}\n\
         actual errors:\n{:#?}",
        expected,
        source,
        result.errors()
    );
}

/// Assert that a program is rejected and contains all requested error codes.
fn assert_has_all_codes(
    source: &str,
    expected: &[ValidationErrorCode],
) {
    let result = validate(source);

    assert!(
        result.is_invalid(),
        "expected validation failure but program was accepted:\n{}",
        source
    );

    for code in expected {
        assert!(
            result.errors().iter().any(|error| error.code() == *code),
            "expected validation error `{}`.\n\
             source:\n{}\n\
             actual errors:\n{:#?}",
            code,
            source,
            result.errors()
        );
    }
}

/// Assert that a program validates under one policy but not another.
fn assert_validity_changes_with_policy(
    source: &str,
    permissive: ValidationConfig,
    restrictive: ValidationConfig,
    expected_restrictive_error: ValidationErrorCode,
) {
    let permissive_result =
        validate_with_config(source, permissive);

    assert!(
        permissive_result.is_valid(),
        "expected permissive policy to accept source.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        permissive_result.errors()
    );

    let restrictive_result =
        validate_with_config(source, restrictive);

    assert!(
        restrictive_result.is_invalid(),
        "expected restrictive policy to reject source.\n\
         source:\n{}\n\
         expected error: {}\n\
         actual errors:\n{:#?}",
        source,
        expected_restrictive_error,
        restrictive_result.errors()
    );

    assert!(
        restrictive_result
            .errors()
            .iter()
            .any(|error| {
                error.code() == expected_restrictive_error
            }),
        "expected restrictive policy error `{}`.\n\
         source:\n{}\n\
         actual errors:\n{:#?}",
        expected_restrictive_error,
        source,
        restrictive_result.errors()
    );
}

/// Return the set of error codes emitted by a validation result.
///
/// BTreeSet gives the test a deterministic representation and prevents tests
/// from accidentally depending on diagnostic ordering unless ordering itself
/// is explicitly being tested.
fn error_codes(
    result: &ValidationResult,
) -> BTreeSet<ValidationErrorCode> {
    result
        .errors()
        .iter()
        .map(|error| error.code())
        .collect()
}

// ============================================================================
// Baseline / production-policy tests
// ============================================================================

#[test]
fn production_policy_targets_openqasm_3_1() {
    let config = ValidationConfig::production();

    assert_eq!(config.max_major_version, 3);
    assert_eq!(config.max_minor_version, 1);

    assert!(!config.allow_missing_version);
    assert!(!config.allow_legacy_declarations);

    assert!(config.allow_includes);

    assert!(!config.allow_extern);
    assert!(!config.allow_calibration);

    assert!(config.allow_timing);

    assert!(!config.allow_extensions);
    assert!(!config.allow_physical_qubits);

    assert!(config.allow_pragmas);
    assert!(config.allow_annotations);

    assert!(!config.allow_runtime_quantum_index);

    assert!(config.max_expression_depth > 0);
    assert!(config.max_expression_nodes > 0);
    assert!(config.max_register_size > 0);
    assert!(config.max_symbols > 0);
    assert!(config.max_parameters > 0);
    assert!(config.max_operands > 0);
    assert!(config.max_gate_call_depth > 0);
    assert!(config.max_statements > 0);
}

#[test]
fn strict_policy_disables_external_or_implementation_defined_features() {
    let config = ValidationConfig::strict();

    assert!(!config.allow_missing_version);
    assert!(!config.allow_includes);
    assert!(!config.allow_extern);
    assert!(!config.allow_calibration);
    assert!(!config.allow_timing);
    assert!(!config.allow_extensions);
    assert!(!config.allow_physical_qubits);
    assert!(!config.allow_pragmas);
    assert!(!config.allow_annotations);
    assert!(!config.allow_runtime_quantum_index);
}

// ============================================================================
// Minimal valid programs
// ============================================================================

#[test]
fn minimal_versioned_program_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
"#,
    );
}

#[test]
fn empty_versioned_program_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.0;
"#,
    );
}

#[test]
fn basic_qubit_and_bit_declarations_are_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit[2] q;
bit[2] c;
"#,
    );
}

#[test]
fn basic_single_qubit_gate_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;
h q;
"#,
    );
}

#[test]
fn basic_two_qubit_gate_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
cx q[0], q[1];
"#,
    );
}

#[test]
fn basic_measurement_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit[2] q;
bit[2] c;

c = measure q;
"#,
    );
}

#[test]
fn basic_reset_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit q;
reset q;
"#,
    );
}

#[test]
fn basic_barrier_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit[2] q;
barrier q;
"#,
    );
}

// ============================================================================
// Version validation
// ============================================================================

#[test]
fn missing_version_is_rejected_by_production_policy() {
    let source = r#"
qubit q;
"#;

    assert_has_code(
        source,
        ValidationErrorCode::MissingVersion,
    );
}

#[test]
fn missing_version_can_be_enabled_explicitly() {
    let source = r#"
qubit q;
"#;

    let mut config = ValidationConfig::production();
    config.allow_missing_version = true;

    assert_validity_changes_with_policy(
        source,
        config,
        ValidationConfig::production(),
        ValidationErrorCode::MissingVersion,
    );
}

#[test]
fn version_3_0_is_accepted_by_production_3_1_policy() {
    assert_valid(
        r#"
OPENQASM 3.0;

qubit q;
"#,
    );
}

#[test]
fn unsupported_version_policy_rejects_program_when_ast_version_is_outside_policy() {
    let source = r#"
OPENQASM 3.1;

qubit q;
"#;

    let mut config = ValidationConfig::production();
    config.max_minor_version = 0;

    assert_has_code_with_config(
        source,
        config,
        ValidationErrorCode::UnsupportedVersion,
    );
}

fn assert_has_code_with_config(
    source: &str,
    config: ValidationConfig,
    expected: ValidationErrorCode,
) {
    let result = validate_with_config(source, config);

    assert!(
        result.is_invalid(),
        "expected validation failure `{}` but source was accepted:\n{}",
        expected,
        source
    );

    assert!(
        result
            .errors()
            .iter()
            .any(|error| error.code() == expected),
        "expected `{}`.\nsource:\n{}\nerrors:\n{:#?}",
        expected,
        source,
        result.errors()
    );
}

// ============================================================================
// Declaration and scope validation
// ============================================================================

#[test]
fn duplicate_global_declaration_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;
qubit q;
"#,
        ValidationErrorCode::DuplicateDeclaration,
    );
}

#[test]
fn duplicate_classical_declaration_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

bit c;
bit c;
"#,
        ValidationErrorCode::DuplicateDeclaration,
    );
}

#[test]
fn unknown_identifier_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;
h missing;
"#,
        ValidationErrorCode::UndefinedIdentifier,
    );
}

#[test]
fn classical_object_cannot_be_used_as_quantum_operand() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

bit c;
h c;
"#,
        ValidationErrorCode::OperandTypeMismatch,
    );
}

#[test]
fn quantum_object_cannot_be_used_as_classical_designator() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;
bit c;

c = q;
"#,
        ValidationErrorCode::OperandTypeMismatch,
    );
}

// ============================================================================
// Register and index validation
// ============================================================================

#[test]
fn quantum_index_zero_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
h q[0];
"#,
    );
}

#[test]
fn quantum_index_at_last_element_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
h q[1];
"#,
    );
}

#[test]
fn quantum_index_out_of_bounds_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
h q[2];
"#,
        ValidationErrorCode::IndexOutOfBounds,
    );
}

#[test]
fn negative_quantum_index_is_rejected_when_constant() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
h q[-1];
"#,
        ValidationErrorCode::IndexOutOfBounds,
    );
}

#[test]
fn runtime_quantum_index_is_disabled_by_production_policy() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

int[32] i;
qubit[4] q;

h q[i];
"#,
        ValidationErrorCode::RuntimeQuantumIndexDisabled,
    );
}

#[test]
fn runtime_quantum_index_can_be_enabled_explicitly() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";

int[32] i;
qubit[4] q;

h q[i];
"#;

    let mut config = ValidationConfig::production();
    config.allow_runtime_quantum_index = true;

    let result =
        validate_with_config(source, config);

    assert!(
        !result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::RuntimeQuantumIndexDisabled
        }),
        "runtime quantum index policy was not enabled:\n{:#?}",
        result.errors()
    );
}

#[test]
fn zero_step_range_is_rejected() {
    let source = r#"
OPENQASM 3.1;

qubit[4] q;

for int i in [0:0:4] {
    reset q[i];
}
"#;

    let result = parse(source);
    let validation =
        validate_program(
            &result,
            &FrontendLimits::production(),
        );

    assert!(
        validation.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::InvalidSlice
        }),
        "expected invalid range step diagnostic.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        validation.errors()
    );
}

// ============================================================================
// Gate validation
// ============================================================================

#[test]
fn unknown_gate_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;
totally_unknown_gate q;
"#,
        ValidationErrorCode::UndefinedGate,
    );
}

#[test]
fn standard_gate_is_resolved_after_standard_library_include() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;
x q;
"#,
    );
}

#[test]
fn standard_gate_parameter_arity_is_validated() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;
rx q;
"#,
        ValidationErrorCode::GateParameterCountMismatch,
    );
}

#[test]
fn standard_gate_qubit_arity_is_validated() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
cx q[0];
"#,
        ValidationErrorCode::GateOperandCountMismatch,
    );
}

#[test]
fn one_qubit_gate_rejects_multiple_scalar_operands_when_not_broadcastable() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
h q[0], q[1];
"#,
        ValidationErrorCode::GateOperandCountMismatch,
    );
}

#[test]
fn matching_register_broadcast_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
qubit[2] r;

cx q, r;
"#,
    );
}

#[test]
fn incompatible_register_broadcast_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
qubit[3] r;

cx q, r;
"#,
        ValidationErrorCode::RegisterBroadcastMismatch,
    );
}

#[test]
fn duplicate_quantum_operand_is_rejected_when_gate_semantics_require_distinct_operands() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;

cx q, q;
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "expected invalid duplicate quantum operands:\n{}",
        source
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::DuplicateQuantumOperand
        }),
        "expected duplicate-operand error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

// ============================================================================
// Gate definitions
// ============================================================================

#[test]
fn simple_gate_definition_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

gate my_gate q {
    x q;
}

qubit q;
my_gate q;
"#,
    );
}

#[test]
fn parameterized_gate_definition_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

gate my_rx(theta) q {
    rx(theta) q;
}

qubit q;
my_rx(1.0) q;
"#,
    );
}

#[test]
fn duplicate_gate_definition_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate q {
    x q;
}

gate my_gate q {
    z q;
}
"#,
        ValidationErrorCode::DuplicateGateDefinition,
    );
}

#[test]
fn duplicate_gate_parameter_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate(theta, theta) q {
    x q;
}
"#,
        ValidationErrorCode::DuplicateGateParameter,
    );
}

#[test]
fn duplicate_gate_formal_operand_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate q, q {
    x q;
}
"#,
        ValidationErrorCode::DuplicateFormalOperand,
    );
}

#[test]
fn undefined_gate_parameter_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate(theta) q {
    rx(phi) q;
}
"#,
        ValidationErrorCode::UndefinedGateParameter,
    );
}

#[test]
fn undefined_formal_gate_operand_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate q {
    x missing;
}
"#,
        ValidationErrorCode::UndefinedFormalOperand,
    );
}

#[test]
fn indexed_gate_formal_operand_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate my_gate q {
    x q[0];
}
"#,
        ValidationErrorCode::IndexedGateFormalOperand,
    );
}

#[test]
fn recursive_gate_definition_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

gate recursive_gate q {
    recursive_gate q;
}
"#,
        ValidationErrorCode::RecursiveGateDefinition,
    );
}

// ============================================================================
// Gate modifiers
// ============================================================================

#[test]
fn inverse_gate_modifier_is_valid_for_supported_gate() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;
inv @ x q;
"#,
    );
}

#[test]
fn controlled_gate_modifier_is_valid_for_supported_gate() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
ctrl @ x q[0], q[1];
"#,
    );
}

// ============================================================================
// Expression validation
// ============================================================================

#[test]
fn integer_classical_declaration_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

int[32] value;
"#,
    );
}

#[test]
fn unsigned_integer_classical_declaration_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

uint[32] value;
"#,
    );
}

#[test]
fn float_classical_declaration_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

float[64] value;
"#,
    );
}

#[test]
fn angle_declaration_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

angle[32] theta;
"#,
    );
}

#[test]
fn boolean_declaration_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

bool flag;
"#,
    );
}

#[test]
fn classical_initializer_type_mismatch_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

bit c = 1;
"#,
        ValidationErrorCode::AssignmentTypeMismatch,
    );
}

#[test]
fn compatible_integer_initializer_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;

int[32] value = 42;
"#,
    );
}

#[test]
fn division_by_zero_is_rejected_when_constant() {
    assert_has_code(
        r#"
OPENQASM 3.1;

int[32] value = 10 / 0;
"#,
        ValidationErrorCode::DivisionByZero,
    );
}

// ============================================================================
// Assignment validation
// ============================================================================

#[test]
fn assignment_to_unknown_identifier_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

unknown = 1;
"#,
        ValidationErrorCode::UndefinedIdentifier,
    );
}

#[test]
fn assignment_to_quantum_object_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;
q = 1;
"#,
        ValidationErrorCode::InvalidAssignmentTarget,
    );
}

// ============================================================================
// Control-flow validation
// ============================================================================

#[test]
fn valid_if_statement_is_accepted() {
    assert_valid(
        r#"
OPENQASM 3.1;

bit c;
qubit q;

if (c) {
    reset q;
}
"#,
    );
}

#[test]
fn invalid_classical_condition_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit q;

if (q) {
    reset q;
}
"#,
        ValidationErrorCode::InvalidCondition,
    );
}

#[test]
fn break_outside_loop_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

break;
"#,
        ValidationErrorCode::BreakOutsideLoop,
    );
}

#[test]
fn continue_outside_loop_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

continue;
"#,
        ValidationErrorCode::ContinueOutsideLoop,
    );
}

#[test]
fn valid_break_inside_loop_is_accepted() {
    assert_valid(
        r#"
OPENQASM 3.1;

for int i in [0:4] {
    break;
}
"#,
    );
}

#[test]
fn valid_continue_inside_loop_is_accepted() {
    assert_valid(
        r#"
OPENQASM 3.1;

for int i in [0:4] {
    continue;
}
"#,
    );
}

#[test]
fn return_outside_subroutine_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

return;
"#,
        ValidationErrorCode::ReturnOutsideSubroutine,
    );
}

// ============================================================================
// Measurement validation
// ============================================================================

#[test]
fn measurement_register_widths_are_compatible() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit[2] q;
bit[2] c;

c = measure q;
"#,
    );
}

#[test]
fn measurement_destination_width_mismatch_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;

qubit[2] q;
bit c;

c = measure q;
"#,
        ValidationErrorCode::InvalidMeasurementDestination,
    );
}

#[test]
fn measurement_source_must_be_quantum() {
    assert_has_code(
        r#"
OPENQASM 3.1;

bit c;
bit result;

result = measure c;
"#,
        ValidationErrorCode::InvalidMeasurementSource,
    );
}

// ============================================================================
// Include validation
// ============================================================================

#[test]
fn standard_library_include_is_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;
h q;
"#,
    );
}

#[test]
fn duplicate_include_is_rejected() {
    assert_has_code(
        r#"
OPENQASM 3.1;
include "stdgates.inc";
include "stdgates.inc";
"#,
        ValidationErrorCode::DuplicateInclude,
    );
}

#[test]
fn include_is_disabled_by_strict_policy() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";
"#;

    assert_has_code_with_config(
        source,
        ValidationConfig::strict(),
        ValidationErrorCode::IncludeDisabled,
    );
}

#[test]
fn include_inside_block_is_rejected() {
    let source = r#"
OPENQASM 3.1;

if (true) {
    include "stdgates.inc";
}
"#;

    assert_has_code(
        source,
        ValidationErrorCode::IncludeOutOfScope,
    );
}

#[test]
fn empty_include_path_is_rejected_if_parser_can_represent_it() {
    let source = r#"
OPENQASM 3.1;
include "";
"#;

    let program = parse(source);
    let result = validate_program(
        &program,
        &FrontendLimits::production(),
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::InvalidInclude
        }),
        "expected invalid include diagnostic.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

// ============================================================================
// Calibration / extern / extension policy
// ============================================================================

#[test]
fn extern_is_disabled_by_production_policy() {
    let source = r#"
OPENQASM 3.1;

extern foo();
"#;

    let result = parse(source);
    let validation =
        validate_program(
            &result,
            &FrontendLimits::production(),
        );

    assert!(
        validation.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::ExternDisabled
        }),
        "expected extern-disabled error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        validation.errors()
    );
}

#[test]
fn calibration_is_disabled_by_production_policy() {
    let source = r#"
OPENQASM 3.1;

cal {
}
"#;

    let result = parse(source);
    let validation =
        validate_program(
            &result,
            &FrontendLimits::production(),
        );

    assert!(
        validation.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::CalibrationDisabled
        }),
        "expected calibration-disabled error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        validation.errors()
    );
}

#[test]
fn defcal_is_disabled_by_production_policy() {
    let source = r#"
OPENQASM 3.1;

defcal q {
}
"#;

    let result = parse(source);
    let validation =
        validate_program(
            &result,
            &FrontendLimits::production(),
        );

    assert!(
        validation.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::CalibrationDisabled
        }),
        "expected calibration-disabled error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        validation.errors()
    );
}

// ============================================================================
// Timing policy
// ============================================================================

#[test]
fn timing_is_allowed_by_production_policy() {
    assert_valid(
        r#"
OPENQASM 3.1;

qubit q;
delay[10ns] q;
"#,
    );
}

#[test]
fn timing_can_be_disabled_explicitly() {
    let source = r#"
OPENQASM 3.1;

qubit q;
delay[10ns] q;
"#;

    let mut config = ValidationConfig::production();
    config.allow_timing = false;

    assert_has_code_with_config(
        source,
        config,
        ValidationErrorCode::TimingDisabled,
    );
}

// ============================================================================
// Pragmas / annotations
// ============================================================================

#[test]
fn pragma_is_allowed_by_production_policy() {
    let source = r#"
OPENQASM 3.1;

#pragma zamani_test
"#;

    let program = parse(source);
    let result = validate_program(
        &program,
        &FrontendLimits::production(),
    );

    assert!(
        !result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::PragmaDisabled
        }),
        "pragma unexpectedly disabled by production policy:\n{:#?}",
        result.errors()
    );
}

#[test]
fn pragma_can_be_disabled_explicitly() {
    let source = r#"
OPENQASM 3.1;

#pragma zamani_test
"#;

    let mut config = ValidationConfig::production();
    config.allow_pragmas = false;

    assert_has_code_with_config(
        source,
        config,
        ValidationErrorCode::PragmaDisabled,
    );
}

// ============================================================================
// Physical-qubit policy
// ============================================================================

#[test]
fn physical_qubits_are_rejected_by_logical_production_policy() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";

x $0;
"#;

    let program = parse(source);
    let result = validate_program(
        &program,
        &FrontendLimits::production(),
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::PhysicalQubitDisabled
        }),
        "expected physical-qubit policy error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

// ============================================================================
// Resource-limit tests
// ============================================================================

#[test]
fn statement_limit_is_enforced_by_validation() {
    let source = r#"
OPENQASM 3.1;

qubit q;

reset q;
reset q;
reset q;
"#;

    let program = parse(source);

    let mut config = ValidationConfig::production();
    config.max_statements = 2;

    let result = validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::StatementLimitExceeded
        }),
        "expected statement-limit error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

#[test]
fn symbol_limit_is_enforced() {
    let source = r#"
OPENQASM 3.1;

bit a;
bit b;
bit c;
"#;

    let program = parse(source);

    let mut config = ValidationConfig::production();
    config.max_symbols = 2;

    let result = validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::SymbolLimitExceeded
        }),
        "expected symbol-limit error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

#[test]
fn parameter_limit_is_enforced() {
    let source = r#"
OPENQASM 3.1;

gate many(theta0, theta1, theta2, theta3) q {
    rx(theta0) q;
}
"#;

    let program = parse(source);

    let mut config = ValidationConfig::production();
    config.max_parameters = 2;

    let result = validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::ParameterLimitExceeded
        }),
        "expected parameter-limit error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

#[test]
fn operand_limit_is_enforced() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[4] q;
cx q[0], q[1];
"#;

    let program = parse(source);

    let mut config = ValidationConfig::production();
    config.max_operands = 1;

    let result = validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::OperandLimitExceeded
        }),
        "expected operand-limit error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

#[test]
fn expression_node_limit_is_enforced() {
    let source = r#"
OPENQASM 3.1;

int[32] value = 1 + 2 + 3 + 4;
"#;

    let program = parse(source);

    let mut config = ValidationConfig::production();
    config.max_expression_nodes = 2;

    let result = validate_program_with_config(
        &program,
        &FrontendLimits::production(),
        config,
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::ExpressionNodeLimitExceeded
        }),
        "expected expression-node limit error.\n\
         source:\n{}\n\
         errors:\n{:#?}",
        source,
        result.errors()
    );
}

// ============================================================================
// Multi-error behavior
// ============================================================================

#[test]
fn validator_reports_multiple_independent_errors_without_panicking() {
    let source = r#"
OPENQASM 3.1;

qubit[2] q;
qubit[3] r;
bit c;

h missing;
cx q, r;
h c;
h q[2];
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "expected invalid program:\n{}",
        source
    );

    let codes = error_codes(&result);

    assert!(
        codes.contains(&ValidationErrorCode::UndefinedIdentifier),
        "missing undefined-identifier diagnostic:\n{:#?}",
        result.errors()
    );

    assert!(
        codes.contains(
            &ValidationErrorCode::RegisterBroadcastMismatch
        ),
        "missing broadcast diagnostic:\n{:#?}",
        result.errors()
    );

    assert!(
        codes.contains(
            &ValidationErrorCode::OperandTypeMismatch
        ),
        "missing operand-type diagnostic:\n{:#?}",
        result.errors()
    );

    assert!(
        codes.contains(
            &ValidationErrorCode::IndexOutOfBounds
        ),
        "missing index diagnostic:\n{:#?}",
        result.errors()
    );
}

// ============================================================================
// Determinism
// ============================================================================

#[test]
fn identical_source_produces_identical_validation_result() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
qubit[3] r;
bit c;

cx q, r;
h c;
h q[2];
"#;

    let first = validate(source);
    let second = validate(source);

    assert_eq!(
        first,
        second,
        "validation must be deterministic"
    );
}

#[test]
fn identical_source_produces_identical_error_code_sequence() {
    let source = r#"
OPENQASM 3.1;

qubit[2] q;
qubit[3] r;

cx q, r;
h q[2];
"#;

    let first = validate(source);
    let second = validate(source);

    let first_codes: Vec<_> =
        first.errors().iter().map(|error| error.code()).collect();

    let second_codes: Vec<_> =
        second.errors().iter().map(|error| error.code()).collect();

    assert_eq!(
        first_codes,
        second_codes,
        "error ordering must be deterministic"
    );
}

// ============================================================================
// Stable diagnostic contracts
// ============================================================================

#[test]
fn every_validation_error_has_a_stable_code() {
    let source = r#"
OPENQASM 3.1;

qubit[2] q;
qubit[3] r;
cx q, r;
h q[2];
"#;

    let result = validate(source);

    assert!(result.is_invalid());

    for error in result.errors() {
        let code = error.code().as_str();

        assert!(
            code.starts_with("QASM-"),
            "validation error has unstable/non-QASM code: {}",
            code
        );

        assert!(
            !error.message().trim().is_empty(),
            "validation error message must never be empty"
        );
    }
}

#[test]
fn validation_errors_preserve_source_spans() {
    let source = r#"
OPENQASM 3.1;

qubit[2] q;
h q[2];
"#;

    let result = validate(source);

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::IndexOutOfBounds
        })
    );

    for error in result.errors() {
        let span = error.span();

        assert_eq!(
            span.source_id(),
            SourceId::from_raw(TEST_SOURCE_ID),
            "validation error must preserve the parser source identity"
        );

        assert!(
            span.start() <= span.end(),
            "validation spans must be half-open and ordered"
        );
    }
}

// ============================================================================
// No-I/O / no-execution semantic boundary
// ============================================================================

#[test]
fn include_validation_does_not_resolve_filesystem_paths() {
    let source = r#"
OPENQASM 3.1;

include "/etc/passwd";
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "arbitrary include must not be treated as an implicit filesystem operation"
    );

    assert!(
        result.errors().iter().any(|error| {
            matches!(
                error.code(),
                ValidationErrorCode::StandardLibraryUnavailable
                    | ValidationErrorCode::InvalidInclude
                    | ValidationErrorCode::DuplicateInclude
            )
        }) || result.errors().iter().any(|error| {
            error.message().contains("include")
        }),
        "validator should reject/flag arbitrary include without performing I/O:\n{:#?}",
        result.errors()
    );
}

#[test]
fn validation_of_extern_does_not_execute_extern() {
    let source = r#"
OPENQASM 3.1;

extern dangerous();
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "extern must be rejected by production policy"
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::ExternDisabled
        }),
        "expected extern-disabled policy error:\n{:#?}",
        result.errors()
    );
}

#[test]
fn validation_of_calibration_does_not_execute_calibration() {
    let source = r#"
OPENQASM 3.1;

cal {
}
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "calibration must be rejected by logical production policy"
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::CalibrationDisabled
        }),
        "expected calibration-disabled policy error:\n{:#?}",
        result.errors()
    );
}

// ============================================================================
// Regression tests for previously identified architectural hazards
// ============================================================================

#[test]
fn validator_uses_actual_program_version_not_maximum_supported_version() {
    let source = r#"
OPENQASM 3.0;

qubit q;
"#;

    let mut config = ValidationConfig::production();

    // The maximum supported version is deliberately newer than the program.
    config.max_minor_version = 1;

    let result = validate_with_config(source, config);

    assert!(
        result.is_valid(),
        "3.0 must remain valid when 3.1 is the maximum supported version:\n\
         errors:\n{:#?}",
        result.errors()
    );
}

#[test]
fn lowering_is_not_part_of_validation() {
    let source = r#"
OPENQASM 3.1;

qubit q;

reset q;
"#;

    let result = validate(source);

    assert!(
        result.is_valid(),
        "validation must judge OpenQASM semantics, not downstream lowering availability:\n\
         errors:\n{:#?}",
        result.errors()
    );
}

#[test]
fn validation_does_not_silently_discard_unsupported_features() {
    let source = r#"
OPENQASM 3.1;

cal {
}
"#;

    let result = validate(source);

    assert!(
        result.is_invalid(),
        "unsupported calibration must produce an explicit diagnostic"
    );

    assert!(
        result.errors().iter().any(|error| {
            error.code()
                == ValidationErrorCode::CalibrationDisabled
        }),
        "unsupported feature was silently discarded:\n{:#?}",
        result.errors()
    );
}

// ============================================================================
// Policy matrix
// ============================================================================

#[test]
fn production_policy_matrix_is_stable() {
    let config = ValidationConfig::production();

    let expected = [
        ("missing_version", !config.allow_missing_version),
        ("legacy_declarations", !config.allow_legacy_declarations),
        ("includes", config.allow_includes),
        ("extern", !config.allow_extern),
        ("calibration", !config.allow_calibration),
        ("timing", config.allow_timing),
        ("extensions", !config.allow_extensions),
        ("physical_qubits", !config.allow_physical_qubits),
        ("pragmas", config.allow_pragmas),
        ("annotations", config.allow_annotations),
        (
            "runtime_quantum_index",
            !config.allow_runtime_quantum_index,
        ),
    ];

    for (name, expected_value) in expected {
        assert!(
            expected_value,
            "production policy changed unexpectedly for `{}`",
            name
        );
    }
}

// ============================================================================
// Public API contract tests
// ============================================================================

#[test]
fn validate_program_defaults_to_production_policy() {
    let source = r#"
qubit q;
"#;

    let program = parse(source);

    let default_result =
        validate_program(
            &program,
            &FrontendLimits::production(),
        );

    let explicit_result =
        validate_program_with_config(
            &program,
            &FrontendLimits::production(),
            ValidationConfig::production(),
        );

    assert_eq!(
        default_result,
        explicit_result,
        "`validate_program` must remain the stable production-policy convenience API"
    );
}

#[test]
fn validation_result_success_contract_is_stable() {
    let result = validate(
        r#"
OPENQASM 3.1;
"#,
    );

    assert!(result.is_valid());
    assert!(!result.is_invalid());
    assert!(result.errors().is_empty());

    assert_eq!(
        result.clone().into_result(),
        Ok(())
    );
}

#[test]
fn validation_result_failure_contract_is_stable() {
    let result = validate(
        r#"
qubit q;
"#,
    );

    assert!(result.is_invalid());
    assert!(!result.is_valid());
    assert!(!result.errors().is_empty());

    let converted =
        result.clone().into_result();

    assert!(converted.is_err());

    let errors =
        converted.expect_err(
            "invalid ValidationResult must convert to Err",
        );

    assert!(!errors.is_empty());
}

// ============================================================================
// Error-code namespace regression tests
// ============================================================================

#[test]
fn semantic_error_codes_remain_in_expected_namespaces() {
    assert_eq!(
        ValidationErrorCode::MissingVersion.as_str(),
        "QASM-V002"
    );

    assert_eq!(
        ValidationErrorCode::DuplicateDeclaration.as_str(),
        "QASM-S001"
    );

    assert_eq!(
        ValidationErrorCode::IndexOutOfBounds.as_str(),
        "QASM-T004"
    );

    assert_eq!(
        ValidationErrorCode::OperandTypeMismatch.as_str(),
        "QASM-Q002"
    );

    assert_eq!(
        ValidationErrorCode::UndefinedGate.as_str(),
        "QASM-G010"
    );

    assert_eq!(
        ValidationErrorCode::DivisionByZero.as_str(),
        "QASM-E006"
    );

    assert_eq!(
        ValidationErrorCode::InvalidMeasurement.as_str(),
        "QASM-M001"
    );

    assert_eq!(
        ValidationErrorCode::IncludeDisabled.as_str(),
        "QASM-I001"
    );

    assert_eq!(
        ValidationErrorCode::ExternDisabled.as_str(),
        "QASM-U001"
    );

    assert_eq!(
        ValidationErrorCode::StatementLimitExceeded.as_str(),
        "QASM-L005"
    );
}

// ============================================================================
// Regression corpus: representative production programs
// ============================================================================

#[test]
fn bell_state_program_is_semantically_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
c = measure q;
"#,
    );
}

#[test]
fn parameterized_rotation_program_is_semantically_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit q;

rx(pi / 2) q;
ry(pi / 4) q;
rz(pi / 8) q;
"#,
    );
}

#[test]
fn multiple_register_program_is_semantically_valid() {
    assert_valid(
        r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[4] q;
bit[4] c;

h q[0];
cx q[0], q[1];
cx q[2], q[3];

c = measure q;
"#,
    );
}

#[test]
fn nested_control_flow_is_semantically_checked() {
    let source = r#"
OPENQASM 3.1;

bit flag;
qubit q;

for int i in [0:2] {
    if (flag) {
        reset q;
    }
}
"#;

    let result = validate(source);

    assert!(
        result.is_valid(),
        "nested valid control flow was rejected:\n{:#?}",
        result.errors()
    );
}

// ============================================================================
// Final production invariant
// ============================================================================

#[test]
fn production_validation_accepts_only_programs_that_survive_all_required_layers() {
    let valid_source = r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
c = measure q;
"#;

    let valid_result = validate(valid_source);

    assert!(
        valid_result.is_valid(),
        "production validation rejected the canonical supported circuit:\n{:#?}",
        valid_result.errors()
    );

    let invalid_source = r#"
OPENQASM 3.1;
include "stdgates.inc";

qubit[2] q;
qubit[3] r;
bit c;

cx q, r;
h c;
h q[2];
"#;

    let invalid_result = validate(invalid_source);

    assert!(
        invalid_result.is_invalid(),
        "production validation accepted semantically invalid circuit"
    );

    let codes = error_codes(&invalid_result);

    assert!(
        codes.contains(
            &ValidationErrorCode::RegisterBroadcastMismatch
        ),
        "missing broadcast validation"
    );

    assert!(
        codes.contains(
            &ValidationErrorCode::OperandTypeMismatch
        ),
        "missing quantum/classical type validation"
    );

    assert!(
        codes.contains(
            &ValidationErrorCode::IndexOutOfBounds
        ),
        "missing index validation"
    );
}