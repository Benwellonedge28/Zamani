//! Zamani Quantum Algorithms — Objective Contracts.
//!
//! Production-grade objective abstractions for the quantum-algorithm
//! subsystem.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - the canonical objective-function contract;
//! - optimization direction;
//! - objective evaluation records;
//! - objective evaluation accounting;
//! - classical objective adapters;
//! - objective validation;
//! - bounded objective evaluation;
//! - deterministic objective semantics;
//! - objective composition/adaptation;
//! - the contract boundary used by quantum objectives.
//!
//! This module deliberately does NOT own:
//!
//! - parameter storage;
//! - quantum circuit storage;
//! - gate definitions;
//! - ansatz construction;
//! - quantum execution;
//! - backend implementations;
//! - optimizer implementations;
//! - optimizer convergence policy;
//! - Hamiltonian implementations;
//! - hardware topology;
//! - routing;
//! - transpilation;
//! - error correction;
//! - persistence;
//! - telemetry transport.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! types.rs
//! execution.rs
//! variational.rs
//! optimizer.rs
//! quantum::ir
//! quantum::routing
//! quantum::transpiler
//! quantum::error_correction
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                 ParameterVector
//!                       │
//!                       ▼
//!                 Objective
//!                       │
//!          ┌────────────┴────────────┐
//!          │                         │
//!          ▼                         ▼
//!   Classical objective       Quantum objective
//!          │                         │
//!          │                  parameterized circuit
//!          │                         │
//!          │                         ▼
//!          │                 quantum::ir::QuantumCircuit
//!          │                         │
//!          │                         ▼
//!          │                    QuantumExecutor
//!          │                         │
//!          └────────────┬────────────┘
//!                       ▼
//!                ObjectiveValue
//!                       │
//!                       ▼
//!                   Optimizer
//! ```
//!
//! The objective layer therefore defines *what value is being optimized*.
//! It does not define *how the quantum program is executed*.
//!
//! # Important design decision
//!
//! `Objective` returns the canonical [`ObjectiveEvaluation`] rather than a
//! raw `f64`. This preserves room for production accounting such as:
//!
//! - objective value;
//! - evaluation index;
//! - execution count;
//! - shot count;
//! - deterministic status;
//! - optional uncertainty;
//! - optional diagnostic metadata.
//!
//! The optimizer may use `evaluation.value`, while higher-level algorithms
//! retain the complete provenance record.
//!
//! # Determinism
//!
//! This module never creates randomness.
//!
//! A deterministic objective must derive its result exclusively from its
//! explicit inputs and its configured execution context.
//!
//! Randomness required by a quantum objective belongs to `ExecutionConfig`
//! and `QuantumExecutor`.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! Invalid input must return `AlgorithmError` rather than panic.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use super::error::{AlgorithmError, Result};
use super::types::{
    ExecutionConfig,
    ObjectiveValue,
    ParameterVector,
};

// ============================================================================
// Objective direction
// ============================================================================

/// Direction in which an objective is optimized.
///
/// The default quantum-algorithm convention is minimization, which is
/// appropriate for VQE energy minimization and many cost-Hamiltonian
/// formulations.
///
/// Algorithms such as maximization-oriented search may explicitly request
/// `Maximize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveDirection {
    /// Lower objective values are better.
    Minimize,

    /// Higher objective values are better.
    Maximize,
}

impl ObjectiveDirection {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Maximize => "maximize",
        }
    }

    /// Returns whether the supplied candidate is better than the current
    /// value under this optimization direction.
    #[must_use]
    pub fn is_better(
        self,
        candidate: ObjectiveValue,
        current: ObjectiveValue,
    ) -> bool {
        match self {
            Self::Minimize => candidate.get() < current.get(),
            Self::Maximize => candidate.get() > current.get(),
        }
    }

    /// Returns the signed improvement from `current` to `candidate`.
    ///
    /// Positive values mean the candidate is better.
    #[must_use]
    pub fn improvement(
        self,
        candidate: ObjectiveValue,
        current: ObjectiveValue,
    ) -> f64 {
        match self {
            Self::Minimize => current.get() - candidate.get(),
            Self::Maximize => candidate.get() - current.get(),
        }
    }
}

impl Default for ObjectiveDirection {
    fn default() -> Self {
        Self::Minimize
    }
}

impl fmt::Display for ObjectiveDirection {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Objective kind
// ============================================================================

/// Broad class of objective being evaluated.
///
/// This is descriptive metadata, not an execution mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveKind {
    /// Purely classical mathematical objective.
    Classical,

    /// Objective derived from quantum execution.
    Quantum,

    /// Expectation-value objective.
    Expectation,

    /// Cost-function objective.
    Cost,

    /// Energy objective, such as VQE.
    Energy,

    /// Algorithm-specific objective.
    Custom,
}

impl ObjectiveKind {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Classical => "classical",
            Self::Quantum => "quantum",
            Self::Expectation => "expectation",
            Self::Cost => "cost",
            Self::Energy => "energy",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ObjectiveKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Objective evaluation
// ============================================================================

/// Result of one objective evaluation.
///
/// This is deliberately richer than a raw `f64`.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveEvaluation {
    /// Validated objective value.
    pub value: ObjectiveValue,

    /// Monotonically increasing evaluation number within one objective
    /// invocation/session.
    pub evaluation_index: u64,

    /// Number of quantum-circuit executions represented by this evaluation.
    ///
    /// Classical objectives normally report zero.
    pub circuit_executions: u64,

    /// Number of measurement shots represented by this evaluation.
    ///
    /// `None` is used when the objective is not shot-based.
    pub shots: Option<u64>,

    /// Optional standard uncertainty/error estimate.
    pub uncertainty: Option<f64>,

    /// Whether the evaluation was performed under a deterministic contract.
    pub deterministic: bool,
}

impl ObjectiveEvaluation {
    /// Creates a classical objective evaluation.
    pub fn classical(
        value: ObjectiveValue,
        evaluation_index: u64,
    ) -> Result<Self> {
        if evaluation_index == 0 {
            return Err(
                AlgorithmError::invalid_input(
                    "objective evaluation index must be greater than zero",
                ),
            );
        }

        Ok(Self {
            value,
            evaluation_index,
            circuit_executions: 0,
            shots: None,
            uncertainty: None,
            deterministic: true,
        })
    }

    /// Creates a quantum objective evaluation.
    pub fn quantum(
        value: ObjectiveValue,
        evaluation_index: u64,
        circuit_executions: u64,
        shots: Option<u64>,
        deterministic: bool,
    ) -> Result<Self> {
        if evaluation_index == 0 {
            return Err(
                AlgorithmError::invalid_input(
                    "objective evaluation index must be greater than zero",
                ),
            );
        }

        if circuit_executions == 0 {
            return Err(
                AlgorithmError::invalid_input(
                    "quantum objective evaluation must execute at least one circuit",
                ),
            );
        }

        if let Some(shots) = shots {
            if shots == 0 {
                return Err(
                    AlgorithmError::invalid_input(
                        "objective evaluation shots must be greater than zero",
                    ),
                );
            }
        }

        Ok(Self {
            value,
            evaluation_index,
            circuit_executions,
            shots,
            uncertainty: None,
            deterministic,
        })
    }

    /// Adds an uncertainty estimate.
    pub fn with_uncertainty(
        mut self,
        uncertainty: f64,
    ) -> Result<Self> {
        if !uncertainty.is_finite() || uncertainty < 0.0 {
            return Err(
                AlgorithmError::non_finite_value(
                    "objective uncertainty",
                    None,
                    uncertainty,
                    "objective uncertainty must be finite and non-negative",
                ),
            );
        }

        self.uncertainty = Some(uncertainty);

        Ok(self)
    }

    /// Returns whether this evaluation is statistically uncertain.
    #[must_use]
    pub const fn has_uncertainty(&self) -> bool {
        self.uncertainty.is_some()
    }

    /// Validates the complete evaluation record.
    pub fn validate(&self) -> Result<()> {
        if self.evaluation_index == 0 {
            return Err(
                AlgorithmError::internal_invariant(
                    "objective_evaluation_index_nonzero",
                    "objective evaluation index cannot be zero",
                ),
            );
        }

        if self.circuit_executions == 0
            && self.shots.is_some()
        {
            return Err(
                AlgorithmError::internal_invariant(
                    "shots_require_execution",
                    "shot accounting cannot exist without circuit execution",
                ),
            );
        }

        if let Some(shots) = self.shots {
            if shots == 0 {
                return Err(
                    AlgorithmError::invalid_input(
                        "objective evaluation shots must be greater than zero",
                    ),
                );
            }
        }

        if let Some(uncertainty) = self.uncertainty {
            if !uncertainty.is_finite()
                || uncertainty < 0.0
            {
                return Err(
                    AlgorithmError::non_finite_value(
                        "objective uncertainty",
                        None,
                        uncertainty,
                        "objective uncertainty must be finite and non-negative",
                    ),
                );
            }
        }

        ObjectiveValue::new(self.value.get()).map(|_| ())
    }
}

// ============================================================================
// Objective statistics
// ============================================================================

/// Cumulative accounting for objective evaluations.
///
/// This is owned by the objective abstraction rather than the optimizer so
/// that the same accounting remains valid when an objective is used by
/// multiple optimization strategies.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObjectiveStatistics {
    /// Number of evaluations performed.
    pub evaluations: u64,

    /// Number of quantum circuit executions represented by evaluations.
    pub circuit_executions: u64,

    /// Number of measurement shots represented by evaluations.
    pub shots: u64,
}

impl ObjectiveStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            evaluations: 0,
            circuit_executions: 0,
            shots: 0,
        }
    }

    /// Records an objective evaluation.
    pub fn record(
        &mut self,
        evaluation: &ObjectiveEvaluation,
    ) -> Result<()> {
        self.evaluations = self
            .evaluations
            .checked_add(1)
            .ok_or_else(|| {
                AlgorithmError::resource_limit_exceeded(
                    super::error::AlgorithmResource::ObjectiveEvaluations,
                    u128::MAX,
                    u128::MAX,
                    "objective evaluation counter overflowed",
                )
            })?;

        self.circuit_executions = self
            .circuit_executions
            .checked_add(evaluation.circuit_executions)
            .ok_or_else(|| {
                AlgorithmError::resource_limit_exceeded(
                    super::error::AlgorithmResource::CircuitExecutions,
                    u128::MAX,
                    u128::MAX,
                    "objective circuit-execution counter overflowed",
                )
            })?;

        if let Some(shots) = evaluation.shots {
            self.shots = self
                .shots
                .checked_add(shots)
                .ok_or_else(|| {
                    AlgorithmError::resource_limit_exceeded(
                        super::error::AlgorithmResource::Shots,
                        u128::MAX,
                        u128::MAX,
                        "objective shot counter overflowed",
                    )
                })?;
        }

        Ok(())
    }

    /// Resets accounting.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

// ============================================================================
// Objective trait
// ============================================================================

/// Canonical objective-function interface.
///
/// Implementations must:
///
/// 1. validate the supplied parameters;
/// 2. reject non-finite results;
/// 3. return the canonical `ObjectiveEvaluation`;
/// 4. avoid panics for invalid external input.
///
/// The optimizer is responsible for deciding how the returned value affects
/// optimization.
pub trait Objective {
    /// Evaluates the objective for a concrete parameter vector.
    fn evaluate(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveEvaluation>;

    /// Returns the optimization direction.
    #[must_use]
    fn direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }

    /// Returns the objective category.
    #[must_use]
    fn kind(&self) -> ObjectiveKind {
        ObjectiveKind::Custom
    }

    /// Returns the number of evaluations recorded by this objective.
    #[must_use]
    fn statistics(&self) -> ObjectiveStatistics {
        ObjectiveStatistics::new()
    }

    /// Resets objective-local statistics.
    fn reset_statistics(&mut self) {}

    /// Evaluates the objective and returns only the scalar value.
    ///
    /// This is intentionally provided as a convenience method so optimizers
    /// can remain concise without discarding the richer evaluation contract
    /// from the primary `evaluate` method.
    fn evaluate_value(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveValue> {
        let evaluation = self.evaluate(parameters)?;
        evaluation.validate()?;
        Ok(evaluation.value)
    }
}

// ============================================================================
// Classical objective
// ============================================================================

/// Function signature for a classical objective.
pub type ClassicalObjectiveFunction =
    Box<
        dyn FnMut(
                &ParameterVector,
            ) -> Result<ObjectiveValue>
            + Send,
    >;

/// Backend-independent classical objective adapter.
///
/// This is useful for:
///
/// - optimizer unit tests;
/// - classical cost functions;
/// - surrogate objectives;
/// - mathematical optimization;
/// - deterministic reference objectives.
pub struct ClassicalObjective {
    function: ClassicalObjectiveFunction,
    direction: ObjectiveDirection,
    evaluations: ObjectiveStatistics,
}

impl fmt::Debug for ClassicalObjective {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ClassicalObjective")
            .field("direction", &self.direction)
            .field("statistics", &self.evaluations)
            .finish_non_exhaustive()
    }
}

impl ClassicalObjective {
    /// Creates a minimization objective from a callable.
    pub fn new<F>(
        function: F,
    ) -> Self
    where
        F: FnMut(
                &ParameterVector,
            ) -> Result<ObjectiveValue>
            + Send
            + 'static,
    {
        Self {
            function: Box::new(function),
            direction: ObjectiveDirection::Minimize,
            evaluations: ObjectiveStatistics::new(),
        }
    }

    /// Creates a classical objective with an explicit direction.
    pub fn with_direction<F>(
        function: F,
        direction: ObjectiveDirection,
    ) -> Self
    where
        F: FnMut(
                &ParameterVector,
            ) -> Result<ObjectiveValue>
            + Send
            + 'static,
    {
        Self {
            function: Box::new(function),
            direction,
            evaluations: ObjectiveStatistics::new(),
        }
    }

    /// Returns the objective direction.
    #[must_use]
    pub const fn objective_direction(
        &self,
    ) -> ObjectiveDirection {
        self.direction
    }
}

impl Objective for ClassicalObjective {
    fn evaluate(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveEvaluation> {
        parameters.validate()?;

        let evaluation_index = self
            .evaluations
            .evaluations
            .checked_add(1)
            .ok_or_else(|| {
                AlgorithmError::resource_limit_exceeded(
                    super::error::AlgorithmResource::ObjectiveEvaluations,
                    u128::MAX,
                    u128::MAX,
                    "objective evaluation counter overflowed",
                )
            })?;

        let value = (self.function)(parameters)
            .map_err(|error| {
                AlgorithmError::objective_evaluation_failed(
                    Some(evaluation_index),
                    error.to_string(),
                )
            })?;

        let value = ObjectiveValue::new(value.get())
            .map_err(|error| {
                AlgorithmError::objective_evaluation_failed(
                    Some(evaluation_index),
                    error.to_string(),
                )
            })?;

        let evaluation =
            ObjectiveEvaluation::classical(
                value,
                evaluation_index,
            )?;

        self.evaluations.record(&evaluation)?;

        Ok(evaluation)
    }

    fn direction(&self) -> ObjectiveDirection {
        self.direction
    }

    fn kind(&self) -> ObjectiveKind {
        ObjectiveKind::Classical
    }

    fn statistics(&self) -> ObjectiveStatistics {
        self.evaluations.clone()
    }

    fn reset_statistics(&mut self) {
        self.evaluations.reset();
    }
}

// ============================================================================
// Quantum objective contract
// ============================================================================

/// Contract for objectives whose value ultimately comes from quantum
/// execution.
///
/// This trait deliberately does not define circuit construction.
///
/// Circuit/ansatz construction belongs to the variational/algorithm layer.
/// Implementations receive a parameter vector and an executor and may build
/// the appropriate logical IR circuit before crossing the execution boundary.
///
/// This keeps:
///
/// ```text
/// Objective
///     │
///     ├── classical mathematics
///     │
///     └── quantum evaluation contract
///                 │
///                 ▼
///             executor
/// ```
///
/// while keeping hardware knowledge out of the objective subsystem.
pub trait QuantumObjective {
    /// Evaluates the quantum objective using the supplied executor.
    fn evaluate_quantum(
        &mut self,
        parameters: &ParameterVector,
        executor: &mut dyn super::execution::QuantumExecutor,
    ) -> Result<ObjectiveEvaluation>;

    /// Returns the optimization direction.
    #[must_use]
    fn direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }

    /// Returns the quantum-objective subtype.
    #[must_use]
    fn kind(&self) -> ObjectiveKind {
        ObjectiveKind::Quantum
    }

    /// Returns objective-local accounting.
    #[must_use]
    fn statistics(&self) -> ObjectiveStatistics {
        ObjectiveStatistics::new()
    }

    /// Resets objective-local accounting.
    fn reset_statistics(&mut self) {}
}

// ============================================================================
// Quantum objective adapter
// ============================================================================

/// Adapter that turns a quantum objective evaluator into the canonical
/// [`Objective`] contract when the executor is owned by the adapter.
///
/// This is intentionally optional.
///
/// Production VQE/QAOA orchestration will normally retain ownership of the
/// executor and use [`QuantumObjective`] directly. This adapter is useful for
/// self-contained objectives and deterministic tests.
pub struct ExecutorObjective<E, F> {
    executor: E,
    evaluator: F,
    direction: ObjectiveDirection,
    kind: ObjectiveKind,
    evaluations: ObjectiveStatistics,
}

impl<E, F> fmt::Debug for ExecutorObjective<E, F>
where
    E: fmt::Debug,
    F: fmt::Debug,
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ExecutorObjective")
            .field("executor", &self.executor)
            .field("evaluator", &self.evaluator)
            .field("direction", &self.direction)
            .field("kind", &self.kind)
            .field("statistics", &self.evaluations)
            .finish()
    }
}

impl<E, F> ExecutorObjective<E, F> {
    /// Creates an executor-owned objective adapter.
    pub fn new(
        executor: E,
        evaluator: F,
    ) -> Self {
        Self {
            executor,
            evaluator,
            direction: ObjectiveDirection::Minimize,
            kind: ObjectiveKind::Quantum,
            evaluations: ObjectiveStatistics::new(),
        }
    }

    /// Sets the optimization direction.
    #[must_use]
    pub fn with_direction(
        mut self,
        direction: ObjectiveDirection,
    ) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the objective kind.
    #[must_use]
    pub fn with_kind(
        mut self,
        kind: ObjectiveKind,
    ) -> Self {
        self.kind = kind;
        self
    }

    /// Returns a mutable reference to the executor.
    pub fn executor_mut(
        &mut self,
    ) -> &mut E {
        &mut self.executor
    }

    /// Returns immutable objective statistics.
    #[must_use]
    pub fn statistics_ref(
        &self,
    ) -> &ObjectiveStatistics {
        &self.evaluations
    }
}

/// Function signature used by [`ExecutorObjective`].
///
/// The function receives:
///
/// - the parameter vector;
/// - the executor.
///
/// It is responsible for constructing the appropriate logical circuit and
/// performing execution through `super::execution::execute`.
pub type QuantumObjectiveFunction =
    Box<
        dyn FnMut(
                &ParameterVector,
                &mut dyn super::execution::QuantumExecutor,
            ) -> Result<ObjectiveEvaluation>
            + Send,
    >;

impl<E> ExecutorObjective<E, QuantumObjectiveFunction>
where
    E: super::execution::QuantumExecutor,
{
    /// Creates an executor-owned quantum objective from a callable.
    pub fn from_function<F>(
        executor: E,
        function: F,
    ) -> Self
    where
        F: FnMut(
                &ParameterVector,
                &mut dyn super::execution::QuantumExecutor,
            ) -> Result<ObjectiveEvaluation>
            + Send
            + 'static,
    {
        Self::new(
            executor,
            Box::new(function),
        )
    }
}

impl<E> Objective
    for ExecutorObjective<E, QuantumObjectiveFunction>
where
    E: super::execution::QuantumExecutor,
{
    fn evaluate(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveEvaluation> {
        parameters.validate()?;

        let evaluation_index = self
            .evaluations
            .evaluations
            .checked_add(1)
            .ok_or_else(|| {
                AlgorithmError::resource_limit_exceeded(
                    super::error::AlgorithmResource::ObjectiveEvaluations,
                    u128::MAX,
                    u128::MAX,
                    "objective evaluation counter overflowed",
                )
            })?;

        let mut evaluation =
            (self.evaluator)(
                parameters,
                &mut self.executor,
            )
            .map_err(|error| {
                AlgorithmError::objective_evaluation_failed(
                    Some(evaluation_index),
                    error.to_string(),
                )
            })?;

        /*
         * The evaluator is responsible for producing the actual execution
         * metadata, but the adapter owns the canonical evaluation index.
         */
        evaluation.evaluation_index =
            evaluation_index;

        evaluation.validate()?;

        self.evaluations.record(&evaluation)?;

        Ok(evaluation)
    }

    fn direction(&self) -> ObjectiveDirection {
        self.direction
    }

    fn kind(&self) -> ObjectiveKind {
        self.kind
    }

    fn statistics(&self) -> ObjectiveStatistics {
        self.evaluations.clone()
    }

    fn reset_statistics(&mut self) {
        self.evaluations.reset();
    }
}

// ============================================================================
// Objective validation helpers
// ============================================================================

/// Validates a parameter vector before objective evaluation.
pub fn validate_parameters(
    parameters: &ParameterVector,
) -> Result<()> {
    parameters.validate()
}

/// Validates an objective evaluation.
pub fn validate_evaluation(
    evaluation: &ObjectiveEvaluation,
) -> Result<()> {
    evaluation.validate()
}

/// Evaluates an objective while enforcing a maximum evaluation count.
///
/// This provides a single guard that the future optimizer can reuse instead
/// of implementing objective-evaluation limits independently.
pub fn evaluate_with_limit(
    objective: &mut dyn Objective,
    parameters: &ParameterVector,
    max_evaluations: u64,
) -> Result<ObjectiveEvaluation> {
    if max_evaluations == 0 {
        return Err(
            AlgorithmError::invalid_configuration(
                "max_objective_evaluations",
                "maximum objective evaluations must be greater than zero",
            ),
        );
    }

    let current =
        objective.statistics().evaluations;

    if current >= max_evaluations {
        return Err(
            AlgorithmError::resource_limit_exceeded(
                super::error::AlgorithmResource::ObjectiveEvaluations,
                u128::from(current)
                    .saturating_add(1),
                u128::from(max_evaluations),
                "objective evaluation limit exceeded",
            ),
        );
    }

    objective.evaluate(parameters)
}

// ============================================================================
// Objective comparison helpers
// ============================================================================

/// Returns whether `candidate` is better than `current`.
#[must_use]
pub fn is_better(
    direction: ObjectiveDirection,
    candidate: ObjectiveValue,
    current: ObjectiveValue,
) -> bool {
    direction.is_better(candidate, current)
}

/// Returns the improvement from `current` to `candidate`.
#[must_use]
pub fn improvement(
    direction: ObjectiveDirection,
    candidate: ObjectiveValue,
    current: ObjectiveValue,
) -> f64 {
    direction.improvement(
        candidate,
        current,
    )
}

/// Returns whether an objective improvement satisfies a tolerance.
///
/// The tolerance must be finite and non-negative.
pub fn improvement_meets_tolerance(
    direction: ObjectiveDirection,
    candidate: ObjectiveValue,
    current: ObjectiveValue,
    tolerance: f64,
) -> Result<bool> {
    if !tolerance.is_finite()
        || tolerance < 0.0
    {
        return Err(
            AlgorithmError::invalid_parameter(
                None,
                Some(tolerance),
                "objective tolerance must be finite and non-negative",
            ),
        );
    }

    Ok(
        improvement(
            direction,
            candidate,
            current,
        ) >= tolerance,
    )
}

// ============================================================================
// Execution configuration validation
// ============================================================================

/// Validates an objective execution configuration.
///
/// Objective-specific policy is intentionally limited here to invariants
/// common to all objectives. Backend/device policy remains in `execution.rs`.
pub fn validate_execution_config(
    config: &ExecutionConfig,
) -> Result<()> {
    config.validate()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_comparison_is_correct() {
        let low =
            ObjectiveValue::new(1.0)
                .expect("finite value");

        let high =
            ObjectiveValue::new(2.0)
                .expect("finite value");

        assert!(
            ObjectiveDirection::Minimize
                .is_better(low, high)
        );

        assert!(
            ObjectiveDirection::Maximize
                .is_better(high, low)
        );

        assert!(
            !ObjectiveDirection::Minimize
                .is_better(high, low)
        );
    }

    #[test]
    fn improvement_is_direction_aware() {
        let current =
            ObjectiveValue::new(5.0)
                .expect("finite value");

        let candidate =
            ObjectiveValue::new(3.0)
                .expect("finite value");

        assert_eq!(
            ObjectiveDirection::Minimize
                .improvement(
                    candidate,
                    current,
                ),
            2.0
        );

        assert_eq!(
            ObjectiveDirection::Maximize
                .improvement(
                    candidate,
                    current,
                ),
            -2.0
        );
    }

    #[test]
    fn classical_objective_validates_parameters() {
        let mut objective =
            ClassicalObjective::new(
                |parameters| {
                    parameters
                        .require_non_empty()?;

                    let x =
                        parameters
                            .get(0)
                            .ok_or_else(|| {
                                AlgorithmError::invalid_parameter(
                                    Some(0),
                                    None,
                                    "missing parameter",
                                )
                            })?;

                    ObjectiveValue::new(
                        (x - 2.0).powi(2),
                    )
                },
            );

        let parameters =
            ParameterVector::new(
                vec![0.0],
            )
            .expect("valid parameters");

        let evaluation =
            objective
                .evaluate(&parameters)
                .expect("evaluation succeeds");

        assert_eq!(
            evaluation.value.get(),
            4.0
        );

        assert_eq!(
            evaluation.evaluation_index,
            1
        );

        assert_eq!(
            objective.statistics().evaluations,
            1
        );
    }

    #[test]
    fn classical_objective_rejects_objective_errors() {
        let mut objective =
            ClassicalObjective::new(
                |_parameters| {
                    Err(
                        AlgorithmError::invalid_input(
                            "intentional failure",
                        ),
                    )
                },
            );

        let parameters =
            ParameterVector::new(
                vec![0.0],
            )
            .expect("valid parameters");

        let error =
            objective
                .evaluate(&parameters)
                .expect_err(
                    "evaluation must fail",
                );

        assert_eq!(
            error.kind(),
            super::super::error::AlgorithmErrorKind::ObjectiveEvaluationFailed
        );
    }

    #[test]
    fn objective_evaluation_rejects_zero_index() {
        let value =
            ObjectiveValue::new(1.0)
                .expect("finite value");

        assert!(
            ObjectiveEvaluation::classical(
                value,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn quantum_evaluation_requires_execution() {
        let value =
            ObjectiveValue::new(1.0)
                .expect("finite value");

        assert!(
            ObjectiveEvaluation::quantum(
                value,
                1,
                0,
                None,
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn uncertainty_must_be_non_negative_and_finite() {
        let value =
            ObjectiveValue::new(1.0)
                .expect("finite value");

        let evaluation =
            ObjectiveEvaluation::classical(
                value,
                1,
            )
            .expect("valid evaluation");

        assert!(
            evaluation
                .clone()
                .with_uncertainty(
                    f64::NAN,
                )
                .is_err()
        );

        assert!(
            evaluation
                .with_uncertainty(-1.0)
                .is_err()
        );

        assert!(
            ObjectiveEvaluation::classical(
                value,
                1,
            )
            .expect("valid evaluation")
            .with_uncertainty(0.1)
            .is_ok()
        );
    }

    #[test]
    fn evaluation_limit_is_enforced() {
        let mut objective =
            ClassicalObjective::new(
                |_parameters| {
                    ObjectiveValue::new(
                        1.0,
                    )
                },
            );

        let parameters =
            ParameterVector::new(
                vec![0.0],
            )
            .expect("valid parameters");

        evaluate_with_limit(
            &mut objective,
            &parameters,
            1,
        )
        .expect(
            "first evaluation succeeds",
        );

        let error =
            evaluate_with_limit(
                &mut objective,
                &parameters,
                1,
            )
            .expect_err(
                "second evaluation exceeds limit",
            );

        assert_eq!(
            error.kind(),
            super::super::error::AlgorithmErrorKind::ResourceLimitExceeded
        );
    }

    #[test]
    fn objective_tolerance_is_validated() {
        let current =
            ObjectiveValue::new(2.0)
                .expect("finite value");

        let candidate =
            ObjectiveValue::new(1.0)
                .expect("finite value");

        assert!(
            improvement_meets_tolerance(
                ObjectiveDirection::Minimize,
                candidate,
                current,
                0.5,
            )
            .expect("valid tolerance")
        );

        assert!(
            improvement_meets_tolerance(
                ObjectiveDirection::Minimize,
                candidate,
                current,
                f64::NAN,
            )
            .is_err()
        );
    }
}