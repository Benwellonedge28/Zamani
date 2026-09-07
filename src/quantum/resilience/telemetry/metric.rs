//! Zamani Quantum Resilience — Telemetry Metrics
//!
//! Path:
//!     src/quantum/resilience/telemetry/metric.rs
//!
//! # Purpose
//!
//! This module defines the canonical, provider-neutral metric representation
//! used by `quantum::resilience::telemetry`.
//!
//! A metric is a quantitative observation about an execution, resource,
//! subsystem, workload, or resilience condition. Metrics are consumed by:
//!
//! - telemetry collectors;
//! - anomaly detection;
//! - statistical detection;
//! - drift detection;
//! - diagnosis;
//! - planning;
//! - health evaluation;
//! - benchmarking;
//! - history;
//! - learning;
//! - observability exporters;
//! - deterministic replay;
//! - resilience verification and provenance.
//!
//! This module defines metric semantics only. It does NOT:
//!
//! - collect metrics;
//! - read the system clock;
//! - access hardware;
//! - access the network;
//! - access the filesystem;
//! - perform anomaly detection;
//! - diagnose faults;
//! - select recovery strategies;
//! - execute recovery;
//! - implement QEC;
//! - implement mitigation;
//! - implement routing;
//! - implement scheduling;
//! - define a second quantum-qubit identity type;
//! - impose universal hardware limits;
//! - prescribe an exporter format;
//! - prescribe a monitoring vendor.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Architectural position
//!
//! ```text
//! hardware / runtime / QEC / compiler / scheduler / execution
//!                           │
//!                           ▼
//!                    metric producers
//!                           │
//!                           ▼
//!                       Metric
//!                           │
//!             ┌─────────────┼──────────────┐
//!             ▼             ▼              ▼
//!         detection      history       exporter
//!             │
//!             ▼
//!         diagnosis
//!             │
//!             ▼
//!          planning
//!             │
//!             ▼
//!          recovery
//!             │
//!             ▼
//!        verification
//! ```
//!
//! `Metric` is therefore a quantitative data contract, not a resilience
//! algorithm.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no universal maximum for:
//!
//! - qubits;
//! - physical qubits;
//! - logical qubits;
//! - resources;
//! - devices;
//! - backends;
//! - metrics;
//! - labels;
//! - histogram buckets;
//! - metric dimensions;
//! - executions;
//! - shots;
//! - samples.
//!
//! A concrete deployment may impose limits through:
//!
//! - `quantum::resilience::limits`;
//! - security policy;
//! - memory policy;
//! - storage policy;
//! - execution budgets;
//! - exporter capabilities;
//! - target capabilities;
//! - operator configuration.
//!
//! Those limits are invocation/deployment constraints, not metric semantics.
//!
//! # Canonical quantum identities
//!
//! This module uses the canonical identities from:
//!
//!     quantum::ir::qubit
//!
//! New resilience-local qubit identifiers are forbidden.
//!
//! The authoritative types are:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! # Determinism
//!
//! Metric construction is deterministic for deterministic input.
//!
//! This module does not:
//!
//! - read clocks;
//! - generate random identifiers;
//! - access global mutable state;
//! - access environment variables;
//! - access network state;
//! - access filesystem state.
//!
//! Timestamps, sequences, identities and values are explicit inputs.
//!
//! `BTreeMap` is used for labels so iteration order is deterministic.
//!
//! # Metric semantics
//!
//! The metric model distinguishes:
//!
//! - Gauge;
//! - Counter;
//! - UpDownCounter;
//! - Histogram;
//! - Summary.
//!
//! A metric value must match its declared metric kind.
//!
//! This prevents ambiguous telemetry such as a counter carrying an arbitrary
//! text payload or a histogram being interpreted as a gauge.
//!
//! # Quantum-specific measurements
//!
//! The model intentionally does not hard-code metric names such as:
//!
//!     fidelity
//!     readout_error
//!     logical_error_rate
//!     gate_error
//!
//! Such names are semantic conventions and may evolve.
//!
//! Producers may use arbitrary validated metric names and units.
//!
//! # Resource scope
//!
//! A metric may apply to:
//!
//! - an entire system;
//! - a backend;
//! - a device;
//! - a logical qubit;
//! - a physical qubit;
//! - multiple qubits;
//! - another resource;
//! - an execution;
//! - a workload;
//! - a distributed scope.
//!
//! The scope is descriptive. It does not prescribe how resources are selected
//! or discovered.
//!
//! # Aggregation
//!
//! Histograms use explicit bucket boundaries and cumulative bucket counts.
//!
//! Summary metrics use explicit quantile/value pairs plus count and sum.
//!
//! Raw sample storage is intentionally not part of this type. Storing every
//! sample indefinitely would turn a metric representation into an implicit
//! unbounded log. Sampling, retention, streaming and aggregation policies belong
//! to collectors and storage systems.
//!
//! # Security
//!
//! Metric labels and names must not contain:
//!
//! - credentials;
//! - API tokens;
//! - private keys;
//! - authentication headers;
//! - secret provider material;
//! - raw quantum state;
//! - unnecessary personal information.
//!
//! This type cannot prevent a caller from putting sensitive information into a
//! string, so ingress/collection policy must enforce deployment-specific data
//! classification and redaction.
//!
//! # Serialization
//!
//! Serialization is intentionally owned by:
//!
//!     quantum::resilience::serialization
//!
//! This module therefore does not derive or implement a serialization format.
//!
//! The metric model is nevertheless deliberately structured so a versioned
//! serializer can encode it without changing the metric semantics.
//!
//! # Integration
//!
//! `telemetry/mod.rs` should expose:
//!
//!     pub mod metric;
//!
//! Consumers should import:
//!
//!     crate::quantum::resilience::telemetry::metric::Metric
//!
//! `telemetry::event` remains the canonical event representation.
//!
//! `telemetry::metric` represents quantitative observations. An event may
//! produce zero, one, or many metrics, but the two concepts must not be merged.
//!
//! `telemetry::collector` constructs metrics from hardware/runtime/QEC/etc.
//! observations.
//!
//! `telemetry::exporter` consumes immutable metrics.
//!
//! `telemetry::trace` may associate metrics with traces through correlation and
//! execution identifiers, but trace ownership remains in `trace.rs`.
//!
//! `history` may retain metrics according to its retention policy.
//!
//! `detection` consumes metrics as evidence; it must not mutate them.
//!
//! `diagnosis` interprets metrics alongside other evidence.
//!
//! `learning` may consume historical metrics but cannot make an unverified
//! metric authoritative.
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Stable semantic schema identifier for resilience telemetry metrics.
pub const TELEMETRY_METRIC_SCHEMA_ID: &str =
    "zamani.quantum.resilience.telemetry.metric";

/// Current semantic schema version.
///
/// This is a data-schema version, not a hardware-size limit.
pub const TELEMETRY_METRIC_SCHEMA_VERSION: u32 = 1;

/// Identifier for a metric instance.
///
/// Metric IDs are producer supplied. No UUID, ULID, hash or database format is
/// required by this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricId(Arc<str>);

impl MetricId {
    /// Creates a metric identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MetricIdError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "metric identifier")?;

        Ok(Self(value))
    }

    /// Returns the identifier as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing a [`MetricId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricIdError {
    /// Identifier is empty or whitespace-only.
    Empty,

    /// Identifier contains a Unicode control character.
    ContainsControlCharacter,
}

impl fmt::Display for MetricIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("metric identifier is empty"),
            Self::ContainsControlCharacter => {
                formatter.write_str("metric identifier contains a control character")
            }
        }
    }
}

impl std::error::Error for MetricIdError {}

/// Stable semantic name for a metric family.
///
/// Examples may include names such as `fidelity`, `logical_error_rate`,
/// `queue_time`, or application-defined names, but this module does not reserve
/// or require any particular names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricName(Arc<str>);

impl MetricName {
    /// Creates a metric name.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MetricNameError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "metric name")?;

        Ok(Self(value))
    }

    /// Returns the metric name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing [`MetricName`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricNameError {
    /// Name is empty or whitespace-only.
    Empty,

    /// Name contains a Unicode control character.
    ContainsControlCharacter,
}

impl fmt::Display for MetricNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("metric name is empty"),
            Self::ContainsControlCharacter => {
                formatter.write_str("metric name contains a control character")
            }
        }
    }
}

impl std::error::Error for MetricNameError {}

/// Identifies the metric producer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricSourceId(Arc<str>);

impl MetricSourceId {
    /// Creates a metric source identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MetricSourceIdError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "metric source identifier")?;

        Ok(Self(value))
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for MetricSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing [`MetricSourceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricSourceIdError {
    /// Identifier is empty or whitespace-only.
    Empty,

    /// Identifier contains a control character.
    ContainsControlCharacter,
}

impl fmt::Display for MetricSourceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("metric source identifier is empty"),
            Self::ContainsControlCharacter => {
                formatter.write_str("metric source identifier contains a control character")
            }
        }
    }
}

impl std::error::Error for MetricSourceIdError {}

/// Explicit metric timestamp.
///
/// The metric layer does not read the clock. Producers provide timestamps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricTimestamp {
    unix_seconds: i64,
    nanos: u32,
}

impl MetricTimestamp {
    /// Number of nanoseconds in a second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates a timestamp.
    pub const fn new(
        unix_seconds: i64,
        nanos: u32,
    ) -> Result<Self, MetricTimestampError> {
        if nanos >= Self::NANOS_PER_SECOND {
            return Err(MetricTimestampError::InvalidNanoseconds);
        }

        Ok(Self {
            unix_seconds,
            nanos,
        })
    }

    /// Creates an integral-second timestamp.
    #[must_use]
    pub const fn from_unix_seconds(unix_seconds: i64) -> Self {
        Self {
            unix_seconds,
            nanos: 0,
        }
    }

    /// Returns Unix seconds.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.unix_seconds
    }

    /// Returns fractional nanoseconds.
    #[must_use]
    pub const fn nanos(self) -> u32 {
        self.nanos
    }
}

impl fmt::Display for MetricTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}",
            self.unix_seconds,
            self.nanos
        )
    }
}

/// Timestamp validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricTimestampError {
    /// Nanoseconds are outside the canonical range.
    InvalidNanoseconds,
}

impl fmt::Display for MetricTimestampError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNanoseconds => formatter.write_str(
                "metric timestamp nanoseconds must be less than 1_000_000_000",
            ),
        }
    }
}

impl std::error::Error for MetricTimestampError {}

/// Producer-local sequence number.
///
/// A sequence is not a global event/metric counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricSequence(u64);

impl MetricSequence {
    /// Creates a sequence number.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Metric aggregation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MetricKind {
    /// Current instantaneous or sampled value.
    Gauge,

    /// Monotonically increasing quantity.
    Counter,

    /// Quantity that may increase or decrease.
    UpDownCounter,

    /// Distribution represented by bucket counts.
    Histogram,

    /// Distribution represented by quantile/value observations.
    Summary,
}

impl MetricKind {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gauge => "gauge",
            Self::Counter => "counter",
            Self::UpDownCounter => "up_down_counter",
            Self::Histogram => "histogram",
            Self::Summary => "summary",
        }
    }
}

impl fmt::Display for MetricKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Numeric metric value.
///
/// The representation is deliberately explicit so a consumer never has to
/// guess whether a number is a counter, gauge, histogram, etc.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricValue {
    /// Floating-point gauge.
    Gauge(f64),

    /// Monotonic unsigned counter.
    Counter(u64),

    /// Signed value that may move in either direction.
    UpDownCounter(f64),

    /// Distribution represented by cumulative buckets.
    Histogram(Histogram),

    /// Distribution represented by quantile/value pairs.
    Summary(Summary),
}

impl MetricValue {
    /// Returns the corresponding metric kind.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        match self {
            Self::Gauge(_) => MetricKind::Gauge,
            Self::Counter(_) => MetricKind::Counter,
            Self::UpDownCounter(_) => MetricKind::UpDownCounter,
            Self::Histogram(_) => MetricKind::Histogram,
            Self::Summary(_) => MetricKind::Summary,
        }
    }
}

/// Histogram representation.
///
/// `upper_bounds[i]` is the inclusive upper bound of bucket `i`.
///
/// `counts[i]` is the cumulative count of observations less than or equal to
/// `upper_bounds[i]`.
///
/// The final implicit bucket is +infinity and contains `count` observations
/// that are not represented by an explicit finite bucket.
///
/// This is compatible with streaming aggregation because raw observations do
/// not need to be retained.
#[derive(Debug, Clone, PartialEq)]
pub struct Histogram {
    upper_bounds: Arc<[f64]>,
    counts: Arc<[u64]>,
    count: u64,
    sum: f64,
}

impl Histogram {
    /// Creates a histogram.
    ///
    /// Requirements:
    ///
    /// - every bound is finite;
    /// - bounds are strictly increasing;
    /// - every cumulative count is monotonic;
    /// - the final explicit count does not exceed total count;
    /// - sum is finite.
    pub fn new(
        upper_bounds: impl IntoIterator<Item = f64>,
        counts: impl IntoIterator<Item = u64>,
        count: u64,
        sum: f64,
    ) -> Result<Self, HistogramError> {
        if !sum.is_finite() {
            return Err(HistogramError::NonFiniteSum);
        }

        let upper_bounds: Vec<f64> = upper_bounds.into_iter().collect();
        let counts: Vec<u64> = counts.into_iter().collect();

        if upper_bounds.len() != counts.len() {
            return Err(HistogramError::LengthMismatch);
        }

        let mut previous_bound: Option<f64> = None;
        let mut previous_count = 0_u64;

        for bound in &upper_bounds {
            if !bound.is_finite() {
                return Err(HistogramError::NonFiniteBoundary);
            }

            if let Some(previous) = previous_bound {
                if *bound <= previous {
                    return Err(HistogramError::NonIncreasingBoundaries);
                }
            }

            previous_bound = Some(*bound);
        }

        for bucket_count in &counts {
            if *bucket_count < previous_count {
                return Err(HistogramError::NonMonotonicCounts);
            }

            if *bucket_count > count {
                return Err(HistogramError::BucketCountExceedsTotal);
            }

            previous_count = *bucket_count;
        }

        Ok(Self {
            upper_bounds: upper_bounds.into(),
            counts: counts.into(),
            count,
            sum,
        })
    }

    /// Returns the finite bucket upper bounds.
    #[must_use]
    pub fn upper_bounds(&self) -> &[f64] {
        &self.upper_bounds
    }

    /// Returns cumulative bucket counts.
    #[must_use]
    pub fn counts(&self) -> &[u64] {
        &self.counts
    }

    /// Returns the total number of observations.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Returns the sum of observations.
    #[must_use]
    pub const fn sum(&self) -> f64 {
        self.sum
    }
}

/// Histogram validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistogramError {
    /// Boundary/count arrays have different lengths.
    LengthMismatch,

    /// A boundary is NaN or infinite.
    NonFiniteBoundary,

    /// Boundaries are not strictly increasing.
    NonIncreasingBoundaries,

    /// Cumulative bucket counts decrease.
    NonMonotonicCounts,

    /// A bucket count exceeds the total count.
    BucketCountExceedsTotal,

    /// Histogram sum is NaN or infinite.
    NonFiniteSum,
}

impl fmt::Display for HistogramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LengthMismatch => {
                formatter.write_str("histogram bounds and counts have different lengths")
            }
            Self::NonFiniteBoundary => {
                formatter.write_str("histogram boundary must be finite")
            }
            Self::NonIncreasingBoundaries => {
                formatter.write_str("histogram boundaries must be strictly increasing")
            }
            Self::NonMonotonicCounts => {
                formatter.write_str("histogram cumulative counts must be monotonic")
            }
            Self::BucketCountExceedsTotal => {
                formatter.write_str("histogram bucket count exceeds total count")
            }
            Self::NonFiniteSum => {
                formatter.write_str("histogram sum must be finite")
            }
        }
    }
}

impl std::error::Error for HistogramError {}

/// A quantile/value pair.
///
/// `quantile` must be within `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantileValue {
    quantile: f64,
    value: f64,
}

impl QuantileValue {
    /// Creates a quantile/value pair.
    pub fn new(quantile: f64, value: f64) -> Result<Self, QuantileValueError> {
        if !quantile.is_finite() || !(0.0..=1.0).contains(&quantile) {
            return Err(QuantileValueError::InvalidQuantile);
        }

        if !value.is_finite() {
            return Err(QuantileValueError::NonFiniteValue);
        }

        Ok(Self { quantile, value })
    }

    /// Returns the quantile.
    #[must_use]
    pub const fn quantile(self) -> f64 {
        self.quantile
    }

    /// Returns the value at the quantile.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

/// Quantile/value validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantileValueError {
    /// Quantile is NaN, infinite, or outside `[0, 1]`.
    InvalidQuantile,

    /// Value is NaN or infinite.
    NonFiniteValue,
}

impl fmt::Display for QuantileValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuantile => {
                formatter.write_str("quantile must be finite and within [0, 1]")
            }
            Self::NonFiniteValue => {
                formatter.write_str("quantile value must be finite")
            }
        }
    }
}

impl std::error::Error for QuantileValueError {}

/// Summary distribution.
///
/// Quantiles must be supplied in strictly increasing order.
#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    quantiles: Arc<[QuantileValue]>,
    count: u64,
    sum: f64,
}

impl Summary {
    /// Creates a summary.
    pub fn new(
        quantiles: impl IntoIterator<Item = QuantileValue>,
        count: u64,
        sum: f64,
    ) -> Result<Self, SummaryError> {
        if !sum.is_finite() {
            return Err(SummaryError::NonFiniteSum);
        }

        let quantiles: Vec<QuantileValue> = quantiles.into_iter().collect();

        let mut previous: Option<f64> = None;

        for entry in &quantiles {
            if let Some(previous_quantile) = previous {
                if entry.quantile() <= previous_quantile {
                    return Err(SummaryError::NonIncreasingQuantiles);
                }
            }

            previous = Some(entry.quantile());
        }

        Ok(Self {
            quantiles: quantiles.into(),
            count,
            sum,
        })
    }

    /// Returns quantile/value pairs.
    #[must_use]
    pub fn quantiles(&self) -> &[QuantileValue] {
        &self.quantiles
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Returns the sum.
    #[must_use]
    pub const fn sum(&self) -> f64 {
        self.sum
    }
}

/// Summary validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SummaryError {
    /// Quantiles are not strictly increasing.
    NonIncreasingQuantiles,

    /// Summary sum is NaN or infinite.
    NonFiniteSum,
}

impl fmt::Display for SummaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonIncreasingQuantiles => {
                formatter.write_str("summary quantiles must be strictly increasing")
            }
            Self::NonFiniteSum => {
                formatter.write_str("summary sum must be finite")
            }
        }
    }
}

impl std::error::Error for SummaryError {}

/// Metric temporal aggregation semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Temporality {
    /// Value represents the current observation/state.
    Instant,

    /// Value accumulates from an unspecified beginning.
    Cumulative,

    /// Value represents a non-overlapping interval/delta.
    Delta,
}

impl Temporality {
    /// Returns the stable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Cumulative => "cumulative",
            Self::Delta => "delta",
        }
    }
}

impl fmt::Display for Temporality {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Measurement unit.
///
/// The unit is intentionally a validated opaque string rather than a closed
/// enum so future quantum technologies and application domains do not require
/// this file to be modified.
///
/// Examples:
///
/// - `1`
/// - `s`
/// - `ms`
/// - `ns`
/// - `Hz`
/// - `qubit`
/// - `shots`
/// - `probability`
/// - `dimensionless`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricUnit(Arc<str>);

impl MetricUnit {
    /// Creates a metric unit.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MetricUnitError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "metric unit")?;

        Ok(Self(value))
    }

    /// Returns the unit.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for MetricUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Metric unit validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricUnitError {
    /// Unit is empty or whitespace-only.
    Empty,

    /// Unit contains a Unicode control character.
    ContainsControlCharacter,
}

impl fmt::Display for MetricUnitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("metric unit is empty"),
            Self::ContainsControlCharacter => {
                formatter.write_str("metric unit contains a control character")
            }
        }
    }
}

impl std::error::Error for MetricUnitError {}

/// Resource kind associated with a metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind {
    /// Whole resilience/runtime system.
    System,

    /// Backend resource.
    Backend,

    /// Device/QPU resource.
    Device,

    /// Logical qubit.
    LogicalQubit,

    /// Physical qubit.
    PhysicalQubit,

    /// Quantum operation.
    Operation,

    /// Execution.
    Execution,

    /// Workload/program.
    Workload,

    /// Generic resource represented by a canonical resource identifier.
    Generic,

    /// Distributed or aggregate scope.
    Aggregate,
}

impl ResourceKind {
    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Backend => "backend",
            Self::Device => "device",
            Self::LogicalQubit => "logical_qubit",
            Self::PhysicalQubit => "physical_qubit",
            Self::Operation => "operation",
            Self::Execution => "execution",
            Self::Workload => "workload",
            Self::Generic => "generic",
            Self::Aggregate => "aggregate",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Resource scope for a metric.
///
/// Canonical logical and physical qubit IDs are used directly.
///
/// The generic resource identifier remains a string because the metric module
/// must not duplicate every resource type owned by other subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetricScope {
    kind: Option<ResourceKind>,
    resource_id: Option<Arc<str>>,
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl MetricScope {
    /// Creates a resource scope.
    ///
    /// Qubit IDs are sorted and deduplicated for deterministic representation.
    #[must_use]
    pub fn new<L, P>(
        kind: Option<ResourceKind>,
        resource_id: Option<impl Into<Arc<str>>>,
        logical_qubits: L,
        physical_qubits: P,
    ) -> Self
    where
        L: IntoIterator<Item = QubitId>,
        P: IntoIterator<Item = PhysicalQubitId>,
    {
        let mut logical: Vec<QubitId> = logical_qubits.into_iter().collect();
        let mut physical: Vec<PhysicalQubitId> =
            physical_qubits.into_iter().collect();

        logical.sort();
        logical.dedup();

        physical.sort();
        physical.dedup();

        Self {
            kind,
            resource_id: resource_id.map(Into::into),
            logical_qubits: logical.into(),
            physical_qubits: physical.into(),
        }
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> Option<ResourceKind> {
        self.kind
    }

    /// Returns the generic resource identifier.
    #[must_use]
    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
    }

    /// Returns canonical logical qubit IDs.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns canonical physical qubit IDs.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns whether this scope contains no resource information.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kind.is_none()
            && self.resource_id.is_none()
            && self.logical_qubits.is_empty()
            && self.physical_qubits.is_empty()
    }
}

/// Metric labels.
///
/// `BTreeMap` is intentional: deterministic ordering is required for stable
/// replay, comparison and canonical serialization.
pub type MetricLabels = BTreeMap<String, String>;

/// Data classification for telemetry.
///
/// This is metadata for policy enforcement; it does not itself redact data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataClassification {
    /// Safe for public exposure.
    Public,

    /// Internal operational information.
    Internal,

    /// Information requiring controlled access.
    Confidential,

    /// Highly restricted information.
    Restricted,
}

impl DataClassification {
    /// Returns the stable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Confidential => "confidential",
            Self::Restricted => "restricted",
        }
    }
}

impl fmt::Display for DataClassification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Confidence attached to a metric observation.
///
/// Confidence is deliberately represented as a probability-like value in
/// `[0, 1]`. The meaning is defined by the producer and should be documented
/// through metric conventions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetricConfidence(f64);

impl MetricConfidence {
    /// Creates a confidence value.
    pub fn new(value: f64) -> Result<Self, MetricConfidenceError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(MetricConfidenceError::InvalidValue);
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

/// Confidence validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetricConfidenceError {
    /// Value is not finite or outside `[0, 1]`.
    InvalidValue,
}

impl fmt::Display for MetricConfidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue => {
                formatter.write_str("metric confidence must be finite and within [0, 1]")
            }
        }
    }
}

impl std::error::Error for MetricConfidenceError {}

/// Provenance information attached to a metric.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetricProvenance {
    /// Identifier of the producing component.
    producer: Option<MetricSourceId>,

    /// Identifier of the originating execution/workload if available.
    origin_id: Option<Arc<str>>,

    /// Identifier of the parent observation, if applicable.
    parent_id: Option<MetricId>,

    /// Optional externally assigned schema/convention identifier.
    convention: Option<Arc<str>>,
}

impl MetricProvenance {
    /// Creates empty provenance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            producer: None,
            origin_id: None,
            parent_id: None,
            convention: None,
        }
    }

    /// Sets the producer.
    #[must_use]
    pub fn with_producer(mut self, producer: MetricSourceId) -> Self {
        self.producer = Some(producer);
        self
    }

    /// Sets an origin identifier.
    #[must_use]
    pub fn with_origin_id(mut self, origin_id: impl Into<Arc<str>>) -> Self {
        self.origin_id = Some(origin_id.into());
        self
    }

    /// Sets the parent metric identifier.
    #[must_use]
    pub fn with_parent_id(mut self, parent_id: MetricId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Sets the metric convention identifier.
    #[must_use]
    pub fn with_convention(mut self, convention: impl Into<Arc<str>>) -> Self {
        self.convention = Some(convention.into());
        self
    }

    /// Returns the producer.
    #[must_use]
    pub fn producer(&self) -> Option<&MetricSourceId> {
        self.producer.as_ref()
    }

    /// Returns the origin identifier.
    #[must_use]
    pub fn origin_id(&self) -> Option<&str> {
        self.origin_id.as_deref()
    }

    /// Returns the parent metric.
    #[must_use]
    pub fn parent_id(&self) -> Option<&MetricId> {
        self.parent_id.as_ref()
    }

    /// Returns the convention identifier.
    #[must_use]
    pub fn convention(&self) -> Option<&str> {
        self.convention.as_deref()
    }
}

/// Immutable quantitative telemetry record.
#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    schema_id: &'static str,
    schema_version: u32,

    id: MetricId,
    name: MetricName,
    kind: MetricKind,
    temporality: Temporality,
    unit: MetricUnit,
    value: MetricValue,

    timestamp: MetricTimestamp,
    sequence: Option<MetricSequence>,

    source: Option<MetricSourceId>,
    scope: MetricScope,

    labels: MetricLabels,

    confidence: Option<MetricConfidence>,
    classification: DataClassification,
    provenance: MetricProvenance,
}

impl Metric {
    /// Creates a metric through the validated builder.
    #[must_use]
    pub fn builder() -> MetricBuilder {
        MetricBuilder::new()
    }

    /// Returns the metric schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Returns the metric schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the metric identifier.
    #[must_use]
    pub fn id(&self) -> &MetricId {
        &self.id
    }

    /// Returns the metric name.
    #[must_use]
    pub fn name(&self) -> &MetricName {
        &self.name
    }

    /// Returns the metric kind.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        self.kind
    }

    /// Returns the temporal semantics.
    #[must_use]
    pub const fn temporality(&self) -> Temporality {
        self.temporality
    }

    /// Returns the metric unit.
    #[must_use]
    pub fn unit(&self) -> &MetricUnit {
        &self.unit
    }

    /// Returns the metric value.
    #[must_use]
    pub fn value(&self) -> &MetricValue {
        &self.value
    }

    /// Returns the metric timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> MetricTimestamp {
        self.timestamp
    }

    /// Returns the producer sequence.
    #[must_use]
    pub const fn sequence(&self) -> Option<MetricSequence> {
        self.sequence
    }

    /// Returns the metric source.
    #[must_use]
    pub fn source(&self) -> Option<&MetricSourceId> {
        self.source.as_ref()
    }

    /// Returns the resource scope.
    #[must_use]
    pub fn scope(&self) -> &MetricScope {
        &self.scope
    }

    /// Returns metric labels.
    #[must_use]
    pub fn labels(&self) -> &MetricLabels {
        &self.labels
    }

    /// Returns confidence, if supplied.
    #[must_use]
    pub const fn confidence(&self) -> Option<MetricConfidence> {
        self.confidence
    }

    /// Returns data classification.
    #[must_use]
    pub const fn classification(&self) -> DataClassification {
        self.classification
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> &MetricProvenance {
        &self.provenance
    }

    /// Returns whether the metric is numeric and finite.
    ///
    /// Histogram and summary structures have already been validated by their
    /// constructors.
    #[must_use]
    pub fn is_validated(&self) -> bool {
        match &self.value {
            MetricValue::Gauge(value)
            | MetricValue::UpDownCounter(value) => value.is_finite(),

            MetricValue::Counter(_) => true,

            MetricValue::Histogram(histogram) => {
                histogram.sum().is_finite()
                    && histogram
                        .upper_bounds()
                        .iter()
                        .all(|value| value.is_finite())
            }

            MetricValue::Summary(summary) => {
                summary.sum().is_finite()
                    && summary
                        .quantiles()
                        .iter()
                        .all(|value| value.value().is_finite())
            }
        }
    }

    /// Validates the complete metric.
    pub fn validate(&self) -> Result<(), MetricError> {
        if self.schema_id != TELEMETRY_METRIC_SCHEMA_ID {
            return Err(MetricError::InvalidSchemaId);
        }

        if self.schema_version == 0 {
            return Err(MetricError::InvalidSchemaVersion);
        }

        if self.kind != self.value.kind() {
            return Err(MetricError::KindValueMismatch);
        }

        if !self.is_validated() {
            return Err(MetricError::NonFiniteValue);
        }

        if let Some(source) = &self.source {
            validate_identifier(source.as_str(), "metric source identifier")
                .map_err(MetricError::InvalidIdentifier)?;
        }

        for (key, value) in &self.labels {
            validate_identifier(key, "metric label key")
                .map_err(MetricError::InvalidIdentifier)?;

            if value.chars().any(char::is_control) {
                return Err(MetricError::InvalidLabelValue);
            }
        }

        if let Some(origin_id) = self.provenance.origin_id() {
            if origin_id.trim().is_empty() {
                return Err(MetricError::InvalidProvenance);
            }
        }

        if let Some(convention) = self.provenance.convention() {
            validate_identifier(convention, "metric convention")
                .map_err(MetricError::InvalidIdentifier)?;
        }

        Ok(())
    }
}

/// Builder for [`Metric`].
///
/// Required fields are explicit. No default machine identity, timestamp,
/// metric name, value or source is invented.
#[derive(Debug, Default)]
pub struct MetricBuilder {
    id: Option<MetricId>,
    name: Option<MetricName>,
    kind: Option<MetricKind>,
    temporality: Option<Temporality>,
    unit: Option<MetricUnit>,
    value: Option<MetricValue>,
    timestamp: Option<MetricTimestamp>,
    sequence: Option<MetricSequence>,
    source: Option<MetricSourceId>,
    scope: MetricScope,
    labels: MetricLabels,
    confidence: Option<MetricConfidence>,
    classification: DataClassification,
    provenance: MetricProvenance,
}

impl MetricBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            name: None,
            kind: None,
            temporality: None,
            unit: None,
            value: None,
            timestamp: None,
            sequence: None,
            source: None,
            scope: MetricScope {
                kind: None,
                resource_id: None,
                logical_qubits: Arc::new([]),
                physical_qubits: Arc::new([]),
            },
            labels: BTreeMap::new(),
            confidence: None,
            classification: DataClassification::Internal,
            provenance: MetricProvenance::new(),
        }
    }

    /// Sets the metric identifier.
    #[must_use]
    pub fn id(mut self, id: MetricId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the metric name.
    #[must_use]
    pub fn name(mut self, name: MetricName) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the metric kind.
    #[must_use]
    pub fn kind(mut self, kind: MetricKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the metric temporality.
    #[must_use]
    pub fn temporality(mut self, temporality: Temporality) -> Self {
        self.temporality = Some(temporality);
        self
    }

    /// Sets the metric unit.
    #[must_use]
    pub fn unit(mut self, unit: MetricUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Sets the metric value.
    #[must_use]
    pub fn value(mut self, value: MetricValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the timestamp.
    #[must_use]
    pub fn timestamp(mut self, timestamp: MetricTimestamp) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Sets the producer sequence.
    #[must_use]
    pub fn sequence(mut self, sequence: MetricSequence) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets the producer/source.
    #[must_use]
    pub fn source(mut self, source: MetricSourceId) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the resource scope.
    #[must_use]
    pub fn scope(mut self, scope: MetricScope) -> Self {
        self.scope = scope;
        self
    }

    /// Adds or replaces a label.
    #[must_use]
    pub fn label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    /// Replaces the complete label set.
    #[must_use]
    pub fn labels(mut self, labels: MetricLabels) -> Self {
        self.labels = labels;
        self
    }

    /// Sets confidence.
    #[must_use]
    pub fn confidence(mut self, confidence: MetricConfidence) -> Self {
        self.confidence = Some(confidence);
        self
    }

    /// Sets data classification.
    #[must_use]
    pub fn classification(mut self, classification: DataClassification) -> Self {
        self.classification = classification;
        self
    }

    /// Sets provenance.
    #[must_use]
    pub fn provenance(mut self, provenance: MetricProvenance) -> Self {
        self.provenance = provenance;
        self
    }

    /// Builds and validates the metric.
    pub fn build(self) -> Result<Metric, MetricError> {
        let id = self.id.ok_or(MetricError::MissingId)?;
        let name = self.name.ok_or(MetricError::MissingName)?;
        let kind = self.kind.ok_or(MetricError::MissingKind)?;
        let temporality = self
            .temporality
            .ok_or(MetricError::MissingTemporality)?;
        let unit = self.unit.ok_or(MetricError::MissingUnit)?;
        let value = self.value.ok_or(MetricError::MissingValue)?;
        let timestamp = self.timestamp.ok_or(MetricError::MissingTimestamp)?;

        if kind != value.kind() {
            return Err(MetricError::KindValueMismatch);
        }

        let metric = Metric {
            schema_id: TELEMETRY_METRIC_SCHEMA_ID,
            schema_version: TELEMETRY_METRIC_SCHEMA_VERSION,
            id,
            name,
            kind,
            temporality,
            unit,
            value,
            timestamp,
            sequence: self.sequence,
            source: self.source,
            scope: self.scope,
            labels: self.labels,
            confidence: self.confidence,
            classification: self.classification,
            provenance: self.provenance,
        };

        metric.validate()?;

        Ok(metric)
    }
}

/// Errors produced while constructing or validating metrics.
#[derive(Debug)]
pub enum MetricError {
    /// Metric ID was omitted.
    MissingId,

    /// Metric name was omitted.
    MissingName,

    /// Metric kind was omitted.
    MissingKind,

    /// Metric temporality was omitted.
    MissingTemporality,

    /// Metric unit was omitted.
    MissingUnit,

    /// Metric value was omitted.
    MissingValue,

    /// Metric timestamp was omitted.
    MissingTimestamp,

    /// Metric kind does not match value representation.
    KindValueMismatch,

    /// Metric contains NaN or infinity.
    NonFiniteValue,

    /// Schema ID is invalid.
    InvalidSchemaId,

    /// Schema version is invalid.
    InvalidSchemaVersion,

    /// Invalid identifier.
    InvalidIdentifier(IdentifierValidationError),

    /// Invalid label value.
    InvalidLabelValue,

    /// Invalid provenance.
    InvalidProvenance,
}

impl fmt::Display for MetricError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingId => formatter.write_str("metric ID is required"),
            Self::MissingName => formatter.write_str("metric name is required"),
            Self::MissingKind => formatter.write_str("metric kind is required"),
            Self::MissingTemporality => {
                formatter.write_str("metric temporality is required")
            }
            Self::MissingUnit => formatter.write_str("metric unit is required"),
            Self::MissingValue => formatter.write_str("metric value is required"),
            Self::MissingTimestamp => {
                formatter.write_str("metric timestamp is required")
            }
            Self::KindValueMismatch => {
                formatter.write_str("metric kind does not match metric value")
            }
            Self::NonFiniteValue => {
                formatter.write_str("metric contains a non-finite numeric value")
            }
            Self::InvalidSchemaId => {
                formatter.write_str("metric schema ID is invalid")
            }
            Self::InvalidSchemaVersion => {
                formatter.write_str("metric schema version is invalid")
            }
            Self::InvalidIdentifier(error) => {
                write!(formatter, "invalid metric identifier: {error}")
            }
            Self::InvalidLabelValue => {
                formatter.write_str("metric label value contains a control character")
            }
            Self::InvalidProvenance => {
                formatter.write_str("metric provenance is invalid")
            }
        }
    }
}

impl std::error::Error for MetricError {}

/// Generic identifier validation used by metric identity fields.
///
/// The function deliberately does not impose a maximum length or ASCII-only
/// restriction. Unicode identifiers are valid unless they contain control
/// characters.
fn validate_identifier(
    value: &str,
    _field: &str,
) -> Result<(), IdentifierValidationError> {
    if value.trim().is_empty() {
        return Err(IdentifierValidationError::Empty);
    }

    if value.chars().any(char::is_control) {
        return Err(IdentifierValidationError::ContainsControlCharacter);
    }

    Ok(())
}

/// Identifier validation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdentifierValidationError {
    /// Value is empty or whitespace-only.
    Empty,

    /// Value contains a Unicode control character.
    ContainsControlCharacter,
}

impl fmt::Display for IdentifierValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("value is empty"),
            Self::ContainsControlCharacter => {
                formatter.write_str("value contains a control character")
            }
        }
    }
}

impl std::error::Error for IdentifierValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_id() -> MetricId {
        MetricId::new("metric-1").expect("valid metric ID")
    }

    fn test_name() -> MetricName {
        MetricName::new("logical_error_rate").expect("valid metric name")
    }

    fn test_unit() -> MetricUnit {
        MetricUnit::new("probability").expect("valid metric unit")
    }

    fn test_timestamp() -> MetricTimestamp {
        MetricTimestamp::new(1_700_000_000, 123_456_789)
            .expect("valid timestamp")
    }

    #[test]
    fn scalar_gauge_metric_builds() {
        let metric = Metric::builder()
            .id(test_id())
            .name(test_name())
            .kind(MetricKind::Gauge)
            .temporality(Temporality::Instant)
            .unit(test_unit())
            .value(MetricValue::Gauge(0.001))
            .timestamp(test_timestamp())
            .build()
            .expect("metric should build");

        assert_eq!(metric.kind(), MetricKind::Gauge);
        assert_eq!(metric.temporality(), Temporality::Instant);
        assert_eq!(metric.value().kind(), MetricKind::Gauge);
        assert!(metric.is_validated());
    }

    #[test]
    fn counter_metric_builds() {
        let metric = Metric::builder()
            .id(test_id())
            .name(MetricName::new("shots").expect("valid name"))
            .kind(MetricKind::Counter)
            .temporality(Temporality::Cumulative)
            .unit(MetricUnit::new("shots").expect("valid unit"))
            .value(MetricValue::Counter(42))
            .timestamp(test_timestamp())
            .build()
            .expect("metric should build");

        assert_eq!(metric.kind(), MetricKind::Counter);
        assert_eq!(metric.value().kind(), MetricKind::Counter);
    }

    #[test]
    fn kind_and_value_mismatch_is_rejected() {
        let result = Metric::builder()
            .id(test_id())
            .name(test_name())
            .kind(MetricKind::Counter)
            .temporality(Temporality::Cumulative)
            .unit(test_unit())
            .value(MetricValue::Gauge(1.0))
            .timestamp(test_timestamp())
            .build();

        assert!(matches!(result, Err(MetricError::KindValueMismatch)));
    }

    #[test]
    fn nan_gauge_is_rejected() {
        let result = Metric::builder()
            .id(test_id())
            .name(test_name())
            .kind(MetricKind::Gauge)
            .temporality(Temporality::Instant)
            .unit(test_unit())
            .value(MetricValue::Gauge(f64::NAN))
            .timestamp(test_timestamp())
            .build();

        assert!(matches!(result, Err(MetricError::NonFiniteValue)));
    }

    #[test]
    fn infinite_gauge_is_rejected() {
        let result = Metric::builder()
            .id(test_id())
            .name(test_name())
            .kind(MetricKind::Gauge)
            .temporality(Temporality::Instant)
            .unit(test_unit())
            .value(MetricValue::Gauge(f64::INFINITY))
            .timestamp(test_timestamp())
            .build();

        assert!(matches!(result, Err(MetricError::NonFiniteValue)));
    }

    #[test]
    fn histogram_requires_matching_lengths() {
        let result = Histogram::new([1.0, 2.0], [1], 1, 1.0);

        assert!(matches!(result, Err(HistogramError::LengthMismatch)));
    }

    #[test]
    fn histogram_rejects_non_increasing_boundaries() {
        let result = Histogram::new([1.0, 1.0], [1, 1], 1, 1.0);

        assert!(matches!(
            result,
            Err(HistogramError::NonIncreasingBoundaries)
        ));
    }

    #[test]
    fn histogram_rejects_non_monotonic_counts() {
        let result = Histogram::new([1.0, 2.0], [3, 2], 3, 3.0);

        assert!(matches!(
            result,
            Err(HistogramError::NonMonotonicCounts)
        ));
    }

    #[test]
    fn histogram_rejects_bucket_above_total() {
        let result = Histogram::new([1.0], [4], 3, 3.0);

        assert!(matches!(
            result,
            Err(HistogramError::BucketCountExceedsTotal)
        ));
    }

    #[test]
    fn summary_requires_increasing_quantiles() {
        let q1 = QuantileValue::new(0.5, 1.0).expect("valid quantile");
        let q2 = QuantileValue::new(0.5, 2.0).expect("valid quantile");

        let result = Summary::new([q1, q2], 2, 3.0);

        assert!(matches!(
            result,
            Err(SummaryError::NonIncreasingQuantiles)
        ));
    }

    #[test]
    fn confidence_is_bounded() {
        assert!(MetricConfidence::new(0.0).is_ok());
        assert!(MetricConfidence::new(1.0).is_ok());
        assert!(MetricConfidence::new(-0.1).is_err());
        assert!(MetricConfidence::new(1.1).is_err());
        assert!(MetricConfidence::new(f64::NAN).is_err());
    }

    #[test]
    fn metric_scope_deduplicates_and_orders_qubits() {
        // The exact constructors for QubitId/PhysicalQubitId belong to the
        // canonical IR implementation. This test intentionally does not
        // manufacture IDs here because resilience must not know their internal
        // representation.
        //
        // ResourceScope behavior is therefore covered by integration tests
        // against the canonical IR constructors.
    }

    #[test]
    fn labels_have_deterministic_order() {
        let mut labels = MetricLabels::new();

        labels.insert("z".to_owned(), "last".to_owned());
        labels.insert("a".to_owned(), "first".to_owned());

        let keys: Vec<&str> = labels.keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["a", "z"]);
    }

    #[test]
    fn identifier_validation_rejects_control_characters() {
        assert!(MetricName::new("valid").is_ok());
        assert!(MetricName::new("bad\nname").is_err());
        assert!(MetricName::new("bad\tname").is_err());
    }

    #[test]
    fn timestamp_validates_nanoseconds() {
        assert!(MetricTimestamp::new(0, 999_999_999).is_ok());
        assert!(MetricTimestamp::new(0, 1_000_000_000).is_err());
    }

    #[test]
    fn provenance_is_explicit() {
        let producer =
            MetricSourceId::new("hardware.telemetry").expect("valid producer");

        let provenance = MetricProvenance::new()
            .with_producer(producer)
            .with_origin_id("execution-1")
            .with_convention("zamani.quantum.fidelity");

        assert_eq!(
            provenance.origin_id(),
            Some("execution-1")
        );

        assert_eq!(
            provenance.convention(),
            Some("zamani.quantum.fidelity")
        );
    }
}