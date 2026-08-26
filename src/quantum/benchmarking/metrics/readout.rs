//! Zamani Quantum Benchmarking — Readout Metrics
//!
//! Production-grade measurement/readout characterization and analysis.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - readout/measurement error metrics;
//! - binary assignment/confusion matrices;
//! - empirical readout calibration analysis;
//! - confidence intervals for readout probabilities;
//! - aggregate readout-fidelity metrics;
//! - per-qubit readout metrics;
//! - validation of raw readout counts;
//! - deterministic conversion between counts and probabilities;
//! - conservative single-qubit readout-error correction;
//! - machine-readable result structures;
//! - explicit provenance of whether a metric came from calibration data or
//!   benchmark samples.
//!
//! This module DOES NOT own:
//!
//! - physical-device communication;
//! - backend execution;
//! - qubit calibration acquisition;
//! - circuit generation;
//! - Quantum IR;
//! - frontend parsing;
//! - routing;
//! - scheduling;
//! - tomography;
//! - statistical experiment orchestration.
//!
//! Those responsibilities belong to their respective Zamani subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::frontend
//!        |
//!        v
//! quantum::ir
//!        |
//!        v
//! quantum::benchmarking
//!        |
//!        +---- execution
//!        |
//!        +---- statistics
//!        |
//!        +---- metrics::readout  <--- THIS MODULE
//!        |
//!        v
//! quantum::hardware
//! ```
//!
//! The benchmarking subsystem may consume hardware calibration data, but the
//! hardware subsystem must never depend on this module.
//!
//! # Readout matrix convention
//!
//! All assignment matrices use the following convention:
//!
//! ```text
//!                 measured 0     measured 1
//! true 0              P00            P01
//! true 1              P10            P11
//! ```
//!
//! where:
//!
//! - P01 = P(measured 1 | true 0)
//! - P10 = P(measured 0 | true 1)
//! - P00 = P(measured 0 | true 0)
//! - P11 = P(measured 1 | true 1)
//!
//! Each row therefore sums to one.
//!
//! # Important statistical distinction
//!
//! A readout calibration error and an observed benchmark error are not the
//! same quantity.
//!
//! Calibration estimates the conditional measurement channel:
//!
//!     P(measured | prepared)
//!
//! Benchmark observations describe the distribution produced by an actual
//! circuit and its measurement process.
//!
//! This module keeps those concepts separate.
//!
//! # Correction warning
//!
//! Readout correction is deliberately limited to independently calibrated
//! binary qubits. This module does NOT claim that independent per-qubit
//! correction removes correlated readout error or full-system assignment
//! error.
//!
//! A future correlated assignment-matrix implementation can be layered on
//! top without changing the public structures defined here.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.
//!
//! # Dependencies
//!
//! Only the Rust standard library and the already-existing Zamani hardware
//! calibration abstraction are required.

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::hardware::calibration::ReadoutCalibration;

// =============================================================================
// Constants
// =============================================================================

/// Default confidence level for readout estimates.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Minimum acceptable absolute determinant for a readout assignment matrix.
///
/// A determinant close to zero means the measurement channel is nearly
/// singular and readout inversion becomes numerically unstable.
pub const DEFAULT_MIN_CORRECTION_DETERMINANT: f64 = 1.0e-12;

/// Maximum absolute correction multiplier accepted by the conservative
/// single-qubit correction implementation.
///
/// This prevents extremely ill-conditioned matrices from producing
/// meaningless estimates.
pub const DEFAULT_MAX_CORRECTION_GAIN: f64 = 100.0;

/// Small numerical tolerance used when validating probabilities.
pub const PROBABILITY_EPSILON: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by readout metric construction or analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadoutError {
    /// No calibration or observation samples were supplied.
    EmptySamples,

    /// A probability is not finite or outside [0, 1].
    InvalidProbability {
        field: String,
        value: f64,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// A shot count is invalid.
    InvalidShotCount,

    /// An observed count is greater than the number of shots.
    CountExceedsShots {
        count: u64,
        shots: u64,
    },

    /// The sum of supplied outcome counts does not equal the declared shot
    /// count.
    ShotCountMismatch {
        declared: u64,
        observed: u64,
    },

    /// A bitstring is empty.
    EmptyBitstring,

    /// A bitstring contains a non-binary character.
    InvalidBitstring {
        bitstring: String,
        character: char,
    },

    /// Bitstrings in one result set do not have the same width.
    InconsistentBitstringWidth {
        expected: usize,
        actual: usize,
        bitstring: String,
    },

    /// A requested qubit is outside the result width.
    InvalidQubit {
        qubit: usize,
        qubit_count: usize,
    },

    /// Calibration data contains no useful measurements.
    UnmeasuredCalibration,

    /// The assignment matrix is invalid.
    InvalidAssignmentMatrix {
        message: String,
    },

    /// The assignment matrix cannot safely be inverted.
    SingularAssignmentMatrix {
        determinant: f64,
    },

    /// Readout correction would be numerically unstable.
    UnstableCorrection {
        determinant: f64,
        gain: f64,
    },

    /// A corrected probability could not be represented as a probability.
    InvalidCorrectedProbability {
        value: f64,
    },

    /// Two collections have incompatible dimensions.
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// A metric cannot be calculated because required information is absent.
    InsufficientData {
        message: String,
    },
}

impl fmt::Display for ReadoutError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySamples => {
                write!(formatter, "readout analysis requires at least one sample")
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "invalid probability for '{}': {}",
                    field, value
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite and strictly between 0 and 1: {}",
                    value
                )
            }

            Self::InvalidShotCount => {
                write!(formatter, "shot count must be greater than zero")
            }

            Self::CountExceedsShots { count, shots } => {
                write!(
                    formatter,
                    "observed count {} exceeds declared shot count {}",
                    count, shots
                )
            }

            Self::ShotCountMismatch {
                declared,
                observed,
            } => {
                write!(
                    formatter,
                    "declared shot count {} does not match observed count total {}",
                    declared, observed
                )
            }

            Self::EmptyBitstring => {
                write!(formatter, "measurement bitstrings cannot be empty")
            }

            Self::InvalidBitstring {
                bitstring,
                character,
            } => {
                write!(
                    formatter,
                    "measurement bitstring '{}' contains invalid character '{}'; \
                     only '0' and '1' are allowed",
                    bitstring, character
                )
            }

            Self::InconsistentBitstringWidth {
                expected,
                actual,
                bitstring,
            } => {
                write!(
                    formatter,
                    "measurement bitstring '{}' has width {}, expected {}",
                    bitstring, actual, expected
                )
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "qubit {} is outside measurement range 0..{}",
                    qubit,
                    qubit_count.saturating_sub(1)
                )
            }

            Self::UnmeasuredCalibration => {
                write!(
                    formatter,
                    "readout calibration contains no empirical calibration shots"
                )
            }

            Self::InvalidAssignmentMatrix { message } => {
                write!(formatter, "invalid readout assignment matrix: {}", message)
            }

            Self::SingularAssignmentMatrix { determinant } => {
                write!(
                    formatter,
                    "readout assignment matrix is singular or numerically \
                     non-invertible; determinant={}",
                    determinant
                )
            }

            Self::UnstableCorrection {
                determinant,
                gain,
            } => {
                write!(
                    formatter,
                    "readout correction is numerically unstable; \
                     determinant={}, correction gain={}",
                    determinant, gain
                )
            }

            Self::InvalidCorrectedProbability { value } => {
                write!(
                    formatter,
                    "readout correction produced invalid probability {}",
                    value
                )
            }

            Self::DimensionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "dimension mismatch: expected {}, received {}",
                    expected, actual
                )
            }

            Self::InsufficientData { message } => {
                write!(formatter, "insufficient readout data: {}", message)
            }
        }
    }
}

impl std::error::Error for ReadoutError {}

// =============================================================================
// Statistical primitives
// =============================================================================

/// A bounded confidence interval for a probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Point estimate.
    pub estimate: f64,

    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level, e.g. 0.95.
    pub confidence_level: f64,

    /// Number of Bernoulli trials represented by the estimate.
    pub samples: u64,
}

impl ConfidenceInterval {
    /// Constructs a validated confidence interval.
    pub fn new(
        estimate: f64,
        lower: f64,
        upper: f64,
        confidence_level: f64,
        samples: u64,
    ) -> Result<Self, ReadoutError> {
        validate_probability("estimate", estimate)?;
        validate_probability("lower", lower)?;
        validate_probability("upper", upper)?;
        validate_confidence_level(confidence_level)?;

        if lower > upper {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message: "confidence lower bound exceeds upper bound".to_string(),
            });
        }

        if samples == 0 {
            return Err(ReadoutError::InvalidShotCount);
        }

        Ok(Self {
            estimate,
            lower,
            upper,
            confidence_level,
            samples,
        })
    }

    /// Width of the confidence interval.
    pub fn width(self) -> f64 {
        self.upper - self.lower
    }

    /// Returns whether the interval contains a threshold.
    pub fn contains(self, threshold: f64) -> Result<bool, ReadoutError> {
        validate_probability("threshold", threshold)?;

        Ok(threshold >= self.lower && threshold <= self.upper)
    }
}

// =============================================================================
// Binary assignment matrix
// =============================================================================

/// Binary measurement assignment matrix.
///
/// Matrix convention:
///
/// ```text
///                measured 0    measured 1
/// true 0             p00           p01
/// true 1             p10           p11
/// ```
///
/// Each row represents a conditional distribution and therefore must sum to
/// one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssignmentMatrix {
    /// P(measured 0 | true 0).
    pub p00: f64,

    /// P(measured 1 | true 0).
    pub p01: f64,

    /// P(measured 0 | true 1).
    pub p10: f64,

    /// P(measured 1 | true 1).
    pub p11: f64,
}

impl AssignmentMatrix {
    /// Constructs and validates an assignment matrix.
    pub fn new(
        p00: f64,
        p01: f64,
        p10: f64,
        p11: f64,
    ) -> Result<Self, ReadoutError> {
        let matrix = Self {
            p00,
            p01,
            p10,
            p11,
        };

        matrix.validate()?;

        Ok(matrix)
    }

    /// Constructs an assignment matrix from the two standard readout error
    /// probabilities.
    pub fn from_error_rates(
        p01: f64,
        p10: f64,
    ) -> Result<Self, ReadoutError> {
        validate_probability("p01", p01)?;
        validate_probability("p10", p10)?;

        Self::new(
            1.0 - p01,
            p01,
            p10,
            1.0 - p10,
        )
    }

    /// Creates an ideal measurement channel.
    pub const fn ideal() -> Self {
        Self {
            p00: 1.0,
            p01: 0.0,
            p10: 0.0,
            p11: 1.0,
        }
    }

    /// Validates the assignment matrix.
    pub fn validate(&self) -> Result<(), ReadoutError> {
        validate_probability("p00", self.p00)?;
        validate_probability("p01", self.p01)?;
        validate_probability("p10", self.p10)?;
        validate_probability("p11", self.p11)?;

        if !approximately_one(self.p00 + self.p01)
            || !approximately_one(self.p10 + self.p11)
        {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message: "each conditional row must sum to one".to_string(),
            });
        }

        Ok(())
    }

    /// Matrix determinant.
    ///
    /// For a valid binary assignment matrix this is:
    ///
    ///     p00 * p11 - p01 * p10
    pub fn determinant(self) -> f64 {
        self.p00 * self.p11 - self.p01 * self.p10
    }

    /// Average probability of assigning the correct state.
    pub fn assignment_fidelity(self) -> f64 {
        (self.p00 + self.p11) / 2.0
    }

    /// Average readout error.
    pub fn average_error(self) -> f64 {
        1.0 - self.assignment_fidelity()
    }

    /// Probability of a 0→1 assignment error.
    pub fn false_positive_rate(self) -> f64 {
        self.p01
    }

    /// Probability of a 1→0 assignment error.
    pub fn false_negative_rate(self) -> f64 {
        self.p10
    }

    /// Maximum directional error.
    pub fn worst_case_error(self) -> f64 {
        self.p01.max(self.p10)
    }

    /// Minimum directional fidelity.
    pub fn worst_case_fidelity(self) -> f64 {
        self.p00.min(self.p11)
    }

    /// Returns whether inversion is safe under the supplied determinant
    /// threshold.
    pub fn is_invertible(self, minimum_determinant: f64) -> bool {
        self.determinant().abs() >= minimum_determinant
    }

    /// Applies the assignment channel to a true probability distribution.
    ///
    /// Input:
    ///
    /// - `p_true_zero`
    /// - `p_true_one`
    ///
    /// Output:
    ///
    /// - measured probability of zero
    /// - measured probability of one
    pub fn apply(
        self,
        p_true_zero: f64,
        p_true_one: f64,
    ) -> Result<(f64, f64), ReadoutError> {
        validate_probability("p_true_zero", p_true_zero)?;
        validate_probability("p_true_one", p_true_one)?;

        if !approximately_one(p_true_zero + p_true_one) {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "true-state probabilities must sum to one"
                        .to_string(),
            });
        }

        let measured_zero =
            self.p00 * p_true_zero + self.p10 * p_true_one;

        let measured_one =
            self.p01 * p_true_zero + self.p11 * p_true_one;

        validate_probability("measured_zero", measured_zero)?;
        validate_probability("measured_one", measured_one)?;

        Ok((measured_zero, measured_one))
    }

    /// Inverts a measured binary distribution to estimate the underlying
    /// true distribution.
    ///
    /// The correction is intentionally bounded to avoid amplifying noise when
    /// the assignment matrix is nearly singular.
    pub fn correct(
        self,
        measured_zero: f64,
        measured_one: f64,
        minimum_determinant: f64,
        maximum_gain: f64,
    ) -> Result<(f64, f64), ReadoutError> {
        validate_probability("measured_zero", measured_zero)?;
        validate_probability("measured_one", measured_one)?;

        if !approximately_one(measured_zero + measured_one) {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "measured probabilities must sum to one"
                        .to_string(),
            });
        }

        if !minimum_determinant.is_finite()
            || minimum_determinant <= 0.0
        {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "minimum determinant must be finite and positive"
                        .to_string(),
            });
        }

        if !maximum_gain.is_finite() || maximum_gain < 1.0 {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "maximum correction gain must be finite and at least one"
                        .to_string(),
            });
        }

        let determinant = self.determinant();

        if determinant.abs() < minimum_determinant {
            return Err(ReadoutError::SingularAssignmentMatrix {
                determinant,
            });
        }

        let gain = 1.0 / determinant.abs();

        if gain > maximum_gain {
            return Err(ReadoutError::UnstableCorrection {
                determinant,
                gain,
            });
        }

        let true_zero =
            (self.p11 * measured_zero - self.p10 * measured_one)
                / determinant;

        let true_one =
            (-self.p01 * measured_zero + self.p00 * measured_one)
                / determinant;

        // A physical probability cannot be negative or greater than one.
        //
        // Small excursions caused by floating-point arithmetic are clipped.
        // Large excursions are rejected because they indicate that the
        // measured distribution is incompatible with this calibration model.
        let corrected_zero =
            normalize_corrected_probability(true_zero)?;

        let corrected_one =
            normalize_corrected_probability(true_one)?;

        let total = corrected_zero + corrected_one;

        if !total.is_finite() || total <= 0.0 {
            return Err(ReadoutError::InvalidCorrectedProbability {
                value: total,
            });
        }

        Ok((
            corrected_zero / total,
            corrected_one / total,
        ))
    }
}

// =============================================================================
// Calibration observations
// =============================================================================

/// Raw calibration observations for one prepared/true state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalibrationCounts {
    /// Number of times the true/prepared state was zero.
    pub prepared_zero: u64,

    /// Number of those zero preparations measured as zero.
    pub measured_zero_from_zero: u64,

    /// Number of those zero preparations measured as one.
    pub measured_one_from_zero: u64,

    /// Number of times the true/prepared state was one.
    pub prepared_one: u64,

    /// Number of those one preparations measured as zero.
    pub measured_zero_from_one: u64,

    /// Number of those one preparations measured as one.
    pub measured_one_from_one: u64,
}

impl CalibrationCounts {
    /// Creates an empty calibration observation.
    pub const fn empty() -> Self {
        Self {
            prepared_zero: 0,
            measured_zero_from_zero: 0,
            measured_one_from_zero: 0,
            prepared_one: 0,
            measured_zero_from_one: 0,
            measured_one_from_one: 0,
        }
    }

    /// Total calibration shots.
    pub fn total_shots(self) -> u64 {
        self.prepared_zero
            .saturating_add(self.prepared_one)
    }

    /// Validates internal count consistency.
    pub fn validate(self) -> Result<(), ReadoutError> {
        if self.prepared_zero == 0 && self.prepared_one == 0 {
            return Err(ReadoutError::EmptySamples);
        }

        let zero_observed = self
            .measured_zero_from_zero
            .saturating_add(self.measured_one_from_zero);

        if zero_observed != self.prepared_zero {
            return Err(ReadoutError::ShotCountMismatch {
                declared: self.prepared_zero,
                observed: zero_observed,
            });
        }

        let one_observed = self
            .measured_zero_from_one
            .saturating_add(self.measured_one_from_one);

        if one_observed != self.prepared_one {
            return Err(ReadoutError::ShotCountMismatch {
                declared: self.prepared_one,
                observed: one_observed,
            });
        }

        Ok(())
    }

    /// Converts calibration counts into an assignment matrix.
    ///
    /// Both prepared states must have at least one calibration shot.
    pub fn assignment_matrix(
        self,
    ) -> Result<AssignmentMatrix, ReadoutError> {
        self.validate()?;

        if self.prepared_zero == 0 || self.prepared_one == 0 {
            return Err(ReadoutError::UnmeasuredCalibration);
        }

        let p00 =
            self.measured_zero_from_zero as f64
                / self.prepared_zero as f64;

        let p01 =
            self.measured_one_from_zero as f64
                / self.prepared_zero as f64;

        let p10 =
            self.measured_zero_from_one as f64
                / self.prepared_one as f64;

        let p11 =
            self.measured_one_from_one as f64
                / self.prepared_one as f64;

        AssignmentMatrix::new(p00, p01, p10, p11)
    }
}

// =============================================================================
// Per-qubit readout metric
// =============================================================================

/// Complete readout characterization for one qubit.
#[derive(Debug, Clone, PartialEq)]
pub struct QubitReadoutMetrics {
    /// Physical/logical qubit identifier.
    pub qubit: usize,

    /// Assignment matrix.
    pub assignment_matrix: AssignmentMatrix,

    /// Assignment fidelity.
    pub assignment_fidelity: f64,

    /// Average readout error.
    pub average_error: f64,

    /// False-positive probability P(1 | 0).
    pub false_positive_rate: f64,

    /// False-negative probability P(0 | 1).
    pub false_negative_rate: f64,

    /// Worst-case directional error.
    pub worst_case_error: f64,

    /// Confidence interval for P(1 | 0).
    pub false_positive_confidence: ConfidenceInterval,

    /// Confidence interval for P(0 | 1).
    pub false_negative_confidence: ConfidenceInterval,

    /// Calibration shots used for prepared zero.
    pub prepared_zero_shots: u64,

    /// Calibration shots used for prepared one.
    pub prepared_one_shots: u64,
}

impl QubitReadoutMetrics {
    /// Calculates metrics from raw calibration counts.
    pub fn from_counts(
        qubit: usize,
        counts: CalibrationCounts,
        confidence_level: f64,
    ) -> Result<Self, ReadoutError> {
        counts.validate()?;
        validate_confidence_level(confidence_level)?;

        let matrix = counts.assignment_matrix()?;

        let false_positive =
            matrix.false_positive_rate();

        let false_negative =
            matrix.false_negative_rate();

        let false_positive_success =
            counts.measured_one_from_zero;

        let false_negative_success =
            counts.measured_zero_from_one;

        let false_positive_confidence =
            wilson_interval(
                false_positive_success,
                counts.prepared_zero,
                confidence_level,
            )?;

        let false_negative_confidence =
            wilson_interval(
                false_negative_success,
                counts.prepared_one,
                confidence_level,
            )?;

        Ok(Self {
            qubit,
            assignment_matrix: matrix,
            assignment_fidelity: matrix.assignment_fidelity(),
            average_error: matrix.average_error(),
            false_positive_rate: false_positive,
            false_negative_rate: false_negative,
            worst_case_error: matrix.worst_case_error(),
            false_positive_confidence,
            false_negative_confidence,
            prepared_zero_shots: counts.prepared_zero,
            prepared_one_shots: counts.prepared_one,
        })
    }

    /// Creates metrics from an existing Zamani hardware calibration record.
    ///
    /// The hardware calibration abstraction stores only p01/p10 and its total
    /// calibration-shot count. Because it does not retain the directional
    /// prepared-state counts, this method produces the assignment metrics and
    /// a conservative confidence interval using the recorded total shots as
    /// the empirical sample count.
    ///
    /// Exact directional confidence intervals should be calculated with
    /// `from_counts` when the raw calibration counts are available.
    pub fn from_hardware_calibration(
        qubit: usize,
        calibration: &ReadoutCalibration,
        confidence_level: f64,
    ) -> Result<Self, ReadoutError> {
        validate_confidence_level(confidence_level)?;

        if calibration.shots == 0 {
            return Err(ReadoutError::UnmeasuredCalibration);
        }

        let matrix = AssignmentMatrix::from_error_rates(
            calibration.p01,
            calibration.p10,
        )?;

        let total_shots = calibration.shots;

        let fp_success =
            rounded_probability_count(
                calibration.p01,
                total_shots,
            );

        let fn_success =
            rounded_probability_count(
                calibration.p10,
                total_shots,
            );

        let false_positive_confidence =
            wilson_interval(
                fp_success,
                total_shots,
                confidence_level,
            )?;

        let false_negative_confidence =
            wilson_interval(
                fn_success,
                total_shots,
                confidence_level,
            )?;

        Ok(Self {
            qubit,
            assignment_matrix: matrix,
            assignment_fidelity: matrix.assignment_fidelity(),
            average_error: matrix.average_error(),
            false_positive_rate: matrix.false_positive_rate(),
            false_negative_rate: matrix.false_negative_rate(),
            worst_case_error: matrix.worst_case_error(),
            false_positive_confidence,
            false_negative_confidence,
            prepared_zero_shots: total_shots,
            prepared_one_shots: total_shots,
        })
    }
}

// =============================================================================
// Readout counts
// =============================================================================

/// Empirical measurement-count distribution.
///
/// Bitstrings are stored in canonical binary form without whitespace,
/// separators, or hexadecimal prefixes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadoutCounts {
    /// Number of measured qubits.
    pub qubit_count: usize,

    /// Total number of shots.
    pub shots: u64,

    /// Counts indexed by canonical binary bitstring.
    pub counts: BTreeMap<String, u64>,
}

impl ReadoutCounts {
    /// Creates an empty count distribution for a fixed number of qubits.
    pub fn new(qubit_count: usize) -> Result<Self, ReadoutError> {
        if qubit_count == 0 {
            return Err(ReadoutError::DimensionMismatch {
                expected: 1,
                actual: 0,
            });
        }

        Ok(Self {
            qubit_count,
            shots: 0,
            counts: BTreeMap::new(),
        })
    }

    /// Creates a count distribution from complete raw counts.
    ///
    /// The declared shot count is derived from the counts and therefore
    /// cannot disagree with the observations.
    pub fn from_counts(
        qubit_count: usize,
        counts: BTreeMap<String, u64>,
    ) -> Result<Self, ReadoutError> {
        let mut result = Self::new(qubit_count)?;

        for (bitstring, count) in counts {
            result.insert(bitstring, count)?;
        }

        result.validate()?;

        Ok(result)
    }

    /// Inserts or replaces an outcome count.
    pub fn insert(
        &mut self,
        bitstring: impl Into<String>,
        count: u64,
    ) -> Result<(), ReadoutError> {
        let bitstring = canonical_bitstring(&bitstring.into())?;

        validate_bitstring_width(
            &bitstring,
            self.qubit_count,
        )?;

        self.counts.insert(bitstring, count);

        self.recalculate_shots();

        Ok(())
    }

    /// Adds shots to an outcome rather than replacing the existing count.
    pub fn add(
        &mut self,
        bitstring: impl Into<String>,
        count: u64,
    ) -> Result<(), ReadoutError> {
        let bitstring = canonical_bitstring(&bitstring.into())?;

        validate_bitstring_width(
            &bitstring,
            self.qubit_count,
        )?;

        let entry = self.counts.entry(bitstring).or_insert(0);

        *entry = entry
            .checked_add(count)
            .ok_or(ReadoutError::ShotCountMismatch {
                declared: u64::MAX,
                observed: u64::MAX,
            })?;

        self.recalculate_shots();

        Ok(())
    }

    /// Validates all counts.
    pub fn validate(&self) -> Result<(), ReadoutError> {
        if self.qubit_count == 0 {
            return Err(ReadoutError::DimensionMismatch {
                expected: 1,
                actual: 0,
            });
        }

        let mut observed = 0u64;

        for (bitstring, count) in &self.counts {
            validate_bitstring_width(
                bitstring,
                self.qubit_count,
            )?;

            observed = observed
                .checked_add(*count)
                .ok_or(ReadoutError::ShotCountMismatch {
                    declared: self.shots,
                    observed: u64::MAX,
                })?;
        }

        if observed != self.shots {
            return Err(ReadoutError::ShotCountMismatch {
                declared: self.shots,
                observed,
            });
        }

        if self.shots == 0 {
            return Err(ReadoutError::EmptySamples);
        }

        Ok(())
    }

    /// Returns the probability of a particular bitstring.
    pub fn probability(
        &self,
        bitstring: &str,
    ) -> Result<f64, ReadoutError> {
        self.validate()?;

        let bitstring = canonical_bitstring(bitstring)?;

        validate_bitstring_width(
            &bitstring,
            self.qubit_count,
        )?;

        let count =
            self.counts.get(&bitstring).copied().unwrap_or(0);

        Ok(count as f64 / self.shots as f64)
    }

    /// Returns the marginal probability of measuring `1` on one qubit.
    ///
    /// Qubit index 0 refers to the first character in the canonical
    /// bitstring. This convention is explicit to avoid silently importing
    /// backend-specific little-endian assumptions.
    pub fn marginal_one_probability(
        &self,
        qubit: usize,
    ) -> Result<f64, ReadoutError> {
        self.validate()?;

        if qubit >= self.qubit_count {
            return Err(ReadoutError::InvalidQubit {
                qubit,
                qubit_count: self.qubit_count,
            });
        }

        let mut ones = 0u64;

        for (bitstring, count) in &self.counts {
            if bitstring.as_bytes()[qubit] == b'1' {
                ones = ones
                    .checked_add(*count)
                    .ok_or(ReadoutError::ShotCountMismatch {
                        declared: self.shots,
                        observed: u64::MAX,
                    })?;
            }
        }

        Ok(ones as f64 / self.shots as f64)
    }

    /// Returns all single-qubit marginal probabilities.
    pub fn marginal_one_probabilities(
        &self,
    ) -> Result<Vec<f64>, ReadoutError> {
        self.validate()?;

        let mut result =
            Vec::with_capacity(self.qubit_count);

        for qubit in 0..self.qubit_count {
            result.push(
                self.marginal_one_probability(qubit)?,
            );
        }

        Ok(result)
    }

    /// Returns the number of shots associated with one qubit being measured
    /// in the requested state.
    pub fn marginal_count(
        &self,
        qubit: usize,
        state: u8,
    ) -> Result<u64, ReadoutError> {
        self.validate()?;

        if qubit >= self.qubit_count {
            return Err(ReadoutError::InvalidQubit {
                qubit,
                qubit_count: self.qubit_count,
            });
        }

        if state > 1 {
            return Err(ReadoutError::InvalidBitstring {
                bitstring: state.to_string(),
                character: '?',
            });
        }

        let expected = if state == 0 { b'0' } else { b'1' };

        let mut total = 0u64;

        for (bitstring, count) in &self.counts {
            if bitstring.as_bytes()[qubit] == expected {
                total = total
                    .checked_add(*count)
                    .ok_or(ReadoutError::ShotCountMismatch {
                        declared: self.shots,
                        observed: u64::MAX,
                    })?;
            }
        }

        Ok(total)
    }

    fn recalculate_shots(&mut self) {
        self.shots = self
            .counts
            .values()
            .fold(0u64, |total, count| {
                total.saturating_add(*count)
            });
    }
}

// =============================================================================
// Aggregate readout metrics
// =============================================================================

/// Aggregate readout characterization across one or more qubits.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadoutMetrics {
    /// Number of characterized qubits.
    pub qubit_count: usize,

    /// Total calibration/measurement shots represented by the input.
    pub shots: u64,

    /// Mean assignment fidelity across qubits.
    pub mean_assignment_fidelity: f64,

    /// Mean readout error across qubits.
    pub mean_readout_error: f64,

    /// Worst observed assignment fidelity.
    pub minimum_assignment_fidelity: f64,

    /// Worst observed readout error.
    pub maximum_readout_error: f64,

    /// Mean false-positive rate.
    pub mean_false_positive_rate: f64,

    /// Mean false-negative rate.
    pub mean_false_negative_rate: f64,

    /// Per-qubit metrics.
    pub per_qubit: Vec<QubitReadoutMetrics>,
}

impl ReadoutMetrics {
    /// Builds aggregate metrics from per-qubit results.
    pub fn from_per_qubit(
        metrics: Vec<QubitReadoutMetrics>,
    ) -> Result<Self, ReadoutError> {
        if metrics.is_empty() {
            return Err(ReadoutError::EmptySamples);
        }

        let mut total_shots = 0u64;
        let mut fidelity_sum = 0.0;
        let mut error_sum = 0.0;
        let mut fp_sum = 0.0;
        let mut fn_sum = 0.0;

        let mut minimum_fidelity = f64::INFINITY;
        let mut maximum_error = f64::NEG_INFINITY;

        for metric in &metrics {
            total_shots = total_shots
                .checked_add(
                    metric
                        .prepared_zero_shots
                        .saturating_add(
                            metric.prepared_one_shots,
                        ),
                )
                .ok_or(ReadoutError::ShotCountMismatch {
                    declared: u64::MAX,
                    observed: u64::MAX,
                })?;

            fidelity_sum += metric.assignment_fidelity;
            error_sum += metric.average_error;
            fp_sum += metric.false_positive_rate;
            fn_sum += metric.false_negative_rate;

            minimum_fidelity =
                minimum_fidelity.min(
                    metric.assignment_fidelity,
                );

            maximum_error =
                maximum_error.max(metric.average_error);
        }

        let count = metrics.len() as f64;

        Ok(Self {
            qubit_count: metrics.len(),
            shots: total_shots,
            mean_assignment_fidelity: fidelity_sum / count,
            mean_readout_error: error_sum / count,
            minimum_assignment_fidelity: minimum_fidelity,
            maximum_readout_error: maximum_error,
            mean_false_positive_rate: fp_sum / count,
            mean_false_negative_rate: fn_sum / count,
            per_qubit: metrics,
        })
    }

    /// Returns the qubit with the worst assignment fidelity.
    pub fn worst_qubit(&self) -> Option<&QubitReadoutMetrics> {
        self.per_qubit.iter().min_by(|left, right| {
            left.assignment_fidelity
                .partial_cmp(&right.assignment_fidelity)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Returns the qubit with the highest readout error.
    pub fn highest_error_qubit(
        &self,
    ) -> Option<&QubitReadoutMetrics> {
        self.per_qubit.iter().max_by(|left, right| {
            left.average_error
                .partial_cmp(&right.average_error)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

// =============================================================================
// Readout analyzer
// =============================================================================

/// Production readout analyzer.
///
/// The analyzer is immutable and contains only numerical/statistical policy.
/// It performs no device I/O and has no global state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReadoutAnalyzer {
    /// Confidence level used for reported intervals.
    pub confidence_level: f64,

    /// Minimum determinant accepted for readout correction.
    pub minimum_correction_determinant: f64,

    /// Maximum correction gain accepted.
    pub maximum_correction_gain: f64,
}

impl Default for ReadoutAnalyzer {
    fn default() -> Self {
        Self {
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            minimum_correction_determinant:
                DEFAULT_MIN_CORRECTION_DETERMINANT,
            maximum_correction_gain:
                DEFAULT_MAX_CORRECTION_GAIN,
        }
    }
}

impl ReadoutAnalyzer {
    /// Creates a validated analyzer.
    pub fn new(
        confidence_level: f64,
    ) -> Result<Self, ReadoutError> {
        validate_confidence_level(confidence_level)?;

        Ok(Self {
            confidence_level,
            ..Self::default()
        })
    }

    /// Creates an analyzer with complete numerical policy.
    pub fn with_correction_policy(
        confidence_level: f64,
        minimum_correction_determinant: f64,
        maximum_correction_gain: f64,
    ) -> Result<Self, ReadoutError> {
        validate_confidence_level(confidence_level)?;

        if !minimum_correction_determinant.is_finite()
            || minimum_correction_determinant <= 0.0
        {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "minimum correction determinant must be finite and positive"
                        .to_string(),
            });
        }

        if !maximum_correction_gain.is_finite()
            || maximum_correction_gain < 1.0
        {
            return Err(ReadoutError::InvalidAssignmentMatrix {
                message:
                    "maximum correction gain must be finite and at least one"
                        .to_string(),
            });
        }

        Ok(Self {
            confidence_level,
            minimum_correction_determinant,
            maximum_correction_gain,
        })
    }

    /// Analyzes one qubit's raw calibration counts.
    pub fn analyze_qubit(
        &self,
        qubit: usize,
        counts: CalibrationCounts,
    ) -> Result<QubitReadoutMetrics, ReadoutError> {
        QubitReadoutMetrics::from_counts(
            qubit,
            counts,
            self.confidence_level,
        )
    }

    /// Analyzes multiple qubits.
    pub fn analyze_qubits(
        &self,
        calibrations: &BTreeMap<usize, CalibrationCounts>,
    ) -> Result<ReadoutMetrics, ReadoutError> {
        if calibrations.is_empty() {
            return Err(ReadoutError::EmptySamples);
        }

        let mut metrics =
            Vec::with_capacity(calibrations.len());

        for (qubit, counts) in calibrations {
            metrics.push(
                self.analyze_qubit(*qubit, *counts)?,
            );
        }

        ReadoutMetrics::from_per_qubit(metrics)
    }

    /// Converts an existing Zamani hardware calibration map into readout
    /// metrics.
    pub fn analyze_hardware_calibration(
        &self,
        calibrations: &BTreeMap<usize, ReadoutCalibration>,
    ) -> Result<ReadoutMetrics, ReadoutError> {
        if calibrations.is_empty() {
            return Err(ReadoutError::EmptySamples);
        }

        let mut metrics =
            Vec::with_capacity(calibrations.len());

        for (qubit, calibration) in calibrations {
            metrics.push(
                QubitReadoutMetrics::from_hardware_calibration(
                    *qubit,
                    calibration,
                    self.confidence_level,
                )?,
            );
        }

        ReadoutMetrics::from_per_qubit(metrics)
    }

    /// Calculates a confidence interval for a binary event.
    pub fn confidence_interval(
        &self,
        successes: u64,
        samples: u64,
    ) -> Result<ConfidenceInterval, ReadoutError> {
        wilson_interval(
            successes,
            samples,
            self.confidence_level,
        )
    }

    /// Applies single-qubit readout correction to a measured binary
    /// probability.
    pub fn correct_binary_probability(
        &self,
        matrix: AssignmentMatrix,
        measured_zero: f64,
        measured_one: f64,
    ) -> Result<(f64, f64), ReadoutError> {
        matrix.correct(
            measured_zero,
            measured_one,
            self.minimum_correction_determinant,
            self.maximum_correction_gain,
        )
    }

    /// Corrects the one-qubit marginals of a multi-qubit count distribution.
    ///
    /// This is deliberately NOT a joint-distribution correction.
    ///
    /// The returned vector contains corrected P(1) for each qubit.
    pub fn correct_marginals(
        &self,
        counts: &ReadoutCounts,
        matrices: &[AssignmentMatrix],
    ) -> Result<Vec<f64>, ReadoutError> {
        counts.validate()?;

        if matrices.len() != counts.qubit_count {
            return Err(ReadoutError::DimensionMismatch {
                expected: counts.qubit_count,
                actual: matrices.len(),
            });
        }

        let mut corrected =
            Vec::with_capacity(counts.qubit_count);

        for qubit in 0..counts.qubit_count {
            let measured_one =
                counts.marginal_one_probability(qubit)?;

            let measured_zero =
                1.0 - measured_one;

            let (_, true_one) =
                self.correct_binary_probability(
                    matrices[qubit],
                    measured_zero,
                    measured_one,
                )?;

            corrected.push(true_one);
        }

        Ok(corrected)
    }

    /// Calculates the uncorrected single-qubit marginals.
    pub fn measured_marginals(
        &self,
        counts: &ReadoutCounts,
    ) -> Result<Vec<f64>, ReadoutError> {
        counts.marginal_one_probabilities()
    }
}

// =============================================================================
// Calibration helpers
// =============================================================================

/// Creates raw calibration counts from directional observations.
///
/// This is useful when a backend reports four counters:
///
/// - 0 prepared → 0 measured
/// - 0 prepared → 1 measured
/// - 1 prepared → 0 measured
/// - 1 prepared → 1 measured
pub fn calibration_counts(
    measured_zero_from_zero: u64,
    measured_one_from_zero: u64,
    measured_zero_from_one: u64,
    measured_one_from_one: u64,
) -> Result<CalibrationCounts, ReadoutError> {
    let result = CalibrationCounts {
        prepared_zero:
            measured_zero_from_zero
                .checked_add(measured_one_from_zero)
                .ok_or(ReadoutError::ShotCountMismatch {
                    declared: u64::MAX,
                    observed: u64::MAX,
                })?,

        measured_zero_from_zero,
        measured_one_from_zero,

        prepared_one:
            measured_zero_from_one
                .checked_add(measured_one_from_one)
                .ok_or(ReadoutError::ShotCountMismatch {
                    declared: u64::MAX,
                    observed: u64::MAX,
                })?,

        measured_zero_from_one,
        measured_one_from_one,
    };

    result.validate()?;

    Ok(result)
}

// =============================================================================
// Public numerical helpers
// =============================================================================

/// Validates a probability.
pub fn validate_probability(
    field: &str,
    value: f64,
) -> Result<(), ReadoutError> {
    if !value.is_finite()
        || value < -PROBABILITY_EPSILON
        || value > 1.0 + PROBABILITY_EPSILON
    {
        return Err(ReadoutError::InvalidProbability {
            field: field.to_string(),
            value,
        });
    }

    Ok(())
}

/// Validates a confidence level.
pub fn validate_confidence_level(
    value: f64,
) -> Result<(), ReadoutError> {
    if !value.is_finite() || !(0.0 < value && value < 1.0) {
        return Err(ReadoutError::InvalidConfidenceLevel {
            value,
        });
    }

    Ok(())
}

/// Calculates a Wilson confidence interval for a binomial proportion.
///
/// Wilson is used rather than the naive normal approximation because it
/// remains well behaved near zero/one and for finite sample sizes.
pub fn wilson_interval(
    successes: u64,
    samples: u64,
    confidence_level: f64,
) -> Result<ConfidenceInterval, ReadoutError> {
    if samples == 0 {
        return Err(ReadoutError::InvalidShotCount);
    }

    if successes > samples {
        return Err(ReadoutError::CountExceedsShots {
            count: successes,
            shots: samples,
        });
    }

    validate_confidence_level(confidence_level)?;

    let estimate =
        successes as f64 / samples as f64;

    let z =
        inverse_normal_cdf(
            0.5 + confidence_level / 2.0,
        );

    let n = samples as f64;
    let z_squared = z * z;

    let denominator =
        1.0 + z_squared / n;

    let center =
        (estimate + z_squared / (2.0 * n))
            / denominator;

    let variance_term =
        estimate * (1.0 - estimate) / n
            + z_squared / (4.0 * n * n);

    let margin =
        z * variance_term.max(0.0).sqrt()
            / denominator;

    let lower =
        (center - margin).clamp(0.0, 1.0);

    let upper =
        (center + margin).clamp(0.0, 1.0);

    ConfidenceInterval::new(
        estimate,
        lower,
        upper,
        confidence_level,
        samples,
    )
}

// =============================================================================
// Normal quantile
// =============================================================================

/// Inverse standard-normal cumulative distribution function.
///
/// Uses the Peter John Acklam rational approximation.
///
/// The implementation is self-contained so the readout metric layer does not
/// require a heavyweight numerical dependency merely to calculate confidence
/// intervals.
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

        return (((((C[0] * q + C[1]) * q + C[2]) * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q
                + D[3])
                * q
                + 1.0);
    }

    if p > HIGH {
        let q =
            (-2.0 * (1.0 - p).ln()).sqrt();

        return -(((((C[0] * q + C[1]) * q + C[2]) * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q
                + D[3])
                * q
                + 1.0);
    }

    let q = p - 0.5;
    let r = q * q;

    (((((A[0] * r + A[1]) * r + A[2]) * r + A[3])
        * r
        + A[4])
        * r
        + A[5])
        * q)
        / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3])
            * r
            + B[4])
            * r)
            + 1.0)
}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn approximately_one(value: f64) -> bool {
    value.is_finite()
        && (value - 1.0).abs()
            <= PROBABILITY_EPSILON
}

fn normalize_corrected_probability(
    value: f64,
) -> Result<f64, ReadoutError> {
    if !value.is_finite() {
        return Err(ReadoutError::InvalidCorrectedProbability {
            value,
        });
    }

    if value < -PROBABILITY_EPSILON
        || value > 1.0 + PROBABILITY_EPSILON
    {
        return Err(ReadoutError::InvalidCorrectedProbability {
            value,
        });
    }

    Ok(value.clamp(0.0, 1.0))
}

fn rounded_probability_count(
    probability: f64,
    samples: u64,
) -> u64 {
    if samples == 0 {
        return 0;
    }

    let value =
        probability.clamp(0.0, 1.0)
            * samples as f64;

    value.round() as u64
}

fn canonical_bitstring(
    bitstring: &str,
) -> Result<String, ReadoutError> {
    let trimmed = bitstring.trim();

    if trimmed.is_empty() {
        return Err(ReadoutError::EmptyBitstring);
    }

    for character in trimmed.chars() {
        if character != '0' && character != '1' {
            return Err(ReadoutError::InvalidBitstring {
                bitstring: trimmed.to_string(),
                character,
            });
        }
    }

    Ok(trimmed.to_string())
}

fn validate_bitstring_width(
    bitstring: &str,
    expected: usize,
) -> Result<(), ReadoutError> {
    let actual = bitstring.len();

    if actual != expected {
        return Err(ReadoutError::InconsistentBitstringWidth {
            expected,
            actual,
            bitstring: bitstring.to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ideal_assignment_matrix_is_perfect() {
        let matrix = AssignmentMatrix::ideal();

        assert_eq!(matrix.assignment_fidelity(), 1.0);
        assert_eq!(matrix.average_error(), 0.0);
        assert_eq!(matrix.false_positive_rate(), 0.0);
        assert_eq!(matrix.false_negative_rate(), 0.0);
        assert_eq!(matrix.determinant(), 1.0);
    }

    #[test]
    fn assignment_matrix_from_error_rates_is_correct() {
        let matrix =
            AssignmentMatrix::from_error_rates(0.1, 0.2)
                .unwrap();

        assert!((matrix.p00 - 0.9).abs() < 1.0e-12);
        assert!((matrix.p01 - 0.1).abs() < 1.0e-12);
        assert!((matrix.p10 - 0.2).abs() < 1.0e-12);
        assert!((matrix.p11 - 0.8).abs() < 1.0e-12);
    }

    #[test]
    fn assignment_matrix_rejects_invalid_rows() {
        let result =
            AssignmentMatrix::new(
                0.5,
                0.5,
                0.5,
                0.6,
            );

        assert!(result.is_err());
    }

    #[test]
    fn assignment_matrix_applies_channel() {
        let matrix =
            AssignmentMatrix::from_error_rates(0.1, 0.2)
                .unwrap();

        let (zero, one) =
            matrix.apply(0.25, 0.75).unwrap();

        assert!((zero + one - 1.0).abs() < 1.0e-12);
        assert!((zero - 0.175).abs() < 1.0e-12);
        assert!((one - 0.825).abs() < 1.0e-12);
    }

    #[test]
    fn ideal_assignment_matrix_does_not_change_distribution() {
        let matrix = AssignmentMatrix::ideal();

        let corrected =
            matrix.correct(
                0.3,
                0.7,
                DEFAULT_MIN_CORRECTION_DETERMINANT,
                DEFAULT_MAX_CORRECTION_GAIN,
            )
            .unwrap();

        assert!((corrected.0 - 0.3).abs() < 1.0e-12);
        assert!((corrected.1 - 0.7).abs() < 1.0e-12);
    }

    #[test]
    fn correction_recovers_known_distribution() {
        let matrix =
            AssignmentMatrix::from_error_rates(0.1, 0.2)
                .unwrap();

        let measured =
            matrix.apply(0.25, 0.75).unwrap();

        let corrected =
            matrix.correct(
                measured.0,
                measured.1,
                DEFAULT_MIN_CORRECTION_DETERMINANT,
                DEFAULT_MAX_CORRECTION_GAIN,
            )
            .unwrap();

        assert!(
            (corrected.0 - 0.25).abs() < 1.0e-10
        );

        assert!(
            (corrected.1 - 0.75).abs() < 1.0e-10
        );
    }

    #[test]
    fn singular_correction_is_rejected() {
        let matrix =
            AssignmentMatrix::new(
                0.5,
                0.5,
                0.5,
                0.5,
            )
            .unwrap();

        let result =
            matrix.correct(
                0.5,
                0.5,
                DEFAULT_MIN_CORRECTION_DETERMINANT,
                DEFAULT_MAX_CORRECTION_GAIN,
            );

        assert!(matches!(
            result,
            Err(ReadoutError::SingularAssignmentMatrix { .. })
        ));
    }

    #[test]
    fn calibration_counts_generate_assignment_matrix() {
        let counts =
            calibration_counts(
                900,
                100,
                200,
                800,
            )
            .unwrap();

        let matrix =
            counts.assignment_matrix().unwrap();

        assert!((matrix.p00 - 0.9).abs() < 1.0e-12);
        assert!((matrix.p01 - 0.1).abs() < 1.0e-12);
        assert!((matrix.p10 - 0.2).abs() < 1.0e-12);
        assert!((matrix.p11 - 0.8).abs() < 1.0e-12);
    }

    #[test]
    fn calibration_counts_reject_mismatched_rows() {
        let counts = CalibrationCounts {
            prepared_zero: 1000,
            measured_zero_from_zero: 900,
            measured_one_from_zero: 50,
            prepared_one: 1000,
            measured_zero_from_one: 200,
            measured_one_from_one: 800,
        };

        assert!(counts.validate().is_err());
    }

    #[test]
    fn wilson_interval_is_bounded() {
        let interval =
            wilson_interval(
                50,
                100,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .unwrap();

        assert!(interval.lower >= 0.0);
        assert!(interval.upper <= 1.0);
        assert!(interval.lower <= interval.estimate);
        assert!(interval.estimate <= interval.upper);
    }

    #[test]
    fn readout_counts_validate_and_calculate_marginal() {
        let mut counts =
            ReadoutCounts::new(2).unwrap();

        counts.add("00", 25).unwrap();
        counts.add("01", 25).unwrap();
        counts.add("10", 25).unwrap();
        counts.add("11", 25).unwrap();

        assert_eq!(counts.shots, 100);

        let q0 =
            counts.marginal_one_probability(0)
                .unwrap();

        let q1 =
            counts.marginal_one_probability(1)
                .unwrap();

        assert!((q0 - 0.5).abs() < 1.0e-12);
        assert!((q1 - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn readout_counts_reject_invalid_bitstrings() {
        let mut counts =
            ReadoutCounts::new(2).unwrap();

        assert!(
            counts.add("0x", 1).is_err()
        );
    }

    #[test]
    fn readout_counts_reject_wrong_width() {
        let mut counts =
            ReadoutCounts::new(2).unwrap();

        assert!(
            counts.add("0", 1).is_err()
        );

        assert!(
            counts.add("000", 1).is_err()
        );
    }

    #[test]
    fn qubit_metrics_are_correct() {
        let counts =
            calibration_counts(
                950,
                50,
                100,
                900,
            )
            .unwrap();

        let metrics =
            QubitReadoutMetrics::from_counts(
                3,
                counts,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .unwrap();

        assert_eq!(metrics.qubit, 3);
        assert!(
            (metrics.assignment_fidelity - 0.925).abs()
                < 1.0e-12
        );
        assert!(
            (metrics.average_error - 0.075).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn aggregate_metrics_find_worst_qubit() {
        let first =
            QubitReadoutMetrics::from_counts(
                0,
                calibration_counts(
                    990,
                    10,
                    20,
                    980,
                )
                .unwrap(),
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .unwrap();

        let second =
            QubitReadoutMetrics::from_counts(
                1,
                calibration_counts(
                    900,
                    100,
                    200,
                    800,
                )
                .unwrap(),
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .unwrap();

        let aggregate =
            ReadoutMetrics::from_per_qubit(
                vec![first, second],
            )
            .unwrap();

        assert_eq!(
            aggregate.worst_qubit().unwrap().qubit,
            1
        );

        assert_eq!(
            aggregate.highest_error_qubit().unwrap().qubit,
            1
        );
    }

    #[test]
    fn analyzer_corrects_marginals() {
        let mut counts =
            ReadoutCounts::new(2).unwrap();

        counts.add("00", 20).unwrap();
        counts.add("01", 30).unwrap();
        counts.add("10", 20).unwrap();
        counts.add("11", 30).unwrap();

        let matrix =
            AssignmentMatrix::ideal();

        let analyzer =
            ReadoutAnalyzer::default();

        let corrected =
            analyzer
                .correct_marginals(
                    &counts,
                    &[matrix, matrix],
                )
                .unwrap();

        assert!((corrected[0] - 0.5).abs() < 1.0e-12);
        assert!((corrected[1] - 0.6).abs() < 1.0e-12);
    }

    #[test]
    fn hardware_calibration_is_supported() {
        let calibration =
            ReadoutCalibration::new(
                0.05,
                0.10,
                1000,
            )
            .unwrap();

        let metrics =
            QubitReadoutMetrics::from_hardware_calibration(
                2,
                &calibration,
                DEFAULT_CONFIDENCE_LEVEL,
            )
            .unwrap();

        assert_eq!(metrics.qubit, 2);
        assert!(
            (metrics.false_positive_rate - 0.05).abs()
                < 1.0e-12
        );
        assert!(
            (metrics.false_negative_rate - 0.10).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn analyzer_rejects_empty_hardware_calibration() {
        let calibration =
            ReadoutCalibration::default();

        let result =
            QubitReadoutMetrics::from_hardware_calibration(
                0,
                &calibration,
                DEFAULT_CONFIDENCE_LEVEL,
            );

        assert!(matches!(
            result,
            Err(ReadoutError::UnmeasuredCalibration)
        ));
    }

    #[test]
    fn analyzer_rejects_dimension_mismatch() {
        let mut counts =
            ReadoutCounts::new(2).unwrap();

        counts.add("00", 100).unwrap();

        let analyzer =
            ReadoutAnalyzer::default();

        let result =
            analyzer.correct_marginals(
                &counts,
                &[AssignmentMatrix::ideal()],
            );

        assert!(matches!(
            result,
            Err(ReadoutError::DimensionMismatch {
                expected: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn confidence_interval_rejects_invalid_counts() {
        assert!(
            wilson_interval(
                101,
                100,
                DEFAULT_CONFIDENCE_LEVEL
            )
            .is_err()
        );
    }

    #[test]
    fn confidence_interval_rejects_zero_samples() {
        assert!(
            wilson_interval(
                0,
                0,
                DEFAULT_CONFIDENCE_LEVEL
            )
            .is_err()
        );
    }

    #[test]
    fn confidence_level_is_validated() {
        assert!(
            validate_confidence_level(0.95).is_ok()
        );

        assert!(
            validate_confidence_level(0.0).is_err()
        );

        assert!(
            validate_confidence_level(1.0).is_err()
        );

        assert!(
            validate_confidence_level(f64::NAN).is_err()
        );
    }
}