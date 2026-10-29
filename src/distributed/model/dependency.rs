
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Logical task dependencies for Zamani distributed computation.
//!
//! # Ownership
//!
//! This module owns the task-level dependency descriptor and graph.
//! It describes relationships between logical distributed tasks, independently
//! of the physical machines, processes, devices, and execution providers that
//! may eventually realize those tasks.
//!
//! It does not own:
//!
//! - task identity: `crate::distributed::ids::TaskId`;
//! - dependency identity allocation: `crate::distributed::ids::DependencyId`;
//! - task definitions or task execution;
//! - source syntax, AST nodes, or semantic name resolution;
//! - canonical quantum operation identities;
//! - operation-level quantum scheduling dependencies;
//! - resource quantities, capability evidence, or resource allocation;
//! - partitioning, placement, routing, or scheduling algorithms;
//! - communication protocols, network endpoints, or transport;
//! - ZUIR, ZCIR, `quantum::ir`, HDLIR, or backend implementations.
//!
//! The operation-level quantum dependency graph remains owned by the existing
//! quantum scheduling subsystem. This graph represents dependencies between
//! distributed tasks, not individual IR operations.
//!
//! # Graph semantics
//!
//! A dependency has a directed source task and destination task.
//!
//! A `Precedence` dependency requires its source task to precede its
//! destination task in a valid topological ordering.
//!
//! A `Feedback` dependency represents a relationship that semantic analysis
//! has explicitly classified as feedback rather than an ordinary DAG ordering
//! constraint. Feedback edges are retained in the graph but excluded from
//! topological ordering. This module does not decide whether a particular
//! feedback relationship is semantically valid; the semantic analysis layer
//! must establish that before execution.
//!
//! Dependencies are a directed multigraph. Multiple dependency records may
//! connect the same pair of tasks when they express distinct reasons or
//! provenance. Each record therefore has its own `DependencyId`.
//!
//! # Scalability and determinism
//!
//! There are no application-defined universal limits on the number of tasks
//! or dependencies. The graph grows according to available memory and
//! configured operational budgets imposed by higher-level callers.
//!
//! Concrete Rust processes and collections remain finite. Allocation failure
//! is not represented as semantic infeasibility, and this module does not
//! claim literal infinite capacity.
//!
//! Task and dependency iteration is deterministic. Topological traversal and
//! cycle detection are iterative, avoiding recursion proportional to graph
//! depth. Ordering is stable for a given set of logical IDs and edges.
//!
//! # Validation boundary
//!
//! Successful insertion guarantees that:
//!
//! - both endpoint tasks are registered;
//! - the dependency ID is unique within this graph;
//! - the dependency is not a self-edge;
//! - the edge is indexed consistently.
//!
//! A graph may be temporarily cyclic while being constructed. Call
//! `topological_order` before executing work that requires an acyclic
//! precedence graph. Feedback edges are excluded from that check.
//!
//! This module validates graph structure only. It does not establish that
//! tasks are semantically valid, capabilities are available, resources are
//! sufficient, or a physical execution plan is feasible.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::distributed::ids::{DependencyId, TaskId};

/// A namespaced, open-world dependency classification.
///
/// Namespaces permit domain extensions without requiring a closed enum
/// containing every possible dependency category. A kind is descriptive
/// metadata; it does not by itself determine execution feasibility.
///
/// Common names may include `data`, `control`, `ordering`, `resource`,
/// `communication`, `measurement`, or application-defined values. These
/// examples are not an exhaustive list.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DependencyKind {
    namespace: String,
    name: String,
}

impl DependencyKind {
    /// Constructs a validated dependency kind.
    ///
    /// Components must not be empty or whitespace-only and must not contain
    /// Unicode control characters. Their spelling is preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns `DependencyValidationError::InvalidKind` when either component
    /// is invalid.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, DependencyValidationError> {
        let namespace = namespace.into();
        let name = name.into();

        if !valid_component(&namespace) || !valid_component(&name) {
            return Err(DependencyValidationError::InvalidKind {
                namespace,
                name,
            });
        }

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the classification name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable qualified spelling `namespace:name`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for DependencyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.namespace, self.name)
    }
}

fn valid_component(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

/// Defines how a dependency participates in task ordering.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum DependencyRelation {
    /// The source task must precede the destination task.
    ///
    /// These edges participate in DAG validation and topological ordering.
    Precedence,

    /// An explicitly classified feedback relationship.
    ///
    /// Feedback edges are preserved but excluded from ordinary topological
    /// ordering. Semantic analysis must validate the feedback semantics
    /// before a plan containing such an edge is executed.
    Feedback,
}

/// A logical dependency between two distributed tasks.
///
/// This value does not identify a physical execution location or dictate
/// which scheduling algorithm must satisfy it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskDependency {
    id: DependencyId,
    from: TaskId,
    to: TaskId,
    kind: DependencyKind,
    relation: DependencyRelation,
}

impl TaskDependency {
    /// Constructs a dependency descriptor.
    ///
    /// This constructor validates the descriptor's local structure. It does
    /// not verify that either task exists in a particular graph; insertion
    /// into `DependencyGraph` performs that validation.
    ///
    /// # Errors
    ///
    /// Returns `DependencyValidationError::SelfDependency` if both endpoints
    /// refer to the same task.
    pub fn new(
        id: DependencyId,
        from: TaskId,
        to: TaskId,
        kind: DependencyKind,
        relation: DependencyRelation,
    ) -> Result<Self, DependencyValidationError> {
        if from == to {
            return Err(DependencyValidationError::SelfDependency { task: from });
        }

        Ok(Self {
            id,
            from,
            to,
            kind,
            relation,
        })
    }

    /// Returns this dependency's stable logical identity.
    #[must_use]
    pub const fn id(&self) -> DependencyId {
        self.id
    }

    /// Returns the source task.
    #[must_use]
    pub const fn from(&self) -> TaskId {
        self.from
    }

    /// Returns the destination task.
    #[must_use]
    pub const fn to(&self) -> TaskId {
        self.to
    }

    /// Returns the open-world dependency classification.
    #[must_use]
    pub const fn kind(&self) -> &DependencyKind {
        &self.kind
    }

    /// Returns the ordering relation.
    #[must_use]
    pub const fn relation(&self) -> DependencyRelation {
        self.relation
    }
}

/// Errors produced while constructing or validating dependency descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DependencyValidationError {
    /// A dependency kind has an empty or invalid component.
    InvalidKind {
        /// Supplied namespace.
        namespace: String,
        /// Supplied kind name.
        name: String,
    },

    /// A task cannot depend on itself through this graph representation.
    SelfDependency {
        /// The task referenced by both endpoints.
        task: TaskId,
    },
}

impl fmt::Display for DependencyValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKind { namespace, name } => write!(
                formatter,
                "invalid dependency kind namespace {:?}, name {:?}",
                namespace, name
            ),
            Self::SelfDependency { task } => {
                write!(formatter, "task {task} cannot depend on itself")
            }
        }
    }
}

impl Error for DependencyValidationError {}

/// Errors produced by dependency graph mutation and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DependencyGraphError {
    /// The task is already registered in this graph.
    DuplicateTask {
        /// Existing task identity.
        task: TaskId,
    },

    /// The task is not registered in this graph.
    UnknownTask {
        /// Missing task identity.
        task: TaskId,
    },

    /// A dependency ID already exists in this graph.
    DuplicateDependency {
        /// Conflicting dependency identity.
        dependency: DependencyId,
    },

    /// The dependency source is not registered.
    UnknownSource {
        /// Dependency identity.
        dependency: DependencyId,
        /// Missing task identity.
        task: TaskId,
    },

    /// The dependency destination is not registered.
    UnknownDestination {
        /// Dependency identity.
        dependency: DependencyId,
        /// Missing task identity.
        task: TaskId,
    },

    /// A task cannot be removed while dependencies still reference it.
    TaskHasDependencies {
        /// Task that could not be removed.
        task: TaskId,
    },

    /// The requested dependency is not present.
    UnknownDependency {
        /// Missing dependency identity.
        dependency: DependencyId,
    },

    /// The precedence subgraph contains a cycle.
    ///
    /// `blocked_tasks` contains tasks left with nonzero indegree after
    /// Kahn's algorithm. It can include tasks downstream of a cycle, so it
    /// is a diagnostic set rather than a minimal cycle path.
    CycleDetected {
        /// Tasks blocked from topological completion.
        blocked_tasks: Vec<TaskId>,
    },
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateTask { task } => {
                write!(formatter, "task {task} is already registered")
            }
            Self::UnknownTask { task } => {
                write!(formatter, "task {task} is not registered")
            }
            Self::DuplicateDependency { dependency } => {
                write!(formatter, "dependency {dependency} is already registered")
            }
            Self::UnknownSource { dependency, task } => write!(
                formatter,
                "dependency {dependency} has unregistered source task {task}"
            ),
            Self::UnknownDestination { dependency, task } => write!(
                formatter,
                "dependency {dependency} has unregistered destination task {task}"
            ),
            Self::TaskHasDependencies { task } => write!(
                formatter,
                "task {task} cannot be removed while dependencies reference it"
            ),
            Self::UnknownDependency { dependency } => {
                write!(formatter, "dependency {dependency} is not registered")
            }
            Self::CycleDetected { blocked_tasks } => write!(
                formatter,
                "precedence dependency graph contains a cycle; \
                 {} task(s) remain blocked",
                blocked_tasks.len()
            ),
        }
    }
}

impl Error for DependencyGraphError {}

/// A deterministic directed multigraph of logical task dependencies.
///
/// Tasks are registered independently of edges. The graph preserves all
/// dependency records, including multiple edges between the same task pair.
/// Mutation requires exclusive access through `&mut self`; the graph has no
/// global state, locks, or hidden runtime initialization.
#[derive(Clone, Debug, Default)]
pub struct DependencyGraph {
    tasks: BTreeSet<TaskId>,
    dependencies: BTreeMap<DependencyId, TaskDependency>,
    outgoing: BTreeMap<TaskId, BTreeSet<DependencyId>>,
    incoming: BTreeMap<TaskId, BTreeSet<DependencyId>>,
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered tasks.
    #[must_use]
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    /// Returns the number of dependency records.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether no tasks are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Returns whether a task is registered.
    #[must_use]
    pub fn contains_task(&self, task: TaskId) -> bool {
        self.tasks.contains(&task)
    }

    /// Returns whether a dependency identity is registered.
    #[must_use]
    pub fn contains_dependency(&self, dependency: DependencyId) -> bool {
        self.dependencies.contains_key(&dependency)
    }

    /// Registers a task in the graph.
    ///
    /// Registration does not imply that the task is semantically valid or
    /// executable.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateTask` if the identity is already present.
    pub fn add_task(&mut self, task: TaskId) -> Result<(), DependencyGraphError> {
        if !self.tasks.insert(task) {
            return Err(DependencyGraphError::DuplicateTask { task });
        }

        self.outgoing.entry(task).or_default();
        self.incoming.entry(task).or_default();
        Ok(())
    }

    /// Removes a task that has no incoming or outgoing dependencies.
    ///
    /// The operation is atomic from the caller's perspective: a task with
    /// incident edges remains registered when this method returns an error.
    ///
    /// # Errors
    ///
    /// Returns `UnknownTask` when the task is absent, or `TaskHasDependencies`
    /// when any dependency references it.
    pub fn remove_task(&mut self, task: TaskId) -> Result<(), DependencyGraphError> {
        if !self.tasks.contains(&task) {
            return Err(DependencyGraphError::UnknownTask { task });
        }

        let has_outgoing = self.outgoing.get(&task).is_some_and(|edges| !edges.is_empty());
        let has_incoming = self.incoming.get(&task).is_some_and(|edges| !edges.is_empty());

        if has_outgoing || has_incoming {
            return Err(DependencyGraphError::TaskHasDependencies { task });
        }

        self.tasks.remove(&task);
        self.outgoing.remove(&task);
        self.incoming.remove(&task);
        Ok(())
    }

    /// Inserts a dependency after validating its identity and endpoints.
    ///
    /// Distinct dependency IDs are retained even when their endpoints and
    /// classifications match. A rejected insertion leaves the graph unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error for a duplicate dependency ID or an unregistered
    /// endpoint.
    pub fn add_dependency(
        &mut self,
        dependency: TaskDependency,
    ) -> Result<(), DependencyGraphError> {
        if self.dependencies.contains_key(&dependency.id) {
            return Err(DependencyGraphError::DuplicateDependency {
                dependency: dependency.id,
            });
        }

        if !self.tasks.contains(&dependency.from) {
            return Err(DependencyGraphError::UnknownSource {
                dependency: dependency.id,
                task: dependency.from,
            });
        }

        if !self.tasks.contains(&dependency.to) {
            return Err(DependencyGraphError::UnknownDestination {
                dependency: dependency.id,
                task: dependency.to,
            });
        }

        let id = dependency.id;
        let from = dependency.from;
        let to = dependency.to;

        self.dependencies.insert(id, dependency);
        self.outgoing.entry(from).or_default().insert(id);
        self.incoming.entry(to).or_default().insert(id);
        Ok(())
    }

    /// Removes a dependency by its identity.
    ///
    /// # Errors
    ///
    /// Returns `UnknownDependency` if the ID is not present.
    pub fn remove_dependency(
        &mut self,
        dependency: DependencyId,
    ) -> Result<TaskDependency, DependencyGraphError> {
        let edge = self
            .dependencies
            .remove(&dependency)
            .ok_or(DependencyGraphError::UnknownDependency { dependency })?;

        if let Some(edges) = self.outgoing.get_mut(&edge.from) {
            edges.remove(&dependency);
        }
        if let Some(edges) = self.incoming.get_mut(&edge.to) {
            edges.remove(&dependency);
        }

        Ok(edge)
    }

    /// Looks up a dependency by its identity.
    #[must_use]
    pub fn dependency(&self, dependency: DependencyId) -> Option<&TaskDependency> {
        self.dependencies.get(&dependency)
    }

    /// Iterates over all registered task IDs in deterministic order.
    pub fn tasks(&self) -> impl Iterator<Item = TaskId> + '_ {
        self.tasks.iter().copied()
    }

    /// Iterates over all dependencies in deterministic ID order.
    pub fn dependencies(&self) -> impl Iterator<Item = &TaskDependency> + '_ {
        self.dependencies.values()
    }

    /// Iterates over dependencies leaving a task in deterministic ID order.
    ///
    /// Returns an error when the task is not registered.
    pub fn outgoing(
        &self,
        task: TaskId,
    ) -> Result<impl Iterator<Item = &TaskDependency> + '_, DependencyGraphError> {
        let edges = self
            .outgoing
            .get(&task)
            .ok_or(DependencyGraphError::UnknownTask { task })?;

        Ok(edges.iter().filter_map(|id| self.dependencies.get(id)))
    }

    /// Iterates over dependencies entering a task in deterministic ID order.
    ///
    /// Returns an error when the task is not registered.
    pub fn incoming(
        &self,
        task: TaskId,
    ) -> Result<impl Iterator<Item = &TaskDependency> + '_, DependencyGraphError> {
        let edges = self
            .incoming
            .get(&task)
            .ok_or(DependencyGraphError::UnknownTask { task })?;

        Ok(edges.iter().filter_map(|id| self.dependencies.get(id)))
    }

    /// Returns a deterministic topological ordering of the precedence graph.
    ///
    /// Feedback edges are deliberately excluded. The result contains every
    /// registered task exactly once if the precedence subgraph is acyclic.
    ///
    /// # Errors
    ///
    /// Returns `CycleDetected` when one or more precedence cycles prevent a
    /// complete topological ordering.
    pub fn topological_order(&self) -> Result<Vec<TaskId>, DependencyGraphError> {
        let mut indegree: BTreeMap<TaskId, usize> =
            self.tasks.iter().copied().map(|task| (task, 0)).collect();

        for dependency in self.dependencies.values() {
            if dependency.relation == DependencyRelation::Precedence {
                if let Some(count) = indegree.get_mut(&dependency.to) {
                    *count = count.checked_add(1).unwrap_or(usize::MAX);
                }
            }
        }

        let mut ready: BTreeSet<TaskId> = indegree
            .iter()
            .filter_map(|(task, count)| (*count == 0).then_some(*task))
            .collect();

        let mut ordered = Vec::with_capacity(self.tasks.len());

        while let Some(task) = ready.pop_first() {
            ordered.push(task);

            if let Some(edge_ids) = self.outgoing.get(&task) {
                for edge_id in edge_ids {
                    let Some(dependency) = self.dependencies.get(edge_id) else {
                        continue;
                    };

                    if dependency.relation != DependencyRelation::Precedence {
                        continue;
                    }

                    let Some(count) = indegree.get_mut(&dependency.to) else {
                        continue;
                    };

                    *count -= 1;
                    if *count == 0 {
                        ready.insert(dependency.to);
                    }
                }
            }
        }

        if ordered.len() != self.tasks.len() {
            let blocked_tasks = indegree
                .into_iter()
                .filter_map(|(task, count)| (count != 0).then_some(task))
                .collect();

            return Err(DependencyGraphError::CycleDetected { blocked_tasks });
        }

        Ok(ordered)
    }

    /// Validates that the precedence subgraph is acyclic.
    ///
    /// Feedback relationships do not participate in this DAG validation.
    ///
    /// # Errors
    ///
    /// Returns `CycleDetected` when a precedence cycle exists.
    pub fn validate_acyclic(&self) -> Result<(), DependencyGraphError> {
        self.topological_order().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(sequence: u128) -> TaskId {
        TaskId::new(1, sequence).expect("non-zero test ID")
    }

    fn dependency_id(sequence: u128) -> DependencyId {
        DependencyId::new(2, sequence).expect("non-zero test ID")
    }

    fn kind(name: &str) -> DependencyKind {
        DependencyKind::new("test", name).expect("valid test dependency kind")
    }

    fn edge(
        id: u128,
        from: TaskId,
        to: TaskId,
        relation: DependencyRelation,
        kind_name: &str,
    ) -> TaskDependency {
        TaskDependency::new(
            dependency_id(id),
            from,
            to,
            kind(kind_name),
            relation,
        )
        .expect("valid test edge")
    }

    #[test]
    fn empty_graph_has_empty_topological_order() {
        let graph = DependencyGraph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.topological_order().unwrap(), Vec::<TaskId>::new());
    }

    #[test]
    fn topological_order_respects_precedence() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);
        let c = task(3);

        graph.add_task(a).unwrap();
        graph.add_task(b).unwrap();
        graph.add_task(c).unwrap();

        graph.add_dependency(edge(1, a, c, DependencyRelation::Precedence, "order")).unwrap();
        graph.add_dependency(edge(2, b, c, DependencyRelation::Precedence, "data")).unwrap();

        let order = graph.topological_order().unwrap();
        assert_eq!(order.len(), 3);
        assert!(order.iter().position(|id| *id == a).unwrap()
            < order.iter().position(|id| *id == c).unwrap());
        assert!(order.iter().position(|id| *id == b).unwrap()
            < order.iter().position(|id| *id == c).unwrap());
    }

    #[test]
    fn duplicate_reasons_between_same_tasks_are_preserved() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);

        graph.add_task(a).unwrap();
        graph.add_task(b).unwrap();
        graph.add_dependency(edge(1, a, b, DependencyRelation::Precedence, "data")).unwrap();
        graph.add_dependency(edge(2, a, b, DependencyRelation::Precedence, "control")).unwrap();

        assert_eq!(graph.dependency_count(), 2);
        assert_eq!(graph.outgoing(a).unwrap().count(), 2);
        assert_eq!(graph.incoming(b).unwrap().count(), 2);
    }

    #[test]
    fn precedence_cycle_is_reported() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);
        let c = task(3);

        for task_id in [a, b, c] {
            graph.add_task(task_id).unwrap();
        }

        graph.add_dependency(edge(1, a, b, DependencyRelation::Precedence, "order")).unwrap();
        graph.add_dependency(edge(2, b, c, DependencyRelation::Precedence, "order")).unwrap();
        graph.add_dependency(edge(3, c, a, DependencyRelation::Precedence, "order")).unwrap();

        assert!(matches!(
            graph.topological_order(),
            Err(DependencyGraphError::CycleDetected { .. })
        ));
    }

    #[test]
    fn feedback_edges_do_not_create_a_dag_cycle() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);

        graph.add_task(a).unwrap();
        graph.add_task(b).unwrap();
        graph.add_dependency(edge(1, a, b, DependencyRelation::Precedence, "order")).unwrap();
        graph.add_dependency(edge(2, b, a, DependencyRelation::Feedback, "feedback")).unwrap();

        assert_eq!(graph.topological_order().unwrap(), vec![a, b]);
    }

    #[test]
    fn unknown_endpoints_are_rejected_without_mutation() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);
        graph.add_task(a).unwrap();

        let dependency = edge(1, a, b, DependencyRelation::Precedence, "order");
        assert!(matches!(
            graph.add_dependency(dependency),
            Err(DependencyGraphError::UnknownDestination { .. })
        ));
        assert_eq!(graph.dependency_count(), 0);
        assert_eq!(graph.outgoing(a).unwrap().count(), 0);
    }

    #[test]
    fn task_with_incident_edges_cannot_be_removed() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);

        graph.add_task(a).unwrap();
        graph.add_task(b).unwrap();
        graph.add_dependency(edge(1, a, b, DependencyRelation::Precedence, "order")).unwrap();

        assert!(matches!(
            graph.remove_task(a),
            Err(DependencyGraphError::TaskHasDependencies { .. })
        ));
        assert!(graph.contains_task(a));
    }

    #[test]
    fn removal_keeps_indexes_consistent() {
        let mut graph = DependencyGraph::new();
        let a = task(1);
        let b = task(2);

        graph.add_task(a).unwrap();
        graph.add_task(b).unwrap();
        graph.add_dependency(edge(1, a, b, DependencyRelation::Precedence, "order")).unwrap();

        let removed = graph.remove_dependency(dependency_id(1)).unwrap();
        assert_eq!(removed.from(), a);
        assert_eq!(removed.to(), b);
        assert_eq!(graph.outgoing(a).unwrap().count(), 0);
        assert_eq!(graph.incoming(b).unwrap().count(), 0);
    }

    #[test]
    fn kind_validation_rejects_control_characters() {
        assert!(DependencyKind::new("test", "bad\nname").is_err());
        assert!(DependencyKind::new(" ", "order").is_err());
    }
}
