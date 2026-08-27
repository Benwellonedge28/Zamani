//! Zamani Quantum Benchmarking — Regression Analysis Tests
//!
//! Production integration tests for:
//!
//!     src/quantum/benchmarking/analysis/regression.rs
//!
//! # Purpose
//!
//! These tests verify the complete historical benchmark regression contract:
//!
//! ```text
//! BaselineMetric
//!       │
//!       ▼
//! Baseline::compare_metrics()
//!       │
//!       ▼
//! BaselineSetComparison
//!       │
//!       ▼
//! analysis::regression
//!       │
//!       ▼
//! RegressionReport
//! ```
//!
//! The tests intentionally exercise public APIs only.
//!
//! # Scientific contract
//!
//! A regression is NOT defined merely as:
//!
//!     candidate != baseline
//!
//! Instead:
//!
//! 1. `analysis::compare` determines what changed.
//! 2. `analysis::baseline` determines which scoped baseline belongs to the
//!    candidate.
//! 3. `analysis::regression` applies an explicit regression policy.
//!
//! This separation is essential for production quantum benchmarking because:
//!
//! - higher-is-better metrics must be interpreted differently from
//!   lower-is-better metrics;
//! - numerical changes are not automatically statistically significant;
//! - dimensions such as width, depth and problem size are part of benchmark
//!   identity;
//! - missing data must never silently become zero;
//! - policy thresholds must be explicit;
//! - historical baselines must remain immutable;
//! - CI behavior must be deterministic.
//!
//! # Benchmark families covered by this contract
//!
//! These tests are intentionally protocol-independent. The same regression
//! machinery can therefore be used by:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved RB;
//! - simultaneous RB;
//! - purity RB;
//! - leakage RB;
//! - cycle benchmarking;
//! - layer fidelity;
//! - XEB;
//! - random circuit sampling;
//! - mirror circuits;
//! - SPAM;
//! - gate/process fidelity;
//! - coherence;
//! - crosstalk;
//! - drift;
//! - tomography;
//! - volumetric benchmarking;
//! - application benchmarks;
//! - QEC benchmarks;
//! - custom Zamani benchmarks.
//!
//! # Architectural boundary
//!
//! This test module does NOT:
//!
//! - execute quantum circuits;
//! - access quantum hardware;
//! - access a simulator;
//! - generate random circuits;
//! - access clocks;
//! - access files;
//! - access the network;
//! - inspect environment variables;
//! - mutate global state;
//! - implement comparison mathematics;
//! - implement statistical hypothesis tests;
//! - implement baseline lookup;
//! - implement regression policy logic itself.
//!
//! Those responsibilities belong to the production modules under test.
//!
//! # Resource-safety contract
//!
//! The tests verify that malformed or hostile regression inputs are rejected
//! before they can cause unbounded work.
//!
//! In particular:
//!
//! - zero metric limits are rejected;
//! - invalid thresholds are rejected;
//! - excessive candidate collections are rejected;
//! - missing scoped metrics are not silently ignored when CI completeness is
//!   required.
//!
//! # Reproducibility contract
//!
//! Regression analysis must be deterministic for identical inputs.
//!
//! No test depends on:
//!
//! - wall-clock time;
//! - random numbers;
//! - machine identity;
//! - thread scheduling;
//! - filesystem state;
//! - network state;
//! - Rust Debug formatting.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features.
//! No unsafe code.
//! No additional dependencies.
//!
//! # Integration contract
//!
//! This file requires the following authoritative modules:
//!
//! ```text
//! quantum::benchmarking::analysis::baseline
//! quantum::benchmarking::analysis::regression
//! quantum::benchmarking::core::metric
//! ```
//!
//! It does not require protocol implementations.
//!
//! To compile this file, `tests/mod.rs` must expose it with:
//!
//! ```rust
//! mod regression_tests;
//! ```
//!
//! No production source file should be modified specifically for these tests.
//!
//! # Design rule
//!
//! If this test file fails after adding a new benchmark protocol, the protocol
//! should adapt to the established baseline/regression contracts rather than
//! changing regression semantics to accommodate one protocol.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use super::super::analysis::baseline::{
    Baseline,
    BaselineComparisonPolicy,
    BaselineMetric,
};

use super::super::analysis::regression::{
    evaluate_baseline,
    evaluate_baseline_comparison,
    evaluate_metric_comparison,
    RegressionDecision,
    RegressionError,
    RegressionPolicy,
    RegressionReport,
    RegressionSeverity,
    REGRESSION_SCHEMA_VERSION,
    DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD,
    DEFAULT_MAX_REGRESSION_METRICS,
    DEFAULT_RELATIVE_DEGRADATION_THRESHOLD,
    MAX_REGRESSION_THRESHOLD,
};

use super::super::core::metric::{
    Metric,
    MetricKind,
    MetricUnit,
};

// =============================================================================
// Stable test constants
// =============================================================================

const BENCHMARK_ID: &str = "quantum_volume";
const BENCHMARK_VERSION: &str = "1.0.0";

const BASELINE_ID: &str = "qv-regression-baseline";

const WIDTH_4: &[(&str, &str)] = &[("qubits", "4"), ("depth", "4")];
const WIDTH_8: &[(&str, &str)] = &[("qubits", "8"), ("depth", "8")];
const WIDTH_16: &[(&str, &str)] = &[("qubits", "16"), ("depth", "16")];

const QV_BASELINE_VALUE: f64 = 16.0;
const QV_IMPROVED_VALUE: f64 = 32.0;
const QV_REGRESSED_VALUE: f64 = 8.0;

const ERROR_BASELINE_VALUE: f64 = 0.01;
const ERROR_IMPROVED_VALUE: f64 = 0.005;
const ERROR_REGRESSED_VALUE: f64 = 0.02;

const RUNTIME_BASELINE_VALUE: f64 = 100.0;
const RUNTIME_IMPROVED_VALUE: f64 = 80.0;
const RUNTIME_REGRESSED_VALUE: f64 = 130.0;

// =============================================================================
// Fixture helpers
// =============================================================================

fn metric(
    kind: MetricKind,
    unit: MetricUnit,
    value: f64,
) -> Metric {
    Metric::new(kind, unit, value)
        .expect("test metric must be valid")
}

fn qv_metric(value: f64) -> Metric {
    metric(
        MetricKind::QuantumVolume,
        MetricUnit::Dimensionless,
        value,
    )
}

fn error_rate_metric(value: f64) -> Metric {
    metric(
        MetricKind::ErrorRate,
        MetricUnit::Probability,
        value,
    )
}

fn runtime_metric(value: f64) -> Metric {
    metric(
        MetricKind::Runtime,
        MetricUnit::Milliseconds,
        value,
    )
}

fn baseline_metric(
    dimensions: &[(&str, &str)],
    metric: Metric,
) -> BaselineMetric {
    Baseline::builder("fixture-builder")
        .benchmark(BENCHMARK_ID, BENCHMARK_VERSION)
        .add_metric(dimensions.to_vec(), metric)
        .expect("fixture baseline metric must be accepted")
        .build()
        .expect("fixture baseline must build")
        .metrics()
        .first()
        .expect("fixture baseline must contain one metric")
        .clone()
}

fn baseline_with_qv(
    value: f64,
    dimensions: &[(&str, &str)],
) -> Baseline {
    Baseline::builder(BASELINE_ID)
        .benchmark(BENCHMARK_ID, BENCHMARK_VERSION)
        .add_metric(
            dimensions.to_vec(),
            qv_metric(value),
        )
        .expect("QV baseline metric must be accepted")
        .build()
        .expect("QV baseline must build")
}

fn baseline_with_error_rate(
    value: f64,
    dimensions: &[(&str, &str)],
) -> Baseline {
    Baseline::builder(BASELINE_ID)
        .benchmark(BENCHMARK_ID, BENCHMARK_VERSION)
        .add_metric(
            dimensions.to_vec(),
            error_rate_metric(value),
        )
        .expect("error-rate baseline metric must be accepted")
        .build()
        .expect("error-rate baseline must build")
}

fn baseline_with_runtime(
    value: f64,
    dimensions: &[(&str, &str)],
) -> Baseline {
    Baseline::builder(BASELINE_ID)
        .benchmark(BENCHMARK_ID, BENCHMARK_VERSION)
        .add_metric(
            dimensions.to_vec(),
            runtime_metric(value),
        )
        .expect("runtime baseline metric must be accepted")
        .build()
        .expect("runtime baseline must build")
}

fn candidate_metric(
    dimensions: &[(&str, &str)],
    metric: Metric,
) -> BaselineMetric {
    baseline_metric(dimensions, metric)
}

fn default_policy() -> RegressionPolicy {
    RegressionPolicy::default()
}

fn analysis_policy() -> RegressionPolicy {
    RegressionPolicy::analysis()
}

// =============================================================================
// Policy validation
// =============================================================================

#[test]
fn default_policy_is_valid() {
    let policy = default_policy();

    assert!(policy.validate().is_ok());
}

#[test]
fn ci_policy_is_valid() {
    let policy = RegressionPolicy::ci();

    assert!(policy.validate().is_ok());
}

#[test]
fn analysis_policy_is_valid() {
    let policy = analysis_policy();

    assert!(policy.validate().is_ok());
}

#[test]
fn default_policy_uses_documented_thresholds() {
    let policy = default_policy();

    assert_eq!(
        policy.absolute_degradation_threshold,
        DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD
    );

    assert_eq!(
        policy.relative_degradation_threshold,
        DEFAULT_RELATIVE_DEGRADATION_THRESHOLD
    );

    assert!(policy.threshold_is_either);
}

#[test]
fn default_policy_requires_complete_baseline() {
    let policy = default_policy();

    assert!(policy.require_complete_baseline);
}

#[test]
fn default_policy_requires_complete_candidate() {
    let policy = default_policy();

    assert!(policy.require_complete_candidate);
}

#[test]
fn default_policy_fails_on_unresolved_statistics() {
    let policy = default_policy();

    assert!(policy.fail_on_statistically_unresolved);
}

#[test]
fn default_policy_does_not_fail_on_neutral_metrics() {
    let policy = default_policy();

    assert!(!policy.fail_on_neutral);
}

#[test]
fn policy_id_is_deterministic() {
    let first = default_policy().id();
    let second = default_policy().id();

    assert_eq!(first, second);
}

#[test]
fn different_policy_thresholds_have_different_policy_ids() {
    let first = default_policy();

    let mut second = default_policy();
    second.relative_degradation_threshold = 0.10;

    assert_ne!(first.id(), second.id());
}

#[test]
fn different_policy_completeness_settings_have_different_ids() {
    let first = default_policy();

    let mut second = default_policy();
    second.require_complete_baseline = false;

    assert_ne!(first.id(), second.id());
}

#[test]
fn zero_metric_limit_is_rejected() {
    let mut policy = default_policy();
    policy.max_metrics = 0;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidMetricLimit { maximum: 0 })
    ));
}

#[test]
fn negative_absolute_threshold_is_rejected() {
    let mut policy = default_policy();
    policy.absolute_degradation_threshold = -0.0001;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "absolute_degradation_threshold",
            ..
        })
    ));
}

#[test]
fn negative_relative_threshold_is_rejected() {
    let mut policy = default_policy();
    policy.relative_degradation_threshold = -0.0001;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "relative_degradation_threshold",
            ..
        })
    ));
}

#[test]
fn threshold_above_supported_range_is_rejected() {
    let mut policy = default_policy();
    policy.relative_degradation_threshold =
        MAX_REGRESSION_THRESHOLD + f64::EPSILON;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "relative_degradation_threshold",
            ..
        })
    ));
}

#[test]
fn non_finite_absolute_threshold_is_rejected() {
    let mut policy = default_policy();
    policy.absolute_degradation_threshold = f64::NAN;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "absolute_degradation_threshold",
            ..
        })
    ));
}

#[test]
fn positive_infinite_relative_threshold_is_rejected() {
    let mut policy = default_policy();
    policy.relative_degradation_threshold = f64::INFINITY;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "relative_degradation_threshold",
            ..
        })
    ));
}

#[test]
fn negative_infinite_relative_threshold_is_rejected() {
    let mut policy = default_policy();
    policy.relative_degradation_threshold = f64::NEG_INFINITY;

    let result = policy.validate();

    assert!(matches!(
        result,
        Err(RegressionError::InvalidThreshold {
            field: "relative_degradation_threshold",
            ..
        })
    ));
}

// =============================================================================
// Basic improvement/regression behavior
// =============================================================================

#[test]
fn higher_is_better_improvement_is_not_a_regression() {
    let baseline =
        baseline_with_qv(QV_BASELINE_VALUE, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(QV_IMPROVED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("improvement should analyze successfully");

    assert_eq!(
        report.regression_count,
        0
    );

    assert_eq!(
        report.improvement_count,
        1
    );

    assert!(!report.has_regression());
    assert!(!report.is_ci_failure());
    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );
    assert_eq!(
        report.metrics[0].severity,
        RegressionSeverity::None
    );
}

#[test]
fn higher_is_better_material_degradation_is_regression() {
    let baseline =
        baseline_with_qv(QV_BASELINE_VALUE, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(QV_REGRESSED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("regression should analyze successfully");

    assert_eq!(
        report.regression_count,
        1
    );

    assert!(report.has_regression());
    assert!(report.is_ci_failure());

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );

    assert_eq!(
        report.metrics[0].severity,
        RegressionSeverity::Error
    );
}

#[test]
fn lower_is_better_improvement_is_not_a_regression() {
    let baseline =
        baseline_with_error_rate(
            ERROR_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate = candidate_metric(
        WIDTH_4,
        error_rate_metric(ERROR_IMPROVED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("error-rate improvement should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );

    assert_eq!(
        report.regression_count,
        0
    );

    assert!(!report.is_ci_failure());
}

#[test]
fn lower_is_better_degradation_is_regression() {
    let baseline =
        baseline_with_error_rate(
            ERROR_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate = candidate_metric(
        WIDTH_4,
        error_rate_metric(ERROR_REGRESSED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("error-rate regression should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );

    assert_eq!(
        report.regression_count,
        1
    );

    assert!(report.is_ci_failure());
}

#[test]
fn lower_is_better_runtime_improvement_is_recognized() {
    let baseline =
        baseline_with_runtime(
            RUNTIME_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate = candidate_metric(
        WIDTH_4,
        runtime_metric(RUNTIME_IMPROVED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("runtime improvement should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );

    assert_eq!(
        report.regression_count,
        0
    );
}

#[test]
fn lower_is_better_runtime_degradation_is_recognized() {
    let baseline =
        baseline_with_runtime(
            RUNTIME_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate = candidate_metric(
        WIDTH_4,
        runtime_metric(RUNTIME_REGRESSED_VALUE),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("runtime regression should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );

    assert_eq!(
        report.regression_count,
        1
    );
}

// =============================================================================
// Threshold behavior
// =============================================================================

#[test]
fn small_degradation_can_be_classified_below_threshold() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(99.0),
    );

    let mut policy = default_policy();

    policy.absolute_degradation_threshold = 2.0;
    policy.relative_degradation_threshold = 0.05;
    policy.threshold_is_either = true;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("thresholded comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::BelowThreshold
    );

    assert_eq!(
        report.regression_count,
        0
    );

    assert!(!report.is_ci_failure());
}

#[test]
fn relative_threshold_can_detect_small_absolute_change() {
    let baseline =
        baseline_with_qv(1000.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(900.0),
    );

    let mut policy = default_policy();

    policy.absolute_degradation_threshold = 200.0;
    policy.relative_degradation_threshold = 0.05;
    policy.threshold_is_either = true;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("relative threshold comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );

    assert!(
        report.metrics[0]
            .exceeded_relative_threshold
    );

    assert!(
        !report.metrics[0]
            .exceeded_absolute_threshold
    );
}

#[test]
fn absolute_threshold_can_detect_regression_when_relative_threshold_is_not_met() {
    let baseline =
        baseline_with_qv(1000.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(990.0),
    );

    let mut policy = default_policy();

    policy.absolute_degradation_threshold = 5.0;
    policy.relative_degradation_threshold = 0.05;
    policy.threshold_is_either = true;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("absolute threshold comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );

    assert!(
        report.metrics[0]
            .exceeded_absolute_threshold
    );

    assert!(
        !report.metrics[0]
            .exceeded_relative_threshold
    );
}

#[test]
fn both_threshold_mode_requires_both_thresholds() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(90.0),
    );

    let mut policy = default_policy();

    policy.absolute_degradation_threshold = 5.0;
    policy.relative_degradation_threshold = 0.20;
    policy.threshold_is_either = false;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("both-threshold comparison should succeed");

    assert!(
        report.metrics[0]
            .exceeded_absolute_threshold
    );

    assert!(
        !report.metrics[0]
            .exceeded_relative_threshold
    );

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::BelowThreshold
    );
}

#[test]
fn either_threshold_mode_requires_only_one_threshold() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(90.0),
    );

    let mut policy = default_policy();

    policy.absolute_degradation_threshold = 5.0;
    policy.relative_degradation_threshold = 0.20;
    policy.threshold_is_either = true;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("either-threshold comparison should succeed");

    assert!(
        report.metrics[0]
            .exceeded_absolute_threshold
    );

    assert!(
        !report.metrics[0]
            .exceeded_relative_threshold
    );

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Regression
    );
}

#[test]
fn zero_degradation_does_not_trigger_regression_threshold() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(100.0),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("identical values should compare successfully");

    assert_eq!(
        report.metrics[0].degradation,
        0.0
    );

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::NoRegression
    );

    assert_eq!(
        report.regression_count,
        0
    );
}

// =============================================================================
// Direction-normalized degradation
// =============================================================================

#[test]
fn higher_is_better_degradation_is_positive() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(80.0),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert_eq!(
        report.metrics[0].degradation,
        20.0
    );
}

#[test]
fn higher_is_better_improvement_has_negative_degradation() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(120.0),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert_eq!(
        report.metrics[0].degradation,
        -20.0
    );
}

#[test]
fn lower_is_better_degradation_is_positive() {
    let baseline =
        baseline_with_error_rate(0.01, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        error_rate_metric(0.02),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert!(
        report.metrics[0].degradation > 0.0
    );
}

#[test]
fn lower_is_better_improvement_has_negative_degradation() {
    let baseline =
        baseline_with_error_rate(0.01, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        error_rate_metric(0.005),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert!(
        report.metrics[0].degradation < 0.0
    );
}

// =============================================================================
// Relative degradation
// =============================================================================

#[test]
fn higher_is_better_relative_degradation_is_direction_normalized() {
    let baseline =
        baseline_with_qv(100.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(80.0),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    let relative =
        report.metrics[0]
            .relative_degradation
            .expect("relative degradation should exist");

    assert!(
        (relative - 0.20).abs() < 1.0e-12
    );
}

#[test]
fn lower_is_better_relative_degradation_is_positive_for_worsening() {
    let baseline =
        baseline_with_error_rate(0.01, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        error_rate_metric(0.012),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    let relative =
        report.metrics[0]
            .relative_degradation
            .expect("relative degradation should exist");

    assert!(
        relative > 0.0
    );
}

#[test]
fn zero_baseline_does_not_create_infinite_relative_degradation() {
    let baseline =
        baseline_with_qv(0.0, WIDTH_4);

    let candidate = candidate_metric(
        WIDTH_4,
        qv_metric(1.0),
    );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("zero-baseline comparison should remain representable");

    assert!(
        report.metrics[0]
            .relative_degradation
            .is_none()
    );

    assert!(
        report.metrics[0]
            .degradation
            .is_finite()
    );
}

// =============================================================================
// Scoped dimensions
// =============================================================================

#[test]
fn identical_metric_values_at_different_dimensions_are_not_cross_compared() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("width 4 baseline should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("width 8 baseline should be valid")
            .build()
            .expect("multi-dimensional baseline should build");

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(16.0),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("scoped comparison should succeed");

    assert_eq!(
        report.compared_metric_count,
        1
    );

    assert_eq!(
        report.metrics[0].identity,
        baseline.metrics()[1].identity()
    );
}

#[test]
fn missing_scope_is_reported_instead_of_matching_another_dimension() {
    let baseline =
        baseline_with_qv(16.0, WIDTH_4);

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(16.0),
        );

    let mut policy = analysis_policy();

    policy.require_complete_baseline = false;
    policy.require_complete_candidate = false;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("analysis mode should represent missing scope");

    assert_eq!(
        report.missing_baseline_count,
        1
    );

    assert!(
        report
            .missing_baseline
            .len()
            == 1
    );

    assert_eq!(
        report.compared_metric_count,
        0
    );
}

#[test]
fn required_missing_baseline_fails_ci_analysis() {
    let baseline =
        baseline_with_qv(16.0, WIDTH_4);

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(16.0),
        );

    let result =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        );

    assert!(matches!(
        result,
        Err(RegressionError::IncompleteBaseline {
            count: 1
        })
    ));
}

// =============================================================================
// Missing candidate metrics
// =============================================================================

#[test]
fn missing_candidate_metric_is_not_treated_as_zero() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("width 4 baseline should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("width 8 baseline should be valid")
            .build()
            .expect("baseline should build");

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(16.0),
        );

    let mut policy = analysis_policy();

    policy.require_complete_baseline = false;
    policy.require_complete_candidate = false;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("analysis mode should represent missing candidate");

    assert_eq!(
        report.missing_candidate_count,
        1
    );

    assert_eq!(
        report.compared_metric_count,
        1
    );

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::NoRegression
    );
}

#[test]
fn required_missing_candidate_fails_ci_analysis() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("width 4 baseline should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("width 8 baseline should be valid")
            .build()
            .expect("baseline should build");

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(16.0),
        );

    let result =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        );

    assert!(matches!(
        result,
        Err(RegressionError::IncompleteCandidate {
            count: 1
        })
    ));
}

// =============================================================================
// Multiple metrics
// =============================================================================

#[test]
fn report_preserves_independent_metric_decisions() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("QV metric should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                error_rate_metric(0.01),
            )
            .expect("error-rate metric should be valid")
            .build()
            .expect("mixed baseline should build");

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            error_rate_metric(0.02),
        ),
    ];

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("mixed benchmark report should succeed");

    assert_eq!(
        report.compared_metric_count,
        2
    );

    assert_eq!(
        report.improvement_count,
        1
    );

    assert_eq!(
        report.regression_count,
        1
    );

    assert!(report.is_ci_failure());

    let first =
        &report.metrics[0];

    let second =
        &report.metrics[1];

    assert_eq!(
        first.decision,
        RegressionDecision::Improvement
    );

    assert_eq!(
        second.decision,
        RegressionDecision::Regression
    );
}

#[test]
fn regression_iterator_returns_only_actual_regressions() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("QV baseline should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("second QV baseline should be valid")
            .build()
            .expect("baseline should build");

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(8.0),
        ),
    ];

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("report should succeed");

    let regressions =
        report.regressions().collect::<Vec<_>>();

    assert_eq!(
        regressions.len(),
        1
    );

    assert_eq!(
        regressions[0].decision,
        RegressionDecision::Regression
    );
}

#[test]
fn gate_failure_iterator_matches_error_severity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("regression report should succeed");

    let failures =
        report.gate_failures().collect::<Vec<_>>();

    assert_eq!(
        failures.len(),
        1
    );

    assert!(
        failures[0].fails_gate()
    );
}

// =============================================================================
// Report invariants
// =============================================================================

#[test]
fn report_contains_expected_schema_version() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("report should succeed");

    assert_eq!(
        report.schema_version,
        REGRESSION_SCHEMA_VERSION
    );
}

#[test]
fn report_preserves_baseline_identity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("report should succeed");

    assert_eq!(
        report.baseline_id,
        BASELINE_ID
    );
}

#[test]
fn report_preserves_benchmark_identity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("report should succeed");

    assert_eq!(
        report.benchmark_id,
        BENCHMARK_ID
    );

    assert_eq!(
        report.benchmark_version,
        BENCHMARK_VERSION
    );
}

#[test]
fn report_contains_deterministic_policy_identity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let policy =
        default_policy();

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("report should succeed");

    assert_eq!(
        report.policy_id,
        policy.id()
    );
}

#[test]
fn report_counts_are_consistent() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("first metric should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("second metric should be valid")
            .build()
            .expect("baseline should build");

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(8.0),
        ),
    ];

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("report should succeed");

    assert_eq!(
        report.metrics.len(),
        report.compared_metric_count
    );

    assert_eq!(
        report.improvement_count
            + report.regression_count
            + report.below_threshold_count
            + report.statistically_unresolved_count
            + report.neutral_count,
        report.compared_metric_count
    );
}

#[test]
fn regression_report_is_cloneable_without_mutating_original() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let original =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("report should succeed");

    let cloned =
        original.clone();

    assert_eq!(
        original,
        cloned
    );
}

#[test]
fn baseline_remains_unchanged_after_regression_analysis() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let before =
        baseline.clone();

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let _report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("report should succeed");

    assert_eq!(
        baseline,
        before
    );
}

// =============================================================================
// Already-computed BaselineSetComparison integration
// =============================================================================

#[test]
fn already_computed_baseline_comparison_can_be_evaluated() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let comparison =
        baseline
            .compare_metrics(
                &[candidate],
                &BaselineComparisonPolicy {
                    metric_policy:
                        super::super::analysis::compare::ComparisonPolicy::default(),
                    require_complete_baseline: false,
                    require_complete_candidate: false,
                },
            )
            .expect("baseline comparison should succeed");

    let report =
        evaluate_baseline_comparison(
            &comparison,
            &default_policy(),
        )
        .expect("precomputed comparison should evaluate");

    assert_eq!(
        report.regression_count,
        1
    );

    assert!(report.is_ci_failure());
}

#[test]
fn evaluate_metric_comparison_preserves_authoritative_comparison() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let comparison =
        baseline
            .compare_metric(
                WIDTH_4,
                &candidate.metric,
                &BaselineComparisonPolicy::default(),
            )
            .expect("single baseline comparison should succeed");

    let regression =
        evaluate_metric_comparison(
            &comparison,
            &default_policy(),
        )
        .expect("metric regression should evaluate");

    assert_eq!(
        regression.identity,
        comparison.identity()
    );

    assert_eq!(
        regression.comparison,
        comparison.comparison
    );
}

// =============================================================================
// Policy mode behavior
// =============================================================================

#[test]
fn analysis_policy_can_represent_missing_data_without_failing() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &analysis_policy(),
        )
        .expect("analysis policy should preserve incomplete information");

    assert_eq!(
        report.missing_baseline_count,
        1
    );

    assert!(!report.is_ci_failure());
}

#[test]
fn ci_policy_rejects_missing_data() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let result =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &RegressionPolicy::ci(),
        );

    assert!(matches!(
        result,
        Err(RegressionError::IncompleteBaseline {
            count: 1
        })
    ));
}

// =============================================================================
// Resource limits
// =============================================================================

#[test]
fn candidate_metric_limit_is_enforced_before_analysis() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let mut policy =
        default_policy();

    policy.max_metrics = 1;

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(QV_IMPROVED_VALUE),
        ),
    ];

    let result =
        evaluate_baseline(
            &baseline,
            &candidates,
            &policy,
        );

    assert!(matches!(
        result,
        Err(RegressionError::TooManyMetrics {
            count: 2,
            maximum: 1
        })
    ));
}

#[test]
fn regression_metric_limit_is_bounded_by_global_safety_limit() {
    let mut policy =
        default_policy();

    policy.max_metrics =
        DEFAULT_MAX_REGRESSION_METRICS + 1;

    let result =
        policy.validate();

    assert!(result.is_ok());

    // The public evaluator still imposes the hard global limit on actual
    // candidate collections. This test documents that policy limits and hard
    // resource limits are distinct concepts.
}

#[test]
fn empty_candidate_set_is_representable_for_analysis_policy() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let mut policy =
        analysis_policy();

    policy.require_complete_candidate = false;
    policy.require_complete_baseline = false;

    let report =
        evaluate_baseline(
            &baseline,
            &[],
            &policy,
        )
        .expect("empty candidate set should be representable");

    assert_eq!(
        report.compared_metric_count,
        0
    );

    assert_eq!(
        report.regression_count,
        0
    );

    assert!(!report.has_regression());
}

// =============================================================================
// Severity semantics
// =============================================================================

#[test]
fn no_regression_has_no_severity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert_eq!(
        report.severity,
        RegressionSeverity::None
    );
}

#[test]
fn actual_regression_has_error_severity() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("comparison should succeed");

    assert_eq!(
        report.severity,
        RegressionSeverity::Error
    );

    assert!(
        report.severity.fails_gate()
    );
}

#[test]
fn below_threshold_degradation_is_warning_not_error() {
    let baseline =
        baseline_with_qv(
            100.0,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(99.0),
        );

    let mut policy =
        default_policy();

    policy.absolute_degradation_threshold = 2.0;
    policy.relative_degradation_threshold = 0.05;

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("comparison should succeed");

    assert_eq!(
        report.severity,
        RegressionSeverity::Warning
    );

    assert!(!report.is_ci_failure());
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_reports() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(QV_REGRESSED_VALUE),
        );

    let policy =
        default_policy();

    let first =
        evaluate_baseline(
            &baseline,
            &[candidate.clone()],
            &policy,
        )
        .expect("first report should succeed");

    let second =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &policy,
        )
        .expect("second report should succeed");

    assert_eq!(
        first,
        second
    );
}

#[test]
fn policy_id_is_independent_of_execution_order() {
    let first =
        RegressionPolicy::ci();

    let second =
        RegressionPolicy::ci();

    assert_eq!(
        first.id(),
        second.id()
    );
}

#[test]
fn regression_decision_ids_are_stable() {
    assert_eq!(
        RegressionDecision::Improvement.id(),
        "improvement"
    );

    assert_eq!(
        RegressionDecision::NoRegression.id(),
        "no_regression"
    );

    assert_eq!(
        RegressionDecision::BelowThreshold.id(),
        "below_threshold"
    );

    assert_eq!(
        RegressionDecision::Regression.id(),
        "regression"
    );

    assert_eq!(
        RegressionDecision::StatisticallyUnresolved.id(),
        "statistically_unresolved"
    );

    assert_eq!(
        RegressionDecision::Neutral.id(),
        "neutral"
    );

    assert_eq!(
        RegressionDecision::MissingBaseline.id(),
        "missing_baseline"
    );

    assert_eq!(
        RegressionDecision::MissingCandidate.id(),
        "missing_candidate"
    );
}

#[test]
fn severity_ids_are_stable() {
    assert_eq!(
        RegressionSeverity::None.id(),
        "none"
    );

    assert_eq!(
        RegressionSeverity::Warning.id(),
        "warning"
    );

    assert_eq!(
        RegressionSeverity::Error.id(),
        "error"
    );
}

// =============================================================================
// Regression report semantic helpers
// =============================================================================

#[test]
fn all_improved_is_true_only_when_every_compared_metric_improved() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("first metric should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("second metric should be valid")
            .build()
            .expect("baseline should build");

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(64.0),
        ),
    ];

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("all-improved report should succeed");

    assert!(
        report.all_improved()
    );
}

#[test]
fn all_improved_is_false_when_one_metric_regresses() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("first metric should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                qv_metric(32.0),
            )
            .expect("second metric should be valid")
            .build()
            .expect("baseline should build");

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(8.0),
        ),
    ];

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("mixed report should succeed");

    assert!(
        !report.all_improved()
    );
}

#[test]
fn all_improved_is_false_for_empty_comparison() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let mut policy =
        analysis_policy();

    policy.require_complete_baseline = false;
    policy.require_complete_candidate = false;

    let report =
        evaluate_baseline(
            &baseline,
            &[],
            &policy,
        )
        .expect("empty analysis should succeed");

    assert!(
        !report.all_improved()
    );
}

// =============================================================================
// Baseline identity safety
// =============================================================================

#[test]
fn same_metric_kind_at_different_widths_has_distinct_baseline_identity() {
    let first =
        baseline_metric(
            WIDTH_4,
            qv_metric(16.0),
        );

    let second =
        baseline_metric(
            WIDTH_8,
            qv_metric(16.0),
        );

    assert_ne!(
        first.identity(),
        second.identity()
    );
}

#[test]
fn same_dimensions_with_different_metric_kind_has_distinct_identity() {
    let first =
        baseline_metric(
            WIDTH_4,
            qv_metric(16.0),
        );

    let second =
        baseline_metric(
            WIDTH_4,
            error_rate_metric(0.01),
        );

    assert_ne!(
        first.identity(),
        second.identity()
    );
}

#[test]
fn same_dimensions_with_different_metric_unit_has_distinct_identity() {
    let first =
        baseline_metric(
            WIDTH_4,
            metric(
                MetricKind::Runtime,
                MetricUnit::Milliseconds,
                100.0,
            ),
        );

    let second =
        baseline_metric(
            WIDTH_4,
            metric(
                MetricKind::Runtime,
                MetricUnit::Seconds,
                100.0,
            ),
        );

    assert_ne!(
        first.identity(),
        second.identity()
    );
}

// =============================================================================
// Large-but-valid practical regression set
// =============================================================================

#[test]
fn many_scoped_metrics_can_be_compared_without_protocol_specific_logic() {
    let mut builder =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            );

    let mut candidates =
        Vec::new();

    for index in 0usize..32 {
        let dimension =
            index.to_string();

        let dimensions =
            vec![("case", dimension.as_str())];

        builder = builder
            .add_metric(
                dimensions.clone(),
                qv_metric(
                    100.0
                        + index as f64,
                ),
            )
            .expect("generated baseline metric should be valid");

        candidates.push(
            candidate_metric(
                &dimensions,
                qv_metric(
                    100.0
                        + index as f64
                        + 10.0,
                ),
            ),
        );
    }

    let baseline =
        builder
            .build()
            .expect("large practical baseline should build");

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &default_policy(),
        )
        .expect("large practical report should succeed");

    assert_eq!(
        report.compared_metric_count,
        32
    );

    assert_eq!(
        report.improvement_count,
        32
    );

    assert_eq!(
        report.regression_count,
        0
    );

    assert!(
        !report.is_ci_failure()
    );
}

// =============================================================================
// Different benchmark versions
// =============================================================================

#[test]
fn baseline_benchmark_version_is_preserved_in_report() {
    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                "2.0.0",
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("versioned baseline metric should be valid")
            .build()
            .expect("versioned baseline should build");

    let candidate =
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("versioned comparison should succeed");

    assert_eq!(
        report.benchmark_version,
        "2.0.0"
    );
}

// =============================================================================
// Metric-family coverage
// =============================================================================

#[test]
fn quantum_volume_metric_uses_higher_is_better_semantics() {
    let baseline =
        qv_metric(16.0);

    let candidate =
        qv_metric(32.0);

    let baseline_entry =
        baseline_metric(
            WIDTH_4,
            baseline.clone(),
        );

    let candidate_entry =
        candidate_metric(
            WIDTH_4,
            candidate.clone(),
        );

    let reference =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                baseline,
            )
            .expect("QV metric should be valid")
            .build()
            .expect("baseline should build");

    let report =
        evaluate_baseline(
            &reference,
            &[candidate_entry],
            &default_policy(),
        )
        .expect("QV comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );

    // Keep the helper alive in this fixture to make it explicit that a
    // BaselineMetric is the transport object consumed by the regression API.
    assert_eq!(
        baseline_entry.identity(),
        reference.metrics()[0].identity()
    );
}

#[test]
fn error_rate_metric_uses_lower_is_better_semantics() {
    let baseline =
        baseline_with_error_rate(
            ERROR_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            error_rate_metric(
                ERROR_IMPROVED_VALUE,
            ),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("error-rate comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );
}

#[test]
fn runtime_metric_uses_lower_is_better_semantics() {
    let baseline =
        baseline_with_runtime(
            RUNTIME_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_4,
            runtime_metric(
                RUNTIME_IMPROVED_VALUE,
            ),
        );

    let report =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect("runtime comparison should succeed");

    assert_eq!(
        report.metrics[0].decision,
        RegressionDecision::Improvement
    );
}

// =============================================================================
// Public constants / contract checks
// =============================================================================

#[test]
fn regression_schema_version_is_positive() {
    assert!(
        REGRESSION_SCHEMA_VERSION > 0
    );
}

#[test]
fn global_regression_metric_limit_is_positive() {
    assert!(
        DEFAULT_MAX_REGRESSION_METRICS > 0
    );
}

#[test]
fn maximum_threshold_is_nonzero() {
    assert!(
        MAX_REGRESSION_THRESHOLD > 0.0
    );
}

// =============================================================================
// RegressionError semantics
// =============================================================================

#[test]
fn too_many_metrics_error_contains_actual_and_allowed_counts() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let mut policy =
        default_policy();

    policy.max_metrics = 1;

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            qv_metric(64.0),
        ),
    ];

    let error =
        evaluate_baseline(
            &baseline,
            &candidates,
            &policy,
        )
        .expect_err(
            "metric limit should be enforced",
        );

    match error {
        RegressionError::TooManyMetrics {
            count,
            maximum,
        } => {
            assert_eq!(
                count,
                2
            );

            assert_eq!(
                maximum,
                1
            );
        }

        other => {
            panic!(
                "unexpected regression error: {:?}",
                other
            );
        }
    }
}

#[test]
fn incomplete_baseline_error_contains_missing_count() {
    let baseline =
        baseline_with_qv(
            QV_BASELINE_VALUE,
            WIDTH_4,
        );

    let candidate =
        candidate_metric(
            WIDTH_8,
            qv_metric(QV_IMPROVED_VALUE),
        );

    let error =
        evaluate_baseline(
            &baseline,
            &[candidate],
            &default_policy(),
        )
        .expect_err(
            "missing baseline must fail strict policy",
        );

    match error {
        RegressionError::IncompleteBaseline {
            count,
        } => {
            assert_eq!(
                count,
                1
            );
        }

        other => {
            panic!(
                "unexpected regression error: {:?}",
                other
            );
        }
    }
}

// =============================================================================
// End-to-end contract test
// =============================================================================

#[test]
fn complete_regression_pipeline_behaves_as_documented() {
    // -------------------------------------------------------------------------
    // Baseline
    // -------------------------------------------------------------------------

    let baseline =
        Baseline::builder(BASELINE_ID)
            .benchmark(
                BENCHMARK_ID,
                BENCHMARK_VERSION,
            )
            .add_metric(
                WIDTH_4.to_vec(),
                qv_metric(16.0),
            )
            .expect("QV baseline should be valid")
            .add_metric(
                WIDTH_8.to_vec(),
                error_rate_metric(0.01),
            )
            .expect("error-rate baseline should be valid")
            .add_metric(
                WIDTH_16.to_vec(),
                runtime_metric(100.0),
            )
            .expect("runtime baseline should be valid")
            .build()
            .expect("complete baseline should build");

    // -------------------------------------------------------------------------
    // Candidate
    // -------------------------------------------------------------------------

    let candidates = vec![
        candidate_metric(
            WIDTH_4,
            qv_metric(32.0),
        ),
        candidate_metric(
            WIDTH_8,
            error_rate_metric(0.02),
        ),
        candidate_metric(
            WIDTH_16,
            runtime_metric(80.0),
        ),
    ];

    // -------------------------------------------------------------------------
    // Regression policy
    // -------------------------------------------------------------------------

    let policy =
        RegressionPolicy::ci();

    // -------------------------------------------------------------------------
    // Analysis
    // -------------------------------------------------------------------------

    let report =
        evaluate_baseline(
            &baseline,
            &candidates,
            &policy,
        )
        .expect(
            "complete regression pipeline should succeed",
        );

    // -------------------------------------------------------------------------
    // Expected result
    // -------------------------------------------------------------------------

    assert_eq!(
        report.schema_version,
        REGRESSION_SCHEMA_VERSION
    );

    assert_eq!(
        report.baseline_id,
        BASELINE_ID
    );

    assert_eq!(
        report.benchmark_id,
        BENCHMARK_ID
    );

    assert_eq!(
        report.benchmark_version,
        BENCHMARK_VERSION
    );

    assert_eq!(
        report.compared_metric_count,
        3
    );

    assert_eq!(
        report.improvement_count,
        2
    );

    assert_eq!(
        report.regression_count,
        1
    );

    assert_eq!(
        report.missing_baseline_count,
        0
    );

    assert_eq!(
        report.missing_candidate_count,
        0
    );

    assert!(
        report.has_regression()
    );

    assert!(
        report.is_ci_failure()
    );

    assert_eq!(
        report.severity,
        RegressionSeverity::Error
    );

    let regressions =
        report.regressions().collect::<Vec<_>>();

    assert_eq!(
        regressions.len(),
        1
    );

    assert_eq!(
        regressions[0].comparison.metric_kind(),
        MetricKind::ErrorRate
    );
}