//! Cross-module integration tests for `quantum::algorithms`.
//!
//! These tests verify that the production contracts compose correctly across
//! module boundaries without introducing a simulator, QPU, hardware vendor,
//! routing implementation, or transpiler dependency.
//!
//! The integration boundary covered here is:
//!
//! ```text
//! algorithms::types::ParameterVector
//!             │
//!             ▼
//! algorithms::objective::ClassicalObjective
//!             │
//!             ▼
//! algorithms::optimizer::GradientDescent
//!             │
//!             ▼
//! algorithms::optimizer::OptimizationResult
//! ```
//!
//! This is intentionally narrower than the execution integration boundary.
//! Quantum backend integration belongs in the execution/integration layer once
//! a concrete deterministic executor exists. These tests must remain runnable
//! on every supported build machine without quantum hardware.
//!
//! # Ownership
//!
//! This file owns cross-module contract verification only. It does not define
//! production behavior or duplicate validation logic from the modules under
//! test.
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

use super::super::error::AlgorithmError;
use super::super::objective::{
    ClassicalObjective,
    Objective,
    ObjectiveDirection,
};
use super::super::optimizer::{
    ConvergenceStatus,
    GradientDescent,
    OptimizationConfig,
    Optimizer,
};
use super::super::types::{
    ObjectiveValue,
    ParameterVector,
};

/// Deterministic scalar objective used to exercise the complete classical
/// algorithm pipeline.
///
/// f(x) = (x - 2)^2
fn quadratic_objective() -> ClassicalObjective {
    ClassicalObjective::new(|parameters| {
        let x = parameters
            .get(0)
            .ok_or_else(|| {
                AlgorithmError::invalid_input(
                    "parameters",
                    "quadratic integration objective requires parameter 0",
                )
            })?;

        ObjectiveValue::new((x - 2.0).powi(2))
    })
}

/// Deterministic optimization configuration deliberately kept small enough
/// for a fast integration test while leaving sufficient budget for the
/// finite-difference gradient implementation.
fn integration_config() -> OptimizationConfig {
    OptimizationConfig {
        learning_rate: 0.1,
        tolerance: 1.0e-7,
        max_iterations: 200,
        gradient_epsilon: 1.0e-6,
        patience: 25,
        objective_improvement_tolerance: 1.0e-12,
        max_parameter_magnitude: 1.0e6,
        max_objective_evaluations: 2_000,
        max_gradient_evaluations: 500,
        max_optimizer_steps: 500,
        min_learning_rate: 1.0e-15,
    }
}

#[test]
fn parameter_objective_optimizer_result_pipeline_composes() {
    let initial = ParameterVector::new(vec![0.0])
        .expect("integration parameters must be valid");

    let config = integration_config();

    config
        .validate()
        .expect("integration optimizer configuration must be valid");

    let mut objective = quadratic_objective();
    let mut optimizer = GradientDescent::new();

    assert_eq!(optimizer.name(), "gradient_descent");
    assert_eq!(objective.direction(), ObjectiveDirection::Minimize);

    let result = optimizer
        .optimize(&mut objective, initial, &config)
        .expect("complete algorithm pipeline must succeed");

    result
        .validate()
        .expect("composed optimization result must satisfy its contract");

    assert!(!result.parameters.is_empty());
    assert!(!result.best_parameters.is_empty());

    assert_eq!(
        result.parameters.len(),
        result.best_parameters.len()
    );

    let x = result
        .parameters
        .get(0)
        .expect("final parameter must exist");

    assert!(
        (x - 2.0).abs() < 1.0e-3,
        "optimizer should approach the quadratic minimum; x={x}"
    );

    assert!(
        result.objective.get() < 1.0e-5,
        "final objective should be close to zero; value={}",
        result.objective.get()
    );

    assert!(
        result.best_objective.get()
            <= result.objective.get() + 1.0e-12,
        "best objective must not be worse than the final objective"
    );

    assert_eq!(
        result.statistics.objective_evaluations,
        objective.statistics().evaluations
    );

    assert_eq!(
        result.statistics.iterations,
        result.history.len() as u64
    );

    assert!(
        result.statistics.objective_evaluations
            <= config.max_objective_evaluations
    );

    assert!(
        result.statistics.gradient_evaluations
            <= config.max_gradient_evaluations
    );

    assert!(
        result.statistics.optimizer_steps
            <= config.max_optimizer_steps
    );
}

#[test]
fn objective_statistics_survive_optimizer_composition() {
    let initial = ParameterVector::new(vec![1.0])
        .expect("integration parameters must be valid");

    let config = OptimizationConfig {
        max_iterations: 2,
        max_objective_evaluations: 32,
        max_gradient_evaluations: 8,
        max_optimizer_steps: 8,
        ..integration_config()
    };

    let mut objective = quadratic_objective();
    let mut optimizer = GradientDescent::new();

    let result = optimizer
        .optimize(&mut objective, initial, &config)
        .expect("bounded integration run must complete");

    let statistics = objective.statistics();

    assert_eq!(
        statistics.evaluations,
        result.statistics.objective_evaluations
    );

    // Classical objectives never claim quantum execution.
    assert_eq!(statistics.circuit_executions, 0);
    assert_eq!(statistics.shots, 0);

    assert_eq!(
        result.history.len() as u64,
        result.statistics.iterations
    );
}

#[test]
fn integration_pipeline_is_deterministic_across_independent_instances() {
    let initial = ParameterVector::new(vec![-0.5])
        .expect("integration parameters must be valid");

    let config = integration_config();

    let mut objective_a = quadratic_objective();
    let mut optimizer_a = GradientDescent::new();

    let result_a = optimizer_a
        .optimize(
            &mut objective_a,
            initial.clone(),
            &config,
        )
        .expect("first integration run must succeed");

    let mut objective_b = quadratic_objective();
    let mut optimizer_b = GradientDescent::new();

    let result_b = optimizer_b
        .optimize(
            &mut objective_b,
            initial,
            &config,
        )
        .expect("second integration run must succeed");

    assert_eq!(result_a, result_b);
    assert_eq!(
        objective_a.statistics(),
        objective_b.statistics()
    );
}

#[test]
fn integration_pipeline_preserves_objective_direction() {
    let initial = ParameterVector::new(vec![0.0])
        .expect("integration parameters must be valid");

    let config = integration_config();

    let mut objective = ClassicalObjective::with_direction(
        |parameters| {
            let x = parameters
                .get(0)
                .ok_or_else(|| {
                    AlgorithmError::invalid_input(
                        "parameters",
                        "maximization integration objective requires parameter 0",
                    )
                })?;

            ObjectiveValue::new(-((x - 2.0).powi(2)))
        },
        ObjectiveDirection::Maximize,
    );

    let mut optimizer = GradientDescent::new();

    assert_eq!(
        objective.direction(),
        ObjectiveDirection::Maximize
    );

    let result = optimizer
        .optimize(
            &mut objective,
            initial,
            &config,
        )
        .expect("maximization pipeline must succeed");

    result
        .validate()
        .expect("maximization result must be valid");

    let x = result
        .parameters
        .get(0)
        .expect("final parameter must exist");

    assert!(
        (x - 2.0).abs() < 1.0e-3,
        "maximization should approach x=2; x={x}"
    );

    assert!(
        result.best_objective.get()
            >= result.objective.get() - 1.0e-12,
        "best maximization objective must not be worse than final objective"
    );
}

#[test]
fn bounded_integration_run_reports_a_valid_terminal_status() {
    let initial = ParameterVector::new(vec![10.0])
        .expect("integration parameters must be valid");

    let config = OptimizationConfig {
        max_iterations: 1,
        max_objective_evaluations: 32,
        max_gradient_evaluations: 8,
        max_optimizer_steps: 8,
        ..integration_config()
    };

    let mut objective = quadratic_objective();
    let mut optimizer = GradientDescent::new();

    let result = optimizer
        .optimize(
            &mut objective,
            initial,
            &config,
        )
        .expect("bounded integration run must return a result");

    result
        .validate()
        .expect("bounded result must satisfy the result contract");

    assert!(matches!(
        result.status,
        ConvergenceStatus::MaxIterations
            | ConvergenceStatus::Converged
            | ConvergenceStatus::Stagnated
            | ConvergenceStatus::MaxEvaluations
            | ConvergenceStatus::MaxOptimizerSteps
    ));
}