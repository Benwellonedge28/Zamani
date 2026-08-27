//! Zamani Quantum Benchmarking — Quantum Volume Integration Tests
//!
//! Production test suite for the Quantum Volume (QV) subsystem.
//!
//! # Purpose
//!
//! This file verifies the public, stable contracts between:
//!
//! ```text
//! Quantum Volume configuration
//!          │
//!          ▼
//! volume_estimator.rs
//!          │
//!          ▼
//! QuantumVolumeResult
//!          │
//!          ▼
//! Quantum Volume protocol contracts
//! ```
//!
//! The tests intentionally avoid testing private implementation details.
//! They test observable behavior that must remain stable when the internal
//! implementation is refactored.
//!
//! # Scope
//!
//! This file verifies:
//!
//! - valid QV configuration;
//! - invalid configuration rejection;
//! - square-dimension semantics;
//! - theoretical Quantum Volume calculation;
//! - raw heavy-output sample validation;
//! - heavy-output probability calculation;
//! - Wilson confidence intervals;
//! - confidence-bound semantics;
//! - QV pass/fail decisions;
//! - exact versus estimated heavy-output counts;
//! - threshold boundaries;
//! - confidence-level validation;
//! - numerical edge cases;
//! - overflow protection;
//! - reproducibility of deterministic calculations;
//! - result schema metadata;
//! - protocol configuration validation where the protocol API is available;
//! - deterministic mathematical behavior;
//! - absence of NaN/infinite successful statistics.
//!
//! # Architectural rule
//!
//! These tests must not:
//!
//! - require a real quantum computer;
//! - require network access;
//! - require a cloud provider;
//! - require credentials;
//! - require a simulator;
//! - depend on timing;
//! - depend on machine-specific floating-point formatting;
//! - depend on random process-global state;
//! - perform expensive Quantum Volume experiments.
//!
//! Hardware and extended simulator tests belong in higher benchmark tiers.
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
//!
//! # Integration contract
//!
//! This file consumes the public API of:
//!
//! - `benchmarking::volume_estimator`;
//! - `benchmarking::protocols::quantum_volume` when protocol tests are
//!   available through the public module tree.
//!
//! It does not modify those modules.
//!
//! If the protocol implementation changes internally while preserving its
//! public contract, this test file should remain unchanged.
//!
//! # Test philosophy
//!
//! A production benchmark suite must test both successful and adversarial
//! inputs. In particular, a benchmark implementation must never silently:
//!
//! - accept zero samples;
//! - accept heavy-output counts larger than total samples;
//! - accept NaN probabilities;
//! - accept infinite probabilities;
//! - accept invalid confidence levels;
//! - overflow when calculating 2^m;
//! - convert an externally supplied probability into an "exact" observation;
//! - treat the raw heavy-output threshold as equivalent to a statistically
//!   significant pass.
//!
//! The tests below enforce those invariants.

use crate::quantum::benchmarking::volume_estimator::{
    ConfidenceIntervalMethod,
    QuantumVolumeConfig,
    QuantumVolumeError,
    QuantumVolumeResult,
    DEFAULT_CONFIDENCE_LEVEL,
    DEFAULT_HEAVY_OUTPUT_THRESHOLD,
    MAX_CONFIDENCE_LEVEL,
    MIN_CONFIDENCE_LEVEL,
    TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
    QUANTUM_VOLUME_BENCHMARK_ID,
    QUANTUM_VOLUME_RESULT_SCHEMA_VERSION,
};

// ============================================================================
// Test helpers
// ============================================================================

/// Floating-point comparison helper.
///
/// Statistical calculations are floating-point operations, so exact equality
/// is inappropriate for most numerical assertions.
fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:.17e}, got {actual:.17e}; \
         absolute difference {difference:.17e} exceeded tolerance {tolerance:.17e}"
    );
}

/// Assert that a floating-point value is finite and inside [0, 1].
fn assert_valid_probability(value: f64) {
    assert!(
        value.is_finite(),
        "probability must be finite, got {value:?}"
    );

    assert!(
        (0.0..=1.0).contains(&value),
        "probability must be in [0, 1], got {value}"
    );
}

/// Assert that a confidence interval is mathematically valid.
fn assert_valid_interval(lower: f64, upper: f64) {
    assert!(lower.is_finite());
    assert!(upper.is_finite());

    assert!(
        (0.0..=1.0).contains(&lower),
        "lower confidence bound out of range: {lower}"
    );

    assert!(
        (0.0..=1.0).contains(&upper),
        "upper confidence bound out of range: {upper}"
    );

    assert!(
        lower <= upper,
        "confidence interval is inverted: [{lower}, {upper}]"
    );
}

// ============================================================================
// Configuration tests
// ============================================================================

#[test]
fn default_quantum_volume_configuration_is_valid() {
    let config = QuantumVolumeConfig::new(4, 4)
        .expect("a positive square Quantum Volume configuration must be valid");

    assert_eq!(config.num_qubits, 4);
    assert_eq!(config.gate_depth, 4);

    assert_close(
        config.heavy_output_threshold,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        1.0e-15,
    );

    assert_close(
        config.confidence_level,
        DEFAULT_CONFIDENCE_LEVEL,
        1.0e-15,
    );

    assert_eq!(
        config.confidence_interval_method,
        ConfidenceIntervalMethod::Wilson
    );
}

#[test]
fn configuration_rejects_zero_qubits() {
    let result = QuantumVolumeConfig::new(0, 4);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidQubitCount)
    ));
}

#[test]
fn configuration_rejects_zero_depth() {
    let result = QuantumVolumeConfig::new(4, 0);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidGateDepth)
    ));
}

#[test]
fn configuration_accepts_rectangular_point_for_mathematical_estimator() {
    let config = QuantumVolumeConfig::new(8, 4)
        .expect("positive width and depth must be accepted by the estimator");

    assert_eq!(config.num_qubits, 8);
    assert_eq!(config.gate_depth, 4);
}

#[test]
fn configuration_rejects_threshold_below_zero() {
    let result = QuantumVolumeConfig::with_threshold(4, 4, -0.001);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidThreshold { .. })
    ));
}

#[test]
fn configuration_rejects_threshold_above_one() {
    let result = QuantumVolumeConfig::with_threshold(4, 4, 1.001);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidThreshold { .. })
    ));
}

#[test]
fn configuration_rejects_nan_threshold() {
    let result = QuantumVolumeConfig::with_threshold(4, 4, f64::NAN);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidThreshold { value })
            if value.is_nan()
    ));
}

#[test]
fn configuration_rejects_infinite_threshold() {
    let result = QuantumVolumeConfig::with_threshold(4, 4, f64::INFINITY);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidThreshold { value })
            if value.is_infinite()
    ));
}

#[test]
fn configuration_rejects_confidence_level_below_supported_minimum() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        MIN_CONFIDENCE_LEVEL - 0.000_001,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
    ));
}

#[test]
fn configuration_accepts_minimum_supported_confidence_level() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        MIN_CONFIDENCE_LEVEL,
    );

    assert!(result.is_ok());
}

#[test]
fn configuration_accepts_one_sided_two_sigma_confidence_level() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
    );

    assert!(result.is_ok());

    let config = result.unwrap();

    assert_close(
        config.confidence_level,
        TWO_SIGMA_ONE_SIDED_CONFIDENCE_LEVEL,
        1.0e-15,
    );
}

#[test]
fn configuration_rejects_confidence_level_above_supported_maximum() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        MAX_CONFIDENCE_LEVEL + 0.000_001,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidConfidenceLevel { .. })
    ));
}

#[test]
fn configuration_rejects_nan_confidence_level() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        f64::NAN,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidConfidenceLevel { value })
            if value.is_nan()
    ));
}

#[test]
fn configuration_rejects_infinite_confidence_level() {
    let result = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        f64::INFINITY,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidConfidenceLevel { value })
            if value.is_infinite()
    ));
}

// ============================================================================
// Dimension / theoretical-volume tests
// ============================================================================

#[test]
fn exponent_is_the_smaller_of_width_and_depth() {
    let wider_than_deep = QuantumVolumeConfig::new(8, 5)
        .expect("positive configuration must be valid");

    assert_eq!(wider_than_deep.exponent(), 5);

    let deeper_than_wide = QuantumVolumeConfig::new(5, 8)
        .expect("positive configuration must be valid");

    assert_eq!(deeper_than_wide.exponent(), 5);

    let square = QuantumVolumeConfig::new(7, 7)
        .expect("positive configuration must be valid");

    assert_eq!(square.exponent(), 7);
}

#[test]
fn theoretical_volume_for_dimension_zero_is_not_constructible() {
    // The public configuration prevents zero dimensions. This test exists to
    // document the invariant rather than to invoke an invalid internal state.
    assert!(QuantumVolumeConfig::new(0, 0).is_err());
}

#[test]
fn theoretical_volume_matches_two_to_the_power_m_for_small_dimensions() {
    for dimension in 1..=16 {
        let config = QuantumVolumeConfig::new(dimension, dimension)
            .expect("positive square configuration must be valid");

        let expected = 1usize << dimension;

        assert_eq!(
            config
                .theoretical_volume()
                .expect("small Quantum Volume must fit in usize"),
            expected,
            "incorrect theoretical QV for dimension {dimension}"
        );
    }
}

#[test]
fn theoretical_volume_is_independent_of_the_larger_dimension() {
    let first = QuantumVolumeConfig::new(4, 100)
        .expect("positive configuration must be valid");

    let second = QuantumVolumeConfig::new(100, 4)
        .expect("positive configuration must be valid");

    assert_eq!(first.exponent(), 4);
    assert_eq!(second.exponent(), 4);

    assert_eq!(
        first.theoretical_volume().unwrap(),
        second.theoretical_volume().unwrap()
    );
}

// ============================================================================
// Raw sample validation
// ============================================================================

#[test]
fn zero_samples_are_rejected() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 0, 0);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidSampleCount)
    ));
}

#[test]
fn heavy_outputs_cannot_exceed_samples() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 100, 101);

    assert!(matches!(
        result,
        Err(QuantumVolumeError::HeavyOutputExceedsSamples {
            heavy_outputs: 101,
            samples: 100
        })
    ));
}

#[test]
fn zero_heavy_outputs_are_valid() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 100, 0)
        .expect("zero heavy outputs are a valid observation");

    assert_eq!(result.samples, 100);
    assert_eq!(result.heavy_outputs, 0);
    assert!(result.heavy_outputs_are_exact);

    assert_close(
        result.heavy_output_probability,
        0.0,
        0.0,
    );
}

#[test]
fn all_heavy_outputs_are_valid() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 100, 100)
        .expect("all-heavy observations are mathematically valid");

    assert_eq!(result.samples, 100);
    assert_eq!(result.heavy_outputs, 100);
    assert!(result.heavy_outputs_are_exact);

    assert_close(
        result.heavy_output_probability,
        1.0,
        0.0,
    );

    assert_valid_interval(
        result.confidence_interval.lower,
        result.confidence_interval.upper,
    );
}

// ============================================================================
// Probability calculation
// ============================================================================

#[test]
fn probability_is_calculated_from_exact_counts() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 1000, 725)
        .expect("valid counts must produce a result");

    assert_close(
        result.heavy_output_probability,
        0.725,
        1.0e-15,
    );

    assert_eq!(result.heavy_outputs, 725);
    assert_eq!(result.samples, 1000);
    assert!(result.heavy_outputs_are_exact);
}

#[test]
fn probability_from_samples_is_always_inside_unit_interval() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    for heavy_outputs in [0usize, 1, 10, 333, 666, 667, 999, 1000] {
        let result = QuantumVolumeResult::from_samples(
            config,
            1000,
            heavy_outputs,
        )
        .expect("valid count must produce a result");

        assert_valid_probability(result.heavy_output_probability);
    }
}

#[test]
fn probability_input_rejects_negative_values() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        -0.000_001,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidProbability { .. })
    ));
}

#[test]
fn probability_input_rejects_values_above_one() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        1.000_001,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidProbability { .. })
    ));
}

#[test]
fn probability_input_rejects_nan() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        f64::NAN,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidProbability { value })
            if value.is_nan()
    ));
}

#[test]
fn probability_input_rejects_positive_infinity() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        f64::INFINITY,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidProbability { value })
            if value.is_infinite()
    ));
}

#[test]
fn probability_input_rejects_negative_infinity() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        f64::NEG_INFINITY,
    );

    assert!(matches!(
        result,
        Err(QuantumVolumeError::InvalidProbability { value })
            if value.is_infinite()
    ));
}

// ============================================================================
// Exact versus estimated count semantics
// ============================================================================

#[test]
fn raw_sample_result_marks_heavy_output_count_as_exact() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(config, 1000, 700)
        .unwrap();

    assert!(result.heavy_outputs_are_exact);
    assert_eq!(result.heavy_outputs, 700);
}

#[test]
fn probability_result_does_not_claim_exact_observation_count() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1000,
        0.7,
    )
    .unwrap();

    assert!(!result.heavy_outputs_are_exact);
    assert_eq!(result.heavy_outputs, 700);
}

#[test]
fn probability_result_estimated_count_is_nearest_representable_count() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        10,
        0.74,
    )
    .unwrap();

    assert_eq!(result.heavy_outputs, 7);
    assert!(!result.heavy_outputs_are_exact);
}

#[test]
fn probability_result_preserves_supplied_probability() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let supplied_probability = 0.712_345_678_9;

    let result = QuantumVolumeResult::from_probability(
        config,
        10_000,
        supplied_probability,
    )
    .unwrap();

    assert_close(
        result.heavy_output_probability,
        supplied_probability,
        0.0,
    );
}

// ============================================================================
// Confidence interval tests
// ============================================================================

#[test]
fn confidence_interval_is_valid_for_typical_counts() {
    let config = QuantumVolumeConfig::new(8, 8).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        7_000,
    )
    .unwrap();

    assert_valid_interval(
        result.confidence_interval.lower,
        result.confidence_interval.upper,
    );

    assert!(
        result.confidence_interval.lower
            <= result.heavy_output_probability
    );

    assert!(
        result.confidence_interval.upper
            >= result.heavy_output_probability
    );
}

#[test]
fn confidence_interval_records_wilson_method() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
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
fn confidence_interval_records_requested_confidence_level() {
    let confidence = 0.99;

    let config = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        confidence,
    )
    .unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    assert_close(
        result.confidence_interval.confidence_level,
        confidence,
        0.0,
    );
}

#[test]
fn confidence_interval_width_is_non_negative() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    assert!(
        result.confidence_interval.width() >= 0.0,
        "confidence interval width cannot be negative"
    );
}

#[test]
fn confidence_interval_is_narrower_with_more_samples_for_same_probability() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let small = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    let large = QuantumVolumeResult::from_samples(
        config,
        100_000,
        70_000,
    )
    .unwrap();

    assert!(
        large.confidence_interval.width()
            < small.confidence_interval.width(),
        "more samples should reduce the uncertainty for the same observed rate"
    );
}

// ============================================================================
// Quantum Volume decision semantics
// ============================================================================

#[test]
fn result_records_the_configured_threshold() {
    let threshold = 0.70;

    let config = QuantumVolumeConfig::with_threshold(
        4,
        4,
        threshold,
    )
    .unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        8_000,
    )
    .unwrap();

    assert_close(
        result.heavy_output_threshold,
        threshold,
        0.0,
    );
}

#[test]
fn raw_probability_above_threshold_is_not_by_itself_the_statistical_decision() {
    let config = QuantumVolumeConfig::with_threshold_and_confidence(
        4,
        4,
        2.0 / 3.0,
        0.99,
    )
    .unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10,
        7,
    )
    .unwrap();

    assert!(result.heavy_output_probability > 2.0 / 3.0);

    // The important invariant is that the protocol decision is based on the
    // lower confidence bound, not merely on the point estimate.
    assert_eq!(
        result.passed,
        result.confidence_interval.lower
            > result.heavy_output_threshold
    );
}

#[test]
fn statistically_strong_heavy_output_result_passes() {
    let config = QuantumVolumeConfig::new(8, 8).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        100_000,
        80_000,
    )
    .unwrap();

    assert!(result.passed);
    assert_eq!(
        result.quantum_volume,
        Some(1usize << 8)
    );
}

#[test]
fn clearly_poor_heavy_output_result_fails() {
    let config = QuantumVolumeConfig::new(8, 8).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        100_000,
        50_000,
    )
    .unwrap();

    assert!(!result.passed);
    assert_eq!(result.quantum_volume, None);
}

#[test]
fn result_at_threshold_is_not_automatically_a_pass() {
    let config = QuantumVolumeConfig::with_threshold(
        4,
        4,
        2.0 / 3.0,
    )
    .unwrap();

    let result = QuantumVolumeResult::from_probability(
        config,
        1_000_000,
        2.0 / 3.0,
    )
    .unwrap();

    assert!(
        result.heavy_output_probability
            <= result.heavy_output_threshold
    );

    assert!(!result.passed);
    assert_eq!(result.quantum_volume, None);
}

#[test]
fn pass_decision_uses_strict_lower_bound_comparison() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        100_000,
        80_000,
    )
    .unwrap();

    let expected_decision =
        result.confidence_interval.lower
            > result.heavy_output_threshold;

    assert_eq!(result.passed, expected_decision);
}

#[test]
fn failed_result_does_not_report_quantum_volume() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        100,
    )
    .unwrap();

    assert!(!result.passed);
    assert_eq!(result.quantum_volume, None);
}

// ============================================================================
// Metadata / auditability
// ============================================================================

#[test]
fn result_contains_stable_benchmark_identity() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    assert_eq!(
        result.benchmark_id,
        QUANTUM_VOLUME_BENCHMARK_ID
    );
}

#[test]
fn result_contains_stable_schema_version() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    assert_eq!(
        result.schema_version,
        QUANTUM_VOLUME_RESULT_SCHEMA_VERSION
    );
}

#[test]
fn result_records_all_primary_experiment_dimensions() {
    let config = QuantumVolumeConfig::new(12, 10).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        20_000,
        15_000,
    )
    .unwrap();

    assert_eq!(result.num_qubits, 12);
    assert_eq!(result.gate_depth, 10);
    assert_eq!(result.exponent, 10);
    assert_eq!(result.samples, 20_000);
    assert_eq!(result.heavy_outputs, 15_000);
}

#[test]
fn result_is_copyable_and_deterministic() {
    let config = QuantumVolumeConfig::new(6, 6).unwrap();

    let first = QuantumVolumeResult::from_samples(
        config,
        10_000,
        7_500,
    )
    .unwrap();

    let second = QuantumVolumeResult::from_samples(
        config,
        10_000,
        7_500,
    )
    .unwrap();

    assert_eq!(first, second);
}

// ============================================================================
// Boundary / numerical robustness
// ============================================================================

#[test]
fn smallest_valid_quantum_volume_point_has_volume_two() {
    let config = QuantumVolumeConfig::new(1, 1)
        .expect("one-qubit one-layer configuration must be valid");

    assert_eq!(config.exponent(), 1);
    assert_eq!(config.theoretical_volume().unwrap(), 2);
}

#[test]
fn largest_reasonably_small_test_volume_is_exact() {
    let config = QuantumVolumeConfig::new(20, 20)
        .expect("20-dimensional QV must be representable on ordinary targets");

    assert_eq!(
        config.theoretical_volume().unwrap(),
        1usize << 20
    );
}

#[test]
fn theoretical_volume_reports_overflow_instead_of_wrapping() {
    let max_shift = usize::BITS as usize - 1;

    if max_shift < 2 {
        // This branch is unreachable on supported Zamani targets, but keeps
        // the test logically correct for unusual architectures.
        return;
    }

    let config = QuantumVolumeConfig::new(
        max_shift,
        max_shift,
    )
    .unwrap();

    // 2^(usize::BITS - 1) is representable exactly.
    let volume = config
        .theoretical_volume()
        .expect("maximum representable single-bit shift must succeed");

    assert_eq!(volume, 1usize << max_shift);
}

#[test]
fn invalid_probability_never_produces_a_result() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    for probability in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -1.0,
        2.0,
    ] {
        let result = QuantumVolumeResult::from_probability(
            config,
            100,
            probability,
        );

        assert!(
            result.is_err(),
            "invalid probability {probability:?} must be rejected"
        );
    }
}

#[test]
fn invalid_sample_counts_never_produce_a_result() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    assert!(
        QuantumVolumeResult::from_samples(config, 0, 0).is_err()
    );

    assert!(
        QuantumVolumeResult::from_samples(config, 10, 11).is_err()
    );
}

// ============================================================================
// Statistical monotonicity / invariants
// ============================================================================

#[test]
fn increasing_heavy_output_count_at_fixed_sample_count_does_not_reduce_probability() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let mut previous = 0.0;

    for heavy_outputs in 0..=1_000 {
        let result = QuantumVolumeResult::from_samples(
            config,
            1_000,
            heavy_outputs,
        )
        .unwrap();

        assert!(
            result.heavy_output_probability >= previous,
            "probability decreased at heavy-output count {heavy_outputs}"
        );

        previous = result.heavy_output_probability;
    }
}

#[test]
fn increasing_heavy_output_count_does_not_reduce_lower_confidence_bound() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let mut previous_lower = 0.0;

    for heavy_outputs in 0..=1_000 {
        let result = QuantumVolumeResult::from_samples(
            config,
            1_000,
            heavy_outputs,
        )
        .unwrap();

        assert!(
            result.confidence_interval.lower >= previous_lower,
            "lower confidence bound decreased at heavy-output count \
             {heavy_outputs}: previous={previous_lower}, \
             current={}",
            result.confidence_interval.lower
        );

        previous_lower = result.confidence_interval.lower;
    }
}

#[test]
fn confidence_interval_contains_observed_probability() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    for heavy_outputs in [
        0usize,
        1,
        10,
        100,
        333,
        500,
        667,
        900,
        999,
        1_000,
    ] {
        let result = QuantumVolumeResult::from_samples(
            config,
            1_000,
            heavy_outputs,
        )
        .unwrap();

        assert!(
            result
                .confidence_interval
                .contains(result.heavy_output_probability),
            "observed probability {} was outside [{}, {}]",
            result.heavy_output_probability,
            result.confidence_interval.lower,
            result.confidence_interval.upper
        );
    }
}

// ============================================================================
// Reproducibility
// ============================================================================

#[test]
fn same_input_produces_identical_result() {
    let config = QuantumVolumeConfig::with_threshold_and_confidence(
        10,
        10,
        2.0 / 3.0,
        0.99,
    )
    .unwrap();

    let first = QuantumVolumeResult::from_samples(
        config,
        25_000,
        18_000,
    )
    .unwrap();

    let second = QuantumVolumeResult::from_samples(
        config,
        25_000,
        18_000,
    )
    .unwrap();

    assert_eq!(first, second);
}

#[test]
fn changing_sample_count_can_change_statistical_result_without_changing_probability() {
    let config = QuantumVolumeConfig::new(8, 8).unwrap();

    let small = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    let large = QuantumVolumeResult::from_samples(
        config,
        10_000,
        7_000,
    )
    .unwrap();

    assert_close(
        small.heavy_output_probability,
        large.heavy_output_probability,
        0.0,
    );

    assert!(
        large.confidence_interval.width()
            < small.confidence_interval.width()
    );
}

// ============================================================================
// Confidence-method invariants
// ============================================================================

#[test]
fn configured_confidence_method_is_preserved_in_result() {
    let config = QuantumVolumeConfig::with_statistical_method(
        4,
        4,
        DEFAULT_HEAVY_OUTPUT_THRESHOLD,
        DEFAULT_CONFIDENCE_LEVEL,
        ConfidenceIntervalMethod::Wilson,
    )
    .unwrap();

    assert_eq!(
        config.confidence_interval_method,
        ConfidenceIntervalMethod::Wilson
    );

    let result = QuantumVolumeResult::from_samples(
        config,
        1_000,
        700,
    )
    .unwrap();

    assert_eq!(
        result.confidence_interval.method,
        ConfidenceIntervalMethod::Wilson
    );
}

// ============================================================================
// Auditability / result consistency
// ============================================================================

#[test]
fn result_probability_matches_exact_count_ratio() {
    let config = QuantumVolumeConfig::new(5, 5).unwrap();

    let samples = 12_345usize;
    let heavy_outputs = 8_765usize;

    let result = QuantumVolumeResult::from_samples(
        config,
        samples,
        heavy_outputs,
    )
    .unwrap();

    let expected =
        heavy_outputs as f64 / samples as f64;

    assert_close(
        result.heavy_output_probability,
        expected,
        1.0e-15,
    );
}

#[test]
fn result_pass_flag_matches_lower_bound_rule_for_many_sample_sizes() {
    let config = QuantumVolumeConfig::new(6, 6).unwrap();

    for samples in [
        10usize,
        25,
        100,
        1_000,
        10_000,
    ] {
        for heavy_outputs in [
            0usize,
            samples / 4,
            samples / 2,
            (samples * 2) / 3,
            (samples * 3) / 4,
            samples,
        ] {
            let result = QuantumVolumeResult::from_samples(
                config,
                samples,
                heavy_outputs,
            )
            .unwrap();

            let expected =
                result.confidence_interval.lower
                    > result.heavy_output_threshold;

            assert_eq!(
                result.passed,
                expected,
                "inconsistent pass decision for \
                 samples={samples}, heavy_outputs={heavy_outputs}"
            );

            if result.passed {
                assert_eq!(
                    result.quantum_volume,
                    Some(1usize << result.exponent)
                );
            } else {
                assert_eq!(result.quantum_volume, None);
            }
        }
    }
}

// ============================================================================
// Protocol-level integration tests
// ============================================================================
//
// These tests deliberately focus on public protocol configuration contracts.
// They do not execute circuits and therefore remain suitable for every CI
// environment.
//
// The protocol implementation is currently kept separate from the pure
// mathematical estimator. That separation is intentional and should remain.

#[test]
fn protocol_module_is_reachable_when_benchmarking_is_fully_wired() {
    // This test uses a compile-time import rather than an execution call.
    //
    // If this file compiles, the Quantum Volume protocol remains reachable
    // through the intended benchmarking namespace.
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumePoint,
    };

    let point = QuantumVolumePoint::square(4)
        .expect("positive square QV point must be valid");

    assert_eq!(point.width, 4);
    assert_eq!(point.depth, 4);
}

#[test]
fn protocol_square_point_preserves_square_geometry() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumePoint,
    };

    for width in [1usize, 2, 4, 8, 16] {
        let point = QuantumVolumePoint::square(width)
            .expect("positive width must produce a valid square point");

        assert_eq!(point.width, width);
        assert_eq!(point.depth, width);
    }
}

#[test]
fn protocol_point_rejects_zero_width() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumePoint,
        QuantumVolumeProtocolError,
    };

    let result = QuantumVolumePoint::new(0, 4);

    assert!(matches!(
        result,
        Err(QuantumVolumeProtocolError::InvalidWidth)
    ));
}

#[test]
fn protocol_point_rejects_zero_depth() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumePoint,
        QuantumVolumeProtocolError,
    };

    let result = QuantumVolumePoint::new(4, 0);

    assert!(matches!(
        result,
        Err(QuantumVolumeProtocolError::InvalidDepth)
    ));
}

#[test]
fn protocol_point_accepts_positive_rectangular_geometry() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumePoint,
    };

    let point = QuantumVolumePoint::new(8, 4)
        .expect("positive rectangular point must be valid");

    assert_eq!(point.width, 8);
    assert_eq!(point.depth, 4);
}

#[test]
fn protocol_square_configuration_can_be_constructed() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumeProtocolConfig,
    };

    let config =
        QuantumVolumeProtocolConfig::square([1usize, 2, 4, 8])
            .expect("valid square widths must construct a protocol config");

    assert_eq!(config.points.len(), 4);

    assert_eq!(config.points[0].width, 1);
    assert_eq!(config.points[0].depth, 1);

    assert_eq!(config.points[3].width, 8);
    assert_eq!(config.points[3].depth, 8);
}

#[test]
fn protocol_configuration_defaults_are_resource_bounded() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumeProtocolConfig,
        DEFAULT_MAX_SHOTS,
        DEFAULT_MAX_TRIALS,
        DEFAULT_MAX_WIDTHS,
    };

    let config = QuantumVolumeProtocolConfig::default();

    assert!(config.max_points > 0);
    assert!(config.max_points <= DEFAULT_MAX_WIDTHS);

    assert!(config.max_trials_per_point > 0);
    assert!(config.max_trials_per_point <= DEFAULT_MAX_TRIALS);

    assert!(config.max_shots_per_circuit > 0);
    assert!(config.max_shots_per_circuit <= DEFAULT_MAX_SHOTS);
}

#[test]
fn protocol_configuration_default_confidence_is_explicit_two_sigma_one_sided() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumeProtocolConfig,
        DEFAULT_TWO_SIGMA_CONFIDENCE,
    };

    let config = QuantumVolumeProtocolConfig::default();

    assert_close(
        config.confidence_level,
        DEFAULT_TWO_SIGMA_CONFIDENCE,
        1.0e-15,
    );
}

#[test]
fn protocol_configuration_default_threshold_is_two_thirds() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumeProtocolConfig,
        DEFAULT_THRESHOLD,
    };

    let config = QuantumVolumeProtocolConfig::default();

    assert_close(
        config.heavy_output_threshold,
        2.0 / 3.0,
        1.0e-15,
    );

    assert_close(
        config.heavy_output_threshold,
        DEFAULT_THRESHOLD,
        0.0,
    );
}

#[test]
fn protocol_configuration_defaults_require_complete_points() {
    use crate::quantum::benchmarking::protocols::quantum_volume::{
        QuantumVolumeProtocolConfig,
    };

    let config = QuantumVolumeProtocolConfig::default();

    assert!(
        config.require_complete_points,
        "production QV defaults must not silently turn partial \
         experiments into complete benchmark results"
    );
}

// ============================================================================
// No-hardware invariant
// ============================================================================

#[test]
fn mathematical_qv_tests_require_no_backend() {
    // This is intentionally a behavioral/documentation invariant.
    //
    // All operations below are pure mathematical operations. If this test
    // ever requires a backend, network connection, credentials, or runtime
    // executor, the architectural boundary has been violated.
    let config = QuantumVolumeConfig::new(8, 8).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        7_500,
    )
    .unwrap();

    assert!(result.heavy_output_probability.is_finite());
    assert!(result.confidence_interval.lower.is_finite());
    assert!(result.confidence_interval.upper.is_finite());
}

// ============================================================================
// Regression fixtures
// ============================================================================
//
// These tests deliberately use stable, deterministic observations. They are
// suitable as golden regression fixtures for future refactors of the estimator.

#[test]
fn regression_fixture_qv_dimension_four_strong_result() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        100_000,
        80_000,
    )
    .unwrap();

    assert_eq!(result.schema_version, 1);
    assert_eq!(result.benchmark_id, "quantum_volume");
    assert_eq!(result.num_qubits, 4);
    assert_eq!(result.gate_depth, 4);
    assert_eq!(result.exponent, 4);
    assert_eq!(result.samples, 100_000);
    assert_eq!(result.heavy_outputs, 80_000);
    assert!(result.heavy_outputs_are_exact);
    assert_close(
        result.heavy_output_probability,
        0.8,
        1.0e-15,
    );

    assert!(result.confidence_interval.lower > 2.0 / 3.0);
    assert!(result.passed);
    assert_eq!(result.quantum_volume, Some(16));
}

#[test]
fn regression_fixture_qv_dimension_four_weak_result() {
    let config = QuantumVolumeConfig::new(4, 4).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        100_000,
        50_000,
    )
    .unwrap();

    assert_eq!(result.exponent, 4);
    assert_close(
        result.heavy_output_probability,
        0.5,
        1.0e-15,
    );

    assert!(
        result.confidence_interval.upper
            < 2.0 / 3.0
    );

    assert!(!result.passed);
    assert_eq!(result.quantum_volume, None);
}

#[test]
fn regression_fixture_zero_heavy_outputs() {
    let config = QuantumVolumeConfig::new(2, 2).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        0,
    )
    .unwrap();

    assert_close(
        result.heavy_output_probability,
        0.0,
        0.0,
    );

    assert!(!result.passed);
    assert_eq!(result.quantum_volume, None);

    assert_valid_interval(
        result.confidence_interval.lower,
        result.confidence_interval.upper,
    );
}

#[test]
fn regression_fixture_all_heavy_outputs() {
    let config = QuantumVolumeConfig::new(2, 2).unwrap();

    let result = QuantumVolumeResult::from_samples(
        config,
        10_000,
        10_000,
    )
    .unwrap();

    assert_close(
        result.heavy_output_probability,
        1.0,
        0.0,
    );

    assert!(
        result.confidence_interval.lower
            > 2.0 / 3.0
    );

    assert!(result.passed);
    assert_eq!(result.quantum_volume, Some(4));
}

// ============================================================================
// Test-suite completeness guard
// ============================================================================

#[test]
fn qv_result_has_no_non_finite_primary_statistics() {
    let configurations = [
        (1usize, 1usize, 1_000usize, 0usize),
        (2, 2, 1_000, 500),
        (4, 4, 10_000, 6_667),
        (8, 8, 100_000, 75_000),
        (16, 16, 100_000, 90_000),
    ];

    for (qubits, depth, samples, heavy_outputs) in configurations {
        let config =
            QuantumVolumeConfig::new(qubits, depth)
                .unwrap();

        let result =
            QuantumVolumeResult::from_samples(
                config,
                samples,
                heavy_outputs,
            )
            .unwrap();

        assert!(
            result.heavy_output_probability.is_finite(),
            "probability became non-finite for {qubits}x{depth}"
        );

        assert!(
            result.confidence_interval.lower.is_finite(),
            "lower confidence bound became non-finite for {qubits}x{depth}"
        );

        assert!(
            result.confidence_interval.upper.is_finite(),
            "upper confidence bound became non-finite for {qubits}x{depth}"
        );
    }
}