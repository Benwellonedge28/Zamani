//! Zamani Quantum — Backend Operational Status
//!
//! Production-grade, provider-independent operational state model for
//! quantum-computing execution backends.
//!
//! # Responsibility
//!
//! This module is the authoritative owner of backend operational status.
//!
//! It defines:
//!
//! - backend lifecycle state;
//! - availability semantics;
//! - degradation semantics;
//! - maintenance semantics;
//! - retirement semantics;
//! - status timestamps;
//! - status freshness;
//! - queue-related operational hints;
//! - health score representation;
//! - status transition validation;
//! - status snapshots;
//! - status provenance;
//! - deterministic serialization;
//! - stable machine-readable status identifiers;
//! - status classification helpers.
//!
//! This module deliberately does NOT:
//!
//! - communicate with providers;
//! - perform network requests;
//! - authenticate;
//! - store credentials;
//! - submit quantum jobs;
//! - cancel quantum jobs;
//! - retrieve quantum results;
//! - perform hardware health checks;
//! - own telemetry transport;
//! - own queue scheduling;
//! - own backend capabilities;
//! - own topology;
//! - own calibration;
//! - perform routing;
//! - perform scheduling;
//! - perform benchmarking;
//! - perform simulation;
//! - perform emulation.
//!
//! Those responsibilities belong to other hardware modules.
//!
//! # Architectural position
//!
//! ```text
//! Provider / Local Backend Adapter
//!              |
//!              | observed operational state
//!              v
//!      BackendStatusSnapshot
//!              |
//!       +------+------+------+------+
//!       |      |      |      |      |
//!       v      v      v      v      v
//!     backend health queue execution registry
//!       |      |      |      |      |
//!       +------+------+------+------+
//!              |
//!              v
//!       compatibility / Danga / benchmarking
//! ```
//!
//! `backend_status.rs` is therefore an evidence/state representation.
//! It does not decide whether a provider is healthy; a health subsystem
//! observes the provider and constructs a validated status snapshot.
//!
//! # Status versus capability
//!
//! Status and capability are intentionally separate.
//!
//! ```text
//! BackendStatus
//!     = What is happening to the backend now?
//!
//! BackendCapabilities
//!     = What can the backend do?
//!
//! BackendLimits
//!     = What resource envelope does it expose?
//!
//! HardwareTopology
//!     = What physical connectivity exists?
//!
//! CalibrationSnapshot
//!     = What measured hardware properties are currently known?
//! ```
//!
//! A backend may therefore be:
//!
//! ```text
//! status       = Busy
//! capabilities = dynamic circuits supported
//! topology     = fully connected
//! calibration  = current
//! ```
//!
//! Status MUST NOT be used as a substitute for capability negotiation.
//!
//! # Status semantics
//!
//! The authoritative states are:
//!
//! - `Unknown`
//! - `Initializing`
//! - `Available`
//! - `Busy`
//! - `Maintenance`
//! - `Degraded`
//! - `Unavailable`
//! - `Offline`
//! - `Error`
//! - `Retired`
//!
//! These states are intentionally more expressive than a simple
//! available/unavailable boolean.
//!
//! In particular:
//!
//! - `Busy` means the backend is operational but currently occupied;
//! - `Maintenance` means provider-controlled maintenance is occurring;
//! - `Degraded` means the backend is operational but one or more aspects
//!   are below normal operating expectations;
//! - `Unavailable` means execution cannot currently be accepted;
//! - `Offline` means the backend cannot currently be reached or observed;
//! - `Error` means an operational fault has been observed;
//! - `Retired` means the backend is permanently withdrawn;
//! - `Unknown` means no trustworthy status has been established;
//! - `Initializing` means the backend is transitioning toward operational use.
//!
//! # Important distinction: Offline versus Unavailable
//!
//! `Offline` does not necessarily mean the physical device is broken.
//!
//! It means Zamani currently cannot establish a usable connection or
//! authoritative observation.
//!
//! `Unavailable` means the backend is known to be unavailable for execution.
//!
//! This distinction matters for provider failover and retry policies.
//!
//! # Important distinction: Busy versus Degraded
//!
//! `Busy` is normally a scheduling/occupancy state.
//!
//! `Degraded` is a quality/operational-integrity state.
//!
//! A backend may be both busy and degraded in reality, but this status
//! abstraction intentionally represents one primary state at a time.
//! Additional conditions belong in `StatusCondition`.
//!
//! # No implicit time source
//!
//! Status values do not query the system clock.
//!
//! Callers provide timestamps explicitly.
//!
//! This makes:
//!
//! - tests deterministic;
//! - replay deterministic;
//! - distributed systems easier to reason about;
//! - historical status reproducible;
//! - provider timestamps preservable.
//!
//! # Security
//!
//! This module never stores:
//!
//! - API keys;
//! - passwords;
//! - access tokens;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - credential material.
//!
//! Provider-specific error messages MUST be sanitized before being inserted
//! into status metadata.
//!
//! # Determinism
//!
//! All collections use deterministic ordering.
//!
//! No global mutable state is used.
//!
//! No random number generation is used.
//!
//! No network state is read.
//!
//! No environment variables are read.
//!
//! # Serialization
//!
//! The status model intentionally uses only stable, primitive data structures
//! and derives Serde serialization when the crate's Serde dependency is
//! available.
//!
//! Serialized representations contain no credentials.
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
//! # Integration contract
//!
//! This file intentionally has no dependency on other Zamani hardware files.
//!
//! Later modules consume it:
//!
//! - `backend.rs`
//! - `backend_trait.rs`
//! - `backend_config.rs`
//! - `health.rs`
//! - `provider.rs`
//! - `provider_registry.rs`
//! - `device_registry.rs`
//! - `discovery.rs`
//! - `execution.rs`
//! - `job.rs`
//! - `queue.rs`
//! - `telemetry.rs`
//! - adapters;
//! - benchmarking;
//! - Danga.
//!
//! None of those modules should redefine `BackendStatus`.
//!
//! The integration rule is:
//!
//! ```text
//! crate::quantum::hardware::backend_status::BackendStatus
//! ```
//!
//! becomes the sole authoritative status enum.
//!
//! `backend.rs` must re-export or import this type rather than defining a
//! second status enum.
//!
//! # Stability
//!
//! `BackendStatus`, `StatusSeverity`, `StatusConditionKind`,
//! `BackendStatusSnapshot`, and their stable string representations form the
//! provider-neutral status contract.
//!
//! Provider-specific states must be mapped into these states by adapters.
//!
//! Provider-specific state names must never leak into this core module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

// ============================================================================
// Schema
// ============================================================================

/// Stable schema identifier for backend status data.
pub const BACKEND_STATUS_SCHEMA_ID: &str =
    "zamani.quantum.hardware.backend_status";

/// Serialized schema version.
///
/// Increment this only when serialized semantics change incompatibly.
pub const BACKEND_STATUS_SCHEMA_VERSION: u16 = 1;

/// Maximum backend identifier length in UTF-8 bytes.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum provider identifier length in UTF-8 bytes.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum device identifier length in UTF-8 bytes.
pub const MAX_DEVICE_ID_LENGTH: usize = 512;

/// Maximum status message length in UTF-8 bytes.
pub const MAX_STATUS_MESSAGE_LENGTH: usize = 4096;

/// Maximum condition count in one snapshot.
pub const MAX_STATUS_CONDITIONS: usize = 256;

/// Maximum metadata property count.
pub const MAX_STATUS_METADATA_PROPERTIES: usize = 4096;

/// Maximum metadata key length.
pub const MAX_STATUS_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_STATUS_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum status history entries represented by a caller.
pub const MAX_STATUS_HISTORY_ENTRIES: usize = 100_000;

/// Maximum queue depth representable by the normalized model.
///
/// This is intentionally finite so malformed provider responses cannot
/// allocate unbounded resources merely by declaring an enormous queue.
pub const MAX_QUEUE_DEPTH: u64 = 1_000_000_000;

/// Maximum queue position representable by the normalized model.
pub const MAX_QUEUE_POSITION: u64 = 1_000_000_000;

/// Maximum health score.
pub const MAX_HEALTH_SCORE: u16 = 100;

/// Maximum confidence score.
pub const MAX_CONFIDENCE_SCORE: u16 = 100;

// ============================================================================
// Backend status
// ============================================================================

/// Authoritative provider-neutral operational state of a quantum backend.
///
/// The enum represents the primary state only. Additional simultaneous
/// conditions belong in [`StatusCondition`].
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
#[non_exhaustive]
pub enum BackendStatus {
    /// No trustworthy operational observation exists.
    Unknown,

    /// Backend is being initialized or brought online.
    Initializing,

    /// Backend is operational and normally accepting execution requests.
    Available,

    /// Backend is operational but currently occupied or processing work.
    Busy,

    /// Backend is intentionally unavailable because maintenance is occurring.
    Maintenance,

    /// Backend remains operational but one or more operational properties are
    /// below their expected level.
    Degraded,

    /// Backend is known to be unavailable for execution.
    Unavailable,

    /// Backend cannot currently be reached or observed.
    Offline,

    /// An operational error has been observed.
    Error,

    /// Backend has permanently ceased operation.
    Retired,
}

impl BackendStatus {
    /// Returns the stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Initializing => "initializing",
            Self::Available => "available",
            Self::Busy => "busy",
            Self::Maintenance => "maintenance",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::Offline => "offline",
            Self::Error => "error",
            Self::Retired => "retired",
        }
    }

    /// Returns whether normal execution submission is permitted by status
    /// alone.
    ///
    /// This does NOT mean that a workload is compatible with the backend.
    pub const fn is_submission_eligible(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns whether the backend is operational in the broad sense.
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Available
                | Self::Busy
                | Self::Maintenance
                | Self::Degraded
        )
    }

    /// Returns whether the backend is permanently retired.
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns whether the backend is unavailable for normal execution.
    pub const fn is_unavailable(self) -> bool {
        matches!(
            self,
            Self::Unknown
                | Self::Initializing
                | Self::Maintenance
                | Self::Unavailable
                | Self::Offline
                | Self::Error
                | Self::Retired
        )
    }

    /// Returns whether the state normally permits queue placement.
    pub const fn may_accept_queue_submission(self) -> bool {
        matches!(
            self,
            Self::Available | Self::Busy | Self::Degraded
        )
    }

    /// Returns whether the status represents an abnormal condition.
    pub const fn is_abnormal(self) -> bool {
        matches!(
            self,
            Self::Degraded
                | Self::Unavailable
                | Self::Offline
                | Self::Error
        )
    }

    /// Returns a conservative severity classification.
    pub const fn severity(self) -> StatusSeverity {
        match self {
            Self::Unknown => StatusSeverity::Unknown,
            Self::Initializing => StatusSeverity::Info,
            Self::Available => StatusSeverity::Normal,
            Self::Busy => StatusSeverity::Info,
            Self::Maintenance => StatusSeverity::Warning,
            Self::Degraded => StatusSeverity::Warning,
            Self::Unavailable => StatusSeverity::Error,
            Self::Offline => StatusSeverity::Error,
            Self::Error => StatusSeverity::Critical,
            Self::Retired => StatusSeverity::Critical,
        }
    }

    /// Returns whether the status may transition directly to the target
    /// status according to the provider-neutral lifecycle model.
    ///
    /// This validates lifecycle sanity, not provider-specific policy.
    pub const fn can_transition_to(self, target: Self) -> bool {
        if self == target {
            return true;
        }

        match self {
            Self::Unknown => true,

            Self::Initializing => matches!(
                target,
                Self::Available
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Available => matches!(
                target,
                Self::Busy
                    | Self::Maintenance
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Busy => matches!(
                target,
                Self::Available
                    | Self::Maintenance
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Maintenance => matches!(
                target,
                Self::Initializing
                    | Self::Available
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Degraded => matches!(
                target,
                Self::Available
                    | Self::Busy
                    | Self::Maintenance
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Unavailable => matches!(
                target,
                Self::Initializing
                    | Self::Available
                    | Self::Degraded
                    | Self::Maintenance
                    | Self::Offline
                    | Self::Error
                    | Self::Retired
            ),

            Self::Offline => matches!(
                target,
                Self::Initializing
                    | Self::Available
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Error
                    | Self::Retired
            ),

            Self::Error => matches!(
                target,
                Self::Initializing
                    | Self::Available
                    | Self::Degraded
                    | Self::Maintenance
                    | Self::Unavailable
                    | Self::Offline
                    | Self::Retired
            ),

            // Retirement is terminal in the provider-neutral model.
            Self::Retired => false,
        }
    }
}

impl Default for BackendStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for BackendStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Severity
// ============================================================================

/// Provider-neutral status severity.
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
pub enum StatusSeverity {
    /// No trustworthy severity has been established.
    Unknown,

    /// Normal informational condition.
    Info,

    /// Normal operational state.
    Normal,

    /// Condition deserves attention but does not necessarily prevent
    /// execution.
    Warning,

    /// Execution should normally not proceed.
    Error,

    /// Serious operational failure.
    Critical,
}

impl StatusSeverity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Info => "info",
            Self::Normal => "normal",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Returns true if this severity indicates a problem.
    pub const fn is_problematic(self) -> bool {
        matches!(
            self,
            Self::Warning | Self::Error | Self::Critical
        )
    }

    /// Returns a numeric ordering suitable for deterministic comparisons.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::Info => 1,
            Self::Normal => 2,
            Self::Warning => 3,
            Self::Error => 4,
            Self::Critical => 5,
        }
    }
}

impl fmt::Display for StatusSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Condition kind
// ============================================================================

/// Standardized operational condition attached to a backend status.
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
#[non_exhaustive]
pub enum StatusConditionKind {
    /// Queue is currently non-empty.
    QueueActive,

    /// Queue is approaching or exceeding normal capacity.
    QueueCongested,

    /// Backend is undergoing planned maintenance.
    PlannedMaintenance,

    /// Backend is undergoing emergency maintenance.
    EmergencyMaintenance,

    /// Calibration is older than the normal operating window.
    CalibrationStale,

    /// Calibration is unavailable.
    CalibrationUnavailable,

    /// One or more hardware components are degraded.
    HardwareDegraded,

    /// Readout subsystem is degraded.
    ReadoutDegraded,

    /// Control subsystem is degraded.
    ControlDegraded,

    /// Connectivity to the provider is degraded.
    ConnectivityDegraded,

    /// Authentication is currently failing.
    AuthenticationFailure,

    /// Authorization is currently failing.
    AuthorizationFailure,

    /// Provider API is reporting an error.
    ProviderError,

    /// Backend firmware has an operational problem.
    FirmwareError,

    /// Backend is undergoing recovery.
    Recovery,

    /// Backend is rate limited.
    RateLimited,

    /// Backend has exceeded a provider-side resource quota.
    QuotaExceeded,

    /// Backend is reachable but currently refusing work.
    SubmissionBlocked,

    /// Status observation is older than the allowed freshness window.
    ObservationStale,

    /// Backend status source is internally inconsistent.
    Inconsistent,

    /// Provider-specific condition that has no standardized equivalent.
    Other,
}

impl StatusConditionKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueueActive => "queue_active",
            Self::QueueCongested => "queue_congested",
            Self::PlannedMaintenance => "planned_maintenance",
            Self::EmergencyMaintenance => "emergency_maintenance",
            Self::CalibrationStale => "calibration_stale",
            Self::CalibrationUnavailable => "calibration_unavailable",
            Self::HardwareDegraded => "hardware_degraded",
            Self::ReadoutDegraded => "readout_degraded",
            Self::ControlDegraded => "control_degraded",
            Self::ConnectivityDegraded => "connectivity_degraded",
            Self::AuthenticationFailure => "authentication_failure",
            Self::AuthorizationFailure => "authorization_failure",
            Self::ProviderError => "provider_error",
            Self::FirmwareError => "firmware_error",
            Self::Recovery => "recovery",
            Self::RateLimited => "rate_limited",
            Self::QuotaExceeded => "quota_exceeded",
            Self::SubmissionBlocked => "submission_blocked",
            Self::ObservationStale => "observation_stale",
            Self::Inconsistent => "inconsistent",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for StatusConditionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Timestamp
// ============================================================================

/// Non-negative Unix timestamp represented in nanoseconds.
///
/// Nanoseconds are used so status ordering remains deterministic even when
/// multiple observations occur within the same second.
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
pub struct StatusTimestamp(u64);

impl StatusTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(value: u64) -> Self {
        Self(value)
    }

    /// Returns Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u64 {
        self.0
    }

    /// Returns Unix seconds, truncating sub-second precision.
    pub const fn as_unix_seconds(self) -> u64 {
        self.0 / 1_000_000_000
    }
}

impl fmt::Display for StatusTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}ns", self.0)
    }
}

// ============================================================================
// Queue information
// ============================================================================

/// Normalized queue information associated with a backend status observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueStatus {
    /// Current queue depth, when known.
    pub depth: Option<u64>,

    /// Position of the caller's job, when known.
    pub position: Option<u64>,

    /// Estimated wait in milliseconds, when known.
    pub estimated_wait_ms: Option<u64>,

    /// Whether the provider explicitly reported queue information.
    pub authoritative: bool,
}

impl QueueStatus {
    /// Creates an empty queue observation.
    pub const fn unknown() -> Self {
        Self {
            depth: None,
            position: None,
            estimated_wait_ms: None,
            authoritative: false,
        }
    }

    /// Validates queue information.
    pub fn validate(&self) -> Result<(), BackendStatusError> {
        if let Some(depth) = self.depth {
            if depth > MAX_QUEUE_DEPTH {
                return Err(BackendStatusError::QueueDepthTooLarge {
                    value: depth,
                    maximum: MAX_QUEUE_DEPTH,
                });
            }
        }

        if let Some(position) = self.position {
            if position > MAX_QUEUE_POSITION {
                return Err(BackendStatusError::QueuePositionTooLarge {
                    value: position,
                    maximum: MAX_QUEUE_POSITION,
                });
            }
        }

        if let (Some(depth), Some(position)) = (self.depth, self.position) {
            if depth == 0 && position > 0 {
                return Err(BackendStatusError::InvalidQueuePosition {
                    depth,
                    position,
                });
            }

            if depth > 0 && position > depth {
                return Err(BackendStatusError::InvalidQueuePosition {
                    depth,
                    position,
                });
            }
        }

        Ok(())
    }

    /// Returns true if at least one queue field is known.
    pub const fn is_known(&self) -> bool {
        self.depth.is_some()
            || self.position.is_some()
            || self.estimated_wait_ms.is_some()
    }
}

impl Default for QueueStatus {
    fn default() -> Self {
        Self::unknown()
    }
}

// ============================================================================
// Status condition
// ============================================================================

/// Additional condition attached to a status snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusCondition {
    /// Standardized condition category.
    pub kind: StatusConditionKind,

    /// Condition severity.
    pub severity: StatusSeverity,

    /// Whether the condition is currently active.
    pub active: bool,

    /// Optional stable condition identifier.
    pub condition_id: Option<String>,

    /// Optional sanitized human-readable message.
    pub message: Option<String>,
}

impl StatusCondition {
    /// Creates an active condition.
    pub fn active(
        kind: StatusConditionKind,
        severity: StatusSeverity,
    ) -> Self {
        Self {
            kind,
            severity,
            active: true,
            condition_id: None,
            message: None,
        }
    }

    /// Creates an inactive condition.
    pub fn inactive(
        kind: StatusConditionKind,
        severity: StatusSeverity,
    ) -> Self {
        Self {
            kind,
            severity,
            active: false,
            condition_id: None,
            message: None,
        }
    }

    /// Adds a stable condition identifier.
    pub fn with_condition_id(
        mut self,
        condition_id: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let condition_id = condition_id.into();

        validate_identifier(
            "condition_id",
            &condition_id,
            MAX_STATUS_METADATA_KEY_LENGTH,
        )?;

        self.condition_id = Some(condition_id);
        Ok(self)
    }

    /// Adds a sanitized message.
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let message = message.into();

        if message.len() > MAX_STATUS_MESSAGE_LENGTH {
            return Err(BackendStatusError::MessageTooLong {
                length: message.len(),
                maximum: MAX_STATUS_MESSAGE_LENGTH,
            });
        }

        if contains_secret_like_content(&message) {
            return Err(BackendStatusError::SensitiveInformation);
        }

        self.message = Some(message);
        Ok(self)
    }

    /// Validates this condition.
    pub fn validate(&self) -> Result<(), BackendStatusError> {
        if let Some(condition_id) = &self.condition_id {
            validate_identifier(
                "condition_id",
                condition_id,
                MAX_STATUS_METADATA_KEY_LENGTH,
            )?;
        }

        if let Some(message) = &self.message {
            if message.len() > MAX_STATUS_MESSAGE_LENGTH {
                return Err(BackendStatusError::MessageTooLong {
                    length: message.len(),
                    maximum: MAX_STATUS_MESSAGE_LENGTH,
                });
            }

            if contains_secret_like_content(message) {
                return Err(BackendStatusError::SensitiveInformation);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Backend status snapshot
// ============================================================================

/// Immutable operational status observation.
///
/// A snapshot is evidence observed at one point in time. It is not a live
/// connection to the backend and does not automatically refresh.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendStatusSnapshot {
    /// Schema identifier.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Backend identifier.
    pub backend_id: String,

    /// Provider identifier, if the backend belongs to a provider.
    pub provider_id: Option<String>,

    /// Device identifier, if the provider exposes one separately.
    pub device_id: Option<String>,

    /// Primary operational status.
    pub status: BackendStatus,

    /// Status observation timestamp.
    pub observed_at: StatusTimestamp,

    /// Optional source timestamp supplied by the provider.
    pub source_timestamp: Option<StatusTimestamp>,

    /// Optional source identifier.
    pub source: Option<String>,

    /// Whether the source is authoritative.
    pub authoritative: bool,

    /// Optional normalized queue information.
    pub queue: QueueStatus,

    /// Optional health score from 0 to 100.
    pub health_score: Option<u16>,

    /// Confidence in this status observation from 0 to 100.
    pub confidence: u16,

    /// Additional operational conditions.
    pub conditions: Vec<StatusCondition>,

    /// Deterministically ordered sanitized metadata.
    pub metadata: BTreeMap<String, String>,
}

impl BackendStatusSnapshot {
    /// Creates a new status snapshot.
    pub fn new(
        backend_id: impl Into<String>,
        status: BackendStatus,
        observed_at: StatusTimestamp,
    ) -> Result<Self, BackendStatusError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        Ok(Self {
            schema_id: BACKEND_STATUS_SCHEMA_ID.to_owned(),
            schema_version: BACKEND_STATUS_SCHEMA_VERSION,
            backend_id,
            provider_id: None,
            device_id: None,
            status,
            observed_at,
            source_timestamp: None,
            source: None,
            authoritative: false,
            queue: QueueStatus::unknown(),
            health_score: None,
            confidence: 100,
            conditions: Vec::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Adds a provider identifier.
    pub fn with_provider_id(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let provider_id = provider_id.into();

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        self.provider_id = Some(provider_id);
        Ok(self)
    }

    /// Adds a device identifier.
    pub fn with_device_id(
        mut self,
        device_id: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let device_id = device_id.into();

        validate_identifier(
            "device_id",
            &device_id,
            MAX_DEVICE_ID_LENGTH,
        )?;

        self.device_id = Some(device_id);
        Ok(self)
    }

    /// Sets an explicit source timestamp.
    pub fn with_source_timestamp(
        mut self,
        timestamp: StatusTimestamp,
    ) -> Self {
        self.source_timestamp = Some(timestamp);
        self
    }

    /// Marks the observation as authoritative or non-authoritative.
    pub fn with_authoritative(mut self, authoritative: bool) -> Self {
        self.authoritative = authoritative;
        self
    }

    /// Adds a source identifier.
    pub fn with_source(
        mut self,
        source: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let source = source.into();

        validate_identifier(
            "source",
            &source,
            MAX_STATUS_METADATA_KEY_LENGTH,
        )?;

        self.source = Some(source);
        Ok(self)
    }

    /// Sets queue information.
    pub fn with_queue(
        mut self,
        queue: QueueStatus,
    ) -> Result<Self, BackendStatusError> {
        queue.validate()?;
        self.queue = queue;
        Ok(self)
    }

    /// Sets health score.
    pub fn with_health_score(
        mut self,
        score: u16,
    ) -> Result<Self, BackendStatusError> {
        if score > MAX_HEALTH_SCORE {
            return Err(BackendStatusError::HealthScoreOutOfRange {
                value: score,
            });
        }

        self.health_score = Some(score);
        Ok(self)
    }

    /// Sets observation confidence.
    pub fn with_confidence(
        mut self,
        confidence: u16,
    ) -> Result<Self, BackendStatusError> {
        if confidence > MAX_CONFIDENCE_SCORE {
            return Err(BackendStatusError::ConfidenceOutOfRange {
                value: confidence,
            });
        }

        self.confidence = confidence;
        Ok(self)
    }

    /// Adds an operational condition.
    pub fn with_condition(
        mut self,
        condition: StatusCondition,
    ) -> Result<Self, BackendStatusError> {
        condition.validate()?;

        if self.conditions.len() >= MAX_STATUS_CONDITIONS {
            return Err(BackendStatusError::ConditionLimitExceeded {
                maximum: MAX_STATUS_CONDITIONS,
            });
        }

        self.conditions.push(condition);
        Ok(self)
    }

    /// Adds sanitized metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BackendStatusError> {
        let key = key.into();
        let value = value.into();

        validate_metadata(&key, &value)?;

        if !self.metadata.contains_key(&key)
            && self.metadata.len() >= MAX_STATUS_METADATA_PROPERTIES
        {
            return Err(BackendStatusError::MetadataLimitExceeded {
                maximum: MAX_STATUS_METADATA_PROPERTIES,
            });
        }

        self.metadata.insert(key, value);
        Ok(self)
    }

    /// Validates the entire snapshot.
    pub fn validate(&self) -> Result<(), BackendStatusError> {
        if self.schema_id != BACKEND_STATUS_SCHEMA_ID {
            return Err(BackendStatusError::InvalidSchemaId {
                expected: BACKEND_STATUS_SCHEMA_ID.to_owned(),
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != BACKEND_STATUS_SCHEMA_VERSION {
            return Err(BackendStatusError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        validate_identifier(
            "backend_id",
            &self.backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if let Some(provider_id) = &self.provider_id {
            validate_identifier(
                "provider_id",
                provider_id,
                MAX_PROVIDER_ID_LENGTH,
            )?;
        }

        if let Some(device_id) = &self.device_id {
            validate_identifier(
                "device_id",
                device_id,
                MAX_DEVICE_ID_LENGTH,
            )?;
        }

        if let Some(source) = &self.source {
            validate_identifier(
                "source",
                source,
                MAX_STATUS_METADATA_KEY_LENGTH,
            )?;
        }

        self.queue.validate()?;

        if let Some(score) = self.health_score {
            if score > MAX_HEALTH_SCORE {
                return Err(BackendStatusError::HealthScoreOutOfRange {
                    value: score,
                });
            }
        }

        if self.confidence > MAX_CONFIDENCE_SCORE {
            return Err(BackendStatusError::ConfidenceOutOfRange {
                value: self.confidence,
            });
        }

        if self.conditions.len() > MAX_STATUS_CONDITIONS {
            return Err(BackendStatusError::ConditionLimitExceeded {
                maximum: MAX_STATUS_CONDITIONS,
            });
        }

        for condition in &self.conditions {
            condition.validate()?;
        }

        if self.metadata.len() > MAX_STATUS_METADATA_PROPERTIES {
            return Err(BackendStatusError::MetadataLimitExceeded {
                maximum: MAX_STATUS_METADATA_PROPERTIES,
            });
        }

        for (key, value) in &self.metadata {
            validate_metadata(key, value)?;
        }

        if let Some(source_timestamp) = self.source_timestamp {
            if source_timestamp > self.observed_at {
                return Err(BackendStatusError::FutureSourceTimestamp);
            }
        }

        Ok(())
    }

    /// Returns true when this status permits ordinary submission.
    ///
    /// This performs status-only evaluation. Capability compatibility,
    /// calibration validity, authorization, queue policy, and workload
    /// validation must be performed elsewhere.
    pub const fn is_submission_eligible(&self) -> bool {
        self.status.is_submission_eligible()
    }

    /// Returns true if the snapshot contains an active condition of the
    /// requested type.
    pub fn has_active_condition(
        &self,
        kind: StatusConditionKind,
    ) -> bool {
        self.conditions
            .iter()
            .any(|condition| condition.kind == kind && condition.active)
    }

    /// Returns the highest severity among the primary status and active
    /// conditions.
    pub fn effective_severity(&self) -> StatusSeverity {
        self.conditions
            .iter()
            .filter(|condition| condition.active)
            .map(|condition| condition.severity)
            .fold(self.status.severity(), |current, candidate| {
                if candidate.rank() > current.rank() {
                    candidate
                } else {
                    current
                }
            })
    }

    /// Returns true when the status is potentially unsafe for execution.
    ///
    /// This is intentionally conservative.
    pub fn requires_attention(&self) -> bool {
        self.effective_severity().is_problematic()
            || self.confidence < 50
    }

    /// Returns the status age in nanoseconds relative to another observation
    /// timestamp.
    ///
    /// If `now` precedes the observation timestamp, an error is returned
    /// instead of silently producing an invalid age.
    pub fn age_at(
        &self,
        now: StatusTimestamp,
    ) -> Result<u64, BackendStatusError> {
        now.as_unix_nanos()
            .checked_sub(self.observed_at.as_unix_nanos())
            .ok_or(BackendStatusError::FutureObservation)
    }

    /// Determines whether this observation is stale at the supplied
    /// timestamp.
    pub fn is_stale_at(
        &self,
        now: StatusTimestamp,
        maximum_age_ns: u64,
    ) -> Result<bool, BackendStatusError> {
        let age = self.age_at(now)?;
        Ok(age > maximum_age_ns)
    }

    /// Validates freshness against a maximum allowed age.
    pub fn require_fresh_at(
        &self,
        now: StatusTimestamp,
        maximum_age_ns: u64,
    ) -> Result<(), BackendStatusError> {
        let age = self.age_at(now)?;

        if age > maximum_age_ns {
            return Err(BackendStatusError::StaleStatus {
                age_ns: age,
                maximum_age_ns,
            });
        }

        Ok(())
    }

    /// Creates a transition from this snapshot to a new status.
    ///
    /// The old snapshot remains unchanged.
    pub fn transition(
        &self,
        status: BackendStatus,
        observed_at: StatusTimestamp,
    ) -> Result<Self, BackendStatusError> {
        if !self.status.can_transition_to(status) {
            return Err(BackendStatusError::InvalidTransition {
                from: self.status,
                to: status,
            });
        }

        if observed_at < self.observed_at {
            return Err(BackendStatusError::NonMonotonicTimestamp {
                previous: self.observed_at,
                current: observed_at,
            });
        }

        let mut next = self.clone();

        next.status = status;
        next.observed_at = observed_at;

        // Queue position is a point-in-time observation. Carrying it
        // implicitly into a later status would make it appear current when it
        // is not. Therefore transitions clear it unless the caller explicitly
        // attaches a new queue observation.
        next.queue = QueueStatus::unknown();

        next.validate()?;

        Ok(next)
    }

    /// Produces a deterministic status fingerprint input.
    ///
    /// This is NOT a cryptographic signature and does not authenticate the
    /// provider.
    pub fn canonical_summary(&self) -> String {
        let mut output = String::new();

        output.push_str(&self.backend_id);
        output.push('|');
        output.push_str(self.status.as_str());
        output.push('|');
        output.push_str(&self.observed_at.as_unix_nanos().to_string());
        output.push('|');
        output.push_str(&self.confidence.to_string());

        if let Some(provider_id) = &self.provider_id {
            output.push('|');
            output.push_str(provider_id);
        }

        if let Some(device_id) = &self.device_id {
            output.push('|');
            output.push_str(device_id);
        }

        output
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by backend status construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendStatusError {
    /// Required identifier is empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// Identifier exceeds its maximum length.
    IdentifierTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// Identifier contains unsupported characters.
    InvalidIdentifier {
        field: &'static str,
    },

    /// Message exceeds its maximum length.
    MessageTooLong {
        length: usize,
        maximum: usize,
    },

    /// Sensitive information was detected in a status field.
    SensitiveInformation,

    /// Too many conditions were supplied.
    ConditionLimitExceeded {
        maximum: usize,
    },

    /// Too many metadata fields were supplied.
    MetadataLimitExceeded {
        maximum: usize,
    },

    /// Metadata key is invalid.
    InvalidMetadataKey,

    /// Metadata value is too large.
    MetadataValueTooLong {
        length: usize,
        maximum: usize,
    },

    /// Queue depth is beyond the normalized limit.
    QueueDepthTooLarge {
        value: u64,
        maximum: u64,
    },

    /// Queue position is beyond the normalized limit.
    QueuePositionTooLarge {
        value: u64,
        maximum: u64,
    },

    /// Queue position is inconsistent with queue depth.
    InvalidQueuePosition {
        depth: u64,
        position: u64,
    },

    /// Health score is greater than 100.
    HealthScoreOutOfRange {
        value: u16,
    },

    /// Confidence is greater than 100.
    ConfidenceOutOfRange {
        value: u16,
    },

    /// Source timestamp is later than the observation timestamp.
    FutureSourceTimestamp,

    /// Current observation precedes previous observation.
    NonMonotonicTimestamp {
        previous: StatusTimestamp,
        current: StatusTimestamp,
    },

    /// Requested transition violates lifecycle semantics.
    InvalidTransition {
        from: BackendStatus,
        to: BackendStatus,
    },

    /// Observation occurs after the supplied reference timestamp.
    FutureObservation,

    /// Status is older than the permitted age.
    StaleStatus {
        age_ns: u64,
        maximum_age_ns: u64,
    },

    /// Schema identifier does not match this module.
    InvalidSchemaId {
        expected: String,
        actual: String,
    },

    /// Serialized schema version is unsupported.
    UnsupportedSchemaVersion {
        version: u16,
    },
}

impl fmt::Display for BackendStatusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::IdentifierTooLong {
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

            Self::MessageTooLong { length, maximum } => {
                write!(
                    formatter,
                    "status message is {length} bytes long; maximum is {maximum}"
                )
            }

            Self::SensitiveInformation => {
                formatter.write_str(
                    "status data appears to contain sensitive credential information",
                )
            }

            Self::ConditionLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "status condition limit exceeded; maximum is {maximum}"
                )
            }

            Self::MetadataLimitExceeded { maximum } => {
                write!(
                    formatter,
                    "status metadata limit exceeded; maximum is {maximum}"
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str("invalid status metadata key")
            }

            Self::MetadataValueTooLong { length, maximum } => {
                write!(
                    formatter,
                    "status metadata value is {length} bytes long; maximum is {maximum}"
                )
            }

            Self::QueueDepthTooLarge { value, maximum } => {
                write!(
                    formatter,
                    "queue depth {value} exceeds maximum {maximum}"
                )
            }

            Self::QueuePositionTooLarge { value, maximum } => {
                write!(
                    formatter,
                    "queue position {value} exceeds maximum {maximum}"
                )
            }

            Self::InvalidQueuePosition { depth, position } => {
                write!(
                    formatter,
                    "queue position {position} is inconsistent with queue depth {depth}"
                )
            }

            Self::HealthScoreOutOfRange { value } => {
                write!(
                    formatter,
                    "health score {value} is outside the range 0..=100"
                )
            }

            Self::ConfidenceOutOfRange { value } => {
                write!(
                    formatter,
                    "confidence {value} is outside the range 0..=100"
                )
            }

            Self::FutureSourceTimestamp => {
                formatter.write_str(
                    "source timestamp is later than observation timestamp",
                )
            }

            Self::NonMonotonicTimestamp { previous, current } => {
                write!(
                    formatter,
                    "status timestamp moved backwards: previous={previous}, current={current}"
                )
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid backend status transition: {from} -> {to}"
                )
            }

            Self::FutureObservation => {
                formatter.write_str(
                    "status observation occurs after the supplied reference time",
                )
            }

            Self::StaleStatus {
                age_ns,
                maximum_age_ns,
            } => {
                write!(
                    formatter,
                    "backend status is stale: age={age_ns} ns; maximum={maximum_age_ns} ns"
                )
            }

            Self::InvalidSchemaId { expected, actual } => {
                write!(
                    formatter,
                    "invalid backend-status schema ID: expected `{expected}`, got `{actual}`"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported backend-status schema version {version}"
                )
            }
        }
    }
}

impl std::error::Error for BackendStatusError {}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendStatusError> {
    if value.is_empty() {
        return Err(BackendStatusError::EmptyIdentifier { field });
    }

    if value.len() > maximum {
        return Err(BackendStatusError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    if value != value.trim() {
        return Err(BackendStatusError::InvalidIdentifier { field });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendStatusError::InvalidIdentifier { field });
    }

    if contains_secret_like_content(value) {
        return Err(BackendStatusError::SensitiveInformation);
    }

    Ok(())
}

fn validate_metadata(
    key: &str,
    value: &str,
) -> Result<(), BackendStatusError> {
    if key.is_empty() {
        return Err(BackendStatusError::InvalidMetadataKey);
    }

    if key.len() > MAX_STATUS_METADATA_KEY_LENGTH {
        return Err(BackendStatusError::InvalidMetadataKey);
    }

    if key != key.trim() || key.chars().any(char::is_control) {
        return Err(BackendStatusError::InvalidMetadataKey);
    }

    if value.len() > MAX_STATUS_METADATA_VALUE_LENGTH {
        return Err(BackendStatusError::MetadataValueTooLong {
            length: value.len(),
            maximum: MAX_STATUS_METADATA_VALUE_LENGTH,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendStatusError::SensitiveInformation);
    }

    if contains_secret_like_content(key)
        || contains_secret_like_content(value)
    {
        return Err(BackendStatusError::SensitiveInformation);
    }

    Ok(())
}

/// Conservative defense-in-depth detection of common credential-bearing
/// field names.
///
/// This is deliberately not a secret scanner. Authentication belongs to the
/// credential/authentication subsystem.
fn contains_secret_like_content(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const SENSITIVE_MARKERS: &[&str] = &[
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
        "secret_key",
        "secretkey",
        "client_secret",
        "clientsecret",
        "session_cookie",
        "cookie",
        "bearer ",
    ];

    SENSITIVE_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const T0: StatusTimestamp = StatusTimestamp::from_unix_nanos(1_000);
    const T1: StatusTimestamp = StatusTimestamp::from_unix_nanos(2_000);
    const T2: StatusTimestamp = StatusTimestamp::from_unix_nanos(3_000);

    #[test]
    fn status_strings_are_stable() {
        assert_eq!(BackendStatus::Unknown.as_str(), "unknown");
        assert_eq!(
            BackendStatus::Initializing.as_str(),
            "initializing"
        );
        assert_eq!(BackendStatus::Available.as_str(), "available");
        assert_eq!(BackendStatus::Busy.as_str(), "busy");
        assert_eq!(
            BackendStatus::Maintenance.as_str(),
            "maintenance"
        );
        assert_eq!(BackendStatus::Degraded.as_str(), "degraded");
        assert_eq!(
            BackendStatus::Unavailable.as_str(),
            "unavailable"
        );
        assert_eq!(BackendStatus::Offline.as_str(), "offline");
        assert_eq!(BackendStatus::Error.as_str(), "error");
        assert_eq!(BackendStatus::Retired.as_str(), "retired");
    }

    #[test]
    fn available_and_degraded_are_submission_eligible() {
        assert!(BackendStatus::Available.is_submission_eligible());
        assert!(BackendStatus::Degraded.is_submission_eligible());

        assert!(!BackendStatus::Busy.is_submission_eligible());
        assert!(!BackendStatus::Maintenance.is_submission_eligible());
        assert!(!BackendStatus::Unavailable.is_submission_eligible());
        assert!(!BackendStatus::Offline.is_submission_eligible());
        assert!(!BackendStatus::Error.is_submission_eligible());
        assert!(!BackendStatus::Retired.is_submission_eligible());
    }

    #[test]
    fn busy_is_operational_but_not_direct_submission_eligible() {
        assert!(BackendStatus::Busy.is_operational());
        assert!(!BackendStatus::Busy.is_submission_eligible());
        assert!(BackendStatus::Busy.may_accept_queue_submission());
    }

    #[test]
    fn maintenance_is_operational_but_not_submission_eligible() {
        assert!(BackendStatus::Maintenance.is_operational());
        assert!(!BackendStatus::Maintenance.is_submission_eligible());
    }

    #[test]
    fn retired_is_terminal() {
        assert!(BackendStatus::Retired.is_retired());
        assert!(!BackendStatus::Retired.can_transition_to(
            BackendStatus::Available
        ));
        assert!(!BackendStatus::Retired.can_transition_to(
            BackendStatus::Unknown
        ));
    }

    #[test]
    fn ordinary_lifecycle_transition_is_allowed() {
        assert!(BackendStatus::Unknown.can_transition_to(
            BackendStatus::Initializing
        ));

        assert!(BackendStatus::Initializing.can_transition_to(
            BackendStatus::Available
        ));

        assert!(BackendStatus::Available.can_transition_to(
            BackendStatus::Busy
        ));

        assert!(BackendStatus::Busy.can_transition_to(
            BackendStatus::Available
        ));
    }

    #[test]
    fn invalid_lifecycle_transition_is_rejected() {
        assert!(!BackendStatus::Available.can_transition_to(
            BackendStatus::Initializing
        ));

        assert!(!BackendStatus::Available.can_transition_to(
            BackendStatus::Unknown
        ));
    }

    #[test]
    fn timestamp_round_trip_is_exact() {
        let timestamp = StatusTimestamp::from_unix_nanos(123_456_789);
        assert_eq!(timestamp.as_unix_nanos(), 123_456_789);
        assert_eq!(timestamp.as_unix_seconds(), 0);
    }

    #[test]
    fn queue_information_validates() {
        let queue = QueueStatus {
            depth: Some(10),
            position: Some(5),
            estimated_wait_ms: Some(1_000),
            authoritative: true,
        };

        assert!(queue.validate().is_ok());
        assert!(queue.is_known());
    }

    #[test]
    fn invalid_queue_position_is_rejected() {
        let queue = QueueStatus {
            depth: Some(5),
            position: Some(6),
            estimated_wait_ms: None,
            authoritative: true,
        };

        assert!(matches!(
            queue.validate(),
            Err(BackendStatusError::InvalidQueuePosition {
                depth: 5,
                position: 6
            })
        ));
    }

    #[test]
    fn empty_backend_id_is_rejected() {
        let result = BackendStatusSnapshot::new(
            "",
            BackendStatus::Unknown,
            T0,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::EmptyIdentifier {
                field: "backend_id"
            })
        ));
    }

    #[test]
    fn whitespace_backend_id_is_rejected() {
        let result = BackendStatusSnapshot::new(
            " backend ",
            BackendStatus::Unknown,
            T0,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::InvalidIdentifier {
                field: "backend_id"
            })
        ));
    }

    #[test]
    fn control_character_in_identifier_is_rejected() {
        let result = BackendStatusSnapshot::new(
            "backend\n1",
            BackendStatus::Unknown,
            T0,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::InvalidIdentifier {
                field: "backend_id"
            })
        ));
    }

    #[test]
    fn credential_like_backend_id_is_rejected() {
        let result = BackendStatusSnapshot::new(
            "api_key_backend",
            BackendStatus::Available,
            T0,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::SensitiveInformation)
        ));
    }

    #[test]
    fn snapshot_defaults_are_conservative() {
        let snapshot = BackendStatusSnapshot::new(
            "local:test",
            BackendStatus::Unknown,
            T0,
        )
        .expect("valid backend ID");

        assert_eq!(snapshot.status, BackendStatus::Unknown);
        assert!(!snapshot.authoritative);
        assert_eq!(snapshot.confidence, 100);
        assert_eq!(snapshot.queue, QueueStatus::unknown());
        assert!(snapshot.conditions.is_empty());
        assert!(snapshot.metadata.is_empty());
    }

    #[test]
    fn snapshot_can_be_fully_validated() {
        let condition = StatusCondition::active(
            StatusConditionKind::QueueActive,
            StatusSeverity::Info,
        )
        .with_condition_id("queue-active")
        .expect("valid condition ID")
        .with_message("backend queue contains pending work")
        .expect("safe message");

        let queue = QueueStatus {
            depth: Some(10),
            position: Some(3),
            estimated_wait_ms: Some(500),
            authoritative: true,
        };

        let snapshot = BackendStatusSnapshot::new(
            "local:test",
            BackendStatus::Busy,
            T0,
        )
        .expect("valid backend")
        .with_provider_id("local")
        .expect("valid provider")
        .with_device_id("simulator-0")
        .expect("valid device")
        .with_authoritative(true)
        .with_source("local-adapter")
        .expect("valid source")
        .with_queue(queue)
        .expect("valid queue")
        .with_health_score(100)
        .expect("valid health")
        .with_confidence(100)
        .expect("valid confidence")
        .with_condition(condition)
        .expect("valid condition")
        .with_metadata("environment", "test")
        .expect("valid metadata");

        assert!(snapshot.validate().is_ok());
        assert!(!snapshot.is_submission_eligible());
        assert_eq!(
            snapshot.effective_severity(),
            StatusSeverity::Normal
        );
    }

    #[test]
    fn degraded_condition_increases_effective_severity() {
        let condition = StatusCondition::active(
            StatusConditionKind::ReadoutDegraded,
            StatusSeverity::Warning,
        );

        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_condition(condition)
        .expect("valid condition");

        assert_eq!(
            snapshot.effective_severity(),
            StatusSeverity::Warning
        );
        assert!(snapshot.requires_attention());
    }

    #[test]
    fn stale_status_is_detected() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend");

        assert!(
            snapshot
                .is_stale_at(T2, 1_000)
                .expect("valid age")
        );

        assert!(
            snapshot
                .is_stale_at(T1, 1_000)
                .expect("valid age")
        );
    }

    #[test]
    fn fresh_status_is_accepted() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T1,
        )
        .expect("valid backend");

        assert!(
            snapshot
                .require_fresh_at(T2, 1_000)
                .is_ok()
        );
    }

    #[test]
    fn future_reference_is_rejected() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T2,
        )
        .expect("valid backend");

        assert!(matches!(
            snapshot.age_at(T1),
            Err(BackendStatusError::FutureObservation)
        ));
    }

    #[test]
    fn transition_preserves_identity() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_provider_id("provider")
        .expect("valid provider")
        .with_device_id("device")
        .expect("valid device");

        let next = snapshot
            .transition(BackendStatus::Busy, T1)
            .expect("valid transition");

        assert_eq!(next.backend_id, "backend");
        assert_eq!(next.provider_id.as_deref(), Some("provider"));
        assert_eq!(next.device_id.as_deref(), Some("device"));
        assert_eq!(next.status, BackendStatus::Busy);
        assert_eq!(next.observed_at, T1);
    }

    #[test]
    fn transition_rejects_backwards_time() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T1,
        )
        .expect("valid backend");

        let result = snapshot.transition(
            BackendStatus::Busy,
            T0,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::NonMonotonicTimestamp {
                previous: T1,
                current: T0
            })
        ));
    }

    #[test]
    fn transition_rejects_invalid_lifecycle_change() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend");

        let result = snapshot.transition(
            BackendStatus::Initializing,
            T1,
        );

        assert!(matches!(
            result,
            Err(BackendStatusError::InvalidTransition {
                from: BackendStatus::Available,
                to: BackendStatus::Initializing
            })
        ));
    }

    #[test]
    fn transition_clears_old_queue_observation() {
        let queue = QueueStatus {
            depth: Some(10),
            position: Some(2),
            estimated_wait_ms: Some(500),
            authoritative: true,
        };

        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_queue(queue)
        .expect("valid queue");

        let next = snapshot
            .transition(BackendStatus::Busy, T1)
            .expect("valid transition");

        assert_eq!(next.queue, QueueStatus::unknown());
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_metadata("z", "3")
        .expect("valid metadata")
        .with_metadata("a", "1")
        .expect("valid metadata")
        .with_metadata("m", "2")
        .expect("valid metadata");

        let keys: Vec<&String> = snapshot.metadata.keys().collect();

        assert_eq!(
            keys,
            vec![
                &"a".to_owned(),
                &"m".to_owned(),
                &"z".to_owned()
            ]
        );
    }

    #[test]
    fn status_condition_message_rejects_secret_marker() {
        let result = StatusCondition::active(
            StatusConditionKind::Other,
            StatusSeverity::Warning,
        )
        .with_message("api_key=secret");

        assert!(matches!(
            result,
            Err(BackendStatusError::SensitiveInformation)
        ));
    }

    #[test]
    fn metadata_rejects_secret_marker() {
        let result = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_metadata("api_key", "secret");

        assert!(matches!(
            result,
            Err(BackendStatusError::SensitiveInformation)
        ));
    }

    #[test]
    fn source_timestamp_cannot_be_after_observation() {
        let result = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_source_timestamp(T1);

        assert!(matches!(
            result.validate(),
            Err(BackendStatusError::FutureSourceTimestamp)
        ));
    }

    #[test]
    fn health_score_must_be_within_range() {
        let result = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_health_score(101);

        assert!(matches!(
            result,
            Err(BackendStatusError::HealthScoreOutOfRange {
                value: 101
            })
        ));
    }

    #[test]
    fn confidence_must_be_within_range() {
        let result = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_confidence(101);

        assert!(matches!(
            result,
            Err(BackendStatusError::ConfidenceOutOfRange {
                value: 101
            })
        ));
    }

    #[test]
    fn schema_is_stable() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend");

        assert_eq!(
            snapshot.schema_id,
            BACKEND_STATUS_SCHEMA_ID
        );

        assert_eq!(
            snapshot.schema_version,
            BACKEND_STATUS_SCHEMA_VERSION
        );
    }

    #[test]
    fn canonical_summary_is_deterministic() {
        let first = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend");

        let second = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend");

        assert_eq!(
            first.canonical_summary(),
            second.canonical_summary()
        );
    }

    #[test]
    fn serde_round_trip_is_supported() {
        let snapshot = BackendStatusSnapshot::new(
            "backend",
            BackendStatus::Available,
            T0,
        )
        .expect("valid backend")
        .with_metadata("region", "local")
        .expect("valid metadata");

        let serialized =
            serde_json::to_string(&snapshot).expect("serialize");

        let decoded: BackendStatusSnapshot =
            serde_json::from_str(&serialized).expect("deserialize");

        assert_eq!(snapshot, decoded);
    }
}