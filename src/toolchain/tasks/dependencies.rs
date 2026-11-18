
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task Dependency Integration
//!
//! Path: `src/toolchain/tasks/dependencies.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Derives and validates task dependencies from the canonical task input
//! declarations in `tasks::model`, then builds the canonical task graph
//! through `tasks::graph::TaskGraph`.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The mapping between task-output inputs and producer tasks.
//! - Validation of task-output references.
//! - Validation of required-input / optional-output compatibility.
//! - Deterministic input-to-producer bindings.
//! - Construction of the task dependency plan.
//!
//! This module does not own:
//!
//! - Task identities or task definitions.
//! - Task input or output declarations.
//! - Graph storage, graph algorithms or cycle detection.
//! - Package dependency resolution.
//! - Compiler dependency analysis.
//! - Task provider discovery.
//! - Task execution, scheduling or cancellation.
//! - Physical resource allocation.
//! - Hardware discovery or runtime execution.
//!
//! ## Canonical authorities
//!
//! Task definitions and input/output declarations:
//!
//!     crate::toolchain::tasks::model
//!
//! Task graph:
//!
//!     crate::toolchain::build::graph::BuildGraph
//!
//! Task graph access path:
//!
//!     crate::toolchain::tasks::graph::TaskGraph
//!
//! Task identities:
//!
//!     crate::toolchain::ids::TaskId
//!
//! Error and result contracts:
//!
//!     crate::toolchain::errors
//!     crate::toolchain::result
//!
//! ## Dependency direction
//!
//! A task input may refer to an output produced by another task:
//!
//!     consumer task
//!         |
//!         | input references producer.output
//!         v
//!     producer task
//!
//! The canonical graph edge points from the dependent task to its
//! prerequisite:
//!
//!     consumer -> producer
//!
//! The canonical graph supplies dependency-first topological ordering.
//!
//! ## Optional inputs
//!
//! An optional input that references a task output still creates an
//! ordering relationship. The producer must reach a terminal outcome
//! before the consumer is considered, so the consumer cannot observe a
//! partially produced result.
//!
//! However, an optional input does not require the producer to provide
//! that output. The consumer may proceed without the artifact if the
//! producer did not produce it, provided the execution policy permits
//! that outcome.
//!
//! A required input may not reference an output declared optional,
//! because the producer has not promised to produce that output.
//!
//! ## Determinism
//!
//! Definitions, inputs, bindings and graph edges are processed in
//! canonical ordered-map order.
//!
//! Duplicate producer-consumer edges are inserted only once, while
//! every individual input binding is retained.
//!
//! ## Scalability
//!
//! No fixed task count, dependency count, input count, output count,
//! graph depth, domain count, target count or hardware count is imposed.
//!
//! Storage is proportional to the supplied task definitions and
//! task-output bindings. Processing is constrained by available memory,
//! address space and configured operational budgets.
//!
//! Literal infinite memory or execution is not promised.
//!
//! ## Integration
//!
//! `tasks::dependencies` derives the task graph from `TaskDefinition`
//! values. It does not replace `build::graph::BuildGraph`.
//!
//! The task executor must consume the resulting graph and bindings
//! rather than expecting a separate `TaskDefinition::prerequisites()`
//! API or inventing another dependency representation.

use std::collections::{BTreeMap, BTreeSet};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::TaskId;
use crate::toolchain::result::ToolchainResult;

use super::graph::TaskGraph;
use super::model::{TaskDefinition, TaskInputSource};

// ============================================================================
// TASK DEPENDENCY BINDING
// ============================================================================

/// Connects one consumer input to one declared producer output.
///
/// A binding preserves the input-level relationship. Multiple bindings
/// may refer to the same producer task; the graph stores that
/// producer-consumer relationship only once.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskDependencyBinding {
    consumer: TaskId,
    input_name: String,
    producer: TaskId,
    output_name: String,
    required: bool,
}

impl TaskDependencyBinding {
    fn new(
        consumer: TaskId,
        input_name: String,
        producer: TaskId,
        output_name: String,
        required: bool,
    ) -> Self {
        Self {
            consumer,
            input_name,
            producer,
            output_name,
            required,
        }
    }

    /// Returns the task that consumes the input.
    #[must_use]
    pub fn consumer(&self) -> &TaskId {
        &self.consumer
    }

    /// Returns the declared input name on the consumer.
    #[must_use]
    pub fn input_name(&self) -> &str {
        &self.input_name
    }

    /// Returns the task that produces the referenced output.
    #[must_use]
    pub fn producer(&self) -> &TaskId {
        &self.producer
    }

    /// Returns the declared output name on the producer.
    #[must_use]
    pub fn output_name(&self) -> &str {
        &self.output_name
    }

    /// Returns whether the consumer requires this input.
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.required
    }
}

// ============================================================================
// TASK DEPENDENCY PLAN
// ============================================================================

/// Validated relationship between task definitions, task inputs and
/// the canonical task dependency graph.
///
/// The plan stores no competing graph representation. Its graph is the
/// canonical `TaskGraph` alias for `build::graph::BuildGraph`.
///
/// Bindings preserve the relationship between a consumer's named input
/// and a producer's named output. The graph itself stores only task-level
/// prerequisite relationships.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskDependencyPlan {
    graph: TaskGraph,
    bindings_by_consumer: BTreeMap<TaskId, Vec<TaskDependencyBinding>>,
    binding_count: usize,
}

impl TaskDependencyPlan {
    /// Derives a validated task dependency plan from task definitions.
    ///
    /// The supplied map key must match the identity inside each
    /// `TaskDefinition`.
    ///
    /// Every task is inserted into the canonical graph, including tasks
    /// that have no task-output inputs.
    ///
    /// Every `TaskInputSource::TaskOutput` reference is checked against
    /// the producer definition and its declared outputs.
    ///
    /// Multiple inputs from the same producer create one graph edge
    /// but retain separate input bindings.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - A map key differs from its task definition identity.
    /// - A task definition fails its own validation.
    /// - A referenced producer task is missing.
    /// - A referenced producer output is not declared.
    /// - A required input references an optional output.
    /// - A self-dependency or dependency cycle is detected.
    /// - The canonical graph rejects a relationship.
    pub fn from_definitions(
        definitions: &BTreeMap<TaskId, TaskDefinition>,
    ) -> ToolchainResult<Self> {
        let mut graph = TaskGraph::new();

        // First pass: validate and insert every task. This ensures that
        // dependency validation never depends on insertion order.
        for (map_id, definition) in definitions {
            if definition.id() != map_id {
                return Err(validation_error(format!(
                    "Task-definition map key `{map_id}` does not match \
                     the embedded task identity `{}`",
                    definition.id()
                )));
            }

            definition.validate()?;
            graph.add_task(map_id.clone())?;
        }

        let mut bindings_by_consumer =
            BTreeMap::<TaskId, Vec<TaskDependencyBinding>>::new();

        // The graph owns task-level edges. This set prevents multiple
        // named inputs from inserting the same edge more than once.
        let mut inserted_edges = BTreeSet::<(TaskId, TaskId)>::new();

        let mut binding_count = 0usize;

        // Second pass: validate every task-output reference and derive
        // the canonical graph relationships.
        for (consumer_id, consumer_definition) in definitions {
            for input in consumer_definition.inputs().values() {
                let TaskInputSource::TaskOutput {
                    task: producer_id,
                    output: output_name,
                } = input.source()
                else {
                    // Project sources, configuration values, packages
                    // and external artifacts are not task-graph edges.
                    continue;
                };

                if producer_id == consumer_id {
                    return Err(validation_error(format!(
                        "Task `{consumer_id}` cannot consume its own \
                         output `{output_name}`"
                    )));
                }

                let producer_definition =
                    definitions.get(producer_id).ok_or_else(|| {
                        ToolchainError::not_found(format!(
                            "Task `{consumer_id}` input `{}` references \
                             missing producer task `{producer_id}`",
                            input.name()
                        ))
                        .with_operation("tasks.dependencies")
                    })?;

                let output_declaration = producer_definition
                    .outputs()
                    .get(output_name)
                    .ok_or_else(|| {
                        ToolchainError::not_found(format!(
                            "Task `{consumer_id}` input `{}` references \
                             undeclared output `{output_name}` on \
                             producer `{producer_id}`",
                            input.name()
                        ))
                        .with_operation("tasks.dependencies")
                    })?;

                // A required consumer input cannot rely on an output
                // that the producer itself declares optional.
                if input.is_required() && !output_declaration.is_required() {
                    return Err(validation_error(format!(
                        "Required input `{}` on task `{consumer_id}` \
                         references optional output `{output_name}` \
                         on task `{producer_id}`",
                        input.name()
                    )));
                }

                let binding = TaskDependencyBinding::new(
                    consumer_id.clone(),
                    input.name().to_owned(),
                    producer_id.clone(),
                    output_name.clone(),
                    input.is_required(),
                );

                let consumer_bindings = bindings_by_consumer
                    .entry(consumer_id.clone())
                    .or_default();

                consumer_bindings.try_reserve(1).map_err(|error| {
                    allocation_error(
                        "task dependency input bindings",
                        error,
                    )
                })?;

                consumer_bindings.push(binding);

                binding_count = binding_count.checked_add(1).ok_or_else(|| {
                    ToolchainError::resource_exhausted(
                        "Task dependency binding count exceeds the \
                         representable address-space count",
                    )
                    .with_operation("tasks.dependencies")
                })?;

                let edge = (consumer_id.clone(), producer_id.clone());

                if inserted_edges.insert(edge) {
                    graph.add_dependency(consumer_id, producer_id)?;
                }
            }
        }

        // The canonical graph is the sole authority for graph
        // validation and topological ordering.
        graph.validate()?;

        // Force cycle detection during plan construction. A plan is
        // returned only when the graph can be ordered successfully.
        let _ = graph.topological_order()?;

        Ok(Self {
            graph,
            bindings_by_consumer,
            binding_count,
        })
    }

    /// Returns the canonical task dependency graph.
    #[must_use]
    pub fn graph(&self) -> &TaskGraph {
        &self.graph
    }

    /// Consumes the plan and returns its canonical graph.
    ///
    /// Input bindings are discarded. Use this only when the caller
    /// requires graph structure and does not need input resolution.
    #[must_use]
    pub fn into_graph(self) -> TaskGraph {
        self.graph
    }

    /// Returns all input bindings for one consumer task.
    ///
    /// The bindings are ordered by the consumer's declared input name.
    /// An empty slice means the task has no task-output inputs.
    #[must_use]
    pub fn bindings_for(
        &self,
        consumer: &TaskId,
    ) -> &[TaskDependencyBinding] {
        self.bindings_by_consumer
            .get(consumer)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Iterates over every task-output input binding.
    ///
    /// Iteration is deterministic: consumers are ordered by `TaskId`,
    /// then bindings are ordered by input name.
    pub fn bindings(
        &self,
    ) -> impl Iterator<Item = &TaskDependencyBinding> {
        self.bindings_by_consumer
            .values()
            .flat_map(|bindings| bindings.iter())
    }

    /// Returns the number of input-level task-output bindings.
    ///
    /// This may exceed the graph's edge count because multiple inputs
    /// can consume outputs from the same producer.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.binding_count
    }

    /// Returns the number of tasks in the canonical graph.
    #[must_use]
    pub fn task_count(&self) -> usize {
        self.graph.task_count()
    }

    /// Returns the number of unique task-level prerequisite edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.graph.dependency_count()
    }

    /// Returns the direct prerequisites of a task.
    ///
    /// The returned identities are in canonical order.
    /// Returns `None` when the task is not in the graph.
    #[must_use]
    pub fn prerequisites_of(
        &self,
        task: &TaskId,
    ) -> Option<&[TaskId]> {
        self.graph.dependencies_of(task)
    }

    /// Returns deterministic dependency-first execution eligibility
    /// order from the canonical graph.
    ///
    /// This is an ordering result, not an execution or scheduling
    /// operation.
    pub fn topological_order(&self) -> ToolchainResult<Vec<TaskId>> {
        self.graph.topological_order()
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Derives the canonical task dependency graph from task definitions.
///
/// This convenience function discards the input-level bindings after
/// validation. Call `TaskDependencyPlan::from_definitions` when the
/// executor also needs to resolve named producer outputs.
pub fn derive_task_graph(
    definitions: &BTreeMap<TaskId, TaskDefinition>,
) -> ToolchainResult<TaskGraph> {
    Ok(TaskDependencyPlan::from_definitions(definitions)?.into_graph())
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("tasks.dependencies")
}

fn allocation_error(
    subject: &str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "Unable to reserve memory for {subject}: {error}"
    ))
    .with_operation("tasks.dependencies")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::{
        IdNamespace,
        TaskId,
        WorkspaceId,
    };

    use crate::toolchain::tasks::model::{
        CancellationBehavior,
        ConcurrencySafety,
        Resumability,
        RetrySafety,
        TaskExecutionContract,
        TaskInput,
        TaskKind,
        TaskOutput,
    };

    fn task_id(value: u128) -> TaskId {
        TaskId::new(
            IdNamespace::new("test").expect("valid namespace"),
            value,
        )
        .expect("valid task identity")
    }

    fn workspace_id() -> WorkspaceId {
        WorkspaceId::new(
            IdNamespace::new("test").expect("valid namespace"),
            1,
        )
        .expect("valid workspace identity")
    }

    fn definition(id: TaskId) -> TaskDefinition {
        TaskDefinition::new(
            id,
            TaskKind::new(
                "zamani.toolchain",
                "test",
                "1",
            )
            .expect("valid task kind"),
            workspace_id(),
            TaskExecutionContract::new(
                ConcurrencySafety::Concurrent,
                CancellationBehavior::Cooperative,
                RetrySafety::Safe,
                Resumability::NotResumable,
                true,
            ),
        )
    }

    fn required_output(name: &str) -> TaskOutput {
        TaskOutput::new(name, "test.artifact", true)
            .expect("valid output declaration")
    }

    fn optional_output(name: &str) -> TaskOutput {
        TaskOutput::new(name, "test.artifact", false)
            .expect("valid output declaration")
    }

    fn task_output_input(
        name: &str,
        producer: TaskId,
        output: &str,
        required: bool,
    ) -> TaskInput {
        TaskInput::new(
            name,
            TaskInputSource::TaskOutput {
                task: producer,
                output: output.to_owned(),
            },
            required,
        )
        .expect("valid task input")
    }

    fn definitions(
        tasks: impl IntoIterator<Item = TaskDefinition>,
    ) -> BTreeMap<TaskId, TaskDefinition> {
        tasks
            .into_iter()
            .map(|task| (task.id().clone(), task))
            .collect()
    }

    #[test]
    fn derives_dependency_from_task_output_input() {
        let producer_id = task_id(1);
        let consumer_id = task_id(2);

        let mut producer = definition(producer_id.clone());
        producer
            .add_output(required_output("object"))
            .expect("add output");

        let mut consumer = definition(consumer_id.clone());
        consumer
            .add_input(task_output_input(
                "object_input",
                producer_id.clone(),
                "object",
                true,
            ))
            .expect("add input");

        let definitions = definitions([producer, consumer]);

        let plan = TaskDependencyPlan::from_definitions(&definitions)
            .expect("valid dependency plan");

        assert_eq!(plan.task_count(), 2);
        assert_eq!(plan.dependency_count(), 1);
        assert_eq!(
            plan.prerequisites_of(&consumer_id),
            Some([producer_id.clone()].as_slice())
        );

        let order = plan.topological_order().expect("valid order");

        let producer_position = order
            .iter()
            .position(|id| id == &producer_id)
            .expect("producer is present");

        let consumer_position = order
            .iter()
            .position(|id| id == &consumer_id)
            .expect("consumer is present");

        assert!(producer_position < consumer_position);
    }

    #[test]
    fn preserves_multiple_input_bindings_but_deduplicates_graph_edge() {
        let producer_id = task_id(1);
        let consumer_id = task_id(2);

        let mut producer = definition(producer_id.clone());
        producer
            .add_output(required_output("object"))
            .expect("add object output");
        producer
            .add_output(required_output("metadata"))
            .expect("add metadata output");

        let mut consumer = definition(consumer_id.clone());
        consumer
            .add_input(task_output_input(
                "object_input",
                producer_id.clone(),
                "object",
                true,
            ))
            .expect("add first input");
        consumer
            .add_input(task_output_input(
                "metadata_input",
                producer_id,
                "metadata",
                true,
            ))
            .expect("add second input");

        let plan = TaskDependencyPlan::from_definitions(
            &definitions([producer, consumer]),
        )
        .expect("valid dependency plan");

        assert_eq!(plan.dependency_count(), 1);
        assert_eq!(plan.binding_count(), 2);
        assert_eq!(plan.bindings_for(&consumer_id).len(), 2);
    }

    #[test]
    fn rejects_missing_producer_task() {
        let missing_producer = task_id(1);
        let consumer_id = task_id(2);

        let mut consumer = definition(consumer_id);
        consumer
            .add_input(task_output_input(
                "input",
                missing_producer,
                "output",
                true,
            ))
            .expect("add input");

        let result = TaskDependencyPlan::from_definitions(
            &definitions([consumer]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_producer_output() {
        let producer_id = task_id(1);
        let consumer_id = task_id(2);

        let producer = definition(producer_id.clone());

        let mut consumer = definition(consumer_id);
        consumer
            .add_input(task_output_input(
                "input",
                producer_id,
                "missing_output",
                true,
            ))
            .expect("add input");

        let result = TaskDependencyPlan::from_definitions(
            &definitions([producer, consumer]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_required_input_from_optional_output() {
        let producer_id = task_id(1);
        let consumer_id = task_id(2);

        let mut producer = definition(producer_id.clone());
        producer
            .add_output(optional_output("possibly_absent"))
            .expect("add optional output");

        let mut consumer = definition(consumer_id);
        consumer
            .add_input(task_output_input(
                "required_input",
                producer_id,
                "possibly_absent",
                true,
            ))
            .expect("add required input");

        let result = TaskDependencyPlan::from_definitions(
            &definitions([producer, consumer]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_optional_input_from_optional_output() {
        let producer_id = task_id(1);
        let consumer_id = task_id(2);

        let mut producer = definition(producer_id.clone());
        producer
            .add_output(optional_output("possibly_absent"))
            .expect("add optional output");

        let mut consumer = definition(consumer_id.clone());
        consumer
            .add_input(task_output_input(
                "optional_input",
                producer_id,
                "possibly_absent",
                false,
            ))
            .expect("add optional input");

        let plan = TaskDependencyPlan::from_definitions(
            &definitions([producer, consumer]),
        )
        .expect("optional binding is valid");

        assert_eq!(plan.dependency_count(), 1);
        assert_eq!(plan.binding_count(), 1);
        assert!(!plan.bindings_for(&consumer_id)[0].is_required());
    }

    #[test]
    fn rejects_dependency_cycles() {
        let first_id = task_id(1);
        let second_id = task_id(2);

        let mut first = definition(first_id.clone());
        first
            .add_output(required_output("first_output"))
            .expect("add first output");
        first
            .add_input(task_output_input(
                "from_second",
                second_id.clone(),
                "second_output",
                true,
            ))
            .expect("add first input");

        let mut second = definition(second_id);
        second
            .add_output(required_output("second_output"))
            .expect("add second output");
        second
            .add_input(task_output_input(
                "from_first",
                first_id,
                "first_output",
                true,
            ))
            .expect("add second input");

        let result = TaskDependencyPlan::from_definitions(
            &definitions([first, second]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn independent_tasks_are_retained_in_the_graph() {
        let first = definition(task_id(1));
        let second = definition(task_id(2));
        let third = definition(task_id(3));

        let plan = TaskDependencyPlan::from_definitions(
            &definitions([first, second, third]),
        )
        .expect("independent tasks form a valid graph");

        assert_eq!(plan.task_count(), 3);
        assert_eq!(plan.dependency_count(), 0);
        assert_eq!(plan.binding_count(), 0);
        assert_eq!(plan.topological_order().expect("valid order").len(), 3);
    }

    #[test]
    fn rejects_mismatched_definition_map_identity() {
        let map_id = task_id(1);
        let embedded_id = task_id(2);

        let task = definition(embedded_id);

        let mut definitions = BTreeMap::new();
        definitions.insert(map_id, task);

        assert!(TaskDependencyPlan::from_definitions(&definitions).is_err());
    }

    #[test]
    fn non_task_inputs_do_not_create_graph_edges() {
        let task = definition(task_id(1));

        let plan = TaskDependencyPlan::from_definitions(
            &definitions([task]),
        )
        .expect("valid task without task-output dependencies");

        assert_eq!(plan.task_count(), 1);
        assert_eq!(plan.dependency_count(), 0);
        assert_eq!(plan.binding_count(), 0);
    }
}
