//! Zamani Quantum Intermediate Representation — Canonical Validation
//!
//! Production-grade validation for the hardware-independent Zamani Quantum IR.
//!
//! # Architectural boundary
//!
//! This module validates the semantic representation of quantum programs.
//!
//! It answers:
//!
//! > Is this IR structurally, semantically, and resource-policy valid?
//!
//! It deliberately does NOT answer:
//!
//! - whether a physical qubit exists;
//! - whether physical qubits are connected;
//! - whether routing is possible;
//! - which native instruction implements an operation;
//! - which pulse implements a gate;
//! - whether a calibration is valid;
//! - whether a particular QPU supports an operation;
//! - how an operation is scheduled;
//! - how a backend communicates with hardware;
//! - how error correction is decoded;
//! - how a simulator executes the circuit.
//!
//! Those concerns belong to downstream subsystems.
//!
//! # Validation layers
//!
//! Validation is performed in deterministic layers:
//!
//! 1. Policy validation
//!    - validate the supplied `QuantumIrLimits` configuration.
//!
//! 2. Namespace validation
//!    - logical qubit namespace;
//!    - logical classical namespace.
//!
//! 3. Structural validation
//!    - operation arity;
//!    - duplicate logical operands;
//!    - parameter arity;
//!    - parameter validity;
//!    - measurement representation;
//!    - classical destinations.
//!
//! 4. Resource validation
//!    - qubits;
//!    - classical bits;
//!    - operations;
//!    - operands;
//!    - parameters;
//!    - measurements;
//!    - barriers;
//!    - validation work;
//!    - circuit depth;
//!    - metadata;
//!
//! 5. Whole-program semantic validation
//!    - measurement destination uniqueness;
//!    - measurement payload consistency;
//!    - deterministic invariants.
//!
//! # Scalability
//!
//! Zamani has no architectural fixed quantum-machine size.
//!
//! The validator therefore never treats:
//!
//! - 63;
//! - 64;
//! - 4096;
//! - 1_000_000;
//!
//! as a language-level quantum limit.
//!
//! Such numbers may occur as explicit deployment policy values, but they are
//! never interpreted as the maximum number of qubits Zamani can represent.
//!
//! `QuantumIrLimits::unbounded()` permits trusted workloads to use the full
//! representable identifier/resource domain, subject to the actual process,
//! operating-system, compiler, allocator, and target resources.
//!
//! # Important scalability property
//!
//! Depth validation uses a sparse map keyed by `QubitId` rather than allocating
//! a vector with one entry for every declared logical qubit.
//!
//! Therefore:
//!
//! ```text
//! declared qubits = extremely large
//! touched qubits  = small
//!
//! validation memory ≈ touched qubits
//! ```
//!
//! rather than:
//!
//! ```text
//! validation memory ≈ declared qubits
//! ```
//!
//! This is important for distributed, sparse, generated, logical, and future
//! large-scale quantum programs.
//!
//! # Untrusted IR
//!
//! Constructors in `gate.rs`, `measurement.rs`, and `circuit.rs` already
//! perform local validation. This module must nevertheless validate again
//! because IR can eventually originate from:
//!
//! - deserialization;
//! - generated IR;
//! - compiler transformations;
//! - optimization passes;
//! - replay;
//! - external tooling;
//! - future serialization formats.
//!
//! Validation must therefore never assume that a caller previously used a
//! safe constructor.
//!
//! # Hardware independence
//!
//! This module MUST remain independent of:
//!
//! - `quantum::hardware`;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC implementation;
//! - simulator execution;
//! - frontend parsing;
//! - backend communication.
//!
//! It consumes canonical semantic IR only.
//!
//! # Canonical qubit identity
//!
//! All logical qubit references use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! There is intentionally no `super::qubits` module.
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
//! - no `unsafe`;
//! - no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::circuit::QuantumCircuit;
use super::errors::{
    IrError,
    IrGateError,
    IrIdentifierError,
    IrMeasurementError,
    IrParameterError,
    IrResult,
};
use super::gate::{Gate, GateKind};
use super::limits::QuantumIrLimits;
use super::measurement::{
    Measurement,
    MeasurementGroup,
};
use super::qubit::QubitId;

// =============================================================================
// Validation configuration
// =============================================================================

/// Controls how canonical IR validation is performed.
///
/// The configuration is immutable for the duration of one validation pass.
/// This makes validation deterministic and prevents one part of validation
/// from silently using a different policy than another part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Explicit resource/security policy.
    pub limits: QuantumIrLimits,

    /// Enables strict validation semantics.
    ///
    /// Strict validation is appropriate at trust boundaries:
    ///
    /// - deserialization;
    /// - compiler boundaries;
    /// - external IR ingestion;
    /// - replay;
    /// - release builds.
    pub strict: bool,

    /// Whether a circuit containing zero operations is valid.
    pub allow_empty_circuit: bool,

    /// Whether whole-circuit semantic checks are enabled.
    pub semantic_checks: bool,
}

impl ValidationConfig {
    /// Creates a production-oriented validation configuration.
    pub const fn new(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Returns the standard production policy.
    pub fn production() -> Self {
        Self::new(QuantumIrLimits::production())
    }

    /// Returns a strict configuration using explicit limits.
    pub const fn strict(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Returns an explicitly permissive semantic configuration.
    ///
    /// Resource limits and structural validation remain active.
    /// Only optional whole-program semantic checks are disabled.
    pub const fn permissive(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: false,
            allow_empty_circuit: true,
            semantic_checks: false,
        }
    }

    /// Enables or disables strict mode.
    pub const fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Enables or disables empty circuits.
    pub const fn with_empty_circuits(mut self, allow: bool) -> Self {
        self.allow_empty_circuit = allow;
        self
    }

    /// Enables or disables whole-circuit semantic validation.
    pub const fn with_semantic_checks(mut self, enabled: bool) -> Self {
        self.semantic_checks = enabled;
        self
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Validation context
// =============================================================================

/// Immutable context shared by all validation operations in one pass.
#[derive(Debug, Clone, Copy)]
struct ValidationContext<'a> {
    config: &'a ValidationConfig,
    num_qubits: usize,
    num_classical_bits: usize,
}

impl<'a> ValidationContext<'a> {
    const fn new(
        config: &'a ValidationConfig,
        num_qubits: usize,
        num_classical_bits: usize,
    ) -> Self {
        Self {
            config,
            num_qubits,
            num_classical_bits,
        }
    }

    fn limits(&self) -> &QuantumIrLimits {
        &self.config.limits
    }
}

// =============================================================================
// Public circuit validation API
// =============================================================================

/// Validates a complete circuit using the standard production policy.
pub fn validate_circuit(circuit: &QuantumCircuit) -> IrResult<()> {
    validate_circuit_with_config(circuit, &ValidationConfig::production())
}

/// Validates a complete circuit against explicit resource limits.
pub fn validate_circuit_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    let config = ValidationConfig::new(*limits);
    validate_circuit_with_config(circuit, &config)
}

/// Validates a complete circuit against an explicit configuration.
///
/// This is the canonical entry point for untrusted or externally reconstructed
/// IR.
pub fn validate_circuit_with_config(
    circuit: &QuantumCircuit,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_limits(&config.limits)?;

    let context = ValidationContext::new(
        config,
        circuit.num_qubits(),
        circuit.num_classical_bits(),
    );

    validate_namespace_limits(&context)?;
    validate_empty_circuit_policy(&context, circuit)?;

    validate_circuit_structure(&context, circuit)?;
    validate_circuit_resources(&context, circuit)?;

    if config.semantic_checks {
        validate_circuit_semantics(&context, circuit)?;
    }

    Ok(())
}

// =============================================================================
// Namespace validation
// =============================================================================

fn validate_namespace_limits(
    context: &ValidationContext<'_>,
) -> IrResult<()> {
    context
        .limits()
        .check_qubits(context.num_qubits)
        .map_err(IrError::from)?;

    context
        .limits()
        .check_classical_bits(context.num_classical_bits)
        .map_err(IrError::from)?;

    Ok(())
}

fn validate_empty_circuit_policy(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    if !context.config.allow_empty_circuit && circuit.is_empty() {
        return Err(IrError::InvalidStructure {
            message: "empty circuits are not permitted by the validation configuration",
        });
    }

    Ok(())
}

// =============================================================================
// Circuit structural validation
// =============================================================================

fn validate_circuit_structure(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let operations = circuit.operations();

    validate_operation_count(
        context,
        operations.len(),
    )?;

    for (operation_index, gate) in operations.iter().enumerate() {
        validate_operation_at(
            context,
            gate,
            operation_index,
        )?;
    }

    Ok(())
}

// =============================================================================
// Operation count
// =============================================================================

fn validate_operation_count(
    context: &ValidationContext<'_>,
    count: usize,
) -> IrResult<()> {
    context
        .limits()
        .check_operations(count)
        .map_err(IrError::from)
}

// =============================================================================
// Operation validation
// =============================================================================

/// Validates one logical operation against an explicit context.
pub fn validate_operation(
    gate: &Gate,
    num_qubits: usize,
    num_classical_bits: usize,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_limits(&config.limits)?;

    let context = ValidationContext::new(
        config,
        num_qubits,
        num_classical_bits,
    );

    validate_namespace_limits(&context)?;

    validate_operation_at(
        &context,
        gate,
        0,
    )
}

/// Alias for callers that explicitly deal with gate semantics.
pub fn validate_gate(
    gate: &Gate,
    num_qubits: usize,
    num_classical_bits: usize,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_operation(
        gate,
        num_qubits,
        num_classical_bits,
        config,
    )
}

fn validate_operation_at(
    context: &ValidationContext<'_>,
    gate: &Gate,
    operation_index: usize,
) -> IrResult<()> {
    consume_validation_work(
        context,
        operation_index,
        1,
    )?;

    validate_gate_structure(gate)?;

    validate_gate_operands(
        context,
        gate,
    )?;

    validate_gate_parameters(
        context,
        gate,
    )?;

    validate_classical_target(
        context,
        gate,
    )?;

    validate_measurement_shape(gate)?;

    if context.config.semantic_checks {
        validate_gate_semantics(
            context,
            gate,
        )?;
    }

    Ok(())
}

// =============================================================================
// Gate structure
// =============================================================================

fn validate_gate_structure(gate: &Gate) -> IrResult<()> {
    let expected = gate.kind().operand_count();
    let actual = gate.qubit_count();

    if !expected.accepts(actual) {
        if gate.kind().is_barrier() {
            return Err(IrError::Gate(
                IrGateError::InvalidStructure {
                    gate: gate.kind().as_str(),
                    reason: "a barrier must contain at least one logical qubit",
                },
            ));
        }

        if gate.kind().is_reset() {
            return Err(IrError::Gate(
                IrGateError::InvalidStructure {
                    gate: gate.kind().as_str(),
                    reason: "reset requires exactly one logical qubit",
                },
            ));
        }

        return Err(IrError::Gate(
            IrGateError::InvalidQubitCount {
                gate: gate.kind().as_str(),
                expected: expected.exact_value(),
                actual,
            },
        ));
    }

    validate_duplicate_qubits(gate)?;

    let expected_parameters =
        gate.kind().parameter_count();

    let actual_parameters =
        gate.parameter_count();

    if expected_parameters != actual_parameters {
        return Err(IrError::Gate(
            IrGateError::InvalidParameterCount {
                gate: gate.kind().as_str(),
                expected: expected_parameters,
                actual: actual_parameters,
            },
        ));
    }

    Ok(())
}

// =============================================================================
// Duplicate logical operands
// =============================================================================

fn validate_duplicate_qubits(gate: &Gate) -> IrResult<()> {
    let mut seen = BTreeSet::<QubitId>::new();

    for &qubit in gate.qubits() {
        if !seen.insert(qubit) {
            return Err(IrError::Qubit(
                super::errors::IrQubitError::Duplicate {
                    qubit: qubit.index(),
                },
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Gate operands
// =============================================================================

fn validate_gate_operands(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let operand_count = gate.qubit_count();

    context
        .limits()
        .check_operands(operand_count)
        .map_err(IrError::from)?;

    for &qubit in gate.qubits() {
        let index = qubit.index();

        if index >= context.num_qubits {
            return Err(IrError::Identifier(
                IrIdentifierError::QubitOutOfRange {
                    index,
                    count: context.num_qubits,
                },
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Parameter validation
// =============================================================================

fn validate_gate_parameters(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let parameter_count =
        gate.parameter_count();

    context
        .limits()
        .check_parameters(parameter_count)
        .map_err(IrError::from)?;

    let expected =
        gate.kind().parameter_count();

    if parameter_count != expected {
        return Err(IrError::Gate(
            IrGateError::InvalidParameterCount {
                gate: gate.kind().as_str(),
                expected,
                actual: parameter_count,
            },
        ));
    }

    for parameter in gate.parameters() {
        parameter
            .validate()
            .map_err(|_| {
                IrError::Parameter(
                    IrParameterError::NonFinite,
                )
            })?;
    }

    Ok(())
}

/// Validates a parameter independently.
pub fn validate_parameter(
    parameter: &super::parameter::Parameter,
) -> IrResult<()> {
    parameter
        .validate()
        .map_err(|_| {
            IrError::Parameter(
                IrParameterError::NonFinite,
            )
        })
}

// =============================================================================
// Classical target validation
// =============================================================================

fn validate_classical_target(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    match gate.classical_target() {
        Some(index) => {
            if !gate.kind().requires_classical_target() {
                return Err(IrError::Gate(
                    IrGateError::InvalidClassicalTarget {
                        gate: gate.kind().as_str(),
                    },
                ));
            }

            if index >= context.num_classical_bits {
                return Err(IrError::Identifier(
                    IrIdentifierError::ClassicalBitOutOfRange {
                        index,
                        count: context.num_classical_bits,
                    },
                ));
            }
        }

        None => {
            if gate.kind().requires_classical_target() {
                return Err(IrError::Measurement(
                    IrMeasurementError::MissingClassicalTarget,
                ));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Measurement representation
// =============================================================================

fn validate_measurement_shape(
    gate: &Gate,
) -> IrResult<()> {
    if gate.kind().is_measurement() {
        let measurement = gate.measurement().ok_or(
            IrError::Measurement(
                IrMeasurementError::InvalidConfiguration {
                    reason:
                        "measurement gate requires a measurement payload",
                },
            ),
        )?;

        let classical_target =
            gate.classical_target().ok_or(
                IrError::Measurement(
                    IrMeasurementError::MissingClassicalTarget,
                ),
            )?;

        let measurement_target =
            measurement.classical_bit().index();

        if classical_target != measurement_target {
            return Err(IrError::Gate(
                IrGateError::InvalidStructure {
                    gate: gate.kind().as_str(),
                    reason:
                        "gate classical target does not match measurement destination",
                },
            ));
        }

        if gate.qubit_count() != 1 {
            return Err(IrError::Measurement(
                IrMeasurementError::InvalidConfiguration {
                    reason:
                        "measurement requires exactly one logical qubit",
                },
            ));
        }

        if measurement.qubit() != gate.qubits()[0] {
            return Err(IrError::Gate(
                IrGateError::InvalidStructure {
                    gate: gate.kind().as_str(),
                    reason:
                        "measurement payload qubit does not match gate operand",
                },
            ));
        }
    } else {
        if gate.measurement().is_some() {
            return Err(IrError::Gate(
                IrGateError::InvalidStructure {
                    gate: gate.kind().as_str(),
                    reason:
                        "non-measurement operation cannot contain a measurement payload",
                },
            ));
        }

        if gate.classical_target().is_some() {
            return Err(IrError::Gate(
                IrGateError::InvalidClassicalTarget {
                    gate: gate.kind().as_str(),
                },
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Gate semantic validation
// =============================================================================

fn validate_gate_semantics(
    _context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let kind = gate.kind();

    if kind.is_measurement() {
        if gate.qubit_count() != 1 {
            return Err(IrError::Measurement(
                IrMeasurementError::InvalidConfiguration {
                    reason:
                        "measurement must operate on exactly one logical qubit",
                },
            ));
        }

        if gate.classical_target().is_none() {
            return Err(IrError::Measurement(
                IrMeasurementError::MissingClassicalTarget,
            ));
        }
    }

    if kind.is_barrier()
        && gate.classical_target().is_some()
    {
        return Err(IrError::Gate(
            IrGateError::InvalidClassicalTarget {
                gate: kind.as_str(),
            },
        ));
    }

    if kind.is_reset()
        && gate.classical_target().is_some()
    {
        return Err(IrError::Gate(
            IrGateError::InvalidClassicalTarget {
                gate: kind.as_str(),
            },
        ));
    }

    Ok(())
}

// =============================================================================
// Rich measurement validation
// =============================================================================

/// Validates a canonical rich measurement object.
///
/// This is separate from gate validation because `Measurement` contains
/// richer semantic information than the lowered `GateKind::Measure`.
pub fn validate_measurement(
    measurement: &Measurement,
    num_qubits: usize,
    num_classical_bits: usize,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_limits(&config.limits)?;

    let context = ValidationContext::new(
        config,
        num_qubits,
        num_classical_bits,
    );

    context
        .limits()
        .check_measurements(1)
        .map_err(IrError::from)?;

    validate_measurement_structure(
        &context,
        measurement,
    )
}

fn validate_measurement_structure(
    context: &ValidationContext<'_>,
    measurement: &Measurement,
) -> IrResult<()> {
    let qubit = measurement.qubit();
    let classical_bit =
        measurement.classical_bit();

    if qubit.index() >= context.num_qubits {
        return Err(IrError::Identifier(
            IrIdentifierError::QubitOutOfRange {
                index: qubit.index(),
                count: context.num_qubits,
            },
        ));
    }

    if classical_bit.index()
        >= context.num_classical_bits
    {
        return Err(IrError::Identifier(
            IrIdentifierError::ClassicalBitOutOfRange {
                index: classical_bit.index(),
                count: context.num_classical_bits,
            },
        ));
    }

    measurement
        .validate()
        .map_err(|_| {
            IrError::Measurement(
                IrMeasurementError::InvalidConfiguration {
                    reason:
                        "measurement violates its semantic contract",
                },
            )
        })?;

    Ok(())
}

// =============================================================================
// Measurement group validation
// =============================================================================

/// Validates a complete measurement group.
///
/// Ordering is preserved exactly as supplied.
pub fn validate_measurement_group(
    group: &MeasurementGroup,
    num_qubits: usize,
    num_classical_bits: usize,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_limits(&config.limits)?;

    let measurements =
        group.measurements();

    context_check_measurement_count(
        &config.limits,
        measurements.len(),
    )?;

    let mut qubits =
        BTreeSet::<QubitId>::new();

    let mut classical_bits =
        BTreeSet::<usize>::new();

    for (index, measurement) in
        measurements.iter().enumerate()
    {
        consume_validation_work(
            &ValidationContext::new(
                config,
                num_qubits,
                num_classical_bits,
            ),
            index,
            1,
        )?;

        validate_measurement(
            measurement,
            num_qubits,
            num_classical_bits,
            config,
        )?;

        let qubit =
            measurement.qubit();

        let classical_bit =
            measurement.classical_bit().index();

        if !qubits.insert(qubit) {
            return Err(IrError::Measurement(
                IrMeasurementError::DuplicateQubit {
                    qubit: qubit.index(),
                },
            ));
        }

        if !classical_bits.insert(classical_bit) {
            return Err(IrError::Measurement(
                IrMeasurementError::DuplicateClassicalTarget {
                    bit: classical_bit,
                },
            ));
        }
    }

    Ok(())
}

fn context_check_measurement_count(
    limits: &QuantumIrLimits,
    count: usize,
) -> IrResult<()> {
    limits
        .check_measurements(count)
        .map_err(IrError::from)
}

// =============================================================================
// Whole-circuit semantic validation
// =============================================================================

fn validate_circuit_semantics(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let mut measured_classical_bits =
        BTreeSet::<usize>::new();

    let mut measurement_count = 0usize;

    for (operation_index, gate) in
        circuit.operations().iter().enumerate()
    {
        consume_validation_work(
            context,
            operation_index,
            1,
        )?;

        if !gate.is_measurement() {
            continue;
        }

        measurement_count =
            measurement_count
                .checked_add(1)
                .ok_or(IrError::Invariant {
                    message:
                        "measurement count overflow",
                })?;

        let classical_bit =
            gate.classical_target().ok_or(
                IrError::Measurement(
                    IrMeasurementError::MissingClassicalTarget,
                ),
            )?;

        if !measured_classical_bits
            .insert(classical_bit)
        {
            return Err(IrError::Measurement(
                IrMeasurementError::DuplicateClassicalTarget {
                    bit: classical_bit,
                },
            ));
        }
    }

    context
        .limits()
        .check_measurements(measurement_count)
        .map_err(IrError::from)?;

    Ok(())
}

// =============================================================================
// Circuit resource validation
// =============================================================================

fn validate_circuit_resources(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let limits = context.limits();

    limits
        .check_operations(circuit.len())
        .map_err(IrError::from)?;

    let mut measurements = 0usize;
    let mut barriers = 0usize;

    for (operation_index, gate) in
        circuit.operations().iter().enumerate()
    {
        consume_validation_work(
            context,
            operation_index,
            1,
        )?;

        if gate.is_measurement() {
            measurements =
                measurements
                    .checked_add(1)
                    .ok_or(IrError::Invariant {
                        message:
                            "measurement count overflow",
                    })?;
        }

        if gate.is_barrier() {
            barriers =
                barriers
                    .checked_add(1)
                    .ok_or(IrError::Invariant {
                        message:
                            "barrier count overflow",
                    })?;
        }

        limits
            .check_operands(gate.qubit_count())
            .map_err(IrError::from)?;

        limits
            .check_parameters(gate.parameter_count())
            .map_err(IrError::from)?;
    }

    limits
        .check_measurements(measurements)
        .map_err(IrError::from)?;

    limits
        .check_barriers(barriers)
        .map_err(IrError::from)?;

    let depth =
        calculate_depth_bounded(
            context,
            circuit,
        )?;

    limits
        .check_depth(depth)
        .map_err(IrError::from)?;

    validate_metadata_size(
        circuit,
        limits.max_metadata_bytes(),
    )?;

    Ok(())
}

// =============================================================================
// Metadata validation
// =============================================================================

fn validate_metadata_size(
    circuit: &QuantumCircuit,
    maximum: usize,
) -> IrResult<()> {
    let metadata =
        circuit.metadata();

    let size =
        metadata
            .byte_size()
            .map_err(|_| {
                IrError::Invariant {
                    message:
                        "metadata size arithmetic overflow",
                }
            })?;

    if size > maximum {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_metadata_bytes",
                size,
                maximum,
            ),
        ));
    }

    Ok(())
}

// =============================================================================
// Validation-work accounting
// =============================================================================

/// Charges deterministic validation work.
///
/// The validator uses explicit work accounting so a maliciously generated IR
/// cannot bypass the configured validation budget simply by choosing a very
/// large operation count.
///
/// `operation_index` is zero-based and is used only to identify the operation
/// currently being processed.
fn consume_validation_work(
    context: &ValidationContext<'_>,
    operation_index: usize,
    additional: usize,
) -> IrResult<()> {
    let steps =
        operation_index
            .checked_add(1)
            .and_then(|value| {
                value.checked_add(additional)
            })
            .ok_or(IrError::Invariant {
                message:
                    "validation work accounting overflow",
            })?;

    if steps >
        context.limits().max_validation_steps()
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_validation_steps",
                steps,
                context
                    .limits()
                    .max_validation_steps(),
            ),
        ));
    }

    Ok(())
}

// =============================================================================
// Policy validation
// =============================================================================

/// Validates the resource-policy object itself.
///
/// Important:
///
/// Zero is NOT universally invalid.
///
/// `QuantumIrLimits` deliberately permits zero for ordinary resources so a
/// policy can intentionally prohibit a resource. Only the limits module itself
/// defines which policy fields are structurally invalid.
///
/// Consequently this function delegates policy validity to
/// `QuantumIrLimits::validate()` instead of imposing a second incompatible
/// policy.
pub fn validate_limits(
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    limits
        .validate()
        .map_err(IrError::from)
}

// =============================================================================
// Sparse depth calculation
// =============================================================================

/// Calculates logical circuit depth using sparse qubit state.
///
/// This function deliberately does not allocate one depth entry for every
/// declared logical qubit.
///
/// Only qubits actually touched by operations consume memory.
///
/// This is important for very large sparse logical namespaces.
fn calculate_depth_bounded(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<usize> {
    if circuit.is_empty() {
        return Ok(0);
    }

    let maximum_depth =
        context.limits().max_depth();

    let mut qubit_depths =
        std::collections::BTreeMap::<
            QubitId,
            usize,
        >::new();

    let mut maximum_seen = 0usize;

    for (operation_index, gate) in
        circuit.operations().iter().enumerate()
    {
        consume_validation_work(
            context,
            operation_index,
            gate.qubit_count(),
        )?;

        let mut latest = 0usize;

        for &qubit in gate.qubits() {
            if qubit.index()
                >= context.num_qubits
            {
                return Err(IrError::Identifier(
                    IrIdentifierError::QubitOutOfRange {
                        index: qubit.index(),
                        count: context.num_qubits,
                    },
                ));
            }

            if let Some(depth) =
                qubit_depths.get(&qubit)
            {
                latest = latest.max(*depth);
            }
        }

        let next =
            latest
                .checked_add(1)
                .ok_or(IrError::Invariant {
                    message:
                        "circuit depth arithmetic overflow",
                })?;

        if next > maximum_depth {
            return Err(IrError::Limit(
                super::errors::IrLimitError::new(
                    "max_depth",
                    next,
                    maximum_depth,
                ),
            ));
        }

        for &qubit in gate.qubits() {
            qubit_depths.insert(
                qubit,
                next,
            );
        }

        maximum_seen =
            maximum_seen.max(next);
    }

    Ok(maximum_seen)
}

// =============================================================================
// Validation summary
// =============================================================================

/// Deterministic structural validation summary.
///
/// This is not a cryptographic hash. It is a compact structural result useful
/// for tests, diagnostics, benchmarking metadata, and deterministic validation
/// comparisons.
///
/// Cryptographic identity belongs to `hash.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationSummary {
    /// Logical qubit namespace size.
    pub qubits: usize,

    /// Classical namespace size.
    pub classical_bits: usize,

    /// Number of logical operations.
    pub operations: usize,

    /// Number of measurement operations.
    pub measurements: usize,

    /// Number of barrier operations.
    pub barriers: usize,

    /// Logical circuit depth.
    pub depth: usize,
}

impl ValidationSummary {
    /// Validates and summarizes a circuit.
    pub fn from_circuit(
        circuit: &QuantumCircuit,
        config: &ValidationConfig,
    ) -> IrResult<Self> {
        validate_circuit_with_config(
            circuit,
            config,
        )?;

        let context =
            ValidationContext::new(
                config,
                circuit.num_qubits(),
                circuit.num_classical_bits(),
            );

        let depth =
            calculate_depth_bounded(
                &context,
                circuit,
            )?;

        Ok(Self {
            qubits: circuit.num_qubits(),
            classical_bits:
                circuit.num_classical_bits(),
            operations: circuit.len(),
            measurements:
                circuit.measurement_count(),
            barriers:
                circuit.barrier_count(),
            depth,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn production_config() -> ValidationConfig {
        ValidationConfig::production()
    }

    #[test]
    fn empty_circuit_is_valid_by_default() {
        let circuit =
            QuantumCircuit::new(2, 0);

        assert!(
            validate_circuit(
                &circuit
            )
            .is_ok()
        );
    }

    #[test]
    fn single_qubit_gate_is_valid() {
        let mut circuit =
            QuantumCircuit::new(2, 0);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        assert!(
            validate_circuit(
                &circuit
            )
            .is_ok()
        );
    }

    #[test]
    fn two_qubit_gate_is_valid() {
        let mut circuit =
            QuantumCircuit::new(2, 0);

        circuit
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        assert!(
            validate_circuit(
                &circuit
            )
            .is_ok()
        );
    }

    #[test]
    fn operation_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_operations(1);

        let config =
            ValidationConfig::new(
                limits,
            );

        let mut circuit =
            QuantumCircuit::new(2, 0);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(1)).unwrap(),
            )
            .unwrap();

        assert!(
            matches!(
                validate_circuit_with_config(
                    &circuit,
                    &config,
                ),
                Err(IrError::Limit(_))
            )
        );
    }

    #[test]
    fn operand_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_operands(2);

        let config =
            ValidationConfig::new(
                limits,
            );

        let gate =
            Gate::ccx(
                q(0),
                q(1),
                q(2),
            )
            .unwrap();

        assert!(
            matches!(
                validate_operation(
                    &gate,
                    3,
                    0,
                    &config,
                ),
                Err(IrError::Limit(_))
            )
        );
    }

    #[test]
    fn depth_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_depth(2);

        let config =
            ValidationConfig::new(
                limits,
            );

        let mut circuit =
            QuantumCircuit::new(1, 0);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        assert!(
            matches!(
                validate_circuit_with_config(
                    &circuit,
                    &config,
                ),
                Err(IrError::Limit(_))
            )
        );
    }

    #[test]
    fn sparse_depth_does_not_depend_on_declared_namespace_size() {
        let limits =
            QuantumIrLimits::unbounded();

        let config =
            ValidationConfig::new(
                limits,
            );

        let mut circuit =
            QuantumCircuit::new(
                usize::MAX,
                0,
            );

        circuit
            .push(
                Gate::x(
                    q(usize::MAX - 1),
                )
                .unwrap(),
            )
            .unwrap();

        assert!(
            validate_circuit_with_config(
                &circuit,
                &config,
            )
            .is_ok()
        );
    }

    #[test]
    fn logical_qubit_zero_is_valid() {
        let config =
            production_config();

        let gate =
            Gate::x(q(0)).unwrap();

        assert!(
            validate_operation(
                &gate,
                1,
                0,
                &config,
            )
            .is_ok()
        );
    }

    #[test]
    fn qubit_out_of_namespace_is_rejected() {
        let config =
            production_config();

        let gate =
            Gate::x(q(1)).unwrap();

        assert!(
            matches!(
                validate_operation(
                    &gate,
                    1,
                    0,
                    &config,
                ),
                Err(IrError::Identifier(_))
            )
        );
    }

    #[test]
    fn duplicate_measurement_destinations_are_rejected() {
        let config =
            production_config();

        let measurement_a =
            Measurement::z(
                q(0),
                super::super::measurement::ClassicalBitId::new(0),
            )
            .unwrap();

        let measurement_b =
            Measurement::z(
                q(1),
                super::super::measurement::ClassicalBitId::new(0),
            )
            .unwrap();

        let mut circuit =
            QuantumCircuit::new(2, 1);

        circuit
            .push(
                Gate::from_measurement(
                    measurement_a,
                )
                .unwrap(),
            )
            .unwrap();

        circuit
            .push(
                Gate::from_measurement(
                    measurement_b,
                )
                .unwrap(),
            )
            .unwrap();

        assert!(
            matches!(
                validate_circuit_with_config(
                    &circuit,
                    &config,
                ),
                Err(IrError::Measurement(_))
            )
        );
    }

    #[test]
    fn validation_summary_is_deterministic() {
        let config =
            production_config();

        let mut first =
            QuantumCircuit::new(2, 0);

        first
            .push(
                Gate::h(q(0)).unwrap(),
            )
            .unwrap();

        first
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        let mut second =
            QuantumCircuit::new(2, 0);

        second
            .push(
                Gate::h(q(0)).unwrap(),
            )
            .unwrap();

        second
            .push(
                Gate::cx(
                    q(0),
                    q(1),
                )
                .unwrap(),
            )
            .unwrap();

        let first_summary =
            ValidationSummary::from_circuit(
                &first,
                &config,
            )
            .unwrap();

        let second_summary =
            ValidationSummary::from_circuit(
                &second,
                &config,
            )
            .unwrap();

        assert_eq!(
            first_summary,
            second_summary
        );
    }

    #[test]
    fn failed_validation_does_not_mutate_circuit() {
        let mut circuit =
            QuantumCircuit::new(1, 0);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        let before =
            circuit.len();

        let limits =
            QuantumIrLimits::production()
                .with_max_operations(0);

        let config =
            ValidationConfig::new(
                limits,
            );

        assert!(
            validate_circuit_with_config(
                &circuit,
                &config,
            )
            .is_err()
        );

        assert_eq!(
            circuit.len(),
            before
        );
    }

    #[test]
    fn zero_resource_policy_is_not_rejected_as_malformed() {
        let limits =
            QuantumIrLimits::production()
                .with_max_qubits(0)
                .with_max_classical_bits(0)
                .with_max_operations(0);

        assert!(
            validate_limits(
                &limits
            )
            .is_ok()
        );
    }

    #[test]
    fn unbounded_policy_is_accepted() {
        let limits =
            QuantumIrLimits::unbounded();

        assert!(
            validate_limits(
                &limits
            )
            .is_ok()
        );
    }
}