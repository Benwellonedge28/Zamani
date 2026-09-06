//! # Quantum resilience drift detection
//!
//! Streaming, backend-independent detection of temporal drift in quantum
//! execution and hardware metrics.
//!
//! ## Architectural responsibility
//!
//! This module answers one question:
//!
//! > "Has an observed metric changed sufficiently from its configured
//! > reference/baseline behavior to constitute drift evidence?"
//!
//! It does NOT:
//!
//! - diagnose the root cause;
//! - perform calibration;
//! - modify hardware;
//! - reroute circuits;
//! - reschedule execution;
//! - perform QEC;
//! - perform error mitigation;
//! - select a backend;
//! - initiate recovery;
//! - own quantum topology;
//! - assume a fixed number of qubits;
//! - assume a specific quantum technology.
//!
//! Those responsibilities belong to the corresponding resilience,
//! hardware, routing, scheduling, QEC, ZQN, and recovery subsystems.
//!
//! ## Scaling model
//!
//! A `DriftDetector` maintains only constant-size state:
//!
//! - sample count;
//! - running mean;
//! - running variance accumulator;
//! - last observation sequence;
//! - baseline statistics;
//! - current drift state;
//! - configurable consecutive-confirmation counters.
//!
//! It does not retain the complete observation history.
//!
//! Therefore memory consumption is O(1) per detector instance.
//!
//! A deployment may instantiate one detector for one metric, one detector
//! for one resource, or many detectors across a distributed machine. The
//! registry/orchestration layer is responsible for managing the number of
//! detector instances.
//!
//! ## Numerical model
//!
//! `f64` is used for scalar measurements because hardware telemetry commonly
//! contains real-valued physical quantities. NaN and infinities are rejected.
//!
//! The implementation uses Welford's online algorithm for numerical
//! stability and does not store an observation window.
//!
//! ## Important distinction
//!
//! Drift detection is not the same as threshold detection.
//!
//! A threshold detector asks:
//!
//!     "Is x outside an allowed absolute range?"
//!
//! This detector asks:
//!
//!     "Has the statistical behavior of x moved away from its reference
//!      behavior sufficiently to constitute drift?"
//!
//! ## Quantum resource identity
//!
//! This file intentionally does not define or duplicate `QubitId`.
//!
//! Resource identity belongs to `quantum::resilience::model::resource` and
//! the canonical quantum IR/hardware layers. When a caller needs a
//! qubit-specific drift signal, the normalized detection layer should
//! associate this detector's metric with the canonical resource identity,
//! including `quantum::ir::qubit::QubitId` where appropriate.
//!
//! This prevents the detector from creating a second incompatible qubit
//! identity model.
//!
//! ## Determinism
//!
//! Given identical observations, configuration, and floating-point
//! environment, the detector follows deterministic state transitions.
//!
//! ## Safety invariant
//!
//! A drift event is evidence only. It must never directly cause a recovery
//! action. Diagnosis, policy, planning, and verification remain responsible
//! for deciding what to do with that evidence.

use core::fmt;
use core::num::NonZeroU64;

/// Stable identifier for a drift detector.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DriftDetectorId(String);

impl DriftDetectorId {
    /// Creates a detector identifier.
    ///
    /// Empty identifiers are rejected because identifiers participate in
    /// provenance, telemetry, registry lookup, and deterministic replay.
    pub fn new(value: impl Into<String>) -> Result<Self, DriftError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DriftError::InvalidConfiguration(
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

/// Identifier for the metric being monitored.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DriftMetricId(String);

impl DriftMetricId {
    /// Creates a metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, DriftError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DriftError::InvalidConfiguration(
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

/// Policy controlling how observations are incorporated into the
/// detector's reference statistics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaselinePolicy {
    /// Establish the baseline from the first configured number of samples.
    ///
    /// During baseline collection no drift is emitted.
    FixedInitial {
        /// Number of samples required to establish the baseline.
        samples: NonZeroU64,
    },

    /// Use a caller-provided baseline and never update it automatically.
    Fixed,

    /// Adapt the baseline only while the detector is in the normal state.
    ///
    /// This prevents confirmed drift from silently becoming the new normal.
    Adaptive {
        /// Exponential adaptation factor in `(0, 1]`.
        alpha: DriftAlpha,
    },
}

impl BaselinePolicy {
    /// Validates the policy.
    pub fn validate(self) -> Result<(), DriftError> {
        if let Self::Adaptive { alpha } = self {
            alpha.validate()?;
        }

        Ok(())
    }
}

/// Validated exponential adaptation coefficient.
///
/// The valid range is strictly greater than zero and less than or equal to
/// one. A value of `1.0` makes the current observation fully replace the
/// previous mean contribution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriftAlpha(f64);

impl DriftAlpha {
    /// Creates an adaptation coefficient.
    pub fn new(value: f64) -> Result<Self, DriftError> {
        if !value.is_finite() {
            return Err(DriftError::NonFiniteValue);
        }

        if value <= 0.0 || value > 1.0 {
            return Err(DriftError::InvalidConfiguration(
                "drift adaptation alpha must be greater than 0 and less than or equal to 1",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }

    fn validate(self) -> Result<(), DriftError> {
        Self::new(self.0).map(|_| ())
    }
}

/// How drift magnitude is measured.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DriftMeasure {
    /// Absolute difference between current and baseline mean.
    Absolute,

    /// Difference normalized by the baseline standard deviation.
    ///
    /// This is useful when metrics have different natural scales.
    Standardized {
        /// Minimum standard deviation used to avoid division by zero.
        ///
        /// This is a configuration value, never an implicit constant.
        minimum_stddev: f64,
    },

    /// Relative difference:
    ///
    ///     |current - baseline| / |baseline|
    ///
    /// A configured denominator floor avoids division by zero.
    Relative {
        /// Minimum absolute baseline magnitude permitted as denominator.
        denominator_floor: f64,
    },
}

impl DriftMeasure {
    fn validate(self) -> Result<(), DriftError> {
        match self {
            Self::Absolute => Ok(()),

            Self::Standardized { minimum_stddev } => {
                validate_positive_finite(
                    minimum_stddev,
                    "minimum standard deviation",
                )
            }

            Self::Relative { denominator_floor } => {
                validate_positive_finite(
                    denominator_floor,
                    "relative denominator floor",
                )
            }
        }
    }

    fn compute(
        self,
        baseline_mean: f64,
        baseline_stddev: f64,
        current_mean: f64,
    ) -> Result<f64, DriftError> {
        let difference = (current_mean - baseline_mean).abs();

        match self {
            Self::Absolute => Ok(difference),

            Self::Standardized { minimum_stddev } => {
                let denominator = baseline_stddev.max(minimum_stddev);

                if !denominator.is_finite() || denominator <= 0.0 {
                    return Err(DriftError::NumericalFailure(
                        "standardized drift denominator is invalid",
                    ));
                }

                Ok(difference / denominator)
            }

            Self::Relative { denominator_floor } => {
                let denominator =
                    baseline_mean.abs().max(denominator_floor);

                if !denominator.is_finite() || denominator <= 0.0 {
                    return Err(DriftError::NumericalFailure(
                        "relative drift denominator is invalid",
                    ));
                }

                Ok(difference / denominator)
            }
        }
    }
}

/// Policy controlling when a measured drift becomes an emitted event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmationPolicy {
    /// Emit immediately when the drift threshold is crossed.
    Immediate,

    /// Require a configurable number of consecutive drift observations.
    Consecutive {
        /// Number of consecutive drift observations required.
        required: NonZeroU64,
    },
}

impl ConfirmationPolicy {
    fn validate(self) -> Result<(), DriftError> {
        Ok(())
    }
}

/// Policy controlling how quickly a confirmed drift condition is cleared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClearPolicy {
    /// Clear immediately when the measured drift falls below the threshold.
    Immediate,

    /// Require a configurable number of consecutive non-drift observations.
    Consecutive {
        /// Number of consecutive non-drift observations required.
        required: NonZeroU64,
    },
}

impl ClearPolicy {
    fn validate(self) -> Result<(), DriftError> {
        Ok(())
    }
}

/// Sequence ordering policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SequencePolicy {
    /// Each observation must have a strictly larger sequence number.
    StrictlyIncreasing,

    /// Equal sequence numbers are accepted, which is useful for deterministic
    /// replay and multiple observations sharing a logical sample boundary.
    NonDecreasing,
}

impl Default for SequencePolicy {
    fn default() -> Self {
        Self::NonDecreasing
    }
}

/// Controls whether every observation or only state transitions are emitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmissionPolicy {
    /// Emit only when drift becomes confirmed or clears.
    TransitionsOnly,

    /// Emit every observation for which the measured drift is at or above
    /// the configured threshold.
    EveryDrift,

    /// Emit an evaluation for every observation.
    EveryEvaluation,
}

impl Default for EmissionPolicy {
    fn default() -> Self {
        Self::TransitionsOnly
    }
}

/// Current detector state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriftState {
    /// No confirmed drift is currently active.
    Normal,

    /// Drift has been confirmed according to the configured policy.
    Drifted,
}

/// Classification of a detector output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DriftEventKind {
    /// Drift was newly confirmed.
    DriftStarted,

    /// Previously confirmed drift has cleared.
    DriftCleared,

    /// Drift remains active.
    DriftContinues,

    /// Observation does not currently indicate drift.
    Normal,
}

/// A single metric observation.
///
/// The detector does not impose a time representation. The caller provides
/// both a monotonically ordered sequence and an optional timestamp.
///
/// The sequence is the authoritative ordering value for deterministic
/// processing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriftObservation {
    /// Detector sequence number.
    pub sequence: u64,

    /// Optional external timestamp represented as nanoseconds.
    ///
    /// `None` is valid when the source does not expose a timestamp.
    pub timestamp_nanos: Option<u128>,

    /// Observed metric value.
    pub value: f64,
}

impl DriftObservation {
    /// Constructs an observation without an external timestamp.
    pub fn new(sequence: u64, value: f64) -> Result<Self, DriftError> {
        Self::with_timestamp(sequence, None, value)
    }

    /// Constructs an observation with an optional timestamp.
    pub fn with_timestamp(
        sequence: u64,
        timestamp_nanos: Option<u128>,
        value: f64,
    ) -> Result<Self, DriftError> {
        if !value.is_finite() {
            return Err(DriftError::NonFiniteValue);
        }

        Ok(Self {
            sequence,
            timestamp_nanos,
            value,
        })
    }
}

/// A validated baseline supplied by the caller.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriftBaseline {
    /// Number of observations represented by the baseline.
    pub sample_count: u64,

    /// Baseline mean.
    pub mean: f64,

    /// Baseline population variance.
    pub variance: f64,
}

impl DriftBaseline {
    /// Constructs a baseline.
    ///
    /// A zero-sample baseline is invalid because it cannot represent a
    /// statistical reference.
    pub fn new(
        sample_count: u64,
        mean: f64,
        variance: f64,
    ) -> Result<Self, DriftError> {
        if sample_count == 0 {
            return Err(DriftError::InvalidConfiguration(
                "baseline sample count must be greater than zero",
            ));
        }

        if !mean.is_finite() || !variance.is_finite() {
            return Err(DriftError::NonFiniteValue);
        }

        if variance < 0.0 {
            return Err(DriftError::InvalidConfiguration(
                "baseline variance must not be negative",
            ));
        }

        Ok(Self {
            sample_count,
            mean,
            variance,
        })
    }

    fn standard_deviation(self) -> f64 {
        self.variance.sqrt()
    }
}

/// Complete detector configuration.
///
/// Every operational threshold is explicit. There are no hidden sensitivity
/// constants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriftConfig {
    /// How baseline statistics are managed.
    pub baseline_policy: BaselinePolicy,

    /// How drift magnitude is computed.
    pub measure: DriftMeasure,

    /// Drift magnitude at or above which an observation is considered
    /// evidence of drift.
    pub detection_threshold: f64,

    /// Confirmation rule.
    pub confirmation: ConfirmationPolicy,

    /// Clear rule.
    pub clearing: ClearPolicy,

    /// Sequence ordering policy.
    pub sequence_policy: SequencePolicy,

    /// Event emission policy.
    pub emission: EmissionPolicy,
}

impl DriftConfig {
    /// Validates the complete configuration.
    pub fn validate(self) -> Result<(), DriftError> {
        self.baseline_policy.validate()?;
        self.measure.validate()?;
        self.confirmation.validate()?;
        self.clearing.validate()?;

        if !self.detection_threshold.is_finite()
            || self.detection_threshold < 0.0
        {
            return Err(DriftError::InvalidConfiguration(
                "detection threshold must be finite and non-negative",
            ));
        }

        Ok(())
    }
}

impl Default for DriftConfig {
    fn default() -> Self {
        // This default is intentionally conservative and contains no
        // hardware-specific threshold. Applications requiring production
        // decisions should normally provide explicit configuration.
        Self {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 0.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::NonDecreasing,
            emission: EmissionPolicy::TransitionsOnly,
        }
    }
}

/// Online statistical state.
///
/// Welford's algorithm is used so that the detector does not need to retain
/// the observation history.
#[derive(Clone, Copy, Debug, PartialEq)]
struct OnlineStatistics {
    count: u64,
    mean: f64,
    m2: f64,
}

impl OnlineStatistics {
    const EMPTY: Self = Self {
        count: 0,
        mean: 0.0,
        m2: 0.0,
    };

    fn update(&mut self, value: f64) -> Result<(), DriftError> {
        if !value.is_finite() {
            return Err(DriftError::NonFiniteValue);
        }

        if self.count == u64::MAX {
            return Err(DriftError::CounterOverflow);
        }

        self.count += 1;

        let count = self.count as f64;

        let delta = value - self.mean;
        self.mean += delta / count;

        let delta_after = value - self.mean;
        self.m2 += delta * delta_after;

        if !self.mean.is_finite() || !self.m2.is_finite() {
            return Err(DriftError::NumericalFailure(
                "online statistical state became non-finite",
            ));
        }

        // Floating-point roundoff can produce a tiny negative value even
        // though variance is mathematically non-negative.
        if self.m2 < 0.0 {
            self.m2 = 0.0;
        }

        Ok(())
    }

    fn variance(self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.m2 / self.count as f64
        }
    }

    fn baseline(self) -> Option<DriftBaseline> {
        if self.count == 0 {
            None
        } else {
            Some(DriftBaseline {
                sample_count: self.count,
                mean: self.mean,
                variance: self.variance(),
            })
        }
    }
}

/// Result of processing one observation.
#[derive(Clone, Debug, PartialEq)]
pub struct DriftEvaluation {
    /// Detector identity.
    pub detector_id: DriftDetectorId,

    /// Metric identity.
    pub metric_id: DriftMetricId,

    /// Observation sequence.
    pub sequence: u64,

    /// Optional observation timestamp.
    pub timestamp_nanos: Option<u128>,

    /// Observed value.
    pub value: f64,

    /// Current baseline mean.
    pub baseline_mean: f64,

    /// Current baseline standard deviation.
    pub baseline_stddev: f64,

    /// Computed drift magnitude.
    pub drift_magnitude: f64,

    /// Configured detection threshold.
    pub detection_threshold: f64,

    /// Whether this individual observation exceeds the drift threshold.
    pub drift_observed: bool,

    /// Detector state before this observation.
    pub previous_state: DriftState,

    /// Detector state after this observation.
    pub current_state: DriftState,

    /// Number of consecutive observations supporting the current
    /// transition direction.
    pub confirmation_streak: u64,

    /// Event classification.
    pub kind: DriftEventKind,
}

impl DriftEvaluation {
    /// Returns true if this evaluation represents a newly confirmed drift.
    #[must_use]
    pub fn started(&self) -> bool {
        self.kind == DriftEventKind::DriftStarted
    }

    /// Returns true if this evaluation represents a cleared drift.
    #[must_use]
    pub fn cleared(&self) -> bool {
        self.kind == DriftEventKind::DriftCleared
    }

    /// Returns true if drift is active after this evaluation.
    #[must_use]
    pub fn is_drifted(&self) -> bool {
        self.current_state == DriftState::Drifted
    }
}

/// Errors produced by the drift detector.
#[derive(Clone, Debug, PartialEq)]
pub enum DriftError {
    /// Configuration is invalid.
    InvalidConfiguration(&'static str),

    /// An observation contains a non-finite number.
    NonFiniteValue,

    /// An observation sequence violates the configured ordering policy.
    SequenceRegression {
        previous: u64,
        current: u64,
    },

    /// The observation cannot be associated with this detector.
    MetricMismatch {
        expected: DriftMetricId,
        actual: DriftMetricId,
    },

    /// An internal statistical computation became invalid.
    NumericalFailure(&'static str),

    /// An internal counter cannot represent another observation.
    CounterOverflow,

    /// A baseline was required but has not been established.
    BaselineUnavailable,
}

impl fmt::Display for DriftError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid drift configuration: {message}")
            }

            Self::NonFiniteValue => {
                write!(formatter, "drift observation must be finite")
            }

            Self::SequenceRegression { previous, current } => write!(
                formatter,
                "drift observation sequence regressed: previous={previous}, current={current}"
            ),

            Self::MetricMismatch { expected, actual } => write!(
                formatter,
                "drift metric mismatch: expected={}, actual={}",
                expected.as_str(),
                actual.as_str()
            ),

            Self::NumericalFailure(message) => {
                write!(formatter, "drift numerical failure: {message}")
            }

            Self::CounterOverflow => {
                write!(formatter, "drift detector counter overflow")
            }

            Self::BaselineUnavailable => {
                write!(formatter, "drift baseline is not available")
            }
        }
    }
}

impl std::error::Error for DriftError {}

/// Streaming drift detector.
///
/// One instance monitors one metric stream. This deliberate one-stream
/// contract avoids an internal unbounded map and gives callers explicit
/// control over resource partitioning.
///
/// For example, the higher-level registry may maintain detectors for:
///
/// - a device-wide gate fidelity metric;
/// - a specific physical qubit;
/// - a coupling;
/// - readout fidelity;
/// - T1/T2;
/// - leakage;
/// - logical error rate;
/// - execution latency.
///
/// The detector itself does not need to know which kind of resource the
/// metric describes.
#[derive(Clone, Debug)]
pub struct DriftDetector {
    detector_id: DriftDetectorId,
    metric_id: DriftMetricId,
    config: DriftConfig,

    baseline: Option<DriftBaseline>,
    calibration_statistics: OnlineStatistics,

    state: DriftState,

    last_sequence: Option<u64>,

    positive_streak: u64,
    negative_streak: u64,

    observations: u64,
}

impl DriftDetector {
    /// Creates a detector without an externally supplied baseline.
    ///
    /// `BaselinePolicy::FixedInitial` must be used when the detector is
    /// expected to establish its own initial baseline.
    pub fn new(
        detector_id: DriftDetectorId,
        metric_id: DriftMetricId,
        config: DriftConfig,
    ) -> Result<Self, DriftError> {
        config.validate()?;

        if matches!(config.baseline_policy, BaselinePolicy::Fixed) {
            return Err(DriftError::InvalidConfiguration(
                "fixed baseline policy requires an explicit baseline; use new_with_baseline",
            ));
        }

        Ok(Self {
            detector_id,
            metric_id,
            config,
            baseline: None,
            calibration_statistics: OnlineStatistics::EMPTY,
            state: DriftState::Normal,
            last_sequence: None,
            positive_streak: 0,
            negative_streak: 0,
            observations: 0,
        })
    }

    /// Creates a detector with an explicit baseline.
    pub fn new_with_baseline(
        detector_id: DriftDetectorId,
        metric_id: DriftMetricId,
        config: DriftConfig,
        baseline: DriftBaseline,
    ) -> Result<Self, DriftError> {
        config.validate()?;

        if matches!(
            config.baseline_policy,
            BaselinePolicy::FixedInitial { .. }
        ) {
            return Err(DriftError::InvalidConfiguration(
                "fixed-initial policy must establish its baseline from observations",
            ));
        }

        Ok(Self {
            detector_id,
            metric_id,
            config,
            baseline: Some(baseline),
            calibration_statistics: OnlineStatistics::EMPTY,
            state: DriftState::Normal,
            last_sequence: None,
            positive_streak: 0,
            negative_streak: 0,
            observations: 0,
        })
    }

    /// Returns the detector identifier.
    #[must_use]
    pub fn detector_id(&self) -> &DriftDetectorId {
        &self.detector_id
    }

    /// Returns the metric identifier.
    #[must_use]
    pub fn metric_id(&self) -> &DriftMetricId {
        &self.metric_id
    }

    /// Returns the configuration.
    #[must_use]
    pub fn config(&self) -> DriftConfig {
        self.config
    }

    /// Returns the current detector state.
    #[must_use]
    pub fn state(&self) -> DriftState {
        self.state
    }

    /// Returns the current baseline.
    #[must_use]
    pub fn baseline(&self) -> Option<DriftBaseline> {
        self.baseline
    }

    /// Returns the total number of processed observations.
    #[must_use]
    pub fn observations(&self) -> u64 {
        self.observations
    }

    /// Processes exactly one observation.
    ///
    /// `None` means that the observation produced no event under the selected
    /// emission policy.
    pub fn observe(
        &mut self,
        observation: DriftObservation,
    ) -> Result<Option<DriftEvaluation>, DriftError> {
        self.validate_sequence(observation.sequence)?;

        if self.observations == u64::MAX {
            return Err(DriftError::CounterOverflow);
        }

        self.observations += 1;

        self.last_sequence = Some(observation.sequence);

        if self.baseline.is_none() {
            self.collect_initial_baseline(observation)?;

            return Ok(None);
        }

        let baseline = self.baseline.ok_or(DriftError::BaselineUnavailable)?;

        let current_mean = match self.config.baseline_policy {
            BaselinePolicy::Fixed
            | BaselinePolicy::FixedInitial { .. } => {
                observation.value
            }

            BaselinePolicy::Adaptive { .. } => {
                // The current observation is intentionally evaluated against
                // the existing baseline before that baseline is adapted.
                observation.value
            }
        };

        let drift_magnitude = self.config.measure.compute(
            baseline.mean,
            baseline.standard_deviation(),
            current_mean,
        )?;

        if !drift_magnitude.is_finite() {
            return Err(DriftError::NumericalFailure(
                "computed drift magnitude is non-finite",
            ));
        }

        let drift_observed =
            drift_magnitude >= self.config.detection_threshold;

        let previous_state = self.state;

        let event_kind = self.update_state(drift_observed)?;

        self.update_baseline(observation.value, drift_observed)?;

        let evaluation = DriftEvaluation {
            detector_id: self.detector_id.clone(),
            metric_id: self.metric_id.clone(),
            sequence: observation.sequence,
            timestamp_nanos: observation.timestamp_nanos,
            value: observation.value,
            baseline_mean: baseline.mean,
            baseline_stddev: baseline.standard_deviation(),
            drift_magnitude,
            detection_threshold: self.config.detection_threshold,
            drift_observed,
            previous_state,
            current_state: self.state,
            confirmation_streak: self.current_streak(),
            kind: event_kind,
        };

        if self.should_emit(evaluation.kind, evaluation.drift_observed) {
            Ok(Some(evaluation))
        } else {
            Ok(None)
        }
    }

    /// Processes an unbounded stream without accumulating it internally.
    ///
    /// The callback controls whether evaluations are persisted, forwarded to
    /// telemetry, converted into incidents, or discarded.
    ///
    /// Memory usage inside this detector remains O(1).
    pub fn process_stream<I, F>(
        &mut self,
        observations: I,
        mut emit: F,
    ) -> Result<(), DriftError>
    where
        I: IntoIterator<Item = DriftObservation>,
        F: FnMut(DriftEvaluation),
    {
        for observation in observations {
            if let Some(evaluation) = self.observe(observation)? {
                emit(evaluation);
            }
        }

        Ok(())
    }

    /// Installs a new baseline and resets detector state.
    ///
    /// This operation should normally be performed only by a trusted
    /// calibration/management layer after the new baseline has been
    /// validated.
    pub fn replace_baseline(
        &mut self,
        baseline: DriftBaseline,
    ) -> Result<(), DriftError> {
        self.baseline = Some(baseline);
        self.state = DriftState::Normal;
        self.positive_streak = 0;
        self.negative_streak = 0;

        Ok(())
    }

    /// Resets detector state while preserving its configured baseline.
    ///
    /// This is useful for deterministic replay or execution-session
    /// boundaries.
    pub fn reset(&mut self) {
        self.state = DriftState::Normal;
        self.last_sequence = None;
        self.positive_streak = 0;
        self.negative_streak = 0;
        self.observations = 0;
        self.calibration_statistics = OnlineStatistics::EMPTY;
    }

    /// Resets the detector and removes its baseline.
    ///
    /// This is intended for a new calibration epoch.
    pub fn reset_calibration(&mut self) {
        self.reset();
        self.baseline = None;
    }

    fn validate_sequence(&self, sequence: u64) -> Result<(), DriftError> {
        let Some(previous) = self.last_sequence else {
            return Ok(());
        };

        match self.config.sequence_policy {
            SequencePolicy::StrictlyIncreasing if sequence <= previous => {
                Err(DriftError::SequenceRegression {
                    previous,
                    current: sequence,
                })
            }

            SequencePolicy::NonDecreasing if sequence < previous => {
                Err(DriftError::SequenceRegression {
                    previous,
                    current: sequence,
                })
            }

            _ => Ok(()),
        }
    }

    fn collect_initial_baseline(
        &mut self,
        observation: DriftObservation,
    ) -> Result<(), DriftError> {
        let required = match self.config.baseline_policy {
            BaselinePolicy::FixedInitial { samples } => samples.get(),
            BaselinePolicy::Fixed | BaselinePolicy::Adaptive { .. } => {
                return Err(DriftError::BaselineUnavailable);
            }
        };

        self.calibration_statistics.update(observation.value)?;

        if self.calibration_statistics.count >= required {
            let baseline = self
                .calibration_statistics
                .baseline()
                .ok_or(DriftError::BaselineUnavailable)?;

            self.baseline = Some(baseline);
        }

        Ok(())
    }

    fn update_baseline(
        &mut self,
        value: f64,
        drift_observed: bool,
    ) -> Result<(), DriftError> {
        let BaselinePolicy::Adaptive { alpha } =
            self.config.baseline_policy
        else {
            return Ok(());
        };

        // Never adapt the baseline from an observation that currently
        // indicates drift. Otherwise a persistent failure could teach the
        // detector that the failed state is normal.
        if drift_observed {
            return Ok(());
        }

        let Some(previous) = self.baseline else {
            return Err(DriftError::BaselineUnavailable);
        };

        let a = alpha.get();

        let new_mean = previous.mean + a * (value - previous.mean);

        let deviation = value - new_mean;

        let old_variance = previous.variance;

        let new_variance =
            (1.0 - a) * (old_variance + a * deviation * deviation);

        if !new_mean.is_finite() || !new_variance.is_finite() {
            return Err(DriftError::NumericalFailure(
                "adaptive baseline became non-finite",
            ));
        }

        self.baseline = Some(DriftBaseline {
            sample_count: previous.sample_count.saturating_add(1),
            mean: new_mean,
            variance: new_variance.max(0.0),
        });

        Ok(())
    }

    fn update_state(
        &mut self,
        drift_observed: bool,
    ) -> Result<DriftEventKind, DriftError> {
        match self.state {
            DriftState::Normal => {
                self.negative_streak = 0;

                if drift_observed {
                    self.increment_positive_streak()?;

                    if self.confirmation_reached() {
                        self.state = DriftState::Drifted;
                        self.positive_streak = 0;

                        return Ok(DriftEventKind::DriftStarted);
                    }
                } else {
                    self.positive_streak = 0;
                }

                Ok(DriftEventKind::Normal)
            }

            DriftState::Drifted => {
                if drift_observed {
                    self.positive_streak = 0;
                    self.negative_streak = 0;

                    Ok(DriftEventKind::DriftContinues)
                } else {
                    self.increment_negative_streak()?;

                    if self.clearing_reached() {
                        self.state = DriftState::Normal;
                        self.negative_streak = 0;

                        Ok(DriftEventKind::DriftCleared)
                    } else {
                        Ok(DriftEventKind::DriftContinues)
                    }
                }
            }
        }
    }

    fn increment_positive_streak(&mut self) -> Result<(), DriftError> {
        if self.positive_streak == u64::MAX {
            return Err(DriftError::CounterOverflow);
        }

        self.positive_streak += 1;
        Ok(())
    }

    fn increment_negative_streak(&mut self) -> Result<(), DriftError> {
        if self.negative_streak == u64::MAX {
            return Err(DriftError::CounterOverflow);
        }

        self.negative_streak += 1;
        Ok(())
    }

    fn confirmation_reached(&self) -> bool {
        match self.config.confirmation {
            ConfirmationPolicy::Immediate => true,

            ConfirmationPolicy::Consecutive { required } => {
                self.positive_streak >= required.get()
            }
        }
    }

    fn clearing_reached(&self) -> bool {
        match self.config.clearing {
            ClearPolicy::Immediate => true,

            ClearPolicy::Consecutive { required } => {
                self.negative_streak >= required.get()
            }
        }
    }

    fn current_streak(&self) -> u64 {
        match self.state {
            DriftState::Normal => self.positive_streak,
            DriftState::Drifted => self.negative_streak,
        }
    }

    fn should_emit(
        &self,
        kind: DriftEventKind,
        drift_observed: bool,
    ) -> bool {
        match self.config.emission {
            EmissionPolicy::TransitionsOnly => {
                matches!(
                    kind,
                    DriftEventKind::DriftStarted
                        | DriftEventKind::DriftCleared
                )
            }

            EmissionPolicy::EveryDrift => drift_observed,

            EmissionPolicy::EveryEvaluation => true,
        }
    }
}

fn validate_positive_finite(
    value: f64,
    name: &'static str,
) -> Result<(), DriftError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(DriftError::InvalidConfiguration(name));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detector(
        config: DriftConfig,
        baseline: DriftBaseline,
    ) -> DriftDetector {
        DriftDetector::new_with_baseline(
            DriftDetectorId::new("test-detector").expect("valid id"),
            DriftMetricId::new("test.metric").expect("valid metric"),
            config,
            baseline,
        )
        .expect("valid detector")
    }

    fn observation(sequence: u64, value: f64) -> DriftObservation {
        DriftObservation::new(sequence, value).expect("valid observation")
    }

    #[test]
    fn rejects_non_finite_observation() {
        assert!(DriftObservation::new(0, f64::NAN).is_err());
        assert!(DriftObservation::new(0, f64::INFINITY).is_err());
        assert!(DriftObservation::new(0, f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn detects_absolute_drift() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let result = detector
            .observe(observation(1, 12.0))
            .expect("evaluation should succeed");

        assert!(result.is_some());

        let evaluation = result.expect("evaluation");

        assert_eq!(
            evaluation.kind,
            DriftEventKind::DriftStarted
        );
        assert_eq!(evaluation.current_state, DriftState::Drifted);
        assert!(evaluation.drift_observed);
    }

    #[test]
    fn does_not_trigger_below_threshold() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let result = detector
            .observe(observation(1, 10.5))
            .expect("evaluation should succeed")
            .expect("evaluation should be emitted");

        assert_eq!(result.kind, DriftEventKind::Normal);
        assert_eq!(result.current_state, DriftState::Normal);
        assert!(!result.drift_observed);
    }

    #[test]
    fn supports_consecutive_confirmation() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Consecutive {
                required: NonZeroU64::new(3).expect("nonzero"),
            },
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        assert!(
            detector
                .observe(observation(1, 12.0))
                .expect("valid")
                .is_none()
        );

        assert!(
            detector
                .observe(observation(2, 12.0))
                .expect("valid")
                .is_none()
        );

        let event = detector
            .observe(observation(3, 12.0))
            .expect("valid")
            .expect("third observation starts drift");

        assert_eq!(event.kind, DriftEventKind::DriftStarted);
        assert_eq!(detector.state(), DriftState::Drifted);
    }

    #[test]
    fn supports_consecutive_clearing() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Consecutive {
                required: NonZeroU64::new(2).expect("nonzero"),
            },
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let start = detector
            .observe(observation(1, 12.0))
            .expect("valid")
            .expect("drift start");

        assert_eq!(start.kind, DriftEventKind::DriftStarted);

        assert!(
            detector
                .observe(observation(2, 10.1))
                .expect("valid")
                .is_none()
        );

        let cleared = detector
            .observe(observation(3, 10.1))
            .expect("valid")
            .expect("drift clear");

        assert_eq!(cleared.kind, DriftEventKind::DriftCleared);
        assert_eq!(detector.state(), DriftState::Normal);
    }

    #[test]
    fn rejects_sequence_regression() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(10, 1.0, 0.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        detector
            .observe(observation(10, 1.0))
            .expect("first observation");

        let error = detector
            .observe(observation(9, 1.0))
            .expect_err("sequence regression must fail");

        assert_eq!(
            error,
            DriftError::SequenceRegression {
                previous: 10,
                current: 9
            }
        );
    }

    #[test]
    fn allows_equal_sequence_for_replay_mode() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::NonDecreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(10, 1.0, 0.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        assert!(
            detector
                .observe(observation(1, 1.0))
                .expect("valid")
                .is_some()
        );

        assert!(
            detector
                .observe(observation(1, 1.0))
                .expect("equal sequence is valid")
                .is_some()
        );
    }

    #[test]
    fn computes_standardized_drift() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Standardized {
                minimum_stddev: 0.5,
            },
            detection_threshold: 2.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let result = detector
            .observe(observation(1, 12.0))
            .expect("valid")
            .expect("evaluation");

        assert!((result.drift_magnitude - 2.0).abs() < 1e-12);
        assert!(result.drift_observed);
    }

    #[test]
    fn computes_relative_drift() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Relative {
                denominator_floor: 1.0,
            },
            detection_threshold: 0.1,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 0.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let result = detector
            .observe(observation(1, 11.0))
            .expect("valid")
            .expect("evaluation");

        assert!((result.drift_magnitude - 0.1).abs() < 1e-12);
        assert!(result.drift_observed);
    }

    #[test]
    fn adaptive_baseline_does_not_learn_drift() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Adaptive {
                alpha: DriftAlpha::new(0.5).expect("valid alpha"),
            },
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 0.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        detector
            .observe(observation(1, 12.0))
            .expect("valid");

        let baseline_after_drift =
            detector.baseline().expect("baseline remains");

        assert_eq!(baseline_after_drift.mean, 10.0);
    }

    #[test]
    fn adaptive_baseline_learns_normal_change() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Adaptive {
                alpha: DriftAlpha::new(0.5).expect("valid alpha"),
            },
            measure: DriftMeasure::Absolute,
            detection_threshold: 10.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(100, 10.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        detector
            .observe(observation(1, 12.0))
            .expect("valid");

        let updated =
            detector.baseline().expect("baseline remains");

        assert!(updated.mean > 10.0);
    }

    #[test]
    fn initial_baseline_is_streaming_and_bounded() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::FixedInitial {
                samples: NonZeroU64::new(3).expect("nonzero"),
            },
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let mut detector = DriftDetector::new(
            DriftDetectorId::new("initial").expect("valid"),
            DriftMetricId::new("metric").expect("valid"),
            config,
        )
        .expect("valid detector");

        assert!(
            detector
                .observe(observation(1, 10.0))
                .expect("valid")
                .is_none()
        );

        assert!(
            detector
                .observe(observation(2, 10.0))
                .expect("valid")
                .is_none()
        );

        assert!(
            detector
                .observe(observation(3, 10.0))
                .expect("valid")
                .is_none()
        );

        assert!(detector.baseline().is_some());

        let evaluation = detector
            .observe(observation(4, 12.0))
            .expect("valid")
            .expect("evaluation");

        assert!(evaluation.drift_observed);
    }

    #[test]
    fn stream_processing_does_not_require_internal_collection() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::EveryEvaluation,
        };

        let baseline =
            DriftBaseline::new(10, 0.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        let mut count = 0_u64;

        detector
            .process_stream(
                (0_u64..10_000).map(|sequence| {
                    observation(sequence, 0.5)
                }),
                |_| {
                    count += 1;
                },
            )
            .expect("stream should process");

        assert_eq!(count, 10_000);
    }

    #[test]
    fn reset_preserves_baseline() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(10, 0.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        detector
            .observe(observation(1, 2.0))
            .expect("valid");

        assert_eq!(detector.state(), DriftState::Drifted);

        detector.reset();

        assert_eq!(detector.state(), DriftState::Normal);
        assert!(detector.baseline().is_some());
        assert_eq!(detector.observations(), 0);
    }

    #[test]
    fn reset_calibration_removes_baseline() {
        let config = DriftConfig {
            baseline_policy: BaselinePolicy::Fixed,
            measure: DriftMeasure::Absolute,
            detection_threshold: 1.0,
            confirmation: ConfirmationPolicy::Immediate,
            clearing: ClearPolicy::Immediate,
            sequence_policy: SequencePolicy::StrictlyIncreasing,
            emission: EmissionPolicy::TransitionsOnly,
        };

        let baseline =
            DriftBaseline::new(10, 0.0, 1.0).expect("valid baseline");

        let mut detector = detector(config, baseline);

        detector.reset_calibration();

        assert!(detector.baseline().is_none());
        assert_eq!(detector.state(), DriftState::Normal);
    }

    #[test]
    fn validates_alpha() {
        assert!(DriftAlpha::new(0.0).is_err());
        assert!(DriftAlpha::new(-0.1).is_err());
        assert!(DriftAlpha::new(f64::NAN).is_err());
        assert!(DriftAlpha::new(f64::INFINITY).is_err());

        assert!(DriftAlpha::new(0.1).is_ok());
        assert!(DriftAlpha::new(1.0).is_ok());
    }

    #[test]
    fn rejects_invalid_baseline() {
        assert!(DriftBaseline::new(0, 0.0, 0.0).is_err());
        assert!(DriftBaseline::new(1, f64::NAN, 0.0).is_err());
        assert!(DriftBaseline::new(1, 0.0, -1.0).is_err());
    }
}