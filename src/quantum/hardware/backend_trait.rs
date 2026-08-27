//! Zamani Quantum — Provider-Neutral Backend Execution Contract
//!
//! Production hardware adapter boundary for `quantum::hardware`.
//!
//! # Responsibility
//!
//! This module defines the stable, provider-independent contract that a real
//! quantum backend adapter must implement.
//!
//! `backend.rs` describes WHAT a backend is.
//! `backend_trait.rs` describes HOW an adapter executes against that backend.
//!
//! The contract covers:
//!
//! - immutable backend descriptor access;
//! - adapter identity/versioning;
//! - provider-neutral program payloads;
//! - preflight validation;
//! - asynchronous submission;
//! - provider-neutral job identity;
//! - job lifecycle;
//! - status polling;
//! - normalized result retrieval;
//! - cancellation;
//! - queue information;
//! - health information;
//! - synchronous execution where natively supported;
//! - object-safe provider adapters;
//! - local/simulator/emulator adapters;
//! - remote QPU adapters;
//! - provider-independent conformance.
//!
//! It deliberately does NOT own:
//!
//! - provider HTTP/network clients;
//! - authentication;
//! - credentials;
//! - provider SDK types;
//! - OpenQASM parsing;
//! - QIR generation;
//! - transpilation;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration storage;
//! - benchmarking mathematics;
//! - job persistence;
//! - provider registries;
//! - global state.
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
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! optimization                 error correction
//!      |                             |
//!      +--------------+--------------+
//!                     |
//!                     v
//!             compatibility
//!                     |
//!             +-------+-------+
//!             |               |
//!             v               v
//!          routing        scheduling
//!             |               |
//!             +-------+-------+
//!                     |
//!                     v
//!              BackendProgram
//!                     |
//!                     v
//!          QuantumBackendAdapter
//!                     |
//!        +------------+-------------+
//!        |            |             |
//!        v            v             v
//!      local        simulator     provider
//!                                  adapters
//!                                    |
//!                       +------------+-------------+
//!                       |            |             |
//!                      IBM         IonQ         Braket...
//!                                                    |
//!                                                    v
//!                                                   QPU
//! ```
//!
//! Benchmarking consumes this boundary.
//!
//! Hardware never depends on benchmarking.
//!
//! # Why this file exists
//!
//! `backend.rs` already owns the concrete `QuantumBackend` aggregate. The
//! execution contract must therefore use a different name.
//!
//! The stable abstraction is:
//!
//! ```text
//! QuantumBackend
//!        = backend description
//!
//! QuantumBackendAdapter
//!        = executable provider adapter
//! ```
//!
//! This separation prevents provider-specific execution code from leaking
//! into the canonical backend model.
//!
//! # Object safety
//!
//! `QuantumBackendAdapter` is intentionally object-safe.
//!
//! Registries can therefore store:
//!
//! ```text
//! Box<dyn QuantumBackendAdapter>
//! Arc<dyn QuantumBackendAdapter>
//! ```
//!
//! without knowing the provider at compile time.
//!
//! # Execution semantics
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
//! An adapter MUST NOT report `Completed` unless the normalized result is
//! retrievable.
//!
//! An adapter MUST NOT submit a workload that failed preflight.
//!
//! A provider-specific failure MUST be translated into `BackendError`.
//!
//! # Program boundary
//!
//! `BackendProgram` is intentionally opaque.
//!
//! It can represent:
//!
//! - Zamani Quantum IR;
//! - OpenQASM;
//! - QIR;
//! - pulse programs;
//! - analog programs;
//! - annealing programs;
//! - logical programs;
//! - provider-native programs.
//!
//! This prevents the hardware trait from depending on future format-specific
//! modules.
//!
//! # Security
//!
//! This module never stores credentials.
//!
//! Program payloads and identifiers MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - secret material.
//!
//! Provider authentication belongs in `credentials.rs` and
//! `authentication.rs`.
//!
//! `BackendProgram` deliberately does not expose its bytes through `Debug`.
//!
//! # Determinism
//!
//! This module:
//!
//! - performs no network I/O;
//! - reads no system clock;
//! - generates no random values;
//! - owns no global mutable state;
//! - uses deterministic identifiers;
//! - uses deterministic lifecycle semantics.
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
//! This file is independently complete once `backend.rs` exists.
//!
//! Future modules consume it as follows:
//!
//! `execution.rs`
//!     Converts canonical workloads into `BackendProgram` and drives
//!     submission/status/result.
//!
//! `job.rs`
//!     May wrap or re-export `BackendJobId`, `BackendJobState`, and
//!     `BackendJobStatus`.
//!
//! `queue.rs`
//!     Consumes `BackendQueueInfo`.
//!
//! `health.rs`
//!     Consumes `BackendHealth`.
//!
//! `provider.rs`
//!     Owns provider-level grouping but does not redefine this trait.
//!
//! `provider_registry.rs`
//!     Stores `dyn QuantumBackendAdapter`.
//!
//! `device_registry.rs`
//!     Indexes adapters by backend/device identity.
//!
//! `adapters/*`
//!     Implement `QuantumBackendAdapter`.
//!
//! `benchmarking`
//!     Uses the same lifecycle and records backend/job provenance.
//!
//! Danga
//!     Uses this contract rather than implementing a second quantum execution
//!     API.
//!
//! Adding a provider MUST NOT require changing this file.
//!
//! # Stability rule
//!
//! Once this file is accepted, provider implementations must adapt to this
//! contract rather than forcing changes into it for provider-specific behavior.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::time::Duration;

use super::backend::{
    BackendError,
    BackendStatus,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

/// Stable schema identifier for this execution contract.
pub const BACKEND_TRAIT_SCHEMA_ID: &str =
    "zamani.quantum.hardware.backend_trait";

/// Semantic version of this contract.
pub const BACKEND_TRAIT_SCHEMA_VERSION: u16 = 1;

/// Maximum encoded provider-neutral program size.
///
/// Larger workloads should eventually use an artifact/streaming boundary
/// rather than forcing the entire payload through memory.
pub const MAX_PROGRAM_PAYLOAD_BYTES: usize = 256 * 1024 * 1024;

/// Maximum program-format identifier length.
pub const MAX_PROGRAM_FORMAT_LENGTH: usize = 128;

/// Maximum provider job identifier length.
pub const MAX_JOB_ID_LENGTH: usize = 1024;

/// Maximum request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

/// Maximum adapter identifier length.
pub const MAX_ADAPTER_ID_LENGTH: usize = 256;

/// Maximum adapter version length.
pub const MAX_ADAPTER_VERSION_LENGTH: usize = 128;

/// Maximum provider API version length.
pub const MAX_PROVIDER_API_VERSION_LENGTH: usize = 128;

/// =============================================================================
/// Program payload
/// =============================================================================

/// Immutable provider-neutral quantum program payload.
///
/// The payload format is deliberately represented as a stable string instead
/// of an enum so new formats can be introduced without modifying this core
/// execution contract.
///
/// Examples:
///
/// ```text
/// zamani-ir
/// openqasm-3.1
/// qir
/// pulse
/// analog
/// annealing
/// logical
/// provider-native
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct BackendProgram {
    format: String,
    bytes: Vec<u8>,
}

impl BackendProgram {
    /// Creates a validated immutable program payload.
    pub fn new(
        format: impl Into<String>,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self, BackendError> {
        let format = format.into();
        let bytes = bytes.into();

        validate_format(&format)?;

        if bytes.is_empty() {
            return Err(BackendError::ExecutionUnavailable);
        }

        if bytes.len() > MAX_PROGRAM_PAYLOAD_BYTES {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_PROGRAM_PAYLOAD_BYTES,
            });
        }

        Ok(Self { format, bytes })
    }

    /// Returns the program format.
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns an immutable view of the encoded program.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the encoded payload size.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl fmt::Debug for BackendProgram {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackendProgram")
            .field("format", &self.format)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

/// =============================================================================
/// Job identity
/// =============================================================================

/// Stable provider-neutral quantum job identifier.
///
/// The identifier may internally correspond to:
///
/// - IBM job ID;
/// - IonQ execution ID;
/// - Amazon Braket quantum-task ARN;
/// - local execution ID;
/// - simulator task ID;
/// - another provider identifier.
///
/// Provider-specific meaning MUST remain inside the adapter.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendJobId(String);

impl BackendJobId {
    /// Creates a validated job identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, BackendError> {
        let value = value.into();

        validate_identifier(
            "job_id",
            &value,
            MAX_JOB_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical job identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for BackendJobId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("BackendJobId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for BackendJobId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// =============================================================================
/// Job lifecycle
/// =============================================================================

/// Provider-neutral quantum job state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendJobState {
    /// Accepted by the adapter/provider but not yet queued.
    Created,

    /// Waiting for execution capacity.
    Queued,

    /// Currently executing.
    Running,

    /// Cancellation has been requested.
    Cancelling,

    /// Successfully cancelled.
    Cancelled,

    /// Completed and result is available.
    Completed,

    /// Execution failed.
    Failed,

    /// Provider expired the job.
    Expired,

    /// Execution exceeded the applicable deadline.
    TimedOut,

    /// Provider returned an unknown/unmappable state.
    Unknown,
}

impl BackendJobState {
    /// Returns whether this is a terminal state.
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

    /// Returns whether the job is still active.
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Created
                | Self::Queued
                | Self::Running
                | Self::Cancelling
        )
    }

    /// Stable machine-readable state.
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
}

impl fmt::Display for BackendJobState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Provider-neutral submitted-job handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendJob {
    /// Stable job identifier.
    pub id: BackendJobId,

    /// Backend used for submission.
    pub backend_id: String,

    /// Optional caller request identifier.
    pub request_id: Option<String>,

    /// Initial lifecycle state.
    pub state: BackendJobState,
}

impl BackendJob {
    /// Creates a validated job handle.
    pub fn new(
        id: BackendJobId,
        backend_id: impl Into<String>,
        request_id: Option<String>,
        state: BackendJobState,
    ) -> Result<Self, BackendError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            512,
        )?;

        if let Some(request_id) = &request_id {
            validate_identifier(
                "request_id",
                request_id,
                MAX_REQUEST_ID_LENGTH,
            )?;
        }

        Ok(Self {
            id,
            backend_id,
            request_id,
            state,
        })
    }
}

/// Detailed normalized job status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendJobStatus {
    /// Job identity and lifecycle state.
    pub job: BackendJob,

    /// Provider status text, if available.
    ///
    /// This must never contain secrets.
    pub provider_status: Option<String>,

    /// Queue position, when known.
    pub queue_position: Option<usize>,

    /// Estimated waiting time, when known.
    pub estimated_wait: Option<Duration>,

    /// Whether a normalized result can currently be retrieved.
    pub result_available: bool,
}

impl BackendJobStatus {
    /// Returns true when a result may safely be requested.
    pub const fn can_retrieve_result(&self) -> bool {
        self.result_available
            && matches!(self.job.state, BackendJobState::Completed)
    }

    /// Returns true when cancellation may still be meaningful.
    pub const fn can_request_cancellation(&self) -> bool {
        matches!(
            self.job.state,
            BackendJobState::Created
                | BackendJobState::Queued
                | BackendJobState::Running
        )
    }
}

/// =============================================================================
/// Queue
/// =============================================================================

/// Normalized provider queue information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendQueueInfo {
    /// Number of jobs currently ahead of a new submission.
    pub pending_jobs: Option<usize>,

    /// Estimated wait before execution.
    pub estimated_wait: Option<Duration>,

    /// Whether new submissions are currently accepted.
    pub accepting_submissions: bool,
}

/// =============================================================================
/// Health
/// =============================================================================

/// Provider-neutral health state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BackendHealthState {
    /// No authoritative health information is currently available.
    Unknown,

    /// Backend is healthy.
    Healthy,

    /// Backend is operational but degraded.
    Degraded,

    /// Backend is known to be unhealthy.
    Unhealthy,

    /// Backend could not be reached.
    Unreachable,
}

impl BackendHealthState {
    /// Returns whether execution may be attempted based on health alone.
    ///
    /// This is deliberately conservative: `Unknown` is not considered safe.
    pub const fn permits_execution(self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Stable machine-readable state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unreachable => "unreachable",
        }
    }
}

/// Normalized backend health report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendHealth {
    /// Normalized health state.
    pub state: BackendHealthState,

    /// Backend operational status.
    pub backend_status: BackendStatus,

    /// Optional non-secret diagnostic.
    pub message: Option<String>,
}

/// =============================================================================
/// Cancellation
/// =============================================================================

/// Normalized cancellation outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CancellationOutcome {
    /// Provider accepted cancellation.
    Accepted,

    /// Cancellation is pending.
    Pending,

    /// Provider/backend does not support cancellation for this job/state.
    Unsupported,

    /// Job was already terminal.
    AlreadyTerminal,
}

/// Result of a cancellation request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCancellation {
    /// Job affected.
    pub job: BackendJobId,

    /// Normalized outcome.
    pub outcome: CancellationOutcome,
}

/// =============================================================================
/// Adapter metadata
/// =============================================================================

/// Immutable adapter identity and version information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendAdapterInfo {
    /// Stable adapter identifier.
    ///
    /// Examples:
    ///
    /// ```text
    /// local
    /// ibm
    /// ionq
    /// aws-braket
    /// iqm
    /// rigetti
    /// quantinuum
    /// quera
    /// ```
    pub adapter_id: String,

    /// Adapter semantic version.
    pub adapter_version: String,

    /// Provider API version, when known.
    pub provider_api_version: Option<String>,

    /// Whether this adapter declares itself production-ready.
    pub production_ready: bool,
}

impl BackendAdapterInfo {
    /// Constructs validated adapter metadata.
    pub fn new(
        adapter_id: impl Into<String>,
        adapter_version: impl Into<String>,
        production_ready: bool,
    ) -> Result<Self, BackendError> {
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

        Ok(Self {
            adapter_id,
            adapter_version,
            provider_api_version: None,
            production_ready,
        })
    }

    /// Adds a provider API version.
    pub fn with_provider_api_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let version = version.into();

        validate_identifier(
            "provider_api_version",
            &version,
            MAX_PROVIDER_API_VERSION_LENGTH,
        )?;

        self.provider_api_version = Some(version);
        Ok(self)
    }
}

/// =============================================================================
/// Canonical adapter trait
/// =============================================================================

/// Stable provider-neutral quantum backend adapter.
///
/// This is the central execution contract for all real hardware adapters.
///
/// Implementations must be:
///
/// - `Send`;
/// - `Sync`;
/// - object-safe;
/// - provider-independent at the public boundary.
///
/// Provider-specific networking, authentication, SDKs and serialization must
/// remain behind the implementation.
pub trait QuantumBackendAdapter: Send + Sync {
    /// Returns the immutable canonical backend descriptor.
    ///
    /// The descriptor comes from `backend.rs`.
    ///
    /// The adapter must not mutate the descriptor concurrently.
    fn backend(&self) -> &QuantumBackend;

    /// Returns immutable adapter metadata.
    fn adapter_info(&self) -> &BackendAdapterInfo;

    /// Performs side-effect-free validation before submission.
    ///
    /// This default implementation validates request structure and payload
    /// integrity. Provider adapters should extend it with provider-specific
    /// capability checks.
    fn preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), BackendError> {
        request.validate_structure()?;

        if program.is_empty() {
            return Err(BackendError::ExecutionUnavailable);
        }

        Ok(())
    }

    /// Submits a quantum workload.
    ///
    /// Implementations MUST:
    ///
    /// 1. call/perform the equivalent of `preflight`;
    /// 2. reject invalid workloads before provider submission;
    /// 3. never manufacture a job ID after provider rejection;
    /// 4. normalize the provider job identity;
    /// 5. return the initial lifecycle state.
    ///
    /// The method must not return the final result for an asynchronous
    /// provider.
    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError>;

    /// Retrieves normalized lifecycle state for a submitted job.
    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError>;

    /// Retrieves the completed normalized result.
    ///
    /// Implementations MUST NOT return `Ok(ExecutionResult)` while the job is
    /// still running or while its result is not complete.
    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError>;

    /// Requests provider-side cancellation.
    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError>;

    /// Returns queue information if supported.
    ///
    /// The default is conservative and reports unavailable rather than
    /// inventing queue information.
    fn queue_info(&self) -> Result<BackendQueueInfo, BackendError> {
        Err(BackendError::ExecutionUnavailable)
    }

    /// Performs a provider/backend health check.
    ///
    /// The default is deliberately `Unknown`, not `Healthy`.
    fn health(&self) -> Result<BackendHealth, BackendError> {
        Ok(BackendHealth {
            state: BackendHealthState::Unknown,
            backend_status: BackendStatus::Unknown,
            message: None,
        })
    }

    /// Performs native synchronous execution when the adapter supports it.
    ///
    /// The default implementation performs preflight and then refuses to
    /// pretend that asynchronous execution is synchronous.
    ///
    /// Remote adapters should normally implement:
    ///
    /// ```text
    /// submit -> poll -> result
    /// ```
    ///
    /// rather than implementing blocking behavior here.
    fn execute(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ExecutionResult, BackendError> {
        self.preflight(request, program)?;
        Err(BackendError::ExecutionUnavailable)
    }

    /// Returns whether cancellation is supported.
    ///
    /// This is a capability query and performs no network operation.
    fn supports_cancellation(&self) -> bool {
        false
    }

    /// Returns whether queue information is supported.
    ///
    /// This is a capability query and performs no network operation.
    fn supports_queue_info(&self) -> bool {
        false
    }

    /// Returns whether the adapter has a native synchronous execution API.
    fn supports_synchronous_execution(&self) -> bool {
        false
    }
}

/// =============================================================================
/// Conformance marker
/// =============================================================================

/// Marker trait for adapters that have passed Zamani's backend conformance
/// suite.
///
/// This is intentionally NOT blanket-implemented. An adapter must explicitly
/// opt into the conformance category after passing the suite.
///
/// This prevents the type system from falsely claiming that every adapter is
/// production-conformant merely because it implements the basic trait.
pub trait ConformantQuantumBackendAdapter: QuantumBackendAdapter {}

/// =============================================================================
/// Validation helpers
/// =============================================================================

fn validate_format(format: &str) -> Result<(), BackendError> {
    if format.trim().is_empty()
        || format.len() > MAX_PROGRAM_FORMAT_LENGTH
        || format.chars().any(char::is_control)
    {
        return Err(BackendError::ExecutionUnavailable);
    }

    Ok(())
}

fn validate_identifier(
    field: &str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendError> {
    if field.trim().is_empty()
        || value.trim().is_empty()
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::ExecutionUnavailable);
    }

    Ok(())
}

/// =============================================================================
/// Tests
/// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_accepts_non_empty_payload() {
        let program =
            BackendProgram::new(
                "openqasm-3.1",
                b"OPENQASM 3.0;".to_vec(),
            )
            .expect("valid program");

        assert_eq!(program.format(), "openqasm-3.1");
        assert_eq!(program.len(), 13);
        assert!(!program.is_empty());
        assert_eq!(program.bytes(), b"OPENQASM 3.0;");
    }

    #[test]
    fn program_rejects_empty_payload() {
        assert!(
            BackendProgram::new(
                "openqasm-3.1",
                Vec::<u8>::new()
            )
            .is_err()
        );
    }

    #[test]
    fn program_debug_does_not_expose_program_bytes() {
        let program =
            BackendProgram::new(
                "openqasm-3.1",
                b"SECRET_PROGRAM_PAYLOAD".to_vec(),
            )
            .expect("valid program");

        let debug = format!("{program:?}");

        assert!(debug.contains("openqasm-3.1"));
        assert!(debug.contains("byte_len"));
        assert!(!debug.contains("SECRET_PROGRAM_PAYLOAD"));
    }

    #[test]
    fn job_id_is_deterministic() {
        let id =
            BackendJobId::new("provider-task-001")
                .expect("valid job id");

        assert_eq!(id.as_str(), "provider-task-001");
        assert_eq!(id.to_string(), "provider-task-001");
    }

    #[test]
    fn lifecycle_terminal_states_are_correct() {
        assert!(BackendJobState::Completed.is_terminal());
        assert!(BackendJobState::Cancelled.is_terminal());
        assert!(BackendJobState::Failed.is_terminal());
        assert!(BackendJobState::Expired.is_terminal());
        assert!(BackendJobState::TimedOut.is_terminal());

        assert!(!BackendJobState::Running.is_terminal());
        assert!(!BackendJobState::Queued.is_terminal());
    }

    #[test]
    fn lifecycle_active_states_are_correct() {
        assert!(BackendJobState::Created.is_active());
        assert!(BackendJobState::Queued.is_active());
        assert!(BackendJobState::Running.is_active());
        assert!(BackendJobState::Cancelling.is_active());

        assert!(!BackendJobState::Completed.is_active());
        assert!(!BackendJobState::Failed.is_active());
    }

    #[test]
    fn completed_job_can_report_result_availability() {
        let job =
            BackendJob::new(
                BackendJobId::new("job-1")
                    .expect("valid job"),
                "local/simulator",
                None,
                BackendJobState::Completed,
            )
            .expect("valid backend job");

        let status = BackendJobStatus {
            job,
            provider_status: None,
            queue_position: None,
            estimated_wait: None,
            result_available: true,
        };

        assert!(status.can_retrieve_result());
        assert!(!status.can_request_cancellation());
    }

    #[test]
    fn running_job_can_be_cancelled() {
        let job =
            BackendJob::new(
                BackendJobId::new("job-2")
                    .expect("valid job"),
                "provider/qpu",
                None,
                BackendJobState::Running,
            )
            .expect("valid backend job");

        let status = BackendJobStatus {
            job,
            provider_status: None,
            queue_position: None,
            estimated_wait: None,
            result_available: false,
        };

        assert!(!status.can_retrieve_result());
        assert!(status.can_request_cancellation());
    }

    #[test]
    fn health_is_conservative() {
        assert!(
            BackendHealthState::Healthy
                .permits_execution()
        );

        assert!(
            BackendHealthState::Degraded
                .permits_execution()
        );

        assert!(
            !BackendHealthState::Unknown
                .permits_execution()
        );

        assert!(
            !BackendHealthState::Unhealthy
                .permits_execution()
        );

        assert!(
            !BackendHealthState::Unreachable
                .permits_execution()
        );
    }

    #[test]
    fn adapter_info_validates_identity() {
        let info =
            BackendAdapterInfo::new(
                "local",
                "1.0.0",
                true,
            )
            .expect("valid adapter info");

        assert_eq!(info.adapter_id, "local");
        assert_eq!(info.adapter_version, "1.0.0");
        assert!(info.production_ready);
    }

    #[test]
    fn adapter_trait_is_object_safe() {
        fn accepts_object(
            _: &dyn QuantumBackendAdapter,
        ) {
        }

        let _ = accepts_object;
    }
}