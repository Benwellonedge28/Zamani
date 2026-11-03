
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Meta-Learning Contracts
//!
//! Path: `src/ai/learning/meta.rs`
//! Rust baseline: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines target-independent contracts for meta-learning.
//!
//! Meta-learning describes learning systems whose computational objective
//! includes learning how to learn, adapting learning behavior across tasks,
//! or optimizing reusable learning procedures.
//!
//! This module describes:
//!
//! - Meta-learning identities.
//! - Meta-learning algorithm references.
//! - Inner-loop learning contracts.
//! - Outer-loop learning contracts.
//! - Task-distribution references.
//! - Meta-objective references.
//! - Symbolic update-count references.
//! - Optimizer and gradient references.
//! - Meta-learning parameter bindings.
//! - Training-run lineage.
//! - Canonical AI requirements.
//! - Structural and registry-backed validation.
//!
//! It does not execute meta-learning.
//!
//! ## Ownership
//!
//! This module owns meta-learning-specific declarative contracts.
//!
//! It reuses:
//!
//! - `QualifiedName` from frontend AI semantics.
//! - `AiModelId` and `AiTrainingRunId` from AI identifiers.
//! - `LearningAlgorithmId` and `LearningAlgorithmVersion` from algorithms.
//! - `LearningAlgorithmRegistry` for exact algorithm resolution.
//! - `TrainingValueRef` for symbolic training values.
//! - `AiRequirements` for computational requirements.
//! - `AiError` for structured failures.
//!
//! ## Does not own
//!
//! - Source syntax or AST construction.
//! - Name resolution.
//! - Type checking.
//! - Tensor storage.
//! - Gradient computation.
//! - Optimizer implementations.
//! - Numerical kernels.
//! - Training execution.
//! - Model parameter allocation.
//! - Dataset storage.
//! - Hardware discovery.
//! - Device selection.
//! - Resource allocation.
//! - Scheduling or placement.
//! - Distributed communication.
//! - Runtime execution.
//! - Backend-specific implementations.
//! - Universal IR construction.
//!
//! ## Grammar integration
//!
//! The existing grammar already describes meta-learning as a learning
//! and training concept:
//!
//! - `grammar/ai/training.g4`
//! - `grammar/ai/learning.g4`
//! - `grammar/ai/ai.g4`
//!
//! These grammar files remain authoritative for source syntax.
//!
//! The canonical frontend AST adapter must translate validated source
//! constructs into the contracts defined here.
//!
//! This file does not define a second meta-learning grammar or AST.
//!
//! ## POCO-REAF
//!
//! Meta-learning contracts describe computational intent rather than
//! physical execution.
//!
//! The same definition may be realized on compatible:
//!
//! - Embedded systems.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - AI accelerators.
//! - Quantum-assisted systems.
//! - Neuromorphic systems.
//! - Heterogeneous systems.
//! - Distributed systems.
//! - Future computational architectures.
//!
//! There is no language-level maximum for:
//!
//! - Meta-learning definitions.
//! - Tasks.
//! - Training runs.
//! - Parameter bindings.
//! - Inner-loop updates.
//! - Outer-loop updates.
//! - Model parameters.
//! - Workers.
//! - Accelerators.
//! - Devices.
//! - Distributed nodes.
//!
//! Counts that depend on runtime values are represented through symbolic
//! references rather than arbitrary fixed integer ceilings.
//!
//! Actual resource feasibility belongs to downstream authorities.
//!
//! Literal infinite computation is not promised.
//!
//! ## Validation guarantees
//!
//! Local validation establishes structural consistency only.
//!
//! Registry-backed validation additionally establishes that every referenced
//! learning algorithm is registered at the exact requested version.
//!
//! Neither validation stage establishes resource feasibility, authorization,
//! target compatibility, implementation availability, or execution success.
//!
//! ## Freeze criteria
//!
//! - Canonical AI identities and errors are reused.
//! - Algorithm references use exact versions.
//! - Inner and outer learning loops have independent contracts.
//! - Task distributions and objectives are explicit references.
//! - Dynamic update counts remain symbolic.
//! - Duplicate parameter bindings are rejected.
//! - Training lineage is retained.
//! - Requirements are preserved without being silently weakened.
//! - No execution or hardware selection occurs.
//! - No arbitrary workload ceilings exist.
//! - No unsafe Rust is used.

use std::collections::{BTreeMap, BTreeSet};

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::ai::ids::{
    AiModelId,
    AiTrainingRunId,
};

use crate::ai::learning::algorithms::{
    LearningAlgorithmId,
    LearningAlgorithmRegistry,
    LearningAlgorithmVersion,
};

use crate::ai::requirements::AiRequirements;

use crate::ai::learning::training::TrainingValueRef;

use crate::frontend::semantic::domains::ai::QualifiedName;


// ============================================================================
// VALIDATION
// ============================================================================

fn invalid_input(
    message: impl Into<String>,
) -> AiError {
    AiError::new(
        AiErrorKind::InvalidInput,
        message,
    )
    .with_phase(AiErrorPhase::Validation)
}

fn validate_text(
    field: &str,
    value: &str,
) -> AiResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{field} cannot be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{field} cannot contain control characters"),
        ));
    }

    Ok(())
}

fn validate_qualified_name(
    field: &str,
    value: &QualifiedName,
) -> AiResult<()> {
    validate_text(
        &format!("{field} namespace"),
        &value.namespace,
    )?;

    validate_text(
        &format!("{field} name"),
        &value.name,
    )
}


// ============================================================================
// META-LEARNING IDENTITY
// ============================================================================

/// Stable logical identity of a meta-learning definition.
///
/// This identity is scoped to the meta-learning domain. It is not a
/// replacement for the universal ZUIR identity system or a persistent
/// content fingerprint.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaLearningId(String);

impl MetaLearningId {
    /// Construct a validated meta-learning identity.
    pub fn new(
        value: impl Into<String>,
    ) -> AiResult<Self> {
        let value = value.into();

        validate_text(
            "meta-learning identity",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the identity as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MetaLearningId {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}


// ============================================================================
// LEARNING ALGORITHM REFERENCE
// ============================================================================

/// Exact reference to a registered learning algorithm.
///
/// The identity and version are resolved through the existing
/// `LearningAlgorithmRegistry`.
///
/// There is deliberately no implicit selection of the newest version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaLearningAlgorithmRef {
    identity: LearningAlgorithmId,
    version: LearningAlgorithmVersion,
}

impl MetaLearningAlgorithmRef {
    /// Construct an exact learning algorithm reference.
    pub fn new(
        identity: LearningAlgorithmId,
        version: LearningAlgorithmVersion,
    ) -> AiResult<Self> {
        validate_qualified_name(
            "learning algorithm",
            &identity,
        )?;

        Ok(Self {
            identity,
            version,
        })
    }

    /// Return the canonical learning algorithm identity.
    #[must_use]
    pub fn identity(&self) -> &LearningAlgorithmId {
        &self.identity
    }

    /// Return the exact requested algorithm version.
    #[must_use]
    pub const fn version(&self) -> LearningAlgorithmVersion {
        self.version
    }

    /// Verify that this exact algorithm version is registered.
    ///
    /// No fallback to another version is permitted.
    pub fn validate_registered(
        &self,
        registry: &LearningAlgorithmRegistry,
    ) -> AiResult<()> {
        if registry.contains(
            &self.identity,
            self.version,
        ) {
            return Ok(());
        }

        Err(invalid_input(
            format!(
                "learning algorithm {} at version {} is not registered",
                self.identity,
                self.version,
            ),
        ))
    }
}


// ============================================================================
// META-LEARNING LOOP CONTRACT
// ============================================================================

/// Describes one learning loop used by a meta-learning definition.
///
/// A loop describes its algorithm, objective, optional update-count
/// expression, optimizer reference, gradient reference, and data flow.
///
/// Update counts are symbolic references because their value may depend
/// on task properties, learned schedules, or runtime conditions.
///
/// This structure does not execute the loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaLearningLoop {
    algorithm: MetaLearningAlgorithmRef,
    objective: TrainingValueRef,
    update_count: Option<TrainingValueRef>,
    optimizer: Option<TrainingValueRef>,
    gradient: Option<TrainingValueRef>,
    inputs: Vec<TrainingValueRef>,
    outputs: Vec<TrainingValueRef>,
}

impl MetaLearningLoop {
    /// Construct a loop with its algorithm and objective.
    pub fn new(
        algorithm: MetaLearningAlgorithmRef,
        objective: TrainingValueRef,
    ) -> Self {
        Self {
            algorithm,
            objective,
            update_count: None,
            optimizer: None,
            gradient: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// Set the symbolic update-count expression.
    #[must_use]
    pub fn with_update_count(
        mut self,
        value: TrainingValueRef,
    ) -> Self {
        self.update_count = Some(value);
        self
    }

    /// Set the optimizer reference.
    #[must_use]
    pub fn with_optimizer(
        mut self,
        value: TrainingValueRef,
    ) -> Self {
        self.optimizer = Some(value);
        self
    }

    /// Set the gradient reference.
    #[must_use]
    pub fn with_gradient(
        mut self,
        value: TrainingValueRef,
    ) -> Self {
        self.gradient = Some(value);
        self
    }

    /// Append an input reference.
    #[must_use]
    pub fn with_input(
        mut self,
        value: TrainingValueRef,
    ) -> Self {
        self.inputs.push(value);
        self
    }

    /// Append an output reference.
    #[must_use]
    pub fn with_output(
        mut self,
        value: TrainingValueRef,
    ) -> Self {
        self.outputs.push(value);
        self
    }

    /// Return the exact algorithm reference.
    #[must_use]
    pub fn algorithm(&self) -> &MetaLearningAlgorithmRef {
        &self.algorithm
    }

    /// Return the objective reference.
    #[must_use]
    pub fn objective(&self) -> &TrainingValueRef {
        &self.objective
    }

    /// Return the optional symbolic update count.
    #[must_use]
    pub fn update_count(&self) -> Option<&TrainingValueRef> {
        self.update_count.as_ref()
    }

    /// Return the optional optimizer reference.
    #[must_use]
    pub fn optimizer(&self) -> Option<&TrainingValueRef> {
        self.optimizer.as_ref()
    }

    /// Return the optional gradient reference.
    #[must_use]
    pub fn gradient(&self) -> Option<&TrainingValueRef> {
        self.gradient.as_ref()
    }

    /// Return input references in declaration order.
    #[must_use]
    pub fn inputs(&self) -> &[TrainingValueRef] {
        &self.inputs
    }

    /// Return output references in declaration order.
    #[must_use]
    pub fn outputs(&self) -> &[TrainingValueRef] {
        &self.outputs
    }

    /// Validate the local loop structure.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "loop objective reference",
            self.objective.as_str(),
        )?;

        if let Some(value) = &self.update_count {
            validate_text(
                "loop update-count reference",
                value.as_str(),
            )?;
        }

        if let Some(value) = &self.optimizer {
            validate_text(
                "loop optimizer reference",
                value.as_str(),
            )?;
        }

        if let Some(value) = &self.gradient {
            validate_text(
                "loop gradient reference",
                value.as_str(),
            )?;
        }

        for value in self.inputs.iter().chain(self.outputs.iter()) {
            validate_text(
                "loop value reference",
                value.as_str(),
            )?;
        }

        Ok(())
    }

    /// Validate that this loop's exact algorithm version is registered.
    pub fn validate_registered(
        &self,
        registry: &LearningAlgorithmRegistry,
    ) -> AiResult<()> {
        self.algorithm.validate_registered(registry)
    }
}


// ============================================================================
// META-LEARNING DEFINITION
// ============================================================================

/// Complete declarative meta-learning contract.
///
/// The definition connects:
///
/// - A model being adapted or optimized.
/// - A meta-learning algorithm.
/// - A distribution of tasks.
/// - An inner learning loop.
/// - An outer learning loop.
/// - A meta-objective.
/// - Optional symbolic parameter bindings.
/// - Related training runs.
/// - Canonical AI requirements.
///
/// The definition is target-independent and does not perform training.
#[derive(Clone, Debug)]
pub struct MetaLearningDefinition {
    id: MetaLearningId,
    model: AiModelId,
    algorithm: MetaLearningAlgorithmRef,
    task_distribution: TrainingValueRef,
    meta_objective: TrainingValueRef,
    inner_loop: MetaLearningLoop,
    outer_loop: MetaLearningLoop,
    parameter_bindings: BTreeMap<String, TrainingValueRef>,
    training_runs: BTreeSet<AiTrainingRunId>,
    requirements: AiRequirements,
    description: Option<String>,
}

impl MetaLearningDefinition {
    /// Construct a meta-learning definition.
    ///
    /// The inner and outer loops are explicit so that their algorithms,
    /// objectives, and data-flow contracts cannot be confused.
    pub fn new(
        id: MetaLearningId,
        model: AiModelId,
        algorithm: MetaLearningAlgorithmRef,
        task_distribution: TrainingValueRef,
        meta_objective: TrainingValueRef,
        inner_loop: MetaLearningLoop,
        outer_loop: MetaLearningLoop,
    ) -> Self {
        Self {
            id,
            model,
            algorithm,
            task_distribution,
            meta_objective,
            inner_loop,
            outer_loop,
            parameter_bindings: BTreeMap::new(),
            training_runs: BTreeSet::new(),
            requirements: AiRequirements::new(),
            description: None,
        }
    }

    /// Add a named symbolic parameter binding.
    ///
    /// Duplicate names are rejected rather than silently overwritten.
    pub fn add_parameter_binding(
        &mut self,
        name: impl Into<String>,
        value: TrainingValueRef,
    ) -> AiResult<()> {
        let name = name.into();

        validate_text(
            "meta-learning parameter name",
            &name,
        )?;

        validate_text(
            "meta-learning parameter reference",
            value.as_str(),
        )?;

        if self.parameter_bindings.contains_key(&name) {
            return Err(invalid_input(
                format!(
                    "duplicate meta-learning parameter binding: {name}"
                ),
            ));
        }

        self.parameter_bindings.insert(name, value);

        Ok(())
    }

    /// Add a related training-run identity.
    ///
    /// Duplicate references are idempotently ignored because this collection
    /// represents a set of lineage references.
    #[must_use]
    pub fn with_training_run(
        mut self,
        run: AiTrainingRunId,
    ) -> Self {
        self.training_runs.insert(run);
        self
    }

    /// Replace the canonical AI requirements.
    ///
    /// Requirements are preserved exactly. This method does not weaken,
    /// reinterpret, or prove their feasibility.
    #[must_use]
    pub fn with_requirements(
        mut self,
        requirements: AiRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Attach a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> AiResult<Self> {
        let description = description.into();

        validate_text(
            "meta-learning description",
            &description,
        )?;

        self.description = Some(description);

        Ok(self)
    }

    /// Return the meta-learning identity.
    #[must_use]
    pub fn id(&self) -> &MetaLearningId {
        &self.id
    }

    /// Return the model identity.
    #[must_use]
    pub const fn model(&self) -> AiModelId {
        self.model
    }

    /// Return the meta-learning algorithm reference.
    #[must_use]
    pub fn algorithm(&self) -> &MetaLearningAlgorithmRef {
        &self.algorithm
    }

    /// Return the task-distribution reference.
    #[must_use]
    pub fn task_distribution(&self) -> &TrainingValueRef {
        &self.task_distribution
    }

    /// Return the meta-objective reference.
    #[must_use]
    pub fn meta_objective(&self) -> &TrainingValueRef {
        &self.meta_objective
    }

    /// Return the inner learning loop.
    #[must_use]
    pub fn inner_loop(&self) -> &MetaLearningLoop {
        &self.inner_loop
    }

    /// Return the outer learning loop.
    #[must_use]
    pub fn outer_loop(&self) -> &MetaLearningLoop {
        &self.outer_loop
    }

    /// Return parameter bindings in deterministic key order.
    #[must_use]
    pub fn parameter_bindings(
        &self,
    ) -> &BTreeMap<String, TrainingValueRef> {
        &self.parameter_bindings
    }

    /// Return associated training-run identities.
    #[must_use]
    pub fn training_runs(
        &self,
    ) -> &BTreeSet<AiTrainingRunId> {
        &self.training_runs
    }

    /// Return the canonical AI requirements.
    #[must_use]
    pub fn requirements(&self) -> &AiRequirements {
        &self.requirements
    }

    /// Return the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Validate local structural invariants.
    ///
    /// This does not resolve symbolic values or prove that the referenced
    /// model, datasets, objectives, or training runs exist.
    pub fn validate(&self) -> AiResult<()> {
        validate_text(
            "task-distribution reference",
            self.task_distribution.as_str(),
        )?;

        validate_text(
            "meta-objective reference",
            self.meta_objective.as_str(),
        )?;

        self.inner_loop.validate()?;
        self.outer_loop.validate()?;

        for (name, value) in &self.parameter_bindings {
            validate_text(
                "meta-learning parameter name",
                name,
            )?;

            validate_text(
                "meta-learning parameter reference",
                value.as_str(),
            )?;
        }

        if let Some(description) = &self.description {
            validate_text(
                "meta-learning description",
                description,
            )?;
        }

        Ok(())
    }

    /// Validate every exact algorithm reference against the supplied
    /// learning algorithm registry.
    ///
    /// This method performs no algorithm selection and does not silently
    /// substitute a compatible version.
    pub fn validate_registered_algorithms(
        &self,
        registry: &LearningAlgorithmRegistry,
    ) -> AiResult<()> {
        self.algorithm.validate_registered(registry)?;
        self.inner_loop.validate_registered(registry)?;
        self.outer_loop.validate_registered(registry)?;

        Ok(())
    }

    /// Validate both local structure and exact algorithm registration.
    pub fn validate_with_registry(
        &self,
        registry: &LearningAlgorithmRegistry,
    ) -> AiResult<()> {
        self.validate()?;
        self.validate_registered_algorithms(registry)?;

        Ok(())
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::learning::algorithms::{
        AlgorithmSignature,
        LearningAlgorithmDescriptor,
    };

    fn qualified(
        namespace: &str,
        name: &str,
    ) -> QualifiedName {
        QualifiedName::new(namespace, name)
            .expect("valid qualified name")
    }

    fn algorithm_ref(
        name: &str,
    ) -> MetaLearningAlgorithmRef {
        MetaLearningAlgorithmRef::new(
            qualified("zamani.ai.learning", name),
            LearningAlgorithmVersion::new(1, 0, 0),
        )
        .expect("valid algorithm reference")
    }

    fn value(
        name: &str,
    ) -> TrainingValueRef {
        TrainingValueRef::new(name)
            .expect("valid training value reference")
    }

    fn loop_contract(
        algorithm: &str,
        objective: &str,
    ) -> MetaLearningLoop {
        MetaLearningLoop::new(
            algorithm_ref(algorithm),
            value(objective),
        )
    }

    fn definition() -> MetaLearningDefinition {
        MetaLearningDefinition::new(
            MetaLearningId::new("meta_experiment")
                .expect("valid identity"),
            AiModelId::new(1)
                .expect("valid model identity"),
            algorithm_ref("meta_optimizer"),
            value("task_distribution"),
            value("meta_objective"),
            loop_contract("inner_learner", "task_loss"),
            loop_contract("outer_learner", "meta_loss"),
        )
    }

    fn register_algorithm(
        registry: &mut LearningAlgorithmRegistry,
        name: &str,
    ) {
        let identity = qualified(
            "zamani.ai.learning",
            name,
        );

        let family = qualified(
            "zamani.ai.learning.family",
            "meta_learning",
        );

        let descriptor = LearningAlgorithmDescriptor::new(
            identity,
            LearningAlgorithmVersion::new(1, 0, 0),
            family,
            AlgorithmSignature::default(),
        )
        .expect("valid algorithm descriptor");

        registry
            .register(descriptor)
            .expect("algorithm registration");
    }

    #[test]
    fn rejects_empty_meta_learning_identity() {
        assert!(
            MetaLearningId::new(" ").is_err()
        );
    }

    #[test]
    fn constructs_valid_meta_learning_definition() {
        let definition = definition();

        assert!(
            definition.validate().is_ok()
        );
    }

    #[test]
    fn supports_symbolic_update_counts() {
        let loop_contract = loop_contract(
            "inner_learner",
            "task_loss",
        )
        .with_update_count(
            value("task_dependent_update_count"),
        );

        assert_eq!(
            loop_contract
                .update_count()
                .expect("update count")
                .as_str(),
            "task_dependent_update_count",
        );

        assert!(
            loop_contract.validate().is_ok()
        );
    }

    #[test]
    fn supports_optimizer_and_gradient_references() {
        let loop_contract = loop_contract(
            "inner_learner",
            "task_loss",
        )
        .with_optimizer(value("optimizer"))
        .with_gradient(value("gradient"));

        assert!(
            loop_contract.optimizer().is_some()
        );

        assert!(
            loop_contract.gradient().is_some()
        );
    }

    #[test]
    fn preserves_ordered_loop_inputs_and_outputs() {
        let loop_contract = loop_contract(
            "inner_learner",
            "task_loss",
        )
        .with_input(value("task_data"))
        .with_input(value("initial_parameters"))
        .with_output(value("adapted_parameters"));

        assert_eq!(
            loop_contract.inputs().len(),
            2,
        );

        assert_eq!(
            loop_contract.outputs().len(),
            1,
        );
    }

    #[test]
    fn rejects_duplicate_parameter_bindings() {
        let mut definition = definition();

        definition
            .add_parameter_binding(
                "learning_rate",
                value("initial_learning_rate"),
            )
            .expect("first binding");

        assert!(
            definition
                .add_parameter_binding(
                    "learning_rate",
                    value("other_learning_rate"),
                )
                .is_err()
        );
    }

    #[test]
    fn supports_training_run_lineage() {
        let definition = definition()
            .with_training_run(
                AiTrainingRunId::new(10)
                    .expect("valid training run"),
            )
            .with_training_run(
                AiTrainingRunId::new(11)
                    .expect("valid training run"),
            );

        assert_eq!(
            definition.training_runs().len(),
            2,
        );
    }

    #[test]
    fn rejects_unregistered_algorithm_versions() {
        let definition = definition();
        let registry = LearningAlgorithmRegistry::new();

        assert!(
            definition
                .validate_registered_algorithms(&registry)
                .is_err()
        );
    }

    #[test]
    fn resolves_exact_registered_algorithm_versions() {
        let mut registry = LearningAlgorithmRegistry::new();

        register_algorithm(
            &mut registry,
            "meta_optimizer",
        );

        register_algorithm(
            &mut registry,
            "inner_learner",
        );

        register_algorithm(
            &mut registry,
            "outer_learner",
        );

        let definition = definition();

        assert!(
            definition
                .validate_with_registry(&registry)
                .is_ok()
        );
    }

    #[test]
    fn does_not_substitute_another_algorithm_version() {
        let mut registry = LearningAlgorithmRegistry::new();

        let identity = qualified(
            "zamani.ai.learning",
            "meta_optimizer",
        );

        let descriptor = LearningAlgorithmDescriptor::new(
            identity,
            LearningAlgorithmVersion::new(2, 0, 0),
            qualified(
                "zamani.ai.learning.family",
                "meta_learning",
            ),
            AlgorithmSignature::default(),
        )
        .expect("valid descriptor");

        registry
            .register(descriptor)
            .expect("registration");

        assert!(
            algorithm_ref("meta_optimizer")
                .validate_registered(&registry)
                .is_err()
        );
    }

    #[test]
    fn accepts_large_dynamic_parameter_collections() {
        let mut definition = definition();

        for index in 0..1_000 {
            definition
                .add_parameter_binding(
                    format!("parameter_{index}"),
                    value(&format!("symbolic_parameter_{index}")),
                )
                .expect("unique parameter binding");
        }

        assert_eq!(
            definition.parameter_bindings().len(),
            1_000,
        );

        assert!(
            definition.validate().is_ok()
        );
    }

    #[test]
    fn rejects_control_characters_in_descriptions() {
        let result = definition()
            .with_description("invalid\n description");

        assert!(result.is_err());
    }
}
