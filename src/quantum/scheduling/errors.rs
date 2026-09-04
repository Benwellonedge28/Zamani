//! Zamani Quantum Scheduling — Canonical Error Model
//!
//! This module defines the stable error contract for
//! `crate::quantum::scheduling`.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "Why could a scheduling request not be accepted, planned,
//! > verified, serialized, or completed?"
//!
//! It owns:
//!
//! - `SchedulingError`;
//! - structured scheduling failure categories;
//! - operation/resource/qubit/timing context attached to failures;
//! - stable human-readable error formatting;
//! - `std::error::Error` integration;
//! - deterministic error classification;
//! - conversion from scheduler-local error types;
//! - a small result alias for scheduler APIs.
//!
//! It does NOT own:
//!
//! - scheduling algorithms;
//! - dependency graphs;
//! - resource calendars;
//! - timing calculations;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - QEC decoding;
//! - noise modelling;
//! - frontend parsing;
//! - quantum semantics;
//! - compiler policy.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! Scheduling uses the canonical Zamani IR identities:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! crate::quantum::ir::core::identity::ResourceId
//! crate::quantum::ir::core::identity::ScheduleId
//!
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No scheduler-specific `QubitId` or `PhysicalQubitId` is defined here.
//!
//! This is intentional. The repository's canonical quantum IR requires
//! logical and physical qubit identity to remain separate and centrally
//! owned by `quantum::ir::qubit`.
//!
//! # Error design principles
//!
//! A production scheduler must never require callers to parse an error string
//! in order to determine what went wrong.
//!
//! Therefore errors contain structured fields whenever the information is
//! known.
//!
//! Human-readable text is presentation only.
//!
//! Machine logic should use:
//!
//! - `SchedulingError::kind()`;
//! - `SchedulingError::operation_id()`;
//! - `SchedulingError::resource_id()`;
//! - `SchedulingError::qubit()`;
//! - `SchedulingError::is_retryable()`;
//! - `SchedulingError::is_invalid_input()`.
//!
//! # Scalability
//!
//! No hardware size is encoded in this module.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum schedule depth;
//! - maximum dependency count;
//! - fixed hardware topology;
//! - fixed channel count.
//!
//! Numeric values used for identifiers and temporal quantities are merely
//! representations supplied by the canonical types.
//!
//! Actual resource limits belong to explicit scheduling policy, target
//! capabilities, host resources, or execution limits.
//!
//! # Determinism
//!
//! Error variants contain no timestamps generated from the wall clock and no
//! hidden global state. Given the same failed scheduling request and the same
//! deterministic context, the same structured error can be produced.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Integration
//!
//! Dependency direction:
//!
//! ```text
//! quantum::ir::core::identity
//! quantum::ir::qubit
//!             │
//!             ▼
//!     scheduling::types
//!             │
//!             ▼
//!     scheduling::errors
//!             │
//!       ┌─────┼──────────────┐
//!       ▼     ▼              ▼
//!    planner constraints verification
//!       │     │              │
//!       └─────┼──────────────┘
//!             ▼
//!        ScheduleResult
//! ```
//!
//! The error module does not depend on planners, algorithms, hardware,
//! routing, QEC, or runtime modules.
//!
//! This keeps the contract stable while those subsystems evolve.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;

use crate::quantum::ir::core::identity::{
    OperationId,
    ResourceId,
    ScheduleId,
};
use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

// These types are defined by scheduling::types.
// Keeping the dependency one-way allows this file to remain the canonical
// error contract while all scheduler implementations evolve behind it.
use super::types::{
    DependencyId,
    Duration,
    ReservationId,
    TimePoint,
};

// ============================================================================
// Public result aliases
// ============================================================================

/// Canonical result type for scheduling APIs.
///
/// Every public scheduling operation should prefer:
///
/// ```text
/// SchedulingResult<T>
/// ```
///
/// instead of introducing another scheduler-specific result alias.
///
/// This keeps error handling uniform across:
///
/// - planners;
/// - policies;
/// - resource management;
/// - verification;
/// - transformations;
/// - serialization;
/// - diagnostics;
/// - plugins.
pub type SchedulingResult<T> = Result<T, SchedulingError>;

// ============================================================================
// Error kind
// ============================================================================

/// Stable machine-readable classification of a scheduling error.
///
/// This enum is intentionally independent from the concrete error payload.
/// Consumers that need to branch on error categories should use this type
/// rather than parsing display strings.
///
/// New variants may be added in future compatible releases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingErrorKind {
    /// The caller supplied an invalid scheduling input.
    InvalidInput,

    /// An operation or operation description is invalid.
    InvalidOperation,

    /// The dependency representation is invalid.
    InvalidDependencyGraph,

    /// A dependency cycle prevents scheduling.
    CycleDetected,

    /// An operation requires timing information that was not supplied.
    MissingDuration,

    /// A supplied duration is invalid.
    InvalidDuration,

    /// A requested resource is not currently available.
    ResourceUnavailable,

    /// A resource reservation conflicts with another reservation.
    ResourceConflict,

    /// A timing requirement cannot be satisfied.
    TimingConflict,

    /// An alignment requirement cannot be satisfied.
    AlignmentViolation,

    /// A generic scheduler constraint was violated.
    ConstraintViolation,

    /// The target does not support an operation.
    UnsupportedOperation,

    /// The requested computation cannot be scheduled under the supplied
    /// constraints and resources.
    Unschedulable,

    /// A schedule cannot meet its deadline.
    DeadlineExceeded,

    /// A resource capacity requirement exceeds the available capacity.
    CapacityExceeded,

    /// The produced schedule failed verification.
    VerificationFailed,

    /// A schedule could not be encoded or decoded.
    SerializationError,

    /// A dynamically loaded or registered scheduler failed.
    PluginError,

    /// Scheduling was explicitly cancelled.
    Cancelled,

    /// A scheduling operation exceeded an execution deadline.
    Timeout,

    /// An internal invariant was violated.
    Internal,
}

impl SchedulingErrorKind {
    /// Returns whether the error normally indicates invalid caller input.
    #[must_use]
    pub const fn is_invalid_input(self) -> bool {
        matches!(
            self,
            Self::InvalidInput
                | Self::InvalidOperation
                | Self::InvalidDependencyGraph
                | Self::CycleDetected
                | Self::MissingDuration
                | Self::InvalidDuration
                | Self::UnsupportedOperation
        )
    }

    /// Returns whether the error may be retryable after the execution
    /// environment changes.
    #[must_use]
    pub const fn is_environmental(self) -> bool {
        matches!(
            self,
            Self::ResourceUnavailable
                | Self::ResourceConflict
                | Self::TimingConflict
                | Self::AlignmentViolation
                | Self::CapacityExceeded
                | Self::DeadlineExceeded
                | Self::Timeout
        )
    }

    /// Returns whether the caller may reasonably retry without changing the
    /// semantic input.
    #[must_use]
    pub const fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::ResourceUnavailable
                | Self::ResourceConflict
                | Self::TimingConflict
                | Self::CapacityExceeded
                | Self::Timeout
        )
    }

    /// Returns a stable category string suitable for logs, telemetry, or
    /// external adapters.
    ///
    /// The returned strings are identifiers, not user-facing prose.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidOperation => "invalid_operation",
            Self::InvalidDependencyGraph => "invalid_dependency_graph",
            Self::CycleDetected => "cycle_detected",
            Self::MissingDuration => "missing_duration",
            Self::InvalidDuration => "invalid_duration",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::ResourceConflict => "resource_conflict",
            Self::TimingConflict => "timing_conflict",
            Self::AlignmentViolation => "alignment_violation",
            Self::ConstraintViolation => "constraint_violation",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::Unschedulable => "unschedulable",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::CapacityExceeded => "capacity_exceeded",
            Self::VerificationFailed => "verification_failed",
            Self::SerializationError => "serialization_error",
            Self::PluginError => "plugin_error",
            Self::Cancelled => "cancelled",
            Self::Timeout => "timeout",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for SchedulingErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Scheduling error
// ============================================================================

/// Canonical production scheduling error.
///
/// The variants intentionally carry enough context for diagnostics without
/// requiring a dependency on scheduler implementation modules.
///
/// The scheduler may add richer diagnostic information through
/// `diagnostics::*`; this error remains the concise machine-readable failure
/// contract.
///
/// # Important
///
/// An error is not itself a schedule.
///
/// A scheduler must never return a partially constructed schedule as a
/// successful `Ok` value after detecting one of these errors unless the API
/// explicitly defines an analysis/partial-result mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingError {
    /// General invalid scheduler input.
    InvalidInput {
        /// Stable explanation of what was invalid.
        reason: String,
    },

    /// An operation contains invalid scheduling information.
    InvalidOperation {
        /// Operation responsible for the failure, when known.
        operation: Option<OperationId>,

        /// Optional logical qubit involved in the failure.
        qubit: Option<QubitId>,

        /// Optional physical qubit involved in the failure.
        physical_qubit: Option<PhysicalQubitId>,

        /// Stable explanation.
        reason: String,
    },

    /// A dependency relationship is invalid.
    InvalidDependencyGraph {
        /// Dependency responsible for the failure, when known.
        dependency: Option<DependencyId>,

        /// Predecessor operation, when known.
        predecessor: Option<OperationId>,

        /// Successor operation, when known.
        successor: Option<OperationId>,

        /// Stable explanation.
        reason: String,
    },

    /// A cycle was detected in a graph that must be acyclic.
    CycleDetected {
        /// An operation participating in the detected cycle, when known.
        operation: Option<OperationId>,

        /// Optional dependency participating in the cycle.
        dependency: Option<DependencyId>,

        /// Number of nodes identified in the reported cycle, when known.
        ///
        /// This is diagnostic metadata, not a scheduler limit.
        cycle_size: Option<u128>,
    },

    /// An operation has no usable duration information.
    MissingDuration {
        /// Operation requiring the duration.
        operation: OperationId,
    },

    /// An operation contains an invalid duration.
    InvalidDuration {
        /// Operation associated with the invalid duration, when known.
        operation: Option<OperationId>,

        /// Duration value, when it was representable.
        duration: Option<Duration>,

        /// Stable explanation.
        reason: String,
    },

    /// A resource cannot currently be used.
    ResourceUnavailable {
        /// Resource requested.
        resource: ResourceId,

        /// Operation requesting the resource, when known.
        operation: Option<OperationId>,

        /// Requested start time, when known.
        requested_start: Option<TimePoint>,

        /// Requested duration, when known.
        requested_duration: Option<Duration>,

        /// Stable explanation.
        reason: String,
    },

    /// A resource reservation overlaps or otherwise conflicts with another
    /// reservation.
    ResourceConflict {
        /// Resource involved in the conflict.
        resource: ResourceId,

        /// Operation requesting the conflicting reservation.
        operation: Option<OperationId>,

        /// Operation already occupying the resource, when known.
        conflicting_operation: Option<OperationId>,

        /// Existing reservation, when known.
        conflicting_reservation: Option<ReservationId>,

        /// Requested interval start, when known.
        requested_start: Option<TimePoint>,

        /// Existing reservation start, when known.
        conflicting_start: Option<TimePoint>,

        /// Stable explanation.
        reason: String,
    },

    /// Temporal constraints cannot all be satisfied.
    TimingConflict {
        /// Operation affected by the conflict, when known.
        operation: Option<OperationId>,

        /// Earliest permitted start, when known.
        earliest_start: Option<TimePoint>,

        /// Latest permitted start, when known.
        latest_start: Option<TimePoint>,

        /// Stable explanation.
        reason: String,
    },

    /// A target alignment requirement was violated.
    AlignmentViolation {
        /// Operation affected by the violation.
        operation: Option<OperationId>,

        /// Requested start time, when known.
        requested_start: Option<TimePoint>,

        /// Alignment quantum/grid, when representable by the scheduling
        /// timing model.
        alignment: Option<Duration>,

        /// Stable explanation.
        reason: String,
    },

    /// A generic scheduling constraint was violated.
    ConstraintViolation {
        /// Operation affected by the constraint, when known.
        operation: Option<OperationId>,

        /// Resource affected by the constraint, when known.
        resource: Option<ResourceId>,

        /// Logical qubit affected by the constraint, when known.
        qubit: Option<QubitId>,

        /// Physical qubit affected by the constraint, when known.
        physical_qubit: Option<PhysicalQubitId>,

        /// Stable constraint identifier.
        constraint: String,

        /// Stable explanation.
        reason: String,
    },

    /// The target does not support the requested operation.
    UnsupportedOperation {
        /// Operation that is unsupported, when known.
        operation: Option<OperationId>,

        /// Canonical operation name.
        operation_name: String,

        /// Stable explanation.
        reason: String,
    },

    /// The complete request cannot be scheduled under the supplied target,
    /// resource, timing and policy constraints.
    Unschedulable {
        /// Operation at which scheduling became impossible, when known.
        operation: Option<OperationId>,

        /// Stable explanation.
        reason: String,
    },

    /// The requested deadline cannot be satisfied.
    DeadlineExceeded {
        /// Operation causing the deadline miss, when known.
        operation: Option<OperationId>,

        /// Schedule identifier, when known.
        schedule: Option<ScheduleId>,

        /// Required deadline.
        deadline: TimePoint,

        /// Predicted completion time, when known.
        completion: Option<TimePoint>,
    },

    /// A resource capacity requirement cannot be satisfied.
    CapacityExceeded {
        /// Resource whose capacity was exceeded.
        resource: ResourceId,

        /// Operation requesting the capacity, when known.
        operation: Option<OperationId>,

        /// Required capacity.
        required: u128,

        /// Available capacity.
        available: u128,
    },

    /// A completed or candidate schedule failed verification.
    VerificationFailed {
        /// Operation involved in the failed invariant, when known.
        operation: Option<OperationId>,

        /// Resource involved in the failed invariant, when known.
        resource: Option<ResourceId>,

        /// Stable verification category.
        invariant: String,

        /// Stable explanation.
        reason: String,
    },

    /// Serialization or deserialization failed.
    SerializationError {
        /// Encoding/decoding operation.
        operation: SerializationOperation,

        /// Stable schema identifier, when known.
        schema: Option<String>,

        /// Stable explanation.
        reason: String,
    },

    /// A scheduler plugin failed.
    PluginError {
        /// Plugin identifier.
        plugin: String,

        /// Stable plugin operation.
        operation: PluginOperation,

        /// Stable explanation.
        reason: String,
    },

    /// Scheduling was explicitly cancelled.
    Cancelled {
        /// Optional stable cancellation reason.
        reason: Option<String>,
    },

    /// Scheduling exceeded an externally supplied execution deadline.
    Timeout {
        /// Operation being processed when timeout was observed, when known.
        operation: Option<OperationId>,

        /// Optional execution deadline.
        deadline: Option<TimePoint>,
    },

    /// An internal scheduler invariant was violated.
    ///
    /// This should be rare. It represents a scheduler implementation failure,
    /// not an expected property of a difficult quantum program.
    Internal {
        /// Stable invariant identifier.
        invariant: String,

        /// Stable explanation.
        reason: String,
    },
}

// ============================================================================
// Supporting enums
// ============================================================================

/// Serialization operation that failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SerializationOperation {
    /// Encoding an in-memory schedule.
    Encode,

    /// Decoding an external representation.
    Decode,

    /// Schema validation.
    Validate,
}

impl fmt::Display for SerializationOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Encode => "encode",
            Self::Decode => "decode",
            Self::Validate => "validate",
        };

        formatter.write_str(value)
    }
}

/// Operation performed by a scheduler plugin when a plugin error occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PluginOperation {
    /// Plugin initialization.
    Initialize,

    /// Plugin capability discovery.
    Describe,

    /// Plugin scheduling.
    Schedule,

    /// Plugin verification.
    Verify,

    /// Plugin shutdown.
    Shutdown,

    /// Other plugin-defined operation.
    Other,
}

impl fmt::Display for PluginOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Initialize => "initialize",
            Self::Describe => "describe",
            Self::Schedule => "schedule",
            Self::Verify => "verify",
            Self::Shutdown => "shutdown",
            Self::Other => "other",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// Error classification
// ============================================================================

impl SchedulingError {
    /// Returns the stable machine-readable error category.
    #[must_use]
    pub const fn kind(&self) -> SchedulingErrorKind {
        match self {
            Self::InvalidInput { .. } => SchedulingErrorKind::InvalidInput,
            Self::InvalidOperation { .. } => SchedulingErrorKind::InvalidOperation,
            Self::InvalidDependencyGraph { .. } => {
                SchedulingErrorKind::InvalidDependencyGraph
            }
            Self::CycleDetected { .. } => SchedulingErrorKind::CycleDetected,
            Self::MissingDuration { .. } => SchedulingErrorKind::MissingDuration,
            Self::InvalidDuration { .. } => SchedulingErrorKind::InvalidDuration,
            Self::ResourceUnavailable { .. } => {
                SchedulingErrorKind::ResourceUnavailable
            }
            Self::ResourceConflict { .. } => SchedulingErrorKind::ResourceConflict,
            Self::TimingConflict { .. } => SchedulingErrorKind::TimingConflict,
            Self::AlignmentViolation { .. } => {
                SchedulingErrorKind::AlignmentViolation
            }
            Self::ConstraintViolation { .. } => {
                SchedulingErrorKind::ConstraintViolation
            }
            Self::UnsupportedOperation { .. } => {
                SchedulingErrorKind::UnsupportedOperation
            }
            Self::Unschedulable { .. } => SchedulingErrorKind::Unschedulable,
            Self::DeadlineExceeded { .. } => SchedulingErrorKind::DeadlineExceeded,
            Self::CapacityExceeded { .. } => SchedulingErrorKind::CapacityExceeded,
            Self::VerificationFailed { .. } => {
                SchedulingErrorKind::VerificationFailed
            }
            Self::SerializationError { .. } => {
                SchedulingErrorKind::SerializationError
            }
            Self::PluginError { .. } => SchedulingErrorKind::PluginError,
            Self::Cancelled { .. } => SchedulingErrorKind::Cancelled,
            Self::Timeout { .. } => SchedulingErrorKind::Timeout,
            Self::Internal { .. } => SchedulingErrorKind::Internal,
        }
    }

    /// Returns the operation ID associated with the error, if known.
    #[must_use]
    pub const fn operation_id(&self) -> Option<OperationId> {
        match self {
            Self::InvalidInput { .. } => None,

            Self::InvalidOperation { operation, .. } => *operation,

            Self::InvalidDependencyGraph {
                predecessor,
                successor,
                ..
            } => match predecessor {
                Some(id) => Some(*id),
                None => *successor,
            },

            Self::CycleDetected { operation, .. } => *operation,

            Self::MissingDuration { operation } => Some(*operation),

            Self::InvalidDuration { operation, .. } => *operation,

            Self::ResourceUnavailable { operation, .. } => *operation,

            Self::ResourceConflict { operation, .. } => *operation,

            Self::TimingConflict { operation, .. } => *operation,

            Self::AlignmentViolation { operation, .. } => *operation,

            Self::ConstraintViolation { operation, .. } => *operation,

            Self::UnsupportedOperation { operation, .. } => *operation,

            Self::Unschedulable { operation, .. } => *operation,

            Self::DeadlineExceeded { operation, .. } => *operation,

            Self::CapacityExceeded { operation, .. } => *operation,

            Self::VerificationFailed { operation, .. } => *operation,

            Self::SerializationError { .. } => None,

            Self::PluginError { .. } => None,

            Self::Cancelled { .. } => None,

            Self::Timeout { operation, .. } => *operation,

            Self::Internal { .. } => None,
        }
    }

    /// Returns the resource ID associated with the error, if known.
    #[must_use]
    pub const fn resource_id(&self) -> Option<ResourceId> {
        match self {
            Self::ResourceUnavailable { resource, .. } => Some(*resource),

            Self::ResourceConflict { resource, .. } => Some(*resource),

            Self::ConstraintViolation { resource, .. } => *resource,

            Self::CapacityExceeded { resource, .. } => Some(*resource),

            Self::VerificationFailed { resource, .. } => *resource,

            _ => None,
        }
    }

    /// Returns the logical qubit associated with the error, if known.
    #[must_use]
    pub const fn qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::InvalidOperation { qubit, .. } => *qubit,

            Self::ConstraintViolation { qubit, .. } => *qubit,

            _ => None,
        }
    }

    /// Returns the physical qubit associated with the error, if known.
    #[must_use]
    pub const fn physical_qubit_id(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::InvalidOperation {
                physical_qubit, ..
            } => *physical_qubit,

            Self::ConstraintViolation {
                physical_qubit, ..
            } => *physical_qubit,

            _ => None,
        }
    }

    /// Returns whether the error represents invalid caller/program input.
    #[must_use]
    pub const fn is_invalid_input(&self) -> bool {
        self.kind().is_invalid_input()
    }

    /// Returns whether the error may be caused by the current execution
    /// environment rather than the semantic program.
    #[must_use]
    pub const fn is_environmental(&self) -> bool {
        self.kind().is_environmental()
    }

    /// Returns whether retrying may succeed without changing the semantic
    /// program.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        self.kind().is_retryable()
    }

    /// Returns whether this error indicates scheduler implementation failure.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(self, Self::Internal { .. })
    }

    /// Returns whether scheduling was explicitly cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled { .. })
    }

    /// Returns a concise stable error-code string.
    ///
    /// This is appropriate for machine-readable diagnostics, APIs and
    /// telemetry. It must not be used as a substitute for structured matching
    /// inside Rust code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind().as_str()
    }

    /// Creates a generic invalid-input error.
    #[must_use]
    pub fn invalid_input(reason: impl Into<String>) -> Self {
        Self::InvalidInput {
            reason: reason.into(),
        }
    }

    /// Creates a generic unschedulable error.
    #[must_use]
    pub fn unschedulable(reason: impl Into<String>) -> Self {
        Self::Unschedulable {
            operation: None,
            reason: reason.into(),
        }
    }

    /// Creates a cancellation error without a reason.
    #[must_use]
    pub const fn cancelled() -> Self {
        Self::Cancelled { reason: None }
    }

    /// Creates a cancellation error with a reason.
    #[must_use]
    pub fn cancelled_with_reason(reason: impl Into<String>) -> Self {
        Self::Cancelled {
            reason: Some(reason.into()),
        }
    }
}

// ============================================================================
// std::error::Error
// ============================================================================

impl Error for SchedulingError {}

// ============================================================================
// Display
// ============================================================================

impl fmt::Display for SchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { reason } => {
                write!(formatter, "invalid scheduling input: {reason}")
            }

            Self::InvalidOperation {
                operation,
                qubit,
                physical_qubit,
                reason,
            } => {
                write!(formatter, "invalid scheduling operation")?;

                if let Some(operation) = operation {
                    write!(formatter, " {operation}")?;
                }

                if let Some(qubit) = qubit {
                    write!(formatter, " logical qubit {qubit}")?;
                }

                if let Some(physical_qubit) = physical_qubit {
                    write!(formatter, " physical qubit {physical_qubit}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::InvalidDependencyGraph {
                dependency,
                predecessor,
                successor,
                reason,
            } => {
                write!(formatter, "invalid dependency graph")?;

                if let Some(dependency) = dependency {
                    write!(formatter, " dependency {dependency}")?;
                }

                if let Some(predecessor) = predecessor {
                    write!(formatter, " predecessor {predecessor}")?;
                }

                if let Some(successor) = successor {
                    write!(formatter, " successor {successor}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::CycleDetected {
                operation,
                dependency,
                cycle_size,
            } => {
                write!(formatter, "dependency cycle detected")?;

                if let Some(operation) = operation {
                    write!(formatter, " at {operation}")?;
                }

                if let Some(dependency) = dependency {
                    write!(formatter, " through {dependency}")?;
                }

                if let Some(size) = cycle_size {
                    write!(formatter, " (reported cycle size {size})")?;
                }

                Ok(())
            }

            Self::MissingDuration { operation } => {
                write!(
                    formatter,
                    "missing scheduling duration for operation {operation}"
                )
            }

            Self::InvalidDuration {
                operation,
                duration,
                reason,
            } => {
                write!(formatter, "invalid operation duration")?;

                if let Some(operation) = operation {
                    write!(formatter, " for {operation}")?;
                }

                if let Some(duration) = duration {
                    write!(formatter, " ({duration})")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::ResourceUnavailable {
                resource,
                operation,
                requested_start,
                requested_duration,
                reason,
            } => {
                write!(formatter, "resource {resource} unavailable")?;

                if let Some(operation) = operation {
                    write!(formatter, " for operation {operation}")?;
                }

                if let Some(start) = requested_start {
                    write!(formatter, " at {start}")?;
                }

                if let Some(duration) = requested_duration {
                    write!(formatter, " for {duration}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::ResourceConflict {
                resource,
                operation,
                conflicting_operation,
                conflicting_reservation,
                requested_start,
                conflicting_start,
                reason,
            } => {
                write!(formatter, "resource conflict on {resource}")?;

                if let Some(operation) = operation {
                    write!(formatter, " for operation {operation}")?;
                }

                if let Some(conflicting_operation) = conflicting_operation {
                    write!(
                        formatter,
                        " with operation {conflicting_operation}"
                    )?;
                }

                if let Some(reservation) = conflicting_reservation {
                    write!(formatter, " reservation {reservation}")?;
                }

                if let Some(start) = requested_start {
                    write!(formatter, " requested at {start}")?;
                }

                if let Some(start) = conflicting_start {
                    write!(formatter, ", conflicting interval starts at {start}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::TimingConflict {
                operation,
                earliest_start,
                latest_start,
                reason,
            } => {
                write!(formatter, "timing conflict")?;

                if let Some(operation) = operation {
                    write!(formatter, " for operation {operation}")?;
                }

                if let Some(earliest) = earliest_start {
                    write!(formatter, " earliest={earliest}")?;
                }

                if let Some(latest) = latest_start {
                    write!(formatter, " latest={latest}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::AlignmentViolation {
                operation,
                requested_start,
                alignment,
                reason,
            } => {
                write!(formatter, "timing alignment violation")?;

                if let Some(operation) = operation {
                    write!(formatter, " for operation {operation}")?;
                }

                if let Some(start) = requested_start {
                    write!(formatter, " at {start}")?;
                }

                if let Some(alignment) = alignment {
                    write!(formatter, " with alignment {alignment}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::ConstraintViolation {
                operation,
                resource,
                qubit,
                physical_qubit,
                constraint,
                reason,
            } => {
                write!(
                    formatter,
                    "constraint violation [{constraint}]"
                )?;

                if let Some(operation) = operation {
                    write!(formatter, " operation {operation}")?;
                }

                if let Some(resource) = resource {
                    write!(formatter, " resource {resource}")?;
                }

                if let Some(qubit) = qubit {
                    write!(formatter, " logical qubit {qubit}")?;
                }

                if let Some(physical_qubit) = physical_qubit {
                    write!(formatter, " physical qubit {physical_qubit}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::UnsupportedOperation {
                operation,
                operation_name,
                reason,
            } => {
                write!(
                    formatter,
                    "unsupported quantum operation {operation_name}"
                )?;

                if let Some(operation) = operation {
                    write!(formatter, " ({operation})")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::Unschedulable { operation, reason } => {
                write!(formatter, "schedule is unschedulable")?;

                if let Some(operation) = operation {
                    write!(formatter, " near operation {operation}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::DeadlineExceeded {
                operation,
                schedule,
                deadline,
                completion,
            } => {
                write!(
                    formatter,
                    "scheduling deadline exceeded: deadline={deadline}"
                )?;

                if let Some(completion) = completion {
                    write!(formatter, ", completion={completion}")?;
                }

                if let Some(operation) = operation {
                    write!(formatter, ", operation={operation}")?;
                }

                if let Some(schedule) = schedule {
                    write!(formatter, ", schedule={schedule}")?;
                }

                Ok(())
            }

            Self::CapacityExceeded {
                resource,
                operation,
                required,
                available,
            } => {
                write!(
                    formatter,
                    "resource {resource} capacity exceeded: required={required}, available={available}"
                )?;

                if let Some(operation) = operation {
                    write!(formatter, ", operation={operation}")?;
                }

                Ok(())
            }

            Self::VerificationFailed {
                operation,
                resource,
                invariant,
                reason,
            } => {
                write!(
                    formatter,
                    "schedule verification failed [{invariant}]"
                )?;

                if let Some(operation) = operation {
                    write!(formatter, " operation {operation}")?;
                }

                if let Some(resource) = resource {
                    write!(formatter, " resource {resource}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::SerializationError {
                operation,
                schema,
                reason,
            } => {
                write!(
                    formatter,
                    "schedule serialization {}",
                    operation
                )?;

                if let Some(schema) = schema {
                    write!(formatter, " using schema {schema}")?;
                }

                write!(formatter, ": {reason}")
            }

            Self::PluginError {
                plugin,
                operation,
                reason,
            } => {
                write!(
                    formatter,
                    "scheduler plugin {plugin} failed during {operation}: {reason}"
                )
            }

            Self::Cancelled { reason } => {
                write!(formatter, "scheduling cancelled")?;

                if let Some(reason) = reason {
                    write!(formatter, ": {reason}")?;
                }

                Ok(())
            }

            Self::Timeout {
                operation,
                deadline,
            } => {
                write!(formatter, "scheduling timed out")?;

                if let Some(operation) = operation {
                    write!(formatter, " while processing {operation}")?;
                }

                if let Some(deadline) = deadline {
                    write!(formatter, " at deadline {deadline}")?;
                }

                Ok(())
            }

            Self::Internal {
                invariant,
                reason,
            } => {
                write!(
                    formatter,
                    "internal scheduling invariant violated [{invariant}]: {reason}"
                )
            }
        }
    }
}

// ============================================================================
// Error source support
// ============================================================================
//
// `SchedulingError` intentionally stores stable scheduler-domain information
// directly instead of wrapping arbitrary source errors. This keeps the public
// contract independent from concrete serialization, plugin, hardware, or
// runtime libraries.
//
// Those subsystem-specific errors should be translated into one of the
// structured SchedulingError variants at the integration boundary.
//
// This avoids coupling scheduling/errors.rs to every future dependency.

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // The test module intentionally tests only this file's contract.
    // It does not instantiate planners, hardware, routing, QEC, or runtime
    // objects. Those belong to integration tests.

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn schedule(value: u64) -> ScheduleId {
        ScheduleId::new(value)
    }

    fn logical_qubit(value: usize) -> QubitId {
        QubitId::new(value)
    }

    fn physical_qubit(value: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(value)
    }

    fn time(value: u128) -> TimePoint {
        TimePoint::new(value)
    }

    fn duration(value: u128) -> Duration {
        Duration::new(value)
    }

    fn dependency(value: u64) -> DependencyId {
        DependencyId::new(value)
    }

    fn reservation(value: u64) -> ReservationId {
        ReservationId::new(value)
    }

    #[test]
    fn error_kind_is_stable() {
        let error = SchedulingError::MissingDuration {
            operation: operation(7),
        };

        assert_eq!(
            error.kind(),
            SchedulingErrorKind::MissingDuration
        );

        assert_eq!(
            error.code(),
            "missing_duration"
        );
    }

    #[test]
    fn operation_context_is_recoverable_without_string_parsing() {
        let id = operation(42);

        let error = SchedulingError::MissingDuration {
            operation: id,
        };

        assert_eq!(error.operation_id(), Some(id));
    }

    #[test]
    fn resource_context_is_recoverable_without_string_parsing() {
        let id = resource(17);

        let error = SchedulingError::ResourceUnavailable {
            resource: id,
            operation: Some(operation(3)),
            requested_start: Some(time(100)),
            requested_duration: Some(duration(25)),
            reason: String::from("resource is reserved"),
        };

        assert_eq!(error.resource_id(), Some(id));
        assert_eq!(error.operation_id(), Some(operation(3)));
    }

    #[test]
    fn logical_qubit_context_uses_canonical_ir_identity() {
        let id = logical_qubit(9);

        let error = SchedulingError::InvalidOperation {
            operation: Some(operation(1)),
            qubit: Some(id),
            physical_qubit: None,
            reason: String::from("invalid operand"),
        };

        assert_eq!(error.qubit_id(), Some(id));
    }

    #[test]
    fn physical_qubit_context_uses_canonical_ir_identity() {
        let id = physical_qubit(12);

        let error = SchedulingError::ConstraintViolation {
            operation: Some(operation(1)),
            resource: None,
            qubit: None,
            physical_qubit: Some(id),
            constraint: String::from("physical_qubit_exclusivity"),
            reason: String::from("physical qubit is busy"),
        };

        assert_eq!(error.physical_qubit_id(), Some(id));
    }

    #[test]
    fn invalid_input_is_classified_correctly() {
        let error = SchedulingError::invalid_input("missing target");

        assert!(error.is_invalid_input());
        assert!(!error.is_environmental());
        assert!(!error.is_retryable());
    }

    #[test]
    fn resource_conflict_is_retryable() {
        let error = SchedulingError::ResourceConflict {
            resource: resource(2),
            operation: Some(operation(3)),
            conflicting_operation: Some(operation(4)),
            conflicting_reservation: Some(reservation(5)),
            requested_start: Some(time(100)),
            conflicting_start: Some(time(80)),
            reason: String::from("exclusive resource overlap"),
        };

        assert!(error.is_environmental());
        assert!(error.is_retryable());
        assert_eq!(
            error.operation_id(),
            Some(operation(3))
        );
        assert_eq!(
            error.resource_id(),
            Some(resource(2))
        );
    }

    #[test]
    fn cycle_detection_is_not_environmental() {
        let error = SchedulingError::CycleDetected {
            operation: Some(operation(10)),
            dependency: Some(dependency(11)),
            cycle_size: Some(4),
        };

        assert!(error.is_invalid_input());
        assert!(!error.is_environmental());
        assert!(!error.is_retryable());
    }

    #[test]
    fn deadline_error_preserves_schedule_context() {
        let error = SchedulingError::DeadlineExceeded {
            operation: Some(operation(4)),
            schedule: Some(schedule(9)),
            deadline: time(1000),
            completion: Some(time(1200)),
        };

        assert_eq!(
            error.operation_id(),
            Some(operation(4))
        );
        assert_eq!(
            error.kind(),
            SchedulingErrorKind::DeadlineExceeded
        );
    }

    #[test]
    fn cancellation_is_explicit() {
        let error =
            SchedulingError::cancelled_with_reason("caller requested cancellation");

        assert!(error.is_cancelled());
        assert_eq!(
            error.kind(),
            SchedulingErrorKind::Cancelled
        );
    }

    #[test]
    fn internal_error_is_not_reported_as_input_error() {
        let error = SchedulingError::Internal {
            invariant: String::from("reservation_order"),
            reason: String::from("reservation end precedes start"),
        };

        assert!(error.is_internal());
        assert!(!error.is_invalid_input());
        assert!(!error.is_retryable());
    }

    #[test]
    fn display_is_human_readable() {
        let error = SchedulingError::ResourceUnavailable {
            resource: resource(5),
            operation: Some(operation(7)),
            requested_start: Some(time(100)),
            requested_duration: Some(duration(20)),
            reason: String::from("maintenance window"),
        };

        let text = error.to_string();

        assert!(text.contains("resource"));
        assert!(text.contains("operation"));
        assert!(text.contains("maintenance window"));
    }

    #[test]
    fn serialization_error_has_machine_category() {
        let error = SchedulingError::SerializationError {
            operation: SerializationOperation::Decode,
            schema: Some(String::from("zamani.schedule.v1")),
            reason: String::from("invalid interval"),
        };

        assert_eq!(
            error.kind(),
            SchedulingErrorKind::SerializationError
        );
        assert!(!error.is_retryable());
    }

    #[test]
    fn plugin_failure_has_machine_category() {
        let error = SchedulingError::PluginError {
            plugin: String::from("example.scheduler"),
            operation: PluginOperation::Schedule,
            reason: String::from("plugin rejected target"),
        };

        assert_eq!(
            error.kind(),
            SchedulingErrorKind::PluginError
        );
    }

    #[test]
    fn result_alias_uses_canonical_error() {
        fn fail() -> SchedulingResult<()> {
            Err(SchedulingError::invalid_input("test"))
        }

        assert!(fail().is_err());
    }
}