//! Determinism contract tests for `quantum::algorithms`.
//!
//! These tests exercise the stable algorithm-layer contracts already owned by
//! `types`, `objective`, and `optimizer`. They deliberately do not depend on a
//! simulator, QPU, routing implementation, transpiler, or hardware.
//!
//! # Determinism contract
//!
//! For a deterministic algorithm invocation, the same:
//!
//! - algorithm implementation;
//! - initial parameter vector;
//! - objective;
//! - optimizer configuration;
//! - explicit seed where applicable;
//!
//! must produce the same observable algorithm result.
//!
//! The deterministic reference optimizer currently used here is
//! `GradientDescent`, which creates no randomness. The tests therefore verify
//! the strongest deterministic contract available without coupling this test
//! file to a concrete execution backend.
//!
//! # Integration boundary
//!
//! ```text
//! ParameterVector ──► ClassicalObjective ──► GradientDescent
//!        │                    │                     │
//!        └────────────────────┴─────────────────────┘
//!                              │
//!                              ▼
//!                    OptimizationResult
//! ```
//!
//! Quantum execution determinism is separately enforced by
//! `execution::ExecutionResult::validate_against`. This file does not invent
//! a backend result merely to exercise that boundary.
//!
//! # Ownership
//!
//! This file owns cross-component determinism tests only.
//!
//! It does not redefine:
//!
//! - deterministic configuration;
//! - seed semantics;
//! - optimizer behavior;
//! - objective behavior;
//! - execution behavior;
//! - algorithm metadata.
//!
//! Those contracts remain owned by their respective production modules.
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
use super::super::objective::{ClassicalObjective, Objective};
use super::super::optimizer::{
    GradientDescent,
    OptimizationConfig,
    Optimizer,
};
use super::super::types::{
    AlgorithmId,
    AlgorithmMetadata,
    AlgorithmVersion,
    ObjectiveValue,
    ParameterVector,
    Seed,
};

/// Creates the deterministic reference objective used by the tests.
///
/// The function is intentionally:
///
/// - pure;
/// - finite;
/// - deterministic;
/// - independent of global state;
/// - independent of system time;
/// - independent of randomness;
/// - independent of a quantum backend.
///
/// The objective is:
///
/// `f(x, y) = x² + 2y²`
fn quadratic_objective() -> ClassicalObjective {
    ClassicalObjective::new(|parameters| {
        let x = parameters.get(0).ok_or_else(|| {
            AlgorithmError::invalid_input(
                "parameter[0]",
                "missing x parameter",
            )
        })?;

        let y = parameters.get(1).ok_or_else(|| {
            AlgorithmError::invalid_input(
                "parameter[1]",
                "missing y parameter",
            )
        })?;

        ObjectiveValue::new(x.mul_add(x, 2.0 * y * y))
    })
}

/// Returns a deliberately small deterministic optimizer configuration.
///
/// The limits are explicit so that the test proves deterministic behavior
/// without depending on production-default resource ceilings.
fn deterministic_config() -> OptimizationConfig {
    OptimizationConfig {
        learning_rate: 0.1,
        tolerance: 1.0e-8,
        max_iterations: 12,
        gradient_epsilon: 1.0e-6,
        patience: 0,
        objective_improvement_tolerance: 1.0e-12,
        max_parameter_magnitude: 1.0e6,
        max_objective_evaluations: 256,
        max_gradient_evaluations: 64,
        max_optimizer_steps: 64,
        min_learning_rate: 1.0e-15,
    }
}

#[test]
fn identical_deterministic_optimization_runs_are_equal() {
    let initial = ParameterVector::new(vec![0.75, -0.5])
        .expect("test parameters must be valid");

    let config = deterministic_config();

    let mut objective_a = quadratic_objective();
    let mut optimizer_a = GradientDescent::new();

    let result_a = optimizer_a
        .optimize(
            &mut objective_a,
            initial.clone(),
            &config,
        )
        .expect("first deterministic optimization must succeed");

    let mut objective_b = quadratic_objective();
    let mut optimizer_b = GradientDescent::new();

    let result_b = optimizer_b
        .optimize(
            &mut objective_b,
            initial,
            &config,
        )
        .expect("second deterministic optimization must succeed");

    result_a
        .validate()
        .expect("first optimization result must be valid");

    result_b
        .validate()
        .expect("second optimization result must be valid");

    // OptimizationResult equality covers:
    //
    // - final parameters;
    // - final objective;
    // - best objective;
    // - best parameters;
    // - convergence status;
    // - optimization statistics;
    // - complete optimization history.
    //
    // This is deliberately stronger than comparing only the final objective.
    assert_eq!(result_a, result_b);
}

#[test]
fn repeated_objective_evaluation_is_deterministic_for_identical_parameters() {
    let parameters = ParameterVector::new(vec![0.25, -0.75])
        .expect("test parameters must be valid");

    let mut objective = quadratic_objective();

    let first = objective
        .evaluate(&parameters)
        .expect("first objective evaluation must succeed");

    let second = objective
        .evaluate(&parameters)
        .expect("second objective evaluation must succeed");

    first
        .validate()
        .expect("first objective evaluation must be valid");

    second
        .validate()
        .expect("second objective evaluation must be valid");

    // The mathematical observation must be identical.
    assert_eq!(first.value, second.value);
    assert_eq!(first.circuit_executions, second.circuit_executions);
    assert_eq!(first.shots, second.shots);
    assert_eq!(first.uncertainty, second.uncertainty);

    // Classical objectives are deterministic by construction.
    assert!(first.deterministic);
    assert!(second.deterministic);

    // Evaluation indices are accounting metadata, not part of mathematical
    // determinism. They intentionally advance within one objective session.
    assert_eq!(first.evaluation_index, 1);
    assert_eq!(second.evaluation_index, 2);
}

#[test]
fn deterministic_parameter_inputs_are_stable_across_clones() {
    let original = ParameterVector::new(vec![
        0.0,
        1.25,
        -2.5,
        std::f64::consts::PI,
    ])
    .expect("test parameters must be valid");

    let cloned = original.clone();

    assert_eq!(original, cloned);
    assert_eq!(original.as_slice(), cloned.as_slice());
    assert_eq!(original.max_abs(), cloned.max_abs());
}

#[test]
fn explicit_seed_is_stable() {
    let seed_a = Seed::new(0x5A17_2026);
    let seed_b = Seed::new(0x5A17_2026);
    let seed_c = Seed::new(0x5A17_2027);

    assert_eq!(seed_a, seed_b);
    assert_ne!(seed_a, seed_c);

    assert_eq!(seed_a.get(), 0x5A17_2026);
    assert_eq!(seed_b.get(), 0x5A17_2026);
    assert_eq!(seed_c.get(), 0x5A17_2027);
}

#[test]
fn algorithm_metadata_is_stable_for_replay_identity() {
    let first = AlgorithmMetadata::new(
        AlgorithmId::Vqe,
        AlgorithmVersion::initial(),
    );

    let second = AlgorithmMetadata::new(
        AlgorithmId::Vqe,
        AlgorithmVersion::new(1, 0, 0),
    );

    assert_eq!(first, second);

    assert_eq!(first.algorithm_id(), AlgorithmId::Vqe);
    assert_eq!(
        first.version(),
        AlgorithmVersion::initial()
    );
}

#[test]
fn independent_deterministic_runs_remain_equal() {
    let initial = ParameterVector::new(vec![1.0, 1.0])
        .expect("test parameters must be valid");

    let config = deterministic_config();

    let mut results = Vec::with_capacity(3);

    for _ in 0..3 {
        let mut objective = quadratic_objective();
        let mut optimizer = GradientDescent::new();

        let result = optimizer
            .optimize(
                &mut objective,
                initial.clone(),
                &config,
            )
            .expect("deterministic optimization must succeed");

        result
            .validate()
            .expect("optimization result must remain valid");

        results.push(result);
    }

    assert_eq!(results[0], results[1]);
    assert_eq!(results[1], results[2]);
}