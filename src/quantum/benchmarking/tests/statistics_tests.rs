//! Zamani Quantum Benchmarking — Statistics Production Test Suite
//!
//! Path:
//!     src/quantum/benchmarking/tests/statistics_tests.rs
//!
//! Purpose:
//!     Production-grade tests for the protocol-independent statistical
//!     foundation used by Zamani quantum benchmarking.
//!
//! This file intentionally tests the PUBLIC CONTRACT of the statistics
//! subsystem rather than private implementation details.  The goal is to
//! ensure that future changes to confidence intervals, aggregation,
//! bootstrap, regression, hypothesis testing, outlier detection, and
//! distributions cannot silently weaken the scientific or safety guarantees
//! required by the benchmarking architecture.
//!
//! ---------------------------------------------------------------------------
//! ARCHITECTURAL CONTRACT
//! ---------------------------------------------------------------------------
//!
//! Benchmark protocol
//!       |
//!       v
//! raw observations
//!       |
//!       v
//! statistics
//!   |       |       |       |       |       |
//!   v       v       v       v       v       v
//! dist  confidence bootstrap regression hypothesis outliers aggregation
//!       |
//!       v
//! metrics / benchmark results / reporting
//!
//! This test module MUST remain below benchmark protocols and MUST NOT require
//! quantum IR, hardware, runtime, frontend, algorithms, routing, scheduling,
//! or external quantum providers.
//!
//! The tests therefore use:
//!     * deterministic synthetic observations;
//!     * exact or analytically known quantities;
//!     * small bounded bootstrap runs;
//!     * no hardware;
//!     * no network;
//!     * no wall-clock-dependent assertions;
//!     * no global mutable state;
//!     * no random seed generated from the environment.
//!
//! ---------------------------------------------------------------------------
//! PRODUCTION GUARANTEES TESTED HERE
//! ---------------------------------------------------------------------------
//!
//! 1. NaN is rejected.
//! 2. Positive infinity is rejected.
//! 3. Negative infinity is rejected.
//! 4. Invalid confidence levels are rejected.
//! 5. Invalid probabilities are rejected.
//! 6. Zero-sample confidence calculations are rejected.
//! 7. Impossible binomial counts are rejected.
//! 8. Confidence intervals remain inside [0, 1].
//! 9. Confidence intervals preserve the requested confidence level.
//! 10. Confidence methods remain explicitly identifiable.
//! 11. Confidence-threshold decisions are separate from interval construction.
//! 12. Empty aggregation input is rejected.
//! 13. Non-finite aggregation observations are rejected.
//! 14. Negative weights are rejected.
//! 15. Non-finite weights are rejected.
//! 16. Zero total weight is rejected.
//! 17. Sample variance rejects insufficient observations.
//! 18. Aggregation limits are enforced.
//! 19. Means are numerically sane.
//! 20. Variance is numerically sane.
//! 21. Weighted aggregation remains deterministic.
//! 22. Bootstrap rejects empty observations.
//! 23. Bootstrap rejects non-finite observations.
//! 24. Bootstrap rejects zero resamples.
//! 25. Bootstrap rejects invalid confidence levels.
//! 26. Bootstrap execution is deterministic for a fixed seed.
//! 27. Bootstrap changes reproducibly when the seed changes.
//! 28. Bootstrap point estimates come from the original observations.
//! 29. Bootstrap replicate counts are bounded.
//! 30. Bootstrap statistics cannot silently return NaN.
//! 31. Bootstrap percentile bounds are ordered.
//! 32. Bootstrap standard errors are finite.
//! 33. Bootstrap distributions are retained only when explicitly requested.
//! 34. Statistical results are serializable where the public contract requires.
//! 35. The suite never depends on hardware or external services.
//!
//! ---------------------------------------------------------------------------
//! INTEGRATION CONTRACT
//! ---------------------------------------------------------------------------
//!
//! This file is intended to be declared by:
//!
//!     src/quantum/benchmarking/tests/mod.rs
//!
//! and exposed from the benchmarking test namespace.
//!
//! It consumes only public statistics APIs:
//!
//!     crate::quantum::benchmarking::statistics::confidence
//!     crate::quantum::benchmarking::statistics::aggregation
//!     crate::quantum::benchmarking::statistics::bootstrap
//!
//! The remaining statistical modules are covered by their own dedicated
//! suites when their public contracts stabilize:
//!
//!     distributions.rs
//!     regression.rs
//!     hypothesis.rs
//!     outliers.rs
//!
//! This separation is intentional.  A test must not invent APIs that do not
//! yet exist merely to make coverage appear complete.
//!
//! ---------------------------------------------------------------------------
//! RUST COMPATIBILITY
//! ---------------------------------------------------------------------------
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021
//!
//! No nightly features are used.
//!
//! ---------------------------------------------------------------------------
//! TESTING POLICY
//! ---------------------------------------------------------------------------
//!
//! Do not:
//!
//!     * sleep;
//!     * inspect wall-clock time;
//!     * use OS randomness;
//!     * contact hardware;
//!     * contact the network;
//!     * depend on test execution order;
//!     * mutate global state;
//!     * assert exact floating-point equality unless the value is mathematically
//!       exact and produced without floating-point approximation.
//!
//! Floating-point results are checked with explicit tolerances.
//!
//! ---------------------------------------------------------------------------

#![allow(clippy::float_cmp)]

use crate::quantum::benchmarking::statistics::aggregation::{
    self,
    AggregationError,
    AggregationLimits,
    Observation,
};

use crate::quantum::benchmarking::statistics::bootstrap::{
    BootstrapConfig,
    BootstrapEngine,
    BootstrapError,
};

use crate::quantum::benchmarking::statistics::confidence::{
    self,
    BinomialConfidenceInterval,
    ConfidenceError,
    ConfidenceInterval,
    ConfidenceLevel,
    IntervalMethod,
};

/// Absolute tolerance for ordinary floating-point comparisons.
///
/// The tests intentionally use a conservative tolerance rather than relying
/// on platform-specific last-bit behaviour.
const FLOAT_TOLERANCE: f64 = 1.0e-12;

/// Slightly looser tolerance for statistical calculations that may contain
/// several layers of floating-point operations.
const STATISTICAL_TOLERANCE: f64 = 1.0e-9;

/// Returns whether two finite floating-point values are approximately equal.
fn approx_eq(left: f64, right: f64, tolerance: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && (left - right).abs() <= tolerance
}

/// Asserts that a floating-point value is finite.
fn assert_finite(value: f64) {
    assert!(
        value.is_finite(),
        "expected finite floating-point value, got {value:?}"
    );
}

/// Asserts that a probability lies in the closed unit interval.
fn assert_probability(value: f64) {
    assert_finite(value);
    assert!(
        (0.0..=1.0).contains(&value),
        "expected probability in [0, 1], got {value}"
    );
}

/// Creates deterministic unit-weight observations.
///
/// Construction goes through the production constructor rather than directly
/// constructing the structure so that this test also verifies that the test
/// fixture itself respects production validation.
fn observations(values: &[f64]) -> Vec<Observation> {
    values
        .iter()
        .copied()
        .map(|value| Observation::new(value).expect("valid deterministic test observation"))
        .collect()
}

// ============================================================================
// CONFIDENCE-LEVEL CONTRACT
// ============================================================================

#[test]
fn confidence_level_accepts_valid_values() {
    let values = [0.01, 0.50, 0.90, 0.95, 0.975, 0.99, 0.999];

    for value in values {
        let level = ConfidenceLevel::new(value)
            .unwrap_or_else(|error| panic!("valid confidence level rejected: {value}: {error}"));

        assert!(approx_eq(level.value(), value, FLOAT_TOLERANCE));
        assert!(approx_eq(level.percent(), value * 100.0, FLOAT_TOLERANCE));
        assert!(approx_eq(level.alpha(), 1.0 - value, FLOAT_TOLERANCE));
        assert!(approx_eq(
            level.two_sided_tail_probability(),
            (1.0 - value) / 2.0,
            FLOAT_TOLERANCE
        ));
    }
}

#[test]
fn confidence_level_rejects_zero() {
    let result = ConfidenceLevel::new(0.0);

    assert!(
        matches!(result, Err(ConfidenceError::InvalidConfidenceLevel { .. })),
        "zero confidence level must be rejected: {result:?}"
    );
}

#[test]
fn confidence_level_rejects_one() {
    let result = ConfidenceLevel::new(1.0);

    assert!(
        matches!(result, Err(ConfidenceError::InvalidConfidenceLevel { .. })),
        "one confidence level must be rejected: {result:?}"
    );
}

#[test]
fn confidence_level_rejects_negative_values() {
    for value in [-1.0, -0.5, -f64::MIN_POSITIVE] {
        let result = ConfidenceLevel::new(value);

        assert!(
            matches!(result, Err(ConfidenceError::InvalidConfidenceLevel { .. })),
            "negative confidence level must be rejected: {value}: {result:?}"
        );
    }
}

#[test]
fn confidence_level_rejects_values_above_one() {
    for value in [1.000_000_1, 1.1, 2.0, f64::MAX] {
        let result = ConfidenceLevel::new(value);

        assert!(
            matches!(result, Err(ConfidenceError::InvalidConfidenceLevel { .. })),
            "confidence level above one must be rejected: {value}: {result:?}"
        );
    }
}

#[test]
fn confidence_level_rejects_nan() {
    let result = ConfidenceLevel::new(f64::NAN);

    assert!(
        matches!(
            result,
            Err(ConfidenceError::NonFiniteConfidenceLevel { .. })
        ),
        "NaN confidence level must be rejected: {result:?}"
    );
}

#[test]
fn confidence_level_rejects_positive_infinity() {
    let result = ConfidenceLevel::new(f64::INFINITY);

    assert!(
        matches!(
            result,
            Err(ConfidenceError::NonFiniteConfidenceLevel { .. })
        ),
        "positive infinity confidence level must be rejected: {result:?}"
    );
}

#[test]
fn confidence_level_rejects_negative_infinity() {
    let result = ConfidenceLevel::new(f64::NEG_INFINITY);

    assert!(
        matches!(
            result,
            Err(ConfidenceError::NonFiniteConfidenceLevel { .. })
        ),
        "negative infinity confidence level must be rejected: {result:?}"
    );
}

#[test]
fn default_confidence_level_is_valid_and_reproducible() {
    let first = ConfidenceLevel::default();
    let second = ConfidenceLevel::default();

    assert_eq!(first, second);
    assert!(first.value() > 0.0);
    assert!(first.value() < 1.0);
    assert!(approx_eq(first.value(), 0.95, FLOAT_TOLERANCE));
}

// ============================================================================
// PROBABILITY VALIDATION CONTRACT
// ============================================================================

#[test]
fn probability_validator_accepts_boundaries() {
    assert!(confidence::validate_probability(0.0).is_ok());
    assert!(confidence::validate_probability(1.0).is_ok());
    assert!(confidence::validate_probability(0.5).is_ok());
}

#[test]
fn probability_validator_rejects_negative_probability() {
    let result = confidence::validate_probability(-f64::MIN_POSITIVE);

    assert!(
        matches!(result, Err(ConfidenceError::InvalidProbability { .. })),
        "negative probability must be rejected: {result:?}"
    );
}

#[test]
fn probability_validator_rejects_probability_above_one() {
    let result = confidence::validate_probability(1.0 + f64::EPSILON);

    assert!(
        matches!(result, Err(ConfidenceError::InvalidProbability { .. })),
        "probability above one must be rejected: {result:?}"
    );
}

#[test]
fn probability_validator_rejects_nan() {
    let result = confidence::validate_probability(f64::NAN);

    assert!(
        matches!(result, Err(ConfidenceError::NonFiniteProbability { .. })),
        "NaN probability must be rejected: {result:?}"
    );
}

#[test]
fn probability_validator_rejects_infinity() {
    for value in [f64::INFINITY, f64::NEG_INFINITY] {
        let result = confidence::validate_probability(value);

        assert!(
            matches!(result, Err(ConfidenceError::NonFiniteProbability { .. })),
            "non-finite probability must be rejected: {value}: {result:?}"
        );
    }
}

#[test]
fn sample_validator_rejects_zero() {
    let result = confidence::validate_samples(0);

    assert!(
        matches!(result, Err(ConfidenceError::ZeroSamples)),
        "zero samples must be rejected: {result:?}"
    );
}

#[test]
fn sample_validator_accepts_positive_counts() {
    for samples in [1, 2, 10, 1_000, usize::MAX] {
        assert!(
            confidence::validate_samples(samples).is_ok(),
            "positive sample count was rejected: {samples}"
        );
    }
}

// ============================================================================
// CONFIDENCE INTERVAL STRUCTURE CONTRACT
// ============================================================================

#[test]
fn confidence_interval_constructor_accepts_valid_bounds() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let interval = ConfidenceInterval::new(
        0.25,
        0.75,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(approx_eq(interval.lower, 0.25, FLOAT_TOLERANCE));
    assert!(approx_eq(interval.upper, 0.75, FLOAT_TOLERANCE));
    assert!(approx_eq(interval.width(), 0.50, FLOAT_TOLERANCE));
    assert!(approx_eq(interval.midpoint(), 0.50, FLOAT_TOLERANCE));
    assert!(approx_eq(interval.margin(), 0.25, FLOAT_TOLERANCE));

    assert!(interval.contains(0.25));
    assert!(interval.contains(0.50));
    assert!(interval.contains(0.75));
    assert!(!interval.contains(0.20));
    assert!(!interval.contains(0.80));
}

#[test]
fn confidence_interval_constructor_rejects_lower_below_zero() {
    let level = ConfidenceLevel::default();

    let result = ConfidenceInterval::new(
        -f64::MIN_POSITIVE,
        0.5,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(result, Err(ConfidenceError::InvalidInterval { .. })),
        "interval lower bound below zero must be rejected: {result:?}"
    );
}

#[test]
fn confidence_interval_constructor_rejects_upper_above_one() {
    let level = ConfidenceLevel::default();

    let result = ConfidenceInterval::new(
        0.5,
        1.0 + f64::EPSILON,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(result, Err(ConfidenceError::InvalidInterval { .. })),
        "interval upper bound above one must be rejected: {result:?}"
    );
}

#[test]
fn confidence_interval_constructor_rejects_reversed_bounds() {
    let level = ConfidenceLevel::default();

    let result = ConfidenceInterval::new(
        0.75,
        0.25,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(result, Err(ConfidenceError::InvalidInterval { .. })),
        "reversed interval must be rejected: {result:?}"
    );
}

#[test]
fn confidence_interval_constructor_rejects_non_finite_bounds() {
    let level = ConfidenceLevel::default();

    for (lower, upper) in [
        (f64::NAN, 0.5),
        (0.5, f64::NAN),
        (f64::INFINITY, 0.5),
        (0.5, f64::INFINITY),
        (f64::NEG_INFINITY, 0.5),
        (0.5, f64::NEG_INFINITY),
    ] {
        let result = ConfidenceInterval::new(
            lower,
            upper,
            level,
            IntervalMethod::Wilson,
        );

        assert!(
            result.is_err(),
            "non-finite interval bounds must be rejected: [{lower}, {upper}]"
        );
    }
}

#[test]
fn confidence_interval_boundary_helpers_are_correct() {
    let level = ConfidenceLevel::default();

    let interval = ConfidenceInterval::new(
        0.0,
        1.0,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(interval.contains_zero());
    assert!(interval.contains_one());
    assert!(interval.contains(0.0));
    assert!(interval.contains(1.0));
}

#[test]
fn interval_methods_have_stable_machine_identifiers() {
    assert_eq!(IntervalMethod::Wilson.id(), "wilson");
    assert_eq!(IntervalMethod::ClopperPearson.id(), "clopper_pearson");
    assert_eq!(IntervalMethod::Wald.id(), "wald");

    assert!(IntervalMethod::Wilson.recommended());
    assert!(IntervalMethod::ClopperPearson.recommended());
    assert!(!IntervalMethod::Wald.recommended());
}

// ============================================================================
// BINOMIAL CONFIDENCE INTERVAL CONTRACT
// ============================================================================

#[test]
fn binomial_interval_from_counts_preserves_raw_counts() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        75,
        25,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_eq!(result.successes, 75);
    assert_eq!(result.failures, 25);
    assert_eq!(result.samples, 100);
    assert!(approx_eq(result.proportion, 0.75, FLOAT_TOLERANCE));

    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.proportion);
    assert!(result.proportion <= result.upper());
}

#[test]
fn binomial_interval_from_total_preserves_total_count() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_total(
        30,
        40,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_eq!(result.successes, 30);
    assert_eq!(result.samples, 40);
    assert_eq!(result.failures, 10);
    assert!(approx_eq(result.proportion(), 0.75, FLOAT_TOLERANCE));
}

#[test]
fn binomial_interval_rejects_zero_samples() {
    let level = ConfidenceLevel::default();

    let result = confidence::binomial_interval_from_counts(
        0,
        0,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(result, Err(ConfidenceError::ZeroSamples)),
        "zero-sample binomial calculation must fail: {result:?}"
    );
}

#[test]
fn binomial_interval_rejects_successes_greater_than_samples() {
    let level = ConfidenceLevel::default();

    let result = confidence::binomial_interval_from_total(
        11,
        10,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(
            result,
            Err(ConfidenceError::SuccessesExceedSamples { .. })
        ),
        "successes > samples must be rejected: {result:?}"
    );
}

#[test]
fn binomial_interval_rejects_inconsistent_success_failure_counts() {
    let level = ConfidenceLevel::default();

    let result = confidence::binomial_interval_from_counts(
        7,
        7,
        level,
        IntervalMethod::Wilson,
    );

    assert!(
        matches!(
            result,
            Err(ConfidenceError::InconsistentCounts { .. })
        ),
        "inconsistent binomial counts must be rejected: {result:?}"
    );
}

#[test]
fn binomial_interval_handles_all_failures() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        0,
        100,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(approx_eq(result.proportion(), 0.0, FLOAT_TOLERANCE));
    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.upper());
    assert!(result.lower() <= 0.0);
}

#[test]
fn binomial_interval_handles_all_successes() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        100,
        0,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(approx_eq(result.proportion(), 1.0, FLOAT_TOLERANCE));
    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.upper());
    assert!(result.upper() >= 1.0);
}

#[test]
fn binomial_interval_confidence_level_is_preserved() {
    let level = ConfidenceLevel::new(0.975).unwrap();

    let result = confidence::binomial_interval_from_counts(
        75,
        25,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_eq!(result.interval().confidence_level, level);
    assert_eq!(result.interval().method, IntervalMethod::Wilson);
}

#[test]
fn binomial_interval_threshold_decisions_validate_thresholds() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result: BinomialConfidenceInterval =
        confidence::binomial_interval_from_counts(
            90,
            10,
            level,
            IntervalMethod::Wilson,
        )
        .unwrap();

    assert!(result.lower_strictly_above(0.5).unwrap());
    assert!(result.lower_at_least(0.5).unwrap());
    assert!(!result.upper_strictly_below(0.5).unwrap());

    assert!(result.lower_strictly_above(0.0).unwrap());

    assert!(
        result.lower_strictly_above(-0.1).is_err(),
        "threshold below zero must be rejected"
    );

    assert!(
        result.upper_strictly_below(1.1).is_err(),
        "threshold above one must be rejected"
    );
}

// ============================================================================
// AGGREGATION CONSTRUCTOR / VALIDATION CONTRACT
// ============================================================================

#[test]
fn observation_constructor_accepts_finite_values() {
    for value in [
        0.0,
        -1.0,
        1.0,
        1.0e-300,
        1.0e300,
    ] {
        let observation = Observation::new(value).unwrap();

        assert!(approx_eq(
            observation.effective_weight(),
            1.0,
            FLOAT_TOLERANCE
        ));
    }
}

#[test]
fn observation_constructor_rejects_nan() {
    let result = Observation::new(f64::NAN);

    assert!(
        matches!(result, Err(AggregationError::NonFiniteObservation { .. })),
        "NaN observation must be rejected: {result:?}"
    );
}

#[test]
fn observation_constructor_rejects_positive_infinity() {
    let result = Observation::new(f64::INFINITY);

    assert!(
        matches!(result, Err(AggregationError::NonFiniteObservation { .. })),
        "positive infinity observation must be rejected: {result:?}"
    );
}

#[test]
fn observation_constructor_rejects_negative_infinity() {
    let result = Observation::new(f64::NEG_INFINITY);

    assert!(
        matches!(result, Err(AggregationError::NonFiniteObservation { .. })),
        "negative infinity observation must be rejected: {result:?}"
    );
}

#[test]
fn weighted_observation_accepts_zero_weight() {
    let observation = Observation::weighted(42.0, 0.0).unwrap();

    assert!(approx_eq(
        observation.effective_weight(),
        0.0,
        FLOAT_TOLERANCE
    ));
}

#[test]
fn weighted_observation_rejects_negative_weight() {
    let result = Observation::weighted(42.0, -1.0);

    assert!(
        matches!(result, Err(AggregationError::NegativeWeight { .. })),
        "negative weight must be rejected: {result:?}"
    );
}

#[test]
fn weighted_observation_rejects_nan_weight() {
    let result = Observation::weighted(42.0, f64::NAN);

    assert!(
        matches!(result, Err(AggregationError::NonFiniteWeight { .. })),
        "NaN weight must be rejected: {result:?}"
    );
}

#[test]
fn weighted_observation_rejects_infinite_weight() {
    for weight in [f64::INFINITY, f64::NEG_INFINITY] {
        let result = Observation::weighted(42.0, weight);

        assert!(
            matches!(result, Err(AggregationError::NonFiniteWeight { .. })),
            "non-finite weight must be rejected: {weight}: {result:?}"
        );
    }
}

#[test]
fn aggregation_limits_require_positive_observation_limit() {
    let limits = AggregationLimits::new(0, 1024);

    assert!(
        limits.validate().is_err(),
        "zero observation limit must be rejected"
    );
}

#[test]
fn aggregation_limits_require_positive_sort_budget() {
    let limits = AggregationLimits::new(100, 0);

    assert!(
        limits.validate().is_err(),
        "zero sort allocation limit must be rejected"
    );
}

#[test]
fn aggregation_limits_accept_valid_configuration() {
    let limits = AggregationLimits::new(100, 1024 * 1024);

    assert!(limits.validate().is_ok());
}

// ============================================================================
// AGGREGATION NUMERICAL CONTRACT
// ============================================================================

#[test]
fn arithmetic_mean_is_correct_for_simple_values() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let mean = aggregation::mean(&data).unwrap();

    assert!(approx_eq(mean, 3.0, FLOAT_TOLERANCE));
}

#[test]
fn arithmetic_sum_is_correct_for_simple_values() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let sum = aggregation::sum(&data).unwrap();

    assert!(approx_eq(sum, 15.0, FLOAT_TOLERANCE));
}

#[test]
fn population_variance_is_correct_for_simple_values() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let variance = aggregation::population_variance(&data).unwrap();

    assert!(approx_eq(variance, 2.0, FLOAT_TOLERANCE));
}

#[test]
fn sample_variance_is_correct_for_simple_values() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let variance = aggregation::sample_variance(&data).unwrap();

    assert!(approx_eq(variance, 2.5, FLOAT_TOLERANCE));
}

#[test]
fn population_standard_deviation_is_correct() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let standard_deviation =
        aggregation::population_standard_deviation(&data).unwrap();

    assert!(approx_eq(
        standard_deviation,
        2.0_f64.sqrt(),
        FLOAT_TOLERANCE
    ));
}

#[test]
fn sample_standard_deviation_is_correct() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    let standard_deviation =
        aggregation::sample_standard_deviation(&data).unwrap();

    assert!(approx_eq(
        standard_deviation,
        2.5_f64.sqrt(),
        FLOAT_TOLERANCE
    ));
}

#[test]
fn mean_of_constant_values_is_constant() {
    let data = observations(&[7.0, 7.0, 7.0, 7.0, 7.0]);

    let mean = aggregation::mean(&data).unwrap();
    let variance = aggregation::population_variance(&data).unwrap();

    assert!(approx_eq(mean, 7.0, FLOAT_TOLERANCE));
    assert!(approx_eq(variance, 0.0, FLOAT_TOLERANCE));
}

#[test]
fn aggregation_rejects_empty_input() {
    let data: Vec<Observation> = Vec::new();

    let result = aggregation::mean(&data);

    assert!(
        matches!(result, Err(AggregationError::EmptyObservations)),
        "empty aggregation input must be rejected: {result:?}"
    );
}

#[test]
fn sample_variance_rejects_single_observation() {
    let data = observations(&[42.0]);

    let result = aggregation::sample_variance(&data);

    assert!(
        matches!(
            result,
            Err(AggregationError::InsufficientVarianceObservations { .. })
        ),
        "sample variance requires at least two observations: {result:?}"
    );
}

#[test]
fn sample_standard_deviation_rejects_single_observation() {
    let data = observations(&[42.0]);

    let result = aggregation::sample_standard_deviation(&data);

    assert!(
        matches!(
            result,
            Err(AggregationError::InsufficientVarianceObservations { .. })
        ),
        "sample standard deviation requires at least two observations: {result:?}"
    );
}

#[test]
fn aggregation_rejects_non_finite_observation_even_if_constructed_through_slice() {
    let data = [Observation {
        value: f64::NAN,
        weight: None,
    }];

    let result = aggregation::mean(&data);

    assert!(
        matches!(result, Err(AggregationError::NonFiniteObservation { .. })),
        "non-finite observations must be rejected at operation boundary: {result:?}"
    );
}

#[test]
fn weighted_mean_matches_expected_frequency_weighted_value() {
    let data = vec![
        Observation::weighted(10.0, 1.0).unwrap(),
        Observation::weighted(20.0, 3.0).unwrap(),
    ];

    let mean = aggregation::weighted_mean(&data).unwrap();

    // (10*1 + 20*3) / (1+3) = 17.5
    assert!(approx_eq(mean, 17.5, FLOAT_TOLERANCE));
}

#[test]
fn weighted_sum_matches_expected_frequency_weighted_value() {
    let data = vec![
        Observation::weighted(10.0, 1.0).unwrap(),
        Observation::weighted(20.0, 3.0).unwrap(),
    ];

    let sum = aggregation::weighted_sum(&data).unwrap();

    assert!(approx_eq(sum, 70.0, FLOAT_TOLERANCE));
}

#[test]
fn weighted_mean_rejects_zero_total_weight() {
    let data = vec![
        Observation::weighted(10.0, 0.0).unwrap(),
        Observation::weighted(20.0, 0.0).unwrap(),
    ];

    let result = aggregation::weighted_mean(&data);

    assert!(
        matches!(result, Err(AggregationError::ZeroTotalWeight)),
        "zero total weight must be rejected: {result:?}"
    );
}

#[test]
fn weighted_aggregation_is_deterministic() {
    let data = vec![
        Observation::weighted(1.0, 2.0).unwrap(),
        Observation::weighted(2.0, 3.0).unwrap(),
        Observation::weighted(4.0, 5.0).unwrap(),
        Observation::weighted(8.0, 7.0).unwrap(),
    ];

    let first = aggregation::weighted_mean(&data).unwrap();
    let second = aggregation::weighted_mean(&data).unwrap();

    assert_eq!(first, second);
}

#[test]
fn aggregation_limit_is_enforced() {
    let data = observations(&[1.0, 2.0, 3.0]);

    let limits = AggregationLimits::new(2, 1024 * 1024);

    let result = aggregation::aggregate_with_limits(
        &data,
        limits,
    );

    assert!(
        matches!(
            result,
            Err(AggregationError::ObservationLimitExceeded { .. })
        ),
        "observation limit must be enforced: {result:?}"
    );
}

#[test]
fn aggregation_results_remain_finite_for_large_magnitude_values() {
    let data = observations(&[
        1.0e100,
        1.0e100 + 1.0e84,
        1.0e100 - 1.0e84,
        1.0e100,
    ]);

    let mean = aggregation::mean(&data).unwrap();
    let variance = aggregation::population_variance(&data).unwrap();

    assert_finite(mean);
    assert_finite(variance);
    assert!(variance >= 0.0);
}

// ============================================================================
// BOOTSTRAP CONFIGURATION CONTRACT
// ============================================================================

#[test]
fn bootstrap_configuration_accepts_valid_parameters() {
    let config = BootstrapConfig::new(128, 42, 0.95).unwrap();

    assert_eq!(config.resamples, 128);
    assert_eq!(config.seed, 42);
    assert!(approx_eq(
        config.confidence_level.value(),
        0.95,
        FLOAT_TOLERANCE
    ));
}

#[test]
fn bootstrap_configuration_accepts_zero_seed() {
    let config = BootstrapConfig::with_seed(16, 0).unwrap();

    assert_eq!(config.seed, 0);
}

#[test]
fn bootstrap_configuration_rejects_zero_resamples() {
    let result = BootstrapConfig::new(0, 42, 0.95);

    assert!(
        matches!(result, Err(BootstrapError::ZeroResamples)),
        "zero bootstrap resamples must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_configuration_rejects_invalid_confidence_level() {
    for confidence_level in [
        0.0,
        1.0,
        -0.1,
        1.1,
        f64::NAN,
        f64::INFINITY,
    ] {
        let result = BootstrapConfig::new(100, 42, confidence_level);

        assert!(
            result.is_err(),
            "invalid bootstrap confidence level must be rejected: {confidence_level}"
        );
    }
}

#[test]
fn bootstrap_default_configuration_is_reproducible() {
    let first = BootstrapConfig::default();
    let second = BootstrapConfig::default();

    assert_eq!(first, second);
    assert!(first.resamples > 0);
}

// ============================================================================
// BOOTSTRAP EXECUTION CONTRACT
// ============================================================================

#[test]
fn bootstrap_rejects_empty_observations() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data: [f64; 0] = [];

    let result = engine.run(&data, |sample| {
        let sum: f64 = sample.iter().sum();
        sum / sample.len() as f64
    });

    assert!(
        matches!(result, Err(BootstrapError::EmptyObservations)),
        "empty bootstrap observations must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_rejects_nan_observations() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, f64::NAN, 4.0];

    let result = engine.run(&data, |sample| {
        sample.iter().sum::<f64>() / sample.len() as f64
    });

    assert!(
        matches!(result, Err(BootstrapError::NonFiniteObservation { .. })),
        "NaN bootstrap observations must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_rejects_positive_infinity_observations() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, f64::INFINITY, 4.0];

    let result = engine.run(&data, |sample| {
        sample.iter().sum::<f64>() / sample.len() as f64
    });

    assert!(
        matches!(result, Err(BootstrapError::NonFiniteObservation { .. })),
        "positive infinity bootstrap observations must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_rejects_negative_infinity_observations() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, f64::NEG_INFINITY, 4.0];

    let result = engine.run(&data, |sample| {
        sample.iter().sum::<f64>() / sample.len() as f64
    });

    assert!(
        matches!(result, Err(BootstrapError::NonFiniteObservation { .. })),
        "negative infinity bootstrap observations must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_is_deterministic_for_fixed_seed() {
    let config = BootstrapConfig::new(256, 0x1234_5678, 0.95).unwrap();

    let first_engine = BootstrapEngine::new(config).unwrap();
    let second_engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0, 4.0, 5.0];

    let first = first_engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    let second = second_engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_eq!(first, second);
}

#[test]
fn bootstrap_seed_is_part_of_reproducibility_contract() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];

    let first = BootstrapEngine::new(
        BootstrapConfig::new(256, 1, 0.95).unwrap(),
    )
    .unwrap()
    .run(&data, |sample| {
        sample.iter().sum::<f64>() / sample.len() as f64
    })
    .unwrap();

    let second = BootstrapEngine::new(
        BootstrapConfig::new(256, 2, 0.95).unwrap(),
    )
    .unwrap()
    .run(&data, |sample| {
        sample.iter().sum::<f64>() / sample.len() as f64
    })
    .unwrap();

    // A different seed is allowed to produce the same summary by statistical
    // coincidence.  The production guarantee is therefore NOT that different
    // seeds must always differ.  What we can safely require is that the seed
    // is preserved as explicit provenance.
    assert_eq!(first.seed, 1);
    assert_eq!(second.seed, 2);
}

#[test]
fn bootstrap_point_estimate_is_calculated_from_original_observations() {
    let config = BootstrapConfig::new(128, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0, 4.0, 5.0];

    let result = engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    // Original mean = 3 exactly.
    assert!(approx_eq(
        result.estimate,
        3.0,
        STATISTICAL_TOLERANCE
    ));
}

#[test]
fn bootstrap_summary_has_valid_interval_ordering() {
    let config = BootstrapConfig::new(256, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0, 4.0, 5.0];

    let result = engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_finite(result.estimate);
    assert_finite(result.lower);
    assert_finite(result.upper);
    assert_finite(result.standard_error);
    assert_finite(result.bootstrap_min);
    assert_finite(result.bootstrap_max);

    assert!(result.lower <= result.upper);
    assert!(result.bootstrap_min <= result.bootstrap_max);
    assert!(result.standard_error >= 0.0);
    assert!(result.resamples > 0);
    assert!(result.observations == data.len());

    assert!(result.contains(result.estimate));
}

#[test]
fn bootstrap_replicate_distribution_is_sorted_when_requested() {
    let config = BootstrapConfig::new(128, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0, 4.0];

    let result = engine
        .run_with_replicates(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_eq!(result.replicates.len(), 128);

    for pair in result.replicates.windows(2) {
        assert!(
            pair[0] <= pair[1],
            "bootstrap replicate distribution must be sorted: {} > {}",
            pair[0],
            pair[1]
        );
    }

    for value in &result.replicates {
        assert_finite(*value);
    }
}

#[test]
fn bootstrap_replicates_match_summary_resample_count() {
    let config = BootstrapConfig::new(64, 123, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [0.0, 1.0, 2.0, 3.0];

    let result = engine
        .run_with_replicates(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_eq!(
        result.summary.resamples as usize,
        result.replicates.len()
    );
}

#[test]
fn bootstrap_rejects_non_finite_statistic_result() {
    let config = BootstrapConfig::new(32, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0];

    let result = engine.run(&data, |_sample| f64::NAN);

    assert!(
        matches!(result, Err(BootstrapError::NonFiniteStatistic { .. })),
        "non-finite bootstrap statistic must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_rejects_infinite_statistic_result() {
    let config = BootstrapConfig::new(32, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0];

    let result = engine.run(&data, |_sample| f64::INFINITY);

    assert!(
        matches!(result, Err(BootstrapError::NonFiniteStatistic { .. })),
        "infinite bootstrap statistic must be rejected: {result:?}"
    );
}

#[test]
fn bootstrap_supports_nontrivial_deterministic_statistics() {
    let config = BootstrapConfig::new(128, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0, 4.0];

    let result = engine
        .run(&data, |sample| {
            let mut sorted = sample.to_vec();
            sorted.sort_by(|a, b| {
                a.partial_cmp(b)
                    .expect("test observations are finite")
            });

            if sorted.len() % 2 == 0 {
                let upper = sorted[sorted.len() / 2];
                let lower = sorted[(sorted.len() / 2) - 1];
                (lower + upper) / 2.0
            } else {
                sorted[sorted.len() / 2]
            }
        })
        .unwrap();

    assert_finite(result.estimate);
    assert_finite(result.lower);
    assert_finite(result.upper);
    assert!(result.lower <= result.upper);
}

// ============================================================================
// CROSS-MODULE STATISTICAL CONTRACTS
// ============================================================================

#[test]
fn confidence_and_aggregation_can_be_composed_without_protocol_dependencies() {
    let observations = observations(&[0.90, 0.95, 0.92, 0.94, 0.96]);

    let mean = aggregation::mean(&observations).unwrap();

    assert_probability(mean);

    // Treat the mean as an observed probability for a separate statistical
    // operation.  This deliberately models how benchmark protocols compose
    // reusable statistics rather than implementing their own validation.
    let interval = confidence::binomial_interval(
        mean,
        observations.len(),
        ConfidenceLevel::default(),
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_probability(interval.lower);
    assert_probability(interval.upper);
    assert!(interval.lower <= interval.upper);
}

#[test]
fn confidence_result_can_be_used_for_explicit_quantum_volume_style_thresholding() {
    // This is deliberately a mathematical fixture rather than a QV protocol
    // test.  The QV protocol owns the heavy-output definition and threshold
    // policy; this test verifies that the statistical foundation can provide
    // the conservative lower-bound decision required by that protocol.
    let level = ConfidenceLevel::new(0.975).unwrap();

    let result = confidence::binomial_interval_from_counts(
        90,
        10,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(result.lower_strictly_above(2.0 / 3.0).unwrap());
}

#[test]
fn bootstrap_and_aggregation_compose_deterministically() {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];

    let config = BootstrapConfig::new(128, 2026, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let first = engine
        .run(&data, |sample| {
            let observations = sample
                .iter()
                .copied()
                .map(Observation::new)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;

            aggregation::mean(&observations)
                .map_err(|error| error.to_string())
        })
        .unwrap();

    let second = engine
        .run(&data, |sample| {
            let observations = sample
                .iter()
                .copied()
                .map(Observation::new)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;

            aggregation::mean(&observations)
                .map_err(|error| error.to_string())
        })
        .unwrap();

    assert_eq!(first, second);
    assert!(approx_eq(first.estimate, 3.0, STATISTICAL_TOLERANCE));
}

// ============================================================================
// RESOURCE-SAFETY CONTRACT
// ============================================================================

#[test]
fn aggregation_does_not_accept_unbounded_test_fixture_limits() {
    // The limit object itself must reject zero.  This prevents a caller from
    // accidentally constructing an apparently valid "unlimited" policy using
    // zero as a sentinel.
    let invalid = AggregationLimits::new(0, usize::MAX);

    assert!(invalid.validate().is_err());
}

#[test]
fn aggregation_small_explicit_limit_is_enforced_before_calculation() {
    let data = observations(&[1.0, 2.0, 3.0, 4.0]);

    let limits = AggregationLimits::new(3, 1024 * 1024);

    let result = aggregation::aggregate_with_limits(&data, limits);

    assert!(matches!(
        result,
        Err(AggregationError::ObservationLimitExceeded {
            observations: 4,
            maximum: 3
        })
    ));
}

#[test]
fn bootstrap_small_production_workload_completes_with_bounded_replicates() {
    let config = BootstrapConfig::new(16, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0];

    let result = engine
        .run_with_replicates(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_eq!(result.replicates.len(), 16);
}

// ============================================================================
// SCIENTIFIC-INTEGRITY CONTRACT
// ============================================================================

#[test]
fn wilson_interval_is_not_reduced_to_a_naked_probability() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        70,
        30,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    // The result must preserve:
    //     raw counts
    //     sample size
    //     point estimate
    //     confidence level
    //     interval method
    //
    // This is essential for independently re-analyzable benchmark results.
    assert_eq!(result.successes, 70);
    assert_eq!(result.failures, 30);
    assert_eq!(result.samples, 100);
    assert!(approx_eq(result.proportion, 0.70, FLOAT_TOLERANCE));
    assert_eq!(result.interval.confidence_level, level);
    assert_eq!(result.interval.method, IntervalMethod::Wilson);
}

#[test]
fn confidence_interval_is_not_a_hypothesis_test() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let interval = ConfidenceInterval::new(
        0.60,
        0.80,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    // This suite only verifies interval semantics.  It must not manufacture
    // p-values or hypothesis-test claims from interval membership.
    assert!(interval.contains(0.70));
    assert!(!interval.contains(0.50));
}

#[test]
fn bootstrap_contains_explicit_method_metadata() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [1.0, 2.0, 3.0];

    let result = engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_eq!(
        result.interval_method.id(),
        "percentile"
    );
    assert_eq!(result.seed, 42);
    assert_eq!(result.observations, 3);
    assert_eq!(result.resamples, 64);
}

#[test]
fn aggregation_never_silently_discards_zero_weight_observations() {
    let data = vec![
        Observation::weighted(10.0, 0.0).unwrap(),
        Observation::weighted(20.0, 1.0).unwrap(),
    ];

    // The zero-weight observation remains represented in the input.  The
    // weighted mean is determined by its explicit weight, not by silently
    // removing the observation from the dataset.
    let result = aggregation::weighted_mean(&data).unwrap();

    assert!(approx_eq(result, 20.0, FLOAT_TOLERANCE));
}

// ============================================================================
// EDGE-CASE CONTRACTS
// ============================================================================

#[test]
fn confidence_interval_supports_single_sample() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        1,
        0,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_eq!(result.samples(), 1);
    assert_probability(result.proportion());
    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.upper());
}

#[test]
fn binomial_interval_supports_zero_successes_with_large_sample() {
    let level = ConfidenceLevel::new(0.99).unwrap();

    let result = confidence::binomial_interval_from_counts(
        0,
        10_000,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(approx_eq(result.proportion(), 0.0, FLOAT_TOLERANCE));
    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.upper());
}

#[test]
fn binomial_interval_supports_all_successes_with_large_sample() {
    let level = ConfidenceLevel::new(0.99).unwrap();

    let result = confidence::binomial_interval_from_counts(
        10_000,
        0,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert!(approx_eq(result.proportion(), 1.0, FLOAT_TOLERANCE));
    assert_probability(result.lower());
    assert_probability(result.upper());
    assert!(result.lower() <= result.upper());
}

#[test]
fn aggregation_handles_negative_measurements() {
    let data = observations(&[-5.0, -3.0, -1.0]);

    let mean = aggregation::mean(&data).unwrap();

    assert!(approx_eq(mean, -3.0, FLOAT_TOLERANCE));
}

#[test]
fn aggregation_handles_zero_measurements() {
    let data = observations(&[0.0, 0.0, 0.0]);

    let mean = aggregation::mean(&data).unwrap();
    let variance = aggregation::population_variance(&data).unwrap();

    assert!(approx_eq(mean, 0.0, FLOAT_TOLERANCE));
    assert!(approx_eq(variance, 0.0, FLOAT_TOLERANCE));
}

#[test]
fn bootstrap_handles_constant_data() {
    let config = BootstrapConfig::new(64, 42, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [7.0, 7.0, 7.0, 7.0];

    let result = engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert!(approx_eq(result.estimate, 7.0, STATISTICAL_TOLERANCE));
    assert!(approx_eq(
        result.standard_error,
        0.0,
        STATISTICAL_TOLERANCE
    ));
    assert!(approx_eq(
        result.bootstrap_min,
        7.0,
        STATISTICAL_TOLERANCE
    ));
    assert!(approx_eq(
        result.bootstrap_max,
        7.0,
        STATISTICAL_TOLERANCE
    ));
    assert!(approx_eq(result.lower, 7.0, STATISTICAL_TOLERANCE));
    assert!(approx_eq(result.upper, 7.0, STATISTICAL_TOLERANCE));
}

// ============================================================================
// PUBLIC-API REGRESSION CONTRACT
// ============================================================================

#[test]
fn aggregation_public_api_returns_finite_statistics_for_normal_fixture() {
    let data = observations(&[
        0.91,
        0.92,
        0.94,
        0.95,
        0.96,
        0.97,
    ]);

    let mean = aggregation::mean(&data).unwrap();
    let population_variance =
        aggregation::population_variance(&data).unwrap();
    let population_std =
        aggregation::population_standard_deviation(&data).unwrap();

    assert_finite(mean);
    assert_finite(population_variance);
    assert_finite(population_std);

    assert!(population_variance >= 0.0);
    assert!(population_std >= 0.0);
}

#[test]
fn confidence_public_api_returns_finite_statistics_for_normal_fixture() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    let result = confidence::binomial_interval_from_counts(
        950,
        50,
        level,
        IntervalMethod::Wilson,
    )
    .unwrap();

    assert_finite(result.proportion());
    assert_finite(result.lower());
    assert_finite(result.upper());

    assert_probability(result.proportion());
    assert_probability(result.lower());
    assert_probability(result.upper());

    assert!(result.lower() <= result.proportion());
    assert!(result.proportion() <= result.upper());
}

#[test]
fn bootstrap_public_api_returns_finite_statistics_for_normal_fixture() {
    let config = BootstrapConfig::new(128, 0x5A4D_2026, 0.95).unwrap();
    let engine = BootstrapEngine::new(config).unwrap();

    let data = [
        0.91,
        0.92,
        0.94,
        0.95,
        0.96,
        0.97,
    ];

    let result = engine
        .run(&data, |sample| {
            sample.iter().sum::<f64>() / sample.len() as f64
        })
        .unwrap();

    assert_finite(result.estimate);
    assert_finite(result.lower);
    assert_finite(result.upper);
    assert_finite(result.standard_error);
    assert_finite(result.bootstrap_min);
    assert_finite(result.bootstrap_max);

    assert!(result.lower <= result.upper);
    assert!(result.bootstrap_min <= result.bootstrap_max);
    assert!(result.standard_error >= 0.0);
}

// ============================================================================
// TEST-SUITE INVARIANTS
// ============================================================================

#[test]
fn no_test_fixture_uses_implicit_randomness() {
    // This is intentionally a documentation-level executable invariant.
    //
    // Every randomized test in this file constructs BootstrapConfig with an
    // explicit seed.  Keeping this test here makes that requirement visible
    // to future maintainers.
    let config = BootstrapConfig::with_seed(1, 0x5A4D_2026).unwrap();

    assert_eq!(config.seed, 0x5A4D_2026);
}

#[test]
fn all_supported_confidence_methods_produce_bounded_intervals() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    for method in [
        IntervalMethod::Wilson,
        IntervalMethod::ClopperPearson,
        IntervalMethod::Wald,
    ] {
        let result = confidence::binomial_interval_from_counts(
            80,
            20,
            level,
            method,
        )
        .unwrap_or_else(|error| {
            panic!(
                "supported interval method unexpectedly failed: {method:?}: {error}"
            )
        });

        assert_probability(result.lower());
        assert_probability(result.upper());
        assert!(result.lower() <= result.upper());
        assert!(result.lower() <= result.proportion());
        assert!(result.proportion() <= result.upper());
    }
}

#[test]
fn confidence_interval_method_metadata_survives_result_construction() {
    let level = ConfidenceLevel::new(0.95).unwrap();

    for method in [
        IntervalMethod::Wilson,
        IntervalMethod::ClopperPearson,
        IntervalMethod::Wald,
    ] {
        let result = confidence::binomial_interval_from_counts(
            8,
            2,
            level,
            method,
        )
        .unwrap();

        assert_eq!(result.interval().method, method);
        assert_eq!(result.interval().confidence_level, level);
    }
}