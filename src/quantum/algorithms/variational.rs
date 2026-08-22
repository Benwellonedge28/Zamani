//! Zamani Quantum Algorithms — Generic Variational Orchestration.
//!
//! This module is the stable orchestration layer shared by variational quantum
//! algorithms. It deliberately does not implement an optimizer, objective,
//! backend, hardware adapter, routing pass, or error-correction mechanism.
//!
//! # Responsibility
//!
//! `variational.rs` owns only:
//!
//! - the generic variational ansatz contract;
//! - immutable variational problem validation;
//! - variational orchestration configuration;
//! - algorithm metadata and result assembly;
//! - convergence-policy enforcement at the VQA boundary;
//! - compatibility aliases for the former public variational API.
//!
//! The canonical implementations remain in:
//!
//! ```text
//! error.rs      -> AlgorithmError / Result
//! types.rs      -> ParameterVector / metadata / limits
//! objective.rs  -> Objective / ObjectiveEvaluation
//! optimizer.rs  -> OptimizationConfig / Optimizer / OptimizationResult
//! execution.rs  -> QuantumExecutor / ExecutionRequest / ExecutionResult
//! quantum::ir   -> QuantumCircuit and circuit semantics
//! ```
//!
//! # Architectural boundary
//!
//! ```text
//! VariationalProblem
//!        │
//!        ├── Ansatz ───────────────► quantum::ir::QuantumCircuit
//!        │
//!        └── ParameterVector
//!                  │
//!                  ▼
//!              Objective
//!                  │
//!                  ▼
//!              Optimizer
//!                  │
//!                  ▼
//!        OptimizationResult
//!                  │
//!                  ▼
//!          VariationalResult
//! ```
//!
//! A quantum objective is responsible for connecting its parameter vector to
//! an ansatz and, where appropriate, to `execution::QuantumExecutor`. This
//! module does not duplicate that execution path.
//!
//! # Important integration rule
//!
//! Algorithm-specific ansatz traits may wrap [`Ansatz`] when they need richer
//! domain semantics. For example, VQE may require a Hamiltonian-specific
//! contract. Such specialized traits must not redefine `ParameterVector` or
//! optimizer/result types.
//!
//! # Determinism
//!
//! This module creates no randomness. Determinism belongs to the explicit
//! execution configuration and optimizer/objective contracts. The generic
//! orchestration layer only preserves and reports those contracts.
//!
//! # Numerical safety
//!
//! Parameter validation is delegated to `ParameterVector`. Objective values
//! are validated by `ObjectiveEvaluation`, and optimizer results are validated
//! by `OptimizationResult`. No raw unchecked floating-point result is exposed
//! by this module.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1. No nightly features and no external dependencies are required.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::error::{AlgorithmError, Result};

pub use super::objective::{
    Objective,
    ObjectiveDirection,
    ObjectiveStatistics,
};

pub use super::optimizer::{
    ConvergenceStatus,
    GradientDescent,
    OptimizationConfig,
    OptimizationResult,
    OptimizationStatistics,
    OptimizationStep,
    Optimizer,
};

use super::types::{
    AlgorithmId,
    AlgorithmMetadata,
    AlgorithmVersion,
    ParameterVector,
};

/// Backward-compatible name for the canonical parameter vector.
///
/// New code should use [`ParameterVector`] from `types.rs` directly.
pub type Parameters = ParameterVector;

/// Stable semantic version of the generic variational orchestration contract.
pub const VARIATIONAL_VERSION: AlgorithmVersion =
    AlgorithmVersion::new(1, 0, 0);

// =============================================================================
// Ansatz
// =============================================================================

/// Backend-independent parameterized quantum ansatz.
///
/// The ansatz converts concrete classical values into a logical Quantum IR
/// circuit. It never executes the circuit and therefore cannot depend on a
/// simulator, QPU, hardware topology, credentials, or transport.
pub trait Ansatz {
    /// Returns the number of classical parameters accepted by this ansatz.
    fn parameter_count(&self) -> Result<usize>;

    /// Builds a logical Quantum IR circuit from validated parameters.
    fn build(&self, parameters: &ParameterVector) -> Result<QuantumCircuit>;

    /// Performs ansatz-local validation independent of a parameter vector.
    fn validate(&self) -> Result<()> {
        let parameter_count = self.parameter_count()?;

        if parameter_count == 0 {
            return Err(AlgorithmError::invalid_configuration(
                "parameter_count",
                "a variational ansatz must expose at least one parameter",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Variational problem
// =============================================================================

/// Immutable, validated input to generic variational orchestration.
///
/// The problem owns the ansatz and initial parameters. Optimization policy is
/// intentionally kept in [`VariationalConfig`] so the same problem can be
/// executed with different optimizers without mutating the mathematical input.
pub struct VariationalProblem<A> {
    ansatz: A,
    initial_parameters: ParameterVector,
}

impl<A> fmt::Debug for VariationalProblem<A>
where
    A: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VariationalProblem")
            .field("ansatz", &self.ansatz)
            .field("initial_parameters", &self.initial_parameters)
            .finish()
    }
}

impl<A> VariationalProblem<A>
where
    A: Ansatz,
{
    /// Creates and fully validates a variational problem.
    pub fn new(
        ansatz: A,
        initial_parameters: ParameterVector,
    ) -> Result<Self> {
        ansatz.validate()?;
        initial_parameters.validate()?;
        initial_parameters.require_non_empty()?;

        let expected = ansatz.parameter_count()?;
        let actual = initial_parameters.len();

        if expected != actual {
            return Err(AlgorithmError::DimensionMismatch {
                expected_name: "ansatz parameter count".to_string(),
                expected,
                actual_name: "initial parameter count".to_string(),
                actual,
                message:
                    "variational ansatz and initial parameter vector \
                     dimensions must match"
                        .to_string(),
            });
        }

        Ok(Self {
            ansatz,
            initial_parameters,
        })
    }

    /// Returns the immutable ansatz.
    #[must_use]
    pub fn ansatz(&self) -> &A {
        &self.ansatz
    }

    /// Returns the immutable initial parameters.
    #[must_use]
    pub fn initial_parameters(&self) -> &ParameterVector {
        &self.initial_parameters
    }

    /// Returns the validated number of classical parameters.
    pub fn parameter_count(&self) -> Result<usize> {
        self.ansatz.parameter_count()
    }

    /// Builds the initial logical circuit without executing it.
    ///
    /// This is the explicit integration point between generic variational
    /// orchestration and the canonical Quantum IR.
    pub fn build_initial_circuit(&self) -> Result<QuantumCircuit> {
        self.ansatz.build(&self.initial_parameters)
    }
}

// =============================================================================
// Variational configuration
// =============================================================================

/// Policy controlling one generic variational optimization invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct VariationalConfig {
    /// Canonical classical optimization policy.
    pub optimization: OptimizationConfig,

    /// If true, a non-converged bounded result is returned as an error rather
    /// than as a successful result carrying a non-converged status.
    pub require_convergence: bool,
}

impl Default for VariationalConfig {
    fn default() -> Self {
        Self {
            optimization: OptimizationConfig::default(),
            require_convergence: false,
        }
    }
}

impl VariationalConfig {
    /// Validates the complete variational policy.
    pub fn validate(&self) -> Result<()> {
        self.optimization.validate()
    }
}

// =============================================================================
// Variational statistics
// =============================================================================

/// Immutable accounting for a complete variational invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariationalStatistics {
    /// Optimizer statistics.
    pub optimization: OptimizationStatistics,

    /// Objective-layer accounting.
    pub objective: ObjectiveStatistics,
}

impl VariationalStatistics {
    fn from_run(
        optimization: &OptimizationResult,
        objective: ObjectiveStatistics,
    ) -> Self {
        Self {
            optimization: optimization.statistics.clone(),
            objective,
        }
    }
}

// =============================================================================
// Variational result
// =============================================================================

/// Complete, validated result of one generic variational invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct VariationalResult {
    /// Stable algorithm metadata.
    pub metadata: AlgorithmMetadata,

    /// Direction used by the objective.
    pub direction: ObjectiveDirection,

    /// Canonical optimization result.
    pub optimization: OptimizationResult,

    /// Combined optimizer/objective accounting.
    pub statistics: VariationalStatistics,
}

impl VariationalResult {
    /// Returns the terminal convergence status.
    #[must_use]
    pub const fn status(&self) -> ConvergenceStatus {
        self.optimization.status
    }

    /// Returns whether the optimizer converged.
    #[must_use]
    pub const fn converged(&self) -> bool {
        self.optimization.converged()
    }

    /// Returns the final parameter vector.
    #[must_use]
    pub fn parameters(&self) -> &ParameterVector {
        &self.optimization.parameters
    }

    /// Returns the best parameter vector.
    #[must_use]
    pub fn best_parameters(&self) -> &ParameterVector {
        &self.optimization.best_parameters
    }

    /// Returns the final objective value.
    #[must_use]
    pub fn objective(&self) -> super::types::ObjectiveValue {
        self.optimization.objective
    }

    /// Returns the best objective value observed.
    #[must_use]
    pub fn best_objective(&self) -> super::types::ObjectiveValue {
        self.optimization.best_objective
    }

    /// Validates the complete result contract.
    pub fn validate(&self) -> Result<()> {
        if self.metadata.algorithm != AlgorithmId::Variational {
            return Err(AlgorithmError::internal_invariant(
                "variational_result_algorithm_id",
                "generic variational result metadata must identify \
                 the variational algorithm",
            ));
        }

        if self.metadata.version != VARIATIONAL_VERSION {
            return Err(AlgorithmError::internal_invariant(
                "variational_result_version",
                "generic variational result metadata contains an \
                 unexpected version",
            ));
        }

        self.optimization.validate()?;

        if self.statistics.optimization
            != self.optimization.statistics
        {
            return Err(AlgorithmError::internal_invariant(
                "variational_statistics_optimization",
                "variational statistics must match optimizer statistics",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Variational algorithm
// =============================================================================

/// Generic backend-independent variational algorithm orchestrator.
///
/// The optimizer is injected, which makes the orchestration independent from
/// any particular optimization strategy and allows deterministic reference
/// testing with [`GradientDescent`].
#[derive(Debug, Clone)]
pub struct VariationalAlgorithm<O = GradientDescent> {
    optimizer: O,
    config: VariationalConfig,
    metadata: AlgorithmMetadata,
}

impl Default for VariationalAlgorithm<GradientDescent> {
    fn default() -> Self {
        Self {
            optimizer: GradientDescent::new(),
            config: VariationalConfig::default(),
            metadata: AlgorithmMetadata::new(
                AlgorithmId::Variational,
                VARIATIONAL_VERSION,
            ),
        }
    }
}

impl<O> VariationalAlgorithm<O>
where
    O: Optimizer,
{
    /// Creates a validated variational orchestrator.
    pub fn new(
        optimizer: O,
        config: VariationalConfig,
    ) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            optimizer,
            config,
            metadata: AlgorithmMetadata::new(
                AlgorithmId::Variational,
                VARIATIONAL_VERSION,
            ),
        })
    }

    /// Associates an implementation identifier with result metadata.
    pub fn with_implementation<S: Into<String>>(
        mut self,
        implementation: S,
    ) -> Result<Self> {
        self.metadata = self
            .metadata
            .with_implementation(implementation)?;

        Ok(self)
    }

    /// Returns the immutable variational configuration.
    #[must_use]
    pub const fn config(&self) -> &VariationalConfig {
        &self.config
    }

    /// Returns the optimizer's stable name.
    #[must_use]
    pub fn optimizer_name(&self) -> &'static str {
        self.optimizer.name()
    }

    /// Optimizes an already validated initial parameter vector.
    ///
    /// This is the low-level integration point used by specialized algorithms
    /// such as VQE and QAOA. The objective remains responsible for converting
    /// parameters into quantum execution when required.
    pub fn optimize(
        &mut self,
        objective: &mut dyn Objective,
        initial: ParameterVector,
    ) -> Result<VariationalResult> {
        initial.validate()?;
        initial.require_non_empty()?;

        let direction = objective.direction();

        let optimization = self.optimizer.optimize(
            objective,
            initial,
            &self.config.optimization,
        )?;

        optimization.validate()?;

        let objective_statistics = objective.statistics();

        let result = VariationalResult {
            metadata: self.metadata.clone(),
            direction,
            statistics: VariationalStatistics::from_run(
                &optimization,
                objective_statistics,
            ),
            optimization,
        };

        result.validate()?;

        if self.config.require_convergence
            && !result.converged()
        {
            return Err(AlgorithmError::convergence_failure(
                AlgorithmId::Variational.as_str(),
                None,
                format!(
                    "variational optimization terminated with status {}",
                    result.status()
                ),
            ));
        }

        Ok(result)
    }

    /// Solves a validated variational problem using the supplied objective.
    ///
    /// The ansatz is validated by `VariationalProblem`. The objective remains
    /// the execution-aware component, so this method does not execute the
    /// initial circuit merely to validate the problem.
    pub fn solve<A>(
        &mut self,
        problem: &VariationalProblem<A>,
        objective: &mut dyn Objective,
    ) -> Result<VariationalResult>
    where
        A: Ansatz,
    {
        let expected = problem.parameter_count()?;
        let actual = problem.initial_parameters().len();

        if expected != actual {
            return Err(AlgorithmError::DimensionMismatch {
                expected_name:
                    "ansatz parameter count".to_string(),
                expected,
                actual_name:
                    "initial parameter count".to_string(),
                actual,
                message:
                    "variational problem dimensions changed after validation"
                        .to_string(),
            });
        }

        self.optimize(
            objective,
            problem.initial_parameters().clone(),
        )
    }
}

// =============================================================================
// Compatibility alias
// =============================================================================

/// Backward-compatible name for [`VariationalAlgorithm`].
///
/// Existing callers can migrate without retaining a second implementation.
pub type VariationalOptimizer<O = GradientDescent> =
    VariationalAlgorithm<O>;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct QuadraticObjective;

    impl Objective for QuadraticObjective {
        fn evaluate(
            &mut self,
            parameters: &ParameterVector,
        ) -> Result<
            super::super::objective::ObjectiveEvaluation,
        > {
            let x = parameters.get(0).ok_or_else(|| {
                AlgorithmError::invalid_input(
                    "parameters",
                    "quadratic objective requires one parameter",
                )
            })?;

            let value =
                super::super::types::ObjectiveValue::new(
                    (x - 2.0).powi(2),
                )?;

            super::super::objective::ObjectiveEvaluation::classical(
                value,
                1,
            )
        }
    }

    #[derive(Debug)]
    struct OneParameterAnsatz;

    impl Ansatz for OneParameterAnsatz {
        fn parameter_count(&self) -> Result<usize> {
            Ok(1)
        }

        fn build(
            &self,
            _parameters: &ParameterVector,
        ) -> Result<QuantumCircuit> {
            Err(
                AlgorithmError::UnsupportedOperation {
                    operation:
                        "test_ansatz_build".to_string(),
                    message:
                        "the unit test does not require circuit construction"
                            .to_string(),
                },
            )
        }
    }

    #[test]
    fn problem_rejects_dimension_mismatch() {
        let result = VariationalProblem::new(
            OneParameterAnsatz,
            ParameterVector::new(vec![0.0, 1.0])
                .unwrap(),
        );

        assert!(matches!(
            result,
            Err(
                AlgorithmError::DimensionMismatch { .. }
            )
        ));
    }

    #[test]
    fn problem_rejects_empty_parameters() {
        let result = VariationalProblem::new(
            OneParameterAnsatz,
            ParameterVector::new(Vec::new())
                .unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_gradient_descent_is_reproducible() {
        let parameters =
            ParameterVector::new(vec![0.0])
                .unwrap();

        let config = VariationalConfig {
            optimization: OptimizationConfig {
                learning_rate: 0.1,
                tolerance: 1.0e-7,
                max_iterations: 500,
                gradient_epsilon: 1.0e-6,
                patience: 50,
                objective_improvement_tolerance:
                    1.0e-10,
                max_parameter_magnitude: 1.0e6,
                max_objective_evaluations: 10_000,
                max_gradient_evaluations: 1_000,
                max_optimizer_steps: 1_000,
                min_learning_rate: 1.0e-15,
            },
            require_convergence: true,
        };

        let mut first =
            VariationalAlgorithm::new(
                GradientDescent::new(),
                config.clone(),
            )
            .unwrap();

        let mut second =
            VariationalAlgorithm::new(
                GradientDescent::new(),
                config,
            )
            .unwrap();

        let mut objective_one =
            QuadraticObjective;

        let mut objective_two =
            QuadraticObjective;

        let result_one = first
            .optimize(
                &mut objective_one,
                parameters.clone(),
            )
            .unwrap();

        let result_two = second
            .optimize(
                &mut objective_two,
                parameters,
            )
            .unwrap();

        assert_eq!(result_one, result_two);

        assert!(
            (result_one
                .best_parameters()
                .get(0)
                .unwrap()
                - 2.0)
                .abs()
                < 1.0e-3
        );

        assert!(
            result_one.best_objective().get()
                < 1.0e-5
        );

        assert!(result_one.converged());
    }

    #[test]
    fn non_convergence_can_be_required() {
        let config = VariationalConfig {
            optimization: OptimizationConfig {
                max_iterations: 1,
                ..OptimizationConfig::default()
            },
            require_convergence: true,
        };

        let mut algorithm =
            VariationalAlgorithm::new(
                GradientDescent::new(),
                config,
            )
            .unwrap();

        let mut objective =
            QuadraticObjective;

        let result = algorithm.optimize(
            &mut objective,
            ParameterVector::new(vec![0.0])
                .unwrap(),
        );

        assert!(matches!(
            result,
            Err(
                AlgorithmError::ConvergenceFailure { .. }
            )
        ));
    }

    #[test]
    fn result_contract_is_self_consistent() {
        let config = VariationalConfig {
            optimization: OptimizationConfig {
                learning_rate: 0.1,
                max_iterations: 2,
                max_objective_evaluations: 100,
                max_gradient_evaluations: 10,
                max_optimizer_steps: 10,
                ..OptimizationConfig::default()
            },
            require_convergence: false,
        };

        let mut algorithm =
            VariationalAlgorithm::new(
                GradientDescent::new(),
                config,
            )
            .unwrap();

        let mut objective =
            QuadraticObjective;

        let result = algorithm
            .optimize(
                &mut objective,
                ParameterVector::new(vec![0.0])
                    .unwrap(),
            )
            .unwrap();

        result.validate().unwrap();

        assert_eq!(
            result.statistics.optimization,
            result.optimization.statistics
        );

        assert_eq!(
            result.metadata.algorithm,
            AlgorithmId::Variational
        );

        assert_eq!(
            result.metadata.version,
            VARIATIONAL_VERSION
        );
    }
}