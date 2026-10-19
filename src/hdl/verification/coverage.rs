
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Coverage Model
//!
//! Path: `src/hdl/verification/coverage.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Source-level coverage model and observation aggregation contract.
//!
//! ## Purpose
//!
//! Represent HDL coverage goals, associate provider-reported observations
//! with those goals, validate their identities, and aggregate the results.
//!
//! This module does not determine whether a property is semantically valid.
//! It does not execute simulation, perform formal exploration, or prove that
//! a behavior is reachable. Those responsibilities belong to the canonical
//! semantic pipeline and registered verification providers.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md` owns normative HDL semantics.
//! - `grammar/hdl/verification.g4` owns coverage declaration composition.
//! - `grammar/hdl/assertions.g4` owns property specifications, references,
//!   arguments, and verification qualifiers.
//! - `src/hdl/verification/assertions.rs` owns reusable assertion/property
//!   source-level model types.
//! - `src/frontend/semantic/domains/hdl.rs` owns `HdlId` and
//!   `HdlSourceLocation`.
//! - The canonical AST and semantic model own expressions, name resolution,
//!   typing, source identity, and semantic provenance.
//!
//! ## Owns
//!
//! - A coverage-goal declaration model.
//! - Coverage observation records and their reported outcomes.
//! - Structural identity validation and observation aggregation.
//! - Configurable operational budgets and deterministic diagnostics.
//!
//! ## Does not own
//!
//! - A second AST, expression tree, property grammar, or source-location type.
//! - Expression typing, name resolution, or property semantic validation.
//! - Simulation, formal proof, model checking, or reachability algorithms.
//! - Verification-provider authentication or evidence-certificate checking.
//! - Target feasibility, hardware allocation, synthesis, or physical realization.
//! - Any intermediate representation, HDLIR builder, pass, or lowering.
//!
//! ## Result soundness
//!
//! `Covered` means at least one accepted observation reports a hit for the
//! goal. It does not independently establish provider soundness.
//!
//! `NotCovered` means observations were complete and consistently reported
//! that the goal was not reached. It does not prove the goal unreachable.
//!
//! Unknown, unmeasured, contradictory, or incomplete observations are not
//! silently converted to coverage success.
//!
//! A coverage goal with no observations is unmeasured, not covered.
//! An empty set of goals is reported as `NoGoals`, not as successful coverage.
//!
//! ## Scalability
//!
//! This module imposes no language-level maximum on coverage goals, property
//! arguments, qualifiers, or observation records. Collections are dynamically
//! sized. Optional budgets are operational policies, not HDL semantics.
//!
//! No software can promise literal infinity: execution, storage, and identity
//! representations remain finite. Budget exhaustion and arithmetic overflow
//! are reported explicitly rather than changing coverage meaning.
//!
//! ## Integration
//!
//! 1. The frontend adapter maps each coverage declaration to a `CoverageGoal`.
//! 2. The adapter reuses canonical expression/property identities and source
//!    locations; it does not parse expression text here.
//! 3. A simulation, formal-cover, or other registered provider emits
//!    `CoverageObservation` records for the requested goal IDs.
//! 4. The caller invokes `analyze_coverage` with the goals, observations,
//!    and operational limits.
//! 5. The result is a structural and observational summary, not a proof.
//!
//! The provider and caller must establish the observation's scope, source
//! version, assumptions, provider identity, and trust policy before passing
//! it to this aggregator. A matching goal ID alone is not authenticated
//! evidence.
//!
//! ## Freeze criteria
//!
//! - Canonical identity, expression, and source-location types are reused.
//! - Unknown and incomplete observations cannot be promoted to coverage.
//! - Duplicate and orphaned records are reported deterministically.
//! - Operational budget exhaustion cannot produce a complete result.
//! - Count overflow is reported instead of wrapping.
//! - Tests and workspace integration checks pass on Rust 1.97+.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::domains::hdl::{HdlId, HdlSourceLocation};
use crate::hdl::verification::assertions::{
    ExpressionRef, VerificationArgument, VerificationQualifier,
};

// -----------------------------------------------------------------------------
// Goal declarations
// -----------------------------------------------------------------------------

/// A source-level HDL coverage goal.
///
/// `expression` refers to a canonical AST expression through the same opaque
/// adapter contract used by the assertion/property subsystem. `property`
/// identifies a reusable property when the frontend has resolved one.
/// The expression remains a reference, not a duplicate expression AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageGoal {
    /// Stable semantic identity of this goal.
    pub id: HdlId,
    /// Optional source-level label.
    pub label: Option<String>,
    /// Canonical expression reference for the coverage condition.
    pub expression: ExpressionRef,
    /// Reusable property identity, when the goal refers to a named property.
    pub property: Option<HdlId>,
    /// Arguments supplied to a named property, preserving source order.
    pub arguments: Vec<VerificationArgument>,
    /// Clock, disable, or extension-defined qualifiers.
    pub qualifiers: Vec<VerificationQualifier>,
    /// Source location of the declaration, if available.
    pub location: Option<HdlSourceLocation>,
}

impl CoverageGoal {
    /// Construct a coverage goal after validating its local input shape.
    ///
    /// This constructor does not resolve the expression or property and does
    /// not establish semantic validity or reachability.
    pub fn new(
        id: HdlId,
        label: Option<String>,
        expression: ExpressionRef,
        property: Option<HdlId>,
        arguments: Vec<VerificationArgument>,
        qualifiers: Vec<VerificationQualifier>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, CoverageInputError> {
        if label
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(CoverageInputError::WhitespaceOnlyLabel);
        }

        Ok(Self {
            id,
            label,
            expression,
            property,
            arguments,
            qualifiers,
            location,
        })
    }
}

// -----------------------------------------------------------------------------
// Provider observations
// -----------------------------------------------------------------------------

/// Outcome reported by a coverage provider for one observation.
///
/// These values describe provider-reported evidence, not an independently
/// checked proof. The caller is responsible for provider trust and scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoverageObservationOutcome {
    /// The provider observed the behavior or reached the cover goal.
    Hit,
    /// The observation completed but did not reach the goal.
    ///
    /// This is not a proof that the goal is unreachable.
    Miss,
    /// The provider could not determine whether the goal was reached.
    Unknown,
    /// The observation did not complete.
    Incomplete,
    /// No observation was performed for this record.
    NotMeasured,
}

/// A provider-reported observation associated with a coverage goal.
///
/// `occurrences` is the number of occurrences represented by this record.
/// It is meaningful for `Hit`; for other outcomes it should normally be zero.
/// A zero-occurrence `Hit` is rejected by analysis rather than treated as
/// evidence of coverage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageObservation {
    /// Identity of the coverage goal being observed.
    pub goal_id: HdlId,
    /// Provider-reported outcome.
    pub outcome: CoverageObservationOutcome,
    /// Number of hits represented by this observation.
    pub occurrences: u128,
    /// Source location associated with the observation, if available.
    pub location: Option<HdlSourceLocation>,
}

impl CoverageObservation {
    /// Construct an observation record.
    ///
    /// Goal existence, outcome consistency, provider authenticity, and
    /// observation scope are checked by the appropriate integration layer.
    #[must_use]
    pub fn new(
        goal_id: HdlId,
        outcome: CoverageObservationOutcome,
        occurrences: u128,
        location: Option<HdlSourceLocation>,
    ) -> Self {
        Self {
            goal_id,
            outcome,
            occurrences,
            location,
        }
    }
}

// -----------------------------------------------------------------------------
// Operational limits
// -----------------------------------------------------------------------------

/// Configurable operational budgets for coverage aggregation.
///
/// These are not language-level limits. `None` means this budget is not
/// imposed by this configuration. Host resource exhaustion can still occur.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CoverageLimits {
    /// Maximum coverage goals examined in one invocation.
    pub max_goals: Option<usize>,
    /// Maximum observation records examined in one invocation.
    pub max_observations: Option<usize>,
    /// Maximum diagnostics retained.
    ///
    /// When the limit prevents retaining a diagnostic that would otherwise
    /// be emitted, the analysis is incomplete. A limit of zero is valid and
    /// retains no diagnostics.
    pub max_diagnostics: Option<usize>,
}

// -----------------------------------------------------------------------------
// Diagnostics and results
// -----------------------------------------------------------------------------

/// Stable machine-readable diagnostic code for coverage analysis.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum CoverageDiagnosticCode {
    /// A coverage goal repeats an identity in the supplied scope.
    DuplicateGoalIdentity,
    /// Two goals use the same non-empty label in the supplied scope.
    DuplicateLabel,
    /// An observation refers to a goal absent from the supplied scope.
    UnknownGoal,
    /// More than one observation record uses the same goal identity.
    ///
    /// Multiple observations are otherwise allowed. This diagnostic is used
    /// only for duplicate record identity when record identity is represented
    /// by the goal ID; use separate provider aggregation before this API if
    /// multiple independent samples must be combined.
    DuplicateObservation,
    /// A hit outcome has no positive hit count.
    HitWithoutOccurrence,
    /// A non-hit outcome contains a non-zero occurrence count.
    UnexpectedOccurrences,
    /// An observation set contains conflicting outcomes.
    ConflictingObservations,
    /// The hit counter cannot represent the aggregate occurrence count.
    CountOverflow,
    /// A configured operational budget prevented complete analysis.
    BudgetExceeded,
    /// Analysis could not complete.
    IncompleteAnalysis,
}

/// Severity of a coverage diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoverageDiagnosticSeverity {
    /// Confirmed structural or data-contract error.
    Error,
    /// Unresolved or potentially misleading observation.
    Warning,
    /// Informational diagnostic.
    Note,
}

/// Structured coverage diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageDiagnostic {
    /// Stable diagnostic code.
    pub code: CoverageDiagnosticCode,
    /// Diagnostic severity.
    pub severity: CoverageDiagnosticSeverity,
    /// Coverage goal identity, when applicable.
    pub goal_id: Option<HdlId>,
    /// Human-readable explanation.
    pub message: String,
    /// Primary source location, if available.
    pub location: Option<HdlSourceLocation>,
}

/// Aggregated state of one coverage goal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CoverageGoalStatus {
    /// At least one accepted hit was observed.
    Covered,
    /// Observations completed and consistently reported no hit.
    NotCovered,
    /// Evidence is absent, unknown, or contradictory.
    Inconclusive,
    /// The observation process did not complete.
    Incomplete,
    /// No observation was supplied.
    NotMeasured,
    /// The goal or its observation data was structurally invalid.
    Invalid,
}

/// Aggregate counters for coverage analysis.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CoverageSummary {
    /// Number of distinct goals in the supplied scope.
    pub goals: usize,
    /// Number of goals with at least one accepted hit.
    pub covered: usize,
    /// Number of goals with complete observations and no hits.
    pub not_covered: usize,
    /// Number of goals with inconclusive observations.
    pub inconclusive: usize,
    /// Number of goals whose observation did not complete.
    pub incomplete: usize,
    /// Number of goals with no observations.
    pub not_measured: usize,
    /// Number of goals affected by structural errors.
    pub invalid: usize,
    /// Sum of accepted hit occurrences, represented as `u128`.
    ///
    /// If this sum overflows, the analysis is incomplete and this field
    /// contains the largest successfully accumulated partial total.
    pub total_occurrences: u128,
}

/// Overall status of coverage aggregation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverageAnalysisStatus {
    /// No goals were supplied. This is not successful coverage.
    NoGoals,
    /// Analysis completed with every goal covered.
    AllCovered,
    /// Analysis completed, with at least one goal not covered.
    SomeNotCovered,
    /// Analysis completed but one or more goals remain inconclusive or
    /// unmeasured.
    Inconclusive,
    /// One or more input records are structurally invalid.
    Invalid,
    /// The analysis did not complete.
    Incomplete,
}

/// Result of aggregating a set of coverage goals and observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoverageAnalysisResult {
    /// Overall analysis status.
    pub status: CoverageAnalysisStatus,
    /// Whether all requested analysis work completed.
    pub complete: bool,
    /// Summary counters.
    pub summary: CoverageSummary,
    /// Per-goal status in deterministic identity order.
    pub goals: BTreeMap<HdlId, CoverageGoalStatus>,
    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<CoverageDiagnostic>,
}

impl CoverageAnalysisResult {
    fn from_parts(
        complete: bool,
        summary: CoverageSummary,
        goals: BTreeMap<HdlId, CoverageGoalStatus>,
        mut diagnostics: Vec<CoverageDiagnostic>,
    ) -> Self {
        diagnostics.sort_by(|left, right| {
            (
                &left.goal_id,
                &left.code,
                &left.location,
                &left.message,
                &left.severity,
            )
                .cmp(&(
                    &right.goal_id,
                    &right.code,
                    &right.location,
                    &right.message,
                    &right.severity,
                ))
        });

        let has_error = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == CoverageDiagnosticSeverity::Error);

        let has_incomplete_diagnostic = diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.code,
                CoverageDiagnosticCode::BudgetExceeded
                    | CoverageDiagnosticCode::IncompleteAnalysis
                    | CoverageDiagnosticCode::CountOverflow
            )
        });

        let status = if !complete || has_incomplete_diagnostic {
            CoverageAnalysisStatus::Incomplete
        } else if has_error {
            CoverageAnalysisStatus::Invalid
        } else if summary.goals == 0 {
            CoverageAnalysisStatus::NoGoals
        } else if summary.invalid > 0 {
            CoverageAnalysisStatus::Invalid
        } else if summary.covered == summary.goals {
            CoverageAnalysisStatus::AllCovered
        } else if summary.not_covered > 0 {
            CoverageAnalysisStatus::SomeNotCovered
        } else {
            CoverageAnalysisStatus::Inconclusive
        };

        Self {
            status,
            complete: complete && !has_incomplete_diagnostic,
            summary,
            goals,
            diagnostics,
        }
    }
}

/// Error raised when a coverage input record cannot be constructed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverageInputError {
    /// A label was present but contained only whitespace.
    WhitespaceOnlyLabel,
}

impl fmt::Display for CoverageInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WhitespaceOnlyLabel => {
                formatter.write_str("coverage label cannot be whitespace only")
            }
        }
    }
}

impl std::error::Error for CoverageInputError {}

// -----------------------------------------------------------------------------
// Analysis
// -----------------------------------------------------------------------------

/// Analyze coverage declarations and aggregate provider observations.
///
/// The input slices define the analysis scope. This function does not infer
/// missing observations, execute verification, resolve expressions, or
/// authenticate providers.
///
/// The `max_diagnostics` limit is enforced when diagnostics are emitted, so
/// retained diagnostic memory is bounded by that policy. If diagnostics are
/// suppressed by the limit, `complete` is false. Goal and observation limits
/// are checked before analysis begins; exceeding either prevents partial
/// certification of the supplied scope.
#[must_use]
pub fn analyze_coverage(
    goals: &[CoverageGoal],
    observations: &[CoverageObservation],
    limits: &CoverageLimits,
) -> CoverageAnalysisResult {
    let mut summary = CoverageSummary::default();
    let mut statuses = BTreeMap::<HdlId, CoverageGoalStatus>::new();
    let mut diagnostics = Vec::<CoverageDiagnostic>::new();
    let mut complete = true;
    let mut diagnostic_limit_reached = false;

    let mut emit = |diagnostic: CoverageDiagnostic| {
        if limits
            .max_diagnostics
            .is_some_and(|maximum| diagnostics.len() >= maximum)
        {
            diagnostic_limit_reached = true;
            complete = false;
        } else {
            diagnostics.push(diagnostic);
        }
    };

    let goals_exceed_limit = limits
        .max_goals
        .is_some_and(|maximum| goals.len() > maximum);
    let observations_exceed_limit = limits
        .max_observations
        .is_some_and(|maximum| observations.len() > maximum);

    if goals_exceed_limit || observations_exceed_limit {
        let message = match (goals_exceed_limit, observations_exceed_limit) {
            (true, true) => "coverage goal and observation budgets were exceeded",
            (true, false) => "coverage goal budget was exceeded",
            (false, true) => "coverage observation budget was exceeded",
            (false, false) => unreachable!(),
        };

        emit(CoverageDiagnostic {
            code: CoverageDiagnosticCode::BudgetExceeded,
            severity: CoverageDiagnosticSeverity::Error,
            goal_id: None,
            message: message.to_owned(),
            location: None,
        });

        summary.goals = goals.len();
        complete = false;

        if diagnostic_limit_reached {
            diagnostics.clear();
        }

        return CoverageAnalysisResult::from_parts(
            false,
            summary,
            statuses,
            diagnostics,
        );
    }

    let mut goal_by_id = BTreeMap::<HdlId, &CoverageGoal>::new();
    let mut duplicate_goals = BTreeSet::<HdlId>::new();
    let mut labels = BTreeMap::<String, HdlId>::new();

    for goal in goals {
        summary.goals = summary.goals.saturating_add(1);

        if goal_by_id.insert(goal.id.clone(), goal).is_some() {
            duplicate_goals.insert(goal.id.clone());
            statuses.insert(goal.id.clone(), CoverageGoalStatus::Invalid);

            emit(CoverageDiagnostic {
                code: CoverageDiagnosticCode::DuplicateGoalIdentity,
                severity: CoverageDiagnosticSeverity::Error,
                goal_id: Some(goal.id.clone()),
                message: format!("duplicate coverage goal identity `{}`", goal.id),
                location: goal.location.clone(),
            });
        } else {
            statuses.insert(goal.id.clone(), CoverageGoalStatus::NotMeasured);
        }

        if let Some(label) = goal.label.as_deref() {
            if !label.trim().is_empty() {
                if let Some(previous_id) = labels.insert(label.to_owned(), goal.id.clone()) {
                    if previous_id != goal.id {
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::DuplicateLabel,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal.id.clone()),
                            message: format!(
                                "coverage label `{label}` is used by multiple goals"
                            ),
                            location: goal.location.clone(),
                        });
                        statuses.insert(goal.id.clone(), CoverageGoalStatus::Invalid);
                        statuses.insert(previous_id, CoverageGoalStatus::Invalid);
                    }
                }
            }
        }
    }

    // Aggregate each goal's records separately. Multiple observation records
    // for one goal are permitted so long as their outcomes are compatible.
    let mut by_goal = BTreeMap::<HdlId, Vec<&CoverageObservation>>::new();

    for observation in observations {
        if !goal_by_id.contains_key(&observation.goal_id) {
            emit(CoverageDiagnostic {
                code: CoverageDiagnosticCode::UnknownGoal,
                severity: CoverageDiagnosticSeverity::Error,
                goal_id: Some(observation.goal_id.clone()),
                message: format!(
                    "coverage observation refers to unknown goal `{}`",
                    observation.goal_id
                ),
                location: observation.location.clone(),
            });
            continue;
        }

        by_goal
            .entry(observation.goal_id.clone())
            .or_default()
            .push(observation);
    }

    for (goal_id, goal) in &goal_by_id {
        if duplicate_goals.contains(goal_id) {
            statuses.insert(goal_id.clone(), CoverageGoalStatus::Invalid);
            continue;
        }

        let Some(records) = by_goal.get(goal_id) else {
            statuses.insert(goal_id.clone(), CoverageGoalStatus::NotMeasured);
            continue;
        };

        let mut saw_hit = false;
        let mut saw_miss = false;
        let mut saw_unknown = false;
        let mut saw_incomplete = false;
        let mut saw_not_measured = false;
        let mut goal_invalid = false;
        let mut hit_total = 0_u128;

        for observation in records {
            match observation.outcome {
                CoverageObservationOutcome::Hit => {
                    if observation.occurrences == 0 {
                        goal_invalid = true;
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::HitWithoutOccurrence,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal_id.clone()),
                            message: "a hit observation must report at least one occurrence"
                                .to_owned(),
                            location: observation.location.clone(),
                        });
                    } else {
                        saw_hit = true;
                        match hit_total.checked_add(observation.occurrences) {
                            Some(total) => hit_total = total,
                            None => {
                                saw_incomplete = true;
                                complete = false;
                                emit(CoverageDiagnostic {
                                    code: CoverageDiagnosticCode::CountOverflow,
                                    severity: CoverageDiagnosticSeverity::Error,
                                    goal_id: Some(goal_id.clone()),
                                    message: "aggregate coverage occurrence count overflowed"
                                        .to_owned(),
                                    location: observation.location.clone(),
                                });
                            }
                        }
                    }
                }
                CoverageObservationOutcome::Miss => {
                    saw_miss = true;
                    if observation.occurrences != 0 {
                        goal_invalid = true;
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::UnexpectedOccurrences,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal_id.clone()),
                            message: "a miss observation cannot report hit occurrences"
                                .to_owned(),
                            location: observation.location.clone(),
                        });
                    }
                }
                CoverageObservationOutcome::Unknown => {
                    saw_unknown = true;
                    if observation.occurrences != 0 {
                        goal_invalid = true;
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::UnexpectedOccurrences,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal_id.clone()),
                            message: "an unknown observation cannot report hit occurrences"
                                .to_owned(),
                            location: observation.location.clone(),
                        });
                    }
                }
                CoverageObservationOutcome::Incomplete => {
                    saw_incomplete = true;
                    complete = false;
                    if observation.occurrences != 0 {
                        goal_invalid = true;
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::UnexpectedOccurrences,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal_id.clone()),
                            message: "an incomplete observation cannot report hit occurrences"
                                .to_owned(),
                            location: observation.location.clone(),
                        });
                    }
                }
                CoverageObservationOutcome::NotMeasured => {
                    saw_not_measured = true;
                    if observation.occurrences != 0 {
                        goal_invalid = true;
                        emit(CoverageDiagnostic {
                            code: CoverageDiagnosticCode::UnexpectedOccurrences,
                            severity: CoverageDiagnosticSeverity::Error,
                            goal_id: Some(goal_id.clone()),
                            message: "an unmeasured observation cannot report hit occurrences"
                                .to_owned(),
                            location: observation.location.clone(),
                        });
                    }
                }
            }
        }

        if saw_hit && (saw_miss || saw_unknown || saw_incomplete || saw_not_measured) {
            emit(CoverageDiagnostic {
                code: CoverageDiagnosticCode::ConflictingObservations,
                severity: CoverageDiagnosticSeverity::Warning,
                goal_id: Some(goal_id.clone()),
                message: "coverage goal has a hit and one or more non-hit observations"
                    .to_owned(),
                location: goal.location.clone(),
            });
        }

        let status = if goal_invalid {
            CoverageGoalStatus::Invalid
        } else if saw_incomplete {
            CoverageGoalStatus::Incomplete
        } else if saw_hit {
            CoverageGoalStatus::Covered
        } else if saw_unknown {
            CoverageGoalStatus::Inconclusive
        } else if saw_not_measured {
            CoverageGoalStatus::NotMeasured
        } else if saw_miss {
            CoverageGoalStatus::NotCovered
        } else {
            CoverageGoalStatus::NotMeasured
        };

        statuses.insert(goal_id.clone(), status);

        if saw_hit && !goal_invalid {
            match summary.total_occurrences.checked_add(hit_total) {
                Some(total) => summary.total_occurrences = total,
                None => {
                    complete = false;
                    emit(CoverageDiagnostic {
                        code: CoverageDiagnosticCode::CountOverflow,
                        severity: CoverageDiagnosticSeverity::Error,
                        goal_id: Some(goal_id.clone()),
                        message: "total coverage occurrence count overflowed".to_owned(),
                        location: goal.location.clone(),
                    });
                }
            }
        }
    }

    // Derive summary counts from the final per-goal statuses, ensuring the
    // counters cannot drift from the status map.
    summary.covered = 0;
    summary.not_covered = 0;
    summary.inconclusive = 0;
    summary.incomplete = 0;
    summary.not_measured = 0;
    summary.invalid = 0;

    for status in statuses.values() {
        match status {
            CoverageGoalStatus::Covered => summary.covered = summary.covered.saturating_add(1),
            CoverageGoalStatus::NotCovered => {
                summary.not_covered = summary.not_covered.saturating_add(1);
            }
            CoverageGoalStatus::Inconclusive => {
                summary.inconclusive = summary.inconclusive.saturating_add(1);
            }
            CoverageGoalStatus::Incomplete => {
                summary.incomplete = summary.incomplete.saturating_add(1);
            }
            CoverageGoalStatus::NotMeasured => {
                summary.not_measured = summary.not_measured.saturating_add(1);
            }
            CoverageGoalStatus::Invalid => summary.invalid = summary.invalid.saturating_add(1),
        }
    }

    if diagnostic_limit_reached {
        complete = false;
        // Do not retain a partial diagnostic list that appears complete when
        // the configured limit suppressed additional diagnostics.
        emit(CoverageDiagnostic {
            code: CoverageDiagnosticCode::BudgetExceeded,
            severity: CoverageDiagnosticSeverity::Warning,
            goal_id: None,
            message: "coverage diagnostic budget was reached; diagnostics are incomplete"
                .to_owned(),
            location: None,
        });
        if diagnostic_limit_reached {
            diagnostics.truncate(limits.max_diagnostics.unwrap_or(usize::MAX));
        }
    }

    if !complete
        && !diagnostics.iter().any(|diagnostic| {
            matches!(
                diagnostic.code,
                CoverageDiagnosticCode::BudgetExceeded
                    | CoverageDiagnosticCode::IncompleteAnalysis
            )
        })
    {
        emit(CoverageDiagnostic {
            code: CoverageDiagnosticCode::IncompleteAnalysis,
            severity: CoverageDiagnosticSeverity::Warning,
            goal_id: None,
            message: "coverage analysis did not complete".to_owned(),
            location: None,
        });
    }

    CoverageAnalysisResult::from_parts(complete, summary, statuses, diagnostics)
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be valid")
    }

    fn expression(value: &str) -> ExpressionRef {
        ExpressionRef::new(value).expect("test expression reference must be valid")
    }

    fn goal(value: &str) -> CoverageGoal {
        CoverageGoal::new(
            id(value),
            None,
            expression(&format!("ast:{value}")),
            None,
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("test goal must be valid")
    }

    fn observation(
        goal_id: &str,
        outcome: CoverageObservationOutcome,
        occurrences: u128,
    ) -> CoverageObservation {
        CoverageObservation::new(id(goal_id), outcome, occurrences, None)
    }

    #[test]
    fn empty_scope_is_not_reported_as_successful_coverage() {
        let result = analyze_coverage(&[], &[], &CoverageLimits::default());

        assert_eq!(result.status, CoverageAnalysisStatus::NoGoals);
        assert!(result.complete);
        assert_eq!(result.summary.goals, 0);
    }

    #[test]
    fn a_hit_covers_a_goal_and_preserves_occurrence_count() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("ready", CoverageObservationOutcome::Hit, 3)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::AllCovered);
        assert_eq!(result.goals.get(&id("ready")), Some(&CoverageGoalStatus::Covered));
        assert_eq!(result.summary.total_occurrences, 3);
    }

    #[test]
    fn a_miss_is_not_a_proof_of_unreachability() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("ready", CoverageObservationOutcome::Miss, 0)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::SomeNotCovered);
        assert_eq!(
            result.goals.get(&id("ready")),
            Some(&CoverageGoalStatus::NotCovered)
        );
    }

    #[test]
    fn a_goal_without_observations_is_unmeasured() {
        let result = analyze_coverage(&[goal("ready")], &[], &CoverageLimits::default());

        assert_eq!(result.status, CoverageAnalysisStatus::Inconclusive);
        assert_eq!(
            result.goals.get(&id("ready")),
            Some(&CoverageGoalStatus::NotMeasured)
        );
    }

    #[test]
    fn unknown_observation_is_not_promoted_to_covered() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("ready", CoverageObservationOutcome::Unknown, 0)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Inconclusive);
        assert_eq!(
            result.goals.get(&id("ready")),
            Some(&CoverageGoalStatus::Inconclusive)
        );
    }

    #[test]
    fn incomplete_observation_makes_analysis_incomplete() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("ready", CoverageObservationOutcome::Incomplete, 0)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn zero_occurrence_hit_is_invalid() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("ready", CoverageObservationOutcome::Hit, 0)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Invalid);
        assert_eq!(
            result.goals.get(&id("ready")),
            Some(&CoverageGoalStatus::Invalid)
        );
    }

    #[test]
    fn observations_for_unknown_goals_are_reported() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[observation("missing", CoverageObservationOutcome::Hit, 1)],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == CoverageDiagnosticCode::UnknownGoal
        }));
    }

    #[test]
    fn duplicate_goal_identity_is_invalid() {
        let result = analyze_coverage(
            &[goal("ready"), goal("ready")],
            &[],
            &CoverageLimits::default(),
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == CoverageDiagnosticCode::DuplicateGoalIdentity
        }));
    }

    #[test]
    fn goal_budget_exhaustion_does_not_certify_results() {
        let result = analyze_coverage(
            &[goal("ready")],
            &[],
            &CoverageLimits {
                max_goals: Some(0),
                ..CoverageLimits::default()
            },
        );

        assert_eq!(result.status, CoverageAnalysisStatus::Incomplete);
        assert!(!result.complete);
    }

    #[test]
    fn diagnostic_budget_zero_does_not_retain_diagnostics_or_claim_completeness() {
        let result = analyze_coverage(
            &[goal("ready"), goal("ready")],
            &[],
            &CoverageLimits {
                max_diagnostics: Some(0),
                ..CoverageLimits::default()
            },
        );

        assert!(!result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn results_are_deterministic_for_reordered_inputs() {
        let goals_a = vec![goal("alpha"), goal("beta")];
        let goals_b = vec![goal("beta"), goal("alpha")];
        let observations_a = vec![
            observation("alpha", CoverageObservationOutcome::Hit, 2),
            observation("beta", CoverageObservationOutcome::Miss, 0),
        ];
        let observations_b = vec![
            observation("beta", CoverageObservationOutcome::Miss, 0),
            observation("alpha", CoverageObservationOutcome::Hit, 2),
        ];

        let a = analyze_coverage(&goals_a, &observations_a, &CoverageLimits::default());
        let b = analyze_coverage(&goals_b, &observations_b, &CoverageLimits::default());

        assert_eq!(a.status, b.status);
        assert_eq!(a.summary, b.summary);
        assert_eq!(a.goals, b.goals);
        assert_eq!(a.diagnostics, b.diagnostics);
    }
}
