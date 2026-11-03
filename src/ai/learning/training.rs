
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Training Workflow Contracts
//!
//! Path: src/ai/learning/training.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent AI training workflow descriptions.
//!
//! This module describes WHAT a training computation must accomplish.
//! It does not implement numerical training or execute computational work.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Training workflow definitions.
//! - Training phase descriptions.
//! - Training step descriptions.
//! - Training dependency relationships.
//! - Objective, loss and optimizer references.
//! - Training metrics.
//! - Hyperparameter references.
//! - Dataset references.
//! - Training schedules.
//! - Checkpoint intent.
//! - Reproducibility intent.
//! - Training lifecycle contracts.
//! - Structural training validation.
//! - Training execution-boundary descriptions.
//!
//! ## Does not own
//!
//! - Source syntax or AST construction.
//! - Name resolution.
//! - Type checking.
//! - Tensor storage.
//! - Model parameter storage.
//! - Gradient computation.
//! - Automatic differentiation.
//! - Optimizer implementations.
//! - Numerical kernels.
//! - Dataset storage.
//! - Physical checkpoint storage.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling or placement.
//! - Distributed communication.
//! - Runtime execution.
//! - Backend-specific implementations.
//!
//! ## Canonical dependencies
//!
//! AI identifiers:
//!     crate::ai::ids
//!
//! AI errors:
//!     crate::ai::error
//!
//! AI requirements:
//!     crate::ai::requirements
//!
//! Model identities:
//!     crate::ai::ids::AiModelId
//!
//! Dataset identities:
//!     crate::ai::ids::AiDatasetId
//!
//! Training identities:
//!     crate::ai::ids::AiTrainingRunId
//!
//! Learning algorithm contracts:
//!     crate::ai::learning::algorithms
//!
//! Training syntax:
//!     grammar/ai/training.g4
//!
//! Learning syntax:
//!     grammar/ai/learning.g4
//!
//! ## POCO-REAF
//!
//! This module introduces no universal ceilings for:
//!
//! - Training runs.
//! - Models.
//! - Datasets.
//! - Phases.
//! - Steps.
//! - Parameters.
//! - Objectives.
//! - Losses.
//! - Metrics.
//! - Hyperparameters.
//! - Checkpoints.
//! - Workers.
//! - Accelerators.
//! - Devices.
//! - Distributed nodes.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Execution duration.
//!
//! Actual resource feasibility belongs to downstream authorities.
//!
//! Literal infinite computation is not promised.
//!
//! ## Integration
//!
//! Frontend semantic AI:
//!     Converts canonical AST training declarations into validated
//!     training-domain descriptions.
//!
//! Model:
//!     Supplies AiModelId and model compatibility information.
//!
//! Learning algorithms:
//!     Supplies registered algorithm contracts.
//!
//! Objectives:
//!     Resolves objective and loss references.
//!
//! Optimization:
//!     Resolves optimizer references and update semantics.
//!
//! Gradients:
//!     Supplies gradient computation contracts.
//!
//! Differentiation:
//!     Supplies forward/reverse differentiation representations.
//!
//! ZUIR:
//!     Receives lowered canonical operations through an adapter.
//!
//! Distributed:
//!     Handles distributed execution, checkpoint recovery and coordination.
//!
//! Runtime:
//!     Owns actual training execution and lifecycle management.
//!
//! ## Important
//!
//! A structurally valid training definition is not proof that:
//!
//! - All symbols resolve.
//! - All operations are registered.
//! - An optimizer implementation exists.
//! - Gradients can be computed.
//! - Required capabilities exist.
//! - Resources are sufficient.
//! - Execution is authorized.
//! - Training has succeeded.
//!
//! These are separate validation and execution stages.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::{
    AiDatasetId,
    AiModelId,
    AiTrainingRunId,
};

use crate::ai::requirements::AiRequirements;

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_text(
    description: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(
            AiError::new(
                AiErrorKind::InvalidInput,
                format!("{description} cannot be empty"),
            )
            .with_phase(AiErrorPhase::Validation),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            AiError::new(
                AiErrorKind::InvalidInput,
                format!("{description} contains control characters"),
            )
            .with_phase(AiErrorPhase::Validation),
        );
    }

    Ok(())
}

fn training_error(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::TrainingFailed,
        message,
    )
    .with_phase(AiErrorPhase::Training)
}

fn invalid_training_input(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
    .with_phase(AiErrorPhase::Validation)
}

// ============================================================================
// TRAINING IDENTITIES
// ============================================================================

/// Stable identity of an individual training step.
///
/// This identity is local to its containing training run.
/// It does not represent a machine instruction, kernel or thread.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrainingStepId(String);

impl TrainingStepId {
    /// Creates a validated training step identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();
        validate_text("training step identity", &value)?;

        Ok(Self(value))
    }

    /// Returns the underlying identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrainingStepId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Identity of a training phase.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrainingPhaseId(String);

impl TrainingPhaseId {
    /// Creates a validated phase identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();
        validate_text("training phase identity", &value)?;

        Ok(Self(value))
    }

    /// Returns the underlying identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrainingPhaseId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An open-world semantic reference to a training-related value.
///
/// Examples include model objectives, optimizer definitions, metrics,
/// schedules, expressions and extension-defined concepts.
///
/// The referenced declaration is resolved by canonical semantic analysis
/// or the appropriate AI registry.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrainingValueRef(String);

impl TrainingValueRef {
    /// Creates a validated symbolic reference.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();
        validate_text("training value reference", &value)?;

        Ok(Self(value))
    }

    /// Returns the symbolic reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrainingValueRef {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// TRAINING MEMBER CLASSIFICATION
// ============================================================================

/// Semantic category of a named training declaration.
///
/// The category is descriptive. It does not identify a concrete algorithm.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TrainingBindingKind {
    /// Model reference.
    Model,

    /// Dataset reference.
    Dataset,

    /// Training objective.
    Objective,

    /// Loss definition.
    Loss,

    /// Optimizer definition.
    Optimizer,

    /// Evaluation metric.
    Metric,

    /// Hyperparameter.
    Hyperparameter,

    /// Learning-rate or other training schedule.
    Schedule,

    /// Validation configuration.
    Validation,

    /// Evaluation configuration.
    Evaluation,

    /// Checkpoint declaration.
    Checkpoint,

    /// Reproducibility declaration.
    Reproducibility,

    /// Distributed training intent.
    Distributed,

    /// Accelerator or execution capability intent.
    Accelerator,

    /// Hybrid classical/quantum training intent.
    Hybrid,

    /// Extension-defined training declaration.
    Extension,
}

/// A named symbolic binding used by a training workflow.
///
/// Values remain references to canonical semantic expressions.
/// This module does not duplicate the frontend expression AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingBinding {
    kind: TrainingBindingKind,
    name: String,
    value: TrainingValueRef,
}

impl TrainingBinding {
    /// Creates a training binding.
    pub fn new(
        kind: TrainingBindingKind,
        name: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<Self> {
        let name = name.into();
        validate_text("training binding name", &name)?;

        Ok(Self {
            kind,
            name,
            value,
        })
    }

    /// Returns the binding category.
    #[must_use]
    pub const fn kind(&self) -> TrainingBindingKind {
        self.kind
    }

    /// Returns the binding name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns its symbolic value reference.
    #[must_use]
    pub fn value(&self) -> &TrainingValueRef {
        &self.value
    }
}

// ============================================================================
// TRAINING STEP
// ============================================================================

/// An abstract training operation.
///
/// The operation name is open-world. It may refer to:
///
/// - Forward computation.
/// - Loss evaluation.
/// - Gradient computation.
/// - Parameter update.
/// - Validation.
/// - Evaluation.
/// - Checkpoint creation.
/// - Reinforcement learning.
/// - Federated aggregation.
/// - Quantum-assisted computation.
/// - Extension-defined operations.
///
/// Actual operation semantics are resolved through the canonical
/// learning algorithm and AI operation registries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingOperation {
    identity: TrainingValueRef,
}

impl TrainingOperation {
    /// Creates an operation reference.
    pub fn new(
        identity: impl Into<String>,
    ) -> AiResult<Self> {
        Ok(Self {
            identity: TrainingValueRef::new(identity)?,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn identity(&self) -> &TrainingValueRef {
        &self.identity
    }
}

/// A symbolic dependency on another training step.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TrainingStepDependency {
    step: TrainingStepId,
}

impl TrainingStepDependency {
    /// Creates a dependency reference.
    #[must_use]
    pub fn new(step: TrainingStepId) -> Self {
        Self { step }
    }

    /// Returns the referenced step.
    #[must_use]
    pub fn step(&self) -> &TrainingStepId {
        &self.step
    }
}

/// A target-independent training step.
///
/// Steps describe logical computation, not physical execution units.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingStep {
    id: TrainingStepId,
    operation: TrainingOperation,
    dependencies: Vec<TrainingStepDependency>,
    inputs: Vec<TrainingValueRef>,
    outputs: Vec<TrainingValueRef>,
}

impl TrainingStep {
    /// Creates a training step.
    pub fn new(
        id: TrainingStepId,
        operation: TrainingOperation,
    ) -> Self {
        Self {
            id,
            operation,
            dependencies: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Adds a dependency.
    #[must_use]
    pub fn with_dependency(
        mut self,
        dependency: TrainingStepId,
    ) -> Self {
        self.dependencies.push(
            TrainingStepDependency::new(dependency),
        );
        self
    }

    /// Adds an input reference.
    #[must_use]
    pub fn with_input(
        mut self,
        input: TrainingValueRef,
    ) -> Self {
        self.inputs.push(input);
        self
    }

    /// Adds an output reference.
    #[must_use]
    pub fn with_output(
        mut self,
        output: TrainingValueRef,
    ) -> Self {
        self.outputs.push(output);
        self
    }

    /// Returns the step identity.
    #[must_use]
    pub fn id(&self) -> &TrainingStepId {
        &self.id
    }

    /// Returns the operation.
    #[must_use]
    pub fn operation(&self) -> &TrainingOperation {
        &self.operation
    }

    /// Returns declared dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[TrainingStepDependency] {
        &self.dependencies
    }

    /// Returns input references.
    #[must_use]
    pub fn inputs(&self) -> &[TrainingValueRef] {
        &self.inputs
    }

    /// Returns output references.
    #[must_use]
    pub fn outputs(&self) -> &[TrainingValueRef] {
        &self.outputs
    }

    /// Validates local step invariants.
    pub fn validate(&self) -> AiResult<()> {
        let mut dependencies = BTreeSet::new();

        for dependency in &self.dependencies {
            if dependency.step == self.id {
                return Err(
                    invalid_training_input(
                        "a training step cannot depend on itself",
                    ),
                );
            }

            if !dependencies.insert(&dependency.step) {
                return Err(
                    invalid_training_input(
                        "duplicate training step dependency",
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// TRAINING PHASE
// ============================================================================

/// A logically grouped training phase.
///
/// Phase names are open-world. The language does not impose a fixed
/// sequence such as warmup, training and fine-tuning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingPhase {
    id: TrainingPhaseId,
    description: Option<String>,
    steps: Vec<TrainingStep>,
}

impl TrainingPhase {
    /// Creates an empty phase.
    #[must_use]
    pub fn new(id: TrainingPhaseId) -> Self {
        Self {
            id,
            description: None,
            steps: Vec::new(),
        }
    }

    /// Sets an optional description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();
        validate_text("training phase description", &description)?;

        self.description = Some(description);
        Ok(self)
    }

    /// Adds a training step.
    #[must_use]
    pub fn with_step(
        mut self,
        step: TrainingStep,
    ) -> Self {
        self.steps.push(step);
        self
    }

    /// Returns the phase identity.
    #[must_use]
    pub fn id(&self) -> &TrainingPhaseId {
        &self.id
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns steps in their declared source order.
    #[must_use]
    pub fn steps(&self) -> &[TrainingStep] {
        &self.steps
    }
}

// ============================================================================
// CHECKPOINT INTENT
// ============================================================================

/// Logical reason for creating a training checkpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CheckpointTrigger {
    /// Explicitly requested checkpoint.
    Explicit,

    /// Checkpoint associated with a training phase boundary.
    PhaseBoundary,

    /// Checkpoint associated with a recovery policy.
    Recovery,

    /// Checkpoint associated with a validation result.
    Validation,

    /// Checkpoint associated with a lifecycle transition.
    LifecycleTransition,

    /// Extension-defined trigger.
    Extension,
}

/// Logical checkpoint consistency requirements.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum CheckpointConsistency {
    /// A checkpoint must represent a consistent logical state.
    Consistent,

    /// The checkpoint may represent an explicitly supported
    /// asynchronously captured state.
    Asynchronous,

    /// Consistency semantics are determined by an extension.
    ExtensionDefined,
}

/// A checkpoint declaration.
///
/// This describes persistence and recovery intent only.
///
/// Physical storage, serialization implementation, transport,
/// replication and recovery remain downstream responsibilities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingCheckpoint {
    name: String,
    trigger: CheckpointTrigger,
    consistency: CheckpointConsistency,
    condition: Option<TrainingValueRef>,
}

impl TrainingCheckpoint {
    /// Creates a checkpoint declaration.
    pub fn new(
        name: impl Into<String>,
        trigger: CheckpointTrigger,
        consistency: CheckpointConsistency,
    ) -> AiResult<Self> {
        let name = name.into();
        validate_text("checkpoint name", &name)?;

        Ok(Self {
            name,
            trigger,
            consistency,
            condition: None,
        })
    }

    /// Adds a symbolic checkpoint condition.
    pub fn with_condition(
        mut self,
        condition: TrainingValueRef,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Returns the checkpoint name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns its trigger.
    #[must_use]
    pub const fn trigger(&self) -> CheckpointTrigger {
        self.trigger
    }

    /// Returns its consistency requirement.
    #[must_use]
    pub const fn consistency(&self) -> CheckpointConsistency {
        self.consistency
    }

    /// Returns its optional symbolic condition.
    #[must_use]
    pub fn condition(&self) -> Option<&TrainingValueRef> {
        self.condition.as_ref()
    }
}

// ============================================================================
// REPRODUCIBILITY
// ============================================================================

/// Reproducibility requirement of a training workflow.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TrainingReproducibility {
    /// No additional reproducibility guarantee is declared.
    Unspecified,

    /// Reproduction is required for the same declared inputs,
    /// environment contract and compatible implementation.
    Reproducible,

    /// Deterministic behavior is required.
    Deterministic,

    /// Reproducibility is required within the declared numerical
    /// equivalence contract.
    NumericallyEquivalent,
}

/// Explicit reproducibility metadata.
///
/// Random seeds and initialization policies are symbolic references,
/// not implicitly generated values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingReproducibilityContract {
    requirement: TrainingReproducibility,
    seed: Option<TrainingValueRef>,
    initialization: Option<TrainingValueRef>,
    numerical_contract: Option<TrainingValueRef>,
}

impl TrainingReproducibilityContract {
    /// Creates a reproducibility contract.
    #[must_use]
    pub fn new(
        requirement: TrainingReproducibility,
    ) -> Self {
        Self {
            requirement,
            seed: None,
            initialization: None,
            numerical_contract: None,
        }
    }

    /// Supplies an explicit seed reference.
    #[must_use]
    pub fn with_seed(
        mut self,
        seed: TrainingValueRef,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Supplies an initialization-policy reference.
    #[must_use]
    pub fn with_initialization(
        mut self,
        initialization: TrainingValueRef,
    ) -> Self {
        self.initialization = Some(initialization);
        self
    }

    /// Supplies a numerical-equivalence contract.
    #[must_use]
    pub fn with_numerical_contract(
        mut self,
        contract: TrainingValueRef,
    ) -> Self {
        self.numerical_contract = Some(contract);
        self
    }

    /// Returns the reproducibility requirement.
    #[must_use]
    pub const fn requirement(&self) -> TrainingReproducibility {
        self.requirement
    }

    /// Returns the optional seed.
    #[must_use]
    pub fn seed(&self) -> Option<&TrainingValueRef> {
        self.seed.as_ref()
    }

    /// Returns the optional initialization reference.
    #[must_use]
    pub fn initialization(&self) -> Option<&TrainingValueRef> {
        self.initialization.as_ref()
    }

    /// Returns the optional numerical contract.
    #[must_use]
    pub fn numerical_contract(&self) -> Option<&TrainingValueRef> {
        self.numerical_contract.as_ref()
    }
}

// ============================================================================
// TRAINING LIFECYCLE
// ============================================================================

/// Logical state of a training run.
///
/// This is a state contract, not an execution engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TrainingLifecycle {
    /// Definition exists but has not been validated.
    Defined,

    /// Structural and required semantic validation completed.
    Validated,

    /// Execution has started.
    Running,

    /// Execution has been suspended.
    Paused,

    /// Training completed successfully.
    Completed,

    /// Training failed.
    Failed,

    /// Training was cancelled.
    Cancelled,
}

impl TrainingLifecycle {
    /// Determines whether a transition is permitted.
    ///
    /// The runtime remains responsible for confirming that the
    /// corresponding real-world execution transition occurred.
    #[must_use]
    pub const fn permits(
        self,
        next: Self,
    ) -> bool {
        use TrainingLifecycle::*;

        matches!(
            (self, next),
            (Defined, Validated)
                | (Defined, Failed)
                | (Defined, Cancelled)
                | (Validated, Running)
                | (Validated, Failed)
                | (Validated, Cancelled)
                | (Running, Paused)
                | (Running, Completed)
                | (Running, Failed)
                | (Running, Cancelled)
                | (Paused, Running)
                | (Paused, Failed)
                | (Paused, Cancelled)
        )
    }

    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled
        )
    }
}

// ============================================================================
// TRAINING DEFINITION
// ============================================================================

/// Complete target-independent training workflow description.
///
/// This type contains computational intent and symbolic references.
///
/// It does not contain:
///
/// - Device handles.
/// - Physical addresses.
/// - Backend objects.
/// - Runtime threads.
/// - Allocated tensor buffers.
/// - Hardware identifiers.
/// - Distributed transport handles.
///
/// Model and dataset identities reuse the existing AI identity authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingDefinition {
    id: AiTrainingRunId,
    model: AiModelId,
    datasets: Vec<AiDatasetId>,
    bindings: Vec<TrainingBinding>,
    phases: Vec<TrainingPhase>,
    checkpoints: Vec<TrainingCheckpoint>,
    requirements: AiRequirements,
    reproducibility: TrainingReproducibilityContract,
}

impl TrainingDefinition {
    /// Creates a training definition.
    ///
    /// A definition is initially unvalidated.
    pub fn new(
        id: AiTrainingRunId,
        model: AiModelId,
    ) -> Self {
        Self {
            id,
            model,
            datasets: Vec::new(),
            bindings: Vec::new(),
            phases: Vec::new(),
            checkpoints: Vec::new(),
            requirements: AiRequirements::new(),
            reproducibility:
                TrainingReproducibilityContract::new(
                    TrainingReproducibility::Unspecified,
                ),
        }
    }

    /// Adds a dataset reference.
    #[must_use]
    pub fn with_dataset(
        mut self,
        dataset: AiDatasetId,
    ) -> Self {
        self.datasets.push(dataset);
        self
    }

    /// Adds a named training binding.
    #[must_use]
    pub fn with_binding(
        mut self,
        binding: TrainingBinding,
    ) -> Self {
        self.bindings.push(binding);
        self
    }

    /// Adds a training phase.
    #[must_use]
    pub fn with_phase(
        mut self,
        phase: TrainingPhase,
    ) -> Self {
        self.phases.push(phase);
        self
    }

    /// Adds a checkpoint declaration.
    #[must_use]
    pub fn with_checkpoint(
        mut self,
        checkpoint: TrainingCheckpoint,
    ) -> Self {
        self.checkpoints.push(checkpoint);
        self
    }

    /// Supplies canonical AI requirements.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Supplies a reproducibility contract.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        reproducibility: TrainingReproducibilityContract,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Returns the training run identity.
    #[must_use]
    pub const fn id(&self) -> AiTrainingRunId {
        self.id
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn model(&self) -> AiModelId {
        self.model
    }

    /// Returns dataset identities.
    #[must_use]
    pub fn datasets(&self) -> &[AiDatasetId] {
        &self.datasets
    }

    /// Returns training bindings.
    #[must_use]
    pub fn bindings(&self) -> &[TrainingBinding] {
        &self.bindings
    }

    /// Returns training phases.
    #[must_use]
    pub fn phases(&self) -> &[TrainingPhase] {
        &self.phases
    }

    /// Returns checkpoint declarations.
    #[must_use]
    pub fn checkpoints(&self) -> &[TrainingCheckpoint] {
        &self.checkpoints
    }

    /// Returns canonical AI requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns reproducibility requirements.
    #[must_use]
    pub fn reproducibility(
        &self,
    ) -> &TrainingReproducibilityContract {
        &self.reproducibility
    }

    /// Returns the total number of declared training steps.
    ///
    /// This is a descriptive count, not a capacity limit.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.phases
            .iter()
            .map(|phase| phase.steps.len())
            .sum()
    }

    /// Performs structural validation.
    ///
    /// Symbol resolution, type checking, objective compatibility,
    /// optimizer compatibility, differentiation legality,
    /// capability satisfaction and physical feasibility remain
    /// separate validation responsibilities.
    pub fn validate(&self) -> AiResult<TrainingValidationReport> {
        let mut issues = Vec::new();

        if !self.requirements.is_complete() {
            issues.push(
                "AI requirement analysis is incomplete".to_owned(),
            );
        }

        let mut dataset_ids = BTreeSet::new();

        for dataset in &self.datasets {
            if !dataset_ids.insert(*dataset) {
                issues.push(
                    format!("duplicate dataset identity: {dataset}"),
                );
            }
        }

        let mut binding_keys = BTreeSet::new();

        for binding in &self.bindings {
            let key = (binding.kind, binding.name.clone());

            if !binding_keys.insert(key) {
                issues.push(
                    format!(
                        "duplicate training binding: {}",
                        binding.name,
                    ),
                );
            }
        }

        let mut phase_ids = BTreeSet::new();

        for phase in &self.phases {
            if !phase_ids.insert(phase.id.clone()) {
                issues.push(
                    format!(
                        "duplicate training phase: {}",
                        phase.id,
                    ),
                );
            }
        }

        let mut steps: BTreeMap<
            TrainingStepId,
            &TrainingStep,
        > = BTreeMap::new();

        for phase in &self.phases {
            for step in &phase.steps {
                if let Err(error) = step.validate() {
                    issues.push(error.to_string());
                }

                if steps.insert(step.id.clone(), step).is_some() {
                    issues.push(
                        format!(
                            "duplicate training step identity: {}",
                            step.id,
                        ),
                    );
                }
            }
        }

        self.validate_dependencies(&steps, &mut issues);

        let mut checkpoint_names = BTreeSet::new();

        for checkpoint in &self.checkpoints {
            if !checkpoint_names.insert(checkpoint.name.clone()) {
                issues.push(
                    format!(
                        "duplicate checkpoint name: {}",
                        checkpoint.name,
                    ),
                );
            }
        }

        if issues.is_empty() {
            Ok(TrainingValidationReport {
                run_id: self.id,
                model_id: self.model,
                phase_count: self.phases.len(),
                step_count: steps.len(),
                dataset_count: self.datasets.len(),
                checkpoint_count: self.checkpoints.len(),
                complete: true,
                issues,
            })
        } else {
            Ok(TrainingValidationReport {
                run_id: self.id,
                model_id: self.model,
                phase_count: self.phases.len(),
                step_count: steps.len(),
                dataset_count: self.datasets.len(),
                checkpoint_count: self.checkpoints.len(),
                complete: false,
                issues,
            })
        }
    }

    fn validate_dependencies(
        &self,
        steps: &BTreeMap<TrainingStepId, &TrainingStep>,
        issues: &mut Vec<String>,
    ) {
        // Kahn's algorithm.
        //
        // This validates forward references and cycles without
        // imposing a recursive graph-depth limit.

        let mut indegree: BTreeMap<TrainingStepId, usize> =
            BTreeMap::new();

        let mut dependents: BTreeMap<
            TrainingStepId,
            Vec<TrainingStepId>,
        > = BTreeMap::new();

        for step_id in steps.keys() {
            indegree.insert(step_id.clone(), 0);
        }

        for (step_id, step) in steps {
            for dependency in &step.dependencies {
                if !steps.contains_key(&dependency.step) {
                    issues.push(
                        format!(
                            "step {step_id} references unknown dependency {}",
                            dependency.step,
                        ),
                    );
                    continue;
                }

                if let Some(degree) = indegree.get_mut(step_id) {
                    *degree += 1;
                }

                dependents
                    .entry(dependency.step.clone())
                    .or_default()
                    .push(step_id.clone());
            }
        }

        let mut ready: BTreeSet<TrainingStepId> = indegree
            .iter()
            .filter_map(|(id, degree)| {
                (*degree == 0).then_some(id.clone())
            })
            .collect();

        let mut visited = 0usize;

        while let Some(current) = ready.pop_first() {
            visited += 1;

            if let Some(children) = dependents.get(&current) {
                for child in children {
                    if let Some(degree) = indegree.get_mut(child) {
                        *degree -= 1;

                        if *degree == 0 {
                            ready.insert(child.clone());
                        }
                    }
                }
            }
        }

        if visited != steps.len() {
            issues.push(
                "training dependency graph contains a cycle".to_owned(),
            );
        }
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of structural training validation.
///
/// A complete report establishes only that this module's structural
/// checks succeeded. It is not a certificate of execution feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingValidationReport {
    run_id: AiTrainingRunId,
    model_id: AiModelId,
    phase_count: usize,
    step_count: usize,
    dataset_count: usize,
    checkpoint_count: usize,
    complete: bool,
    issues: Vec<String>,
}

impl TrainingValidationReport {
    /// Returns the training run identity.
    #[must_use]
    pub const fn run_id(&self) -> AiTrainingRunId {
        self.run_id
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.model_id
    }

    /// Returns the phase count.
    #[must_use]
    pub const fn phase_count(&self) -> usize {
        self.phase_count
    }

    /// Returns the step count.
    #[must_use]
    pub const fn step_count(&self) -> usize {
        self.step_count
    }

    /// Returns the dataset count.
    #[must_use]
    pub const fn dataset_count(&self) -> usize {
        self.dataset_count
    }

    /// Returns the checkpoint count.
    #[must_use]
    pub const fn checkpoint_count(&self) -> usize {
        self.checkpoint_count
    }

    /// Returns whether structural validation completed successfully.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns structural validation issues.
    #[must_use]
    pub fn issues(&self) -> &[String] {
        &self.issues
    }

    /// Converts an incomplete report into a structured AI error.
    pub fn require_complete(&self) -> AiResult<()> {
        if self.complete {
            return Ok(());
        }

        let description = self.issues.join("; ");

        Err(
            invalid_training_input(
                format!(
                    "training validation incomplete: {description}"
                ),
            ),
        )
    }
}

// ============================================================================
// TRAINING EXECUTION INTENT
// ============================================================================

/// Abstract execution intent for a training workflow.
///
/// These values describe computational requirements, not hardware choices.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingExecutionIntent {
    run_id: AiTrainingRunId,
    model_id: AiModelId,
    requirements: AiRequirements,
    reproducibility: TrainingReproducibilityContract,
}

impl TrainingExecutionIntent {
    /// Creates an execution intent from a training definition.
    ///
    /// An incomplete training definition cannot produce an accepted intent.
    pub fn from_definition(
        definition: &TrainingDefinition,
    ) -> AiResult<Self> {
        let report = definition.validate()?;
        report.require_complete()?;

        Ok(Self {
            run_id: definition.id,
            model_id: definition.model,
            requirements: definition.requirements.clone(),
            reproducibility: definition.reproducibility.clone(),
        })
    }

    /// Returns the training run identity.
    #[must_use]
    pub const fn run_id(&self) -> AiTrainingRunId {
        self.run_id
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn model_id(&self) -> AiModelId {
        self.model_id
    }

    /// Returns the requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Returns reproducibility requirements.
    #[must_use]
    pub fn reproducibility(
        &self,
    ) -> &TrainingReproducibilityContract {
        &self.reproducibility
    }
}

// ============================================================================
// LIFECYCLE TRANSITIONS
// ============================================================================

/// Tracks declared lifecycle transitions for a training run.
///
/// It does not assert that physical execution has taken place.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrainingLifecycleRecord {
    run_id: AiTrainingRunId,
    state: TrainingLifecycle,
    history: Vec<TrainingLifecycle>,
}

impl TrainingLifecycleRecord {
    /// Creates an initial lifecycle record.
    #[must_use]
    pub fn new(run_id: AiTrainingRunId) -> Self {
        Self {
            run_id,
            state: TrainingLifecycle::Defined,
            history: vec![TrainingLifecycle::Defined],
        }
    }

    /// Returns the training run identity.
    #[must_use]
    pub const fn run_id(&self) -> AiTrainingRunId {
        self.run_id
    }

    /// Returns the current state.
    #[must_use]
    pub const fn state(&self) -> TrainingLifecycle {
        self.state
    }

    /// Returns lifecycle history in transition order.
    #[must_use]
    pub fn history(&self) -> &[TrainingLifecycle] {
        &self.history
    }

    /// Applies a permitted logical lifecycle transition.
    pub fn transition_to(
        &mut self,
        next: TrainingLifecycle,
    ) -> AiResult<()> {
        if !self.state.permits(next) {
            return Err(
                training_error(
                    format!(
                        "invalid training lifecycle transition: {:?} -> {:?}",
                        self.state,
                        next,
                    ),
                ),
            );
        }

        self.state = next;
        self.history.push(next);

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn run_id() -> AiTrainingRunId {
        AiTrainingRunId::new(1).expect("valid run ID")
    }

    fn model_id() -> AiModelId {
        AiModelId::new(1).expect("valid model ID")
    }

    fn dataset_id() -> AiDatasetId {
        AiDatasetId::new(1).expect("valid dataset ID")
    }

    fn step_id(value: &str) -> TrainingStepId {
        TrainingStepId::new(value).expect("valid step ID")
    }

    fn phase_id(value: &str) -> TrainingPhaseId {
        TrainingPhaseId::new(value).expect("valid phase ID")
    }

    fn operation(value: &str) -> TrainingOperation {
        TrainingOperation::new(value).expect("valid operation")
    }

    fn step(value: &str) -> TrainingStep {
        TrainingStep::new(
            step_id(value),
            operation("ai.training.forward"),
        )
    }

    fn definition() -> TrainingDefinition {
        TrainingDefinition::new(run_id(), model_id())
            .with_dataset(dataset_id())
            .with_phase(
                TrainingPhase::new(phase_id("training"))
                    .with_step(step("forward"))
                    .with_step(
                        step("loss")
                            .with_dependency(step_id("forward")),
                    )
                    .with_step(
                        step("gradient")
                            .with_dependency(step_id("loss")),
                    )
                    .with_step(
                        step("update")
                            .with_dependency(step_id("gradient")),
                    ),
            )
    }

    #[test]
    fn constructs_valid_training_definition() {
        let result = definition()
            .validate()
            .expect("validation should run");

        assert!(result.is_complete());
        assert_eq!(result.step_count(), 4);
        assert_eq!(result.phase_count(), 1);
    }

    #[test]
    fn accepts_forward_references() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("optimization"))
                .with_step(
                    step("update")
                        .with_dependency(step_id("gradient")),
                )
                .with_step(
                    step("gradient")
                        .with_dependency(step_id("loss")),
                )
                .with_step(
                    step("loss")
                        .with_dependency(step_id("forward")),
                )
                .with_step(step("forward")),
        );

        let result = training.validate().unwrap();

        assert!(result.is_complete());
    }

    #[test]
    fn rejects_unknown_dependencies() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(
                    step("update")
                        .with_dependency(step_id("missing")),
                ),
        );

        let result = training.validate().unwrap();

        assert!(!result.is_complete());
        assert!(
            result.issues().iter().any(
                |issue| issue.contains("unknown dependency")
            )
        );
    }

    #[test]
    fn rejects_dependency_cycles() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(
                    step("first")
                        .with_dependency(step_id("second")),
                )
                .with_step(
                    step("second")
                        .with_dependency(step_id("first")),
                ),
        );

        let result = training.validate().unwrap();

        assert!(!result.is_complete());
        assert!(
            result.issues().iter().any(
                |issue| issue.contains("cycle")
            )
        );
    }

    #[test]
    fn rejects_duplicate_step_ids() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(step("forward"))
                .with_step(step("forward")),
        );

        assert!(
            !training.validate().unwrap().is_complete()
        );
    }

    #[test]
    fn rejects_duplicate_phase_ids() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training")),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training")),
        );

        assert!(
            !training.validate().unwrap().is_complete()
        );
    }

    #[test]
    fn rejects_duplicate_dataset_ids() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_dataset(dataset_id())
        .with_dataset(dataset_id())
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(step("forward")),
        );

        assert!(
            !training.validate().unwrap().is_complete()
        );
    }

    #[test]
    fn rejects_self_dependency() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(
                    step("forward")
                        .with_dependency(step_id("forward")),
                ),
        );

        assert!(
            !training.validate().unwrap().is_complete()
        );
    }

    #[test]
    fn supports_open_world_training_operations() {
        let operation = TrainingOperation::new(
            "future.photonic.learning.operation",
        )
        .unwrap();

        assert_eq!(
            operation.identity().as_str(),
            "future.photonic.learning.operation"
        );
    }

    #[test]
    fn supports_multiple_checkpoint_policies() {
        let checkpoint = TrainingCheckpoint::new(
            "recovery",
            CheckpointTrigger::Recovery,
            CheckpointConsistency::Consistent,
        )
        .unwrap();

        assert_eq!(
            checkpoint.trigger(),
            CheckpointTrigger::Recovery
        );
    }

    #[test]
    fn requires_explicit_reproducibility_information() {
        let contract = TrainingReproducibilityContract::new(
            TrainingReproducibility::Deterministic,
        )
        .with_seed(
            TrainingValueRef::new("experiment.seed").unwrap(),
        );

        assert_eq!(
            contract.requirement(),
            TrainingReproducibility::Deterministic
        );

        assert!(contract.seed().is_some());
    }

    #[test]
    fn lifecycle_accepts_valid_transitions() {
        let mut record = TrainingLifecycleRecord::new(run_id());

        record
            .transition_to(TrainingLifecycle::Validated)
            .unwrap();

        record
            .transition_to(TrainingLifecycle::Running)
            .unwrap();

        record
            .transition_to(TrainingLifecycle::Completed)
            .unwrap();

        assert_eq!(
            record.state(),
            TrainingLifecycle::Completed
        );

        assert!(record.state().is_terminal());
    }

    #[test]
    fn lifecycle_rejects_invalid_transitions() {
        let mut record = TrainingLifecycleRecord::new(run_id());

        assert!(
            record
                .transition_to(TrainingLifecycle::Completed)
                .is_err()
        );
    }

    #[test]
    fn terminal_lifecycle_cannot_restart() {
        let mut record = TrainingLifecycleRecord::new(run_id());

        record
            .transition_to(TrainingLifecycle::Validated)
            .unwrap();

        record
            .transition_to(TrainingLifecycle::Running)
            .unwrap();

        record
            .transition_to(TrainingLifecycle::Completed)
            .unwrap();

        assert!(
            record
                .transition_to(TrainingLifecycle::Running)
                .is_err()
        );
    }

    #[test]
    fn execution_intent_requires_valid_training() {
        let training = definition();

        let intent = TrainingExecutionIntent::from_definition(
            &training,
        )
        .expect("valid training intent");

        assert_eq!(intent.run_id(), run_id());
        assert_eq!(intent.model_id(), model_id());
    }

    #[test]
    fn invalid_training_cannot_produce_execution_intent() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("training"))
                .with_step(
                    step("update")
                        .with_dependency(step_id("missing")),
                ),
        );

        assert!(
            TrainingExecutionIntent::from_definition(&training)
                .is_err()
        );
    }

    #[test]
    fn supports_multiple_training_phases() {
        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(
            TrainingPhase::new(phase_id("preparation"))
                .with_step(step("prepare")),
        )
        .with_phase(
            TrainingPhase::new(phase_id("optimization"))
                .with_step(
                    step("optimize")
                        .with_dependency(step_id("prepare")),
                ),
        )
        .with_phase(
            TrainingPhase::new(phase_id("evaluation"))
                .with_step(
                    step("evaluate")
                        .with_dependency(step_id("optimize")),
                ),
        );

        assert!(
            training.validate().unwrap().is_complete()
        );
    }

    #[test]
    fn supports_empty_operational_requirements() {
        let training = definition();

        assert!(training.requirements().is_complete());
    }

    #[test]
    fn supports_large_structural_workflows() {
        let mut phase = TrainingPhase::new(
            phase_id("large_workflow"),
        );

        let mut previous: Option<TrainingStepId> = None;

        for index in 0..10_000 {
            let id = step_id(
                &format!("step_{index}"),
            );

            let mut current = TrainingStep::new(
                id.clone(),
                operation("ai.training.forward"),
            );

            if let Some(previous_id) = previous {
                current = current.with_dependency(previous_id);
            }

            phase = phase.with_step(current);
            previous = Some(id);
        }

        let training = TrainingDefinition::new(
            run_id(),
            model_id(),
        )
        .with_phase(phase);

        let result = training.validate().unwrap();

        assert!(result.is_complete());
        assert_eq!(result.step_count(), 10_000);
    }

    #[test]
    fn rejects_empty_identities() {
        assert!(TrainingStepId::new("").is_err());
        assert!(TrainingPhaseId::new("").is_err());
        assert!(TrainingValueRef::new("").is_err());
    }
}
