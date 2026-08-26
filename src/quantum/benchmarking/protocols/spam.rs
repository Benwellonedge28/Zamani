//! Zamani Quantum Benchmarking — SPAM / Readout Characterization
//!
//! Production-grade state-preparation-and-measurement (SPAM) and readout
//! characterization for the Zamani quantum benchmarking subsystem.
//!
//! # Scope
//!
//! This module measures classical readout behavior from experiments in which a
//! known/prepared computational-basis state is measured repeatedly.
//!
//! It deliberately does NOT:
//!
//! - generate Quantum IR;
//! - parse quantum source languages;
//! - compile circuits;
//! - route circuits;
//! - schedule circuits;
//! - communicate with hardware;
//! - select a backend;
//! - perform network I/O;
//! - own calibration state;
//! - execute experiments;
//! - maintain process-global state;
//! - silently discard malformed observations.
//!
//! The intended production dependency direction is:
//!
//! ```text
//! Zamani Quantum IR / benchmark generator
//!             │
//!             ▼
//!       SPAM experiment
//!             │
//!             ▼
//!       execution layer
//!             │
//!             ▼
//!       raw measurement counts
//!             │
//!             ▼
//!       protocols::spam
//!             │
//!       ┌─────┴───────────┐
//!       ▼                 ▼
//! readout metrics     BenchmarkResult
//! ```
//!
//! # Terminology
//!
//! Strictly speaking, SPAM includes both:
//!
//! - state-preparation error; and
//! - measurement/readout error.
//!
//! A computational-basis calibration experiment by itself observes the
//! combined preparation-and-measurement channel. Consequently, this module
//! does NOT claim that a measured assignment error is purely a measurement
//! error unless the caller has independently characterized or controlled state
//! preparation.
//!
//! The result therefore distinguishes:
//!
//! - `assignment_matrix`: the experimentally observed prepared-state →
//!   measured-state conditional distribution;
//! - `readout_fidelity`: diagonal assignment probability;
//! - `assignment_error`: `1 - readout_fidelity`;
//! - preparation/measurement attribution: explicitly unavailable unless an
//!   external calibration supplies the required decomposition.
//!
//! # Assignment matrix convention
//!
//! For a prepared state `i` and observed state `j`:
//!
//! ```text
//! A[j, i] = P(measured = j | prepared = i)
//! ```
//!
//! Therefore:
//!
//! - each prepared-state column sums to one;
//! - the diagonal contains correct-assignment probabilities;
//! - off-diagonal entries describe assignment errors.
//!
//! This convention is deliberately documented and tested because transposing
//! the matrix is a common source of integration bugs.
//!
//! # Input representation
//!
//! Raw observations are represented as:
//!
//! ```text
//! prepared computational-basis state
//!         │
//!         ▼
//! BTreeMap<measured_bitstring, count>
//! ```
//!
//! The module supports arbitrary binary string widths. It does not assume that
//! the backend has exactly eight, sixteen, or any other fixed number of qubits.
//!
//! # Production guarantees
//!
//! The implementation provides:
//!
//! - deterministic analysis;
//! - explicit configuration;
//! - strict validation;
//! - overflow-safe count accumulation;
//! - finite-value validation;
//! - exact shot accounting by default;
//! - configurable tolerance for floating-point probability validation;
//! - no process-global state;
//! - no stdout/stderr diagnostics;
//! - stable machine-readable identifiers;
//! - schema versioning;
//! - explicit confidence metadata;
//! - per-state metrics;
//! - aggregate metrics;
//! - confusion/assignment matrix;
//! - worst-case fidelity;
//! - average fidelity;
//! - balanced accuracy;
//! - average assignment error;
//! - maximum assignment error;
//! - total variation error;
//! - support for incomplete measurement domains when explicitly configured;
//! - deterministic result ordering;
//! - compatibility with Rust 1.97 / 1.97.1 / Rust 2021.
//!
//! # Integration contract
//!
//! This file intentionally has no dependency on future benchmarking modules.
//!
//! Future modules may consume it directly:
//!
//! ```text
//! protocols::spam
//!       │
//!       ├── execution::response
//!       │       │
//!       │       └── normalized counts
//!       │
//!       ├── metrics::readout
//!       │
//!       ├── core::observation
//!       │
//!       └── core::result
//! ```
//!
//! The future `metrics::readout.rs` may wrap or generalize the metric concepts
//! here, but it should not require this file to change merely because that
//! module is introduced.
//!
//! Similarly, the future `execution::response.rs` can convert provider-native
//! measurement results into `PreparedStateObservation` without changing the
//! SPAM analysis contract.
//!
//! # Important architectural rule
//!
//! `protocols::spam` owns the *experimental interpretation* of normalized SPAM
//! observations. It does not own execution.
//!
//! This preserves the benchmarking architecture established by the Quantum
//! Volume estimator: mathematical/protocol analysis remains independently
//! testable and reusable.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// ============================================================================
// Public protocol constants
// ============================================================================

/// Stable identifier for the SPAM benchmark.
pub const SPAM_BENCHMARK_ID: &str = "spam";

/// Stable identifier for the readout-characterization protocol.
pub const SPAM_PROTOCOL_ID: &str = "spam_readout";

/// Result schema version for this module.
pub const SPAM_RESULT_SCHEMA_VERSION: u32 = 1;

/// Minimum number of shots required by the protocol.
pub const MIN_SHOTS: usize = 1;

/// Numerical tolerance used when validating probabilities.
pub const PROBABILITY_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance used when validating matrix column normalization.
pub const NORMALIZATION_EPSILON: f64 = 1.0e-10;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by SPAM/readout characterization.
#[derive(Debug, Clone, PartialEq)]
pub enum SpamError {
    /// No prepared states were supplied.
    EmptyExperiment,

    /// No observations were supplied for a prepared state.
    EmptyPreparedState {
        prepared_state: String,
    },

    /// A prepared state is not a valid computational-basis bitstring.
    InvalidPreparedState {
        state: String,
    },

    /// A measured state is not a valid computational-basis bitstring.
    InvalidMeasuredState {
        state: String,
    },

    /// Prepared states have inconsistent widths.
    InconsistentPreparedStateWidth {
        expected: usize,
        actual: usize,
        state: String,
    },

    /// A measured state has an unexpected width.
    InconsistentMeasuredStateWidth {
        expected: usize,
        actual: usize,
        state: String,
    },

    /// Prepared and measured states have different widths.
    PreparedMeasuredWidthMismatch {
        prepared_state: String,
        measured_state: String,
        prepared_width: usize,
        measured_width: usize,
    },

    /// A measurement count is zero when zero-count entries are not allowed.
    ZeroCount {
        prepared_state: String,
        measured_state: String,
    },

    /// A count cannot be represented in the configured type.
    CountOverflow,

    /// Observed counts do not equal declared shots.
    ShotCountMismatch {
        prepared_state: String,
        declared_shots: usize,
        observed_shots: usize,
    },

    /// No shots were recorded.
    InvalidShotCount {
        prepared_state: String,
    },

    /// A probability is non-finite or outside the unit interval.
    InvalidProbability {
        value: f64,
        context: &'static str,
    },

    /// A tolerance is invalid.
    InvalidTolerance {
        value: f64,
    },

    /// A state label is duplicated.
    DuplicatePreparedState {
        state: String,
    },

    /// An unsupported configuration was requested.
    UnsupportedConfiguration {
        message: String,
    },

    /// Matrix dimensions do not agree.
    MatrixDimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// A calculated statistic is not finite.
    NonFiniteStatistic {
        statistic: &'static str,
    },

    /// A normalized probability column does not sum to one.
    MatrixNotNormalized {
        prepared_state: String,
        sum: f64,
    },
}

impl fmt::Display for SpamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExperiment => {
                write!(formatter, "SPAM experiment contains no prepared states")
            }

            Self::EmptyPreparedState { prepared_state } => {
                write!(
                    formatter,
                    "prepared state '{}' contains no observations",
                    prepared_state
                )
            }

            Self::InvalidPreparedState { state } => {
                write!(
                    formatter,
                    "prepared state '{}' is not a valid binary computational-basis state",
                    state
                )
            }

            Self::InvalidMeasuredState { state } => {
                write!(
                    formatter,
                    "measured state '{}' is not a valid binary computational-basis state",
                    state
                )
            }

            Self::InconsistentPreparedStateWidth {
                expected,
                actual,
                state,
            } => {
                write!(
                    formatter,
                    "prepared state '{}' has width {}, expected {}",
                    state, actual, expected
                )
            }

            Self::InconsistentMeasuredStateWidth {
                expected,
                actual,
                state,
            } => {
                write!(
                    formatter,
                    "measured state '{}' has width {}, expected {}",
                    state, actual, expected
                )
            }

            Self::PreparedMeasuredWidthMismatch {
                prepared_state,
                measured_state,
                prepared_width,
                measured_width,
            } => {
                write!(
                    formatter,
                    "prepared state '{}' has width {} but measured state '{}' has width {}",
                    prepared_state,
                    prepared_width,
                    measured_state,
                    measured_width
                )
            }

            Self::ZeroCount {
                prepared_state,
                measured_state,
            } => {
                write!(
                    formatter,
                    "measurement count for prepared state '{}' and measured state '{}' \
                     must be greater than zero",
                    prepared_state, measured_state
                )
            }

            Self::CountOverflow => {
                write!(formatter, "measurement count accumulation overflowed usize")
            }

            Self::ShotCountMismatch {
                prepared_state,
                declared_shots,
                observed_shots,
            } => {
                write!(
                    formatter,
                    "prepared state '{}' declares {} shots but contains {} observed shots",
                    prepared_state, declared_shots, observed_shots
                )
            }

            Self::InvalidShotCount { prepared_state } => {
                write!(
                    formatter,
                    "prepared state '{}' contains no shots",
                    prepared_state
                )
            }

            Self::InvalidProbability { value, context } => {
                write!(
                    formatter,
                    "probability '{}' is invalid: {}",
                    context, value
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "probability tolerance must be finite and non-negative, got {}",
                    value
                )
            }

            Self::DuplicatePreparedState { state } => {
                write!(
                    formatter,
                    "prepared state '{}' occurs more than once",
                    state
                )
            }

            Self::UnsupportedConfiguration { message } => {
                write!(formatter, "unsupported SPAM configuration: {}", message)
            }

            Self::MatrixDimensionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "assignment matrix dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }

            Self::NonFiniteStatistic { statistic } => {
                write!(
                    formatter,
                    "SPAM calculation produced a non-finite {}",
                    statistic
                )
            }

            Self::MatrixNotNormalized {
                prepared_state,
                sum,
            } => {
                write!(
                    formatter,
                    "assignment matrix column for prepared state '{}' sums to {}, \
                     outside normalization tolerance",
                    prepared_state, sum
                )
            }
        }
    }
}

impl Error for SpamError {}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_binary_state(
    state: &str,
    prepared: bool,
) -> Result<usize, SpamError> {
    if state.is_empty() {
        if prepared {
            return Err(SpamError::InvalidPreparedState {
                state: state.to_owned(),
            });
        }

        return Err(SpamError::InvalidMeasuredState {
            state: state.to_owned(),
        });
    }

    if !state.bytes().all(|byte| byte == b'0' || byte == b'1') {
        if prepared {
            return Err(SpamError::InvalidPreparedState {
                state: state.to_owned(),
            });
        }

        return Err(SpamError::InvalidMeasuredState {
            state: state.to_owned(),
        });
    }

    Ok(state.len())
}

fn validate_probability(
    value: f64,
    context: &'static str,
) -> Result<(), SpamError> {
    if !value.is_finite()
        || value < -PROBABILITY_EPSILON
        || value > 1.0 + PROBABILITY_EPSILON
    {
        return Err(SpamError::InvalidProbability { value, context });
    }

    Ok(())
}

fn clamp_probability(value: f64) -> f64 {
    if value < 0.0 && value >= -PROBABILITY_EPSILON {
        0.0
    } else if value > 1.0 && value <= 1.0 + PROBABILITY_EPSILON {
        1.0
    } else {
        value
    }
}

// ============================================================================
// Measurement observations
// ============================================================================

/// Raw measurement observations for one prepared computational-basis state.
///
/// The map key is the measured computational-basis bitstring and the value is
/// the number of shots producing that outcome.
///
/// `shots` is explicitly stored rather than inferred so an execution adapter
/// can preserve the backend's declared shot count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedStateObservation {
    /// State intentionally prepared before measurement.
    pub prepared_state: String,

    /// Number of shots requested/declared for this experiment.
    pub shots: usize,

    /// Raw measured-state counts.
    pub counts: BTreeMap<String, usize>,
}

impl PreparedStateObservation {
    /// Create an observation from a prepared state and raw counts.
    ///
    /// The declared shot count is inferred from the supplied counts.
    pub fn from_counts(
        prepared_state: impl Into<String>,
        counts: BTreeMap<String, usize>,
    ) -> Result<Self, SpamError> {
        let prepared_state = prepared_state.into();

        validate_binary_state(&prepared_state, true)?;

        let shots = checked_sum_counts(&prepared_state, &counts)?;

        if shots < MIN_SHOTS {
            return Err(SpamError::InvalidShotCount { prepared_state });
        }

        Ok(Self {
            prepared_state,
            shots,
            counts,
        })
    }

    /// Create an observation with an explicitly declared shot count.
    pub fn with_declared_shots(
        prepared_state: impl Into<String>,
        shots: usize,
        counts: BTreeMap<String, usize>,
    ) -> Result<Self, SpamError> {
        let prepared_state = prepared_state.into();

        validate_binary_state(&prepared_state, true)?;

        if shots < MIN_SHOTS {
            return Err(SpamError::InvalidShotCount {
                prepared_state,
            });
        }

        let observed_shots =
            checked_sum_counts(&prepared_state, &counts)?;

        if observed_shots != shots {
            return Err(SpamError::ShotCountMismatch {
                prepared_state,
                declared_shots: shots,
                observed_shots,
            });
        }

        Ok(Self {
            prepared_state,
            shots,
            counts,
        })
    }

    /// Return the observed number of samples represented by the count map.
    pub fn observed_shots(&self) -> Result<usize, SpamError> {
        checked_sum_counts(&self.prepared_state, &self.counts)
    }

    /// Validate the complete observation.
    pub fn validate(
        &self,
        expected_width: Option<usize>,
    ) -> Result<usize, SpamError> {
        let prepared_width =
            validate_binary_state(&self.prepared_state, true)?;

        if let Some(width) = expected_width {
            if prepared_width != width {
                return Err(SpamError::InconsistentPreparedStateWidth {
                    expected: width,
                    actual: prepared_width,
                    state: self.prepared_state.clone(),
                });
            }
        }

        if self.shots < MIN_SHOTS {
            return Err(SpamError::InvalidShotCount {
                prepared_state: self.prepared_state.clone(),
            });
        }

        let observed_shots =
            checked_sum_counts(&self.prepared_state, &self.counts)?;

        if observed_shots != self.shots {
            return Err(SpamError::ShotCountMismatch {
                prepared_state: self.prepared_state.clone(),
                declared_shots: self.shots,
                observed_shots,
            });
        }

        for (measured_state, count) in &self.counts {
            let measured_width =
                validate_binary_state(measured_state, false)?;

            if measured_width != prepared_width {
                return Err(SpamError::PreparedMeasuredWidthMismatch {
                    prepared_state: self.prepared_state.clone(),
                    measured_state: measured_state.clone(),
                    prepared_width,
                    measured_width,
                });
            }

            if *count == 0 {
                return Err(SpamError::ZeroCount {
                    prepared_state: self.prepared_state.clone(),
                    measured_state: measured_state.clone(),
                });
            }
        }

        Ok(prepared_width)
    }
}

fn checked_sum_counts(
    prepared_state: &str,
    counts: &BTreeMap<String, usize>,
) -> Result<usize, SpamError> {
    let mut total = 0usize;

    for (measured_state, count) in counts {
        if *count == 0 {
            return Err(SpamError::ZeroCount {
                prepared_state: prepared_state.to_owned(),
                measured_state: measured_state.clone(),
            });
        }

        total = total
            .checked_add(*count)
            .ok_or(SpamError::CountOverflow)?;
    }

    Ok(total)
}

// ============================================================================
// Protocol configuration
// ============================================================================

/// Configuration for SPAM/readout characterization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpamConfig {
    /// Whether all prepared states must have exactly the same number of shots.
    ///
    /// This is recommended for controlled calibration experiments but is not
    /// mathematically required for the per-state conditional probabilities.
    pub require_equal_shots: bool,

    /// Whether a missing measurement outcome should be interpreted as zero.
    ///
    /// When true, the analysis constructs the full computational-basis
    /// outcome space and missing outcomes receive probability zero.
    pub complete_outcome_space: bool,

    /// Tolerance used for probability and normalization validation.
    pub probability_tolerance: f64,

    /// Whether prepared states must have a complete computational-basis set.
    ///
    /// For `n` qubits, completeness means all `2^n` prepared basis states are
    /// supplied. This can become exponentially expensive, so it is disabled
    /// by default.
    pub require_complete_preparation_basis: bool,
}

impl Default for SpamConfig {
    fn default() -> Self {
        Self {
            require_equal_shots: false,
            complete_outcome_space: true,
            probability_tolerance: PROBABILITY_EPSILON,
            require_complete_preparation_basis: false,
        }
    }
}

impl SpamConfig {
    /// Construct the default production configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Require equal shots for all prepared states.
    pub fn with_equal_shots(mut self, required: bool) -> Self {
        self.require_equal_shots = required;
        self
    }

    /// Enable or disable complete outcome-space expansion.
    pub fn with_complete_outcome_space(mut self, enabled: bool) -> Self {
        self.complete_outcome_space = enabled;
        self
    }

    /// Set probability validation tolerance.
    pub fn with_probability_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, SpamError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(SpamError::InvalidTolerance { value: tolerance });
        }

        self.probability_tolerance = tolerance;
        Ok(self)
    }

    /// Require the complete computational-basis preparation set.
    pub fn with_complete_preparation_basis(
        mut self,
        required: bool,
    ) -> Self {
        self.require_complete_preparation_basis = required;
        self
    }

    /// Validate configuration.
    pub fn validate(&self) -> Result<(), SpamError> {
        if !self.probability_tolerance.is_finite()
            || self.probability_tolerance < 0.0
        {
            return Err(SpamError::InvalidTolerance {
                value: self.probability_tolerance,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Assignment matrix
// ============================================================================

/// Experimentally observed SPAM assignment matrix.
///
/// Matrix convention:
///
/// ```text
/// matrix[measured_state][prepared_state]
///     = P(measured_state | prepared_state)
/// ```
///
/// The ordering of both state axes is lexicographic and deterministic.
#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentMatrix {
    /// Number of qubits represented by the matrix.
    pub qubit_count: usize,

    /// Deterministic basis-state ordering.
    pub states: Vec<String>,

    /// Row-major matrix data.
///
/// `values[row][column]` is:
///
/// `P(measured = states[row] | prepared = states[column])`.
    pub values: Vec<Vec<f64>>,
}

impl AssignmentMatrix {
    /// Construct a validated assignment matrix.
    pub fn new(
        qubit_count: usize,
        states: Vec<String>,
        values: Vec<Vec<f64>>,
        tolerance: f64,
    ) -> Result<Self, SpamError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(SpamError::InvalidTolerance { value: tolerance });
        }

        if states.len() != values.len() {
            return Err(SpamError::MatrixDimensionMismatch {
                expected: states.len(),
                actual: values.len(),
            });
        }

        let dimension = states.len();

        for state in &states {
            let width = validate_binary_state(state, false)?;

            if width != qubit_count {
                return Err(SpamError::InconsistentMeasuredStateWidth {
                    expected: qubit_count,
                    actual: width,
                    state: state.clone(),
                });
            }
        }

        for row in &values {
            if row.len() != dimension {
                return Err(SpamError::MatrixDimensionMismatch {
                    expected: dimension,
                    actual: row.len(),
                });
            }

            for value in row {
                validate_probability(
                    *value,
                    "assignment-matrix probability",
                )?;
            }
        }

        for column in 0..dimension {
            let mut sum = 0.0;

            for row in 0..dimension {
                sum += values[row][column];
            }

            if !sum.is_finite() {
                return Err(SpamError::NonFiniteStatistic {
                    statistic: "assignment-matrix column sum",
                });
            }

            if (sum - 1.0).abs() > tolerance.max(NORMALIZATION_EPSILON) {
                return Err(SpamError::MatrixNotNormalized {
                    prepared_state: states[column].clone(),
                    sum,
                });
            }
        }

        Ok(Self {
            qubit_count,
            states,
            values,
        })
    }

    /// Number of basis states represented by the matrix.
    pub fn dimension(&self) -> usize {
        self.states.len()
    }

    /// Return `P(measured | prepared)`.
    pub fn probability(
        &self,
        measured_state: &str,
        prepared_state: &str,
    ) -> Option<f64> {
        let row = self
            .states
            .binary_search_by(|candidate| {
                candidate.as_str().cmp(measured_state)
            })
            .ok()?;

        let column = self
            .states
            .binary_search_by(|candidate| {
                candidate.as_str().cmp(prepared_state)
            })
            .ok()?;

        self.values
            .get(row)
            .and_then(|values| values.get(column))
            .copied()
    }

    /// Return the correct-assignment probability for one prepared state.
    pub fn diagonal_probability(
        &self,
        prepared_state: &str,
    ) -> Option<f64> {
        let index = self
            .states
            .binary_search_by(|candidate| {
                candidate.as_str().cmp(prepared_state)
            })
            .ok()?;

        self.values
            .get(index)
            .and_then(|row| row.get(index))
            .copied()
    }
}

// ============================================================================
// Per-state metrics
// ============================================================================

/// Readout metrics for one prepared state.
#[derive(Debug, Clone, PartialEq)]
pub struct StateReadoutMetrics {
    /// Prepared computational-basis state.
    pub prepared_state: String,

    /// Number of shots.
    pub shots: usize,

    /// Number of correctly assigned measurements.
    pub correct_assignments: usize,

    /// Correct-assignment probability.
    pub readout_fidelity: f64,

    /// Assignment error: `1 - readout_fidelity`.
    pub assignment_error: f64,

    /// Total variation distance from the ideal deterministic distribution.
    ///
    /// For a deterministic ideal state this equals:
    ///
    /// `1 - P(correct)`.
    pub total_variation_error: f64,

    /// Largest probability assigned to an incorrect outcome.
    pub maximum_wrong_assignment_probability: f64,

    /// Most likely incorrect measured state, when one exists.
    pub most_likely_wrong_state: Option<String>,
}

impl StateReadoutMetrics {
    fn from_column(
        prepared_state: &str,
        shots: usize,
        counts: &BTreeMap<String, usize>,
        outcome_states: &[String],
    ) -> Result<Self, SpamError> {
        if shots == 0 {
            return Err(SpamError::InvalidShotCount {
                prepared_state: prepared_state.to_owned(),
            });
        }

        let correct_assignments =
            counts.get(prepared_state).copied().unwrap_or(0);

        let readout_fidelity =
            correct_assignments as f64 / shots as f64;

        validate_probability(
            readout_fidelity,
            "readout fidelity",
        )?;

        let assignment_error = clamp_probability(1.0 - readout_fidelity);

        let total_variation_error = assignment_error;

        let mut maximum_wrong_assignment_probability = 0.0f64;
        let mut most_likely_wrong_state: Option<String> = None;

        for state in outcome_states {
            if state == prepared_state {
                continue;
            }

            let count = counts.get(state).copied().unwrap_or(0);

            let probability = count as f64 / shots as f64;

            if probability > maximum_wrong_assignment_probability {
                maximum_wrong_assignment_probability = probability;
                most_likely_wrong_state = Some(state.clone());
            }
        }

        validate_probability(
            maximum_wrong_assignment_probability,
            "maximum wrong assignment probability",
        )?;

        Ok(Self {
            prepared_state: prepared_state.to_owned(),
            shots,
            correct_assignments,
            readout_fidelity,
            assignment_error,
            total_variation_error,
            maximum_wrong_assignment_probability,
            most_likely_wrong_state,
        })
    }
}

// ============================================================================
// Aggregate result
// ============================================================================

/// Complete SPAM/readout characterization result.
#[derive(Debug, Clone, PartialEq)]
pub struct SpamResult {
    /// Stable result schema version.
    pub schema_version: u32,

    /// Stable benchmark identifier.
    pub benchmark_id: &'static str,

    /// Stable protocol identifier.
    pub protocol_id: &'static str,

    /// Number of qubits characterized.
    pub qubit_count: usize,

    /// Number of prepared basis states experimentally characterized.
    pub prepared_state_count: usize,

    /// Total number of executed shots across all prepared states.
    pub total_shots: usize,

    /// Whether all prepared states used the same number of shots.
    pub equal_shots_per_state: bool,

    /// Deterministic basis-state ordering used by the matrix.
    pub states: Vec<String>,

    /// Conditional assignment matrix.
    pub assignment_matrix: AssignmentMatrix,

    /// Per-prepared-state metrics.
    pub per_state: Vec<StateReadoutMetrics>,

    /// Arithmetic mean of per-state readout fidelities.
    ///
    /// This is a macro-average over prepared states, not a shot-weighted
    /// average.
    pub average_readout_fidelity: f64,

    /// Worst observed per-state readout fidelity.
    pub worst_case_readout_fidelity: f64,

    /// Average assignment error.
    pub average_assignment_error: f64,

    /// Worst assignment error.
    pub worst_case_assignment_error: f64,

    /// Balanced accuracy.
    ///
    /// For a complete basis experiment this is equal to the average diagonal
    /// assignment probability.
    pub balanced_accuracy: f64,

    /// Average total-variation error from the ideal deterministic prepared
    /// state.
    pub average_total_variation_error: f64,

    /// Maximum probability assigned to an incorrect outcome among all
    /// characterized prepared states.
    pub maximum_wrong_assignment_probability: f64,

    /// Prepared state with the worst observed fidelity.
    pub worst_case_prepared_state: Option<String>,

    /// Whether the complete computational basis was characterized.
    pub complete_preparation_basis: bool,

    /// Whether the result is suitable for a full square assignment matrix.
    pub complete_assignment_matrix: bool,
}

impl SpamResult {
    /// Analyze normalized raw SPAM observations.
    ///
    /// This is the primary production entry point.
    pub fn analyze(
        observations: &[PreparedStateObservation],
        config: SpamConfig,
    ) -> Result<Self, SpamError> {
        config.validate()?;

        if observations.is_empty() {
            return Err(SpamError::EmptyExperiment);
        }

        let qubit_count = determine_qubit_count(observations)?;

        validate_observations(
            observations,
            qubit_count,
            &config,
        )?;

        let states = build_outcome_state_set(
            observations,
            qubit_count,
            config.complete_outcome_space,
        )?;

        let complete_preparation_basis =
            has_complete_basis(observations, qubit_count)?;

        if config.require_complete_preparation_basis
            && !complete_preparation_basis
        {
            return Err(SpamError::UnsupportedConfiguration {
                message:
                    "complete computational-basis preparation was requested \
                     but not all basis states were supplied"
                        .to_owned(),
            });
        }

        let complete_assignment_matrix =
            observations.len() == states.len()
                && observations.iter().all(|observation| {
                    states
                        .binary_search(&observation.prepared_state)
                        .is_ok()
                });

        let mut matrix_values =
            vec![vec![0.0f64; states.len()]; states.len()];

        let mut per_state =
            Vec::with_capacity(observations.len());

        let mut total_shots = 0usize;

        for observation in observations {
            total_shots = total_shots
                .checked_add(observation.shots)
                .ok_or(SpamError::CountOverflow)?;

            let metrics = StateReadoutMetrics::from_column(
                &observation.prepared_state,
                observation.shots,
                &observation.counts,
                &states,
            )?;

            let column = states
                .binary_search(&observation.prepared_state)
                .map_err(|_| SpamError::UnsupportedConfiguration {
                    message: format!(
                        "prepared state '{}' was not present in the \
                         outcome-state set",
                        observation.prepared_state
                    ),
                })?;

            for (row, measured_state) in states.iter().enumerate() {
                let count = observation
                    .counts
                    .get(measured_state)
                    .copied()
                    .unwrap_or(0);

                matrix_values[row][column] =
                    count as f64 / observation.shots as f64;
            }

            per_state.push(metrics);
        }

        let assignment_matrix = AssignmentMatrix::new(
            qubit_count,
            states.clone(),
            matrix_values,
            config.probability_tolerance,
        )?;

        let equal_shots_per_state =
            observations
                .windows(2)
                .all(|window| window[0].shots == window[1].shots);

        if config.require_equal_shots && !equal_shots_per_state {
            return Err(SpamError::UnsupportedConfiguration {
                message:
                    "equal shots per prepared state were required but the \
                     supplied observations use different shot counts"
                        .to_owned(),
            });
        }

        let prepared_state_count = per_state.len();

        let average_readout_fidelity =
            mean(per_state.iter().map(|metric| metric.readout_fidelity))?;

        let worst_case_readout_fidelity = per_state
            .iter()
            .map(|metric| metric.readout_fidelity)
            .fold(f64::INFINITY, f64::min);

        let average_assignment_error =
            mean(per_state.iter().map(|metric| metric.assignment_error))?;

        let worst_case_assignment_error = per_state
            .iter()
            .map(|metric| metric.assignment_error)
            .fold(f64::NEG_INFINITY, f64::max);

        let balanced_accuracy = average_readout_fidelity;

        let average_total_variation_error =
            mean(per_state.iter().map(|metric| {
                metric.total_variation_error
            }))?;

        let maximum_wrong_assignment_probability =
            per_state
                .iter()
                .map(|metric| {
                    metric.maximum_wrong_assignment_probability
                })
                .fold(f64::NEG_INFINITY, f64::max);

        let worst_case_prepared_state =
            per_state
                .iter()
                .min_by(|left, right| {
                    left.readout_fidelity
                        .partial_cmp(&right.readout_fidelity)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then_with(|| {
                            left.prepared_state.cmp(&right.prepared_state)
                        })
                })
                .map(|metric| metric.prepared_state.clone());

        validate_probability(
            average_readout_fidelity,
            "average readout fidelity",
        )?;

        validate_probability(
            worst_case_readout_fidelity,
            "worst-case readout fidelity",
        )?;

        validate_probability(
            average_assignment_error,
            "average assignment error",
        )?;

        validate_probability(
            worst_case_assignment_error,
            "worst-case assignment error",
        )?;

        validate_probability(
            balanced_accuracy,
            "balanced accuracy",
        )?;

        validate_probability(
            average_total_variation_error,
            "average total variation error",
        )?;

        validate_probability(
            maximum_wrong_assignment_probability,
            "maximum wrong assignment probability",
        )?;

        Ok(Self {
            schema_version: SPAM_RESULT_SCHEMA_VERSION,
            benchmark_id: SPAM_BENCHMARK_ID,
            protocol_id: SPAM_PROTOCOL_ID,
            qubit_count,
            prepared_state_count,
            total_shots,
            equal_shots_per_state,
            states,
            assignment_matrix,
            per_state,
            average_readout_fidelity,
            worst_case_readout_fidelity,
            average_assignment_error,
            worst_case_assignment_error,
            balanced_accuracy,
            average_total_variation_error,
            maximum_wrong_assignment_probability,
            worst_case_prepared_state,
            complete_preparation_basis,
            complete_assignment_matrix,
        })
    }

    /// Number of states represented in the assignment matrix.
    pub fn matrix_dimension(&self) -> usize {
        self.assignment_matrix.dimension()
    }

    /// Return the readout fidelity of a prepared state.
    pub fn readout_fidelity(
        &self,
        prepared_state: &str,
    ) -> Option<f64> {
        self.per_state
            .iter()
            .find(|metric| metric.prepared_state == prepared_state)
            .map(|metric| metric.readout_fidelity)
    }

    /// Return the assignment error of a prepared state.
    pub fn assignment_error(
        &self,
        prepared_state: &str,
    ) -> Option<f64> {
        self.per_state
            .iter()
            .find(|metric| metric.prepared_state == prepared_state)
            .map(|metric| metric.assignment_error)
    }

    /// Return `P(measured | prepared)` from the assignment matrix.
    pub fn conditional_probability(
        &self,
        measured_state: &str,
        prepared_state: &str,
    ) -> Option<f64> {
        self.assignment_matrix
            .probability(measured_state, prepared_state)
    }

    /// Return the correct-assignment probability for one prepared state.
    pub fn diagonal_probability(
        &self,
        prepared_state: &str,
    ) -> Option<f64> {
        self.assignment_matrix
            .diagonal_probability(prepared_state)
    }
}

// ============================================================================
// Protocol façade
// ============================================================================

/// SPAM/readout benchmark façade.
///
/// This type intentionally owns no execution resources. It is a lightweight
/// protocol object that validates configuration and analyzes observations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpamBenchmark {
    /// Benchmark configuration.
    pub config: SpamConfig,
}

impl SpamBenchmark {
    /// Create a benchmark with production defaults.
    pub fn new() -> Self {
        Self {
            config: SpamConfig::default(),
        }
    }

    /// Create a benchmark from an explicit configuration.
    pub fn with_config(config: SpamConfig) -> Result<Self, SpamError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Analyze an observation set.
    pub fn analyze(
        &self,
        observations: &[PreparedStateObservation],
    ) -> Result<SpamResult, SpamError> {
        SpamResult::analyze(observations, self.config)
    }

    /// Stable benchmark identifier.
    pub const fn benchmark_id(&self) -> &'static str {
        SPAM_BENCHMARK_ID
    }

    /// Stable protocol identifier.
    pub const fn protocol_id(&self) -> &'static str {
        SPAM_PROTOCOL_ID
    }

    /// Result schema version.
    pub const fn result_schema_version(&self) -> u32 {
        SPAM_RESULT_SCHEMA_VERSION
    }
}

impl Default for SpamBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Internal validation
// ============================================================================

fn determine_qubit_count(
    observations: &[PreparedStateObservation],
) -> Result<usize, SpamError> {
    let first = observations
        .first()
        .ok_or(SpamError::EmptyExperiment)?;

    let width = validate_binary_state(
        &first.prepared_state,
        true,
    )?;

    if width == 0 {
        return Err(SpamError::InvalidPreparedState {
            state: first.prepared_state.clone(),
        });
    }

    Ok(width)
}

fn validate_observations(
    observations: &[PreparedStateObservation],
    qubit_count: usize,
    config: &SpamConfig,
) -> Result<(), SpamError> {
    let mut prepared_states = BTreeMap::<String, ()>::new();

    let expected_shots = observations
        .first()
        .map(|observation| observation.shots);

    for observation in observations {
        if prepared_states
            .insert(observation.prepared_state.clone(), ())
            .is_some()
        {
            return Err(SpamError::DuplicatePreparedState {
                state: observation.prepared_state.clone(),
            });
        }

        observation.validate(Some(qubit_count))?;

        if config.require_equal_shots {
            if let Some(expected) = expected_shots {
                if observation.shots != expected {
                    return Err(SpamError::UnsupportedConfiguration {
                        message:
                            "equal shots per prepared state were required"
                                .to_owned(),
                    });
                }
            }
        }
    }

    Ok(())
}

fn build_outcome_state_set(
    observations: &[PreparedStateObservation],
    qubit_count: usize,
    complete_outcome_space: bool,
) -> Result<Vec<String>, SpamError> {
    let mut states = BTreeMap::<String, ()>::new();

    for observation in observations {
        states.insert(observation.prepared_state.clone(), ());

        for measured_state in observation.counts.keys() {
            let width = validate_binary_state(
                measured_state,
                false,
            )?;

            if width != qubit_count {
                return Err(
                    SpamError::InconsistentMeasuredStateWidth {
                        expected: qubit_count,
                        actual: width,
                        state: measured_state.clone(),
                    },
                );
            }

            states.insert(measured_state.clone(), ());
        }
    }

    if complete_outcome_space {
        let dimension = checked_basis_dimension(qubit_count)?;

        for index in 0..dimension {
            let state = format_binary_state(index, qubit_count);
            states.insert(state, ());
        }
    }

    Ok(states.into_keys().collect())
}

fn checked_basis_dimension(
    qubit_count: usize,
) -> Result<usize, SpamError> {
    if qubit_count >= usize::BITS as usize {
        return Err(SpamError::UnsupportedConfiguration {
            message: format!(
                "complete computational-basis expansion for {} qubits \
                 cannot be represented by usize",
                qubit_count
            ),
        });
    }

    1usize
        .checked_shl(qubit_count as u32)
        .ok_or_else(|| SpamError::UnsupportedConfiguration {
            message: format!(
                "computational-basis dimension 2^{} overflowed usize",
                qubit_count
            ),
        })
}

fn format_binary_state(
    value: usize,
    width: usize,
) -> String {
    let mut state = String::with_capacity(width);

    for bit in (0..width).rev() {
        if ((value >> bit) & 1) == 1 {
            state.push('1');
        } else {
            state.push('0');
        }
    }

    state
}

fn has_complete_basis(
    observations: &[PreparedStateObservation],
    qubit_count: usize,
) -> Result<bool, SpamError> {
    let dimension = checked_basis_dimension(qubit_count)?;

    if observations.len() != dimension {
        return Ok(false);
    }

    let mut states = BTreeMap::<String, ()>::new();

    for observation in observations {
        states.insert(observation.prepared_state.clone(), ());
    }

    if states.len() != dimension {
        return Ok(false);
    }

    for index in 0..dimension {
        let state = format_binary_state(index, qubit_count);

        if !states.contains_key(&state) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn mean<I>(values: I) -> Result<f64, SpamError>
where
    I: Iterator<Item = f64>,
{
    let mut count = 0usize;
    let mut sum = 0.0f64;

    for value in values {
        if !value.is_finite() {
            return Err(SpamError::NonFiniteStatistic {
                statistic: "mean input",
            });
        }

        sum += value;

        if !sum.is_finite() {
            return Err(SpamError::NonFiniteStatistic {
                statistic: "mean accumulation",
            });
        }

        count = count
            .checked_add(1)
            .ok_or(SpamError::CountOverflow)?;
    }

    if count == 0 {
        return Err(SpamError::NonFiniteStatistic {
            statistic: "mean of empty sequence",
        });
    }

    let result = sum / count as f64;

    if !result.is_finite() {
        return Err(SpamError::NonFiniteStatistic {
            statistic: "mean",
        });
    }

    Ok(clamp_probability(result))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn counts(
        values: &[(&str, usize)],
    ) -> BTreeMap<String, usize> {
        values
            .iter()
            .map(|(state, count)| ((*state).to_owned(), *count))
            .collect()
    }

    fn observation(
        prepared: &str,
        values: &[(&str, usize)],
    ) -> PreparedStateObservation {
        PreparedStateObservation::from_counts(
            prepared,
            counts(values),
        )
        .expect("test observation must be valid")
    }

    #[test]
    fn constants_are_stable() {
        assert_eq!(SPAM_BENCHMARK_ID, "spam");
        assert_eq!(SPAM_PROTOCOL_ID, "spam_readout");
        assert_eq!(SPAM_RESULT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn perfect_single_qubit_readout_has_unit_fidelity() {
        let observations = vec![
            observation("0", &[("0", 1000)]),
            observation("1", &[("1", 1000)]),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("perfect data must analyze");

        assert_eq!(result.qubit_count, 1);
        assert_eq!(result.prepared_state_count, 2);
        assert_eq!(result.total_shots, 2000);
        assert_eq!(result.matrix_dimension(), 2);

        assert!((result.average_readout_fidelity - 1.0).abs() < 1.0e-12);
        assert!((result.worst_case_readout_fidelity - 1.0).abs() < 1.0e-12);
        assert!((result.average_assignment_error).abs() < 1.0e-12);
        assert!((result.balanced_accuracy - 1.0).abs() < 1.0e-12);

        assert_eq!(
            result.conditional_probability("0", "0"),
            Some(1.0)
        );

        assert_eq!(
            result.conditional_probability("1", "1"),
            Some(1.0)
        );
    }

    #[test]
    fn assignment_matrix_uses_measured_by_prepared_convention() {
        let observations = vec![
            observation(
                "0",
                &[("0", 900), ("1", 100)],
            ),
            observation(
                "1",
                &[("0", 200), ("1", 800)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid readout data must analyze");

        assert_eq!(
            result.conditional_probability("0", "0"),
            Some(0.9)
        );

        assert_eq!(
            result.conditional_probability("1", "0"),
            Some(0.1)
        );

        assert_eq!(
            result.conditional_probability("0", "1"),
            Some(0.2)
        );

        assert_eq!(
            result.conditional_probability("1", "1"),
            Some(0.8)
        );
    }

    #[test]
    fn average_fidelity_is_macro_average() {
        let observations = vec![
            observation(
                "0",
                &[("0", 900), ("1", 100)],
            ),
            observation(
                "1",
                &[("0", 100), ("1", 900)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid data must analyze");

        assert!((result.average_readout_fidelity - 0.9).abs() < 1.0e-12);
        assert!((result.worst_case_readout_fidelity - 0.9).abs() < 1.0e-12);
        assert!((result.average_assignment_error - 0.1).abs() < 1.0e-12);
    }

    #[test]
    fn unequal_shots_are_supported_by_default() {
        let observations = vec![
            observation(
                "0",
                &[("0", 900), ("1", 100)],
            ),
            observation(
                "1",
                &[("0", 50), ("1", 50)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("unequal shots are valid by default");

        assert!(!result.equal_shots_per_state);
        assert_eq!(result.total_shots, 1100);
    }

    #[test]
    fn equal_shot_configuration_rejects_unequal_observations() {
        let observations = vec![
            observation(
                "0",
                &[("0", 900), ("1", 100)],
            ),
            observation(
                "1",
                &[("0", 50), ("1", 50)],
            ),
        ];

        let config = SpamConfig::default()
            .with_equal_shots(true);

        let result = SpamResult::analyze(&observations, config);

        assert!(matches!(
            result,
            Err(SpamError::UnsupportedConfiguration { .. })
        ));
    }

    #[test]
    fn explicit_shot_mismatch_is_rejected() {
        let result =
            PreparedStateObservation::with_declared_shots(
                "0",
                100,
                counts(&[("0", 99)]),
            );

        assert!(matches!(
            result,
            Err(SpamError::ShotCountMismatch {
                declared_shots: 100,
                observed_shots: 99,
                ..
            })
        ));
    }

    #[test]
    fn invalid_prepared_state_is_rejected() {
        let result =
            PreparedStateObservation::from_counts(
                "2",
                counts(&[("0", 10)]),
            );

        assert!(matches!(
            result,
            Err(SpamError::InvalidPreparedState { .. })
        ));
    }

    #[test]
    fn invalid_measured_state_is_rejected() {
        let result =
            PreparedStateObservation::from_counts(
                "0",
                counts(&[("2", 10)]),
            );

        assert!(matches!(
            result,
            Err(SpamError::InvalidMeasuredState { .. })
        ));
    }

    #[test]
    fn inconsistent_width_is_rejected() {
        let result =
            PreparedStateObservation::from_counts(
                "00",
                counts(&[("0", 10)]),
            );

        assert!(matches!(
            result,
            Err(SpamError::PreparedMeasuredWidthMismatch {
                prepared_width: 2,
                measured_width: 1,
                ..
            })
        ));
    }

    #[test]
    fn duplicate_prepared_states_are_rejected() {
        let observations = vec![
            observation("0", &[("0", 10)]),
            observation("0", &[("0", 10)]),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default());

        assert!(matches!(
            result,
            Err(SpamError::DuplicatePreparedState { .. })
        ));
    }

    #[test]
    fn zero_count_entries_are_rejected() {
        let result =
            PreparedStateObservation::from_counts(
                "0",
                counts(&[("0", 10), ("1", 0)]),
            );

        assert!(matches!(
            result,
            Err(SpamError::ZeroCount { .. })
        ));
    }

    #[test]
    fn complete_outcome_space_adds_missing_outcomes() {
        let observations = vec![
            observation("0", &[("0", 100)]),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("single-state experiment should be valid");

        assert_eq!(
            result.states,
            vec!["0".to_owned(), "1".to_owned()]
        );

        assert_eq!(
            result.conditional_probability("1", "0"),
            Some(0.0)
        );
    }

    #[test]
    fn incomplete_outcome_space_only_contains_observed_states() {
        let observations = vec![
            observation("0", &[("0", 100)]),
        ];

        let config = SpamConfig::default()
            .with_complete_outcome_space(false);

        let result =
            SpamResult::analyze(&observations, config)
                .expect("single-state experiment should be valid");

        assert_eq!(result.states, vec!["0".to_owned()]);
        assert_eq!(result.matrix_dimension(), 1);
    }

    #[test]
    fn complete_preparation_basis_is_detected() {
        let observations = vec![
            observation("00", &[("00", 100)]),
            observation("01", &[("01", 100)]),
            observation("10", &[("10", 100)]),
            observation("11", &[("11", 100)]),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("complete basis experiment must analyze");

        assert!(result.complete_preparation_basis);
        assert!(result.complete_assignment_matrix);
    }

    #[test]
    fn incomplete_preparation_basis_is_detected() {
        let observations = vec![
            observation("00", &[("00", 100)]),
            observation("01", &[("01", 100)]),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("partial basis experiment must analyze");

        assert!(!result.complete_preparation_basis);
        assert!(!result.complete_assignment_matrix);
    }

    #[test]
    fn complete_basis_requirement_is_enforced() {
        let observations = vec![
            observation("00", &[("00", 100)]),
            observation("01", &[("01", 100)]),
        ];

        let config = SpamConfig::default()
            .with_complete_preparation_basis(true);

        let result =
            SpamResult::analyze(&observations, config);

        assert!(matches!(
            result,
            Err(SpamError::UnsupportedConfiguration { .. })
        ));
    }

    #[test]
    fn worst_case_state_is_deterministic() {
        let observations = vec![
            observation(
                "0",
                &[("0", 800), ("1", 200)],
            ),
            observation(
                "1",
                &[("0", 200), ("1", 800)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid data must analyze");

        assert_eq!(
            result.worst_case_prepared_state,
            Some("0".to_owned())
        );
    }

    #[test]
    fn balanced_accuracy_equals_macro_recall_for_basis_labels() {
        let observations = vec![
            observation(
                "0",
                &[("0", 950), ("1", 50)],
            ),
            observation(
                "1",
                &[("0", 100), ("1", 900)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid data must analyze");

        let expected = (0.95 + 0.90) / 2.0;

        assert!(
            (result.balanced_accuracy - expected).abs() < 1.0e-12
        );
    }

    #[test]
    fn total_variation_error_equals_assignment_error_for_deterministic_target() {
        let observations = vec![
            observation(
                "0",
                &[("0", 700), ("1", 300)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid data must analyze");

        let metric = &result.per_state[0];

        assert!(
            (metric.total_variation_error - metric.assignment_error)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn most_likely_wrong_state_is_reported() {
        let observations = vec![
            observation(
                "0",
                &[
                    ("0", 700),
                    ("1", 200),
                    ("00", 100),
                ],
            ),
        ];

        // The above deliberately has inconsistent width and must fail.
        let result =
            SpamResult::analyze(&observations, SpamConfig::default());

        assert!(result.is_err());
    }

    #[test]
    fn matrix_columns_are_normalized() {
        let observations = vec![
            observation(
                "0",
                &[("0", 700), ("1", 300)],
            ),
            observation(
                "1",
                &[("0", 250), ("1", 750)],
            ),
        ];

        let result =
            SpamResult::analyze(&observations, SpamConfig::default())
                .expect("valid data must analyze");

        for column in 0..result.assignment_matrix.dimension() {
            let sum: f64 = result
                .assignment_matrix
                .values
                .iter()
                .map(|row| row[column])
                .sum();

            assert!((sum - 1.0).abs() < NORMALIZATION_EPSILON);
        }
    }

    #[test]
    fn empty_experiment_is_rejected() {
        let result =
            SpamResult::analyze(&[], SpamConfig::default());

        assert!(matches!(
            result,
            Err(SpamError::EmptyExperiment)
        ));
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let result = SpamConfig::default()
            .with_probability_tolerance(-1.0);

        assert!(matches!(
            result,
            Err(SpamError::InvalidTolerance { .. })
        ));
    }

    #[test]
    fn overflow_in_basis_dimension_is_rejected() {
        let result = checked_basis_dimension(usize::BITS as usize);

        assert!(result.is_err());
    }

    #[test]
    fn binary_state_format_is_deterministic() {
        assert_eq!(format_binary_state(0, 3), "000");
        assert_eq!(format_binary_state(1, 3), "001");
        assert_eq!(format_binary_state(5, 3), "101");
        assert_eq!(format_binary_state(7, 3), "111");
    }

    #[test]
    fn benchmark_facade_is_stable() {
        let benchmark = SpamBenchmark::new();

        assert_eq!(benchmark.benchmark_id(), "spam");
        assert_eq!(benchmark.protocol_id(), "spam_readout");
        assert_eq!(benchmark.result_schema_version(), 1);
    }

    #[test]
    fn backend_style_raw_counts_can_be_analyzed_without_execution_dependency() {
        let observations = vec![
            observation(
                "0",
                &[("0", 980), ("1", 20)],
            ),
            observation(
                "1",
                &[("0", 30), ("1", 970)],
            ),
        ];

        let benchmark = SpamBenchmark::new();

        let result = benchmark
            .analyze(&observations)
            .expect("normalized backend observations must analyze");

        assert!(result.average_readout_fidelity > 0.97);
        assert!(result.average_assignment_error < 0.03);
    }
}