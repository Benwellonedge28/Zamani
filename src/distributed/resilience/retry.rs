
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Distributed Resilience — Retry Decisions
//!
//! Path:
//!     src/distributed/resilience/retry.rs
//!
//! Rust:
//!     1.97+
//!
//! Edition:
//!     2021
//!
//! Safety:
//!     No unsafe Rust.
//!
//! Dependencies:
//!     Existing distributed configuration and error contracts.
//!     Rust standard library.
//!
//! # Purpose
//!
//! Provides deterministic, provider-independent retry evaluation for
//! distributed workloads.
//!
//! This module decides whether another attempt may be considered.
//! It does not execute the retry.
//!
//! # Ownership
//!
//! This module owns:
//! - Retry evaluation input.
//! - Execution-state evidence required for retry decisions.
//! - Explicit retry-safety evidence.
//! - Retry decision results.
//! - Retry-policy evaluation.
//!
//! This module does not own:
//! - Execution.
//! - Scheduling.
//! - Placement.
//! - Networking.
//! - Transport.
//! - Idempotency storage.
//! - Checkpointing.
//! - Rollback.
//! - Recovery execution.
//! - Security authentication.
//! - Authorization.
//! - Semantic analysis.
//! - Domain-specific IR.
//! - Physical hardware allocation.
//! - Timers or sleeping.
//! - Background tasks.
//! - Global state.
//!
//! # Fundamental invariant
//!
//! An error being retryable does not mean that repeating the operation
//! is semantically safe.
//!
//! A retry decision requires:
//!
//!     error retry advice
//!     + execution-state evidence
//!     + explicit retry-safety evidence
//!     + configured retry policy
//!     + deadline validity
//!     + cancellation validity
//!     + capacity availability
//!
//! Unknown outcomes must be reconciled before another submission.
//!
//! Partial execution must be handled by the recovery authority.
//!
//! # Scalability
//!
//! This module imposes no universal maximum on:
//! - tasks;
//! - participants;
//! - workers;
//! - machines;
//! - retries;
//! - computational domains;
//! - devices;
//! - resources.
//!
//! Retry limits are configurable operational policies.
//!
//! An unlimited retry policy does not create an infinite execution loop.
//! Cancellation, deadlines, resource availability, semantic safety and
//! external policy remain authoritative.
//!
//! # Integration
//!
//! `distributed/config.rs`:
//!     Owns the canonical RetryPolicy.
//!
//! `distributed/error.rs`:
//!     Owns DistributedError, RetryAdvice and DeliveryState.
//!
//! `distributed/execution/`:
//!     Supplies execution-state and retry-safety evidence.
//!
//! `distributed/planning/`:
//!     Replans when the existing realization is infeasible.
//!
//! `distributed/resilience/idempotency.rs`:
//!     Supplies durable deduplication evidence.
//!
//! `distributed/resilience/recovery.rs`:
//!     Owns recovery and reconciliation.
//!
//! `distributed/resilience/checkpoint.rs`:
//!     Owns checkpoint validity.
//!
//! `distributed/observability/`:
//!     Records retry decisions and provenance.
//!
//! `runtime/`:
//!     Supplies cancellation, deadlines and actual execution state.
//!
//! `quantum::ir`:
//!     Remains the authoritative quantum representation.
//!
//! No domain-specific retry implementation is introduced here.

use core::fmt;
use std::time::Duration;

use crate::distributed::config::RetryPolicy;
use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedErrorKind,
    RetryAdvice,
};

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable identifier for the distributed retry-decision contract.
pub const DISTRIBUTED_RETRY_SCHEMA_ID: &str =
    "zamani.distributed.resilience.retry";

/// Version of the distributed retry-decision contract.
///
/// This is not the language, IR, transport or configuration version.
pub const DISTRIBUTED_RETRY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// EXECUTION STATE
// ============================================================================

/// Explicit knowledge about the preceding execution attempt.
///
/// This state must be supplied by the authoritative execution boundary.
/// It must never be guessed from an error message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PreviousAttemptState {
    /// The operation was not submitted to an execution boundary.
    ///
    /// No remote execution is known to have started.
    NotSubmitted,

    /// The execution boundary explicitly rejected the request before
    /// execution started.
    RejectedBeforeExecution,

    /// Execution failed before a semantic result was produced.
    ///
    /// This does not automatically establish that no side effects occurred.
    FailedBeforeResult,

    /// The operation completed and returned a failure result.
    ///
    /// Repeating it still requires explicit side-effect safety.
    CompletedFailure,

    /// Some portion of the computation executed.
    ///
    /// Repeating the complete operation may duplicate or corrupt effects.
    PartialExecution,

    /// Execution may have occurred, but its outcome is not established.
    OutcomeUnknown,

    /// The operation completed successfully.
    Succeeded,
}

// ============================================================================
// RETRY SAFETY
// ============================================================================

/// Explicit evidence governing whether an operation may be repeated.
///
/// This evidence must come from the task's semantic contract, execution
/// provider, or authoritative idempotency subsystem.
///
/// A caller must not construct stronger evidence than the actual
/// execution guarantees establish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetrySafety {
    /// Repeating the operation is prohibited.
    Never,

    /// The operation is established to be safe to repeat.
    ///
    /// This must account for externally observable side effects.
    Idempotent,

    /// A durable idempotency authority has confirmed that the same
    /// operation identity is protected against duplicate execution.
    ///
    /// The operation key must be stable across attempts, and the
    /// deduplication guarantee must cover the relevant execution boundary.
    DurableDeduplicationConfirmed,
}

// ============================================================================
// CAPACITY EVIDENCE
// ============================================================================

/// Explicit evidence that another attempt has sufficient operational
/// capacity to be considered.
///
/// This does not replace planning or resource feasibility checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetryCapacity {
    /// Whether the retry execution budget permits another attempt.
    pub execution_budget_available: bool,

    /// Whether the relevant resources are currently feasible.
    pub resources_feasible: bool,

    /// Whether required execution capabilities remain available.
    pub capabilities_available: bool,
}

impl RetryCapacity {
    /// Creates affirmative capacity evidence.
    #[must_use]
    pub const fn available() -> Self {
        Self {
            execution_budget_available: true,
            resources_feasible: true,
            capabilities_available: true,
        }
    }

    /// Creates unavailable capacity evidence.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            execution_budget_available: false,
            resources_feasible: false,
            capabilities_available: false,
        }
    }

    /// Returns whether all capacity requirements are satisfied.
    #[must_use]
    pub const fn permits_retry(self) -> bool {
        self.execution_budget_available
            && self.resources_feasible
            && self.capabilities_available
    }
}

impl Default for RetryCapacity {
    fn default() -> Self {
        Self::unavailable()
    }
}

// ============================================================================
// SEMANTIC VALIDITY
// ============================================================================

/// Evidence concerning the validity of repeating the logical computation.
///
/// Semantic validity is not inferred from operational failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetrySemanticState {
    /// The original computation remains semantically valid.
    Valid,

    /// The computation requires fresh semantic or planning validation.
    RequiresRevalidation,

    /// The computation is semantically invalid.
    Invalid,
}

// ============================================================================
// EVALUATION INPUT
// ============================================================================

/// Complete evidence supplied to one retry evaluation.
///
/// All observations are explicit. This module does not inspect clocks,
/// execution providers, network connections or global state.
#[derive(Debug, Clone, Copy)]
pub struct RetryEvaluationInput<'a> {
    /// Error from the preceding operation.
    pub error: &'a DistributedError,

    /// Execution state established by the execution authority.
    pub previous_state: PreviousAttemptState,

    /// Explicit permission to repeat the operation.
    pub safety: RetrySafety,

    /// Semantic validity of the operation.
    pub semantic_state: RetrySemanticState,

    /// Number of retries already performed.
    ///
    /// The initial execution is not counted as a retry.
    pub retries_already_performed: usize,

    /// Time remaining until the authoritative execution deadline.
    ///
    /// `None` means this evaluation has no externally imposed remaining
    /// deadline. It does not mean execution must continue forever.
    pub remaining_deadline: Option<Duration>,

    /// Whether cancellation has been requested.
    pub cancelled: bool,

    /// Capacity evidence for another attempt.
    pub capacity: RetryCapacity,
}

impl<'a> RetryEvaluationInput<'a> {
    /// Creates an evaluation input from explicit evidence.
    #[must_use]
    pub const fn new(
        error: &'a DistributedError,
        previous_state: PreviousAttemptState,
        safety: RetrySafety,
        semantic_state: RetrySemanticState,
        retries_already_performed: usize,
        remaining_deadline: Option<Duration>,
        cancelled: bool,
        capacity: RetryCapacity,
    ) -> Self {
        Self {
            error,
            previous_state,
            safety,
            semantic_state,
            retries_already_performed,
            remaining_deadline,
            cancelled,
            capacity,
        }
    }
}

// ============================================================================
// DECISION REASONS
// ============================================================================

/// Machine-readable explanation for a retry decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetryStopReason {
    /// The configured policy does not permit another retry.
    PolicyExhausted,

    /// Error classification prohibits retry.
    ErrorNotRetryable,

    /// Cancellation has been requested.
    Cancelled,

    /// The operation deadline has expired.
    DeadlineExpired,

    /// The retry delay cannot fit within the remaining deadline.
    InsufficientDeadline,

    /// The operation is not safe to repeat.
    UnsafeToRepeat,

    /// Semantic validation failed.
    SemanticInvalid,

    /// Semantic validation must be performed again.
    RevalidationRequired,

    /// Required execution budget is exhausted.
    ExecutionBudgetUnavailable,

    /// Required resources are unavailable.
    ResourcesUnavailable,

    /// Required capabilities are unavailable.
    CapabilitiesUnavailable,

    /// The previous attempt succeeded.
    AlreadySucceeded,
}

/// Action required before another execution attempt may be considered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetryReconciliation {
    /// Query authoritative execution records to establish whether the
    /// previous operation executed or completed.
    EstablishOutcome,

    /// Recover or reconcile partially executed work.
    RecoverPartialExecution,

    /// Obtain a new feasible execution plan.
    Replan,

    /// Revalidate semantic obligations before another attempt.
    RevalidateSemantics,
}

// ============================================================================
// RETRY DECISION
// ============================================================================

/// Result of evaluating one retry opportunity.
///
/// `RetryPermitted` is advisory. It authorizes no network request,
/// execution, scheduling decision or resource allocation by itself.
///
/// Exactly one retry opportunity is evaluated per call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RetryDecision {
    /// One additional attempt may be considered.
    ///
    /// The caller must still obtain a fresh execution plan and use the
    /// actual recovery/execution authority.
    RetryPermitted {
        /// Requested delay before the next attempt.
        delay: Duration,

        /// Number of retries that will have been performed if the
        /// next attempt is submitted.
        retry_number: usize,
    },

    /// The operation must be reconciled before another submission.
    ReconcileRequired {
        /// Required reconciliation action.
        action: RetryReconciliation,
    },

    /// A fresh execution plan is required.
    ReplanRequired,

    /// Retry is prohibited.
    Stop {
        /// Reason retry was prohibited.
        reason: RetryStopReason,
    },
}

impl RetryDecision {
    /// Returns whether the decision permits consideration of another attempt.
    #[must_use]
    pub const fn retry_permitted(self) -> bool {
        matches!(self, Self::RetryPermitted { .. })
    }

    /// Returns the requested retry delay, if a retry is permitted.
    #[must_use]
    pub const fn delay(self) -> Option<Duration> {
        match self {
            Self::RetryPermitted { delay, .. } => Some(delay),
            _ => None,
        }
    }

    /// Returns the required reconciliation action, if any.
    #[must_use]
    pub const fn reconciliation(self) -> Option<RetryReconciliation> {
        match self {
            Self::ReconcileRequired { action } => Some(action),
            _ => None,
        }
    }

    /// Returns the stop reason, if retry is prohibited.
    #[must_use]
    pub const fn stop_reason(self) -> Option<RetryStopReason> {
        match self {
            Self::Stop { reason } => Some(reason),
            _ => None,
        }
    }
}

// ============================================================================
// POLICY EVALUATOR
// ============================================================================

/// Stateless distributed retry-policy evaluator.
///
/// The evaluator consumes the existing configuration and error authorities.
/// It does not own or mutate retry budgets.
#[derive(Debug, Default, Clone, Copy)]
pub struct RetryEvaluator;

impl RetryEvaluator {
    /// Creates an evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates one retry opportunity.
    ///
    /// Evaluation order is intentionally conservative:
    ///
    /// 1. Reject successful operations.
    /// 2. Respect cancellation.
    /// 3. Resolve unknown or partial execution.
    /// 4. Respect semantic validity.
    /// 5. Respect canonical error classification.
    /// 6. Respect retry safety.
    /// 7. Respect configured attempt limits.
    /// 8. Respect execution capacity.
    /// 9. Respect deadlines.
    /// 10. Return one retry recommendation.
    ///
    /// This method is deterministic for identical inputs.
    ///
    /// It does not sleep, read the clock, perform I/O, execute an operation,
    /// or generate randomness.
    #[must_use]
    pub fn evaluate(
        &self,
        policy: &RetryPolicy,
        input: &RetryEvaluationInput<'_>,
    ) -> RetryDecision {
        // A successful operation must never be submitted again through
        // the retry path.
        if matches!(
            input.previous_state,
            PreviousAttemptState::Succeeded
        ) {
            return RetryDecision::Stop {
                reason: RetryStopReason::AlreadySucceeded,
            };
        }

        // Cancellation is authoritative.
        if input.cancelled
            || matches!(
                input.error.kind(),
                DistributedErrorKind::Cancelled
            )
        {
            return RetryDecision::Stop {
                reason: RetryStopReason::Cancelled,
            };
        }

        // An unknown outcome must never become a blind resubmission.
        if matches!(
            input.previous_state,
            PreviousAttemptState::OutcomeUnknown
        ) || matches!(
            input.error.retry_advice(),
            RetryAdvice::ReconcileOutcomeFirst
        ) {
            return RetryDecision::ReconcileRequired {
                action: RetryReconciliation::EstablishOutcome,
            };
        }

        // Partial execution requires recovery, not repetition of the
        // original operation.
        if matches!(
            input.previous_state,
            PreviousAttemptState::PartialExecution
        ) {
            return RetryDecision::ReconcileRequired {
                action: RetryReconciliation::RecoverPartialExecution,
            };
        }

        // Semantic invalidity is never repaired by retry.
        match input.semantic_state {
            RetrySemanticState::Invalid => {
                return RetryDecision::Stop {
                    reason: RetryStopReason::SemanticInvalid,
                };
            }

            RetrySemanticState::RequiresRevalidation => {
                return RetryDecision::ReconcileRequired {
                    action: RetryReconciliation::RevalidateSemantics,
                };
            }

            RetrySemanticState::Valid => {}
        }

        // The canonical error model remains authoritative.
        match input.error.retry_advice() {
            RetryAdvice::Never => {
                return RetryDecision::Stop {
                    reason: RetryStopReason::ErrorNotRetryable,
                };
            }

            RetryAdvice::Replan => {
                return RetryDecision::ReplanRequired;
            }

            RetryAdvice::ReconcileOutcomeFirst => {
                return RetryDecision::ReconcileRequired {
                    action: RetryReconciliation::EstablishOutcome,
                };
            }

            RetryAdvice::OnlyIfIdempotent
            | RetryAdvice::PossiblyAfterBackoff => {}
        }

        // A submission that never reached the execution boundary,
        // or was explicitly rejected before execution, has no known
        // execution side effect.
        //
        // All other execution states require explicit retry safety.
        let definitely_not_executed = matches!(
            input.previous_state,
            PreviousAttemptState::NotSubmitted
                | PreviousAttemptState::RejectedBeforeExecution
        );

        if !definitely_not_executed {
            match input.safety {
                RetrySafety::Never => {
                    return RetryDecision::Stop {
                        reason: RetryStopReason::UnsafeToRepeat,
                    };
                }

                RetrySafety::Idempotent
                | RetrySafety::DurableDeduplicationConfirmed => {}
            }
        }

        // The existing RetryPolicy counts the initial attempt.
        // Therefore retries = max_attempts - 1.
        let maximum_retries = policy
            .max_attempts()
            .get()
            .saturating_sub(1);

        if input.retries_already_performed >= maximum_retries {
            return RetryDecision::Stop {
                reason: RetryStopReason::PolicyExhausted,
            };
        }

        // Do not retry when operational capacity is unavailable.
        if !input.capacity.execution_budget_available {
            return RetryDecision::Stop {
                reason: RetryStopReason::ExecutionBudgetUnavailable,
            };
        }

        if !input.capacity.resources_feasible {
            return RetryDecision::Stop {
                reason: RetryStopReason::ResourcesUnavailable,
            };
        }

        if !input.capacity.capabilities_available {
            return RetryDecision::Stop {
                reason: RetryStopReason::CapabilitiesUnavailable,
            };
        }

        // Compute the delay without sleeping.
        //
        // RetryPolicy uses a zero-based retry index.
        let delay = policy.delay_before_retry(
            input.retries_already_performed,
        );

        // Check the externally supplied remaining deadline.
        if let Some(remaining) = input.remaining_deadline {
            if remaining.is_zero() {
                return RetryDecision::Stop {
                    reason: RetryStopReason::DeadlineExpired,
                };
            }

            if delay > remaining {
                return RetryDecision::Stop {
                    reason: RetryStopReason::InsufficientDeadline,
                };
            }
        }

        // Saturating addition prevents arithmetic overflow in the
        // reported retry number.
        let retry_number = input
            .retries_already_performed
            .saturating_add(1);

        RetryDecision::RetryPermitted {
            delay,
            retry_number,
        }
    }
}

// ============================================================================
// VALIDATION / DISPLAY
// ============================================================================

impl fmt::Display for RetryStopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::PolicyExhausted => "retry policy exhausted",
            Self::ErrorNotRetryable => "error does not permit retry",
            Self::Cancelled => "operation cancelled",
            Self::DeadlineExpired => "operation deadline expired",
            Self::InsufficientDeadline => {
                "retry delay exceeds remaining deadline"
            }
            Self::UnsafeToRepeat => {
                "operation is not established as safe to repeat"
            }
            Self::SemanticInvalid => "operation is semantically invalid",
            Self::RevalidationRequired => {
                "semantic revalidation is required"
            }
            Self::ExecutionBudgetUnavailable => {
                "execution budget unavailable"
            }
            Self::ResourcesUnavailable => "required resources unavailable",
            Self::CapabilitiesUnavailable => {
                "required capabilities unavailable"
            }
            Self::AlreadySucceeded => "operation already succeeded",
        };

        f.write_str(message)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;

    fn policy(max_attempts: usize) -> RetryPolicy {
        RetryPolicy::new(
            NonZeroUsize::new(max_attempts)
                .expect("test attempt count must be nonzero"),
            Duration::from_millis(10),
            Duration::from_secs(5),
            2,
        )
        .expect("test retry policy must be valid")
    }

    fn transport_error(delivery: DeliveryState) -> DistributedError {
        DistributedError::TransportFailed {
            operation: "submit".to_owned(),
            delivery,
            message: "simulated transport failure".to_owned(),
        }
    }

    fn evaluate(
        policy: &RetryPolicy,
        error: &DistributedError,
        state: PreviousAttemptState,
        safety: RetrySafety,
        cancelled: bool,
    ) -> RetryDecision {
        let input = RetryEvaluationInput::new(
            error,
            state,
            safety,
            RetrySemanticState::Valid,
            0,
            None,
            cancelled,
            RetryCapacity::available(),
        );

        RetryEvaluator::new().evaluate(policy, &input)
    }

    #[test]
    fn permits_safe_operation_that_was_never_submitted() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            false,
        );

        assert_eq!(
            decision,
            RetryDecision::RetryPermitted {
                delay: Duration::from_millis(10),
                retry_number: 1,
            }
        );
    }

    #[test]
    fn permits_explicit_pre_execution_rejection() {
        let p = policy(3);
        let error = transport_error(
            DeliveryState::RejectedBeforeExecution,
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::RejectedBeforeExecution,
            RetrySafety::Never,
            false,
        );

        assert!(decision.retry_permitted());
    }

    #[test]
    fn refuses_unsafe_completed_operation() {
        let p = policy(3);
        let error = DistributedError::execution_failed(
            None,
            "operation failed",
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::CompletedFailure,
            RetrySafety::Never,
            false,
        );

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::UnsafeToRepeat)
        );
    }

    #[test]
    fn permits_idempotent_completed_operation() {
        let p = policy(3);
        let error = DistributedError::execution_failed(
            None,
            "operation failed",
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::CompletedFailure,
            RetrySafety::Idempotent,
            false,
        );

        assert!(decision.retry_permitted());
    }

    #[test]
    fn unknown_outcome_requires_reconciliation() {
        let p = policy(3);
        let error = transport_error(DeliveryState::ResponseLost);

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::OutcomeUnknown,
            RetrySafety::Idempotent,
            false,
        );

        assert_eq!(
            decision.reconciliation(),
            Some(RetryReconciliation::EstablishOutcome)
        );
    }

    #[test]
    fn partial_execution_requires_recovery() {
        let p = policy(3);
        let error = DistributedError::execution_failed(
            None,
            "partial execution",
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::PartialExecution,
            RetrySafety::Idempotent,
            false,
        );

        assert_eq!(
            decision.reconciliation(),
            Some(RetryReconciliation::RecoverPartialExecution)
        );
    }

    #[test]
    fn cancellation_always_prevents_retry() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Idempotent,
            true,
        );

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::Cancelled)
        );
    }

    #[test]
    fn successful_operation_is_never_retried() {
        let p = policy(3);
        let error = DistributedError::execution_failed(
            None,
            "stale error evidence",
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::Succeeded,
            RetrySafety::Idempotent,
            false,
        );

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::AlreadySucceeded)
        );
    }

    #[test]
    fn semantic_invalidity_prevents_retry() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Idempotent,
            RetrySemanticState::Invalid,
            0,
            None,
            false,
            RetryCapacity::available(),
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::SemanticInvalid)
        );
    }

    #[test]
    fn revalidation_is_required_when_semantics_are_uncertain() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Idempotent,
            RetrySemanticState::RequiresRevalidation,
            0,
            None,
            false,
            RetryCapacity::available(),
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.reconciliation(),
            Some(RetryReconciliation::RevalidateSemantics)
        );
    }

    #[test]
    fn configured_attempt_limit_is_respected() {
        let p = policy(2);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            RetrySemanticState::Valid,
            1,
            None,
            false,
            RetryCapacity::available(),
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::PolicyExhausted)
        );
    }

    #[test]
    fn zero_remaining_deadline_prevents_retry() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            RetrySemanticState::Valid,
            0,
            Some(Duration::ZERO),
            false,
            RetryCapacity::available(),
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::DeadlineExpired)
        );
    }

    #[test]
    fn delay_must_fit_remaining_deadline() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            RetrySemanticState::Valid,
            0,
            Some(Duration::from_millis(5)),
            false,
            RetryCapacity::available(),
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::InsufficientDeadline)
        );
    }

    #[test]
    fn unavailable_resources_prevent_retry() {
        let p = policy(3);
        let error = transport_error(DeliveryState::NotSent);

        let input = RetryEvaluationInput::new(
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            RetrySemanticState::Valid,
            0,
            None,
            false,
            RetryCapacity {
                execution_budget_available: true,
                resources_feasible: false,
                capabilities_available: true,
            },
        );

        let decision = RetryEvaluator::new().evaluate(&p, &input);

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::ResourcesUnavailable)
        );
    }

    #[test]
    fn security_failures_are_not_retryable() {
        let p = policy(3);

        let error = DistributedError::AuthenticationFailed {
            peer: None,
            message: "identity verification failed".to_owned(),
        };

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Idempotent,
            false,
        );

        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::ErrorNotRetryable)
        );
    }

    #[test]
    fn feasibility_failure_requests_replanning() {
        let p = policy(3);

        let error = DistributedError::resource_unsatisfied(
            "memory.bytes",
            Some(1024),
            Some(512),
            Some("bytes".to_owned()),
            true,
        );

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            false,
        );

        assert_eq!(decision, RetryDecision::ReplanRequired);
    }

    #[test]
    fn unlimited_policy_does_not_create_unbounded_execution() {
        let p = RetryPolicy::default();
        let error = transport_error(DeliveryState::NotSent);

        let decision = evaluate(
            &p,
            &error,
            PreviousAttemptState::NotSubmitted,
            RetrySafety::Never,
            false,
        );

        // The existing configuration defaults to one total attempt.
        assert_eq!(
            decision.stop_reason(),
            Some(RetryStopReason::PolicyExhausted)
        );
    }
}
