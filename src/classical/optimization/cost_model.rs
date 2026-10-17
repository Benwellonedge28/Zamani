
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Optimization Cost Model.
//!
//! Path: `src/classical/optimization/cost_model.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Rank optimization candidates using explicitly supplied cost estimates,
//! optimization objectives, and optional operational budgets.
//!
//! # Architectural authority
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/spec/resources.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/classical/optimization/legality.rs`
//! - `src/classical/optimization/passes.rs`
//! - `src/classical/optimization/vectorization.rs`
//! - `src/classical/execution/requirements.rs`
//! - `src/classical/execution/capabilities.rs`
//! - `src/zuir/`, when selected by the compilation pipeline
//! - `src/quantum/ir/classical/`, the existing classical IR boundary
//!
//! # Ownership
//!
//! This module owns cost dimensions, cost estimates, objective ordering,
//! budget checks, deterministic candidate ranking, and selection reports.
//!
//! It does not own or independently establish semantic legality, target
//! feasibility, resource availability, IR structure, target selection,
//! scheduling, memory allocation, or execution.
//!
//! # Critical correctness rule
//!
//! Cost is not proof of correctness. A candidate is considered for selection
//! only if its legality status is `Legal` and its supplied feasibility status
//! is `Feasible`. Illegal, unproven, infeasible, or unknown-feasibility
//! candidates are never selected.
//!
//! # Scalability
//!
//! There are no universal limits on candidate count, objective count, cost
//! dimensions, machine count, memory size, or operation count. Collections
//! grow according to available resources and the host's representable sizes.
//! Integer arithmetic is checked where totals are computed.
//!
//! # Determinism
//!
//! Objectives are evaluated in caller-supplied order. Ties are resolved by
//! the candidate's stable identifier. Ordered maps and sets are used for
//! reproducible behavior.
//!
//! # Integration status
//!
//! This module is a pure planning component. A canonical-IR adapter or
//! optimization pass must supply estimates and authoritative legality and
//! feasibility results. This module does not mutate IR or execute a pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use super::legality::LegalityStatus;

/// Result type for cost-model operations.
pub type CostModelResult<T> = Result<T, CostModelError>;

/// Validates an extensible textual identifier.
///
/// Empty or whitespace-only identifiers and identifiers containing control
/// characters are rejected. No fixed length limit is imposed.
fn validate_identifier(value: &str, kind: &'static str) -> CostModelResult<()> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(CostModelError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Identifies a measurable cost dimension and its unit.
///
/// The unit is part of the identity so that estimates expressed in different
/// units cannot accidentally be compared as though they were equivalent.
/// Examples include `latency` measured in `ns`, `memory` measured in `bytes`,
/// and extension-defined dimensions with domain-specific units.
///
/// This type does not define a closed universe of cost dimensions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CostDimension {
    name: String,
    unit: String,
}

impl CostDimension {
    /// Creates a cost dimension with an explicit name and unit.
    pub fn new(
        name: impl Into<String>,
        unit: impl Into<String>,
    ) -> CostModelResult<Self> {
        let name = name.into();
        let unit = unit.into();

        validate_identifier(&name, "cost dimension")?;
        validate_identifier(&unit, "cost unit")?;

        Ok(Self { name, unit })
    }

    /// Returns the dimension's stable name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the dimension's unit.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

impl fmt::Display for CostDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.unit)
    }
}

/// Selects the estimate used to compare candidates.
///
/// A policy must choose its comparison basis explicitly. The basis affects
/// ranking only; it never changes the semantics or legality of a candidate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonBasis {
    /// Compare the lower bound of each estimate.
    LowerBound,

    /// Compare the expected estimate when one is available.
    Expected,

    /// Compare the upper bound of each estimate.
    UpperBound,
}

/// An estimate for one cost dimension.
///
/// Estimates can be exact, bounded, symbolic, or unknown. Symbolic and
/// unknown estimates are preserved rather than guessed. Candidates whose
/// objective values cannot be compared under the selected policy are
/// rejected from ranking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CostEstimate {
    kind: CostEstimateKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CostEstimateKind {
    Exact(u128),
    Range {
        lower: u128,
        expected: u128,
        upper: u128,
    },
    Symbolic(String),
    Unknown,
}

impl CostEstimate {
    /// Creates an exact nonnegative cost estimate.
    ///
    /// The value is an amount in the unit defined by its `CostDimension`.
    #[must_use]
    pub fn exact(value: u128) -> Self {
        Self {
            kind: CostEstimateKind::Exact(value),
        }
    }

    /// Creates a bounded estimate.
    ///
    /// The values must satisfy `lower <= expected <= upper`.
    pub fn range(
        lower: u128,
        expected: u128,
        upper: u128,
    ) -> CostModelResult<Self> {
        if lower > expected || expected > upper {
            return Err(CostModelError::InvalidEstimateRange {
                lower,
                expected,
                upper,
            });
        }

        Ok(Self {
            kind: CostEstimateKind::Range {
                lower,
                expected,
                upper,
            },
        })
    }

    /// Creates a symbolic estimate supplied by an analysis or cost provider.
    ///
    /// The expression is opaque to this module. It is not parsed, evaluated,
    /// or treated as a numeric estimate.
    pub fn symbolic(expression: impl Into<String>) -> CostModelResult<Self> {
        let expression = expression.into();
        validate_identifier(&expression, "symbolic cost expression")?;

        Ok(Self {
            kind: CostEstimateKind::Symbolic(expression),
        })
    }

    /// Creates an estimate whose value is not currently known.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            kind: CostEstimateKind::Unknown,
        }
    }

    /// Returns the exact value when this estimate is exact.
    #[must_use]
    pub fn as_exact(&self) -> Option<u128> {
        match self.kind {
            CostEstimateKind::Exact(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the symbolic expression when this estimate is symbolic.
    #[must_use]
    pub fn as_symbolic(&self) -> Option<&str> {
        match &self.kind {
            CostEstimateKind::Symbolic(expression) => Some(expression),
            _ => None,
        }
    }

    /// Returns whether this estimate has a numeric value.
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        matches!(
            self.kind,
            CostEstimateKind::Exact(_) | CostEstimateKind::Range { .. }
        )
    }

    fn comparison_value(&self, basis: ComparisonBasis) -> Option<u128> {
        match self.kind {
            CostEstimateKind::Exact(value) => Some(value),
            CostEstimateKind::Range {
                lower,
                expected,
                upper,
            } => Some(match basis {
                ComparisonBasis::LowerBound => lower,
                ComparisonBasis::Expected => expected,
                ComparisonBasis::UpperBound => upper,
            }),
            CostEstimateKind::Symbolic(_) | CostEstimateKind::Unknown => None,
        }
    }

    fn upper_bound(&self) -> Option<u128> {
        match self.kind {
            CostEstimateKind::Exact(value) => Some(value),
            CostEstimateKind::Range { upper, .. } => Some(upper),
            CostEstimateKind::Symbolic(_) | CostEstimateKind::Unknown => None,
        }
    }
}

/// Authoritative feasibility status supplied by target or resource analysis.
///
/// This status is not inferred from cost estimates. In particular, a cheap
/// candidate is not necessarily feasible.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CandidateFeasibility {
    /// The relevant analysis established feasibility for this candidate.
    Feasible,

    /// The relevant analysis established that this candidate cannot satisfy
    /// the applicable requirements or constraints.
    Infeasible {
        /// Explanation from the responsible analysis.
        reason: String,
    },

    /// Available evidence does not establish feasibility.
    Unknown {
        /// Explanation of the missing or inconclusive evidence.
        reason: String,
    },
}

impl CandidateFeasibility {
    /// Creates a known-feasible status.
    #[must_use]
    pub fn feasible() -> Self {
        Self::Feasible
    }

    /// Creates an infeasible status with a diagnostic explanation.
    #[must_use]
    pub fn infeasible(reason: impl Into<String>) -> Self {
        Self::Infeasible {
            reason: reason.into(),
        }
    }

    /// Creates an unknown status with an explanation.
    #[must_use]
    pub fn unknown(reason: impl Into<String>) -> Self {
        Self::Unknown {
            reason: reason.into(),
        }
    }

    /// Returns true only when feasibility has been established.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        matches!(self, Self::Feasible)
    }
}

/// One optimization candidate with its supplied analysis results.
///
/// Candidate identifiers must be unique within one selection operation.
/// Estimates are keyed by dimension, so a candidate cannot have two
/// competing estimates for the same dimension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OptimizationCandidate {
    id: String,
    legality: LegalityStatus,
    feasibility: CandidateFeasibility,
    estimates: BTreeMap<CostDimension, CostEstimate>,
}

impl OptimizationCandidate {
    /// Creates a candidate with explicit legality and feasibility results.
    pub fn new(
        id: impl Into<String>,
        legality: LegalityStatus,
        feasibility: CandidateFeasibility,
    ) -> CostModelResult<Self> {
        let id = id.into();
        validate_identifier(&id, "candidate identifier")?;

        Ok(Self {
            id,
            legality,
            feasibility,
            estimates: BTreeMap::new(),
        })
    }

    /// Returns the stable candidate identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the supplied legality assessment.
    #[must_use]
    pub const fn legality(&self) -> LegalityStatus {
        self.legality
    }

    /// Returns the supplied feasibility assessment.
    #[must_use]
    pub fn feasibility(&self) -> &CandidateFeasibility {
        &self.feasibility
    }

    /// Returns estimates in deterministic dimension order.
    #[must_use]
    pub fn estimates(&self) -> &BTreeMap<CostDimension, CostEstimate> {
        &self.estimates
    }

    /// Adds or replaces the estimate for a dimension.
    pub fn set_estimate(
        &mut self,
        dimension: CostDimension,
        estimate: CostEstimate,
    ) {
        self.estimates.insert(dimension, estimate);
    }

    /// Returns the estimate for a dimension, if supplied.
    #[must_use]
    pub fn estimate(&self, dimension: &CostDimension) -> Option<&CostEstimate> {
        self.estimates.get(dimension)
    }
}

/// Policy for deterministic cost-based candidate selection.
///
/// Objectives are evaluated lexicographically in the supplied order:
/// the first objective is compared first, then the second objective breaks
/// ties, and so on. Lower numeric values are preferred for every objective.
///
/// Budgets are independent upper bounds. Each budget must be satisfied by a
/// known estimate upper bound; an unknown or symbolic value cannot prove
/// compliance with a numeric budget.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CostModelPolicy {
    objectives: Vec<CostDimension>,
    comparison_basis: ComparisonBasis,
    budgets: BTreeMap<CostDimension, u128>,
}

impl CostModelPolicy {
    /// Creates a policy with at least one ordered objective.
    ///
    /// Duplicate objectives are rejected because they add no useful
    /// tie-breaking information and can conceal configuration mistakes.
    pub fn new(
        objectives: impl IntoIterator<Item = CostDimension>,
        comparison_basis: ComparisonBasis,
    ) -> CostModelResult<Self> {
        let objectives: Vec<_> = objectives.into_iter().collect();

        if objectives.is_empty() {
            return Err(CostModelError::EmptyObjectives);
        }

        let mut seen = BTreeSet::new();
        for objective in &objectives {
            if !seen.insert(objective.clone()) {
                return Err(CostModelError::DuplicateObjective {
                    dimension: objective.clone(),
                });
            }
        }

        Ok(Self {
            objectives,
            comparison_basis,
            budgets: BTreeMap::new(),
        })
    }

    /// Returns objectives in their priority order.
    #[must_use]
    pub fn objectives(&self) -> &[CostDimension] {
        &self.objectives
    }

    /// Returns the comparison basis used for ranking.
    #[must_use]
    pub const fn comparison_basis(&self) -> ComparisonBasis {
        self.comparison_basis
    }

    /// Returns configured numeric budgets.
    #[must_use]
    pub fn budgets(&self) -> &BTreeMap<CostDimension, u128> {
        &self.budgets
    }

    /// Adds or replaces an upper-bound budget for a cost dimension.
    ///
    /// For example, a caller may set a latency budget or a memory budget.
    /// These values are operational policy, not language-level limits.
    pub fn set_budget(
        &mut self,
        dimension: CostDimension,
        maximum: u128,
    ) {
        self.budgets.insert(dimension, maximum);
    }

    /// Builder-style form of [`Self::set_budget`].
    #[must_use]
    pub fn with_budget(
        mut self,
        dimension: CostDimension,
        maximum: u128,
    ) -> Self {
        self.set_budget(dimension, maximum);
        self
    }
}

/// Why a candidate was not admitted to the ranking.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RejectionReason {
    /// The candidate's transformation is not proven legal.
    LegalityNotProven {
        /// Actual legality assessment.
        status: LegalityStatus,
    },

    /// Feasibility has not been established.
    FeasibilityNotEstablished {
        /// Explanation from the supplied feasibility assessment.
        reason: String,
    },

    /// A required objective has no comparable numeric estimate.
    MissingComparableEstimate {
        /// Objective without a comparable value.
        dimension: CostDimension,
    },

    /// No numeric upper bound was available to check a configured budget.
    BudgetCannotBeVerified {
        /// Budget dimension whose bound could not be checked.
        dimension: CostDimension,
    },

    /// The candidate's known upper bound exceeds an explicit budget.
    BudgetExceeded {
        /// Budget dimension that was exceeded.
        dimension: CostDimension,

        /// Candidate's known upper bound.
        upper_bound: u128,

        /// Configured maximum.
        maximum: u128,
    },
}

/// A candidate rejected by the selection policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RejectedCandidate {
    /// Rejected candidate's stable identifier.
    pub candidate_id: String,

    /// Reason the candidate was rejected.
    pub reason: RejectionReason,
}

/// One candidate admitted to the ranking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankedCandidate {
    /// Candidate's stable identifier.
    pub candidate_id: String,

    /// Numeric values used in objective-priority order.
    pub objective_values: Vec<u128>,
}

/// Result of evaluating a set of candidates.
///
/// `selected` is absent when no candidate satisfies the policy. Rejections
/// remain available so that callers can produce diagnostics or try another
/// explicitly configured policy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionReport {
    /// Selected candidate, if any.
    pub selected: Option<OptimizationCandidate>,

    /// Eligible candidates in deterministic ranking order.
    pub ranking: Vec<RankedCandidate>,

    /// Candidates excluded from ranking, in input order.
    pub rejected: Vec<RejectedCandidate>,
}

impl SelectionReport {
    /// Returns whether a candidate was selected.
    #[must_use]
    pub fn has_selection(&self) -> bool {
        self.selected.is_some()
    }
}

/// Errors in policy construction or candidate input.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CostModelError {
    /// An identifier is empty, whitespace-only, or contains control
    /// characters.
    InvalidIdentifier {
        /// Kind of identifier being validated.
        kind: &'static str,

        /// Rejected identifier.
        value: String,
    },

    /// No objectives were supplied.
    EmptyObjectives,

    /// The same objective was supplied more than once.
    DuplicateObjective {
        /// Repeated objective dimension.
        dimension: CostDimension,
    },

    /// A range estimate violates `lower <= expected <= upper`.
    InvalidEstimateRange {
        /// Supplied lower bound.
        lower: u128,

        /// Supplied expected value.
        expected: u128,

        /// Supplied upper bound.
        upper: u128,
    },

    /// Multiple candidates share the same identifier.
    DuplicateCandidate {
        /// Repeated identifier.
        candidate_id: String,
    },
}

impl fmt::Display for CostModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind}: {value:?}")
            }
            Self::EmptyObjectives => {
                f.write_str("cost-model policy requires at least one objective")
            }
            Self::DuplicateObjective { dimension } => {
                write!(f, "duplicate cost objective: {dimension}")
            }
            Self::InvalidEstimateRange {
                lower,
                expected,
                upper,
            } => write!(
                f,
                "invalid cost estimate range: lower={lower}, \
                 expected={expected}, upper={upper}"
            ),
            Self::DuplicateCandidate { candidate_id } => {
                write!(f, "duplicate optimization candidate: {candidate_id}")
            }
        }
    }
}

impl Error for CostModelError {}

/// Selects the best eligible candidate under an explicit policy.
///
/// Eligibility requires both:
///
/// - `LegalityStatus::Legal`;
/// - `CandidateFeasibility::Feasible`.
///
/// Each configured budget is checked against the candidate's known numeric
/// upper bound. Every objective must have a numeric value for the configured
/// comparison basis. Unknown or symbolic objective values are never guessed.
///
/// Ranking is lexicographic according to `policy.objectives()`. When all
/// objective values tie, candidate identifiers break the tie in ascending
/// lexical order. This makes the result independent of hash iteration order.
///
/// The function does not run analyses, infer missing estimates, mutate IR,
/// allocate resources, or execute candidates.
pub fn select_best(
    candidates: impl IntoIterator<Item = OptimizationCandidate>,
    policy: &CostModelPolicy,
) -> CostModelResult<SelectionReport> {
    let mut seen_ids = BTreeSet::new();
    let mut eligible: Vec<(OptimizationCandidate, Vec<u128>)> = Vec::new();
    let mut rejected = Vec::new();

    for candidate in candidates {
        if !seen_ids.insert(candidate.id.clone()) {
            return Err(CostModelError::DuplicateCandidate {
                candidate_id: candidate.id,
            });
        }

        if candidate.legality != LegalityStatus::Legal {
            rejected.push(RejectedCandidate {
                candidate_id: candidate.id.clone(),
                reason: RejectionReason::LegalityNotProven {
                    status: candidate.legality,
                },
            });
            continue;
        }

        match &candidate.feasibility {
            CandidateFeasibility::Feasible => {}
            CandidateFeasibility::Infeasible { reason }
            | CandidateFeasibility::Unknown { reason } => {
                rejected.push(RejectedCandidate {
                    candidate_id: candidate.id.clone(),
                    reason: RejectionReason::FeasibilityNotEstablished {
                        reason: reason.clone(),
                    },
                });
                continue;
            }
        }

        let mut budget_rejection = None;

        for (dimension, maximum) in &policy.budgets {
            let upper_bound = candidate
                .estimates
                .get(dimension)
                .and_then(CostEstimate::upper_bound);

            match upper_bound {
                None => {
                    budget_rejection = Some(
                        RejectionReason::BudgetCannotBeVerified {
                            dimension: dimension.clone(),
                        },
                    );
                    break;
                }
                Some(upper_bound) if upper_bound > *maximum => {
                    budget_rejection = Some(RejectionReason::BudgetExceeded {
                        dimension: dimension.clone(),
                        upper_bound,
                        maximum: *maximum,
                    });
                    break;
                }
                Some(_) => {}
            }
        }

        if let Some(reason) = budget_rejection {
            rejected.push(RejectedCandidate {
                candidate_id: candidate.id.clone(),
                reason,
            });
            continue;
        }

        let mut objective_values = Vec::with_capacity(policy.objectives.len());
        let mut missing_objective = None;

        for dimension in &policy.objectives {
            let value = candidate
                .estimates
                .get(dimension)
                .and_then(|estimate| {
                    estimate.comparison_value(policy.comparison_basis)
                });

            match value {
                Some(value) => objective_values.push(value),
                None => {
                    missing_objective = Some(dimension.clone());
                    break;
                }
            }
        }

        if let Some(dimension) = missing_objective {
            rejected.push(RejectedCandidate {
                candidate_id: candidate.id.clone(),
                reason: RejectionReason::MissingComparableEstimate {
                    dimension,
                },
            });
            continue;
        }

        eligible.push((candidate, objective_values));
    }

    eligible.sort_by(|left, right| {
        left.1
            .cmp(&right.1)
            .then_with(|| left.0.id.cmp(&right.0.id))
    });

    let ranking: Vec<_> = eligible
        .iter()
        .map(|(candidate, objective_values)| RankedCandidate {
            candidate_id: candidate.id.clone(),
            objective_values: objective_values.clone(),
        })
        .collect();

    let selected = eligible.first().map(|(candidate, _)| candidate.clone());

    Ok(SelectionReport {
        selected,
        ranking,
        rejected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dimension(name: &str, unit: &str) -> CostDimension {
        CostDimension::new(name, unit).expect("valid dimension")
    }

    fn candidate(
        id: &str,
        latency: u128,
    ) -> OptimizationCandidate {
        let mut candidate = OptimizationCandidate::new(
            id,
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        candidate.set_estimate(
            dimension("latency", "ns"),
            CostEstimate::exact(latency),
        );

        candidate
    }

    fn latency_policy() -> CostModelPolicy {
        CostModelPolicy::new(
            [dimension("latency", "ns")],
            ComparisonBasis::Expected,
        )
        .expect("valid policy")
    }

    #[test]
    fn selects_lowest_cost_candidate() {
        let report = select_best(
            [candidate("slow", 20), candidate("fast", 5)],
            &latency_policy(),
        )
        .expect("selection succeeds");

        assert_eq!(report.selected.as_ref().map(|c| c.id()), Some("fast"));
        assert_eq!(report.ranking.len(), 2);
    }

    #[test]
    fn ties_are_broken_by_stable_identifier() {
        let report = select_best(
            [candidate("zeta", 5), candidate("alpha", 5)],
            &latency_policy(),
        )
        .expect("selection succeeds");

        assert_eq!(report.selected.as_ref().map(|c| c.id()), Some("alpha"));
    }

    #[test]
    fn illegal_candidate_is_never_selected() {
        let illegal = OptimizationCandidate::new(
            "illegal",
            LegalityStatus::Illegal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        let report = select_best(
            [illegal, candidate("legal", 10)],
            &latency_policy(),
        )
        .expect("selection succeeds");

        assert_eq!(report.selected.as_ref().map(|c| c.id()), Some("legal"));
        assert_eq!(report.rejected.len(), 1);
    }

    #[test]
    fn unproven_legality_is_not_accepted() {
        let unproven = OptimizationCandidate::new(
            "unproven",
            LegalityStatus::Unproven,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        let report = select_best([unproven], &latency_policy())
            .expect("selection report is produced");

        assert!(report.selected.is_none());
        assert_eq!(report.rejected.len(), 1);
    }

    #[test]
    fn unknown_feasibility_is_not_accepted() {
        let unknown = OptimizationCandidate::new(
            "unknown",
            LegalityStatus::Legal,
            CandidateFeasibility::unknown("capabilities not resolved"),
        )
        .expect("valid candidate");

        let report = select_best([unknown], &latency_policy())
            .expect("selection report is produced");

        assert!(report.selected.is_none());
    }

    #[test]
    fn symbolic_objective_is_not_guessed() {
        let mut symbolic = OptimizationCandidate::new(
            "symbolic",
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        symbolic.set_estimate(
            dimension("latency", "ns"),
            CostEstimate::symbolic("measured_latency").expect("valid expression"),
        );

        let report = select_best([symbolic], &latency_policy())
            .expect("selection report is produced");

        assert!(report.selected.is_none());
        assert!(matches!(
            report.rejected[0].reason,
            RejectionReason::MissingComparableEstimate { .. }
        ));
    }

    #[test]
    fn budget_uses_upper_bound_not_expected_value() {
        let mut policy = latency_policy();
        policy.set_budget(dimension("latency", "ns"), 10);

        let mut candidate = OptimizationCandidate::new(
            "wide-range",
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        candidate.set_estimate(
            dimension("latency", "ns"),
            CostEstimate::range(2, 5, 12).expect("valid range"),
        );

        let report = select_best([candidate], &policy)
            .expect("selection report is produced");

        assert!(report.selected.is_none());
        assert!(matches!(
            report.rejected[0].reason,
            RejectionReason::BudgetExceeded {
                upper_bound: 12,
                maximum: 10,
                ..
            }
        ));
    }

    #[test]
    fn unknown_budget_bound_is_not_assumed_to_fit() {
        let policy = latency_policy()
            .with_budget(dimension("latency", "ns"), 100);

        let mut candidate = OptimizationCandidate::new(
            "unknown-cost",
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        candidate.set_estimate(
            dimension("latency", "ns"),
            CostEstimate::unknown(),
        );

        let report = select_best([candidate], &policy)
            .expect("selection report is produced");

        assert!(report.selected.is_none());
        assert!(matches!(
            report.rejected[0].reason,
            RejectionReason::BudgetCannotBeVerified { .. }
        ));
    }

    #[test]
    fn objectives_are_lexicographic() {
        let latency = dimension("latency", "ns");
        let memory = dimension("memory", "bytes");

        let mut first = OptimizationCandidate::new(
            "first",
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");
        first.set_estimate(latency.clone(), CostEstimate::exact(5));
        first.set_estimate(memory.clone(), CostEstimate::exact(100));

        let mut second = OptimizationCandidate::new(
            "second",
            LegalityStatus::Legal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");
        second.set_estimate(latency.clone(), CostEstimate::exact(5));
        second.set_estimate(memory.clone(), CostEstimate::exact(50));

        let policy = CostModelPolicy::new(
            [latency, memory],
            ComparisonBasis::Expected,
        )
        .expect("valid policy");

        let report = select_best([first, second], &policy)
            .expect("selection succeeds");

        assert_eq!(report.selected.as_ref().map(|c| c.id()), Some("second"));
    }

    #[test]
    fn large_cost_values_are_preserved_without_float_conversion() {
        let value = u128::MAX;
        let mut candidate = candidate("large", value);

        assert_eq!(
            candidate
                .estimate(&dimension("latency", "ns"))
                .and_then(CostEstimate::as_exact),
            Some(value)
        );

        let report = select_best([candidate.clone()], &latency_policy())
            .expect("selection succeeds");

        assert_eq!(report.selected, Some(candidate));
    }

    #[test]
    fn duplicate_candidate_ids_are_rejected() {
        let result = select_best(
            [candidate("same", 1), candidate("same", 2)],
            &latency_policy(),
        );

        assert!(matches!(
            result,
            Err(CostModelError::DuplicateCandidate { .. })
        ));
    }

    #[test]
    fn invalid_ranges_are_rejected() {
        assert!(matches!(
            CostEstimate::range(10, 5, 20),
            Err(CostModelError::InvalidEstimateRange { .. })
        ));

        assert!(matches!(
            CostEstimate::range(1, 20, 10),
            Err(CostModelError::InvalidEstimateRange { .. })
        ));
    }

    #[test]
    fn empty_objectives_are_rejected() {
        assert!(matches!(
            CostModelPolicy::new(
                std::iter::empty(),
                ComparisonBasis::Expected
            ),
            Err(CostModelError::EmptyObjectives)
        ));
    }

    #[test]
    fn no_eligible_candidates_produces_diagnostics() {
        let illegal = OptimizationCandidate::new(
            "illegal",
            LegalityStatus::Illegal,
            CandidateFeasibility::Feasible,
        )
        .expect("valid candidate");

        let report = select_best([illegal], &latency_policy())
            .expect("selection report is produced");

        assert!(report.selected.is_none());
        assert_eq!(report.ranking.len(), 0);
        assert_eq!(report.rejected.len(), 1);
    }
}
