//! Unified error model for Zamani Quantum Error Correction.
//!
//! This module is the canonical public error boundary for the QEC subsystem.
//!
//! Architectural rule:
//!
//! ```text
//!                    UNTRUSTED INPUT
//!                          │
//!                          ▼
//!                    VALIDATION
//!                          │
//!                          ▼
//!                 MODULE-SPECIFIC ERROR
//!                          │
//!                          ▼
//!                      QecError
//!                          │
//!          ┌───────────────┼────────────────┐
//!          ▼               ▼                ▼
//!       caller          telemetry        recovery
//! ```
//!
//! Individual implementation modules may retain specialized internal error
//! types when that improves local diagnostics. Public/high-level APIs should
//! convert those errors into `QecError`.
//!
//! Design goals:
//!
//! - No panic-based validation.
//! - Stable machine-readable error classification.
//! - Human-readable diagnostics.
//! - Explicit resource-limit failures.
//! - Explicit memory and time failures.
//! - Explicit cancellation.
//! - Explicit decoder failures.
//! - Explicit numerical failures.
//! - Safe handling of malformed/untrusted input.
//! - Deterministic classification.
//! - Compatibility with `Result<T, QecError>`.
//! - No dependence on diagnostic strings for programmatic recovery.
//!
//! `QecError` intentionally does not own the resource policy itself.
//! Resource policy belongs to the limits/resource-management layers.
//! This module only provides the canonical error representation.

use core::fmt;
use std::error::Error;
use std::time::Duration;

// ============================================================================
// Result
// ============================================================================

/// Canonical result type for public QEC APIs.
pub type QecResult<T> = Result<T, QecError>;

// ============================================================================
// QecError
// ============================================================================

/// Canonical error returned by the Quantum Error Correction subsystem.
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

    /// A configured QEC resource limit was exceeded.
    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        current: u128,
        limit: u128,
        message: String,
    },

    /// A memory-specific resource limit was exceeded.
    MemoryLimitExceeded {
        requested_bytes: u64,
        current_bytes: u64,
        limit_bytes: u64,
        message: String,
    },

    /// A time/deadline limit was exceeded.
    TimeLimitExceeded {
        elapsed_nanos: u64,
        limit_nanos: u64,
        message: String,
    },

    /// The operation was explicitly cancelled.
    CancellationRequested {
        message: String,
    },

    /// A decoder failed to produce a valid result.
    DecoderFailure {
        decoder: DecoderKind,
        message: String,
    },

    /// A numerical operation produced an unsafe or invalid result.
    NumericalFailure {
        operation: NumericalOperation,
        message: String,
    },

    /// The requested configuration or capability is unsupported.
    UnsupportedConfiguration {
        feature: String,
        message: String,
    },

    /// An internal implementation invariant was violated.
    ///
    /// This represents a programming defect rather than malformed input.
    /// It must not be used as a substitute for normal validation.
    InternalInvariantViolation {
        invariant: String,
        message: String,
    },
}

// ============================================================================
// ResourceKind
// ============================================================================

/// Canonical high-level resource dimensions understood by the QEC policy.
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
    MemoryBytes,
    QpuShots,
    QpuCircuits,
    AllocationCount,
    Partitions,
    StreamBuffer,
    Custom,
}

impl ResourceKind {
    /// Stable machine-readable resource identifier.
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
            Self::MemoryBytes => "memory_bytes",
            Self::QpuShots => "qpu_shots",
            Self::QpuCircuits => "qpu_circuits",
            Self::AllocationCount => "allocation_count",
            Self::Partitions => "partitions",
            Self::StreamBuffer => "stream_buffer",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// DecoderKind
// ============================================================================

/// Decoder responsible for a decoding failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecoderKind {
    SurfaceCode,
    Mwpm,
    UnionFind,
    BeliefPropagation,
    TensorNetwork,
    LookupTable,
    Streaming,
    Distributed,
    Custom,
}

impl DecoderKind {
    /// Stable machine-readable decoder identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::Mwpm => "mwpm",
            Self::UnionFind => "union_find",
            Self::BeliefPropagation => "belief_propagation",
            Self::TensorNetwork => "tensor_network",
            Self::LookupTable => "lookup_table",
            Self::Streaming => "streaming",
            Self::Distributed => "distributed",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for DecoderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// NumericalOperation
// ============================================================================

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
    MatrixOperation,
    StabilizerAlgebra,
    SyndromeCalculation,
    StatisticalEstimate,
    Custom,
}

impl NumericalOperation {
    /// Stable machine-readable operation identifier.
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
            Self::MatrixOperation => "matrix_operation",
            Self::StabilizerAlgebra => "stabilizer_algebra",
            Self::SyndromeCalculation => "syndrome_calculation",
            Self::StatisticalEstimate => "statistical_estimate",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for NumericalOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Error classification
// ============================================================================

/// Stable high-level error category.
///
/// Callers should use this instead of matching human-readable strings.
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

impl QecErrorKind {
    /// Stable machine-readable category.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidTopology => "invalid_topology",
            Self::InvalidStabilizer => "invalid_stabilizer",
            Self::InvalidSyndrome => "invalid_syndrome",
            Self::InvalidGraph => "invalid_graph",
            Self::InvalidProbability => "invalid_probability",
            Self::ResourceLimitExceeded => "resource_limit_exceeded",
            Self::MemoryLimitExceeded => "memory_limit_exceeded",
            Self::TimeLimitExceeded => "time_limit_exceeded",
            Self::CancellationRequested => "cancellation_requested",
            Self::DecoderFailure => "decoder_failure",
            Self::NumericalFailure => "numerical_failure",
            Self::UnsupportedConfiguration => "unsupported_configuration",
            Self::InternalInvariantViolation => "internal_invariant_violation",
        }
    }
}

impl fmt::Display for QecErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Error severity
// ============================================================================

/// Operational severity of a QEC failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QecErrorSeverity {
    /// The caller supplied invalid data.
    Input,

    /// The configured workload exceeded a deliberate safety boundary.
    Resource,

    /// The operation was deliberately stopped.
    Cancellation,

    /// The requested operation could not be completed correctly.
    Operational,

    /// The requested feature/configuration is unsupported.
    Configuration,

    /// Indicates a likely software defect.
    Internal,
}

impl QecErrorSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Resource => "resource",
            Self::Cancellation => "cancellation",
            Self::Operational => "operational",
            Self::Configuration => "configuration",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for QecErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Constructors and classification
// ============================================================================

impl QecError {
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

    pub fn invalid_probability(
        probability: f64,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidProbability {
            probability,
            message: message.into(),
        }
    }

    pub fn resource_limit(
        resource: ResourceKind,
        requested: u128,
        current: u128,
        limit: u128,
        message: impl Into<String>,
    ) -> Self {
        Self::ResourceLimitExceeded {
            resource,
            requested,
            current,
            limit,
            message: message.into(),
        }
    }

    pub fn memory_limit(
        requested_bytes: u64,
        current_bytes: u64,
        limit_bytes: u64,
        message: impl Into<String>,
    ) -> Self {
        Self::MemoryLimitExceeded {
            requested_bytes,
            current_bytes,
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

    /// Stable high-level classification.
    pub const fn kind(&self) -> QecErrorKind {
        match self {
            Self::InvalidInput { .. } => QecErrorKind::InvalidInput,
            Self::InvalidTopology { .. } => QecErrorKind::InvalidTopology,
            Self::InvalidStabilizer { .. } => QecErrorKind::InvalidStabilizer,
            Self::InvalidSyndrome { .. } => QecErrorKind::InvalidSyndrome,
            Self::InvalidGraph { .. } => QecErrorKind::InvalidGraph,
            Self::InvalidProbability { .. } => QecErrorKind::InvalidProbability,
            Self::ResourceLimitExceeded { .. } => {
                QecErrorKind::ResourceLimitExceeded
            }
            Self::MemoryLimitExceeded { .. } => {
                QecErrorKind::MemoryLimitExceeded
            }
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

    /// Returns the canonical diagnostic message.
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidInput { message }
            | Self::InvalidTopology { message }
            | Self::InvalidStabilizer { message }
            | Self::InvalidSyndrome { message }
            | Self::InvalidGraph { message }
            | Self::CancellationRequested { message }
            | Self::InvalidProbability { message, .. }
            | Self::ResourceLimitExceeded { message, .. }
            | Self::MemoryLimitExceeded { message, .. }
            | Self::TimeLimitExceeded { message, .. }
            | Self::DecoderFailure { message, .. }
            | Self::NumericalFailure { message, .. }
            | Self::UnsupportedConfiguration { message, .. }
            | Self::InternalInvariantViolation { message, .. } => message,
        }
    }

    /// Returns the operational severity.
    pub const fn severity(&self) -> QecErrorSeverity {
        match self {
            Self::InvalidInput { .. }
            | Self::InvalidTopology { .. }
            | Self::InvalidStabilizer { .. }
            | Self::InvalidSyndrome { .. }
            | Self::InvalidGraph { .. }
            | Self::InvalidProbability { .. } => QecErrorSeverity::Input,

            Self::ResourceLimitExceeded { .. }
            | Self::MemoryLimitExceeded { .. }
            | Self::TimeLimitExceeded { .. } => QecErrorSeverity::Resource,

            Self::CancellationRequested { .. } => QecErrorSeverity::Cancellation,

            Self::DecoderFailure { .. }
            | Self::NumericalFailure { .. } => QecErrorSeverity::Operational,

            Self::UnsupportedConfiguration { .. } => {
                QecErrorSeverity::Configuration
            }

            Self::InternalInvariantViolation { .. } => QecErrorSeverity::Internal,
        }
    }

    /// Returns whether the error originated from caller-controlled input.
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

    /// Returns whether the error represents resource exhaustion.
    pub const fn is_resource_error(&self) -> bool {
        matches!(
            self,
            Self::ResourceLimitExceeded { .. }
                | Self::MemoryLimitExceeded { .. }
                | Self::TimeLimitExceeded { .. }
        )
    }

    /// Returns whether the operation was cancelled.
    pub const fn is_cancellation(&self) -> bool {
        matches!(self, Self::CancellationRequested { .. })
    }

    /// Returns whether the error represents an implementation defect.
    pub const fn is_internal(&self) -> bool {
        matches!(self, Self::InternalInvariantViolation { .. })
    }

    /// Returns the affected resource if this is a resource error.
    pub const fn resource(&self) -> Option<ResourceKind> {
        match self {
            Self::ResourceLimitExceeded { resource, .. } => Some(*resource),

            Self::MemoryLimitExceeded { .. } => {
                Some(ResourceKind::MemoryBytes)
            }

            Self::TimeLimitExceeded { .. } => None,

            _ => None,
        }
    }

    /// Returns the decoder associated with a decoder failure.
    pub const fn decoder(&self) -> Option<DecoderKind> {
        match self {
            Self::DecoderFailure { decoder, .. } => Some(*decoder),
            _ => None,
        }
    }

    /// Returns the numerical operation associated with a numerical failure.
    pub const fn numerical_operation(
        &self,
    ) -> Option<NumericalOperation> {
        match self {
            Self::NumericalFailure { operation, .. } => Some(*operation),
            _ => None,
        }
    }

    /// Conservative retry policy.
    ///
    /// A retry without changing configuration/input is only considered safe
    /// when the error represents a transient execution condition. Since the
    /// current QEC runtime does not encode transient-vs-permanent resource
    /// exhaustion separately, resource failures are conservatively marked
    /// non-retryable.
    pub const fn is_retryable(&self) -> bool {
        match self {
            Self::InvalidInput { .. }
            | Self::InvalidTopology { .. }
            | Self::InvalidStabilizer { .. }
            | Self::InvalidSyndrome { .. }
            | Self::InvalidGraph { .. }
            | Self::InvalidProbability { .. }
            | Self::ResourceLimitExceeded { .. }
            | Self::MemoryLimitExceeded { .. }
            | Self::TimeLimitExceeded { .. }
            | Self::CancellationRequested { .. }
            | Self::UnsupportedConfiguration { .. }
            | Self::InternalInvariantViolation { .. } => false,

            // Decoder and numerical failures may become recoverable when
            // the caller selects a different decoder/backend/strategy.
            Self::DecoderFailure { .. }
            | Self::NumericalFailure { .. } => false,
        }
    }
}

// ============================================================================
// Display / std::error::Error
// ============================================================================

impl fmt::Display for QecError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}",
            self.code(),
            self.message()
        )
    }
}

impl Error for QecError {}

// ============================================================================
// Primitive conversions
// ============================================================================

impl From<std::num::TryFromIntError> for QecError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::numerical_failure(
            NumericalOperation::IntegerConversion,
            error.to_string(),
        )
    }
}

impl From<std::num::ParseIntError> for QecError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::invalid_input(format!(
            "integer parsing failed: {error}"
        ))
    }
}

impl From<std::num::ParseFloatError> for QecError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::invalid_input(format!(
            "floating-point parsing failed: {error}"
        ))
    }
}

impl From<std::io::Error> for QecError {
    fn from(error: std::io::Error) -> Self {
        Self::invalid_input(format!("I/O operation failed: {error}"))
    }
}

// ============================================================================
// Resource-layer integration
// ============================================================================

/// Convert the resource manager's resource dimension into the canonical
/// high-level QEC resource dimension.
///
/// This keeps `resources.rs` independent of `errors.rs` while allowing public
/// APIs to expose only `QecError`.
impl From<crate::quantum::error_correction::resources::ResourceKind>
    for ResourceKind
{
    fn from(
        resource: crate::quantum::error_correction::resources::ResourceKind,
    ) -> Self {
        use crate::quantum::error_correction::resources::ResourceKind as R;

        match resource {
            R::MemoryBytes => Self::MemoryBytes,
            R::SyndromeEvents => Self::SyndromeEvents,
            R::GraphNodes => Self::GraphNodes,
            R::GraphEdges => Self::GraphEdges,
            R::DecoderIterations => Self::DecoderIterations,
            R::ParallelWorkers => Self::Parallelism,
        }
    }
}

/// Converts runtime resource-manager failures into the canonical QEC error.
impl From<
    crate::quantum::error_correction::resources::ResourceError,
> for QecError {
    fn from(
        error: crate::quantum::error_correction::resources::ResourceError,
    ) -> Self {
        use crate::quantum::error_correction::resources::ResourceError as R;

        match error {
            R::InvalidLimit { reason } => {
                Self::invalid_input(format!(
                    "invalid resource policy: {reason}"
                ))
            }

            R::LimitExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                let resource = ResourceKind::from(resource);

                if resource == ResourceKind::MemoryBytes {
                    Self::memory_limit(
                        requested,
                        current,
                        limit,
                        format!(
                            "memory resource limit exceeded: \
                             requested {requested} bytes, \
                             current {current} bytes, \
                             limit {limit} bytes"
                        ),
                    )
                } else {
                    Self::resource_limit(
                        resource,
                        u128::from(requested),
                        u128::from(current),
                        u128::from(limit),
                        format!(
                            "{resource} resource limit exceeded: \
                             requested {requested}, \
                             current {current}, \
                             limit {limit}"
                        ),
                    )
                }
            }

            R::ParallelismLimitExceeded {
                requested,
                current,
                limit,
            } => Self::resource_limit(
                ResourceKind::Parallelism,
                requested as u128,
                current as u128,
                limit as u128,
                format!(
                    "parallelism limit exceeded: \
                     requested {requested}, \
                     current {current}, \
                     limit {limit}"
                ),
            ),

            R::QuotaExceeded {
                resource,
                requested,
                current,
                limit,
            } => {
                let resource = ResourceKind::from(resource);

                Self::resource_limit(
                    resource,
                    u128::from(requested),
                    u128::from(current),
                    u128::from(limit),
                    format!(
                        "{resource} operation quota exceeded: \
                         requested {requested}, \
                         current {current}, \
                         quota {limit}"
                    ),
                )
            }

            R::ParallelismQuotaExceeded {
                requested,
                current,
                limit,
            } => Self::resource_limit(
                ResourceKind::Parallelism,
                requested as u128,
                current as u128,
                limit as u128,
                format!(
                    "parallelism operation quota exceeded: \
                     requested {requested}, \
                     current {current}, \
                     quota {limit}"
                ),
            ),

            R::ArithmeticOverflow { resource } => {
                let resource = ResourceKind::from(resource);

                Self::numerical_failure(
                    NumericalOperation::IntegerConversion,
                    format!(
                        "resource accounting overflow for {resource}"
                    ),
                )
            }

            R::WallTimeLimitExceeded { elapsed, limit } => {
                Self::time_limit(
                    duration_to_nanos_saturating(elapsed),
                    duration_to_nanos_saturating(limit),
                    format!(
                        "QEC wall-time limit exceeded: \
                         elapsed {:?}, limit {:?}",
                        elapsed, limit
                    ),
                )
            }

            R::Cancelled => Self::cancelled(
                "QEC resource manager reported cancellation",
            ),
        }
    }
}

// ============================================================================
// Optional high-level conversions
// ============================================================================

/// Convert a `Duration` to nanoseconds without overflowing `u64`.
fn duration_to_nanos_saturating(duration: Duration) -> u64 {
    duration
        .as_nanos()
        .try_into()
        .unwrap_or(u64::MAX)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_works() {
        let result: QecResult<u32> = Ok(42);
        assert_eq!(result.unwrap(), 42);
    }

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
            QecError::resource_limit(
                ResourceKind::GraphNodes,
                101,
                100,
                100,
                "too many nodes",
            )
            .code(),
            "QEC-RESOURCE-001"
        );
    }

    #[test]
    fn error_kind_is_stable() {
        let error = QecError::invalid_syndrome("invalid syndrome");

        assert_eq!(
            error.kind(),
            QecErrorKind::InvalidSyndrome
        );

        assert_eq!(
            error.kind().as_str(),
            "invalid_syndrome"
        );
    }

    #[test]
    fn severity_is_classified_correctly() {
        assert_eq!(
            QecError::invalid_input("bad").severity(),
            QecErrorSeverity::Input
        );

        assert_eq!(
            QecError::memory_limit(100, 90, 90, "memory").severity(),
            QecErrorSeverity::Resource
        );

        assert_eq!(
            QecError::cancelled("cancelled").severity(),
            QecErrorSeverity::Cancellation
        );

        assert_eq!(
            QecError::invariant("broken", "internal").severity(),
            QecErrorSeverity::Internal
        );
    }

    #[test]
    fn resource_classification_works() {
        let error = QecError::resource_limit(
            ResourceKind::GraphEdges,
            101,
            100,
            100,
            "too many edges",
        );

        assert!(error.is_resource_error());
        assert!(!error.is_input_error());
        assert_eq!(
            error.resource(),
            Some(ResourceKind::GraphEdges)
        );
    }

    #[test]
    fn memory_is_a_resource_error() {
        let error =
            QecError::memory_limit(1024, 2048, 2048, "memory exhausted");

        assert!(error.is_resource_error());
        assert_eq!(
            error.resource(),
            Some(ResourceKind::MemoryBytes)
        );
    }

    #[test]
    fn cancellation_is_not_a_generic_failure() {
        let error = QecError::cancelled("user requested cancellation");

        assert!(error.is_cancellation());
        assert_eq!(
            error.kind(),
            QecErrorKind::CancellationRequested
        );
        assert_eq!(
            error.severity(),
            QecErrorSeverity::Cancellation
        );
    }

    #[test]
    fn decoder_metadata_is_available() {
        let error = QecError::decoder_failure(
            DecoderKind::Mwpm,
            "matching failed",
        );

        assert_eq!(error.decoder(), Some(DecoderKind::Mwpm));
        assert_eq!(error.kind(), QecErrorKind::DecoderFailure);
        assert_eq!(error.code(), "QEC-DECODER-001");
    }

    #[test]
    fn numerical_metadata_is_available() {
        let error = QecError::numerical_failure(
            NumericalOperation::DistanceCalculation,
            "distance calculation exceeded numeric range",
        );

        assert_eq!(
            error.numerical_operation(),
            Some(NumericalOperation::DistanceCalculation)
        );

        assert_eq!(
            error.kind(),
            QecErrorKind::NumericalFailure
        );
    }

    #[test]
    fn display_contains_machine_code() {
        let error = QecError::invalid_input("invalid qubit index");

        let rendered = error.to_string();

        assert!(rendered.contains("QEC-INPUT-001"));
        assert!(rendered.contains("invalid qubit index"));
    }

    #[test]
    fn integer_conversion_becomes_numerical_error() {
        let result: Result<u8, _> = u8::try_from(1000u16);

        let error = result.unwrap_err();
        let qec_error: QecError = error.into();

        assert_eq!(
            qec_error.kind(),
            QecErrorKind::NumericalFailure
        );

        assert_eq!(
            qec_error.numerical_operation(),
            Some(NumericalOperation::IntegerConversion)
        );
    }

    #[test]
    fn duration_conversion_saturates() {
        let duration = Duration::from_secs(u64::MAX);

        assert_eq!(
            duration_to_nanos_saturating(duration),
            u64::MAX
        );
    }

    #[test]
    fn resource_kind_identifiers_are_stable() {
        assert_eq!(
            ResourceKind::GraphNodes.as_str(),
            "graph_nodes"
        );

        assert_eq!(
            ResourceKind::MemoryBytes.as_str(),
            "memory_bytes"
        );

        assert_eq!(
            ResourceKind::QpuShots.as_str(),
            "qpu_shots"
        );
    }

    #[test]
    fn decoder_kind_identifiers_are_stable() {
        assert_eq!(
            DecoderKind::Mwpm.as_str(),
            "mwpm"
        );

        assert_eq!(
            DecoderKind::UnionFind.as_str(),
            "union_find"
        );
    }

    #[test]
    fn numerical_operation_identifiers_are_stable() {
        assert_eq!(
            NumericalOperation::DistanceCalculation.as_str(),
            "distance_calculation"
        );

        assert_eq!(
            NumericalOperation::StatisticalEstimate.as_str(),
            "statistical_estimate"
        );
    }
}