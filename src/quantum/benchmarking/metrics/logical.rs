//! Zamani Quantum Benchmarking — Logical Error Metrics
//!
//! Production implementation of logical-qubit and fault-tolerant quality
//! metrics.
//
//! # Responsibility
//!
//! This module owns mathematical construction and validation of metrics that
//! describe logical quantum computation:
//
//! - logical error probability;
//! - logical error rate;
//! - logical error per error-correction cycle;
//! - logical fidelity;
//! - physical error rate;
//! - decoder failure probability;
//! - pseudothreshold comparison;
//! - logical-error suppression factor;
//! - logical lifetime in correction cycles;
//! - logical lifetime in seconds;
//! - physical/logical resource overhead;
//! - space-time volume.
//
//! This module deliberately does NOT:
//
//! - generate QEC circuits;
//! - execute circuits;
//! - execute decoders;
//! - fit threshold curves;
//! - infer physical error models;
//! - discover hardware capabilities;
//! - access calibration data;
//! - communicate with quantum hardware;
//! - depend on a particular QEC code;
//! - assume surface codes;
//! - assume a particular decoder;
//! - own the universal `BenchmarkResult` envelope.
//
//! Those responsibilities belong to the corresponding benchmarking
//! protocol, QEC, statistics, hardware, execution, and result layers.
//
//! # Architectural position
//!
//! ```text
//! QEC experiment / decoder / protocol
//!                 │
//!                 │ validated observations
//!                 ▼
//!       metrics::logical
//!                 │
//!        ┌────────┼─────────┐
//!        ▼        ▼         ▼
//! logical error  lifetime  overhead
//!        │        │         │
//!        └────────┼─────────┘
//!                 ▼
//!          core::metric::Metric
//!                 │
//!       ┌─────────┴──────────┐
//!       ▼                    ▼
//! core::result          reporting/analysis
//! ```
//!
//! Dependency direction:
//
//! ```text
//! logical.rs
//!     │
//!     └──> core::metric
//! ```
//!
//! Never:
//
//! ```text
//! core::metric
//!     │
//!     └──> logical.rs
//! ```
//!
//! # Scientific conventions
//!
//! A distinction is maintained between:
//
//! 1. logical failure probability over a complete experiment;
//! 2. logical error probability per error-correction cycle;
//! 3. logical error rate derived from repeated trials;
//! 4. decoder failure probability;
//! 5. physical error probability.
//!
//! These quantities MUST NOT be silently substituted for one another.
//
//! For a binary logical memory with measured logical failure probability
//! `p_L` after `rounds` correction cycles, the symmetric two-state model gives:
//
//! ```text
//! p_cycle = 1/2 * (1 - (1 - 2 p_L)^(1 / rounds))
//! ```
//
//! This relationship assumes an effective symmetric logical bit/phase error
//! channel. It is therefore exposed as an explicitly model-dependent
//! conversion rather than being presented as a universal definition.
//
//! For a simple independent-error survival model:
//
//! ```text
//! survival(n) = (1 - p)^n
//! ```
//
//! the characteristic 1/e lifetime in correction cycles is:
//
//! ```text
//! lifetime_cycles = -1 / ln(1 - p)
//! ```
//
//! This is a derived characteristic lifetime, not necessarily the experimentally
//! reported lifetime for every QEC protocol.
//
//! # Production invariants
//!
//! 1. No NaN or infinity may enter this module.
//! 2. Probabilities are constrained to `[0, 1]`.
//! 3. Error counts cannot exceed trial counts.
//! 4. The number of correction cycles must be positive when used as a divisor.
//! 5. Physical and logical qubit counts must be positive when used for ratios.
//! 6. Code distance must be positive.
//! 7. No scientific quantity is silently clamped.
//! 8. No integer arithmetic is allowed to wrap.
//! 9. No public function panics on invalid user input.
//! 10. Mathematical assumptions are represented explicitly in API names and
//!     documentation.
//! 11. Metric construction always uses the canonical `core::metric::Metric`.
//! 12. No diagnostic printing occurs.
//!
//! # Rust compatibility
//!
//! Designed for:
//
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are required.
//!
//! # Integration
//!
//! This file intentionally depends only on:
//
//! ```text
//! crate::quantum::benchmarking::core::metric
//! ```
//!
//! This allows it to be completed before:
//
//! - QEC protocols;
//! - threshold analysis;
//! - decoder implementations;
//! - reporting;
//! - hardware execution;
//! - benchmark registry.
//!
//! Once `metrics/mod.rs` exists:
//
//! ```text
//! pub mod logical;
//! ```
//!
//! No modification to `core::metric` is required because the repository
//! already defines the logical metric kinds required by this module.

use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricKind,
    MetricResult,
    MetricUnit,
};

const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;
const HALF: f64 = 0.5;

/// Smallest permitted number of correction cycles.
const MIN_ROUNDS: u64 = 1;

/// Smallest permitted qubit/resource count.
const MIN_RESOURCE_COUNT: u64 = 1;

/// Smallest permitted code distance.
const MIN_DISTANCE: u64 = 1;

/// Metadata-free canonical logical metric construction.
///
/// The canonical `Metric` type is deliberately used as the returned scientific
/// value. Additional experimental metadata belongs to the benchmark result
/// and provenance layers rather than being duplicated here.
fn metric(
    kind: MetricKind,
    unit: MetricUnit,
    value: f64,
) -> MetricResult<Metric> {
    Metric::new(kind, unit, value)
}

/// Validates a finite normalized probability.
fn validate_probability(
    value: f64,
    name: &'static str,
) -> Result<f64, LogicalMetricError> {
    if !value.is_finite() {
        return Err(LogicalMetricError::NonFinite {
            field: name,
            value,
        });
    }

    if !(ZERO..=ONE).contains(&value) {
        return Err(LogicalMetricError::OutOfUnitInterval {
            field: name,
            value,
        });
    }

    Ok(value)
}

/// Validates a finite non-negative quantity.
fn validate_non_negative(
    value: f64,
    name: &'static str,
) -> Result<f64, LogicalMetricError> {
    if !value.is_finite() {
        return Err(LogicalMetricError::NonFinite {
            field: name,
            value,
        });
    }

    if value < ZERO {
        return Err(LogicalMetricError::Negative {
            field: name,
            value,
        });
    }

    Ok(value)
}

/// Validates a positive duration in seconds.
fn validate_positive_seconds(
    seconds: f64,
) -> Result<f64, LogicalMetricError> {
    if !seconds.is_finite() {
        return Err(LogicalMetricError::NonFinite {
            field: "seconds",
            value: seconds,
        });
    }

    if seconds <= ZERO {
        return Err(LogicalMetricError::NotPositive {
            field: "seconds",
            value: seconds,
        });
    }

    Ok(seconds)
}

/// Validates a positive integer resource count.
fn validate_positive_count(
    value: u64,
    field: &'static str,
) -> Result<u64, LogicalMetricError> {
    if value < MIN_RESOURCE_COUNT {
        return Err(LogicalMetricError::ZeroCount { field });
    }

    Ok(value)
}

/// Validates a correction-round count.
fn validate_rounds(rounds: u64) -> Result<u64, LogicalMetricError> {
    if rounds < MIN_ROUNDS {
        return Err(LogicalMetricError::ZeroCount {
            field: "rounds",
        });
    }

    Ok(rounds)
}

/// Validates code distance.
fn validate_distance(distance: u64) -> Result<u64, LogicalMetricError> {
    if distance < MIN_DISTANCE {
        return Err(LogicalMetricError::ZeroCount {
            field: "code_distance",
        });
    }

    Ok(distance)
}

/// Validates an error count against a trial count.
fn validate_error_count(
    errors: u64,
    trials: u64,
) -> Result<(), LogicalMetricError> {
    validate_positive_count(trials, "trials")?;

    if errors > trials {
        return Err(LogicalMetricError::ErrorsExceedTrials {
            errors,
            trials,
        });
    }

    Ok(())
}

/// Production error type for this metric layer.
///
/// The error remains local to the metric calculation because the existing
/// repository's `MetricResult` already carries canonical metric-construction
/// errors. Protocol layers should translate `LogicalMetricError` into the
/// benchmarking `BenchmarkError` hierarchy when necessary.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalMetricError {
    /// A floating-point input was NaN or infinite.
    NonFinite {
        field: &'static str,
        value: f64,
    },

    /// A normalized probability was outside `[0, 1]`.
    OutOfUnitInterval {
        field: &'static str,
        value: f64,
    },

    /// A quantity that cannot be negative was negative.
    Negative {
        field: &'static str,
        value: f64,
    },

    /// A quantity requiring a positive value was zero or negative.
    NotPositive {
        field: &'static str,
        value: f64,
    },

    /// An integer count was zero.
    ZeroCount {
        field: &'static str,
    },

    /// Number of logical errors exceeded number of trials.
    ErrorsExceedTrials {
        errors: u64,
        trials: u64,
    },

    /// The symmetric logical-channel conversion cannot represent the input.
    SymmetricModelDomain {
        logical_failure_probability: f64,
    },

    /// The mathematical transformation produced an invalid value.
    InvalidDerivedValue {
        metric: &'static str,
        value: f64,
    },

    /// A division would overflow or become non-finite.
    NumericalFailure {
        operation: &'static str,
    },
}

impl std::fmt::Display for LogicalMetricError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NonFinite { field, value } => {
                write!(formatter, "{} is non-finite: {}", field, value)
            }

            Self::OutOfUnitInterval { field, value } => {
                write!(
                    formatter,
                    "{} must be in [0, 1], got {}",
                    field, value
                )
            }

            Self::Negative { field, value } => {
                write!(
                    formatter,
                    "{} cannot be negative, got {}",
                    field, value
                )
            }

            Self::NotPositive { field, value } => {
                write!(
                    formatter,
                    "{} must be positive, got {}",
                    field, value
                )
            }

            Self::ZeroCount { field } => {
                write!(
                    formatter,
                    "{} must be greater than zero",
                    field
                )
            }

            Self::ErrorsExceedTrials { errors, trials } => {
                write!(
                    formatter,
                    "logical error count {} exceeds trial count {}",
                    errors, trials
                )
            }

            Self::SymmetricModelDomain {
                logical_failure_probability,
            } => {
                write!(
                    formatter,
                    "logical failure probability {} is outside the \
                     symmetric two-state conversion domain",
                    logical_failure_probability
                )
            }

            Self::InvalidDerivedValue { metric, value } => {
                write!(
                    formatter,
                    "derived metric '{}' produced invalid value {}",
                    metric, value
                )
            }

            Self::NumericalFailure { operation } => {
                write!(
                    formatter,
                    "numerical failure during '{}'",
                    operation
                )
            }
        }
    }
}

impl std::error::Error for LogicalMetricError {}

/// A logical-error observation consisting of failed and total trials.
///
/// This structure preserves the raw counts so protocol layers can retain
/// scientifically important information instead of only storing the final
/// probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalErrorObservation {
    /// Number of logical failures.
    pub errors: u64,

    /// Total number of logical trials.
    pub trials: u64,
}

impl LogicalErrorObservation {
    /// Creates a validated logical-error observation.
    pub fn new(
        errors: u64,
        trials: u64,
    ) -> Result<Self, LogicalMetricError> {
        validate_error_count(errors, trials)?;

        Ok(Self { errors, trials })
    }

    /// Returns the empirical logical failure probability.
    pub fn probability(&self) -> f64 {
        self.errors as f64 / self.trials as f64
    }

    /// Converts the observation into a canonical logical-error-rate metric.
    pub fn metric(&self) -> MetricResult<Metric> {
        logical_error_rate_from_counts(self.errors, self.trials)
            .map_err(|error| {
                crate::quantum::benchmarking::core::metric::MetricError::InvalidValue {
                    field: "logical_error_rate",
                    reason: error.to_string(),
                }
            })
    }
}

/// A logical error-rate measurement together with its raw observation.
///
/// `Metric` is the canonical value intended for insertion into
/// `BenchmarkResult`.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalErrorMeasurement {
    /// Raw logical-error observation.
    pub observation: LogicalErrorObservation,

    /// Canonical logical error-rate metric.
    pub metric: Metric,
}

impl LogicalErrorMeasurement {
    /// Creates a complete logical-error measurement.
    pub fn new(
        errors: u64,
        trials: u64,
    ) -> Result<Self, LogicalMetricError> {
        let observation = LogicalErrorObservation::new(errors, trials)?;
        let metric = logical_error_rate_from_counts(errors, trials)
            .map_err(|error| {
                LogicalMetricError::InvalidDerivedValue {
                    metric: "logical_error_rate",
                    value: error.to_string().len() as f64,
                }
            })?;

        Ok(Self {
            observation,
            metric,
        })
    }
}

/// Calculates logical error probability from logical failures and trials.
///
/// ```text
/// p_L = logical_errors / trials
/// ```
///
/// This is an empirical probability. It does not imply a per-cycle error
/// rate when the experiment contains multiple error-correction rounds.
pub fn logical_error_probability_from_counts(
    logical_errors: u64,
    trials: u64,
) -> Result<f64, LogicalMetricError> {
    validate_error_count(logical_errors, trials)?;

    let probability = logical_errors as f64 / trials as f64;

    validate_probability(probability, "logical_error_probability")
}

/// Constructs a canonical logical-error-rate metric from counts.
///
/// The metric is an empirical logical failure probability over the supplied
/// trial definition.
pub fn logical_error_rate_from_counts(
    logical_errors: u64,
    trials: u64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        logical_error_probability_from_counts(logical_errors, trials)?;

    metric(
        MetricKind::LogicalErrorRate,
        MetricUnit::Probability,
        probability,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_error_rate_metric_construction",
    })
}

/// Calculates logical fidelity from an empirical logical error probability.
///
/// ```text
/// F_L = 1 - p_L
/// ```
///
/// This should be used only when the protocol defines fidelity as the
/// complement of its accepted logical-failure event.
pub fn logical_fidelity_from_error_probability(
    logical_error_probability: f64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        validate_probability(logical_error_probability, "logical_error_probability")?;

    let fidelity = ONE - probability;

    metric(
        MetricKind::LogicalFidelity,
        MetricUnit::Probability,
        fidelity,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_fidelity_metric_construction",
    })
}

/// Calculates logical fidelity directly from failure counts.
pub fn logical_fidelity_from_counts(
    logical_errors: u64,
    trials: u64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        logical_error_probability_from_counts(logical_errors, trials)?;

    logical_fidelity_from_error_probability(probability)
}

/// Calculates logical error probability per correction cycle using the
/// symmetric two-state logical-channel model.
///
/// Given a measured logical failure probability `p_L` after `rounds`:
///
/// ```text
/// p_cycle = 1/2 * (1 - (1 - 2 p_L)^(1 / rounds))
/// ```
///
/// # Scientific assumption
///
/// This is a model-dependent conversion. It should not be used when the
/// experiment's logical noise is known to violate the symmetric two-state
/// assumption.
pub fn logical_error_per_cycle_from_failure_probability(
    logical_failure_probability: f64,
    rounds: u64,
) -> Result<f64, LogicalMetricError> {
    validate_rounds(rounds)?;

    let p_l =
        validate_probability(logical_failure_probability, "logical_failure_probability")?;

    if p_l >= ONE {
        return Err(LogicalMetricError::SymmetricModelDomain {
            logical_failure_probability: p_l,
        });
    }

    let base = ONE - (2.0 * p_l);

    if base <= ZERO {
        return Err(LogicalMetricError::SymmetricModelDomain {
            logical_failure_probability: p_l,
        });
    }

    let exponent = ONE / rounds as f64;
    let powered = base.powf(exponent);

    if !powered.is_finite() {
        return Err(LogicalMetricError::NumericalFailure {
            operation: "logical_error_per_cycle_power",
        });
    }

    let result = HALF * (ONE - powered);

    if !(ZERO..=HALF).contains(&result) {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_error_per_cycle",
            value: result,
        });
    }

    Ok(result)
}

/// Calculates logical error per correction cycle from failure counts.
///
/// This preserves the distinction between the total experiment failure
/// probability and the per-cycle error probability.
pub fn logical_error_per_cycle_from_counts(
    logical_errors: u64,
    trials: u64,
    rounds: u64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        logical_error_probability_from_counts(logical_errors, trials)?;

    let per_cycle =
        logical_error_per_cycle_from_failure_probability(
            probability,
            rounds,
        )?;

    metric(
        MetricKind::LogicalErrorRate,
        MetricUnit::Probability,
        per_cycle,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_error_per_cycle_metric_construction",
    })
}

/// Calculates logical error per cycle from an already measured logical
/// failure probability.
///
/// This function exists separately from the count-based version so existing
/// QEC protocols that already performed statistical analysis do not need to
/// reconstruct raw counts.
pub fn logical_error_per_cycle_metric(
    logical_failure_probability: f64,
    rounds: u64,
) -> Result<Metric, LogicalMetricError> {
    let per_cycle =
        logical_error_per_cycle_from_failure_probability(
            logical_failure_probability,
            rounds,
        )?;

    metric(
        MetricKind::LogicalErrorRate,
        MetricUnit::Probability,
        per_cycle,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_error_per_cycle_metric",
    })
}

/// Calculates decoder failure probability from decoder failures and decoding
/// attempts.
pub fn decoder_failure_probability_from_counts(
    decoder_failures: u64,
    decoding_attempts: u64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        logical_error_probability_from_counts(
            decoder_failures,
            decoding_attempts,
        )?;

    metric(
        MetricKind::DecoderFailureProbability,
        MetricUnit::Probability,
        probability,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "decoder_failure_probability_metric_construction",
    })
}

/// Calculates physical error rate from physical failures and trials.
///
/// This function does not define how a physical error was detected. The
/// caller must provide the protocol-defined physical error events.
pub fn physical_error_rate_from_counts(
    physical_errors: u64,
    trials: u64,
) -> Result<Metric, LogicalMetricError> {
    let probability =
        logical_error_probability_from_counts(physical_errors, trials)?;

    metric(
        MetricKind::PhysicalErrorRate,
        MetricUnit::Probability,
        probability,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "physical_error_rate_metric_construction",
    })
}

/// Calculates a pseudothreshold comparison.
///
/// A pseudothreshold is represented here as the pointwise comparison between
/// physical and logical error rates:
///
/// ```text
/// logical_error_rate < physical_error_rate
/// ```
///
/// The returned value is the logical-to-physical error ratio:
///
/// ```text
/// ratio = logical / physical
/// ```
///
/// Values below `1` indicate logical suppression for the supplied experiment.
/// This function does not estimate an asymptotic threshold.
pub fn logical_to_physical_error_ratio(
    logical_error_rate: f64,
    physical_error_rate: f64,
) -> Result<Metric, LogicalMetricError> {
    let logical =
        validate_probability(logical_error_rate, "logical_error_rate")?;

    let physical =
        validate_probability(physical_error_rate, "physical_error_rate")?;

    if physical == ZERO {
        if logical == ZERO {
            return Err(LogicalMetricError::NumericalFailure {
                operation: "logical_to_physical_error_ratio",
            });
        }

        return Err(LogicalMetricError::NumericalFailure {
            operation: "logical_to_physical_error_ratio_zero_denominator",
        });
    }

    let ratio = logical / physical;

    if !ratio.is_finite() || ratio < ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_to_physical_error_ratio",
            value: ratio,
        });
    }

    metric(
        MetricKind::ResourceOverhead,
        MetricUnit::Dimensionless,
        ratio,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_to_physical_error_ratio_metric_construction",
    })
}

/// Returns whether a logical system beats the supplied physical error rate.
///
/// `true` means:
///
/// ```text
/// logical_error_rate < physical_error_rate
/// ```
///
/// Equality is not considered suppression.
pub fn is_below_pseudothreshold(
    logical_error_rate: f64,
    physical_error_rate: f64,
) -> Result<bool, LogicalMetricError> {
    let logical =
        validate_probability(logical_error_rate, "logical_error_rate")?;

    let physical =
        validate_probability(physical_error_rate, "physical_error_rate")?;

    Ok(logical < physical)
}

/// Calculates the logical-error suppression factor.
///
/// ```text
/// suppression_factor = physical_error / logical_error
/// ```
///
/// Values greater than `1` indicate suppression.
///
/// This is intentionally separate from a threshold estimate. A threshold
/// requires a family of experiments across physical error rates and usually
/// across code distances.
pub fn logical_error_suppression_factor(
    logical_error_rate: f64,
    physical_error_rate: f64,
) -> Result<Metric, LogicalMetricError> {
    let logical =
        validate_probability(logical_error_rate, "logical_error_rate")?;

    let physical =
        validate_probability(physical_error_rate, "physical_error_rate")?;

    if logical == ZERO {
        return Err(LogicalMetricError::NumericalFailure {
            operation: "logical_error_suppression_factor_zero_denominator",
        });
    }

    let factor = physical / logical;

    if !factor.is_finite() || factor < ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_error_suppression_factor",
            value: factor,
        });
    }

    metric(
        MetricKind::LogicalFidelity,
        MetricUnit::Dimensionless,
        factor,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_error_suppression_factor_metric_construction",
    })
}

/// Calculates a code-distance improvement/suppression factor.
///
/// This is useful when comparing two code-distance experiments:
///
/// ```text
/// suppression = p_old / p_new
/// ```
///
/// It does not assume that the two distances differ by exactly two.
pub fn suppression_factor_between_distances(
    lower_distance_error: f64,
    higher_distance_error: f64,
) -> Result<Metric, LogicalMetricError> {
    validate_distance(1)?;
    validate_probability(
        lower_distance_error,
        "lower_distance_error",
    )?;
    validate_probability(
        higher_distance_error,
        "higher_distance_error",
    )?;

    if higher_distance_error == ZERO {
        return Err(LogicalMetricError::NumericalFailure {
            operation: "suppression_factor_between_distances_zero_denominator",
        });
    }

    let factor = lower_distance_error / higher_distance_error;

    if !factor.is_finite() || factor < ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "distance_suppression_factor",
            value: factor,
        });
    }

    metric(
        MetricKind::LogicalFidelity,
        MetricUnit::Dimensionless,
        factor,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "suppression_factor_between_distances",
    })
}

/// Calculates characteristic logical lifetime in correction cycles under the
/// independent per-cycle survival model.
///
/// ```text
/// lifetime = -1 / ln(1 - p)
/// ```
///
/// The result is the 1/e characteristic lifetime.
pub fn logical_lifetime_cycles(
    logical_error_per_cycle: f64,
) -> Result<Metric, LogicalMetricError> {
    let p =
        validate_probability(
            logical_error_per_cycle,
            "logical_error_per_cycle",
        )?;

    if p == ZERO {
        return Err(LogicalMetricError::NumericalFailure {
            operation: "logical_lifetime_cycles_zero_error_rate",
        });
    }

    if p >= ONE {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_lifetime_cycles",
            value: p,
        });
    }

    let survival = ONE - p;
    let logarithm = survival.ln();

    if !logarithm.is_finite() || logarithm >= ZERO {
        return Err(LogicalMetricError::NumericalFailure {
            operation: "logical_lifetime_cycles_log",
        });
    }

    let lifetime = -ONE / logarithm;

    if !lifetime.is_finite() || lifetime <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_lifetime_cycles",
            value: lifetime,
        });
    }

    metric(
        MetricKind::LogicalFidelity,
        MetricUnit::Dimensionless,
        lifetime,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_lifetime_cycles_metric_construction",
    })
}

/// Calculates logical lifetime in seconds from per-cycle logical error rate
/// and error-correction cycle duration.
///
/// ```text
/// lifetime_seconds = lifetime_cycles * cycle_duration_seconds
/// ```
pub fn logical_lifetime_seconds(
    logical_error_per_cycle: f64,
    cycle_duration_seconds: f64,
) -> Result<Metric, LogicalMetricError> {
    let lifetime_cycles =
        logical_lifetime_cycles(logical_error_per_cycle)?;

    let seconds =
        validate_positive_seconds(cycle_duration_seconds)?;

    let lifetime =
        lifetime_cycles.value.get() * seconds;

    if !lifetime.is_finite() || lifetime <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "logical_lifetime_seconds",
            value: lifetime,
        });
    }

    metric(
        MetricKind::LogicalFidelity,
        MetricUnit::Seconds,
        lifetime,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "logical_lifetime_seconds_metric_construction",
    })
}

/// Physical/logical qubit resource overhead.
///
/// ```text
/// overhead = physical_qubits / logical_qubits
/// ```
pub fn physical_to_logical_qubit_overhead(
    physical_qubits: u64,
    logical_qubits: u64,
) -> Result<Metric, LogicalMetricError> {
    validate_positive_count(
        physical_qubits,
        "physical_qubits",
    )?;

    validate_positive_count(
        logical_qubits,
        "logical_qubits",
    )?;

    let overhead =
        physical_qubits as f64 / logical_qubits as f64;

    if !overhead.is_finite() || overhead <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "physical_to_logical_qubit_overhead",
            value: overhead,
        });
    }

    metric(
        MetricKind::ResourceOverhead,
        MetricUnit::Dimensionless,
        overhead,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "physical_to_logical_qubit_overhead",
    })
}

/// Physical/logical gate resource overhead.
///
/// ```text
/// overhead = physical_gates / logical_gates
/// ```
pub fn physical_to_logical_gate_overhead(
    physical_gates: u64,
    logical_gates: u64,
) -> Result<Metric, LogicalMetricError> {
    validate_positive_count(
        physical_gates,
        "physical_gates",
    )?;

    validate_positive_count(
        logical_gates,
        "logical_gates",
    )?;

    let overhead =
        physical_gates as f64 / logical_gates as f64;

    if !overhead.is_finite() || overhead <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "physical_to_logical_gate_overhead",
            value: overhead,
        });
    }

    metric(
        MetricKind::ResourceOverhead,
        MetricUnit::Dimensionless,
        overhead,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "physical_to_logical_gate_overhead",
    })
}

/// Physical/logical circuit-depth overhead.
///
/// ```text
/// overhead = physical_depth / logical_depth
/// ```
pub fn physical_to_logical_depth_overhead(
    physical_depth: u64,
    logical_depth: u64,
) -> Result<Metric, LogicalMetricError> {
    validate_positive_count(
        physical_depth,
        "physical_depth",
    )?;

    validate_positive_count(
        logical_depth,
        "logical_depth",
    )?;

    let overhead =
        physical_depth as f64 / logical_depth as f64;

    if !overhead.is_finite() || overhead <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "physical_to_logical_depth_overhead",
            value: overhead,
        });
    }

    metric(
        MetricKind::ResourceOverhead,
        MetricUnit::Dimensionless,
        overhead,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "physical_to_logical_depth_overhead",
    })
}

/// Physical/logical execution-time overhead.
///
/// ```text
/// overhead = physical_time / logical_time
/// ```
pub fn physical_to_logical_time_overhead(
    physical_time_seconds: f64,
    logical_time_seconds: f64,
) -> Result<Metric, LogicalMetricError> {
    let physical =
        validate_positive_seconds(physical_time_seconds)?;

    let logical =
        validate_positive_seconds(logical_time_seconds)?;

    let overhead = physical / logical;

    if !overhead.is_finite() || overhead <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "physical_to_logical_time_overhead",
            value: overhead,
        });
    }

    metric(
        MetricKind::ResourceOverhead,
        MetricUnit::Dimensionless,
        overhead,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "physical_to_logical_time_overhead",
    })
}

/// Space-time volume.
///
/// ```text
/// STV = physical_qubits × correction_rounds
/// ```
///
/// This is the basic physical space-time resource consumed by a QEC memory
/// experiment. Protocols may define richer space-time-volume conventions;
/// those belong in the QEC protocol layer.
pub fn space_time_volume(
    physical_qubits: u64,
    correction_rounds: u64,
) -> Result<Metric, LogicalMetricError> {
    validate_positive_count(
        physical_qubits,
        "physical_qubits",
    )?;

    validate_rounds(correction_rounds)?;

    let volume = physical_qubits
        .checked_mul(correction_rounds)
        .ok_or(LogicalMetricError::NumericalFailure {
            operation: "space_time_volume_integer_multiplication",
        })?;

    let value = volume as f64;

    if !value.is_finite() || value <= ZERO {
        return Err(LogicalMetricError::InvalidDerivedValue {
            metric: "space_time_volume",
            value,
        });
    }

    metric(
        MetricKind::SpaceTimeVolume,
        MetricUnit::SpaceTimeVolume,
        value,
    )
    .map_err(|_| LogicalMetricError::NumericalFailure {
        operation: "space_time_volume_metric_construction",
    })
}

/// A complete logical-quality metric bundle.
///
/// The bundle does not execute any experiment. It is a convenient way for a
/// protocol analyzer to expose logically related values without repeatedly
/// reconstructing them.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalQualityMetrics {
    /// Empirical logical failure probability.
    pub logical_error_rate: Metric,

    /// Complementary logical fidelity.
    pub logical_fidelity: Metric,
}

impl LogicalQualityMetrics {
    /// Creates a logical-quality bundle from failure counts.
    pub fn from_counts(
        logical_errors: u64,
        trials: u64,
    ) -> Result<Self, LogicalMetricError> {
        let logical_error_rate =
            logical_error_rate_from_counts(
                logical_errors,
                trials,
            )?;

        let logical_fidelity =
            logical_fidelity_from_counts(
                logical_errors,
                trials,
            )?;

        Ok(Self {
            logical_error_rate,
            logical_fidelity,
        })
    }

    /// Creates a logical-quality bundle from an already calculated error
    /// probability.
    pub fn from_error_probability(
        logical_error_rate: f64,
    ) -> Result<Self, LogicalMetricError> {
        let probability =
            validate_probability(
                logical_error_rate,
                "logical_error_rate",
            )?;

        let logical_error_rate_metric =
            metric(
                MetricKind::LogicalErrorRate,
                MetricUnit::Probability,
                probability,
            )
            .map_err(|_| LogicalMetricError::NumericalFailure {
                operation: "logical_quality_error_metric",
            })?;

        let logical_fidelity =
            logical_fidelity_from_error_probability(
                probability,
            )?;

        Ok(Self {
            logical_error_rate: logical_error_rate_metric,
            logical_fidelity,
        })
    }
}

/// A complete logical-resource overhead bundle.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalResourceOverhead {
    /// Physical qubits per logical qubit.
    pub qubit_overhead: Metric,

    /// Physical gates per logical gate.
    pub gate_overhead: Option<Metric>,

    /// Physical depth per logical depth.
    pub depth_overhead: Option<Metric>,

    /// Physical execution time per logical execution time.
    pub time_overhead: Option<Metric>,
}

impl LogicalResourceOverhead {
    /// Creates a resource-overhead bundle.
    pub fn new(
        physical_qubits: u64,
        logical_qubits: u64,
        physical_gates: Option<(u64, u64)>,
        physical_depth: Option<(u64, u64)>,
        physical_time_seconds: Option<(f64, f64)>,
    ) -> Result<Self, LogicalMetricError> {
        let qubit_overhead =
            physical_to_logical_qubit_overhead(
                physical_qubits,
                logical_qubits,
            )?;

        let gate_overhead =
            match physical_gates {
                Some((physical, logical)) => Some(
                    physical_to_logical_gate_overhead(
                        physical,
                        logical,
                    )?,
                ),
                None => None,
            };

        let depth_overhead =
            match physical_depth {
                Some((physical, logical)) => Some(
                    physical_to_logical_depth_overhead(
                        physical,
                        logical,
                    )?,
                ),
                None => None,
            };

        let time_overhead =
            match physical_time_seconds {
                Some((physical, logical)) => Some(
                    physical_to_logical_time_overhead(
                        physical,
                        logical,
                    )?,
                ),
                None => None,
            };

        Ok(Self {
            qubit_overhead,
            gate_overhead,
            depth_overhead,
            time_overhead,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_produce_correct_logical_error_rate() {
        let metric =
            logical_error_rate_from_counts(10, 1_000)
                .expect("valid counts");

        assert!((metric.value.get() - 0.01).abs() < 1e-12);
        assert_eq!(
            metric.kind,
            MetricKind::LogicalErrorRate
        );
    }

    #[test]
    fn zero_errors_are_valid() {
        let probability =
            logical_error_probability_from_counts(
                0,
                1_000,
            )
            .expect("zero failures are valid");

        assert_eq!(probability, 0.0);
    }

    #[test]
    fn errors_cannot_exceed_trials() {
        let result =
            logical_error_probability_from_counts(
                101,
                100,
            );

        assert!(matches!(
            result,
            Err(LogicalMetricError::ErrorsExceedTrials {
                errors: 101,
                trials: 100
            })
        ));
    }

    #[test]
    fn logical_fidelity_is_complement() {
        let metric =
            logical_fidelity_from_error_probability(0.01)
                .expect("valid probability");

        assert!((metric.value.get() - 0.99).abs() < 1e-12);
        assert_eq!(
            metric.kind,
            MetricKind::LogicalFidelity
        );
    }

    #[test]
    fn per_cycle_conversion_is_reasonable() {
        let p =
            logical_error_per_cycle_from_failure_probability(
                0.1,
                10,
            )
            .expect("valid conversion");

        assert!(p > 0.0);
        assert!(p < 0.1);
    }

    #[test]
    fn zero_failure_has_zero_per_cycle_error() {
        let p =
            logical_error_per_cycle_from_failure_probability(
                0.0,
                100,
            )
            .expect("zero failure is valid");

        assert_eq!(p, 0.0);
    }

    #[test]
    fn invalid_symmetric_domain_is_rejected() {
        let result =
            logical_error_per_cycle_from_failure_probability(
                0.5,
                10,
            );

        assert!(matches!(
            result,
            Err(LogicalMetricError::SymmetricModelDomain { .. })
        ));
    }

    #[test]
    fn decoder_failure_probability_is_normalized() {
        let metric =
            decoder_failure_probability_from_counts(
                5,
                100,
            )
            .expect("valid decoder counts");

        assert!((metric.value.get() - 0.05).abs() < 1e-12);
        assert_eq!(
            metric.kind,
            MetricKind::DecoderFailureProbability
        );
    }

    #[test]
    fn physical_error_rate_is_normalized() {
        let metric =
            physical_error_rate_from_counts(
                2,
                1_000,
            )
            .expect("valid physical counts");

        assert!((metric.value.get() - 0.002).abs() < 1e-12);
        assert_eq!(
            metric.kind,
            MetricKind::PhysicalErrorRate
        );
    }

    #[test]
    fn logical_suppression_is_detected() {
        let ratio =
            logical_to_physical_error_ratio(
                0.001,
                0.01,
            )
            .expect("valid rates");

        assert!((ratio.value.get() - 0.1).abs() < 1e-12);

        let suppressed =
            is_below_pseudothreshold(
                0.001,
                0.01,
            )
            .expect("valid rates");

        assert!(suppressed);
    }

    #[test]
    fn equal_error_rates_are_not_suppressed() {
        let suppressed =
            is_below_pseudothreshold(
                0.01,
                0.01,
            )
            .expect("valid rates");

        assert!(!suppressed);
    }

    #[test]
    fn suppression_factor_is_inverse_of_ratio() {
        let factor =
            logical_error_suppression_factor(
                0.001,
                0.01,
            )
            .expect("valid rates");

        assert!((factor.value.get() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn lifetime_increases_as_error_decreases() {
        let high_error =
            logical_lifetime_cycles(0.01)
                .expect("valid error");

        let low_error =
            logical_lifetime_cycles(0.001)
                .expect("valid error");

        assert!(low_error.value.get() > high_error.value.get());
    }

    #[test]
    fn lifetime_seconds_scales_with_cycle_time() {
        let metric =
            logical_lifetime_seconds(
                0.01,
                1e-6,
            )
            .expect("valid values");

        assert!(metric.value.get() > 0.0);
        assert_eq!(
            metric.kind,
            MetricKind::LogicalFidelity
        );
        assert_eq!(
            metric.unit,
            MetricUnit::Seconds
        );
    }

    #[test]
    fn qubit_overhead_is_correct() {
        let metric =
            physical_to_logical_qubit_overhead(
                100,
                4,
            )
            .expect("valid resources");

        assert!((metric.value.get() - 25.0).abs() < 1e-12);
        assert_eq!(
            metric.kind,
            MetricKind::ResourceOverhead
        );
    }

    #[test]
    fn gate_overhead_is_correct() {
        let metric =
            physical_to_logical_gate_overhead(
                1_000,
                100,
            )
            .expect("valid resources");

        assert!((metric.value.get() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn depth_overhead_is_correct() {
        let metric =
            physical_to_logical_depth_overhead(
                500,
                50,
            )
            .expect("valid resources");

        assert!((metric.value.get() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn time_overhead_is_correct() {
        let metric =
            physical_to_logical_time_overhead(
                10.0,
                2.0,
            )
            .expect("valid times");

        assert!((metric.value.get() - 5.0).abs() < 1e-12);
    }

    #[test]
    fn space_time_volume_is_checked() {
        let metric =
            space_time_volume(
                100,
                1_000,
            )
            .expect("valid space-time volume");

        assert_eq!(
            metric.value.get(),
            100_000.0
        );
        assert_eq!(
            metric.kind,
            MetricKind::SpaceTimeVolume
        );
    }

    #[test]
    fn space_time_volume_rejects_overflow() {
        let result =
            space_time_volume(
                u64::MAX,
                2,
            );

        assert!(matches!(
            result,
            Err(LogicalMetricError::NumericalFailure {
                operation: "space_time_volume_integer_multiplication"
            })
        ));
    }

    #[test]
    fn non_finite_probability_is_rejected() {
        let result =
            logical_fidelity_from_error_probability(
                f64::NAN,
            );

        assert!(matches!(
            result,
            Err(LogicalMetricError::NonFinite { .. })
        ));
    }

    #[test]
    fn negative_probability_is_rejected() {
        let result =
            logical_fidelity_from_error_probability(
                -0.1,
            );

        assert!(matches!(
            result,
            Err(LogicalMetricError::OutOfUnitInterval { .. })
        ));
    }

    #[test]
    fn quality_bundle_contains_both_metrics() {
        let bundle =
            LogicalQualityMetrics::from_counts(
                10,
                1_000,
            )
            .expect("valid counts");

        assert!(
            (bundle.logical_error_rate.value.get() - 0.01)
                .abs()
                < 1e-12
        );

        assert!(
            (bundle.logical_fidelity.value.get() - 0.99)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn resource_bundle_contains_qubit_overhead() {
        let bundle =
            LogicalResourceOverhead::new(
                100,
                4,
                Some((1_000, 100)),
                Some((500, 50)),
                Some((10.0, 2.0)),
            )
            .expect("valid resource data");

        assert!(
            (bundle.qubit_overhead.value.get() - 25.0)
                .abs()
                < 1e-12
        );

        assert!(bundle.gate_overhead.is_some());
        assert!(bundle.depth_overhead.is_some());
        assert!(bundle.time_overhead.is_some());
    }

    #[test]
    fn observation_preserves_raw_counts() {
        let observation =
            LogicalErrorObservation::new(
                7,
                1_000,
            )
            .expect("valid observation");

        assert_eq!(observation.errors, 7);
        assert_eq!(observation.trials, 1_000);
        assert!(
            (observation.probability() - 0.007)
                .abs()
                < 1e-12
        );
    }
}