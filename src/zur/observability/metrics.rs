
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Metrics
//!
//! Path: src/zur/observability/metrics.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Provides ZUR-specific runtime metrics and typed metric handles.
//!
//! This module measures runtime operations and operational behavior
//! across classical, quantum, HDL, AI, tensor, hybrid, distributed,
//! accelerator and future computational domains.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - ZUR metric naming and namespace validation.
//! - ZUR metric descriptors.
//! - ZUR metric registration interfaces.
//! - Typed runtime counter, gauge and histogram handles.
//! - Runtime metric configuration.
//! - Runtime metric collection integration.
//! - Runtime metric snapshots.
//!
//! It does not own:
//!
//! - Generic metric storage.
//! - Generic metric aggregation.
//! - Distributed metric identities.
//! - Execution lifecycle state.
//! - Runtime operation identity allocation.
//! - Distributed tracing.
//! - Source-language observability semantics.
//! - Compiler profiling.
//! - JIT hotness decisions.
//! - Resource allocation.
//! - Resource admission.
//! - Hardware discovery.
//! - Metric persistence.
//! - Metric export protocols.
//! - Domain-specific semantic measurements.
//!
//! CANONICAL METRICS AUTHORITY
//!
//! Generic metric collection is delegated to:
//!
//! crate::distributed::observability::metrics
//!
//! RUNTIME IDENTITY AUTHORITY
//!
//! RuntimeOperationId remains owned by:
//!
//! crate::zur::ids
//!
//! OPERATIONAL ERROR AUTHORITY
//!
//! ZurError and ZurResult remain owned by:
//!
//! crate::zur::error
//!
//! ARCHITECTURAL PRINCIPLES
//!
//! Registration does not imply execution.
//! Measurement does not imply successful computation.
//! Metrics do not make scheduling decisions.
//! Metrics do not allocate physical resources.
//! Metrics failures are not computation failures.
//!
//! SCALABILITY
//!
//! No fixed maximum is imposed on metric definitions,
//! observations, providers, domains or hardware resources.
//!
//! An optional series budget is an operational constraint.
//! It is not a semantic or hardware capability ceiling.
//!
//! Actual storage and throughput remain bounded by available resources.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable registry.
//! No fabricated observations.
//! No automatic network export.
//!
//! INTEGRATION
//!
//! - distributed/observability/metrics.rs:
//!   Canonical generic metric collection.
//!
//! - zur/ids.rs:
//!   RuntimeOperationId.
//!
//! - zur/error.rs:
//!   ZurError and ZurResult.
//!
//! - zur/observability/events.rs:
//!   Future structured runtime event-to-metric adapters.
//!
//! - zur/observability/tracing.rs:
//!   Trace and span correlation.
//!
//! - zur/observability/profiling.rs:
//!   Runtime execution profiling.
//!
//! - zur/execution/:
//!   Runtime execution measurements.
//!
//! - zur/jit/:
//!   Runtime compilation and specialization measurements.
//!
//! - zur/resources/:
//!   Operational resource measurements.
//!
//! - zur/distributed/:
//!   Runtime integration with canonical distributed execution.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::distributed::observability::metrics::{
    LabelName,
    LabelValue,
    MetricDescriptor,
    MetricKind,
    MetricLabels,
    MetricName,
    MetricSeriesSnapshot,
    MetricSnapshot,
    MetricSnapshotValue,
    MetricUnit,
    MetricsConfig,
    MetricsError,
    MetricsRegistry,
};

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result of a ZUR metrics operation.
pub type ZurMetricsResult<T> = ZurResult<T>;

// ============================================================================
// ERRORS
// ============================================================================

/// Converts a generic metrics error into the canonical ZUR error model.
///
/// Metrics failures are operational failures. They must not automatically
/// be reported as failures of the computation being observed.
fn metrics_error(error: MetricsError) -> ZurError {
    let kind = match &error {
        MetricsError::InvalidMetricName
        | MetricsError::InvalidUnit
        | MetricsError::InvalidLabelName
        | MetricsError::SensitiveLabelName
        | MetricsError::EmptyLabelValue
        | MetricsError::InvalidHistogramBoundaries
        | MetricsError::InvalidHistogramObservation
        | MetricsError::NonFiniteGauge => ErrorKind::Validation,

        MetricsError::ConflictingDescriptor { .. } => {
            ErrorKind::ContractViolation
        }

        MetricsError::CounterOverflow { .. } => ErrorKind::ResourceUnavailable,

        MetricsError::SeriesBudgetExceeded { .. } => {
            ErrorKind::BudgetExhausted
        }

        MetricsError::RegistryPoisoned => ErrorKind::Concurrency,

        MetricsError::InternalInvariantViolation => ErrorKind::Internal,
    };

    ZurError::with_code(
        kind,
        "zur.observability.metrics.failure",
        error.to_string(),
    )
    .expect("static ZUR metrics error code must be valid")
    .with_component("zur.observability.metrics")
}

// ============================================================================
// METRIC NAME
// ============================================================================

/// A validated ZUR-specific metric name.
///
/// All ZUR metric names must begin with:
///
/// `zur.`
///
/// Examples:
///
/// - `zur.execution.operations`
/// - `zur.execution.duration`
/// - `zur.resources.memory.usage`
/// - `zur.concurrency.tasks.completed`
/// - `zur.jit.compilations`
/// - `zur.distributed.execution.failures`
/// - `zur.providers.operations`
///
/// The namespace is extensible. It is not a closed metric catalogue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZurMetricName(MetricName);

impl ZurMetricName {
    /// Creates a validated ZUR metric name.
    pub fn new(value: impl Into<String>) -> ZurMetricsResult<Self> {
        let value = value.into();

        if !value.starts_with("zur.")
            || value.len() == "zur.".len()
        {
            return Err(
                ZurError::invalid_input(
                    "ZUR metric names must begin with the nonempty namespace `zur.`",
                )
                .with_component("zur.observability.metrics"),
            );
        }

        let name = MetricName::new(value)
            .map_err(metrics_error)?;

        Ok(Self(name))
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the underlying canonical generic metric name.
    #[must_use]
    pub fn as_generic_name(&self) -> &MetricName {
        &self.0
    }
}

impl fmt::Display for ZurMetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// METRIC UNIT
// ============================================================================

/// A validated ZUR metric unit.
///
/// Units are descriptive metadata.
///
/// Dimensional analysis and unit conversion remain the responsibility
/// of the appropriate semantic or measurement subsystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZurMetricUnit(MetricUnit);

impl ZurMetricUnit {
    /// Creates a metric unit.
    pub fn new(value: impl Into<String>) -> ZurMetricsResult<Self> {
        let unit = MetricUnit::new(value)
            .map_err(metrics_error)?;

        Ok(Self(unit))
    }

    /// Returns the unit identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the underlying generic unit.
    #[must_use]
    pub fn as_generic_unit(&self) -> &MetricUnit {
        &self.0
    }
}

// ============================================================================
// METRIC LABELS
// ============================================================================

/// A deterministic set of validated ZUR metric labels.
///
/// Labels should describe bounded categories.
///
/// Appropriate examples:
///
/// - execution_mode
/// - operation_kind
/// - outcome
/// - service_category
/// - provider_category
/// - execution_domain
///
/// Do not use:
///
/// - RuntimeOperationId.
/// - ExecutionId.
/// - SessionId.
/// - Trace IDs.
/// - Arbitrary timestamps.
/// - User identities.
/// - Credentials.
/// - Private execution payloads.
///
/// High-cardinality correlation belongs in tracing and structured events.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZurMetricLabels {
    labels: MetricLabels,
}

impl ZurMetricLabels {
    /// Creates an empty label collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            labels: BTreeMap::new(),
        }
    }

    /// Inserts or replaces a label.
    ///
    /// The underlying generic registry validates label names and values.
    ///
    /// Returns the previous value, if present.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurMetricsResult<Option<String>> {
        let name = LabelName::new(name)
            .map_err(metrics_error)?;

        let value = LabelValue::new(value)
            .map_err(metrics_error)?;

        let previous = self.labels.insert(name, value);

        Ok(previous.map(|value| value.as_str().to_owned()))
    }

    /// Returns the number of labels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Returns the canonical generic metric labels.
    #[must_use]
    pub fn as_generic_labels(&self) -> &MetricLabels {
        &self.labels
    }
}

// ============================================================================
// METRIC DESCRIPTOR
// ============================================================================

/// Describes one ZUR runtime metric.
///
/// The descriptor does not imply that the metric has been registered,
/// collected, exported or persisted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZurMetricDescriptor {
    name: ZurMetricName,
    kind: MetricKind,
    unit: ZurMetricUnit,
    description: String,
}

impl ZurMetricDescriptor {
    /// Creates a ZUR metric descriptor.
    pub fn new(
        name: ZurMetricName,
        kind: MetricKind,
        unit: ZurMetricUnit,
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
    pub fn name(&self) -> &ZurMetricName {
        &self.name
    }

    /// Returns the metric kind.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        self.kind
    }

    /// Returns the metric unit.
    #[must_use]
    pub fn unit(&self) -> &ZurMetricUnit {
        &self.unit
    }

    /// Returns the description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    fn to_generic(&self) -> MetricDescriptor {
        MetricDescriptor::new(
            self.name.0.clone(),
            self.kind,
            self.unit.0.clone(),
            self.description.clone(),
        )
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration for ZUR metric collection.
///
/// A series budget is optional.
///
/// An absent budget means that this adapter does not impose an additional
/// registry-level series limit. It does not promise unlimited physical
/// memory or processing capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ZurMetricsConfig {
    max_series: Option<NonZeroUsize>,
}

impl ZurMetricsConfig {
    /// Creates a configuration without an explicit series budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_series: None,
        }
    }

    /// Creates a configuration with an explicit series budget.
    ///
    /// The budget is operational, not semantic.
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

    fn to_generic(self) -> MetricsConfig {
        match self.max_series {
            Some(maximum) => {
                MetricsConfig::with_max_series(maximum)
            }

            None => MetricsConfig::unlimited(),
        }
    }
}

// ============================================================================
// METRIC HANDLES
// ============================================================================

/// Handle to a registered ZUR counter.
///
/// Clones share the same underlying metric series.
#[derive(Debug, Clone)]
pub struct ZurCounter {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl ZurCounter {
    /// Adds an amount to the counter.
    ///
    /// Counter overflow is reported explicitly.
    pub fn increment(&self, amount: u64) -> ZurMetricsResult<u64> {
        self.registry
            .increment(&self.name, &self.labels, amount)
            .map_err(metrics_error)
    }

    /// Increments the counter by one.
    pub fn increment_one(&self) -> ZurMetricsResult<u64> {
        self.increment(1)
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// Handle to a registered ZUR gauge.
///
/// Gauges may increase or decrease.
#[derive(Debug, Clone)]
pub struct ZurGauge {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl ZurGauge {
    /// Sets the gauge to a finite value.
    pub fn set(&self, value: f64) -> ZurMetricsResult<()> {
        self.registry
            .set_gauge(&self.name, &self.labels, value)
            .map_err(metrics_error)
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// Handle to a registered ZUR histogram.
///
/// The caller supplies finite, strictly increasing boundaries.
///
/// The underlying registry maintains an additional positive-infinity
/// bucket.
#[derive(Debug, Clone)]
pub struct ZurHistogram {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl ZurHistogram {
    /// Records a finite, non-negative observation.
    pub fn observe(&self, value: f64) -> ZurMetricsResult<()> {
        self.registry
            .observe(&self.name, &self.labels, value)
            .map_err(metrics_error)
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

// ============================================================================
// REGISTRY
// ============================================================================

/// ZUR-specific adapter over the canonical generic metrics registry.
///
/// This registry does not create another metrics storage engine.
///
/// Cloning a registry shares its underlying collection.
///
/// The registry does not:
///
/// - Create a global singleton.
/// - Start background workers.
/// - Establish network connections.
/// - Export telemetry automatically.
/// - Persist metrics automatically.
///
/// Use `from_shared` when several subsystems must share exactly the
/// same underlying metric collection.
#[derive(Debug, Clone)]
pub struct ZurMetricsRegistry {
    registry: Arc<MetricsRegistry>,
}

impl Default for ZurMetricsRegistry {
    fn default() -> Self {
        Self::new(ZurMetricsConfig::default())
    }
}

impl ZurMetricsRegistry {
    /// Creates a new ZUR metrics registry.
    ///
    /// This creates one generic metrics registry owned through Arc.
    #[must_use]
    pub fn new(config: ZurMetricsConfig) -> Self {
        Self {
            registry: Arc::new(
                MetricsRegistry::new(config.to_generic()),
            ),
        }
    }

    /// Creates a ZUR adapter over an existing shared generic registry.
    ///
    /// This is the preferred integration when the application already
    /// has a canonical metrics registry shared by other subsystems.
    #[must_use]
    pub fn from_shared(
        registry: Arc<MetricsRegistry>,
    ) -> Self {
        Self { registry }
    }

    /// Returns the canonical shared generic metrics registry.
    ///
    /// This allows existing exporters and integration components to
    /// consume the same collection without introducing another registry.
    #[must_use]
    pub fn as_shared_registry(&self) -> &Arc<MetricsRegistry> {
        &self.registry
    }

    /// Registers a ZUR counter and returns its update handle.
    pub fn register_counter(
        &self,
        descriptor: ZurMetricDescriptor,
        labels: ZurMetricLabels,
    ) -> ZurMetricsResult<ZurCounter> {
        self.validate_kind(&descriptor, MetricKind::Counter)?;

        let name = descriptor.name.0.clone();
        let labels = labels.labels;

        self.registry
            .register(descriptor.to_generic(), labels.clone())
            .map_err(metrics_error)?;

        Ok(ZurCounter {
            registry: Arc::clone(&self.registry),
            name,
            labels,
        })
    }

    /// Registers a ZUR gauge and returns its update handle.
    pub fn register_gauge(
        &self,
        descriptor: ZurMetricDescriptor,
        labels: ZurMetricLabels,
    ) -> ZurMetricsResult<ZurGauge> {
        self.validate_kind(&descriptor, MetricKind::Gauge)?;

        let name = descriptor.name.0.clone();
        let labels = labels.labels;

        self.registry
            .register(descriptor.to_generic(), labels.clone())
            .map_err(metrics_error)?;

        Ok(ZurGauge {
            registry: Arc::clone(&self.registry),
            name,
            labels,
        })
    }

    /// Registers a ZUR histogram and returns its update handle.
    ///
    /// Histogram boundaries are supplied by the caller. No universal
    /// resolution or bucket-count ceiling is imposed.
    pub fn register_histogram(
        &self,
        descriptor: ZurMetricDescriptor,
        labels: ZurMetricLabels,
        boundaries: Vec<f64>,
    ) -> ZurMetricsResult<ZurHistogram> {
        self.validate_kind(&descriptor, MetricKind::Histogram)?;

        let name = descriptor.name.0.clone();
        let labels = labels.labels;

        self.registry
            .register_histogram(
                descriptor.to_generic(),
                labels.clone(),
                boundaries,
            )
            .map_err(metrics_error)?;

        Ok(ZurHistogram {
            registry: Arc::clone(&self.registry),
            name,
            labels,
        })
    }

    /// Returns a deterministic snapshot of all registered metric series
    /// in the underlying shared registry.
    ///
    /// When the registry is shared with other subsystems, their generic
    /// metric series may also appear in this snapshot.
    pub fn snapshot(&self) -> ZurMetricsResult<MetricSnapshot> {
        self.registry
            .snapshot()
            .map_err(metrics_error)
    }

    /// Returns the number of registered series in the shared registry.
    pub fn series_count(&self) -> ZurMetricsResult<usize> {
        self.registry
            .series_count()
            .map_err(metrics_error)
    }

    /// Returns whether this registry shares the same allocation as
    /// another adapter.
    ///
    /// This is an identity comparison, not a metric compatibility test.
    #[must_use]
    pub fn shares_registry_with(
        &self,
        other: &Self,
    ) -> bool {
        Arc::ptr_eq(&self.registry, &other.registry)
    }

    fn validate_kind(
        &self,
        descriptor: &ZurMetricDescriptor,
        expected: MetricKind,
    ) -> ZurMetricsResult<()> {
        if descriptor.kind != expected {
            return Err(
                ZurError::with_code(
                    ErrorKind::ContractViolation,
                    "zur.observability.metrics.kind_mismatch",
                    format!(
                        "metric `{}` has kind `{}` but registration requires `{}`",
                        descriptor.name,
                        descriptor.kind.as_str(),
                        expected.as_str(),
                    ),
                )
                .expect("static ZUR error code must be valid")
                .with_component("zur.observability.metrics"),
            );
        }

        Ok(())
    }
}

// ============================================================================
// PUBLIC SNAPSHOT TYPES
// ============================================================================

/// Immutable generic metric-series snapshot.
///
/// This alias deliberately preserves the canonical snapshot representation.
pub type ZurMetricSeriesSnapshot = MetricSeriesSnapshot;

/// Immutable generic metric value.
///
/// This alias avoids creating a competing snapshot value model.
pub type ZurMetricSnapshotValue = MetricSnapshotValue;

/// Immutable snapshot of the canonical metrics registry.
///
/// This alias preserves the generic metrics authority.
pub type ZurMetricSnapshot = MetricSnapshot;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(
        name: &str,
        kind: MetricKind,
        unit: &str,
    ) -> ZurMetricDescriptor {
        ZurMetricDescriptor::new(
            ZurMetricName::new(name).expect("valid metric name"),
            kind,
            ZurMetricUnit::new(unit).expect("valid unit"),
            "ZUR runtime test metric",
        )
    }

    fn labels() -> ZurMetricLabels {
        ZurMetricLabels::new()
    }

    #[test]
    fn metric_names_require_zur_namespace() {
        assert!(
            ZurMetricName::new("zur.execution.operations").is_ok()
        );

        assert!(
            ZurMetricName::new("zur.resources.memory.usage").is_ok()
        );

        assert!(
            ZurMetricName::new("execution.operations").is_err()
        );

        assert!(
            ZurMetricName::new("zur.").is_err()
        );
    }

    #[test]
    fn future_metric_names_remain_extensible() {
        assert!(
            ZurMetricName::new(
                "zur.future.photonics.optical.operations"
            )
            .is_ok()
        );
    }

    #[test]
    fn labels_use_canonical_validation() {
        let mut values = labels();

        assert!(
            values.insert("outcome", "completed").is_ok()
        );

        assert!(
            values.insert("access_token", "secret").is_err()
        );

        assert_eq!(values.len(), 1);
    }

    #[test]
    fn counter_registration_and_updates_work() {
        let registry = ZurMetricsRegistry::default();

        let counter = registry
            .register_counter(
                descriptor(
                    "zur.execution.operations",
                    MetricKind::Counter,
                    "count",
                ),
                labels(),
            )
            .expect("counter registration");

        assert_eq!(counter.increment(3).expect("increment"), 3);

        assert_eq!(counter.increment_one().expect("increment"), 4);

        assert_eq!(
            registry.series_count().expect("series count"),
            1,
        );
    }

    #[test]
    fn gauge_registration_and_updates_work() {
        let registry = ZurMetricsRegistry::default();

        let gauge = registry
            .register_gauge(
                descriptor(
                    "zur.resources.memory.usage",
                    MetricKind::Gauge,
                    "bytes",
                ),
                labels(),
            )
            .expect("gauge registration");

        assert!(gauge.set(1024.0).is_ok());

        assert!(gauge.set(f64::NAN).is_err());
    }

    #[test]
    fn histogram_registration_and_updates_work() {
        let registry = ZurMetricsRegistry::default();

        let histogram = registry
            .register_histogram(
                descriptor(
                    "zur.execution.duration",
                    MetricKind::Histogram,
                    "seconds",
                ),
                labels(),
                vec![0.001, 0.01, 0.1, 1.0],
            )
            .expect("histogram registration");

        assert!(histogram.observe(0.005).is_ok());

        assert!(histogram.observe(0.5).is_ok());

        assert!(histogram.observe(-1.0).is_err());

        assert!(histogram.observe(f64::INFINITY).is_err());
    }

    #[test]
    fn incompatible_metric_kind_is_rejected() {
        let registry = ZurMetricsRegistry::default();

        let result = registry.register_counter(
            descriptor(
                "zur.execution.operations",
                MetricKind::Gauge,
                "count",
            ),
            labels(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn cloned_handles_share_the_same_metric_series() {
        let registry = ZurMetricsRegistry::default();

        let counter = registry
            .register_counter(
                descriptor(
                    "zur.execution.operations",
                    MetricKind::Counter,
                    "count",
                ),
                labels(),
            )
            .expect("registration");

        let cloned = counter.clone();

        assert_eq!(counter.increment_one().expect("increment"), 1);

        assert_eq!(cloned.increment_one().expect("increment"), 2);
    }

    #[test]
    fn adapters_can_share_one_generic_registry() {
        let generic = Arc::new(MetricsRegistry::default());

        let first = ZurMetricsRegistry::from_shared(
            Arc::clone(&generic),
        );

        let second = ZurMetricsRegistry::from_shared(generic);

        assert!(first.shares_registry_with(&second));
    }

    #[test]
    fn snapshots_are_available() {
        let registry = ZurMetricsRegistry::default();

        registry
            .register_counter(
                descriptor(
                    "zur.execution.operations",
                    MetricKind::Counter,
                    "count",
                ),
                labels(),
            )
            .expect("registration");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.series.len(), 1);
    }

    #[test]
    fn explicit_series_budget_is_respected() {
        let registry = ZurMetricsRegistry::new(
            ZurMetricsConfig::with_max_series(
                NonZeroUsize::new(1).expect("nonzero"),
            ),
        );

        registry
            .register_counter(
                descriptor(
                    "zur.execution.operations",
                    MetricKind::Counter,
                    "count",
                ),
                labels(),
            )
            .expect("first series");

        let result = registry.register_counter(
            descriptor(
                "zur.execution.operations",
                MetricKind::Counter,
                "count",
            ),
            {
                let mut values = labels();
                values.insert("outcome", "failed").expect("label");
                values
            },
        );

        assert!(result.is_err());

        assert_eq!(
            registry.series_count().expect("count"),
            1,
        );
    }

    #[test]
    fn snapshots_do_not_claim_execution_success() {
        let registry = ZurMetricsRegistry::default();

        let counter = registry
            .register_counter(
                descriptor(
                    "zur.execution.operations",
                    MetricKind::Counter,
                    "count",
                ),
                labels(),
            )
            .expect("registration");

        counter.increment_one().expect("increment");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.series.len(), 1);
    }
}
