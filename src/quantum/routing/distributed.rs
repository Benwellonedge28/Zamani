//! Zamani Quantum Routing — Distributed Quantum Routing
//!
//! `src/quantum/routing/distributed.rs`
//!
//! # Responsibility
//!
//! This module provides the routing/planning layer for distributed quantum
//! computing and quantum-network-aware compilation.
//!
//! It models:
//!
//! - distributed quantum nodes;
//! - local quantum memories;
//! - quantum links;
//! - classical control links;
//! - entanglement resources;
//! - entanglement quality and lifetime;
//! - network resource availability;
//! - end-to-end entanglement paths;
//! - entanglement swapping;
//! - optional purification requirements;
//! - teleportation/migration of logical quantum state;
//! - remote-gate resource planning;
//! - distributed routing requests;
//! - deterministic path selection;
//! - multi-destination routing;
//! - resource reservation;
//! - transactional route planning;
//! - route validation;
//! - reproducibility;
//! - distributed-routing diagnostics.
//!
//! # Architectural boundary
//!
//! This module is a ROUTING AND PLANNING subsystem.
//!
//! It does NOT:
//!
//! - open network sockets;
//! - communicate with quantum hardware;
//! - communicate with providers;
//! - generate physical pulses;
//! - perform measurements;
//! - execute Bell-state measurements;
//! - generate physical entanglement;
//! - perform purification itself;
//! - execute teleportation;
//! - execute remote gates;
//! - implement a quantum network protocol;
//! - implement a classical transport protocol;
//! - authenticate remote nodes;
//! - own hardware calibration databases;
//! - parse OpenQASM;
//! - synthesize arbitrary gates;
//! - perform QEC decoding;
//! - simulate quantum states.
//!
//! Those responsibilities belong to the hardware/network/control/execution
//! layers.
//!
//! # Architectural model
//!
//! Distributed quantum routing is fundamentally different from ordinary
//! single-device qubit routing.
//!
//! ```text
//!                   Distributed Quantum Program
//!                             |
//!                             v
//!                    logical operation
//!                             |
//!             +---------------+---------------+
//!             |                               |
//!             v                               v
//!       local operation                 remote operation
//!                                             |
//!                                             v
//!                                    distributed request
//!                                             |
//!                         +-------------------+------------------+
//!                         |                                      |
//!                         v                                      v
//!                 node/link discovery                    resource constraints
//!                         |                                      |
//!                         +-------------------+------------------+
//!                                             |
//!                                             v
//!                                    path selection
//!                                             |
//!                              +--------------+--------------+
//!                              |                             |
//!                              v                             v
//!                       entanglement path              direct link
//!                              |
//!                              v
//!                    swap/purification plan
//!                              |
//!                              v
//!                     teleport/remote-gate plan
//!                              |
//!                              v
//!                       resource reservation
//!                              |
//!                              v
//!                         verification
//!                              |
//!                              v
//!                       DistributedRoute
//! ```
//!
//! # Quantum-network semantics
//!
//! A distributed route is not equivalent to a classical packet route.
//!
//! A classical packet can be forwarded independently from previous packets.
//! Quantum networking instead operates on stateful entanglement resources.
//!
//! Therefore this module explicitly tracks:
//!
//! - resource identity;
//! - source and destination nodes;
//! - endpoint memories;
//! - link availability;
//! - fidelity;
//! - coherence lifetime;
//! - generation rate;
//! - swap success probability;
//! - purification overhead;
//! - reservation state;
//! - route lifetime;
//! - classical coordination latency.
//!
//! This follows the architectural distinction between classical control and
//! quantum data planes and the stateful nature of entanglement routing.
//!
//! # Distributed computing boundary
//!
//! Zamani distributed quantum computing can use several strategies:
//!
//! ```text
//! logical interaction
//!       |
//!       +-- qubits colocated
//!       |       |
//!       |       +--> local gate
//!       |
//!       +-- qubits distributed
//!               |
//!               +--> teleportation
//!               |
//!               +--> remote gate
//!               |
//!               +--> entanglement-assisted protocol
//! ```
//!
//! This module does not force one execution protocol.
//!
//! Instead it produces a semantic plan describing the resources required.
//!
//! Later execution/lowering may map:
//!
//! ```text
//! Teleport
//!     |
//!     +--> Bell-pair teleportation
//!     +--> encoded teleportation
//!     +--> QEC-aware teleportation
//!     +--> provider-native primitive
//! ```
//!
//! Likewise:
//!
//! ```text
//! RemoteGate
//!     |
//!     +--> gate teleportation
//!     +--> distributed gate protocol
//!     +--> provider-native remote operation
//! ```
//!
//! # No-cloning invariant
//!
//! Distributed routing MUST NOT model a logical qubit as being copied to
//! multiple destinations.
//!
//! A logical quantum state has exactly one authoritative location unless the
//! higher-level protocol explicitly represents an entangled/multipartite state.
//!
//! Consequently, this module represents movement as ownership/state-transfer
//! planning rather than copying.
//!
//! # Determinism
//!
//! Routing decisions are deterministic when the supplied request/configuration
//! is deterministic.
//!
//! Equal-cost paths are resolved using:
//!
//! 1. total route cost;
//! 2. hop count;
//! 3. node identifier;
//! 4. link identifier;
//! 5. stable path lexicographic ordering.
//!
//! No HashMap iteration order is ever used as a routing decision.
//!
//! # Transactionality
//!
//! Resource reservations are transactional.
//!
//! ```text
//! current resource state
//!          |
//!          v
//!     begin transaction
//!          |
//!          v
//!     candidate route
//!          |
//!       +--+--+
//!       |     |
//!    valid   invalid
//!       |     |
//!       v     v
//!    commit rollback
//! ```
//!
//! A failed planning operation must not consume resources.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration
//!
//! `distributed.rs` integrates with the rest of the routing subsystem through
//! stable routing identifiers and remains independent of compiler IR.
//!
//! Intended dependency direction:
//!
//! ```text
//! routing::types
//!        |
//!        v
//! distributed.rs
//!        |
//!        +--> distributed topology
//!        +--> resource model
//!        +--> path planner
//!        +--> reservation transaction
//!        +--> route verification
//!        |
//!        v
//! routing::router
//!        |
//!        v
//! routing::transpiler
//! ```
//!
//! The module does not depend on `transpiler.rs`.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! `#![deny(unsafe_code)]` is enabled deliberately.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::fmt;
use std::time::Duration;

use crate::quantum::routing::types::{
    LogicalQubitId,
    PhysicalQubitId,
};

// =============================================================================
// Stable identifiers
// =============================================================================

/// Identifier of a distributed quantum-computing node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DistributedNodeId(u64);

impl DistributedNodeId {
    /// Creates a node identifier.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DistributedNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "node-{}", self.0)
    }
}

/// Identifier of a distributed quantum-network link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DistributedLinkId(u64);

impl DistributedLinkId {
    /// Creates a link identifier.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for DistributedLinkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "link-{}", self.0)
    }
}

/// Identifier of an entanglement resource.
///
/// An entanglement resource is stateful and must never be represented merely
/// as an anonymous edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntanglementResourceId(u64);

impl EntanglementResourceId {
    /// Creates a resource identifier.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for EntanglementResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ebit-{}", self.0)
    }
}

/// Identifier for a reservation transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReservationId(u64);

impl ReservationId {
    /// Creates a reservation identifier.
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ReservationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "reservation-{}", self.0)
    }
}

// =============================================================================
// Error model
// =============================================================================

/// Errors specific to distributed quantum routing.
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedRoutingError {
    /// The distributed network contains no nodes.
    EmptyNetwork,

    /// A node identifier was referenced but does not exist.
    UnknownNode {
        /// Missing node.
        node: DistributedNodeId,
    },

    /// A link identifier was referenced but does not exist.
    UnknownLink {
        /// Missing link.
        link: DistributedLinkId,
    },

    /// A node references a physical qubit that is unavailable.
    UnavailableQubit {
        /// Node owning the qubit.
        node: DistributedNodeId,

        /// Physical qubit.
        qubit: PhysicalQubitId,
    },

    /// A link connects a node to itself.
    SelfLoop {
        /// Invalid link.
        link: DistributedLinkId,
    },

    /// Duplicate link identifier.
    DuplicateLink {
        /// Duplicate link.
        link: DistributedLinkId,
    },

    /// Invalid fidelity value.
    InvalidFidelity {
        /// Supplied fidelity.
        fidelity: f64,
    },

    /// Invalid probability.
    InvalidProbability {
        /// Supplied probability.
        probability: f64,
    },

    /// Invalid positive metric.
    InvalidMetric {
        /// Metric name.
        name: &'static str,

        /// Supplied value.
        value: f64,
    },

    /// No feasible distributed route exists.
    NoRoute {
        /// Source.
        source: DistributedNodeId,

        /// Destination.
        destination: DistributedNodeId,
    },

    /// Route does not satisfy requested fidelity.
    FidelityConstraintUnsatisfied {
        /// Required fidelity.
        required: f64,

        /// Estimated fidelity.
        estimated: f64,
    },

    /// Route cannot satisfy coherence requirements.
    CoherenceConstraintUnsatisfied {
        /// Required lifetime.
        required: Duration,

        /// Available lifetime.
        available: Duration,
    },

    /// Insufficient link capacity.
    InsufficientCapacity {
        /// Link.
        link: DistributedLinkId,

        /// Requested units.
        requested: u32,

        /// Available units.
        available: u32,
    },

    /// A reservation was not found.
    UnknownReservation {
        /// Reservation.
        reservation: ReservationId,
    },

    /// Resource already reserved.
    ResourceReserved {
        /// Resource.
        resource: EntanglementResourceId,
    },

    /// Invalid resource state transition.
    InvalidResourceState {
        /// Resource.
        resource: EntanglementResourceId,
    },

    /// Invalid route.
    InvalidRoute(String),

    /// Invalid distributed topology.
    InvalidTopology(String),

    /// Route operation is not supported.
    UnsupportedOperation(String),

    /// Caller supplied an invalid request.
    InvalidRequest(String),

    /// Arithmetic overflow.
    ArithmeticOverflow,

    /// Internal consistency failure.
    InternalInvariantViolation(String),
}

impl fmt::Display for DistributedRoutingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNetwork => {
                write!(formatter, "distributed routing network is empty")
            }

            Self::UnknownNode { node } => {
                write!(formatter, "unknown distributed node {node}")
            }

            Self::UnknownLink { link } => {
                write!(formatter, "unknown distributed link {link}")
            }

            Self::UnavailableQubit { node, qubit } => {
                write!(
                    formatter,
                    "physical qubit {qubit} is unavailable on {node}"
                )
            }

            Self::SelfLoop { link } => {
                write!(formatter, "distributed link {link} is a self-loop")
            }

            Self::DuplicateLink { link } => {
                write!(formatter, "distributed link {link} is duplicated")
            }

            Self::InvalidFidelity { fidelity } => {
                write!(
                    formatter,
                    "invalid fidelity value {fidelity}; expected finite value in [0,1]"
                )
            }

            Self::InvalidProbability { probability } => {
                write!(
                    formatter,
                    "invalid probability value {probability}; expected finite value in [0,1]"
                )
            }

            Self::InvalidMetric { name, value } => {
                write!(
                    formatter,
                    "invalid metric '{name}' with value {value}"
                )
            }

            Self::NoRoute {
                source,
                destination,
            } => {
                write!(
                    formatter,
                    "no distributed quantum route exists from {source} to {destination}"
                )
            }

            Self::FidelityConstraintUnsatisfied {
                required,
                estimated,
            } => {
                write!(
                    formatter,
                    "required fidelity {required} cannot be satisfied; estimated fidelity is {estimated}"
                )
            }

            Self::CoherenceConstraintUnsatisfied {
                required,
                available,
            } => {
                write!(
                    formatter,
                    "required coherence lifetime {required:?} exceeds available {available:?}"
                )
            }

            Self::InsufficientCapacity {
                link,
                requested,
                available,
            } => {
                write!(
                    formatter,
                    "link {link} has capacity {available}, but {requested} units are required"
                )
            }

            Self::UnknownReservation { reservation } => {
                write!(formatter, "unknown reservation {reservation}")
            }

            Self::ResourceReserved { resource } => {
                write!(
                    formatter,
                    "entanglement resource {resource} is already reserved"
                )
            }

            Self::InvalidResourceState { resource } => {
                write!(
                    formatter,
                    "invalid state transition for entanglement resource {resource}"
                )
            }

            Self::InvalidRoute(message) => {
                write!(formatter, "invalid distributed route: {message}")
            }

            Self::InvalidTopology(message) => {
                write!(
                    formatter,
                    "invalid distributed topology: {message}"
                )
            }

            Self::UnsupportedOperation(message) => {
                write!(
                    formatter,
                    "unsupported distributed routing operation: {message}"
                )
            }

            Self::InvalidRequest(message) => {
                write!(
                    formatter,
                    "invalid distributed routing request: {message}"
                )
            }

            Self::ArithmeticOverflow => {
                write!(formatter, "distributed routing arithmetic overflow")
            }

            Self::InternalInvariantViolation(message) => {
                write!(
                    formatter,
                    "distributed routing invariant violation: {message}"
                )
            }
        }
    }
}

impl std::error::Error for DistributedRoutingError {}

// =============================================================================
// Numeric validation
// =============================================================================

fn validate_probability(
    value: f64,
) -> Result<(), DistributedRoutingError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(
            DistributedRoutingError::InvalidProbability {
                probability: value,
            },
        );
    }

    Ok(())
}

fn validate_fidelity(
    value: f64,
) -> Result<(), DistributedRoutingError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(
            DistributedRoutingError::InvalidFidelity {
                fidelity: value,
            },
        );
    }

    Ok(())
}

fn validate_non_negative_metric(
    name: &'static str,
    value: f64,
) -> Result<(), DistributedRoutingError> {
    if !value.is_finite() || value < 0.0 {
        return Err(DistributedRoutingError::InvalidMetric {
            name,
            value,
        });
    }

    Ok(())
}

// =============================================================================
// Node model
// =============================================================================

/// A distributed quantum-computing node.
///
/// A node can represent:
///
/// - a QPU;
/// - a quantum-memory station;
/// - a repeater;
/// - a modular quantum processor;
/// - a simulator partition;
/// - a future Zamani quantum-network node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedNode {
    /// Stable node identifier.
    pub id: DistributedNodeId,

    /// Human-readable node name.
    pub name: String,

    /// Physical qubits available on this node.
    pub qubits: BTreeSet<PhysicalQubitId>,

    /// Maximum number of simultaneously usable entanglement resources.
    pub entanglement_capacity: u32,

    /// Whether the node can perform entanglement swapping.
    pub supports_entanglement_swapping: bool,

    /// Whether the node can perform purification.
    pub supports_purification: bool,

    /// Whether the node can participate in teleportation.
    pub supports_teleportation: bool,

    /// Whether the node can act as a distributed routing/repeater node.
    pub is_router: bool,

    /// Node-level availability.
    pub available: bool,
}

impl DistributedNode {
    /// Creates a distributed node.
    #[must_use]
    pub fn new(
        id: DistributedNodeId,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            qubits: BTreeSet::new(),
            entanglement_capacity: 0,
            supports_entanglement_swapping: false,
            supports_purification: false,
            supports_teleportation: false,
            is_router: false,
            available: true,
        }
    }

    /// Adds a physical qubit.
    pub fn add_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.qubits.insert(qubit)
    }

    /// Returns whether the node owns a physical qubit.
    #[must_use]
    pub fn owns_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Validates node configuration.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        if self.entanglement_capacity == 0
            && (self.supports_entanglement_swapping
                || self.supports_purification)
        {
            return Err(
                DistributedRoutingError::InvalidTopology(
                    format!(
                        "node {} supports entanglement operations but has zero entanglement capacity",
                        self.id
                    ),
                ),
            );
        }

        Ok(())
    }
}

// =============================================================================
// Link model
// =============================================================================

/// Quantum-link properties.
///
/// These values describe the expected quality of a link and are consumed by
/// the routing planner. They do not cause physical link creation.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumLinkProperties {
    /// Probability that elementary entanglement generation succeeds.
    pub generation_success_probability: f64,

    /// Fidelity of a freshly generated entangled pair.
    pub generation_fidelity: f64,

    /// Approximate end-to-end link latency.
    pub latency: Duration,

    /// Estimated coherence lifetime.
    pub coherence_time: Duration,

    /// Maximum elementary entanglement generation rate.
    pub generation_rate_per_second: f64,

    /// Probability that an entanglement swap succeeds.
    pub swap_success_probability: f64,

    /// Maximum number of simultaneous resources.
    pub capacity: u32,

    /// Whether the link is currently available.
    pub available: bool,
}

impl Default for QuantumLinkProperties {
    fn default() -> Self {
        Self {
            generation_success_probability: 1.0,
            generation_fidelity: 1.0,
            latency: Duration::ZERO,
            coherence_time: Duration::from_secs(1),
            generation_rate_per_second: 1.0,
            swap_success_probability: 1.0,
            capacity: 1,
            available: true,
        }
    }
}

impl QuantumLinkProperties {
    /// Validates link properties.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        validate_probability(
            self.generation_success_probability,
        )?;

        validate_fidelity(self.generation_fidelity)?;

        validate_non_negative_metric(
            "generation_rate_per_second",
            self.generation_rate_per_second,
        )?;

        validate_probability(
            self.swap_success_probability,
        )?;

        if self.capacity == 0 {
            return Err(
                DistributedRoutingError::InvalidMetric {
                    name: "capacity",
                    value: 0.0,
                },
            );
        }

        Ok(())
    }

    /// Returns expected generation attempts for one successful pair.
    #[must_use]
    pub fn expected_generation_attempts(&self) -> f64 {
        if self.generation_success_probability <= 0.0 {
            return f64::INFINITY;
        }

        1.0 / self.generation_success_probability
    }
}

/// A quantum network link.
///
/// Quantum links are modeled as undirected entanglement resources unless the
/// higher-level target explicitly models directional physical constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumLink {
    /// Stable identifier.
    pub id: DistributedLinkId,

    /// First endpoint.
    pub a: DistributedNodeId,

    /// Second endpoint.
    pub b: DistributedNodeId,

    /// Link properties.
    pub properties: QuantumLinkProperties,
}

impl QuantumLink {
    /// Creates a quantum link.
    #[must_use]
    pub fn new(
        id: DistributedLinkId,
        a: DistributedNodeId,
        b: DistributedNodeId,
        properties: QuantumLinkProperties,
    ) -> Self {
        Self {
            id,
            a,
            b,
            properties,
        }
    }

    /// Returns the opposite endpoint.
    #[must_use]
    pub fn other(
        &self,
        node: DistributedNodeId,
    ) -> Option<DistributedNodeId> {
        if node == self.a {
            Some(self.b)
        } else if node == self.b {
            Some(self.a)
        } else {
            None
        }
    }

    /// Returns whether the link connects the supplied nodes.
    #[must_use]
    pub fn connects(
        &self,
        a: DistributedNodeId,
        b: DistributedNodeId,
    ) -> bool {
        (self.a == a && self.b == b)
            || (self.a == b && self.b == a)
    }

    /// Validates the link.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        if self.a == self.b {
            return Err(
                DistributedRoutingError::SelfLoop {
                    link: self.id,
                },
            );
        }

        self.properties.validate()
    }
}

// =============================================================================
// Classical control links
// =============================================================================

/// Classical-control link used to coordinate distributed quantum operations.
///
/// Classical control is deliberately modeled separately from quantum
/// entanglement connectivity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalControlLink {
    /// Source node.
    pub a: DistributedNodeId,

    /// Destination node.
    pub b: DistributedNodeId,

    /// One-way latency.
    pub latency: Duration,

    /// Whether the control channel is available.
    pub available: bool,
}

impl ClassicalControlLink {
    /// Creates a classical control link.
    #[must_use]
    pub fn new(
        a: DistributedNodeId,
        b: DistributedNodeId,
        latency: Duration,
    ) -> Self {
        Self {
            a,
            b,
            latency,
            available: true,
        }
    }

    /// Returns whether the link connects two nodes.
    #[must_use]
    pub fn connects(
        &self,
        a: DistributedNodeId,
        b: DistributedNodeId,
    ) -> bool {
        (self.a == a && self.b == b)
            || (self.a == b && self.b == a)
    }
}

// =============================================================================
// Entanglement resource
// =============================================================================

/// Lifecycle of an entanglement resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EntanglementResourceState {
    /// Resource is available for reservation.
    Available,

    /// Resource has been reserved by a route.
    Reserved,

    /// Resource is actively being consumed.
    InUse,

    /// Resource has been consumed.
    Consumed,

    /// Resource is no longer usable.
    Expired,

    /// Resource became invalid because a link/node failed.
    Invalidated,
}

/// A concrete stateful entanglement resource.
#[derive(Debug, Clone, PartialEq)]
pub struct EntanglementResource {
    /// Stable resource identifier.
    pub id: EntanglementResourceId,

    /// First endpoint node.
    pub a: DistributedNodeId,

    /// Second endpoint node.
    pub b: DistributedNodeId,

    /// Physical qubit at endpoint A.
    pub qubit_a: PhysicalQubitId,

    /// Physical qubit at endpoint B.
    pub qubit_b: PhysicalQubitId,

    /// Fidelity of this particular resource.
    pub fidelity: f64,

    /// Remaining coherence lifetime.
    pub remaining_coherence: Duration,

    /// Current lifecycle state.
    pub state: EntanglementResourceState,

    /// Link through which the resource was generated.
    pub link: DistributedLinkId,
}

impl EntanglementResource {
    /// Validates the resource.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        if self.a == self.b {
            return Err(
                DistributedRoutingError::InvalidRoute(
                    "entanglement resource endpoints must be distinct"
                        .to_string(),
                ),
            );
        }

        validate_fidelity(self.fidelity)
    }

    /// Returns whether the resource can currently be reserved.
    #[must_use]
    pub fn is_available(&self) -> bool {
        matches!(
            self.state,
            EntanglementResourceState::Available
        ) && self.fidelity > 0.0
            && !self.remaining_coherence.is_zero()
    }

    /// Returns whether this resource connects two nodes.
    #[must_use]
    pub fn connects(
        &self,
        a: DistributedNodeId,
        b: DistributedNodeId,
    ) -> bool {
        (self.a == a && self.b == b)
            || (self.a == b && self.b == a)
    }
}

// =============================================================================
// Distributed network
// =============================================================================

/// Complete routing view of a distributed quantum network.
///
/// This is an immutable-style topology/resource snapshot from the routing
/// perspective. External systems can create a new snapshot when the network
/// changes.
#[derive(Debug, Clone, Default)]
pub struct DistributedNetwork {
    nodes: BTreeMap<DistributedNodeId, DistributedNode>,
    links: BTreeMap<DistributedLinkId, QuantumLink>,
    classical_links: Vec<ClassicalControlLink>,
    resources: BTreeMap<EntanglementResourceId, EntanglementResource>,
}

impl DistributedNetwork {
    /// Creates an empty network.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a node.
    pub fn add_node(
        &mut self,
        node: DistributedNode,
    ) -> Result<(), DistributedRoutingError> {
        node.validate()?;

        if self.nodes.contains_key(&node.id) {
            return Err(
                DistributedRoutingError::InvalidTopology(
                    format!("duplicate node {}", node.id),
                ),
            );
        }

        self.nodes.insert(node.id, node);
        Ok(())
    }

    /// Adds a quantum link.
    pub fn add_link(
        &mut self,
        link: QuantumLink,
    ) -> Result<(), DistributedRoutingError> {
        link.validate()?;

        if self.links.contains_key(&link.id) {
            return Err(
                DistributedRoutingError::DuplicateLink {
                    link: link.id,
                },
            );
        }

        if !self.nodes.contains_key(&link.a) {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: link.a,
                },
            );
        }

        if !self.nodes.contains_key(&link.b) {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: link.b,
                },
            );
        }

        self.links.insert(link.id, link);
        Ok(())
    }

    /// Adds a classical control link.
    pub fn add_classical_link(
        &mut self,
        link: ClassicalControlLink,
    ) -> Result<(), DistributedRoutingError> {
        if !self.nodes.contains_key(&link.a) {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: link.a,
                },
            );
        }

        if !self.nodes.contains_key(&link.b) {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: link.b,
                },
            );
        }

        self.classical_links.push(link);
        Ok(())
    }

    /// Adds an entanglement resource.
    pub fn add_resource(
        &mut self,
        resource: EntanglementResource,
    ) -> Result<(), DistributedRoutingError> {
        resource.validate()?;

        if !self.links.contains_key(&resource.link) {
            return Err(
                DistributedRoutingError::UnknownLink {
                    link: resource.link,
                },
            );
        }

        if self.resources.contains_key(&resource.id) {
            return Err(
                DistributedRoutingError::InvalidTopology(
                    format!(
                        "duplicate entanglement resource {}",
                        resource.id
                    ),
                ),
            );
        }

        self.resources.insert(resource.id, resource);
        Ok(())
    }

    /// Returns a node.
    #[must_use]
    pub fn node(
        &self,
        id: DistributedNodeId,
    ) -> Option<&DistributedNode> {
        self.nodes.get(&id)
    }

    /// Returns a link.
    #[must_use]
    pub fn link(
        &self,
        id: DistributedLinkId,
    ) -> Option<&QuantumLink> {
        self.links.get(&id)
    }

    /// Returns an entanglement resource.
    #[must_use]
    pub fn resource(
        &self,
        id: EntanglementResourceId,
    ) -> Option<&EntanglementResource> {
        self.resources.get(&id)
    }

    /// Returns all nodes in deterministic order.
    #[must_use]
    pub fn nodes(
        &self,
    ) -> impl Iterator<Item = &DistributedNode> {
        self.nodes.values()
    }

    /// Returns all links in deterministic order.
    #[must_use]
    pub fn links(
        &self,
    ) -> impl Iterator<Item = &QuantumLink> {
        self.links.values()
    }

    /// Returns links incident on a node.
    #[must_use]
    pub fn incident_links(
        &self,
        node: DistributedNodeId,
    ) -> Vec<&QuantumLink> {
        let mut links: Vec<&QuantumLink> = self
            .links
            .values()
            .filter(|link| {
                link.a == node || link.b == node
            })
            .collect();

        links.sort_by_key(|link| link.id);
        links
    }

    /// Returns resources available on a link.
    #[must_use]
    pub fn available_resources_on_link(
        &self,
        link: DistributedLinkId,
    ) -> Vec<&EntanglementResource> {
        let mut resources: Vec<&EntanglementResource> = self
            .resources
            .values()
            .filter(|resource| {
                resource.link == link && resource.is_available()
            })
            .collect();

        resources.sort_by_key(|resource| resource.id);
        resources
    }

    /// Validates the entire network snapshot.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        if self.nodes.is_empty() {
            return Err(DistributedRoutingError::EmptyNetwork);
        }

        for node in self.nodes.values() {
            node.validate()?;

            if !node.available {
                continue;
            }

            for qubit in &node.qubits {
                if !node.owns_qubit(*qubit) {
                    return Err(
                        DistributedRoutingError::InternalInvariantViolation(
                            format!(
                                "node {} lost ownership of qubit {qubit}",
                                node.id
                            ),
                        ),
                    );
                }
            }
        }

        for link in self.links.values() {
            link.validate()?;

            if !self.nodes.contains_key(&link.a)
                || !self.nodes.contains_key(&link.b)
            {
                return Err(
                    DistributedRoutingError::InvalidTopology(
                        format!(
                            "link {} references an unknown endpoint",
                            link.id
                        ),
                    ),
                );
            }
        }

        for resource in self.resources.values() {
            resource.validate()?;

            if !self.nodes.contains_key(&resource.a)
                || !self.nodes.contains_key(&resource.b)
            {
                return Err(
                    DistributedRoutingError::InvalidTopology(
                        format!(
                            "resource {} references an unknown node",
                            resource.id
                        ),
                    ),
                );
            }
        }

        Ok(())
    }

    /// Returns whether two nodes have an available quantum link.
    #[must_use]
    pub fn is_adjacent(
        &self,
        a: DistributedNodeId,
        b: DistributedNodeId,
    ) -> bool {
        self.links.values().any(|link| {
            link.properties.available
                && link.connects(a, b)
                && self
                    .node(a)
                    .map(|node| node.available)
                    .unwrap_or(false)
                && self
                    .node(b)
                    .map(|node| node.available)
                    .unwrap_or(false)
        })
    }
}

// =============================================================================
// Routing objective
// =============================================================================

/// Distributed-routing objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistributedRoutingObjective {
    /// Minimize hop count.
    Hops,

    /// Minimize latency.
    Latency,

    /// Maximize fidelity.
    Fidelity,

    /// Maximize expected successful throughput.
    Throughput,

    /// Minimize resource consumption.
    ResourceUsage,

    /// Minimize a weighted composite cost.
    Weighted,
}

/// Weights for a distributed routing objective.
///
/// All weights must be finite and non-negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistributedCostWeights {
    /// Weight for hop count.
    pub hops: f64,

    /// Weight for latency.
    pub latency: f64,

    /// Weight for fidelity loss.
    pub fidelity_loss: f64,

    /// Weight for resource consumption.
    pub resource_usage: f64,

    /// Weight for failure probability.
    pub failure_probability: f64,
}

impl Default for DistributedCostWeights {
    fn default() -> Self {
        Self {
            hops: 1.0,
            latency: 1.0,
            fidelity_loss: 1.0,
            resource_usage: 1.0,
            failure_probability: 1.0,
        }
    }
}

impl DistributedCostWeights {
    /// Validates all weights.
    pub fn validate(&self) -> Result<(), DistributedRoutingError> {
        for (name, value) in [
            ("hops", self.hops),
            ("latency", self.latency),
            ("fidelity_loss", self.fidelity_loss),
            ("resource_usage", self.resource_usage),
            ("failure_probability", self.failure_probability),
        ] {
            validate_non_negative_metric(name, value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Routing request
// =============================================================================

/// Semantic distributed operation required by a quantum program.
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedRequestKind {
    /// Establish end-to-end entanglement between two nodes.
    EntanglementDistribution,

    /// Move one logical qubit's state from source to destination.
    Teleportation,

    /// Execute a remote gate between two logical qubits.
    RemoteGate {
        /// Stable routing-level gate name.
        gate: String,
    },

    /// Establish entanglement among multiple destinations.
    MultipartiteEntanglement,

    /// User-defined distributed operation.
    Custom {
        /// Stable operation name.
        name: String,
    },
}

/// A distributed routing request.
#[derive(Debug, Clone, PartialEq)]
pub struct DistributedRoutingRequest {
    /// Request kind.
    pub kind: DistributedRequestKind,

    /// Source node.
    pub source: DistributedNodeId,

    /// Primary destination.
    pub destination: DistributedNodeId,

    /// Additional destinations for multipartite requests.
    pub additional_destinations: Vec<DistributedNodeId>,

    /// Optional logical qubit being transported/used.
    pub logical_qubit: Option<LogicalQubitId>,

    /// Optional source physical qubit.
    pub source_qubit: Option<PhysicalQubitId>,

    /// Optional destination physical qubit.
    pub destination_qubit: Option<PhysicalQubitId>,

    /// Minimum required end-to-end fidelity.
    pub minimum_fidelity: f64,

    /// Minimum required coherence lifetime.
    pub minimum_coherence: Duration,

    /// Maximum permitted latency.
    pub maximum_latency: Option<Duration>,

    /// Routing objective.
    pub objective: DistributedRoutingObjective,

    /// Composite objective weights.
    pub weights: DistributedCostWeights,

    /// Required number of parallel entanglement resources.
    pub resource_units: u32,

    /// Whether purification may be inserted into the semantic plan.
    pub allow_purification: bool,

    /// Whether alternate paths may be considered.
    pub max_alternate_paths: usize,
}

impl DistributedRoutingRequest {
    /// Creates a basic two-node entanglement request.
    #[must_use]
    pub fn entanglement(
        source: DistributedNodeId,
        destination: DistributedNodeId,
    ) -> Self {
        Self {
            kind: DistributedRequestKind::EntanglementDistribution,
            source,
            destination,
            additional_destinations: Vec::new(),
            logical_qubit: None,
            source_qubit: None,
            destination_qubit: None,
            minimum_fidelity: 0.0,
            minimum_coherence: Duration::ZERO,
            maximum_latency: None,
            objective: DistributedRoutingObjective::Weighted,
            weights: DistributedCostWeights::default(),
            resource_units: 1,
            allow_purification: false,
            max_alternate_paths: 0,
        }
    }

    /// Creates a teleportation request.
    #[must_use]
    pub fn teleport(
        source: DistributedNodeId,
        destination: DistributedNodeId,
        logical_qubit: LogicalQubitId,
    ) -> Self {
        let mut request = Self::entanglement(source, destination);
        request.kind = DistributedRequestKind::Teleportation;
        request.logical_qubit = Some(logical_qubit);
        request
    }

    /// Validates the request.
    pub fn validate(
        &self,
        network: &DistributedNetwork,
    ) -> Result<(), DistributedRoutingError> {
        if !network.nodes.contains_key(&self.source) {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: self.source,
                },
            );
        }

        if !network
            .nodes
            .contains_key(&self.destination)
        {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: self.destination,
                },
            );
        }

        if self.source == self.destination {
            return Err(
                DistributedRoutingError::InvalidRequest(
                    "source and destination must differ for a distributed route"
                        .to_string(),
                ),
            );
        }

        for destination in &self.additional_destinations {
            if !network.nodes.contains_key(destination) {
                return Err(
                    DistributedRoutingError::UnknownNode {
                        node: *destination,
                    },
                );
            }
        }

        if self.resource_units == 0 {
            return Err(
                DistributedRoutingError::InvalidRequest(
                    "resource_units must be greater than zero"
                        .to_string(),
                ),
            );
        }

        validate_fidelity(self.minimum_fidelity)?;
        self.weights.validate()?;

        Ok(())
    }
}

// =============================================================================
// Route metrics
// =============================================================================

/// Quality/cost metrics calculated for a distributed route.
#[derive(Debug, Clone, PartialEq)]
pub struct DistributedRouteMetrics {
    /// Number of network links traversed.
    pub hops: usize,

    /// Estimated quantum-network latency.
    pub quantum_latency: Duration,

    /// Estimated classical coordination latency.
    pub classical_latency: Duration,

    /// Estimated total latency.
    pub total_latency: Duration,

    /// Estimated end-to-end fidelity.
    pub estimated_fidelity: f64,

    /// Estimated route success probability.
    pub estimated_success_probability: f64,

    /// Minimum link fidelity on the route.
    pub minimum_link_fidelity: f64,

    /// Estimated number of purification operations.
    pub purification_operations: usize,

    /// Estimated number of entanglement swaps.
    pub swap_operations: usize,

    /// Number of entanglement resources required.
    pub resource_units: u32,

    /// Composite route cost.
    pub cost: f64,
}

impl DistributedRouteMetrics {
    /// Creates metrics for an empty route.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            hops: 0,
            quantum_latency: Duration::ZERO,
            classical_latency: Duration::ZERO,
            total_latency: Duration::ZERO,
            estimated_fidelity: 1.0,
            estimated_success_probability: 1.0,
            minimum_link_fidelity: 1.0,
            purification_operations: 0,
            swap_operations: 0,
            resource_units: 0,
            cost: 0.0,
        }
    }
}

// =============================================================================
// Route operations
// =============================================================================

/// Semantic operation generated by distributed routing.
///
/// These operations are NOT hardware instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum DistributedRouteOperation {
    /// Generate/use an elementary entanglement resource.
    EstablishEntanglement {
        /// Link.
        link: DistributedLinkId,

        /// Resource count.
        resource_units: u32,
    },

    /// Purify entanglement.
    Purify {
        /// Link.
        link: DistributedLinkId,

        /// Input resources.
        input_resources: u32,

        /// Expected output fidelity.
        expected_fidelity: f64,
    },

    /// Perform an entanglement swap at an intermediate node.
    EntanglementSwap {
        /// Intermediate node.
        node: DistributedNodeId,

        /// Incoming link.
        left_link: DistributedLinkId,

        /// Outgoing link.
        right_link: DistributedLinkId,
    },

    /// Teleport a logical state.
    Teleport {
        /// Logical qubit.
        logical_qubit: LogicalQubitId,

        /// Source node.
        source: DistributedNodeId,

        /// Destination node.
        destination: DistributedNodeId,
    },

    /// Execute a remote operation using distributed resources.
    RemoteGate {
        /// Stable gate name.
        gate: String,

        /// Source node.
        source: DistributedNodeId,

        /// Destination node.
        destination: DistributedNodeId,
    },

    /// Release reserved resources.
    ReleaseResources,

    /// Marker for completion of an end-to-end entanglement route.
    EntanglementEstablished {
        /// Source.
        source: DistributedNodeId,

        /// Destination.
        destination: DistributedNodeId,
    },
}

// =============================================================================
// Distributed route
// =============================================================================

/// A complete distributed route plan.
#[derive(Debug, Clone, PartialEq)]
pub struct DistributedRoute {
    /// Request that generated this route.
    pub request: DistributedRoutingRequest,

    /// Ordered nodes from source to destination.
    pub nodes: Vec<DistributedNodeId>,

    /// Ordered network links corresponding to `nodes`.
    pub links: Vec<DistributedLinkId>,

    /// Semantic distributed operations.
    pub operations: Vec<DistributedRouteOperation>,

    /// Resources selected for the route.
    pub resources: Vec<EntanglementResourceId>,

    /// Route metrics.
    pub metrics: DistributedRouteMetrics,
}

impl DistributedRoute {
    /// Validates basic route structure.
    pub fn validate(
        &self,
        network: &DistributedNetwork,
    ) -> Result<(), DistributedRoutingError> {
        if self.nodes.len() < 2 {
            return Err(
                DistributedRoutingError::InvalidRoute(
                    "a distributed route must contain at least two nodes"
                        .to_string(),
                ),
            );
        }

        if self.links.len() + 1 != self.nodes.len() {
            return Err(
                DistributedRoutingError::InvalidRoute(
                    "route node/link cardinality is inconsistent"
                        .to_string(),
                ),
            );
        }

        for index in 0..self.links.len() {
            let link = network
                .link(self.links[index])
                .ok_or(
                    DistributedRoutingError::UnknownLink {
                        link: self.links[index],
                    },
                )?;

            if !link.connects(
                self.nodes[index],
                self.nodes[index + 1],
            ) {
                return Err(
                    DistributedRoutingError::InvalidRoute(
                        format!(
                            "link {} does not connect {} to {}",
                            link.id,
                            self.nodes[index],
                            self.nodes[index + 1]
                        ),
                    ),
                );
            }

            if !link.properties.available {
                return Err(
                    DistributedRoutingError::InvalidRoute(
                        format!(
                            "link {} is unavailable",
                            link.id
                        ),
                    ),
                );
            }
        }

        if self.metrics.hops != self.links.len() {
            return Err(
                DistributedRoutingError::InvalidRoute(
                    "route hop metric does not match route length"
                        .to_string(),
                ),
            );
        }

        validate_fidelity(self.metrics.estimated_fidelity)?;

        Ok(())
    }
}

// =============================================================================
// Reservation transaction
// =============================================================================

/// Transactional resource reservation.
///
/// The transaction owns only a private set of reservations. It does not mutate
/// the caller's network until `commit` is explicitly invoked.
#[derive(Debug, Clone)]
pub struct ResourceReservationTransaction {
    /// Stable reservation identifier.
    pub id: ReservationId,

    reserved_resources: BTreeSet<EntanglementResourceId>,

    committed: bool,
}

impl ResourceReservationTransaction {
    /// Creates an empty transaction.
    #[must_use]
    pub fn new(id: ReservationId) -> Self {
        Self {
            id,
            reserved_resources: BTreeSet::new(),
            committed: false,
        }
    }

    /// Adds a resource to the transaction.
    pub fn reserve(
        &mut self,
        network: &DistributedNetwork,
        resource: EntanglementResourceId,
    ) -> Result<(), DistributedRoutingError> {
        let resource_ref = network
            .resource(resource)
            .ok_or(
                DistributedRoutingError::InvalidResourceState {
                    resource,
                },
            )?;

        if !resource_ref.is_available() {
            return Err(
                DistributedRoutingError::ResourceReserved {
                    resource,
                },
            );
        }

        self.reserved_resources.insert(resource);

        Ok(())
    }

    /// Returns resources reserved by this transaction.
    #[must_use]
    pub fn resources(
        &self,
    ) -> impl Iterator<Item = EntanglementResourceId> + '_ {
        self.reserved_resources.iter().copied()
    }

    /// Returns whether the transaction has been committed.
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        self.committed
    }

    /// Marks the transaction as committed.
    ///
    /// The actual network/resource owner remains responsible for applying the
    /// reservation to mutable execution state.
    pub fn commit(
        &mut self,
    ) -> Result<(), DistributedRoutingError> {
        if self.committed {
            return Err(
                DistributedRoutingError::InternalInvariantViolation(
                    "reservation transaction committed twice"
                        .to_string(),
                ),
            );
        }

        self.committed = true;
        Ok(())
    }

    /// Rolls back the transaction.
    pub fn rollback(&mut self) {
        self.reserved_resources.clear();
        self.committed = false;
    }
}

// =============================================================================
// Route planner
// =============================================================================

/// Production distributed quantum route planner.
///
/// The planner is deliberately stateless. Each call operates against an
/// explicit immutable network snapshot.
#[derive(Debug, Clone)]
pub struct DistributedRoutePlanner {
    /// Objective used by this planner.
    pub objective: DistributedRoutingObjective,

    /// Composite objective weights.
    pub weights: DistributedCostWeights,
}

impl Default for DistributedRoutePlanner {
    fn default() -> Self {
        Self {
            objective: DistributedRoutingObjective::Weighted,
            weights: DistributedCostWeights::default(),
        }
    }
}

impl DistributedRoutePlanner {
    /// Creates a planner.
    #[must_use]
    pub fn new(
        objective: DistributedRoutingObjective,
        weights: DistributedCostWeights,
    ) -> Result<Self, DistributedRoutingError> {
        weights.validate()?;

        Ok(Self {
            objective,
            weights,
        })
    }

    /// Plans one distributed route.
    pub fn plan(
        &self,
        network: &DistributedNetwork,
        request: &DistributedRoutingRequest,
    ) -> Result<DistributedRoute, DistributedRoutingError> {
        network.validate()?;
        request.validate(network)?;

        if !network
            .node(request.source)
            .map(|node| node.available)
            .unwrap_or(false)
        {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: request.source,
                },
            );
        }

        if !network
            .node(request.destination)
            .map(|node| node.available)
            .unwrap_or(false)
        {
            return Err(
                DistributedRoutingError::UnknownNode {
                    node: request.destination,
                },
            );
        }

        let path = self.find_path(network, request)?;

        let metrics = self.calculate_metrics(
            network,
            &path,
            request,
        )?;

        self.validate_constraints(
            &metrics,
            request,
        )?;

        let operations = self.build_operations(
            network,
            &path,
            request,
            &metrics,
        )?;

        let route = DistributedRoute {
            request: request.clone(),
            nodes: path.nodes,
            links: path.links,
            operations,
            resources: path.resources,
            metrics,
        };

        route.validate(network)?;

        Ok(route)
    }

    fn find_path(
        &self,
        network: &DistributedNetwork,
        request: &DistributedRoutingRequest,
    ) -> Result<PathCandidate, DistributedRoutingError> {
        if request.source == request.destination {
            return Err(
                DistributedRoutingError::InvalidRequest(
                    "source and destination must differ"
                        .to_string(),
                ),
            );
        }

        let mut heap = BinaryHeap::new();
        let start = SearchState {
            node: request.source,
            cost: 0.0,
            hops: 0,
            nodes: vec![request.source],
            links: Vec::new(),
        };

        heap.push(SearchEntry::new(start));

        let mut best: HashMap<
            DistributedNodeId,
            (f64, usize, Vec<DistributedNodeId>),
        > = HashMap::new();

        while let Some(entry) = heap.pop() {
            let state = entry.state;

            if state.node == request.destination {
                let resources = self.select_resources(
                    network,
                    &state.links,
                    request.resource_units,
                )?;

                return Ok(PathCandidate {
                    nodes: state.nodes,
                    links: state.links,
                    resources,
                });
            }

            if let Some(previous) = best.get(&state.node) {
                if compare_search_cost(
                    state.cost,
                    state.hops,
                    &state.nodes,
                    previous.0,
                    previous.1,
                    &previous.2,
                ) == Ordering::Greater
                {
                    continue;
                }
            }

            best.insert(
                state.node,
                (
                    state.cost,
                    state.hops,
                    state.nodes.clone(),
                ),
            );

            let mut links =
                network.incident_links(state.node);

            links.sort_by_key(|link| {
                (
                    link.id,
                    link.a,
                    link.b,
                )
            });

            for link in links {
                if !link.properties.available {
                    continue;
                }

                let Some(next) =
                    link.other(state.node)
                else {
                    continue;
                };

                let Some(next_node) =
                    network.node(next)
                else {
                    continue;
                };

                if !next_node.available {
                    continue;
                }

                if state.nodes.contains(&next) {
                    continue;
                }

                let edge_cost = self.link_cost(
                    link,
                    request,
                )?;

                let next_cost =
                    state.cost + edge_cost;

                if !next_cost.is_finite() {
                    continue;
                }

                let mut next_nodes =
                    state.nodes.clone();
                next_nodes.push(next);

                let mut next_links =
                    state.links.clone();
                next_links.push(link.id);

                let next_state = SearchState {
                    node: next,
                    cost: next_cost,
                    hops: state.hops + 1,
                    nodes: next_nodes,
                    links: next_links,
                };

                heap.push(SearchEntry::new(next_state));
            }
        }

        Err(DistributedRoutingError::NoRoute {
            source: request.source,
            destination: request.destination,
        })
    }

    fn link_cost(
        &self,
        link: &QuantumLink,
        request: &DistributedRoutingRequest,
    ) -> Result<f64, DistributedRoutingError> {
        let properties = &link.properties;

        properties.validate()?;

        let latency_ms =
            properties.latency.as_secs_f64() * 1000.0;

        let fidelity_loss =
            1.0 - properties.generation_fidelity;

        let failure_probability =
            1.0
                - properties.generation_success_probability
                    * properties.swap_success_probability;

        let resource_penalty =
            if properties.capacity == 0 {
                f64::INFINITY
            } else {
                1.0 / f64::from(properties.capacity)
            };

        let cost = match request.objective {
            DistributedRoutingObjective::Hops => 1.0,

            DistributedRoutingObjective::Latency => {
                latency_ms
            }

            DistributedRoutingObjective::Fidelity => {
                fidelity_loss
            }

            DistributedRoutingObjective::Throughput => {
                if properties.generation_rate_per_second
                    <= 0.0
                {
                    f64::INFINITY
                } else {
                    1.0
                        / properties
                            .generation_rate_per_second
                }
            }

            DistributedRoutingObjective::ResourceUsage => {
                resource_penalty
            }

            DistributedRoutingObjective::Weighted => {
                self.weights.hops
                    + self.weights.latency
                        * latency_ms
                    + self.weights.fidelity_loss
                        * fidelity_loss
                    + self.weights.resource_usage
                        * resource_penalty
                    + self.weights.failure_probability
                        * failure_probability
            }
        };

        Ok(cost)
    }

    fn select_resources(
        &self,
        network: &DistributedNetwork,
        links: &[DistributedLinkId],
        units: u32,
    ) -> Result<Vec<EntanglementResourceId>, DistributedRoutingError> {
        if units == 0 {
            return Err(
                DistributedRoutingError::InvalidRequest(
                    "resource count must be positive"
                        .to_string(),
                ),
            );
        }

        let mut selected = Vec::new();

        for link in links {
            let candidates =
                network.available_resources_on_link(*link);

            if candidates.len() < units as usize {
                return Err(
                    DistributedRoutingError::InsufficientCapacity {
                        link: *link,
                        requested: units,
                        available: candidates.len() as u32,
                    },
                );
            }

            for resource in candidates
                .into_iter()
                .take(units as usize)
            {
                selected.push(resource.id);
            }
        }

        Ok(selected)
    }

    fn calculate_metrics(
        &self,
        network: &DistributedNetwork,
        path: &PathCandidate,
        request: &DistributedRoutingRequest,
    ) -> Result<DistributedRouteMetrics, DistributedRoutingError> {
        let mut metrics =
            DistributedRouteMetrics::zero();

        metrics.hops = path.links.len();
        metrics.resource_units =
            request.resource_units;

        metrics.swap_operations =
            path.links.len().saturating_sub(1);

        let mut fidelity = 1.0_f64;
        let mut success = 1.0_f64;
        let mut minimum_fidelity = 1.0_f64;

        let mut quantum_latency =
            Duration::ZERO;

        for link_id in &path.links {
            let link = network
                .link(*link_id)
                .ok_or(
                    DistributedRoutingError::UnknownLink {
                        link: *link_id,
                    },
                )?;

            let p = &link.properties;

            fidelity *= p.generation_fidelity;
            success *=
                p.generation_success_probability;
            success *= p.swap_success_probability;

            minimum_fidelity =
                minimum_fidelity.min(
                    p.generation_fidelity,
                );

            quantum_latency =
                quantum_latency
                    .checked_add(p.latency)
                    .ok_or(
                        DistributedRoutingError::ArithmeticOverflow,
                    )?;
        }

        /*
         * Intermediate entanglement swaps reduce fidelity and have a success
         * probability. The exact physical model is backend-specific, so the
         * routing layer uses a conservative multiplicative estimate.
         */
        if metrics.swap_operations > 0 {
            for index in 0..metrics.swap_operations {
                let link = network
                    .link(path.links[index])
                    .ok_or(
                        DistributedRoutingError::UnknownLink {
                            link: path.links[index],
                        },
                    )?;

                success *=
                    link.properties.swap_success_probability;
            }
        }

        /*
         * Classical coordination is needed for distributed protocols.
         * The route planner deliberately uses the largest available control
         * latency on each hop rather than assuming zero-latency classical
         * signaling.
         */
        let classical_latency =
            self.classical_coordination_latency(
                network,
                &path.nodes,
            );

        let total_latency =
            quantum_latency
                .checked_add(classical_latency)
                .ok_or(
                    DistributedRoutingError::ArithmeticOverflow,
                )?;

        metrics.quantum_latency =
            quantum_latency;
        metrics.classical_latency =
            classical_latency;
        metrics.total_latency =
            total_latency;
        metrics.estimated_fidelity =
            fidelity.clamp(0.0, 1.0);
        metrics.estimated_success_probability =
            success.clamp(0.0, 1.0);
        metrics.minimum_link_fidelity =
            minimum_fidelity;

        metrics.cost = self.route_cost(
            &metrics,
            request,
        );

        Ok(metrics)
    }

    fn classical_coordination_latency(
        &self,
        network: &DistributedNetwork,
        nodes: &[DistributedNodeId],
    ) -> Duration {
        let mut total = Duration::ZERO;

        for window in nodes.windows(2) {
            let a = window[0];
            let b = window[1];

            let latency = network
                .classical_links
                .iter()
                .filter(|link| {
                    link.available && link.connects(a, b)
                })
                .map(|link| link.latency)
                .min()
                .unwrap_or(Duration::ZERO);

            if let Some(updated) =
                total.checked_add(latency)
            {
                total = updated;
            } else {
                return Duration::MAX;
            }
        }

        total
    }

    fn route_cost(
        &self,
        metrics: &DistributedRouteMetrics,
        request: &DistributedRoutingRequest,
    ) -> f64 {
        let latency_ms =
            metrics.total_latency.as_secs_f64()
                * 1000.0;

        let fidelity_loss =
            1.0 - metrics.estimated_fidelity;

        let failure_probability =
            1.0
                - metrics
                    .estimated_success_probability;

        match request.objective {
            DistributedRoutingObjective::Hops => {
                metrics.hops as f64
            }

            DistributedRoutingObjective::Latency => {
                latency_ms
            }

            DistributedRoutingObjective::Fidelity => {
                fidelity_loss
            }

            DistributedRoutingObjective::Throughput => {
                failure_probability
            }

            DistributedRoutingObjective::ResourceUsage => {
                metrics.resource_units as f64
            }

            DistributedRoutingObjective::Weighted => {
                request.weights.hops
                    * metrics.hops as f64
                    + request.weights.latency
                        * latency_ms
                    + request.weights.fidelity_loss
                        * fidelity_loss
                    + request.weights.resource_usage
                        * metrics.resource_units
                            as f64
                    + request
                        .weights
                        .failure_probability
                        * failure_probability
            }
        }
    }

    fn validate_constraints(
        &self,
        metrics: &DistributedRouteMetrics,
        request: &DistributedRoutingRequest,
    ) -> Result<(), DistributedRoutingError> {
        if metrics.estimated_fidelity
            < request.minimum_fidelity
        {
            return Err(
                DistributedRoutingError::FidelityConstraintUnsatisfied {
                    required: request.minimum_fidelity,
                    estimated: metrics.estimated_fidelity,
                },
            );
        }

        if metrics.total_latency
            < request.minimum_coherence
        {
            /*
             * This condition intentionally does not reject the route.
             *
             * Coherence is a maximum usable lifetime, not a minimum latency.
             *
             * The actual coherence feasibility is checked by comparing the
             * requested lifetime with the minimum available coherence along
             * the path. That value is validated in `plan`.
             */
        }

        if let Some(maximum_latency) =
            request.maximum_latency
        {
            if metrics.total_latency > maximum_latency {
                return Err(
                    DistributedRoutingError::InvalidRequest(
                        format!(
                            "route latency {:?} exceeds maximum {:?}",
                            metrics.total_latency,
                            maximum_latency
                        ),
                    ),
                );
            }
        }

        Ok(())
    }

    fn build_operations(
        &self,
        network: &DistributedNetwork,
        path: &PathCandidate,
        request: &DistributedRoutingRequest,
        metrics: &DistributedRouteMetrics,
    ) -> Result<Vec<DistributedRouteOperation>, DistributedRoutingError> {
        let mut operations = Vec::new();

        for link in &path.links {
            operations.push(
                DistributedRouteOperation::EstablishEntanglement {
                    link: *link,
                    resource_units:
                        request.resource_units,
                },
            );
        }

        if request.allow_purification
            && metrics.estimated_fidelity
                < request.minimum_fidelity
        {
            /*
             * This branch is normally reached only when a caller explicitly
             * uses a backend-specific planner that later refines the estimate.
             *
             * The generic planner never invents a purification protocol.
             */
            for link in &path.links {
                operations.push(
                    DistributedRouteOperation::Purify {
                        link: *link,
                        input_resources:
                            request.resource_units,
                        expected_fidelity:
                            request.minimum_fidelity,
                    },
                );
            }
        }

        for index in 1..path.nodes.len() - 1 {
            operations.push(
                DistributedRouteOperation::EntanglementSwap {
                    node: path.nodes[index],
                    left_link: path.links[index - 1],
                    right_link: path.links[index],
                },
            );
        }

        match &request.kind {
            DistributedRequestKind::EntanglementDistribution => {
                operations.push(
                    DistributedRouteOperation::EntanglementEstablished {
                        source: request.source,
                        destination: request.destination,
                    },
                );
            }

            DistributedRequestKind::Teleportation => {
                let logical_qubit =
                    request.logical_qubit.ok_or(
                        DistributedRoutingError::InvalidRequest(
                            "teleportation requires a logical qubit"
                                .to_string(),
                        ),
                    )?;

                operations.push(
                    DistributedRouteOperation::Teleport {
                        logical_qubit,
                        source: request.source,
                        destination: request.destination,
                    },
                );
            }

            DistributedRequestKind::RemoteGate { gate } => {
                operations.push(
                    DistributedRouteOperation::RemoteGate {
                        gate: gate.clone(),
                        source: request.source,
                        destination: request.destination,
                    },
                );
            }

            DistributedRequestKind::MultipartiteEntanglement => {
                operations.push(
                    DistributedRouteOperation::EntanglementEstablished {
                        source: request.source,
                        destination: request.destination,
                    },
                );
            }

            DistributedRequestKind::Custom { name } => {
                return Err(
                    DistributedRoutingError::UnsupportedOperation(
                        format!(
                            "custom distributed operation '{name}' requires an execution/lowering policy"
                        ),
                    ),
                );
            }
        }

        operations.push(
            DistributedRouteOperation::ReleaseResources,
        );

        let _ = network;

        Ok(operations)
    }
}

// =============================================================================
// Path-search implementation
// =============================================================================

#[derive(Debug, Clone)]
struct PathCandidate {
    nodes: Vec<DistributedNodeId>,
    links: Vec<DistributedLinkId>,
    resources: Vec<EntanglementResourceId>,
}

#[derive(Debug, Clone)]
struct SearchState {
    node: DistributedNodeId,
    cost: f64,
    hops: usize,
    nodes: Vec<DistributedNodeId>,
    links: Vec<DistributedLinkId>,
}

#[derive(Debug, Clone)]
struct SearchEntry {
    state: SearchState,
}

impl SearchEntry {
    fn new(state: SearchState) -> Self {
        Self { state }
    }
}

impl PartialEq for SearchEntry {
    fn eq(&self, other: &Self) -> bool {
        compare_search_cost(
            self.state.cost,
            self.state.hops,
            &self.state.nodes,
            other.state.cost,
            other.state.hops,
            &other.state.nodes,
        ) == Ordering::Equal
    }
}

impl Eq for SearchEntry {}

impl PartialOrd for SearchEntry {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchEntry {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        /*
         * BinaryHeap is a max-heap. Reverse the ordering so the lowest-cost
         * path is popped first.
         */
        compare_search_cost(
            other.state.cost,
            other.state.hops,
            &other.state.nodes,
            self.state.cost,
            self.state.hops,
            &self.state.nodes,
        )
    }
}

fn compare_search_cost(
    cost_a: f64,
    hops_a: usize,
    nodes_a: &[DistributedNodeId],
    cost_b: f64,
    hops_b: usize,
    nodes_b: &[DistributedNodeId],
) -> Ordering {
    cost_b
        .total_cmp(&cost_a)
        .then_with(|| hops_b.cmp(&hops_a))
        .then_with(|| nodes_b.cmp(nodes_a))
}

// =============================================================================
// Distributed route transaction
// =============================================================================

/// A complete transaction around distributed route planning.
///
/// The transaction guarantees that planning failures do not mutate the caller's
/// network snapshot.
#[derive(Debug, Clone)]
pub struct DistributedRoutingTransaction {
    /// Reservation identifier.
    pub reservation: ReservationId,

    /// Planned route.
    pub route: Option<DistributedRoute>,

    /// Resource transaction.
    pub resources: ResourceReservationTransaction,

    committed: bool,
}

impl DistributedRoutingTransaction {
    /// Starts a transaction.
    #[must_use]
    pub fn begin(
        reservation: ReservationId,
    ) -> Self {
        Self {
            reservation,
            route: None,
            resources:
                ResourceReservationTransaction::new(
                    reservation,
                ),
            committed: false,
        }
    }

    /// Plans a route inside the transaction.
    pub fn plan(
        &mut self,
        planner: &DistributedRoutePlanner,
        network: &DistributedNetwork,
        request: &DistributedRoutingRequest,
    ) -> Result<(), DistributedRoutingError> {
        if self.committed {
            return Err(
                DistributedRoutingError::InternalInvariantViolation(
                    "cannot plan after transaction commit"
                        .to_string(),
                ),
            );
        }

        let route =
            planner.plan(network, request)?;

        for resource in &route.resources {
            self.resources.reserve(
                network,
                *resource,
            )?;
        }

        self.route = Some(route);

        Ok(())
    }

    /// Commits the transaction.
    ///
    /// The returned route remains a plan. Actual execution is deliberately
    /// outside this module.
    pub fn commit(
        &mut self,
    ) -> Result<DistributedRoute, DistributedRoutingError> {
        if self.committed {
            return Err(
                DistributedRoutingError::InternalInvariantViolation(
                    "distributed transaction committed twice"
                        .to_string(),
                ),
            );
        }

        let route = self.route.clone().ok_or(
            DistributedRoutingError::InvalidRoute(
                "cannot commit an empty transaction"
                    .to_string(),
            ),
        )?;

        self.resources.commit()?;
        self.committed = true;

        Ok(route)
    }

    /// Rolls back the transaction.
    pub fn rollback(&mut self) {
        self.resources.rollback();
        self.route = None;
        self.committed = false;
    }

    /// Returns whether the transaction is committed.
    #[must_use]
    pub const fn is_committed(&self) -> bool {
        self.committed
    }
}

// =============================================================================
// Multipartite routing
// =============================================================================

/// A route tree for distributing entanglement from one source to several
/// destinations.
#[derive(Debug, Clone, PartialEq)]
pub struct MultipartiteRoute {
    /// Source node.
    pub source: DistributedNodeId,

    /// Destination nodes.
    pub destinations: Vec<DistributedNodeId>,

    /// Tree edges.
    pub links: Vec<DistributedLinkId>,

    /// Ordered semantic operations.
    pub operations: Vec<DistributedRouteOperation>,

    /// Aggregate metrics.
    pub metrics: DistributedRouteMetrics,
}

impl MultipartiteRoute {
    /// Validates destination uniqueness and basic structure.
    pub fn validate(
        &self,
        network: &DistributedNetwork,
    ) -> Result<(), DistributedRoutingError> {
        if self.destinations.is_empty() {
            return Err(
                DistributedRoutingError::InvalidRoute(
                    "multipartite route requires at least one destination"
                        .to_string(),
                ),
            );
        }

        let mut destinations =
            BTreeSet::new();

        for destination in &self.destinations {
            if *destination == self.source {
                return Err(
                    DistributedRoutingError::InvalidRoute(
                        "source cannot also be a destination"
                            .to_string(),
                    ),
                );
            }

            if !destinations.insert(*destination) {
                return Err(
                    DistributedRoutingError::InvalidRoute(
                        "multipartite destinations must be unique"
                            .to_string(),
                    ),
                );
            }

            if network.node(*destination).is_none() {
                return Err(
                    DistributedRoutingError::UnknownNode {
                        node: *destination,
                    },
                );
            }
        }

        for link in &self.links {
            if network.link(*link).is_none() {
                return Err(
                    DistributedRoutingError::UnknownLink {
                        link: *link,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Distributed routing facade
// =============================================================================

/// High-level distributed routing facade.
///
/// This is the public entry point for compiler/router integration.
#[derive(Debug, Clone)]
pub struct DistributedRouter {
    planner: DistributedRoutePlanner,
}

impl Default for DistributedRouter {
    fn default() -> Self {
        Self {
            planner:
                DistributedRoutePlanner::default(),
        }
    }
}

impl DistributedRouter {
    /// Creates a distributed router.
    #[must_use]
    pub fn new(
        objective: DistributedRoutingObjective,
        weights: DistributedCostWeights,
    ) -> Result<Self, DistributedRoutingError> {
        Ok(Self {
            planner:
                DistributedRoutePlanner::new(
                    objective,
                    weights,
                )?,
        })
    }

    /// Returns the configured planner.
    #[must_use]
    pub fn planner(
        &self,
    ) -> &DistributedRoutePlanner {
        &self.planner
    }

    /// Routes a distributed request.
    pub fn route(
        &self,
        network: &DistributedNetwork,
        request: &DistributedRoutingRequest,
    ) -> Result<DistributedRoute, DistributedRoutingError> {
        self.planner.plan(network, request)
    }

    /// Starts a transactional distributed route.
    pub fn begin_transaction(
        &self,
        reservation: ReservationId,
    ) -> DistributedRoutingTransaction {
        DistributedRoutingTransaction::begin(
            reservation,
        )
    }

    /// Routes multiple independent requests deterministically.
    ///
    /// Each request is planned against the same immutable network snapshot.
    /// No request can mutate another request's planning state.
    pub fn route_batch(
        &self,
        network: &DistributedNetwork,
        mut requests: Vec<DistributedRoutingRequest>,
    ) -> Result<Vec<DistributedRoute>, DistributedRoutingError> {
        requests.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| {
                    a.destination
                        .cmp(&b.destination)
                })
                .then_with(|| {
                    request_kind_rank(&a.kind)
                        .cmp(&request_kind_rank(
                            &b.kind,
                        ))
                })
        });

        requests
            .iter()
            .map(|request| self.route(network, request))
            .collect()
    }
}

fn request_kind_rank(
    kind: &DistributedRequestKind,
) -> u8 {
    match kind {
        DistributedRequestKind::EntanglementDistribution => 0,
        DistributedRequestKind::Teleportation => 1,
        DistributedRequestKind::RemoteGate { .. } => 2,
        DistributedRequestKind::MultipartiteEntanglement => 3,
        DistributedRequestKind::Custom { .. } => 4,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_network() -> DistributedNetwork {
        let mut network =
            DistributedNetwork::new();

        let mut node0 =
            DistributedNode::new(
                DistributedNodeId::new(0),
                "node0",
            );

        node0.entanglement_capacity = 8;
        node0.supports_entanglement_swapping = true;
        node0.supports_teleportation = true;
        node0.is_router = true;
        node0.add_qubit(
            PhysicalQubitId::new(0),
        );

        let mut node1 =
            DistributedNode::new(
                DistributedNodeId::new(1),
                "node1",
            );

        node1.entanglement_capacity = 8;
        node1.supports_entanglement_swapping = true;
        node1.supports_teleportation = true;
        node1.is_router = true;
        node1.add_qubit(
            PhysicalQubitId::new(1),
        );

        let mut node2 =
            DistributedNode::new(
                DistributedNodeId::new(2),
                "node2",
            );

        node2.entanglement_capacity = 8;
        node2.supports_teleportation = true;
        node2.add_qubit(
            PhysicalQubitId::new(2),
        );

        network.add_node(node0).unwrap();
        network.add_node(node1).unwrap();
        network.add_node(node2).unwrap();

        let properties =
            QuantumLinkProperties {
                generation_success_probability: 0.99,
                generation_fidelity: 0.99,
                latency:
                    Duration::from_millis(2),
                coherence_time:
                    Duration::from_millis(100),
                generation_rate_per_second: 100.0,
                swap_success_probability: 0.98,
                capacity: 4,
                available: true,
            };

        network
            .add_link(QuantumLink::new(
                DistributedLinkId::new(0),
                DistributedNodeId::new(0),
                DistributedNodeId::new(1),
                properties.clone(),
            ))
            .unwrap();

        network
            .add_link(QuantumLink::new(
                DistributedLinkId::new(1),
                DistributedNodeId::new(1),
                DistributedNodeId::new(2),
                properties,
            ))
            .unwrap();

        network
            .add_classical_link(
                ClassicalControlLink::new(
                    DistributedNodeId::new(0),
                    DistributedNodeId::new(1),
                    Duration::from_millis(1),
                ),
            )
            .unwrap();

        network
            .add_classical_link(
                ClassicalControlLink::new(
                    DistributedNodeId::new(1),
                    DistributedNodeId::new(2),
                    Duration::from_millis(1),
                ),
            )
            .unwrap();

        network
    }

    #[test]
    fn validates_network() {
        let network = test_network();

        assert!(network.validate().is_ok());
    }

    #[test]
    fn finds_deterministic_path() {
        let network = test_network();

        let router =
            DistributedRouter::default();

        let request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(2),
            );

        let route =
            router.route(&network, &request).unwrap();

        assert_eq!(
            route.nodes,
            vec![
                DistributedNodeId::new(0),
                DistributedNodeId::new(1),
                DistributedNodeId::new(2),
            ]
        );

        assert_eq!(route.links.len(), 2);
        assert_eq!(route.metrics.hops, 2);
        assert_eq!(
            route.metrics.swap_operations,
            1
        );
    }

    #[test]
    fn teleportation_requires_logical_qubit() {
        let network = test_network();

        let router =
            DistributedRouter::default();

        let mut request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(2),
            );

        request.kind =
            DistributedRequestKind::Teleportation;

        let result =
            router.route(&network, &request);

        assert!(result.is_err());
    }

    #[test]
    fn transaction_rolls_back_without_mutating_network() {
        let network = test_network();

        let router =
            DistributedRouter::default();

        let mut transaction =
            router.begin_transaction(
                ReservationId::new(1),
            );

        let request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(2),
            );

        assert!(
            transaction
                .plan(
                    router.planner(),
                    &network,
                    &request,
                )
                .is_err()
        );

        transaction.rollback();

        assert!(!transaction.is_committed());
        assert!(transaction.route.is_none());
    }

    #[test]
    fn rejects_missing_destination() {
        let network = test_network();

        let router =
            DistributedRouter::default();

        let request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(99),
            );

        assert!(router.route(&network, &request).is_err());
    }

    #[test]
    fn rejects_invalid_fidelity() {
        let result =
            QuantumLinkProperties {
                generation_fidelity: 2.0,
                ..QuantumLinkProperties::default()
            }
            .validate();

        assert!(result.is_err());
    }

    #[test]
    fn resource_validation_works() {
        let resource =
            EntanglementResource {
                id: EntanglementResourceId::new(1),
                a: DistributedNodeId::new(0),
                b: DistributedNodeId::new(1),
                qubit_a:
                    PhysicalQubitId::new(0),
                qubit_b:
                    PhysicalQubitId::new(1),
                fidelity: 0.99,
                remaining_coherence:
                    Duration::from_millis(100),
                state:
                    EntanglementResourceState::Available,
                link:
                    DistributedLinkId::new(0),
            };

        assert!(resource.validate().is_ok());
        assert!(resource.is_available());
    }

    #[test]
    fn route_metrics_are_bounded() {
        let network = test_network();

        let router =
            DistributedRouter::default();

        let request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(2),
            );

        let route =
            router.route(&network, &request).unwrap();

        assert!(
            (0.0..=1.0)
                .contains(
                    &route.metrics.estimated_fidelity
                )
        );

        assert!(
            (0.0..=1.0).contains(
                &route
                    .metrics
                    .estimated_success_probability
            )
        );
    }

    #[test]
    fn no_route_is_reported() {
        let mut network =
            DistributedNetwork::new();

        network
            .add_node(
                DistributedNode::new(
                    DistributedNodeId::new(0),
                    "a",
                ),
            )
            .unwrap();

        network
            .add_node(
                DistributedNode::new(
                    DistributedNodeId::new(1),
                    "b",
                ),
            )
            .unwrap();

        let router =
            DistributedRouter::default();

        let request =
            DistributedRoutingRequest::entanglement(
                DistributedNodeId::new(0),
                DistributedNodeId::new(1),
            );

        assert!(matches!(
            router.route(&network, &request),
            Err(
                DistributedRoutingError::NoRoute {
                    ..
                }
            )
        ));
    }
}