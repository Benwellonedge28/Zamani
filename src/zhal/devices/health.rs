
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Device Health
//!
//! Path: src/zhal/devices/health.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines the universal, provider-independent hardware health
//! observation contract for Zamani.
//!
//! Supports health observations originating from:
//!
//! - Classical processors and memory systems.
//! - Quantum processors and quantum control systems.
//! - FPGA and ASIC hardware.
//! - GPUs, TPUs, NPUs and other accelerators.
//! - Embedded processors and microcontrollers.
//! - Sensors and actuators.
//! - Remote and distributed hardware.
//! - Hybrid computing systems.
//! - Neuromorphic hardware.
//! - Photonic and optical computing.
//! - Analog and in-memory computing.
//! - Future computational technologies.
//!
//! Hardware categories and health-check dimensions are open-world.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Hardware health status.
//! - Health severity.
//! - Extensible health-check identifiers.
//! - Provider/device health subjects.
//! - Timestamped health observations.
//! - Structured health diagnostics.
//! - Health-report construction.
//! - Deterministic health aggregation.
//! - Health evidence freshness assessment.
//! - Health serialization validation.
//!
//! # Existing authorities
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Hardware configuration:
//!     crate::zhal::config
//!
//! Hardware provenance:
//!     crate::zhal::model::snapshot
//!
//! Quantum-specific health:
//!     crate::quantum::hardware::health
//!
//! Distributed participant health:
//!     crate::distributed::membership::health
//!
//! Hardware device inventory:
//!     crate::zhal::devices::inventory
//!
//! # Important distinctions
//!
//! Health is observed operational state.
//!
//! Health is not:
//!
//! - Capability.
//! - Authorization.
//! - Authentication.
//! - Resource reservation.
//! - Execution permission.
//! - Semantic validity.
//!
//! A device may support an operation while temporarily being
//! unavailable to perform it.
//!
//! Health evidence never independently authorizes execution.
//!
//! # Scalability
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Providers.
//! - Devices.
//! - Health-check dimensions.
//! - Observations.
//! - Diagnostics.
//! - Identifier lengths.
//! - Metadata references.
//!
//! Actual memory consumption is constrained by available resources,
//! address-space representation and configured operational budgets.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! # Monitoring
//!
//! This module does not perform health probes or create monitoring
//! threads.
//!
//! Provider implementations and monitoring services perform probes
//! and construct validated observations.
//!
//! Periodic monitoring policy belongs to:
//!
//!     crate::zhal::config::HealthPolicy
//!
//! Observation freshness policy belongs to:
//!
//!     crate::zhal::config::ObservationPolicy
//!
//! # Safety
//!
//! No unsafe Rust is permitted.

use core::fmt;

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize};

use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::stdlib::provider::ProviderId;

use crate::zhal::config::{
    ObservationPolicy,
    StaleObservationPolicy,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::model::snapshot::SnapshotProvenance;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable identity of this component.
pub const COMPONENT: &str = "zhal.devices.health";

/// Health observation schema identifier.
pub const HEALTH_SCHEMA_ID: &str =
    "zamani.zhal.devices.health";

/// Health observation schema version.
///
/// Increment when serialized semantics change incompatibly.
pub const HEALTH_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn health_error(
    kind: ZhalErrorKind,
    field: &'static str,
    reason: &'static str,
) -> ZhalError {
    ZhalError::new(
        kind,
        format!("{field}: {reason}"),
    )
    .in_component(COMPONENT)
}

fn invalid_identifier(
    field: &'static str,
) -> ZhalError {
    health_error(
        ZhalErrorKind::InvalidData,
        field,
        "identifier must be nonempty and contain no whitespace or control characters",
    )
}

fn invalid_text(
    field: &'static str,
) -> ZhalError {
    health_error(
        ZhalErrorKind::InvalidData,
        field,
        "text must be nonempty, valid and free of sensitive information",
    )
}

// ============================================================================
// INPUT VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
        || value.chars().any(char::is_control)
    {
        return Err(invalid_identifier(field));
    }

    Ok(())
}

fn contains_secret_material(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const SECRET_MARKERS: &[&str] = &[
        "api_key=",
        "api-key=",
        "access_token=",
        "access-token=",
        "authorization:",
        "bearer ",
        "password=",
        "private_key=",
        "private-key=",
        "session_cookie=",
        "set-cookie:",
        "secret=",
    ];

    SECRET_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

fn validate_safe_text(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
        || contains_secret_material(value)
    {
        return Err(invalid_text(field));
    }

    Ok(())
}

fn validate_safe_key(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    validate_identifier(value, field)?;

    let normalized = value.to_ascii_lowercase();

    const SENSITIVE_KEYS: &[&str] = &[
        "api_key",
        "access_token",
        "authorization",
        "password",
        "private_key",
        "secret",
        "session_cookie",
        "cookie",
    ];

    if SENSITIVE_KEYS
        .iter()
        .any(|key| normalized.contains(key))
    {
        return Err(invalid_text(field));
    }

    Ok(())
}

// ============================================================================
// STANDARD HEALTH-CHECK IDENTIFIERS
// ============================================================================

/// Common health-check identifiers.
///
/// These are conveniences, not a closed enumeration.
///
/// New hardware providers can introduce additional namespaced
/// health dimensions without modifying this module.
pub mod checks {
    /// Provider reachability.
    pub const REACHABILITY: &str = "hardware.reachability";

    /// Provider authentication observation.
    pub const AUTHENTICATION: &str = "hardware.authentication";

    /// Authorization service observation.
    pub const AUTHORIZATION: &str = "hardware.authorization";

    /// API and protocol compatibility.
    pub const API_COMPATIBILITY: &str =
        "hardware.api_compatibility";

    /// Physical device operational state.
    pub const DEVICE_OPERATIONAL: &str =
        "hardware.device_operational";

    /// Provider service health.
    pub const PROVIDER_SERVICE: &str =
        "hardware.provider_service";

    /// Execution service health.
    pub const EXECUTION: &str = "hardware.execution";

    /// Queue service health.
    pub const QUEUE: &str = "hardware.queue";

    /// Result retrieval service health.
    pub const RESULT_RETRIEVAL: &str =
        "hardware.result_retrieval";

    /// Resource availability observation.
    pub const RESOURCE_AVAILABILITY: &str =
        "hardware.resource_availability";

    /// Communication interface health.
    pub const COMMUNICATION: &str =
        "hardware.communication";

    /// Memory subsystem health.
    pub const MEMORY: &str = "hardware.memory";

    /// Processing subsystem health.
    pub const PROCESSING: &str = "hardware.processing";

    /// Power subsystem health.
    pub const POWER: &str = "hardware.power";

    /// Thermal subsystem health.
    pub const THERMAL: &str = "hardware.thermal";

    /// Firmware health.
    pub const FIRMWARE: &str = "hardware.firmware";

    /// Driver health.
    pub const DRIVER: &str = "hardware.driver";

    /// Interconnect health.
    pub const INTERCONNECT: &str = "hardware.interconnect";

    /// Calibration health observation.
    ///
    /// Domain-specific calibration semantics remain owned
    /// by the corresponding hardware domain.
    pub const CALIBRATION: &str = "hardware.calibration";

    /// Topology observation.
    pub const TOPOLOGY: &str = "hardware.topology";

    /// Timing subsystem health.
    pub const TIMING: &str = "hardware.timing";

    /// Cancellation service health.
    pub const CANCELLATION: &str =
        "hardware.cancellation";

    /// Hardware safety-interlock observation.
    pub const SAFETY_INTERLOCK: &str =
        "hardware.safety_interlock";
}

// ============================================================================
// HEALTH-CHECK IDENTITY
// ============================================================================

/// Extensible identity of a hardware health-check dimension.
///
/// Identifiers are namespaced strings rather than a closed enum.
///
/// This allows future providers and technologies to introduce
/// additional health dimensions without modifying ZHAL.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct HealthCheckId(String);

impl HealthCheckId {
    /// Constructs a validated health-check identity.
    ///
    /// No arbitrary identifier-length limit is imposed.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "health_check",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact health-check identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the identifier's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the identifier is empty.
    ///
    /// Valid identifiers cannot be empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl fmt::Display for HealthCheckId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for HealthCheckId {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value)
            .map_err(D::Error::custom)
    }
}

// ============================================================================
// HEALTH SUBJECT
// ============================================================================

/// Identifies the hardware entity whose health is observed.
///
/// Provider and device identities reuse their existing
/// canonical repository authorities.
///
/// A device identity remains provider-scoped.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(tag = "scope", content = "identity", rename_all = "snake_case")]
pub enum HealthSubject {
    /// Provider-wide health observation.
    Provider(ProviderId),

    /// Individual hardware-device health observation.
    Device(HardwareDeviceIdentity),
}

impl HealthSubject {
    /// Returns the provider identity associated with this subject.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        match self {
            Self::Provider(provider) => provider,
            Self::Device(device) => device.provider_id(),
        }
    }

    /// Returns the device identity when this is a device observation.
    #[must_use]
    pub fn device_identity(
        &self,
    ) -> Option<&HardwareDeviceIdentity> {
        match self {
            Self::Provider(_) => None,
            Self::Device(device) => Some(device),
        }
    }

    /// Returns whether this subject represents a provider.
    #[must_use]
    pub const fn is_provider(&self) -> bool {
        matches!(self, Self::Provider(_))
    }

    /// Returns whether this subject represents a device.
    #[must_use]
    pub const fn is_device(&self) -> bool {
        matches!(self, Self::Device(_))
    }
}

// ============================================================================
// HEALTH STATUS
// ============================================================================

/// Operational state reported by a health observation.
///
/// These states describe observed conditions, not permissions.
///
/// Unknown information must never be interpreted as healthy.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    /// No authoritative health information is available.
    Unknown,

    /// The observed component is healthy.
    Healthy,

    /// The component is operational but degraded.
    Degraded,

    /// The component is temporarily unavailable.
    Unavailable,

    /// The component has failed.
    Failed,

    /// The component has been retired.
    Retired,
}

impl HealthStatus {
    /// Returns the stable machine-readable representation.
    #[must_use]
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

    /// Returns whether the state is fully healthy.
    #[must_use]
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Returns whether the state is potentially operational.
    ///
    /// This does not authorize execution.
    #[must_use]
    pub const fn is_operational(self) -> bool {
        matches!(
            self,
            Self::Healthy | Self::Degraded
        )
    }

    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns a deterministic severity rank.
    #[must_use]
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

    /// Returns the more severe of two health states.
    #[must_use]
    pub const fn max_severity(
        self,
        other: Self,
    ) -> Self {
        if self.severity_rank() >= other.severity_rank() {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for HealthStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// HEALTH DIAGNOSTIC SEVERITY
// ============================================================================

/// Severity of a health diagnostic.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthSeverity {
    /// Informational observation.
    Info,

    /// Potential operational issue.
    Warning,

    /// Significant operational issue.
    Error,

    /// Critical operational issue.
    Critical,
}

impl HealthSeverity {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Returns the severity rank.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Critical => 3,
        }
    }
}

impl fmt::Display for HealthSeverity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// HEALTH TIMESTAMP
// ============================================================================

/// Timestamp representing nanoseconds since the Unix epoch.
///
/// The representation uses u128 to avoid unnecessarily restricting
/// timestamps to a narrower integer representation.
///
/// Clock acquisition is explicit.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct HealthTimestamp {
    unix_nanos: u128,
}

impl HealthTimestamp {
    /// Constructs a timestamp from Unix nanoseconds.
    #[must_use]
    pub const fn from_unix_nanos(
        unix_nanos: u128,
    ) -> Self {
        Self { unix_nanos }
    }

    /// Returns Unix nanoseconds.
    #[must_use]
    pub const fn as_unix_nanos(self) -> u128 {
        self.unix_nanos
    }

    /// Obtains the current system timestamp.
    ///
    /// This is an explicit convenience method.
    ///
    /// # Errors
    ///
    /// Returns an error if the system clock is before the Unix epoch.
    pub fn now() -> ZhalResult<Self> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| {
                health_error(
                    ZhalErrorKind::Internal,
                    "timestamp",
                    "system clock precedes Unix epoch",
                )
            })?;

        Ok(Self::from_unix_nanos(
            duration.as_nanos(),
        ))
    }

    /// Returns the nonnegative age relative to another timestamp.
    ///
    /// A future timestamp has age zero. Callers must separately
    /// inspect whether the timestamp is future-dated.
    #[must_use]
    pub const fn age_since(
        self,
        now: Self,
    ) -> u128 {
        now.unix_nanos.saturating_sub(
            self.unix_nanos,
        )
    }

    /// Returns whether this timestamp is later than another.
    #[must_use]
    pub const fn is_after(
        self,
        other: Self,
    ) -> bool {
        self.unix_nanos > other.unix_nanos
    }
}

impl fmt::Display for HealthTimestamp {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}ns",
            self.unix_nanos,
        )
    }
}

// ============================================================================
// HEALTH DIAGNOSTICS
// ============================================================================

/// Structured explanation of a health condition.
///
/// Diagnostic messages must not contain credentials or secret material.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct HealthDiagnostic {
    code: String,
    severity: HealthSeverity,
    message: String,
}

impl HealthDiagnostic {
    /// Constructs a validated health diagnostic.
    pub fn new(
        code: impl Into<String>,
        severity: HealthSeverity,
        message: impl Into<String>,
    ) -> ZhalResult<Self> {
        let code = code.into();
        let message = message.into();

        validate_identifier(
            &code,
            "diagnostic.code",
        )?;

        validate_safe_text(
            &message,
            "diagnostic.message",
        )?;

        Ok(Self {
            code,
            severity,
            message,
        })
    }

    /// Returns the diagnostic code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> HealthSeverity {
        self.severity
    }

    /// Returns the safe diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Validates the diagnostic.
    pub fn validate(&self) -> ZhalResult<()> {
        validate_identifier(
            &self.code,
            "diagnostic.code",
        )?;

        validate_safe_text(
            &self.message,
            "diagnostic.message",
        )
    }
}

impl<'de> Deserialize<'de> for HealthDiagnostic {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            code: String,
            severity: HealthSeverity,
            message: String,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.code,
            wire.severity,
            wire.message,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// OBSERVATION FRESHNESS
// ============================================================================

/// Freshness classification of a health observation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum HealthFreshness {
    /// No maximum observation age was configured.
    NotConstrained,

    /// Observation is within the configured age.
    Fresh,

    /// Observation exceeds the configured age.
    Stale,

    /// Observation timestamp is later than the assessment time.
    ///
    /// This may indicate clock skew or invalid provider time.
    FutureDated,
}

impl HealthFreshness {
    /// Returns whether the observation satisfies its freshness policy.
    #[must_use]
    pub const fn satisfies_policy(self) -> bool {
        matches!(
            self,
            Self::NotConstrained | Self::Fresh
        )
    }

    /// Returns whether the observation is explicitly stale.
    #[must_use]
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }

    /// Returns whether the timestamp is future-dated.
    #[must_use]
    pub const fn is_future_dated(self) -> bool {
        matches!(self, Self::FutureDated)
    }
}

// ============================================================================
// HEALTH OBSERVATION
// ============================================================================

/// One timestamped health observation.
///
/// An observation represents one health dimension for one provider
/// or hardware device.
///
/// Repeated observations belong in separate reports or an external
/// history/telemetry subsystem.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct HealthObservation {
    subject: HealthSubject,
    check: HealthCheckId,
    status: HealthStatus,
    observed_at: HealthTimestamp,
    provenance: SnapshotProvenance,
    diagnostics: Vec<HealthDiagnostic>,
}

impl HealthObservation {
    /// Constructs a health observation.
    ///
    /// The source identifies the provider or monitoring component
    /// responsible for producing the observation.
    pub fn new(
        subject: HealthSubject,
        check: HealthCheckId,
        status: HealthStatus,
        observed_at: HealthTimestamp,
        source: impl Into<String>,
    ) -> ZhalResult<Self> {
        let provenance = SnapshotProvenance::new(source)
            .map_err(|_| {
                invalid_text("provenance.source")
            })?;

        let observation = Self {
            subject,
            check,
            status,
            observed_at,
            provenance,
            diagnostics: Vec::new(),
        };

        observation.validate()?;

        Ok(observation)
    }

    /// Returns the observed subject.
    #[must_use]
    pub fn subject(&self) -> &HealthSubject {
        &self.subject
    }

    /// Returns the health-check identity.
    #[must_use]
    pub fn check(&self) -> &HealthCheckId {
        &self.check
    }

    /// Returns the observed health state.
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> HealthTimestamp {
        self.observed_at
    }

    /// Returns observation provenance.
    #[must_use]
    pub fn provenance(&self) -> &SnapshotProvenance {
        &self.provenance
    }

    /// Returns immutable diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[HealthDiagnostic] {
        &self.diagnostics
    }

    /// Associates an external evidence reference.
    pub fn with_evidence_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ZhalResult<Self> {
        self.provenance = self.provenance
            .with_evidence_reference(reference)
            .map_err(|_| {
                invalid_identifier(
                    "provenance.evidence_reference",
                )
            })?;

        Ok(self)
    }

    /// Associates an external provider reference.
    pub fn with_provider_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ZhalResult<Self> {
        self.provenance = self.provenance
            .with_provider_reference(reference)
            .map_err(|_| {
                invalid_identifier(
                    "provenance.provider_reference",
                )
            })?;

        Ok(self)
    }

    /// Associates a collection-method description.
    pub fn with_collection_method(
        mut self,
        method: impl Into<String>,
    ) -> ZhalResult<Self> {
        self.provenance = self.provenance
            .with_collection_method(method)
            .map_err(|_| {
                invalid_text(
                    "provenance.collection_method",
                )
            })?;

        Ok(self)
    }

    /// Adds a diagnostic to this observation.
    pub fn with_diagnostic(
        mut self,
        diagnostic: HealthDiagnostic,
    ) -> ZhalResult<Self> {
        diagnostic.validate()?;

        self.diagnostics.push(diagnostic);

        Ok(self)
    }

    /// Validates the complete observation.
    pub fn validate(&self) -> ZhalResult<()> {
        validate_identifier(
            self.check.as_str(),
            "health_check",
        )?;

        validate_safe_text(
            self.provenance.source(),
            "provenance.source",
        )?;

        if let Some(reference) =
            self.provenance.evidence_reference()
        {
            validate_identifier(
                reference,
                "provenance.evidence_reference",
            )?;
        }

        if let Some(reference) =
            self.provenance.provider_reference()
        {
            validate_identifier(
                reference,
                "provenance.provider_reference",
            )?;
        }

        if let Some(method) =
            self.provenance.collection_method()
        {
            validate_safe_text(
                method,
                "provenance.collection_method",
            )?;
        }

        for diagnostic in &self.diagnostics {
            diagnostic.validate()?;
        }

        Ok(())
    }

    /// Evaluates freshness using the existing observation policy.
    ///
    /// No clock is acquired implicitly.
    #[must_use]
    pub fn freshness_at(
        &self,
        now: HealthTimestamp,
        policy: &ObservationPolicy,
    ) -> HealthFreshness {
        if self.observed_at.is_after(now) {
            return HealthFreshness::FutureDated;
        }

        let Some(maximum_age) = policy.maximum_age() else {
            return HealthFreshness::NotConstrained;
        };

        let maximum_age_nanos = maximum_age.as_nanos();

        if self.observed_at.age_since(now) > maximum_age_nanos {
            HealthFreshness::Stale
        } else {
            HealthFreshness::Fresh
        }
    }
}

impl<'de> Deserialize<'de> for HealthObservation {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            subject: HealthSubject,
            check: HealthCheckId,
            status: HealthStatus,
            observed_at: HealthTimestamp,
            provenance: SnapshotProvenance,
            diagnostics: Vec<HealthDiagnostic>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let observation = Self {
            subject: wire.subject,
            check: wire.check,
            status: wire.status,
            observed_at: wire.observed_at,
            provenance: wire.provenance,
            diagnostics: wire.diagnostics,
        };

        observation
            .validate()
            .map_err(D::Error::custom)?;

        Ok(observation)
    }
}

// ============================================================================
// PER-OBSERVATION FRESHNESS RESULT
// ============================================================================

/// Freshness assessment for one health-check dimension.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ObservationFreshness {
    check: HealthCheckId,
    freshness: HealthFreshness,
}

impl ObservationFreshness {
    /// Returns the health-check identity.
    #[must_use]
    pub fn check(&self) -> &HealthCheckId {
        &self.check
    }

    /// Returns the freshness classification.
    #[must_use]
    pub const fn freshness(&self) -> HealthFreshness {
        self.freshness
    }
}

// ============================================================================
// HEALTH ASSESSMENT
// ============================================================================

/// Result of assessing a health report at a particular time.
///
/// This is an informational assessment, not an execution grant.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct HealthAssessment {
    status: HealthStatus,
    observations: Vec<ObservationFreshness>,
    stale_policy: StaleObservationPolicy,
}

impl HealthAssessment {
    /// Returns the observed aggregate health state.
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }

    /// Returns individual freshness assessments.
    #[must_use]
    pub fn observations(&self) -> &[ObservationFreshness] {
        &self.observations
    }

    /// Returns whether every observation satisfies its freshness policy.
    #[must_use]
    pub fn freshness_satisfied(&self) -> bool {
        self.observations
            .iter()
            .all(|entry| entry.freshness.satisfies_policy())
    }

    /// Returns whether any observation is stale.
    #[must_use]
    pub fn contains_stale_observations(&self) -> bool {
        self.observations.iter().any(
            |entry| entry.freshness.is_stale(),
        )
    }

    /// Returns whether any observation is future-dated.
    #[must_use]
    pub fn contains_future_dated_observations(&self) -> bool {
        self.observations.iter().any(
            |entry| entry.freshness.is_future_dated(),
        )
    }

    /// Returns whether freshness-dependent consumers should reject
    /// this assessment under the configured stale-observation policy.
    ///
    /// This does not make an authorization or execution decision.
    #[must_use]
    pub fn must_reject_freshness_dependent_decision(&self) -> bool {
        matches!(
            self.stale_policy,
            StaleObservationPolicy::RejectForFreshnessDependentDecisions
        ) && !self.freshness_satisfied()
    }
}

// ============================================================================
// HEALTH REPORT
// ============================================================================

/// Immutable aggregate of health observations for one subject.
///
/// Observations are maintained in deterministic health-check order.
///
/// A report contains at most one current observation per check.
/// Historical samples belong to the existing telemetry/history
/// authorities.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct HealthReport {
    subject: HealthSubject,
    observations: Vec<HealthObservation>,
    status: HealthStatus,
}

impl HealthReport {
    /// Creates an empty health report.
    ///
    /// An empty report has Unknown health.
    #[must_use]
    pub fn empty(
        subject: HealthSubject,
    ) -> Self {
        Self {
            subject,
            observations: Vec::new(),
            status: HealthStatus::Unknown,
        }
    }

    /// Constructs a report from validated observations.
    ///
    /// Observations must refer to the same provider or device.
    ///
    /// Duplicate health-check identities are rejected.
    ///
    /// The resulting order is deterministic.
    pub fn from_observations(
        subject: HealthSubject,
        mut observations: Vec<HealthObservation>,
    ) -> ZhalResult<Self> {
        for observation in &observations {
            observation.validate()?;

            if observation.subject != subject {
                return Err(health_error(
                    ZhalErrorKind::InvalidData,
                    "health_report.subject",
                    "observation subject differs from report subject",
                ));
            }
        }

        observations.sort_by(
            |left, right| left.check.cmp(&right.check),
        );

        let mut identities = BTreeSet::new();

        for observation in &observations {
            if !identities.insert(observation.check.clone()) {
                return Err(health_error(
                    ZhalErrorKind::InvalidData,
                    "health_report.observations",
                    "duplicate health-check identity",
                ));
            }
        }

        let status = aggregate_status(&observations);

        Ok(Self {
            subject,
            observations,
            status,
        })
    }

    /// Returns the observed subject.
    #[must_use]
    pub fn subject(&self) -> &HealthSubject {
        &self.subject
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.subject.provider_id()
    }

    /// Returns the device identity when applicable.
    #[must_use]
    pub fn device_identity(
        &self,
    ) -> Option<&HardwareDeviceIdentity> {
        self.subject.device_identity()
    }

    /// Returns the aggregate health status.
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }

    /// Returns all observations in deterministic order.
    #[must_use]
    pub fn observations(&self) -> &[HealthObservation] {
        &self.observations
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the report has no observations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Returns whether all recorded observations are healthy.
    ///
    /// An empty report is not healthy.
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        !self.is_empty() && self.status.is_healthy()
    }

    /// Returns whether the aggregate is potentially operational.
    ///
    /// An empty or unknown report is not operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        !self.is_empty() && self.status.is_operational()
    }

    /// Looks up an observation by its health-check identity.
    #[must_use]
    pub fn observation(
        &self,
        check: &HealthCheckId,
    ) -> Option<&HealthObservation> {
        self.observations
            .binary_search_by(
                |observation| observation.check.cmp(check),
            )
            .ok()
            .map(|index| &self.observations[index])
    }

    /// Returns whether the report contains a health-check dimension.
    #[must_use]
    pub fn contains_check(
        &self,
        check: &HealthCheckId,
    ) -> bool {
        self.observation(check).is_some()
    }

    /// Returns all health checks that are not fully healthy.
    ///
    /// The returned list is deterministically ordered.
    #[must_use]
    pub fn unhealthy_checks(&self) -> Vec<HealthCheckId> {
        self.observations
            .iter()
            .filter(|observation| {
                !observation.status.is_healthy()
            })
            .map(|observation| observation.check.clone())
            .collect()
    }

    /// Assesses observation freshness using an explicit time and
    /// the existing canonical ObservationPolicy.
    ///
    /// Health status and freshness remain separate.
    #[must_use]
    pub fn assess(
        &self,
        now: HealthTimestamp,
        policy: &ObservationPolicy,
    ) -> HealthAssessment {
        let observations = self.observations
            .iter()
            .map(|observation| ObservationFreshness {
                check: observation.check.clone(),
                freshness: observation.freshness_at(
                    now,
                    policy,
                ),
            })
            .collect();

        HealthAssessment {
            status: self.status,
            observations,
            stale_policy: policy.stale_policy(),
        }
    }

    /// Validates report consistency and aggregate status.
    pub fn validate(&self) -> ZhalResult<()> {
        let reconstructed = Self::from_observations(
            self.subject.clone(),
            self.observations.clone(),
        )?;

        if reconstructed.status != self.status {
            return Err(health_error(
                ZhalErrorKind::InvalidData,
                "health_report.status",
                "aggregate status does not match its observations",
            ));
        }

        if reconstructed.observations != self.observations {
            return Err(health_error(
                ZhalErrorKind::InvalidData,
                "health_report.observations",
                "observations are not in canonical order",
            ));
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for HealthReport {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            subject: HealthSubject,
            observations: Vec<HealthObservation>,
            status: HealthStatus,
        }

        let wire = Wire::deserialize(deserializer)?;

        let report = Self::from_observations(
            wire.subject,
            wire.observations,
        )
        .map_err(D::Error::custom)?;

        if report.status != wire.status {
            return Err(D::Error::custom(
                "serialized health aggregate does not match observations",
            ));
        }

        Ok(report)
    }
}

// ============================================================================
// DETERMINISTIC AGGREGATION
// ============================================================================

fn aggregate_status(
    observations: &[HealthObservation],
) -> HealthStatus {
    if observations.is_empty() {
        return HealthStatus::Unknown;
    }

    observations
        .iter()
        .map(|observation| observation.status)
        .fold(
            HealthStatus::Healthy,
            HealthStatus::max_severity,
        )
}

// ============================================================================
// COMPONENT API
// ============================================================================

/// Returns the stable health component identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the health schema identifier.
#[must_use]
pub const fn schema_id() -> &'static str {
    HEALTH_SCHEMA_ID
}

/// Returns the health schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    HEALTH_SCHEMA_VERSION
}

/// Returns the component safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    "unsafe-rust-forbidden"
}

/// Returns the scalability policy.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    "open-world-resource-bounded-health-observations"
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::HardwareDeviceIdentity;

    fn provider_id() -> ProviderId {
        ProviderId::new(
            "org.zamani.test.hardware",
        )
        .expect("valid provider identity")
    }

    fn provider_subject() -> HealthSubject {
        HealthSubject::Provider(provider_id())
    }

    fn device_subject() -> HealthSubject {
        HealthSubject::Device(
            HardwareDeviceIdentity::new(
                provider_id(),
                DeviceId::new("device.test.001")
                    .expect("valid device identity"),
            ),
        )
    }

    fn timestamp(value: u128) -> HealthTimestamp {
        HealthTimestamp::from_unix_nanos(value)
    }

    fn observation(
        check: &str,
        status: HealthStatus,
    ) -> HealthObservation {
        HealthObservation::new(
            device_subject(),
            HealthCheckId::new(check)
                .expect("valid check identity"),
            status,
            timestamp(1_000),
            "provider.health_probe",
        )
        .expect("valid health observation")
    }

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            component_id(),
            "zhal.devices.health",
        );
    }

    #[test]
    fn schema_contract_is_available() {
        assert_eq!(
            schema_id(),
            "zamani.zhal.devices.health",
        );

        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn health_check_identifiers_are_open_world() {
        let check = HealthCheckId::new(
            "future.photonic.optical_stability",
        )
        .expect("future check");

        assert_eq!(
            check.as_str(),
            "future.photonic.optical_stability",
        );
    }

    #[test]
    fn health_check_identifiers_reject_whitespace() {
        assert!(
            HealthCheckId::new(
                "hardware device health",
            )
            .is_err()
        );
    }

    #[test]
    fn health_check_identifiers_have_no_artificial_length_limit() {
        let identifier = format!(
            "future.{}",
            "photonic.".repeat(1_000),
        );

        let check = HealthCheckId::new(
            identifier.clone(),
        )
        .expect("long identifier");

        assert_eq!(check.as_str(), identifier);
    }

    #[test]
    fn provider_and_device_subjects_preserve_identity() {
        let provider = provider_subject();

        assert!(provider.is_provider());
        assert!(provider.device_identity().is_none());

        let device = device_subject();

        assert!(device.is_device());
        assert!(device.device_identity().is_some());

        assert_eq!(
            provider.provider_id(),
            device.provider_id(),
        );
    }

    #[test]
    fn unknown_is_not_healthy() {
        assert!(!HealthStatus::Unknown.is_healthy());
        assert!(!HealthStatus::Unknown.is_operational());
    }

    #[test]
    fn degraded_is_potentially_operational() {
        assert!(
            HealthStatus::Degraded.is_operational()
        );

        assert!(
            !HealthStatus::Degraded.is_healthy()
        );
    }

    #[test]
    fn health_severity_is_deterministic() {
        assert_eq!(
            HealthStatus::Healthy.max_severity(
                HealthStatus::Degraded,
            ),
            HealthStatus::Degraded,
        );

        assert_eq!(
            HealthStatus::Failed.max_severity(
                HealthStatus::Unavailable,
            ),
            HealthStatus::Failed,
        );
    }

    #[test]
    fn diagnostics_reject_secret_material() {
        assert!(
            HealthDiagnostic::new(
                "hardware.communication.failure",
                HealthSeverity::Error,
                "authorization: Bearer confidential-value",
            )
            .is_err()
        );
    }

    #[test]
    fn diagnostics_accept_safe_messages() {
        let diagnostic = HealthDiagnostic::new(
            "hardware.temperature.warning",
            HealthSeverity::Warning,
            "Temperature observation exceeded the configured warning threshold.",
        )
        .expect("safe diagnostic");

        assert_eq!(
            diagnostic.severity(),
            HealthSeverity::Warning,
        );
    }

    #[test]
    fn observation_preserves_provenance() {
        let observation = observation(
            checks::DEVICE_OPERATIONAL,
            HealthStatus::Healthy,
        );

        assert_eq!(
            observation.provenance().source(),
            "provider.health_probe",
        );
    }

    #[test]
    fn observations_support_external_evidence() {
        let observation = observation(
            checks::DEVICE_OPERATIONAL,
            HealthStatus::Healthy,
        )
        .with_evidence_reference(
            "evidence.device.health.001",
        )
        .expect("evidence reference");

        assert_eq!(
            observation.provenance().evidence_reference(),
            Some("evidence.device.health.001"),
        );
    }

    #[test]
    fn reports_start_unknown() {
        let report = HealthReport::empty(
            device_subject(),
        );

        assert_eq!(
            report.status(),
            HealthStatus::Unknown,
        );

        assert!(!report.is_healthy());
        assert!(!report.is_operational());
        assert!(report.is_empty());
    }

    #[test]
    fn reports_aggregate_healthy_observations() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
                observation(
                    checks::MEMORY,
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("valid report");

        assert_eq!(
            report.status(),
            HealthStatus::Healthy,
        );

        assert!(report.is_healthy());
    }

    #[test]
    fn reports_preserve_degraded_state() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
                observation(
                    checks::THERMAL,
                    HealthStatus::Degraded,
                ),
            ],
        )
        .expect("valid report");

        assert_eq!(
            report.status(),
            HealthStatus::Degraded,
        );

        assert!(report.is_operational());
        assert!(!report.is_healthy());
    }

    #[test]
    fn unknown_observation_is_not_silently_healthy() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
                observation(
                    "hardware.future_subsystem",
                    HealthStatus::Unknown,
                ),
            ],
        )
        .expect("valid report");

        assert_eq!(
            report.status(),
            HealthStatus::Unknown,
        );
    }

    #[test]
    fn reports_reject_mixed_subjects() {
        let result = HealthReport::from_observations(
            device_subject(),
            vec![
                HealthObservation::new(
                    provider_subject(),
                    HealthCheckId::new(
                        checks::PROVIDER_SERVICE,
                    )
                    .expect("check"),
                    HealthStatus::Healthy,
                    timestamp(1_000),
                    "provider.health_probe",
                )
                .expect("observation"),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn reports_reject_duplicate_check_identifiers() {
        let result = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
                observation(
                    checks::PROCESSING,
                    HealthStatus::Degraded,
                ),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn reports_sort_observations_deterministically() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    "hardware.zeta",
                    HealthStatus::Healthy,
                ),
                observation(
                    "hardware.alpha",
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("valid report");

        assert_eq!(
            report.observations()[0].check().as_str(),
            "hardware.alpha",
        );

        assert_eq!(
            report.observations()[1].check().as_str(),
            "hardware.zeta",
        );
    }

    #[test]
    fn reports_support_check_lookup() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("report");

        let check = HealthCheckId::new(
            checks::PROCESSING,
        )
        .expect("check");

        assert!(report.contains_check(&check));

        assert_eq!(
            report.observation(&check).map(
                HealthObservation::status,
            ),
            Some(HealthStatus::Healthy),
        );
    }

    #[test]
    fn future_timestamps_are_not_treated_as_fresh() {
        let observation = observation(
            checks::PROCESSING,
            HealthStatus::Healthy,
        );

        let policy = ObservationPolicy::default();

        assert_eq!(
            observation.freshness_at(
                timestamp(2_000),
                &policy,
            ),
            HealthFreshness::FutureDated,
        );
    }

    #[test]
    fn freshness_is_assessed_using_existing_policy() {
        let observation = observation(
            checks::PROCESSING,
            HealthStatus::Healthy,
        );

        let policy = ObservationPolicy::default()
            .with_maximum_age(
                Some(std::time::Duration::from_nanos(100)),
            );

        assert_eq!(
            observation.freshness_at(
                timestamp(1_050),
                &policy,
            ),
            HealthFreshness::Fresh,
        );

        assert_eq!(
            observation.freshness_at(
                timestamp(1_500),
                &policy,
            ),
            HealthFreshness::Stale,
        );
    }

    #[test]
    fn stale_policy_can_require_rejection() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("report");

        let policy = ObservationPolicy::default()
            .with_maximum_age(
                Some(std::time::Duration::from_nanos(10)),
            )
            .with_stale_policy(
                StaleObservationPolicy::RejectForFreshnessDependentDecisions,
            );

        let assessment = report.assess(
            timestamp(2_000),
            &policy,
        );

        assert!(
            assessment.must_reject_freshness_dependent_decision()
        );
    }

    #[test]
    fn freshness_does_not_change_observed_health_status() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("report");

        let policy = ObservationPolicy::default()
            .with_maximum_age(
                Some(std::time::Duration::from_nanos(1)),
            );

        let assessment = report.assess(
            timestamp(2_000),
            &policy,
        );

        assert_eq!(
            assessment.status(),
            HealthStatus::Healthy,
        );

        assert!(
            assessment.contains_stale_observations()
        );
    }

    #[test]
    fn report_validation_checks_aggregate_consistency() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::PROCESSING,
                    HealthStatus::Degraded,
                ),
            ],
        )
        .expect("report");

        assert!(report.validate().is_ok());
    }

    #[test]
    fn reports_have_no_fixed_observation_ceiling() {
        let subject = device_subject();

        let observations = (0..2_000)
            .map(|index| {
                observation(
                    &format!("hardware.extension.{index}"),
                    HealthStatus::Healthy,
                )
            })
            .collect();

        let report = HealthReport::from_observations(
            subject,
            observations,
        )
        .expect("large report");

        assert_eq!(report.len(), 2_000);
        assert!(report.is_healthy());
    }

    #[test]
    fn report_provider_identity_is_canonical() {
        let report = HealthReport::empty(
            provider_subject(),
        );

        assert_eq!(
            report.provider_id(),
            &provider_id(),
        );

        assert!(report.device_identity().is_none());
    }

    #[test]
    fn report_device_identity_is_preserved() {
        let report = HealthReport::from_observations(
            device_subject(),
            vec![
                observation(
                    checks::DEVICE_OPERATIONAL,
                    HealthStatus::Healthy,
                ),
            ],
        )
        .expect("report");

        assert!(report.device_identity().is_some());
    }

    #[test]
    fn scalability_policy_is_open_world() {
        assert!(
            scalability_policy().contains("open-world")
        );
    }

    #[test]
    fn safety_policy_forbids_unsafe_rust() {
        assert_eq!(
            safety_policy(),
            "unsafe-rust-forbidden",
        );
    }
}
