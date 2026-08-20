//! Unified error model for Zamani Quantum Error Correction.
//!
//! This module is the canonical public error boundary for the QEC subsystem.
//!
//! # Architectural contract
//!
//! ```text
//! untrusted input
//!       │
//!       ▼
//!   validation
//!       │
//!       ▼
//! module-specific error
//!       │
//!       ▼
//!    QecError
//!       │
//!   ┌───┼──────────────┐
//!   ▼   ▼              ▼
//! caller telemetry   recovery
//! ```
//!
//! `errors.rs` owns error representation and classification.
//! It does not own resource policy, allocation, cancellation state,
//! authorization, configuration, telemetry transport, or decoder policy.
//!
//! # Design goals
//!
//! - deterministic machine-readable classification;
//! - stable error codes;
//! - human-readable diagnostics;
//! - explicit resource exhaustion;
//! - explicit memory exhaustion;
//! - explicit time/deadline exhaustion;
//! - explicit cancellation;
//! - explicit capability denial;
//! - explicit version incompatibility;
//! - explicit checkpoint/cache failures;
//! - explicit backend/QPU failures;
//! - explicit numerical failures;
//! - safe conversion of lower-level errors;
//! - no panic-based validation;
//! - no dependency on diagnostic strings for recovery logic;
//! - compatibility with Rust 1.97.1.
//!
//! # Integration
//!
//! Foundation modules may remain independent from this module when required
//! to avoid dependency cycles. In particular:
//!
//! ```text
//! arithmetic.rs
//!      │
//!      └── ArithmeticError
//!              │
//!              ▼
//!       higher-level boundary
//!              │
//!              ▼
//!          QecError
//! ```
//!
//! `limits.rs`, `memory.rs`, `resources.rs`, `cancellation.rs`,
//! `validation.rs`, decoders, checkpointing, caching, distributed execution,
//! and QPU execution may use `QecError` directly.
//!
//! # Rust compatibility
//!
//! This file intentionally uses only stable standard-library facilities
//! available to the repository's pinned Rust 1.97.1 toolchain.

use core::fmt;
use std::error::Error;
use std::time::Duration;

// ============================================================================
// Canonical result
// ============================================================================

/// Canonical result type for public QEC APIs.
pub type QecResult<T> = Result<T, QecError>;

// ============================================================================
// Canonical error
// ============================================================================

/// Canonical error returned by the Quantum Error Correction subsystem.
///
/// Variants are intentionally explicit. Callers should use [`QecError::kind`]
/// or [`QecError::code`] for programmatic handling rather than parsing
/// diagnostic strings.
#[derive(Debug, Clone, PartialEq)]
pub enum QecError {
    /// Generic malformed or invalid caller input.
    InvalidInput {
        message: String,
    },

    /// Invalid surface-code or QEC topology.
    InvalidTopology {
        message: String,
    },

    /// Invalid stabilizer definition or algebraic state.
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

    /// Probability outside the valid mathematical domain.
    InvalidProbability {
        probability: f64,
        message: String,
    },

    /// A configured non-memory resource limit was exceeded.
    ResourceLimitExceeded {
        resource: ResourceKind,
        requested: u128,
        current: u128,
        limit: u128,
        message: String,
    },

    /// A memory-specific limit was exceeded.
    MemoryLimitExceeded {
        requested_bytes: u64,
        current_bytes: u64,
        limit_bytes: u64,
        message: String,
    },

    /// A time or deadline limit was exceeded.
    TimeLimitExceeded {
        elapsed_nanos: u64,
        limit_nanos: u64,
        message: String,
    },

    /// Cooperative cancellation was requested.
    CancellationRequested {
        message: String,
    },

    /// A required capability was not granted.
    CapabilityDenied {
        capability: String,
        operation: String,
        message: String,
    },

    /// A decoder failed to produce a valid result.
    DecoderFailure {
        decoder: DecoderKind,
        message: String,
    },

    /// A numerical operation could not be completed safely.
    NumericalFailure {
        operation: NumericalOperation,
        message: String,
    },

    /// A requested feature or configuration is unsupported.
    UnsupportedConfiguration {
        feature: String,
        message: String,
    },

    /// A version/schema compatibility check failed.
    VersionMismatch {
        component: String,
        expected: String,
        actual: String,
        message: String,
    },

    /// A checkpoint was structurally invalid.
    CheckpointInvalid {
        message: String,
    },

    /// A checkpoint failed integrity validation.
    CheckpointCorrupt {
        message: String,
    },

    /// A cache entry failed validation.
    CacheInvalid {
        message: String,
    },

    /// A classical backend failed.
    BackendFailure {
        backend: String,
        message: String,
    },

    /// A QPU operation failed.
    QpuFailure {
        backend: String,
        operation: String,
        message: String,
    },

    /// An internal invariant was violated.
    ///
    /// This represents a software defect, not normal malformed input.
    InternalInvariantViolation {
        invariant: String,
        message: String,
    },
}

// ============================================================================
// Resource classification
// ============================================================================

/// Canonical resource dimensions understood by QEC resource policy.
///
/// `limits.rs` owns policy. `resources.rs` owns runtime accounting.
/// `errors.rs` only identifies the resource associated with a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    CodeDistance,
    Qubits,
    Stabilizers,
    StabilizerWeight,
    SyndromeEvents,
    MeasurementRounds,
    GraphNodes,
    GraphEdges,
    DecoderIterations,
    DecoderOperations,
    Parallelism,
    Workers,
    MemoryBytes,
    CheckpointSize,
    Checkpoints,
    QpuShots,
    QpuCircuits,
    Allocations,
    Partitions,
    StreamBuffer,
    LogicalWeight,
    Operations,
    Time,
    Custom,
}

impl ResourceKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDistance => "code_distance",
            Self::Qubits => "qubits",
            Self::Stabilizers => "stabilizers",
            Self::StabilizerWeight => "stabilizer_weight",
            Self::SyndromeEvents => "syndrome_events",
            Self::MeasurementRounds => "measurement_rounds",
            Self::GraphNodes => "graph_nodes",
            Self::GraphEdges => "graph_edges",
            Self::DecoderIterations => "decoder_iterations",
            Self::DecoderOperations => "decoder_operations",
            Self::Parallelism => "parallelism",
            Self::Workers => "workers",
            Self::MemoryBytes => "memory_bytes",
            Self::CheckpointSize => "checkpoint_size",
            Self::Checkpoints => "checkpoints",
            Self::QpuShots => "qpu_shots",
            Self::QpuCircuits => "qpu_circuits",
            Self::Allocations => "allocations",
            Self::Partitions => "partitions",
            Self::StreamBuffer => "stream_buffer",
            Self::LogicalWeight => "logical_weight",
            Self::Operations => "operations",
            Self::Time => "time",
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
// Decoder classification
// ============================================================================

/// Decoder responsible for a decoding failure.
///
/// This is descriptive classification, not decoder authorization.
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
    Identity,
    Custom,
}

impl DecoderKind {
    /// Stable machine-readable identifier.
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
            Self::Identity => "identity",
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
// Numerical classification
// ============================================================================

/// Numerical operation associated with a numerical failure.
///
/// This intentionally matches the responsibilities of `arithmetic.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericalOperation {
    ProbabilityValidation,
    LogProbability,
    WeightCalculation,
    DistanceCalculation,
    CoordinateCalculation,
    IntegerConversion,
    FloatingPointConversion,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Combination,
    MemorySizeCalculation,
    IndexCalculation,
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
            Self::Addition => "addition",
            Self::Subtraction => "subtraction",
            Self::Multiplication => "multiplication",
            Self::Division => "division",
            Self::Exponentiation => "exponentiation",
            Self::Combination => "combination",
            Self::MemorySizeCalculation => "memory_size_calculation",
            Self::IndexCalculation => "index_calculation",
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
// Stable error kind
// ============================================================================

/// Stable high-level error category.
///
/// Use this enum for control flow and recovery decisions.
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
    CapabilityDenied,
    DecoderFailure,
    NumericalFailure,
    UnsupportedConfiguration,
    VersionMismatch,
    CheckpointInvalid,
    CheckpointCorrupt,
    CacheInvalid,
    BackendFailure,
    QpuFailure,
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
            Self::CapabilityDenied => "capability_denied",
            Self::DecoderFailure => "decoder_failure",
            Self::NumericalFailure => "numerical_failure",
            Self::UnsupportedConfiguration => "unsupported_configuration",
            Self::VersionMismatch => "version_mismatch",
            Self::CheckpointInvalid => "checkpoint_invalid",
            Self::CheckpointCorrupt => "checkpoint_corrupt",
            Self::CacheInvalid => "cache_invalid",
            Self::BackendFailure => "backend_failure",
            Self::QpuFailure => "qpu_failure",
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

/// Operational severity of a QEC error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QecErrorSeverity {
    /// Caller supplied invalid input.
    Input,

    /// Workload exceeded an intentional safety/resource boundary.
    Resource,

    /// Work was deliberately stopped.
    Cancellation,

    /// An operational subsystem failed.
    Operational,

    /// Configuration or compatibility prevented execution.
    Configuration,

    /// A software invariant was violated.
    Internal,
}

impl QecErrorSeverity {
    /// Stable machine-readable severity.
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
// Constructors
// ============================================================================

impl QecError {
    /// Creates an invalid-input error.
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Creates an invalid-topology error.
    pub fn invalid_topology(message: impl Into<String>) -> Self {
        Self::InvalidTopology {
            message: message.into(),
        }
    }

    /// Creates an invalid-stabilizer error.
    pub fn invalid_stabilizer(message: impl Into<String>) -> Self {
        Self::InvalidStabilizer {
            message: message.into(),
        }
    }

    /// Creates an invalid-syndrome error.
    pub fn invalid_syndrome(message: impl Into<String>) -> Self {
        Self::InvalidSyndrome {
            message: message.into(),
        }
    }

    /// Creates an invalid-graph error.
    pub fn invalid_graph(message: impl Into<String>) -> Self {
        Self::InvalidGraph {
            message: message.into(),
        }
    }

    /// Creates an invalid-probability error.
    pub fn invalid_probability(
        probability: f64,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidProbability {
            probability,
            message: message.into(),
        }
    }

    /// Creates a generic resource-limit failure.
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

    /// Creates a memory-limit failure.
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

    /// Creates a time-limit failure.
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

    /// Creates a cancellation failure.
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::CancellationRequested {
            message: message.into(),
        }
    }

    /// Creates a capability-denied failure.
    pub fn capability_denied(
        capability: impl Into<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::CapabilityDenied {
            capability: capability.into(),
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates a decoder failure.
    pub fn decoder_failure(
        decoder: DecoderKind,
        message: impl Into<String>,
    ) -> Self {
        Self::DecoderFailure {
            decoder,
            message: message.into(),
        }
    }

    /// Creates a numerical failure.
    pub fn numerical_failure(
        operation: NumericalOperation,
        message: impl Into<String>,
    ) -> Self {
        Self::NumericalFailure {
            operation,
            message: message.into(),
        }
    }

    /// Creates an unsupported-configuration failure.
    pub fn unsupported(
        feature: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::UnsupportedConfiguration {
            feature: feature.into(),
            message: message.into(),
        }
    }

    /// Creates a version mismatch.
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

    /// Creates an invalid-checkpoint failure.
    pub fn checkpoint_invalid(message: impl Into<String>) -> Self {
        Self::CheckpointInvalid {
            message: message.into(),
        }
    }

    /// Creates a corrupt-checkpoint failure.
    pub fn checkpoint_corrupt(message: impl Into<String>) -> Self {
        Self::CheckpointCorrupt {
            message: message.into(),
        }
    }

    /// Creates an invalid-cache failure.
    pub fn cache_invalid(message: impl Into<String>) -> Self {
        Self::CacheInvalid {
            message: message.into(),
        }
    }

    /// Creates a classical backend failure.
    pub fn backend_failure(
        backend: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::BackendFailure {
            backend: backend.into(),
            message: message.into(),
        }
    }

    /// Creates a QPU failure.
    pub fn qpu_failure(
        backend: impl Into<String>,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::QpuFailure {
            backend: backend.into(),
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Creates an internal invariant failure.
    pub fn invariant(
        invariant: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InternalInvariantViolation {
            invariant: invariant.into(),
            message: message.into(),
        }
    }

    // ========================================================================
    // Classification
    // ========================================================================

    /// Returns the stable high-level error category.
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
            Self::CapabilityDenied { .. } => QecErrorKind::CapabilityDenied,
            Self::DecoderFailure { .. } => QecErrorKind::DecoderFailure,
            Self::NumericalFailure { .. } => QecErrorKind::NumericalFailure,
            Self::UnsupportedConfiguration { .. } => {
                QecErrorKind::UnsupportedConfiguration
            }
            Self::VersionMismatch { .. } => QecErrorKind::VersionMismatch,
            Self::CheckpointInvalid { .. } => QecErrorKind::CheckpointInvalid,
            Self::CheckpointCorrupt { .. } => QecErrorKind::CheckpointCorrupt,
            Self::CacheInvalid { .. } => QecErrorKind::CacheInvalid,
            Self::BackendFailure { .. } => QecErrorKind::BackendFailure,
            Self::QpuFailure { .. } => QecErrorKind::QpuFailure,
            Self::InternalInvariantViolation { .. } => {
                QecErrorKind::InternalInvariantViolation
            }
        }
    }

    /// Returns the stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        self.kind().as_str()
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

            Self::CancellationRequested { .. } => {
                QecErrorSeverity::Cancellation
            }

            Self::UnsupportedConfiguration { .. }
            | Self::VersionMismatch { .. } => QecErrorSeverity::Configuration,

            Self::InternalInvariantViolation { .. } => {
                QecErrorSeverity::Internal
            }

            Self::CapabilityDenied { .. }
            | Self::DecoderFailure { .. }
            | Self::NumericalFailure { .. }
            | Self::CheckpointInvalid { .. }
            | Self::CheckpointCorrupt { .. }
            | Self::CacheInvalid { .. }
            | Self::BackendFailure { .. }
            | Self::QpuFailure { .. } => QecErrorSeverity::Operational,
        }
    }

    /// Returns the decoder involved in the error, when applicable.
    pub const fn decoder(&self) -> Option<DecoderKind> {
        match self {
            Self::DecoderFailure { decoder, .. } => Some(*decoder),
            _ => None,
        }
    }

    /// Returns the resource involved in the error, when applicable.
    pub const fn resource(&self) -> Option<ResourceKind> {
        match self {
            Self::ResourceLimitExceeded { resource, .. } => Some(*resource),
            Self::MemoryLimitExceeded { .. } => Some(ResourceKind::MemoryBytes),
            Self::TimeLimitExceeded { .. } => Some(ResourceKind::Time),
            _ => None,
        }
    }

    /// Returns the numerical operation involved in the error, when applicable.
    pub const fn numerical_operation(&self) -> Option<NumericalOperation> {
        match self {
            Self::NumericalFailure { operation, .. } => Some(*operation),
            _ => None,
        }
    }

    /// Returns whether this failure is caused by resource exhaustion.
    pub const fn is_resource_failure(&self) -> bool {
        matches!(
            self,
            Self::ResourceLimitExceeded { .. }
                | Self::MemoryLimitExceeded { .. }
                | Self::TimeLimitExceeded { .. }
        )
    }

    /// Returns whether this failure is cancellation.
    pub const fn is_cancelled(&self) -> bool {
        matches!(self, Self::CancellationRequested { .. })
    }

    /// Returns whether this failure indicates invalid caller input.
    pub const fn is_input_failure(&self) -> bool {
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

    /// Returns whether this failure indicates an internal programming defect.
    pub const fn is_internal_failure(&self) -> bool {
        matches!(self, Self::InternalInvariantViolation { .. })
    }

    /// Returns the primary diagnostic message.
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidInput { message }
            | Self::InvalidTopology { message }
            | Self::InvalidStabilizer { message }
            | Self::InvalidSyndrome { message }
            | Self::InvalidGraph { message }
            | Self::CancellationRequested { message }
            | Self::CheckpointInvalid { message }
            | Self::CheckpointCorrupt { message }
            | Self::CacheInvalid { message } => message,

            Self::InvalidProbability { message, .. } => message,

            Self::ResourceLimitExceeded { message, .. } => message,

            Self::MemoryLimitExceeded { message, .. } => message,

            Self::TimeLimitExceeded { message, .. } => message,

            Self::CapabilityDenied { message, .. } => message,

            Self::DecoderFailure { message, .. } => message,

            Self::NumericalFailure { message, .. } => message,

            Self::UnsupportedConfiguration { message, .. } => message,

            Self::VersionMismatch { message, .. } => message,

            Self::BackendFailure { message, .. } => message,

            Self::QpuFailure { message, .. } => message,

            Self::InternalInvariantViolation { message, .. } => message,
        }
    }

    /// Returns the capability name for a capability failure.
    pub fn capability(&self) -> Option<&str> {
        match self {
            Self::CapabilityDenied { capability, .. } => Some(capability),
            _ => None,
        }
    }

    /// Returns the operation for a capability or QPU failure.
    pub fn operation(&self) -> Option<&str> {
        match self {
            Self::CapabilityDenied { operation, .. }
            | Self::QpuFailure { operation, .. } => Some(operation),
            _ => None,
        }
    }

    /// Returns the backend name for a backend/QPU failure.
    pub fn backend(&self) -> Option<&str> {
        match self {
            Self::BackendFailure { backend, .. }
            | Self::QpuFailure { backend, .. } => Some(backend),
            _ => None,
        }
    }

    /// Returns the expected version for a version mismatch.
    pub fn expected_version(&self) -> Option<&str> {
        match self {
            Self::VersionMismatch { expected, .. } => Some(expected),
            _ => None,
        }
    }

    /// Returns the actual version for a version mismatch.
    pub fn actual_version(&self) -> Option<&str> {
        match self {
            Self::VersionMismatch { actual, .. } => Some(actual),
            _ => None,
        }
    }

    /// Returns the component for a version mismatch.
    pub fn component(&self) -> Option<&str> {
        match self {
            Self::VersionMismatch { component, .. } => Some(component),
            _ => None,
        }
    }

    /// Returns the requested resource amount for resource failures.
    pub fn requested_resource(&self) -> Option<u128> {
        match self {
            Self::ResourceLimitExceeded { requested, .. } => Some(*requested),
            Self::MemoryLimitExceeded {
                requested_bytes, ..
            } => Some(u128::from(*requested_bytes)),
            _ => None,
        }
    }

    /// Returns the current resource usage for resource failures.
    pub fn current_resource(&self) -> Option<u128> {
        match self {
            Self::ResourceLimitExceeded { current, .. } => Some(*current),
            Self::MemoryLimitExceeded { current_bytes, .. } => {
                Some(u128::from(*current_bytes))
            }
            _ => None,
        }
    }

    /// Returns the configured resource limit for resource failures.
    pub fn resource_limit_value(&self) -> Option<u128> {
        match self {
            Self::ResourceLimitExceeded { limit, .. } => Some(*limit),
            Self::MemoryLimitExceeded { limit_bytes, .. } => {
                Some(u128::from(*limit_bytes))
            }
            _ => None,
        }
    }

    /// Returns the elapsed duration for a time-limit failure.
    pub fn elapsed(&self) -> Option<Duration> {
        match self {
            Self::TimeLimitExceeded { elapsed_nanos, .. } => {
                Some(Duration::from_nanos(*elapsed_nanos))
            }
            _ => None,
        }
    }

    /// Returns the configured time limit for a time-limit failure.
    pub fn time_limit_duration(&self) -> Option<Duration> {
        match self {
            Self::TimeLimitExceeded { limit_nanos, .. } => {
                Some(Duration::from_nanos(*limit_nanos))
            }
            _ => None,
        }
    }
}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for QecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::InvalidTopology { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::InvalidStabilizer { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::InvalidSyndrome { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::InvalidGraph { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::InvalidProbability {
                probability,
                message,
            } => write!(
                f,
                "[{}] probability={}: {}",
                self.code(),
                probability,
                message
            ),

            Self::ResourceLimitExceeded {
                resource,
                requested,
                current,
                limit,
                message,
            } => write!(
                f,
                "[{}] resource={} requested={} current={} limit={}: {}",
                self.code(),
                resource,
                requested,
                current,
                limit,
                message
            ),

            Self::MemoryLimitExceeded {
                requested_bytes,
                current_bytes,
                limit_bytes,
                message,
            } => write!(
                f,
                "[{}] requested_bytes={} current_bytes={} limit_bytes={}: {}",
                self.code(),
                requested_bytes,
                current_bytes,
                limit_bytes,
                message
            ),

            Self::TimeLimitExceeded {
                elapsed_nanos,
                limit_nanos,
                message,
            } => write!(
                f,
                "[{}] elapsed_nanos={} limit_nanos={}: {}",
                self.code(),
                elapsed_nanos,
                limit_nanos,
                message
            ),

            Self::CancellationRequested { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::CapabilityDenied {
                capability,
                operation,
                message,
            } => write!(
                f,
                "[{}] capability={} operation={}: {}",
                self.code(),
                capability,
                operation,
                message
            ),

            Self::DecoderFailure { decoder, message } => {
                write!(
                    f,
                    "[{}] decoder={}: {}",
                    self.code(),
                    decoder,
                    message
                )
            }

            Self::NumericalFailure {
                operation,
                message,
            } => write!(
                f,
                "[{}] operation={}: {}",
                self.code(),
                operation,
                message
            ),

            Self::UnsupportedConfiguration { feature, message } => {
                write!(
                    f,
                    "[{}] feature={}: {}",
                    self.code(),
                    feature,
                    message
                )
            }

            Self::VersionMismatch {
                component,
                expected,
                actual,
                message,
            } => write!(
                f,
                "[{}] component={} expected={} actual={}: {}",
                self.code(),
                component,
                expected,
                actual,
                message
            ),

            Self::CheckpointInvalid { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::CheckpointCorrupt { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::CacheInvalid { message } => {
                write!(f, "[{}] {}", self.code(), message)
            }

            Self::BackendFailure { backend, message } => {
                write!(
                    f,
                    "[{}] backend={}: {}",
                    self.code(),
                    backend,
                    message
                )
            }

            Self::QpuFailure {
                backend,
                operation,
                message,
            } => write!(
                f,
                "[{}] backend={} operation={}: {}",
                self.code(),
                backend,
                operation,
                message
            ),

            Self::InternalInvariantViolation {
                invariant,
                message,
            } => write!(
                f,
                "[{}] invariant={}: {}",
                self.code(),
                invariant,
                message
            ),
        }
    }
}

impl Error for QecError {}

// ============================================================================
// Arithmetic integration
// ============================================================================

/// Converts the foundational arithmetic error model into the canonical QEC
/// numerical error boundary.
///
/// `arithmetic.rs` intentionally remains independent from `errors.rs`.
/// This conversion belongs here, at the integration boundary.
impl From<crate::quantum::error_correction::arithmetic::ArithmeticError>
    for QecError
{
    fn from(
        error: crate::quantum::error_correction::arithmetic::ArithmeticError,
    ) -> Self {
        use crate::quantum::error_correction::arithmetic::ArithmeticError;

        let operation = match error {
            ArithmeticError::IntegerOverflow
            | ArithmeticError::IntegerUnderflow
            | ArithmeticError::IntegerMultiplicationOverflow => {
                NumericalOperation::Addition
            }

            ArithmeticError::DivisionByZero
            | ArithmeticError::InvalidDenominator => {
                NumericalOperation::Division
            }

            ArithmeticError::AbsoluteValueOverflow => {
                NumericalOperation::IntegerConversion
            }

            ArithmeticError::NaN
            | ArithmeticError::Infinite
            | ArithmeticError::NonFinite
            | ArithmeticError::NumericalOverflow => {
                NumericalOperation::FloatingPointConversion
            }

            ArithmeticError::InvalidProbability
            | ArithmeticError::NegativeProbability => {
                NumericalOperation::ProbabilityValidation
            }

            ArithmeticError::LogarithmOfZero
            | ArithmeticError::LogarithmOfNegative => {
                NumericalOperation::LogProbability
            }

            ArithmeticError::NegativeWeight => {
                NumericalOperation::WeightCalculation
            }

            ArithmeticError::InvalidDistance => {
                NumericalOperation::DistanceCalculation
            }

            ArithmeticError::LimitExceeded => {
                NumericalOperation::MemorySizeCalculation
            }

            ArithmeticError::ConversionOverflow => {
                NumericalOperation::IntegerConversion
            }

            ArithmeticError::ExponentiationOverflow => {
                NumericalOperation::Exponentiation
            }

            ArithmeticError::CombinationOverflow => {
                NumericalOperation::Combination
            }

            ArithmeticError::InvalidOperation => {
                NumericalOperation::Custom
            }
        };

        Self::numerical_failure(operation, error.to_string())
    }
}

// ============================================================================
// Convenience conversions for common standard-library errors
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
        Self::invalid_input(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for QecError {
    fn from(error: std::num::ParseFloatError) -> Self {
        Self::invalid_input(error.to_string())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_result_is_usable() {
        let result: QecResult<u64> = Ok(42);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn stable_kind_and_code_are_consistent() {
        let error = QecError::invalid_topology("invalid boundary");

        assert_eq!(error.kind(), QecErrorKind::InvalidTopology);
        assert_eq!(error.code(), "invalid_topology");
        assert_eq!(error.severity(), QecErrorSeverity::Input);
    }

    #[test]
    fn resource_errors_are_classified_correctly() {
        let error = QecError::resource_limit(
            ResourceKind::GraphNodes,
            101,
            100,
            100,
            "graph node limit exceeded",
        );

        assert!(error.is_resource_failure());
        assert_eq!(error.resource(), Some(ResourceKind::GraphNodes));
        assert_eq!(error.requested_resource(), Some(101));
        assert_eq!(error.current_resource(), Some(100));
        assert_eq!(error.resource_limit_value(), Some(100));
    }

    #[test]
    fn memory_errors_are_resource_failures() {
        let error =
            QecError::memory_limit(2048, 4096, 4096, "memory denied");

        assert!(error.is_resource_failure());
        assert_eq!(
            error.resource(),
            Some(ResourceKind::MemoryBytes)
        );
        assert_eq!(error.requested_resource(), Some(2048));
    }

    #[test]
    fn cancellation_is_not_a_generic_failure() {
        let error = QecError::cancelled("deadline reached");

        assert!(error.is_cancelled());
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
        let error =
            QecError::decoder_failure(DecoderKind::Mwpm, "matching failed");

        assert_eq!(error.decoder(), Some(DecoderKind::Mwpm));
        assert_eq!(
            error.kind(),
            QecErrorKind::DecoderFailure
        );
    }

    #[test]
    fn numerical_metadata_is_available() {
        let error = QecError::numerical_failure(
            NumericalOperation::Combination,
            "overflow",
        );

        assert_eq!(
            error.numerical_operation(),
            Some(NumericalOperation::Combination)
        );
        assert_eq!(
            error.kind(),
            QecErrorKind::NumericalFailure
        );
    }

    #[test]
    fn capability_failure_is_explicit() {
        let error = QecError::capability_denied(
            "qpu_submit",
            "submit_circuit",
            "capability not granted",
        );

        assert_eq!(
            error.kind(),
            QecErrorKind::CapabilityDenied
        );
        assert_eq!(error.capability(), Some("qpu_submit"));
        assert_eq!(error.operation(), Some("submit_circuit"));
    }

    #[test]
    fn version_mismatch_exposes_versions() {
        let error = QecError::version_mismatch(
            "checkpoint",
            "2.1",
            "1.0",
            "schema mismatch",
        );

        assert_eq!(
            error.kind(),
            QecErrorKind::VersionMismatch
        );
        assert_eq!(error.component(), Some("checkpoint"));
        assert_eq!(error.expected_version(), Some("2.1"));
        assert_eq!(error.actual_version(), Some("1.0"));
    }

    #[test]
    fn qpu_failure_does_not_expose_credentials() {
        let error = QecError::qpu_failure(
            "backend-a",
            "submit",
            "submission rejected",
        );

        let rendered = error.to_string();

        assert!(rendered.contains("backend-a"));
        assert!(rendered.contains("submission rejected"));
        assert!(!rendered.contains("password"));
        assert!(!rendered.contains("secret"));
        assert!(!rendered.contains("token"));
        assert!(!rendered.contains("api_key"));
    }

    #[test]
    fn internal_invariant_is_classified_as_internal() {
        let error =
            QecError::invariant("decoder state", "unreachable state");

        assert!(error.is_internal_failure());
        assert_eq!(
            error.severity(),
            QecErrorSeverity::Internal
        );
    }

    #[test]
    fn arithmetic_error_maps_to_numerical_failure() {
        use crate::quantum::error_correction::arithmetic::ArithmeticError;

        let error: QecError = ArithmeticError::CombinationOverflow.into();

        assert_eq!(
            error.kind(),
            QecErrorKind::NumericalFailure
        );
        assert_eq!(
            error.numerical_operation(),
            Some(NumericalOperation::Combination)
        );
    }

    #[test]
    fn display_contains_stable_code() {
        let error = QecError::invalid_input("bad input");
        let text = error.to_string();

        assert!(text.starts_with("[invalid_input]"));
        assert!(text.contains("bad input"));
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
        assert_eq!(DecoderKind::Mwpm.as_str(), "mwpm");
        assert_eq!(
            DecoderKind::UnionFind.as_str(),
            "union_find"
        );
    }

    #[test]
    fn numerical_operation_identifiers_are_stable() {
        assert_eq!(
            NumericalOperation::Combination.as_str(),
            "combination"
        );
        assert_eq!(
            NumericalOperation::MemorySizeCalculation.as_str(),
            "memory_size_calculation"
        );
    }

    #[test]
    fn time_limit_round_trip_is_safe() {
        let error =
            QecError::time_limit(5_000_000, 10_000_000, "deadline");

        assert_eq!(
            error.elapsed(),
            Some(Duration::from_nanos(5_000_000))
        );

        assert_eq!(
            error.time_limit_duration(),
            Some(Duration::from_nanos(10_000_000))
        );
    }
}