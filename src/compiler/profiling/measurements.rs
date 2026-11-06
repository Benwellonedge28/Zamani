
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Profiling Measurements.
//!
//! Path:
//!     src/compiler/profiling/measurements.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines immutable, structured measurements produced by compiler
//! instrumentation and supplied by compatible profiling providers.
//!
//! A measurement represents one observation of a named metric.
//!
//! This module does not execute compiler stages, read hardware
//! performance counters, discover devices, allocate resources,
//! aggregate observations, or change program semantics.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler measurement records.
//! - Compiler measurement values.
//! - Measurement stability classification.
//! - Measurement source identification.
//! - Measurement metadata.
//! - Measurement validation.
//! - Conversion of completed compiler-stage observations into
//!   compiler measurement records.
//!
//! This module does not own:
//!
//! - Generic metric registration.
//! - Metric counters, gauges, and histograms.
//! - Metric aggregation.
//! - Event transport.
//! - Distributed event persistence.
//! - Distributed tracing.
//! - Compiler stage identity allocation.
//! - Compiler stage execution.
//! - Compilation lifecycle.
//! - Hardware performance counters.
//! - Device discovery.
//! - Physical resource allocation.
//! - Quantum scheduling.
//! - Quantum measurement semantics.
//! - Optimization algorithms.
//! - Runtime execution.
//! - Canonical intermediate representations.
//!
//! # Canonical authorities
//!
//! Metric names and units:
//!     crate::distributed::observability::metrics
//!
//! Event timestamps and correlation:
//!     crate::distributed::observability::events
//!
//! Compiler stage identity:
//!     crate::compiler::pipeline::stage
//!
//! Compiler profiling lifecycle observations:
//!     crate::compiler::profiling::events
//!
//! Compiler profiling aggregation:
//!     crate::compiler::profiling::aggregation
//!
//! Optimization telemetry consumers:
//!     crate::compiler::sro
//!
//! Quantum scheduling measurements:
//!     crate::quantum::scheduling::diagnostics::profile
//!
//! # POCO-REAF
//!
//! Measurements are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - accelerator vendor;
//! - operating system;
//! - machine size;
//! - memory capacity;
//! - processor count;
//! - worker count;
//! - device count;
//! - distributed topology;
//! - computational domain.
//!
//! A measurement describes an observation, not a particular
//! physical machine.
//!
//! # Scalability
//!
//! This module imposes no arbitrary universal maximum on:
//!
//! - measurements;
//! - metric names;
//! - metric units;
//! - compiler stages;
//! - compilation units;
//! - compilation sessions;
//! - computational domains;
//! - hardware targets;
//! - devices;
//! - workers;
//! - operations.
//!
//! It does not retain an unbounded history of measurements.
//!
//! Individual records are independently owned values.
//!
//! Storage, retention, buffering, sampling, and throughput policies
//! belong to the measurement consumer or configured provider.
//!
//! The absence of an application-level limit does not imply
//! mathematically infinite physical memory.
//!
//! # Determinism
//!
//! Deterministic measurements and environment-dependent observations
//! are explicitly distinguished.
//!
//! Host elapsed time is an environmental observation.
//!
//! Deterministic operation counts may be used in reproducibility
//! analysis when their inputs and measurement procedure are stable.
//!
//! Environment-dependent observations must not silently become
//! semantic identity or deterministic build-cache inputs.
//!
//! # Security
//!
//! Measurements contain operational metadata.
//!
//! They must not contain:
//!
//! - source-code payloads;
//! - credentials;
//! - private keys;
//! - authentication tokens;
//! - arbitrary execution payloads;
//! - private application data.
//!
//! Measurement producers remain responsible for classifying and
//! redacting information before constructing measurements.
//!
//! # Failure semantics
//!
//! Measurement construction failures are explicit.
//!
//! Measurement failure does not imply compiler-stage failure.
//!
//! Measurement collection failure must not silently change
//! compilation semantics.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use crate::compiler::pipeline::stage::StageId;

use crate::compiler::profiling::events::{
    CompilerProfileEvent,
    StageEventKind,
};

use crate::distributed::observability::events::{
    EventContext,
    EventError,
    EventTimestamp,
    EventValue,
};

use crate::distributed::observability::metrics::{
    MetricName,
    MetricUnit,
};

/// Public API version for compiler profiling measurements.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, IR version,
/// compiler release version, and backend ABI version.
pub const COMPILER_PROFILE_MEASUREMENTS_API_VERSION: u16 = 1;

/// Result returned by compiler profiling measurement operations.
pub type MeasurementResult<T> =
    Result<T, MeasurementError>;

/// Classification of the stability of a measurement.
///
/// This classification is essential for reproducible compilation
/// and for distinguishing semantic information from observations
/// of the environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MeasurementStability {
    /// The observation is deterministic for identical declared
    /// inputs and measurement procedures.
    ///
    /// Examples include deterministic compiler operation counts.
    Deterministic,

    /// The observation depends on the environment in which the
    /// measurement was performed.
    ///
    /// Examples include host elapsed time and observed memory usage.
    EnvironmentDependent,

    /// The observation depends on a particular target realization.
    ///
    /// Examples include target-specific resource utilization
    /// and accelerator performance measurements.
    TargetDependent,

    /// The observation was supplied by an external measurement
    /// provider.
    ///
    /// The provider must establish its own provenance and
    /// reliability guarantees.
    ExternallySupplied,
}

impl MeasurementStability {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deterministic => "deterministic",
            Self::EnvironmentDependent => "environment_dependent",
            Self::TargetDependent => "target_dependent",
            Self::ExternallySupplied => "externally_supplied",
        }
    }

    /// Returns whether this measurement may be considered
    /// deterministic for reproducibility analysis.
    ///
    /// This classification alone does not prove that the
    /// measurement is suitable for semantic identity or caching.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        matches!(self, Self::Deterministic)
    }
}

impl fmt::Display for MeasurementStability {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Open-world identity of the subsystem or provider that produced
/// a measurement.
///
/// This is descriptive provenance metadata.
///
/// It does not identify a physical device or select a backend.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeasurementSource(Arc<str>);

impl MeasurementSource {
    /// Constructs a measurement source identifier.
    ///
    /// Identifiers must be nonempty and must not contain
    /// whitespace or control characters.
    ///
    /// No arbitrary identifier-length limit is imposed.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> MeasurementResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(MeasurementError::InvalidSource);
        }

        if value.chars().any(|character| {
            character.is_whitespace() || character.is_control()
        }) {
            return Err(MeasurementError::InvalidSource);
        }

        Ok(Self(value))
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for MeasurementSource {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Numeric value associated with one profiling observation.
///
/// Unsigned and signed integer values retain their exact
/// representations.
///
/// Floating-point values must be finite.
///
/// Duration observations should normally be represented as
/// unsigned nanoseconds with the corresponding `ns` metric unit.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum MeasurementValue {
    /// Unsigned integer measurement.
    Unsigned(u128),

    /// Signed integer measurement.
    Signed(i128),

    /// Floating-point measurement.
    ///
    /// NaN and infinite values are rejected during measurement
    /// construction.
    Float(f64),
}

impl MeasurementValue {
    /// Constructs a validated floating-point measurement.
    pub fn float(
        value: f64,
    ) -> MeasurementResult<Self> {
        if !value.is_finite() {
            return Err(MeasurementError::NonFiniteValue);
        }

        Ok(Self::Float(value))
    }

    /// Returns whether the measurement value is valid.
    ///
    /// This check is also performed by `Measurement::new`.
    #[must_use]
    pub fn is_valid(self) -> bool {
        match self {
            Self::Unsigned(_) | Self::Signed(_) => true,
            Self::Float(value) => value.is_finite(),
        }
    }

    /// Returns the value as an unsigned integer when applicable.
    #[must_use]
    pub const fn as_unsigned(self) -> Option<u128> {
        match self {
            Self::Unsigned(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the value as a signed integer when applicable.
    #[must_use]
    pub const fn as_signed(self) -> Option<i128> {
        match self {
            Self::Signed(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the value as a floating-point number when applicable.
    ///
    /// This does not perform an implicit conversion from integer
    /// representations.
    #[must_use]
    pub const fn as_float(self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }
}

/// Errors associated with measurement construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MeasurementError {
    /// Measurement source identity is invalid.
    InvalidSource,

    /// Compilation identity is invalid.
    InvalidCompilationIdentity,

    /// Metric value is not finite.
    NonFiniteValue,

    /// Measurement attribute key is invalid.
    InvalidAttributeKey,

    /// An attribute with the same key already exists.
    DuplicateAttribute(String),

    /// A completed stage event has no supplied duration.
    MissingStageDuration,

    /// A stage event was supplied that does not represent
    /// successful stage completion.
    StageNotCompleted,

    /// The system clock is unavailable.
    Clock(EventError),
}

impl fmt::Display for MeasurementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidSource => {
                formatter.write_str(
                    "invalid compiler measurement source",
                )
            }

            Self::InvalidCompilationIdentity => {
                formatter.write_str(
                    "invalid compiler measurement compilation identity",
                )
            }

            Self::NonFiniteValue => {
                formatter.write_str(
                    "compiler measurement value must be finite",
                )
            }

            Self::InvalidAttributeKey => {
                formatter.write_str(
                    "invalid compiler measurement attribute key",
                )
            }

            Self::DuplicateAttribute(key) => {
                write!(
                    formatter,
                    "compiler measurement attribute already exists: {key}"
                )
            }

            Self::MissingStageDuration => {
                formatter.write_str(
                    "completed compiler stage has no recorded duration",
                )
            }

            Self::StageNotCompleted => {
                formatter.write_str(
                    "compiler stage measurement requires a completed stage event",
                )
            }

            Self::Clock(error) => {
                write!(
                    formatter,
                    "compiler measurement clock error: {error}"
                )
            }
        }
    }
}

impl Error for MeasurementError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Clock(error) => Some(error),
            _ => None,
        }
    }
}

/// Immutable, validated compiler profiling measurement.
///
/// A measurement represents one observation.
///
/// It does not own:
///
/// - metric registration;
/// - observation aggregation;
/// - retention;
/// - persistence;
/// - transport;
/// - export;
/// - optimization decisions.
///
/// Its metric name and unit reuse the canonical distributed
/// observability metric contracts.
///
/// Its stage identity reuses the canonical compiler StageId.
///
/// Its correlation metadata reuses distributed EventContext.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    name: MetricName,
    unit: MetricUnit,
    value: MeasurementValue,
    observed_at: EventTimestamp,
    stability: MeasurementStability,
    source: MeasurementSource,

    stage: Option<StageId>,

    compilation_identity: Option<Arc<str>>,

    context: EventContext,

    attributes: BTreeMap<String, EventValue>,
}

impl Measurement {
    /// Constructs a measurement using an explicitly supplied
    /// observation timestamp.
    ///
    /// Explicit timestamps make deterministic tests, replay,
    /// and externally synchronized measurement systems possible.
    pub fn new(
        name: MetricName,
        unit: MetricUnit,
        value: MeasurementValue,
        observed_at: EventTimestamp,
        stability: MeasurementStability,
        source: MeasurementSource,
    ) -> MeasurementResult<Self> {
        if !value.is_valid() {
            return Err(MeasurementError::NonFiniteValue);
        }

        Ok(Self {
            name,
            unit,
            value,
            observed_at,
            stability,
            source,
            stage: None,
            compilation_identity: None,
            context: EventContext::new(),
            attributes: BTreeMap::new(),
        })
    }

    /// Constructs a measurement using the current system wall clock.
    ///
    /// Clock errors are reported explicitly.
    ///
    /// This constructor is intended for real observations.
    /// Use `new` when deterministic timestamps are required.
    pub fn observed(
        name: MetricName,
        unit: MetricUnit,
        value: MeasurementValue,
        stability: MeasurementStability,
        source: MeasurementSource,
    ) -> MeasurementResult<Self> {
        let timestamp = EventTimestamp::now()
            .map_err(MeasurementError::Clock)?;

        Self::new(
            name,
            unit,
            value,
            timestamp,
            stability,
            source,
        )
    }

    /// Constructs a duration measurement from a Rust host duration.
    ///
    /// Duration is represented as an exact unsigned number of
    /// nanoseconds.
    ///
    /// The metric unit is `ns`.
    ///
    /// Host elapsed time must normally be classified as
    /// EnvironmentDependent.
    pub fn duration(
        name: MetricName,
        duration: Duration,
        observed_at: EventTimestamp,
        stability: MeasurementStability,
        source: MeasurementSource,
    ) -> MeasurementResult<Self> {
        let unit = MetricUnit::new("ns")
            .expect("the canonical nanosecond unit is valid");

        Self::new(
            name,
            unit,
            MeasurementValue::Unsigned(duration.as_nanos()),
            observed_at,
            stability,
            source,
        )
    }

    /// Creates a compiler-stage duration measurement from an
    /// existing completed compiler profiling event.
    ///
    /// This method reuses the authoritative event:
    ///
    /// - StageId.
    /// - Completion classification.
    /// - Timestamp.
    /// - Compilation identity.
    /// - Measured duration.
    ///
    /// It does not independently execute or time a compiler stage.
    ///
    /// A missing duration or a non-completed stage is rejected.
    pub fn from_completed_stage(
        event: &CompilerProfileEvent,
    ) -> MeasurementResult<Self> {
        if event.kind() != StageEventKind::Completed {
            return Err(MeasurementError::StageNotCompleted);
        }

        let duration = event
            .duration()
            .ok_or(MeasurementError::MissingStageDuration)?;

        let name = MetricName::new(
            "compiler.stage.duration",
        )
        .expect("the canonical compiler metric name is valid");

        let source = MeasurementSource::new(
            "compiler.pipeline",
        )
        .expect("the canonical compiler source is valid");

        let mut measurement = Self::duration(
            name,
            duration,
            event.timestamp(),
            MeasurementStability::EnvironmentDependent,
            source,
        )?;

        measurement.stage = Some(event.stage().clone());

        if let Some(identity) = event.compilation_identity() {
            measurement = measurement
                .with_compilation_identity(identity)?;
        }

        measurement.context = event.context().clone();

        Ok(measurement)
    }

    /// Associates the canonical compiler pipeline stage identity.
    ///
    /// This does not create a new stage identifier.
    #[must_use]
    pub fn with_stage(
        mut self,
        stage: StageId,
    ) -> Self {
        self.stage = Some(stage);
        self
    }

    /// Associates the logical compilation identity.
    ///
    /// Identity allocation remains the responsibility of the
    /// compilation session/context.
    pub fn with_compilation_identity(
        mut self,
        identity: impl Into<Arc<str>>,
    ) -> MeasurementResult<Self> {
        let identity = identity.into();

        if identity.trim().is_empty()
            || identity.chars().any(char::is_control)
        {
            return Err(
                MeasurementError::InvalidCompilationIdentity,
            );
        }

        self.compilation_identity = Some(identity);

        Ok(self)
    }

    /// Associates distributed correlation and causation metadata.
    ///
    /// The canonical distributed observability subsystem remains
    /// responsible for identity allocation and trace propagation.
    #[must_use]
    pub fn with_context(
        mut self,
        context: EventContext,
    ) -> Self {
        self.context = context;
        self
    }

    /// Inserts a measurement attribute.
    ///
    /// Keys must be nonempty, unambiguous, and free of obvious
    /// secret-bearing names.
    ///
    /// Duplicate keys are rejected rather than silently replaced.
    ///
    /// Values are operational metadata only.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: EventValue,
    ) -> MeasurementResult<Self> {
        let key = key.into();

        validate_attribute_key(&key)?;

        if self.attributes.contains_key(&key) {
            return Err(
                MeasurementError::DuplicateAttribute(key),
            );
        }

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn name(&self) -> &MetricName {
        &self.name
    }

    /// Returns the canonical metric unit.
    #[must_use]
    pub fn unit(&self) -> &MetricUnit {
        &self.unit
    }

    /// Returns the exact numeric observation.
    #[must_use]
    pub const fn value(&self) -> MeasurementValue {
        self.value
    }

    /// Returns the wall-clock observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> EventTimestamp {
        self.observed_at
    }

    /// Returns the measurement stability classification.
    #[must_use]
    pub const fn stability(&self) -> MeasurementStability {
        self.stability
    }

    /// Returns the subsystem or provider that produced the observation.
    #[must_use]
    pub fn source(&self) -> &MeasurementSource {
        &self.source
    }

    /// Returns the associated compiler stage, if available.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the logical compilation identity, if available.
    #[must_use]
    pub fn compilation_identity(&self) -> Option<&str> {
        self.compilation_identity.as_deref()
    }

    /// Returns distributed correlation metadata.
    #[must_use]
    pub fn context(&self) -> &EventContext {
        &self.context
    }

    /// Returns deterministically ordered measurement attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, EventValue> {
        &self.attributes
    }

    /// Returns whether the measurement is classified as deterministic.
    ///
    /// This classification is informational.
    /// Consumers must still verify that the measurement procedure
    /// and its inputs are suitable for their intended use.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        self.stability.is_deterministic()
    }
}

/// Validates a measurement attribute key.
///
/// No arbitrary maximum key length is imposed.
///
/// Accepted characters:
///
/// - ASCII letters;
/// - ASCII digits;
/// - underscore;
/// - hyphen;
/// - dot.
///
/// Empty dotted components are rejected.
fn validate_attribute_key(
    key: &str,
) -> MeasurementResult<()> {
    if key.is_empty()
        || key.trim() != key
        || key.split('.').any(str::is_empty)
        || !key.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, '_' | '-' | '.')
        })
    {
        return Err(
            MeasurementError::InvalidAttributeKey,
        );
    }

    let normalized = key.to_ascii_lowercase();

    const SENSITIVE_PARTS: &[&str] = &[
        "password",
        "passwd",
        "secret",
        "credential",
        "authorization",
        "access_token",
        "private_key",
        "payload",
    ];

    if SENSITIVE_PARTS
        .iter()
        .any(|part| normalized.contains(part))
    {
        return Err(
            MeasurementError::InvalidAttributeKey,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::profiling::events::{
        CompilerProfileEvent,
        StageEventKind,
    };

    fn timestamp() -> EventTimestamp {
        EventTimestamp::new(
            1_800_000_000,
            123_456_789,
        )
        .expect("valid test timestamp")
    }

    fn stage() -> StageId {
        StageId::new(
            "zamani.compiler.optimization",
        )
        .expect("valid test stage")
    }

    fn source() -> MeasurementSource {
        MeasurementSource::new(
            "compiler.pipeline",
        )
        .expect("valid test source")
    }

    fn metric_name() -> MetricName {
        MetricName::new(
            "compiler.operations",
        )
        .expect("valid test metric name")
    }

    fn unit() -> MetricUnit {
        MetricUnit::new(
            "operations",
        )
        .expect("valid test metric unit")
    }

    fn measurement() -> Measurement {
        Measurement::new(
            metric_name(),
            unit(),
            MeasurementValue::Unsigned(42),
            timestamp(),
            MeasurementStability::Deterministic,
            source(),
        )
        .expect("valid test measurement")
    }

    #[test]
    fn constructs_valid_measurement() {
        let value = measurement();

        assert_eq!(
            value.name().as_str(),
            "compiler.operations"
        );

        assert_eq!(
            value.unit().as_str(),
            "operations"
        );

        assert_eq!(
            value.value().as_unsigned(),
            Some(42)
        );

        assert!(value.is_deterministic());
    }

    #[test]
    fn preserves_signed_integer_precision() {
        let value = Measurement::new(
            metric_name(),
            unit(),
            MeasurementValue::Signed(i128::MIN),
            timestamp(),
            MeasurementStability::Deterministic,
            source(),
        )
        .expect("valid signed measurement");

        assert_eq!(
            value.value().as_signed(),
            Some(i128::MIN)
        );
    }

    #[test]
    fn preserves_unsigned_integer_precision() {
        let value = Measurement::new(
            metric_name(),
            unit(),
            MeasurementValue::Unsigned(u128::MAX),
            timestamp(),
            MeasurementStability::Deterministic,
            source(),
        )
        .expect("valid unsigned measurement");

        assert_eq!(
            value.value().as_unsigned(),
            Some(u128::MAX)
        );
    }

    #[test]
    fn rejects_nan_measurements() {
        assert_eq!(
            MeasurementValue::float(f64::NAN),
            Err(MeasurementError::NonFiniteValue)
        );
    }

    #[test]
    fn rejects_infinite_measurements() {
        assert_eq!(
            MeasurementValue::float(f64::INFINITY),
            Err(MeasurementError::NonFiniteValue)
        );
    }

    #[test]
    fn accepts_finite_float_measurements() {
        let value = MeasurementValue::float(12.75)
            .expect("finite float");

        assert_eq!(
            value.as_float(),
            Some(12.75)
        );
    }

    #[test]
    fn rejects_non_finite_float_in_constructor() {
        let result = Measurement::new(
            metric_name(),
            unit(),
            MeasurementValue::Float(f64::NAN),
            timestamp(),
            MeasurementStability::Deterministic,
            source(),
        );

        assert_eq!(
            result,
            Err(MeasurementError::NonFiniteValue)
        );
    }

    #[test]
    fn rejects_empty_source() {
        assert_eq!(
            MeasurementSource::new(""),
            Err(MeasurementError::InvalidSource)
        );
    }

    #[test]
    fn rejects_source_control_characters() {
        assert_eq!(
            MeasurementSource::new("compiler\npipeline"),
            Err(MeasurementError::InvalidSource)
        );
    }

    #[test]
    fn preserves_open_world_source_names() {
        let value = MeasurementSource::new(
            "future.photonic.profiler",
        )
        .expect("valid extension source");

        assert_eq!(
            value.as_str(),
            "future.photonic.profiler"
        );
    }

    #[test]
    fn associates_compiler_stage() {
        let value = measurement()
            .with_stage(stage());

        assert_eq!(
            value.stage().map(StageId::as_str),
            Some("zamani.compiler.optimization")
        );
    }

    #[test]
    fn validates_compilation_identity() {
        let result = measurement()
            .with_compilation_identity("compilation-001")
            .expect("valid compilation identity");

        assert_eq!(
            result.compilation_identity(),
            Some("compilation-001")
        );

        assert_eq!(
            measurement().with_compilation_identity(" "),
            Err(MeasurementError::InvalidCompilationIdentity)
        );
    }

    #[test]
    fn preserves_explicit_timestamp() {
        assert_eq!(
            measurement().observed_at(),
            timestamp()
        );
    }

    #[test]
    fn classifies_environment_dependent_observations() {
        let value = Measurement::duration(
            metric_name(),
            Duration::from_millis(10),
            timestamp(),
            MeasurementStability::EnvironmentDependent,
            source(),
        )
        .expect("valid duration");

        assert!(!value.is_deterministic());

        assert_eq!(
            value.value().as_unsigned(),
            Some(10_000_000)
        );

        assert_eq!(
            value.unit().as_str(),
            "ns"
        );
    }

    #[test]
    fn rejects_invalid_measurement_attribute() {
        assert_eq!(
            measurement().with_attribute(
                "private_key",
                EventValue::Boolean(true)
            ),
            Err(MeasurementError::InvalidAttributeKey)
        );
    }

    #[test]
    fn rejects_duplicate_measurement_attribute() {
        let first = measurement()
            .with_attribute(
                "compiler.optimization.enabled",
                EventValue::Boolean(true),
            )
            .expect("first attribute");

        assert_eq!(
            first.with_attribute(
                "compiler.optimization.enabled",
                EventValue::Boolean(false),
            ),
            Err(MeasurementError::DuplicateAttribute(
                "compiler.optimization.enabled".to_owned()
            ))
        );
    }

    #[test]
    fn accepts_extensible_measurement_attributes() {
        let value = measurement()
            .with_attribute(
                "future.photonic.operation_count",
                EventValue::Unsigned(100),
            )
            .expect("valid extension attribute");

        assert_eq!(
            value.attributes().len(),
            1
        );
    }

    #[test]
    fn converts_completed_stage_event_to_measurement() {
        let event = CompilerProfileEvent::new(
            stage(),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_compilation_identity("compilation-001")
        .expect("valid compilation identity")
        .with_duration(Duration::from_millis(5));

        let measurement =
            Measurement::from_completed_stage(&event)
                .expect("valid completed stage measurement");

        assert_eq!(
            measurement.name().as_str(),
            "compiler.stage.duration"
        );

        assert_eq!(
            measurement.unit().as_str(),
            "ns"
        );

        assert_eq!(
            measurement.value().as_unsigned(),
            Some(5_000_000)
        );

        assert_eq!(
            measurement.stage().map(StageId::as_str),
            Some("zamani.compiler.optimization")
        );

        assert_eq!(
            measurement.compilation_identity(),
            Some("compilation-001")
        );

        assert_eq!(
            measurement.stability(),
            MeasurementStability::EnvironmentDependent
        );
    }

    #[test]
    fn rejects_stage_start_measurement() {
        let event = CompilerProfileEvent::new(
            stage(),
            StageEventKind::Started,
            timestamp(),
        );

        assert_eq!(
            Measurement::from_completed_stage(&event),
            Err(MeasurementError::StageNotCompleted)
        );
    }

    #[test]
    fn rejects_completed_stage_without_duration() {
        let event = CompilerProfileEvent::new(
            stage(),
            StageEventKind::Completed,
            timestamp(),
        );

        assert_eq!(
            Measurement::from_completed_stage(&event),
            Err(MeasurementError::MissingStageDuration)
        );
    }

    #[test]
    fn measurement_records_are_independently_owned() {
        let first = measurement();
        let second = first.clone();

        assert_eq!(first, second);
    }

    #[test]
    fn future_measurement_stability_is_not_hardcoded() {
        let source = MeasurementSource::new(
            "future.hardware.telemetry",
        )
        .expect("valid source");

        let value = Measurement::new(
            metric_name(),
            unit(),
            MeasurementValue::Unsigned(100),
            timestamp(),
            MeasurementStability::ExternallySupplied,
            source,
        )
        .expect("valid external measurement");

        assert_eq!(
            value.stability(),
            MeasurementStability::ExternallySupplied
        );
    }
}
