
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Zamani distributed-computing error boundary.
//!
//! Path: `src/distributed/error.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Defines the canonical error vocabulary for distributed planning,
//! capability and resource feasibility, placement, scheduling, routing,
//! execution, transport, membership, security, cancellation, and recovery.
//!
//! This module describes failures. It does not perform network operations,
//! retry requests, allocate resources, recover execution, select hardware,
//! or alter program semantics.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - DistributedError and DistributedErrorKind.
//! - DistributedErrorSeverity and RetryAdvice.
//! - DeliveryState for classifying uncertain remote-operation outcomes.
//! - DistributedResult<T>.
//! - Stable machine-readable error codes.
//! - Context and external-error cause preservation.
//!
//! This module does not own:
//! - Source syntax, AST, semantic analysis, or source diagnostics.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or other domain IRs.
//! - Backend selection, physical resource allocation, or device discovery.
//! - Network protocol implementation, scheduling, or placement algorithms.
//! - Automatic retries, checkpointing, recovery, or fallback execution.
//!
//! # Portability and scalability
//!
//! Errors distinguish semantic invalidity from resource or capability
//! infeasibility. A computation that cannot run on the currently available
//! resources is not necessarily an invalid program.
//!
//! No fixed node, task, worker, device, message, memory, or cluster ceiling
//! is imposed here. Concrete resource budgets belong to validated
//! configuration and runtime policy. Actual representation and resource
//! limitations must be reported honestly.
//!
//! # Retry safety
//!
//! An error's retry advice is guidance, not permission to repeat a side effect.
//! In particular, a lost response does not prove that a remote operation
//! failed to execute. Callers must consult operation semantics, idempotency,
//! deadlines, and durable execution records before retrying.
//!
//! # Integration
//!
//! - `src/distributed/mod.rs`: declares and exports this module.
//! - `src/distributed/config.rs`: maps configuration validation failures.
//! - `src/distributed/capabilities.rs` and `requirements.rs`: report
//!   capability and resource infeasibility.
//! - `src/distributed/planning/`: reports invalid or infeasible plans.
//! - `src/distributed/transport/`: maps protocol and transport failures.
//! - `src/distributed/execution/`: propagates execution, cancellation,
//!   and outcome uncertainty.
//! - `src/distributed/resilience/`: consumes retry advice without treating
//!   it as an unconditional retry command.
//! - `src/runtime/distributed.rs`: replaces string-only failures through
//!   an explicit compatibility migration.
//! - `src/runtime/omniversal_kernel.rs`: propagates typed failures.
//! - `src/error_reporting.rs`: may translate these errors into compiler or
//!   runtime diagnostics without taking ownership of their classification.
//!
//! No caller should parse Display text for control flow.

use core::fmt;
use std::error::Error;

/// Canonical result type for distributed-subsystem operations.
pub type DistributedResult<T> = Result<T, DistributedError>;

/// Stable classification of a distributed-computing failure.
///
/// Use `code()` or `kind()` for programmatic decisions. Human-readable error
/// messages are explanatory text and are not a machine-readable interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistributedErrorKind {
    /// Caller-provided arguments or metadata are invalid.
    InvalidInput,

    /// Runtime or planner configuration is invalid.
    InvalidConfiguration,

    /// The requested program violates an established semantic contract.
    SemanticInvalid,

    /// A required execution capability cannot be provided.
    CapabilityUnsatisfied,

    /// A required resource or configured operational budget is unavailable.
    ResourceUnsatisfied,

    /// The requested computation cannot be represented by the selected
    /// realization without violating its requirements.
    Unrepresentable,

    /// Planning could not produce a valid execution plan.
    PlanningFailed,

    /// The selected placement or schedule violates a required condition.
    PlacementFailed,

    /// No valid schedule satisfies the specified dependencies or constraints.
    SchedulingFailed,

    /// A required communication route cannot be realized.
    RoutingFailed,

    /// A requested feature or provider operation is not implemented or
    /// is explicitly unsupported by the selected provider.
    UnsupportedOperation,

    /// A transport operation failed.
    TransportFailed,

    /// A message could not be encoded or decoded.
    SerializationFailed,

    /// A protocol, schema, or interface version is incompatible.
    ProtocolIncompatible,

    /// A remote participant or service could not be identified or verified.
    AuthenticationFailed,

    /// An authenticated identity is not permitted to perform an operation.
    AuthorizationDenied,

    /// A remote operation's completion status cannot be established safely.
    OperationOutcomeUnknown,

    /// An execution operation failed.
    ExecutionFailed,

    /// An operation exceeded its applicable deadline.
    DeadlineExceeded,

    /// An operation was cancelled.
    Cancelled,

    /// An execution or state transition conflicts with existing state.
    Conflict,

    /// Required execution state cannot be transferred or migrated safely.
    StateTransferFailed,

    /// Recovery or reconciliation failed.
    RecoveryFailed,

    /// A participant, service, or runtime is unavailable.
    Unavailable,

    /// The runtime is shutting down or has already shut down.
    RuntimeUnavailable,

    /// A received message or external value violates a protocol contract.
    InvalidMessage,

    /// An internal invariant was violated.
    InternalInvariantViolation,

    /// An unexpected failure occurred in an external subsystem.
    ExternalFailure,

    /// Additional context was added to an existing error.
    Context,
}

impl DistributedErrorKind {
    /// Returns the stable machine-readable code for this error kind.
    ///
    /// These codes are public compatibility identifiers. Changing an
    /// existing code requires a compatibility review.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "distributed.invalid_input",
            Self::InvalidConfiguration => "distributed.invalid_configuration",
            Self::SemanticInvalid => "distributed.semantic_invalid",
            Self::CapabilityUnsatisfied => "distributed.capability_unsatisfied",
            Self::ResourceUnsatisfied => "distributed.resource_unsatisfied",
            Self::Unrepresentable => "distributed.unrepresentable",
            Self::PlanningFailed => "distributed.planning_failed",
            Self::PlacementFailed => "distributed.placement_failed",
            Self::SchedulingFailed => "distributed.scheduling_failed",
            Self::RoutingFailed => "distributed.routing_failed",
            Self::UnsupportedOperation => "distributed.unsupported_operation",
            Self::TransportFailed => "distributed.transport_failed",
            Self::SerializationFailed => "distributed.serialization_failed",
            Self::ProtocolIncompatible => "distributed.protocol_incompatible",
            Self::AuthenticationFailed => "distributed.authentication_failed",
            Self::AuthorizationDenied => "distributed.authorization_denied",
            Self::OperationOutcomeUnknown => "distributed.operation_outcome_unknown",
            Self::ExecutionFailed => "distributed.execution_failed",
            Self::DeadlineExceeded => "distributed.deadline_exceeded",
            Self::Cancelled => "distributed.cancelled",
            Self::Conflict => "distributed.conflict",
            Self::StateTransferFailed => "distributed.state_transfer_failed",
            Self::RecoveryFailed => "distributed.recovery_failed",
            Self::Unavailable => "distributed.unavailable",
            Self::RuntimeUnavailable => "distributed.runtime_unavailable",
            Self::InvalidMessage => "distributed.invalid_message",
            Self::InternalInvariantViolation => {
                "distributed.internal_invariant_violation"
            }
            Self::ExternalFailure => "distributed.external_failure",
            Self::Context => "distributed.context",
        }
    }
}

/// Severity category for diagnostics and operational reporting.
///
/// Severity does not determine whether an error is retryable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistributedErrorSeverity {
    /// Invalid caller input or configuration.
    Input,

    /// A program or requested realization violates a required contract.
    Validation,

    /// A required capability or resource is unavailable.
    Feasibility,

    /// A failure occurred while planning or executing a workload.
    Execution,

    /// A communication, membership, or external-service failure occurred.
    Operational,

    /// A security boundary rejected an operation.
    Security,

    /// An operation could not be completed within its deadline or was
    /// cancelled.
    Cancellation,

    /// An internal invariant or unexpected implementation failure occurred.
    Internal,
}

/// Delivery knowledge for a remote operation.
///
/// This classification describes what the caller knows about delivery and
/// execution. It must be supplied by the transport or execution layer when
/// that information is available.
///
/// `Unknown` is deliberately conservative: absence of evidence that an
/// operation executed is not evidence that it did not execute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryState {
    /// The operation was definitely not sent to the remote execution boundary.
    NotSent,

    /// The remote boundary explicitly rejected the operation before execution.
    RejectedBeforeExecution,

    /// The operation may have been delivered or executed.
    MayHaveExecuted,

    /// The operation may have completed, but its response was lost.
    ResponseLost,

    /// The caller cannot establish the delivery or execution state.
    Unknown,
}

/// Conservative guidance for retry and recovery consumers.
///
/// This enum does not execute retries and does not establish idempotency.
/// A retrying subsystem must still apply its own policy, deadline, cancellation,
/// security, side-effect, and durable-deduplication rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RetryAdvice {
    /// Retrying the same request is not justified by this error.
    Never,

    /// Re-evaluate capabilities, resources, or the plan before attempting
    /// another realization.
    Replan,

    /// Retry only after the operation's idempotency and side-effect semantics
    /// have been established by the caller.
    OnlyIfIdempotent,

    /// A transient failure may have occurred. The caller must apply its
    /// configured retry policy and verify the operation is safe to repeat.
    PossiblyAfterBackoff,

    /// Reconcile execution state or consult durable operation records before
    /// deciding whether to retry.
    ReconcileOutcomeFirst,
}

/// Canonical distributed-computing error.
///
/// The enum intentionally does not implement `Clone` or `PartialEq` because
/// external errors may own non-cloneable source objects. Use `kind()`, `code()`
/// and structured fields for classification instead of comparing whole errors
/// or parsing their display messages.
#[derive(Debug)]
pub enum DistributedError {
    /// Caller input or distributed metadata is invalid.
    InvalidInput {
        /// Explanation of the invalid input.
        message: String,
    },

    /// Configuration is invalid or inconsistent.
    InvalidConfiguration {
        /// Configuration field or policy identifier.
        setting: String,

        /// Explanation of the configuration failure.
        message: String,
    },

    /// A source-level or semantic contract is violated.
    SemanticInvalid {
        /// Contract or semantic rule identifier.
        rule: String,

        /// Explanation of the violation.
        message: String,
    },

    /// A required capability cannot be supplied by the available providers.
    CapabilityUnsatisfied {
        /// Capability identifier supplied by the relevant registry.
        capability: String,

        /// Optional explanation of the failed feasibility check.
        explanation: Option<String>,
    },

    /// A resource requirement cannot be satisfied.
    ///
    /// Quantities may be unknown. `None` must not be replaced with a guessed
    /// value or an invented universal maximum.
    ResourceUnsatisfied {
        /// Resource identifier, such as `memory.bytes` or `compute.units`.
        resource: String,

        /// Requested amount, when known.
        requested: Option<u128>,

        /// Available or configured amount, when known.
        available: Option<u128>,

        /// Unit associated with the amounts, when known.
        unit: Option<String>,

        /// Whether the reported limit comes from an explicit operational
        /// policy rather than a semantic restriction.
        configured_budget: bool,
    },

    /// The selected realization cannot represent the requested computation.
    Unrepresentable {
        /// Representation, provider, or target identifier.
        realization: String,

        /// Explanation of the representation failure.
        message: String,
    },

    /// Planning failed to produce a valid execution plan.
    PlanningFailed {
        /// Planning stage or policy identifier.
        stage: String,

        /// Explanation of the failure.
        message: String,
    },

    /// Placement cannot satisfy the required constraints.
    PlacementFailed {
        /// Logical task or placement subject, when known.
        subject: Option<String>,

        /// Explanation of the placement failure.
        message: String,
    },

    /// Scheduling could not produce a valid schedule.
    SchedulingFailed {
        /// Scheduling stage or policy identifier.
        stage: String,

        /// Explanation of the scheduling failure.
        message: String,
    },

    /// A required communication route cannot be realized.
    RoutingFailed {
        /// Logical source identifier, when known.
        source: Option<String>,

        /// Logical destination identifier, when known.
        destination: Option<String>,

        /// Explanation of the routing failure.
        message: String,
    },

    /// The selected provider does not support the requested operation.
    UnsupportedOperation {
        /// Stable operation identifier.
        operation: String,

        /// Provider or domain identifier, when known.
        provider: Option<String>,

        /// Explanation of the limitation.
        message: String,
    },

    /// A transport operation failed.
    TransportFailed {
        /// Transport operation, such as `send`, `receive`, or `connect`.
        operation: String,

        /// Best available knowledge of delivery or execution.
        delivery: DeliveryState,

        /// Explanation of the transport failure.
        message: String,
    },

    /// Message encoding or decoding failed.
    SerializationFailed {
        /// Operation or format identifier.
        operation: String,

        /// Explanation of the serialization failure.
        message: String,
    },

    /// Protocol or schema compatibility failed.
    ProtocolIncompatible {
        /// Protocol, schema, or interface identifier.
        subject: String,

        /// Expected version or compatibility requirement.
        expected: String,

        /// Observed version or value.
        actual: String,
    },

    /// Peer identity could not be authenticated.
    AuthenticationFailed {
        /// Peer or identity identifier, when safe to disclose.
        peer: Option<String>,

        /// Explanation that does not expose credentials or secrets.
        message: String,
    },

    /// An authenticated peer is not authorized for an operation.
    AuthorizationDenied {
        /// Principal identifier, when safe to disclose.
        principal: Option<String>,

        /// Requested operation or policy identifier.
        operation: String,
    },

    /// The outcome of a potentially side-effecting remote operation is unknown.
    OperationOutcomeUnknown {
        /// Stable operation or idempotency key, when available.
        operation_id: Option<String>,

        /// Best available delivery classification.
        delivery: DeliveryState,

        /// Explanation of the uncertainty.
        message: String,
    },

    /// Execution failed.
    ExecutionFailed {
        /// Task or execution identifier, when known.
        execution_id: Option<String>,

        /// Explanation of the failure.
        message: String,
    },

    /// An operation exceeded its applicable deadline.
    DeadlineExceeded {
        /// Operation or execution identifier.
        operation: String,

        /// Whether execution may have started.
        delivery: DeliveryState,
    },

    /// An operation was cancelled.
    Cancelled {
        /// Operation or execution identifier.
        operation: Option<String>,

        /// Cancellation origin or explanation.
        reason: String,
    },

    /// A requested state transition conflicts with existing state.
    Conflict {
        /// State, resource, operation, or version identifier.
        subject: String,

        /// Explanation of the conflict.
        message: String,
    },

    /// State transfer or migration could not be completed safely.
    StateTransferFailed {
        /// Logical state identifier, when known.
        state_id: Option<String>,

        /// Explanation of the transfer failure.
        message: String,
    },

    /// Recovery or reconciliation failed.
    RecoveryFailed {
        /// Recovery stage or execution identifier.
        subject: String,

        /// Explanation of the failure.
        message: String,
    },

    /// A participant or service is unavailable.
    Unavailable {
        /// Participant, service, or provider identifier.
        subject: String,

        /// Explanation of the unavailability.
        message: String,
    },

    /// The distributed runtime is not accepting operations.
    RuntimeUnavailable {
        /// Runtime state or lifecycle explanation.
        message: String,
    },

    /// A message violates the established protocol contract.
    InvalidMessage {
        /// Message type or protocol field.
        subject: String,

        /// Explanation of the validation failure.
        message: String,
    },

    /// An internal invariant was violated.
    InternalInvariantViolation {
        /// Invariant identifier.
        invariant: String,

        /// Explanation of the violation.
        message: String,
    },

    /// A failure originating in another subsystem.
    ///
    /// The boxed source preserves the original error chain. Its display text
    /// is explanatory only; callers should classify errors before converting
    /// them into this variant whenever a more specific distributed variant
    /// applies.
    ExternalFailure {
        /// Name of the originating subsystem.
        subsystem: String,

        /// Additional explanatory context.
        message: String,

        /// Original error, preserved when one is available.
        source: Option<Box<dyn Error + Send + Sync + 'static>>,
    },

    /// Additional context around an existing distributed error.
    ///
    /// The original classification is preserved by `kind()` and `code()`.
    Context {
        /// Context added by the caller.
        context: String,

        /// Original distributed error.
        source: Box<DistributedError>,
    },
}

impl DistributedError {
    /// Creates an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Creates a configuration error.
    #[must_use]
    pub fn invalid_configuration(
        setting: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidConfiguration {
            setting: setting.into(),
            message: message.into(),
        }
    }

    /// Creates a semantic-contract violation.
    #[must_use]
    pub fn semantic_invalid(
        rule: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::SemanticInvalid {
            rule: rule.into(),
            message: message.into(),
        }
    }

    /// Creates a capability-infeasibility error.
    #[must_use]
    pub fn capability_unsatisfied(
        capability: impl Into<String>,
        explanation: Option<String>,
    ) -> Self {
        Self::CapabilityUnsatisfied {
            capability: capability.into(),
            explanation,
        }
    }

    /// Creates a resource-infeasibility error.
    ///
    /// Unknown quantities remain `None`. Resource limits describe actual
    /// availability or explicit policy, not universal language ceilings.
    #[must_use]
    pub fn resource_unsatisfied(
        resource: impl Into<String>,
        requested: Option<u128>,
        available: Option<u128>,
        unit: Option<String>,
        configured_budget: bool,
    ) -> Self {
        Self::ResourceUnsatisfied {
            resource: resource.into(),
            requested,
            available,
            unit,
            configured_budget,
        }
    }

    /// Creates a planning failure.
    #[must_use]
    pub fn planning_failed(
        stage: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::PlanningFailed {
            stage: stage.into(),
            message: message.into(),
        }
    }

    /// Creates a transport failure with explicit delivery knowledge.
    #[must_use]
    pub fn transport_failed(
        operation: impl Into<String>,
        delivery: DeliveryState,
        message: impl Into<String>,
    ) -> Self {
        Self::TransportFailed {
            operation: operation.into(),
            delivery,
            message: message.into(),
        }
    }

    /// Creates an error for a remote operation with an unknown outcome.
    #[must_use]
    pub fn outcome_unknown(
        operation_id: Option<String>,
        delivery: DeliveryState,
        message: impl Into<String>,
    ) -> Self {
        Self::OperationOutcomeUnknown {
            operation_id,
            delivery,
            message: message.into(),
        }
    }

    /// Wraps an existing error with additional context.
    ///
    /// The original kind, stable code, retry advice, and source chain remain
    /// accessible through the wrapper.
    #[must_use]
    pub fn with_context(self, context: impl Into<String>) -> Self {
        Self::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Wraps an error from another subsystem while preserving its source.
    #[must_use]
    pub fn external<E>(
        subsystem: impl Into<String>,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::ExternalFailure {
            subsystem: subsystem.into(),
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates an external failure without an underlying error object.
    #[must_use]
    pub fn external_message(
        subsystem: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ExternalFailure {
            subsystem: subsystem.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Returns the underlying non-context error kind.
    #[must_use]
    pub fn kind(&self) -> DistributedErrorKind {
        match self {
            Self::InvalidInput { .. } => DistributedErrorKind::InvalidInput,
            Self::InvalidConfiguration { .. } => {
                DistributedErrorKind::InvalidConfiguration
            }
            Self::SemanticInvalid { .. } => DistributedErrorKind::SemanticInvalid,
            Self::CapabilityUnsatisfied { .. } => {
                DistributedErrorKind::CapabilityUnsatisfied
            }
            Self::ResourceUnsatisfied { .. } => {
                DistributedErrorKind::ResourceUnsatisfied
            }
            Self::Unrepresentable { .. } => DistributedErrorKind::Unrepresentable,
            Self::PlanningFailed { .. } => DistributedErrorKind::PlanningFailed,
            Self::PlacementFailed { .. } => DistributedErrorKind::PlacementFailed,
            Self::SchedulingFailed { .. } => DistributedErrorKind::SchedulingFailed,
            Self::RoutingFailed { .. } => DistributedErrorKind::RoutingFailed,
            Self::UnsupportedOperation { .. } => {
                DistributedErrorKind::UnsupportedOperation
            }
            Self::TransportFailed { .. } => DistributedErrorKind::TransportFailed,
            Self::SerializationFailed { .. } => {
                DistributedErrorKind::SerializationFailed
            }
            Self::ProtocolIncompatible { .. } => {
                DistributedErrorKind::ProtocolIncompatible
            }
            Self::AuthenticationFailed { .. } => {
                DistributedErrorKind::AuthenticationFailed
            }
            Self::AuthorizationDenied { .. } => {
                DistributedErrorKind::AuthorizationDenied
            }
            Self::OperationOutcomeUnknown { .. } => {
                DistributedErrorKind::OperationOutcomeUnknown
            }
            Self::ExecutionFailed { .. } => DistributedErrorKind::ExecutionFailed,
            Self::DeadlineExceeded { .. } => DistributedErrorKind::DeadlineExceeded,
            Self::Cancelled { .. } => DistributedErrorKind::Cancelled,
            Self::Conflict { .. } => DistributedErrorKind::Conflict,
            Self::StateTransferFailed { .. } => {
                DistributedErrorKind::StateTransferFailed
            }
            Self::RecoveryFailed { .. } => DistributedErrorKind::RecoveryFailed,
            Self::Unavailable { .. } => DistributedErrorKind::Unavailable,
            Self::RuntimeUnavailable { .. } => {
                DistributedErrorKind::RuntimeUnavailable
            }
            Self::InvalidMessage { .. } => DistributedErrorKind::InvalidMessage,
            Self::InternalInvariantViolation { .. } => {
                DistributedErrorKind::InternalInvariantViolation
            }
            Self::ExternalFailure { .. } => DistributedErrorKind::ExternalFailure,
            Self::Context { source, .. } => source.kind(),
        }
    }

    /// Returns the stable machine-readable error code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        self.kind().as_str()
    }

    /// Returns the diagnostic severity of this failure.
    #[must_use]
    pub fn severity(&self) -> DistributedErrorSeverity {
        match self.kind() {
            DistributedErrorKind::InvalidInput
            | DistributedErrorKind::InvalidConfiguration => {
                DistributedErrorSeverity::Input
            }

            DistributedErrorKind::SemanticInvalid
            | DistributedErrorKind::Unrepresentable
            | DistributedErrorKind::UnsupportedOperation
            | DistributedErrorKind::InvalidMessage
            | DistributedErrorKind::ProtocolIncompatible => {
                DistributedErrorSeverity::Validation
            }

            DistributedErrorKind::CapabilityUnsatisfied
            | DistributedErrorKind::ResourceUnsatisfied
            | DistributedErrorKind::PlacementFailed
            | DistributedErrorKind::SchedulingFailed
            | DistributedErrorKind::RoutingFailed => {
                DistributedErrorSeverity::Feasibility
            }

            DistributedErrorKind::PlanningFailed
            | DistributedErrorKind::ExecutionFailed
            | DistributedErrorKind::OperationOutcomeUnknown
            | DistributedErrorKind::StateTransferFailed
            | DistributedErrorKind::RecoveryFailed
            | DistributedErrorKind::Conflict => {
                DistributedErrorSeverity::Execution
            }

            DistributedErrorKind::TransportFailed
            | DistributedErrorKind::SerializationFailed
            | DistributedErrorKind::Unavailable
            | DistributedErrorKind::RuntimeUnavailable
            | DistributedErrorKind::ExternalFailure => {
                DistributedErrorSeverity::Operational
            }

            DistributedErrorKind::AuthenticationFailed
            | DistributedErrorKind::AuthorizationDenied => {
                DistributedErrorSeverity::Security
            }

            DistributedErrorKind::DeadlineExceeded
            | DistributedErrorKind::Cancelled => {
                DistributedErrorSeverity::Cancellation
            }

            DistributedErrorKind::InternalInvariantViolation => {
                DistributedErrorSeverity::Internal
            }

            // `kind()` unwraps contextual errors. This branch is defensive.
            DistributedErrorKind::Context => DistributedErrorSeverity::Internal,
        }
    }

    /// Returns conservative retry/recovery guidance.
    ///
    /// This method never authorizes an automatic retry. The caller must also
    /// check idempotency, side effects, deadlines, cancellation, and its
    /// configured resilience policy.
    #[must_use]
    pub fn retry_advice(&self) -> RetryAdvice {
        match self {
            Self::Context { source, .. } => source.retry_advice(),

            Self::TransportFailed { delivery, .. }
            | Self::DeadlineExceeded { delivery, .. } => match delivery {
                DeliveryState::NotSent | DeliveryState::RejectedBeforeExecution => {
                    RetryAdvice::PossiblyAfterBackoff
                }
                DeliveryState::MayHaveExecuted
                | DeliveryState::ResponseLost
                | DeliveryState::Unknown => RetryAdvice::ReconcileOutcomeFirst,
            },

            Self::OperationOutcomeUnknown { .. } => {
                RetryAdvice::ReconcileOutcomeFirst
            }

            Self::CapabilityUnsatisfied { .. }
            | Self::ResourceUnsatisfied { .. }
            | Self::Unrepresentable { .. }
            | Self::PlanningFailed { .. }
            | Self::PlacementFailed { .. }
            | Self::SchedulingFailed { .. }
            | Self::RoutingFailed { .. } => RetryAdvice::Replan,

            Self::Unavailable { .. } => RetryAdvice::PossiblyAfterBackoff,

            Self::ExecutionFailed { .. }
            | Self::StateTransferFailed { .. }
            | Self::RecoveryFailed { .. }
            | Self::Conflict { .. } => RetryAdvice::OnlyIfIdempotent,

            Self::InvalidInput { .. }
            | Self::InvalidConfiguration { .. }
            | Self::SemanticInvalid { .. }
            | Self::UnsupportedOperation { .. }
            | Self::SerializationFailed { .. }
            | Self::ProtocolIncompatible { .. }
            | Self::AuthenticationFailed { .. }
            | Self::AuthorizationDenied { .. }
            | Self::Cancelled { .. }
            | Self::RuntimeUnavailable { .. }
            | Self::InvalidMessage { .. }
            | Self::InternalInvariantViolation { .. }
            | Self::ExternalFailure { .. } => RetryAdvice::Never,
        }
    }

    /// Returns whether this error represents a semantic invalidity rather
    /// than a realization or operational failure.
    #[must_use]
    pub fn is_semantic_failure(&self) -> bool {
        matches!(self.kind(), DistributedErrorKind::SemanticInvalid)
    }

    /// Returns whether this error reports an unsatisfied capability or
    /// resource requirement.
    #[must_use]
    pub fn is_feasibility_failure(&self) -> bool {
        matches!(
            self.kind(),
            DistributedErrorKind::CapabilityUnsatisfied
                | DistributedErrorKind::ResourceUnsatisfied
                | DistributedErrorKind::Unrepresentable
                | DistributedErrorKind::PlacementFailed
                | DistributedErrorKind::SchedulingFailed
                | DistributedErrorKind::RoutingFailed
        )
    }

    /// Returns the underlying distributed error, transparently traversing
    /// context wrappers.
    #[must_use]
    pub fn root(&self) -> &DistributedError {
        match self {
            Self::Context { source, .. } => source.root(),
            _ => self,
        }
    }
}

impl fmt::Display for DistributedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message } => {
                write!(formatter, "invalid distributed input: {message}")
            }

            Self::InvalidConfiguration { setting, message } => {
                write!(formatter, "invalid configuration for {setting}: {message}")
            }

            Self::SemanticInvalid { rule, message } => {
                write!(formatter, "distributed semantic rule {rule} violated: {message}")
            }

            Self::CapabilityUnsatisfied {
                capability,
                explanation,
            } => {
                write!(formatter, "required capability is unavailable: {capability}")?;
                if let Some(explanation) = explanation {
                    write!(formatter, ": {explanation}")?;
                }
                Ok(())
            }

            Self::ResourceUnsatisfied {
                resource,
                requested,
                available,
                unit,
                configured_budget,
            } => {
                write!(formatter, "resource requirement cannot be satisfied: {resource}")?;

                if let Some(requested) = requested {
                    write!(formatter, "; requested {requested}")?;
                }

                if let Some(available) = available {
                    write!(formatter, "; available {available}")?;
                }

                if let Some(unit) = unit {
                    write!(formatter, " {unit}")?;
                }

                if *configured_budget {
                    write!(formatter, "; an explicit operational budget applies")?;
                }

                Ok(())
            }

            Self::Unrepresentable {
                realization,
                message,
            } => write!(
                formatter,
                "computation is not representable by {realization}: {message}"
            ),

            Self::PlanningFailed { stage, message } => {
                write!(formatter, "distributed planning failed at {stage}: {message}")
            }

            Self::PlacementFailed { subject, message } => {
                write!(formatter, "distributed placement failed")?;
                if let Some(subject) = subject {
                    write!(formatter, " for {subject}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::SchedulingFailed { stage, message } => {
                write!(formatter, "distributed scheduling failed at {stage}: {message}")
            }

            Self::RoutingFailed {
                source,
                destination,
                message,
            } => {
                write!(formatter, "distributed routing failed")?;
                if let Some(source) = source {
                    write!(formatter, " from {source}")?;
                }
                if let Some(destination) = destination {
                    write!(formatter, " to {destination}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::UnsupportedOperation {
                operation,
                provider,
                message,
            } => {
                write!(formatter, "unsupported distributed operation {operation}")?;
                if let Some(provider) = provider {
                    write!(formatter, " by {provider}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::TransportFailed {
                operation,
                delivery,
                message,
            } => write!(
                formatter,
                "transport operation {operation} failed ({delivery:?}): {message}"
            ),

            Self::SerializationFailed { operation, message } => {
                write!(formatter, "distributed serialization failed during {operation}: {message}")
            }

            Self::ProtocolIncompatible {
                subject,
                expected,
                actual,
            } => write!(
                formatter,
                "incompatible distributed protocol for {subject}: expected {expected}, observed {actual}"
            ),

            Self::AuthenticationFailed { peer, message } => {
                write!(formatter, "distributed peer authentication failed")?;
                if let Some(peer) = peer {
                    write!(formatter, " for {peer}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::AuthorizationDenied {
                principal,
                operation,
            } => {
                write!(formatter, "distributed authorization denied for operation {operation}")?;
                if let Some(principal) = principal {
                    write!(formatter, " (principal {principal})")?;
                }
                Ok(())
            }

            Self::OperationOutcomeUnknown {
                operation_id,
                delivery,
                message,
            } => {
                write!(
                    formatter,
                    "distributed operation outcome is unknown ({delivery:?})"
                )?;
                if let Some(operation_id) = operation_id {
                    write!(formatter, " for operation {operation_id}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::ExecutionFailed {
                execution_id,
                message,
            } => {
                write!(formatter, "distributed execution failed")?;
                if let Some(execution_id) = execution_id {
                    write!(formatter, " for {execution_id}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::DeadlineExceeded { operation, delivery } => write!(
                formatter,
                "deadline exceeded for {operation}; delivery state: {delivery:?}"
            ),

            Self::Cancelled { operation, reason } => {
                write!(formatter, "distributed operation cancelled")?;
                if let Some(operation) = operation {
                    write!(formatter, " ({operation})")?;
                }
                write!(formatter, ": {reason}")
            }

            Self::Conflict { subject, message } => {
                write!(formatter, "distributed state conflict for {subject}: {message}")
            }

            Self::StateTransferFailed { state_id, message } => {
                write!(formatter, "distributed state transfer failed")?;
                if let Some(state_id) = state_id {
                    write!(formatter, " for {state_id}")?;
                }
                write!(formatter, ": {message}")
            }

            Self::RecoveryFailed { subject, message } => {
                write!(formatter, "distributed recovery failed for {subject}: {message}")
            }

            Self::Unavailable { subject, message } => {
                write!(formatter, "distributed resource {subject} is unavailable: {message}")
            }

            Self::RuntimeUnavailable { message } => {
                write!(formatter, "distributed runtime unavailable: {message}")
            }

            Self::InvalidMessage { subject, message } => {
                write!(formatter, "invalid distributed message ({subject}): {message}")
            }

            Self::InternalInvariantViolation { invariant, message } => {
                write!(formatter, "distributed invariant {invariant} violated: {message}")
            }

            Self::ExternalFailure {
                subsystem,
                message,
                source,
            } => {
                write!(formatter, "failure in external subsystem {subsystem}: {message}")?;
                if let Some(source) = source {
                    write!(formatter, " (caused by: {source})")?;
                }
                Ok(())
            }

            Self::Context { context, source } => {
                write!(formatter, "{context}: {source}")
            }
        }
    }
}

impl Error for DistributedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ExternalFailure {
                source: Some(source),
                ..
            } => Some(source.as_ref()),

            Self::Context { source, .. } => Some(source.as_ref()),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn error_codes_are_stable_and_namespaced() {
        let cases = [
            (
                DistributedError::invalid_input("bad input"),
                "distributed.invalid_input",
            ),
            (
                DistributedError::capability_unsatisfied("quantum.execution", None),
                "distributed.capability_unsatisfied",
            ),
            (
                DistributedError::resource_unsatisfied(
                    "memory.bytes",
                    Some(100),
                    Some(50),
                    Some("bytes".to_owned()),
                    true,
                ),
                "distributed.resource_unsatisfied",
            ),
            (
                DistributedError::transport_failed(
                    "send",
                    DeliveryState::NotSent,
                    "connection unavailable",
                ),
                "distributed.transport_failed",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.code(), expected);
            assert!(error.code().starts_with("distributed."));
        }
    }

    #[test]
    fn contextual_wrapping_preserves_kind_and_code() {
        let error = DistributedError::invalid_input("invalid task")
            .with_context("while validating a distributed request");

        assert_eq!(error.kind(), DistributedErrorKind::InvalidInput);
        assert_eq!(error.code(), "distributed.invalid_input");
        assert!(error.to_string().contains("while validating"));
        assert!(error.root().to_string().contains("invalid task"));
    }

    #[test]
    fn contextual_wrapping_preserves_retry_advice() {
        let error = DistributedError::transport_failed(
            "send",
            DeliveryState::ResponseLost,
            "response was lost",
        )
        .with_context("while invoking a remote service");

        assert_eq!(
            error.retry_advice(),
            RetryAdvice::ReconcileOutcomeFirst
        );
    }

    #[test]
    fn unknown_remote_outcome_requires_reconciliation() {
        let error = DistributedError::outcome_unknown(
            Some("operation-123".to_owned()),
            DeliveryState::Unknown,
            "execution status could not be confirmed",
        );

        assert_eq!(
            error.retry_advice(),
            RetryAdvice::ReconcileOutcomeFirst
        );
        assert_eq!(
            error.kind(),
            DistributedErrorKind::OperationOutcomeUnknown
        );
    }

    #[test]
    fn definitely_unsent_transport_operation_may_be_retried_by_policy() {
        let error = DistributedError::transport_failed(
            "connect",
            DeliveryState::NotSent,
            "connection attempt failed",
        );

        assert_eq!(
            error.retry_advice(),
            RetryAdvice::PossiblyAfterBackoff
        );
    }

    #[test]
    fn authorization_failures_are_not_retryable() {
        let error = DistributedError::AuthorizationDenied {
            principal: Some("worker-identity".to_owned()),
            operation: "execute-workload".to_owned(),
        };

        assert_eq!(error.retry_advice(), RetryAdvice::Never);
        assert_eq!(error.severity(), DistributedErrorSeverity::Security);
    }

    #[test]
    fn resource_exhaustion_is_not_semantic_invalidity() {
        let error = DistributedError::resource_unsatisfied(
            "memory.bytes",
            Some(200),
            Some(100),
            Some("bytes".to_owned()),
            true,
        );

        assert!(!error.is_semantic_failure());
        assert!(error.is_feasibility_failure());

        match error {
            DistributedError::ResourceUnsatisfied {
                requested,
                available,
                configured_budget,
                ..
            } => {
                assert_eq!(requested, Some(200));
                assert_eq!(available, Some(100));
                assert!(configured_budget);
            }
            _ => panic!("expected a resource feasibility error"),
        }
    }

    #[test]
    fn unknown_resource_quantities_remain_unknown() {
        let error = DistributedError::resource_unsatisfied(
            "accelerator.memory",
            None,
            None,
            Some("bytes".to_owned()),
            false,
        );

        match error {
            DistributedError::ResourceUnsatisfied {
                requested,
                available,
                ..
            } => {
                assert_eq!(requested, None);
                assert_eq!(available, None);
            }
            _ => panic!("expected a resource feasibility error"),
        }
    }

    #[test]
    fn external_error_preserves_its_source_chain() {
        let original = std::io::Error::other("connection reset");
        let error = DistributedError::external(
            "networking",
            "request failed",
            original,
        );

        assert_eq!(error.kind(), DistributedErrorKind::ExternalFailure);
        assert!(StdError::source(&error).is_some());
        assert!(error.to_string().contains("request failed"));
        assert!(error.to_string().contains("connection reset"));
    }

    #[test]
    fn error_display_is_human_readable() {
        let error = DistributedError::ProtocolIncompatible {
            subject: "distributed-message".to_owned(),
            expected: "v2".to_owned(),
            actual: "v1".to_owned(),
        };

        assert!(error.to_string().contains("distributed-message"));
        assert!(error.to_string().contains("expected v2"));
        assert!(error.to_string().contains("observed v1"));
    }

    #[test]
    fn feasibility_errors_are_distinct_from_semantic_errors() {
        let semantic = DistributedError::semantic_invalid(
            "dependency-order",
            "a required dependency is invalid",
        );

        let capability =
            DistributedError::capability_unsatisfied("required.compute", None);

        assert!(semantic.is_semantic_failure());
        assert!(!capability.is_semantic_failure());
        assert!(capability.is_feasibility_failure());
    }

    #[test]
    fn retry_advice_never_overrides_cancellation_or_security() {
        let cancelled = DistributedError::Cancelled {
            operation: Some("task-1".to_owned()),
            reason: "caller requested cancellation".to_owned(),
        };

        let authentication = DistributedError::AuthenticationFailed {
            peer: None,
            message: "identity verification failed".to_owned(),
        };

        assert_eq!(cancelled.retry_advice(), RetryAdvice::Never);
        assert_eq!(authentication.retry_advice(), RetryAdvice::Never);
    }
}
