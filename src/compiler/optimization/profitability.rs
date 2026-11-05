
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization Profitability.
//!
//! Path:
//!     src/compiler/optimization/profitability.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! Determine whether an optimization candidate is expected to improve an
//! explicitly selected cost metric relative to a baseline.
//!
//! # Architectural authorities
//!
//! - `super::planning`: optimization candidates and transformation contracts.
//! - `super::cost_model`: cost metrics, units, estimates and uncertainty.
//! - `super::policies`: optimization policy configuration.
//! - `crate::zuir::pass`: canonical pass contracts and execution.
//! - `crate::optimizer`: canonical optimization implementation.
//!
//! # Ownership
//!
//! This module owns profitability assessment only.
//!
//! It does not establish semantic legality, execute transformations,
//! verify IR, or guarantee actual runtime performance.
//!
//! # Numerical contract
//!
//! All cost estimates are finite and non-negative.
//! Cost units must match exactly.
//! Arithmetic overflow is reported.
//! Missing estimates are errors, never implicit zero.
//! Uncertainty is considered explicitly.
//!
//! # Scalability
//!
//! No universal limits are imposed on:
//! - optimization candidates;
//! - cost metrics;
//! - computing domains;
//! - target types;
//! - workload dimensions;
//! - optimization objectives.
//!
//! Actual memory, arithmetic and configured resource constraints remain
//! applicable.
//!
//! # Safety
//!
//! Safe Rust only. Unsafe Rust is forbidden.

use std::error::Error;
use std::fmt;

use super::cost_model::{
    CostEstimate,
    MetricId,
    Quantity,
    UnitId,
};

use super::planning::{
    OptimizationCandidate,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Stable public profitability API version.
pub const PROFITABILITY_API_VERSION: u16 = 1;

// ============================================================================
// RESULT
// ============================================================================

/// Result type for profitability operations.
pub type ProfitabilityResult<T> = Result<T, ProfitabilityError>;

// ============================================================================
// OBJECTIVE DIRECTION
// ============================================================================

/// Direction in which an objective should improve.
///
/// This is deliberately separate from the metric identifier.
/// A metric name alone does not determine whether larger or smaller
/// values are desirable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveDirection {
    /// Lower values are preferable.
    Minimize,

    /// Higher values are preferable.
    Maximize,
}

// ============================================================================
// UNCERTAINTY POLICY
// ============================================================================

/// Determines how estimate uncertainty affects profitability.
///
/// Uncertainty is not discarded or silently treated as zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertaintyMode {
    /// Compare expected values only.
    ///
    /// This is appropriate only when the caller explicitly accepts
    /// expected-value-based decisions.
    ExpectedValue,

    /// Require the candidate to remain beneficial under the
    /// conservative bounds of both estimates.
    ///
    /// This is the default.
    Conservative,
}

// ============================================================================
// PROFITABILITY POLICY
// ============================================================================

/// Configurable profitability evaluation policy.
///
/// The minimum improvement is expressed in the same unit as the
/// objective metric. The evaluator checks that unit before comparing
/// the threshold.
///
/// No universal threshold is imposed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProfitabilityPolicy {
    /// Minimum acceptable absolute improvement.
    ///
    /// Zero means that a strictly positive improvement is sufficient.
    pub minimum_improvement: Quantity,

    /// How uncertainty is treated.
    pub uncertainty_mode: UncertaintyMode,
}

impl ProfitabilityPolicy {
    /// Creates a profitability policy.
    #[must_use]
    pub const fn new(
        minimum_improvement: Quantity,
        uncertainty_mode: UncertaintyMode,
    ) -> Self {
        Self {
            minimum_improvement,
            uncertainty_mode,
        }
    }

    /// Creates a conservative policy with no additional improvement
    /// threshold.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            minimum_improvement: Quantity::zero(),
            uncertainty_mode: UncertaintyMode::Conservative,
        }
    }

    /// Creates an expected-value policy with no additional threshold.
    #[must_use]
    pub const fn expected_value() -> Self {
        Self {
            minimum_improvement: Quantity::zero(),
            uncertainty_mode: UncertaintyMode::ExpectedValue,
        }
    }
}

impl Default for ProfitabilityPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// DECISION
// ============================================================================

/// Result of evaluating optimization profitability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfitabilityStatus {
    /// The improvement satisfies the selected profitability policy.
    Profitable,

    /// The expected improvement does not exceed the required threshold.
    NotProfitable,

    /// Expected improvement exists, but uncertainty prevents a
    /// conservative profitability conclusion.
    Indeterminate,
}

impl ProfitabilityStatus {
    /// Returns whether the profitability policy is satisfied.
    ///
    /// This does not establish transformation legality.
    #[must_use]
    pub const fn is_profitable(self) -> bool {
        matches!(self, Self::Profitable)
    }

    /// Returns whether the result requires further evidence.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

// ============================================================================
// SIGNED IMPROVEMENT
// ============================================================================

/// Signed improvement in the objective's metric unit.
///
/// Positive values indicate improvement.
/// Negative values indicate regression.
/// Zero indicates no improvement.
///
/// A signed value is necessary because `Quantity` intentionally
/// represents only non-negative costs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Improvement {
    value: f64,
}

impl Improvement {
    /// Creates a finite signed improvement.
    fn new(value: f64) -> ProfitabilityResult<Self> {
        if !value.is_finite() {
            return Err(ProfitabilityError::ArithmeticOverflow);
        }

        Ok(Self { value })
    }

    /// Returns the signed improvement.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.value
    }

    /// Returns whether the improvement is positive.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.value > 0.0
    }

    /// Returns whether the improvement is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.value == 0.0
    }

    /// Returns whether the improvement is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.value < 0.0
    }
}

// ============================================================================
// PROFITABILITY ASSESSMENT
// ============================================================================

/// Complete profitability assessment for one optimization candidate.
///
/// The result is analytical evidence. It is not a performance guarantee
/// and does not authorize execution of a transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitabilityAssessment {
    /// Stable optimization pass identity.
    pub pass_id: String,

    /// Objective metric.
    pub metric: MetricId,

    /// Unit of the objective metric.
    pub unit: UnitId,

    /// Direction of improvement.
    pub direction: ObjectiveDirection,

    /// Baseline expected value.
    pub baseline_expected: Quantity,

    /// Candidate expected value.
    pub candidate_expected: Quantity,

    /// Expected signed improvement.
    pub expected_improvement: Improvement,

    /// Conservative signed improvement.
    ///
    /// For minimization:
    ///     baseline lower bound - candidate upper bound
    ///
    /// For maximization:
    ///     candidate lower bound - baseline upper bound
    pub conservative_improvement: Improvement,

    /// Minimum required improvement.
    pub minimum_improvement: Quantity,

    /// Uncertainty policy used.
    pub uncertainty_mode: UncertaintyMode,

    /// Final profitability decision.
    pub status: ProfitabilityStatus,
}

impl ProfitabilityAssessment {
    /// Returns whether the optimization satisfies the profitability policy.
    #[must_use]
    pub fn is_profitable(&self) -> bool {
        self.status.is_profitable()
    }

    /// Returns whether the result needs additional evidence.
    #[must_use]
    pub fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }

    /// Returns the expected improvement as a signed value.
    #[must_use]
    pub fn expected_improvement_value(&self) -> f64 {
        self.expected_improvement.get()
    }

    /// Returns the conservative improvement as a signed value.
    #[must_use]
    pub fn conservative_improvement_value(&self) -> f64 {
        self.conservative_improvement.get()
    }
}

// ============================================================================
// EVALUATOR
// ============================================================================

/// Evaluates optimization profitability.
///
/// This type is stateless. Callers can reuse it across optimization
/// candidates and compilation sessions.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProfitabilityEvaluator;

impl ProfitabilityEvaluator {
    /// Creates a profitability evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates one candidate against baseline and candidate estimates.
    ///
    /// The candidate is validated using the existing planning contract.
    ///
    /// The caller must separately establish that:
    ///
    /// - the transformation is legal;
    /// - required analyses are available;
    /// - preconditions hold;
    /// - semantic-preservation obligations are satisfied;
    /// - input and output IR verification requirements are met.
    ///
    /// Those responsibilities remain with the canonical planner,
    /// pass manager and verifier.
    pub fn evaluate(
        &self,
        candidate: &OptimizationCandidate,
        baseline: &CostEstimate,
        candidate_estimate: &CostEstimate,
        metric: &MetricId,
        direction: ObjectiveDirection,
        policy: ProfitabilityPolicy,
    ) -> ProfitabilityResult<ProfitabilityAssessment> {
        candidate.validate().map_err(|error| {
            ProfitabilityError::InvalidCandidate {
                pass_id: candidate.pass_id().to_owned(),
                reason: error.to_string(),
            }
        })?;

        self.evaluate_estimates(
            candidate.pass_id(),
            baseline,
            candidate_estimate,
            metric,
            direction,
            policy,
        )
    }

    /// Evaluates two cost estimates without requiring a pass candidate.
    ///
    /// This method is useful for evaluating alternative optimization
    /// strategies before constructing an `OptimizationCandidate`.
    ///
    /// It does not establish legality or authorize execution.
    pub fn evaluate_estimates(
        &self,
        pass_id: &str,
        baseline: &CostEstimate,
        candidate: &CostEstimate,
        metric: &MetricId,
        direction: ObjectiveDirection,
        policy: ProfitabilityPolicy,
    ) -> ProfitabilityResult<ProfitabilityAssessment> {
        validate_identifier(pass_id, "pass")?;

        let baseline_value = baseline
            .get(metric)
            .ok_or_else(|| ProfitabilityError::MissingMetric {
                metric: metric.as_str().to_owned(),
                estimate: "baseline".to_owned(),
            })?;

        let candidate_value = candidate
            .get(metric)
            .ok_or_else(|| ProfitabilityError::MissingMetric {
                metric: metric.as_str().to_owned(),
                estimate: "candidate".to_owned(),
            })?;

        let baseline_unit = baseline
            .unit(metric)
            .ok_or_else(|| ProfitabilityError::MissingUnit {
                metric: metric.as_str().to_owned(),
                estimate: "baseline".to_owned(),
            })?;

        let candidate_unit = candidate
            .unit(metric)
            .ok_or_else(|| ProfitabilityError::MissingUnit {
                metric: metric.as_str().to_owned(),
                estimate: "candidate".to_owned(),
            })?;

        if baseline_unit != candidate_unit {
            return Err(ProfitabilityError::UnitMismatch {
                metric: metric.as_str().to_owned(),
                baseline_unit: baseline_unit.as_str().to_owned(),
                candidate_unit: candidate_unit.as_str().to_owned(),
            });
        }

        let baseline_expected = baseline_value.expected;
        let candidate_expected = candidate_value.expected;

        let expected_improvement = match direction {
            ObjectiveDirection::Minimize => {
                baseline_expected.get() - candidate_expected.get()
            }

            ObjectiveDirection::Maximize => {
                candidate_expected.get() - baseline_expected.get()
            }
        };

        let baseline_lower = checked_lower_bound(
            baseline_expected,
            baseline_value.uncertainty,
        )?;

        let baseline_upper = checked_upper_bound(
            baseline_expected,
            baseline_value.uncertainty,
        )?;

        let candidate_lower = checked_lower_bound(
            candidate_expected,
            candidate_value.uncertainty,
        )?;

        let candidate_upper = checked_upper_bound(
            candidate_expected,
            candidate_value.uncertainty,
        )?;

        let conservative_improvement = match direction {
            ObjectiveDirection::Minimize => {
                baseline_lower - candidate_upper
            }

            ObjectiveDirection::Maximize => {
                candidate_lower - baseline_upper
            }
        };

        let expected_improvement = Improvement::new(
            expected_improvement,
        )?;

        let conservative_improvement = Improvement::new(
            conservative_improvement,
        )?;

        let threshold = policy.minimum_improvement.get();

        let status = match policy.uncertainty_mode {
            UncertaintyMode::ExpectedValue => {
                if expected_improvement.get() > threshold {
                    ProfitabilityStatus::Profitable
                } else {
                    ProfitabilityStatus::NotProfitable
                }
            }

            UncertaintyMode::Conservative => {
                if conservative_improvement.get() > threshold {
                    ProfitabilityStatus::Profitable
                } else if expected_improvement.get() > threshold {
                    ProfitabilityStatus::Indeterminate
                } else {
                    ProfitabilityStatus::NotProfitable
                }
            }
        };

        Ok(ProfitabilityAssessment {
            pass_id: pass_id.to_owned(),
            metric: metric.clone(),
            unit: baseline_unit.clone(),
            direction,
            baseline_expected,
            candidate_expected,
            expected_improvement,
            conservative_improvement,
            minimum_improvement: policy.minimum_improvement,
            uncertainty_mode: policy.uncertainty_mode,
            status,
        })
    }
}

// ============================================================================
// NUMERICAL HELPERS
// ============================================================================

/// Computes a non-negative lower confidence bound.
///
/// Costs cannot be negative, so the lower bound is clamped to zero.
fn checked_lower_bound(
    expected: Quantity,
    uncertainty: Quantity,
) -> ProfitabilityResult<f64> {
    let value = expected.get() - uncertainty.get();

    if !value.is_finite() {
        return Err(ProfitabilityError::ArithmeticOverflow);
    }

    Ok(value.max(0.0))
}

/// Computes an upper uncertainty bound.
fn checked_upper_bound(
    expected: Quantity,
    uncertainty: Quantity,
) -> ProfitabilityResult<f64> {
    let value = expected.get() + uncertainty.get();

    if !value.is_finite() {
        return Err(ProfitabilityError::ArithmeticOverflow);
    }

    Ok(value)
}

/// Validates an extensible identifier.
///
/// No arbitrary identifier-length limit is imposed.
fn validate_identifier(
    value: &str,
    kind: &str,
) -> ProfitabilityResult<()> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(ProfitabilityError::InvalidIdentifier {
            kind: kind.to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// ERRORS
// ============================================================================

/// Profitability evaluation errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfitabilityError {
    /// Invalid pass or other identifier.
    InvalidIdentifier {
        /// Identifier category.
        kind: String,
    },

    /// Candidate failed its existing planning validation.
    InvalidCandidate {
        /// Pass identity.
        pass_id: String,

        /// Underlying validation failure.
        reason: String,
    },

    /// A required metric was not present in an estimate.
    MissingMetric {
        /// Metric identity.
        metric: String,

        /// Estimate in which it was missing.
        estimate: String,
    },

    /// A metric did not have a declared unit.
    MissingUnit {
        /// Metric identity.
        metric: String,

        /// Estimate in which the unit was missing.
        estimate: String,
    },

    /// Baseline and candidate use different units.
    UnitMismatch {
        /// Metric identity.
        metric: String,

        /// Baseline unit.
        baseline_unit: String,

        /// Candidate unit.
        candidate_unit: String,
    },

    /// Arithmetic exceeded the finite numerical representation.
    ArithmeticOverflow,
}

impl fmt::Display for ProfitabilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind } => {
                write!(formatter, "invalid {kind} identifier")
            }

            Self::InvalidCandidate {
                pass_id,
                reason,
            } => {
                write!(
                    formatter,
                    "optimization candidate {pass_id:?} is invalid: {reason}"
                )
            }

            Self::MissingMetric {
                metric,
                estimate,
            } => {
                write!(
                    formatter,
                    "metric {metric:?} is missing from the {estimate} estimate"
                )
            }

            Self::MissingUnit {
                metric,
                estimate,
            } => {
                write!(
                    formatter,
                    "metric {metric:?} has no unit in the {estimate} estimate"
                )
            }

            Self::UnitMismatch {
                metric,
                baseline_unit,
                candidate_unit,
            } => {
                write!(
                    formatter,
                    "metric {metric:?} has incompatible units: \
                     baseline={baseline_unit:?}, \
                     candidate={candidate_unit:?}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "profitability arithmetic exceeded finite numerical range",
                )
            }
        }
    }
}

impl Error for ProfitabilityError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::cost_model::{
        CostProfile,
        CostCoefficient,
        CostModel,
        AnalyticalCostModel,
        ModelId,
        Objective,
        ObjectiveWeight,
        ProfileId,
        Workload,
    };

    fn metric(value: &str) -> MetricId {
        MetricId::new(value).expect("valid metric")
    }

    fn unit(value: &str) -> UnitId {
        UnitId::new(value).expect("valid unit")
    }

    fn quantity(value: f64) -> Quantity {
        Quantity::new(value).expect("valid quantity")
    }

    fn estimate(
        profile_name: &str,
        expected: f64,
        uncertainty: f64,
    ) -> CostEstimate {
        let metric = metric("execution.latency");

        let mut profile = CostProfile::new(
            ProfileId::new(profile_name).expect("valid profile"),
            "1",
        )
        .expect("valid profile");

        profile
            .register(
                metric.clone(),
                unit("seconds"),
                CostCoefficient::new(
                    quantity(expected),
                    quantity(uncertainty),
                ),
            )
            .expect("register coefficient");

        let mut workload = Workload::new();
        workload.set(metric, quantity(1.0));

        let model = AnalyticalCostModel::new(
            ModelId::new("test.analytical").expect("valid model"),
        );

        model
            .estimate(&profile, &workload)
            .expect("estimate")
    }

    #[test]
    fn conservative_minimization_accepts_robust_improvement() {
        let baseline = estimate("baseline", 100.0, 2.0);
        let candidate = estimate("candidate", 70.0, 2.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::conservative(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::Profitable
        );

        assert_eq!(
            result.expected_improvement_value(),
            30.0
        );

        assert_eq!(
            result.conservative_improvement_value(),
            26.0
        );
    }

    #[test]
    fn conservative_minimization_rejects_regression() {
        let baseline = estimate("baseline", 50.0, 1.0);
        let candidate = estimate("candidate", 70.0, 1.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::conservative(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::NotProfitable
        );
    }

    #[test]
    fn uncertainty_can_make_decision_indeterminate() {
        let baseline = estimate("baseline", 100.0, 20.0);
        let candidate = estimate("candidate", 90.0, 20.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::conservative(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::Indeterminate
        );
    }

    #[test]
    fn expected_value_policy_uses_expected_values() {
        let baseline = estimate("baseline", 100.0, 20.0);
        let candidate = estimate("candidate", 90.0, 20.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::expected_value(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::Profitable
        );
    }

    #[test]
    fn maximization_accepts_improvement() {
        let baseline = estimate("baseline", 50.0, 1.0);
        let candidate = estimate("candidate", 80.0, 1.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "tensor.throughput.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Maximize,
                ProfitabilityPolicy::conservative(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::Profitable
        );

        assert_eq!(
            result.expected_improvement_value(),
            30.0
        );
    }

    #[test]
    fn minimum_improvement_threshold_is_respected() {
        let baseline = estimate("baseline", 100.0, 1.0);
        let candidate = estimate("candidate", 95.0, 1.0);

        let policy = ProfitabilityPolicy::new(
            quantity(10.0),
            UncertaintyMode::ExpectedValue,
        );

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                policy,
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::NotProfitable
        );
    }

    #[test]
    fn missing_metric_is_an_error() {
        let baseline = estimate("baseline", 100.0, 1.0);
        let candidate = estimate("candidate", 90.0, 1.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("future.custom.metric"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::default(),
            );

        assert!(matches!(
            result,
            Err(ProfitabilityError::MissingMetric { .. })
        ));
    }

    #[test]
    fn unit_mismatch_is_rejected() {
        let baseline = estimate("baseline", 100.0, 1.0);

        let metric_id = metric("execution.latency");

        let mut profile = CostProfile::new(
            ProfileId::new("candidate").expect("profile"),
            "1",
        )
        .expect("profile");

        profile
            .register(
                metric_id.clone(),
                unit("milliseconds"),
                CostCoefficient::new(
                    quantity(90.0),
                    quantity(1.0),
                ),
            )
            .expect("register");

        let mut workload = Workload::new();
        workload.set(metric_id, quantity(1.0));

        let model = AnalyticalCostModel::new(
            ModelId::new("test.analytical").expect("model"),
        );

        let candidate = model
            .estimate(&profile, &workload)
            .expect("estimate");

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::default(),
            );

        assert!(matches!(
            result,
            Err(ProfitabilityError::UnitMismatch { .. })
        ));
    }

    #[test]
    fn objective_threshold_uses_the_metric_unit() {
        let baseline = estimate("baseline", 100.0, 1.0);
        let candidate = estimate("candidate", 90.0, 1.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::new(
                    quantity(5.0),
                    UncertaintyMode::ExpectedValue,
                ),
            )
            .expect("assessment");

        assert_eq!(
            result.minimum_improvement.get(),
            5.0
        );

        assert_eq!(
            result.status,
            ProfitabilityStatus::Profitable
        );
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let baseline = estimate(
            "baseline",
            f64::MAX,
            f64::MAX,
        );

        let candidate = estimate(
            "candidate",
            1.0,
            0.0,
        );

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::default(),
            );

        assert!(matches!(
            result,
            Err(ProfitabilityError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn empty_workload_cost_estimates_are_not_assumed() {
        let baseline = estimate("baseline", 0.0, 0.0);
        let candidate = estimate("candidate", 0.0, 0.0);

        let result = ProfitabilityEvaluator::new()
            .evaluate_estimates(
                "classical.optimize",
                &baseline,
                &candidate,
                &metric("execution.latency"),
                ObjectiveDirection::Minimize,
                ProfitabilityPolicy::default(),
            )
            .expect("assessment");

        assert_eq!(
            result.status,
            ProfitabilityStatus::NotProfitable
        );
    }
}
