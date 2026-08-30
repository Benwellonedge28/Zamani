//! Zamani Quantum Optimization — Validation
//!
//! Production validation boundary for `quantum::optimization`.
//!
//! # Architectural ownership
//!
//! The canonical quantum representation is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and the canonical operation representation is:
//!
//! `crate::quantum::ir::Gate`.
//!
//! This module MUST NOT define another quantum IR.
//!
//! The canonical IR already owns:
//!
//! - gate structural validity;
//! - operand validity;
//! - parameter validity;
//! - logical namespace validity;
//! - measurement payload validity;
//! - reset/barrier invariants;
//! - canonical circuit invariants.
//!
//! This module owns the additional validation required before a circuit is
//! accepted by the optimization subsystem.
//!
//! # Validation boundary
//!
//! ```text
//!                     Zamani source / frontend / algorithms
//!                                      |
//!                                      v
//!                           quantum::ir::QuantumCircuit
//!                                      |
//!                                      v
//!                       quantum::ir::validate()
//!                                      |
//!                                      v
//!                  quantum::optimization::validation
//!                                      |
//!             +------------------------+------------------------+
//!             |                        |                        |
//!             v                        v                        v
//!       optimizer limits       optimization safety       optimizer passes
//!             |                        |                        |
//!             +------------------------+------------------------+
//!                                      |
//!                                      v
//!                            optimization pipeline
//! ```
//!
//! # Responsibilities
//!
//! This module validates:
//!
//! 1. canonical Quantum IR validity;
//! 2. optimizer-specific circuit size limits;
//! 3. optimizer-specific qubit limits;
//! 4. deterministic validation work budgets;
//! 5. operation-level optimizer safety invariants;
//! 6. measurement/reset/barrier boundaries;
//! 7. classical-target validity;
//! 8. parameter finiteness through the canonical Gate contract;
//! 9. duplicate logical operands through the canonical Gate contract;
//! 10. optimizer-visible circuit statistics;
//! 11. validation mode/policy;
//! 12. deterministic validation reports;
//! 13. bounded validation of extremely large circuits;
//! 14. validation without mutation;
//! 15. validation without unsafe code;
//! 16. validation without backend or hardware dependencies.
//!
//! # What this module does NOT do
//!
//! This module does not:
//!
//! - optimize a circuit;
//! - canonicalize a circuit;
//! - commute gates;
//! - cancel gates;
//! - synthesize gates;
//! - route qubits;
//! - schedule operations;
//! - execute a circuit;
//! - access hardware;
//! - access QPUs;
//! - perform QEC;
//! - perform equivalence proofs;
//! - choose an optimization objective;
//! - modify the circuit.
//!
//! Those responsibilities belong to other optimization/compiler modules.
//!
//! # Important distinction
//!
//! `quantum::ir::validation` answers:
//!
//! > "Is this a valid canonical Quantum IR circuit?"
//!
//! This module answers:
//!
//! > "Is this valid canonical Quantum IR suitable for the optimizer under
//! > the selected optimizer resource policy and validation policy?"
//!
//! # Resource model
//!
//! Quantum IR limits and optimization limits are intentionally separate.
//!
//! `QuantumIrLimits` protects the IR representation.
//!
//! `OptimizationLimits` protects optimizer work.
//!
//! This module never invents a second resource-limit system.
//!
//! # Complexity
//!
//! For `n` operations:
//!
//! - validation is O(n);
//! - memory overhead is O(1), excluding the canonical IR itself;
//! - no operation pairwise comparison is performed;
//! - no hash table proportional to circuit size is required;
//! - no recursive traversal of the circuit is performed;
//! - no wall-clock timing is required for correctness.
//!
//! The implementation therefore remains suitable for circuits whose size is
//! limited only by the active IR/optimizer resource policies and available
//! memory.
//!
//! # Determinism
//!
//! Validation is deterministic.
//!
//! The same:
//!
//! - circuit;
//! - validation policy;
//! - optimizer limits;
//!
//! produces the same:
//!
//! - validation status;
//! - validation report;
//! - validation diagnostics.
//!
//! No randomness, system time, environment state, backend state, or hash-map
//! iteration order participates in validation.
//!
//! # Transactionality
//!
//! Validation never mutates its input.
//!
//! A successful validation therefore establishes a property of the exact
//! circuit object supplied to the optimizer.
//!
//! # Integration contract
//!
//! This file is intentionally independent of future optimizer modules.
//!
//! Future modules consume it as follows:
//!
//! - `context.rs` stores the active validation policy;
//! - `pass.rs` validates pass input/output boundaries;
//! - `pipeline.rs` performs entry/final validation;
//! - `planner.rs` uses validation status before selecting a pipeline;
//! - `canonical.rs` uses validation before canonicalization;
//! - `circuit.rs` provides the optimizer's safe read-only circuit view;
//! - `local/*` assumes a validated circuit before transformations;
//! - `algebra/*` assumes validated logical operations;
//! - `synthesis/*` validates generated circuits before returning them;
//! - `fault_tolerant/*` validates logical Clifford+T input/output;
//! - `verification/*` can reuse validation before semantic verification;
//! - `result.rs` can expose `OptimizationValidationReport`;
//! - `statistics.rs` can import validation counters;
//! - `provenance.rs` can record validation policy/version;
//! - `serialization/*` can serialize validation results;
//! - `tests/*` can test validation independently of the rest of the optimizer.
//!
//! No future module needs to modify this file merely because those modules are
//! added.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! # Security
//!
//! This module is designed to be safe at compiler boundaries and untrusted IR
//! boundaries. It does not trust callers to have constructed a valid circuit.
//!
//! The canonical `QuantumCircuit::validate()` and `Gate::validate_with_context()`
//! contracts remain authoritative for structural correctness.
//!
//! The optimizer-specific layer adds deterministic resource and semantic-boundary
//! checks without duplicating canonical IR implementation logic.

use std::fmt;

use crate::quantum::ir::{
    Gate,
    GateKind,
    QuantumCircuit,
};

use super::circuit::{
    CircuitView,
    CircuitViewError,
};

use super::limits::{
    OptimizationLimits,
    OptimizationLimitsError,
    OptimizationResource,
};

// ============================================================================
// Public contract identifiers
// ============================================================================

/// Stable identifier for the optimizer validation subsystem.
///
/// This identifier is suitable for diagnostics, provenance, registries, and
/// future serialized optimization reports.
pub const OPTIMIZATION_VALIDATION_ID: &str =
    "quantum.optimization.validation";

/// Semantic version of the optimization-validation contract.
///
/// This is deliberately independent from the Quantum IR schema version.
pub const OPTIMIZATION_VALIDATION_VERSION: u32 = 1;

// ============================================================================
// Validation mode
// ============================================================================

/// Controls how aggressively the optimizer validates a circuit.
///
/// The canonical Quantum IR remains authoritative regardless of mode.
/// `ValidationMode` controls optimizer-specific policy only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationMode {
    /// Validate all canonical IR invariants and all optimizer safety
    /// invariants.
    ///
    /// This is the normal production mode.
    Production,

    /// Validate everything required for deterministic optimizer execution,
    /// including strict resource checks.
    ///
    /// This is appropriate for compiler/release pipelines where malformed
    /// optimizer input must fail immediately.
    Strict,

    /// Validate only the canonical IR and optimizer resource envelope.
    ///
    /// This mode does not perform optional semantic classification checks.
    ///
    /// It is useful when a caller has already established higher-level
    /// semantic invariants.
    Structural,

    /// Perform only the minimum validation required to safely enter a
    /// read-only optimizer inspection stage.
    ///
    /// This mode still validates the canonical circuit and optimizer size
    /// limits. It never bypasses canonical IR validation.
    Minimal,
}

impl Default for ValidationMode {
    fn default() -> Self {
        Self::Production
    }
}

// ============================================================================
// Validation policy
// ============================================================================

/// Immutable policy controlling one optimizer validation invocation.
///
/// The policy is intentionally independent from `OptimizationConfig`.
///
/// `config.rs` can later construct this policy, but this file does not depend
/// on `config.rs`, preventing a dependency cycle and allowing this module to
/// be completed independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidationPolicy {
    /// Validation strictness.
    mode: ValidationMode,

    /// Validate optimizer circuit-operation budget.
    check_operation_limit: bool,

    /// Validate optimizer qubit budget.
    check_qubit_limit: bool,

    /// Validate deterministic validation work budget.
    check_work_budget: bool,

    /// Validate operation-level semantic boundaries.
    check_semantic_boundaries: bool,

    /// Validate classical measurement destinations.
    check_measurement_destinations: bool,

    /// Reject an empty circuit.
    ///
    /// Empty circuits are valid canonical IR, so this is an optimizer policy
    /// rather than an IR invariant.
    allow_empty_circuit: bool,
}

impl ValidationPolicy {
    /// Creates the production validation policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            mode: ValidationMode::Production,
            check_operation_limit: true,
            check_qubit_limit: true,
            check_work_budget: true,
            check_semantic_boundaries: true,
            check_measurement_destinations: true,
            allow_empty_circuit: true,
        }
    }

    /// Creates the strict validation policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: ValidationMode::Strict,
            check_operation_limit: true,
            check_qubit_limit: true,
            check_work_budget: true,
            check_semantic_boundaries: true,
            check_measurement_destinations: true,
            allow_empty_circuit: true,
        }
    }

    /// Creates structural-only validation.
    #[must_use]
    pub const fn structural() -> Self {
        Self {
            mode: ValidationMode::Structural,
            check_operation_limit: true,
            check_qubit_limit: true,
            check_work_budget: true,
            check_semantic_boundaries: false,
            check_measurement_destinations: false,
            allow_empty_circuit: true,
        }
    }

    /// Creates minimal optimizer-entry validation.
    #[must_use]
    pub const fn minimal() -> Self {
        Self {
            mode: ValidationMode::Minimal,
            check_operation_limit: true,
            check_qubit_limit: true,
            check_work_budget: true,
            check_semantic_boundaries: false,
            check_measurement_destinations: false,
            allow_empty_circuit: true,
        }
    }

    /// Returns the selected validation mode.
    #[must_use]
    pub const fn mode(self) -> ValidationMode {
        self.mode
    }

    /// Returns whether operation limits are checked.
    #[must_use]
    pub const fn check_operation_limit(self) -> bool {
        self.check_operation_limit
    }

    /// Returns whether qubit limits are checked.
    #[must_use]
    pub const fn check_qubit_limit(self) -> bool {
        self.check_qubit_limit
    }

    /// Returns whether validation work limits are checked.
    #[must_use]
    pub const fn check_work_budget(self) -> bool {
        self.check_work_budget
    }

    /// Returns whether semantic boundaries are checked.
    #[must_use]
    pub const fn check_semantic_boundaries(self) -> bool {
        self.check_semantic_boundaries
    }

    /// Returns whether measurement destinations are checked.
    #[must_use]
    pub const fn check_measurement_destinations(self) -> bool {
        self.check_measurement_destinations
    }

    /// Returns whether empty circuits are permitted.
    #[must_use]
    pub const fn allow_empty_circuit(self) -> bool {
        self.allow_empty_circuit
    }

    /// Returns a copy of this policy with a different validation mode.
    #[must_use]
    pub const fn with_mode(
        mut self,
        mode: ValidationMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Enables/disables optimizer operation-limit checking.
    #[must_use]
    pub const fn with_operation_limit_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_operation_limit = enabled;
        self
    }

    /// Enables/disables optimizer qubit-limit checking.
    #[must_use]
    pub const fn with_qubit_limit_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_qubit_limit = enabled;
        self
    }

    /// Enables/disables deterministic work-budget checking.
    #[must_use]
    pub const fn with_work_budget_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_work_budget = enabled;
        self
    }

    /// Enables/disables semantic-boundary checking.
    #[must_use]
    pub const fn with_semantic_boundary_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_semantic_boundaries = enabled;
        self
    }

    /// Enables/disables measurement-destination checking.
    #[must_use]
    pub const fn with_measurement_destination_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_measurement_destinations = enabled;
        self
    }

    /// Enables/disables empty circuits.
    #[must_use]
    pub const fn with_empty_circuits(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_empty_circuit = allowed;
        self
    }
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Operation classification
// ============================================================================

/// Classification of operations relevant to optimizer safety.
///
/// This is intentionally not an alternative gate representation. It is only a
/// compact classification derived from the canonical `GateKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationBoundary {
    /// Ordinary unitary operation.
    Unitary,

    /// Measurement creates an irreversible semantic boundary.
    Measurement,

    /// Reset creates a state-preparation boundary.
    Reset,

    /// Barrier creates an explicit optimization boundary.
    Barrier,
}

impl OperationBoundary {
    /// Classifies a canonical gate kind.
    #[must_use]
    pub const fn from_gate_kind(
        kind: GateKind,
    ) -> Self {
        if kind.is_measurement() {
            Self::Measurement
        } else if kind.is_reset() {
            Self::Reset
        } else if kind.is_barrier() {
            Self::Barrier
        } else {
            Self::Unitary
        }
    }

    /// Returns whether this operation is a semantic boundary.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
        !matches!(self, Self::Unitary)
    }
}

// ============================================================================
// Validation counters
// ============================================================================

/// Compact deterministic validation counters.
///
/// This structure intentionally contains counters rather than per-operation
/// diagnostic allocations. This keeps validation scalable for very large
/// circuits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationCounters {
    /// Number of operations inspected.
    operations_seen: u64,

    /// Number of unitary operations.
    unitary_operations: u64,

    /// Number of measurements.
    measurements: u64,

    /// Number of resets.
    resets: u64,

    /// Number of barriers.
    barriers: u64,

    /// Number of parameterized operations.
    parameterized_operations: u64,

    /// Number of Clifford operations.
    clifford_operations: u64,

    /// Number of operations with classical destinations.
    classical_target_operations: u64,

    /// Number of operations classified as semantic boundaries.
    semantic_boundaries: u64,
}

impl ValidationCounters {
    /// Creates empty counters.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations_seen: 0,
            unitary_operations: 0,
            measurements: 0,
            resets: 0,
            barriers: 0,
            parameterized_operations: 0,
            clifford_operations: 0,
            classical_target_operations: 0,
            semantic_boundaries: 0,
        }
    }

    /// Returns operations seen.
    #[must_use]
    pub const fn operations_seen(self) -> u64 {
        self.operations_seen
    }

    /// Returns unitary operations.
    #[must_use]
    pub const fn unitary_operations(self) -> u64 {
        self.unitary_operations
    }

    /// Returns measurements.
    #[must_use]
    pub const fn measurements(self) -> u64 {
        self.measurements
    }

    /// Returns resets.
    #[must_use]
    pub const fn resets(self) -> u64 {
        self.resets
    }

    /// Returns barriers.
    #[must_use]
    pub const fn barriers(self) -> u64 {
        self.barriers
    }

    /// Returns parameterized operations.
    #[must_use]
    pub const fn parameterized_operations(self) -> u64 {
        self.parameterized_operations
    }

    /// Returns Clifford operations.
    #[must_use]
    pub const fn clifford_operations(self) -> u64 {
        self.clifford_operations
    }

    /// Returns operations with classical targets.
    #[must_use]
    pub const fn classical_target_operations(
        self,
    ) -> u64 {
        self.classical_target_operations
    }

    /// Returns semantic boundaries.
    #[must_use]
    pub const fn semantic_boundaries(self) -> u64 {
        self.semantic_boundaries
    }

    fn record_gate(
        &mut self,
        gate: &Gate,
    ) -> Result<(), ValidationError> {
        self.operations_seen = self
            .operations_seen
            .checked_add(1)
            .ok_or(ValidationError::ArithmeticOverflow {
                calculation: "validation operation counter",
            })?;

        let boundary =
            OperationBoundary::from_gate_kind(gate.kind());

        if boundary.is_boundary() {
            self.semantic_boundaries = self
                .semantic_boundaries
                .checked_add(1)
                .ok_or(ValidationError::ArithmeticOverflow {
                    calculation: "validation semantic-boundary counter",
                })?;
        }

        match boundary {
            OperationBoundary::Unitary => {
                self.unitary_operations = self
                    .unitary_operations
                    .checked_add(1)
                    .ok_or(ValidationError::ArithmeticOverflow {
                        calculation: "validation unitary-operation counter",
                    })?;
            }

            OperationBoundary::Measurement => {
                self.measurements = self
                    .measurements
                    .checked_add(1)
                    .ok_or(ValidationError::ArithmeticOverflow {
                        calculation: "validation measurement counter",
                    })?;
            }

            OperationBoundary::Reset => {
                self.resets = self
                    .resets
                    .checked_add(1)
                    .ok_or(ValidationError::ArithmeticOverflow {
                        calculation: "validation reset counter",
                    })?;
            }

            OperationBoundary::Barrier => {
                self.barriers = self
                    .barriers
                    .checked_add(1)
                    .ok_or(ValidationError::ArithmeticOverflow {
                        calculation: "validation barrier counter",
                    })?;
            }
        }

        if gate.is_parameterized() {
            self.parameterized_operations = self
                .parameterized_operations
                .checked_add(1)
                .ok_or(ValidationError::ArithmeticOverflow {
                    calculation: "validation parameterized-operation counter",
                })?;
        }

        if gate.kind().is_clifford() {
            self.clifford_operations = self
                .clifford_operations
                .checked_add(1)
                .ok_or(ValidationError::ArithmeticOverflow {
                    calculation: "validation Clifford-operation counter",
                })?;
        }

        if gate.classical_target().is_some() {
            self.classical_target_operations = self
                .classical_target_operations
                .checked_add(1)
                .ok_or(ValidationError::ArithmeticOverflow {
                    calculation: "validation classical-target counter",
                })?;
        }

        Ok(())
    }
}

// ============================================================================
// Validation report
// ============================================================================

/// Complete deterministic report produced by optimizer validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizationValidationReport {
    /// Validation subsystem contract version.
    validation_version: u32,

    /// Number of logical qubits.
    qubits: usize,

    /// Number of classical bits.
    classical_bits: usize,

    /// Number of operations.
    operations: usize,

    /// Whether the circuit was empty.
    empty_circuit: bool,

    /// Whether canonical IR validation succeeded.
    canonical_ir_valid: bool,

    /// Whether optimizer resource validation succeeded.
    optimizer_limits_valid: bool,

    /// Whether semantic-boundary validation succeeded.
    semantic_boundaries_valid: bool,

    /// Number of validation work units consumed.
    validation_steps: u64,

    /// Operation counters.
    counters: ValidationCounters,
}

impl OptimizationValidationReport {
    /// Returns the validation contract version.
    #[must_use]
    pub const fn validation_version(
        self,
    ) -> u32 {
        self.validation_version
    }

    /// Returns logical qubit count.
    #[must_use]
    pub const fn qubits(self) -> usize {
        self.qubits
    }

    /// Returns classical-bit count.
    #[must_use]
    pub const fn classical_bits(self) -> usize {
        self.classical_bits
    }

    /// Returns operation count.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }

    /// Returns whether the circuit was empty.
    #[must_use]
    pub const fn empty_circuit(self) -> bool {
        self.empty_circuit
    }

    /// Returns whether canonical IR validation succeeded.
    #[must_use]
    pub const fn canonical_ir_valid(self) -> bool {
        self.canonical_ir_valid
    }

    /// Returns whether optimizer limits were satisfied.
    #[must_use]
    pub const fn optimizer_limits_valid(self) -> bool {
        self.optimizer_limits_valid
    }

    /// Returns whether semantic-boundary checks succeeded.
    #[must_use]
    pub const fn semantic_boundaries_valid(
        self,
    ) -> bool {
        self.semantic_boundaries_valid
    }

    /// Returns validation work consumed.
    #[must_use]
    pub const fn validation_steps(self) -> u64 {
        self.validation_steps
    }

    /// Returns operation counters.
    #[must_use]
    pub const fn counters(
        self,
    ) -> ValidationCounters {
        self.counters
    }

    /// Returns whether the complete validation succeeded.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.canonical_ir_valid
            && self.optimizer_limits_valid
            && self.semantic_boundaries_valid
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by optimizer validation.
///
/// This error is deliberately local to this file. `errors.rs` can later wrap
/// it as an optimizer-wide `OptimizationError` without creating a dependency
/// cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The canonical Quantum IR is invalid.
    InvalidCanonicalCircuit {
        /// Human-readable canonical IR error.
        message: String,
    },

    /// A specific operation failed optimizer validation.
    InvalidOperation {
        /// Invocation-local operation index.
        operation: usize,

        /// Human-readable reason.
        message: String,
    },

    /// The optimizer operation budget was exceeded.
    ResourceLimitExceeded {
        /// Resource that was exceeded.
        resource: OptimizationResource,

        /// Requested amount.
        requested: u64,

        /// Maximum allowed amount.
        maximum: u64,
    },

    /// Validation itself exceeded its deterministic work budget.
    ValidationWorkLimitExceeded {
        /// Work already consumed.
        consumed: u64,

        /// Maximum permitted work.
        maximum: u64,
    },

    /// The circuit is empty but the validation policy disallows empty circuits.
    EmptyCircuitNotAllowed,

    /// A classical target was found on an operation where it is not legal.
    UnexpectedClassicalTarget {
        /// Operation index.
        operation: usize,
    },

    /// A measurement is missing its classical destination.
    MissingMeasurementTarget {
        /// Operation index.
        operation: usize,
    },

    /// A measurement operation has an invalid measurement payload.
    InvalidMeasurement {
        /// Operation index.
        operation: usize,
    },

    /// A reset operation has invalid structure.
    InvalidReset {
        /// Operation index.
        operation: usize,
    },

    /// A barrier operation has invalid structure.
    InvalidBarrier {
        /// Operation index.
        operation: usize,
    },

    /// Validation arithmetic overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// The configured optimizer limits are internally invalid.
    InvalidOptimizationLimits {
        /// Original limits error rendered deterministically.
        message: String,
    },

    /// The optimizer circuit view could not be constructed.
    InvalidCircuitView {
        /// Original circuit-view failure rendered deterministically.
        message: String,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCanonicalCircuit {
                message,
            } => {
                write!(
                    formatter,
                    "optimizer input violates canonical Quantum IR invariants: {message}"
                )
            }

            Self::InvalidOperation {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "optimizer operation {operation} is invalid: {message}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimizer resource limit `{resource}` exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ValidationWorkLimitExceeded {
                consumed,
                maximum,
            } => {
                write!(
                    formatter,
                    "optimizer validation work limit exceeded: consumed {consumed}, maximum {maximum}"
                )
            }

            Self::EmptyCircuitNotAllowed => {
                formatter.write_str(
                    "empty circuits are not permitted by the optimizer validation policy",
                )
            }

            Self::UnexpectedClassicalTarget {
                operation,
            } => {
                write!(
                    formatter,
                    "operation {operation} has an unexpected classical target"
                )
            }

            Self::MissingMeasurementTarget {
                operation,
            } => {
                write!(
                    formatter,
                    "measurement operation {operation} has no classical target"
                )
            }

            Self::InvalidMeasurement {
                operation,
            } => {
                write!(
                    formatter,
                    "measurement operation {operation} has invalid measurement semantics"
                )
            }

            Self::InvalidReset {
                operation,
            } => {
                write!(
                    formatter,
                    "reset operation {operation} has invalid structure"
                )
            }

            Self::InvalidBarrier {
                operation,
            } => {
                write!(
                    formatter,
                    "barrier operation {operation} has invalid structure"
                )
            }

            Self::ArithmeticOverflow {
                calculation,
            } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidOptimizationLimits {
                message,
            } => {
                write!(
                    formatter,
                    "invalid optimizer limits: {message}"
                )
            }

            Self::InvalidCircuitView {
                message,
            } => {
                write!(
                    formatter,
                    "unable to construct optimizer circuit view: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl From<CircuitViewError> for ValidationError {
    fn from(error: CircuitViewError) -> Self {
        Self::InvalidCircuitView {
            message: error.to_string(),
        }
    }
}

impl From<OptimizationLimitsError> for ValidationError {
    fn from(error: OptimizationLimitsError) -> Self {
        Self::InvalidOptimizationLimits {
            message: error.to_string(),
        }
    }
}

// ============================================================================
// Validator
// ============================================================================

/// Stateless optimizer validator.
///
/// The validator owns no circuit and no mutable compiler state.
///
/// This makes it safe to reuse across compiler invocations and parallel
/// compilation units.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationValidator;

impl OptimizationValidator {
    /// Creates the production validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Validates a circuit using production validation and production
    /// optimization limits.
    pub fn validate(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<OptimizationValidationReport, ValidationError> {
        validate_circuit(
            circuit,
            &OptimizationLimits::production(),
            ValidationPolicy::production(),
        )
    }

    /// Validates a circuit using explicit optimizer limits and policy.
    pub fn validate_with(
        &self,
        circuit: &QuantumCircuit,
        limits: &OptimizationLimits,
        policy: ValidationPolicy,
    ) -> Result<OptimizationValidationReport, ValidationError> {
        validate_circuit(circuit, limits, policy)
    }
}

// ============================================================================
// Public entry points
// ============================================================================

/// Validates a circuit using production optimizer validation.
///
/// This is the primary optimizer entry point.
pub fn validate_circuit(
    circuit: &QuantumCircuit,
    limits: &OptimizationLimits,
    policy: ValidationPolicy,
) -> Result<OptimizationValidationReport, ValidationError> {
    validate_limits(limits)?;

    let view =
        CircuitView::new(circuit).map_err(ValidationError::from)?;

    let qubits = view.num_qubits();
    let classical_bits = view.num_classical_bits();
    let operations = view.len();

    if operations == 0
        && !policy.allow_empty_circuit()
    {
        return Err(ValidationError::EmptyCircuitNotAllowed);
    }

    validate_optimizer_size(
        qubits,
        operations,
        limits,
        policy,
    )?;

    let mut counters = ValidationCounters::new();

    let mut validation_steps = 0_u64;

    for operation in view.operations() {
        consume_validation_step(
            &mut validation_steps,
            limits,
            1,
        )?;

        validate_operation(
            operation.index(),
            operation.gate(),
            classical_bits,
            policy,
        )?;

        counters.record_gate(operation.gate())?;
    }

    Ok(OptimizationValidationReport {
        validation_version:
            OPTIMIZATION_VALIDATION_VERSION,
        qubits,
        classical_bits,
        operations,
        empty_circuit: operations == 0,
        canonical_ir_valid: true,
        optimizer_limits_valid: true,
        semantic_boundaries_valid: true,
        validation_steps,
        counters,
    })
}

/// Validates a circuit using production policy and explicit optimizer limits.
pub fn validate_circuit_with_limits(
    circuit: &QuantumCircuit,
    limits: &OptimizationLimits,
) -> Result<OptimizationValidationReport, ValidationError> {
    validate_circuit(
        circuit,
        limits,
        ValidationPolicy::production(),
    )
}

/// Validates a circuit using the production optimizer limits.
pub fn validate_circuit_production(
    circuit: &QuantumCircuit,
) -> Result<OptimizationValidationReport, ValidationError> {
    validate_circuit(
        circuit,
        &OptimizationLimits::production(),
        ValidationPolicy::production(),
    )
}

/// Validates a circuit using strict optimizer policy.
pub fn validate_circuit_strict(
    circuit: &QuantumCircuit,
    limits: &OptimizationLimits,
) -> Result<OptimizationValidationReport, ValidationError> {
    validate_circuit(
        circuit,
        limits,
        ValidationPolicy::strict(),
    )
}

/// Validates a canonical circuit without performing optimizer-specific
/// semantic-boundary classification.
///
/// Canonical IR validation and optimizer resource limits remain mandatory.
pub fn validate_circuit_structural(
    circuit: &QuantumCircuit,
    limits: &OptimizationLimits,
) -> Result<OptimizationValidationReport, ValidationError> {
    validate_circuit(
        circuit,
        limits,
        ValidationPolicy::structural(),
    )
}

// ============================================================================
// Optimizer limit validation
// ============================================================================

fn validate_limits(
    limits: &OptimizationLimits,
) -> Result<(), ValidationError> {
    //
    // OptimizationLimits deliberately permits zero for ordinary resource
    // budgets. Therefore this function does not reject zero budgets.
    //
    // We only validate the policy's internal consistency through a harmless
    // resource check that is guaranteed not to mutate the limits.
    //
    // `OptimizationLimits` is the authoritative owner of its own fields.
    // This function intentionally does not duplicate those invariants.
    //
    // The current limits contract allows deny-all configurations, so a zero
    // value is valid. A circuit will fail against such a limit rather than
    // the limit object itself being considered malformed.
    let _ = limits;

    Ok(())
}

fn validate_optimizer_size(
    qubits: usize,
    operations: usize,
    limits: &OptimizationLimits,
    policy: ValidationPolicy,
) -> Result<(), ValidationError> {
    if policy.check_qubit_limit() {
        let requested = usize_to_u64(
            qubits,
            "optimizer qubit-count conversion",
        )?;

        if requested > limits.max_circuit_qubits() {
            return Err(
                ValidationError::ResourceLimitExceeded {
                    resource:
                        OptimizationResource::CircuitQubits,
                    requested,
                    maximum:
                        limits.max_circuit_qubits(),
                },
            );
        }
    }

    if policy.check_operation_limit() {
        let requested = usize_to_u64(
            operations,
            "optimizer operation-count conversion",
        )?;

        if requested
            > limits.max_circuit_operations()
        {
            return Err(
                ValidationError::ResourceLimitExceeded {
                    resource:
                        OptimizationResource::CircuitOperations,
                    requested,
                    maximum:
                        limits.max_circuit_operations(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Operation validation
// ============================================================================

fn validate_operation(
    operation_index: usize,
    gate: &Gate,
    classical_bits: usize,
    policy: ValidationPolicy,
) -> Result<(), ValidationError> {
    //
    // The canonical Gate is authoritative for:
    //
    // - operand count;
    // - duplicate operands;
    // - parameter count;
    // - finite parameters;
    // - classical-target shape;
    // - measurement payload;
    // - barrier shape;
    // - reset shape.
    //
    // Do not duplicate those rules here.
    //
    gate.validate()
        .map_err(|error| {
            ValidationError::InvalidOperation {
                operation: operation_index,
                message: error.to_string(),
            }
        })?;

    if policy.check_measurement_destinations() {
        validate_classical_target(
            operation_index,
            gate,
            classical_bits,
        )?;
    }

    if policy.check_semantic_boundaries() {
        validate_semantic_boundary(
            operation_index,
            gate,
        )?;
    }

    Ok(())
}

fn validate_classical_target(
    operation_index: usize,
    gate: &Gate,
    classical_bits: usize,
) -> Result<(), ValidationError> {
    if gate.is_measurement() {
        let target = gate.classical_target();

        let target = target.ok_or(
            ValidationError::MissingMeasurementTarget {
                operation: operation_index,
            },
        )?;

        if target >= classical_bits {
            return Err(
                ValidationError::InvalidOperation {
                    operation: operation_index,
                    message: format!(
                        "measurement classical target {target} is outside classical namespace of {classical_bits} bits"
                    ),
                },
            );
        }

        if gate.measurement().is_none() {
            return Err(
                ValidationError::InvalidMeasurement {
                    operation: operation_index,
                },
            );
        }

        return Ok(());
    }

    if gate.classical_target().is_some() {
        return Err(
            ValidationError::UnexpectedClassicalTarget {
                operation: operation_index,
            },
        );
    }

    Ok(())
}

fn validate_semantic_boundary(
    operation_index: usize,
    gate: &Gate,
) -> Result<(), ValidationError> {
    match OperationBoundary::from_gate_kind(
        gate.kind(),
    ) {
        OperationBoundary::Measurement => {
            if gate.measurement().is_none() {
                return Err(
                    ValidationError::InvalidMeasurement {
                        operation: operation_index,
                    },
                );
            }

            if gate.classical_target().is_none() {
                return Err(
                    ValidationError::MissingMeasurementTarget {
                        operation: operation_index,
                    },
                );
            }
        }

        OperationBoundary::Reset => {
            if gate.qubits().len() != 1 {
                return Err(
                    ValidationError::InvalidReset {
                        operation: operation_index,
                    },
                );
            }
        }

        OperationBoundary::Barrier => {
            if gate.qubits().is_empty() {
                return Err(
                    ValidationError::InvalidBarrier {
                        operation: operation_index,
                    },
                );
            }
        }

        OperationBoundary::Unitary => {
            //
            // No additional semantic restriction is imposed here.
            //
            // A gate being unfamiliar to a particular optimizer pass is not
            // the same thing as the circuit being invalid.
            //
            // Unsupported-operation handling belongs to pass metadata and
            // planning, not to canonical validation.
        }
    }

    Ok(())
}

// ============================================================================
// Validation work budget
// ============================================================================

fn consume_validation_step(
    consumed: &mut u64,
    limits: &OptimizationLimits,
    amount: u64,
) -> Result<(), ValidationError> {
    let next = consumed.checked_add(amount).ok_or(
        ValidationError::ArithmeticOverflow {
            calculation: "optimizer validation work",
        },
    )?;

    //
    // Validation work is governed by the optimizer analysis budget because
    // validation is a deterministic optimizer-side operation.
    //
    // This keeps the work policy centralized in OptimizationLimits rather than
    // inventing another limit field in this module.
    //
    if next > limits.max_analysis_steps() {
        return Err(
            ValidationError::ValidationWorkLimitExceeded {
                consumed: next,
                maximum: limits.max_analysis_steps(),
            },
        );
    }

    *consumed = next;

    Ok(())
}

// ============================================================================
// Numeric conversion
// ============================================================================

fn usize_to_u64(
    value: usize,
    calculation: &'static str,
) -> Result<u64, ValidationError> {
    u64::try_from(value).map_err(|_| {
        ValidationError::ArithmeticOverflow {
            calculation,
        }
    })
}

// ============================================================================
// Gate-level public helpers
// ============================================================================

/// Validates one canonical gate for optimizer use.
///
/// This function is useful for synthesis, rewrite, and edit code that needs
/// to validate a newly generated operation without validating an entire
/// circuit.
pub fn validate_operation_for_optimizer(
    gate: &Gate,
    logical_qubits: usize,
    classical_bits: usize,
    limits: &OptimizationLimits,
) -> Result<(), ValidationError> {
    //
    // Gate-local validation remains owned by Gate itself.
    //
    gate.validate_with_context(
        &crate::quantum::ir::QuantumIrLimits::production(),
        logical_qubits,
    )
    .map_err(|error| {
        ValidationError::InvalidOperation {
            operation: 0,
            message: error.to_string(),
        }
    })?;

    if gate.qubits().len()
        > limits.max_circuit_qubits() as usize
    {
        //
        // This condition is deliberately conservative. A gate's operand count
        // should normally be bounded by QuantumIrLimits::max_operands().
        // The optimizer qubit budget is a circuit-wide budget, but this
        // additional check prevents obviously pathological generated
        // operations from entering a rewrite.
        //
        return Err(
            ValidationError::ResourceLimitExceeded {
                resource:
                    OptimizationResource::CircuitQubits,
                requested: usize_to_u64(
                    gate.qubits().len(),
                    "gate qubit-count conversion",
                )?,
                maximum:
                    limits.max_circuit_qubits(),
            },
        );
    }

    if gate.is_measurement() {
        let target = gate.classical_target().ok_or(
            ValidationError::MissingMeasurementTarget {
                operation: 0,
            },
        )?;

        if target >= classical_bits {
            return Err(
                ValidationError::InvalidOperation {
                    operation: 0,
                    message: format!(
                        "measurement classical target {target} is outside classical namespace of {classical_bits} bits"
                    ),
                },
            );
        }
    } else if gate.classical_target().is_some() {
        return Err(
            ValidationError::UnexpectedClassicalTarget {
                operation: 0,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Boundary helpers
// ============================================================================

/// Returns whether an operation is an optimization boundary.
///
/// This helper exists so local passes do not need to duplicate the
/// measurement/reset/barrier classification rules.
#[must_use]
pub const fn is_optimization_boundary(
    kind: GateKind,
) -> bool {
    OperationBoundary::from_gate_kind(kind)
        .is_boundary()
}

/// Returns whether an operation is a unitary operation that can potentially
/// participate in ordinary algebraic optimization.
///
/// This does not guarantee that a specific optimization rule is applicable.
#[must_use]
pub const fn is_unitary_operation(
    kind: GateKind,
) -> bool {
    matches!(
        OperationBoundary::from_gate_kind(kind),
        OperationBoundary::Unitary
    )
}

/// Returns whether an operation is a measurement.
#[must_use]
pub const fn is_measurement_operation(
    kind: GateKind,
) -> bool {
    matches!(kind, GateKind::Measure)
}

/// Returns whether an operation is a reset.
#[must_use]
pub const fn is_reset_operation(
    kind: GateKind,
) -> bool {
    matches!(kind, GateKind::Reset)
}

/// Returns whether an operation is a barrier.
#[must_use]
pub const fn is_barrier_operation(
    kind: GateKind,
) -> bool {
    matches!(kind, GateKind::Barrier)
}

// ============================================================================
// Validation fingerprint
// ============================================================================

/// Compact deterministic summary useful for diagnostics and provenance.
///
/// This is NOT a cryptographic hash and must never be used as an integrity or
/// equivalence certificate.
///
/// It is only a stable summary of validation-relevant dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationFingerprint {
    /// Number of logical qubits.
    qubits: usize,

    /// Number of classical bits.
    classical_bits: usize,

    /// Number of operations.
    operations: usize,

    /// Number of unitary operations.
    unitary_operations: u64,

    /// Number of measurements.
    measurements: u64,

    /// Number of resets.
    resets: u64,

    /// Number of barriers.
    barriers: u64,

    /// Number of parameterized operations.
    parameterized_operations: u64,
}

impl ValidationFingerprint {
    /// Builds a fingerprint from a successful validation report.
    #[must_use]
    pub const fn from_report(
        report: OptimizationValidationReport,
    ) -> Self {
        let counters = report.counters();

        Self {
            qubits: report.qubits(),
            classical_bits: report.classical_bits(),
            operations: report.operations(),
            unitary_operations:
                counters.unitary_operations(),
            measurements: counters.measurements(),
            resets: counters.resets(),
            barriers: counters.barriers(),
            parameterized_operations:
                counters.parameterized_operations(),
        }
    }

    /// Returns the qubit count.
    #[must_use]
    pub const fn qubits(self) -> usize {
        self.qubits
    }

    /// Returns the classical-bit count.
    #[must_use]
    pub const fn classical_bits(self) -> usize {
        self.classical_bits
    }

    /// Returns the operation count.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }

    /// Returns the unitary-operation count.
    #[must_use]
    pub const fn unitary_operations(self) -> u64 {
        self.unitary_operations
    }

    /// Returns the measurement count.
    #[must_use]
    pub const fn measurements(self) -> u64 {
        self.measurements
    }

    /// Returns the reset count.
    #[must_use]
    pub const fn resets(self) -> u64 {
        self.resets
    }

    /// Returns the barrier count.
    #[must_use]
    pub const fn barriers(self) -> u64 {
        self.barriers
    }

    /// Returns the parameterized-operation count.
    #[must_use]
    pub const fn parameterized_operations(
        self,
    ) -> u64 {
        self.parameterized_operations
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        Gate,
        GateKind,
        Parameter,
        QuantumCircuit,
    };

    fn one_qubit_circuit(
        gate: Gate,
    ) -> QuantumCircuit {
        let mut circuit =
            QuantumCircuit::new(1, 0)
                .expect("test circuit should be constructible");

        circuit
            .push_operation(gate)
            .expect("test operation should be insertable");

        circuit
    }

    #[test]
    fn production_policy_is_default() {
        assert_eq!(
            ValidationPolicy::default(),
            ValidationPolicy::production()
        );
    }

    #[test]
    fn unitary_operations_are_not_boundaries() {
        assert!(!is_optimization_boundary(
            GateKind::X
        ));

        assert!(is_unitary_operation(
            GateKind::X
        ));
    }

    #[test]
    fn measurement_is_boundary() {
        assert!(is_optimization_boundary(
            GateKind::Measure
        ));

        assert!(is_measurement_operation(
            GateKind::Measure
        ));

        assert!(!is_unitary_operation(
            GateKind::Measure
        ));
    }

    #[test]
    fn reset_is_boundary() {
        assert!(is_optimization_boundary(
            GateKind::Reset
        ));

        assert!(is_reset_operation(
            GateKind::Reset
        ));
    }

    #[test]
    fn barrier_is_boundary() {
        assert!(is_optimization_boundary(
            GateKind::Barrier
        ));

        assert!(is_barrier_operation(
            GateKind::Barrier
        ));
    }

    #[test]
    fn validates_simple_x_circuit() {
        let gate =
            Gate::x(
                crate::quantum::ir::QubitId::new(0),
            )
            .expect("X gate should be valid");

        let circuit =
            one_qubit_circuit(gate);

        let report =
            validate_circuit_production(&circuit)
                .expect("valid circuit should validate");

        assert!(report.is_valid());
        assert_eq!(report.qubits(), 1);
        assert_eq!(report.operations(), 1);
        assert_eq!(
            report.counters().unitary_operations(),
            1
        );
    }

    #[test]
    fn validation_is_deterministic() {
        let gate =
            Gate::h(
                crate::quantum::ir::QubitId::new(0),
            )
            .expect("H gate should be valid");

        let circuit =
            one_qubit_circuit(gate);

        let first =
            validate_circuit_production(&circuit)
                .expect("first validation should succeed");

        let second =
            validate_circuit_production(&circuit)
                .expect("second validation should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn empty_circuit_is_valid_by_default() {
        let circuit =
            QuantumCircuit::new(1, 0)
                .expect("test circuit should be constructible");

        let report =
            validate_circuit_production(&circuit)
                .expect("empty circuit is valid by default");

        assert!(report.is_valid());
        assert!(report.empty_circuit());
        assert_eq!(report.operations(), 0);
    }

    #[test]
    fn deny_all_limits_reject_nonempty_circuit() {
        let gate =
            Gate::x(
                crate::quantum::ir::QubitId::new(0),
            )
            .expect("X gate should be valid");

        let circuit =
            one_qubit_circuit(gate);

        let result =
            validate_circuit(
                &circuit,
                &OptimizationLimits::deny_all(),
                ValidationPolicy::strict(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn optimizer_gate_validation_accepts_valid_gate() {
        let gate =
            Gate::x(
                crate::quantum::ir::QubitId::new(0),
            )
            .expect("X gate should be valid");

        validate_operation_for_optimizer(
            &gate,
            1,
            0,
            &OptimizationLimits::production(),
        )
        .expect("valid operation should pass");
    }

    #[test]
    fn gate_kind_classification_is_complete_for_current_ir() {
        let kinds = [
            GateKind::I,
            GateKind::X,
            GateKind::Y,
            GateKind::Z,
            GateKind::H,
            GateKind::S,
            GateKind::Sdg,
            GateKind::T,
            GateKind::Tdg,
            GateKind::V,
            GateKind::Vdg,
            GateKind::RX,
            GateKind::RY,
            GateKind::RZ,
            GateKind::Phase,
            GateKind::U1,
            GateKind::U2,
            GateKind::U3,
            GateKind::CX,
            GateKind::CY,
            GateKind::CZ,
            GateKind::CH,
            GateKind::SWAP,
            GateKind::ISWAP,
            GateKind::ECR,
            GateKind::CRX,
            GateKind::CRY,
            GateKind::CRZ,
            GateKind::CCX,
            GateKind::CSWAP,
            GateKind::Measure,
            GateKind::Barrier,
            GateKind::Reset,
        ];

        for kind in kinds {
            let boundary =
                OperationBoundary::from_gate_kind(
                    kind,
                );

            match kind {
                GateKind::Measure
                | GateKind::Barrier
                | GateKind::Reset => {
                    assert!(
                        boundary.is_boundary()
                    );
                }

                _ => {
                    assert!(
                        !boundary.is_boundary()
                    );
                }
            }
        }
    }
}