//! Zamani Quantum Scheduling — Dependency Graph
//!
//! Path:
//!     src/quantum/scheduling/ir/dependency.rs
//!
//! # Purpose
//!
//! This module owns the scheduler-local directed dependency graph.
//!
//! It answers:
//!
//! > Which scheduling operations must precede which other scheduling
//! > operations, and how can those relationships be queried, validated,
//! > traversed, and diagnosed efficiently?
//!
//! The graph is deliberately independent of:
//!
//! - timing;
//! - resources;
//! - routing;
//! - hardware providers;
//! - calibration;
//! - QEC implementation;
//! - noise models;
//! - runtime execution;
//! - scheduling policies;
//! - scheduling algorithms.
//!
//! Those concerns consume this graph through stable interfaces.
//!
//! # Architectural position
//!
//! ```text
//! canonical quantum::ir
//!          │
//!          ▼
//! semantic dependency analysis
//!          │
//!          ▼
//! scheduling::ir::dependency
//!          │
//!      ┌───┼───────────────┐
//!      ▼   ▼               ▼
//! planners  critical-path  verification
//!      │   analysis         │
//!      └───────┬────────────┘
//!              ▼
//!          schedulers
//! ```
//!
//! # Canonical identity ownership
//!
//! Scheduler operation identities are represented by `OperationRef` from
//! `scheduling::types`.
//!
//! `OperationRef` wraps the canonical IR operation identity.
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This graph does not recreate either qubit identity.
//!
//! Likewise, dependency identity and dependency classification are owned by:
//!
//! ```text
//! crate::quantum::scheduling::types::DependencyId
//! crate::quantum::scheduling::types::DependencyKind
//! crate::quantum::scheduling::types::DependencyRef
//! ```
//!
//! # Dependency semantics
//!
//! This module stores dependency relationships but does not reinterpret their
//! semantic meaning.
//!
//! For example, a dependency may represent:
//!
//! - explicit program ordering;
//! - quantum data dependence;
//! - classical data dependence;
//! - measurement dependence;
//! - control dependence;
//! - resource-derived ordering;
//! - communication ordering;
//! - QEC ordering;
//! - another scheduler-supported dependency class.
//!
//! The layer creating the dependency determines its meaning.
//!
//! The graph only guarantees the structural relationship:
//!
//! ```text
//! from -> to
//! ```
//!
//! where `from` must precede `to` when the dependency is active.
//!
//! # Multiple dependencies
//!
//! Multiple dependencies between the same pair of operations are legal.
//!
//! They may represent different semantic reasons:
//!
//! ```text
//! A --dependency 1--> B
//! A --dependency 2--> B
//! ```
//!
//! Therefore adjacency is indexed by `DependencyId`, not merely by operation
//! pairs.
//!
//! # Graph invariants
//!
//! A valid graph guarantees:
//!
//! 1. Every dependency endpoint is registered.
//! 2. A dependency identity occurs at most once.
//! 3. A dependency cannot connect an operation to itself.
//! 4. Every registered operation has incoming and outgoing adjacency entries.
//! 5. Every stored dependency appears in both endpoint indexes.
//! 6. No dangling dependency indexes exist.
//! 7. A graph requiring DAG semantics can be validated independently.
//!
//! Cycles are intentionally not rejected during every insertion. Incremental
//! graph construction and diagnostic workflows may temporarily contain cycles.
//! Production planners that require a DAG must call `validate_acyclic()`.
//!
//! # Scalability
//!
//! This implementation contains no:
//!
//! - maximum operation count;
//! - maximum dependency count;
//! - maximum graph depth;
//! - maximum graph width;
//! - maximum qubit count;
//! - fixed topology;
//! - fixed resource count;
//! - fixed schedule duration.
//!
//! `usize` is used only for Rust collection sizes, indexes, and counts.
//!
//! Graph traversal is iterative rather than recursive, so a dependency chain
//! with very large depth does not consume stack space proportional to graph
//! depth.
//!
//! The implementation does not allocate a timeline proportional to execution
//! duration.
//!
//! # Determinism
//!
//! `BTreeMap` and `BTreeSet` are deliberately used for semantic graph storage.
//!
//! This guarantees deterministic:
//!
//! - node iteration;
//! - dependency iteration;
//! - predecessor iteration;
//! - successor iteration;
//! - topological ordering;
//! - cycle reporting.
//!
//! No result depends on randomized hash-map iteration.
//!
//! # Complexity
//!
//! Let:
//!
//! - `V` = number of operation nodes;
//! - `E` = number of dependency records.
//!
//! Node/edge collection operations are logarithmic in their respective
//! collection sizes.
//!
//! Topological ordering is:
//!
//! ```text
//! O((V + E) log V)
//! ```
//!
//! because the ready set is deterministic and ordered.
//!
//! Structural graph validation is:
//!
//! ```text
//! O((V + E) log V)
//! ```
//!
//! with deterministic collections.
//!
//! Memory consumption is:
//!
//! ```text
//! O(V + E)
//! ```
//!
//! and is independent of physical execution time.
//!
//! # Concurrency
//!
//! The graph contains no global mutable state and no interior mutability.
//!
//! Mutation requires `&mut self`.
//!
//! Once construction is complete, callers may place the graph behind `Arc` and
//! share it among read-only analyses.
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
//! - no `unsafe` code.
//!
//! The no-unsafe requirement is compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::types
//!      │
//!      ▼
//! scheduling::ir::dependency
//! ```
//!
//! Downstream:
//!
//! ```text
//! scheduling::ir::dependency
//!      ├── planners
//!      ├── algorithms
//!      ├── critical-path analysis
//!      ├── verification
//!      ├── optimization
//!      └── diagnostics
//! ```
//!
//! This file must not import hardware, routing, QEC, runtime, provider SDKs,
//! timing implementations, or resource calendars.
//!
//! # Safety
//!
//! No unsafe code is used.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::super::errors::{SchedulingError, SchedulingResult};
use super::super::types::{
    DependencyId,
    DependencyRef,
    OperationRef,
};

// =============================================================================
// Statistics
// =============================================================================

/// Immutable statistics describing a dependency graph.
///
/// These values are observational metadata only. They are never resource
/// limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyGraphStats {
    /// Number of registered operation nodes.
    pub nodes: usize,

    /// Number of dependency records.
    pub dependencies: usize,

    /// Number of root nodes.
    ///
    /// A root has no incoming dependencies.
    pub roots: usize,

    /// Number of leaf nodes.
    ///
    /// A leaf has no outgoing dependencies.
    pub leaves: usize,

    /// Maximum incoming degree observed in the graph.
    pub maximum_in_degree: usize,

    /// Maximum outgoing degree observed in the graph.
    pub maximum_out_degree: usize,
}

impl DependencyGraphStats {
    /// Returns `true` when no operation nodes are registered.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.nodes == 0
    }

    /// Returns `true` when at least one dependency exists.
    #[must_use]
    pub const fn has_dependencies(self) -> bool {
        self.dependencies != 0
    }

    /// Returns `true` when the graph consists entirely of independent nodes.
    #[must_use]
    pub const fn is_dependency_free(self) -> bool {
        self.dependencies == 0
    }
}

// =============================================================================
// Cycle
// =============================================================================

/// A deterministically reconstructed directed dependency cycle.
///
/// The path is closed:
///
/// ```text
/// A -> B -> C -> A
/// ```
///
/// Consequently the first and last entries are equal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCycle {
    operations: Vec<OperationRef>,
}

impl DependencyCycle {
    fn new(operations: Vec<OperationRef>) -> Option<Self> {
        if operations.len() < 2 {
            return None;
        }

        if operations.first() != operations.last() {
            return None;
        }

        Some(Self { operations })
    }

    /// Returns the closed cycle path.
    #[must_use]
    pub fn operations(&self) -> &[OperationRef] {
        &self.operations
    }

    /// Returns the number of distinct operation nodes in the cycle.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.operations.len().saturating_sub(1)
    }

    /// Returns whether the cycle contains an operation.
    #[must_use]
    pub fn contains(&self, operation: OperationRef) -> bool {
        self.operations
            .iter()
            .any(|candidate| *candidate == operation)
    }

    /// Returns the first operation in the cycle.
    #[must_use]
    pub fn first(&self) -> Option<OperationRef> {
        self.operations.first().copied()
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
// Graph
// =============================================================================

/// Deterministic scheduler dependency graph.
///
/// The graph stores operation nodes and dependency records separately so that
/// dependency provenance is never lost when multiple relationships exist
/// between the same pair of operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraph {
    /// Registered operation nodes.
    nodes: BTreeSet<OperationRef>,

    /// Dependency records keyed by stable dependency identity.
    dependencies: BTreeMap<DependencyId, DependencyRef>,

    /// Outgoing dependency IDs indexed by source operation.
    outgoing: BTreeMap<OperationRef, BTreeSet<DependencyId>>,

    /// Incoming dependency IDs indexed by destination operation.
    incoming: BTreeMap<OperationRef, BTreeSet<DependencyId>>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    // =========================================================================
    // Construction
    // =========================================================================

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

    /// Creates an empty graph while accepting allocation hints.
    ///
    /// `BTreeMap` and `BTreeSet` do not expose a stable capacity-reservation
    /// contract on the supported Rust versions. The hints are therefore
    /// intentionally non-semantic and currently ignored.
    ///
    /// They exist so callers can express an allocation expectation without
    /// turning it into a machine-size limit or changing the public constructor
    /// when a future storage implementation can make use of it.
    #[must_use]
    pub fn with_capacity_hint(
        node_capacity: usize,
        dependency_capacity: usize,
    ) -> Self {
        let graph = Self::new();

        let _ = node_capacity;
        let _ = dependency_capacity;

        graph
    }

    // =========================================================================
    // Basic queries
    // =========================================================================

    /// Returns the number of registered operation nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependency records.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the graph contains no operation nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether an operation reference is registered.
    #[must_use]
    pub fn contains_operation(&self, operation: OperationRef) -> bool {
        self.nodes.contains(&operation)
    }

    /// Returns whether an operation identity is registered.
    #[must_use]
    pub fn contains_operation_id(&self, operation: OperationId) -> bool {
        self.nodes
            .iter()
            .any(|reference| reference.id() == operation)
    }

    /// Returns whether a dependency identity is registered.
    #[must_use]
    pub fn contains_dependency(&self, dependency: DependencyId) -> bool {
        self.dependencies.contains_key(&dependency)
    }

    /// Returns all registered operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> impl Iterator<Item = OperationRef> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns all dependency records in deterministic identity order.
    #[must_use]
    pub fn dependencies(&self) -> impl Iterator<Item = &DependencyRef> + '_ {
        self.dependencies.values()
    }

    /// Returns the dependency record for an identity.
    #[must_use]
    pub fn dependency(
        &self,
        dependency: DependencyId,
    ) -> Option<&DependencyRef> {
        self.dependencies.get(&dependency)
    }

    // =========================================================================
    // Node mutation
    // =========================================================================

    /// Registers one operation node.
    ///
    /// Registering the exact same `OperationRef` is idempotent and returns
    /// `Ok(false)`.
    ///
    /// Registering a different scheduler reference for an already registered
    /// canonical operation identity is rejected.
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
                    "operation identity `{}` is already registered with a \
                     different scheduler reference",
                    operation.id()
                ),
            });
        }

        self.nodes.insert(operation);
        self.outgoing.entry(operation).or_default();
        self.incoming.entry(operation).or_default();

        Ok(true)
    }

    /// Registers multiple operation nodes.
    ///
    /// Input ordering does not affect internal ordering.
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
                            "operation count overflowed the host collection \
                             size",
                        ),
                    })?;
            }
        }

        Ok(added)
    }

    /// Removes an operation and all dependency records incident to it.
    ///
    /// Returns `true` when the operation existed.
    ///
    /// Removal is deterministic and maintains all graph indexes.
    pub fn remove_operation(
        &mut self,
        operation: OperationRef,
    ) -> SchedulingResult<bool> {
        if !self.nodes.remove(&operation) {
            return Ok(false);
        }

        let outgoing_ids = self
            .outgoing
            .remove(&operation)
            .unwrap_or_default();

        let incoming_ids = self
            .incoming
            .remove(&operation)
            .unwrap_or_default();

        for dependency_id in outgoing_ids
            .into_iter()
            .chain(incoming_ids.into_iter())
        {
            self.remove_dependency_internal(dependency_id);
        }

        Ok(true)
    }

    /// Removes all graph contents.
    ///
    /// Returns the number of removed operation nodes.
    pub fn clear(&mut self) -> usize {
        let count = self.nodes.len();

        self.nodes.clear();
        self.dependencies.clear();
        self.outgoing.clear();
        self.incoming.clear();

        count
    }

    // =========================================================================
    // Dependency mutation
    // =========================================================================

    /// Adds one dependency.
    ///
    /// Both endpoints must already be registered.
    ///
    /// A dependency ID may occur only once.
    ///
    /// Distinct dependency IDs may connect the same pair of operations.
    pub fn add_dependency(
        &mut self,
        dependency: DependencyRef,
    ) -> SchedulingResult<bool> {
        let dependency_id = dependency.id();
        let predecessor = dependency.from();
        let successor = dependency.to();

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
                    "dependency source `{predecessor}` is not registered"
                ),
            });
        }

        if !self.nodes.contains(&successor) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: format!(
                    "dependency destination `{successor}` is not registered"
                ),
            });
        }

        if self.dependencies.contains_key(&dependency_id) {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency_id),
                predecessor: Some(predecessor.id()),
                successor: Some(successor.id()),
                reason: format!(
                    "dependency identity `{dependency_id}` is already \
                     registered"
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

    /// Adds multiple dependency records.
    ///
    /// If insertion fails, the graph is restored to its state immediately
    /// before the call.
    ///
    /// This transactional behavior prevents partially constructed dependency
    /// sets from escaping after invalid input.
    pub fn add_dependencies<I>(
        &mut self,
        dependencies: I,
    ) -> SchedulingResult<usize>
    where
        I: IntoIterator<Item = DependencyRef>,
    {
        let mut inserted = Vec::new();

        for dependency in dependencies {
            match self.add_dependency(dependency) {
                Ok(true) => inserted.push(dependency.id()),
                Ok(false) => {}
                Err(error) => {
                    for dependency_id in inserted {
                        self.remove_dependency_internal(dependency_id);
                    }

                    return Err(error);
                }
            }
        }

        Ok(inserted.len())
    }

    /// Removes one dependency.
    ///
    /// Returns the removed dependency when it existed.
    pub fn remove_dependency(
        &mut self,
        dependency: DependencyId,
    ) -> Option<DependencyRef> {
        self.remove_dependency_internal(dependency)
    }

    fn remove_dependency_internal(
        &mut self,
        dependency: DependencyId,
    ) -> Option<DependencyRef> {
        let record = self.dependencies.remove(&dependency)?;

        let predecessor = record.from();
        let successor = record.to();

        if let Some(ids) = self.outgoing.get_mut(&predecessor) {
            ids.remove(&dependency);
        }

        if let Some(ids) = self.incoming.get_mut(&successor) {
            ids.remove(&dependency);
        }

        Some(record)
    }

    // =========================================================================
    // Adjacency
    // =========================================================================

    /// Returns dependency IDs leaving an operation in deterministic order.
    #[must_use]
    pub fn outgoing_dependencies(
        &self,
        operation: OperationRef,
    ) -> impl Iterator<Item = DependencyId> + '_ {
        self.outgoing
            .get(&operation)
            .into_iter()
            .flat_map(|ids| ids.iter().copied())
    }

    /// Returns dependency IDs entering an operation in deterministic order.
    #[must_use]
    pub fn incoming_dependencies(
        &self,
        operation: OperationRef,
    ) -> impl Iterator<Item = DependencyId> + '_ {
        self.incoming
            .get(&operation)
            .into_iter()
            .flat_map(|ids| ids.iter().copied())
    }

    /// Returns direct successors of an operation.
    ///
    /// Duplicate dependency reasons between the same pair are collapsed in
    /// this operation-level view.
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

    /// Returns direct predecessors of an operation.
    ///
    /// Duplicate dependency reasons between the same pair are collapsed in
    /// this operation-level view.
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

    /// Returns the number of direct predecessors.
    #[must_use]
    pub fn indegree(&self, operation: OperationRef) -> usize {
        self.incoming
            .get(&operation)
            .map_or(0, BTreeSet::len)
    }

    /// Returns the number of direct successors.
    #[must_use]
    pub fn outdegree(&self, operation: OperationRef) -> usize {
        self.outgoing
            .get(&operation)
            .map_or(0, BTreeSet::len)
    }

    /// Returns whether an operation has no predecessors.
    #[must_use]
    pub fn is_root(&self, operation: OperationRef) -> bool {
        self.indegree(operation) == 0
    }

    /// Returns whether an operation has no successors.
    #[must_use]
    pub fn is_leaf(&self, operation: OperationRef) -> bool {
        self.outdegree(operation) == 0
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Computes deterministic graph statistics.
    #[must_use]
    pub fn statistics(&self) -> DependencyGraphStats {
        let mut roots = 0usize;
        let mut leaves = 0usize;
        let mut maximum_in_degree = 0usize;
        let mut maximum_out_degree = 0usize;

        for operation in &self.nodes {
            let incoming = self.indegree(*operation);
            let outgoing = self.outdegree(*operation);

            if incoming == 0 {
                roots += 1;
            }

            if outgoing == 0 {
                leaves += 1;
            }

            maximum_in_degree = maximum_in_degree.max(incoming);
            maximum_out_degree = maximum_out_degree.max(outgoing);
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

    // =========================================================================
    // Structural validation
    // =========================================================================

    /// Validates the graph's structural invariants.
    ///
    /// This does not require the graph to be acyclic.
    ///
    /// Use `validate_acyclic()` when DAG semantics are required.
    pub fn validate_structure(&self) -> SchedulingResult<()> {
        // Every node must have both adjacency entries.
        for operation in &self.nodes {
            if !self.outgoing.contains_key(operation) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: Some(operation.id()),
                    successor: None,
                    reason: String::from(
                        "registered operation has no outgoing adjacency entry",
                    ),
                });
            }

            if !self.incoming.contains_key(operation) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: Some(operation.id()),
                    successor: None,
                    reason: String::from(
                        "registered operation has no incoming adjacency entry",
                    ),
                });
            }
        }

        // Every outgoing index must refer to an existing dependency and the
        // dependency must actually originate at the indexed operation.
        for (operation, dependency_ids) in &self.outgoing {
            if !self.nodes.contains(operation) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: Some(operation.id()),
                    successor: None,
                    reason: String::from(
                        "outgoing adjacency references an unregistered \
                         operation",
                    ),
                });
            }

            for dependency_id in dependency_ids {
                let dependency = self
                    .dependencies
                    .get(dependency_id)
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: Some(operation.id()),
                            successor: None,
                            reason: String::from(
                                "outgoing adjacency references a missing \
                                 dependency record",
                            ),
                        }
                    })?;

                if dependency.from() != *operation {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(operation.id()),
                        successor: Some(dependency.to().id()),
                        reason: String::from(
                            "outgoing adjacency and dependency source \
                             disagree",
                        ),
                    });
                }
            }
        }

        // Every incoming index must refer to an existing dependency and the
        // dependency must actually terminate at the indexed operation.
        for (operation, dependency_ids) in &self.incoming {
            if !self.nodes.contains(operation) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "incoming adjacency references an unregistered \
                         operation",
                    ),
                });
            }

            for dependency_id in dependency_ids {
                let dependency = self
                    .dependencies
                    .get(dependency_id)
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: Some(*dependency_id),
                            predecessor: None,
                            successor: Some(operation.id()),
                            reason: String::from(
                                "incoming adjacency references a missing \
                                 dependency record",
                            ),
                        }
                    })?;

                if dependency.to() != *operation {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: Some(*dependency_id),
                        predecessor: Some(dependency.from().id()),
                        successor: Some(operation.id()),
                        reason: String::from(
                            "incoming adjacency and dependency destination \
                             disagree",
                        ),
                    });
                }
            }
        }

        // Every dependency must have registered endpoints and must be present
        // in both adjacency indexes.
        for (dependency_id, dependency) in &self.dependencies {
            let predecessor = dependency.from();
            let successor = dependency.to();

            if predecessor == successor {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: Some(*dependency_id),
                    predecessor: Some(predecessor.id()),
                    successor: Some(successor.id()),
                    reason: String::from(
                        "self-dependency detected",
                    ),
                });
            }

            if !self.nodes.contains(&predecessor) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: Some(*dependency_id),
                    predecessor: Some(predecessor.id()),
                    successor: Some(successor.id()),
                    reason: String::from(
                        "dependency source is not registered",
                    ),
                });
            }

            if !self.nodes.contains(&successor) {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: Some(*dependency_id),
                    predecessor: Some(predecessor.id()),
                    successor: Some(successor.id()),
                    reason: String::from(
                        "dependency destination is not registered",
                    ),
                });
            }

            let outgoing_contains = self
                .outgoing
                .get(&predecessor)
                .is_some_and(|ids| ids.contains(dependency_id));

            if !outgoing_contains {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: Some(*dependency_id),
                    predecessor: Some(predecessor.id()),
                    successor: Some(successor.id()),
                    reason: String::from(
                        "dependency is missing from outgoing adjacency",
                    ),
                });
            }

            let incoming_contains = self
                .incoming
                .get(&successor)
                .is_some_and(|ids| ids.contains(dependency_id));

            if !incoming_contains {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: Some(*dependency_id),
                    predecessor: Some(predecessor.id()),
                    successor: Some(successor.id()),
                    reason: String::from(
                        "dependency is missing from incoming adjacency",
                    ),
                });
            }
        }

        Ok(())
    }

    // =========================================================================
    // Topological ordering
    // =========================================================================

    /// Returns a deterministic topological ordering.
    ///
    /// The operation with the smallest scheduler ordering is selected whenever
    /// multiple operations are simultaneously ready.
    ///
    /// This method is iterative and therefore safe for extremely deep DAGs.
    pub fn topological_order(&self) -> SchedulingResult<Vec<OperationRef>> {
        self.validate_structure()?;

        let mut indegrees = BTreeMap::<OperationRef, usize>::new();

        for operation in &self.nodes {
            indegrees.insert(*operation, self.indegree(*operation));
        }

        let mut ready = BTreeSet::<OperationRef>::new();

        for (operation, indegree) in &indegrees {
            if *indegree == 0 {
                ready.insert(*operation);
            }
        }

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(operation) = ready.pop_first() {
            order.push(operation);

            if let Some(dependencies) = self.outgoing.get(&operation) {
                for dependency_id in dependencies {
                    let dependency = match self.dependencies.get(dependency_id)
                    {
                        Some(value) => value,
                        None => {
                            return Err(
                                SchedulingError::InvalidDependencyGraph {
                                    dependency: Some(*dependency_id),
                                    predecessor: Some(operation.id()),
                                    successor: None,
                                    reason: String::from(
                                        "outgoing adjacency references a \
                                         missing dependency",
                                    ),
                                },
                            );
                        }
                    };

                    let successor = dependency.to();

                    let indegree = match indegrees.get_mut(&successor) {
                        Some(value) => value,
                        None => {
                            return Err(
                                SchedulingError::InvalidDependencyGraph {
                                    dependency: Some(*dependency_id),
                                    predecessor: Some(operation.id()),
                                    successor: Some(successor.id()),
                                    reason: String::from(
                                        "dependency destination is not \
                                         registered in indegree state",
                                    ),
                                },
                            );
                        }
                    };

                    if *indegree == 0 {
                        return Err(
                            SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "indegree underflow indicates \
                                     inconsistent graph adjacency",
                                ),
                            },
                        );
                    }

                    *indegree -= 1;

                    if *indegree == 0 {
                        ready.insert(successor);
                    }
                }
            }
        }

        if order.len() == self.nodes.len() {
            return Ok(order);
        }

        let cycle = self
            .find_cycle()
            .ok_or_else(|| SchedulingError::CycleDetected {
                operation: None,
                dependency: None,
                cycle_size: None,
            })?;

        let cycle_size = u128::try_from(cycle.node_count()).ok();

        let first_operation = cycle.first().map(OperationRef::id);

        let dependency = self
            .dependency_between_cycle_edges(&cycle)
            .map(|value| value.id());

        Err(SchedulingError::CycleDetected {
            operation: first_operation,
            dependency,
            cycle_size,
        })
    }

    /// Validates that the graph is acyclic.
    pub fn validate_acyclic(&self) -> SchedulingResult<()> {
        let _ = self.topological_order()?;
        Ok(())
    }

    /// Returns all root operations in deterministic order.
    #[must_use]
    pub fn roots(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .copied()
            .filter(|operation| self.indegree(*operation) == 0)
            .collect()
    }

    /// Returns all leaf operations in deterministic order.
    #[must_use]
    pub fn leaves(&self) -> Vec<OperationRef> {
        self.nodes
            .iter()
            .copied()
            .filter(|operation| self.outdegree(*operation) == 0)
            .collect()
    }

    // =========================================================================
    // Cycle detection
    // =========================================================================

    /// Finds one deterministic directed cycle, if one exists.
    ///
    /// The traversal is iterative.
    #[must_use]
    pub fn find_cycle(&self) -> Option<DependencyCycle> {
        if self.nodes.is_empty() {
            return None;
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum VisitState {
            Visiting,
            Visited,
        }

        let mut state = BTreeMap::<OperationRef, VisitState>::new();
        let mut parent = BTreeMap::<OperationRef, OperationRef>::new();

        for start in &self.nodes {
            if state.contains_key(start) {
                continue;
            }

            state.insert(*start, VisitState::Visiting);

            let mut stack = Vec::<(OperationRef, Vec<OperationRef>, usize)>::new();

            stack.push((
                *start,
                self.successors(*start),
                0,
            ));

            while let Some((operation, successors, next_index)) =
                stack.last_mut()
            {
                if *next_index >= successors.len() {
                    state.insert(*operation, VisitState::Visited);
                    stack.pop();
                    continue;
                }

                let successor = successors[*next_index];
                *next_index += 1;

                match state.get(&successor).copied() {
                    None => {
                        state.insert(successor, VisitState::Visiting);
                        parent.insert(successor, *operation);

                        stack.push((
                            successor,
                            self.successors(successor),
                            0,
                        ));
                    }

                    Some(VisitState::Visited) => {}

                    Some(VisitState::Visiting) => {
                        let cycle =
                            Self::reconstruct_cycle(*operation, successor, &parent);

                        if let Some(cycle) = cycle {
                            return Some(cycle);
                        }

                        return None;
                    }
                }
            }
        }

        None
    }

    fn reconstruct_cycle(
        current: OperationRef,
        ancestor: OperationRef,
        parent: &BTreeMap<OperationRef, OperationRef>,
    ) -> Option<DependencyCycle> {
        let mut reverse_path = Vec::<OperationRef>::new();
        reverse_path.push(current);

        let mut cursor = current;

        while cursor != ancestor {
            cursor = *parent.get(&cursor)?;
            reverse_path.push(cursor);
        }

        reverse_path.reverse();
        reverse_path.push(ancestor);

        DependencyCycle::new(reverse_path)
    }

    fn dependency_between_cycle_edges(
        &self,
        cycle: &DependencyCycle,
    ) -> Option<&DependencyRef> {
        let operations = cycle.operations();

        if operations.len() < 2 {
            return None;
        }

        for pair in operations.windows(2) {
            let from = pair[0];
            let to = pair[1];

            if let Some(dependency_ids) = self.outgoing.get(&from) {
                for dependency_id in dependency_ids {
                    if let Some(dependency) =
                        self.dependencies.get(dependency_id)
                    {
                        if dependency.to() == to {
                            return Some(dependency);
                        }
                    }
                }
            }
        }

        None
    }

    // =========================================================================
    // Reachability
    // =========================================================================

    /// Returns whether `from` can reach `to`.
    ///
    /// This is an iterative depth-first traversal.
    #[must_use]
    pub fn can_reach(
        &self,
        from: OperationRef,
        to: OperationRef,
    ) -> bool {
        if from == to {
            return self.nodes.contains(&from);
        }

        if !self.nodes.contains(&from) || !self.nodes.contains(&to) {
            return false;
        }

        let mut visited = BTreeSet::<OperationRef>::new();
        let mut stack = vec![from];

        while let Some(operation) = stack.pop() {
            if !visited.insert(operation) {
                continue;
            }

            for successor in self.successors(operation) {
                if successor == to {
                    return true;
                }

                if !visited.contains(&successor) {
                    stack.push(successor);
                }
            }
        }

        false
    }

    /// Returns all operations reachable from `from`.
    ///
    /// The returned vector is deterministic.
    #[must_use]
    pub fn reachable_from(
        &self,
        from: OperationRef,
    ) -> Vec<OperationRef> {
        if !self.nodes.contains(&from) {
            return Vec::new();
        }

        let mut visited = BTreeSet::<OperationRef>::new();
        let mut stack = vec![from];

        while let Some(operation) = stack.pop() {
            if !visited.insert(operation) {
                continue;
            }

            for successor in self.successors(operation).into_iter().rev() {
                if !visited.contains(&successor) {
                    stack.push(successor);
                }
            }
        }

        visited.into_iter().collect()
    }

    /// Returns all operations that can reach `to`.
    ///
    /// The returned vector is deterministic.
    #[must_use]
    pub fn ancestors_of(
        &self,
        to: OperationRef,
    ) -> Vec<OperationRef> {
        if !self.nodes.contains(&to) {
            return Vec::new();
        }

        let mut visited = BTreeSet::<OperationRef>::new();
        let mut stack = vec![to];

        while let Some(operation) = stack.pop() {
            if !visited.insert(operation) {
                continue;
            }

            for predecessor in self.predecessors(operation).into_iter().rev() {
                if !visited.contains(&predecessor) {
                    stack.push(predecessor);
                }
            }
        }

        visited.into_iter().collect()
    }

    // =========================================================================
    // Dependency queries
    // =========================================================================

    /// Returns dependency records connecting `from` directly to `to`.
    ///
    /// All distinct dependency identities are preserved.
    #[must_use]
    pub fn dependencies_between(
        &self,
        from: OperationRef,
        to: OperationRef,
    ) -> Vec<&DependencyRef> {
        let mut result = Vec::new();

        if let Some(dependency_ids) = self.outgoing.get(&from) {
            for dependency_id in dependency_ids {
                if let Some(dependency) =
                    self.dependencies.get(dependency_id)
                {
                    if dependency.to() == to {
                        result.push(dependency);
                    }
                }
            }
        }

        result
    }

    /// Returns whether at least one dependency directly connects two
    /// operations.
    #[must_use]
    pub fn has_dependency(
        &self,
        from: OperationRef,
        to: OperationRef,
    ) -> bool {
        self.outgoing
            .get(&from)
            .is_some_and(|dependency_ids| {
                dependency_ids.iter().any(|dependency_id| {
                    self.dependencies
                        .get(dependency_id)
                        .is_some_and(|dependency| dependency.to() == to)
                })
            })
    }

    // =========================================================================
    // Snapshot
    // =========================================================================

    /// Returns a read-only deterministic snapshot of the graph.
    ///
    /// The snapshot owns its collections and can therefore be moved into
    /// another analysis task without borrowing the original graph.
    #[must_use]
    pub fn snapshot(&self) -> DependencyGraphSnapshot {
        DependencyGraphSnapshot {
            nodes: self.nodes.clone(),
            dependencies: self.dependencies.clone(),
            outgoing: self.outgoing.clone(),
            incoming: self.incoming.clone(),
        }
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Immutable owned dependency-graph snapshot.
///
/// This type is useful when several read-only analyses need an independent
/// graph view without retaining a borrow of a mutable construction graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraphSnapshot {
    nodes: BTreeSet<OperationRef>,
    dependencies: BTreeMap<DependencyId, DependencyRef>,
    outgoing: BTreeMap<OperationRef, BTreeSet<DependencyId>>,
    incoming: BTreeMap<OperationRef, BTreeSet<DependencyId>>,
}

impl DependencyGraphSnapshot {
    /// Returns the number of operation nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns all operations in deterministic order.
    #[must_use]
    pub fn operations(&self) -> impl Iterator<Item = OperationRef> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns all dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> impl Iterator<Item = &DependencyRef> + '_ {
        self.dependencies.values()
    }

    /// Returns direct successors.
    #[must_use]
    pub fn successors(
        &self,
        operation: OperationRef,
    ) -> Vec<OperationRef> {
        let mut result = BTreeSet::new();

        if let Some(ids) = self.outgoing.get(&operation) {
            for dependency_id in ids {
                if let Some(dependency) =
                    self.dependencies.get(dependency_id)
                {
                    result.insert(dependency.to());
                }
            }
        }

        result.into_iter().collect()
    }

    /// Returns direct predecessors.
    #[must_use]
    pub fn predecessors(
        &self,
        operation: OperationRef,
    ) -> Vec<OperationRef> {
        let mut result = BTreeSet::new();

        if let Some(ids) = self.incoming.get(&operation) {
            for dependency_id in ids {
                if let Some(dependency) =
                    self.dependencies.get(dependency_id)
                {
                    result.insert(dependency.from());
                }
            }
        }

        result.into_iter().collect()
    }

    /// Returns deterministic topological order.
    pub fn topological_order(&self) -> SchedulingResult<Vec<OperationRef>> {
        let mut indegrees = BTreeMap::<OperationRef, usize>::new();

        for operation in &self.nodes {
            let degree = self
                .incoming
                .get(operation)
                .map_or(0, BTreeSet::len);

            indegrees.insert(*operation, degree);
        }

        let mut ready = BTreeSet::<OperationRef>::new();

        for (operation, degree) in &indegrees {
            if *degree == 0 {
                ready.insert(*operation);
            }
        }

        let mut result = Vec::with_capacity(self.nodes.len());

        while let Some(operation) = ready.pop_first() {
            result.push(operation);

            if let Some(ids) = self.outgoing.get(&operation) {
                for dependency_id in ids {
                    let dependency =
                        self.dependencies.get(dependency_id).ok_or_else(
                            || SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: None,
                                reason: String::from(
                                    "snapshot references a missing \
                                     dependency",
                                ),
                            },
                        )?;

                    let successor = dependency.to();

                    let degree =
                        indegrees.get_mut(&successor).ok_or_else(|| {
                            SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "snapshot dependency destination is not \
                                     registered",
                                ),
                            }
                        })?;

                    if *degree == 0 {
                        return Err(
                            SchedulingError::InvalidDependencyGraph {
                                dependency: Some(*dependency_id),
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "snapshot indegree underflow",
                                ),
                            },
                        );
                    }

                    *degree -= 1;

                    if *degree == 0 {
                        ready.insert(successor);
                    }
                }
            }
        }

        if result.len() == self.nodes.len() {
            return Ok(result);
        }

        Err(SchedulingError::CycleDetected {
            operation: None,
            dependency: None,
            cycle_size: None,
        })
    }

    /// Validates DAG semantics.
    pub fn validate_acyclic(&self) -> SchedulingResult<()> {
        let _ = self.topological_order()?;
        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::OperationId;

    fn operation(value: u64) -> OperationRef {
        OperationRef::new(OperationId::from(value))
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
            crate::quantum::scheduling::types::DependencyKind::Explicit,
        )
    }

    #[test]
    fn empty_graph_is_valid_and_acyclic() {
        let graph = DependencyGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate_structure().is_ok());
        assert!(graph.validate_acyclic().is_ok());
        assert!(graph.topological_order().is_ok());
    }

    #[test]
    fn operation_registration_is_idempotent() {
        let mut graph = DependencyGraph::new();
        let first = operation(1);

        assert_eq!(graph.add_operation(first).unwrap(), true);
        assert_eq!(graph.add_operation(first).unwrap(), false);
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn dependency_requires_registered_endpoints() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph.add_operation(first).unwrap();

        let result = graph.add_dependency(dependency(1, first, second));

        assert!(result.is_err());
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut graph = DependencyGraph::new();
        let first = operation(1);

        graph.add_operation(first).unwrap();

        let result = graph.add_dependency(dependency(1, first, first));

        assert!(result.is_err());
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn topological_order_is_deterministic() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([third, first, second])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, second, third))
            .unwrap();

        let order = graph.topological_order().unwrap();

        assert_eq!(order, vec![first, second, third]);
    }

    #[test]
    fn independent_nodes_are_ordered_deterministically() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([third, first, second])
            .unwrap();

        let order = graph.topological_order().unwrap();

        assert_eq!(order, vec![first, second, third]);
    }

    #[test]
    fn multiple_dependency_reasons_are_preserved() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph.add_operations([first, second]).unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, first, second))
            .unwrap();

        assert_eq!(graph.dependency_count(), 2);
        assert_eq!(graph.dependencies_between(first, second).len(), 2);
        assert_eq!(graph.successors(first), vec![second]);
        assert_eq!(graph.predecessors(second), vec![first]);
    }

    #[test]
    fn cycle_is_detected() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, second, third))
            .unwrap();

        graph
            .add_dependency(dependency(3, third, first))
            .unwrap();

        assert!(graph.find_cycle().is_some());
        assert!(graph.validate_acyclic().is_err());
    }

    #[test]
    fn reachability_is_iterative() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, second, third))
            .unwrap();

        assert!(graph.can_reach(first, third));
        assert!(!graph.can_reach(third, first));
    }

    #[test]
    fn remove_dependency_preserves_graph_integrity() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph.add_operations([first, second]).unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        assert_eq!(graph.dependency_count(), 1);

        let removed = graph.remove_dependency(DependencyId::new(1));

        assert!(removed.is_some());
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate_structure().is_ok());
    }

    #[test]
    fn remove_operation_removes_incident_dependencies() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, second, third))
            .unwrap();

        graph.remove_operation(second).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate_structure().is_ok());
    }

    #[test]
    fn roots_and_leaves_are_correct() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, second, third))
            .unwrap();

        assert_eq!(graph.roots(), vec![first]);
        assert_eq!(graph.leaves(), vec![third]);
    }

    #[test]
    fn snapshot_is_independent() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);

        graph.add_operations([first, second]).unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        let snapshot = graph.snapshot();

        graph.remove_dependency(DependencyId::new(1));

        assert_eq!(snapshot.dependency_count(), 1);
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn statistics_are_correct() {
        let mut graph = DependencyGraph::new();

        let first = operation(1);
        let second = operation(2);
        let third = operation(3);

        graph
            .add_operations([first, second, third])
            .unwrap();

        graph
            .add_dependency(dependency(1, first, second))
            .unwrap();

        graph
            .add_dependency(dependency(2, first, third))
            .unwrap();

        let statistics = graph.statistics();

        assert_eq!(statistics.nodes, 3);
        assert_eq!(statistics.dependencies, 2);
        assert_eq!(statistics.roots, 1);
        assert_eq!(statistics.leaves, 2);
        assert_eq!(statistics.maximum_in_degree, 1);
        assert_eq!(statistics.maximum_out_degree, 2);
    }
}