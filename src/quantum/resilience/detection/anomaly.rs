//! Zamani Quantum Resilience — Online Anomaly Detection.
//!
//! Path:
//!     src/quantum/resilience/detection/anomaly.rs
//!
//! # Purpose
//!
//! This module provides a deterministic, provider-neutral, streaming anomaly
//! detector for numerical resilience observations.
//!
//! The detector answers:
//!
//! > "Does the current numerical observation differ sufficiently from the
//! > established statistical baseline?"
//!
//! It does NOT answer:
//!
//! - what caused the anomaly;
//! - whether a physical qubit has failed;
//! - whether a backend is faulty;
//! - which recovery action should be taken;
//! - whether a circuit result is semantically correct;
//! - whether a quantum fault is present with certainty.
//!
//! Those responsibilities belong to diagnosis, policy, planning, recovery,
//! ZQN, QEC, and verification respectively.
//!
//! # Algorithm
//!
//! The detector uses an online Welford accumulator:
//!
//! ```text
//! count
//! mean
//! M2
//! ```
//!
//! For each numeric observation:
//!
//! 1. Validate the observation.
//! 2. Compare it against the baseline accumulated from preceding observations.
//! 3. Compute a standardized deviation when a variance estimate exists.
//! 4. Apply the configured anomaly threshold.
//! 5. Emit a normalized `DetectionSignal` when appropriate.
//! 6. Update the baseline according to the configured baseline policy.
//!
//! The current observation is therefore not allowed to influence its own
//! anomaly score before that score is calculated.
//!
//! This prevents an extreme observation from immediately masking itself.
//!
//! # Why Welford?
//!
//! Welford's online algorithm provides:
//!
//! - one-pass processing;
//! - constant detector state;
//! - numerically stable mean/variance updates;
//! - no fixed history window;
//! - no machine-size assumptions;
//! - deterministic processing;
//! - suitability for streaming telemetry.
//!
//! Memory usage is independent of the number of observations processed.
//!
//! # Scaling
//!
//! This implementation deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OBSERVATIONS
//! MAX_BACKENDS
//! MAX_HISTORY
//! MAX_WINDOW
//! ```
//!
//! A detector can process observations from a single qubit, a QPU, a
//! distributed quantum system, or a heterogeneous fleet without changing the
//! algorithm.
//!
//! Concrete execution remains limited only by resources available to the
//! caller.
//!
//! # Quantum resource identity
//!
//! This module does not define a quantum resource identifier.
//!
//! If the producer associates an observation with a quantum resource, it must
//! use the canonical Zamani IR types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The association belongs to the observation/resource model, not to the
//! numerical anomaly algorithm.
//!
//! # Determinism
//!
//! The detector:
//!
//! - does not read the system clock;
//! - does not generate random numbers;
//! - does not inspect environment variables;
//! - does not use global mutable state;
//! - does not depend on thread scheduling;
//! - does not depend on hash-map iteration order.
//!
//! Given identical:
//!
//! - detector configuration;
//! - detector state;
//! - observation sequence;
//! - detection context;
//!
//! it produces identical results.
//!
//! # No unsafe Rust
//!
//! This module forbids unsafe Rust and is intended for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration
//!
//! This module implements:
//!
//! ```text
//! crate::quantum::resilience::detection::detector::Detector
//! ```
//!
//! and consumes:
//!
//! ```text
//! DetectionInput
//! DetectionObservation
//! ObservationPayload
//! DetectionContext
//! DetectionSignal
//! DetectionOutput
//! DetectionMetadata
//! DetectionClassification
//! DetectionConfidence
//! DetectorIdentity
//! ```
//!
//! Errors are returned through:
//!
//! ```text
//! crate::quantum::resilience::errors::ResilienceError
//! ```
//!
//! The detector is intentionally independent of:
//!
//! ```text
//! diagnosis
//! policy
//! planning
//! adaptation
//! recovery
//! mitigation
//! verification
//! hardware providers
//! routing
//! scheduling
//! QEC implementations
//! ```
//!
//! Those layers consume the normalized signal produced here.
//!
//! # Statistical interpretation
//!
//! The standardized deviation is a detection score, not a physical fault
//! probability.
//!
//! In particular:
//!
//! ```text
//! z-score != fault probability
//! z-score != fidelity
//! z-score != logical error rate
//! z-score != recovery probability
//! ```
//!
//! The confidence emitted by this detector expresses confidence in the
//! configured statistical anomaly criterion, not physical certainty.
//!
//! # Baseline semantics
//!
//! Baseline observations are observations that are accepted into the running
//! statistical model.
//!
//! By default, anomalous observations are still incorporated into the baseline
//! after being detected. This permits the detector to adapt to persistent
//! distribution changes.
//!
//! Applications that require anomaly-preserving baselines can disable that
//! behavior through `AnomalyDetectorConfig::update_baseline_on_anomaly`.
//!
//! # Important operational rule
//!
//! The detector must normally be scoped to a homogeneous metric stream.
//!
//! For example, do NOT feed:
//!
//! ```text
//! gate fidelity
//! readout latency
//! queue time
//! qubit temperature
//!
//! ```
//!
//! into one detector state unless a higher-level model explicitly defines them
//! as the same statistical variable.
//!
//! A detector instance should normally correspond to one coherent metric
//! distribution, or to a partition explicitly maintained by the caller.
//!
//! # Missing/non-numeric observations
//!
//! This detector operates on numerical payloads.
//!
//! `Boolean`, `Text`, and `Marker` observations are ignored rather than
//! interpreted numerically.
//!
//! This is intentional. Anomaly detection must never silently assign arbitrary
//! numerical meanings to heterogeneous payload types.
//!
//! # Large integer values
//!
//! Integer payloads are converted to `f64` because the detector's statistical
//! accumulator is floating-point.
//!
//! Extremely large integers may therefore lose integer precision during
//! conversion. Such values remain valid observations, but callers requiring
//! exact integer-domain anomaly semantics should use a dedicated detector
//! rather than this floating-point statistical detector.
//!
//! # Numerical safety
//!
//! Non-finite input is rejected by the canonical detector observation
//! constructor. This module additionally guards all calculations against
//! non-finite intermediate results.
//!
//! Arithmetic overflow or invalid numerical state is reported as a resilience
//! error instead of being silently converted into an anomaly.
//!
//! # Production properties
//!
//! This implementation provides:
//!
//! - streaming processing;
//! - O(1) detector state;
//! - deterministic results;
//! - explicit configuration;
//! - configurable sensitivity;
//! - configurable minimum baseline size;
//! - configurable anomaly direction;
//! - configurable baseline adaptation;
//! - numerical validation;
//! - stale-data and trust-policy enforcement through `DetectionContext`;
//! - normalized resilience signals;
//! - integration with the existing detector contract;
//! - unit tests for numerical and lifecycle behavior;
//! - no unsafe code;
//! - no hardware-specific assumptions;
//! - no provider-specific assumptions.
//!
//! # Architecture
//!
//! ```text
//! telemetry / hardware / runtime / QEC / ZQN
//!                     |
//!                     v
//!             DetectionObservation
//!                     |
//!                     v
//!              AnomalyDetector
//!                     |
//!             +-------+-------+
//!             |               |
//!        baseline          score
//!             |               |
//!             +-------+-------+
//!                     |
//!                     v
//!              DetectionSignal
//!                     |
//!                     v
//!                 diagnosis
//! ```
//!
//! The anomaly detector is therefore a detector, not a resilience controller.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::num::NonZeroU64;

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionContext,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionPayload,
    DetectionSequence,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    ObservationPayload,
    SignalId,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

/// Stable schema identifier for this detector configuration and implementation.
pub const ANOMALY_DETECTOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.detection.anomaly";

/// Semantic version of the anomaly detector contract.
pub const ANOMALY_DETECTOR_SCHEMA_VERSION: u16 = 1;

/// Default detector implementation name.
///
/// This is an identity only. It is not a provider/backend identifier.
pub const ANOMALY_DETECTOR_NAME: &str = "online-statistical-anomaly";

/// Controls which direction of deviation constitutes an anomaly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AnomalyDirection {
    /// Detect unusually high values.
    High,

    /// Detect unusually low values.
    Low,

    /// Detect deviations in either direction.
    Both,
}

impl AnomalyDirection {
    /// Returns the stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
            Self::Both => "both",
        }
    }

    fn accepts(self, deviation: f64) -> bool {
        match self {
            Self::High => deviation > 0.0,
            Self::Low => deviation < 0.0,
            Self::Both => deviation != 0.0,
        }
    }
}

/// Defines what happens to an observation after it has been classified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BaselineUpdatePolicy {
    /// Always incorporate valid numeric observations.
    Always,

    /// Incorporate observations only when they were not classified as
    /// anomalies.
    ExcludeAnomalies,
}

impl BaselineUpdatePolicy {
    /// Returns the stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::ExcludeAnomalies => "exclude_anomalies",
        }
    }
}

/// Configuration for [`AnomalyDetector`].
///
/// The configuration contains no hardware-specific values.
///
/// All sensitivity and resource-related behavior is explicit so that a
/// deployment can supply its own policy rather than inheriting hidden
/// constants.
#[derive(Debug, Clone, PartialEq)]
pub struct AnomalyDetectorConfig {
    /// Minimum number of preceding numeric observations required before
    /// anomaly scoring begins.
    ///
    /// This is a policy/configuration value, not a machine-size limit.
    minimum_observations: NonZeroU64,

    /// Standardized-deviation threshold required for anomaly classification.
    ///
    /// Must be finite and strictly greater than zero.
    threshold: f64,

    /// Direction in which anomalies are detected.
    direction: AnomalyDirection,

    /// Controls whether anomalous observations update the baseline.
    baseline_update_policy: BaselineUpdatePolicy,
}

impl AnomalyDetectorConfig {
    /// Creates a validated anomaly detector configuration.
    pub fn new(
        minimum_observations: NonZeroU64,
        threshold: f64,
        direction: AnomalyDirection,
        baseline_update_policy: BaselineUpdatePolicy,
    ) -> ResilienceResult<Self> {
        if !threshold.is_finite() || threshold <= 0.0 {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
            ));
        }

        Ok(Self {
            minimum_observations,
            threshold,
            direction,
            baseline_update_policy,
        })
    }

    /// Returns a conservative general-purpose configuration.
    ///
    /// This constructor is provided for convenience only. It is not a
    /// hardware assumption and does not impose a system-wide resilience
    /// default.
    ///
    /// Applications with explicit resilience policy should construct the
    /// configuration themselves.
    pub fn standard() -> ResilienceResult<Self> {
        Self::new(
            NonZeroU64::new(2).ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::InvalidConfiguration)
            })?,
            3.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
    }

    /// Returns the minimum baseline observation count.
    #[must_use]
    pub const fn minimum_observations(&self) -> NonZeroU64 {
        self.minimum_observations
    }

    /// Returns the configured anomaly threshold.
    #[must_use]
    pub const fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Returns the configured direction.
    #[must_use]
    pub const fn direction(&self) -> AnomalyDirection {
        self.direction
    }

    /// Returns the baseline update policy.
    #[must_use]
    pub const fn baseline_update_policy(&self) -> BaselineUpdatePolicy {
        self.baseline_update_policy
    }
}

/// Online numerical baseline.
///
/// This is Welford's population-variance accumulator.
///
/// The accumulator deliberately uses only:
///
/// ```text
/// count
/// mean
/// M2
/// ```
///
/// regardless of the number of observations processed.
#[derive(Debug, Clone, Copy, PartialEq)]
struct RunningStatistics {
    count: u64,
    mean: f64,
    m2: f64,
}

impl RunningStatistics {
    /// Creates an empty accumulator.
    #[must_use]
    const fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    /// Returns the number of observations in the baseline.
    #[must_use]
    const fn count(self) -> u64 {
        self.count
    }

    /// Returns the current mean.
    #[must_use]
    const fn mean(self) -> f64 {
        self.mean
    }

    /// Returns the population variance when enough observations exist.
    fn variance(self) -> ResilienceResult<Option<f64>> {
        if self.count == 0 {
            return Ok(None);
        }

        let variance = self.m2 / self.count as f64;

        if !variance.is_finite() || variance < 0.0 {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DetectionInconsistent,
            ));
        }

        Ok(Some(variance))
    }

    /// Returns the population standard deviation.
    fn standard_deviation(self) -> ResilienceResult<Option<f64>> {
        match self.variance()? {
            Some(variance) => {
                let deviation = variance.sqrt();

                if !deviation.is_finite() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                    ));
                }

                Ok(Some(deviation))
            }
            None => Ok(None),
        }
    }

    /// Adds one finite observation.
    fn update(&mut self, value: f64) -> ResilienceResult<()> {
        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        if self.count == u64::MAX {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let next_count = self.count + 1;

        if self.count == 0 {
            self.count = 1;
            self.mean = value;
            self.m2 = 0.0;
            return Ok(());
        }

        let delta = value - self.mean;

        if !delta.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let next_mean = self.mean + delta / next_count as f64;

        if !next_mean.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let delta_after_mean = value - next_mean;

        if !delta_after_mean.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let next_m2 = self.m2 + delta * delta_after_mean;

        if !next_m2.is_finite() || next_m2 < 0.0 {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        self.count = next_count;
        self.mean = next_mean;
        self.m2 = next_m2;

        Ok(())
    }
}

/// A computed anomaly score for one observation.
///
/// The score is kept separate from `DetectionSignal` so that the detector's
/// numerical reasoning remains testable without coupling the tests to signal
/// construction.
#[derive(Debug, Clone, Copy, PartialEq)]
struct AnomalyScore {
    deviation: f64,
    standardized_deviation: Option<f64>,
    anomalous: bool,
    confidence: DetectionConfidence,
}

impl AnomalyScore {
    /// Returns the signed deviation from the baseline mean.
    #[must_use]
    const fn deviation(self) -> f64 {
        self.deviation
    }

    /// Returns the standardized deviation, when variance was available.
    #[must_use]
    const fn standardized_deviation(self) -> Option<f64> {
        self.standardized_deviation
    }

    /// Returns whether the observation is anomalous.
    #[must_use]
    const fn anomalous(self) -> bool {
        self.anomalous
    }

    /// Returns detector confidence.
    #[must_use]
    const fn confidence(self) -> DetectionConfidence {
        self.confidence
    }
}

/// Production online anomaly detector.
///
/// The detector is stateful. One instance should normally represent one
/// coherent statistical stream.
///
/// For multiple independent resources, create independent detector instances
/// or partition the higher-level detector registry by the canonical resource
/// identity.
#[derive(Debug, Clone)]
pub struct AnomalyDetector {
    identity: DetectorIdentity,
    config: AnomalyDetectorConfig,
    statistics: RunningStatistics,
}

impl AnomalyDetector {
    /// Creates an anomaly detector.
    pub fn new(
        identity: DetectorIdentity,
        config: AnomalyDetectorConfig,
    ) -> ResilienceResult<Self> {
        Ok(Self {
            identity,
            config,
            statistics: RunningStatistics::new(),
        })
    }

    /// Creates a detector using the convenience identity and standard
    /// configuration.
    pub fn standard() -> ResilienceResult<Self> {
        Self::new(
            DetectorIdentity::new(ANOMALY_DETECTOR_NAME, "1.0.0")?,
            AnomalyDetectorConfig::standard()?,
        )
    }

    /// Returns the detector configuration.
    #[must_use]
    pub const fn config(&self) -> &AnomalyDetectorConfig {
        &self.config
    }

    /// Returns the number of numeric observations currently in the baseline.
    #[must_use]
    pub const fn baseline_count(&self) -> u64 {
        self.statistics.count()
    }

    /// Returns the current baseline mean.
    ///
    /// `None` means that no numeric observation has yet been incorporated.
    #[must_use]
    pub const fn baseline_mean(&self) -> Option<f64> {
        if self.statistics.count() == 0 {
            None
        } else {
            Some(self.statistics.mean())
        }
    }

    /// Returns the current baseline standard deviation.
    pub fn baseline_standard_deviation(&self) -> ResilienceResult<Option<f64>> {
        self.statistics.standard_deviation()
    }

    /// Scores one value against the current baseline.
    ///
    /// The value is scored BEFORE it is incorporated into the baseline.
    fn score_value(&self, value: f64) -> ResilienceResult<AnomalyScore> {
        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        if self.statistics.count() < self.config.minimum_observations().get() {
            return Ok(AnomalyScore {
                deviation: 0.0,
                standardized_deviation: None,
                anomalous: false,
                confidence: DetectionConfidence::zero(),
            });
        }

        let mean = self.statistics.mean();

        if !mean.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DetectionInconsistent,
            ));
        }

        let deviation = value - mean;

        if !deviation.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let standard_deviation = self
            .statistics
            .standard_deviation()?
            .ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::DetectionInconclusive)
            })?;

        // A zero variance means that all baseline observations have been
        // identical. In that situation, any non-zero deviation is evidence of
        // a distribution change, but a standardized score cannot be computed.
        if standard_deviation == 0.0 {
            let anomalous = deviation != 0.0 && self.config.direction.accepts(deviation);

            let confidence = if anomalous {
                DetectionConfidence::full()
            } else {
                DetectionConfidence::zero()
            };

            return Ok(AnomalyScore {
                deviation,
                standardized_deviation: None,
                anomalous,
                confidence,
            });
        }

        let standardized_deviation = deviation / standard_deviation;

        if !standardized_deviation.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        let magnitude = standardized_deviation.abs();

        let direction_matches = self.config.direction.accepts(deviation);
        let anomalous = direction_matches && magnitude >= self.config.threshold();

        // This is a monotonic confidence mapping from standardized deviation
        // to [0,1]. It is deliberately not presented as a probability of a
        // physical fault.
        let confidence_value = if anomalous {
            let normalized = magnitude / self.config.threshold();

            if !normalized.is_finite() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                ));
            }

            let confidence = 1.0 - (-normalized).exp();

            if !confidence.is_finite() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                ));
            }

            confidence.clamp(0.0, 1.0)
        } else {
            0.0
        };

        let confidence = DetectionConfidence::new(confidence_value)?;

        Ok(AnomalyScore {
            deviation,
            standardized_deviation: Some(standardized_deviation),
            anomalous,
            confidence,
        })
    }

    /// Extracts a numerical value from the canonical detector observation.
    ///
    /// Non-numeric observations are intentionally ignored.
    fn numeric_value(observation: &DetectionObservation) -> Option<f64> {
        match observation.payload() {
            ObservationPayload::Integer(value) => Some(*value as f64),
            ObservationPayload::Unsigned(value) => Some(*value as f64),
            ObservationPayload::Float(value) => Some(*value),
            ObservationPayload::Boolean(_)
            | ObservationPayload::Text(_)
            | ObservationPayload::Marker => None,
        }
    }

    /// Creates the deterministic signal identity for an anomalous observation.
    ///
    /// The detector emits at most one anomaly signal for a given observation,
    /// so the canonical observation ID is sufficient as the local signal ID.
    ///
    /// Detector identity remains part of `DetectionSignal`, preventing the
    /// detector from claiming global ownership of the numeric ID.
    fn signal_id(observation: &DetectionObservation) -> ResilienceResult<SignalId> {
        SignalId::from_u64(observation.id().value()).ok_or_else(|| {
            ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
        })
    }

    /// Evaluates one observation.
    ///
    /// Returns `Some(signal)` when the observation is anomalous and `None`
    /// otherwise.
    fn evaluate_observation(
        &mut self,
        context: &DetectionContext,
        observation: &DetectionObservation,
    ) -> ResilienceResult<Option<DetectionSignal>> {
        if observation.sequence() != context.sequence() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DetectionInconsistent,
            ));
        }

        if context.require_verified_observations() && !observation.trust().is_verified() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::UntrustedObservation,
            ));
        }

        if !context.allow_stale_observations() && observation.freshness().is_stale() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DetectionDataStale,
            ));
        }

        let Some(value) = Self::numeric_value(observation) else {
            return Ok(None);
        };

        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        let score = self.score_value(value)?;

        let should_update = match self.config.baseline_update_policy() {
            BaselineUpdatePolicy::Always => true,
            BaselineUpdatePolicy::ExcludeAnomalies => !score.anomalous(),
        };

        if should_update {
            self.statistics.update(value)?;
        }

        if !score.anomalous() {
            return Ok(None);
        }

        let signal_id = Self::signal_id(observation)?;

        Ok(Some(DetectionSignal::new(
            signal_id,
            self.identity.clone(),
            DetectionClassification::Anomaly,
            score.confidence(),
            Some(observation.id()),
            observation.sequence(),
        )))
    }

    /// Returns the signed deviation of a value from the current baseline.
    ///
    /// This is primarily useful to diagnostic/testing integrations that want
    /// to inspect the detector's numerical state without modifying it.
    pub fn deviation_from_baseline(&self, value: f64) -> ResilienceResult<Option<f64>> {
        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        if self.statistics.count() == 0 {
            return Ok(None);
        }

        let deviation = value - self.statistics.mean();

        if !deviation.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        Ok(Some(deviation))
    }

    /// Returns the standardized deviation of a value from the current
    /// baseline.
    pub fn standardized_deviation_from_baseline(
        &self,
        value: f64,
    ) -> ResilienceResult<Option<f64>> {
        if !value.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        if self.statistics.count() == 0 {
            return Ok(None);
        }

        let standard_deviation = self.statistics.standard_deviation()?;

        let Some(standard_deviation) = standard_deviation else {
            return Ok(None);
        };

        if standard_deviation == 0.0 {
            return Ok(None);
        }

        let standardized = (value - self.statistics.mean()) / standard_deviation;

        if !standardized.is_finite() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
            ));
        }

        Ok(Some(standardized))
    }
}

impl Detector for AnomalyDetector {
    fn identity(&self) -> &DetectorIdentity {
        &self.identity
    }

    fn detect<'a, I>(
        &mut self,
        input: DetectionInput<'a, I>,
    ) -> ResilienceResult<DetectionOutput>
    where
        I: Iterator<Item = &'a DetectionObservation>,
    {
        let context = input.context().clone();

        let mut observations_examined = 0_u64;
        let mut signals = Vec::new();

        for observation in input.observations() {
            observations_examined = observations_examined.checked_add(1).ok_or_else(|| {
                ResilienceError::new(ResilienceErrorCode::ArithmeticOverflow)
            })?;

            if let Some(signal) = self.evaluate_observation(&context, observation)? {
                signals.push(signal);
            }
        }

        let metadata = DetectionMetadata::new(
            self.identity.clone(),
            context.sequence(),
            observations_examined,
        );

        Ok(DetectionOutput::new(metadata, signals))
    }

    fn is_available(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        self.statistics = RunningStatistics::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::from_u64(value).expect("sequence must be non-zero")
    }

    fn observation(
        id: u64,
        value: f64,
    ) -> DetectionObservation {
        DetectionObservation::new(
            crate::quantum::resilience::detection::detector::ObservationId::from_u64(id)
                .expect("observation ID must be non-zero"),
            sequence(1),
            crate::quantum::resilience::detection::detector::ObservationSource::Runtime,
            crate::quantum::resilience::detection::detector::ObservationTrust::Verified,
            crate::quantum::resilience::detection::detector::ObservationFreshness::Fresh,
            ObservationPayload::Float(value),
        )
        .expect("observation must be valid")
    }

    fn context() -> DetectionContext {
        DetectionContext::new(sequence(1), false, true)
    }

    fn detector() -> AnomalyDetector {
        let config = AnomalyDetectorConfig::new(
            NonZeroU64::new(3).expect("minimum must be non-zero"),
            3.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::ExcludeAnomalies,
        )
        .expect("configuration must be valid");

        AnomalyDetector::new(
            DetectorIdentity::new("test-anomaly", "1.0.0")
                .expect("identity must be valid"),
            config,
        )
        .expect("detector must be valid")
    }

    #[test]
    fn configuration_rejects_non_positive_threshold() {
        let result = AnomalyDetectorConfig::new(
            NonZeroU64::new(1).expect("non-zero"),
            0.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        );

        assert!(result.is_err());
    }

    #[test]
    fn configuration_rejects_non_finite_threshold() {
        let result = AnomalyDetectorConfig::new(
            NonZeroU64::new(1).expect("non-zero"),
            f64::NAN,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        );

        assert!(result.is_err());
    }

    #[test]
    fn running_statistics_are_online() {
        let mut statistics = RunningStatistics::new();

        statistics.update(1.0).expect("update");
        statistics.update(2.0).expect("update");
        statistics.update(3.0).expect("update");

        assert_eq!(statistics.count(), 3);
        assert!((statistics.mean() - 2.0).abs() < 1.0e-12);

        let variance = statistics
            .variance()
            .expect("variance calculation")
            .expect("variance exists");

        assert!((variance - (2.0 / 3.0)).abs() < 1.0e-12);
    }

    #[test]
    fn minimum_observations_create_warmup_period() {
        let detector = detector();

        assert_eq!(detector.baseline_count(), 0);
        assert_eq!(detector.baseline_mean(), None);
    }

    #[test]
    fn anomaly_is_not_scored_against_itself() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 10.0),
            observation(3, 10.0),
            observation(4, 100.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection should succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].observation_id(),
            crate::quantum::resilience::detection::detector::ObservationId::from_u64(4)
        );
    }

    #[test]
    fn normal_values_do_not_emit_signals() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 11.0),
            observation(3, 10.0),
            observation(4, 11.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection should succeed");

        assert!(output.is_empty());
    }

    #[test]
    fn high_direction_ignores_low_anomalies() {
        let config = AnomalyDetectorConfig::new(
            NonZeroU64::new(3).expect("non-zero"),
            2.0,
            AnomalyDirection::High,
            BaselineUpdatePolicy::ExcludeAnomalies,
        )
        .expect("configuration");

        let mut detector = AnomalyDetector::new(
            DetectorIdentity::new("high-only", "1.0.0").expect("identity"),
            config,
        )
        .expect("detector");

        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 10.0),
            observation(3, 10.0),
            observation(4, 0.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert!(output.is_empty());
    }

    #[test]
    fn low_direction_detects_low_anomalies() {
        let config = AnomalyDetectorConfig::new(
            NonZeroU64::new(3).expect("non-zero"),
            2.0,
            AnomalyDirection::Low,
            BaselineUpdatePolicy::ExcludeAnomalies,
        )
        .expect("configuration");

        let mut detector = AnomalyDetector::new(
            DetectorIdentity::new("low-only", "1.0.0").expect("identity"),
            config,
        )
        .expect("detector");

        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 10.0),
            observation(3, 10.0),
            observation(4, 0.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn constant_baseline_detects_distribution_change() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 5.0),
            observation(2, 5.0),
            observation(3, 5.0),
            observation(4, 7.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::Anomaly
        );
    }

    #[test]
    fn anomaly_confidence_is_normalized() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 11.0),
            observation(3, 9.0),
            observation(4, 100.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(output.len(), 1);

        let confidence = output.signals()[0].confidence().value();

        assert!((0.0..=1.0).contains(&confidence));
    }

    #[test]
    fn non_numeric_payloads_are_ignored() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            DetectionObservation::new(
                crate::quantum::resilience::detection::detector::ObservationId::from_u64(1)
                    .expect("ID"),
                sequence(1),
                crate::quantum::resilience::detection::detector::ObservationSource::Runtime,
                crate::quantum::resilience::detection::detector::ObservationTrust::Verified,
                crate::quantum::resilience::detection::detector::ObservationFreshness::Fresh,
                ObservationPayload::Text("not numeric".to_owned()),
            )
            .expect("observation"),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert!(output.is_empty());
        assert_eq!(detector.baseline_count(), 0);
    }

    #[test]
    fn stale_observations_are_rejected_by_context() {
        let mut detector = detector();

        let stale = DetectionObservation::new(
            crate::quantum::resilience::detection::detector::ObservationId::from_u64(1)
                .expect("ID"),
            sequence(1),
            crate::quantum::resilience::detection::detector::ObservationSource::Runtime,
            crate::quantum::resilience::detection::detector::ObservationTrust::Verified,
            crate::quantum::resilience::detection::detector::ObservationFreshness::Stale,
            ObservationPayload::Float(10.0),
        )
        .expect("observation");

        let result = detector.detect(DetectionInput::new(
            &context(),
            [stale].iter(),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn untrusted_observations_are_rejected_when_required() {
        let mut detector = detector();

        let observation = DetectionObservation::new(
            crate::quantum::resilience::detection::detector::ObservationId::from_u64(1)
                .expect("ID"),
            sequence(1),
            crate::quantum::resilience::detection::detector::ObservationSource::Runtime,
            crate::quantum::resilience::detection::detector::ObservationTrust::Unverified,
            crate::quantum::resilience::detection::detector::ObservationFreshness::Fresh,
            ObservationPayload::Float(10.0),
        )
        .expect("observation");

        let result = detector.detect(DetectionInput::new(
            &context(),
            [observation].iter(),
        ));

        assert!(result.is_err());
    }

    #[test]
    fn reset_clears_baseline() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 1.0),
            observation(2, 2.0),
            observation(3, 3.0),
        ];

        detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(detector.baseline_count(), 3);

        detector.reset();

        assert_eq!(detector.baseline_count(), 0);
        assert_eq!(detector.baseline_mean(), None);
    }

    #[test]
    fn detector_identity_is_preserved_in_signal() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 10.0),
            observation(3, 10.0),
            observation(4, 100.0),
        ];

        let output = detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(
            output.signals()[0].detector().name(),
            "test-anomaly"
        );
    }

    #[test]
    fn exclude_anomalies_preserves_baseline() {
        let mut detector = detector();
        let context = context();

        let observations = vec![
            observation(1, 10.0),
            observation(2, 10.0),
            observation(3, 10.0),
            observation(4, 100.0),
        ];

        detector
            .detect(DetectionInput::new(&context, observations.iter()))
            .expect("detection");

        assert_eq!(detector.baseline_count(), 4);

        let mean = detector.baseline_mean().expect("baseline mean");

        // The anomalous value is excluded, so the baseline remains at 10.
        assert!((mean - 10.0).abs() < 1.0e-12);
    }

    #[test]
    fn wrong_sequence_is_rejected() {
        let mut detector = detector();
        let context = context();

        let wrong_sequence = DetectionObservation::new(
            crate::quantum::resilience::detection::detector::ObservationId::from_u64(1)
                .expect("ID"),
            sequence(2),
            crate::quantum::resilience::detection::detector::ObservationSource::Runtime,
            crate::quantum::resilience::detection::detector::ObservationTrust::Verified,
            crate::quantum::resilience::detection::detector::ObservationFreshness::Fresh,
            ObservationPayload::Float(1.0),
        )
        .expect("observation");

        let result = detector.detect(DetectionInput::new(
            &context,
            [wrong_sequence].iter(),
        ));

        assert!(result.is_err());
    }
}