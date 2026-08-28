//! Zamani Quantum — Canonical Hardware Failure Model
//!
//! Production-grade, provider-neutral failure classification and retry policy
//! for `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module models what happened when a hardware operation did not complete
//! successfully and, separately, whether a caller may safely retry it.
//!
//! It owns:
//!
//! - failure phase classification;
//! - failure kind classification;
//! - transient/permanent/unknown failure semantics;
//! - retry safety classification;
//! - request idempotency classification;
//! - retry decisions;
//! - bounded retry policy validation;
//! - deterministic exponential backoff calculation;
//! - retry-attempt accounting;
//! - failure severity;
//! - causal failure chains;
//! - safe provider-neutral failure metadata;
//! - deterministic failure fingerprints;
//! - failure aggregation/statistics;
//! - failure-policy validation;
//! - provider-independent conformance tests.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP clients;
//! - provider SDKs;
//! - authentication;
//! - credentials;
//! - API tokens;
//! - network transports;
//! - sleeping/thread scheduling;
//! - job management;
//! - queue management;
//! - execution;
//! - backend discovery;
//! - benchmark mathematics;
//! - routing;
//! - scheduling;
//! - calibration acquisition;
//! - provider-specific error types.
//!
//! Those systems consume this module.
//!
//! # Architectural position
//!
//! ```text
//! provider / transport / hardware
//!              |
//!              v
//!        provider adapter
//!              |
//!              v
//!       hardware::errors
//!              |
//!              v
//!       hardware::failure
//!              |
//!       +------+------+
//!       |             |
//!       v             v
//!   retry policy   telemetry
//!       |
//!       v
//! execution / job / queue
//! ```
//!
//! `errors.rs` answers:
//!
//! > What error occurred?
//!
//! `failure.rs` answers:
//!
//! > What does this failure mean operationally, and may this operation be
//! > retried safely?
//!
//! Keeping those questions separate prevents the canonical error taxonomy from
//! becoming coupled to execution policy.
//!
//! # Critical safety rule
//!
//! A retry is NEVER inferred merely because an error is classified as
//! transient.
//!
//! Retry requires BOTH:
//!
//! 1. a failure that is potentially retryable; and
//! 2. an operation whose retry semantics are safe.
//!
//! This distinction is critical for quantum execution because blindly
//! resubmitting a provider request can duplicate a physical QPU execution and
//! therefore duplicate cost, consume shots, or create scientifically invalid
//! measurements.
//!
//! # Idempotency
//!
//! The caller must explicitly declare whether retrying the operation is safe.
//!
//! ```text
//! Safe + retryable       -> retry may be permitted
//! Unsafe + retryable     -> retry forbidden
//! Unknown + retryable    -> retry forbidden by default
//! ```
//!
//! The module never assumes that a provider job submission is idempotent.
//!
//! # Determinism
//!
//! This module performs no I/O and reads no clocks or random sources.
//!
//! Backoff calculations are pure functions. The caller supplies any observed
//! retry-after information and decides when to sleep.
//!
//! # Security
//!
//! Failure records MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - raw request bodies containing credentials;
//! - raw provider authentication responses;
//! - arbitrary secret material.
//!
//! Context is deliberately bounded and rejects obvious secret-like keys.
//!
//! # Rust compatibility
//!
//! Target:
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
//! This file intentionally depends only on the Rust standard library.
//!
//! That makes it an independent foundation file.
//!
//! Downstream consumers may use:
//!
//! - `hardware::errors` to obtain the canonical error code;
//! - `hardware::execution` to classify failed execution;
//! - `hardware::job` to record job failure state;
//! - `hardware::queue` to classify queue failures;
//! - provider adapters to construct failure records;
//! - `hardware::telemetry` to aggregate failures;
//! - benchmarking to record reproducibility metadata;
//! - Danga to expose stable failure information.
//!
//! No downstream module needs to modify this file to integrate with it.
//!
//! # Integration with `errors.rs`
//!
//! `errors.rs` remains authoritative for canonical hardware error categories
//! and error codes.
//!
//! This module intentionally does not import `errors.rs` because the failure
//! model must remain independently usable by transports/adapters during error
//! construction.
//!
//! An adapter may therefore map:
//!
//! ```text
//! HardwareError
//!      |
//!      +--> FailureKind
//!      +--> FailurePhase
//!      +--> Retryability
//! ```
//!
//! through its own explicit classification layer.
//!
//! This avoids a dependency cycle and prevents the error taxonomy from owning
//! execution policy.
//!
//! # Integration with execution
//!
//! `execution.rs` should record one `FailureRecord` whenever an accepted
//! workload cannot complete successfully.
//!
//! # Integration with jobs
//!
//! `job.rs` should retain the latest `FailureRecord` for terminal or retryable
//! job failures.
//!
//! # Integration with queues
//!
//! `queue.rs` should classify queue rejection, queue timeout, queue expiry and
//! provider scheduling failures using this model.
//!
//! # Integration with providers
//!
//! Provider adapters should normalize provider failures into this model without
//! adding provider-specific variants to this module.
//!
//! # Integration with telemetry
//!
//! Telemetry should aggregate stable fields such as:
//!
//! - phase;
//! - kind;
//! - permanence;
//! - severity;
//! - retry decision;
//! - provider code;
//! - backend ID;
//! - job ID;
//! - request ID.
//!
//! Human-readable messages MUST NOT be used as metric identifiers.
//!
//! # Stability
//!
//! Public enums and their `as_str()` values are serialization/API contracts.
//! New variants may be added in a backwards-compatible release, but existing
//! stable identifiers MUST NOT be renamed or silently repurposed.
//!
//! # Safety
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! No unsafe operations are required.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const FAILURE_SCHEMA_ID: &str = "zamani.quantum.hardware.failure";

/// Semantic version of this failure contract.
pub const FAILURE_SCHEMA_VERSION: u16 = 1;

/// Maximum failure message length.
pub const MAX_FAILURE_MESSAGE_LENGTH: usize = 4096;

/// Maximum provider error-code length.
pub const MAX_PROVIDER_CODE_LENGTH: usize = 256;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum job identifier length.
pub const MAX_JOB_ID_LENGTH: usize = 1024;

/// Maximum request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

/// Maximum failure context fields.
pub const MAX_CONTEXT_FIELDS: usize = 64;

/// Maximum context key length.
pub const MAX_CONTEXT_KEY_LENGTH: usize = 128;

/// Maximum context value length.
pub const MAX_CONTEXT_VALUE_LENGTH: usize = 1024;

/// Maximum retry attempts allowed by policy.
pub const MAX_RETRY_ATTEMPTS: u32 = 1000;

/// Maximum retry delay in milliseconds.
pub const MAX_RETRY_DELAY_MS: u64 = 86_400_000;

/// Maximum backoff multiplier.
pub const MAX_BACKOFF_MULTIPLIER: u32 = 1024;

// =============================================================================
// Failure phase
// =============================================================================

/// Execution lifecycle phase in which a failure occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailurePhase {
    /// Failure occurred before a request was accepted.
    Preflight,

    /// Capability/compatibility validation failed.
    Validation,

    /// Request could not be submitted.
    Submission,

    /// Request was accepted but waiting in a queue.
    Queue,

    /// Physical/software execution was underway.
    Execution,

    /// Provider/backend was cancelling work.
    Cancellation,

    /// Execution succeeded but result acquisition failed.
    ResultRetrieval,

    /// Result was acquired but could not be normalized.
    ResultNormalization,

    /// Failure occurred while discovering a backend.
    Discovery,

    /// Failure occurred while obtaining backend health/status.
    HealthCheck,

    /// Failure occurred while obtaining calibration information.
    Calibration,

    /// Failure occurred while communicating with a remote provider.
    Transport,

    /// Failure occurred while authenticating.
    Authentication,

    /// Failure occurred while checking authorization.
    Authorization,

    /// Failure occurred while serializing/deserializing data.
    Serialization,

    /// Failure originated in local simulation/emulation.
    LocalExecution,

    /// Internal invariant failure.
    Internal,
}

impl FailurePhase {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preflight => "preflight",
            Self::Validation => "validation",
            Self::Submission => "submission",
            Self::Queue => "queue",
            Self::Execution => "execution",
            Self::Cancellation => "cancellation",
            Self::ResultRetrieval => "result_retrieval",
            Self::ResultNormalization => "result_normalization",
            Self::Discovery => "discovery",
            Self::HealthCheck => "health_check",
            Self::Calibration => "calibration",
            Self::Transport => "transport",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::Serialization => "serialization",
            Self::LocalExecution => "local_execution",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for FailurePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Failure kind
// =============================================================================

/// Provider-neutral operational classification of a failure.
///
/// This is intentionally more specific than a raw error category while
/// remaining independent of any provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailureKind {
    /// Caller supplied an invalid request.
    InvalidRequest,

    /// Requested capability is unavailable.
    CapabilityMismatch,

    /// Hardware resources are insufficient.
    ResourceLimit,

    /// Topology cannot satisfy the workload.
    TopologyMismatch,

    /// Required calibration is unavailable or invalid.
    CalibrationFailure,

    /// Calibration is too old for the requested policy.
    StaleCalibration,

    /// Hardware/backend is temporarily unavailable.
    BackendUnavailable,

    /// Backend is permanently retired.
    BackendRetired,

    /// Queue is temporarily unavailable.
    QueueUnavailable,

    /// Queue deadline expired.
    QueueTimeout,

    /// Provider rejected a request before execution.
    SubmissionRejected,

    /// Provider accepted a request but later rejected execution.
    ExecutionRejected,

    /// Physical execution failed.
    HardwareExecutionFailure,

    /// Provider reported a transient execution failure.
    TransientExecutionFailure,

    /// Provider reported an unrecoverable execution failure.
    PermanentExecutionFailure,

    /// Remote service throttled the request.
    RateLimited,

    /// Remote service temporarily failed.
    ServiceUnavailable,

    /// Network connection failed.
    NetworkFailure,

    /// Network request timed out.
    TransportTimeout,

    /// Operation exceeded its overall execution deadline.
    ExecutionTimeout,

    /// Authentication credentials were rejected.
    AuthenticationFailure,

    /// Authorization failed.
    AuthorizationFailure,

    /// Cancellation was rejected or failed.
    CancellationFailure,

    /// Result was not available.
    ResultUnavailable,

    /// Result payload was malformed.
    InvalidResult,

    /// Result normalization failed.
    ResultNormalizationFailure,

    /// Serialization failed.
    SerializationFailure,

    /// Local simulator/emulator failed.
    LocalExecutionFailure,

    /// Internal invariant was violated.
    InternalInvariant,

    /// Operation is unsupported.
    Unsupported,

    /// Failure cannot be classified reliably.
    Unknown,
}

impl FailureKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::CapabilityMismatch => "capability_mismatch",
            Self::ResourceLimit => "resource_limit",
            Self::TopologyMismatch => "topology_mismatch",
            Self::CalibrationFailure => "calibration_failure",
            Self::StaleCalibration => "stale_calibration",
            Self::BackendUnavailable => "backend_unavailable",
            Self::BackendRetired => "backend_retired",
            Self::QueueUnavailable => "queue_unavailable",
            Self::QueueTimeout => "queue_timeout",
            Self::SubmissionRejected => "submission_rejected",
            Self::ExecutionRejected => "execution_rejected",
            Self::HardwareExecutionFailure => "hardware_execution_failure",
            Self::TransientExecutionFailure => "transient_execution_failure",
            Self::PermanentExecutionFailure => "permanent_execution_failure",
            Self::RateLimited => "rate_limited",
            Self::ServiceUnavailable => "service_unavailable",
            Self::NetworkFailure => "network_failure",
            Self::TransportTimeout => "transport_timeout",
            Self::ExecutionTimeout => "execution_timeout",
            Self::AuthenticationFailure => "authentication_failure",
            Self::AuthorizationFailure => "authorization_failure",
            Self::CancellationFailure => "cancellation_failure",
            Self::ResultUnavailable => "result_unavailable",
            Self::InvalidResult => "invalid_result",
            Self::ResultNormalizationFailure => "result_normalization_failure",
            Self::SerializationFailure => "serialization_failure",
            Self::LocalExecutionFailure => "local_execution_failure",
            Self::InternalInvariant => "internal_invariant",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
        }
    }

    /// Returns the conservative default permanence classification.
    pub const fn default_permanence(self) -> FailurePermanence {
        match self {
            Self::BackendUnavailable
            | Self::QueueUnavailable
            | Self::QueueTimeout
            | Self::RateLimited
            | Self::ServiceUnavailable
            | Self::NetworkFailure
            | Self::TransportTimeout
            | Self::TransientExecutionFailure => FailurePermanence::Transient,

            Self::InvalidRequest
            | Self::CapabilityMismatch
            | Self::ResourceLimit
            | Self::TopologyMismatch
            | Self::StaleCalibration
            | Self::BackendRetired
            | Self::SubmissionRejected
            | Self::ExecutionRejected
            | Self::PermanentExecutionFailure
            | Self::AuthenticationFailure
            | Self::AuthorizationFailure
            | Self::InvalidResult
            | Self::SerializationFailure
            | Self::InternalInvariant
            | Self::Unsupported => FailurePermanence::Permanent,

            Self::CalibrationFailure
            | Self::HardwareExecutionFailure
            | Self::ExecutionTimeout
            | Self::CancellationFailure
            | Self::ResultUnavailable
            | Self::ResultNormalizationFailure
            | Self::LocalExecutionFailure
            | Self::Unknown => FailurePermanence::Unknown,
        }
    }
}

impl fmt::Display for FailureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Failure permanence
// =============================================================================

/// Operational permanence of a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailurePermanence {
    /// External state may change and make the same operation succeed later.
    Transient,

    /// Repeating the same operation without changing inputs/state is not
    /// expected to succeed.
    Permanent,

    /// The system cannot safely determine permanence.
    Unknown,
}

impl FailurePermanence {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::Permanent => "permanent",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the failure is potentially retryable based on
    /// permanence alone.
    pub const fn may_retry(self) -> bool {
        matches!(self, Self::Transient)
    }
}

impl fmt::Display for FailurePermanence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Failure severity
// =============================================================================

/// Operational severity of a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FailureSeverity {
    /// Diagnostic/non-fatal condition.
    Info,

    /// Operation failed but system integrity is unaffected.
    Warning,

    /// Operation failed and caller intervention may be required.
    Error,

    /// Safety/integrity boundary was violated.
    Critical,
}

impl FailureSeverity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Conservative default severity for a failure kind.
    pub const fn default_for(kind: FailureKind) -> Self {
        match kind {
            FailureKind::InternalInvariant
            | FailureKind::HardwareExecutionFailure
            | FailureKind::PermanentExecutionFailure => Self::Critical,

            FailureKind::InvalidRequest
            | FailureKind::CapabilityMismatch
            | FailureKind::ResourceLimit
            | FailureKind::TopologyMismatch
            | FailureKind::AuthenticationFailure
            | FailureKind::AuthorizationFailure
            | FailureKind::Unsupported
            | FailureKind::BackendRetired
            | FailureKind::InvalidResult => Self::Error,

            _ => Self::Error,
        }
    }
}

impl fmt::Display for FailureSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Retry safety
// =============================================================================

/// Whether repeating an operation is semantically safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetrySafety {
    /// Repeating the operation is explicitly safe.
    Safe,

    /// Repeating the operation may duplicate externally observable work.
    Unsafe,

    /// Safety was not established.
    Unknown,
}

impl RetrySafety {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Unsafe => "unsafe",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for RetrySafety {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Idempotency
// =============================================================================

/// Idempotency state of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Idempotency {
    /// Provider/request contract guarantees repeated submission has the same
    /// externally observable effect.
    Idempotent,

    /// Repeating the request may produce another physical execution or other
    /// side effect.
    NonIdempotent,

    /// Idempotency cannot be established.
    Unknown,
}

impl Idempotency {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idempotent => "idempotent",
            Self::NonIdempotent => "non_idempotent",
            Self::Unknown => "unknown",
        }
    }

    /// Converts idempotency to the conservative retry-safety model.
    pub const fn retry_safety(self) -> RetrySafety {
        match self {
            Self::Idempotent => RetrySafety::Safe,
            Self::NonIdempotent => RetrySafety::Unsafe,
            Self::Unknown => RetrySafety::Unknown,
        }
    }
}

impl fmt::Display for Idempotency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Retry decision
// =============================================================================

/// Final decision produced by the retry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetryDecision {
    /// Retry is permitted and should normally be attempted.
    Retry,

    /// Retry is permitted only if caller-specific conditions are satisfied.
    RetryConditional,

    /// Retry is forbidden.
    DoNotRetry,

    /// The policy cannot establish a safe decision.
    Unknown,
}

impl RetryDecision {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::RetryConditional => "retry_conditional",
            Self::DoNotRetry => "do_not_retry",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether retry is allowed without additional caller policy.
    pub const fn is_retry(self) -> bool {
        matches!(self, Self::Retry)
    }

    /// Returns whether the decision permits a retry under some condition.
    pub const fn may_retry(self) -> bool {
        matches!(self, Self::Retry | Self::RetryConditional)
    }
}

impl fmt::Display for RetryDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Retry reason
// =============================================================================

/// Machine-readable reason for a retry decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetryReason {
    /// Failure is permanently unrecoverable.
    PermanentFailure,

    /// Failure is transient and operation is safely retryable.
    TransientAndSafe,

    /// Failure is transient but operation is not known to be safe.
    RetrySafetyUnknown,

    /// Operation is explicitly non-idempotent.
    NonIdempotent,

    /// Maximum retry attempts were exhausted.
    RetryLimitReached,

    /// Retry policy disabled retries.
    RetryPolicyDisabled,

    /// Retry delay exceeds configured maximum.
    DelayLimitReached,

    /// Retry requires a condition not currently satisfied.
    ConditionalPolicy,

    /// Failure classification is unknown.
    UnknownFailure,

    /// Failure is not retryable by semantics.
    NotRetryable,
}

impl RetryReason {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PermanentFailure => "permanent_failure",
            Self::TransientAndSafe => "transient_and_safe",
            Self::RetrySafetyUnknown => "retry_safety_unknown",
            Self::NonIdempotent => "non_idempotent",
            Self::RetryLimitReached => "retry_limit_reached",
            Self::RetryPolicyDisabled => "retry_policy_disabled",
            Self::DelayLimitReached => "delay_limit_reached",
            Self::ConditionalPolicy => "conditional_policy",
            Self::UnknownFailure => "unknown_failure",
            Self::NotRetryable => "not_retryable",
        }
    }
}

impl fmt::Display for RetryReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Retry-after hint
// =============================================================================

/// Provider/server hint for when another attempt may be appropriate.
///
/// This is data only. The failure module never sleeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RetryAfter {
    milliseconds: u64,
}

impl RetryAfter {
    /// Creates a retry-after hint.
    pub fn from_millis(milliseconds: u64) -> Result<Self, RetryAfterError> {
        if milliseconds > MAX_RETRY_DELAY_MS {
            return Err(RetryAfterError::TooLarge {
                milliseconds,
                maximum: MAX_RETRY_DELAY_MS,
            });
        }

        Ok(Self { milliseconds })
    }

    /// Returns the delay in milliseconds.
    pub const fn as_millis(self) -> u64 {
        self.milliseconds
    }
}

/// Invalid retry-after hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryAfterError {
    /// Delay exceeded the safety bound.
    TooLarge {
        milliseconds: u64,
        maximum: u64,
    },
}

impl fmt::Display for RetryAfterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge {
                milliseconds,
                maximum,
            } => write!(
                f,
                "retry-after delay {} ms exceeds maximum {} ms",
                milliseconds, maximum
            ),
        }
    }
}

impl std::error::Error for RetryAfterError {}

// =============================================================================
// Backoff
// =============================================================================

/// Deterministic exponential backoff configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackoffPolicy {
    /// Initial delay in milliseconds.
    pub initial_delay_ms: u64,

    /// Maximum delay in milliseconds.
    pub maximum_delay_ms: u64,

    /// Multiplication factor.
    pub multiplier: u32,
}

impl BackoffPolicy {
    /// Creates and validates a backoff policy.
    pub fn new(
        initial_delay_ms: u64,
        maximum_delay_ms: u64,
        multiplier: u32,
    ) -> Result<Self, BackoffPolicyError> {
        if initial_delay_ms > MAX_RETRY_DELAY_MS {
            return Err(BackoffPolicyError::InitialDelayTooLarge {
                value: initial_delay_ms,
            });
        }

        if maximum_delay_ms > MAX_RETRY_DELAY_MS {
            return Err(BackoffPolicyError::MaximumDelayTooLarge {
                value: maximum_delay_ms,
            });
        }

        if maximum_delay_ms < initial_delay_ms {
            return Err(BackoffPolicyError::MaximumBelowInitial);
        }

        if multiplier == 0 {
            return Err(BackoffPolicyError::ZeroMultiplier);
        }

        if multiplier > MAX_BACKOFF_MULTIPLIER {
            return Err(BackoffPolicyError::MultiplierTooLarge {
                value: multiplier,
            });
        }

        Ok(Self {
            initial_delay_ms,
            maximum_delay_ms,
            multiplier,
        })
    }

    /// Returns a conservative production default.
    pub const fn default_production() -> Self {
        Self {
            initial_delay_ms: 100,
            maximum_delay_ms: 30_000,
            multiplier: 2,
        }
    }

    /// Calculates a deterministic delay for an attempt.
    ///
    /// Attempt zero returns the initial delay.
    ///
    /// Overflow saturates safely and is then clamped to the configured
    /// maximum.
    pub fn delay_for_attempt(self, attempt: u32) -> u64 {
        let mut delay = self.initial_delay_ms;
        let mut remaining = attempt;

        while remaining > 0 {
            delay = delay.saturating_mul(u64::from(self.multiplier));

            if delay >= self.maximum_delay_ms {
                return self.maximum_delay_ms;
            }

            remaining -= 1;
        }

        delay.min(self.maximum_delay_ms)
    }
}

/// Errors constructing a backoff policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackoffPolicyError {
    /// Initial delay is too large.
    InitialDelayTooLarge { value: u64 },

    /// Maximum delay is too large.
    MaximumDelayTooLarge { value: u64 },

    /// Maximum delay cannot be smaller than initial delay.
    MaximumBelowInitial,

    /// Multiplier cannot be zero.
    ZeroMultiplier,

    /// Multiplier exceeds safety limit.
    MultiplierTooLarge { value: u32 },
}

impl fmt::Display for BackoffPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitialDelayTooLarge { value } => {
                write!(f, "initial retry delay {} ms is too large", value)
            }
            Self::MaximumDelayTooLarge { value } => {
                write!(f, "maximum retry delay {} ms is too large", value)
            }
            Self::MaximumBelowInitial => {
                f.write_str("maximum retry delay cannot be below initial delay")
            }
            Self::ZeroMultiplier => {
                f.write_str("retry backoff multiplier cannot be zero")
            }
            Self::MultiplierTooLarge { value } => {
                write!(f, "retry backoff multiplier {} is too large", value)
            }
        }
    }
}

impl std::error::Error for BackoffPolicyError {}

// =============================================================================
// Retry policy
// =============================================================================

/// Complete retry policy.
///
/// This structure contains policy only. It never performs the retry itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetryPolicy {
    /// Whether retries are globally enabled.
    pub enabled: bool,

    /// Maximum number of retries after the initial attempt.
    pub maximum_retries: u32,

    /// Whether an unknown failure may be retried.
    ///
    /// Production defaults should keep this false.
    pub retry_unknown_failures: bool,

    /// Whether unknown idempotency may be retried.
    ///
    /// Production defaults should keep this false.
    pub retry_unknown_idempotency: bool,

    /// Whether an explicitly transient failure may be retried when the
    /// operation is known to be idempotent.
    pub retry_transient: bool,

    /// Deterministic backoff policy.
    pub backoff: BackoffPolicy,
}

impl RetryPolicy {
    /// Conservative production policy.
    pub const fn production() -> Self {
        Self {
            enabled: true,
            maximum_retries: 3,
            retry_unknown_failures: false,
            retry_unknown_idempotency: false,
            retry_transient: true,
            backoff: BackoffPolicy::default_production(),
        }
    }

    /// Policy that completely disables automatic retry.
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            maximum_retries: 0,
            retry_unknown_failures: false,
            retry_unknown_idempotency: false,
            retry_transient: false,
            backoff: BackoffPolicy::default_production(),
        }
    }

    /// Validates the policy.
    pub const fn validate(self) -> Result<(), RetryPolicyError> {
        if self.maximum_retries > MAX_RETRY_ATTEMPTS {
            return Err(RetryPolicyError::TooManyRetries {
                value: self.maximum_retries,
            });
        }

        if self.enabled && self.maximum_retries == 0 {
            return Err(RetryPolicyError::EnabledWithoutRetries);
        }

        Ok(())
    }

    /// Calculates a retry decision.
    ///
    /// `attempt` is zero-based:
    ///
    /// - attempt 0 = first execution;
    /// - attempt 1 = first retry;
    /// - attempt 2 = second retry.
    pub const fn decide(
        self,
        failure: &FailureRecord,
        attempt: u32,
    ) -> RetryDecisionResult {
        if !self.enabled {
            return RetryDecisionResult::new(
                RetryDecision::DoNotRetry,
                RetryReason::RetryPolicyDisabled,
            );
        }

        if attempt >= self.maximum_retries {
            return RetryDecisionResult::new(
                RetryDecision::DoNotRetry,
                RetryReason::RetryLimitReached,
            );
        }

        match failure.permanence {
            FailurePermanence::Permanent => {
                return RetryDecisionResult::new(
                    RetryDecision::DoNotRetry,
                    RetryReason::PermanentFailure,
                );
            }

            FailurePermanence::Unknown => {
                if !self.retry_unknown_failures {
                    return RetryDecisionResult::new(
                        RetryDecision::DoNotRetry,
                        RetryReason::UnknownFailure,
                    );
                }
            }

            FailurePermanence::Transient => {}
        }

        if !self.retry_transient
            && failure.permanence == FailurePermanence::Transient
        {
            return RetryDecisionResult::new(
                RetryDecision::DoNotRetry,
                RetryReason::NotRetryable,
            );
        }

        match failure.retry_safety {
            RetrySafety::Unsafe => {
                return RetryDecisionResult::new(
                    RetryDecision::DoNotRetry,
                    RetryReason::NonIdempotent,
                );
            }

            RetrySafety::Unknown => {
                if !self.retry_unknown_idempotency {
                    return RetryDecisionResult::new(
                        RetryDecision::DoNotRetry,
                        RetryReason::RetrySafetyUnknown,
                    );
                }

                return RetryDecisionResult::new(
                    RetryDecision::RetryConditional,
                    RetryReason::ConditionalPolicy,
                );
            }

            RetrySafety::Safe => {}
        }

        RetryDecisionResult::new(
            RetryDecision::Retry,
            RetryReason::TransientAndSafe,
        )
    }

    /// Calculates the deterministic backoff delay for a retry attempt.
    pub fn delay_for_attempt(self, attempt: u32) -> u64 {
        self.backoff.delay_for_attempt(attempt)
    }
}

/// Retry policy validation failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryPolicyError {
    /// Retry count exceeds the hard safety limit.
    TooManyRetries { value: u32 },

    /// Retries are enabled but zero retries are configured.
    EnabledWithoutRetries,
}

impl fmt::Display for RetryPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyRetries { value } => {
                write!(f, "retry count {} exceeds safety limit", value)
            }
            Self::EnabledWithoutRetries => {
                f.write_str("retry policy is enabled but maximum retries is zero")
            }
        }
    }
}

impl std::error::Error for RetryPolicyError {}

// =============================================================================
// Retry decision result
// =============================================================================

/// Retry decision plus its stable reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RetryDecisionResult {
    /// Final decision.
    pub decision: RetryDecision,

    /// Reason for that decision.
    pub reason: RetryReason,
}

impl RetryDecisionResult {
    /// Creates a decision result.
    pub const fn new(
        decision: RetryDecision,
        reason: RetryReason,
    ) -> Self {
        Self { decision, reason }
    }
}

// =============================================================================
// Failure context
// =============================================================================

/// Bounded deterministic diagnostic context.
///
/// This is intentionally not a general-purpose metadata store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureContext {
    fields: BTreeMap<String, String>,
}

impl Default for FailureContext {
    fn default() -> Self {
        Self::new()
    }
}

impl FailureContext {
    /// Creates an empty context.
    pub fn new() -> Self {
        Self {
            fields: BTreeMap::new(),
        }
    }

    /// Inserts a safe diagnostic field.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), FailureContextError> {
        let key = key.into();
        let value = value.into();

        validate_context_key(&key)?;
        validate_context_value(&value)?;

        if is_secret_like(&key) {
            return Err(FailureContextError::SensitiveKey);
        }

        if self.fields.len() >= MAX_CONTEXT_FIELDS
            && !self.fields.contains_key(&key)
        {
            return Err(FailureContextError::TooManyFields);
        }

        self.fields.insert(key, value);
        Ok(())
    }

    /// Gets a field.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(String::as_str)
    }

    /// Returns the number of fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns whether the context is empty.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns deterministic field iteration.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.fields
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

/// Failure context validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailureContextError {
    /// Context key is empty.
    EmptyKey,

    /// Context key is too long.
    KeyTooLong,

    /// Context key contains forbidden control characters.
    InvalidKey,

    /// Context value is too long.
    ValueTooLong,

    /// Context value contains forbidden control characters.
    InvalidValue,

    /// Key appears to contain secret material.
    SensitiveKey,

    /// Context field limit was exceeded.
    TooManyFields,
}

impl fmt::Display for FailureContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => f.write_str("failure context key cannot be empty"),
            Self::KeyTooLong => f.write_str("failure context key is too long"),
            Self::InvalidKey => {
                f.write_str("failure context key contains invalid characters")
            }
            Self::ValueTooLong => {
                f.write_str("failure context value is too long")
            }
            Self::InvalidValue => {
                f.write_str("failure context value contains invalid characters")
            }
            Self::SensitiveKey => {
                f.write_str("failure context key appears to contain secret material")
            }
            Self::TooManyFields => {
                f.write_str("failure context contains too many fields")
            }
        }
    }
}

impl std::error::Error for FailureContextError {}

// =============================================================================
// Failure record
// =============================================================================

/// Complete provider-neutral failure record.
///
/// This is the primary integration type for execution, jobs, queues, adapters,
/// telemetry and benchmarking.
///
/// The record is immutable after construction except for its bounded diagnostic
/// context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureRecord {
    /// Failure phase.
    pub phase: FailurePhase,

    /// Provider-neutral failure kind.
    pub kind: FailureKind,

    /// Failure permanence.
    pub permanence: FailurePermanence,

    /// Failure severity.
    pub severity: FailureSeverity,

    /// Retry safety.
    pub retry_safety: RetrySafety,

    /// Idempotency classification used to derive retry safety.
    pub idempotency: Idempotency,

    /// Safe human-readable message.
    message: String,

    /// Optional provider-native error code.
    provider_code: Option<String>,

    /// Optional backend identifier.
    backend_id: Option<String>,

    /// Optional job identifier.
    job_id: Option<String>,

    /// Optional request identifier.
    request_id: Option<String>,

    /// Optional provider retry-after hint.
    retry_after: Option<RetryAfter>,

    /// Bounded structured context.
    context: FailureContext,

    /// Optional causal parent.
    cause: Option<Box<FailureRecord>>,
}

impl FailureRecord {
    /// Creates a failure record using the conservative default permanence and
    /// severity for the supplied kind.
    pub fn new(
        phase: FailurePhase,
        kind: FailureKind,
        message: impl Into<String>,
        idempotency: Idempotency,
    ) -> Result<Self, FailureRecordError> {
        let message = message.into();

        validate_message(&message)?;

        let permanence = kind.default_permanence();
        let severity = FailureSeverity::default_for(kind);
        let retry_safety = idempotency.retry_safety();

        Ok(Self {
            phase,
            kind,
            permanence,
            severity,
            retry_safety,
            idempotency,
            message,
            provider_code: None,
            backend_id: None,
            job_id: None,
            request_id: None,
            retry_after: None,
            context: FailureContext::new(),
            cause: None,
        })
    }

    /// Overrides permanence explicitly.
    pub const fn with_permanence(
        mut self,
        permanence: FailurePermanence,
    ) -> Self {
        self.permanence = permanence;
        self
    }

    /// Overrides severity explicitly.
    pub const fn with_severity(
        mut self,
        severity: FailureSeverity,
    ) -> Self {
        self.severity = severity;
        self
    }

    /// Overrides retry safety explicitly.
    ///
    /// This should only be used when the caller has stronger evidence than the
    /// idempotency classification.
    pub const fn with_retry_safety(
        mut self,
        retry_safety: RetrySafety,
    ) -> Self {
        self.retry_safety = retry_safety;
        self
    }

    /// Adds a provider-native code after validating it.
    pub fn with_provider_code(
        mut self,
        code: impl Into<String>,
    ) -> Result<Self, FailureRecordError> {
        let code = code.into();

        if code.is_empty() {
            return Err(FailureRecordError::EmptyProviderCode);
        }

        if code.len() > MAX_PROVIDER_CODE_LENGTH {
            return Err(FailureRecordError::ProviderCodeTooLong);
        }

        if contains_control(&code) {
            return Err(FailureRecordError::InvalidProviderCode);
        }

        if is_secret_like(&code) {
            return Err(FailureRecordError::SensitiveProviderCode);
        }

        self.provider_code = Some(code);
        Ok(self)
    }

    /// Adds a backend identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, FailureRecordError> {
        let backend_id = backend_id.into();

        validate_identifier(
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
            FailureRecordError::InvalidBackendId,
            FailureRecordError::BackendIdTooLong,
        )?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Adds a job identifier.
    pub fn with_job_id(
        mut self,
        job_id: impl Into<String>,
    ) -> Result<Self, FailureRecordError> {
        let job_id = job_id.into();

        validate_identifier(
            &job_id,
            MAX_JOB_ID_LENGTH,
            FailureRecordError::InvalidJobId,
            FailureRecordError::JobIdTooLong,
        )?;

        self.job_id = Some(job_id);
        Ok(self)
    }

    /// Adds a request identifier.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, FailureRecordError> {
        let request_id = request_id.into();

        validate_identifier(
            &request_id,
            MAX_REQUEST_ID_LENGTH,
            FailureRecordError::InvalidRequestId,
            FailureRecordError::RequestIdTooLong,
        )?;

        self.request_id = Some(request_id);
        Ok(self)
    }

    /// Adds a retry-after hint.
    pub fn with_retry_after(
        mut self,
        retry_after: RetryAfter,
    ) -> Self {
        self.retry_after = Some(retry_after);
        self
    }

    /// Adds structured diagnostic context.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, FailureRecordError> {
        self.context
            .insert(key, value)
            .map_err(FailureRecordError::Context)?;
        Ok(self)
    }

    /// Adds a causal failure.
    ///
    /// The causal record is bounded through the same validation rules as an
    /// ordinary failure record. Callers should avoid constructing excessively
    /// deep chains.
    pub fn with_cause(
        mut self,
        cause: FailureRecord,
    ) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// Returns the safe human-readable message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the provider code, if present.
    pub fn provider_code(&self) -> Option<&str> {
        self.provider_code.as_deref()
    }

    /// Returns the backend identifier, if present.
    pub fn backend_id(&self) -> Option<&str> {
        self.backend_id.as_deref()
    }

    /// Returns the job identifier, if present.
    pub fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Returns the request identifier, if present.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Returns the retry-after hint.
    pub const fn retry_after(&self) -> Option<RetryAfter> {
        self.retry_after
    }

    /// Returns diagnostic context.
    pub fn context(&self) -> &FailureContext {
        &self.context
    }

    /// Returns the causal failure.
    pub fn cause(&self) -> Option<&FailureRecord> {
        self.cause.as_deref()
    }

    /// Returns whether this record represents a critical failure.
    pub const fn is_critical(&self) -> bool {
        matches!(self.severity, FailureSeverity::Critical)
    }

    /// Returns whether this record is terminal by permanence.
    pub const fn is_terminal(&self) -> bool {
        matches!(self.permanence, FailurePermanence::Permanent)
    }

    /// Calculates a deterministic retry decision.
    pub const fn retry_decision(
        &self,
        policy: RetryPolicy,
        attempt: u32,
    ) -> RetryDecisionResult {
        policy.decide(self, attempt)
    }

    /// Calculates a stable fingerprint.
    ///
    /// The fingerprint deliberately excludes the free-form human-readable
    /// message and arbitrary context so that changing diagnostic wording does
    /// not create a different failure class.
    pub fn fingerprint(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        self.phase.hash(&mut hasher);
        self.kind.hash(&mut hasher);
        self.permanence.hash(&mut hasher);
        self.severity.hash(&mut hasher);
        self.retry_safety.hash(&mut hasher);
        self.idempotency.hash(&mut hasher);

        if let Some(provider_code) = &self.provider_code {
            provider_code.hash(&mut hasher);
        }

        hasher.finish()
    }
}

/// Failure record validation/build error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailureRecordError {
    /// Message is empty.
    EmptyMessage,

    /// Message is too long.
    MessageTooLong,

    /// Message contains control characters.
    InvalidMessage,

    /// Provider code is empty.
    EmptyProviderCode,

    /// Provider code is too long.
    ProviderCodeTooLong,

    /// Provider code contains invalid characters.
    InvalidProviderCode,

    /// Provider code appears sensitive.
    SensitiveProviderCode,

    /// Backend identifier is invalid.
    InvalidBackendId,

    /// Backend identifier is too long.
    BackendIdTooLong,

    /// Job identifier is invalid.
    InvalidJobId,

    /// Job identifier is too long.
    JobIdTooLong,

    /// Request identifier is invalid.
    InvalidRequestId,

    /// Request identifier is too long.
    RequestIdTooLong,

    /// Structured context failed validation.
    Context(FailureContextError),
}

impl fmt::Display for FailureRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMessage => f.write_str("failure message cannot be empty"),
            Self::MessageTooLong => f.write_str("failure message is too long"),
            Self::InvalidMessage => {
                f.write_str("failure message contains invalid control characters")
            }
            Self::EmptyProviderCode => {
                f.write_str("provider error code cannot be empty")
            }
            Self::ProviderCodeTooLong => {
                f.write_str("provider error code is too long")
            }
            Self::InvalidProviderCode => {
                f.write_str("provider error code contains invalid characters")
            }
            Self::SensitiveProviderCode => {
                f.write_str("provider error code appears to contain secret material")
            }
            Self::InvalidBackendId => {
                f.write_str("backend identifier is invalid")
            }
            Self::BackendIdTooLong => {
                f.write_str("backend identifier is too long")
            }
            Self::InvalidJobId => {
                f.write_str("job identifier is invalid")
            }
            Self::JobIdTooLong => {
                f.write_str("job identifier is too long")
            }
            Self::InvalidRequestId => {
                f.write_str("request identifier is invalid")
            }
            Self::RequestIdTooLong => {
                f.write_str("request identifier is too long")
            }
            Self::Context(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for FailureRecordError {}

// =============================================================================
// Failure chain
// =============================================================================

/// Bounded causal failure chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureChain {
    failures: Vec<FailureRecord>,
}

impl FailureChain {
    /// Maximum causal-chain depth.
    pub const MAX_DEPTH: usize = 32;

    /// Creates a chain containing the supplied failure.
    pub fn new(root: FailureRecord) -> Self {
        Self {
            failures: vec![root],
        }
    }

    /// Appends a failure if the depth limit has not been reached.
    pub fn push(&mut self, failure: FailureRecord) -> Result<(), FailureChainError> {
        if self.failures.len() >= Self::MAX_DEPTH {
            return Err(FailureChainError::TooDeep);
        }

        self.failures.push(failure);
        Ok(())
    }

    /// Returns the chain depth.
    pub fn len(&self) -> usize {
        self.failures.len()
    }

    /// Returns whether the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.failures.is_empty()
    }

    /// Returns failures in causal order.
    pub fn iter(&self) -> impl Iterator<Item = &FailureRecord> {
        self.failures.iter()
    }

    /// Returns the root failure.
    pub fn root(&self) -> Option<&FailureRecord> {
        self.failures.first()
    }

    /// Returns the most recent failure.
    pub fn latest(&self) -> Option<&FailureRecord> {
        self.failures.last()
    }
}

/// Failure-chain construction error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureChainError {
    /// Causal chain exceeded its hard depth bound.
    TooDeep,
}

impl fmt::Display for FailureChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooDeep => {
                f.write_str("failure causal chain exceeds maximum depth")
            }
        }
    }
}

impl std::error::Error for FailureChainError {}

// =============================================================================
// Failure statistics
// =============================================================================

/// Deterministic aggregate statistics for a collection of failures.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FailureStatistics {
    by_phase: BTreeMap<FailurePhase, u64>,
    by_kind: BTreeMap<FailureKind, u64>,
    by_permanence: BTreeMap<FailurePermanence, u64>,
    by_severity: BTreeMap<FailureSeverity, u64>,
    retryable: u64,
    non_retryable: u64,
}

impl FailureStatistics {
    /// Creates empty statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one failure.
    pub fn record(&mut self, failure: &FailureRecord) {
        increment(&mut self.by_phase, failure.phase);
        increment(&mut self.by_kind, failure.kind);
        increment(&mut self.by_permanence, failure.permanence);
        increment(&mut self.by_severity, failure.severity);

        if failure.retry_safety == RetrySafety::Safe
            && failure.permanence == FailurePermanence::Transient
        {
            self.retryable = self.retryable.saturating_add(1);
        } else {
            self.non_retryable = self.non_retryable.saturating_add(1);
        }
    }

    /// Number of recorded failures.
    pub fn total(&self) -> u64 {
        self.by_kind.values().copied().sum()
    }

    /// Number of potentially retryable failures.
    pub const fn retryable(&self) -> u64 {
        self.retryable
    }

    /// Number of non-retryable/unknown-safety failures.
    pub const fn non_retryable(&self) -> u64 {
        self.non_retryable
    }

    /// Count by phase.
    pub fn phase_count(&self, phase: FailurePhase) -> u64 {
        self.by_phase.get(&phase).copied().unwrap_or(0)
    }

    /// Count by failure kind.
    pub fn kind_count(&self, kind: FailureKind) -> u64 {
        self.by_kind.get(&kind).copied().unwrap_or(0)
    }

    /// Count by permanence.
    pub fn permanence_count(&self, permanence: FailurePermanence) -> u64 {
        self.by_permanence.get(&permanence).copied().unwrap_or(0)
    }

    /// Count by severity.
    pub fn severity_count(&self, severity: FailureSeverity) -> u64 {
        self.by_severity.get(&severity).copied().unwrap_or(0)
    }
}

fn increment<K>(map: &mut BTreeMap<K, u64>, key: K)
where
    K: Ord,
{
    let entry = map.entry(key).or_insert(0);
    *entry = entry.saturating_add(1);
}

// =============================================================================
// Classification helpers
// =============================================================================

/// Classifies a failure using the conservative default policy.
pub const fn classify_failure(
    kind: FailureKind,
    idempotency: Idempotency,
) -> (FailurePermanence, RetrySafety, FailureSeverity) {
    (
        kind.default_permanence(),
        idempotency.retry_safety(),
        FailureSeverity::default_for(kind),
    )
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_message(message: &str) -> Result<(), FailureRecordError> {
    if message.is_empty() {
        return Err(FailureRecordError::EmptyMessage);
    }

    if message.len() > MAX_FAILURE_MESSAGE_LENGTH {
        return Err(FailureRecordError::MessageTooLong);
    }

    if contains_control(message) {
        return Err(FailureRecordError::InvalidMessage);
    }

    Ok(())
}

fn validate_identifier<E1, E2>(
    value: &str,
    maximum: usize,
    invalid: E1,
    too_long: E2,
) -> Result<(), E1>
where
    E1: From<E2>,
{
    if value.is_empty() {
        return Err(invalid);
    }

    if value.len() > maximum {
        return Err(invalid);
    }

    if contains_control(value) {
        return Err(invalid);
    }

    if value.trim() != value {
        return Err(invalid);
    }

    Ok(())
}

fn validate_context_key(key: &str) -> Result<(), FailureContextError> {
    if key.is_empty() {
        return Err(FailureContextError::EmptyKey);
    }

    if key.len() > MAX_CONTEXT_KEY_LENGTH {
        return Err(FailureContextError::KeyTooLong);
    }

    if contains_control(key) {
        return Err(FailureContextError::InvalidKey);
    }

    Ok(())
}

fn validate_context_value(
    value: &str,
) -> Result<(), FailureContextError> {
    if value.len() > MAX_CONTEXT_VALUE_LENGTH {
        return Err(FailureContextError::ValueTooLong);
    }

    if contains_control(value) {
        return Err(FailureContextError::InvalidValue);
    }

    Ok(())
}

fn contains_control(value: &str) -> bool {
    value.chars().any(char::is_control)
}

fn is_secret_like(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const SECRET_TERMS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "accesstoken",
        "authorization",
        "bearer",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "secret",
        "cookie",
        "session_token",
        "sessiontoken",
        "credential",
        "credentials",
    ];

    SECRET_TERMS
        .iter()
        .any(|term| normalized.contains(term))
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for FailureRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}",
            self.phase,
            self.kind,
            self.permanence,
            self.severity
        )?;

        if let Some(provider_code) = &self.provider_code {
            write!(f, " provider_code={}", provider_code)?;
        }

        if let Some(backend_id) = &self.backend_id {
            write!(f, " backend_id={}", backend_id)?;
        }

        if let Some(job_id) = &self.job_id {
            write!(f, " job_id={}", job_id)?;
        }

        if let Some(request_id) = &self.request_id {
            write!(f, " request_id={}", request_id)?;
        }

        write!(f, ": {}", self.message)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            FAILURE_SCHEMA_ID,
            "zamani.quantum.hardware.failure"
        );
        assert_eq!(FAILURE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn failure_kind_identifiers_are_stable() {
        assert_eq!(
            FailureKind::RateLimited.as_str(),
            "rate_limited"
        );
        assert_eq!(
            FailureKind::HardwareExecutionFailure.as_str(),
            "hardware_execution_failure"
        );
        assert_eq!(
            FailureKind::Unknown.as_str(),
            "unknown"
        );
    }

    #[test]
    fn permanent_failure_is_not_retryable() {
        let failure = FailureRecord::new(
            FailurePhase::Validation,
            FailureKind::InvalidRequest,
            "invalid workload",
            Idempotency::Idempotent,
        )
        .expect("valid failure");

        let result = RetryPolicy::production().decide(&failure, 0);

        assert_eq!(result.decision, RetryDecision::DoNotRetry);
        assert_eq!(
            result.reason,
            RetryReason::PermanentFailure
        );
    }

    #[test]
    fn transient_idempotent_failure_is_retryable() {
        let failure = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "temporary network failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure");

        let result = RetryPolicy::production().decide(&failure, 0);

        assert_eq!(result.decision, RetryDecision::Retry);
        assert_eq!(
            result.reason,
            RetryReason::TransientAndSafe
        );
    }

    #[test]
    fn transient_non_idempotent_failure_is_not_retryable() {
        let failure = FailureRecord::new(
            FailurePhase::Submission,
            FailureKind::ServiceUnavailable,
            "provider unavailable",
            Idempotency::NonIdempotent,
        )
        .expect("valid failure");

        let result = RetryPolicy::production().decide(&failure, 0);

        assert_eq!(
            result.decision,
            RetryDecision::DoNotRetry
        );
        assert_eq!(
            result.reason,
            RetryReason::NonIdempotent
        );
    }

    #[test]
    fn unknown_idempotency_is_conservative() {
        let failure = FailureRecord::new(
            FailurePhase::Submission,
            FailureKind::NetworkFailure,
            "connection lost",
            Idempotency::Unknown,
        )
        .expect("valid failure");

        let result = RetryPolicy::production().decide(&failure, 0);

        assert_eq!(
            result.decision,
            RetryDecision::DoNotRetry
        );
        assert_eq!(
            result.reason,
            RetryReason::RetrySafetyUnknown
        );
    }

    #[test]
    fn unknown_failure_is_conservative() {
        let failure = FailureRecord::new(
            FailurePhase::Execution,
            FailureKind::Unknown,
            "unknown provider failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure");

        let result = RetryPolicy::production().decide(&failure, 0);

        assert_eq!(
            result.decision,
            RetryDecision::DoNotRetry
        );
        assert_eq!(
            result.reason,
            RetryReason::UnknownFailure
        );
    }

    #[test]
    fn retry_limit_is_enforced() {
        let failure = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "temporary failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure");

        let policy = RetryPolicy::production();

        let result = policy.decide(&failure, policy.maximum_retries);

        assert_eq!(
            result.decision,
            RetryDecision::DoNotRetry
        );
        assert_eq!(
            result.reason,
            RetryReason::RetryLimitReached
        );
    }

    #[test]
    fn disabled_policy_never_retries() {
        let failure = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "temporary failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure");

        let result = RetryPolicy::disabled().decide(&failure, 0);

        assert_eq!(
            result.decision,
            RetryDecision::DoNotRetry
        );
        assert_eq!(
            result.reason,
            RetryReason::RetryPolicyDisabled
        );
    }

    #[test]
    fn exponential_backoff_is_deterministic() {
        let policy = BackoffPolicy::new(100, 1_000, 2)
            .expect("valid policy");

        assert_eq!(policy.delay_for_attempt(0), 100);
        assert_eq!(policy.delay_for_attempt(1), 200);
        assert_eq!(policy.delay_for_attempt(2), 400);
        assert_eq!(policy.delay_for_attempt(3), 800);
        assert_eq!(policy.delay_for_attempt(4), 1_000);
        assert_eq!(policy.delay_for_attempt(100), 1_000);
    }

    #[test]
    fn backoff_saturates_without_overflow() {
        let policy = BackoffPolicy::new(
            u64::MAX / 2,
            u64::MAX,
            1024,
        )
        .expect("valid policy");

        assert_eq!(
            policy.delay_for_attempt(100),
            u64::MAX
        );
    }

    #[test]
    fn backoff_rejects_invalid_configuration() {
        assert_eq!(
            BackoffPolicy::new(1_000, 100, 2),
            Err(BackoffPolicyError::MaximumBelowInitial)
        );

        assert_eq!(
            BackoffPolicy::new(100, 1_000, 0),
            Err(BackoffPolicyError::ZeroMultiplier)
        );
    }

    #[test]
    fn context_is_deterministic() {
        let mut context = FailureContext::new();

        context
            .insert("z", "last")
            .expect("valid context");

        context
            .insert("a", "first")
            .expect("valid context");

        let fields: Vec<(&str, &str)> =
            context.iter().collect();

        assert_eq!(
            fields,
            vec![("a", "first"), ("z", "last")]
        );
    }

    #[test]
    fn context_rejects_secret_like_keys() {
        let mut context = FailureContext::new();

        assert_eq!(
            context.insert("api_key", "redacted"),
            Err(FailureContextError::SensitiveKey)
        );

        assert_eq!(
            context.insert("access_token", "redacted"),
            Err(FailureContextError::SensitiveKey)
        );
    }

    #[test]
    fn provider_code_rejects_secret_like_values() {
        let result = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "network failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure")
        .with_provider_code("access_token");

        assert_eq!(
            result,
            Err(FailureRecordError::SensitiveProviderCode)
        );
    }

    #[test]
    fn identifiers_reject_control_characters() {
        let result = FailureRecord::new(
            FailurePhase::Execution,
            FailureKind::Unknown,
            "failure",
            Idempotency::Idempotent,
        )
        .expect("valid failure")
        .with_backend_id("backend\nid");

        assert_eq!(
            result,
            Err(FailureRecordError::InvalidBackendId)
        );
    }

    #[test]
    fn retry_after_is_bounded() {
        assert_eq!(
            RetryAfter::from_millis(1_000)
                .expect("valid")
                .as_millis(),
            1_000
        );

        assert_eq!(
            RetryAfter::from_millis(MAX_RETRY_DELAY_MS + 1),
            Err(RetryAfterError::TooLarge {
                milliseconds: MAX_RETRY_DELAY_MS + 1,
                maximum: MAX_RETRY_DELAY_MS,
            })
        );
    }

    #[test]
    fn failure_fingerprint_ignores_message() {
        let first = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "message one",
            Idempotency::Idempotent,
        )
        .expect("valid");

        let second = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "message two",
            Idempotency::Idempotent,
        )
        .expect("valid");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn provider_code_changes_fingerprint() {
        let first = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "network failure",
            Idempotency::Idempotent,
        )
        .expect("valid");

        let second = first
            .clone()
            .with_provider_code("E_TEMPORARY")
            .expect("valid");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn failure_chain_is_bounded() {
        let failure = FailureRecord::new(
            FailurePhase::Internal,
            FailureKind::Unknown,
            "failure",
            Idempotency::Unknown,
        )
        .expect("valid");

        let mut chain = FailureChain::new(failure);

        for _ in 1..FailureChain::MAX_DEPTH {
            chain
                .push(
                    FailureRecord::new(
                        FailurePhase::Internal,
                        FailureKind::Unknown,
                        "failure",
                        Idempotency::Unknown,
                    )
                    .expect("valid"),
                )
                .expect("within depth");
        }

        assert_eq!(
            chain.len(),
            FailureChain::MAX_DEPTH
        );

        assert_eq!(
            chain.push(
                FailureRecord::new(
                    FailurePhase::Internal,
                    FailureKind::Unknown,
                    "failure",
                    Idempotency::Unknown,
                )
                .expect("valid"),
            ),
            Err(FailureChainError::TooDeep)
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let first = FailureRecord::new(
            FailurePhase::Transport,
            FailureKind::NetworkFailure,
            "network failure",
            Idempotency::Idempotent,
        )
        .expect("valid");

        let second = FailureRecord::new(
            FailurePhase::Execution,
            FailureKind::HardwareExecutionFailure,
            "hardware failure",
            Idempotency::NonIdempotent,
        )
        .expect("valid");

        let mut statistics = FailureStatistics::new();

        statistics.record(&first);
        statistics.record(&second);

        assert_eq!(statistics.total(), 2);
        assert_eq!(
            statistics.phase_count(FailurePhase::Transport),
            1
        );
        assert_eq!(
            statistics.kind_count(FailureKind::NetworkFailure),
            1
        );
        assert_eq!(
            statistics.kind_count(
                FailureKind::HardwareExecutionFailure
            ),
            1
        );
        assert_eq!(statistics.retryable(), 1);
        assert_eq!(statistics.non_retryable(), 1);
    }

    #[test]
    fn classification_helper_is_conservative() {
        let classification = classify_failure(
            FailureKind::NetworkFailure,
            Idempotency::Idempotent,
        );

        assert_eq!(
            classification.0,
            FailurePermanence::Transient
        );

        assert_eq!(
            classification.1,
            RetrySafety::Safe
        );

        assert_eq!(
            classification.2,
            FailureSeverity::Error
        );
    }

    #[test]
    fn production_policy_is_valid() {
        assert!(
            RetryPolicy::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn disabled_policy_is_valid() {
        assert!(
            RetryPolicy::disabled()
                .validate()
                .is_ok()
        );
    }
}