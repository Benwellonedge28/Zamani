
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Learning Subsystem Tests
//!
//! Path: `src/ai/tests/learning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides behavioral and integration-contract tests for the canonical
//! Zamani AI learning subsystem.
//!
//! These tests exercise the actual learning-domain implementations.
//! They do not create competing learning models, tensor representations,
//! semantic systems, intermediate representations, or execution engines.
//!
//! ## Ownership
//!
//! This file owns tests for:
//!
//! - Learning algorithm contracts.
//! - Training workflow definitions.
//! - Training dependency validation.
//! - Training objectives.
//! - Loss contracts.
//! - Reinforcement learning.
//! - Learning extensibility.
//! - Structural validation.
//! - Error handling.
//! - Dynamic collection growth.
//! - POCO-REAF architectural invariants.
//!
//! ## Canonical integration
//!
//! Learning algorithms:
//!     crate::ai::learning::algorithms
//!
//! Training:
//!     crate::ai::learning::training
//!
//! Objectives and losses:
//!     crate::ai::learning::objectives
//!
//! Optimizers:
//!     crate::ai::learning::optimization
//!
//! Gradients:
//!     crate::ai::learning::gradients
//!
//! Reinforcement learning:
//!     crate::ai::learning::reinforcement
//!
//! Federated learning:
//!     crate::ai::learning::federated
//!
//! Transfer learning:
//!     crate::ai::learning::transfer
//!
//! Adaptive learning:
//!     crate::ai::learning::adaptation
//!
//! Canonical identifiers:
//!     crate::ai::ids
//!
//! Canonical AI requirements:
//!     crate::ai::requirements
//!
//! Canonical semantic authority:
//!     crate::frontend::semantic
//!
//! Universal representation:
//!     crate::zuir
//!
//! ## Grammar integration
//!
//! The tests exercise the semantic contracts corresponding to:
//!
//!     grammar/ai/learning.g4
//!     grammar/ai/training.g4
//!     grammar/ai/reinforcement.g4
//!     grammar/ai/federated-learning.g4
//!     grammar/ai/transfer-learning.g4
//!     grammar/ai/adaptation.g4
//!     grammar/ai/optimization.g4
//!
//! Universal learning syntax remains owned by:
//!
//!     grammar/statements/learn.g4
//!
//! Parser and AST integration tests belong to the frontend integration
//! test suite. These tests do not pretend that constructing a Rust
//! contract proves that source parsing has succeeded.
//!
//! ## POCO-REAF
//!
//! Tests verify that learning descriptions remain independent of:
//!
//! - Processor architecture.
//! - Accelerator vendor.
//! - Hardware topology.
//! - Physical memory capacity.
//! - Device count.
//! - Worker count.
//! - Tensor rank.
//! - Distributed node count.
//!
//! Dynamically growing collections are tested without establishing
//! language-level workload ceilings.
//!
//! Literal infinite computation is not promised.
//!
//! ## Important distinction
//!
//! Structural validation is not:
//!
//! - Semantic symbol resolution.
//! - Mathematical correctness.
//! - Gradient correctness.
//! - Numerical convergence.
//! - Capability satisfaction.
//! - Resource feasibility.
//! - Security authorization.
//! - Runtime execution.
//!
//! Tests must never treat a successful declaration or validation
//! as evidence that training has actually executed.
//!
//! ## Freeze criteria
//!
//! - Tests use canonical learning-domain types.
//! - Tests verify observable behavior.
//! - Invalid dependencies are detected.
//! - Cycles are detected.
//! - Objective invariants are enforced.
//! - Loss contracts preserve symbolic references.
//! - Reinforcement contracts validate their structures.
//! - Dynamic collections are supported.
//! - No arbitrary semantic workload ceilings are introduced.
//! - No hardware-specific assumptions are introduced.
//! - No unsafe Rust is used.

use crate::ai::ids::{
    AiDatasetId,
    AiModelId,
    AiTrainingRunId,
};

use crate::ai::learning::algorithms::{
    AlgorithmPort,
    AlgorithmPortCardinality,
    AlgorithmSignature,
};

use crate::ai::learning::objectives::{
    LossDefinition,
    LossReduction,
    ObjectiveComposition,
    ObjectiveDefinition,
    ObjectiveDirection,
    ObjectiveTerm,
};

use crate::ai::learning::reinforcement::{
    EnvironmentContract,
    EnvironmentDynamics,
    EnvironmentRef,
    EpisodeSemantics,
    Observability,
    PolicyBehavior,
    PolicyContract,
    PolicyUpdateSemantics,
    ReinforcementLearningProblem,
    RewardAggregation,
    RewardComponent,
    RewardSpecification,
};

use crate::ai::learning::training::{
    TrainingBinding,
    TrainingBindingKind,
    TrainingDefinition,
    TrainingOperation,
    TrainingPhase,
    TrainingPhaseId,
    TrainingStep,
    TrainingStepId,
    TrainingValueRef,
};

use crate::zuir::version::VersionTriple;

use crate::frontend::semantic::domains::ai::QualifiedName;


// ============================================================================
// TEST FIXTURES
// ============================================================================

/// Constructs a valid symbolic training reference.
fn value(
    name: &str,
) -> TrainingValueRef {
    TrainingValueRef::new(name)
        .expect("test fixture must have a valid symbolic reference")
}

/// Constructs a model identity.
fn model_id(
    value: u128,
) -> AiModelId {
    AiModelId::new(value)
        .expect("model identity must be nonzero")
}

/// Constructs a dataset identity.
fn dataset_id(
    value: u128,
) -> AiDatasetId {
    AiDatasetId::new(value)
        .expect("dataset identity must be nonzero")
}

/// Constructs a training run identity.
fn training_run_id(
    value: u128,
) -> AiTrainingRunId {
    AiTrainingRunId::new(value)
        .expect("training run identity must be nonzero")
}

/// Constructs a valid training operation.
fn operation(
    name: &str,
) -> TrainingOperation {
    TrainingOperation::new(name)
        .expect("test operation must be valid")
}

/// Constructs a training step.
fn step(
    identity: &str,
    operation_name: &str,
) -> TrainingStep {
    TrainingStep::new(
        TrainingStepId::new(identity)
            .expect("valid step identity"),
        operation(operation_name),
    )
}

/// Constructs a training phase.
fn phase(
    identity: &str,
) -> TrainingPhase {
    TrainingPhase::new(
        TrainingPhaseId::new(identity)
            .expect("valid phase identity"),
    )
}

/// Constructs a valid single-objective definition.
fn single_objective() -> ObjectiveDefinition {
    let term = ObjectiveTerm::new(
        "primary",
        value("training.loss"),
        ObjectiveDirection::Minimize,
    )
    .expect("valid objective term");

    ObjectiveDefinition::new(
        value("learning.objective"),
        VersionTriple::new(1, 0, 0),
        ObjectiveComposition::Single,
    )
    .expect("valid objective definition")
    .with_term(term)
}


// ============================================================================
// ALGORITHM CONTRACT TESTS
// ============================================================================

/// Algorithm signatures must accept canonical semantic identities.
#[test]
fn algorithm_signature_accepts_canonical_semantic_types() {
    let input = AlgorithmPort::new(
        QualifiedName::new(
            "learning",
            "input",
        )
        .expect("valid input identity"),
        QualifiedName::new(
            "tensor",
            "value",
        )
        .expect("valid tensor identity"),
        AlgorithmPortCardinality::One,
    )
    .expect("valid algorithm input");

    let output = AlgorithmPort::new(
        QualifiedName::new(
            "learning",
            "output",
        )
        .expect("valid output identity"),
        QualifiedName::new(
            "tensor",
            "value",
        )
        .expect("valid tensor identity"),
        AlgorithmPortCardinality::Variadic,
    )
    .expect("valid algorithm output");

    let signature = AlgorithmSignature::new(
        vec![input],
        vec![output],
    );

    assert!(
        signature.validate().is_ok(),
        "valid algorithm signatures must be accepted"
    );

    assert_eq!(signature.inputs().len(), 1);
    assert_eq!(signature.outputs().len(), 1);
}

/// Algorithm signatures must reject duplicate port identities.
#[test]
fn algorithm_signature_rejects_duplicate_ports() {
    let identity = || {
        QualifiedName::new(
            "learning",
            "shared",
        )
        .expect("valid port identity")
    };

    let data_type = QualifiedName::new(
        "tensor",
        "value",
    )
    .expect("valid data type");

    let input = AlgorithmPort::new(
        identity(),
        data_type.clone(),
        AlgorithmPortCardinality::One,
    )
    .expect("valid input");

    let output = AlgorithmPort::new(
        identity(),
        data_type,
        AlgorithmPortCardinality::One,
    )
    .expect("valid output");

    let signature = AlgorithmSignature::new(
        vec![input],
        vec![output],
    );

    assert!(
        signature.validate().is_err(),
        "duplicate input/output identities must be rejected"
    );
}


// ============================================================================
// TRAINING WORKFLOW TESTS
// ============================================================================

/// A valid training workflow must pass structural validation.
#[test]
fn training_definition_accepts_valid_workflow() {
    let first = step(
        "forward",
        "learning.forward",
    )
    .with_output(value("model.predictions"));

    let second = step(
        "loss",
        "learning.loss",
    )
    .with_dependency(
        TrainingStepId::new("forward")
            .expect("valid dependency"),
    )
    .with_input(value("model.predictions"))
    .with_output(value("training.loss"));

    let third = step(
        "update",
        "learning.parameter_update",
    )
    .with_dependency(
        TrainingStepId::new("loss")
            .expect("valid dependency"),
    )
    .with_input(value("training.loss"))
    .with_output(value("model.updated"));

    let training = TrainingDefinition::new(
        training_run_id(1),
        model_id(1),
    )
    .with_dataset(dataset_id(1))
    .with_phase(
        phase("training")
            .with_step(first)
            .with_step(second)
            .with_step(third),
    );

    let report = training
        .validate()
        .expect("structural validation must produce a report");

    assert!(
        report.is_complete(),
        "valid workflow must pass structural validation"
    );

    assert_eq!(report.phase_count(), 1);
    assert_eq!(report.step_count(), 3);
    assert_eq!(report.dataset_count(), 1);

    assert!(
        report.issues().is_empty(),
        "valid workflow must not contain structural issues"
    );

    assert!(
        report.require_complete().is_ok(),
        "complete structural validation must be accepted"
    );
}

/// Training dependencies may reference previously or subsequently
/// declared steps, provided the complete graph is valid.
#[test]
fn training_dependencies_are_resolved_across_declaration_order() {
    let dependent = step(
        "second",
        "learning.second",
    )
    .with_dependency(
        TrainingStepId::new("first")
            .expect("valid dependency"),
    );

    let prerequisite = step(
        "first",
        "learning.first",
    );

    let training = TrainingDefinition::new(
        training_run_id(2),
        model_id(2),
    )
    .with_phase(
        phase("phase")
            .with_step(dependent)
            .with_step(prerequisite),
    );

    let report = training
        .validate()
        .expect("validation report");

    assert!(
        report.is_complete(),
        "valid forward references must be supported"
    );
}

/// Unknown dependencies must not silently pass validation.
#[test]
fn training_rejects_unknown_dependencies() {
    let invalid_step = step(
        "consumer",
        "learning.consume",
    )
    .with_dependency(
        TrainingStepId::new("missing")
            .expect("valid symbolic identity"),
    );

    let training = TrainingDefinition::new(
        training_run_id(3),
        model_id(3),
    )
    .with_phase(
        phase("phase")
            .with_step(invalid_step),
    );

    let report = training
        .validate()
        .expect("structural report should be returned");

    assert!(
        !report.is_complete(),
        "unknown dependencies must make validation incomplete"
    );

    assert!(
        !report.issues().is_empty(),
        "validation must explain the unresolved dependency"
    );

    assert!(
        report.require_complete().is_err(),
        "incomplete reports must not be accepted as complete"
    );
}

/// Cyclic training dependencies must be rejected.
#[test]
fn training_rejects_dependency_cycles() {
    let first = step(
        "first",
        "learning.first",
    )
    .with_dependency(
        TrainingStepId::new("second")
            .expect("valid dependency"),
    );

    let second = step(
        "second",
        "learning.second",
    )
    .with_dependency(
        TrainingStepId::new("first")
            .expect("valid dependency"),
    );

    let training = TrainingDefinition::new(
        training_run_id(4),
        model_id(4),
    )
    .with_phase(
        phase("phase")
            .with_step(first)
            .with_step(second),
    );

    let report = training
        .validate()
        .expect("validation report");

    assert!(
        !report.is_complete(),
        "cyclic dependency graphs must be rejected"
    );
}

/// A step must not depend on itself.
#[test]
fn training_step_rejects_self_dependency() {
    let identity = TrainingStepId::new("self")
        .expect("valid identity");

    let result = TrainingStep::new(
        identity.clone(),
        operation("learning.operation"),
    )
    .with_dependency(identity)
    .validate();

    assert!(
        result.is_err(),
        "self-dependencies are invalid"
    );
}

/// Duplicate dependencies must be rejected.
#[test]
fn training_step_rejects_duplicate_dependencies() {
    let dependency = TrainingStepId::new("source")
        .expect("valid dependency");

    let result = TrainingStep::new(
        TrainingStepId::new("consumer")
            .expect("valid identity"),
        operation("learning.operation"),
    )
    .with_dependency(dependency.clone())
    .with_dependency(dependency)
    .validate();

    assert!(
        result.is_err(),
        "duplicate dependencies must be rejected"
    );
}

/// Duplicate training step identities must be rejected.
#[test]
fn training_rejects_duplicate_step_identities() {
    let training = TrainingDefinition::new(
        training_run_id(5),
        model_id(5),
    )
    .with_phase(
        phase("phase")
            .with_step(step("duplicate", "learning.first"))
            .with_step(step("duplicate", "learning.second")),
    );

    let report = training
        .validate()
        .expect("validation report");

    assert!(
        !report.is_complete(),
        "duplicate step identities must be rejected"
    );
}

/// Duplicate phase identities must be rejected.
#[test]
fn training_rejects_duplicate_phase_identities() {
    let training = TrainingDefinition::new(
        training_run_id(6),
        model_id(6),
    )
    .with_phase(phase("duplicate"))
    .with_phase(phase("duplicate"));

    let report = training
        .validate()
        .expect("validation report");

    assert!(
        !report.is_complete(),
        "duplicate phase identities must be rejected"
    );
}

/// Duplicate datasets must be rejected.
#[test]
fn training_rejects_duplicate_dataset_identities() {
    let training = TrainingDefinition::new(
        training_run_id(7),
        model_id(7),
    )
    .with_dataset(dataset_id(10))
    .with_dataset(dataset_id(10));

    let report = training
        .validate()
        .expect("validation report");

    assert!(
        !report.is_complete(),
        "duplicate dataset identities must be rejected"
    );
}

/// Training bindings must retain their semantic category and identity.
#[test]
fn training_binding_preserves_symbolic_semantics() {
    let binding = TrainingBinding::new(
        TrainingBindingKind::Objective,
        "main_objective",
        value("learning.objective"),
    )
    .expect("valid training binding");

    assert_eq!(
        binding.kind(),
        TrainingBindingKind::Objective
    );

    assert_eq!(
        binding.name(),
        "main_objective"
    );

    assert_eq!(
        binding.value().as_str(),
        "learning.objective"
    );
}

/// Invalid empty binding names must be rejected.
#[test]
fn training_binding_rejects_empty_names() {
    let binding = TrainingBinding::new(
        TrainingBindingKind::Objective,
        "",
        value("learning.objective"),
    );

    assert!(
        binding.is_err(),
        "empty binding names must be rejected"
    );
}


// ============================================================================
// DYNAMIC WORKFLOW GROWTH
// ============================================================================

/// Training definitions must support dynamically growing workflows.
///
/// This is a finite regression workload, not a language-level
/// maximum or a promise of unlimited physical execution.
#[test]
fn training_supports_dynamically_growing_step_collections() {
    let mut training = TrainingDefinition::new(
        training_run_id(8),
        model_id(8),
    );

    let mut training_phase = phase("dynamic_phase");

    let mut previous: Option<TrainingStepId> = None;

    for index in 0_u128..256 {
        let identity = format!("step_{index}");

        let step_id = TrainingStepId::new(identity.clone())
            .expect("generated identity must be valid");

        let mut current = TrainingStep::new(
            step_id.clone(),
            operation("learning.extension_operation"),
        );

        if let Some(dependency) = previous {
            current = current.with_dependency(dependency);
        }

        training_phase = training_phase.with_step(current);

        previous = Some(step_id);
    }

    training = training.with_phase(training_phase);

    let report = training
        .validate()
        .expect("dynamic workflow validation");

    assert!(
        report.is_complete(),
        "dynamically constructed workflows must validate"
    );

    assert_eq!(report.step_count(), 256);
}


// ============================================================================
// OBJECTIVE TESTS
// ============================================================================

/// Single-objective composition must accept exactly one term.
#[test]
fn objective_accepts_single_term() {
    let objective = single_objective();

    assert!(
        objective.validate().is_ok(),
        "valid single objective must pass validation"
    );

    assert_eq!(objective.terms().len(), 1);

    assert_eq!(
        objective.identity().as_str(),
        "learning.objective"
    );
}

/// Single-objective composition must reject multiple terms.
#[test]
fn objective_rejects_multiple_terms_in_single_composition() {
    let objective = single_objective()
        .with_term(
            ObjectiveTerm::new(
                "secondary",
                value("training.secondary_loss"),
                ObjectiveDirection::Minimize,
            )
            .expect("valid objective term"),
        );

    assert!(
        objective.validate().is_err(),
        "single composition cannot silently accept multiple terms"
    );
}

/// An objective without terms must be rejected.
#[test]
fn objective_rejects_empty_terms() {
    let objective = ObjectiveDefinition::new(
        value("learning.empty"),
        VersionTriple::new(1, 0, 0),
        ObjectiveComposition::Single,
    )
    .expect("valid definition");

    assert!(
        objective.validate().is_err(),
        "empty objectives must be rejected"
    );
}

/// Weighted objectives must explicitly provide every weight.
#[test]
fn weighted_objective_requires_explicit_weights() {
    let objective = ObjectiveDefinition::new(
        value("learning.weighted"),
        VersionTriple::new(1, 0, 0),
        ObjectiveComposition::Weighted,
    )
    .expect("valid definition")
    .with_term(
        ObjectiveTerm::new(
            "accuracy",
            value("metrics.accuracy"),
            ObjectiveDirection::Maximize,
        )
        .expect("valid term"),
    );

    assert!(
        objective.validate().is_err(),
        "weighted objectives must not invent missing weights"
    );
}

/// Weighted objectives must accept explicitly declared weights.
#[test]
fn weighted_objective_accepts_explicit_weights() {
    let term = ObjectiveTerm::new(
        "accuracy",
        value("metrics.accuracy"),
        ObjectiveDirection::Maximize,
    )
    .expect("valid term")
    .with_weight(value("weights.accuracy"))
    .expect("valid weight");

    let objective = ObjectiveDefinition::new(
        value("learning.weighted"),
        VersionTriple::new(1, 0, 0),
        ObjectiveComposition::Weighted,
    )
    .expect("valid definition")
    .with_term(term);

    assert!(
        objective.validate().is_ok(),
        "explicitly weighted objectives must validate"
    );
}

/// Objective term identities must be unique.
#[test]
fn objective_rejects_duplicate_term_identities() {
    let objective = single_objective()
        .with_term(
            ObjectiveTerm::new(
                "primary",
                value("training.other_loss"),
                ObjectiveDirection::Maximize,
            )
            .expect("valid term"),
        );

    assert!(
        objective.validate().is_err(),
        "duplicate objective terms must be rejected"
    );
}

/// Custom objective composition must remain extensible.
#[test]
fn objective_supports_extension_defined_composition() {
    let objective = ObjectiveDefinition::new(
        value("learning.custom"),
        VersionTriple::new(1, 0, 0),
        ObjectiveComposition::Custom(
            value("extensions.custom_composition"),
        ),
    )
    .expect("valid custom definition")
    .with_term(
        ObjectiveTerm::new(
            "term",
            value("training.loss"),
            ObjectiveDirection::Minimize,
        )
        .expect("valid term"),
    );

    assert!(
        objective.validate().is_ok(),
        "extension-defined composition must be supported"
    );
}

/// Objectives must convert into the canonical training binding.
#[test]
fn objective_integrates_with_training_binding() {
    let objective = single_objective();

    let binding = objective
        .to_training_binding()
        .expect("objective binding");

    assert_eq!(
        binding.kind(),
        TrainingBindingKind::Objective
    );

    assert_eq!(
        binding.value().as_str(),
        "learning.objective"
    );
}


// ============================================================================
// LOSS CONTRACT TESTS
// ============================================================================

/// Loss definitions must preserve canonical symbolic references.
#[test]
fn loss_definition_preserves_symbolic_inputs() {
    let loss = LossDefinition::new(
        value("loss.mean_squared"),
        VersionTriple::new(1, 0, 0),
        value("operations.mean_squared"),
        value("model.predictions"),
        value("dataset.targets"),
    )
    .expect("valid loss definition");

    assert!(
        loss.validate().is_ok(),
        "valid loss contracts must pass validation"
    );

    assert_eq!(
        loss.identity().as_str(),
        "loss.mean_squared"
    );

    assert_eq!(
        loss.predictions().as_str(),
        "model.predictions"
    );

    assert_eq!(
        loss.targets().as_str(),
        "dataset.targets"
    );
}

/// Loss reduction must remain an extensible symbolic contract.
#[test]
fn loss_supports_explicit_reduction() {
    let reduction = LossReduction::new(
        value("operations.custom_reduction"),
    )
    .expect("valid reduction");

    let loss = LossDefinition::new(
        value("loss.custom"),
        VersionTriple::new(1, 0, 0),
        value("operations.custom_loss"),
        value("predictions"),
        value("targets"),
    )
    .expect("valid loss")
    .with_reduction(reduction)
    .expect("valid reduction assignment");

    assert!(
        loss.validate().is_ok(),
        "valid reduction contracts must be accepted"
    );

    assert!(
        loss.reduction().is_some()
    );
}

/// Loss definitions must support symbolic masks and sample weights.
#[test]
fn loss_supports_masks_and_sample_weights() {
    let loss = LossDefinition::new(
        value("loss.weighted"),
        VersionTriple::new(1, 0, 0),
        value("operations.weighted_loss"),
        value("predictions"),
        value("targets"),
    )
    .expect("valid loss")
    .with_sample_weights(value("dataset.sample_weights"))
    .expect("valid weights")
    .with_mask(value("dataset.mask"))
    .expect("valid mask");

    assert!(
        loss.validate().is_ok()
    );

    assert_eq!(
        loss.sample_weights()
            .expect("sample weights")
            .as_str(),
        "dataset.sample_weights"
    );

    assert_eq!(
        loss.mask()
            .expect("mask")
            .as_str(),
        "dataset.mask"
    );
}


// ============================================================================
// REINFORCEMENT LEARNING TESTS
// ============================================================================

/// Reinforcement-learning contracts must compose their canonical
/// environment, policy and reward interfaces.
#[test]
fn reinforcement_learning_accepts_valid_problem() {
    let environment = EnvironmentContract::new(
        EnvironmentRef::new("environment.control")
            .expect("valid environment"),
        EnvironmentDynamics::Stochastic,
        Observability::PartiallyObservable,
        EpisodeSemantics::Episodic,
        value("environment.state"),
        value("environment.action"),
    )
    .expect("valid environment contract");

    let policy = PolicyContract::new(
        value("policy.primary"),
        PolicyBehavior::Stochastic,
        PolicyUpdateSemantics::Trainable,
        value("environment.observation"),
        value("environment.selected_action"),
    )
    .expect("valid policy");

    let reward = RewardSpecification::new(
        RewardAggregation::Scalar,
        vec![
            RewardComponent::new(
                "reward",
                value("environment.reward"),
            )
            .expect("valid reward component"),
        ],
    );

    let problem = ReinforcementLearningProblem::new(
        environment,
        policy,
        reward,
    )
    .expect("valid reinforcement problem");

    assert!(
        problem.validate().is_ok(),
        "valid reinforcement problems must pass structural validation"
    );

    assert_eq!(
        problem.policy().identity().as_str(),
        "policy.primary"
    );

    assert_eq!(
        problem.reward().components().len(),
        1
    );
}

/// Empty reward specifications must be rejected.
#[test]
fn reinforcement_learning_rejects_empty_reward() {
    let reward = RewardSpecification::new(
        RewardAggregation::Scalar,
        Vec::new(),
    );

    assert!(
        reward.validate().is_err(),
        "a reward specification requires at least one component"
    );
}

/// Duplicate reward component identities must be rejected.
#[test]
fn reinforcement_learning_rejects_duplicate_reward_components() {
    let reward = RewardSpecification::new(
        RewardAggregation::MultiObjective,
        vec![
            RewardComponent::new(
                "utility",
                value("reward.utility"),
            )
            .expect("first component"),
            RewardComponent::new(
                "utility",
                value("reward.safety"),
            )
            .expect("second component"),
        ],
    );

    assert!(
        reward.validate().is_err(),
        "reward component identities must be unique"
    );
}

/// Reinforcement policies must preserve their update semantics.
#[test]
fn reinforcement_policy_preserves_update_semantics() {
    let policy = PolicyContract::new(
        value("policy.fixed"),
        PolicyBehavior::Deterministic,
        PolicyUpdateSemantics::Fixed,
        value("state"),
        value("action"),
    )
    .expect("valid policy");

    assert_eq!(
        policy.update_semantics(),
        PolicyUpdateSemantics::Fixed
    );

    assert!(
        policy.validate().is_ok()
    );
}


// ============================================================================
// VALIDATION BOUNDARY TESTS
// ============================================================================

/// Successful structural validation must not be described as execution.
#[test]
fn structural_training_validation_does_not_execute_training() {
    let training = TrainingDefinition::new(
        training_run_id(9),
        model_id(9),
    )
    .with_phase(
        phase("declaration")
            .with_step(
                step(
                    "declaration_step",
                    "learning.forward",
                ),
            ),
    );

    let report = training
        .validate()
        .expect("structural validation");

    assert!(
        report.is_complete()
    );

    // The report establishes structural validity only.
    // No runtime, backend, hardware or training result is created here.
    assert_eq!(
        report.step_count(),
        1
    );
}

/// Invalid input references must fail at construction boundaries.
#[test]
fn symbolic_references_reject_empty_values() {
    assert!(
        TrainingValueRef::new("").is_err()
    );

    assert!(
        TrainingValueRef::new("   ").is_err()
    );
}

/// Zero is not a valid canonical AI model identity.
#[test]
fn model_identity_rejects_zero() {
    assert!(
        AiModelId::new(0).is_err()
    );
}

/// Zero is not a valid canonical dataset identity.
#[test]
fn dataset_identity_rejects_zero() {
    assert!(
        AiDatasetId::new(0).is_err()
    );
}

/// Zero is not a valid canonical training-run identity.
#[test]
fn training_run_identity_rejects_zero() {
    assert!(
        AiTrainingRunId::new(0).is_err()
    );
}


// ============================================================================
// REPRODUCIBILITY OF DECLARATIONS
// ============================================================================

/// Identical workflow declarations must produce identical structural
/// validation results.
#[test]
fn equivalent_training_definitions_validate_consistently() {
    fn construct() -> TrainingDefinition {
        TrainingDefinition::new(
            training_run_id(100),
            model_id(100),
        )
        .with_dataset(dataset_id(100))
        .with_phase(
            phase("training")
                .with_step(step(
                    "forward",
                    "learning.forward",
                ))
                .with_step(
                    step(
                        "loss",
                        "learning.loss",
                    )
                    .with_dependency(
                        TrainingStepId::new("forward")
                            .expect("dependency"),
                    ),
                ),
        )
    }

    let first = construct()
        .validate()
        .expect("first report");

    let second = construct()
        .validate()
        .expect("second report");

    assert_eq!(
        first,
        second,
        "equivalent declarations must validate deterministically"
    );
}
