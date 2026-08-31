//! Zamani Quantum IR — Distributed Quantum Computation Model.
//!
//! Canonical, hardware-independent representation of distributed quantum
//! computation.
//!
//! # Architectural role
//!
//! This module answers:
//!
//! > "What distributed quantum computation does the program semantically
//! > require?"
//!
//! It represents:
//!
//! - distributed quantum nodes;
//! - logical qubits associated with distributed locations;
//! - physical-qubit references when explicitly required by a lowered IR;
//! - quantum communication links;
//! - link endpoints;
//! - entanglement resources;
//! - remote qubit references;
//! - quantum state transfer requirements;
//! - teleportation semantics;
//! - remote quantum operations;
//! - distributed measurements;
//! - classical communication dependencies;
//! - entanglement-generation requirements;
//! - communication/resource constraints;
//! - distributed execution domains;
//! - deterministic sparse topology descriptions;
//! - extensible technology-neutral distributed operations.
//!
//! # This module does NOT own
//!
//! This module does not:
//!
//! - discover hardware;
//! - allocate hardware;
//! - choose a physical machine;
//! - perform routing;
//! - perform network routing;
//! - schedule operations;
//! - generate pulses;
//! - perform calibration;
//! - execute a QPU;
//! - establish real network connections;
//! - transmit classical messages;
//! - generate entanglement;
//! - perform teleportation;
//! - simulate quantum states;
//! - decode quantum error-correction syndromes;
//! - select a vendor;
//! - impose a maximum number of nodes;
//! - impose a maximum number of qubits;
//! - impose a maximum number of links;
//! - impose a fixed network topology.
//!
//! Those responsibilities belong to downstream routing, hardware, scheduler,
//! backend, runtime, simulator and networking subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! The same distributed IR can describe:
//!
//! ```text
//! one node
//! two nodes
//! many nodes
//! sparse networks
//! dense networks
//! hierarchical networks
//! dynamically discovered networks
//! planetary-scale distributed systems
//! future quantum-network architectures
//! ```
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_NODES
//! MAX_LINKS
//! MAX_REMOTE_QUBITS
//! MAX_NETWORK_DISTANCE
//! ```
//!
//! Practical limits are supplied by:
//!
//! - compiler/resource policy;
//! - host resources;
//! - target capabilities;
//! - execution environment;
//! - networking infrastructure.
//!
//! They are not semantic limits of this model.
//!
//! # Canonical qubit identity
//!
//! Logical and physical qubits are intentionally different types.
//!
//! New code must use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A logical qubit represents semantic program identity.
//!
//! A physical qubit identifies a physical target resource only when a
//! downstream mapping has explicitly introduced one.
//!
//! # Distributed-program model
//!
//! The model is deliberately sparse:
//!
//! ```text
//! DistributedProgram
//! │
//! ├── nodes
//! ├── links
//! ├── logical-qubit locations
//! ├── entanglement resources
//! ├── classical channels
//! └── distributed operations
//! ```
//!
//! No dense matrix, adjacency matrix or fixed-size array is required.
//!
//! This allows a program containing a tiny distributed system and a program
//! containing a very large sparse network to use the same representation.
//!
//! # Semantic distinction
//!
//! ```text
//! Node
//!     abstract distributed execution domain
//!
//! Link
//!     abstract communication/entanglement relationship
//!
//! QubitLocation
//!     semantic association of a logical qubit with a node
//!
//! RemoteQubit
//!     semantic reference to a qubit outside the current local domain
//!
//! EntanglementResource
//!     semantic requirement/reference to shared entanglement
//!
//! Teleportation
//!     semantic state-transfer operation
//!
//! RemoteOperation
//!     semantic operation whose operands span distributed domains
//!
//! ClassicalChannel
//!     semantic classical communication relationship
//! ```
//!
//! # Integration contract
//!
//! This file intentionally depends only on:
//!
//! - `quantum::ir::qubit::{QubitId, PhysicalQubitId}`;
//! - Rust's standard library.
//!
//! It does not depend on routing, hardware, scheduling, backend, simulator,
//! QEC or networking implementations.
//!
//! Therefore downstream components can be added or changed without requiring
//! this file to be modified.
//!
//! Expected consumers include:
//!
//! - `quantum::ir::program`;
//! - `quantum::ir::operation`;
//! - `quantum::ir::resource`;
//! - `quantum::ir::capability`;
//! - `quantum::ir::validation`;
//! - `quantum::ir::analysis`;
//! - `quantum::routing`;
//! - `quantum::hardware`;
//! - `quantum::scheduling`;
//! - `quantum::qec`;
//! - distributed runtimes;
//! - backend adapters.
//!
//! # Determinism
//!
//! Ordered collections are used for semantic collections so iteration order
//! is deterministic.
//!
//! No `HashMap` or unordered collection is used for canonical topology or
//! operation storage.
//!
//! # Safety
//!
//! No unsafe Rust is permitted.
//!
//! `#![forbid(unsafe_code)]` makes this compiler-enforced.
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
//! - no external dependencies.
//!
//! # Important semantic rule
//!
//! Creating a node, link, physical qubit reference or entanglement resource
//! does not assert that the corresponding real-world resource exists.
//!
//! Hardware validation occurs downstream.
//!
//! # Security/resource rule
//!
//! All potentially overflowing arithmetic is checked.
//!
//! Identifiers are opaque typed values.
//!
//! Semantic identity is never inferred from collection position.
//!
//! No fixed-size resource assumption exists in this file.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Identifier types
// ============================================================================

/// Stable identifier for an abstract distributed execution node.
///
/// A node is a semantic location/domain, not necessarily a physical machine.
///
/// The value has no prescribed relationship with array indexes, IP addresses,
/// hostnames or vendor identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(u64);

impl NodeId {
    /// Creates a node identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identifier.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for NodeId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<NodeId> for u64 {
    fn from(value: NodeId) -> Self {
        value.value()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "node{}", self.0)
    }
}

/// Stable identifier for a distributed quantum communication link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LinkId(u64);

impl LinkId {
    /// Creates a link identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identifier.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for LinkId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<LinkId> for u64 {
    fn from(value: LinkId) -> Self {
        value.value()
    }
}

impl fmt::Display for LinkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "link{}", self.0)
    }
}

/// Stable identifier for an entanglement resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntanglementResourceId(u64);

impl EntanglementResourceId {
    /// Creates an entanglement-resource identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for EntanglementResourceId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<EntanglementResourceId> for u64 {
    fn from(value: EntanglementResourceId) -> Self {
        value.value()
    }
}

impl fmt::Display for EntanglementResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "entanglement{}", self.0)
    }
}

/// Stable identifier for a distributed classical communication channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalChannelId(u64);

impl ClassicalChannelId {
    /// Creates a classical-channel identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for ClassicalChannelId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalChannelId> for u64 {
    fn from(value: ClassicalChannelId) -> Self {
        value.value()
    }
}

impl fmt::Display for ClassicalChannelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical-channel{}", self.0)
    }
}

/// Stable identifier for a distributed operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DistributedOperationId(u64);

impl DistributedOperationId {
    /// Creates an operation identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for DistributedOperationId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<DistributedOperationId> for u64 {
    fn from(value: DistributedOperationId) -> Self {
        value.value()
    }
}

impl fmt::Display for DistributedOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "distributed-op{}", self.0)
    }
}

// ============================================================================
// Resource quantities
// ============================================================================

/// A finite or explicitly unbounded distributed resource quantity.
///
/// `Unbounded` is semantic and must never be represented using `u64::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedQuantity {
    /// A finite amount.
    Finite(u64),

    /// No finite upper bound is expressed.
    Unbounded,
}

impl DistributedQuantity {
    /// Creates a finite quantity.
    #[must_use]
    pub const fn finite(value: u64) -> Self {
        Self::Finite(value)
    }

    /// Creates an unbounded quantity.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Returns whether this quantity is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns a finite value when available.
    #[must_use]
    pub const fn as_finite(self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Checked addition.
    pub const fn checked_add(self, rhs: Self) -> Result<Self, DistributedError> {
        match (self, rhs) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Ok(Self::Unbounded),
            (Self::Finite(lhs), Self::Finite(rhs)) => match lhs.checked_add(rhs) {
                Some(value) => Ok(Self::Finite(value)),
                None => Err(DistributedError::ArithmeticOverflow),
            },
        }
    }
}

impl Default for DistributedQuantity {
    fn default() -> Self {
        Self::Finite(0)
    }
}

// ============================================================================
// Node
// ============================================================================

/// Abstract distributed execution node.
///
/// A node is intentionally technology-neutral.
///
/// It may eventually correspond to:
///
/// - a QPU;
/// - a logical quantum computer;
/// - a quantum memory;
/// - a network repeater;
/// - a simulator partition;
/// - an edge quantum processor;
/// - another future execution domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumNode {
    id: NodeId,
    label: Option<String>,
}

impl QuantumNode {
    /// Creates a node with no descriptive label.
    #[must_use]
    pub const fn new(id: NodeId) -> Self {
        Self { id, label: None }
    }

    /// Creates a node with a semantic label.
    #[must_use]
    pub fn with_label(id: NodeId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: Some(label.into()),
        }
    }

    /// Returns the node identifier.
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

// ============================================================================
// Link endpoints
// ============================================================================

/// Endpoint of a distributed link.
///
/// The endpoint is a semantic node reference. It does not specify a network
/// interface, optical port, frequency, address, cable, satellite or vendor
/// resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LinkEndpoint {
    node: NodeId,
}

impl LinkEndpoint {
    /// Creates an endpoint.
    #[must_use]
    pub const fn new(node: NodeId) -> Self {
        Self { node }
    }

    /// Returns the endpoint node.
    #[must_use]
    pub const fn node(self) -> NodeId {
        self.node
    }
}

/// Directionality of a quantum communication link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LinkDirection {
    /// Communication/entanglement is conceptually bidirectional.
    Bidirectional,

    /// Communication/entanglement is directed from source to destination.
    Directed,
}

/// Semantic kind of distributed link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QuantumLinkKind {
    /// Link whose primary semantic role is quantum communication.
    Quantum,

    /// Link whose primary semantic role is entanglement distribution.
    Entanglement,

    /// Link supporting both quantum communication and entanglement.
    Hybrid,

    /// Technology-independent custom link.
    Custom,
}

/// Abstract quantum communication link.
///
/// No physical networking implementation is embedded here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumLink {
    id: LinkId,
    source: LinkEndpoint,
    destination: LinkEndpoint,
    direction: LinkDirection,
    kind: QuantumLinkKind,
    capacity: DistributedQuantity,
}

impl QuantumLink {
    /// Creates a link.
    ///
    /// A self-link is rejected because distributed communication semantics
    /// require distinct endpoint domains. Local operations should remain local.
    pub fn new(
        id: LinkId,
        source: NodeId,
        destination: NodeId,
        direction: LinkDirection,
        kind: QuantumLinkKind,
    ) -> Result<Self, DistributedError> {
        if source == destination {
            return Err(DistributedError::SelfLink { node: source });
        }

        Ok(Self {
            id,
            source: LinkEndpoint::new(source),
            destination: LinkEndpoint::new(destination),
            direction,
            kind,
            capacity: DistributedQuantity::Unbounded,
        })
    }

    /// Sets an abstract finite or unbounded capacity.
    #[must_use]
    pub const fn with_capacity(mut self, capacity: DistributedQuantity) -> Self {
        self.capacity = capacity;
        self
    }

    /// Returns the link identifier.
    #[must_use]
    pub const fn id(&self) -> LinkId {
        self.id
    }

    /// Returns the source endpoint.
    #[must_use]
    pub const fn source(&self) -> LinkEndpoint {
        self.source
    }

    /// Returns the destination endpoint.
    #[must_use]
    pub const fn destination(&self) -> LinkEndpoint {
        self.destination
    }

    /// Returns the link direction.
    #[must_use]
    pub const fn direction(&self) -> LinkDirection {
        self.direction
    }

    /// Returns the link kind.
    #[must_use]
    pub const fn kind(&self) -> QuantumLinkKind {
        self.kind
    }

    /// Returns the abstract capacity.
    #[must_use]
    pub const fn capacity(&self) -> DistributedQuantity {
        self.capacity
    }

    /// Returns whether the link connects the supplied node pair.
    #[must_use]
    pub const fn connects(self, lhs: NodeId, rhs: NodeId) -> bool {
        match self.direction {
            LinkDirection::Bidirectional => {
                (self.source.node() == lhs && self.destination.node() == rhs)
                    || (self.source.node() == rhs && self.destination.node() == lhs)
            }
            LinkDirection::Directed => {
                self.source.node() == lhs && self.destination.node() == rhs
            }
        }
    }

    /// Returns whether the supplied node can be the source of a transfer.
    #[must_use]
    pub const fn can_transfer_from(self, node: NodeId) -> bool {
        match self.direction {
            LinkDirection::Bidirectional => {
                self.source.node() == node || self.destination.node() == node
            }
            LinkDirection::Directed => self.source.node() == node,
        }
    }
}

// ============================================================================
// Logical-qubit locations
// ============================================================================

/// Semantic location of a logical qubit.
///
/// A qubit can be:
///
/// - local to a node;
/// - explicitly remote;
/// - unplaced.
///
/// `Unplaced` is useful before distributed placement/routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitLocation {
    /// Qubit is semantically associated with a node.
    Local(NodeId),

    /// Qubit is known to be remote relative to a node.
    Remote(NodeId),

    /// Placement has not yet been selected.
    Unplaced,
}

impl QubitLocation {
    /// Returns the node if one is explicitly associated.
    #[must_use]
    pub const fn node(self) -> Option<NodeId> {
        match self {
            Self::Local(node) | Self::Remote(node) => Some(node),
            Self::Unplaced => None,
        }
    }

    /// Returns whether this is a local location.
    #[must_use]
    pub const fn is_local(self) -> bool {
        matches!(self, Self::Local(_))
    }

    /// Returns whether this is a remote location.
    #[must_use]
    pub const fn is_remote(self) -> bool {
        matches!(self, Self::Remote(_))
    }

    /// Returns whether placement is unresolved.
    #[must_use]
    pub const fn is_unplaced(self) -> bool {
        matches!(self, Self::Unplaced)
    }
}

/// Logical qubit plus distributed location.
///
/// This deliberately uses the canonical `QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DistributedQubit {
    qubit: QubitId,
    location: QubitLocation,
}

impl DistributedQubit {
    /// Creates an unplaced distributed qubit.
    #[must_use]
    pub const fn unplaced(qubit: QubitId) -> Self {
        Self {
            qubit,
            location: QubitLocation::Unplaced,
        }
    }

    /// Creates a locally placed distributed qubit.
    #[must_use]
    pub const fn local(qubit: QubitId, node: NodeId) -> Self {
        Self {
            qubit,
            location: QubitLocation::Local(node),
        }
    }

    /// Creates a remotely referenced distributed qubit.
    #[must_use]
    pub const fn remote(qubit: QubitId, node: NodeId) -> Self {
        Self {
            qubit,
            location: QubitLocation::Remote(node),
        }
    }

    /// Returns the canonical logical qubit identity.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the distributed location.
    #[must_use]
    pub const fn location(self) -> QubitLocation {
        self.location
    }
}

// ============================================================================
// Remote qubit
// ============================================================================

/// Explicit remote-qubit reference.
///
/// This is preferable to representing remote qubits by raw node/qubit pairs
/// throughout the rest of the distributed model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoteQubit {
    qubit: QubitId,
    home: NodeId,
    accessed_from: NodeId,
}

impl RemoteQubit {
    /// Creates a remote reference.
    pub fn new(
        qubit: QubitId,
        home: NodeId,
        accessed_from: NodeId,
    ) -> Result<Self, DistributedError> {
        if home == accessed_from {
            return Err(DistributedError::RemoteReferenceIsLocal {
                qubit,
                node: home,
            });
        }

        Ok(Self {
            qubit,
            home,
            accessed_from,
        })
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns the qubit's home node.
    #[must_use]
    pub const fn home(self) -> NodeId {
        self.home
    }

    /// Returns the node from which it is being referenced.
    #[must_use]
    pub const fn accessed_from(self) -> NodeId {
        self.accessed_from
    }
}

// ============================================================================
// Entanglement
// ============================================================================

/// Semantic quality information for an entanglement resource.
///
/// No specific physical fidelity model is assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntanglementQuality {
    /// Optional minimum quality expressed in arbitrary canonical units.
    ///
    /// The interpretation is defined by the associated dialect/capability
    /// contract rather than by this model.
    minimum: Option<u64>,
}

impl EntanglementQuality {
    /// Creates unspecified quality.
    #[must_use]
    pub const fn unspecified() -> Self {
        Self { minimum: None }
    }

    /// Creates a minimum quality requirement.
    #[must_use]
    pub const fn minimum(value: u64) -> Self {
        Self {
            minimum: Some(value),
        }
    }

    /// Returns the optional minimum.
    #[must_use]
    pub const fn minimum_value(self) -> Option<u64> {
        self.minimum
    }
}

impl Default for EntanglementQuality {
    fn default() -> Self {
        Self::unspecified()
    }
}

/// Semantic lifecycle of an entanglement resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntanglementState {
    /// Resource is requested but not yet established.
    Required,

    /// Resource has been established semantically.
    Available,

    /// Resource has been consumed.
    Consumed,

    /// Resource is invalidated.
    Invalidated,
}

/// A semantic entanglement resource.
///
/// It describes a required/shared resource without creating it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntanglementResource {
    id: EntanglementResourceId,
    endpoints: (NodeId, NodeId),
    pairs: DistributedQuantity,
    quality: EntanglementQuality,
    state: EntanglementState,
}

impl EntanglementResource {
    /// Creates an entanglement requirement between two distinct nodes.
    pub fn new(
        id: EntanglementResourceId,
        first: NodeId,
        second: NodeId,
    ) -> Result<Self, DistributedError> {
        if first == second {
            return Err(DistributedError::SelfEntanglement { node: first });
        }

        let endpoints = if first <= second {
            (first, second)
        } else {
            (second, first)
        };

        Ok(Self {
            id,
            endpoints,
            pairs: DistributedQuantity::finite(1),
            quality: EntanglementQuality::unspecified(),
            state: EntanglementState::Required,
        })
    }

    /// Sets the required number of entangled pairs.
    #[must_use]
    pub const fn with_pairs(mut self, pairs: DistributedQuantity) -> Self {
        self.pairs = pairs;
        self
    }

    /// Sets an entanglement quality requirement.
    #[must_use]
    pub const fn with_quality(mut self, quality: EntanglementQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Sets the semantic lifecycle state.
    #[must_use]
    pub const fn with_state(mut self, state: EntanglementState) -> Self {
        self.state = state;
        self
    }

    /// Returns the resource ID.
    #[must_use]
    pub const fn id(&self) -> EntanglementResourceId {
        self.id
    }

    /// Returns both endpoint nodes in deterministic order.
    #[must_use]
    pub const fn endpoints(&self) -> (NodeId, NodeId) {
        self.endpoints
    }

    /// Returns the pair quantity.
    #[must_use]
    pub const fn pairs(&self) -> DistributedQuantity {
        self.pairs
    }

    /// Returns quality requirements.
    #[must_use]
    pub const fn quality(&self) -> EntanglementQuality {
        self.quality
    }

    /// Returns lifecycle state.
    #[must_use]
    pub const fn state(&self) -> EntanglementState {
        self.state
    }
}

// ============================================================================
// Classical communication
// ============================================================================

/// Semantic direction of a classical communication channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalChannelDirection {
    /// Both endpoints may communicate.
    Bidirectional,

    /// Communication is from source to destination.
    Directed,
}

/// Abstract distributed classical communication channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalChannel {
    id: ClassicalChannelId,
    source: NodeId,
    destination: NodeId,
    direction: ClassicalChannelDirection,
}

impl ClassicalChannel {
    /// Creates a classical channel.
    pub fn new(
        id: ClassicalChannelId,
        source: NodeId,
        destination: NodeId,
        direction: ClassicalChannelDirection,
    ) -> Result<Self, DistributedError> {
        if source == destination {
            return Err(DistributedError::SelfClassicalChannel { node: source });
        }

        Ok(Self {
            id,
            source,
            destination,
            direction,
        })
    }

    /// Returns the channel ID.
    #[must_use]
    pub const fn id(self) -> ClassicalChannelId {
        self.id
    }

    /// Returns source.
    #[must_use]
    pub const fn source(self) -> NodeId {
        self.source
    }

    /// Returns destination.
    #[must_use]
    pub const fn destination(self) -> NodeId {
        self.destination
    }

    /// Returns direction.
    #[must_use]
    pub const fn direction(self) -> ClassicalChannelDirection {
        self.direction
    }
}

// ============================================================================
// Distributed operation operands
// ============================================================================

/// Operand participating in a distributed operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedOperand {
    /// Local logical qubit.
    Qubit(QubitId),

    /// Explicit remote qubit.
    RemoteQubit(RemoteQubit),

    /// Physical qubit reference introduced by a lower-level IR.
    PhysicalQubit(PhysicalQubitId),

    /// Existing entanglement resource.
    Entanglement(EntanglementResourceId),

    /// Distributed node reference.
    Node(NodeId),

    /// Quantum communication link.
    Link(LinkId),

    /// Classical communication channel.
    ClassicalChannel(ClassicalChannelId),
}

// ============================================================================
// Distributed operation kinds
// ============================================================================

/// Standard semantic distributed-operation kinds.
///
/// This is deliberately not the complete universe of distributed operations.
/// Technology-specific or future operations can use `Custom`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedOperationKind {
    /// Establish or request shared entanglement.
    GenerateEntanglement,

    /// Consume entanglement to transfer an unknown quantum state.
    Teleport,

    /// Move a logical qubit between distributed domains.
    TransferQubit,

    /// Apply an operation whose operands span distributed domains.
    RemoteOperation,

    /// Perform a distributed measurement.
    DistributedMeasurement,

    /// Consume entanglement without specifying the physical protocol.
    ConsumeEntanglement,

    /// Synchronize distributed quantum domains.
    Synchronize,

    /// Technology-neutral custom distributed operation.
    Custom(String),
}

impl DistributedOperationKind {
    /// Returns whether this operation requires a quantum communication
    /// relationship.
    #[must_use]
    pub fn requires_quantum_link(&self) -> bool {
        matches!(
            self,
            Self::GenerateEntanglement
                | Self::Teleport
                | Self::TransferQubit
                | Self::RemoteOperation
                | Self::ConsumeEntanglement
        )
    }

    /// Returns whether this operation may require classical communication.
    #[must_use]
    pub fn requires_classical_communication(&self) -> bool {
        matches!(
            self,
            Self::Teleport
                | Self::RemoteOperation
                | Self::DistributedMeasurement
                | Self::Synchronize
        )
    }
}

// ============================================================================
// Distributed operation
// ============================================================================

/// A semantic distributed operation.
///
/// It describes intent, not execution.
///
/// For example, a teleportation operation means:
///
/// ```text
/// transfer the quantum state associated with source qubit
/// from source domain to destination domain
/// using the declared/required distributed resources
/// ```
///
/// It does not specify:
///
/// - Bell-state preparation implementation;
/// - physical communication protocol;
/// - photon frequency;
/// - repeater protocol;
/// - pulse sequence;
/// - hardware device;
/// - network packet format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedOperation {
    id: DistributedOperationId,
    kind: DistributedOperationKind,
    operands: Vec<DistributedOperand>,
    entanglement: Vec<EntanglementResourceId>,
    quantum_links: Vec<LinkId>,
    classical_channels: Vec<ClassicalChannelId>,
    source_node: Option<NodeId>,
    destination_node: Option<NodeId>,
}

impl DistributedOperation {
    /// Creates an operation with no operands or explicit resources.
    #[must_use]
    pub fn new(id: DistributedOperationId, kind: DistributedOperationKind) -> Self {
        Self {
            id,
            kind,
            operands: Vec::new(),
            entanglement: Vec::new(),
            quantum_links: Vec::new(),
            classical_channels: Vec::new(),
            source_node: None,
            destination_node: None,
        }
    }

    /// Adds an operand.
    #[must_use]
    pub fn with_operand(mut self, operand: DistributedOperand) -> Self {
        self.operands.push(operand);
        self
    }

    /// Adds an entanglement resource.
    #[must_use]
    pub fn with_entanglement(mut self, resource: EntanglementResourceId) -> Self {
        self.entanglement.push(resource);
        self
    }

    /// Adds a quantum link requirement.
    #[must_use]
    pub fn with_quantum_link(mut self, link: LinkId) -> Self {
        self.quantum_links.push(link);
        self
    }

    /// Adds a classical channel requirement.
    #[must_use]
    pub fn with_classical_channel(mut self, channel: ClassicalChannelId) -> Self {
        self.classical_channels.push(channel);
        self
    }

    /// Sets the source node.
    #[must_use]
    pub const fn with_source_node(mut self, node: NodeId) -> Self {
        self.source_node = Some(node);
        self
    }

    /// Sets the destination node.
    #[must_use]
    pub const fn with_destination_node(mut self, node: NodeId) -> Self {
        self.destination_node = Some(node);
        self
    }

    /// Returns operation ID.
    #[must_use]
    pub const fn id(&self) -> DistributedOperationId {
        self.id
    }

    /// Returns operation kind.
    #[must_use]
    pub const fn kind(&self) -> &DistributedOperationKind {
        &self.kind
    }

    /// Returns operation operands.
    #[must_use]
    pub fn operands(&self) -> &[DistributedOperand] {
        &self.operands
    }

    /// Returns required entanglement resources.
    #[must_use]
    pub fn entanglement(&self) -> &[EntanglementResourceId] {
        &self.entanglement
    }

    /// Returns required quantum links.
    #[must_use]
    pub fn quantum_links(&self) -> &[LinkId] {
        &self.quantum_links
    }

    /// Returns required classical channels.
    #[must_use]
    pub fn classical_channels(&self) -> &[ClassicalChannelId] {
        &self.classical_channels
    }

    /// Returns optional source node.
    #[must_use]
    pub const fn source_node(&self) -> Option<NodeId> {
        self.source_node
    }

    /// Returns optional destination node.
    #[must_use]
    pub const fn destination_node(&self) -> Option<NodeId> {
        self.destination_node
    }
}

// ============================================================================
// Distributed operation dependency
// ============================================================================

/// Explicit dependency between distributed operations.
///
/// The IR represents the dependency; a scheduler decides when it is
/// satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DistributedDependency {
    predecessor: DistributedOperationId,
    successor: DistributedOperationId,
}

impl DistributedDependency {
    /// Creates a dependency.
    pub fn new(
        predecessor: DistributedOperationId,
        successor: DistributedOperationId,
    ) -> Result<Self, DistributedError> {
        if predecessor == successor {
            return Err(DistributedError::SelfDependency {
                operation: predecessor,
            });
        }

        Ok(Self {
            predecessor,
            successor,
        })
    }

    /// Returns predecessor.
    #[must_use]
    pub const fn predecessor(self) -> DistributedOperationId {
        self.predecessor
    }

    /// Returns successor.
    #[must_use]
    pub const fn successor(self) -> DistributedOperationId {
        self.successor
    }
}

// ============================================================================
// Transfer semantics
// ============================================================================

/// Semantic mode for moving a quantum state between nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransferMode {
    /// Teleportation-style transfer requiring shared entanglement and
    /// classical feed-forward.
    Teleportation,

    /// Abstract direct quantum state transfer.
    ///
    /// Whether a physical target can perform this is a capability question.
    Direct,

    /// Technology-specific transfer semantics.
    Custom,
}

/// Quantum state-transfer requirement.
///
/// This is semantic. It does not contain a physical protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuantumTransfer {
    qubit: QubitId,
    source: NodeId,
    destination: NodeId,
    mode: TransferMode,
}

impl QuantumTransfer {
    /// Creates a state-transfer requirement.
    pub fn new(
        qubit: QubitId,
        source: NodeId,
        destination: NodeId,
        mode: TransferMode,
    ) -> Result<Self, DistributedError> {
        if source == destination {
            return Err(DistributedError::LocalTransfer {
                qubit,
                node: source,
            });
        }

        Ok(Self {
            qubit,
            source,
            destination,
            mode,
        })
    }

    /// Returns the logical qubit being transferred.
    #[must_use]
    pub const fn qubit(self) -> QubitId {
        self.qubit
    }

    /// Returns source.
    #[must_use]
    pub const fn source(self) -> NodeId {
        self.source
    }

    /// Returns destination.
    #[must_use]
    pub const fn destination(self) -> NodeId {
        self.destination
    }

    /// Returns transfer mode.
    #[must_use]
    pub const fn mode(self) -> TransferMode {
        self.mode
    }
}

// ============================================================================
// Distributed program
// ============================================================================

/// Canonical distributed quantum program model.
///
/// The structure is sparse and deterministic.
///
/// It is suitable for:
///
/// - a single-node degenerate distributed program;
/// - a two-node protocol;
/// - a large sparse quantum network;
/// - dynamically selected target networks.
///
/// It does not allocate a dense node-by-node topology matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedProgram {
    nodes: BTreeMap<NodeId, QuantumNode>,
    links: BTreeMap<LinkId, QuantumLink>,
    classical_channels: BTreeMap<ClassicalChannelId, ClassicalChannel>,
    qubits: BTreeMap<QubitId, DistributedQubit>,
    entanglement: BTreeMap<EntanglementResourceId, EntanglementResource>,
    operations: BTreeMap<DistributedOperationId, DistributedOperation>,
    dependencies: BTreeSet<DistributedDependency>,
    transfers: BTreeMap<QubitId, QuantumTransfer>,
}

impl Default for DistributedProgram {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedProgram {
    /// Creates an empty distributed program.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            links: BTreeMap::new(),
            classical_channels: BTreeMap::new(),
            qubits: BTreeMap::new(),
            entanglement: BTreeMap::new(),
            operations: BTreeMap::new(),
            dependencies: BTreeSet::new(),
            transfers: BTreeMap::new(),
        }
    }

    /// Adds a node.
    ///
    /// Duplicate identifiers are rejected.
    pub fn add_node(&mut self, node: QuantumNode) -> Result<(), DistributedError> {
        let id = node.id();

        if self.nodes.contains_key(&id) {
            return Err(DistributedError::DuplicateNode { node: id });
        }

        self.nodes.insert(id, node);
        Ok(())
    }

    /// Adds a quantum link.
    ///
    /// Both endpoints must already exist.
    pub fn add_link(&mut self, link: QuantumLink) -> Result<(), DistributedError> {
        let id = link.id();

        if self.links.contains_key(&id) {
            return Err(DistributedError::DuplicateLink { link: id });
        }

        self.require_node(link.source().node())?;
        self.require_node(link.destination().node())?;

        self.links.insert(id, link);
        Ok(())
    }

    /// Adds a classical channel.
    ///
    /// Both endpoints must already exist.
    pub fn add_classical_channel(
        &mut self,
        channel: ClassicalChannel,
    ) -> Result<(), DistributedError> {
        let id = channel.id();

        if self.classical_channels.contains_key(&id) {
            return Err(DistributedError::DuplicateClassicalChannel { channel: id });
        }

        self.require_node(channel.source())?;
        self.require_node(channel.destination())?;

        self.classical_channels.insert(id, channel);
        Ok(())
    }

    /// Adds a logical qubit.
    ///
    /// The referenced node must exist when the qubit is explicitly placed.
    pub fn add_qubit(&mut self, qubit: DistributedQubit) -> Result<(), DistributedError> {
        let id = qubit.qubit();

        if self.qubits.contains_key(&id) {
            return Err(DistributedError::DuplicateQubit { qubit: id });
        }

        if let Some(node) = qubit.location().node() {
            self.require_node(node)?;
        }

        self.qubits.insert(id, qubit);
        Ok(())
    }

    /// Adds an entanglement resource.
    ///
    /// Both endpoints must exist.
    pub fn add_entanglement(
        &mut self,
        resource: EntanglementResource,
    ) -> Result<(), DistributedError> {
        let id = resource.id();

        if self.entanglement.contains_key(&id) {
            return Err(DistributedError::DuplicateEntanglement { resource: id });
        }

        let (first, second) = resource.endpoints();

        self.require_node(first)?;
        self.require_node(second)?;

        self.entanglement.insert(id, resource);
        Ok(())
    }

    /// Adds a distributed operation.
    ///
    /// Referenced nodes, links, entanglement resources and channels must
    /// already exist.
    pub fn add_operation(
        &mut self,
        operation: DistributedOperation,
    ) -> Result<(), DistributedError> {
        let id = operation.id();

        if self.operations.contains_key(&id) {
            return Err(DistributedError::DuplicateOperation { operation: id });
        }

        if let Some(node) = operation.source_node() {
            self.require_node(node)?;
        }

        if let Some(node) = operation.destination_node() {
            self.require_node(node)?;
        }

        for operand in operation.operands() {
            self.validate_operand(*operand)?;
        }

        for link in operation.quantum_links() {
            self.require_link(*link)?;
        }

        for resource in operation.entanglement() {
            self.require_entanglement(*resource)?;
        }

        for channel in operation.classical_channels() {
            self.require_classical_channel(*channel)?;
        }

        self.operations.insert(id, operation);
        Ok(())
    }

    /// Adds an explicit operation dependency.
    pub fn add_dependency(
        &mut self,
        dependency: DistributedDependency,
    ) -> Result<(), DistributedError> {
        if !self.operations.contains_key(&dependency.predecessor()) {
            return Err(DistributedError::UnknownOperation {
                operation: dependency.predecessor(),
            });
        }

        if !self.operations.contains_key(&dependency.successor()) {
            return Err(DistributedError::UnknownOperation {
                operation: dependency.successor(),
            });
        }

        self.dependencies.insert(dependency);
        Ok(())
    }

    /// Adds a quantum transfer requirement.
    ///
    /// The qubit must already exist and both nodes must already exist.
    pub fn add_transfer(&mut self, transfer: QuantumTransfer) -> Result<(), DistributedError> {
        if self.transfers.contains_key(&transfer.qubit()) {
            return Err(DistributedError::DuplicateTransfer {
                qubit: transfer.qubit(),
            });
        }

        self.require_qubit(transfer.qubit())?;
        self.require_node(transfer.source())?;
        self.require_node(transfer.destination())?;

        self.transfers.insert(transfer.qubit(), transfer);
        Ok(())
    }

    /// Returns a node.
    #[must_use]
    pub fn node(&self, id: NodeId) -> Option<&QuantumNode> {
        self.nodes.get(&id)
    }

    /// Returns a quantum link.
    #[must_use]
    pub fn link(&self, id: LinkId) -> Option<&QuantumLink> {
        self.links.get(&id)
    }

    /// Returns a logical distributed qubit.
    #[must_use]
    pub fn qubit(&self, id: QubitId) -> Option<&DistributedQubit> {
        self.qubits.get(&id)
    }

    /// Returns an entanglement resource.
    #[must_use]
    pub fn entanglement(&self, id: EntanglementResourceId) -> Option<&EntanglementResource> {
        self.entanglement.get(&id)
    }

    /// Returns a distributed operation.
    #[must_use]
    pub fn operation(&self, id: DistributedOperationId) -> Option<&DistributedOperation> {
        self.operations.get(&id)
    }

    /// Returns a classical channel.
    #[must_use]
    pub fn classical_channel(&self, id: ClassicalChannelId) -> Option<&ClassicalChannel> {
        self.classical_channels.get(&id)
    }

    /// Returns a transfer associated with a qubit.
    #[must_use]
    pub fn transfer(&self, qubit: QubitId) -> Option<&QuantumTransfer> {
        self.transfers.get(&qubit)
    }

    /// Returns nodes in deterministic identifier order.
    #[must_use]
    pub fn nodes(&self) -> impl Iterator<Item = &QuantumNode> {
        self.nodes.values()
    }

    /// Returns links in deterministic identifier order.
    #[must_use]
    pub fn links(&self) -> impl Iterator<Item = &QuantumLink> {
        self.links.values()
    }

    /// Returns distributed qubits in deterministic identifier order.
    #[must_use]
    pub fn qubits(&self) -> impl Iterator<Item = &DistributedQubit> {
        self.qubits.values()
    }

    /// Returns entanglement resources in deterministic identifier order.
    #[must_use]
    pub fn entanglement_resources(
        &self,
    ) -> impl Iterator<Item = &EntanglementResource> {
        self.entanglement.values()
    }

    /// Returns operations in deterministic identifier order.
    #[must_use]
    pub fn operations(&self) -> impl Iterator<Item = &DistributedOperation> {
        self.operations.values()
    }

    /// Returns classical channels in deterministic identifier order.
    #[must_use]
    pub fn classical_channels(&self) -> impl Iterator<Item = &ClassicalChannel> {
        self.classical_channels.values()
    }

    /// Returns operation dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> impl Iterator<Item = &DistributedDependency> {
        self.dependencies.iter()
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of links.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of distributed operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns all links incident to a node.
    ///
    /// The result is deterministic.
    #[must_use]
    pub fn links_for_node(&self, node: NodeId) -> Vec<LinkId> {
        self.links
            .values()
            .filter(|link| {
                link.source().node() == node || link.destination().node() == node
            })
            .map(QuantumLink::id)
            .collect()
    }

    /// Returns all logical qubits explicitly located on a node.
    #[must_use]
    pub fn qubits_at_node(&self, node: NodeId) -> Vec<QubitId> {
        self.qubits
            .values()
            .filter_map(|qubit| match qubit.location() {
                QubitLocation::Local(location) if location == node => Some(qubit.qubit()),
                _ => None,
            })
            .collect()
    }

    /// Performs complete structural validation.
    ///
    /// This validates internal references and local invariants only.
    ///
    /// It does NOT validate whether a real target hardware system supports
    /// the represented topology or operation.
    pub fn validate(&self) -> Result<(), DistributedError> {
        for link in self.links.values() {
            self.require_node(link.source().node())?;
            self.require_node(link.destination().node())?;
        }

        for channel in self.classical_channels.values() {
            self.require_node(channel.source())?;
            self.require_node(channel.destination())?;
        }

        for qubit in self.qubits.values() {
            if let Some(node) = qubit.location().node() {
                self.require_node(node)?;
            }
        }

        for resource in self.entanglement.values() {
            let (first, second) = resource.endpoints();

            self.require_node(first)?;
            self.require_node(second)?;

            if first == second {
                return Err(DistributedError::SelfEntanglement { node: first });
            }
        }

        for operation in self.operations.values() {
            if let Some(node) = operation.source_node() {
                self.require_node(node)?;
            }

            if let Some(node) = operation.destination_node() {
                self.require_node(node)?;
            }

            for operand in operation.operands() {
                self.validate_operand(*operand)?;
            }

            for link in operation.quantum_links() {
                self.require_link(*link)?;
            }

            for resource in operation.entanglement() {
                self.require_entanglement(*resource)?;
            }

            for channel in operation.classical_channels() {
                self.require_classical_channel(*channel)?;
            }
        }

        for dependency in &self.dependencies {
            if !self.operations.contains_key(&dependency.predecessor()) {
                return Err(DistributedError::UnknownOperation {
                    operation: dependency.predecessor(),
                });
            }

            if !self.operations.contains_key(&dependency.successor()) {
                return Err(DistributedError::UnknownOperation {
                    operation: dependency.successor(),
                });
            }
        }

        for transfer in self.transfers.values() {
            self.require_qubit(transfer.qubit())?;
            self.require_node(transfer.source())?;
            self.require_node(transfer.destination())?;
        }

        Ok(())
    }

    /// Returns whether the program contains no semantic objects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.links.is_empty()
            && self.classical_channels.is_empty()
            && self.qubits.is_empty()
            && self.entanglement.is_empty()
            && self.operations.is_empty()
            && self.dependencies.is_empty()
            && self.transfers.is_empty()
    }

    fn require_node(&self, node: NodeId) -> Result<(), DistributedError> {
        if self.nodes.contains_key(&node) {
            Ok(())
        } else {
            Err(DistributedError::UnknownNode { node })
        }
    }

    fn require_link(&self, link: LinkId) -> Result<(), DistributedError> {
        if self.links.contains_key(&link) {
            Ok(())
        } else {
            Err(DistributedError::UnknownLink { link })
        }
    }

    fn require_qubit(&self, qubit: QubitId) -> Result<(), DistributedError> {
        if self.qubits.contains_key(&qubit) {
            Ok(())
        } else {
            Err(DistributedError::UnknownQubit { qubit })
        }
    }

    fn require_entanglement(
        &self,
        resource: EntanglementResourceId,
    ) -> Result<(), DistributedError> {
        if self.entanglement.contains_key(&resource) {
            Ok(())
        } else {
            Err(DistributedError::UnknownEntanglement { resource })
        }
    }

    fn require_classical_channel(
        &self,
        channel: ClassicalChannelId,
    ) -> Result<(), DistributedError> {
        if self.classical_channels.contains_key(&channel) {
            Ok(())
        } else {
            Err(DistributedError::UnknownClassicalChannel { channel })
        }
    }

    fn require_operation(
        &self,
        operation: DistributedOperationId,
    ) -> Result<(), DistributedError> {
        if self.operations.contains_key(&operation) {
            Ok(())
        } else {
            Err(DistributedError::UnknownOperation { operation })
        }
    }

    fn validate_operand(&self, operand: DistributedOperand) -> Result<(), DistributedError> {
        match operand {
            DistributedOperand::Qubit(qubit) => self.require_qubit(qubit),
            DistributedOperand::RemoteQubit(remote) => {
                self.require_qubit(remote.qubit())?;
                self.require_node(remote.home())?;
                self.require_node(remote.accessed_from())
            }
            DistributedOperand::PhysicalQubit(_) => Ok(()),
            DistributedOperand::Entanglement(resource) => {
                self.require_entanglement(resource)
            }
            DistributedOperand::Node(node) => self.require_node(node),
            DistributedOperand::Link(link) => self.require_link(link),
            DistributedOperand::ClassicalChannel(channel) => {
                self.require_classical_channel(channel)
            }
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the distributed quantum IR model.
///
/// These errors are deliberately local to this module. The canonical
/// `quantum::ir::errors` layer may wrap them at the global validation boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributedError {
    /// Arithmetic overflow occurred.
    ArithmeticOverflow,

    /// A node identifier was already declared.
    DuplicateNode {
        node: NodeId,
    },

    /// A link identifier was already declared.
    DuplicateLink {
        link: LinkId,
    },

    /// A qubit identifier was already declared.
    DuplicateQubit {
        qubit: QubitId,
    },

    /// An entanglement resource identifier was already declared.
    DuplicateEntanglement {
        resource: EntanglementResourceId,
    },

    /// A distributed operation identifier was already declared.
    DuplicateOperation {
        operation: DistributedOperationId,
    },

    /// A classical channel identifier was already declared.
    DuplicateClassicalChannel {
        channel: ClassicalChannelId,
    },

    /// A transfer for a logical qubit already exists.
    DuplicateTransfer {
        qubit: QubitId,
    },

    /// A referenced node does not exist.
    UnknownNode {
        node: NodeId,
    },

    /// A referenced link does not exist.
    UnknownLink {
        link: LinkId,
    },

    /// A referenced qubit does not exist.
    UnknownQubit {
        qubit: QubitId,
    },

    /// A referenced entanglement resource does not exist.
    UnknownEntanglement {
        resource: EntanglementResourceId,
    },

    /// A referenced operation does not exist.
    UnknownOperation {
        operation: DistributedOperationId,
    },

    /// A referenced classical channel does not exist.
    UnknownClassicalChannel {
        channel: ClassicalChannelId,
    },

    /// A link connects a node to itself.
    SelfLink {
        node: NodeId,
    },

    /// An entanglement resource connects a node to itself.
    SelfEntanglement {
        node: NodeId,
    },

    /// A classical channel connects a node to itself.
    SelfClassicalChannel {
        node: NodeId,
    },

    /// A remote reference points to the same node.
    RemoteReferenceIsLocal {
        qubit: QubitId,
        node: NodeId,
    },

    /// A transfer source and destination are identical.
    LocalTransfer {
        qubit: QubitId,
        node: NodeId,
    },

    /// An operation depends on itself.
    SelfDependency {
        operation: DistributedOperationId,
    },
}

impl fmt::Display for DistributedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ArithmeticOverflow => {
                formatter.write_str("distributed IR arithmetic overflow")
            }

            Self::DuplicateNode { node } => {
                write!(formatter, "duplicate distributed node {node}")
            }

            Self::DuplicateLink { link } => {
                write!(formatter, "duplicate distributed link {link}")
            }

            Self::DuplicateQubit { qubit } => {
                write!(formatter, "duplicate distributed qubit {qubit}")
            }

            Self::DuplicateEntanglement { resource } => {
                write!(formatter, "duplicate entanglement resource {resource}")
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate distributed operation {operation}")
            }

            Self::DuplicateClassicalChannel { channel } => {
                write!(formatter, "duplicate classical channel {channel}")
            }

            Self::DuplicateTransfer { qubit } => {
                write!(formatter, "duplicate transfer requirement for {qubit}")
            }

            Self::UnknownNode { node } => {
                write!(formatter, "unknown distributed node {node}")
            }

            Self::UnknownLink { link } => {
                write!(formatter, "unknown distributed link {link}")
            }

            Self::UnknownQubit { qubit } => {
                write!(formatter, "unknown logical qubit {qubit}")
            }

            Self::UnknownEntanglement { resource } => {
                write!(formatter, "unknown entanglement resource {resource}")
            }

            Self::UnknownOperation { operation } => {
                write!(formatter, "unknown distributed operation {operation}")
            }

            Self::UnknownClassicalChannel { channel } => {
                write!(formatter, "unknown classical channel {channel}")
            }

            Self::SelfLink { node } => {
                write!(formatter, "distributed quantum link cannot connect {node} to itself")
            }

            Self::SelfEntanglement { node } => {
                write!(
                    formatter,
                    "entanglement resource cannot have identical endpoints: {node}"
                )
            }

            Self::SelfClassicalChannel { node } => {
                write!(
                    formatter,
                    "classical channel cannot connect {node} to itself"
                )
            }

            Self::RemoteReferenceIsLocal { qubit, node } => {
                write!(
                    formatter,
                    "remote reference for {qubit} is local to {node}"
                )
            }

            Self::LocalTransfer { qubit, node } => {
                write!(
                    formatter,
                    "quantum transfer for {qubit} has identical source and destination {node}"
                )
            }

            Self::SelfDependency { operation } => {
                write!(
                    formatter,
                    "distributed operation {operation} cannot depend on itself"
                )
            }
        }
    }
}

impl std::error::Error for DistributedError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn two_node_program() -> DistributedProgram {
        let mut program = DistributedProgram::new();

        program
            .add_node(QuantumNode::new(NodeId::new(0)))
            .expect("node 0 should be insertable");

        program
            .add_node(QuantumNode::new(NodeId::new(1)))
            .expect("node 1 should be insertable");

        program
    }

    #[test]
    fn identifiers_are_typed_and_stable() {
        let logical = QubitId::new(42);
        let physical = PhysicalQubitId::new(42);

        assert_eq!(logical.index(), 42);
        assert_eq!(physical.index(), 42);

        assert_ne!(
            format!("{logical}"),
            format!("{physical}"),
            "logical and physical identities must remain distinguishable"
        );
    }

    #[test]
    fn nodes_are_sparse_and_not_size_limited() {
        let mut program = DistributedProgram::new();

        program
            .add_node(QuantumNode::new(NodeId::new(0)))
            .expect("node 0");

        program
            .add_node(QuantumNode::new(NodeId::new(u64::MAX)))
            .expect("large node identifier");

        assert_eq!(program.node_count(), 2);
        assert!(program.node(NodeId::new(u64::MAX)).is_some());
    }

    #[test]
    fn links_require_existing_nodes() {
        let mut program = DistributedProgram::new();

        let link = QuantumLink::new(
            LinkId::new(0),
            NodeId::new(0),
            NodeId::new(1),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Entanglement,
        )
        .expect("link itself is valid");

        assert_eq!(
            program.add_link(link),
            Err(DistributedError::UnknownNode {
                node: NodeId::new(0)
            })
        );

        program
            .add_node(QuantumNode::new(NodeId::new(0)))
            .expect("node 0");

        program
            .add_node(QuantumNode::new(NodeId::new(1)))
            .expect("node 1");

        program
            .add_link(link)
            .expect("link should now be valid");
    }

    #[test]
    fn logical_qubits_use_canonical_qubit_id() {
        let mut program = two_node_program();

        program
            .add_qubit(DistributedQubit::local(
                QubitId::new(7),
                NodeId::new(0),
            ))
            .expect("qubit should be insertable");

        assert_eq!(
            program.qubit(QubitId::new(7)),
            Some(&DistributedQubit::local(
                QubitId::new(7),
                NodeId::new(0)
            ))
        );
    }

    #[test]
    fn remote_reference_requires_different_nodes() {
        let qubit = QubitId::new(1);

        assert!(RemoteQubit::new(qubit, NodeId::new(0), NodeId::new(1)).is_ok());

        assert_eq!(
            RemoteQubit::new(qubit, NodeId::new(0), NodeId::new(0)),
            Err(DistributedError::RemoteReferenceIsLocal {
                qubit,
                node: NodeId::new(0)
            })
        );
    }

    #[test]
    fn entanglement_endpoints_are_canonicalized() {
        let resource = EntanglementResource::new(
            EntanglementResourceId::new(0),
            NodeId::new(9),
            NodeId::new(2),
        )
        .expect("resource should be valid");

        assert_eq!(
            resource.endpoints(),
            (NodeId::new(2), NodeId::new(9))
        );
    }

    #[test]
    fn distributed_program_validates_references() {
        let mut program = two_node_program();

        program
            .add_qubit(DistributedQubit::local(
                QubitId::new(0),
                NodeId::new(0),
            ))
            .expect("qubit");

        let link = QuantumLink::new(
            LinkId::new(0),
            NodeId::new(0),
            NodeId::new(1),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Hybrid,
        )
        .expect("link");

        program.add_link(link).expect("link insertion");

        let resource = EntanglementResource::new(
            EntanglementResourceId::new(0),
            NodeId::new(0),
            NodeId::new(1),
        )
        .expect("resource");

        program
            .add_entanglement(resource)
            .expect("entanglement insertion");

        let operation = DistributedOperation::new(
            DistributedOperationId::new(0),
            DistributedOperationKind::Teleport,
        )
        .with_operand(DistributedOperand::Qubit(QubitId::new(0)))
        .with_entanglement(EntanglementResourceId::new(0))
        .with_quantum_link(LinkId::new(0))
        .with_source_node(NodeId::new(0))
        .with_destination_node(NodeId::new(1));

        program
            .add_operation(operation)
            .expect("operation insertion");

        program.validate().expect("program should validate");
    }

    #[test]
    fn dependencies_cannot_reference_unknown_operations() {
        let mut program = two_node_program();

        let operation = DistributedOperation::new(
            DistributedOperationId::new(0),
            DistributedOperationKind::Synchronize,
        );

        program
            .add_operation(operation)
            .expect("operation");

        let dependency = DistributedDependency::new(
            DistributedOperationId::new(0),
            DistributedOperationId::new(1),
        )
        .expect("dependency");

        assert_eq!(
            program.add_dependency(dependency),
            Err(DistributedError::UnknownOperation {
                operation: DistributedOperationId::new(1)
            })
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        assert_eq!(
            DistributedDependency::new(
                DistributedOperationId::new(1),
                DistributedOperationId::new(1)
            ),
            Err(DistributedError::SelfDependency {
                operation: DistributedOperationId::new(1)
            })
        );
    }

    #[test]
    fn transfer_requires_distinct_domains() {
        assert!(QuantumTransfer::new(
            QubitId::new(0),
            NodeId::new(0),
            NodeId::new(1),
            TransferMode::Teleportation
        )
        .is_ok());

        assert_eq!(
            QuantumTransfer::new(
                QubitId::new(0),
                NodeId::new(0),
                NodeId::new(0),
                TransferMode::Teleportation
            ),
            Err(DistributedError::LocalTransfer {
                qubit: QubitId::new(0),
                node: NodeId::new(0)
            })
        );
    }

    #[test]
    fn deterministic_iteration_is_preserved() {
        let mut program = DistributedProgram::new();

        program
            .add_node(QuantumNode::new(NodeId::new(10)))
            .expect("node 10");

        program
            .add_node(QuantumNode::new(NodeId::new(2)))
            .expect("node 2");

        program
            .add_node(QuantumNode::new(NodeId::new(7)))
            .expect("node 7");

        let ids: Vec<NodeId> = program.nodes().map(QuantumNode::id).collect();

        assert_eq!(
            ids,
            vec![
                NodeId::new(2),
                NodeId::new(7),
                NodeId::new(10)
            ]
        );
    }

    #[test]
    fn unbounded_quantity_is_not_max_integer() {
        assert!(DistributedQuantity::unbounded().is_unbounded());
        assert_eq!(
            DistributedQuantity::unbounded().as_finite(),
            None
        );
    }

    #[test]
    fn checked_quantity_addition_rejects_overflow() {
        assert_eq!(
            DistributedQuantity::finite(u64::MAX)
                .checked_add(DistributedQuantity::finite(1)),
            Err(DistributedError::ArithmeticOverflow)
        );
    }

    #[test]
    fn empty_program_is_valid() {
        let program = DistributedProgram::new();

        assert!(program.is_empty());
        program.validate().expect("empty program is valid");
    }

    #[test]
    fn physical_qubit_operand_is_explicit() {
        let operand =
            DistributedOperand::PhysicalQubit(PhysicalQubitId::new(123));

        assert_eq!(
            operand,
            DistributedOperand::PhysicalQubit(PhysicalQubitId::new(123))
        );
    }
}