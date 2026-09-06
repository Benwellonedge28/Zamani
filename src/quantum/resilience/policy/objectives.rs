//! Zamani Quantum Resilience — Policy Objectives
//!
//! Path:
//!     src/quantum/resilience/policy/objectives.rs
//!
//! Purpose:
//!     Defines provider-independent, composable optimization objectives used
//!     by the resilience policy and planning layers.
//!
//! Architectural role:
//!     This module describes WHAT the resilience planner should prefer when
//!     multiple otherwise-valid execution/recovery alternatives exist.
//!
//!     It does NOT:
//!       - execute quantum programs;
//!       - execute recovery actions;
//!       - perform routing;
//!       - perform scheduling;
//!       - perform optimization passes;
//!       - implement QEC;
//!       - implement error mitigation;
//!       - discover hardware;
//!       - inspect provider SDKs;
//!       - define canonical quantum IR;
//!       - define fault semantics;
//!       - enforce hard semantic/security constraints;
//!       - impose machine-size limits.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Fundamental rule
//!
//! Objectives are preferences, not safety guarantees.
//!
//! A candidate execution/recovery plan must first satisfy:
//!
//!     semantic constraints
//!     + capability constraints
//!     + security constraints
//!     + policy constraints
//!     + resource/budget constraints
//!     + verification requirements
//!
//! Only then may objectives be used to rank feasible alternatives.
//!
//! Therefore:
//!
//!     "better availability"
//!
//! MUST NEVER make an otherwise unsafe or semantically invalid action valid.
//!
//! # Write once, scale everywhere
//!
//! No objective contains:
//!
//!     - a fixed qubit count;
//!     - a fixed topology;
//!     - a fixed backend;
//!     - a provider name;
//!     - a fixed retry count;
//!     - a fixed fidelity threshold;
//!     - a fixed execution time;
//!     - a fixed resource limit.
//!
//! All quantitative values are supplied by the caller, planner, capability
//! model, runtime observations, or another authoritative subsystem.
//!
//! This permits the same policy representation to operate over:
//!
//!     one qubit
//!     -> small QPU
//!     -> large QPU
//!     -> logical/fault-tolerant machine
//!     -> multiple QPUs
//!     -> heterogeneous distributed quantum execution.
//!
//! "Infinity" therefore means that this module imposes no artificial finite
//! machine-size ceiling. Actual execution remains bounded by discovered and
//! configured resources.
//!
//! # Determinism
//!
//! Objective evaluation is deterministic for identical inputs.
//!
//! The module:
//!
//!     - performs no I/O;
//!     - reads no environment variables;
//!     - reads no clock;
//!     - owns no global mutable state;
//!     - performs no hidden concurrency;
//!     - performs no random sampling;
//!     - does not depend on hash-map iteration order.
//!
//! Ordered collections are used where collection ordering is observable.
//!
//! # Floating-point policy
//!
//! Floating-point objective values are accepted only when finite.
//!
//! NaN and positive/negative infinity are rejected at construction time.
//! This prevents undefined or non-total ranking behavior from entering the
//! planner.
//!
//! # Security
//!
//! This module does not contain credentials, provider secrets, authentication
//! material, or executable plugin state.
//!
//! # Canonical quantum identity
//!
//! This file normally does not need physical qubit identities because an
//! objective is intentionally resource-agnostic.
//!
//! If a future objective needs to associate evidence with a quantum resource,
//! it MUST use the canonical types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No competing resilience-specific qubit identifier may be introduced.
//!
//! # Integration
//!
//! Policy:
//!     `policy/policy.rs` owns policy-level composition and invokes this
//!     module to represent preferences.
//!
//! Constraints:
//!     `policy/constraints.rs` decides whether a candidate is permitted.
//!     Objectives MUST NOT replace those checks.
//!
//! Budgets:
//!     `policy/budgets.rs` supplies available budgets. Objective values may
//!     describe resource consumption, but this module does not enforce the
//!     budget itself.
//!
//! Planning:
//!     `planning/planner.rs` evaluates feasible alternatives using these
//!     objectives.
//!
//! Ranking:
//!     `planning/ranking.rs` may consume `ObjectiveSet` and `ObjectiveScore`.
//!
//! Cost:
//!     `planning/cost.rs` provides candidate cost/resource measurements.
//!
//! Hardware:
//!     `quantum::hardware` supplies capabilities and observations. This
//!     module never discovers them.
//!
//! Routing:
//!     `quantum::routing` supplies routing results/costs where applicable.
//!
//! Scheduling:
//!     `quantum::scheduling` supplies schedule/timing measurements.
//!
//! Optimization:
//!     `quantum::optimization` supplies optimization results/costs.
//!
//! QEC:
//!     The QEC subsystem supplies logical-error/correction measurements.
//!
//! Verification:
//!     `verification/*` determines whether a candidate is semantically valid.
//!     Objective ranking must occur only after required feasibility checks.
//!
//! Telemetry/history/learning:
//!     These subsystems may provide measurements or predictions used to
//!     populate objective values, but objective evaluation remains deterministic
//!     and must not blindly trust unverified predictions.
//!
//! Serialization:
//!     `serialization/*` may serialize these types. The schema identifier and
//!     version below form this module's compatibility boundary.
//!
//! # Rust contract
//!
//! Target:
//!     Rust 1.97 / Rust 1.97.1
//!
//! Language:
//!     Rust 2021
//!
//! Safety:
//!     `unsafe` is forbidden.
//!
//! Dependencies:
//!     Standard library only.
//!
//! Serde can be added by a higher-level serialization adapter without making
//! this core policy contract depend on a particular wire format.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for resilience objective policies.
pub const RESILIENCE_OBJECTIVES_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy.objectives";

/// Semantic version of the objective contract.
///
/// This version is independent from the Zamani IR version.
pub const RESILIENCE_OBJECTIVES_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Objective identifier
// =============================================================================

/// Identifies a resilience optimization objective.
///
/// Standard objectives cover the dimensions that are common across quantum
/// execution environments. `Custom` allows future objective dimensions to be
/// introduced without requiring a new enum variant for every domain-specific
/// metric.
///
/// A custom objective name is part of the policy contract and therefore must
/// be stable, non-empty, and free of control characters.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectiveKind {
    /// Preserve the semantic correctness of the computation.
    ///
    /// This is normally better represented as a hard constraint/verification
    /// requirement. It exists here because planners may need to expose
    /// correctness as an explicit optimization dimension among already-valid
    /// candidates.
    Correctness,

    /// Prefer higher expected/result fidelity.
    Fidelity,

    /// Prefer lower logical error probability.
    LogicalErrorRate,

    /// Prefer lower physical error exposure.
    PhysicalErrorRate,

    /// Prefer lower end-to-end latency.
    Latency,

    /// Prefer lower execution time.
    ExecutionTime,

    /// Prefer lower queue/wait time.
    QueueTime,

    /// Prefer higher availability/completion probability.
    Availability,

    /// Prefer lower total resource consumption.
    ResourceUsage,

    /// Prefer lower quantum-resource consumption.
    QuantumResourceUsage,

    /// Prefer lower classical-resource consumption.
    ClassicalResourceUsage,

    /// Prefer fewer physical qubits or equivalent physical resources.
    PhysicalResourceUsage,

    /// Prefer lower logical-resource overhead.
    LogicalResourceUsage,

    /// Prefer lower shot/sample consumption.
    ShotUsage,

    /// Prefer lower energy consumption.
    Energy,

    /// Prefer lower monetary or abstract execution cost.
    Cost,

    /// Prefer lower compilation/recompilation effort.
    CompilationCost,

    /// Prefer lower routing overhead.
    RoutingCost,

    /// Prefer lower scheduling overhead.
    SchedulingCost,

    /// Prefer lower mitigation overhead.
    MitigationCost,

    /// Prefer lower QEC overhead.
    QecCost,

    /// Prefer lower recovery overhead.
    RecoveryCost,

    /// Prefer higher resilience/recovery success probability.
    RecoverySuccessProbability,

    /// Prefer lower operational risk.
    Risk,

    /// Prefer lower disruption to an already-running computation.
    Disruption,

    /// Prefer lower amount of program transformation.
    TransformationMagnitude,

    /// Prefer higher stability of the chosen execution environment.
    Stability,

    /// Prefer lower migration overhead.
    MigrationCost,

    /// Application-specific or future objective.
    ///
    /// The string is deliberately used instead of a fixed enum variant so
    /// adding a domain-specific objective does not require changing this
    /// module's standard objective set.
    Custom(String),
}

impl ObjectiveKind {
    /// Returns a stable machine-readable name.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Correctness => "correctness",
            Self::Fidelity => "fidelity",
            Self::LogicalErrorRate => "logical_error_rate",
            Self::PhysicalErrorRate => "physical_error_rate",
            Self::Latency => "latency",
            Self::ExecutionTime => "execution_time",
            Self::QueueTime => "queue_time",
            Self::Availability => "availability",
            Self::ResourceUsage => "resource_usage",
            Self::QuantumResourceUsage => "quantum_resource_usage",
            Self::ClassicalResourceUsage => "classical_resource_usage",
            Self::PhysicalResourceUsage => "physical_resource_usage",
            Self::LogicalResourceUsage => "logical_resource_usage",
            Self::ShotUsage => "shot_usage",
            Self::Energy => "energy",
            Self::Cost => "cost",
            Self::CompilationCost => "compilation_cost",
            Self::RoutingCost => "routing_cost",
            Self::SchedulingCost => "scheduling_cost",
            Self::MitigationCost => "mitigation_cost",
            Self::QecCost => "qec_cost",
            Self::RecoveryCost => "recovery_cost",
            Self::RecoverySuccessProbability => "recovery_success_probability",
            Self::Risk => "risk",
            Self::Disruption => "disruption",
            Self::TransformationMagnitude => "transformation_magnitude",
            Self::Stability => "stability",
            Self::MigrationCost => "migration_cost",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Creates a custom objective identifier.
    ///
    /// Returns an error when the name is empty or contains a control
    /// character.
    pub fn custom<S>(name: S) -> Result<Self, ObjectiveError>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_objective_name(&name)?;

        Ok(Self::Custom(name))
    }

    /// Returns whether this is a custom objective.
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Returns whether the objective is normally a "higher is better"
    /// quantity.
    ///
    /// Cost-like objectives default to minimization.
    pub const fn default_direction(&self) -> ObjectiveDirection {
        match self {
            Self::Fidelity
            | Self::Availability
            | Self::RecoverySuccessProbability
            | Self::Stability
            | Self::Correctness => ObjectiveDirection::Maximize,

            Self::LogicalErrorRate
            | Self::PhysicalErrorRate
            | Self::Latency
            | Self::ExecutionTime
            | Self::QueueTime
            | Self::ResourceUsage
            | Self::QuantumResourceUsage
            | Self::ClassicalResourceUsage
            | Self::PhysicalResourceUsage
            | Self::LogicalResourceUsage
            | Self::ShotUsage
            | Self::Energy
            | Self::Cost
            | Self::CompilationCost
            | Self::RoutingCost
            | Self::SchedulingCost
            | Self::MitigationCost
            | Self::QecCost
            | Self::RecoveryCost
            | Self::Risk
            | Self::Disruption
            | Self::TransformationMagnitude
            | Self::MigrationCost
            | Self::Custom(_) => ObjectiveDirection::Minimize,
        }
    }
}

impl fmt::Display for ObjectiveKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Objective direction
// =============================================================================

/// Defines whether larger or smaller objective values are preferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveDirection {
    /// Larger values are better.
    Maximize,

    /// Smaller values are better.
    Minimize,
}

impl ObjectiveDirection {
    /// Returns a stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Maximize => "maximize",
            Self::Minimize => "minimize",
        }
    }

    /// Converts a raw objective value into a signed optimization value.
    ///
    /// Larger returned values are always better regardless of the original
    /// direction.
    pub fn utility(self, value: ObjectiveValue) -> ObjectiveValue {
        match self {
            Self::Maximize => value,
            Self::Minimize => value.negate(),
        }
    }
}

impl fmt::Display for ObjectiveDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Objective value
// =============================================================================

/// A validated finite scalar used for objective measurements.
///
/// The resilience planner must never rank NaN or infinite objective values.
/// This wrapper enforces that invariant at construction time.
///
/// The value is intentionally represented as `f64` because objective metrics
/// such as probability, fidelity, latency and predicted cost may naturally be
/// fractional. No assumption is made about the scale or physical unit; the
/// objective definition is responsible for declaring its unit/meaning.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveValue(f64);

impl ObjectiveValue {
    /// Creates a finite objective value.
    pub fn new(value: f64) -> Result<Self, ObjectiveError> {
        if !value.is_finite() {
            return Err(ObjectiveError::NonFiniteValue { value });
        }

        Ok(Self(value))
    }

    /// Creates zero.
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns the underlying value.
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns the absolute value.
    pub fn abs(self) -> Self {
        // `self.0` is finite by construction and abs() of a finite f64 is
        // finite.
        Self(self.0.abs())
    }

    /// Returns the negated value.
    pub fn negate(self) -> Self {
        Self(-self.0)
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> Result<Self, ObjectiveError> {
        let value = self.0 + other.0;

        if !value.is_finite() {
            return Err(ObjectiveError::ArithmeticOverflow);
        }

        Ok(Self(value))
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> Result<Self, ObjectiveError> {
        let value = self.0 - other.0;

        if !value.is_finite() {
            return Err(ObjectiveError::ArithmeticOverflow);
        }

        Ok(Self(value))
    }

    /// Checked multiplication.
    pub fn checked_mul(self, other: Self) -> Result<Self, ObjectiveError> {
        let value = self.0 * other.0;

        if !value.is_finite() {
            return Err(ObjectiveError::ArithmeticOverflow);
        }

        Ok(Self(value))
    }

    /// Checked multiplication by a scalar weight.
    pub fn checked_mul_f64(self, multiplier: f64) -> Result<Self, ObjectiveError> {
        if !multiplier.is_finite() {
            return Err(ObjectiveError::NonFiniteWeight {
                weight: multiplier,
            });
        }

        let value = self.0 * multiplier;

        if !value.is_finite() {
            return Err(ObjectiveError::ArithmeticOverflow);
        }

        Ok(Self(value))
    }
}

impl Default for ObjectiveValue {
    fn default() -> Self {
        Self::zero()
    }
}

impl Eq for ObjectiveValue {}

impl PartialOrd for ObjectiveValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectiveValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl fmt::Display for ObjectiveValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.to_string())
    }
}

// =============================================================================
// Objective weight
// =============================================================================

/// Validated objective weight.
///
/// The weight controls the relative influence of an objective during scalar
/// aggregation.
///
/// A weight may be zero, which effectively disables an objective without
/// requiring its definition to be removed from a policy.
///
/// Negative weights are intentionally rejected. Direction must be expressed by
/// `ObjectiveDirection`, not by a negative weight. This prevents ambiguous
/// configurations such as a "maximize" objective with a negative weight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectiveWeight(f64);

impl ObjectiveWeight {
    /// Creates a non-negative finite weight.
    pub fn new(weight: f64) -> Result<Self, ObjectiveError> {
        if !weight.is_finite() {
            return Err(ObjectiveError::NonFiniteWeight { weight });
        }

        if weight < 0.0 {
            return Err(ObjectiveError::NegativeWeight { weight });
        }

        Ok(Self(weight))
    }

    /// Returns zero weight.
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns a neutral unit weight.
    pub const fn one() -> Self {
        Self(1.0)
    }

    /// Returns the raw weight.
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns whether this weight contributes to aggregation.
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }
}

impl Default for ObjectiveWeight {
    fn default() -> Self {
        Self::one()
    }
}

impl Eq for ObjectiveWeight {}

impl PartialOrd for ObjectiveWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectiveWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

// =============================================================================
// Objective specification
// =============================================================================

/// Defines one objective in a policy.
///
/// The specification is immutable and contains no runtime observation itself.
///
/// Runtime candidate measurements belong in `ObjectiveValueSet`.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveSpec {
    kind: ObjectiveKind,
    direction: ObjectiveDirection,
    weight: ObjectiveWeight,
}

impl ObjectiveSpec {
    /// Creates an objective using the objective kind's default direction and
    /// unit weight.
    pub fn new(kind: ObjectiveKind) -> Self {
        let direction = kind.default_direction();

        Self {
            kind,
            direction,
            weight: ObjectiveWeight::one(),
        }
    }

    /// Creates an objective with explicit direction and weight.
    pub fn with_configuration(
        kind: ObjectiveKind,
        direction: ObjectiveDirection,
        weight: ObjectiveWeight,
    ) -> Self {
        Self {
            kind,
            direction,
            weight,
        }
    }

    /// Returns the objective kind.
    pub const fn kind(&self) -> &ObjectiveKind {
        &self.kind
    }

    /// Returns the objective direction.
    pub const fn direction(&self) -> ObjectiveDirection {
        self.direction
    }

    /// Returns the objective weight.
    pub const fn weight(&self) -> ObjectiveWeight {
        self.weight
    }

    /// Returns the stable objective name.
    pub fn name(&self) -> &str {
        self.kind.as_str()
    }

    /// Computes the weighted utility of a raw measurement.
    pub fn utility(&self, value: ObjectiveValue) -> Result<ObjectiveValue, ObjectiveError> {
        let directional = self.direction.utility(value);

        directional.checked_mul_f64(self.weight.get())
    }
}

// =============================================================================
// Objective measurement set
// =============================================================================

/// Candidate measurements supplied to the objective evaluator.
///
/// `BTreeMap` is deliberately used rather than `HashMap` so iteration order is
/// stable and deterministic.
///
/// Missing measurements are not silently interpreted as zero. This is
/// important because a missing metric is not equivalent to a zero metric.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectiveValueSet {
    values: BTreeMap<ObjectiveKind, ObjectiveValue>,
}

impl ObjectiveValueSet {
    /// Creates an empty measurement set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces a measurement.
    pub fn insert(
        &mut self,
        kind: ObjectiveKind,
        value: ObjectiveValue,
    ) -> Option<ObjectiveValue> {
        self.values.insert(kind, value)
    }

    /// Creates a measurement set from an iterator.
    pub fn from_iter<I>(values: I) -> Self
    where
        I: IntoIterator<Item = (ObjectiveKind, ObjectiveValue)>,
    {
        Self {
            values: values.into_iter().collect(),
        }
    }

    /// Returns a measurement.
    pub fn get(&self, kind: &ObjectiveKind) -> Option<ObjectiveValue> {
        self.values.get(kind).copied()
    }

    /// Returns whether a measurement exists.
    pub fn contains(&self, kind: &ObjectiveKind) -> bool {
        self.values.contains_key(kind)
    }

    /// Returns the number of supplied measurements.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no measurements are present.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns deterministic ordered measurements.
    pub fn iter(&self) -> impl Iterator<Item = (&ObjectiveKind, &ObjectiveValue)> {
        self.values.iter()
    }
}

// =============================================================================
// Objective contribution
// =============================================================================

/// Result of evaluating one objective against a candidate measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectiveContribution {
    /// Direction-normalized weighted utility.
    utility: ObjectiveValue,

    /// Raw candidate measurement.
    value: ObjectiveValue,
}

impl ObjectiveContribution {
    /// Creates a contribution.
    pub const fn new(value: ObjectiveValue, utility: ObjectiveValue) -> Self {
        Self { value, utility }
    }

    /// Returns the raw value.
    pub const fn value(self) -> ObjectiveValue {
        self.value
    }

    /// Returns the normalized weighted utility.
    pub const fn utility(self) -> ObjectiveValue {
        self.utility
    }
}

// =============================================================================
// Objective score
// =============================================================================

/// Aggregate score for a candidate.
///
/// A score is meaningful only relative to the exact objective set and values
/// used to compute it. The planner should retain those inputs in provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveScore {
    total: ObjectiveValue,
    contributions: BTreeMap<ObjectiveKind, ObjectiveContribution>,
}

impl ObjectiveScore {
    /// Creates a score.
    pub fn new(
        total: ObjectiveValue,
        contributions: BTreeMap<ObjectiveKind, ObjectiveContribution>,
    ) -> Self {
        Self {
            total,
            contributions,
        }
    }

    /// Returns the aggregate score.
    pub const fn total(&self) -> ObjectiveValue {
        self.total
    }

    /// Returns the contribution of an objective.
    pub fn contribution(
        &self,
        kind: &ObjectiveKind,
    ) -> Option<&ObjectiveContribution> {
        self.contributions.get(kind)
    }

    /// Returns all contributions in deterministic order.
    pub fn contributions(
        &self,
    ) -> impl Iterator<Item = (&ObjectiveKind, &ObjectiveContribution)> {
        self.contributions.iter()
    }

    /// Returns the number of evaluated objectives.
    pub fn len(&self) -> usize {
        self.contributions.len()
    }

    /// Returns whether no objectives were evaluated.
    pub fn is_empty(&self) -> bool {
        self.contributions.is_empty()
    }
}

// =============================================================================
// Objective set
// =============================================================================

/// Immutable-style collection of simultaneously active resilience objectives.
///
/// Objectives are keyed by `ObjectiveKind`. Adding the same objective again
/// replaces its specification, preventing accidental double-counting.
///
/// Ordering is deterministic because the internal collection is ordered.
///
/// No fixed number of objectives is imposed.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ObjectiveSet {
    objectives: BTreeMap<ObjectiveKind, ObjectiveSpec>,
}

impl ObjectiveSet {
    /// Creates an empty objective set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a set from objective specifications.
    pub fn from_specs<I>(specs: I) -> Result<Self, ObjectiveError>
    where
        I: IntoIterator<Item = ObjectiveSpec>,
    {
        let mut set = Self::new();

        for spec in specs {
            set.insert(spec)?;
        }

        Ok(set)
    }

    /// Adds or replaces an objective.
    ///
    /// Replacing an objective is deterministic and prevents duplicate
    /// objective weighting.
    pub fn insert(&mut self, spec: ObjectiveSpec) -> Result<(), ObjectiveError> {
        if spec.name().is_empty() {
            return Err(ObjectiveError::EmptyObjectiveName);
        }

        self.objectives.insert(spec.kind.clone(), spec);

        Ok(())
    }

    /// Removes an objective.
    pub fn remove(&mut self, kind: &ObjectiveKind) -> Option<ObjectiveSpec> {
        self.objectives.remove(kind)
    }

    /// Returns an objective.
    pub fn get(&self, kind: &ObjectiveKind) -> Option<&ObjectiveSpec> {
        self.objectives.get(kind)
    }

    /// Returns whether an objective exists.
    pub fn contains(&self, kind: &ObjectiveKind) -> bool {
        self.objectives.contains_key(kind)
    }

    /// Returns the number of objectives.
    pub fn len(&self) -> usize {
        self.objectives.len()
    }

    /// Returns whether the set has no objectives.
    pub fn is_empty(&self) -> bool {
        self.objectives.is_empty()
    }

    /// Returns objectives in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&ObjectiveKind, &ObjectiveSpec)> {
        self.objectives.iter()
    }

    /// Returns whether at least one objective has non-zero weight.
    pub fn has_active_objective(&self) -> bool {
        self.objectives.values().any(|spec| !spec.weight().is_zero())
    }

    /// Evaluates all configured objectives against candidate measurements.
    ///
    /// Every configured objective must have a measurement.
    ///
    /// This deliberate strictness prevents missing data from silently becoming
    /// an apparently favorable zero.
    pub fn evaluate(
        &self,
        values: &ObjectiveValueSet,
    ) -> Result<ObjectiveScore, ObjectiveError> {
        let mut total = ObjectiveValue::zero();
        let mut contributions = BTreeMap::new();

        for (kind, spec) in &self.objectives {
            let value = values
                .get(kind)
                .ok_or_else(|| ObjectiveError::MissingValue {
                    objective: kind.clone(),
                })?;

            let utility = spec.utility(value)?;

            total = total.checked_add(utility)?;

            contributions.insert(
                kind.clone(),
                ObjectiveContribution::new(value, utility),
            );
        }

        Ok(ObjectiveScore::new(total, contributions))
    }
}

// =============================================================================
// Built-in objective policies
// =============================================================================

impl ObjectiveSet {
    /// Creates a correctness-first objective set.
    ///
    /// Important:
    ///     This does NOT make correctness a hard safety guarantee. Semantic
    ///     validity must still be enforced by policy constraints and
    ///     verification.
    pub fn correctness_first() -> Self {
        let mut set = Self::new();

        // These constructions cannot fail because all constants are finite and
        // non-negative. The helper is intentionally private so production code
        // does not need to handle impossible construction errors here.
        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::Correctness,
            ObjectiveDirection::Maximize,
            ObjectiveWeight::one(),
        ));
        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::Fidelity,
            ObjectiveDirection::Maximize,
            ObjectiveWeight::one(),
        ));
        set
    }

    /// Creates an availability-oriented objective set.
    ///
    /// This still does not override semantic verification or safety policy.
    pub fn availability_first() -> Self {
        let mut set = Self::new();

        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::Availability,
            ObjectiveDirection::Maximize,
            ObjectiveWeight::one(),
        ));
        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::Latency,
            ObjectiveDirection::Minimize,
            ObjectiveWeight::one(),
        ));

        set
    }

    /// Creates a resource-efficiency objective set.
    pub fn resource_efficient() -> Self {
        let mut set = Self::new();

        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::ResourceUsage,
            ObjectiveDirection::Minimize,
            ObjectiveWeight::one(),
        ));
        set.insert_unchecked(ObjectiveSpec::with_configuration(
            ObjectiveKind::Cost,
            ObjectiveDirection::Minimize,
            ObjectiveWeight::one(),
        ));

        set
    }

    /// Creates an empty objective policy.
    ///
    /// An empty set means the planner must not infer preferences from this
    /// module. The caller/policy layer may then apply its own deterministic
    /// tie-breaking rules.
    pub fn none() -> Self {
        Self::new()
    }

    fn insert_unchecked(&mut self, spec: ObjectiveSpec) {
        self.objectives.insert(spec.kind.clone(), spec);
    }
}

// =============================================================================
// Objective comparison
// =============================================================================

/// Compares two objective scores.
///
/// Returns:
///
///     Greater -> left candidate is preferred
///     Equal   -> no preference
///     Less    -> right candidate is preferred
///
/// This comparison is deterministic.
///
/// The comparison is based on the aggregate utility. When aggregate utility is
/// equal, objective contributions are compared in deterministic objective-key
/// order to avoid depending on collection insertion order.
///
/// The planner may use a richer Pareto/ranking algorithm instead; this function
/// provides a deterministic scalar ordering only.
pub fn compare_scores(left: &ObjectiveScore, right: &ObjectiveScore) -> Ordering {
    match left.total.cmp(&right.total) {
        Ordering::Equal => compare_contributions(left, right),
        ordering => ordering,
    }
}

fn compare_contributions(
    left: &ObjectiveScore,
    right: &ObjectiveScore,
) -> Ordering {
    let mut left_iter = left.contributions.iter();
    let mut right_iter = right.contributions.iter();

    loop {
        match (left_iter.next(), right_iter.next()) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (Some((left_kind, left_value)), Some((right_kind, right_value))) => {
                match left_kind.cmp(right_kind) {
                    Ordering::Equal => {
                        match left_value.utility.cmp(&right_value.utility) {
                            Ordering::Equal => continue,
                            ordering => return ordering,
                        }
                    }
                    ordering => return ordering,
                }
            }
        }
    }
}

// =============================================================================
// Objective validation
// =============================================================================

/// Validates an objective set before it is accepted by a policy.
///
/// This is intentionally separate from policy constraints. It checks the
/// objective representation itself, not whether a quantum execution is safe.
pub fn validate_objectives(
    objectives: &ObjectiveSet,
) -> Result<(), ObjectiveError> {
    for (kind, spec) in objectives.iter() {
        validate_objective_name(kind.as_str())?;

        if spec.name().is_empty() {
            return Err(ObjectiveError::EmptyObjectiveName);
        }

        if !spec.weight().get().is_finite() {
            return Err(ObjectiveError::NonFiniteWeight {
                weight: spec.weight().get(),
            });
        }
    }

    Ok(())
}

fn validate_objective_name(name: &str) -> Result<(), ObjectiveError> {
    if name.is_empty() {
        return Err(ObjectiveError::EmptyObjectiveName);
    }

    if name.chars().any(char::is_control) {
        return Err(ObjectiveError::InvalidObjectiveName);
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the objective subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveError {
    /// Objective identifier is empty.
    EmptyObjectiveName,

    /// Objective identifier contains a control character.
    InvalidObjectiveName,

    /// Objective measurement is not finite.
    NonFiniteValue {
        /// Invalid floating-point value.
        value: f64,
    },

    /// Objective weight is not finite.
    NonFiniteWeight {
        /// Invalid weight.
        weight: f64,
    },

    /// Objective weight is negative.
    NegativeWeight {
        /// Invalid weight.
        weight: f64,
    },

    /// An objective required for evaluation has no candidate measurement.
    MissingValue {
        /// Objective whose value is missing.
        objective: ObjectiveKind,
    },

    /// Arithmetic would produce a non-finite result.
    ArithmeticOverflow,
}

impl fmt::Display for ObjectiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObjectiveName => {
                formatter.write_str("objective name must not be empty")
            }
            Self::InvalidObjectiveName => {
                formatter.write_str("objective name contains a control character")
            }
            Self::NonFiniteValue { value } => {
                write!(formatter, "objective value is not finite: {value}")
            }
            Self::NonFiniteWeight { weight } => {
                write!(formatter, "objective weight is not finite: {weight}")
            }
            Self::NegativeWeight { weight } => {
                write!(formatter, "objective weight must not be negative: {weight}")
            }
            Self::MissingValue { objective } => {
                write!(
                    formatter,
                    "objective measurement is missing: {objective}"
                )
            }
            Self::ArithmeticOverflow => {
                formatter.write_str("objective arithmetic produced a non-finite value")
            }
        }
    }
}

impl std::error::Error for ObjectiveError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn objective_value_rejects_nan() {
        let result = ObjectiveValue::new(f64::NAN);

        assert!(matches!(
            result,
            Err(ObjectiveError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn objective_value_rejects_positive_infinity() {
        let result = ObjectiveValue::new(f64::INFINITY);

        assert!(matches!(
            result,
            Err(ObjectiveError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn objective_value_rejects_negative_infinity() {
        let result = ObjectiveValue::new(f64::NEG_INFINITY);

        assert!(matches!(
            result,
            Err(ObjectiveError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn objective_weight_rejects_negative_values() {
        let result = ObjectiveWeight::new(-1.0);

        assert!(matches!(
            result,
            Err(ObjectiveError::NegativeWeight { .. })
        ));
    }

    #[test]
    fn objective_weight_rejects_nan() {
        let result = ObjectiveWeight::new(f64::NAN);

        assert!(matches!(
            result,
            Err(ObjectiveError::NonFiniteWeight { .. })
        ));
    }

    #[test]
    fn objective_weight_rejects_infinity() {
        let result = ObjectiveWeight::new(f64::INFINITY);

        assert!(matches!(
            result,
            Err(ObjectiveError::NonFiniteWeight { .. })
        ));
    }

    #[test]
    fn custom_objective_rejects_empty_name() {
        let result = ObjectiveKind::custom("");

        assert!(matches!(
            result,
            Err(ObjectiveError::EmptyObjectiveName)
        ));
    }

    #[test]
    fn custom_objective_rejects_control_characters() {
        let result = ObjectiveKind::custom("latency\nattack");

        assert!(matches!(
            result,
            Err(ObjectiveError::InvalidObjectiveName)
        ));
    }

    #[test]
    fn custom_objective_accepts_stable_name() {
        let result = ObjectiveKind::custom("application.expected_value");

        assert!(result.is_ok());

        assert_eq!(
            result.expect("validated custom objective").as_str(),
            "application.expected_value"
        );
    }

    #[test]
    fn objective_set_replaces_duplicate_objective() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::with_configuration(
                ObjectiveKind::Latency,
                ObjectiveDirection::Minimize,
                ObjectiveWeight::one(),
            ))
            .expect("valid objective");

        objectives
            .insert(ObjectiveSpec::with_configuration(
                ObjectiveKind::Latency,
                ObjectiveDirection::Minimize,
                ObjectiveWeight::new(2.0).expect("valid weight"),
            ))
            .expect("valid replacement");

        assert_eq!(objectives.len(), 1);

        assert_eq!(
            objectives
                .get(&ObjectiveKind::Latency)
                .expect("latency objective")
                .weight()
                .get(),
            2.0
        );
    }

    #[test]
    fn minimize_direction_is_normalized() {
        let spec = ObjectiveSpec::with_configuration(
            ObjectiveKind::Latency,
            ObjectiveDirection::Minimize,
            ObjectiveWeight::one(),
        );

        let value = ObjectiveValue::new(10.0).expect("finite value");

        let utility = spec.utility(value).expect("valid utility");

        assert_eq!(utility.get(), -10.0);
    }

    #[test]
    fn maximize_direction_is_preserved() {
        let spec = ObjectiveSpec::with_configuration(
            ObjectiveKind::Fidelity,
            ObjectiveDirection::Maximize,
            ObjectiveWeight::one(),
        );

        let value = ObjectiveValue::new(0.98).expect("finite value");

        let utility = spec.utility(value).expect("valid utility");

        assert_eq!(utility.get(), 0.98);
    }

    #[test]
    fn weight_is_applied() {
        let spec = ObjectiveSpec::with_configuration(
            ObjectiveKind::Fidelity,
            ObjectiveDirection::Maximize,
            ObjectiveWeight::new(2.0).expect("valid weight"),
        );

        let value = ObjectiveValue::new(0.5).expect("finite value");

        let utility = spec.utility(value).expect("valid utility");

        assert_eq!(utility.get(), 1.0);
    }

    #[test]
    fn multiple_objectives_are_supported() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Latency))
            .expect("valid objective");

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Cost))
            .expect("valid objective");

        assert_eq!(objectives.len(), 3);
    }

    #[test]
    fn evaluation_requires_all_values() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Latency))
            .expect("valid objective");

        let mut values = ObjectiveValueSet::new();

        values.insert(
            ObjectiveKind::Fidelity,
            ObjectiveValue::new(0.99).expect("finite value"),
        );

        let result = objectives.evaluate(&values);

        assert!(matches!(
            result,
            Err(ObjectiveError::MissingValue {
                objective: ObjectiveKind::Latency
            })
        ));
    }

    #[test]
    fn evaluation_is_deterministic() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Latency))
            .expect("valid objective");

        let values = ObjectiveValueSet::from_iter([
            (
                ObjectiveKind::Latency,
                ObjectiveValue::new(10.0).expect("finite value"),
            ),
            (
                ObjectiveKind::Fidelity,
                ObjectiveValue::new(0.99).expect("finite value"),
            ),
        ]);

        let first = objectives.evaluate(&values).expect("evaluation succeeds");
        let second = objectives.evaluate(&values).expect("evaluation succeeds");

        assert_eq!(first, second);
    }

    #[test]
    fn better_fidelity_produces_higher_score() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        let mut low_values = ObjectiveValueSet::new();

        low_values.insert(
            ObjectiveKind::Fidelity,
            ObjectiveValue::new(0.90).expect("finite value"),
        );

        let mut high_values = ObjectiveValueSet::new();

        high_values.insert(
            ObjectiveKind::Fidelity,
            ObjectiveValue::new(0.99).expect("finite value"),
        );

        let low = objectives
            .evaluate(&low_values)
            .expect("evaluation succeeds");

        let high = objectives
            .evaluate(&high_values)
            .expect("evaluation succeeds");

        assert_eq!(compare_scores(&high, &low), Ordering::Greater);
    }

    #[test]
    fn lower_latency_produces_higher_score() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::new(ObjectiveKind::Latency))
            .expect("valid objective");

        let mut slow_values = ObjectiveValueSet::new();

        slow_values.insert(
            ObjectiveKind::Latency,
            ObjectiveValue::new(100.0).expect("finite value"),
        );

        let mut fast_values = ObjectiveValueSet::new();

        fast_values.insert(
            ObjectiveKind::Latency,
            ObjectiveValue::new(10.0).expect("finite value"),
        );

        let slow = objectives
            .evaluate(&slow_values)
            .expect("evaluation succeeds");

        let fast = objectives
            .evaluate(&fast_values)
            .expect("evaluation succeeds");

        assert_eq!(compare_scores(&fast, &slow), Ordering::Greater);
    }

    #[test]
    fn zero_weight_does_not_contribute() {
        let mut objectives = ObjectiveSet::new();

        objectives
            .insert(ObjectiveSpec::with_configuration(
                ObjectiveKind::Latency,
                ObjectiveDirection::Minimize,
                ObjectiveWeight::zero(),
            ))
            .expect("valid objective");

        let mut values = ObjectiveValueSet::new();

        values.insert(
            ObjectiveKind::Latency,
            ObjectiveValue::new(1000.0).expect("finite value"),
        );

        let score = objectives.evaluate(&values).expect("evaluation succeeds");

        assert_eq!(score.total().get(), 0.0);
    }

    #[test]
    fn objective_names_are_stable() {
        assert_eq!(
            ObjectiveKind::LogicalErrorRate.as_str(),
            "logical_error_rate"
        );

        assert_eq!(
            ObjectiveKind::RecoverySuccessProbability.as_str(),
            "recovery_success_probability"
        );

        assert_eq!(
            ObjectiveKind::MigrationCost.as_str(),
            "migration_cost"
        );
    }

    #[test]
    fn correctness_first_contains_multiple_objectives() {
        let objectives = ObjectiveSet::correctness_first();

        assert!(objectives.contains(&ObjectiveKind::Correctness));
        assert!(objectives.contains(&ObjectiveKind::Fidelity));
    }

    #[test]
    fn availability_first_does_not_override_semantic_constraints() {
        let objectives = ObjectiveSet::availability_first();

        assert!(objectives.contains(&ObjectiveKind::Availability));

        // This test documents an architectural property: this module only
        // expresses preferences. It has no API capable of accepting an
        // otherwise invalid execution.
        assert!(!objectives.contains(&ObjectiveKind::Correctness));
    }

    #[test]
    fn deterministic_order_is_independent_of_insertion_order() {
        let mut first = ObjectiveSet::new();
        let mut second = ObjectiveSet::new();

        first
            .insert(ObjectiveSpec::new(ObjectiveKind::Cost))
            .expect("valid objective");

        first
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        second
            .insert(ObjectiveSpec::new(ObjectiveKind::Fidelity))
            .expect("valid objective");

        second
            .insert(ObjectiveSpec::new(ObjectiveKind::Cost))
            .expect("valid objective");

        assert_eq!(first, second);
    }

    #[test]
    fn no_objectives_is_valid() {
        let objectives = ObjectiveSet::none();

        assert!(objectives.is_empty());
        assert!(!objectives.has_active_objective());

        let values = ObjectiveValueSet::new();

        let score = objectives.evaluate(&values).expect("empty evaluation");

        assert_eq!(score.total().get(), 0.0);
        assert!(score.is_empty());
    }

    #[test]
    fn objective_value_order_is_total_for_finite_values() {
        let a = ObjectiveValue::new(-1.0).expect("finite value");
        let b = ObjectiveValue::new(1.0).expect("finite value");

        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn checked_arithmetic_rejects_non_finite_results() {
        let huge = ObjectiveValue::new(f64::MAX).expect("finite value");

        let result = huge.checked_mul_f64(2.0);

        assert!(matches!(
            result,
            Err(ObjectiveError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn validation_accepts_built_in_objectives() {
        let objectives = ObjectiveSet::correctness_first();

        assert!(validate_objectives(&objectives).is_ok());
    }
}