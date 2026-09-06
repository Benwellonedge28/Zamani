//! Zamani Quantum Resilience — Deterministic Recovery-Plan Ranking
//!
//! Path:
//!     src/quantum/resilience/planning/ranking.rs
//!
//! Purpose:
//!     Provide a production-grade, provider-independent, deterministic ranking
//!     engine for resilience recovery-plan candidates.
//!
//! Architectural position:
//!
//!     Detection
//!         |
//!         v
//!     Diagnosis
//!         |
//!         v
//!     Policy
//!         |
//!         v
//!     Planner
//!         |
//!         +--> action.rs
//!         +--> feasibility.rs
//!         +--> cost.rs
//!         +--> plan.rs
//!         |
//!         v
//!     ranking.rs
//!         |
//!         v
//!     selected RecoveryPlan
//!         |
//!         v
//!     adaptation / recovery / mitigation
//!         |
//!         v
//!     verification
//!
//! -----------------------------------------------------------------------------
//! Responsibility
//! -----------------------------------------------------------------------------
//!
//! This module answers:
//!
//!     "Given already-generated candidate plans and their normalized evidence,
//!      which candidates are preferable under the supplied ranking policy?"
//!
//! This module does NOT:
//!
//! - execute recovery;
//! - determine hardware capabilities;
//! - discover hardware;
//! - diagnose faults;
//! - determine feasibility;
//! - authorize actions;
//! - perform routing;
//! - perform scheduling;
//! - compile;
//! - optimize;
//! - perform QEC;
//! - perform error mitigation;
//! - communicate with a quantum backend;
//! - mutate Quantum IR;
//! - invent retry counts;
//! - assume a fixed number of qubits;
//! - assume a fixed number of machines;
//! - assume a fixed provider;
//! - treat estimated cost as execution truth.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! -----------------------------------------------------------------------------
//! Production invariants
//! -----------------------------------------------------------------------------
//!
//! 1. No unsafe code.
//! 2. No provider-specific logic.
//! 3. No fixed quantum-machine-size limit.
//! 4. No hard-coded retry count.
//! 5. No hard-coded fidelity threshold.
//! 6. No hard-coded qubit count.
//! 7. No hard-coded backend count.
//! 8. No wall-clock access.
//! 9. No randomness.
//! 10. No global mutable state.
//! 11. No HashMap iteration-order dependence.
//! 12. No floating-point NaN/infinity semantics.
//! 13. Checked arithmetic for ranking scores.
//! 14. Explicit handling of infeasible and unknown candidates.
//! 15. Deterministic tie breaking.
//! 16. Caller-supplied ranking policy.
//! 17. Caller-supplied candidate limits.
//! 18. Ranking never becomes authorization.
//!
//! -----------------------------------------------------------------------------
//! "Atom to everywhere"
//! -----------------------------------------------------------------------------
//!
//! This module introduces no architectural quantum-resource ceiling.
//!
//! A ranking invocation can operate on candidates representing:
//!
//!     one-qubit execution
//!     small QPU
//!     large QPU
//!     logical fault-tolerant machine
//!     heterogeneous backend fleet
//!     distributed quantum system
//!
//! without changing the ranking algorithm.
//!
//! The only unavoidable limits are the limits of the Rust process, available
//! memory, caller-supplied limits, and the number of candidates actually
//! provided to this module.
//!
//! The number of physical qubits is deliberately irrelevant to this module.
//!
//! -----------------------------------------------------------------------------
//! Canonical quantum identity
//! -----------------------------------------------------------------------------
//!
//! Ranking normally does not need to inspect individual qubits. If an adapter
//! needs quantum-resource identity, it MUST use the canonical repository types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and, where exposed by the repository contract:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file deliberately does not define another QubitId.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! planning/action.rs
//!     Provides ActionKind and action semantics.
//!
//! planning/cost.rs
//!     Remains the authoritative multidimensional resilience cost model.
//!     This ranking module intentionally does not duplicate RecoveryCost.
//!
//! planning/feasibility.rs
//!     Provides normalized feasibility results to the planner. Ranking treats
//!     feasibility as input evidence; it does not recompute feasibility.
//!
//! planning/plan.rs
//!     Owns the immutable RecoveryPlan representation. The planner/adaptor
//!     supplies RankingCandidate values derived from RecoveryPlan instances.
//!
//! planning/planner.rs
//!     Generates candidate plans and invokes this ranking engine.
//!
//! policy/*
//!     Supplies objective weights, safety requirements, budgets and preferences.
//!
//! verification/*
//!     Remains the final authority over result acceptance.
//!
//! adaptation/* / recovery/* / mitigation/*
//!     Execute the selected plan after independent authorization gates.
//!
//! -----------------------------------------------------------------------------
//! Important design decision
//! -----------------------------------------------------------------------------
//!
//! `RankingCandidate` is intentionally NOT `RecoveryPlan`.
//!
//! This avoids coupling the ranking algorithm to the complete plan object and
//! prevents ranking from becoming responsible for plan construction.
//!
//! The planner creates a normalized ranking view:
//
//!     RecoveryPlan
//!         |
//!         v
//!     RankingCandidate
//!         |
//!         v
//!     RankingEngine
//!         |
//!         v
//!     RankedCandidate
//!
//! This means the immutable plan contract can evolve independently from the
//! ranking implementation.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! Ranking is deterministic for identical normalized inputs and identical
//! RankingPolicy values.
//!
//! Ordering is established using:
//!
//!     1. feasibility class;
//!     2. policy-selected objective score;
//!     3. protective priority where configured;
//!     4. confidence/risk evidence where configured;
//!     5. stable candidate identity.
//!
//! The final candidate identity comparison is mandatory so that two otherwise
//! identical candidates never depend on input order.
//!
//! -----------------------------------------------------------------------------
//! Numeric model
//! -----------------------------------------------------------------------------
//!
//! Floating-point numbers are deliberately avoided.
//!
//! Scores use signed i128 fixed-point arithmetic.
//!
//! A policy weight is represented by `FixedScore` using six decimal places.
//!
//! This gives deterministic ordering and avoids:
//!
//!     NaN
//!     +infinity
//!     -infinity
//!
//! The fixed-point precision is a representation choice, NOT a quantum-machine
//! limit.
//!
//! -----------------------------------------------------------------------------
//! Safety invariant
//! -----------------------------------------------------------------------------
//!
//! Ranking can recommend a candidate.
//!
//! Ranking cannot authorize it.
//!
//! Therefore:
//
//!     ranked == selected
//!
//! does NOT imply:
//
//!     executable
//!
//! Execution must still pass:
//!
//!     policy
//!     + authorization
//!     + feasibility
//!     + execution-time preconditions
//!     + semantic verification
//!
//! -----------------------------------------------------------------------------
//! Rust compatibility
//! -----------------------------------------------------------------------------
//!
//! Compatible with:
//!
//!     Rust 1.97
//!     Rust 1.97.1
//!     Rust 2021
//!     stable Rust
//!     no nightly features
//!     no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroUsize;

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable schema identifier for resilience ranking.
pub const RANKING_SCHEMA_ID: &str =
    "zamani.quantum.resilience.planning.ranking";

/// Semantic schema version.
pub const RANKING_SCHEMA_VERSION: u16 = 1;

/// Implementation version.
pub const RANKING_IMPLEMENTATION_VERSION: u32 = 1;

/// Fixed decimal precision used for ranking scores.
pub const SCORE_SCALE: i128 = 1_000_000;

// =============================================================================
// Fixed score
// =============================================================================

/// Deterministic signed fixed-point score.
///
/// The represented value is:
///
///     units / 1_000_000
///
/// This type is intentionally independent of hardware size and provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FixedScore {
    units: i128,
}

impl FixedScore {
    /// Zero.
    pub const ZERO: Self = Self { units: 0 };

    /// One.
    pub const ONE: Self = Self {
        units: SCORE_SCALE,
    };

    /// Creates a score from an integer.
    ///
    /// Returns `None` if scaling cannot be represented.
    #[must_use]
    pub const fn from_integer(value: i128) -> Option<Self> {
        match value.checked_mul(SCORE_SCALE) {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Creates a score from already-scaled units.
    #[must_use]
    pub const fn from_scaled_units(units: i128) -> Self {
        Self { units }
    }

    /// Returns scaled units.
    #[must_use]
    pub const fn scaled_units(self) -> i128 {
        self.units
    }

    /// Returns whether the score is negative.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.units < 0
    }

    /// Adds two scores with overflow detection.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.units.checked_add(other.units) {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Subtracts two scores with overflow detection.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.units.checked_sub(other.units) {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }

    /// Multiplies two fixed-point scores.
    #[must_use]
    pub const fn checked_mul(self, other: Self) -> Option<Self> {
        match self.units.checked_mul(other.units) {
            Some(product) => Some(Self {
                units: product / SCORE_SCALE,
            }),
            None => None,
        }
    }

    /// Returns the absolute value.
    #[must_use]
    pub const fn checked_abs(self) -> Option<Self> {
        match self.units.checked_abs() {
            Some(units) => Some(Self { units }),
            None => None,
        }
    }
}

impl PartialOrd for FixedScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FixedScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.units.cmp(&other.units)
    }
}

impl fmt::Display for FixedScore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negative = self.units < 0;
        let magnitude = self.units.unsigned_abs();

        let whole = magnitude / SCORE_SCALE as u128;
        let fractional = magnitude % SCORE_SCALE as u128;

        if negative {
            write!(formatter, "-")?;
        }

        write!(formatter, "{whole}.{fractional:06}")
    }
}

// =============================================================================
// Ranking objective
// =============================================================================

/// Ranking dimensions understood by the resilience planner.
///
/// These are objective semantics, not measurements.
///
/// The actual values are supplied by `RankingCandidate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RankingObjective {
    /// Prefer higher execution confidence.
    Confidence,

    /// Prefer lower estimated execution time.
    Time,

    /// Prefer fewer additional shots.
    Shots,

    /// Prefer lower resource pressure.
    ResourcePressure,

    /// Prefer lower logical-error contribution.
    LogicalError,

    /// Prefer lower estimated financial cost.
    Financial,

    /// Prefer lower compilation overhead.
    Compilation,

    /// Prefer lower routing overhead.
    Routing,

    /// Prefer lower scheduling overhead.
    Scheduling,

    /// Prefer lower QEC overhead.
    Qec,

    /// Prefer lower mitigation overhead.
    Mitigation,

    /// Prefer lower migration overhead.
    Migration,

    /// Prefer lower verification overhead.
    Verification,

    /// Prefer plans requiring fewer adaptation stages.
    AdaptationComplexity,

    /// Prefer plans with stronger safety evidence.
    Safety,

    /// Prefer plans with stronger semantic-preservation evidence.
    SemanticPreservation,
}

impl RankingObjective {
    /// Stable serialized identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confidence => "confidence",
            Self::Time => "time",
            Self::Shots => "shots",
            Self::ResourcePressure => "resource_pressure",
            Self::LogicalError => "logical_error",
            Self::Financial => "financial",
            Self::Compilation => "compilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Qec => "qec",
            Self::Mitigation => "mitigation",
            Self::Migration => "migration",
            Self::Verification => "verification",
            Self::AdaptationComplexity => "adaptation_complexity",
            Self::Safety => "safety",
            Self::SemanticPreservation => "semantic_preservation",
        }
    }

    /// Whether a larger value is preferable.
    #[must_use]
    pub const fn maximize(self) -> bool {
        matches!(
            self,
            Self::Confidence
                | Self::Safety
                | Self::SemanticPreservation
        )
    }
}

// =============================================================================
// Feasibility class
// =============================================================================

/// Ranking-level classification of candidate feasibility.
///
/// Ranking does not calculate feasibility. It consumes this normalized result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeasibilityClass {
    /// Candidate is definitely infeasible.
    Infeasible,

    /// Candidate cannot currently be proven feasible.
    Unknown,

    /// Candidate is feasible under supplied evidence.
    Feasible,
}

impl FeasibilityClass {
    /// Returns whether the candidate is feasible.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Returns whether the candidate is definitively infeasible.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Returns whether the candidate requires more information.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Candidate identifier
// =============================================================================

/// Stable identity used exclusively for deterministic tie-breaking.
///
/// This is deliberately caller supplied.
///
/// The ranking layer does not generate UUIDs or random identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CandidateId(String);

impl CandidateId {
    /// Creates a candidate identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, RankingError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RankingError::EmptyCandidateId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CandidateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Candidate metrics
// =============================================================================

/// Normalized objective metrics for one candidate.
///
/// All values are caller supplied.
///
/// A missing metric means "unknown", not zero.
///
/// The planner may derive these values from:
///
///     planning/cost.rs
///     planning/feasibility.rs
///     policy/*
///     history/*
///     verification/*
///
/// The ranking layer does not reinterpret missing values as successful
/// execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RankingMetrics {
    /// Confidence in the candidate's expected outcome.
    pub confidence: Option<FixedScore>,

    /// Estimated execution time.
    pub time: Option<FixedScore>,

    /// Estimated additional shots.
    pub shots: Option<FixedScore>,

    /// Estimated resource pressure.
    pub resource_pressure: Option<FixedScore>,

    /// Estimated logical-error contribution.
    pub logical_error: Option<FixedScore>,

    /// Estimated financial cost.
    pub financial: Option<FixedScore>,

    /// Estimated compilation overhead.
    pub compilation: Option<FixedScore>,

    /// Estimated routing overhead.
    pub routing: Option<FixedScore>,

    /// Estimated scheduling overhead.
    pub scheduling: Option<FixedScore>,

    /// Estimated QEC overhead.
    pub qec: Option<FixedScore>,

    /// Estimated mitigation overhead.
    pub mitigation: Option<FixedScore>,

    /// Estimated migration overhead.
    pub migration: Option<FixedScore>,

    /// Estimated verification overhead.
    pub verification: Option<FixedScore>,

    /// Normalized adaptation complexity.
    pub adaptation_complexity: Option<FixedScore>,

    /// Safety evidence score.
    pub safety: Option<FixedScore>,

    /// Semantic-preservation evidence score.
    pub semantic_preservation: Option<FixedScore>,
}

impl RankingMetrics {
    /// Returns a metric by objective.
    #[must_use]
    pub const fn get(self, objective: RankingObjective) -> Option<FixedScore> {
        match objective {
            RankingObjective::Confidence => self.confidence,
            RankingObjective::Time => self.time,
            RankingObjective::Shots => self.shots,
            RankingObjective::ResourcePressure => self.resource_pressure,
            RankingObjective::LogicalError => self.logical_error,
            RankingObjective::Financial => self.financial,
            RankingObjective::Compilation => self.compilation,
            RankingObjective::Routing => self.routing,
            RankingObjective::Scheduling => self.scheduling,
            RankingObjective::Qec => self.qec,
            RankingObjective::Mitigation => self.mitigation,
            RankingObjective::Migration => self.migration,
            RankingObjective::Verification => self.verification,
            RankingObjective::AdaptationComplexity => {
                self.adaptation_complexity
            }
            RankingObjective::Safety => self.safety,
            RankingObjective::SemanticPreservation => {
                self.semantic_preservation
            }
        }
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// Normalized candidate consumed by the ranking engine.
///
/// This is intentionally smaller than `RecoveryPlan`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankingCandidate {
    id: CandidateId,
    feasibility: FeasibilityClass,
    metrics: RankingMetrics,

    /// Number of actions in the candidate plan.
    ///
    /// This is supplied by the planner and is not interpreted as a quantum
    /// machine-size value.
    action_count: usize,

    /// Whether this candidate is protective.
    ///
    /// This should be derived from `planning::action::ActionKind`.
    protective: bool,

    /// Caller-supplied deterministic secondary priority.
    priority: i64,
}

impl RankingCandidate {
    /// Creates a candidate.
    pub fn new(
        id: CandidateId,
        feasibility: FeasibilityClass,
        metrics: RankingMetrics,
        action_count: usize,
        protective: bool,
        priority: i64,
    ) -> Self {
        Self {
            id,
            feasibility,
            metrics,
            action_count,
            protective,
            priority,
        }
    }

    /// Returns the stable candidate identifier.
    #[must_use]
    pub fn id(&self) -> &CandidateId {
        &self.id
    }

    /// Returns feasibility.
    #[must_use]
    pub const fn feasibility(&self) -> FeasibilityClass {
        self.feasibility
    }

    /// Returns metrics.
    #[must_use]
    pub const fn metrics(&self) -> RankingMetrics {
        self.metrics
    }

    /// Returns action count.
    #[must_use]
    pub const fn action_count(&self) -> usize {
        self.action_count
    }

    /// Returns whether this candidate is protective.
    #[must_use]
    pub const fn protective(&self) -> bool {
        self.protective
    }

    /// Returns planner-supplied priority.
    #[must_use]
    pub const fn priority(&self) -> i64 {
        self.priority
    }
}

// =============================================================================
// Missing metric policy
// =============================================================================

/// Defines how ranking handles an objective whose metric is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingMetricPolicy {
    /// Reject the candidate from ranking when a required metric is absent.
    Reject,

    /// Keep the candidate but rank it after candidates with known metrics.
    UnknownAfterKnown,

    /// Treat missing metrics as neutral.
    ///
    /// This is useful only when the caller has explicitly established that a
    /// missing dimension is irrelevant to the selected objective.
    Neutral,
}

// =============================================================================
// Feasibility policy
// =============================================================================

/// Defines how unknown and infeasible candidates participate in ranking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeasibilityPolicy {
    /// Rank only feasible candidates.
    FeasibleOnly,

    /// Rank feasible candidates first, then unknown candidates.
    FeasibleThenUnknown,

    /// Rank all candidates while keeping infeasible candidates last.
    IncludeInfeasible,

    /// Rank all candidates exactly according to objective score, while still
    /// retaining feasibility state for the caller.
    ObjectiveOnly,
}

// =============================================================================
// Ranking mode
// =============================================================================

/// Primary ranking algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RankingMode {
    /// Weighted sum of configured objective scores.
    Weighted,

    /// Lexicographic comparison using objective order.
    Lexicographic,

    /// Pareto-frontier ordering followed by deterministic tie breaking.
    Pareto,

    /// Feasibility first, then weighted objective ranking.
    FeasibilityFirst,
}

// =============================================================================
// Objective weight
// =============================================================================

/// Weighted ranking objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectiveWeight {
    /// Objective being weighted.
    pub objective: RankingObjective,

    /// Weight.
    pub weight: FixedScore,

    /// Missing-value handling.
    pub missing: MissingMetricPolicy,
}

impl ObjectiveWeight {
    /// Creates a weighted objective.
    #[must_use]
    pub const fn new(
        objective: RankingObjective,
        weight: FixedScore,
        missing: MissingMetricPolicy,
    ) -> Self {
        Self {
            objective,
            weight,
            missing,
        }
    }
}

// =============================================================================
// Ranking limits
// =============================================================================

/// Caller-supplied limits for one ranking operation.
///
/// These are operational safeguards, not quantum-machine limits.
///
/// `None` means no ranking-layer limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RankingLimits {
    /// Maximum number of input candidates to accept.
    pub max_candidates: Option<NonZeroUsize>,

    /// Maximum number of ranked results to return.
    pub max_results: Option<NonZeroUsize>,
}

impl RankingLimits {
    /// No ranking-layer limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_candidates: None,
            max_results: None,
        }
    }

    fn validate(self) -> Result<Self, RankingError> {
        if let Some(limit) = self.max_candidates {
            if limit.get() == 0 {
                return Err(RankingError::InvalidLimit);
            }
        }

        if let Some(limit) = self.max_results {
            if limit.get() == 0 {
                return Err(RankingError::InvalidLimit);
            }
        }

        Ok(self)
    }
}

// =============================================================================
// Ranking policy
// =============================================================================

/// Complete ranking policy.
///
/// Nothing in this structure is tied to a specific QPU, provider, qubit
/// count, topology or retry count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankingPolicy {
    /// Primary ranking mode.
    pub mode: RankingMode,

    /// Feasibility handling.
    pub feasibility: FeasibilityPolicy,

    /// Objective weights in deterministic caller-defined order.
    pub objectives: Vec<ObjectiveWeight>,

    /// Whether protective candidates receive priority when other scores tie.
    pub prioritize_protective: bool,

    /// Whether caller priority participates in tie breaking.
    pub use_priority: bool,

    /// Operational limits.
    pub limits: RankingLimits,
}

impl RankingPolicy {
    /// Creates a policy.
    pub fn new(
        mode: RankingMode,
        feasibility: FeasibilityPolicy,
        objectives: Vec<ObjectiveWeight>,
    ) -> Result<Self, RankingError> {
        if objectives.is_empty() {
            return Err(RankingError::NoObjectives);
        }

        Self::validate_objectives(&objectives)?;

        Ok(Self {
            mode,
            feasibility,
            objectives,
            prioritize_protective: true,
            use_priority: true,
            limits: RankingLimits::unlimited(),
        })
    }

    /// Returns a strict safety-oriented weighted policy.
    ///
    /// This is a policy constructor, not a hardware-specific default.
    pub fn strict_safety() -> Result<Self, RankingError> {
        Self::new(
            RankingMode::FeasibilityFirst,
            FeasibilityPolicy::FeasibleThenUnknown,
            vec![
                ObjectiveWeight::new(
                    RankingObjective::Safety,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
                ObjectiveWeight::new(
                    RankingObjective::SemanticPreservation,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
                ObjectiveWeight::new(
                    RankingObjective::Confidence,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
                ObjectiveWeight::new(
                    RankingObjective::LogicalError,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
            ],
        )
    }

    /// Validates objective configuration.
    fn validate_objectives(
        objectives: &[ObjectiveWeight],
    ) -> Result<(), RankingError> {
        for (index, objective) in objectives.iter().enumerate() {
            if objective.weight.is_negative() {
                return Err(RankingError::NegativeWeight { index });
            }
        }

        Ok(())
    }

    /// Validates the entire policy.
    pub fn validate(&self) -> Result<(), RankingError> {
        if self.objectives.is_empty() {
            return Err(RankingError::NoObjectives);
        }

        Self::validate_objectives(&self.objectives)?;
        self.limits.validate()?;

        Ok(())
    }
}

// =============================================================================
// Score component
// =============================================================================

/// One auditable score contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScoreComponent {
    /// Objective.
    pub objective: RankingObjective,

    /// Normalized metric.
    pub metric: Option<FixedScore>,

    /// Applied weight.
    pub weight: FixedScore,

    /// Signed contribution.
    pub contribution: Option<FixedScore>,
}

// =============================================================================
// Candidate score
// =============================================================================

/// Complete deterministic score of one candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateScore {
    /// Candidate identity.
    pub id: CandidateId,

    /// Feasibility classification.
    pub feasibility: FeasibilityClass,

    /// Total weighted score.
    pub total: Option<FixedScore>,

    /// Objective contributions.
    pub components: Vec<ScoreComponent>,

    /// Whether at least one required metric was unknown.
    pub has_unknown_metric: bool,

    /// Whether the candidate is Pareto dominated.
    pub pareto_dominated: bool,
}

impl CandidateScore {
    fn invalid_metric(&self) -> bool {
        self.has_unknown_metric
    }
}

// =============================================================================
// Ranked candidate
// =============================================================================

/// Candidate plus its complete ranking evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedCandidate {
    /// Original candidate.
    pub candidate: RankingCandidate,

    /// Calculated score.
    pub score: CandidateScore,

    /// Final deterministic rank, one-based.
    pub rank: usize,
}

// =============================================================================
// Ranking result
// =============================================================================

/// Complete ranking result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankingResult {
    /// Ranked candidates.
    pub candidates: Vec<RankedCandidate>,

    /// Number of candidates supplied before output limiting.
    pub input_count: usize,

    /// Number of candidates rejected by the ranking policy.
    pub rejected_count: usize,
}

impl RankingResult {
    /// Returns the highest-ranked candidate.
    #[must_use]
    pub fn best(&self) -> Option<&RankedCandidate> {
        self.candidates.first()
    }

    /// Returns whether no candidate survived ranking.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Returns the number of retained candidates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.candidates.len()
    }
}

// =============================================================================
// Ranking errors
// =============================================================================

/// Errors produced by the ranking layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RankingError {
    /// Candidate identifier was empty.
    EmptyCandidateId,

    /// No objectives were supplied.
    NoObjectives,

    /// An objective weight was negative.
    NegativeWeight {
        /// Objective index.
        index: usize,
    },

    /// A ranking limit was invalid.
    InvalidLimit,

    /// Candidate count exceeded the caller-supplied limit.
    CandidateLimitExceeded {
        /// Number supplied.
        actual: usize,

        /// Maximum accepted.
        maximum: usize,
    },

    /// Score arithmetic overflowed.
    ScoreOverflow,

    /// Required metric was unavailable.
    MissingMetric {
        /// Candidate identity.
        candidate: CandidateId,

        /// Objective whose metric was missing.
        objective: RankingObjective,
    },

    /// No rankable candidates remained.
    NoRankableCandidates,

    /// A policy was internally inconsistent.
    InvalidPolicy,
}

impl fmt::Display for RankingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCandidateId => {
                formatter.write_str("ranking candidate identifier must not be empty")
            }
            Self::NoObjectives => {
                formatter.write_str("ranking policy must contain at least one objective")
            }
            Self::NegativeWeight { index } => {
                write!(formatter, "ranking objective {index} has a negative weight")
            }
            Self::InvalidLimit => {
                formatter.write_str("ranking limit must be non-zero")
            }
            Self::CandidateLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "ranking candidate limit exceeded: {actual} > {maximum}"
                )
            }
            Self::ScoreOverflow => {
                formatter.write_str("ranking score arithmetic overflowed")
            }
            Self::MissingMetric {
                candidate,
                objective,
            } => {
                write!(
                    formatter,
                    "candidate {candidate} is missing metric {}",
                    objective.as_str()
                )
            }
            Self::NoRankableCandidates => {
                formatter.write_str("no rankable recovery-plan candidates remain")
            }
            Self::InvalidPolicy => {
                formatter.write_str("ranking policy is internally inconsistent")
            }
        }
    }
}

impl std::error::Error for RankingError {}

// =============================================================================
// Ranking engine
// =============================================================================

/// Deterministic ranking engine.
///
/// The engine contains no mutable global state and is safe to construct per
/// planner invocation.
#[derive(Debug, Clone)]
pub struct RankingEngine {
    policy: RankingPolicy,
}

impl RankingEngine {
    /// Creates a ranking engine.
    pub fn new(policy: RankingPolicy) -> Result<Self, RankingError> {
        policy.validate()?;

        Ok(Self { policy })
    }

    /// Returns the ranking policy.
    #[must_use]
    pub fn policy(&self) -> &RankingPolicy {
        &self.policy
    }

    /// Ranks candidates.
    ///
    /// Complexity:
    ///
    ///     O(n log n)
    ///
    /// where `n` is the number of supplied candidates.
    ///
    /// The complexity depends on candidate count, NOT qubit count.
    pub fn rank<I>(
        &self,
        candidates: I,
    ) -> Result<RankingResult, RankingError>
    where
        I: IntoIterator<Item = RankingCandidate>,
    {
        let candidates: Vec<RankingCandidate> = candidates.into_iter().collect();

        let input_count = candidates.len();

        if let Some(limit) = self.policy.limits.max_candidates {
            if input_count > limit.get() {
                return Err(RankingError::CandidateLimitExceeded {
                    actual: input_count,
                    maximum: limit.get(),
                });
            }
        }

        let mut scored = Vec::with_capacity(input_count);
        let mut rejected_count = 0usize;

        for candidate in candidates {
            if !self.include_by_feasibility(candidate.feasibility()) {
                rejected_count = rejected_count
                    .checked_add(1)
                    .ok_or(RankingError::ScoreOverflow)?;

                continue;
            }

            let score = self.score_candidate(&candidate)?;

            if score.invalid_metric()
                && self
                    .policy
                    .objectives
                    .iter()
                    .any(|objective| {
                        matches!(
                            objective.missing,
                            MissingMetricPolicy::Reject
                        )
                    })
            {
                rejected_count = rejected_count
                    .checked_add(1)
                    .ok_or(RankingError::ScoreOverflow)?;

                continue;
            }

            scored.push((candidate, score));
        }

        if scored.is_empty() {
            return Err(RankingError::NoRankableCandidates);
        }

        if matches!(self.policy.mode, RankingMode::Pareto) {
            self.mark_pareto_dominance(&mut scored);
        }

        scored.sort_by(|left, right| self.compare(left, right));

        let max_results = self
            .policy
            .limits
            .max_results
            .map(NonZeroUsize::get)
            .unwrap_or(scored.len());

        let retained = scored.len().min(max_results);

        let candidates = scored
            .into_iter()
            .take(retained)
            .enumerate()
            .map(|(index, (candidate, score))| RankedCandidate {
                candidate,
                score,
                rank: index.saturating_add(1),
            })
            .collect();

        Ok(RankingResult {
            candidates,
            input_count,
            rejected_count,
        })
    }

    // -------------------------------------------------------------------------
    // Feasibility
    // -------------------------------------------------------------------------

    fn include_by_feasibility(&self, feasibility: FeasibilityClass) -> bool {
        match self.policy.feasibility {
            FeasibilityPolicy::FeasibleOnly => feasibility.is_feasible(),

            FeasibilityPolicy::FeasibleThenUnknown => {
                !feasibility.is_infeasible()
            }

            FeasibilityPolicy::IncludeInfeasible => true,

            FeasibilityPolicy::ObjectiveOnly => true,
        }
    }

    fn feasibility_order(
        &self,
        feasibility: FeasibilityClass,
    ) -> u8 {
        match self.policy.feasibility {
            FeasibilityPolicy::FeasibleOnly => match feasibility {
                FeasibilityClass::Feasible => 0,
                FeasibilityClass::Unknown => 1,
                FeasibilityClass::Infeasible => 2,
            },

            FeasibilityPolicy::FeasibleThenUnknown => match feasibility {
                FeasibilityClass::Feasible => 0,
                FeasibilityClass::Unknown => 1,
                FeasibilityClass::Infeasible => 2,
            },

            FeasibilityPolicy::IncludeInfeasible => match feasibility {
                FeasibilityClass::Feasible => 0,
                FeasibilityClass::Unknown => 1,
                FeasibilityClass::Infeasible => 2,
            },

            FeasibilityPolicy::ObjectiveOnly => 0,
        }
    }

    // -------------------------------------------------------------------------
    // Scoring
    // -------------------------------------------------------------------------

    fn score_candidate(
        &self,
        candidate: &RankingCandidate,
    ) -> Result<CandidateScore, RankingError> {
        let mut components =
            Vec::with_capacity(self.policy.objectives.len());

        let mut total = Some(FixedScore::ZERO);
        let mut has_unknown_metric = false;

        for objective in &self.policy.objectives {
            let metric = candidate.metrics().get(objective.objective);

            let contribution = match metric {
                Some(value) => {
                    let normalized = if objective.objective.maximize() {
                        value
                    } else {
                        // Lower is better. Convert the metric into a
                        // higher-is-better score by negating it.
                        FixedScore::from_scaled_units(
                            value.scaled_units().checked_neg().ok_or(
                                RankingError::ScoreOverflow,
                            )?,
                        )
                    };

                    normalized
                        .checked_mul(objective.weight)
                        .ok_or(RankingError::ScoreOverflow)
                        .map(Some)?
                }

                None => {
                    has_unknown_metric = true;

                    match objective.missing {
                        MissingMetricPolicy::Reject => None,

                        MissingMetricPolicy::UnknownAfterKnown => None,

                        MissingMetricPolicy::Neutral => {
                            Some(FixedScore::ZERO)
                        }
                    }
                }
            };

            if let Some(value) = contribution {
                total = total
                    .unwrap_or(FixedScore::ZERO)
                    .checked_add(value)
                    .ok_or(RankingError::ScoreOverflow)
                    .map(Some)?;
            } else {
                total = None;
            }

            components.push(ScoreComponent {
                objective: objective.objective,
                metric,
                weight: objective.weight,
                contribution,
            });
        }

        Ok(CandidateScore {
            id: candidate.id().clone(),
            feasibility: candidate.feasibility(),
            total,
            components,
            has_unknown_metric,
            pareto_dominated: false,
        })
    }

    // -------------------------------------------------------------------------
    // Pareto
    // -------------------------------------------------------------------------

    fn mark_pareto_dominance(
        &self,
        candidates: &mut [(RankingCandidate, CandidateScore)],
    ) {
        for index in 0..candidates.len() {
            let mut dominated = false;

            for other_index in 0..candidates.len() {
                if index == other_index {
                    continue;
                }

                if self.dominates(
                    &candidates[other_index].0,
                    &candidates[index].0,
                ) {
                    dominated = true;
                    break;
                }
            }

            candidates[index].1.pareto_dominated = dominated;
        }
    }

    fn dominates(
        &self,
        left: &RankingCandidate,
        right: &RankingCandidate,
    ) -> bool {
        let mut strictly_better = false;

        for objective in &self.policy.objectives {
            let left_metric = left.metrics().get(objective.objective);
            let right_metric = right.metrics().get(objective.objective);

            let (Some(left_value), Some(right_value)) =
                (left_metric, right_metric)
            else {
                continue;
            };

            let comparison = if objective.objective.maximize() {
                left_value.cmp(&right_value)
            } else {
                right_value.cmp(&left_value)
            };

            if comparison == Ordering::Less {
                return false;
            }

            if comparison == Ordering::Greater {
                strictly_better = true;
            }
        }

        strictly_better
    }

    // -------------------------------------------------------------------------
    // Ordering
    // -------------------------------------------------------------------------

    fn compare(
        &self,
        left: &(RankingCandidate, CandidateScore),
        right: &(RankingCandidate, CandidateScore),
    ) -> Ordering {
        if !matches!(
            self.policy.feasibility,
            FeasibilityPolicy::ObjectiveOnly
        ) {
            let left_feasibility =
                self.feasibility_order(left.0.feasibility());

            let right_feasibility =
                self.feasibility_order(right.0.feasibility());

            let feasibility_order =
                left_feasibility.cmp(&right_feasibility);

            if feasibility_order != Ordering::Equal {
                return feasibility_order;
            }
        }

        match self.policy.mode {
            RankingMode::Weighted
            | RankingMode::FeasibilityFirst => {
                let weighted = self.compare_total_score(left, right);

                if weighted != Ordering::Equal {
                    return weighted;
                }
            }

            RankingMode::Lexicographic => {
                let lexicographic =
                    self.compare_lexicographic(left, right);

                if lexicographic != Ordering::Equal {
                    return lexicographic;
                }
            }

            RankingMode::Pareto => {
                let left_dominated = left.1.pareto_dominated;
                let right_dominated = right.1.pareto_dominated;

                match (left_dominated, right_dominated) {
                    (false, true) => return Ordering::Less,
                    (true, false) => return Ordering::Greater,
                    _ => {}
                }

                let weighted = self.compare_total_score(left, right);

                if weighted != Ordering::Equal {
                    return weighted;
                }
            }
        }

        if self.policy.prioritize_protective {
            let protective_order =
                right.0.protective().cmp(&left.0.protective());

            if protective_order != Ordering::Equal {
                return protective_order;
            }
        }

        if self.policy.use_priority {
            let priority_order =
                right.0.priority().cmp(&left.0.priority());

            if priority_order != Ordering::Equal {
                return priority_order;
            }
        }

        // Fewer actions is preferred as a deterministic, provider-neutral
        // complexity tie-breaker.
        let action_count_order =
            left.0.action_count().cmp(&right.0.action_count());

        if action_count_order != Ordering::Equal {
            return action_count_order;
        }

        // Final mandatory total ordering.
        left.0.id().cmp(right.0.id())
    }

    fn compare_total_score(
        &self,
        left: &(RankingCandidate, CandidateScore),
        right: &(RankingCandidate, CandidateScore),
    ) -> Ordering {
        match (left.1.total, right.1.total) {
            (Some(left_score), Some(right_score)) => {
                right_score.cmp(&left_score)
            }

            (Some(_), None) => Ordering::Less,

            (None, Some(_)) => Ordering::Greater,

            (None, None) => Ordering::Equal,
        }
    }

    fn compare_lexicographic(
        &self,
        left: &(RankingCandidate, CandidateScore),
        right: &(RankingCandidate, CandidateScore),
    ) -> Ordering {
        for objective in &self.policy.objectives {
            let left_metric =
                left.0.metrics().get(objective.objective);

            let right_metric =
                right.0.metrics().get(objective.objective);

            let comparison =
                match (left_metric, right_metric) {
                    (Some(left_value), Some(right_value)) => {
                        if objective.objective.maximize() {
                            right_value.cmp(&left_value)
                        } else {
                            left_value.cmp(&right_value)
                        }
                    }

                    (Some(_), None) => Ordering::Less,

                    (None, Some(_)) => Ordering::Greater,

                    (None, None) => Ordering::Equal,
                };

            if comparison != Ordering::Equal {
                return comparison;
            }
        }

        Ordering::Equal
    }
}

// =============================================================================
// Deterministic convenience functions
// =============================================================================

/// Rank candidates with a supplied policy.
pub fn rank<I>(
    candidates: I,
    policy: RankingPolicy,
) -> Result<RankingResult, RankingError>
where
    I: IntoIterator<Item = RankingCandidate>,
{
    RankingEngine::new(policy)?.rank(candidates)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(
        id: &str,
        feasibility: FeasibilityClass,
        confidence: i128,
        time: i128,
    ) -> RankingCandidate {
        let confidence =
            FixedScore::from_integer(confidence).expect("test score");

        let time =
            FixedScore::from_integer(time).expect("test score");

        RankingCandidate::new(
            CandidateId::new(id).expect("test id"),
            feasibility,
            RankingMetrics {
                confidence: Some(confidence),
                time: Some(time),
                ..RankingMetrics::default()
            },
            1,
            false,
            0,
        )
    }

    fn policy() -> RankingPolicy {
        RankingPolicy::new(
            RankingMode::Weighted,
            FeasibilityPolicy::FeasibleThenUnknown,
            vec![
                ObjectiveWeight::new(
                    RankingObjective::Confidence,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
                ObjectiveWeight::new(
                    RankingObjective::Time,
                    FixedScore::ONE,
                    MissingMetricPolicy::Reject,
                ),
            ],
        )
        .expect("test policy")
    }

    #[test]
    fn deterministic_ordering() {
        let engine =
            RankingEngine::new(policy()).expect("engine");

        let a = candidate(
            "a",
            FeasibilityClass::Feasible,
            9,
            2,
        );

        let b = candidate(
            "b",
            FeasibilityClass::Feasible,
            8,
            2,
        );

        let first = engine
            .rank(vec![a.clone(), b.clone()])
            .expect("ranking");

        let second = engine
            .rank(vec![b, a])
            .expect("ranking");

        assert_eq!(
            first.candidates[0].candidate.id(),
            second.candidates[0].candidate.id()
        );
    }

    #[test]
    fn feasible_candidates_precede_unknown() {
        let engine =
            RankingEngine::new(policy()).expect("engine");

        let unknown = candidate(
            "unknown",
            FeasibilityClass::Unknown,
            100,
            1,
        );

        let feasible = candidate(
            "feasible",
            FeasibilityClass::Feasible,
            1,
            100,
        );

        let result = engine
            .rank(vec![unknown, feasible])
            .expect("ranking");

        assert_eq!(
            result.candidates[0].candidate.id().as_str(),
            "feasible"
        );
    }

    #[test]
    fn lower_time_is_better() {
        let engine =
            RankingEngine::new(policy()).expect("engine");

        let fast = candidate(
            "fast",
            FeasibilityClass::Feasible,
            5,
            1,
        );

        let slow = candidate(
            "slow",
            FeasibilityClass::Feasible,
            5,
            10,
        );

        let result = engine
            .rank(vec![slow, fast])
            .expect("ranking");

        assert_eq!(
            result.candidates[0].candidate.id().as_str(),
            "fast"
        );
    }

    #[test]
    fn protective_candidate_breaks_equal_score() {
        let mut left = candidate(
            "left",
            FeasibilityClass::Feasible,
            5,
            5,
        );

        let right = candidate(
            "right",
            FeasibilityClass::Feasible,
            5,
            5,
        );

        left.protective = true;

        let engine =
            RankingEngine::new(policy()).expect("engine");

        let result = engine
            .rank(vec![right, left])
            .expect("ranking");

        assert_eq!(
            result.candidates[0].candidate.id().as_str(),
            "left"
        );
    }

    #[test]
    fn candidate_limit_is_caller_supplied() {
        let mut policy = policy();

        policy.limits = RankingLimits {
            max_candidates: NonZeroUsize::new(1),
            max_results: None,
        };

        let engine =
            RankingEngine::new(policy).expect("engine");

        let result = engine.rank(vec![
            candidate("a", FeasibilityClass::Feasible, 1, 1),
            candidate("b", FeasibilityClass::Feasible, 1, 1),
        ]);

        assert!(matches!(
            result,
            Err(RankingError::CandidateLimitExceeded { .. })
        ));
    }

    #[test]
    fn result_limit_is_caller_supplied() {
        let mut policy = policy();

        policy.limits = RankingLimits {
            max_candidates: None,
            max_results: NonZeroUsize::new(1),
        };

        let engine =
            RankingEngine::new(policy).expect("engine");

        let result = engine
            .rank(vec![
                candidate("a", FeasibilityClass::Feasible, 1, 1),
                candidate("b", FeasibilityClass::Feasible, 2, 2),
            ])
            .expect("ranking");

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn missing_required_metric_is_rejected() {
        let engine =
            RankingEngine::new(policy()).expect("engine");

        let candidate = RankingCandidate::new(
            CandidateId::new("missing").expect("id"),
            FeasibilityClass::Feasible,
            RankingMetrics::default(),
            1,
            false,
            0,
        );

        let result = engine.rank(vec![candidate]);

        assert!(matches!(
            result,
            Err(RankingError::NoRankableCandidates)
        ));
    }

    #[test]
    fn candidate_identity_provides_total_order() {
        let engine =
            RankingEngine::new(policy()).expect("engine");

        let a = candidate(
            "a",
            FeasibilityClass::Feasible,
            5,
            5,
        );

        let b = candidate(
            "b",
            FeasibilityClass::Feasible,
            5,
            5,
        );

        let result = engine
            .rank(vec![b, a])
            .expect("ranking");

        assert_eq!(
            result.candidates[0].candidate.id().as_str(),
            "a"
        );
        assert_eq!(
            result.candidates[1].candidate.id().as_str(),
            "b"
        );
    }

    #[test]
    fn no_floating_point_is_required_for_ordering() {
        let a = FixedScore::from_scaled_units(1_000_001);
        let b = FixedScore::from_scaled_units(1_000_002);

        assert!(b > a);
    }

    #[test]
    fn pareto_frontier_is_preferred() {
        let mut policy = policy();
        policy.mode = RankingMode::Pareto;

        let engine =
            RankingEngine::new(policy).expect("engine");

        let balanced = candidate(
            "balanced",
            FeasibilityClass::Feasible,
            8,
            8,
        );

        let dominated = candidate(
            "dominated",
            FeasibilityClass::Feasible,
            7,
            9,
        );

        let result = engine
            .rank(vec![dominated, balanced])
            .expect("ranking");

        assert_eq!(
            result.candidates[0].candidate.id().as_str(),
            "balanced"
        );
    }
}