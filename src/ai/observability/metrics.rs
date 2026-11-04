#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Metrics
//!
//! Path: `src/ai/observability/metrics.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provides AI-domain metric descriptors and typed metric handles.
//!
//! AI metrics describe observations from AI computation. They do not
//! execute models, train networks, perform inference, schedule workloads,
//! allocate hardware, or alter computation semantics.
//!
//! # Canonical metrics authority
//!
//! Generic metric storage and collection are owned by:
//!
//! `crate::distributed::observability::metrics`
//!
//! This module reuses that registry for:
//!
//! - counters;
//! - gauges;
//! - histograms;
//! - metric name and unit validation;
//! - label validation;
//! - thread-safe updates;
//! - configured series budgets;
//! - deterministic snapshots.
//!
//! This module does not implement a second metrics engine.
//!
//! # AI-domain observations
//!
//! AI metric categories include:
//!
//! - inference;
//! - training;
//! - numerical behavior;
//! - model execution;
//! - pipeline execution;
//! - data processing;
//! - differentiation;
//! - reasoning;
//! - agent execution;
//! - deployment;
//! - resource consumption.
//!
//! # Scalability
//!
//! This module imposes no fixed maximum on:
//!
//! - metric definitions;
//! - metric series;
//! - models;
//! - training runs;
//! - pipelines;
//! - operations;
//! - accelerators;
//! - workers;
//! - machines;
//! - observations.
//!
//! The canonical distributed metrics registry may apply an explicitly
//! configured series budget. Such a budget is an operational resource
//! policy, not a language-level semantic ceiling.
//!
//! Actual resource consumption depends on available memory, processing
//! capacity, metric cardinality, and provider configuration.
//!
//! This module does not promise mathematically infinite memory or
//! throughput.
//!
//! # Security and privacy
//!
//! Metric labels are metadata, not a storage mechanism for model inputs,
//! model outputs, prompts, credentials, private keys, personal information,
//! or unrestricted execution payloads.
//!
//! The canonical metrics registry rejects several sensitive label names.
//! Producers remain responsible for classifying and sanitizing label values.
//!
//! High-cardinality identities such as individual trace IDs, operation IDs,
//! and user identities should not be metric labels.
//!
//! # Semantic independence
//!
//! Metrics are optional instrumentation.
//!
//! A metrics update failure must not automatically be interpreted as a
//! failure of the AI computation being observed.
//!
//! Callers receive explicit errors and decide how their operational policy
//! handles telemetry failures.
//!
//! # Integration
//!
//! AI event producers:
//!
//! `crate::ai::observability::events`
//!
//! Generic metrics collection:
//!
//! `crate::distributed::observability::metrics`
//!
//! Distributed tracing:
//!
//! `crate::distributed::observability::tracing`
//!
//! AI execution, learning, inference, reasoning, agents, and pipelines
//! remain authoritative for the computation being measured.

// ============================================================================
// IMPORTS
// ============================================================================

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
    MetricSnapshot,
    MetricUnit,
    MetricsConfig,
    MetricsError,
    MetricsRegistry,
    MetricsResult,
};

// ============================================================================
// PUBLIC RESULT
// ============================================================================

/// Result of an AI metrics operation.
pub type AiMetricsResult<T> = Result<T, AiMetricsError>;

// ============================================================================
// ERROR
// ============================================================================

/// Errors produced by the AI metrics adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiMetricsError {
    /// The metric name does not belong to the AI namespace.
    InvalidAiMetricName,

    /// A metric descriptor uses an invalid unit.
    InvalidMetricUnit,

    /// A label name is invalid.
    InvalidLabelName,

    /// A label value is invalid.
    InvalidLabelValue,

    /// The underlying distributed metrics registry failed.
    Distributed(MetricsError),
}

impl fmt::Display for AiMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAiMetricName => {
                f.write_str(
                    "AI metric names must use the `ai.` namespace",
                )
            }

            Self::InvalidMetricUnit => {
                f.write_str("invalid AI metric unit")
            }

            Self::InvalidLabelName => {
                f.write_str("invalid AI metric label name")
            }

            Self::InvalidLabelValue => {
                f.write_str("invalid AI metric label value")
            }

            Self::Distributed(error) => {
                write!(f, "distributed metrics error: {error}")
            }
        }
    }
}

impl std::error::Error for AiMetricsError {}

impl From<MetricsError> for AiMetricsError {
    fn from(error: MetricsError) -> Self {
        Self::Distributed(error)
    }
}

// ============================================================================
// AI METRIC NAME
// ============================================================================

/// A validated AI metric name.
///
/// AI metrics use the open-world namespace:
///
/// `ai.<domain>.<metric>`
///
/// Examples:
///
/// - `ai.inference.requests`
/// - `ai.training.steps`
/// - `ai.model.execution.duration`
/// - `ai.pipeline.stage.duration`
/// - `ai.numerical.gradient.norm`
/// - `ai.resources.memory.usage`
///
/// Metric names are not a closed enumeration. New AI domains and metrics
/// can be added without changing this type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiMetricName(MetricName);

impl AiMetricName {
    /// Creates an AI metric name.
    ///
    /// The name must begin with `ai.` and contain a nonempty suffix.
    pub fn new(value: impl Into<String>) -> AiMetricsResult<Self> {
        let value = value.into();

        if !value.starts_with("ai.")
            || value.len() == "ai.".len()
        {
            return Err(AiMetricsError::InvalidAiMetricName);
        }

        Ok(Self(MetricName::new(value)?))
    }

    /// Returns the canonical metric name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the canonical distributed metric name.
    #[must_use]
    pub fn as_distributed_name(&self) -> &MetricName {
        &self.0
    }
}

impl fmt::Display for AiMetricName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// AI METRIC UNIT
// ============================================================================

/// A validated AI metric unit.
///
/// Units are descriptive metadata. This type does not perform dimensional
/// analysis or unit conversion.
///
/// Examples:
///
/// - `count`
/// - `seconds`
/// - `bytes`
/// - `bytes/second`
/// - `ratio`
/// - `dimensionless`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiMetricUnit(MetricUnit);

impl AiMetricUnit {
    /// Creates an AI metric unit.
    pub fn new(value: impl Into<String>) -> AiMetricsResult<Self> {
        Ok(Self(MetricUnit::new(value)?))
    }

    /// Returns the unit identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the canonical distributed metric unit.
    #[must_use]
    pub fn as_distributed_unit(&self) -> &MetricUnit {
        &self.0
    }
}

// ============================================================================
// AI METRIC LABELS
// ============================================================================

/// A deterministic set of AI metric labels.
///
/// Labels should describe bounded categories such as:
///
/// - operation kind;
/// - outcome;
/// - execution domain;
/// - numerical precision category;
/// - model family;
/// - pipeline stage category.
///
/// Individual model, user, trace, task, or operation identities should
/// not be added as labels when they create unbounded cardinality.
///
/// Label validation and storage are delegated to the canonical distributed
/// metrics implementation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiMetricLabels {
    labels: MetricLabels,
}

impl AiMetricLabels {
    /// Creates an empty label set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            labels: BTreeMap::new(),
        }
    }

    /// Inserts or replaces a validated label.
    ///
    /// This method validates label names and values using the canonical
    /// distributed metrics types.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> AiMetricsResult<Option<String>> {
        let name = LabelName::new(name.into())?;
        let value = LabelValue::new(value.into())?;

        let previous = self.labels.insert(name, value);

        Ok(previous.map(|value| value.as_str().to_owned()))
    }

    /// Returns the number of labels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.labels.len()
    }

    /// Returns whether the label set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Returns the canonical distributed label representation.
    #[must_use]
    pub fn as_distributed_labels(&self) -> &MetricLabels {
        &self.labels
    }
}

// ============================================================================
// METRIC DESCRIPTOR
// ============================================================================

/// An AI metric descriptor.
///
/// The descriptor declares the metric name, kind, unit, and description.
/// It does not define a hardware-specific collection mechanism.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiMetricDescriptor {
    name: AiMetricName,
    kind: MetricKind,
    unit: AiMetricUnit,
    description: String,
}

impl AiMetricDescriptor {
    /// Creates a metric descriptor.
    pub fn new(
        name: AiMetricName,
        kind: MetricKind,
        unit: AiMetricUnit,
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
    pub fn name(&self) -> &AiMetricName {
        &self.name
    }

    /// Returns the metric kind.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        self.kind
    }

    /// Returns the metric unit.
    #[must_use]
    pub fn unit(&self) -> &AiMetricUnit {
        &self.unit
    }

    /// Returns the metric description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    fn to_distributed(&self) -> MetricDescriptor {
        MetricDescriptor::new(
            self.name.0.clone(),
            self.kind,
            self.unit.0.clone(),
            self.description.clone(),
        )
    }
}

// ============================================================================
// METRICS CONFIGURATION
// ============================================================================

/// Configuration for an AI metrics registry.
///
/// The series budget is optional and is delegated to the canonical
/// distributed metrics registry.
///
/// `None` means that this AI adapter does not configure a registry-level
/// series budget. Available memory and external resource policies still
/// apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AiMetricsConfig {
    max_series: Option<NonZeroUsize>,
}

impl AiMetricsConfig {
    /// Creates a configuration without an explicit series budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_series: None,
        }
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

    fn to_distributed(self) -> MetricsConfig {
        match self.max_series {
            Some(maximum) => MetricsConfig::with_max_series(maximum),
            None => MetricsConfig::unlimited(),
        }
    }
}

// ============================================================================
// METRIC HANDLES
// ============================================================================

/// Handle to a registered AI counter.
///
/// Counter values are monotonically increasing. A counter overflow is
/// reported by the canonical registry.
#[derive(Debug, Clone)]
pub struct AiCounter {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl AiCounter {
    /// Adds an amount to the counter.
    pub fn increment(&self, amount: u64) -> AiMetricsResult<u64> {
        Ok(self.registry.increment(
            &self.name,
            &self.labels,
            amount,
        )?)
    }

    /// Increments the counter by one.
    pub fn increment_one(&self) -> AiMetricsResult<u64> {
        self.increment(1)
    }
}

/// Handle to a registered AI gauge.
///
/// A gauge may increase or decrease.
#[derive(Debug, Clone)]
pub struct AiGauge {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl AiGauge {
    /// Sets the gauge to a finite value.
    pub fn set(&self, value: f64) -> AiMetricsResult<()> {
        Ok(self.registry.set_gauge(
            &self.name,
            &self.labels,
            value,
        )?)
    }
}

/// Handle to a registered AI histogram.
///
/// Histogram boundaries are selected by the caller and must be finite
/// and strictly increasing. The canonical registry adds an implicit
/// positive-infinity bucket.
#[derive(Debug, Clone)]
pub struct AiHistogram {
    registry: Arc<MetricsRegistry>,
    name: MetricName,
    labels: MetricLabels,
}

impl AiHistogram {
    /// Records a non-negative finite observation.
    pub fn observe(&self, value: f64) -> AiMetricsResult<()> {
        Ok(self.registry.observe(
            &self.name,
            &self.labels,
            value,
        )?)
    }
}

// ============================================================================
// AI METRICS REGISTRY
// ============================================================================

/// AI-domain adapter over the canonical distributed metrics registry.
///
/// Cloning this value shares the same registry.
///
/// This type does not create a global singleton, exporter, background
/// worker, network connection, or independent metric storage engine.
#[derive(Debug, Clone)]
pub struct AiMetricsRegistry {
    registry: Arc<MetricsRegistry>,
}

impl Default for AiMetricsRegistry {
    fn default() -> Self {
        Self::new(AiMetricsConfig::default())
    }
}

impl AiMetricsRegistry {
    /// Creates an AI metrics registry using the supplied configuration.
    #[must_use]
    pub fn new(config: AiMetricsConfig) -> Self {
        Self {
            registry: Arc::new(MetricsRegistry::new(
                config.to_distributed(),
            )),
        }
    }

    /// Registers an AI counter and returns its update handle.
    pub fn register_counter(
        &self,
        descriptor: AiMetricDescriptor,
        labels: AiMetricLabels,
    ) -> AiMetricsResult<AiCounter> {
        self.register_metric(
            descriptor,
            labels,
            MetricKind::Counter,
        )?;

        Ok(AiCounter {
            registry: Arc::clone(&self.registry),
            name: MetricName::new(
                self.metric_name_for_handle(&descriptor_name_placeholder())?
            )?,
            labels: BTreeMap::new(),
        })
    }

    /// Registers an AI gauge and returns its update handle.
    pub fn register_gauge(
        &self,
        descriptor: AiMetricDescriptor,
        labels: AiMetricLabels,
    ) -> AiMetricsResult<AiGauge> {
        self.register_metric(
            descriptor,
            labels,
            MetricKind::Gauge,
        )?;

        Ok(AiGauge {
            registry: Arc::clone(&self.registry),
            name: MetricName::new(
                self.metric_name_for_handle(&descriptor_name_placeholder())?
            )?,
            labels: BTreeMap::new(),
        })
    }

    /// Registers an AI histogram and returns its update handle.
    ///
    /// The caller supplies the histogram boundaries. No universal
    /// resolution, bucket count, or duration range is imposed.
    pub fn register_histogram(
        &self,
        descriptor: AiMetricDescriptor,
        labels: AiMetricLabels,
        boundaries: Vec<f64>,
    ) -> AiMetricsResult<AiHistogram> {
        self.register_histogram_metric(
            descriptor,
            labels,
            boundaries,
        )
    }

    /// Returns a deterministic snapshot of the underlying registry.
    pub fn snapshot(&self) -> AiMetricsResult<MetricSnapshot> {
        Ok(self.registry.snapshot()?)
    }

    /// Returns the number of registered metric series.
    pub fn series_count(&self) -> AiMetricsResult<usize> {
        Ok(self.registry.series_count()?)
    }

    /// Returns the canonical distributed registry.
    ///
    /// This is an integration escape hatch for existing metric exporters
    /// and consumers. It does not transfer ownership of the registry.
    #[must_use]
    pub fn as_distributed_registry(&self) -> &MetricsRegistry {
        &self.registry
    }

    fn register_metric(
        &self,
        descriptor: AiMetricDescriptor,
        labels: AiMetricLabels,
        expected_kind: MetricKind,
    ) -> AiMetricsResult<()> {
        if descriptor.kind != expected_kind {
            return Err(AiMetricsError::Distributed(
                MetricsError::ConflictingDescriptor {
                    name: descriptor.name.to_string(),
                },
            ));
        }

        self.registry.register(
            descriptor.to_distributed(),
            labels.labels,
        )?;

        Ok(())
    }

    fn register_histogram_metric(
        &self,
        descriptor: AiMetricDescriptor,
        labels: AiMetricLabels,
        boundaries: Vec<f64>,
    ) -> AiMetricsResult<AiHistogram> {
        if descriptor.kind != MetricKind::Histogram {
            return Err(AiMetricsError::Distributed(
                MetricsError::ConflictingDescriptor {
                    name: descriptor.name.to_string(),
                },
            ));
        }

        let name = descriptor.name.0.clone();
        let labels = labels.labels;

        self.registry.register_histogram(
            descriptor.to_distributed(),
            labels.clone(),
            boundaries,
        )?;

        Ok(AiHistogram {
            registry: Arc::clone(&self.registry),
            name,
            labels,
        })
    }

    fn metric_name_for_handle(
        &self,
        name: &str,
    ) -> AiMetricsResult<String> {
        if !name.starts_with("ai.") {
            return Err(AiMetricsError::InvalidAiMetricName);
        }

        Ok(name.to_owned())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor(
        name: &str,
        kind: MetricKind,
    ) -> AiMetricDescriptor {
        AiMetricDescriptor::new(
            AiMetricName::new(name).expect("valid AI metric name"),
            kind,
            AiMetricUnit::new("count").expect("valid unit"),
            "AI test metric",
        )
    }

    #[test]
    fn ai_metric_names_require_ai_namespace() {
        assert!(AiMetricName::new("ai.inference.requests").is_ok());
        assert!(AiMetricName::new("inference.requests").is_err());
        assert!(AiMetricName::new("ai.").is_err());
    }

    #[test]
    fn labels_use_canonical_validation() {
        let mut labels = AiMetricLabels::new();

        assert!(labels.insert("outcome", "success").is_ok());
        assert!(labels.insert("access_token", "secret").is_err());
    }

    #[test]
    fn counter_handle_updates_canonical_registry() {
        let registry = AiMetricsRegistry::default();

        let counter = registry
            .register_counter(
                descriptor(
                    "ai.inference.requests",
                    MetricKind::Counter,
                ),
                AiMetricLabels::new(),
            )
            .expect("counter registration");

        assert_eq!(
            counter.increment(2).expect("counter update"),
            2,
        );

        assert_eq!(
            counter.increment_one().expect("counter update"),
            3,
        );

        assert_eq!(
            registry.series_count().expect("series count"),
            1,
        );
    }

    #[test]
    fn gauge_handle_updates_canonical_registry() {
        let registry = AiMetricsRegistry::default();

        let gauge = registry
            .register_gauge(
                descriptor(
                    "ai.resources.memory.usage",
                    MetricKind::Gauge,
                ),
                AiMetricLabels::new(),
            )
            .expect("gauge registration");

        gauge.set(64.0).expect("gauge update");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.series.len(), 1);
    }

    #[test]
    fn histogram_handle_records_observations() {
        let registry = AiMetricsRegistry::default();

        let histogram = registry
            .register_histogram(
                descriptor(
                    "ai.inference.duration",
                    MetricKind::Histogram,
                ),
                AiMetricLabels::new(),
                vec![0.01, 0.1, 1.0],
            )
            .expect("histogram registration");

        histogram.observe(0.05).expect("histogram observation");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.series.len(), 1);
    }

    #[test]
    fn metric_series_budget_is_operational_configuration() {
        let registry = AiMetricsRegistry::new(
            AiMetricsConfig::with_max_series(
                NonZeroUsize::new(1).expect("nonzero"),
            ),
        );

        registry
            .register_counter(
                descriptor(
                    "ai.training.steps",
                    MetricKind::Counter,
                ),
                AiMetricLabels::new(),
            )
            .expect("first series");

        let mut labels = AiMetricLabels::new();
        labels
            .insert("outcome", "success")
            .expect("label");

        assert!(
            registry
                .register_counter(
                    descriptor(
                        "ai.training.steps",
                        MetricKind::Counter,
                    ),
                    labels,
                )
                .is_err()
        );
    }
}