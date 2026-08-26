//! Zamani Quantum Benchmarking — Metric Comparison
//!
//! Production-grade comparison of quantum-benchmarking metrics.
//!
//! # Purpose
//!
//! This module answers one narrowly defined question:
//!
//! > Given two already-computed benchmark metrics, or two collections of
//! > metrics, how do they differ and what can be safely concluded from that
//! > difference?
//!
//! This module deliberately does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - select hardware;
//! - communicate with providers;
//! - compile or transpile circuits;
//! - perform routing;
//! - perform scheduling;
//! - calculate protocol-specific metrics;
//! - fit randomized-benchmarking curves;
//! - calculate confidence intervals;
//! - establish benchmark baselines;
//! - detect historical regressions;
//! - produce reports;
//! - mutate global state;
//! - print diagnostics.
//!
//! Those responsibilities belong to other benchmarking layers.
//!
//! # Architectural position
//!
//! ```text
//! Benchmark execution / protocol / stored result
//!                    │
//!                    ▼
//!              core::metric
//!                    │
//!                    ▼
//!          analysis::compare
//!             │          │
//!             ▼          ▼
//!        comparison    comparison
//!          result        result
//!             │          │
//!             └────┬─────┘
//!                  ▼
//!          reporting / baseline / regression
//! ```
//!
//! The dependency direction is intentionally:
//!
//! ```text
//! analysis::compare
//!        │
//!        ▼
//! core::metric
//! ```
//!
//! and never:
//!
//! ```text
//! core::metric
//!        │
//!        ▼
//! analysis::compare
//! ```
//!
//! # Why comparison is a separate subsystem
//!
//! A metric value alone is not sufficient for a scientifically meaningful
//! comparison. A production comparison needs at least:
//!
//! - metric identity;
//! - unit;
//! - baseline value;
//! - candidate value;
//! - absolute difference;
//! - relative difference when mathematically defined;
//! - optimization direction;
//! - uncertainty when available;
//! - confidence intervals when available;
//! - sample counts when available;
//! - circuit/shot counts when available;
//! - quality classifications;
//! - compatibility validation;
//! - explicit conclusion semantics;
//! - warnings about missing statistical information.
//!
//! This module therefore never returns a naked `f64` as its primary result.
//!
//! # Important scientific rule
//!
//! A numerical difference is not automatically a statistically significant
//! difference.
//!
//! For that reason this module distinguishes:
//!
//! 1. numerical change;
//! 2. direction of change;
//! 3. confidence-interval relationship;
//! 4. uncertainty-based separation score;
//! 5. statistical conclusion.
//!
//! In particular, this module does NOT treat confidence-interval overlap as a
//! universal significance test. It only reports whether supplied intervals are
//! disjoint. Formal hypothesis testing belongs in `statistics::hypothesis`.
//!
//! # Optimization direction
//!
//! `MetricDirection` from `core::metric` is authoritative.
//!
//! ```text
//! HigherIsBetter
//!     candidate > baseline  => improvement
//!     candidate < baseline  => regression
//!
//! LowerIsBetter
//!     candidate < baseline  => improvement
//!     candidate > baseline  => regression
//!
//! Neutral
//!     no generic improvement/regression conclusion
//! ```
//!
//! An explicitly overridden `MetricDirection` on `Metric` is therefore
//! respected. This is essential for metrics whose interpretation cannot be
//! safely inferred from their enum variant alone.
//!
//! # Relative change
//!
//! Relative change is defined as:
//!
//! ```text
//! (candidate - baseline) / |baseline|
//! ```
//!
//! This avoids changing the sign of the denominator for negative-valued
//! metrics.
//!
//! If the baseline is zero, relative change is undefined unless the candidate
//! is also zero. The API reports this explicitly rather than returning
//! infinity or NaN.
//!
//! # Ratio
//!
//! The ratio is:
//!
//! ```text
//! candidate / baseline
//! ```
//!
//! It is only returned when the baseline is non-zero.
//!
//! # Statistical separation
//!
//! When both metrics provide standard uncertainties:
//!
//! ```text
//! z = |candidate - baseline|
//!     / sqrt(u_baseline^2 + u_candidate^2)
//! ```
//!
//! This is an uncertainty-separation diagnostic only. It is NOT a p-value and
//! is NOT a substitute for a formal statistical hypothesis test.
//!
//! When confidence intervals are supplied, the module also reports whether
//! the intervals are disjoint.
//!
//! # Multi-metric comparison
//!
//! `compare_metric_sets()` compares collections by metric identity:
//!
//! ```text
//! MetricKind + MetricUnit
//! ```
//!
//! Duplicate identities are rejected rather than silently overwritten.
//!
//! This makes the function safe for use by future:
//!
//! - `core::result`;
//! - `analysis::baseline`;
//! - `analysis::regression`;
//! - `reporting`;
//! - CI regression checks;
//! - cross-backend comparison;
//! - simulator-versus-hardware comparison;
//! - compiler-version comparison;
//! - calibration comparison.
//!
//! # Reproducibility
//!
//! This module is deterministic for deterministic inputs.
//!
//! It does not access clocks, random generators, hardware, environment
//! variables, files, network services, or process-global state.
//!
//! # Security and resource safety
//!
//! The implementation:
//!
//! - performs no unbounded allocation;
//! - does not recurse;
//! - does not use unsafe code;
//! - rejects non-finite values inherited from malformed/deserialized data;
//! - rejects incompatible metric identities;
//! - rejects duplicate identities in metric collections;
//! - uses checked arithmetic where overflow is possible;
//! - never panics as part of normal comparison operation.
//!
//! # Integration contract
//!
//! This file is intentionally complete without requiring future files.
//!
//! Current dependency:
//!
//! ```text
//! analysis::compare
//!        │
//!        ▼
//! core::metric
//! ```
//!
//! Future consumers can integrate without modifying this file:
//!
//! ```text
//! core::result
//!        │
//!        ▼
//! analysis::compare
//!        │
//!        ├── analysis::baseline
//!        ├── analysis::regression
//!        └── reporting::*
//! ```
//!
//! `core::result` should adapt its metric collection to
//! `compare_metric_sets()` rather than changing this module.
//!
//! `analysis::baseline` should store `MetricComparison` or
//! `MetricSetComparison` rather than adding baseline state here.
//!
//! `analysis::regression` should consume `MetricComparison` and apply temporal
//! or policy-specific regression rules there.
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
//! No additional crate dependency is introduced.
//!
//! Serde is intentionally not required here. The canonical `Metric` already
//! owns serialization. Comparison objects are analysis products and can be
//! serialized later by the reporting layer without forcing this independent
//! module to depend on a reporting schema.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! Primary types:
//!
//! - `ComparisonPolicy`
//! - `ComparisonConclusion`
//! - `RelativeChange`
//! - `UncertaintySeparation`
//! - `ConfidenceRelationship`
//! - `MetricComparison`
//! - `MetricSetComparison`
//! - `MetricComparisonError`
//!
//! Primary functions:
//!
//! - `compare_metrics()`
//! - `compare_metric_sets()`
//!
//! -----------------------------------------------------------------------------
//! Example
//! -----------------------------------------------------------------------------
//!
//! ```rust
//! use crate::quantum::benchmarking::analysis::compare::{
//!     compare_metrics,
//!     ComparisonConclusion,
//!     ComparisonPolicy,
//! };
//! use crate::quantum::benchmarking::core::metric::{
//!     Metric,
//!     MetricKind,
//!     MetricUnit,
//! };
//!
//! let baseline = Metric::new(
//!     MetricKind::QuantumVolume,
//!     MetricUnit::Dimensionless,
//!     32.0,
//! ).unwrap();
//!
//! let candidate = Metric::new(
//!     MetricKind::QuantumVolume,
//!     MetricUnit::Dimensionless,
//!     64.0,
//! ).unwrap();
//!
//! let comparison = compare_metrics(
//!     &baseline,
//!     &candidate,
//!     &ComparisonPolicy::default(),
//! ).unwrap();
//!
//! assert_eq!(
//!     comparison.conclusion,
//!     ComparisonConclusion::Improvement
//! );
//! ```
//!

use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricDirection,
    MetricKind,
    MetricUnit,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable comparison-analysis schema version.
///
/// This version belongs to the semantic comparison contract and is independent
/// of the Zamani compiler version.
pub const COMPARISON_SCHEMA_VERSION: u32 = 1;

/// Default numerical tolerance used when determining whether two floating-point
/// values are effectively equal.
///
/// This tolerance is deliberately small. Scientific comparison should not use a
/// broad tolerance that can hide real performance changes.
pub const DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance used for effectively-equal values.
pub const DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-9;

/// Maximum accepted tolerance.
///
/// This prevents a malformed configuration from effectively declaring
/// substantially different benchmark results equal.
pub const MAX_COMPARISON_TOLERANCE: f64 = 1.0;

/// Default uncertainty-separation threshold.
///
/// This is a diagnostic threshold only. It does not constitute a universal
/// hypothesis-test threshold.
pub const DEFAULT_UNCERTAINTY_SEPARATION_Z: f64 = 2.0;

/// Maximum number of metrics accepted by `compare_metric_sets()`.
///
/// This is a defensive bound against pathological input. The value is large
/// enough for practical benchmark result sets while preventing accidental
/// quadratic explosions from malicious or corrupted inputs.
pub const DEFAULT_MAX_METRICS_PER_SET: usize = 100_000;

// =============================================================================
// Comparison policy
// =============================================================================

/// Policy controlling how two metrics are compared.
///
/// The policy is deliberately independent of benchmark protocols. A protocol
/// can construct a policy appropriate to its comparison requirements without
/// changing this module.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonPolicy {
    /// Absolute tolerance used when determining effective equality.
    pub absolute_tolerance: f64,

    /// Relative tolerance used when determining effective equality.
    pub relative_tolerance: f64,

    /// Whether metric kinds must match.
    pub require_same_kind: bool,

    /// Whether units must match.
    pub require_same_unit: bool,

    /// Whether optimization directions must match.
    pub require_same_direction: bool,

    /// Whether the comparison should calculate uncertainty separation when
    /// both metrics provide uncertainties.
    pub calculate_uncertainty_separation: bool,

    /// Z-score-like diagnostic threshold for uncertainty separation.
    ///
    /// This is not a statistical hypothesis-test threshold.
    pub uncertainty_separation_threshold: f64,

    /// Maximum number of metrics accepted in one set comparison.
    pub max_metrics_per_set: usize,
}

impl Default for ComparisonPolicy {
    fn default() -> Self {
        Self {
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
            require_same_kind: true,
            require_same_unit: true,
            require_same_direction: true,
            calculate_uncertainty_separation: true,
            uncertainty_separation_threshold: DEFAULT_UNCERTAINTY_SEPARATION_Z,
            max_metrics_per_set: DEFAULT_MAX_METRICS_PER_SET,
        }
    }
}

impl ComparisonPolicy {
    /// Validates the policy.
    pub fn validate(&self) -> Result<(), MetricComparisonError> {
        validate_tolerance(
            self.absolute_tolerance,
            "absolute_tolerance",
        )?;

        validate_tolerance(
            self.relative_tolerance,
            "relative_tolerance",
        )?;

        if self.uncertainty_separation_threshold.is_nan()
            || self.uncertainty_separation_threshold.is_infinite()
            || self.uncertainty_separation_threshold < 0.0
        {
            return Err(MetricComparisonError::InvalidPolicy {
                field: "uncertainty_separation_threshold",
                value: self.uncertainty_separation_threshold,
            });
        }

        if self.max_metrics_per_set == 0 {
            return Err(MetricComparisonError::InvalidMetricSetLimit);
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by metric comparison.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricComparisonError {
    /// The comparison policy contains an invalid value.
    InvalidPolicy {
        /// Invalid policy field.
        field: &'static str,

        /// Invalid field value.
        value: f64,
    },

    /// The maximum metric-set size was zero.
    InvalidMetricSetLimit,

    /// A metric contains a non-finite value.
    NonFiniteMetricValue {
        /// Metric identity.
        metric: String,

        /// Invalid value.
        value: f64,
    },

    /// A metric contains a non-finite uncertainty.
    NonFiniteUncertainty {
        /// Metric identity.
        metric: String,

        /// Invalid uncertainty.
        value: f64,
    },

    /// Metric kinds do not match.
    MetricKindMismatch {
        /// Baseline metric kind.
        baseline: String,

        /// Candidate metric kind.
        candidate: String,
    },

    /// Metric units do not match.
    MetricUnitMismatch {
        /// Baseline unit.
        baseline: String,

        /// Candidate unit.
        candidate: String,
    },

    /// Metric optimization directions do not match.
    MetricDirectionMismatch {
        /// Baseline direction.
        baseline: MetricDirection,

        /// Candidate direction.
        candidate: MetricDirection,
    },

    /// The metric collection contains too many entries.
    MetricSetTooLarge {
        /// Number of supplied metrics.
        count: usize,

        /// Maximum allowed metrics.
        maximum: usize,
    },

    /// The metric collection contains duplicate metric identities.
    DuplicateMetricIdentity {
        /// Duplicated metric identity.
        identity: String,
    },

    /// A required metric could not be found.
    MissingMetric {
        /// Missing metric identity.
        identity: String,
    },

    /// A metric set contains an invalid metric.
    InvalidMetric {
        /// Metric identity.
        identity: String,

        /// Reason supplied by the metric.
        reason: String,
    },
}

impl fmt::Display for MetricComparisonError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidPolicy { field, value } => {
                write!(
                    formatter,
                    "invalid comparison policy field `{}` with value {}",
                    field,
                    value
                )
            }

            Self::InvalidMetricSetLimit => {
                write!(
                    formatter,
                    "maximum metric-set size must be greater than zero"
                )
            }

            Self::NonFiniteMetricValue { metric, value } => {
                write!(
                    formatter,
                    "metric `{}` contains non-finite value {}",
                    metric,
                    value
                )
            }

            Self::NonFiniteUncertainty { metric, value } => {
                write!(
                    formatter,
                    "metric `{}` contains non-finite uncertainty {}",
                    metric,
                    value
                )
            }

            Self::MetricKindMismatch {
                baseline,
                candidate,
            } => {
                write!(
                    formatter,
                    "metric kind mismatch: baseline `{}` versus candidate `{}`",
                    baseline,
                    candidate
                )
            }

            Self::MetricUnitMismatch {
                baseline,
                candidate,
            } => {
                write!(
                    formatter,
                    "metric unit mismatch: baseline `{}` versus candidate `{}`",
                    baseline,
                    candidate
                )
            }

            Self::MetricDirectionMismatch {
                baseline,
                candidate,
            } => {
                write!(
                    formatter,
                    "metric direction mismatch: baseline `{:?}` versus candidate `{:?}`",
                    baseline,
                    candidate
                )
            }

            Self::MetricSetTooLarge { count, maximum } => {
                write!(
                    formatter,
                    "metric set contains {} metrics, exceeding the maximum of {}",
                    count,
                    maximum
                )
            }

            Self::DuplicateMetricIdentity { identity } => {
                write!(
                    formatter,
                    "metric set contains duplicate metric identity `{}`",
                    identity
                )
            }

            Self::MissingMetric { identity } => {
                write!(
                    formatter,
                    "required metric `{}` was not found",
                    identity
                )
            }

            Self::InvalidMetric { identity, reason } => {
                write!(
                    formatter,
                    "metric `{}` is invalid: {}",
                    identity,
                    reason
                )
            }
        }
    }
}

impl Error for MetricComparisonError {}

// =============================================================================
// Conclusion
// =============================================================================

/// Semantic conclusion of a metric comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonConclusion {
    /// Candidate is better according to the metric direction.
    Improvement,

    /// Candidate is worse according to the metric direction.
    Regression,

    /// Values are effectively equal under the comparison policy.
    NoMaterialChange,

    /// Metric direction is neutral, so no generic better/worse conclusion is
    /// scientifically justified.
    Neutral,

    /// The candidate is numerically different, but the comparison does not
    /// have enough statistical information to make a stronger conclusion.
    DifferenceWithoutStatisticalConclusion,
}

impl ComparisonConclusion {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Improvement => "improvement",
            Self::Regression => "regression",
            Self::NoMaterialChange => "no_material_change",
            Self::Neutral => "neutral",
            Self::DifferenceWithoutStatisticalConclusion => {
                "difference_without_statistical_conclusion"
            }
        }
    }

    /// Returns whether the conclusion identifies a performance improvement.
    pub const fn is_improvement(self) -> bool {
        matches!(self, Self::Improvement)
    }

    /// Returns whether the conclusion identifies a performance regression.
    pub const fn is_regression(self) -> bool {
        matches!(self, Self::Regression)
    }
}

impl fmt::Display for ComparisonConclusion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

// =============================================================================
// Relative change
// =============================================================================

/// Result of calculating relative change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativeChange {
    /// A finite relative change exists.
    Defined {
        /// Relative change as a fraction.
        ///
        /// `0.10` means a 10% increase in the numerical value.
        fraction: f64,
    },

    /// Baseline and candidate are both zero.
    BothZero,

    /// Baseline is zero and candidate is non-zero, so relative change is
    /// mathematically undefined.
    UndefinedFromZeroBaseline,
}

impl RelativeChange {
    /// Returns the finite fraction if one exists.
    pub const fn fraction(self) -> Option<f64> {
        match self {
            Self::Defined { fraction } => Some(fraction),
            Self::BothZero
            | Self::UndefinedFromZeroBaseline => None,
        }
    }

    /// Returns the percentage change when defined.
    pub fn percent(self) -> Option<f64> {
        self.fraction().map(|value| value * 100.0)
    }

    /// Returns a stable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Defined { .. } => "defined",
            Self::BothZero => "both_zero",
            Self::UndefinedFromZeroBaseline => {
                "undefined_from_zero_baseline"
            }
        }
    }
}

// =============================================================================
// Confidence relationship
// =============================================================================

/// Relationship between supplied confidence intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceRelationship {
    /// Neither metric supplied a confidence interval.
    Unavailable,

    /// Only the baseline supplied a confidence interval.
    BaselineOnly,

    /// Only the candidate supplied a confidence interval.
    CandidateOnly,

    /// Both intervals exist and are disjoint.
    Disjoint,

    /// Both intervals exist and overlap.
    Overlapping,

    /// Both intervals exist and touch exactly at a boundary.
    Touching,
}

impl ConfidenceRelationship {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::BaselineOnly => "baseline_only",
            Self::CandidateOnly => "candidate_only",
            Self::Disjoint => "disjoint",
            Self::Overlapping => "overlapping",
            Self::Touching => "touching",
        }
    }

    /// Returns whether both intervals were supplied.
    pub const fn has_both(self) -> bool {
        matches!(
            self,
            Self::Disjoint
                | Self::Overlapping
                | Self::Touching
        )
    }
}

// =============================================================================
// Uncertainty separation
// =============================================================================

/// Uncertainty-based separation diagnostic.
///
/// This is deliberately named `UncertaintySeparation` rather than
/// `StatisticalSignificance` because it is not a formal hypothesis test.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UncertaintySeparation {
    /// Absolute numerical difference.
    pub absolute_difference: f64,

    /// Combined standard uncertainty.
    pub combined_uncertainty: f64,

    /// Separation score:
    ///
    /// `absolute_difference / combined_uncertainty`
    pub score: f64,

    /// Configured diagnostic threshold.
    pub threshold: f64,

    /// Whether the score meets the configured diagnostic threshold.
    pub exceeds_threshold: bool,
}

impl UncertaintySeparation {
    /// Creates an uncertainty-separation diagnostic.
    pub fn new(
        absolute_difference: f64,
        baseline_uncertainty: f64,
        candidate_uncertainty: f64,
        threshold: f64,
    ) -> Option<Self> {
        if !absolute_difference.is_finite()
            || !baseline_uncertainty.is_finite()
            || !candidate_uncertainty.is_finite()
            || !threshold.is_finite()
        {
            return None;
        }

        if baseline_uncertainty < 0.0
            || candidate_uncertainty < 0.0
            || threshold < 0.0
        {
            return None;
        }

        let combined_variance = baseline_uncertainty
            .mul_add(
                baseline_uncertainty,
                candidate_uncertainty
                    * candidate_uncertainty,
            );

        if !combined_variance.is_finite()
            || combined_variance < 0.0
        {
            return None;
        }

        let combined_uncertainty = combined_variance.sqrt();

        if combined_uncertainty == 0.0 {
            return Some(Self {
                absolute_difference,
                combined_uncertainty,
                score: if absolute_difference == 0.0 {
                    0.0
                } else {
                    f64::INFINITY
                },
                threshold,
                exceeds_threshold: absolute_difference > 0.0,
            });
        }

        let score = absolute_difference / combined_uncertainty;

        if !score.is_finite() {
            return None;
        }

        Some(Self {
            absolute_difference,
            combined_uncertainty,
            score,
            threshold,
            exceeds_threshold: score >= threshold,
        })
    }
}

// =============================================================================
// Metric identity
// =============================================================================

/// Stable identity of a metric for collection comparison.
///
/// The identity intentionally consists of metric kind and unit only.
///
/// Benchmark dimensions such as qubit count, circuit depth or problem size
/// belong in the surrounding result/dimension model. They should not be
/// silently encoded into the metric identity here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetricIdentity {
    /// Metric kind.
    pub kind: MetricKind,

    /// Metric unit.
    pub unit: MetricUnit,
}

impl MetricIdentity {
    /// Creates a metric identity.
    pub fn new(
        kind: MetricKind,
        unit: MetricUnit,
    ) -> Self {
        Self { kind, unit }
    }

    /// Returns a stable human/machine-readable identity.
    pub fn id(&self) -> String {
        format!("{}:{}", self.kind.id(), self.unit.id())
    }
}

impl fmt::Display for MetricIdentity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.kind.id(),
            self.unit.id()
        )
    }
}

// =============================================================================
// Metric comparison
// =============================================================================

/// Complete comparison between a baseline metric and a candidate metric.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricComparison {
    /// Comparison schema version.
    pub schema_version: u32,

    /// Metric identity.
    pub identity: MetricIdentity,

    /// Baseline value.
    pub baseline_value: f64,

    /// Candidate value.
    pub candidate_value: f64,

    /// Signed numerical difference:
    ///
    /// `candidate - baseline`
    pub absolute_difference: f64,

    /// Relative numerical difference.
    pub relative_change: RelativeChange,

    /// Candidate/baseline ratio when defined.
    pub ratio: Option<f64>,

    /// Metric optimization direction.
    pub direction: MetricDirection,

    /// Final semantic conclusion.
    pub conclusion: ComparisonConclusion,

    /// Confidence-interval relationship.
    pub confidence_relationship: ConfidenceRelationship,

    /// Optional uncertainty separation diagnostic.
    pub uncertainty_separation: Option<UncertaintySeparation>,

    /// Baseline uncertainty, if supplied.
    pub baseline_uncertainty: Option<f64>,

    /// Candidate uncertainty, if supplied.
    pub candidate_uncertainty: Option<f64>,

    /// Baseline sample count, if supplied.
    pub baseline_sample_count: Option<u64>,

    /// Candidate sample count, if supplied.
    pub candidate_sample_count: Option<u64>,

    /// Baseline shot count, if supplied.
    pub baseline_shot_count: Option<u64>,

    /// Candidate shot count, if supplied.
    pub candidate_shot_count: Option<u64>,

    /// Baseline circuit count, if supplied.
    pub baseline_circuit_count: Option<u64>,

    /// Candidate circuit count, if supplied.
    pub candidate_circuit_count: Option<u64>,

    /// Whether the numerical difference is within the configured equality
    /// tolerance.
    pub within_tolerance: bool,

    /// Whether both metrics provide compatible confidence intervals that are
    /// disjoint.
    ///
    /// This is only a diagnostic. It is not a universal significance test.
    pub confidence_intervals_disjoint: bool,
}

impl MetricComparison {
    /// Returns the signed percentage change when defined.
    pub fn percent_change(&self) -> Option<f64> {
        self.relative_change.percent()
    }

    /// Returns whether the candidate is numerically larger than the baseline.
    pub const fn candidate_is_numerically_larger(&self) -> bool {
        self.absolute_difference > 0.0
    }

    /// Returns whether the candidate is numerically smaller than the baseline.
    pub const fn candidate_is_numerically_smaller(&self) -> bool {
        self.absolute_difference < 0.0
    }

    /// Returns whether the comparison produced a performance improvement.
    pub const fn is_improvement(&self) -> bool {
        self.conclusion.is_improvement()
    }

    /// Returns whether the comparison produced a performance regression.
    pub const fn is_regression(&self) -> bool {
        self.conclusion.is_regression()
    }

    /// Returns whether the comparison has both confidence intervals.
    pub const fn has_confidence_intervals(&self) -> bool {
        self.confidence_relationship.has_both()
    }

    /// Returns whether a stronger statistical conclusion should be delegated
    /// to `statistics::hypothesis`.
    pub const fn requires_hypothesis_testing(&self) -> bool {
        !self.within_tolerance
            && self.confidence_relationship.has_both()
            && self.uncertainty_separation.is_none()
    }
}

// =============================================================================
// Metric-set comparison
// =============================================================================

/// Complete comparison of two collections of metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSetComparison {
    /// Comparison schema version.
    pub schema_version: u32,

    /// Per-metric comparisons.
    pub comparisons: Vec<MetricComparison>,

    /// Metrics present only in the baseline.
    pub baseline_only: Vec<MetricIdentity>,

    /// Metrics present only in the candidate.
    pub candidate_only: Vec<MetricIdentity>,

    /// Number of improvements.
    pub improvement_count: usize,

    /// Number of regressions.
    pub regression_count: usize,

    /// Number of neutral/no-material-change results.
    pub unchanged_count: usize,

    /// Number of comparisons without a generic directional conclusion.
    pub neutral_count: usize,

    /// Number of comparisons with a numerical difference but without a
    /// statistical conclusion.
    pub statistically_unresolved_count: usize,
}

impl MetricSetComparison {
    /// Returns whether every shared metric improved.
    pub fn all_improved(&self) -> bool {
        !self.comparisons.is_empty()
            && self.regression_count == 0
            && self.neutral_count == 0
            && self.statistically_unresolved_count == 0
            && self.improvement_count == self.comparisons.len()
    }

    /// Returns whether at least one regression exists.
    pub const fn has_regression(&self) -> bool {
        self.regression_count > 0
    }

    /// Returns whether the two sets contain exactly the same metric identities.
    pub fn have_same_metric_identities(&self) -> bool {
        self.baseline_only.is_empty()
            && self.candidate_only.is_empty()
    }
}

// =============================================================================
// Public comparison functions
// =============================================================================

/// Compares one baseline metric against one candidate metric.
///
/// The baseline is the reference value. The candidate is the new value being
/// evaluated.
///
/// The function does not mutate either metric.
pub fn compare_metrics(
    baseline: &Metric,
    candidate: &Metric,
    policy: &ComparisonPolicy,
) -> Result<MetricComparison, MetricComparisonError> {
    policy.validate()?;

    validate_metric(baseline)?;
    validate_metric(candidate)?;

    let baseline_identity = MetricIdentity::new(
        baseline.kind.clone(),
        baseline.unit.clone(),
    );

    let candidate_identity = MetricIdentity::new(
        candidate.kind.clone(),
        candidate.unit.clone(),
    );

    if policy.require_same_kind
        && baseline.kind != candidate.kind
    {
        return Err(
            MetricComparisonError::MetricKindMismatch {
                baseline: baseline.kind.id(),
                candidate: candidate.kind.id(),
            },
        );
    }

    if policy.require_same_unit
        && baseline.unit != candidate.unit
    {
        return Err(
            MetricComparisonError::MetricUnitMismatch {
                baseline: baseline.unit.id(),
                candidate: candidate.unit.id(),
            },
        );
    }

    if policy.require_same_direction
        && baseline.direction != candidate.direction
    {
        return Err(
            MetricComparisonError::MetricDirectionMismatch {
                baseline: baseline.direction,
                candidate: candidate.direction,
            },
        );
    }

    let identity = if baseline_identity == candidate_identity {
        baseline_identity
    } else {
        // This branch is only reachable when policy permits comparing different
        // identities. The baseline identity remains the stable identity of the
        // comparison because it is the reference metric.
        baseline_identity
    };

    let baseline_value = baseline.value.get();
    let candidate_value = candidate.value.get();

    let absolute_difference =
        candidate_value - baseline_value;

    if !absolute_difference.is_finite() {
        return Err(
            MetricComparisonError::NonFiniteMetricValue {
                metric: identity.id(),
                value: absolute_difference,
            },
        );
    }

    let within_tolerance = effectively_equal(
        baseline_value,
        candidate_value,
        policy.absolute_tolerance,
        policy.relative_tolerance,
    );

    let relative_change =
        calculate_relative_change(
            baseline_value,
            candidate_value,
        );

    let ratio = calculate_ratio(
        baseline_value,
        candidate_value,
    );

    let direction = candidate.direction;

    let confidence_relationship =
        compare_confidence_intervals(
            baseline,
            candidate,
        );

    let confidence_intervals_disjoint =
        matches!(
            confidence_relationship,
            ConfidenceRelationship::Disjoint
        );

    let uncertainty_separation =
        if policy.calculate_uncertainty_separation {
            match (
                baseline.uncertainty,
                candidate.uncertainty,
            ) {
                (
                    Some(baseline_uncertainty),
                    Some(candidate_uncertainty),
                ) => UncertaintySeparation::new(
                    absolute_difference.abs(),
                    baseline_uncertainty.get(),
                    candidate_uncertainty.get(),
                    policy.uncertainty_separation_threshold,
                ),
                _ => None,
            }
        } else {
            None
        };

    let conclusion = determine_conclusion(
        baseline_value,
        candidate_value,
        direction,
        within_tolerance,
        confidence_relationship,
        uncertainty_separation,
    );

    Ok(MetricComparison {
        schema_version: COMPARISON_SCHEMA_VERSION,
        identity,
        baseline_value,
        candidate_value,
        absolute_difference,
        relative_change,
        ratio,
        direction,
        conclusion,
        confidence_relationship,
        uncertainty_separation,
        baseline_uncertainty: baseline
            .uncertainty
            .map(|value| value.get()),
        candidate_uncertainty: candidate
            .uncertainty
            .map(|value| value.get()),
        baseline_sample_count: baseline.sample_count,
        candidate_sample_count: candidate.sample_count,
        baseline_shot_count: baseline.shot_count,
        candidate_shot_count: candidate.shot_count,
        baseline_circuit_count: baseline.circuit_count,
        candidate_circuit_count: candidate.circuit_count,
        within_tolerance,
        confidence_intervals_disjoint,
    })
}

/// Compares two collections of metrics.
///
/// Metrics are matched by `MetricKind + MetricUnit`.
///
/// Duplicate metric identities are rejected rather than silently overwritten.
///
/// Metrics existing in only one collection are reported separately.
pub fn compare_metric_sets(
    baseline: &[Metric],
    candidate: &[Metric],
    policy: &ComparisonPolicy,
) -> Result<MetricSetComparison, MetricComparisonError> {
    policy.validate()?;

    if baseline.len() > policy.max_metrics_per_set {
        return Err(
            MetricComparisonError::MetricSetTooLarge {
                count: baseline.len(),
                maximum: policy.max_metrics_per_set,
            },
        );
    }

    if candidate.len() > policy.max_metrics_per_set {
        return Err(
            MetricComparisonError::MetricSetTooLarge {
                count: candidate.len(),
                maximum: policy.max_metrics_per_set,
            },
        );
    }

    validate_metric_set(baseline)?;
    validate_metric_set(candidate)?;

    let baseline_identities =
        metric_identities(baseline)?;

    let candidate_identities =
        metric_identities(candidate)?;

    let mut comparisons = Vec::new();
    let mut baseline_only = Vec::new();
    let mut candidate_only = Vec::new();

    for baseline_metric in baseline {
        let baseline_identity = MetricIdentity::new(
            baseline_metric.kind.clone(),
            baseline_metric.unit.clone(),
        );

        match find_metric_by_identity(
            candidate,
            &baseline_identity,
        ) {
            Some(candidate_metric) => {
                comparisons.push(compare_metrics(
                    baseline_metric,
                    candidate_metric,
                    policy,
                )?);
            }

            None => {
                baseline_only.push(baseline_identity);
            }
        }
    }

    for candidate_identity in &candidate_identities {
        if !contains_identity(
            &baseline_identities,
            candidate_identity,
        ) {
            candidate_only.push(candidate_identity.clone());
        }
    }

    let mut improvement_count = 0usize;
    let mut regression_count = 0usize;
    let mut unchanged_count = 0usize;
    let mut neutral_count = 0usize;
    let mut statistically_unresolved_count = 0usize;

    for comparison in &comparisons {
        match comparison.conclusion {
            ComparisonConclusion::Improvement => {
                improvement_count =
                    improvement_count.saturating_add(1);
            }

            ComparisonConclusion::Regression => {
                regression_count =
                    regression_count.saturating_add(1);
            }

            ComparisonConclusion::NoMaterialChange => {
                unchanged_count =
                    unchanged_count.saturating_add(1);
            }

            ComparisonConclusion::Neutral => {
                neutral_count =
                    neutral_count.saturating_add(1);
            }

            ComparisonConclusion::DifferenceWithoutStatisticalConclusion => {
                statistically_unresolved_count =
                    statistically_unresolved_count
                        .saturating_add(1);
            }
        }
    }

    Ok(MetricSetComparison {
        schema_version: COMPARISON_SCHEMA_VERSION,
        comparisons,
        baseline_only,
        candidate_only,
        improvement_count,
        regression_count,
        unchanged_count,
        neutral_count,
        statistically_unresolved_count,
    })
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_metric(
    metric: &Metric,
) -> Result<(), MetricComparisonError> {
    let identity = format!(
        "{}:{}",
        metric.kind.id(),
        metric.unit.id()
    );

    metric
        .validate()
        .map_err(|error| {
            MetricComparisonError::InvalidMetric {
                identity: identity.clone(),
                reason: error.to_string(),
            }
        })?;

    let value = metric.value.get();

    if !value.is_finite() {
        return Err(
            MetricComparisonError::NonFiniteMetricValue {
                metric: identity.clone(),
                value,
            },
        );
    }

    if let Some(uncertainty) = metric.uncertainty {
        let uncertainty_value = uncertainty.get();

        if !uncertainty_value.is_finite()
            || uncertainty_value < 0.0
        {
            return Err(
                MetricComparisonError::NonFiniteUncertainty {
                    metric: identity,
                    value: uncertainty_value,
                },
            );
        }
    }

    Ok(())
}

fn validate_metric_set(
    metrics: &[Metric],
) -> Result<(), MetricComparisonError> {
    for metric in metrics {
        validate_metric(metric)?;
    }

    let identities = metric_identities(metrics)?;

    for index in 0..identities.len() {
        for other_index in (index + 1)..identities.len() {
            if identities[index] == identities[other_index] {
                return Err(
                    MetricComparisonError::DuplicateMetricIdentity {
                        identity: identities[index].id(),
                    },
                );
            }
        }
    }

    Ok(())
}

fn metric_identities(
    metrics: &[Metric],
) -> Result<Vec<MetricIdentity>, MetricComparisonError> {
    let mut identities = Vec::with_capacity(metrics.len());

    for metric in metrics {
        let identity = MetricIdentity::new(
            metric.kind.clone(),
            metric.unit.clone(),
        );

        if contains_identity(&identities, &identity) {
            return Err(
                MetricComparisonError::DuplicateMetricIdentity {
                    identity: identity.id(),
                },
            );
        }

        identities.push(identity);
    }

    Ok(identities)
}

fn contains_identity(
    identities: &[MetricIdentity],
    target: &MetricIdentity,
) -> bool {
    identities.iter().any(|identity| identity == target)
}

fn find_metric_by_identity<'a>(
    metrics: &'a [Metric],
    identity: &MetricIdentity,
) -> Option<&'a Metric> {
    metrics.iter().find(|metric| {
        metric.kind == identity.kind
            && metric.unit == identity.unit
    })
}

// =============================================================================
// Numerical helpers
// =============================================================================

fn validate_tolerance(
    value: f64,
    field: &'static str,
) -> Result<(), MetricComparisonError> {
    if !value.is_finite()
        || value < 0.0
        || value > MAX_COMPARISON_TOLERANCE
    {
        return Err(
            MetricComparisonError::InvalidPolicy {
                field,
                value,
            },
        );
    }

    Ok(())
}

fn effectively_equal(
    baseline: f64,
    candidate: f64,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    let difference = (candidate - baseline).abs();

    if difference <= absolute_tolerance {
        return true;
    }

    let scale = baseline.abs().max(candidate.abs());

    if scale == 0.0 {
        return difference == 0.0;
    }

    difference <= relative_tolerance * scale
}

fn calculate_relative_change(
    baseline: f64,
    candidate: f64,
) -> RelativeChange {
    if baseline == 0.0 {
        if candidate == 0.0 {
            RelativeChange::BothZero
        } else {
            RelativeChange::UndefinedFromZeroBaseline
        }
    } else {
        let fraction =
            (candidate - baseline) / baseline.abs();

        if fraction.is_finite() {
            RelativeChange::Defined { fraction }
        } else {
            RelativeChange::UndefinedFromZeroBaseline
        }
    }
}

fn calculate_ratio(
    baseline: f64,
    candidate: f64,
) -> Option<f64> {
    if baseline == 0.0 {
        return None;
    }

    let ratio = candidate / baseline;

    if ratio.is_finite() {
        Some(ratio)
    } else {
        None
    }
}

// =============================================================================
// Confidence analysis
// =============================================================================

fn compare_confidence_intervals(
    baseline: &Metric,
    candidate: &Metric,
) -> ConfidenceRelationship {
    match (
        baseline.confidence.as_ref(),
        candidate.confidence.as_ref(),
    ) {
        (None, None) => {
            ConfidenceRelationship::Unavailable
        }

        (Some(_), None) => {
            ConfidenceRelationship::BaselineOnly
        }

        (None, Some(_)) => {
            ConfidenceRelationship::CandidateOnly
        }

        (Some(baseline_confidence), Some(candidate_confidence)) => {
            let baseline_lower =
                baseline_confidence.lower.get();
            let baseline_upper =
                baseline_confidence.upper.get();

            let candidate_lower =
                candidate_confidence.lower.get();
            let candidate_upper =
                candidate_confidence.upper.get();

            if baseline_upper < candidate_lower
                || candidate_upper < baseline_lower
            {
                ConfidenceRelationship::Disjoint
            } else if baseline_upper == candidate_lower
                || candidate_upper == baseline_lower
            {
                ConfidenceRelationship::Touching
            } else {
                ConfidenceRelationship::Overlapping
            }
        }
    }
}

// =============================================================================
// Conclusion logic
// =============================================================================

fn determine_conclusion(
    baseline: f64,
    candidate: f64,
    direction: MetricDirection,
    within_tolerance: bool,
    confidence_relationship: ConfidenceRelationship,
    uncertainty_separation: Option<UncertaintySeparation>,
) -> ComparisonConclusion {
    if within_tolerance {
        return ComparisonConclusion::NoMaterialChange;
    }

    if direction == MetricDirection::Neutral {
        return ComparisonConclusion::Neutral;
    }

    let numerically_better = match direction {
        MetricDirection::HigherIsBetter => {
            candidate > baseline
        }

        MetricDirection::LowerIsBetter => {
            candidate < baseline
        }

        MetricDirection::Neutral => false,
    };

    let numerically_worse = match direction {
        MetricDirection::HigherIsBetter => {
            candidate < baseline
        }

        MetricDirection::LowerIsBetter => {
            candidate > baseline
        }

        MetricDirection::Neutral => false,
    };

    // If the supplied confidence intervals are disjoint, the directional
    // conclusion has substantially stronger evidence than a purely numerical
    // comparison. We still do not call it "statistically significant", because
    // confidence-interval semantics and hypothesis-test semantics depend on the
    // underlying experiment.
    if matches!(
        confidence_relationship,
        ConfidenceRelationship::Disjoint
    ) {
        if numerically_better {
            return ComparisonConclusion::Improvement;
        }

        if numerically_worse {
            return ComparisonConclusion::Regression;
        }
    }

    // If both uncertainties exist, use the explicit separation diagnostic.
    //
    // Again, this is not a hypothesis test. It simply prevents the comparison
    // layer from claiming more certainty than its inputs support.
    if let Some(separation) = uncertainty_separation {
        if separation.exceeds_threshold {
            if numerically_better {
                return ComparisonConclusion::Improvement;
            }

            if numerically_worse {
                return ComparisonConclusion::Regression;
            }
        }

        return ComparisonConclusion::DifferenceWithoutStatisticalConclusion;
    }

    // No statistical information was supplied. The comparison is still useful
    // as a numerical directional comparison, but it is explicitly classified
    // as unresolved rather than pretending the difference is statistically
    // established.
    ComparisonConclusion::DifferenceWithoutStatisticalConclusion
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::benchmarking::core::metric::{
        ConfidenceMethod,
        MetricConfidence,
        MetricKind,
        MetricUnit,
    };

    fn quantum_volume(value: f64) -> Metric {
        Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            value,
        )
        .expect("valid metric")
    }

    fn execution_time(value: f64) -> Metric {
        Metric::new(
            MetricKind::ExecutionTime,
            MetricUnit::Seconds,
            value,
        )
        .expect("valid metric")
    }

    fn probability(value: f64) -> Metric {
        Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            value,
        )
        .expect("valid metric")
    }

    #[test]
    fn higher_is_better_detects_improvement_when_statistically_supported() {
        let baseline = quantum_volume(32.0);
        let candidate = quantum_volume(64.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::DifferenceWithoutStatisticalConclusion
        );

        assert_eq!(
            comparison.absolute_difference,
            32.0
        );
    }

    #[test]
    fn higher_is_better_detects_improvement_with_disjoint_confidence_intervals() {
        let baseline = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            32.0,
        )
        .expect("valid baseline")
        .with_confidence(
            MetricConfidence::new(
                0.95,
                30.0,
                34.0,
                ConfidenceMethod::Wilson,
            )
            .expect("valid confidence"),
        )
        .expect("confidence should contain value");

        let candidate = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            64.0,
        )
        .expect("valid candidate")
        .with_confidence(
            MetricConfidence::new(
                0.95,
                62.0,
                66.0,
                ConfidenceMethod::Wilson,
            )
            .expect("valid confidence"),
        )
        .expect("confidence should contain value");

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.confidence_relationship,
            ConfidenceRelationship::Disjoint
        );

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::Improvement
        );

        assert!(comparison.is_improvement());
    }

    #[test]
    fn lower_is_better_detects_improvement() {
        let baseline = execution_time(10.0);
        let candidate = execution_time(5.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.direction,
            MetricDirection::LowerIsBetter
        );

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::DifferenceWithoutStatisticalConclusion
        );

        assert_eq!(
            comparison.absolute_difference,
            -5.0
        );
    }

    #[test]
    fn neutral_metric_never_claims_improvement() {
        let baseline = Metric::new(
            MetricKind::ObjectiveValue,
            MetricUnit::Dimensionless,
            10.0,
        )
        .expect("valid metric")
        .with_direction(MetricDirection::Neutral);

        let candidate = Metric::new(
            MetricKind::ObjectiveValue,
            MetricUnit::Dimensionless,
            20.0,
        )
        .expect("valid metric")
        .with_direction(MetricDirection::Neutral);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::Neutral
        );
    }

    #[test]
    fn effectively_equal_values_are_no_material_change() {
        let baseline = quantum_volume(32.0);
        let candidate = quantum_volume(
            32.0 + 1.0e-13,
        );

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::NoMaterialChange
        );

        assert!(comparison.within_tolerance);
    }

    #[test]
    fn relative_change_is_calculated_from_absolute_baseline_scale() {
        let baseline = execution_time(-10.0);
        let candidate = execution_time(-5.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        match comparison.relative_change {
            RelativeChange::Defined { fraction } => {
                assert_eq!(fraction, 0.5);
            }

            _ => panic!(
                "expected defined relative change"
            ),
        }
    }

    #[test]
    fn relative_change_is_undefined_for_nonzero_from_zero() {
        let baseline = execution_time(0.0);
        let candidate = execution_time(1.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.relative_change,
            RelativeChange::UndefinedFromZeroBaseline
        );

        assert_eq!(comparison.ratio, None);
    }

    #[test]
    fn relative_change_is_both_zero_when_both_are_zero() {
        let baseline = execution_time(0.0);
        let candidate = execution_time(0.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.relative_change,
            RelativeChange::BothZero
        );

        assert_eq!(comparison.conclusion,
            ComparisonConclusion::NoMaterialChange);
    }

    #[test]
    fn metric_kind_mismatch_is_rejected() {
        let baseline = quantum_volume(32.0);
        let candidate = probability(0.9);

        let error = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect_err("different kinds must be rejected");

        assert!(matches!(
            error,
            MetricComparisonError::MetricKindMismatch { .. }
        ));
    }

    #[test]
    fn metric_unit_mismatch_is_rejected() {
        let baseline = Metric::new(
            MetricKind::ExecutionTime,
            MetricUnit::Seconds,
            1.0,
        )
        .expect("valid baseline");

        let candidate = Metric::new(
            MetricKind::ExecutionTime,
            MetricUnit::Milliseconds,
            1000.0,
        )
        .expect("valid candidate");

        let error = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect_err("different units must be rejected");

        assert!(matches!(
            error,
            MetricComparisonError::MetricUnitMismatch { .. }
        ));
    }

    #[test]
    fn direction_mismatch_is_rejected() {
        let baseline = quantum_volume(32.0);

        let candidate = quantum_volume(64.0)
            .with_direction(MetricDirection::LowerIsBetter);

        let error = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect_err("different directions must be rejected");

        assert!(matches!(
            error,
            MetricComparisonError::MetricDirectionMismatch { .. }
        ));
    }

    #[test]
    fn confidence_intervals_are_classified_as_disjoint() {
        let baseline = probability(0.60)
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    0.55,
                    0.65,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let candidate = probability(0.80)
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    0.75,
                    0.85,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.confidence_relationship,
            ConfidenceRelationship::Disjoint
        );

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::Improvement
        );
    }

    #[test]
    fn overlapping_confidence_intervals_do_not_claim_statistical_improvement() {
        let baseline = probability(0.60)
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    0.50,
                    0.70,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let candidate = probability(0.65)
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    0.55,
                    0.75,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.confidence_relationship,
            ConfidenceRelationship::Overlapping
        );

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::DifferenceWithoutStatisticalConclusion
        );
    }

    #[test]
    fn uncertainty_separation_is_calculated() {
        let baseline = probability(0.50)
            .with_uncertainty(0.01)
            .expect("valid uncertainty");

        let candidate = probability(0.60)
            .with_uncertainty(0.01)
            .expect("valid uncertainty");

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        let separation = comparison
            .uncertainty_separation
            .expect("separation should exist");

        assert!(separation.score > 2.0);
        assert!(separation.exceeds_threshold);
    }

    #[test]
    fn metric_set_comparison_matches_by_identity() {
        let baseline = vec![
            quantum_volume(32.0),
            execution_time(10.0),
        ];

        let candidate = vec![
            execution_time(5.0),
            quantum_volume(64.0),
        ];

        let comparison = compare_metric_sets(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("set comparison should succeed");

        assert_eq!(
            comparison.comparisons.len(),
            2
        );

        assert!(
            comparison.baseline_only.is_empty()
        );

        assert!(
            comparison.candidate_only.is_empty()
        );
    }

    #[test]
    fn metric_set_comparison_reports_baseline_only_metrics() {
        let baseline = vec![
            quantum_volume(32.0),
        ];

        let candidate = vec![
            execution_time(5.0),
        ];

        let comparison = compare_metric_sets(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("set comparison should succeed");

        assert_eq!(
            comparison.comparisons.len(),
            0
        );

        assert_eq!(
            comparison.baseline_only.len(),
            1
        );

        assert_eq!(
            comparison.candidate_only.len(),
            1
        );
    }

    #[test]
    fn duplicate_metric_identity_is_rejected() {
        let baseline = vec![
            quantum_volume(32.0),
            quantum_volume(64.0),
        ];

        let candidate = vec![
            quantum_volume(128.0),
        ];

        let error = compare_metric_sets(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect_err(
            "duplicate identities must be rejected",
        );

        assert!(matches!(
            error,
            MetricComparisonError::DuplicateMetricIdentity { .. }
        ));
    }

    #[test]
    fn custom_direction_is_respected() {
        let baseline = Metric::new(
            MetricKind::ObjectiveValue,
            MetricUnit::Dimensionless,
            10.0,
        )
        .expect("valid metric")
        .with_direction(
            MetricDirection::LowerIsBetter,
        );

        let candidate = Metric::new(
            MetricKind::ObjectiveValue,
            MetricUnit::Dimensionless,
            5.0,
        )
        .expect("valid metric")
        .with_direction(
            MetricDirection::LowerIsBetter,
        );

        let baseline = baseline
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    9.0,
                    11.0,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let candidate = candidate
            .with_confidence(
                MetricConfidence::new(
                    0.95,
                    4.0,
                    6.0,
                    ConfidenceMethod::Wilson,
                )
                .expect("valid confidence"),
            )
            .expect("confidence should contain value");

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.conclusion,
            ComparisonConclusion::Improvement
        );
    }

    #[test]
    fn confidence_relationship_is_unavailable_without_intervals() {
        let baseline = quantum_volume(32.0);
        let candidate = quantum_volume(64.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.confidence_relationship,
            ConfidenceRelationship::Unavailable
        );
    }

    #[test]
    fn policy_can_disable_direction_matching() {
        let baseline = quantum_volume(32.0);

        let candidate = quantum_volume(64.0)
            .with_direction(
                MetricDirection::LowerIsBetter,
            );

        let policy = ComparisonPolicy {
            require_same_direction: false,
            ..ComparisonPolicy::default()
        };

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &policy,
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.direction,
            MetricDirection::LowerIsBetter
        );
    }

    #[test]
    fn policy_rejects_invalid_tolerance() {
        let policy = ComparisonPolicy {
            absolute_tolerance: -1.0,
            ..ComparisonPolicy::default()
        };

        let error = policy
            .validate()
            .expect_err("invalid tolerance must fail");

        assert!(matches!(
            error,
            MetricComparisonError::InvalidPolicy { .. }
        ));
    }

    #[test]
    fn metric_identity_is_stable() {
        let identity = MetricIdentity::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
        );

        assert_eq!(
            identity.id(),
            "quantum_volume:dimensionless"
        );
    }

    #[test]
    fn ratio_is_calculated_when_baseline_is_nonzero() {
        let baseline = quantum_volume(32.0);
        let candidate = quantum_volume(64.0);

        let comparison = compare_metrics(
            &baseline,
            &candidate,
            &ComparisonPolicy::default(),
        )
        .expect("comparison should succeed");

        assert_eq!(
            comparison.ratio,
            Some(2.0)
        );
    }

    #[test]
    fn set_summary_counts_are_correct() {
        let baseline = vec![
            Metric::new(
                MetricKind::QuantumVolume,
                MetricUnit::Dimensionless,
                32.0,
            )
            .expect("valid metric"),
        ];

        let candidate = vec![
            Metric::new(
                MetricKind::QuantumVolume,
                MetricUnit::Dimensionless,
                64.0,
            )
            .expect("valid metric"),
        ];

        let policy = ComparisonPolicy {
            calculate_uncertainty_separation: false,
            ..ComparisonPolicy::default()
        };

        let comparison = compare_metric_sets(
            &baseline,
            &candidate,
            &policy,
        )
        .expect("set comparison should succeed");

        assert_eq!(
            comparison.comparisons.len(),
            1
        );

        assert_eq!(
            comparison.statistically_unresolved_count,
            1
        );

        assert_eq!(
            comparison.improvement_count,
            0
        );
    }
}