//! Zamani Quantum Scheduling — Distributed Network Model.
//!
//! `src/quantum/scheduling/distributed/network.rs`
//!
//! # Purpose
//!
//! This module provides the scheduler-facing representation of a distributed
//! quantum execution network.
//!
//! It answers:
//!
//! > "Which distributed execution nodes and communication links exist in the
//! > scheduling target, and which paths are structurally available between
//! > them?"
//!
//! It deliberately does NOT answer:
//!
//! - how a physical network is discovered;
//! - how a network connection is opened;
//! - how packets are transmitted;
//! - how entanglement is physically generated;
//! - how a QPU is executed;
//! - how authentication is performed;
//! - how hardware is calibrated;
//! - which physical route is globally optimal for every possible objective;
//! - how a scheduler reserves resources;
//! - how quantum semantics are represented.
//!
//! Those responsibilities belong to hardware, runtime, transport, routing,
//! communication, resource and scheduler-policy subsystems respectively.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum IR
//!                                │
//!                                │ canonical NodeId / LinkId / qubits
//!                                ▼
//!                         quantum::routing
//!                                │
//!                         placement / route
//!                                ▼
//!                       quantum::scheduling
//!                                │
//!              ┌─────────────────┼──────────────────┐
//!              │                 │                  │
//!              ▼                 ▼                  ▼
//!       distributed::node  distributed::link  distributed::communication
//!              │                 │                  │
//!              └─────────────────┼──────────────────┘
//!                                ▼
//!                     distributed::network
//!                                │
//!                    topology / path structure
//!                                │
//!                                ▼
//!                       scheduler planners
//!                                │
//!                                ▼
//!                    resources / timing / constraints
//!                                │
//!                                ▼
//!                       verification / result
//!                                │
//!                                ▼
//!                         hardware / runtime
//! ```
//!
//! # Ownership boundaries
//!
//! ## Canonical IR
//!
//! The canonical distributed IR owns semantic distributed identities:
//!
//! ```text
//! crate::quantum::ir::model::distributed::NodeId
//! crate::quantum::ir::model::distributed::LinkId
//! ```
//!
//! This module MUST NOT define replacement types for those identities.
//!
//! ## Qubits
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not create another qubit identity system.
//!
//! A network describes connectivity between distributed execution domains.
//! Qubit placement and logical-to-physical mapping are supplied by the IR,
//! routing subsystem or their adapters.
//!
//! ## `distributed/link.rs`
//!
//! `distributed/link.rs` describes one scheduler-visible communication link:
//!
//! ```text
//! link identity
//! endpoints
//! direction
//! link kind
//! capacity
//! timing
//! availability
//! link-local resources
//! ```
//!
//! This module describes the collection of those links:
//!
//! ```text
//! nodes
//! links
//! adjacency
//! topology
//! path structure
//! network-level metadata
//! ```
//!
//! It intentionally does not duplicate `ScheduledLink`.
//!
//! ## `distributed/communication.rs`
//!
//! Communication requests describe an operation that needs communication.
//!
//! This module answers only the structural question:
//!
//! ```text
//! Can source node reach destination node?
//! Which link sequence connects them?
//! ```
//!
//! Timing, capacity and reservation decisions remain scheduler concerns.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written at the semantic level.
//!
//! The same program may be scheduled against:
//!
//! ```text
//! one node
//! two nodes
//! many nodes
//! multi-chip systems
//! modular QPUs
//! quantum data centres
//! quantum networks
//! heterogeneous quantum/classical systems
//! future distributed quantum architectures
//! ```
//!
//! No network size is encoded in this module.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_NODES
//! MAX_LINKS
//! MAX_HOPS
//! MAX_NETWORK_SIZE
//! MAX_QUBITS
//! ```
//!
//! "Infinity" means that this module imposes no artificial finite
//! architectural ceiling. A real compilation remains bounded by available
//! memory, CPU time, target resources, policy limits and the actual target
//! description.
//!
//! # Sparse representation
//!
//! Networks are represented as sparse graphs:
//!
//! ```text
//! BTreeMap<NodeId, NodeRecord>
//! BTreeMap<LinkId, NetworkLink>
//! BTreeMap<NodeId, BTreeSet<LinkId>>
//! ```
//!
//! This avoids allocating a dense `N × N` adjacency matrix.
//!
//! A network with a small number of nodes remains inexpensive, while a large
//! sparse network does not require storage proportional to every possible pair
//! of nodes.
//!
//! # Determinism
//!
//! Ordered maps and sets are deliberately used.
//!
//! When multiple paths have equal cost, deterministic tie-breaking is based on
//! canonical typed identifiers and path order rather than hash-map iteration.
//!
//! This is important for:
//!
//! - reproducible compilation;
//! - deterministic testing;
//! - diagnostics;
//! - schedule provenance;
//! - distributed compilation;
//! - regression testing.
//!
//! # Algorithms
//!
//! The network model provides structural traversal and deterministic shortest
//! path selection using a non-negative integer edge cost.
//!
//! It does NOT hard-code one universal routing objective.
//!
//! A caller may provide a `PathCost` implementation appropriate for its
//! scheduling objective.
//!
//! Examples include:
//!
//! - hop count;
//! - latency;
//! - communication cost;
//! - reliability penalty;
//! - fidelity penalty;
//! - resource pressure;
//! - a composite target-specific metric.
//!
//! More sophisticated routing belongs to `quantum::routing`.
//!
//! # Arithmetic safety
//!
//! Path costs use checked arithmetic.
//!
//! Overflow is reported as an error instead of wrapping.
//!
//! No wrapping arithmetic is used for scheduling semantics.
//!
//! # Execution boundary
//!
//! This module contains no:
//!
//! - sockets;
//! - threads;
//! - async runtime;
//! - network transport;
//! - vendor SDK;
//! - authentication;
//! - hardware handles;
//! - QPU execution.
//!
//! The graph is a pure scheduling data model.
//!
//! # Thread safety
//!
//! `QuantumNetwork` owns ordinary Rust collections and contains no global
//! mutable state or interior mutability.
//!
//! Separate network instances can therefore safely be used by independent
//! compilation jobs. Sharing or parallel traversal follows ordinary Rust
//! ownership rules.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::model::distributed
//! quantum::ir::qubit
//! distributed::node
//! distributed::link
//! ```
//!
//! Downstream:
//!
//! ```text
//! distributed::communication
//! planners
//! resources
//! constraints
//! adapters::routing
//! adapters::hardware
//! verification
//! diagnostics
//! result
//! runtime
//! ```
//!
//! The dependency direction is intentionally one-way.
//!
//! This module must not import scheduler algorithms, runtime transports,
//! hardware SDKs or routing implementations.
//!
//! Therefore adding a scheduler algorithm, hardware backend, runtime,
//! communication planner or routing strategy must not require changing the
//! topology representation.
//!
//! # Important invariant
//!
//! A successful `add_link` operation guarantees:
//!
//! ```text
//! source != destination
//! link identity is unique
//! both endpoint nodes exist
//! adjacency contains the link
//! link endpoints are preserved exactly
//! ```
//!
//! A successful path returned by this module guarantees:
//!
//! ```text
//! path contains the requested source and destination
//! every consecutive pair of nodes is connected by the returned link
//! every returned link belongs to this network
//! path contains no repeated node
//! ```
//!
//! The final invariant prevents cycles from appearing in simple-path results.
//!
//! # Network versus scheduling
//!
//! This module does NOT reserve a path.
//!
//! A path returned here is a structural candidate.
//!
//! The scheduler subsequently combines it with:
//!
//! ```text
//! communication requirements
//! +
//! link capacity
//! +
//! link availability
//! +
//! timing windows
//! +
//! resource reservations
//! +
//! dependencies
//! ```
//!
//! to determine when communication may occur.
//!
//! # No artificial machine assumptions
//!
//! Nothing in this file assumes:
//!
//! - a particular number of nodes;
//! - a particular number of links;
//! - a particular topology;
//! - a particular quantum technology;
//! - a particular number of qubits;
//! - a particular communication protocol;
//! - a particular clock frequency;
//! - a particular network diameter.
//!
//! Those are target properties.
//!
//! # Safety
//!
//! No unsafe Rust is permitted.
//!
//! The restriction is compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::model::distributed::{
    LinkDirection,
    LinkId,
    LinkEndpoint,
    NodeId,
    QuantumLinkKind,
};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the scheduler-facing distributed network model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    /// A node with the requested identity does not exist.
    NodeNotFound {
        /// Missing node identity.
        node: NodeId,
    },

    /// A link with the requested identity does not exist.
    LinkNotFound {
        /// Missing link identity.
        link: LinkId,
    },

    /// A node was registered more than once.
    DuplicateNode {
        /// Existing node identity.
        node: NodeId,
    },

    /// A link was registered more than once.
    DuplicateLink {
        /// Existing link identity.
        link: LinkId,
    },

    /// A link connects a node to itself.
    SelfLink {
        /// Node used at both endpoints.
        node: NodeId,
    },

    /// A link endpoint was not present in the network.
    EndpointNotFound {
        /// Missing endpoint.
        node: NodeId,
    },

    /// A link has an endpoint that is inconsistent with the requested path.
    EndpointMismatch {
        /// Link identity.
        link: LinkId,

        /// Expected node.
        expected: NodeId,

        /// Actual endpoint.
        actual: NodeId,
    },

    /// A path request contains the same source and destination.
    SameEndpoint {
        /// Endpoint node.
        node: NodeId,
    },

    /// No route exists between the requested nodes.
    NoPath {
        /// Source node.
        source: NodeId,

        /// Destination node.
        destination: NodeId,
    },

    /// A path contains an invalid link sequence.
    InvalidPath {
        /// Position in the supplied path.
        position: usize,
    },

    /// A path contains repeated nodes.
    CyclicPath,

    /// A caller attempted to exceed a policy-imposed traversal limit.
    ///
    /// This is a caller policy, not an architectural network limit.
    TraversalLimitExceeded,

    /// A caller supplied a negative/invalid conceptual cost.
    ///
    /// The built-in path cost is unsigned, so this variant is primarily
    /// reserved for extensible cost implementations.
    InvalidCost,

    /// Path-cost addition overflowed the representable integer range.
    CostOverflow,

    /// A network invariant was violated.
    InvariantViolation {
        /// Human-readable invariant description.
        reason: String,
    },
}

impl fmt::Display for NetworkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NodeNotFound { node } => {
                write!(formatter, "distributed network node not found: {node}")
            }

            Self::LinkNotFound { link } => {
                write!(formatter, "distributed network link not found: {link}")
            }

            Self::DuplicateNode { node } => {
                write!(formatter, "distributed network node already exists: {node}")
            }

            Self::DuplicateLink { link } => {
                write!(formatter, "distributed network link already exists: {link}")
            }

            Self::SelfLink { node } => {
                write!(formatter, "distributed network link cannot connect node {node} to itself")
            }

            Self::EndpointNotFound { node } => {
                write!(formatter, "distributed network link endpoint not found: {node}")
            }

            Self::EndpointMismatch {
                link,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "link {link} endpoint mismatch: expected {expected}, found {actual}"
                )
            }

            Self::SameEndpoint { node } => {
                write!(formatter, "source and destination are the same node: {node}")
            }

            Self::NoPath {
                source,
                destination,
            } => {
                write!(
                    formatter,
                    "no distributed path exists from {source} to {destination}"
                )
            }

            Self::InvalidPath { position } => {
                write!(formatter, "invalid distributed path at position {position}")
            }

            Self::CyclicPath => {
                write!(formatter, "distributed path contains a cycle")
            }

            Self::TraversalLimitExceeded => {
                write!(formatter, "distributed path traversal policy limit exceeded")
            }

            Self::InvalidCost => {
                write!(formatter, "invalid distributed path cost")
            }

            Self::CostOverflow => {
                write!(formatter, "distributed path cost overflow")
            }

            Self::InvariantViolation { reason } => {
                write!(formatter, "distributed network invariant violation: {reason}")
            }
        }
    }
}

impl std::error::Error for NetworkError {}

// =============================================================================
// Network node record
// =============================================================================

/// Minimal scheduler-facing node record.
///
/// Detailed node capabilities remain owned by `distributed::node`.
///
/// The network only needs the node's canonical identity and optional
/// scheduler-facing label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkNode {
    id: NodeId,
    label: Option<String>,
}

impl NetworkNode {
    /// Creates a network node record.
    #[must_use]
    pub const fn new(id: NodeId) -> Self {
        Self {
            id,
            label: None,
        }
    }

    /// Creates a network node record with a descriptive label.
    #[must_use]
    pub fn with_label(id: NodeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: Some(label.into()),
        }
    }

    /// Returns the canonical node identity.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

// =============================================================================
// Network link
// =============================================================================

/// Scheduler-facing structural representation of one network link.
///
/// Detailed capacity, timing, calibration and availability information belongs
/// to `distributed::link`.
///
/// This type intentionally contains only the topology information required by
/// the network graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkLink {
    id: LinkId,
    source: NodeId,
    destination: NodeId,
    direction: LinkDirection,
    kind: QuantumLinkKind,
}

impl NetworkLink {
    /// Creates a topology link.
    ///
    /// The endpoint nodes must be different.
    pub fn new(
        id: LinkId,
        source: NodeId,
        destination: NodeId,
        direction: LinkDirection,
        kind: QuantumLinkKind,
    ) -> Result<Self, NetworkError> {
        if source == destination {
            return Err(NetworkError::SelfLink { node: source });
        }

        Ok(Self {
            id,
            source,
            destination,
            direction,
            kind,
        })
    }

    /// Returns the canonical link identity.
    #[must_use]
    pub const fn id(&self) -> LinkId {
        self.id
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns the destination endpoint.
    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    /// Returns the canonical link direction.
    #[must_use]
    pub const fn direction(&self) -> LinkDirection {
        self.direction
    }

    /// Returns the semantic quantum link kind.
    #[must_use]
    pub const fn kind(&self) -> QuantumLinkKind {
        self.kind
    }

    /// Returns whether the link can be traversed from `from` to `to`.
    #[must_use]
    pub const fn permits(&self, from: NodeId, to: NodeId) -> bool {
        match self.direction {
            LinkDirection::Bidirectional => {
                (from == self.source && to == self.destination)
                    || (from == self.destination && to == self.source)
            }

            LinkDirection::Directed => {
                from == self.source && to == self.destination
            }
        }
    }

    /// Returns the opposite endpoint when traversing from `node`.
    #[must_use]
    pub const fn other_endpoint(self, node: NodeId) -> Option<NodeId> {
        if node == self.source {
            Some(self.destination)
        } else if node == self.destination {
            Some(self.source)
        } else {
            None
        }
    }
}

// =============================================================================
// Path
// =============================================================================

/// A validated simple path through a distributed network.
///
/// The path stores both nodes and links so downstream scheduling code does not
/// need to reconstruct topology relationships.
///
/// For `N` nodes in a path there are exactly `N - 1` links.
///
/// No maximum path length is imposed by this type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkPath {
    source: NodeId,
    destination: NodeId,
    nodes: Vec<NodeId>,
    links: Vec<LinkId>,
    cost: u128,
}

impl NetworkPath {
    /// Creates a validated path.
    fn new(
        source: NodeId,
        destination: NodeId,
        nodes: Vec<NodeId>,
        links: Vec<LinkId>,
        cost: u128,
    ) -> Result<Self, NetworkError> {
        if source == destination {
            return Err(NetworkError::SameEndpoint { node: source });
        }

        if nodes.len() < 2 || links.len().checked_add(1) != Some(nodes.len()) {
            return Err(NetworkError::InvalidPath { position: 0 });
        }

        let mut seen = BTreeSet::new();

        for node in &nodes {
            if !seen.insert(*node) {
                return Err(NetworkError::CyclicPath);
            }
        }

        if nodes.first().copied() != Some(source) {
            return Err(NetworkError::InvalidPath { position: 0 });
        }

        if nodes.last().copied() != Some(destination) {
            return Err(NetworkError::InvalidPath {
                position: nodes.len().saturating_sub(1),
            });
        }

        Ok(Self {
            source,
            destination,
            nodes,
            links,
            cost,
        })
    }

    /// Returns the source node.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns the destination node.
    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    /// Returns all nodes in traversal order.
    #[must_use]
    pub fn nodes(&self) -> &[NodeId] {
        &self.nodes
    }

    /// Returns all links in traversal order.
    #[must_use]
    pub fn links(&self) -> &[LinkId] {
        &self.links
    }

    /// Returns the number of links traversed.
    #[must_use]
    pub fn hop_count(&self) -> usize {
        self.links.len()
    }

    /// Returns the deterministic path cost.
    #[must_use]
    pub const fn cost(&self) -> u128 {
        self.cost
    }

    /// Returns whether this path contains no intermediate nodes.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        self.links.len() == 1
    }

    /// Returns an iterator over node pairs.
    pub fn node_pairs(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        self.nodes.windows(2).filter_map(|window| {
            match window {
                [from, to] => Some((*from, *to)),
                _ => None,
            }
        })
    }
}

// =============================================================================
// Path cost
// =============================================================================

/// Cost assigned to traversing a network link.
///
/// Costs must be non-negative.
///
/// A cost does not necessarily represent physical time. Callers may use it
/// for latency, hop count, reliability penalty, fidelity penalty, resource
/// pressure or another routing objective.
///
/// The scheduler may use a different cost model from the routing subsystem.
pub trait PathCost {
    /// Calculates the cost of one link.
    fn cost(&self, link: &NetworkLink) -> Result<u128, NetworkError>;
}

/// Unit cost where every link costs one.
///
/// This provides deterministic minimum-hop traversal without assuming that
/// every physical link has the same latency or fidelity.
#[derive(Debug, Clone, Copy, Default)]
pub struct HopCountCost;

impl PathCost for HopCountCost {
    fn cost(&self, _link: &NetworkLink) -> Result<u128, NetworkError> {
        Ok(1)
    }
}

/// Zero cost for every link.
///
/// This is useful for callers that want deterministic structural traversal
/// while treating all links as equivalent.
///
/// It must not be interpreted as physical zero latency.
#[derive(Debug, Clone, Copy, Default)]
pub struct ZeroCost;

impl PathCost for ZeroCost {
    fn cost(&self, _link: &NetworkLink) -> Result<u128, NetworkError> {
        Ok(0)
    }
}

// =============================================================================
// Traversal policy
// =============================================================================

/// Optional caller-controlled traversal policy.
///
/// This is not a network architecture limit.
///
/// It exists so an application can protect a compilation job from spending
/// unbounded resources on a particular request.
///
/// The default policy has no traversal limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TraversalPolicy {
    max_visited_nodes: Option<u64>,
}

impl TraversalPolicy {
    /// Creates an unrestricted traversal policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_visited_nodes: None,
        }
    }

    /// Creates a traversal policy with an explicit caller-provided limit.
    ///
    /// The limit is an execution policy, not a network-size restriction.
    #[must_use]
    pub const fn with_max_visited_nodes(max: u64) -> Self {
        Self {
            max_visited_nodes: Some(max),
        }
    }

    /// Returns the configured limit.
    #[must_use]
    pub const fn max_visited_nodes(self) -> Option<u64> {
        self.max_visited_nodes
    }

    fn permits(self, visited: usize) -> bool {
        match self.max_visited_nodes {
            Some(limit) => match u64::try_from(visited) {
                Ok(value) => value <= limit,
                Err(_) => false,
            },
            None => true,
        }
    }
}

// =============================================================================
// Queue entry
// =============================================================================

/// Internal deterministic shortest-path queue entry.
///
/// `BTreeSet` is used instead of a binary heap so ordering is fully explicit
/// and stable even when two candidates have equal costs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct QueueEntry {
    cost: u128,
    node: NodeId,
}

// =============================================================================
// Network
// =============================================================================

/// Scheduler-facing distributed network topology.
///
/// The network owns:
///
/// ```text
/// node registry
/// link registry
/// adjacency index
/// topology metadata
/// ```
///
/// It does not own:
///
/// ```text
/// hardware
/// resource reservations
/// transport
/// routing policy
/// scheduling policy
/// execution
/// ```
///
/// The topology is sparse and dynamically constructible.
///
/// No fixed number of nodes or links is assumed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QuantumNetwork {
    nodes: BTreeMap<NodeId, NetworkNode>,
    links: BTreeMap<LinkId, NetworkLink>,
    adjacency: BTreeMap<NodeId, BTreeSet<LinkId>>,
}

impl QuantumNetwork {
    /// Creates an empty distributed network.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of registered links.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Returns whether the network contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Adds a node.
    ///
    /// Node identity is canonical and must be unique within this network.
    pub fn add_node(&mut self, node: NetworkNode) -> Result<(), NetworkError> {
        let id = node.id();

        if self.nodes.contains_key(&id) {
            return Err(NetworkError::DuplicateNode { node: id });
        }

        self.nodes.insert(id, node);
        self.adjacency.entry(id).or_default();

        Ok(())
    }

    /// Adds a canonical node identity without a label.
    pub fn insert_node(&mut self, id: NodeId) -> Result<(), NetworkError> {
        self.add_node(NetworkNode::new(id))
    }

    /// Returns a node.
    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&NetworkNode> {
        self.nodes.get(&id)
    }

    /// Returns all nodes in deterministic identity order.
    pub fn nodes(&self) -> impl Iterator<Item = &NetworkNode> {
        self.nodes.values()
    }

    /// Adds a topology link.
    ///
    /// Both endpoint nodes must already exist.
    pub fn add_link(&mut self, link: NetworkLink) -> Result<(), NetworkError> {
        let id = link.id();

        if self.links.contains_key(&id) {
            return Err(NetworkError::DuplicateLink { link: id });
        }

        let source = link.source();
        let destination = link.destination();

        if source == destination {
            return Err(NetworkError::SelfLink { node: source });
        }

        if !self.nodes.contains_key(&source) {
            return Err(NetworkError::EndpointNotFound { node: source });
        }

        if !self.nodes.contains_key(&destination) {
            return Err(NetworkError::EndpointNotFound {
                node: destination,
            });
        }

        self.links.insert(id, link);

        self.adjacency
            .entry(source)
            .or_default()
            .insert(id);

        self.adjacency
            .entry(destination)
            .or_default()
            .insert(id);

        Ok(())
    }

    /// Adds a link directly from canonical distributed identities.
    pub fn insert_link(
        &mut self,
        id: LinkId,
        source: NodeId,
        destination: NodeId,
        direction: LinkDirection,
        kind: QuantumLinkKind,
    ) -> Result<(), NetworkError> {
        let link = NetworkLink::new(
            id,
            source,
            destination,
            direction,
            kind,
        )?;

        self.add_link(link)
    }

    /// Returns a link.
    #[must_use]
    pub fn link(&self, id: LinkId) -> Option<&NetworkLink> {
        self.links.get(&id)
    }

    /// Returns all links in deterministic identity order.
    pub fn links(&self) -> impl Iterator<Item = &NetworkLink> {
        self.links.values()
    }

    /// Returns all link identities incident to a node.
    ///
    /// The returned collection is deterministically ordered.
    pub fn incident_links(
        &self,
        node: NodeId,
    ) -> Result<&BTreeSet<LinkId>, NetworkError> {
        if !self.nodes.contains_key(&node) {
            return Err(NetworkError::NodeNotFound { node });
        }

        self.adjacency
            .get(&node)
            .ok_or(NetworkError::InvariantViolation {
                reason: format!("node {node} has no adjacency entry"),
            })
    }

    /// Returns neighboring nodes reachable from `node`.
    ///
    /// Directionality is respected.
    ///
    /// Neighbors are returned in deterministic node-identity order.
    pub fn neighbors(
        &self,
        node: NodeId,
    ) -> Result<BTreeSet<NodeId>, NetworkError> {
        if !self.nodes.contains_key(&node) {
            return Err(NetworkError::NodeNotFound { node });
        }

        let mut result = BTreeSet::new();

        let links = self
            .adjacency
            .get(&node)
            .ok_or(NetworkError::InvariantViolation {
                reason: format!("node {node} has no adjacency entry"),
            })?;

        for link_id in links {
            let link = self
                .links
                .get(link_id)
                .ok_or(NetworkError::LinkNotFound { link: *link_id })?;

            if let Some(neighbor) = link.other_endpoint(node) {
                if link.permits(node, neighbor) {
                    result.insert(neighbor);
                }
            }
        }

        Ok(result)
    }

    /// Returns whether a directed traversal is structurally possible.
    #[must_use]
    pub fn connected(&self, source: NodeId, destination: NodeId) -> bool {
        if source == destination {
            return self.nodes.contains_key(&source);
        }

        if !self.nodes.contains_key(&source)
            || !self.nodes.contains_key(&destination)
        {
            return false;
        }

        self.neighbors(source)
            .map(|neighbors| neighbors.contains(&destination))
            .unwrap_or(false)
    }

    /// Finds one deterministic minimum-hop path.
    ///
    /// This method is intentionally a convenience API. It must not be treated
    /// as the universal quantum-network routing algorithm.
    pub fn shortest_path(
        &self,
        source: NodeId,
        destination: NodeId,
    ) -> Result<NetworkPath, NetworkError> {
        self.shortest_path_with_cost(
            source,
            destination,
            &HopCountCost,
            TraversalPolicy::unrestricted(),
        )
    }

    /// Finds a deterministic minimum-cost path.
    ///
    /// All edge costs must be non-negative.
    ///
    /// Equal-cost candidates are resolved deterministically using node and
    /// predecessor identities.
    pub fn shortest_path_with_cost<C: PathCost>(
        &self,
        source: NodeId,
        destination: NodeId,
        cost_model: &C,
        policy: TraversalPolicy,
    ) -> Result<NetworkPath, NetworkError> {
        self.validate_endpoint(source)?;
        self.validate_endpoint(destination)?;

        if source == destination {
            return Err(NetworkError::SameEndpoint { node: source });
        }

        let mut distances: BTreeMap<NodeId, u128> = BTreeMap::new();
        let mut predecessor: BTreeMap<NodeId, (NodeId, LinkId)> = BTreeMap::new();
        let mut queue = BTreeSet::new();
        let mut visited = BTreeSet::new();

        distances.insert(source, 0);
        queue.insert(QueueEntry {
            cost: 0,
            node: source,
        });

        while let Some(current) = queue.pop_first() {
            if !policy.permits(visited.len()) {
                return Err(NetworkError::TraversalLimitExceeded);
            }

            if !visited.insert(current.node) {
                continue;
            }

            if current.node == destination {
                break;
            }

            let links = self
                .adjacency
                .get(&current.node)
                .ok_or(NetworkError::InvariantViolation {
                    reason: format!(
                        "node {} has no adjacency entry",
                        current.node
                    ),
                })?;

            for link_id in links {
                let link = self
                    .links
                    .get(link_id)
                    .ok_or(NetworkError::LinkNotFound { link: *link_id })?;

                let Some(neighbor) = link.other_endpoint(current.node) else {
                    continue;
                };

                if !link.permits(current.node, neighbor) {
                    continue;
                }

                if visited.contains(&neighbor) {
                    continue;
                }

                let edge_cost = cost_model.cost(link)?;

                let candidate_cost = current
                    .cost
                    .checked_add(edge_cost)
                    .ok_or(NetworkError::CostOverflow)?;

                let should_update = match distances.get(&neighbor) {
                    None => true,
                    Some(existing) => candidate_cost < *existing,
                };

                if should_update {
                    if let Some(previous_cost) = distances.get(&neighbor) {
                        queue.remove(&QueueEntry {
                            cost: *previous_cost,
                            node: neighbor,
                        });
                    }

                    distances.insert(neighbor, candidate_cost);

                    predecessor.insert(
                        neighbor,
                        (current.node, *link_id),
                    );

                    queue.insert(QueueEntry {
                        cost: candidate_cost,
                        node: neighbor,
                    });
                } else if let Some(existing) = distances.get(&neighbor) {
                    if candidate_cost == *existing {
                        let replace = match predecessor.get(&neighbor) {
                            None => true,
                            Some((previous_node, previous_link)) => {
                                (current.node, *link_id)
                                    < (*previous_node, *previous_link)
                            }
                        };

                        if replace {
                            predecessor.insert(
                                neighbor,
                                (current.node, *link_id),
                            );
                        }
                    }
                }
            }
        }

        let total_cost = match distances.get(&destination) {
            Some(value) => *value,
            None => {
                return Err(NetworkError::NoPath {
                    source,
                    destination,
                });
            }
        };

        self.reconstruct_path(
            source,
            destination,
            &predecessor,
            total_cost,
        )
    }

    /// Returns every structurally reachable node from `source`.
    ///
    /// This traversal ignores timing, capacity, availability and resource
    /// contention. Those belong to scheduling.
    pub fn reachable_nodes(
        &self,
        source: NodeId,
    ) -> Result<BTreeSet<NodeId>, NetworkError> {
        self.reachable_nodes_with_policy(
            source,
            TraversalPolicy::unrestricted(),
        )
    }

    /// Returns every structurally reachable node under a caller policy.
    pub fn reachable_nodes_with_policy(
        &self,
        source: NodeId,
        policy: TraversalPolicy,
    ) -> Result<BTreeSet<NodeId>, NetworkError> {
        self.validate_endpoint(source)?;

        let mut visited = BTreeSet::new();
        let mut frontier = BTreeSet::new();

        frontier.insert(source);

        while let Some(node) = frontier.pop_first() {
            if !policy.permits(visited.len()) {
                return Err(NetworkError::TraversalLimitExceeded);
            }

            if !visited.insert(node) {
                continue;
            }

            for neighbor in self.neighbors(node)? {
                if !visited.contains(&neighbor) {
                    frontier.insert(neighbor);
                }
            }
        }

        Ok(visited)
    }

    /// Validates all internal topology invariants.
    ///
    /// This is useful after importing topology data from an adapter or
    /// deserializing a network representation.
    pub fn validate(&self) -> Result<(), NetworkError> {
        for (node_id, node) in &self.nodes {
            if node.id() != *node_id {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "node registry key {} does not match node identity {}",
                        node_id,
                        node.id()
                    ),
                });
            }

            if !self.adjacency.contains_key(node_id) {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "node {} is missing adjacency entry",
                        node_id
                    ),
                });
            }
        }

        for (link_id, link) in &self.links {
            if link.id() != *link_id {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "link registry key {} does not match link identity {}",
                        link_id,
                        link.id()
                    ),
                });
            }

            let source = link.source();
            let destination = link.destination();

            if source == destination {
                return Err(NetworkError::SelfLink { node: source });
            }

            if !self.nodes.contains_key(&source) {
                return Err(NetworkError::EndpointNotFound { node: source });
            }

            if !self.nodes.contains_key(&destination) {
                return Err(NetworkError::EndpointNotFound {
                    node: destination,
                });
            }

            let source_adjacency = self
                .adjacency
                .get(&source)
                .ok_or(NetworkError::InvariantViolation {
                    reason: format!(
                        "source node {} has no adjacency entry",
                        source
                    ),
                })?;

            if !source_adjacency.contains(link_id) {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "source node {} is missing link {} from adjacency",
                        source,
                        link_id
                    ),
                });
            }

            let destination_adjacency = self
                .adjacency
                .get(&destination)
                .ok_or(NetworkError::InvariantViolation {
                    reason: format!(
                        "destination node {} has no adjacency entry",
                        destination
                    ),
                })?;

            if !destination_adjacency.contains(link_id) {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "destination node {} is missing link {} from adjacency",
                        destination,
                        link_id
                    ),
                });
            }
        }

        for (node, link_ids) in &self.adjacency {
            if !self.nodes.contains_key(node) {
                return Err(NetworkError::InvariantViolation {
                    reason: format!(
                        "adjacency contains unknown node {}",
                        node
                    ),
                });
            }

            for link_id in link_ids {
                let link = self
                    .links
                    .get(link_id)
                    .ok_or(NetworkError::LinkNotFound { link: *link_id })?;

                if link.source() != *node && link.destination() != *node {
                    return Err(NetworkError::InvariantViolation {
                        reason: format!(
                            "adjacency node {} references unrelated link {}",
                            node,
                            link_id
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Removes a link from the network.
    ///
    /// Removing a topology link does not affect any external scheduler
    /// reservation. Existing schedules must be invalidated or revalidated by
    /// their owning scheduler.
    pub fn remove_link(
        &mut self,
        id: LinkId,
    ) -> Result<NetworkLink, NetworkError> {
        let link = self
            .links
            .remove(&id)
            .ok_or(NetworkError::LinkNotFound { link: id })?;

        if let Some(source_links) = self.adjacency.get_mut(&link.source()) {
            source_links.remove(&id);
        }

        if let Some(destination_links) =
            self.adjacency.get_mut(&link.destination())
        {
            destination_links.remove(&id);
        }

        Ok(link)
    }

    /// Removes a node if it has no incident links.
    ///
    /// A node with links must be disconnected explicitly first. This prevents
    /// accidental creation of a partially invalid topology.
    pub fn remove_node(
        &mut self,
        id: NodeId,
    ) -> Result<NetworkNode, NetworkError> {
        self.validate_endpoint(id)?;

        let incident = self
            .adjacency
            .get(&id)
            .ok_or(NetworkError::InvariantViolation {
                reason: format!("node {} has no adjacency entry", id),
            })?;

        if !incident.is_empty() {
            return Err(NetworkError::InvariantViolation {
                reason: format!(
                    "node {} still has {} incident link(s)",
                    id,
                    incident.len()
                ),
            });
        }

        self.adjacency.remove(&id);

        self.nodes
            .remove(&id)
            .ok_or(NetworkError::NodeNotFound { node: id })
    }

    /// Returns the degree of a node considering structural incidence.
    ///
    /// For directed links, this includes both incoming and outgoing
    /// incidence. Use `neighbors` when direction-sensitive reachability is
    /// required.
    pub fn degree(&self, node: NodeId) -> Result<usize, NetworkError> {
        self.incident_links(node).map(BTreeSet::len)
    }

    /// Returns the number of reachable outgoing neighbors.
    pub fn out_degree(
        &self,
        node: NodeId,
    ) -> Result<usize, NetworkError> {
        self.neighbors(node).map(|neighbors| neighbors.len())
    }

    /// Returns all links between two nodes that permit traversal from source
    /// to destination.
    ///
    /// Multiple parallel links are supported.
    pub fn links_between(
        &self,
        source: NodeId,
        destination: NodeId,
    ) -> Result<BTreeSet<LinkId>, NetworkError> {
        self.validate_endpoint(source)?;
        self.validate_endpoint(destination)?;

        let mut result = BTreeSet::new();

        let links = self
            .adjacency
            .get(&source)
            .ok_or(NetworkError::InvariantViolation {
                reason: format!(
                    "node {} has no adjacency entry",
                    source
                ),
            })?;

        for link_id in links {
            let link = self
                .links
                .get(link_id)
                .ok_or(NetworkError::LinkNotFound { link: *link_id })?;

            if link.permits(source, destination) {
                result.insert(*link_id);
            }
        }

        Ok(result)
    }

    /// Creates a validated path from a caller-provided node/link sequence.
    ///
    /// This is useful when `quantum::routing` has already selected a route.
    ///
    /// The method verifies topology consistency but does not apply resource,
    /// timing or availability constraints.
    pub fn validate_path(
        &self,
        source: NodeId,
        destination: NodeId,
        nodes: Vec<NodeId>,
        links: Vec<LinkId>,
    ) -> Result<NetworkPath, NetworkError> {
        self.validate_endpoint(source)?;
        self.validate_endpoint(destination)?;

        if source == destination {
            return Err(NetworkError::SameEndpoint { node: source });
        }

        if nodes.len() < 2 || links.len().checked_add(1) != Some(nodes.len()) {
            return Err(NetworkError::InvalidPath { position: 0 });
        }

        if nodes.first().copied() != Some(source) {
            return Err(NetworkError::InvalidPath { position: 0 });
        }

        if nodes.last().copied() != Some(destination) {
            return Err(NetworkError::InvalidPath {
                position: nodes.len().saturating_sub(1),
            });
        }

        let mut seen = BTreeSet::new();

        for (index, node) in nodes.iter().enumerate() {
            if !seen.insert(*node) {
                return Err(NetworkError::CyclicPath);
            }

            if !self.nodes.contains_key(node) {
                return Err(NetworkError::NodeNotFound { node: *node });
            }

            if index > 0 {
                let previous = nodes[index - 1];
                let link_id = links[index - 1];

                let link = self
                    .links
                    .get(&link_id)
                    .ok_or(NetworkError::LinkNotFound {
                        link: link_id,
                    })?;

                if !link.permits(previous, *node) {
                    return Err(NetworkError::EndpointMismatch {
                        link: link_id,
                        expected: *node,
                        actual: link
                            .other_endpoint(previous)
                            .unwrap_or(previous),
                    });
                }
            }
        }

        Ok(NetworkPath::new(
            source,
            destination,
            nodes,
            links,
            0,
        )?)
    }

    /// Clears the entire topology.
    ///
    /// External reservations and schedules are not modified. Owners of those
    /// objects must revalidate them against the new topology.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.links.clear();
        self.adjacency.clear();
    }

    fn validate_endpoint(
        &self,
        node: NodeId,
    ) -> Result<(), NetworkError> {
        if self.nodes.contains_key(&node) {
            Ok(())
        } else {
            Err(NetworkError::NodeNotFound { node })
        }
    }

    fn reconstruct_path(
        &self,
        source: NodeId,
        destination: NodeId,
        predecessor: &BTreeMap<NodeId, (NodeId, LinkId)>,
        total_cost: u128,
    ) -> Result<NetworkPath, NetworkError> {
        let mut nodes_reversed = Vec::new();
        let mut links_reversed = Vec::new();

        let mut current = destination;
        nodes_reversed.push(current);

        while current != source {
            let (previous, link_id) = predecessor
                .get(&current)
                .copied()
                .ok_or(NetworkError::NoPath {
                    source,
                    destination,
                })?;

            let link = self
                .links
                .get(&link_id)
                .ok_or(NetworkError::LinkNotFound { link: link_id })?;

            if !link.permits(previous, current) {
                return Err(NetworkError::EndpointMismatch {
                    link: link_id,
                    expected: current,
                    actual: link
                        .other_endpoint(previous)
                        .unwrap_or(previous),
                });
            }

            links_reversed.push(link_id);
            current = previous;
            nodes_reversed.push(current);

            if nodes_reversed.len() > self.nodes.len() {
                return Err(NetworkError::CyclicPath);
            }
        }

        nodes_reversed.reverse();
        links_reversed.reverse();

        NetworkPath::new(
            source,
            destination,
            nodes_reversed,
            links_reversed,
            total_cost,
        )
    }
}

// =============================================================================
// Network builder
// =============================================================================

/// Deterministic builder for a distributed scheduler network.
#[derive(Debug, Default)]
pub struct QuantumNetworkBuilder {
    network: QuantumNetwork,
}

impl QuantumNetworkBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node.
    pub fn node(mut self, node: NetworkNode) -> Result<Self, NetworkError> {
        self.network.add_node(node)?;
        Ok(self)
    }

    /// Adds a node identity.
    pub fn node_id(mut self, id: NodeId) -> Result<Self, NetworkError> {
        self.network.insert_node(id)?;
        Ok(self)
    }

    /// Adds a topology link.
    pub fn link(
        mut self,
        link: NetworkLink,
    ) -> Result<Self, NetworkError> {
        self.network.add_link(link)?;
        Ok(self)
    }

    /// Adds a topology link from canonical identities.
    pub fn link_ids(
        mut self,
        id: LinkId,
        source: NodeId,
        destination: NodeId,
        direction: LinkDirection,
        kind: QuantumLinkKind,
    ) -> Result<Self, NetworkError> {
        self.network.insert_link(
            id,
            source,
            destination,
            direction,
            kind,
        )?;
        Ok(self)
    }

    /// Validates and returns the completed network.
    pub fn build(self) -> Result<QuantumNetwork, NetworkError> {
        self.network.validate()?;
        Ok(self.network)
    }
}

// =============================================================================
// Structural statistics
// =============================================================================

/// Deterministic summary of a distributed network topology.
///
/// This intentionally reports structural quantities only.
///
/// It does not report physical performance metrics because those belong to
/// hardware/link capability models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NetworkStatistics {
    /// Number of nodes.
    pub nodes: usize,

    /// Number of links.
    pub links: usize,

    /// Number of directed links.
    pub directed_links: usize,

    /// Number of bidirectional links.
    pub bidirectional_links: usize,

    /// Number of isolated nodes.
    pub isolated_nodes: usize,
}

impl QuantumNetwork {
    /// Computes deterministic structural statistics.
    #[must_use]
    pub fn statistics(&self) -> NetworkStatistics {
        let mut directed_links = 0usize;
        let mut bidirectional_links = 0usize;

        for link in self.links.values() {
            match link.direction() {
                LinkDirection::Directed => {
                    directed_links = directed_links.saturating_add(1);
                }

                LinkDirection::Bidirectional => {
                    bidirectional_links =
                        bidirectional_links.saturating_add(1);
                }
            }
        }

        let isolated_nodes = self
            .adjacency
            .values()
            .filter(|links| links.is_empty())
            .count();

        NetworkStatistics {
            nodes: self.nodes.len(),
            links: self.links.len(),
            directed_links,
            bidirectional_links,
            isolated_nodes,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn node(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn link(value: u64) -> LinkId {
        LinkId::new(value)
    }

    fn make_network() -> QuantumNetwork {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");
        network.insert_node(node(2)).expect("node");
        network.insert_node(node(3)).expect("node");

        network
            .insert_link(
                link(1),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("link");

        network
            .insert_link(
                link(2),
                node(2),
                node(3),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("link");

        network
    }

    #[test]
    fn empty_network_is_valid() {
        let network = QuantumNetwork::new();

        assert!(network.is_empty());
        assert_eq!(network.node_count(), 0);
        assert_eq!(network.link_count(), 0);
        assert!(network.validate().is_ok());
    }

    #[test]
    fn node_identity_is_unique() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("first node");

        let result = network.insert_node(node(1));

        assert_eq!(
            result,
            Err(NetworkError::DuplicateNode { node: node(1) })
        );
    }

    #[test]
    fn link_requires_existing_endpoints() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");

        let result = network.insert_link(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        );

        assert_eq!(
            result,
            Err(NetworkError::EndpointNotFound { node: node(2) })
        );
    }

    #[test]
    fn self_link_is_rejected() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");

        let result = network.insert_link(
            link(1),
            node(1),
            node(1),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        );

        assert_eq!(
            result,
            Err(NetworkError::SelfLink { node: node(1) })
        );
    }

    #[test]
    fn neighbors_respect_bidirectional_links() {
        let network = make_network();

        let neighbors = network.neighbors(node(2)).expect("neighbors");

        assert!(neighbors.contains(&node(1)));
        assert!(neighbors.contains(&node(3)));
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn directed_links_respect_direction() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");
        network.insert_node(node(2)).expect("node");

        network
            .insert_link(
                link(1),
                node(1),
                node(2),
                LinkDirection::Directed,
                QuantumLinkKind::Classical,
            )
            .expect("link");

        assert!(network.connected(node(1), node(2)));
        assert!(!network.connected(node(2), node(1)));

        let neighbors = network.neighbors(node(2)).expect("neighbors");
        assert!(neighbors.is_empty());
    }

    #[test]
    fn shortest_path_finds_multihop_route() {
        let network = make_network();

        let path = network
            .shortest_path(node(1), node(3))
            .expect("path");

        assert_eq!(path.source(), node(1));
        assert_eq!(path.destination(), node(3));
        assert_eq!(path.nodes(), &[node(1), node(2), node(3)]);
        assert_eq!(path.links(), &[link(1), link(2)]);
        assert_eq!(path.hop_count(), 2);
        assert_eq!(path.cost(), 2);
    }

    #[test]
    fn no_path_is_reported() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");
        network.insert_node(node(2)).expect("node");

        let result = network.shortest_path(node(1), node(2));

        assert_eq!(
            result,
            Err(NetworkError::NoPath {
                source: node(1),
                destination: node(2),
            })
        );
    }

    #[test]
    fn parallel_links_are_supported() {
        let mut network = QuantumNetwork::new();

        network.insert_node(node(1)).expect("node");
        network.insert_node(node(2)).expect("node");

        network
            .insert_link(
                link(1),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("link");

        network
            .insert_link(
                link(2),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Classical,
            )
            .expect("parallel link");

        let links = network
            .links_between(node(1), node(2))
            .expect("links");

        assert_eq!(links.len(), 2);
        assert!(links.contains(&link(1)));
        assert!(links.contains(&link(2)));
    }

    #[test]
    fn caller_supplied_path_is_validated() {
        let network = make_network();

        let path = network
            .validate_path(
                node(1),
                node(3),
                vec![node(1), node(2), node(3)],
                vec![link(1), link(2)],
            )
            .expect("valid path");

        assert_eq!(path.nodes(), &[node(1), node(2), node(3)]);
    }

    #[test]
    fn cyclic_path_is_rejected() {
        let network = make_network();

        let result = network.validate_path(
            node(1),
            node(3),
            vec![node(1), node(2), node(1), node(3)],
            vec![link(1), link(1), link(2)],
        );

        assert_eq!(result, Err(NetworkError::CyclicPath));
    }

    #[test]
    fn unreachable_nodes_are_not_reported() {
        let mut network = make_network();

        network.insert_node(node(99)).expect("isolated node");

        let reachable = network
            .reachable_nodes(node(1))
            .expect("reachable nodes");

        assert!(reachable.contains(&node(1)));
        assert!(reachable.contains(&node(2)));
        assert!(reachable.contains(&node(3)));
        assert!(!reachable.contains(&node(99)));
    }

    #[test]
    fn statistics_are_deterministic() {
        let network = make_network();

        let statistics = network.statistics();

        assert_eq!(statistics.nodes, 3);
        assert_eq!(statistics.links, 2);
        assert_eq!(statistics.bidirectional_links, 2);
        assert_eq!(statistics.directed_links, 0);
        assert_eq!(statistics.isolated_nodes, 0);
    }

    #[test]
    fn builder_produces_valid_network() {
        let network = QuantumNetworkBuilder::new()
            .node_id(node(1))
            .expect("node")
            .node_id(node(2))
            .expect("node")
            .link_ids(
                link(1),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("link")
            .build()
            .expect("network");

        assert_eq!(network.node_count(), 2);
        assert_eq!(network.link_count(), 1);
        assert!(network.validate().is_ok());
    }

    #[test]
    fn traversal_policy_is_optional() {
        let network = make_network();

        let result = network.reachable_nodes_with_policy(
            node(1),
            TraversalPolicy::with_max_visited_nodes(1),
        );

        assert_eq!(
            result,
            Err(NetworkError::TraversalLimitExceeded)
        );
    }

    #[test]
    fn zero_cost_traversal_is_supported() {
        let network = make_network();

        let path = network
            .shortest_path_with_cost(
                node(1),
                node(3),
                &ZeroCost,
                TraversalPolicy::unrestricted(),
            )
            .expect("path");

        assert_eq!(path.cost(), 0);
        assert_eq!(
            path.nodes(),
            &[node(1), node(2), node(3)]
        );
    }
}