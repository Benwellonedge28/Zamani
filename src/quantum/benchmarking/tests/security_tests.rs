//! Zamani Quantum Benchmarking — Security and Adversarial Tests
//!
//! Production security-boundary tests for the benchmarking subsystem.
//!
//! This module verifies that untrusted benchmark configuration and externally
//! supplied statistical observations cannot turn the Quantum Volume estimator
//! into a panic source, arithmetic-overflow source, non-finite-statistic
//! source, or silently-invalid scientific result.
//!
//! The benchmark subsystem processes data that may originate from Zamani
//! source programs, serialized requests, simulators, hardware adapters,
//! remote execution services, CI, and persisted results. Validation is
//! therefore part of the scientific correctness contract.
//!
//! # Security boundary
//!
//! These tests cover hostile values including zero dimensions, maximum integer
//! dimensions and counts, invalid count relationships, negative probabilities,
//! values greater than one, NaN, infinities, invalid confidence levels,
//! confidence-interval corruption, decision-boundary ambiguity, integer
//! overflow, non-deterministic behavior, and public-entry-point panics.
//!
//! No test requires hardware, a cloud provider, credentials, network access,
//! a simulator, filesystem state, timing, or process-global mutable state.
//!
//! # Integration contract
//!
//! This file intentionally consumes only the public API of
//! `benchmarking::volume_estimator`. It therefore remains independently useful
//! while protocol, execution, registry, reporting, statistics, and Zamani
//! language integration layers evolve.
//!
//! Downstream modules must preserve the invariants established here:
//!
//! - reject malformed inputs before statistical computation;
//! - never silently accept NaN or infinity as scientific data;
//! - never perform unchecked QV exponent arithmetic;
//! - distinguish exact observations from probability-only estimates;
//! - use the lower confidence bound for the QV statistical decision;
//! - expose explicit statistical-method metadata;
//! - return structured errors rather than panic on hostile public inputs.
//!
//! Rust compatibility: Rust 1.97 / 1.97.1, Rust 2021.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::benchmarking::volume_estimator::{
    ConfidenceInterval,
    ConfidenceIntervalMethod,
    QuantumVolumeConfig,
    QuantumVolumeError,
    QuantumVolumeResult,
    DEFAULT_CONFIDENCE_LEVEL,
    DEFAULT_HEAVY_OUTPUT_THRESHOLD,
    MAX_CONFIDENCE_LEVEL,
    MIN_CONFIDENCE_LEVEL,
    QUANTUM_VOLUME_BENCHMARK_ID,
    QUANTUM_VOLUME_RESULT_SCHEMA_VERSION,
};

// ============================================================================
// Test helpers
// ============================================================================

fn valid_config() -> QuantumVolumeConfig {
    QuantumVolumeConfig::new(4, 4)
        .expect("canonical small production configuration must be valid")
}

fn assert_probability(value: f64) {
    assert!(value.is_finite(), "statistic must be finite: {value:?}");
    assert!(
        (0.0..=1.0).contains(&value),
        "statistic out of range: {value}"
    );
}

fn assert_valid_interval(interval: &ConfidenceInterval) {
    assert!(interval.lower.is_finite());
    assert!(interval.upper.is_finite());

    assert!(
        (0.0..=1.0).contains(&interval.lower),
        "lower confidence bound out of range"
    );

    assert!(
        (0.0..=1.0).contains(&interval.upper),
        "upper confidence bound out of range"
    );

    assert!(
        interval.lower <= interval.upper,
        "confidence interval is inverted"
    );

    assert!(interval.confidence_level.is_finite());

    assert!(
        (MIN_CONFIDENCE_LEVEL..=MAX_CONFIDENCE_LEVEL)
            .contains(&interval.confidence_level)
    );
}

// ============================================================================
// Configuration attack surface
// ============================================================================

#[test]
fn zero_qubits_are_rejected_at_the_public_boundary() {
    assert!(matches!(
        QuantumVolumeConfig::new(0, 1),
        Err(QuantumVolumeError::InvalidQubitCount)
    ));
}

#[test]
fn zero_depth_is_rejected_at_the_public_boundary() {
    assert!(matches!(
        QuantumVolumeConfig::new(1, 0),
        Err(QuantumVolumeError::InvalidGateDepth)
    ));
}

#[test]
fn negative_threshold_is_rejected() {
    assert!(matches!(
        QuantumVolumeConfig::with_threshold(4, 4, -f64::EPSILON),
        Err(QuantumVolumeError::InvalidThreshold { .. })
    ));
}

#[test]
fn threshold_above_one_is_rejected() {
    assert!(matches!(
        QuantumVolumeConfig::with_threshold(4, 4, 1.0 + f64::EPSILON),
        Err(QuantumVolumeError::InvalidThreshold { .. })
    ));
}

#[test]
fn nan_and_infinite_thresholds_are_rejected() {
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(matches!(
            QuantumVolumeConfig::with_threshold(4, 4, value),
            Err(QuantumVolumeError::InvalidThreshold { .. })
        ));
    }
}

#[test]
fn nan_and_infinite_confidence_levels_are_rejected() {
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(matches!(
            QuantumVolumeConfig::with_threshold_and_confidence(
                4,
                4,
                DEFAULT_HEAVY_OUTPUT_THRESHOLD,
                value,
            ),
            Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
        ));
    }
}

#[test]
fn confidence_level_outside_supported_range_is_rejected() {
    assert!(matches!(
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            MIN_CONFIDENCE_LEVEL - f64::EPSILON,
        ),
        Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
    ));

    assert!(matches!(
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            MAX_CONFIDENCE_LEVEL + f64::EPSILON,
        ),
        Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
    ));
}

#[test]
fn supported_confidence_boundaries_are_accepted() {
    assert!(
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            MIN_CONFIDENCE_LEVEL,
        )
        .is_ok()
    );

    assert!(
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            MAX_CONFIDENCE_LEVEL,
        )
        .is_ok()
    );
}

#[test]
fn maximum_integer_dimensions_do_not_panic_validation() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        QuantumVolumeConfig::new(usize::MAX, usize::MAX)
    }));

    assert!(
        result.is_ok(),
        "hostile dimensions must not panic validation"
    );

    assert!(result.unwrap().is_ok());
}

// ============================================================================
// Integer overflow and resource-amplification boundaries
// ============================================================================

#[test]
fn maximum_dimension_cannot_become_an_unchecked_power_of_two() {
    let config = QuantumVolumeConfig::new(usize::MAX, usize::MAX)
        .expect("maximum dimensions are valid configuration values");

    let result = catch_unwind(AssertUnwindSafe(|| {
        config.theoretical_volume()
    }));

    assert!(
        result.is_ok(),
        "QV volume calculation must not panic"
    );

    assert!(
        result.unwrap().is_err(),
        "unrepresentable QV must return an error"
    );
}

#[test]
fn maximum_sample_count_with_zero_heavy_outputs_does_not_panic() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        QuantumVolumeResult::from_samples(
            valid_config(),
            usize::MAX,
            0,
        )
    }));

    assert!(
        result.is_ok(),
        "maximum sample count must not cause a panic"
    );

    let result = result
        .unwrap()
        .expect("zero heavy outputs are valid");

    assert_eq!(result.samples, usize::MAX);
    assert_eq!(result.heavy_outputs, 0);
    assert!(result.heavy_outputs_are_exact);

    assert_probability(result.heavy_output_probability);
    assert_valid_interval(&result.confidence_interval);
}

#[test]
fn heavy_output_count_greater_than_samples_is_rejected_without_wraparound() {
    let result = QuantumVolumeResult::from_samples(
        valid_config(),
        1,
        usize::MAX,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::HeavyOutputExceedsSamples {
            heavy_outputs: usize::MAX,
            samples: 1,
        })
    ));
}

#[test]
fn maximum_equal_counts_are_accepted() {
    let result = QuantumVolumeResult::from_samples(
        valid_config(),
        usize::MAX,
        usize::MAX,
    )
    .expect("equal sample and heavy-output counts are valid");

    assert_eq!(result.samples, usize::MAX);
    assert_eq!(result.heavy_outputs, usize::MAX);
    assert_eq!(result.heavy_output_probability, 1.0);
    assert!(result.heavy_outputs_are_exact);
}

// ============================================================================
// Statistical input validation
// ============================================================================

#[test]
fn invalid_probability_values_are_rejected() {
    for value in [
        f64::NEG_INFINITY,
        -1.0,
        -f64::EPSILON,
        1.0 + f64::EPSILON,
        f64::INFINITY,
        f64::NAN,
    ] {
        assert!(matches!(
            QuantumVolumeResult::from_probability(
                valid_config(),
                1000,
                value,
            ),
            Err(QuantumVolumeError::InvalidProbability { .. })
        ));
    }
}

#[test]
fn zero_samples_are_rejected_even_for_finite_probability() {
    assert!(matches!(
        QuantumVolumeResult::from_probability(
            valid_config(),
            0,
            0.5,
        ),
        Err(QuantumVolumeError::InvalidSampleCount)
    ));
}

#[test]
fn probability_boundaries_zero_and_one_are_accepted() {
    let zero = QuantumVolumeResult::from_probability(
        valid_config(),
        1,
        0.0,
    )
    .expect("zero probability is valid");

    let one = QuantumVolumeResult::from_probability(
        valid_config(),
        1,
        1.0,
    )
    .expect("unit probability is valid");

    assert_eq!(zero.heavy_output_probability, 0.0);
    assert_eq!(one.heavy_output_probability, 1.0);

    assert!(!zero.heavy_outputs_are_exact);
    assert!(!one.heavy_outputs_are_exact);
}

#[test]
fn exact_counts_are_not_confused_with_probability_only_observations() {
    let exact = QuantumVolumeResult::from_samples(
        valid_config(),
        1000,
        710,
    )
    .expect("valid raw counts must succeed");

    let estimated = QuantumVolumeResult::from_probability(
        valid_config(),
        1000,
        0.71,
    )
    .expect("valid probability must succeed");

    assert!(exact.heavy_outputs_are_exact);
    assert_eq!(exact.heavy_outputs, 710);

    assert!(!estimated.heavy_outputs_are_exact);
}

// ============================================================================
// Confidence interval hardening
// ============================================================================

#[test]
fn confidence_interval_rejects_non_finite_bounds() {
    for (lower, upper) in [
        (f64::NAN, 0.9),
        (f64::INFINITY, 0.9),
        (0.1, f64::NAN),
        (0.1, f64::INFINITY),
    ] {
        assert!(matches!(
            ConfidenceInterval::new(
                lower,
                upper,
                DEFAULT_CONFIDENCE_LEVEL,
                ConfidenceIntervalMethod::Wilson,
            ),
            Err(QuantumVolumeError::NonFiniteStatistic { .. })
        ));
    }
}

#[test]
fn confidence_interval_rejects_invalid_bounds() {
    for (lower, upper) in [
        (-f64::EPSILON, 0.9),
        (0.1, 1.0 + f64::EPSILON),
        (0.9, 0.1),
    ] {
        assert!(matches!(
            ConfidenceInterval::new(
                lower,
                upper,
                DEFAULT_CONFIDENCE_LEVEL,
                ConfidenceIntervalMethod::Wilson,
            ),
            Err(QuantumVolumeError::InvalidProbability { .. })
        ));
    }
}

#[test]
fn confidence_interval_rejects_invalid_confidence_level() {
    assert!(matches!(
        ConfidenceInterval::new(
            0.1,
            0.9,
            f64::NAN,
            ConfidenceIntervalMethod::Wilson,
        ),
        Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
    ));
}

#[test]
fn valid_closed_unit_interval_is_accepted() {
    let interval = ConfidenceInterval::new(
        0.0,
        1.0,
        DEFAULT_CONFIDENCE_LEVEL,
        ConfidenceIntervalMethod::Wilson,
    )
    .expect("closed unit interval is valid");

    assert_valid_interval(&interval);
    assert_eq!(interval.width(), 1.0);
}

// ============================================================================
// Statistical decision security
// ============================================================================

#[test]
fn raw_probability_is_not_alone_sufficient_for_a_qv_pass() {
    let config =
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            DEFAULT_HEAVY_OUTPUT_THRESHOLD,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        3,
        2,
    )
    .expect("two heavy observations out of three are valid");

    assert_probability(result.heavy_output_probability);

    assert!(!result.passed);

    assert!(
        result.confidence_interval.lower
            <= result.heavy_output_threshold
    );
}

#[test]
fn qv_pass_requires_lower_confidence_bound_strictly_above_threshold() {
    let config =
        QuantumVolumeConfig::with_threshold_and_confidence(
            4,
            4,
            0.5,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        9_000,
    )
    .expect("valid high-quality observations must succeed");

    assert!(
        result.confidence_interval.lower
            > result.heavy_output_threshold
    );

    assert!(result.passed);
    assert_eq!(result.quantum_volume, Some(16));
}

#[test]
fn threshold_touching_is_not_a_strict_pass() {
    let interval = ConfidenceInterval::new(
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        0.9,
        DEFAULT_CONFIDENCE_LEVEL,
        ConfidenceIntervalMethod::Wilson,
    )
    .unwrap();

    assert!(
        !interval.strictly_above(
            DEFAULT_HEAVY_OUTPUT_THRESHOLD
        )
    );
}

#[test]
fn non_finite_thresholds_cannot_be_used_as_success_conditions() {
    let interval = ConfidenceInterval::new(
        0.8,
        0.9,
        DEFAULT_CONFIDENCE_LEVEL,
        ConfidenceIntervalMethod::Wilson,
    )
    .unwrap();

    assert!(!interval.strictly_above(f64::NAN));
    assert!(!interval.strictly_above(f64::INFINITY));
    assert!(!interval.strictly_above(f64::NEG_INFINITY));
}

// ============================================================================
// Reproducibility and result-integrity guards
// ============================================================================

#[test]
fn identical_inputs_produce_identical_results() {
    let config = valid_config();

    let first =
        QuantumVolumeResult::from_samples(
            config,
            10_000,
            7_250,
        )
        .unwrap();

    let second =
        QuantumVolumeResult::from_samples(
            config,
            10_000,
            7_250,
        )
        .unwrap();

    assert_eq!(first, second);
}

#[test]
fn configuration_validation_is_idempotent() {
    let config = valid_config();

    let first = config
        .validate()
        .expect("first validation must succeed");

    let second = first
        .validate()
        .expect("second validation must succeed");

    assert_eq!(first, second);
}

#[test]
fn result_identity_metadata_is_stable() {
    let result =
        QuantumVolumeResult::from_samples(
            valid_config(),
            1000,
            800,
        )
        .unwrap();

    assert_eq!(
        result.schema_version,
        QUANTUM_VOLUME_RESULT_SCHEMA_VERSION
    );

    assert_eq!(
        result.benchmark_id,
        QUANTUM_VOLUME_BENCHMARK_ID
    );
}

#[test]
fn successful_results_have_finite_bounded_primary_statistics() {
    for (samples, heavy_outputs) in [
        (1usize, 0usize),
        (1000usize, 500usize),
        (1000usize, 999usize),
        (usize::MAX, 0usize),
        (usize::MAX, usize::MAX),
    ] {
        let result =
            QuantumVolumeResult::from_samples(
                valid_config(),
                samples,
                heavy_outputs,
            )
            .expect("selected count pairs are valid");

        assert_probability(
            result.heavy_output_probability
        );

        assert_valid_interval(
            &result.confidence_interval
        );
    }
}

// ============================================================================
// Panic-safety matrices
// ============================================================================

#[test]
fn hostile_configuration_matrix_does_not_panic() {
    let thresholds = [
        f64::NEG_INFINITY,
        -1.0,
        -f64::EPSILON,
        0.0,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        1.0,
        1.0 + f64::EPSILON,
        f64::INFINITY,
        f64::NAN,
    ];

    let confidence_levels = [
        f64::NEG_INFINITY,
        0.0,
        MIN_CONFIDENCE_LEVEL - f64::EPSILON,
        MIN_CONFIDENCE_LEVEL,
        DEFAULT_CONFIDENCE_LEVEL,
        MAX_CONFIDENCE_LEVEL,
        MAX_CONFIDENCE_LEVEL + f64::EPSILON,
        f64::INFINITY,
        f64::NAN,
    ];

    for threshold in thresholds {
        for confidence_level in confidence_levels {
            let result = catch_unwind(
                AssertUnwindSafe(|| {
                    QuantumVolumeConfig::
                        with_threshold_and_confidence(
                            usize::MAX,
                            usize::MAX,
                            threshold,
                            confidence_level,
                        )
                }),
            );

            assert!(
                result.is_ok(),
                "validation panicked for \
                 threshold={threshold:?}, \
                 confidence={confidence_level:?}"
            );
        }
    }
}

#[test]
fn hostile_probability_matrix_does_not_panic() {
    let probabilities = [
        f64::NEG_INFINITY,
        -1.0,
        -f64::EPSILON,
        0.0,
        0.5,
        1.0,
        1.0 + f64::EPSILON,
        f64::INFINITY,
        f64::NAN,
    ];

    for samples in [
        0usize,
        1,
        2,
        1000,
        usize::MAX,
    ] {
        for probability in probabilities {
            let result = catch_unwind(
                AssertUnwindSafe(|| {
                    QuantumVolumeResult::from_probability(
                        valid_config(),
                        samples,
                        probability,
                    )
                }),
            );

            assert!(
                result.is_ok(),
                "probability validation panicked for \
                 samples={samples}, \
                 probability={probability:?}"
            );
        }
    }
}

// ============================================================================
// Explicit protocol-boundary invariants
// ============================================================================

#[test]
fn estimator_remains_execution_independent() {
    // Deliberately no executor, backend, simulator, network,
    // credentials, timing, or hardware dependency.
    let result =
        QuantumVolumeResult::from_samples(
            valid_config(),
            1000,
            800,
        )
        .expect(
            "pure mathematical estimation must succeed \
             without execution"
        );

    assert_probability(
        result.heavy_output_probability
    );

    assert_valid_interval(
        &result.confidence_interval
    );
}

#[test]
fn statistical_method_is_explicit_in_every_confidence_interval() {
    let result =
        QuantumVolumeResult::from_samples(
            valid_config(),
            1000,
            800,
        )
        .unwrap();

    assert_eq!(
        result.confidence_interval.method,
        ConfidenceIntervalMethod::Wilson
    );

    assert_eq!(
        result.confidence_interval.method.as_str(),
        "wilson"
    );
}

#[test]
fn defaults_remain_finite_and_bounded() {
    let config = valid_config();

    assert!(config.heavy_output_threshold.is_finite());
    assert!(config.confidence_level.is_finite());

    assert!(
        (0.0..=1.0)
            .contains(&config.heavy_output_threshold)
    );

    assert!(
        (MIN_CONFIDENCE_LEVEL..=MAX_CONFIDENCE_LEVEL)
            .contains(&config.confidence_level)
    );
}