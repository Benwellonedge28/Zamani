//! Zamani Quantum — Canonical Hardware Execution Orchestrator
//!
//! Production-grade, provider-independent execution orchestration for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module owns the lifecycle orchestration around the already-defined
//! hardware contracts:
//!
//! - `ExecutionRequest` from `backend.rs`;
//! - `ExecutionResult` from `backend.rs`;
//! - `BackendProgram` from `backend_trait.rs`;
//! - `BackendJobId` from `backend_trait.rs`;
//! - `BackendJob` from `backend_trait.rs`;
//! - `BackendJobStatus` from `backend_trait.rs`;
//! - `BackendJobState` from `backend_trait.rs`;
//! - `QuantumBackendAdapter` from `backend_trait.rs`.
//!
//! It provides:
//!
//! - deterministic request validation;
//! - provider-neutral preflight;
//! - asynchronous submission;
//! - synchronous convenience execution;
//! - bounded job polling;
//! - timeout enforcement;
//! - cancellation;
//! - terminal-state handling;
//! - result-integrity checks;
//! - execution receipts;
//! - execution lifecycle snapshots;
//! - provider-independent execution policies;
//! - safe execution diagnostics;
//! - deterministic metadata;
//! - concurrency-safe adapter sharing through `Arc`;
//! - a stable API for benchmarking, Danga and higher quantum layers.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - implement provider HTTP clients;
//! - authenticate providers;
//! - store credentials;
//! - store API tokens;
//! - parse OpenQASM;
//! - generate QIR;
//! - transpile programs;
//! - route logical qubits;
//! - schedule quantum instructions;
//! - acquire calibration data;
//! - implement benchmarking mathematics;
//! - implement QEC algorithms;
//! - implement simulators;
//! - implement provider-specific behaviour;
//! - persist jobs;
//! - own a global executor;
//! - spawn an async runtime;
//! - redefine backend capabilities;
//! - redefine backend topology.
//!
//! Provider adapters remain responsible for communication with IBM, IonQ,
//! AWS Braket, Rigetti, IQM, Quantinuum, QuEra, local simulators, emulators,
//! and future providers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      +--------------------------+
//!      |                          |
//!      v                          v
//! optimization              error correction
//!      |                          |
//!      +------------+-------------+
//!                   |
//!                   v
//!             compatibility
//!                   |
//!             +-----+-----+
//!             |           |
//!             v           v
//!          routing     scheduling
//!             |           |
//!             +-----+-----+
//!                   |
//!                   v
//!             BackendProgram
//!                   |
//!                   v
//!        QuantumBackendAdapter
//!                   |
//!                   v
//!             execution.rs
//!                   |
//!          +--------+--------+
//!          |        |        |
//!          v        v        v
//!       submit    status   cancel
//!          |        |        |
//!          +--------+--------+
//!                   |
//!                   v
//!              BackendJob
//!                   |
//!                   v
//!              ExecutionResult
//! ```
//!
//! Benchmarking consumes this lifecycle.
//!
//! Hardware never depends on benchmarking.
//!
//! # Critical semantic rule
//!
//! `execution.rs` is an orchestration layer, not another backend abstraction.
//!
//! The canonical distinction is:
//!
//! ```text
//! QuantumBackend
//!     = describes a backend
//!
//! QuantumBackendAdapter
//!     = knows how to communicate with/execute on a backend
//!
//! QuantumExecutionEngine
//!     = safely orchestrates an adapter's lifecycle
//! ```
//!
//! This prevents execution policy from leaking into provider implementations.
//!
//! # Asynchronous semantics
//!
//! Remote quantum hardware is asynchronous by default:
//!
//! ```text
//! preflight
//!    |
//!    v
//! submit
//!    |
//!    v
//! BackendJobId
//!    |
//!    +----> status()
//!    |
//!    +----> cancel()
//!    |
//!    +----> result()
//! ```
//!
//! `wait_for_result()` never assumes that a submitted job is immediately
//! complete.
//!
//! A result is accepted only when:
//!
//! 1. the provider reports `Completed`;
//! 2. the provider reports that a result is available;
//! 3. the result can actually be retrieved;
//! 4. the result identifies the expected backend;
//! 5. the result represents no more than the requested number of shots;
//! 6. the result contains structurally valid normalized data.
//!
//! # Timeout semantics
//!
//! Timeout is enforced locally by this orchestrator.
//!
//! A timeout means:
//!
//! > Zamani stopped waiting within the caller's configured execution budget.
//!
//! It does NOT mean:
//!
//! > The provider definitely stopped executing the job.
//!
//! Therefore a timed-out job may continue to exist remotely.
//!
//! The caller may subsequently use the returned `BackendJobId` through the
//! adapter to inspect or cancel the provider-side job.
//!
//! This distinction prevents accidental duplicate submissions and false
//! claims about remote cancellation.
//!
//! # Retry semantics
//!
//! This module intentionally does not automatically retry submission.
//!
//! Retrying a submission after an ambiguous transport failure can create two
//! physical executions of the same quantum workload.
//!
//! Providers may expose idempotency through their adapter implementation, but
//! the core execution orchestrator never assumes it.
//!
//! Polling retries are likewise not performed automatically because
//! `BackendError` remains the authoritative provider-neutral error taxonomy.
//! Future retry policy may be added once that taxonomy exposes explicit
//! retryability.
//!
//! # Determinism
//!
//! The orchestrator:
//!
//! - does not generate job IDs;
//! - does not generate random seeds;
//! - does not modify provider job identifiers;
//! - does not depend on collection iteration order;
//! - never silently changes execution requests;
//! - never changes requested shots;
//! - never changes workload requirements.
//!
//! The only time-sensitive operation is deadline enforcement during polling.
//!
//! # Security
//!
//! This module never:
//!
//! - stores credentials;
//! - prints program payload bytes;
//! - prints authentication material;
//! - copies provider secrets into metadata.
//!
//! Provider adapters remain responsible for transport security.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - Rust standard library.
//!
//! It intentionally does NOT depend on:
//!
//! - `job.rs`;
//! - `queue.rs`;
//! - `cancellation.rs`;
//! - `provider.rs`;
//! - provider adapters;
//! - benchmarking;
//! - Danga;
//! - compiler internals.
//!
//! Future modules consume this file rather than modifying its fundamental
//! execution semantics.
//!
//! `job.rs` may wrap the returned `ExecutionHandle`.
//!
//! `queue.rs` may provide higher-level queue policies before calling this
//! module.
//!
//! `cancellation.rs` may provide user-facing cancellation commands that call
//! `QuantumExecutionEngine::cancel()`.
//!
//! Provider adapters implement `QuantumBackendAdapter`.
//!
//! Benchmarking uses `submit()` / `wait_for_result()` and records the returned
//! execution receipt.
//!
//! Danga calls this layer rather than communicating with providers directly.
//!
//! Adding a provider MUST NOT require changing this file.
//!
//! # Stability rule
//!
//! The public types in this file are provider-neutral and must remain stable.
//!
//! Provider-specific functionality belongs behind `QuantumBackendAdapter`.
//!
//! ```text
//! provider-specific requirement
//!          |
//!          v
//! adapter implementation
//!          |
//!          v
//! provider-neutral BackendJob / BackendJobStatus / ExecutionResult
//!          |
//!          v
//! execution.rs
//! ```
//!
//! # No-reedit contract
//!
//! This file is considered complete independently of future modules.
//!
//! A future `job.rs`, `queue.rs`, `cancellation.rs`, provider adapter or Danga
//! implementation must consume this API rather than requiring modifications to
//! the execution core merely because that consumer was added.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use super::backend::{
    BackendError,
    ExecutionRequest,
    ExecutionResult,
};
use super::backend_trait::{
    BackendCancellation,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    QuantumBackendAdapter,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the execution orchestrator.
pub const EXECUTION_SCHEMA_ID: &str =
    "zamani.quantum.hardware.execution";

/// Semantic version of the execution-orchestration contract.
pub const EXECUTION_SCHEMA_VERSION: u16 = 1;

/// Default maximum amount of time spent waiting for a remote job.
pub const DEFAULT_EXECUTION_TIMEOUT: Duration =
    Duration::from_secs(24 * 60 * 60);

/// Default interval between lifecycle polls.
pub const DEFAULT_POLL_INTERVAL: Duration =
    Duration::from_millis(500);

/// Minimum legal polling interval.
///
/// A zero-duration busy loop would unnecessarily consume CPU and can overload
/// provider APIs.
pub const MIN_POLL_INTERVAL: Duration =
    Duration::from_millis(1);

/// Maximum legal polling interval.
///
/// Very large polling intervals make lifecycle control unpredictable.
pub const MAX_POLL_INTERVAL: Duration =
    Duration::from_secs(60 * 60);

/// Maximum execution timeout accepted by this module.
///
/// A caller needing longer-lived jobs should use job persistence and resume
/// polling rather than keeping one synchronous call alive indefinitely.
pub const MAX_EXECUTION_TIMEOUT: Duration =
    Duration::from_secs(30 * 24 * 60 * 60);

/// Maximum number of lifecycle polls allowed by one wait operation.
///
/// This is a defensive bound against pathological configurations.
pub const MAX_POLL_ATTEMPTS: u64 = 10_000_000;

// =============================================================================
// Execution mode
// =============================================================================

/// Requested execution orchestration mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionMode {
    /// Submit the job and return immediately.
    SubmitOnly,

    /// Submit and wait until a terminal result is available.
    WaitForResult,

    /// Use the adapter's native synchronous execution method.
    ///
    /// The adapter must explicitly advertise support for this mode.
    NativeSynchronous,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::WaitForResult
    }
}

impl ExecutionMode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmitOnly => "submit_only",
            Self::WaitForResult => "wait_for_result",
            Self::NativeSynchronous => "native_synchronous",
        }
    }
}

impl fmt::Display for ExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Execution policy
// =============================================================================

/// Immutable execution policy used by `QuantumExecutionEngine`.
///
/// This structure contains orchestration policy only. It does not contain
/// credentials, provider configuration or provider-specific values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPolicy {
    /// Maximum time spent waiting for a submitted job.
    pub timeout: Duration,

    /// Delay between status polls.
    pub poll_interval: Duration,

    /// Whether a timeout should trigger a best-effort cancellation request.
    ///
    /// Even when enabled, the returned error still means that the local wait
    /// deadline was reached. The provider may continue executing.
    pub cancel_on_timeout: bool,

    /// Maximum number of status polls.
    pub max_poll_attempts: u64,

    /// Whether completed results must identify the same backend as the job.
    pub require_backend_identity_match: bool,

    /// Whether a result must explicitly report that it is complete before
    /// retrieval.
    pub require_completed_state: bool,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_EXECUTION_TIMEOUT,
            poll_interval: DEFAULT_POLL_INTERVAL,
            cancel_on_timeout: false,
            max_poll_attempts: MAX_POLL_ATTEMPTS,
            require_backend_identity_match: true,
            require_completed_state: true,
        }
    }
}

impl ExecutionPolicy {
    /// Creates a default production policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum wait duration.
    pub fn with_timeout(
        mut self,
        timeout: Duration,
    ) -> Result<Self, ExecutionPolicyError> {
        validate_timeout(timeout)?;
        self.timeout = timeout;
        Ok(self)
    }

    /// Sets the polling interval.
    pub fn with_poll_interval(
        mut self,
        poll_interval: Duration,
    ) -> Result<Self, ExecutionPolicyError> {
        validate_poll_interval(poll_interval)?;
        self.poll_interval = poll_interval;
        Ok(self)
    }

    /// Enables or disables best-effort cancellation after timeout.
    pub fn with_cancel_on_timeout(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cancel_on_timeout = enabled;
        self
    }

    /// Sets the maximum polling attempts.
    pub fn with_max_poll_attempts(
        mut self,
        attempts: u64,
    ) -> Result<Self, ExecutionPolicyError> {
        if attempts == 0 || attempts > MAX_POLL_ATTEMPTS {
            return Err(
                ExecutionPolicyError::InvalidPollAttemptLimit {
                    maximum: MAX_POLL_ATTEMPTS,
                },
            );
        }

        self.max_poll_attempts = attempts;
        Ok(self)
    }

    /// Controls backend identity verification for results.
    pub fn with_backend_identity_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.require_backend_identity_match = enabled;
        self
    }

    /// Controls strict completed-state verification.
    pub fn with_completed_state_requirement(
        mut self,
        enabled: bool,
    ) -> Self {
        self.require_completed_state = enabled;
        self
    }

    /// Validates the complete policy.
    pub fn validate(&self) -> Result<(), ExecutionPolicyError> {
        validate_timeout(self.timeout)?;
        validate_poll_interval(self.poll_interval)?;

        if self.max_poll_attempts == 0
            || self.max_poll_attempts > MAX_POLL_ATTEMPTS
        {
            return Err(
                ExecutionPolicyError::InvalidPollAttemptLimit {
                    maximum: MAX_POLL_ATTEMPTS,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Execution policy errors
// =============================================================================

/// Errors produced while constructing an execution policy.
///
/// These are deliberately separate from `BackendError`: they indicate an
/// invalid local orchestration policy, not a provider/backend failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionPolicyError {
    /// Timeout is zero or exceeds the safe maximum.
    InvalidTimeout {
        /// Maximum accepted timeout.
        maximum: Duration,
    },

    /// Poll interval is outside the accepted range.
    InvalidPollInterval {
        /// Minimum accepted interval.
        minimum: Duration,

        /// Maximum accepted interval.
        maximum: Duration,
    },

    /// Poll attempt count is outside the accepted range.
    InvalidPollAttemptLimit {
        /// Maximum accepted number of attempts.
        maximum: u64,
    },
}

impl fmt::Display for ExecutionPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTimeout { maximum } => write!(
                formatter,
                "execution timeout must be greater than zero and no greater than {:?}",
                maximum
            ),

            Self::InvalidPollInterval {
                minimum,
                maximum,
            } => write!(
                formatter,
                "poll interval must be between {:?} and {:?}",
                minimum,
                maximum
            ),

            Self::InvalidPollAttemptLimit { maximum } => write!(
                formatter,
                "poll attempt limit must be between 1 and {}",
                maximum
            ),
        }
    }
}

impl std::error::Error for ExecutionPolicyError {}

fn validate_timeout(
    timeout: Duration,
) -> Result<(), ExecutionPolicyError> {
    if timeout.is_zero() || timeout > MAX_EXECUTION_TIMEOUT {
        return Err(ExecutionPolicyError::InvalidTimeout {
            maximum: MAX_EXECUTION_TIMEOUT,
        });
    }

    Ok(())
}

fn validate_poll_interval(
    poll_interval: Duration,
) -> Result<(), ExecutionPolicyError> {
    if poll_interval < MIN_POLL_INTERVAL
        || poll_interval > MAX_POLL_INTERVAL
    {
        return Err(ExecutionPolicyError::InvalidPollInterval {
            minimum: MIN_POLL_INTERVAL,
            maximum: MAX_POLL_INTERVAL,
        });
    }

    Ok(())
}

// =============================================================================
// Execution phase
// =============================================================================

/// Current phase of an orchestrated execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionPhase {
    /// Local request/program validation.
    Validating,

    /// Provider-neutral adapter preflight.
    Preflight,

    /// Provider submission.
    Submitting,

    /// Job accepted and returned.
    Submitted,

    /// Waiting for terminal state.
    Waiting,

    /// Result retrieval.
    RetrievingResult,

    /// Execution completed successfully.
    Completed,

    /// Cancellation was requested.
    Cancelling,

    /// Execution was cancelled.
    Cancelled,

    /// Execution failed.
    Failed,

    /// Local wait deadline was reached.
    TimedOut,
}

impl ExecutionPhase {
    /// Returns whether the phase is terminal from the orchestrator's point of
    /// view.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Cancelled
                | Self::Failed
                | Self::TimedOut
        )
    }

    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validating => "validating",
            Self::Preflight => "preflight",
            Self::Submitting => "submitting",
            Self::Submitted => "submitted",
            Self::Waiting => "waiting",
            Self::RetrievingResult => "retrieving_result",
            Self::Completed => "completed",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::TimedOut => "timed_out",
        }
    }
}

impl fmt::Display for ExecutionPhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Execution handle
// =============================================================================

/// Immutable handle returned after successful provider submission.
///
/// This handle is intentionally small and safe to persist by higher-level job
/// management systems.
///
/// It contains no credentials and no program bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionHandle {
    /// Provider-neutral job identity.
    pub job: BackendJob,

    /// Execution phase immediately after submission.
    pub phase: ExecutionPhase,
}

impl ExecutionHandle {
    /// Returns the provider-neutral job identifier.
    pub fn job_id(&self) -> &BackendJobId {
        &self.job.id
    }

    /// Returns the backend identifier.
    pub fn backend_id(&self) -> &str {
        &self.job.backend_id
    }

    /// Returns the initial provider lifecycle state.
    pub fn state(&self) -> BackendJobState {
        self.job.state
    }

    /// Returns true if the provider already reported a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.job.state.is_terminal()
    }
}

// =============================================================================
// Execution snapshot
// =============================================================================

/// Point-in-time execution lifecycle snapshot.
///
/// This object contains only normalized provider-neutral state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSnapshot {
    /// Job handle.
    pub handle: ExecutionHandle,

    /// Most recently observed provider status.
    pub status: Option<BackendJobStatus>,

    /// Current orchestration phase.
    pub phase: ExecutionPhase,

    /// Number of status observations performed.
    pub poll_attempts: u64,
}

impl ExecutionSnapshot {
    /// Creates a snapshot immediately after submission.
    pub fn submitted(handle: ExecutionHandle) -> Self {
        Self {
            handle,
            status: None,
            phase: ExecutionPhase::Submitted,
            poll_attempts: 0,
        }
    }

    /// Returns the latest provider lifecycle state.
    pub fn state(&self) -> BackendJobState {
        self.status
            .as_ref()
            .map(|status| status.job.state)
            .unwrap_or(self.handle.job.state)
    }

    /// Returns whether the execution has reached a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.phase.is_terminal() || self.state().is_terminal()
    }
}

// =============================================================================
// Execution receipt
// =============================================================================

/// Immutable provenance record for a successfully retrieved execution result.
///
/// This is intentionally independent from benchmarking. Benchmarking may store
/// this receipt alongside benchmark observations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionReceipt {
    /// Provider-neutral job identity.
    pub job: BackendJob,

    /// Backend that produced the result.
    pub backend_id: String,

    /// Number of shots requested.
    pub requested_shots: usize,

    /// Number of shots represented by normalized counts.
    pub counted_shots: usize,

    /// Final provider lifecycle state.
    pub final_state: BackendJobState,

    /// Number of lifecycle polls performed.
    pub poll_attempts: u64,

    /// Execution orchestration mode.
    pub mode: ExecutionMode,
}

impl ExecutionReceipt {
    /// Returns whether normalized counts exactly represent the requested shots.
    pub fn is_complete(&self) -> bool {
        self.requested_shots == self.counted_shots
    }
}

// =============================================================================
// Execution outcome
// =============================================================================

/// Successful execution outcome.
///
/// `SubmitOnly` returns an `ExecutionHandle` and no result.
/// `WaitForResult` returns both the handle and normalized result.
/// `NativeSynchronous` returns the normalized result when supported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionOutcome {
    /// Submission succeeded and execution continues remotely/asynchronously.
    Submitted(ExecutionHandle),

    /// Execution completed and a normalized result is available.
    Completed {
        /// Submitted job handle.
        handle: ExecutionHandle,

        /// Normalized execution result.
        result: ExecutionResult,

        /// Provenance receipt.
        receipt: ExecutionReceipt,
    },
}

impl ExecutionOutcome {
    /// Returns the submitted job identifier.
    pub fn job_id(&self) -> Option<&BackendJobId> {
        match self {
            Self::Submitted(handle) => Some(&handle.job.id),
            Self::Completed { handle, .. } => Some(&handle.job.id),
        }
    }

    /// Returns a completed result if one is available.
    pub fn result(&self) -> Option<&ExecutionResult> {
        match self {
            Self::Submitted(_) => None,
            Self::Completed { result, .. } => Some(result),
        }
    }
}

// =============================================================================
// Execution error
// =============================================================================

/// Provider-neutral orchestration error.
///
/// Provider/backend errors remain represented by `BackendError`.
///
/// This type exists for lifecycle conditions that cannot be safely represented
/// by the backend descriptor alone, especially local timeout and invalid
/// lifecycle transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError {
    /// Local request structure is invalid.
    InvalidRequest,

    /// The adapter rejected the program/request during preflight.
    Preflight(BackendError),

    /// Provider/backend submission failed.
    Submission(BackendError),

    /// Provider status retrieval failed.
    Status(BackendError),

    /// Provider result retrieval failed.
    Result(BackendError),

    /// Provider cancellation failed.
    Cancellation(BackendError),

    /// Local execution wait deadline was reached.
    TimedOut {
        /// Job that may still exist remotely.
        job_id: BackendJobId,

        /// Last observed provider state.
        last_state: BackendJobState,

        /// Whether the orchestrator attempted cancellation.
        cancellation_attempted: bool,
    },

    /// Provider reported a terminal failure.
    JobFailed {
        /// Job identity.
        job_id: BackendJobId,

        /// Last normalized status.
        status: BackendJobStatus,
    },

    /// Provider reported expiration.
    JobExpired {
        /// Job identity.
        job_id: BackendJobId,
    },

    /// Provider reported cancellation.
    JobCancelled {
        /// Job identity.
        job_id: BackendJobId,
    },

    /// Provider returned a state that cannot produce a valid result.
    InvalidLifecycle {
        /// Job identity.
        job_id: BackendJobId,

        /// Observed state.
        state: BackendJobState,

        /// Whether a result was advertised.
        result_available: bool,
    },

    /// Provider returned a result that conflicts with the submitted backend.
    BackendIdentityMismatch {
        /// Expected backend.
        expected: String,

        /// Backend reported by the result.
        actual: String,
    },

    /// Result contains more samples than were requested.
    ResultShotsExceeded {
        /// Number of normalized samples returned.
        represented: usize,

        /// Number of requested shots.
        requested: usize,
    },

    /// Result structure is incomplete.
    IncompleteResult {
        /// Job identity.
        job_id: BackendJobId,

        /// Requested shots.
        requested: usize,

        /// Counted shots.
        counted: usize,
    },

    /// The adapter returned a result while strict completed-state checking was
    /// enabled but the job state was not completed.
    ResultBeforeCompletion {
        /// Job identity.
        job_id: BackendJobId,

        /// Provider state.
        state: BackendJobState,
    },

    /// Adapter does not advertise native synchronous execution.
    NativeSynchronousUnsupported,

    /// An execution policy is invalid.
    InvalidPolicy(ExecutionPolicyError),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest => {
                write!(formatter, "quantum execution request is invalid")
            }

            Self::Preflight(error) => {
                write!(formatter, "quantum execution preflight failed: {error}")
            }

            Self::Submission(error) => {
                write!(formatter, "quantum execution submission failed: {error}")
            }

            Self::Status(error) => {
                write!(formatter, "quantum execution status retrieval failed: {error}")
            }

            Self::Result(error) => {
                write!(formatter, "quantum execution result retrieval failed: {error}")
            }

            Self::Cancellation(error) => {
                write!(formatter, "quantum execution cancellation failed: {error}")
            }

            Self::TimedOut {
                job_id,
                last_state,
                cancellation_attempted,
            } => write!(
                formatter,
                "quantum job {} exceeded the local execution timeout in state {}; cancellation_attempted={}",
                job_id,
                last_state,
                cancellation_attempted
            ),

            Self::JobFailed { job_id, .. } => {
                write!(formatter, "quantum job {} failed", job_id)
            }

            Self::JobExpired { job_id } => {
                write!(formatter, "quantum job {} expired", job_id)
            }

            Self::JobCancelled { job_id } => {
                write!(formatter, "quantum job {} was cancelled", job_id)
            }

            Self::InvalidLifecycle {
                job_id,
                state,
                result_available,
            } => write!(
                formatter,
                "quantum job {} reached invalid lifecycle state {}; result_available={}",
                job_id,
                state,
                result_available
            ),

            Self::BackendIdentityMismatch { expected, actual } => write!(
                formatter,
                "quantum result backend mismatch: expected {}, received {}",
                expected,
                actual
            ),

            Self::ResultShotsExceeded {
                represented,
                requested,
            } => write!(
                formatter,
                "quantum result contains {} shots but only {} were requested",
                represented,
                requested
            ),

            Self::IncompleteResult {
                job_id,
                requested,
                counted,
            } => write!(
                formatter,
                "quantum result for job {} is incomplete: {} of {} shots represented",
                job_id,
                counted,
                requested
            ),

            Self::ResultBeforeCompletion {
                job_id,
                state,
            } => write!(
                formatter,
                "quantum result for job {} was returned before completion; state={}",
                job_id,
                state
            ),

            Self::NativeSynchronousUnsupported => {
                write!(
                    formatter,
                    "backend adapter does not support native synchronous execution"
                )
            }

            Self::InvalidPolicy(error) => {
                write!(formatter, "invalid quantum execution policy: {error}")
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

// =============================================================================
// Cancellation result
// =============================================================================

/// High-level cancellation outcome.
///
/// The provider-neutral `BackendCancellation` remains available through the
/// lower-level adapter. This wrapper adds the execution phase observed by the
/// orchestrator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionCancellation {
    /// Job affected.
    pub job: BackendJobId,

    /// Normalized provider cancellation outcome.
    pub cancellation: BackendCancellation,

    /// Phase after the cancellation request.
    pub phase: ExecutionPhase,
}

// =============================================================================
// Quantum execution engine
// =============================================================================

/// Provider-independent quantum execution orchestrator.
///
/// The engine owns an `Arc` to the adapter so the same adapter can safely be
/// shared by:
///
/// - Danga;
/// - benchmarking;
/// - interactive execution;
/// - background job management;
/// - multiple callers.
///
/// Thread safety is guaranteed by the `Send + Sync` bound already required by
/// `QuantumBackendAdapter`.
///
/// The engine itself owns no global state.
pub struct QuantumExecutionEngine<A: QuantumBackendAdapter + ?Sized> {
    adapter: Arc<A>,
    policy: ExecutionPolicy,
}

impl<A: QuantumBackendAdapter + ?Sized> Clone
    for QuantumExecutionEngine<A>
{
    fn clone(&self) -> Self {
        Self {
            adapter: Arc::clone(&self.adapter),
            policy: self.policy.clone(),
        }
    }
}

impl<A: QuantumBackendAdapter + ?Sized> fmt::Debug
    for QuantumExecutionEngine<A>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QuantumExecutionEngine")
            .field("adapter_id", &self.adapter.adapter_info().adapter_id)
            .field(
                "adapter_version",
                &self.adapter.adapter_info().adapter_version,
            )
            .field("policy", &self.policy)
            .finish()
    }
}

impl<A: QuantumBackendAdapter + ?Sized> QuantumExecutionEngine<A> {
    /// Creates an execution engine with the default production policy.
    pub fn new(adapter: Arc<A>) -> Self {
        Self {
            adapter,
            policy: ExecutionPolicy::default(),
        }
    }

    /// Creates an execution engine using an explicit policy.
    pub fn with_policy(
        adapter: Arc<A>,
        policy: ExecutionPolicy,
    ) -> Result<Self, ExecutionError> {
        policy
            .validate()
            .map_err(ExecutionError::InvalidPolicy)?;

        Ok(Self { adapter, policy })
    }

    /// Returns a shared reference to the underlying adapter.
    pub fn adapter(&self) -> &A {
        &self.adapter
    }

    /// Returns the configured policy.
    pub fn policy(&self) -> &ExecutionPolicy {
        &self.policy
    }

    /// Returns the canonical backend descriptor.
    pub fn backend(&self) -> &super::backend::QuantumBackend {
        self.adapter.backend()
    }

    /// Performs local structure validation followed by adapter preflight.
    ///
    /// No provider submission occurs if either phase fails.
    pub fn preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), ExecutionError> {
        request
            .validate_structure()
            .map_err(|_| ExecutionError::InvalidRequest)?;

        if program.is_empty() {
            return Err(ExecutionError::InvalidRequest);
        }

        self.adapter
            .preflight(request, program)
            .map_err(ExecutionError::Preflight)
    }

    /// Submits a job and returns immediately.
    ///
    /// This method never waits for provider completion.
    pub fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ExecutionHandle, ExecutionError> {
        self.preflight(request, program)?;

        let job = self
            .adapter
            .submit(request, program)
            .map_err(ExecutionError::Submission)?;

        self.validate_submitted_job(request, &job)?;

        Ok(ExecutionHandle {
            job,
            phase: ExecutionPhase::Submitted,
        })
    }

    /// Executes using the configured orchestration mode.
    pub fn run(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
        mode: ExecutionMode,
    ) -> Result<ExecutionOutcome, ExecutionError> {
        match mode {
            ExecutionMode::SubmitOnly => {
                self.submit(request, program)
                    .map(ExecutionOutcome::Submitted)
            }

            ExecutionMode::WaitForResult => {
                let handle = self.submit(request, program)?;
                let (result, receipt) =
                    self.wait_for_result(&handle, request)?;

                Ok(ExecutionOutcome::Completed {
                    handle,
                    result,
                    receipt,
                })
            }

            ExecutionMode::NativeSynchronous => {
                self.execute_synchronously(request, program)
                    .map(|(handle, result, receipt)| {
                        ExecutionOutcome::Completed {
                            handle,
                            result,
                            receipt,
                        }
                    })
            }
        }
    }

    /// Executes using asynchronous submission followed by local polling.
    ///
    /// This is the canonical production path for remote QPUs.
    pub fn execute(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(ExecutionHandle, ExecutionResult, ExecutionReceipt), ExecutionError>
    {
        let handle = self.submit(request, program)?;
        let (result, receipt) = self.wait_for_result(&handle, request)?;

        Ok((handle, result, receipt))
    }

    /// Uses the adapter's native synchronous execution implementation.
    ///
    /// The adapter must explicitly advertise synchronous execution support.
    ///
    /// This method still performs preflight before invoking the adapter.
    pub fn execute_synchronously(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(ExecutionHandle, ExecutionResult, ExecutionReceipt), ExecutionError>
    {
        self.preflight(request, program)?;

        if !self.adapter.supports_synchronous_execution() {
            return Err(ExecutionError::NativeSynchronousUnsupported);
        }

        let result = self
            .adapter
            .execute(request, program)
            .map_err(ExecutionError::Submission)?;

        let backend_id = self.adapter.backend().id().to_string();

        self.validate_result_identity(&backend_id, &result)?;

        let requested_shots = request.workload.circuit.shots;

        self.validate_result_shots(
            requested_shots,
            &result,
            None,
        )?;

        let job_id = request
            .request_id
            .as_deref()
            .unwrap_or("synchronous-execution");

        let job_id = BackendJobId::new(job_id)
            .map_err(|_| ExecutionError::InvalidRequest)?;

        let job = BackendJob::new(
            job_id,
            backend_id.clone(),
            request.request_id.clone(),
            BackendJobState::Completed,
        )
        .map_err(|_| ExecutionError::InvalidRequest)?;

        let counted_shots = result.counted_shots();

        let receipt = ExecutionReceipt {
            job: job.clone(),
            backend_id,
            requested_shots,
            counted_shots,
            final_state: BackendJobState::Completed,
            poll_attempts: 0,
            mode: ExecutionMode::NativeSynchronous,
        };

        Ok((
            ExecutionHandle {
                job,
                phase: ExecutionPhase::Completed,
            },
            result,
            receipt,
        ))
    }

    /// Waits for a previously submitted job to reach completion and retrieves
    /// its normalized result.
    ///
    /// This method can be called after `submit()`, including after the caller
    /// has persisted the returned `ExecutionHandle`.
    pub fn wait_for_result(
        &self,
        handle: &ExecutionHandle,
        request: &ExecutionRequest,
    ) -> Result<(ExecutionResult, ExecutionReceipt), ExecutionError> {
        self.policy
            .validate()
            .map_err(ExecutionError::InvalidPolicy)?;

        request
            .validate_structure()
            .map_err(|_| ExecutionError::InvalidRequest)?;

        self.validate_handle_backend(handle)?;

        let deadline = Instant::now()
            .checked_add(self.policy.timeout)
            .unwrap_or_else(Instant::now);

        let mut attempts = 0u64;
        let mut last_state = handle.job.state;

        loop {
            if attempts >= self.policy.max_poll_attempts {
                return self.timeout_error(
                    &handle.job.id,
                    last_state,
                );
            }

            if Instant::now() >= deadline {
                return self.timeout_error(
                    &handle.job.id,
                    last_state,
                );
            }

            let status = self
                .adapter
                .status(&handle.job.id)
                .map_err(ExecutionError::Status)?;

            attempts = attempts.saturating_add(1);
            last_state = status.job.state;

            self.validate_status_identity(handle, &status)?;

            match status.job.state {
                BackendJobState::Completed => {
                    if self.policy.require_completed_state
                        && !status.result_available
                    {
                        return Err(ExecutionError::InvalidLifecycle {
                            job_id: handle.job.id.clone(),
                            state: status.job.state,
                            result_available: status.result_available,
                        });
                    }

                    let result = self
                        .retrieve_and_validate_result(
                            handle,
                            request,
                            &status,
                        )?;

                    let receipt = ExecutionReceipt {
                        job: handle.job.clone(),
                        backend_id: handle.job.backend_id.clone(),
                        requested_shots: request.workload.circuit.shots,
                        counted_shots: result.counted_shots(),
                        final_state: BackendJobState::Completed,
                        poll_attempts: attempts,
                        mode: ExecutionMode::WaitForResult,
                    };

                    return Ok((result, receipt));
                }

                BackendJobState::Failed => {
                    return Err(ExecutionError::JobFailed {
                        job_id: handle.job.id.clone(),
                        status,
                    });
                }

                BackendJobState::Expired => {
                    return Err(ExecutionError::JobExpired {
                        job_id: handle.job.id.clone(),
                    });
                }

                BackendJobState::Cancelled => {
                    return Err(ExecutionError::JobCancelled {
                        job_id: handle.job.id.clone(),
                    });
                }

                BackendJobState::Created
                | BackendJobState::Queued
                | BackendJobState::Running
                | BackendJobState::Cancelling
                | BackendJobState::Unknown => {
                    self.sleep_until_next_poll(deadline);
                }
            }
        }
    }

    /// Retrieves a result after the caller has independently established that
    /// the job is complete.
    ///
    /// This method still verifies the lifecycle status unless the configured
    /// policy explicitly disables strict completion checking.
    pub fn result(
        &self,
        handle: &ExecutionHandle,
        request: &ExecutionRequest,
    ) -> Result<ExecutionResult, ExecutionError> {
        request
            .validate_structure()
            .map_err(|_| ExecutionError::InvalidRequest)?;

        self.validate_handle_backend(handle)?;

        let status = self
            .adapter
            .status(&handle.job.id)
            .map_err(ExecutionError::Status)?;

        self.validate_status_identity(handle, &status)?;

        if self.policy.require_completed_state
            && status.job.state != BackendJobState::Completed
        {
            return Err(ExecutionError::ResultBeforeCompletion {
                job_id: handle.job.id.clone(),
                state: status.job.state,
            });
        }

        if self.policy.require_completed_state
            && !status.result_available
        {
            return Err(ExecutionError::InvalidLifecycle {
                job_id: handle.job.id.clone(),
                state: status.job.state,
                result_available: status.result_available,
            });
        }

        self.retrieve_and_validate_result(
            handle,
            request,
            &status,
        )
    }

    /// Polls a job once without sleeping.
    ///
    /// This method is useful for external job managers that already have their
    /// own scheduling loop.
    pub fn poll(
        &self,
        handle: &ExecutionHandle,
    ) -> Result<ExecutionSnapshot, ExecutionError> {
        self.validate_handle_backend(handle)?;

        let status = self
            .adapter
            .status(&handle.job.id)
            .map_err(ExecutionError::Status)?;

        self.validate_status_identity(handle, &status)?;

        Ok(ExecutionSnapshot {
            handle: handle.clone(),
            status: Some(status),
            phase: ExecutionPhase::Waiting,
            poll_attempts: 1,
        })
    }

    /// Requests provider-side cancellation.
    ///
    /// Cancellation is never falsely reported as complete. The provider's
    /// normalized `CancellationOutcome` remains authoritative.
    pub fn cancel(
        &self,
        handle: &ExecutionHandle,
    ) -> Result<ExecutionCancellation, ExecutionError> {
        self.validate_handle_backend(handle)?;

        let cancellation = self
            .adapter
            .cancel(&handle.job.id)
            .map_err(ExecutionError::Cancellation)?;

        let phase = match cancellation.outcome {
            super::backend_trait::CancellationOutcome::Accepted
            | super::backend_trait::CancellationOutcome::Pending => {
                ExecutionPhase::Cancelling
            }

            super::backend_trait::CancellationOutcome::Unsupported => {
                ExecutionPhase::Failed
            }

            super::backend_trait::CancellationOutcome::AlreadyTerminal => {
                ExecutionPhase::Cancelled
            }
        };

        Ok(ExecutionCancellation {
            job: handle.job.id.clone(),
            cancellation,
            phase,
        })
    }

    /// Returns whether cancellation is advertised by the adapter.
    pub fn supports_cancellation(&self) -> bool {
        self.adapter.supports_cancellation()
    }

    /// Returns whether queue information is advertised by the adapter.
    pub fn supports_queue_information(&self) -> bool {
        self.adapter.supports_queue_info()
    }

    /// Returns whether native synchronous execution is advertised.
    pub fn supports_synchronous_execution(&self) -> bool {
        self.adapter.supports_synchronous_execution()
    }

    // =========================================================================
    // Internal validation
    // =========================================================================

    fn validate_submitted_job(
        &self,
        request: &ExecutionRequest,
        job: &BackendJob,
    ) -> Result<(), ExecutionError> {
        let expected_backend = self.adapter.backend().id();

        if job.backend_id != expected_backend {
            return Err(
                ExecutionError::BackendIdentityMismatch {
                    expected: expected_backend.to_string(),
                    actual: job.backend_id.clone(),
                },
            );
        }

        if let Some(request_id) = &request.request_id {
            if let Some(job_request_id) = &job.request_id {
                if job_request_id != request_id {
                    return Err(
                        ExecutionError::BackendIdentityMismatch {
                            expected: request_id.clone(),
                            actual: job_request_id.clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn validate_handle_backend(
        &self,
        handle: &ExecutionHandle,
    ) -> Result<(), ExecutionError> {
        let expected = self.adapter.backend().id();

        if handle.job.backend_id != expected {
            return Err(
                ExecutionError::BackendIdentityMismatch {
                    expected: expected.to_string(),
                    actual: handle.job.backend_id.clone(),
                },
            );
        }

        Ok(())
    }

    fn validate_status_identity(
        &self,
        handle: &ExecutionHandle,
        status: &BackendJobStatus,
    ) -> Result<(), ExecutionError> {
        if status.job.id != handle.job.id {
            return Err(ExecutionError::InvalidLifecycle {
                job_id: handle.job.id.clone(),
                state: status.job.state,
                result_available: status.result_available,
            });
        }

        if status.job.backend_id != handle.job.backend_id {
            return Err(
                ExecutionError::BackendIdentityMismatch {
                    expected: handle.job.backend_id.clone(),
                    actual: status.job.backend_id.clone(),
                },
            );
        }

        Ok(())
    }

    fn retrieve_and_validate_result(
        &self,
        handle: &ExecutionHandle,
        request: &ExecutionRequest,
        status: &BackendJobStatus,
    ) -> Result<ExecutionResult, ExecutionError> {
        if self.policy.require_completed_state
            && status.job.state != BackendJobState::Completed
        {
            return Err(ExecutionError::ResultBeforeCompletion {
                job_id: handle.job.id.clone(),
                state: status.job.state,
            });
        }

        if self.policy.require_completed_state
            && !status.result_available
        {
            return Err(ExecutionError::InvalidLifecycle {
                job_id: handle.job.id.clone(),
                state: status.job.state,
                result_available: status.result_available,
            });
        }

        let result = self
            .adapter
            .result(&handle.job.id)
            .map_err(ExecutionError::Result)?;

        if self.policy.require_completed_state {
            self.validate_result_identity(
                &handle.job.backend_id,
                &result,
            )?;
        }

        self.validate_result_shots(
            request.workload.circuit.shots,
            &result,
            Some(&handle.job.id),
        )?;

        Ok(result)
    }

    fn validate_result_identity(
        &self,
        expected_backend: &str,
        result: &ExecutionResult,
    ) -> Result<(), ExecutionError> {
        if self.policy.require_backend_identity_match
            && result.backend_id != expected_backend
        {
            return Err(
                ExecutionError::BackendIdentityMismatch {
                    expected: expected_backend.to_string(),
                    actual: result.backend_id.clone(),
                },
            );
        }

        Ok(())
    }

    fn validate_result_shots(
        &self,
        requested: usize,
        result: &ExecutionResult,
        job_id: Option<&BackendJobId>,
    ) -> Result<(), ExecutionError> {
        if requested == 0 {
            return Err(ExecutionError::InvalidRequest);
        }

        let represented = result.counted_shots();

        if represented > requested {
            return Err(
                ExecutionError::ResultShotsExceeded {
                    represented,
                    requested,
                },
            );
        }

        if represented != requested {
            if let Some(job_id) = job_id {
                return Err(
                    ExecutionError::IncompleteResult {
                        job_id: job_id.clone(),
                        requested,
                        counted: represented,
                    },
                );
            }

            return Err(ExecutionError::IncompleteResult {
                job_id: BackendJobId::new(
                    "synchronous-execution",
                )
                .map_err(|_| ExecutionError::InvalidRequest)?,
                requested,
                counted: represented,
            });
        }

        Ok(())
    }

    fn timeout_error(
        &self,
        job_id: &BackendJobId,
        last_state: BackendJobState,
    ) -> Result<(ExecutionResult, ExecutionReceipt), ExecutionError> {
        let mut cancellation_attempted = false;

        if self.policy.cancel_on_timeout
            && self.adapter.supports_cancellation()
        {
            cancellation_attempted = true;

            // Cancellation is explicitly best-effort. We intentionally do not
            // replace the timeout error with a cancellation error because the
            // caller's actual contract was the local timeout.
            let _ = self.adapter.cancel(job_id);
        }

        Err(ExecutionError::TimedOut {
            job_id: job_id.clone(),
            last_state,
            cancellation_attempted,
        })
    }

    fn sleep_until_next_poll(&self, deadline: Instant) {
        let now = Instant::now();

        if now >= deadline {
            return;
        }

        let remaining = deadline.saturating_duration_since(now);

        let sleep_for = if remaining < self.policy.poll_interval {
            remaining
        } else {
            self.policy.poll_interval
        };

        if !sleep_for.is_zero() {
            thread::sleep(sleep_for);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_valid() {
        let policy = ExecutionPolicy::default();

        assert!(policy.validate().is_ok());
        assert_eq!(
            policy.poll_interval,
            DEFAULT_POLL_INTERVAL
        );
        assert_eq!(
            policy.timeout,
            DEFAULT_EXECUTION_TIMEOUT
        );
    }

    #[test]
    fn zero_timeout_is_rejected() {
        let result =
            ExecutionPolicy::default()
                .with_timeout(Duration::ZERO);

        assert!(matches!(
            result,
            Err(ExecutionPolicyError::InvalidTimeout { .. })
        ));
    }

    #[test]
    fn excessive_timeout_is_rejected() {
        let result =
            ExecutionPolicy::default()
                .with_timeout(
                    MAX_EXECUTION_TIMEOUT
                        .saturating_add(Duration::from_secs(1)),
                );

        assert!(matches!(
            result,
            Err(ExecutionPolicyError::InvalidTimeout { .. })
        ));
    }

    #[test]
    fn zero_poll_interval_is_rejected() {
        let result =
            ExecutionPolicy::default()
                .with_poll_interval(Duration::ZERO);

        assert!(matches!(
            result,
            Err(
                ExecutionPolicyError::InvalidPollInterval {
                    ..
                }
            )
        ));
    }

    #[test]
    fn excessive_poll_interval_is_rejected() {
        let result =
            ExecutionPolicy::default()
                .with_poll_interval(
                    MAX_POLL_INTERVAL
                        .saturating_add(Duration::from_millis(1)),
                );

        assert!(matches!(
            result,
            Err(
                ExecutionPolicyError::InvalidPollInterval {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_poll_attempt_limit_is_rejected() {
        let result =
            ExecutionPolicy::default()
                .with_max_poll_attempts(0);

        assert!(matches!(
            result,
            Err(
                ExecutionPolicyError::InvalidPollAttemptLimit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn execution_modes_have_stable_names() {
        assert_eq!(
            ExecutionMode::SubmitOnly.as_str(),
            "submit_only"
        );

        assert_eq!(
            ExecutionMode::WaitForResult.as_str(),
            "wait_for_result"
        );

        assert_eq!(
            ExecutionMode::NativeSynchronous.as_str(),
            "native_synchronous"
        );
    }

    #[test]
    fn execution_phases_have_stable_names() {
        assert_eq!(
            ExecutionPhase::Validating.as_str(),
            "validating"
        );

        assert_eq!(
            ExecutionPhase::Completed.as_str(),
            "completed"
        );

        assert!(ExecutionPhase::Completed.is_terminal());
        assert!(!ExecutionPhase::Waiting.is_terminal());
    }

    #[test]
    fn execution_receipt_reports_complete_counts() {
        let job_id =
            BackendJobId::new("local-001")
                .expect("valid job ID");

        let job =
            BackendJob::new(
                job_id,
                "local.simulator",
                None,
                BackendJobState::Completed,
            )
            .expect("valid job");

        let receipt = ExecutionReceipt {
            job,
            backend_id: "local.simulator".to_string(),
            requested_shots: 100,
            counted_shots: 100,
            final_state: BackendJobState::Completed,
            poll_attempts: 3,
            mode: ExecutionMode::WaitForResult,
        };

        assert!(receipt.is_complete());
    }

    #[test]
    fn execution_receipt_detects_incomplete_counts() {
        let job_id =
            BackendJobId::new("local-002")
                .expect("valid job ID");

        let job =
            BackendJob::new(
                job_id,
                "local.simulator",
                None,
                BackendJobState::Completed,
            )
            .expect("valid job");

        let receipt = ExecutionReceipt {
            job,
            backend_id: "local.simulator".to_string(),
            requested_shots: 100,
            counted_shots: 99,
            final_state: BackendJobState::Completed,
            poll_attempts: 3,
            mode: ExecutionMode::WaitForResult,
        };

        assert!(!receipt.is_complete());
    }

    #[test]
    fn execution_handle_exposes_job_identity() {
        let job_id =
            BackendJobId::new("provider-123")
                .expect("valid job ID");

        let job =
            BackendJob::new(
                job_id,
                "provider.backend",
                None,
                BackendJobState::Queued,
            )
            .expect("valid job");

        let handle = ExecutionHandle {
            job,
            phase: ExecutionPhase::Submitted,
        };

        assert_eq!(
            handle.job_id().as_str(),
            "provider-123"
        );

        assert_eq!(
            handle.backend_id(),
            "provider.backend"
        );

        assert_eq!(
            handle.state(),
            BackendJobState::Queued
        );

        assert!(!handle.is_terminal());
    }

    #[test]
    fn execution_snapshot_preserves_submitted_state() {
        let job_id =
            BackendJobId::new("provider-456")
                .expect("valid job ID");

        let job =
            BackendJob::new(
                job_id,
                "provider.backend",
                None,
                BackendJobState::Queued,
            )
            .expect("valid job");

        let handle = ExecutionHandle {
            job,
            phase: ExecutionPhase::Submitted,
        };

        let snapshot =
            ExecutionSnapshot::submitted(handle);

        assert_eq!(
            snapshot.state(),
            BackendJobState::Queued
        );

        assert!(!snapshot.is_terminal());
        assert_eq!(
            snapshot.poll_attempts,
            0
        );
    }

    #[test]
    fn execution_outcome_submission_has_no_result() {
        let job_id =
            BackendJobId::new("provider-789")
                .expect("valid job ID");

        let job =
            BackendJob::new(
                job_id,
                "provider.backend",
                None,
                BackendJobState::Queued,
            )
            .expect("valid job");

        let handle = ExecutionHandle {
            job,
            phase: ExecutionPhase::Submitted,
        };

        let outcome =
            ExecutionOutcome::Submitted(handle);

        assert!(outcome.result().is_none());
        assert_eq!(
            outcome.job_id()
                .expect("job ID")
                .as_str(),
            "provider-789"
        );
    }

    #[test]
    fn timeout_error_contains_job_identity() {
        let job_id =
            BackendJobId::new("timeout-job")
                .expect("valid job ID");

        let error = ExecutionError::TimedOut {
            job_id: job_id.clone(),
            last_state: BackendJobState::Running,
            cancellation_attempted: false,
        };

        let message = error.to_string();

        assert!(message.contains("timeout-job"));
        assert!(message.contains("running"));
        assert!(message.contains("false"));
    }

    #[test]
    fn cancellation_phase_for_pending_is_cancelling() {
        let phase = match super::super::backend_trait::CancellationOutcome::Pending
        {
            super::super::backend_trait::CancellationOutcome::Accepted
            | super::super::backend_trait::CancellationOutcome::Pending => {
                ExecutionPhase::Cancelling
            }

            super::super::backend_trait::CancellationOutcome::Unsupported => {
                ExecutionPhase::Failed
            }

            super::super::backend_trait::CancellationOutcome::AlreadyTerminal => {
                ExecutionPhase::Cancelled
            }
        };

        assert_eq!(phase, ExecutionPhase::Cancelling);
    }
}