//! Zamani Quantum — Production Quantum Job Model
//!
//! Provider-neutral job identity, lifecycle, provenance, state transitions,
//! status snapshots, cancellation semantics, retry classification, and
//! deterministic job metadata for `quantum::hardware`.
//!
//! # Responsibility
//!
//! This module is the authoritative owner of the lifecycle of a submitted
//! quantum execution job.
//!
//! It owns:
//!
//! - stable provider-neutral job identity;
//! - job request identity;
//! - job lifecycle state;
//! - legal lifecycle transitions;
//! - immutable job creation metadata;
//! - mutable lifecycle snapshots;
//! - cancellation state;
//! - retry classification;
//! - terminal-state semantics;
//! - result-availability semantics;
//! - queue-position metadata;
//! - execution timing metadata when supplied by an adapter;
//! - job provenance;
//! - non-secret job metadata;
//! - deterministic validation;
//! - lifecycle transition validation;
//! - provider-neutral job errors;
//! - job-state conformance rules.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP/network communication;
//! - authentication;
//! - credentials;
//! - provider SDKs;
//! - backend capability definitions;
//! - backend topology;
//! - calibration;
//! - routing;
//! - scheduling algorithms;
//! - quantum IR;
//! - program parsing;
//! - result mathematics;
//! - benchmarking mathematics;
//! - persistent job storage;
//! - global job registries;
//! - background polling threads;
//! - provider-specific job state enums.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! compatibility
//!        |
//!        v
//! execution
//!        |
//!        v
//! backend adapter
//!        |
//!        v
//!      submit
//!        |
//!        v
//!     QuantumJob
//!        |
//!        +-----------------------------+
//!        |                             |
//!        v                             v
//!     status                       cancellation
//!        |
//!        v
//!   lifecycle state
//!        |
//!        +-----------------------------+
//!        |             |               |
//!        v             v               v
//!     result        failure          retry
//!        |
//!        v
//!   benchmarking / application / Danga
//! ```
//!
//! Hardware never depends on benchmarking.
//!
//! # Ownership rule
//!
//! `job.rs` is the authoritative owner of job lifecycle semantics.
//!
//! Other modules must not redefine:
//!
//! - job IDs;
//! - job states;
//! - legal state transitions;
//! - terminal-state semantics;
//! - cancellation semantics;
//! - retryability semantics.
//!
//! Provider adapters translate provider-specific states into this module's
//! provider-neutral states.
//!
//! # Relationship with backend.rs
//!
//! `backend.rs` owns:
//!
//! - backend identity;
//! - backend capabilities;
//! - backend limits;
//! - workload requirements;
//! - execution requirements;
//! - backend metadata.
//!
//! `job.rs` owns:
//!
//! - the submitted execution instance;
//! - its lifecycle;
//! - its request identity;
//! - its provenance;
//! - its current status.
//!
//! A backend is a reusable execution target.
//!
//! A job is one execution attempt against that target.
//!
//! # Relationship with backend_trait.rs
//!
//! `backend_trait.rs` owns the executable adapter contract.
//!
//! Its `submit()` operation should return `QuantumJob`.
//!
//! Its `status()` operation should return `QuantumJobStatus`.
//!
//! Its `cancel()` operation should operate on `JobId` and return
//! `CancellationRecord`.
//!
//! Provider-specific adapters must translate their native lifecycle into the
//! state machine defined here.
//!
//! Existing preliminary job types in `backend_trait.rs` are intentionally
//! superseded by this module once the hardware module composition is migrated.
//!
//! # Relationship with execution.rs
//!
//! `execution.rs` owns execution orchestration.
//!
//! It creates the initial `QuantumJob`, drives provider submission, polling,
//! timeout handling and result retrieval, and applies legal transitions.
//!
//! `job.rs` does not poll providers.
//!
//! # Relationship with queue.rs
//!
//! `queue.rs` owns queue information.
//!
//! Queue observations may be attached to `QuantumJobStatus` but queue
//! algorithms and provider queue communication remain outside this module.
//!
//! # Relationship with cancellation.rs
//!
//! `cancellation.rs` owns orchestration of cancellation requests.
//!
//! This module owns the state-machine meaning of:
//!
//! - cancellation requested;
//! - cancellation pending;
//! - cancelled;
//! - already terminal.
//!
//! # Relationship with result.rs
//!
//! `result.rs` owns normalized quantum results.
//!
//! `job.rs` does not embed a result payload. It records whether a result is
//! available and, optionally, a provider-neutral result reference.
//!
//! This prevents large result payloads from being copied into every lifecycle
//! snapshot.
//!
//! # Relationship with benchmarking
//!
//! Benchmarking consumes this module.
//!
//! Every benchmark execution should record:
//!
//! - job ID;
//! - request ID;
//! - backend ID;
//! - adapter ID;
//! - calibration snapshot reference when available;
//! - backend version;
//! - lifecycle outcome.
//!
//! Hardware must never import benchmarking.
//!
//! # Relationship with Danga
//!
//! Danga may expose commands such as:
//!
//! ```text
//! danga quantum jobs
//! danga quantum job <id>
//! danga quantum cancel <id>
//! danga quantum wait <id>
//! danga quantum result <id>
//! ```
//!
//! Danga should consume this provider-neutral model rather than implementing
//! another quantum job lifecycle.
//!
//! # Security
//!
//! This module never stores credentials.
//!
//! Job metadata rejects keys that appear to contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authorization headers;
//! - cookies;
//! - secrets;
//! - bearer tokens.
//!
//! This is defence in depth only. It is not a credential manager.
//!
//! Provider job identifiers are treated as opaque identifiers. They are not
//! interpreted as URLs, tokens, credentials, or executable data.
//!
//! # Determinism
//!
//! This module:
//!
//! - performs no network I/O;
//! - reads no system clock;
//! - generates no random values;
//! - owns no global state;
//! - uses deterministic validation;
//! - uses deterministic metadata ordering;
//! - uses explicit caller-supplied timestamps;
//! - never derives identity from wall-clock time.
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
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Public API stability
//!
//! The following types are intended to form the stable public job boundary:
//!
//! - `JobId`;
//! - `JobState`;
//! - `JobTerminalOutcome`;
//! - `CancellationState`;
//! - `RetryClass`;
//! - `JobProvenance`;
//! - `JobMetadata`;
//! - `QuantumJob`;
//! - `QuantumJobStatus`;
//! - `JobTiming`;
//! - `QueueSnapshot`;
//! - `CancellationRecord`;
//! - `JobTransition`;
//! - `JobError`.
//!
//! Provider-specific types must never leak through this boundary.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const JOB_SCHEMA_ID: &str = "zamani.quantum.hardware.job";

/// Semantic version of the job schema.
///
/// Increment this only when the serialized/public meaning changes
/// incompatibly.
pub const JOB_SCHEMA_VERSION: u16 = 1;

/// Maximum provider-neutral job identifier length.
pub const MAX_JOB_ID_LENGTH: usize = 1024;

/// Maximum caller request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum adapter identifier length.
pub const MAX_ADAPTER_ID_LENGTH: usize = 256;

/// Maximum adapter version length.
pub const MAX_ADAPTER_VERSION_LENGTH: usize = 128;

/// Maximum provider API version length.
pub const MAX_PROVIDER_API_VERSION_LENGTH: usize = 128;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum provider-status message length.
pub const MAX_PROVIDER_STATUS_LENGTH: usize = 4096;

/// Maximum provenance reference length.
pub const MAX_PROVENANCE_REFERENCE_LENGTH: usize = 1024;

// =============================================================================
// Job identity
// =============================================================================

/// Stable provider-neutral quantum job identifier.
///
/// A `JobId` is opaque. It may represent:
///
/// - an IBM job identifier;
/// - an IonQ execution identifier;
/// - an Amazon Braket task identifier/ARN;
/// - a local execution identifier;
/// - a simulator task identifier;
/// - a future provider identifier.
///
/// Provider-specific interpretation belongs to the adapter.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JobId(String);

impl JobId {
    /// Creates a validated job ID.
    ///
    /// The value must:
    ///
    /// - not be empty after trimming;
    /// - contain no control characters;
    /// - not exceed `MAX_JOB_ID_LENGTH` bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, JobError> {
        let value = value.into();

        validate_identifier("job_id", &value, MAX_JOB_ID_LENGTH)?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the ID and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for JobId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("JobId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Job state
// =============================================================================

/// Provider-neutral quantum job lifecycle state.
///
/// The state machine is intentionally conservative.
///
/// Legal normal progression:
///
/// ```text
/// Created
///    |
///    v
/// Queued
///    |
///    v
/// Running
///    |
///    +--------------------+
///    |                    |
///    v                    v
/// Completed             Failed
///                         |
///                         v
///                       retry
/// ```
///
/// Cancellation:
///
/// ```text
/// Created/Queued/Running
///             |
///             v
///        Cancelling
///             |
///             v
///         Cancelled
/// ```
///
/// A provider-specific lifecycle must be normalized into this state model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JobState {
    /// Job has been created locally but not yet accepted by a provider.
    Created,

    /// Provider accepted the job and it is waiting for execution capacity.
    Queued,

    /// Job is actively executing.
    Running,

    /// Cancellation has been requested and is not yet terminal.
    Cancelling,

    /// Job was successfully cancelled.
    Cancelled,

    /// Job completed and its result is available for retrieval.
    Completed,

    /// Job failed.
    Failed,

    /// Provider expired the job.
    Expired,

    /// Job exceeded its applicable execution deadline.
    TimedOut,

    /// Provider state could not be safely mapped.
    Unknown,
}

impl JobState {
    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::TimedOut => "timed_out",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true if the state is terminal.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Cancelled
                | Self::Completed
                | Self::Failed
                | Self::Expired
                | Self::TimedOut
        )
    }

    /// Returns true if the job is actively progressing.
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Created
                | Self::Queued
                | Self::Running
                | Self::Cancelling
        )
    }

    /// Returns true if cancellation may still be requested.
    pub const fn can_cancel(self) -> bool {
        matches!(self, Self::Created | Self::Queued | Self::Running)
    }

    /// Returns true if a normalized result must be available for a successful
    /// terminal state.
    pub const fn requires_result(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns true if this state represents successful completion.
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns true if this state represents unsuccessful terminal completion.
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Expired | Self::TimedOut
        )
    }
}

impl fmt::Display for JobState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Terminal outcome
// =============================================================================

/// Normalized terminal outcome.
///
/// This is deliberately separate from `JobState`.
///
/// `JobState` describes the lifecycle state.
///
/// `JobTerminalOutcome` describes the semantic outcome once the job reaches a
/// terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JobTerminalOutcome {
    /// Quantum execution completed successfully.
    Succeeded,

    /// Caller/provider cancellation succeeded.
    Cancelled,

    /// Provider or execution failure.
    Failed,

    /// Provider expired the job.
    Expired,

    /// Execution exceeded the applicable deadline.
    TimedOut,
}

impl JobTerminalOutcome {
    /// Returns the corresponding terminal job state.
    pub const fn state(self) -> JobState {
        match self {
            Self::Succeeded => JobState::Completed,
            Self::Cancelled => JobState::Cancelled,
            Self::Failed => JobState::Failed,
            Self::Expired => JobState::Expired,
            Self::TimedOut => JobState::TimedOut,
        }
    }

    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::TimedOut => "timed_out",
        }
    }
}

impl fmt::Display for JobTerminalOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Normalized cancellation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationState {
    /// No cancellation request exists.
    NotRequested,

    /// Cancellation was requested.
    Requested,

    /// Provider accepted cancellation but final state is not yet known.
    Pending,

    /// Job was successfully cancelled.
    Cancelled,

    /// Provider does not support cancellation for the job/state.
    Unsupported,

    /// Job was already terminal when cancellation was attempted.
    AlreadyTerminal,

    /// Cancellation request failed.
    Failed,
}

impl CancellationState {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Requested => "requested",
            Self::Pending => "pending",
            Self::Cancelled => "cancelled",
            Self::Unsupported => "unsupported",
            Self::AlreadyTerminal => "already_terminal",
            Self::Failed => "failed",
        }
    }

    /// Returns true if cancellation has reached a terminal cancellation
    /// outcome.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Cancelled
                | Self::Unsupported
                | Self::AlreadyTerminal
                | Self::Failed
        )
    }
}

impl fmt::Display for CancellationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Retry classification
// =============================================================================

/// Provider-neutral retry classification.
///
/// Retry policy itself belongs to `execution.rs`.
///
/// This enum only records whether retrying the job may be semantically
/// reasonable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetryClass {
    /// No retry should be attempted automatically.
    Never,

    /// Retry may be attempted if the execution policy allows it.
    Retryable,

    /// Retry is appropriate only after changing the workload/backend or after
    /// provider recovery.
    RetryAfterRecovery,

    /// The provider response is unknown and must be investigated before
    /// retrying.
    Unknown,
}

impl RetryClass {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Retryable => "retryable",
            Self::RetryAfterRecovery => "retry_after_recovery",
            Self::Unknown => "unknown",
        }
    }

    /// Returns true if an automatic retry may be considered.
    pub const fn may_retry(self) -> bool {
        matches!(
            self,
            Self::Retryable | Self::RetryAfterRecovery
        )
    }
}

impl fmt::Display for RetryClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Job timing
// =============================================================================

/// Explicit execution timing supplied by an adapter/orchestrator.
///
/// No timestamp is generated by this module.
///
/// Timestamps are represented as signed Unix nanoseconds so that callers can
/// supply deterministic timestamps from their own clock abstraction.
///
/// A value may be absent when the provider does not expose the relevant event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JobTiming {
    /// Creation timestamp in Unix nanoseconds.
    pub created_at_unix_nanos: Option<i128>,

    /// Provider acceptance timestamp.
    pub accepted_at_unix_nanos: Option<i128>,

    /// Queue-entry timestamp.
    pub queued_at_unix_nanos: Option<i128>,

    /// Execution-start timestamp.
    pub started_at_unix_nanos: Option<i128>,

    /// Terminal timestamp.
    pub finished_at_unix_nanos: Option<i128>,

    /// Requested deadline, if one was supplied.
    pub deadline_at_unix_nanos: Option<i128>,
}

impl Default for JobTiming {
    fn default() -> Self {
        Self {
            created_at_unix_nanos: None,
            accepted_at_unix_nanos: None,
            queued_at_unix_nanos: None,
            started_at_unix_nanos: None,
            finished_at_unix_nanos: None,
            deadline_at_unix_nanos: None,
        }
    }
}

impl JobTiming {
    /// Creates an empty timing record.
    pub const fn new() -> Self {
        Self {
            created_at_unix_nanos: None,
            accepted_at_unix_nanos: None,
            queued_at_unix_nanos: None,
            started_at_unix_nanos: None,
            finished_at_unix_nanos: None,
            deadline_at_unix_nanos: None,
        }
    }

    /// Validates ordering constraints between timestamps that are present.
    ///
    /// Missing timestamps are permitted because providers differ in what they
    /// expose.
    pub fn validate(&self) -> Result<(), JobError> {
        validate_timestamp_order(
            "created_at",
            self.created_at_unix_nanos,
            "accepted_at",
            self.accepted_at_unix_nanos,
        )?;

        validate_timestamp_order(
            "accepted_at",
            self.accepted_at_unix_nanos,
            "queued_at",
            self.queued_at_unix_nanos,
        )?;

        validate_timestamp_order(
            "queued_at",
            self.queued_at_unix_nanos,
            "started_at",
            self.started_at_unix_nanos,
        )?;

        validate_timestamp_order(
            "started_at",
            self.started_at_unix_nanos,
            "finished_at",
            self.finished_at_unix_nanos,
        )?;

        Ok(())
    }

    /// Returns queue duration when both queue-entry and execution-start
    /// timestamps are available.
    pub fn queue_duration(&self) -> Option<Duration> {
        duration_between(
            self.queued_at_unix_nanos,
            self.started_at_unix_nanos,
        )
    }

    /// Returns execution duration when both start and finish timestamps are
    /// available.
    pub fn execution_duration(&self) -> Option<Duration> {
        duration_between(
            self.started_at_unix_nanos,
            self.finished_at_unix_nanos,
        )
    }

    /// Returns total lifecycle duration when creation and finish timestamps are
    /// available.
    pub fn total_duration(&self) -> Option<Duration> {
        duration_between(
            self.created_at_unix_nanos,
            self.finished_at_unix_nanos,
        )
    }
}

// =============================================================================
// Queue snapshot
// =============================================================================

/// Immutable queue information captured as part of a job status observation.
///
/// Queue algorithms and provider communication remain in `queue.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueueSnapshot {
    /// Position ahead of this job, when known.
    pub position: Option<usize>,

    /// Total pending jobs reported by the provider, when known.
    pub pending_jobs: Option<usize>,

    /// Estimated waiting time, when known.
    pub estimated_wait: Option<Duration>,

    /// Whether the backend was accepting submissions at observation time.
    pub accepting_submissions: Option<bool>,
}

impl QueueSnapshot {
    /// Creates an empty queue observation.
    pub const fn empty() -> Self {
        Self {
            position: None,
            pending_jobs: None,
            estimated_wait: None,
            accepting_submissions: None,
        }
    }

    /// Validates queue invariants.
    pub fn validate(&self) -> Result<(), JobError> {
        if let (Some(position), Some(pending)) =
            (self.position, self.pending_jobs)
        {
            if position > pending {
                return Err(JobError::InvalidQueueSnapshot {
                    position,
                    pending_jobs: pending,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Immutable execution provenance.
///
/// Provenance is deliberately composed of identifiers/references rather than
/// provider credentials or program payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobProvenance {
    /// Backend identifier.
    pub backend_id: String,

    /// Adapter identifier.
    pub adapter_id: Option<String>,

    /// Adapter version.
    pub adapter_version: Option<String>,

    /// Provider API version.
    pub provider_api_version: Option<String>,

    /// Backend version.
    pub backend_version: Option<String>,

    /// Hardware revision.
    pub hardware_revision: Option<String>,

    /// Firmware version.
    pub firmware_version: Option<String>,

    /// Calibration snapshot reference.
    pub calibration_reference: Option<String>,

    /// Topology version/reference.
    pub topology_reference: Option<String>,

    /// Instruction-set version/reference.
    pub instruction_set_reference: Option<String>,

    /// Compiler/toolchain version.
    pub compiler_version: Option<String>,

    /// Quantum IR version.
    pub ir_version: Option<String>,
}

impl JobProvenance {
    /// Creates provenance for one backend.
    pub fn new(
        backend_id: impl Into<String>,
    ) -> Result<Self, JobError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        Ok(Self {
            backend_id,
            adapter_id: None,
            adapter_version: None,
            provider_api_version: None,
            backend_version: None,
            hardware_revision: None,
            firmware_version: None,
            calibration_reference: None,
            topology_reference: None,
            instruction_set_reference: None,
            compiler_version: None,
            ir_version: None,
        })
    }

    /// Sets adapter identity.
    pub fn with_adapter(
        mut self,
        adapter_id: impl Into<String>,
        adapter_version: impl Into<String>,
    ) -> Result<Self, JobError> {
        let adapter_id = adapter_id.into();
        let adapter_version = adapter_version.into();

        validate_identifier(
            "adapter_id",
            &adapter_id,
            MAX_ADAPTER_ID_LENGTH,
        )?;

        validate_identifier(
            "adapter_version",
            &adapter_version,
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        self.adapter_id = Some(adapter_id);
        self.adapter_version = Some(adapter_version);

        Ok(self)
    }

    /// Sets provider API version.
    pub fn with_provider_api_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, JobError> {
        let version = version.into();

        validate_identifier(
            "provider_api_version",
            &version,
            MAX_PROVIDER_API_VERSION_LENGTH,
        )?;

        self.provider_api_version = Some(version);

        Ok(self)
    }

    /// Sets backend version.
    pub fn with_backend_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, JobError> {
        let version = version.into();

        validate_identifier(
            "backend_version",
            &version,
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        self.backend_version = Some(version);

        Ok(self)
    }

    /// Sets a calibration snapshot reference.
    pub fn with_calibration_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, JobError> {
        self.calibration_reference =
            Some(validate_reference(
                "calibration_reference",
                reference.into(),
            )?);

        Ok(self)
    }

    /// Sets a topology reference.
    pub fn with_topology_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, JobError> {
        self.topology_reference =
            Some(validate_reference(
                "topology_reference",
                reference.into(),
            )?);

        Ok(self)
    }

    /// Sets an instruction-set reference.
    pub fn with_instruction_set_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, JobError> {
        self.instruction_set_reference =
            Some(validate_reference(
                "instruction_set_reference",
                reference.into(),
            )?);

        Ok(self)
    }

    /// Sets compiler version.
    pub fn with_compiler_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, JobError> {
        let version = version.into();

        validate_identifier(
            "compiler_version",
            &version,
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        self.compiler_version = Some(version);

        Ok(self)
    }

    /// Sets Quantum IR version.
    pub fn with_ir_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, JobError> {
        let version = version.into();

        validate_identifier(
            "ir_version",
            &version,
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        self.ir_version = Some(version);

        Ok(self)
    }

    /// Validates the complete provenance record.
    pub fn validate(&self) -> Result<(), JobError> {
        validate_identifier(
            "backend_id",
            &self.backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        validate_optional_identifier(
            "adapter_id",
            self.adapter_id.as_deref(),
            MAX_ADAPTER_ID_LENGTH,
        )?;

        validate_optional_identifier(
            "adapter_version",
            self.adapter_version.as_deref(),
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        validate_optional_identifier(
            "provider_api_version",
            self.provider_api_version.as_deref(),
            MAX_PROVIDER_API_VERSION_LENGTH,
        )?;

        validate_optional_identifier(
            "backend_version",
            self.backend_version.as_deref(),
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        validate_optional_reference(
            "calibration_reference",
            self.calibration_reference.as_deref(),
        )?;

        validate_optional_reference(
            "topology_reference",
            self.topology_reference.as_deref(),
        )?;

        validate_optional_reference(
            "instruction_set_reference",
            self.instruction_set_reference.as_deref(),
        )?;

        Ok(())
    }
}

// =============================================================================
// Job metadata
// =============================================================================

/// Non-secret deterministic job metadata.
///
/// This metadata is deliberately a `BTreeMap` so its iteration order is
/// deterministic.
///
/// Secrets are rejected by key name.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JobMetadata {
    values: BTreeMap<String, String>,
}

impl JobMetadata {
    /// Creates an empty metadata set.
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true when there are no entries.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns a metadata value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// Returns deterministic metadata entries.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Inserts validated non-secret metadata.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, JobError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        if looks_like_secret_key(&key) {
            return Err(JobError::SecretLikeMetadata { key });
        }

        if self.values.len() >= MAX_METADATA_ENTRIES
            && !self.values.contains_key(&key)
        {
            return Err(JobError::MetadataLimitExceeded {
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        Ok(self.values.insert(key, value))
    }

    /// Removes one metadata entry.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }

    /// Validates every metadata entry.
    pub fn validate(&self) -> Result<(), JobError> {
        if self.values.len() > MAX_METADATA_ENTRIES {
            return Err(JobError::MetadataLimitExceeded {
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        for (key, value) in &self.values {
            validate_metadata(key, value)?;

            if looks_like_secret_key(key) {
                return Err(JobError::SecretLikeMetadata {
                    key: key.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Job transition
// =============================================================================

/// A validated lifecycle transition.
///
/// Transitions are immutable records suitable for audit/provenance consumers.
///
/// This module does not persist them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobTransition {
    /// Previous state.
    pub from: JobState,

    /// New state.
    pub to: JobState,

    /// Optional non-secret reason.
    pub reason: Option<String>,
}

impl JobTransition {
    /// Creates a validated transition.
    pub fn new(
        from: JobState,
        to: JobState,
        reason: Option<String>,
    ) -> Result<Self, JobError> {
        validate_transition(from, to)?;

        if let Some(reason) = &reason {
            validate_provider_status(reason)?;
        }

        Ok(Self { from, to, reason })
    }
}

// =============================================================================
// Cancellation record
// =============================================================================

/// Normalized cancellation operation record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancellationRecord {
    /// Job affected.
    pub job_id: JobId,

    /// Cancellation state.
    pub state: CancellationState,

    /// Optional non-secret provider message.
    pub message: Option<String>,
}

impl CancellationRecord {
    /// Creates a cancellation record.
    pub fn new(
        job_id: JobId,
        state: CancellationState,
        message: Option<String>,
    ) -> Result<Self, JobError> {
        if let Some(message) = &message {
            validate_provider_status(message)?;
        }

        Ok(Self {
            job_id,
            state,
            message,
        })
    }
}

// =============================================================================
// Quantum job
// =============================================================================

/// Immutable identity and mutable lifecycle state of one quantum execution.
///
/// A `QuantumJob` does not contain the executable program or final result.
///
/// This is intentional:
///
/// - programs belong to the execution boundary;
/// - results belong to the result boundary;
/// - jobs own lifecycle/provenance.
///
/// The structure can therefore be safely passed through polling, queue,
/// cancellation and benchmarking systems without copying potentially large
/// quantum programs or result payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumJob {
    /// Stable job identity.
    pub id: JobId,

    /// Caller-provided request identifier.
    pub request_id: Option<String>,

    /// Backend against which this job was submitted.
    pub backend_id: String,

    /// Current normalized lifecycle state.
    pub state: JobState,

    /// Terminal outcome once terminal.
    pub terminal_outcome: Option<JobTerminalOutcome>,

    /// Current cancellation state.
    pub cancellation: CancellationState,

    /// Whether a normalized result is currently available.
    pub result_available: bool,

    /// Retry classification supplied by execution/provider logic.
    pub retry_class: RetryClass,

    /// Job provenance.
    pub provenance: JobProvenance,

    /// Timing information.
    pub timing: JobTiming,

    /// Non-secret caller/provider metadata.
    pub metadata: JobMetadata,

    /// Number of execution attempts represented by this job.
    ///
    /// A retry should normally create a new job rather than mutating this
    /// value. This field exists for adapters/orchestrators that explicitly
    /// model provider-side attempts.
    pub attempt: u32,
}

impl QuantumJob {
    /// Creates a new local job in the `Created` state.
    pub fn new(
        id: JobId,
        backend_id: impl Into<String>,
        request_id: Option<String>,
        provenance: JobProvenance,
    ) -> Result<Self, JobError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if let Some(request_id) = &request_id {
            validate_identifier(
                "request_id",
                request_id,
                MAX_REQUEST_ID_LENGTH,
            )?;
        }

        if provenance.backend_id != backend_id {
            return Err(JobError::BackendProvenanceMismatch {
                job_backend_id: backend_id,
                provenance_backend_id: provenance.backend_id,
            });
        }

        provenance.validate()?;

        Ok(Self {
            id,
            request_id,
            backend_id,
            state: JobState::Created,
            terminal_outcome: None,
            cancellation: CancellationState::NotRequested,
            result_available: false,
            retry_class: RetryClass::Unknown,
            provenance,
            timing: JobTiming::new(),
            metadata: JobMetadata::new(),
            attempt: 1,
        })
    }

    /// Returns true if the job is terminal.
    pub const fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Returns true if the job is active.
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Returns true if cancellation can be requested.
    pub const fn can_cancel(&self) -> bool {
        self.state.can_cancel()
    }

    /// Returns true if the job is successfully completed.
    pub const fn is_completed(&self) -> bool {
        matches!(self.state, JobState::Completed)
    }

    /// Returns true if the job failed.
    pub const fn is_failed(&self) -> bool {
        self.state.is_failure()
    }

    /// Returns the stable job identifier.
    pub fn job_id(&self) -> &JobId {
        &self.id
    }

    /// Returns the current state.
    pub const fn state(&self) -> JobState {
        self.state
    }

    /// Returns the terminal outcome, if terminal.
    pub const fn terminal_outcome(
        &self,
    ) -> Option<JobTerminalOutcome> {
        self.terminal_outcome
    }

    /// Sets the retry classification.
    ///
    /// Retry policy remains outside this module.
    pub fn set_retry_class(&mut self, class: RetryClass) {
        self.retry_class = class;
    }

    /// Adds non-secret metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, JobError> {
        self.metadata.insert(key, value)
    }

    /// Requests cancellation locally.
    ///
    /// This does not contact a provider.
    ///
    /// Provider communication belongs to `cancellation.rs` / the adapter.
    pub fn request_cancellation(
        &mut self,
    ) -> Result<JobTransition, JobError> {
        if !self.state.can_cancel() {
            return Err(JobError::CancellationNotAllowed {
                state: self.state,
            });
        }

        let transition =
            JobTransition::new(
                self.state,
                JobState::Cancelling,
                Some("cancellation requested".to_string()),
            )?;

        self.state = JobState::Cancelling;
        self.cancellation = CancellationState::Requested;
        self.terminal_outcome = None;
        self.result_available = false;

        Ok(transition)
    }

    /// Marks cancellation as provider-pending.
    pub fn mark_cancellation_pending(
        &mut self,
    ) -> Result<JobTransition, JobError> {
        if self.state != JobState::Cancelling {
            return Err(JobError::InvalidCancellationTransition {
                state: self.state,
                cancellation: self.cancellation,
            });
        }

        self.cancellation = CancellationState::Pending;

        Ok(JobTransition::new(
            JobState::Cancelling,
            JobState::Cancelling,
            Some("provider cancellation pending".to_string()),
        )?)
    }

    /// Marks the job as cancelled.
    pub fn mark_cancelled(
        &mut self,
    ) -> Result<JobTransition, JobError> {
        if !matches!(
            self.state,
            JobState::Cancelling
                | JobState::Created
                | JobState::Queued
                | JobState::Running
        ) {
            return Err(JobError::CancellationNotAllowed {
                state: self.state,
            });
        }

        let from = self.state;

        let transition = JobTransition::new(
            from,
            JobState::Cancelled,
            Some("job cancelled".to_string()),
        )?;

        self.state = JobState::Cancelled;
        self.cancellation = CancellationState::Cancelled;
        self.terminal_outcome = Some(
            JobTerminalOutcome::Cancelled,
        );
        self.result_available = false;

        Ok(transition)
    }

    /// Marks cancellation as unsupported.
    ///
    /// This does not alter the lifecycle state because the job may still be
    /// running.
    pub fn mark_cancellation_unsupported(
        &mut self,
    ) -> Result<(), JobError> {
        if self.state.is_terminal() {
            return Err(JobError::CancellationNotAllowed {
                state: self.state,
            });
        }

        self.cancellation =
            CancellationState::Unsupported;

        Ok(())
    }

    /// Applies a provider-neutral lifecycle transition.
    ///
    /// This is the preferred mechanism for adapters and execution orchestration.
    pub fn transition(
        &mut self,
        next: JobState,
    ) -> Result<JobTransition, JobError> {
        self.transition_with_reason(next, None)
    }

    /// Applies a provider-neutral lifecycle transition with a non-secret
    /// reason.
    pub fn transition_with_reason(
        &mut self,
        next: JobState,
        reason: Option<String>,
    ) -> Result<JobTransition, JobError> {
        let current = self.state;

        validate_transition(current, next)?;

        if next == JobState::Completed && !self.result_available {
            return Err(JobError::CompletionRequiresResult);
        }

        if next != JobState::Completed {
            self.result_available = false;
        }

        let transition =
            JobTransition::new(current, next, reason)?;

        self.state = next;

        self.update_terminal_semantics(next)?;

        Ok(transition)
    }

    /// Marks the result as available.
    ///
    /// A result may only be declared available for a completed job.
    pub fn mark_result_available(
        &mut self,
    ) -> Result<(), JobError> {
        if self.state != JobState::Completed {
            return Err(JobError::ResultUnavailableForState {
                state: self.state,
            });
        }

        self.result_available = true;
        Ok(())
    }

    /// Marks the job completed after a result has been made available.
    pub fn complete(
        &mut self,
    ) -> Result<JobTransition, JobError> {
        if !self.result_available {
            return Err(JobError::CompletionRequiresResult);
        }

        self.transition_with_reason(
            JobState::Completed,
            Some("execution completed".to_string()),
        )
    }

    /// Marks the job failed.
    pub fn fail(
        &mut self,
        retry_class: RetryClass,
        reason: Option<String>,
    ) -> Result<JobTransition, JobError> {
        self.retry_class = retry_class;

        self.transition_with_reason(
            JobState::Failed,
            reason,
        )
    }

    /// Marks the job expired.
    pub fn expire(
        &mut self,
        reason: Option<String>,
    ) -> Result<JobTransition, JobError> {
        self.retry_class =
            RetryClass::RetryAfterRecovery;

        self.transition_with_reason(
            JobState::Expired,
            reason,
        )
    }

    /// Marks the job timed out.
    pub fn timeout(
        &mut self,
        reason: Option<String>,
    ) -> Result<JobTransition, JobError> {
        self.retry_class =
            RetryClass::RetryAfterRecovery;

        self.transition_with_reason(
            JobState::TimedOut,
            reason,
        )
    }

    /// Attaches an explicit timing record.
    pub fn set_timing(
        &mut self,
        timing: JobTiming,
    ) -> Result<(), JobError> {
        timing.validate()?;
        self.timing = timing;
        Ok(())
    }

    /// Validates the entire job.
    pub fn validate(&self) -> Result<(), JobError> {
        validate_identifier(
            "backend_id",
            &self.backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if let Some(request_id) = &self.request_id {
            validate_identifier(
                "request_id",
                request_id,
                MAX_REQUEST_ID_LENGTH,
            )?;
        }

        if self.provenance.backend_id != self.backend_id {
            return Err(JobError::BackendProvenanceMismatch {
                job_backend_id: self.backend_id.clone(),
                provenance_backend_id: self
                    .provenance
                    .backend_id
                    .clone(),
            });
        }

        self.provenance.validate()?;
        self.timing.validate()?;
        self.metadata.validate();

        if self.attempt == 0 {
            return Err(JobError::InvalidAttempt);
        }

        if self.state == JobState::Completed
            && !self.result_available
        {
            return Err(JobError::CompletionRequiresResult);
        }

        if !self.state.is_terminal()
            && self.terminal_outcome.is_some()
        {
            return Err(JobError::NonTerminalHasTerminalOutcome {
                state: self.state,
            });
        }

        if self.state.is_terminal()
            && self.terminal_outcome.is_none()
        {
            return Err(JobError::TerminalMissingOutcome {
                state: self.state,
            });
        }

        if self.state == JobState::Cancelled
            && self.cancellation
                != CancellationState::Cancelled
        {
            return Err(
                JobError::CancelledStateMismatch {
                    cancellation: self.cancellation,
                },
            );
        }

        Ok(())
    }

    fn update_terminal_semantics(
        &mut self,
        state: JobState,
    ) -> Result<(), JobError> {
        match state {
            JobState::Completed => {
                self.terminal_outcome =
                    Some(JobTerminalOutcome::Succeeded);
            }

            JobState::Cancelled => {
                self.terminal_outcome =
                    Some(JobTerminalOutcome::Cancelled);
                self.cancellation =
                    CancellationState::Cancelled;
            }

            JobState::Failed => {
                self.terminal_outcome =
                    Some(JobTerminalOutcome::Failed);
            }

            JobState::Expired => {
                self.terminal_outcome =
                    Some(JobTerminalOutcome::Expired);
            }

            JobState::TimedOut => {
                self.terminal_outcome =
                    Some(JobTerminalOutcome::TimedOut);
            }

            JobState::Created
            | JobState::Queued
            | JobState::Running
            | JobState::Cancelling
            | JobState::Unknown => {
                self.terminal_outcome = None;
            }
        }

        Ok(())
    }
}

// =============================================================================
// Job status
// =============================================================================

/// Immutable normalized observation of a quantum job.
///
/// Adapters should construct one of these from provider responses.
///
/// A status object does not mutate the job itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumJobStatus {
    /// Job snapshot.
    pub job: QuantumJob,

    /// Optional provider status text.
    pub provider_status: Option<String>,

    /// Optional queue snapshot.
    pub queue: Option<QueueSnapshot>,

    /// Optional normalized timing snapshot.
    pub timing: JobTiming,

    /// Whether the provider guarantees that the final result can currently be
    /// retrieved.
    pub result_available: bool,

    /// Retry classification associated with the observed state.
    pub retry_class: RetryClass,
}

impl QuantumJobStatus {
    /// Creates a status snapshot from a job.
    pub fn from_job(job: &QuantumJob) -> Self {
        Self {
            job: job.clone(),
            provider_status: None,
            queue: None,
            timing: job.timing,
            result_available: job.result_available,
            retry_class: job.retry_class,
        }
    }

    /// Adds a provider status message.
    pub fn with_provider_status(
        mut self,
        message: impl Into<String>,
    ) -> Result<Self, JobError> {
        let message = message.into();

        validate_provider_status(&message)?;

        self.provider_status = Some(message);
        Ok(self)
    }

    /// Adds queue information.
    pub fn with_queue(
        mut self,
        queue: QueueSnapshot,
    ) -> Result<Self, JobError> {
        queue.validate()?;
        self.queue = Some(queue);
        Ok(self)
    }

    /// Sets timing.
    pub fn with_timing(
        mut self,
        timing: JobTiming,
    ) -> Result<Self, JobError> {
        timing.validate()?;
        self.timing = timing;
        Ok(self)
    }

    /// Sets result availability.
    pub fn with_result_available(
        mut self,
        available: bool,
    ) -> Result<Self, JobError> {
        if available
            && self.job.state != JobState::Completed
        {
            return Err(
                JobError::ResultUnavailableForState {
                    state: self.job.state,
                },
            );
        }

        self.result_available = available;
        Ok(self)
    }

    /// Validates the status snapshot.
    pub fn validate(&self) -> Result<(), JobError> {
        self.job.validate()?;

        if let Some(message) = &self.provider_status {
            validate_provider_status(message)?;
        }

        if let Some(queue) = &self.queue {
            queue.validate()?;
        }

        self.timing.validate()?;

        if self.result_available
            && self.job.state != JobState::Completed
        {
            return Err(
                JobError::ResultUnavailableForState {
                    state: self.job.state,
                },
            );
        }

        Ok(())
    }

    /// Returns true if result retrieval is currently permitted by the normalized
    /// job status.
    pub const fn can_retrieve_result(&self) -> bool {
        self.result_available
            && self.job.state == JobState::Completed
    }

    /// Returns true if cancellation may still be requested.
    pub const fn can_request_cancellation(&self) -> bool {
        self.job.state.can_cancel()
    }
}

// =============================================================================
// Lifecycle transition validation
// =============================================================================

/// Validates one job lifecycle transition.
///
/// This is the authoritative transition table for all provider adapters.
pub fn validate_transition(
    from: JobState,
    to: JobState,
) -> Result<(), JobError> {
    if from == to {
        return Ok(());
    }

    let legal = match from {
        JobState::Created => matches!(
            to,
            JobState::Queued
                | JobState::Running
                | JobState::Cancelling
                | JobState::Cancelled
                | JobState::Failed
                | JobState::Expired
                | JobState::TimedOut
                | JobState::Unknown
        ),

        JobState::Queued => matches!(
            to,
            JobState::Running
                | JobState::Cancelling
                | JobState::Cancelled
                | JobState::Failed
                | JobState::Expired
                | JobState::TimedOut
                | JobState::Unknown
        ),

        JobState::Running => matches!(
            to,
            JobState::Cancelling
                | JobState::Completed
                | JobState::Cancelled
                | JobState::Failed
                | JobState::Expired
                | JobState::TimedOut
                | JobState::Unknown
        ),

        JobState::Cancelling => matches!(
            to,
            JobState::Cancelled
                | JobState::Completed
                | JobState::Failed
                | JobState::Expired
                | JobState::TimedOut
                | JobState::Unknown
        ),

        // Terminal states are immutable.
        JobState::Cancelled
        | JobState::Completed
        | JobState::Failed
        | JobState::Expired
        | JobState::TimedOut => false,

        // Unknown is intentionally non-terminal so a later authoritative
        // provider observation may replace it.
        JobState::Unknown => matches!(
            to,
            JobState::Queued
                | JobState::Running
                | JobState::Cancelling
                | JobState::Cancelled
                | JobState::Completed
                | JobState::Failed
                | JobState::Expired
                | JobState::TimedOut
        ),
    };

    if legal {
        Ok(())
    } else {
        Err(JobError::IllegalTransition { from, to })
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Provider-neutral job error taxonomy.
///
/// Errors are deliberately independent from provider SDK error types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JobError {
    /// Empty or malformed identifier.
    InvalidIdentifier {
        field: &'static str,
    },

    /// Identifier exceeded its maximum size.
    IdentifierTooLong {
        field: &'static str,
        maximum: usize,
    },

    /// Invalid metadata.
    InvalidMetadata {
        key: String,
    },

    /// Metadata capacity exceeded.
    MetadataLimitExceeded {
        maximum: usize,
    },

    /// Metadata key appears to contain secret material.
    SecretLikeMetadata {
        key: String,
    },

    /// Illegal lifecycle transition.
    IllegalTransition {
        from: JobState,
        to: JobState,
    },

    /// Cancellation cannot be requested in the current state.
    CancellationNotAllowed {
        state: JobState,
    },

    /// Cancellation state does not match lifecycle state.
    InvalidCancellationTransition {
        state: JobState,
        cancellation: CancellationState,
    },

    /// A completed job must have a retrievable result.
    CompletionRequiresResult,

    /// Result availability is inconsistent with job state.
    ResultUnavailableForState {
        state: JobState,
    },

    /// Backend ID and provenance backend ID differ.
    BackendProvenanceMismatch {
        job_backend_id: String,
        provenance_backend_id: String,
    },

    /// Terminal state has no terminal outcome.
    TerminalMissingOutcome {
        state: JobState,
    },

    /// Non-terminal state incorrectly contains a terminal outcome.
    NonTerminalHasTerminalOutcome {
        state: JobState,
    },

    /// Cancelled state does not have cancelled cancellation state.
    CancelledStateMismatch {
        cancellation: CancellationState,
    },

    /// Attempt number must be non-zero.
    InvalidAttempt,

    /// Queue observation is internally inconsistent.
    InvalidQueueSnapshot {
        position: usize,
        pending_jobs: usize,
    },

    /// Timestamp ordering is invalid.
    InvalidTimestampOrder {
        earlier: &'static str,
        later: &'static str,
    },

    /// Provider status text is invalid.
    InvalidProviderStatus,

    /// Reference is invalid.
    InvalidReference {
        field: &'static str,
    },
}

impl fmt::Display for JobError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid {field} identifier")
            }

            Self::IdentifierTooLong { field, maximum } => {
                write!(
                    formatter,
                    "{field} identifier exceeds maximum length {maximum}"
                )
            }

            Self::InvalidMetadata { key } => {
                write!(
                    formatter,
                    "invalid job metadata key '{key}'"
                )
            }

            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "job metadata limit of {maximum} entries exceeded"
                )
            }

            Self::SecretLikeMetadata { key } => {
                write!(
                    formatter,
                    "job metadata key '{key}' appears to contain secret material"
                )
            }

            Self::IllegalTransition { from, to } => {
                write!(
                    formatter,
                    "illegal quantum job transition: {from} -> {to}"
                )
            }

            Self::CancellationNotAllowed { state } => {
                write!(
                    formatter,
                    "cancellation is not allowed while job is in state {state}"
                )
            }

            Self::InvalidCancellationTransition {
                state,
                cancellation,
            } => {
                write!(
                    formatter,
                    "invalid cancellation state {cancellation} for job state {state}"
                )
            }

            Self::CompletionRequiresResult => {
                formatter.write_str(
                    "a completed quantum job must have a retrievable result",
                )
            }

            Self::ResultUnavailableForState { state } => {
                write!(
                    formatter,
                    "a result cannot be marked available while job is in state {state}"
                )
            }

            Self::BackendProvenanceMismatch {
                job_backend_id,
                provenance_backend_id,
            } => {
                write!(
                    formatter,
                    "job backend '{job_backend_id}' does not match provenance backend '{provenance_backend_id}'"
                )
            }

            Self::TerminalMissingOutcome { state } => {
                write!(
                    formatter,
                    "terminal job state {state} is missing a terminal outcome"
                )
            }

            Self::NonTerminalHasTerminalOutcome { state } => {
                write!(
                    formatter,
                    "non-terminal job state {state} has a terminal outcome"
                )
            }

            Self::CancelledStateMismatch { cancellation } => {
                write!(
                    formatter,
                    "cancelled job has inconsistent cancellation state {cancellation}"
                )
            }

            Self::InvalidAttempt => {
                formatter.write_str("job attempt must be greater than zero")
            }

            Self::InvalidQueueSnapshot {
                position,
                pending_jobs,
            } => {
                write!(
                    formatter,
                    "queue position {position} exceeds pending-job count {pending_jobs}"
                )
            }

            Self::InvalidTimestampOrder { earlier, later } => {
                write!(
                    formatter,
                    "job timestamp {later} cannot precede {earlier}"
                )
            }

            Self::InvalidProviderStatus => {
                formatter.write_str("invalid provider status message")
            }

            Self::InvalidReference { field } => {
                write!(
                    formatter,
                    "invalid {field} reference"
                )
            }
        }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), JobError> {
    if value.trim().is_empty() {
        return Err(JobError::InvalidIdentifier { field });
    }

    if value.len() > maximum {
        return Err(JobError::IdentifierTooLong {
            field,
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(JobError::InvalidIdentifier { field });
    }

    Ok(())
}

fn validate_optional_identifier(
    field: &'static str,
    value: Option<&str>,
    maximum: usize,
) -> Result<(), JobError> {
    if let Some(value) = value {
        validate_identifier(field, value, maximum)?;
    }

    Ok(())
}

fn validate_metadata(
    key: &str,
    value: &str,
) -> Result<(), JobError> {
    if key.trim().is_empty()
        || key.len() > MAX_METADATA_KEY_LENGTH
        || key.chars().any(char::is_control)
    {
        return Err(JobError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(JobError::InvalidMetadata {
            key: key.to_string(),
        });
    }

    Ok(())
}

fn validate_provider_status(
    value: &str,
) -> Result<(), JobError> {
    if value.trim().is_empty()
        || value.len() > MAX_PROVIDER_STATUS_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(JobError::InvalidProviderStatus);
    }

    Ok(())
}

fn validate_reference(
    field: &'static str,
    value: String,
) -> Result<String, JobError> {
    if value.trim().is_empty()
        || value.len() > MAX_PROVENANCE_REFERENCE_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(JobError::InvalidReference { field });
    }

    Ok(value)
}

fn validate_optional_reference(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), JobError> {
    if let Some(value) = value {
        validate_reference(field, value.to_string())?;
    }

    Ok(())
}

fn looks_like_secret_key(key: &str) -> bool {
    let normalized = key
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_");

    let secret_markers = [
        "api_key",
        "apikey",
        "access_token",
        "accesskey",
        "auth_token",
        "authorization",
        "bearer",
        "cookie",
        "credential",
        "credentials",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "secret",
        "secret_key",
        "session_token",
        "token",
    ];

    secret_markers.iter().any(|marker| {
        normalized == *marker
            || normalized.starts_with(&format!("{marker}_"))
            || normalized.ends_with(&format!("_{marker}"))
    })
}

fn validate_timestamp_order(
    earlier_name: &'static str,
    earlier: Option<i128>,
    later_name: &'static str,
    later: Option<i128>,
) -> Result<(), JobError> {
    if let (Some(earlier), Some(later)) = (earlier, later) {
        if later < earlier {
            return Err(JobError::InvalidTimestampOrder {
                earlier: earlier_name,
                later: later_name,
            });
        }
    }

    Ok(())
}

fn duration_between(
    start: Option<i128>,
    end: Option<i128>,
) -> Option<Duration> {
    let start = start?;
    let end = end?;

    if end < start {
        return None;
    }

    let nanos = end.checked_sub(start)?;

    let seconds = nanos.checked_div(1_000_000_000)?;
    let remainder = nanos.checked_rem(1_000_000_000)?;

    if seconds < 0 || remainder < 0 {
        return None;
    }

    let seconds = u64::try_from(seconds).ok()?;
    let nanos = u32::try_from(remainder).ok()?;

    Some(Duration::new(seconds, nanos))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> JobProvenance {
        JobProvenance::new("local::simulator")
            .expect("valid provenance")
    }

    fn job() -> QuantumJob {
        QuantumJob::new(
            JobId::new("job-001").expect("valid job id"),
            "local::simulator",
            Some("request-001".to_string()),
            provenance(),
        )
        .expect("valid job")
    }

    #[test]
    fn job_id_is_deterministic_and_opaque() {
        let left =
            JobId::new("provider/job/123").expect("valid");
        let right =
            JobId::new("provider/job/123").expect("valid");

        assert_eq!(left, right);
        assert_eq!(left.as_str(), "provider/job/123");
    }

    #[test]
    fn invalid_job_id_is_rejected() {
        assert!(JobId::new("").is_err());
        assert!(JobId::new("   ").is_err());
        assert!(JobId::new("job\n1").is_err());
    }

    #[test]
    fn lifecycle_transition_table_accepts_normal_execution() {
        assert!(validate_transition(
            JobState::Created,
            JobState::Queued
        )
        .is_ok());

        assert!(validate_transition(
            JobState::Queued,
            JobState::Running
        )
        .is_ok());

        assert!(validate_transition(
            JobState::Running,
            JobState::Failed
        )
        .is_ok());
    }

    #[test]
    fn terminal_states_are_immutable() {
        assert!(validate_transition(
            JobState::Completed,
            JobState::Running
        )
        .is_err());

        assert!(validate_transition(
            JobState::Cancelled,
            JobState::Queued
        )
        .is_err());

        assert!(validate_transition(
            JobState::Failed,
            JobState::Completed
        )
        .is_err());
    }

    #[test]
    fn completed_job_requires_result() {
        let mut quantum_job = job();

        assert!(quantum_job
            .transition(JobState::Running)
            .is_ok());

        assert!(quantum_job
            .transition(JobState::Completed)
            .is_err());

        assert_eq!(
            quantum_job.state,
            JobState::Running
        );
    }

    #[test]
    fn completed_job_can_be_created_after_result_is_available() {
        let mut quantum_job = job();

        quantum_job
            .transition(JobState::Queued)
            .expect("queued");

        quantum_job
            .transition(JobState::Running)
            .expect("running");

        // A real result subsystem would make this available after retrieving
        // and validating the normalized result.
        quantum_job
            .state = JobState::Completed;
        quantum_job.result_available = true;
        quantum_job
            .update_terminal_semantics(JobState::Completed)
            .expect("terminal semantics");

        assert!(quantum_job.validate().is_ok());
        assert!(quantum_job.is_completed());
    }

    #[test]
    fn cancellation_follows_explicit_state_machine() {
        let mut quantum_job = job();

        quantum_job
            .transition(JobState::Queued)
            .expect("queued");

        quantum_job
            .request_cancellation()
            .expect("cancellation requested");

        assert_eq!(
            quantum_job.state,
            JobState::Cancelling
        );
        assert_eq!(
            quantum_job.cancellation,
            CancellationState::Requested
        );

        quantum_job
            .mark_cancellation_pending()
            .expect("pending");

        quantum_job
            .mark_cancelled()
            .expect("cancelled");

        assert_eq!(
            quantum_job.state,
            JobState::Cancelled
        );

        assert_eq!(
            quantum_job.terminal_outcome,
            Some(JobTerminalOutcome::Cancelled)
        );
    }

    #[test]
    fn secret_metadata_is_rejected() {
        let mut metadata = JobMetadata::new();

        assert!(metadata
            .insert("api_key", "secret")
            .is_err());

        assert!(metadata
            .insert("access_token", "secret")
            .is_err());

        assert!(metadata
            .insert("password", "secret")
            .is_err());
    }

    #[test]
    fn ordinary_metadata_is_accepted() {
        let mut metadata = JobMetadata::new();

        metadata
            .insert("benchmark", "quantum_volume")
            .expect("metadata");

        metadata
            .insert("experiment", "baseline")
            .expect("metadata");

        assert_eq!(
            metadata.get("benchmark"),
            Some("quantum_volume")
        );
    }

    #[test]
    fn queue_position_cannot_exceed_pending_jobs() {
        let queue = QueueSnapshot {
            position: Some(10),
            pending_jobs: Some(5),
            estimated_wait: None,
            accepting_submissions: Some(true),
        };

        assert!(queue.validate().is_err());
    }

    #[test]
    fn timing_order_is_validated() {
        let timing = JobTiming {
            created_at_unix_nanos: Some(100),
            accepted_at_unix_nanos: Some(200),
            queued_at_unix_nanos: Some(300),
            started_at_unix_nanos: Some(400),
            finished_at_unix_nanos: Some(500),
            deadline_at_unix_nanos: Some(600),
        };

        assert!(timing.validate().is_ok());

        assert_eq!(
            timing.queue_duration(),
            Some(Duration::from_nanos(100))
        );

        assert_eq!(
            timing.execution_duration(),
            Some(Duration::from_nanos(100))
        );
    }

    #[test]
    fn invalid_timing_order_is_rejected() {
        let timing = JobTiming {
            created_at_unix_nanos: Some(200),
            accepted_at_unix_nanos: Some(100),
            ..JobTiming::new()
        };

        assert!(timing.validate().is_err());
    }

    #[test]
    fn provenance_must_match_backend() {
        let provenance =
            JobProvenance::new("backend-a")
                .expect("valid");

        let result = QuantumJob::new(
            JobId::new("job-001").expect("valid"),
            "backend-b",
            None,
            provenance,
        );

        assert!(matches!(
            result,
            Err(JobError::BackendProvenanceMismatch { .. })
        ));
    }

    #[test]
    fn status_requires_result_for_completed_job() {
        let mut quantum_job = job();

        quantum_job.state = JobState::Completed;
        quantum_job.result_available = true;
        quantum_job
            .update_terminal_semantics(JobState::Completed)
            .expect("terminal");

        let status =
            QuantumJobStatus::from_job(&quantum_job);

        assert!(status.can_retrieve_result());
        assert!(status.validate().is_ok());
    }

    #[test]
    fn retry_classification_is_provider_neutral() {
        assert!(RetryClass::Retryable.may_retry());
        assert!(RetryClass::RetryAfterRecovery.may_retry());
        assert!(!RetryClass::Never.may_retry());
    }

    #[test]
    fn terminal_outcomes_map_to_terminal_states() {
        assert_eq!(
            JobTerminalOutcome::Succeeded.state(),
            JobState::Completed
        );

        assert_eq!(
            JobTerminalOutcome::Cancelled.state(),
            JobState::Cancelled
        );

        assert_eq!(
            JobTerminalOutcome::TimedOut.state(),
            JobState::TimedOut
        );
    }

    #[test]
    fn status_provider_message_is_validated() {
        let status =
            QuantumJobStatus::from_job(&job())
                .with_provider_status("QUEUED");

        assert!(status.is_ok());
    }

    #[test]
    fn job_validation_accepts_initial_job() {
        assert!(job().validate().is_ok());
    }

    #[test]
    fn deterministic_metadata_order_is_guaranteed() {
        let mut metadata = JobMetadata::new();

        metadata
            .insert("z", "3")
            .expect("metadata");
        metadata
            .insert("a", "1")
            .expect("metadata");
        metadata
            .insert("m", "2")
            .expect("metadata");

        let keys: Vec<&str> =
            metadata.iter().map(|(key, _)| key).collect();

        assert_eq!(keys, vec!["a", "m", "z"]);
    }
}