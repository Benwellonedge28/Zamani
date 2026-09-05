//! Zamani Quantum Scheduling — Multi-Objective Optimization
//!
//! Path:
//!     src/quantum/scheduling/optimization/multi_objective.rs
//!
//! # Purpose
//!
//! Provides the production multi-objective optimization contract for quantum
//! schedules.
//!
//! The module converts a collection of independently measured schedule
//! objectives into:
//!
//! - validated objective specifications;
//! - normalized objective values;
//! - weighted scalar scores;
//! - lexicographic comparisons;
//! - Pareto dominance comparisons;
//! - Pareto-frontier maintenance;
//! - deterministic candidate selection;
//! - objective-by-objective diagnostics.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! routing
//!     │
//!     ▼
//! scheduling
//!     │
//!     ├── planning
//!     ├── timing
//!     ├── resources
//!     ├── verification
//!     │
//!     ▼
//! scheduling::result / analysis artifacts
//!     │
//!     ▼
//! scheduling::optimization::multi_objective   ◄── this module
//!     │
//!     ├── scalarization
//!     ├── lexicographic comparison
//!     ├── Pareto dominance
//!     └── Pareto frontier
//! ```
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - objective component definitions;
//! - objective direction;
//! - explicit caller-supplied weights;
//! - optional caller-supplied normalization bounds;
//! - finite-value validation;
//! - weighted scalarization;
//! - lexicographic ordering;
//! - Pareto dominance;
//! - Pareto frontier maintenance;
//! - deterministic candidate comparison;
//! - objective evaluation diagnostics;
//! - optimization configuration validation;
//! - objective aggregation without machine-specific assumptions.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware discovery;
//! - calibration;
//! - scheduling itself;
//! - QEC;
//! - noise modelling;
//! - runtime execution;
//! - serialization formats;
//! - compiler frontend syntax.
//!
//! # Canonical identity rule
//!
//! This file intentionally does not define:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - `ResourceId`.
//!
//! If a future objective needs quantum identity, it MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The multi-objective optimizer itself normally does not need qubit identity.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once. Objective evaluation must therefore be
//! independent of:
//!
//! - qubit count;
//! - operation count;
//! - topology size;
//! - hardware vendor;
//! - technology;
//! - number of channels;
//! - number of QPUs;
//! - schedule depth;
//! - fixed machine dimensions.
//!
//! There are no machine-size constants in this module.
//!
//! "Infinity" means that the implementation imposes no artificial finite
//! quantum-machine limit. A concrete optimization remains bounded by the
//! actual candidate set, process memory, execution resources, and caller
//! policy.
//!
//! # Correctness principle
//!
//! Objective optimization is always subordinate to hard scheduling validity.
//!
//! ```text
//! validity constraints
//!        │
//!        ▼
//! feasible candidates
//!        │
//!        ▼
//! objective optimization
//! ```
//!
//! An objective is NEVER allowed to trade away:
//!
//! - dependency correctness;
//! - resource correctness;
//! - timing correctness;
//! - target compatibility;
//! - semantic preservation;
//! - verification requirements.
//!
//! # Objective directions
//!
//! Every objective explicitly declares whether lower or higher values are
//! preferred.
//!
//! This avoids incorrectly assuming that every metric is a minimization
//! problem.
//!
//! Examples:
//!
//! ```text
//! makespan  → minimize
//! depth     → minimize
//! idle time → minimize
//! energy    → minimize
//! fidelity  → maximize
//! ```
//!
//! No implicit interpretation is used.
//!
//! # Weighted scalarization
//!
//! For normalized objective values x_i and normalized weights w_i:
//!
//! ```text
//! score = Σ w_i * x_i
//! ```
//!
//! Before scalarization, maximization objectives are transformed into a
//! minimization-compatible form:
//!
//! ```text
//! maximize x  →  1 - x
//! minimize x  →  x
//! ```
//!
//! This transformation is only applied after explicit normalization.
//!
//! The module does not assume that physical metrics naturally lie in [0, 1].
//! Caller-provided bounds or explicit normalization are required when raw
//! metrics do not already have a common scale.
//!
//! # Pareto dominance
//!
//! Candidate A dominates candidate B when:
//!
//! - A is no worse than B for every objective;
//! - A is strictly better than B for at least one objective.
//!
//! No objective weight is required for Pareto dominance.
//!
//! # Pareto frontier scalability
//!
//! The frontier is maintained incrementally.
//!
//! A candidate is rejected immediately if an existing frontier point dominates
//! it. Otherwise dominated frontier points are removed.
//!
//! The algorithm is exact. The frontier itself may legitimately become large;
//! the module does not impose an artificial frontier-size limit.
//!
//! # Determinism
//!
//! Deterministic mode uses:
//!
//! 1. objective declaration order;
//! 2. candidate insertion order where semantic order is supplied;
//! 3. explicit candidate identifiers where available;
//! 4. stable floating-point comparisons;
//! 5. no hash-map iteration for semantic ordering.
//!
//! Candidate identifiers are optional because this module does not own
//! scheduler identity.
//!
//! # Floating-point safety
//!
//! Objective values are represented by a validated finite `f64` wrapper.
//!
//! NaN and infinities are rejected at the boundary.
//!
//! Arithmetic is checked for:
//!
//! - non-finite results;
//! - invalid normalization ranges;
//! - overflow into infinity;
//! - invalid weights.
//!
//! No unsafe Rust is used.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! The intended flow is:
//!
//! ```text
//! scheduling::result
//!        │
//!        ├── makespan
//!        ├── depth
//!        ├── idle time
//!        ├── resource utilization
//!        └── target/QZN-derived metrics
//!        │
//!        ▼
//! ObjectiveVector
//!        │
//!        ▼
//! MultiObjectiveConfig
//!        │
//!        ▼
//! MultiObjectiveOptimizer
//!        │
//!        ├── weighted score
//!        ├── lexicographic comparison
//!        └── Pareto frontier
//!        │
//!        ▼
//! scheduling::result
//! ```
//!
//! The optimizer can therefore be used by:
//!
//! - `scheduling::planners`;
//! - `scheduling::algorithms`;
//! - `scheduling::optimization::makespan`;
//! - `scheduling::optimization::idle_time`;
//! - `scheduling::optimization::fidelity`;
//! - `scheduling::optimization::energy`;
//! - `scheduling::optimization::depth`;
//! - `scheduling::policies`;
//! - `scheduling::result`;
//! - `scheduling::diagnostics`;
//! - scheduling benchmarking;
//! - hardware/ZQN adapters.
//!
//! None of those modules are required by this foundational implementation.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by multi-objective optimization.
#[derive(Debug, Clone, PartialEq)]
pub enum MultiObjectiveError {
    /// No objective was configured.
    EmptyObjectiveSet,

    /// The same objective appears more than once.
    DuplicateObjective {
        /// Name of the duplicated objective.
        objective: String,
    },

    /// A weight is invalid.
    InvalidWeight {
        /// Objective associated with the weight.
        objective: String,
        /// Invalid weight.
        weight: f64,
    },

    /// All supplied weights are zero.
    ZeroTotalWeight,

    /// An objective value is not finite.
    NonFiniteValue {
        /// Objective name.
        objective: String,
        /// Invalid value.
        value: f64,
    },

    /// A normalization range is invalid.
    InvalidNormalizationRange {
        /// Objective name.
        objective: String,
        /// Lower bound.
        minimum: f64,
        /// Upper bound.
        maximum: f64,
    },

    /// A normalized value is not finite.
    NormalizationFailure {
        /// Objective name.
        objective: String,
        /// Raw value.
        value: f64,
    },

    /// Scalarization produced a non-finite value.
    ScalarizationOverflow,

    /// Objective vector does not match the configured objective schema.
    SchemaMismatch {
        /// Expected objective count.
        expected: usize,
        /// Actual objective count.
        actual: usize,
    },

    /// A candidate cannot be evaluated.
    CandidateEvaluation {
        /// Candidate identifier or description.
        candidate: String,
        /// Reason.
        reason: String,
    },

    /// No feasible candidate was supplied.
    NoCandidates,

    /// A required candidate is missing.
    CandidateNotFound {
        /// Candidate identifier.
        candidate: String,
    },
}

impl fmt::Display for MultiObjectiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObjectiveSet => {
                formatter.write_str("multi-objective configuration contains no objectives")
            }

            Self::DuplicateObjective { objective } => {
                write!(formatter, "objective `{objective}` is configured more than once")
            }

            Self::InvalidWeight { objective, weight } => {
                write!(
                    formatter,
                    "objective `{objective}` has invalid weight `{weight}`"
                )
            }

            Self::ZeroTotalWeight => {
                formatter.write_str("multi-objective configuration has zero total weight")
            }

            Self::NonFiniteValue { objective, value } => {
                write!(
                    formatter,
                    "objective `{objective}` has non-finite value `{value}`"
                )
            }

            Self::InvalidNormalizationRange {
                objective,
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "objective `{objective}` has invalid normalization range [{minimum}, {maximum}]"
                )
            }

            Self::NormalizationFailure { objective, value } => {
                write!(
                    formatter,
                    "objective `{objective}` could not be normalized from value `{value}`"
                )
            }

            Self::ScalarizationOverflow => {
                formatter.write_str("multi-objective scalarization produced a non-finite value")
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    formatter,
                    "objective schema mismatch: expected {expected} values, received {actual}"
                )
            }

            Self::CandidateEvaluation { candidate, reason } => {
                write!(
                    formatter,
                    "candidate `{candidate}` could not be evaluated: {reason}"
                )
            }

            Self::NoCandidates => {
                formatter.write_str("no candidate schedules were supplied")
            }

            Self::CandidateNotFound { candidate } => {
                write!(formatter, "candidate `{candidate}` was not found")
            }
        }
    }
}

impl std::error::Error for MultiObjectiveError {}

/// Result alias for this module.
pub type MultiObjectiveResult<T> = Result<T, MultiObjectiveError>;

// =============================================================================
// Finite value
// =============================================================================

/// A finite IEEE-754 floating-point value.
///
/// This wrapper prevents NaN and infinity from crossing the objective
/// evaluation boundary.
#[derive(Clone, Copy, Debug)]
pub struct FiniteValue(f64);

impl FiniteValue {
    /// Creates a finite value.
    pub fn new(value: f64) -> MultiObjectiveResult<Self> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(MultiObjectiveError::NonFiniteValue {
                objective: "unnamed".to_owned(),
                value,
            })
        }
    }

    /// Creates a finite value while associating an objective name with errors.
    pub fn named(name: &str, value: f64) -> MultiObjectiveResult<Self> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(MultiObjectiveError::NonFiniteValue {
                objective: name.to_owned(),
                value,
            })
        }
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl PartialEq for FiniteValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for FiniteValue {}

impl PartialOrd for FiniteValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiniteValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

// =============================================================================
// Objective direction
// =============================================================================

/// Direction in which an objective is optimized.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ObjectiveDirection {
    /// Smaller values are better.
    Minimize,

    /// Larger values are better.
    Maximize,
}

impl ObjectiveDirection {
    /// Returns whether the first value is better than the second.
    #[must_use]
    pub fn better(self, left: f64, right: f64) -> bool {
        match self {
            Self::Minimize => left < right,
            Self::Maximize => left > right,
        }
    }

    /// Returns whether the first value is no worse than the second.
    #[must_use]
    pub fn no_worse(self, left: f64, right: f64) -> bool {
        match self {
            Self::Minimize => left <= right,
            Self::Maximize => left >= right,
        }
    }

    /// Converts a raw value into minimization orientation.
    ///
    /// This is only appropriate for normalized values in [0, 1].
    pub fn to_minimization(self, normalized: f64) -> MultiObjectiveResult<f64> {
        if !normalized.is_finite() {
            return Err(MultiObjectiveError::ScalarizationOverflow);
        }

        let value = match self {
            Self::Minimize => normalized,
            Self::Maximize => 1.0 - normalized,
        };

        if value.is_finite() {
            Ok(value)
        } else {
            Err(MultiObjectiveError::ScalarizationOverflow)
        }
    }
}

// =============================================================================
// Standard scheduling objective identifiers
// =============================================================================

/// Standard scheduling objective kinds.
///
/// Custom objectives are supported through [`ObjectiveKey::Custom`].
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ObjectiveKey {
    /// Feasibility-oriented objective.
    Feasible,

    /// Total schedule duration.
    Makespan,

    /// Temporal schedule depth.
    Depth,

    /// Total or normalized resource idle time.
    IdleTime,

    /// Estimated physical fidelity.
    Fidelity,

    /// Estimated execution energy/resource cost.
    Energy,

    /// Target/resource utilization.
    ResourceUtilization,

    /// Communication latency/cost.
    Communication,

    /// Estimated error probability or error cost.
    Error,

    /// Caller-defined objective.
    Custom(String),
}

impl ObjectiveKey {
    /// Returns a stable human-readable objective name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Feasible => "feasible",
            Self::Makespan => "makespan",
            Self::Depth => "depth",
            Self::IdleTime => "idle-time",
            Self::Fidelity => "fidelity",
            Self::Energy => "energy",
            Self::ResourceUtilization => "resource-utilization",
            Self::Communication => "communication",
            Self::Error => "error",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns the conventional optimization direction.
    ///
    /// Custom objectives default to minimization. Callers should explicitly
    /// specify the direction for custom metrics rather than relying on this
    /// convenience method.
    #[must_use]
    pub fn conventional_direction(&self) -> ObjectiveDirection {
        match self {
            Self::Feasible => ObjectiveDirection::Maximize,
            Self::Makespan => ObjectiveDirection::Minimize,
            Self::Depth => ObjectiveDirection::Minimize,
            Self::IdleTime => ObjectiveDirection::Minimize,
            Self::Fidelity => ObjectiveDirection::Maximize,
            Self::Energy => ObjectiveDirection::Minimize,
            Self::ResourceUtilization => ObjectiveDirection::Maximize,
            Self::Communication => ObjectiveDirection::Minimize,
            Self::Error => ObjectiveDirection::Minimize,
            Self::Custom(_) => ObjectiveDirection::Minimize,
        }
    }
}

impl fmt::Display for ObjectiveKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Normalization
// =============================================================================

/// Normalization specification for one objective.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Normalization {
    /// Do not normalize the objective.
    ///
    /// This should only be used when all objectives already have compatible
    /// scales.
    None,

    /// Explicit minimum and maximum supplied by the caller.
    Range {
        /// Lower bound.
        minimum: f64,

        /// Upper bound.
        maximum: f64,
    },

    /// Normalize against the candidate population.
    ///
    /// The optimizer determines the range from the candidates supplied to the
    /// current optimization invocation.
    Population,

    /// Normalize against a caller-provided reference value.
    ///
    /// The reference is interpreted as a scale, not as a machine constant.
    Reference {
        /// Positive finite scale.
        scale: f64,
    },
}

impl Normalization {
    fn validate(self, objective: &str) -> MultiObjectiveResult<()> {
        match self {
            Self::None | Self::Population => Ok(()),

            Self::Range {
                minimum,
                maximum,
            } => {
                if !minimum.is_finite()
                    || !maximum.is_finite()
                    || maximum <= minimum
                {
                    return Err(MultiObjectiveError::InvalidNormalizationRange {
                        objective: objective.to_owned(),
                        minimum,
                        maximum,
                    });
                }

                Ok(())
            }

            Self::Reference { scale } => {
                if !scale.is_finite() || scale <= 0.0 {
                    return Err(MultiObjectiveError::InvalidNormalizationRange {
                        objective: objective.to_owned(),
                        minimum: 0.0,
                        maximum: scale,
                    });
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Objective specification
// =============================================================================

/// Complete specification of one objective.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectiveSpec {
    key: ObjectiveKey,
    direction: ObjectiveDirection,
    weight: FiniteValue,
    normalization: Normalization,
}

impl ObjectiveSpec {
    /// Creates an objective specification.
    pub fn new(
        key: ObjectiveKey,
        direction: ObjectiveDirection,
        weight: f64,
        normalization: Normalization,
    ) -> MultiObjectiveResult<Self> {
        let name = key.name();

        if !weight.is_finite() || weight < 0.0 {
            return Err(MultiObjectiveError::InvalidWeight {
                objective: name.to_owned(),
                weight,
            });
        }

        normalization.validate(name)?;

        Ok(Self {
            key,
            direction,
            weight: FiniteValue(weight),
            normalization,
        })
    }

    /// Creates an objective using the conventional direction.
    pub fn conventional(
        key: ObjectiveKey,
        weight: f64,
        normalization: Normalization,
    ) -> MultiObjectiveResult<Self> {
        Self::new(
            key.clone(),
            key.conventional_direction(),
            weight,
            normalization,
        )
    }

    /// Returns the objective key.
    #[must_use]
    pub fn key(&self) -> &ObjectiveKey {
        &self.key
    }

    /// Returns the objective name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.key.name()
    }

    /// Returns the direction.
    #[must_use]
    pub const fn direction(&self) -> ObjectiveDirection {
        self.direction
    }

    /// Returns the weight.
    #[must_use]
    pub const fn weight(&self) -> f64 {
        self.weight.get()
    }

    /// Returns the normalization policy.
    #[must_use]
    pub const fn normalization(&self) -> Normalization {
        self.normalization
    }
}

// =============================================================================
// Objective value
// =============================================================================

/// One evaluated objective value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ObjectiveValue {
    raw: FiniteValue,
}

impl ObjectiveValue {
    /// Creates a validated objective value.
    pub fn new(objective: &ObjectiveSpec, value: f64) -> MultiObjectiveResult<Self> {
        Ok(Self {
            raw: FiniteValue::named(objective.name(), value)?,
        })
    }

    /// Returns the raw objective value.
    #[must_use]
    pub const fn raw(self) -> f64 {
        self.raw.get()
    }
}

// =============================================================================
// Objective vector
// =============================================================================

/// Ordered objective values for one candidate schedule.
///
/// The order MUST match the objective schema supplied to the optimizer.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectiveVector {
    values: Vec<ObjectiveValue>,
}

impl ObjectiveVector {
    /// Creates an objective vector.
    pub fn new(values: Vec<ObjectiveValue>) -> Self {
        Self { values }
    }

    /// Creates a vector directly from raw values.
    pub fn from_raw(
        objectives: &[ObjectiveSpec],
        values: &[f64],
    ) -> MultiObjectiveResult<Self> {
        if objectives.len() != values.len() {
            return Err(MultiObjectiveError::SchemaMismatch {
                expected: objectives.len(),
                actual: values.len(),
            });
        }

        let mut result = Vec::with_capacity(values.len());

        for (objective, value) in objectives.iter().zip(values.iter().copied()) {
            result.push(ObjectiveValue::new(objective, value)?);
        }

        Ok(Self::new(result))
    }

    /// Returns the number of objective values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether this vector contains no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns one objective value.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<ObjectiveValue> {
        self.values.get(index).copied()
    }

    /// Returns all objective values.
    #[must_use]
    pub fn values(&self) -> &[ObjectiveValue] {
        &self.values
    }
}

// =============================================================================
// Candidate
// =============================================================================

/// A candidate schedule together with its evaluated objectives.
///
/// `T` is deliberately unconstrained. The optimizer does not know whether the
/// candidate is a schedule, an intermediate plan, a hardware mapping, or
/// another immutable optimization artifact.
#[derive(Clone, Debug, PartialEq)]
pub struct Candidate<T> {
    /// Caller-owned candidate payload.
    pub payload: T,

    /// Optional deterministic identifier.
    pub id: Option<u128>,

    /// Objective vector.
    pub objectives: ObjectiveVector,
}

impl<T> Candidate<T> {
    /// Creates a candidate.
    #[must_use]
    pub fn new(
        payload: T,
        id: Option<u128>,
        objectives: ObjectiveVector,
    ) -> Self {
        Self {
            payload,
            id,
            objectives,
        }
    }
}

// =============================================================================
// Optimization method
// =============================================================================

/// Multi-objective optimization method.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum OptimizationMethod {
    /// Weighted scalarization.
    WeightedSum,

    /// Lexicographic ordering.
    Lexicographic,

    /// Pareto dominance/frontier extraction.
    Pareto,

    /// First use Pareto dominance, then weighted scoring among nondominated
    /// candidates.
    ParetoThenWeighted,

    /// First use weighted scoring, retaining ties for Pareto analysis.
    WeightedThenPareto,
}

impl Default for OptimizationMethod {
    fn default() -> Self {
        Self::ParetoThenWeighted
    }
}

// =============================================================================
// Tie policy
// =============================================================================

/// Deterministic tie-breaking policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TiePolicy {
    /// Preserve the first candidate.
    First,

    /// Preserve the last candidate.
    Last,

    /// Select the candidate with the smallest explicit identifier.
    LowestId,

    /// Select the candidate with the largest explicit identifier.
    HighestId,
}

impl Default for TiePolicy {
    fn default() -> Self {
        Self::First
    }
}

// =============================================================================
// Multi-objective configuration
// =============================================================================

/// Complete multi-objective optimization configuration.
///
/// The configuration is immutable after construction.
#[derive(Clone, Debug, PartialEq)]
pub struct MultiObjectiveConfig {
    objectives: Vec<ObjectiveSpec>,
    method: OptimizationMethod,
    tie_policy: TiePolicy,
    reject_non_finite: bool,
}

impl MultiObjectiveConfig {
    /// Creates a configuration from an explicit objective list.
    pub fn new(
        objectives: Vec<ObjectiveSpec>,
    ) -> MultiObjectiveResult<Self> {
        if objectives.is_empty() {
            return Err(MultiObjectiveError::EmptyObjectiveSet);
        }

        let mut names = std::collections::BTreeSet::new();
        let mut positive_weight = false;

        for objective in &objectives {
            let name = objective.name().to_owned();

            if !names.insert(name.clone()) {
                return Err(MultiObjectiveError::DuplicateObjective {
                    objective: name,
                });
            }

            if objective.weight() > 0.0 {
                positive_weight = true;
            }
        }

        if !positive_weight {
            return Err(MultiObjectiveError::ZeroTotalWeight);
        }

        Ok(Self {
            objectives,
            method: OptimizationMethod::default(),
            tie_policy: TiePolicy::default(),
            reject_non_finite: true,
        })
    }

    /// Sets the optimization method.
    #[must_use]
    pub fn with_method(mut self, method: OptimizationMethod) -> Self {
        self.method = method;
        self
    }

    /// Sets deterministic tie handling.
    #[must_use]
    pub fn with_tie_policy(mut self, policy: TiePolicy) -> Self {
        self.tie_policy = policy;
        self
    }

    /// Sets non-finite value behavior.
    ///
    /// Production scheduling should normally retain the default `true`.
    #[must_use]
    pub fn reject_non_finite(mut self, reject: bool) -> Self {
        self.reject_non_finite = reject;
        self
    }

    /// Returns configured objectives.
    #[must_use]
    pub fn objectives(&self) -> &[ObjectiveSpec] {
        &self.objectives
    }

    /// Returns the selected optimization method.
    #[must_use]
    pub const fn method(&self) -> OptimizationMethod {
        self.method
    }

    /// Returns the tie policy.
    #[must_use]
    pub const fn tie_policy(&self) -> TiePolicy {
        self.tie_policy
    }

    /// Returns whether non-finite values are rejected.
    #[must_use]
    pub const fn reject_non_finite(&self) -> bool {
        self.reject_non_finite
    }
}

// =============================================================================
// Evaluation
// =============================================================================

/// Normalized objective evaluation for one candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectiveEvaluation {
    raw: ObjectiveVector,
    normalized: Vec<FiniteValue>,
    minimization: Vec<FiniteValue>,
    weighted_score: FiniteValue,
}

impl ObjectiveEvaluation {
    /// Returns raw objective values.
    #[must_use]
    pub fn raw(&self) -> &ObjectiveVector {
        &self.raw
    }

    /// Returns normalized values.
    #[must_use]
    pub fn normalized(&self) -> &[FiniteValue] {
        &self.normalized
    }

    /// Returns minimization-oriented normalized values.
    #[must_use]
    pub fn minimization_values(&self) -> &[FiniteValue] {
        &self.minimization
    }

    /// Returns the weighted scalar score.
    #[must_use]
    pub const fn weighted_score(&self) -> f64 {
        self.weighted_score.get()
    }
}

// =============================================================================
// Comparison
// =============================================================================

/// Relationship between two objective vectors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Dominance {
    /// Left candidate dominates right candidate.
    LeftDominates,

    /// Right candidate dominates left candidate.
    RightDominates,

    /// Both candidates have identical objective values.
    Equal,

    /// Neither candidate dominates the other.
    NonDominated,
}

// =============================================================================
// Result
// =============================================================================

/// Result of optimizing a candidate population.
#[derive(Clone, Debug, PartialEq)]
pub struct MultiObjectiveResultArtifact<T> {
    selected: Option<Candidate<T>>,
    selected_evaluation: Option<ObjectiveEvaluation>,
    pareto_frontier: Vec<Candidate<T>>,
    frontier_evaluations: Vec<ObjectiveEvaluation>,
    candidate_count: usize,
}

impl<T> MultiObjectiveResultArtifact<T> {
    /// Returns the selected candidate.
    #[must_use]
    pub fn selected(&self) -> Option<&Candidate<T>> {
        self.selected.as_ref()
    }

    /// Returns the selected evaluation.
    #[must_use]
    pub fn selected_evaluation(&self) -> Option<&ObjectiveEvaluation> {
        self.selected_evaluation.as_ref()
    }

    /// Returns the Pareto frontier.
    #[must_use]
    pub fn pareto_frontier(&self) -> &[Candidate<T>] {
        &self.pareto_frontier
    }

    /// Returns evaluations corresponding to the Pareto frontier.
    #[must_use]
    pub fn frontier_evaluations(&self) -> &[ObjectiveEvaluation] {
        &self.frontier_evaluations
    }

    /// Returns the number of evaluated candidates.
    #[must_use]
    pub const fn candidate_count(&self) -> usize {
        self.candidate_count
    }
}

// =============================================================================
// Optimizer
// =============================================================================

/// Production multi-objective optimizer.
///
/// The optimizer is stateless between calls and therefore can safely be used
/// concurrently by independent scheduling invocations.
#[derive(Clone, Debug)]
pub struct MultiObjectiveOptimizer {
    config: MultiObjectiveConfig,
}

impl MultiObjectiveOptimizer {
    /// Creates an optimizer from validated configuration.
    #[must_use]
    pub fn new(config: MultiObjectiveConfig) -> Self {
        Self { config }
    }

    /// Returns the optimizer configuration.
    #[must_use]
    pub fn config(&self) -> &MultiObjectiveConfig {
        &self.config
    }

    /// Evaluates one objective vector.
    pub fn evaluate(
        &self,
        vector: ObjectiveVector,
    ) -> MultiObjectiveResult<ObjectiveEvaluation> {
        self.validate_vector(&vector)?;

        let population_ranges = None;
        self.evaluate_with_ranges(vector, population_ranges)
    }

    /// Evaluates a population and selects the best candidate according to the
    /// configured method.
    pub fn optimize<T: Clone>(
        &self,
        candidates: &[Candidate<T>],
    ) -> MultiObjectiveResult<MultiObjectiveResultArtifact<T>> {
        if candidates.is_empty() {
            return Err(MultiObjectiveError::NoCandidates);
        }

        for candidate in candidates {
            self.validate_vector(&candidate.objectives)?;
        }

        let ranges = self.population_ranges(candidates)?;

        let mut evaluations = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            evaluations.push(self.evaluate_with_ranges(
                candidate.objectives.clone(),
                Some(&ranges),
            )?);
        }

        let frontier_indices =
            self.pareto_frontier_indices(candidates, &evaluations);

        let selected_index = match self.config.method() {
            OptimizationMethod::WeightedSum => {
                self.select_weighted(candidates, &evaluations)?
            }

            OptimizationMethod::Lexicographic => {
                self.select_lexicographic(candidates)?
            }

            OptimizationMethod::Pareto => {
                self.select_from_frontier(
                    candidates,
                    &evaluations,
                    &frontier_indices,
                )?
            }

            OptimizationMethod::ParetoThenWeighted => {
                self.select_from_frontier(
                    candidates,
                    &evaluations,
                    &frontier_indices,
                )?
            }

            OptimizationMethod::WeightedThenPareto => {
                let weighted = self.select_weighted(candidates, &evaluations)?;

                let best_score =
                    evaluations[weighted].weighted_score();

                let tied: Vec<usize> = evaluations
                    .iter()
                    .enumerate()
                    .filter_map(|(index, evaluation)| {
                        if evaluation.weighted_score() == best_score {
                            Some(index)
                        } else {
                            None
                        }
                    })
                    .collect();

                if tied.len() == 1 {
                    weighted
                } else {
                    self.select_from_indices(
                        candidates,
                        &evaluations,
                        &tied,
                    )?
                }
            }
        };

        let mut frontier = Vec::with_capacity(frontier_indices.len());
        let mut frontier_evaluations =
            Vec::with_capacity(frontier_indices.len());

        for index in frontier_indices {
            frontier.push(candidates[index].clone());
            frontier_evaluations.push(evaluations[index].clone());
        }

        Ok(MultiObjectiveResultArtifact {
            selected: Some(candidates[selected_index].clone()),
            selected_evaluation: Some(evaluations[selected_index].clone()),
            pareto_frontier: frontier,
            frontier_evaluations,
            candidate_count: candidates.len(),
        })
    }

    /// Returns the Pareto frontier from an already evaluated candidate set.
    pub fn pareto_frontier<T>(
        &self,
        candidates: &[Candidate<T>],
    ) -> MultiObjectiveResult<Vec<usize>> {
        if candidates.is_empty() {
            return Ok(Vec::new());
        }

        for candidate in candidates {
            self.validate_vector(&candidate.objectives)?;
        }

        let ranges = self.population_ranges(candidates)?;

        let mut evaluations = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            evaluations.push(self.evaluate_with_ranges(
                candidate.objectives.clone(),
                Some(&ranges),
            )?);
        }

        Ok(self.pareto_frontier_indices(candidates, &evaluations))
    }

    /// Compares two already evaluated objective vectors.
    pub fn dominance(
        &self,
        left: &ObjectiveVector,
        right: &ObjectiveVector,
    ) -> MultiObjectiveResult<Dominance> {
        self.validate_vector(left)?;
        self.validate_vector(right)?;

        let left_evaluation = self.evaluate(left.clone())?;
        let right_evaluation = self.evaluate(right.clone())?;

        Ok(Self::dominance_from_evaluations(
            &left_evaluation,
            &right_evaluation,
        ))
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    fn validate_vector(
        &self,
        vector: &ObjectiveVector,
    ) -> MultiObjectiveResult<()> {
        if vector.len() != self.config.objectives().len() {
            return Err(MultiObjectiveError::SchemaMismatch {
                expected: self.config.objectives().len(),
                actual: vector.len(),
            });
        }

        if self.config.reject_non_finite() {
            for (objective, value) in self
                .config
                .objectives()
                .iter()
                .zip(vector.values())
            {
                if !value.raw().is_finite() {
                    return Err(MultiObjectiveError::NonFiniteValue {
                        objective: objective.name().to_owned(),
                        value: value.raw(),
                    });
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Population normalization
    // -------------------------------------------------------------------------

    fn population_ranges<T>(
        &self,
        candidates: &[Candidate<T>],
    ) -> MultiObjectiveResult<Vec<(f64, f64)>> {
        let count = self.config.objectives().len();

        let mut ranges = Vec::with_capacity(count);

        for objective_index in 0..count {
            let objective =
                &self.config.objectives()[objective_index];

            match objective.normalization() {
                Normalization::Range {
                    minimum,
                    maximum,
                } => {
                    ranges.push((minimum, maximum));
                }

                Normalization::Population => {
                    let first = candidates[0]
                        .objectives
                        .get(objective_index)
                        .ok_or_else(|| {
                            MultiObjectiveError::SchemaMismatch {
                                expected: count,
                                actual: candidates[0].objectives.len(),
                            }
                        })?
                        .raw();

                    let mut minimum = first;
                    let mut maximum = first;

                    for candidate in candidates.iter().skip(1) {
                        let value = candidate
                            .objectives
                            .get(objective_index)
                            .ok_or_else(|| {
                                MultiObjectiveError::SchemaMismatch {
                                    expected: count,
                                    actual: candidate.objectives.len(),
                                }
                            })?
                            .raw();

                        if !value.is_finite() {
                            return Err(MultiObjectiveError::NonFiniteValue {
                                objective: objective.name().to_owned(),
                                value,
                            });
                        }

                        if value < minimum {
                            minimum = value;
                        }

                        if value > maximum {
                            maximum = value;
                        }
                    }

                    if maximum == minimum {
                        // A constant objective contains no information for
                        // ranking. Treat all candidates as equally optimal.
                        ranges.push((minimum, maximum));
                    } else {
                        ranges.push((minimum, maximum));
                    }
                }

                Normalization::Reference { scale } => {
                    ranges.push((0.0, scale));
                }

                Normalization::None => {
                    ranges.push((0.0, 0.0));
                }
            }
        }

        Ok(ranges)
    }

    fn evaluate_with_ranges(
        &self,
        vector: ObjectiveVector,
        ranges: Option<&[(f64, f64)]>,
    ) -> MultiObjectiveResult<ObjectiveEvaluation> {
        self.validate_vector(&vector)?;

        let mut normalized =
            Vec::with_capacity(vector.len());

        let mut minimization =
            Vec::with_capacity(vector.len());

        for (index, (objective, value)) in self
            .config
            .objectives()
            .iter()
            .zip(vector.values())
            .enumerate()
        {
            let raw = value.raw();

            let normalized_value = match objective.normalization() {
                Normalization::None => raw,

                Normalization::Range {
                    minimum,
                    maximum,
                } => {
                    Self::normalize_range(
                        objective,
                        raw,
                        minimum,
                        maximum,
                    )?
                }

                Normalization::Population => {
                    let (minimum, maximum) = ranges
                        .and_then(|items| items.get(index).copied())
                        .ok_or_else(|| {
                            MultiObjectiveError::NormalizationFailure {
                                objective: objective.name().to_owned(),
                                value: raw,
                            }
                        })?;

                    if minimum == maximum {
                        0.0
                    } else {
                        Self::normalize_range(
                            objective,
                            raw,
                            minimum,
                            maximum,
                        )?
                    }
                }

                Normalization::Reference { scale } => {
                    let value = raw / scale;

                    if value.is_finite() {
                        value
                    } else {
                        return Err(
                            MultiObjectiveError::NormalizationFailure {
                                objective: objective.name().to_owned(),
                                value: raw,
                            },
                        );
                    }
                }
            };

            if !normalized_value.is_finite() {
                return Err(
                    MultiObjectiveError::NormalizationFailure {
                        objective: objective.name().to_owned(),
                        value: raw,
                    },
                );
            }

            let minimization_value =
                match objective.normalization() {
                    Normalization::None => raw,
                    _ => objective
                        .direction()
                        .to_minimization(normalized_value)?,
                };

            normalized.push(FiniteValue(normalized_value));
            minimization.push(FiniteValue(minimization_value));
        }

        let score =
            self.weighted_score_from_minimization(&minimization)?;

        Ok(ObjectiveEvaluation {
            raw: vector,
            normalized,
            minimization,
            weighted_score: FiniteValue(score),
        })
    }

    fn normalize_range(
        objective: &ObjectiveSpec,
        value: f64,
        minimum: f64,
        maximum: f64,
    ) -> MultiObjectiveResult<f64> {
        if maximum <= minimum
            || !minimum.is_finite()
            || !maximum.is_finite()
        {
            return Err(MultiObjectiveError::InvalidNormalizationRange {
                objective: objective.name().to_owned(),
                minimum,
                maximum,
            });
        }

        let normalized =
            (value - minimum) / (maximum - minimum);

        if normalized.is_finite() {
            Ok(normalized)
        } else {
            Err(MultiObjectiveError::NormalizationFailure {
                objective: objective.name().to_owned(),
                value,
            })
        }
    }

    // -------------------------------------------------------------------------
    // Scalarization
    // -------------------------------------------------------------------------

    fn weighted_score_from_minimization(
        &self,
        values: &[FiniteValue],
    ) -> MultiObjectiveResult<f64> {
        if values.len() != self.config.objectives().len() {
            return Err(MultiObjectiveError::SchemaMismatch {
                expected: self.config.objectives().len(),
                actual: values.len(),
            });
        }

        let mut numerator = 0.0_f64;
        let mut denominator = 0.0_f64;

        for (objective, value) in self
            .config
            .objectives()
            .iter()
            .zip(values.iter())
        {
            let weight = objective.weight();

            if weight == 0.0 {
                continue;
            }

            numerator += weight * value.get();
            denominator += weight;

            if !numerator.is_finite() || !denominator.is_finite() {
                return Err(MultiObjectiveError::ScalarizationOverflow);
            }
        }

        if denominator <= 0.0 || !denominator.is_finite() {
            return Err(MultiObjectiveError::ZeroTotalWeight);
        }

        let score = numerator / denominator;

        if score.is_finite() {
            Ok(score)
        } else {
            Err(MultiObjectiveError::ScalarizationOverflow)
        }
    }

    // -------------------------------------------------------------------------
    // Pareto
    // -------------------------------------------------------------------------

    fn dominance_from_evaluations(
        left: &ObjectiveEvaluation,
        right: &ObjectiveEvaluation,
    ) -> Dominance {
        let mut left_better = false;
        let mut right_better = false;

        for (left_value, right_value) in left
            .minimization_values()
            .iter()
            .zip(right.minimization_values())
        {
            let left_value = left_value.get();
            let right_value = right_value.get();

            if left_value < right_value {
                left_better = true;
            } else if left_value > right_value {
                right_better = true;
            }

            if left_better && right_better {
                return Dominance::NonDominated;
            }
        }

        match (left_better, right_better) {
            (true, false) => Dominance::LeftDominates,
            (false, true) => Dominance::RightDominates,
            (false, false) => Dominance::Equal,
            (true, true) => Dominance::NonDominated,
        }
    }

    fn pareto_frontier_indices<T>(
        &self,
        candidates: &[Candidate<T>],
        evaluations: &[ObjectiveEvaluation],
    ) -> Vec<usize> {
        let mut frontier: Vec<usize> = Vec::new();

        for candidate_index in 0..candidates.len() {
            let mut dominated = false;
            let mut removal_positions = Vec::new();

            for (position, &frontier_index) in frontier.iter().enumerate() {
                match Self::dominance_from_evaluations(
                    &evaluations[frontier_index],
                    &evaluations[candidate_index],
                ) {
                    Dominance::LeftDominates => {
                        dominated = true;
                        break;
                    }

                    Dominance::RightDominates => {
                        removal_positions.push(position);
                    }

                    Dominance::Equal => {
                        // Preserve the earlier point. It gives deterministic
                        // frontier semantics without duplicating identical
                        // objective vectors.
                        dominated = true;
                        break;
                    }

                    Dominance::NonDominated => {}
                }
            }

            if dominated {
                continue;
            }

            for position in removal_positions.into_iter().rev() {
                frontier.remove(position);
            }

            frontier.push(candidate_index);
        }

        frontier
    }

    // -------------------------------------------------------------------------
    // Selection
    // -------------------------------------------------------------------------

    fn select_weighted<T>(
        &self,
        candidates: &[Candidate<T>],
        evaluations: &[ObjectiveEvaluation],
    ) -> MultiObjectiveResult<usize> {
        if candidates.is_empty() {
            return Err(MultiObjectiveError::NoCandidates);
        }

        let mut best = 0usize;

        for index in 1..candidates.len() {
            let left = evaluations[index].weighted_score();
            let right = evaluations[best].weighted_score();

            if left < right {
                best = index;
            } else if left == right {
                best = self.tie(best, index, candidates);
            }
        }

        Ok(best)
    }

    fn select_lexicographic<T>(
        &self,
        candidates: &[Candidate<T>],
    ) -> MultiObjectiveResult<usize> {
        if candidates.is_empty() {
            return Err(MultiObjectiveError::NoCandidates);
        }

        let mut best = 0usize;

        for index in 1..candidates.len() {
            let mut comparison = Ordering::Equal;

            for (objective_index, objective) in
                self.config.objectives().iter().enumerate()
            {
                let left = candidates[index]
                    .objectives
                    .get(objective_index)
                    .ok_or_else(|| {
                        MultiObjectiveError::SchemaMismatch {
                            expected: self.config.objectives().len(),
                            actual: candidates[index].objectives.len(),
                        }
                    })?
                    .raw();

                let right = candidates[best]
                    .objectives
                    .get(objective_index)
                    .ok_or_else(|| {
                        MultiObjectiveError::SchemaMismatch {
                            expected: self.config.objectives().len(),
                            actual: candidates[best].objectives.len(),
                        }
                    })?
                    .raw();

                if objective.direction().better(left, right) {
                    comparison = Ordering::Less;
                    break;
                }

                if objective.direction().better(right, left) {
                    comparison = Ordering::Greater;
                    break;
                }
            }

            match comparison {
                Ordering::Less => best = index,
                Ordering::Equal => {
                    best = self.tie(best, index, candidates);
                }
                Ordering::Greater => {}
            }
        }

        Ok(best)
    }

    fn select_from_frontier<T>(
        &self,
        candidates: &[Candidate<T>],
        evaluations: &[ObjectiveEvaluation],
        frontier: &[usize],
    ) -> MultiObjectiveResult<usize> {
        if frontier.is_empty() {
            return Err(MultiObjectiveError::NoCandidates);
        }

        self.select_from_indices(
            candidates,
            evaluations,
            frontier,
        )
    }

    fn select_from_indices<T>(
        &self,
        candidates: &[Candidate<T>],
        evaluations: &[ObjectiveEvaluation],
        indices: &[usize],
    ) -> MultiObjectiveResult<usize> {
        if indices.is_empty() {
            return Err(MultiObjectiveError::NoCandidates);
        }

        let mut best = indices[0];

        for &index in indices.iter().skip(1) {
            let left = evaluations[index].weighted_score();
            let right = evaluations[best].weighted_score();

            if left < right {
                best = index;
            } else if left == right {
                best = self.tie(best, index, candidates);
            }
        }

        Ok(best)
    }

    fn tie<T>(
        &self,
        current: usize,
        challenger: usize,
        candidates: &[Candidate<T>],
    ) -> usize {
        match self.config.tie_policy() {
            TiePolicy::First => current,

            TiePolicy::Last => challenger,

            TiePolicy::LowestId => {
                match (
                    candidates[current].id,
                    candidates[challenger].id,
                ) {
                    (Some(left), Some(right)) => {
                        if right < left {
                            challenger
                        } else {
                            current
                        }
                    }

                    (None, Some(_)) => challenger,
                    _ => current,
                }
            }

            TiePolicy::HighestId => {
                match (
                    candidates[current].id,
                    candidates[challenger].id,
                ) {
                    (Some(left), Some(right)) => {
                        if right > left {
                            challenger
                        } else {
                            current
                        }
                    }

                    (Some(_), None) => current,
                    (None, Some(_)) => challenger,
                    (None, None) => current,
                }
            }
        }
    }
}

// =============================================================================
// Integration helpers
// =============================================================================

/// Creates the standard scheduling objective specification for a
/// `SchedulingObjective` value.
///
/// This adapter deliberately depends only on the stable scheduling configuration
/// contract. It does not import planners, hardware, routing, or QEC.
pub fn specification_for(
    objective: crate::quantum::scheduling::config::SchedulingObjective,
    weight: f64,
    normalization: Normalization,
) -> MultiObjectiveResult<ObjectiveSpec> {
    use crate::quantum::scheduling::config::SchedulingObjective;

    match objective {
        SchedulingObjective::Feasible => ObjectiveSpec::conventional(
            ObjectiveKey::Feasible,
            weight,
            normalization,
        ),

        SchedulingObjective::Makespan => ObjectiveSpec::conventional(
            ObjectiveKey::Makespan,
            weight,
            normalization,
        ),

        SchedulingObjective::Depth => ObjectiveSpec::conventional(
            ObjectiveKey::Depth,
            weight,
            normalization,
        ),

        SchedulingObjective::IdleTime => ObjectiveSpec::conventional(
            ObjectiveKey::IdleTime,
            weight,
            normalization,
        ),

        SchedulingObjective::Fidelity => ObjectiveSpec::conventional(
            ObjectiveKey::Fidelity,
            weight,
            normalization,
        ),

        SchedulingObjective::Energy => ObjectiveSpec::conventional(
            ObjectiveKey::Energy,
            weight,
            normalization,
        ),

        SchedulingObjective::MultiObjective => Err(
            MultiObjectiveError::CandidateEvaluation {
                candidate: "configuration".to_owned(),
                reason: "SchedulingObjective::MultiObjective is a composition mode, not a scalar objective; provide explicit ObjectiveSpec values".to_owned(),
            },
        ),
    }
}

/// Convenience constructor for a standard multi-objective scheduling
/// configuration.
///
/// Every weight remains caller-supplied. No implicit hardware-specific weight
/// exists here.
pub fn standard_configuration(
    makespan_weight: f64,
    depth_weight: f64,
    idle_time_weight: f64,
    fidelity_weight: f64,
    energy_weight: f64,
) -> MultiObjectiveResult<MultiObjectiveConfig> {
    let objectives = vec![
        ObjectiveSpec::conventional(
            ObjectiveKey::Makespan,
            makespan_weight,
            Normalization::Population,
        )?,
        ObjectiveSpec::conventional(
            ObjectiveKey::Depth,
            depth_weight,
            Normalization::Population,
        )?,
        ObjectiveSpec::conventional(
            ObjectiveKey::IdleTime,
            idle_time_weight,
            Normalization::Population,
        )?,
        ObjectiveSpec::conventional(
            ObjectiveKey::Fidelity,
            fidelity_weight,
            Normalization::Population,
        )?,
        ObjectiveSpec::conventional(
            ObjectiveKey::Energy,
            energy_weight,
            Normalization::Population,
        )?,
    ];

    MultiObjectiveConfig::new(objectives)
}

/// Convenience function for Pareto comparison without constructing a full
/// optimizer.
pub fn compare_pareto(
    objectives: &[ObjectiveSpec],
    left: &ObjectiveVector,
    right: &ObjectiveVector,
) -> MultiObjectiveResult<Dominance> {
    let config = MultiObjectiveConfig::new(
        objectives.to_vec(),
    )?;

    let optimizer = MultiObjectiveOptimizer::new(config);

    optimizer.dominance(left, right)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn objective(
        key: ObjectiveKey,
        direction: ObjectiveDirection,
        weight: f64,
    ) -> ObjectiveSpec {
        ObjectiveSpec::new(
            key,
            direction,
            weight,
            Normalization::Range {
                minimum: 0.0,
                maximum: 100.0,
            },
        )
        .expect("valid objective")
    }

    #[test]
    fn rejects_empty_configuration() {
        assert_eq!(
            MultiObjectiveConfig::new(Vec::new()),
            Err(MultiObjectiveError::EmptyObjectiveSet)
        );
    }

    #[test]
    fn rejects_duplicate_objectives() {
        let first = objective(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            1.0,
        );

        let second = objective(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            1.0,
        );

        assert!(matches!(
            MultiObjectiveConfig::new(vec![first, second]),
            Err(MultiObjectiveError::DuplicateObjective { .. })
        ));
    }

    #[test]
    fn rejects_negative_weight() {
        let result = ObjectiveSpec::new(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            -1.0,
            Normalization::None,
        );

        assert!(matches!(
            result,
            Err(MultiObjectiveError::InvalidWeight { .. })
        ));
    }

    #[test]
    fn rejects_nan_weight() {
        let result = ObjectiveSpec::new(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            f64::NAN,
            Normalization::None,
        );

        assert!(matches!(
            result,
            Err(MultiObjectiveError::InvalidWeight { .. })
        ));
    }

    #[test]
    fn rejects_infinite_objective_value() {
        let spec = objective(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            1.0,
        );

        let result = ObjectiveValue::new(&spec, f64::INFINITY);

        assert!(matches!(
            result,
            Err(MultiObjectiveError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn maximize_direction_is_converted_for_scalarization() {
        let spec = objective(
            ObjectiveKey::Fidelity,
            ObjectiveDirection::Maximize,
            1.0,
        );

        let config =
            MultiObjectiveConfig::new(vec![spec])
                .expect("valid config");

        let optimizer = MultiObjectiveOptimizer::new(config);

        let vector =
            ObjectiveVector::from_raw(
                optimizer.config().objectives(),
                &[90.0],
            )
            .expect("valid vector");

        let evaluation =
            optimizer.evaluate(vector)
                .expect("valid evaluation");

        assert!(evaluation.weighted_score() < 0.2);
    }

    #[test]
    fn minimize_direction_is_preserved() {
        let spec = objective(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            1.0,
        );

        let config =
            MultiObjectiveConfig::new(vec![spec])
                .expect("valid config");

        let optimizer = MultiObjectiveOptimizer::new(config);

        let vector =
            ObjectiveVector::from_raw(
                optimizer.config().objectives(),
                &[90.0],
            )
            .expect("valid vector");

        let evaluation =
            optimizer.evaluate(vector)
                .expect("valid evaluation");

        assert!(evaluation.weighted_score() > 0.8);
    }

    #[test]
    fn detects_left_dominance() {
        let objectives = vec![
            objective(
                ObjectiveKey::Makespan,
                ObjectiveDirection::Minimize,
                1.0,
            ),
            objective(
                ObjectiveKey::Fidelity,
                ObjectiveDirection::Maximize,
                1.0,
            ),
        ];

        let config =
            MultiObjectiveConfig::new(objectives.clone())
                .expect("valid config");

        let optimizer = MultiObjectiveOptimizer::new(config);

        let left =
            ObjectiveVector::from_raw(
                &objectives,
                &[20.0, 90.0],
            )
            .expect("valid left");

        let right =
            ObjectiveVector::from_raw(
                &objectives,
                &[40.0, 80.0],
            )
            .expect("valid right");

        assert_eq!(
            optimizer
                .dominance(&left, &right)
                .expect("comparison"),
            Dominance::LeftDominates
        );
    }

    #[test]
    fn detects_non_dominance() {
        let objectives = vec![
            objective(
                ObjectiveKey::Makespan,
                ObjectiveDirection::Minimize,
                1.0,
            ),
            objective(
                ObjectiveKey::Fidelity,
                ObjectiveDirection::Maximize,
                1.0,
            ),
        ];

        let config =
            MultiObjectiveConfig::new(objectives.clone())
                .expect("valid config");

        let optimizer = MultiObjectiveOptimizer::new(config);

        let left =
            ObjectiveVector::from_raw(
                &objectives,
                &[20.0, 70.0],
            )
            .expect("valid left");

        let right =
            ObjectiveVector::from_raw(
                &objectives,
                &[40.0, 90.0],
            )
            .expect("valid right");

        assert_eq!(
            optimizer
                .dominance(&left, &right)
                .expect("comparison"),
            Dominance::NonDominated
        );
    }

    #[test]
    fn finds_pareto_frontier() {
        let objectives = vec![
            objective(
                ObjectiveKey::Makespan,
                ObjectiveDirection::Minimize,
                1.0,
            ),
            objective(
                ObjectiveKey::Fidelity,
                ObjectiveDirection::Maximize,
                1.0,
            ),
        ];

        let config =
            MultiObjectiveConfig::new(objectives.clone())
                .expect("valid config")
                .with_method(OptimizationMethod::Pareto);

        let optimizer = MultiObjectiveOptimizer::new(config);

        let candidates = vec![
            Candidate::new(
                "A",
                Some(1),
                ObjectiveVector::from_raw(
                    &objectives,
                    &[20.0, 80.0],
                )
                .expect("A"),
            ),
            Candidate::new(
                "B",
                Some(2),
                ObjectiveVector::from_raw(
                    &objectives,
                    &[30.0, 70.0],
                )
                .expect("B"),
            ),
            Candidate::new(
                "C",
                Some(3),
                ObjectiveVector::from_raw(
                    &objectives,
                    &[40.0, 90.0],
                )
                .expect("C"),
            ),
        ];

        let result =
            optimizer.optimize(&candidates)
                .expect("optimization");

        assert_eq!(result.pareto_frontier().len(), 2);
    }

    #[test]
    fn deterministic_first_tie_policy() {
        let objectives = vec![
            objective(
                ObjectiveKey::Makespan,
                ObjectiveDirection::Minimize,
                1.0,
            ),
        ];

        let config =
            MultiObjectiveConfig::new(objectives.clone())
                .expect("valid config")
                .with_method(OptimizationMethod::WeightedSum)
                .with_tie_policy(TiePolicy::First);

        let optimizer = MultiObjectiveOptimizer::new(config);

        let vector_a =
            ObjectiveVector::from_raw(
                &objectives,
                &[50.0],
            )
            .expect("A");

        let vector_b =
            ObjectiveVector::from_raw(
                &objectives,
                &[50.0],
            )
            .expect("B");

        let candidates = vec![
            Candidate::new("A", Some(10), vector_a),
            Candidate::new("B", Some(20), vector_b),
        ];

        let result =
            optimizer.optimize(&candidates)
                .expect("optimization");

        assert_eq!(
            result.selected()
                .expect("selected")
                .payload,
            "A"
        );
    }

    #[test]
    fn deterministic_lowest_id_tie_policy() {
        let objectives = vec![
            objective(
                ObjectiveKey::Makespan,
                ObjectiveDirection::Minimize,
                1.0,
            ),
        ];

        let config =
            MultiObjectiveConfig::new(objectives.clone())
                .expect("valid config")
                .with_method(OptimizationMethod::WeightedSum)
                .with_tie_policy(TiePolicy::LowestId);

        let optimizer = MultiObjectiveOptimizer::new(config);

        let candidates = vec![
            Candidate::new(
                "higher",
                Some(20),
                ObjectiveVector::from_raw(
                    &objectives,
                    &[50.0],
                )
                .expect("candidate"),
            ),
            Candidate::new(
                "lower",
                Some(10),
                ObjectiveVector::from_raw(
                    &objectives,
                    &[50.0],
                )
                .expect("candidate"),
            ),
        ];

        let result =
            optimizer.optimize(&candidates)
                .expect("optimization");

        assert_eq!(
            result.selected()
                .expect("selected")
                .payload,
            "lower"
        );
    }

    #[test]
    fn zero_range_population_objective_is_constant() {
        let objective = ObjectiveSpec::conventional(
            ObjectiveKey::Makespan,
            1.0,
            Normalization::Population,
        )
        .expect("objective");

        let config =
            MultiObjectiveConfig::new(vec![objective.clone()])
                .expect("config");

        let optimizer =
            MultiObjectiveOptimizer::new(config);

        let candidates = vec![
            Candidate::new(
                "A",
                None,
                ObjectiveVector::from_raw(
                    &[objective.clone()],
                    &[100.0],
                )
                .expect("A"),
            ),
            Candidate::new(
                "B",
                None,
                ObjectiveVector::from_raw(
                    &[objective],
                    &[100.0],
                )
                .expect("B"),
            ),
        ];

        let result =
            optimizer.optimize(&candidates)
                .expect("optimization");

        assert_eq!(result.selected().unwrap().payload, "A");
    }

    #[test]
    fn standard_configuration_is_explicitly_weighted() {
        let config =
            standard_configuration(
                5.0,
                2.0,
                1.0,
                3.0,
                1.0,
            )
            .expect("configuration");

        assert_eq!(config.objectives().len(), 5);
    }

    #[test]
    fn custom_objective_is_supported() {
        let spec = ObjectiveSpec::new(
            ObjectiveKey::Custom(
                "decoder-latency".to_owned(),
            ),
            ObjectiveDirection::Minimize,
            1.0,
            Normalization::Population,
        )
        .expect("custom objective");

        assert_eq!(
            spec.name(),
            "decoder-latency"
        );
    }

    #[test]
    fn objective_order_is_semantically_stable() {
        let first = objective(
            ObjectiveKey::Makespan,
            ObjectiveDirection::Minimize,
            1.0,
        );

        let second = objective(
            ObjectiveKey::Fidelity,
            ObjectiveDirection::Maximize,
            1.0,
        );

        let config =
            MultiObjectiveConfig::new(
                vec![first.clone(), second.clone()],
            )
            .expect("config");

        assert_eq!(
            config.objectives()[0].key(),
            first.key()
        );

        assert_eq!(
            config.objectives()[1].key(),
            second.key()
        );
    }
}