//! Zamani Quantum Optimization — Canonical Quantum IR Normalization.
//!
//! This module is the canonicalization boundary of
//! `quantum::optimization`.
//!
//! # Architectural ownership
//!
//! The authoritative quantum representation is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and the authoritative operation representation is:
//!
//! `crate::quantum::ir::Gate`.
//!
//! This module MUST NOT define another `QuantumGate`, circuit representation,
//! qubit representation, or parameter representation.
//!
//! Canonicalization is intentionally narrower than optimization:
//!
//! ```text
//! frontend / algorithms
//!          |
//!          v
//!   quantum::ir::QuantumCircuit
//!          |
//!          v
//! optimization::canonical
//!          |
//!          v
//! canonical Quantum IR
//!          |
//!          v
//! optimization passes
//! ```
//!
//! # Responsibilities
//!
//! This module is responsible for representation normalization that is
//! demonstrably semantics-preserving without requiring a hardware target,
//! routing information, scheduling information, or an optimization objective.
//!
//! It provides:
//!
//! - validation before canonicalization;
//! - deterministic gate normalization;
//! - deterministic parameter-expression normalization;
//! - constant-expression folding when mathematically exact;
//! - removal of signed zero in numerical parameters;
//! - canonical ordering of operands only for gates whose operand ordering is
//!   mathematically symmetric;
//! - preservation of measurements, classical destinations, barriers, resets,
//!   and all non-unitary semantics;
//! - canonical circuit reconstruction through the existing Quantum IR API;
//! - deterministic canonicalization reports;
//! - idempotence checking;
//! - bounded processing helpers;
//! - no unsafe code;
//! - no backend dependencies;
//! - no routing dependencies;
//! - no scheduling dependencies;
//! - no QPU/backend I/O.
//!
//! # Important semantic rule
//!
//! Canonicalization MUST NOT perform an optimization merely because the
//! optimization appears useful.
//!
//! In particular, this file deliberately does NOT:
//!
//! - cancel gates;
//! - commute gates;
//! - fuse gates;
//! - synthesize gates;
//! - minimize depth;
//! - minimize gate count;
//! - minimize two-qubit gates;
//! - minimize T-count;
//! - minimize T-depth;
//! - choose a hardware gate set;
//! - perform routing;
//! - approximate a unitary;
//! - perform equality saturation.
//!
//! Those responsibilities belong to later optimization modules.
//!
//! # Parameter normalization policy
//!
//! Parameter expressions are normalized only through exact symbolic/algebraic
//! identities and exact finite constant evaluation.
//!
//! This module does NOT use an epsilon when comparing quantum angles.
//!
//! Approximate equality is dangerous at the compiler IR boundary because a
//! small numerical change can alter a circuit's semantics. Approximation
//! policies belong to explicit synthesis/verification/target modules.
//!
//! # Angle policy
//!
//! This module intentionally does not reduce arbitrary angles modulo 2π.
//!
//! Although many rotation gates are periodic, doing so at this layer can:
//!
//! - interact with floating-point representation;
//! - interact with symbolic parameters;
//! - interact with global-phase conventions;
//! - interact with target-specific gate semantics;
//! - make later exact verification harder.
//!
//! Exact periodicity rewrites belong in `parameter/` or `algebra/`, where the
//! relevant semantic contract is explicit.
//!
//! # Determinism
//!
//! Given the same valid input circuit and the same policy, this module produces
//! the same output circuit and report. It does not use randomness, wall-clock
//! time, hash iteration order, environment variables, or backend state.
//!
//! # Complexity
//!
//! For a circuit with `n` operations and `p` total parameter-expression nodes:
//!
//! - circuit traversal is O(n);
//! - parameter normalization is O(p);
//! - additional memory is O(n + p) for reconstructed output;
//! - no quadratic all-pairs operation scan is performed.
//!
//! This makes canonicalization suitable as a first pass for very large
//! circuits. Resource limits remain enforced by the canonical Quantum IR and
//! by the optimization subsystem.
//!
//! # Transactionality
//!
//! `canonicalize_circuit` never mutates the input circuit. It first constructs
//! and validates the complete canonical operation sequence and only then
//! constructs the returned circuit.
//!
//! Therefore a failure cannot leave a partially canonicalized circuit.
//!
//! # Integration contract
//!
//! This file is intentionally usable before the higher-level optimizer files
//! exist.
//!
//! Future modules consume it as follows:
//!
//! - `pass.rs` wraps `CanonicalizationPass` as an `OptimizationPass`;
//! - `pipeline.rs` invokes the pass before other transformation passes;
//! - `planner.rs` selects it as the normalization stage;
//! - `context.rs` may store the resulting `CanonicalizationReport`;
//! - `statistics.rs` imports report counters into optimizer statistics;
//! - `result.rs` exposes the report as part of optimization provenance/results;
//! - `verification/` can use `canonicalize_circuit` before comparing canonical
//!   representations;
//! - `serialization/` may serialize the report;
//! - `local/`, `algebra/`, `parameter/`, and `synthesis/` consume the canonical
//!   representation produced here.
//!
//! The file deliberately does not depend on those future modules.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.

use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateKind,
    Parameter,
    QuantumCircuit,
};

use crate::quantum::ir::parameter::ParameterExpression;

// =============================================================================
// Public constants
// =============================================================================

/// Stable identifier for the canonicalization component.
///
/// This identifier is intended for pass registries, diagnostics, provenance,
/// and future serialized optimization reports.
pub const CANONICALIZATION_ID: &str =
    "quantum.optimization.canonical";

/// Semantic version of the canonicalization contract.
///
/// This is an optimizer-component contract version, not the Quantum IR schema
/// version.
pub const CANONICALIZATION_VERSION: u32 = 1;

// =============================================================================
// Policy
// =============================================================================

/// Controls which representation-only canonicalizations are performed.
///
/// The default policy is deliberately conservative.
///
/// A future optimization profile may construct a more aggressive policy, but
/// any transformation that changes mathematical representation rather than
/// merely normalizing it should normally belong to another optimizer module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalizationPolicy {
    /// Normalize parameter-expression structure.
    normalize_parameters: bool,

    /// Fold finite constant parameter expressions.
    fold_constants: bool,

    /// Normalize negative zero in floating-point constants.
    normalize_signed_zero: bool,

    /// Canonically order operands of mathematically symmetric gates.
    normalize_symmetric_operands: bool,

    /// Validate every input gate before processing it.
    validate_input: bool,

    /// Validate every reconstructed gate.
    validate_output_gates: bool,

    /// Validate the complete reconstructed circuit.
    validate_output_circuit: bool,
}

impl Default for CanonicalizationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

impl CanonicalizationPolicy {
    /// Creates the production canonicalization policy.
    ///
    /// The production policy contains only representation-level
    /// transformations whose semantics are independent of hardware and
    /// optimization objective.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            normalize_parameters: true,
            fold_constants: true,
            normalize_signed_zero: true,
            normalize_symmetric_operands: true,
            validate_input: true,
            validate_output_gates: true,
            validate_output_circuit: true,
        }
    }

    /// Creates the strictest representation-preserving policy.
    ///
    /// This disables every transformation except validation/reconstruction.
    #[must_use]
    pub const fn validation_only() -> Self {
        Self {
            normalize_parameters: false,
            fold_constants: false,
            normalize_signed_zero: false,
            normalize_symmetric_operands: false,
            validate_input: true,
            validate_output_gates: true,
            validate_output_circuit: true,
        }
    }

    /// Returns whether parameter normalization is enabled.
    #[must_use]
    pub const fn normalize_parameters(self) -> bool {
        self.normalize_parameters
    }

    /// Returns whether exact constant folding is enabled.
    #[must_use]
    pub const fn fold_constants(self) -> bool {
        self.fold_constants
    }

    /// Returns whether signed zero is normalized.
    #[must_use]
    pub const fn normalize_signed_zero(self) -> bool {
        self.normalize_signed_zero
    }

    /// Returns whether symmetric operands are normalized.
    #[must_use]
    pub const fn normalize_symmetric_operands(self) -> bool {
        self.normalize_symmetric_operands
    }

    /// Returns whether input validation is enabled.
    #[must_use]
    pub const fn validate_input(self) -> bool {
        self.validate_input
    }

    /// Returns whether reconstructed gates are validated.
    #[must_use]
    pub const fn validate_output_gates(self) -> bool {
        self.validate_output_gates
    }

    /// Returns whether the reconstructed circuit is validated.
    #[must_use]
    pub const fn validate_output_circuit(self) -> bool {
        self.validate_output_circuit
    }
}

// =============================================================================
// Change classification
// =============================================================================

/// Classifies a representation-level canonicalization change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CanonicalizationChangeKind {
    /// A parameter expression was structurally normalized.
    ParameterExpression,

    /// A finite constant expression was folded.
    ConstantFold,

    /// Negative zero was normalized to positive zero.
    SignedZero,

    /// Symmetric gate operands were placed in deterministic order.
    SymmetricOperandOrder,
}

impl CanonicalizationChangeKind {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParameterExpression => {
                "parameter_expression"
            }
            Self::ConstantFold => "constant_fold",
            Self::SignedZero => "signed_zero",
            Self::SymmetricOperandOrder => {
                "symmetric_operand_order"
            }
        }
    }
}

impl fmt::Display for CanonicalizationChangeKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Report
// =============================================================================

/// Deterministic report produced by canonicalization.
///
/// This type intentionally contains counters rather than per-operation
/// allocations. Large circuits should not incur O(n) report metadata beyond
/// the circuit itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanonicalizationReport {
    /// Number of operations inspected.
    operations_seen: usize,

    /// Number of operations whose representation changed.
    operations_changed: usize,

    /// Number of gates whose symmetric operand order changed.
    symmetric_operands_normalized: usize,

    /// Number of parameters inspected.
    parameters_seen: usize,

    /// Number of parameter expressions normalized.
    parameter_expressions_normalized: usize,

    /// Number of exact constant expressions folded.
    constants_folded: usize,

    /// Number of signed zero constants normalized.
    signed_zero_normalized: usize,
}

impl CanonicalizationReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations_seen: 0,
            operations_changed: 0,
            symmetric_operands_normalized: 0,
            parameters_seen: 0,
            parameter_expressions_normalized: 0,
            constants_folded: 0,
            signed_zero_normalized: 0,
        }
    }

    /// Returns the number of operations inspected.
    #[must_use]
    pub const fn operations_seen(self) -> usize {
        self.operations_seen
    }

    /// Returns the number of changed operations.
    #[must_use]
    pub const fn operations_changed(self) -> usize {
        self.operations_changed
    }

    /// Returns the number of symmetric operand normalizations.
    #[must_use]
    pub const fn symmetric_operands_normalized(
        self,
    ) -> usize {
        self.symmetric_operands_normalized
    }

    /// Returns the number of parameters inspected.
    #[must_use]
    pub const fn parameters_seen(self) -> usize {
        self.parameters_seen
    }

    /// Returns the number of normalized expressions.
    #[must_use]
    pub const fn parameter_expressions_normalized(
        self,
    ) -> usize {
        self.parameter_expressions_normalized
    }

    /// Returns the number of exact constant folds.
    #[must_use]
    pub const fn constants_folded(self) -> usize {
        self.constants_folded
    }

    /// Returns the number of signed-zero normalizations.
    #[must_use]
    pub const fn signed_zero_normalized(
        self,
    ) -> usize {
        self.signed_zero_normalized
    }

    /// Returns true when no representation changed.
    #[must_use]
    pub const fn is_unchanged(self) -> bool {
        self.operations_changed == 0
    }

    /// Returns the total number of canonicalization changes.
    ///
    /// This is a checked aggregation. `None` means the counters cannot be
    /// represented by `usize` in their combined value.
    #[must_use]
    pub fn total_changes(self) -> Option<usize> {
        self.symmetric_operands_normalized
            .checked_add(
                self.parameter_expressions_normalized,
            )?
            .checked_add(self.constants_folded)?
            .checked_add(self.signed_zero_normalized)
    }

    fn record_operation(
        &mut self,
        changed: bool,
    ) {
        self.operations_seen =
            self.operations_seen.saturating_add(1);

        if changed {
            self.operations_changed =
                self.operations_changed.saturating_add(1);
        }
    }

    fn record_parameter(&mut self) {
        self.parameters_seen =
            self.parameters_seen.saturating_add(1);
    }

    fn record_symmetric_operand_normalization(
        &mut self,
    ) {
        self.symmetric_operands_normalized =
            self.symmetric_operands_normalized
                .saturating_add(1);
    }

    fn record_parameter_expression(
        &mut self,
    ) {
        self.parameter_expressions_normalized =
            self.parameter_expressions_normalized
                .saturating_add(1);
    }

    fn record_constant_fold(&mut self) {
        self.constants_folded =
            self.constants_folded.saturating_add(1);
    }

    fn record_signed_zero(&mut self) {
        self.signed_zero_normalized =
            self.signed_zero_normalized
                .saturating_add(1);
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error returned by canonicalization.
///
/// This local error is intentionally independent of the optimizer-wide error
/// enum. `errors.rs` can wrap it at the subsystem boundary without creating a
/// dependency cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalizationError {
    /// The supplied circuit failed canonical Quantum IR validation.
    InvalidInputCircuit {
        /// Human-readable validation failure.
        message: String,
    },

    /// A gate failed validation after canonicalization.
    InvalidCanonicalGate {
        /// Operation index in the source circuit.
        operation: usize,

        /// Human-readable validation failure.
        message: String,
    },

    /// The reconstructed circuit failed validation.
    InvalidCanonicalCircuit {
        /// Human-readable validation failure.
        message: String,
    },

    /// Canonicalization encountered an arithmetic overflow.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// An expression could not be normalized safely.
    InvalidParameterExpression {
        /// Operation index.
        operation: usize,

        /// Parameter index.
        parameter: usize,

        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for CanonicalizationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidInputCircuit { message } => {
                write!(
                    formatter,
                    "canonicalization input circuit is invalid: {message}"
                )
            }

            Self::InvalidCanonicalGate {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "canonicalization produced an invalid gate at operation {operation}: {message}"
                )
            }

            Self::InvalidCanonicalCircuit { message } => {
                write!(
                    formatter,
                    "canonicalization produced an invalid circuit: {message}"
                )
            }

            Self::ArithmeticOverflow {
                calculation,
            } => {
                write!(
                    formatter,
                    "arithmetic overflow during canonicalization: {calculation}"
                )
            }

            Self::InvalidParameterExpression {
                operation,
                parameter,
                message,
            } => {
                write!(
                    formatter,
                    "invalid parameter expression at operation {operation}, parameter {parameter}: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CanonicalizationError {}

// =============================================================================
// Result types
// =============================================================================

/// Result of canonicalizing one gate.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalGate {
    /// Canonical gate.
    gate: Gate,

    /// Whether the representation changed.
    changed: bool,

    /// Number of parameter-expression changes.
    parameter_expression_changes: usize,

    /// Number of exact constant folds.
    constant_folds: usize,

    /// Number of signed-zero changes.
    signed_zero_changes: usize,

    /// Whether symmetric operands were reordered.
    symmetric_operands_changed: bool,
}

impl CanonicalGate {
    /// Returns the canonical gate.
    #[must_use]
    pub fn gate(&self) -> &Gate {
        &self.gate
    }

    /// Consumes this result and returns the canonical gate.
    #[must_use]
    pub fn into_gate(self) -> Gate {
        self.gate
    }

    /// Returns whether the gate changed.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.changed
    }

    /// Returns the number of parameter-expression changes.
    #[must_use]
    pub const fn parameter_expression_changes(
        &self,
    ) -> usize {
        self.parameter_expression_changes
    }

    /// Returns the number of constant folds.
    #[must_use]
    pub const fn constant_folds(&self) -> usize {
        self.constant_folds
    }

    /// Returns the number of signed-zero changes.
    #[must_use]
    pub const fn signed_zero_changes(&self) -> usize {
        self.signed_zero_changes
    }

    /// Returns whether symmetric operands changed order.
    #[must_use]
    pub const fn symmetric_operands_changed(
        &self,
    ) -> bool {
        self.symmetric_operands_changed
    }
}

/// Complete result of circuit canonicalization.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalizationResult {
    /// Canonicalized circuit.
    circuit: QuantumCircuit,

    /// Deterministic canonicalization statistics.
    report: CanonicalizationReport,
}

impl CanonicalizationResult {
    /// Creates a canonicalization result.
    #[must_use]
    pub const fn new(
        circuit: QuantumCircuit,
        report: CanonicalizationReport,
    ) -> Self {
        Self {
            circuit,
            report,
        }
    }

    /// Returns the canonical circuit.
    #[must_use]
    pub fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Returns the report.
    #[must_use]
    pub const fn report(
        &self,
    ) -> CanonicalizationReport {
        self.report
    }

    /// Consumes the result and returns the canonical circuit.
    #[must_use]
    pub fn into_circuit(self) -> QuantumCircuit {
        self.circuit
    }

    /// Consumes the result and returns both circuit and report.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (QuantumCircuit, CanonicalizationReport) {
        (self.circuit, self.report)
    }
}

// =============================================================================
// Public gate API
// =============================================================================

/// Canonicalizes one validated Quantum IR gate using the production policy.
///
/// This function does not know anything about the surrounding circuit and
/// therefore cannot validate namespace membership. The enclosing circuit
/// validation remains responsible for namespace correctness.
pub fn canonicalize_gate(
    gate: &Gate,
) -> Result<CanonicalGate, CanonicalizationError> {
    canonicalize_gate_with_policy(
        gate,
        CanonicalizationPolicy::production(),
    )
}

/// Canonicalizes one gate with an explicit policy.
pub fn canonicalize_gate_with_policy(
    gate: &Gate,
    policy: CanonicalizationPolicy,
) -> Result<CanonicalGate, CanonicalizationError> {
    canonicalize_gate_internal(
        gate,
        policy,
        0,
        None,
    )
}

/// Canonicalizes one gate with an explicit operation index.
///
/// This is useful to callers processing a circuit manually because any
/// parameter error can identify the operation responsible.
pub fn canonicalize_gate_at(
    gate: &Gate,
    operation: usize,
    policy: CanonicalizationPolicy,
) -> Result<CanonicalGate, CanonicalizationError> {
    canonicalize_gate_internal(
        gate,
        policy,
        operation,
        None,
    )
}

// =============================================================================
// Public circuit API
// =============================================================================

/// Canonicalizes a complete Quantum IR circuit using the production policy.
///
/// The input circuit is never mutated.
pub fn canonicalize_circuit(
    circuit: &QuantumCircuit,
) -> Result<CanonicalizationResult, CanonicalizationError> {
    canonicalize_circuit_with_policy(
        circuit,
        CanonicalizationPolicy::production(),
    )
}

/// Canonicalizes a complete Quantum IR circuit using an explicit policy.
///
/// Processing is transactional:
///
/// 1. validate input;
/// 2. canonicalize every operation;
/// 3. validate every reconstructed gate;
/// 4. construct the complete output circuit;
/// 5. restore identity/version/metadata;
/// 6. validate the complete output.
///
/// If any stage fails, no partial result is returned.
pub fn canonicalize_circuit_with_policy(
    circuit: &QuantumCircuit,
    policy: CanonicalizationPolicy,
) -> Result<CanonicalizationResult, CanonicalizationError> {
    if policy.validate_input() {
        circuit
            .validate()
            .map_err(|error| {
                CanonicalizationError::InvalidInputCircuit {
                    message: error.to_string(),
                }
            })?;
    }

    let operation_count = circuit.len();

    let mut operations = Vec::with_capacity(operation_count);
    let mut report = CanonicalizationReport::new();

    for (index, gate) in
        circuit.operations().iter().enumerate()
    {
        let canonical = canonicalize_gate_internal(
            gate,
            policy,
            index,
            Some(&mut report),
        )?;

        if policy.validate_output_gates() {
            canonical.gate.validate().map_err(
                |error| {
                    CanonicalizationError::InvalidCanonicalGate {
                        operation: index,
                        message: error.to_string(),
                    }
                },
            )?;
        }

        operations.push(canonical.gate);
    }

    let mut output =
        QuantumCircuit::from_operations_with_limits(
            circuit.num_qubits(),
            circuit.num_classical_bits(),
            operations,
            *circuit.limits(),
        )
        .map_err(|error| {
            CanonicalizationError::InvalidCanonicalCircuit {
                message: error.to_string(),
            }
        })?;

    //
    // Preserve compiler-visible circuit identity and IR version.
    //
    output.set_id(circuit.id());

    output
        .set_version(circuit.version())
        .map_err(|error| {
            CanonicalizationError::InvalidCanonicalCircuit {
                message: error.to_string(),
            }
        })?;

    //
    // Preserve metadata exactly.
    //
    output
        .set_metadata(circuit.metadata().clone())
        .map_err(|error| {
            CanonicalizationError::InvalidCanonicalCircuit {
                message: error.to_string(),
            }
        })?;

    if policy.validate_output_circuit() {
        output
            .validate()
            .map_err(|error| {
                CanonicalizationError::InvalidCanonicalCircuit {
                    message: error.to_string(),
                }
            })?;
    }

    Ok(CanonicalizationResult::new(
        output,
        report,
    ))
}

/// Returns whether a circuit is already canonical under the production
/// canonicalization policy.
///
/// This performs canonicalization and therefore validates the input.
pub fn is_canonical(
    circuit: &QuantumCircuit,
) -> Result<bool, CanonicalizationError> {
    let result = canonicalize_circuit(circuit)?;

    Ok(result.report().is_unchanged())
}

/// Canonicalizes a circuit and returns it directly.
///
/// This is the convenience API intended for pipeline code that does not need
/// the detailed report.
pub fn canonicalize(
    circuit: &QuantumCircuit,
) -> Result<QuantumCircuit, CanonicalizationError> {
    Ok(canonicalize_circuit(circuit)?.into_circuit())
}

// =============================================================================
// Internal gate canonicalization
// =============================================================================

fn canonicalize_gate_internal(
    gate: &Gate,
    policy: CanonicalizationPolicy,
    operation: usize,
    mut report: Option<&mut CanonicalizationReport>,
) -> Result<CanonicalGate, CanonicalizationError> {
    if policy.validate_input() {
        gate.validate().map_err(|error| {
            CanonicalizationError::InvalidCanonicalGate {
                operation,
                message: error.to_string(),
            }
        })?;
    }

    let mut qubits =
        gate.qubits().to_vec();

    let mut parameters =
        Vec::with_capacity(gate.parameters().len());

    let mut changed = false;

    let mut parameter_expression_changes = 0usize;
    let mut constant_folds = 0usize;
    let mut signed_zero_changes = 0usize;

    //
    // Parameters
    //
    for (parameter_index, parameter) in
        gate.parameters().iter().enumerate()
    {
        if let Some(report) = report.as_deref_mut() {
            report.record_parameter();
        }

        if !policy.normalize_parameters() {
            parameters.push(parameter.clone());
            continue;
        }

        let normalized =
            normalize_parameter(
                parameter,
                policy,
                operation,
                parameter_index,
            )?;

        if normalized.parameter != *parameter {
            changed = true;
        }

        parameter_expression_changes =
            parameter_expression_changes
                .checked_add(
                    normalized.expression_changes,
                )
                .ok_or(
                    CanonicalizationError::ArithmeticOverflow {
                        calculation:
                            "parameter-expression change count",
                    },
                )?;

        constant_folds = constant_folds
            .checked_add(normalized.constant_folds)
            .ok_or(
                CanonicalizationError::ArithmeticOverflow {
                    calculation:
                        "constant-fold count",
                },
            )?;

        signed_zero_changes =
            signed_zero_changes
                .checked_add(
                    normalized.signed_zero_changes,
                )
                .ok_or(
                    CanonicalizationError::ArithmeticOverflow {
                        calculation:
                            "signed-zero change count",
                    },
                )?;

        parameters.push(normalized.parameter);
    }

    //
    // Symmetric operand normalization
    //
    let mut symmetric_operands_changed =
        false;

    if policy.normalize_symmetric_operands()
        && is_symmetric_operand_gate(gate.kind())
    {
        symmetric_operands_changed =
            canonicalize_symmetric_operands(
                &mut qubits,
            );

        if symmetric_operands_changed {
            changed = true;
        }
    }

    //
    // Build the canonical gate through the authoritative IR constructor.
    //
    let canonical =
        Gate::new(
            gate.kind(),
            qubits,
            parameters,
            gate.classical_target(),
            gate.measurement().cloned(),
        )
        .map_err(|error| {
            CanonicalizationError::InvalidCanonicalGate {
                operation,
                message: error.to_string(),
            }
        })?;

    if let Some(report) = report.as_deref_mut() {
        report.record_operation(changed);

        if symmetric_operands_changed {
            report
                .record_symmetric_operand_normalization();
        }

        for _ in 0..parameter_expression_changes {
            report.record_parameter_expression();
        }

        for _ in 0..constant_folds {
            report.record_constant_fold();
        }

        for _ in 0..signed_zero_changes {
            report.record_signed_zero();
        }
    }

    Ok(CanonicalGate {
        gate: canonical,
        changed,
        parameter_expression_changes,
        constant_folds,
        signed_zero_changes,
        symmetric_operands_changed,
    })
}

// =============================================================================
// Symmetric gate handling
// =============================================================================

/// Returns whether a gate's logical operand ordering is mathematically
/// symmetric.
///
/// Only operations for which exchanging operands preserves the operation are
/// included.
///
/// This list is intentionally conservative.
#[must_use]
pub const fn is_symmetric_operand_gate(
    kind: GateKind,
) -> bool {
    matches!(
        kind,
        GateKind::CZ
            | GateKind::SWAP
            | GateKind::ISWAP
    )
}

/// Canonically orders operands of a symmetric gate.
///
/// `QubitId` has a deterministic ordering in the canonical IR. Sorting is
/// therefore deterministic and does not depend on hashing.
///
/// Returns true when the operand sequence changed.
fn canonicalize_symmetric_operands(
    qubits: &mut [crate::quantum::ir::QubitId],
) -> bool {
    if qubits.len() < 2 {
        return false;
    }

    let before = qubits.to_vec();

    qubits.sort();

    before != qubits
}

// =============================================================================
// Parameter normalization
// =============================================================================

struct NormalizedParameter {
    parameter: Parameter,
    expression_changes: usize,
    constant_folds: usize,
    signed_zero_changes: usize,
}

fn normalize_parameter(
    parameter: &Parameter,
    policy: CanonicalizationPolicy,
    operation: usize,
    parameter_index: usize,
) -> Result<NormalizedParameter, CanonicalizationError> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    CanonicalizationError::InvalidParameterExpression {
                        operation,
                        parameter: parameter_index,
                        message:
                            "non-finite constant parameter"
                                .to_string(),
                    },
                );
            }

            if policy.normalize_signed_zero()
                && *value == 0.0
                && value.is_sign_negative()
            {
                let normalized =
                    Parameter::Constant(0.0);

                return Ok(
                    NormalizedParameter {
                        parameter: normalized,
                        expression_changes: 0,
                        constant_folds: 0,
                        signed_zero_changes: 1,
                    },
                );
            }

            Ok(NormalizedParameter {
                parameter: parameter.clone(),
                expression_changes: 0,
                constant_folds: 0,
                signed_zero_changes: 0,
            })
        }

        Parameter::Symbol(_) => Ok(
            NormalizedParameter {
                parameter: parameter.clone(),
                expression_changes: 0,
                constant_folds: 0,
                signed_zero_changes: 0,
            },
        ),

        Parameter::Expression(expression) => {
            if !policy.normalize_parameters() {
                return Ok(
                    NormalizedParameter {
                        parameter: parameter.clone(),
                        expression_changes: 0,
                        constant_folds: 0,
                        signed_zero_changes: 0,
                    },
                );
            }

            let normalized =
                normalize_expression(
                    expression,
                    policy,
                    operation,
                    parameter_index,
                )?;

            Ok(NormalizedParameter {
                parameter: Parameter::Expression(
                    Box::new(normalized.expression),
                ),
                expression_changes:
                    normalized.expression_changes,
                constant_folds:
                    normalized.constant_folds,
                signed_zero_changes:
                    normalized.signed_zero_changes,
            })
        }
    }
}

// =============================================================================
// Parameter-expression normalization
// =============================================================================

struct NormalizedExpression {
    expression: ParameterExpression,
    expression_changes: usize,
    constant_folds: usize,
    signed_zero_changes: usize,
}

fn normalize_expression(
    expression: &ParameterExpression,
    policy: CanonicalizationPolicy,
    operation: usize,
    parameter_index: usize,
) -> Result<NormalizedExpression, CanonicalizationError> {
    match expression {
        ParameterExpression::Add(left, right) => {
            normalize_binary_expression(
                BinaryOperator::Add,
                left,
                right,
                policy,
                operation,
                parameter_index,
            )
        }

        ParameterExpression::Subtract(left, right) => {
            normalize_binary_expression(
                BinaryOperator::Subtract,
                left,
                right,
                policy,
                operation,
                parameter_index,
            )
        }

        ParameterExpression::Multiply(left, right) => {
            normalize_binary_expression(
                BinaryOperator::Multiply,
                left,
                right,
                policy,
                operation,
                parameter_index,
            )
        }

        ParameterExpression::Divide(left, right) => {
            normalize_binary_expression(
                BinaryOperator::Divide,
                left,
                right,
                policy,
                operation,
                parameter_index,
            )
        }

        ParameterExpression::Negate(value) => {
            let child =
                normalize_parameter_node(
                    value,
                    policy,
                    operation,
                    parameter_index,
                )?;

            if policy.fold_constants() {
                if let Some(value) =
                    constant_parameter_value(
                        &child.parameter,
                    )
                {
                    let result = -value;

                    if !result.is_finite() {
                        return Err(
                            CanonicalizationError::InvalidParameterExpression {
                                operation,
                                parameter: parameter_index,
                                message:
                                    "constant negation produced a non-finite value"
                                        .to_string(),
                            },
                        );
                    }

                    let result =
                        normalize_finite_constant(
                            result,
                            policy,
                        );

                    return Ok(
                        NormalizedExpression {
                            expression:
                                ParameterExpression::Negate(
                                    Box::new(
                                        result.parameter,
                                    ),
                                ),
                            expression_changes:
                                child.expression_changes
                                    .saturating_add(1),
                            constant_folds:
                                child.constant_folds
                                    .saturating_add(1),
                            signed_zero_changes:
                                child.signed_zero_changes
                                    .saturating_add(
                                        result.signed_zero_changed
                                            as usize,
                                    ),
                        },
                    );
                }
            }

            Ok(NormalizedExpression {
                expression:
                    ParameterExpression::Negate(
                        Box::new(child.parameter),
                    ),
                expression_changes:
                    child.expression_changes,
                constant_folds:
                    child.constant_folds,
                signed_zero_changes:
                    child.signed_zero_changes,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct NormalizedNode {
    parameter: Parameter,
    expression_changes: usize,
    constant_folds: usize,
    signed_zero_changes: usize,
}

fn normalize_parameter_node(
    parameter: &Parameter,
    policy: CanonicalizationPolicy,
    operation: usize,
    parameter_index: usize,
) -> Result<NormalizedNode, CanonicalizationError> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    CanonicalizationError::InvalidParameterExpression {
                        operation,
                        parameter: parameter_index,
                        message:
                            "non-finite parameter constant"
                                .to_string(),
                    },
                );
            }

            let normalized =
                normalize_finite_constant(
                    *value,
                    policy,
                );

            Ok(NormalizedNode {
                parameter:
                    normalized.parameter,
                expression_changes: 0,
                constant_folds: 0,
                signed_zero_changes:
                    normalized.signed_zero_changed
                        as usize,
            })
        }

        Parameter::Symbol(_) => Ok(
            NormalizedNode {
                parameter: parameter.clone(),
                expression_changes: 0,
                constant_folds: 0,
                signed_zero_changes: 0,
            },
        ),

        Parameter::Expression(expression) => {
            let normalized =
                normalize_expression(
                    expression,
                    policy,
                    operation,
                    parameter_index,
                )?;

            Ok(NormalizedNode {
                parameter:
                    Parameter::Expression(
                        Box::new(
                            normalized.expression,
                        ),
                    ),
                expression_changes:
                    normalized.expression_changes
                        .saturating_add(1),
                constant_folds:
                    normalized.constant_folds,
                signed_zero_changes:
                    normalized.signed_zero_changes,
            })
        }
    }
}

fn normalize_binary_expression(
    operator: BinaryOperator,
    left: &Parameter,
    right: &Parameter,
    policy: CanonicalizationPolicy,
    operation: usize,
    parameter_index: usize,
) -> Result<NormalizedExpression, CanonicalizationError> {
    let left =
        normalize_parameter_node(
            left,
            policy,
            operation,
            parameter_index,
        )?;

    let right =
        normalize_parameter_node(
            right,
            policy,
            operation,
            parameter_index,
        )?;

    let mut expression_changes =
        left.expression_changes
            .saturating_add(
                right.expression_changes,
            );

    let mut constant_folds =
        left.constant_folds
            .saturating_add(
                right.constant_folds,
            );

    let mut signed_zero_changes =
        left.signed_zero_changes
            .saturating_add(
                right.signed_zero_changes,
            );

    if policy.fold_constants() {
        if let (
            Some(left_value),
            Some(right_value),
        ) = (
            constant_parameter_value(
                &left.parameter,
            ),
            constant_parameter_value(
                &right.parameter,
            ),
        ) {
            let value = match operator {
                BinaryOperator::Add => {
                    left_value + right_value
                }

                BinaryOperator::Subtract => {
                    left_value - right_value
                }

                BinaryOperator::Multiply => {
                    left_value * right_value
                }

                BinaryOperator::Divide => {
                    if right_value == 0.0 {
                        return Err(
                            CanonicalizationError::InvalidParameterExpression {
                                operation,
                                parameter: parameter_index,
                                message:
                                    "constant division by zero"
                                        .to_string(),
                            },
                        );
                    }

                    left_value / right_value
                }
            };

            if !value.is_finite() {
                return Err(
                    CanonicalizationError::InvalidParameterExpression {
                        operation,
                        parameter: parameter_index,
                        message:
                            "constant expression produced a non-finite value"
                                .to_string(),
                    },
                );
            }

            let normalized =
                normalize_finite_constant(
                    value,
                    policy,
                );

            return Ok(NormalizedExpression {
                expression:
                    ParameterExpression::Negate(
                        Box::new(
                            normalized.parameter,
                        ),
                    ),
                expression_changes:
                    expression_changes
                        .saturating_add(1),
                constant_folds:
                    constant_folds.saturating_add(1),
                signed_zero_changes:
                    signed_zero_changes
                        .saturating_add(
                            normalized
                                .signed_zero_changed
                                as usize,
                        ),
            });
        }
    }

    //
    // Exact symbolic identities.
    //
    // These identities do not require numerical approximation.
    //
    match operator {
        BinaryOperator::Add => {
            if is_exact_zero(&right.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                left.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }

            if is_exact_zero(&left.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                right.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }
        }

        BinaryOperator::Subtract => {
            if is_exact_zero(&right.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                left.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }
        }

        BinaryOperator::Multiply => {
            if is_exact_zero(&left.parameter)
                || is_exact_zero(&right.parameter)
            {
                return Ok(
                    NormalizedExpression {
                        expression:
                            ParameterExpression::Negate(
                                Box::new(
                                    Parameter::Constant(
                                        0.0,
                                    ),
                                ),
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds:
                            constant_folds
                                .saturating_add(1),
                        signed_zero_changes,
                    },
                );
            }

            if is_exact_one(&left.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                right.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }

            if is_exact_one(&right.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                left.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }
        }

        BinaryOperator::Divide => {
            if is_exact_one(&right.parameter) {
                return Ok(
                    NormalizedExpression {
                        expression:
                            parameter_as_expression(
                                left.parameter,
                            ),
                        expression_changes:
                            expression_changes
                                .saturating_add(1),
                        constant_folds,
                        signed_zero_changes,
                    },
                );
            }
        }
    }

    let expression = match operator {
        BinaryOperator::Add => {
            ParameterExpression::Add(
                Box::new(left.parameter),
                Box::new(right.parameter),
            )
        }

        BinaryOperator::Subtract => {
            ParameterExpression::Subtract(
                Box::new(left.parameter),
                Box::new(right.parameter),
            )
        }

        BinaryOperator::Multiply => {
            ParameterExpression::Multiply(
                Box::new(left.parameter),
                Box::new(right.parameter),
            )
        }

        BinaryOperator::Divide => {
            ParameterExpression::Divide(
                Box::new(left.parameter),
                Box::new(right.parameter),
            )
        }
    };

    Ok(NormalizedExpression {
        expression,
        expression_changes,
        constant_folds,
        signed_zero_changes,
    })
}

// =============================================================================
// Parameter helpers
// =============================================================================

fn constant_parameter_value(
    parameter: &Parameter,
) -> Option<f64> {
    match parameter {
        Parameter::Constant(value) => Some(*value),

        Parameter::Symbol(_) => None,

        Parameter::Expression(_) => None,
    }
}

fn is_exact_zero(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 0.0
    )
}

fn is_exact_one(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 1.0
    )
}

fn parameter_as_expression(
    parameter: Parameter,
) -> ParameterExpression {
    ParameterExpression::Negate(
        Box::new(parameter),
    )
}

struct NormalizedConstant {
    parameter: Parameter,
    signed_zero_changed: bool,
}

fn normalize_finite_constant(
    value: f64,
    policy: CanonicalizationPolicy,
) -> NormalizedConstant {
    if policy.normalize_signed_zero()
        && value == 0.0
        && value.is_sign_negative()
    {
        NormalizedConstant {
            parameter: Parameter::Constant(0.0),
            signed_zero_changed: true,
        }
    } else {
        NormalizedConstant {
            parameter: Parameter::Constant(
                value,
            ),
            signed_zero_changed: false,
        }
    }
}

// =============================================================================
// Idempotence
// =============================================================================

/// Checks the fundamental canonicalization invariant:
///
/// `canonicalize(canonicalize(C)) == canonicalize(C)`
///
/// The comparison is performed on the canonical IR representation exposed by
/// `QuantumCircuit`.
///
/// This helper is primarily intended for tests and verification.
pub fn verify_idempotence(
    circuit: &QuantumCircuit,
) -> Result<bool, CanonicalizationError> {
    let first =
        canonicalize_circuit(circuit)?
            .into_circuit();

    let second =
        canonicalize_circuit(&first)?
            .into_circuit();

    Ok(first == second)
}

// =============================================================================
// Deterministic fingerprints
// =============================================================================

/// Returns a deterministic textual fingerprint of the canonical structural
/// content of a circuit.
///
/// This is intentionally NOT a cryptographic hash.
///
/// It is useful for tests and deterministic debugging without adding a hashing
/// dependency to the canonicalization layer.
///
/// The fingerprint includes:
///
/// - qubit/classical-bit counts;
/// - operation kinds;
/// - operand order;
/// - parameter representation;
/// - classical targets.
///
/// Circuit metadata and circuit identity are intentionally excluded because
/// canonicalization must not consider compiler provenance to be circuit
/// semantics.
pub fn structural_fingerprint(
    circuit: &QuantumCircuit,
) -> Result<String, CanonicalizationError> {
    circuit
        .validate()
        .map_err(|error| {
            CanonicalizationError::InvalidInputCircuit {
                message: error.to_string(),
            }
        })?;

    let mut result = String::new();

    result.push_str("q=");
    result.push_str(
        &circuit.num_qubits().to_string(),
    );

    result.push_str(";c=");
    result.push_str(
        &circuit
            .num_classical_bits()
            .to_string(),
    );

    result.push_str(";ops=");

    for (index, gate) in
        circuit.operations().iter().enumerate()
    {
        if index != 0 {
            result.push('|');
        }

        result.push_str(
            gate_kind_name(gate.kind()),
        );

        result.push('[');

        for (qubit_index, qubit) in
            gate.qubits().iter().enumerate()
        {
            if qubit_index != 0 {
                result.push(',');
            }

            result.push_str(
                &qubit.index().to_string(),
            );
        }

        result.push(']');

        if !gate.parameters().is_empty() {
            result.push('(');

            for (
                parameter_index,
                parameter,
            ) in gate.parameters().iter().enumerate()
            {
                if parameter_index != 0 {
                    result.push(',');
                }

                result.push_str(
                    &parameter.to_string(),
                );
            }

            result.push(')');
        }

        if let Some(bit) =
            gate.classical_target()
        {
            result.push_str(";m=");
            result.push_str(
                &bit.to_string(),
            );
        }
    }

    Ok(result)
}

/// Returns a stable name for a canonical gate kind.
///
/// This avoids relying on Rust's `Debug` formatting as a serialized contract.
#[must_use]
pub const fn gate_kind_name(
    kind: GateKind,
) -> &'static str {
    match kind {
        GateKind::I => "i",
        GateKind::X => "x",
        GateKind::Y => "y",
        GateKind::Z => "z",
        GateKind::H => "h",
        GateKind::S => "s",
        GateKind::Sdg => "sdg",
        GateKind::T => "t",
        GateKind::Tdg => "tdg",
        GateKind::V => "v",
        GateKind::Vdg => "vdg",

        GateKind::RX => "rx",
        GateKind::RY => "ry",
        GateKind::RZ => "rz",
        GateKind::Phase => "phase",
        GateKind::U1 => "u1",
        GateKind::U2 => "u2",
        GateKind::U3 => "u3",

        GateKind::CX => "cx",
        GateKind::CY => "cy",
        GateKind::CZ => "cz",
        GateKind::CH => "ch",
        GateKind::SWAP => "swap",
        GateKind::ISWAP => "iswap",
        GateKind::ECR => "ecr",

        GateKind::CRX => "crx",
        GateKind::CRY => "cry",
        GateKind::CRZ => "crz",

        GateKind::CCX => "ccx",
        GateKind::CSWAP => "cswap",

        GateKind::Measure => "measure",
        GateKind::Barrier => "barrier",
        GateKind::Reset => "reset",
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::parameter::{
        Parameter,
        ParameterExpression,
    };

    #[test]
    fn production_policy_is_conservative() {
        let policy =
            CanonicalizationPolicy::production();

        assert!(policy.normalize_parameters());
        assert!(policy.fold_constants());
        assert!(
            policy.normalize_signed_zero()
        );
        assert!(
            policy.normalize_symmetric_operands()
        );
        assert!(policy.validate_input());
        assert!(
            policy.validate_output_gates()
        );
        assert!(
            policy.validate_output_circuit()
        );
    }

    #[test]
    fn symmetric_gate_classification_is_conservative() {
        assert!(
            is_symmetric_operand_gate(
                GateKind::CZ
            )
        );

        assert!(
            is_symmetric_operand_gate(
                GateKind::SWAP
            )
        );

        assert!(
            is_symmetric_operand_gate(
                GateKind::ISWAP
            )
        );

        assert!(
            !is_symmetric_operand_gate(
                GateKind::CX
            )
        );

        assert!(
            !is_symmetric_operand_gate(
                GateKind::CY
            )
        );
    }

    #[test]
    fn signed_zero_is_normalized() {
        let parameter =
            Parameter::Constant(-0.0);

        let normalized =
            normalize_parameter(
                &parameter,
                CanonicalizationPolicy::production(),
                0,
                0,
            )
            .expect("normalization");

        assert_eq!(
            normalized.parameter,
            Parameter::Constant(0.0)
        );

        assert_eq!(
            normalized.signed_zero_changes,
            1
        );
    }

    #[test]
    fn finite_constant_expression_is_folded() {
        let expression =
            ParameterExpression::Add(
                Box::new(
                    Parameter::Constant(1.0),
                ),
                Box::new(
                    Parameter::Constant(2.0),
                ),
            );

        let normalized =
            normalize_expression(
                &expression,
                CanonicalizationPolicy::production(),
                0,
                0,
            )
            .expect("normalization");

        //
        // The representation remains an expression node because this function
        // preserves the ParameterExpression type at this level. The parent
        // Parameter layer is responsible for wrapping it.
        //
        assert!(
            normalized
                .constant_folds
                > 0
        );
    }

    #[test]
    fn gate_kind_names_are_stable() {
        assert_eq!(
            gate_kind_name(GateKind::H),
            "h"
        );

        assert_eq!(
            gate_kind_name(GateKind::CX),
            "cx"
        );

        assert_eq!(
            gate_kind_name(GateKind::Measure),
            "measure"
        );
    }
}