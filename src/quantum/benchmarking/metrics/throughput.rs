//! Zamani Quantum Benchmarking — Throughput Metrics
//!
//! Production throughput calculations for quantum-computing benchmarks.
//!
//! # Purpose
//!
//! This module converts measured workload counts and elapsed execution time
//! into canonical Zamani [`Metric`] values.
//!
//! It intentionally does NOT:
//!
//! - execute circuits;
//! - submit jobs;
//! - communicate with hardware;
//! - generate benchmark circuits;
//! - determine backend capabilities;
//! - perform protocol-specific compilation;
//! - own the benchmark result model;
//! - depend on a particular quantum-computing technology.
//!
//! Those responsibilities belong to the execution, protocol, hardware, and
//! result layers respectively.
//!
//! # Supported throughput families
//!
//! The module supports:
//!
//! - generic operations/second;
//! - shots/second;
//! - circuits/second;
//! - gates/second;
//! - two-qubit gates/second;
//! - circuit layers/second;
//! - maximum circuits/second (MCPS-style measurement);
//! - CLOPS-style circuit-layer throughput;
//! - quality-gated throughput;
//! - repeated observations and aggregate statistics.
//!
//! # Architectural position
//!
//! ```text
//! quantum::benchmarking::execution
//!              │
//!              ▼
//!       execution timing/counts
//!              │
//!              ▼
//!   benchmarking::metrics::throughput
//!              │
//!              ▼
//!       core::metric::Metric
//!              │
//!       ┌──────┴────────┐
//!       ▼               ▼
//!   core::result    reporting
//! ```
//!
//! The dependency direction is deliberately one-way:
//!
//! ```text
//! throughput -> core::metric
//! ```
//!
//! Never:
//!
//! ```text
//! core::metric -> throughput
//! ```
//!
//! # Important semantic distinction
//!
//! Throughput is not the same thing as fidelity or quality.
//!
//! A system may execute many circuits per second while producing poor
//! results. Conversely, a high-quality system may have low throughput.
//!
//! Therefore this module can attach a quality-envelope description to a
//! throughput metric, but it does not silently combine quality and speed into
//! one number.
//!
//! # CLOPS
//!
//! CLOPS means Circuit Layer Operations Per Second. It is a workload-specific
//! throughput metric rather than a universal replacement for circuits/sec.
//!
//! The conventional form is conceptually:
//
//! ```text
//! CLOPS = templates × parameter_updates × shots × layers / elapsed_time
//! ```
//!
//! The exact timing boundary and workload definition MUST be recorded by the
//! caller. Different benchmark protocols can legitimately produce different
//! numbers if they measure different timing scopes.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//! Edition: Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration
//!
//! This file depends only on:
//!
//! ```text
//! crate::quantum::benchmarking::core::metric
//! std::time::Duration
//! ```
//!
//! It is therefore safe to implement before the execution, protocol,
//! statistics, reporting, and result modules.
//!
//! Once `metrics/mod.rs` exists, expose it with:
//!
//! ```text
//! pub mod throughput;
//! ```
//!
//! No changes to `core::metric` are required because the repository already
//! defines the required throughput metric kinds and rate units.

use std::fmt;
use std::time::Duration;

use crate::quantum::benchmarking::core::metric::{
    FiniteF64,
    Metric,
    MetricError,
    MetricKind,
    MetricResult,
    MetricUnit,
};

/// Smallest meaningful elapsed time accepted by the throughput calculator.
///
/// A zero-duration measurement is invalid because a finite throughput cannot
/// be inferred from zero elapsed time.
///
/// The implementation additionally rejects `Duration::ZERO` directly, so this
/// constant is primarily documentation of the semantic boundary.
const MIN_ELAPSED_TIME: Duration = Duration::from_nanos(1);

/// A measured workload count and its elapsed wall-clock duration.
///
/// This is the fundamental input to ordinary throughput calculations.
///
/// The count is deliberately `u64` so the metric can represent workloads
/// larger than `usize` on 32-bit targets and remain independent of host
/// pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThroughputObservation {
    /// Number of completed workload items.
    pub count: u64,

    /// Elapsed time associated with the completed workload.
    pub elapsed: Duration,
}

impl ThroughputObservation {
    /// Creates a throughput observation.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - `count == 0`;
    /// - `elapsed == Duration::ZERO`.
    pub fn new(count: u64, elapsed: Duration) -> Result<Self, ThroughputError> {
        if count == 0 {
            return Err(ThroughputError::ZeroWork);
        }

        validate_duration(elapsed)?;

        Ok(Self { count, elapsed })
    }

    /// Calculates the raw rate represented by this observation.
    pub fn rate_per_second(&self) -> Result<f64, ThroughputError> {
        calculate_rate(self.count, self.elapsed)
    }
}

/// Supported throughput dimensions.
///
/// Each variant maps to an existing canonical Zamani metric kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThroughputKind {
    /// Generic operations per second.
    OperationsPerSecond,

    /// Measurement shots per second.
    ShotsPerSecond,

    /// Completed circuits per second.
    CircuitsPerSecond,

    /// Quantum gates per second.
    GatesPerSecond,

    /// Two-qubit gates per second.
    TwoQubitGatesPerSecond,

    /// Circuit layers per second.
    LayersPerSecond,

    /// Generic throughput.
    Throughput,
}

impl ThroughputKind {
    /// Returns the canonical Zamani metric kind.
    pub fn metric_kind(self) -> MetricKind {
        match self {
            Self::OperationsPerSecond => MetricKind::Throughput,
            Self::ShotsPerSecond => MetricKind::ShotsPerSecond,
            Self::CircuitsPerSecond => MetricKind::CircuitsPerSecond,
            Self::GatesPerSecond => MetricKind::GatesPerSecond,
            Self::TwoQubitGatesPerSecond => MetricKind::TwoQubitGatesPerSecond,
            Self::LayersPerSecond => MetricKind::LayersPerSecond,
            Self::Throughput => MetricKind::Throughput,
        }
    }

    /// Returns a stable identifier for the throughput dimension.
    pub const fn id(self) -> &'static str {
        match self {
            Self::OperationsPerSecond => "operations_per_second",
            Self::ShotsPerSecond => "shots_per_second",
            Self::CircuitsPerSecond => "circuits_per_second",
            Self::GatesPerSecond => "gates_per_second",
            Self::TwoQubitGatesPerSecond => "two_qubit_gates_per_second",
            Self::LayersPerSecond => "layers_per_second",
            Self::Throughput => "throughput",
        }
    }
}

/// A single canonical throughput metric with its measurement semantics.
///
/// This wrapper retains the source count and timing boundary in addition to
/// the canonical [`Metric`]. The canonical metric remains the value intended
/// for insertion into `BenchmarkResult`.
#[derive(Debug, Clone, PartialEq)]
pub struct ThroughputMeasurement {
    /// Throughput dimension.
    pub kind: ThroughputKind,

    /// Number of completed workload units.
    pub count: u64,

    /// Measured elapsed time.
    pub elapsed: Duration,

    /// Canonical Zamani metric.
    pub metric: Metric,
}

impl ThroughputMeasurement {
    /// Calculates a throughput measurement from one observation.
    pub fn from_observation(
        kind: ThroughputKind,
        observation: ThroughputObservation,
    ) -> Result<Self, ThroughputError> {
        let rate = observation.rate_per_second()?;

        let metric = Metric::new(
            kind.metric_kind(),
            MetricUnit::Hertz,
            rate,
        )?
        .with_sample_count(observation.count)
        .map_err(ThroughputError::Metric)?;

        Ok(Self {
            kind,
            count: observation.count,
            elapsed: observation.elapsed,
            metric,
        })
    }

    /// Returns the calculated rate.
    #[inline]
    pub fn rate_per_second(&self) -> f64 {
        self.metric.value.get()
    }
}

/// A quality envelope that can be associated with a throughput measurement.
///
/// The throughput value itself remains independent of quality. This type
/// records the quality condition under which the throughput was measured.
///
/// For example, a benchmark may require:
///
/// ```text
/// layer_fidelity >= 0.90
/// ```
///
/// and then report the achieved throughput under that condition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QualityEnvelope {
    /// Minimum acceptable quality.
    pub minimum_quality: f64,
}

impl QualityEnvelope {
    /// Creates a quality envelope.
    ///
    /// Quality is represented as a normalized fraction in [0, 1].
    pub fn new(minimum_quality: f64) -> Result<Self, ThroughputError> {
        validate_unit_interval(
            minimum_quality,
            "quality-envelope minimum",
        )?;

        Ok(Self { minimum_quality })
    }

    /// Returns whether an observed quality satisfies the envelope.
    pub fn accepts(&self, observed_quality: f64) -> Result<bool, ThroughputError> {
        validate_unit_interval(
            observed_quality,
            "observed quality",
        )?;

        Ok(observed_quality >= self.minimum_quality)
    }
}

/// A throughput measurement together with a quality-envelope evaluation.
///
/// This is intentionally separate from [`ThroughputMeasurement`] so that
/// ordinary circuits/sec calculations do not imply any quality claim.
#[derive(Debug, Clone, PartialEq)]
pub struct QualityGatedThroughput {
    /// Underlying throughput measurement.
    pub throughput: ThroughputMeasurement,

    /// Required quality envelope.
    pub envelope: QualityEnvelope,

    /// Actual measured quality.
    pub observed_quality: f64,

    /// Whether the quality requirement was satisfied.
    pub accepted: bool,
}

impl QualityGatedThroughput {
    /// Constructs a quality-gated throughput result.
    pub fn new(
        throughput: ThroughputMeasurement,
        envelope: QualityEnvelope,
        observed_quality: f64,
    ) -> Result<Self, ThroughputError> {
        let accepted = envelope.accepts(observed_quality)?;

        Ok(Self {
            throughput,
            envelope,
            observed_quality,
            accepted,
        })
    }
}

/// Inputs for a CLOPS-style calculation.
///
/// This structure deliberately exposes every numerator term instead of
/// accepting a precomputed numerator. This prevents callers from hiding
/// overflow, unit mistakes, or protocol semantics inside one integer.
///
/// The resulting metric is:
///
/// ```text
/// templates × parameter_updates × shots × layers / elapsed_time
/// ```
///
/// Each factor must represent the workload actually included in the timing
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClopsWorkload {
    /// Number of circuit templates.
    pub templates: u64,

    /// Number of parameter-update iterations per template.
    pub parameter_updates: u64,

    /// Number of shots per parameterized circuit execution.
    pub shots: u64,

    /// Number of benchmark layers represented by each circuit.
    pub layers: u64,

    /// Total elapsed benchmark duration.
    pub elapsed: Duration,
}

impl ClopsWorkload {
    /// Creates a CLOPS workload.
    ///
    /// All multiplicative workload dimensions must be non-zero.
    pub fn new(
        templates: u64,
        parameter_updates: u64,
        shots: u64,
        layers: u64,
        elapsed: Duration,
    ) -> Result<Self, ThroughputError> {
        if templates == 0 {
            return Err(ThroughputError::ZeroTemplates);
        }

        if parameter_updates == 0 {
            return Err(ThroughputError::ZeroParameterUpdates);
        }

        if shots == 0 {
            return Err(ThroughputError::ZeroShots);
        }

        if layers == 0 {
            return Err(ThroughputError::ZeroLayers);
        }

        validate_duration(elapsed)?;

        Ok(Self {
            templates,
            parameter_updates,
            shots,
            layers,
            elapsed,
        })
    }

    /// Returns the total logical circuit executions represented by the
    /// workload.
    ///
    /// Uses checked integer arithmetic so malformed or extreme input cannot
    /// wrap silently.
    pub fn circuit_executions(&self) -> Result<u64, ThroughputError> {
        let value = self
            .templates
            .checked_mul(self.parameter_updates)
            .ok_or(ThroughputError::WorkloadOverflow)?;

        value
            .checked_mul(self.shots)
            .ok_or(ThroughputError::WorkloadOverflow)
    }

    /// Returns the total number of circuit layers represented by the
    /// workload.
    pub fn layer_operations(&self) -> Result<u64, ThroughputError> {
        let executions = self.circuit_executions()?;

        executions
            .checked_mul(self.layers)
            .ok_or(ThroughputError::WorkloadOverflow)
    }

    /// Calculates the CLOPS value.
    ///
    /// Floating-point multiplication is used after each checked integer
    /// workload component has been validated. This prevents integer overflow
    /// while retaining exact representation for all values representable by
    /// `f64`.
    pub fn rate(&self) -> Result<f64, ThroughputError> {
        let layers = self.layer_operations()?;

        calculate_rate(layers, self.elapsed)
    }

    /// Converts the workload into a canonical Zamani metric.
    pub fn metric(&self) -> Result<Metric, ThroughputError> {
        let rate = self.rate()?;

        Metric::new(
            MetricKind::LayersPerSecond,
            MetricUnit::Hertz,
            rate,
        )?
        .with_sample_count(self.templates)
        .map_err(ThroughputError::Metric)
    }
}

/// A maximum-circuits-per-second style workload.
///
/// Unlike CLOPS, MCPS is intentionally a direct circuit-throughput measure.
/// It should normally include the timing boundary defined by the benchmark
/// protocol, including whatever measurement/reset/reinitialization phases the
/// protocol specifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McpsWorkload {
    /// Number of completed circuits.
    pub circuits: u64,

    /// Total elapsed time.
    pub elapsed: Duration,
}

impl McpsWorkload {
    /// Creates an MCPS workload.
    pub fn new(
        circuits: u64,
        elapsed: Duration,
    ) -> Result<Self, ThroughputError> {
        let observation = ThroughputObservation::new(circuits, elapsed)?;

        Ok(Self {
            circuits: observation.count,
            elapsed: observation.elapsed,
        })
    }

    /// Calculates maximum circuits per second for the measured workload.
    pub fn rate(&self) -> Result<f64, ThroughputError> {
        calculate_rate(self.circuits, self.elapsed)
    }

    /// Converts the result to the canonical circuits/sec metric.
    pub fn metric(&self) -> Result<Metric, ThroughputError> {
        let rate = self.rate()?;

        Metric::new(
            MetricKind::CircuitsPerSecond,
            MetricUnit::Hertz,
            rate,
        )?
        .with_sample_count(self.circuits)
        .map_err(ThroughputError::Metric)
    }
}

/// A collection of independent throughput observations.
///
/// This is useful when a benchmark executes multiple repeated runs and wants
/// to preserve every observation before calculating summary statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThroughputSeries {
    /// Throughput dimension represented by all observations.
    pub kind: ThroughputKind,

    /// Individual observations.
    observations: Vec<ThroughputObservation>,
}

impl ThroughputSeries {
    /// Creates an empty throughput series.
    pub fn new(kind: ThroughputKind) -> Self {
        Self {
            kind,
            observations: Vec::new(),
        }
    }

    /// Adds an observation.
    pub fn push(
        &mut self,
        observation: ThroughputObservation,
    ) {
        self.observations.push(observation);
    }

    /// Returns the number of observations.
    #[inline]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the series contains no observations.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Returns the observations.
    #[inline]
    pub fn observations(&self) -> &[ThroughputObservation] {
        &self.observations
    }

    /// Calculates every observation's rate without discarding failures.
    pub fn rates(&self) -> Result<Vec<f64>, ThroughputError> {
        let mut rates = Vec::with_capacity(self.observations.len());

        for observation in &self.observations {
            rates.push(observation.rate_per_second()?);
        }

        Ok(rates)
    }

    /// Calculates the arithmetic mean throughput.
    ///
    /// This is a summary statistic only. It does not replace the individual
    /// observations and should not be confused with aggregate workload/time
    /// throughput.
    pub fn mean_rate(&self) -> Result<f64, ThroughputError> {
        let rates = self.rates()?;

        if rates.is_empty() {
            return Err(ThroughputError::InsufficientObservations);
        }

        let mut mean = 0.0;

        for (index, rate) in rates.iter().enumerate() {
            let weight = 1.0 / (index as f64 + 1.0);
            mean += (*rate - mean) * weight;
        }

        ensure_finite(mean)?;

        Ok(mean)
    }

    /// Calculates the median throughput.
    pub fn median_rate(&self) -> Result<f64, ThroughputError> {
        let mut rates = self.rates()?;

        if rates.is_empty() {
            return Err(ThroughputError::InsufficientObservations);
        }

        rates.sort_by(f64::total_cmp);

        let middle = rates.len() / 2;

        let median = if rates.len() % 2 == 0 {
            (rates[middle - 1] + rates[middle]) / 2.0
        } else {
            rates[middle]
        };

        ensure_finite(median)?;

        Ok(median)
    }

    /// Calculates the sample standard deviation of throughput.
    ///
    /// Returns zero for a single observation.
    pub fn standard_deviation(&self) -> Result<f64, ThroughputError> {
        let rates = self.rates()?;

        if rates.is_empty() {
            return Err(ThroughputError::InsufficientObservations);
        }

        if rates.len() == 1 {
            return Ok(0.0);
        }

        let mean = calculate_mean(&rates)?;

        let mut squared_error = 0.0;

        for rate in &rates {
            let delta = *rate - mean;
            squared_error += delta * delta;
        }

        let variance = squared_error / (rates.len() as f64 - 1.0);
        ensure_finite(variance)?;

        let deviation = variance.sqrt();
        ensure_finite(deviation)?;

        Ok(deviation)
    }

    /// Produces a canonical metric using the mean of repeated observations.
    ///
    /// The standard deviation is attached as the metric's uncertainty.
    pub fn mean_metric(&self) -> Result<Metric, ThroughputError> {
        let mean = self.mean_rate()?;
        let deviation = self.standard_deviation()?;

        Metric::new(
            self.kind.metric_kind(),
            MetricUnit::Hertz,
            mean,
        )?
        .with_uncertainty(deviation)
        .map_err(ThroughputError::Metric)
    }

    /// Produces a canonical metric using the median of repeated observations.
    pub fn median_metric(&self) -> Result<Metric, ThroughputError> {
        let median = self.median_rate()?;

        Metric::new(
            self.kind.metric_kind(),
            MetricUnit::Hertz,
            median,
        )
        .map_err(ThroughputError::Metric)
    }
}

/// Calculates throughput for an arbitrary count and elapsed duration.
///
/// This is the lowest-level calculation used by all ordinary throughput
/// metrics.
///
/// ```text
/// rate = count / elapsed_seconds
/// ```
pub fn calculate_rate(
    count: u64,
    elapsed: Duration,
) -> Result<f64, ThroughputError> {
    if count == 0 {
        return Err(ThroughputError::ZeroWork);
    }

    validate_duration(elapsed)?;

    let seconds = elapsed.as_secs_f64();

    if !seconds.is_finite() || seconds <= 0.0 {
        return Err(ThroughputError::InvalidElapsedTime);
    }

    let rate = count as f64 / seconds;

    ensure_finite(rate)?;

    if rate <= 0.0 {
        return Err(ThroughputError::InvalidCalculatedRate);
    }

    Ok(rate)
}

/// Calculates shots per second.
pub fn shots_per_second(
    shots: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::ShotsPerSecond,
        shots,
        elapsed,
    )
}

/// Calculates circuits per second.
pub fn circuits_per_second(
    circuits: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::CircuitsPerSecond,
        circuits,
        elapsed,
    )
}

/// Calculates gates per second.
pub fn gates_per_second(
    gates: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::GatesPerSecond,
        gates,
        elapsed,
    )
}

/// Calculates two-qubit gates per second.
pub fn two_qubit_gates_per_second(
    gates: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::TwoQubitGatesPerSecond,
        gates,
        elapsed,
    )
}

/// Calculates circuit layers per second.
pub fn layers_per_second(
    layers: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::LayersPerSecond,
        layers,
        elapsed,
    )
}

/// Calculates generic operations per second.
pub fn operations_per_second(
    operations: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::Throughput,
        operations,
        elapsed,
    )
}

/// Calculates generic throughput from an observation.
pub fn throughput(
    observation: ThroughputObservation,
) -> Result<Metric, ThroughputError> {
    metric_from_count(
        MetricKind::Throughput,
        observation.count,
        observation.elapsed,
    )
}

/// Converts a count/time pair to a canonical metric.
///
/// All rate units are represented using `MetricUnit::Hertz`, which is the
/// repository's existing canonical unit for quantities measured per second.
fn metric_from_count(
    kind: MetricKind,
    count: u64,
    elapsed: Duration,
) -> Result<Metric, ThroughputError> {
    let rate = calculate_rate(count, elapsed)?;

    let metric = Metric::new(
        kind,
        MetricUnit::Hertz,
        rate,
    )?
    .with_sample_count(count)
    .map_err(ThroughputError::Metric)?;

    Ok(metric)
}

/// Validates a duration used as a throughput denominator.
fn validate_duration(
    elapsed: Duration,
) -> Result<(), ThroughputError> {
    if elapsed < MIN_ELAPSED_TIME {
        return Err(ThroughputError::InvalidElapsedTime);
    }

    let seconds = elapsed.as_secs_f64();

    if !seconds.is_finite() || seconds <= 0.0 {
        return Err(ThroughputError::InvalidElapsedTime);
    }

    Ok(())
}

/// Ensures a floating-point result is finite.
fn ensure_finite(value: f64) -> Result<(), ThroughputError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ThroughputError::NonFiniteResult)
    }
}

/// Validates a normalized quality value.
fn validate_unit_interval(
    value: f64,
    name: &'static str,
) -> Result<(), ThroughputError> {
    if !value.is_finite() {
        return Err(ThroughputError::NonFiniteInput {
            field: name,
            value,
        });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(ThroughputError::OutOfRange {
            field: name,
            value,
            minimum: 0.0,
            maximum: 1.0,
        });
    }

    Ok(())
}

/// Calculates a numerically stable arithmetic mean.
///
/// This helper uses incremental averaging so a large number of observations
/// does not require summing all values into one potentially overflowing
/// intermediate floating-point value.
fn calculate_mean(values: &[f64]) -> Result<f64, ThroughputError> {
    if values.is_empty() {
        return Err(ThroughputError::InsufficientObservations);
    }

    let mut mean = 0.0;

    for (index, value) in values.iter().enumerate() {
        ensure_finite(*value)?;

        let weight = 1.0 / (index as f64 + 1.0);
        mean += (*value - mean) * weight;
    }

    ensure_finite(mean)?;

    Ok(mean)
}

/// Errors produced by throughput calculation and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum ThroughputError {
    /// No workload was completed.
    ZeroWork,

    /// Elapsed time is zero, invalid, or cannot be represented as a positive
    /// finite number of seconds.
    InvalidElapsedTime,

    /// Calculated throughput is not finite.
    NonFiniteResult,

    /// Calculated throughput is not positive.
    InvalidCalculatedRate,

    /// Input floating-point value is NaN or infinity.
    NonFiniteInput {
        /// Semantic input field name.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// Floating-point value lies outside an allowed interval.
    OutOfRange {
        /// Semantic input field name.
        field: &'static str,

        /// Supplied value.
        value: f64,

        /// Minimum allowed value.
        minimum: f64,

        /// Maximum allowed value.
        maximum: f64,
    },

    /// No observations are available for an aggregate statistic.
    InsufficientObservations,

    /// Number of CLOPS templates is zero.
    ZeroTemplates,

    /// Number of CLOPS parameter updates is zero.
    ZeroParameterUpdates,

    /// Number of CLOPS shots is zero.
    ZeroShots,

    /// Number of CLOPS layers is zero.
    ZeroLayers,

    /// CLOPS workload multiplication would overflow `u64`.
    WorkloadOverflow,

    /// A canonical metric could not be created.
    Metric(MetricError),
}

impl fmt::Display for ThroughputError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroWork => {
                write!(
                    formatter,
                    "throughput requires at least one completed workload item"
                )
            }

            Self::InvalidElapsedTime => {
                write!(
                    formatter,
                    "throughput requires a positive finite elapsed duration"
                )
            }

            Self::NonFiniteResult => {
                write!(
                    formatter,
                    "calculated throughput is not finite"
                )
            }

            Self::InvalidCalculatedRate => {
                write!(
                    formatter,
                    "calculated throughput must be greater than zero"
                )
            }

            Self::NonFiniteInput { field, value } => {
                write!(
                    formatter,
                    "{field} must be finite, received {value}"
                )
            }

            Self::OutOfRange {
                field,
                value,
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} must be in [{minimum}, {maximum}], received {value}"
                )
            }

            Self::InsufficientObservations => {
                write!(
                    formatter,
                    "at least one throughput observation is required"
                )
            }

            Self::ZeroTemplates => {
                write!(
                    formatter,
                    "CLOPS requires at least one circuit template"
                )
            }

            Self::ZeroParameterUpdates => {
                write!(
                    formatter,
                    "CLOPS requires at least one parameter update"
                )
            }

            Self::ZeroShots => {
                write!(
                    formatter,
                    "CLOPS requires at least one shot"
                )
            }

            Self::ZeroLayers => {
                write!(
                    formatter,
                    "CLOPS requires at least one circuit layer"
                )
            }

            Self::WorkloadOverflow => {
                write!(
                    formatter,
                    "CLOPS workload size exceeds the supported u64 range"
                )
            }

            Self::Metric(error) => {
                write!(
                    formatter,
                    "unable to construct throughput metric: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ThroughputError {}

/// Allows callers to inspect the underlying canonical metric error.
impl From<MetricError> for ThroughputError {
    fn from(error: MetricError) -> Self {
        Self::Metric(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_shots_per_second() {
        let metric = shots_per_second(
            1_000,
            Duration::from_secs(2),
        )
        .expect("valid throughput");

        assert_eq!(
            metric.kind,
            MetricKind::ShotsPerSecond
        );

        assert!((metric.value.get() - 500.0).abs() < 1e-12);

        assert_eq!(
            metric.unit,
            MetricUnit::Hertz
        );

        assert_eq!(
            metric.sample_count,
            Some(1_000)
        );
    }

    #[test]
    fn calculates_circuits_per_second() {
        let metric = circuits_per_second(
            250,
            Duration::from_secs(5),
        )
        .expect("valid throughput");

        assert!((metric.value.get() - 50.0).abs() < 1e-12);
    }

    #[test]
    fn calculates_gates_per_second() {
        let metric = gates_per_second(
            10_000,
            Duration::from_secs(4),
        )
        .expect("valid throughput");

        assert!((metric.value.get() - 2_500.0).abs() < 1e-12);
    }

    #[test]
    fn calculates_two_qubit_gates_per_second() {
        let metric = two_qubit_gates_per_second(
            800,
            Duration::from_secs(2),
        )
        .expect("valid throughput");

        assert!((metric.value.get() - 400.0).abs() < 1e-12);
    }

    #[test]
    fn calculates_layers_per_second() {
        let metric = layers_per_second(
            1_200,
            Duration::from_secs(3),
        )
        .expect("valid throughput");

        assert!((metric.value.get() - 400.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_zero_work() {
        let result = circuits_per_second(
            0,
            Duration::from_secs(1),
        );

        assert_eq!(
            result,
            Err(ThroughputError::ZeroWork)
        );
    }

    #[test]
    fn rejects_zero_duration() {
        let result = circuits_per_second(
            1,
            Duration::ZERO,
        );

        assert_eq!(
            result,
            Err(ThroughputError::InvalidElapsedTime)
        );
    }

    #[test]
    fn rejects_sub_nanosecond_duration_boundary() {
        // Rust Duration cannot represent fractions below one nanosecond, so
        // Duration::ZERO is the effective minimum invalid representation.
        let result = circuits_per_second(
            1,
            Duration::ZERO,
        );

        assert_eq!(
            result,
            Err(ThroughputError::InvalidElapsedTime)
        );
    }

    #[test]
    fn generic_observation_calculates_rate() {
        let observation = ThroughputObservation::new(
            10,
            Duration::from_secs(2),
        )
        .expect("valid observation");

        assert!((observation.rate_per_second().unwrap() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn clops_calculation_is_correct() {
        let workload = ClopsWorkload::new(
            100,
            10,
            100,
            5,
            Duration::from_secs(10),
        )
        .expect("valid CLOPS workload");

        // 100 * 10 * 100 * 5 / 10 = 5,000.
        let rate = workload.rate().expect("valid CLOPS");

        assert!((rate - 5_000.0).abs() < 1e-12);
    }

    #[test]
    fn clops_metric_uses_layer_throughput_kind() {
        let workload = ClopsWorkload::new(
            100,
            10,
            100,
            5,
            Duration::from_secs(10),
        )
        .expect("valid CLOPS workload");

        let metric = workload.metric().expect("valid metric");

        assert_eq!(
            metric.kind,
            MetricKind::LayersPerSecond
        );

        assert!((metric.value.get() - 5_000.0).abs() < 1e-12);
    }

    #[test]
    fn clops_reports_circuit_executions() {
        let workload = ClopsWorkload::new(
            100,
            10,
            100,
            5,
            Duration::from_secs(10),
        )
        .expect("valid CLOPS workload");

        assert_eq!(
            workload.circuit_executions().unwrap(),
            100_000
        );

        assert_eq!(
            workload.layer_operations().unwrap(),
            500_000
        );
    }

    #[test]
    fn clops_rejects_overflow() {
        let workload = ClopsWorkload::new(
            u64::MAX,
            2,
            1,
            1,
            Duration::from_secs(1),
        )
        .expect("individual fields are valid");

        assert_eq!(
            workload.circuit_executions(),
            Err(ThroughputError::WorkloadOverflow)
        );
    }

    #[test]
    fn mcps_is_circuit_throughput() {
        let workload = McpsWorkload::new(
            2_000,
            Duration::from_secs(4),
        )
        .expect("valid MCPS workload");

        assert!((workload.rate().unwrap() - 500.0).abs() < 1e-12);

        let metric = workload.metric().expect("valid metric");

        assert_eq!(
            metric.kind,
            MetricKind::CircuitsPerSecond
        );
    }

    #[test]
    fn quality_envelope_accepts_threshold() {
        let envelope =
            QualityEnvelope::new(0.90)
                .expect("valid quality envelope");

        assert!(
            envelope.accepts(0.90).unwrap()
        );

        assert!(
            envelope.accepts(0.95).unwrap()
        );

        assert!(
            !envelope.accepts(0.89).unwrap()
        );
    }

    #[test]
    fn quality_gated_throughput_does_not_modify_rate() {
        let throughput =
            ThroughputMeasurement::from_observation(
                ThroughputKind::CircuitsPerSecond,
                ThroughputObservation::new(
                    1_000,
                    Duration::from_secs(2),
                )
                .unwrap(),
            )
            .unwrap();

        let gated = QualityGatedThroughput::new(
            throughput,
            QualityEnvelope::new(0.90).unwrap(),
            0.95,
        )
        .unwrap();

        assert!(gated.accepted);

        assert!(
            (gated.throughput.rate_per_second() - 500.0).abs()
                < 1e-12
        );
    }

    #[test]
    fn rejects_invalid_quality() {
        assert_eq!(
            QualityEnvelope::new(1.1),
            Err(ThroughputError::OutOfRange {
                field: "quality-envelope minimum",
                value: 1.1,
                minimum: 0.0,
                maximum: 1.0,
            })
        );
    }

    #[test]
    fn series_mean_is_correct() {
        let mut series =
            ThroughputSeries::new(
                ThroughputKind::CircuitsPerSecond,
            );

        series.push(
            ThroughputObservation::new(
                100,
                Duration::from_secs(1),
            )
            .unwrap(),
        );

        series.push(
            ThroughputObservation::new(
                200,
                Duration::from_secs(2),
            )
            .unwrap(),
        );

        // Both observations equal 100 circuits/sec.
        assert!(
            (series.mean_rate().unwrap() - 100.0).abs()
                < 1e-12
        );
    }

    #[test]
    fn series_median_is_correct() {
        let mut series =
            ThroughputSeries::new(
                ThroughputKind::CircuitsPerSecond,
            );

        series.push(
            ThroughputObservation::new(
                100,
                Duration::from_secs(1),
            )
            .unwrap(),
        );

        series.push(
            ThroughputObservation::new(
                300,
                Duration::from_secs(1),
            )
            .unwrap(),
        );

        series.push(
            ThroughputObservation::new(
                200,
                Duration::from_secs(1),
            )
            .unwrap(),
        );

        assert!(
            (series.median_rate().unwrap() - 200.0).abs()
                < 1e-12
        );
    }

    #[test]
    fn series_standard_deviation_is_zero_for_constant_rate() {
        let mut series =
            ThroughputSeries::new(
                ThroughputKind::CircuitsPerSecond,
            );

        series.push(
            ThroughputObservation::new(
                100,
                Duration::from_secs(1),
            )
            .unwrap(),
        );

        series.push(
            ThroughputObservation::new(
                200,
                Duration::from_secs(2),
            )
            .unwrap(),
        );

        assert!(
            series.standard_deviation().unwrap().abs()
                < 1e-12
        );
    }

    #[test]
    fn empty_series_is_rejected() {
        let series =
            ThroughputSeries::new(
                ThroughputKind::CircuitsPerSecond,
            );

        assert_eq!(
            series.mean_rate(),
            Err(ThroughputError::InsufficientObservations)
        );

        assert_eq!(
            series.median_rate(),
            Err(ThroughputError::InsufficientObservations)
        );

        assert_eq!(
            series.standard_deviation(),
            Err(ThroughputError::InsufficientObservations)
        );
    }

    #[test]
    fn metric_is_valid_under_core_metric_contract() {
        let metric = gates_per_second(
            1_000,
            Duration::from_secs(1),
        )
        .expect("valid metric");

        metric
            .validate()
            .expect("throughput metric must validate");
    }

    #[test]
    fn throughput_kind_ids_are_stable() {
        assert_eq!(
            ThroughputKind::ShotsPerSecond.id(),
            "shots_per_second"
        );

        assert_eq!(
            ThroughputKind::CircuitsPerSecond.id(),
            "circuits_per_second"
        );

        assert_eq!(
            ThroughputKind::GatesPerSecond.id(),
            "gates_per_second"
        );

        assert_eq!(
            ThroughputKind::TwoQubitGatesPerSecond.id(),
            "two_qubit_gates_per_second"
        );

        assert_eq!(
            ThroughputKind::LayersPerSecond.id(),
            "layers_per_second"
        );
    }
}