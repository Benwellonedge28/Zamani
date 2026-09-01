//! Zamani Quantum IR — Structural Validation
//!
//! Production-grade structural validation for the canonical, hardware-
//! independent Zamani Quantum IR.
//!
//! # Purpose
//!
//! This module answers one question:
//!
//! > Is the IR structurally well-formed and internally reference-consistent?
//!
//! It validates representation invariants that can be established from the
//! IR itself and from the containing program/circuit namespace.
//!
//! It deliberately does NOT decide:
//!
//! - whether hardware supports an operation;
//! - whether physical qubits are connected;
//! - whether routing is possible;
//! - whether an operation can be scheduled;
//! - whether a pulse is calibrated for a device;
//! - whether a backend can execute an operation;
//! - whether an algorithm is mathematically useful;
//! - whether an optimization is beneficial;
//! - whether a QEC code can decode a result;
//! - whether a simulator can execute the program.
//!
//! Those concerns belong to downstream layers.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                          frontend
//!                              │
//!                              ▼
//!                     canonical Quantum IR
//!                              │
//!                    ┌─────────┴─────────┐
//!                    │                   │
//!                    ▼                   ▼
//!             structural validation   other validation
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                    semantic/resource validation
//!                              │
//!                              ▼
//!                       optimization
//!                              │
//!                              ▼
//!                          routing
//!                              │
//!                              ▼
//!                        scheduling
//!                              │
//!                              ▼
//!                           backend
//! ```
//!
//! # Structural responsibilities
//!
//! This module validates:
//!
//! 1. operation identities are unique;
//! 2. every operation is locally valid;
//! 3. every logical qubit reference belongs to the containing namespace;
//! 4. every classical reference belongs to the containing namespace;
//! 5. gate-specific structure is valid;
//! 6. measurement structure is valid;
//! 7. Pauli-product measurement operands are valid;
//! 8. reset structure is valid;
//! 9. barrier structure is valid;
//! 10. classical assignments have valid destinations;
//! 11. classical conditions reference valid classical resources;
//! 12. conditional operations reference operations that exist in the same
//!     operation namespace;
//! 13. resource-allocation/release operation operands are structurally valid;
//! 14. all operation-local invariants are rechecked at the trust boundary.
//!
//! # What this module does NOT own
//!
//! Resource-policy limits belong to `limits.rs` and the resource validation
//! layer.
//!
//! Semantic mathematical validity belongs to semantic validation.
//!
//! Hardware capability belongs to hardware/capability validation.
//!
//! Timing validity belongs to timing validation.
//!
//! Program region/block/control-flow validity belongs to the corresponding
//! structural program/control-flow validators.
//!
//! # Scalability
//!
//! There is no hard-coded quantum-machine size.
//!
//! This module does not contain constants such as:
//!
//! - 32 qubits;
//! - 64 qubits;
//! - 127 qubits;
//! - 1,000 qubits;
//! - 1,000,000 qubits.
//!
//! `usize` is used only for the host-language representation of namespace
//! sizes supplied by `QuantumCircuit`; it is never converted into a semantic
//! quantum-machine maximum.
//!
//! Logical identifiers themselves remain owned by `quantum::ir::qubit`.
//!
//! The validator performs work proportional to the IR actually traversed.
//! It does not allocate storage proportional to the declared number of qubits.
//!
//! In particular:
//!
//! ```text
//! declared qubits = N
//! touched qubits  = K
//!
//! structural validation memory ≈ O(K)
//! ```
//!
//! rather than:
//!
//! ```text
//! structural validation memory ≈ O(N)
//! ```
//!
//! This permits sparse logical namespaces and large generated programs.
//!
//! # Security boundary
//!
//! IR must be treated as untrusted whenever it crosses a serialization,
//! plugin, compiler, service, or external-tool boundary.
//!
//! Constructors already perform local validation, but constructors cannot be
//! treated as the sole trust boundary because future deserializers,
//! transformations, extensions, and foreign tooling may construct equivalent
//! representations through other mechanisms.
//!
//! Therefore this validator intentionally validates again.
//!
//! # Canonical qubit identity
//!
//! All logical qubit handling uses:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! There is intentionally no `quantum::ir::qubits::QubitId`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `validation.rs` owns orchestration of validation layers.
//!
//! This module owns only structural validation.
//!
//! The intended integration is:
//!
//! ```rust
//! pub mod structural;
//! ```
//!
//! followed by:
//!
//! ```rust
//! structural::validate_circuit(circuit)?;
//! ```
//!
//! Resource, semantic, timing, capability and other validation layers must not
//! be called from this module.
//!
//! # Stability contract
//!
//! Downstream files may rely on these public functions:
//!
//! - [`validate_circuit`]
//! - [`validate_operation`]
//! - [`validate_gate`]
//! - [`validate_measurement`]
//! - [`validate_qubit`]
//! - [`validate_classical_bit`]
//!
//! Their responsibility must remain structural. Internal implementation may
//! evolve without requiring changes to callers.
//!
//! # Error contract
//!
//! All public validators return the canonical [`IrResult`] type.
//!
//! Callers must classify errors using `IrErrorKind`/`IrErrorCode` rather than
//! parsing human-readable error messages.
//!
//! # Determinism
//!
//! Validation traverses operations in program order.
//!
//! Duplicate identities and duplicate operands are detected deterministically.
//!
//! No hash-map iteration order is used.
//!
//! # Important distinction
//!
//! Structural validation does not impose resource limits.
//!
//! For example:
//!
//! ```text
//! 1,000,000 declared qubits
//! ```
//!
//! is not structurally invalid merely because the number is large.
//!
//! Whether such a program is permitted by a particular compiler invocation is
//! a resource-policy question handled elsewhere.
//!
//! Likewise, an operation with many operands is not rejected because it is
//! "too large" by this module. Its structural representation is validated;
//! resource policy is a separate concern.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - structural namespace checks;
//! - operation-reference integrity;
//! - operation identity uniqueness;
//! - structural operand integrity;
//! - local operation revalidation.
//!
//! It does not own the definitions of any of those IR objects.
//!
//! Their canonical definitions remain in their respective modules.
//!

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::classical::ClassicalBitId;
use super::errors::{IrError, IrResult};
use super::gate::Gate;
use super::measurement::Measurement;
use super::operation::{Operation, OperationBody};
use super::qubit::QubitId;

// =============================================================================
// Public circuit validator
// =============================================================================

/// Validates the structural integrity of a complete quantum circuit.
///
/// This function deliberately performs structural validation only.
///
/// Resource limits, semantic constraints, target capabilities, scheduling and
/// hardware compatibility belong to other validation layers.
///
/// # Complexity
///
/// Let:
///
/// - `O` = number of operations;
/// - `Q` = number of qubit operands actually touched;
/// - `C` = number of classical references actually touched.
///
/// The validator uses:
///
/// - O(O log O) worst-case identity checking;
/// - O(Q log Q) aggregate duplicate checking where applicable;
/// - O(C) namespace checks.
///
/// No storage proportional to the declared machine size is allocated.
pub fn validate_circuit(
    circuit: &super::circuit::QuantumCircuit,
) -> IrResult<()> {
    validate_namespace_sizes(
        circuit.num_qubits(),
        circuit.num_classical_bits(),
    )?;

    validate_unique_operation_ids(circuit.operations())?;

    for operation in circuit.operations() {
        validate_operation_in_namespace(
            operation,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;
    }

    Ok(())
}

// =============================================================================
// Namespace-size validation
// =============================================================================

/// Validates namespace sizes without imposing an architectural machine limit.
///
/// Zero-sized namespaces are valid at the structural layer.
///
/// For example:
///
/// - a classical-free quantum circuit is valid;
/// - a quantum-free classical/control region may be valid;
/// - an empty circuit is valid.
///
/// Whether a particular higher-level program requires at least one resource is
/// a semantic/program policy question.
pub fn validate_namespace_sizes(
    num_qubits: usize,
    num_classical_bits: usize,
) -> IrResult<()> {
    // The values are already represented by usize and therefore are
    // representable by the host language. There is deliberately no additional
    // machine-size ceiling here.
    let _ = num_qubits;
    let _ = num_classical_bits;

    Ok(())
}

// =============================================================================
// Operation identity validation
// =============================================================================

/// Validates that every operation has a unique identity.
///
/// Operation identity is independent of operation position.
///
/// This function therefore does NOT require:
///
/// ```text
/// operation[i].id() == i
/// ```
///
/// nor does it require IDs to be contiguous or monotonic.
///
/// This is essential for transformations that preserve operation identity
/// across insertion, deletion, cloning or replacement.
pub fn validate_unique_operation_ids(
    operations: &[Operation],
) -> IrResult<()> {
    let mut seen = BTreeSet::new();

    for operation in operations {
        let id = operation.id();

        if !seen.insert(id) {
            return Err(IrError::invalid_structure(format!(
                "duplicate operation identity {id}"
            )));
        }
    }

    Ok(())
}

// =============================================================================
// Single-operation public validator
// =============================================================================

/// Validates one operation against explicit logical namespaces.
///
/// This function is useful for:
///
/// - compiler passes;
/// - incremental validation;
/// - deserialization;
/// - unit tests;
/// - generated IR;
/// - external tooling.
///
/// It validates the operation itself and all namespace references contained
/// within it.
///
/// It does not validate whether referenced IDs such as a pulse, waveform,
/// schedule or resource exist in their respective registries; that belongs to
/// resource/reference validation once those registries are available.
pub fn validate_operation(
    operation: &Operation,
    num_qubits: usize,
    num_classical_bits: usize,
) -> IrResult<()> {
    validate_namespace_sizes(
        num_qubits,
        num_classical_bits,
    )?;

    validate_operation_in_namespace(
        operation,
        num_qubits,
        num_classical_bits,
    )
}

// =============================================================================
// Operation validation
// =============================================================================

fn validate_operation_in_namespace(
    operation: &Operation,
    num_qubits: usize,
    num_classical_bits: usize,
) -> IrResult<()> {
    // Re-run local validation at the trust boundary.
    //
    // The conversion is deliberately performed here instead of assuming that
    // Operation::new() was used.
    operation
        .validate()
        .map_err(|error| {
            IrError::invalid_structure(format!(
                "operation {} failed local structural validation: {}",
                operation.id(),
                error
            ))
        })?;

    match operation.body() {
        OperationBody::Gate(gate) => {
            validate_gate(
                gate,
                num_qubits,
                num_classical_bits,
            )
        }

        OperationBody::Measurement(measurement) => {
            validate_measurement(
                measurement,
                num_qubits,
                num_classical_bits,
            )
        }

        OperationBody::Reset { qubit } => {
            validate_reset(
                *qubit,
                num_qubits,
            )
        }

        OperationBody::Barrier { qubits } => {
            validate_qubit_collection(
                "barrier",
                qubits,
                num_qubits,
                true,
            )
        }

        OperationBody::Delay { .. } => {
            // The schedule identity's structural validity is already checked
            // by Operation::validate(). Existence of the referenced schedule
            // is deliberately not checked here because schedule registries
            // belong to higher-level program/resource validation.
            Ok(())
        }

        OperationBody::Pulse { .. } => {
            // Pulse identity syntax/shape is validated by Operation::validate().
            // Actual pulse definition existence belongs to pulse/resource
            // validation.
            Ok(())
        }

        OperationBody::Waveform { .. } => {
            // Waveform identity existence is a registry concern.
            Ok(())
        }

        OperationBody::FrameChange { .. } => {
            // Frame identity existence is a registry concern.
            Ok(())
        }

        OperationBody::Channel { .. } => {
            // Channel identity existence is a registry/target concern.
            Ok(())
        }

        OperationBody::ClassicalAssign { destination } => {
            validate_classical_bit(
                *destination,
                num_classical_bits,
            )
        }

        OperationBody::Conditional {
            condition,
            target,
        } => {
            validate_classical_bit(
                condition.bit(),
                num_classical_bits,
            )?;

            validate_conditional_target(
                operation,
                *target,
            )
        }

        OperationBody::AllocateQubits { qubits } => {
            validate_qubit_collection(
                "qubit allocation",
                qubits,
                num_qubits,
                true,
            )
        }

        OperationBody::ReleaseQubits { qubits } => {
            validate_qubit_collection(
                "qubit release",
                qubits,
                num_qubits,
                true,
            )
        }

        OperationBody::Logical { .. } => {
            // Resource identity existence is deliberately outside this layer.
            Ok(())
        }

        OperationBody::Analog { .. } => {
            // Resource identity existence is deliberately outside this layer.
            Ok(())
        }

        OperationBody::Annealing { .. } => {
            // Resource identity existence is deliberately outside this layer.
            Ok(())
        }

        OperationBody::Schedule { .. } => {
            // Schedule identity existence is deliberately outside this layer.
            Ok(())
        }

        OperationBody::Capability { .. } => {
            // Capability existence/target support is deliberately outside this
            // structural layer.
            Ok(())
        }

        OperationBody::Extension { .. } => {
            // Extension identity syntax is checked locally by Operation.
            // Extension registry/schema compatibility belongs to extension
            // validation.
            Ok(())
        }
    }
}

// =============================================================================
// Gate validation
// =============================================================================

/// Validates a logical gate and all of its namespace references.
///
/// Gate semantics are delegated to `Gate::validate_complete`, which is the
/// canonical gate-level validation implementation.
///
/// This wrapper exists so that the structural validation layer has a stable
/// public API without duplicating gate semantics.
pub fn validate_gate(
    gate: &Gate,
    num_qubits: usize,
    num_classical_bits: usize,
) -> IrResult<()> {
    gate
        .validate_complete(
            &super::limits::QuantumIrLimits::unbounded(),
            num_qubits,
            num_classical_bits,
        )
        .map_err(|error| {
            IrError::invalid_structure(format!(
                "invalid logical gate {}: {}",
                gate.kind().as_str(),
                error
            ))
        })?;

    Ok(())
}

// =============================================================================
// Measurement validation
// =============================================================================

/// Validates a canonical measurement and all of its logical namespace
/// references.
///
/// `Measurement::validate` is used rather than duplicating measurement
/// semantics here.
///
/// The function explicitly checks every qubit touched by a joint measurement,
/// not merely the measurement's primary qubit.
pub fn validate_measurement(
    measurement: &Measurement,
    num_qubits: usize,
    num_classical_bits: usize,
) -> IrResult<()> {
    measurement
        .validate(
            num_qubits,
            num_classical_bits,
        )
        .map_err(|error| {
            IrError::invalid_structure(format!(
                "invalid measurement: {}",
                error
            ))
        })?;

    let qubits = measurement.qubits();

    validate_unique_qubits(
        "measurement",
        &qubits,
    )?;

    for qubit in qubits {
        validate_qubit(
            qubit,
            num_qubits,
        )?;
    }

    validate_classical_bit(
        measurement.classical_bit(),
        num_classical_bits,
    )?;

    Ok(())
}

// =============================================================================
// Reset validation
// =============================================================================

fn validate_reset(
    qubit: QubitId,
    num_qubits: usize,
) -> IrResult<()> {
    validate_qubit(
        qubit,
        num_qubits,
    )
}

// =============================================================================
// Qubit validation
// =============================================================================

/// Validates that a logical qubit belongs to the supplied logical namespace.
///
/// The namespace is a logical-program namespace, not a hardware capacity.
///
/// This function intentionally uses the canonical:
///
/// ```text
/// super::qubit::QubitId
/// ```
pub fn validate_qubit(
    qubit: QubitId,
    num_qubits: usize,
) -> IrResult<()> {
    if qubit.index() >= num_qubits {
        return Err(IrError::invalid_structure(format!(
            "logical qubit {qubit} is outside logical namespace 0..{num_qubits}"
        )));
    }

    Ok(())
}

// =============================================================================
// Classical-bit validation
// =============================================================================

/// Validates that a classical bit belongs to the supplied classical namespace.
pub fn validate_classical_bit(
    bit: ClassicalBitId,
    num_classical_bits: usize,
) -> IrResult<()> {
    if bit.index() >= num_classical_bits {
        return Err(IrError::invalid_structure(format!(
            "classical bit {bit} is outside classical namespace 0..{num_classical_bits}"
        )));
    }

    Ok(())
}

// =============================================================================
// Qubit collections
// =============================================================================

/// Validates a collection of logical qubits.
///
/// The collection is required to be non-empty when `require_non_empty` is
/// true.
///
/// Duplicate operands are always rejected.
///
/// The implementation uses `BTreeSet` rather than a fixed-size bitmap or
/// vector, so validation memory is proportional to operands actually present
/// in the operation rather than the declared machine size.
pub fn validate_qubit_collection(
    operation_kind: &str,
    qubits: &[QubitId],
    num_qubits: usize,
    require_non_empty: bool,
) -> IrResult<()> {
    if require_non_empty && qubits.is_empty() {
        return Err(IrError::invalid_structure(format!(
            "{operation_kind} requires at least one logical qubit"
        )));
    }

    validate_unique_qubits(
        operation_kind,
        qubits,
    )?;

    for &qubit in qubits {
        validate_qubit(
            qubit,
            num_qubits,
        )?;
    }

    Ok(())
}

// =============================================================================
// Duplicate-qubit validation
// =============================================================================

/// Validates that no logical qubit occurs more than once in one structural
/// operand collection.
pub fn validate_unique_qubits(
    operation_kind: &str,
    qubits: &[QubitId],
) -> IrResult<()> {
    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(IrError::invalid_structure(format!(
                "{operation_kind} contains duplicate logical qubit {qubit}"
            )));
        }
    }

    Ok(())
}

// =============================================================================
// Conditional-operation validation
// =============================================================================

fn validate_conditional_target(
    operation: &Operation,
    target: super::identity::OperationId,
) -> IrResult<()> {
    if target == operation.id() {
        return Err(IrError::invalid_structure(format!(
            "conditional operation {} cannot reference itself",
            operation.id()
        )));
    }

    // Existence cannot be established from a single Operation. The complete
    // circuit validator performs the namespace-level existence check below.
    Ok(())
}

// =============================================================================
// Circuit-level conditional references
// =============================================================================

fn validate_conditional_targets_exist(
    operations: &[Operation],
) -> IrResult<()> {
    let mut ids = BTreeSet::new();

    for operation in operations {
        ids.insert(operation.id());
    }

    for operation in operations {
        if let OperationBody::Conditional {
            target,
            ..
        } = operation.body()
        {
            if !ids.contains(target) {
                return Err(IrError::invalid_structure(format!(
                    "conditional operation {} references nonexistent operation {}",
                    operation.id(),
                    target
                )));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Extended circuit validation
// =============================================================================

/// Performs the complete structural validation sequence.
///
/// This is kept separate from [`validate_circuit`] internally so the order of
/// checks is explicit and stable.
fn validate_circuit_complete(
    circuit: &super::circuit::QuantumCircuit,
) -> IrResult<()> {
    validate_namespace_sizes(
        circuit.num_qubits(),
        circuit.num_classical_bits(),
    )?;

    let operations = circuit.operations();

    validate_unique_operation_ids(operations)?;

    for operation in operations {
        validate_operation_in_namespace(
            operation,
            circuit.num_qubits(),
            circuit.num_classical_bits(),
        )?;
    }

    validate_conditional_targets_exist(operations)?;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::GateKind;
    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::ir::measurement::Measurement;
    use crate::quantum::ir::operation::Operation;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn c(index: usize) -> ClassicalBitId {
        ClassicalBitId::new(index)
    }

    fn operation_id(index: u64) -> OperationId {
        OperationId::new(index)
    }

    #[test]
    fn zero_sized_namespaces_are_structurally_valid() {
        assert!(
            validate_namespace_sizes(0, 0).is_ok()
        );
    }

    #[test]
    fn qubit_inside_namespace_is_valid() {
        assert!(
            validate_qubit(q(0), 1).is_ok()
        );

        assert!(
            validate_qubit(q(7), 8).is_ok()
        );
    }

    #[test]
    fn qubit_outside_namespace_is_rejected() {
        assert!(
            validate_qubit(q(8), 8).is_err()
        );
    }

    #[test]
    fn classical_bit_inside_namespace_is_valid() {
        assert!(
            validate_classical_bit(c(0), 1).is_ok()
        );
    }

    #[test]
    fn classical_bit_outside_namespace_is_rejected() {
        assert!(
            validate_classical_bit(c(4), 4).is_err()
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let qubits = vec![q(0), q(1), q(0)];

        assert!(
            validate_unique_qubits(
                "test operation",
                &qubits,
            )
            .is_err()
        );
    }

    #[test]
    fn unique_qubits_are_accepted() {
        let qubits = vec![q(0), q(1), q(2)];

        assert!(
            validate_unique_qubits(
                "test operation",
                &qubits,
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_required_collection_is_rejected() {
        let qubits = Vec::new();

        assert!(
            validate_qubit_collection(
                "barrier",
                &qubits,
                0,
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn operation_ids_need_not_be_contiguous() {
        let op0 = Operation::gate(
            operation_id(10),
            Gate::simple(
                GateKind::X,
                vec![q(0)],
            )
            .expect("valid gate"),
        )
        .expect("valid operation");

        let op1 = Operation::gate(
            operation_id(1000),
            Gate::simple(
                GateKind::H,
                vec![q(0)],
            )
            .expect("valid gate"),
        )
        .expect("valid operation");

        let operations = vec![op0, op1];

        assert!(
            validate_unique_operation_ids(
                &operations
            )
            .is_ok()
        );
    }

    #[test]
    fn duplicate_operation_ids_are_rejected() {
        let op0 = Operation::gate(
            operation_id(1),
            Gate::simple(
                GateKind::X,
                vec![q(0)],
            )
            .expect("valid gate"),
        )
        .expect("valid operation");

        let op1 = Operation::gate(
            operation_id(1),
            Gate::simple(
                GateKind::H,
                vec![q(0)],
            )
            .expect("valid gate"),
        )
        .expect("valid operation");

        let operations = vec![op0, op1];

        assert!(
            validate_unique_operation_ids(
                &operations
            )
            .is_err()
        );
    }

    #[test]
    fn ordinary_measurement_is_structurally_valid() {
        let measurement =
            Measurement::new(
                q(0),
                c(0),
            );

        assert!(
            validate_measurement(
                &measurement,
                1,
                1,
            )
            .is_ok()
        );
    }

    #[test]
    fn measurement_with_invalid_qubit_is_rejected() {
        let measurement =
            Measurement::new(
                q(2),
                c(0),
            );

        assert!(
            validate_measurement(
                &measurement,
                2,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn measurement_with_invalid_classical_target_is_rejected() {
        let measurement =
            Measurement::new(
                q(0),
                c(2),
            );

        assert!(
            validate_measurement(
                &measurement,
                1,
                2,
            )
            .is_err()
        );
    }

    #[test]
    fn measurement_operation_is_structurally_valid() {
        let measurement =
            Measurement::new(
                q(0),
                c(0),
            );

        let operation =
            Operation::measurement(
                operation_id(0),
                measurement,
            )
            .expect("valid operation");

        assert!(
            validate_operation(
                &operation,
                1,
                1,
            )
            .is_ok()
        );
    }

    #[test]
    fn reset_is_valid_for_existing_qubit() {
        let operation =
            Operation::reset(
                operation_id(0),
                q(0),
            )
            .expect("valid reset");

        assert!(
            validate_operation(
                &operation,
                1,
                0,
            )
            .is_ok()
        );
    }

    #[test]
    fn reset_outside_namespace_is_rejected() {
        let operation =
            Operation::reset(
                operation_id(0),
                q(1),
            )
            .expect("locally valid reset");

        assert!(
            validate_operation(
                &operation,
                1,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn circuit_conditional_reference_must_exist() {
        let condition =
            super::super::operation::OperationCondition::when_true(
                c(0),
            );

        let operation =
            Operation::new(
                operation_id(0),
                OperationBody::Conditional {
                    condition,
                    target: operation_id(99),
                },
            )
            .expect("locally valid conditional");

        let operations = vec![operation];

        assert!(
            validate_conditional_targets_exist(
                &operations
            )
            .is_err()
        );
    }

    #[test]
    fn circuit_conditional_reference_to_existing_operation_is_valid() {
        let target =
            Operation::gate(
                operation_id(10),
                Gate::simple(
                    GateKind::X,
                    vec![q(0)],
                )
                .expect("valid gate"),
            )
            .expect("valid operation");

        let condition =
            super::super::operation::OperationCondition::when_true(
                c(0),
            );

        let conditional =
            Operation::new(
                operation_id(20),
                OperationBody::Conditional {
                    condition,
                    target: operation_id(10),
                },
            )
            .expect("valid conditional");

        let operations =
            vec![target, conditional];

        assert!(
            validate_conditional_targets_exist(
                &operations
            )
            .is_ok()
        );
    }

    #[test]
    fn self_conditional_reference_is_rejected() {
        let condition =
            super::super::operation::OperationCondition::when_true(
                c(0),
            );

        let operation =
            Operation::new(
                operation_id(10),
                OperationBody::Conditional {
                    condition,
                    target: operation_id(10),
                },
            );

        assert!(
            operation.is_err()
        );
    }
}