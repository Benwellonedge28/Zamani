//! Zamani Quantum Algorithms — Canonical Error Boundary.
//!
//! This module defines the single error contract for the complete
//! `quantum::algorithms` subsystem.
//!
//! # Architectural responsibility
//!
//! `error.rs` owns:
//!
//! - canonical algorithm error representation;
//! - stable machine-readable error codes;
//! - stable high-level error classification;
//! - error severity classification;
//! - retryability classification;
//! - human-readable diagnostics;
//! - the canonical `Result<T>` alias;
//! - constructors for common algorithm failures.
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
//! - error correction;
//! - optimization policy;
//! - resource policy;
//! - persistence;
//! - telemetry transport.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Dependency direction
//!
//! This module intentionally has no dependency on sibling algorithm modules.
//!
//! ```text
//!                    error.rs
//!                       │
//!          ┌────────────┼────────────┐
//!          │            │            │
//!          ▼            ▼            ▼
//!       types.rs   execution.rs  objective.rs
//!          │            │            │
//!          └────────────┼────────────┘
//!                       ▼
//!                 optimizer.rs
//!                       │
//!                       ▼
//!                variational.rs
//!                       │
//!             ┌─────────┼─────────┐
//!             ▼         ▼         ▼
//!            VQE       QAOA     other VQAs
//!
//! error.rs is therefore a foundation contract, not an integration layer.
//! ```
//!
//! # Integration with Quantum IR
//!
//! The quantum IR owns its own domain errors such as `GateError`,
//! `MeasurementError`, and `QubitError`. This module deliberately does not
//! redefine them.
//!
//! Higher-level algorithm modules may map those errors into
//! `AlgorithmError::InvalidCircuit`, `AlgorithmError::ExecutionFailed`, or
//! another appropriate algorithm-level category.
//!
//! ```text
//! quantum::ir::*Error
//!        │
//!        ▼
//! algorithm-level mapping
//!        │
//!        ▼
//! AlgorithmError
//! ```
//!
//! # Integration with execution
//!
//! `execution.rs` should use this boundary for failures such as:
//!
//! - invalid execution requests;
//! - backend unavailability;
//! - execution failure;
//! - timeout;
//! - cancellation;
//! - resource exhaustion;
//! - unsupported execution modes;
//! - non-finite execution results.
//!
//! # Integration with optimization
//!
//! `optimizer.rs` should use this boundary for:
//!
//! - invalid optimization configuration;
//! - objective evaluation failure;
//! - non-finite objective values;
//! - numerical instability;
//! - convergence failure;
//! - optimizer divergence;
//! - resource exhaustion;
//! - invalid parameter updates.
//!
//! # Integration with deterministic execution
//!
//! Deterministic execution is configured by higher-level modules.
//! This file only provides the canonical errors for deterministic-contract
//! violations.
//!
//! # Integration with replay/versioning
//!
//! Algorithm replay and versioning may use the stable `code()` and `kind()`
//! values from this module. Error diagnostics must never be parsed to make
//! control-flow decisions.
//!
//! # Rust compatibility
//!
//! This file is intentionally implemented using stable Rust standard-library
//! functionality compatible with Rust 1.97.1.
//!
//! No unstable features are required.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! Invalid external input must be represented as an error rather than causing
//! a panic.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::error::Error;

// ============================================================================
// Canonical Result
// ============================================================================

/// Canonical result type for the complete quantum algorithms subsystem.
///
/// All public algorithm APIs should eventually return this result type
/// instead of defining independent algorithm-specific result aliases.
pub type Result<T> = std::result::Result<T, AlgorithmError>;

// ============================================================================
// Error Kind
// ============================================================================

/// Stable high-level classification of an algorithm failure.
///
/// Consumers should use this type for control flow instead of matching
/// diagnostic strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmErrorKind {
    /// Caller supplied malformed or invalid input.
    InvalidInput,

    /// Algorithm configuration is invalid.
    InvalidConfiguration,

    /// Algorithm identifier or version is invalid/incompatible.
    InvalidAlgorithm,

    /// Qubit count or qubit-related configuration is invalid.
    InvalidQubitCount,

    /// Parameter vector or parameter value is invalid.
    InvalidParameter,

    /// Two dimensions that must agree do not agree.
    DimensionMismatch,

    /// Circuit supplied to or generated by an algorithm is invalid.
    InvalidCircuit,

    /// Requested operation is not supported by the algorithm contract.
    UnsupportedOperation,

    /// An objective could not be evaluated successfully.
    ObjectiveEvaluationFailed,

    /// Execution of a logical quantum program failed.
    ExecutionFailed,

    /// Required execution backend is unavailable.
    BackendUnavailable,

    /// Execution or optimization exceeded a configured time limit.
    Timeout,

    /// Cooperative cancellation stopped execution.
    Cancelled,

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded,

    /// A numerical value was NaN or infinite when a finite value was required.
    NonFiniteValue,

    /// Numerical calculations became unstable or invalid.
    NumericalInstability,

    /// Optimization failed to satisfy its convergence contract.
    ConvergenceFailure,

    /// The optimizer itself failed.
    OptimizationFailed,

    /// Deterministic/reproducible execution requirements were violated.
    DeterminismViolation,

    /// Serialization or deserialization failed.
    SerializationFailure,

    /// Replay or reproduction validation failed.
    ReplayFailure,

    /// Version/schema compatibility failed.
    VersionMismatch,

    /// An internal invariant was violated.
    InternalInvariantViolation,
}

impl AlgorithmErrorKind {
    /// Returns the stable machine-readable error category.
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
            Self::ObjectiveEvaluationFailed => "objective_evaluation_failed",
            Self::ExecutionFailed => "execution_failed",
            Self::BackendUnavailable => "backend_unavailable",
            Self::Timeout => "timeout",
            Self::Cancelled => "cancelled",
            Self::ResourceLimitExceeded => "resource_limit_exceeded",
            Self::NonFiniteValue => "non_finite_value",
            Self::NumericalInstability => "numerical_instability",
            Self::ConvergenceFailure => "convergence_failure",
            Self::OptimizationFailed => "optimization_failed",
            Self::DeterminismViolation => "determinism_violation",
            Self::SerializationFailure => "serialization_failure",
            Self::ReplayFailure => "replay_failure",
            Self::VersionMismatch => "version_mismatch",
            Self::InternalInvariantViolation => "internal_invariant_violation",
        }
    }
}

impl fmt::Display for AlgorithmErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Error Severity
// ============================================================================

/// Operational severity/classification of an algorithm error.
///
/// This classification is intentionally independent from telemetry systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmErrorSeverity {
    /// Caller supplied invalid input.
    Input,

    /// Configuration is invalid or incompatible.
    Configuration,

    /// The requested operation could not be performed by the available
    /// algorithm/backend.
    Operational,

    /// A resource or execution boundary was reached.
    Resource,

    /// Execution was intentionally stopped.
    Cancellation,

    /// A numerical or convergence condition prevented reliable completion.
    Numerical,

    /// An internal software invariant was violated.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Resource Classification
// ============================================================================

/// Resource dimensions that the algorithms subsystem may enforce.
///
/// This is classification only.
///
/// Resource policy belongs to `types.rs` / future resource-policy contracts,
/// while execution accounting belongs to `execution.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgorithmResource {
    /// Logical qubits required by an algorithm.
    Qubits,

    /// Quantum circuit gate count.
    Gates,

    /// Circuit depth.
    CircuitDepth,

    /// Number of measurement shots.
    Shots,

    /// Number of algorithm iterations.
    Iterations,

    /// Number of objective evaluations.
    ObjectiveEvaluations,

    /// Number of gradient evaluations.
    GradientEvaluations,

    /// Number of circuits submitted for execution.
    CircuitExecutions,

    /// Estimated/consumed memory in bytes.
    MemoryBytes,

    /// Execution time.
    Time,

    /// Number of optimizer steps.
    OptimizerSteps,

    /// Number of parameters.
    Parameters,

    /// Algorithm-specific resource dimension.
    Custom,
}

impl AlgorithmResource {
    /// Returns the stable machine-readable resource identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qubits => "qubits",
            Self::Gates => "gates",
            Self::CircuitDepth => "circuit_depth",
            Self::Shots => "shots",
            Self::Iterations => "iterations",
            Self::ObjectiveEvaluations => "objective_evaluations",
            Self::GradientEvaluations => "gradient_evaluations",
            Self::CircuitExecutions => "circuit_executions",
            Self::MemoryBytes => "memory_bytes",
            Self::Time => "time",
            Self::OptimizerSteps => "optimizer_steps",
            Self::Parameters => "parameters",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for AlgorithmResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Canonical Error
// ============================================================================

/// Canonical error returned by the quantum algorithms subsystem.
///
/// This enum intentionally contains structured information where that
/// information is useful to callers, while avoiding dependencies on future
/// sibling modules.
///
/// The enum is the stable error boundary. New algorithm implementations
/// should map their failures into these categories rather than introducing
/// independent public error enums.
#[derive(Debug, Clone, PartialEq)]
pub enum AlgorithmError {
    /// Generic malformed or invalid caller input.
    InvalidInput {
        /// Human-readable diagnostic.
        message: String,
    },

    /// Algorithm configuration failed validation.
    InvalidConfiguration {
        /// Configuration field responsible for the failure.
        field: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Algorithm identity or algorithm-specific configuration is invalid.
    InvalidAlgorithm {
        /// Algorithm identifier.
        algorithm: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Invalid qubit count or qubit-related input.
    InvalidQubitCount {
        /// Supplied count.
        count: usize,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Invalid parameter value or parameter index.
    InvalidParameter {
        /// Parameter index when applicable.
        index: Option<usize>,

        /// Supplied value when applicable.
        value: Option<f64>,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Two dimensions that must agree are incompatible.
    DimensionMismatch {
        /// Name of the first dimension.
        expected_name: String,

        /// Expected dimension.
        expected: usize,

        /// Name of the actual dimension.
        actual_name: String,

        /// Actual dimension.
        actual: usize,

        /// Human-readable diagnostic.
        message: String,
    },

    /// A circuit failed algorithm-level validation.
    ///
    /// The IR itself remains responsible for its own detailed circuit errors.
    InvalidCircuit {
        /// Optional circuit identifier or context.
        circuit: Option<String>,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Requested operation is outside the supported algorithm contract.
    UnsupportedOperation {
        /// Operation that was requested.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// An objective function could not be evaluated.
    ObjectiveEvaluationFailed {
        /// Evaluation number when known.
        evaluation: Option<u64>,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Quantum execution failed after an execution request was accepted.
    ExecutionFailed {
        /// Backend identifier when known.
        backend: Option<String>,

        /// Operation being executed.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Required backend was not available.
    BackendUnavailable {
        /// Backend identifier.
        backend: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Execution or algorithm processing exceeded a time limit.
    Timeout {
        /// Operation that timed out.
        operation: String,

        /// Configured limit in nanoseconds, when known.
        limit_nanos: Option<u64>,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Cooperative cancellation stopped the operation.
    Cancelled {
        /// Operation that was cancelled.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// A configured resource limit was exceeded.
    ResourceLimitExceeded {
        /// Resource dimension.
        resource: AlgorithmResource,

        /// Requested amount.
        requested: u128,

        /// Configured limit.
        limit: u128,

        /// Human-readable diagnostic.
        message: String,
    },

    /// A value was non-finite where a finite value was required.
    NonFiniteValue {
        /// Context in which the value was encountered.
        context: String,

        /// Index when applicable.
        index: Option<usize>,

        /// Invalid value.
        value: f64,

        /// Human-readable diagnostic.
        message: String,
    },

    /// A numerical calculation became unstable.
    NumericalInstability {
        /// Operation producing the instability.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// The requested convergence contract was not satisfied.
    ConvergenceFailure {
        /// Algorithm/optimizer context.
        algorithm: String,

        /// Number of iterations completed.
        iterations: Option<usize>,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Classical optimization itself failed.
    OptimizationFailed {
        /// Optimizer identifier.
        optimizer: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Deterministic execution/reproduction requirements were violated.
    DeterminismViolation {
        /// Determinism contract being enforced.
        contract: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Algorithm configuration/result serialization failed.
    SerializationFailure {
        /// Serialization operation.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Replay/reproduction validation failed.
    ReplayFailure {
        /// Replay operation.
        operation: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// Algorithm/schema/backend compatibility failed.
    VersionMismatch {
        /// Component whose version was incompatible.
        component: String,

        /// Expected version.
        expected: String,

        /// Actual version.
        actual: String,

        /// Human-readable diagnostic.
        message: String,
    },

    /// An internal invariant was violated.
    ///
    /// This represents a software defect rather than ordinary malformed
    /// caller input.
    InternalInvariantViolation {
        /// Name of the violated invariant.
        invariant: String,

        /// Human-readable diagnostic.
        message: String,
    },
}

// ============================================================================
// Constructors
// ============================================================================

impl AlgorithmError {
    /// Creates an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Creates an invalid-configuration error.
    #[must_use]
    pub fn invalid_configuration(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidConfiguration {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Creates an invalid-algorithm error.
    #[must_use]
    pub fn invalid_algorithm(
        algorithm: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidAlgorithm {
            algorithm: algorithm.into(),
            message: message.into(),
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
        index: Option<usize>,
        value: Option<f64>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidParameter {
            index,
            value,
            message: message.into(),
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

    /// Creates an objective-evaluation error.
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

    /// Creates an execution error.
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
        resource: AlgorithmResource,
        requested: u128,
        limit: u128,
        message: impl Into<String>,
    ) -> Self {
        Self::ResourceLimitExceeded {
            resource,
            requested,
            limit,
            message: message.into(),
        }
    }

    /// Creates a non-finite-value error.
    #[must_use]
    pub fn non_finite_value(
        context: impl Into<String>,
        index: Option<usize>,
        value: f64,
        message: impl Into<String>,
    ) -> Self {
        Self::NonFiniteValue {
            context: context.into(),
            index,
            value,
            message: message.into(),
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

    /// Creates a convergence-failure error.
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

    /// Creates an optimization-failure error.
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

    /// Creates a determinism-contract violation.
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

    /// Creates an internal invariant violation.
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

// ============================================================================
// Classification
// ============================================================================

impl AlgorithmError {
    /// Returns the stable high-level error category.
    #[must_use]
    pub const fn kind(&self) -> AlgorithmErrorKind {
        match self {
            Self::InvalidInput { .. } => AlgorithmErrorKind::InvalidInput,
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
            Self::Timeout { .. } => AlgorithmErrorKind::Timeout,
            Self::Cancelled { .. } => AlgorithmErrorKind::Cancelled,
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
            Self::ReplayFailure { .. } => AlgorithmErrorKind::ReplayFailure,
            Self::VersionMismatch { .. } => {
                AlgorithmErrorKind::VersionMismatch
            }
            Self::InternalInvariantViolation { .. } => {
                AlgorithmErrorKind::InternalInvariantViolation
            }
        }
    }

    /// Returns the stable machine-readable error code.
    ///
    /// These codes are intended for logs, telemetry, replay records and
    /// programmatic consumers. They must not depend on human-readable text.
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
            | Self::InvalidCircuit { .. } => AlgorithmErrorSeverity::Input,

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
            | Self::ReplayFailure { .. } => AlgorithmErrorSeverity::Operational,

            Self::Timeout { .. }
            | Self::ResourceLimitExceeded { .. } => {
                AlgorithmErrorSeverity::Resource
            }

            Self::Cancelled { .. } => AlgorithmErrorSeverity::Cancellation,

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

    /// Returns whether retrying the exact same operation may reasonably
    /// succeed without changing the request.
    ///
    /// This is deliberately conservative.
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

    /// Returns whether the error is caused by caller-controlled input.
    #[must_use]
    pub const fn is_input_error(&self) -> bool {
        matches!(
            self.severity(),
            AlgorithmErrorSeverity::Input
        )
    }

    /// Returns whether the error represents an internal software defect.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(
            self,
            Self::InternalInvariantViolation { .. }
        )
    }

    /// Returns whether the failure represents an intentional resource or
    /// execution boundary.
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

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message } => {
                write!(f, "invalid algorithm input: {message}")
            }

            Self::InvalidConfiguration {
                field,
                message,
            } => {
                write!(
                    f,
                    "invalid algorithm configuration for '{field}': {message}"
                )
            }

            Self::InvalidAlgorithm {
                algorithm,
                message,
            } => {
                write!(
                    f,
                    "invalid algorithm '{algorithm}': {message}"
                )
            }

            Self::InvalidQubitCount {
                count,
                message,
            } => {
                write!(
                    f,
                    "invalid qubit count {count}: {message}"
                )
            }

            Self::InvalidParameter {
                index,
                value,
                message,
            } => {
                match (index, value) {
                    (Some(index), Some(value)) => write!(
                        f,
                        "invalid parameter at index {index} with value {value}: {message}"
                    ),
                    (Some(index), None) => write!(
                        f,
                        "invalid parameter at index {index}: {message}"
                    ),
                    (None, Some(value)) => write!(
                        f,
                        "invalid parameter value {value}: {message}"
                    ),
                    (None, None) => {
                        write!(f, "invalid parameter: {message}")
                    }
                }
            }

            Self::DimensionMismatch {
                expected_name,
                expected,
                actual_name,
                actual,
                message,
            } => {
                write!(
                    f,
                    "dimension mismatch: {expected_name}={expected}, \
                     {actual_name}={actual}: {message}"
                )
            }

            Self::InvalidCircuit {
                circuit,
                message,
            } => {
                if let Some(circuit) = circuit {
                    write!(
                        f,
                        "invalid circuit '{circuit}': {message}"
                    )
                } else {
                    write!(f, "invalid circuit: {message}")
                }
            }

            Self::UnsupportedOperation {
                operation,
                message,
            } => {
                write!(
                    f,
                    "unsupported operation '{operation}': {message}"
                )
            }

            Self::ObjectiveEvaluationFailed {
                evaluation,
                message,
            } => {
                if let Some(evaluation) = evaluation {
                    write!(
                        f,
                        "objective evaluation {evaluation} failed: {message}"
                    )
                } else {
                    write!(
                        f,
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
                        f,
                        "quantum execution failed on backend \
                         '{backend}' during '{operation}': {message}"
                    )
                } else {
                    write!(
                        f,
                        "quantum execution failed during \
                         '{operation}': {message}"
                    )
                }
            }

            Self::BackendUnavailable {
                backend,
                message,
            } => {
                write!(
                    f,
                    "backend '{backend}' unavailable: {message}"
                )
            }

            Self::Timeout {
                operation,
                limit_nanos,
                message,
            } => {
                if let Some(limit_nanos) = limit_nanos {
                    write!(
                        f,
                        "operation '{operation}' timed out after \
                         {limit_nanos} ns: {message}"
                    )
                } else {
                    write!(
                        f,
                        "operation '{operation}' timed out: {message}"
                    )
                }
            }

            Self::Cancelled {
                operation,
                message,
            } => {
                write!(
                    f,
                    "operation '{operation}' was cancelled: {message}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
                message,
            } => {
                write!(
                    f,
                    "algorithm resource limit exceeded for {resource}: \
                     requested {requested}, limit {limit}: {message}"
                )
            }

            Self::NonFiniteValue {
                context,
                index,
                value,
                message,
            } => {
                if let Some(index) = index {
                    write!(
                        f,
                        "non-finite value {value} in {context} at index \
                         {index}: {message}"
                    )
                } else {
                    write!(
                        f,
                        "non-finite value {value} in {context}: {message}"
                    )
                }
            }

            Self::NumericalInstability {
                operation,
                message,
            } => {
                write!(
                    f,
                    "numerical instability during '{operation}': {message}"
                )
            }

            Self::ConvergenceFailure {
                algorithm,
                iterations,
                message,
            } => {
                if let Some(iterations) = iterations {
                    write!(
                        f,
                        "algorithm '{algorithm}' failed to converge after \
                         {iterations} iterations: {message}"
                    )
                } else {
                    write!(
                        f,
                        "algorithm '{algorithm}' failed to converge: {message}"
                    )
                }
            }

            Self::OptimizationFailed {
                optimizer,
                message,
            } => {
                write!(
                    f,
                    "optimizer '{optimizer}' failed: {message}"
                )
            }

            Self::DeterminismViolation {
                contract,
                message,
            } => {
                write!(
                    f,
                    "determinism contract '{contract}' violated: {message}"
                )
            }

            Self::SerializationFailure {
                operation,
                message,
            } => {
                write!(
                    f,
                    "serialization operation '{operation}' failed: {message}"
                )
            }

            Self::ReplayFailure {
                operation,
                message,
            } => {
                write!(
                    f,
                    "replay operation '{operation}' failed: {message}"
                )
            }

            Self::VersionMismatch {
                component,
                expected,
                actual,
                message,
            } => {
                write!(
                    f,
                    "version mismatch for '{component}': expected \
                     {expected}, actual {actual}: {message}"
                )
            }

            Self::InternalInvariantViolation {
                invariant,
                message,
            } => {
                write!(
                    f,
                    "internal algorithm invariant '{invariant}' violated: \
                     {message}"
                )
            }
        }
    }
}

impl Error for AlgorithmError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_machine_readable_values() {
        let error = AlgorithmError::invalid_input("invalid test input");

        assert_eq!(error.kind(), AlgorithmErrorKind::InvalidInput);
        assert_eq!(error.code(), "invalid_input");
        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Input
        );
    }

    #[test]
    fn invalid_parameter_is_classified_as_input_error() {
        let error = AlgorithmError::invalid_parameter(
            Some(3),
            Some(f64::NAN),
            "parameter must be finite",
        );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::InvalidParameter
        );
        assert!(error.is_input_error());
        assert!(!error.is_retryable());
    }

    #[test]
    fn resource_limit_is_not_an_internal_failure() {
        let error = AlgorithmError::resource_limit_exceeded(
            AlgorithmResource::Qubits,
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
    fn backend_unavailability_is_retryable() {
        let error = AlgorithmError::backend_unavailable(
            "reference-simulator",
            "backend is temporarily unavailable",
        );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::BackendUnavailable
        );
        assert!(error.is_retryable());
        assert!(!error.is_input_error());
    }

    #[test]
    fn timeout_is_retryable_and_resource_classified() {
        let error = AlgorithmError::timeout(
            "execute",
            Some(1_000_000),
            "execution deadline exceeded",
        );

        assert_eq!(error.kind(), AlgorithmErrorKind::Timeout);
        assert!(error.is_retryable());
        assert!(error.is_resource_boundary());
        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Resource
        );
    }

    #[test]
    fn cancellation_is_not_a_retryable_failure_by_default() {
        let error = AlgorithmError::cancelled(
            "optimization",
            "caller requested cancellation",
        );

        assert_eq!(error.kind(), AlgorithmErrorKind::Cancelled);
        assert!(!error.is_retryable());
        assert!(error.is_resource_boundary());
    }

    #[test]
    fn internal_invariant_is_classified_as_internal() {
        let error = AlgorithmError::internal_invariant(
            "parameter_count_matches_ansatz",
            "internal state became inconsistent",
        );

        assert!(error.is_internal());
        assert_eq!(
            error.severity(),
            AlgorithmErrorSeverity::Internal
        );
        assert!(!error.is_retryable());
    }

    #[test]
    fn non_finite_values_have_dedicated_classification() {
        let error = AlgorithmError::non_finite_value(
            "objective",
            Some(0),
            f64::INFINITY,
            "objective must remain finite",
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
    fn dimension_mismatch_preserves_both_dimensions() {
        let error = AlgorithmError::dimension_mismatch(
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
    fn resource_names_are_stable() {
        assert_eq!(
            AlgorithmResource::ObjectiveEvaluations.as_str(),
            "objective_evaluations"
        );

        assert_eq!(
            AlgorithmResource::CircuitDepth.as_str(),
            "circuit_depth"
        );
    }

    #[test]
    fn error_kind_names_are_stable() {
        assert_eq!(
            AlgorithmErrorKind::ExecutionFailed.as_str(),
            "execution_failed"
        );

        assert_eq!(
            AlgorithmErrorKind::DeterminismViolation.as_str(),
            "determinism_violation"
        );
    }

    #[test]
    fn display_contains_context_without_being_required_for_classification() {
        let error = AlgorithmError::execution_failed(
            Some("test-backend".to_owned()),
            "expectation",
            "backend returned an invalid result",
        );

        assert_eq!(
            error.kind(),
            AlgorithmErrorKind::ExecutionFailed
        );

        let text = error.to_string();

        assert!(text.contains("test-backend"));
        assert!(text.contains("expectation"));
        assert!(text.contains("invalid result"));
    }
}