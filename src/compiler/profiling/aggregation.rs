
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Profiling Aggregation.
//!
//! Path:
//!     src/compiler/profiling/aggregation.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Aggregates immutable compiler profiling Measurement records.
//!
//! This module provides streaming aggregation. It does not retain
//! the original measurements after updating the corresponding
//! aggregate.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Measurement grouping.
//! - Streaming aggregation.
//! - Numeric accumulation.
//! - Aggregate snapshots.
//! - Aggregate merging.
//! - Configurable aggregation budgets.
//! - Explicit overflow behavior.
//! - Adaptation of individual measurements into the canonical
//!   distributed MetricsRegistry.
//!
//! This module does not own:
//!
//! - Measurement construction.
//! - Compiler stage identity.
//! - Compiler stage execution.
//! - Event transport.
//! - Distributed tracing.
//! - Metric registration infrastructure.
//! - The canonical metrics registry.
//! - Hardware performance counters.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Optimization decisions.
//! - Persistent storage.
//! - Network exporters.
//!
//! # Canonical authorities
//!
//! Measurement records:
//!     crate::compiler::profiling::measurements
//!
//! Compiler stage identity:
//!     crate::compiler::pipeline::stage
//!
//! Metric names, units and registry:
//!     crate::distributed::observability::metrics
//!
//! Distributed event and correlation metadata:
//!     crate::distributed::observability::events
//!
//! # Scalability
//!
//! There are no universal limits on:
//!
//! - observations;
//! - compiler stages;
//! - computational domains;
//! - hardware types;
//! - devices;
//! - workers;
//! - targets;
//! - metric names;
//! - metric units.
//!
//! Aggregation retains one accumulator per configured grouping key,
//! not an unbounded history of measurements.
//!
//! An optional maximum group count is an operational budget.
//! It is not a semantic or hardware limit.
//!
//! # Determinism
//!
//! Group keys and snapshots use BTreeMap ordering.
//!
//! Integer accumulation is exact and checked.
//!
//! Floating-point accumulation is checked for non-finite results.
//!
//! Snapshot ordering does not depend on thread scheduling.
//!
//! Floating-point addition is not mathematically associative.
//! Therefore, merging floating-point snapshots in different orders
//! may produce different least-significant bits.
//!
//! # Duplicate observations
//!
//! Every successful record() call represents one observation.
//!
//! Measurement does not currently provide a globally unique sample
//! identity. This module therefore does not guess whether two
//! identical measurements represent a retry or two real observations.
//!
//! Producers requiring exactly-once ingestion must deduplicate using
//! their authoritative event or observation identity before calling
//! record().
//!
//! # Retention
//!
//! Raw measurements are not retained.
//!
//! Aggregate groups remain until explicitly removed by the owner or
//! the aggregator is dropped.
//!
//! Persistence and long-term retention belong to downstream storage
//! providers.
//!
//! # Concurrency
//!
//! A standard-library Mutex provides a portable, consistent baseline.
//!
//! High-contention deployments may implement a compatible sharded
//! aggregator after benchmarking actual workloads.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::{Mutex, PoisonError};

use crate::compiler::profiling::measurements::{
    Measurement,
    MeasurementStability,
    MeasurementValue,
};

use crate::distributed::observability::events::EventTimestamp;

use crate::distributed::observability::metrics::{
    LabelValue,
    MetricDescriptor,
    MetricKind,
    MetricLabels,
    MetricName,
    MetricUnit,
    MetricsError,
    MetricsRegistry,
};

/// Public API version for compiler profiling aggregation.
pub const COMPILER_PROFILE_AGGREGATION_API_VERSION: u16 = 1;

/// Result returned by aggregation operations.
pub type AggregationResult<T> = Result<T, AggregationError>;

/// Determines how the aggregator behaves when a new group would
/// exceed its configured group-count budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregationOverflowPolicy {
    /// Reject the new observation without changing existing groups.
    Reject,

    /// Explicitly discard the new observation and report that it
    /// was not aggregated.
    ///
    /// Existing aggregate values are never modified by this policy.
    DropNewest,
}

/// Determines which measurement dimensions distinguish groups.
///
/// Grouping dimensions are configurable because compilation identity
/// and stage identity can create high-cardinality datasets.
///
/// A disabled dimension is deliberately omitted from the group key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregationDimensions {
    /// Distinguish observations by their measurement source.
    pub source: bool,

    /// Distinguish deterministic and environment-dependent data.
    pub stability: bool,

    /// Distinguish compiler pipeline stages.
    pub stage: bool,

    /// Distinguish individual compilation identities.
    ///
    /// This is disabled by default because compilation identities
    /// can create high cardinality.
    pub compilation_identity: bool,
}

impl Default for AggregationDimensions {
    fn default() -> Self {
        Self {
            source: true,
            stability: true,
            stage: true,
            compilation_identity: false,
        }
    }
}

/// Configuration for a compiler profiling aggregator.
///
/// `max_groups` is an optional operational resource budget.
///
/// `None` does not mean unlimited physical memory. It means that
/// this component does not impose an additional group-count limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AggregationConfig {
    max_groups: Option<NonZeroUsize>,
    overflow_policy: AggregationOverflowPolicy,
    dimensions: AggregationDimensions,
}

impl Default for AggregationConfig {
    fn default() -> Self {
        Self {
            max_groups: None,
            overflow_policy: AggregationOverflowPolicy::Reject,
            dimensions: AggregationDimensions::default(),
        }
    }
}

impl AggregationConfig {
    /// Creates an aggregator configuration without an explicit
    /// group-count budget.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_groups: None,
            overflow_policy: AggregationOverflowPolicy::Reject,
            dimensions: AggregationDimensions {
                source: true,
                stability: true,
                stage: true,
                compilation_identity: false,
            },
        }
    }

    /// Sets an optional operational group-count budget.
    #[must_use]
    pub const fn with_max_groups(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.max_groups = maximum;
        self
    }

    /// Sets the behavior when the group budget is exhausted.
    #[must_use]
    pub const fn with_overflow_policy(
        mut self,
        policy: AggregationOverflowPolicy,
    ) -> Self {
        self.overflow_policy = policy;
        self
    }

    /// Sets the dimensions used to distinguish aggregate groups.
    #[must_use]
    pub const fn with_dimensions(
        mut self,
        dimensions: AggregationDimensions,
    ) -> Self {
        self.dimensions = dimensions;
        self
    }

    /// Returns the optional group-count budget.
    #[must_use]
    pub const fn max_groups(self) -> Option<NonZeroUsize> {
        self.max_groups
    }

    /// Returns the overflow policy.
    #[must_use]
    pub const fn overflow_policy(self) -> AggregationOverflowPolicy {
        self.overflow_policy
    }

    /// Returns the grouping dimensions.
    #[must_use]
    pub const fn dimensions(self) -> AggregationDimensions {
        self.dimensions
    }
}

/// Describes the outcome of one aggregation ingestion operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregationOutcome {
    /// The observation was successfully incorporated.
    Recorded,

    /// The observation was deliberately discarded by the
    /// configured DropNewest policy.
    DroppedByBudget,
}

/// Errors returned by profiling aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregationError {
    /// The aggregator synchronization primitive was poisoned.
    SynchronizationPoisoned,

    /// The configured group-count budget has been exhausted.
    GroupBudgetExceeded {
        /// Configured maximum group count.
        maximum: usize,
    },

    /// The aggregate observation count would overflow.
    CountOverflow,

    /// The numeric aggregate would overflow.
    NumericOverflow,

    /// The aggregate received incompatible numeric representations.
    NumericRepresentationMismatch,

    /// A floating-point result became non-finite.
    NonFiniteResult,

    /// A supplied snapshot is incompatible with the destination.
    IncompatibleSnapshot,

    /// The metric descriptor does not describe the supplied
    /// measurement.
    DescriptorMismatch,

    /// The selected metric kind cannot represent the measurement.
    UnsupportedMetricValue,

    /// Conversion to the registry's floating-point representation
    /// would lose integer precision.
    NumericPrecisionLoss,

    /// The canonical distributed metrics subsystem rejected an
    /// operation.
    Metrics(MetricsError),
}

impl fmt::Display for AggregationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::SynchronizationPoisoned => {
                formatter.write_str(
                    "compiler profiling aggregation synchronization was poisoned",
                )
            }

            Self::GroupBudgetExceeded { maximum } => {
                write!(
                    formatter,
                    "compiler profiling aggregation group budget exceeded: {maximum}"
                )
            }

            Self::CountOverflow => {
                formatter.write_str(
                    "compiler profiling aggregate count overflow",
                )
            }

            Self::NumericOverflow => {
                formatter.write_str(
                    "compiler profiling numeric accumulation overflow",
                )
            }

            Self::NumericRepresentationMismatch => {
                formatter.write_str(
                    "compiler profiling aggregate has incompatible numeric representations",
                )
            }

            Self::NonFiniteResult => {
                formatter.write_str(
                    "compiler profiling aggregation produced a non-finite result",
                )
            }

            Self::IncompatibleSnapshot => {
                formatter.write_str(
                    "compiler profiling aggregation snapshot is incompatible",
                )
            }

            Self::DescriptorMismatch => {
                formatter.write_str(
                    "metric descriptor does not match the measurement",
                )
            }

            Self::UnsupportedMetricValue => {
                formatter.write_str(
                    "measurement cannot be represented by the selected metric kind",
                )
            }

            Self::NumericPrecisionLoss => {
                formatter.write_str(
                    "conversion to floating-point metric representation would lose integer precision",
                )
            }

            Self::Metrics(error) => {
                write!(
                    formatter,
                    "distributed metrics registry error: {error}"
                )
            }
        }
    }
}

impl Error for AggregationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Metrics(error) => Some(error),
            _ => None,
        }
    }
}

impl From<MetricsError> for AggregationError {
    fn from(error: MetricsError) -> Self {
        Self::Metrics(error)
    }
}

impl<T> From<PoisonError<T>> for AggregationError {
    fn from(_: PoisonError<T>) -> Self {
        Self::SynchronizationPoisoned
    }
}

/// Exact or floating-point aggregate total.
///
/// Integer totals are not converted to floating-point values.
/// This prevents silent loss of integer precision.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum NumericTotal {
    /// Exact unsigned integer total.
    Unsigned(u128),

    /// Exact signed integer total.
    Signed(i128),

    /// Finite floating-point total.
    Float(f64),
}

/// Stable identity of one aggregate group.
///
/// Group keys deliberately exclude arbitrary measurement attributes
/// and distributed trace identifiers by default. This prevents
/// accidental cardinality explosion.
///
/// The grouping dimensions are selected by AggregationConfig.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AggregateKey {
    name: MetricName,
    unit: MetricUnit,
    source: Option<String>,
    stability: Option<String>,
    stage: Option<String>,
    compilation_identity: Option<String>,
}

impl AggregateKey {
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

    /// Returns the measurement source when included in grouping.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the measurement stability classification when
    /// included in grouping.
    #[must_use]
    pub fn stability(&self) -> Option<&str> {
        self.stability.as_deref()
    }

    /// Returns the stage identity when included in grouping.
    #[must_use]
    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }

    /// Returns the compilation identity when included in grouping.
    #[must_use]
    pub fn compilation_identity(&self) -> Option<&str> {
        self.compilation_identity.as_deref()
    }
}

/// Immutable summary of one aggregate group.
#[derive(Debug, Clone, PartialEq)]
pub struct AggregateSummary {
    key: AggregateKey,
    count: u128,
    total: NumericTotal,
    minimum: MeasurementValue,
    maximum: MeasurementValue,
    first_observed_at: EventTimestamp,
    last_observed_at: EventTimestamp,
}

impl AggregateSummary {
    /// Returns the group identity.
    #[must_use]
    pub fn key(&self) -> &AggregateKey {
        &self.key
    }

    /// Returns the number of accepted observations.
    #[must_use]
    pub const fn count(&self) -> u128 {
        self.count
    }

    /// Returns the accumulated numeric total.
    #[must_use]
    pub const fn total(&self) -> NumericTotal {
        self.total
    }

    /// Returns the minimum observed value.
    #[must_use]
    pub const fn minimum(&self) -> MeasurementValue {
        self.minimum
    }

    /// Returns the maximum observed value.
    #[must_use]
    pub const fn maximum(&self) -> MeasurementValue {
        self.maximum
    }

    /// Returns the earliest observation timestamp.
    #[must_use]
    pub const fn first_observed_at(&self) -> EventTimestamp {
        self.first_observed_at
    }

    /// Returns the latest observation timestamp.
    #[must_use]
    pub const fn last_observed_at(&self) -> EventTimestamp {
        self.last_observed_at
    }

    /// Returns the arithmetic mean when it can be represented
    /// as a finite f64.
    ///
    /// This is a presentation convenience. The exact integer
    /// total remains available through total().
    #[must_use]
    pub fn mean(&self) -> Option<f64> {
        let total = match self.total {
            NumericTotal::Unsigned(value) => value as f64,
            NumericTotal::Signed(value) => value as f64,
            NumericTotal::Float(value) => value,
        };

        let mean = total / self.count as f64;

        mean.is_finite().then_some(mean)
    }
}

/// Deterministic immutable snapshot of aggregate groups.
#[derive(Debug, Clone, PartialEq)]
pub struct AggregationSnapshot {
    api_version: u16,
    groups: Vec<AggregateSummary>,
}

impl AggregationSnapshot {
    /// Returns the aggregation API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        self.api_version
    }

    /// Returns groups in deterministic key order.
    #[must_use]
    pub fn groups(&self) -> &[AggregateSummary] {
        &self.groups
    }

    /// Returns the number of groups.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Returns whether the snapshot contains no groups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns the total accepted observation count.
    ///
    /// Returns None if the sum of group counts cannot be represented.
    #[must_use]
    pub fn observation_count(&self) -> Option<u128> {
        self.groups
            .iter()
            .try_fold(0_u128, |total, group| {
                total.checked_add(group.count)
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AccumulatedTotal {
    Unsigned(u128),
    Signed(i128),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct Accumulator {
    count: u128,
    total: AccumulatedTotal,
    minimum: MeasurementValue,
    maximum: MeasurementValue,
    first_observed_at: EventTimestamp,
    last_observed_at: EventTimestamp,
}

impl Accumulator {
    fn new(measurement: &Measurement) -> Self {
        let value = measurement.value();
        let timestamp = measurement.observed_at();

        let total = match value {
            MeasurementValue::Unsigned(value) => {
                AccumulatedTotal::Unsigned(value)
            }
            MeasurementValue::Signed(value) => {
                AccumulatedTotal::Signed(value)
            }
            MeasurementValue::Float(value) => {
                AccumulatedTotal::Float(value)
            }
        };

        Self {
            count: 1,
            total,
            minimum: value,
            maximum: value,
            first_observed_at: timestamp,
            last_observed_at: timestamp,
        }
    }

    fn add(
        &mut self,
        measurement: &Measurement,
    ) -> AggregationResult<()> {
        let value = measurement.value();

        let next_count = self
            .count
            .checked_add(1)
            .ok_or(AggregationError::CountOverflow)?;

        let next_total = match (self.total, value) {
            (
                AccumulatedTotal::Unsigned(current),
                MeasurementValue::Unsigned(incoming),
            ) => {
                AccumulatedTotal::Unsigned(
                    current
                        .checked_add(incoming)
                        .ok_or(AggregationError::NumericOverflow)?,
                )
            }

            (
                AccumulatedTotal::Signed(current),
                MeasurementValue::Signed(incoming),
            ) => {
                AccumulatedTotal::Signed(
                    current
                        .checked_add(incoming)
                        .ok_or(AggregationError::NumericOverflow)?,
                )
            }

            (
                AccumulatedTotal::Float(current),
                MeasurementValue::Float(incoming),
            ) => {
                let result = current + incoming;

                if !result.is_finite() {
                    return Err(AggregationError::NonFiniteResult);
                }

                AccumulatedTotal::Float(result)
            }

            _ => {
                return Err(
                    AggregationError::NumericRepresentationMismatch,
                );
            }
        };

        let next_minimum = minimum(value, self.minimum)?;
        let next_maximum = maximum(value, self.maximum)?;

        let timestamp = measurement.observed_at();

        self.count = next_count;
        self.total = next_total;
        self.minimum = next_minimum;
        self.maximum = next_maximum;
        self.first_observed_at =
            self.first_observed_at.min(timestamp);
        self.last_observed_at =
            self.last_observed_at.max(timestamp);

        Ok(())
    }

    fn merge(
        &mut self,
        other: &Self,
    ) -> AggregationResult<()> {
        let next_count = self
            .count
            .checked_add(other.count)
            .ok_or(AggregationError::CountOverflow)?;

        let next_total = match (self.total, other.total) {
            (
                AccumulatedTotal::Unsigned(left),
                AccumulatedTotal::Unsigned(right),
            ) => {
                AccumulatedTotal::Unsigned(
                    left.checked_add(right)
                        .ok_or(AggregationError::NumericOverflow)?,
                )
            }

            (
                AccumulatedTotal::Signed(left),
                AccumulatedTotal::Signed(right),
            ) => {
                AccumulatedTotal::Signed(
                    left.checked_add(right)
                        .ok_or(AggregationError::NumericOverflow)?,
                )
            }

            (
                AccumulatedTotal::Float(left),
                AccumulatedTotal::Float(right),
            ) => {
                let result = left + right;

                if !result.is_finite() {
                    return Err(AggregationError::NonFiniteResult);
                }

                AccumulatedTotal::Float(result)
            }

            _ => {
                return Err(
                    AggregationError::NumericRepresentationMismatch,
                );
            }
        };

        let next_minimum = minimum(
            self.minimum,
            other.minimum,
        )?;

        let next_maximum = maximum(
            self.maximum,
            other.maximum,
        )?;

        self.count = next_count;
        self.total = next_total;
        self.minimum = next_minimum;
        self.maximum = next_maximum;
        self.first_observed_at = self
            .first_observed_at
            .min(other.first_observed_at);
        self.last_observed_at = self
            .last_observed_at
            .max(other.last_observed_at);

        Ok(())
    }

    fn summary(
        &self,
        key: AggregateKey,
    ) -> AggregateSummary {
        let total = match self.total {
            AccumulatedTotal::Unsigned(value) => {
                NumericTotal::Unsigned(value)
            }
            AccumulatedTotal::Signed(value) => {
                NumericTotal::Signed(value)
            }
            AccumulatedTotal::Float(value) => {
                NumericTotal::Float(value)
            }
        };

        AggregateSummary {
            key,
            count: self.count,
            total,
            minimum: self.minimum,
            maximum: self.maximum,
            first_observed_at: self.first_observed_at,
            last_observed_at: self.last_observed_at,
        }
    }
}

fn minimum(
    left: MeasurementValue,
    right: MeasurementValue,
) -> AggregationResult<MeasurementValue> {
    match (left, right) {
        (
            MeasurementValue::Unsigned(a),
            MeasurementValue::Unsigned(b),
        ) => Ok(MeasurementValue::Unsigned(a.min(b))),

        (
            MeasurementValue::Signed(a),
            MeasurementValue::Signed(b),
        ) => Ok(MeasurementValue::Signed(a.min(b))),

        (
            MeasurementValue::Float(a),
            MeasurementValue::Float(b),
        ) => Ok(MeasurementValue::Float(a.min(b))),

        _ => Err(
            AggregationError::NumericRepresentationMismatch,
        ),
    }
}

fn maximum(
    left: MeasurementValue,
    right: MeasurementValue,
) -> AggregationResult<MeasurementValue> {
    match (left, right) {
        (
            MeasurementValue::Unsigned(a),
            MeasurementValue::Unsigned(b),
        ) => Ok(MeasurementValue::Unsigned(a.max(b))),

        (
            MeasurementValue::Signed(a),
            MeasurementValue::Signed(b),
        ) => Ok(MeasurementValue::Signed(a.max(b))),

        (
            MeasurementValue::Float(a),
            MeasurementValue::Float(b),
        ) => Ok(MeasurementValue::Float(a.max(b))),

        _ => Err(
            AggregationError::NumericRepresentationMismatch,
        ),
    }
}

#[derive(Debug, Default)]
struct AggregatorState {
    groups: BTreeMap<AggregateKey, Accumulator>,
}

/// Thread-safe streaming compiler profiling aggregator.
///
/// The aggregator stores summaries rather than raw observations.
///
/// Clones are not implicitly shared. Share the aggregator through
/// `Arc<ProfileAggregator>` when multiple producers need one instance.
#[derive(Debug)]
pub struct ProfileAggregator {
    config: AggregationConfig,
    state: Mutex<AggregatorState>,
}

impl Default for ProfileAggregator {
    fn default() -> Self {
        Self::new(AggregationConfig::default())
    }
}

impl ProfileAggregator {
    /// Creates a new profiling aggregator.
    #[must_use]
    pub fn new(config: AggregationConfig) -> Self {
        Self {
            config,
            state: Mutex::new(AggregatorState::default()),
        }
    }

    /// Returns the immutable aggregator configuration.
    #[must_use]
    pub const fn config(&self) -> AggregationConfig {
        self.config
    }

    /// Records one immutable measurement.
    ///
    /// Every successful call counts as one observation.
    ///
    /// A failed call leaves the previous aggregate unchanged.
    pub fn record(
        &self,
        measurement: &Measurement,
    ) -> AggregationResult<AggregationOutcome> {
        let key = self.make_key(measurement);
        let mut state = self.state.lock()?;

        if let Some(existing) = state.groups.get_mut(&key) {
            existing.add(measurement)?;
            return Ok(AggregationOutcome::Recorded);
        }

        if let Some(maximum) = self.config.max_groups {
            if state.groups.len() >= maximum.get() {
                return match self.config.overflow_policy {
                    AggregationOverflowPolicy::Reject => {
                        Err(AggregationError::GroupBudgetExceeded {
                            maximum: maximum.get(),
                        })
                    }

                    AggregationOverflowPolicy::DropNewest => {
                        Ok(AggregationOutcome::DroppedByBudget)
                    }
                };
            }
        }

        state.groups.insert(
            key,
            Accumulator::new(measurement),
        );

        Ok(AggregationOutcome::Recorded)
    }

    /// Records multiple measurements incrementally.
    ///
    /// Processing stops on the first error.
    ///
    /// Earlier successful observations remain committed.
    ///
    /// The returned count is the number of accepted observations.
    pub fn record_all<'a, I>(
        &self,
        measurements: I,
    ) -> AggregationResult<u128>
    where
        I: IntoIterator<Item = &'a Measurement>,
    {
        let mut accepted = 0_u128;

        for measurement in measurements {
            match self.record(measurement)? {
                AggregationOutcome::Recorded => {
                    accepted = accepted
                        .checked_add(1)
                        .ok_or(AggregationError::CountOverflow)?;
                }

                AggregationOutcome::DroppedByBudget => {}
            }
        }

        Ok(accepted)
    }

    /// Returns a deterministic immutable snapshot.
    pub fn snapshot(&self) -> AggregationResult<AggregationSnapshot> {
        let state = self.state.lock()?;

        let groups = state
            .groups
            .iter()
            .map(|(key, accumulator)| {
                accumulator.summary(key.clone())
            })
            .collect();

        Ok(AggregationSnapshot {
            api_version: COMPILER_PROFILE_AGGREGATION_API_VERSION,
            groups,
        })
    }

    /// Returns the number of aggregate groups.
    pub fn group_count(&self) -> AggregationResult<usize> {
        Ok(self.state.lock()?.groups.len())
    }

    /// Returns the number of accepted observations.
    ///
    /// Returns an error if the total cannot be represented.
    pub fn observation_count(&self) -> AggregationResult<u128> {
        self.state
            .lock()?
            .groups
            .values()
            .try_fold(0_u128, |total, accumulator| {
                total
                    .checked_add(accumulator.count)
                    .ok_or(AggregationError::CountOverflow)
            })
    }

    /// Removes all aggregate groups.
    ///
    /// This is an explicit lifecycle operation. It does not happen
    /// automatically when a budget is reached.
    pub fn clear(&self) -> AggregationResult<()> {
        self.state.lock()?.groups.clear();
        Ok(())
    }

    /// Merges an immutable snapshot into this aggregator.
    ///
    /// The merge is transactional: if any group is incompatible,
    /// overflows, or exceeds the group budget, no changes are made.
    ///
    /// Snapshot group ordering is not relied upon for correctness.
    pub fn merge_snapshot(
        &self,
        snapshot: &AggregationSnapshot,
    ) -> AggregationResult<()> {
        if snapshot.api_version
            != COMPILER_PROFILE_AGGREGATION_API_VERSION
        {
            return Err(AggregationError::IncompatibleSnapshot);
        }

        let mut state = self.state.lock()?;
        let mut replacement = state.groups.clone();

        for summary in &snapshot.groups {
            let key = summary.key.clone();

            let incoming = accumulator_from_summary(summary)?;

            if let Some(existing) = replacement.get_mut(&key) {
                existing.merge(&incoming)?;
            } else {
                if let Some(maximum) = self.config.max_groups {
                    if replacement.len() >= maximum.get() {
                        return Err(
                            AggregationError::GroupBudgetExceeded {
                                maximum: maximum.get(),
                            },
                        );
                    }
                }

                replacement.insert(key, incoming);
            }
        }

        state.groups = replacement;

        Ok(())
    }

    fn make_key(
        &self,
        measurement: &Measurement,
    ) -> AggregateKey {
        let dimensions = self.config.dimensions;

        AggregateKey {
            name: measurement.name().clone(),
            unit: measurement.unit().clone(),

            source: dimensions
                .source
                .then(|| measurement.source().as_str().to_owned()),

            stability: dimensions
                .stability
                .then(|| measurement.stability().as_str().to_owned()),

            stage: dimensions
                .stage
                .then(|| measurement.stage().map(|stage| {
                    stage.as_str().to_owned()
                }))
                .flatten(),

            compilation_identity: dimensions
                .compilation_identity
                .then(|| measurement.compilation_identity().map(str::to_owned))
                .flatten(),
        }
    }
}

fn accumulator_from_summary(
    summary: &AggregateSummary,
) -> AggregationResult<Accumulator> {
    if summary.count == 0 {
        return Err(AggregationError::IncompatibleSnapshot);
    }

    let total = match summary.total {
        NumericTotal::Unsigned(value) => {
            AccumulatedTotal::Unsigned(value)
        }
        NumericTotal::Signed(value) => {
            AccumulatedTotal::Signed(value)
        }
        NumericTotal::Float(value) if value.is_finite() => {
            AccumulatedTotal::Float(value)
        }
        NumericTotal::Float(_) => {
            return Err(AggregationError::NonFiniteResult);
        }
    };

    let compatible = matches!(
        (summary.minimum, summary.maximum, total),
        (
            MeasurementValue::Unsigned(_),
            MeasurementValue::Unsigned(_),
            AccumulatedTotal::Unsigned(_)
        ) | (
            MeasurementValue::Signed(_),
            MeasurementValue::Signed(_),
            AccumulatedTotal::Signed(_)
        ) | (
            MeasurementValue::Float(_),
            MeasurementValue::Float(_),
            AccumulatedTotal::Float(_)
        )
    );

    if !compatible {
        return Err(AggregationError::IncompatibleSnapshot);
    }

    Ok(Accumulator {
        count: summary.count,
        total,
        minimum: summary.minimum,
        maximum: summary.maximum,
        first_observed_at: summary.first_observed_at,
        last_observed_at: summary.last_observed_at,
    })
}

/// Publishes one measurement into the existing distributed metrics
/// registry.
///
/// The descriptor and series must already be registered with the
/// registry. This adapter does not create another registry.
///
/// The descriptor name and unit must exactly match the measurement.
///
/// Counter observations must be unsigned integers representable by
/// the registry's u64 counter.
///
/// Gauge and histogram observations use f64. Integer values are
/// accepted only when conversion is exact.
///
/// Histogram observations must be non-negative.
pub fn publish_measurement_to_registry(
    registry: &MetricsRegistry,
    descriptor: &MetricDescriptor,
    labels: &MetricLabels,
    measurement: &Measurement,
) -> AggregationResult<()> {
    if descriptor.name() != measurement.name()
        || descriptor.unit() != measurement.unit()
    {
        return Err(AggregationError::DescriptorMismatch);
    }

    match descriptor.kind() {
        MetricKind::Counter => {
            let amount = match measurement.value() {
                MeasurementValue::Unsigned(value) => {
                    u64::try_from(value)
                        .map_err(|_| AggregationError::NumericOverflow)?
                }
                _ => {
                    return Err(
                        AggregationError::UnsupportedMetricValue,
                    );
                }
            };

            registry.increment(
                measurement.name(),
                labels,
                amount,
            )?;
        }

        MetricKind::Gauge => {
            let value = value_as_exact_f64(measurement.value())?;

            registry.set_gauge(
                measurement.name(),
                labels,
                value,
            )?;
        }

        MetricKind::Histogram => {
            let value = value_as_exact_f64(measurement.value())?;

            if value < 0.0 {
                return Err(
                    AggregationError::UnsupportedMetricValue,
                );
            }

            registry.observe(
                measurement.name(),
                labels,
                value,
            )?;
        }
    }

    Ok(())
}

fn value_as_exact_f64(
    value: MeasurementValue,
) -> AggregationResult<f64> {
    let converted = match value {
        MeasurementValue::Unsigned(value) => {
            let converted = value as f64;

            if !converted.is_finite()
                || converted as u128 != value
            {
                return Err(
                    AggregationError::NumericPrecisionLoss,
                );
            }

            converted
        }

        MeasurementValue::Signed(value) => {
            let converted = value as f64;

            if !converted.is_finite()
                || converted as i128 != value
            {
                return Err(
                    AggregationError::NumericPrecisionLoss,
                );
            }

            converted
        }

        MeasurementValue::Float(value) => {
            if !value.is_finite() {
                return Err(AggregationError::NonFiniteResult);
            }

            value
        }
    };

    Ok(converted)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::profiling::measurements::{
        MeasurementSource,
    };

    fn measurement(
        name: &str,
        value: MeasurementValue,
        timestamp: EventTimestamp,
    ) -> Measurement {
        Measurement::new(
            MetricName::new(name).expect("valid metric name"),
            MetricUnit::new("count").expect("valid unit"),
            value,
            timestamp,
            MeasurementStability::Deterministic,
            MeasurementSource::new("compiler.tests")
                .expect("valid measurement source"),
        )
        .expect("valid measurement")
    }

    fn timestamp(seconds: u64) -> EventTimestamp {
        EventTimestamp::new(seconds, 0)
            .expect("valid timestamp")
    }

    #[test]
    fn records_and_summarizes_integer_measurements() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.operations",
                MeasurementValue::Unsigned(4),
                timestamp(10),
            ))
            .expect("record succeeds");

        aggregator
            .record(&measurement(
                "compiler.operations",
                MeasurementValue::Unsigned(6),
                timestamp(11),
            ))
            .expect("record succeeds");

        let snapshot = aggregator.snapshot().expect("snapshot succeeds");

        assert_eq!(snapshot.group_count(), 1);

        let group = &snapshot.groups()[0];

        assert_eq!(group.count(), 2);
        assert_eq!(group.total(), NumericTotal::Unsigned(10));
        assert_eq!(
            group.minimum(),
            MeasurementValue::Unsigned(4)
        );
        assert_eq!(
            group.maximum(),
            MeasurementValue::Unsigned(6)
        );
        assert_eq!(group.mean(), Some(5.0));
    }

    #[test]
    fn snapshots_are_sorted_and_deterministic() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.z",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("record succeeds");

        aggregator
            .record(&measurement(
                "compiler.a",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("record succeeds");

        let first = aggregator.snapshot().expect("snapshot");
        let second = aggregator.snapshot().expect("snapshot");

        assert_eq!(first, second);
        assert!(
            first.groups()[0].key().name().as_str()
                < first.groups()[1].key().name().as_str()
        );
    }

    #[test]
    fn group_budget_rejects_new_groups_without_mutation() {
        let config = AggregationConfig::default()
            .with_max_groups(NonZeroUsize::new(1));

        let aggregator = ProfileAggregator::new(config);

        aggregator
            .record(&measurement(
                "compiler.first",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("first group succeeds");

        let result = aggregator.record(&measurement(
            "compiler.second",
            MeasurementValue::Unsigned(1),
            timestamp(2),
        ));

        assert!(matches!(
            result,
            Err(AggregationError::GroupBudgetExceeded {
                maximum: 1
            })
        ));

        assert_eq!(
            aggregator.group_count().expect("count"),
            1
        );
    }

    #[test]
    fn drop_newest_reports_data_loss_explicitly() {
        let config = AggregationConfig::default()
            .with_max_groups(NonZeroUsize::new(1))
            .with_overflow_policy(
                AggregationOverflowPolicy::DropNewest,
            );

        let aggregator = ProfileAggregator::new(config);

        aggregator
            .record(&measurement(
                "compiler.first",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("first group succeeds");

        let outcome = aggregator
            .record(&measurement(
                "compiler.second",
                MeasurementValue::Unsigned(1),
                timestamp(2),
            ))
            .expect("drop is an explicit outcome");

        assert_eq!(
            outcome,
            AggregationOutcome::DroppedByBudget
        );

        assert_eq!(
            aggregator.observation_count().expect("count"),
            1
        );
    }

    #[test]
    fn integer_overflow_does_not_mutate_existing_aggregate() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.counter",
                MeasurementValue::Unsigned(u128::MAX),
                timestamp(1),
            ))
            .expect("first observation");

        let result = aggregator.record(&measurement(
            "compiler.counter",
            MeasurementValue::Unsigned(1),
            timestamp(2),
        ));

        assert_eq!(
            result,
            Err(AggregationError::NumericOverflow)
        );

        let snapshot = aggregator.snapshot().expect("snapshot");

        assert_eq!(
            snapshot.groups()[0].total(),
            NumericTotal::Unsigned(u128::MAX)
        );

        assert_eq!(snapshot.groups()[0].count(), 1);
    }

    #[test]
    fn mixed_numeric_representations_are_rejected() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.metric",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("first observation");

        let result = aggregator.record(&measurement(
            "compiler.metric",
            MeasurementValue::Float(1.5),
            timestamp(2),
        ));

        assert_eq!(
            result,
            Err(AggregationError::NumericRepresentationMismatch)
        );
    }

    #[test]
    fn floating_point_overflow_is_rejected() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.float",
                MeasurementValue::Float(f64::MAX),
                timestamp(1),
            ))
            .expect("first observation");

        let result = aggregator.record(&measurement(
            "compiler.float",
            MeasurementValue::Float(f64::MAX),
            timestamp(2),
        ));

        assert_eq!(
            result,
            Err(AggregationError::NonFiniteResult)
        );
    }

    #[test]
    fn merge_snapshot_combines_compatible_groups() {
        let left = ProfileAggregator::default();
        let right = ProfileAggregator::default();

        left.record(&measurement(
            "compiler.merge",
            MeasurementValue::Unsigned(2),
            timestamp(1),
        ))
        .expect("left observation");

        right.record(&measurement(
            "compiler.merge",
            MeasurementValue::Unsigned(3),
            timestamp(2),
        ))
        .expect("right observation");

        let right_snapshot = right.snapshot().expect("snapshot");

        left.merge_snapshot(&right_snapshot)
            .expect("merge succeeds");

        let snapshot = left.snapshot().expect("snapshot");
        let group = &snapshot.groups()[0];

        assert_eq!(group.count(), 2);
        assert_eq!(group.total(), NumericTotal::Unsigned(5));
        assert_eq!(group.first_observed_at(), timestamp(1));
        assert_eq!(group.last_observed_at(), timestamp(2));
    }

    #[test]
    fn compilation_identity_grouping_is_configurable() {
        let dimensions = AggregationDimensions {
            source: true,
            stability: true,
            stage: true,
            compilation_identity: true,
        };

        let aggregator = ProfileAggregator::new(
            AggregationConfig::default()
                .with_dimensions(dimensions),
        );

        let first = measurement(
            "compiler.build",
            MeasurementValue::Unsigned(1),
            timestamp(1),
        )
        .with_compilation_identity("build-a")
        .expect("identity");

        let second = measurement(
            "compiler.build",
            MeasurementValue::Unsigned(1),
            timestamp(2),
        )
        .with_compilation_identity("build-b")
        .expect("identity");

        aggregator.record(&first).expect("record");
        aggregator.record(&second).expect("record");

        assert_eq!(
            aggregator.group_count().expect("count"),
            2
        );
    }

    #[test]
    fn clear_removes_all_aggregate_groups() {
        let aggregator = ProfileAggregator::default();

        aggregator
            .record(&measurement(
                "compiler.clear",
                MeasurementValue::Unsigned(1),
                timestamp(1),
            ))
            .expect("record");

        aggregator.clear().expect("clear");

        assert_eq!(
            aggregator.group_count().expect("count"),
            0
        );
    }

    #[test]
    fn integer_to_f64_adapter_rejects_precision_loss() {
        assert_eq!(
            value_as_exact_f64(
                MeasurementValue::Unsigned(u128::MAX)
            ),
            Err(AggregationError::NumericPrecisionLoss)
        );
    }

    #[test]
    fn aggregation_dimensions_can_combine_sources() {
        let dimensions = AggregationDimensions {
            source: false,
            stability: true,
            stage: true,
            compilation_identity: false,
        };

        let aggregator = ProfileAggregator::new(
            AggregationConfig::default()
                .with_dimensions(dimensions),
        );

        let first = measurement(
            "compiler.shared",
            MeasurementValue::Unsigned(2),
            timestamp(1),
        );

        let second = Measurement::new(
            MetricName::new("compiler.shared").unwrap(),
            MetricUnit::new("count").unwrap(),
            MeasurementValue::Unsigned(3),
            timestamp(2),
            MeasurementStability::Deterministic,
            MeasurementSource::new("backend.provider").unwrap(),
        )
        .unwrap();

        aggregator.record(&first).unwrap();
        aggregator.record(&second).unwrap();

        assert_eq!(aggregator.group_count().unwrap(), 1);
        assert_eq!(
            aggregator.snapshot().unwrap().groups()[0].total(),
            NumericTotal::Unsigned(5)
        );
    }
}
