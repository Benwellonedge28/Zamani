//! Zamani Quantum Scheduling — Dependency Graph
//!
//! Path:
//!     src/quantum/scheduling/ir/dependency.rs
//!
//! # Purpose
//!
//! This module provides the scheduler-local dependency graph consumed by
//! planning, scheduling, critical-path analysis, verification, and scheduling
//! transformations.
//!
//! It answers:
//!
//! > Which scheduler operations must precede which other scheduler operations,
//! > and how can those relationships be traversed efficiently and
//! > deterministically?
//!
//! # Architectural boundary
//!
//! ```text
//! canonical Zamani Quantum IR
//!             │
//!             ▼
//!      semantic dependency analysis
//!             │
//!             ▼
//! scheduling::ir::dependency
//!             │
//!       ┌─────┼─────────────┐
//!       ▼     ▼             ▼
//!   planners  critical    verification
//!             path
//! ```
//!
//! This module owns:
//!
//! - scheduler-local dependency graph storage;
//! - operation-node registration;
//! - dependency insertion/removal;
//! - deterministic predecessor/successor queries;
//! - indegree/outdegree analysis;
//! - deterministic topological traversal;
//! - cycle detection;
//! - graph validation;
//! - graph statistics;
//! - graph snapshots suitable for read-only concurrent scheduling.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - hardware topology;
//! - routing;
//! - timestamps;
//! - resource calendars;
//! - scheduling policies;
//! - scheduling algorithms;
//! - QEC decoding;
//! - noise modelling;
//! - hardware execution.
//!
//! # Canonical identities
//!
//! Scheduler operation identities are represented through the existing
//! `OperationRef` type from `scheduling::types`, whose underlying semantic
//! identity is the canonical:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Logical and physical qubit identities remain owned exclusively by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not redefine either qubit type.
//!
//! A dependency graph does not need to import `QubitId` directly because qubit
//! dependencies are represented by the operation relationships that consume
//! them. The canonical qubit types therefore remain at the IR/resource
//! boundaries where they are actually required.
//!
//! # Dependency semantics
//!
//! `DependencyRef` and `DependencyKind` are owned by `scheduling::types`.
//! This graph indexes those scheduler dependency records.
//!
//! The graph deliberately does not reinterpret `DependencyKind`. A dependency
//! kind is metadata supplied by the semantic dependency construction layer.
//! Whether a dependency is semantically mandatory, conditional, or otherwise
//! special must be decided by the layer that created the dependency and by the
//! scheduling policy consuming it.
//!
//! Consequently, this graph does not contain vendor-specific or
//! gate-specific dependency rules.
//!
//! # Multiple dependencies between the same operations
//!
//! More than one dependency may exist between the same pair of operations when
//! they have distinct scheduler dependency identities/reasons.
//!
//! Therefore adjacency is indexed by `DependencyId`, not merely by the pair:
//!
//! ```text
//! predecessor -> { dependency IDs } -> successor
//! ```
//!
//! This avoids silently discarding dependency provenance.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! - maximum operation count;
//! - maximum dependency count;
//! - maximum qubit count;
//! - maximum graph depth;
//! - fixed graph width;
//! - fixed topology;
//! - fixed hardware size.
//!
//! Collection capacities are determined by actual workloads and available host
//! resources.
//!
//! `usize` appears only where Rust collection APIs require a collection size or
//! count. Semantic identities remain strongly typed.
//!
//! The implementation avoids recursive graph traversal so that very deep
//! dependency chains do not consume the call stack proportional to circuit
//! depth.
//!
//! # Determinism
//!
//! Semantic graph storage uses `BTreeMap` and `BTreeSet`.
//!
//! Consequently:
//!
//! - node iteration is deterministic;
//! - dependency iteration is deterministic;
//! - predecessor/successor iteration is deterministic;
//! - topological ordering is deterministic;
//! - cycle reporting is deterministic.
//!
//! No semantic result depends on randomized hash iteration.
//!
//! # Complexity
//!
//! Let:
//!
//! - `V` = number of registered operation nodes;
//! - `E` = number of dependency edges.
//!
//! Registration and edge insertion are O(log V) / O(log E) per collection
//! operation.
//!
//! Topological traversal is O((V + E) log V) because the deterministic ready
//! set is ordered.
//!
//! Cycle detection is O((V + E) log V) in deterministic mode.
//!
//! The graph never allocates a timeline proportional to execution duration.
//!
//! # Immutability and concurrency
//!
//! `DependencyGraph` owns ordinary Rust collections and has no global mutable
//! state or interior mutability.
//!
//! Once a graph has been built and handed to a scheduler, callers can wrap it
//! in `Arc` and share it across read-only analysis/planning tasks safely.
//!
//! Mutation is explicit and requires `&mut self`.
//!
//! # Construction invariant
//!
//! Every stored dependency must have both endpoints registered.
//!
//! A dependency cannot be inserted when either endpoint is unknown.
//!
//! Self-dependencies are rejected because an operation cannot be required to
//! precede itself in a scheduler ordering graph.
//!
//! # Cycle semantics
//!
//! Cycles are not necessarily rejected during insertion because callers may:
//!
//! - construct graphs incrementally;
//! - perform validation at a separate compiler phase;
//! - represent conditional/dynamic relationships before lowering;
//! - inspect invalid graphs for diagnostics.
//!
//! Production scheduling must call `validate_acyclic()` before an algorithm
//! requiring a DAG performs scheduling.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! scheduling::types
//!     │
//!     ▼
//! SchedulingDependencyGraph
//! ```
//!
//! Downstream:
//!
//! ```text
//! SchedulingDependencyGraph
//!     ├── planners
//!     ├── algorithms
//!     ├── critical-path analysis
//!     ├── verification
//!     ├── optimization
//!     └── diagnostics
//! ```
//!
//! The graph must remain independent of hardware, routing, QEC, ZQN, runtime,
//! and provider SDKs.
//!
//! # Safety
//!
//! No `unsafe` code is used.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::super::errors::{SchedulingError, SchedulingResult};
use super::super::types::{
    DependencyId,
    DependencyKind,
    DependencyRef,
    OperationRef,
};

// =============================================================================
// Graph statistics
// =============================================================================

/// Immutable statistics describing a dependency graph.
///
/// Counts are observational metadata. They are not machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyGraphStats {
    /// Number of registered operation nodes.
    pub nodes: usize,

    /// Number of dependency edges.
    pub dependencies: usize,

    /// Number of nodes without predecessors.
    pub roots: usize,

    /// Number of nodes without successors.
    pub leaves: usize,

    /// Maximum number of incoming edges for one node.
    pub maximum_in_degree: usize,

    /// Maximum number of outgoing edges for one node.
    pub maximum_out_degree: usize,
}

impl DependencyGraphStats {
    /// Returns whether the graph contains no operation nodes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.nodes == 0
    }

    /// Returns whether the graph contains no dependency edges.
    #[must_use]
    pub const fn has_dependencies(self) -> bool {
        self.dependencies != 0
    }
}

// =============================================================================
// Cycle
// =============================================================================

/// Deterministically reported directed dependency cycle.
///
/// The first and last operation are the same so the path explicitly closes
/// the cycle.
///
/// Example:
///
/// ```text
/// op1 -> op2 -> op3 -> op1
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCycle {
    operations: Vec<OperationRef>,
}

impl DependencyCycle {
    /// Creates a cycle from a validated closed path.
    ///
    /// The caller must provide at least two entries and the first and last
    /// operation must be equal.
    fn new(operations: Vec<OperationRef>) -> Option<Self> {
        if operations.len() < 2 {
            return None;
        }

        if operations.first() != operations.last() {
            return None;
        }

        Some(Self { operations })
    }

    /// Returns the operations participating in the cycle.
    ///
    /// The returned slice is closed: the first operation equals the last.
    #[must_use]
    pub fn operations(&self) -> &[OperationRef] {
        &self.operations
    }

    /// Returns the number of distinct operation nodes in the cycle.
    ///
    /// Because the path is closed, this is one less than `operations().len()`.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.operations.len().saturating_sub(1)
    }

    /// Returns whether the cycle contains the supplied operation.
    #[must_use]
    pub fn contains(&self, operation: OperationRef) -> bool {
        self.operations
            .iter()
            .any(|candidate| *candidate == operation)
    }
}

impl fmt::Display for DependencyCycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, operation) in self.operations.iter().enumerate() {
            if index != 0 {
                formatter.write_str(" -> ")?;
            }

            write!(formatter, "{operation}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Dependency graph
// =============================================================================

/// Deterministic scheduler dependency graph.
///
/// The graph is intentionally operation-centric. Resource dependencies,
/// timing constraints, communication constraints, measurement dependencies,
/// and QEC relationships are represented by dependency edges generated by
/// their respective semantic/adapter layers.
///
/// The graph itself does not know how a dependency was discovered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraph {
    /// Registered operation nodes.
    nodes: BTreeSet<OperationRef>,

    /// Dependency records keyed by their stable dependency identity.
    dependencies: BTreeMap<DependencyId, DependencyRef>,

    /// Outgoing adjacency:
    ///
    /// operation -> dependency IDs whose source is that operation.
    outgoing: BTreeMap<OperationRef, BTreeSet<DependencyId>>,

    /// Incoming adjacency:
    ///
    /// operation -> dependency IDs whose target is that operation.
    incoming: BTreeMap<OperationRef, BTreeSet<DependencyId>>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
            dependencies: BTreeMap::new(),
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
        }
    }

    /// Creates an empty graph with caller-supplied collection capacities.
    ///
    /// This is an allocation optimization only. It is not a semantic limit.
    ///
    /// The method intentionally accepts no "maximum operation" or
    /// "maximum dependency" policy. Capacity is merely an initial allocation
    /// hint and Rust collections remain dynamically growable.
    #[must_use]
    pub fn with_capacity_hint(
        node_capacity: usize,
        dependency_capacity: usize,
    ) -> Self {
        let mut graph = Self::new();

        // BTreeMap/BTreeSet do not expose stable reserve APIs on the supported
        // Rust contract. The parameters are therefore intentionally ignored
        // rather than introducing an alternative collection implementation or
        // an artificial limit.
        //
        // Keeping the API allows future storage optimizations without changing
        // the semantic graph contract.
        let _ = node_capacity;
        let _ = dependency_capacity;

        graph
    }

    /// Returns the number of registered operation nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the graph has no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether the graph contains the supplied operation.
    #[must_use]
    pub fn contains_operation(&self, operation: OperationRef) -> bool {
        self.nodes.contains(&operation)
    }

    /// Returns whether a canonical operation identity is registered.
    #[must_use]
    pub fn contains_operation_id(&self, operation: OperationId) -> bool {
        self.nodes.iter().any(|reference| reference.id() == operation)
    }

    /// Returns whether the supplied dependency identity is registered.
    #[must_use]
    pub fn contains_dependency(&self, dependency: DependencyId) -> bool {
        self.dependencies.contains_key(&dependency)
    }

    /// Registers one operation node.
    ///
    /// Registration is idempotent for the exact same `OperationRef`.
    ///
    /// A different `OperationRef` carrying the same canonical operation
    /// identity is rejected because one semantic operation must not have two
    /// scheduler identities in one graph.
    pub fn add_operation(
        &mut self,
        operation: OperationRef,
    ) -> SchedulingResult<bool> {
        if self.nodes.contains(&operation) {
            return Ok(false);
        }

        if self.contains_operation_id(operation.id()) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: Some(operation.id()),
                successor: None,
                reason: format!(
                    "operation identity `{}` is already registered with a different scheduler reference",
                    operation.id()
                ),
            });
        }

        self.nodes.insert(operation);
        self.outgoing.entry(operation).or_default();
        self.incoming.entry(operation).or_default();

        Ok(true)
    }

    /// Registers multiple operations.
    ///
    /// The operation list may be supplied in any order. The graph's internal
    /// representation remains deterministic.
    pub fn add_operations<I>(
        &mut self,
        operations: I,
    ) -> SchedulingResult<usize>
    where
        I: IntoIterator<Item = OperationRef>,
    {
        let mut added = 0usize;

        for operation in operations {
            if self.add_operation(operation)? {
                added = added
                    .checked_add(1)
                    .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: None,
                        successor: None,
                        reason: String::from(
                            "operation count overflowed the host collection size",
                        ),
                    })?;
            }
        }

        Ok(added)
    }

    /// Adds one dependency edge.
    ///
    /// Both endpoints must already be registered.
    ///
    /// A dependency ID may occur only once in a graph.
    ///
    /// Multiple distinct dependency IDs between the same operation pair are
    /// allowed because they may represent distinct semantic reasons.
    pub fn add_dependency(
        &mut self,
        dependency: DependencyRef,
    ) -> SchedulingResult<bool> {
        let predecessor = dependency.from();
        let successor = dependency.to();
        let dependency_id = dependency.id();

        if predecessor == successor {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: String::from(
                    "an operation cannot depend on itself",
                ),
            });
        }

        if !self.nodes.contains(&predecessor) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: format!(
                    "dependency predecessor `{predecessor}` is not registered"
                ),
            });
        }

        if !self.nodes.contains(&successor) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: format!(
                    "dependency successor `{successor}` is not registered"
                ),
            });
        }

        if self.dependencies.contains_key(&dependency_id) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: format!(
                    "dependency identity `{dependency_id}` is already registered"
                ),
            });
        }

        self.dependencies.insert(dependency_id, dependency);

        self.outgoing
            .entry(predecessor)
            .or_default()
            .insert(dependency_id);

        self.incoming
            .entry(successor)
            .or_default()
            .insert(dependency_id);

        Ok(true)
    }

    /// Adds multiple dependency edges.
    ///
    /// If any dependency fails validation, no rollback is attempted. Callers
    /// requiring transactional construction should construct a temporary graph
    /// and replace their working graph only after this method succeeds.
    pub fn add_dependencies<I>(
        &mut self,
        dependencies: I,
    ) -> SchedulingResult<usize>
    where
        I: IntoIterator<Item = DependencyRef>,
    {
        let mut added = 0usize;

        for dependency in dependencies {
            if self.add_dependency(dependency)? {
                added = added
                    .checked_add(1)
                    .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: None,
                        successor: None,
                        reason: String::from(
                            "dependency count overflowed the host collection size",
                        ),
                    })?;
            }
        }

        Ok(added)
    }

    /// Removes a dependency by stable dependency identity.
    ///
    /// Returns the removed dependency when it existed.
    pub fn remove_dependency(
        &mut self,
        dependency_id: DependencyId,
    ) -> SchedulingResult<Option<DependencyRef>> {
        let Some(dependency) = self.dependencies.remove(&dependency_id) else {
            return Ok(None);
        };

        let predecessor = dependency.from();
        let successor = dependency.to();

        if let Some(edges) = self.outgoing.get_mut(&predecessor) {
            edges.remove(&dependency_id);
        }

        if let Some(edges) = self.incoming.get_mut(&successor) {
            edges.remove(&dependency_id);
        }

        Ok(Some(dependency))
    }

    /// Removes an operation and all dependency edges incident on it.
    ///
    /// This operation is intentionally explicit because removing a node changes
    /// the dependency graph semantics.
    pub fn remove_operation(
        &mut self,
        operation: OperationRef,
    ) -> SchedulingResult<bool> {
        if !self.nodes.contains(&operation) {
            return Ok(false);
        }

        let outgoing = self
            .outgoing
            .get(&operation)
            .cloned()
            .unwrap_or_default();

        let incoming = self
            .incoming
            .get(&operation)
            .cloned()
            .unwrap_or_default();

        for dependency in outgoing {
            self.remove_dependency(dependency)?;
        }

        for dependency in incoming {
            self.remove_dependency(dependency)?;
        }

        self.outgoing.remove(&operation);
        self.incoming.remove(&operation);
        self.nodes.remove(&operation);

        Ok(true)
    }

    /// Returns an immutable dependency record.
    #[must_use]
    pub fn dependency(
        &self,
        dependency_id: DependencyId,
    ) -> Option<&DependencyRef> {
        self.dependencies.get(&dependency_id)
    }

    /// Returns all dependency records in deterministic dependency-ID order.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> impl Iterator<Item = &DependencyRef> {
        self.dependencies.values()
    }

    /// Returns all registered operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> impl Iterator<Item = &OperationRef> {
        self.nodes.iter()
    }

    /// Returns outgoing dependency IDs for an operation.
    #[must_use]
    pub fn outgoing_dependencies(
        &self,
        operation: OperationRef,
    ) -> impl Iterator<Item = DependencyId> + '_ {
        self.outgoing
            .get(&operation)
            .into_iter()
            .flat_map(|dependencies| dependencies.iter().copied())
    }

    /// Returns incoming dependency IDs for an operation.
    #[must_use]
    pub fn incoming_dependencies(
        &self,
        operation: OperationRef,
    ) -> impl Iterator<Item = DependencyId> + '_ {
        self.incoming
            .get(&operation)
            .into_iter()
            .flat_map(|dependencies| dependencies.iter().copied())
    }

    /// Returns all immediate successors of an operation.
    ///
    /// Successors are returned in deterministic order.
    #[must_use]
    pub fn successors(
        &self,
        operation: OperationRef,
    ) -> Vec<OperationRef> {
        let mut successors = BTreeSet::new();

        if let Some(dependencies) = self.outgoing.get(&operation) {
            for dependency_id in dependencies {
                if let Some(dependency) =
                    self.dependencies.get(dependency_id)
                {
                    successors.insert(dependency.to());
                }
            }
        }

        successors.into_iter().collect()
    }

    /// Returns all immediate predecessors of an operation.
    ///
    /// Predecessors are returned in deterministic order.
    #[must_use]
    pub fn predecessors(
        &self,
        operation: OperationRef,
    ) -> Vec<OperationRef> {
        let mut predecessors = BTreeSet::new();

        if let Some(dependencies) = self.incoming.get(&operation) {
            for dependency_id in dependencies {
                if let Some(dependency) =
                    self.dependencies.get(dependency_id)
                {
                    predecessors.insert(dependency.from());
                }
            }
        }

        predecessors.into_iter().collect()
    }

    /// Returns the number of immediate predecessors.
    #[must_use]
    pub fn in_degree(&self, operation: OperationRef) -> usize {
        self.incoming
            .get(&operation)
            .map_or(0usize, BTreeSet::len)
    }

    /// Returns the number of immediate successors.
    #[must_use]
    pub fn out_degree(&self, operation: OperationRef) -> usize {
        self.outgoing
            .get(&operation)
            .map_or(0usize, BTreeSet::len)
    }

    /// Returns all root operations.
    ///
    /// A root has no incoming dependency edges.
    #[must_use]
    pub fn roots(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .copied()
            .filter(|operation| self.in_degree(*operation) == 0)
            .collect()
    }

    /// Returns all leaf operations.
    ///
    /// A leaf has no outgoing dependency edges.
    #[must_use]
    pub fn leaves(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .copied()
            .filter(|operation| self.out_degree(*operation) == 0)
            .collect()
    }

    /// Returns graph statistics.
    #[must_use]
    pub fn statistics(&self) -> DependencyGraphStats {
        let mut roots = 0usize;
        let mut leaves = 0usize;
        let mut maximum_in_degree = 0usize;
        let mut maximum_out_degree = 0usize;

        for operation in &self.nodes {
            let in_degree = self.in_degree(*operation);
            let out_degree = self.out_degree(*operation);

            if in_degree == 0 {
                roots += 1;
            }

            if out_degree == 0 {
                leaves += 1;
            }

            maximum_in_degree =
                maximum_in_degree.max(in_degree);
            maximum_out_degree =
                maximum_out_degree.max(out_degree);
        }

        DependencyGraphStats {
            nodes: self.nodes.len(),
            dependencies: self.dependencies.len(),
            roots,
            leaves,
            maximum_in_degree,
            maximum_out_degree,
        }
    }

    /// Returns a deterministic topological ordering.
    ///
    /// When multiple operations are simultaneously ready, the smallest
    /// `OperationRef` is selected. This makes the result reproducible without
    /// imposing semantic meaning on insertion order.
    ///
    /// Returns `CycleDetected` when mandatory graph ordering cannot be
    /// represented as a DAG.
    pub fn topological_order(
        &self,
    ) -> SchedulingResult<Vec<OperationRef>> {
        let mut indegree: BTreeMap<OperationRef, usize> =
            self.nodes
                .iter()
                .copied()
                .map(|operation| {
                    (operation, self.in_degree(operation))
                })
                .collect();

        let mut ready = BTreeSet::new();

        for (operation, degree) in &indegree {
            if *degree == 0 {
                ready.insert(*operation);
            }
        }

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(operation) = ready.pop_first() {
            order.push(operation);

            if let Some(dependencies) =
                self.outgoing.get(&operation)
            {
                for dependency_id in dependencies {
                    let Some(dependency) =
                        self.dependencies.get(dependency_id)
                    else {
                        return Err(
                            SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: None,
                                reason: String::from(
                                    "outgoing adjacency references a missing dependency record",
                                ),
                            },
                        );
                    };

                    let successor = dependency.to();

                    let Some(degree) =
                        indegree.get_mut(&successor)
                    else {
                        return Err(
                            SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "dependency references an unregistered successor",
                                ),
                            },
                        );
                    };

                    *degree = degree.checked_sub(1).ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: Some(operation.id()),
                            successor: Some(successor.id()),
                            reason: String::from(
                                "dependency indegree underflowed",
                            ),
                        }
                    })?;

                    if *degree == 0 {
                        ready.insert(successor);
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            let cycle = self
                .find_cycle()
                .map(|cycle| cycle.to_string())
                .unwrap_or_else(|| {
                    String::from(
                        "one or more dependency cycles exist",
                    )
                });

            return Err(SchedulingError::CycleDetected {
                operation: None,
                dependency: None,
                cycle_size: Some(
                    self.nodes.len().saturating_sub(order.len())
                        as u128,
                ),
            }
            .with_reason(cycle));
        }

        Ok(order)
    }

    /// Validates that the graph is a DAG.
    pub fn validate_acyclic(&self) -> SchedulingResult<()> {
        let _ = self.topological_order()?;
        Ok(())
    }

    /// Finds one deterministic directed cycle, if one exists.
    ///
    /// The traversal is iterative and therefore does not consume stack space
    /// proportional to graph depth.
    #[must_use]
    pub fn find_cycle(&self) -> Option<DependencyCycle> {
        #[derive(Clone, Copy)]
        struct Frame {
            node: OperationRef,
            next_dependency_index: usize,
        }

        let mut color: BTreeMap<OperationRef, u8> =
            self.nodes
                .iter()
                .copied()
                .map(|operation| (operation, 0u8))
                .collect();

        let mut stack: Vec<Frame> = Vec::new();
        let mut path: Vec<OperationRef> = Vec::new();
        let mut path_position: BTreeMap<OperationRef, usize> =
            BTreeMap::new();

        for start in self.nodes.iter().copied() {
            if color.get(&start).copied().unwrap_or(0) != 0 {
                continue;
            }

            color.insert(start, 1);
            path_position.insert(start, 0);
            path.push(start);

            stack.push(Frame {
                node: start,
                next_dependency_index: 0,
            });

            while let Some(frame) = stack.last_mut() {
                let dependencies: Vec<DependencyId> =
                    self.outgoing_dependencies(frame.node).collect();

                if frame.next_dependency_index
                    >= dependencies.len()
                {
                    color.insert(frame.node, 2);
                    path_position.remove(&frame.node);
                    path.pop();
                    stack.pop();
                    continue;
                }

                let dependency_id =
                    dependencies[frame.next_dependency_index];

                frame.next_dependency_index += 1;

                let Some(dependency) =
                    self.dependencies.get(&dependency_id)
                else {
                    // A malformed graph cannot normally reach this point
                    // because all mutation methods preserve the invariant.
                    // Ignore it here; validate_structure() reports it.
                    continue;
                };

                let successor = dependency.to();

                match color.get(&successor).copied().unwrap_or(0) {
                    0 => {
                        let position = path.len();

                        color.insert(successor, 1);
                        path_position.insert(
                            successor,
                            position,
                        );
                        path.push(successor);

                        stack.push(Frame {
                            node: successor,
                            next_dependency_index: 0,
                        });
                    }

                    1 => {
                        let Some(&cycle_start) =
                            path_position.get(&successor)
                        else {
                            continue;
                        };

                        let mut cycle =
                            path[cycle_start..].to_vec();
                        cycle.push(successor);

                        return DependencyCycle::new(cycle);
                    }

                    2 => {}
                    _ => {}
                }
            }
        }

        None
    }

    /// Validates all internal graph invariants without performing scheduling.
    ///
    /// This should be used by deserialization and adapter boundaries before a
    /// graph is accepted by a production scheduler.
    pub fn validate_structure(
        &self,
    ) -> SchedulingResult<()> {
        // Every registered node must have both adjacency entries.
        for operation in &self.nodes {
            if !self.outgoing.contains_key(operation) {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(operation.id()),
                        successor: None,
                        reason: String::from(
                            "registered operation has no outgoing adjacency entry",
                        ),
                    },
                );
            }

            if !self.incoming.contains_key(operation) {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(operation.id()),
                        successor: None,
                        reason: String::from(
                            "registered operation has no incoming adjacency entry",
                        ),
                    },
                );
            }
        }

        // Every dependency must have registered endpoints and reciprocal
        // adjacency entries.
        for (dependency_id, dependency) in &self.dependencies {
            let predecessor = dependency.from();
            let successor = dependency.to();

            if predecessor == successor {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "self-dependency violates graph invariants",
                        ),
                    },
                );
            }

            if !self.nodes.contains(&predecessor) {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "dependency predecessor is not registered",
                        ),
                    },
                );
            }

            if !self.nodes.contains(&successor) {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "dependency successor is not registered",
                        ),
                    },
                );
            }

            let outgoing_contains = self
                .outgoing
                .get(&predecessor)
                .map(|edges| edges.contains(dependency_id))
                .unwrap_or(false);

            if !outgoing_contains {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "dependency is missing from outgoing adjacency",
                        ),
                    },
                );
            }

            let incoming_contains = self
                .incoming
                .get(&successor)
                .map(|edges| edges.contains(dependency_id))
                .unwrap_or(false);

            if !incoming_contains {
                return Err(
                    SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "dependency is missing from incoming adjacency",
                        ),
                    },
                );
            }
        }

        // Every adjacency reference must resolve to a dependency whose
        // endpoint matches the adjacency owner.
        for (operation, dependencies) in &self.outgoing {
            for dependency_id in dependencies {
                let Some(dependency) =
                    self.dependencies.get(dependency_id)
                else {
                    return Err(
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: Some(operation.id()),
                            successor: None,
                            reason: String::from(
                                "outgoing adjacency references an unknown dependency",
                            ),
                        },
                    );
                };

                if dependency.from() != *operation {
                    return Err(
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: Some(operation.id()),
                            successor: Some(
                                dependency.to().id(),
                            ),
                            reason: String::from(
                                "outgoing adjacency endpoint does not match dependency source",
                            ),
                        },
                    );
                }
            }
        }

        for (operation, dependencies) in &self.incoming {
            for dependency_id in dependencies {
                let Some(dependency) =
                    self.dependencies.get(dependency_id)
                else {
                    return Err(
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: None,
                            successor: Some(operation.id()),
                            reason: String::from(
                                "incoming adjacency references an unknown dependency",
                            ),
                        },
                    );
                };

                if dependency.to() != *operation {
                    return Err(
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: Some(
                                dependency.from().id(),
                            ),
                            successor: Some(operation.id()),
                            reason: String::from(
                                "incoming adjacency endpoint does not match dependency target",
                            ),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns the dependency kinds on edges from one operation to another.
    ///
    /// The returned collection is deterministic and preserves multiple
    /// dependency records when their kinds differ.
    #[must_use]
    pub fn dependency_kinds(
        &self,
        predecessor: OperationRef,
        successor: OperationRef,
    ) -> Vec<DependencyKind> {
        let Some(dependencies) =
            self.outgoing.get(&predecessor)
        else {
            return Vec::new();
        };

        let mut kinds = Vec::new();

        for dependency_id in dependencies {
            let Some(dependency) =
                self.dependencies.get(dependency_id)
            else {
                continue;
            };

            if dependency.to() == successor {
                kinds.push(dependency.kind());
            }
        }

        kinds
    }

    /// Returns all dependencies directly connecting two operations.
    ///
    /// Multiple records are preserved.
    #[must_use]
    pub fn dependencies_between(
        &self,
        predecessor: OperationRef,
        successor: OperationRef,
    ) -> Vec<DependencyRef> {
        let Some(dependencies) =
            self.outgoing.get(&predecessor)
        else {
            return Vec::new();
        };

        dependencies
            .iter()
            .filter_map(|dependency_id| {
                self.dependencies.get(dependency_id).copied()
            })
            .filter(|dependency| dependency.to() == successor)
            .collect()
    }

    /// Returns all graph nodes reachable from an operation.
    ///
    /// The traversal is iterative and deterministic.
    #[must_use]
    pub fn reachable_successors(
        &self,
        start: OperationRef,
    ) -> Vec<OperationRef> {
        if !self.nodes.contains(&start) {
            return Vec::new();
        }

        let mut visited = BTreeSet::new();
        let mut frontier = BTreeSet::new();

        frontier.insert(start);

        while let Some(operation) = frontier.pop_first() {
            if !visited.insert(operation) {
                continue;
            }

            for successor in self.successors(operation) {
                if !visited.contains(&successor) {
                    frontier.insert(successor);
                }
            }
        }

        visited.remove(&start);
        visited.into_iter().collect()
    }

    /// Returns all graph nodes that can reach an operation.
    ///
    /// The traversal is iterative and deterministic.
    #[must_use]
    pub fn reachable_predecessors(
        &self,
        start: OperationRef,
    ) -> Vec<OperationRef> {
        if !self.nodes.contains(&start) {
            return Vec::new();
        }

        let mut visited = BTreeSet::new();
        let mut frontier = BTreeSet::new();

        frontier.insert(start);

        while let Some(operation) = frontier.pop_first() {
            if !visited.insert(operation) {
                continue;
            }

            for predecessor in self.predecessors(operation) {
                if !visited.contains(&predecessor) {
                    frontier.insert(predecessor);
                }
            }
        }

        visited.remove(&start);
        visited.into_iter().collect()
    }

    /// Returns a deterministic immutable graph snapshot.
    ///
    /// The returned clone is useful when a caller needs an independently owned
    /// analysis graph. For ordinary concurrent read-only use, prefer
    /// `Arc<DependencyGraph>` to avoid copying the graph.
    #[must_use]
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

// =============================================================================
// Formatting
// =============================================================================

impl fmt::Display for DependencyGraph {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        writeln!(
            formatter,
            "DependencyGraph(nodes={}, dependencies={})",
            self.node_count(),
            self.dependency_count()
        )?;

        for operation in &self.nodes {
            write!(formatter, "  {operation}")?;

            let dependencies =
                self.outgoing.get(operation);

            if let Some(dependencies) = dependencies {
                if !dependencies.is_empty() {
                    formatter.write_str(" -> ")?;

                    let mut first = true;

                    for dependency_id in dependencies {
                        let Some(dependency) =
                            self.dependencies.get(dependency_id)
                        else {
                            continue;
                        };

                        if !first {
                            formatter.write_str(", ")?;
                        }

                        first = false;

                        write!(
                            formatter,
                            "{}",
                            dependency.to()
                        )?;
                    }
                }
            }

            formatter.write_str("\n")?;
        }

        Ok(())
    }
}

// =============================================================================
// Error integration
// =============================================================================

/// Extension helper for adding stable diagnostic context to a scheduling
/// cycle error.
///
/// The canonical error type deliberately owns the machine-readable category;
/// this helper only adds the deterministic cycle explanation to the existing
/// reason field.
trait SchedulingErrorReason {
    /// Adds a human-readable diagnostic reason.
    fn with_reason(self, reason: String) -> Self;
}

impl SchedulingErrorReason for SchedulingError {
    fn with_reason(
        self,
        reason: String,
    ) -> Self {
        match self {
            Self::CycleDetected {
                operation,
                dependency,
                cycle_size,
            } => Self::InvalidDependencyGraph {
                dependency,
                predecessor: operation,
                successor: None,
                reason,
            }
            .with_cycle_metadata(cycle_size),

            other => other,
        }
    }
}

/// Internal helper used to preserve cycle-size information when converting
/// into the canonical invalid-graph representation.
///
/// The method intentionally remains private to this module.
trait CycleMetadata {
    /// Attaches cycle metadata to a structured error.
    fn with_cycle_metadata(self, cycle_size: Option<u128>) -> Self;
}

impl CycleMetadata for SchedulingError {
    fn with_cycle_metadata(
        self,
        cycle_size: Option<u128>,
    ) -> Self {
        match self {
            Self::InvalidDependencyGraph {
                dependency,
                predecessor,
                successor,
                reason,
            } => Self::InvalidDependencyGraph {
                dependency,
                predecessor,
                successor,
                reason: match cycle_size {
                    Some(size) => format!(
                        "{reason}; cycle contains at least {size} unresolved node(s)"
                    ),
                    None => reason,
                },
            },

            other => other,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationRef {
        OperationRef::new(OperationId::new(value))
    }

    fn dependency(
        value: u64,
        from: OperationRef,
        to: OperationRef,
    ) -> DependencyRef {
        DependencyRef::new(
            DependencyId::new(value),
            from,
            to,
            DependencyKind::QuantumData,
        )
        .expect("test dependency must be valid")
    }

    #[test]
    fn empty_graph_is_empty() {
        let graph = DependencyGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn operation_registration_is_idempotent() {
        let mut graph = DependencyGraph::new();
        let first = operation(1);

        assert_eq!(
            graph
                .add_operation(first)
                .expect("registration must succeed"),
            true
        );

        assert_eq!(
            graph
                .add_operation(first)
                .expect("duplicate registration must be idempotent"),
            false
        );

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn dependency_requires_registered_endpoints() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        let edge = dependency(1, first, second);

        let error = graph
            .add_dependency(edge)
            .expect_err("unregistered endpoints must be rejected");

        assert_eq!(
            error.kind(),
            crate::quantum::scheduling::errors::SchedulingErrorKind::InvalidDependencyGraph
        );
    }

    #[test]
    fn dependency_is_indexed_both_directions() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph
            .add_operation(first)
            .expect("registration must succeed");

        graph
            .add_operation(second)
            .expect("registration must succeed");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        assert_eq!(
            graph.successors(first),
            vec![second]
        );

        assert_eq!(
            graph.predecessors(second),
            vec![first]
        );

        assert_eq!(graph.in_degree(second), 1);
        assert_eq!(graph.out_degree(first), 1);
    }

    #[test]
    fn multiple_dependency_reasons_are_preserved() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph
            .add_operation(first)
            .expect("registration must succeed");

        graph
            .add_operation(second)
            .expect("registration must succeed");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("first dependency must succeed");

        graph
            .add_dependency(dependency(2, first, second))
            .expect("second dependency must succeed");

        assert_eq!(
            graph.dependencies_between(first, second).len(),
            2
        );

        assert_eq!(
            graph.successors(first),
            vec![second]
        );

        assert_eq!(
            graph.out_degree(first),
            2
        );
    }

    #[test]
    fn deterministic_topological_order_is_stable() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([third, first, second])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, third))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, second, third))
            .expect("dependency must succeed");

        let order = graph
            .topological_order()
            .expect("graph must be acyclic");

        assert_eq!(
            order,
            vec![first, second, third]
        );
    }

    #[test]
    fn cycle_is_detected() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, second, third))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(3, third, first))
            .expect("dependency must succeed");

        let cycle =
            graph.find_cycle().expect("cycle must be found");

        assert_eq!(cycle.node_count(), 3);
        assert_eq!(
            cycle.operations().first(),
            cycle.operations().last()
        );

        assert!(
            graph.validate_acyclic().is_err(),
            "cyclic graph must fail DAG validation"
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut graph = DependencyGraph::new();
        let first = operation(1);

        graph
            .add_operation(first)
            .expect("operation must register");

        let error = graph
            .add_dependency(dependency(1, first, first))
            .expect_err("self dependency must be rejected");

        assert_eq!(
            error.kind(),
            crate::quantum::scheduling::errors::SchedulingErrorKind::InvalidDependencyGraph
        );
    }

    #[test]
    fn removing_operation_removes_incident_dependencies() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, second, third))
            .expect("dependency must succeed");

        graph
            .remove_operation(second)
            .expect("operation removal must succeed");

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.successors(first).is_empty());
        assert!(graph.predecessors(third).is_empty());
    }

    #[test]
    fn structure_validation_succeeds_for_valid_graph() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph
            .add_operations([first, second])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .validate_structure()
            .expect("valid graph must pass validation");
    }

    #[test]
    fn roots_and_leaves_are_correct() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, second, third))
            .expect("dependency must succeed");

        assert_eq!(graph.roots(), vec![first]);
        assert_eq!(graph.leaves(), vec![third]);
    }

    #[test]
    fn reachability_is_deterministic() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);
        let fourth = operation(4);

        graph
            .add_operations([first, second, third, fourth])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, first, third))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(3, third, fourth))
            .expect("dependency must succeed");

        assert_eq!(
            graph.reachable_successors(first),
            vec![second, third, fourth]
        );

        assert_eq!(
            graph.reachable_predecessors(fourth),
            vec![first, third]
        );
    }

    #[test]
    fn statistics_are_consistent() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .expect("operations must register");

        graph
            .add_dependency(dependency(1, first, second))
            .expect("dependency must succeed");

        graph
            .add_dependency(dependency(2, first, third))
            .expect("dependency must succeed");

        let stats = graph.statistics();

        assert_eq!(stats.nodes, 3);
        assert_eq!(stats.dependencies, 2);
        assert_eq!(stats.roots, 1);
        assert_eq!(stats.leaves, 2);
        assert_eq!(stats.maximum_out_degree, 2);
        assert_eq!(stats.maximum_in_degree, 1);
    }
}