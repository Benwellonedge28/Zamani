//! Zamani Quantum — Hardware Health
//!
//! Production-grade, provider-independent health and readiness model for
//! quantum execution targets.
//!
//! # Responsibility
//!
//! This module is the authoritative representation of the HEALTH STATE of a
//! quantum execution target.
//!
//! It answers:
//!
//! - Is the target reachable?
//! - Is the target operational?
//! - Is it accepting work?
//! - Is it degraded?
//! - Is authentication valid?
//! - Is the API compatible?
//! - Is calibration available/current?
//! - Is topology information available?
//! - Is the execution service available?
//! - Is the queue service available?
//! - Is result retrieval available?
//! - When was health last observed?
//! - What caused a degraded/unavailable state?
//! - Is the health observation fresh enough for a requested operation?
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - perform network I/O;
//! - call provider APIs;
//! - authenticate against providers;
//! - own credentials;
//! - store API keys or tokens;
//! - execute quantum programs;
//! - submit jobs;
//! - cancel jobs;
//! - retrieve quantum results;
//! - acquire calibration data;
//! - mutate topology;
//! - perform routing;
//! - perform scheduling;
//! - perform benchmarking;
//! - perform QEC;
//! - perform transpilation;
//! - parse OpenQASM;
//! - generate QIR;
//! - run simulators;
//! - run hardware emulators.
//!
//! A provider adapter, local backend, registry, or monitoring subsystem
//! performs the actual health probe and constructs a validated `HealthReport`.
//!
//! This module only defines the provider-neutral health contract and its
//! deterministic validation/aggregation semantics.
//!
//! # Architectural position
//!
//! ```text
//!                 Provider / Local Adapter
//!                           |
//!                    performs actual probe
//!                           |
//!                           v
//!                    HealthObservation
//!                           |
//!                           v
//!                      HealthReport
//!                           |
//!            +--------------+---------------+
//!            |              |               |
//!            v              v               v
//!        backend        registry        execution
//!            |              |               |
//!            +--------------+---------------+
//!                           |
//!                           v
//!                  compatibility / Danga
//!
//! Hardware health never depends on benchmarking.
//! ```
//!
//! # Integration contract
//!
//! This file is intentionally independent from the other planned hardware
//! modules.
//!
//! It may be consumed by:
//!
//! - `backend.rs`;
//! - `backend_status.rs`;
//! - `backend_trait.rs`;
//! - `provider.rs`;
//! - `provider_registry.rs`;
//! - `device_registry.rs`;
//! - `discovery.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `queue.rs`;
//! - `telemetry.rs`;
//! - `compatibility.rs`;
//! - `validation.rs`;
//! - provider adapters;
//! - local adapters;
//! - benchmarking;
//! - Danga.
//!
//! None of those modules are required to compile this file.
//!
//! # Important integration rule
//!
//! Health is OBSERVED STATE.
//!
//! It is not capability state.
//!
//! For example:
//!
//! ```text
//! capability: measurement = true
//! health:       measurement service = unavailable
//! ```
//!
//! A backend may permanently support a feature while temporarily being unable
//! to execute it.
//!
//! Likewise:
//!
//! ```text
//! capability: pulse_control = true
//! health:       pulse service = degraded
//! ```
//!
//! Capability and health must therefore never be collapsed into one structure.
//!
//! # Health dimensions
//!
//! Health is multidimensional rather than a single boolean.
//!
//! The canonical dimensions are:
//!
//! - reachability;
//! - authentication;
//! - authorization;
//! - API compatibility;
//! - device operational state;
//! - execution;
//! - queue;
//! - result retrieval;
//! - calibration;
//! - topology;
//! - timing;
//! - provider service;
//! - resource availability.
//!
//! A provider may report some dimensions as unknown. Unknown is not silently
//! converted to healthy.
//!
//! # Safety model
//!
//! `HealthStatus::Healthy` means that the corresponding health evidence was
//! successfully observed and passed validation.
//!
//! It does NOT mean that the backend is guaranteed to remain healthy after the
//! observation.
//!
//! Health is inherently time-dependent.
//!
//! # Freshness
//!
//! Health observations contain an observation timestamp.
//!
//! Consumers can apply a maximum permitted age through `HealthFreshnessPolicy`.
//!
//! This prevents stale health information from being treated as current.
//!
//! # Determinism
//!
//! This module uses `BTreeMap`/`BTreeSet` where ordered collections are exposed.
//!
//! Health aggregation is deterministic:
//!
//! - dimensions are evaluated in stable order;
//! - diagnostics are sorted;
//! - duplicate diagnostic identifiers are rejected;
//! - severity aggregation follows a fixed ordering;
//! - no hash-map iteration order affects the result.
//!
//! # Security
//!
//! Health diagnostics MUST NOT contain credentials.
//!
//! This module rejects diagnostic text containing obvious secret-bearing field
//! names such as:
//!
//! - api_key;
//! - access_token;
//! - authorization;
//! - password;
//! - private_key;
//! - secret;
//! - session_cookie.
//!
//! This is defense in depth, not a replacement for a secret-management system.
//!
//! # Clock handling
//!
//! Health observations use Unix nanoseconds.
//!
//! System-clock acquisition is NOT performed automatically by health models.
//!
//! Callers provide timestamps explicitly, which makes health records:
//!
//! - deterministic;
//! - testable;
//! - replayable;
//! - serializable;
//! - suitable for remote providers.
//!
//! `HealthTimestamp::now()` is provided only as an explicit convenience.
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
//! # Unsafe policy
//!
//! Unsafe Rust is forbidden.
//!
//! # Serialization
//!
//! All public persisted health structures implement Serde serialization.
//!
//! The schema identifier and version are included as stable constants.
//!
//! Serialization is provider-neutral and contains no credentials.
//!
//! # File-freeze rule
//!
//! This module is complete independently.
//!
//! Future modules MUST adapt to this contract rather than requiring changes to
//! this file merely because another hardware subsystem is implemented.
//!
//! -----------------------------------------------------------------------------
//! Public contract
//! -----------------------------------------------------------------------------
//!
//! `HealthReport` is the canonical aggregate.
//!
//! `HealthObservation` represents one individual health probe.
//!
//! `HealthCheck` identifies the thing that was checked.
//!
//! `HealthStatus` represents the result.
//!
//! `HealthSeverity` represents diagnostic seriousness.
//!
//! `HealthFreshnessPolicy` controls whether an observation is sufficiently
//! recent for a consumer.
//!
//! `HealthError` represents construction/validation failures.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const HEALTH_SCHEMA_ID: &str = "zamani.quantum.hardware.health";

/// Serialized schema version.
///
/// Increment only when serialized semantics change incompatibly.
pub const HEALTH_SCHEMA_VERSION: u16 = 1;

/// Maximum backend/device identifier length in UTF-8 bytes.
pub const MAX_TARGET_ID_LENGTH: usize = 512;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum health-check identifier length.
pub const MAX_CHECK_ID_LENGTH: usize = 256;

/// Maximum diagnostic code length.
pub const MAX_DIAGNOSTIC_CODE_LENGTH: usize = 256;

/// Maximum diagnostic message length.
pub const MAX_DIAGNOSTIC_MESSAGE_LENGTH: usize = 4096;

/// Maximum diagnostic count per report.
pub const MAX_DIAGNOSTICS: usize = 4096;

/// Maximum health observation count per report.
pub const MAX_OBSERVATIONS: usize = 256;

/// Maximum metadata fields.
pub const MAX_METADATA_FIELDS: usize = 4096;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum timestamp age representable by a freshness policy.
///
/// This is a safety bound against accidental duration arithmetic overflow.
pub const MAX_FRESHNESS_AGE_NS: u64 = u64::MAX;

/// Nanoseconds per second.
const NANOS_PER_SECOND: u64 = 1_000_000_000;

// =============================================================================
// Health status
// =============================================================================

/// Result of an individual health check or aggregate health evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    /// No authoritative health information exists.
    Unknown,

    /// The checked component is healthy.
    Healthy,

    /// The checked component is operational but degraded.
    Degraded,

    /// The checked component is temporarily unavailable.
    Unavailable,

    /// The checked component failed.
    Failed,

    /// The target is permanently retired.
    Retired,
}

impl HealthStatus {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Failed => "failed",
            Self::Retired => "retired",
        }
    }

    /// Returns true only when the health state is fully healthy.
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Returns true when execution may potentially proceed, subject to
    /// operation-specific policy.
    pub const fn is_operational(self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Returns true when the state represents a terminal condition.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns a deterministic severity rank.
    ///
    /// Higher values represent more serious states.
    pub const fn severity_rank(self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Unknown => 1,
            Self::Degraded => 2,
            Self::Unavailable => 3,
            Self::Failed => 4,
            Self::Retired => 5,
        }
    }

    /// Returns the more severe of two statuses.
    pub const fn max_severity(self, other: Self) -> Self {
        if self.severity_rank() >= other.severity_rank() {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Health severity
// =============================================================================

/// Severity of a health diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSeverity {
    /// Informational observation.
    Info,

    /// Potential issue that does not necessarily block execution.
    Warning,

    /// Significant issue.
    Error,

    /// Issue that prevents safe operation.
    Critical,
}

impl HealthSeverity {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Deterministic severity rank.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Critical => 3,
        }
    }

    /// Returns the more severe level.
    pub const fn max(self, other: Self) -> Self {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for HealthSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Health check
// =============================================================================

/// Canonical health-check dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthCheck {
    /// Can the target/provider be reached?
    Reachability,

    /// Are credentials/authentication mechanisms accepted?
    Authentication,

    /// Is the caller authorized to use the target?
    Authorization,

    /// Is the provider API/version compatible?
    ApiCompatibility,

    /// Is the physical/software target operational?
    DeviceOperational,

    /// Can new work be accepted/executed?
    Execution,

    /// Is queue service operational?
    Queue,

    /// Can execution results be retrieved?
    ResultRetrieval,

    /// Is usable calibration information available?
    Calibration,

    /// Is topology information available and valid?
    Topology,

    /// Is timing information available and valid?
    Timing,

    /// Is the provider service operational?
    ProviderService,

    /// Are required execution resources currently available?
    ResourceAvailability,

    /// Is local runtime infrastructure operational?
    Runtime,

    /// Is the provider's job service operational?
    JobService,

    /// Is cancellation service operational?
    Cancellation,
}

impl HealthCheck {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reachability => "reachability",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::ApiCompatibility => "api_compatibility",
            Self::DeviceOperational => "device_operational",
            Self::Execution => "execution",
            Self::Queue => "queue",
            Self::ResultRetrieval => "result_retrieval",
            Self::Calibration => "calibration",
            Self::Topology => "topology",
            Self::Timing => "timing",
            Self::ProviderService => "provider_service",
            Self::ResourceAvailability => "resource_availability",
            Self::Runtime => "runtime",
            Self::JobService => "job_service",
            Self::Cancellation => "cancellation",
        }
    }

    /// Returns the canonical check order.
    ///
    /// This is used to guarantee deterministic report ordering.
    pub const fn canonical_order() -> &'static [Self] {
        &[
            Self::Reachability,
            Self::Authentication,
            Self::Authorization,
            Self::ApiCompatibility,
            Self::ProviderService,
            Self::DeviceOperational,
            Self::Runtime,
            Self::ResourceAvailability,
            Self::Execution,
            Self::JobService,
            Self::Queue,
            Self::ResultRetrieval,
            Self::Cancellation,
            Self::Calibration,
            Self::Topology,
            Self::Timing,
        ]
    }
}

impl fmt::Display for HealthCheck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Unix timestamp represented as nanoseconds since the Unix epoch.
///
/// The type is deliberately independent of `SystemTime` so health records can
/// be transported between machines and providers.
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
pub struct HealthTimestamp {
    nanos_since_epoch: u64,
}

impl HealthTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(nanos_since_epoch: u64) -> Self {
        Self { nanos_since_epoch }
    }

    /// Returns Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u64 {
        self.nanos_since_epoch
    }

    /// Returns the current system timestamp.
    ///
    /// This is an explicit convenience method. Core health validation never
    /// obtains the system clock implicitly.
    pub fn now() -> Result<Self, HealthError> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| HealthError::ClockBeforeUnixEpoch)?;

        let seconds = duration.as_secs();

        let nanos = seconds
            .checked_mul(NANOS_PER_SECOND)
            .and_then(|value| value.checked_add(u64::from(duration.subsec_nanos())))
            .ok_or(HealthError::TimestampOverflow)?;

        Ok(Self::from_unix_nanos(nanos))
    }

    /// Returns the age of this timestamp relative to `now`.
    ///
    /// A timestamp in the future has age zero rather than producing a negative
    /// duration.
    pub const fn age_since(
        self,
        now: HealthTimestamp,
    ) -> u64 {
        now.as_unix_nanos()
            .saturating_sub(self.as_unix_nanos())
    }
}

impl fmt::Display for HealthTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}ns", self.nanos_since_epoch)
    }
}

// =============================================================================
// Target reference
// =============================================================================

/// Provider-neutral reference to the target whose health is being measured.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HealthTarget {
    /// Stable provider identifier.
    pub provider_id: String,

    /// Stable backend/device identifier.
    pub target_id: String,
}

impl HealthTarget {
    /// Creates and validates a target reference.
    pub fn new(
        provider_id: impl Into<String>,
        target_id: impl Into<String>,
    ) -> Result<Self, HealthError> {
        let provider_id = provider_id.into();
        let target_id = target_id.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        validate_identifier(
            "target_id",
            &target_id,
            MAX_TARGET_ID_LENGTH,
        )?;

        Ok(Self {
            provider_id,
            target_id,
        })
    }
}

// =============================================================================
// Diagnostic
// =============================================================================

/// Structured explanation of a health condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthDiagnostic {
    /// Stable diagnostic code.
    pub code: String,

    /// Severity.
    pub severity: HealthSeverity,

    /// Human-readable message.
    pub message: String,

    /// Optional remediation guidance.
    pub remediation: Option<String>,

    /// Optional provider-native error code.
    ///
    /// This must never contain credentials.
    pub provider_code: Option<String>,

    /// Whether retrying the operation may succeed.
    pub retryable: bool,
}

impl HealthDiagnostic {
    /// Creates a diagnostic.
    pub fn new(
        code: impl Into<String>,
        severity: HealthSeverity,
        message: impl Into<String>,
    ) -> Result<Self, HealthError> {
        Self::with_details(
            code,
            severity,
            message,
            None,
            None,
            false,
        )
    }

    /// Creates a fully specified diagnostic.
    pub fn with_details(
        code: impl Into<String>,
        severity: HealthSeverity,
        message: impl Into<String>,
        remediation: Option<String>,
        provider_code: Option<String>,
        retryable: bool,
    ) -> Result<Self, HealthError> {
        let code = code.into();
        let message = message.into();

        validate_identifier(
            "diagnostic_code",
            &code,
            MAX_DIAGNOSTIC_CODE_LENGTH,
        )?;

        if message.trim().is_empty() {
            return Err(HealthError::EmptyField {
                field: "diagnostic_message",
            });
        }

        if message.len() > MAX_DIAGNOSTIC_MESSAGE_LENGTH {
            return Err(HealthError::FieldTooLong {
                field: "diagnostic_message",
                length: message.len(),
                maximum: MAX_DIAGNOSTIC_MESSAGE_LENGTH,
            });
        }

        reject_secret_like_text(&message)?;

        if let Some(value) = remediation.as_deref() {
            if value.len() > MAX_DIAGNOSTIC_MESSAGE_LENGTH {
                return Err(HealthError::FieldTooLong {
                    field: "diagnostic_remediation",
                    length: value.len(),
                    maximum: MAX_DIAGNOSTIC_MESSAGE_LENGTH,
                });
            }

            reject_secret_like_text(value)?;
        }

        if let Some(value) = provider_code.as_deref() {
            validate_identifier(
                "provider_code",
                value,
                MAX_DIAGNOSTIC_CODE_LENGTH,
            )?;

            reject_secret_like_text(value)?;
        }

        Ok(Self {
            code,
            severity,
            message,
            remediation,
            provider_code,
            retryable,
        })
    }
}

// =============================================================================
// Health observation
// =============================================================================

/// One immutable health-check observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthObservation {
    /// Target being checked.
    pub target: HealthTarget,

    /// Dimension being checked.
    pub check: HealthCheck,

    /// Result of the check.
    pub status: HealthStatus,

    /// Time at which the observation was made.
    pub observed_at: HealthTimestamp,

    /// Optional measured latency in nanoseconds.
    pub latency_ns: Option<u64>,

    /// Diagnostics explaining the observation.
    pub diagnostics: Vec<HealthDiagnostic>,

    /// Additional deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl HealthObservation {
    /// Creates a basic observation.
    pub fn new(
        target: HealthTarget,
        check: HealthCheck,
        status: HealthStatus,
        observed_at: HealthTimestamp,
    ) -> Result<Self, HealthError> {
        Self::with_details(
            target,
            check,
            status,
            observed_at,
            None,
            Vec::new(),
            BTreeMap::new(),
        )
    }

    /// Creates a complete observation.
    pub fn with_details(
        target: HealthTarget,
        check: HealthCheck,
        status: HealthStatus,
        observed_at: HealthTimestamp,
        latency_ns: Option<u64>,
        diagnostics: Vec<HealthDiagnostic>,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, HealthError> {
        if diagnostics.len() > MAX_DIAGNOSTICS {
            return Err(HealthError::DiagnosticLimitExceeded {
                requested: diagnostics.len(),
                maximum: MAX_DIAGNOSTICS,
            });
        }

        validate_diagnostics(&diagnostics)?;

        validate_metadata(&metadata)?;

        if matches!(status, HealthStatus::Healthy)
            && diagnostics.iter().any(|diagnostic| {
                matches!(
                    diagnostic.severity,
                    HealthSeverity::Error | HealthSeverity::Critical
                )
            })
        {
            return Err(HealthError::InconsistentObservation {
                message: "healthy observation cannot contain error or critical diagnostics",
            });
        }

        if matches!(status, HealthStatus::Failed | HealthStatus::Unavailable)
            && diagnostics.is_empty()
        {
            return Err(HealthError::MissingFailureDiagnostic);
        }

        Ok(Self {
            target,
            check,
            status,
            observed_at,
            latency_ns,
            diagnostics,
            metadata,
        })
    }

    /// Returns the highest diagnostic severity in the observation.
    pub fn diagnostic_severity(&self) -> HealthSeverity {
        self.diagnostics
            .iter()
            .fold(HealthSeverity::Info, |current, diagnostic| {
                current.max(diagnostic.severity)
            })
    }

    /// Returns true when this observation is older than `maximum_age_ns`.
    pub const fn is_stale(
        &self,
        now: HealthTimestamp,
        maximum_age_ns: u64,
    ) -> bool {
        self.observed_at.age_since(now) > maximum_age_ns
    }
}

// =============================================================================
// Freshness policy
// =============================================================================

/// Policy controlling how old a health observation may be before it is
/// considered stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthFreshnessPolicy {
    /// Maximum permitted age in nanoseconds.
    pub maximum_age_ns: u64,

    /// Whether a stale observation should be treated as unsafe.
    pub stale_is_blocking: bool,
}

impl HealthFreshnessPolicy {
    /// Creates a freshness policy.
    pub const fn new(
        maximum_age_ns: u64,
        stale_is_blocking: bool,
    ) -> Self {
        Self {
            maximum_age_ns,
            stale_is_blocking,
        }
    }

    /// Policy requiring observations to be no older than one minute.
    pub const fn one_minute() -> Self {
        Self::new(
            60 * NANOS_PER_SECOND,
            true,
        )
    }

    /// Policy requiring observations to be no older than five minutes.
    pub const fn five_minutes() -> Self {
        Self::new(
            5 * 60 * NANOS_PER_SECOND,
            true,
        )
    }

    /// Policy requiring observations to be no older than one hour.
    pub const fn one_hour() -> Self {
        Self::new(
            60 * 60 * NANOS_PER_SECOND,
            true,
        )
    }

    /// Validates the policy.
    pub const fn validate(self) -> Result<(), HealthError> {
        if self.maximum_age_ns > MAX_FRESHNESS_AGE_NS {
            return Err(HealthError::InvalidFreshnessPolicy);
        }

        Ok(())
    }

    /// Checks an observation for freshness.
    pub const fn check(
        self,
        observation: &HealthObservation,
        now: HealthTimestamp,
    ) -> Result<(), HealthError> {
        let age_ns = observation.observed_at.age_since(now);

        if age_ns > self.maximum_age_ns {
            return Err(HealthError::StaleHealth {
                age_ns,
                maximum_age_ns: self.maximum_age_ns,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Health report
// =============================================================================

/// Complete immutable health report for one execution target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthReport {
    /// Schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Target whose health was measured.
    pub target: HealthTarget,

    /// Aggregate health state.
    pub status: HealthStatus,

    /// Time at which this report was generated.
    pub generated_at: HealthTimestamp,

    /// Time of the newest underlying observation.
    pub observed_at: HealthTimestamp,

    /// Individual health observations.
    pub observations: BTreeMap<HealthCheck, HealthObservation>,

    /// Aggregate diagnostics.
    pub diagnostics: Vec<HealthDiagnostic>,

    /// Deterministic report metadata.
    pub metadata: BTreeMap<String, String>,
}

impl HealthReport {
    /// Creates an empty report.
    ///
    /// Empty reports are intentionally `Unknown`; callers must add actual
    /// observations before treating health as authoritative.
    pub fn empty(
        target: HealthTarget,
        generated_at: HealthTimestamp,
    ) -> Self {
        Self {
            schema_id: HEALTH_SCHEMA_ID.to_owned(),
            schema_version: HEALTH_SCHEMA_VERSION,
            target,
            status: HealthStatus::Unknown,
            generated_at,
            observed_at: generated_at,
            observations: BTreeMap::new(),
            diagnostics: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a report from validated observations.
    pub fn from_observations(
        target: HealthTarget,
        generated_at: HealthTimestamp,
        observations: Vec<HealthObservation>,
    ) -> Result<Self, HealthError> {
        if observations.is_empty() {
            return Err(HealthError::EmptyReport);
        }

        if observations.len() > MAX_OBSERVATIONS {
            return Err(HealthError::ObservationLimitExceeded {
                requested: observations.len(),
                maximum: MAX_OBSERVATIONS,
            });
        }

        let mut map = BTreeMap::new();

        for observation in observations {
            if observation.target != target {
                return Err(HealthError::TargetMismatch);
            }

            if map.insert(observation.check, observation).is_some() {
                return Err(HealthError::DuplicateCheck {
                    check: "health check already exists",
                });
            }
        }

        let status = aggregate_status(map.values());

        let observed_at = map
            .values()
            .map(|observation| observation.observed_at)
            .max()
            .unwrap_or(generated_at);

        let diagnostics = aggregate_diagnostics(map.values())?;

        let report = Self {
            schema_id: HEALTH_SCHEMA_ID.to_owned(),
            schema_version: HEALTH_SCHEMA_VERSION,
            target,
            status,
            generated_at,
            observed_at,
            observations: map,
            diagnostics,
            metadata: BTreeMap::new(),
        };

        report.validate()?;

        Ok(report)
    }

    /// Validates the complete report.
    pub fn validate(&self) -> Result<(), HealthError> {
        if self.schema_id != HEALTH_SCHEMA_ID {
            return Err(HealthError::InvalidSchemaId {
                expected: HEALTH_SCHEMA_ID,
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != HEALTH_SCHEMA_VERSION {
            return Err(HealthError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if self.observations.is_empty() {
            return Err(HealthError::EmptyReport);
        }

        if self.observations.len() > MAX_OBSERVATIONS {
            return Err(HealthError::ObservationLimitExceeded {
                requested: self.observations.len(),
                maximum: MAX_OBSERVATIONS,
            });
        }

        for (check, observation) in &self.observations {
            if *check != observation.check {
                return Err(HealthError::ObservationKeyMismatch);
            }

            if observation.target != self.target {
                return Err(HealthError::TargetMismatch);
            }
        }

        validate_diagnostics(&self.diagnostics)?;
        validate_metadata(&self.metadata)?;

        let expected_status = aggregate_status(self.observations.values());

        if expected_status != self.status {
            return Err(HealthError::InconsistentReportStatus {
                expected: expected_status,
                actual: self.status,
            });
        }

        Ok(())
    }

    /// Returns whether all recorded checks are healthy.
    pub fn is_healthy(&self) -> bool {
        self.status.is_healthy()
    }

    /// Returns whether the target is operational according to this report.
    pub fn is_operational(&self) -> bool {
        self.status.is_operational()
    }

    /// Returns whether this report contains a particular check.
    pub fn contains_check(&self, check: HealthCheck) -> bool {
        self.observations.contains_key(&check)
    }

    /// Returns one observation.
    pub fn observation(
        &self,
        check: HealthCheck,
    ) -> Option<&HealthObservation> {
        self.observations.get(&check)
    }

    /// Returns the age of the newest observation.
    pub const fn age_ns(
        &self,
        now: HealthTimestamp,
    ) -> u64 {
        self.observed_at.age_since(now)
    }

    /// Returns whether this entire report is stale according to a policy.
    pub const fn is_stale(
        &self,
        now: HealthTimestamp,
        policy: HealthFreshnessPolicy,
    ) -> bool {
        self.age_ns(now) > policy.maximum_age_ns
    }

    /// Validates freshness.
    pub const fn require_fresh(
        &self,
        now: HealthTimestamp,
        policy: HealthFreshnessPolicy,
    ) -> Result<(), HealthError> {
        policy.check(
            &HealthObservation {
                target: self.target.clone(),
                check: HealthCheck::ProviderService,
                status: self.status,
                observed_at: self.observed_at,
                latency_ns: None,
                diagnostics: Vec::new(),
                metadata: BTreeMap::new(),
            },
            now,
        )
    }

    /// Returns all failed/degraded/unavailable checks in deterministic order.
    pub fn unhealthy_checks(&self) -> Vec<HealthCheck> {
        self.observations
            .iter()
            .filter_map(|(check, observation)| {
                if observation.status.is_healthy() {
                    None
                } else {
                    Some(*check)
                }
            })
            .collect()
    }

    /// Adds metadata while preserving report validity.
    ///
    /// This consumes and returns the report, keeping the public report
    /// effectively immutable once shared.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, HealthError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_entry(&key, &value)?;

        if self.metadata.len() >= MAX_METADATA_FIELDS
            && !self.metadata.contains_key(&key)
        {
            return Err(HealthError::MetadataLimitExceeded {
                requested: self.metadata.len() + 1,
                maximum: MAX_METADATA_FIELDS,
            });
        }

        self.metadata.insert(key, value);
        self.validate()?;

        Ok(self)
    }
}

// =============================================================================
// Health policy
// =============================================================================

/// Policy describing which health dimensions must be healthy before a
/// particular operation is permitted.
///
/// This allows execution, benchmarking, discovery, and Danga to use different
/// operational policies without modifying `HealthReport`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthPolicy {
    /// Checks that must be present.
    pub required_checks: BTreeSet<HealthCheck>,

    /// Checks that may be degraded without blocking.
    pub allow_degraded: BTreeSet<HealthCheck>,

    /// Whether unknown checks are blocking.
    pub unknown_is_blocking: bool,

    /// Whether stale health is blocking.
    pub stale_is_blocking: bool,

    /// Freshness policy.
    pub freshness: HealthFreshnessPolicy,
}

impl HealthPolicy {
    /// Creates a conservative production policy.
    pub fn production() -> Self {
        let mut required_checks = BTreeSet::new();

        required_checks.insert(HealthCheck::Reachability);
        required_checks.insert(HealthCheck::Authentication);
        required_checks.insert(HealthCheck::Authorization);
        required_checks.insert(HealthCheck::ApiCompatibility);
        required_checks.insert(HealthCheck::DeviceOperational);
        required_checks.insert(HealthCheck::Execution);
        required_checks.insert(HealthCheck::ResultRetrieval);

        Self {
            required_checks,
            allow_degraded: BTreeSet::new(),
            unknown_is_blocking: true,
            stale_is_blocking: true,
            freshness: HealthFreshnessPolicy::five_minutes(),
        }
    }

    /// Creates a discovery-oriented policy.
    pub fn discovery() -> Self {
        let mut required_checks = BTreeSet::new();

        required_checks.insert(HealthCheck::Reachability);
        required_checks.insert(HealthCheck::ApiCompatibility);
        required_checks.insert(HealthCheck::ProviderService);

        Self {
            required_checks,
            allow_degraded: BTreeSet::from([
                HealthCheck::ProviderService,
            ]),
            unknown_is_blocking: false,
            stale_is_blocking: false,
            freshness: HealthFreshnessPolicy::one_hour(),
        }
    }

    /// Creates a benchmarking policy.
    ///
    /// Benchmarking itself remains outside this module. This policy merely
    /// describes the health evidence required before a benchmark may execute.
    pub fn benchmarking() -> Self {
        let mut required_checks = HealthPolicy::production()
            .required_checks;

        required_checks.insert(HealthCheck::Calibration);
        required_checks.insert(HealthCheck::Timing);

        Self {
            required_checks,
            allow_degraded: BTreeSet::new(),
            unknown_is_blocking: true,
            stale_is_blocking: true,
            freshness: HealthFreshnessPolicy::five_minutes(),
        }
    }

    /// Evaluates a report against this policy.
    pub fn evaluate(
        &self,
        report: &HealthReport,
        now: HealthTimestamp,
    ) -> HealthEvaluation {
        let mut diagnostics = Vec::new();

        if report.is_stale(now, self.freshness) {
            diagnostics.push(
                HealthDiagnostic::with_details(
                    "health.stale",
                    if self.stale_is_blocking {
                        HealthSeverity::Critical
                    } else {
                        HealthSeverity::Warning
                    },
                    "health report is older than the permitted freshness window",
                    Some(
                        "refresh the target health report before proceeding"
                            .to_owned(),
                    ),
                    None,
                    true,
                )
                .expect("static health diagnostic must be valid"),
            );
        }

        for check in &self.required_checks {
            match report.observation(*check) {
                None => {
                    diagnostics.push(
                        HealthDiagnostic::with_details(
                            "health.check_missing",
                            if self.unknown_is_blocking {
                                HealthSeverity::Critical
                            } else {
                                HealthSeverity::Warning
                            },
                            format!(
                                "required health check `{}` is missing",
                                check.as_str()
                            ),
                            Some(
                                "perform the required health check before \
                                 proceeding"
                                    .to_owned(),
                            ),
                            None,
                            true,
                        )
                        .expect("static health diagnostic must be valid"),
                    );
                }

                Some(observation) => {
                    match observation.status {
                        HealthStatus::Healthy => {}

                        HealthStatus::Degraded
                            if self.allow_degraded.contains(check) => {}

                        HealthStatus::Unknown => {
                            diagnostics.push(
                                HealthDiagnostic::with_details(
                                    "health.check_unknown",
                                    if self.unknown_is_blocking {
                                        HealthSeverity::Critical
                                    } else {
                                        HealthSeverity::Warning
                                    },
                                    format!(
                                        "required health check `{}` is unknown",
                                        check.as_str()
                                    ),
                                    Some(
                                        "obtain an authoritative health \
                                         observation"
                                            .to_owned(),
                                    ),
                                    None,
                                    true,
                                )
                                .expect("static health diagnostic must be valid"),
                            );
                        }

                        status => {
                            diagnostics.push(
                                HealthDiagnostic::with_details(
                                    "health.check_unhealthy",
                                    HealthSeverity::Critical,
                                    format!(
                                        "required health check `{}` is `{}`",
                                        check.as_str(),
                                        status.as_str()
                                    ),
                                    Some(
                                        "resolve the reported health issue \
                                         before execution"
                                            .to_owned(),
                                    ),
                                    None,
                                    status != HealthStatus::Retired,
                                )
                                .expect("static health diagnostic must be valid"),
                            );
                        }
                    }
                }
            }
        }

        let allowed = diagnostics.iter().all(|diagnostic| {
            diagnostic.severity == HealthSeverity::Info
                || diagnostic.severity == HealthSeverity::Warning
        });

        HealthEvaluation {
            allowed,
            report_status: report.status,
            diagnostics,
        }
    }
}

// =============================================================================
// Health evaluation
// =============================================================================

/// Result of applying a `HealthPolicy` to a `HealthReport`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthEvaluation {
    /// Whether the operation may proceed under the selected policy.
    pub allowed: bool,

    /// Aggregate report state.
    pub report_status: HealthStatus,

    /// Policy-specific diagnostics.
    pub diagnostics: Vec<HealthDiagnostic>,
}

impl HealthEvaluation {
    /// Returns true if execution is permitted.
    pub const fn is_allowed(&self) -> bool {
        self.allowed
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors generated while constructing or validating health state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthError {
    /// Required field was empty.
    EmptyField {
        field: &'static str,
    },

    /// Field exceeded its permitted size.
    FieldTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// Identifier contains invalid characters/format.
    InvalidIdentifier {
        field: &'static str,
    },

    /// Health diagnostic appears to contain a secret.
    SecretLikeData {
        field: &'static str,
    },

    /// Too many diagnostics.
    DiagnosticLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many observations.
    ObservationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many metadata entries.
    MetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Empty report.
    EmptyReport,

    /// Same health check appeared more than once.
    DuplicateCheck {
        check: &'static str,
    },

    /// Observation belongs to another target.
    TargetMismatch,

    /// Observation map key differs from observation check.
    ObservationKeyMismatch,

    /// Aggregate status differs from underlying observations.
    InconsistentReportStatus {
        expected: HealthStatus,
        actual: HealthStatus,
    },

    /// Observation contains logically contradictory state.
    InconsistentObservation {
        message: &'static str,
    },

    /// Failed/unavailable state did not contain an explanation.
    MissingFailureDiagnostic,

    /// Schema ID does not match.
    InvalidSchemaId {
        expected: &'static str,
        actual: String,
    },

    /// Schema version is not supported.
    UnsupportedSchemaVersion {
        version: u16,
    },

    /// Freshness policy is invalid.
    InvalidFreshnessPolicy,

    /// Observation is stale.
    StaleHealth {
        age_ns: u64,
        maximum_age_ns: u64,
    },

    /// System clock predates Unix epoch.
    ClockBeforeUnixEpoch,

    /// Timestamp arithmetic overflowed.
    TimestampOverflow,

    /// Invalid metadata.
    InvalidMetadata {
        key: String,
    },
}

impl fmt::Display for HealthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::FieldTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {length} bytes long; maximum is {maximum}"
                )
            }

            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid identifier for {field}")
            }

            Self::SecretLikeData { field } => {
                write!(
                    formatter,
                    "{field} appears to contain secret-bearing data"
                )
            }

            Self::DiagnosticLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "health report contains {requested} diagnostics; \
                     maximum is {maximum}"
                )
            }

            Self::ObservationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "health report contains {requested} observations; \
                     maximum is {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "health metadata contains {requested} entries; \
                     maximum is {maximum}"
                )
            }

            Self::EmptyReport => {
                formatter.write_str(
                    "health report must contain at least one observation",
                )
            }

            Self::DuplicateCheck { check } => {
                write!(formatter, "duplicate health check: {check}")
            }

            Self::TargetMismatch => {
                formatter.write_str(
                    "health observation belongs to a different target",
                )
            }

            Self::ObservationKeyMismatch => {
                formatter.write_str(
                    "health observation map key does not match observation",
                )
            }

            Self::InconsistentReportStatus { expected, actual } => {
                write!(
                    formatter,
                    "inconsistent health report status: expected \
                     {expected}, got {actual}"
                )
            }

            Self::InconsistentObservation { message } => {
                write!(formatter, "inconsistent health observation: {message}")
            }

            Self::MissingFailureDiagnostic => {
                formatter.write_str(
                    "failed or unavailable health state requires a diagnostic",
                )
            }

            Self::InvalidSchemaId { expected, actual } => {
                write!(
                    formatter,
                    "invalid health schema ID: expected `{expected}`, \
                     got `{actual}`"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported health schema version {version}"
                )
            }

            Self::InvalidFreshnessPolicy => {
                formatter.write_str("invalid health freshness policy")
            }

            Self::StaleHealth {
                age_ns,
                maximum_age_ns,
            } => {
                write!(
                    formatter,
                    "health observation is stale: age={age_ns}ns, \
                     maximum={maximum_age_ns}ns"
                )
            }

            Self::ClockBeforeUnixEpoch => {
                formatter.write_str("system clock is before Unix epoch")
            }

            Self::TimestampOverflow => {
                formatter.write_str("health timestamp arithmetic overflow")
            }

            Self::InvalidMetadata { key } => {
                write!(formatter, "invalid health metadata key `{key}`")
            }
        }
    }
}

impl std::error::Error for HealthError {}

// =============================================================================
// Internal validation
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HealthError> {
    if value.trim().is_empty() {
        return Err(HealthError::EmptyField { field });
    }

    if value.len() > maximum {
        return Err(HealthError::FieldTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(HealthError::InvalidIdentifier { field });
    }

    Ok(())
}

fn validate_diagnostics(
    diagnostics: &[HealthDiagnostic],
) -> Result<(), HealthError> {
    if diagnostics.len() > MAX_DIAGNOSTICS {
        return Err(HealthError::DiagnosticLimitExceeded {
            requested: diagnostics.len(),
            maximum: MAX_DIAGNOSTICS,
        });
    }

    let mut codes = BTreeSet::new();

    for diagnostic in diagnostics {
        validate_identifier(
            "diagnostic_code",
            &diagnostic.code,
            MAX_DIAGNOSTIC_CODE_LENGTH,
        )?;

        if !codes.insert(diagnostic.code.clone()) {
            return Err(HealthError::DuplicateCheck {
                check: "duplicate diagnostic code",
            });
        }

        if diagnostic.message.trim().is_empty() {
            return Err(HealthError::EmptyField {
                field: "diagnostic_message",
            });
        }

        reject_secret_like_text(&diagnostic.message)?;

        if let Some(remediation) = diagnostic.remediation.as_deref() {
            reject_secret_like_text(remediation)?;
        }

        if let Some(provider_code) = diagnostic.provider_code.as_deref() {
            reject_secret_like_text(provider_code)?;
        }
    }

    Ok(())
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<(), HealthError> {
    if metadata.len() > MAX_METADATA_FIELDS {
        return Err(HealthError::MetadataLimitExceeded {
            requested: metadata.len(),
            maximum: MAX_METADATA_FIELDS,
        });
    }

    for (key, value) in metadata {
        validate_metadata_entry(key, value)?;
    }

    Ok(())
}

fn validate_metadata_entry(
    key: &str,
    value: &str,
) -> Result<(), HealthError> {
    if key.trim().is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(HealthError::InvalidMetadata {
            key: key.to_owned(),
        });
    }

    if key.chars().any(|character| character.is_control()) {
        return Err(HealthError::InvalidMetadata {
            key: key.to_owned(),
        });
    }

    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(HealthError::FieldTooLong {
            field: "health_metadata_value",
            length: value.len(),
            maximum: MAX_METADATA_VALUE_LENGTH,
        });
    }

    reject_secret_like_text(value)?;

    Ok(())
}

fn reject_secret_like_text(value: &str) -> Result<(), HealthError> {
    let normalized = value.to_ascii_lowercase();

    const SECRET_MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "authorization:",
        "bearer ",
        "password",
        "private_key",
        "private-key",
        "secret_key",
        "secret-key",
        "session_cookie",
        "session-cookie",
        "client_secret",
        "client-secret",
    ];

    if SECRET_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        return Err(HealthError::SecretLikeData {
            field: "health_text",
        });
    }

    Ok(())
}

fn aggregate_status<'a, I>(
    observations: I,
) -> HealthStatus
where
    I: IntoIterator<Item = &'a HealthObservation>,
{
    let mut status = HealthStatus::Healthy;

    for observation in observations {
        status = status.max_severity(observation.status);

        if observation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == HealthSeverity::Critical)
            && status.severity_rank() < HealthStatus::Failed.severity_rank()
        {
            status = HealthStatus::Failed;
        }
    }

    status
}

fn aggregate_diagnostics<'a, I>(
    observations: I,
) -> Result<Vec<HealthDiagnostic>, HealthError>
where
    I: IntoIterator<Item = &'a HealthObservation>,
{
    let mut diagnostics = Vec::new();

    for observation in observations {
        diagnostics.extend(observation.diagnostics.iter().cloned());
    }

    diagnostics.sort_by(|left, right| {
        left.code
            .cmp(&right.code)
            .then_with(|| left.severity.cmp(&right.severity))
            .then_with(|| left.message.cmp(&right.message))
    });

    validate_diagnostics(&diagnostics)?;

    Ok(diagnostics)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn target() -> HealthTarget {
        HealthTarget::new(
            "local",
            "local://simulator/statevector",
        )
        .expect("test target must be valid")
    }

    fn timestamp(value: u64) -> HealthTimestamp {
        HealthTimestamp::from_unix_nanos(value)
    }

    fn healthy_observation(
        check: HealthCheck,
    ) -> HealthObservation {
        HealthObservation::new(
            target(),
            check,
            HealthStatus::Healthy,
            timestamp(1_000_000_000),
        )
        .expect("healthy observation must be valid")
    }

    #[test]
    fn schema_constants_are_stable() {
        assert_eq!(
            HEALTH_SCHEMA_ID,
            "zamani.quantum.hardware.health"
        );
        assert_eq!(HEALTH_SCHEMA_VERSION, 1);
    }

    #[test]
    fn health_status_order_is_deterministic() {
        assert!(
            HealthStatus::Healthy.severity_rank()
                < HealthStatus::Degraded.severity_rank()
        );

        assert!(
            HealthStatus::Degraded.severity_rank()
                < HealthStatus::Failed.severity_rank()
        );

        assert!(
            HealthStatus::Failed.severity_rank()
                < HealthStatus::Retired.severity_rank()
        );
    }

    #[test]
    fn health_target_rejects_empty_provider() {
        let result = HealthTarget::new(
            "",
            "device",
        );

        assert!(result.is_err());
    }

    #[test]
    fn health_target_rejects_empty_target() {
        let result = HealthTarget::new(
            "provider",
            "",
        );

        assert!(result.is_err());
    }

    #[test]
    fn diagnostic_rejects_secret_like_text() {
        let result = HealthDiagnostic::new(
            "authentication.failure",
            HealthSeverity::Error,
            "authorization: Bearer abc123",
        );

        assert!(matches!(
            result,
            Err(HealthError::SecretLikeData { .. })
        ));
    }

    #[test]
    fn healthy_observation_is_valid_without_diagnostics() {
        let observation =
            healthy_observation(HealthCheck::Reachability);

        assert_eq!(
            observation.status,
            HealthStatus::Healthy
        );
    }

    #[test]
    fn failed_observation_requires_diagnostic() {
        let result = HealthObservation::new(
            target(),
            HealthCheck::Reachability,
            HealthStatus::Failed,
            timestamp(1_000_000_000),
        );

        assert!(matches!(
            result,
            Err(HealthError::MissingFailureDiagnostic)
        ));
    }

    #[test]
    fn healthy_observation_rejects_error_diagnostic() {
        let diagnostic = HealthDiagnostic::new(
            "test.error",
            HealthSeverity::Error,
            "service is broken",
        )
        .expect("diagnostic must be valid");

        let result = HealthObservation::with_details(
            target(),
            HealthCheck::Reachability,
            HealthStatus::Healthy,
            timestamp(1_000_000_000),
            None,
            vec![diagnostic],
            BTreeMap::new(),
        );

        assert!(matches!(
            result,
            Err(HealthError::InconsistentObservation { .. })
        ));
    }

    #[test]
    fn report_requires_observations() {
        let report =
            HealthReport::empty(
                target(),
                timestamp(1_000_000_000),
            );

        assert!(matches!(
            report.validate(),
            Err(HealthError::EmptyReport)
        ));
    }

    #[test]
    fn report_aggregates_healthy_observations() {
        let observations = vec![
            healthy_observation(
                HealthCheck::Reachability,
            ),
            healthy_observation(
                HealthCheck::Execution,
            ),
        ];

        let report = HealthReport::from_observations(
            target(),
            timestamp(2_000_000_000),
            observations,
        )
        .expect("report must be valid");

        assert_eq!(
            report.status,
            HealthStatus::Healthy
        );

        assert!(report.is_healthy());
        assert!(report.is_operational());
    }

    #[test]
    fn report_detects_unhealthy_observation() {
        let diagnostic = HealthDiagnostic::new(
            "execution.unavailable",
            HealthSeverity::Critical,
            "execution service is unavailable",
        )
        .expect("diagnostic must be valid");

        let observation =
            HealthObservation::with_details(
                target(),
                HealthCheck::Execution,
                HealthStatus::Unavailable,
                timestamp(1_000_000_000),
                None,
                vec![diagnostic],
                BTreeMap::new(),
            )
            .expect("observation must be valid");

        let report = HealthReport::from_observations(
            target(),
            timestamp(2_000_000_000),
            vec![
                healthy_observation(
                    HealthCheck::Reachability,
                ),
                observation,
            ],
        )
        .expect("report must be valid");

        assert_eq!(
            report.status,
            HealthStatus::Failed
        );

        assert!(!report.is_healthy());
        assert!(!report.is_operational());
    }

    #[test]
    fn report_rejects_duplicate_checks() {
        let result = HealthReport::from_observations(
            target(),
            timestamp(2_000_000_000),
            vec![
                healthy_observation(
                    HealthCheck::Reachability,
                ),
                healthy_observation(
                    HealthCheck::Reachability,
                ),
            ],
        );

        assert!(matches!(
            result,
            Err(HealthError::DuplicateCheck { .. })
        ));
    }

    #[test]
    fn report_rejects_mixed_targets() {
        let other_target = HealthTarget::new(
            "other-provider",
            "other-device",
        )
        .expect("other target must be valid");

        let observation =
            HealthObservation::new(
                other_target,
                HealthCheck::Execution,
                HealthStatus::Healthy,
                timestamp(1_000_000_000),
            )
            .expect("observation must be valid");

        let result =
            HealthReport::from_observations(
                target(),
                timestamp(2_000_000_000),
                vec![observation],
            );

        assert!(matches!(
            result,
            Err(HealthError::TargetMismatch)
        ));
    }

    #[test]
    fn freshness_detects_stale_report() {
        let report =
            HealthReport::from_observations(
                target(),
                timestamp(2_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                ],
            )
            .expect("report must be valid");

        assert!(
            report.is_stale(
                timestamp(10_000_000_000),
                1_000_000,
            )
        );
    }

    #[test]
    fn freshness_accepts_recent_report() {
        let report =
            HealthReport::from_observations(
                target(),
                timestamp(2_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                ],
            )
            .expect("report must be valid");

        assert!(
            !report.is_stale(
                timestamp(2_500_000_000),
                1_000_000_000,
            )
        );
    }

    #[test]
    fn production_policy_requires_core_checks() {
        let policy = HealthPolicy::production();

        assert!(
            policy
                .required_checks
                .contains(&HealthCheck::Reachability)
        );

        assert!(
            policy
                .required_checks
                .contains(&HealthCheck::Execution)
        );

        assert!(
            policy
                .required_checks
                .contains(&HealthCheck::ResultRetrieval)
        );
    }

    #[test]
    fn production_policy_rejects_missing_required_checks() {
        let report =
            HealthReport::from_observations(
                target(),
                timestamp(1_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                ],
            )
            .expect("report must be valid");

        let evaluation =
            HealthPolicy::production().evaluate(
                &report,
                timestamp(1_000_000_000),
            );

        assert!(!evaluation.allowed);
        assert!(!evaluation.diagnostics.is_empty());
    }

    #[test]
    fn discovery_policy_is_less_strict() {
        let report =
            HealthReport::from_observations(
                target(),
                timestamp(1_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                    healthy_observation(
                        HealthCheck::ApiCompatibility,
                    ),
                    healthy_observation(
                        HealthCheck::ProviderService,
                    ),
                ],
            )
            .expect("report must be valid");

        let evaluation =
            HealthPolicy::discovery().evaluate(
                &report,
                timestamp(1_000_000_000),
            );

        assert!(evaluation.allowed);
    }

    #[test]
    fn benchmarking_policy_requires_calibration_and_timing() {
        let policy = HealthPolicy::benchmarking();

        assert!(
            policy
                .required_checks
                .contains(&HealthCheck::Calibration)
        );

        assert!(
            policy
                .required_checks
                .contains(&HealthCheck::Timing)
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let report =
            HealthReport::from_observations(
                target(),
                timestamp(1_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                ],
            )
            .expect("report must be valid")
            .with_metadata(
                "z",
                "last",
            )
            .expect("metadata must be valid")
            .with_metadata(
                "a",
                "first",
            )
            .expect("metadata must be valid");

        let keys: Vec<&String> =
            report.metadata.keys().collect();

        assert_eq!(
            keys,
            vec![
                &"a".to_owned(),
                &"z".to_owned(),
            ]
        );
    }

    #[test]
    fn unhealthy_checks_are_deterministic() {
        let diagnostic = HealthDiagnostic::new(
            "execution.failure",
            HealthSeverity::Critical,
            "execution unavailable",
        )
        .expect("diagnostic must be valid");

        let failed =
            HealthObservation::with_details(
                target(),
                HealthCheck::Execution,
                HealthStatus::Failed,
                timestamp(1_000_000_000),
                None,
                vec![diagnostic],
                BTreeMap::new(),
            )
            .expect("observation must be valid");

        let report =
            HealthReport::from_observations(
                target(),
                timestamp(1_000_000_000),
                vec![
                    healthy_observation(
                        HealthCheck::Reachability,
                    ),
                    failed,
                ],
            )
            .expect("report must be valid");

        assert_eq!(
            report.unhealthy_checks(),
            vec![HealthCheck::Execution]
        );
    }

    #[test]
    fn timestamp_age_saturates_for_future_timestamp() {
        let future = timestamp(2_000);
        let now = timestamp(1_000);

        assert_eq!(
            future.age_since(now),
            0
        );
    }

    #[test]
    fn timestamp_age_is_correct() {
        let observation = timestamp(1_000);
        let now = timestamp(5_000);

        assert_eq!(
            observation.age_since(now),
            4_000
        );
    }

    #[test]
    fn health_check_order_is_stable() {
        let order = HealthCheck::canonical_order();

        assert_eq!(
            order.first().copied(),
            Some(HealthCheck::Reachability)
        );

        assert_eq!(
            order.last().copied(),
            Some(HealthCheck::Timing)
        );
    }
}