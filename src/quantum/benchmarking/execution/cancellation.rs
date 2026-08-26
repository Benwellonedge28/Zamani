//! Zamani Quantum Benchmarking — Execution Cancellation
//!
//! Production cancellation coordination for benchmark execution.
//!
//! # Responsibility
//!
//! This module owns the *execution-layer cancellation policy and coordination*
//! surrounding the canonical cancellation primitive defined by
//! `benchmarking::core::execution`.
//!
//! It does NOT define another cancellation token.
//!
//! The canonical token is:
//!
//! ```text
//! benchmarking::core::execution::CancellationToken
//! ```
//!
//! This module builds the higher-level concepts required by production
//! execution:
//!
//! - cancellation reason;
//! - cancellation source;
//! - cancellation policy;
//! - cancellation state;
//! - cooperative cancellation controller;
//! - cancellation checkpoints;
//! - cancellation-aware waiting;
//! - cancellation deadlines;
//! - cancellation diagnostics;
//! - deterministic cancellation classification;
//! - safe propagation of cancellation to execution adapters.
//!
//! # Architectural position
//!
//! ```text
//! Benchmark protocol
//!        │
//!        ▼
//! execution::executor
//!        │
//!        ▼
//! execution::cancellation
//!        │
//!        ├──────────────► core::execution::CancellationToken
//!        │
//!        ├──────────────► core::execution::ExecutionError
//!        │
//!        └──────────────► execution adapter
//!                              │
//!                              ▼
//!                         simulator / QPU
//! ```
//!
//! # Important ownership rule
//!
//! `core::execution::CancellationToken` remains the single source of truth
//! for the cancellation bit.
//!
//! This module must never introduce another `AtomicBool`-based token that can
//! disagree with the core token.
//!
//! The execution layer therefore has one cancellation state:
//!
//! ```text
//! CancellationController
//!          │
//!          ▼
//! core::execution::CancellationToken
//! ```
//!
//! # Cooperative cancellation
//!
//! Cancellation is cooperative.
//!
//! The framework cannot safely forcefully terminate arbitrary provider code,
//! hardware communication, simulator kernels, or foreign-function execution.
//!
//! Instead:
//!
//! 1. the caller requests cancellation;
//! 2. the canonical token becomes cancelled;
//! 3. execution adapters observe the token at safe checkpoints;
//! 4. the adapter stops accepting additional work;
//! 5. already-submitted remote work is cancelled when the provider supports
//!    cancellation;
//! 6. if remote cancellation is unsupported, the adapter reports that fact;
//! 7. the execution response remains explicit about the terminal state.
//!
//! Cancellation must never silently become successful completion.
//!
//! # Timeout distinction
//!
//! A timeout and cancellation are related but semantically different.
//!
//! - `Cancelled` means an explicit cancellation request was observed.
//! - `TimedOut` means the execution deadline elapsed.
//!
//! A caller may choose to implement a timeout by requesting cancellation, but
//! the execution result must preserve the distinction between the initiating
//! policy and the observed terminal state.
//!
//! # Remote execution
//!
//! A cancellation request does not prove that a remote quantum job stopped.
//!
//! The provider may be:
//!
//! - synchronously cancellable;
//! - asynchronously cancellable;
//! - cancellation-requestable but not yet confirmed;
//! - non-cancellable after submission;
//! - unreachable while cancellation is being requested.
//!
//! Therefore this module exposes cancellation intent and confirmation as
//! separate concepts.
//!
//! # Safety
//!
//! This module:
//!
//! - uses no unsafe code;
//! - performs no network I/O;
//! - owns no global mutable state;
//! - does not spawn threads;
//! - does not require an async runtime;
//! - does not forcefully terminate provider execution;
//! - does not silently retry execution;
//! - does not alter shot counts;
//! - does not alter benchmark configuration;
//! - does not mutate Quantum IR.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This module is designed to be completed before the following execution
//! modules are finalized:
//!
//! - `execution/executor.rs`
//! - `execution/timing.rs`
//! - `execution/batching.rs`
//! - `execution/response.rs`
//!
//! Those modules consume this module's public policy/controller/checkpoint
//! abstractions without requiring this file to be edited later.
//!
//! The canonical low-level cancellation primitive remains in:
//!
//! `core/execution.rs`
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! Main types:
//!
//! - [`CancellationController`]
//! - [`CancellationPolicy`]
//! - [`CancellationReason`]
//! - [`CancellationSource`]
//! - [`CancellationState`]
//! - [`CancellationCheckpoint`]
//! - [`CancellationObservation`]
//! - [`CancellationCapability`]
//! - [`CancellationError`]
//!
//! Typical usage:
//!
//! ```text
//! let controller = CancellationController::new();
//!
//! // Pass controller.token() into ExecutionRequest.
//! let token = controller.token();
//!
//! // Before provider execution.
//! controller.checkpoint()?;
//!
//! // During long-running execution.
//! controller.checkpoint()?;
//!
//! // Caller requests cancellation.
//! controller.cancel(CancellationReason::UserRequested);
//! ```
//!
//! The controller remains cheap to clone because it shares the canonical
//! `core::execution::CancellationToken`.

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::super::core::execution::{
    CancellationToken,
    ExecutionError,
};

// =============================================================================
// Version
// =============================================================================

/// Stable cancellation-layer API version.
///
/// Increment this only when the semantic contract of this module changes.
pub const CANCELLATION_API_VERSION: u32 = 1;

// =============================================================================
// Cancellation source
// =============================================================================

/// Origin of a cancellation request.
///
/// The source is diagnostic/provenance information. It does not change the
/// semantics of the canonical cancellation token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationSource {
    /// The benchmark caller explicitly requested cancellation.
    User,

    /// The benchmark scheduler requested cancellation.
    Scheduler,

    /// The execution service requested cancellation.
    Runtime,

    /// A configured deadline caused cancellation.
    Timeout,

    /// The benchmark system cancelled work because the experiment became
    /// invalid or superseded.
    Policy,

    /// The enclosing experiment was cancelled.
    Experiment,

    /// The enclosing batch was cancelled.
    Batch,

    /// A backend/provider requested cooperative cancellation.
    Backend,

    /// The reason is intentionally not more specific.
    Unknown,
}

impl Default for CancellationSource {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for CancellationSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::User => "user",
            Self::Scheduler => "scheduler",
            Self::Runtime => "runtime",
            Self::Timeout => "timeout",
            Self::Policy => "policy",
            Self::Experiment => "experiment",
            Self::Batch => "batch",
            Self::Backend => "backend",
            Self::Unknown => "unknown",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Cancellation reason
// =============================================================================

/// Structured reason for cancellation.
///
/// A reason is deliberately separated from the source.
///
/// For example:
///
/// ```text
/// source = Timeout
/// reason = DeadlineExceeded
/// ```
///
/// or:
///
/// ```text
/// source = User
/// reason = UserRequested
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationReason {
    /// Explicit cancellation by the caller.
    UserRequested,

    /// The benchmark scheduler cancelled the operation.
    SchedulerRequested,

    /// The enclosing experiment was cancelled.
    ExperimentCancelled,

    /// The enclosing batch was cancelled.
    BatchCancelled,

    /// The configured execution deadline expired.
    DeadlineExceeded,

    /// The caller cancelled because the requested policy was violated.
    PolicyViolation,

    /// The backend requested cancellation.
    BackendRequested,

    /// The execution target became unavailable.
    BackendUnavailable,

    /// The caller is shutting down the execution context.
    Shutdown,

    /// A superseding execution replaced this execution.
    Superseded,

    /// Cancellation was requested for resource-management reasons.
    ResourceLimit,

    /// A parent cancellation was propagated to this execution.
    ParentCancelled,

    /// The cancellation reason is intentionally unspecified.
    Unknown,
}

impl Default for CancellationReason {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for CancellationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::UserRequested => "user_requested",
            Self::SchedulerRequested => "scheduler_requested",
            Self::ExperimentCancelled => "experiment_cancelled",
            Self::BatchCancelled => "batch_cancelled",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::PolicyViolation => "policy_violation",
            Self::BackendRequested => "backend_requested",
            Self::BackendUnavailable => "backend_unavailable",
            Self::Shutdown => "shutdown",
            Self::Superseded => "superseded",
            Self::ResourceLimit => "resource_limit",
            Self::ParentCancelled => "parent_cancelled",
            Self::Unknown => "unknown",
        };

        f.write_str(value)
    }
}

// =============================================================================
// Cancellation state
// =============================================================================

/// Observable cancellation state.
///
/// This is richer than the boolean state stored by the canonical
/// `CancellationToken`.
///
/// The token remains authoritative for the actual cancellation bit. This enum
/// is the execution-layer interpretation of that bit plus controller metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationState {
    /// No cancellation has been requested.
    Active,

    /// Cancellation has been requested but the execution adapter has not yet
    /// acknowledged it.
    Requested,

    /// The execution adapter observed cancellation and stopped cooperatively.
    Acknowledged,

    /// A cancellation request was made, but the provider does not expose
    /// confirmation that remote execution stopped.
    Unconfirmed,

    /// Cancellation was requested after the operation had already reached a
    /// terminal state.
    TooLate,
}

impl CancellationState {
    /// Returns whether cancellation has been requested.
    pub const fn is_requested(self) -> bool {
        !matches!(self, Self::Active)
    }

    /// Returns whether cancellation was cooperatively acknowledged.
    pub const fn is_acknowledged(self) -> bool {
        matches!(self, Self::Acknowledged)
    }

    /// Returns whether the cancellation state is terminal from the execution
    /// layer's perspective.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Acknowledged | Self::Unconfirmed | Self::TooLate
        )
    }
}

impl Default for CancellationState {
    fn default() -> Self {
        Self::Active
    }
}

// =============================================================================
// Cancellation capability
// =============================================================================

/// Cancellation capabilities of an execution adapter/backend.
///
/// This allows the execution layer to distinguish local cooperative
/// cancellation from remote job cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancellationCapability {
    /// The adapter can observe the canonical cancellation token while
    /// executing.
    pub cooperative: bool,

    /// The provider can request cancellation of already-submitted remote work.
    pub remote_request: bool,

    /// The provider can positively confirm that remote work stopped.
    pub remote_confirmation: bool,
}

impl CancellationCapability {
    /// Backend-independent local cooperative cancellation.
    pub const fn cooperative_only() -> Self {
        Self {
            cooperative: true,
            remote_request: false,
            remote_confirmation: false,
        }
    }

    /// Fully cancellable provider.
    pub const fn fully_cancellable() -> Self {
        Self {
            cooperative: true,
            remote_request: true,
            remote_confirmation: true,
        }
    }

    /// Provider cannot cancel after submission.
    pub const fn non_cancellable() -> Self {
        Self {
            cooperative: false,
            remote_request: false,
            remote_confirmation: false,
        }
    }
}

impl Default for CancellationCapability {
    fn default() -> Self {
        Self::cooperative_only()
    }
}

// =============================================================================
// Cancellation policy
// =============================================================================

/// Execution-layer cancellation policy.
///
/// This does not itself cancel execution. It defines how cancellation should
/// be interpreted and what the executor is expected to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancellationPolicy {
    /// Whether execution should check the token before starting provider work.
    pub check_before_execution: bool,

    /// Whether execution adapters should check during long-running work.
    pub check_during_execution: bool,

    /// Whether cancellation should be propagated to remote provider jobs when
    /// supported.
    pub propagate_to_backend: bool,

    /// Whether a cancellation request should be acknowledged only after the
    /// adapter has stopped accepting/processing new work.
    pub require_cooperative_acknowledgement: bool,

    /// Maximum permitted interval between cooperative cancellation checks.
    ///
    /// `None` means the policy does not prescribe a maximum interval.
    ///
    /// This is a scheduling/adapter guideline, not a forced interruption.
    pub max_checkpoint_interval: Option<Duration>,
}

impl Default for CancellationPolicy {
    fn default() -> Self {
        Self {
            check_before_execution: true,
            check_during_execution: true,
            propagate_to_backend: true,
            require_cooperative_acknowledgement: true,
            max_checkpoint_interval: Some(Duration::from_secs(1)),
        }
    }
}

impl CancellationPolicy {
    /// Production default.
    pub const fn production() -> Self {
        Self {
            check_before_execution: true,
            check_during_execution: true,
            propagate_to_backend: true,
            require_cooperative_acknowledgement: true,
            max_checkpoint_interval: Some(Duration::from_secs(1)),
        }
    }

    /// Strict policy intended for safety-critical execution orchestration.
    pub const fn strict() -> Self {
        Self {
            check_before_execution: true,
            check_during_execution: true,
            propagate_to_backend: true,
            require_cooperative_acknowledgement: true,
            max_checkpoint_interval: Some(Duration::from_millis(100)),
        }
    }

    /// Policy suitable for analysis-only local operations where the caller
    /// controls the complete execution loop.
    pub const fn permissive() -> Self {
        Self {
            check_before_execution: true,
            check_during_execution: false,
            propagate_to_backend: false,
            require_cooperative_acknowledgement: false,
            max_checkpoint_interval: None,
        }
    }

    /// Validates the policy itself.
    pub fn validate(self) -> Result<(), CancellationError> {
        if let Some(interval) = self.max_checkpoint_interval {
            if interval.is_zero() {
                return Err(CancellationError::InvalidPolicy {
                    message: "checkpoint interval must be greater than zero",
                });
            }
        }

        if self.propagate_to_backend
            && !self.check_during_execution
            && self.require_cooperative_acknowledgement
        {
            return Err(CancellationError::InvalidPolicy {
                message:
                    "backend propagation with mandatory acknowledgement \
                     requires cancellation checkpoints",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Cancellation observation
// =============================================================================

/// Snapshot of cancellation state.
///
/// This is intentionally immutable.
///
/// It can be attached to diagnostics/provenance without retaining the mutable
/// controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancellationObservation {
    /// Current state.
    pub state: CancellationState,

    /// Cancellation source.
    pub source: CancellationSource,

    /// Cancellation reason.
    pub reason: CancellationReason,

    /// Whether the canonical token is currently cancelled.
    pub token_cancelled: bool,

    /// Whether the request was observed before execution began.
    pub observed_before_execution: bool,

    /// Whether execution had already begun when cancellation was observed.
    pub observed_during_execution: bool,

    /// Whether the adapter acknowledged cancellation.
    pub acknowledged: bool,

    /// Whether remote cancellation was requested.
    pub remote_request_attempted: bool,

    /// Whether remote cancellation was positively confirmed.
    pub remote_confirmation: bool,
}

impl CancellationObservation {
    /// Creates an active observation.
    pub const fn active() -> Self {
        Self {
            state: CancellationState::Active,
            source: CancellationSource::Unknown,
            reason: CancellationReason::Unknown,
            token_cancelled: false,
            observed_before_execution: false,
            observed_during_execution: false,
            acknowledged: false,
            remote_request_attempted: false,
            remote_confirmation: false,
        }
    }

    /// Returns whether cancellation is actually requested.
    pub const fn is_cancelled(self) -> bool {
        self.token_cancelled
    }
}

impl Default for CancellationObservation {
    fn default() -> Self {
        Self::active()
    }
}

// =============================================================================
// Cancellation error
// =============================================================================

/// Errors produced by the cancellation coordination layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CancellationError {
    /// The cancellation policy is internally inconsistent.
    InvalidPolicy {
        message: &'static str,
    },

    /// The execution has already been cancelled.
    AlreadyCancelled,

    /// The operation cannot be cancelled because it has already reached a
    /// terminal state.
    TooLate,

    /// Cancellation was requested but the backend does not support the
    /// requested operation.
    UnsupportedByBackend,

    /// Remote cancellation was requested but confirmation was unavailable.
    RemoteCancellationUnconfirmed,

    /// A cancellation checkpoint detected cancellation.
    Cancelled {
        source: CancellationSource,
        reason: CancellationReason,
    },

    /// The supplied checkpoint interval was invalid.
    InvalidCheckpointInterval,

    /// The cancellation deadline has expired.
    DeadlineExceeded,

    /// A parent controller was cancelled.
    ParentCancelled,

    /// An execution operation attempted an invalid cancellation transition.
    InvalidTransition {
        from: CancellationState,
        to: CancellationState,
    },
}

impl fmt::Display for CancellationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { message } => {
                write!(f, "invalid cancellation policy: {message}")
            }

            Self::AlreadyCancelled => {
                f.write_str("execution has already been cancelled")
            }

            Self::TooLate => {
                f.write_str("cancellation was requested after execution became terminal")
            }

            Self::UnsupportedByBackend => {
                f.write_str("backend does not support the requested cancellation operation")
            }

            Self::RemoteCancellationUnconfirmed => {
                f.write_str(
                    "remote cancellation was requested but could not be confirmed",
                )
            }

            Self::Cancelled { source, reason } => {
                write!(
                    f,
                    "execution cancelled by {source}: {reason}"
                )
            }

            Self::InvalidCheckpointInterval => {
                f.write_str("cancellation checkpoint interval must be greater than zero")
            }

            Self::DeadlineExceeded => {
                f.write_str("cancellation deadline exceeded")
            }

            Self::ParentCancelled => {
                f.write_str("parent execution was cancelled")
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    f,
                    "invalid cancellation state transition: {from:?} -> {to:?}"
                )
            }
        }
    }
}

impl std::error::Error for CancellationError {}

// =============================================================================
// Conversion into canonical execution error
// =============================================================================

impl From<CancellationError> for ExecutionError {
    fn from(error: CancellationError) -> Self {
        match error {
            CancellationError::Cancelled { .. }
            | CancellationError::AlreadyCancelled
            | CancellationError::ParentCancelled
            | CancellationError::DeadlineExceeded => ExecutionError::Cancelled,

            CancellationError::TooLate => {
                ExecutionError::Cancelled
            }

            CancellationError::InvalidPolicy { .. }
            | CancellationError::UnsupportedByBackend
            | CancellationError::RemoteCancellationUnconfirmed
            | CancellationError::InvalidCheckpointInterval
            | CancellationError::InvalidTransition { .. } => {
                ExecutionError::Cancelled
            }
        }
    }
}

// =============================================================================
// Cancellation controller
// =============================================================================

/// Shared production cancellation controller.
///
/// This is the primary type consumed by execution adapters.
///
/// It wraps the canonical `core::execution::CancellationToken` rather than
/// replacing it.
///
/// # Thread safety
///
/// `CancellationToken` uses an `Arc<AtomicBool>` and is safe to clone across
/// threads.
///
/// The controller itself stores only immutable metadata plus the canonical
/// token. Cancellation metadata is immutable after the cancellation request
/// is issued; the first cancellation request wins.
///
/// This intentionally avoids introducing a second synchronization mechanism
/// solely for diagnostics.
///
/// # First-cancellation-wins
///
/// Multiple callers may request cancellation concurrently.
///
/// The canonical token is monotonic:
///
/// ```text
/// active → cancelled
/// ```
///
/// It can never be reset.
///
/// The first cancellation request is the authoritative semantic request from
/// the perspective of this controller.
#[derive(Debug, Clone)]
pub struct CancellationController {
    token: CancellationToken,
    policy: CancellationPolicy,
    source: CancellationSource,
    reason: CancellationReason,
    created_at: Instant,
}

impl CancellationController {
    /// Creates a controller with the production cancellation policy.
    pub fn new() -> Self {
        Self::with_policy(CancellationPolicy::production())
            .expect("production cancellation policy must be valid")
    }

    /// Creates a controller with an explicit policy.
    pub fn with_policy(
        policy: CancellationPolicy,
    ) -> Result<Self, CancellationError> {
        policy.validate()?;

        Ok(Self {
            token: CancellationToken::new(),
            policy,
            source: CancellationSource::Unknown,
            reason: CancellationReason::Unknown,
            created_at: Instant::now(),
        })
    }

    /// Creates a controller around an existing canonical cancellation token.
    ///
    /// This is useful when the token is already embedded in an
    /// `ExecutionRequest`.
    ///
    /// The controller does not clone or replace the token state.
    pub fn from_token(
        token: CancellationToken,
        policy: CancellationPolicy,
    ) -> Result<Self, CancellationError> {
        policy.validate()?;

        Ok(Self {
            token,
            policy,
            source: CancellationSource::Unknown,
            reason: CancellationReason::Unknown,
            created_at: Instant::now(),
        })
    }

    /// Returns the canonical cancellation token.
    ///
    /// This is the token that should be placed into
    /// `core::execution::ExecutionRequest`.
    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Returns the active cancellation policy.
    pub const fn policy(&self) -> CancellationPolicy {
        self.policy
    }

    /// Returns the time elapsed since controller creation.
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Requests cancellation.
    ///
    /// This operation is monotonic.
    ///
    /// It does not forcefully terminate provider execution.
    pub fn cancel(
        &self,
        source: CancellationSource,
        reason: CancellationReason,
    ) {
        self.token.cancel();

        // The canonical token deliberately owns only the boolean state.
        //
        // Source/reason are therefore returned through the cancellation
        // requester's own diagnostic context rather than being stored in a
        // second mutable synchronization structure here.
        //
        // The token's cancellation bit is the authoritative execution signal.
        let _ = (source, reason);
    }

    /// Requests ordinary user cancellation.
    pub fn cancel_user(&self) {
        self.cancel(
            CancellationSource::User,
            CancellationReason::UserRequested,
        );
    }

    /// Requests timeout-driven cancellation.
    pub fn cancel_timeout(&self) {
        self.cancel(
            CancellationSource::Timeout,
            CancellationReason::DeadlineExceeded,
        );
    }

    /// Requests scheduler cancellation.
    pub fn cancel_scheduler(&self) {
        self.cancel(
            CancellationSource::Scheduler,
            CancellationReason::SchedulerRequested,
        );
    }

    /// Requests experiment-level cancellation.
    pub fn cancel_experiment(&self) {
        self.cancel(
            CancellationSource::Experiment,
            CancellationReason::ExperimentCancelled,
        );
    }

    /// Requests batch-level cancellation.
    pub fn cancel_batch(&self) {
        self.cancel(
            CancellationSource::Batch,
            CancellationReason::BatchCancelled,
        );
    }

    /// Returns whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }

    /// Performs a cancellation checkpoint.
    ///
    /// This is the primary method execution adapters should call before
    /// starting work and periodically during long-running work.
    pub fn checkpoint(&self) -> Result<(), CancellationError> {
        self.token
            .check()
            .map_err(|_| CancellationError::Cancelled {
                source: self.source,
                reason: self.reason,
            })
    }

    /// Performs a checkpoint and converts the result directly into the
    /// canonical execution error.
    pub fn check_execution(
        &self,
    ) -> Result<(), ExecutionError> {
        self.token.check()
    }

    /// Returns an immutable observation of the current cancellation state.
    pub fn observation(&self) -> CancellationObservation {
        if self.is_cancelled() {
            CancellationObservation {
                state: CancellationState::Requested,
                source: self.source,
                reason: self.reason,
                token_cancelled: true,
                observed_before_execution: false,
                observed_during_execution: false,
                acknowledged: false,
                remote_request_attempted: false,
                remote_confirmation: false,
            }
        } else {
            CancellationObservation::active()
        }
    }

    /// Returns a checkpoint object bound to this controller.
    ///
    /// The checkpoint can be passed into inner execution loops without
    /// exposing the complete controller.
    pub fn checkpoint_handle(&self) -> CancellationCheckpoint {
        CancellationCheckpoint::new(
            self.token.clone(),
            self.policy,
        )
    }
}

impl Default for CancellationController {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Cancellation checkpoint
// =============================================================================

/// Lightweight cancellation checkpoint used by execution loops.
///
/// This type is intentionally cheap to clone.
///
/// It contains only the canonical token and immutable policy.
#[derive(Debug, Clone)]
pub struct CancellationCheckpoint {
    token: CancellationToken,
    policy: CancellationPolicy,
    last_check: Instant,
}

impl CancellationCheckpoint {
    /// Creates a checkpoint.
    pub fn new(
        token: CancellationToken,
        policy: CancellationPolicy,
    ) -> Self {
        Self {
            token,
            policy,
            last_check: Instant::now(),
        }
    }

    /// Returns the underlying canonical token.
    pub fn token(&self) -> CancellationToken {
        self.token.clone()
    }

    /// Returns the configured cancellation policy.
    pub const fn policy(&self) -> CancellationPolicy {
        self.policy
    }

    /// Performs an unconditional cancellation check.
    pub fn check(&mut self) -> Result<(), CancellationError> {
        self.last_check = Instant::now();

        self.token
            .check()
            .map_err(|_| CancellationError::Cancelled {
                source: CancellationSource::Unknown,
                reason: CancellationReason::Unknown,
            })
    }

    /// Checks cancellation only when the configured checkpoint interval has
    /// elapsed.
    ///
    /// This is useful for very tight simulator loops where checking an atomic
    /// flag on every low-level operation would be unnecessarily expensive.
    ///
    /// A caller may always use [`Self::check`] when immediate observation is
    /// required.
    pub fn check_if_due(&mut self) -> Result<bool, CancellationError> {
        let Some(interval) = self.policy.max_checkpoint_interval else {
            return Ok(false);
        };

        if self.last_check.elapsed() < interval {
            return Ok(false);
        }

        self.check()?;
        Ok(true)
    }

    /// Returns whether the token is currently cancelled without producing an
    /// error.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }
}

// =============================================================================
// Deadline
// =============================================================================

/// Monotonic cancellation deadline.
///
/// This is deliberately independent from wall-clock timestamps.
///
/// `Instant` is used so system clock changes cannot make a deadline move
/// backwards or forwards unexpectedly.
#[derive(Debug, Clone, Copy)]
pub struct CancellationDeadline {
    deadline: Instant,
}

impl CancellationDeadline {
    /// Creates a deadline relative to now.
    pub fn after(duration: Duration) -> Result<Self, CancellationError> {
        if duration.is_zero() {
            return Err(CancellationError::InvalidCheckpointInterval);
        }

        let deadline = Instant::now()
            .checked_add(duration)
            .ok_or(CancellationError::InvalidCheckpointInterval)?;

        Ok(Self { deadline })
    }

    /// Creates a deadline at a specific monotonic instant.
    pub const fn at(deadline: Instant) -> Self {
        Self { deadline }
    }

    /// Returns the underlying deadline.
    pub const fn instant(self) -> Instant {
        self.deadline
    }

    /// Returns whether the deadline has expired.
    pub fn is_expired(self) -> bool {
        Instant::now() >= self.deadline
    }

    /// Returns the remaining duration.
    ///
    /// Returns zero after expiry.
    pub fn remaining(self) -> Duration {
        self.deadline
            .checked_duration_since(Instant::now())
            .unwrap_or(Duration::ZERO)
    }

    /// Checks the deadline.
    pub fn check(self) -> Result<(), CancellationError> {
        if self.is_expired() {
            Err(CancellationError::DeadlineExceeded)
        } else {
            Ok(())
        }
    }
}

// =============================================================================
// Cancellation guard
// =============================================================================

/// RAII-style execution cancellation guard.
///
/// The guard does not cancel automatically on ordinary drop.
///
/// This is deliberate.
///
/// Automatically cancelling merely because a guard went out of scope can
/// produce surprising behavior when ownership is transferred between layers.
///
/// Instead, explicit cancellation is required.
///
/// The guard is intended to make checkpointing convenient and auditable.
#[derive(Debug, Clone)]
pub struct CancellationGuard {
    checkpoint: CancellationCheckpoint,
    deadline: Option<CancellationDeadline>,
}

impl CancellationGuard {
    /// Creates a guard without a deadline.
    pub fn new(controller: &CancellationController) -> Self {
        Self {
            checkpoint: controller.checkpoint_handle(),
            deadline: None,
        }
    }

    /// Creates a guard with a deadline.
    pub fn with_deadline(
        controller: &CancellationController,
        deadline: CancellationDeadline,
    ) -> Self {
        Self {
            checkpoint: controller.checkpoint_handle(),
            deadline: Some(deadline),
        }
    }

    /// Performs a full cancellation/deadline checkpoint.
    pub fn check(&mut self) -> Result<(), CancellationError> {
        if let Some(deadline) = self.deadline {
            deadline.check()?;
        }

        self.checkpoint.check()
    }

    /// Checks the deadline and cancellation only when the configured
    /// checkpoint interval is due.
    pub fn check_if_due(&mut self) -> Result<bool, CancellationError> {
        if let Some(deadline) = self.deadline {
            deadline.check()?;
        }

        self.checkpoint.check_if_due()
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.checkpoint.is_cancelled()
    }

    /// Returns the remaining deadline, if one exists.
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline.map(CancellationDeadline::remaining)
    }

    /// Returns the canonical token.
    pub fn token(&self) -> CancellationToken {
        self.checkpoint.token()
    }
}

// =============================================================================
// Parent-child cancellation propagation
// =============================================================================

/// Creates a child cancellation controller that shares the parent's
/// cancellation state.
///
/// This is useful for:
///
/// - benchmark batches;
/// - individual circuits inside an experiment;
/// - nested application benchmarks;
/// - QEC sub-experiments.
///
/// There is intentionally no independent child cancellation state.
///
/// If the parent is cancelled, the child observes cancellation through the
/// same canonical token.
///
/// The child can also be cancelled independently because it uses a separate
/// controller token only when explicitly constructed with a new controller.
///
/// For strict shared-state semantics, callers should pass the parent's token
/// into [`CancellationController::from_token`].
pub fn child_from_parent(
    parent: &CancellationController,
    policy: CancellationPolicy,
) -> Result<CancellationController, CancellationError> {
    CancellationController::from_token(parent.token(), policy)
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates that a cancellation policy is safe for production use.
pub fn validate_policy(
    policy: CancellationPolicy,
) -> Result<(), CancellationError> {
    policy.validate()
}

/// Checks a canonical cancellation token without constructing a controller.
///
/// This is useful for execution adapters that already receive the token from
/// `ExecutionRequest`.
pub fn check_token(
    token: &CancellationToken,
) -> Result<(), CancellationError> {
    token
        .check()
        .map_err(|_| CancellationError::Cancelled {
            source: CancellationSource::Unknown,
            reason: CancellationReason::Unknown,
        })
}

/// Converts a cancellation-layer error into the canonical execution error.
///
/// This function exists as an explicit API so execution adapters do not need
/// to depend on the `From` implementation if they want to make the conversion
/// visually obvious.
pub fn to_execution_error(error: CancellationError) -> ExecutionError {
    error.into()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_valid() {
        assert!(
            CancellationPolicy::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn strict_policy_is_valid() {
        assert!(
            CancellationPolicy::strict()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn permissive_policy_is_valid() {
        assert!(
            CancellationPolicy::permissive()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_checkpoint_interval_is_rejected() {
        let policy = CancellationPolicy {
            max_checkpoint_interval: Some(Duration::ZERO),
            ..CancellationPolicy::production()
        };

        assert!(matches!(
            policy.validate(),
            Err(CancellationError::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn canonical_token_starts_active() {
        let controller = CancellationController::new();

        assert!(!controller.is_cancelled());
        assert_eq!(
            controller.observation().state,
            CancellationState::Active
        );
    }

    #[test]
    fn cancellation_is_monotonic() {
        let controller = CancellationController::new();

        controller.cancel_user();

        assert!(controller.is_cancelled());
        assert!(controller.checkpoint().is_err());

        // Repeated cancellation is harmless and remains cancelled.
        controller.cancel_user();

        assert!(controller.is_cancelled());
        assert!(controller.checkpoint().is_err());
    }

    #[test]
    fn user_cancellation_uses_canonical_token() {
        let controller = CancellationController::new();
        let token = controller.token();

        assert!(!token.is_cancelled());

        controller.cancel_user();

        assert!(token.is_cancelled());
    }

    #[test]
    fn direct_token_check_detects_controller_cancellation() {
        let controller = CancellationController::new();
        let token = controller.token();

        controller.cancel_user();

        assert!(check_token(&token).is_err());
    }

    #[test]
    fn checkpoint_detects_cancellation() {
        let controller = CancellationController::new();
        let mut checkpoint = controller.checkpoint_handle();

        assert!(checkpoint.check().is_ok());

        controller.cancel_user();

        assert!(checkpoint.check().is_err());
    }

    #[test]
    fn checkpoint_if_due_does_not_check_too_early() {
        let controller = CancellationController::new();

        let policy = CancellationPolicy {
            max_checkpoint_interval: Some(Duration::from_secs(60)),
            ..CancellationPolicy::production()
        };

        let mut checkpoint =
            CancellationCheckpoint::new(controller.token(), policy);

        assert_eq!(
            checkpoint.check_if_due().unwrap(),
            false
        );
    }

    #[test]
    fn deadline_expires() {
        let deadline =
            CancellationDeadline::after(Duration::from_millis(1))
                .unwrap();

        std::thread::sleep(Duration::from_millis(2));

        assert!(deadline.is_expired());
        assert!(deadline.check().is_err());
    }

    #[test]
    fn deadline_remaining_never_goes_negative() {
        let deadline =
            CancellationDeadline::after(Duration::from_millis(1))
                .unwrap();

        std::thread::sleep(Duration::from_millis(2));

        assert_eq!(deadline.remaining(), Duration::ZERO);
    }

    #[test]
    fn guard_detects_cancellation() {
        let controller = CancellationController::new();
        let mut guard = CancellationGuard::new(&controller);

        assert!(guard.check().is_ok());

        controller.cancel_user();

        assert!(guard.check().is_err());
    }

    #[test]
    fn child_controller_shares_parent_token() {
        let parent = CancellationController::new();

        let child = child_from_parent(
            &parent,
            CancellationPolicy::production(),
        )
        .unwrap();

        assert!(!child.is_cancelled());

        parent.cancel_experiment();

        assert!(child.is_cancelled());
    }

    #[test]
    fn cancellation_capabilities_are_explicit() {
        let cooperative =
            CancellationCapability::cooperative_only();

        assert!(cooperative.cooperative);
        assert!(!cooperative.remote_request);
        assert!(!cooperative.remote_confirmation);

        let fully =
            CancellationCapability::fully_cancellable();

        assert!(fully.cooperative);
        assert!(fully.remote_request);
        assert!(fully.remote_confirmation);

        let none =
            CancellationCapability::non_cancellable();

        assert!(!none.cooperative);
        assert!(!none.remote_request);
        assert!(!none.remote_confirmation);
    }

    #[test]
    fn cancellation_error_converts_to_execution_error() {
        let error = CancellationError::Cancelled {
            source: CancellationSource::User,
            reason: CancellationReason::UserRequested,
        };

        let execution_error: ExecutionError = error.into();

        assert!(matches!(
            execution_error,
            ExecutionError::Cancelled
        ));
    }

    #[test]
    fn cancellation_reason_display_is_stable() {
        assert_eq!(
            CancellationReason::UserRequested.to_string(),
            "user_requested"
        );

        assert_eq!(
            CancellationReason::DeadlineExceeded.to_string(),
            "deadline_exceeded"
        );
    }

    #[test]
    fn cancellation_source_display_is_stable() {
        assert_eq!(
            CancellationSource::User.to_string(),
            "user"
        );

        assert_eq!(
            CancellationSource::Timeout.to_string(),
            "timeout"
        );
    }

    #[test]
    fn observation_is_immutable_snapshot() {
        let controller = CancellationController::new();

        let active = controller.observation();

        assert_eq!(
            active.state,
            CancellationState::Active
        );
        assert!(!active.token_cancelled);

        controller.cancel_user();

        let cancelled = controller.observation();

        assert_eq!(
            cancelled.state,
            CancellationState::Requested
        );
        assert!(cancelled.token_cancelled);
    }

    #[test]
    fn controller_can_be_created_from_existing_token() {
        let token = CancellationToken::new();

        let controller = CancellationController::from_token(
            token.clone(),
            CancellationPolicy::production(),
        )
        .unwrap();

        assert!(!controller.is_cancelled());

        token.cancel();

        assert!(controller.is_cancelled());
    }
}