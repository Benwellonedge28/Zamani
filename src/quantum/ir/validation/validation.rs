//! Zamani Quantum IR — Canonical Validation Engine
//!
//! Production-grade structural, semantic, namespace, resource-policy and
//! determinism validation for the canonical Zamani Quantum IR.
//!
//! # Module path
//!
//! ```text
//! quantum::ir::validation::validation
//! ```
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > Is this IR representation internally valid under the supplied
//! > validation/resource policy?
//!
//! It does NOT answer:
//!
//! - whether a physical QPU exists;
//! - whether physical qubits are connected;
//! - whether routing is possible;
//! - whether a target supports an operation;
//! - which native gate is selected;
//! - which calibration is selected;
//! - how pulses are synthesized;
//! - how scheduling is performed;
//! - how a backend communicates with hardware;
//! - how a simulator represents quantum state;
//! - how QEC is decoded.
//!
//! Those are downstream concerns.
//!
//! # Validation layers
//!
//! Validation is deliberately layered:
//!
//! ```text
//! policy
//!   ↓
//! namespace
//!   ↓
//! structure
//!   ↓
//! operands
//!   ↓
//! parameters
//!   ↓
//! classical destinations
//!   ↓
//! measurement consistency
//!   ↓
//! semantic invariants
//!   ↓
//! resource accounting
//! ```
//!
//! Every layer is deterministic and uses the canonical IR types.
//!
//! # Scalability
//!
//! Zamani has no architectural fixed quantum-machine size.
//!
//! This validator therefore:
//!
//! - never hard-codes a maximum qubit count;
//! - never allocates state proportional to declared qubit count;
//! - never uses fixed-size qubit arrays;
//! - never uses machine-size bit masks;
//! - never assumes a particular topology;
//! - never assumes a particular hardware architecture;
//! - uses sparse sets for touched resources;
//! - uses checked arithmetic for validation work;
//! - delegates actual resource ceilings to `QuantumIrLimits`.
//!
//! A policy limit is a deployment/security boundary, not the maximum size
//! of the Zamani language.
//!
//! # Trust boundary
//!
//! Validation must be repeated at trust boundaries even when constructors
//! already validate their arguments.
//!
//! IR may originate from:
//!
//! - a frontend;
//! - deserialization;
//! - generated code;
//! - optimization passes;
//! - external tools;
//! - cache/replay;
//! - distributed compilation;
//! - future dialects.
//!
//! Therefore this module does not assume that constructors were used.
//!
//! # Canonical qubit identity
//!
//! The authoritative type is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! No `qubits` module is referenced here.
//!
//! # Rust
//!
//! Compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The compiler-enforced `forbid(unsafe_code)` attribute is intentional.
//!
//! # Integration contract
//!
//! This file consumes:
//!
//! - `circuit.rs` for `QuantumCircuit`;
//! - `gate.rs` for gate semantics;
//! - `limits.rs` for resource policy;
//! - `measurement.rs` for measurement semantics;
//! - `qubit.rs` for canonical qubit identity;
//! - `errors.rs` for canonical diagnostics.
//!
//! It must not depend on:
//!
//! - hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - simulator;
//! - QEC implementation;
//! - backend execution;
//! - frontend parsing.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! `ValidationConfig`
//!     Immutable policy/configuration for one validation pass.
//!
//! `validate_circuit`
//!     Production validation entry point.
//!
//! `validate_circuit_with_limits`
//!     Production validation with explicit resource policy.
//!
//! `validate_circuit_with_config`
//!     Fully controlled validation entry point.
//!
//! `validate_operation`
//!     Validates one gate against a logical/classical namespace.
//!
//! `validate_gate`
//!     Explicit gate-validation alias.
//!
//! `validate_limits`
//!     Validates the policy itself.
//!
//! The API intentionally keeps validation deterministic and side-effect free.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;

use super::super::circuit::QuantumCircuit;
use super::super::errors::{
    IrError,
    IrGateError,
    IrIdentifierError,
    IrResult,
};
use super::super::gate::Gate;
use super::super::limits::QuantumIrLimits;
use super::super::qubit::QubitId;

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for one complete IR validation pass.
///
/// The configuration is immutable from the validator's perspective. A single
/// configuration therefore governs every validation stage of one invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Explicit IR resource/security policy.
    pub limits: QuantumIrLimits,

    /// Enables strict validation mode.
    ///
    /// Strict mode is intended for trust boundaries such as deserialization,
    /// compiler services, release builds and external IR ingestion.
    pub strict: bool,

    /// Whether a circuit containing zero operations is permitted.
    pub allow_empty_circuit: bool,

    /// Whether whole-circuit semantic checks are performed.
    pub semantic_checks: bool,
}

impl ValidationConfig {
    /// Creates the normal production validation policy.
    pub const fn new(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Creates the standard production configuration.
    #[must_use]
    pub fn production() -> Self {
        Self::new(QuantumIrLimits::production())
    }

    /// Creates strict validation using explicit limits.
    pub const fn strict(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: true,
            allow_empty_circuit: true,
            semantic_checks: true,
        }
    }

    /// Creates a configuration that disables optional whole-program semantic
    /// checks while retaining structural and resource checks.
    ///
    /// This should not normally be used at a trust boundary.
    pub const fn permissive(limits: QuantumIrLimits) -> Self {
        Self {
            limits,
            strict: false,
            allow_empty_circuit: true,
            semantic_checks: false,
        }
    }

    /// Enables or disables strict mode.
    #[must_use]
    pub const fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Enables or disables empty circuits.
    #[must_use]
    pub const fn with_empty_circuits(mut self, allow: bool) -> Self {
        self.allow_empty_circuit = allow;
        self
    }

    /// Enables or disables whole-program semantic validation.
    #[must_use]
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
// Internal validation context
// =============================================================================

/// Immutable state shared by one validation pass.
///
/// No mutable global state is used. This makes validation safe to execute
/// concurrently for independent IR objects.
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

    #[inline]
    fn limits(&self) -> &QuantumIrLimits {
        &self.config.limits
    }
}

// =============================================================================
// Public circuit validation
// =============================================================================

/// Validates a complete circuit using the standard production policy.
///
/// This is the preferred entry point for already-constructed canonical IR.
pub fn validate_circuit(circuit: &QuantumCircuit) -> IrResult<()> {
    let config = ValidationConfig::production();

    validate_circuit_with_config(circuit, &config)
}

/// Validates a complete circuit against an explicit resource policy.
///
/// The policy is copied into the validation configuration. The circuit itself
/// is never modified.
pub fn validate_circuit_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> IrResult<()> {
    let config = ValidationConfig::new(*limits);

    validate_circuit_with_config(circuit, &config)
}

/// Validates a complete circuit against an explicit validation configuration.
///
/// This is the canonical trust-boundary entry point.
///
/// Validation is:
///
/// - deterministic;
/// - read-only;
/// - side-effect free;
/// - independent of hardware;
/// - independent of optimization;
/// - independent of routing;
/// - independent of scheduling.
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

    validate_namespace(&context)?;
    validate_empty_policy(&context, circuit)?;
    validate_operation_count(&context, circuit.operations().len())?;
    validate_operations(&context, circuit)?;

    if config.semantic_checks {
        validate_circuit_semantics(&context, circuit)?;
    }

    Ok(())
}

// =============================================================================
// Policy validation
// =============================================================================

/// Validates the supplied resource policy.
///
/// This function intentionally performs no quantum-domain validation.
pub fn validate_limits(limits: &QuantumIrLimits) -> IrResult<()> {
    limits.validate().map_err(IrError::from)
}

// =============================================================================
// Namespace validation
// =============================================================================

fn validate_namespace(context: &ValidationContext<'_>) -> IrResult<()> {
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

// =============================================================================
// Empty-program policy
// =============================================================================

fn validate_empty_policy(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    if !context.config.allow_empty_circuit && circuit.is_empty() {
        return Err(IrError::InvalidStructure {
            message: "empty circuits are prohibited by validation policy",
        });
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
// Operation traversal
// =============================================================================

fn validate_operations(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    /*
     * This traversal deliberately uses the circuit's existing ordered
     * operation storage.
     *
     * No vector is allocated based on the declared qubit count.
     *
     * Memory therefore remains proportional to the validation state actually
     * required by the operations being inspected.
     */
    for (operation_index, gate) in circuit.operations().iter().enumerate() {
        validate_operation_at(
            context,
            gate,
            operation_index,
        )?;
    }

    Ok(())
}

// =============================================================================
// Public single-operation validation
// =============================================================================

/// Validates one gate against explicit logical/classical namespaces.
///
/// This function is useful for:
///
/// - frontend lowering;
/// - transformation passes;
/// - circuit builders;
/// - generated IR;
/// - incremental validation.
///
/// It does not assume that the gate was constructed through a safe constructor.
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

    validate_namespace(&context)?;

    validate_operation_at(
        &context,
        gate,
        0,
    )
}

/// Explicit gate-oriented alias.
///
/// This exists because callers working at the gate layer often naturally
/// describe the object as a gate rather than an operation.
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

// =============================================================================
// Single operation validation
// =============================================================================

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
    validate_gate_operands(context, gate)?;
    validate_gate_parameters(context, gate)?;
    validate_classical_target(context, gate)?;
    validate_measurement_representation(gate)?;

    if context.config.semantic_checks {
        validate_gate_semantics(context, gate)?;
    }

    Ok(())
}

// =============================================================================
// Gate structure
// =============================================================================

fn validate_gate_structure(gate: &Gate) -> IrResult<()> {
    let kind = gate.kind();

    let expected_operands = kind.operand_count();
    let actual_operands = gate.qubits().len();

    if !expected_operands.accepts(actual_operands) {
        return Err(IrError::Gate(
            IrGateError::InvalidOperandCount {
                gate: kind,
                expected: expected_operands,
                actual: actual_operands,
            },
        ));
    }

    /*
     * `Gate` is intentionally validated again here rather than trusting its
     * constructor. Deserialized or externally generated IR may bypass local
     * constructors.
     */
    validate_duplicate_qubits(gate)?;

    let expected_parameters = kind.parameter_count();
    let actual_parameters = gate.parameter_count();

    if expected_parameters != actual_parameters {
        return Err(IrError::Gate(
            IrGateError::InvalidParameterCount {
                gate: kind,
                expected: expected_parameters,
                actual: actual_parameters,
            },
        ));
    }

    /*
     * Measurement/reset/barrier semantics are validated explicitly below.
     * Keeping them separate avoids accidentally treating special operations
     * as ordinary unitary gates.
     */
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
                super::super::errors::IrQubitError::Duplicate {
                    qubit: qubit.index(),
                },
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Qubit namespace
// =============================================================================

fn validate_gate_operands(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let operand_count = gate.qubits().len();

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
    let count = gate.parameter_count();

    context
        .limits()
        .check_parameters(count)
        .map_err(IrError::from)?;

    for (index, parameter) in gate.parameters().iter().enumerate() {
        /*
         * Parameter-specific semantic validity remains owned by the canonical
         * Parameter type/constructor.
         *
         * This validator intentionally does not reinterpret parameters as
         * hardware quantities.
         */
        if parameter.is_invalid() {
            return Err(IrError::Parameter(
                super::super::errors::IrParameterError::Invalid {
                    index,
                    parameter: parameter.clone(),
                },
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Classical destination validation
// =============================================================================

fn validate_classical_target(
    context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    match gate.classical_target() {
        Some(target) => {
            if !gate.kind().requires_classical_target() {
                return Err(IrError::Gate(
                    IrGateError::UnexpectedClassicalTarget {
                        gate: gate.kind(),
                        target,
                    },
                ));
            }

            if target >= context.num_classical_bits {
                return Err(IrError::Identifier(
                    IrIdentifierError::ClassicalBitOutOfRange {
                        index: target,
                        count: context.num_classical_bits,
                    },
                ));
            }
        }

        None if gate.kind().requires_classical_target() => {
            return Err(IrError::Gate(
                IrGateError::MissingClassicalTarget,
            ));
        }

        None => {}
    }

    Ok(())
}

// =============================================================================
// Measurement representation
// =============================================================================

fn validate_measurement_representation(
    gate: &Gate,
) -> IrResult<()> {
    let has_measurement = gate.measurement().is_some();
    let is_measurement = gate.kind().is_measurement();

    match (is_measurement, has_measurement) {
        (true, false) => Err(IrError::Gate(
            IrGateError::MissingMeasurement,
        )),

        (false, true) => Err(IrError::Gate(
            IrGateError::UnexpectedMeasurement {
                gate: gate.kind(),
            },
        )),

        _ => Ok(()),
    }
}

// =============================================================================
// Semantic gate validation
// =============================================================================

fn validate_gate_semantics(
    _context: &ValidationContext<'_>,
    gate: &Gate,
) -> IrResult<()> {
    let kind = gate.kind();

    /*
     * Special semantic invariants.
     *
     * The cardinality contract is already checked above, but explicit checks
     * here make the invariants robust against future GateKind changes.
     */

    if kind.is_barrier() && gate.qubits().is_empty() {
        return Err(IrError::Gate(
            IrGateError::EmptyBarrier,
        ));
    }

    if kind.is_reset() && gate.qubits().len() != 1 {
        return Err(IrError::Gate(
            IrGateError::InvalidResetOperandCount {
                actual: gate.qubits().len(),
            },
        ));
    }

    if kind.is_measurement() {
        if gate.measurement().is_none() {
            return Err(IrError::Gate(
                IrGateError::MissingMeasurement,
            ));
        }

        if gate.classical_target().is_none() {
            return Err(IrError::Gate(
                IrGateError::MissingClassicalTarget,
            ));
        }
    }

    /*
     * A non-measurement gate must never carry a classical destination.
     * This is intentionally checked here as well as in the representation
     * layer because external/deserialized IR can violate constructor-level
     * invariants.
     */
    if !kind.permits_classical_target()
        && gate.classical_target().is_some()
    {
        let target = gate
            .classical_target()
            .expect("checked is_some above");

        return Err(IrError::Gate(
            IrGateError::UnexpectedClassicalTarget {
                gate: kind,
                target,
            },
        ));
    }

    Ok(())
}

// =============================================================================
// Whole-circuit semantic validation
// =============================================================================

fn validate_circuit_semantics(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    validate_measurement_namespace(context, circuit)?;
    validate_circuit_depth_policy(context, circuit)?;

    Ok(())
}

// =============================================================================
// Measurement namespace
// =============================================================================

fn validate_measurement_namespace(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    let mut measurement_count = 0usize;
    let mut classical_targets = BTreeSet::<usize>::new();

    for gate in circuit.operations() {
        if !gate.kind().is_measurement() {
            continue;
        }

        measurement_count = measurement_count
            .checked_add(1)
            .ok_or_else(|| {
                IrError::from(
                    super::super::limits::LimitsError::ArithmeticOverflow {
                        resource:
                            super::super::limits::ResourceKind::Measurements,
                    },
                )
            })?;

        if let Some(target) = gate.classical_target() {
            /*
             * A classical destination can be written by more than one
             * measurement in some dynamic programs only if the surrounding
             * IR explicitly models that write/ordering semantics.
             *
             * QuantumCircuit is the simple ordered circuit representation,
             * therefore duplicate measurement destinations are rejected here
             * to prevent ambiguous static write semantics.
             */
            if !classical_targets.insert(target) {
                return Err(IrError::Measurement(
                    super::super::errors::IrMeasurementError::DuplicateClassicalTarget {
                        target,
                    },
                ));
            }
        }
    }

    context
        .limits()
        .check_measurements(measurement_count)
        .map_err(IrError::from)?;

    Ok(())
}

// =============================================================================
// Circuit depth
// =============================================================================

fn validate_circuit_depth_policy(
    context: &ValidationContext<'_>,
    circuit: &QuantumCircuit,
) -> IrResult<()> {
    /*
     * Depth is computed sparsely.
     *
     * We intentionally do NOT allocate:
     *
     *     vec![0; circuit.num_qubits()]
     *
     * because a program may declare a very large logical namespace while
     * touching only a small subset of it.
     *
     * The map/set therefore scales with the number of qubits actually touched
     * by the circuit.
     */
    let mut depth_by_qubit =
        std::collections::BTreeMap::<QubitId, usize>::new();

    let mut circuit_depth = 0usize;

    for gate in circuit.operations() {
        if gate.qubits().is_empty() {
            continue;
        }

        let mut operation_depth = 0usize;

        for &qubit in gate.qubits() {
            let current = depth_by_qubit
                .get(&qubit)
                .copied()
                .unwrap_or(0);

            operation_depth = operation_depth.max(current);
        }

        let next_depth = operation_depth
            .checked_add(1)
            .ok_or_else(|| {
                IrError::from(
                    super::super::limits::LimitsError::ArithmeticOverflow {
                        resource:
                            super::super::limits::ResourceKind::CircuitDepth,
                    },
                )
            })?;

        for &qubit in gate.qubits() {
            depth_by_qubit.insert(qubit, next_depth);
        }

        circuit_depth = circuit_depth.max(next_depth);
    }

    context
        .limits()
        .check_depth(circuit_depth)
        .map_err(IrError::from)?;

    Ok(())
}

// =============================================================================
// Validation work accounting
// =============================================================================

fn consume_validation_work(
    context: &ValidationContext<'_>,
    operation_index: usize,
    base_work: usize,
) -> IrResult<()> {
    /*
     * Validation work is accounted explicitly so maliciously large generated
     * IR cannot force unbounded validation effort without crossing the
     * configured policy.
     *
     * The accounting itself uses checked arithmetic.
     */
    let operation_work = operation_index
        .checked_add(base_work)
        .ok_or_else(|| {
            IrError::from(
                super::super::limits::LimitsError::ArithmeticOverflow {
                    resource:
                        super::super::limits::ResourceKind::ValidationSteps,
                },
            )
        })?;

    context
        .limits()
        .check_validation_steps(operation_work)
        .map_err(IrError::from)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn production_config() -> ValidationConfig {
        ValidationConfig::production()
    }

    #[test]
    fn production_configuration_is_strict() {
        let config = production_config();

        assert!(config.strict);
        assert!(config.semantic_checks);
        assert!(config.allow_empty_circuit);
    }

    #[test]
    fn permissive_configuration_disables_optional_semantics() {
        let config =
            ValidationConfig::permissive(
                QuantumIrLimits::production(),
            );

        assert!(!config.strict);
        assert!(!config.semantic_checks);
        assert!(config.allow_empty_circuit);
    }

    #[test]
    fn configuration_builders_are_immutable_style() {
        let base = production_config();

        let changed = base
            .clone()
            .with_strict(false)
            .with_empty_circuits(false)
            .with_semantic_checks(false);

        assert!(base.strict);
        assert!(base.allow_empty_circuit);
        assert!(base.semantic_checks);

        assert!(!changed.strict);
        assert!(!changed.allow_empty_circuit);
        assert!(!changed.semantic_checks);
    }

    #[test]
    fn validation_does_not_require_hardware() {
        /*
         * This test is intentionally architectural.
         *
         * The validation module can be compiled and exercised without any
         * hardware module, backend, simulator, routing implementation or
         * scheduler.
         */
        let _ = ValidationConfig::production();
    }
}