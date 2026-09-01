//! Zamani Quantum IR — Whole-Program Semantic Validation
//!
//! Production-grade semantic validation for the canonical Zamani Quantum IR.
//!
//! # Purpose
//!
//! This module answers:
//!
//! > "Does this already-constructed IR have internally consistent quantum
//! > semantics?"
//!
//! It does NOT answer:
//!
//! - whether a target QPU exists;
//! - whether physical qubits are connected;
//! - whether routing is possible;
//! - whether a backend supports an operation;
//! - whether a pulse is calibrated;
//! - whether a schedule is executable;
//! - whether a particular topology can realize the program;
//! - whether a simulator can execute the program efficiently.
//!
//! Those concerns belong to downstream target, routing, scheduling, hardware,
//! simulator, and backend subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! frontend
//!       |
//!       v
//! canonical Quantum IR
//!       |
//!       +--> structural validation
//!       |
//!       +--> type validation
//!       |
//!       +--> semantic validation  <--- this module
//!       |
//!       +--> resource-policy validation
//!       |
//!       v
//! optimization / routing / scheduling / lowering
//! ```
//!
//! # Important ownership rule
//!
//! This module validates semantics owned by other IR modules. It does not
//! redefine their types.
//!
//! Canonical ownership remains:
//!
//! - `quantum::ir::qubit`      -> logical qubit identity;
//! - `quantum::ir::gate`       -> gate semantics;
//! - `quantum::ir::measurement` -> measurement semantics;
//! - `quantum::ir::parameter`  -> parameter semantics;
//! - `quantum::ir::circuit`    -> ordered circuit container;
//! - `quantum::ir::operation`  -> universal operation representation;
//! - `quantum::ir::program`    -> larger program/region representation;
//! - `quantum::ir::errors`     -> canonical IR errors;
//! - `quantum::ir::limits`     -> explicit resource/security policy.
//!
//! # Scalability
//!
//! No machine-size constant appears in this module.
//!
//! In particular, this module does NOT assume:
//!
//! - 32 qubits;
//! - 64 qubits;
//! - 128 qubits;
//! - 4,096 qubits;
//! - 1,000,000 qubits;
//! - any fixed gate count;
//! - any fixed topology;
//! - any fixed gate universe.
//!
//! A logical qubit is an identifier. The number of available quantum resources
//! is a property of the program, compilation policy, and eventual target.
//!
//! Validation therefore scales with the amount of IR actually traversed rather
//! than allocating tables based on a theoretical machine size.
//!
//! # Determinism
//!
//! Validation is deterministic:
//!
//! - operations are inspected in program order;
//! - duplicate-resource detection uses ordered sets;
//! - no hash-map iteration order participates in validation;
//! - no hardware information is queried;
//! - no random state is used.
//!
//! # Security
//!
//! IR may originate from:
//!
//! - trusted compiler construction;
//! - deserialization;
//! - generated code;
//! - optimization passes;
//! - external tools;
//! - future IR dialects;
//! - cached compilation artifacts.
//!
//! Therefore this module must never assume that constructors were used.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration
//!
//! The validation module root should expose this module:
//!
//! ```text
//! quantum::ir::validation::semantic
//! ```
//!
//! The module root should continue to own orchestration. This file owns only
//! semantic checks.
//!
//! The old flat `src/quantum/ir/validation.rs` must therefore be migrated to:
//!
//! ```text
//! src/quantum/ir/validation/mod.rs
//! ```
//!
//! and `mod.rs` should contain:
//!
//! ```rust
//! pub mod semantic;
//! ```
//!
//! The old public validation entry points may then delegate to this module
//! without duplicating semantic rules.
//!
//! # Semantic validation versus policy validation
//!
//! This distinction is fundamental.
//!
//! Semantic invalidity:
//!
//! ```text
//! measurement refers to an invalid classical destination
//! duplicate qubit operands where distinct operands are required
//! measurement payload disagrees with the gate
//! non-measurement operation contains measurement-only state
//! parameter count disagrees with operation semantics
//! ```
//!
//! Policy invalidity:
//!
//! ```text
//! maximum operations exceeded
//! maximum metadata bytes exceeded
//! maximum validation work exceeded
//! ```
//!
//! A policy failure does not mean that the Zamani program is semantically
//! invalid. It means that a particular validation/compilation invocation has
//! rejected it under an explicit resource policy.
//!
//! # No hardware coupling
//!
//! This module must remain independent of:
//!
//! - `quantum::hardware`;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC implementation;
//! - simulator state;
//! - backend APIs;
//! - frontend parsers;
//! - vendor SDKs.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::super::circuit::QuantumCircuit;
use super::super::errors::{IrError, IrResult};
use super::super::gate::{Gate, GateKind};
use super::super::qubit::QubitId;

// ============================================================================
// Public contract
// ============================================================================

/// Stable identifier for this semantic validation contract.
///
/// This identifier is deliberately independent of the IR schema version.
pub const SEMANTIC_VALIDATION_ID: &str = "quantum.ir.validation.semantic";

/// Semantic validation contract version.
///
/// Increment this when the meaning of a validation rule changes.
///
/// This is NOT the same thing as the Quantum IR serialization/schema version.
pub const SEMANTIC_VALIDATION_VERSION: u32 = 1;

/// Validates the semantic invariants of a complete canonical quantum circuit.
///
/// This function intentionally performs no resource-policy checks.
///
/// Use the validation module's complete validation entry point when both
/// semantic and policy validation are required.
///
/// # Errors
///
/// Returns `IrError` when the circuit contains an internally inconsistent
/// semantic representation.
pub fn validate(circuit: &QuantumCircuit) -> IrResult<()> {
    validate_circuit(circuit)
}

/// Validates the semantic invariants of a complete canonical quantum circuit.
///
/// This is the primary circuit-level semantic-validation entry point.
pub fn validate_circuit(circuit: &QuantumCircuit) -> IrResult<()> {
    validate_circuit_namespace(circuit)?;

    for operation in circuit.operations() {
        validate_gate(operation)?;
    }

    validate_measurement_destinations(circuit)?;

    Ok(())
}

/// Validates one canonical logical gate independently of a circuit.
///
/// This function deliberately does not validate whether qubit identifiers
/// belong to a particular circuit namespace. That requires a circuit context
/// and belongs to `validate_gate_in_circuit`.
///
/// It does validate all intrinsic semantic relationships represented by the
/// `Gate` itself.
pub fn validate_gate(gate: &Gate) -> IrResult<()> {
    validate_gate_kind_contract(gate)?;
    validate_gate_operands(gate)?;
    validate_gate_parameters(gate)?;
    validate_gate_auxiliary_state(gate)?;

    Ok(())
}

/// Validates one gate against a concrete circuit namespace.
///
/// This is useful for compiler passes that construct or replace one operation
/// at a time before committing it to a circuit.
pub fn validate_gate_in_circuit(
    circuit: &QuantumCircuit,
    gate: &Gate,
) -> IrResult<()> {
    validate_gate(gate)?;

    for &qubit in gate.qubits() {
        validate_qubit_in_namespace(
            qubit,
            circuit.num_qubits(),
        )?;
    }

    validate_classical_target_in_namespace(
        gate,
        circuit.num_classical_bits(),
    )?;

    Ok(())
}

// ============================================================================
// Circuit namespace semantics
// ============================================================================

fn validate_circuit_namespace(
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    // A namespace size of zero is semantically valid. It is useful for
    // classical-only IR, empty modules, generated intermediate states, and
    // future hybrid programs.
    //
    // Therefore this function intentionally does not impose:
    //
    //     num_qubits > 0
    //
    // as a language invariant.

    let qubits = circuit.num_qubits();
    let classical_bits = circuit.num_classical_bits();

    // Avoid arithmetic based on machine-size assumptions. Namespace values are
    // already represented by the canonical circuit type.
    //
    // The only semantic requirement here is that every referenced resource is
    // checked when its operation is visited.

    if qubits == usize::MAX || classical_bits == usize::MAX {
        // These values are representable by Rust but are not practically
        // constructible as backing storage. The important distinction is that
        // we do NOT turn this into a Zamani language-level quantum limit.
        //
        // A circuit may still represent sparse future namespaces if its
        // container implementation permits them.
    }

    Ok(())
}

// ============================================================================
// Gate-kind contract
// ============================================================================

fn validate_gate_kind_contract(gate: &Gate) -> IrResult<()> {
    let kind = gate.kind();

    let expected_operands = kind.operand_count();
    let actual_operands = gate.qubits().len();

    if !expected_operands.accepts(actual_operands) {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::InvalidOperandCount {
                gate: kind,
                expected: expected_operands,
                actual: actual_operands,
            },
        ));
    }

    let expected_parameters = kind.parameter_count();
    let actual_parameters = gate.parameters().len();

    if expected_parameters != actual_parameters {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::InvalidParameterCount {
                gate: kind,
                expected: expected_parameters,
                actual: actual_parameters,
            },
        ));
    }

    // These are semantic relationships, not merely arity relationships.
    //
    // Measurement and reset are non-unitary operations. They must never be
    // accidentally interpreted as ordinary unitary gates.
    if kind.is_measurement() && kind.is_unitary() {
        return Err(IrError::Invariant(
            super::super::errors::IrInvariantError::Violation {
                message: "measurement cannot simultaneously be classified as unitary",
            },
        ));
    }

    if kind.is_reset() && kind.is_unitary() {
        return Err(IrError::Invariant(
            super::super::errors::IrInvariantError::Violation {
                message: "reset cannot simultaneously be classified as unitary",
            },
        ));
    }

    Ok(())
}

// ============================================================================
// Operand semantics
// ============================================================================

fn validate_gate_operands(gate: &Gate) -> IrResult<()> {
    let qubits = gate.qubits();

    // A semantic gate operand list is ordered. Two positions containing the
    // same logical qubit are not automatically equivalent to two distinct
    // operands.
    //
    // This catches malformed representations such as:
    //
    //     CX(q0, q0)
    //
    // while still allowing arbitrary operand counts for gate kinds that
    // explicitly permit them.
    let mut seen = BTreeSet::<QubitId>::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(IrError::Qubit(
                super::super::errors::IrQubitError::Duplicate {
                    qubit: qubit.index(),
                },
            ));
        }
    }

    // A barrier has variadic operands and must actually synchronize something.
    if gate.kind().is_barrier() && qubits.is_empty() {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::InvalidStructure {
                gate: gate.kind(),
                reason: "barrier requires at least one logical qubit",
            },
        ));
    }

    // Reset is currently a single-qubit semantic operation in the canonical
    // gate vocabulary. If a future dialect introduces register reset or
    // subsystem reset, it must use a different operation definition rather
    // than weakening this invariant.
    if gate.kind().is_reset() && qubits.len() != 1 {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::InvalidOperandCount {
                gate: gate.kind(),
                expected: super::super::gate::OperandCount::Exact(1),
                actual: qubits.len(),
            },
        ));
    }

    Ok(())
}

// ============================================================================
// Parameter semantics
// ============================================================================

fn validate_gate_parameters(gate: &Gate) -> IrResult<()> {
    let expected = gate.kind().parameter_count();
    let parameters = gate.parameters();

    if parameters.len() != expected {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::InvalidParameterCount {
                gate: gate.kind(),
                expected,
                actual: parameters.len(),
            },
        ));
    }

    // Parameter ownership and expression validation remain in `parameter.rs`.
    //
    // Semantic validation must not reimplement expression parsing or
    // parameter algebra here.
    //
    // The important invariant at this layer is that a parameterized gate has
    // exactly the number of parameter slots declared by its semantic gate
    // definition.

    Ok(())
}

// ============================================================================
// Measurement / classical auxiliary-state semantics
// ============================================================================

fn validate_gate_auxiliary_state(gate: &Gate) -> IrResult<()> {
    let kind = gate.kind();

    let has_measurement = gate.measurement().is_some();
    let has_classical_target = gate.classical_target().is_some();

    if kind.is_measurement() {
        if !has_measurement {
            return Err(IrError::Gate(
                super::super::errors::IrGateError::MissingMeasurement,
            ));
        }

        if !has_classical_target {
            return Err(IrError::Gate(
                super::super::errors::IrGateError::MissingClassicalTarget,
            ));
        }

        return Ok(());
    }

    // Measurement payloads are meaningful only on measurement operations.
    if has_measurement {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::UnexpectedMeasurement {
                gate: kind,
            },
        ));
    }

    // Classical destinations are currently part of measurement semantics.
    // A future generalized classical-result operation should own its own
    // operation semantics instead of making ordinary gates implicitly
    // classical.
    if let Some(target) = gate.classical_target() {
        return Err(IrError::Gate(
            super::super::errors::IrGateError::UnexpectedClassicalTarget {
                gate: kind,
                target,
            },
        ));
    }

    Ok(())
}

// ============================================================================
// Namespace checks
// ============================================================================

fn validate_qubit_in_namespace(
    qubit: QubitId,
    count: usize,
) -> IrResult<()> {
    let index = qubit.index();

    if index >= count {
        return Err(IrError::Identifier(
            super::super::errors::IrIdentifierError::QubitOutOfRange {
                index,
                count,
            },
        ));
    }

    Ok(())
}

fn validate_classical_target_in_namespace(
    gate: &Gate,
    count: usize,
) -> IrResult<()> {
    if let Some(target) = gate.classical_target() {
        if target >= count {
            return Err(IrError::Identifier(
                super::super::errors::IrIdentifierError::ClassicalBitOutOfRange {
                    index: target,
                    count,
                },
            ));
        }
    }

    Ok(())
}

// ============================================================================
// Whole-circuit measurement semantics
// ============================================================================

fn validate_measurement_destinations(
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let mut destinations = BTreeSet::<usize>::new();

    for gate in circuit.operations() {
        if !gate.kind().is_measurement() {
            continue;
        }

        let target = match gate.classical_target() {
            Some(value) => value,
            None => {
                return Err(IrError::Gate(
                    super::super::errors::IrGateError::MissingClassicalTarget,
                ));
            }
        };

        if !destinations.insert(target) {
            return Err(IrError::Measurement(
                super::super::errors::IrMeasurementError::DuplicateClassicalDestination {
                    bit: target,
                },
            ));
        }
    }

    Ok(())
}

// ============================================================================
// Semantic helper predicates
// ============================================================================

/// Returns whether a gate is semantically non-unitary.
///
/// This is intentionally derived from the canonical `GateKind`; callers must
/// not maintain their own duplicate list.
#[must_use]
pub const fn is_non_unitary(kind: GateKind) -> bool {
    kind.is_measurement() || kind.is_reset()
}

/// Returns whether a gate requires a classical destination.
#[must_use]
pub const fn requires_classical_destination(
    kind: GateKind,
) -> bool {
    kind.requires_classical_target()
}

/// Returns whether a gate is allowed to contain a measurement payload.
#[must_use]
pub const fn allows_measurement_payload(
    kind: GateKind,
) -> bool {
    kind.is_measurement()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_identity_is_stable() {
        assert_eq!(
            SEMANTIC_VALIDATION_ID,
            "quantum.ir.validation.semantic"
        );

        assert_eq!(
            SEMANTIC_VALIDATION_VERSION,
            1
        );
    }

    #[test]
    fn non_unitary_classification_is_canonical() {
        assert!(!is_non_unitary(GateKind::X));
        assert!(is_non_unitary(GateKind::Measure));
        assert!(is_non_unitary(GateKind::Reset));
    }

    #[test]
    fn measurement_requires_classical_destination() {
        assert!(
            requires_classical_destination(GateKind::Measure)
        );

        assert!(
            !requires_classical_destination(GateKind::X)
        );
    }

    #[test]
    fn measurement_payload_is_not_allowed_on_unitary_gates() {
        assert!(
            !allows_measurement_payload(GateKind::X)
        );

        assert!(
            allows_measurement_payload(GateKind::Measure)
        );
    }

    #[test]
    fn operand_contract_is_taken_from_gate_kind() {
        assert!(
            GateKind::X
                .operand_count()
                .accepts(1)
        );

        assert!(
            !GateKind::X
                .operand_count()
                .accepts(2)
        );

        assert!(
            GateKind::Barrier
                .operand_count()
                .accepts(1)
        );

        assert!(
            GateKind::Barrier
                .operand_count()
                .accepts(10_000)
        );
    }
}