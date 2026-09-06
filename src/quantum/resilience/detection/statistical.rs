//! Streaming statistical detection primitives for Zamani quantum resilience.
//!
//! # Purpose
//!
//! This module provides backend-independent, bounded-memory statistical
//! detectors for continuously observed quantum-system metrics.
//!
//! It is intentionally independent from:
//!
//! - quantum::ir
//! - quantum::ir::qubit
//! - hardware providers
//! - QEC implementations
//! - routing
//! - scheduling
//! - optimization
//! - ZQN fault semantics
//! - recovery
//!
//! Those systems provide observations and consume the normalized statistical
//! evidence emitted here.
//!
//! # Design goals
//!
//! - Rust 1.97 / 1.97.1 compatible.
//! - No `unsafe`.
//! - O(1) memory per detector.
//! - No fixed number of qubits, devices, shots, samples, or backends.
//! - Streaming operation.
//! - Deterministic operation.
//! - Explicit numerical validation.
//! - Explicit configuration; no hidden thresholds.
//! - Suitable for tiny systems and very large distributed systems.
//! - No provider-specific behavior.
//! - No automatic recovery.
//! - No semantic acceptance of quantum results.
//!
//! # Important semantic boundary
//!
//! A statistical detector produces *evidence*, not a final diagnosis.
//!
//! ```text
//! observation
//!     |
//!     v
//! statistical detector
//!     |
//!     v
//! statistical evidence
//!     |
//!     +----> diagnosis
//!     |
//!     +----> threshold/anomaly detector
//!     |
//!     +----> incident correlation
//! ```
//!
//! The detector must never directly retry, reroute, recompile, migrate,
//! modify QEC, or accept/reject a quantum result.
//!
//! # Resource identity
//!
//! This module intentionally does not define a second `QubitId` type and does
//! not require `quantum::ir::qubit::QubitId`.
//!
//! A caller that is monitoring a specific logical or physical qubit should
//! associate the resulting evidence with the canonical resource identity in
//! the surrounding detection/model layer.
//!
//! # Numerical policy
//!
//! NaN and infinite floating-point observations are rejected. Silently
//! treating NaN as a normal observation is unsafe for a resilience system.
//!
//! Floating-point calculations use checked operations where overflow,
//! underflow, or non-finite results could otherwise silently poison the
//! detector state.
//!
//! # Algorithms
//!
//! This module provides:
//!
//! - Welford online mean/variance.
//! - EWMA.
//! - Standardized deviation scoring.
//! - One-sided CUSUM.
//! - Two-sided CUSUM.
//! - Configurable consecutive evidence.
//!
//! These are deliberately separate detectors. The module does not pretend
//! that one statistical method is universally appropriate for all quantum
//! telemetry.

use core::fmt;
use core::num::NonZeroU64;

/// Stable identifier for a statistical detector.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatisticalDetectorId(String);

impl StatisticalDetectorId {
    /// Creates a detector identifier.
    ///
    /// Empty identifiers are rejected because detector identity is part of
    /// provenance and correlation.
    pub fn new(value: impl Into<String>) -> Result<Self, StatisticalError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StatisticalError::InvalidConfiguration(
                "detector identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StatisticalDetectorId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Identifier of the metric being monitored.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatisticalMetricId(String);

impl StatisticalMetricId {
    /// Creates a metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StatisticalError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(StatisticalError::InvalidConfiguration(
                "metric identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the metric identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StatisticalMetricId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A validated finite statistical observation.
///
/// The detector operates on scalar measurements. Resource identity,
/// timestamps, topology, and quantum-specific meaning belong to the
/// surrounding observation model.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StatisticalObservation {
    /// Metric value.
    pub value: f64,

    /// Monotonic sequence number supplied by the observation source.
    pub sequence: u64,
}

impl StatisticalObservation {
    /// Creates a validated observation.
    pub fn new(value: f64, sequence: u64) -> Result<Self, StatisticalError> {
        validate_finite(value)?;

        Ok(Self { value, sequence })
    }
}

/// Policy governing observation sequence ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SequencePolicy {
    /// A sequence number must be greater than the previous one.
    StrictlyIncreasing,

    /// Equal sequence numbers are allowed, which is useful for deterministic
    /// replay and idempotent telemetry delivery.
    NonDecreasing,
}

/// Numerical result of a statistical test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StatisticalScore {
    /// Raw observation.
    pub value: f64,

    /// Statistical score.
    ///
    /// The meaning depends on the detector:
    ///
    /// - Welford: standardized z-score when variance is available.
    /// - EWMA: normalized deviation when a baseline is supplied.
    /// - CUSUM: accumulated score.
    pub score: f64,

    /// Number of observations incorporated by the detector.
    pub sample_count: u64,
}

/// Classification of detector output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatisticalEventKind {
    /// There is insufficient evidence to classify the observation.
    InsufficientEvidence,

    /// Observation is statistically normal under the configured detector.
    Normal,

    /// Observation exceeded the configured statistical criterion.
    Anomaly,

    /// The detector crossed its configured change threshold.
    ChangeDetected,

    /// A previously active change condition has returned to normal.
    ChangeCleared,
}

/// Statistical evidence emitted by a detector.
#[derive(Clone, Debug, PartialEq)]
pub struct StatisticalEvidence {
    /// Detector identity.
    pub detector_id: StatisticalDetectorId,

    /// Monitored metric.
    pub metric_id: StatisticalMetricId,

    /// Observation sequence.
    pub sequence: u64,

    /// Original observation.
    pub observation: f64,

    /// Statistical score.
    pub score: f64,

    /// Number of samples incorporated.
    pub sample_count: u64,

    /// Event classification.
    pub kind: StatisticalEventKind,

    /// Whether the detector was active before this observation.
    pub previously_active: bool,

    /// Whether the detector is active after this observation.
    pub active: bool,
}

/// Common detector interface.
///
/// Implementations must maintain bounded memory and must not perform
/// recovery or hardware operations.
pub trait StatisticalDetector {
    /// Processes one observation.
    fn observe(
        &mut self,
        observation: StatisticalObservation,
    ) -> Result<StatisticalEvidence, StatisticalError>;

    /// Returns the detector identity.
    fn detector_id(&self) -> &StatisticalDetectorId;

    /// Returns the metric identity.
    fn metric_id(&self) -> &StatisticalMetricId;

    /// Resets detector state.
    fn reset(&mut self);
}

/// Running mean/variance state using Welford's numerically stable algorithm.
///
/// Memory is constant regardless of the number of observations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunningStatistics {
    count: u64,
    mean: f64,
    m2: f64,
}

impl RunningStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    /// Number of observations.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Current mean.
    #[must_use]
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    /// Population variance.
    ///
    /// Returns `None` until at least one observation exists.
    #[must_use]
    pub fn population_variance(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.m2 / self.count as f64)
        }
    }

    /// Sample variance.
    ///
    /// Returns `None` until at least two observations exist.
    #[must_use]
    pub fn sample_variance(&self) -> Option<f64> {
        if self.count < 2 {
            None
        } else {
            Some(self.m2 / (self.count - 1) as f64)
        }
    }

    /// Population standard deviation.
    #[must_use]
    pub fn population_standard_deviation(&self) -> Option<f64> {
        self.population_variance().map(f64::sqrt)
    }

    /// Sample standard deviation.
    #[must_use]
    pub fn sample_standard_deviation(&self) -> Option<f64> {
        self.sample_variance().map(f64::sqrt)
    }

    /// Adds one observation.
    pub fn update(&mut self, value: f64) -> Result<(), StatisticalError> {
        validate_finite(value)?;

        if self.count == u64::MAX {
            return Err(StatisticalError::CounterOverflow);
        }

        let next_count = self.count + 1;

        let delta = value - self.mean;

        let next_mean = if self.count == 0 {
            value
        } else {
            let denominator = next_count as f64;

            if !denominator.is_finite() || denominator == 0.0 {
                return Err(StatisticalError::NumericFailure(
                    "invalid running-statistics denominator",
                ));
            }

            self.mean + delta / denominator
        };

        let delta_after = value - next_mean;
        let next_m2 = self.m2 + delta * delta_after;

        if !next_mean.is_finite() || !next_m2.is_finite() {
            return Err(StatisticalError::NumericFailure(
                "running statistics became non-finite",
            ));
        }

        self.count = next_count;
        self.mean = next_mean;
        self.m2 = if next_m2 < 0.0 && next_m2 > -f64::EPSILON {
            0.0
        } else {
            next_m2
        };

        Ok(())
    }

    /// Calculates the z-score relative to the current state.
    ///
    /// The current observation is not included in the calculation.
    #[must_use]
    pub fn z_score_against_current(&self, value: f64) -> Option<f64> {
        if self.count < 2 {
            return None;
        }

        let deviation = self.sample_standard_deviation()?;

        if deviation == 0.0 {
            if value == self.mean {
                Some(0.0)
            } else if value > self.mean {
                Some(f64::INFINITY)
            } else {
                Some(f64::NEG_INFINITY)
            }
        } else {
            Some((value - self.mean) / deviation)
        }
    }
}

impl Default for RunningStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// EWMA configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EwmaConfig {
    /// Smoothing coefficient.
    ///
    /// Must satisfy `0 < alpha <= 1`.
    pub alpha: f64,
}

impl EwmaConfig {
    /// Creates EWMA configuration.
    pub fn new(alpha: f64) -> Result<Self, StatisticalError> {
        validate_finite(alpha)?;

        if alpha <= 0.0 || alpha > 1.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "EWMA alpha must satisfy 0 < alpha <= 1",
            ));
        }

        Ok(Self { alpha })
    }
}

/// Streaming exponentially weighted moving average.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ewma {
    config: EwmaConfig,
    value: Option<f64>,
    count: u64,
}

impl Ewma {
    /// Creates an empty EWMA.
    #[must_use]
    pub const fn new(config: EwmaConfig) -> Self {
        Self {
            config,
            value: None,
            count: 0,
        }
    }

    /// Returns the current EWMA.
    #[must_use]
    pub const fn value(&self) -> Option<f64> {
        self.value
    }

    /// Returns the number of observations incorporated.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Updates the EWMA.
    pub fn update(&mut self, observation: f64) -> Result<f64, StatisticalError> {
        validate_finite(observation)?;

        if self.count == u64::MAX {
            return Err(StatisticalError::CounterOverflow);
        }

        let next = match self.value {
            None => observation,
            Some(previous) => {
                let delta = observation - previous;
                previous + self.config.alpha * delta
            }
        };

        if !next.is_finite() {
            return Err(StatisticalError::NumericFailure(
                "EWMA became non-finite",
            ));
        }

        self.value = Some(next);
        self.count += 1;

        Ok(next)
    }

    /// Resets the state while retaining configuration.
    pub const fn reset(&mut self) {
        self.value = None;
        self.count = 0;
    }
}

/// Direction of a CUSUM detector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CusumDirection {
    /// Detect upward changes.
    Increase,

    /// Detect downward changes.
    Decrease,

    /// Detect either direction.
    Both,
}

/// CUSUM configuration.
///
/// The detector compares observations against a reference mean. The
/// reference mean is supplied explicitly rather than being silently inferred,
/// because silently changing the reference model can hide real drift.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CusumConfig {
    /// Reference mean.
    pub reference_mean: f64,

    /// Allowed per-observation deviation before evidence accumulates.
    ///
    /// Must be finite and non-negative.
    pub allowance: f64,

    /// Detection threshold.
    ///
    /// Must be finite and strictly positive.
    pub threshold: f64,

    /// Direction of change to detect.
    pub direction: CusumDirection,
}

impl CusumConfig {
    /// Creates a validated CUSUM configuration.
    pub fn new(
        reference_mean: f64,
        allowance: f64,
        threshold: f64,
        direction: CusumDirection,
    ) -> Result<Self, StatisticalError> {
        validate_finite(reference_mean)?;
        validate_finite(allowance)?;
        validate_finite(threshold)?;

        if allowance < 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "CUSUM allowance must not be negative",
            ));
        }

        if threshold <= 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "CUSUM threshold must be positive",
            ));
        }

        Ok(Self {
            reference_mean,
            allowance,
            threshold,
            direction,
        })
    }
}

/// Streaming CUSUM detector state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cusum {
    config: CusumConfig,
    positive: f64,
    negative: f64,
    count: u64,
    active: bool,
}

impl Cusum {
    /// Creates a CUSUM detector.
    #[must_use]
    pub const fn new(config: CusumConfig) -> Self {
        Self {
            config,
            positive: 0.0,
            negative: 0.0,
            count: 0,
            active: false,
        }
    }

    /// Returns positive cumulative evidence.
    #[must_use]
    pub const fn positive_score(&self) -> f64 {
        self.positive
    }

    /// Returns negative cumulative evidence.
    #[must_use]
    pub const fn negative_score(&self) -> f64 {
        self.negative
    }

    /// Returns whether the detector is currently active.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    /// Returns number of processed observations.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Processes an observation.
    ///
    /// Once a threshold is crossed, the corresponding CUSUM accumulator is
    /// reset. This permits continued monitoring without unbounded numerical
    /// growth.
    pub fn update(&mut self, value: f64) -> Result<CusumUpdate, StatisticalError> {
        validate_finite(value)?;

        if self.count == u64::MAX {
            return Err(StatisticalError::CounterOverflow);
        }

        self.count += 1;

        let deviation = value - self.config.reference_mean;

        let positive_candidate = (self.positive + deviation - self.config.allowance).max(0.0);
        let negative_candidate = (self.negative - deviation - self.config.allowance).max(0.0);

        if !positive_candidate.is_finite() || !negative_candidate.is_finite() {
            return Err(StatisticalError::NumericFailure(
                "CUSUM state became non-finite",
            ));
        }

        self.positive = positive_candidate;
        self.negative = negative_candidate;

        let previous_active = self.active;

        let positive_crossed = self.positive >= self.config.threshold;
        let negative_crossed = self.negative >= self.config.threshold;

        let changed = match self.config.direction {
            CusumDirection::Increase => positive_crossed,
            CusumDirection::Decrease => negative_crossed,
            CusumDirection::Both => positive_crossed || negative_crossed,
        };

        if changed {
            self.active = true;

            if positive_crossed {
                self.positive = 0.0;
            }

            if negative_crossed {
                self.negative = 0.0;
            }
        } else if self.active {
            self.active = false;
        }

        let event = if !previous_active && self.active {
            StatisticalEventKind::ChangeDetected
        } else if previous_active && !self.active {
            StatisticalEventKind::ChangeCleared
        } else if self.active {
            StatisticalEventKind::ChangeDetected
        } else {
            StatisticalEventKind::Normal
        };

        Ok(CusumUpdate {
            positive_score: self.positive,
            negative_score: self.negative,
            event,
            active: self.active,
        })
    }

    /// Resets accumulated evidence.
    pub const fn reset(&mut self) {
        self.positive = 0.0;
        self.negative = 0.0;
        self.count = 0;
        self.active = false;
    }
}

/// Result of one CUSUM update.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CusumUpdate {
    /// Current positive cumulative score.
    pub positive_score: f64,

    /// Current negative cumulative score.
    pub negative_score: f64,

    /// Resulting event.
    pub event: StatisticalEventKind,

    /// Whether the detector is active.
    pub active: bool,
}

/// Configuration for a streaming standardized statistical detector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardScoreConfig {
    /// Absolute score at or above which an anomaly is emitted.
    pub threshold: f64,

    /// Minimum number of baseline observations required before a score is
    /// considered meaningful.
    pub minimum_samples: NonZeroU64,

    /// Whether the baseline should continue learning while monitoring.
    pub update_baseline: bool,
}

impl StandardScoreConfig {
    /// Creates standardized-score configuration.
    pub fn new(
        threshold: f64,
        minimum_samples: NonZeroU64,
        update_baseline: bool,
    ) -> Result<Self, StatisticalError> {
        validate_finite(threshold)?;

        if threshold <= 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "standard-score threshold must be positive",
            ));
        }

        Ok(Self {
            threshold,
            minimum_samples,
            update_baseline,
        })
    }
}

/// Streaming standardized-score detector.
#[derive(Clone, Debug)]
pub struct StandardScoreDetector {
    detector_id: StatisticalDetectorId,
    metric_id: StatisticalMetricId,
    config: StandardScoreConfig,
    sequence_policy: SequencePolicy,
    statistics: RunningStatistics,
    last_sequence: Option<u64>,
    active: bool,
}

impl StandardScoreDetector {
    /// Creates a standardized-score detector.
    pub fn new(
        detector_id: StatisticalDetectorId,
        metric_id: StatisticalMetricId,
        config: StandardScoreConfig,
        sequence_policy: SequencePolicy,
    ) -> Self {
        Self {
            detector_id,
            metric_id,
            config,
            sequence_policy,
            statistics: RunningStatistics::new(),
            last_sequence: None,
            active: false,
        }
    }

    /// Returns running statistics.
    #[must_use]
    pub const fn statistics(&self) -> &RunningStatistics {
        &self.statistics
    }

    /// Returns whether the detector is active.
    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    fn validate_sequence(&self, sequence: u64) -> Result<(), StatisticalError> {
        if let Some(previous) = self.last_sequence {
            let valid = match self.sequence_policy {
                SequencePolicy::StrictlyIncreasing => sequence > previous,
                SequencePolicy::NonDecreasing => sequence >= previous,
            };

            if !valid {
                return Err(StatisticalError::SequenceRegression {
                    previous,
                    current: sequence,
                });
            }
        }

        Ok(())
    }
}

impl StatisticalDetector for StandardScoreDetector {
    fn observe(
        &mut self,
        observation: StatisticalObservation,
    ) -> Result<StatisticalEvidence, StatisticalError> {
        self.validate_sequence(observation.sequence)?;

        let previous_active = self.active;

        let score = self
            .statistics
            .z_score_against_current(observation.value);

        let (kind, active) = match score {
            None => (StatisticalEventKind::InsufficientEvidence, false),

            Some(value) if !value.is_finite() => {
                let anomalous = observation.value != self.statistics.mean();

                if anomalous {
                    (StatisticalEventKind::Anomaly, true)
                } else {
                    (StatisticalEventKind::Normal, false)
                }
            }

            Some(value) => {
                let anomalous = value.abs() >= self.config.threshold;

                if anomalous {
                    (StatisticalEventKind::Anomaly, true)
                } else {
                    (StatisticalEventKind::Normal, false)
                }
            }
        };

        if self.config.update_baseline {
            self.statistics.update(observation.value)?;
        }

        self.last_sequence = Some(observation.sequence);
        self.active = active;

        Ok(StatisticalEvidence {
            detector_id: self.detector_id.clone(),
            metric_id: self.metric_id.clone(),
            sequence: observation.sequence,
            observation: observation.value,
            score: score.unwrap_or(0.0),
            sample_count: self.statistics.count(),
            kind,
            previously_active: previous_active,
            active,
        })
    }

    fn detector_id(&self) -> &StatisticalDetectorId {
        &self.detector_id
    }

    fn metric_id(&self) -> &StatisticalMetricId {
        &self.metric_id
    }

    fn reset(&mut self) {
        self.statistics = RunningStatistics::new();
        self.last_sequence = None;
        self.active = false;
    }
}

/// Configuration for an EWMA deviation detector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EwmaDetectorConfig {
    /// EWMA parameters.
    pub ewma: EwmaConfig,

    /// Absolute deviation required to activate the detector.
    pub threshold: f64,

    /// Optional fixed reference.
    ///
    /// If `None`, the first EWMA value becomes the reference.
    pub reference: Option<f64>,

    /// Minimum samples before activation is allowed.
    pub minimum_samples: NonZeroU64,
}

impl EwmaDetectorConfig {
    /// Validates EWMA detector configuration.
    pub fn validate(&self) -> Result<(), StatisticalError> {
        validate_finite(self.threshold)?;

        if self.threshold <= 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "EWMA detector threshold must be positive",
            ));
        }

        if let Some(reference) = self.reference {
            validate_finite(reference)?;
        }

        Ok(())
    }
}

/// Streaming EWMA deviation detector.
#[derive(Clone, Debug)]
pub struct EwmaDetector {
    detector_id: StatisticalDetectorId,
    metric_id: StatisticalMetricId,
    config: EwmaDetectorConfig,
    sequence_policy: SequencePolicy,
    ewma: Ewma,
    reference: Option<f64>,
    last_sequence: Option<u64>,
    active: bool,
}

impl EwmaDetector {
    /// Creates an EWMA detector.
    pub fn new(
        detector_id: StatisticalDetectorId,
        metric_id: StatisticalMetricId,
        config: EwmaDetectorConfig,
        sequence_policy: SequencePolicy,
    ) -> Result<Self, StatisticalError> {
        config.validate()?;

        let reference = config.reference;

        Ok(Self {
            detector_id,
            metric_id,
            ewma: Ewma::new(config.ewma),
            config,
            sequence_policy,
            reference,
            last_sequence: None,
            active: false,
        })
    }

    /// Returns the current EWMA.
    #[must_use]
    pub const fn value(&self) -> Option<f64> {
        self.ewma.value()
    }

    /// Returns the configured/effective reference.
    #[must_use]
    pub const fn reference(&self) -> Option<f64> {
        self.reference
    }

    fn validate_sequence(&self, sequence: u64) -> Result<(), StatisticalError> {
        if let Some(previous) = self.last_sequence {
            let valid = match self.sequence_policy {
                SequencePolicy::StrictlyIncreasing => sequence > previous,
                SequencePolicy::NonDecreasing => sequence >= previous,
            };

            if !valid {
                return Err(StatisticalError::SequenceRegression {
                    previous,
                    current: sequence,
                });
            }
        }

        Ok(())
    }
}

impl StatisticalDetector for EwmaDetector {
    fn observe(
        &mut self,
        observation: StatisticalObservation,
    ) -> Result<StatisticalEvidence, StatisticalError> {
        self.validate_sequence(observation.sequence)?;

        let previous_active = self.active;
        let ewma_value = self.ewma.update(observation.value)?;

        if self.reference.is_none() {
            self.reference = Some(ewma_value);
        }

        let reference = self.reference.ok_or(StatisticalError::NumericFailure(
            "EWMA reference unexpectedly unavailable",
        ))?;

        let deviation = (ewma_value - reference).abs();

        if !deviation.is_finite() {
            return Err(StatisticalError::NumericFailure(
                "EWMA deviation became non-finite",
            ));
        }

        let active = self.ewma.count() >= self.config.minimum_samples.get()
            && deviation >= self.config.threshold;

        let kind = if self.ewma.count() < self.config.minimum_samples.get() {
            StatisticalEventKind::InsufficientEvidence
        } else if active {
            StatisticalEventKind::Anomaly
        } else {
            StatisticalEventKind::Normal
        };

        self.last_sequence = Some(observation.sequence);
        self.active = active;

        Ok(StatisticalEvidence {
            detector_id: self.detector_id.clone(),
            metric_id: self.metric_id.clone(),
            sequence: observation.sequence,
            observation: observation.value,
            score: deviation,
            sample_count: self.ewma.count(),
            kind,
            previously_active: previous_active,
            active,
        })
    }

    fn detector_id(&self) -> &StatisticalDetectorId {
        &self.detector_id
    }

    fn metric_id(&self) -> &StatisticalMetricId {
        &self.metric_id
    }

    fn reset(&mut self) {
        self.ewma.reset();
        self.reference = self.config.reference;
        self.last_sequence = None;
        self.active = false;
    }
}

/// Configuration for the statistical CUSUM detector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CusumDetectorConfig {
    /// CUSUM parameters.
    pub cusum: CusumConfig,

    /// Minimum observations required before a change can be reported.
    pub minimum_samples: NonZeroU64,
}

impl CusumDetectorConfig {
    /// Validates configuration.
    pub const fn validate(&self) -> Result<(), StatisticalError> {
        if self.cusum.allowance < 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "CUSUM allowance must not be negative",
            ));
        }

        if self.cusum.threshold <= 0.0 {
            return Err(StatisticalError::InvalidConfiguration(
                "CUSUM threshold must be positive",
            ));
        }

        Ok(())
    }
}

/// Streaming CUSUM detector.
#[derive(Clone, Debug)]
pub struct CusumDetector {
    detector_id: StatisticalDetectorId,
    metric_id: StatisticalMetricId,
    config: CusumDetectorConfig,
    sequence_policy: SequencePolicy,
    cusum: Cusum,
    last_sequence: Option<u64>,
}

impl CusumDetector {
    /// Creates a CUSUM detector.
    pub fn new(
        detector_id: StatisticalDetectorId,
        metric_id: StatisticalMetricId,
        config: CusumDetectorConfig,
        sequence_policy: SequencePolicy,
    ) -> Result<Self, StatisticalError> {
        config.validate()?;

        Ok(Self {
            detector_id,
            metric_id,
            cusum: Cusum::new(config.cusum),
            config,
            sequence_policy,
            last_sequence: None,
        })
    }

    /// Returns current CUSUM state.
    #[must_use]
    pub const fn cusum(&self) -> &Cusum {
        &self.cusum
    }

    fn validate_sequence(&self, sequence: u64) -> Result<(), StatisticalError> {
        if let Some(previous) = self.last_sequence {
            let valid = match self.sequence_policy {
                SequencePolicy::StrictlyIncreasing => sequence > previous,
                SequencePolicy::NonDecreasing => sequence >= previous,
            };

            if !valid {
                return Err(StatisticalError::SequenceRegression {
                    previous,
                    current: sequence,
                });
            }
        }

        Ok(())
    }
}

impl StatisticalDetector for CusumDetector {
    fn observe(
        &mut self,
        observation: StatisticalObservation,
    ) -> Result<StatisticalEvidence, StatisticalError> {
        self.validate_sequence(observation.sequence)?;

        let previous_active = self.cusum.active();
        let update = self.cusum.update(observation.value)?;

        let sample_count = self.cusum.count();

        let kind = if sample_count < self.config.minimum_samples.get() {
            StatisticalEventKind::InsufficientEvidence
        } else {
            update.event
        };

        self.last_sequence = Some(observation.sequence);

        let score = match self.config.cusum.direction {
            CusumDirection::Increase => update.positive_score,
            CusumDirection::Decrease => update.negative_score,
            CusumDirection::Both => update.positive_score.max(update.negative_score),
        };

        Ok(StatisticalEvidence {
            detector_id: self.detector_id.clone(),
            metric_id: self.metric_id.clone(),
            sequence: observation.sequence,
            observation: observation.value,
            score,
            sample_count,
            kind,
            previously_active: previous_active,
            active: update.active && sample_count >= self.config.minimum_samples.get(),
        })
    }

    fn detector_id(&self) -> &StatisticalDetectorId {
        &self.detector_id
    }

    fn metric_id(&self) -> &StatisticalMetricId {
        &self.metric_id
    }

    fn reset(&mut self) {
        self.cusum.reset();
        self.last_sequence = None;
    }
}

/// Consecutive-evidence requirement.
///
/// Requiring multiple observations can reduce sensitivity to isolated
/// telemetry noise while increasing detection latency. This tradeoff must be
/// explicitly configured rather than hidden in the detector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsecutiveRequirement {
    /// Activate immediately.
    Immediate,

    /// Require the specified number of consecutive positive observations.
    Require(NonZeroU64),
}

/// Tracks consecutive positive/negative evidence in O(1) memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsecutiveTracker {
    requirement: ConsecutiveRequirement,
    count: u64,
}

impl ConsecutiveTracker {
    /// Creates a tracker.
    #[must_use]
    pub const fn new(requirement: ConsecutiveRequirement) -> Self {
        Self {
            requirement,
            count: 0,
        }
    }

    /// Adds one positive observation.
    ///
    /// Returns `true` once the configured requirement is reached.
    pub fn positive(&mut self) -> Result<bool, StatisticalError> {
        match self.requirement {
            ConsecutiveRequirement::Immediate => Ok(true),

            ConsecutiveRequirement::Require(required) => {
                if self.count == u64::MAX {
                    return Err(StatisticalError::CounterOverflow);
                }

                self.count += 1;
                Ok(self.count >= required.get())
            }
        }
    }

    /// Clears accumulated evidence.
    pub const fn negative(&mut self) {
        self.count = 0;
    }

    /// Returns the current streak.
    #[must_use]
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Returns the configured requirement.
    #[must_use]
    pub const fn requirement(&self) -> ConsecutiveRequirement {
        self.requirement
    }
}

/// Statistical detector errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatisticalError {
    /// Invalid detector configuration.
    InvalidConfiguration(&'static str),

    /// Invalid observation value.
    InvalidObservation(&'static str),

    /// Sequence ordering was violated.
    SequenceRegression {
        /// Previously accepted sequence.
        previous: u64,

        /// Incoming sequence.
        current: u64,
    },

    /// An internal numerical operation became invalid.
    NumericFailure(&'static str),

    /// A bounded counter cannot represent another observation.
    CounterOverflow,
}

impl fmt::Display for StatisticalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid statistical configuration: {message}")
            }

            Self::InvalidObservation(message) => {
                write!(formatter, "invalid statistical observation: {message}")
            }

            Self::SequenceRegression { previous, current } => {
                write!(
                    formatter,
                    "statistical observation sequence regression: previous={previous}, current={current}"
                )
            }

            Self::NumericFailure(message) => {
                write!(formatter, "statistical numerical failure: {message}")
            }

            Self::CounterOverflow => {
                formatter.write_str("statistical observation counter overflow")
            }
        }
    }
}

impl std::error::Error for StatisticalError {}

fn validate_finite(value: f64) -> Result<(), StatisticalError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(StatisticalError::InvalidObservation(
            "statistical values must be finite",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detector_id() -> StatisticalDetectorId {
        StatisticalDetectorId::new("test.detector").expect("valid detector id")
    }

    fn metric_id() -> StatisticalMetricId {
        StatisticalMetricId::new("test.metric").expect("valid metric id")
    }

    fn observation(value: f64, sequence: u64) -> StatisticalObservation {
        StatisticalObservation::new(value, sequence).expect("valid observation")
    }

    #[test]
    fn running_statistics_is_empty_initially() {
        let statistics = RunningStatistics::new();

        assert_eq!(statistics.count(), 0);
        assert_eq!(statistics.mean(), 0.0);
        assert_eq!(statistics.population_variance(), None);
        assert_eq!(statistics.sample_variance(), None);
    }

    #[test]
    fn running_statistics_computes_mean() {
        let mut statistics = RunningStatistics::new();

        statistics.update(1.0).expect("valid");
        statistics.update(2.0).expect("valid");
        statistics.update(3.0).expect("valid");

        assert_eq!(statistics.count(), 3);
        assert!((statistics.mean() - 2.0).abs() < 1e-12);

        let variance = statistics
            .sample_variance()
            .expect("variance should exist");

        assert!((variance - 1.0).abs() < 1e-12);
    }

    #[test]
    fn running_statistics_rejects_nan() {
        let mut statistics = RunningStatistics::new();

        assert!(matches!(
            statistics.update(f64::NAN),
            Err(StatisticalError::InvalidObservation(_))
        ));
    }

    #[test]
    fn running_statistics_rejects_infinity() {
        let mut statistics = RunningStatistics::new();

        assert!(statistics.update(f64::INFINITY).is_err());
        assert!(statistics.update(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn ewma_starts_at_first_observation() {
        let config = EwmaConfig::new(0.5).expect("valid");

        let mut ewma = Ewma::new(config);

        assert!((ewma.update(10.0).expect("valid") - 10.0).abs() < 1e-12);
        assert!((ewma.update(20.0).expect("valid") - 15.0).abs() < 1e-12);
    }

    #[test]
    fn ewma_rejects_invalid_alpha() {
        assert!(EwmaConfig::new(0.0).is_err());
        assert!(EwmaConfig::new(-0.1).is_err());
        assert!(EwmaConfig::new(1.1).is_err());
        assert!(EwmaConfig::new(f64::NAN).is_err());
    }

    #[test]
    fn cusum_detects_increase() {
        let config =
            CusumConfig::new(0.0, 0.0, 3.0, CusumDirection::Increase).expect("valid");

        let mut cusum = Cusum::new(config);

        assert_eq!(
            cusum.update(1.0).expect("valid").event,
            StatisticalEventKind::Normal
        );

        assert_eq!(
            cusum.update(1.0).expect("valid").event,
            StatisticalEventKind::Normal
        );

        assert_eq!(
            cusum.update(1.0).expect("valid").event,
            StatisticalEventKind::ChangeDetected
        );
    }

    #[test]
    fn cusum_detects_decrease() {
        let config =
            CusumConfig::new(0.0, 0.0, 3.0, CusumDirection::Decrease).expect("valid");

        let mut cusum = Cusum::new(config);

        assert_eq!(
            cusum.update(-1.0).expect("valid").event,
            StatisticalEventKind::Normal
        );

        assert_eq!(
            cusum.update(-1.0).expect("valid").event,
            StatisticalEventKind::Normal
        );

        assert_eq!(
            cusum.update(-1.0).expect("valid").event,
            StatisticalEventKind::ChangeDetected
        );
    }

    #[test]
    fn standard_detector_requires_baseline() {
        let minimum = NonZeroU64::new(2).expect("non-zero");

        let config =
            StandardScoreConfig::new(3.0, minimum, true).expect("valid");

        let mut detector = StandardScoreDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::StrictlyIncreasing,
        );

        let first = detector
            .observe(observation(1.0, 1))
            .expect("valid");

        assert_eq!(first.kind, StatisticalEventKind::InsufficientEvidence);
    }

    #[test]
    fn standard_detector_detects_outlier() {
        let minimum = NonZeroU64::new(2).expect("non-zero");

        let config =
            StandardScoreConfig::new(2.0, minimum, true).expect("valid");

        let mut detector = StandardScoreDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::StrictlyIncreasing,
        );

        detector
            .observe(observation(10.0, 1))
            .expect("valid");

        detector
            .observe(observation(10.0, 2))
            .expect("valid");

        let evidence = detector
            .observe(observation(20.0, 3))
            .expect("valid");

        assert_eq!(evidence.kind, StatisticalEventKind::Anomaly);
        assert!(evidence.active);
    }

    #[test]
    fn standard_detector_rejects_sequence_regression() {
        let minimum = NonZeroU64::new(1).expect("non-zero");

        let config =
            StandardScoreConfig::new(2.0, minimum, true).expect("valid");

        let mut detector = StandardScoreDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::StrictlyIncreasing,
        );

        detector
            .observe(observation(1.0, 10))
            .expect("valid");

        assert!(matches!(
            detector.observe(observation(1.0, 9)),
            Err(StatisticalError::SequenceRegression {
                previous: 10,
                current: 9
            })
        ));
    }

    #[test]
    fn non_decreasing_sequence_accepts_replay() {
        let minimum = NonZeroU64::new(1).expect("non-zero");

        let config =
            StandardScoreConfig::new(2.0, minimum, true).expect("valid");

        let mut detector = StandardScoreDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::NonDecreasing,
        );

        detector
            .observe(observation(1.0, 10))
            .expect("valid");

        assert!(detector.observe(observation(1.0, 10)).is_ok());
    }

    #[test]
    fn ewma_detector_uses_explicit_reference() {
        let config = EwmaDetectorConfig {
            ewma: EwmaConfig::new(0.5).expect("valid"),
            threshold: 2.0,
            reference: Some(10.0),
            minimum_samples: NonZeroU64::new(1).expect("non-zero"),
        };

        let mut detector = EwmaDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::StrictlyIncreasing,
        )
        .expect("valid");

        let evidence = detector
            .observe(observation(20.0, 1))
            .expect("valid");

        assert_eq!(evidence.kind, StatisticalEventKind::Anomaly);
    }

    #[test]
    fn ewma_detector_rejects_non_finite_reference() {
        let config = EwmaDetectorConfig {
            ewma: EwmaConfig::new(0.5).expect("valid"),
            threshold: 2.0,
            reference: Some(f64::NAN),
            minimum_samples: NonZeroU64::new(1).expect("non-zero"),
        };

        assert!(
            EwmaDetector::new(
                detector_id(),
                metric_id(),
                config,
                SequencePolicy::StrictlyIncreasing,
            )
            .is_err()
        );
    }

    #[test]
    fn consecutive_requirement_is_bounded() {
        let required = NonZeroU64::new(3).expect("non-zero");

        let mut tracker =
            ConsecutiveTracker::new(ConsecutiveRequirement::Require(required));

        assert!(!tracker.positive().expect("valid"));
        assert!(!tracker.positive().expect("valid"));
        assert!(tracker.positive().expect("valid"));

        tracker.negative();

        assert_eq!(tracker.count(), 0);
    }

    #[test]
    fn detector_reset_is_complete() {
        let minimum = NonZeroU64::new(1).expect("non-zero");

        let config =
            StandardScoreConfig::new(2.0, minimum, true).expect("valid");

        let mut detector = StandardScoreDetector::new(
            detector_id(),
            metric_id(),
            config,
            SequencePolicy::StrictlyIncreasing,
        );

        detector
            .observe(observation(10.0, 1))
            .expect("valid");

        detector.reset();

        assert_eq!(detector.statistics().count(), 0);
        assert_eq!(detector.statistics().mean(), 0.0);

        detector
            .observe(observation(20.0, 1))
            .expect("sequence should restart after reset");
    }

    #[test]
    fn detector_identity_is_preserved() {
        let detector = StatisticalDetectorId::new("resilience.statistics").expect("valid");

        assert_eq!(detector.as_str(), "resilience.statistics");
        assert_eq!(detector.to_string(), "resilience.statistics");
    }

    #[test]
    fn metric_identity_is_preserved() {
        let metric = StatisticalMetricId::new("quantum.logical_error_rate")
            .expect("valid");

        assert_eq!(metric.as_str(), "quantum.logical_error_rate");
        assert_eq!(metric.to_string(), "quantum.logical_error_rate");
    }
}