//! Zamani Quantum IR — Target Topology
//!
//! Production-grade, deterministic, hardware-independent representation of
//! physical quantum-resource connectivity.
//!
//! # Architectural role
//!
//! This module answers one question:
//!
//! > Which physical quantum resources are structurally connected to which
//! > other physical quantum resources?
//!
//! It intentionally does NOT answer:
//!
//! - which provider owns the resources;
//! - whether a device is online;
//! - whether a resource is calibrated;
//! - how good a coupling is;
//! - which native gate is supported;
//! - how logical qubits are mapped;
//! - how routing is performed;
//! - when operations execute;
//! - how pulses are generated;
//! - how a backend executes an operation;
//! - how a QEC decoder works;
//! - how a simulator represents quantum state.
//!
//! Those responsibilities belong to downstream layers.
//!
//! # Canonical identity boundary
//!
//! Physical qubit identity is owned by:
//!
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//!
//! This module MUST NOT define another physical-qubit identifier.
//!
//! Logical qubit identity remains:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! Logical qubits MUST NOT be inserted into a physical topology.
//!
//! The mapping between logical and physical identities belongs to the mapping
//! and routing subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to any compatible target with sufficient resources and capabilities.
//!
//! Therefore this module contains:
//!
//! - no maximum qubit count;
//! - no maximum topology size;
//! - no vendor-specific topology;
//! - no fixed lattice;
//! - no fixed connectivity class;
//! - no assumptions about superconducting, ion-trap, neutral-atom,
//!   photonic, spin, topological, annealing, or future architectures.
//!
//! A topology is finite because the target hardware instance is finite, not
//! because the IR imposes a finite architectural ceiling.
//!
//! "Unbounded" or future scalability is represented by the absence of a
//! topology-size constant. Actual allocation is bounded only by available
//! memory, host limits, target resources, and explicit compiler policies.
//!
//! # Topology semantics
//!
//! The topology is a graph:
//!
//! ```text
//! physical resource
//!        |
//!        | coupling
//!        v
//! physical resource
//! ```
//!
//! An edge may be:
//!
//! - bidirectional;
//! - directed.
//!
//! Bidirectional means the physical coupling exists in both directions.
//!
//! Directed means the native topology explicitly distinguishes source and
//! target.
//!
//! This module does NOT infer that a directed edge can execute a reversed
//! operation. A routing/compiler layer must make that decision using the
//! target instruction set and capabilities.
//!
//! # Determinism
//!
//! All externally observable ordering is deterministic:
//!
//! - nodes are ordered by `PhysicalQubitId`;
//! - edges are canonically ordered;
//! - adjacency lists are sorted;
//! - breadth-first traversal is deterministic;
//! - shortest-path tie-breaking is deterministic;
//! - connected-component traversal is deterministic;
//! - canonical serialization is deterministic.
//!
//! `HashMap` is deliberately not used for semantic topology storage.
//!
//! # Scalability
//!
//! The representation is sparse.
//!
//! Isolated resources consume node storage only.
//!
//! A topology containing N resources and E couplings stores O(N + E)
//! structural information rather than an N x N adjacency matrix.
//!
//! This is essential for large sparse machines.
//!
//! Dense topologies are represented by explicitly adding their edges. The IR
//! does not special-case small, medium, or large machines.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no external dependencies
//! - no unsafe code
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration
//!
//! ```text
//! quantum::ir::qubit
//!         |
//!         v
//! resources::topology
//!         |
//!         +---- resources::capability
//!         +---- resources::resource
//!         |
//!         +---- routing
//!         +---- mapping
//!         +---- scheduling
//!         +---- hardware
//!         +---- benchmarking
//! ```
//!
//! This module may be consumed by those systems.
//!
//! It must never depend on them.
//!
//! # Serialization
//!
//! The canonical IR serialization subsystem owns the final wire format.
//!
//! This file therefore exposes deterministic semantic iteration and a
//! canonical textual representation useful to serialization/hashing layers,
//! but does not introduce a competing external serialization format.
//!
//! # Hashing
//!
//! `Hash` implementations are semantic only.
//!
//! Canonical IR cryptographic hashing remains owned by the IR hashing layer.
//!
//! A deterministic `canonical_bytes()` representation is provided so that
//! higher layers can feed exactly the same semantic bytes into their chosen
//! canonical hash implementation.
//!
//! # Validation
//!
//! Topology invariants are checked eagerly during mutation and can also be
//! checked explicitly with `validate()`.
//!
//! Invalid topology state is never silently accepted.
//!
//! # Ownership contract
//!
//! This file owns:
//!
//! - topology nodes;
//! - topology edges;
//! - edge directionality;
//! - deterministic adjacency;
//! - topology construction;
//! - topology mutation;
//! - structural validation;
//! - structural connectivity queries;
//! - deterministic shortest paths;
//! - connected components;
//! - structural statistics;
//! - canonical topology representation.
//!
//! It does NOT own:
//!
//! - physical device identity beyond `PhysicalQubitId`;
//! - calibration;
//! - gate support;
//! - routing policy;
//! - mapping policy;
//! - scheduling;
//! - execution;
//! - provider APIs.
//!
//! # Important distinction
//!
//! ```text
//! topology
//!     = what is physically connected
//!
//! capability
//!     = what the target can do
//!
//! instruction set
//!     = which operations are native
//!
//! mapping
//!     = where logical resources are placed
//!
//! routing
//!     = how logical interactions are made executable
//!
//! scheduling
//!     = when operations execute
//! ```
//!
//! Keeping these concepts separate is mandatory for Zamani's
//! "write once, scale anywhere" architecture.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use super::super::qubit::PhysicalQubitId;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier.
///
/// This is intentionally local to the topology semantic model. It is not a
/// replacement for the canonical Quantum IR version.
pub const TOPOLOGY_SCHEMA_ID: &str = "zamani.quantum.ir.resources.topology";

/// Semantic schema revision.
///
/// Breaking semantic changes must also be reflected through the canonical IR
/// version/migration system.
pub const TOPOLOGY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Connectivity
// =============================================================================

/// Directionality of a physical coupling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Connectivity {
    /// The physical coupling exists in both directions.
    ///
    /// This describes topology only. Native instruction support is owned by
    /// the target instruction-set/capability layer.
    Bidirectional,

    /// The physical coupling has an explicit native source and target.
    Directed,
}

impl Connectivity {
    /// Returns the stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bidirectional => "bidirectional",
            Self::Directed => "directed",
        }
    }

    /// Returns whether this connectivity is directed.
    #[must_use]
    pub const fn is_directed(self) -> bool {
        matches!(self, Self::Directed)
    }

    /// Returns whether this connectivity is bidirectional.
    #[must_use]
    pub const fn is_bidirectional(self) -> bool {
        matches!(self, Self::Bidirectional)
    }
}

// =============================================================================
// Path semantics
// =============================================================================

/// Controls how graph traversal interprets topology directionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathMode {
    /// Respect the native direction of directed couplings.
    ///
    /// Bidirectional couplings remain traversable in both directions.
    Directed,

    /// Treat every physical coupling as an undirected physical adjacency.
    ///
    /// This is useful for physical connectivity analysis and distance
    /// estimation. It MUST NOT be interpreted as proof that a reversed
    /// operation is natively executable.
    Undirected,
}

impl Default for PathMode {
    fn default() -> Self {
        Self::Directed
    }
}

impl PathMode {
    /// Returns the stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Directed => "directed",
            Self::Undirected => "undirected",
        }
    }
}

// =============================================================================
// Topology edge
// =============================================================================

/// A structural coupling between two physical quantum resources.
///
/// The endpoints are canonical IR physical identities.
///
/// Calibration, fidelity, timing, crosstalk, native gates and other target
/// properties are deliberately absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TopologyEdge {
    /// Source physical resource.
    pub source: PhysicalQubitId,

    /// Target physical resource.
    pub target: PhysicalQubitId,

    /// Connectivity semantics.
    pub connectivity: Connectivity,
}

impl TopologyEdge {
    /// Creates a bidirectional edge.
    ///
    /// Endpoint order is canonicalized so that the same undirected coupling
    /// has exactly one representation.
    pub fn bidirectional(
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, TopologyError> {
        if source == target {
            return Err(TopologyError::SelfCoupling { qubit: source });
        }

        let (source, target) = canonical_pair(source, target);

        Ok(Self {
            source,
            target,
            connectivity: Connectivity::Bidirectional,
        })
    }

    /// Creates a directed edge.
    ///
    /// Direction is preserved exactly.
    pub fn directed(
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, TopologyError> {
        if source == target {
            return Err(TopologyError::SelfCoupling { qubit: source });
        }

        Ok(Self {
            source,
            target,
            connectivity: Connectivity::Directed,
        })
    }

    /// Returns the endpoint pair independent of direction.
    #[must_use]
    pub fn undirected_pair(self) -> (PhysicalQubitId, PhysicalQubitId) {
        canonical_pair(self.source, self.target)
    }

    /// Returns whether this edge contains the supplied physical resource.
    #[must_use]
    pub fn contains(self, qubit: PhysicalQubitId) -> bool {
        self.source == qubit || self.target == qubit
    }

    /// Returns the opposite endpoint when the supplied qubit is incident.
    #[must_use]
    pub fn opposite(self, qubit: PhysicalQubitId) -> Option<PhysicalQubitId> {
        if self.source == qubit {
            Some(self.target)
        } else if self.target == qubit {
            Some(self.source)
        } else {
            None
        }
    }

    /// Returns whether this edge permits traversal from source to target
    /// under the requested path mode.
    #[must_use]
    pub fn permits(
        self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        mode: PathMode,
    ) -> bool {
        match mode {
            PathMode::Directed => match self.connectivity {
                Connectivity::Directed => {
                    self.source == source && self.target == target
                }
                Connectivity::Bidirectional => {
                    (self.source == source && self.target == target)
                        || (self.source == target && self.target == source)
                }
            },

            PathMode::Undirected => {
                self.undirected_pair() == canonical_pair(source, target)
            }
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Structural topology errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    /// A topology cannot be created with zero nodes when a concrete topology
    /// constructor requires at least one node.
    ZeroResources,

    /// Compatibility alias for callers that use qubit terminology.
    ZeroQubits,

    /// The requested physical resource is not present.
    UnknownResource {
        /// Missing resource.
        resource: PhysicalQubitId,
    },

    /// Compatibility alias using qubit terminology.
    UnknownQubit {
        /// Missing physical qubit.
        qubit: PhysicalQubitId,
    },

    /// A resource cannot be connected to itself.
    SelfCoupling {
        /// Invalid resource.
        qubit: PhysicalQubitId,
    },

    /// The exact edge already exists.
    DuplicateEdge {
        /// Existing source.
        source: PhysicalQubitId,

        /// Existing target.
        target: PhysicalQubitId,

        /// Existing connectivity.
        connectivity: Connectivity,
    },

    /// A bidirectional edge conflicts with an existing directed representation.
    ConflictingConnectivity {
        /// First endpoint.
        source: PhysicalQubitId,

        /// Second endpoint.
        target: PhysicalQubitId,
    },

    /// The topology contains an invariant violation.
    InvalidTopology {
        /// Human-readable diagnostic.
        message: String,
    },

    /// No path exists under the requested path semantics.
    NoPath {
        /// Source resource.
        source: PhysicalQubitId,

        /// Target resource.
        target: PhysicalQubitId,

        /// Traversal semantics.
        mode: PathMode,
    },

    /// Numeric capacity/collection operation overflowed.
    NumericOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroResources | Self::ZeroQubits => {
                formatter.write_str(
                    "a concrete topology requires at least one physical resource",
                )
            }

            Self::UnknownResource { resource } => {
                write!(formatter, "unknown physical resource {resource}")
            }

            Self::UnknownQubit { qubit } => {
                write!(formatter, "unknown physical qubit {qubit}")
            }

            Self::SelfCoupling { qubit } => {
                write!(formatter, "physical resource {qubit} cannot couple to itself")
            }

            Self::DuplicateEdge {
                source,
                target,
                connectivity,
            } => {
                write!(
                    formatter,
                    "duplicate {connectivity:?} topology edge {source} -> {target}"
                )
            }

            Self::ConflictingConnectivity { source, target } => {
                write!(
                    formatter,
                    "conflicting topology connectivity for physical pair {source} <-> {target}"
                )
            }

            Self::InvalidTopology { message } => {
                write!(formatter, "invalid topology: {message}")
            }

            Self::NoPath {
                source,
                target,
                mode,
            } => {
                write!(
                    formatter,
                    "no {mode:?} topology path exists from {source} to {target}"
                )
            }

            Self::NumericOverflow { operation } => {
                write!(
                    formatter,
                    "numeric overflow while performing topology operation {operation}"
                )
            }
        }
    }
}

impl std::error::Error for TopologyError {}

// =============================================================================
// Statistics
// =============================================================================

/// Structural topology statistics.
///
/// These values describe graph structure only.
///
/// They do NOT describe:
///
/// - gate fidelity;
/// - error rates;
/// - calibration;
/// - latency;
/// - crosstalk;
/// - device health;
/// - throughput.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopologyStatistics {
    /// Number of physical resources.
    pub resource_count: usize,

    /// Number of stored coupling edges.
    pub edge_count: usize,

    /// Number of directed edges.
    pub directed_edge_count: usize,

    /// Number of bidirectional edges.
    pub bidirectional_edge_count: usize,

    /// Number of resources with at least one physical neighbour.
    pub connected_resource_count: usize,

    /// Number of weakly connected components.
    pub connected_components: usize,

    /// Minimum undirected degree.
    pub minimum_degree: usize,

    /// Maximum undirected degree.
    pub maximum_degree: usize,

    /// Average undirected degree.
    pub average_degree: f64,

    /// Undirected graph density in the range `[0, 1]`.
    pub undirected_density: f64,

    /// Whether the physical graph is weakly connected.
    pub is_connected: bool,
}

impl TopologyStatistics {
    /// Returns whether all resources belong to one weakly connected component.
    #[must_use]
    pub const fn is_fully_connected(self) -> bool {
        self.is_connected
    }
}

// =============================================================================
// Topology
// =============================================================================

/// Canonical sparse physical topology.
///
/// # Representation
///
/// Nodes are stored as a deterministic ordered set.
///
/// Edges are stored as a deterministic ordered set.
///
/// Adjacency maps are materialized for efficient downstream routing and
/// connectivity queries.
///
/// # Complexity
///
/// Let:
///
/// - N = number of physical resources;
/// - E = number of topology edges.
///
/// Construction and mutation are O(log N + log E) per ordered-set update,
/// with adjacency maintenance also O(log N).
///
/// BFS shortest-path operations are O(N + E) in the reachable subgraph.
///
/// Storage is O(N + E).
///
/// No dense N x N matrix is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topology {
    nodes: BTreeSet<PhysicalQubitId>,

    edges: BTreeSet<TopologyEdge>,

    /// Outgoing native adjacency.
    outgoing: BTreeMap<PhysicalQubitId, BTreeSet<PhysicalQubitId>>,

    /// Incoming native adjacency.
    incoming: BTreeMap<PhysicalQubitId, BTreeSet<PhysicalQubitId>>,

    /// Physical adjacency independent of direction.
    undirected: BTreeMap<PhysicalQubitId, BTreeSet<PhysicalQubitId>>,
}

impl Default for Topology {
    fn default() -> Self {
        Self::empty()
    }
}

impl Topology {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty topology.
    ///
    /// An empty topology is useful while incrementally discovering or
    /// constructing a target. `validate()` accepts it as a valid structural
    /// value because there is no invalid node or edge.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            nodes: BTreeSet::new(),
            edges: BTreeSet::new(),
            outgoing: BTreeMap::new(),
            incoming: BTreeMap::new(),
            undirected: BTreeMap::new(),
        }
    }

    /// Creates a topology containing one or more isolated physical resources.
    ///
    /// No couplings are added.
    pub fn from_nodes<I>(nodes: I) -> Result<Self, TopologyError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        let mut topology = Self::empty();

        for node in nodes {
            topology.add_node(node);
        }

        topology.validate()?;

        Ok(topology)
    }

    /// Creates a topology from nodes and edges.
    ///
    /// All endpoints must already be present in `nodes`.
    pub fn from_parts<I, J>(
        nodes: I,
        edges: J,
    ) -> Result<Self, TopologyError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
        J: IntoIterator<Item = TopologyEdge>,
    {
        let mut topology = Self::from_nodes(nodes)?;

        for edge in edges {
            topology.add_edge(edge)?;
        }

        topology.validate()?;

        Ok(topology)
    }

    /// Creates a topology from a node range.
    ///
    /// This is a convenience for dense integer physical identifiers and does
    /// not impose a machine-size limit.
    ///
    /// `start..end` follows normal Rust half-open range semantics.
    pub fn from_range(
        start: usize,
        end: usize,
    ) -> Result<Self, TopologyError> {
        if start > end {
            return Err(TopologyError::InvalidTopology {
                message: format!(
                    "invalid physical-qubit range: start {start} exceeds end {end}"
                ),
            });
        }

        let mut topology = Self::empty();

        for index in start..end {
            topology.add_node(PhysicalQubitId::new(index));
        }

        topology.validate()?;

        Ok(topology)
    }

    // =========================================================================
    // Node management
    // =========================================================================

    /// Adds a physical resource if it does not already exist.
    ///
    /// Re-adding an existing resource is idempotent.
    pub fn add_node(&mut self, node: PhysicalQubitId) -> bool {
        if !self.nodes.insert(node) {
            return false;
        }

        self.outgoing.insert(node, BTreeSet::new());
        self.incoming.insert(node, BTreeSet::new());
        self.undirected.insert(node, BTreeSet::new());

        true
    }

    /// Removes a physical resource and all incident couplings.
    ///
    /// Returns `true` when the resource existed.
    pub fn remove_node(
        &mut self,
        node: PhysicalQubitId,
    ) -> bool {
        if !self.nodes.remove(&node) {
            return false;
        }

        let neighbours = self
            .undirected
            .get(&node)
            .cloned()
            .unwrap_or_default();

        for neighbour in neighbours {
            if let Some(adjacency) = self.undirected.get_mut(&neighbour) {
                adjacency.remove(&node);
            }

            if let Some(adjacency) = self.outgoing.get_mut(&neighbour) {
                adjacency.remove(&node);
            }

            if let Some(adjacency) = self.incoming.get_mut(&neighbour) {
                adjacency.remove(&node);
            }
        }

        self.outgoing.remove(&node);
        self.incoming.remove(&node);
        self.undirected.remove(&node);

        self.edges.retain(|edge| !edge.contains(node));

        // Rebuild the directional adjacency for remaining nodes because a
        // removed node may have been present in either endpoint position.
        self.rebuild_adjacency();

        true
    }

    /// Returns whether a physical resource exists.
    #[must_use]
    pub fn contains_node(&self, node: PhysicalQubitId) -> bool {
        self.nodes.contains(&node)
    }

    /// Returns the number of physical resources.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the topology contains no physical resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns deterministic physical-resource iteration.
    pub fn nodes(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.nodes.iter().copied()
    }

    // =========================================================================
    // Edge management
    // =========================================================================

    /// Adds a topology edge.
    ///
    /// Both endpoints must already exist.
    ///
    /// A directed edge `a -> b` and a directed edge `b -> a` may coexist.
    /// They represent two explicitly directed physical connections.
    ///
    /// A bidirectional edge cannot coexist with either directed representation
    /// for the same unordered physical pair because that would create
    /// ambiguous topology semantics.
    pub fn add_edge(
        &mut self,
        edge: TopologyEdge,
    ) -> Result<(), TopologyError> {
        self.require_node(edge.source)?;
        self.require_node(edge.target)?;

        if edge.source == edge.target {
            return Err(TopologyError::SelfCoupling {
                qubit: edge.source,
            });
        }

        if self.edges.contains(&edge) {
            return Err(TopologyError::DuplicateEdge {
                source: edge.source,
                target: edge.target,
                connectivity: edge.connectivity,
            });
        }

        let pair = edge.undirected_pair();

        if edge.connectivity == Connectivity::Bidirectional {
            let conflicting = self.edges.iter().any(|existing| {
                existing.undirected_pair() == pair
                    && existing.connectivity == Connectivity::Directed
            });

            if conflicting {
                return Err(TopologyError::ConflictingConnectivity {
                    source: pair.0,
                    target: pair.1,
                });
            }
        } else {
            let conflicting = self.edges.iter().any(|existing| {
                existing.undirected_pair() == pair
                    && existing.connectivity == Connectivity::Bidirectional
            });

            if conflicting {
                return Err(TopologyError::ConflictingConnectivity {
                    source: pair.0,
                    target: pair.1,
                });
            }
        }

        self.edges.insert(edge);
        self.index_edge(edge);

        Ok(())
    }

    /// Removes an exact topology edge.
    ///
    /// Returns `true` when the edge existed.
    pub fn remove_edge(
        &mut self,
        edge: TopologyEdge,
    ) -> bool {
        if !self.edges.remove(&edge) {
            return false;
        }

        self.rebuild_adjacency();
        true
    }

    /// Removes all edges between two physical resources.
    ///
    /// This removes both directions when both directed edges exist.
    ///
    /// Returns the number of removed edges.
    pub fn remove_connection(
        &mut self,
        first: PhysicalQubitId,
        second: PhysicalQubitId,
    ) -> usize {
        let pair = canonical_pair(first, second);

        let before = self.edges.len();

        self.edges
            .retain(|edge| edge.undirected_pair() != pair);

        self.rebuild_adjacency();

        before.saturating_sub(self.edges.len())
    }

    /// Returns deterministic edge iteration.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = TopologyEdge> + '_ {
        self.edges.iter().copied()
    }

    /// Returns the number of topology edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether an exact edge exists.
    #[must_use]
    pub fn contains_edge(&self, edge: TopologyEdge) -> bool {
        self.edges.contains(&edge)
    }

    /// Returns whether any physical coupling exists between two resources,
    /// ignoring direction.
    #[must_use]
    pub fn connected(
        &self,
        first: PhysicalQubitId,
        second: PhysicalQubitId,
    ) -> bool {
        if first == second {
            return self.nodes.contains(&first);
        }

        self.undirected
            .get(&first)
            .map_or(false, |adjacency| adjacency.contains(&second))
    }

    /// Returns whether the native topology permits traversal from `source`
    /// to `target`.
    #[must_use]
    pub fn permits(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        mode: PathMode,
    ) -> bool {
        match mode {
            PathMode::Directed => self
                .outgoing
                .get(&source)
                .map_or(false, |adjacency| adjacency.contains(&target)),

            PathMode::Undirected => self
                .undirected
                .get(&source)
                .map_or(false, |adjacency| adjacency.contains(&target)),
        }
    }

    // =========================================================================
    // Adjacency
    // =========================================================================

    /// Returns deterministic native outgoing neighbours.
    pub fn outgoing_neighbors(
        &self,
        source: PhysicalQubitId,
    ) -> Result<Vec<PhysicalQubitId>, TopologyError> {
        self.require_node(source)?;

        Ok(self
            .outgoing
            .get(&source)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default())
    }

    /// Returns deterministic native incoming neighbours.
    pub fn incoming_neighbors(
        &self,
        target: PhysicalQubitId,
    ) -> Result<Vec<PhysicalQubitId>, TopologyError> {
        self.require_node(target)?;

        Ok(self
            .incoming
            .get(&target)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default())
    }

    /// Returns deterministic physical neighbours ignoring direction.
    pub fn neighbors(
        &self,
        node: PhysicalQubitId,
    ) -> Result<Vec<PhysicalQubitId>, TopologyError> {
        self.require_node(node)?;

        Ok(self
            .undirected
            .get(&node)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default())
    }

    /// Returns the undirected degree of a resource.
    pub fn degree(
        &self,
        node: PhysicalQubitId,
    ) -> Result<usize, TopologyError> {
        self.require_node(node)?;

        Ok(self
            .undirected
            .get(&node)
            .map_or(0, BTreeSet::len))
    }

    // =========================================================================
    // Connectivity analysis
    // =========================================================================

    /// Returns whether all physical resources belong to one weakly connected
    /// component.
    ///
    /// An empty topology is considered vacuously connected by this predicate.
    /// Callers that require a non-empty target should enforce that policy
    /// separately.
    #[must_use]
    pub fn is_connected(&self) -> bool {
        if self.nodes.len() <= 1 {
            return true;
        }

        self.connected_components() == 1
    }

    /// Returns the number of weakly connected components.
    ///
    /// Direction is ignored because this method answers physical connectivity,
    /// not directed execution reachability.
    #[must_use]
    pub fn connected_components(&self) -> usize {
        let mut visited = BTreeSet::new();
        let mut components = 0usize;

        for &start in &self.nodes {
            if visited.contains(&start) {
                continue;
            }

            components = components.saturating_add(1);

            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited.insert(start);

            while let Some(current) = queue.pop_front() {
                if let Some(neighbours) = self.undirected.get(&current) {
                    for &next in neighbours {
                        if visited.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }
        }

        components
    }

    /// Returns the weakly connected component containing `start`.
    pub fn component_containing(
        &self,
        start: PhysicalQubitId,
    ) -> Result<Vec<PhysicalQubitId>, TopologyError> {
        self.require_node(start)?;

        let mut result = Vec::new();
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            result.push(current);

            if let Some(neighbours) = self.undirected.get(&current) {
                for &next in neighbours {
                    if visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }
        }

        result.sort_unstable();

        Ok(result)
    }

    /// Returns deterministic weakly connected components.
    ///
    /// Components are ordered by their smallest physical-qubit identifier.
    pub fn components(
        &self,
    ) -> Vec<Vec<PhysicalQubitId>> {
        let mut visited = BTreeSet::new();
        let mut components = Vec::new();

        for &start in &self.nodes {
            if visited.contains(&start) {
                continue;
            }

            let mut component = Vec::new();
            let mut queue = VecDeque::new();

            visited.insert(start);
            queue.push_back(start);

            while let Some(current) = queue.pop_front() {
                component.push(current);

                if let Some(neighbours) = self.undirected.get(&current) {
                    for &next in neighbours {
                        if visited.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }

            component.sort_unstable();
            components.push(component);
        }

        components
    }

    // =========================================================================
    // Shortest paths
    // =========================================================================

    /// Returns the deterministic shortest physical path between two resources.
    ///
    /// The returned vector includes both endpoints.
    ///
    /// If `source == target`, the result is `[source]`.
    ///
    /// BFS is appropriate because topology edges are unweighted here.
    ///
    /// Calibration-dependent routing costs must be handled by the routing
    /// subsystem, not by this IR topology model.
    pub fn shortest_path(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        mode: PathMode,
    ) -> Result<Vec<PhysicalQubitId>, TopologyError> {
        self.require_node(source)?;
        self.require_node(target)?;

        if source == target {
            return Ok(vec![source]);
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut predecessor: BTreeMap<
            PhysicalQubitId,
            PhysicalQubitId,
        > = BTreeMap::new();

        visited.insert(source);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let neighbours = match mode {
                PathMode::Directed => self.outgoing.get(&current),
                PathMode::Undirected => self.undirected.get(&current),
            };

            let Some(neighbours) = neighbours else {
                continue;
            };

            for &next in neighbours {
                if !visited.insert(next) {
                    continue;
                }

                predecessor.insert(next, current);

                if next == target {
                    return Ok(reconstruct_path(
                        source,
                        target,
                        &predecessor,
                    ));
                }

                queue.push_back(next);
            }
        }

        Err(TopologyError::NoPath {
            source,
            target,
            mode,
        })
    }

    /// Returns the number of edges in the deterministic shortest path.
    pub fn distance(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        mode: PathMode,
    ) -> Result<usize, TopologyError> {
        let path = self.shortest_path(source, target, mode)?;

        Ok(path.len().saturating_sub(1))
    }

    /// Returns whether a path exists under the requested traversal semantics.
    pub fn has_path(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        mode: PathMode,
    ) -> Result<bool, TopologyError> {
        self.require_node(source)?;
        self.require_node(target)?;

        if source == target {
            return Ok(true);
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();

        visited.insert(source);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let neighbours = match mode {
                PathMode::Directed => self.outgoing.get(&current),
                PathMode::Undirected => self.undirected.get(&current),
            };

            let Some(neighbours) = neighbours else {
                continue;
            };

            for &next in neighbours {
                if next == target {
                    return Ok(true);
                }

                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        Ok(false)
    }

    // =========================================================================
    // Statistics
    // =========================================================================

    /// Calculates deterministic structural statistics.
    ///
    /// This operation is O(N + E) apart from ordered-set iteration.
    ///
    /// Expensive all-pairs distance analysis is intentionally NOT performed
    /// here. Use `diameter()` or `average_shortest_path()` explicitly when
    /// required.
    #[must_use]
    pub fn statistics(&self) -> TopologyStatistics {
        let resource_count = self.nodes.len();
        let edge_count = self.edges.len();

        let directed_edge_count = self
            .edges
            .iter()
            .filter(|edge| edge.connectivity == Connectivity::Directed)
            .count();

        let bidirectional_edge_count = edge_count
            .saturating_sub(directed_edge_count);

        let connected_resource_count = self
            .nodes
            .iter()
            .filter(|node| {
                self.undirected
                    .get(node)
                    .map_or(false, |set| !set.is_empty())
            })
            .count();

        let connected_components = self.connected_components();

        let mut minimum_degree = 0usize;
        let mut maximum_degree = 0usize;
        let mut degree_sum = 0usize;

        if resource_count > 0 {
            minimum_degree = usize::MAX;

            for node in &self.nodes {
                let degree = self
                    .undirected
                    .get(node)
                    .map_or(0, BTreeSet::len);

                minimum_degree = minimum_degree.min(degree);
                maximum_degree = maximum_degree.max(degree);
                degree_sum = degree_sum.saturating_add(degree);
            }
        }

        if resource_count == 0 {
            minimum_degree = 0;
        }

        let average_degree = if resource_count == 0 {
            0.0
        } else {
            degree_sum as f64 / resource_count as f64
        };

        let possible_pairs = resource_count
            .checked_mul(resource_count.saturating_sub(1))
            .map(|value| value / 2)
            .unwrap_or(usize::MAX);

        let unique_undirected_pairs = self
            .edges
            .iter()
            .map(|edge| edge.undirected_pair())
            .collect::<BTreeSet<_>>()
            .len();

        let undirected_density = if possible_pairs == 0 {
            0.0
        } else {
            unique_undirected_pairs as f64 / possible_pairs as f64
        };

        TopologyStatistics {
            resource_count,
            edge_count,
            directed_edge_count,
            bidirectional_edge_count,
            connected_resource_count,
            connected_components,
            minimum_degree,
            maximum_degree,
            average_degree,
            undirected_density,
            is_connected: resource_count <= 1
                || connected_components == 1,
        }
    }

    /// Returns the undirected graph diameter.
    ///
    /// Returns `None` when the topology is disconnected.
    ///
    /// Complexity is O(N * (N + E)) in the worst case.
    ///
    /// This operation is deliberately explicit so merely constructing a large
    /// topology does not accidentally trigger an all-pairs traversal.
    pub fn diameter(&self) -> Result<Option<usize>, TopologyError> {
        if self.nodes.len() <= 1 {
            return Ok(Some(0));
        }

        if !self.is_connected() {
            return Ok(None);
        }

        let mut diameter = 0usize;

        for &source in &self.nodes {
            let distances = self.bfs_distances(
                source,
                PathMode::Undirected,
            )?;

            if let Some(maximum) = distances.values().copied().max() {
                diameter = diameter.max(maximum);
            }
        }

        Ok(Some(diameter))
    }

    /// Returns the average undirected shortest-path distance over all
    /// unordered reachable pairs.
    ///
    /// Returns `None` for a topology with fewer than two resources or when no
    /// pair is reachable.
    ///
    /// Complexity is O(N * (N + E)).
    pub fn average_shortest_path(
        &self,
    ) -> Result<Option<f64>, TopologyError> {
        if self.nodes.len() < 2 {
            return Ok(None);
        }

        let mut total_distance = 0u128;
        let mut pair_count = 0u128;

        for &source in &self.nodes {
            let distances = self.bfs_distances(
                source,
                PathMode::Undirected,
            )?;

            for (&target, &distance) in &distances {
                if target > source {
                    total_distance = total_distance
                        .checked_add(distance as u128)
                        .ok_or(TopologyError::NumericOverflow {
                            operation: "average_shortest_path",
                        })?;

                    pair_count = pair_count
                        .checked_add(1)
                        .ok_or(TopologyError::NumericOverflow {
                            operation: "average_shortest_path",
                        })?;
                }
            }
        }

        if pair_count == 0 {
            Ok(None)
        } else {
            Ok(Some(
                total_distance as f64 / pair_count as f64
            ))
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates every topology invariant.
    ///
    /// This method is intentionally public so deserialization, provider
    /// discovery and compatibility layers can validate reconstructed topology
    /// values before accepting them.
    pub fn validate(&self) -> Result<(), TopologyError> {
        if self.nodes.len() != self.outgoing.len()
            || self.nodes.len() != self.incoming.len()
            || self.nodes.len() != self.undirected.len()
        {
            return Err(TopologyError::InvalidTopology {
                message: String::from(
                    "node and adjacency-map cardinalities differ",
                ),
            });
        }

        for node in &self.nodes {
            if !self.outgoing.contains_key(node) {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "missing outgoing adjacency for {node}"
                    ),
                });
            }

            if !self.incoming.contains_key(node) {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "missing incoming adjacency for {node}"
                    ),
                });
            }

            if !self.undirected.contains_key(node) {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "missing undirected adjacency for {node}"
                    ),
                });
            }
        }

        for edge in &self.edges {
            if edge.source == edge.target {
                return Err(TopologyError::SelfCoupling {
                    qubit: edge.source,
                });
            }

            if !self.nodes.contains(&edge.source) {
                return Err(TopologyError::UnknownResource {
                    resource: edge.source,
                });
            }

            if !self.nodes.contains(&edge.target) {
                return Err(TopologyError::UnknownResource {
                    resource: edge.target,
                });
            }
        }

        for edge in &self.edges {
            let pair = edge.undirected_pair();

            for other in &self.edges {
                if edge == other {
                    continue;
                }

                if edge.connectivity == Connectivity::Bidirectional
                    && other.connectivity == Connectivity::Directed
                    && pair == other.undirected_pair()
                {
                    return Err(TopologyError::ConflictingConnectivity {
                        source: pair.0,
                        target: pair.1,
                    });
                }
            }
        }

        let mut expected_outgoing: BTreeMap<
            PhysicalQubitId,
            BTreeSet<PhysicalQubitId>,
        > = BTreeMap::new();

        let mut expected_incoming: BTreeMap<
            PhysicalQubitId,
            BTreeSet<PhysicalQubitId>,
        > = BTreeMap::new();

        let mut expected_undirected: BTreeMap<
            PhysicalQubitId,
            BTreeSet<PhysicalQubitId>,
        > = BTreeMap::new();

        for &node in &self.nodes {
            expected_outgoing.insert(node, BTreeSet::new());
            expected_incoming.insert(node, BTreeSet::new());
            expected_undirected.insert(node, BTreeSet::new());
        }

        for edge in &self.edges {
            index_edge_into(
                &mut expected_outgoing,
                &mut expected_incoming,
                &mut expected_undirected,
                edge,
            )?;
        }

        if self.outgoing != expected_outgoing {
            return Err(TopologyError::InvalidTopology {
                message: String::from(
                    "outgoing adjacency does not match edge set",
                ),
            });
        }

        if self.incoming != expected_incoming {
            return Err(TopologyError::InvalidTopology {
                message: String::from(
                    "incoming adjacency does not match edge set",
                ),
            });
        }

        if self.undirected != expected_undirected {
            return Err(TopologyError::InvalidTopology {
                message: String::from(
                    "undirected adjacency does not match edge set",
                ),
            });
        }

        Ok(())
    }

    // =========================================================================
    // Canonical representation
    // =========================================================================

    /// Returns a deterministic canonical byte representation.
    ///
    /// This is intentionally a semantic representation rather than a general
    /// serialization format.
    ///
    /// The canonical IR serialization subsystem may use these bytes as part of
    /// its own canonical encoding or hashing pipeline.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        append_len_prefixed_str(
            &mut bytes,
            TOPOLOGY_SCHEMA_ID,
        );

        bytes.extend_from_slice(
            &TOPOLOGY_SCHEMA_VERSION.to_le_bytes(),
        );

        append_u64(
            &mut bytes,
            self.nodes.len() as u64,
        );

        for node in &self.nodes {
            append_u64(
                &mut bytes,
                node.index() as u64,
            );
        }

        append_u64(
            &mut bytes,
            self.edges.len() as u64,
        );

        for edge in &self.edges {
            append_u64(
                &mut bytes,
                edge.source.index() as u64,
            );

            append_u64(
                &mut bytes,
                edge.target.index() as u64,
            );

            bytes.push(match edge.connectivity {
                Connectivity::Bidirectional => 0,
                Connectivity::Directed => 1,
            });
        }

        bytes
    }

    /// Returns a deterministic hexadecimal fingerprint source.
    ///
    /// This is NOT intended to replace the canonical cryptographic hashing
    /// subsystem. It is a stable, dependency-free semantic digest useful for
    /// diagnostics and tests.
    #[must_use]
    pub fn semantic_fingerprint(&self) -> u64 {
        // FNV-1a is used only as a deterministic local fingerprint. Security
        // and collision resistance belong to the canonical hashing subsystem.
        let mut hash = 0xcbf29ce484222325u64;

        for byte in self.canonical_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        hash
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn require_node(
        &self,
        node: PhysicalQubitId,
    ) -> Result<(), TopologyError> {
        if self.nodes.contains(&node) {
            Ok(())
        } else {
            Err(TopologyError::UnknownResource {
                resource: node,
            })
        }
    }

    fn index_edge(
        &mut self,
        edge: TopologyEdge,
    ) {
        match edge.connectivity {
            Connectivity::Directed => {
                if let Some(outgoing) =
                    self.outgoing.get_mut(&edge.source)
                {
                    outgoing.insert(edge.target);
                }

                if let Some(incoming) =
                    self.incoming.get_mut(&edge.target)
                {
                    incoming.insert(edge.source);
                }

                if let Some(adjacency) =
                    self.undirected.get_mut(&edge.source)
                {
                    adjacency.insert(edge.target);
                }

                if let Some(adjacency) =
                    self.undirected.get_mut(&edge.target)
                {
                    adjacency.insert(edge.source);
                }
            }

            Connectivity::Bidirectional => {
                if let Some(outgoing) =
                    self.outgoing.get_mut(&edge.source)
                {
                    outgoing.insert(edge.target);
                }

                if let Some(outgoing) =
                    self.outgoing.get_mut(&edge.target)
                {
                    outgoing.insert(edge.source);
                }

                if let Some(incoming) =
                    self.incoming.get_mut(&edge.source)
                {
                    incoming.insert(edge.target);
                }

                if let Some(incoming) =
                    self.incoming.get_mut(&edge.target)
                {
                    incoming.insert(edge.source);
                }

                if let Some(adjacency) =
                    self.undirected.get_mut(&edge.source)
                {
                    adjacency.insert(edge.target);
                }

                if let Some(adjacency) =
                    self.undirected.get_mut(&edge.target)
                {
                    adjacency.insert(edge.source);
                }
            }
        }
    }

    fn rebuild_adjacency(&mut self) {
        self.outgoing.clear();
        self.incoming.clear();
        self.undirected.clear();

        for &node in &self.nodes {
            self.outgoing.insert(node, BTreeSet::new());
            self.incoming.insert(node, BTreeSet::new());
            self.undirected.insert(node, BTreeSet::new());
        }

        let edges = self.edges.iter().copied().collect::<Vec<_>>();

        for edge in edges {
            self.index_edge(edge);
        }
    }

    fn bfs_distances(
        &self,
        source: PhysicalQubitId,
        mode: PathMode,
    ) -> Result<
        BTreeMap<PhysicalQubitId, usize>,
        TopologyError,
    > {
        self.require_node(source)?;

        let mut distances = BTreeMap::new();
        let mut queue = VecDeque::new();

        distances.insert(source, 0);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let current_distance =
                distances.get(&current).copied().unwrap_or(0);

            let neighbours = match mode {
                PathMode::Directed => self.outgoing.get(&current),
                PathMode::Undirected => self.undirected.get(&current),
            };

            let Some(neighbours) = neighbours else {
                continue;
            };

            for &next in neighbours {
                if distances.contains_key(&next) {
                    continue;
                }

                let distance =
                    current_distance
                        .checked_add(1)
                        .ok_or(
                            TopologyError::NumericOverflow {
                                operation: "bfs distance",
                            },
                        )?;

                distances.insert(next, distance);
                queue.push_back(next);
            }
        }

        Ok(distances)
    }
}

// =============================================================================
// Helper functions
// =============================================================================

/// Returns a canonical unordered physical-resource pair.
#[must_use]
fn canonical_pair(
    first: PhysicalQubitId,
    second: PhysicalQubitId,
) -> (PhysicalQubitId, PhysicalQubitId) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

/// Reconstructs a shortest path from a predecessor map.
fn reconstruct_path(
    source: PhysicalQubitId,
    target: PhysicalQubitId,
    predecessor: &BTreeMap<PhysicalQubitId, PhysicalQubitId>,
) -> Vec<PhysicalQubitId> {
    let mut path = Vec::new();
    let mut current = target;

    path.push(current);

    while current != source {
        let Some(previous) = predecessor.get(&current).copied() else {
            // This state cannot occur after BFS found the target. Returning
            // the partial path is safer than panicking in production code.
            break;
        };

        current = previous;
        path.push(current);
    }

    path.reverse();
    path
}

/// Inserts an edge into independently constructed adjacency maps.
fn index_edge_into(
    outgoing: &mut BTreeMap<
        PhysicalQubitId,
        BTreeSet<PhysicalQubitId>,
    >,
    incoming: &mut BTreeMap<
        PhysicalQubitId,
        BTreeSet<PhysicalQubitId>,
    >,
    undirected: &mut BTreeMap<
        PhysicalQubitId,
        BTreeSet<PhysicalQubitId>,
    >,
    edge: &TopologyEdge,
) -> Result<(), TopologyError> {
    let source_outgoing =
        outgoing
            .get_mut(&edge.source)
            .ok_or(TopologyError::UnknownResource {
                resource: edge.source,
            })?;

    let target_incoming =
        incoming
            .get_mut(&edge.target)
            .ok_or(TopologyError::UnknownResource {
                resource: edge.target,
            })?;

    let source_undirected =
        undirected
            .get_mut(&edge.source)
            .ok_or(TopologyError::UnknownResource {
                resource: edge.source,
            })?;

    let target_undirected =
        undirected
            .get_mut(&edge.target)
            .ok_or(TopologyError::UnknownResource {
                resource: edge.target,
            })?;

    match edge.connectivity {
        Connectivity::Directed => {
            source_outgoing.insert(edge.target);
            target_incoming.insert(edge.source);
        }

        Connectivity::Bidirectional => {
            source_outgoing.insert(edge.target);

            outgoing
                .get_mut(&edge.target)
                .ok_or(TopologyError::UnknownResource {
                    resource: edge.target,
                })?
                .insert(edge.source);

            target_incoming.insert(edge.source);

            incoming
                .get_mut(&edge.source)
                .ok_or(TopologyError::UnknownResource {
                    resource: edge.source,
                })?
                .insert(edge.target);
        }
    }

    source_undirected.insert(edge.target);
    target_undirected.insert(edge.source);

    Ok(())
}

/// Appends a length-prefixed UTF-8 string to canonical bytes.
fn append_len_prefixed_str(
    output: &mut Vec<u8>,
    value: &str,
) {
    append_u64(output, value.len() as u64);
    output.extend_from_slice(value.as_bytes());
}

/// Appends a little-endian u64.
fn append_u64(
    output: &mut Vec<u8>,
    value: u64,
) {
    output.extend_from_slice(&value.to_le_bytes());
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn empty_topology_is_valid() {
        let topology = Topology::empty();

        assert!(topology.is_empty());
        assert_eq!(topology.node_count(), 0);
        assert_eq!(topology.edge_count(), 0);
        assert!(topology.validate().is_ok());
    }

    #[test]
    fn nodes_are_deterministic() {
        let topology =
            Topology::from_nodes([q(7), q(1), q(4)]).unwrap();

        let nodes: Vec<_> = topology.nodes().collect();

        assert_eq!(nodes, vec![q(1), q(4), q(7)]);
    }

    #[test]
    fn bidirectional_edge_is_canonicalized() {
        let edge = TopologyEdge::bidirectional(q(5), q(2))
            .unwrap();

        assert_eq!(edge.source, q(2));
        assert_eq!(edge.target, q(5));
        assert_eq!(
            edge.connectivity,
            Connectivity::Bidirectional
        );
    }

    #[test]
    fn directed_edges_preserve_direction() {
        let edge =
            TopologyEdge::directed(q(5), q(2)).unwrap();

        assert_eq!(edge.source, q(5));
        assert_eq!(edge.target, q(2));
        assert_eq!(
            edge.connectivity,
            Connectivity::Directed
        );
    }

    #[test]
    fn topology_requires_existing_nodes() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        let result =
            topology.add_edge(
                TopologyEdge::bidirectional(q(0), q(2))
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(TopologyError::UnknownResource {
                resource
            }) if resource == q(2)
        ));
    }

    #[test]
    fn self_coupling_is_rejected() {
        let result =
            TopologyEdge::bidirectional(q(1), q(1));

        assert!(matches!(
            result,
            Err(TopologyError::SelfCoupling { qubit })
                if qubit == q(1)
        ));
    }

    #[test]
    fn bidirectional_adjacency_is_two_way() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        topology
            .add_edge(
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        assert!(topology.permits(
            q(0),
            q(1),
            PathMode::Directed
        ));

        assert!(topology.permits(
            q(1),
            q(0),
            PathMode::Directed
        ));

        assert_eq!(
            topology.neighbors(q(0)).unwrap(),
            vec![q(1)]
        );

        assert_eq!(
            topology.neighbors(q(1)).unwrap(),
            vec![q(0)]
        );
    }

    #[test]
    fn directed_adjacency_respects_direction() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        assert!(topology.permits(
            q(0),
            q(1),
            PathMode::Directed
        ));

        assert!(!topology.permits(
            q(1),
            q(0),
            PathMode::Directed
        ));

        assert!(topology.permits(
            q(1),
            q(0),
            PathMode::Undirected
        ));
    }

    #[test]
    fn opposite_directed_edges_can_coexist() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(1), q(0))
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(topology.edge_count(), 2);
        assert!(topology.permits(
            q(0),
            q(1),
            PathMode::Directed
        ));
        assert!(topology.permits(
            q(1),
            q(0),
            PathMode::Directed
        ));
    }

    #[test]
    fn conflicting_bidirectional_and_directed_edges_are_rejected() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        let result =
            topology.add_edge(
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
            );

        assert!(matches!(
            result,
            Err(TopologyError::ConflictingConnectivity { .. })
        ));
    }

    #[test]
    fn shortest_path_is_deterministic() {
        let topology = Topology::from_parts(
            [q(0), q(1), q(2), q(3)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(3))
                    .unwrap(),
                TopologyEdge::bidirectional(q(0), q(2))
                    .unwrap(),
                TopologyEdge::bidirectional(q(2), q(3))
                    .unwrap(),
            ],
        )
        .unwrap();

        let path = topology
            .shortest_path(
                q(0),
                q(3),
                PathMode::Undirected,
            )
            .unwrap();

        // Ordered adjacency makes q1 win the tie deterministically.
        assert_eq!(
            path,
            vec![q(0), q(1), q(3)]
        );
    }

    #[test]
    fn disconnected_topology_has_no_path() {
        let topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        let result = topology.shortest_path(
            q(0),
            q(1),
            PathMode::Undirected,
        );

        assert!(matches!(
            result,
            Err(TopologyError::NoPath {
                source,
                target,
                mode: PathMode::Undirected
            }) if source == q(0) && target == q(1)
        ));
    }

    #[test]
    fn connected_components_are_deterministic() {
        let topology = Topology::from_parts(
            [q(0), q(1), q(2), q(3)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(2), q(3))
                    .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            topology.components(),
            vec![
                vec![q(0), q(1)],
                vec![q(2), q(3)]
            ]
        );

        assert_eq!(topology.connected_components(), 2);
        assert!(!topology.is_connected());
    }

    #[test]
    fn removing_node_removes_incident_edges() {
        let mut topology = Topology::from_parts(
            [q(0), q(1), q(2)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(2))
                    .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(topology.edge_count(), 2);

        assert!(topology.remove_node(q(1)));

        assert_eq!(topology.node_count(), 2);
        assert_eq!(topology.edge_count(), 0);
        assert!(topology.validate().is_ok());
    }

    #[test]
    fn statistics_are_structural() {
        let topology = Topology::from_parts(
            [q(0), q(1), q(2)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(2))
                    .unwrap(),
            ],
        )
        .unwrap();

        let statistics = topology.statistics();

        assert_eq!(statistics.resource_count, 3);
        assert_eq!(statistics.edge_count, 2);
        assert_eq!(statistics.minimum_degree, 1);
        assert_eq!(statistics.maximum_degree, 2);
        assert!(statistics.is_connected);
    }

    #[test]
    fn canonical_bytes_are_insertion_order_independent() {
        let first = Topology::from_parts(
            [q(0), q(1), q(2)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::directed(q(1), q(2))
                    .unwrap(),
            ],
        )
        .unwrap();

        let second = Topology::from_parts(
            [q(2), q(0), q(1)],
            [
                TopologyEdge::directed(q(1), q(2))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(0))
                    .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(
            first.canonical_bytes(),
            second.canonical_bytes()
        );

        assert_eq!(
            first.semantic_fingerprint(),
            second.semantic_fingerprint()
        );
    }

    #[test]
    fn removing_connection_removes_both_directed_edges() {
        let mut topology =
            Topology::from_nodes([q(0), q(1)]).unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(0), q(1))
                    .unwrap(),
            )
            .unwrap();

        topology
            .add_edge(
                TopologyEdge::directed(q(1), q(0))
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(
            topology.remove_connection(q(0), q(1)),
            2
        );

        assert_eq!(topology.edge_count(), 0);
        assert!(!topology.connected(q(0), q(1)));
    }

    #[test]
    fn range_constructor_is_half_open() {
        let topology =
            Topology::from_range(2, 5).unwrap();

        assert_eq!(
            topology.nodes().collect::<Vec<_>>(),
            vec![q(2), q(3), q(4)]
        );
    }

    #[test]
    fn diameter_is_explicit() {
        let topology = Topology::from_parts(
            [q(0), q(1), q(2)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(2))
                    .unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(topology.diameter().unwrap(), Some(2));
    }

    #[test]
    fn average_shortest_path_is_explicit() {
        let topology = Topology::from_parts(
            [q(0), q(1), q(2)],
            [
                TopologyEdge::bidirectional(q(0), q(1))
                    .unwrap(),
                TopologyEdge::bidirectional(q(1), q(2))
                    .unwrap(),
            ],
        )
        .unwrap();

        let average =
            topology.average_shortest_path().unwrap();

        assert_eq!(average, Some(4.0 / 3.0));
    }

    #[test]
    fn canonical_fingerprint_changes_with_semantics() {
        let first = Topology::from_parts(
            [q(0), q(1)],
            [TopologyEdge::bidirectional(q(0), q(1)).unwrap()],
        )
        .unwrap();

        let second = Topology::from_parts(
            [q(0), q(1)],
            [TopologyEdge::directed(q(0), q(1)).unwrap()],
        )
        .unwrap();

        assert_ne!(
            first.semantic_fingerprint(),
            second.semantic_fingerprint()
        );
    }
}