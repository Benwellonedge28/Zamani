//! Zamani Quantum Benchmarking — Regression Analysis
//!
//! Production-grade historical benchmark regression detection.
//!
//! # Purpose
//!
//! This module answers:
//!
//! > Has a candidate benchmark result materially regressed relative to an
//! > established baseline under an explicit, reproducible regression policy?
//!
//! This module is intentionally a policy/decision layer.
//!
//! It does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - select hardware;
//! - communicate with quantum providers;
//! - compile or route circuits;
//! - schedule circuits;
//! - calculate protocol-specific metrics;
//! - fit randomized-benchmarking curves;
//! - calculate confidence intervals;
//! - replace `analysis::compare`;
//! - replace `analysis::baseline`;
//! - mutate historical baselines;
//! - access clocks;
//! - access filesystem state;
//! - access network state;
//! - access environment variables;
//! - print diagnostics.
//!
//! Those responsibilities belong to other layers.
//!
//! # Architectural position
//!
//! ```text
//! Benchmark execution
//!        │
//!        ▼
//! core::metric
//!        │
//!        ▼
//! analysis::compare
//!        │
//!        ▼
//! analysis::baseline
//!        │
//!        ▼
//! analysis::regression
//!        │
//!        ├──────────────► reporting
//!        ├──────────────► CI
//!        ├──────────────► registry
//!        └──────────────► Zamani stdlib
//! ```
//!
//! The authoritative dependency direction is:
//!
//! ```text
//! core::metric
//!      ↓
//! analysis::compare
//!      ↓
//! analysis::baseline
//!      ↓
//! analysis::regression
//! ```
//!
//! This module must never introduce a dependency in the opposite direction.
//!
//! # Critical scientific distinction
//!
//! A numerical change is not automatically a regression.
//!
//! `analysis::compare` already distinguishes:
//!
//! - improvement;
//! - regression;
//! - no material change;
//! - neutral;
//! - numerical difference without a statistical conclusion.
//!
//! This module converts those authoritative comparison results into an
//! explicit regression-policy decision.
//!
//! Therefore this module does NOT reimplement:
//!
//! ```text
//! candidate - baseline
//! relative change
//! ratio
//! confidence interval relationship
//! uncertainty separation
//! metric direction
//! ```
//!
//! Those calculations remain owned by `analysis::compare`.
//!
//! # Regression policy
//!
//! A production regression detector must make its policy explicit.
//!
//! The policy supports:
//!
//! - absolute degradation threshold;
//! - relative degradation threshold;
//! - optional minimum percentage threshold;
//! - optional requirement for a complete baseline;
//! - optional requirement for a complete candidate;
//! - treatment of statistically unresolved changes;
//! - treatment of neutral metrics;
//! - treatment of missing metrics;
//! - maximum number of metrics;
//! - deterministic policy identity.
//!
//! This avoids hard-coding a universal "5% regression" rule.
//!
//! A 5% change can be insignificant for one metric and catastrophic for
//! another.
//!
//! # Optimization direction
//!
//! The underlying `MetricComparison` owns the authoritative metric direction.
//!
//! ```text
//! HigherIsBetter
//!     candidate < baseline => degradation
//!
//! LowerIsBetter
//!     candidate > baseline => degradation
//!
//! Neutral
//!     no generic regression conclusion
//! ```
//!
//! `MetricKind::default_direction()` is therefore NOT reimplemented here.
//!
//! # Two-threshold model
//!
//! A regression may be required to exceed BOTH:
//!
//! ```text
//! absolute degradation >= absolute threshold
//! AND
//! relative degradation >= relative threshold
//! ```
//!
//! or either threshold may independently trigger a regression depending on
//! policy.
//!
//! The default policy intentionally uses OR semantics because a tiny baseline
//! can make a large relative regression scientifically important even when the
//! absolute difference is small, while a large baseline can make a meaningful
//! absolute regression have a modest relative percentage.
//!
//! # Statistical caution
//!
//! This module does not turn confidence-interval overlap into a hypothesis
//! test.
//!
//! If `analysis::compare` reports:
//!
//! ```text
//! DifferenceWithoutStatisticalConclusion
//! ```
//!
//! this module can either:
//!
//! - report it as unresolved; or
//! - fail the regression gate conservatively,
//!
//! according to explicit policy.
//!
//! Formal statistical testing remains the responsibility of
//! `statistics::hypothesis`.
//!
//! # Missing metrics
//!
//! Missing metrics are never silently treated as zero.
//!
//! A candidate missing a baseline metric can mean:
//!
//! - the benchmark changed;
//! - the result is incomplete;
//! - a metric was intentionally removed;
//! - execution failed;
//! - the candidate was generated with incompatible configuration.
//!
//! The policy therefore controls whether missing metrics are fatal.
//!
//! # Determinism
//!
//! Regression analysis is deterministic for deterministic inputs.
//!
//! It does not access:
//!
//! - clocks;
//! - random generators;
//! - filesystem;
//! - network;
//! - environment;
//! - process-global state.
//!
//! # Resource safety
//!
//! This module:
//!
//! - rejects non-finite thresholds;
//! - rejects negative thresholds;
//! - bounds candidate metric count;
//! - never recursively processes input;
//! - never performs unbounded allocation;
//! - never mutates the baseline;
//! - never panics for ordinary invalid input;
//! - preserves all per-metric decisions.
//!
//! # Integration contract
//!
//! This file is intentionally complete without requiring future edits when
//! reporting, CI, registry, or the Zamani standard library are added.
//!
//! Required existing modules:
//!
//! ```text
//! analysis::baseline
//! analysis::compare
//! core::metric
//! ```
//!
//! Existing baseline infrastructure already provides:
//!
//! ```text
//! Baseline
//! BaselineMetric
//! BaselineComparison
//! BaselineSetComparison
//! BaselineComparisonPolicy
//! ```
//!
//! and existing comparison infrastructure provides:
//!
//! ```text
//! MetricComparison
//! MetricSetComparison
//! ComparisonConclusion
//! MetricDirection
//! ```
//!
//! The baseline layer explicitly defines regression as a downstream consumer
//! of `BaselineComparison`. It also preserves scoped dimensions so that,
//! for example, QV at width 8 cannot accidentally be compared with QV at
//! width 16. 
//!
//! # Future integration
//!
//! ```text
//! BenchmarkResult
//!       │
//!       ▼
//! extract BaselineMetric entries
//!       │
//!       ▼
//! Baseline::compare_metrics()
//!       │
//!       ▼
//! BaselineSetComparison
//!       │
//!       ▼
//! RegressionAnalyzer::evaluate()
//!       │
//!       ▼
//! RegressionReport
//!       │
//!       ├── reporting::*
//!       ├── CI gate
//!       ├── registry
//!       └── Zamani stdlib
//! ```
//!
//! No protocol implementation should import this module directly unless it is
//! explicitly implementing historical/regression analysis.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features.
//! No unsafe code.
//! No additional crate dependency.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! Main types:
//!
//! - `RegressionPolicy`
//! - `RegressionDecision`
//! - `RegressionSeverity`
//! - `MetricRegression`
//! - `RegressionReport`
//! - `RegressionError`
//!
//! Main functions:
//!
//! - `evaluate_baseline_comparison()`
//! - `evaluate_baseline()`
//!
//! Main methods:
//!
//! - `RegressionPolicy::validate()`
//! - `RegressionPolicy::default()`
//! - `RegressionReport::has_regression()`
//! - `RegressionReport::is_ci_failure()`
//! - `RegressionReport::regressions()`
//!
//! -----------------------------------------------------------------------------
//! Example
//! -----------------------------------------------------------------------------
//!
//! ```rust
//! use crate::quantum::benchmarking::analysis::baseline::{
//!     Baseline,
//!     BaselineMetric,
//! };
//! use crate::quantum::benchmarking::analysis::regression::{
//!     evaluate_baseline,
//!     RegressionPolicy,
//! };
//!
//! // Construct a Baseline and candidate metrics using the authoritative
//! // baseline API.
//!
//! let policy = RegressionPolicy::default();
//!
//! // `evaluate_baseline` performs:
//! //
//! // candidate metrics
//! //       ↓
//! // Baseline::compare_metrics()
//! //       ↓
//! // regression policy
//! //       ↓
//! // RegressionReport
//! ```
//!
//! -----------------------------------------------------------------------------
//! Design rule
//! -----------------------------------------------------------------------------
//!
//! `analysis::compare` answers:
//!
//!     "What changed?"
//!
//! `analysis::baseline` answers:
//!
//!     "Which historical/reference value corresponds to this candidate?"
//!
//! `analysis::regression` answers:
//!
//!     "Does that change violate the configured regression policy?"
//!
//! Keeping those questions separate prevents the benchmarking architecture
//! from accumulating competing definitions of regression.
//!

#![deny(unsafe_code)]

use std::fmt;

use crate::quantum::benchmarking::analysis::baseline::{
    Baseline,
    BaselineComparison,
    BaselineComparisonPolicy,
    BaselineError,
    BaselineMetric,
    BaselineSetComparison,
};

use crate::quantum::benchmarking::analysis::compare::{
    ComparisonConclusion,
    MetricComparison,
};

use crate::quantum::benchmarking::core::metric::MetricDirection;

// =============================================================================
// Public constants
// =============================================================================

/// Semantic version of the regression-analysis contract.
pub const REGRESSION_SCHEMA_VERSION: u32 = 1;

/// Default maximum number of candidate metrics accepted by one regression
/// analysis operation.
///
/// This is deliberately bounded independently of baseline storage limits.
pub const DEFAULT_MAX_REGRESSION_METRICS: usize = 100_000;

/// Default absolute degradation threshold.
///
/// This value is deliberately zero because the default regression policy
/// should not silently ignore a real regression merely because its numerical
/// magnitude is small.
///
/// The relative threshold provides the practical noise gate.
pub const DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD: f64 = 0.0;

/// Default relative degradation threshold.
///
/// A 5% degradation is considered materially regressive by the default CI
/// policy.
///
/// This is a policy default, not a scientific universal.
pub const DEFAULT_RELATIVE_DEGRADATION_THRESHOLD: f64 = 0.05;

/// Default minimum absolute percentage degradation.
///
/// This is expressed as a fraction, not a percentage integer.
///
/// `0.05` means 5%.
pub const DEFAULT_MIN_RELATIVE_DEGRADATION: f64 = 0.05;

/// Default policy requires a complete baseline for CI gating.
///
/// Missing baseline entries are not silently accepted.
pub const DEFAULT_REQUIRE_COMPLETE_BASELINE: bool = true;

/// Default policy requires a complete candidate for CI gating.
pub const DEFAULT_REQUIRE_COMPLETE_CANDIDATE: bool = true;

/// Default treatment of statistically unresolved changes.
///
/// Such changes should not be automatically declared regressions, but they
/// should cause a CI gate to fail under the conservative default.
pub const DEFAULT_FAIL_ON_STATISTICALLY_UNRESOLVED: bool = true;

/// Default treatment of neutral metrics.
///
/// Neutral metrics do not constitute regressions.
pub const DEFAULT_FAIL_ON_NEUTRAL: bool = false;

/// Maximum supported threshold.
///
/// This prevents malformed configuration from declaring a 1000% "threshold"
/// or otherwise disabling regression detection accidentally.
pub const MAX_REGRESSION_THRESHOLD: f64 = 1.0;

/// Maximum accepted metric count for a report.
pub const MAX_REGRESSION_REPORT_METRICS: usize =
    DEFAULT_MAX_REGRESSION_METRICS;

// =============================================================================
// Regression severity
// =============================================================================

/// Severity assigned to a detected regression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegressionSeverity {
    /// Informational; no regression was detected.
    None,

    /// A regression was detected but remains below the strongest configured
    /// escalation level.
    Warning,

    /// A regression violates the configured CI/regression gate.
    Error,
}

impl RegressionSeverity {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Returns whether this severity fails a CI gate.
    pub const fn fails_gate(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl fmt::Display for RegressionSeverity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

// =============================================================================
// Regression decision
// =============================================================================

/// Final policy decision for one metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegressionDecision {
    /// Candidate improved.
    Improvement,

    /// Candidate changed but not enough to constitute a regression.
    NoRegression,

    /// Candidate is worse numerically, but does not exceed the configured
    /// regression threshold.
    BelowThreshold,

    /// Candidate violates the configured regression threshold.
    Regression,

    /// Numerical difference exists but the comparison did not establish a
    /// generic statistical conclusion.
    StatisticallyUnresolved,

    /// Metric cannot be interpreted as a directional performance metric.
    Neutral,

    /// Metric could not be compared because the corresponding baseline entry
    /// is absent.
    MissingBaseline,

    /// Baseline exists but candidate metric is absent.
    MissingCandidate,
}

impl RegressionDecision {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Improvement => "improvement",
            Self::NoRegression => "no_regression",
            Self::BelowThreshold => "below_threshold",
            Self::Regression => "regression",
            Self::StatisticallyUnresolved => "statistically_unresolved",
            Self::Neutral => "neutral",
            Self::MissingBaseline => "missing_baseline",
            Self::MissingCandidate => "missing_candidate",
        }
    }

    /// Returns whether this decision is an actual regression.
    pub const fn is_regression(self) -> bool {
        matches!(self, Self::Regression)
    }

    /// Returns whether this decision should be treated as unresolved.
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::StatisticallyUnresolved)
    }

    /// Returns whether this decision represents an improvement.
    pub const fn is_improvement(self) -> bool {
        matches!(self, Self::Improvement)
    }
}

impl fmt::Display for RegressionDecision {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

// =============================================================================
// Policy
// =============================================================================

/// Policy controlling production regression detection.
///
/// The policy is immutable after construction and contains no process-global
/// state.
///
/// Thresholds are fractions:
///
/// ```text
/// 0.01 = 1%
/// 0.05 = 5%
/// 0.10 = 10%
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegressionPolicy {
    /// Absolute degradation threshold in the metric's native unit.
    ///
    /// This is compared against the direction-normalized degradation
    /// magnitude.
    pub absolute_degradation_threshold: f64,

    /// Relative degradation threshold.
    ///
    /// `0.05` means 5%.
    pub relative_degradation_threshold: f64,

    /// Whether exceeding either the absolute OR relative threshold is
    /// sufficient to declare a regression.
    ///
    /// If false, BOTH thresholds must be exceeded.
    pub threshold_is_either: bool,

    /// Whether candidate and baseline must contain identical metric scopes.
    pub require_complete_baseline: bool,

    /// Whether candidate and baseline must contain identical metric scopes.
    pub require_complete_candidate: bool,

    /// Whether a statistically unresolved difference should fail the CI gate.
    pub fail_on_statistically_unresolved: bool,

    /// Whether a neutral metric should fail the CI gate.
    ///
    /// This is normally false because neutral metrics are not directional
    /// performance metrics.
    pub fail_on_neutral: bool,

    /// Maximum number of candidate metrics accepted.
    pub max_metrics: usize,
}

impl Default for RegressionPolicy {
    fn default() -> Self {
        Self {
            absolute_degradation_threshold:
                DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD,

            relative_degradation_threshold:
                DEFAULT_RELATIVE_DEGRADATION_THRESHOLD,

            threshold_is_either: true,

            require_complete_baseline:
                DEFAULT_REQUIRE_COMPLETE_BASELINE,

            require_complete_candidate:
                DEFAULT_REQUIRE_COMPLETE_CANDIDATE,

            fail_on_statistically_unresolved:
                DEFAULT_FAIL_ON_STATISTICALLY_UNRESOLVED,

            fail_on_neutral: DEFAULT_FAIL_ON_NEUTRAL,

            max_metrics: DEFAULT_MAX_REGRESSION_METRICS,
        }
    }
}

impl RegressionPolicy {
    /// Creates a conservative policy suitable for CI.
    #[must_use]
    pub const fn ci() -> Self {
        Self {
            absolute_degradation_threshold:
                DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD,

            relative_degradation_threshold:
                DEFAULT_RELATIVE_DEGRADATION_THRESHOLD,

            threshold_is_either: true,

            require_complete_baseline: true,

            require_complete_candidate: true,

            fail_on_statistically_unresolved: true,

            fail_on_neutral: false,

            max_metrics: DEFAULT_MAX_REGRESSION_METRICS,
        }
    }

    /// Creates a permissive analysis policy.
    ///
    /// This policy still reports regressions but does not make missing metrics
    /// or unresolved statistical results automatically fail the gate.
    #[must_use]
    pub const fn analysis() -> Self {
        Self {
            absolute_degradation_threshold:
                DEFAULT_ABSOLUTE_DEGRADATION_THRESHOLD,

            relative_degradation_threshold:
                DEFAULT_RELATIVE_DEGRADATION_THRESHOLD,

            threshold_is_either: true,

            require_complete_baseline: false,

            require_complete_candidate: false,

            fail_on_statistically_unresolved: false,

            fail_on_neutral: false,

            max_metrics: DEFAULT_MAX_REGRESSION_METRICS,
        }
    }

    /// Validates this policy.
    pub fn validate(&self) -> Result<(), RegressionError> {
        validate_threshold(
            self.absolute_degradation_threshold,
            "absolute_degradation_threshold",
        )?;

        validate_threshold(
            self.relative_degradation_threshold,
            "relative_degradation_threshold",
        )?;

        if self.max_metrics == 0 {
            return Err(
                RegressionError::InvalidMetricLimit {
                    maximum: self.max_metrics,
                },
            );
        }

        Ok(())
    }

    /// Returns a deterministic policy identifier.
    ///
    /// This string is suitable for provenance/reporting. It intentionally
    /// contains only policy semantics and no timestamps or process state.
    #[must_use]
    pub fn id(&self) -> String {
        format!(
            "regression-v{}-abs={:.17e}-rel={:.17e}-mode={}-complete-baseline={}-complete-candidate={}-unresolved={}-neutral={}-max={}",
            REGRESSION_SCHEMA_VERSION,
            self.absolute_degradation_threshold,
            self.relative_degradation_threshold,
            if self.threshold_is_either {
                "either"
            } else {
                "both"
            },
            self.require_complete_baseline,
            self.require_complete_candidate,
            self.fail_on_statistically_unresolved,
            self.fail_on_neutral,
            self.max_metrics,
        )
    }
}

// =============================================================================
// Per-metric regression
// =============================================================================

/// Complete regression decision for one scoped benchmark metric.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricRegression {
    /// Regression-analysis schema version.
    pub schema_version: u32,

    /// Scoped baseline metric identity.
    pub identity: String,

    /// Authoritative comparison result.
    pub comparison: MetricComparison,

    /// Policy decision.
    pub decision: RegressionDecision,

    /// Severity.
    pub severity: RegressionSeverity,

    /// Direction-normalized degradation.
///
/// For a higher-is-better metric:
///
/// ```text
/// baseline - candidate
/// ```
///
/// For a lower-is-better metric:
///
/// ```text
/// candidate - baseline
/// ```
///
/// Therefore positive values always mean degradation.
    pub degradation: f64,

    /// Direction-normalized relative degradation.
///
/// Positive values represent degradation.
///
/// `None` means the underlying relative comparison is undefined.
    pub relative_degradation: Option<f64>,

    /// Whether the absolute threshold was exceeded.
    pub exceeded_absolute_threshold: bool,

    /// Whether the relative threshold was exceeded.
    pub exceeded_relative_threshold: bool,
}

impl MetricRegression {
    /// Returns whether this metric is a policy regression.
    #[must_use]
    pub const fn is_regression(&self) -> bool {
        self.decision.is_regression()
    }

    /// Returns whether this metric fails the configured CI gate.
    #[must_use]
    pub const fn fails_gate(&self) -> bool {
        self.severity.fails_gate()
    }
}

// =============================================================================
// Regression report
// =============================================================================

/// Complete regression-analysis report.
///
/// This is an immutable analysis product. It does not mutate the supplied
/// baseline or candidate data.
#[derive(Debug, Clone, PartialEq)]
pub struct RegressionReport {
    /// Regression schema version.
    pub schema_version: u32,

    /// Stable policy identity.
    pub policy_id: String,

    /// Baseline identity.
    pub baseline_id: String,

    /// Benchmark identity.
    pub benchmark_id: String,

    /// Benchmark version.
    pub benchmark_version: String,

    /// Number of candidate metrics supplied.
    pub candidate_metric_count: usize,

    /// Number of metrics compared.
    pub compared_metric_count: usize,

    /// Per-metric decisions.
    pub metrics: Vec<MetricRegression>,

    /// Candidate metrics missing from the baseline.
    pub missing_baseline: Vec<String>,

    /// Baseline metrics missing from the candidate.
    pub missing_candidate: Vec<String>,

    /// Number of detected regressions.
    pub regression_count: usize,

    /// Number of improvements.
    pub improvement_count: usize,

    /// Number of below-threshold degradations.
    pub below_threshold_count: usize,

    /// Number of statistically unresolved changes.
    pub statistically_unresolved_count: usize,

    /// Number of neutral metrics.
    pub neutral_count: usize,

    /// Number of missing-baseline entries.
    pub missing_baseline_count: usize,

    /// Number of missing-candidate entries.
    pub missing_candidate_count: usize,

    /// Overall severity.
    pub severity: RegressionSeverity,

    /// Whether the report fails the configured CI gate.
    pub ci_failure: bool,
}

impl RegressionReport {
    /// Returns whether at least one metric regressed.
    #[must_use]
    pub const fn has_regression(&self) -> bool {
        self.regression_count > 0
    }

    /// Returns whether the report fails the configured CI gate.
    #[must_use]
    pub const fn is_ci_failure(&self) -> bool {
        self.ci_failure
    }

    /// Returns whether the report contains unresolved statistical changes.
    #[must_use]
    pub const fn has_statistically_unresolved(&self) -> bool {
        self.statistically_unresolved_count > 0
    }

    /// Returns whether all compared metrics improved.
    #[must_use]
    pub fn all_improved(&self) -> bool {
        self.compared_metric_count > 0
            && self.regression_count == 0
            && self.below_threshold_count == 0
            && self.statistically_unresolved_count == 0
            && self.neutral_count == 0
            && self.improvement_count == self.compared_metric_count
            && self.missing_baseline_count == 0
            && self.missing_candidate_count == 0
    }

    /// Returns an iterator over actual regressions.
    pub fn regressions(
        &self,
    ) -> impl Iterator<Item = &MetricRegression> {
        self.metrics
            .iter()
            .filter(|metric| metric.is_regression())
    }

    /// Returns an iterator over metrics that fail the CI gate.
    pub fn gate_failures(
        &self,
    ) -> impl Iterator<Item = &MetricRegression> {
        self.metrics
            .iter()
            .filter(|metric| metric.fails_gate())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by regression analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum RegressionError {
    /// Regression policy is invalid.
    InvalidThreshold {
        /// Policy field.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// Maximum metric count is zero.
    InvalidMetricLimit {
        /// Invalid maximum.
        maximum: usize,
    },

    /// Too many candidate metrics were supplied.
    TooManyMetrics {
        /// Supplied count.
        count: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// The baseline contains no matching candidate metric where completeness
    /// was required.
    IncompleteBaseline {
        /// Number of missing baseline entries.
        count: usize,
    },

    /// Candidate is missing baseline entries where completeness was required.
    IncompleteCandidate {
        /// Number of missing candidate entries.
        count: usize,
    },

    /// A comparison could not be evaluated.
    Baseline {
        /// Underlying baseline error.
        source: BaselineError,
    },

    /// A comparison object contains an invalid degradation value.
    NonFiniteDegradation {
        /// Metric identity.
        identity: String,

        /// Invalid value.
        value: f64,
    },

    /// A relative degradation is non-finite.
    NonFiniteRelativeDegradation {
        /// Metric identity.
        identity: String,

        /// Invalid value.
        value: f64,
    },
}

impl fmt::Display for RegressionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidThreshold { field, value } => {
                write!(
                    formatter,
                    "invalid regression threshold `{}`: {}",
                    field,
                    value
                )
            }

            Self::InvalidMetricLimit { maximum } => {
                write!(
                    formatter,
                    "regression metric limit must be greater than zero; got {}",
                    maximum
                )
            }

            Self::TooManyMetrics {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "regression analysis received {} metrics; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::IncompleteBaseline { count } => {
                write!(
                    formatter,
                    "candidate contains {} metric scope(s) absent from the baseline",
                    count
                )
            }

            Self::IncompleteCandidate { count } => {
                write!(
                    formatter,
                    "candidate is missing {} baseline metric scope(s)",
                    count
                )
            }

            Self::Baseline { source } => {
                write!(
                    formatter,
                    "baseline comparison failed: {}",
                    source
                )
            }

            Self::NonFiniteDegradation {
                identity,
                value,
            } => {
                write!(
                    formatter,
                    "metric `{}` produced non-finite degradation {}",
                    identity,
                    value
                )
            }

            Self::NonFiniteRelativeDegradation {
                identity,
                value,
            } => {
                write!(
                    formatter,
                    "metric `{}` produced non-finite relative degradation {}",
                    identity,
                    value
                )
            }
        }
    }
}

impl std::error::Error for RegressionError {}

impl From<BaselineError> for RegressionError {
    fn from(error: BaselineError) -> Self {
        Self::Baseline { source: error }
    }
}

// =============================================================================
// Public API — complete baseline evaluation
// =============================================================================

/// Evaluates candidate metrics against a historical baseline.
///
/// This is the primary integration point for CI, reporting, and future Zamani
/// standard-library APIs.
///
/// The function performs:
///
/// ```text
/// candidate metrics
///       │
///       ▼
/// Baseline::compare_metrics()
///       │
///       ▼
/// scoped MetricComparison values
///       │
///       ▼
/// regression policy
///       │
///       ▼
/// RegressionReport
/// ```
///
/// The baseline itself is never modified.
///
/// # Completeness
///
/// The underlying baseline API intentionally reports missing entries rather
/// than silently dropping them. This function applies the regression policy's
/// completeness requirements on top of that comparison result.
///
/// # Errors
///
/// An incomplete candidate/baseline is an error only when explicitly required
/// by policy.
///
/// Statistical unresolved states are not returned as Rust errors because they
/// are scientifically meaningful analysis outcomes. They remain inside the
/// `RegressionReport` and can optionally fail the CI gate.
pub fn evaluate_baseline(
    baseline: &Baseline,
    candidates: &[BaselineMetric],
    policy: &RegressionPolicy,
) -> Result<RegressionReport, RegressionError> {
    policy.validate()?;

    if candidates.len() > policy.max_metrics {
        return Err(RegressionError::TooManyMetrics {
            count: candidates.len(),
            maximum: policy.max_metrics,
        });
    }

    if candidates.len() > DEFAULT_MAX_REGRESSION_METRICS {
        return Err(RegressionError::TooManyMetrics {
            count: candidates.len(),
            maximum: DEFAULT_MAX_REGRESSION_METRICS,
        });
    }

    let baseline_policy = BaselineComparisonPolicy {
        metric_policy:
            crate::quantum::benchmarking::analysis::compare::ComparisonPolicy::default(),

        // Completeness is deliberately enforced below because the regression
        // policy owns the CI semantics.
        require_complete_baseline: false,
        require_complete_candidate: false,
    };

    let comparison = baseline
        .compare_metrics(
            candidates,
            &baseline_policy,
        )
        .map_err(RegressionError::Baseline)?;

    evaluate_baseline_comparison(
        &comparison,
        policy,
    )
}

// =============================================================================
// Public API — already-computed baseline comparison
// =============================================================================

/// Converts an already-computed `BaselineSetComparison` into a regression
/// report.
///
/// This function is useful when another analysis layer has already performed
/// baseline matching and wants to avoid repeating it.
///
/// It is therefore the preferred integration point for:
///
/// - cached baseline comparisons;
/// - reporting;
/// - benchmark dashboards;
/// - registry analysis;
/// - CI pipelines that persist comparison artifacts.
pub fn evaluate_baseline_comparison(
    comparison: &BaselineSetComparison,
    policy: &RegressionPolicy,
) -> Result<RegressionReport, RegressionError> {
    policy.validate()?;

    if comparison.comparisons.len()
        > policy.max_metrics
    {
        return Err(RegressionError::TooManyMetrics {
            count: comparison.comparisons.len(),
            maximum: policy.max_metrics,
        });
    }

    if policy.require_complete_baseline
        && !comparison.missing_baseline.is_empty()
    {
        return Err(
            RegressionError::IncompleteBaseline {
                count: comparison.missing_baseline.len(),
            },
        );
    }

    if policy.require_complete_candidate
        && !comparison.missing_candidate.is_empty()
    {
        return Err(
            RegressionError::IncompleteCandidate {
                count: comparison.missing_candidate.len(),
            },
        );
    }

    let mut metrics = Vec::with_capacity(
        comparison.comparisons.len(),
    );

    for item in &comparison.comparisons {
        metrics.push(
            evaluate_metric_comparison(
                item,
                policy,
            )?,
        );
    }

    let regression_count = metrics
        .iter()
        .filter(|item| {
            item.decision == RegressionDecision::Regression
        })
        .count();

    let improvement_count = metrics
        .iter()
        .filter(|item| {
            item.decision == RegressionDecision::Improvement
        })
        .count();

    let below_threshold_count = metrics
        .iter()
        .filter(|item| {
            item.decision
                == RegressionDecision::BelowThreshold
        })
        .count();

    let statistically_unresolved_count = metrics
        .iter()
        .filter(|item| {
            item.decision
                == RegressionDecision::StatisticallyUnresolved
        })
        .count();

    let neutral_count = metrics
        .iter()
        .filter(|item| {
            item.decision == RegressionDecision::Neutral
        })
        .count();

    let missing_baseline_count =
        comparison.missing_baseline.len();

    let missing_candidate_count =
        comparison.missing_candidate.len();

    let mut severity = RegressionSeverity::None;

    if regression_count > 0 {
        severity = RegressionSeverity::Error;
    }

    if policy.fail_on_statistically_unresolved
        && statistically_unresolved_count > 0
    {
        severity = RegressionSeverity::Error;
    }

    if policy.fail_on_neutral && neutral_count > 0 {
        severity = RegressionSeverity::Error;
    }

    let ci_failure =
        severity.fails_gate()
            || (policy.require_complete_baseline
                && missing_baseline_count > 0)
            || (policy.require_complete_candidate
                && missing_candidate_count > 0);

    if !ci_failure
        && (statistically_unresolved_count > 0
            || neutral_count > 0
            || below_threshold_count > 0)
    {
        severity = RegressionSeverity::Warning;
    }

    Ok(RegressionReport {
        schema_version: REGRESSION_SCHEMA_VERSION,
        policy_id: policy.id(),
        baseline_id: comparison.baseline_id.clone(),
        benchmark_id: comparison.benchmark_id.clone(),
        benchmark_version:
            comparison.benchmark_version.clone(),
        candidate_metric_count:
            comparison.comparisons.len()
                + comparison.missing_baseline.len(),
        compared_metric_count:
            comparison.comparisons.len(),
        metrics,
        missing_baseline:
            comparison.missing_baseline.clone(),
        missing_candidate:
            comparison.missing_candidate.clone(),
        regression_count,
        improvement_count,
        below_threshold_count,
        statistically_unresolved_count,
        neutral_count,
        missing_baseline_count,
        missing_candidate_count,
        severity,
        ci_failure,
    })
}

// =============================================================================
// Per-metric evaluation
// =============================================================================

/// Evaluates one authoritative `BaselineComparison`.
///
/// No comparison mathematics are duplicated here.
pub fn evaluate_metric_comparison(
    comparison: &BaselineComparison,
    policy: &RegressionPolicy,
) -> Result<MetricRegression, RegressionError> {
    policy.validate()?;

    let metric = &comparison.comparison;

    let degradation =
        direction_normalized_degradation(metric);

    if !degradation.is_finite() {
        return Err(
            RegressionError::NonFiniteDegradation {
                identity: comparison.identity(),
                value: degradation,
            },
        );
    }

    let relative_degradation =
        direction_normalized_relative_degradation(
            metric,
            degradation,
        );

    if let Some(value) = relative_degradation {
        if !value.is_finite() {
            return Err(
                RegressionError::NonFiniteRelativeDegradation {
                    identity: comparison.identity(),
                    value,
                },
            );
        }
    }

    let exceeded_absolute_threshold =
        degradation
            >= policy.absolute_degradation_threshold
            && degradation > 0.0;

    let exceeded_relative_threshold =
        match relative_degradation {
            Some(value) => {
                value
                    >= policy.relative_degradation_threshold
                    && value > 0.0
            }

            None => false,
        };

    let threshold_exceeded =
        if policy.threshold_is_either {
            exceeded_absolute_threshold
                || exceeded_relative_threshold
        } else {
            exceeded_absolute_threshold
                && exceeded_relative_threshold
        };

    let (decision, severity) = match metric.conclusion {
        ComparisonConclusion::Improvement => (
            RegressionDecision::Improvement,
            RegressionSeverity::None,
        ),

        ComparisonConclusion::NoMaterialChange => (
            RegressionDecision::NoRegression,
            RegressionSeverity::None,
        ),

        ComparisonConclusion::Neutral => (
            RegressionDecision::Neutral,
            if policy.fail_on_neutral {
                RegressionSeverity::Error
            } else {
                RegressionSeverity::Warning
            },
        ),

        ComparisonConclusion::DifferenceWithoutStatisticalConclusion => (
            RegressionDecision::StatisticallyUnresolved,
            if policy.fail_on_statistically_unresolved {
                RegressionSeverity::Error
            } else {
                RegressionSeverity::Warning
            },
        ),

        ComparisonConclusion::Regression => {
            if threshold_exceeded {
                (
                    RegressionDecision::Regression,
                    RegressionSeverity::Error,
                )
            } else {
                (
                    RegressionDecision::BelowThreshold,
                    RegressionSeverity::Warning,
                )
            }
        },
    };

    Ok(MetricRegression {
        schema_version: REGRESSION_SCHEMA_VERSION,
        identity: comparison.identity(),
        comparison: metric.clone(),
        decision,
        severity,
        degradation,
        relative_degradation,
        exceeded_absolute_threshold,
        exceeded_relative_threshold,
    })
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Converts the authoritative comparison into a direction-normalized
/// degradation.
///
/// Positive means worse.
///
/// Negative means better.
///
/// Zero means no numerical degradation.
///
/// This function does not infer direction; it consumes the direction already
/// attached to `MetricComparison`.
fn direction_normalized_degradation(
    comparison: &MetricComparison,
) -> f64 {
    match comparison.direction {
        MetricDirection::HigherIsBetter => {
            comparison.baseline_value
                - comparison.candidate_value
        }

        MetricDirection::LowerIsBetter => {
            comparison.candidate_value
                - comparison.baseline_value
        }

        MetricDirection::Neutral => 0.0,
    }
}

/// Converts the authoritative relative-change result into a
/// direction-normalized relative degradation.
///
/// The underlying comparison stores:
///
/// ```text
/// (candidate - baseline) / |baseline|
/// ```
///
/// Therefore:
///
/// - higher-is-better reverses the sign;
/// - lower-is-better preserves the sign.
///
/// Positive values always mean degradation.
fn direction_normalized_relative_degradation(
    comparison: &MetricComparison,
    degradation: f64,
) -> Option<f64> {
    match comparison.relative_change.fraction() {
        Some(relative_change) => {
            let value = match comparison.direction {
                MetricDirection::HigherIsBetter => {
                    -relative_change
                }

                MetricDirection::LowerIsBetter => {
                    relative_change
                }

                MetricDirection::Neutral => 0.0,
            };

            Some(value)
        }

        None => {
            // A non-zero absolute degradation with a zero baseline does not
            // have a mathematically defined relative degradation.
            //
            // We deliberately return None instead of infinity.
            if degradation == 0.0 {
                Some(0.0)
            } else {
                None
            }
        }
    }
}

/// Validates one policy threshold.
fn validate_threshold(
    value: f64,
    field: &'static str,
) -> Result<(), RegressionError> {
    if !value.is_finite()
        || value < 0.0
        || value > MAX_REGRESSION_THRESHOLD
    {
        return Err(
            RegressionError::InvalidThreshold {
                field,
                value,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::benchmarking::analysis::baseline::{
        Baseline,
        BaselineMetric,
    };

    use crate::quantum::benchmarking::core::metric::{
        Metric,
        MetricKind,
        MetricUnit,
    };

    fn metric(
        kind: MetricKind,
        value: f64,
    ) -> Metric {
        Metric::new(
            kind,
            MetricUnit::Dimensionless,
            value,
        )
        .expect("test metric must be valid")
    }

    fn baseline_with_metric(
        metric: Metric,
    ) -> Baseline {
        Baseline::builder("test-baseline")
            .benchmark("test", "1.0")
            .add_metric(
                vec![("size", "8")],
                metric,
            )
            .expect("metric should be accepted")
            .build()
            .expect("baseline should be valid")
    }

    fn candidate(
        metric: Metric,
    ) -> Vec<BaselineMetric> {
        vec![
            BaselineMetric::new(
                vec![
                    crate::quantum::benchmarking::analysis::baseline::BaselineDimension::new(
                        "size",
                        "8",
                    )
                    .expect("dimension"),
                ],
                metric,
            )
            .expect("candidate metric"),
        ]
    }

    #[test]
    fn policy_default_is_valid() {
        RegressionPolicy::default()
            .validate()
            .expect("default policy must be valid");
    }

    #[test]
    fn ci_policy_is_valid() {
        RegressionPolicy::ci()
            .validate()
            .expect("CI policy must be valid");
    }

    #[test]
    fn analysis_policy_is_valid() {
        RegressionPolicy::analysis()
            .validate()
            .expect("analysis policy must be valid");
    }

    #[test]
    fn invalid_negative_threshold_is_rejected() {
        let policy = RegressionPolicy {
            absolute_degradation_threshold: -0.1,
            ..RegressionPolicy::default()
        };

        assert!(
            policy.validate().is_err()
        );
    }

    #[test]
    fn invalid_nan_threshold_is_rejected() {
        let policy = RegressionPolicy {
            relative_degradation_threshold:
                f64::NAN,
            ..RegressionPolicy::default()
        };

        assert!(
            policy.validate().is_err()
        );
    }

    #[test]
    fn invalid_infinite_threshold_is_rejected() {
        let policy = RegressionPolicy {
            relative_degradation_threshold:
                f64::INFINITY,
            ..RegressionPolicy::default()
        };

        assert!(
            policy.validate().is_err()
        );
    }

    #[test]
    fn improvement_is_not_regression() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::QuantumVolume,
                100.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::QuantumVolume,
                120.0,
            )),
            &RegressionPolicy::default(),
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            0
        );

        assert_eq!(
            report.improvement_count,
            1
        );

        assert!(!report.has_regression());
    }

    #[test]
    fn higher_is_better_detects_relative_regression() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::QuantumVolume,
                100.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::QuantumVolume,
                90.0,
            )),
            &RegressionPolicy::default(),
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            1
        );

        assert!(
            report.has_regression()
        );

        assert!(
            report.is_ci_failure()
        );

        let item =
            &report.metrics[0];

        assert_eq!(
            item.decision,
            RegressionDecision::Regression
        );

        assert!(
            item.degradation > 0.0
        );

        assert!(
            item.relative_degradation
                .expect("relative degradation")
                > 0.05
        );
    }

    #[test]
    fn small_regression_is_below_threshold() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::QuantumVolume,
                100.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::QuantumVolume,
                97.0,
            )),
            &RegressionPolicy::default(),
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            0
        );

        assert_eq!(
            report.below_threshold_count,
            1
        );

        assert!(
            !report.has_regression()
        );
    }

    #[test]
    fn lower_is_better_reverses_degradation_direction() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::ExecutionTime,
                100.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::ExecutionTime,
                110.0,
            )),
            &RegressionPolicy::default(),
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            1
        );

        assert!(
            report.metrics[0]
                .degradation
                > 0.0
        );
    }

    #[test]
    fn_lower_is_better_improvement_is_not_regression() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::ExecutionTime,
                100.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::ExecutionTime,
                90.0,
            )),
            &RegressionPolicy::default(),
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            0
        );

        assert_eq!(
            report.improvement_count,
            1
        );
    }

    #[test]
    fn zero_baseline_does_not_create_infinite_relative_degradation() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::ExecutionTime,
                0.0,
            ));

        let report = evaluate_baseline(
            &baseline,
            &candidate(metric(
                MetricKind::ExecutionTime,
                1.0,
            )),
            &RegressionPolicy {
                absolute_degradation_threshold: 0.5,
                relative_degradation_threshold: 0.05,
                ..RegressionPolicy::default()
            },
        )
        .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            1
        );

        assert!(
            report.metrics[0]
                .relative_degradation
                .is_none()
        );
    }

    #[test]
    fn missing_candidate_is_reported() {
        let baseline =
            Baseline::builder("test-baseline")
                .benchmark("test", "1.0")
                .add_metric(
                    vec![("size", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        100.0,
                    ),
                )
                .expect("metric")
                .add_metric(
                    vec![("size", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        200.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let candidates = candidate(
            metric(
                MetricKind::QuantumVolume,
                110.0,
            ),
        );

        let policy =
            RegressionPolicy::analysis();

        let report =
            evaluate_baseline(
                &baseline,
                &candidates,
                &policy,
            )
            .expect("analysis policy should allow missing candidate");

        assert_eq!(
            report.missing_candidate_count,
            1
        );

        assert!(
            !report.has_regression()
        );
    }

    #[test]
    fn missing_candidate_fails_ci_policy() {
        let baseline =
            Baseline::builder("test-baseline")
                .benchmark("test", "1.0")
                .add_metric(
                    vec![("size", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        100.0,
                    ),
                )
                .expect("metric")
                .add_metric(
                    vec![("size", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        200.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let candidates = candidate(
            metric(
                MetricKind::QuantumVolume,
                110.0,
            ),
        );

        let result =
            evaluate_baseline(
                &baseline,
                &candidates,
                &RegressionPolicy::ci(),
            );

        assert!(
            matches!(
                result,
                Err(
                    RegressionError::IncompleteCandidate {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn policy_identifier_is_deterministic() {
        let first =
            RegressionPolicy::default()
                .id();

        let second =
            RegressionPolicy::default()
                .id();

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn either_threshold_mode_can_detect_absolute_regression() {
        let policy = RegressionPolicy {
            absolute_degradation_threshold: 5.0,
            relative_degradation_threshold: 0.50,
            threshold_is_either: true,
            ..RegressionPolicy::default()
        };

        let baseline =
            baseline_with_metric(metric(
                MetricKind::ExecutionTime,
                100.0,
            ));

        let report =
            evaluate_baseline(
                &baseline,
                &candidate(metric(
                    MetricKind::ExecutionTime,
                    106.0,
                )),
                &policy,
            )
            .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            1
        );
    }

    #[test]
    fn both_threshold_mode_requires_both_thresholds() {
        let policy = RegressionPolicy {
            absolute_degradation_threshold: 5.0,
            relative_degradation_threshold: 0.10,
            threshold_is_either: false,
            ..RegressionPolicy::default()
        };

        let baseline =
            baseline_with_metric(metric(
                MetricKind::ExecutionTime,
                100.0,
            ));

        let report =
            evaluate_baseline(
                &baseline,
                &candidate(metric(
                    MetricKind::ExecutionTime,
                    106.0,
                )),
                &policy,
            )
            .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            0
        );

        assert_eq!(
            report.below_threshold_count,
            1
        );
    }

    #[test]
    fn excessive_candidate_count_is_rejected() {
        let policy =
            RegressionPolicy {
                max_metrics: 1,
                ..RegressionPolicy::default()
            };

        let baseline =
            baseline_with_metric(metric(
                MetricKind::QuantumVolume,
                100.0,
            ));

        let candidates = vec![
            BaselineMetric::new(
                vec![
                    crate::quantum::benchmarking::analysis::baseline::BaselineDimension::new(
                        "size",
                        "8",
                    )
                    .expect("dimension"),
                ],
                metric(
                    MetricKind::QuantumVolume,
                    100.0,
                ),
            )
            .expect("candidate"),
            BaselineMetric::new(
                vec![
                    crate::quantum::benchmarking::analysis::baseline::BaselineDimension::new(
                        "size",
                        "9",
                    )
                    .expect("dimension"),
                ],
                metric(
                    MetricKind::QuantumVolume,
                    100.0,
                ),
            )
            .expect("candidate"),
        ];

        let result =
            evaluate_baseline(
                &baseline,
                &candidates,
                &policy,
            );

        assert!(
            matches!(
                result,
                Err(
                    RegressionError::TooManyMetrics {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn no_material_change_is_not_regression() {
        let baseline =
            baseline_with_metric(metric(
                MetricKind::QuantumVolume,
                100.0,
            ));

        let report =
            evaluate_baseline(
                &baseline,
                &candidate(metric(
                    MetricKind::QuantumVolume,
                    100.0,
                )),
                &RegressionPolicy::default(),
            )
            .expect("evaluation should succeed");

        assert_eq!(
            report.regression_count,
            0
        );

        assert_eq!(
            report.improvement_count,
            0
        );

        assert_eq!(
            report.below_threshold_count,
            0
        );
    }
}