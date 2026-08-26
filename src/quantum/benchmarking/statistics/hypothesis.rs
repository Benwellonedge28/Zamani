//! Zamani Quantum Benchmarking — Hypothesis Testing
//!
//! Production statistical hypothesis-testing primitives used by the quantum
//! benchmarking subsystem.
//!
//! # Architectural role
//!
//! This module is intentionally independent of:
//!
//! - quantum IR
//! - quantum hardware
//! - benchmark protocols
//! - benchmark result storage
//! - report generation
//! - concrete metric implementations
//!
//! It provides deterministic mathematical/statistical operations which can
//! later be consumed by:
//!
//! - randomized benchmarking
//! - interleaved randomized benchmarking
//! - simultaneous randomized benchmarking
//! - cycle benchmarking
//! - Quantum Volume
//! - XEB
//! - application benchmarking
//! - QEC threshold experiments
//! - regression detection
//! - backend comparisons
//! - calibration comparisons
//!
//! # Design principles
//!
//! 1. No global state.
//! 2. No random numbers.
//! 3. No logging or printing.
//! 4. No floating-point `NaN`/infinity is accepted as valid input.
//! 5. Statistical assumptions are represented explicitly.
//! 6. Results contain the test statistic, p-value, effect size, decision,
//!    sample information, and configuration.
//! 7. A statistically significant result is not automatically interpreted as
//!    practically significant.
//! 8. The caller chooses the alternative hypothesis explicitly.
//! 9. The caller chooses the significance level explicitly.
//! 10. No test silently changes a requested one-sided test into a two-sided
//!     test or vice versa.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021.
//!
//! This module intentionally uses only the Rust standard library.
//!
//! # Statistical scope
//!
//! The module currently provides:
//!
//! - one-sample proportion z-tests
//! - exact one-sample binomial tests
//! - two-sample proportion z-tests
//! - one-sample mean z-tests when the population standard deviation is known
//! - two-sample mean z-tests when population standard deviations are known
//! - normal CDF / survival probability
//! - normal quantiles
//! - common-effect confidence intervals
//!
//! These primitives are deliberately small and composable. More specialized
//! tests should be implemented as separate statistical modules rather than
//! turning this file into a protocol-specific implementation.
//!
//! # Important scientific rule
//!
//! Hypothesis testing does not establish physical causality. For example,
//! rejecting "two backend error rates are equal" does not prove why they
//! differ. Causal attribution belongs to the analysis/diagnosis layer.

use std::fmt;

// =============================================================================
// Public constants
// =============================================================================

/// Conventional 5% significance level.
pub const DEFAULT_SIGNIFICANCE_LEVEL: f64 = 0.05;

/// Conventional 95% confidence level.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Maximum sample count for an exact binomial test.
///
/// Exact binomial testing is O(n) in the number of possible successes. A
/// bounded implementation prevents an accidental enormous exact calculation
/// from monopolizing a benchmark process.
///
/// Large experiments should normally use the asymptotic proportion test or a
/// dedicated statistical implementation.
pub const MAX_EXACT_BINOMIAL_TRIALS: usize = 1_000_000;

/// Numerical tolerance used when validating probabilities and proportions.
const PROBABILITY_EPSILON: f64 = 1.0e-12;

/// Numerical tolerance used when comparing p-values to significance levels.
const DECISION_EPSILON: f64 = 1.0e-15;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by hypothesis-testing operations.
#[derive(Debug, Clone, PartialEq)]
pub enum HypothesisTestError {
    /// A probability/proportion was not finite or was outside [0, 1].
    InvalidProbability {
        name: &'static str,
        value: f64,
    },

    /// A significance level was not strictly between 0 and 1.
    InvalidSignificanceLevel {
        value: f64,
    },

    /// A confidence level was not strictly between 0 and 1.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// A trial/sample count was zero.
    EmptySample {
        name: &'static str,
    },

    /// A number of successes exceeded the number of trials.
    SuccessesExceedTrials {
        successes: usize,
        trials: usize,
    },

    /// A sample standard deviation was invalid.
    InvalidStandardDeviation {
        name: &'static str,
        value: f64,
    },

    /// A known population standard deviation was zero where a z-test needs
    /// a positive standard error.
    ZeroStandardError,

    /// The requested exact calculation would exceed the bounded workload.
    ExactCalculationTooLarge {
        trials: usize,
        maximum: usize,
    },

    /// A numerical calculation produced a non-finite result.
    NumericalFailure {
        operation: &'static str,
    },
}

impl fmt::Display for HypothesisTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProbability { name, value } => write!(
                f,
                "{} must be finite and within [0, 1], got {}",
                name, value
            ),

            Self::InvalidSignificanceLevel { value } => write!(
                f,
                "significance level must be finite and strictly between 0 and 1, got {}",
                value
            ),

            Self::InvalidConfidenceLevel { value } => write!(
                f,
                "confidence level must be finite and strictly between 0 and 1, got {}",
                value
            ),

            Self::EmptySample { name } => {
                write!(f, "{} must contain at least one observation", name)
            }

            Self::SuccessesExceedTrials {
                successes,
                trials,
            } => write!(
                f,
                "success count {} cannot exceed trial count {}",
                successes, trials
            ),

            Self::InvalidStandardDeviation { name, value } => write!(
                f,
                "{} must be finite and strictly greater than zero, got {}",
                name, value
            ),

            Self::ZeroStandardError => {
                write!(f, "standard error must be strictly greater than zero")
            }

            Self::ExactCalculationTooLarge { trials, maximum } => write!(
                f,
                "exact binomial calculation with {} trials exceeds maximum {}",
                trials, maximum
            ),

            Self::NumericalFailure { operation } => {
                write!(f, "numerical failure while performing {}", operation)
            }
        }
    }
}

impl std::error::Error for HypothesisTestError {}

// =============================================================================
// Alternative hypothesis
// =============================================================================

/// Direction of an alternative hypothesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alternative {
    /// H₁: parameter < null value.
    Less,

    /// H₁: parameter != null value.
    TwoSided,

    /// H₁: parameter > null value.
    Greater,
}

impl Alternative {
    /// Returns the number of tails used by the corresponding test.
    pub const fn tails(self) -> usize {
        match self {
            Self::Less | Self::Greater => 1,
            Self::TwoSided => 2,
        }
    }
}

// =============================================================================
// Decision
// =============================================================================

/// Statistical decision produced by a hypothesis test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HypothesisDecision {
    /// Reject H₀ at the requested significance level.
    RejectNull,

    /// Do not reject H₀ at the requested significance level.
    FailToRejectNull,
}

impl HypothesisDecision {
    /// Returns true when the null hypothesis was rejected.
    pub const fn rejects_null(self) -> bool {
        matches!(self, Self::RejectNull)
    }

    /// Returns true when the null hypothesis was not rejected.
    pub const fn fails_to_reject_null(self) -> bool {
        matches!(self, Self::FailToRejectNull)
    }
}

// =============================================================================
// P-value method
// =============================================================================

/// Statistical method used to calculate the p-value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PValueMethod {
    /// Normal approximation to a proportion test.
    NormalApproximation,

    /// Exact binomial calculation.
    ExactBinomial,

    /// Normal z-test with known population standard deviation.
    KnownVarianceZTest,
}

// =============================================================================
// Test statistic
// =============================================================================

/// Test statistic produced by a hypothesis test.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestStatistic {
    /// Standardized statistic.
    pub value: f64,

    /// Name of the statistic.
    pub name: &'static str,
}

impl TestStatistic {
    fn new(value: f64, name: &'static str) -> Result<Self, HypothesisTestError> {
        if !value.is_finite() {
            return Err(HypothesisTestError::NumericalFailure {
                operation: "test statistic calculation",
            });
        }

        Ok(Self { value, name })
    }
}

// =============================================================================
// Hypothesis-test result
// =============================================================================

/// Complete result of a hypothesis test.
///
/// This is intentionally self-contained so that a result can be serialized or
/// transformed into the future benchmarking `Metric` / `BenchmarkResult`
/// structures without rerunning the statistical calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HypothesisTestResult {
    /// Statistical test used.
    pub method: PValueMethod,

    /// Direction of H₁.
    pub alternative: Alternative,

    /// Requested significance level α.
    pub significance_level: f64,

    /// Test statistic.
    pub statistic: TestStatistic,

    /// p-value.
    pub p_value: f64,

    /// Decision at the requested α.
    pub decision: HypothesisDecision,

    /// Estimated effect relative to the null/reference value.
    pub effect: f64,

    /// Optional standardized effect.
    ///
    /// For proportion tests this is the z statistic. For mean z-tests it is
    /// also the z statistic. It is kept separately so consumers do not have
    /// to infer semantics from `statistic.name`.
    pub standardized_effect: f64,

    /// Number of observations used by the test.
    pub observations: usize,

    /// Null/reference value.
    pub null_value: f64,
}

impl HypothesisTestResult {
    /// Returns true if the p-value is below the requested significance level.
    pub const fn is_statistically_significant(&self) -> bool {
        self.decision.rejects_null()
    }

    /// Returns the p-value in a bounded, safe form.
    ///
    /// Valid hypothesis-test results always contain a p-value in [0, 1].
    pub fn p_value(&self) -> f64 {
        self.p_value
    }
}

// =============================================================================
// Proportion test helpers
// =============================================================================

/// Validates a probability/proportion.
fn validate_probability(
    name: &'static str,
    value: f64,
) -> Result<(), HypothesisTestError> {
    if !value.is_finite()
        || value < -PROBABILITY_EPSILON
        || value > 1.0 + PROBABILITY_EPSILON
    {
        return Err(HypothesisTestError::InvalidProbability { name, value });
    }

    Ok(())
}

/// Clamps a value which has only accumulated tiny floating-point boundary
/// error.
fn clamp_probability(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Validates a significance level.
fn validate_significance_level(
    significance_level: f64,
) -> Result<(), HypothesisTestError> {
    if !significance_level.is_finite()
        || significance_level <= 0.0
        || significance_level >= 1.0
    {
        return Err(HypothesisTestError::InvalidSignificanceLevel {
            value: significance_level,
        });
    }

    Ok(())
}

/// Validates a confidence level.
fn validate_confidence_level(
    confidence_level: f64,
) -> Result<(), HypothesisTestError> {
    if !confidence_level.is_finite()
        || confidence_level <= 0.0
        || confidence_level >= 1.0
    {
        return Err(HypothesisTestError::InvalidConfidenceLevel {
            value: confidence_level,
        });
    }

    Ok(())
}

/// Validates a trial count.
fn validate_trials(
    name: &'static str,
    trials: usize,
) -> Result<(), HypothesisTestError> {
    if trials == 0 {
        return Err(HypothesisTestError::EmptySample { name });
    }

    Ok(())
}

/// Validates successes/trials.
fn validate_successes(
    successes: usize,
    trials: usize,
) -> Result<(), HypothesisTestError> {
    validate_trials("trials", trials)?;

    if successes > trials {
        return Err(HypothesisTestError::SuccessesExceedTrials {
            successes,
            trials,
        });
    }

    Ok(())
}

/// Calculates the empirical proportion.
fn empirical_proportion(
    successes: usize,
    trials: usize,
) -> Result<f64, HypothesisTestError> {
    validate_successes(successes, trials)?;

    let proportion = successes as f64 / trials as f64;

    if !proportion.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "empirical proportion calculation",
        });
    }

    Ok(clamp_probability(proportion))
}

/// Calculates the standard error for a one-sample proportion under H₀.
fn one_proportion_null_standard_error(
    null_probability: f64,
    trials: usize,
) -> Result<f64, HypothesisTestError> {
    validate_probability("null_probability", null_probability)?;
    validate_trials("trials", trials)?;

    let n = trials as f64;
    let variance = null_probability * (1.0 - null_probability) / n;

    if variance <= 0.0 {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    let standard_error = variance.sqrt();

    if !standard_error.is_finite() || standard_error <= 0.0 {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    Ok(standard_error)
}

/// Converts a z statistic to a p-value for an alternative hypothesis.
fn normal_p_value(
    statistic: f64,
    alternative: Alternative,
) -> Result<f64, HypothesisTestError> {
    if !statistic.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "normal p-value calculation",
        });
    }

    let p = match alternative {
        Alternative::Less => normal_cdf(statistic),

        Alternative::Greater => normal_survival(statistic),

        Alternative::TwoSided => {
            2.0 * normal_survival(statistic.abs())
        }
    };

    if !p.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "normal p-value calculation",
        });
    }

    Ok(clamp_probability(p))
}

/// Converts a p-value and significance level into an explicit decision.
fn decision(
    p_value: f64,
    significance_level: f64,
) -> Result<HypothesisDecision, HypothesisTestError> {
    validate_significance_level(significance_level)?;

    if !p_value.is_finite() || !(0.0..=1.0).contains(&p_value) {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "hypothesis decision calculation",
        });
    }

    if p_value < significance_level
        || (p_value - significance_level).abs() <= DECISION_EPSILON
    {
        Ok(HypothesisDecision::RejectNull)
    } else {
        Ok(HypothesisDecision::FailToRejectNull)
    }
}

// =============================================================================
// One-sample proportion z-test
// =============================================================================

/// Performs a one-sample proportion z-test.
///
/// H₀: p = `null_probability`
///
/// H₁ is selected through `alternative`.
///
/// This is appropriate when the normal approximation to the binomial
/// distribution is justified. For small samples or probabilities near the
/// boundaries, use [`exact_binomial_test`].
///
/// # Example
///
/// A benchmark observed 730 successful executions in 1,000 shots and wants to
/// test whether the true success probability is greater than 2/3:
///
/// ```
/// # use zamani::quantum::benchmarking::statistics::hypothesis::*;
/// let result = one_proportion_z_test(
///     730,
///     1000,
///     2.0 / 3.0,
///     Alternative::Greater,
///     0.05,
/// ).unwrap();
///
/// assert!(result.p_value < 0.05);
/// ```
pub fn one_proportion_z_test(
    successes: usize,
    trials: usize,
    null_probability: f64,
    alternative: Alternative,
    significance_level: f64,
) -> Result<HypothesisTestResult, HypothesisTestError> {
    validate_successes(successes, trials)?;
    validate_probability("null_probability", null_probability)?;
    validate_significance_level(significance_level)?;

    let observed = empirical_proportion(successes, trials)?;
    let standard_error =
        one_proportion_null_standard_error(null_probability, trials)?;

    let statistic_value = (observed - null_probability) / standard_error;

    let statistic = TestStatistic::new(statistic_value, "z")?;

    let p_value = normal_p_value(statistic_value, alternative)?;

    let test_decision = decision(p_value, significance_level)?;

    Ok(HypothesisTestResult {
        method: PValueMethod::NormalApproximation,
        alternative,
        significance_level,
        statistic,
        p_value,
        decision: test_decision,
        effect: observed - null_probability,
        standardized_effect: statistic_value,
        observations: trials,
        null_value: null_probability,
    })
}

// =============================================================================
// Exact binomial test
// =============================================================================

/// Performs an exact one-sample binomial hypothesis test.
///
/// H₀: p = `null_probability`
///
/// H₁ is selected through `alternative`.
///
/// Unlike the normal-approximation proportion test, this calculates the
/// binomial probability directly. It is therefore appropriate for small
/// samples where asymptotic assumptions may be questionable.
///
/// The implementation is bounded by [`MAX_EXACT_BINOMIAL_TRIALS`] to prevent
/// accidental pathological workloads.
///
/// For the two-sided test, the p-value is the probability of observing
/// outcomes whose probability under H₀ is less than or equal to the observed
/// outcome's probability. This is the standard exact-discrete ordering rather
/// than simply doubling one tail.
pub fn exact_binomial_test(
    successes: usize,
    trials: usize,
    null_probability: f64,
    alternative: Alternative,
    significance_level: f64,
) -> Result<HypothesisTestResult, HypothesisTestError> {
    validate_successes(successes, trials)?;
    validate_probability("null_probability", null_probability)?;
    validate_significance_level(significance_level)?;

    if trials > MAX_EXACT_BINOMIAL_TRIALS {
        return Err(HypothesisTestError::ExactCalculationTooLarge {
            trials,
            maximum: MAX_EXACT_BINOMIAL_TRIALS,
        });
    }

    let observed = empirical_proportion(successes, trials)?;

    let observed_log_probability =
        binomial_log_probability(successes, trials, null_probability)?;

    let p_value = match alternative {
        Alternative::Less => {
            binomial_lower_tail(successes, trials, null_probability)?
        }

        Alternative::Greater => {
            binomial_upper_tail(successes, trials, null_probability)?
        }

        Alternative::TwoSided => {
            binomial_two_sided_p_value(
                successes,
                trials,
                null_probability,
                observed_log_probability,
            )?
        }
    };

    let p_value = clamp_probability(p_value);

    let test_decision = decision(p_value, significance_level)?;

    Ok(HypothesisTestResult {
        method: PValueMethod::ExactBinomial,
        alternative,
        significance_level,
        statistic: TestStatistic::new(
            observed,
            "observed_proportion",
        )?,
        p_value,
        decision: test_decision,
        effect: observed - null_probability,
        standardized_effect: observed - null_probability,
        observations: trials,
        null_value: null_probability,
    })
}

/// Calculates log(n choose k).
fn log_binomial_coefficient(
    n: usize,
    k: usize,
) -> Result<f64, HypothesisTestError> {
    if k > n {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "binomial coefficient calculation",
        });
    }

    let k = k.min(n - k);

    if k == 0 {
        return Ok(0.0);
    }

    let mut result = 0.0;

    for i in 1..=k {
        result += ((n - k + i) as f64).ln();
        result -= (i as f64).ln();
    }

    if !result.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "log binomial coefficient calculation",
        });
    }

    Ok(result)
}

/// Calculates log(P(X = k)) for X ~ Binomial(n, p).
fn binomial_log_probability(
    successes: usize,
    trials: usize,
    probability: f64,
) -> Result<f64, HypothesisTestError> {
    validate_successes(successes, trials)?;
    validate_probability("probability", probability)?;

    if probability == 0.0 {
        return Ok(if successes == 0 {
            0.0
        } else {
            f64::NEG_INFINITY
        });
    }

    if probability == 1.0 {
        return Ok(if successes == trials {
            0.0
        } else {
            f64::NEG_INFINITY
        });
    }

    let coefficient =
        log_binomial_coefficient(trials, successes)?;

    let successes_f = successes as f64;
    let failures_f = (trials - successes) as f64;

    let log_probability = coefficient
        + successes_f * probability.ln()
        + failures_f * (1.0 - probability).ln();

    if log_probability.is_nan() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "binomial log probability calculation",
        });
    }

    Ok(log_probability)
}

/// Calculates exp(log_probability), safely mapping extreme negative values to
/// zero.
fn probability_from_log(log_probability: f64) -> f64 {
    if log_probability == f64::NEG_INFINITY {
        0.0
    } else {
        log_probability.exp()
    }
}

/// Calculates the exact lower tail P(X <= k).
fn binomial_lower_tail(
    successes: usize,
    trials: usize,
    probability: f64,
) -> Result<f64, HypothesisTestError> {
    let mut sum = 0.0;

    for k in 0..=successes {
        let log_p = binomial_log_probability(
            k,
            trials,
            probability,
        )?;

        sum += probability_from_log(log_p);

        if !sum.is_finite() {
            return Err(HypothesisTestError::NumericalFailure {
                operation: "exact binomial lower-tail calculation",
            });
        }
    }

    Ok(clamp_probability(sum))
}

/// Calculates the exact upper tail P(X >= k).
fn binomial_upper_tail(
    successes: usize,
    trials: usize,
    probability: f64,
) -> Result<f64, HypothesisTestError> {
    let mut sum = 0.0;

    for k in successes..=trials {
        let log_p = binomial_log_probability(
            k,
            trials,
            probability,
        )?;

        sum += probability_from_log(log_p);

        if !sum.is_finite() {
            return Err(HypothesisTestError::NumericalFailure {
                operation: "exact binomial upper-tail calculation",
            });
        }
    }

    Ok(clamp_probability(sum))
}

/// Calculates the exact two-sided binomial p-value using probability ordering.
fn binomial_two_sided_p_value(
    observed_successes: usize,
    trials: usize,
    probability: f64,
    observed_log_probability: f64,
) -> Result<f64, HypothesisTestError> {
    let mut sum = 0.0;

    for k in 0..=trials {
        let log_p =
            binomial_log_probability(k, trials, probability)?;

        if log_p <= observed_log_probability + PROBABILITY_EPSILON {
            sum += probability_from_log(log_p);
        }

        if !sum.is_finite() {
            return Err(HypothesisTestError::NumericalFailure {
                operation: "exact binomial two-sided calculation",
            });
        }
    }

    // Keep the parameter explicitly used so that the ordering semantics remain
    // obvious to future maintainers.
    let _ = observed_successes;

    Ok(clamp_probability(sum))
}

// =============================================================================
// Two-sample proportion z-test
// =============================================================================

/// Performs a two-sample test for equality of proportions.
///
/// H₀: p₁ = p₂
///
/// H₁ is selected through `alternative`, where the parameter is interpreted as
/// p₁ - p₂.
///
/// This is appropriate for comparing benchmark success probabilities,
/// failure probabilities, readout assignment rates, and similar proportions
/// when the normal approximation is justified.
pub fn two_proportion_z_test(
    successes_a: usize,
    trials_a: usize,
    successes_b: usize,
    trials_b: usize,
    alternative: Alternative,
    significance_level: f64,
) -> Result<HypothesisTestResult, HypothesisTestError> {
    validate_successes(successes_a, trials_a)?;
    validate_successes(successes_b, trials_b)?;
    validate_significance_level(significance_level)?;

    let proportion_a =
        empirical_proportion(successes_a, trials_a)?;
    let proportion_b =
        empirical_proportion(successes_b, trials_b)?;

    let total_successes = successes_a
        .checked_add(successes_b)
        .ok_or(HypothesisTestError::NumericalFailure {
            operation: "pooled success count calculation",
        })?;

    let total_trials = trials_a
        .checked_add(trials_b)
        .ok_or(HypothesisTestError::NumericalFailure {
            operation: "pooled trial count calculation",
        })?;

    let pooled_probability =
        empirical_proportion(total_successes, total_trials)?;

    let variance = pooled_probability
        * (1.0 - pooled_probability)
        * (1.0 / trials_a as f64 + 1.0 / trials_b as f64);

    if variance <= 0.0 || !variance.is_finite() {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    let standard_error = variance.sqrt();

    if standard_error <= 0.0 || !standard_error.is_finite() {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    let effect = proportion_a - proportion_b;

    let statistic_value = effect / standard_error;

    let statistic = TestStatistic::new(statistic_value, "z")?;

    let p_value = normal_p_value(statistic_value, alternative)?;

    let test_decision = decision(p_value, significance_level)?;

    let observations = total_trials;

    Ok(HypothesisTestResult {
        method: PValueMethod::NormalApproximation,
        alternative,
        significance_level,
        statistic,
        p_value,
        decision: test_decision,
        effect,
        standardized_effect: statistic_value,
        observations,
        null_value: 0.0,
    })
}

// =============================================================================
// Mean z-tests
// =============================================================================

/// Validates a known population standard deviation.
fn validate_population_standard_deviation(
    name: &'static str,
    standard_deviation: f64,
) -> Result<(), HypothesisTestError> {
    if !standard_deviation.is_finite()
        || standard_deviation <= 0.0
    {
        return Err(HypothesisTestError::InvalidStandardDeviation {
            name,
            value: standard_deviation,
        });
    }

    Ok(())
}

/// Performs a one-sample z-test for a population mean with known population
/// standard deviation.
///
/// H₀: μ = `null_mean`
///
/// H₁ is selected through `alternative`.
///
/// This test intentionally requires a known population standard deviation.
/// For ordinary benchmark sample standard deviations, a Student's t-test is
/// more appropriate and should be implemented in a dedicated module rather
/// than silently treating the sample standard deviation as known.
pub fn one_mean_z_test(
    sample_mean: f64,
    population_standard_deviation: f64,
    observations: usize,
    null_mean: f64,
    alternative: Alternative,
    significance_level: f64,
) -> Result<HypothesisTestResult, HypothesisTestError> {
    if !sample_mean.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "sample mean validation",
        });
    }

    if !null_mean.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "null mean validation",
        });
    }

    validate_population_standard_deviation(
        "population_standard_deviation",
        population_standard_deviation,
    )?;

    validate_trials("observations", observations)?;
    validate_significance_level(significance_level)?;

    let standard_error =
        population_standard_deviation / (observations as f64).sqrt();

    if !standard_error.is_finite() || standard_error <= 0.0 {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    let effect = sample_mean - null_mean;
    let statistic_value = effect / standard_error;

    let statistic = TestStatistic::new(statistic_value, "z")?;

    let p_value = normal_p_value(statistic_value, alternative)?;

    let test_decision = decision(p_value, significance_level)?;

    Ok(HypothesisTestResult {
        method: PValueMethod::KnownVarianceZTest,
        alternative,
        significance_level,
        statistic,
        p_value,
        decision: test_decision,
        effect,
        standardized_effect: statistic_value,
        observations,
        null_value: null_mean,
    })
}

/// Performs a two-sample z-test for the difference between means with known
/// population standard deviations.
///
/// H₀: μ₁ - μ₂ = 0
///
/// H₁ is selected through `alternative`.
///
/// The effect is always reported as:
///
///     mean_a - mean_b
pub fn two_mean_z_test(
    mean_a: f64,
    standard_deviation_a: f64,
    observations_a: usize,
    mean_b: f64,
    standard_deviation_b: f64,
    observations_b: usize,
    alternative: Alternative,
    significance_level: f64,
) -> Result<HypothesisTestResult, HypothesisTestError> {
    if !mean_a.is_finite() || !mean_b.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "sample mean validation",
        });
    }

    validate_population_standard_deviation(
        "standard_deviation_a",
        standard_deviation_a,
    )?;

    validate_population_standard_deviation(
        "standard_deviation_b",
        standard_deviation_b,
    )?;

    validate_trials("observations_a", observations_a)?;
    validate_trials("observations_b", observations_b)?;
    validate_significance_level(significance_level)?;

    let variance_a =
        standard_deviation_a * standard_deviation_a
            / observations_a as f64;

    let variance_b =
        standard_deviation_b * standard_deviation_b
            / observations_b as f64;

    let standard_error = (variance_a + variance_b).sqrt();

    if !standard_error.is_finite() || standard_error <= 0.0 {
        return Err(HypothesisTestError::ZeroStandardError);
    }

    let effect = mean_a - mean_b;
    let statistic_value = effect / standard_error;

    let statistic = TestStatistic::new(statistic_value, "z")?;

    let p_value = normal_p_value(statistic_value, alternative)?;

    let test_decision = decision(p_value, significance_level)?;

    let observations = observations_a
        .checked_add(observations_b)
        .ok_or(HypothesisTestError::NumericalFailure {
            operation: "combined observation count calculation",
        })?;

    Ok(HypothesisTestResult {
        method: PValueMethod::KnownVarianceZTest,
        alternative,
        significance_level,
        statistic,
        p_value,
        decision: test_decision,
        effect,
        standardized_effect: statistic_value,
        observations,
        null_value: 0.0,
    })
}

// =============================================================================
// Normal distribution
// =============================================================================

/// Standard normal probability density function.
pub fn normal_pdf(x: f64) -> f64 {
    if !x.is_finite() {
        return 0.0;
    }

    const INV_SQRT_2PI: f64 =
        0.398_942_280_401_432_677_94;

    INV_SQRT_2PI * (-0.5 * x * x).exp()
}

/// Standard normal cumulative distribution function.
///
/// This uses a high-quality rational approximation and requires no external
/// numerical dependency.
pub fn normal_cdf(x: f64) -> f64 {
    if x == f64::NEG_INFINITY {
        return 0.0;
    }

    if x == f64::INFINITY {
        return 1.0;
    }

    if !x.is_finite() {
        return f64::NAN;
    }

    // Abramowitz-Stegun-style approximation with symmetry.
    //
    // Maximum absolute error is sufficiently small for benchmark hypothesis
    // decisions at conventional significance levels.
    const P: f64 = 0.231_641_9;

    const B1: f64 = 0.319_381_530;
    const B2: f64 = -0.356_563_782;
    const B3: f64 = 1.781_477_937;
    const B4: f64 = -1.821_255_978;
    const B5: f64 = 1.330_274_429;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let ax = x.abs();

    let t = 1.0 / (1.0 + P * ax);

    let polynomial =
        ((((B5 * t + B4) * t + B3) * t + B2) * t + B1)
            * t;

    let tail = normal_pdf(ax) * polynomial;

    let result = if sign > 0.0 {
        1.0 - tail
    } else {
        tail
    };

    clamp_probability(result)
}

/// Standard normal upper-tail probability.
///
/// Returns P(Z >= x).
pub fn normal_survival(x: f64) -> f64 {
    if x == f64::NEG_INFINITY {
        return 1.0;
    }

    if x == f64::INFINITY {
        return 0.0;
    }

    if !x.is_finite() {
        return f64::NAN;
    }

    clamp_probability(1.0 - normal_cdf(x))
}

/// Inverse standard-normal CDF.
///
/// Returns z such that:
///
///     P(Z <= z) = p
///
/// Uses the Acklam rational approximation.
pub fn inverse_normal_cdf(
    p: f64,
) -> Result<f64, HypothesisTestError> {
    validate_probability("p", p)?;

    if p == 0.0 {
        return Ok(f64::NEG_INFINITY);
    }

    if p == 1.0 {
        return Ok(f64::INFINITY);
    }

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

    let result;

    if p < LOW {
        let q = (-2.0 * p.ln()).sqrt();

        result = (((((C[0] * q + C[1]) * q + C[2]) * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q
                + D[3])
                * q
                + 1.0);
    } else if p <= HIGH {
        let q = p - 0.5;
        let r = q * q;

        result = (((((A[0] * r + A[1]) * r + A[2]) * r
            + A[3])
            * r
            + A[4])
            * r
            + A[5])
            * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r
                + B[3])
                * r
                + B[4])
                * r)
                + 1.0);
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();

        result = -(((((C[0] * q + C[1]) * q + C[2]) * q
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

    if !result.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "inverse normal CDF calculation",
        });
    }

    Ok(result)
}

// =============================================================================
// Confidence interval for a difference in proportions
// =============================================================================

/// A confidence interval for an effect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfidenceInterval {
    /// Point estimate of the effect.
    pub estimate: f64,

    /// Lower confidence bound.
    pub lower: f64,

    /// Upper confidence bound.
    pub upper: f64,

    /// Confidence level used.
    pub confidence_level: f64,
}

impl ConfidenceInterval {
    /// Returns true when zero lies outside the interval.
    pub fn excludes_zero(&self) -> bool {
        self.lower > 0.0 || self.upper < 0.0
    }

    /// Returns the interval width.
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }
}

/// Calculates a Wald confidence interval for the difference between two
/// independent proportions.
///
/// Effect:
///
///     p₁ - p₂
///
/// This method is deliberately named explicitly. It should not be confused
/// with Wilson/Newcombe intervals, which are generally preferable for
/// proportions near 0 or 1.
///
/// The function is useful for quick effect-size reporting alongside the
/// two-proportion hypothesis test. Protocols requiring high-quality interval
/// estimation should use a dedicated proportion confidence-interval module.
pub fn two_proportion_wald_confidence_interval(
    successes_a: usize,
    trials_a: usize,
    successes_b: usize,
    trials_b: usize,
    confidence_level: f64,
) -> Result<ConfidenceInterval, HypothesisTestError> {
    validate_successes(successes_a, trials_a)?;
    validate_successes(successes_b, trials_b)?;
    validate_confidence_level(confidence_level)?;

    let p_a =
        empirical_proportion(successes_a, trials_a)?;
    let p_b =
        empirical_proportion(successes_b, trials_b)?;

    let effect = p_a - p_b;

    let variance = p_a * (1.0 - p_a) / trials_a as f64
        + p_b * (1.0 - p_b) / trials_b as f64;

    if variance < 0.0 || !variance.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "two-proportion confidence interval variance",
        });
    }

    let standard_error = variance.sqrt();

    let alpha = 1.0 - confidence_level;
    let z = inverse_normal_cdf(1.0 - alpha / 2.0)?;

    let margin = z * standard_error;

    let lower = effect - margin;
    let upper = effect + margin;

    if !lower.is_finite() || !upper.is_finite() {
        return Err(HypothesisTestError::NumericalFailure {
            operation: "two-proportion confidence interval",
        });
    }

    Ok(ConfidenceInterval {
        estimate: effect,
        lower,
        upper,
        confidence_level,
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_cdf_zero_is_one_half() {
        let value = normal_cdf(0.0);

        assert!((value - 0.5).abs() < 1.0e-7);
    }

    #[test]
    fn normal_cdf_is_monotonic_at_common_points() {
        let a = normal_cdf(-1.0);
        let b = normal_cdf(0.0);
        let c = normal_cdf(1.0);

        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn normal_cdf_and_survival_sum_to_one() {
        for x in [-4.0, -1.0, 0.0, 1.0, 4.0] {
            let sum = normal_cdf(x) + normal_survival(x);

            assert!((sum - 1.0).abs() < 1.0e-7);
        }
    }

    #[test]
    fn inverse_normal_cdf_round_trips_common_values() {
        for p in [0.001, 0.025, 0.05, 0.5, 0.95, 0.975, 0.999] {
            let z = inverse_normal_cdf(p).unwrap();
            let recovered = normal_cdf(z);

            assert!(
                (recovered - p).abs() < 1.0e-5,
                "p={}, z={}, recovered={}",
                p,
                z,
                recovered
            );
        }
    }

    #[test]
    fn one_proportion_test_rejects_large_positive_difference() {
        let result = one_proportion_z_test(
            730,
            1000,
            2.0 / 3.0,
            Alternative::Greater,
            0.05,
        )
        .unwrap();

        assert!(result.p_value < 0.05);
        assert!(result.decision.rejects_null());
        assert!(result.effect > 0.0);
    }

    #[test]
    fn one_proportion_test_does_not_reject_matching_probability() {
        let result = one_proportion_z_test(
            500,
            1000,
            0.5,
            Alternative::TwoSided,
            0.05,
        )
        .unwrap();

        assert!(result.p_value > 0.05);
        assert!(result.decision.fails_to_reject_null());
    }

    #[test]
    fn exact_binomial_test_handles_zero_probability() {
        let result = exact_binomial_test(
            0,
            10,
            0.0,
            Alternative::TwoSided,
            0.05,
        )
        .unwrap();

        assert!((result.p_value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn exact_binomial_test_handles_unit_probability() {
        let result = exact_binomial_test(
            10,
            10,
            1.0,
            Alternative::TwoSided,
            0.05,
        )
        .unwrap();

        assert!((result.p_value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn exact_binomial_detects_strong_difference() {
        let result = exact_binomial_test(
            10,
            10,
            0.5,
            Alternative::Greater,
            0.05,
        )
        .unwrap();

        assert!(result.p_value < 0.05);
        assert!(result.decision.rejects_null());
    }

    #[test]
    fn exact_binomial_rejects_oversized_request() {
        let result = exact_binomial_test(
            1,
            MAX_EXACT_BINOMIAL_TRIALS + 1,
            0.5,
            Alternative::TwoSided,
            0.05,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::ExactCalculationTooLarge { .. })
        ));
    }

    #[test]
    fn two_proportion_test_detects_difference() {
        let result = two_proportion_z_test(
            900,
            1000,
            800,
            1000,
            Alternative::Greater,
            0.05,
        )
        .unwrap();

        assert!(result.effect > 0.0);
        assert!(result.p_value < 0.05);
        assert!(result.decision.rejects_null());
    }

    #[test]
    fn two_proportion_test_identical_samples_do_not_differ() {
        let result = two_proportion_z_test(
            800,
            1000,
            800,
            1000,
            Alternative::TwoSided,
            0.05,
        )
        .unwrap();

        assert!((result.effect).abs() < 1.0e-12);
        assert!(result.decision.fails_to_reject_null());
    }

    #[test]
    fn one_mean_z_test_detects_difference() {
        let result = one_mean_z_test(
            105.0,
            10.0,
            100,
            100.0,
            Alternative::Greater,
            0.05,
        )
        .unwrap();

        assert!(result.effect > 0.0);
        assert!(result.p_value < 0.05);
        assert!(result.decision.rejects_null());
    }

    #[test]
    fn two_mean_z_test_detects_difference() {
        let result = two_mean_z_test(
            110.0,
            10.0,
            100,
            100.0,
            10.0,
            100,
            Alternative::Greater,
            0.05,
        )
        .unwrap();

        assert!(result.effect > 0.0);
        assert!(result.p_value < 0.05);
        assert!(result.decision.rejects_null());
    }

    #[test]
    fn confidence_interval_contains_zero_for_equal_samples() {
        let interval =
            two_proportion_wald_confidence_interval(
                500,
                1000,
                500,
                1000,
                0.95,
            )
            .unwrap();

        assert!((interval.estimate).abs() < 1.0e-12);
        assert!(interval.lower <= 0.0);
        assert!(interval.upper >= 0.0);
        assert!(!interval.excludes_zero());
    }

    #[test]
    fn confidence_interval_excludes_zero_for_large_difference() {
        let interval =
            two_proportion_wald_confidence_interval(
                950,
                1000,
                500,
                1000,
                0.95,
            )
            .unwrap();

        assert!(interval.estimate > 0.0);
        assert!(interval.excludes_zero());
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let result = one_proportion_z_test(
            5,
            10,
            1.5,
            Alternative::Greater,
            0.05,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn invalid_significance_is_rejected() {
        let result = one_proportion_z_test(
            5,
            10,
            0.5,
            Alternative::Greater,
            1.0,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::InvalidSignificanceLevel { .. })
        ));
    }

    #[test]
    fn successes_cannot_exceed_trials() {
        let result = one_proportion_z_test(
            11,
            10,
            0.5,
            Alternative::Greater,
            0.05,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::SuccessesExceedTrials { .. })
        ));
    }

    #[test]
    fn zero_observations_are_rejected() {
        let result = one_mean_z_test(
            1.0,
            1.0,
            0,
            0.0,
            Alternative::Greater,
            0.05,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::EmptySample { .. })
        ));
    }

    #[test]
    fn zero_population_standard_deviation_is_rejected() {
        let result = one_mean_z_test(
            1.0,
            0.0,
            10,
            0.0,
            Alternative::Greater,
            0.05,
        );

        assert!(matches!(
            result,
            Err(HypothesisTestError::InvalidStandardDeviation { .. })
        ));
    }

    #[test]
    fn decision_is_explicit() {
        assert!(
            HypothesisDecision::RejectNull.rejects_null()
        );

        assert!(
            HypothesisDecision::FailToRejectNull
                .fails_to_reject_null()
        );
    }

    #[test]
    fn alternative_tail_counts_are_correct() {
        assert_eq!(Alternative::Less.tails(), 1);
        assert_eq!(Alternative::Greater.tails(), 1);
        assert_eq!(Alternative::TwoSided.tails(), 2);
    }
}