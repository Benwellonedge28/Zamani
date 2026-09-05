//! Zamani Quantum Scheduling — Distributed Communication Model.
//!
//! Path:
//!     src/quantum/scheduling/distributed/communication.rs
//!
//! # Purpose
//!
//! This module defines the scheduler-facing semantic representation of
//! communication requirements between distributed execution domains.
//!
//! It answers:
//!
//! > "What communication must occur, between which distributed domains,
//! > using which selected communication resources, and what scheduling
//! > readiness/dependency information does that communication introduce?"
//!
//! # Architectural responsibility
//!
//! This module OWNS:
//!
//! - communication transfer requirements;
//! - communication operation identity;
//! - source/destination node references;
//! - selected link/path references;
//! - quantum/classical communication classification;
//! - entanglement-generation requirements;
//! - teleportation communication requirements;
//! - synchronization communication requirements;
//! - remote-operation communication requirements;
//! - communication resource requirements;
//! - communication duration/latency information;
//! - communication readiness information;
//! - communication dependency descriptions;
//! - communication windows;
//! - communication metadata;
//! - deterministic communication requirement collections;
//! - validation of communication descriptions.
//!
//! This module DOES NOT OWN:
//!
//! - quantum semantics;
//! - logical-to-physical routing;
//! - network topology discovery;
//! - path finding;
//! - hardware discovery;
//! - resource inventory;
//! - resource calendars;
//! - scheduling algorithms;
//! - communication execution;
//! - sockets or network transports;
//! - entanglement generation;
//! - teleportation protocol execution;
//! - QEC decoding;
//! - classical message transmission;
//! - hardware calibration;
//! - vendor APIs.
//!
//! Those responsibilities belong to the corresponding subsystems.
//!
//! # Architectural position
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
//!       +-----------------------------+
//!       |                             |
//!       v                             v
//! distributed semantic IR       selected path/placement
//!                                     |
//!                                     v
//!                         scheduling::distributed::communication
//!                                     |
//!                     +---------------+---------------+
//!                     |               |               |
//!                     v               v               v
//!                  timing         resources       constraints
//!                     |               |               |
//!                     +---------------+---------------+
//!                                     |
//!                                     v
//!                                  planner
//!                                     |
//!                                     v
//!                                verification
//!                                     |
//!                                     v
//!                              hardware/runtime
//! ```
//!
//! # Critical separation
//!
//! `routing` answers:
//!
//! > WHERE should communication occur?
//!
//! This module answers:
//!
//! > WHAT communication requirement must the scheduler account for?
//!
//! `scheduling::constraints::communication` answers:
//!
//! > WHEN is a candidate communication schedule legal?
//!
//! `distributed::link` describes:
//!
//! > WHAT scheduler-visible link capabilities exist?
//!
//! `distributed::network` describes:
//!
//! > WHAT distributed topology exists?
//!
//! Keeping these boundaries separate prevents this file from becoming a
//! second routing engine, network implementation, or scheduling algorithm.
//!
//! # Canonical identity ownership
//!
//! This module MUST NOT define replacement identities for:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - `ResourceId`;
//! - `NodeId`;
//! - `LinkId`;
//! - `ClassicalChannelId`;
//! - `EntanglementResourceId`;
//! - `DistributedOperationId`.
//!
//! Canonical quantum identities come from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Canonical distributed identities come from:
//!
//! ```text
//! crate::quantum::ir::model::distributed
//! ```
//!
//! Canonical scheduler resource/time identities come from:
//!
//! ```text
//! crate::quantum::ir::core::identity
//! crate::quantum::scheduling::types
//! ```
//!
//! # Qubit identity
//!
//! When communication explicitly refers to qubits, this module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! It never creates scheduler-specific qubit identifiers.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level.
//!
//! The same program may be scheduled against:
//!
//! - one quantum processor;
//! - multiple chips;
//! - modular quantum computers;
//! - distributed QPUs;
//! - quantum data centers;
//! - quantum networks;
//! - heterogeneous quantum/classical systems;
//! - future communication architectures.
//!
//! No machine-size assumption is encoded here.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_NODES
//! MAX_LINKS
//! MAX_HOPS
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_MESSAGES
//! ```
//!
//! "Infinity" means that this module introduces no artificial finite
//! architectural ceiling. A concrete compilation remains bounded by the
//! actual target, compiler address space, execution policy and available
//! resources.
//!
//! # Sparse representation
//!
//! Communication paths are represented as ordered sparse collections of link
//! identities. No dense adjacency matrix or fixed-size network representation
//! is required.
//!
//! # Timing
//!
//! Timing uses the scheduler's abstract:
//!
//! ```text
//! TimePoint
//! Duration
//! ```
//!
//! No physical unit is assumed.
//!
//! A duration may represent nanoseconds, device ticks, logical time, calibrated
//! time or another target-defined coordinate only after the timing/hardware
//! adapter establishes its interpretation.
//!
//! # Half-open intervals
//!
//! Communication windows use:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Thus:
//!
//! ```text
//! [0, 10)
//! [10, 20)
//! ```
//!
//! do not overlap.
//!
//! # Determinism
//!
//! Ordered collections are used wherever collection order is semantically or
//! diagnostically observable.
//!
//! No scheduling decision depends on hash-map iteration order.
//!
//! # Arithmetic safety
//!
//! Potentially overflowing duration arithmetic is checked.
//!
//! Wrapping arithmetic is never used for scheduling semantics.
//!
//! # Dynamic scheduling
//!
//! The model supports both static and dynamic scheduling.
//!
//! Static scheduling can use:
//!
//! ```text
//! earliest_start
//! duration
//! latency
//! dependencies
//! resource requirements
//! ```
//!
//! Dynamic scheduling can update readiness as:
//!
//! ```text
//! communication begins
//!        |
//!        v
//! transmission completes
//!        |
//!        v
//! classical result becomes available
//!        |
//!        v
//! remote operation becomes schedulable
//! ```
//!
//! This file does not execute those events; it represents them.
//!
//! # Distributed scaling
//!
//! A communication requirement can contain:
//!
//! - zero or more selected links;
//! - arbitrary source/destination nodes;
//! - arbitrary resource requirements;
//! - arbitrary dependencies;
//! - optional timing information;
//! - optional classical/quantum payload semantics.
//!
//! There is no assumption that a communication path has one hop.
//!
//! # Thread safety
//!
//! All types are ordinary owned values with no global mutable state and no
//! interior mutability.
//!
//! # Rust compatibility
//!
//! Supported:
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
//! quantum::ir::core::identity
//! quantum::scheduling::types
//! routing
//! ```
//!
//! Downstream:
//!
//! ```text
//! distributed::link
//! distributed::network
//! constraints::communication
//! resources
//! timing
//! planners
//! dynamic scheduling
//! verification
//! adapters::routing
//! adapters::hardware
//! runtime
//! ```
//!
//! This module deliberately does not import those downstream implementation
//! modules. They consume this stable contract.
//!
//! Therefore adding or changing a planner, routing algorithm, hardware adapter,
//! network implementation or runtime must not require modifying this file just
//! to make the dependency graph compile.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is enforced below.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};
use crate::quantum::ir::model::distributed::{
    ClassicalChannelId,
    DistributedOperationId,
    EntanglementResourceId,
    LinkId,
    NodeId,
};
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::scheduling::types::{Duration, TimePoint};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating a distributed
/// communication requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommunicationError {
    /// A source and destination node are identical when a remote communication
    /// operation requires distinct domains.
    SameEndpoint {
        /// The node used as both source and destination.
        node: NodeId,
    },

    /// A required identifier was duplicated.
    DuplicateIdentifier {
        /// Human-readable category of the duplicated item.
        category: &'static str,
    },

    /// A resource requirement requested zero capacity.
    ZeroResourceQuantity {
        /// Resource involved.
        resource: ResourceId,
    },

    /// A duration was required but was not supplied.
    MissingDuration,

    /// The supplied end time precedes the start time.
    InvalidWindow {
        /// Window start.
        start: TimePoint,

        /// Window end.
        end: TimePoint,
    },

    /// Duration arithmetic overflowed the scheduler's representable range.
    ArithmeticOverflow,

    /// A path was required but no path was supplied.
    EmptyPath,

    /// A path contains a repeated link.
    RepeatedLink {
        /// Repeated link identity.
        link: LinkId,
    },

    /// A path contains a repeated node in a context where that is invalid.
    RepeatedNode {
        /// Repeated node identity.
        node: NodeId,
    },

    /// A path does not connect the requested endpoints.
    PathEndpointMismatch {
        /// Expected source.
        expected_source: NodeId,

        /// Actual path source.
        actual_source: NodeId,

        /// Expected destination.
        expected_destination: NodeId,

        /// Actual path destination.
        actual_destination: NodeId,
    },

    /// A path contains a link whose endpoint relationship is unavailable to
    /// this semantic layer.
    UnvalidatedPath,

    /// A communication operation references an invalid operation identity.
    InvalidOperationIdentity,

    /// A logical qubit is duplicated in one communication requirement.
    DuplicateLogicalQubit {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// A physical qubit is duplicated in one communication requirement.
    DuplicatePhysicalQubit {
        /// Duplicated physical qubit.
        qubit: PhysicalQubitId,
    },
}

impl fmt::Display for CommunicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameEndpoint { node } => {
                write!(formatter, "communication source and destination are the same node: {node}")
            }

            Self::DuplicateIdentifier { category } => {
                write!(formatter, "duplicate communication identifier: {category}")
            }

            Self::ZeroResourceQuantity { resource } => {
                write!(formatter, "communication resource {resource} has zero quantity")
            }

            Self::MissingDuration => {
                formatter.write_str("communication duration is not available")
            }

            Self::InvalidWindow { start, end } => {
                write!(
                    formatter,
                    "invalid communication window: start {start} is after end {end}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str("communication timing arithmetic overflowed")
            }

            Self::EmptyPath => {
                formatter.write_str("communication path is empty")
            }

            Self::RepeatedLink { link } => {
                write!(formatter, "communication path contains repeated link {link}")
            }

            Self::RepeatedNode { node } => {
                write!(formatter, "communication path contains repeated node {node}")
            }

            Self::PathEndpointMismatch {
                expected_source,
                actual_source,
                expected_destination,
                actual_destination,
            } => {
                write!(
                    formatter,
                    "communication path endpoints do not match: expected {expected_source}->{expected_destination}, got {actual_source}->{actual_destination}"
                )
            }

            Self::UnvalidatedPath => {
                formatter.write_str(
                    "communication path connectivity has not been validated by a topology-aware component",
                )
            }

            Self::InvalidOperationIdentity => {
                formatter.write_str("invalid distributed communication operation identity")
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(formatter, "logical qubit appears more than once: {qubit:?}")
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(formatter, "physical qubit appears more than once: {qubit:?}")
            }
        }
    }
}

impl std::error::Error for CommunicationError {}

/// Result alias for this module.
pub type CommunicationResult<T> = Result<T, CommunicationError>;

// =============================================================================
// Communication kind
// =============================================================================

/// Semantic category of distributed communication.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationKind {
    /// Quantum state/information transfer.
    Quantum,

    /// Classical information transfer.
    Classical,

    /// Entanglement generation or consumption.
    Entanglement,

    /// Teleportation-related communication.
    Teleportation,

    /// Distributed synchronization.
    Synchronization,

    /// Communication required by a remote quantum operation.
    RemoteOperation,

    /// Target- or program-defined communication semantic.
    Custom(String),
}

impl CommunicationKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quantum => "quantum",
            Self::Classical => "classical",
            Self::Entanglement => "entanglement",
            Self::Teleportation => "teleportation",
            Self::Synchronization => "synchronization",
            Self::RemoteOperation => "remote-operation",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether the communication carries quantum information.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        matches!(
            self,
            Self::Quantum | Self::Entanglement | Self::Teleportation
        )
    }

    /// Returns whether the communication carries classical information.
    #[must_use]
    pub const fn is_classical(&self) -> bool {
        matches!(self, Self::Classical | Self::Synchronization)
    }
}

impl fmt::Display for CommunicationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(value) => write!(formatter, "custom:{value}"),
            _ => formatter.write_str(self.as_str()),
        }
    }
}

// =============================================================================
// Resource requirement
// =============================================================================

/// One scheduler resource required by a communication operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommunicationResourceRequirement {
    resource: ResourceId,
    quantity: u128,
}

impl CommunicationResourceRequirement {
    /// Creates a resource requirement.
    pub const fn new(
        resource: ResourceId,
        quantity: u128,
    ) -> CommunicationResult<Self> {
        if quantity == 0 {
            return Err(CommunicationError::ZeroResourceQuantity { resource });
        }

        Ok(Self { resource, quantity })
    }

    /// Returns the canonical scheduler resource identity.
    #[must_use]
    pub const fn resource(self) -> ResourceId {
        self.resource
    }

    /// Returns the required resource quantity.
    #[must_use]
    pub const fn quantity(self) -> u128 {
        self.quantity
    }
}

// =============================================================================
// Qubit reference
// =============================================================================

/// Qubit explicitly associated with a distributed communication requirement.
///
/// Logical and physical identities remain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationQubit {
    /// Semantic logical qubit.
    Logical(QubitId),

    /// Physical qubit introduced by a downstream mapping.
    Physical(PhysicalQubitId),
}

impl CommunicationQubit {
    /// Returns the logical qubit when this is a logical reference.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        match self {
            Self::Logical(value) => Some(value),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical qubit when this is a physical reference.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(value) => Some(value),
        }
    }
}

// =============================================================================
// Communication path
// =============================================================================

/// Sparse selected communication path.
///
/// This is a result/requirement supplied by routing or distributed planning.
/// It does not perform path finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationPath {
    source: NodeId,
    destination: NodeId,
    links: Vec<LinkId>,
}

impl CommunicationPath {
    /// Creates a communication path.
    ///
    /// At least one link is required for a remote path.
    pub fn new(
        source: NodeId,
        destination: NodeId,
        links: Vec<LinkId>,
    ) -> CommunicationResult<Self> {
        if source == destination {
            return Err(CommunicationError::SameEndpoint { node: source });
        }

        if links.is_empty() {
            return Err(CommunicationError::EmptyPath);
        }

        let mut seen = BTreeSet::new();

        for link in &links {
            if !seen.insert(*link) {
                return Err(CommunicationError::RepeatedLink { link: *link });
            }
        }

        Ok(Self {
            source,
            destination,
            links,
        })
    }

    /// Returns the path source.
    #[must_use]
    pub const fn source(&self) -> NodeId {
        self.source
    }

    /// Returns the path destination.
    #[must_use]
    pub const fn destination(&self) -> NodeId {
        self.destination
    }

    /// Returns the selected link sequence.
    #[must_use]
    pub fn links(&self) -> &[LinkId] {
        &self.links
    }

    /// Returns the number of links in the path.
    #[must_use]
    pub fn hop_count(&self) -> usize {
        self.links.len()
    }

    /// Returns whether the path contains a particular link.
    #[must_use]
    pub fn contains_link(&self, link: LinkId) -> bool {
        self.links.contains(&link)
    }
}

// =============================================================================
// Communication window
// =============================================================================

/// Legal scheduling window for communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommunicationWindow {
    earliest_start: TimePoint,
    latest_start: Option<TimePoint>,
}

impl CommunicationWindow {
    /// Creates a window with an earliest start and optional latest start.
    pub fn new(
        earliest_start: TimePoint,
        latest_start: Option<TimePoint>,
    ) -> CommunicationResult<Self> {
        if let Some(latest) = latest_start {
            if latest < earliest_start {
                return Err(CommunicationError::InvalidWindow {
                    start: earliest_start,
                    end: latest,
                });
            }
        }

        Ok(Self {
            earliest_start,
            latest_start,
        })
    }

    /// Creates an unconstrained-start window beginning at the supplied time.
    #[must_use]
    pub const fn from(earliest_start: TimePoint) -> Self {
        Self {
            earliest_start,
            latest_start: None,
        }
    }

    /// Returns the earliest legal start.
    #[must_use]
    pub const fn earliest_start(self) -> TimePoint {
        self.earliest_start
    }

    /// Returns the latest legal start, if any.
    #[must_use]
    pub const fn latest_start(self) -> Option<TimePoint> {
        self.latest_start
    }

    /// Tests whether a start time is legal.
    #[must_use]
    pub const fn contains(self, start: TimePoint) -> bool {
        if start < self.earliest_start {
            return false;
        }

        match self.latest_start {
            Some(latest) => start <= latest,
            None => true,
        }
    }
}

// =============================================================================
// Timing
// =============================================================================

/// Timing information associated with communication.
///
/// Duration and latency remain separate because communication may have both
/// resource occupation time and propagation/coordination delay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommunicationTiming {
    duration: Option<Duration>,
    latency: Option<Duration>,
}

impl CommunicationTiming {
    /// Creates unresolved communication timing.
    #[must_use]
    pub const fn unresolved() -> Self {
        Self {
            duration: None,
            latency: None,
        }
    }

    /// Creates timing with an explicit duration.
    #[must_use]
    pub const fn duration(duration: Duration) -> Self {
        Self {
            duration: Some(duration),
            latency: None,
        }
    }

    /// Creates timing with duration and latency.
    #[must_use]
    pub const fn with_latency(
        duration: Duration,
        latency: Duration,
    ) -> Self {
        Self {
            duration: Some(duration),
            latency: Some(latency),
        }
    }

    /// Returns communication execution/occupation duration.
    #[must_use]
    pub const fn duration(self) -> Option<Duration> {
        self.duration
    }

    /// Returns propagation/coordination latency.
    #[must_use]
    pub const fn latency(self) -> Option<Duration> {
        self.latency
    }

    /// Returns the total duration represented by this timing model.
    ///
    /// The operation duration and latency are added with checked arithmetic.
    pub const fn checked_total(self) -> CommunicationResult<Duration> {
        match (self.duration, self.latency) {
            (Some(duration), Some(latency)) => {
                match duration.checked_add(latency) {
                    Some(total) => Ok(total),
                    None => Err(CommunicationError::ArithmeticOverflow),
                }
            }

            (Some(duration), None) => Ok(duration),

            (None, Some(latency)) => Ok(latency),

            (None, None) => Err(CommunicationError::MissingDuration),
        }
    }
}

impl Default for CommunicationTiming {
    fn default() -> Self {
        Self::unresolved()
    }
}

// =============================================================================
// Dependency
// =============================================================================

/// A prerequisite for communication readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationDependency {
    /// An ordinary quantum IR operation must complete.
    Operation(OperationId),

    /// A distributed operation must complete.
    DistributedOperation(DistributedOperationId),

    /// A classical channel event must complete.
    ClassicalChannel(ClassicalChannelId),

    /// An entanglement resource must become available.
    Entanglement(EntanglementResourceId),

    /// Another communication operation must complete.
    Communication(DistributedOperationId),
}

// =============================================================================
// Readiness
// =============================================================================

/// Runtime/static readiness state of communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationReadiness {
    /// All known prerequisites have completed.
    Ready,

    /// The communication is waiting on a dependency.
    Waiting,

    /// The communication cannot currently be scheduled.
    Blocked,

    /// The readiness cannot yet be determined.
    Unknown,
}

impl Default for CommunicationReadiness {
    fn default() -> Self {
        Self::Unknown
    }
}

// =============================================================================
// Payload
// =============================================================================

/// Payload semantics for a communication requirement.
///
/// This intentionally does not represent actual message bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationPayload {
    /// A number of logical qubit references.
    LogicalQubits(BTreeSet<QubitId>),

    /// A number of explicitly mapped physical qubits.
    PhysicalQubits(BTreeSet<PhysicalQubitId>),

    /// Classical information measured in abstract units.
    ClassicalUnits(u128),

    /// Entanglement resource units.
    EntanglementUnits(u128),

    /// No payload is represented; the communication is synchronization-only.
    None,

    /// Target-defined payload semantics.
    Custom(String),
}

impl CommunicationPayload {
    /// Returns the number of logical qubits when applicable.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        match self {
            Self::LogicalQubits(qubits) => qubits.len(),
            _ => 0,
        }
    }

    /// Returns the number of physical qubits when applicable.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        match self {
            Self::PhysicalQubits(qubits) => qubits.len(),
            _ => 0,
        }
    }
}

// =============================================================================
// Communication requirement
// =============================================================================

/// Complete scheduler-facing distributed communication requirement.
///
/// This is the central type exported by this module.
///
/// It contains declarative information only. Constructing one does not send a
/// message, generate entanglement, execute teleportation, reserve a hardware
/// resource, or perform network communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationRequirement {
    operation: DistributedOperationId,
    source_operation: Option<OperationId>,

    source: NodeId,
    destination: NodeId,

    kind: CommunicationKind,

    path: Option<CommunicationPath>,

    payload: CommunicationPayload,

    resources: BTreeMap<ResourceId, u128>,

    dependencies: BTreeSet<CommunicationDependency>,

    timing: CommunicationTiming,

    window: Option<CommunicationWindow>,

    readiness: CommunicationReadiness,

    metadata: BTreeMap<String, String>,
}

impl CommunicationRequirement {
    /// Creates a new communication requirement.
    ///
    /// No hardware or network operation occurs.
    pub fn new(
        operation: DistributedOperationId,
        source: NodeId,
        destination: NodeId,
        kind: CommunicationKind,
    ) -> CommunicationResult<Self> {
        if source == destination {
            return Err(CommunicationError::SameEndpoint { node: source });
        }

        Ok(Self {
            operation,
            source_operation: None,
            source,
            destination,
            kind,
            path: None,
            payload: CommunicationPayload::None,
            resources: BTreeMap::new(),
            dependencies: BTreeSet::new(),
            timing: CommunicationTiming::unresolved(),
            window: None,
            readiness: CommunicationReadiness::Unknown,
            metadata: BTreeMap::new(),
        })
    }

    /// Associates the communication with the source quantum IR operation.
    #[must_use]
    pub const fn with_source_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.source_operation = Some(operation);
        self
    }

    /// Associates an already-selected communication path.
    ///
    /// Path selection remains the responsibility of routing/distributed
    /// planning.
    pub fn with_path(
        mut self,
        path: CommunicationPath,
    ) -> CommunicationResult<Self> {
        if path.source() != self.source || path.destination() != self.destination {
            return Err(CommunicationError::PathEndpointMismatch {
                expected_source: self.source,
                actual_source: path.source(),
                expected_destination: self.destination,
                actual_destination: path.destination(),
            });
        }

        self.path = Some(path);
        Ok(self)
    }

    /// Sets the communication payload.
    #[must_use]
    pub fn with_payload(
        mut self,
        payload: CommunicationPayload,
    ) -> Self {
        self.payload = payload;
        self
    }

    /// Adds one scheduler resource requirement.
    pub fn with_resource(
        mut self,
        requirement: CommunicationResourceRequirement,
    ) -> CommunicationResult<Self> {
        let resource = requirement.resource();
        let quantity = requirement.quantity();

        match self.resources.get(&resource).copied() {
            Some(existing) => {
                let combined = existing
                    .checked_add(quantity)
                    .ok_or(CommunicationError::ArithmeticOverflow)?;

                self.resources.insert(resource, combined);
            }

            None => {
                self.resources.insert(resource, quantity);
            }
        }

        Ok(self)
    }

    /// Adds a communication dependency.
    #[must_use]
    pub fn with_dependency(
        mut self,
        dependency: CommunicationDependency,
    ) -> Self {
        self.dependencies.insert(dependency);
        self
    }

    /// Adds multiple communication dependencies.
    pub fn with_dependencies<I>(
        mut self,
        dependencies: I,
    ) -> Self
    where
        I: IntoIterator<Item = CommunicationDependency>,
    {
        self.dependencies.extend(dependencies);
        self
    }

    /// Sets communication timing.
    #[must_use]
    pub const fn with_timing(
        mut self,
        timing: CommunicationTiming,
    ) -> Self {
        self.timing = timing;
        self
    }

    /// Sets a legal communication start window.
    pub fn with_window(
        mut self,
        window: CommunicationWindow,
    ) -> CommunicationResult<Self> {
        self.window = Some(window);
        Ok(self)
    }

    /// Sets readiness state.
    #[must_use]
    pub const fn with_readiness(
        mut self,
        readiness: CommunicationReadiness,
    ) -> Self {
        self.readiness = readiness;
        self
    }

    /// Adds deterministic metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns the distributed operation identity.
    #[must_use]
    pub const fn operation(&self) -> DistributedOperationId {
        self.operation
    }

    /// Returns the source quantum operation, if one exists.
    #[must_use]
    pub const fn source_operation(&self) -> Option<OperationId> {
        self.source_operation
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

    /// Returns communication kind.
    #[must_use]
    pub fn kind(&self) -> &CommunicationKind {
        &self.kind
    }

    /// Returns the selected path, if routing has supplied one.
    #[must_use]
    pub fn path(&self) -> Option<&CommunicationPath> {
        self.path.as_ref()
    }

    /// Returns the communication payload.
    #[must_use]
    pub fn payload(&self) -> &CommunicationPayload {
        &self.payload
    }

    /// Returns communication resource requirements.
    #[must_use]
    pub fn resources(&self) -> &BTreeMap<ResourceId, u128> {
        &self.resources
    }

    /// Returns communication dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<CommunicationDependency> {
        &self.dependencies
    }

    /// Returns communication timing.
    #[must_use]
    pub const fn timing(&self) -> CommunicationTiming {
        self.timing
    }

    /// Returns the optional legal scheduling window.
    #[must_use]
    pub const fn window(&self) -> Option<CommunicationWindow> {
        self.window
    }

    /// Returns communication readiness.
    #[must_use]
    pub const fn readiness(&self) -> CommunicationReadiness {
        self.readiness
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns the number of selected communication hops.
    #[must_use]
    pub fn hop_count(&self) -> usize {
        self.path.as_ref().map_or(0, CommunicationPath::hop_count)
    }

    /// Returns whether a selected path is available.
    #[must_use]
    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }

    /// Returns whether all statically known scheduling information is ready.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.readiness, CommunicationReadiness::Ready)
    }

    /// Returns the earliest legal start time.
    ///
    /// When no communication window is supplied, the scheduler may determine
    /// readiness from its dependency/resource context.
    #[must_use]
    pub const fn earliest_start(&self) -> Option<TimePoint> {
        self.window.map(CommunicationWindow::earliest_start)
    }

    /// Calculates the communication completion time for a supplied start.
    ///
    /// This does not reserve anything.
    pub fn checked_finish(
        &self,
        start: TimePoint,
    ) -> CommunicationResult<TimePoint> {
        if let Some(window) = self.window {
            if !window.contains(start) {
                return Err(CommunicationError::InvalidWindow {
                    start,
                    end: window.latest_start.unwrap_or(start),
                });
            }
        }

        let duration = self.timing.checked_total()?;

        start
            .checked_add(duration)
            .ok_or(CommunicationError::ArithmeticOverflow)
    }

    /// Validates the complete declarative communication requirement.
    pub fn validate(&self) -> CommunicationResult<()> {
        if self.source == self.destination {
            return Err(CommunicationError::SameEndpoint {
                node: self.source,
            });
        }

        if let Some(path) = &self.path {
            if path.source() != self.source
                || path.destination() != self.destination
            {
                return Err(CommunicationError::PathEndpointMismatch {
                    expected_source: self.source,
                    actual_source: path.source(),
                    expected_destination: self.destination,
                    actual_destination: path.destination(),
                });
            }
        }

        for (resource, quantity) in &self.resources {
            if *quantity == 0 {
                return Err(CommunicationError::ZeroResourceQuantity {
                    resource: *resource,
                });
            }
        }

        if let Some(window) = self.window {
            if let Some(latest) = window.latest_start {
                if latest < window.earliest_start {
                    return Err(CommunicationError::InvalidWindow {
                        start: window.earliest_start,
                        end: latest,
                    });
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Communication set
// =============================================================================

/// Deterministic collection of distributed communication requirements.
///
/// This type exists so planners can consume communication requirements without
/// assuming a fixed number of nodes, links or operations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommunicationSet {
    requirements: BTreeMap<DistributedOperationId, CommunicationRequirement>,
}

impl CommunicationSet {
    /// Creates an empty communication set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a communication requirement.
    ///
    /// Duplicate distributed operation identities are rejected.
    pub fn insert(
        &mut self,
        requirement: CommunicationRequirement,
    ) -> CommunicationResult<()> {
        let operation = requirement.operation();

        if self.requirements.contains_key(&operation) {
            return Err(CommunicationError::DuplicateIdentifier {
                category: "distributed communication operation",
            });
        }

        requirement.validate()?;

        self.requirements.insert(operation, requirement);

        Ok(())
    }

    /// Returns one communication requirement.
    #[must_use]
    pub fn get(
        &self,
        operation: DistributedOperationId,
    ) -> Option<&CommunicationRequirement> {
        self.requirements.get(&operation)
    }

    /// Returns the number of communication requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether no communication requirements exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns deterministic communication requirements.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&DistributedOperationId, &CommunicationRequirement)> {
        self.requirements.iter()
    }

    /// Returns whether the set contains an operation.
    #[must_use]
    pub fn contains(
        &self,
        operation: DistributedOperationId,
    ) -> bool {
        self.requirements.contains_key(&operation)
    }

    /// Returns all communication operation identities in deterministic order.
    #[must_use]
    pub fn operation_ids(&self) -> impl Iterator<Item = DistributedOperationId> + '_ {
        self.requirements.keys().copied()
    }

    /// Validates every communication requirement.
    pub fn validate(&self) -> CommunicationResult<()> {
        for requirement in self.requirements.values() {
            requirement.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_local_remote_communication() {
        let node = NodeId::new(1);

        let result = CommunicationRequirement::new(
            DistributedOperationId::new(1),
            node,
            node,
            CommunicationKind::Quantum,
        );

        assert!(matches!(
            result,
            Err(CommunicationError::SameEndpoint { .. })
        ));
    }

    #[test]
    fn rejects_empty_path() {
        let result = CommunicationPath::new(
            NodeId::new(1),
            NodeId::new(2),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(CommunicationError::EmptyPath)
        ));
    }

    #[test]
    fn rejects_duplicate_path_link() {
        let link = LinkId::new(7);

        let result = CommunicationPath::new(
            NodeId::new(1),
            NodeId::new(2),
            vec![link, link],
        );

        assert!(matches!(
            result,
            Err(CommunicationError::RepeatedLink { .. })
        ));
    }

    #[test]
    fn resource_quantities_accumulate_checked() {
        let resource = ResourceId::new(4);

        let requirement = CommunicationRequirement::new(
            DistributedOperationId::new(1),
            NodeId::new(1),
            NodeId::new(2),
            CommunicationKind::Quantum,
        )
        .expect("valid communication requirement")
        .with_resource(
            CommunicationResourceRequirement::new(resource, 2)
                .expect("valid resource requirement"),
        )
        .expect("resource insertion")
        .with_resource(
            CommunicationResourceRequirement::new(resource, 3)
                .expect("valid resource requirement"),
        )
        .expect("resource insertion");

        assert_eq!(requirement.resources().get(&resource), Some(&5));
    }

    #[test]
    fn communication_window_is_half_open_only_for_resource_intervals() {
        let window = CommunicationWindow::new(
            TimePoint::new(10),
            Some(TimePoint::new(20)),
        )
        .expect("valid window");

        assert!(window.contains(TimePoint::new(10)));
        assert!(window.contains(TimePoint::new(20)));
        assert!(!window.contains(TimePoint::new(9)));
    }

    #[test]
    fn timing_uses_checked_arithmetic() {
        let timing = CommunicationTiming::with_latency(
            Duration::new(10),
            Duration::new(5),
        );

        assert_eq!(
            timing.checked_total().expect("valid timing"),
            Duration::new(15)
        );
    }

    #[test]
    fn communication_finish_is_checked() {
        let requirement = CommunicationRequirement::new(
            DistributedOperationId::new(1),
            NodeId::new(1),
            NodeId::new(2),
            CommunicationKind::Quantum,
        )
        .expect("valid requirement")
        .with_timing(CommunicationTiming::with_latency(
            Duration::new(10),
            Duration::new(5),
        ));

        let finish = requirement
            .checked_finish(TimePoint::new(100))
            .expect("valid finish");

        assert_eq!(finish, TimePoint::new(115));
    }

    #[test]
    fn communication_set_is_deterministic() {
        let mut set = CommunicationSet::new();

        let first = CommunicationRequirement::new(
            DistributedOperationId::new(2),
            NodeId::new(1),
            NodeId::new(3),
            CommunicationKind::Classical,
        )
        .expect("valid requirement");

        let second = CommunicationRequirement::new(
            DistributedOperationId::new(1),
            NodeId::new(1),
            NodeId::new(2),
            CommunicationKind::Quantum,
        )
        .expect("valid requirement");

        set.insert(first).expect("insert");
        set.insert(second).expect("insert");

        let ids: Vec<_> = set.operation_ids().collect();

        assert_eq!(
            ids,
            vec![
                DistributedOperationId::new(1),
                DistributedOperationId::new(2)
            ]
        );
    }

    #[test]
    fn logical_and_physical_qubits_remain_distinct() {
        let logical = CommunicationQubit::Logical(QubitId::new(1));
        let physical = CommunicationQubit::Physical(PhysicalQubitId::new(7));

        assert_eq!(logical.logical(), Some(QubitId::new(1)));
        assert_eq!(logical.physical(), None);

        assert_eq!(physical.logical(), None);
        assert_eq!(physical.physical(), Some(PhysicalQubitId::new(7)));
    }
}