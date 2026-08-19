//! Unified error model for Zamani Quantum Error Correction.
//!
//! This module defines the canonical error boundary for the QEC subsystem.
//!
//! Design goals:
//! - No panic-based validation.
//! - Stable, machine-readable error classification.
//! - Human-readable diagnostic messages.
//! - Structured context without requiring allocation for every error.
//! - Explicit resource-limit failures.
//! - Explicit cancellation and timeout handling.
//! - Safe handling of malformed/untrusted input.
//! - Deterministic classification.
//! - Compatibility with `Result<T, QecError>` throughout the subsystem.
//!
//! Architectural rule:
//!
//! ```text
//! External input
//!      |
//!      v
//!   validation
//!      |
//!      v
//!   QecResult<T>
//!      |
//!      +--------------------+
//!      |                    |
//!   success               QecError
//!                              |
//!                 +------------+------------+
//!                 |            |            |
//!              recoverable  resource    invalid input
//! ```
//!
//! This file deliberately does not perform validation itself. Individual
//! modules (`validation`, `surface_code`, `stabilizer`, `syndrome`, etc.)
//! produce these errors at their respective boundaries.

use core::fmt;

/// Result type used throughout the QEC subsystem.
pub type QecResult<T> = Result<T, QecError>;

/// Canonical QEC subsystem error.
///
/// The variants are intentionally explicit so callers can distinguish:
///
/// - malformed input;
/// - invalid mathematical structures;
/// - resource exhaustion;
/// - cancellation;
/// - numerical failures;
/// - decoder failures;
/// - unsupported configurations;
/// - internal invariant violations.
///
/// Errors should be returned rather than triggering panics.
#[derive(Debug, Clone, PartialEq)]
pub enum QecError {
    /// Generic invalid external or user-provided input.
    InvalidInput {
        message: String,
    },

    /// Invalid surface-code or QEC topology.
    InvalidTopology {
        message: String,
    },

    /// Invalid stabilizer definition or stabilizer algebra.
    InvalidStabilizer {
        message: String,
    },

    /// Invalid syndrome or detection-event data.
    InvalidSyndrome {
        message: String,
    },

    /// Invalid decoding graph.
    InvalidGraph {
        message: String,
    },

    /// Probability is outside the mathematically valid domain.
    InvalidProbability {
        probability: f64,
        message: String,
    },

    /// A configured QEC resource limit has been exceeded.
    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        limit: u128,
        message: String,
    },

    /// A memory-specific resource limit has been exceeded.
    MemoryLimitExceeded {
        requested_bytes: u64,
        limit_bytes: u64,
        message: String,
    },

    /// A time/deadline limit has been exceeded.
    TimeLimitExceeded {
        elapsed_nanos: u64,
        limit_nanos: u64,
        message: String,
    },

    /// The operation was explicitly cancelled.
    CancellationRequested {
        message: String,
    },

    /// The decoder could not produce a valid decoding result.
    DecoderFailure {
        decoder: DecoderKind,
        message: String,
    },

    /// A numerical operation produced an unsafe or invalid value.
    NumericalFailure {
        operation: NumericalOperation,
        message: String,
    },

    /// The requested configuration or feature is not supported.
    UnsupportedConfiguration {
        feature: String,
        message: String,
    },

    /// An internal invariant was violated.
    ///
    /// This represents a programming/implementation defect rather than
    /// malformed user input. It must never be used as a substitute for
    /// normal validation.
    InternalInvariantViolation {
        invariant: String,
        message: String,
    },
}

/// Categories of resources that can be bounded by the QEC runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    CodeDistance,
    Qubits,
    Stabilizers,
    SyndromeEvents,
    MeasurementRounds,
    GraphNodes,
    GraphEdges,
    DecoderIterations,
    Parallelism,
    CheckpointSize,
    AllocationCount,
    Custom,
}

impl ResourceKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDistance => "code_distance",
            Self::Qubits => "qubits",
            Self::Stabilizers => "stabilizers",
            Self::SyndromeEvents => "syndrome_events",
            Self::MeasurementRounds => "measurement_rounds",
            Self::GraphNodes => "graph_nodes",
            Self::GraphEdges => "graph_edges",
            Self::DecoderIterations => "decoder_iterations",
            Self::Parallelism => "parallelism",
            Self::CheckpointSize => "checkpoint_size",
            Self::AllocationCount => "allocation_count",
            Self::Custom => "custom",
        }
    }
}

/// Decoder responsible for an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecoderKind {
    SurfaceCode,
    Mwpm,
    UnionFind,
    Custom,
}

impl DecoderKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::Mwpm => "mwpm",
            Self::UnionFind => "union_find",
            Self::Custom => "custom",
        }
    }
}

/// Numerical operation associated with a numerical failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericalOperation {
    ProbabilityValidation,
    LogProbability,
    WeightCalculation,
    DistanceCalculation,
    CoordinateCalculation,
    IntegerConversion,
    FloatingPointConversion,
    Accumulation,
    Normalization,
    MatchingWeight,
    Custom,
}

impl NumericalOperation {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProbabilityValidation => "probability_validation",
            Self::LogProbability => "log_probability",
            Self::WeightCalculation => "weight_calculation",
            Self::DistanceCalculation => "distance_calculation",
            Self::CoordinateCalculation => "coordinate_calculation",
            Self::IntegerConversion => "integer_conversion",
            Self::FloatingPointConversion => "floating_point_conversion",
            Self::Accumulation => "accumulation",
            Self::Normalization => "normalization",
            Self::MatchingWeight => "matching_weight",
            Self::Custom => "custom",
        }
    }
}

/// Stable high-level error classification.
///
/// This is preferable to matching on diagnostic strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QecErrorKind {
    InvalidInput,
    InvalidTopology,
    InvalidStabilizer,
    InvalidSyndrome,
    InvalidGraph,
    InvalidProbability,
    ResourceLimitExceeded,
    MemoryLimitExceeded,
    TimeLimitExceeded,
    CancellationRequested,
    DecoderFailure,
    NumericalFailure,
    UnsupportedConfiguration,
    InternalInvariantViolation,
}

impl QecError {
    // -----------------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------------

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    pub fn invalid_topology(message: impl Into<String>) -> Self {
        Self::InvalidTopology {
            message: message.into(),
        }
    }

    pub fn invalid_stabilizer(message: impl Into<String>) -> Self {
        Self::InvalidStabilizer {
            message: message.into(),
        }
    }

    pub fn invalid_syndrome(message: impl Into<String>) -> Self {
        Self::InvalidSyndrome {
            message: message.into(),
        }
    }

    pub fn invalid_graph(message: impl Into<String>) -> Self {
        Self::InvalidGraph {
            message: message.into(),
        }
    }

    pub fn invalid_probability(probability: f64, message: impl Into<String>) -> Self {
        Self::InvalidProbability {
            probability,
            message: message.into(),
        }
    }

    pub fn resource_limit(
        resource: ResourceKind,
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

    pub fn memory_limit(
        requested_bytes: u64,
        limit_bytes: u64,
        message: impl Into<String>,
    ) -> Self {
        Self::MemoryLimitExceeded {
            requested_bytes,
            limit_bytes,
            message: message.into(),
        }
    }

    pub fn time_limit(
        elapsed_nanos: u64,
        limit_nanos: u64,
        message: impl Into<String>,
    ) -> Self {
        Self::TimeLimitExceeded {
            elapsed_nanos,
            limit_nanos,
            message: message.into(),
        }
    }

    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::CancellationRequested {
            message: message.into(),
        }
    }

    pub fn decoder_failure(
        decoder: DecoderKind,
        message: impl Into<String>,
    ) -> Self {
        Self::DecoderFailure {
            decoder,
            message: message.into(),
        }
    }

    pub fn numerical_failure(
        operation: NumericalOperation,
        message: impl Into<String>,
    ) -> Self {
        Self::NumericalFailure {
            operation,
            message: message.into(),
        }
    }

    pub fn unsupported(
        feature: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::UnsupportedConfiguration {
            feature: feature.into(),
            message: message.into(),
        }
    }

    pub fn invariant(
        invariant: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InternalInvariantViolation {
            invariant: invariant.into(),
            message: message.into(),
        }
    }

    // -----------------------------------------------------------------------
    // Classification
    // -----------------------------------------------------------------------

    /// Returns the stable error category.
    pub const fn kind(&self) -> QecErrorKind {
        match self {
            Self::InvalidInput { .. } => QecErrorKind::InvalidInput,
            Self::InvalidTopology { .. } => QecErrorKind::InvalidTopology,
            Self::InvalidStabilizer { .. } => QecErrorKind::InvalidStabilizer,
            Self::InvalidSyndrome { .. } => QecErrorKind::InvalidSyndrome,
            Self::InvalidGraph { .. } => QecErrorKind::InvalidGraph,
            Self::InvalidProbability { .. } => QecErrorKind::InvalidProbability,
            Self::ResourceLimitExceeded { .. } => QecErrorKind::ResourceLimitExceeded,
            Self::MemoryLimitExceeded { .. } => QecErrorKind::MemoryLimitExceeded,
            Self::TimeLimitExceeded { .. } => QecErrorKind::TimeLimitExceeded,
            Self::CancellationRequested { .. } => {
                QecErrorKind::CancellationRequested
            }
            Self::DecoderFailure { .. } => QecErrorKind::DecoderFailure,
            Self::NumericalFailure { .. } => QecErrorKind::NumericalFailure,
            Self::UnsupportedConfiguration { .. } => {
                QecErrorKind::UnsupportedConfiguration
            }
            Self::InternalInvariantViolation { .. } => {
                QecErrorKind::InternalInvariantViolation
            }
        }
    }

    /// Stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidInput { .. } => "QEC-INPUT-001",
            Self::InvalidTopology { .. } => "QEC-TOPOLOGY-001",
            Self::InvalidStabilizer { .. } => "QEC-STABILIZER-001",
            Self::InvalidSyndrome { .. } => "QEC-SYNDROME-001",
            Self::InvalidGraph { .. } => "QEC-GRAPH-001",
            Self::InvalidProbability { .. } => "QEC-PROBABILITY-001",
            Self::ResourceLimitExceeded { .. } => "QEC-RESOURCE-001",
            Self::MemoryLimitExceeded { .. } => "QEC-MEMORY-001",
            Self::TimeLimitExceeded { .. } => "QEC-TIME-001",
            Self::CancellationRequested { .. } => "QEC-CANCEL-001",
            Self::DecoderFailure { .. } => "QEC-DECODER-001",
            Self::NumericalFailure { .. } => "QEC-NUMERICAL-001",
            Self::UnsupportedConfiguration { .. } => "QEC-CONFIG-001",
            Self::InternalInvariantViolation { .. } => "QEC-INTERNAL-001",
        }
    }

    /// Whether retrying the same operation without changing its inputs or
    /// configuration is likely to succeed.
    ///
    /// This is deliberately conservative.
    pub const fn is_retryable(&self) -> bool {
        match self {
            Self::InvalidInput { .. }
            | Self::InvalidTopology { .. }
            | Self::InvalidStabilizer { .. }
            | Self::InvalidSyndrome { .. }
            | Self::InvalidGraph { .. }
            | Self::InvalidProbability { .. }
            | Self::UnsupportedConfiguration { .. }
            | Self::InternalInvariantViolation { .. } => false,

            Self::ResourceLimitExceeded { .. }
            | Self::MemoryLimitExceeded { .. }
            | Self::TimeLimitExceeded { .. }
            | Self::CancellationRequested { .. }
            | Self::DecoderFailure { .. }
            | Self::NumericalFailure { .. } => false,
        }
    }

    /// Whether the error is caused by caller-controlled input.
    pub const fn is_input_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidInput { .. }
                | Self::InvalidTopology { .. }
                | Self::InvalidStabilizer { .. }
                | Self::InvalidSyndrome { .. }
                | Self::InvalidGraph { .. }
                | Self::InvalidProbability { .. }
        )
    }

    /// Whether the failure indicates a configured resource boundary.
    pub const fn is_resource_error(&self) -> bool {
        matches!(
            self,
            Self::ResourceLimitExceeded { .. }
                | Self::MemoryLimitExceeded { .. }
                | Self::TimeLimitExceeded { .. }
        )
    }

    /// Whether this error represents cancellation rather than failure.
    pub const fn is_cancellation(&self) -> bool {
        matches!(self, Self::CancellationRequested { .. })
    }

    /// Whether this is an implementation defect rather than invalid input.
    pub const fn is_internal(&self) -> bool {
        matches!(self, Self::InternalInvariantViolation { .. })
    }

    /// Returns the main human-readable diagnostic.
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidInput { message }
            | Self::InvalidTopology { message }
            | Self::InvalidStabilizer { message }
            | Self::InvalidSyndrome { message }
            | Self::InvalidGraph { message }
            | Self::CancellationRequested { message } => message,

            Self::InvalidProbability { message, .. }
            | Self::ResourceLimitExceeded { message, .. }
            | Self::MemoryLimitExceeded { message, .. }
            | Self::TimeLimitExceeded { message, .. }
            | Self::DecoderFailure { message, .. }
            | Self::NumericalFailure { message, .. }
            | Self::UnsupportedConfiguration { message, .. }
            | Self::InternalInvariantViolation { message, .. } => message,
        }
    }
}

impl fmt::Display for QecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}] {}", self.code(), self.message())
    }
}

impl std::error::Error for QecError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(
            QecError::invalid_input("bad input").code(),
            "QEC-INPUT-001"
        );

        assert_eq!(
            QecError::invalid_topology("bad topology").code(),
            "QEC-TOPOLOGY-001"
        );

        assert_eq!(
            QecError::invalid_stabilizer("bad stabilizer").code(),
            "QEC-STABILIZER-001"
        );

        assert_eq!(
            QecError::invalid_syndrome("bad syndrome").code(),
            "QEC-SYNDROME-001"
        );

        assert_eq!(
            QecError::invalid_graph("bad graph").code(),
            "QEC-GRAPH-001"
        );
    }

    #[test]
    fn input_errors_are_classified_correctly() {
        assert!(QecError::invalid_input("x").is_input_error());
        assert!(QecError::invalid_topology("x").is_input_error());
        assert!(QecError::invalid_stabilizer("x").is_input_error());
        assert!(QecError::invalid_syndrome("x").is_input_error());
        assert!(QecError::invalid_graph("x").is_input_error());

        assert!(!QecError::cancelled("x").is_input_error());
    }

    #[test]
    fn resource_errors_are_classified_correctly() {
        assert!(
            QecError::resource_limit(
                ResourceKind::Qubits,
                101,
                100,
                "too many qubits",
            )
            .is_resource_error()
        );

        assert!(
            QecError::memory_limit(
                2048,
                1024,
                "memory limit exceeded",
            )
            .is_resource_error()
        );

        assert!(
            QecError::time_limit(
                2_000,
                1_000,
                "deadline exceeded",
            )
            .is_resource_error()
        );
    }

    #[test]
    fn cancellation_is_not_an_input_error() {
        let error = QecError::cancelled("operation cancelled");

        assert!(error.is_cancellation());
        assert!(!error.is_input_error());
        assert!(!error.is_internal());
    }

    #[test]
    fn invariant_violation_is_internal() {
        let error = QecError::invariant(
            "stabilizer_commutation",
            "validated stabilizers do not commute",
        );

        assert!(error.is_internal());
        assert!(!error.is_input_error());
        assert!(!error.is_retryable());
        assert_eq!(error.code(), "QEC-INTERNAL-001");
    }

    #[test]
    fn display_contains_stable_code() {
        let error = QecError::invalid_probability(
            1.5,
            "probability must be in [0, 1]",
        );

        let rendered = error.to_string();

        assert!(rendered.contains("QEC-PROBABILITY-001"));
        assert!(rendered.contains("probability must be in [0, 1]"));
    }

    #[test]
    fn resource_kinds_have_stable_names() {
        assert_eq!(ResourceKind::Qubits.as_str(), "qubits");
        assert_eq!(
            ResourceKind::GraphNodes.as_str(),
            "graph_nodes"
        );
        assert_eq!(
            ResourceKind::MeasurementRounds.as_str(),
            "measurement_rounds"
        );
    }

    #[test]
    fn decoder_kinds_have_stable_names() {
        assert_eq!(DecoderKind::Mwpm.as_str(), "mwpm");
        assert_eq!(DecoderKind::UnionFind.as_str(), "union_find");
    }

    #[test]
    fn numerical_operations_have_stable_names() {
        assert_eq!(
            NumericalOperation::LogProbability.as_str(),
            "log_probability"
        );

        assert_eq!(
            NumericalOperation::WeightCalculation.as_str(),
            "weight_calculation"
        );
    }

    #[test]
    fn result_alias_works() {
        fn operation() -> QecResult<u32> {
            Ok(42)
        }

        assert_eq!(operation().expect("test operation should succeed"), 42);
    }
}