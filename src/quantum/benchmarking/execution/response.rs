//! Zamani Quantum Benchmarking — Execution Response
//!
//! Production execution-response boundary for the quantum benchmarking
//! subsystem.
//!
//! # Responsibility
//!
//! This module defines the canonical response returned by a benchmark
//! executor after attempting to execute a benchmark execution request.
//!
//! It is intentionally backend-independent.
//!
//! The response preserves:
//!
//! - request identity;
//! - backend identity;
//! - execution status;
//! - requested and completed shots;
//! - partial execution;
//! - normalized raw observations;
//! - execution timing;
//! - backend/provider metadata;
//! - provider-native execution identifiers;
//! - retry/attempt information;
//! - cancellation state;
//! - timeout state;
//! - structured diagnostics;
//! - deterministic response fingerprint;
//! - schema version;
//! - response provenance metadata.
//!
//! # It does NOT
//!
//! This module does not:
//!
//! - execute circuits;
//! - generate circuits;
//! - compile circuits;
//! - route circuits;
//! - schedule circuits;
//! - calculate benchmark metrics;
//! - calculate Quantum Volume;
//! - perform RB/XEB/QEC analysis;
//! - determine scientific benchmark pass/fail;
//! - select hardware;
//! - communicate with a provider;
//! - perform retries itself;
//! - perform logging;
//! - print diagnostics;
//! - assume every backend returns bitstrings.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! BenchmarkExperiment
//!        │
//!        ▼
//! execution::request
//!        │
//!        ▼
//! BenchmarkExecutor
//!        │
//!        ├──────── simulator
//!        ├──────── hardware
//!        ├──────── emulator
//!        ├──────── annealer
//!        └──────── analog backend
//!        │
//!        ▼
//! execution::response
//!        │
//!        ├──────── core::observation
//!        │
//!        ├──────── statistics
//!        │
//!        ├──────── metrics
//!        │
//!        ├──────── protocols
//!        │
//!        └──────── core::result
//! ```
//!
//! # Critical semantic distinction
//!
//! A successful execution is NOT the same thing as a successful benchmark.
//!
//! Example:
//!
//! ```text
//! execution_status = Completed
//! ```
//!
//! means the requested execution completed.
//!
//! The Quantum Volume protocol may subsequently determine:
//!
//! ```text
//! benchmark_status = Failed
//! ```
//!
//! because the measured quality was below the acceptance threshold.
//!
//! This response therefore contains execution state only. Scientific
//! interpretation belongs to the benchmark-analysis layer.
//!
//! # Partial execution
//!
//! Partial execution is first-class.
//!
//! A remote QPU can execute some circuits successfully and fail on later
//! circuits. A cancellation can occur after several shots. A provider can
//! return a timeout after an unknown amount of remote work.
//!
//! The response must preserve what is known rather than converting such a
//! situation into a misleading binary success/failure value.
//!
//! # Backend-neutral observations
//!
//! The canonical observation model already supports counts, probabilities,
//! expectation values, state vectors, density matrices, analog data,
//! annealing samples, QEC syndromes, timing, calibration and backend metadata.
//!
//! This response therefore stores normalized `Observation` values instead of
//! defining another observation representation.
//!
//! # Deterministic fingerprint
//!
//! A response fingerprint is calculated from a canonical serializable
//! representation whose field ordering is deterministic.
//!
//! `BTreeMap` is used for metadata and diagnostics where appropriate.
//!
//! The fingerprint is useful for:
//!
//! - reproducibility;
//! - cache identity;
//! - result deduplication;
//! - audit trails;
//! - regression fixtures;
//! - CI artifacts.
//!
//! It is NOT a cryptographic authorization mechanism.
//!
//! # Resource safety
//!
//! Response construction enforces `BenchmarkLimits` before retaining large
//! observation/diagnostic collections.
//!
//! Deserialized responses MUST be validated before being trusted.
//!
//! # Serialization
//!
//! This module uses Serde because execution responses are part of the
//! machine-readable benchmark interchange boundary.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! No nightly features.
//! No unsafe code.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! execution::request::BenchmarkExecutionRequest
//!              │
//!              ▼
//! execution::executor
//!              │
//!              ▼
//! execution::response::ExecutionResponse
//! ```
//!
//! Downstream:
//!
//! ```text
//! ExecutionResponse
//!      │
//!      ├── core::observation
//!      ├── metrics::*
//!      ├── statistics::*
//!      ├── protocols::*
//!      ├── core::result
//!      ├── reporting::*
//!      └── analysis::*
//! ```
//!
//! This file must remain independent of individual benchmark protocols.

#![deny(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::request::{
    ExecutionBackendId,
    ExecutionRequestId,
};

use super::super::core::limits::{
    BenchmarkLimits,
    LimitError,
};

use super::super::core::observation::Observation;

// =============================================================================
// Schema
// =============================================================================

/// Current execution-response schema version.
///
/// Increment only when the serialized semantic contract changes.
pub const EXECUTION_RESPONSE_SCHEMA_VERSION: u16 = 1;

/// Stable schema identifier.
pub const EXECUTION_RESPONSE_SCHEMA_ID: &str =
    "zamani.quantum.benchmark.execution.response";

/// Maximum provider execution ID length.
pub const MAX_PROVIDER_EXECUTION_ID_LENGTH: usize = 512;

/// Maximum diagnostic code length.
pub const MAX_DIAGNOSTIC_CODE_LENGTH: usize = 128;

/// Maximum diagnostic message length.
pub const MAX_DIAGNOSTIC_MESSAGE_LENGTH: usize = 16 * 1024;

/// Maximum number of response observations.
///
/// The global `BenchmarkLimits` policy remains authoritative, but this
/// additional hard ceiling prevents accidental pathological deserialization.
pub const MAX_RESPONSE_OBSERVATIONS: usize = 100_000_000;

/// Maximum number of diagnostics retained in one response.
pub const MAX_RESPONSE_DIAGNOSTICS: usize = 4_096;

/// Maximum number of metadata fields retained in one response.
pub const MAX_RESPONSE_METADATA_FIELDS: usize = 16_384;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 16 * 1024;

/// Maximum fingerprint length in bytes.
///
/// SHA-256 hexadecimal output is 64 bytes, so this is intentionally larger
/// than required to leave room for future fingerprint algorithms.
pub const MAX_FINGERPRINT_LENGTH: usize = 128;

// =============================================================================
// Execution status
// =============================================================================

/// Lifecycle state of an execution response.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionResponseStatus {
    /// Execution was accepted but no work has started.
    Pending,

    /// Execution is currently running.
    Running,

    /// All requested work completed successfully.
    Completed,

    /// Some requested work completed, but some work did not.
    PartiallyCompleted,

    /// Execution was cancelled.
    Cancelled,

    /// Execution failed before all requested work completed.
    Failed,

    /// Execution exceeded its allowed deadline.
    TimedOut,
}

impl ExecutionResponseStatus {
    /// Returns whether the status represents a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::PartiallyCompleted
                | Self::Cancelled
                | Self::Failed
                | Self::TimedOut
        )
    }

    /// Returns whether all requested work completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether the response contains partial work.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(
            self,
            Self::Running | Self::PartiallyCompleted
        )
    }

    /// Returns whether execution itself encountered failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::TimedOut
        )
    }

    /// Returns whether cancellation caused termination.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl Default for ExecutionResponseStatus {
    fn default() -> Self {
        Self::Pending
    }
}

// =============================================================================
// Diagnostic severity
// =============================================================================

/// Severity of an execution diagnostic.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionDiagnosticSeverity {
    /// Informational execution metadata.
    Info,

    /// Non-fatal execution issue.
    Warning,

    /// Fatal execution issue.
    Error,
}

impl ExecutionDiagnosticSeverity {
    /// Returns whether the diagnostic represents an execution error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

// =============================================================================
// Diagnostic
// =============================================================================

/// Structured execution diagnostic.
///
/// Diagnostics are data. This module never logs or prints them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionDiagnostic {
    /// Severity.
    pub severity: ExecutionDiagnosticSeverity,

    /// Stable machine-readable diagnostic code.
    pub code: String,

    /// Human-readable explanation.
    pub message: String,

    /// Optional circuit/workload scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl ExecutionDiagnostic {
    /// Creates a validated diagnostic.
    pub fn new(
        severity: ExecutionDiagnosticSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, ResponseError> {
        let diagnostic = Self {
            severity,
            code: code.into(),
            message: message.into(),
            scope: None,
        };

        diagnostic.validate()?;

        Ok(diagnostic)
    }

    /// Adds a workload/circuit scope.
    pub fn with_scope(
        mut self,
        scope: impl Into<String>,
    ) -> Result<Self, ResponseError> {
        let scope = scope.into();

        if scope.trim().is_empty() {
            return Err(ResponseError::InvalidDiagnostic {
                field: "scope",
                reason: "scope cannot be empty",
            });
        }

        self.scope = Some(scope);
        self.validate()?;

        Ok(self)
    }

    /// Validates the diagnostic.
    pub fn validate(&self) -> Result<(), ResponseError> {
        validate_identifier(
            "diagnostic.code",
            &self.code,
            MAX_DIAGNOSTIC_CODE_LENGTH,
        )?;

        validate_bounded_non_empty(
            "diagnostic.message",
            &self.message,
            MAX_DIAGNOSTIC_MESSAGE_LENGTH,
        )?;

        if let Some(scope) = &self.scope {
            validate_bounded_non_empty(
                "diagnostic.scope",
                scope,
                MAX_METADATA_VALUE_LENGTH,
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Timing
// =============================================================================

/// Execution timing breakdown.
///
/// These fields intentionally remain separate because queue latency,
/// compilation latency and actual quantum execution time have different
/// meanings for benchmarking.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ExecutionTiming {
    /// Time spent waiting in a provider/backend queue.
    pub queue_time: Duration,

    /// Time spent submitting the request.
    pub submission_time: Duration,

    /// Time spent executing quantum work.
    pub execution_time: Duration,

    /// Time spent receiving/decoding provider results.
    pub result_transfer_time: Duration,

    /// Time spent normalizing raw backend observations.
    pub normalization_time: Duration,

    /// End-to-end wall time measured by the executor.
    pub total_time: Duration,
}

impl Default for ExecutionTiming {
    fn default() -> Self {
        Self {
            queue_time: Duration::ZERO,
            submission_time: Duration::ZERO,
            execution_time: Duration::ZERO,
            result_transfer_time: Duration::ZERO,
            normalization_time: Duration::ZERO,
            total_time: Duration::ZERO,
        }
    }
}

impl ExecutionTiming {
    /// Creates an empty timing record.
    pub const fn zero() -> Self {
        Self {
            queue_time: Duration::ZERO,
            submission_time: Duration::ZERO,
            execution_time: Duration::ZERO,
            result_transfer_time: Duration::ZERO,
            normalization_time: Duration::ZERO,
            total_time: Duration::ZERO,
        }
    }

    /// Validates timing invariants.
    ///
    /// The individual components must be non-negative by construction.
    /// `total_time` is allowed to be less than the sum of components because
    /// providers may execute some phases concurrently.
    pub fn validate(&self) -> Result<(), ResponseError> {
        // Duration cannot represent a negative value, so validation mainly
        // protects the total-duration arithmetic from overflow when callers
        // request derived totals.
        let _ = self
            .queue_time
            .checked_add(self.submission_time)
            .and_then(|value| value.checked_add(self.execution_time))
            .and_then(|value| value.checked_add(self.result_transfer_time))
            .and_then(|value| value.checked_add(self.normalization_time))
            .ok_or(ResponseError::TimingOverflow)?;

        Ok(())
    }

    /// Returns the sum of the individually recorded phases.
    pub fn phase_sum(&self) -> Result<Duration, ResponseError> {
        self.queue_time
            .checked_add(self.submission_time)
            .and_then(|value| value.checked_add(self.execution_time))
            .and_then(|value| value.checked_add(self.result_transfer_time))
            .and_then(|value| value.checked_add(self.normalization_time))
            .ok_or(ResponseError::TimingOverflow)
    }
}

// =============================================================================
// Backend metadata
// =============================================================================

/// Backend metadata captured at execution time.
///
/// Provider-specific fields are retained in `attributes` rather than leaking
/// provider SDK types into the benchmarking subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendExecutionMetadata {
    /// Backend/provider name.
    pub provider: String,

    /// Backend technology description.
    ///
    /// Examples:
    /// - superconducting
    /// - trapped_ion
    /// - neutral_atom
    /// - photonic
    /// - simulator
    /// - annealing
    /// - custom
    pub technology: String,

    /// Backend software/API version if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Device/calibration identity if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration_id: Option<String>,

    /// Calibration timestamp represented as Unix milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration_timestamp_ms: Option<u64>,

    /// Provider-neutral backend attributes.
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
}

impl BackendExecutionMetadata {
    /// Creates validated backend metadata.
    pub fn new(
        provider: impl Into<String>,
        technology: impl Into<String>,
    ) -> Result<Self, ResponseError> {
        let metadata = Self {
            provider: provider.into(),
            technology: technology.into(),
            version: None,
            calibration_id: None,
            calibration_timestamp_ms: None,
            attributes: BTreeMap::new(),
        };

        metadata.validate()?;

        Ok(metadata)
    }

    /// Adds a backend metadata field.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ResponseError> {
        insert_metadata(
            &mut self.attributes,
            key.into(),
            value.into(),
        )?;

        self.validate()?;

        Ok(self)
    }

    /// Validates backend metadata.
    pub fn validate(&self) -> Result<(), ResponseError> {
        validate_bounded_non_empty(
            "backend.provider",
            &self.provider,
            MAX_METADATA_VALUE_LENGTH,
        )?;

        validate_bounded_non_empty(
            "backend.technology",
            &self.technology,
            MAX_METADATA_VALUE_LENGTH,
        )?;

        if let Some(version) = &self.version {
            validate_bounded_non_empty(
                "backend.version",
                version,
                MAX_METADATA_VALUE_LENGTH,
            )?;
        }

        if let Some(calibration_id) = &self.calibration_id {
            validate_bounded_non_empty(
                "backend.calibration_id",
                calibration_id,
                MAX_METADATA_VALUE_LENGTH,
            )?;
        }

        validate_metadata_map(&self.attributes)?;

        Ok(())
    }
}

// =============================================================================
// Response statistics
// =============================================================================

/// Execution-level counters.
///
/// These values describe what happened during execution. They are not
/// benchmark metrics.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ExecutionStatistics {
    /// Number of requested shots.
    pub requested_shots: u64,

    /// Number of shots known to have completed.
    pub completed_shots: u64,

    /// Number of requested shots known to have failed.
    pub failed_shots: u64,

    /// Number of attempts performed by the executor.
    ///
    /// This includes the initial attempt.
    pub attempts: u32,

    /// Number of retries after the initial attempt.
    pub retries: u32,
}

impl ExecutionStatistics {
    /// Creates statistics for a request.
    pub fn new(requested_shots: u64) -> Result<Self, ResponseError> {
        if requested_shots == 0 {
            return Err(ResponseError::InvalidShotCount);
        }

        Ok(Self {
            requested_shots,
            completed_shots: 0,
            failed_shots: 0,
            attempts: 0,
            retries: 0,
        })
    }

    /// Returns whether all requested shots completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completed_shots == self.requested_shots
    }

    /// Returns whether execution is partial.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.completed_shots > 0
            && self.completed_shots < self.requested_shots
    }

    /// Validates the counters.
    pub fn validate(&self) -> Result<(), ResponseError> {
        if self.requested_shots == 0 {
            return Err(ResponseError::InvalidShotCount);
        }

        if self.completed_shots > self.requested_shots {
            return Err(ResponseError::ShotCountInconsistent);
        }

        if self.failed_shots > self.requested_shots {
            return Err(ResponseError::ShotCountInconsistent);
        }

        let accounted = self
            .completed_shots
            .checked_add(self.failed_shots)
            .ok_or(ResponseError::ShotCountOverflow)?;

        if accounted > self.requested_shots {
            return Err(ResponseError::ShotCountInconsistent);
        }

        if self.retries > self.attempts {
            return Err(ResponseError::AttemptCountInconsistent);
        }

        if self.attempts == 0 && (self.completed_shots > 0 || self.failed_shots > 0) {
            return Err(ResponseError::AttemptCountInconsistent);
        }

        Ok(())
    }
}

// =============================================================================
// Response
// =============================================================================

/// Canonical production execution response.
///
/// This is the object returned by the benchmarking execution layer.
///
/// The response is deliberately independent of benchmark protocol semantics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionResponse {
    /// Schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: String,

    /// Request that produced this response.
    pub request_id: ExecutionRequestId,

    /// Backend selected for execution.
    pub backend_id: ExecutionBackendId,

    /// Provider-native execution identifier, when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_execution_id: Option<String>,

    /// Execution lifecycle state.
    pub status: ExecutionResponseStatus,

    /// Execution counters.
    pub statistics: ExecutionStatistics,

    /// Normalized raw observations.
    ///
    /// These are deliberately not converted into benchmark metrics here.
    #[serde(default)]
    pub observations: Vec<Observation>,

    /// Execution timing breakdown.
    pub timing: ExecutionTiming,

    /// Backend metadata captured at execution time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<BackendExecutionMetadata>,

    /// Structured diagnostics.
    #[serde(default)]
    pub diagnostics: Vec<ExecutionDiagnostic>,

    /// Provider-neutral response metadata.
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,

    /// Optional start timestamp supplied by the executor.
    ///
    /// This is Unix epoch milliseconds, supplied externally. The response
    /// layer deliberately does not call the system clock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at_ms: Option<u64>,

    /// Optional completion timestamp supplied by the executor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at_ms: Option<u64>,

    /// Deterministic SHA-256 fingerprint of the response payload.
    ///
    /// This field is populated by `finalize`.
    pub fingerprint: String,
}

impl ExecutionResponse {
    /// Creates a response for a newly accepted request.
    pub fn pending(
        request_id: ExecutionRequestId,
        backend_id: ExecutionBackendId,
        requested_shots: u64,
    ) -> Result<Self, ResponseError> {
        let statistics = ExecutionStatistics::new(requested_shots)?;

        let response = Self {
            schema_version: EXECUTION_RESPONSE_SCHEMA_VERSION,
            schema_id: EXECUTION_RESPONSE_SCHEMA_ID.to_owned(),
            request_id,
            backend_id,
            provider_execution_id: None,
            status: ExecutionResponseStatus::Pending,
            statistics,
            observations: Vec::new(),
            timing: ExecutionTiming::zero(),
            backend: None,
            diagnostics: Vec::new(),
            metadata: BTreeMap::new(),
            started_at_ms: None,
            completed_at_ms: None,
            fingerprint: String::new(),
        };

        response.validate()?;

        Ok(response)
    }

    /// Creates a completed response.
    pub fn completed(
        request_id: ExecutionRequestId,
        backend_id: ExecutionBackendId,
        statistics: ExecutionStatistics,
        observations: Vec<Observation>,
        timing: ExecutionTiming,
    ) -> Result<Self, ResponseError> {
        let mut response = Self {
            schema_version: EXECUTION_RESPONSE_SCHEMA_VERSION,
            schema_id: EXECUTION_RESPONSE_SCHEMA_ID.to_owned(),
            request_id,
            backend_id,
            provider_execution_id: None,
            status: ExecutionResponseStatus::Completed,
            statistics,
            observations,
            timing,
            backend: None,
            diagnostics: Vec::new(),
            metadata: BTreeMap::new(),
            started_at_ms: None,
            completed_at_ms: None,
            fingerprint: String::new(),
        };

        response.finalize()
    }

    /// Creates a response representing partial execution.
    pub fn partially_completed(
        request_id: ExecutionRequestId,
        backend_id: ExecutionBackendId,
        statistics: ExecutionStatistics,
        observations: Vec<Observation>,
        timing: ExecutionTiming,
    ) -> Result<Self, ResponseError> {
        let mut response = Self {
            schema_version: EXECUTION_RESPONSE_SCHEMA_VERSION,
            schema_id: EXECUTION_RESPONSE_SCHEMA_ID.to_owned(),
            request_id,
            backend_id,
            provider_execution_id: None,
            status: ExecutionResponseStatus::PartiallyCompleted,
            statistics,
            observations,
            timing,
            backend: None,
            diagnostics: Vec::new(),
            metadata: BTreeMap::new(),
            started_at_ms: None,
            completed_at_ms: None,
            fingerprint: String::new(),
        };

        response.finalize()
    }

    /// Creates a failed response with no observations.
    pub fn failed(
        request_id: ExecutionRequestId,
        backend_id: ExecutionBackendId,
        requested_shots: u64,
        diagnostic: ExecutionDiagnostic,
    ) -> Result<Self, ResponseError> {
        let mut statistics = ExecutionStatistics::new(requested_shots)?;
        statistics.attempts = 1;

        let mut response = Self {
            schema_version: EXECUTION_RESPONSE_SCHEMA_VERSION,
            schema_id: EXECUTION_RESPONSE_SCHEMA_ID.to_owned(),
            request_id,
            backend_id,
            provider_execution_id: None,
            status: ExecutionResponseStatus::Failed,
            statistics,
            observations: Vec::new(),
            timing: ExecutionTiming::zero(),
            backend: None,
            diagnostics: vec![diagnostic],
            metadata: BTreeMap::new(),
            started_at_ms: None,
            completed_at_ms: None,
            fingerprint: String::new(),
        };

        response.finalize()
    }

    /// Returns whether the response represents a successful execution.
    ///
    /// This means execution completed. It does NOT mean the benchmark's
    /// scientific acceptance criterion passed.
    #[must_use]
    pub const fn execution_succeeded(&self) -> bool {
        matches!(
            self.status,
            ExecutionResponseStatus::Completed
        )
    }

    /// Returns whether some observations were produced.
    #[must_use]
    pub fn has_observations(&self) -> bool {
        !self.observations.is_empty()
    }

    /// Returns whether the response is partial.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.status.is_partial()
    }

    /// Returns whether the response is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Returns the number of retained normalized observations.
    #[must_use]
    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    /// Adds a normalized observation.
    ///
    /// The response is marked as needing re-finalization after mutation.
    pub fn push_observation(
        &mut self,
        observation: Observation,
        limits: &BenchmarkLimits,
    ) -> Result<(), ResponseError> {
        if self.status.is_terminal()
            && matches!(
                self.status,
                ExecutionResponseStatus::Completed
            )
        {
            return Err(ResponseError::ImmutableCompletedResponse);
        }

        let next_count = self
            .observations
            .len()
            .checked_add(1)
            .ok_or(ResponseError::ObservationCountOverflow)?;

        if next_count > MAX_RESPONSE_OBSERVATIONS {
            return Err(ResponseError::ObservationLimitExceeded {
                requested: next_count as u64,
                maximum: MAX_RESPONSE_OBSERVATIONS as u64,
            });
        }

        limits.check_observations(
            u64::try_from(next_count).map_err(|_| {
                ResponseError::ObservationCountOverflow
            })?,
        )?;

        self.observations.push(observation);

        self.fingerprint.clear();

        Ok(())
    }

    /// Adds a structured diagnostic.
    pub fn push_diagnostic(
        &mut self,
        diagnostic: ExecutionDiagnostic,
        limits: &BenchmarkLimits,
    ) -> Result<(), ResponseError> {
        diagnostic.validate()?;

        let next_count = self
            .diagnostics
            .len()
            .checked_add(1)
            .ok_or(ResponseError::DiagnosticCountOverflow)?;

        if next_count > MAX_RESPONSE_DIAGNOSTICS {
            return Err(ResponseError::DiagnosticLimitExceeded {
                requested: next_count as u64,
                maximum: MAX_RESPONSE_DIAGNOSTICS as u64,
            });
        }

        limits.check_diagnostics(next_count)?;

        self.diagnostics.push(diagnostic);

        self.fingerprint.clear();

        Ok(())
    }

    /// Adds response metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
        limits: &BenchmarkLimits,
    ) -> Result<(), ResponseError> {
        insert_metadata(
            &mut self.metadata,
            key.into(),
            value.into(),
        )?;

        if self.metadata.len() > MAX_RESPONSE_METADATA_FIELDS {
            return Err(ResponseError::MetadataLimitExceeded {
                requested: self.metadata.len() as u64,
                maximum: MAX_RESPONSE_METADATA_FIELDS as u64,
            });
        }

        limits.check_diagnostics(self.metadata.len())?;

        self.fingerprint.clear();

        Ok(())
    }

    /// Finalizes the response and calculates its deterministic fingerprint.
    pub fn finalize(
        &mut self,
    ) -> Result<Self, ResponseError> {
        self.validate_without_fingerprint()?;

        self.fingerprint = self.calculate_fingerprint()?;

        self.validate()?;

        Ok(self.clone())
    }

    /// Validates the complete response, including its fingerprint.
    pub fn validate(&self) -> Result<(), ResponseError> {
        self.validate_without_fingerprint()?;

        if self.fingerprint.is_empty() {
            return Err(ResponseError::MissingFingerprint);
        }

        if self.fingerprint.len() > MAX_FINGERPRINT_LENGTH {
            return Err(ResponseError::FingerprintTooLong);
        }

        let expected = self.calculate_fingerprint()?;

        if self.fingerprint != expected {
            return Err(ResponseError::FingerprintMismatch);
        }

        Ok(())
    }

    /// Validates structural response state without requiring a fingerprint.
    ///
    /// This is useful while an executor is progressively constructing a
    /// response.
    pub fn validate_without_fingerprint(
        &self,
    ) -> Result<(), ResponseError> {
        if self.schema_version != EXECUTION_RESPONSE_SCHEMA_VERSION {
            return Err(ResponseError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if self.schema_id != EXECUTION_RESPONSE_SCHEMA_ID {
            return Err(ResponseError::InvalidSchemaId);
        }

        if self.request_id.as_str().trim().is_empty() {
            return Err(ResponseError::InvalidRequestId);
        }

        if self.backend_id.as_str().trim().is_empty() {
            return Err(ResponseError::InvalidBackendId);
        }

        self.statistics.validate()?;
        self.timing.validate()?;

        if self.observations.len() > MAX_RESPONSE_OBSERVATIONS {
            return Err(ResponseError::ObservationLimitExceeded {
                requested: self.observations.len() as u64,
                maximum: MAX_RESPONSE_OBSERVATIONS as u64,
            });
        }

        if self.diagnostics.len() > MAX_RESPONSE_DIAGNOSTICS {
            return Err(ResponseError::DiagnosticLimitExceeded {
                requested: self.diagnostics.len() as u64,
                maximum: MAX_RESPONSE_DIAGNOSTICS as u64,
            });
        }

        if self.metadata.len() > MAX_RESPONSE_METADATA_FIELDS {
            return Err(ResponseError::MetadataLimitExceeded {
                requested: self.metadata.len() as u64,
                maximum: MAX_RESPONSE_METADATA_FIELDS as u64,
            });
        }

        for diagnostic in &self.diagnostics {
            diagnostic.validate()?;
        }

        validate_metadata_map(&self.metadata)?;

        if let Some(provider_execution_id) =
            &self.provider_execution_id
        {
            validate_bounded_non_empty(
                "provider_execution_id",
                provider_execution_id,
                MAX_PROVIDER_EXECUTION_ID_LENGTH,
            )?;
        }

        if let Some(started) = self.started_at_ms {
            if let Some(completed) = self.completed_at_ms {
                if completed < started {
                    return Err(ResponseError::TimestampOrderInvalid);
                }
            }
        }

        self.validate_status_invariants()?;

        if let Some(backend) = &self.backend {
            backend.validate()?;
        }

        Ok(())
    }

    /// Calculates the response fingerprint.
    ///
    /// The fingerprint excludes the fingerprint field itself.
    pub fn calculate_fingerprint(
        &self,
    ) -> Result<String, ResponseError> {
        let material = FingerprintMaterial::from_response(self);

        let serialized =
            serde_json::to_vec(&material)
                .map_err(|_| ResponseError::FingerprintSerialization)?;

        let digest = Sha256::digest(&serialized);

        Ok(hex_encode_lower(&digest))
    }

    /// Returns the fingerprint.
    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns the normalized observations.
    #[must_use]
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }

    /// Returns the execution diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[ExecutionDiagnostic] {
        &self.diagnostics
    }

    /// Returns backend metadata, if available.
    #[must_use]
    pub fn backend_metadata(
        &self,
    ) -> Option<&BackendExecutionMetadata> {
        self.backend.as_ref()
    }

    /// Returns execution timing.
    #[must_use]
    pub const fn timing(&self) -> ExecutionTiming {
        self.timing
    }

    fn validate_status_invariants(
        &self,
    ) -> Result<(), ResponseError> {
        match self.status {
            ExecutionResponseStatus::Pending => {
                if self.statistics.attempts != 0 {
                    return Err(
                        ResponseError::InvalidPendingState,
                    );
                }

                if self.statistics.completed_shots != 0 {
                    return Err(
                        ResponseError::InvalidPendingState,
                    );
                }

                if !self.observations.is_empty() {
                    return Err(
                        ResponseError::InvalidPendingState,
                    );
                }
            }

            ExecutionResponseStatus::Running => {
                if self.statistics.attempts == 0 {
                    return Err(
                        ResponseError::InvalidRunningState,
                    );
                }
            }

            ExecutionResponseStatus::Completed => {
                if !self.statistics.is_complete() {
                    return Err(
                        ResponseError::InvalidCompletedState,
                    );
                }

                if self.statistics.failed_shots != 0 {
                    return Err(
                        ResponseError::InvalidCompletedState,
                    );
                }

                if self.statistics.attempts == 0 {
                    return Err(
                        ResponseError::InvalidCompletedState,
                    );
                }
            }

            ExecutionResponseStatus::PartiallyCompleted => {
                if self.statistics.completed_shots == 0 {
                    return Err(
                        ResponseError::InvalidPartialState,
                    );
                }

                if self.statistics.is_complete() {
                    return Err(
                        ResponseError::InvalidPartialState,
                    );
                }
            }

            ExecutionResponseStatus::Cancelled => {
                if self.statistics.is_complete() {
                    return Err(
                        ResponseError::InvalidCancelledState,
                    );
                }
            }

            ExecutionResponseStatus::Failed => {
                if self.statistics.is_complete() {
                    return Err(
                        ResponseError::InvalidFailedState,
                    );
                }
            }

            ExecutionResponseStatus::TimedOut => {
                if self.statistics.is_complete() {
                    return Err(
                        ResponseError::InvalidTimedOutState,
                    );
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Fingerprint material
// =============================================================================

/// Canonical fingerprint input.
///
/// Keeping this separate from `ExecutionResponse` prevents the fingerprint
/// field from recursively influencing its own hash.
#[derive(Serialize)]
struct FingerprintMaterial<'a> {
    schema_version: u16,
    schema_id: &'a str,
    request_id: &'a ExecutionRequestId,
    backend_id: &'a ExecutionBackendId,
    provider_execution_id: &'a Option<String>,
    status: ExecutionResponseStatus,
    statistics: ExecutionStatistics,
    observations: &'a [Observation],
    timing: ExecutionTiming,
    backend: &'a Option<BackendExecutionMetadata>,
    diagnostics: &'a [ExecutionDiagnostic],
    metadata: &'a BTreeMap<String, String>,
    started_at_ms: &'a Option<u64>,
    completed_at_ms: &'a Option<u64>,
}

impl<'a> FingerprintMaterial<'a> {
    fn from_response(
        response: &'a ExecutionResponse,
    ) -> Self {
        Self {
            schema_version: response.schema_version,
            schema_id: response.schema_id.as_str(),
            request_id: &response.request_id,
            backend_id: &response.backend_id,
            provider_execution_id:
                &response.provider_execution_id,
            status: response.status,
            statistics: response.statistics,
            observations: &response.observations,
            timing: response.timing,
            backend: &response.backend,
            diagnostics: &response.diagnostics,
            metadata: &response.metadata,
            started_at_ms: &response.started_at_ms,
            completed_at_ms: &response.completed_at_ms,
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the execution-response boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseError {
    UnsupportedSchemaVersion {
        version: u16,
    },

    InvalidSchemaId,

    InvalidRequestId,

    InvalidBackendId,

    InvalidShotCount,

    ShotCountInconsistent,

    ShotCountOverflow,

    AttemptCountInconsistent,

    ObservationCountOverflow,

    ObservationLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    DiagnosticCountOverflow,

    DiagnosticLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    MetadataLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    InvalidMetadataKey,

    InvalidMetadataValue,

    MetadataKeyTooLong,

    MetadataValueTooLong,

    TooManyMetadataFields,

    MetadataSizeOverflow,

    InvalidDiagnostic {
        field: &'static str,
        reason: &'static str,
    },

    ProviderExecutionIdTooLong,

    TimestampOrderInvalid,

    TimingOverflow,

    InvalidPendingState,

    InvalidRunningState,

    InvalidCompletedState,

    InvalidPartialState,

    InvalidCancelledState,

    InvalidFailedState,

    InvalidTimedOutState,

    MissingFingerprint,

    FingerprintTooLong,

    FingerprintMismatch,

    FingerprintSerialization,

    ImmutableCompletedResponse,

    Limit(LimitError),
}

impl fmt::Display for ResponseError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported execution-response schema version: {version}"
                )
            }

            Self::InvalidSchemaId => {
                formatter.write_str(
                    "invalid execution-response schema identifier",
                )
            }

            Self::InvalidRequestId => {
                formatter.write_str(
                    "execution response contains an invalid request ID",
                )
            }

            Self::InvalidBackendId => {
                formatter.write_str(
                    "execution response contains an invalid backend ID",
                )
            }

            Self::InvalidShotCount => {
                formatter.write_str(
                    "execution response must contain at least one requested shot",
                )
            }

            Self::ShotCountInconsistent => {
                formatter.write_str(
                    "execution shot counters are inconsistent",
                )
            }

            Self::ShotCountOverflow => {
                formatter.write_str(
                    "execution shot counter overflowed",
                )
            }

            Self::AttemptCountInconsistent => {
                formatter.write_str(
                    "execution attempt counters are inconsistent",
                )
            }

            Self::ObservationCountOverflow => {
                formatter.write_str(
                    "execution observation count overflowed",
                )
            }

            Self::ObservationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "execution response contains {requested} observations; maximum is {maximum}"
                )
            }

            Self::DiagnosticCountOverflow => {
                formatter.write_str(
                    "execution diagnostic count overflowed",
                )
            }

            Self::DiagnosticLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "execution response contains {requested} diagnostics; maximum is {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "execution response contains {requested} metadata fields; maximum is {maximum}"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str(
                    "execution response metadata key is invalid",
                )
            }

            Self::InvalidMetadataValue => {
                formatter.write_str(
                    "execution response metadata value is invalid",
                )
            }

            Self::MetadataKeyTooLong => {
                formatter.write_str(
                    "execution response metadata key is too long",
                )
            }

            Self::MetadataValueTooLong => {
                formatter.write_str(
                    "execution response metadata value is too long",
                )
            }

            Self::TooManyMetadataFields => {
                formatter.write_str(
                    "execution response contains too many metadata fields",
                )
            }

            Self::MetadataSizeOverflow => {
                formatter.write_str(
                    "execution response metadata size overflowed",
                )
            }

            Self::InvalidDiagnostic { field, reason } => {
                write!(
                    formatter,
                    "invalid execution diagnostic {field}: {reason}"
                )
            }

            Self::ProviderExecutionIdTooLong => {
                formatter.write_str(
                    "provider execution ID is too long",
                )
            }

            Self::TimestampOrderInvalid => {
                formatter.write_str(
                    "execution response timestamps are out of order",
                )
            }

            Self::TimingOverflow => {
                formatter.write_str(
                    "execution timing arithmetic overflowed",
                )
            }

            Self::InvalidPendingState => {
                formatter.write_str(
                    "pending response contains execution work",
                )
            }

            Self::InvalidRunningState => {
                formatter.write_str(
                    "running response must contain at least one attempt",
                )
            }

            Self::InvalidCompletedState => {
                formatter.write_str(
                    "completed response does not contain a complete execution",
                )
            }

            Self::InvalidPartialState => {
                formatter.write_str(
                    "partially completed response has invalid shot counters",
                )
            }

            Self::InvalidCancelledState => {
                formatter.write_str(
                    "cancelled response cannot contain complete execution",
                )
            }

            Self::InvalidFailedState => {
                formatter.write_str(
                    "failed response cannot contain complete execution",
                )
            }

            Self::InvalidTimedOutState => {
                formatter.write_str(
                    "timed-out response cannot contain complete execution",
                )
            }

            Self::MissingFingerprint => {
                formatter.write_str(
                    "execution response has not been finalized",
                )
            }

            Self::FingerprintTooLong => {
                formatter.write_str(
                    "execution response fingerprint is too long",
                )
            }

            Self::FingerprintMismatch => {
                formatter.write_str(
                    "execution response fingerprint does not match its payload",
                )
            }

            Self::FingerprintSerialization => {
                formatter.write_str(
                    "execution response could not be serialized for fingerprinting",
                )
            }

            Self::ImmutableCompletedResponse => {
                formatter.write_str(
                    "a completed execution response cannot be mutated",
                )
            }

            Self::Limit(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ResponseError {}

impl From<LimitError> for ResponseError {
    fn from(error: LimitError) -> Self {
        Self::Limit(error)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ResponseError> {
    if value.trim().is_empty() {
        return Err(ResponseError::InvalidDiagnostic {
            field,
            reason: "identifier cannot be empty",
        });
    }

    if value.len() > maximum {
        return Err(ResponseError::InvalidDiagnostic {
            field,
            reason: "identifier exceeds maximum length",
        });
    }

    Ok(())
}

fn validate_bounded_non_empty(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), ResponseError> {
    if value.trim().is_empty() {
        return Err(ResponseError::InvalidDiagnostic {
            field,
            reason: "value cannot be empty",
        });
    }

    if value.len() > maximum {
        return Err(ResponseError::InvalidDiagnostic {
            field,
            reason: "value exceeds maximum length",
        });
    }

    Ok(())
}

fn validate_metadata_map(
    metadata: &BTreeMap<String, String>,
) -> Result<(), ResponseError> {
    if metadata.len() > MAX_RESPONSE_METADATA_FIELDS {
        return Err(ResponseError::TooManyMetadataFields);
    }

    let mut total_bytes = 0usize;

    for (key, value) in metadata {
        if key.trim().is_empty() {
            return Err(ResponseError::InvalidMetadataKey);
        }

        if value.trim().is_empty() {
            return Err(ResponseError::InvalidMetadataValue);
        }

        if key.len() > MAX_METADATA_KEY_LENGTH {
            return Err(ResponseError::MetadataKeyTooLong);
        }

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(ResponseError::MetadataValueTooLong);
        }

        total_bytes = total_bytes
            .checked_add(key.len())
            .and_then(|value_size| {
                value_size.checked_add(value.len())
            })
            .ok_or(ResponseError::MetadataSizeOverflow)?;
    }

    Ok(())
}

fn insert_metadata(
    metadata: &mut BTreeMap<String, String>,
    key: String,
    value: String,
) -> Result<(), ResponseError> {
    if key.trim().is_empty() {
        return Err(ResponseError::InvalidMetadataKey);
    }

    if value.trim().is_empty() {
        return Err(ResponseError::InvalidMetadataValue);
    }

    if key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(ResponseError::MetadataKeyTooLong);
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(ResponseError::MetadataValueTooLong);
    }

    if !metadata.contains_key(&key)
        && metadata.len() >= MAX_RESPONSE_METADATA_FIELDS
    {
        return Err(ResponseError::TooManyMetadataFields);
    }

    metadata.insert(key, value);

    Ok(())
}

/// Converts a SHA-256 digest into lower-case hexadecimal without requiring
/// another encoding dependency.
fn hex_encode_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request_id() -> ExecutionRequestId {
        ExecutionRequestId::new("benchmark-execution-001")
            .expect("valid request ID")
    }

    fn backend_id() -> ExecutionBackendId {
        ExecutionBackendId::new("local.simulator")
            .expect("valid backend ID")
    }

    fn statistics_complete() -> ExecutionStatistics {
        ExecutionStatistics {
            requested_shots: 100,
            completed_shots: 100,
            failed_shots: 0,
            attempts: 1,
            retries: 0,
        }
    }

    #[test]
    fn timing_zero_is_valid() {
        let timing = ExecutionTiming::zero();

        assert_eq!(timing.total_time, Duration::ZERO);
        assert!(timing.validate().is_ok());
    }

    #[test]
    fn statistics_reject_completed_shots_above_requested() {
        let statistics = ExecutionStatistics {
            requested_shots: 10,
            completed_shots: 11,
            failed_shots: 0,
            attempts: 1,
            retries: 0,
        };

        assert_eq!(
            statistics.validate(),
            Err(ResponseError::ShotCountInconsistent)
        );
    }

    #[test]
    fn statistics_reject_retries_above_attempts() {
        let statistics = ExecutionStatistics {
            requested_shots: 10,
            completed_shots: 0,
            failed_shots: 0,
            attempts: 1,
            retries: 2,
        };

        assert_eq!(
            statistics.validate(),
            Err(ResponseError::AttemptCountInconsistent)
        );
    }

    #[test]
    fn pending_response_has_no_attempts() {
        let response = ExecutionResponse::pending(
            request_id(),
            backend_id(),
            100,
        )
        .expect("valid pending response");

        assert_eq!(
            response.status,
            ExecutionResponseStatus::Pending
        );

        assert!(response.fingerprint.is_empty());
    }

    #[test]
    fn completed_response_requires_complete_statistics() {
        let result = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        );

        assert!(result.is_ok());

        let response = result.expect("valid response");

        assert_eq!(
            response.status,
            ExecutionResponseStatus::Completed
        );

        assert_eq!(response.fingerprint.len(), 64);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn completed_response_fingerprint_is_deterministic() {
        let first = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        )
        .expect("valid response");

        let second = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        )
        .expect("valid response");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn fingerprint_detects_mutation() {
        let mut response = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        )
        .expect("valid response");

        response
            .metadata
            .insert(
                "provider".to_owned(),
                "test".to_owned(),
            );

        assert_eq!(
            response.validate(),
            Err(ResponseError::FingerprintMismatch)
        );
    }

    #[test]
    fn timestamp_order_is_checked() {
        let mut response = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        )
        .expect("valid response");

        response.started_at_ms = Some(2_000);
        response.completed_at_ms = Some(1_000);

        assert_eq!(
            response.validate(),
            Err(ResponseError::TimestampOrderInvalid)
        );
    }

    #[test]
    fn diagnostic_validation_rejects_empty_message() {
        let result = ExecutionDiagnostic::new(
            ExecutionDiagnosticSeverity::Error,
            "EXECUTION_FAILED",
            "",
        );

        assert!(result.is_err());
    }

    #[test]
    fn backend_metadata_is_deterministic() {
        let metadata =
            BackendExecutionMetadata::new(
                "zamani",
                "simulator",
            )
            .expect("valid metadata")
            .with_attribute(
                "implementation",
                "reference",
            )
            .expect("valid attribute");

        assert_eq!(
            metadata.attributes.get("implementation"),
            Some(&"reference".to_owned())
        );
    }

    #[test]
    fn phase_sum_is_checked() {
        let timing = ExecutionTiming {
            queue_time: Duration::from_millis(10),
            submission_time: Duration::from_millis(20),
            execution_time: Duration::from_millis(30),
            result_transfer_time: Duration::from_millis(40),
            normalization_time: Duration::from_millis(50),
            total_time: Duration::from_millis(150),
        };

        assert_eq!(
            timing.phase_sum().expect("valid timing"),
            Duration::from_millis(150)
        );
    }

    #[test]
    fn failed_response_contains_error_diagnostic() {
        let diagnostic = ExecutionDiagnostic::new(
            ExecutionDiagnosticSeverity::Error,
            "BACKEND_FAILURE",
            "backend execution failed",
        )
        .expect("valid diagnostic");

        let response = ExecutionResponse::failed(
            request_id(),
            backend_id(),
            100,
            diagnostic,
        )
        .expect("valid failed response");

        assert_eq!(
            response.status,
            ExecutionResponseStatus::Failed
        );

        assert_eq!(response.diagnostics.len(), 1);
        assert!(response.validate().is_ok());
    }

    #[test]
    fn completed_response_cannot_be_mutated_after_completion() {
        let mut response = ExecutionResponse::completed(
            request_id(),
            backend_id(),
            statistics_complete(),
            Vec::new(),
            ExecutionTiming::zero(),
        )
        .expect("valid response");

        let result = response.push_diagnostic(
            ExecutionDiagnostic::new(
                ExecutionDiagnosticSeverity::Warning,
                "TEST",
                "test",
            )
            .expect("valid diagnostic"),
            &BenchmarkLimits::default(),
        );

        assert_eq!(
            result,
            Err(ResponseError::ImmutableCompletedResponse)
        );
    }
}