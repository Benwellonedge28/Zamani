//! Zamani Quantum IR — Dependency Analysis
//!
//! Path:
//!     src/quantum/ir/analysis/dependencies.rs
//!
//! # Purpose
//!
//! This module constructs a deterministic, sparse dependency graph for the
//! canonical Zamani Quantum IR.
//!
//! The analysis answers:
//!
//! > Which semantic IR operations must precede which other operations?
//!
//! It is a READ-ONLY analysis.
//!
//! It never:
//!
//! - mutates an operation;
//! - mutates a circuit;
//! - changes operation order;
//! - performs optimization;
//! - performs routing;
//! - performs scheduling;
//! - allocates physical qubits;
//! - selects a backend;
//! - performs calibration;
//! - synthesizes pulses;
//! - executes quantum hardware;
//! - simulates quantum state.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! frontend
//!       |
//!       v
//! canonical Quantum IR
//!       |
//!       +-------------------+
//!       |                   |
//!       v                   v
//! validation          analysis/dependencies.rs
//!                           |
//!                           v
//!                  dependency information
//!                           |
//!              +------------+------------+
//!              |            |            |
//!              v            v            v
//!          optimization  routing     scheduling
//! ```
//!
//! The dependency graph is semantic information. It is NOT a schedule.
//!
//! # Dependency categories
//!
//! The implementation recognizes:
//!
//! 1. `QubitOrder`
//!    Two operations touch the same logical qubit. Program order establishes
//!    a required semantic ordering boundary.
//!
//! 2. `ClassicalData`
//!    Two operations reference the same classical bit. The later operation
//!    must observe the classical state established by the earlier operation.
//!
//! 3. `MeasurementFeedback`
//!    A classical condition on an operation depends on an earlier operation
//!    touching the corresponding classical measurement bit.
//!
//! 4. `ProgramOrder`
//!    An explicit dependency supplied by a higher-level integration boundary.
//!
//! `ProgramOrder` is intentionally NOT generated between every pair of
//! operations. Doing that would turn an otherwise sparse graph into an
//! unnecessary O(N) chain and would incorrectly prevent independent
//! operations from being analyzed as independent.
//!
//! # Scalability
//!
//! This implementation deliberately avoids:
//!
//! ```text
//! N x N adjacency matrices
//! N x N boolean dependency tables
//! one entry per declared qubit
//! one entry per possible classical bit
//! fixed hardware-size arrays
//! fixed qubit-count assumptions
//! ```
//!
//! Instead, it tracks only resources actually referenced by operations.
//!
//! Therefore a circuit can declare a very large logical namespace while using
//! only a small subset of it without forcing this analysis to allocate
//! storage for every declared resource.
//!
//! The practical workload is:
//!
//! ```text
//! O(N log R + E log E)
//! ```
//!
//! where:
//!
//! - N = number of analyzed operations;
//! - R = number of distinct referenced semantic resources;
//! - E = number of dependency edges actually emitted.
//!
//! The exact cost depends on the number of resources touched by each
//! operation. No constant is used as a quantum-machine limit.
//!
//! # "Infinity" semantics
//!
//! Zamani does not represent an actually infinite IR program in memory.
//!
//! "Scale to infinity" means that no artificial machine-size ceiling is
//! introduced by this file. A concrete finite program is limited only by:
//!
//! - the host's available resources;
//! - explicit compiler/service resource policies;
//! - the representation's integer/addressing limits;
//! - the size of the actual input.
//!
//! # Canonical qubit identity
//!
//! All quantum dependency tracking uses:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module never imports a legacy `qubits::QubitId`.
//!
//! `QubitId` is treated as a logical semantic identity. Physical qubits are
//! deliberately outside this analysis.
//!
//! # Determinism
//!
//! Results are deterministic:
//!
//! - operations are processed in supplied program order;
//! - nodes retain program order;
//! - resource maps use `BTreeMap`;
//! - merged edges are deterministic;
//! - edge ordering is deterministic;
//! - dependency reasons are represented by a stable bitset;
//! - no `HashMap` iteration order becomes public semantic output.
//!
//! # Safety
//!
//! The implementation:
//!
//! - uses checked arithmetic where arithmetic is required;
//! - rejects duplicate operation identities;
//! - detects self-dependencies;
//! - does not use unsafe code;
//! - does not allocate based on declared qubit count;
//! - does not create recursive graph structures;
//! - does not use recursion for graph traversal.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This file consumes the following canonical contracts:
//!
//! `operation.rs`
//!     Supplies `Operation`, `OperationId`, logical qubit references,
//!     classical-bit references, and operation conditions.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId`.
//!
//! `circuit.rs` / `model/circuit.rs`
//!     Supplies ordered operation storage. A caller can pass its
//!     `operations()` slice to [`analyze_operations`].
//!
//! `validation.rs`
//!     Should be run before dependency analysis when complete IR validation
//!     is required. This module nevertheless performs the local invariants
//!     needed to protect its own graph construction.
//!
//! `optimization/`
//!     May consume this graph to determine legal transformation boundaries.
//!
//! `routing/`
//!     May consume qubit dependency information without modifying this graph.
//!
//! `scheduling/`
//!     May consume the graph as one source of precedence constraints.
//!
//! `hardware/`
//!     Is intentionally not required by this module.
//!
//! `serialization/`
//!     Must serialize dependency analysis only if dependency analysis is made
//!     part of an explicit derived artifact. The canonical IR itself does not
//!     become dependent on this analysis.
//!
//! `hashing/`
//!     Must not silently incorporate derived dependency-analysis storage into
//!     canonical semantic hashing unless the dependency graph itself becomes
//!     a declared semantic IR object.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe.
//!
//! # Ownership contract
//!
//! OWNS:
//!
//! - dependency edge semantics;
//! - dependency reason classification;
//! - sparse dependency graph construction;
//! - deterministic dependency queries;
//! - dependency-analysis errors.
//!
//! DOES NOT OWN:
//!
//! - operation semantics;
//! - qubit identity;
//! - classical-bit identity;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - optimization;
//! - execution;
//! - physical resource allocation.
//!
//! # Important design rule
//!
//! Dependency analysis must remain an analysis of the IR that exists.
//!
//! It must never invent hardware dependencies.
//!
//! For example:
//!
//! ```text
//! q0 -> H
//! q1 -> X
//! ```
//!
//! has no semantic dependency merely because some future hardware may not
//! execute H and X concurrently.
//!
//! Such a restriction belongs to hardware capabilities and scheduling.
//!
//! Conversely:
//!
//! ```text
//! measure q0 -> c0
//! if c0 { X q1 }
//! ```
//!
//! contains a real semantic dependency because the second operation depends on
//! classical information produced by the first operation.
//!
//! # Why sparse last-use tracking?
//!
//! A naïve implementation would compare every operation with every later
//! operation:
//!
//! ```text
//! for i in operations:
//!     for j in operations after i:
//!         compare resources
//! ```
//!
//! That is O(N²) even when operations touch completely independent qubits.
//!
//! This implementation instead maintains:
//!
//! ```text
//! logical resource -> last operation using it
//! ```
//!
//! When an operation touches a resource, only its immediately preceding user
//! is relevant for the minimal sequential dependency chain for that resource.
//!
//! This is both substantially more scalable and sufficient to reconstruct the
//! transitive ordering through the graph.
//!
//! # No unsafe
//!
//! This file intentionally contains:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! No unsafe escape hatch is provided.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::identity::OperationId;
use super::super::operation::Operation;
use super::super::qubit::QubitId;

// =============================================================================
// Dependency kind
// =============================================================================

/// Semantic reason why one operation depends on another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyKind {
    /// Both operations reference the same logical qubit.
    QubitOrder,

    /// Both operations reference the same classical bit.
    ClassicalData,

    /// The later operation uses a classical bit produced/affected by an
    /// earlier measurement-related operation.
    MeasurementFeedback,

    /// Explicit ordering supplied by an integration boundary.
    ///
    /// This is not automatically generated for all adjacent operations.
    ProgramOrder,
}

impl DependencyKind {
    /// Stable numeric representation used by deterministic reason sets.
    const fn bit(self) -> u8 {
        match self {
            Self::QubitOrder => 1 << 0,
            Self::ClassicalData => 1 << 1,
            Self::MeasurementFeedback => 1 << 2,
            Self::ProgramOrder => 1 << 3,
        }
    }
}

// =============================================================================
// Dependency reason set
// =============================================================================

/// Compact deterministic set of dependency reasons.
///
/// A bitset is used instead of `Vec<DependencyKind>` so merging multiple
/// reasons for the same edge remains O(1) and does not create duplicate
/// allocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DependencyReasonSet {
    bits: u8,
}

impl DependencyReasonSet {
    /// Creates an empty reason set.
    #[must_use]
    pub const fn new() -> Self {
        Self { bits: 0 }
    }

    /// Adds one dependency reason.
    pub const fn insert(&mut self, kind: DependencyKind) {
        self.bits |= kind.bit();
    }

    /// Returns whether the reason set contains `kind`.
    #[must_use]
    pub const fn contains(self, kind: DependencyKind) -> bool {
        self.bits & kind.bit() != 0
    }

    /// Returns whether no reasons are present.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bits == 0
    }

    /// Returns the number of distinct dependency reasons.
    #[must_use]
    pub const fn len(self) -> usize {
        self.bits.count_ones() as usize
    }

    /// Returns all reasons in stable semantic order.
    ///
    /// The returned array is fixed-size to avoid allocation. `None` entries
    /// represent unused positions.
    #[must_use]
    pub const fn as_array(self) -> [Option<DependencyKind>; 4] {
        let mut result = [None, None, None, None];
        let mut index = 0usize;

        if self.contains(DependencyKind::QubitOrder) {
            result[index] = Some(DependencyKind::QubitOrder);
            index += 1;
        }

        if self.contains(DependencyKind::ClassicalData) {
            result[index] = Some(DependencyKind::ClassicalData);
            index += 1;
        }

        if self.contains(DependencyKind::MeasurementFeedback) {
            result[index] = Some(DependencyKind::MeasurementFeedback);
            index += 1;
        }

        if self.contains(DependencyKind::ProgramOrder) {
            result[index] = Some(DependencyKind::ProgramOrder);
        }

        result
    }

    /// Returns an iterator over the contained reasons.
    #[must_use]
    pub fn iter(self) -> DependencyReasonIter {
        DependencyReasonIter {
            set: self,
            index: 0,
        }
    }
}

/// Iterator over dependency reasons in deterministic order.
#[derive(Debug, Clone, Copy)]
pub struct DependencyReasonIter {
    set: DependencyReasonSet,
    index: usize,
}

impl Iterator for DependencyReasonIter {
    type Item = DependencyKind;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 4 {
            let index = self.index;
            self.index += 1;

            let kind = match index {
                0 => DependencyKind::QubitOrder,
                1 => DependencyKind::ClassicalData,
                2 => DependencyKind::MeasurementFeedback,
                3 => DependencyKind::ProgramOrder,
                _ => return None,
            };

            if self.set.contains(kind) {
                return Some(kind);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.set.len().saturating_sub(self.index);
        (0, Some(remaining))
    }
}

// =============================================================================
// Dependency edge
// =============================================================================

/// One directed dependency edge.
///
/// ```text
/// source
///    |
///    v
/// target
/// ```
///
/// The edge means `target` cannot be semantically considered independent from
/// `source` under the dependency reasons recorded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyEdge {
    source: OperationId,
    target: OperationId,
    reasons: DependencyReasonSet,
}

impl DependencyEdge {
    /// Creates an edge.
    ///
    /// The constructor rejects self-dependencies.
    pub fn new(
        source: OperationId,
        target: OperationId,
        reasons: DependencyReasonSet,
    ) -> Result<Self, DependencyError> {
        if source == target {
            return Err(DependencyError::SelfDependency { operation: source });
        }

        if reasons.is_empty() {
            return Err(DependencyError::EmptyReason {
                source,
                target,
            });
        }

        Ok(Self {
            source,
            target,
            reasons,
        })
    }

    /// Returns the source operation.
    #[must_use]
    pub const fn source(self) -> OperationId {
        self.source
    }

    /// Returns the target operation.
    #[must_use]
    pub const fn target(self) -> OperationId {
        self.target
    }

    /// Returns the semantic dependency reasons.
    #[must_use]
    pub const fn reasons(self) -> DependencyReasonSet {
        self.reasons
    }

    /// Returns whether this edge has the specified reason.
    #[must_use]
    pub const fn has_reason(self, kind: DependencyKind) -> bool {
        self.reasons.contains(kind)
    }
}

// =============================================================================
// Dependency graph
// =============================================================================

/// Deterministic sparse dependency graph for a sequence of canonical
/// operations.
///
/// The graph stores only actual operations and actual dependency edges.
///
/// It does not allocate an adjacency matrix.
///
/// Nodes retain the original semantic operation order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraph {
    nodes: Vec<OperationId>,
    edges: Vec<DependencyEdge>,

    /// Outgoing edge indexes for each node.
    ///
    /// The outer vector is proportional to the number of operations, not the
    /// number of declared qubits.
    outgoing: Vec<Vec<usize>>,

    /// Incoming edge indexes for each node.
    incoming: Vec<Vec<usize>>,

    /// Stable node lookup.
    node_index: BTreeMap<OperationId, usize>,

    /// Longest dependency depth ending at each node.
    ///
    /// Depth is a logical graph depth, not physical execution latency.
    depths: Vec<usize>,
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            outgoing: Vec::new(),
            incoming: Vec::new(),
            node_index: BTreeMap::new(),
            depths: Vec::new(),
        }
    }

    /// Returns whether the graph contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Number of operation nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of dependency edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns operation nodes in original semantic order.
    #[must_use]
    pub fn nodes(&self) -> &[OperationId] {
        &self.nodes
    }

    /// Returns dependency edges in deterministic order.
    #[must_use]
    pub fn edges(&self) -> &[DependencyEdge] {
        &self.edges
    }

    /// Returns the logical dependency depth of one operation.
    ///
    /// The first node has depth 0.
    ///
    /// A node whose deepest predecessor has depth `d` has depth `d + 1`.
    ///
    /// This is not hardware latency.
    #[must_use]
    pub fn depth_of(&self, operation: OperationId) -> Option<usize> {
        self.node_index
            .get(&operation)
            .copied()
            .and_then(|index| self.depths.get(index).copied())
    }

    /// Returns the maximum logical dependency depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.depths.iter().copied().max().unwrap_or(0)
    }

    /// Returns whether the graph is acyclic.
    ///
    /// A graph constructed by [`analyze_operations`] should always be acyclic
    /// because dependency edges are emitted only from earlier operations to
    /// later operations.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.topological_order().is_some()
    }

    /// Returns the predecessors of an operation.
    ///
    /// The returned slice is ordered by deterministic edge insertion order.
    #[must_use]
    pub fn predecessors(
        &self,
        operation: OperationId,
    ) -> Option<Vec<DependencyEdge>> {
        let index = self.node_index.get(&operation).copied()?;

        let result = self.incoming[index]
            .iter()
            .filter_map(|edge_index| self.edges.get(*edge_index).copied())
            .collect();

        Some(result)
    }

    /// Returns the successors of an operation.
    ///
    /// The returned slice is ordered by deterministic edge insertion order.
    #[must_use]
    pub fn successors(
        &self,
        operation: OperationId,
    ) -> Option<Vec<DependencyEdge>> {
        let index = self.node_index.get(&operation).copied()?;

        let result = self.outgoing[index]
            .iter()
            .filter_map(|edge_index| self.edges.get(*edge_index).copied())
            .collect();

        Some(result)
    }

    /// Returns a deterministic topological ordering.
    ///
    /// Because the canonical dependency builder emits only forward semantic
    /// dependencies, the normal result is the original operation order.
    ///
    /// `None` is returned if the graph contains a cycle.
    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<OperationId>> {
        let node_count = self.nodes.len();

        if node_count == 0 {
            return Some(Vec::new());
        }

        let mut indegree = Vec::with_capacity(node_count);

        for incoming in &self.incoming {
            indegree.push(incoming.len());
        }

        let mut ready = BTreeSet::<usize>::new();

        for (index, degree) in indegree.iter().copied().enumerate() {
            if degree == 0 {
                ready.insert(index);
            }
        }

        let mut order = Vec::with_capacity(node_count);

        while let Some(&index) = ready.iter().next() {
            ready.remove(&index);

            order.push(self.nodes[index]);

            for edge_index in &self.outgoing[index] {
                let edge = self.edges[*edge_index];

                let target = self.node_index.get(&edge.target)?;

                let target_degree = indegree.get_mut(*target)?;

                if *target_degree == 0 {
                    return None;
                }

                *target_degree -= 1;

                if *target_degree == 0 {
                    ready.insert(*target);
                }
            }
        }

        if order.len() == node_count {
            Some(order)
        } else {
            None
        }
    }

    /// Returns the number of edges with a particular dependency reason.
    #[must_use]
    pub fn edge_count_with_reason(&self, reason: DependencyKind) -> usize {
        self.edges
            .iter()
            .filter(|edge| edge.has_reason(reason))
            .count()
    }

    /// Returns all edges with a particular dependency reason.
    ///
    /// The returned vector contains references into graph-owned edge storage.
    #[must_use]
    pub fn edges_with_reason(
        &self,
        reason: DependencyKind,
    ) -> Vec<DependencyEdge> {
        self.edges
            .iter()
            .copied()
            .filter(|edge| edge.has_reason(reason))
            .collect()
    }

    /// Returns whether a direct dependency exists.
    #[must_use]
    pub fn depends_directly_on(
        &self,
        target: OperationId,
        source: OperationId,
    ) -> bool {
        self.node_index
            .get(&target)
            .and_then(|index| self.incoming.get(*index))
            .map(|edges| {
                edges.iter().any(|edge_index| {
                    self.edges
                        .get(*edge_index)
                        .map_or(false, |edge| edge.source == source)
                })
            })
            .unwrap_or(false)
    }

    /// Returns whether `target` transitively depends on `source`.
    ///
    /// This performs an iterative traversal and therefore does not recurse
    /// through the graph.
    pub fn depends_transitively_on(
        &self,
        target: OperationId,
        source: OperationId,
    ) -> bool {
        if target == source {
            return false;
        }

        let target_index = match self.node_index.get(&target).copied() {
            Some(index) => index,
            None => return false,
        };

        let source_index = match self.node_index.get(&source).copied() {
            Some(index) => index,
            None => return false,
        };

        let mut stack = Vec::<usize>::new();
        let mut visited = BTreeSet::<usize>::new();

        stack.push(target_index);

        while let Some(current) = stack.pop() {
            if current == source_index {
                return true;
            }

            if !visited.insert(current) {
                continue;
            }

            for edge_index in &self.incoming[current] {
                if let Some(edge) = self.edges.get(*edge_index) {
                    if let Some(predecessor) =
                        self.node_index.get(&edge.source).copied()
                    {
                        stack.push(predecessor);
                    }
                }
            }
        }

        false
    }

    fn add_node(
        &mut self,
        id: OperationId,
    ) -> Result<usize, DependencyError> {
        if self.node_index.contains_key(&id) {
            return Err(DependencyError::DuplicateOperationId { id });
        }

        let index = self.nodes.len();

        self.nodes.push(id);
        self.node_index.insert(id, index);
        self.outgoing.push(Vec::new());
        self.incoming.push(Vec::new());
        self.depths.push(0);

        Ok(index)
    }

    fn add_or_merge_edge(
        &mut self,
        source: OperationId,
        target: OperationId,
        reasons: DependencyReasonSet,
    ) -> Result<(), DependencyError> {
        if source == target {
            return Err(DependencyError::SelfDependency {
                operation: source,
            });
        }

        let source_index = self
            .node_index
            .get(&source)
            .copied()
            .ok_or(DependencyError::UnknownOperation { operation: source })?;

        let target_index = self
            .node_index
            .get(&target)
            .copied()
            .ok_or(DependencyError::UnknownOperation { operation: target })?;

        if source_index >= target_index {
            return Err(DependencyError::BackwardDependency {
                source,
                target,
            });
        }

        if let Some(edge_index) = self.outgoing[source_index]
            .iter()
            .copied()
            .find(|edge_index| {
                self.edges
                    .get(*edge_index)
                    .map_or(false, |edge| edge.target == target)
            })
        {
            let edge = self
                .edges
                .get_mut(edge_index)
                .ok_or(DependencyError::InternalGraphCorruption)?;

            let mut merged = edge.reasons;

            for reason in reasons.iter() {
                merged.insert(reason);
            }

            edge.reasons = merged;
        } else {
            let edge = DependencyEdge::new(source, target, reasons)?;

            let edge_index = self.edges.len();

            self.edges.push(edge);
            self.outgoing[source_index].push(edge_index);
            self.incoming[target_index].push(edge_index);
        }

        let source_depth = self
            .depths
            .get(source_index)
            .copied()
            .ok_or(DependencyError::InternalGraphCorruption)?;

        let candidate_depth = source_depth
            .checked_add(1)
            .ok_or(DependencyError::DepthOverflow)?;

        let target_depth = self
            .depths
            .get_mut(target_index)
            .ok_or(DependencyError::InternalGraphCorruption)?;

        if candidate_depth > *target_depth {
            *target_depth = candidate_depth;
        }

        Ok(())
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Dependency analysis errors
// =============================================================================

/// Errors produced by dependency analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyError {
    /// The same stable operation identity occurred more than once.
    DuplicateOperationId {
        /// Duplicated operation identity.
        id: OperationId,
    },

    /// A dependency refers to an operation that is not present.
    UnknownOperation {
        /// Referenced operation identity.
        operation: OperationId,
    },

    /// An operation would depend directly on itself.
    SelfDependency {
        /// Operation identity.
        operation: OperationId,
    },

    /// A dependency points backward in semantic program order.
    BackwardDependency {
        /// Source operation.
        source: OperationId,

        /// Target operation.
        target: OperationId,
    },

    /// An edge was created without a semantic reason.
    EmptyReason {
        /// Source operation.
        source: OperationId,

        /// Target operation.
        target: OperationId,
    },

    /// Graph depth overflowed the host `usize` representation.
    DepthOverflow,

    /// Internal graph invariants were violated.
    ///
    /// This is a defensive error rather than a recoverable user input error.
    InternalGraphCorruption,

    /// A resource index could not be represented safely.
    ResourceIndexOverflow,

    /// The analyzed operation collection exceeded representable storage.
    OperationCountOverflow,
}

impl fmt::Display for DependencyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperationId { id } => {
                write!(
                    formatter,
                    "dependency analysis encountered duplicate operation identity {id}"
                )
            }

            Self::UnknownOperation { operation } => {
                write!(
                    formatter,
                    "dependency analysis references unknown operation {operation}"
                )
            }

            Self::SelfDependency { operation } => {
                write!(
                    formatter,
                    "operation {operation} cannot depend directly on itself"
                )
            }

            Self::BackwardDependency { source, target } => {
                write!(
                    formatter,
                    "dependency {source} -> {target} violates semantic program order"
                )
            }

            Self::EmptyReason { source, target } => {
                write!(
                    formatter,
                    "dependency {source} -> {target} has no semantic reason"
                )
            }

            Self::DepthOverflow => {
                formatter.write_str(
                    "dependency graph logical depth overflowed usize",
                )
            }

            Self::InternalGraphCorruption => {
                formatter.write_str(
                    "internal dependency graph invariant was violated",
                )
            }

            Self::ResourceIndexOverflow => {
                formatter.write_str(
                    "dependency resource index arithmetic overflowed",
                )
            }

            Self::OperationCountOverflow => {
                formatter.write_str(
                    "dependency operation count overflowed",
                )
            }
        }
    }
}

impl std::error::Error for DependencyError {}

// =============================================================================
// Analysis result
// =============================================================================

/// Complete deterministic result of dependency analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyAnalysis {
    graph: DependencyGraph,
}

impl DependencyAnalysis {
    /// Creates an analysis result from a completed graph.
    #[must_use]
    pub const fn new(graph: DependencyGraph) -> Self {
        Self { graph }
    }

    /// Returns the dependency graph.
    #[must_use]
    pub const fn graph(&self) -> &DependencyGraph {
        &self.graph
    }

    /// Consumes the analysis and returns the graph.
    #[must_use]
    pub fn into_graph(self) -> DependencyGraph {
        self.graph
    }

    /// Returns the number of operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Returns the logical dependency depth.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.graph.depth()
    }

    /// Returns whether the dependency graph is acyclic.
    #[must_use]
    pub fn is_acyclic(&self) -> bool {
        self.graph.is_acyclic()
    }

    /// Returns a deterministic topological ordering.
    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<OperationId>> {
        self.graph.topological_order()
    }
}

// =============================================================================
// Resource tracking
// =============================================================================

/// Internal semantic resource key.
///
/// Qubits and classical bits are kept in separate namespaces so `q0` can
/// never collide with `c0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ResourceKey {
    Qubit(QubitId),
    Classical(usize),
}

/// Tracks the most recent operation touching each semantic resource.
///
/// The map contains only resources actually referenced by operations.
#[derive(Debug, Default)]
struct LastUse {
    resources: BTreeMap<ResourceKey, OperationId>,
}

impl LastUse {
    fn previous(
        &self,
        resource: ResourceKey,
    ) -> Option<OperationId> {
        self.resources.get(&resource).copied()
    }

    fn update(
        &mut self,
        resource: ResourceKey,
        operation: OperationId,
    ) {
        self.resources.insert(resource, operation);
    }
}

// =============================================================================
// Public analysis entry points
// =============================================================================

/// Analyzes a canonical ordered operation slice.
///
/// This is the primary low-level API.
///
/// A circuit can be passed directly as:
///
/// ```text
/// analyze_operations(circuit.operations())
/// ```
///
/// The function does not mutate the operation slice.
pub fn analyze_operations(
    operations: &[Operation],
) -> Result<DependencyAnalysis, DependencyError> {
    let mut graph = DependencyGraph::new();

    // -------------------------------------------------------------------------
    // Phase 1: establish every node and validate identity uniqueness.
    // -------------------------------------------------------------------------
    //
    // Doing this before edge construction means a malformed operation sequence
    // cannot leave a partially built graph as the returned result.
    //
    // It also gives all subsequent resource references a complete identity
    // namespace.

    for operation in operations {
        graph.add_node(operation.id())?;
    }

    // -------------------------------------------------------------------------
    // Phase 2: sparse dependency construction.
    // -------------------------------------------------------------------------

    let mut last_use = LastUse::default();

    for operation in operations {
        let operation_id = operation.id();

        // A local per-operation map merges multiple reasons involving the same
        // predecessor. For example, two operations may share both q0 and c0.
        //
        // This avoids emitting duplicate edges:
        //
        //     op1 -> op2 [q0]
        //     op1 -> op2 [c0]
        //
        // and instead emits:
        //
        //     op1 -> op2 [q0, c0]
        //
        let mut predecessors =
            BTreeMap::<OperationId, DependencyReasonSet>::new();

        // ---------------------------------------------------------------------
        // Quantum dependencies.
        // ---------------------------------------------------------------------

        //
        // `qubits_vec()` is intentionally used here because OperationBody
        // variants have different storage layouts. The operation API owns the
        // representation details; this analysis only consumes the semantic
        // logical qubit list.
        //
        for qubit in operation.body().qubits_vec() {
            let resource = ResourceKey::Qubit(qubit);

            if let Some(previous) = last_use.previous(resource) {
                let reasons =
                    predecessors.entry(previous).or_default();

                reasons.insert(DependencyKind::QubitOrder);
            }
        }

        // ---------------------------------------------------------------------
        // Classical dependencies.
        // ---------------------------------------------------------------------

        //
        // The canonical operation API exposes classical references through
        // `classical_bits()`. The classical namespace is represented by the
        // canonical classical-bit index used by the current operation model.
        //
        for bit in operation.classical_bits() {
            let resource = ResourceKey::Classical(*bit);

            if let Some(previous) = last_use.previous(resource) {
                let reasons =
                    predecessors.entry(previous).or_default();

                reasons.insert(DependencyKind::ClassicalData);
            }
        }

        // ---------------------------------------------------------------------
        // Measurement feedback dependencies.
        // ---------------------------------------------------------------------

        //
        // A condition is stronger than merely "both operations touch c0":
        //
        //     measure q0 -> c0
        //     if c0 { X q1 }
        //
        // is a measurement-feedback dependency.
        //
        // The condition bit is already normally present in `classical_bits()`
        // for canonical operations. We add the specialized reason here while
        // retaining the same sparse last-use structure.
        //
        if let Some(condition) = operation.condition() {
            let bit = condition.bit().index();
            let resource = ResourceKey::Classical(bit);

            if let Some(previous) = last_use.previous(resource) {
                let reasons =
                    predecessors.entry(previous).or_default();

                reasons.insert(DependencyKind::MeasurementFeedback);
            }
        }

        // ---------------------------------------------------------------------
        // Commit all predecessor edges.
        // ---------------------------------------------------------------------

        for (source, reasons) in predecessors {
            graph.add_or_merge_edge(
                source,
                operation_id,
                reasons,
            )?;
        }

        // ---------------------------------------------------------------------
        // Update last-use state AFTER dependency extraction.
        // ---------------------------------------------------------------------
        //
        // This ordering is essential. Updating before querying would cause the
        // current operation to appear as its own predecessor.
        //

        for qubit in operation.body().qubits_vec() {
            last_use.update(
                ResourceKey::Qubit(qubit),
                operation_id,
            );
        }

        for bit in operation.classical_bits() {
            last_use.update(
                ResourceKey::Classical(*bit),
                operation_id,
            );
        }

        if let Some(condition) = operation.condition() {
            let bit = condition.bit().index();

            last_use.update(
                ResourceKey::Classical(bit),
                operation_id,
            );
        }
    }

    // -------------------------------------------------------------------------
    // Final graph invariant.
    // -------------------------------------------------------------------------

    //
    // Every generated edge is forward in operation order, so this should
    // always succeed. Keeping the check here makes the contract explicit and
    // protects future extensions of the analysis.
    //

    if !graph.is_acyclic() {
        return Err(DependencyError::InternalGraphCorruption);
    }

    Ok(DependencyAnalysis::new(graph))
}

/// Convenience alias for callers that think in terms of a dependency graph.
///
/// This function is intentionally separate from `analyze_operations` so the
/// returned type can evolve without changing the low-level analysis contract.
pub fn build_dependency_graph(
    operations: &[Operation],
) -> Result<DependencyGraph, DependencyError> {
    analyze_operations(operations).map(DependencyAnalysis::into_graph)
}

// =============================================================================
// Derived dependency helpers
// =============================================================================

/// Returns all direct predecessors of an operation.
#[must_use]
pub fn predecessors(
    graph: &DependencyGraph,
    operation: OperationId,
) -> Option<Vec<DependencyEdge>> {
    graph.predecessors(operation)
}

/// Returns all direct successors of an operation.
#[must_use]
pub fn successors(
    graph: &DependencyGraph,
    operation: OperationId,
) -> Option<Vec<DependencyEdge>> {
    graph.successors(operation)
}

/// Returns whether `target` has a direct dependency on `source`.
#[must_use]
pub fn depends_directly_on(
    graph: &DependencyGraph,
    target: OperationId,
    source: OperationId,
) -> bool {
    graph.depends_directly_on(target, source)
}

/// Returns whether `target` transitively depends on `source`.
#[must_use]
pub fn depends_transitively_on(
    graph: &DependencyGraph,
    target: OperationId,
    source: OperationId,
) -> bool {
    graph.depends_transitively_on(target, source)
}

/// Computes the logical dependency depth of an operation.
#[must_use]
pub fn dependency_depth(
    graph: &DependencyGraph,
    operation: OperationId,
) -> Option<usize> {
    graph.depth_of(operation)
}

/// Computes the maximum logical dependency depth.
#[must_use]
pub fn dependency_depth_of_graph(
    graph: &DependencyGraph,
) -> usize {
    graph.depth()
}

// =============================================================================
// Explicit integration API
// =============================================================================

/// Adds an explicit program-order dependency to an already-built graph.
///
/// This API is intentionally explicit because semantic program order should
/// not automatically become a dependency between every pair of operations.
///
/// The source operation must precede the target operation in the graph's
/// canonical node ordering.
pub fn add_program_order_dependency(
    graph: &mut DependencyGraph,
    source: OperationId,
    target: OperationId,
) -> Result<(), DependencyError> {
    let mut reasons = DependencyReasonSet::new();
    reasons.insert(DependencyKind::ProgramOrder);

    graph.add_or_merge_edge(
        source,
        target,
        reasons,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> OperationId {
        OperationId::new(value)
    }

    #[test]
    fn empty_reason_set_is_empty() {
        let reasons = DependencyReasonSet::new();

        assert!(reasons.is_empty());
        assert_eq!(reasons.len(), 0);
    }

    #[test]
    fn reason_set_is_deterministic() {
        let mut reasons = DependencyReasonSet::new();

        reasons.insert(DependencyKind::MeasurementFeedback);
        reasons.insert(DependencyKind::QubitOrder);

        let values: Vec<DependencyKind> =
            reasons.iter().collect();

        assert_eq!(
            values,
            vec![
                DependencyKind::QubitOrder,
                DependencyKind::MeasurementFeedback,
            ]
        );
    }

    #[test]
    fn reason_set_deduplicates() {
        let mut reasons = DependencyReasonSet::new();

        reasons.insert(DependencyKind::QubitOrder);
        reasons.insert(DependencyKind::QubitOrder);

        assert_eq!(reasons.len(), 1);
        assert!(reasons.contains(DependencyKind::QubitOrder));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        let result =
            DependencyEdge::new(id(1), id(1), reasons);

        assert!(matches!(
            result,
            Err(DependencyError::SelfDependency {
                operation
            }) if operation == id(1)
        ));
    }

    #[test]
    fn empty_edge_reason_is_rejected() {
        let result =
            DependencyEdge::new(
                id(1),
                id(2),
                DependencyReasonSet::new(),
            );

        assert!(matches!(
            result,
            Err(DependencyError::EmptyReason {
                source,
                target
            }) if source == id(1) && target == id(2)
        ));
    }

    #[test]
    fn graph_preserves_node_order() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(10)).unwrap();
        graph.add_node(id(20)).unwrap();
        graph.add_node(id(30)).unwrap();

        assert_eq!(
            graph.nodes(),
            &[id(10), id(20), id(30)]
        );
    }

    #[test]
    fn duplicate_node_is_rejected() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(10)).unwrap();

        let result = graph.add_node(id(10));

        assert!(matches!(
            result,
            Err(DependencyError::DuplicateOperationId { id })
                if id == id(10)
        ));
    }

    #[test]
    fn backward_dependency_is_rejected() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(10)).unwrap();
        graph.add_node(id(20)).unwrap();

        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        let result =
            graph.add_or_merge_edge(
                id(20),
                id(10),
                reasons,
            );

        assert!(matches!(
            result,
            Err(DependencyError::BackwardDependency {
                source,
                target
            }) if source == id(20) && target == id(10)
        ));
    }

    #[test]
    fn edge_reasons_merge() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();

        let mut qubit_reason = DependencyReasonSet::new();
        qubit_reason.insert(DependencyKind::QubitOrder);

        let mut classical_reason = DependencyReasonSet::new();
        classical_reason.insert(DependencyKind::ClassicalData);

        graph
            .add_or_merge_edge(
                id(1),
                id(2),
                qubit_reason,
            )
            .unwrap();

        graph
            .add_or_merge_edge(
                id(1),
                id(2),
                classical_reason,
            )
            .unwrap();

        assert_eq!(graph.edge_count(), 1);

        let edge = graph.edges()[0];

        assert!(edge.has_reason(
            DependencyKind::QubitOrder
        ));

        assert!(edge.has_reason(
            DependencyKind::ClassicalData
        ));

        assert_eq!(edge.reasons().len(), 2);
    }

    #[test]
    fn graph_depth_is_updated_from_edges() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();
        graph.add_node(id(3)).unwrap();

        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        graph
            .add_or_merge_edge(
                id(1),
                id(2),
                reasons,
            )
            .unwrap();

        graph
            .add_or_merge_edge(
                id(2),
                id(3),
                reasons,
            )
            .unwrap();

        assert_eq!(graph.depth_of(id(1)), Some(0));
        assert_eq!(graph.depth_of(id(2)), Some(1));
        assert_eq!(graph.depth_of(id(3)), Some(2));
        assert_eq!(graph.depth(), 2);
    }

    #[test]
    fn topological_order_is_deterministic() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();
        graph.add_node(id(3)).unwrap();

        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        graph
            .add_or_merge_edge(
                id(1),
                id(3),
                reasons,
            )
            .unwrap();

        graph
            .add_or_merge_edge(
                id(2),
                id(3),
                reasons,
            )
            .unwrap();

        assert_eq!(
            graph.topological_order(),
            Some(vec![
                id(1),
                id(2),
                id(3)
            ])
        );
    }

    #[test]
    fn direct_dependency_query_works() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();

        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        graph
            .add_or_merge_edge(
                id(1),
                id(2),
                reasons,
            )
            .unwrap();

        assert!(
            graph.depends_directly_on(
                id(2),
                id(1)
            )
        );

        assert!(
            !graph.depends_directly_on(
                id(1),
                id(2)
            )
        );
    }

    #[test]
    fn transitive_dependency_query_works() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();
        graph.add_node(id(3)).unwrap();

        let mut reasons = DependencyReasonSet::new();
        reasons.insert(DependencyKind::QubitOrder);

        graph
            .add_or_merge_edge(
                id(1),
                id(2),
                reasons,
            )
            .unwrap();

        graph
            .add_or_merge_edge(
                id(2),
                id(3),
                reasons,
            )
            .unwrap();

        assert!(
            graph.depends_transitively_on(
                id(3),
                id(1)
            )
        );

        assert!(
            !graph.depends_transitively_on(
                id(1),
                id(3)
            )
        );
    }

    #[test]
    fn resource_namespaces_do_not_collide() {
        let q =
            ResourceKey::Qubit(QubitId::new(0));

        let c =
            ResourceKey::Classical(0);

        assert_ne!(q, c);
    }

    #[test]
    fn program_order_can_be_added_explicitly() {
        let mut graph = DependencyGraph::new();

        graph.add_node(id(1)).unwrap();
        graph.add_node(id(2)).unwrap();

        add_program_order_dependency(
            &mut graph,
            id(1),
            id(2),
        )
        .unwrap();

        assert_eq!(graph.edge_count(), 1);
        assert!(
            graph.edges()[0]
                .has_reason(
                    DependencyKind::ProgramOrder
                )
        );
    }

    #[test]
    fn empty_graph_is_acyclic() {
        let graph = DependencyGraph::new();

        assert!(graph.is_empty());
        assert!(graph.is_acyclic());
        assert_eq!(graph.depth(), 0);
        assert_eq!(
            graph.topological_order(),
            Some(Vec::new())
        );
    }
}