//! Zamani Quantum Hardware — Production Telemetry
//!
//! Provider-independent telemetry, metrics, observations, and execution
//! provenance for the quantum hardware abstraction layer.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - provider-neutral hardware telemetry;
//! - execution timing measurements;
//! - submission/queue/execution/result latency metrics;
//! - success/failure/cancellation counters;
//! - retry counters;
//! - backend availability observations;
//! - calibration-age observations;
//! - bounded metric labels;
//! - deterministic metric snapshots;
//! - bounded in-process metric aggregation;
//! - histogram aggregation;
//! - quantile estimation from bounded histograms;
//! - telemetry event classification;
//! - provider-error classification;
//! - telemetry health metadata;
//! - telemetry configuration;
//! - telemetry redaction and secret rejection;
//! - explicit timestamp handling;
//! - telemetry schema versioning;
//! - deterministic serialization;
//! - bounded memory behavior;
//! - thread-safe metric recording;
//! - immutable snapshot generation;
//! - integration contracts for backend, execution, job, queue,
//!   provider, health, benchmarking, and Danga.
//!
//! It deliberately does NOT own:
//!
//! - provider networking;
//! - HTTP clients;
//! - authentication;
//! - credentials;
//! - API keys;
//! - provider SDKs;
//! - backend discovery;
//! - backend capability negotiation;
//! - topology;
//! - calibration acquisition;
//! - queue scheduling;
//! - job lifecycle;
//! - execution;
//! - result retrieval;
//! - routing;
//! - scheduling;
//! - benchmarking mathematics;
//! - error-correction algorithms;
//! - simulation;
//! - emulation.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! Compatibility / Routing / Scheduling
//!        |
//!        v
//! Execution
//!        |
//!        +-------------------------+
//!        |                         |
//!        v                         v
//!      Job                     Backend
//!        |                         |
//!        v                         v
//!      Queue                   Provider
//!        |                         |
//!        +------------+------------+
//!                     |
//!                     v
//!                Telemetry
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!       Counters  Histograms  Events
//!          |          |          |
//!          +----------+----------+
//!                     |
//!                     v
//!              TelemetrySnapshot
//!                     |
//!          +----------+----------+
//!          |                     |
//!          v                     v
//!     Benchmarking             Danga
//! ```
//!
//! # Fundamental rule
//!
//! Telemetry is observational.
//!
//! It MUST NOT alter quantum program semantics, execution behavior, backend
//! selection, routing decisions, scheduling decisions, or result values.
//!
//! Telemetry failures MUST NOT silently corrupt quantum execution.
//!
//! The recommended production policy is:
//!
//! ```text
//! execution success
//!       |
//!       +---- telemetry success -> retain telemetry
//!       |
//!       +---- telemetry failure -> report telemetry failure separately
//!                              |
//!                              v
//!                         preserve execution result
//! ```
//!
//! A caller MAY choose to fail closed for compliance-sensitive environments,
//! but that policy belongs to the caller/configuration and is never implicit.
//!
//! # Integration contract
//!
//! This file intentionally has no dependency on other Zamani hardware files.
//!
//! It uses stable provider-neutral identifiers represented as bounded strings.
//! This avoids dependency cycles while allowing later modules to integrate
//! without reopening this file.
//!
//! Later modules consume this contract as follows:
//!
//! - `backend.rs` records backend-level observations;
//! - `backend_status.rs` supplies status observations;
//! - `execution.rs` records submission/execution/result timing;
//! - `job.rs` records lifecycle transitions;
//! - `queue.rs` records queue depth and queue latency;
//! - `health.rs` records health-check observations;
//! - `provider.rs` records provider-level observations;
//! - adapters record provider error classifications;
//! - `calibration.rs` supplies calibration age metadata;
//! - benchmarking consumes immutable telemetry snapshots;
//! - Danga exposes telemetry inspection;
//! - serialization can serialize `TelemetrySnapshot`;
//! - future exporters can consume snapshots without changing the core model.
//!
//! None of those modules should redefine:
//!
//! - metric names;
//! - telemetry event categories;
//! - telemetry error classes;
//! - histogram semantics;
//! - label validation;
//! - secret rejection rules;
//! - snapshot structure.
//!
//! # Integration rule
//!
//! The canonical path is:
//!
//! ```text
//! crate::quantum::hardware::telemetry
//! ```
//!
//! Consumers should import the public types from this module rather than
//! creating provider-specific telemetry structures.
//!
//! # Provider independence
//!
//! Provider-specific error codes may be retained only as bounded opaque
//! identifiers after sanitization. Provider names and codes MUST NOT change
//! the core telemetry taxonomy.
//!
//! For example:
//!
//! ```text
//! provider = "ibm"
//! provider_error_code = "some_provider_code"
//! classification = "rate_limited"
//! retryable = true
//! ```
//!
//! The classification remains Zamani-owned.
//!
//! # Cardinality protection
//!
//! Telemetry is a production subsystem and must not allow arbitrary user data
//! to become metric labels.
//!
//! The implementation therefore:
//!
//! - bounds label count;
//! - bounds label key length;
//! - bounds label value length;
//! - rejects secret-looking labels;
//! - rejects control characters;
//! - rejects empty labels;
//! - uses deterministic `BTreeMap` ordering;
//! - does not automatically include arbitrary request metadata;
//! - does not automatically include quantum-program source;
//! - does not automatically include result payloads;
//! - does not automatically include authentication data.
//!
//! Job IDs and request IDs may be recorded as observation context, but they
//! should normally NOT be used as metric labels because they have extremely
//! high cardinality.
//!
//! # Timing
//!
//! This module does not query the wall clock.
//!
//! Callers provide timestamps and durations explicitly.
//!
//! For local elapsed-time measurement, callers should use `std::time::Instant`
//! and convert the measured duration into `DurationMicros` before recording.
//!
//! This design preserves deterministic replay and allows provider timestamps
//! to be retained without conflating local and provider clocks.
//!
//! # Numeric safety
//!
//! Telemetry rejects:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative durations;
//! - negative queue depth;
//! - values outside configured bounds;
//! - impossible percentage values.
//!
//! Counters saturate rather than wrapping.
//!
//! # Thread safety
//!
//! `Telemetry` is safe to share between threads.
//!
//! Internal state is protected by `std::sync::Mutex`.
//!
//! Lock poisoning is surfaced as an explicit error rather than silently
//! recovering potentially inconsistent telemetry state.
//!
//! # Memory safety
//!
//! Telemetry is bounded.
//!
//! It never stores an unbounded event stream.
//!
//! The implementation stores aggregate metrics and a bounded ring of recent
//! events. Applications requiring durable event storage should export
//! snapshots/events to a separate persistence or observability subsystem.
//!
//! # Security
//!
//! Telemetry MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - TLS credentials;
//! - provider session tokens;
//! - quantum program source;
//! - raw secret input data.
//!
//! Secret-looking metadata keys are rejected rather than merely redacted.
//!
//! This is intentional: rejecting dangerous telemetry input makes accidental
//! secret leakage easier to detect during development and production.
//!
//! # Serialization
//!
//! Public telemetry models derive Serde serialization.
//!
//! Serialized snapshots contain only explicitly supplied telemetry fields.
//!
//! No credentials are ever serialized.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Stability
//!
//! Stable concepts in this module are:
//!
//! - `Telemetry`;
//! - `TelemetryConfig`;
//! - `TelemetrySnapshot`;
//! - `MetricName`;
//! - `MetricKind`;
//! - `TelemetryEvent`;
//! - `TelemetryEventKind`;
//! - `TelemetryErrorClass`;
//! - `MetricLabels`;
//! - `HistogramSnapshot`;
//! - `CounterSnapshot`;
//! - `TelemetryHealth`;
//! - `TelemetryError`.
//!
//! New exporters and provider adapters must consume these contracts instead
//! of changing their meaning.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

// =============================================================================
// Schema
// =============================================================================

/// Stable telemetry schema identifier.
pub const TELEMETRY_SCHEMA_ID: &str = "zamani.quantum.hardware.telemetry";

/// Semantic version of the telemetry contract.
pub const TELEMETRY_SCHEMA_VERSION: u16 = 1;

/// Maximum metric-name length.
pub const MAX_METRIC_NAME_LENGTH: usize = 256;

/// Maximum metric-label key length.
pub const MAX_LABEL_KEY_LENGTH: usize = 128;

/// Maximum metric-label value length.
pub const MAX_LABEL_VALUE_LENGTH: usize = 512;

/// Maximum number of labels on a metric.
pub const MAX_LABELS: usize = 32;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 256;

/// Maximum backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum device identifier length.
pub const MAX_DEVICE_ID_LENGTH: usize = 512;

/// Maximum job identifier length.
pub const MAX_JOB_ID_LENGTH: usize = 512;

/// Maximum request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

/// Maximum provider error-code length.
pub const MAX_PROVIDER_ERROR_CODE_LENGTH: usize = 256;

/// Maximum telemetry event message length.
pub const MAX_EVENT_MESSAGE_LENGTH: usize = 2048;

/// Maximum number of recent events retained in memory.
pub const DEFAULT_MAX_RECENT_EVENTS: usize = 1024;

/// Maximum number of configured histogram buckets.
pub const MAX_HISTOGRAM_BUCKETS: usize = 64;

/// Maximum histogram observation value in microseconds.
pub const MAX_DURATION_MICROS: u64 = 86_400_000_000;

/// Maximum queue depth.
pub const MAX_QUEUE_DEPTH: u64 = 1_000_000_000;

/// Maximum calibration age in seconds representable by this model.
pub const MAX_CALIBRATION_AGE_SECONDS: u64 = 31_536_000_000;

/// Maximum percentage.
pub const MAX_PERCENTAGE: f64 = 100.0;

// =============================================================================
// Metric names
// =============================================================================

/// Stable metric name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MetricName(String);

impl MetricName {
    /// Creates a validated metric name.
    pub fn new(value: impl Into<String>) -> Result<Self, TelemetryError> {
        let value = value.into();

        validate_identifier(
            &value,
            "metric_name",
            MAX_METRIC_NAME_LENGTH,
        )?;

        if value
            .chars()
            .any(|character| !(character.is_ascii_alphanumeric()
                || matches!(character, '_' | '.' | ':' | '-')))
        {
            return Err(TelemetryError::InvalidMetricName {
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Standard metric names
// =============================================================================

/// Standard telemetry metric names.
///
/// These names are deliberately constants rather than free-form strings so
/// core Zamani components can share stable semantics.
pub mod metrics {
    /// Number of execution submissions.
    pub const EXECUTION_SUBMISSIONS: &str =
        "zamani.quantum.hardware.execution.submissions";

    /// Number of successfully completed executions.
    pub const EXECUTION_SUCCESSES: &str =
        "zamani.quantum.hardware.execution.successes";

    /// Number of failed executions.
    pub const EXECUTION_FAILURES: &str =
        "zamani.quantum.hardware.execution.failures";

    /// Number of cancelled executions.
    pub const EXECUTION_CANCELLATIONS: &str =
        "zamani.quantum.hardware.execution.cancellations";

    /// Number of retries.
    pub const EXECUTION_RETRIES: &str =
        "zamani.quantum.hardware.execution.retries";

    /// Submission latency.
    pub const SUBMISSION_LATENCY_MICROS: &str =
        "zamani.quantum.hardware.execution.submission_latency_us";

    /// Queue latency.
    pub const QUEUE_LATENCY_MICROS: &str =
        "zamani.quantum.hardware.execution.queue_latency_us";

    /// Execution latency.
    pub const EXECUTION_LATENCY_MICROS: &str =
        "zamani.quantum.hardware.execution.execution_latency_us";

    /// Result retrieval latency.
    pub const RESULT_LATENCY_MICROS: &str =
        "zamani.quantum.hardware.execution.result_latency_us";

    /// Total end-to-end latency.
    pub const TOTAL_LATENCY_MICROS: &str =
        "zamani.quantum.hardware.execution.total_latency_us";

    /// Queue depth.
    pub const QUEUE_DEPTH: &str =
        "zamani.quantum.hardware.queue.depth";

    /// Queue position.
    pub const QUEUE_POSITION: &str =
        "zamani.quantum.hardware.queue.position";

    /// Backend availability observations.
    pub const BACKEND_AVAILABILITY: &str =
        "zamani.quantum.hardware.backend.availability";

    /// Backend health score.
    pub const BACKEND_HEALTH_SCORE: &str =
        "zamani.quantum.hardware.backend.health_score";

    /// Calibration age.
    pub const CALIBRATION_AGE_SECONDS: &str =
        "zamani.quantum.hardware.calibration.age_seconds";

    /// Provider errors.
    pub const PROVIDER_ERRORS: &str =
        "zamani.quantum.hardware.provider.errors";

    /// Authentication failures.
    pub const AUTHENTICATION_FAILURES: &str =
        "zamani.quantum.hardware.provider.authentication_failures";

    /// Authorization failures.
    pub const AUTHORIZATION_FAILURES: &str =
        "zamani.quantum.hardware.provider.authorization_failures";

    /// Timeout count.
    pub const TIMEOUTS: &str =
        "zamani.quantum.hardware.execution.timeouts";

    /// Rate-limit count.
    pub const RATE_LIMITS: &str =
        "zamani.quantum.hardware.provider.rate_limits";

    /// Transport-failure count.
    pub const TRANSPORT_FAILURES: &str =
        "zamani.quantum.hardware.provider.transport_failures";
}

// =============================================================================
// Metric kind
// =============================================================================

/// Aggregation kind for a telemetry metric.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum MetricKind {
    /// Monotonically increasing counter.
    Counter,

    /// Distribution of observed non-negative values.
    Histogram,

    /// Current gauge-like value.
    Gauge,
}

impl MetricKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Histogram => "histogram",
            Self::Gauge => "gauge",
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Labels
// =============================================================================

/// Deterministically ordered, bounded metric labels.
///
/// Labels are intentionally separate from arbitrary telemetry metadata.
///
/// High-cardinality identifiers such as job IDs should normally remain event
/// context rather than becoming labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricLabels(BTreeMap<String, String>);

impl Default for MetricLabels {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricLabels {
    /// Creates an empty label set.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Adds a validated label.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), TelemetryError> {
        let key = key.into();
        let value = value.into();

        validate_label(&key, &value)?;

        if self.0.len() >= MAX_LABELS && !self.0.contains_key(&key) {
            return Err(TelemetryError::LabelLimitExceeded {
                maximum: MAX_LABELS,
            });
        }

        self.0.insert(key, value);
        Ok(())
    }

    /// Returns a label value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    /// Returns the number of labels.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether no labels exist.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns deterministic label iteration.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.0.iter().map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Returns the underlying deterministic map by reference.
    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}

// =============================================================================
// Counter
// =============================================================================

/// Immutable counter snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterSnapshot {
    /// Number of observations.
    pub value: u64,
}

impl CounterSnapshot {
    /// Creates a counter snapshot.
    pub const fn new(value: u64) -> Self {
        Self { value }
    }
}

// =============================================================================
// Histogram
// =============================================================================

/// A bounded histogram bucket.
///
/// `upper_bound` is inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistogramBucket {
    /// Inclusive upper bound of this bucket.
    pub upper_bound: u64,

    /// Number of observations assigned to this bucket.
    pub count: u64,
}

/// Immutable histogram snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramSnapshot {
    /// Number of observations.
    pub count: u64,

    /// Sum of all observed values.
    pub sum: u128,

    /// Minimum observed value.
    pub min: Option<u64>,

    /// Maximum observed value.
    pub max: Option<u64>,

    /// Deterministically ordered buckets.
    pub buckets: Vec<HistogramBucket>,
}

impl HistogramSnapshot {
    /// Returns the arithmetic mean when observations exist.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            return None;
        }

        Some(self.sum as f64 / self.count as f64)
    }

    /// Estimates a quantile from the bucket distribution.
    ///
    /// This is an upper-bound estimate rather than an exact raw-sample
    /// quantile. Raw samples are intentionally not retained.
    pub fn quantile(&self, quantile: f64) -> Option<u64> {
        if self.count == 0 || !quantile.is_finite() {
            return None;
        }

        let quantile = quantile.clamp(0.0, 1.0);

        if quantile == 0.0 {
            return self.min;
        }

        let target = ((self.count as f64) * quantile).ceil() as u64;

        let mut cumulative = 0_u64;

        for bucket in &self.buckets {
            cumulative = cumulative.saturating_add(bucket.count);

            if cumulative >= target {
                return Some(bucket.upper_bound);
            }
        }

        self.max
    }

    /// Estimates the median.
    pub fn median(&self) -> Option<u64> {
        self.quantile(0.5)
    }

    /// Estimates p95.
    pub fn p95(&self) -> Option<u64> {
        self.quantile(0.95)
    }

    /// Estimates p99.
    pub fn p99(&self) -> Option<u64> {
        self.quantile(0.99)
    }
}

// =============================================================================
// Telemetry event kinds
// =============================================================================

/// Provider-neutral event category.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum TelemetryEventKind {
    /// Backend discovered.
    BackendDiscovered,

    /// Backend status observed.
    BackendStatusObserved,

    /// Backend health observed.
    BackendHealthObserved,

    /// Execution submitted.
    ExecutionSubmitted,

    /// Execution started.
    ExecutionStarted,

    /// Execution completed.
    ExecutionCompleted,

    /// Execution failed.
    ExecutionFailed,

    /// Execution cancelled.
    ExecutionCancelled,

    /// Execution timed out.
    ExecutionTimedOut,

    /// Execution retried.
    ExecutionRetried,

    /// Queue observation.
    QueueObserved,

    /// Calibration observation.
    CalibrationObserved,

    /// Provider error.
    ProviderError,

    /// Authentication failure.
    AuthenticationFailure,

    /// Authorization failure.
    AuthorizationFailure,

    /// Rate limiting.
    RateLimited,

    /// Transport failure.
    TransportFailure,

    /// Telemetry itself encountered a problem.
    TelemetryFailure,
}

impl TelemetryEventKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackendDiscovered => "backend_discovered",
            Self::BackendStatusObserved => "backend_status_observed",
            Self::BackendHealthObserved => "backend_health_observed",
            Self::ExecutionSubmitted => "execution_submitted",
            Self::ExecutionStarted => "execution_started",
            Self::ExecutionCompleted => "execution_completed",
            Self::ExecutionFailed => "execution_failed",
            Self::ExecutionCancelled => "execution_cancelled",
            Self::ExecutionTimedOut => "execution_timed_out",
            Self::ExecutionRetried => "execution_retried",
            Self::QueueObserved => "queue_observed",
            Self::CalibrationObserved => "calibration_observed",
            Self::ProviderError => "provider_error",
            Self::AuthenticationFailure => "authentication_failure",
            Self::AuthorizationFailure => "authorization_failure",
            Self::RateLimited => "rate_limited",
            Self::TransportFailure => "transport_failure",
            Self::TelemetryFailure => "telemetry_failure",
        }
    }
}

impl fmt::Display for TelemetryEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provider error classification
// =============================================================================

/// Provider-independent error classification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum TelemetryErrorClass {
    /// No classification is available.
    Unknown,

    /// Authentication failed.
    Authentication,

    /// Authorization failed.
    Authorization,

    /// Request was rate limited.
    RateLimited,

    /// Provider transport failed.
    Transport,

    /// Provider rejected the request as invalid.
    InvalidRequest,

    /// Backend cannot currently execute the workload.
    BackendUnavailable,

    /// Queue operation failed.
    Queue,

    /// Execution failed after acceptance.
    Execution,

    /// Result retrieval failed.
    ResultRetrieval,

    /// Operation exceeded a configured timeout.
    Timeout,

    /// Operation was cancelled.
    Cancellation,

    /// Calibration data was invalid or unavailable.
    Calibration,

    /// Capability mismatch.
    CapabilityMismatch,

    /// Serialization/deserialization failure.
    Serialization,

    /// Telemetry infrastructure failure.
    Telemetry,

    /// Unknown provider-specific failure.
    Provider,
}

impl TelemetryErrorClass {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::RateLimited => "rate_limited",
            Self::Transport => "transport",
            Self::InvalidRequest => "invalid_request",
            Self::BackendUnavailable => "backend_unavailable",
            Self::Queue => "queue",
            Self::Execution => "execution",
            Self::ResultRetrieval => "result_retrieval",
            Self::Timeout => "timeout",
            Self::Cancellation => "cancellation",
            Self::Calibration => "calibration",
            Self::CapabilityMismatch => "capability_mismatch",
            Self::Serialization => "serialization",
            Self::Telemetry => "telemetry",
            Self::Provider => "provider",
        }
    }

    /// Conservative retry classification.
    pub const fn is_retryable(self) -> bool {
        matches!(
            self,
            Self::RateLimited
                | Self::Transport
                | Self::BackendUnavailable
                | Self::Queue
                | Self::Timeout
        )
    }
}

impl fmt::Display for TelemetryErrorClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Event
// =============================================================================

/// Bounded telemetry event.
///
/// Events are diagnostic context. Aggregate metrics should be used for
/// monitoring and alerting because event retention is intentionally bounded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetryEvent {
    /// Event category.
    pub kind: TelemetryEventKind,

    /// Caller-supplied timestamp in Unix microseconds.
    ///
    /// Zero is permitted when a caller deliberately has no wall-clock
    /// timestamp. Consumers should treat zero as "timestamp unavailable".
    pub timestamp_unix_micros: u64,

    /// Provider identifier, if known.
    pub provider_id: Option<String>,

    /// Backend identifier, if known.
    pub backend_id: Option<String>,

    /// Device identifier, if known.
    pub device_id: Option<String>,

    /// Job identifier, if known.
    ///
    /// This is event context and is deliberately not a default metric label.
    pub job_id: Option<String>,

    /// Request identifier, if known.
    pub request_id: Option<String>,

    /// Error classification, if applicable.
    pub error_class: Option<TelemetryErrorClass>,

    /// Provider error code, if applicable.
    pub provider_error_code: Option<String>,

    /// Retryability observed by the caller.
    pub retryable: Option<bool>,

    /// Human-readable sanitized message.
    pub message: Option<String>,

    /// Non-secret structured fields.
    pub fields: BTreeMap<String, String>,
}

impl TelemetryEvent {
    /// Creates an event with no contextual identifiers.
    pub fn new(
        kind: TelemetryEventKind,
        timestamp_unix_micros: u64,
    ) -> Self {
        Self {
            kind,
            timestamp_unix_micros,
            provider_id: None,
            backend_id: None,
            device_id: None,
            job_id: None,
            request_id: None,
            error_class: None,
            provider_error_code: None,
            retryable: None,
            message: None,
            fields: BTreeMap::new(),
        }
    }

    /// Sets the provider identifier.
    pub fn with_provider_id(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        self.provider_id = Some(validate_identifier(
            &provider_id.into(),
            "provider_id",
            MAX_PROVIDER_ID_LENGTH,
        )?);
        Ok(self)
    }

    /// Sets the backend identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        self.backend_id = Some(validate_identifier(
            &backend_id.into(),
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?);
        Ok(self)
    }

    /// Sets the device identifier.
    pub fn with_device_id(
        mut self,
        device_id: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        self.device_id = Some(validate_identifier(
            &device_id.into(),
            "device_id",
            MAX_DEVICE_ID_LENGTH,
        )?);
        Ok(self)
    }

    /// Sets the job identifier.
    pub fn with_job_id(
        mut self,
        job_id: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        self.job_id = Some(validate_identifier(
            &job_id.into(),
            "job_id",
            MAX_JOB_ID_LENGTH,
        )?);
        Ok(self)
    }

    /// Sets the request identifier.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        self.request_id = Some(validate_identifier(
            &request_id.into(),
            "request_id",
            MAX_REQUEST_ID_LENGTH,
        )?);
        Ok(self)
    }

    /// Adds a provider error classification.
    pub fn with_error(
        mut self,
        class: TelemetryErrorClass,
        provider_error_code: Option<String>,
        retryable: bool,
    ) -> Result<Self, TelemetryError> {
        self.error_class = Some(class);
        self.retryable = Some(retryable);

        if let Some(code) = provider_error_code {
            self.provider_error_code = Some(validate_identifier(
                &code,
                "provider_error_code",
                MAX_PROVIDER_ERROR_CODE_LENGTH,
            )?);
        }

        Ok(self)
    }

    /// Adds a bounded sanitized message.
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> Result<Self, TelemetryError> {
        let message = message.into();

        validate_text(
            &message,
            "message",
            MAX_EVENT_MESSAGE_LENGTH,
        )?;

        if contains_secret_marker(&message) {
            return Err(TelemetryError::SecretRejected {
                field: "message".to_owned(),
            });
        }

        self.message = Some(message);
        Ok(self)
    }

    /// Adds a structured non-secret field.
    pub fn insert_field(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), TelemetryError> {
        let key = key.into();
        let value = value.into();

        validate_label(&key, &value)?;

        if self.fields.len() >= MAX_LABELS && !self.fields.contains_key(&key) {
            return Err(TelemetryError::LabelLimitExceeded {
                maximum: MAX_LABELS,
            });
        }

        self.fields.insert(key, value);
        Ok(())
    }
}

// =============================================================================
// Backend availability
// =============================================================================

/// Normalized backend availability observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailabilityObservation {
    /// Backend is available.
    Available,

    /// Backend is busy.
    Busy,

    /// Backend is degraded.
    Degraded,

    /// Backend is under maintenance.
    Maintenance,

    /// Backend is unavailable.
    Unavailable,

    /// Backend is offline.
    Offline,

    /// Backend is in an error state.
    Error,

    /// Backend state is unknown.
    Unknown,

    /// Backend has retired.
    Retired,
}

impl AvailabilityObservation {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Busy => "busy",
            Self::Degraded => "degraded",
            Self::Maintenance => "maintenance",
            Self::Unavailable => "unavailable",
            Self::Offline => "offline",
            Self::Error => "error",
            Self::Unknown => "unknown",
            Self::Retired => "retired",
        }
    }

    /// Returns one for states eligible for normal submission.
    pub const fn is_submission_eligible(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }
}

// =============================================================================
// Duration helpers
// =============================================================================

/// Validated non-negative duration represented in microseconds.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct DurationMicros(u64);

impl DurationMicros {
    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a validated duration.
    pub const fn new(value: u64) -> Result<Self, TelemetryError> {
        if value > MAX_DURATION_MICROS {
            return Err(TelemetryError::ValueOutOfRange {
                field: "duration_micros",
            });
        }

        Ok(Self(value))
    }

    /// Converts a standard duration.
    pub fn from_duration(duration: Duration) -> Result<Self, TelemetryError> {
        let micros = duration.as_micros();

        if micros > MAX_DURATION_MICROS as u128 {
            return Err(TelemetryError::ValueOutOfRange {
                field: "duration_micros",
            });
        }

        Ok(Self(micros as u64))
    }

    /// Returns microseconds.
    pub const fn as_micros(self) -> u64 {
        self.0
    }

    /// Converts into `Duration`.
    pub const fn as_duration(self) -> Duration {
        Duration::from_micros(self.0)
    }
}

// =============================================================================
// Histogram configuration
// =============================================================================

/// Histogram bucket configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistogramConfig {
    /// Strictly increasing inclusive upper bounds.
    pub upper_bounds: Vec<u64>,
}

impl HistogramConfig {
    /// Creates a validated histogram configuration.
    pub fn new(
        mut upper_bounds: Vec<u64>,
    ) -> Result<Self, TelemetryError> {
        if upper_bounds.is_empty() {
            return Err(TelemetryError::InvalidHistogramConfiguration);
        }

        if upper_bounds.len() > MAX_HISTOGRAM_BUCKETS {
            return Err(TelemetryError::HistogramBucketLimitExceeded {
                maximum: MAX_HISTOGRAM_BUCKETS,
            });
        }

        upper_bounds.sort_unstable();

        if upper_bounds.windows(2).any(|window| window[0] == window[1]) {
            return Err(TelemetryError::InvalidHistogramConfiguration);
        }

        if upper_bounds
            .iter()
            .any(|value| *value > MAX_DURATION_MICROS)
        {
            return Err(TelemetryError::ValueOutOfRange {
                field: "histogram_bucket",
            });
        }

        Ok(Self { upper_bounds })
    }

    /// Standard latency buckets in microseconds.
    pub fn standard_latency() -> Self {
        // These values intentionally cover sub-millisecond through one day.
        Self {
            upper_bounds: vec![
                100,
                500,
                1_000,
                2_500,
                5_000,
                10_000,
                25_000,
                50_000,
                100_000,
                250_000,
                500_000,
                1_000_000,
                2_500_000,
                5_000_000,
                10_000_000,
                30_000_000,
                60_000_000,
                300_000_000,
                600_000_000,
                3_600_000_000,
                86_400_000_000,
            ],
        }
    }

    /// Standard queue-depth buckets.
    pub fn standard_queue_depth() -> Self {
        Self {
            upper_bounds: vec![
                0,
                1,
                2,
                5,
                10,
                25,
                50,
                100,
                250,
                500,
                1_000,
                2_500,
                5_000,
                10_000,
                25_000,
                50_000,
                100_000,
                250_000,
                500_000,
                1_000_000,
                MAX_QUEUE_DEPTH,
            ],
        }
    }
}

// =============================================================================
// Telemetry configuration
// =============================================================================

/// Configuration for in-process telemetry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// Enables telemetry recording.
    pub enabled: bool,

    /// Maximum number of recent events retained in memory.
    pub max_recent_events: usize,

    /// Latency histogram configuration.
    pub latency_histogram: HistogramConfig,

    /// Queue histogram configuration.
    pub queue_histogram: HistogramConfig,

    /// Whether event messages are retained.
    ///
    /// Setting this to false is recommended for high-security deployments.
    pub retain_event_messages: bool,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_recent_events: DEFAULT_MAX_RECENT_EVENTS,
            latency_histogram: HistogramConfig::standard_latency(),
            queue_histogram: HistogramConfig::standard_queue_depth(),
            retain_event_messages: true,
        }
    }
}

impl TelemetryConfig {
    /// Validates configuration invariants.
    pub fn validate(&self) -> Result<(), TelemetryError> {
        if self.max_recent_events > 1_000_000 {
            return Err(TelemetryError::ConfigurationLimitExceeded {
                field: "max_recent_events",
            });
        }

        if self.max_recent_events == 0 {
            // Zero is valid: callers explicitly disable event retention while
            // aggregate metrics remain enabled.
        }

        HistogramConfig::new(self.latency_histogram.upper_bounds.clone())?;
        HistogramConfig::new(self.queue_histogram.upper_bounds.clone())?;

        Ok(())
    }
}

// =============================================================================
// Internal histogram state
// =============================================================================

#[derive(Debug, Clone)]
struct HistogramState {
    config: HistogramConfig,
    counts: Vec<u64>,
    overflow: u64,
    count: u64,
    sum: u128,
    min: Option<u64>,
    max: Option<u64>,
}

impl HistogramState {
    fn new(config: HistogramConfig) -> Self {
        Self {
            counts: vec![0; config.upper_bounds.len()],
            config,
            overflow: 0,
            count: 0,
            sum: 0,
            min: None,
            max: None,
        }
    }

    fn observe(&mut self, value: u64) -> Result<(), TelemetryError> {
        if value > MAX_DURATION_MICROS {
            return Err(TelemetryError::ValueOutOfRange {
                field: "histogram_value",
            });
        }

        self.count = self.count.saturating_add(1);
        self.sum = self.sum.saturating_add(value as u128);

        self.min = Some(match self.min {
            Some(current) => current.min(value),
            None => value,
        });

        self.max = Some(match self.max {
            Some(current) => current.max(value),
            None => value,
        });

        match self
            .config
            .upper_bounds
            .binary_search_by(|bound| bound.cmp(&value))
        {
            Ok(index) => {
                self.counts[index] = self.counts[index].saturating_add(1);
            }
            Err(index) => {
                if index < self.counts.len() {
                    self.counts[index] =
                        self.counts[index].saturating_add(1);
                } else {
                    self.overflow = self.overflow.saturating_add(1);
                }
            }
        }

        Ok(())
    }

    fn snapshot(&self) -> HistogramSnapshot {
        let mut buckets = Vec::with_capacity(self.config.upper_bounds.len());

        for (index, upper_bound) in self.config.upper_bounds.iter().enumerate() {
            buckets.push(HistogramBucket {
                upper_bound: *upper_bound,
                count: self.counts[index],
            });
        }

        // The final bucket is not artificially inflated by overflow. The
        // overflow count is represented by an explicit bucket at the maximum
        // configured bound plus the total count remains authoritative.
        if self.overflow > 0 {
            if let Some(last) = buckets.last_mut() {
                last.count = last.count.saturating_add(self.overflow);
            }
        }

        HistogramSnapshot {
            count: self.count,
            sum: self.sum,
            min: self.min,
            max: self.max,
            buckets,
        }
    }
}

// =============================================================================
// Metric storage
// =============================================================================

#[derive(Debug, Clone)]
struct MetricState {
    kind: MetricKind,
    counter: u64,
    gauge: f64,
    histogram: Option<HistogramState>,
}

impl MetricState {
    fn counter() -> Self {
        Self {
            kind: MetricKind::Counter,
            counter: 0,
            gauge: 0.0,
            histogram: None,
        }
    }

    fn gauge() -> Self {
        Self {
            kind: MetricKind::Gauge,
            counter: 0,
            gauge: 0.0,
            histogram: None,
        }
    }

    fn histogram(config: HistogramConfig) -> Self {
        Self {
            kind: MetricKind::Histogram,
            counter: 0,
            gauge: 0.0,
            histogram: Some(HistogramState::new(config)),
        }
    }
}

/// Immutable metric snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricSnapshot {
    /// Metric name.
    pub name: MetricName,

    /// Metric aggregation kind.
    pub kind: MetricKind,

    /// Metric labels.
    pub labels: MetricLabels,

    /// Counter value.
    pub counter: Option<CounterSnapshot>,

    /// Gauge value.
    pub gauge: Option<f64>,

    /// Histogram distribution.
    pub histogram: Option<HistogramSnapshot>,
}

// =============================================================================
// Telemetry health
// =============================================================================

/// Health of the telemetry subsystem itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryHealth {
    /// Telemetry is operating normally.
    Healthy,

    /// Telemetry is operating but some observations were rejected.
    Degraded,

    /// Telemetry cannot currently record observations.
    Failed,
}

impl TelemetryHealth {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
        }
    }
}

// =============================================================================
// Telemetry snapshot
// =============================================================================

/// Immutable, deterministic telemetry snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    /// Schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Caller-supplied snapshot timestamp.
    pub timestamp_unix_micros: u64,

    /// Telemetry health.
    pub health: TelemetryHealth,

    /// Aggregate metrics.
    pub metrics: Vec<MetricSnapshot>,

    /// Recent bounded events.
    pub recent_events: Vec<TelemetryEvent>,

    /// Number of rejected observations.
    pub rejected_observations: u64,

    /// Number of dropped events due to bounded retention.
    pub dropped_events: u64,
}

// =============================================================================
// Errors
// =============================================================================

/// Production telemetry errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemetryError {
    /// Invalid identifier.
    InvalidIdentifier {
        field: &'static str,
    },

    /// Invalid metric name.
    InvalidMetricName {
        value: String,
    },

    /// Text exceeded a configured bound.
    TextTooLong {
        field: &'static str,
    },

    /// Control characters were found.
    InvalidText {
        field: &'static str,
    },

    /// A secret-looking value was supplied.
    SecretRejected {
        field: String,
    },

    /// Too many labels were supplied.
    LabelLimitExceeded {
        maximum: usize,
    },

    /// Histogram configuration is invalid.
    InvalidHistogramConfiguration,

    /// Histogram bucket count is too large.
    HistogramBucketLimitExceeded {
        maximum: usize,
    },

    /// Numeric value is outside the supported range.
    ValueOutOfRange {
        field: &'static str,
    },

    /// Non-finite floating-point value.
    NonFiniteValue {
        field: &'static str,
    },

    /// Configuration violates a production limit.
    ConfigurationLimitExceeded {
        field: &'static str,
    },

    /// Metric kind does not match its existing registration.
    MetricKindConflict {
        metric: String,
    },

    /// Histogram configuration does not match an existing metric.
    HistogramConfigurationConflict {
        metric: String,
    },

    /// Internal telemetry lock was poisoned.
    LockPoisoned,

    /// A metric cannot be used in the requested operation.
    MetricOperationMismatch {
        metric: String,
    },
}

impl fmt::Display for TelemetryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid telemetry identifier: {field}")
            }
            Self::InvalidMetricName { value } => {
                write!(formatter, "invalid metric name: {value}")
            }
            Self::TextTooLong { field } => {
                write!(formatter, "telemetry field too long: {field}")
            }
            Self::InvalidText { field } => {
                write!(formatter, "invalid telemetry text: {field}")
            }
            Self::SecretRejected { field } => {
                write!(formatter, "secret-like telemetry field rejected: {field}")
            }
            Self::LabelLimitExceeded { maximum } => {
                write!(formatter, "telemetry label limit exceeded: {maximum}")
            }
            Self::InvalidHistogramConfiguration => {
                formatter.write_str("invalid telemetry histogram configuration")
            }
            Self::HistogramBucketLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "telemetry histogram bucket limit exceeded: {maximum}"
                )
            }
            Self::ValueOutOfRange { field } => {
                write!(formatter, "telemetry value out of range: {field}")
            }
            Self::NonFiniteValue { field } => {
                write!(formatter, "telemetry value is not finite: {field}")
            }
            Self::ConfigurationLimitExceeded { field } => {
                write!(formatter, "telemetry configuration limit exceeded: {field}")
            }
            Self::MetricKindConflict { metric } => {
                write!(formatter, "metric kind conflict: {metric}")
            }
            Self::HistogramConfigurationConflict { metric } => {
                write!(
                    formatter,
                    "histogram configuration conflict: {metric}"
                )
            }
            Self::LockPoisoned => {
                formatter.write_str("telemetry state lock poisoned")
            }
            Self::MetricOperationMismatch { metric } => {
                write!(formatter, "invalid operation for metric: {metric}")
            }
        }
    }
}

impl std::error::Error for TelemetryError {}

// =============================================================================
// Internal telemetry state
// =============================================================================

#[derive(Debug)]
struct TelemetryState {
    metrics: BTreeMap<(MetricName, MetricLabels), MetricState>,
    recent_events: std::collections::VecDeque<TelemetryEvent>,
    rejected_observations: u64,
    dropped_events: u64,
    health: TelemetryHealth,
}

// =============================================================================
// Telemetry
// =============================================================================

/// Thread-safe bounded in-process telemetry collector.
///
/// `Telemetry` is intentionally an aggregate collector rather than a logging
/// framework or remote exporter.
///
/// The object can safely be shared using `Arc<Telemetry>`.
#[derive(Debug)]
pub struct Telemetry {
    config: TelemetryConfig,
    state: Mutex<TelemetryState>,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new(TelemetryConfig::default())
            .expect("default telemetry configuration must be valid")
    }
}

impl Telemetry {
    /// Creates a validated telemetry collector.
    pub fn new(config: TelemetryConfig) -> Result<Self, TelemetryError> {
        config.validate()?;

        Ok(Self {
            state: Mutex::new(TelemetryState {
                metrics: BTreeMap::new(),
                recent_events: std::collections::VecDeque::with_capacity(
                    config.max_recent_events.min(DEFAULT_MAX_RECENT_EVENTS),
                ),
                rejected_observations: 0,
                dropped_events: 0,
                health: TelemetryHealth::Healthy,
            }),
            config,
        })
    }

    /// Returns whether telemetry recording is enabled.
    pub const fn enabled(&self) -> bool {
        self.config.enabled
    }

    /// Returns the immutable configuration.
    pub const fn config(&self) -> &TelemetryConfig {
        &self.config
    }

    /// Increments a counter metric by one.
    pub fn increment(
        &self,
        name: MetricName,
        labels: MetricLabels,
    ) -> Result<(), TelemetryError> {
        self.increment_by(name, labels, 1)
    }

    /// Increments a counter metric by an arbitrary bounded amount.
    pub fn increment_by(
        &self,
        name: MetricName,
        labels: MetricLabels,
        amount: u64,
    ) -> Result<(), TelemetryError> {
        if !self.config.enabled {
            return Ok(());
        }

        let mut state = self.lock_state()?;

        let metric = state
            .metrics
            .entry((name.clone(), labels))
            .or_insert_with(MetricState::counter);

        if metric.kind != MetricKind::Counter {
            state.rejected_observations =
                state.rejected_observations.saturating_add(1);
            state.health = TelemetryHealth::Degraded;

            return Err(TelemetryError::MetricKindConflict {
                metric: name.0,
            });
        }

        metric.counter = metric.counter.saturating_add(amount);

        Ok(())
    }

    /// Sets a gauge metric.
    pub fn set_gauge(
        &self,
        name: MetricName,
        labels: MetricLabels,
        value: f64,
    ) -> Result<(), TelemetryError> {
        if !value.is_finite() {
            return self.reject(TelemetryError::NonFiniteValue {
                field: "gauge",
            });
        }

        if !self.config.enabled {
            return Ok(());
        }

        let mut state = self.lock_state()?;

        let metric = state
            .metrics
            .entry((name.clone(), labels))
            .or_insert_with(MetricState::gauge);

        if metric.kind != MetricKind::Gauge {
            state.rejected_observations =
                state.rejected_observations.saturating_add(1);
            state.health = TelemetryHealth::Degraded;

            return Err(TelemetryError::MetricKindConflict {
                metric: name.0,
            });
        }

        metric.gauge = value;

        Ok(())
    }

    /// Records a histogram observation using the supplied configuration.
    pub fn observe(
        &self,
        name: MetricName,
        labels: MetricLabels,
        value: u64,
        histogram_config: HistogramConfig,
    ) -> Result<(), TelemetryError> {
        if !self.config.enabled {
            return Ok(());
        }

        let mut state = self.lock_state()?;

        let key = (name.clone(), labels);

        let metric = state.metrics.entry(key).or_insert_with(|| {
            MetricState::histogram(histogram_config.clone())
        });

        if metric.kind != MetricKind::Histogram {
            state.rejected_observations =
                state.rejected_observations.saturating_add(1);
            state.health = TelemetryHealth::Degraded;

            return Err(TelemetryError::MetricKindConflict {
                metric: name.0,
            });
        }

        let histogram = metric
            .histogram
            .as_mut()
            .ok_or_else(|| TelemetryError::MetricOperationMismatch {
                metric: name.0.clone(),
            })?;

        if histogram.config != histogram_config {
            state.rejected_observations =
                state.rejected_observations.saturating_add(1);
            state.health = TelemetryHealth::Degraded;

            return Err(TelemetryError::HistogramConfigurationConflict {
                metric: name.0,
            });
        }

        histogram.observe(value)?;

        Ok(())
    }

    /// Records a duration in microseconds.
    pub fn observe_duration(
        &self,
        name: MetricName,
        labels: MetricLabels,
        duration: DurationMicros,
    ) -> Result<(), TelemetryError> {
        self.observe(
            name,
            labels,
            duration.as_micros(),
            self.config.latency_histogram.clone(),
        )
    }

    /// Records queue depth.
    pub fn observe_queue_depth(
        &self,
        labels: MetricLabels,
        depth: u64,
    ) -> Result<(), TelemetryError> {
        if depth > MAX_QUEUE_DEPTH {
            return self.reject(TelemetryError::ValueOutOfRange {
                field: "queue_depth",
            });
        }

        let name = MetricName::new(metrics::QUEUE_DEPTH)?;

        self.observe(
            name,
            labels,
            depth,
            self.config.queue_histogram.clone(),
        )
    }

    /// Records backend availability as a gauge value.
    ///
    /// `1.0` means submission-eligible, `0.0` means not submission-eligible.
    /// The detailed state remains available through the event stream.
    pub fn observe_availability(
        &self,
        labels: MetricLabels,
        availability: AvailabilityObservation,
    ) -> Result<(), TelemetryError> {
        let mut availability_labels = labels;

        availability_labels.insert(
            "state",
            availability.as_str(),
        )?;

        let name = MetricName::new(metrics::BACKEND_AVAILABILITY)?;

        self.set_gauge(
            name,
            availability_labels,
            if availability.is_submission_eligible() {
                1.0
            } else {
                0.0
            },
        )
    }

    /// Records a backend health score in the range 0..=100.
    pub fn observe_health_score(
        &self,
        labels: MetricLabels,
        score: f64,
    ) -> Result<(), TelemetryError> {
        if !score.is_finite() {
            return self.reject(TelemetryError::NonFiniteValue {
                field: "health_score",
            });
        }

        if !(0.0..=100.0).contains(&score) {
            return self.reject(TelemetryError::ValueOutOfRange {
                field: "health_score",
            });
        }

        self.set_gauge(
            MetricName::new(metrics::BACKEND_HEALTH_SCORE)?,
            labels,
            score,
        )
    }

    /// Records calibration age in seconds.
    pub fn observe_calibration_age(
        &self,
        labels: MetricLabels,
        age_seconds: u64,
    ) -> Result<(), TelemetryError> {
        if age_seconds > MAX_CALIBRATION_AGE_SECONDS {
            return self.reject(TelemetryError::ValueOutOfRange {
                field: "calibration_age_seconds",
            });
        }

        let name = MetricName::new(metrics::CALIBRATION_AGE_SECONDS)?;

        self.set_gauge(
            name,
            labels,
            age_seconds as f64,
        )
    }

    /// Records an immutable telemetry event.
    pub fn record_event(
        &self,
        mut event: TelemetryEvent,
    ) -> Result<(), TelemetryError> {
        if !self.config.enabled {
            return Ok(());
        }

        if !self.config.retain_event_messages {
            event.message = None;
        }

        let mut state = self.lock_state()?;

        if self.config.max_recent_events == 0 {
            return Ok(());
        }

        if state.recent_events.len() >= self.config.max_recent_events {
            state.recent_events.pop_front();
            state.dropped_events =
                state.dropped_events.saturating_add(1);
        }

        state.recent_events.push_back(event);

        Ok(())
    }

    /// Records a provider error and increments its aggregate counters.
    pub fn record_error(
        &self,
        labels: MetricLabels,
        class: TelemetryErrorClass,
        provider_error_code: Option<String>,
        retryable: bool,
        timestamp_unix_micros: u64,
    ) -> Result<(), TelemetryError> {
        let event_kind = match class {
            TelemetryErrorClass::Authentication => {
                TelemetryEventKind::AuthenticationFailure
            }
            TelemetryErrorClass::Authorization => {
                TelemetryEventKind::AuthorizationFailure
            }
            TelemetryErrorClass::RateLimited => {
                TelemetryEventKind::RateLimited
            }
            TelemetryErrorClass::Transport => {
                TelemetryEventKind::TransportFailure
            }
            TelemetryErrorClass::Timeout => {
                TelemetryEventKind::ExecutionTimedOut
            }
            _ => TelemetryEventKind::ProviderError,
        };

        let event = TelemetryEvent::new(
            event_kind,
            timestamp_unix_micros,
        )
        .with_error(
            class,
            provider_error_code,
            retryable,
        )?;

        self.record_event(event)?;

        let provider_error_name =
            MetricName::new(metrics::PROVIDER_ERRORS)?;

        self.increment(provider_error_name, labels.clone())?;

        let specific_metric = match class {
            TelemetryErrorClass::Authentication => {
                Some(metrics::AUTHENTICATION_FAILURES)
            }
            TelemetryErrorClass::Authorization => {
                Some(metrics::AUTHORIZATION_FAILURES)
            }
            TelemetryErrorClass::RateLimited => {
                Some(metrics::RATE_LIMITS)
            }
            TelemetryErrorClass::Transport => {
                Some(metrics::TRANSPORT_FAILURES)
            }
            TelemetryErrorClass::Timeout => {
                Some(metrics::TIMEOUTS)
            }
            _ => None,
        };

        if let Some(metric) = specific_metric {
            self.increment(MetricName::new(metric)?, labels)?;
        }

        Ok(())
    }

    /// Produces an immutable deterministic snapshot.
    pub fn snapshot(
        &self,
        timestamp_unix_micros: u64,
    ) -> Result<TelemetrySnapshot, TelemetryError> {
        let state = self.lock_state()?;

        let metrics = state
            .metrics
            .iter()
            .map(|((name, labels), metric)| {
                let histogram = metric
                    .histogram
                    .as_ref()
                    .map(HistogramState::snapshot);

                MetricSnapshot {
                    name: name.clone(),
                    kind: metric.kind,
                    labels: labels.clone(),
                    counter: if metric.kind == MetricKind::Counter {
                        Some(CounterSnapshot::new(metric.counter))
                    } else {
                        None
                    },
                    gauge: if metric.kind == MetricKind::Gauge {
                        Some(metric.gauge)
                    } else {
                        None
                    },
                    histogram,
                }
            })
            .collect();

        Ok(TelemetrySnapshot {
            schema_id: TELEMETRY_SCHEMA_ID.to_owned(),
            schema_version: TELEMETRY_SCHEMA_VERSION,
            timestamp_unix_micros,
            health: state.health,
            metrics,
            recent_events: state.recent_events.iter().cloned().collect(),
            rejected_observations: state.rejected_observations,
            dropped_events: state.dropped_events,
        })
    }

    /// Returns the current telemetry health.
    pub fn health(&self) -> Result<TelemetryHealth, TelemetryError> {
        Ok(self.lock_state()?.health)
    }

    /// Clears aggregate metrics and recent events.
    ///
    /// This is useful for test isolation and explicit metric epochs.
    ///
    /// It does not change configuration.
    pub fn reset(&self) -> Result<(), TelemetryError> {
        let mut state = self.lock_state()?;

        state.metrics.clear();
        state.recent_events.clear();
        state.rejected_observations = 0;
        state.dropped_events = 0;
        state.health = TelemetryHealth::Healthy;

        Ok(())
    }

    fn lock_state(&self) -> Result<MutexGuard<'_, TelemetryState>, TelemetryError> {
        self.state
            .lock()
            .map_err(|_| TelemetryError::LockPoisoned)
    }

    fn reject<T>(
        &self,
        error: TelemetryError,
    ) -> Result<T, TelemetryError> {
        if let Ok(mut state) = self.state.lock() {
            state.rejected_observations =
                state.rejected_observations.saturating_add(1);
            state.health = TelemetryHealth::Degraded;
        }

        Err(error)
    }
}

// =============================================================================
// High-level execution telemetry helper
// =============================================================================

/// Provider-neutral execution timing information.
///
/// This structure allows `execution.rs` and `job.rs` to construct one
/// consistent observation without requiring telemetry to understand job
/// lifecycle internals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionTiming {
    /// Time spent submitting the request.
    pub submission: Option<DurationMicros>,

    /// Time spent waiting in a queue.
    pub queue: Option<DurationMicros>,

    /// Time spent executing on the backend.
    pub execution: Option<DurationMicros>,

    /// Time spent retrieving/normalizing results.
    pub result: Option<DurationMicros>,

    /// Total end-to-end elapsed time.
    pub total: Option<DurationMicros>,
}

impl ExecutionTiming {
    /// Creates an empty timing record.
    pub const fn new() -> Self {
        Self {
            submission: None,
            queue: None,
            execution: None,
            result: None,
            total: None,
        }
    }

    /// Records all available execution timing metrics.
    pub fn record(
        &self,
        telemetry: &Telemetry,
        labels: MetricLabels,
    ) -> Result<(), TelemetryError> {
        if let Some(duration) = self.submission {
            telemetry.observe_duration(
                MetricName::new(metrics::SUBMISSION_LATENCY_MICROS)?,
                labels.clone(),
                duration,
            )?;
        }

        if let Some(duration) = self.queue {
            telemetry.observe_duration(
                MetricName::new(metrics::QUEUE_LATENCY_MICROS)?,
                labels.clone(),
                duration,
            )?;
        }

        if let Some(duration) = self.execution {
            telemetry.observe_duration(
                MetricName::new(metrics::EXECUTION_LATENCY_MICROS)?,
                labels.clone(),
                duration,
            )?;
        }

        if let Some(duration) = self.result {
            telemetry.observe_duration(
                MetricName::new(metrics::RESULT_LATENCY_MICROS)?,
                labels.clone(),
                duration,
            )?;
        }

        if let Some(duration) = self.total {
            telemetry.observe_duration(
                MetricName::new(metrics::TOTAL_LATENCY_MICROS)?,
                labels,
                duration,
            )?;
        }

        Ok(())
    }
}

impl Default for ExecutionTiming {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Execution outcome
// =============================================================================

/// Normalized execution outcome for telemetry.
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
pub enum ExecutionOutcome {
    /// Accepted/submitted.
    Submitted,

    /// Successfully completed.
    Succeeded,

    /// Failed.
    Failed,

    /// Cancelled.
    Cancelled,

    /// Timed out.
    TimedOut,
}

impl ExecutionOutcome {
    /// Records the aggregate outcome.
    pub fn record(
        self,
        telemetry: &Telemetry,
        labels: MetricLabels,
    ) -> Result<(), TelemetryError> {
        let metric = match self {
            Self::Submitted => metrics::EXECUTION_SUBMISSIONS,
            Self::Succeeded => metrics::EXECUTION_SUCCESSES,
            Self::Failed => metrics::EXECUTION_FAILURES,
            Self::Cancelled => metrics::EXECUTION_CANCELLATIONS,
            Self::TimedOut => metrics::TIMEOUTS,
        };

        telemetry.increment(MetricName::new(metric)?, labels)
    }

    /// Returns the corresponding event kind.
    pub const fn event_kind(self) -> TelemetryEventKind {
        match self {
            Self::Submitted => TelemetryEventKind::ExecutionSubmitted,
            Self::Succeeded => TelemetryEventKind::ExecutionCompleted,
            Self::Failed => TelemetryEventKind::ExecutionFailed,
            Self::Cancelled => TelemetryEventKind::ExecutionCancelled,
            Self::TimedOut => TelemetryEventKind::ExecutionTimedOut,
        }
    }
}

// =============================================================================
// Retry telemetry
// =============================================================================

/// Records a retry observation.
pub fn record_retry(
    telemetry: &Telemetry,
    labels: MetricLabels,
    timestamp_unix_micros: u64,
) -> Result<(), TelemetryError> {
    telemetry.increment(
        MetricName::new(metrics::EXECUTION_RETRIES)?,
        labels.clone(),
    )?;

    telemetry.record_event(
        TelemetryEvent::new(
            TelemetryEventKind::ExecutionRetried,
            timestamp_unix_micros,
        ),
    )
}

// =============================================================================
// Standard label constructors
// =============================================================================

/// Creates the recommended backend labels.
///
/// This function deliberately excludes job/request IDs to avoid high
/// cardinality.
pub fn backend_labels(
    provider_id: Option<&str>,
    backend_id: &str,
) -> Result<MetricLabels, TelemetryError> {
    let mut labels = MetricLabels::new();

    if let Some(provider_id) = provider_id {
        labels.insert(
            "provider",
            validate_identifier(
                provider_id,
                "provider_id",
                MAX_PROVIDER_ID_LENGTH,
            )?,
        )?;
    }

    labels.insert(
        "backend",
        validate_identifier(
            backend_id,
            "backend_id",
            MAX_BACKEND_ID_LENGTH,
        )?,
    )?;

    Ok(labels)
}

// =============================================================================
// Validation
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
    maximum: usize,
) -> Result<String, TelemetryError> {
    if value.is_empty()
        || value.len() > maximum
        || value.trim() != value
        || value.chars().any(|character| character.is_control())
    {
        return Err(TelemetryError::InvalidIdentifier { field });
    }

    if contains_secret_marker(value) {
        return Err(TelemetryError::SecretRejected {
            field: field.to_owned(),
        });
    }

    Ok(value.to_owned())
}

fn validate_text(
    value: &str,
    field: &'static str,
    maximum: usize,
) -> Result<(), TelemetryError> {
    if value.len() > maximum {
        return Err(TelemetryError::TextTooLong { field });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(TelemetryError::InvalidText { field });
    }

    Ok(())
}

fn validate_label(
    key: &str,
    value: &str,
) -> Result<(), TelemetryError> {
    if key.is_empty()
        || key.len() > MAX_LABEL_KEY_LENGTH
        || value.len() > MAX_LABEL_VALUE_LENGTH
    {
        return Err(TelemetryError::InvalidIdentifier {
            field: "metric_label",
        });
    }

    if key.trim() != key || value.trim() != value {
        return Err(TelemetryError::InvalidIdentifier {
            field: "metric_label",
        });
    }

    if key.chars().any(|character| character.is_control())
        || value.chars().any(|character| character.is_control())
    {
        return Err(TelemetryError::InvalidText {
            field: "metric_label",
        });
    }

    if contains_secret_marker(key) || contains_secret_marker(value) {
        return Err(TelemetryError::SecretRejected {
            field: key.to_owned(),
        });
    }

    Ok(())
}

fn contains_secret_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const SECRET_MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "accesstoken",
        "authorization",
        "auth_header",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "secret",
        "session_cookie",
        "sessioncookie",
        "bearer ",
        "token=",
        "api-key",
    ];

    SECRET_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn telemetry() -> Telemetry {
        Telemetry::default()
    }

    #[test]
    fn metric_name_is_deterministic() {
        let name = MetricName::new("zamani.test.counter")
            .expect("metric name should be valid");

        assert_eq!(name.as_str(), "zamani.test.counter");
        assert_eq!(name.to_string(), "zamani.test.counter");
    }

    #[test]
    fn invalid_metric_name_is_rejected() {
        assert!(MetricName::new("metric name").is_err());
        assert!(MetricName::new("metric/name").is_err());
    }

    #[test]
    fn secret_label_is_rejected() {
        let mut labels = MetricLabels::new();

        assert!(
            labels
                .insert("api_key", "secret-value")
                .is_err()
        );

        assert!(
            labels
                .insert("normal", "contains_api_key")
                .is_err()
        );
    }

    #[test]
    fn label_count_is_bounded() {
        let mut labels = MetricLabels::new();

        for index in 0..MAX_LABELS {
            labels
                .insert(
                    format!("key{index}"),
                    "value",
                )
                .expect("label should fit");
        }

        assert!(
            labels
                .insert("one-too-many", "value")
                .is_err()
        );
    }

    #[test]
    fn histogram_rejects_duplicate_buckets() {
        assert!(
            HistogramConfig::new(vec![1, 1, 2])
                .is_err()
        );
    }

    #[test]
    fn histogram_records_values() {
        let config = HistogramConfig::new(vec![10, 20, 30])
            .expect("configuration should be valid");

        let mut state = HistogramState::new(config);

        state.observe(5).expect("observation should work");
        state.observe(15).expect("observation should work");
        state.observe(25).expect("observation should work");
        state.observe(50).expect("observation should work");

        let snapshot = state.snapshot();

        assert_eq!(snapshot.count, 4);
        assert_eq!(snapshot.min, Some(5));
        assert_eq!(snapshot.max, Some(50));
        assert_eq!(snapshot.sum, 95);
        assert_eq!(snapshot.mean(), Some(23.75));
    }

    #[test]
    fn histogram_quantiles_are_available() {
        let config = HistogramConfig::new(vec![10, 20, 30, 40])
            .expect("configuration should be valid");

        let mut state = HistogramState::new(config);

        for value in [1, 5, 10, 15, 20, 25, 30, 35, 40] {
            state.observe(value).expect("observation should work");
        }

        let snapshot = state.snapshot();

        assert!(snapshot.median().is_some());
        assert!(snapshot.p95().is_some());
        assert!(snapshot.p99().is_some());
    }

    #[test]
    fn counters_are_recorded() {
        let telemetry = telemetry();

        telemetry
            .increment(
                MetricName::new("test.counter")
                    .expect("valid name"),
                MetricLabels::new(),
            )
            .expect("increment should work");

        telemetry
            .increment_by(
                MetricName::new("test.counter")
                    .expect("valid name"),
                MetricLabels::new(),
                4,
            )
            .expect("increment should work");

        let snapshot = telemetry
            .snapshot(123)
            .expect("snapshot should work");

        assert_eq!(snapshot.metrics.len(), 1);

        let metric = &snapshot.metrics[0];

        assert_eq!(
            metric.counter,
            Some(CounterSnapshot::new(5))
        );
    }

    #[test]
    fn gauge_is_recorded() {
        let telemetry = telemetry();

        telemetry
            .set_gauge(
                MetricName::new("test.gauge")
                    .expect("valid name"),
                MetricLabels::new(),
                42.5,
            )
            .expect("gauge should work");

        let snapshot = telemetry
            .snapshot(123)
            .expect("snapshot should work");

        assert_eq!(
            snapshot.metrics[0].gauge,
            Some(42.5)
        );
    }

    #[test]
    fn non_finite_gauge_is_rejected() {
        let telemetry = telemetry();

        assert!(
            telemetry
                .set_gauge(
                    MetricName::new("test.gauge")
                        .expect("valid name"),
                    MetricLabels::new(),
                    f64::NAN,
                )
                .is_err()
        );

        assert_eq!(
            telemetry.health().expect("health should work"),
            TelemetryHealth::Degraded
        );
    }

    #[test]
    fn metric_kind_conflict_is_rejected() {
        let telemetry = telemetry();

        let name = MetricName::new("same.metric")
            .expect("valid name");

        telemetry
            .increment(
                name.clone(),
                MetricLabels::new(),
            )
            .expect("counter should work");

        assert!(
            telemetry
                .set_gauge(
                    name,
                    MetricLabels::new(),
                    1.0,
                )
                .is_err()
        );
    }

    #[test]
    fn events_are_bounded() {
        let config = TelemetryConfig {
            max_recent_events: 2,
            ..TelemetryConfig::default()
        };

        let telemetry =
            Telemetry::new(config)
                .expect("configuration should work");

        for timestamp in 0..4 {
            telemetry
                .record_event(
                    TelemetryEvent::new(
                        TelemetryEventKind::ExecutionStarted,
                        timestamp,
                    ),
                )
                .expect("event should work");
        }

        let snapshot = telemetry
            .snapshot(100)
            .expect("snapshot should work");

        assert_eq!(snapshot.recent_events.len(), 2);
        assert_eq!(snapshot.dropped_events, 2);
        assert_eq!(
            snapshot.recent_events[0].timestamp_unix_micros,
            2
        );
        assert_eq!(
            snapshot.recent_events[1].timestamp_unix_micros,
            3
        );
    }

    #[test]
    fn event_message_can_be_disabled() {
        let config = TelemetryConfig {
            retain_event_messages: false,
            ..TelemetryConfig::default()
        };

        let telemetry =
            Telemetry::new(config)
                .expect("configuration should work");

        let event = TelemetryEvent::new(
            TelemetryEventKind::ExecutionCompleted,
            1,
        )
        .with_message("safe diagnostic message")
        .expect("message should work");

        telemetry
            .record_event(event)
            .expect("event should work");

        let snapshot = telemetry
            .snapshot(2)
            .expect("snapshot should work");

        assert_eq!(
            snapshot.recent_events[0].message,
            None
        );
    }

    #[test]
    fn backend_labels_do_not_include_high_cardinality_ids() {
        let labels = backend_labels(
            Some("provider"),
            "backend",
        )
        .expect("labels should work");

        assert_eq!(labels.get("provider"), Some("provider"));
        assert_eq!(labels.get("backend"), Some("backend"));
        assert_eq!(labels.get("job"), None);
    }

    #[test]
    fn availability_is_recorded() {
        let telemetry = telemetry();

        telemetry
            .observe_availability(
                MetricLabels::new(),
                AvailabilityObservation::Available,
            )
            .expect("availability should work");

        let snapshot = telemetry
            .snapshot(1)
            .expect("snapshot should work");

        assert_eq!(snapshot.metrics.len(), 1);
        assert_eq!(
            snapshot.metrics[0].gauge,
            Some(1.0)
        );
    }

    #[test]
    fn health_score_is_bounded() {
        let telemetry = telemetry();

        assert!(
            telemetry
                .observe_health_score(
                    MetricLabels::new(),
                    101.0,
                )
                .is_err()
        );

        telemetry
            .observe_health_score(
                MetricLabels::new(),
                75.0,
            )
            .expect("valid score should work");
    }

    #[test]
    fn calibration_age_is_bounded() {
        let telemetry = telemetry();

        telemetry
            .observe_calibration_age(
                MetricLabels::new(),
                100,
            )
            .expect("valid age should work");

        assert!(
            telemetry
                .observe_calibration_age(
                    MetricLabels::new(),
                    MAX_CALIBRATION_AGE_SECONDS + 1,
                )
                .is_err()
        );
    }

    #[test]
    fn execution_outcome_records_metric() {
        let telemetry = telemetry();

        ExecutionOutcome::Succeeded
            .record(
                &telemetry,
                MetricLabels::new(),
            )
            .expect("outcome should work");

        let snapshot = telemetry
            .snapshot(1)
            .expect("snapshot should work");

        assert_eq!(
            snapshot.metrics[0].name.as_str(),
            metrics::EXECUTION_SUCCESSES
        );
    }

    #[test]
    fn execution_timing_records_all_present_values() {
        let telemetry = telemetry();

        let timing = ExecutionTiming {
            submission: Some(
                DurationMicros::new(100)
                    .expect("duration should work"),
            ),
            queue: Some(
                DurationMicros::new(200)
                    .expect("duration should work"),
            ),
            execution: Some(
                DurationMicros::new(300)
                    .expect("duration should work"),
            ),
            result: Some(
                DurationMicros::new(400)
                    .expect("duration should work"),
            ),
            total: Some(
                DurationMicros::new(1_000)
                    .expect("duration should work"),
            ),
        };

        timing
            .record(
                &telemetry,
                MetricLabels::new(),
            )
            .expect("timing should work");

        let snapshot = telemetry
            .snapshot(1)
            .expect("snapshot should work");

        assert_eq!(snapshot.metrics.len(), 5);
    }

    #[test]
    fn provider_error_classification_is_retryable_when_expected() {
        assert!(
            TelemetryErrorClass::Transport.is_retryable()
        );

        assert!(
            TelemetryErrorClass::RateLimited.is_retryable()
        );

        assert!(
            !TelemetryErrorClass::Authentication.is_retryable()
        );
    }

    #[test]
    fn provider_error_is_recorded() {
        let telemetry = telemetry();

        telemetry
            .record_error(
                MetricLabels::new(),
                TelemetryErrorClass::RateLimited,
                Some("provider_rate_limit".to_owned()),
                true,
                123,
            )
            .expect("error should be recorded");

        let snapshot = telemetry
            .snapshot(456)
            .expect("snapshot should work");

        assert!(
            snapshot
                .metrics
                .iter()
                .any(|metric| {
                    metric.name.as_str()
                        == metrics::PROVIDER_ERRORS
                })
        );

        assert_eq!(
            snapshot.recent_events.len(),
            1
        );

        assert_eq!(
            snapshot.recent_events[0].kind,
            TelemetryEventKind::RateLimited
        );
    }

    #[test]
    fn retry_is_recorded() {
        let telemetry = telemetry();

        record_retry(
            &telemetry,
            MetricLabels::new(),
            100,
        )
        .expect("retry should work");

        let snapshot = telemetry
            .snapshot(200)
            .expect("snapshot should work");

        assert!(
            snapshot
                .metrics
                .iter()
                .any(|metric| {
                    metric.name.as_str()
                        == metrics::EXECUTION_RETRIES
                })
        );

        assert_eq!(
            snapshot.recent_events[0].kind,
            TelemetryEventKind::ExecutionRetried
        );
    }

    #[test]
    fn telemetry_can_be_disabled_without_failing_callers() {
        let config = TelemetryConfig {
            enabled: false,
            ..TelemetryConfig::default()
        };

        let telemetry =
            Telemetry::new(config)
                .expect("configuration should work");

        telemetry
            .increment(
                MetricName::new("disabled.counter")
                    .expect("valid name"),
                MetricLabels::new(),
            )
            .expect("disabled telemetry should be harmless");

        let snapshot = telemetry
            .snapshot(1)
            .expect("snapshot should work");

        assert!(snapshot.metrics.is_empty());
        assert!(snapshot.recent_events.is_empty());
    }

    #[test]
    fn reset_clears_state() {
        let telemetry = telemetry();

        telemetry
            .increment(
                MetricName::new("test.counter")
                    .expect("valid name"),
                MetricLabels::new(),
            )
            .expect("counter should work");

        telemetry
            .reset()
            .expect("reset should work");

        let snapshot = telemetry
            .snapshot(1)
            .expect("snapshot should work");

        assert!(snapshot.metrics.is_empty());
        assert!(snapshot.recent_events.is_empty());
        assert_eq!(snapshot.rejected_observations, 0);
        assert_eq!(snapshot.dropped_events, 0);
    }

    #[test]
    fn serialization_round_trip_works() {
        let telemetry = telemetry();

        telemetry
            .increment(
                MetricName::new("serialization.counter")
                    .expect("valid name"),
                MetricLabels::new(),
            )
            .expect("counter should work");

        let snapshot = telemetry
            .snapshot(123)
            .expect("snapshot should work");

        let json =
            serde_json::to_string(&snapshot)
                .expect("serialization should work");

        let restored: TelemetrySnapshot =
            serde_json::from_str(&json)
                .expect("deserialization should work");

        assert_eq!(snapshot, restored);
    }

    #[test]
    fn secret_event_message_is_rejected() {
        let result = TelemetryEvent::new(
            TelemetryEventKind::ProviderError,
            1,
        )
        .with_message("api_key=do-not-store");

        assert!(result.is_err());
    }

    #[test]
    fn duration_from_std_duration_works() {
        let duration = DurationMicros::from_duration(
            Duration::from_millis(5),
        )
        .expect("duration should work");

        assert_eq!(duration.as_micros(), 5_000);
    }
}