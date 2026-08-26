//! Zamani Quantum Benchmarking — Execution Request
//!
//! Production execution-request boundary for the quantum benchmarking
//! subsystem.
//!
//! # Responsibility
//!
//! This module defines the immutable, validated request that connects a
//! benchmark experiment to an execution adapter.
//!
//! It owns:
//!
//! - request identity;
//! - experiment identity;
//! - circuit identity;
//! - requested shot count;
//! - execution mode;
//! - timeout policy;
//! - retry policy;
//! - priority;
//! - backend selection;
//! - optional backend version constraint;
//! - request tags;
//! - deterministic request metadata;
//! - request-level resource limits;
//! - validation before execution;
//! - deterministic request fingerprinting.
//!
//! It does NOT own:
//!
//! - quantum circuit semantics;
//! - circuit generation;
//! - circuit compilation;
//! - routing;
//! - scheduling;
//! - hardware communication;
//! - backend-specific protocol payloads;
//! - statistical analysis;
//! - benchmark metrics;
//! - result interpretation.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! BenchmarkExperiment
//!        │
//!        ▼
//! BenchmarkExecutionRequest
//!        │
//!        ├──────────────► execution::executor
//!        │
//!        └──────────────► hardware/backend adapter
//!                              │
//!                              ▼
//!                         simulator / QPU
//! ```
//!
//! # Important integration rule
//!
//! `BenchmarkExecutionRequest` is deliberately different from
//! `quantum::hardware::backend::ExecutionRequest`.
//!
//! The benchmarking request describes *why and under what policy* a circuit
//! should be executed. The hardware request describes *what the backend needs*
//! to execute it.
//!
//! The conversion between the two belongs in the execution adapter/executor
//! layer, not in this module.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.
//!
//! # Serialization
//!
//! This request is intended to be transportable and persistable. Only stable
//! request metadata is serialized. The canonical QuantumCircuit remains owned
//! by the benchmarking core/experiment layer and is referenced here by its
//! stable `CircuitId`.
//!
//! This avoids coupling execution transport to the internal representation of
//! `QuantumCircuit`.
//!
//! # Determinism
//!
//! Request fingerprints are calculated from a canonical field ordering.
//! `BTreeMap` is used for metadata/tags so logically equivalent requests do
//! not acquire different fingerprints because of insertion order.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::quantum::ir::CircuitId;

use super::super::core::errors::BenchmarkError;
use super::super::core::limits::BenchmarkLimits;
use super::super::core::provenance::ExperimentId;

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable identifier for a benchmark execution request.
///
/// The identifier is distinct from the experiment identifier because one
/// experiment may generate multiple execution requests.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionRequestId(String);

impl ExecutionRequestId {
    /// Creates an identifier from a validated string.
    pub fn new(value: impl Into<String>) -> Result<Self, RequestError> {
        let value = value.into();

        validate_identifier("execution request ID", &value)?;

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionRequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Backend selection
// =============================================================================

/// Stable backend-selection policy.
///
/// The benchmarking subsystem does not contain provider-specific credentials
/// or transport configuration. Those belong to backend/runtime adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendSelection {
    /// Select the default backend configured by the caller/runtime.
    Default,

    /// Select a backend by its stable identifier.
    Id(String),
}

impl BackendSelection {
    /// Creates a backend-ID selection.
    pub fn id(value: impl Into<String>) -> Result<Self, RequestError> {
        let value = value.into();

        validate_identifier("backend ID", &value)?;

        Ok(Self::Id(value))
    }

    /// Returns the selected backend ID, if explicitly specified.
    pub fn backend_id(&self) -> Option<&str> {
        match self {
            Self::Default => None,
            Self::Id(value) => Some(value.as_str()),
        }
    }

    fn canonical_value(&self) -> String {
        match self {
            Self::Default => "default".to_owned(),
            Self::Id(value) => {
                format!("id:{value}")
            }
        }
    }
}

// =============================================================================
// Execution mode
// =============================================================================

/// Requested execution mode.
///
/// A benchmark may execute against a simulator, emulator, physical QPU, or
/// another backend. This enum expresses the caller's intent without
/// introducing provider-specific types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Prefer a local software simulator.
    Simulator,

    /// Prefer a hardware-model emulator.
    Emulator,

    /// Require physical quantum hardware.
    Qpu,

    /// Allow the backend registry to choose an appropriate target.
    Auto,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Auto
    }
}

// =============================================================================
// Retry policy
// =============================================================================

/// Retry policy for execution failures.
///
/// Retries are deliberately conservative. The executor must only retry
/// failures that it knows are safe to retry.
///
/// In particular, a timeout after an unknown remote submission state must not
/// automatically result in duplicate hardware execution unless the backend
/// provides idempotent submission semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retries after the initial attempt.
    pub max_retries: u32,

    /// Base delay between retries in milliseconds.
    pub initial_backoff_ms: u64,

    /// Maximum delay between retries in milliseconds.
    pub max_backoff_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 0,
            initial_backoff_ms: 0,
            max_backoff_ms: 0,
        }
    }
}

impl RetryPolicy {
    /// Creates a no-retry policy.
    pub const fn none() -> Self {
        Self {
            max_retries: 0,
            initial_backoff_ms: 0,
            max_backoff_ms: 0,
        }
    }

    /// Creates a retry policy.
    pub fn new(
        max_retries: u32,
        initial_backoff_ms: u64,
        max_backoff_ms: u64,
    ) -> Result<Self, RequestError> {
        if max_backoff_ms < initial_backoff_ms {
            return Err(RequestError::InvalidRetryPolicy {
                message: "maximum backoff cannot be smaller than initial backoff",
            });
        }

        Ok(Self {
            max_retries,
            initial_backoff_ms,
            max_backoff_ms,
        })
    }

    /// Returns whether retries are enabled.
    pub const fn enabled(&self) -> bool {
        self.max_retries > 0
    }
}

// =============================================================================
// Priority
// =============================================================================

/// Execution priority.
///
/// This is a scheduling hint only. It must never be interpreted as permission
/// to bypass backend safety, quota, fairness, or authorization policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ExecutionPriority {
    Background,
    Normal,
    High,
    Critical,
}

impl Default for ExecutionPriority {
    fn default() -> Self {
        Self::Normal
    }
}

// =============================================================================
// Timeout policy
// =============================================================================

/// Execution timeout policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeoutPolicy {
    /// Maximum wall-clock execution duration.
    ///
    /// This is a request-level deadline, not a guarantee that the backend
    /// itself supports cancellation.
    pub timeout_ms: u64,
}

impl TimeoutPolicy {
    /// Creates a timeout policy from a duration.
    pub fn from_duration(duration: Duration) -> Result<Self, RequestError> {
        let millis = duration.as_millis();

        let timeout_ms =
            u64::try_from(millis).map_err(|_| RequestError::TimeoutOverflow)?;

        if timeout_ms == 0 {
            return Err(RequestError::InvalidTimeout);
        }

        Ok(Self { timeout_ms })
    }

    /// Creates a timeout policy from milliseconds.
    pub const fn from_millis(timeout_ms: u64) -> Result<Self, RequestError> {
        if timeout_ms == 0 {
            return Err(RequestError::InvalidTimeout);
        }

        Ok(Self { timeout_ms })
    }

    /// Returns the timeout as a [`Duration`].
    pub fn duration(&self) -> Duration {
        Duration::from_millis(self.timeout_ms)
    }
}

// =============================================================================
// Request limits
// =============================================================================

/// Request-local resource constraints.
///
/// These are intentionally stricter than, or equal to, the global benchmark
/// limits. Backend limits are checked later by the backend capability layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestLimits {
    /// Maximum number of shots permitted by this request.
    pub max_shots: usize,

    /// Maximum number of bytes permitted for request metadata.
    pub max_metadata_bytes: usize,
}

impl Default for RequestLimits {
    fn default() -> Self {
        Self {
            max_shots: 0,
            max_metadata_bytes: 0,
        }
    }
}

impl RequestLimits {
    /// Creates unrestricted request-local limits.
    ///
    /// Zero means "use the enclosing/global policy".
    pub const fn unlimited() -> Self {
        Self {
            max_shots: 0,
            max_metadata_bytes: 0,
        }
    }

    /// Creates explicit limits.
    pub const fn new(
        max_shots: usize,
        max_metadata_bytes: usize,
    ) -> Self {
        Self {
            max_shots,
            max_metadata_bytes,
        }
    }

    fn validate_against(
        &self,
        shots: usize,
        metadata_bytes: usize,
        global: &BenchmarkLimits,
    ) -> Result<(), RequestError> {
        if self.max_shots != 0 && shots > self.max_shots {
            return Err(RequestError::ShotLimitExceeded {
                requested: shots,
                maximum: self.max_shots,
            });
        }

        if global.max_shots != 0 && shots > global.max_shots {
            return Err(RequestError::GlobalShotLimitExceeded {
                requested: shots,
                maximum: global.max_shots,
            });
        }

        if self.max_metadata_bytes != 0
            && metadata_bytes > self.max_metadata_bytes
        {
            return Err(RequestError::MetadataLimitExceeded {
                requested: metadata_bytes,
                maximum: self.max_metadata_bytes,
            });
        }

        if global.max_metadata_bytes != 0
            && metadata_bytes > global.max_metadata_bytes
        {
            return Err(RequestError::GlobalMetadataLimitExceeded {
                requested: metadata_bytes,
                maximum: global.max_metadata_bytes,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Request error
// =============================================================================

/// Errors produced while creating or validating an execution request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    EmptyRequestId,

    InvalidRequestId {
        reason: &'static str,
    },

    InvalidExperimentId,

    InvalidCircuitId,

    InvalidBackendId,

    ZeroShots,

    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    GlobalShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    MetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    GlobalMetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    MetadataKeyEmpty,

    MetadataValueEmpty,

    MetadataKeyTooLong,

    MetadataValueTooLong,

    MetadataSizeOverflow,

    TooManyMetadataEntries,

    InvalidTimeout,

    TimeoutOverflow,

    InvalidRetryPolicy {
        message: &'static str,
    },

    InvalidIdempotencyKey,

    IdempotencyKeyTooLong,

    EmptyTag,

    TagTooLong,

    TooManyTags,

    InvalidLimits,

    RequestAlreadyCancelled,

    InvalidRequest {
        message: &'static str,
    },
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequestId => {
                f.write_str("execution request ID cannot be empty")
            }

            Self::InvalidRequestId { reason } => {
                write!(f, "invalid execution request ID: {reason}")
            }

            Self::InvalidExperimentId => {
                f.write_str("experiment ID is invalid")
            }

            Self::InvalidCircuitId => {
                f.write_str("circuit ID is invalid")
            }

            Self::InvalidBackendId => {
                f.write_str("backend ID is invalid")
            }

            Self::ZeroShots => {
                f.write_str("execution request must contain at least one shot")
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request requires {requested} shots but the request limit is {maximum}"
                )
            }

            Self::GlobalShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request requires {requested} shots but the global benchmark limit is {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request metadata requires {requested} bytes but the request limit is {maximum}"
                )
            }

            Self::GlobalMetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "request metadata requires {requested} bytes but the global benchmark limit is {maximum}"
                )
            }

            Self::MetadataKeyEmpty => {
                f.write_str("metadata keys cannot be empty")
            }

            Self::MetadataValueEmpty => {
                f.write_str("metadata values cannot be empty")
            }

            Self::MetadataKeyTooLong => {
                f.write_str("metadata key exceeds the permitted length")
            }

            Self::MetadataValueTooLong => {
                f.write_str("metadata value exceeds the permitted length")
            }

            Self::MetadataSizeOverflow => {
                f.write_str("metadata size calculation overflowed")
            }

            Self::TooManyMetadataEntries => {
                f.write_str("request contains too many metadata entries")
            }

            Self::InvalidTimeout => {
                f.write_str("execution timeout must be greater than zero")
            }

            Self::TimeoutOverflow => {
                f.write_str("execution timeout cannot be represented")
            }

            Self::InvalidRetryPolicy { message } => {
                write!(f, "invalid retry policy: {message}")
            }

            Self::InvalidIdempotencyKey => {
                f.write_str("idempotency key is invalid")
            }

            Self::IdempotencyKeyTooLong => {
                f.write_str("idempotency key is too long")
            }

            Self::EmptyTag => {
                f.write_str("request tags cannot be empty")
            }

            Self::TagTooLong => {
                f.write_str("request tag is too long")
            }

            Self::TooManyTags => {
                f.write_str("execution request contains too many tags")
            }

            Self::InvalidLimits => {
                f.write_str("execution request contains invalid limits")
            }

            Self::RequestAlreadyCancelled => {
                f.write_str("execution request has already been cancelled")
            }

            Self::InvalidRequest { message } => {
                write!(f, "invalid execution request: {message}")
            }
        }
    }
}

impl std::error::Error for RequestError {}

impl From<RequestError> for BenchmarkError {
    fn from(error: RequestError) -> Self {
        BenchmarkError::InvalidExecutionRequest {
            message: error.to_string(),
        }
    }
}

// =============================================================================
// Execution request
// =============================================================================

/// Immutable, validated request for executing one benchmark circuit.
///
/// A request refers to a canonical circuit by `CircuitId`. The actual
/// `QuantumCircuit` remains owned by the benchmark experiment/circuit layer.
///
/// This is intentional:
///
/// ```text
/// request
///   │
///   └── CircuitId ─────────► BenchmarkCircuit / QuantumCircuit
/// ```
///
/// rather than embedding a second circuit representation inside the
/// execution layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkExecutionRequest {
    /// Stable request identity.
    request_id: ExecutionRequestId,

    /// Experiment that owns this execution.
    experiment_id: ExperimentId,

    /// Canonical Quantum IR circuit identity.
    circuit_id: CircuitId,

    /// Number of requested shots.
    shots: usize,

    /// Requested execution target kind.
    execution_mode: ExecutionMode,

    /// Backend selection policy.
    backend: BackendSelection,

    /// Maximum permitted execution time.
    timeout: Option<TimeoutPolicy>,

    /// Retry behavior.
    retry_policy: RetryPolicy,

    /// Scheduling hint.
    priority: ExecutionPriority,

    /// Optional idempotency key.
    ///
    /// This must be supplied for remote providers when the provider supports
    /// idempotent submission. The executor decides whether it is required.
    idempotency_key: Option<String>,

    /// Stable caller-defined tags.
    tags: Vec<String>,

    /// Stable metadata.
    metadata: BTreeMap<String, String>,

    /// Request-local resource constraints.
    limits: RequestLimits,
}

impl BenchmarkExecutionRequest {
    /// Creates a production execution request with conservative defaults.
    pub fn new(
        request_id: ExecutionRequestId,
        experiment_id: ExperimentId,
        circuit_id: CircuitId,
        shots: usize,
    ) -> Result<Self, RequestError> {
        if shots == 0 {
            return Err(RequestError::ZeroShots);
        }

        let request = Self {
            request_id,
            experiment_id,
            circuit_id,
            shots,
            execution_mode: ExecutionMode::Auto,
            backend: BackendSelection::Default,
            timeout: None,
            retry_policy: RetryPolicy::none(),
            priority: ExecutionPriority::Normal,
            idempotency_key: None,
            tags: Vec::new(),
            metadata: BTreeMap::new(),
            limits: RequestLimits::unlimited(),
        };

        request.validate_basic()?;

        Ok(request)
    }

    // -------------------------------------------------------------------------
    // Builder-style configuration
    // -------------------------------------------------------------------------

    /// Sets the execution mode.
    pub fn with_execution_mode(
        mut self,
        mode: ExecutionMode,
    ) -> Self {
        self.execution_mode = mode;
        self
    }

    /// Sets the backend selection policy.
    pub fn with_backend(
        mut self,
        backend: BackendSelection,
    ) -> Result<Self, RequestError> {
        if let BackendSelection::Id(ref id) = backend {
            validate_identifier("backend ID", id)?;
        }

        self.backend = backend;
        Ok(self)
    }

    /// Sets an execution timeout.
    pub fn with_timeout(
        mut self,
        timeout: TimeoutPolicy,
    ) -> Result<Self, RequestError> {
        if timeout.timeout_ms == 0 {
            return Err(RequestError::InvalidTimeout);
        }

        self.timeout = Some(timeout);
        Ok(self)
    }

    /// Sets retry behavior.
    pub fn with_retry_policy(
        mut self,
        retry_policy: RetryPolicy,
    ) -> Result<Self, RequestError> {
        retry_policy.validate()?;

        self.retry_policy = retry_policy;
        Ok(self)
    }

    /// Sets execution priority.
    pub fn with_priority(
        mut self,
        priority: ExecutionPriority,
    ) -> Self {
        self.priority = priority;
        self
    }

    /// Sets an idempotency key.
    pub fn with_idempotency_key(
        mut self,
        key: impl Into<String>,
    ) -> Result<Self, RequestError> {
        let key = key.into();

        validate_idempotency_key(&key)?;

        self.idempotency_key = Some(key);
        Ok(self)
    }

    /// Adds a deterministic tag.
    pub fn with_tag(
        mut self,
        tag: impl Into<String>,
    ) -> Result<Self, RequestError> {
        let tag = tag.into();

        validate_tag(&tag)?;

        if self.tags.len() >= MAX_TAGS {
            return Err(RequestError::TooManyTags);
        }

        if !self.tags.iter().any(|existing| existing == &tag) {
            self.tags.push(tag);
            self.tags.sort();
        }

        Ok(self)
    }

    /// Adds stable metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, RequestError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_entry(&key, &value)?;

        if !self.metadata.contains_key(&key)
            && self.metadata.len() >= MAX_METADATA_ENTRIES
        {
            return Err(RequestError::TooManyMetadataEntries);
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Sets request-local resource limits.
    pub fn with_limits(
        mut self,
        limits: RequestLimits,
    ) -> Result<Self, RequestError> {
        if limits.max_shots == 0
            && limits.max_metadata_bytes == 0
        {
            self.limits = limits;
            return Ok(self);
        }

        self.limits = limits;

        self.validate_basic()?;

        Ok(self)
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns the stable request ID.
    pub fn request_id(&self) -> &ExecutionRequestId {
        &self.request_id
    }

    /// Returns the owning experiment ID.
    pub fn experiment_id(&self) -> &ExperimentId {
        &self.experiment_id
    }

    /// Returns the canonical circuit ID.
    pub fn circuit_id(&self) -> &CircuitId {
        &self.circuit_id
    }

    /// Returns the requested shot count.
    pub const fn shots(&self) -> usize {
        self.shots
    }

    /// Returns the execution mode.
    pub const fn execution_mode(&self) -> ExecutionMode {
        self.execution_mode
    }

    /// Returns the backend selection.
    pub fn backend(&self) -> &BackendSelection {
        &self.backend
    }

    /// Returns the configured timeout.
    pub const fn timeout(&self) -> Option<TimeoutPolicy> {
        self.timeout
    }

    /// Returns the retry policy.
    pub const fn retry_policy(&self) -> RetryPolicy {
        self.retry_policy
    }

    /// Returns execution priority.
    pub const fn priority(&self) -> ExecutionPriority {
        self.priority
    }

    /// Returns the optional idempotency key.
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    /// Returns request tags.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Returns request metadata.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns request-local limits.
    pub const fn limits(&self) -> RequestLimits {
        self.limits
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Performs request-only validation.
    ///
    /// This does not validate whether a backend supports the circuit. Backend
    /// capability validation belongs to the execution/backend layer.
    pub fn validate_basic(&self) -> Result<(), RequestError> {
        validate_identifier(
            "execution request ID",
            self.request_id.as_str(),
        )?;

        if self.shots == 0 {
            return Err(RequestError::ZeroShots);
        }

        if let BackendSelection::Id(id) = &self.backend {
            validate_identifier("backend ID", id)?;
        }

        if let Some(timeout) = self.timeout {
            if timeout.timeout_ms == 0 {
                return Err(RequestError::InvalidTimeout);
            }
        }

        self.retry_policy.validate()?;

        if let Some(key) = &self.idempotency_key {
            validate_idempotency_key(key)?;
        }

        if self.tags.len() > MAX_TAGS {
            return Err(RequestError::TooManyTags);
        }

        for tag in &self.tags {
            validate_tag(tag)?;
        }

        if self.metadata.len() > MAX_METADATA_ENTRIES {
            return Err(RequestError::TooManyMetadataEntries);
        }

        for (key, value) in &self.metadata {
            validate_metadata_entry(key, value)?;
        }

        let metadata_bytes = self.metadata_byte_size()?;

        if self.limits.max_shots != 0
            && self.shots > self.limits.max_shots
        {
            return Err(RequestError::ShotLimitExceeded {
                requested: self.shots,
                maximum: self.limits.max_shots,
            });
        }

        if self.limits.max_metadata_bytes != 0
            && metadata_bytes > self.limits.max_metadata_bytes
        {
            return Err(RequestError::MetadataLimitExceeded {
                requested: metadata_bytes,
                maximum: self.limits.max_metadata_bytes,
            });
        }

        Ok(())
    }

    /// Validates the request against the global benchmark limits.
    pub fn validate_against(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), RequestError> {
        self.validate_basic()?;

        let metadata_bytes = self.metadata_byte_size()?;

        self.limits.validate_against(
            self.shots,
            metadata_bytes,
            limits,
        )?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Canonical representation and fingerprinting
    // -------------------------------------------------------------------------

    /// Returns the deterministic canonical representation used for hashing.
    ///
    /// This representation deliberately avoids `Debug` formatting and Rust
    /// implementation details.
    pub fn canonical_form(&self) -> String {
        let mut output = String::new();

        append_field(
            &mut output,
            "request_id",
            self.request_id.as_str(),
        );

        append_field(
            &mut output,
            "experiment_id",
            &self.experiment_id.to_string(),
        );

        append_field(
            &mut output,
            "circuit_id",
            &self.circuit_id.to_string(),
        );

        append_field(
            &mut output,
            "shots",
            &self.shots.to_string(),
        );

        append_field(
            &mut output,
            "execution_mode",
            execution_mode_name(self.execution_mode),
        );

        append_field(
            &mut output,
            "backend",
            &self.backend.canonical_value(),
        );

        append_field(
            &mut output,
            "timeout_ms",
            self.timeout
                .map(|value| value.timeout_ms.to_string())
                .unwrap_or_else(|| "none".to_owned())
                .as_str(),
        );

        append_field(
            &mut output,
            "max_retries",
            &self.retry_policy.max_retries.to_string(),
        );

        append_field(
            &mut output,
            "initial_backoff_ms",
            &self.retry_policy.initial_backoff_ms.to_string(),
        );

        append_field(
            &mut output,
            "max_backoff_ms",
            &self.retry_policy.max_backoff_ms.to_string(),
        );

        append_field(
            &mut output,
            "priority",
            priority_name(self.priority),
        );

        append_field(
            &mut output,
            "idempotency_key",
            self.idempotency_key
                .as_deref()
                .unwrap_or("none"),
        );

        append_field(
            &mut output,
            "max_shots",
            &self.limits.max_shots.to_string(),
        );

        append_field(
            &mut output,
            "max_metadata_bytes",
            &self.limits.max_metadata_bytes.to_string(),
        );

        for tag in &self.tags {
            append_field(&mut output, "tag", tag);
        }

        for (key, value) in &self.metadata {
            append_field(
                &mut output,
                &format!("metadata:{key}"),
                value,
            );
        }

        output
    }

    /// Returns a SHA-256 fingerprint of the complete canonical request.
    ///
    /// SHA-256 is used here as a deterministic content fingerprint, not as a
    /// secret or authentication mechanism.
    pub fn fingerprint(&self) -> String {
        let canonical = self.canonical_form();

        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());

        hex::encode(hasher.finalize())
    }

    /// Returns a stable, human-readable short fingerprint.
    pub fn short_fingerprint(&self) -> String {
        let fingerprint = self.fingerprint();

        fingerprint[..16].to_owned()
    }
}

// =============================================================================
// Retry validation
// =============================================================================

impl RetryPolicy {
    fn validate(&self) -> Result<(), RequestError> {
        if self.max_backoff_ms < self.initial_backoff_ms {
            return Err(RequestError::InvalidRetryPolicy {
                message: "maximum backoff cannot be smaller than initial backoff",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Constants
// =============================================================================

/// Maximum UTF-8 bytes allowed in an identifier.
const MAX_IDENTIFIER_BYTES: usize = 256;

/// Maximum UTF-8 bytes allowed in an idempotency key.
const MAX_IDEMPOTENCY_KEY_BYTES: usize = 256;

/// Maximum UTF-8 bytes allowed in a metadata key.
const MAX_METADATA_KEY_BYTES: usize = 256;

/// Maximum UTF-8 bytes allowed in a metadata value.
const MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Maximum number of metadata entries.
const MAX_METADATA_ENTRIES: usize = 128;

/// Maximum UTF-8 bytes allowed in a tag.
const MAX_TAG_BYTES: usize = 128;

/// Maximum number of tags.
const MAX_TAGS: usize = 64;

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    name: &'static str,
    value: &str,
) -> Result<(), RequestError> {
    if value.is_empty() {
        return match name {
            "execution request ID" => {
                Err(RequestError::EmptyRequestId)
            }

            "backend ID" => Err(RequestError::InvalidBackendId),

            _ => Err(RequestError::InvalidRequest {
                message: "identifier cannot be empty",
            }),
        };
    }

    if value.len() > MAX_IDENTIFIER_BYTES {
        return Err(RequestError::InvalidRequestId {
            reason: "identifier exceeds maximum length",
        });
    }

    if value.trim() != value {
        return Err(RequestError::InvalidRequestId {
            reason: "identifier cannot contain leading or trailing whitespace",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(RequestError::InvalidRequestId {
            reason: "identifier cannot contain control characters",
        });
    }

    Ok(())
}

fn validate_idempotency_key(
    value: &str,
) -> Result<(), RequestError> {
    if value.is_empty() {
        return Err(RequestError::InvalidIdempotencyKey);
    }

    if value.len() > MAX_IDEMPOTENCY_KEY_BYTES {
        return Err(RequestError::IdempotencyKeyTooLong);
    }

    if value.chars().any(char::is_control) {
        return Err(RequestError::InvalidIdempotencyKey);
    }

    Ok(())
}

fn validate_tag(value: &str) -> Result<(), RequestError> {
    if value.is_empty() {
        return Err(RequestError::EmptyTag);
    }

    if value.len() > MAX_TAG_BYTES {
        return Err(RequestError::TagTooLong);
    }

    if value.chars().any(char::is_control) {
        return Err(RequestError::InvalidRequest {
            message: "tags cannot contain control characters",
        });
    }

    Ok(())
}

fn validate_metadata_entry(
    key: &str,
    value: &str,
) -> Result<(), RequestError> {
    if key.is_empty() {
        return Err(RequestError::MetadataKeyEmpty);
    }

    if value.is_empty() {
        return Err(RequestError::MetadataValueEmpty);
    }

    if key.len() > MAX_METADATA_KEY_BYTES {
        return Err(RequestError::MetadataKeyTooLong);
    }

    if value.len() > MAX_METADATA_VALUE_BYTES {
        return Err(RequestError::MetadataValueTooLong);
    }

    if key.chars().any(char::is_control) {
        return Err(RequestError::InvalidRequest {
            message: "metadata keys cannot contain control characters",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(RequestError::InvalidRequest {
            message: "metadata values cannot contain control characters",
        });
    }

    Ok(())
}

fn execution_mode_name(mode: ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::Simulator => "simulator",
        ExecutionMode::Emulator => "emulator",
        ExecutionMode::Qpu => "qpu",
        ExecutionMode::Auto => "auto",
    }
}

fn priority_name(priority: ExecutionPriority) -> &'static str {
    match priority {
        ExecutionPriority::Background => "background",
        ExecutionPriority::Normal => "normal",
        ExecutionPriority::High => "high",
        ExecutionPriority::Critical => "critical",
    }
}

fn append_field(
    output: &mut String,
    key: &str,
    value: &str,
) {
    output.push_str(key.len().to_string().as_str());
    output.push(':');
    output.push_str(key);

    output.push('=');

    output.push_str(value.len().to_string().as_str());
    output.push(':');
    output.push_str(value);

    output.push('\n');
}

fn metadata_byte_size(
    metadata: &BTreeMap<String, String>,
) -> Result<usize, RequestError> {
    let mut total = 0usize;

    for (key, value) in metadata {
        let entry_size = key
            .len()
            .checked_add(value.len())
            .and_then(|size| size.checked_add(1))
            .ok_or(RequestError::MetadataSizeOverflow)?;

        total = total
            .checked_add(entry_size)
            .ok_or(RequestError::MetadataSizeOverflow)?;
    }

    Ok(total)
}

impl BenchmarkExecutionRequest {
    fn metadata_byte_size(&self) -> Result<usize, RequestError> {
        metadata_byte_size(&self.metadata)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> BenchmarkExecutionRequest {
        let request_id =
            ExecutionRequestId::new("request-001").unwrap();

        let experiment_id =
            ExperimentId::new("experiment-001").unwrap();

        let circuit_id =
            CircuitId::new("circuit-001").unwrap();

        BenchmarkExecutionRequest::new(
            request_id,
            experiment_id,
            circuit_id,
            1_000,
        )
        .unwrap()
    }

    #[test]
    fn valid_request_is_created() {
        let request = request();

        assert_eq!(request.shots(), 1_000);
        assert_eq!(
            request.execution_mode(),
            ExecutionMode::Auto
        );
        assert_eq!(
            request.priority(),
            ExecutionPriority::Normal
        );
    }

    #[test]
    fn zero_shots_are_rejected() {
        let request_id =
            ExecutionRequestId::new("request-001").unwrap();

        let experiment_id =
            ExperimentId::new("experiment-001").unwrap();

        let circuit_id =
            CircuitId::new("circuit-001").unwrap();

        let result = BenchmarkExecutionRequest::new(
            request_id,
            experiment_id,
            circuit_id,
            0,
        );

        assert_eq!(
            result,
            Err(RequestError::ZeroShots)
        );
    }

    #[test]
    fn backend_id_is_validated() {
        assert!(
            BackendSelection::id("").is_err()
        );

        assert!(
            BackendSelection::id("local-simulator").is_ok()
        );
    }

    #[test]
    fn timeout_must_be_positive() {
        assert!(
            TimeoutPolicy::from_millis(0).is_err()
        );

        assert!(
            TimeoutPolicy::from_millis(1_000).is_ok()
        );
    }

    #[test]
    fn retry_policy_rejects_invalid_backoff() {
        let result =
            RetryPolicy::new(3, 2_000, 1_000);

        assert!(result.is_err());
    }

    #[test]
    fn tags_are_deterministically_ordered() {
        let request = request()
            .with_tag("z")
            .unwrap()
            .with_tag("a")
            .unwrap();

        assert_eq!(
            request.tags(),
            &["a".to_owned(), "z".to_owned()]
        );
    }

    #[test]
    fn duplicate_tags_do_not_change_semantics() {
        let request = request()
            .with_tag("benchmark")
            .unwrap()
            .with_tag("benchmark")
            .unwrap();

        assert_eq!(
            request.tags(),
            &["benchmark".to_owned()]
        );
    }

    #[test]
    fn metadata_is_deterministic() {
        let first = request()
            .with_metadata("b", "2")
            .unwrap()
            .with_metadata("a", "1")
            .unwrap();

        let second = request()
            .with_metadata("a", "1")
            .unwrap()
            .with_metadata("b", "2")
            .unwrap();

        assert_eq!(
            first.canonical_form(),
            second.canonical_form()
        );

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn changing_a_semantic_field_changes_fingerprint() {
        let first = request();

        let second = first
            .clone()
            .with_execution_mode(
                ExecutionMode::Simulator
            );

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn request_fingerprint_is_sha256_length() {
        let request = request();

        assert_eq!(
            request.fingerprint().len(),
            64
        );
    }

    #[test]
    fn short_fingerprint_is_stable_length() {
        let request = request();

        assert_eq!(
            request.short_fingerprint().len(),
            16
        );
    }

    #[test]
    fn request_limits_are_enforced() {
        let request = request()
            .with_limits(RequestLimits::new(500, 1_024))
            .unwrap();

        let result = request.validate_basic();

        assert_eq!(
            result,
            Err(RequestError::ShotLimitExceeded {
                requested: 1_000,
                maximum: 500,
            })
        );
    }

    #[test]
    fn metadata_limit_is_enforced() {
        let request = request()
            .with_limits(RequestLimits::new(0, 2))
            .unwrap();

        let result = request
            .clone()
            .with_metadata("a", "b")
            .unwrap()
            .validate_basic();

        assert_eq!(
            result,
            Err(RequestError::MetadataLimitExceeded {
                requested: 3,
                maximum: 2,
            })
        );
    }

    #[test]
    fn idempotency_key_is_preserved() {
        let request = request()
            .with_idempotency_key("idem-001")
            .unwrap();

        assert_eq!(
            request.idempotency_key(),
            Some("idem-001")
        );
    }

    #[test]
    fn request_can_be_validated_against_global_limits() {
        let request = request();

        let limits = BenchmarkLimits::new()
            .with_max_shots(2_000);

        assert!(
            request.validate_against(&limits).is_ok()
        );
    }
}