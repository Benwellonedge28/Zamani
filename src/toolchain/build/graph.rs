
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Dependency Graph
//!
//! Path: `src/toolchain/build/graph.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the deterministic dependency graph for tasks belonging to
//! one logical Zamani toolchain build.
//!
//! A build graph determines which build tasks must complete before
//! other build tasks become eligible for execution.
//!
//! It is an orchestration data structure, not an execution scheduler.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Build-task graph structure.
//! - Task membership in a build graph.
//! - Task prerequisite relationships.
//! - Dependency insertion and removal.
//! - Structural graph validation.
//! - Dependency-cycle prevention.
//! - Cycle-path diagnostics.
//! - Deterministic dependency-first topological ordering.
//! - Deterministic dependency-ready waves.
//!
//! This module does not own:
//!
//! - Task identity allocation.
//! - Task metadata or execution state.
//! - Package dependency resolution.
//! - Compiler dependency discovery.
//! - Compiler dependency graphs.
//! - Compiler parallel execution.
//! - Worker pools.
//! - Physical scheduling.
//! - Resource allocation.
//! - Target discovery.
//! - Hardware discovery.
//! - Compilation.
//! - Artifact generation or verification.
//! - Runtime execution.
//! - Distributed execution.
//!
//! ## Canonical authorities
//!
//! Task identities:
//!     crate::toolchain::ids::TaskId
//!
//! Errors:
//!     crate::toolchain::errors
//!
//! Result contract:
//!     crate::toolchain::result
//!
//! Compiler dependency graph:
//!     crate::compiler::context::dependency_context
//!
//! Compiler dependency planning:
//!     crate::compiler::parallel::dependencies
//!
//! Compiler build execution:
//!     crate::compiler::parallel::build
//!
//! Operational budgets:
//!     crate::compiler::configuration::budget
//!
//! ## Edge direction
//!
//! Every edge is directed from a dependent task to its prerequisite.
//!
//!     application -> library
//!
//! The library must complete before the application is eligible.
//!
//! Topological output reverses this relationship into execution
//! eligibility order:
//!
//!     library, application
//!
//! ## Determinism
//!
//! Task identities are maintained in canonical sorted order.
//!
//! Dependencies are maintained in canonical sorted order.
//!
//! Topological ordering uses the smallest available TaskId whenever
//! multiple independent tasks are ready.
//!
//! Dependency waves use the same canonical ordering.
//!
//! Results do not depend on hash-map iteration, insertion timing,
//! worker identity, or execution completion order.
//!
//! ## Transactional mutation
//!
//! A dependency insertion is checked for:
//!
//! - Existing dependent task.
//! - Existing prerequisite task.
//! - Duplicate relationship.
//! - Self-dependency.
//! - Cycle creation.
//! - Required memory reservation.
//!
//! The graph is mutated only after all validation and required
//! allocation have succeeded.
//!
//! Removing a task also removes its incoming and outgoing relationships.
//!
//! ## Scalability
//!
//! No application-defined maximum exists for:
//!
//! - Tasks.
//! - Dependencies.
//! - Dependents.
//! - Graph depth.
//! - Computing domains.
//! - Targets.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed participants.
//!
//! Storage is proportional to the number of graph vertices and edges.
//!
//! Operations are constrained by actual memory, address space,
//! representational limits and explicitly configured operational
//! budgets.
//!
//! Literal infinite memory or execution is not promised.
//!
//! Allocation failures that can be detected through fallible
//! collection reservation are reported instead of being silently
//! interpreted as successful graph operations.
//!
//! ## Concurrency
//!
//! A BuildGraph is an ordinary owned data structure.
//!
//! It contains no global mutable state, hidden locks, worker pool,
//! or implicit scheduler.
//!
//! Callers may share immutable graph references between threads.
//! Mutation requires exclusive access.
//!
//! ## Serialization
//!
//! This is an in-process graph contract.
//!
//! Persistent graph serialization must be introduced through a
//! separately versioned format, not accidental serialization of
//! internal implementation details.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fmt;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::TaskId;
use crate::toolchain::result::ToolchainResult;

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn graph_error(
    kind: GraphFailure,
    message: impl Into<String>,
) -> ToolchainError {
    let message = message.into();

    let error = match kind {
        GraphFailure::InvalidInput => {
            ToolchainError::invalid_input(message)
        }
        GraphFailure::Validation => {
            ToolchainError::validation_failed(message)
        }
        GraphFailure::NotFound => {
            ToolchainError::not_found(message)
        }
        GraphFailure::ResourceExhausted => {
            ToolchainError::resource_exhausted(message)
        }
    };

    error.with_operation("build.graph")
}

#[derive(Debug, Clone, Copy)]
enum GraphFailure {
    InvalidInput,
    Validation,
    NotFound,
    ResourceExhausted,
}

fn allocation_error(
    subject: &str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    graph_error(
        GraphFailure::ResourceExhausted,
        format!(
            "Unable to reserve memory for {subject}: {error}"
        ),
    )
}

fn missing_task(task: &TaskId) -> ToolchainError {
    graph_error(
        GraphFailure::NotFound,
        format!("Build graph does not contain task {task}"),
    )
}

fn invalid_graph(message: impl Into<String>) -> ToolchainError {
    graph_error(GraphFailure::Validation, message)
}

// ============================================================================
// INTERNAL NODE
// ============================================================================

/// One build task and its direct prerequisites.
///
/// The node contains only graph relationships.
/// Task metadata and execution state belong to the task subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BuildGraphNode {
    task: TaskId,
    dependencies: Vec<TaskId>,
}

impl BuildGraphNode {
    fn new(task: TaskId) -> Self {
        Self {
            task,
            dependencies: Vec::new(),
        }
    }
}

// ============================================================================
// BUILD GRAPH
// ============================================================================

/// Deterministic dependency graph for one logical toolchain build.
///
/// The graph stores tasks in sorted identity order.
///
/// Each node stores its direct prerequisites in sorted identity order.
///
/// The representation deliberately avoids:
///
/// - Closed task-type enumerations.
/// - Hardware-specific node types.
/// - Fixed task or edge limits.
/// - Hash-based nondeterministic traversal.
/// - A competing compiler dependency representation.
///
/// A graph may contain independent tasks, a single task, or no tasks.
///
/// Empty graphs are structurally valid. Whether an empty build is
/// meaningful is decided by the build-plan and build-service contracts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildGraph {
    nodes: Vec<BuildGraphNode>,
}

impl BuildGraph {
    /// Construct an empty build graph.
    ///
    /// No task IDs or global state are allocated.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// Return the number of tasks in the graph.
    #[must_use]
    pub fn task_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of direct dependency relationships.
    ///
    /// The count is calculated from the stored relationships and does
    /// not rely on a separately maintained counter.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.nodes
            .iter()
            .map(|node| node.dependencies.len())
            .sum()
    }

    /// Return whether the graph contains no tasks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return whether a task belongs to this graph.
    #[must_use]
    pub fn contains_task(&self, task: &TaskId) -> bool {
        self.node_index(task).is_ok()
    }

    /// Return all task identities in canonical sorted order.
    ///
    /// The returned iterator is independent of insertion order.
    pub fn tasks(&self) -> impl ExactSizeIterator<Item = &TaskId> {
        self.nodes.iter().map(|node| &node.task)
    }

    /// Return the direct prerequisites of a task.
    ///
    /// The returned slice is sorted by canonical TaskId order.
    ///
    /// Returns `None` if the task is not part of this graph.
    #[must_use]
    pub fn dependencies_of(
        &self,
        task: &TaskId,
    ) -> Option<&[TaskId]> {
        self.node_index(task)
            .ok()
            .map(|index| self.nodes[index].dependencies.as_slice())
    }

    /// Add a task to the graph.
    ///
    /// The task identity must not already exist.
    ///
    /// Insertion preserves canonical ordering.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `InvalidInput` if the task already exists.
    /// - `ResourceExhausted` if memory reservation fails.
    pub fn add_task(
        &mut self,
        task: TaskId,
    ) -> ToolchainResult<()> {
        let insertion_index = match self.node_index(&task) {
            Ok(_) => {
                return Err(graph_error(
                    GraphFailure::InvalidInput,
                    format!("Task {task} already exists in the build graph"),
                ));
            }
            Err(index) => index,
        };

        self.nodes
            .try_reserve(1)
            .map_err(|error| allocation_error("build graph nodes", error))?;

        self.nodes
            .insert(insertion_index, BuildGraphNode::new(task));

        Ok(())
    }

    /// Remove a task and all relationships referencing it.
    ///
    /// Removing a prerequisite also removes that prerequisite from
    /// every dependent task.
    ///
    /// The remaining graph is kept in canonical sorted order.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the task does not exist.
    pub fn remove_task(
        &mut self,
        task: &TaskId,
    ) -> ToolchainResult<()> {
        let index = self
            .node_index(task)
            .map_err(|_| missing_task(task))?;

        self.nodes.remove(index);

        for node in &mut self.nodes {
            if let Ok(position) = node.dependencies.binary_search(task) {
                node.dependencies.remove(position);
            }
        }

        Ok(())
    }

    /// Add a prerequisite relationship.
    ///
    /// The first argument is the dependent task.
    ///
    /// The second argument is the prerequisite task.
    ///
    /// Example:
    ///
    /// `add_dependency(&application, &library)`
    ///
    /// means that `application` depends on `library`.
    ///
    /// Both tasks must already exist.
    ///
    /// Duplicate relationships and cycles are rejected.
    ///
    /// The graph remains unchanged when validation or memory
    /// reservation fails.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `NotFound` if either task is missing.
    /// - `InvalidInput` for a duplicate relationship.
    /// - `ValidationFailed` for a self-dependency or cycle.
    /// - `ResourceExhausted` if memory reservation fails.
    pub fn add_dependency(
        &mut self,
        dependent: &TaskId,
        prerequisite: &TaskId,
    ) -> ToolchainResult<()> {
        let dependent_index = self
            .node_index(dependent)
            .map_err(|_| missing_task(dependent))?;

        let prerequisite_index = self
            .node_index(prerequisite)
            .map_err(|_| missing_task(prerequisite))?;

        if dependent_index == prerequisite_index {
            return Err(invalid_graph(format!(
                "Task {dependent} cannot depend on itself"
            )));
        }

        let dependencies =
            &self.nodes[dependent_index].dependencies;

        match dependencies.binary_search(prerequisite) {
            Ok(_) => {
                return Err(graph_error(
                    GraphFailure::InvalidInput,
                    format!(
                        "Dependency {dependent} -> {prerequisite} already exists"
                    ),
                ));
            }
            Err(_) => {}
        }

        // The proposed edge is:
        //
        // dependent -> prerequisite
        //
        // It creates a cycle precisely when an existing path already
        // connects prerequisite back to dependent.

        if let Some(path) = self.find_path(
            prerequisite_index,
            dependent_index,
        )? {
            let mut cycle = Vec::new();

            cycle
                .try_reserve(path.len().saturating_add(1))
                .map_err(|error| {
                    allocation_error("cycle diagnostic", error)
                })?;

            cycle.push(dependent.clone());

            for index in path {
                cycle.push(self.nodes[index].task.clone());
            }

            return Err(invalid_graph(format!(
                "Adding dependency {dependent} -> {prerequisite} \
                 would create a dependency cycle: {}",
                format_cycle(&cycle)
            )));
        }

        // Reserve before mutating the graph.
        self.nodes[dependent_index]
            .dependencies
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("task dependency list", error)
            })?;

        let dependencies =
            &mut self.nodes[dependent_index].dependencies;

        let insertion_index = dependencies
            .binary_search(prerequisite)
            .expect_err("duplicate dependency was checked above");

        dependencies.insert(
            insertion_index,
            prerequisite.clone(),
        );

        Ok(())
    }

    /// Remove a direct prerequisite relationship.
    ///
    /// Both task identities must exist.
    ///
    /// Returns `true` if the relationship existed and was removed.
    ///
    /// Returns `false` if both tasks exist but the relationship does
    /// not exist.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if either task is missing.
    pub fn remove_dependency(
        &mut self,
        dependent: &TaskId,
        prerequisite: &TaskId,
    ) -> ToolchainResult<bool> {
        let dependent_index = self
            .node_index(dependent)
            .map_err(|_| missing_task(dependent))?;

        self.node_index(prerequisite)
            .map_err(|_| missing_task(prerequisite))?;

        let dependencies =
            &mut self.nodes[dependent_index].dependencies;

        match dependencies.binary_search(prerequisite) {
            Ok(index) => {
                dependencies.remove(index);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Return the deterministic dependency-first task order.
    ///
    /// When multiple tasks are ready, the task with the smallest
    /// canonical TaskId is selected first.
    ///
    /// This method calculates eligibility order only.
    ///
    /// It does not execute tasks, allocate workers, schedule hardware,
    /// reserve resources, or perform compilation.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `ValidationFailed` if an invalid dependency graph is detected.
    /// - `ResourceExhausted` if temporary graph-analysis memory cannot
    ///   be reserved.
    pub fn topological_order(
        &self,
    ) -> ToolchainResult<Vec<TaskId>> {
        let mut indegree = Vec::new();

        indegree
            .try_reserve_exact(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological indegree table", error)
            })?;

        let mut dependents = Vec::new();

        dependents
            .try_reserve_exact(self.nodes.len())
            .map_err(|error| {
                allocation_error("reverse dependency index", error)
            })?;

        for node in &self.nodes {
            indegree.push(node.dependencies.len());
            dependents.push(Vec::<usize>::new());
        }

        // Build the reverse index:
        //
        // dependency -> tasks that depend on it.
        //
        // All references are checked before traversal.

        for (dependent_index, node) in self.nodes.iter().enumerate() {
            for dependency in &node.dependencies {
                let dependency_index = self
                    .node_index(dependency)
                    .map_err(|_| {
                        invalid_graph(format!(
                            "Task {} references missing prerequisite {}",
                            node.task,
                            dependency
                        ))
                    })?;

                dependents[dependency_index]
                    .try_reserve(1)
                    .map_err(|error| {
                        allocation_error(
                            "reverse dependency relationships",
                            error,
                        )
                    })?;

                dependents[dependency_index]
                    .push(dependent_index);
            }
        }

        // The node vector is sorted by TaskId. Therefore the smallest
        // index is also the smallest canonical TaskId.

        let mut ready = BinaryHeap::new();

        ready
            .try_reserve(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological ready queue", error)
            })?;

        for (index, count) in indegree.iter().enumerate() {
            if *count == 0 {
                ready.push(Reverse(index));
            }
        }

        let mut output = Vec::new();

        output
            .try_reserve_exact(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological output", error)
            })?;

        while let Some(Reverse(index)) = ready.pop() {
            output.push(self.nodes[index].task.clone());

            for dependent_index in &dependents[index] {
                let count = &mut indegree[*dependent_index];

                *count = count.checked_sub(1).ok_or_else(|| {
                    invalid_graph(
                        "Dependency accounting underflow detected",
                    )
                })?;

                if *count == 0 {
                    ready.push(Reverse(*dependent_index));
                }
            }
        }

        if output.len() != self.nodes.len() {
            return Err(invalid_graph(
                "The build graph contains a dependency cycle",
            ));
        }

        Ok(output)
    }

    /// Return deterministic dependency-ready waves.
    ///
    /// Every task in a wave has all its prerequisites in earlier waves.
    ///
    /// Tasks within one wave have no ordering dependency on one another.
    ///
    /// This is a structural dependency representation, not a physical
    /// execution schedule or a worker-count decision.
    ///
    /// A downstream executor must still apply task state, failure
    /// propagation, cancellation, resource budgets and provider
    /// requirements.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::topological_order`].
    pub fn topological_waves(
        &self,
    ) -> ToolchainResult<Vec<Vec<TaskId>>> {
        let order = self.topological_order()?;

        let mut levels = Vec::new();

        levels
            .try_reserve_exact(self.nodes.len())
            .map_err(|error| {
                allocation_error("topological level table", error)
            })?;

        levels.resize(self.nodes.len(), 0usize);

        let mut maximum_level = 0usize;

        for task in &order {
            let task_index = self
                .node_index(task)
                .map_err(|_| {
                    invalid_graph(
                        "Topological order contains an unknown task",
                    )
                })?;

            let mut level = 0usize;

            for dependency in &self.nodes[task_index].dependencies {
                let dependency_index = self
                    .node_index(dependency)
                    .map_err(|_| {
                        invalid_graph(
                            "Build graph references a missing prerequisite",
                        )
                    })?;

                let next_level = levels[dependency_index]
                    .checked_add(1)
                    .ok_or_else(|| {
                        graph_error(
                            GraphFailure::ResourceExhausted,
                            "Build graph depth exceeds the representable level range",
                        )
                    })?;

                level = level.max(next_level);
            }

            levels[task_index] = level;
            maximum_level = maximum_level.max(level);
        }

        if order.is_empty() {
            return Ok(Vec::new());
        }

        let wave_count = maximum_level
            .checked_add(1)
            .ok_or_else(|| {
                graph_error(
                    GraphFailure::ResourceExhausted,
                    "Build graph wave count exceeds the representable range",
                )
            })?;

        let mut waves = Vec::new();

        waves
            .try_reserve_exact(wave_count)
            .map_err(|error| {
                allocation_error("dependency-ready waves", error)
            })?;

        for _ in 0..wave_count {
            waves.push(Vec::new());
        }

        // Iterating the canonical topological order preserves
        // deterministic ordering inside every wave.

        for task in order {
            let index = self
                .node_index(&task)
                .map_err(|_| {
                    invalid_graph(
                        "Topological task disappeared during wave construction",
                    )
                })?;

            let level = levels[index];

            waves[level]
                .try_reserve(1)
                .map_err(|error| {
                    allocation_error("dependency-ready wave tasks", error)
                })?;

            waves[level].push(task);
        }

        Ok(waves)
    }

    /// Validate all graph invariants.
    ///
    /// Checks:
    ///
    /// - Canonical task ordering.
    /// - Task identity uniqueness.
    /// - Canonical dependency ordering.
    /// - Dependency uniqueness.
    /// - Dependency existence.
    /// - Absence of cycles.
    ///
    /// # Errors
    ///
    /// Returns `ValidationFailed` for an invalid graph and
    /// `ResourceExhausted` when validation cannot reserve memory.
    pub fn validate(&self) -> ToolchainResult<()> {
        for pair in self.nodes.windows(2) {
            if pair[0].task >= pair[1].task {
                return Err(invalid_graph(
                    "Build graph task identities are not strictly ordered",
                ));
            }
        }

        for node in &self.nodes {
            for pair in node.dependencies.windows(2) {
                if pair[0] >= pair[1] {
                    return Err(invalid_graph(format!(
                        "Task {} has duplicate or unsorted prerequisites",
                        node.task
                    )));
                }
            }

            for dependency in &node.dependencies {
                if self.node_index(dependency).is_err() {
                    return Err(invalid_graph(format!(
                        "Task {} references missing prerequisite {}",
                        node.task,
                        dependency
                    )));
                }
            }
        }

        self.topological_order()?;

        Ok(())
    }

    /// Find a deterministic existing path between two graph nodes.
    ///
    /// The path includes both endpoints.
    ///
    /// Breadth-first traversal is used so the returned path is
    /// deterministic and has the fewest edges among the available paths.
    ///
    /// This is an internal graph operation.
    fn find_path(
        &self,
        start: usize,
        target: usize,
    ) -> ToolchainResult<Option<Vec<usize>>> {
        let count = self.nodes.len();

        let mut visited = Vec::new();

        visited
            .try_reserve_exact(count)
            .map_err(|error| {
                allocation_error("cycle-detection visited table", error)
            })?;

        visited.resize(count, false);

        let mut parents = Vec::new();

        parents
            .try_reserve_exact(count)
            .map_err(|error| {
                allocation_error("cycle-detection parent table", error)
            })?;

        parents.resize(count, None::<usize>);

        let mut queue = VecDeque::new();

        queue
            .try_reserve(count)
            .map_err(|error| {
                allocation_error("cycle-detection queue", error)
            })?;

        visited[start] = true;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            if current == target {
                let mut path = Vec::new();

                path
                    .try_reserve(count)
                    .map_err(|error| {
                        allocation_error("cycle path", error)
                    })?;

                let mut cursor = target;
                path.push(cursor);

                while cursor != start {
                    cursor = parents[cursor].ok_or_else(|| {
                        invalid_graph(
                            "Cycle-detection predecessor chain is inconsistent",
                        )
                    })?;

                    path.push(cursor);
                }

                path.reverse();

                return Ok(Some(path));
            }

            for dependency in &self.nodes[current].dependencies {
                let next = self
                    .node_index(dependency)
                    .map_err(|_| {
                        invalid_graph(format!(
                            "Task {} references missing prerequisite {}",
                            self.nodes[current].task,
                            dependency
                        ))
                    })?;

                if !visited[next] {
                    visited[next] = true;
                    parents[next] = Some(current);
                    queue.push_back(next);
                }
            }
        }

        Ok(None)
    }

    /// Find a task index through binary search.
    ///
    /// On success, returns its index.
    ///
    /// On failure, returns the canonical insertion position.
    fn node_index(
        &self,
        task: &TaskId,
    ) -> Result<usize, usize> {
        self.nodes
            .binary_search_by(|node| node.task.cmp(task))
    }
}

// ============================================================================
// CYCLE FORMATTING
// ============================================================================

fn format_cycle(cycle: &[TaskId]) -> String {
    let mut output = String::new();

    for (index, task) in cycle.iter().enumerate() {
        if index > 0 {
            output.push_str(" -> ");
        }

        use fmt::Write;

        // Writing to String is infallible under the fmt::Write contract.
        let _ = write!(&mut output, "{task}");
    }

    output
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    fn task(value: u128) -> TaskId {
        TaskId::new(
            IdNamespace::new("test").expect("valid namespace"),
            value,
        )
        .expect("valid task identity")
    }

    fn graph_with_tasks(values: &[u128]) -> BuildGraph {
        let mut graph = BuildGraph::new();

        for value in values {
            graph
                .add_task(task(*value))
                .expect("task insertion should succeed");
        }

        graph
    }

    #[test]
    fn empty_graph_is_valid() {
        let graph = BuildGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate().is_ok());
        assert!(graph.topological_order().unwrap().is_empty());
        assert!(graph.topological_waves().unwrap().is_empty());
    }

    #[test]
    fn single_task_is_valid() {
        let graph = graph_with_tasks(&[1]);

        assert_eq!(graph.task_count(), 1);
        assert_eq!(
            graph.topological_order().unwrap(),
            vec![task(1)]
        );

        assert_eq!(
            graph.topological_waves().unwrap(),
            vec![vec![task(1)]]
        );
    }

    #[test]
    fn tasks_are_stored_in_canonical_order() {
        let graph = graph_with_tasks(&[9, 2, 7, 1, 4]);

        assert_eq!(
            graph.tasks().cloned().collect::<Vec<_>>(),
            vec![
                task(1),
                task(2),
                task(4),
                task(7),
                task(9),
            ]
        );
    }

    #[test]
    fn dependency_is_executed_before_dependent() {
        let mut graph = graph_with_tasks(&[1, 2]);

        graph
            .add_dependency(&task(1), &task(2))
            .unwrap();

        assert_eq!(
            graph.topological_order().unwrap(),
            vec![task(2), task(1)]
        );
    }

    #[test]
    fn independent_tasks_use_canonical_order() {
        let graph = graph_with_tasks(&[5, 2, 8, 1]);

        assert_eq!(
            graph.topological_order().unwrap(),
            vec![task(1), task(2), task(5), task(8)]
        );
    }

    #[test]
    fn independent_tasks_share_a_dependency_wave() {
        let graph = graph_with_tasks(&[5, 2, 8, 1]);

        assert_eq!(
            graph.topological_waves().unwrap(),
            vec![vec![
                task(1),
                task(2),
                task(5),
                task(8)
            ]]
        );
    }

    #[test]
    fn dependency_waves_respect_transitive_dependencies() {
        let mut graph = graph_with_tasks(&[1, 2, 3, 4, 5]);

        graph.add_dependency(&task(2), &task(1)).unwrap();
        graph.add_dependency(&task(3), &task(1)).unwrap();
        graph.add_dependency(&task(4), &task(2)).unwrap();
        graph.add_dependency(&task(5), &task(4)).unwrap();

        assert_eq!(
            graph.topological_waves().unwrap(),
            vec![
                vec![task(1)],
                vec![task(2), task(3)],
                vec![task(4)],
                vec![task(5)],
            ]
        );
    }

    #[test]
    fn duplicate_task_is_rejected() {
        let mut graph = graph_with_tasks(&[1]);

        let result = graph.add_task(task(1));

        assert!(result.is_err());
        assert_eq!(graph.task_count(), 1);
    }

    #[test]
    fn duplicate_dependency_is_rejected() {
        let mut graph = graph_with_tasks(&[1, 2]);

        graph.add_dependency(&task(1), &task(2)).unwrap();

        let result = graph.add_dependency(&task(1), &task(2));

        assert!(result.is_err());
        assert_eq!(graph.dependency_count(), 1);
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut graph = graph_with_tasks(&[1]);

        assert!(
            graph.add_dependency(&task(1), &task(1)).is_err()
        );

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn direct_cycle_is_rejected_transactionally() {
        let mut graph = graph_with_tasks(&[1, 2]);

        graph.add_dependency(&task(1), &task(2)).unwrap();

        let before = graph.clone();

        assert!(
            graph.add_dependency(&task(2), &task(1)).is_err()
        );

        assert_eq!(graph, before);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn transitive_cycle_is_rejected() {
        let mut graph = graph_with_tasks(&[1, 2, 3, 4]);

        graph.add_dependency(&task(1), &task(2)).unwrap();
        graph.add_dependency(&task(2), &task(3)).unwrap();
        graph.add_dependency(&task(3), &task(4)).unwrap();

        let before = graph.clone();

        assert!(
            graph.add_dependency(&task(4), &task(1)).is_err()
        );

        assert_eq!(graph, before);
    }

    #[test]
    fn missing_dependent_is_rejected() {
        let mut graph = graph_with_tasks(&[1]);

        assert!(
            graph.add_dependency(&task(2), &task(1)).is_err()
        );

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn missing_prerequisite_is_rejected() {
        let mut graph = graph_with_tasks(&[1]);

        assert!(
            graph.add_dependency(&task(1), &task(2)).is_err()
        );

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn removing_dependency_preserves_other_relationships() {
        let mut graph = graph_with_tasks(&[1, 2, 3]);

        graph.add_dependency(&task(1), &task(2)).unwrap();
        graph.add_dependency(&task(1), &task(3)).unwrap();

        assert!(
            graph.remove_dependency(&task(1), &task(2)).unwrap()
        );

        assert_eq!(
            graph.dependencies_of(&task(1)).unwrap(),
            &[task(3)]
        );

        assert_eq!(graph.dependency_count(), 1);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn removing_missing_relationship_returns_false() {
        let mut graph = graph_with_tasks(&[1, 2]);

        assert!(
            !graph.remove_dependency(&task(1), &task(2)).unwrap()
        );
    }

    #[test]
    fn removing_task_removes_incoming_and_outgoing_edges() {
        let mut graph = graph_with_tasks(&[1, 2, 3, 4]);

        graph.add_dependency(&task(1), &task(2)).unwrap();
        graph.add_dependency(&task(3), &task(2)).unwrap();
        graph.add_dependency(&task(4), &task(3)).unwrap();

        graph.remove_task(&task(2)).unwrap();

        assert!(!graph.contains_task(&task(2)));
        assert_eq!(graph.dependency_count(), 1);

        assert_eq!(
            graph.dependencies_of(&task(4)).unwrap(),
            &[task(3)]
        );

        assert_eq!(
            graph.dependencies_of(&task(1)).unwrap(),
            &[]
        );

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn removing_unknown_task_returns_error() {
        let mut graph = graph_with_tasks(&[1]);

        assert!(graph.remove_task(&task(2)).is_err());
        assert_eq!(graph.task_count(), 1);
    }

    #[test]
    fn removing_unknown_dependency_endpoint_returns_error() {
        let mut graph = graph_with_tasks(&[1]);

        assert!(
            graph.remove_dependency(&task(1), &task(2)).is_err()
        );

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn dependency_order_does_not_depend_on_insertion_order() {
        let mut first = graph_with_tasks(&[1, 2, 3, 4]);
        let mut second = graph_with_tasks(&[4, 3, 2, 1]);

        first.add_dependency(&task(4), &task(2)).unwrap();
        first.add_dependency(&task(3), &task(1)).unwrap();
        first.add_dependency(&task(4), &task(3)).unwrap();

        second.add_dependency(&task(4), &task(3)).unwrap();
        second.add_dependency(&task(3), &task(1)).unwrap();
        second.add_dependency(&task(4), &task(2)).unwrap();

        assert_eq!(
            first.topological_order().unwrap(),
            second.topological_order().unwrap()
        );

        assert_eq!(
            first.topological_waves().unwrap(),
            second.topological_waves().unwrap()
        );
    }

    #[test]
    fn multiple_prerequisites_are_ordered_deterministically() {
        let mut graph = graph_with_tasks(&[1, 2, 3, 4]);

        graph.add_dependency(&task(4), &task(3)).unwrap();
        graph.add_dependency(&task(4), &task(1)).unwrap();
        graph.add_dependency(&task(4), &task(2)).unwrap();

        assert_eq!(
            graph.dependencies_of(&task(4)).unwrap(),
            &[task(1), task(2), task(3)]
        );

        assert_eq!(
            graph.topological_order().unwrap(),
            vec![task(1), task(2), task(3), task(4)]
        );
    }

    #[test]
    fn graph_clone_is_independent() {
        let mut original = graph_with_tasks(&[1, 2]);

        original.add_dependency(&task(1), &task(2)).unwrap();

        let mut cloned = original.clone();

        cloned.remove_task(&task(2)).unwrap();

        assert_eq!(original.task_count(), 2);
        assert_eq!(cloned.task_count(), 1);

        assert!(original.validate().is_ok());
        assert!(cloned.validate().is_ok());
    }

    #[test]
    fn graph_validation_accepts_large_structural_families() {
        let mut graph = BuildGraph::new();

        for value in 1..=256 {
            graph.add_task(task(value)).unwrap();
        }

        for value in 2..=256 {
            graph
                .add_dependency(&task(value), &task(value - 1))
                .unwrap();
        }

        assert_eq!(graph.task_count(), 256);
        assert_eq!(graph.dependency_count(), 255);

        assert!(graph.validate().is_ok());

        let order = graph.topological_order().unwrap();

        assert_eq!(order.first(), Some(&task(1)));
        assert_eq!(order.last(), Some(&task(256)));

        let waves = graph.topological_waves().unwrap();

        assert_eq!(waves.len(), 256);
    }
}
