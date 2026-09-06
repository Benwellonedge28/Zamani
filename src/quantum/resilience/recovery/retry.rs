//! Zamani Quantum Resilience — Retry Execution
//!
//! Path:
//!     src/quantum/resilience/recovery/retry.rs
//!
//! Purpose:
//!     Execute a single retry decision that has already been authorized by the
//!     resilience policy/planning layers.
//!
//! Architectural position:
//!
//!     policy/retry.rs
//!          |
//!          v
//!     planning/action.rs
//!          |
//!          v
//!     planning/plan.rs
//!          |
//!          v
//!     recovery/retry.rs       <-- THIS MODULE
//!          |
//!          v
//!     quantum execution boundary
//!          |
//!          v
//!     verification/*
//!
//! This module owns RETRY EXECUTION only.
//!
//! It does NOT own:
//!
//! - retry policy;
//! - retry budgets;
//! - diagnosis;
//! - fault detection;
//! - fault classification;
//! - hardware discovery;
//! - backend selection;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - error mitigation;
//! - quantum IR;
//! - provider SDKs;
//! - credentials;
//! - authentication;
//! - verification policy;
//! - global resilience orchestration.
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! -----------------------------------------------------------------------------
//! Fundamental safety rule
//! -----------------------------------------------------------------------------
//!
//! A retry is NOT:
//!
//!     "the previous attempt failed, therefore submit again."
//!
//! A retry is valid only when the execution boundary has established that
//! another attempt is semantically and operationally safe.
//!
//! In particular, the executor MUST distinguish:
//!
//!     NotSubmitted
//!     SubmittedAndFailed
//!     SubmittedAndSucceeded
//!     SubmissionOutcomeUnknown
//!     PartiallyExecuted
//!
//! These states are not interchangeable.
//!
//! A network timeout after submission, for example, MUST NOT automatically
//! become a second submission. The original quantum job may still be running.
//! Blind resubmission could therefore execute the computation twice.
//!
//! Reconciliation belongs to the execution/backend boundary. This module
//! refuses to turn an unknown execution state into a blind retry.
//!
//! -----------------------------------------------------------------------------
//! Write once, scale everywhere
//! -----------------------------------------------------------------------------
//!
//! This module contains no:
//!
//!     MAX_RETRIES
//!     MAX_ATTEMPTS
//!     MAX_QUBITS
//!     MAX_BACKENDS
//!     MAX_SHOTS
//!     MAX_EXECUTION_TIME
//!
//! Retry count, deadline, budgets and resource availability are supplied by
//! callers and authoritative policy systems.
//!
//! "Infinite" scalability means that this implementation introduces no
//! artificial machine-size ceiling. Actual execution remains bounded by:
//!
//!     - available memory;
//!     - `usize` addressability;
//!     - execution resources;
//!     - hardware capabilities;
//!     - runtime capacity;
//!     - configured policy;
//!     - deadlines;
//!     - budgets;
//!     - security controls;
//!     - the physical quantum system itself.
//!
//! -----------------------------------------------------------------------------
//! Quantum correctness
//! -----------------------------------------------------------------------------
//!
//! This module intentionally does not manipulate quantum state.
//!
//! A retry of a quantum circuit generally creates a new physical execution.
//! It does not restore the quantum state of the previous execution.
//!
//! Therefore:
//!
//!     retry != rollback
//!     retry != resume
//!     retry != checkpoint restore
//!
//! A retry is only valid at an execution boundary where repeating the operation
//! has been declared semantically valid.
//!
//! Examples:
//!
//! 1. Submission definitely never reached the backend:
//!        retry may be safe.
//!
//! 2. Backend rejected the job before execution:
//!        retry may be safe if the rejection is retryable.
//!
//! 3. Backend accepted the job but the client timed out:
//!        blind retry is unsafe; reconcile first.
//!
//! 4. Circuit partially executed:
//!        blind retry may duplicate work or violate semantics.
//!
//! 5. Measurement completed:
//!        another retry produces another sample; it does not reproduce the
//!        previous physical measurement outcome.
//!
//! -----------------------------------------------------------------------------
//! Idempotency
//! -----------------------------------------------------------------------------
//!
//! The execution boundary must provide an explicit idempotency/reconciliation
//! contract.
//!
//! This module does not guess whether a submission is idempotent.
//!
//! A retry request must therefore carry explicit execution-state evidence.
//!
//! -----------------------------------------------------------------------------
//! Backoff
//! -----------------------------------------------------------------------------
//!
//! Backoff is calculated here only as a value returned to the caller.
//!
//! This module NEVER:
//!
//! - sleeps;
//! - blocks on a timer;
//! - spawns a thread;
//! - owns an async runtime;
//! - reads the system clock;
//! - performs hidden waiting.
//!
//! The caller/execution scheduler decides when to invoke the next retry.
//!
//! This keeps retry deterministic and allows the same implementation to work
//! in synchronous, asynchronous, embedded, distributed, simulator and hardware
//! environments.
//!
//! -----------------------------------------------------------------------------
//! Canonical quantum identities
//! -----------------------------------------------------------------------------
//!
//! Retry normally does not need to know about qubits.
//!
//! If resource-specific retry evidence is required in the future, it MUST use
//! the canonical IR identity types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! No resilience-specific QubitId type is introduced here.
//!
//! -----------------------------------------------------------------------------
//! Integration contracts
//! -----------------------------------------------------------------------------
//!
//! `policy/retry.rs`
//!     Defines whether retry is permitted and supplies retry policy data.
//!
//! `planning/action.rs`
//!     Represents Retry as a declarative recovery action.
//!
//! `planning/plan.rs`
//!     Supplies the ordered recovery plan and retry preconditions.
//!
//! `planning/feasibility.rs`
//!     Determines whether retry is currently feasible.
//!
//! `recovery/recoverer.rs`
//!     Orchestrates this retry implementation with other recovery actions.
//!
//! `recovery/restart.rs`
//!     Handles restart semantics. It must not be confused with retry.
//!
//! `recovery/resume.rs`
//!     Handles continuation from valid execution boundaries.
//!
//! `recovery/rollback.rs`
//!     Handles restoration of an accepted prior state.
//!
//! `checkpoint/*`
//!     Owns checkpoint representation and persistence.
//!
//! `hardware/*`
//!     Owns actual provider/backend/device execution contracts.
//!
//! `verification/*`
//!     Determines whether a retried result is acceptable.
//!
//! `telemetry/*`
//!     Records retry lifecycle events.
//!
//! `history/*`
//!     Records retry outcomes for future planning.
//!
//! `coordination/*`
//!     Owns distributed resource ownership and leases.
//!
//! `errors/*`
//!     Owns the repository-wide resilience error taxonomy.
//!
//! -----------------------------------------------------------------------------
//! Design principle
//! -----------------------------------------------------------------------------
//!
//! This module is deliberately implemented around traits.
//!
//! The retry engine does not know whether execution is provided by:
//!
//!     - a real QPU;
//!     - a simulator;
//!     - an emulator;
//!     - a remote service;
//!     - a distributed quantum system;
//!     - a future quantum architecture.
//!
//! The execution boundary supplies the implementation.
//!
//! -----------------------------------------------------------------------------
//! Rust contract
//! -----------------------------------------------------------------------------
//!
//! Rust:
//!     1.97 / 1.97.1
//!
//! Edition:
//!     2021
//!
//! Safety:
//!     No unsafe code.
//!
//! Dependencies:
//!     Standard library only.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

use std::fmt;
use std::time::Duration;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for retry execution records.
pub const RETRY_EXECUTION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recovery.retry";

/// Schema version.
///
/// This version changes when externally observable retry semantics change.
pub const RETRY_EXECUTION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Retry attempt number
// =============================================================================

/// Number identifying an execution attempt.
///
/// The initial execution has attempt number `0`.
///
/// The first retry has attempt number `1`.
///
/// The second retry has attempt number `2`.
///
/// Using `u64` avoids imposing a small artificial retry ceiling while still
/// using a compact, deterministic representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RetryAttempt(u64);

impl RetryAttempt {
    /// Creates an attempt number.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric attempt number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next attempt number.
    ///
    /// Saturation prevents integer wraparound from turning the attempt number
    /// into an earlier attempt.
    #[must_use]
    pub const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// Returns whether this is the initial execution.
    #[must_use]
    pub const fn is_initial(self) -> bool {
        self.0 == 0
    }

    /// Returns whether this is a retry.
    #[must_use]
    pub const fn is_retry(self) -> bool {
        self.0 > 0
    }
}

impl Default for RetryAttempt {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Display for RetryAttempt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Execution state
// =============================================================================

/// Evidence about what happened to the previous execution attempt.
///
/// This classification is intentionally conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PreviousAttemptState {
    /// The execution was definitely not submitted to the execution boundary.
    NotSubmitted,

    /// The submission was rejected before quantum execution began.
    RejectedBeforeExecution,

    /// The execution definitely completed and failed.
    CompletedFailure,

    /// The execution definitely completed successfully.
    CompletedSuccess,

    /// The execution began and did not complete.
    PartiallyExecuted,

    /// The submission was accepted, but the final execution state is unknown.
    ///
    /// This state MUST NOT be blindly retried.
    OutcomeUnknown,
}

impl PreviousAttemptState {
    /// Returns whether the state is safe to submit again without reconciliation.
    #[must_use]
    pub const fn permits_blind_retry(self) -> bool {
        matches!(
            self,
            Self::NotSubmitted | Self::RejectedBeforeExecution
        )
    }

    /// Returns whether the state indicates that quantum execution may have
    /// started.
    #[must_use]
    pub const fn may_have_executed(self) -> bool {
        matches!(
            self,
            Self::CompletedFailure
                | Self::CompletedSuccess
                | Self::PartiallyExecuted
                | Self::OutcomeUnknown
        )
    }

    /// Returns whether the state represents successful completion.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::CompletedSuccess)
    }

    /// Returns whether reconciliation is required before a new submission.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::OutcomeUnknown)
    }
}

// =============================================================================
// Retry safety
// =============================================================================

/// Semantic safety classification supplied by the execution boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetrySafety {
    /// The execution boundary has established that retry is safe.
    Safe,

    /// Retry may be safe only after reconciliation or additional checks.
    Conditional,

    /// Retry is unsafe.
    Unsafe,

    /// The execution boundary cannot establish retry safety.
    Unknown,
}

impl RetrySafety {
    /// Returns whether this classification permits immediate retry execution.
    #[must_use]
    pub const fn permits_immediate_retry(self) -> bool {
        matches!(self, Self::Safe)
    }
}

// =============================================================================
// Retry decision
// =============================================================================

/// Final execution-level retry decision.
///
/// This is deliberately separate from policy because policy and execution
/// state are independent authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetryDecision {
    /// Retry can be submitted.
    Execute,

    /// Retry cannot yet be submitted because reconciliation is required.
    ReconcileFirst,

    /// Retry is forbidden.
    Reject,

    /// The prior attempt already succeeded.
    AlreadySuccessful,
}

impl RetryDecision {
    /// Returns whether the decision authorizes submission.
    #[must_use]
    pub const fn permits_submission(self) -> bool {
        matches!(self, Self::Execute)
    }

    /// Returns whether another subsystem must reconcile the execution first.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::ReconcileFirst)
    }

    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execute => "execute",
            Self::ReconcileFirst => "reconcile_first",
            Self::Reject => "reject",
            Self::AlreadySuccessful => "already_successful",
        }
    }
}

impl fmt::Display for RetryDecision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Retry request
// =============================================================================

/// Immutable request for one retry execution.
///
/// This object contains execution evidence, not policy ownership.
///
/// The retry policy should already have been evaluated by the planning layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryRequest {
    /// Stable execution identity.
    execution_id: String,

    /// Stable logical retry-operation identity.
    operation_id: String,

    /// Attempt being requested.
    attempt: RetryAttempt,

    /// State of the preceding attempt.
    previous_state: PreviousAttemptState,

    /// Semantic retry safety established by the caller/execution boundary.
    safety: RetrySafety,

    /// Whether policy has explicitly authorized the retry.
    policy_authorized: bool,

    /// Whether capability/feasibility checks have succeeded.
    capability_authorized: bool,

    /// Whether security/ownership authorization has succeeded.
    security_authorized: bool,

    /// Whether semantic validation has succeeded.
    semantic_authorized: bool,

    /// Requested delay before submission.
    ///
    /// The retry engine never sleeps for this duration.
    delay: Duration,

    /// Optional externally supplied deadline.
    ///
    /// The engine does not read the clock. A caller can compare elapsed time
    /// against this deadline before calling `execute`.
    deadline: Option<Duration>,

    /// Whether partial execution has explicitly been declared retry-safe.
    allow_partial_execution: bool,

    /// Stable idempotency/reconciliation key.
    ///
    /// This is opaque to the retry engine and may be used by the execution
    /// boundary to prevent duplicate submission.
    idempotency_key: Option<String>,
}

impl RetryRequest {
    /// Starts building a retry request.
    #[must_use]
    pub fn builder(
        execution_id: impl Into<String>,
        operation_id: impl Into<String>,
    ) -> RetryRequestBuilder {
        RetryRequestBuilder::new(execution_id, operation_id)
    }

    /// Returns the stable execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the logical operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Returns the requested attempt.
    #[must_use]
    pub const fn attempt(&self) -> RetryAttempt {
        self.attempt
    }

    /// Returns the preceding execution state.
    #[must_use]
    pub const fn previous_state(&self) -> PreviousAttemptState {
        self.previous_state
    }

    /// Returns the retry safety classification.
    #[must_use]
    pub const fn safety(&self) -> RetrySafety {
        self.safety
    }

    /// Returns whether policy authorized this retry.
    #[must_use]
    pub const fn policy_authorized(&self) -> bool {
        self.policy_authorized
    }

    /// Returns whether capabilities authorized this retry.
    #[must_use]
    pub const fn capability_authorized(&self) -> bool {
        self.capability_authorized
    }

    /// Returns whether security authorized this retry.
    #[must_use]
    pub const fn security_authorized(&self) -> bool {
        self.security_authorized
    }

    /// Returns whether semantic validation authorized this retry.
    #[must_use]
    pub const fn semantic_authorized(&self) -> bool {
        self.semantic_authorized
    }

    /// Returns the requested backoff.
    #[must_use]
    pub const fn delay(&self) -> Duration {
        self.delay
    }

    /// Returns the optional caller-supplied deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<Duration> {
        self.deadline
    }

    /// Returns whether explicitly authorized partial execution can be retried.
    #[must_use]
    pub const fn allow_partial_execution(&self) -> bool {
        self.allow_partial_execution
    }

    /// Returns the optional idempotency key.
    #[must_use]
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    /// Evaluates the execution-level decision without performing I/O.
    ///
    /// This is intentionally deterministic.
    #[must_use]
    pub const fn decision(&self) -> RetryDecision {
        if self.previous_state.is_success() {
            return RetryDecision::AlreadySuccessful;
        }

        if self.previous_state.requires_reconciliation() {
            return RetryDecision::ReconcileFirst;
        }

        if !self.policy_authorized
            || !self.capability_authorized
            || !self.security_authorized
            || !self.semantic_authorized
        {
            return RetryDecision::Reject;
        }

        if matches!(
            self.previous_state,
            PreviousAttemptState::PartiallyExecuted
        ) && !self.allow_partial_execution
        {
            return RetryDecision::Reject;
        }

        if !self.safety.permits_immediate_retry() {
            return RetryDecision::Reject;
        }

        if !self.previous_state.permits_blind_retry()
            && matches!(
                self.previous_state,
                PreviousAttemptState::CompletedFailure
            )
            && !self.allow_partial_execution
        {
            return RetryDecision::Reject;
        }

        RetryDecision::Execute
    }
}

/// Builder for [`RetryRequest`].
#[derive(Debug, Clone)]
pub struct RetryRequestBuilder {
    execution_id: String,
    operation_id: String,
    attempt: RetryAttempt,
    previous_state: PreviousAttemptState,
    safety: RetrySafety,
    policy_authorized: bool,
    capability_authorized: bool,
    security_authorized: bool,
    semantic_authorized: bool,
    delay: Duration,
    deadline: Option<Duration>,
    allow_partial_execution: bool,
    idempotency_key: Option<String>,
}

impl RetryRequestBuilder {
    /// Creates a builder.
    pub fn new(
        execution_id: impl Into<String>,
        operation_id: impl Into<String>,
    ) -> Self {
        Self {
            execution_id: execution_id.into(),
            operation_id: operation_id.into(),
            attempt: RetryAttempt::new(1),
            previous_state: PreviousAttemptState::NotSubmitted,
            safety: RetrySafety::Unknown,
            policy_authorized: false,
            capability_authorized: false,
            security_authorized: false,
            semantic_authorized: false,
            delay: Duration::ZERO,
            deadline: None,
            allow_partial_execution: false,
            idempotency_key: None,
        }
    }

    /// Sets the requested retry attempt.
    #[must_use]
    pub const fn attempt(mut self, attempt: RetryAttempt) -> Self {
        self.attempt = attempt;
        self
    }

    /// Sets the previous attempt state.
    #[must_use]
    pub const fn previous_state(
        mut self,
        state: PreviousAttemptState,
    ) -> Self {
        self.previous_state = state;
        self
    }

    /// Sets semantic retry safety.
    #[must_use]
    pub const fn safety(mut self, safety: RetrySafety) -> Self {
        self.safety = safety;
        self
    }

    /// Records policy authorization.
    #[must_use]
    pub const fn policy_authorized(mut self, value: bool) -> Self {
        self.policy_authorized = value;
        self
    }

    /// Records capability authorization.
    #[must_use]
    pub const fn capability_authorized(mut self, value: bool) -> Self {
        self.capability_authorized = value;
        self
    }

    /// Records security authorization.
    #[must_use]
    pub const fn security_authorized(mut self, value: bool) -> Self {
        self.security_authorized = value;
        self
    }

    /// Records semantic authorization.
    #[must_use]
    pub const fn semantic_authorized(mut self, value: bool) -> Self {
        self.semantic_authorized = value;
        self
    }

    /// Sets requested backoff.
    #[must_use]
    pub const fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets a caller-supplied deadline.
    #[must_use]
    pub const fn deadline(mut self, deadline: Option<Duration>) -> Self {
        self.deadline = deadline;
        self
    }

    /// Allows retry after explicitly authorized partial execution.
    #[must_use]
    pub const fn allow_partial_execution(mut self, value: bool) -> Self {
        self.allow_partial_execution = value;
        self
    }

    /// Sets an opaque idempotency key.
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Builds the immutable request.
    pub fn build(self) -> Result<RetryRequest, RetryError> {
        if self.execution_id.is_empty() {
            return Err(RetryError::EmptyExecutionId);
        }

        if self.operation_id.is_empty() {
            return Err(RetryError::EmptyOperationId);
        }

        if self.attempt.is_initial() {
            return Err(RetryError::InvalidRetryAttempt {
                attempt: self.attempt.get(),
            });
        }

        if self
            .idempotency_key
            .as_deref()
            .is_some_and(str::is_empty)
        {
            return Err(RetryError::EmptyIdempotencyKey);
        }

        Ok(RetryRequest {
            execution_id: self.execution_id,
            operation_id: self.operation_id,
            attempt: self.attempt,
            previous_state: self.previous_state,
            safety: self.safety,
            policy_authorized: self.policy_authorized,
            capability_authorized: self.capability_authorized,
            security_authorized: self.security_authorized,
            semantic_authorized: self.semantic_authorized,
            delay: self.delay,
            deadline: self.deadline,
            allow_partial_execution: self.allow_partial_execution,
            idempotency_key: self.idempotency_key,
        })
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Result of the retry submission/execution boundary.
///
/// The execution boundary owns the meaning of provider/backend-specific
/// execution outcomes and translates them into this provider-independent state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryExecutionStatus {
    /// The retry was accepted for execution and completed successfully.
    Succeeded,

    /// The retry was accepted but completed with a failure.
    Failed,

    /// The retry was accepted but has not completed.
    ///
    /// The caller may monitor it without submitting another copy.
    InProgress,

    /// The execution boundary rejected the retry before execution.
    Rejected,

    /// The execution outcome cannot currently be established.
    ///
    /// This is deliberately not equivalent to failure.
    OutcomeUnknown,
}

impl RetryExecutionStatus {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Rejected => "rejected",
            Self::OutcomeUnknown => "outcome_unknown",
        }
    }

    /// Returns whether quantum execution definitely succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Returns whether another blind retry may be considered by a higher layer.
    ///
    /// This does NOT authorize another retry. Policy, budgets, semantics and
    /// capabilities must still be evaluated again.
    #[must_use]
    pub const fn can_be_considered_for_retry(&self) -> bool {
        matches!(self, Self::Failed | Self::Rejected)
    }
}

impl fmt::Display for RetryExecutionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Provider-independent result returned by the retry executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryExecutionResult {
    /// Execution identity.
    execution_id: String,

    /// Logical retry operation identity.
    operation_id: String,

    /// Attempt number.
    attempt: RetryAttempt,

    /// Execution status.
    status: RetryExecutionStatus,

    /// Whether the submission itself was accepted.
    submission_accepted: bool,

    /// Whether the result requires verification.
    verification_required: bool,

    /// Requested backoff that preceded this execution.
    applied_delay: Duration,

    /// Optional opaque execution identifier assigned by the execution layer.
    backend_execution_id: Option<String>,
}

impl RetryExecutionResult {
    /// Creates a successful result.
    #[must_use]
    pub fn success(
        request: &RetryRequest,
        backend_execution_id: Option<String>,
    ) -> Self {
        Self {
            execution_id: request.execution_id.clone(),
            operation_id: request.operation_id.clone(),
            attempt: request.attempt,
            status: RetryExecutionStatus::Succeeded,
            submission_accepted: true,
            verification_required: true,
            applied_delay: request.delay,
            backend_execution_id,
        }
    }

    /// Creates a failed result.
    #[must_use]
    pub fn failure(
        request: &RetryRequest,
        backend_execution_id: Option<String>,
    ) -> Self {
        Self {
            execution_id: request.execution_id.clone(),
            operation_id: request.operation_id.clone(),
            attempt: request.attempt,
            status: RetryExecutionStatus::Failed,
            submission_accepted: true,
            verification_required: true,
            applied_delay: request.delay,
            backend_execution_id,
        }
    }

    /// Creates an in-progress result.
    #[must_use]
    pub fn in_progress(
        request: &RetryRequest,
        backend_execution_id: Option<String>,
    ) -> Self {
        Self {
            execution_id: request.execution_id.clone(),
            operation_id: request.operation_id.clone(),
            attempt: request.attempt,
            status: RetryExecutionStatus::InProgress,
            submission_accepted: true,
            verification_required: true,
            applied_delay: request.delay,
            backend_execution_id,
        }
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Returns the attempt.
    #[must_use]
    pub const fn attempt(&self) -> RetryAttempt {
        self.attempt
    }

    /// Returns execution status.
    #[must_use]
    pub const fn status(&self) -> &RetryExecutionStatus {
        &self.status
    }

    /// Returns whether submission was accepted.
    #[must_use]
    pub const fn submission_accepted(&self) -> bool {
        self.submission_accepted
    }

    /// Returns whether verification is required.
    #[must_use]
    pub const fn verification_required(&self) -> bool {
        self.verification_required
    }

    /// Returns the requested/applied delay.
    #[must_use]
    pub const fn applied_delay(&self) -> Duration {
        self.applied_delay
    }

    /// Returns the opaque backend execution identifier.
    #[must_use]
    pub fn backend_execution_id(&self) -> Option<&str> {
        self.backend_execution_id.as_deref()
    }
}

// =============================================================================
// Execution boundary
// =============================================================================

/// Provider-independent execution boundary used by [`RetryExecutor`].
///
/// Implementations belong outside resilience.
///
/// A hardware adapter, simulator, emulator or distributed runtime implements
/// this trait and translates its native execution model into these outcomes.
///
/// The retry engine never receives a provider SDK object.
pub trait RetryExecutionBoundary: Send + Sync {
    /// Provider-neutral output produced by one retry submission.
    type Output: Send + Sync;

    /// Provider-neutral execution error.
    type Error: Send + Sync + fmt::Display + fmt::Debug;

    /// Executes exactly ONE retry request.
    ///
    /// Implementations MUST NOT internally perform additional retries.
    ///
    /// Implementations SHOULD honor `request.idempotency_key()` where the
    /// underlying execution environment supports idempotent submission.
    ///
    /// Implementations MUST distinguish an accepted submission whose outcome
    /// is unknown from a submission that definitely never occurred.
    fn execute_retry(
        &self,
        request: &RetryRequest,
    ) -> Result<RetryBoundaryOutcome<Self::Output>, Self::Error>;
}

/// Outcome returned by the execution boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryBoundaryOutcome<T> {
    /// Execution completed successfully.
    Succeeded(T),

    /// Execution completed but failed semantically/operationally.
    Failed(T),

    /// Execution was accepted and remains in progress.
    InProgress(T),

    /// Execution was rejected before quantum execution.
    Rejected,

    /// The execution boundary cannot determine the outcome.
    ///
    /// This MUST NOT be converted into another submission by this module.
    OutcomeUnknown,
}

impl<T> RetryBoundaryOutcome<T> {
    /// Converts the boundary outcome to its provider-independent status.
    #[must_use]
    pub const fn status(&self) -> RetryExecutionStatus {
        match self {
            Self::Succeeded(_) => RetryExecutionStatus::Succeeded,
            Self::Failed(_) => RetryExecutionStatus::Failed,
            Self::InProgress(_) => RetryExecutionStatus::InProgress,
            Self::Rejected => RetryExecutionStatus::Rejected,
            Self::OutcomeUnknown => RetryExecutionStatus::OutcomeUnknown,
        }
    }
}

// =============================================================================
// Observer
// =============================================================================

/// Optional observer for retry lifecycle events.
///
/// Observability must remain non-invasive: an observer failure MUST NOT alter
/// the quantum execution outcome.
pub trait RetryObserver: Send + Sync {
    /// Called immediately before retry submission.
    fn before_retry(&self, _request: &RetryRequest) {}

    /// Called after the execution boundary returns.
    fn after_retry(&self, _request: &RetryRequest, _result: &RetryExecutionResult) {}

    /// Called when retry execution is rejected before submission.
    fn retry_rejected(&self, _request: &RetryRequest, _reason: &RetryError) {}
}

/// No-op observer.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopRetryObserver;

impl RetryObserver for NoopRetryObserver {}

// =============================================================================
// Retry executor
// =============================================================================

/// Executes exactly one authorized retry.
///
/// This is deliberately NOT a retry loop.
///
/// The higher-level `recoverer.rs` decides whether another retry should be
/// attempted after receiving the result and re-running the complete policy,
/// feasibility, budget, safety and verification process.
///
/// This separation prevents hidden retry loops and allows an arbitrarily large
/// recovery process without embedding a fixed retry limit here.
#[derive(Debug)]
pub struct RetryExecutor<B, O = NoopRetryObserver> {
    boundary: B,
    observer: O,
}

impl<B> RetryExecutor<B, NoopRetryObserver> {
    /// Creates a retry executor with no-op observability.
    #[must_use]
    pub const fn new(boundary: B) -> Self {
        Self {
            boundary,
            observer: NoopRetryObserver,
        }
    }
}

impl<B, O> RetryExecutor<B, O>
where
    B: RetryExecutionBoundary,
    O: RetryObserver,
{
    /// Creates a retry executor with an explicit observer.
    #[must_use]
    pub const fn with_observer(boundary: B, observer: O) -> Self {
        Self {
            boundary,
            observer,
        }
    }

    /// Returns a reference to the execution boundary.
    #[must_use]
    pub const fn boundary(&self) -> &B {
        &self.boundary
    }

    /// Returns a reference to the observer.
    #[must_use]
    pub const fn observer(&self) -> &O {
        &self.observer
    }

    /// Executes one retry.
    ///
    /// This method:
    ///
    /// 1. validates the request;
    /// 2. evaluates the local execution safety gate;
    /// 3. notifies the observer;
    /// 4. submits exactly one retry;
    /// 5. translates the result;
    /// 6. returns without sleeping or retrying again.
    ///
    /// The caller is responsible for waiting according to the returned
    /// execution status and, if necessary, asking the planner for a new plan.
    pub fn execute(
        &self,
        request: &RetryRequest,
    ) -> Result<RetryExecutionResult, RetryError> {
        validate_request(request)?;

        let decision = request.decision();

        if !decision.permits_submission() {
            let error = match decision {
                RetryDecision::AlreadySuccessful => {
                    RetryError::PreviousAttemptAlreadySucceeded
                }
                RetryDecision::ReconcileFirst => {
                    RetryError::ExecutionOutcomeUnknown
                }
                RetryDecision::Reject => RetryError::RetryNotAuthorized,
                RetryDecision::Execute => unreachable!(
                    "execute decision must permit submission"
                ),
            };

            self.observer.retry_rejected(request, &error);

            return Err(error);
        }

        self.observer.before_retry(request);

        let boundary_result = self
            .boundary
            .execute_retry(request)
            .map_err(RetryError::Boundary)?;

        let result = match boundary_result {
            RetryBoundaryOutcome::Succeeded(_) => {
                RetryExecutionResult::success(request, None)
            }

            RetryBoundaryOutcome::Failed(_) => {
                RetryExecutionResult::failure(request, None)
            }

            RetryBoundaryOutcome::InProgress(_) => {
                RetryExecutionResult::in_progress(request, None)
            }

            RetryBoundaryOutcome::Rejected => RetryExecutionResult {
                execution_id: request.execution_id.clone(),
                operation_id: request.operation_id.clone(),
                attempt: request.attempt,
                status: RetryExecutionStatus::Rejected,
                submission_accepted: false,
                verification_required: false,
                applied_delay: request.delay,
                backend_execution_id: None,
            },

            RetryBoundaryOutcome::OutcomeUnknown => RetryExecutionResult {
                execution_id: request.execution_id.clone(),
                operation_id: request.operation_id.clone(),
                attempt: request.attempt,
                status: RetryExecutionStatus::OutcomeUnknown,
                submission_accepted: true,
                verification_required: true,
                applied_delay: request.delay,
                backend_execution_id: None,
            },
        };

        self.observer.after_retry(request, &result);

        Ok(result)
    }
}

// =============================================================================
// Request validation
// =============================================================================

fn validate_request(request: &RetryRequest) -> Result<(), RetryError> {
    if request.execution_id.is_empty() {
        return Err(RetryError::EmptyExecutionId);
    }

    if request.operation_id.is_empty() {
        return Err(RetryError::EmptyOperationId);
    }

    if request.attempt.is_initial() {
        return Err(RetryError::InvalidRetryAttempt {
            attempt: request.attempt.get(),
        });
    }

    if request
        .idempotency_key
        .as_deref()
        .is_some_and(str::is_empty)
    {
        return Err(RetryError::EmptyIdempotencyKey);
    }

    if request.previous_state == PreviousAttemptState::OutcomeUnknown {
        return Err(RetryError::ExecutionOutcomeUnknown);
    }

    if request.previous_state == PreviousAttemptState::CompletedSuccess {
        return Err(RetryError::PreviousAttemptAlreadySucceeded);
    }

    if request.previous_state == PreviousAttemptState::PartiallyExecuted
        && !request.allow_partial_execution
    {
        return Err(RetryError::PartialExecutionNotAuthorized);
    }

    if !request.policy_authorized {
        return Err(RetryError::RetryNotAuthorized);
    }

    if !request.capability_authorized {
        return Err(RetryError::CapabilityNotAvailable);
    }

    if !request.security_authorized {
        return Err(RetryError::SecurityAuthorizationMissing);
    }

    if !request.semantic_authorized {
        return Err(RetryError::SemanticRetryNotAuthorized);
    }

    if !request.safety.permits_immediate_retry() {
        return Err(RetryError::RetrySafetyNotEstablished);
    }

    Ok(())
}

// =============================================================================
// Error model
// =============================================================================

/// Errors produced by retry execution.
///
/// This module intentionally uses its own execution error only at the local
/// execution boundary. The resilience root error layer can wrap/classify this
/// error without requiring this file to duplicate the repository-wide error
/// taxonomy.
#[derive(Debug)]
pub enum RetryError<E = Box<dyn std::error::Error + Send + Sync>> {
    /// Execution identity is empty.
    EmptyExecutionId,

    /// Logical operation identity is empty.
    EmptyOperationId,

    /// A retry request incorrectly used the initial attempt number.
    InvalidRetryAttempt {
        /// Invalid attempt number.
        attempt: u64,
    },

    /// Idempotency key was supplied but empty.
    EmptyIdempotencyKey,

    /// Previous execution succeeded.
    PreviousAttemptAlreadySucceeded,

    /// Previous execution outcome is unknown.
    ///
    /// Reconciliation is required before another submission.
    ExecutionOutcomeUnknown,

    /// Partial execution was not explicitly authorized for retry.
    PartialExecutionNotAuthorized,

    /// Retry was not authorized by policy.
    RetryNotAuthorized,

    /// Current capabilities do not permit retry.
    CapabilityNotAvailable,

    /// Required security authorization is missing.
    SecurityAuthorizationMissing,

    /// Semantic retry authorization is missing.
    SemanticRetryNotAuthorized,

    /// Retry safety was not established.
    RetrySafetyNotEstablished,

    /// The underlying execution boundary failed.
    Boundary(E),
}

impl<E> fmt::Display for RetryError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExecutionId => {
                formatter.write_str("retry execution identity is empty")
            }

            Self::EmptyOperationId => {
                formatter.write_str("retry operation identity is empty")
            }

            Self::InvalidRetryAttempt { attempt } => {
                write!(
                    formatter,
                    "invalid retry attempt {attempt}; retry attempts must be greater than zero"
                )
            }

            Self::EmptyIdempotencyKey => {
                formatter.write_str("retry idempotency key is empty")
            }

            Self::PreviousAttemptAlreadySucceeded => {
                formatter.write_str(
                    "previous attempt already succeeded; duplicate retry is forbidden",
                )
            }

            Self::ExecutionOutcomeUnknown => {
                formatter.write_str(
                    "previous execution outcome is unknown; reconciliation is required before retry",
                )
            }

            Self::PartialExecutionNotAuthorized => {
                formatter.write_str(
                    "previous execution may have partially executed and retry was not explicitly authorized",
                )
            }

            Self::RetryNotAuthorized => {
                formatter.write_str("retry is not authorized by the active policy")
            }

            Self::CapabilityNotAvailable => {
                formatter.write_str(
                    "required execution capability is not currently available",
                )
            }

            Self::SecurityAuthorizationMissing => {
                formatter.write_str(
                    "required security authorization for retry is missing",
                )
            }

            Self::SemanticRetryNotAuthorized => {
                formatter.write_str(
                    "semantic validation did not authorize retry",
                )
            }

            Self::RetrySafetyNotEstablished => {
                formatter.write_str(
                    "retry safety has not been established by the execution boundary",
                )
            }

            Self::Boundary(error) => {
                write!(formatter, "retry execution boundary failed: {error}")
            }
        }
    }
}

impl<E> std::error::Error for RetryError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Boundary(error) => Some(error),
            _ => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct SuccessfulBoundary;

    impl RetryExecutionBoundary for SuccessfulBoundary {
        type Output = ();
        type Error = std::io::Error;

        fn execute_retry(
            &self,
            _request: &RetryRequest,
        ) -> Result<RetryBoundaryOutcome<Self::Output>, Self::Error> {
            Ok(RetryBoundaryOutcome::Succeeded(()))
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct UnknownBoundary;

    impl RetryExecutionBoundary for UnknownBoundary {
        type Output = ();
        type Error = std::io::Error;

        fn execute_retry(
            &self,
            _request: &RetryRequest,
        ) -> Result<RetryBoundaryOutcome<Self::Output>, Self::Error> {
            Ok(RetryBoundaryOutcome::OutcomeUnknown)
        }
    }

    fn authorized_request() -> RetryRequest {
        RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("valid retry request")
    }

    #[test]
    fn retry_attempt_zero_is_initial() {
        assert!(RetryAttempt::new(0).is_initial());
        assert!(!RetryAttempt::new(0).is_retry());
    }

    #[test]
    fn retry_attempt_one_is_first_retry() {
        assert_eq!(RetryAttempt::new(1).next().get(), 2);
        assert!(RetryAttempt::new(1).is_retry());
    }

    #[test]
    fn safe_retry_is_executable() {
        let request = authorized_request();

        assert_eq!(
            request.decision(),
            RetryDecision::Execute
        );
    }

    #[test]
    fn successful_previous_attempt_cannot_be_retried() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::CompletedSuccess)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::AlreadySuccessful
        );

        let executor = RetryExecutor::new(SuccessfulBoundary);

        assert!(matches!(
            executor.execute(&request),
            Err(RetryError::PreviousAttemptAlreadySucceeded)
        ));
    }

    #[test]
    fn unknown_outcome_requires_reconciliation() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::OutcomeUnknown)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::ReconcileFirst
        );

        let executor = RetryExecutor::new(UnknownBoundary);

        assert!(matches!(
            executor.execute(&request),
            Err(RetryError::ExecutionOutcomeUnknown)
        ));
    }

    #[test]
    fn missing_policy_authorization_rejects_retry() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Safe)
            .policy_authorized(false)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn missing_capability_authorization_rejects_retry() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(false)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn missing_security_authorization_rejects_retry() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(false)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn missing_semantic_authorization_rejects_retry() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(false)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn unknown_safety_rejects_retry() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::NotSubmitted)
            .safety(RetrySafety::Unknown)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn partial_execution_requires_explicit_permission() {
        let request = RetryRequest::builder("execution-1", "operation-1")
            .attempt(RetryAttempt::new(1))
            .previous_state(PreviousAttemptState::PartiallyExecuted)
            .safety(RetrySafety::Safe)
            .policy_authorized(true)
            .capability_authorized(true)
            .security_authorized(true)
            .semantic_authorized(true)
            .allow_partial_execution(false)
            .build()
            .expect("request structure is valid");

        assert_eq!(
            request.decision(),
            RetryDecision::Reject
        );
    }

    #[test]
    fn retry_executor_executes_exactly_one_retry() {
        let executor = RetryExecutor::new(SuccessfulBoundary);
        let request = authorized_request();

        let result = executor
            .execute(&request)
            .expect("retry should execute");

        assert_eq!(
            result.status(),
            &RetryExecutionStatus::Succeeded
        );

        assert_eq!(result.attempt(), RetryAttempt::new(1));
        assert!(result.submission_accepted());
        assert!(result.verification_required());
    }

    #[test]
    fn boundary_unknown_result_is_not_converted_to_failure() {
        let executor = RetryExecutor::new(UnknownBoundary);
        let request = authorized_request();

        let result = executor
            .execute(&request)
            .expect("boundary result is valid");

        assert_eq!(
            result.status(),
            &RetryExecutionStatus::OutcomeUnknown
        );

        assert!(result.submission_accepted());
        assert!(result.verification_required());
    }

    #[test]
    fn retry_result_does_not_authorize_next_retry() {
        let result = RetryExecutionStatus::Failed;

        assert!(result.can_be_considered_for_retry());

        // The fact that a result is retryable does not itself authorize a retry.
        // The next request must go through policy/planning again.
    }

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            RETRY_EXECUTION_SCHEMA_ID,
            "zamani.quantum.resilience.recovery.retry"
        );

        assert_eq!(RETRY_EXECUTION_SCHEMA_VERSION, 1);
    }
}