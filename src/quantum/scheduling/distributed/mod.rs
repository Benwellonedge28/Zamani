//! Zamani Quantum Scheduling — Distributed Scheduling
//!
//! Path:
//!     src/quantum/scheduling/distributed/mod.rs
//!
//! # Purpose
//!
//! Production-grade scheduling contracts and deterministic scheduling support
//! for distributed quantum computation.
//!
//! This module answers:
//!
//!     "WHEN can distributed quantum work execute?"
//!
//! It does NOT redefine what distributed quantum computation means.
//!
//! Canonical distributed semantics remain owned by:
//!
//!     crate::quantum::ir::model::distributed
//!
//! Canonical logical/physical qubit identities remain owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! Canonical scheduling identities and temporal values remain owned by:
//!
//!     crate::quantum::scheduling::types
//!
//! This module adds scheduling-specific concepts:
//!
//! - distributed scheduling requests;
//! - node/resource availability;
//! - communication-resource reservations;
//! - distributed operation intervals;
//! - dependency-aware distributed planning;
//! - link/path reservations;
//! - classical communication latency;
//! - entanglement-resource consumption;
//! - deterministic schedule construction;
//! - schedule verification;
//! - incremental scheduling epochs.
//!
//! # Architectural boundary
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling::adapters
//!       |
//!       v
//! scheduling::distributed
//!       |
//!       +-----------------------------+
//!       |                             |
//!       v                             v
//! node/resource timing          communication timing
//!       |                             |
//!       +-------------+---------------+
//!                     |
//!                     v
//!              distributed schedule
//!                     |
//!                     v
//!                 verifier
//!                     |
//!                     v
//!                  runtime
//! ```
//!
//! # Canonical identity rule
//!
//! This module MUST NOT create another `QubitId` or `PhysicalQubitId`.
//!
//! Always use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Operation/resource identities likewise come from the canonical IR:
//!
//!     crate::quantum::ir::core::identity::OperationId
//!     crate::quantum::ir::core::identity::ResourceId
//!
//! # Scalability
//!
//! There are no architectural constants for:
//!
//! - number of nodes;
//! - number of qubits;
//! - number of links;
//! - number of operations;
//! - network diameter;
//! - communication rounds;
//! - QEC distance;
//! - schedule depth;
//! - resource capacity.
//!
//! A practical compilation is bounded only by actual host, compiler, target,
//! policy and execution resources.
//!
//! "Infinity" therefore means:
//!
//!     no artificial finite machine-size ceiling is encoded here.
//!
//! # Distributed scheduling principle
//!
//! Routing answers:
//!
//!     WHERE?
//!
//! Distributed scheduling answers:
//!
//!     WHEN?
//!
//! Routing may supply a path such as:
//!
//!     node A -> node B -> node C
//!
//! Scheduling determines when each communication/resource reservation can
//! occur while respecting:
//!
//! - dependencies;
//! - node availability;
//! - link availability;
//! - link capacity;
//! - entanglement availability;
//! - classical communication latency;
//! - operation duration;
//! - release times;
//! - deadlines;
//! - resource conflicts.
//!
//! # No execution
//!
//! This module never:
//!
//! - opens sockets;
//! - sends network messages;
//! - contacts a QPU;
//! - performs authentication;
//! - generates entanglement;
//! - invokes a decoder;
//! - performs physical routing;
//! - controls hardware.
//!
//! It produces a schedule that downstream runtime/hardware components may
//! execute.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly;
//! - no unsafe.
//!
//! The no-unsafe policy is compiler-enforced.
//!
//! # Determinism
//!
//! BTreeMap/BTreeSet are used wherever deterministic ordering matters.
//!
//! Given identical:
//!
//! - input;
//! - target snapshot;
//! - policy;
//! - dependency graph;
//! - seed/context;
//!
//! scheduling produces identical results.
//!
//! No hidden global state is used.
//!
//! # Future extraction
//!
//! This file is intentionally contract-complete.
//!
//! Its types may later be mechanically separated into:
//!
//!     node.rs
//!     link.rs
//!     communication.rs
//!     network.rs
//!
//! without changing semantic ownership.
//!
//! The composition root can then become:
//!
//!     pub mod node;
//!     pub mod link;
//!     pub mod communication;
//!     pub mod network;
//!
//! The current implementation does not require those files to exist.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::model::distributed::{LinkId, NodeId};
use crate::quantum::scheduling::types::{
    DependencyId,
    Duration,
    EpochId,
    ReservationId,
    ScheduleId,
    TimePoint,
};

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by distributed scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributedSchedulingError {
    /// Referenced node does not exist in the scheduling target.
    UnknownNode(NodeId),

    /// Referenced communication link does not exist.
    UnknownLink(LinkId),

    /// A logical qubit has no distributed location.
    QubitLocationMissing(QubitId),

    /// A physical qubit has no distributed location.
    PhysicalQubitLocationMissing(PhysicalQubitId),

    /// An operation references a dependency that is absent.
    UnknownDependency(DependencyId),

    /// An operation references another operation that is absent.
    UnknownOperation(OperationId),

    /// The dependency graph contains a cycle.
    DependencyCycle,

    /// A requested operation cannot be represented by the supplied target.
    UnsupportedOperation,

    /// A resource reservation could not be represented because arithmetic
    /// overflow would occur.
    ArithmeticOverflow,

    /// A deadline cannot be satisfied.
    DeadlineExceeded(OperationId),

    /// A resource is unavailable during the requested interval.
    ResourceUnavailable(ResourceId),

    /// A communication link is unavailable during the requested interval.
    LinkUnavailable(LinkId),

    /// Link capacity is insufficient.
    LinkCapacityExceeded(LinkId),

    /// A schedule invariant is violated.
    VerificationFailed(String),

    /// The request contains semantically inconsistent information.
    InvalidRequest(String),

    /// A schedule cannot be produced under the supplied constraints.
    Unschedulable(String),
}

impl fmt::Display for DistributedSchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode(node) => write!(formatter, "unknown distributed node: {}", node),
            Self::UnknownLink(link) => write!(formatter, "unknown distributed link: {}", link),
            Self::QubitLocationMissing(qubit) => {
                write!(formatter, "missing distributed location for logical qubit {:?}", qubit)
            }
            Self::PhysicalQubitLocationMissing(qubit) => {
                write!(
                    formatter,
                    "missing distributed location for physical qubit {:?}",
                    qubit
                )
            }
            Self::UnknownDependency(id) => {
                write!(formatter, "unknown scheduling dependency: {}", id)
            }
            Self::UnknownOperation(id) => {
                write!(formatter, "unknown operation: {:?}", id)
            }
            Self::DependencyCycle => write!(formatter, "distributed dependency graph contains a cycle"),
            Self::UnsupportedOperation => write!(formatter, "unsupported distributed operation"),
            Self::ArithmeticOverflow => write!(formatter, "distributed scheduling arithmetic overflow"),
            Self::DeadlineExceeded(id) => {
                write!(formatter, "deadline exceeded for operation {:?}", id)
            }
            Self::ResourceUnavailable(id) => {
                write!(formatter, "resource unavailable: {:?}", id)
            }
            Self::LinkUnavailable(id) => {
                write!(formatter, "communication link unavailable: {}", id)
            }
            Self::LinkCapacityExceeded(id) => {
                write!(formatter, "communication link capacity exceeded: {}", id)
            }
            Self::VerificationFailed(reason) => {
                write!(formatter, "distributed schedule verification failed: {}", reason)
            }
            Self::InvalidRequest(reason) => {
                write!(formatter, "invalid distributed scheduling request: {}", reason)
            }
            Self::Unschedulable(reason) => {
                write!(formatter, "distributed schedule is unschedulable: {}", reason)
            }
        }
    }
}

impl std::error::Error for DistributedSchedulingError {}

/// Result type for distributed scheduling.
pub type DistributedSchedulingResult<T> =
    Result<T, DistributedSchedulingError>;

// ============================================================================
// Capacity
// ============================================================================

/// Resource capacity.
///
/// `Unbounded` is explicit and is never represented by a sentinel such as
/// `u64::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capacity {
    /// Finite resource capacity.
    Finite(u64),

    /// No finite capacity is declared.
    Unbounded,
}

impl Capacity {
    /// Creates finite capacity.
    #[must_use]
    pub const fn finite(value: u64) -> Self {
        Self::Finite(value)
    }

    /// Creates unbounded capacity.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Returns true when the capacity is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Checks whether a requested quantity can fit.
    #[must_use]
    pub const fn can_fit(self, requested: u64) -> bool {
        match self {
            Self::Finite(capacity) => requested <= capacity,
            Self::Unbounded => true,
        }
    }
}

impl Default for Capacity {
    fn default() -> Self {
        Self::Finite(0)
    }
}

// ============================================================================
// Resource availability
// ============================================================================

/// An immutable availability interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AvailabilityWindow {
    /// Earliest time at which the resource is available.
    pub start: TimePoint,

    /// Duration for which it remains available.
    pub duration: Duration,
}

impl AvailabilityWindow {
    /// Creates a window.
    pub const fn new(start: TimePoint, duration: Duration) -> Self {
        Self { start, duration }
    }

    /// Returns the exclusive end.
    pub fn end(self) -> DistributedSchedulingResult<TimePoint> {
        self.start
            .checked_add(self.duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)
    }

    /// Checks whether the complete interval is inside this window.
    pub fn contains(
        self,
        start: TimePoint,
        duration: Duration,
    ) -> DistributedSchedulingResult<bool> {
        let end = start
            .checked_add(duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

        Ok(start >= self.start && end <= self.end()?)
    }
}

// ============================================================================
// Node scheduling state
// ============================================================================

/// Distributed node scheduling description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeScheduleResource {
    /// Canonical distributed node identity.
    pub node: NodeId,

    /// Node-local scheduling capacity.
    ///
    /// A value of one means exclusive occupancy.
    /// Larger values permit explicit concurrent work.
    pub capacity: Capacity,

    /// Initial availability.
    pub availability: Vec<AvailabilityWindow>,

    /// Optional associated physical qubits.
    ///
    /// The qubit identity itself remains canonical.
    pub physical_qubits: BTreeSet<PhysicalQubitId>,
}

impl NodeScheduleResource {
    /// Creates a node resource.
    #[must_use]
    pub fn new(node: NodeId, capacity: Capacity) -> Self {
        Self {
            node,
            capacity,
            availability: Vec::new(),
            physical_qubits: BTreeSet::new(),
        }
    }

    /// Adds an availability window.
    pub fn add_availability(&mut self, window: AvailabilityWindow) {
        self.availability.push(window);
        self.availability.sort_by_key(|entry| entry.start);
    }

    /// Adds a physical qubit associated with the node.
    pub fn add_physical_qubit(&mut self, qubit: PhysicalQubitId) {
        self.physical_qubits.insert(qubit);
    }
}

// ============================================================================
// Communication resources
// ============================================================================

/// Distributed communication-resource kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationKind {
    /// Quantum communication or entanglement generation.
    Quantum,

    /// Classical communication.
    Classical,

    /// Resource carrying both quantum and classical scheduling obligations.
    Hybrid,
}

/// A schedulable communication link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationResource {
    /// Canonical link identity.
    pub link: LinkId,

    /// Link source.
    pub source: NodeId,

    /// Link destination.
    pub destination: NodeId,

    /// Communication type.
    pub kind: CommunicationKind,

    /// Number of simultaneously reservable units.
    pub capacity: Capacity,

    /// Duration required by one communication reservation.
    pub duration: Duration,

    /// Optional propagation/communication latency.
    pub latency: Duration,

    /// Availability windows.
    pub availability: Vec<AvailabilityWindow>,

    /// Optional scheduler resource identity.
    pub resource: Option<ResourceId>,
}

impl CommunicationResource {
    /// Creates a communication resource.
    #[must_use]
    pub const fn new(
        link: LinkId,
        source: NodeId,
        destination: NodeId,
        kind: CommunicationKind,
        capacity: Capacity,
        duration: Duration,
        latency: Duration,
    ) -> Self {
        Self {
            link,
            source,
            destination,
            kind,
            capacity,
            duration,
            latency,
            availability: Vec::new(),
            resource: None,
        }
    }

    /// Adds an availability window.
    pub fn add_availability(&mut self, window: AvailabilityWindow) {
        self.availability.push(window);
        self.availability.sort_by_key(|entry| entry.start);
    }

    /// Returns whether the resource can carry a reservation.
    pub fn supports(
        &self,
        requested_start: TimePoint,
        requested_duration: Duration,
    ) -> DistributedSchedulingResult<bool> {
        if self.availability.is_empty() {
            return Ok(true);
        }

        self.availability.iter().try_fold(false, |found, window| {
            if found {
                return Ok(true);
            }

            window.contains(requested_start, requested_duration)
        })
    }
}

// ============================================================================
// Distributed qubit locations
// ============================================================================

/// Location of a logical qubit in the distributed execution topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitLocation {
    /// Canonical logical qubit identity.
    pub logical: QubitId,

    /// Distributed execution node.
    pub node: NodeId,

    /// Optional mapped physical qubit.
    pub physical: Option<PhysicalQubitId>,
}

impl QubitLocation {
    /// Creates a logical-qubit location.
    #[must_use]
    pub const fn logical(logical: QubitId, node: NodeId) -> Self {
        Self {
            logical,
            node,
            physical: None,
        }
    }

    /// Associates a physical qubit.
    #[must_use]
    pub const fn with_physical(
        logical: QubitId,
        node: NodeId,
        physical: PhysicalQubitId,
    ) -> Self {
        Self {
            logical,
            node,
            physical: Some(physical),
        }
    }
}

// ============================================================================
// Communication path
// ============================================================================

/// A scheduler-selected communication path.
///
/// This does not perform routing. It records a path supplied by the routing
/// subsystem or explicitly selected by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationPath {
    /// Ordered nodes traversed by the communication.
    pub nodes: Vec<NodeId>,

    /// Ordered communication links traversed by the communication.
    pub links: Vec<LinkId>,
}

impl CommunicationPath {
    /// Creates a path after validating its basic structural relationship.
    pub fn new(
        nodes: Vec<NodeId>,
        links: Vec<LinkId>,
    ) -> DistributedSchedulingResult<Self> {
        if nodes.is_empty() {
            return Err(DistributedSchedulingError::InvalidRequest(
                "communication path must contain at least one node".to_string(),
            ));
        }

        if nodes.len() != links.len().saturating_add(1) {
            return Err(DistributedSchedulingError::InvalidRequest(
                "communication path must contain exactly one more node than link"
                    .to_string(),
            ));
        }

        Ok(Self { nodes, links })
    }

    /// Returns the source node.
    #[must_use]
    pub fn source(&self) -> NodeId {
        self.nodes[0]
    }

    /// Returns the destination node.
    #[must_use]
    pub fn destination(&self) -> NodeId {
        self.nodes[self.nodes.len() - 1]
    }
}

// ============================================================================
// Operation kinds
// ============================================================================

/// Distributed scheduling operation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedOperationKind {
    /// A local operation with distributed scheduling context.
    Local,

    /// Quantum state transfer.
    Teleportation,

    /// Entanglement generation.
    EntanglementGeneration,

    /// Remote quantum operation.
    RemoteQuantum,

    /// Classical communication.
    ClassicalCommunication,

    /// Synchronization barrier.
    Synchronization,

    /// User-defined distributed scheduling operation.
    Custom,
}

// ============================================================================
// Operation request
// ============================================================================

/// One operation submitted to the distributed scheduler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedOperationRequest {
    /// Canonical quantum IR operation identity.
    pub operation: OperationId,

    /// Operation category.
    pub kind: DistributedOperationKind,

    /// Logical qubits participating in the operation.
    pub qubits: BTreeSet<QubitId>,

    /// Physical qubits, when mapping has already occurred.
    pub physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Explicit communication path supplied by routing.
    pub path: Option<CommunicationPath>,

    /// Duration of the semantic operation.
    pub duration: Duration,

    /// Earliest permitted start.
    pub release_time: TimePoint,

    /// Optional latest completion time.
    pub deadline: Option<TimePoint>,

    /// Required dependency operations.
    pub dependencies: BTreeSet<OperationId>,

    /// Optional explicit scheduler resource requirements.
    pub resources: BTreeMap<ResourceId, u64>,

    /// Optional classical communication latency after operation completion.
    pub classical_latency: Duration,

    /// Whether this operation requires exclusive node occupancy.
    pub exclusive_node: bool,
}

impl DistributedOperationRequest {
    /// Creates a local operation request.
    #[must_use]
    pub fn local(operation: OperationId, duration: Duration) -> Self {
        Self {
            operation,
            kind: DistributedOperationKind::Local,
            qubits: BTreeSet::new(),
            physical_qubits: BTreeSet::new(),
            path: None,
            duration,
            release_time: TimePoint::ZERO,
            deadline: None,
            dependencies: BTreeSet::new(),
            resources: BTreeMap::new(),
            classical_latency: Duration::ZERO,
            exclusive_node: true,
        }
    }

    /// Adds a logical qubit.
    #[must_use]
    pub fn with_qubit(mut self, qubit: QubitId) -> Self {
        self.qubits.insert(qubit);
        self
    }

    /// Adds a physical qubit.
    #[must_use]
    pub fn with_physical_qubit(mut self, qubit: PhysicalQubitId) -> Self {
        self.physical_qubits.insert(qubit);
        self
    }

    /// Assigns a communication path.
    #[must_use]
    pub fn with_path(mut self, path: CommunicationPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets release time.
    #[must_use]
    pub const fn with_release_time(mut self, time: TimePoint) -> Self {
        self.release_time = time;
        self
    }

    /// Sets a deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: TimePoint) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Adds a dependency.
    #[must_use]
    pub fn depends_on(mut self, operation: OperationId) -> Self {
        self.dependencies.insert(operation);
        self
    }

    /// Adds a scheduler resource requirement.
    #[must_use]
    pub fn requires_resource(mut self, resource: ResourceId, quantity: u64) -> Self {
        self.resources.insert(resource, quantity);
        self
    }

    /// Sets classical latency.
    #[must_use]
    pub const fn with_classical_latency(mut self, latency: Duration) -> Self {
        self.classical_latency = latency;
        self
    }

    /// Validates intrinsic request invariants.
    pub fn validate(&self) -> DistributedSchedulingResult<()> {
        if let Some(path) = &self.path {
            if path.links.is_empty()
                && path.nodes.len() > 1
            {
                return Err(DistributedSchedulingError::InvalidRequest(
                    "multi-node operation requires communication links".to_string(),
                ));
            }
        }

        if let Some(deadline) = self.deadline {
            let earliest_finish = self
                .release_time
                .checked_add(self.duration)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            if earliest_finish > deadline {
                return Err(DistributedSchedulingError::DeadlineExceeded(self.operation));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Dependency graph
// ============================================================================

/// Distributed scheduling dependency graph.
#[derive(Debug, Clone, Default)]
pub struct DistributedDependencyGraph {
    predecessors: BTreeMap<OperationId, BTreeSet<OperationId>>,
    successors: BTreeMap<OperationId, BTreeSet<OperationId>>,
}

impl DistributedDependencyGraph {
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an operation node.
    pub fn add_operation(&mut self, operation: OperationId) {
        self.predecessors.entry(operation).or_default();
        self.successors.entry(operation).or_default();
    }

    /// Adds a dependency edge.
    pub fn add_dependency(
        &mut self,
        dependency: OperationId,
        dependent: OperationId,
    ) -> DistributedSchedulingResult<()> {
        self.add_operation(dependency);
        self.add_operation(dependent);

        self.predecessors
            .entry(dependent)
            .or_default()
            .insert(dependency);

        self.successors
            .entry(dependency)
            .or_default()
            .insert(dependent);

        if self.has_cycle() {
            if let Some(predecessors) = self.predecessors.get_mut(&dependent) {
                predecessors.remove(&dependency);
            }

            if let Some(successors) = self.successors.get_mut(&dependency) {
                successors.remove(&dependent);
            }

            return Err(DistributedSchedulingError::DependencyCycle);
        }

        Ok(())
    }

    /// Returns direct predecessors.
    #[must_use]
    pub fn predecessors(&self, operation: OperationId) -> Option<&BTreeSet<OperationId>> {
        self.predecessors.get(&operation)
    }

    /// Returns direct successors.
    #[must_use]
    pub fn successors(&self, operation: OperationId) -> Option<&BTreeSet<OperationId>> {
        self.successors.get(&operation)
    }

    /// Returns all graph operations.
    #[must_use]
    pub fn operations(&self) -> impl Iterator<Item = OperationId> + '_ {
        self.predecessors.keys().copied()
    }

    /// Detects cycles using iterative Kahn traversal.
    #[must_use]
    pub fn has_cycle(&self) -> bool {
        let mut indegree = BTreeMap::new();

        for operation in self.predecessors.keys() {
            let degree = self
                .predecessors
                .get(operation)
                .map_or(0, BTreeSet::len);

            indegree.insert(*operation, degree);
        }

        let mut ready = BTreeSet::new();

        for (operation, degree) in &indegree {
            if *degree == 0 {
                ready.insert(*operation);
            }
        }

        let mut visited = 0usize;

        while let Some(operation) = ready.pop_first() {
            visited = visited.saturating_add(1);

            if let Some(successors) = self.successors.get(&operation) {
                for successor in successors {
                    if let Some(degree) = indegree.get_mut(successor) {
                        *degree = degree.saturating_sub(1);

                        if *degree == 0 {
                            ready.insert(*successor);
                        }
                    }
                }
            }
        }

        visited != indegree.len()
    }

    /// Produces deterministic topological order.
    pub fn topological_order(
        &self,
    ) -> DistributedSchedulingResult<Vec<OperationId>> {
        if self.has_cycle() {
            return Err(DistributedSchedulingError::DependencyCycle);
        }

        let mut indegree = BTreeMap::new();

        for operation in self.predecessors.keys() {
            indegree.insert(
                *operation,
                self.predecessors
                    .get(operation)
                    .map_or(0, BTreeSet::len),
            );
        }

        let mut ready = BTreeSet::new();

        for (operation, degree) in &indegree {
            if *degree == 0 {
                ready.insert(*operation);
            }
        }

        let mut result = Vec::with_capacity(indegree.len());

        while let Some(operation) = ready.pop_first() {
            result.push(operation);

            if let Some(successors) = self.successors.get(&operation) {
                for successor in successors {
                    if let Some(degree) = indegree.get_mut(successor) {
                        *degree = degree.saturating_sub(1);

                        if *degree == 0 {
                            ready.insert(*successor);
                        }
                    }
                }
            }
        }

        if result.len() != indegree.len() {
            return Err(DistributedSchedulingError::DependencyCycle);
        }

        Ok(result)
    }
}

// ============================================================================
// Reservations
// ============================================================================

/// A distributed scheduling reservation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedReservation {
    /// Scheduler reservation identity.
    pub reservation: ReservationId,

    /// Source operation.
    pub operation: OperationId,

    /// Optional scheduler resource.
    pub resource: Option<ResourceId>,

    /// Optional communication link.
    pub link: Option<LinkId>,

    /// Node on which the reservation occurs.
    pub node: Option<NodeId>,

    /// Start time.
    pub start: TimePoint,

    /// Duration.
    pub duration: Duration,

    /// End time.
    pub end: TimePoint,

    /// Reserved capacity.
    pub quantity: u64,
}

impl DistributedReservation {
    /// Creates a reservation.
    pub fn new(
        reservation: ReservationId,
        operation: OperationId,
        start: TimePoint,
        duration: Duration,
        quantity: u64,
    ) -> DistributedSchedulingResult<Self> {
        let end = start
            .checked_add(duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

        Ok(Self {
            reservation,
            operation,
            resource: None,
            link: None,
            node: None,
            start,
            duration,
            end,
            quantity,
        })
    }

    /// Returns true when two reservations overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

// ============================================================================
// Scheduled operation
// ============================================================================

/// Result of scheduling one distributed operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledDistributedOperation {
    /// Source canonical operation.
    pub operation: OperationId,

    /// Distributed operation category.
    pub kind: DistributedOperationKind,

    /// Scheduled start.
    pub start: TimePoint,

    /// Scheduled duration.
    pub duration: Duration,

    /// Scheduled completion.
    pub end: TimePoint,

    /// Node participants.
    pub nodes: BTreeSet<NodeId>,

    /// Logical qubits.
    pub qubits: BTreeSet<QubitId>,

    /// Physical qubits.
    pub physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Communication links consumed.
    pub links: BTreeSet<LinkId>,

    /// Reservations created for the operation.
    pub reservations: Vec<ReservationId>,
}

// ============================================================================
// Schedule
// ============================================================================

/// Complete distributed schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedSchedule {
    /// Stable schedule identity.
    pub id: ScheduleId,

    /// Scheduling epoch.
    pub epoch: EpochId,

    /// Scheduled operations.
    pub operations: BTreeMap<OperationId, ScheduledDistributedOperation>,

    /// All resource reservations.
    pub reservations: BTreeMap<ReservationId, DistributedReservation>,

    /// Overall makespan.
    pub makespan: TimePoint,

    /// Whether verification succeeded.
    pub verified: bool,
}

impl DistributedSchedule {
    /// Creates an empty schedule.
    #[must_use]
    pub fn new(id: ScheduleId, epoch: EpochId) -> Self {
        Self {
            id,
            epoch,
            operations: BTreeMap::new(),
            reservations: BTreeMap::new(),
            makespan: TimePoint::ZERO,
            verified: false,
        }
    }

    /// Inserts a scheduled operation.
    pub fn insert_operation(
        &mut self,
        operation: ScheduledDistributedOperation,
    ) {
        if operation.end > self.makespan {
            self.makespan = operation.end;
        }

        self.operations.insert(operation.operation, operation);
    }

    /// Inserts a reservation.
    pub fn insert_reservation(
        &mut self,
        reservation: DistributedReservation,
    ) {
        self.reservations
            .insert(reservation.reservation, reservation);
    }
}

// ============================================================================
// Target snapshot
// ============================================================================

/// Immutable target snapshot consumed by distributed scheduling.
///
/// Hardware discovery and routing happen outside this module. The scheduler
/// receives a snapshot and remains deterministic against that snapshot.
#[derive(Debug, Clone, Default)]
pub struct DistributedSchedulingTarget {
    /// Distributed node resources.
    pub nodes: BTreeMap<NodeId, NodeScheduleResource>,

    /// Communication resources.
    pub links: BTreeMap<LinkId, CommunicationResource>,

    /// Logical-to-node placement.
    pub qubit_locations: BTreeMap<QubitId, QubitLocation>,

    /// Physical-qubit-to-node placement.
    pub physical_locations: BTreeMap<PhysicalQubitId, NodeId>,

    /// Additional scheduler resources.
    pub resource_capacities: BTreeMap<ResourceId, Capacity>,
}

impl DistributedSchedulingTarget {
    /// Creates an empty target snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a node.
    pub fn add_node(&mut self, node: NodeScheduleResource) {
        self.nodes.insert(node.node, node);
    }

    /// Registers a communication resource.
    pub fn add_link(&mut self, link: CommunicationResource) {
        self.links.insert(link.link, link);
    }

    /// Associates a logical qubit with a node.
    pub fn add_qubit_location(&mut self, location: QubitLocation) {
        if let Some(physical) = location.physical {
            self.physical_locations.insert(physical, location.node);
        }

        self.qubit_locations.insert(location.logical, location);
    }

    /// Registers a scheduler resource.
    pub fn add_resource(
        &mut self,
        resource: ResourceId,
        capacity: Capacity,
    ) {
        self.resource_capacities.insert(resource, capacity);
    }

    /// Validates target references.
    pub fn validate(&self) -> DistributedSchedulingResult<()> {
        for location in self.qubit_locations.values() {
            if !self.nodes.contains_key(&location.node) {
                return Err(DistributedSchedulingError::UnknownNode(location.node));
            }

            if let Some(physical) = location.physical {
                if !self.physical_locations.contains_key(&physical) {
                    return Err(
                        DistributedSchedulingError::PhysicalQubitLocationMissing(
                            physical,
                        ),
                    );
                }
            }
        }

        for link in self.links.values() {
            if !self.nodes.contains_key(&link.source) {
                return Err(DistributedSchedulingError::UnknownNode(link.source));
            }

            if !self.nodes.contains_key(&link.destination) {
                return Err(DistributedSchedulingError::UnknownNode(
                    link.destination,
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Scheduling policy
// ============================================================================

/// Distributed scheduling policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedSchedulingPolicy {
    /// Earliest feasible execution.
    EarliestStart,

    /// Critical/dependency order with earliest feasible execution.
    DependencyAware,

    /// Prefer communication resources with lower occupancy.
    CommunicationAware,
}

/// Distributed scheduling configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedSchedulingConfig {
    /// Scheduling policy.
    pub policy: DistributedSchedulingPolicy,

    /// Whether to verify the complete result before returning it.
    pub verify: bool,

    /// Whether deterministic ordering is required.
    pub deterministic: bool,
}

impl Default for DistributedSchedulingConfig {
    fn default() -> Self {
        Self {
            policy: DistributedSchedulingPolicy::DependencyAware,
            verify: true,
            deterministic: true,
        }
    }
}

// ============================================================================
// Scheduler
// ============================================================================

/// Production distributed scheduler.
///
/// The scheduler is stateful only within an explicitly owned compilation
/// session. It contains no global mutable state.
#[derive(Debug, Clone)]
pub struct DistributedScheduler {
    config: DistributedSchedulingConfig,
}

impl DistributedScheduler {
    /// Creates a scheduler from explicit configuration.
    #[must_use]
    pub const fn new(config: DistributedSchedulingConfig) -> Self {
        Self { config }
    }

    /// Returns scheduler configuration.
    #[must_use]
    pub const fn config(&self) -> &DistributedSchedulingConfig {
        &self.config
    }

    /// Builds a dependency graph from operation requests.
    pub fn build_dependency_graph(
        &self,
        operations: &[DistributedOperationRequest],
    ) -> DistributedSchedulingResult<DistributedDependencyGraph> {
        let mut graph = DistributedDependencyGraph::new();

        let operation_ids: BTreeSet<_> =
            operations.iter().map(|operation| operation.operation).collect();

        if operation_ids.len() != operations.len() {
            return Err(DistributedSchedulingError::InvalidRequest(
                "duplicate operation identity".to_string(),
            ));
        }

        for operation in operations {
            operation.validate()?;
            graph.add_operation(operation.operation);
        }

        for operation in operations {
            for dependency in &operation.dependencies {
                if !operation_ids.contains(dependency) {
                    return Err(
                        DistributedSchedulingError::UnknownOperation(*dependency)
                    );
                }

                graph.add_dependency(*dependency, operation.operation)?;
            }
        }

        Ok(graph)
    }

    /// Schedules distributed operations against an immutable target snapshot.
    ///
    /// This is a deterministic earliest-feasible scheduler. It is deliberately
    /// target-agnostic: all machine-specific information enters through
    /// `DistributedSchedulingTarget`.
    pub fn schedule(
        &self,
        schedule_id: ScheduleId,
        epoch: EpochId,
        target: &DistributedSchedulingTarget,
        operations: &[DistributedOperationRequest],
    ) -> DistributedSchedulingResult<DistributedSchedule> {
        target.validate()?;

        let graph = self.build_dependency_graph(operations)?;

        let by_id: BTreeMap<OperationId, &DistributedOperationRequest> =
            operations
                .iter()
                .map(|operation| (operation.operation, operation))
                .collect();

        let order = graph.topological_order()?;

        let mut schedule = DistributedSchedule::new(schedule_id, epoch);

        for operation_id in order {
            let operation = by_id
                .get(&operation_id)
                .copied()
                .ok_or(DistributedSchedulingError::UnknownOperation(
                    operation_id,
                ))?;

            let earliest = self.earliest_dependency_completion(
                &graph,
                operation,
                &schedule,
            )?;

            let start = if earliest > operation.release_time {
                earliest
            } else {
                operation.release_time
            };

            let scheduled = self.schedule_one(
                &mut schedule,
                target,
                operation,
                start,
            )?;

            schedule.insert_operation(scheduled);
        }

        if self.config.verify {
            verify(&schedule, target, operations, &graph)?;
            schedule.verified = true;
        }

        Ok(schedule)
    }

    fn earliest_dependency_completion(
        &self,
        graph: &DistributedDependencyGraph,
        operation: &DistributedOperationRequest,
        schedule: &DistributedSchedule,
    ) -> DistributedSchedulingResult<TimePoint> {
        let mut earliest = operation.release_time;

        let predecessors = graph
            .predecessors(operation.operation)
            .ok_or(DistributedSchedulingError::UnknownOperation(
                operation.operation,
            ))?;

        for predecessor in predecessors {
            let scheduled = schedule
                .operations
                .get(predecessor)
                .ok_or(DistributedSchedulingError::UnknownOperation(
                    *predecessor,
                ))?;

            let mut completion = scheduled.end;

            completion = completion
                .checked_add(operation.classical_latency)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            if completion > earliest {
                earliest = completion;
            }
        }

        Ok(earliest)
    }

    fn schedule_one(
        &self,
        schedule: &mut DistributedSchedule,
        target: &DistributedSchedulingTarget,
        operation: &DistributedOperationRequest,
        initial_start: TimePoint,
    ) -> DistributedSchedulingResult<ScheduledDistributedOperation> {
        let nodes = resolve_nodes(target, operation)?;

        let links = operation
            .path
            .as_ref()
            .map(|path| path.links.iter().copied().collect())
            .unwrap_or_default();

        let mut start = initial_start;

        loop {
            let end = start
                .checked_add(operation.duration)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            if let Some(deadline) = operation.deadline {
                if end > deadline {
                    return Err(
                        DistributedSchedulingError::DeadlineExceeded(
                            operation.operation,
                        ),
                    );
                }
            }

            if !self.nodes_available(
                schedule,
                target,
                &nodes,
                start,
                operation.duration,
                operation.exclusive_node,
            )? {
                start = next_conflict_end(
                    schedule,
                    start,
                    operation.duration,
                    &nodes,
                    &links,
                )?;

                continue;
            }

            if !self.links_available(
                schedule,
                target,
                &links,
                start,
                operation.duration,
            )? {
                start = next_link_conflict_end(
                    schedule,
                    start,
                    operation.duration,
                    &links,
                )?;

                continue;
            }

            if !self.resources_available(
                schedule,
                target,
                &operation.resources,
                start,
                operation.duration,
            )? {
                start = next_resource_conflict_end(
                    schedule,
                    start,
                    operation.duration,
                    &operation.resources,
                )?;

                continue;
            }

            break;
        }

        let end = start
            .checked_add(operation.duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

        let mut reservation_counter = 1u64;
        let mut reservation_ids = Vec::new();

        for node in &nodes {
            let reservation_id = ReservationId::new(reservation_counter);
            reservation_counter = reservation_counter
                .checked_add(1)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            let mut reservation = DistributedReservation::new(
                reservation_id,
                operation.operation,
                start,
                operation.duration,
                1,
            )?;

            reservation.node = Some(*node);

            schedule.insert_reservation(reservation);
            reservation_ids.push(reservation_id);
        }

        for link in &links {
            let reservation_id = ReservationId::new(reservation_counter);
            reservation_counter = reservation_counter
                .checked_add(1)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            let mut reservation = DistributedReservation::new(
                reservation_id,
                operation.operation,
                start,
                operation.duration,
                1,
            )?;

            reservation.link = Some(*link);

            schedule.insert_reservation(reservation);
            reservation_ids.push(reservation_id);
        }

        for (resource, quantity) in &operation.resources {
            let reservation_id = ReservationId::new(reservation_counter);
            reservation_counter = reservation_counter
                .checked_add(1)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            let reservation = DistributedReservation {
                reservation: reservation_id,
                operation: operation.operation,
                resource: Some(*resource),
                link: None,
                node: None,
                start,
                duration: operation.duration,
                end,
                quantity: *quantity,
            };

            schedule.insert_reservation(reservation);
            reservation_ids.push(reservation_id);
        }

        Ok(ScheduledDistributedOperation {
            operation: operation.operation,
            kind: operation.kind,
            start,
            duration: operation.duration,
            end,
            nodes,
            qubits: operation.qubits.clone(),
            physical_qubits: operation.physical_qubits.clone(),
            links,
            reservations: reservation_ids,
        })
    }

    fn nodes_available(
        &self,
        schedule: &DistributedSchedule,
        target: &DistributedSchedulingTarget,
        nodes: &BTreeSet<NodeId>,
        start: TimePoint,
        duration: Duration,
        exclusive: bool,
    ) -> DistributedSchedulingResult<bool> {
        for node in nodes {
            let resource = target
                .nodes
                .get(node)
                .ok_or(DistributedSchedulingError::UnknownNode(*node))?;

            if !resource.availability.is_empty() {
                let available = resource
                    .availability
                    .iter()
                    .try_fold(false, |found, window| {
                        if found {
                            Ok(true)
                        } else {
                            window.contains(start, duration)
                        }
                    })?;

                if !available {
                    return Ok(false);
                }
            }

            let used = schedule
                .reservations
                .values()
                .filter(|reservation| {
                    reservation.node == Some(*node)
                        && reservation.start < start
                            .checked_add(duration)
                            .unwrap_or(start)
                        && start < reservation.end
                })
                .fold(0u64, |total, reservation| {
                    total.saturating_add(reservation.quantity)
                });

            let requested = if exclusive { 1 } else { 1 };

            if resource.capacity != Capacity::Unbounded {
                let capacity = match resource.capacity {
                    Capacity::Finite(value) => value,
                    Capacity::Unbounded => 0,
                };

                if used.saturating_add(requested) > capacity {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    fn links_available(
        &self,
        schedule: &DistributedSchedule,
        target: &DistributedSchedulingTarget,
        links: &BTreeSet<LinkId>,
        start: TimePoint,
        duration: Duration,
    ) -> DistributedSchedulingResult<bool> {
        for link_id in links {
            let link = target
                .links
                .get(link_id)
                .ok_or(DistributedSchedulingError::UnknownLink(
                    *link_id,
                ))?;

            if !link.supports(start, duration)? {
                return Ok(false);
            }

            let end = start
                .checked_add(duration)
                .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

            let used = schedule
                .reservations
                .values()
                .filter(|reservation| {
                    reservation.link == Some(*link_id)
                        && reservation.start < end
                        && start < reservation.end
                })
                .fold(0u64, |total, reservation| {
                    total.saturating_add(reservation.quantity)
                });

            if !link.capacity.can_fit(
                used.saturating_add(1),
            ) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn resources_available(
        &self,
        schedule: &DistributedSchedule,
        target: &DistributedSchedulingTarget,
        resources: &BTreeMap<ResourceId, u64>,
        start: TimePoint,
        duration: Duration,
    ) -> DistributedSchedulingResult<bool> {
        let end = start
            .checked_add(duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

        for (resource, requested) in resources {
            let capacity = target
                .resource_capacities
                .get(resource)
                .copied()
                .ok_or(
                    DistributedSchedulingError::ResourceUnavailable(*resource),
                )?;

            let used = schedule
                .reservations
                .values()
                .filter(|reservation| {
                    reservation.resource == Some(*resource)
                        && reservation.start < end
                        && start < reservation.end
                })
                .fold(0u64, |total, reservation| {
                    total.saturating_add(reservation.quantity)
                });

            if !capacity.can_fit(used.saturating_add(*requested)) {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

// ============================================================================
// Resolution helpers
// ============================================================================

fn resolve_nodes(
    target: &DistributedSchedulingTarget,
    operation: &DistributedOperationRequest,
) -> DistributedSchedulingResult<BTreeSet<NodeId>> {
    let mut nodes = BTreeSet::new();

    for qubit in &operation.qubits {
        let location = target
            .qubit_locations
            .get(qubit)
            .ok_or(
                DistributedSchedulingError::QubitLocationMissing(*qubit)
            )?;

        nodes.insert(location.node);
    }

    for qubit in &operation.physical_qubits {
        let node = target
            .physical_locations
            .get(qubit)
            .copied()
            .ok_or(
                DistributedSchedulingError::PhysicalQubitLocationMissing(
                    *qubit,
                ),
            )?;

        nodes.insert(node);
    }

    if let Some(path) = &operation.path {
        for node in &path.nodes {
            if !target.nodes.contains_key(node) {
                return Err(DistributedSchedulingError::UnknownNode(*node));
            }

            nodes.insert(*node);
        }
    }

    Ok(nodes)
}

fn next_conflict_end(
    schedule: &DistributedSchedule,
    start: TimePoint,
    duration: Duration,
    nodes: &BTreeSet<NodeId>,
    links: &BTreeSet<LinkId>,
) -> DistributedSchedulingResult<TimePoint> {
    let end = start
        .checked_add(duration)
        .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

    let mut candidate = end;

    for reservation in schedule.reservations.values() {
        let relevant = reservation
            .node
            .map(|node| nodes.contains(&node))
            .unwrap_or(false)
            || reservation
                .link
                .map(|link| links.contains(&link))
                .unwrap_or(false);

        if relevant && reservation.start < end && start < reservation.end {
            if reservation.end > candidate {
                candidate = reservation.end;
            }
        }
    }

    Ok(candidate)
}

fn next_link_conflict_end(
    schedule: &DistributedSchedule,
    start: TimePoint,
    duration: Duration,
    links: &BTreeSet<LinkId>,
) -> DistributedSchedulingResult<TimePoint> {
    let end = start
        .checked_add(duration)
        .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

    let mut candidate = end;

    for reservation in schedule.reservations.values() {
        if let Some(link) = reservation.link {
            if links.contains(&link)
                && reservation.start < end
                && start < reservation.end
                && reservation.end > candidate
            {
                candidate = reservation.end;
            }
        }
    }

    Ok(candidate)
}

fn next_resource_conflict_end(
    schedule: &DistributedSchedule,
    start: TimePoint,
    duration: Duration,
    resources: &BTreeMap<ResourceId, u64>,
) -> DistributedSchedulingResult<TimePoint> {
    let end = start
        .checked_add(duration)
        .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

    let mut candidate = end;

    for reservation in schedule.reservations.values() {
        if let Some(resource) = reservation.resource {
            if resources.contains_key(&resource)
                && reservation.start < end
                && start < reservation.end
                && reservation.end > candidate
            {
                candidate = reservation.end;
            }
        }
    }

    Ok(candidate)
}

// ============================================================================
// Verification
// ============================================================================

/// Verifies a distributed schedule against its source request and dependency
/// graph.
///
/// This function performs structural, dependency, timing, resource and
/// topology-independent checks. It deliberately does not contact hardware.
pub fn verify(
    schedule: &DistributedSchedule,
    target: &DistributedSchedulingTarget,
    operations: &[DistributedOperationRequest],
    graph: &DistributedDependencyGraph,
) -> DistributedSchedulingResult<()> {
    if graph.has_cycle() {
        return Err(DistributedSchedulingError::DependencyCycle);
    }

    let requests: BTreeMap<OperationId, &DistributedOperationRequest> =
        operations
            .iter()
            .map(|operation| (operation.operation, operation))
            .collect();

    if schedule.operations.len() != requests.len() {
        return Err(DistributedSchedulingError::VerificationFailed(
            "schedule does not contain exactly one result for each operation"
                .to_string(),
        ));
    }

    for (operation_id, scheduled) in &schedule.operations {
        let request = requests.get(operation_id).ok_or(
            DistributedSchedulingError::UnknownOperation(*operation_id),
        )?;

        let expected_end = scheduled
            .start
            .checked_add(scheduled.duration)
            .ok_or(DistributedSchedulingError::ArithmeticOverflow)?;

        if expected_end != scheduled.end {
            return Err(DistributedSchedulingError::VerificationFailed(
                format!(
                    "operation {:?} has inconsistent start/duration/end",
                    operation_id
                ),
            ));
        }

        if scheduled.start < request.release_time {
            return Err(DistributedSchedulingError::VerificationFailed(
                format!(
                    "operation {:?} starts before its release time",
                    operation_id
                ),
            ));
        }

        if let Some(deadline) = request.deadline {
            if scheduled.end > deadline {
                return Err(DistributedSchedulingError::DeadlineExceeded(
                    *operation_id,
                ));
            }
        }

        for node in &scheduled.nodes {
            if !target.nodes.contains_key(node) {
                return Err(DistributedSchedulingError::UnknownNode(*node));
            }
        }

        for link in &scheduled.links {
            if !target.links.contains_key(link) {
                return Err(DistributedSchedulingError::UnknownLink(*link));
            }
        }

        for dependency in graph
            .predecessors(*operation_id)
            .into_iter()
            .flat_map(|set| set.iter())
        {
            let predecessor = schedule.operations.get(dependency).ok_or(
                DistributedSchedulingError::UnknownOperation(*dependency),
            )?;

            if predecessor.end > scheduled.start {
                return Err(DistributedSchedulingError::VerificationFailed(
                    format!(
                        "dependency {:?} completes after dependent {:?} starts",
                        dependency, operation_id
                    ),
                ));
            }
        }
    }

    verify_reservation_conflicts(schedule, target)?;

    Ok(())
}

fn verify_reservation_conflicts(
    schedule: &DistributedSchedule,
    target: &DistributedSchedulingTarget,
) -> DistributedSchedulingResult<()> {
    let reservations: Vec<&DistributedReservation> =
        schedule.reservations.values().collect();

    for left_index in 0..reservations.len() {
        for right_index in (left_index + 1)..reservations.len() {
            let left = reservations[left_index];
            let right = reservations[right_index];

            if !left.overlaps(right) {
                continue;
            }

            if left.node.is_some()
                && left.node == right.node
            {
                let node = left.node.ok_or(
                    DistributedSchedulingError::VerificationFailed(
                        "node reservation disappeared".to_string(),
                    ),
                )?;

                let capacity = target
                    .nodes
                    .get(&node)
                    .ok_or(
                        DistributedSchedulingError::UnknownNode(node)
                    )?
                    .capacity;

                if capacity != Capacity::Unbounded
                    && matches!(
                        capacity,
                        Capacity::Finite(value) if left.quantity
                            .saturating_add(right.quantity)
                            > value
                    )
                {
                    return Err(
                        DistributedSchedulingError::VerificationFailed(
                            format!(
                                "node {} exceeds declared scheduling capacity",
                                node
                            ),
                        ),
                    );
                }
            }

            if left.link.is_some() && left.link == right.link {
                let link = left.link.ok_or(
                    DistributedSchedulingError::VerificationFailed(
                        "link reservation disappeared".to_string(),
                    ),
                )?;

                let capacity = target
                    .links
                    .get(&link)
                    .ok_or(
                        DistributedSchedulingError::UnknownLink(link)
                    )?
                    .capacity;

                if capacity != Capacity::Unbounded
                    && matches!(
                        capacity,
                        Capacity::Finite(value) if left.quantity
                            .saturating_add(right.quantity)
                            > value
                    )
                {
                    return Err(
                        DistributedSchedulingError::LinkCapacityExceeded(
                            link,
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// Public convenience API
// ============================================================================

/// Schedules a distributed operation set using the default production
/// configuration.
///
/// This function is deliberately free of global state.
pub fn schedule(
    schedule_id: ScheduleId,
    epoch: EpochId,
    target: &DistributedSchedulingTarget,
    operations: &[DistributedOperationRequest],
) -> DistributedSchedulingResult<DistributedSchedule> {
    DistributedScheduler::new(DistributedSchedulingConfig::default())
        .schedule(schedule_id, epoch, target, operations)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn node(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn link(value: u64) -> LinkId {
        LinkId::new(value)
    }

    #[test]
    fn empty_schedule_is_valid() {
        let target = DistributedSchedulingTarget::new();

        let schedule = schedule(
            ScheduleId::new(1),
            EpochId::new(1),
            &target,
            &[],
        )
        .expect("empty schedule must be valid");

        assert!(schedule.operations.is_empty());
        assert!(schedule.reservations.is_empty());
        assert!(schedule.verified);
    }

    #[test]
    fn dependency_order_is_preserved() {
        let mut target = DistributedSchedulingTarget::new();

        target.add_node(NodeScheduleResource::new(
            node(1),
            Capacity::finite(1),
        ));

        let first = DistributedOperationRequest::local(
            operation(1),
            Duration::new(10),
        );

        let second = DistributedOperationRequest::local(
            operation(2),
            Duration::new(10),
        )
        .depends_on(operation(1));

        let result = schedule(
            ScheduleId::new(1),
            EpochId::new(1),
            &target,
            &[first, second],
        )
        .expect("dependent operations should schedule");

        let first_result = result
            .operations
            .get(&operation(1))
            .expect("first operation");

        let second_result = result
            .operations
            .get(&operation(2))
            .expect("second operation");

        assert!(first_result.end <= second_result.start);
    }

    #[test]
    fn link_conflict_is_serialized() {
        let mut target = DistributedSchedulingTarget::new();

        target.add_node(NodeScheduleResource::new(
            node(1),
            Capacity::unbounded(),
        ));

        target.add_node(NodeScheduleResource::new(
            node(2),
            Capacity::unbounded(),
        ));

        target.add_link(CommunicationResource::new(
            link(1),
            node(1),
            node(2),
            CommunicationKind::Quantum,
            Capacity::finite(1),
            Duration::new(10),
            Duration::ZERO,
        ));

        let path = CommunicationPath::new(
            vec![node(1), node(2)],
            vec![link(1)],
        )
        .expect("valid path");

        let first = DistributedOperationRequest::local(
            operation(1),
            Duration::new(10),
        )
        .with_path(path.clone());

        let second = DistributedOperationRequest::local(
            operation(2),
            Duration::new(10),
        )
        .with_path(path);

        let result = schedule(
            ScheduleId::new(1),
            EpochId::new(1),
            &target,
            &[first, second],
        )
        .expect("link-conflicting operations should serialize");

        let first_result = result
            .operations
            .get(&operation(1))
            .expect("first operation");

        let second_result = result
            .operations
            .get(&operation(2))
            .expect("second operation");

        assert!(
            first_result.end <= second_result.start
                || second_result.end <= first_result.start
        );
    }

    #[test]
    fn cycle_is_rejected() {
        let mut target = DistributedSchedulingTarget::new();

        target.add_node(NodeScheduleResource::new(
            node(1),
            Capacity::unbounded(),
        ));

        let first = DistributedOperationRequest::local(
            operation(1),
            Duration::new(1),
        )
        .depends_on(operation(2));

        let second = DistributedOperationRequest::local(
            operation(2),
            Duration::new(1),
        )
        .depends_on(operation(1));

        let result = schedule(
            ScheduleId::new(1),
            EpochId::new(1),
            &target,
            &[first, second],
        );

        assert!(matches!(
            result,
            Err(DistributedSchedulingError::DependencyCycle)
        ));
    }

    #[test]
    fn qubit_identity_comes_from_canonical_ir() {
        let logical = QubitId::new(7);
        let location = QubitLocation::logical(logical, node(1));

        assert_eq!(location.logical, logical);
        assert_eq!(location.node, node(1));
    }

    #[test]
    fn availability_window_is_checked() {
        let window = AvailabilityWindow::new(
            TimePoint::new(10),
            Duration::new(10),
        );

        assert!(
            window
                .contains(TimePoint::new(10), Duration::new(10))
                .expect("valid interval")
        );

        assert!(
            !window
                .contains(TimePoint::new(5), Duration::new(5))
                .expect("valid interval")
        );
    }

    #[test]
    fn unbounded_capacity_does_not_use_sentinel_values() {
        assert!(Capacity::unbounded().can_fit(u64::MAX));
        assert!(Capacity::unbounded().can_fit(u64::MAX));
    }
}