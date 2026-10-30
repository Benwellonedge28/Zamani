
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Distributed metrics collection for Zamani.
//!
//! # Ownership
//!
//! This module owns:
//! - generic distributed counters;
//! - generic distributed gauges;
//! - generic distributed histograms;
//! - metric names, units, and validated labels;
//! - thread-safe in-process metric registration and updates;
//! - configurable metric-series budgets;
//! - deterministic snapshots for downstream exporters.
//!
//! It does not own:
//! - source-level observability syntax or semantic validation;
//! - distributed execution lifecycle state;
//! - distributed event definitions;
//! - tracing spans or trace propagation;
//! - quantum decoder or QPU-specific metrics;
//! - resource admission or scheduling decisions;
//! - persistence, network transport, or exporter protocols.
//!
//! # Integration
//!
//! - `observability/events.rs` may translate structured events into metric
//!   updates where an event has a defined metric interpretation.
//! - `observability/tracing.rs` owns trace/span correlation. Trace IDs must
//!   not be used as metric labels because that creates unbounded cardinality.
//! - `execution/lifecycle.rs` remains the authoritative lifecycle source.
//!   Consumers may translate drained lifecycle events into counters and
//!   duration observations.
//! - `membership/health.rs` remains the authoritative membership-health
//!   source. Consumers may translate health transitions into metric updates.
//! - `resilience/` remains authoritative for retry, checkpoint, recovery,
//!   and idempotency behavior.
//! - `quantum/error_correction/metrics.rs` retains quantum-specific metric
//!   ownership. This module may aggregate generic distributed outcomes but
//!   must not duplicate quantum decoder measurements.
//! - Future exporters consume `MetricSnapshot`; they do not change collection
//!   semantics or introduce a second registry.
//!
//! # Scalability
//!
//! No fixed limit is imposed on metric names, labels, metric series, or
//! observations. Actual memory and processing requirements are constrained
//! by available resources.
//!
//! A deployment may configure a maximum number of registered series through
//! `MetricsConfig`. This is an operational budget, not a language-level or
//! semantic limit. When that budget is exhausted, registration fails
//! explicitly; existing observations are not silently discarded.
//!
//! Histograms use caller-supplied finite bucket boundaries. No fixed
//! resolution, bucket count, or duration range is imposed by this module.
//!
//! # Concurrency
//!
//! The registry uses a standard-library mutex to keep registration, updates,
//! and snapshots consistent. It is suitable as a portable baseline and for
//! low-to-moderate contention. High-throughput deployments should implement
//! a compatible sharded or lock-free collector behind a future collection
//! interface, after benchmarking the actual workload.
//!
//! # Security and privacy
//!
//! Metric labels are metadata, not a place for credentials, private payloads,
//! user identifiers, trace IDs, or arbitrary high-cardinality data.
//! Obvious secret-bearing label names are rejected. Callers remain
//! responsible for classifying and sanitizing label values.
//!
//! No metric value is automatically exported or logged.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{Mutex, PoisonError};

/// Result type for metrics operations.
pub type MetricsResult<T> = Result<T, MetricsError>;

/// Metric kinds supported by the distributed registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MetricKind {
    /// A monotonically increasing unsigned counter.
    Counter,

    /// A value that may increase or decrease.
    Gauge,

    /// A distribution of observed non-negative values.
    Histogram,
}

impl MetricKind {
    /// Returns the stable textual metric-kind identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
            Self::Histogram => "histogram",
        }
    }
}

/// Errors produced by the metrics subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MetricsError {
    /// A metric name is empty or malformed.
    InvalidMetricName,

    /// A metric unit is empty or malformed.
    InvalidUnit,

    /// A label name is empty or malformed.
    InvalidLabelName,

    /// A label name appears to contain sensitive information.
    SensitiveLabelName,

    /// A label value is empty.
    EmptyLabelValue,

    /// A metric was registered with an incompatible kind or unit.
    ConflictingDescriptor {
        /// Metric name.
        name: String,
    },

    /// A histogram boundary is invalid.
    InvalidHistogramBoundaries,

    /// A histogram observation is negative or not finite.
    InvalidHistogramObservation,

    /// A gauge value is not finite.
    NonFiniteGauge,

    /// A counter would overflow its representation.
    CounterOverflow {
        /// Metric name.
        name: String,
    },

    /// The configured metric-series budget has been exhausted.
    SeriesBudgetExceeded {
        /// Configured maximum number of registered series.
        maximum: usize,
    },

    /// The registry mutex was poisoned.
    RegistryPoisoned,

    /// A snapshot or update encountered an invalid internal invariant.
    InternalInvariantViolation,
}

impl fmt::Display for MetricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetricName => {
                f.write_str("invalid metric name")
            }
            Self::InvalidUnit => {
                f.write_str("invalid metric unit")
            }
            Self::InvalidLabelName => {
                f.write_str("invalid metric label name")
            }
            Self::SensitiveLabelName => {
                f.write_str("sensitive metric label names are not permitted")
            }
            Self::EmptyLabelValue => {
                f.write_str("metric label value must not be empty")
            }
            Self::ConflictingDescriptor { name } => {
                write!(f, "conflicting metric descriptor: {name}")
            }
            Self::InvalidHistogramBoundaries => {
                f.write_str("histogram boundaries must be finite and strictly increasing")
            }
            Self::InvalidHistogramObservation => {
                f.write_str("histogram observation must be finite and non-negative")
            }
            Self::NonFiniteGauge => {
                f.write_str("gauge value must be finite")
            }
            Self::CounterOverflow { name } => {
                write!(f, "metric counter overflow: {name}")
            }
            Self::SeriesBudgetExceeded { maximum } => {
                write!(f, "metric series budget exceeded: {maximum}")
            }
            Self::RegistryPoisoned => {
                f.write_str("metrics registry synchronization was poisoned")
            }
            Self::InternalInvariantViolation => {
                f.write_str("metrics registry invariant violated")
            }
        }
    }
}

impl std::error::Error for MetricsError {}

impl<T> From<PoisonError<T>> for MetricsError {
    fn from(_: PoisonError<T>) -> Self {
        Self::RegistryPoisoned
    }
}

/// Configuration for a distributed metrics registry.
///
/// `max_series` is an optional operational budget. `None` means that the
/// registry does not impose a configured series-count budget. Available
/// memory and the deployment's resource policy still apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MetricsConfig {
    max_series: Option<NonZeroUsize>,
}

impl MetricsConfig {
    /// Creates a configuration without a registry-level series budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_series: None }
    }

    /// Creates a configuration with an explicit series budget.
    #[must_use]
    pub const fn with_max_series(maximum: NonZeroUsize) -> Self {
        Self {
            max_series: Some(maximum),
        }
    }

    /// Returns the configured series budget.
    #[must_use]
    pub const fn max_series(self) -> Option<NonZeroUsize> {
        self.max_series
    }
}

/// A validated metric name.
///
/// Names are portable identifiers. They do not encode a backend, vendor,
/// machine, network endpoint, or physical device.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricName(String);

impl MetricName {
    /// Creates a metric name.
    ///
    /// Accepted characters are ASCII letters, digits, underscore, dot,
    /// and hyphen. The first character must be a letter or underscore.
    pub fn new(value: impl Into<String>) -> MetricsResult<Self> {
        let value = value.into();

        let mut chars = value.chars();

        let Some(first) = chars.next() else {
            return Err(MetricsError::InvalidMetricName);
        };

        if !(first.is_ascii_alphabetic() || first == '_')
            || !chars.all(|ch| {
                ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '-')
            })
        {
            return Err(MetricsError::InvalidMetricName);
        }

        Ok(Self(value))
    }

    /// Returns the metric name as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A validated unit associated with a metric.
///
/// Units are descriptive metadata. Conversion and dimensional analysis
/// remain responsibilities of the caller and semantic layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricUnit(String);

impl MetricUnit {
    /// Creates a unit identifier.
    pub fn new(value: impl Into<String>) -> MetricsResult<Self> {
        let value = value.into();

        if value.is_empty()
            || !value
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '/' | '-'))
        {
            return Err(MetricsError::InvalidUnit);
        }

        Ok(Self(value))
    }

    /// Returns the unit identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A validated metric label name.
///
/// Labels should describe bounded dimensions such as operation category,
/// outcome, or execution domain. Do not use task IDs, participant IDs,
/// trace IDs, timestamps, or arbitrary user data as labels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LabelName(String);

impl LabelName {
    /// Creates a label name and rejects obvious secret-bearing names.
    pub fn new(value: impl Into<String>) -> MetricsResult<Self> {
        let value = value.into();

        let mut chars = value.chars();

        let Some(first) = chars.next() else {
            return Err(MetricsError::InvalidLabelName);
        };

        if !(first.is_ascii_alphabetic() || first == '_')
            || !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(MetricsError::InvalidLabelName);
        }

        let normalized = value.to_ascii_lowercase();

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
            return Err(MetricsError::SensitiveLabelName);
        }

        Ok(Self(value))
    }

    /// Returns the label name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A validated metric label value.
///
/// Values are intentionally opaque. Callers must not put secrets, private
/// payloads, personal data, or unbounded identifiers in label values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LabelValue(String);

impl LabelValue {
    /// Creates a non-empty label value.
    pub fn new(value: impl Into<String>) -> MetricsResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(MetricsError::EmptyLabelValue);
        }

        Ok(Self(value))
    }

    /// Returns the label value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A deterministic set of metric labels.
pub type MetricLabels = BTreeMap<LabelName, LabelValue>;

/// A metric descriptor.
///
/// A descriptor's name, kind, and unit must remain consistent across all
/// series registered under that name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricDescriptor {
    name: MetricName,
    kind: MetricKind,
    unit: MetricUnit,
    description: String,
}

impl MetricDescriptor {
    /// Creates a descriptor.
    pub fn new(
        name: MetricName,
        kind: MetricKind,
        unit: MetricUnit,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name,
            kind,
            unit,
            description: description.into(),
        }
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

    /// Returns the unit.
    #[must_use]
    pub fn unit(&self) -> &MetricUnit {
        &self.unit
    }

    /// Returns the human-readable description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A registered metric series identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SeriesKey {
    name: MetricName,
    labels: MetricLabels,
}

/// Internal storage for a metric series.
#[derive(Debug, Clone, PartialEq)]
enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram {
        boundaries: Vec<f64>,
        bucket_counts: Vec<u64>,
        count: u64,
        sum: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
}

/// One immutable metric-series snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSeriesSnapshot {
    /// Metric descriptor.
    pub descriptor: MetricDescriptor,

    /// Labels identifying this series.
    pub labels: MetricLabels,

    /// Counter, gauge, or histogram snapshot.
    pub value: MetricSnapshotValue,
}

/// Immutable value of a metric series.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricSnapshotValue {
    /// Counter total.
    Counter {
        /// Current counter value.
        value: u64,
    },

    /// Current gauge value.
    Gauge {
        /// Current gauge value.
        value: f64,
    },

    /// Histogram aggregate.
    Histogram {
        /// Finite bucket boundaries, in ascending order.
        boundaries: Vec<f64>,

        /// Per-bucket counts. The final entry is the implicit +infinity
        /// bucket. Counts are non-cumulative.
        bucket_counts: Vec<u64>,

        /// Total number of observations.
        count: u64,

        /// Sum of all observations.
        sum: f64,

        /// Smallest observed value, if any.
        minimum: Option<f64>,

        /// Largest observed value, if any.
        maximum: Option<f64>,
    },
}

/// A consistent, deterministic registry snapshot.
///
/// The snapshot is an in-memory value. Persistence, transport, and external
/// schema compatibility are owned by downstream adapters.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSnapshot {
    /// All registered metric series in deterministic order.
    pub series: Vec<MetricSeriesSnapshot>,
}

/// Mutable registry state.
#[derive(Debug, Default)]
struct RegistryState {
    descriptors: BTreeMap<MetricName, MetricDescriptor>,
    values: BTreeMap<SeriesKey, MetricValue>,
}

/// Thread-safe in-process distributed metrics registry.
///
/// Clones should share this registry through `Arc<MetricsRegistry>`.
/// The registry does not create a process-global singleton.
#[derive(Debug)]
pub struct MetricsRegistry {
    config: MetricsConfig,
    state: Mutex<RegistryState>,
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new(MetricsConfig::default())
    }
}

impl MetricsRegistry {
    /// Creates a registry with the supplied operational configuration.
    #[must_use]
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            state: Mutex::new(RegistryState::default()),
        }
    }

    /// Returns the registry configuration.
    #[must_use]
    pub const fn config(&self) -> MetricsConfig {
        self.config
    }

    /// Registers or verifies a metric descriptor and its label series.
    ///
    /// Re-registering an identical descriptor and series is idempotent.
    /// Registering an incompatible descriptor under the same name fails.
    pub fn register(
        &self,
        descriptor: MetricDescriptor,
        labels: MetricLabels,
    ) -> MetricsResult<()> {
        let key = SeriesKey {
            name: descriptor.name.clone(),
            labels,
        };

        let mut state = self.state.lock()?;

        if let Some(existing) = state.descriptors.get(&descriptor.name) {
            if existing.kind != descriptor.kind || existing.unit != descriptor.unit {
                return Err(MetricsError::ConflictingDescriptor {
                    name: descriptor.name.to_string(),
                });
            }
        }

        if state.values.contains_key(&key) {
            return Ok(());
        }

        if let Some(maximum) = self.config.max_series {
            if state.values.len() >= maximum.get() {
                return Err(MetricsError::SeriesBudgetExceeded {
                    maximum: maximum.get(),
                });
            }
        }

        state
            .descriptors
            .entry(descriptor.name.clone())
            .or_insert_with(|| descriptor.clone());

        let value = match descriptor.kind {
            MetricKind::Counter => MetricValue::Counter(0),
            MetricKind::Gauge => MetricValue::Gauge(0.0),
            MetricKind::Histogram => {
                return Err(MetricsError::InvalidHistogramBoundaries);
            }
        };

        state.values.insert(key, value);

        Ok(())
    }

    /// Registers a histogram series with caller-defined finite boundaries.
    ///
    /// The boundaries must be strictly increasing. The implementation adds
    /// an implicit final +infinity bucket.
    pub fn register_histogram(
        &self,
        descriptor: MetricDescriptor,
        labels: MetricLabels,
        boundaries: Vec<f64>,
    ) -> MetricsResult<()> {
        if descriptor.kind != MetricKind::Histogram {
            return Err(MetricsError::ConflictingDescriptor {
                name: descriptor.name.to_string(),
            });
        }

        validate_boundaries(&boundaries)?;

        let key = SeriesKey {
            name: descriptor.name.clone(),
            labels,
        };

        let mut state = self.state.lock()?;

        if let Some(existing) = state.descriptors.get(&descriptor.name) {
            if existing.kind != descriptor.kind || existing.unit != descriptor.unit {
                return Err(MetricsError::ConflictingDescriptor {
                    name: descriptor.name.to_string(),
                });
            }
        }

        if let Some(existing) = state.values.get(&key) {
            match existing {
                MetricValue::Histogram {
                    boundaries: existing_boundaries,
                    ..
                } if existing_boundaries == &boundaries => return Ok(()),
                _ => {
                    return Err(MetricsError::ConflictingDescriptor {
                        name: descriptor.name.to_string(),
                    });
                }
            }
        }

        if let Some(maximum) = self.config.max_series {
            if state.values.len() >= maximum.get() {
                return Err(MetricsError::SeriesBudgetExceeded {
                    maximum: maximum.get(),
                });
            }
        }

        state
            .descriptors
            .entry(descriptor.name.clone())
            .or_insert_with(|| descriptor.clone());

        let bucket_count = boundaries
            .len()
            .checked_add(1)
            .ok_or(MetricsError::InternalInvariantViolation)?;

        state.values.insert(
            key,
            MetricValue::Histogram {
                boundaries,
                bucket_counts: vec![0; bucket_count],
                count: 0,
                sum: 0.0,
                minimum: None,
                maximum: None,
            },
        );

        Ok(())
    }

    /// Adds to a registered counter.
    pub fn increment(
        &self,
        name: &MetricName,
        labels: &MetricLabels,
        amount: u64,
    ) -> MetricsResult<u64> {
        let key = SeriesKey {
            name: name.clone(),
            labels: labels.clone(),
        };

        let mut state = self.state.lock()?;

        let value = state
            .values
            .get_mut(&key)
            .ok_or(MetricsError::InternalInvariantViolation)?;

        match value {
            MetricValue::Counter(current) => {
                let next = current.checked_add(amount).ok_or_else(|| {
                    MetricsError::CounterOverflow {
                        name: name.to_string(),
                    }
                })?;

                *current = next;
                Ok(next)
            }
            _ => Err(MetricsError::ConflictingDescriptor {
                name: name.to_string(),
            }),
        }
    }

    /// Sets a registered gauge.
    pub fn set_gauge(
        &self,
        name: &MetricName,
        labels: &MetricLabels,
        value: f64,
    ) -> MetricsResult<()> {
        if !value.is_finite() {
            return Err(MetricsError::NonFiniteGauge);
        }

        let key = SeriesKey {
            name: name.clone(),
            labels: labels.clone(),
        };

        let mut state = self.state.lock()?;

        let metric = state
            .values
            .get_mut(&key)
            .ok_or(MetricsError::InternalInvariantViolation)?;

        match metric {
            MetricValue::Gauge(current) => {
                *current = value;
                Ok(())
            }
            _ => Err(MetricsError::ConflictingDescriptor {
                name: name.to_string(),
            }),
        }
    }

    /// Records a non-negative finite histogram observation.
    pub fn observe(
        &self,
        name: &MetricName,
        labels: &MetricLabels,
        observation: f64,
    ) -> MetricsResult<()> {
        if !observation.is_finite() || observation < 0.0 {
            return Err(MetricsError::InvalidHistogramObservation);
        }

        let key = SeriesKey {
            name: name.clone(),
            labels: labels.clone(),
        };

        let mut state = self.state.lock()?;

        let metric = state
            .values
            .get_mut(&key)
            .ok_or(MetricsError::InternalInvariantViolation)?;

        match metric {
            MetricValue::Histogram {
                boundaries,
                bucket_counts,
                count,
                sum,
                minimum,
                maximum,
            } => {
                let next_count = count.checked_add(1).ok_or(
                    MetricsError::InternalInvariantViolation,
                )?;

                let next_sum = *sum + observation;

                if !next_sum.is_finite() {
                    return Err(MetricsError::InvalidHistogramObservation);
                }

                let bucket_index = boundaries.partition_point(|bound| observation > *bound);

                let bucket = bucket_counts
                    .get_mut(bucket_index)
                    .ok_or(MetricsError::InternalInvariantViolation)?;

                let next_bucket = bucket.checked_add(1).ok_or(
                    MetricsError::InternalInvariantViolation,
                )?;

                *bucket = next_bucket;
                *count = next_count;
                *sum = next_sum;

                *minimum = Some(
                    minimum.map_or(observation, |current| current.min(observation)),
                );

                *maximum = Some(
                    maximum.map_or(observation, |current| current.max(observation)),
                );

                Ok(())
            }
            _ => Err(MetricsError::ConflictingDescriptor {
                name: name.to_string(),
            }),
        }
    }

    /// Returns a deterministic snapshot of all registered metric series.
    pub fn snapshot(&self) -> MetricsResult<MetricSnapshot> {
        let state = self.state.lock()?;

        let mut series = Vec::with_capacity(state.values.len());

        for (key, value) in &state.values {
            let descriptor = state
                .descriptors
                .get(&key.name)
                .ok_or(MetricsError::InternalInvariantViolation)?
                .clone();

            let snapshot_value = match value {
                MetricValue::Counter(value) => {
                    MetricSnapshotValue::Counter { value: *value }
                }
                MetricValue::Gauge(value) => {
                    MetricSnapshotValue::Gauge { value: *value }
                }
                MetricValue::Histogram {
                    boundaries,
                    bucket_counts,
                    count,
                    sum,
                    minimum,
                    maximum,
                } => MetricSnapshotValue::Histogram {
                    boundaries: boundaries.clone(),
                    bucket_counts: bucket_counts.clone(),
                    count: *count,
                    sum: *sum,
                    minimum: *minimum,
                    maximum: *maximum,
                },
            };

            series.push(MetricSeriesSnapshot {
                descriptor,
                labels: key.labels.clone(),
                value: snapshot_value,
            });
        }

        Ok(MetricSnapshot { series })
    }

    /// Returns the number of registered metric series.
    pub fn series_count(&self) -> MetricsResult<usize> {
        Ok(self.state.lock()?.values.len())
    }
}

fn validate_boundaries(boundaries: &[f64]) -> MetricsResult<()> {
    let mut previous = None;

    for boundary in boundaries {
        if !boundary.is_finite()
            || previous.is_some_and(|value| *boundary <= value)
        {
            return Err(MetricsError::InvalidHistogramBoundaries);
        }

        previous = Some(*boundary);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(name: &str, kind: MetricKind) -> MetricDescriptor {
        MetricDescriptor::new(
            MetricName::new(name).unwrap(),
            kind,
            MetricUnit::new("count").unwrap(),
            "test metric",
        )
    }

    fn empty_labels() -> MetricLabels {
        BTreeMap::new()
    }

    #[test]
    fn metric_names_are_validated() {
        assert!(MetricName::new("distributed.execution.completed").is_ok());
        assert!(MetricName::new("_custom_metric-1").is_ok());
        assert!(MetricName::new("").is_err());
        assert!(MetricName::new("1invalid").is_err());
        assert!(MetricName::new("invalid name").is_err());
    }

    #[test]
    fn units_are_validated() {
        assert!(MetricUnit::new("milliseconds").is_ok());
        assert!(MetricUnit::new("bytes/second").is_ok());
        assert!(MetricUnit::new("").is_err());
        assert!(MetricUnit::new("invalid unit").is_err());
    }

    #[test]
    fn sensitive_label_names_are_rejected() {
        assert!(LabelName::new("outcome").is_ok());
        assert!(LabelName::new("password").is_err());
        assert!(LabelName::new("access_token").is_err());
        assert!(LabelName::new("private_key").is_err());
    }

    #[test]
    fn counter_registration_and_updates_work() {
        let registry = MetricsRegistry::default();
        let metric = descriptor(
            "distributed.execution.completed",
            MetricKind::Counter,
        );
        let name = metric.name().clone();
        let labels = empty_labels();

        registry.register(metric, labels.clone()).unwrap();

        assert_eq!(registry.increment(&name, &labels, 3).unwrap(), 3);
        assert_eq!(registry.increment(&name, &labels, 4).unwrap(), 7);
    }

    #[test]
    fn counter_overflow_is_reported() {
        let registry = MetricsRegistry::default();
        let metric = descriptor("distributed.counter", MetricKind::Counter);
        let name = metric.name().clone();
        let labels = empty_labels();

        registry.register(metric, labels.clone()).unwrap();

        {
            let mut state = registry.state.lock().unwrap();
            state.values.insert(
                SeriesKey {
                    name: name.clone(),
                    labels: labels.clone(),
                },
                MetricValue::Counter(u64::MAX),
            );
        }

        assert!(matches!(
            registry.increment(&name, &labels, 1),
            Err(MetricsError::CounterOverflow { .. })
        ));
    }

    #[test]
    fn gauge_rejects_non_finite_values() {
        let registry = MetricsRegistry::default();
        let metric = descriptor("distributed.queue.depth", MetricKind::Gauge);
        let name = metric.name().clone();
        let labels = empty_labels();

        registry.register(metric, labels.clone()).unwrap();

        assert!(registry.set_gauge(&name, &labels, 12.0).is_ok());
        assert_eq!(
            registry.set_gauge(&name, &labels, f64::NAN),
            Err(MetricsError::NonFiniteGauge)
        );
        assert_eq!(
            registry.set_gauge(&name, &labels, f64::INFINITY),
            Err(MetricsError::NonFiniteGauge)
        );
    }

    #[test]
    fn histogram_records_observations() {
        let registry = MetricsRegistry::default();
        let metric = descriptor(
            "distributed.execution.duration",
            MetricKind::Histogram,
        );
        let name = metric.name().clone();
        let labels = empty_labels();

        registry
            .register_histogram(
                metric,
                labels.clone(),
                vec![1.0, 5.0, 10.0],
            )
            .unwrap();

        registry.observe(&name, &labels, 0.5).unwrap();
        registry.observe(&name, &labels, 3.0).unwrap();
        registry.observe(&name, &labels, 7.0).unwrap();
        registry.observe(&name, &labels, 15.0).unwrap();

        let snapshot = registry.snapshot().unwrap();

        let MetricSnapshotValue::Histogram {
            bucket_counts,
            count,
            sum,
            minimum,
            maximum,
            ..
        } = &snapshot.series[0].value
        else {
            panic!("expected histogram snapshot");
        };

        assert_eq!(*count, 4);
        assert_eq!(*sum, 25.5);
        assert_eq!(*minimum, Some(0.5));
        assert_eq!(*maximum, Some(15.0));
        assert_eq!(bucket_counts, &[1, 1, 1, 1]);
    }

    #[test]
    fn histogram_rejects_invalid_boundaries() {
        let registry = MetricsRegistry::default();

        let result = registry.register_histogram(
            descriptor("distributed.invalid.histogram", MetricKind::Histogram),
            empty_labels(),
            vec![1.0, 1.0, 2.0],
        );

        assert_eq!(
            result,
            Err(MetricsError::InvalidHistogramBoundaries)
        );
    }

    #[test]
    fn histogram_rejects_negative_and_non_finite_observations() {
        let registry = MetricsRegistry::default();
        let metric = descriptor("distributed.latency", MetricKind::Histogram);
        let name = metric.name().clone();
        let labels = empty_labels();

        registry
            .register_histogram(metric, labels.clone(), vec![1.0, 2.0])
            .unwrap();

        assert_eq!(
            registry.observe(&name, &labels, -1.0),
            Err(MetricsError::InvalidHistogramObservation)
        );

        assert_eq!(
            registry.observe(&name, &labels, f64::INFINITY),
            Err(MetricsError::InvalidHistogramObservation)
        );
    }

    #[test]
    fn series_budget_is_enforced_without_dropping_existing_series() {
        let maximum = NonZeroUsize::new(1).unwrap();
        let registry = MetricsRegistry::new(
            MetricsConfig::with_max_series(maximum),
        );

        let first = descriptor("distributed.first", MetricKind::Counter);
        registry.register(first, empty_labels()).unwrap();

        let second = descriptor("distributed.second", MetricKind::Counter);

        assert_eq!(
            registry.register(second, empty_labels()),
            Err(MetricsError::SeriesBudgetExceeded { maximum: 1 })
        );

        assert_eq!(registry.series_count().unwrap(), 1);
    }

    #[test]
    fn repeated_registration_is_idempotent() {
        let registry = MetricsRegistry::default();
        let metric = descriptor("distributed.requests", MetricKind::Counter);

        registry
            .register(metric.clone(), empty_labels())
            .unwrap();

        registry.register(metric, empty_labels()).unwrap();

        assert_eq!(registry.series_count().unwrap(), 1);
    }

    #[test]
    fn conflicting_metric_kind_is_rejected() {
        let registry = MetricsRegistry::default();

        registry
            .register(
                descriptor("distributed.operations", MetricKind::Counter),
                empty_labels(),
            )
            .unwrap();

        let result = registry.register(
            descriptor("distributed.operations", MetricKind::Gauge),
            empty_labels(),
        );

        assert!(matches!(
            result,
            Err(MetricsError::ConflictingDescriptor { .. })
        ));
    }

    #[test]
    fn snapshots_are_deterministically_ordered() {
        let registry = MetricsRegistry::default();

        registry
            .register(
                descriptor("distributed.z_metric", MetricKind::Counter),
                empty_labels(),
            )
            .unwrap();

        registry
            .register(
                descriptor("distributed.a_metric", MetricKind::Gauge),
                empty_labels(),
            )
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        assert_eq!(
            snapshot.series[0].descriptor.name().as_str(),
            "distributed.a_metric"
        );

        assert_eq!(
            snapshot.series[1].descriptor.name().as_str(),
            "distributed.z_metric"
        );
    }
}
