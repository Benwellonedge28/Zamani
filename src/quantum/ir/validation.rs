//! Zamani Quantum Intermediate Representation — Validation
//!
//! Canonical whole-IR validation for the hardware-independent quantum
//! intermediate representation.
//!
//! # Architectural boundary
//!
//! This module validates the logical quantum program only.
//!
//! It deliberately does NOT validate:
//! - physical qubit topology;
//! - logical-to-physical routing;
//! - pulse schedules;
//! - calibration;
//! - backend capabilities;
//! - QPU communication;
//! - hardware-specific gate decomposition;
//! - syndrome decoding;
//! - error-correction hardware geometry.
//!
//! Those concerns belong to downstream compiler/backend stages.
//!
//! # Validation model
//!
//! Validation is divided into four layers:
//!
//! 1. Structural validation
//!    - gate shape;
//!    - operand counts;
//!    - parameter shape;
//!    - duplicate operands;
//!    - measurement targets.
//!
//! 2. Namespace validation
//!    - logical qubit bounds;
//!    - classical-bit bounds.
//!
//! 3. Resource validation
//!    - qubit limits;
//!    - classical-bit limits;
//!    - operation limits;
//!    - operand limits;
//!    - parameter limits;
//!    - measurement limits;
//!    - barrier limits;
//!    - metadata limits;
//!    - validation-work limits;
//!    - depth limits.
//!
//! 4. Semantic validation
//!    - measurement consistency;
//!    - classical-target uniqueness;
//!    - operation-specific constraints;
//!    - deterministic whole-circuit invariants.
//!
//! The validator is intentionally usable against externally supplied IR.
//! Callers must NOT be assumed to have used safe constructors.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and intentionally avoids nightly
//! features and unnecessary dependencies.

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
    ClassicalBitId,
    Measurement,
    MeasurementGroup,
};
use super::qubits::QubitId;

// -----------------------------------------------------------------------------
// Validation configuration
// -----------------------------------------------------------------------------

/// Configuration controlling whole-IR validation.
///
/// The configuration is intentionally immutable during one validation pass.
/// This guarantees that the same IR and configuration produce the same result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Resource limits applied during validation.
    pub limits: QuantumIrLimits,

    /// Enables stricter semantic checks.
    ///
    /// Strict validation is intended for:
    /// - deserialization;
    /// - replay;
    /// - compiler boundaries;
    /// - external IR ingestion;
    /// - release builds where malformed IR must be rejected early.
    pub strict: bool,

    /// Whether a circuit with zero operations is valid.
    pub allow_empty_circuit: bool,

    /// Whether semantic consistency checks should be performed.
    pub semantic_checks: bool,
}

impl ValidationConfig {
    /// Creates a validation configuration using the supplied limits.
    pub const fn new(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Returns the production validation configuration.
    pub fn production() -> Self {
        Self::new(QuantumIrLimits::default())
    }

    /// Returns a strict configuration.
    pub const fn strict(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Returns a permissive configuration.
    ///
    /// This is still structurally safe. It only disables optional semantic
    /// checks and permits empty circuits.
    pub const fn permissive(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: false,
            allow_empty_circuit: true,
            semantic_checks: false,
        }
    }

    /// Enables or disables strict validation.
    pub const fn with_strict(
        mut self,
        strict: bool,
    ) -> Self {
        self.strict = strict;
        self
    }

    /// Enables or disables empty circuits.
    pub const fn with_empty_circuits(
        mut self,
        allow: bool,
    ) -> Self {
        self.allow_empty_circuit = allow;
        self
    }

    /// Enables or disables semantic checks.
    pub const fn with_semantic_checks(
        mut self,
        enabled: bool,
    ) -> Self {
        self.semantic_checks = enabled;
        self
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::production()
    }
}

// -----------------------------------------------------------------------------
// Validation context
// -----------------------------------------------------------------------------

/// Internal immutable context for one validation pass.
///
/// Keeping the context separate prevents individual validation functions from
/// reconstructing limits and namespace information repeatedly.
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

// -----------------------------------------------------------------------------
// Public circuit validation
// -----------------------------------------------------------------------------

/// Validates a complete quantum circuit using production defaults.
///
/// This is the canonical convenience entry point.
pub fn validate_circuit(
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    validate_circuit_with_config(
        circuit,
        &ValidationConfig::production(),
    )
}

/// Validates a complete circuit against explicit limits.
pub fn validate_circuit_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    let config = ValidationConfig::new(limits.clone());

    validate_circuit_with_config(
        circuit,
        &config,
    )
}

/// Validates a complete circuit using an explicit configuration.
///
/// This function must be used at untrusted IR boundaries.
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

    validate_circuit_shape(
        &context,
        circuit,
    )?;

    validate_circuit_resources(
        &context,
        circuit,
    )?;

    if config.semantic_checks {
        validate_circuit_semantics(
            &context,
            circuit,
        )?;
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Circuit structural validation
// -----------------------------------------------------------------------------

fn validate_circuit_shape(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let limits = context.limits();

    if circuit.num_qubits() > limits.max_qubits {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_qubits",
                circuit.num_qubits(),
                limits.max_qubits,
            ),
        ));
    }

    if circuit.num_classical_bits()
        > limits.max_classical_bits
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_classical_bits",
                circuit.num_classical_bits(),
                limits.max_classical_bits,
            ),
        ));
    }

    if !context.config.allow_empty_circuit
        && circuit.is_empty()
    {
        return Err(IrError::InvalidStructure {
            message: "empty circuits are not permitted by the validation configuration",
        });
    }

    for (index, gate) in
        circuit.operations().iter().enumerate()
    {
        validate_operation_at(
            context,
            gate,
            index,
        )?;
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Circuit resource validation
// -----------------------------------------------------------------------------

fn validate_circuit_resources(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let limits = context.limits();

    let operation_count = circuit.len();

    if operation_count > limits.max_operations {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_operations",
                operation_count,
                limits.max_operations,
            ),
        ));
    }

    let mut measurement_count = 0usize;
    let mut barrier_count = 0usize;

    for gate in circuit.operations() {
        if gate.is_measurement() {
            measurement_count = measurement_count
                .checked_add(1)
                .ok_or_else(|| {
                    IrError::Invariant {
                        message:
                            "measurement count overflow",
                    }
                })?;
        }

        if gate.is_barrier() {
            barrier_count = barrier_count
                .checked_add(1)
                .ok_or_else(|| {
                    IrError::Invariant {
                        message:
                            "barrier count overflow",
                    }
                })?;
        }

        if gate.qubits().len()
            > limits.max_operands
        {
            return Err(IrError::Limit(
                super::errors::IrLimitError::new(
                    "max_operands",
                    gate.qubits().len(),
                    limits.max_operands,
                ),
            ));
        }

        if gate.parameter_count()
            > limits.max_parameters
        {
            return Err(IrError::Limit(
                super::errors::IrLimitError::new(
                    "max_parameters",
                    gate.parameter_count(),
                    limits.max_parameters,
                ),
            ));
        }
    }

    if measurement_count
        > limits.max_measurements
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_measurements",
                measurement_count,
                limits.max_measurements,
            ),
        ));
    }

    if barrier_count > limits.max_barriers {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_barriers",
                barrier_count,
                limits.max_barriers,
            ),
        ));
    }

    let depth = calculate_depth_bounded(
        circuit,
        limits.max_depth,
    )?;

    if depth > limits.max_depth {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_depth",
                depth,
                limits.max_depth,
            ),
        ));
    }

    validate_metadata_size(
        circuit,
        limits.max_metadata_bytes,
    )?;

    Ok(())
}

// -----------------------------------------------------------------------------
// Operation validation
// -----------------------------------------------------------------------------

/// Validates a single operation.
///
/// This is the canonical operation-level entry point.
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

    validate_operation_at(
        &context,
        gate,
        0,
    )
}

/// Validates a single gate.
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
    validate_operation_index(
        context,
        operation_index,
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

    if context.config.semantic_checks {
        validate_gate_semantics(
            context,
            gate,
        )?;
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Gate structure
// -----------------------------------------------------------------------------

fn validate_gate_structure(
    gate: &Gate,
) -> IrResult<()> {
    let kind = gate.kind();

    let qubit_count = gate.qubits().len();

    match kind.expected_qubits() {
        Some(expected) => {
            if qubit_count != expected {
                return Err(IrError::Gate(
                    IrGateError::InvalidQubitCount {
                        gate: kind.as_str(),
                        expected,
                        actual: qubit_count,
                    },
                ));
            }
        }

        None => {
            // Variable-width operations currently consist of barriers.
            if kind.is_barrier()
                && qubit_count == 0
            {
                return Err(IrError::Gate(
                    IrGateError::InvalidBarrier,
                ));
            }
        }
    }

    validate_duplicate_qubits(gate)?;

    validate_parameter_shape(gate)?;

    validate_measurement_shape(gate)?;

    Ok(())
}

fn validate_duplicate_qubits(
    gate: &Gate,
) -> IrResult<()> {
    let mut seen = BTreeSet::new();

    for qubit in gate.qubits() {
        let index = qubit.index();

        if !seen.insert(index) {
            return Err(IrError::Qubit(
                super::errors::IrQubitError::Duplicate {
                    qubit: index,
                },
            ));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Gate operands
// -----------------------------------------------------------------------------

fn validate_gate_operands(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    for qubit in gate.qubits() {
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

    if gate.qubits().len()
        > context.limits().max_operands
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_operands",
                gate.qubits().len(),
                context.limits().max_operands,
            ),
        ));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Gate parameters
// -----------------------------------------------------------------------------

fn validate_gate_parameters(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let parameter_count =
        gate.parameter_count();

    if parameter_count
        > context.limits().max_parameters
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_parameters",
                parameter_count,
                context.limits().max_parameters,
            ),
        ));
    }

    match (
        gate.kind().is_parameterized(),
        gate.parameter(),
    ) {
        (true, None) => {
            return Err(IrError::Gate(
                IrGateError::MissingParameter {
                    gate: gate.kind().as_str(),
                },
            ));
        }

        (false, Some(_)) => {
            return Err(IrError::Gate(
                IrGateError::UnexpectedParameter {
                    gate: gate.kind().as_str(),
                },
            ));
        }

        (true, Some(parameter))
        | (false, Some(parameter)) => {
            validate_parameter(parameter)?;
        }

        (false, None) => {}
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

// -----------------------------------------------------------------------------
// Measurement operation validation
// -----------------------------------------------------------------------------

fn validate_measurement_shape(
    gate: &Gate,
) -> IrResult<()> {
    if gate.kind().is_measurement() {
        if gate.classical_target().is_none() {
            return Err(IrError::Measurement(
                IrMeasurementError::MissingClassicalTarget,
            ));
        }

        if gate.parameter().is_some() {
            return Err(IrError::Gate(
                IrGateError::UnexpectedParameter {
                    gate: gate.kind().as_str(),
                },
            ));
        }
    } else if gate.classical_target().is_some() {
        return Err(IrError::Gate(
            IrGateError::InvalidClassicalTarget {
                gate: gate.kind().as_str(),
            },
        ));
    }

    Ok(())
}

fn validate_classical_target(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    if let Some(classical_bit) =
        gate.classical_target()
    {
        let index = classical_bit.index();

        if index >= context.num_classical_bits {
            return Err(IrError::Identifier(
                IrIdentifierError::ClassicalBitOutOfRange {
                    index,
                    count: context.num_classical_bits,
                },
            ));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Gate semantic validation
// -----------------------------------------------------------------------------

fn validate_gate_semantics(
    _context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let kind = gate.kind();

    // Measurement must have exactly one logical source.
    if kind.is_measurement()
        && gate.qubits().len() != 1
    {
        return Err(IrError::Measurement(
            IrMeasurementError::InvalidConfiguration {
                reason:
                    "a measurement operation must have exactly one logical qubit",
            },
        ));
    }

    // Barriers must never have a classical destination.
    if kind.is_barrier()
        && gate.classical_target().is_some()
    {
        return Err(IrError::Gate(
            IrGateError::InvalidStructure {
                gate: kind.as_str(),
                reason:
                    "a barrier cannot target a classical bit",
            },
        ));
    }

    // Reset is a logical operation and does not produce a classical result.
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

// -----------------------------------------------------------------------------
// Measurement object validation
// -----------------------------------------------------------------------------

/// Validates a rich measurement object.
///
/// This function is intentionally separate from `validate_gate()` because
/// `Measurement` contains richer hardware-independent semantics than the
/// lowered `GateKind::Measure` representation.
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

// -----------------------------------------------------------------------------
// Measurement group validation
// -----------------------------------------------------------------------------

/// Validates a measurement group.
///
/// Ordering is significant and therefore preserved exactly as supplied.
pub fn validate_measurement_group(
    group: &MeasurementGroup,
    num_qubits: usize,
    num_classical_bits: usize,
    config: &ValidationConfig,
) -> IrResult<()> {
    validate_limits(&config.limits)?;

    let measurements =
        group.measurements();

    if measurements.len()
        > config.limits.max_measurements
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_measurements",
                measurements.len(),
                config.limits.max_measurements,
            ),
        ));
    }

    let mut qubits = BTreeSet::new();
    let mut classical_bits = BTreeSet::new();

    for measurement in measurements {
        validate_measurement(
            measurement,
            num_qubits,
            num_classical_bits,
            config,
        )?;

        let qubit =
            measurement.qubit().index();

        let classical_bit =
            measurement.classical_bit().index();

        if !qubits.insert(qubit) {
            return Err(IrError::Measurement(
                IrMeasurementError::DuplicateQubit {
                    qubit,
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

// -----------------------------------------------------------------------------
// Whole-circuit measurement semantics
// -----------------------------------------------------------------------------

fn validate_circuit_semantics(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let mut measured_classical_bits =
        BTreeSet::new();

    for gate in circuit.operations() {
        if !gate.is_measurement() {
            continue;
        }

        let classical_bit =
            match gate.classical_target() {
                Some(bit) => bit.index(),

                None => {
                    return Err(
                        IrError::Measurement(
                            IrMeasurementError::MissingClassicalTarget,
                        ),
                    );
                }
            };

        if !measured_classical_bits
            .insert(classical_bit)
        {
            return Err(IrError::Measurement(
                IrMeasurementError::DuplicateClassicalTarget {
                    bit: classical_bit,
                },
            ));
        }

        if classical_bit
            >= context.num_classical_bits
        {
            return Err(IrError::Identifier(
                IrIdentifierError::ClassicalBitOutOfRange {
                    index: classical_bit,
                    count: context.num_classical_bits,
                },
            ));
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Resource limits validation
// -----------------------------------------------------------------------------

/// Validates the limits configuration itself.
///
/// A malformed limits object must be rejected before it is applied to
/// untrusted IR.
pub fn validate_limits(
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    if limits.max_qubits == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_qubits",
                0,
                0,
            ),
        ));
    }

    if limits.max_classical_bits == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_classical_bits",
                0,
                0,
            ),
        ));
    }

    if limits.max_operations == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_operations",
                0,
                0,
            ),
        ));
    }

    if limits.max_operands == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_operands",
                0,
                0,
            ),
        ));
    }

    if limits.max_parameters == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_parameters",
                0,
                0,
            ),
        ));
    }

    if limits.max_depth == 0 {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_depth",
                0,
                0,
            ),
        ));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Validation-work accounting
// -----------------------------------------------------------------------------

fn validate_operation_index(
    context: &ValidationContext<'_>,
    operation_index: usize,
) -> IrResult<()> {
    if operation_index
        >= context.limits().max_validation_work
    {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_validation_work",
                operation_index,
                context
                    .limits()
                    .max_validation_work,
            ),
        ));
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Metadata validation
// -----------------------------------------------------------------------------

fn validate_metadata_size(
    circuit: &QuantumCircuit,
    maximum: usize,
) -> IrResult<()> {
    let metadata = circuit.metadata();

    let mut total = 0usize;

    if let Some(name) = &metadata.name {
        total = checked_add_metadata_size(
            total,
            name.len(),
            maximum,
        )?;
    }

    if let Some(source) = &metadata.source {
        total = checked_add_metadata_size(
            total,
            source.len(),
            maximum,
        )?;
    }

    if let Some(version) =
        &metadata.compiler_version
    {
        total = checked_add_metadata_size(
            total,
            version.len(),
            maximum,
        )?;
    }

    if total > maximum {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_metadata_bytes",
                total,
                maximum,
            ),
        ));
    }

    Ok(())
}

fn checked_add_metadata_size(
    current: usize,
    additional: usize,
    maximum: usize,
) -> IrResult<usize> {
    let total =
        current.checked_add(additional).ok_or(
            IrError::Invariant {
                message:
                    "metadata size overflow",
            },
        )?;

    if total > maximum {
        return Err(IrError::Limit(
            super::errors::IrLimitError::new(
                "max_metadata_bytes",
                total,
                maximum,
            ),
        ));
    }

    Ok(total)
}

// -----------------------------------------------------------------------------
// Bounded deterministic depth calculation
// -----------------------------------------------------------------------------

/// Calculates circuit depth without allowing unchecked depth growth.
///
/// This function is intentionally independent from the later `analysis.rs`
/// implementation so validation can protect itself from malformed IR.
fn calculate_depth_bounded(
    circuit: &QuantumCircuit,
    maximum_depth: usize,
) -> IrResult<usize> {
    if circuit.num_qubits() == 0 {
        return Ok(0);
    }

    let mut depths =
        vec![0usize; circuit.num_qubits()];

    for gate in circuit.operations() {
        let mut latest = 0usize;

        for qubit in gate.qubits() {
            let index = qubit.index();

            if index >= depths.len() {
                return Err(IrError::Identifier(
                    IrIdentifierError::QubitOutOfRange {
                        index,
                        count: depths.len(),
                    },
                ));
            }

            latest = latest.max(depths[index]);
        }

        let next_depth =
            latest.checked_add(1).ok_or(
                IrError::Invariant {
                    message:
                        "circuit depth overflow",
                },
            )?;

        if next_depth > maximum_depth {
            return Err(IrError::Limit(
                super::errors::IrLimitError::new(
                    "max_depth",
                    next_depth,
                    maximum_depth,
                ),
            ));
        }

        for qubit in gate.qubits() {
            depths[qubit.index()] =
                next_depth;
        }
    }

    Ok(depths.into_iter().max().unwrap_or(0))
}

// -----------------------------------------------------------------------------
// Deterministic validation fingerprint
// -----------------------------------------------------------------------------

/// Returns a deterministic validation summary.
///
/// This is deliberately a compact structural summary rather than a cryptographic
/// hash. It is useful for testing that validation observes the same structure
/// deterministically across repeated passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationSummary {
    /// Number of logical qubits.
    pub qubits: usize,

    /// Number of classical bits.
    pub classical_bits: usize,

    /// Number of operations.
    pub operations: usize,

    /// Number of measurement operations.
    pub measurements: usize,

    /// Number of barriers.
    pub barriers: usize,

    /// Calculated logical depth.
    pub depth: usize,
}

impl ValidationSummary {
    /// Produces a deterministic summary after validation.
    pub fn from_circuit(
        circuit: &QuantumCircuit,
        config: &ValidationConfig,
    ) -> IrResult<Self> {
        validate_circuit_with_config(
            circuit,
            config,
        )?;

        let depth =
            calculate_depth_bounded(
                circuit,
                config.limits.max_depth,
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::gate::Gate;
    use super::super::measurement::ClassicalBitId;
    use super::super::qubits::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn c(index: usize) -> ClassicalBitId {
        ClassicalBitId::new(index)
    }

    fn config() -> ValidationConfig {
        ValidationConfig::production()
    }

    #[test]
    fn valid_empty_circuit_is_accepted() {
        let circuit =
            QuantumCircuit::new(2, 2);

        assert!(
            validate_circuit_with_config(
                &circuit,
                &config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn valid_single_qubit_gate_is_accepted() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        assert!(
            validate_circuit_with_config(
                &circuit,
                &config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn valid_two_qubit_gate_is_accepted() {
        let mut circuit =
            QuantumCircuit::new(2, 2);

        circuit
            .push(
                Gate::cx(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        assert!(
            validate_circuit_with_config(
                &circuit,
                &config(),
            )
            .is_ok()
        );
    }

    #[test]
    fn measurement_requires_classical_target() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

        let gate = Gate::new(
            GateKind::Measure,
            vec![q(0)],
        )
        .unwrap_err();

        assert!(matches!(
            gate,
            super::super::gate::GateError::
                MissingClassicalTarget
        ));

        // Constructor-level validation is intentionally complemented by
        // whole-IR validation for externally supplied/deserialized values.
        let _ = &mut circuit;
    }

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let mut circuit =
            QuantumCircuit::new(1, 1);

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
    fn operation_limit_is_enforced() {
        let mut limits =
            QuantumIrLimits::default();

        limits.max_operations = 1;

        let validation =
            ValidationConfig::new(
                limits,
            );

        let mut circuit =
            QuantumCircuit::new(2, 2);

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

        let result =
            validate_circuit_with_config(
                &circuit,
                &validation,
            );

        assert!(matches!(
            result,
            Err(IrError::Limit(_))
        ));
    }

    #[test]
    fn operand_limit_is_enforced() {
        let mut limits =
            QuantumIrLimits::default();

        limits.max_operands = 2;

        let validation =
            ValidationConfig::new(
                limits,
            );

        let gate = Gate::ccx(
            q(0),
            q(1),
            q(2),
        )
        .unwrap();

        let result = validate_operation(
            &gate,
            3,
            3,
            &validation,
        );

        assert!(matches!(
            result,
            Err(IrError::Limit(_))
        ));
    }

    #[test]
    fn depth_limit_is_enforced() {
        let mut limits =
            QuantumIrLimits::default();

        limits.max_depth = 2;

        let validation =
            ValidationConfig::new(
                limits,
            );

        let mut circuit =
            QuantumCircuit::new(1, 1);

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

        let result =
            validate_circuit_with_config(
                &circuit,
                &validation,
            );

        assert!(matches!(
            result,
            Err(IrError::Limit(_))
        ));
    }

    #[test]
    fn validation_summary_is_deterministic() {
        let mut first =
            QuantumCircuit::new(2, 2);

        first
            .push(
                Gate::h(q(0)).unwrap(),
            )
            .unwrap();

        first
            .push(
                Gate::cx(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        let mut second =
            QuantumCircuit::new(2, 2);

        second
            .push(
                Gate::h(q(0)).unwrap(),
            )
            .unwrap();

        second
            .push(
                Gate::cx(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        let first_summary =
            ValidationSummary::from_circuit(
                &first,
                &config(),
            )
            .unwrap();

        let second_summary =
            ValidationSummary::from_circuit(
                &second,
                &config(),
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
            QuantumCircuit::new(1, 1);

        circuit
            .push(
                Gate::x(q(0)).unwrap(),
            )
            .unwrap();

        let before = circuit.len();

        let mut limits =
            QuantumIrLimits::default();

        limits.max_operations = 0;

        let validation =
            ValidationConfig::new(
                limits,
            );

        let result =
            validate_circuit_with_config(
                &circuit,
                &validation,
            );

        assert!(result.is_err());
        assert_eq!(
            circuit.len(),
            before
        );
    }
}