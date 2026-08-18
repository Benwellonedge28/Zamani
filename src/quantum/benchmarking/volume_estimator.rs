//! Zamani Quantum — Production Quantum Volume Estimator
//!
//! Provides deterministic, validated quantum-volume calculations and
//! benchmark-result analysis.
//!
//! Quantum Volume is conventionally reported as:
//!
//!     QV = 2^m
//!
//! where `m` is the largest tested circuit width/depth for which the
//! measured heavy-output probability exceeds the benchmark threshold.
//!
//! IMPORTANT:
//! This module does not pretend to execute quantum circuits. It provides the
//! benchmarking mathematics and result validation layer. Actual circuit
//! generation, execution, sampling, and hardware interaction belong to the
//! quantum backend/benchmarking pipeline.

use std::fmt;

/// Default heavy-output probability threshold used by the estimator.
///
/// A benchmark is considered successful when the measured heavy-output
/// probability is strictly greater than this threshold.
pub const DEFAULT_HEAVY_OUTPUT_THRESHOLD: f64 = 2.0 / 3.0;

/// Default confidence level for statistical reporting.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Maximum exponent representable by `usize` on the current platform.
///
/// We do not permit unchecked `1 << exponent` operations because that could
/// panic or silently overflow on large quantum systems.
fn max_usize_exponent() -> usize {
    usize::BITS as usize - 1
}

/// Errors produced by the Quantum Volume estimator.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantumVolumeError {
    InvalidQubitCount,

    InvalidGateDepth,

    InvalidThreshold,

    InvalidConfidenceLevel,

    InvalidSampleCount,

    InvalidHeavyOutputCount,

    HeavyOutputExceedsSamples,

    ExponentOverflow {
        exponent: usize,
    },

    NonFiniteProbability,
}

impl fmt::Display for QuantumVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount => {
                write!(f, "quantum volume requires at least one qubit")
            }

            Self::InvalidGateDepth => {
                write!(f, "quantum volume requires a gate depth greater than zero")
            }

            Self::InvalidThreshold => {
                write!(
                    f,
                    "heavy-output threshold must be finite and in the range [0, 1]"
                )
            }

            Self::InvalidConfidenceLevel => {
                write!(
                    f,
                    "confidence level must be finite and strictly between 0 and 1"
                )
            }

            Self::InvalidSampleCount => {
                write!(f, "benchmark sample count must be greater than zero")
            }

            Self::InvalidHeavyOutputCount => {
                write!(f, "heavy-output count cannot be negative")
            }

            Self::HeavyOutputExceedsSamples => {
                write!(
                    f,
                    "heavy-output count cannot exceed total sample count"
                )
            }

            Self::ExponentOverflow { exponent } => {
                write!(
                    f,
                    "quantum-volume exponent {} cannot be represented by usize",
                    exponent
                )
            }

            Self::NonFiniteProbability => {
                write!(f, "measured probability must be finite")
            }
        }
    }
}

impl std::error::Error for QuantumVolumeError {}

/// Configuration for a Quantum Volume benchmark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeConfig {
    /// Number of logical/physical qubits tested.
    pub num_qubits: usize,

    /// Depth of the randomized quantum circuits.
    pub gate_depth: usize,

    /// Heavy-output probability threshold.
    pub heavy_output_threshold: f64,

    /// Statistical confidence level.
    pub confidence_level: f64,
}

impl QuantumVolumeConfig {
    /// Creates a configuration using the production defaults.
    pub fn new(
        num_qubits: usize,
        gate_depth: usize,
    ) -> Result<Self, QuantumVolumeError> {
        Self::with_threshold(
            num_qubits,
            gate_depth,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        )
    }

    /// Creates a configuration with an explicit heavy-output threshold.
    pub fn with_threshold(
        num_qubits: usize,
        gate_depth: usize,
        heavy_output_threshold: f64,
    ) -> Result<Self, QuantumVolumeError> {
        let config = Self {
            num_qubits,
            gate_depth,
            heavy_output_threshold,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
        };

        config.validate()?;

        Ok(config)
    }

    /// Validates the benchmark configuration.
    pub fn validate(&self) -> Result<(), QuantumVolumeError> {
        if self.num_qubits == 0 {
            return Err(QuantumVolumeError::InvalidQubitCount);
        }

        if self.gate_depth == 0 {
            return Err(QuantumVolumeError::InvalidGateDepth);
        }

        if !self.heavy_output_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.heavy_output_threshold)
        {
            return Err(QuantumVolumeError::InvalidThreshold);
        }

        if !self.confidence_level.is_finite()
            || !(0.0..1.0).contains(&self.confidence_level)
        {
            return Err(QuantumVolumeError::InvalidConfidenceLevel);
        }

        Ok(())
    }

    /// Returns the benchmark width/depth exponent.
    pub fn exponent(&self) -> usize {
        self.num_qubits.min(self.gate_depth)
    }

    /// Returns the theoretical maximum QV represented by this configuration.
    ///
    /// Returns an error rather than overflowing.
    pub fn theoretical_volume(&self) -> Result<usize, QuantumVolumeError> {
        let exponent = self.exponent();

        if exponent > max_usize_exponent() {
            return Err(QuantumVolumeError::ExponentOverflow { exponent });
        }

        Ok(1usize << exponent)
    }
}

/// Statistical result from an executed Quantum Volume benchmark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeResult {
    /// Number of benchmark shots/samples.
    pub samples: usize,

    /// Number of samples classified as heavy outputs.
    pub heavy_outputs: usize,

    /// Measured heavy-output probability.
    pub heavy_output_probability: f64,

    /// Lower bound of the confidence interval.
    pub confidence_lower: f64,

    /// Upper bound of the confidence interval.
    pub confidence_upper: f64,

    /// Whether the benchmark passed its threshold.
    pub passed: bool,

    /// Quantum Volume if the benchmark passed.
    pub quantum_volume: Option<usize>,

    /// Width/depth exponent.
    pub exponent: usize,
}

impl QuantumVolumeResult {
    /// Creates a benchmark result from raw measurement counts.
    pub fn from_samples(
        config: QuantumVolumeConfig,
        samples: usize,
        heavy_outputs: usize,
    ) -> Result<Self, QuantumVolumeError> {
        config.validate()?;

        if samples == 0 {
            return Err(QuantumVolumeError::InvalidSampleCount);
        }

        if heavy_outputs > samples {
            return Err(QuantumVolumeError::HeavyOutputExceedsSamples);
        }

        let probability = heavy_outputs as f64 / samples as f64;

        Self::from_probability(config, samples, probability)
    }

    /// Creates a benchmark result from a measured heavy-output probability.
    ///
    /// This is useful when a backend has already performed sampling and
    /// statistical processing.
    pub fn from_probability(
        config: QuantumVolumeConfig,
        samples: usize,
        probability: f64,
    ) -> Result<Self, QuantumVolumeError> {
        config.validate()?;

        if samples == 0 {
            return Err(QuantumVolumeError::InvalidSampleCount);
        }

        if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
            return Err(QuantumVolumeError::NonFiniteProbability);
        }

        let (lower, upper) =
            wilson_interval(probability, samples, config.confidence_level);

        let passed = lower > config.heavy_output_threshold;

        let quantum_volume = if passed {
            Some(config.theoretical_volume()?)
        } else {
            None
        };

        Ok(Self {
            samples,
            heavy_outputs: probability_to_count(probability, samples),
            heavy_output_probability: probability,
            confidence_lower: lower,
            confidence_upper: upper,
            passed,
            quantum_volume,
            exponent: config.exponent(),
        })
    }
}

/// Production Quantum Volume estimator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantumVolumeEstimator {
    pub num_qubits: usize,
    pub gate_depth: usize,
    pub heavy_output_threshold: f64,
    pub confidence_level: f64,
}

impl QuantumVolumeEstimator {
    /// Creates a production estimator using the default threshold and
    /// confidence level.
    ///
    /// This constructor preserves the API shape of the previous
    /// implementation while validating its parameters.
    ///
    /// Invalid parameters result in a panic. For fallible production code,
    /// prefer [`QuantumVolumeEstimator::try_new`].
    pub fn new(num_qubits: usize, gate_depth: usize) -> Self {
        Self::try_new(num_qubits, gate_depth)
            .expect("invalid quantum volume estimator configuration")
    }

    /// Fallible production constructor.
    pub fn try_new(
        num_qubits: usize,
        gate_depth: usize,
    ) -> Result<Self, QuantumVolumeError> {
        let config = QuantumVolumeConfig::new(num_qubits, gate_depth)?;

        Ok(Self {
            num_qubits: config.num_qubits,
            gate_depth: config.gate_depth,
            heavy_output_threshold: config.heavy_output_threshold,
            confidence_level: config.confidence_level,
        })
    }

    /// Creates an estimator with a custom heavy-output threshold.
    pub fn with_threshold(
        num_qubits: usize,
        gate_depth: usize,
        threshold: f64,
    ) -> Result<Self, QuantumVolumeError> {
        let config =
            QuantumVolumeConfig::with_threshold(num_qubits, gate_depth, threshold)?;

        Ok(Self {
            num_qubits: config.num_qubits,
            gate_depth: config.gate_depth,
            heavy_output_threshold: config.heavy_output_threshold,
            confidence_level: config.confidence_level,
        })
    }

    /// Returns the width/depth exponent used for QV.
    pub fn exponent(&self) -> usize {
        self.num_qubits.min(self.gate_depth)
    }

    /// Calculates the theoretical Quantum Volume.
    ///
    /// This preserves the behavior of the original implementation but
    /// performs checked arithmetic.
    pub fn estimate_quantum_volume(&self) -> usize {
        match self.try_estimate_quantum_volume() {
            Ok(volume) => volume,
            Err(error) => {
                eprintln!(
                    "[QuantumVolume] Unable to calculate theoretical QV: {}",
                    error
                );
                0
            }
        }
    }

    /// Fallible version of [`estimate_quantum_volume`].
    pub fn try_estimate_quantum_volume(
        &self,
    ) -> Result<usize, QuantumVolumeError> {
        self.config().theoretical_volume()
    }

    /// Creates the benchmark configuration represented by this estimator.
    pub fn config(&self) -> QuantumVolumeConfig {
        QuantumVolumeConfig {
            num_qubits: self.num_qubits,
            gate_depth: self.gate_depth,
            heavy_output_threshold: self.heavy_output_threshold,
            confidence_level: self.confidence_level,
        }
    }

    /// Evaluates measured benchmark samples.
    pub fn evaluate(
        &self,
        samples: usize,
        heavy_outputs: usize,
    ) -> Result<QuantumVolumeResult, QuantumVolumeError> {
        QuantumVolumeResult::from_samples(
            self.config(),
            samples,
            heavy_outputs,
        )
    }

    /// Evaluates a previously calculated heavy-output probability.
    pub fn evaluate_probability(
        &self,
        samples: usize,
        probability: f64,
    ) -> Result<QuantumVolumeResult, QuantumVolumeError> {
        QuantumVolumeResult::from_probability(
            self.config(),
            samples,
            probability,
        )
    }

    /// Returns whether a measured probability exceeds the configured
    /// threshold.
    pub fn passes_threshold(&self, probability: f64) -> bool {
        probability.is_finite()
            && probability > self.heavy_output_threshold
    }
}

/// Calculates a Wilson score confidence interval for a binomial proportion.
///
/// Wilson intervals are preferable to the naive `p ± z*sqrt(...)` interval
/// for small samples or probabilities near 0/1.
fn wilson_interval(
    probability: f64,
    samples: usize,
    confidence_level: f64,
) -> (f64, f64) {
    let z = inverse_normal_cdf(
        0.5 + confidence_level / 2.0,
    );

    let n = samples as f64;
    let z_squared = z * z;

    let denominator = 1.0 + z_squared / n;

    let center =
        (probability + z_squared / (2.0 * n)) / denominator;

    let margin = z
        * ((probability * (1.0 - probability) / n)
            + (z_squared / (4.0 * n * n)))
            .sqrt()
        / denominator;

    (
        (center - margin).clamp(0.0, 1.0),
        (center + margin).clamp(0.0, 1.0),
    )
}

/// Converts a probability into a deterministic sample count.
///
/// This is used only for result metadata when the backend supplies a
/// probability instead of raw counts.
fn probability_to_count(
    probability: f64,
    samples: usize,
) -> usize {
    (probability * samples as f64).round() as usize
}

/// Approximation of the inverse standard-normal CDF.
///
/// Uses the Acklam rational approximation. No external numerical dependency
/// is required.
fn inverse_normal_cdf(p: f64) -> f64 {
    const A: [f64; 6] = [
        -39.69683028665376,
        220.9460984245205,
        -275.9285104469687,
        138.3577518672690,
        -30.66479806614716,
        2.506628277459239,
    ];

    const B: [f64; 5] = [
        -54.47609879822406,
        161.5858368580409,
        -155.6989798598866,
        66.80131188771972,
        -13.28068155288572,
    ];

    const C: [f64; 6] = [
        -0.007784894002430293,
        -0.3223964580411365,
        -2.400758277161838,
        -2.549732539343734,
        4.374664141464968,
        2.938163982698783,
    ];

    const D: [f64; 4] = [
        0.007784695709041462,
        0.3224671290700398,
        2.445134137142996,
        3.754408661907416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }

    if p >= 1.0 {
        return f64::INFINITY;
    }

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        return (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0);
    }

    if p > HIGH {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        return -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q
                + 1.0);
    }

    let q = p - 0.5;
    let r = q * q;

    (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4])
        * r
        + A[5])
        * q)
        / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4])
            * r)
            + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theoretical_qv_matches_standard_formula() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        assert_eq!(
            estimator.estimate_quantum_volume(),
            32
        );
    }

    #[test]
    fn qv_uses_smaller_of_width_and_depth() {
        let estimator = QuantumVolumeEstimator::new(10, 4);

        assert_eq!(estimator.exponent(), 4);
        assert_eq!(estimator.estimate_quantum_volume(), 16);
    }

    #[test]
    fn zero_qubits_are_rejected() {
        assert!(
            QuantumVolumeEstimator::try_new(0, 5).is_err()
        );
    }

    #[test]
    fn zero_depth_is_rejected() {
        assert!(
            QuantumVolumeEstimator::try_new(5, 0).is_err()
        );
    }

    #[test]
    fn invalid_threshold_is_rejected() {
        assert!(
            QuantumVolumeEstimator::with_threshold(
                5,
                5,
                1.5
            )
            .is_err()
        );
    }

    #[test]
    fn sample_count_is_validated() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        assert!(
            estimator.evaluate(0, 0).is_err()
        );
    }

    #[test]
    fn heavy_outputs_cannot_exceed_samples() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        assert!(
            estimator.evaluate(100, 101).is_err()
        );
    }

    #[test]
    fn perfect_heavy_output_passes() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        let result = estimator
            .evaluate(1000, 1000)
            .expect("benchmark should be valid");

        assert!(result.passed);
        assert_eq!(result.quantum_volume, Some(32));
        assert_eq!(result.heavy_output_probability, 1.0);
    }

    #[test]
    fn zero_heavy_outputs_fail() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        let result = estimator
            .evaluate(1000, 0)
            .expect("benchmark should be valid");

        assert!(!result.passed);
        assert_eq!(result.quantum_volume, None);
    }

    #[test]
    fn confidence_interval_is_bounded() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        let result = estimator
            .evaluate(1000, 700)
            .expect("benchmark should be valid");

        assert!(result.confidence_lower >= 0.0);
        assert!(result.confidence_upper <= 1.0);
        assert!(
            result.confidence_lower
                <= result.heavy_output_probability
        );
        assert!(
            result.confidence_upper
                >= result.heavy_output_probability
        );
    }

    #[test]
    fn threshold_check_rejects_non_finite_values() {
        let estimator = QuantumVolumeEstimator::new(5, 5);

        assert!(!estimator.passes_threshold(f64::NAN));
        assert!(!estimator.passes_threshold(f64::INFINITY));
        assert!(!estimator.passes_threshold(f64::NEG_INFINITY));
    }

    #[test]
    fn probability_evaluation_works() {
        let estimator = QuantumVolumeEstimator::new(3, 3);

        let result = estimator
            .evaluate_probability(10_000, 0.75)
            .expect("probability should be valid");

        assert!(result.passed);
        assert_eq!(result.quantum_volume, Some(8));
    }

    #[test]
    fn wilson_interval_is_valid_for_extreme_probabilities() {
        let low = wilson_interval(0.0, 100, 0.95);
        let high = wilson_interval(1.0, 100, 0.95);

        assert!(low.0 >= 0.0);
        assert!(low.1 <= 1.0);
        assert!(high.0 >= 0.0);
        assert!(high.1 <= 1.0);
    }
}