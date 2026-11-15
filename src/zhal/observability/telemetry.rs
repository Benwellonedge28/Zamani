
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Telemetry
//!
//! Path:
//!     src/zhal/observability/telemetry.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines the provider-neutral telemetry observation contract for ZHAL.
//!
//! This module describes observations about physical hardware and hardware
//! providers without introducing another metrics registry, tracing system,
//! event journal, persistence engine, or telemetry exporter.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Hardware telemetry observation contracts.
//! - Hardware observation classifications.
//! - Hardware measurement values.
//! - Hardware measurement units.
//! - Provider attribution.
//! - Opaque hardware subject references.
//! - Explicit observation timestamps.
//! - Execution outcome observations.
//! - Hardware event observations.
//! - Performance and resource observations.
//! - Hardware fault observations.
//! - Telemetry validation.
//! - Telemetry sink interfaces.
//! - Telemetry-specific errors.
//!
//! This module does NOT own:
//!
//! - Hardware discovery.
//! - Device inventory.
//! - Provider registration.
//! - Provider lifecycle.
//! - Device allocation.
//! - Resource allocation.
//! - Scheduling.
//! - Routing.
//! - Quantum execution.
//! - HDL realization.
//! - Classical execution.
//! - Distributed execution.
//! - Hardware performance-counter discovery.
//! - Metric registration or aggregation.
//! - Trace identity allocation.
//! - Trace propagation.
//! - Logging.
//! - Persistence.
//! - Network transport.
//! - Authentication.
//! - Authorization.
//! - Telemetry export protocols.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # Canonical integration authorities
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Operation context:
//!     crate::stdlib::context::OperationContext
//!
//! ZUR observability:
//!     crate::zur::observability
//!
//! Generic metrics:
//!     crate::distributed::observability::metrics
//!
//! Distributed events:
//!     crate::distributed::observability::events
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Quantum hardware telemetry:
//!     crate::quantum::hardware::telemetry
//!
//! Backend telemetry:
//!     crate::backend::telemetry
//!
//! # Fundamental principle
//!
//! Telemetry is observational.
//!
//! Recording, validating, transporting, or exporting an observation MUST NOT
//! change the meaning or outcome of the computation being observed.
//!
//! A telemetry failure is not automatically a hardware execution failure.
//!
//! A successful telemetry submission is not proof of durable persistence.
//!
//! # POCO-REAF
//!
//! This module is independent of:
//!
//! - Processor architecture.
//! - CPU count.
//! - GPU count.
//! - Accelerator count.
//! - QPU count.
//! - Qubit count.
//! - FPGA count.
//! - ASIC structure.
//! - Memory capacity.
//! - Register width.
//! - Network size.
//! - Device count.
//! - Computational domain.
//!
//! No arbitrary universal hardware or telemetry capacity is imposed.
//!
//! Actual observation throughput and retention depend on the supplied sink,
//! available resources, and explicitly configured operational policies.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! # Privacy
//!
//! Telemetry must not be used to store:
//!
//! - Credentials.
//! - Passwords.
//! - Private keys.
//! - Authentication tokens.
//! - Authorization headers.
//! - Private execution payloads.
//! - Quantum circuit payloads.
//! - Arbitrary source text.
//! - Sensitive application data.
//!
//! Producers remain responsible for data classification and sanitization.
//!
//! # Determinism
//!
//! This module:
//!
//! - Does not read the system clock.
//! - Does not generate random identifiers.
//! - Does not access environment variables.
//! - Does not access the filesystem.
//! - Does not access the network.
//! - Does not create background workers.
//! - Does not maintain process-global mutable state.
//!
//! Timestamps are explicit producer-supplied observations.
//!
//! # Concurrency
//!
//! Telemetry sinks must implement Send + Sync.
//!
//! The module does not prescribe a particular synchronization or storage
//! strategy for sinks.
//!
//! A sink may use a mutex, sharding, batching, external storage, or another
//! implementation, provided its own concurrency contract is satisfied.
//!
//! # Completion invariant
//!
//! This file is complete when:
//!
//! 1. Hardware observations have stable contracts.
//! 2. Provider identity reuses stdlib::provider::ProviderId.
//! 3. Hardware subject references remain opaque.
//! 4. Measurements reject non-finite floating-point values.
//! 5. Timestamps are validated.
//! 6. No arbitrary universal hardware limits are introduced.
//! 7. No second generic metrics registry is created.
//! 8. No duplicate tracing authority is created.
//! 9. No automatic I/O or background workers exist.
//! 10. Sink failures remain separate from computation failures.
//! 11. Tests cover validation and portability.
//! 12. The module compiles on the repository Rust baseline.

use core::fmt;
use core::time::Duration;

use std::sync::Arc;

use crate::stdlib::provider::ProviderId;

// ============================================================================
// API AND SCHEMA VERSION
// ============================================================================

/// Stable hardware telemetry schema identifier.
pub const TELEMETRY_SCHEMA_ID: &str =
    "zamani.zhal.observability.telemetry";

/// Hardware telemetry API major version.
pub const TELEMETRY_API_MAJOR: u16 = 1;

/// Hardware telemetry API minor version.
pub const TELEMETRY_API_MINOR: u16 = 0;

/// Hardware telemetry API patch version.
pub const TELEMETRY_API_PATCH: u16 = 0;

/// Complete hardware telemetry API version.
pub const TELEMETRY_API_VERSION: (u16, u16, u16) = (
    TELEMETRY_API_MAJOR,
    TELEMETRY_API_MINOR,
    TELEMETRY_API_PATCH,
);

/// Returns the stable telemetry schema identifier.
#[must_use]
pub const fn telemetry_schema_id() -> &'static str {
    TELEMETRY_SCHEMA_ID
}

/// Returns the hardware telemetry API version.
#[must_use]
pub const fn telemetry_api_version() -> (u16, u16, u16) {
    TELEMETRY_API_VERSION
}

// ============================================================================
// RESULT
// ============================================================================

/// Result of a hardware telemetry operation.
pub type TelemetryResult<T> =
    std::result::Result<T, TelemetryError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Machine-readable telemetry error classifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TelemetryErrorKind {
    /// An identifier is invalid.
    InvalidIdentifier,

    /// A measurement contains an invalid numeric value.
    InvalidMeasurement,

    /// A measurement unit is invalid.
    InvalidUnit,

    /// A timestamp is invalid.
    InvalidTimestamp,

    /// An observation violates its contract.
    InvalidObservation,

    /// The telemetry sink rejected an observation.
    SinkRejected,

    /// The telemetry sink encountered an operational failure.
    SinkFailure,

    /// An internal telemetry invariant was violated.
    InternalInvariantViolation,
}

/// Telemetry-specific error.
///
/// This error is intentionally separate from hardware execution errors.
///
/// Telemetry failure must not automatically be converted into computation
/// failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryError {
    kind: TelemetryErrorKind,
    component: &'static str,
}

impl TelemetryError {
    /// Constructs a telemetry error.
    #[must_use]
    pub const fn new(kind: TelemetryErrorKind) -> Self {
        Self {
            kind,
            component: "zhal.observability.telemetry",
        }
    }

    /// Returns the machine-readable error classification.
    #[must_use]
    pub const fn kind(&self) -> TelemetryErrorKind {
        self.kind
    }

    /// Returns the originating component.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }
}

impl fmt::Display for TelemetryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let description = match self.kind {
            TelemetryErrorKind::InvalidIdentifier =>
                "invalid telemetry identifier",

            TelemetryErrorKind::InvalidMeasurement =>
                "invalid telemetry measurement",

            TelemetryErrorKind::InvalidUnit =>
                "invalid telemetry measurement unit",

            TelemetryErrorKind::InvalidTimestamp =>
                "invalid telemetry timestamp",

            TelemetryErrorKind::InvalidObservation =>
                "invalid hardware telemetry observation",

            TelemetryErrorKind::SinkRejected =>
                "telemetry sink rejected observation",

            TelemetryErrorKind::SinkFailure =>
                "telemetry sink operation failed",

            TelemetryErrorKind::InternalInvariantViolation =>
                "telemetry internal invariant violated",
        };

        write!(
            formatter,
            "{}: {}",
            self.component,
            description
        )
    }
}

impl std::error::Error for TelemetryError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates an open-world qualified telemetry identifier.
///
/// Identifiers are dot-separated.
///
/// Examples:
///
/// - hardware.temperature
/// - hardware.memory.utilization
/// - quantum.execution.latency
/// - hdl.realization.duration
/// - accelerator.compute.throughput
/// - distributed.transport.bandwidth
/// - future.photonic.observation
///
/// No arbitrary identifier length or namespace-depth limit is imposed.
fn validate_qualified_identifier(
    value: &str,
) -> TelemetryResult<()> {
    if value.is_empty() || value.trim() != value {
        return Err(TelemetryError::new(
            TelemetryErrorKind::InvalidIdentifier,
        ));
    }

    let valid = value.split('.').all(|segment| {
        let mut bytes = segment.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        first.is_ascii_alphanumeric()
            && bytes.all(|byte| {
                byte.is_ascii_alphanumeric()
                    || byte == b'_'
                    || byte == b'-'
            })
    });

    if !valid {
        return Err(TelemetryError::new(
            TelemetryErrorKind::InvalidIdentifier,
        ));
    }

    Ok(())
}

/// Validates descriptive text without imposing an arbitrary length limit.
fn validate_descriptive_text(
    value: &str,
    error: TelemetryErrorKind,
) -> TelemetryResult<()> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(TelemetryError::new(error));
    }

    Ok(())
}

// ============================================================================
// TELEMETRY IDENTIFIERS
// ============================================================================

/// Open-world identity of a telemetry observation type.
///
/// This is not a hardware device identity or provider identity.
///
/// It identifies the kind of observation being represented.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TelemetryName(String);

impl TelemetryName {
    /// Constructs a validated observation name.
    pub fn new(
        value: impl Into<String>,
    ) -> TelemetryResult<Self> {
        let value = value.into();

        validate_qualified_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the exact observation name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TelemetryName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Open-world measurement unit.
///
/// Units are descriptive metadata. This type does not perform dimensional
/// analysis or unit conversion.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TelemetryUnit(String);

impl TelemetryUnit {
    /// Constructs a validated measurement unit.
    ///
    /// Units may contain standard unit notation, including `/`, `*`,
    /// exponents, and Unicode symbols.
    pub fn new(
        value: impl Into<String>,
    ) -> TelemetryResult<Self> {
        let value = value.into();

        validate_descriptive_text(
            &value,
            TelemetryErrorKind::InvalidUnit,
        )?;

        Ok(Self(value))
    }

    /// Returns the exact unit representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TelemetryUnit {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// HARDWARE SUBJECT REFERENCES
// ============================================================================

/// Identifies the kind of entity to which a telemetry observation relates.
///
/// This is an opaque reference, not a competing hardware identity system.
///
/// Canonical hardware and device identity remain owned by the corresponding
/// hardware identity authorities.
///
/// The namespace identifies the authority that defines the referenced
/// identity.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TelemetrySubjectRef {
    namespace: String,
    identity: String,
}

impl TelemetrySubjectRef {
    /// Constructs an opaque telemetry subject reference.
    pub fn new(
        namespace: impl Into<String>,
        identity: impl Into<String>,
    ) -> TelemetryResult<Self> {
        let namespace = namespace.into();
        let identity = identity.into();

        validate_qualified_identifier(&namespace)?;

        validate_descriptive_text(
            &identity,
            TelemetryErrorKind::InvalidIdentifier,
        )?;

        Ok(Self {
            namespace,
            identity,
        })
    }

    /// Returns the identity-owning namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the opaque identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

// ============================================================================
// EXPLICIT OBSERVATION TIMESTAMP
// ============================================================================

/// Explicit wall-clock timestamp supplied by an observation producer.
///
/// This type does not read the system clock.
///
/// Unix seconds are signed to support timestamps before the Unix epoch.
/// Nanoseconds must be in the valid subsecond range.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ObservationTimestamp {
    unix_seconds: i64,
    nanoseconds: u32,
}

impl ObservationTimestamp {
    /// Constructs a timestamp.
    ///
    /// # Errors
    ///
    /// Returns InvalidTimestamp when nanoseconds are outside the
    /// valid subsecond range.
    pub fn new(
        unix_seconds: i64,
        nanoseconds: u32,
    ) -> TelemetryResult<Self> {
        if nanoseconds >= 1_000_000_000 {
            return Err(TelemetryError::new(
                TelemetryErrorKind::InvalidTimestamp,
            ));
        }

        Ok(Self {
            unix_seconds,
            nanoseconds,
        })
    }

    /// Returns Unix seconds.
    #[must_use]
    pub const fn unix_seconds(&self) -> i64 {
        self.unix_seconds
    }

    /// Returns the nanosecond component.
    #[must_use]
    pub const fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
}

// ============================================================================
// OBSERVATION CLASSIFICATION
// ============================================================================

/// General classification of a hardware observation.
///
/// The observation name remains open-world, so additional hardware
/// technologies do not require extending this enum.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum ObservationClass {
    /// A discrete hardware lifecycle event.
    Event,

    /// A quantitative hardware measurement.
    Measurement,

    /// A physical or operational resource observation.
    Resource,

    /// A performance observation.
    Performance,

    /// A hardware health observation.
    Health,

    /// A hardware fault or degradation observation.
    Fault,

    /// A provider communication observation.
    Communication,

    /// An extensible observation not covered by another category.
    Extension,
}

// ============================================================================
// OBSERVATION OUTCOME
// ============================================================================

/// Outcome reported by the authoritative producer.
///
/// An outcome is an observation, not an independent verification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum ObservationOutcome {
    /// The observed operation completed successfully.
    Succeeded,

    /// The observed operation failed.
    Failed,

    /// Cancellation was requested or reported.
    Cancelled,

    /// The operation is still pending.
    Pending,

    /// The operation partially completed.
    Partial,

    /// The final outcome has not been established.
    Unknown,
}

// ============================================================================
// MEASUREMENT VALUE
// ============================================================================

/// Numeric telemetry measurement value.
///
/// Integer representations are preserved exactly.
///
/// Floating-point representations must be finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MeasurementValue {
    /// Unsigned integer measurement.
    Unsigned(u128),

    /// Signed integer measurement.
    Signed(i128),

    /// Finite floating-point measurement.
    Float(f64),

    /// Duration represented in nanoseconds.
    DurationNanoseconds(u128),
}

impl MeasurementValue {
    /// Constructs a validated floating-point measurement.
    pub fn float(
        value: f64,
    ) -> TelemetryResult<Self> {
        if !value.is_finite() {
            return Err(TelemetryError::new(
                TelemetryErrorKind::InvalidMeasurement,
            ));
        }

        Ok(Self::Float(value))
    }

    /// Returns whether this measurement contains a finite value.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Float(value) => value.is_finite(),
            Self::Unsigned(_) => true,
            Self::Signed(_) => true,
            Self::DurationNanoseconds(_) => true,
        }
    }
}

// ============================================================================
// MEASUREMENT
// ============================================================================

/// One quantitative hardware observation.
///
/// This is a data contract, not a metric registry.
///
/// Generic aggregation and metric-series management remain owned by
/// distributed observability.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareMeasurement {
    name: TelemetryName,
    value: MeasurementValue,
    unit: TelemetryUnit,
}

impl HardwareMeasurement {
    /// Constructs a hardware measurement.
    pub fn new(
        name: TelemetryName,
        value: MeasurementValue,
        unit: TelemetryUnit,
    ) -> TelemetryResult<Self> {
        if !value.is_valid() {
            return Err(TelemetryError::new(
                TelemetryErrorKind::InvalidMeasurement,
            ));
        }

        Ok(Self {
            name,
            value,
            unit,
        })
    }

    /// Returns the measurement name.
    #[must_use]
    pub fn name(&self) -> &TelemetryName {
        &self.name
    }

    /// Returns the numeric value.
    #[must_use]
    pub const fn value(&self) -> MeasurementValue {
        self.value
    }

    /// Returns the measurement unit.
    #[must_use]
    pub fn unit(&self) -> &TelemetryUnit {
        &self.unit
    }
}

// ============================================================================
// HARDWARE TELEMETRY OBSERVATION
// ============================================================================

/// Immutable-by-interface observation supplied by a hardware producer.
///
/// The observation contains no arbitrary execution payload.
///
/// Provider identity uses the canonical standard-library ProviderId.
///
/// Hardware subject references remain opaque and do not redefine device
/// identity.
///
/// A measurement is optional because not every observation is quantitative.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareTelemetryObservation {
    provider_id: ProviderId,
    name: TelemetryName,
    class: ObservationClass,
    subject: Option<TelemetrySubjectRef>,
    timestamp: Option<ObservationTimestamp>,
    duration: Option<Duration>,
    outcome: Option<ObservationOutcome>,
    measurement: Option<HardwareMeasurement>,
}

impl HardwareTelemetryObservation {
    /// Constructs a hardware telemetry observation.
    ///
    /// The timestamp is optional because the producer may not have a
    /// trustworthy wall-clock source.
    pub fn new(
        provider_id: ProviderId,
        name: TelemetryName,
        class: ObservationClass,
    ) -> Self {
        Self {
            provider_id,
            name,
            class,
            subject: None,
            timestamp: None,
            duration: None,
            outcome: None,
            measurement: None,
        }
    }

    /// Attaches an opaque hardware subject reference.
    #[must_use]
    pub fn with_subject(
        mut self,
        subject: TelemetrySubjectRef,
    ) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Attaches an explicit producer-supplied timestamp.
    #[must_use]
    pub const fn with_timestamp(
        mut self,
        timestamp: ObservationTimestamp,
    ) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Attaches an elapsed duration.
    #[must_use]
    pub const fn with_duration(
        mut self,
        duration: Duration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Attaches an observed operation outcome.
    #[must_use]
    pub const fn with_outcome(
        mut self,
        outcome: ObservationOutcome,
    ) -> Self {
        self.outcome = Some(outcome);
        self
    }

    /// Attaches a quantitative measurement.
    #[must_use]
    pub fn with_measurement(
        mut self,
        measurement: HardwareMeasurement,
    ) -> Self {
        self.measurement = Some(measurement);
        self
    }

    /// Validates the observation's internal contract.
    ///
    /// # Errors
    ///
    /// Returns InvalidObservation if the classification and supplied
    /// measurement are inconsistent.
    pub fn validate(&self) -> TelemetryResult<()> {
        if let Some(measurement) = &self.measurement {
            if !measurement.value().is_valid() {
                return Err(TelemetryError::new(
                    TelemetryErrorKind::InvalidMeasurement,
                ));
            }
        }

        match self.class {
            ObservationClass::Measurement
            | ObservationClass::Resource
            | ObservationClass::Performance => {
                if self.measurement.is_none() {
                    return Err(TelemetryError::new(
                        TelemetryErrorKind::InvalidObservation,
                    ));
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the observation name.
    #[must_use]
    pub fn name(&self) -> &TelemetryName {
        &self.name
    }

    /// Returns the observation classification.
    #[must_use]
    pub const fn class(&self) -> ObservationClass {
        self.class
    }

    /// Returns the optional hardware subject.
    #[must_use]
    pub fn subject(&self) -> Option<&TelemetrySubjectRef> {
        self.subject.as_ref()
    }

    /// Returns the optional timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> Option<ObservationTimestamp> {
        self.timestamp
    }

    /// Returns the optional elapsed duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Returns the optional operation outcome.
    #[must_use]
    pub const fn outcome(&self) -> Option<ObservationOutcome> {
        self.outcome
    }

    /// Returns the optional quantitative measurement.
    #[must_use]
    pub fn measurement(&self) -> Option<&HardwareMeasurement> {
        self.measurement.as_ref()
    }
}

// ============================================================================
// TELEMETRY SINK CONTRACT
// ============================================================================

/// Provider-neutral destination for hardware telemetry observations.
///
/// Implementations may forward observations to existing metrics, event,
/// profiling, or external observability infrastructure.
///
/// This contract does not prescribe storage, networking, buffering,
/// synchronization, retention, or export protocols.
///
/// Implementations must be thread-safe.
///
/// A sink must not modify the original observation.
///
/// Sink errors must remain distinct from hardware execution outcomes.
pub trait HardwareTelemetrySink: Send + Sync {
    /// Accepts one validated hardware telemetry observation.
    ///
    /// # Errors
    ///
    /// Returns a telemetry error if the sink cannot accept the observation.
    ///
    /// Success means only that the sink accepted the observation according
    /// to its own contract. It does not establish durable persistence.
    fn record(
        &self,
        observation: &HardwareTelemetryObservation,
    ) -> TelemetryResult<()>;
}

// ============================================================================
// TELEMETRY RECORDER
// ============================================================================

/// Explicit handle to an injected hardware telemetry sink.
///
/// Cloning this handle shares the same sink implementation.
///
/// It creates no background workers and performs no automatic I/O.
#[derive(Clone)]
pub struct HardwareTelemetryRecorder {
    sink: Arc<dyn HardwareTelemetrySink>,
}

impl HardwareTelemetryRecorder {
    /// Constructs a recorder from an explicit sink.
    #[must_use]
    pub fn new(
        sink: Arc<dyn HardwareTelemetrySink>,
    ) -> Self {
        Self { sink }
    }

    /// Records a validated observation.
    ///
    /// The caller decides how telemetry failures are handled.
    ///
    /// This method never converts a telemetry error into a hardware
    /// execution result.
    pub fn record(
        &self,
        observation: &HardwareTelemetryObservation,
    ) -> TelemetryResult<()> {
        observation.validate()?;

        self.sink.record(observation)
    }
}

impl fmt::Debug for HardwareTelemetryRecorder {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareTelemetryRecorder")
            .finish_non_exhaustive()
    }
}

// ============================================================================
// NO-OP TELEMETRY SINK
// ============================================================================

/// Telemetry sink that intentionally discards observations.
///
/// This supports:
///
/// - Minimal embedded environments.
/// - Runtime-free deployments.
/// - Builds where optional observability is disabled.
/// - Applications that inject telemetry only conditionally.
///
/// The no-op sink does not claim that an observation was persisted.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopHardwareTelemetrySink;

impl HardwareTelemetrySink for NoopHardwareTelemetrySink {
    fn record(
        &self,
        _observation: &HardwareTelemetryObservation,
    ) -> TelemetryResult<()> {
        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    fn provider() -> ProviderId {
        ProviderId::new("zamani.test.hardware")
            .expect("valid provider identity")
    }

    fn name(value: &str) -> TelemetryName {
        TelemetryName::new(value)
            .expect("valid telemetry name")
    }

    fn unit(value: &str) -> TelemetryUnit {
        TelemetryUnit::new(value)
            .expect("valid telemetry unit")
    }

    fn measurement() -> HardwareMeasurement {
        HardwareMeasurement::new(
            name("hardware.temperature"),
            MeasurementValue::Float(42.5),
            unit("degC"),
        )
        .expect("valid measurement")
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            telemetry_schema_id(),
            "zamani.zhal.observability.telemetry"
        );

        assert_eq!(
            telemetry_api_version(),
            (1, 0, 0)
        );
    }

    #[test]
    fn accepts_future_hardware_namespaces() {
        for value in [
            "classical.processor.temperature",
            "quantum.execution.latency",
            "hdl.realization.duration",
            "accelerator.compute.throughput",
            "distributed.transport.bandwidth",
            "embedded.sensor.observation",
            "photonic.hardware.observation",
            "future.nanotechnology.measurement",
        ] {
            assert_eq!(
                TelemetryName::new(value)
                    .expect("valid open-world name")
                    .as_str(),
                value
            );
        }
    }

    #[test]
    fn rejects_invalid_qualified_identifiers() {
        for value in [
            "",
            ".hardware.temperature",
            "hardware.",
            "hardware..temperature",
            "hardware temperature",
            "Hardware.temperature",
            "hardware/temperature",
        ] {
            assert!(
                TelemetryName::new(value).is_err(),
                "unexpectedly accepted {value}"
            );
        }
    }

    #[test]
    fn accepts_unicode_measurement_units() {
        assert_eq!(
            unit("μs").as_str(),
            "μs"
        );

        assert_eq!(
            unit("°C").as_str(),
            "°C"
        );

        assert_eq!(
            unit("bytes/s").as_str(),
            "bytes/s"
        );
    }

    #[test]
    fn rejects_non_finite_measurements() {
        assert!(
            MeasurementValue::float(f64::NAN).is_err()
        );

        assert!(
            MeasurementValue::float(f64::INFINITY).is_err()
        );

        assert!(
            MeasurementValue::float(f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn preserves_large_integer_measurements() {
        let value = u128::MAX;

        let measurement = HardwareMeasurement::new(
            name("hardware.capacity"),
            MeasurementValue::Unsigned(value),
            unit("bytes"),
        )
        .expect("valid measurement");

        assert_eq!(
            measurement.value(),
            MeasurementValue::Unsigned(value)
        );
    }

    #[test]
    fn preserves_signed_measurements() {
        let measurement = HardwareMeasurement::new(
            name("hardware.temperature.offset"),
            MeasurementValue::Signed(-42),
            unit("milli_degC"),
        )
        .expect("valid measurement");

        assert_eq!(
            measurement.value(),
            MeasurementValue::Signed(-42)
        );
    }

    #[test]
    fn validates_timestamp_nanoseconds() {
        assert!(
            ObservationTimestamp::new(
                1_700_000_000,
                999_999_999
            ).is_ok()
        );

        assert!(
            ObservationTimestamp::new(
                1_700_000_000,
                1_000_000_000
            ).is_err()
        );
    }

    #[test]
    fn supports_pre_epoch_timestamps() {
        let timestamp = ObservationTimestamp::new(
            -1,
            500_000_000,
        )
        .expect("valid timestamp");

        assert_eq!(timestamp.unix_seconds(), -1);
        assert_eq!(timestamp.nanoseconds(), 500_000_000);
    }

    #[test]
    fn validates_opaque_hardware_subjects() {
        let subject = TelemetrySubjectRef::new(
            "zhal.hardware",
            "device.example-01",
        )
        .expect("valid subject");

        assert_eq!(
            subject.namespace(),
            "zhal.hardware"
        );

        assert_eq!(
            subject.identity(),
            "device.example-01"
        );
    }

    #[test]
    fn rejects_empty_hardware_subjects() {
        assert!(
            TelemetrySubjectRef::new(
                "zhal.hardware",
                "",
            ).is_err()
        );

        assert!(
            TelemetrySubjectRef::new(
                "",
                "device.example",
            ).is_err()
        );
    }

    #[test]
    fn measurement_observation_requires_measurement() {
        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.temperature"),
            ObservationClass::Measurement,
        );

        assert!(
            observation.validate().is_err()
        );
    }

    #[test]
    fn resource_observation_requires_measurement() {
        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.memory.available"),
            ObservationClass::Resource,
        );

        assert!(
            observation.validate().is_err()
        );
    }

    #[test]
    fn event_observation_does_not_require_measurement() {
        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.device.connected"),
            ObservationClass::Event,
        );

        assert!(
            observation.validate().is_ok()
        );
    }

    #[test]
    fn constructs_complete_hardware_observation() {
        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.temperature"),
            ObservationClass::Measurement,
        )
        .with_subject(
            TelemetrySubjectRef::new(
                "zhal.hardware",
                "device.example",
            )
            .unwrap(),
        )
        .with_timestamp(
            ObservationTimestamp::new(
                1_700_000_000,
                0,
            )
            .unwrap(),
        )
        .with_duration(
            Duration::from_millis(12),
        )
        .with_outcome(
            ObservationOutcome::Succeeded,
        )
        .with_measurement(
            measurement(),
        );

        assert!(
            observation.validate().is_ok()
        );

        assert_eq!(
            observation.provider_id().as_str(),
            "zamani.test.hardware"
        );

        assert_eq!(
            observation.class(),
            ObservationClass::Measurement
        );

        assert!(
            observation.measurement().is_some()
        );
    }

    #[derive(Default)]
    struct TestSink {
        observations: Mutex<Vec<HardwareTelemetryObservation>>,
    }

    impl HardwareTelemetrySink for TestSink {
        fn record(
            &self,
            observation: &HardwareTelemetryObservation,
        ) -> TelemetryResult<()> {
            self.observations
                .lock()
                .map_err(|_| {
                    TelemetryError::new(
                        TelemetryErrorKind::SinkFailure,
                    )
                })?
                .push(observation.clone());

            Ok(())
        }
    }

    #[test]
    fn recorder_delivers_observations_to_injected_sink() {
        let sink = Arc::new(TestSink::default());

        let recorder = HardwareTelemetryRecorder::new(
            sink.clone()
        );

        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.device.connected"),
            ObservationClass::Event,
        );

        recorder
            .record(&observation)
            .expect("observation accepted");

        let stored = sink.observations
            .lock()
            .unwrap();

        assert_eq!(stored.len(), 1);

        assert_eq!(
            stored[0].name().as_str(),
            "hardware.device.connected"
        );
    }

    #[test]
    fn recorder_shares_sink_between_clones() {
        let sink = Arc::new(TestSink::default());

        let recorder = HardwareTelemetryRecorder::new(
            sink.clone()
        );

        let cloned = recorder.clone();

        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.device.disconnected"),
            ObservationClass::Event,
        );

        cloned.record(&observation).unwrap();

        assert_eq!(
            sink.observations.lock().unwrap().len(),
            1
        );
    }

    #[test]
    fn noop_sink_accepts_observations() {
        let sink = NoopHardwareTelemetrySink;

        let observation = HardwareTelemetryObservation::new(
            provider(),
            name("hardware.observation"),
            ObservationClass::Event,
        );

        assert!(
            sink.record(&observation).is_ok()
        );
    }

    #[test]
    fn telemetry_error_is_separate_from_execution_outcome() {
        let error = TelemetryError::new(
            TelemetryErrorKind::SinkFailure
        );

        assert_eq!(
            error.kind(),
            TelemetryErrorKind::SinkFailure
        );

        let execution_outcome =
            ObservationOutcome::Succeeded;

        assert_eq!(
            execution_outcome,
            ObservationOutcome::Succeeded
        );
    }

    #[test]
    fn telemetry_subjects_are_orderable() {
        let first = TelemetrySubjectRef::new(
            "zhal.hardware",
            "device.a",
        ).unwrap();

        let second = TelemetrySubjectRef::new(
            "zhal.hardware",
            "device.b",
        ).unwrap();

        assert!(first < second);
    }

    #[test]
    fn duration_measurements_preserve_large_values() {
        let value = u128::MAX;

        let measurement = HardwareMeasurement::new(
            name("hardware.execution.duration"),
            MeasurementValue::DurationNanoseconds(value),
            unit("ns"),
        )
        .unwrap();

        assert_eq!(
            measurement.value(),
            MeasurementValue::DurationNanoseconds(value)
        );
    }
}
