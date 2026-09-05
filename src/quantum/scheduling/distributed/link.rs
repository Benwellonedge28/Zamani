//! Zamani Quantum Scheduling — Distributed Link Model.
//!
//! `src/quantum/scheduling/distributed/link.rs`
//!
//! # Responsibility
//!
//! This module defines the scheduler-facing representation of a distributed
//! quantum communication link.
//!
//! It answers:
//!
//! > "What distributed communication resource exists between these semantic
//! > nodes, what can it support, and what scheduling constraints does it
//! > expose?"
//!
//! The link model is deliberately separate from:
//!
//! - quantum program semantics;
//! - hardware discovery;
//! - physical networking;
//! - routing/path finding;
//! - network transport;
//! - authentication;
//! - calibration;
//! - pulse generation;
//! - QPU execution;
//! - resource allocation;
//! - scheduling algorithms.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!              canonical NodeId / LinkId / qubits
//!                             │
//!                             ▼
//!                    quantum::routing
//!                             │
//!                     path / placement
//!                             │
//!                             ▼
//!                 quantum::scheduling
//!                             │
//!              ┌──────────────┴──────────────┐
//!              │ distributed::link           │
//!              │                             │
//!              │ endpoint identity           │
//!              │ direction                   │
//!              │ communication capabilities  │
//!              │ abstract capacity           │
//!              │ timing                      │
//!              │ scheduler resources         │
//!              │ availability                │
//!              └──────────────┬──────────────┘
//!                             │
//!                             ▼
//!                       scheduler planner
//!                             │
//!                             ▼
//!                    verification / result
//!                             │
//!                             ▼
//!                      hardware / runtime
//! ```
//!
//! # Canonical identity ownership
//!
//! This module MUST NOT define replacement distributed identities.
//!
//! Node identity comes from:
//!
//! ```text
//! crate::quantum::ir::model::distributed::NodeId
//! ```
//!
//! Link identity comes from:
//!
//! ```text
//! crate::quantum::ir::model::distributed::LinkId
//! ```
//!
//! Logical and physical qubit identities, whenever required by a consumer,
//! come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This avoids the duplicate-identity problem that otherwise occurs when
//! routing, scheduling and the distributed IR invent incompatible wrappers.
//!
//! # Relationship to the canonical distributed IR
//!
//! The canonical distributed IR already models semantic distributed links.
//!
//! This module does NOT replace that representation.
//!
//! Instead:
//!
//! ```text
//! canonical QuantumLink
//!        │
//!        │ semantic distributed requirement/topology
//!        ▼
//! scheduling::distributed::link
//!        │
//!        │ scheduler-facing timing/resource constraints
//!        ▼
//! scheduling planner
//! ```
//!
//! The scheduler-facing representation may therefore contain information that
//! is necessary for scheduling but does not belong in semantic IR, such as:
//!
//! - abstract scheduling capacity;
//! - readiness state;
//! - latency;
//! - duration;
//! - resource requirements;
//! - time windows;
//! - concurrency semantics;
//! - availability.
//!
//! # Write once, scale everywhere
//!
//! A link can represent:
//!
//! - two quantum processors;
//! - two chips;
//! - two modules;
//! - a quantum-network edge;
//! - a repeater connection;
//! - a simulator partition;
//! - a future communication mechanism.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_LINKS
//! MAX_NODES
//! MAX_CHANNELS
//! MAX_QUBITS
//! MAX_HOPS
//! ```
//!
//! The number of links, endpoints, resources, communication units and
//! concurrent transfers is determined by the target description and explicit
//! scheduling policy.
//!
//! "Infinity" means that this module introduces no artificial finite
//! architectural ceiling. Actual compilation remains bounded by the resources
//! of the target and compiler process.
//!
//! # Important scheduling distinction
//!
//! A link is a resource.
//!
//! Routing answers:
//!
//! > "Which link/path should be used?"
//!
//! Scheduling answers:
//!
//! > "When can the selected link resource be used?"
//!
//! This module provides the information required to answer the second question.
//!
//! # Directionality
//!
//! Direction is semantic and must not be inferred from endpoint ordering.
//!
//! A bidirectional link may be used in either direction.
//!
//! A directed link may only be used from its source to its destination.
//!
//! # Capacity
//!
//! Capacity is explicit.
//!
//! A capacity of zero is different from an unbounded capacity.
//!
//! `u64::MAX` is NOT treated as an implicit representation of infinity.
//!
//! `Unbounded` is explicit.
//!
//! # Timing
//!
//! Timing uses the scheduler's abstract `Duration` type.
//!
//! No physical unit such as nanoseconds or device ticks is embedded here.
//!
//! Hardware adapters determine how abstract scheduling durations map to the
//! target's physical timing system.
//!
//! # Determinism
//!
//! Ordered collections are used for resource requirements and metadata where
//! ordering can affect observable behavior.
//!
//! No scheduling decision should depend on `HashMap` iteration order.
//!
//! # Arithmetic safety
//!
//! Potentially overflowing arithmetic is checked.
//!
//! Wrapping arithmetic is never used for scheduling semantics.
//!
//! # Thread safety
//!
//! The data model contains no global mutable state and no interior mutability.
//!
//! It can therefore be shared or transferred between analysis stages subject
//! to the normal ownership rules of the containing scheduler.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! `#![forbid(unsafe_code)]` makes this a compiler-enforced requirement.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::model::distributed
//! quantum::ir::qubit
//! quantum::scheduling::types
//! ```
//!
//! Downstream:
//!
//! ```text
//! distributed::communication
//! distributed::network
//! distributed::node
//! planners
//! resources
//! timing
//! verification
//! adapters::routing
//! adapters::hardware
//! ```
//!
//! The downstream modules consume this contract; this file does not import
//! those implementation modules.
//!
//! Therefore adding or changing a scheduler algorithm, network planner,
//! communication implementation, hardware adapter or runtime must not require
//! changing this file merely to make the dependency graph compile.
//!
//! # No execution
//!
//! Constructing or validating a `QuantumLink` does not establish a real network
//! connection, allocate hardware, generate entanglement, transmit a message,
//! or execute a quantum operation.
//!
//! It is a scheduling description only.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::identity::ResourceId;
use crate::quantum::ir::model::distributed::{
    LinkDirection,
    LinkId,
    NodeId,
    QuantumLinkKind,
};
use crate::quantum::scheduling::types::Duration;

// =============================================================================
// Capacity
// =============================================================================

/// Capacity of a distributed link resource.
///
/// Capacity represents scheduler-visible concurrent usage.
///
/// It does not prescribe the physical implementation of the link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LinkCapacity {
    /// Explicit finite capacity.
    Finite(u64),

    /// The target does not declare a finite capacity.
    Unbounded,
}

impl LinkCapacity {
    /// Creates a finite capacity.
    #[must_use]
    pub const fn finite(value: u64) -> Self {
        Self::Finite(value)
    }

    /// Creates an explicitly unbounded capacity.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Returns whether the capacity is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns the finite value when one exists.
    #[must_use]
    pub const fn as_finite(self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Checks whether a requested quantity fits.
    #[must_use]
    pub const fn can_satisfy(self, requested: u64) -> bool {
        match self {
            Self::Finite(capacity) => requested <= capacity,
            Self::Unbounded => true,
        }
    }

    /// Adds two capacities without overflowing.
    ///
    /// If either operand is unbounded, the result is unbounded.
    pub const fn checked_add(self, other: Self) -> Result<Self, LinkError> {
        match (self, other) {
            (Self::Unbounded, _) | (_, Self::Unbounded) => Ok(Self::Unbounded),

            (Self::Finite(lhs), Self::Finite(rhs)) => {
                match lhs.checked_add(rhs) {
                    Some(value) => Ok(Self::Finite(value)),
                    None => Err(LinkError::ArithmeticOverflow),
                }
            }
        }
    }
}

impl Default for LinkCapacity {
    fn default() -> Self {
        Self::Finite(0)
    }
}

// =============================================================================
// Availability
// =============================================================================

/// Scheduler-visible availability state of a distributed link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LinkAvailability {
    /// The link is available for scheduling.
    Available,

    /// The link is temporarily unavailable.
    Unavailable,

    /// The link is available but degraded.
    Degraded,

    /// Availability has not been established.
    Unknown,
}

impl Default for LinkAvailability {
    fn default() -> Self {
        Self::Unknown
    }
}

impl LinkAvailability {
    /// Returns whether scheduling may consider this link usable.
    ///
    /// Unknown is deliberately rejected rather than interpreted as available.
    #[must_use]
    pub const fn is_schedulable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns whether the link is explicitly unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether availability is unresolved.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Scheduling mode
// =============================================================================

/// Concurrency semantics of a distributed link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LinkConcurrency {
    /// Only one transfer/use may occupy the link at a time.
    Exclusive,

    /// Multiple operations may use the link up to its declared capacity.
    CapacityLimited,

    /// The target declares no finite scheduler-visible concurrency ceiling.
    Unbounded,
}

impl Default for LinkConcurrency {
    fn default() -> Self {
        Self::CapacityLimited
    }
}

// =============================================================================
// Resource requirement
// =============================================================================

/// A scheduler-visible resource requirement associated with a link operation.
///
/// The resource identity is the canonical IR `ResourceId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LinkResourceRequirement {
    resource: ResourceId,
    quantity: u64,
}

impl LinkResourceRequirement {
    /// Creates a resource requirement.
    #[must_use]
    pub const fn new(resource: ResourceId, quantity: u64) -> Self {
        Self {
            resource,
            quantity,
        }
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource(self) -> ResourceId {
        self.resource
    }

    /// Returns the required quantity.
    #[must_use]
    pub const fn quantity(self) -> u64 {
        self.quantity
    }
}

// =============================================================================
// Link
// =============================================================================

/// Scheduler-facing distributed quantum communication link.
///
/// This type is deliberately a descriptor rather than an execution object.
///
/// It does not:
///
/// - open a socket;
/// - allocate a network interface;
/// - establish entanglement;
/// - transmit data;
/// - invoke hardware;
/// - perform routing;
/// - reserve resources in a global scheduler.
///
/// It describes the constraints a scheduler must consider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledLink {
    id: LinkId,
    source: NodeId,
    destination: NodeId,
    direction: LinkDirection,
    kind: QuantumLinkKind,

    /// Scheduler-visible maximum concurrent usage.
    capacity: LinkCapacity,

    /// Concurrency semantics.
    concurrency: LinkConcurrency,

    /// Abstract time required by the communication primitive.
    duration: Option<Duration>,

    /// Abstract propagation/coordination latency.
    ///
    /// This is separate from operation duration because a distributed
    /// operation can have execution and coordination components.
    latency: Option<Duration>,

    /// Scheduler-visible availability.
    availability: LinkAvailability,

    /// Additional resources that must be reserved when this link is used.
    resources: BTreeMap<ResourceId, u64>,

    /// Optional descriptive label.
    label: Option<String>,
}

impl ScheduledLink {
    /// Creates a scheduler-facing link descriptor.
    ///
    /// A link cannot connect a node to itself because local operations should
    /// not be represented as distributed communication.
    pub fn new(
        id: LinkId,
        source: NodeId,
        destination: NodeId,
        direction: LinkDirection,
        kind: QuantumLinkKind,
    ) -> Result<Self, LinkError> {
        if source == destination {
            return Err(LinkError::SelfLink { node: source });
        }

        Ok(Self {
            id,
            source,
            destination,
            direction,
            kind,
            capacity: LinkCapacity::Finite(0),
            concurrency: LinkConcurrency::CapacityLimited,
            duration: None,
            latency: None,
            availability: LinkAvailability::Unknown,
            resources: BTreeMap::new(),
            label: None,
        })
    }

    /// Creates a link from the canonical semantic distributed link.
    ///
    /// Semantic link identity and endpoints are preserved exactly.
    ///
    /// Scheduler-specific information is intentionally left unresolved until
    /// supplied by the target/hardware adapter.
    pub fn from_ir(
        link: &crate::quantum::ir::model::distributed::QuantumLink,
    ) -> Result<Self, LinkError> {
        Self::new(
            link.id(),
            link.source().node(),
            link.destination().node(),
            link.direction(),
            link.kind(),
        )?
        .with_capacity(match link.capacity().as_finite() {
            Some(value) => LinkCapacity::Finite(value),
            None => LinkCapacity::Unbounded,
        })
    }

    /// Sets the scheduler-visible capacity.
    #[must_use]
    pub const fn with_capacity(mut self, capacity: LinkCapacity) -> Self {
        self.capacity = capacity;
        self
    }

    /// Sets concurrency semantics.
    #[must_use]
    pub const fn with_concurrency(
        mut self,
        concurrency: LinkConcurrency,
    ) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Sets the abstract communication duration.
    #[must_use]
    pub const fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the abstract communication latency.
    #[must_use]
    pub const fn with_latency(mut self, latency: Duration) -> Self {
        self.latency = Some(latency);
        self
    }

    /// Sets scheduler-visible availability.
    #[must_use]
    pub const fn with_availability(
        mut self,
        availability: LinkAvailability,
    ) -> Self {
        self.availability = availability;
        self
    }

    /// Sets a descriptive label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Adds a scheduler-visible resource requirement.
    ///
    /// Reusing the same resource identity replaces the previous quantity.
    #[must_use]
    pub fn with_resource(
        mut self,
        requirement: LinkResourceRequirement,
    ) -> Self {
        self.resources
            .insert(requirement.resource(), requirement.quantity());

        self
    }

    /// Returns the canonical distributed link identity.
    #[must_use]
    pub const fn id(&self) -> LinkId {
        self.id
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

    /// Returns the semantic direction.
    #[must_use]
    pub const fn direction(&self) -> LinkDirection {
        self.direction
    }

    /// Returns the semantic link kind.
    #[must_use]
    pub const fn kind(&self) -> QuantumLinkKind {
        self.kind
    }

    /// Returns scheduler-visible capacity.
    #[must_use]
    pub const fn capacity(&self) -> LinkCapacity {
        self.capacity
    }

    /// Returns concurrency semantics.
    #[must_use]
    pub const fn concurrency(&self) -> LinkConcurrency {
        self.concurrency
    }

    /// Returns the optional abstract communication duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Returns the optional abstract latency.
    #[must_use]
    pub const fn latency(&self) -> Option<Duration> {
        self.latency
    }

    /// Returns scheduler-visible availability.
    #[must_use]
    pub const fn availability(&self) -> LinkAvailability {
        self.availability
    }

    /// Returns the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns all additional resource requirements.
    ///
    /// The map is ordered by canonical resource identity.
    #[must_use]
    pub fn resources(&self) -> &BTreeMap<ResourceId, u64> {
        &self.resources
    }

    /// Returns the quantity required from a resource.
    #[must_use]
    pub fn resource_requirement(
        &self,
        resource: ResourceId,
    ) -> Option<u64> {
        self.resources.get(&resource).copied()
    }

    /// Returns whether this link can carry traffic from the supplied node.
    #[must_use]
    pub const fn can_transfer_from(&self, node: NodeId) -> bool {
        match self.direction {
            LinkDirection::Bidirectional => {
                self.source == node || self.destination == node
            }

            LinkDirection::Directed => self.source == node,
        }
    }

    /// Returns whether the supplied ordered pair is a valid transfer.
    #[must_use]
    pub const fn supports_transfer(
        &self,
        source: NodeId,
        destination: NodeId,
    ) -> bool {
        match self.direction {
            LinkDirection::Bidirectional => {
                (self.source == source && self.destination == destination)
                    || (self.source == destination && self.destination == source)
            }

            LinkDirection::Directed => {
                self.source == source && self.destination == destination
            }
        }
    }

    /// Returns whether the link connects the supplied nodes.
    ///
    /// For a bidirectional link, endpoint order does not matter.
    #[must_use]
    pub const fn connects(
        &self,
        first: NodeId,
        second: NodeId,
    ) -> bool {
        self.supports_transfer(first, second)
            || matches!(self.direction, LinkDirection::Directed)
                && self.source == second
                && self.destination == first
    }

    /// Returns the opposite endpoint when the supplied node belongs to the
    /// link.
    ///
    /// For directed links this describes graph adjacency only; it does not
    /// imply that data may legally be transferred in the reverse direction.
    #[must_use]
    pub const fn other_endpoint(
        &self,
        node: NodeId,
    ) -> Option<NodeId> {
        if self.source == node {
            Some(self.destination)
        } else if self.destination == node {
            Some(self.source)
        } else {
            None
        }
    }

    /// Returns whether the link has enough declared capacity for a request.
    #[must_use]
    pub const fn can_satisfy_capacity(
        &self,
        requested: u64,
    ) -> bool {
        self.capacity.can_satisfy(requested)
    }

    /// Returns whether the link is currently eligible for scheduling.
    ///
    /// This does not mean that a particular operation can execute. Operation
    /// compatibility, timing, resource calendars and routing constraints are
    /// checked elsewhere.
    #[must_use]
    pub const fn is_schedulable(&self) -> bool {
        self.availability.is_schedulable()
    }

    /// Validates internal invariants.
    ///
    /// This validates only this descriptor. It does not validate that either
    /// node exists in a network, that the physical link exists, or that a
    /// target actually supports the advertised capabilities.
    pub fn validate(&self) -> Result<(), LinkError> {
        if self.source == self.destination {
            return Err(LinkError::SelfLink {
                node: self.source,
            });
        }

        if let Some(duration) = self.duration {
            if duration.value() == 0 {
                // Zero duration is semantically valid, so it is intentionally
                // not rejected.
                let _ = duration;
            }
        }

        if let Some(latency) = self.latency {
            if latency.value() == 0 {
                // Zero latency is also semantically representable.
                let _ = latency;
            }
        }

        for (resource, quantity) in &self.resources {
            if *quantity == 0 {
                return Err(LinkError::ZeroResourceRequirement {
                    resource: *resource,
                });
            }
        }

        if self.concurrency == LinkConcurrency::Unbounded
            && !self.capacity.is_unbounded()
        {
            return Err(LinkError::InconsistentConcurrencyCapacity);
        }

        Ok(())
    }

    /// Returns a checked total abstract communication time.
    ///
    /// If both duration and latency are supplied, they are added with checked
    /// arithmetic.
    ///
    /// If neither is supplied, `None` is returned because the scheduler does
    /// not have enough information to infer a timing value.
    #[must_use]
    pub fn total_time(&self) -> Result<Option<Duration>, LinkError> {
        match (self.duration, self.latency) {
            (None, None) => Ok(None),
            (Some(duration), None) => Ok(Some(duration)),
            (None, Some(latency)) => Ok(Some(latency)),
            (Some(duration), Some(latency)) => duration
                .checked_add(latency)
                .map(Some)
                .ok_or(LinkError::ArithmeticOverflow),
        }
    }
}

// =============================================================================
// Link set
// =============================================================================

/// Deterministic collection of scheduler-facing distributed links.
///
/// No maximum number of links is encoded.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LinkSet {
    links: BTreeMap<LinkId, ScheduledLink>,
}

impl LinkSet {
    /// Creates an empty link set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            links: BTreeMap::new(),
        }
    }

    /// Inserts a link.
    ///
    /// Returns the previous link with the same canonical identity, if any.
    pub fn insert(
        &mut self,
        link: ScheduledLink,
    ) -> Option<ScheduledLink> {
        self.links.insert(link.id(), link)
    }

    /// Removes a link.
    pub fn remove(
        &mut self,
        id: LinkId,
    ) -> Option<ScheduledLink> {
        self.links.remove(&id)
    }

    /// Returns a link by canonical identity.
    #[must_use]
    pub fn get(
        &self,
        id: LinkId,
    ) -> Option<&ScheduledLink> {
        self.links.get(&id)
    }

    /// Returns a mutable link.
    pub fn get_mut(
        &mut self,
        id: LinkId,
    ) -> Option<&mut ScheduledLink> {
        self.links.get_mut(&id)
    }

    /// Returns whether a link exists.
    #[must_use]
    pub fn contains(
        &self,
        id: LinkId,
    ) -> bool {
        self.links.contains_key(&id)
    }

    /// Returns the number of links.
    #[must_use]
    pub fn len(&self) -> usize {
        self.links.len()
    }

    /// Returns whether no links are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.links.is_empty()
    }

    /// Iterates over links in deterministic canonical-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&LinkId, &ScheduledLink)> {
        self.links.iter()
    }

    /// Iterates over link values in deterministic order.
    pub fn values(
        &self,
    ) -> impl Iterator<Item = &ScheduledLink> {
        self.links.values()
    }

    /// Returns the underlying deterministic map.
    #[must_use]
    pub fn as_map(
        &self,
    ) -> &BTreeMap<LinkId, ScheduledLink> {
        &self.links
    }

    /// Returns links connected to a node.
    ///
    /// The returned iterator is lazy.
    pub fn incident_to(
        &self,
        node: NodeId,
    ) -> impl Iterator<Item = &ScheduledLink> {
        self.links.values().filter(move |link| {
            link.source() == node || link.destination() == node
        })
    }

    /// Returns links that support the requested transfer.
    ///
    /// Directed-link semantics are respected.
    pub fn supporting_transfer(
        &self,
        source: NodeId,
        destination: NodeId,
    ) -> impl Iterator<Item = &ScheduledLink> {
        self.links
            .values()
            .filter(move |link| link.supports_transfer(source, destination))
    }

    /// Returns currently schedulable links.
    pub fn schedulable(
        &self,
    ) -> impl Iterator<Item = &ScheduledLink> {
        self.links.values().filter(|link| link.is_schedulable())
    }

    /// Validates every link.
    pub fn validate(&self) -> Result<(), LinkError> {
        for link in self.links.values() {
            link.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Structural errors produced by the scheduler-facing link model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError {
    /// A link connects a node to itself.
    SelfLink {
        /// Invalid node.
        node: NodeId,
    },

    /// A resource requirement has zero quantity.
    ZeroResourceRequirement {
        /// Resource with the invalid requirement.
        resource: ResourceId,
    },

    /// Concurrency and capacity declarations disagree.
    InconsistentConcurrencyCapacity,

    /// Checked arithmetic overflow occurred.
    ArithmeticOverflow,
}

impl fmt::Display for LinkError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::SelfLink { node } => {
                write!(
                    formatter,
                    "distributed scheduling link cannot connect node {node} to itself"
                )
            }

            Self::ZeroResourceRequirement { resource } => {
                write!(
                    formatter,
                    "distributed link resource {resource} has a zero requirement"
                )
            }

            Self::InconsistentConcurrencyCapacity => {
                write!(
                    formatter,
                    "unbounded link concurrency requires explicitly unbounded capacity"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "distributed link timing or capacity arithmetic overflowed"
                )
            }
        }
    }
}

impl std::error::Error for LinkError {}

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

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    #[test]
    fn self_links_are_rejected() {
        let result = ScheduledLink::new(
            link(1),
            node(7),
            node(7),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        );

        assert!(matches!(
            result,
            Err(LinkError::SelfLink { node }) if node == node(7)
        ));
    }

    #[test]
    fn canonical_identity_is_preserved() {
        let scheduled = ScheduledLink::new(
            link(42),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Hybrid,
        )
        .expect("valid link");

        assert_eq!(scheduled.id(), link(42));
        assert_eq!(scheduled.source(), node(1));
        assert_eq!(scheduled.destination(), node(2));
    }

    #[test]
    fn bidirectional_links_support_both_directions() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link");

        assert!(scheduled.supports_transfer(node(1), node(2)));
        assert!(scheduled.supports_transfer(node(2), node(1)));
    }

    #[test]
    fn directed_links_support_only_declared_direction() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Directed,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link");

        assert!(scheduled.supports_transfer(node(1), node(2)));
        assert!(!scheduled.supports_transfer(node(2), node(1)));
    }

    #[test]
    fn other_endpoint_is_graph_adjacency_not_permission() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Directed,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link");

        assert_eq!(
            scheduled.other_endpoint(node(1)),
            Some(node(2))
        );

        assert_eq!(
            scheduled.other_endpoint(node(2)),
            Some(node(1))
        );

        assert!(!scheduled.supports_transfer(node(2), node(1)));
    }

    #[test]
    fn unknown_links_are_not_schedulable() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link");

        assert!(!scheduled.is_schedulable());
    }

    #[test]
    fn available_links_are_schedulable() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_availability(LinkAvailability::Available);

        assert!(scheduled.is_schedulable());
    }

    #[test]
    fn degraded_links_are_schedulable() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_availability(LinkAvailability::Degraded);

        assert!(scheduled.is_schedulable());
    }

    #[test]
    fn unavailable_links_are_not_schedulable() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_availability(LinkAvailability::Unavailable);

        assert!(!scheduled.is_schedulable());
    }

    #[test]
    fn finite_capacity_is_checked() {
        let capacity = LinkCapacity::finite(8);

        assert!(capacity.can_satisfy(0));
        assert!(capacity.can_satisfy(8));
        assert!(!capacity.can_satisfy(9));
    }

    #[test]
    fn unbounded_capacity_is_explicit() {
        let capacity = LinkCapacity::unbounded();

        assert!(capacity.is_unbounded());
        assert!(capacity.can_satisfy(u64::MAX));
        assert_ne!(
            capacity,
            LinkCapacity::finite(u64::MAX)
        );
    }

    #[test]
    fn capacity_addition_is_checked() {
        let result = LinkCapacity::finite(u64::MAX)
            .checked_add(LinkCapacity::finite(1));

        assert!(matches!(
            result,
            Err(LinkError::ArithmeticOverflow)
        ));
    }

    #[test]
    fn unbounded_capacity_does_not_overflow() {
        let result = LinkCapacity::unbounded()
            .checked_add(LinkCapacity::finite(u64::MAX))
            .expect("unbounded addition is valid");

        assert_eq!(result, LinkCapacity::Unbounded);
    }

    #[test]
    fn resources_are_deterministic() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Hybrid,
        )
        .expect("valid link")
        .with_resource(
            LinkResourceRequirement::new(resource(20), 1),
        )
        .with_resource(
            LinkResourceRequirement::new(resource(3), 2),
        );

        let ids: Vec<ResourceId> =
            scheduled.resources().keys().copied().collect();

        assert_eq!(
            ids,
            vec![resource(3), resource(20)]
        );
    }

    #[test]
    fn zero_resource_requirements_are_rejected() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_resource(
            LinkResourceRequirement::new(resource(5), 0),
        );

        assert!(matches!(
            scheduled.validate(),
            Err(LinkError::ZeroResourceRequirement {
                resource
            }) if resource == resource(5)
        ));
    }

    #[test]
    fn duration_and_latency_are_added_with_checked_arithmetic() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_duration(Duration::new(100))
        .with_latency(Duration::new(25));

        assert_eq!(
            scheduled.total_time().expect("valid timing"),
            Some(Duration::new(125))
        );
    }

    #[test]
    fn missing_timing_information_is_not_invented() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link");

        assert_eq!(
            scheduled.total_time().expect("valid timing"),
            None
        );
    }

    #[test]
    fn zero_duration_is_semantically_allowed() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_duration(Duration::ZERO);

        assert!(scheduled.validate().is_ok());
    }

    #[test]
    fn link_set_is_deterministic() {
        let mut links = LinkSet::new();

        links.insert(
            ScheduledLink::new(
                link(20),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        links.insert(
            ScheduledLink::new(
                link(2),
                node(2),
                node(3),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        links.insert(
            ScheduledLink::new(
                link(11),
                node(3),
                node(4),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        let ids: Vec<LinkId> =
            links.iter().map(|(id, _)| *id).collect();

        assert_eq!(
            ids,
            vec![link(2), link(11), link(20)]
        );
    }

    #[test]
    fn incident_lookup_is_lazy() {
        let mut links = LinkSet::new();

        links.insert(
            ScheduledLink::new(
                link(1),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        links.insert(
            ScheduledLink::new(
                link(2),
                node(3),
                node(4),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        assert_eq!(
            links.incident_to(node(1)).count(),
            1
        );
    }

    #[test]
    fn supporting_transfer_respects_direction() {
        let mut links = LinkSet::new();

        links.insert(
            ScheduledLink::new(
                link(1),
                node(1),
                node(2),
                LinkDirection::Directed,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link"),
        );

        assert_eq!(
            links
                .supporting_transfer(node(1), node(2))
                .count(),
            1
        );

        assert_eq!(
            links
                .supporting_transfer(node(2), node(1))
                .count(),
            0
        );
    }

    #[test]
    fn schedulable_filter_is_deterministic() {
        let mut links = LinkSet::new();

        links.insert(
            ScheduledLink::new(
                link(1),
                node(1),
                node(2),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link")
            .with_availability(LinkAvailability::Available),
        );

        links.insert(
            ScheduledLink::new(
                link(2),
                node(2),
                node(3),
                LinkDirection::Bidirectional,
                QuantumLinkKind::Quantum,
            )
            .expect("valid link")
            .with_availability(LinkAvailability::Unavailable),
        );

        let ids: Vec<LinkId> =
            links.schedulable().map(ScheduledLink::id).collect();

        assert_eq!(ids, vec![link(1)]);
    }

    #[test]
    fn unbounded_concurrency_requires_unbounded_capacity() {
        let scheduled = ScheduledLink::new(
            link(1),
            node(1),
            node(2),
            LinkDirection::Bidirectional,
            QuantumLinkKind::Quantum,
        )
        .expect("valid link")
        .with_capacity(LinkCapacity::finite(4))
        .with_concurrency(LinkConcurrency::Unbounded);

        assert!(matches!(
            scheduled.validate(),
            Err(LinkError::InconsistentConcurrencyCapacity)
        ));
    }
}