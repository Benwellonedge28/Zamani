//! Zamani Quantum Resilience — Mitigation Strategy Selection
//!
//! Path:
//!     src/quantum/resilience/mitigation/selection.rs
//!
//! Purpose:
//!     Deterministically select an applicable quantum-error mitigation
//!     strategy from a supplied strategy set.
//!
//! Architectural position:
//!
//! ```text
//!                         Canonical Zamani Program / IR
//!                                    |
//!                                    v
//!                         Resilience / Policy Layer
//!                                    |
//!                                    v
//!                         Mitigation Selection
//!                                    |
//!                 +------------------+------------------+
//!                 |                  |                  |
//!                 v                  v                  v
//!             strategy A         strategy B         strategy N
//!                 |                  |                  |
//!                 +------------------+------------------+
//!                                    |
//!                                    v
//!                         Approved Strategy Selection
//!                                    |
//!                                    v
//!                         mitigation::executor
//!                                    |
//!                                    v
//!                              Verification
//! ```
//!
//! # Responsibility
//!
//! This module is responsible for:
//!
//! - evaluating registered mitigation strategies;
//! - filtering strategies that are not applicable;
//! - respecting caller mitigation permission;
//! - respecting explicit selection constraints;
//! - ranking otherwise-equivalent candidates deterministically;
//! - preventing ambiguous selections;
//! - returning enough provenance for later execution and verification;
//! - keeping selection independent of hardware providers;
//! - keeping selection independent of strategy implementation;
//! - keeping selection independent of quantum execution;
//! - keeping selection independent of routing, scheduling, optimization and QEC.
//!
//! This module MUST NOT:
//!
//! - execute a quantum circuit;
//! - submit a backend job;
//! - communicate with a provider;
//! - perform routing;
//! - perform scheduling;
//! - compile or optimize a circuit;
//! - implement error mitigation;
//! - implement QEC;
//! - mutate a strategy;
//! - mutate execution state;
//! - silently retry;
//! - create credentials;
//! - read environment variables;
//! - read the system clock;
//! - perform filesystem/network I/O;
//! - use global mutable state;
//! - contain provider-specific branches;
//! - contain fixed qubit counts;
//! - contain fixed backend counts;
//! - contain fixed strategy counts;
//! - contain hard-coded retry counts;
//! - silently convert an unvalidated strategy into an executable strategy.
//!
//! # Write once, scale everywhere
//!
//! The selector has no architectural limit on:
//!
//! - logical qubits;
//! - physical qubits;
//! - circuit size;
//! - circuit depth;
//! - strategy count;
//! - backend count;
//! - mitigation variants;
//! - execution count;
//! - distributed resources.
//!
//! Actual limits are supplied by the execution target, policy, capabilities,
//! budgets and available resources.
//!
//! # Selection safety rule
//!
//! A strategy is executable only when its evaluation is:
//!
//!     Applicability::Applicable
//!
//! These states are NEVER directly executable selections:
//!
//!     RequiresCapabilityValidation
//!     RequiresPolicyValidation
//!     InsufficientInformation
//!     NotApplicable
//!
//! They may be returned as deferred candidates for a higher-level controller,
//! but this module will not authorize them for execution.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - strategy descriptors;
//! - strategy evaluation context;
//! - selection policy;
//! - strategy versions;
//! - deterministic selection seed, when explicitly supplied;
//!
//! selection produces the same result.
//!
//! No randomness is generated implicitly.
//!
//! # Strategy identity
//!
//! Strategy identifiers remain open-ended. This module MUST NOT use a closed
//! match such as:
//!
//! ```text
//! match strategy {
//!     IBM => ...
//!     IonQ => ...
//! }
//! ```
//!
//! New mitigation strategies therefore do not require modification of this
//! selector.
//!
//! # Canonical quantum identity
//!
//! Selection does not define a second qubit identity. Strategy scope is carried
//! by `MitigationScope`, which uses the canonical:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! from `mitigation::strategy`.
//!
//! # Integration
//!
//! `strategy.rs`
//!     Supplies `MitigationStrategy`, `StrategySet`, descriptors and evaluation.
//!
//! `policy/*`
//!     Supplies whether mitigation is Disabled, Allowed or Required and any
//!     higher-level resource/semantic constraints.
//!
//! `planning/*`
//!     May consume the selection result as a candidate resilience action.
//!
//! `executor.rs`
//!     Executes ONLY an already-approved selection/plan. This module does not
//!     execute strategies.
//!
//! `verification/*`
//!     Verifies the semantic validity of the resulting execution.
//!
//! `telemetry/*`
//!     Records the selected strategy identity, version and selection provenance.
//!
//! `history/*`
//!     Records selection outcomes for future statistical analysis.
//!
//! `registry/strategy.rs`
//!     Owns registration/discovery of concrete strategy implementations.
//!
//! `quantum::ir`
//!     Remains authoritative for program semantics and qubit identity.
//!
//! `quantum::hardware`
//!     Remains authoritative for machine capabilities.
//!
//! `quantum::zqn`
//!     Remains authoritative for fault/noise semantics.
//!
//! This file intentionally consumes those contracts rather than duplicating
//! them.
//!
//! # Rust
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use super::strategy::{
    Applicability,
    MitigationScope,
    MitigationStrategy,
    OverheadDimension,
    OverheadLevel,
    StrategyDescriptor,
    StrategyEvaluation,
    StrategyFamily,
    StrategyId,
    StrategyRequirement,
    StrategySet,
    StrategyVersion,
};

use crate::quantum::resilience::api::request::MitigationPermission;

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for mitigation selection.
pub const MITIGATION_SELECTION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.selection";

/// Semantic version of the selection contract.
pub const MITIGATION_SELECTION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Selection mode
// =============================================================================

/// Determines how strictly the selector handles ties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SelectionMode {
    /// Select exactly one deterministic winner when at least one applicable
    /// strategy exists.
    Deterministic,

    /// Permit an unresolved tie to be returned to the higher-level planner.
    ///
    /// No strategy is silently chosen in this mode.
    RequireExplicitTieBreak,
}

impl Default for SelectionMode {
    fn default() -> Self {
        Self::Deterministic
    }
}

impl SelectionMode {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deterministic => "deterministic",
            Self::RequireExplicitTieBreak => "require_explicit_tie_break",
        }
    }
}

impl fmt::Display for SelectionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Selection objective
// =============================================================================

/// Primary objective used to rank otherwise-applicable strategies.
///
/// The selector never uses hardware-specific constants. Concrete overhead is
/// represented by the strategy descriptor and target-specific planning belongs
/// to the planner/executor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SelectionObjective {
    /// Prefer the strategy expected to introduce the least overhead.
    ResourceEfficiency,

    /// Prefer strategies with lower execution overhead.
    ExecutionEfficiency,

    /// Prefer strategies with lower classical processing overhead.
    ClassicalEfficiency,

    /// Prefer strategies that require fewer additional circuit variants.
    VariantEfficiency,

    /// Prefer deterministic strategies.
    Determinism,

    /// Prefer strategies with the strongest declared applicability.
    Applicability,

    /// Do not impose an objective preference; use explicit priority and stable
    /// identity ordering.
    PolicyDefined,
}

impl Default for SelectionObjective {
    fn default() -> Self {
        Self::Applicability
    }
}

impl SelectionObjective {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourceEfficiency => "resource_efficiency",
            Self::ExecutionEfficiency => "execution_efficiency",
            Self::ClassicalEfficiency => "classical_efficiency",
            Self::VariantEfficiency => "variant_efficiency",
            Self::Determinism => "determinism",
            Self::Applicability => "applicability",
            Self::PolicyDefined => "policy_defined",
        }
    }
}

// =============================================================================
// Strategy priority
// =============================================================================

/// Explicit caller-supplied strategy priority.
///
/// This is intentionally associated with an open-ended `StrategyId` rather
/// than a closed enum of known mitigation strategies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyPriority {
    /// Strategy identity.
    pub strategy_id: StrategyId,

    /// Higher values are preferred over lower values.
    ///
    /// This is a policy ordering, not a hardware limit.
    pub priority: i64,
}

impl StrategyPriority {
    /// Creates an explicit strategy priority.
    pub const fn new(strategy_id: StrategyId, priority: i64) -> Self {
        Self {
            strategy_id,
            priority,
        }
    }
}

// =============================================================================
// Family priority
// =============================================================================

/// Optional priority for a broad strategy family.
///
/// Family priority is intentionally secondary to explicit strategy-ID
/// priorities so future custom strategies remain possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FamilyPriority {
    /// Strategy family.
    pub family: StrategyFamily,

    /// Higher values are preferred.
    pub priority: i64,
}

impl FamilyPriority {
    /// Creates a family priority.
    pub const fn new(family: StrategyFamily, priority: i64) -> Self {
        Self { family, priority }
    }
}

// =============================================================================
// Selection configuration
// =============================================================================

/// Immutable configuration controlling strategy selection.
///
/// This configuration contains preferences only. It does not grant hardware
/// capabilities and does not authorize execution by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionConfig {
    /// Mitigation permission supplied by the resilience policy/request layer.
    pub permission: MitigationPermission,

    /// Primary selection objective.
    pub objective: SelectionObjective,

    /// Selection behavior when multiple candidates are otherwise equivalent.
    pub mode: SelectionMode,

    /// Explicit strategy priorities.
    pub strategy_priorities: Arc<[StrategyPriority]>,

    /// Optional family priorities.
    pub family_priorities: Arc<[FamilyPriority]>,

    /// Whether strategies requiring explicit authorization may be considered.
    ///
    /// This does NOT replace policy authorization. It is an additional
    /// fail-closed selector constraint.
    pub explicit_authorization_allowed: bool,

    /// Whether strategies that have target-specific capability validation
    /// pending may be returned as deferred candidates.
    ///
    /// Deferred candidates are never executable selections.
    pub retain_deferred_candidates: bool,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            permission: MitigationPermission::Allowed,
            objective: SelectionObjective::Applicability,
            mode: SelectionMode::Deterministic,
            strategy_priorities: Arc::from([]),
            family_priorities: Arc::from([]),
            explicit_authorization_allowed: true,
            retain_deferred_candidates: true,
        }
    }
}

impl SelectionConfig {
    /// Creates a configuration with mitigation disabled.
    pub fn disabled() -> Self {
        Self {
            permission: MitigationPermission::Disabled,
            ..Self::default()
        }
    }

    /// Creates a configuration requiring mitigation when a valid strategy
    /// exists.
    pub fn required() -> Self {
        Self {
            permission: MitigationPermission::Required,
            ..Self::default()
        }
    }

    /// Adds an explicit strategy priority.
    ///
    /// Duplicate IDs are rejected because ambiguous policy configuration is
    /// unsafe.
    pub fn with_strategy_priorities(
        mut self,
        priorities: impl IntoIterator<Item = StrategyPriority>,
    ) -> Result<Self, SelectionError> {
        let values: Vec<StrategyPriority> = priorities.into_iter().collect();

        validate_unique_strategy_priorities(&values)?;

        self.strategy_priorities = values.into();
        Ok(self)
    }

    /// Adds family priorities.
    ///
    /// Duplicate families are rejected because ambiguous policy configuration
    /// is unsafe.
    pub fn with_family_priorities(
        mut self,
        priorities: impl IntoIterator<Item = FamilyPriority>,
    ) -> Result<Self, SelectionError> {
        let values: Vec<FamilyPriority> = priorities.into_iter().collect();

        validate_unique_family_priorities(&values)?;

        self.family_priorities = values.into();
        Ok(self)
    }

    /// Sets the selection objective.
    pub const fn with_objective(
        mut self,
        objective: SelectionObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Sets the selection mode.
    pub const fn with_mode(mut self, mode: SelectionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets whether explicitly authorized strategies may be considered.
    pub const fn with_explicit_authorization(
        mut self,
        allowed: bool,
    ) -> Self {
        self.explicit_authorization_allowed = allowed;
        self
    }

    /// Sets whether deferred candidates should be retained in the result.
    pub const fn retain_deferred_candidates(
        mut self,
        retain: bool,
    ) -> Self {
        self.retain_deferred_candidates = retain;
        self
    }

    /// Returns the explicit priority for a strategy, if present.
    #[must_use]
    pub fn strategy_priority(&self, id: &StrategyId) -> Option<i64> {
        self.strategy_priorities
            .iter()
            .find(|item| item.strategy_id == *id)
            .map(|item| item.priority)
    }

    /// Returns the family priority, if present.
    #[must_use]
    pub fn family_priority(&self, family: StrategyFamily) -> Option<i64> {
        self.family_priorities
            .iter()
            .find(|item| item.family == family)
            .map(|item| item.priority)
    }
}

// =============================================================================
// Selection request
// =============================================================================

/// Immutable request supplied to the mitigation selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionRequest {
    /// Abstract strategy evaluation context.
    pub context: super::strategy::StrategyContext,

    /// Selection configuration.
    pub config: SelectionConfig,
}

impl SelectionRequest {
    /// Creates a selection request.
    pub fn new(
        context: super::strategy::StrategyContext,
        config: SelectionConfig,
    ) -> Self {
        Self { context, config }
    }

    /// Returns the requested mitigation scope.
    #[must_use]
    pub fn scope(&self) -> &MitigationScope {
        &self.context.scope
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// A strategy candidate after applicability evaluation but before final
/// selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionCandidate {
    /// Strategy identity.
    pub strategy_id: StrategyId,

    /// Strategy version.
    pub strategy_version: StrategyVersion,

    /// Strategy family.
    pub family: StrategyFamily,

    /// Evaluation result.
    pub evaluation: StrategyEvaluation,

    /// Explicit strategy priority.
    pub strategy_priority: i64,

    /// Family priority.
    pub family_priority: i64,

    /// Objective-derived score.
    ///
    /// This is an ordering value, not a physical quantity.
    pub objective_score: i64,
}

impl SelectionCandidate {
    /// Returns whether this candidate is directly executable.
    ///
    /// Only `Applicable` is executable from the selection layer.
    #[must_use]
    pub const fn is_executable(&self) -> bool {
        matches!(self.evaluation.applicability, Applicability::Applicable)
    }
}

// =============================================================================
// Selection result
// =============================================================================

/// Final result of mitigation selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionResult {
    /// No mitigation is permitted by policy.
    Disabled,

    /// No applicable strategy exists.
    NoApplicableStrategy {
        /// Evaluations of all considered strategies.
        evaluations: Arc<[StrategyEvaluation]>,
    },

    /// One strategy was safely selected.
    Selected {
        /// Selected strategy ID.
        strategy_id: StrategyId,

        /// Selected strategy version.
        strategy_version: StrategyVersion,

        /// Full evaluated candidate.
        candidate: SelectionCandidate,

        /// Other deferred candidates, if requested by configuration.
        deferred: Arc<[SelectionCandidate]>,
    },

    /// One or more candidates require additional validation.
    Deferred {
        /// Candidates that cannot yet be safely selected.
        candidates: Arc<[SelectionCandidate]>,
    },

    /// Multiple strategies remain equally ranked and the configured selection
    /// mode requires explicit tie-breaking.
    Ambiguous {
        /// Equally ranked candidates.
        candidates: Arc<[SelectionCandidate]>,
    },
}

impl SelectionResult {
    /// Returns whether a strategy was selected.
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        matches!(self, Self::Selected { .. })
    }

    /// Returns the selected strategy ID.
    #[must_use]
    pub fn strategy_id(&self) -> Option<&StrategyId> {
        match self {
            Self::Selected { strategy_id, .. } => Some(strategy_id),
            _ => None,
        }
    }

    /// Returns the selected strategy version.
    #[must_use]
    pub fn strategy_version(&self) -> Option<StrategyVersion> {
        match self {
            Self::Selected {
                strategy_version, ..
            } => Some(*strategy_version),
            _ => None,
        }
    }
}

// =============================================================================
// Selection report
// =============================================================================

/// Complete immutable selection report.
///
/// This is useful for telemetry, audit, deterministic replay and provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionReport {
    /// Final selection result.
    pub result: SelectionResult,

    /// Number of strategies examined.
    pub evaluated_count: usize,

    /// Number of directly applicable strategies.
    pub applicable_count: usize,

    /// Number of deferred strategies.
    pub deferred_count: usize,

    /// Number of rejected/non-applicable strategies.
    pub rejected_count: usize,
}

impl SelectionReport {
    /// Returns whether selection produced an executable strategy.
    #[must_use]
    pub fn is_selected(&self) -> bool {
        self.result.is_selected()
    }
}

// =============================================================================
// Selector
// =============================================================================

/// Stateless deterministic mitigation selector.
///
/// A selector owns no mutable global registry. The caller supplies the
/// `StrategySet`, allowing multiple independent resilience contexts to use
/// different strategy sets safely.
#[derive(Debug, Clone, Copy, Default)]
pub struct MitigationSelector;

impl MitigationSelector {
    /// Creates a selector.
    pub const fn new() -> Self {
        Self
    }

    /// Selects a mitigation strategy from the supplied strategy set.
    ///
    /// This method never executes a strategy.
    pub fn select(
        &self,
        strategies: &StrategySet,
        request: &SelectionRequest,
    ) -> Result<SelectionReport, SelectionError> {
        validate_config(&request.config)?;

        if matches!(
            request.config.permission,
            MitigationPermission::Disabled
        ) {
            return Ok(SelectionReport {
                result: SelectionResult::Disabled,
                evaluated_count: 0,
                applicable_count: 0,
                deferred_count: 0,
                rejected_count: 0,
            });
        }

        let mut applicable = Vec::new();
        let mut deferred = Vec::new();
        let mut evaluations = Vec::new();

        for strategy in strategies.iter() {
            let evaluation = strategy.evaluate(&request.context);

            evaluations.push(evaluation.clone());

            match evaluation.applicability {
                Applicability::Applicable => {
                    if !request.config.explicit_authorization_allowed
                        && strategy
                            .descriptor()
                            .requires_explicit_authorization
                    {
                        deferred.push(self.build_candidate(
                            strategy,
                            evaluation,
                            request,
                        ));
                        continue;
                    }

                    applicable.push(self.build_candidate(
                        strategy,
                        evaluation,
                        request,
                    ));
                }

                Applicability::RequiresCapabilityValidation
                | Applicability::RequiresPolicyValidation
                | Applicability::InsufficientInformation => {
                    if request.config.retain_deferred_candidates {
                        deferred.push(self.build_candidate(
                            strategy,
                            evaluation,
                            request,
                        ));
                    }
                }

                Applicability::NotApplicable => {}
            }
        }

        let evaluated_count = evaluations.len();
        let applicable_count = applicable.len();
        let deferred_count = deferred.len();
        let rejected_count = evaluated_count
            .saturating_sub(applicable_count)
            .saturating_sub(deferred_count);

        if applicable.is_empty() {
            if matches!(
                request.config.permission,
                MitigationPermission::Required
            ) && !deferred.is_empty()
            {
                return Ok(SelectionReport {
                    result: SelectionResult::Deferred {
                        candidates: deferred.into(),
                    },
                    evaluated_count,
                    applicable_count,
                    deferred_count,
                    rejected_count,
                });
            }

            return Ok(SelectionReport {
                result: SelectionResult::NoApplicableStrategy {
                    evaluations: evaluations.into(),
                },
                evaluated_count,
                applicable_count,
                deferred_count,
                rejected_count,
            });
        }

        sort_candidates(&mut applicable);

        let winner = applicable
            .first()
            .cloned()
            .ok_or(SelectionError::InternalEmptyCandidateSet)?;

        let tied = equally_ranked_candidates(&applicable);

        if tied.len() > 1
            && matches!(
                request.config.mode,
                SelectionMode::RequireExplicitTieBreak
            )
        {
            return Ok(SelectionReport {
                result: SelectionResult::Ambiguous {
                    candidates: tied.into(),
                },
                evaluated_count,
                applicable_count,
                deferred_count,
                rejected_count,
            });
        }

        let selected_id = winner.strategy_id.clone();
        let selected_version = winner.strategy_version;

        let deferred_for_result = deferred.into();

        Ok(SelectionReport {
            result: SelectionResult::Selected {
                strategy_id: selected_id,
                strategy_version: selected_version,
                candidate: winner,
                deferred: deferred_for_result,
            },
            evaluated_count,
            applicable_count,
            deferred_count,
            rejected_count,
        })
    }

    /// Evaluates strategies without selecting one.
    ///
    /// Useful for diagnostics, planning previews and telemetry.
    pub fn evaluate(
        &self,
        strategies: &StrategySet,
        request: &SelectionRequest,
    ) -> Result<Arc<[StrategyEvaluation]>, SelectionError> {
        validate_config(&request.config)?;

        if matches!(
            request.config.permission,
            MitigationPermission::Disabled
        ) {
            return Ok(Arc::from([]));
        }

        Ok(strategies.evaluate_all(&request.context).into())
    }

    fn build_candidate(
        &self,
        strategy: &Arc<dyn MitigationStrategy>,
        evaluation: StrategyEvaluation,
        request: &SelectionRequest,
    ) -> SelectionCandidate {
        let descriptor = strategy.descriptor();

        SelectionCandidate {
            strategy_id: descriptor.id.clone(),
            strategy_version: descriptor.version,
            family: descriptor.family,
            strategy_priority: request
                .config
                .strategy_priority(&descriptor.id)
                .unwrap_or(0),
            family_priority: request
                .config
                .family_priority(descriptor.family)
                .unwrap_or(0),
            objective_score: objective_score(
                descriptor,
                request.config.objective,
            ),
            evaluation,
        }
    }
}

// =============================================================================
// Ranking
// =============================================================================

/// Sorts candidates from best to worst.
///
/// The final identity comparison is always stable and deterministic.
fn sort_candidates(candidates: &mut [SelectionCandidate]) {
    candidates.sort_by(compare_candidates);
}

/// Compares two candidates.
///
/// Ranking precedence:
///
/// 1. applicability;
/// 2. explicit strategy priority;
/// 3. family priority;
/// 4. objective score;
/// 5. deterministic strategy identifier;
/// 6. strategy version.
///
/// The identifier/version fallback guarantees deterministic ordering without
/// relying on registration order.
fn compare_candidates(
    left: &SelectionCandidate,
    right: &SelectionCandidate,
) -> Ordering {
    applicability_rank(left.evaluation.applicability)
        .cmp(&applicability_rank(right.evaluation.applicability))
        .then_with(|| {
            right
                .strategy_priority
                .cmp(&left.strategy_priority)
        })
        .then_with(|| {
            right
                .family_priority
                .cmp(&left.family_priority)
        })
        .then_with(|| {
            right
                .objective_score
                .cmp(&left.objective_score)
        })
        .then_with(|| left.strategy_id.cmp(&right.strategy_id))
        .then_with(|| {
            right
                .strategy_version
                .cmp(&left.strategy_version)
        })
}

/// Returns the rank of an applicability state.
///
/// Only `Applicable` is expected in final candidate ranking. The other values
/// are nevertheless ordered defensively for future use.
#[must_use]
const fn applicability_rank(applicability: Applicability) -> u8 {
    match applicability {
        Applicability::Applicable => 4,
        Applicability::RequiresPolicyValidation => 3,
        Applicability::RequiresCapabilityValidation => 2,
        Applicability::InsufficientInformation => 1,
        Applicability::NotApplicable => 0,
    }
}

/// Returns candidates tied on all policy/objective dimensions before the
/// identity fallback.
fn equally_ranked_candidates(
    candidates: &[SelectionCandidate],
) -> Vec<SelectionCandidate> {
    let Some(first) = candidates.first() else {
        return Vec::new();
    };

    candidates
        .iter()
        .filter(|candidate| {
            candidate.evaluation.applicability
                == first.evaluation.applicability
                && candidate.strategy_priority == first.strategy_priority
                && candidate.family_priority == first.family_priority
                && candidate.objective_score == first.objective_score
        })
        .cloned()
        .collect()
}

// =============================================================================
// Objective scoring
// =============================================================================

/// Produces an objective score from descriptor metadata.
///
/// This function deliberately does not convert qualitative overhead into
/// hardware-specific numerical cost. It only supplies a deterministic ordering
/// signal.
#[must_use]
fn objective_score(
    descriptor: &StrategyDescriptor,
    objective: SelectionObjective,
) -> i64 {
    match objective {
        SelectionObjective::ResourceEfficiency => {
            -overhead_score(descriptor)
        }

        SelectionObjective::ExecutionEfficiency => {
            -dimension_score(
                descriptor,
                OverheadDimension::Time,
            )
        }

        SelectionObjective::ClassicalEfficiency => {
            -dimension_score(
                descriptor,
                OverheadDimension::ClassicalComputation,
            )
        }

        SelectionObjective::VariantEfficiency => {
            -dimension_score(
                descriptor,
                OverheadDimension::Variants,
            )
        }

        SelectionObjective::Determinism => {
            if descriptor.deterministic {
                1
            } else {
                0
            }
        }

        SelectionObjective::Applicability
        | SelectionObjective::PolicyDefined => 0,
    }
}

/// Converts declared qualitative overhead into an ordering score.
///
/// This is NOT a physical resource quantity.
#[must_use]
const fn overhead_level_score(level: OverheadLevel) -> i64 {
    match level {
        OverheadLevel::None => 0,
        OverheadLevel::Low => 1,
        OverheadLevel::Medium => 2,
        OverheadLevel::High => 3,
        OverheadLevel::VeryHigh => 4,
        OverheadLevel::Unknown => 5,
    }
}

/// Calculates the aggregate qualitative overhead score.
#[must_use]
fn overhead_score(descriptor: &StrategyDescriptor) -> i64 {
    descriptor
        .expected_overhead
        .iter()
        .map(|item| overhead_level_score(item.level))
        .sum()
}

/// Calculates the qualitative score for one overhead dimension.
#[must_use]
fn dimension_score(
    descriptor: &StrategyDescriptor,
    dimension: OverheadDimension,
) -> i64 {
    descriptor
        .overhead_for(dimension)
        .map(overhead_level_score)
        .unwrap_or(overhead_level_score(OverheadLevel::Unknown))
}

// =============================================================================
// Configuration validation
// =============================================================================

fn validate_config(config: &SelectionConfig) -> Result<(), SelectionError> {
    validate_unique_strategy_priorities(&config.strategy_priorities)?;
    validate_unique_family_priorities(&config.family_priorities)?;

    if matches!(
        config.permission,
        MitigationPermission::Disabled
    ) && !config.strategy_priorities.is_empty()
    {
        return Err(SelectionError::DisabledPolicyWithPriorities);
    }

    Ok(())
}

fn validate_unique_strategy_priorities(
    priorities: &[StrategyPriority],
) -> Result<(), SelectionError> {
    for (index, current) in priorities.iter().enumerate() {
        if priorities[index + 1..]
            .iter()
            .any(|other| other.strategy_id == current.strategy_id)
        {
            return Err(SelectionError::DuplicateStrategyPriority(
                current.strategy_id.clone(),
            ));
        }
    }

    Ok(())
}

fn validate_unique_family_priorities(
    priorities: &[FamilyPriority],
) -> Result<(), SelectionError> {
    for (index, current) in priorities.iter().enumerate() {
        if priorities[index + 1..]
            .iter()
            .any(|other| other.family == current.family)
        {
            return Err(SelectionError::DuplicateFamilyPriority(
                current.family,
            ));
        }
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by mitigation selection.
///
/// These are selection-contract errors, not quantum execution errors.
/// Runtime/backend errors belong to the central resilience error subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionError {
    /// The same strategy was assigned explicit priority more than once.
    DuplicateStrategyPriority(StrategyId),

    /// The same family was assigned priority more than once.
    DuplicateFamilyPriority(StrategyFamily),

    /// Strategy priorities were supplied while mitigation was explicitly
    /// disabled.
    DisabledPolicyWithPriorities,

    /// Internal invariant: selection attempted to read an empty candidate set.
    InternalEmptyCandidateSet,
}

impl fmt::Display for SelectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateStrategyPriority(id) => {
                write!(
                    formatter,
                    "duplicate mitigation strategy priority for `{id}`"
                )
            }

            Self::DuplicateFamilyPriority(family) => {
                write!(
                    formatter,
                    "duplicate mitigation strategy family priority for `{family}`"
                )
            }

            Self::DisabledPolicyWithPriorities => {
                formatter.write_str(
                    "mitigation is disabled but strategy priorities were supplied",
                )
            }

            Self::InternalEmptyCandidateSet => {
                formatter.write_str(
                    "mitigation selector encountered an empty candidate set",
                )
            }
        }
    }
}

impl std::error::Error for SelectionError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::resilience::mitigation::strategy::{
        ExpectedOverhead,
        StrategyContext,
        StrategyPhase,
    };

    struct TestStrategy {
        descriptor: StrategyDescriptor,
    }

    impl TestStrategy {
        fn new(
            id: &str,
            family: StrategyFamily,
            deterministic: bool,
            overhead: OverheadLevel,
        ) -> Self {
            Self {
                descriptor: StrategyDescriptor {
                    id: StrategyId::new(id)
                        .expect("test strategy ID must be valid"),
                    version: StrategyVersion::new(1, 0, 0),
                    family,
                    phase: StrategyPhase::PostExecution,
                    description: Arc::from("test strategy"),
                    requirements: Arc::from([
                        StrategyRequirement::MeasurementResults,
                        StrategyRequirement::StatisticalAnalysis,
                        StrategyRequirement::Provenance,
                    ]),
                    expected_overhead: Arc::from([
                        ExpectedOverhead::new(
                            OverheadDimension::Executions,
                            overhead,
                        ),
                    ]),
                    deterministic,
                    requires_explicit_authorization: false,
                },
            }
        }
    }

    impl MitigationStrategy for TestStrategy {
        fn descriptor(&self) -> &StrategyDescriptor {
            &self.descriptor
        }
    }

    fn context() -> StrategyContext {
        StrategyContext {
            measurement_results_available: true,
            statistical_analysis_available: true,
            provenance_available: true,
            ..StrategyContext::default()
        }
    }

    fn strategy_set(
        strategies: Vec<TestStrategy>,
    ) -> StrategySet {
        StrategySet::from_strategies(
            strategies
                .into_iter()
                .map(|strategy| {
                    Arc::new(strategy) as Arc<dyn MitigationStrategy>
                }),
        )
    }

    #[test]
    fn disabled_policy_never_selects() {
        let strategies = strategy_set(vec![TestStrategy::new(
            "a",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        )]);

        let request = SelectionRequest::new(
            context(),
            SelectionConfig::disabled(),
        );

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert!(matches!(
            report.result,
            SelectionResult::Disabled
        ));
    }

    #[test]
    fn selects_an_applicable_strategy() {
        let strategies = strategy_set(vec![
            TestStrategy::new(
                "a",
                StrategyFamily::Custom,
                true,
                OverheadLevel::Low,
            ),
            TestStrategy::new(
                "b",
                StrategyFamily::Custom,
                true,
                OverheadLevel::High,
            ),
        ]);

        let request = SelectionRequest::new(
            context(),
            SelectionConfig::default()
                .with_objective(SelectionObjective::ResourceEfficiency),
        );

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert_eq!(
            report.result.strategy_id().map(StrategyId::as_str),
            Some("a")
        );
    }

    #[test]
    fn explicit_strategy_priority_overrides_objective() {
        let strategies = strategy_set(vec![
            TestStrategy::new(
                "cheap",
                StrategyFamily::Custom,
                true,
                OverheadLevel::Low,
            ),
            TestStrategy::new(
                "preferred",
                StrategyFamily::Custom,
                true,
                OverheadLevel::High,
            ),
        ]);

        let preferred_id =
            StrategyId::new("preferred").expect("valid test ID");

        let config = SelectionConfig::default()
            .with_objective(SelectionObjective::ResourceEfficiency)
            .with_strategy_priorities([
                StrategyPriority::new(preferred_id, 100),
            ])
            .expect("priority configuration should be valid");

        let request = SelectionRequest::new(context(), config);

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert_eq!(
            report.result.strategy_id().map(StrategyId::as_str),
            Some("preferred")
        );
    }

    #[test]
    fn deferred_strategy_is_not_executable() {
        let strategy = TestStrategy::new(
            "needs_data",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        );

        let strategies = strategy_set(vec![strategy]);

        let request = SelectionRequest::new(
            StrategyContext::default(),
            SelectionConfig::required(),
        );

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert!(matches!(
            report.result,
            SelectionResult::Deferred { .. }
        ));
    }

    #[test]
    fn required_policy_with_no_candidate_is_not_silently_accepted() {
        let strategies = StrategySet::new();

        let request = SelectionRequest::new(
            context(),
            SelectionConfig::required(),
        );

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert!(matches!(
            report.result,
            SelectionResult::NoApplicableStrategy { .. }
        ));
    }

    #[test]
    fn selection_is_independent_of_registration_order() {
        let first = TestStrategy::new(
            "a",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        );

        let second = TestStrategy::new(
            "b",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        );

        let request = SelectionRequest::new(
            context(),
            SelectionConfig::default(),
        );

        let first_report = MitigationSelector::new()
            .select(
                &strategy_set(vec![first, second]),
                &request,
            )
            .expect("selection should succeed");

        let first = TestStrategy::new(
            "a",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        );

        let second = TestStrategy::new(
            "b",
            StrategyFamily::Custom,
            true,
            OverheadLevel::Low,
        );

        let second_report = MitigationSelector::new()
            .select(
                &strategy_set(vec![second, first]),
                &request,
            )
            .expect("selection should succeed");

        assert_eq!(
            first_report.result.strategy_id(),
            second_report.result.strategy_id()
        );
    }

    #[test]
    fn explicit_tie_mode_reports_ambiguity() {
        let strategies = strategy_set(vec![
            TestStrategy::new(
                "a",
                StrategyFamily::Custom,
                true,
                OverheadLevel::Low,
            ),
            TestStrategy::new(
                "b",
                StrategyFamily::Custom,
                true,
                OverheadLevel::Low,
            ),
        ]);

        let config = SelectionConfig::default()
            .with_mode(SelectionMode::RequireExplicitTieBreak);

        let request = SelectionRequest::new(context(), config);

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert!(matches!(
            report.result,
            SelectionResult::Ambiguous { .. }
        ));
    }

    #[test]
    fn duplicate_strategy_priority_is_rejected() {
        let id = StrategyId::new("same")
            .expect("test ID must be valid");

        let result = SelectionConfig::default()
            .with_strategy_priorities([
                StrategyPriority::new(id.clone(), 1),
                StrategyPriority::new(id, 2),
            ]);

        assert!(matches!(
            result,
            Err(SelectionError::DuplicateStrategyPriority(_))
        ));
    }

    #[test]
    fn deterministic_objective_prefers_deterministic_strategy() {
        let strategies = strategy_set(vec![
            TestStrategy::new(
                "non_deterministic",
                StrategyFamily::Custom,
                false,
                OverheadLevel::Low,
            ),
            TestStrategy::new(
                "deterministic",
                StrategyFamily::Custom,
                true,
                OverheadLevel::Low,
            ),
        ]);

        let config = SelectionConfig::default()
            .with_objective(SelectionObjective::Determinism);

        let request = SelectionRequest::new(context(), config);

        let report = MitigationSelector::new()
            .select(&strategies, &request)
            .expect("selection should succeed");

        assert_eq!(
            report.result.strategy_id().map(StrategyId::as_str),
            Some("deterministic")
        );
    }

    #[test]
    fn empty_strategy_set_is_safe() {
        let request = SelectionRequest::new(
            context(),
            SelectionConfig::default(),
        );

        let report = MitigationSelector::new()
            .select(&StrategySet::new(), &request)
            .expect("empty strategy set is a valid selection state");

        assert!(matches!(
            report.result,
            SelectionResult::NoApplicableStrategy { .. }
        ));
    }
}