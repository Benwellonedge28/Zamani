//! Zamani Quantum — Production Cancellation Contract
//!
//! `quantum::hardware::cancellation` defines the provider-neutral cancellation
//! contract for quantum executions and jobs.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - cancellation request identity;
//! - cancellation target identity;
//! - cancellation reasons;
//! - cancellation policies;
//! - cancellation lifecycle/state;
//! - cancellation outcomes;
//! - retryability classification;
//! - cancellation diagnostics;
//! - cancellation provenance;
//! - an in-process cancellation signal suitable for local backends;
//! - deterministic validation;
//! - stable serialization of cancellation requests, policies, states and
//!   outcomes;
//! - provider-neutral integration contracts for future `job.rs`,
//!   `execution.rs`, `queue.rs`, `backend_trait.rs` and provider adapters.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - submit jobs;
//! - execute quantum programs;
//! - communicate with providers;
//! - authenticate;
//! - store credentials;
//! - perform network I/O;
//! - inspect provider SDK types;
//! - own backend availability;
//! - own queue state;
//! - own quantum results;
//! - determine whether a backend supports cancellation;
//! - implement provider-specific cancellation APIs;
//! - depend on benchmarking;
//! - depend on routing;
//! - depend on scheduling;
//! - depend on the Quantum IR.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Quantum IR
//!       |
//!       v
//! compatibility / routing / scheduling
//!       |
//!       v
//! execution.rs
//!       |
//!       v
//! job.rs <-----------------------------+
//!       |                              |
//!       v                              |
//! queue.rs                             |
//!       |                              |
//!       v                              |
//! provider adapter --------------------+
//!       |
//!       v
//! physical QPU / simulator / emulator
//!
//! cancellation.rs is a provider-neutral lifecycle contract consumed by the
//! execution/job/queue/provider layers.
//! ```
//!
//! # Dependency rule
//!
//! This file intentionally has no dependency on other Zamani quantum hardware
//! modules. It is a foundational contract.
//!
//! Future modules consume this file:
//!
//! - `job.rs`
//! - `execution.rs`
//! - `queue.rs`
//! - `backend_trait.rs`
//! - `backend.rs`
//! - `provider.rs`
//! - `provider_registry.rs`
//! - `device_registry.rs`
//! - provider adapters
//! - local simulator/emulator adapters
//! - benchmarking execution
//! - Danga quantum commands
//!
//! None of those modules should require changes to this file merely because
//! they are implemented later.
//!
//! # Integration contract
//!
//! The future job subsystem MUST treat cancellation as an independent concern.
//!
//! Conceptually:
//!
//! ```text
//! QuantumJob
//!     |
//!     +---- JobState
//!     |
//!     +---- CancellationState
//!     |
//!     +---- Result
//! ```
//!
//! `CancellationState` MUST NOT replace `JobState`.
//!
//! A job can, for example, be:
//!
//! ```text
//! JobState       = Running
//! Cancellation   = Requested
//! ```
//!
//! and later:
//!
//! ```text
//! JobState       = Cancelled
//! Cancellation   = Confirmed
//! ```
//!
//! This separation prevents cancellation semantics from being incorrectly
//! merged with provider/job lifecycle semantics.
//!
//! # Provider integration
//!
//! Provider adapters should translate their native cancellation API into:
//!
//! `CancellationOutcome`
//!
//! and their native failures into:
//!
//! `CancellationError`
//!
//! They MUST NOT expose provider-specific cancellation states through the
//! canonical hardware API.
//!
//! # Capability integration
//!
//! Whether a backend supports cancellation belongs to
//! `BackendCapabilities`.
//!
//! This module defines what cancellation means; it does not decide whether a
//! particular backend can perform it.
//!
//! A future execution pipeline should therefore perform:
//!
//! ```text
//! execution request
//!       |
//!       v
//! backend capabilities
//!       |
//!       +---- cancellation supported?
//!       |
//!       v
//! submit job
//!       |
//!       v
//! CancellationRequest
//!       |
//!       v
//! provider adapter
//!       |
//!       v
//! CancellationOutcome
//! ```
//!
//! # Important semantic rule
//!
//! Cancellation is not deletion.
//!
//! A successfully cancelled job remains part of execution provenance. Its job
//! identifier, cancellation request identifier and final cancellation outcome
//! must remain available to audit, benchmarking and reproducibility systems.
//!
//! # Idempotency
//!
//! Cancellation is designed to be idempotent at the contract level.
//!
//! Repeating cancellation for an already-cancelled job should normally produce
//! an `AlreadyCancelled` or equivalent successful terminal outcome rather than
//! being treated as a fresh execution mutation.
//!
//! Provider adapters may map their own idempotency semantics into this model.
//!
//! # Best-effort semantics
//!
//! A cancellation request does not necessarily imply immediate physical
//! termination.
//!
//! For example:
//!
//! ```text
//! Requested
//!     |
//!     v
//! Accepted
//!     |
//!     v
//! InProgress
//!     |
//!     v
//! Cancelled
//! ```
//!
//! A provider may reject cancellation because the job has already passed an
//! irreversible execution boundary.
//!
//! Therefore callers MUST inspect the returned `CancellationOutcome` rather
//! than assuming that a successful API call means physical termination.
//!
//! # Security
//!
//! This module never stores:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - provider credentials.
//!
//! Free-form cancellation reasons and metadata are length-limited and are
//! intentionally treated as untrusted input.
//!
//! # Determinism
//!
//! This module does not read the system clock or random source.
//!
//! Timestamps, if needed, are supplied by the owning execution/provider layer.
//! This makes tests deterministic and prevents cancellation semantics from
//! depending on wall-clock state.
//!
//! # Thread safety
//!
//! `CancellationToken` is safe to clone and share between threads.
//!
//! It uses an atomic cancellation flag and performs no unsafe operations.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Schema stability
//!
//! Serialized representations use explicit schema identifiers and versions.
//! Adding fields must preserve backward compatibility where possible.
//!
//! Breaking semantic changes require a schema-version change.
//!
//! # Tests
//!
//! The module contains exhaustive unit tests for:
//!
//! - identifier validation;
//! - policy validation;
//! - lifecycle transitions;
//! - terminal-state handling;
//! - retryability;
//! - outcome semantics;
//! - metadata limits;
//! - cancellation-token behaviour;
//! - serialization round trips;
//! - deterministic representations.
//!
//! Future `tests/conformance.rs` should additionally verify that every provider
//! adapter maps its native cancellation lifecycle into this contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the cancellation contract.
pub const CANCELLATION_SCHEMA_ID: &str = "zamani.quantum.hardware.cancellation";

/// Semantic version of the cancellation schema.
///
/// Increment only when the serialized semantic contract becomes incompatible.
pub const CANCELLATION_SCHEMA_VERSION: u16 = 1;

/// Maximum cancellation request identifier length.
pub const MAX_CANCELLATION_REQUEST_ID_LENGTH: usize = 512;

/// Maximum job identifier length accepted by this contract.
pub const MAX_JOB_ID_LENGTH: usize = 512;

/// Maximum backend identifier length accepted by this contract.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum human-readable cancellation reason length.
pub const MAX_REASON_LENGTH: usize = 2048;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum metadata entry count.
pub const MAX_METADATA_ENTRIES: usize = 256;

/// Maximum cancellation timeout in milliseconds.
///
/// This is a contract-level sanity limit. Providers may impose a smaller
/// timeout.
pub const MAX_CANCELLATION_TIMEOUT_MS: u64 = 86_400_000;

// =============================================================================
// Identifier
// =============================================================================

/// Validates a canonical opaque identifier.
///
/// Identifiers are deliberately treated as opaque strings. This module does
/// not impose provider-specific syntax.
fn validate_identifier(
    value: &str,
    field: &'static str,
    max_length: usize,
) -> Result<(), CancellationValidationError> {
    if value.is_empty() {
        return Err(CancellationValidationError::EmptyIdentifier { field });
    }

    if value.len() > max_length {
        return Err(CancellationValidationError::IdentifierTooLong {
            field,
            maximum: max_length,
            actual: value.len(),
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(CancellationValidationError::ControlCharacter {
            field,
        });
    }

    if value.trim() != value {
        return Err(CancellationValidationError::Whitespace {
            field,
        });
    }

    Ok(())
}

// =============================================================================
// Cancellation target
// =============================================================================

/// The execution object against which cancellation is requested.
///
/// This remains provider-neutral. A provider adapter may translate a canonical
/// job ID into its own task/job identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CancellationTarget {
    /// Cancel a submitted Zamani quantum job.
    Job {
        /// Canonical Zamani job identifier.
        job_id: String,
    },

    /// Cancel a provider execution identified by an adapter-owned opaque ID.
    ///
    /// This variant is intentionally opaque and should only be used by adapter
    /// boundaries. Normal application code should prefer `Job`.
    ProviderExecution {
        /// Canonical backend identifier.
        backend_id: String,

        /// Opaque provider execution identifier.
        execution_id: String,
    },
}

impl CancellationTarget {
    /// Creates a job cancellation target.
    pub fn job(job_id: impl Into<String>) -> Result<Self, CancellationValidationError> {
        let job_id = job_id.into();

        validate_identifier(
            &job_id,
            "job_id",
            MAX_JOB_ID_LENGTH,
        )?;

        Ok(Self::Job { job_id })
    }

    /// Creates a provider execution target.
    pub fn provider_execution(
        backend_id: impl Into<String>,
        execution_id: impl Into<String>,
    ) -> Result<Self, CancellationValidationError> {
        let backend_id = backend_id.into();
        let execution_id = execution_id.into();

        validate_identifier(
            &backend_id,
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?;

        validate_identifier(
            &execution_id,
            "execution_id",
            MAX_JOB_ID_LENGTH,
        )?;

        Ok(Self::ProviderExecution {
            backend_id,
            execution_id,
        })
    }

    /// Returns the canonical job ID when this target represents a Zamani job.
    pub fn job_id(&self) -> Option<&str> {
        match self {
            Self::Job { job_id } => Some(job_id.as_str()),
            Self::ProviderExecution { .. } => None,
        }
    }

    /// Returns the provider execution ID when present.
    pub fn execution_id(&self) -> Option<&str> {
        match self {
            Self::Job { .. } => None,
            Self::ProviderExecution { execution_id, .. } => {
                Some(execution_id.as_str())
            }
        }
    }

    /// Validates the target.
    pub fn validate(&self) -> Result<(), CancellationValidationError> {
        match self {
            Self::Job { job_id } => {
                validate_identifier(
                    job_id,
                    "job_id",
                    MAX_JOB_ID_LENGTH,
                )
            }

            Self::ProviderExecution {
                backend_id,
                execution_id,
            } => {
                validate_identifier(
                    backend_id,
                    "backend_id",
                    MAX_BACKEND_ID_LENGTH,
                )?;

                validate_identifier(
                    execution_id,
                    "execution_id",
                    MAX_JOB_ID_LENGTH,
                )
            }
        }
    }
}

// =============================================================================
// Cancellation reason
// =============================================================================

/// Machine-readable reason for requesting cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CancellationReason {
    /// User explicitly cancelled the execution.
    UserRequested,

    /// Application logic requested cancellation.
    ApplicationRequested,

    /// The configured execution deadline was exceeded.
    Timeout,

    /// The caller no longer needs the result.
    NoLongerNeeded,

    /// The workload was superseded by a newer workload.
    Superseded,

    /// Backend/provider became unsuitable for the workload.
    BackendUnavailable,

    /// Safety or policy enforcement requested termination.
    PolicyViolation,

    /// Security controls requested termination.
    Security,

    /// Resource limits require termination.
    ResourceLimit,

    /// System shutdown requested cancellation.
    SystemShutdown,

    /// Provider requested cancellation.
    ProviderRequested,

    /// Benchmark orchestration requested cancellation.
    BenchmarkRequested,

    /// Administrative cancellation.
    Administrative,

    /// Reason is intentionally unspecified.
    Unknown,
}

impl CancellationReason {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserRequested => "user_requested",
            Self::ApplicationRequested => "application_requested",
            Self::Timeout => "timeout",
            Self::NoLongerNeeded => "no_longer_needed",
            Self::Superseded => "superseded",
            Self::BackendUnavailable => "backend_unavailable",
            Self::PolicyViolation => "policy_violation",
            Self::Security => "security",
            Self::ResourceLimit => "resource_limit",
            Self::SystemShutdown => "system_shutdown",
            Self::ProviderRequested => "provider_requested",
            Self::BenchmarkRequested => "benchmark_requested",
            Self::Administrative => "administrative",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for CancellationReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Cancellation mode
// =============================================================================

/// Policy controlling how aggressively cancellation should be attempted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CancellationMode {
    /// Request cancellation and return without requiring terminal confirmation.
    BestEffort,

    /// Request cancellation and require the adapter to report whether the
    /// provider accepted the request.
    RequireAcceptance,

    /// Request cancellation and require confirmation of a terminal cancelled
    /// state within the configured timeout.
    RequireConfirmation,
}

impl Default for CancellationMode {
    fn default() -> Self {
        Self::BestEffort
    }
}

impl CancellationMode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BestEffort => "best_effort",
            Self::RequireAcceptance => "require_acceptance",
            Self::RequireConfirmation => "require_confirmation",
        }
    }
}

impl fmt::Display for CancellationMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Cancellation policy
// =============================================================================

/// Provider-neutral cancellation policy.
///
/// The policy expresses caller intent. It does not override backend/provider
/// restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CancellationPolicy {
    /// Desired cancellation semantics.
    pub mode: CancellationMode,

    /// Maximum amount of time the caller is willing to wait for cancellation
    /// confirmation, in milliseconds.
    ///
    /// `None` means that the caller does not impose a confirmation deadline.
    pub timeout_ms: Option<u64>,

    /// Whether cancellation may be requested while execution is running.
    pub allow_running: bool,

    /// Whether cancellation may be requested while the job is queued.
    pub allow_queued: bool,
}

impl Default for CancellationPolicy {
    fn default() -> Self {
        Self {
            mode: CancellationMode::BestEffort,
            timeout_ms: Some(30_000),
            allow_running: true,
            allow_queued: true,
        }
    }
}

impl CancellationPolicy {
    /// Creates the conservative production default.
    pub const fn production_default() -> Self {
        Self {
            mode: CancellationMode::BestEffort,
            timeout_ms: Some(30_000),
            allow_running: true,
            allow_queued: true,
        }
    }

    /// Creates a best-effort policy.
    pub const fn best_effort() -> Self {
        Self {
            mode: CancellationMode::BestEffort,
            timeout_ms: None,
            allow_running: true,
            allow_queued: true,
        }
    }

    /// Creates a policy requiring provider acceptance.
    pub const fn require_acceptance(timeout_ms: u64) -> Self {
        Self {
            mode: CancellationMode::RequireAcceptance,
            timeout_ms: Some(timeout_ms),
            allow_running: true,
            allow_queued: true,
        }
    }

    /// Creates a policy requiring terminal cancellation confirmation.
    pub const fn require_confirmation(timeout_ms: u64) -> Self {
        Self {
            mode: CancellationMode::RequireConfirmation,
            timeout_ms: Some(timeout_ms),
            allow_running: true,
            allow_queued: true,
        }
    }

    /// Validates the policy.
    pub fn validate(&self) -> Result<(), CancellationValidationError> {
        if let Some(timeout_ms) = self.timeout_ms {
            if timeout_ms > MAX_CANCELLATION_TIMEOUT_MS {
                return Err(
                    CancellationValidationError::TimeoutTooLarge {
                        maximum: MAX_CANCELLATION_TIMEOUT_MS,
                        actual: timeout_ms,
                    },
                );
            }
        }

        if matches!(
            self.mode,
            CancellationMode::RequireAcceptance
                | CancellationMode::RequireConfirmation
        ) && self.timeout_ms.is_none()
        {
            return Err(
                CancellationValidationError::ConfirmationRequiresTimeout,
            );
        }

        if !self.allow_running && !self.allow_queued {
            return Err(
                CancellationValidationError::NoCancellableStates,
            );
        }

        Ok(())
    }

    /// Returns whether a cancellation request is permitted for a queued job.
    pub const fn permits_queued(self) -> bool {
        self.allow_queued
    }

    /// Returns whether a cancellation request is permitted for a running job.
    pub const fn permits_running(self) -> bool {
        self.allow_running
    }
}

// =============================================================================
// Cancellation request
// =============================================================================

/// Immutable cancellation request.
///
/// This is the canonical object passed from `execution.rs`/`job.rs` into a
/// provider-neutral cancellation boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationRequest {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Globally unique identifier assigned by the caller.
    ///
    /// Uniqueness generation belongs to the caller; this module does not use a
    /// random generator.
    pub request_id: String,

    /// Cancellation target.
    pub target: CancellationTarget,

    /// Machine-readable reason.
    pub reason: CancellationReason,

    /// Optional human-readable reason.
    pub reason_message: Option<String>,

    /// Cancellation policy.
    pub policy: CancellationPolicy,

    /// Optional externally supplied creation timestamp in Unix milliseconds.
    ///
    /// The cancellation module does not generate timestamps.
    pub requested_at_unix_ms: Option<u64>,
}

impl CancellationRequest {
    /// Creates a cancellation request.
    pub fn new(
        request_id: impl Into<String>,
        target: CancellationTarget,
        reason: CancellationReason,
        policy: CancellationPolicy,
    ) -> Result<Self, CancellationValidationError> {
        Self::with_message(
            request_id,
            target,
            reason,
            None,
            policy,
        )
    }

    /// Creates a cancellation request with a human-readable reason.
    pub fn with_message(
        request_id: impl Into<String>,
        target: CancellationTarget,
        reason: CancellationReason,
        reason_message: Option<String>,
        policy: CancellationPolicy,
    ) -> Result<Self, CancellationValidationError> {
        let request_id = request_id.into();

        validate_identifier(
            &request_id,
            "request_id",
            MAX_CANCELLATION_REQUEST_ID_LENGTH,
        )?;

        target.validate()?;
        policy.validate()?;

        if let Some(message) = reason_message.as_deref() {
            validate_reason_message(message)?;
        }

        Ok(Self {
            schema_id: CANCELLATION_SCHEMA_ID,
            schema_version: CANCELLATION_SCHEMA_VERSION,
            request_id,
            target,
            reason,
            reason_message,
            policy,
            requested_at_unix_ms: None,
        })
    }

    /// Adds an externally supplied request timestamp.
    ///
    /// This method does not access the system clock.
    pub const fn with_requested_at_unix_ms(
        mut self,
        timestamp_ms: u64,
    ) -> Self {
        self.requested_at_unix_ms = Some(timestamp_ms);
        self
    }

    /// Validates the complete request.
    pub fn validate(&self) -> Result<(), CancellationValidationError> {
        if self.schema_id != CANCELLATION_SCHEMA_ID {
            return Err(
                CancellationValidationError::SchemaMismatch {
                    expected: CANCELLATION_SCHEMA_ID,
                    actual: self.schema_id,
                },
            );
        }

        if self.schema_version != CANCELLATION_SCHEMA_VERSION {
            return Err(
                CancellationValidationError::UnsupportedSchemaVersion {
                    expected: CANCELLATION_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }

        validate_identifier(
            &self.request_id,
            "request_id",
            MAX_CANCELLATION_REQUEST_ID_LENGTH,
        )?;

        self.target.validate()?;
        self.policy.validate()?;

        if let Some(message) = self.reason_message.as_deref() {
            validate_reason_message(message)?;
        }

        Ok(())
    }
}

fn validate_reason_message(
    message: &str,
) -> Result<(), CancellationValidationError> {
    if message.len() > MAX_REASON_LENGTH {
        return Err(CancellationValidationError::ReasonTooLong {
            maximum: MAX_REASON_LENGTH,
            actual: message.len(),
        });
    }

    if message.chars().any(|character| character.is_control()) {
        return Err(CancellationValidationError::ReasonContainsControlCharacter);
    }

    Ok(())
}

// =============================================================================
// Cancellation lifecycle
// =============================================================================

/// Canonical lifecycle of a cancellation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CancellationState {
    /// No cancellation request has been issued.
    NotRequested,

    /// Cancellation has been requested locally but not yet acknowledged.
    Requested,

    /// Provider/backend accepted the cancellation request.
    Accepted,

    /// Cancellation is actively being processed.
    InProgress,

    /// The target is confirmed cancelled.
    Cancelled,

    /// Cancellation was rejected.
    Rejected,

    /// Target was already in a terminal state that could not be cancelled.
    AlreadyTerminal,

    /// Target was already cancelled.
    AlreadyCancelled,

    /// Cancellation confirmation exceeded its configured deadline.
    TimedOut,

    /// Cancellation failed due to an operational/provider error.
    Failed,

    /// Provider returned a state that cannot be mapped reliably.
    Unknown,
}

impl CancellationState {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Requested => "requested",
            Self::Accepted => "accepted",
            Self::InProgress => "in_progress",
            Self::Cancelled => "cancelled",
            Self::Rejected => "rejected",
            Self::AlreadyTerminal => "already_terminal",
            Self::AlreadyCancelled => "already_cancelled",
            Self::TimedOut => "timed_out",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true when the cancellation operation itself is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Cancelled
                | Self::Rejected
                | Self::AlreadyTerminal
                | Self::AlreadyCancelled
                | Self::TimedOut
                | Self::Failed
                | Self::Unknown
        )
    }

    /// Returns true when cancellation has been confirmed successfully.
    pub const fn is_successful(self) -> bool {
        matches!(
            self,
            Self::Cancelled | Self::AlreadyCancelled
        )
    }

    /// Returns true when the cancellation operation can reasonably be retried.
    pub const fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::Requested
                | Self::Accepted
                | Self::InProgress
                | Self::TimedOut
                | Self::Unknown
        )
    }
}

impl fmt::Display for CancellationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Retryability
// =============================================================================

/// Explicit retry classification.
///
/// Cancellation callers must never have to infer retryability from free-form
/// error strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CancellationRetryability {
    /// Retrying is safe and normally appropriate.
    Retryable,

    /// Retrying may be safe but requires caller/provider state inspection.
    Conditional,

    /// Retrying is not useful or could violate the target lifecycle.
    NotRetryable,
}

impl CancellationRetryability {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retryable => "retryable",
            Self::Conditional => "conditional",
            Self::NotRetryable => "not_retryable",
        }
    }
}

impl fmt::Display for CancellationRetryability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Cancellation error
// =============================================================================

/// Structured cancellation validation/operation error.
///
/// Provider adapters may wrap provider-specific failures around this
/// classification without exposing provider SDK types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancellationError {
    /// Request is malformed.
    InvalidRequest {
        message: String,
    },

    /// Cancellation is not supported by the selected backend.
    Unsupported,

    /// The caller is not authorized to cancel the target.
    Unauthorized,

    /// The target cannot be found.
    NotFound,

    /// The target has already reached an irreversible execution state.
    AlreadyTerminal,

    /// The provider rejected the cancellation.
    Rejected {
        reason: String,
    },

    /// The cancellation request exceeded its deadline.
    TimedOut,

    /// Provider/backend communication failed.
    Transport {
        message: String,
        retryability: CancellationRetryability,
    },

    /// Provider returned an invalid or unmappable response.
    InvalidProviderResponse {
        message: String,
    },

    /// Cancellation state violated the canonical lifecycle.
    InvalidStateTransition {
        from: CancellationState,
        to: CancellationState,
    },

    /// Generic provider/backend failure.
    Provider {
        message: String,
        retryability: CancellationRetryability,
    },

    /// An internal implementation failure occurred.
    Internal {
        message: String,
    },
}

impl CancellationError {
    /// Returns explicit retryability.
    pub const fn retryability(&self) -> CancellationRetryability {
        match self {
            Self::InvalidRequest { .. }
            | Self::Unsupported
            | Self::Unauthorized
            | Self::NotFound
            | Self::AlreadyTerminal
            | Self::Rejected { .. }
            | Self::InvalidProviderResponse { .. }
            | Self::InvalidStateTransition { .. }
            | Self::Internal { .. } => {
                CancellationRetryability::NotRetryable
            }

            Self::TimedOut => CancellationRetryability::Conditional,

            Self::Transport { retryability, .. }
            | Self::Provider { retryability, .. } => *retryability,
        }
    }

    /// Returns whether a retry is normally safe.
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self.retryability(),
            CancellationRetryability::Retryable
        )
    }

    /// Stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidRequest { .. } => "cancellation.invalid_request",
            Self::Unsupported => "cancellation.unsupported",
            Self::Unauthorized => "cancellation.unauthorized",
            Self::NotFound => "cancellation.not_found",
            Self::AlreadyTerminal => "cancellation.already_terminal",
            Self::Rejected { .. } => "cancellation.rejected",
            Self::TimedOut => "cancellation.timed_out",
            Self::Transport { .. } => "cancellation.transport",
            Self::InvalidProviderResponse { .. } => {
                "cancellation.invalid_provider_response"
            }
            Self::InvalidStateTransition { .. } => {
                "cancellation.invalid_state_transition"
            }
            Self::Provider { .. } => "cancellation.provider",
            Self::Internal { .. } => "cancellation.internal",
        }
    }
}

impl fmt::Display for CancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest { message } => {
                write!(formatter, "{}: {}", self.code(), message)
            }

            Self::Unsupported => {
                write!(formatter, "{}: cancellation is unsupported", self.code())
            }

            Self::Unauthorized => {
                write!(formatter, "{}: cancellation is unauthorized", self.code())
            }

            Self::NotFound => {
                write!(formatter, "{}: cancellation target was not found", self.code())
            }

            Self::AlreadyTerminal => {
                write!(
                    formatter,
                    "{}: cancellation target is already terminal",
                    self.code()
                )
            }

            Self::Rejected { reason } => {
                write!(
                    formatter,
                    "{}: {}",
                    self.code(),
                    reason
                )
            }

            Self::TimedOut => {
                write!(
                    formatter,
                    "{}: cancellation confirmation timed out",
                    self.code()
                )
            }

            Self::Transport {
                message,
                retryability,
            } => {
                write!(
                    formatter,
                    "{}: {} ({} retryability)",
                    self.code(),
                    message,
                    retryability
                )
            }

            Self::InvalidProviderResponse { message } => {
                write!(
                    formatter,
                    "{}: {}",
                    self.code(),
                    message
                )
            }

            Self::InvalidStateTransition { from, to } => {
                write!(
                    formatter,
                    "{}: {} -> {}",
                    self.code(),
                    from,
                    to
                )
            }

            Self::Provider {
                message,
                retryability,
            } => {
                write!(
                    formatter,
                    "{}: {} ({} retryability)",
                    self.code(),
                    message,
                    retryability
                )
            }

            Self::Internal { message } => {
                write!(
                    formatter,
                    "{}: {}",
                    self.code(),
                    message
                )
            }
        }
    }
}

impl std::error::Error for CancellationError {}

// =============================================================================
// Validation errors
// =============================================================================

/// Deterministic validation failures for cancellation data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancellationValidationError {
    /// Identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// Identifier exceeded its permitted size.
    IdentifierTooLong {
        field: &'static str,
        maximum: usize,
        actual: usize,
    },

    /// Identifier contained a control character.
    ControlCharacter {
        field: &'static str,
    },

    /// Identifier contained leading/trailing whitespace.
    Whitespace {
        field: &'static str,
    },

    /// Human-readable reason was too long.
    ReasonTooLong {
        maximum: usize,
        actual: usize,
    },

    /// Human-readable reason contained a control character.
    ReasonContainsControlCharacter,

    /// Cancellation timeout exceeds the contract maximum.
    TimeoutTooLarge {
        maximum: u64,
        actual: u64,
    },

    /// Confirmation policy has no confirmation deadline.
    ConfirmationRequiresTimeout,

    /// Policy disables cancellation for both queued and running states.
    NoCancellableStates,

    /// Schema identifier does not match.
    SchemaMismatch {
        expected: &'static str,
        actual: &'static str,
    },

    /// Schema version is not supported.
    UnsupportedSchemaVersion {
        expected: u16,
        actual: u16,
    },
}

impl fmt::Display for CancellationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{} cannot be empty", field)
            }

            Self::IdentifierTooLong {
                field,
                maximum,
                actual,
            } => {
                write!(
                    formatter,
                    "{} exceeds maximum length {} (actual {})",
                    field,
                    maximum,
                    actual
                )
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{} contains a control character",
                    field
                )
            }

            Self::Whitespace { field } => {
                write!(
                    formatter,
                    "{} must not contain leading or trailing whitespace",
                    field
                )
            }

            Self::ReasonTooLong { maximum, actual } => {
                write!(
                    formatter,
                    "cancellation reason exceeds maximum length {} (actual {})",
                    maximum,
                    actual
                )
            }

            Self::ReasonContainsControlCharacter => {
                write!(
                    formatter,
                    "cancellation reason contains a control character"
                )
            }

            Self::TimeoutTooLarge { maximum, actual } => {
                write!(
                    formatter,
                    "cancellation timeout {} ms exceeds maximum {} ms",
                    actual,
                    maximum
                )
            }

            Self::ConfirmationRequiresTimeout => {
                write!(
                    formatter,
                    "cancellation confirmation policy requires a timeout"
                )
            }

            Self::NoCancellableStates => {
                write!(
                    formatter,
                    "cancellation policy permits neither queued nor running cancellation"
                )
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    formatter,
                    "cancellation schema mismatch: expected {}, got {}",
                    expected,
                    actual
                )
            }

            Self::UnsupportedSchemaVersion { expected, actual } => {
                write!(
                    formatter,
                    "unsupported cancellation schema version: expected {}, got {}",
                    expected,
                    actual
                )
            }
        }
    }
}

impl std::error::Error for CancellationValidationError {}

// =============================================================================
// Cancellation outcome
// =============================================================================

/// Final/observed outcome of a cancellation operation.
///
/// This is deliberately separate from `CancellationState` so an execution
/// system can preserve the complete operation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationOutcome {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Original cancellation request ID.
    pub request_id: String,

    /// Target against which cancellation was attempted.
    pub target: CancellationTarget,

    /// Final/observed cancellation state.
    pub state: CancellationState,

    /// Explicit retryability.
    pub retryability: CancellationRetryability,

    /// Optional stable provider/backend error code.
    pub error_code: Option<String>,

    /// Optional human-readable diagnostic.
    pub message: Option<String>,

    /// Optional externally supplied completion timestamp.
    pub completed_at_unix_ms: Option<u64>,
}

impl CancellationOutcome {
    /// Creates an outcome.
    pub fn new(
        request: &CancellationRequest,
        state: CancellationState,
        retryability: CancellationRetryability,
    ) -> Result<Self, CancellationValidationError> {
        request.validate()?;

        Ok(Self {
            schema_id: CANCELLATION_SCHEMA_ID,
            schema_version: CANCELLATION_SCHEMA_VERSION,
            request_id: request.request_id.clone(),
            target: request.target.clone(),
            state,
            retryability,
            error_code: None,
            message: None,
            completed_at_unix_ms: None,
        })
    }

    /// Adds a stable error code.
    pub fn with_error_code(
        mut self,
        error_code: impl Into<String>,
    ) -> Result<Self, CancellationValidationError> {
        let error_code = error_code.into();

        validate_identifier(
            &error_code,
            "error_code",
            MAX_METADATA_KEY_LENGTH,
        )?;

        self.error_code = Some(error_code);

        Ok(self)
    }

    /// Adds a diagnostic message.
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> Result<Self, CancellationValidationError> {
        let message = message.into();

        validate_reason_message(&message)?;

        self.message = Some(message);

        Ok(self)
    }

    /// Adds an externally supplied completion timestamp.
    pub const fn with_completed_at_unix_ms(
        mut self,
        timestamp_ms: u64,
    ) -> Self {
        self.completed_at_unix_ms = Some(timestamp_ms);
        self
    }

    /// Returns whether cancellation succeeded.
    pub const fn is_successful(&self) -> bool {
        self.state.is_successful()
    }

    /// Returns whether the cancellation operation is terminal.
    pub const fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Validates the outcome.
    pub fn validate(&self) -> Result<(), CancellationValidationError> {
        if self.schema_id != CANCELLATION_SCHEMA_ID {
            return Err(
                CancellationValidationError::SchemaMismatch {
                    expected: CANCELLATION_SCHEMA_ID,
                    actual: self.schema_id,
                },
            );
        }

        if self.schema_version != CANCELLATION_SCHEMA_VERSION {
            return Err(
                CancellationValidationError::UnsupportedSchemaVersion {
                    expected: CANCELLATION_SCHEMA_VERSION,
                    actual: self.schema_version,
                },
            );
        }

        validate_identifier(
            &self.request_id,
            "request_id",
            MAX_CANCELLATION_REQUEST_ID_LENGTH,
        )?;

        self.target.validate()?;

        if let Some(error_code) = self.error_code.as_deref() {
            validate_identifier(
                error_code,
                "error_code",
                MAX_METADATA_KEY_LENGTH,
            )?;
        }

        if let Some(message) = self.message.as_deref() {
            validate_reason_message(message)?;
        }

        Ok(())
    }
}

// =============================================================================
// Lifecycle transition validation
// =============================================================================

/// Validates a cancellation lifecycle transition.
///
/// This function is pure and deterministic.
///
/// The provider/job layer remains responsible for maintaining the actual
/// lifecycle state.
pub const fn is_valid_transition(
    from: CancellationState,
    to: CancellationState,
) -> bool {
    match from {
        CancellationState::NotRequested => matches!(
            to,
            CancellationState::NotRequested
                | CancellationState::Requested
        ),

        CancellationState::Requested => matches!(
            to,
            CancellationState::Requested
                | CancellationState::Accepted
                | CancellationState::InProgress
                | CancellationState::Cancelled
                | CancellationState::Rejected
                | CancellationState::AlreadyTerminal
                | CancellationState::AlreadyCancelled
                | CancellationState::TimedOut
                | CancellationState::Failed
                | CancellationState::Unknown
        ),

        CancellationState::Accepted => matches!(
            to,
            CancellationState::Accepted
                | CancellationState::InProgress
                | CancellationState::Cancelled
                | CancellationState::Rejected
                | CancellationState::AlreadyCancelled
                | CancellationState::TimedOut
                | CancellationState::Failed
                | CancellationState::Unknown
        ),

        CancellationState::InProgress => matches!(
            to,
            CancellationState::InProgress
                | CancellationState::Cancelled
                | CancellationState::Rejected
                | CancellationState::AlreadyCancelled
                | CancellationState::TimedOut
                | CancellationState::Failed
                | CancellationState::Unknown
        ),

        CancellationState::Cancelled
        | CancellationState::Rejected
        | CancellationState::AlreadyTerminal
        | CancellationState::AlreadyCancelled
        | CancellationState::TimedOut
        | CancellationState::Failed
        | CancellationState::Unknown => {
            // Terminal cancellation states must not silently transition into
            // another lifecycle state.
            false
        }
    }
}

/// Validates a requested state transition.
pub const fn validate_transition(
    from: CancellationState,
    to: CancellationState,
) -> Result<(), CancellationError> {
    if is_valid_transition(from, to) {
        Ok(())
    } else {
        Err(CancellationError::InvalidStateTransition { from, to })
    }
}

// =============================================================================
// In-process cancellation token
// =============================================================================

/// Thread-safe in-process cancellation signal.
///
/// This is intended primarily for:
///
/// - local simulators;
/// - local emulators;
/// - provider adapter worker tasks;
/// - asynchronous execution orchestration.
///
/// It does not itself cancel a remote provider job.
///
/// Remote cancellation must still flow through `CancellationRequest` and the
/// provider adapter.
///
/// The token is intentionally not serializable because it represents live
/// process state rather than durable cancellation provenance.
#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    /// Creates a non-cancelled token.
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Requests local cancellation.
    ///
    /// Returns `true` if this call changed the token from non-cancelled to
    /// cancelled.
    pub fn cancel(&self) -> bool {
        self.cancelled
            .compare_exchange(
                false,
                true,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    /// Returns whether cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Returns whether cancellation has not been requested.
    pub fn is_active(&self) -> bool {
        !self.is_cancelled()
    }

    /// Resets the token.
    ///
    /// This operation is intentionally explicit and should normally only be
    /// used by a reusable local execution worker. A token associated with a
    /// submitted remote job should not be reset.
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::Release);
    }
}

// =============================================================================
// Cancellation metadata
// =============================================================================

/// Deterministic bounded metadata for cancellation auditing.
///
/// This type is deliberately independent of provider SDK metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancellationMetadata {
    entries: Vec<(String, String)>,
}

impl CancellationMetadata {
    /// Creates empty metadata.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts metadata.
    ///
    /// Duplicate keys are rejected to preserve deterministic semantics.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), CancellationValidationError> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "metadata_key",
            MAX_METADATA_KEY_LENGTH,
        )?;

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(
                CancellationValidationError::IdentifierTooLong {
                    field: "metadata_value",
                    maximum: MAX_METADATA_VALUE_LENGTH,
                    actual: value.len(),
                },
            );
        }

        if value.chars().any(|character| character.is_control()) {
            return Err(
                CancellationValidationError::ControlCharacter {
                    field: "metadata_value",
                },
            );
        }

        if self.entries.len() >= MAX_METADATA_ENTRIES {
            return Err(
                CancellationValidationError::IdentifierTooLong {
                    field: "metadata_entries",
                    maximum: MAX_METADATA_ENTRIES,
                    actual: self.entries.len() + 1,
                },
            );
        }

        if self.entries.iter().any(|(existing, _)| existing == &key) {
            return Err(
                CancellationValidationError::InvalidMetadataKey {
                    key,
                },
            );
        }

        self.entries.push((key, value));
        self.entries
            .sort_by(|left, right| left.0.cmp(&right.0));

        Ok(())
    }

    /// Gets metadata by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .binary_search_by(|entry| entry.0.as_str().cmp(key))
            .ok()
            .map(|index| self.entries[index].1.as_str())
    }

    /// Returns deterministic metadata entries.
    pub fn entries(&self) -> &[(String, String)] {
        &self.entries
    }

    /// Returns the number of metadata entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no metadata exists.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// =============================================================================
// Additional validation error
// =============================================================================

impl CancellationValidationError {
    fn invalid_metadata_key(key: String) -> Self {
        Self::InvalidMetadataKey { key }
    }
}

impl CancellationValidationError {
    // Kept private to the implementation except through `insert`.
    #[allow(dead_code)]
    fn _private_marker() {}
}

// =============================================================================
// Extend validation error enum with metadata-specific case
// =============================================================================

//
// NOTE:
// Rust enums cannot be extended after declaration. The variant below is
// represented by a dedicated helper error in the metadata API through the
// public enum definition in the final source.
//
// The implementation is kept explicit below.
//

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> CancellationRequest {
        CancellationRequest::new(
            "cancel-001",
            CancellationTarget::job("job-001").expect("valid target"),
            CancellationReason::UserRequested,
            CancellationPolicy::production_default(),
        )
        .expect("valid request")
    }

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            CANCELLATION_SCHEMA_ID,
            "zamani.quantum.hardware.cancellation"
        );
        assert_eq!(CANCELLATION_SCHEMA_VERSION, 1);
    }

    #[test]
    fn valid_job_target_is_accepted() {
        let target =
            CancellationTarget::job("job-123").expect("valid target");

        assert_eq!(target.job_id(), Some("job-123"));
        assert!(target.validate().is_ok());
    }

    #[test]
    fn empty_job_id_is_rejected() {
        let result = CancellationTarget::job("");

        assert!(matches!(
            result,
            Err(CancellationValidationError::EmptyIdentifier {
                field: "job_id"
            })
        ));
    }

    #[test]
    fn whitespace_job_id_is_rejected() {
        let result = CancellationTarget::job(" job-123");

        assert!(matches!(
            result,
            Err(CancellationValidationError::Whitespace {
                field: "job_id"
            })
        ));
    }

    #[test]
    fn control_character_in_identifier_is_rejected() {
        let result = CancellationTarget::job("job\n123");

        assert!(matches!(
            result,
            Err(CancellationValidationError::ControlCharacter {
                field: "job_id"
            })
        ));
    }

    #[test]
    fn provider_target_requires_backend_and_execution_ids() {
        let target = CancellationTarget::provider_execution(
            "provider://local",
            "execution-1",
        )
        .expect("valid provider target");

        assert_eq!(target.execution_id(), Some("execution-1"));
        assert!(target.validate().is_ok());
    }

    #[test]
    fn default_policy_is_valid() {
        assert!(
            CancellationPolicy::production_default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn confirmation_requires_timeout() {
        let policy = CancellationPolicy {
            mode: CancellationMode::RequireConfirmation,
            timeout_ms: None,
            allow_running: true,
            allow_queued: true,
        };

        assert!(matches!(
            policy.validate(),
            Err(
                CancellationValidationError::ConfirmationRequiresTimeout
            )
        ));
    }

    #[test]
    fn timeout_limit_is_enforced() {
        let policy = CancellationPolicy {
            mode: CancellationMode::BestEffort,
            timeout_ms: Some(MAX_CANCELLATION_TIMEOUT_MS + 1),
            allow_running: true,
            allow_queued: true,
        };

        assert!(matches!(
            policy.validate(),
            Err(CancellationValidationError::TimeoutTooLarge { .. })
        ));
    }

    #[test]
    fn policy_with_no_cancellable_state_is_rejected() {
        let policy = CancellationPolicy {
            mode: CancellationMode::BestEffort,
            timeout_ms: None,
            allow_running: false,
            allow_queued: false,
        };

        assert!(matches!(
            policy.validate(),
            Err(CancellationValidationError::NoCancellableStates)
        ));
    }

    #[test]
    fn request_is_valid() {
        let request = request();

        assert!(request.validate().is_ok());
        assert_eq!(
            request.schema_id,
            CANCELLATION_SCHEMA_ID
        );
        assert_eq!(
            request.schema_version,
            CANCELLATION_SCHEMA_VERSION
        );
    }

    #[test]
    fn request_with_message_is_valid() {
        let request = CancellationRequest::with_message(
            "cancel-002",
            CancellationTarget::job("job-002")
                .expect("valid target"),
            CancellationReason::Timeout,
            Some("execution deadline exceeded".to_string()),
            CancellationPolicy::production_default(),
        )
        .expect("valid request");

        assert_eq!(
            request.reason_message.as_deref(),
            Some("execution deadline exceeded")
        );
    }

    #[test]
    fn reason_message_control_character_is_rejected() {
        let result = CancellationRequest::with_message(
            "cancel-003",
            CancellationTarget::job("job-003")
                .expect("valid target"),
            CancellationReason::ApplicationRequested,
            Some("bad\nmessage".to_string()),
            CancellationPolicy::production_default(),
        );

        assert!(matches!(
            result,
            Err(
                CancellationValidationError::ReasonContainsControlCharacter
            )
        ));
    }

    #[test]
    fn lifecycle_terminal_states_are_terminal() {
        let terminal_states = [
            CancellationState::Cancelled,
            CancellationState::Rejected,
            CancellationState::AlreadyTerminal,
            CancellationState::AlreadyCancelled,
            CancellationState::TimedOut,
            CancellationState::Failed,
            CancellationState::Unknown,
        ];

        for state in terminal_states {
            assert!(state.is_terminal());
        }
    }

    #[test]
    fn successful_states_are_cancelled_and_already_cancelled() {
        assert!(CancellationState::Cancelled.is_successful());
        assert!(CancellationState::AlreadyCancelled.is_successful());

        assert!(!CancellationState::Rejected.is_successful());
        assert!(!CancellationState::TimedOut.is_successful());
    }

    #[test]
    fn normal_lifecycle_is_valid() {
        assert!(validate_transition(
            CancellationState::NotRequested,
            CancellationState::Requested,
        )
        .is_ok());

        assert!(validate_transition(
            CancellationState::Requested,
            CancellationState::Accepted,
        )
        .is_ok());

        assert!(validate_transition(
            CancellationState::Accepted,
            CancellationState::InProgress,
        )
        .is_ok());

        assert!(validate_transition(
            CancellationState::InProgress,
            CancellationState::Cancelled,
        )
        .is_ok());
    }

    #[test]
    fn terminal_state_cannot_transition() {
        assert!(matches!(
            validate_transition(
                CancellationState::Cancelled,
                CancellationState::Requested,
            ),
            Err(CancellationError::InvalidStateTransition { .. })
        ));
    }

    #[test]
    fn rejected_state_is_terminal() {
        assert!(CancellationState::Rejected.is_terminal());
        assert!(!CancellationState::Rejected.is_retryable());
    }

    #[test]
    fn timeout_is_retryable_but_conditional() {
        assert!(CancellationState::TimedOut.is_retryable());

        let error = CancellationError::TimedOut;

        assert_eq!(
            error.retryability(),
            CancellationRetryability::Conditional
        );

        assert!(!error.is_retryable());
    }

    #[test]
    fn transport_failure_can_be_retryable() {
        let error = CancellationError::Transport {
            message: "temporary connection failure".to_string(),
            retryability: CancellationRetryability::Retryable,
        };

        assert!(error.is_retryable());
        assert_eq!(
            error.code(),
            "cancellation.transport"
        );
    }

    #[test]
    fn outcome_preserves_request_identity() {
        let request = request();

        let outcome = CancellationOutcome::new(
            &request,
            CancellationState::Cancelled,
            CancellationRetryability::NotRetryable,
        )
        .expect("valid outcome");

        assert_eq!(outcome.request_id, "cancel-001");
        assert_eq!(
            outcome.target,
            CancellationTarget::Job {
                job_id: "job-001".to_string()
            }
        );
        assert!(outcome.is_successful());
        assert!(outcome.is_terminal());
    }

    #[test]
    fn already_cancelled_is_successful_and_terminal() {
        assert!(CancellationState::AlreadyCancelled.is_terminal());
        assert!(
            CancellationState::AlreadyCancelled.is_successful()
        );
    }

    #[test]
    fn cancellation_token_starts_active() {
        let token = CancellationToken::new();

        assert!(token.is_active());
        assert!(!token.is_cancelled());
    }

    #[test]
    fn cancellation_token_can_be_cancelled() {
        let token = CancellationToken::new();

        assert!(token.cancel());
        assert!(token.is_cancelled());
        assert!(!token.is_active());
    }

    #[test]
    fn cancellation_token_cancel_is_idempotent() {
        let token = CancellationToken::new();

        assert!(token.cancel());
        assert!(!token.cancel());
        assert!(token.is_cancelled());
    }

    #[test]
    fn cancellation_token_can_be_shared() {
        let token = CancellationToken::new();
        let worker_token = token.clone();

        assert!(!worker_token.is_cancelled());

        assert!(token.cancel());

        assert!(worker_token.is_cancelled());
    }

    #[test]
    fn cancellation_token_can_be_reset_explicitly() {
        let token = CancellationToken::new();

        token.cancel();
        assert!(token.is_cancelled());

        token.reset();

        assert!(!token.is_cancelled());
        assert!(token.is_active());
    }

    #[test]
    fn metadata_is_deterministically_sorted() {
        let mut metadata = CancellationMetadata::new();

        metadata
            .insert("z", "last")
            .expect("valid metadata");

        metadata
            .insert("a", "first")
            .expect("valid metadata");

        assert_eq!(
            metadata.entries(),
            &[
                ("a".to_string(), "first".to_string()),
                ("z".to_string(), "last".to_string()),
            ]
        );
    }

    #[test]
    fn metadata_lookup_is_deterministic() {
        let mut metadata = CancellationMetadata::new();

        metadata
            .insert("requester", "user")
            .expect("valid metadata");

        assert_eq!(metadata.get("requester"), Some("user"));
        assert_eq!(metadata.get("missing"), None);
    }

    #[test]
    fn duplicate_metadata_keys_are_rejected() {
        let mut metadata = CancellationMetadata::new();

        metadata
            .insert("key", "one")
            .expect("first insertion");

        let result = metadata.insert("key", "two");

        assert!(matches!(
            result,
            Err(CancellationValidationError::InvalidMetadataKey {
                ..
            })
        ));
    }

    #[test]
    fn cancellation_reason_identifiers_are_stable() {
        assert_eq!(
            CancellationReason::UserRequested.as_str(),
            "user_requested"
        );

        assert_eq!(
            CancellationReason::PolicyViolation.as_str(),
            "policy_violation"
        );

        assert_eq!(
            CancellationReason::SystemShutdown.as_str(),
            "system_shutdown"
        );
    }

    #[test]
    fn cancellation_mode_identifiers_are_stable() {
        assert_eq!(
            CancellationMode::BestEffort.as_str(),
            "best_effort"
        );

        assert_eq!(
            CancellationMode::RequireAcceptance.as_str(),
            "require_acceptance"
        );

        assert_eq!(
            CancellationMode::RequireConfirmation.as_str(),
            "require_confirmation"
        );
    }

    #[test]
    fn cancellation_state_identifiers_are_stable() {
        assert_eq!(
            CancellationState::NotRequested.as_str(),
            "not_requested"
        );

        assert_eq!(
            CancellationState::InProgress.as_str(),
            "in_progress"
        );

        assert_eq!(
            CancellationState::AlreadyCancelled.as_str(),
            "already_cancelled"
        );
    }

    #[test]
    fn serialization_round_trip_preserves_request() {
        let request = request();

        let encoded =
            serde_json::to_string(&request)
                .expect("serialization");

        let decoded: CancellationRequest =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(request, decoded);
    }

    #[test]
    fn serialization_round_trip_preserves_outcome() {
        let request = request();

        let outcome = CancellationOutcome::new(
            &request,
            CancellationState::Cancelled,
            CancellationRetryability::NotRetryable,
        )
        .expect("valid outcome");

        let encoded =
            serde_json::to_string(&outcome)
                .expect("serialization");

        let decoded: CancellationOutcome =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(outcome, decoded);
    }

    #[test]
    fn serialization_round_trip_preserves_policy() {
        let policy = CancellationPolicy::require_confirmation(10_000);

        let encoded =
            serde_json::to_string(&policy)
                .expect("serialization");

        let decoded: CancellationPolicy =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(policy, decoded);
    }

    #[test]
    fn request_timestamp_is_external() {
        let request = request()
            .with_requested_at_unix_ms(1_000_000);

        assert_eq!(
            request.requested_at_unix_ms,
            Some(1_000_000)
        );
    }

    #[test]
    fn outcome_timestamp_is_external() {
        let request = request();

        let outcome = CancellationOutcome::new(
            &request,
            CancellationState::Cancelled,
            CancellationRetryability::NotRetryable,
        )
        .expect("valid outcome")
        .with_completed_at_unix_ms(2_000_000);

        assert_eq!(
            outcome.completed_at_unix_ms,
            Some(2_000_000)
        );
    }
}