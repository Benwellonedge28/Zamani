//! Zamani Quantum Benchmarking — QEC Integration Test Suite.
//!
//! Production-oriented tests for:
//!
//! - QEC benchmarking namespace integrity;
//! - benchmark-family identifiers;
//! - logical-error benchmarking;
//! - unknown-outcome safety;
//! - Wilson confidence intervals;
//! - logical X/Y/Z decomposition;
//! - physical/logical comparison;
//! - code-distance sweeps;
//! - suppression calculations;
//! - deterministic behavior;
//! - invalid-input rejection;
//! - resource/capacity guards;
//! - public API integration of the QEC benchmark family.
//!
//! # Architectural boundary
//!
//! This file tests benchmarking behavior only.
//!
//! It does NOT:
//!
//! - execute a QPU;
//! - access credentials;
//! - access a network;
//! - mutate calibration;
//! - implement a decoder;
//! - implement stabilizer algebra;
//! - implement threshold fitting;
//! - implement surface-code mathematics.
//!
//! The canonical QEC implementation remains responsible for producing
//! `LogicalOutcome` values. The benchmarking layer is responsible for
//! measuring and statistically analyzing those outcomes.
//!
//! # Integration contract
//!
//! ```text
//! quantum::error_correction::logical::LogicalOutcome
//!                         │
//!                         ▼
//!              benchmarking::qec::logical
//!                         │
//!                         ▼
//!                  qec_tests.rs
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!          logical     statistics   regression
//!          metrics      safety       safety
//! ```
//!
//! The tests intentionally use the public API rather than private fields.
//! Consequently they also act as API compatibility tests for future Zamani
//! language, simulator, hardware, reporting, and CI integrations.
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
//! # Determinism
//!
//! Every test uses deterministic inputs. No wall clock, random generator,
//! filesystem, network, thread scheduling, or global mutable state is used.
//!
//! # Security
//!
//! Tests deliberately include malformed and adversarial configuration values
//! so that benchmark input validation remains fail-closed.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use zamani_compiler::quantum::benchmarking::qec::{
    QecBenchmarkKind,
    QEC_BENCHMARKING_API_VERSION,
    QEC_BENCHMARKING_ARCHITECTURE,
    QEC_BENCHMARKING_SUBSYSTEM_ID,
    DECODER_BENCHMARK_ID,
    LOGICAL_BENCHMARK_ID,
    PHYSICAL_BENCHMARK_ID,
    RESOURCE_OVERHEAD_BENCHMARK_ID,
    SURFACE_CODE_BENCHMARK_ID,
    SYNDROME_BENCHMARK_ID,
    THRESHOLD_BENCHMARK_ID,
};

use zamani_compiler::quantum::benchmarking::qec::logical::{
    analyze_logical_outcomes,
    analyze_trials,
    wilson_interval,
    LogicalBenchmark,
    LogicalBenchmarkConfig,
    LogicalBenchmarkComparison,
    LogicalBenchmarkError,
    LogicalBenchmarkResult,
    LogicalBenchmarkSweep,
    LogicalDistancePoint,
    LogicalErrorCounts,
    LogicalTrial,
    UnknownOutcomePolicy,
    DEFAULT_CONFIDENCE_LEVEL,
    DEFAULT_MAX_MATERIALIZED_TRIALS,
    LOGICAL_ERROR_BENCHMARK_ID,
    LOGICAL_ERROR_RESULT_SCHEMA_VERSION,
    MAX_CONFIDENCE_LEVEL,
    MIN_CONFIDENCE_LEVEL,
};

use zamani_compiler::quantum::error_correction::logical::LogicalOutcome;

// ============================================================================
// Test helpers
// ============================================================================

/// Creates the standard production configuration used by most tests.
fn config(
    distance: usize,
) -> LogicalBenchmarkConfig {
    LogicalBenchmarkConfig::new(distance)
        .expect("valid logical benchmark configuration")
}

/// Builds a deterministic outcome sequence.
fn outcomes(
    successes: usize,
    x: usize,
    y: usize,
    z: usize,
) -> Vec<LogicalOutcome> {
    let mut values = Vec::with_capacity(
        successes
            .saturating_add(x)
            .saturating_add(y)
            .saturating_add(z),
    );

    values.extend(
        std::iter::repeat(LogicalOutcome::Identity)
            .take(successes),
    );

    values.extend(
        std::iter::repeat(LogicalOutcome::LogicalX)
            .take(x),
    );

    values.extend(
        std::iter::repeat(LogicalOutcome::LogicalY)
            .take(y),
    );

    values.extend(
        std::iter::repeat(LogicalOutcome::LogicalZ)
            .take(z),
    );

    values
}

/// Asserts that two floating-point values are approximately equal.
///
/// Benchmark tests should not use exact equality for floating-point
/// calculations involving division, square roots, or inverse CDF operations.
fn assert_close(
    actual: f64,
    expected: f64,
    tolerance: f64,
) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected}, got {actual}; difference {difference} > tolerance {tolerance}"
    );
}

// ============================================================================
// Namespace and subsystem integration
// ============================================================================

#[test]
fn qec_subsystem_identity_is_stable() {
    assert_eq!(
        QEC_BENCHMARKING_SUBSYSTEM_ID,
        "zamani.quantum.benchmarking.qec"
    );

    assert_eq!(
        QEC_BENCHMARKING_API_VERSION,
        "1.0.0"
    );

    assert_eq!(
        QEC_BENCHMARKING_ARCHITECTURE,
        "qec-benchmarking-modular-resource-safe"
    );
}

#[test]
fn all_current_qec_benchmark_families_have_stable_identifiers() {
    assert_eq!(
        QecBenchmarkKind::Physical.id(),
        PHYSICAL_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::Logical.id(),
        LOGICAL_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::Syndrome.id(),
        SYNDROME_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::Decoder.id(),
        DECODER_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::Threshold.id(),
        THRESHOLD_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::SurfaceCode.id(),
        SURFACE_CODE_BENCHMARK_ID
    );

    assert_eq!(
        QecBenchmarkKind::ResourceOverhead.id(),
        RESOURCE_OVERHEAD_BENCHMARK_ID
    );
}

#[test]
fn qec_benchmark_family_names_are_stable() {
    assert_eq!(
        QecBenchmarkKind::Physical.name(),
        "Physical QEC"
    );

    assert_eq!(
        QecBenchmarkKind::Logical.name(),
        "Logical QEC"
    );

    assert_eq!(
        QecBenchmarkKind::Syndrome.name(),
        "Syndrome QEC"
    );

    assert_eq!(
        QecBenchmarkKind::Decoder.name(),
        "QEC Decoder"
    );

    assert_eq!(
        QecBenchmarkKind::Threshold.name(),
        "QEC Threshold"
    );

    assert_eq!(
        QecBenchmarkKind::SurfaceCode.name(),
        "Surface Code"
    );

    assert_eq!(
        QecBenchmarkKind::ResourceOverhead.name(),
        "QEC Resource Overhead"
    );
}

#[test]
fn logical_benchmark_requires_logical_execution() {
    assert!(
        QecBenchmarkKind::Logical
            .requires_logical_execution()
    );

    assert!(
        QecBenchmarkKind::Threshold
            .requires_logical_execution()
    );

    assert!(
        QecBenchmarkKind::SurfaceCode
            .requires_logical_execution()
    );

    assert!(
        !QecBenchmarkKind::Physical
            .requires_logical_execution()
    );
}

// ============================================================================
// Canonical LogicalOutcome integration
// ============================================================================

#[test]
fn canonical_logical_outcomes_have_expected_semantics() {
    assert!(
        LogicalOutcome::Identity.is_success()
    );

    assert!(
        !LogicalOutcome::Identity.is_logical_failure()
    );

    assert!(
        !LogicalOutcome::Identity.is_unknown()
    );

    assert!(
        LogicalOutcome::LogicalX.is_logical_failure()
    );

    assert!(
        LogicalOutcome::LogicalY.is_logical_failure()
    );

    assert!(
        LogicalOutcome::LogicalZ.is_logical_failure()
    );

    assert!(
        LogicalOutcome::Unknown.is_unknown()
    );

    assert!(
        !LogicalOutcome::Unknown.is_success()
    );
}

#[test]
fn logical_trial_preserves_canonical_outcome() {
    let trial =
        LogicalTrial::new(LogicalOutcome::LogicalY);

    assert_eq!(
        trial.outcome,
        LogicalOutcome::LogicalY
    );

    assert!(
        trial.is_logical_failure()
    );

    assert!(!trial.is_success());
    assert!(!trial.is_unknown());
}

// ============================================================================
// Configuration validation
// ============================================================================

#[test]
fn valid_logical_configuration_is_accepted() {
    let configuration =
        LogicalBenchmarkConfig::new(3)
            .expect("distance three is valid");

    assert_eq!(
        configuration.code_distance,
        3
    );

    assert_close(
        configuration.confidence_level,
        DEFAULT_CONFIDENCE_LEVEL,
        f64::EPSILON,
    );

    assert_eq!(
        configuration.unknown_outcome_policy,
        UnknownOutcomePolicy::Reject
    );

    assert_eq!(
        configuration.max_materialized_trials,
        DEFAULT_MAX_MATERIALIZED_TRIALS
    );
}

#[test]
fn zero_code_distance_is_rejected() {
    let result =
        LogicalBenchmarkConfig::new(0);

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::InvalidCodeDistance {
                distance: 0
            }
        )
    ));
}

#[test]
fn confidence_level_below_minimum_is_rejected() {
    let configuration =
        config(3)
            .with_confidence_level(
                MIN_CONFIDENCE_LEVEL - 0.000_001,
            );

    assert!(matches!(
        configuration.validate(),
        Err(
            LogicalBenchmarkError::InvalidConfidenceLevel {
                ..
            }
        )
    ));
}

#[test]
fn confidence_level_above_maximum_is_rejected() {
    let configuration =
        config(3)
            .with_confidence_level(
                MAX_CONFIDENCE_LEVEL + 0.000_001,
            );

    assert!(matches!(
        configuration.validate(),
        Err(
            LogicalBenchmarkError::InvalidConfidenceLevel {
                ..
            }
        )
    ));
}

#[test]
fn non_finite_confidence_level_is_rejected() {
    let nan_configuration =
        config(3)
            .with_confidence_level(f64::NAN);

    assert!(matches!(
        nan_configuration.validate(),
        Err(
            LogicalBenchmarkError::InvalidConfidenceLevel {
                ..
            }
        )
    ));

    let infinite_configuration =
        config(3)
            .with_confidence_level(f64::INFINITY);

    assert!(matches!(
        infinite_configuration.validate(),
        Err(
            LogicalBenchmarkError::InvalidConfidenceLevel {
                ..
            }
        )
    ));
}

#[test]
fn physical_error_rate_must_be_a_probability() {
    let negative =
        config(3)
            .with_physical_error_rate(-0.01);

    assert!(
        negative.validate().is_err()
    );

    let above_one =
        config(3)
            .with_physical_error_rate(1.01);

    assert!(
        above_one.validate().is_err()
    );

    let nan =
        config(3)
            .with_physical_error_rate(f64::NAN);

    assert!(
        nan.validate().is_err()
    );
}

#[test]
fn zero_materialization_capacity_is_rejected() {
    let configuration =
        config(3)
            .with_max_materialized_trials(0);

    assert!(matches!(
        configuration.validate(),
        Err(
            LogicalBenchmarkError::CapacityLimitExceeded {
                ..
            }
        )
    ));
}

#[test]
fn excessive_materialization_capacity_is_rejected() {
    let configuration =
        config(3);

    let result =
        LogicalBenchmark::with_capacity(
            configuration,
            configuration
                .max_materialized_trials
                .saturating_add(1),
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::CapacityLimitExceeded {
                ..
            }
        )
    ));
}

// ============================================================================
// Error-count aggregation
// ============================================================================

#[test]
fn logical_error_counts_start_empty() {
    let counts =
        LogicalErrorCounts::new();

    assert_eq!(counts.total(), 0);
    assert_eq!(counts.classified(), 0);
    assert_eq!(counts.logical_failures(), 0);

    counts
        .validate()
        .expect("empty counter is internally consistent");
}

#[test]
fn logical_error_counts_classify_all_outcomes() {
    let mut counts =
        LogicalErrorCounts::new();

    counts.record(
        LogicalOutcome::Identity,
    );

    counts.record(
        LogicalOutcome::LogicalX,
    );

    counts.record(
        LogicalOutcome::LogicalY,
    );

    counts.record(
        LogicalOutcome::LogicalZ,
    );

    counts.record(
        LogicalOutcome::Unknown,
    );

    assert_eq!(counts.total(), 5);
    assert_eq!(counts.classified(), 4);
    assert_eq!(counts.logical_failures(), 3);

    assert_eq!(counts.successes, 1);
    assert_eq!(counts.logical_x, 1);
    assert_eq!(counts.logical_y, 1);
    assert_eq!(counts.logical_z, 1);
    assert_eq!(counts.unknown, 1);

    counts
        .validate()
        .expect("all counters remain consistent");
}

// ============================================================================
// Unknown outcome safety
// ============================================================================

#[test]
fn unknown_outcome_policy_defaults_to_fail_closed_reject() {
    let benchmark =
        LogicalBenchmark::new(config(3))
            .expect("valid benchmark");

    assert_eq!(
        benchmark.config().unknown_outcome_policy,
        UnknownOutcomePolicy::Reject
    );
}

#[test]
fn reject_policy_rejects_unknown_outcome_without_counting_it() {
    let mut benchmark =
        LogicalBenchmark::new(config(3))
            .expect("valid benchmark");

    benchmark
        .record(LogicalOutcome::Identity)
        .expect("identity is valid");

    let result =
        benchmark.record(
            LogicalOutcome::Unknown,
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::UnknownOutcome {
                trial_index: 1
            }
        )
    ));

    assert_eq!(
        benchmark.total_trials(),
        1
    );
}

#[test]
fn exclude_policy_preserves_unknown_accounting() {
    let configuration =
        config(3)
            .with_unknown_policy(
                UnknownOutcomePolicy::Exclude,
            );

    let result =
        analyze_logical_outcomes(
            configuration,
            [
                LogicalOutcome::Identity,
                LogicalOutcome::LogicalX,
                LogicalOutcome::Unknown,
            ],
        )
        .expect(
            "unknown outcomes are excluded by policy",
        );

    assert_eq!(
        result.total_trials,
        3
    );

    assert_eq!(
        result.analyzed_trials,
        2
    );

    assert_eq!(
        result.logical_failures,
        1
    );

    assert_eq!(
        result.unknown_outcomes,
        1
    );

    assert_close(
        result.logical_error_rate,
        0.5,
        1.0e-15,
    );

    assert_eq!(
        result.status(),
        "partial"
    );
}

#[test]
fn count_as_failure_policy_is_conservative() {
    let configuration =
        config(3)
            .with_unknown_policy(
                UnknownOutcomePolicy::CountAsFailure,
            );

    let result =
        analyze_logical_outcomes(
            configuration,
            [
                LogicalOutcome::Identity,
                LogicalOutcome::Unknown,
            ],
        )
        .expect(
            "unknown outcome can be conservatively counted",
        );

    assert_eq!(
        result.total_trials,
        2
    );

    assert_eq!(
        result.analyzed_trials,
        2
    );

    assert_eq!(
        result.logical_failures,
        1
    );

    assert_eq!(
        result.unknown_outcomes,
        1
    );

    assert_close(
        result.logical_error_rate,
        0.5,
        1.0e-15,
    );
}

#[test]
fn all_unknown_excluded_outcomes_produce_no_denominator() {
    let configuration =
        config(3)
            .with_unknown_policy(
                UnknownOutcomePolicy::Exclude,
            );

    let result =
        analyze_logical_outcomes(
            configuration,
            [
                LogicalOutcome::Unknown,
                LogicalOutcome::Unknown,
            ],
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::InvalidDenominator
        )
    ));
}

// ============================================================================
// Logical error-rate calculations
// ============================================================================

#[test]
fn logical_error_rate_is_failures_over_analyzed_trials() {
    let result =
        analyze_logical_outcomes(
            config(5),
            outcomes(90, 5, 3, 2),
        )
        .expect("valid logical experiment");

    assert_eq!(
        result.total_trials,
        100
    );

    assert_eq!(
        result.analyzed_trials,
        100
    );

    assert_eq!(
        result.logical_failures,
        10
    );

    assert_close(
        result.logical_error_rate,
        0.10,
        1.0e-15,
    );

    assert_close(
        result.logical_success_rate,
        0.90,
        1.0e-15,
    );
}

#[test]
fn logical_x_y_z_rates_share_the_documented_denominator() {
    let result =
        analyze_logical_outcomes(
            config(5),
            outcomes(90, 5, 3, 2),
        )
        .expect("valid logical experiment");

    assert_close(
        result.logical_x_error_rate,
        0.05,
        1.0e-15,
    );

    assert_close(
        result.logical_y_error_rate,
        0.03,
        1.0e-15,
    );

    assert_close(
        result.logical_z_error_rate,
        0.02,
        1.0e-15,
    );

    let decomposed =
        result.logical_x_error_rate
            + result.logical_y_error_rate
            + result.logical_z_error_rate;

    assert_close(
        decomposed,
        result.logical_error_rate,
        1.0e-15,
    );
}

#[test]
fn perfect_logical_correction_has_zero_point_error_rate() {
    let result =
        analyze_logical_outcomes(
            config(3),
            std::iter::repeat(
                LogicalOutcome::Identity,
            )
            .take(1_000),
        )
        .expect("perfect experiment is valid");

    assert_eq!(
        result.logical_failures,
        0
    );

    assert_close(
        result.logical_error_rate,
        0.0,
        1.0e-15,
    );

    assert_close(
        result.logical_success_rate,
        1.0,
        1.0e-15,
    );

    assert_eq!(
        result.is_complete(),
        true
    );

    assert_eq!(
        result.status(),
        "complete"
    );
}

#[test]
fn all_logical_failures_have_unit_error_rate() {
    let result =
        analyze_logical_outcomes(
            config(3),
            [
                LogicalOutcome::LogicalX,
                LogicalOutcome::LogicalY,
                LogicalOutcome::LogicalZ,
            ],
        )
        .expect("failure-only experiment is valid");

    assert_eq!(
        result.logical_failures,
        3
    );

    assert_close(
        result.logical_error_rate,
        1.0,
        1.0e-15,
    );

    assert_close(
        result.logical_success_rate,
        0.0,
        1.0e-15,
    );
}

#[test]
fn empty_logical_experiment_is_rejected() {
    let result =
        analyze_logical_outcomes(
            config(3),
            std::iter::empty::<LogicalOutcome>(),
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::EmptyExperiment
        )
    ));
}

// ============================================================================
// Direct LogicalTrial integration
// ============================================================================

#[test]
fn analyze_trials_accepts_canonical_logical_trials() {
    let trials = [
        LogicalTrial::new(
            LogicalOutcome::Identity,
        ),
        LogicalTrial::new(
            LogicalOutcome::LogicalX,
        ),
        LogicalTrial::new(
            LogicalOutcome::LogicalZ,
        ),
    ];

    let result =
        analyze_trials(
            config(3),
            trials,
        )
        .expect("trial analysis succeeds");

    assert_eq!(
        result.total_trials,
        3
    );

    assert_eq!(
        result.logical_failures,
        2
    );

    assert_close(
        result.logical_error_rate,
        2.0 / 3.0,
        1.0e-15,
    );
}

// ============================================================================
// Streaming benchmark integration
// ============================================================================

#[test]
fn streaming_and_batch_analysis_are_equivalent() {
    let input = outcomes(
        80,
        10,
        5,
        5,
    );

    let batch =
        analyze_logical_outcomes(
            config(5),
            input.clone(),
        )
        .expect("batch analysis succeeds");

    let mut streaming =
        LogicalBenchmark::new(
            config(5),
        )
        .expect("streaming benchmark succeeds");

    for outcome in input {
        streaming
            .record(outcome)
            .expect("record succeeds");
    }

    let streaming_result =
        streaming
            .finalize()
            .expect("finalization succeeds");

    assert_eq!(
        batch.total_trials,
        streaming_result.total_trials
    );

    assert_eq!(
        batch.analyzed_trials,
        streaming_result.analyzed_trials
    );

    assert_eq!(
        batch.logical_failures,
        streaming_result.logical_failures
    );

    assert_eq!(
        batch.logical_x_failures,
        streaming_result.logical_x_failures
    );

    assert_eq!(
        batch.logical_y_failures,
        streaming_result.logical_y_failures
    );

    assert_eq!(
        batch.logical_z_failures,
        streaming_result.logical_z_failures
    );

    assert_close(
        batch.logical_error_rate,
        streaming_result.logical_error_rate,
        1.0e-15,
    );
}

#[test]
fn with_capacity_accepts_capacity_at_configured_limit() {
    let configuration =
        config(3);

    let benchmark =
        LogicalBenchmark::with_capacity(
            configuration,
            configuration.max_materialized_trials,
        )
        .expect(
            "capacity exactly at configured limit is valid",
        );

    assert_eq!(
        benchmark.total_trials(),
        0
    );
}

// ============================================================================
// Wilson confidence interval
// ============================================================================

#[test]
fn wilson_interval_rejects_zero_trials() {
    let result =
        wilson_interval(
            0,
            0,
            DEFAULT_CONFIDENCE_LEVEL,
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::InvalidDenominator
        )
    ));
}

#[test]
fn wilson_interval_rejects_successes_above_trials() {
    let result =
        wilson_interval(
            11,
            10,
            DEFAULT_CONFIDENCE_LEVEL,
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::InconsistentCounts
        )
    ));
}

#[test]
fn wilson_interval_contains_point_estimate() {
    let interval =
        wilson_interval(
            10,
            100,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid Wilson interval");

    assert_close(
        interval.estimate,
        0.10,
        1.0e-15,
    );

    assert!(
        interval.lower
            <= interval.estimate
    );

    assert!(
        interval.estimate
            <= interval.upper
    );

    assert!(
        interval.lower >= 0.0
    );

    assert!(
        interval.upper <= 1.0
    );

    assert_eq!(
        interval.successes,
        10
    );

    assert_eq!(
        interval.trials,
        100
    );
}

#[test]
fn wilson_interval_has_positive_width_for_finite_sample() {
    let interval =
        wilson_interval(
            10,
            100,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid Wilson interval");

    assert!(
        interval.width() > 0.0
    );
}

#[test]
fn wilson_interval_midpoint_is_inside_interval() {
    let interval =
        wilson_interval(
            25,
            100,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid Wilson interval");

    let midpoint =
        interval.midpoint();

    assert!(
        midpoint >= interval.lower
    );

    assert!(
        midpoint <= interval.upper
    );
}

#[test]
fn wilson_interval_is_deterministic() {
    let first =
        wilson_interval(
            37,
            10_000,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid interval");

    let second =
        wilson_interval(
            37,
            10_000,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid interval");

    assert_eq!(
        first,
        second
    );
}

#[test]
fn larger_sample_size_narrows_wilson_interval() {
    let small =
        wilson_interval(
            10,
            100,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid interval");

    let large =
        wilson_interval(
            100,
            1_000,
            DEFAULT_CONFIDENCE_LEVEL,
        )
        .expect("valid interval");

    assert!(
        large.width()
            < small.width()
    );
}

// ============================================================================
// Physical-to-logical suppression
// ============================================================================

#[test]
fn physical_error_rate_can_be_attached_as_metadata() {
    let result =
        analyze_logical_outcomes(
            config(5)
                .with_physical_error_rate(
                    0.02,
                ),
            outcomes(99, 1, 0, 0),
        )
        .expect("valid benchmark");

    assert_eq!(
        result.physical_error_rate,
        Some(0.02)
    );

    assert!(
        result.suppresses_physical_error()
            .expect("physical rate is available")
    );

    assert_close(
        result.logical_to_physical_error_ratio
            .expect("physical rate is non-zero"),
        0.5,
        1.0e-15,
    );
}

#[test]
fn zero_physical_error_rate_does_not_create_infinite_ratio() {
    let result =
        analyze_logical_outcomes(
            config(5)
                .with_physical_error_rate(
                    0.0,
                ),
            outcomes(99, 1, 0, 0),
        )
        .expect("valid benchmark");

    assert_eq!(
        result.physical_error_rate,
        Some(0.0)
    );

    assert_eq!(
        result.logical_to_physical_error_ratio,
        None
    );
}

#[test]
fn statistical_suppression_requires_upper_bound_below_physical_rate() {
    let result =
        analyze_logical_outcomes(
            config(5),
            outcomes(99, 1, 0, 0),
        )
        .expect("valid benchmark");

    let physically_small =
        result
            .statistically_below(0.000_000_1)
            .expect("valid probability");

    assert!(
        !physically_small
    );

    let physically_large =
        result
            .statistically_below(0.1)
            .expect("valid probability");

    assert!(
        physically_large
    );
}

#[test]
fn statistically_below_rejects_invalid_probability() {
    let result =
        analyze_logical_outcomes(
            config(3),
            outcomes(9, 1, 0, 0),
        )
        .expect("valid benchmark");

    assert!(
        result
            .statistically_below(-0.1)
            .is_err()
    );

    assert!(
        result
            .statistically_below(1.1)
            .is_err()
    );

    assert!(
        result
            .statistically_below(f64::NAN)
            .is_err()
    );
}

// ============================================================================
// Distance sweep
// ============================================================================

fn result_at_distance(
    distance: usize,
    failures: usize,
    trials: usize,
) -> LogicalBenchmarkResult {
    let successes =
        trials
            .checked_sub(failures)
            .expect("failures cannot exceed trials");

    analyze_logical_outcomes(
        config(distance),
        std::iter::repeat(
            LogicalOutcome::Identity,
        )
        .take(successes)
        .chain(
            std::iter::repeat(
                LogicalOutcome::LogicalX,
            )
            .take(failures),
        ),
    )
    .expect("valid distance benchmark")
}

#[test]
fn distance_point_requires_matching_result_distance() {
    let result =
        result_at_distance(
            3,
            10,
            100,
        );

    let point =
        LogicalDistancePoint::new(
            3,
            result,
        )
        .expect("matching distance");

    assert_eq!(
        point.distance,
        3
    );

    assert_eq!(
        point.result.code_distance,
        3
    );
}

#[test]
fn distance_point_rejects_zero_distance() {
    let result =
        result_at_distance(
            3,
            10,
            100,
        );

    let point =
        LogicalDistancePoint::new(
            0,
            result,
        );

    assert!(matches!(
        point,
        Err(
            LogicalBenchmarkError::InvalidCodeDistance {
                distance: 0
            }
        )
    ));
}

#[test]
fn distance_point_rejects_mismatched_result() {
    let result =
        result_at_distance(
            3,
            10,
            100,
        );

    let point =
        LogicalDistancePoint::new(
            5,
            result,
        );

    assert!(matches!(
        point,
        Err(
            LogicalBenchmarkError::InconsistentCounts
        )
    ));
}

#[test]
fn distance_sweep_starts_empty() {
    let sweep =
        LogicalBenchmarkSweep::new();

    assert!(
        sweep.is_empty()
    );

    assert_eq!(
        sweep.len(),
        0
    );

    assert!(
        sweep.first().is_none()
    );

    assert!(
        sweep.last().is_none()
    );
}

#[test]
fn distance_sweep_requires_strictly_increasing_distances() {
    let mut sweep =
        LogicalBenchmarkSweep::new();

    let first =
        LogicalDistancePoint::new(
            3,
            result_at_distance(
                3,
                10,
                100,
            ),
        )
        .expect("valid point");

    sweep
        .push(first)
        .expect("first point is valid");

    let duplicate =
        LogicalDistancePoint::new(
            3,
            result_at_distance(
                3,
                9,
                100,
            ),
        )
        .expect("valid point");

    let result =
        sweep.push(duplicate);

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::NonMonotonicDistance {
                previous: 3,
                current: 3
            }
        )
    ));
}

#[test]
fn distance_sweep_rejects_decreasing_distance() {
    let mut sweep =
        LogicalBenchmarkSweep::new();

    sweep
        .push(
            LogicalDistancePoint::new(
                5,
                result_at_distance(
                    5,
                    5,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("first point");

    let result =
        sweep.push(
            LogicalDistancePoint::new(
                3,
                result_at_distance(
                    3,
                    10,
                    100,
                ),
            )
            .expect("valid point"),
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::NonMonotonicDistance {
                previous: 5,
                current: 3
            }
        )
    ));
}

#[test]
fn distance_sweep_supports_error_suppression_analysis() {
    let mut sweep =
        LogicalBenchmarkSweep::new();

    sweep
        .push(
            LogicalDistancePoint::new(
                3,
                result_at_distance(
                    3,
                    20,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("push succeeds");

    sweep
        .push(
            LogicalDistancePoint::new(
                5,
                result_at_distance(
                    5,
                    10,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("push succeeds");

    sweep
        .push(
            LogicalDistancePoint::new(
                7,
                result_at_distance(
                    7,
                    5,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("push succeeds");

    assert_eq!(
        sweep.len(),
        3
    );

    let ratio =
        sweep
            .suppression_ratio(
                0,
                2,
            )
            .expect("higher-distance error rate is non-zero");

    assert_close(
        ratio,
        4.0,
        1.0e-15,
    );

    let best =
        sweep
            .best_observed()
            .expect("sweep has observations");

    assert_eq!(
        best.distance,
        7
    );
}

#[test]
fn suppression_ratio_rejects_zero_higher_distance_error_rate() {
    let mut sweep =
        LogicalBenchmarkSweep::new();

    sweep
        .push(
            LogicalDistancePoint::new(
                3,
                result_at_distance(
                    3,
                    10,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("push succeeds");

    sweep
        .push(
            LogicalDistancePoint::new(
                5,
                result_at_distance(
                    5,
                    0,
                    100,
                ),
            )
            .expect("valid point"),
        )
        .expect("push succeeds");

    let result =
        sweep.suppression_ratio(
            0,
            1,
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::InvalidDenominator
        )
    ));
}

#[test]
fn distance_sweep_best_observed_is_deterministic() {
    let mut sweep =
        LogicalBenchmarkSweep::new();

    for (distance, failures) in [
        (3, 20),
        (5, 10),
        (7, 10),
        (9, 15),
    ] {
        sweep
            .push(
                LogicalDistancePoint::new(
                    distance,
                    result_at_distance(
                        distance,
                        failures,
                        100,
                    ),
                )
                .expect("valid point"),
            )
            .expect("ordered distance");
    }

    let best =
        sweep
            .best_observed()
            .expect("non-empty sweep");

    assert_eq!(
        best.distance,
        5
    );
}

// ============================================================================
// Benchmark comparison
// ============================================================================

#[test]
fn benchmark_comparison_reports_absolute_difference() {
    let first =
        result_at_distance(
            5,
            20,
            100,
        );

    let second =
        result_at_distance(
            5,
            10,
            100,
        );

    let comparison =
        LogicalBenchmarkComparison::between(
            &first,
            &second,
        )
        .expect("valid comparison");

    assert_close(
        comparison.first_error_rate,
        0.20,
        1.0e-15,
    );

    assert_close(
        comparison.second_error_rate,
        0.10,
        1.0e-15,
    );

    assert_close(
        comparison.absolute_difference,
        0.10,
        1.0e-15,
    );

    assert_close(
        comparison.relative_difference
            .expect("non-zero denominator"),
        1.0,
        1.0e-15,
    );

    assert_close(
        comparison.improvement_factor
            .expect("non-zero denominator"),
        2.0,
        1.0e-15,
    );
}

#[test]
fn benchmark_comparison_handles_zero_second_error_rate_safely() {
    let first =
        result_at_distance(
            5,
            10,
            100,
        );

    let second =
        result_at_distance(
            5,
            0,
            100,
        );

    let comparison =
        LogicalBenchmarkComparison::between(
            &first,
            &second,
        )
        .expect("zero second error rate is a valid comparison");

    assert_eq!(
        comparison.relative_difference,
        None
    );

    assert_eq!(
        comparison.improvement_factor,
        None
    );
}

#[test]
fn benchmark_comparison_rejects_missing_denominator() {
    let first =
        result_at_distance(
            5,
            10,
            100,
        );

    let second =
        LogicalBenchmarkResult {
            benchmark_id:
                LOGICAL_ERROR_BENCHMARK_ID,
            schema_version:
                LOGICAL_ERROR_RESULT_SCHEMA_VERSION,
            code_distance: 5,
            total_trials: 0,
            analyzed_trials: 0,
            logical_failures: 0,
            successes: 0,
            unknown_outcomes: 0,
            logical_x_failures: 0,
            logical_y_failures: 0,
            logical_z_failures: 0,
            logical_error_rate: 0.0,
            logical_success_rate: 1.0,
            logical_error_confidence_interval:
                wilson_interval(
                    0,
                    1,
                    DEFAULT_CONFIDENCE_LEVEL,
                )
                .expect("fixture interval"),
            logical_x_error_rate: 0.0,
            logical_y_error_rate: 0.0,
            logical_z_error_rate: 0.0,
            physical_error_rate: None,
            logical_to_physical_error_ratio:
                None,
            unknown_outcome_policy:
                UnknownOutcomePolicy::Reject,
            confidence_level:
                DEFAULT_CONFIDENCE_LEVEL,
        };

    let comparison =
        LogicalBenchmarkComparison::between(
            &first,
            &second,
        );

    assert!(matches!(
        comparison,
        Err(
            LogicalBenchmarkError::InvalidDenominator
        )
    ));
}

// ============================================================================
// Result invariants
// ============================================================================

#[test]
fn result_schema_identity_is_stable() {
    let result =
        result_at_distance(
            3,
            10,
            100,
        );

    assert_eq!(
        result.benchmark_id,
        LOGICAL_ERROR_BENCHMARK_ID
    );

    assert_eq!(
        result.schema_version,
        LOGICAL_ERROR_RESULT_SCHEMA_VERSION
    );
}

#[test]
fn result_accounting_invariants_hold() {
    let result =
        analyze_logical_outcomes(
            config(5),
            outcomes(70, 10, 15, 5),
        )
        .expect("valid experiment");

    assert_eq!(
        result.total_trials,
        result.successes
            + result.logical_x_failures
            + result.logical_y_failures
            + result.logical_z_failures
            + result.unknown_outcomes
    );

    assert_eq!(
        result.logical_failures,
        result.logical_x_failures
            + result.logical_y_failures
            + result.logical_z_failures
    );

    assert_eq!(
        result.analyzed_trials,
        result.logical_failures
            + result.successes
    );
}

#[test]
fn result_status_is_complete_without_unknowns() {
    let result =
        result_at_distance(
            3,
            10,
            100,
        );

    assert_eq!(
        result.status(),
        "complete"
    );

    assert!(
        result.is_complete()
    );

    assert!(
        result.has_valid_denominator()
    );
}

#[test]
fn result_status_is_partial_when_unknowns_are_excluded() {
    let result =
        analyze_logical_outcomes(
            config(3)
                .with_unknown_policy(
                    UnknownOutcomePolicy::Exclude,
                ),
            [
                LogicalOutcome::Identity,
                LogicalOutcome::Unknown,
            ],
        )
        .expect("partial result is valid");

    assert_eq!(
        result.status(),
        "partial"
    );

    assert!(
        !result.is_complete()
    );

    assert!(
        result.has_valid_denominator()
    );
}

// ============================================================================
// Reproducibility / deterministic regression protection
// ============================================================================

#[test]
fn identical_logical_experiments_produce_identical_results() {
    let input = outcomes(
        123,
        20,
        7,
        5,
    );

    let first =
        analyze_logical_outcomes(
            config(7)
                .with_physical_error_rate(
                    0.015,
                ),
            input.clone(),
        )
        .expect("first experiment");

    let second =
        analyze_logical_outcomes(
            config(7)
                .with_physical_error_rate(
                    0.015,
                ),
            input,
        )
        .expect("second experiment");

    assert_eq!(
        first,
        second
    );
}

#[test]
fn changing_code_distance_changes_result_identity() {
    let first =
        result_at_distance(
            3,
            10,
            100,
        );

    let second =
        result_at_distance(
            5,
            10,
            100,
        );

    assert_ne!(
        first.code_distance,
        second.code_distance
    );
}

// ============================================================================
// API boundary checks
// ============================================================================

#[test]
fn unknown_policy_identifiers_are_stable() {
    assert_eq!(
        UnknownOutcomePolicy::Exclude.as_str(),
        "exclude"
    );

    assert_eq!(
        UnknownOutcomePolicy::CountAsFailure.as_str(),
        "count_as_failure"
    );

    assert_eq!(
        UnknownOutcomePolicy::Reject.as_str(),
        "reject"
    );
}

#[test]
fn logical_outcome_display_is_machine_stable() {
    assert_eq!(
        LogicalOutcome::Identity.to_string(),
        "identity"
    );

    assert_eq!(
        LogicalOutcome::LogicalX.to_string(),
        "logical-X"
    );

    assert_eq!(
        LogicalOutcome::LogicalY.to_string(),
        "logical-Y"
    );

    assert_eq!(
        LogicalOutcome::LogicalZ.to_string(),
        "logical-Z"
    );

    assert_eq!(
        LogicalOutcome::Unknown.to_string(),
        "unknown"
    );
}

#[test]
fn logical_benchmark_result_exposes_expected_public_contract() {
    let result =
        result_at_distance(
            5,
            10,
            100,
        );

    assert_eq!(
        result.code_distance,
        5
    );

    assert_eq!(
        result.total_trials,
        100
    );

    assert_eq!(
        result.analyzed_trials,
        100
    );

    assert_eq!(
        result.logical_failures,
        10
    );

    assert_eq!(
        result.successes,
        90
    );

    assert!(
        result
            .logical_error_confidence_interval
            .lower
            <= result.logical_error_rate
    );

    assert!(
        result.logical_error_rate
            <= result
                .logical_error_confidence_interval
                .upper
    );
}

// ============================================================================
// Scientific safety checks
// ============================================================================

#[test]
fn logical_error_rate_never_leaves_probability_domain() {
    for failures in [
        0usize,
        1,
        10,
        50,
        99,
        100,
    ] {
        let result =
            result_at_distance(
                3,
                failures,
                100,
            );

        assert!(
            result.logical_error_rate
                >= 0.0
        );

        assert!(
            result.logical_error_rate
                <= 1.0
        );

        assert!(
            result.logical_success_rate
                >= 0.0
        );

        assert!(
            result.logical_success_rate
                <= 1.0
        );

        assert!(
            result
                .logical_error_confidence_interval
                .lower
                >= 0.0
        );

        assert!(
            result
                .logical_error_confidence_interval
                .upper
                <= 1.0
        );
    }
}

#[test]
fn logical_error_decomposition_never_exceeds_total_error_rate() {
    let result =
        analyze_logical_outcomes(
            config(7),
            outcomes(900, 50, 30, 20),
        )
        .expect("valid experiment");

    let decomposition =
        result.logical_x_error_rate
            + result.logical_y_error_rate
            + result.logical_z_error_rate;

    assert!(
        decomposition
            <= result.logical_error_rate
                + 1.0e-15
    );
}

#[test]
fn unknown_outcomes_are_never_silently_converted_to_success() {
    let excluded =
        analyze_logical_outcomes(
            config(3)
                .with_unknown_policy(
                    UnknownOutcomePolicy::Exclude,
                ),
            [
                LogicalOutcome::Identity,
                LogicalOutcome::Unknown,
            ],
        )
        .expect("exclude policy");

    assert_eq!(
        excluded.successes,
        1
    );

    assert_eq!(
        excluded.unknown_outcomes,
        1
    );

    assert_eq!(
        excluded.analyzed_trials,
        1
    );

    let conservative =
        analyze_logical_outcomes(
            config(3)
                .with_unknown_policy(
                    UnknownOutcomePolicy::CountAsFailure,
                ),
            [
                LogicalOutcome::Identity,
                LogicalOutcome::Unknown,
            ],
        )
        .expect("conservative policy");

    assert_eq!(
        conservative.successes,
        1
    );

    assert_eq!(
        conservative.logical_failures,
        1
    );

    assert_eq!(
        conservative.analyzed_trials,
        2
    );
}

// ============================================================================
// Resource-safety regression checks
// ============================================================================

#[test]
fn benchmark_does_not_materialize_trials_when_streaming() {
    let mut benchmark =
        LogicalBenchmark::new(
            config(3),
        )
        .expect("valid benchmark");

    for _ in 0..10_000 {
        benchmark
            .record(
                LogicalOutcome::Identity,
            )
            .expect("record succeeds");
    }

    assert_eq!(
        benchmark.total_trials(),
        10_000
    );

    let result =
        benchmark
            .finalize()
            .expect("finalization succeeds");

    assert_eq!(
        result.analyzed_trials,
        10_000
    );

    assert_close(
        result.logical_error_rate,
        0.0,
        1.0e-15,
    );
}

#[test]
fn distance_sweep_capacity_is_bounded() {
    let result =
        LogicalBenchmarkSweep::with_capacity(
            DEFAULT_MAX_MATERIALIZED_TRIALS
                .saturating_add(1),
        );

    assert!(matches!(
        result,
        Err(
            LogicalBenchmarkError::CapacityLimitExceeded {
                ..
            }
        )
    ));
}

// ============================================================================
// Final integration inventory
// ============================================================================

#[test]
fn qec_benchmark_inventory_is_complete_for_current_modules() {
    let inventory = [
        QecBenchmarkKind::Physical,
        QecBenchmarkKind::Logical,
        QecBenchmarkKind::Syndrome,
        QecBenchmarkKind::Decoder,
        QecBenchmarkKind::Threshold,
        QecBenchmarkKind::SurfaceCode,
        QecBenchmarkKind::ResourceOverhead,
    ];

    let ids: Vec<&'static str> =
        inventory
            .iter()
            .map(|kind| kind.id())
            .collect();

    assert_eq!(
        ids,
        vec![
            PHYSICAL_BENCHMARK_ID,
            LOGICAL_BENCHMARK_ID,
            SYNDROME_BENCHMARK_ID,
            DECODER_BENCHMARK_ID,
            THRESHOLD_BENCHMARK_ID,
            SURFACE_CODE_BENCHMARK_ID,
            RESOURCE_OVERHEAD_BENCHMARK_ID,
        ]
    );
}