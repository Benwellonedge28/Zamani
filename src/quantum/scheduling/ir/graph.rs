//! Zamani Quantum Scheduling — Dependency Graph
//!
//! This module owns the scheduler's directed dependency graph.
//!
//! # Responsibility
//!
//! The graph answers:
//!
//! > "Which scheduling operations must precede which other operations?"
//!
//! It does NOT own:
//!
//! - quantum operation semantics;
//! - qubit identity;
//! - physical topology;
//! - routing;
//! - resource calendars;
//! - operation timing;
//! - scheduling policy;
//! - scheduling algorithms;
//! - QEC semantics;
//! - hardware discovery;
//! - runtime execution.
//!
//! Those concerns belong to their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! This module deliberately reuses:
//!
//! ```text
//! crate::quantum::scheduling::types::OperationRef
//! crate::quantum::scheduling::types::DependencyRef
//! crate::quantum::scheduling::types::DependencyId
//! crate::quantum::scheduling::types::DependencyKind
//! ```
//!
//! `OperationRef` itself wraps the canonical:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! The graph therefore MUST NOT define another operation identity.
//!
//! Likewise, this file intentionally does not import `QubitId`.
//! A dependency graph is expressed between operations. Qubit-aware dependency
//! construction belongs in the operation/IR adapter layer and MUST use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! when qubit identity is required.
//!
//! # Graph model
//!
//! The graph is a directed multigraph:
//!
//! ```text
//! Operation A ──dependency 1──> Operation B
//! Operation A ──dependency 2──> Operation B
//! ```
//!
//! Multiple dependency edges between the same pair of operations are legal
//! because different constraints may independently require the same ordering.
//! For example, two operations may simultaneously have a semantic dependency
//! and an explicit program-order dependency.
//!
//! Dependency identity is therefore the authoritative edge key.
//!
//! # Determinism
//!
//! `BTreeMap` and `BTreeSet` are deliberately used rather than hash-based
//! collections for the graph's canonical traversal structures.
//!
//! This provides deterministic ordering independent of hash randomization.
//!
//! Deterministic ordering is particularly important for:
//!
//! - reproducible compilation;
//! - deterministic scheduler tests;
//! - schedule provenance;
//! - debugging;
//! - regression testing;
//! - distributed compilation;
//! - stable serialization.
//!
//! The complexity is O((V + E) log V) for ordinary graph construction and
//! traversal rather than the ideal O(V + E) possible with hash-based adjacency
//! structures. This trade-off is intentional at the canonical deterministic
//! graph boundary.
//!
//! Implementations that require a different performance/memory trade-off may
//! build an adapter or specialized graph representation without changing this
//! semantic contract.
//!
//! # Scalability
//!
//! There are NO hard-coded limits for:
//!
//! - operations;
//! - dependencies;
//! - qubits;
//! - graph depth;
//! - graph width;
//! - machine size;
//! - resource count;
//! - topology size.
//!
//! Concrete limits are determined by available memory, execution policy and
//! explicit scheduler limits supplied by the surrounding compilation context.
//!
//! Graph traversals are iterative rather than recursive so graph depth does
//! not consume the call stack.
//!
//! # Core invariants
//!
//! A valid graph guarantees:
//!
//! 1. Every dependency endpoint is a registered operation.
//! 2. Every dependency exists in the edge store exactly once.
//! 3. Every edge is present in both its source's outgoing index and its
//!    destination's incoming index.
//! 4. No dependency is a self-edge.
//! 5. Dependency IDs are unique within the graph.
//! 6. Operation references are unique within the graph.
//! 7. Topological order exists iff the graph is acyclic.
//!
//! # Mutation
//!
//! The graph is owned by its scheduler context. It has no global mutable state.
//! Mutation methods maintain all indexes atomically from the caller's
//! perspective: a successful mutation leaves the graph internally consistent;
//! a rejected mutation leaves it unchanged.
//!
//! # Concurrency
//!
//! The graph itself does not use locks or interior mutability. This keeps the
//! ownership model explicit and makes it naturally usable inside immutable
//! scheduler snapshots (`Arc<DependencyGraph>`) once construction is complete.
//!
//! Concurrent graph construction should be performed by an explicit higher
//! level builder/partitioner and merged through this API rather than adding
//! global synchronization to the graph.
//!
//! # Rust
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ▼
//! scheduling::ir::operation
//!      │
//!      ├──────────────► scheduling::ir::dependency
//!      │
//!      ▼
//! scheduling::ir::graph   ◄── this module
//!      │
//!      ├──────────────► scheduling::ir::critical_path
//!      │
//!      ├──────────────► scheduling::planners
//!      │
//!      ├──────────────► scheduling::policies
//!      │
//!      └──────────────► scheduling::verification
//! ```
//!
//! The graph must remain independent of the planner. Planners consume it;
//! they do not define its representation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::super::types::{
    DependencyId,
    DependencyRef,
    OperationRef,
};

// =============================================================================
// Error type
// =============================================================================

/// Errors produced while constructing or querying a dependency graph.
///
/// This error type is deliberately local to the graph boundary so the graph
/// does not need to depend on higher-level scheduler error construction.
///
/// The scheduling `errors.rs` layer can map these errors into its canonical
/// `SchedulingError` without requiring changes to this graph implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DependencyGraphError {
    /// An operation was registered more than once.
    DuplicateOperation {
        /// The duplicated operation.
        operation: OperationRef,
    },

    /// A dependency refers to an operation that is not registered.
    UnknownOperation {
        /// The operation that was not found.
        operation: OperationRef,
        /// The dependency that referenced the operation, when available.
        dependency: Option<DependencyId>,
    },

    /// A dependency ID already exists in this graph.
    DuplicateDependency {
        /// The duplicated dependency.
        dependency: DependencyId,
    },

    /// A self-dependency was supplied.
    SelfDependency {
        /// The operation on both ends of the invalid dependency.
        operation: OperationRef,
    },

    /// The graph contains a directed cycle.
    CycleDetected {
        /// A deterministic representation of one discovered cycle.
        ///
        /// The first operation is repeated at the end when a non-empty cycle
        /// is available.
        cycle: Vec<OperationRef>,
    },

    /// An internal index invariant was violated.
    ///
    /// This indicates a programming error or memory/state corruption in graph
    /// construction logic rather than a normal user scheduling error.
    InconsistentState {
        /// Stable explanation suitable for diagnostics.
        reason: &'static str,
    },

    /// A requested operation does not exist.
    OperationNotFound {
        /// Missing operation.
        operation: OperationRef,
    },

    /// A requested dependency does not exist.
    DependencyNotFound {
        /// Missing dependency.
        dependency: DependencyId,
    },
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate scheduling operation: {operation}")
            }
            Self::UnknownOperation {
                operation,
                dependency,
            } => {
                if let Some(dependency) = dependency {
                    write!(
                        formatter,
                        "dependency {dependency} references unknown operation {operation}"
                    )
                } else {
                    write!(
                        formatter,
                        "unknown scheduling operation: {operation}"
                    )
                }
            }
            Self::DuplicateDependency { dependency } => {
                write!(
                    formatter,
                    "duplicate scheduling dependency: {dependency}"
                )
            }
            Self::SelfDependency { operation } => {
                write!(
                    formatter,
                    "operation {operation} cannot depend on itself"
                )
            }
            Self::CycleDetected { cycle } => {
                if cycle.is_empty() {
                    write!(formatter, "dependency graph contains a cycle")
                } else {
                    write!(formatter, "dependency graph contains a cycle: ")?;
                    for (index, operation) in cycle.iter().enumerate() {
                        if index != 0 {
                            write!(formatter, " -> ")?;
                        }
                        write!(formatter, "{operation}")?;
                    }
                    Ok(())
                }
            }
            Self::InconsistentState { reason } => {
                write!(formatter, "dependency graph invariant violated: {reason}")
            }
            Self::OperationNotFound { operation } => {
                write!(formatter, "operation not found: {operation}")
            }
            Self::DependencyNotFound { dependency } => {
                write!(formatter, "dependency not found: {dependency}")
            }
        }
    }
}

impl Error for DependencyGraphError {}

/// Result type used by graph operations.
pub type DependencyGraphResult<T> = Result<T, DependencyGraphError>;

// =============================================================================
// Graph
// =============================================================================

/// Deterministic directed dependency multigraph for quantum scheduling.
///
/// The graph stores:
///
/// - registered operations;
/// - dependency edges;
/// - outgoing edge indexes;
/// - incoming edge indexes.
///
/// The graph is intentionally independent of:
///
/// - qubit topology;
/// - resource availability;
/// - timing;
/// - scheduling policy.
///
/// Those are evaluated after or alongside dependency analysis by higher-level
/// scheduler components.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// All registered operations.
    nodes: BTreeSet<OperationRef>,

    /// Dependency edges keyed by their stable dependency identity.
    edges: BTreeMap<DependencyId, DependencyRef>,

    /// Source operation -> outgoing dependency IDs.
    outgoing: BTreeMap<OperationRef, BTreeSet<DependencyId>>,

    /// Destination operation -> incoming dependency IDs.
    incoming: BTreeMap<OperationRef, BTreeSet<DependencyId>>,
}

impl DependencyGraph {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a graph containing the supplied operations.
    ///
    /// Dependencies are intentionally added separately so callers can construct
    /// the operation set first and receive deterministic validation errors for
    /// missing endpoints.
    pub fn from_operations<I>(
        operations: I,
    ) -> DependencyGraphResult<Self>
    where
        I: IntoIterator<Item = OperationRef>,
    {
        let mut graph = Self::new();

        for operation in operations {
            graph.add_operation(operation)?;
        }

        Ok(graph)
    }

    /// Registers one operation.
    ///
    /// A duplicate operation is rejected rather than silently ignored. This is
    /// important because duplicate operation identity usually indicates an IR
    /// construction or adapter bug.
    pub fn add_operation(
        &mut self,
        operation: OperationRef,
    ) -> DependencyGraphResult<()> {
        if !self.nodes.insert(operation) {
            return Err(DependencyGraphError::DuplicateOperation {
                operation,
            });
        }

        self.outgoing.entry(operation).or_default();
        self.incoming.entry(operation).or_default();

        Ok(())
    }

    /// Registers multiple operations atomically.
    ///
    /// If any operation is duplicated, the graph is restored to its original
    /// state before returning the error.
    pub fn add_operations<I>(
        &mut self,
        operations: I,
    ) -> DependencyGraphResult<()>
    where
        I: IntoIterator<Item = OperationRef>,
    {
        let additions: Vec<OperationRef> = operations.into_iter().collect();

        let mut seen = BTreeSet::new();

        for &operation in &additions {
            if !seen.insert(operation) || self.nodes.contains(&operation) {
                return Err(DependencyGraphError::DuplicateOperation {
                    operation,
                });
            }
        }

        for operation in additions {
            self.nodes.insert(operation);
            self.outgoing.entry(operation).or_default();
            self.incoming.entry(operation).or_default();
        }

        Ok(())
    }

    // =========================================================================
    // Dependency mutation
    // =========================================================================

    /// Adds one dependency edge.
    ///
    /// Both endpoint operations must already be registered.
    ///
    /// Multiple dependency edges between the same pair of operations are
    /// permitted as long as their `DependencyId`s differ.
    ///
    /// The graph is unchanged when the operation fails.
    pub fn add_dependency(
        &mut self,
        dependency: DependencyRef,
    ) -> DependencyGraphResult<()> {
        let dependency_id = dependency.id();
        let from = dependency.from();
        let to = dependency.to();

        if from == to {
            return Err(DependencyGraphError::SelfDependency {
                operation: from,
            });
        }

        if self.edges.contains_key(&dependency_id) {
            return Err(DependencyGraphError::DuplicateDependency {
                dependency: dependency_id,
            });
        }

        if !self.nodes.contains(&from) {
            return Err(DependencyGraphError::UnknownOperation {
                operation: from,
                dependency: Some(dependency_id),
            });
        }

        if !self.nodes.contains(&to) {
            return Err(DependencyGraphError::UnknownOperation {
                operation: to,
                dependency: Some(dependency_id),
            });
        }

        self.edges.insert(dependency_id, dependency);

        self.outgoing
            .entry(from)
            .or_default()
            .insert(dependency_id);

        self.incoming
            .entry(to)
            .or_default()
            .insert(dependency_id);

        Ok(())
    }

    /// Adds multiple dependency edges atomically.
    ///
    /// The graph is unchanged if any edge fails validation.
    pub fn add_dependencies<I>(
        &mut self,
        dependencies: I,
    ) -> DependencyGraphResult<()>
    where
        I: IntoIterator<Item = DependencyRef>,
    {
        let additions: Vec<DependencyRef> = dependencies.into_iter().collect();

        let mut ids = BTreeSet::new();

        for dependency in &additions {
            let dependency_id = dependency.id();
            let from = dependency.from();
            let to = dependency.to();

            if from == to {
                return Err(DependencyGraphError::SelfDependency {
                    operation: from,
                });
            }

            if !ids.insert(dependency_id)
                || self.edges.contains_key(&dependency_id)
            {
                return Err(DependencyGraphError::DuplicateDependency {
                    dependency: dependency_id,
                });
            }

            if !self.nodes.contains(&from) {
                return Err(DependencyGraphError::UnknownOperation {
                    operation: from,
                    dependency: Some(dependency_id),
                });
            }

            if !self.nodes.contains(&to) {
                return Err(DependencyGraphError::UnknownOperation {
                    operation: to,
                    dependency: Some(dependency_id),
                });
            }
        }

        for dependency in additions {
            let dependency_id = dependency.id();
            let from = dependency.from();
            let to = dependency.to();

            self.edges.insert(dependency_id, dependency);

            self.outgoing
                .entry(from)
                .or_default()
                .insert(dependency_id);

            self.incoming
                .entry(to)
                .or_default()
                .insert(dependency_id);
        }

        Ok(())
    }

    /// Removes a dependency edge.
    ///
    /// Returns `true` if an edge was removed and `false` if it did not exist.
    pub fn remove_dependency(
        &mut self,
        dependency: DependencyId,
    ) -> bool {
        let Some(edge) = self.edges.remove(&dependency) else {
            return false;
        };

        if let Some(outgoing) = self.outgoing.get_mut(&edge.from()) {
            outgoing.remove(&dependency);
        }

        if let Some(incoming) = self.incoming.get_mut(&edge.to()) {
            incoming.remove(&dependency);
        }

        true
    }

    /// Removes an operation and every dependency incident to it.
    ///
    /// Returns the number of removed dependencies.
    ///
    /// The operation itself and all its adjacency indexes are removed as one
    /// logical mutation.
    pub fn remove_operation(
        &mut self,
        operation: OperationRef,
    ) -> DependencyGraphResult<usize> {
        if !self.nodes.contains(&operation) {
            return Err(DependencyGraphError::OperationNotFound {
                operation,
            });
        }

        let mut incident = BTreeSet::new();

        if let Some(outgoing) = self.outgoing.get(&operation) {
            incident.extend(outgoing.iter().copied());
        }

        if let Some(incoming) = self.incoming.get(&operation) {
            incident.extend(incoming.iter().copied());
        }

        let removed_dependencies = incident.len();

        for dependency in incident {
            self.remove_dependency(dependency);
        }

        self.outgoing.remove(&operation);
        self.incoming.remove(&operation);
        self.nodes.remove(&operation);

        Ok(removed_dependencies)
    }

    /// Removes all dependency edges while preserving operations.
    pub fn clear_dependencies(&mut self) {
        self.edges.clear();

        for dependencies in self.outgoing.values_mut() {
            dependencies.clear();
        }

        for dependencies in self.incoming.values_mut() {
            dependencies.clear();
        }
    }

    /// Removes all operations and dependencies.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.outgoing.clear();
        self.incoming.clear();
    }

    // =========================================================================
    // Basic queries
    // =========================================================================

    /// Returns the number of registered operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether the graph contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether the graph contains an operation.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationRef,
    ) -> bool {
        self.nodes.contains(&operation)
    }

    /// Returns whether the graph contains a dependency.
    #[must_use]
    pub fn contains_dependency(
        &self,
        dependency: DependencyId,
    ) -> bool {
        self.edges.contains_key(&dependency)
    }

    /// Returns the operations in deterministic order.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = &OperationRef> {
        self.nodes.iter()
    }

    /// Returns the dependencies in deterministic dependency-ID order.
    pub fn dependencies(
        &self,
    ) -> impl Iterator<Item = &DependencyRef> {
        self.edges.values()
    }

    /// Returns a dependency by identity.
    #[must_use]
    pub fn dependency(
        &self,
        dependency: DependencyId,
    ) -> Option<&DependencyRef> {
        self.edges.get(&dependency)
    }

    /// Returns the number of incoming dependencies of an operation.
    ///
    /// This is the operation's indegree.
    #[must_use]
    pub fn indegree(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<usize> {
        self.ensure_operation(operation)?;

        Ok(self
            .incoming
            .get(&operation)
            .map_or(0, BTreeSet::len))
    }

    /// Returns the number of outgoing dependencies of an operation.
    ///
    /// This is the operation's outdegree.
    #[must_use]
    pub fn outdegree(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<usize> {
        self.ensure_operation(operation)?;

        Ok(self
            .outgoing
            .get(&operation)
            .map_or(0, BTreeSet::len))
    }

    // =========================================================================
    // Adjacency queries
    // =========================================================================

    /// Returns incoming dependency IDs for an operation.
    ///
    /// The returned vector is deterministic and ordered by dependency ID.
    pub fn incoming_dependencies(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<DependencyId>> {
        self.ensure_operation(operation)?;

        Ok(self
            .incoming
            .get(&operation)
            .map_or_else(Vec::new, |dependencies| {
                dependencies.iter().copied().collect()
            }))
    }

    /// Returns outgoing dependency IDs for an operation.
    ///
    /// The returned vector is deterministic and ordered by dependency ID.
    pub fn outgoing_dependencies(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<DependencyId>> {
        self.ensure_operation(operation)?;

        Ok(self
            .outgoing
            .get(&operation)
            .map_or_else(Vec::new, |dependencies| {
                dependencies.iter().copied().collect()
            }))
    }

    /// Returns predecessor operations.
    ///
    /// Multiple dependency edges between the same pair are collapsed into one
    /// predecessor operation.
    pub fn predecessors(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<OperationRef>> {
        self.ensure_operation(operation)?;

        let mut result = BTreeSet::new();

        if let Some(dependencies) = self.incoming.get(&operation) {
            for dependency_id in dependencies {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "incoming index references a missing dependency",
                    })?;

                result.insert(dependency.from());
            }
        }

        Ok(result.into_iter().collect())
    }

    /// Returns successor operations.
    ///
    /// Multiple dependency edges between the same pair are collapsed into one
    /// successor operation.
    pub fn successors(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<OperationRef>> {
        self.ensure_operation(operation)?;

        let mut result = BTreeSet::new();

        if let Some(dependencies) = self.outgoing.get(&operation) {
            for dependency_id in dependencies {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "outgoing index references a missing dependency",
                    })?;

                result.insert(dependency.to());
            }
        }

        Ok(result.into_iter().collect())
    }

    /// Returns dependency references incoming to an operation.
    pub fn incoming_edges(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<DependencyRef>> {
        self.ensure_operation(operation)?;

        let dependencies = self
            .incoming
            .get(&operation)
            .ok_or(DependencyGraphError::InconsistentState {
                reason: "registered operation has no incoming index",
            })?;

        dependencies
            .iter()
            .map(|dependency_id| {
                self.edges
                    .get(dependency_id)
                    .copied()
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "incoming index references a missing dependency",
                    })
            })
            .collect()
    }

    /// Returns dependency references outgoing from an operation.
    pub fn outgoing_edges(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<Vec<DependencyRef>> {
        self.ensure_operation(operation)?;

        let dependencies = self
            .outgoing
            .get(&operation)
            .ok_or(DependencyGraphError::InconsistentState {
                reason: "registered operation has no outgoing index",
            })?;

        dependencies
            .iter()
            .map(|dependency_id| {
                self.edges
                    .get(dependency_id)
                    .copied()
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "outgoing index references a missing dependency",
                    })
            })
            .collect()
    }

    // =========================================================================
    // Root and leaf operations
    // =========================================================================

    /// Returns operations with no incoming dependencies.
    ///
    /// These are the initial ready candidates for dependency-only scheduling.
    ///
    /// Resource and timing constraints may still prevent them from starting.
    pub fn roots(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .filter(|operation| {
                self.incoming
                    .get(operation)
                    .is_none_or(BTreeSet::is_empty)
            })
            .copied()
            .collect()
    }

    /// Returns operations with no outgoing dependencies.
    ///
    /// These are terminal nodes in the dependency graph.
    pub fn leaves(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .filter(|operation| {
                self.outgoing
                    .get(operation)
                    .is_none_or(BTreeSet::is_empty)
            })
            .copied()
            .collect()
    }

    // =========================================================================
    // Acyclicity
    // =========================================================================

    /// Returns whether the dependency graph is acyclic.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.kahn_order().is_ok()
    }

    /// Validates that the graph is acyclic.
    pub fn validate_acyclic(&self) -> DependencyGraphResult<()> {
        self.kahn_order().map(|_| ())
    }

    /// Finds one deterministic directed cycle if one exists.
    ///
    /// The implementation is iterative and therefore does not consume the
    /// process call stack according to graph depth.
    #[must_use]
    pub fn find_cycle(&self) -> Option<Vec<OperationRef>> {
        let remaining = self.remaining_after_kahn().ok()?;

        if remaining.is_empty() {
            return None;
        }

        self.find_cycle_in_residual(&remaining)
    }

    // =========================================================================
    // Topological ordering
    // =========================================================================

    /// Returns a deterministic topological ordering.
    ///
    /// Kahn's algorithm is used with a `BTreeSet` ready queue. Consequently,
    /// when multiple operations are simultaneously ready, the smallest
    /// `OperationRef` is selected first.
    ///
    /// This ordering is deterministic but is NOT itself a scheduling policy.
    /// Planners may use their own priority rules.
    pub fn topological_order(
        &self,
    ) -> DependencyGraphResult<Vec<OperationRef>> {
        self.kahn_order()
    }

    /// Returns deterministic dependency levels.
    ///
    /// Level zero contains all roots.
    ///
    /// Each subsequent level contains operations whose complete predecessor set
    /// occurs in earlier levels.
    ///
    /// This is useful for:
    ///
    /// - parallel dependency analysis;
    /// - circuit-width estimation;
    /// - dependency-only depth;
    /// - scheduler diagnostics;
    /// - partitioning.
    ///
    /// It is NOT a resource-aware schedule.
    pub fn topological_levels(
        &self,
    ) -> DependencyGraphResult<Vec<Vec<OperationRef>>> {
        let mut indegree = self.build_indegree();

        let mut ready = BTreeSet::new();

        for operation in &self.nodes {
            if indegree.get(operation).copied().unwrap_or(0) == 0 {
                ready.insert(*operation);
            }
        }

        let mut levels = Vec::new();
        let mut processed = 0usize;

        while !ready.is_empty() {
            let current: Vec<OperationRef> = ready.iter().copied().collect();

            ready.clear();

            for operation in &current {
                processed += 1;

                let outgoing = self
                    .outgoing
                    .get(operation)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "registered operation has no outgoing index",
                    })?;

                for dependency_id in outgoing {
                    let dependency = self
                        .edges
                        .get(dependency_id)
                        .ok_or(DependencyGraphError::InconsistentState {
                            reason: "outgoing index references a missing dependency",
                        })?;

                    let successor = dependency.to();

                    let count = indegree.get_mut(&successor).ok_or(
                        DependencyGraphError::InconsistentState {
                            reason: "successor operation missing from indegree map",
                        },
                    )?;

                    *count = count.checked_sub(1).ok_or(
                        DependencyGraphError::InconsistentState {
                            reason: "indegree underflow while building levels",
                        },
                    )?;

                    if *count == 0 {
                        ready.insert(successor);
                    }
                }
            }

            levels.push(current);
        }

        if processed != self.nodes.len() {
            let cycle = self.find_cycle().unwrap_or_default();

            return Err(DependencyGraphError::CycleDetected { cycle });
        }

        Ok(levels)
    }

    // =========================================================================
    // Graph validation
    // =========================================================================

    /// Validates all graph storage invariants.
    ///
    /// This is intentionally public because production verification and
    /// regression tests can use it after graph transformations.
    pub fn validate(&self) -> DependencyGraphResult<()> {
        // Every operation must have both adjacency indexes.
        for operation in &self.nodes {
            if !self.outgoing.contains_key(operation) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "operation is missing outgoing adjacency index",
                });
            }

            if !self.incoming.contains_key(operation) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "operation is missing incoming adjacency index",
                });
            }
        }

        // No adjacency index may contain an unknown operation.
        for operation in self.outgoing.keys() {
            if !self.nodes.contains(operation) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "outgoing adjacency contains unknown operation",
                });
            }
        }

        for operation in self.incoming.keys() {
            if !self.nodes.contains(operation) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "incoming adjacency contains unknown operation",
                });
            }
        }

        // Every stored dependency must have valid endpoints and both indexes.
        for (dependency_id, dependency) in &self.edges {
            if *dependency_id != dependency.id() {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "dependency map key does not match dependency identity",
                });
            }

            let from = dependency.from();
            let to = dependency.to();

            if from == to {
                return Err(DependencyGraphError::SelfDependency {
                    operation: from,
                });
            }

            if !self.nodes.contains(&from) {
                return Err(DependencyGraphError::UnknownOperation {
                    operation: from,
                    dependency: Some(*dependency_id),
                });
            }

            if !self.nodes.contains(&to) {
                return Err(DependencyGraphError::UnknownOperation {
                    operation: to,
                    dependency: Some(*dependency_id),
                });
            }

            let outgoing = self.outgoing.get(&from).ok_or(
                DependencyGraphError::InconsistentState {
                    reason: "dependency source has no outgoing adjacency index",
                },
            )?;

            if !outgoing.contains(dependency_id) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "dependency missing from outgoing adjacency index",
                });
            }

            let incoming = self.incoming.get(&to).ok_or(
                DependencyGraphError::InconsistentState {
                    reason: "dependency destination has no incoming adjacency index",
                },
            )?;

            if !incoming.contains(dependency_id) {
                return Err(DependencyGraphError::InconsistentState {
                    reason: "dependency missing from incoming adjacency index",
                });
            }
        }

        // Every outgoing adjacency entry must reference a matching edge.
        for (operation, dependencies) in &self.outgoing {
            for dependency_id in dependencies {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "outgoing adjacency references missing edge",
                    })?;

                if dependency.from() != *operation {
                    return Err(DependencyGraphError::InconsistentState {
                        reason: "outgoing adjacency edge has incorrect source",
                    });
                }
            }
        }

        // Every incoming adjacency entry must reference a matching edge.
        for (operation, dependencies) in &self.incoming {
            for dependency_id in dependencies {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "incoming adjacency references missing edge",
                    })?;

                if dependency.to() != *operation {
                    return Err(DependencyGraphError::InconsistentState {
                        reason: "incoming adjacency edge has incorrect destination",
                    });
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn ensure_operation(
        &self,
        operation: OperationRef,
    ) -> DependencyGraphResult<()> {
        if self.nodes.contains(&operation) {
            Ok(())
        } else {
            Err(DependencyGraphError::OperationNotFound {
                operation,
            })
        }
    }

    fn build_indegree(
        &self,
    ) -> BTreeMap<OperationRef, usize> {
        self.nodes
            .iter()
            .copied()
            .map(|operation| {
                let degree = self
                    .incoming
                    .get(&operation)
                    .map_or(0, BTreeSet::len);

                (operation, degree)
            })
            .collect()
    }

    fn kahn_order(
        &self,
    ) -> DependencyGraphResult<Vec<OperationRef>> {
        let mut indegree = self.build_indegree();

        let mut ready = BTreeSet::new();

        for operation in &self.nodes {
            if indegree.get(operation).copied().unwrap_or(0) == 0 {
                ready.insert(*operation);
            }
        }

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(operation) = ready.pop_first() {
            order.push(operation);

            let outgoing = self
                .outgoing
                .get(&operation)
                .ok_or(DependencyGraphError::InconsistentState {
                    reason: "registered operation has no outgoing index",
                })?;

            for dependency_id in outgoing {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "outgoing index references missing dependency",
                    })?;

                let successor = dependency.to();

                let count = indegree.get_mut(&successor).ok_or(
                    DependencyGraphError::InconsistentState {
                        reason: "successor missing from indegree map",
                    },
                )?;

                *count = count.checked_sub(1).ok_or(
                    DependencyGraphError::InconsistentState {
                        reason: "indegree underflow during topological traversal",
                    },
                )?;

                if *count == 0 {
                    ready.insert(successor);
                }
            }
        }

        if order.len() != self.nodes.len() {
            let cycle = self.find_cycle().unwrap_or_default();

            return Err(DependencyGraphError::CycleDetected { cycle });
        }

        Ok(order)
    }

    /// Returns all operations that remain after Kahn elimination.
    ///
    /// An acyclic graph has no remaining nodes.
    fn remaining_after_kahn(
        &self,
    ) -> DependencyGraphResult<BTreeSet<OperationRef>> {
        let mut indegree = self.build_indegree();

        let mut ready = BTreeSet::new();

        for operation in &self.nodes {
            if indegree.get(operation).copied().unwrap_or(0) == 0 {
                ready.insert(*operation);
            }
        }

        let mut processed = BTreeSet::new();

        while let Some(operation) = ready.pop_first() {
            processed.insert(operation);

            let outgoing = self
                .outgoing
                .get(&operation)
                .ok_or(DependencyGraphError::InconsistentState {
                    reason: "registered operation has no outgoing index",
                })?;

            for dependency_id in outgoing {
                let dependency = self
                    .edges
                    .get(dependency_id)
                    .ok_or(DependencyGraphError::InconsistentState {
                        reason: "outgoing index references missing dependency",
                    })?;

                let successor = dependency.to();

                let count = indegree.get_mut(&successor).ok_or(
                    DependencyGraphError::InconsistentState {
                        reason: "successor missing from indegree map",
                    },
                )?;

                *count = count.checked_sub(1).ok_or(
                    DependencyGraphError::InconsistentState {
                        reason: "indegree underflow during cycle detection",
                    },
                )?;

                if *count == 0 {
                    ready.insert(successor);
                }
            }
        }

        Ok(self
            .nodes
            .difference(&processed)
            .copied()
            .collect())
    }

    /// Finds a cycle entirely inside a Kahn residual graph.
    ///
    /// Every residual vertex has at least one incoming edge from another
    /// residual vertex. Following incoming edges must therefore eventually
    /// repeat a vertex, yielding a directed cycle.
    fn find_cycle_in_residual(
        &self,
        residual: &BTreeSet<OperationRef>,
    ) -> Option<Vec<OperationRef>> {
        let start = residual.iter().next().copied()?;

        let mut path = Vec::new();
        let mut positions = BTreeMap::<OperationRef, usize>::new();

        let mut current = start;

        loop {
            if let Some(&position) = positions.get(&current) {
                let mut cycle = path[position..].to_vec();

                // The traversal followed incoming edges, so reverse it to
                // restore the actual dependency direction.
                cycle.reverse();

                if let Some(first) = cycle.first().copied() {
                    cycle.push(first);
                }

                return Some(cycle);
            }

            positions.insert(current, path.len());
            path.push(current);

            let incoming = self.incoming.get(&current)?;

            let predecessor = incoming
                .iter()
                .filter_map(|dependency_id| self.edges.get(dependency_id))
                .map(DependencyRef::from)
                .map(|dependency| dependency.from())
                .filter(|operation| residual.contains(operation))
                .min()?;

            current = predecessor;
        }
    }
}

// =============================================================================
// Trait implementations
// =============================================================================

impl FromIterator<OperationRef> for DependencyGraph {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = OperationRef>,
    {
        let mut graph = Self::new();

        for operation in iter {
            // `FromIterator` cannot return the duplicate error. A duplicate
            // operation has no semantic distinction from the already
            // registered operation in a set, so this constructor deliberately
            // retains the first occurrence.
            //
            // Call `add_operations` when duplicate rejection is required.
            if graph.nodes.insert(operation) {
                graph.outgoing.entry(operation).or_default();
                graph.incoming.entry(operation).or_default();
            }
        }

        graph
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::OperationId;
    use crate::quantum::scheduling::types::DependencyKind;

    fn operation(value: u64) -> OperationRef {
        OperationRef::new(OperationId::new(value))
    }

    fn dependency(
        id: u64,
        from: OperationRef,
        to: OperationRef,
        kind: DependencyKind,
    ) -> DependencyRef {
        DependencyRef::new(
            DependencyId::new(id),
            from,
            to,
            kind,
        )
        .expect("test dependency must not be a self-edge")
    }

    #[test]
    fn empty_graph_is_valid() {
        let graph = DependencyGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.operation_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.is_acyclic());
        assert!(graph.find_cycle().is_none());
        assert_eq!(
            graph.topological_order().expect("empty graph is valid"),
            Vec::<OperationRef>::new()
        );
        assert_eq!(
            graph.topological_levels().expect("empty graph is valid"),
            Vec::<Vec<OperationRef>>::new()
        );
        graph.validate().expect("empty graph is valid");
    }

    #[test]
    fn operation_registration_is_deterministic() {
        let first = operation(1);
        let second = operation(2);

        let graph = DependencyGraph::from_operations([second, first])
            .expect("operations should be valid");

        assert_eq!(
            graph.operations().copied().collect::<Vec<_>>(),
            vec![first, second]
        );
    }

    #[test]
    fn duplicate_operation_is_rejected() {
        let first = operation(1);

        let mut graph = DependencyGraph::new();
        graph
            .add_operation(first)
            .expect("first operation should be accepted");

        let result = graph.add_operation(first);

        assert!(matches!(
            result,
            Err(DependencyGraphError::DuplicateOperation { .. })
        ));

        assert_eq!(graph.operation_count(), 1);
    }

    #[test]
    fn dependency_requires_registered_endpoints() {
        let first = operation(1);
        let second = operation(2);

        let mut graph = DependencyGraph::new();
        graph
            .add_operation(first)
            .expect("operation should be accepted");

        let edge = dependency(
            1,
            first,
            second,
            DependencyKind::QuantumData,
        );

        let result = graph.add_dependency(edge);

        assert!(matches!(
            result,
            Err(DependencyGraphError::UnknownOperation {
                operation,
                ..
            }) if operation == second
        ));

        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn dependency_is_indexed_in_both_directions() {
        let first = operation(1);
        let second = operation(2);

        let mut graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        let edge = dependency(
            1,
            first,
            second,
            DependencyKind::QuantumData,
        );

        graph
            .add_dependency(edge)
            .expect("dependency should be accepted");

        assert_eq!(
            graph.predecessors(second).expect("valid operation"),
            vec![first]
        );

        assert_eq!(
            graph.successors(first).expect("valid operation"),
            vec![second]
        );

        assert_eq!(
            graph.incoming_dependencies(second)
                .expect("valid operation"),
            vec![DependencyId::new(1)]
        );

        assert_eq!(
            graph.outgoing_dependencies(first)
                .expect("valid operation"),
            vec![DependencyId::new(1)]
        );

        graph.validate().expect("graph invariants must hold");
    }

    #[test]
    fn multiple_dependency_edges_between_same_operations_are_supported() {
        let first = operation(1);
        let second = operation(2);

        let mut graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        graph
            .add_dependencies([
                dependency(
                    1,
                    first,
                    second,
                    DependencyKind::QuantumData,
                ),
                dependency(
                    2,
                    first,
                    second,
                    DependencyKind::Explicit,
                ),
            ])
            .expect("distinct dependency identities should be accepted");

        assert_eq!(graph.dependency_count(), 2);

        // The operation-level predecessor remains unique.
        assert_eq!(
            graph.predecessors(second).expect("valid operation"),
            vec![first]
        );

        graph.validate().expect("graph invariants must hold");
    }

    #[test]
    fn topological_order_is_deterministic() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);
        let fourth = operation(4);

        let mut graph =
            DependencyGraph::from_operations([
                fourth,
                second,
                third,
                first,
            ])
            .expect("operations should be valid");

        graph
            .add_dependencies([
                dependency(
                    1,
                    first,
                    third,
                    DependencyKind::QuantumData,
                ),
                dependency(
                    2,
                    second,
                    third,
                    DependencyKind::QuantumData,
                ),
                dependency(
                    3,
                    third,
                    fourth,
                    DependencyKind::QuantumData,
                ),
            ])
            .expect("dependencies should be valid");

        assert_eq!(
            graph
                .topological_order()
                .expect("graph is acyclic"),
            vec![first, second, third, fourth]
        );
    }

    #[test]
    fn roots_and_leaves_are_correct() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        let mut graph =
            DependencyGraph::from_operations([first, second, third])
                .expect("operations should be valid");

        graph
            .add_dependency(dependency(
                1,
                first,
                second,
                DependencyKind::QuantumData,
            ))
            .expect("dependency should be valid");

        assert_eq!(graph.roots(), vec![first, third]);
        assert_eq!(graph.leaves(), vec![second, third]);
    }

    #[test]
    fn topological_levels_capture_dependency_parallelism() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);
        let fourth = operation(4);

        let mut graph =
            DependencyGraph::from_operations([
                first,
                second,
                third,
                fourth,
            ])
            .expect("operations should be valid");

        graph
            .add_dependencies([
                dependency(
                    1,
                    first,
                    third,
                    DependencyKind::QuantumData,
                ),
                dependency(
                    2,
                    second,
                    third,
                    DependencyKind::QuantumData,
                ),
                dependency(
                    3,
                    third,
                    fourth,
                    DependencyKind::QuantumData,
                ),
            ])
            .expect("dependencies should be valid");

        let levels = graph
            .topological_levels()
            .expect("graph is acyclic");

        assert_eq!(
            levels,
            vec![
                vec![first, second],
                vec![third],
                vec![fourth],
            ]
        );
    }

    #[test]
    fn cycle_is_detected() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        let mut graph =
            DependencyGraph::from_operations([first, second, third])
                .expect("operations should be valid");

        graph
            .add_dependencies([
                dependency(
                    1,
                    first,
                    second,
                    DependencyKind::Explicit,
                ),
                dependency(
                    2,
                    second,
                    third,
                    DependencyKind::Explicit,
                ),
                dependency(
                    3,
                    third,
                    first,
                    DependencyKind::Explicit,
                ),
            ])
            .expect("individual edges may form a cycle");

        assert!(!graph.is_acyclic());

        let cycle = graph.find_cycle().expect("cycle must be found");

        assert!(cycle.len() >= 4);
        assert_eq!(
            cycle.first(),
            cycle.last()
        );

        assert!(matches!(
            graph.topological_order(),
            Err(DependencyGraphError::CycleDetected { .. })
        ));
    }

    #[test]
    fn removing_dependency_preserves_operations() {
        let first = operation(1);
        let second = operation(2);

        let mut graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        graph
            .add_dependency(dependency(
                1,
                first,
                second,
                DependencyKind::Explicit,
            ))
            .expect("dependency should be valid");

        assert!(graph.remove_dependency(DependencyId::new(1)));
        assert!(!graph.contains_dependency(DependencyId::new(1)));

        assert_eq!(graph.operation_count(), 2);
        assert_eq!(graph.dependency_count(), 0);

        graph.validate().expect("graph must remain valid");
    }

    #[test]
    fn removing_operation_removes_incident_edges() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        let mut graph =
            DependencyGraph::from_operations([first, second, third])
                .expect("operations should be valid");

        graph
            .add_dependencies([
                dependency(
                    1,
                    first,
                    second,
                    DependencyKind::Explicit,
                ),
                dependency(
                    2,
                    second,
                    third,
                    DependencyKind::Explicit,
                ),
                dependency(
                    3,
                    first,
                    third,
                    DependencyKind::Explicit,
                ),
            ])
            .expect("dependencies should be valid");

        let removed = graph
            .remove_operation(second)
            .expect("operation must exist");

        assert_eq!(removed, 2);
        assert!(!graph.contains_operation(second));
        assert_eq!(graph.dependency_count(), 1);

        assert_eq!(
            graph
                .successors(first)
                .expect("first still exists"),
            vec![third]
        );

        graph.validate().expect("graph must remain valid");
    }

    #[test]
    fn clearing_dependencies_preserves_nodes() {
        let first = operation(1);
        let second = operation(2);

        let mut graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        graph
            .add_dependency(dependency(
                1,
                first,
                second,
                DependencyKind::Explicit,
            ))
            .expect("dependency should be valid");

        graph.clear_dependencies();

        assert_eq!(graph.operation_count(), 2);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.is_acyclic());
        graph.validate().expect("graph must remain valid");
    }

    #[test]
    fn batch_dependency_insertion_is_atomic() {
        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        let mut graph =
            DependencyGraph::from_operations([first, second, third])
                .expect("operations should be valid");

        let result = graph.add_dependencies([
            dependency(
                1,
                first,
                second,
                DependencyKind::Explicit,
            ),
            dependency(
                1,
                second,
                third,
                DependencyKind::Explicit,
            ),
        ]);

        assert!(matches!(
            result,
            Err(DependencyGraphError::DuplicateDependency { .. })
        ));

        // No partial insertion.
        assert_eq!(graph.dependency_count(), 0);
        graph.validate().expect("graph must remain valid");
    }

    #[test]
    fn validation_catches_no_normal_mutation_errors() {
        let first = operation(1);
        let second = operation(2);

        let graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        graph.validate().expect("fresh graph must be valid");
    }

    #[test]
    fn dependency_removal_is_idempotent() {
        let first = operation(1);
        let second = operation(2);

        let mut graph =
            DependencyGraph::from_operations([first, second])
                .expect("operations should be valid");

        graph
            .add_dependency(dependency(
                1,
                first,
                second,
                DependencyKind::Explicit,
            ))
            .expect("dependency should be valid");

        assert!(graph.remove_dependency(DependencyId::new(1)));
        assert!(!graph.remove_dependency(DependencyId::new(1)));

        graph.validate().expect("graph must remain valid");
    }
}