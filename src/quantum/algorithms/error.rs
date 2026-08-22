//! Zamani Quantum Algorithms — Canonical Error Boundary.
//!
//! This module defines the single error contract for the complete
//! `quantum::algorithms` subsystem.
//!
//! # Architectural responsibility
//!
//! `error.rs` owns:
//!
//! - canonical algorithm errors;
//! - stable machine-readable error codes;
//! - stable error classification;
//! - severity classification;
//! - retryability classification;
//! - structured diagnostics;
//! - canonical `Result<T>`;
//! - constructors for common failures.
//!
//! `error.rs` does NOT own:
//!
//! - algorithm configuration;
//! - parameter storage;
//! - circuit representation;
//! - IR validation;
//! - execution backends;
//! - hardware topology;
//! - routing;
//! - transpilation;
//! - error correction;
//! - optimizer implementations;
//! - objective implementations;
//! - persistence;
//! - telemetry.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Dependency direction
//!
//! ```text
//!                         error.rs
//!                            │
//!              ┌─────────────┼─────────────┐
//!              │             │             │
//!              ▼             ▼             ▼
//!          types.rs     execution.rs  objective.rs
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                       optimizer.rs
//!                            │
//!                            ▼
//!                      variational.rs
//!                            │
//!                   ┌────────┼────────┐
//!                   ▼        ▼        ▼
//!                  VQE      QAOA    other VQAs
//! ```
//!
//! `error.rs` is therefore a foundation contract.
//!
//! # Quantum IR integration
//!
//! The Quantum IR owns circuit/gate/qubit/measurement-specific errors.
//! This module does not duplicate those errors.
//!
//! Higher-level modules map IR failures into an appropriate
//! `AlgorithmError` variant.
//!
//! # Determinism and replay
//!
//! Stable `code()` and `kind()` values are intended for machine-readable
//! diagnostics, replay metadata, and programmatic control flow.
//!
//! Human-readable `Display` output must never be parsed for control flow.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::error::Error;

// =============================================================================
// Canonical Result
// =============================================================================

/// Canonical result type for the quantum algorithms subsystem.
pub type Result<T> = std::result::Result<T, AlgorithmError>;

// =============================================================================
// Error Kind
// =============================================================================

/// Stable machine-readable classification of an algorithm failure.
///
/// This type is intended for programmatic control flow.
///
/// Error messages must never be parsed to determine the error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmErrorKind {
    /// Generic caller input is invalid.
    InvalidInput,

    /// Algorithm configuration is invalid.
    InvalidConfiguration,

    /// Algorithm identity or algorithm-specific identity is invalid.
    InvalidAlgorithm,

    /// Qubit count is invalid.
    InvalidQubitCount,

    /// A parameter is invalid.
    InvalidParameter,

    /// Required dimensions do not agree.
    DimensionMismatch,

    /// A circuit is invalid at the algorithm boundary.
    InvalidCircuit,

    /// Requested operation is unsupported.
    UnsupportedOperation,

    /// An objective could not be evaluated.
    ObjectiveEvaluationFailed,

    /// Execution failed after the request was accepted.
    ExecutionFailed,

    /// Required backend is unavailable.
    BackendUnavailable,

    /// Execution exceeded a time limit.
    Timeout,

    /// Execution was cooperatively cancelled.
    Cancelled,

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded,

    /// A value was NaN or infinite.
    NonFiniteValue,

    /// Numerical processing became unstable.
    NumericalInstability,

    /// Required convergence was not achieved.
    ConvergenceFailure,

    /// The optimizer failed independently of convergence.
    OptimizationFailed,

    /// Determinism requirements were violated.
    DeterminismViolation,

    /// Serialization failed.
    SerializationFailure,

    /// Replay/reproduction validation failed.
    ReplayFailure,

    /// Version/schema compatibility failed.
    VersionMismatch,

    /// Internal invariant was violated.
    InternalInvariantViolation,
}

impl AlgorithmErrorKind {
    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidConfiguration => "invalid_configuration",
            Self::InvalidAlgorithm => "invalid_algorithm",
            Self::InvalidQubitCount => "invalid_qubit_count",
            Self::InvalidParameter => "invalid_parameter",
            Self::DimensionMismatch => "dimension_mismatch",
            Self::InvalidCircuit => "invalid_circuit",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::ObjectiveEvaluationFailed => {
                "objective_evaluation_failed"
            }
            Self::ExecutionFailed => "execution_failed",
            Self::BackendUnavailable => "backend_unavailable",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::ResourceLimitExceeded => {
                "resource_limit_exceeded"
            }
            Self::NonFiniteValue => "non_finite_value",
            Self::NumericalInstability => {
                "numerical_instability"
            }
            Self::ConvergenceFailure => "convergence_failure",
            Self::OptimizationFailed => "optimization_failed",
            Self::DeterminismViolation => {
                "determinism_violation"
            }
            Self::SerializationFailure => {
                "serialization_failure"
            }
            Self::ReplayFailure => "replay_failure",
            Self::VersionMismatch => "version_mismatch",
            Self::InternalInvariantViolation => {
                "internal_invariant_violation"
            }
        }
    }
}

impl fmt::Display for AlgorithmErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error Severity
// =============================================================================

/// Operational severity classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmErrorSeverity {
    /// Caller supplied invalid input.
    Input,

    /// Configuration or compatibility failure.
    Configuration,

    /// Backend or operational failure.
    Operational,

    /// Resource/time boundary.
    Resource,

    /// Intentional cancellation.
    Cancellation,

    /// Numerical/convergence failure.
    Numerical,

    /// Internal software defect.
    Internal,
}

impl AlgorithmErrorSeverity {
    /// Returns the stable machine-readable severity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Configuration => "configuration",
            Self::Operational => "operational",
            Self::Resource => "resource",
            Self::Cancellation => "cancellation",
            Self::Numerical => "numerical",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for AlgorithmErrorSeverity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource
// =============================================================================

/// Resource dimension associated with a resource-limit failure.
///
/// This remains independent of `types::AlgorithmLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmResource {
    Qubits,
    Gates,
    Depth,
    Shots,
    Iterations,
    ObjectiveEvaluations,
    GradientEvaluations,
    Parameters,
    CircuitExecutions,
    MemoryBytes,
    Time,
    OptimizerSteps,
    Custom,
}

impl AlgorithmResource {
    /// Returns the stable machine-readable resource name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qubits => "qubits",
            Self::Gates => "gates",
            Self::Depth => "depth",
            Self::Shots => "shots",
            Self::Iterations => "iterations",
            Self::ObjectiveEvaluations => {
                "objective_evaluations"
            }
            Self::GradientEvaluations => {
                "gradient_evaluations"
            }
            Self::Parameters => "parameters",
            Self::CircuitExecutions => {
                "circuit_executions"
            }
            Self::MemoryBytes => "memory_bytes",
            Self::Time => "time",
            Self::OptimizerSteps => "optimizer_steps",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for AlgorithmResource {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Canonical Error
// =============================================================================

/// Canonical error for the entire quantum algorithms subsystem.
///
/// The fields intentionally remain backend-independent.
///
/// This enum is the stable error boundary used by:
///
/// - `types.rs`;
/// - `execution.rs`;
/// - `objective.rs`;
/// - `optimizer.rs`;
/// - `variational.rs`;
/// - VQE;
/// - QAOA;
/// - Grover;
/// - amplitude algorithms;
/// - phase estimation.
///
/// No concrete algorithm should introduce a second public error vocabulary.
#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmError {
    /// Caller supplied invalid input.
    InvalidInput {
        /// Name of the invalid field/input.
        field: String,

        /// Structured human-readable reason.
        reason: String,
    },

    /// Algorithm configuration failed validation.
    InvalidConfiguration {
        /// Configuration field.
        field: String,

        /// Reason the configuration is invalid.
        reason: String,
    },

    /// Algorithm identity or algorithm-specific identity is invalid.
    InvalidAlgorithm {
        /// Algorithm identifier.
        algorithm: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Qubit count is invalid.
    InvalidQubitCount {
        /// Supplied qubit count.
        count: usize,

        /// Reason for rejection.
        message: String,
    },

    /// Parameter value/index/name is invalid.
    InvalidParameter {
        /// Parameter name or index representation.
        name: String,

        /// Reason for rejection.
        reason: String,
    },

    /// Required dimensions do not agree.
    DimensionMismatch {
        /// Name of the expected dimension.
        expected_name: String,

        /// Expected dimension.
        expected: usize,

        /// Name of the actual dimension.
        actual_name: String,

        /// Actual dimension.
        actual: usize,

        /// Human-readable explanation.
        message: String,
    },

    /// Circuit is invalid at the algorithm boundary.
    ///
    /// Detailed circuit semantics remain owned by the Quantum IR.
    InvalidCircuit {
        /// Optional circuit identifier.
        circuit: Option<String>,

        /// Mapped IR/algorithm diagnostic.
        message: String,
    },

    /// Requested operation is unsupported.
    UnsupportedOperation {
        /// Stable operation identifier.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// Objective evaluation failed.
    ObjectiveEvaluationFailed {
        /// Objective evaluation number when known.
        evaluation: Option<u64>,

        /// Explanation.
        message: String,
    },

    /// Execution failed after request acceptance.
    ExecutionFailed {
        /// Backend identifier when known.
        backend: Option<String>,

        /// Operation being executed.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// Backend required by the request is unavailable.
    BackendUnavailable {
        /// Backend identifier.
        backend: String,

        /// Explanation.
        message: String,
    },

    /// Operation exceeded a configured time limit.
    Timeout {
        /// Operation that timed out.
        operation: String,

        /// Timeout in nanoseconds when known.
        limit_nanos: Option<u64>,

        /// Explanation.
        message: String,
    },

    /// Operation was intentionally cancelled.
    Cancelled {
        /// Operation that was cancelled.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded {
        /// Stable resource identifier.
        resource: String,

        /// Requested amount.
        requested: u64,

        /// Configured maximum.
        limit: u64,

        /// Explanation.
        message: String,
    },

    /// A numerical value was non-finite.
    NonFiniteValue {
        /// Field or mathematical context.
        field: String,

        /// Invalid value.
        value: f64,
    },

    /// Numerical processing became unstable.
    NumericalInstability {
        /// Operation being evaluated.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// Required convergence was not achieved.
    ConvergenceFailure {
        /// Algorithm/optimizer identifier.
        algorithm: String,

        /// Number of iterations when known.
        iterations: Option<usize>,

        /// Explanation.
        message: String,
    },

    /// Optimizer itself failed.
    OptimizationFailed {
        /// Optimizer identifier.
        optimizer: String,

        /// Explanation.
        message: String,
    },

    /// Determinism contract was violated.
    DeterminismViolation {
        /// Determinism contract.
        contract: String,

        /// Explanation.
        message: String,
    },

    /// Serialization failed.
    SerializationFailure {
        /// Serialization operation.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// Replay validation failed.
    ReplayFailure {
        /// Replay operation.
        operation: String,

        /// Explanation.
        message: String,
    },

    /// Version/schema compatibility failed.
    VersionMismatch {
        /// Component whose versions differ.
        component: String,

        /// Expected version.
        expected: String,

        /// Actual version.
        actual: String,

        /// Explanation.
        message: String,
    },

    /// Internal software invariant was violated.
    InternalInvariantViolation {
        /// Name of violated invariant.
        invariant: String,

        /// Explanation.
        message: String,
    },
}

// =============================================================================
// Constructors
// =============================================================================

impl AlgorithmError {
    /// Creates an invalid-input error.
    #[must_use]
    pub fn invalid_input(
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidInput {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid-configuration error.
    #[must_use]
    pub fn invalid_configuration(
        field: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidConfiguration {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid-algorithm error.
    #[must_use]
    pub fn invalid_algorithm(
        algorithm: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidAlgorithm {
            algorithm: algorithm.into(),
            reason: reason.into(),
        }
    }

    /// Creates an invalid-qubit-count error.
    #[must_use]
    pub fn invalid_qubit_count(
        count: usize,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidQubitCount {
            count,
            message: message.into(),
        }
    }

    /// Creates an invalid-parameter error.
    #[must_use]
    pub fn invalid_parameter(
        name: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidParameter {
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Creates a dimension-mismatch error.
    #[must_use]
    pub fn dimension_mismatch(
        expected_name: impl Into<String>,
        expected: usize,
        actual_name: impl Into<String>,
        actual: usize,
        message: impl Into<String>,
    ) -> Self {
        Self::DimensionMismatch {
            expected_name: expected_name.into(),
            expected,
            actual_name: actual_name.into(),
            actual,
            message: message.into(),
        }
    }

    /// Creates an invalid-circuit error.
    #[must_use]
    pub fn invalid_circuit(
        circuit: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidCircuit {
            circuit,
            message: message.into(),
        }
    }

    /// Creates an unsupported-operation error.
    #[must_use]
    pub fn unsupported_operation(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates an objective-evaluation failure.
    #[must_use]
    pub fn objective_evaluation_failed(
        evaluation: Option<u64>,
        message: impl Into<String>,
    ) -> Self {
        Self::ObjectiveEvaluationFailed {
            evaluation,
            message: message.into(),
        }
    }

    /// Creates an execution failure.
    #[must_use]
    pub fn execution_failed(
        backend: Option<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ExecutionFailed {
            backend,
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a backend-unavailable error.
    #[must_use]
    pub fn backend_unavailable(
        backend: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::BackendUnavailable {
            backend: backend.into(),
            message: message.into(),
        }
    }

    /// Creates a timeout error.
    #[must_use]
    pub fn timeout(
        operation: impl Into<String>,
        limit_nanos: Option<u64>,
        message: impl Into<String>,
    ) -> Self {
        Self::Timeout {
            operation: operation.into(),
            limit_nanos,
            message: message.into(),
        }
    }

    /// Creates a cancellation error.
    #[must_use]
    pub fn cancelled(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Cancelled {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a resource-limit error.
    #[must_use]
    pub fn resource_limit_exceeded(
        resource: impl Into<String>,
        requested: u64,
        limit: u64,
        message: impl Into<String>,
    ) -> Self {
        Self::ResourceLimitExceeded {
            resource: resource.into(),
            requested,
            limit,
            message: message.into(),
        }
    }

    /// Creates a non-finite-value error.
    #[must_use]
    pub fn non_finite_value(
        field: impl Into<String>,
        value: f64,
    ) -> Self {
        Self::NonFiniteValue {
            field: field.into(),
            value,
        }
    }

    /// Creates a numerical-instability error.
    #[must_use]
    pub fn numerical_instability(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::NumericalInstability {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a convergence failure.
    #[must_use]
    pub fn convergence_failure(
        algorithm: impl Into<String>,
        iterations: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self::ConvergenceFailure {
            algorithm: algorithm.into(),
            iterations,
            message: message.into(),
        }
    }

    /// Creates an optimization failure.
    #[must_use]
    pub fn optimization_failed(
        optimizer: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::OptimizationFailed {
            optimizer: optimizer.into(),
            message: message.into(),
        }
    }

    /// Creates a determinism violation.
    #[must_use]
    pub fn determinism_violation(
        contract: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::DeterminismViolation {
            contract: contract.into(),
            message: message.into(),
        }
    }

    /// Creates a serialization failure.
    #[must_use]
    pub fn serialization_failure(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::SerializationFailure {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a replay failure.
    #[must_use]
    pub fn replay_failure(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ReplayFailure {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a version mismatch.
    #[must_use]
    pub fn version_mismatch(
        component: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::VersionMismatch {
            component: component.into(),
            expected: expected.into(),
            actual: actual.into(),
            message: message.into(),
        }
    }

    /// Creates an internal invariant failure.
    #[must_use]
    pub fn internal_invariant(
        invariant: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InternalInvariantViolation {
            invariant: invariant.into(),
            message: message.into(),
        }
    }
}

// =============================================================================
// Classification
// =============================================================================

impl AlgorithmError {
    /// Returns the stable error kind.
    #[must_use]
    pub const fn kind(&self) -> AlgorithmErrorKind {
        match self {
            Self::InvalidInput { .. } => {
                AlgorithmErrorKind::InvalidInput
            }
            Self::InvalidConfiguration { .. } => {
                AlgorithmErrorKind::InvalidConfiguration
            }
            Self::InvalidAlgorithm { .. } => {
                AlgorithmErrorKind::InvalidAlgorithm
            }
            Self::InvalidQubitCount { .. } => {
                AlgorithmErrorKind::InvalidQubitCount
            }
            Self::InvalidParameter { .. } => {
                AlgorithmErrorKind::InvalidParameter
            }
            Self::DimensionMismatch { .. } => {
                AlgorithmErrorKind::DimensionMismatch
            }
            Self::InvalidCircuit { .. } => {
                AlgorithmErrorKind::InvalidCircuit
            }
            Self::UnsupportedOperation { .. } => {
                AlgorithmErrorKind::UnsupportedOperation
            }
            Self::ObjectiveEvaluationFailed { .. } => {
                AlgorithmErrorKind::ObjectiveEvaluationFailed
            }
            Self::ExecutionFailed { .. } => {
                AlgorithmErrorKind::ExecutionFailed
            }
            Self::BackendUnavailable { .. } => {
                AlgorithmErrorKind::BackendUnavailable
            }
            Self::Timeout { .. } => {
                AlgorithmErrorKind::Timeout
            }
            Self::Cancelled { .. } => {
                AlgorithmErrorKind::Cancelled
            }
            Self::ResourceLimitExceeded { .. } => {
                AlgorithmErrorKind::ResourceLimitExceeded
            }
            Self::NonFiniteValue { .. } => {
                AlgorithmErrorKind::NonFiniteValue
            }
            Self::NumericalInstability { .. } => {
                AlgorithmErrorKind::NumericalInstability
            }
            Self::ConvergenceFailure { .. } => {
                AlgorithmErrorKind::ConvergenceFailure
            }
            Self::OptimizationFailed { .. } => {
                AlgorithmErrorKind::OptimizationFailed
            }
            Self::DeterminismViolation { .. } => {
                AlgorithmErrorKind::DeterminismViolation
            }
            Self::SerializationFailure { .. } => {
                AlgorithmErrorKind::SerializationFailure
            }
            Self::ReplayFailure { .. } => {
                AlgorithmErrorKind::ReplayFailure
            }
            Self::VersionMismatch { .. } => {
                AlgorithmErrorKind::VersionMismatch
            }
            Self::InternalInvariantViolation { .. } => {
                AlgorithmErrorKind::InternalInvariantViolation
            }
        }
    }

    /// Returns the stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind().as_str()
    }

    /// Returns the operational severity.
    #[must_use]
    pub const fn severity(&self) -> AlgorithmErrorSeverity {
        match self {
            Self::InvalidInput { .. }
            | Self::InvalidQubitCount { .. }
            | Self::InvalidParameter { .. }
            | Self::DimensionMismatch { .. }
            | Self::InvalidCircuit { .. } => {
                AlgorithmErrorSeverity::Input
            }

            Self::InvalidConfiguration { .. }
            | Self::InvalidAlgorithm { .. }
            | Self::UnsupportedOperation { .. }
            | Self::VersionMismatch { .. } => {
                AlgorithmErrorSeverity::Configuration
            }

            Self::ObjectiveEvaluationFailed { .. }
            | Self::ExecutionFailed { .. }
            | Self::BackendUnavailable { .. }
            | Self::SerializationFailure { .. }
            | Self::ReplayFailure { .. } => {
                AlgorithmErrorSeverity::Operational
            }

            Self::Timeout { .. }
            | Self::ResourceLimitExceeded { .. } => {
                AlgorithmErrorSeverity::Resource
            }

            Self::Cancelled { .. } => {
                AlgorithmErrorSeverity::Cancellation
            }

            Self::NonFiniteValue { .. }
            | Self::NumericalInstability { .. }
            | Self::ConvergenceFailure { .. }
            | Self::OptimizationFailed { .. }
            | Self::DeterminismViolation { .. } => {
                AlgorithmErrorSeverity::Numerical
            }

            Self::InternalInvariantViolation { .. } => {
                AlgorithmErrorSeverity::Internal
            }
        }
    }

    /// Returns whether retrying the exact request may reasonably succeed.
    ///
    /// This classification is deliberately conservative.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        match self {
            Self::BackendUnavailable { .. }
            | Self::ExecutionFailed { .. }
            | Self::Timeout { .. } => true,

            Self::InvalidInput { .. }
            | Self::InvalidConfiguration { .. }
            | Self::InvalidAlgorithm { .. }
            | Self::InvalidQubitCount { .. }
            | Self::InvalidParameter { .. }
            | Self::DimensionMismatch { .. }
            | Self::InvalidCircuit { .. }
            | Self::UnsupportedOperation { .. }
            | Self::ObjectiveEvaluationFailed { .. }
            | Self::Cancelled { .. }
            | Self::ResourceLimitExceeded { .. }
            | Self::NonFiniteValue { .. }
            | Self::NumericalInstability { .. }
            | Self::ConvergenceFailure { .. }
            | Self::OptimizationFailed { .. }
            | Self::DeterminismViolation { .. }
            | Self::SerializationFailure { .. }
            | Self::ReplayFailure { .. }
            | Self::VersionMismatch { .. }
            | Self::InternalInvariantViolation { .. } => false,
        }
    }

    /// Returns whether the error originated from caller-controlled input.
    #[must_use]
    pub const fn is_input_error(&self) -> bool {
        matches!(
            self.severity(),
            AlgorithmErrorSeverity::Input
        )
    }

    /// Returns whether the error represents an internal defect.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(
            self,
            Self::InternalInvariantViolation { .. }
        )
    }

    /// Returns whether the error represents an explicit resource,
    /// timeout, or cancellation boundary.
    #[must_use]
    pub const fn is_resource_boundary(&self) -> bool {
        matches!(
            self,
            Self::ResourceLimitExceeded { .. }
                | Self::Timeout { .. }
                | Self::Cancelled { .. }
        )
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for AlgorithmError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidInput {
                field,
                reason,
            } => write!(
                formatter,
                "invalid algorithm input '{field}': {reason}"
            ),

            Self::InvalidConfiguration {
                field,
                reason,
            } => write!(
                formatter,
                "invalid algorithm configuration '{field}': {reason}"
            ),

            Self::InvalidAlgorithm {
                algorithm,
                reason,
            } => write!(
                formatter,
                "invalid algorithm '{algorithm}': {reason}"
            ),

            Self::InvalidQubitCount {
                count,
                message,
            } => write!(
                formatter,
                "invalid qubit count {count}: {message}"
            ),

            Self::InvalidParameter {
                name,
                reason,
            } => write!(
                formatter,
                "invalid parameter '{name}': {reason}"
            ),

            Self::DimensionMismatch {
                expected_name,
                expected,
                actual_name,
                actual,
                message,
            } => write!(
                formatter,
                "dimension mismatch: \
                 {expected_name}={expected}, \
                 {actual_name}={actual}: {message}"
            ),

            Self::InvalidCircuit {
                circuit,
                message,
            } => {
                if let Some(circuit) = circuit {
                    write!(
                        formatter,
                        "invalid circuit '{circuit}': {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "invalid circuit: {message}"
                    )
                }
            }

            Self::UnsupportedOperation {
                operation,
                message,
            } => write!(
                formatter,
                "unsupported operation '{operation}': {message}"
            ),

            Self::ObjectiveEvaluationFailed {
                evaluation,
                message,
            } => {
                if let Some(evaluation) = evaluation {
                    write!(
                        formatter,
                        "objective evaluation {evaluation} failed: \
                         {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "objective evaluation failed: {message}"
                    )
                }
            }

            Self::ExecutionFailed {
                backend,
                operation,
                message,
            } => {
                if let Some(backend) = backend {
                    write!(
                        formatter,
                        "quantum execution failed on \
                         backend '{backend}' during \
                         '{operation}': {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "quantum execution failed during \
                         '{operation}': {message}"
                    )
                }
            }

            Self::BackendUnavailable {
                backend,
                message,
            } => write!(
                formatter,
                "backend '{backend}' unavailable: {message}"
            ),

            Self::Timeout {
                operation,
                limit_nanos,
                message,
            } => {
                if let Some(limit_nanos) = limit_nanos {
                    write!(
                        formatter,
                        "operation '{operation}' timed out \
                         after {limit_nanos} ns: {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "operation '{operation}' timed out: \
                         {message}"
                    )
                }
            }

            Self::Cancelled {
                operation,
                message,
            } => write!(
                formatter,
                "operation '{operation}' was cancelled: {message}"
            ),

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
                message,
            } => write!(
                formatter,
                "resource limit exceeded for {resource}: \
                 requested {requested}, limit {limit}: {message}"
            ),

            Self::NonFiniteValue {
                field,
                value,
            } => write!(
                formatter,
                "non-finite value in '{field}': {value}"
            ),

            Self::NumericalInstability {
                operation,
                message,
            } => write!(
                formatter,
                "numerical instability during \
                 '{operation}': {message}"
            ),

            Self::ConvergenceFailure {
                algorithm,
                iterations,
                message,
            } => {
                if let Some(iterations) = iterations {
                    write!(
                        formatter,
                        "algorithm '{algorithm}' failed to \
                         converge after {iterations} iterations: \
                         {message}"
                    )
                } else {
                    write!(
                        formatter,
                        "algorithm '{algorithm}' failed to \
                         converge: {message}"
                    )
                }
            }

            Self::OptimizationFailed {
                optimizer,
                message,
            } => write!(
                formatter,
                "optimizer '{optimizer}' failed: {message}"
            ),

            Self::DeterminismViolation {
                contract,
                message,
            } => write!(
                formatter,
                "determinism contract '{contract}' violated: \
                 {message}"
            ),

            Self::SerializationFailure {
                operation,
                message,
            } => write!(
                formatter,
                "serialization operation '{operation}' failed: \
                 {message}"
            ),

            Self::ReplayFailure {
                operation,
                message,
            } => write!(
                formatter,
                "replay operation '{operation}' failed: {message}"
            ),

            Self::VersionMismatch {
                component,
                expected,
                actual,
                message,
            } => write!(
                formatter,
                "version mismatch for '{component}': \
                 expected {expected}, actual {actual}: {message}"
            ),

            Self::InternalInvariantViolation {
                invariant,
                message,
            } => write!(
                formatter,
                "internal algorithm invariant \
                 '{invariant}' violated: {message}"
            ),
        }
    }
}

impl Error for AlgorithmError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_uses_algorithm_error() {
        let result: Result<()> =
            Err(AlgorithmError::invalid_input(
                "test",
                "invalid value",
            ));

        assert!(result.is_err());
    }

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(
            AlgorithmErrorKind::InvalidInput.as_str(),
            "invalid_input"
        );

        assert_eq!(
            AlgorithmErrorKind::ExecutionFailed.as_str(),
            "execution_failed"
        );

        assert_eq!(
            AlgorithmErrorKind::ResourceLimitExceeded.as_str(),
            "resource_limit_exceeded"
        );
    }

    #[test]
    fn invalid_input_is_classified_as_input() {
        let error =
            AlgorithmError::invalid_input(
                "parameter",
                "value is invalid",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::InvalidInput
        );

        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Input
        );

        assert!(error.is_input_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn invalid_parameter_matches_types_contract() {
        let error =
            AlgorithmError::invalid_parameter(
                "parameter[0]",
                "value must be finite",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::InvalidParameter
        );

        assert!(error.to_string().contains("parameter[0]"));
    }

    #[test]
    fn qubit_error_matches_execution_contract() {
        let error =
            AlgorithmError::invalid_qubit_count(
                0,
                "qubit count must be positive",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::InvalidQubitCount
        );

        assert!(error.to_string().contains("0"));
    }

    #[test]
    fn resource_error_is_structured() {
        let error =
            AlgorithmError::resource_limit_exceeded(
                "qubits",
                128,
                64,
                "requested circuit exceeds configured limit",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::ResourceLimitExceeded
        );

        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Resource
        );

        assert!(error.is_resource_boundary());
        assert!(!error.is_internal());
        assert!(!error.is_retryable());
    }

    #[test]
    fn backend_unavailable_is_retryable() {
        let error =
            AlgorithmError::backend_unavailable(
                "reference-simulator",
                "backend is unavailable",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::BackendUnavailable
        );

        assert!(error.is_retryable());
        assert!(!error.is_input_error());
    }

    #[test]
    fn execution_failure_is_retryable() {
        let error =
            AlgorithmError::execution_failed(
                Some("simulator".to_owned()),
                "measurement",
                "execution failed",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::ExecutionFailed
        );

        assert!(error.is_retryable());
    }

    #[test]
    fn timeout_is_retryable() {
        let error =
            AlgorithmError::timeout(
                "execute",
                Some(1_000_000),
                "deadline exceeded",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::Timeout
        );

        assert!(error.is_retryable());
        assert!(error.is_resource_boundary());
    }

    #[test]
    fn cancellation_is_not_retryable_by_default() {
        let error =
            AlgorithmError::cancelled(
                "optimization",
                "caller requested cancellation",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::Cancelled
        );

        assert!(!error.is_retryable());
        assert!(error.is_resource_boundary());
    }

    #[test]
    fn deterministic_contract_failure_is_structured() {
        let error =
            AlgorithmError::determinism_violation(
                "deterministic execution",
                "backend reported nondeterministic execution",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::DeterminismViolation
        );

        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Numerical
        );
    }

    #[test]
    fn non_finite_value_is_classified_correctly() {
        let error =
            AlgorithmError::non_finite_value(
                "objective",
                f64::NAN,
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::NonFiniteValue
        );

        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Numerical
        );

        assert!(!error.is_retryable());
    }

    #[test]
    fn internal_invariant_is_internal() {
        let error =
            AlgorithmError::internal_invariant(
                "parameter_count_matches_ansatz",
                "internal state became inconsistent",
            );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::InternalInvariantViolation
        );

        assert!(error.is_internal());
        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Internal
        );

        assert!(!error.is_retryable());
    }

    #[test]
    fn dimension_mismatch_preserves_context() {
        let error =
            AlgorithmError::dimension_mismatch(
                "ansatz_parameters",
                4,
                "provided_parameters",
                3,
                "parameter counts must match",
            );

        let text = error.to_string();

        assert!(text.contains("ansatz_parameters=4"));
        assert!(text.contains("provided_parameters=3"));
    }

    #[test]
    fn display_is_diagnostic_only() {
        let error =
            AlgorithmError::execution_failed(
                Some("reference-simulator".to_owned()),
                "expectation",
                "invalid backend result",
            );

        let text = error.to_string();

        assert!(text.contains("reference-simulator"));
        assert!(text.contains("expectation"));
        assert!(text.contains("invalid backend result"));

        assert_eq!(
            error.code(),
            "execution_failed"
        );
    }

    #[test]
    fn resource_names_are_stable() {
        assert_eq!(
            AlgorithmResource::Qubits.as_str(),
            "qubits"
        );

        assert_eq!(
            AlgorithmResource::ObjectiveEvaluations.as_str(),
            "objective_evaluations"
        );

        assert_eq!(
            AlgorithmResource::CircuitExecutions.as_str(),
            "circuit_executions"
        );
    }
}