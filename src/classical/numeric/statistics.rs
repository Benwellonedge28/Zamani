
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical descriptive and inferential statistics primitives.
//!
//! File: src/classical/numeric/statistics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide concrete, checked statistical operations over finite canonical
//! `ClassicalFloat` values. Algorithms operate on slices, so they can be used
//! with `ClassicalVector`, arrays, matrix rows, and other existing containers
//! without creating another storage abstraction.
//!
//! # Authority and ownership
//!
//! Canonical numeric representation:
//!   `crate::quantum::ir::classical::float::ClassicalFloat`
//!
//! Existing collection ownership:
//!   `crate::classical::collections`
//!
//! Arithmetic syntax:
//!   `grammar/expressions/arithmetic.g4`
//!   `grammar/expressions/expressions.g4`
//!
//! This file owns the concrete statistical algorithms and their errors.
//! It does not own numeric representation, grammar, AST, type inference,
//! symbolic statistics, canonical IR, execution planning, or distributed
//! aggregation.
//!
//! # Numerical contract
//!
//! - Inputs and successful outputs are finite `ClassicalFloat` values.
//! - Mean and variance use online Welford-style accumulation.
//! - Population variance divides by N; sample variance divides by N - 1.
//! - Sample variance and sample covariance require at least two observations.
//! - Quantiles use linear interpolation with the R-7 convention.
//! - Correlation is undefined for a constant input series and returns an
//!   explicit error rather than inventing a value.
//! - Overflow or another failed canonical arithmetic operation is reported.
//! - No implicit numeric conversion, rounding mode, or precision promotion
//!   is introduced.
//!
//! # Scalability contract
//!
//! No fixed observation count, hardware count, vector length, or data-size
//! ceiling is defined. Algorithms consume the supplied slice and are bounded
//! by its representable length, computation time, and available resources.
//! Operations that allocate request capacity fallibly.
//!
//! This module does not promise infinite physical memory or time. Distributed
//! and out-of-core statistics require execution-layer aggregation and resource
//! planning; they must not be simulated by silently truncating the input.
//!
//! # Integration contract
//!
//! 1. Register `pub mod statistics;` in `src/classical/numeric/mod.rs`.
//! 2. Expose `numeric` from `src/classical/mod.rs` while preserving existing
//!    module declarations.
//! 3. Semantic analysis validates operand types and maps concrete operations
//!    to these functions.
//! 4. Symbolic expressions stay symbolic in their existing semantic/IR owner.
//! 5. Distributed aggregation, target selection, and parallel scheduling stay
//!    in their designated execution layers.
//!
//! # Freeze rule
//!
//! Changes to quantile convention, sample/population definitions, error
//! behavior, or public signatures require compatibility review and tests.

use core::fmt;

use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
};

/// Errors produced by concrete statistical operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatisticsError {
    /// An operation requires at least one observation.
    EmptyInput,

    /// An operation requires more observations than were supplied.
    InsufficientSamples {
        /// Minimum observation count required.
        required: usize,
        /// Number of observations supplied.
        actual: usize,
    },

    /// The requested quantile probability is outside the closed interval
    /// [0, 1], or is not a finite number.
    InvalidQuantile,

    /// Paired observations have different lengths.
    LengthMismatch {
        /// Number of values in the first input.
        left: usize,
        /// Number of values in the second input.
        right: usize,
    },

    /// Correlation is undefined because at least one series has zero
    /// variance.
    UndefinedCorrelation,

    /// Checked arithmetic failed or produced a non-finite result.
    Numeric(ClassicalFloatError),

    /// Required working storage could not be reserved.
    AllocationFailed {
        /// Number of elements requested.
        elements: usize,
    },
}

impl fmt::Display for StatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => {
                f.write_str("statistical operation requires non-empty input")
            }
            Self::InsufficientSamples { required, actual } => write!(
                f,
                "insufficient observations: required {required}, received {actual}"
            ),
            Self::InvalidQuantile => {
                f.write_str("quantile probability must be finite and in [0, 1]")
            }
            Self::LengthMismatch { left, right } => write!(
                f,
                "paired input lengths differ: left has {left}, right has {right}"
            ),
            Self::UndefinedCorrelation => {
                f.write_str("correlation is undefined for a constant series")
            }
            Self::Numeric(error) => {
                write!(f, "statistical numeric operation failed: {error}")
            }
            Self::AllocationFailed { elements } => write!(
                f,
                "unable to reserve statistical working storage for {elements} elements"
            ),
        }
    }
}

impl std::error::Error for StatisticsError {}

impl From<ClassicalFloatError> for StatisticsError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Numeric(error)
    }
}

/// Result type for statistical operations.
pub type StatisticsResult<T> = Result<T, StatisticsError>;

/// Summary of a finite, non-empty observation series.
///
/// `sample_variance` and `sample_standard_deviation` are `None` when fewer
/// than two observations are supplied. Population statistics remain defined
/// for a single observation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticsSummary {
    /// Number of observations.
    pub count: usize,

    /// Checked sum of observations.
    pub sum: ClassicalFloat,

    /// Arithmetic mean.
    pub mean: ClassicalFloat,

    /// Smallest observation.
    pub minimum: ClassicalFloat,

    /// Largest observation.
    pub maximum: ClassicalFloat,

    /// Population variance, using divisor N.
    pub population_variance: ClassicalFloat,

    /// Sample variance, using divisor N - 1 when count >= 2.
    pub sample_variance: Option<ClassicalFloat>,

    /// Population standard deviation.
    pub population_standard_deviation: ClassicalFloat,

    /// Sample standard deviation when count >= 2.
    pub sample_standard_deviation: Option<ClassicalFloat>,
}

/// Running state for stable one-pass univariate statistics.
#[derive(Debug, Clone, Copy)]
struct RunningMoments {
    count: usize,
    mean: ClassicalFloat,
    sum_squared_deviations: ClassicalFloat,
    minimum: ClassicalFloat,
    maximum: ClassicalFloat,
}

impl RunningMoments {
    fn new(first: ClassicalFloat) -> Self {
        Self {
            count: 1,
            mean: first,
            sum_squared_deviations: ClassicalFloat::zero(),
            minimum: first,
            maximum: first,
        }
    }

    fn push(&mut self, value: ClassicalFloat) -> StatisticsResult<()> {
        let next_count = self
            .count
            .checked_add(1)
            .ok_or(StatisticsError::InsufficientSamples {
                required: usize::MAX,
                actual: self.count,
            })?;

        let delta = value.checked_sub(self.mean)?;
        let count = ClassicalFloat::new(next_count as f64)?;
        let adjustment = delta.checked_div(count)?;
        let next_mean = self.mean.checked_add(adjustment)?;
        let delta_after_update = value.checked_sub(next_mean)?;
        let increment = delta.checked_mul(delta_after_update)?;

        self.sum_squared_deviations = self
            .sum_squared_deviations
            .checked_add(increment)?;
        self.mean = next_mean;
        self.count = next_count;
        self.minimum = self.minimum.min(value);
        self.maximum = self.maximum.max(value);

        Ok(())
    }

    fn population_variance(self) -> StatisticsResult<ClassicalFloat> {
        self.sum_squared_deviations
            .checked_div(ClassicalFloat::new(self.count as f64)?)
            .map_err(StatisticsError::from)
    }

    fn sample_variance(self) -> StatisticsResult<ClassicalFloat> {
        if self.count < 2 {
            return Err(StatisticsError::InsufficientSamples {
                required: 2,
                actual: self.count,
            });
        }

        self.sum_squared_deviations
            .checked_div(ClassicalFloat::new((self.count - 1) as f64)?)
            .map_err(StatisticsError::from)
    }
}

fn moments(values: &[ClassicalFloat]) -> StatisticsResult<RunningMoments> {
    let (&first, rest) = values.split_first().ok_or(StatisticsError::EmptyInput)?;
    let mut state = RunningMoments::new(first);

    for value in rest.iter().copied() {
        state.push(value)?;
    }

    Ok(state)
}

fn require_pair(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> StatisticsResult<()> {
    if left.len() != right.len() {
        return Err(StatisticsError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    if left.is_empty() {
        return Err(StatisticsError::EmptyInput);
    }

    Ok(())
}

/// Computes the checked sum of all observations.
///
/// A finite result is required. If the sum overflows the canonical floating
/// representation, this function returns an error rather than infinity.
pub fn sum(values: &[ClassicalFloat]) -> StatisticsResult<ClassicalFloat> {
    if values.is_empty() {
        return Err(StatisticsError::EmptyInput);
    }

    let mut total = ClassicalFloat::zero();

    for value in values.iter().copied() {
        total = total.checked_add(value)?;
    }

    Ok(total)
}

/// Computes the arithmetic mean using online accumulation.
///
/// This avoids requiring a potentially overflowing total merely to calculate
/// the mean. The result is still subject to the canonical float's range and
/// rounding behavior.
pub fn mean(values: &[ClassicalFloat]) -> StatisticsResult<ClassicalFloat> {
    Ok(moments(values)?.mean)
}

/// Returns the minimum and maximum observations.
pub fn min_max(
    values: &[ClassicalFloat],
) -> StatisticsResult<(ClassicalFloat, ClassicalFloat)> {
    let state = moments(values)?;
    Ok((state.minimum, state.maximum))
}

/// Computes population variance with divisor N.
pub fn population_variance(
    values: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    moments(values)?.population_variance()
}

/// Computes sample variance with divisor N - 1.
///
/// At least two observations are required.
pub fn sample_variance(
    values: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    moments(values)?.sample_variance()
}

/// Computes population standard deviation.
pub fn population_standard_deviation(
    values: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    Ok(population_variance(values)?.sqrt()?)
}

/// Computes sample standard deviation.
///
/// At least two observations are required.
pub fn sample_standard_deviation(
    values: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    Ok(sample_variance(values)?.sqrt()?)
}

/// Computes a summary in one pass over the observations.
///
/// The sum is also calculated with checked arithmetic. Consequently, the
/// summary can fail when the sum is not representable even if the mean and
/// variance individually would be representable.
pub fn summarize(
    values: &[ClassicalFloat],
) -> StatisticsResult<StatisticsSummary> {
    let state = moments(values)?;
    let total = sum(values)?;
    let population_variance = state.population_variance()?;
    let population_standard_deviation = population_variance.sqrt()?;

    let (sample_variance, sample_standard_deviation) = if state.count >= 2 {
        let variance = state.sample_variance()?;
        (Some(variance), Some(variance.sqrt()?))
    } else {
        (None, None)
    };

    Ok(StatisticsSummary {
        count: state.count,
        sum: total,
        mean: state.mean,
        minimum: state.minimum,
        maximum: state.maximum,
        population_variance,
        sample_variance,
        population_standard_deviation,
        sample_standard_deviation,
    })
}

/// Computes a quantile using linear interpolation and the R-7 convention.
///
/// `probability` must be finite and in [0, 1]. The input is copied and sorted,
/// so this operation allocates O(N) working storage and takes O(N log N) time.
/// The input is never modified.
pub fn quantile(
    values: &[ClassicalFloat],
    probability: f64,
) -> StatisticsResult<ClassicalFloat> {
    if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
        return Err(StatisticsError::InvalidQuantile);
    }

    if values.is_empty() {
        return Err(StatisticsError::EmptyInput);
    }

    if values.len() == 1 {
        return Ok(values[0]);
    }

    let mut sorted = Vec::new();
    sorted
        .try_reserve_exact(values.len())
        .map_err(|_| StatisticsError::AllocationFailed {
            elements: values.len(),
        })?;
    sorted.extend_from_slice(values);

    // ClassicalFloat guarantees finite values, so total_cmp gives a stable,
    // deterministic numeric ordering, including a defined order for signed
    // zero.
    sorted.sort_unstable_by(|left, right| left.get().total_cmp(&right.get()));

    let position = probability * (sorted.len() - 1) as f64;
    let lower_index = position.floor() as usize;
    let upper_index = position.ceil() as usize;

    if lower_index == upper_index {
        return Ok(sorted[lower_index]);
    }

    let fraction = ClassicalFloat::new(position - lower_index as f64)?;
    let lower = sorted[lower_index];
    let upper = sorted[upper_index];
    let difference = upper.checked_sub(lower)?;
    let interpolated = lower.checked_add(difference.checked_mul(fraction)?)?;

    Ok(interpolated)
}

/// Computes the median using the R-7 quantile convention.
pub fn median(values: &[ClassicalFloat]) -> StatisticsResult<ClassicalFloat> {
    quantile(values, 0.5)
}

/// Computes population covariance for paired observations.
///
/// Both inputs must have the same non-zero length. The online update avoids
/// first forming potentially large sums of raw observations.
pub fn population_covariance(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    Ok(paired_moments(left, right)?.co_moment.checked_div(
        ClassicalFloat::new(left.len() as f64)?,
    )?)
}

/// Computes sample covariance for paired observations.
///
/// At least two paired observations are required.
pub fn sample_covariance(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    require_pair(left, right)?;

    if left.len() < 2 {
        return Err(StatisticsError::InsufficientSamples {
            required: 2,
            actual: left.len(),
        });
    }

    Ok(paired_moments(left, right)?.co_moment.checked_div(
        ClassicalFloat::new((left.len() - 1) as f64)?,
    )?)
}

#[derive(Debug, Clone, Copy)]
struct PairedMoments {
    mean_left: ClassicalFloat,
    mean_right: ClassicalFloat,
    left_m2: ClassicalFloat,
    right_m2: ClassicalFloat,
    co_moment: ClassicalFloat,
}

fn paired_moments(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> StatisticsResult<PairedMoments> {
    require_pair(left, right)?;

    let mut count = 0_usize;
    let mut mean_left = ClassicalFloat::zero();
    let mut mean_right = ClassicalFloat::zero();
    let mut left_m2 = ClassicalFloat::zero();
    let mut right_m2 = ClassicalFloat::zero();
    let mut co_moment = ClassicalFloat::zero();

    for (x, y) in left.iter().copied().zip(right.iter().copied()) {
        count = count
            .checked_add(1)
            .ok_or(StatisticsError::InsufficientSamples {
                required: usize::MAX,
                actual: count,
            })?;

        let divisor = ClassicalFloat::new(count as f64)?;
        let delta_left = x.checked_sub(mean_left)?;
        let delta_right = y.checked_sub(mean_right)?;

        mean_left = mean_left.checked_add(delta_left.checked_div(divisor)?)?;
        mean_right = mean_right.checked_add(delta_right.checked_div(divisor)?)?;

        let adjusted_left = x.checked_sub(mean_left)?;
        let adjusted_right = y.checked_sub(mean_right)?;

        left_m2 = left_m2.checked_add(delta_left.checked_mul(adjusted_left)?)?;
        right_m2 = right_m2.checked_add(delta_right.checked_mul(adjusted_right)?)?;
        co_moment =
            co_moment.checked_add(delta_left.checked_mul(adjusted_right)?)?;
    }

    Ok(PairedMoments {
        mean_left,
        mean_right,
        left_m2,
        right_m2,
        co_moment,
    })
}

/// Computes Pearson's correlation coefficient for paired observations.
///
/// The result is mathematically in [-1, 1], subject to floating-point
/// rounding. A constant series has zero variance, so correlation is undefined
/// and returns `UndefinedCorrelation`.
pub fn pearson_correlation(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> StatisticsResult<ClassicalFloat> {
    let state = paired_moments(left, right)?;

    if state.left_m2.is_zero() || state.right_m2.is_zero() {
        return Err(StatisticsError::UndefinedCorrelation);
    }

    let denominator = state.left_m2.sqrt()?.checked_mul(state.right_m2.sqrt()?)?;
    let coefficient = state.co_moment.checked_div(denominator)?;

    // Round-off may produce a value infinitesimally outside the mathematical
    // interval. Clamp only this final coefficient to its defined range.
    Ok(coefficient.max(ClassicalFloat::negative_one()).min(ClassicalFloat::one()))
}

#[cfg(test)]
mod tests {
    use super::{
        mean,
        median,
        min_max,
        pearson_correlation,
        population_covariance,
        population_standard_deviation,
        population_variance,
        quantile,
        sample_covariance,
        sample_standard_deviation,
        sample_variance,
        sum,
        summarize,
        StatisticsError,
    };
    use crate::quantum::ir::classical::float::ClassicalFloat;

    fn f(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value).expect("test value must be finite")
    }

    fn floats(values: &[f64]) -> Vec<ClassicalFloat> {
        values.iter().copied().map(f).collect()
    }

    fn close(actual: ClassicalFloat, expected: f64) {
        let difference = (actual.get() - expected).abs();
        assert!(
            difference <= 1.0e-10,
            "expected {expected}, got {}",
            actual.get()
        );
    }

    #[test]
    fn sum_and_mean_are_correct() {
        let values = floats(&[1.0, 2.0, 3.0, 4.0]);

        close(sum(&values).expect("sum"), 10.0);
        close(mean(&values).expect("mean"), 2.5);
    }

    #[test]
    fn min_max_are_correct() {
        let values = floats(&[8.0, -3.0, 2.0, 10.0, 4.0]);

        let (minimum, maximum) = min_max(&values).expect("min max");

        close(minimum, -3.0);
        close(maximum, 10.0);
    }

    #[test]
    fn population_and_sample_variance_are_distinct() {
        let values = floats(&[1.0, 2.0, 3.0, 4.0]);

        close(population_variance(&values).expect("population variance"), 1.25);
        close(sample_variance(&values).expect("sample variance"), 5.0 / 3.0);
    }

    #[test]
    fn standard_deviations_are_correct() {
        let values = floats(&[1.0, 2.0, 3.0, 4.0]);

        close(
            population_standard_deviation(&values).expect("population deviation"),
            1.25_f64.sqrt(),
        );
        close(
            sample_standard_deviation(&values).expect("sample deviation"),
            (5.0_f64 / 3.0).sqrt(),
        );
    }

    #[test]
    fn median_and_quantile_use_r7_interpolation() {
        let values = floats(&[1.0, 2.0, 3.0, 4.0]);

        close(median(&values).expect("median"), 2.5);
        close(quantile(&values, 0.0).expect("minimum quantile"), 1.0);
        close(quantile(&values, 1.0).expect("maximum quantile"), 4.0);
        close(quantile(&values, 1.0 / 3.0).expect("quantile"), 2.0);
    }

    #[test]
    fn quantile_does_not_modify_input() {
        let values = floats(&[4.0, 1.0, 3.0, 2.0]);
        let original = values;

        let _ = quantile(&values, 0.5).expect("median");

        assert_eq!(values, original);
    }

    #[test]
    fn empty_input_is_rejected() {
        let values: Vec<ClassicalFloat> = Vec::new();

        assert_eq!(mean(&values), Err(StatisticsError::EmptyInput));
        assert_eq!(median(&values), Err(StatisticsError::EmptyInput));
        assert_eq!(sum(&values), Err(StatisticsError::EmptyInput));
    }

    #[test]
    fn invalid_quantiles_are_rejected() {
        let values = floats(&[1.0, 2.0]);

        assert_eq!(
            quantile(&values, -0.1),
            Err(StatisticsError::InvalidQuantile)
        );
        assert_eq!(
            quantile(&values, 1.1),
            Err(StatisticsError::InvalidQuantile)
        );
        assert_eq!(
            quantile(&values, f64::NAN),
            Err(StatisticsError::InvalidQuantile)
        );
    }

    #[test]
    fn sample_statistics_require_two_observations() {
        let values = floats(&[5.0]);

        assert!(matches!(
            sample_variance(&values),
            Err(StatisticsError::InsufficientSamples {
                required: 2,
                actual: 1
            })
        ));

        assert!(matches!(
            sample_standard_deviation(&values),
            Err(StatisticsError::InsufficientSamples {
                required: 2,
                actual: 1
            })
        ));
    }

    #[test]
    fn summary_marks_sample_statistics_unavailable_for_one_value() {
        let result = summarize(&floats(&[5.0])).expect("summary");

        assert_eq!(result.count, 1);
        close(result.mean, 5.0);
        close(result.population_variance, 0.0);
        assert!(result.sample_variance.is_none());
        assert!(result.sample_standard_deviation.is_none());
    }

    #[test]
    fn paired_covariance_is_correct() {
        let left = floats(&[1.0, 2.0, 3.0]);
        let right = floats(&[2.0, 4.0, 6.0]);

        close(population_covariance(&left, &right).expect("population covariance"), 4.0 / 3.0);
        close(sample_covariance(&left, &right).expect("sample covariance"), 2.0);
    }

    #[test]
    fn perfect_linear_relationship_has_unit_correlation() {
        let left = floats(&[1.0, 2.0, 3.0, 4.0]);
        let right = floats(&[3.0, 5.0, 7.0, 9.0]);

        close(pearson_correlation(&left, &right).expect("correlation"), 1.0);
    }

    #[test]
    fn negative_linear_relationship_has_negative_unit_correlation() {
        let left = floats(&[1.0, 2.0, 3.0]);
        let right = floats(&[6.0, 4.0, 2.0]);

        close(pearson_correlation(&left, &right).expect("correlation"), -1.0);
    }

    #[test]
    fn correlation_rejects_constant_series() {
        let left = floats(&[1.0, 1.0, 1.0]);
        let right = floats(&[1.0, 2.0, 3.0]);

        assert_eq!(
            pearson_correlation(&left, &right),
            Err(StatisticsError::UndefinedCorrelation)
        );
    }

    #[test]
    fn paired_length_mismatch_is_rejected() {
        let left = floats(&[1.0, 2.0]);
        let right = floats(&[1.0]);

        assert!(matches!(
            population_covariance(&left, &right),
            Err(StatisticsError::LengthMismatch { left: 2, right: 1 })
        ));
    }

    #[test]
    fn overflow_in_sum_is_reported() {
        let values = floats(&[f64::MAX, f64::MAX]);

        assert!(matches!(sum(&values), Err(StatisticsError::Numeric(_))));
    }

    #[test]
    fn online_mean_handles_large_same_sign_values_without_summing_first() {
        let values = floats(&[f64::MAX / 2.0, f64::MAX / 2.0]);

        close(mean(&values).expect("mean should remain representable"), f64::MAX / 2.0);
    }

    #[test]
    fn summary_reports_all_fields() {
        let result = summarize(&floats(&[1.0, 2.0, 3.0])).expect("summary");

        assert_eq!(result.count, 3);
        close(result.sum, 6.0);
        close(result.mean, 2.0);
        close(result.minimum, 1.0);
        close(result.maximum, 3.0);
        close(result.population_variance, 2.0 / 3.0);
        close(result.sample_variance.expect("sample variance"), 1.0);
    }
}
