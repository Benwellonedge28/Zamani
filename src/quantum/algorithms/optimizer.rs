//! Zamani Quantum Algorithms — Optimization Engine.
//!
//! Production-grade, backend-independent classical optimization contracts
//! used by variational quantum algorithms.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - optimizer configuration;
//! - convergence policy;
//! - convergence status;
//! - optimization statistics;
//! - optimization history;
//! - optimizer state;
//! - optimizer trait;
//! - deterministic gradient descent;
//! - numerical finite-difference gradients;
//! - resource accounting;
//! - optimizer result contracts.
//!
//! This module deliberately does NOT own:
//!
//! - parameter-vector representation;
//! - objective-function representation;
//! - quantum circuits;
//! - quantum gates;
//! - quantum execution;
//! - hardware;
//! - routing;
//! - transpilation;
//! - error correction;
//! - VQE;
//! - QAOA;
//! - algorithm-specific orchestration.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! types.rs
//! objective.rs
//! execution.rs
//! variational.rs
//! quantum::ir
//! quantum::routing
//! quantum::transpiler
//! quantum::error_correction
//! ```
//!
//! # Architectural position
//!
//! ```text
//! ParameterVector
//!       │
//!       ▼
//! Objective
//!       │
//!       ▼
//! Optimizer
//!       │
//!       ├── GradientDescent
//!       ├── ParameterShift
//!       ├── SPSA
//!       └── future optimizers
//!       │
//!       ▼
//! OptimizationResult
//!       │
//!       ▼
//! VariationalAlgorithm
//! ```
//!
//! The optimizer consumes objective evaluations. It does not know whether
//! those evaluations came from a classical function, a simulator, a GPU,
//! or physical quantum hardware.
//!
//! # Determinism
//!
//! Gradient descent implemented here is deterministic because it does not
//! create randomness. Any stochastic optimizer must receive its explicit
//! seed through the surrounding algorithm configuration and must never use
//! global randomness.
//!
//! # Numerical safety
//!
//! Every externally observable floating-point value is validated as finite.
//! Optimizer-generated parameter values, gradients, objective values,
//! learning rates, tolerances, and convergence metrics must not become NaN
//! or infinite.
//!
//! # Resource safety
//!
//! Optimization is bounded by explicit limits for:
//!
//! - iterations;
//! - optimizer steps;
//! - objective evaluations;
//! - gradient evaluations;
//! - parameters;
//! - parameter magnitude.
//!
//! Counters use checked arithmetic.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//! No nightly features.
//! No external dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use super::error::{AlgorithmError, AlgorithmResource, Result};
use super::objective::{
    Objective,
    ObjectiveDirection,
    ObjectiveEvaluation,
};
use super::types::{
    ParameterVector,
    DEFAULT_MAX_GRADIENT_EVALUATIONS,
    DEFAULT_MAX_ITERATIONS,
    DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
    DEFAULT_MAX_OPTIMIZER_STEPS,
    DEFAULT_MAX_PARAMETER_MAGNITUDE,
};

// =============================================================================
// Optimization configuration
// =============================================================================

/// Configuration shared by classical optimizers.
///
/// The configuration is deliberately independent from quantum execution.
/// Quantum-specific resource constraints are enforced by the objective and
/// execution layers.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationConfig {
    /// Initial learning/update rate.
    pub learning_rate: f64,

    /// Absolute objective/gradient convergence tolerance.
    pub tolerance: f64,

    /// Maximum optimizer iterations.
    pub max_iterations: u64,

    /// Finite-difference gradient step.
    ///
    /// Used by `GradientDescent`.
    pub gradient_epsilon: f64,

    /// Number of consecutive iterations permitted without meaningful
    /// objective improvement.
    ///
    /// `0` disables patience-based convergence.
    pub patience: u64,

    /// Minimum objective improvement considered meaningful.
    pub objective_improvement_tolerance: f64,

    /// Maximum permitted absolute parameter magnitude.
    pub max_parameter_magnitude: f64,

    /// Maximum objective evaluations.
    pub max_objective_evaluations: u64,

    /// Maximum gradient evaluations.
    pub max_gradient_evaluations: u64,

    /// Maximum optimizer steps.
    pub max_optimizer_steps: u64,

    /// Minimum permitted learning rate.
    ///
    /// This protects against configuration values that are technically
    /// positive but numerically meaningless.
    pub min_learning_rate: f64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            tolerance: 1.0e-8,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            gradient_epsilon: 1.0e-6,
            patience: 20,
            objective_improvement_tolerance: 1.0e-10,
            max_parameter_magnitude: DEFAULT_MAX_PARAMETER_MAGNITUDE,
            max_objective_evaluations:
                DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
            max_gradient_evaluations:
                DEFAULT_MAX_GRADIENT_EVALUATIONS,
            max_optimizer_steps:
                DEFAULT_MAX_OPTIMIZER_STEPS,
            min_learning_rate: 1.0e-15,
        }
    }
}

impl OptimizationConfig {
    /// Validates the optimizer configuration.
    pub fn validate(&self) -> Result<()> {
        validate_positive_finite(
            self.learning_rate,
            "learning_rate",
        )?;

        validate_positive_finite(
            self.tolerance,
            "tolerance",
        )?;

        validate_positive_finite(
            self.gradient_epsilon,
            "gradient_epsilon",
        )?;

        validate_positive_finite(
            self.objective_improvement_tolerance,
            "objective_improvement_tolerance",
        )?;

        validate_positive_finite(
            self.max_parameter_magnitude,
            "max_parameter_magnitude",
        )?;

        validate_positive_finite(
            self.min_learning_rate,
            "min_learning_rate",
        )?;

        if self.learning_rate < self.min_learning_rate {
            return Err(
                AlgorithmError::invalid_configuration(
                    "learning_rate",
                    format!(
                        "learning rate {} is below minimum {}",
                        self.learning_rate,
                        self.min_learning_rate
                    ),
                ),
            );
        }

        if self.max_iterations == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "max_iterations",
                    "maximum iterations must be greater than zero",
                ),
            );
        }

        if self.max_objective_evaluations == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "max_objective_evaluations",
                    "maximum objective evaluations must be greater than zero",
                ),
            );
        }

        if self.max_gradient_evaluations == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "max_gradient_evaluations",
                    "maximum gradient evaluations must be greater than zero",
                ),
            );
        }

        if self.max_optimizer_steps == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "max_optimizer_steps",
                    "maximum optimizer steps must be greater than zero",
                ),
            );
        }

        Ok(())
    }
}

// =============================================================================
// Convergence status
// =============================================================================

/// Terminal status of an optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvergenceStatus {
    /// Required convergence criteria were satisfied.
    Converged,

    /// Maximum configured iterations were reached.
    MaxIterations,

    /// Maximum objective evaluations were reached.
    MaxEvaluations,

    /// Maximum optimizer steps were reached.
    MaxOptimizerSteps,

    /// Objective value stopped improving.
    Stagnated,

    /// Optimization became numerically divergent.
    Diverged,

    /// Numerical processing failed.
    NumericalFailure,

    /// A configured resource limit prevented continuation.
    ResourceLimit,

    /// Optimization was explicitly cancelled.
    Cancelled,
}

impl ConvergenceStatus {
    /// Returns the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Converged => "converged",
            Self::MaxIterations => "max_iterations",
            Self::MaxEvaluations => "max_evaluations",
            Self::MaxOptimizerSteps => "max_optimizer_steps",
            Self::Stagnated => "stagnated",
            Self::Diverged => "diverged",
            Self::NumericalFailure => "numerical_failure",
            Self::ResourceLimit => "resource_limit",
            Self::Cancelled => "cancelled",
        }
    }

    /// Returns whether the optimizer achieved its requested convergence
    /// criteria.
    #[must_use]
    pub const fn is_converged(self) -> bool {
        matches!(self, Self::Converged)
    }

    /// Returns whether the status represents a non-success terminal state.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::Diverged | Self::NumericalFailure
        )
    }
}

impl fmt::Display for ConvergenceStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Optimization step
// =============================================================================

/// One immutable optimization observation.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationStep {
    /// One-based optimization iteration.
    pub iteration: u64,

    /// Objective evaluation used for this step.
    pub evaluation: ObjectiveEvaluation,

    /// L2 norm of the gradient used for the update.
    pub gradient_norm: f64,

    /// Largest absolute gradient component.
    pub max_gradient_component: f64,

    /// Largest absolute parameter update.
    pub max_parameter_change: f64,

    /// Objective improvement from the preceding accepted objective.
    pub objective_improvement: f64,
}

impl OptimizationStep {
    /// Validates the step.
    pub fn validate(&self) -> Result<()> {
        if self.iteration == 0 {
            return Err(
                AlgorithmError::internal_invariant(
                    "optimization_iteration_nonzero",
                    "optimization iterations are one-based",
                ),
            );
        }

        self.evaluation.validate()?;

        validate_finite(
            self.gradient_norm,
            "gradient_norm",
        )?;

        validate_finite(
            self.max_gradient_component,
            "max_gradient_component",
        )?;

        validate_finite(
            self.max_parameter_change,
            "max_parameter_change",
        )?;

        validate_finite(
            self.objective_improvement,
            "objective_improvement",
        )?;

        if self.gradient_norm < 0.0 {
            return Err(
                AlgorithmError::internal_invariant(
                    "gradient_norm_nonnegative",
                    "gradient norm cannot be negative",
                ),
            );
        }

        if self.max_gradient_component < 0.0 {
            return Err(
                AlgorithmError::internal_invariant(
                    "max_gradient_component_nonnegative",
                    "maximum gradient component cannot be negative",
                ),
            );
        }

        if self.max_parameter_change < 0.0 {
            return Err(
                AlgorithmError::internal_invariant(
                    "max_parameter_change_nonnegative",
                    "maximum parameter change cannot be negative",
                ),
            );
        }

        Ok(())
    }
}

// =============================================================================
// Optimization statistics
// =============================================================================

/// Immutable accounting returned by an optimization run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationStatistics {
    /// Number of optimizer iterations performed.
    pub iterations: u64,

    /// Number of accepted parameter updates.
    pub optimizer_steps: u64,

    /// Number of objective evaluations consumed.
    pub objective_evaluations: u64,

    /// Number of gradient evaluations.
    pub gradient_evaluations: u64,

    /// Number of individual gradient components evaluated.
    pub gradient_components: u64,
}

impl OptimizationStatistics {
    /// Creates empty optimization statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            iterations: 0,
            optimizer_steps: 0,
            objective_evaluations: 0,
            gradient_evaluations: 0,
            gradient_components: 0,
        }
    }
}

impl Default for OptimizationStatistics {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Optimization result
// =============================================================================

/// Complete optimizer result.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationResult {
    /// Final parameter vector.
    pub parameters: ParameterVector,

    /// Final objective value.
    pub objective: super::types::ObjectiveValue,

    /// Best objective value observed during optimization.
    pub best_objective: super::types::ObjectiveValue,

    /// Parameters associated with `best_objective`.
    pub best_parameters: ParameterVector,

    /// Terminal convergence status.
    pub status: ConvergenceStatus,

    /// Optimization accounting.
    pub statistics: OptimizationStatistics,

    /// Immutable optimization history.
    pub history: Vec<OptimizationStep>,
}

impl OptimizationResult {
    /// Returns whether optimization converged.
    #[must_use]
    pub const fn converged(&self) -> bool {
        self.status.is_converged()
    }

    /// Validates the result contract.
    pub fn validate(&self) -> Result<()> {
        if self.parameters.is_empty() {
            return Err(
                AlgorithmError::internal_invariant(
                    "final_parameters_nonempty",
                    "optimization result cannot contain empty parameters",
                ),
            );
        }

        if self.best_parameters.is_empty() {
            return Err(
                AlgorithmError::internal_invariant(
                    "best_parameters_nonempty",
                    "best optimization parameters cannot be empty",
                ),
            );
        }

        if self.parameters.len()
            != self.best_parameters.len()
        {
            return Err(
                AlgorithmError::internal_invariant(
                    "optimization_parameter_dimensions",
                    "final and best parameter dimensions must match",
                ),
            );
        }

        if self.history.len() as u64
            != self.statistics.iterations
        {
            return Err(
                AlgorithmError::internal_invariant(
                    "optimization_history_length",
                    "history length must equal iteration count",
                ),
            );
        }

        for step in &self.history {
            step.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Optimizer trait
// =============================================================================

/// Backend-independent classical optimizer.
///
/// An optimizer consumes an objective and a validated initial parameter
/// vector. It does not know how the objective obtains its value.
pub trait Optimizer {
    /// Stable optimizer identifier.
    #[must_use]
    fn name(&self) -> &'static str;

    /// Performs optimization.
    fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        initial: ParameterVector,
        config: &OptimizationConfig,
    ) -> Result<OptimizationResult>;
}

// =============================================================================
// Gradient descent
// =============================================================================

/// Deterministic finite-difference gradient-descent optimizer.
///
/// This implementation intentionally performs no random sampling. It is
/// therefore suitable as the deterministic reference optimizer for the
/// algorithm subsystem.
///
/// Hardware-native parameter-shift differentiation belongs in a separate
/// optimizer implementation because it requires execution semantics that are
/// outside classical gradient descent.
#[derive(Debug, Clone, Copy, Default)]
pub struct GradientDescent;

impl GradientDescent {
    /// Creates a gradient-descent optimizer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Computes a central finite-difference gradient.
    ///
    /// Each parameter consumes two objective evaluations.
    fn numerical_gradient(
        objective: &mut dyn Objective,
        parameters: &ParameterVector,
        epsilon: f64,
        remaining_objective_evaluations: u64,
        gradient_evaluations: u64,
        max_gradient_evaluations: u64,
    ) -> Result<(Vec<f64>, u64)> {
        if gradient_evaluations
            >= max_gradient_evaluations
        {
            return Err(
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        AlgorithmResource::GradientEvaluations
                            .to_string(),
                    requested: gradient_evaluations
                        .saturating_add(1),
                    limit: max_gradient_evaluations,
                    message:
                        "maximum gradient evaluations reached"
                            .to_string(),
                },
            );
        }

        let parameter_count =
            parameters.len() as u64;

        let required_objective_evaluations =
            parameter_count
                .checked_mul(2)
                .ok_or_else(|| {
                    AlgorithmError::resource_limit_exceeded(
                        AlgorithmResource::ObjectiveEvaluations,
                        u128::MAX,
                        u128::from(
                            remaining_objective_evaluations,
                        ),
                        "gradient objective-evaluation count overflowed",
                    )
                })?;

        if required_objective_evaluations
            > remaining_objective_evaluations
        {
            return Err(
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        AlgorithmResource::ObjectiveEvaluations
                            .to_string(),
                    requested: required_objective_evaluations,
                    limit: remaining_objective_evaluations,
                    message:
                        "insufficient objective-evaluation budget for gradient"
                            .to_string(),
                },
            );
        }

        let mut gradient =
            Vec::with_capacity(parameters.len());

        let mut consumed = 0u64;

        for index in 0..parameters.len() {
            let base = parameters
                .get(index)
                .ok_or_else(|| {
                    AlgorithmError::internal_invariant(
                        "parameter_index_valid",
                        "parameter index disappeared during gradient evaluation",
                    )
                })?;

            let plus_value = base
                .checked_add(epsilon)
                .ok_or_else(|| {
                    AlgorithmError::NumericalInstability {
                        operation:
                            "finite_difference_plus_parameter"
                                .to_string(),
                        message:
                            "parameter plus epsilon overflowed"
                                .to_string(),
                    }
                })?;

            let minus_value = base
                .checked_sub(epsilon)
                .ok_or_else(|| {
                    AlgorithmError::NumericalInstability {
                        operation:
                            "finite_difference_minus_parameter"
                                .to_string(),
                        message:
                            "parameter minus epsilon overflowed"
                                .to_string(),
                    }
                })?;

            let mut plus =
                parameters.clone();

            let mut minus =
                parameters.clone();

            plus.set(index, plus_value)?;
            minus.set(index, minus_value)?;

            let plus_evaluation =
                objective.evaluate(&plus)?;

            let minus_evaluation =
                objective.evaluate(&minus)?;

            plus_evaluation.validate()?;
            minus_evaluation.validate()?;

            consumed = consumed
                .checked_add(2)
                .ok_or_else(|| {
                    AlgorithmError::resource_limit_exceeded(
                        AlgorithmResource::ObjectiveEvaluations,
                        u128::MAX,
                        u128::from(
                            remaining_objective_evaluations,
                        ),
                        "gradient objective-evaluation counter overflowed",
                    )
                })?;

            let numerator =
                plus_evaluation.value.get()
                    - minus_evaluation.value.get();

            let denominator = 2.0 * epsilon;

            let component =
                numerator / denominator;

            if !component.is_finite() {
                return Err(
                    AlgorithmError::NonFiniteValue {
                        field:
                            format!(
                                "gradient[{}]",
                                index
                            ),
                        value: component,
                    },
                );
            }

            gradient.push(component);
        }

        Ok((gradient, consumed))
    }
}

impl Optimizer for GradientDescent {
    fn name(&self) -> &'static str {
        "gradient_descent"
    }

    fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        initial: ParameterVector,
        config: &OptimizationConfig,
    ) -> Result<OptimizationResult> {
        config.validate()?;

        if initial.is_empty() {
            return Err(
                AlgorithmError::invalid_input(
                    "initial_parameters",
                    "optimizer requires at least one parameter",
                ),
            );
        }

        if initial.len() as u64
            > config.max_optimizer_steps
        {
            return Err(
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        AlgorithmResource::Parameters
                            .to_string(),
                    requested: initial.len() as u64,
                    limit: config.max_optimizer_steps,
                    message:
                        "initial parameter vector exceeds optimizer parameter budget"
                            .to_string(),
                },
            );
        }

        validate_parameter_magnitudes(
            &initial,
            config.max_parameter_magnitude,
        )?;

        let mut parameters = initial.clone();

        let initial_evaluation =
            objective.evaluate(&parameters)?;

        initial_evaluation.validate()?;

        let mut objective_evaluations = 1u64;

        let direction =
            objective.direction();

        let mut best_parameters =
            parameters.clone();

        let mut best_objective =
            initial_evaluation.value;

        let mut current_objective =
            initial_evaluation.value;

        let mut history =
            Vec::new();

        let mut optimizer_steps = 0u64;
        let mut gradient_evaluations = 0u64;
        let mut gradient_components = 0u64;
        let mut stagnant_iterations = 0u64;

        for iteration in 1..=config.max_iterations {
            if objective_evaluations
                >= config.max_objective_evaluations
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::MaxEvaluations,
                    OptimizationStatistics {
                        iterations:
                            iteration.saturating_sub(1),
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            if optimizer_steps
                >= config.max_optimizer_steps
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::MaxOptimizerSteps,
                    OptimizationStatistics {
                        iterations:
                            iteration.saturating_sub(1),
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            let remaining_objective_evaluations =
                config
                    .max_objective_evaluations
                    .saturating_sub(
                        objective_evaluations,
                    );

            let (
                gradient,
                consumed_objective_evaluations,
            ) =
                match Self::numerical_gradient(
                    objective,
                    &parameters,
                    config.gradient_epsilon,
                    remaining_objective_evaluations,
                    gradient_evaluations,
                    config.max_gradient_evaluations,
                ) {
                    Ok(value) => value,

                    Err(error) => {
                        if matches!(
                            error,
                            AlgorithmError::ResourceLimitExceeded {
                                resource,
                                ..
                            } if resource
                                == AlgorithmResource::ObjectiveEvaluations
                                    .to_string()
                        ) {
                            return Ok(build_result(
                                parameters,
                                current_objective,
                                best_parameters,
                                best_objective,
                                ConvergenceStatus::MaxEvaluations,
                                OptimizationStatistics {
                                    iterations:
                                        iteration
                                            .saturating_sub(1),
                                    optimizer_steps,
                                    objective_evaluations,
                                    gradient_evaluations,
                                    gradient_components,
                                },
                                history,
                            ));
                        }

                        return Err(error);
                    }
                };

            objective_evaluations =
                objective_evaluations
                    .checked_add(
                        consumed_objective_evaluations,
                    )
                    .ok_or_else(|| {
                        AlgorithmError::resource_limit_exceeded(
                            AlgorithmResource::ObjectiveEvaluations,
                            u128::MAX,
                            u128::from(
                                config
                                    .max_objective_evaluations,
                            ),
                            "objective evaluation counter overflowed",
                        )
                    })?;

            gradient_evaluations =
                gradient_evaluations
                    .checked_add(1)
                    .ok_or_else(|| {
                        AlgorithmError::resource_limit_exceeded(
                            AlgorithmResource::GradientEvaluations,
                            u128::MAX,
                            u128::from(
                                config
                                    .max_gradient_evaluations,
                            ),
                            "gradient evaluation counter overflowed",
                        )
                    })?;

            gradient_components =
                gradient_components
                    .checked_add(
                        gradient.len() as u64,
                    )
                    .ok_or_else(|| {
                        AlgorithmError::resource_limit_exceeded(
                            AlgorithmResource::Parameters,
                            u128::MAX,
                            u128::from(
                                config
                                    .max_optimizer_steps,
                            ),
                            "gradient component counter overflowed",
                        )
                    })?;

            let gradient_norm =
                l2_norm(&gradient)?;

            let max_gradient_component =
                max_abs(&gradient)?;

            if gradient_norm
                <= config.tolerance
            {
                let step =
                    OptimizationStep {
                        iteration,
                        evaluation:
                            initial_evaluation.clone(),
                        gradient_norm,
                        max_gradient_component,
                        max_parameter_change: 0.0,
                        objective_improvement: 0.0,
                    };

                history.push(step);

                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::Converged,
                    OptimizationStatistics {
                        iterations: iteration,
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            let mut next_parameters =
                parameters.clone();

            let mut max_parameter_change =
                0.0f64;

            for (index, gradient_value) in
                gradient.iter().copied().enumerate()
            {
                let current =
                    parameters
                        .get(index)
                        .ok_or_else(|| {
                            AlgorithmError::internal_invariant(
                                "parameter_index_valid",
                                "parameter index disappeared during optimizer update",
                            )
                        })?;

                let update =
                    config.learning_rate
                        * gradient_value;

                if !update.is_finite() {
                    return Err(
                        AlgorithmError::NonFiniteValue {
                            field:
                                format!(
                                    "parameter_update[{}]",
                                    index
                                ),
                            value: update,
                        },
                    );
                }

                let next =
                    match direction {
                        ObjectiveDirection::Minimize => {
                            current - update
                        }
                        ObjectiveDirection::Maximize => {
                            current + update
                        }
                    };

                if !next.is_finite() {
                    return Err(
                        AlgorithmError::NonFiniteValue {
                            field:
                                format!(
                                    "parameter[{}]",
                                    index
                                ),
                            value: next,
                        },
                    );
                }

                if next.abs()
                    > config.max_parameter_magnitude
                {
                    return Err(
                        AlgorithmError::ResourceLimitExceeded {
                            resource:
                                AlgorithmResource::Parameters
                                    .to_string(),
                            requested:
                                next.abs()
                                    as u64,
                            limit:
                                config
                                    .max_parameter_magnitude
                                    as u64,
                            message:
                                format!(
                                    "parameter {} exceeds configured magnitude limit",
                                    index
                                ),
                        },
                    );
                }

                max_parameter_change =
                    max_parameter_change.max(
                        (next - current).abs(),
                    );

                next_parameters.set(
                    index,
                    next,
                )?;
            }

            validate_parameter_magnitudes(
                &next_parameters,
                config.max_parameter_magnitude,
            )?;

            let next_evaluation =
                objective.evaluate(
                    &next_parameters,
                )?;

            next_evaluation.validate()?;

            objective_evaluations =
                objective_evaluations
                    .checked_add(1)
                    .ok_or_else(|| {
                        AlgorithmError::resource_limit_exceeded(
                            AlgorithmResource::ObjectiveEvaluations,
                            u128::MAX,
                            u128::from(
                                config
                                    .max_objective_evaluations,
                            ),
                            "objective evaluation counter overflowed",
                        )
                    })?;

            let improvement =
                direction.improvement(
                    next_evaluation.value,
                    current_objective,
                );

            if !improvement.is_finite() {
                return Err(
                    AlgorithmError::NonFiniteValue {
                        field:
                            "objective_improvement"
                                .to_string(),
                        value: improvement,
                    },
                );
            }

            if improvement
                > config.objective_improvement_tolerance
            {
                stagnant_iterations = 0;
            } else {
                stagnant_iterations =
                    stagnant_iterations
                        .saturating_add(1);
            }

            if direction.is_better(
                next_evaluation.value,
                best_objective,
            ) {
                best_objective =
                    next_evaluation.value;
                best_parameters =
                    next_parameters.clone();
            }

            let step =
                OptimizationStep {
                    iteration,
                    evaluation:
                        next_evaluation.clone(),
                    gradient_norm,
                    max_gradient_component,
                    max_parameter_change,
                    objective_improvement:
                        improvement,
                };

            step.validate()?;

            history.push(step);

            parameters =
                next_parameters;

            current_objective =
                next_evaluation.value;

            optimizer_steps =
                optimizer_steps
                    .checked_add(1)
                    .ok_or_else(|| {
                        AlgorithmError::resource_limit_exceeded(
                            AlgorithmResource::OptimizerSteps,
                            u128::MAX,
                            u128::from(
                                config
                                    .max_optimizer_steps,
                            ),
                            "optimizer step counter overflowed",
                        )
                    })?;

            if max_parameter_change
                <= config.tolerance
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::Converged,
                    OptimizationStatistics {
                        iterations: iteration,
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            if config.patience > 0
                && stagnant_iterations
                    >= config.patience
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::Stagnated,
                    OptimizationStatistics {
                        iterations: iteration,
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            if objective_evaluations
                >= config.max_objective_evaluations
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::MaxEvaluations,
                    OptimizationStatistics {
                        iterations: iteration,
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }

            if optimizer_steps
                >= config.max_optimizer_steps
            {
                return Ok(build_result(
                    parameters,
                    current_objective,
                    best_parameters,
                    best_objective,
                    ConvergenceStatus::MaxOptimizerSteps,
                    OptimizationStatistics {
                        iterations: iteration,
                        optimizer_steps,
                        objective_evaluations,
                        gradient_evaluations,
                        gradient_components,
                    },
                    history,
                ));
            }
        }

        Ok(build_result(
            parameters,
            current_objective,
            best_parameters,
            best_objective,
            ConvergenceStatus::MaxIterations,
            OptimizationStatistics {
                iterations: config.max_iterations,
                optimizer_steps,
                objective_evaluations,
                gradient_evaluations,
                gradient_components,
            },
            history,
        ))
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn build_result(
    parameters: ParameterVector,
    objective: super::types::ObjectiveValue,
    best_parameters: ParameterVector,
    best_objective: super::types::ObjectiveValue,
    status: ConvergenceStatus,
    statistics: OptimizationStatistics,
    history: Vec<OptimizationStep>,
) -> OptimizationResult {
    OptimizationResult {
        parameters,
        objective,
        best_objective,
        best_parameters,
        status,
        statistics,
        history,
    }
}

fn validate_finite(
    value: f64,
    field: &str,
) -> Result<()> {
    if !value.is_finite() {
        return Err(
            AlgorithmError::NonFiniteValue {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(())
}

fn validate_positive_finite(
    value: f64,
    field: &str,
) -> Result<()> {
    validate_finite(value, field)?;

    if value <= 0.0 {
        return Err(
            AlgorithmError::invalid_configuration(
                field,
                format!(
                    "{} must be greater than zero",
                    field
                ),
            ),
        );
    }

    Ok(())
}

fn validate_parameter_magnitudes(
    parameters: &ParameterVector,
    maximum: f64,
) -> Result<()> {
    for index in 0..parameters.len() {
        let value =
            parameters
                .get(index)
                .ok_or_else(|| {
                    AlgorithmError::internal_invariant(
                        "parameter_index_valid",
                        "parameter index was invalid during validation",
                    )
                })?;

        validate_finite(
            value,
            &format!("parameter[{}]", index),
        )?;

        if value.abs() > maximum {
            return Err(
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        AlgorithmResource::Parameters
                            .to_string(),
                    requested:
                        value.abs() as u64,
                    limit:
                        maximum as u64,
                    message:
                        format!(
                            "parameter {} exceeds maximum magnitude {}",
                            index,
                            maximum
                        ),
                },
            );
        }
    }

    Ok(())
}

fn l2_norm(
    values: &[f64],
) -> Result<f64> {
    let mut sum = 0.0f64;

    for value in values {
        validate_finite(
            *value,
            "gradient_component",
        )?;

        sum += value * value;

        if !sum.is_finite() {
            return Err(
                AlgorithmError::NumericalInstability {
                    operation:
                        "gradient_norm".to_string(),
                    message:
                        "gradient norm overflowed"
                            .to_string(),
                },
            );
        }
    }

    let norm = sum.sqrt();

    validate_finite(
        norm,
        "gradient_norm",
    )?;

    Ok(norm)
}

fn max_abs(
    values: &[f64],
) -> Result<f64> {
    let mut maximum = 0.0f64;

    for value in values {
        validate_finite(
            *value,
            "gradient_component",
        )?;

        maximum =
            maximum.max(value.abs());
    }

    Ok(maximum)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::algorithms::types::{
        ObjectiveValue,
        ParameterVector,
    };

    struct Quadratic;

    impl Objective for Quadratic {
        fn evaluate(
            &mut self,
            parameters: &ParameterVector,
        ) -> Result<ObjectiveEvaluation> {
            let x = parameters
                .get(0)
                .ok_or_else(|| {
                    AlgorithmError::invalid_input(
                        "parameters",
                        "quadratic objective requires parameter 0",
                    )
                })?;

            let value =
                ObjectiveValue::new(
                    (x - 2.0).powi(2),
                )?;

            ObjectiveEvaluation::classical(
                value,
                1,
            )
        }

        fn direction(
            &self,
        ) -> ObjectiveDirection {
            ObjectiveDirection::Minimize
        }
    }

    struct MaximizationQuadratic;

    impl Objective for MaximizationQuadratic {
        fn evaluate(
            &mut self,
            parameters: &ParameterVector,
        ) -> Result<ObjectiveEvaluation> {
            let x = parameters
                .get(0)
                .ok_or_else(|| {
                    AlgorithmError::invalid_input(
                        "parameters",
                        "objective requires parameter 0",
                    )
                })?;

            let value =
                ObjectiveValue::new(
                    -((x - 2.0).powi(2)),
                )?;

            ObjectiveEvaluation::classical(
                value,
                1,
            )
        }

        fn direction(
            &self,
        ) -> ObjectiveDirection {
            ObjectiveDirection::Maximize
        }
    }

    #[test]
    fn default_configuration_is_valid() {
        assert!(
            OptimizationConfig::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn zero_iterations_are_rejected() {
        let config =
            OptimizationConfig {
                max_iterations: 0,
                ..OptimizationConfig::default()
            };

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn non_finite_learning_rate_is_rejected() {
        let config =
            OptimizationConfig {
                learning_rate: f64::NAN,
                ..OptimizationConfig::default()
            };

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn gradient_descent_minimizes_quadratic() {
        let parameters =
            ParameterVector::new(vec![0.0])
                .unwrap();

        let config =
            OptimizationConfig {
                learning_rate: 0.1,
                tolerance: 1.0e-7,
                max_iterations: 500,
                gradient_epsilon: 1.0e-6,
                patience: 50,
                ..OptimizationConfig::default()
            };

        let mut objective =
            Quadratic;

        let mut optimizer =
            GradientDescent::new();

        let result =
            optimizer
                .optimize(
                    &mut objective,
                    parameters,
                    &config,
                )
                .unwrap();

        let x =
            result.parameters
                .get(0)
                .unwrap();

        assert!(
            (x - 2.0).abs() < 1.0e-3
        );

        assert!(
            result.objective.get()
                < 1.0e-5
        );

        assert!(
            !result.history.is_empty()
        );

        assert!(
            result.validate().is_ok()
        );
    }

    #[test]
    fn gradient_descent_respects_max_parameter_magnitude() {
        let parameters =
            ParameterVector::new(vec![
                1.0e6
            ])
            .unwrap();

        let config =
            OptimizationConfig {
                max_parameter_magnitude: 10.0,
                ..OptimizationConfig::default()
            };

        let mut objective =
            Quadratic;

        let mut optimizer =
            GradientDescent::new();

        assert!(
            optimizer
                .optimize(
                    &mut objective,
                    parameters,
                    &config,
                )
                .is_err()
        );
    }

    #[test]
    fn gradient_descent_supports_maximization() {
        let parameters =
            ParameterVector::new(vec![0.0])
                .unwrap();

        let config =
            OptimizationConfig {
                learning_rate: 0.1,
                tolerance: 1.0e-7,
                max_iterations: 500,
                gradient_epsilon: 1.0e-6,
                patience: 50,
                ..OptimizationConfig::default()
            };

        let mut objective =
            MaximizationQuadratic;

        let mut optimizer =
            GradientDescent::new();

        let result =
            optimizer
                .optimize(
                    &mut objective,
                    parameters,
                    &config,
                )
                .unwrap();

        let x =
            result.parameters
                .get(0)
                .unwrap();

        assert!(
            (x - 2.0).abs() < 1.0e-3
        );
    }

    #[test]
    fn optimizer_is_deterministic() {
        let parameters =
            ParameterVector::new(vec![0.0])
                .unwrap();

        let config =
            OptimizationConfig {
                max_iterations: 100,
                ..OptimizationConfig::default()
            };

        let mut objective_a =
            Quadratic;

        let mut objective_b =
            Quadratic;

        let mut optimizer_a =
            GradientDescent::new();

        let mut optimizer_b =
            GradientDescent::new();

        let result_a =
            optimizer_a
                .optimize(
                    &mut objective_a,
                    parameters.clone(),
                    &config,
                )
                .unwrap();

        let result_b =
            optimizer_b
                .optimize(
                    &mut objective_b,
                    parameters,
                    &config,
                )
                .unwrap();

        assert_eq!(
            result_a,
            result_b
        );
    }

    #[test]
    fn convergence_status_is_machine_readable() {
        assert_eq!(
            ConvergenceStatus::Converged
                .as_str(),
            "converged"
        );

        assert!(
            ConvergenceStatus::Converged
                .is_converged()
        );

        assert!(
            !ConvergenceStatus::Stagnated
                .is_failure()
        );

        assert!(
            ConvergenceStatus::Diverged
                .is_failure()
        );
    }
}