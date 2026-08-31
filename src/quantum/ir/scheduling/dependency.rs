//! Zamani Quantum IR — Scheduling Dependency Graph
//!
//! Path:
//!     src/quantum/ir/scheduling/dependency.rs
//!
//! # Purpose
//!
//! This module provides the scheduler-facing dependency graph for the
//! canonical Zamani Quantum IR.
//!
//! It consumes semantic dependencies from:
//!
//!     quantum::ir::timing::dependency
//!
//! and indexes them for scheduling, validation, analysis, and downstream
//! transformations.
//!
//! # Architectural boundary
//!
//! ```text
//! canonical semantic IR
//!         |
//!         v
//! timing::dependency
//!         |
//!         v
//! scheduling::dependency
//!         |
//!         +----> scheduler
//!         +----> critical-path analysis
//!         +----> resource scheduling
//!         +----> parallelism analysis
//!         +----> validation
//! ```
//!
//! This module DOES NOT:
//!
//! - assign timestamps;
//! - perform routing;
//! - select hardware;
//! - perform optimization;
//! - allocate physical qubits;
//! - generate pulses;
//! - execute programs;
//! - communicate with a backend.
//!
//! It represents the graph the scheduler consumes.
//!
//! # Important distinction
//!
//! `timing::dependency::TemporalDependency` answers:
//!
//!     "What semantic dependency exists?"
//!
//! This module answers:
//!
//!     "How are those dependencies indexed and traversed by a scheduler?"
//!
//! Therefore there must be exactly one semantic dependency representation.
//!
//! # Scalability
//!
//! There is no fixed number of:
//!
//! - operations;
//! - qubits;
//! - dependencies;
//! - resources;
//! - channels;
//! - graph nodes.
//!
//! The graph grows according to available resources and explicit
//! `QuantumIrLimits` policy.
//!
//! No architectural constant such as 64, 128, 4096, or 1_000_000 is used.
//!
//! `usize` is used only for Rust collection sizes/indices.
//!
//! Stable semantic identities remain strongly typed.
//!
//! # Determinism
//!
//! All public iteration APIs are deterministic.
//!
//! BTreeMap/BTreeSet are intentionally used instead of HashMap/HashSet for
//! semantic graph ordering.
//!
//! The graph does not derive semantic meaning from insertion order.
//!
//! # Cycle semantics
//!
//! A scheduling dependency graph is normally expected to be acyclic for
//! mandatory ordering dependencies.
//!
//! However, this module does NOT reject every graph cycle at insertion time.
//!
//! Reasons:
//!
//! - dynamic/control-flow IR can be represented before lowering;
//! - graph construction may occur incrementally;
//! - validation and scheduling may require different policies;
//! - non-ordering dependencies can legitimately form strongly connected
//!   relationships.
//!
//! `find_cycle()` and `validate_acyclic_ordering()` provide explicit checks.
//!
//! # Qubit integration
//!
//! Qubit resources are represented using the canonical types from:
//!
//!     quantum::ir::qubit::QubitId
//!     quantum::ir::qubit::PhysicalQubitId
//!
//! This module does not redefine qubit identity.
//!
//! Logical and physical qubits remain distinct.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `timing::dependency`
//!     Provides semantic dependency edges.
//!
//! `identity`
//!     Provides stable operation/resource identities.
//!
//! `qubit`
//!     Provides logical/physical qubit identities.
//!
//! `schedule`
//!     Consumes the dependency information while representing concrete
//!     placement.
//!
//! `validation`
//!     Validates graph correctness.
//!
//! `analysis`
//!     Uses graph traversal for dependency/critical-path analysis.
//!
//! `optimization`
//!     May construct a new graph after transforming IR.
//!
//! `hardware`
//!     Is downstream and is never a dependency of this module.
//!
//! # Ownership
//!
//! ## Owns
//!
//! - scheduler-facing graph storage;
//! - node registration;
//! - deterministic adjacency indexes;
//! - dependency insertion/removal;
//! - predecessor/successor queries;
//! - ordering traversal;
//! - cycle detection;
//! - graph validation;
//! - graph statistics.
//!
//! ## Does not own
//!
//! - operation definitions;
//! - dependency semantics;
//! - qubit definitions;
//! - timing primitive definitions;
//! - hardware;
//! - routing;
//! - scheduling policy.
//!
//! # Serialization
//!
//! Graph serialization must serialize semantic dependency records in
//! deterministic order.
//!
//! Internal reverse indexes are derived data and need not be serialized.
//!
//! # Hashing
//!
//! A canonical hash must be based on sorted semantic graph content rather
//! than BTreeMap implementation details.
//!
//! # Thread safety
//!
//! The graph contains ordinary owned Rust collections and has no global
//! mutable state.
//!
//! It can therefore be transferred/shared according to normal Rust ownership
//! and synchronization rules.
//!
//! # Important invariant
//!
//! Every dependency stored in the graph must have both endpoints registered.
//!
//! This prevents dangling scheduling references.
//!
//! The graph intentionally does not attempt to resolve unknown operations.
//! The caller must register all endpoints first.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::timing::dependency::{
    DependencyEndpoint,
    DependencyEndpointKind,
    DependencyKind,
    DependencyStrength,
    TemporalDependency,
};

/// Result type used by the scheduling dependency graph.
pub type DependencyGraphResult<T> = Result<T, DependencyGraphError>;

/// Errors produced by the scheduler-facing dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyGraphError {
    /// An endpoint was referenced without being registered.
    UnknownEndpoint {
        /// Missing endpoint.
        endpoint: DependencyEndpoint,
    },

    /// An endpoint was registered twice with incompatible information.
    DuplicateEndpoint {
        /// Endpoint already present.
        endpoint: DependencyEndpoint,
    },

    /// A dependency was already present.
    DuplicateDependency {
        /// Existing dependency.
        dependency: TemporalDependency,
    },

    /// A dependency was not present.
    MissingDependency {
        /// Requested dependency.
        dependency: TemporalDependency,
    },

    /// A dependency is invalid for the scheduling graph.
    InvalidDependency {
        /// Explanation.
        message: String,
    },

    /// The graph contains a mandatory ordering cycle.
    OrderingCycle {
        /// Deterministic cycle path.
        cycle: Vec<DependencyEndpoint>,
    },

    /// A graph invariant was violated.
    InvariantViolation {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEndpoint { endpoint } => {
                write!(formatter, "unknown dependency endpoint `{endpoint}`")
            }

            Self::DuplicateEndpoint { endpoint } => {
                write!(formatter, "duplicate dependency endpoint `{endpoint}`")
            }

            Self::DuplicateDependency { dependency } => {
                write!(
                    formatter,
                    "duplicate dependency from `{}` to `{}`",
                    dependency.source(),
                    dependency.target()
                )
            }

            Self::MissingDependency { dependency } => {
                write!(
                    formatter,
                    "dependency from `{}` to `{}` does not exist",
                    dependency.source(),
                    dependency.target()
                )
            }

            Self::InvalidDependency { message } => {
                write!(formatter, "invalid scheduling dependency: {message}")
            }

            Self::OrderingCycle { cycle } => {
                formatter.write_str("mandatory scheduling dependency cycle: ")?;

                for (index, endpoint) in cycle.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(" -> ")?;
                    }

                    write!(formatter, "{endpoint}")?;
                }

                Ok(())
            }

            Self::InvariantViolation { message } => {
                write!(formatter, "dependency graph invariant violation: {message}")
            }
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// Category of scheduler graph node.
///
/// A graph node may represent an operation or another semantic endpoint that
/// participates in scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DependencyNodeKind {
    /// Quantum/classical/structural operation.
    Operation,

    /// Produced/consumed value.
    Value,

    /// Abstract scheduling resource.
    Resource,

    /// Pulse-level operation.
    Pulse,

    /// Abstract channel.
    Channel,

    /// Abstract frame.
    Frame,

    /// Waveform definition.
    Waveform,
}

impl From<DependencyEndpointKind> for DependencyNodeKind {
    fn from(value: DependencyEndpointKind) -> Self {
        match value {
            DependencyEndpointKind::Operation => Self::Operation,
            DependencyEndpointKind::Value => Self::Value,
            DependencyEndpointKind::Resource => Self::Resource,
            DependencyEndpointKind::Pulse => Self::Pulse,
            DependencyEndpointKind::Channel => Self::Channel,
            DependencyEndpointKind::Frame => Self::Frame,
            DependencyEndpointKind::Waveform => Self::Waveform,
        }
    }
}

/// A scheduler graph node.
///
/// The node contains only stable identity information. Operation semantics
/// remain owned by the canonical operation IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyNode {
    endpoint: DependencyEndpoint,
}

impl DependencyNode {
    /// Creates a graph node from a semantic endpoint.
    #[must_use]
    pub const fn new(endpoint: DependencyEndpoint) -> Self {
        Self { endpoint }
    }

    /// Returns the underlying endpoint.
    #[must_use]
    pub const fn endpoint(self) -> DependencyEndpoint {
        self.endpoint
    }

    /// Returns the node kind.
    #[must_use]
    pub const fn kind(self) -> DependencyNodeKind {
        match self.endpoint.kind() {
            DependencyEndpointKind::Operation => DependencyNodeKind::Operation,
            DependencyEndpointKind::Value => DependencyNodeKind::Value,
            DependencyEndpointKind::Resource => DependencyNodeKind::Resource,
            DependencyEndpointKind::Pulse => DependencyNodeKind::Pulse,
            DependencyEndpointKind::Channel => DependencyNodeKind::Channel,
            DependencyEndpointKind::Frame => DependencyNodeKind::Frame,
            DependencyEndpointKind::Waveform => DependencyNodeKind::Waveform,
        }
    }
}

impl From<DependencyEndpoint> for DependencyNode {
    fn from(endpoint: DependencyEndpoint) -> Self {
        Self::new(endpoint)
    }
}

impl fmt::Display for DependencyNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.endpoint.fmt(formatter)
    }
}

/// Resource that may induce a scheduler dependency.
///
/// This type exists specifically at the scheduling boundary.
///
/// It does not claim that a physical resource actually exists. Hardware
/// availability is resolved later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingResource {
    /// Logical program qubit.
    LogicalQubit(QubitId),

    /// Physical qubit selected by a later routing/mapping stage.
    PhysicalQubit(PhysicalQubitId),

    /// Abstract IR resource.
    Resource(DependencyEndpoint),
}

impl SchedulingResource {
    /// Creates a logical-qubit scheduling resource.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit scheduling resource.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Returns the logical qubit if this resource is logical.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(qubit),
            Self::PhysicalQubit(_) | Self::Resource(_) => None,
        }
    }

    /// Returns the physical qubit if this resource is physical.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) | Self::Resource(_) => None,
            Self::PhysicalQubit(qubit) => Some(qubit),
        }
    }
}

/// Read-only statistics about a dependency graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyGraphStats {
    /// Number of registered nodes.
    pub nodes: usize,

    /// Number of stored dependency edges.
    pub dependencies: usize,

    /// Number of required dependencies.
    pub required_dependencies: usize,

    /// Number of conditional dependencies.
    pub conditional_dependencies: usize,

    /// Number of preferred dependencies.
    pub preferred_dependencies: usize,

    /// Number of informational dependencies.
    pub informational_dependencies: usize,

    /// Number of isolated nodes.
    pub isolated_nodes: usize,

    /// Maximum outgoing degree.
    pub maximum_out_degree: usize,

    /// Maximum incoming degree.
    pub maximum_in_degree: usize,
}

impl DependencyGraphStats {
    /// Returns whether the graph contains no nodes.
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

/// Deterministic scheduler dependency graph.
///
/// The graph stores semantic dependency edges and maintains forward and
/// reverse indexes.
///
/// `BTreeMap`/`BTreeSet` are deliberately used so that:
///
/// - iteration is deterministic;
/// - output does not depend on hash seeds;
/// - canonical serialization can consume the graph directly;
/// - testing is reproducible.
///
/// The graph is generic over machine size and contains no hardware-size
/// constants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyGraph {
    nodes: BTreeSet<DependencyEndpoint>,

    /// Outgoing semantic dependencies:
    ///
    /// source -> target dependencies.
    outgoing: BTreeMap<DependencyEndpoint, BTreeSet<DependencyEndpoint>>,

    /// Incoming semantic dependencies:
    ///
    /// target -> source dependencies.
    incoming: BTreeMap<DependencyEndpoint, BTreeSet<DependencyEndpoint>>,

    /// Full dependency records keyed by `(source, target)`.
    ///
    /// Multiple dependency kinds between exactly the same endpoints are
    /// intentionally rejected. If several semantic reasons exist, callers
    /// should create a single canonical dependency or introduce distinct
    /// semantic nodes where appropriate.
    dependencies: BTreeMap<
        (DependencyEndpoint, DependencyEndpoint),
        TemporalDependency,
    >,
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
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
            dependencies: BTreeMap::new(),
        }
    }

    /// Creates a graph with caller-requested collection capacities.
    ///
    /// This method exists only as a convenience for callers that already know
    /// an expected workload. The value is never a semantic machine limit.
    ///
    /// BTree collections do not expose a meaningful capacity reservation API,
    /// so this currently behaves exactly like `new()`.
    #[must_use]
    pub fn with_capacity(_expected_nodes: usize, _expected_dependencies: usize) -> Self {
        Self::new()
    }

    /// Returns the number of registered nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of dependency edges.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns whether an endpoint is registered.
    #[must_use]
    pub fn contains_node(&self, endpoint: DependencyEndpoint) -> bool {
        self.nodes.contains(&endpoint)
    }

    /// Registers a node.
    ///
    /// Registration is idempotent only when the node already exists with the
    /// same identity. A node identity is globally unique within this graph.
    pub fn add_node(
        &mut self,
        endpoint: DependencyEndpoint,
    ) -> DependencyGraphResult<bool> {
        if self.nodes.contains(&endpoint) {
            return Ok(false);
        }

        self.nodes.insert(endpoint);
        self.outgoing.entry(endpoint).or_default();
        self.incoming.entry(endpoint).or_default();

        Ok(true)
    }

    /// Registers multiple nodes.
    ///
    /// Nodes are processed in deterministic endpoint order, independent of
    /// caller iteration order.
    pub fn add_nodes<I>(
        &mut self,
        endpoints: I,
    ) -> DependencyGraphResult<usize>
    where
        I: IntoIterator<Item = DependencyEndpoint>,
    {
        let ordered: BTreeSet<_> = endpoints.into_iter().collect();

        let mut added = 0usize;

        for endpoint in ordered {
            if self.add_node(endpoint)? {
                added += 1;
            }
        }

        Ok(added)
    }

    /// Removes an isolated node.
    ///
    /// A node with dependencies cannot be removed because doing so would
    /// silently destroy scheduling semantics.
    pub fn remove_node(
        &mut self,
        endpoint: DependencyEndpoint,
    ) -> DependencyGraphResult<bool> {
        if !self.nodes.contains(&endpoint) {
            return Ok(false);
        }

        if self
            .outgoing
            .get(&endpoint)
            .map_or(false, |edges| !edges.is_empty())
            || self
                .incoming
                .get(&endpoint)
                .map_or(false, |edges| !edges.is_empty())
        {
            return Err(DependencyGraphError::InvalidDependency {
                message: format!(
                    "cannot remove endpoint `{endpoint}` while dependencies still reference it"
                ),
            });
        }

        self.nodes.remove(&endpoint);
        self.outgoing.remove(&endpoint);
        self.incoming.remove(&endpoint);

        Ok(true)
    }

    /// Adds a semantic dependency.
    ///
    /// Both endpoints must already be registered.
    ///
    /// The graph rejects:
    ///
    /// - self-dependencies;
    /// - dangling endpoints;
    /// - duplicate source/target edges.
    pub fn add_dependency(
        &mut self,
        dependency: TemporalDependency,
    ) -> DependencyGraphResult<bool> {
        let source = dependency.source();
        let target = dependency.target();

        if source == target {
            return Err(DependencyGraphError::InvalidDependency {
                message: "a scheduling dependency cannot connect an endpoint to itself"
                    .to_owned(),
            });
        }

        if !self.nodes.contains(&source) {
            return Err(DependencyGraphError::UnknownEndpoint {
                endpoint: source,
            });
        }

        if !self.nodes.contains(&target) {
            return Err(DependencyGraphError::UnknownEndpoint {
                endpoint: target,
            });
        }

        let key = (source, target);

        if self.dependencies.contains_key(&key) {
            return Err(DependencyGraphError::DuplicateDependency {
                dependency,
            });
        }

        self.dependencies.insert(key, dependency);
        self.outgoing
            .entry(source)
            .or_default()
            .insert(target);
        self.incoming
            .entry(target)
            .or_default()
            .insert(source);

        Ok(true)
    }

    /// Adds several dependencies.
    ///
    /// The operation is transactional: if any dependency fails validation,
    /// the graph is restored to its state before the call.
    pub fn add_dependencies<I>(
        &mut self,
        dependencies: I,
    ) -> DependencyGraphResult<usize>
    where
        I: IntoIterator<Item = TemporalDependency>,
    {
        let snapshot = self.clone();

        let mut added = 0usize;

        for dependency in dependencies {
            match self.add_dependency(dependency) {
                Ok(true) => {
                    added += 1;
                }

                Ok(false) => {}

                Err(error) => {
                    *self = snapshot;
                    return Err(error);
                }
            }
        }

        Ok(added)
    }

    /// Removes a dependency.
    pub fn remove_dependency(
        &mut self,
        source: DependencyEndpoint,
        target: DependencyEndpoint,
    ) -> DependencyGraphResult<bool> {
        let key = (source, target);

        let dependency = match self.dependencies.remove(&key) {
            Some(dependency) => dependency,
            None => {
                return Ok(false);
            }
        };

        if let Some(edges) = self.outgoing.get_mut(&source) {
            edges.remove(&target);
        }

        if let Some(edges) = self.incoming.get_mut(&target) {
            edges.remove(&source);
        }

        debug_assert!(
            !self.dependencies.contains_key(&key),
            "dependency removal must remove the canonical dependency record"
        );

        let _ = dependency;

        Ok(true)
    }

    /// Returns a dependency record.
    #[must_use]
    pub fn dependency(
        &self,
        source: DependencyEndpoint,
        target: DependencyEndpoint,
    ) -> Option<&TemporalDependency> {
        self.dependencies.get(&(source, target))
    }

    /// Returns all registered nodes in deterministic order.
    #[must_use]
    pub fn nodes(&self) -> impl Iterator<Item = DependencyEndpoint> + '_ {
        self.nodes.iter().copied()
    }

    /// Returns all semantic dependencies in deterministic order.
    ///
    /// Ordering is by source endpoint and then target endpoint.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> impl Iterator<Item = &TemporalDependency> + '_ {
        self.dependencies.values()
    }

    /// Returns all outgoing successors of an endpoint.
    #[must_use]
    pub fn successors(
        &self,
        endpoint: DependencyEndpoint,
    ) -> impl Iterator<Item = DependencyEndpoint> + '_ {
        self.outgoing
            .get(&endpoint)
            .into_iter()
            .flat_map(|set| set.iter())
            .copied()
    }

    /// Returns all incoming predecessors of an endpoint.
    #[must_use]
    pub fn predecessors(
        &self,
        endpoint: DependencyEndpoint,
    ) -> impl Iterator<Item = DependencyEndpoint> + '_ {
        self.incoming
            .get(&endpoint)
            .into_iter()
            .flat_map(|set| set.iter())
            .copied()
    }

    /// Returns the number of incoming dependencies.
    #[must_use]
    pub fn predecessor_count(
        &self,
        endpoint: DependencyEndpoint,
    ) -> usize {
        self.incoming
            .get(&endpoint)
            .map_or(0, BTreeSet::len)
    }

    /// Returns the number of outgoing dependencies.
    #[must_use]
    pub fn successor_count(
        &self,
        endpoint: DependencyEndpoint,
    ) -> usize {
        self.outgoing
            .get(&endpoint)
            .map_or(0, BTreeSet::len)
    }

    /// Returns all currently-ready nodes.
    ///
    /// A node is ready when it has no incoming dependencies.
    ///
    /// This method does not mutate the graph and therefore does not constitute
    /// a scheduler.
    #[must_use]
    pub fn roots(&self) -> impl Iterator<Item = DependencyEndpoint> + '_ {
        self.nodes.iter().filter(|endpoint| {
            self.incoming
                .get(endpoint)
                .map_or(true, BTreeSet::is_empty)
        })
        .copied()
    }

    /// Returns all terminal nodes.
    ///
    /// A terminal node has no outgoing dependencies.
    #[must_use]
    pub fn leaves(&self) -> impl Iterator<Item = DependencyEndpoint> + '_ {
        self.nodes.iter().filter(|endpoint| {
            self.outgoing
                .get(endpoint)
                .map_or(true, BTreeSet::is_empty)
        })
        .copied()
    }

    /// Returns dependencies of a specific semantic kind.
    #[must_use]
    pub fn dependencies_of_kind(
        &self,
        kind: DependencyKind,
    ) -> impl Iterator<Item = &TemporalDependency> + '_ {
        self.dependencies
            .values()
            .filter(move |dependency| dependency.kind() == kind)
    }

    /// Returns required dependencies.
    #[must_use]
    pub fn required_dependencies(
        &self,
    ) -> impl Iterator<Item = &TemporalDependency> + '_ {
        self.dependencies.values().filter(|dependency| {
            dependency.strength() == DependencyStrength::Required
        })
    }

    /// Returns whether an edge is a mandatory ordering edge.
    #[must_use]
    pub fn is_required_ordering(
        &self,
        source: DependencyEndpoint,
        target: DependencyEndpoint,
    ) -> bool {
        self.dependencies
            .get(&(source, target))
            .map_or(false, |dependency| {
                dependency.strength() == DependencyStrength::Required
                    && dependency.kind().is_ordering()
            })
    }

    /// Returns a deterministic topological ordering of all nodes.
    ///
    /// This uses Kahn's algorithm.
    ///
    /// All dependency kinds are considered ordering edges here because the
    /// scheduler must not discard a semantic dependency merely because it is
    /// not a conventional `HappensBefore` edge.
    ///
    /// For graphs containing cycles, an explicit error is returned.
    pub fn topological_order(
        &self,
    ) -> DependencyGraphResult<Vec<DependencyEndpoint>> {
        let mut indegree: BTreeMap<DependencyEndpoint, usize> =
            BTreeMap::new();

        for node in &self.nodes {
            indegree.insert(*node, self.predecessor_count(*node));
        }

        let mut ready = BTreeSet::new();

        for (node, degree) in &indegree {
            if *degree == 0 {
                ready.insert(*node);
            }
        }

        let mut result = Vec::with_capacity(self.nodes.len());

        while let Some(node) = ready.pop_first() {
            result.push(node);

            for successor in self.successors(node) {
                let degree = indegree
                    .get_mut(&successor)
                    .ok_or_else(|| {
                        DependencyGraphError::InvariantViolation {
                            message: format!(
                                "successor `{successor}` is missing from indegree index"
                            ),
                        }
                    })?;

                *degree = degree.checked_sub(1).ok_or_else(|| {
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "indegree underflow for endpoint `{successor}`"
                        ),
                    }
                })?;

                if *degree == 0 {
                    ready.insert(successor);
                }
            }
        }

        if result.len() != self.nodes.len() {
            let cycle = self.find_cycle().unwrap_or_default();

            return Err(DependencyGraphError::OrderingCycle { cycle });
        }

        Ok(result)
    }

    /// Finds one deterministic dependency cycle.
    ///
    /// The returned path contains the repeated endpoint at the end.
    pub fn find_cycle(
        &self,
    ) -> Option<Vec<DependencyEndpoint>> {
        #[derive(Clone, Copy)]
        enum Mark {
            Visiting,
            Complete,
        }

        fn visit(
            graph: &DependencyGraph,
            node: DependencyEndpoint,
            marks: &mut BTreeMap<DependencyEndpoint, Mark>,
            stack: &mut Vec<DependencyEndpoint>,
        ) -> Option<Vec<DependencyEndpoint>> {
            marks.insert(node, Mark::Visiting);
            stack.push(node);

            for successor in graph.successors(node) {
                match marks.get(&successor).copied() {
                    None => {
                        if let Some(cycle) =
                            visit(graph, successor, marks, stack)
                        {
                            return Some(cycle);
                        }
                    }

                    Some(Mark::Visiting) => {
                        let position = stack
                            .iter()
                            .position(|endpoint| *endpoint == successor)
                            .unwrap_or(0);

                        let mut cycle =
                            stack[position..].to_vec();

                        cycle.push(successor);

                        return Some(cycle);
                    }

                    Some(Mark::Complete) => {}
                }
            }

            stack.pop();
            marks.insert(node, Mark::Complete);

            None
        }

        let mut marks = BTreeMap::new();
        let mut stack = Vec::new();

        for node in self.nodes.iter().copied() {
            if !marks.contains_key(&node) {
                if let Some(cycle) =
                    visit(self, node, &mut marks, &mut stack)
                {
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// Validates that the dependency graph is acyclic.
    pub fn validate_acyclic_ordering(
        &self,
    ) -> DependencyGraphResult<()> {
        if let Some(cycle) = self.find_cycle() {
            return Err(DependencyGraphError::OrderingCycle { cycle });
        }

        Ok(())
    }

    /// Returns the transitive successors of a node.
    ///
    /// The starting node itself is not included.
    ///
    /// The result is deterministic.
    #[must_use]
    pub fn transitive_successors(
        &self,
        start: DependencyEndpoint,
    ) -> BTreeSet<DependencyEndpoint> {
        if !self.nodes.contains(&start) {
            return BTreeSet::new();
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        for successor in self.successors(start) {
            queue.push_back(successor);
        }

        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            for successor in self.successors(node) {
                if !visited.contains(&successor) {
                    queue.push_back(successor);
                }
            }
        }

        visited
    }

    /// Returns the transitive predecessors of a node.
    ///
    /// The starting node itself is not included.
    #[must_use]
    pub fn transitive_predecessors(
        &self,
        start: DependencyEndpoint,
    ) -> BTreeSet<DependencyEndpoint> {
        if !self.nodes.contains(&start) {
            return BTreeSet::new();
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        for predecessor in self.predecessors(start) {
            queue.push_back(predecessor);
        }

        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            for predecessor in self.predecessors(node) {
                if !visited.contains(&predecessor) {
                    queue.push_back(predecessor);
                }
            }
        }

        visited
    }

    /// Returns the graph statistics.
    #[must_use]
    pub fn stats(&self) -> DependencyGraphStats {
        let mut required_dependencies = 0usize;
        let mut conditional_dependencies = 0usize;
        let mut preferred_dependencies = 0usize;
        let mut informational_dependencies = 0usize;

        let mut maximum_out_degree = 0usize;
        let mut maximum_in_degree = 0usize;
        let mut isolated_nodes = 0usize;

        for node in &self.nodes {
            let out_degree = self.successor_count(*node);
            let in_degree = self.predecessor_count(*node);

            maximum_out_degree =
                maximum_out_degree.max(out_degree);

            maximum_in_degree =
                maximum_in_degree.max(in_degree);

            if out_degree == 0 && in_degree == 0 {
                isolated_nodes += 1;
            }
        }

        for dependency in self.dependencies.values() {
            match dependency.strength() {
                DependencyStrength::Required => {
                    required_dependencies += 1;
                }

                DependencyStrength::Conditional => {
                    conditional_dependencies += 1;
                }

                DependencyStrength::Preferred => {
                    preferred_dependencies += 1;
                }

                DependencyStrength::Informational => {
                    informational_dependencies += 1;
                }
            }
        }

        DependencyGraphStats {
            nodes: self.nodes.len(),
            dependencies: self.dependencies.len(),
            required_dependencies,
            conditional_dependencies,
            preferred_dependencies,
            informational_dependencies,
            isolated_nodes,
            maximum_out_degree,
            maximum_in_degree,
        }
    }

    /// Validates all internal graph invariants.
    ///
    /// This is intentionally stronger than cycle detection.
    pub fn validate(&self) -> DependencyGraphResult<()> {
        for node in &self.nodes {
            if !self.outgoing.contains_key(node) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "node `{node}` has no outgoing index entry"
                        ),
                    },
                );
            }

            if !self.incoming.contains_key(node) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "node `{node}` has no incoming index entry"
                        ),
                    },
                );
            }
        }

        for ((source, target), dependency) in &self.dependencies {
            if !self.nodes.contains(source) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "dependency source `{source}` is not registered"
                        ),
                    },
                );
            }

            if !self.nodes.contains(target) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "dependency target `{target}` is not registered"
                        ),
                    },
                );
            }

            if source == target {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message:
                            "self-dependency exists in graph".to_owned(),
                    },
                );
            }

            if dependency.source() != *source
                || dependency.target() != *target
            {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message:
                            "dependency index key does not match dependency endpoints"
                                .to_owned(),
                    },
                );
            }

            let outgoing = self
                .outgoing
                .get(source)
                .ok_or_else(|| {
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "missing outgoing index for `{source}`"
                        ),
                    }
                })?;

            if !outgoing.contains(target) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "dependency `{source} -> {target}` is missing from outgoing index"
                        ),
                    },
                );
            }

            let incoming = self
                .incoming
                .get(target)
                .ok_or_else(|| {
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "missing incoming index for `{target}`"
                        ),
                    }
                })?;

            if !incoming.contains(source) {
                return Err(
                    DependencyGraphError::InvariantViolation {
                        message: format!(
                            "dependency `{source} -> {target}` is missing from incoming index"
                        ),
                    },
                );
            }
        }

        for (source, targets) in &self.outgoing {
            for target in targets {
                if !self.dependencies.contains_key(&(*source, *target)) {
                    return Err(
                        DependencyGraphError::InvariantViolation {
                            message: format!(
                                "outgoing index contains `{source} -> {target}` without dependency record"
                            ),
                        },
                    );
                }
            }
        }

        for (target, sources) in &self.incoming {
            for source in sources {
                if !self.dependencies.contains_key(&(*source, *target)) {
                    return Err(
                        DependencyGraphError::InvariantViolation {
                            message: format!(
                                "incoming index contains `{source} -> {target}` without dependency record"
                            ),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns a deterministic list of operation endpoints.
    #[must_use]
    pub fn operation_nodes(&self) -> impl Iterator<Item = OperationId> + '_ {
        self.nodes.iter().filter_map(|endpoint| match endpoint {
            DependencyEndpoint::Operation(id) => Some(*id),
            _ => None,
        })
    }

    /// Returns the number of operation nodes.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operation_nodes().count()
    }

    /// Returns whether the graph contains a dependency between two endpoints.
    #[must_use]
    pub fn contains_dependency(
        &self,
        source: DependencyEndpoint,
        target: DependencyEndpoint,
    ) -> bool {
        self.dependencies.contains_key(&(source, target))
    }

    /// Returns a deterministic copy of all edges.
    ///
    /// This is useful for serialization, hashing, diagnostics, and tests.
    #[must_use]
    pub fn edge_list(&self) -> Vec<TemporalDependency> {
        self.dependencies.values().cloned().collect()
    }

    /// Clears all nodes and dependencies.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.outgoing.clear();
        self.incoming.clear();
        self.dependencies.clear();
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a scheduler dependency between two operation IDs.
///
/// This helper keeps scheduler code concise without introducing a second
/// dependency representation.
pub fn operation_dependency(
    source: OperationId,
    target: OperationId,
    kind: DependencyKind,
) -> DependencyGraphResult<TemporalDependency> {
    TemporalDependency::new(
        DependencyEndpoint::Operation(source),
        DependencyEndpoint::Operation(target),
        kind,
    )
    .map_err(|error| DependencyGraphError::InvalidDependency {
        message: error.to_string(),
    })
}

/// Creates a required happens-before dependency between operations.
pub fn happens_before(
    source: OperationId,
    target: OperationId,
) -> DependencyGraphResult<TemporalDependency> {
    operation_dependency(
        source,
        target,
        DependencyKind::HappensBefore,
    )
}

/// Creates a program-order dependency between operations.
pub fn program_order(
    source: OperationId,
    target: OperationId,
) -> DependencyGraphResult<TemporalDependency> {
    operation_dependency(
        source,
        target,
        DependencyKind::ProgramOrder,
    )
}

/// Creates a measurement-feedback dependency between two operations.
///
/// The source operation is normally a measurement-producing operation and the
/// target is normally a classically-controlled quantum operation.
pub fn measurement_feedback(
    source: OperationId,
    target: OperationId,
) -> DependencyGraphResult<TemporalDependency> {
    operation_dependency(
        source,
        target,
        DependencyKind::Measurement,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::OperationId;

    fn operation(value: u64) -> DependencyEndpoint {
        DependencyEndpoint::Operation(OperationId::new(value))
    }

    #[test]
    fn graph_starts_empty() {
        let graph = DependencyGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn node_registration_is_idempotent() {
        let mut graph = DependencyGraph::new();
        let node = operation(1);

        assert_eq!(graph.add_node(node).unwrap(), true);
        assert_eq!(graph.add_node(node).unwrap(), false);

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn dependency_requires_registered_endpoints() {
        let mut graph = DependencyGraph::new();

        let source = operation(1);
        let target = operation(2);

        let dependency =
            TemporalDependency::new(
                source,
                target,
                DependencyKind::HappensBefore,
            )
            .unwrap();

        let error =
            graph.add_dependency(dependency).unwrap_err();

        assert!(matches!(
            error,
            DependencyGraphError::UnknownEndpoint { .. }
        ));
    }

    #[test]
    fn dependency_is_indexed_both_directions() {
        let mut graph = DependencyGraph::new();

        let source = operation(1);
        let target = operation(2);

        graph.add_node(source).unwrap();
        graph.add_node(target).unwrap();

        let dependency =
            TemporalDependency::new(
                source,
                target,
                DependencyKind::HappensBefore,
            )
            .unwrap();

        graph.add_dependency(dependency).unwrap();

        assert_eq!(
            graph.successors(source).collect::<Vec<_>>(),
            vec![target]
        );

        assert_eq!(
            graph.predecessors(target).collect::<Vec<_>>(),
            vec![source]
        );

        assert!(graph.contains_dependency(source, target));
    }

    #[test]
    fn topological_order_is_deterministic() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        graph.add_nodes([c, a, b]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    b,
                    c,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            graph.topological_order().unwrap(),
            vec![a, b, c]
        );
    }

    #[test]
    fn cycle_is_detected() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);

        graph.add_nodes([a, b]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    b,
                    a,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        let cycle = graph.find_cycle();

        assert!(cycle.is_some());

        let cycle = cycle.unwrap();

        assert!(cycle.len() >= 3);
        assert_eq!(cycle.first(), cycle.last());
    }

    #[test]
    fn transitive_successors_work() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        graph.add_nodes([a, b, c]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    b,
                    c,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        let successors =
            graph.transitive_successors(a);

        assert!(successors.contains(&b));
        assert!(successors.contains(&c));
        assert_eq!(successors.len(), 2);
    }

    #[test]
    fn transitive_predecessors_work() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        graph.add_nodes([a, b, c]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    b,
                    c,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        let predecessors =
            graph.transitive_predecessors(c);

        assert!(predecessors.contains(&a));
        assert!(predecessors.contains(&b));
        assert_eq!(predecessors.len(), 2);
    }

    #[test]
    fn isolated_nodes_are_valid() {
        let mut graph = DependencyGraph::new();

        graph.add_node(operation(1)).unwrap();

        let stats = graph.stats();

        assert_eq!(stats.nodes, 1);
        assert_eq!(stats.dependencies, 0);
        assert_eq!(stats.isolated_nodes, 1);
    }

    #[test]
    fn graph_validation_succeeds_for_valid_graph() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);

        graph.add_nodes([a, b]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn removing_dependency_preserves_nodes() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);

        graph.add_nodes([a, b]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        assert!(graph.remove_dependency(a, b).unwrap());

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn operation_helpers_use_canonical_dependency_types() {
        let dependency =
            happens_before(OperationId::new(1), OperationId::new(2))
                .unwrap();

        assert_eq!(
            dependency.source(),
            operation(1)
        );

        assert_eq!(
            dependency.target(),
            operation(2)
        );

        assert_eq!(
            dependency.kind(),
            DependencyKind::HappensBefore
        );
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);

        graph.add_nodes([a, b]).unwrap();

        let dependency =
            TemporalDependency::new(
                a,
                b,
                DependencyKind::HappensBefore,
            )
            .unwrap();

        graph
            .add_dependency(dependency.clone())
            .unwrap();

        let error =
            graph.add_dependency(dependency).unwrap_err();

        assert!(matches!(
            error,
            DependencyGraphError::DuplicateDependency { .. }
        ));
    }

    #[test]
    fn batch_insertion_is_transactional() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        graph.add_nodes([a, b]).unwrap();

        let first =
            TemporalDependency::new(
                a,
                b,
                DependencyKind::HappensBefore,
            )
            .unwrap();

        let invalid =
            TemporalDependency::new(
                b,
                c,
                DependencyKind::HappensBefore,
            )
            .unwrap();

        let result =
            graph.add_dependencies([first, invalid]);

        assert!(result.is_err());
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn roots_and_leaves_are_deterministic() {
        let mut graph = DependencyGraph::new();

        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        graph.add_nodes([c, b, a]).unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    a,
                    b,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        graph
            .add_dependency(
                TemporalDependency::new(
                    b,
                    c,
                    DependencyKind::HappensBefore,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            graph.roots().collect::<Vec<_>>(),
            vec![a]
        );

        assert_eq!(
            graph.leaves().collect::<Vec<_>>(),
            vec![c]
        );
    }
}