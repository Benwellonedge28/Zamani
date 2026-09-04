//! Zamani Quantum Scheduling — Resource Model
//!
//! This module defines the foundational resource vocabulary used by the
//! scheduling subsystem.
//!
//! # Architectural responsibility
//!
//! This file answers:
//!
//! > "What execution resource may an operation require, consume, share, or
//! > reserve during scheduling?"
//!
//! The resource model is deliberately more general than "qubits". A quantum
//! operation may require:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - measurement/readout channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - microwave sources;
//! - optical paths;
//! - communication links;
//! - classical processing resources;
//! - memory;
//! - ancillas;
//! - synchronization resources;
//! - future hardware resource classes.
//!
//! The scheduler must not assume that a quantum machine consists only of
//! qubits.
//!
//! # Canonical identity ownership
//!
//! This module MUST NOT define replacement qubit or IR resource identities.
//!
//! Logical and physical qubit identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Resource and operation identities are owned by:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! The scheduling subsystem therefore consumes those canonical types.
//!
//! # Universal-program principle
//!
//! Nothing in this file assumes:
//!
//! - a fixed number of qubits;
//! - a fixed number of channels;
//! - a fixed topology;
//! - a fixed gate arity;
//! - a fixed machine size;
//! - a fixed resource count;
//! - a fixed resource capacity;
//! - a particular vendor;
//! - a particular quantum technology;
//! - a particular clock;
//! - a particular physical unit.
//!
//! A concrete target supplies the actual resource inventory.
//!
//! Therefore the same Zamani program can be scheduled against targets with
//! different resource populations and capacities without changing this
//! resource model.
//!
//! "Infinity" in the Zamani architecture means that the scheduler introduces
//! no artificial finite machine-size ceiling. A concrete compilation remains
//! bounded by the actual target, compiler process, address space, and available
//! resources.
//!
//! # Separation of concerns
//!
//! This file defines resource *semantics*.
//!
//! It does NOT define:
//!
//! - reservation calendars;
//! - scheduling algorithms;
//! - dependency graphs;
//! - timing arithmetic;
//! - hardware discovery;
//! - routing;
//! - calibration;
//! - execution;
//! - QEC algorithms;
//! - serialization formats;
//! - vendor APIs.
//!
//! Those responsibilities belong to other scheduling or quantum subsystems.
//!
//! # Resource identity versus capacity
//!
//! `ResourceId` identifies one resource.
//!
//! Capacity describes how much simultaneous usage the resource permits.
//!
//! These concepts MUST NOT be conflated.
//!
//! For example:
//!
//! ```text
//! ResourceId(42)
//! capacity = 4
//! ```
//!
//! does not mean there are four resources. It means one resource exposes four
//! units of schedulable capacity.
//!
//! # Resource modes
//!
//! A resource may be:
//!
//! - exclusive;
//! - shareable;
//! - capacity-limited;
//! - consumable;
//! - reusable;
//! - hierarchical;
//! - dynamically available.
//!
//! This module represents these properties declaratively.
//!
//! # Timing independence
//!
//! Resources contain no physical timing constants.
//!
//! Timing is supplied by the scheduling timing subsystem and target adapter.
//!
//! # Thread safety
//!
//! All types in this module are ordinary owned values with no global mutable
//! state and no interior mutability.
//!
//! They are therefore suitable for transfer between scheduler components and
//! concurrent analysis when the containing data structures permit it.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!              │
//!              │ canonical qubit identities
//!              ▼
//! quantum::ir::core::identity
//!              │
//!              │ canonical ResourceId
//!              ▼
//! scheduling::resources::resource
//!              │
//!      ┌───────┼────────┐
//!      ▼       ▼        ▼
//!    pool  reservation calendar
//!      │       │        │
//!      └───────┼────────┘
//!              ▼
//!       scheduling planners
//!              │
//!              ▼
//!        verification
//! ```
//!
//! `resource.rs` is intentionally foundational. Adding a new scheduler
//! algorithm, planner, reservation implementation, hardware adapter, routing
//! adapter, QEC integration, or runtime integration should not require
//! modifying this file merely because that component was introduced.
//!
//! # Design invariant
//!
//! A scheduler resource is a semantic capability, not a hardware address.
//!
//! Hardware adapters translate target-specific resources into this generic
//! model.
//!
//! ```text
//! hardware target
//!       │
//!       ▼
//! target resource description
//!       │
//!       ▼
//! Resource
//!       │
//!       ▼
//! generic scheduler
//! ```
//!
//! This keeps the scheduler independent from superconducting, trapped-ion,
//! neutral-atom, photonic, spin, annealing, hybrid, distributed, and future
//! quantum technologies.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::core::identity::ResourceId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Resource kind
// =============================================================================

/// Broad semantic category of a schedulable resource.
///
/// This enum intentionally describes *what a resource represents* rather than
/// which vendor provides it.
///
/// The enum is non-exhaustive so future Zamani versions can introduce
/// additional built-in categories without making downstream code assume that
/// today's list is complete.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceKind {
    /// A logical quantum bit resource.
    LogicalQubit,

    /// A physical quantum bit resource.
    PhysicalQubit,

    /// A generic quantum data/ancilla resource.
    QuantumMemory,

    /// A control/drive resource.
    ControlChannel,

    /// A measurement/readout resource.
    MeasurementChannel,

    /// A readout resonator or equivalent measurement resource.
    ReadoutResonator,

    /// A tunable or fixed coupling resource.
    Coupler,

    /// A laser or optical control resource.
    Laser,

    /// A microwave source.
    MicrowaveSource,

    /// An optical source/path resource.
    OpticalChannel,

    /// A communication link between quantum resources.
    CommunicationLink,

    /// A classical processor/resource required by a quantum operation.
    ClassicalProcessor,

    /// Classical memory used during execution.
    ClassicalMemory,

    /// Quantum ancilla resource.
    Ancilla,

    /// Synchronization resource.
    Synchronization,

    /// Generic accelerator resource.
    Accelerator,

    /// Generic compute resource.
    Compute,

    /// Generic memory resource.
    Memory,

    /// A composite resource grouping other resources.
    Composite,

    /// A target-specific resource class not known to the generic scheduler.
    Custom,
}

impl ResourceKind {
    /// Returns a stable human-readable name for the resource category.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubit => "logical-qubit",
            Self::PhysicalQubit => "physical-qubit",
            Self::QuantumMemory => "quantum-memory",
            Self::ControlChannel => "control-channel",
            Self::MeasurementChannel => "measurement-channel",
            Self::ReadoutResonator => "readout-resonator",
            Self::Coupler => "coupler",
            Self::Laser => "laser",
            Self::MicrowaveSource => "microwave-source",
            Self::OpticalChannel => "optical-channel",
            Self::CommunicationLink => "communication-link",
            Self::ClassicalProcessor => "classical-processor",
            Self::ClassicalMemory => "classical-memory",
            Self::Ancilla => "ancilla",
            Self::Synchronization => "synchronization",
            Self::Accelerator => "accelerator",
            Self::Compute => "compute",
            Self::Memory => "memory",
            Self::Composite => "composite",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the resource category represents a qubit directly.
    #[must_use]
    pub const fn is_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit | Self::PhysicalQubit)
    }

    /// Returns whether the resource category represents a communication
    /// resource.
    #[must_use]
    pub const fn is_communication(self) -> bool {
        matches!(self, Self::CommunicationLink)
    }

    /// Returns whether the resource category represents a classical resource.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::ClassicalProcessor
                | Self::ClassicalMemory
                | Self::Compute
                | Self::Memory
        )
    }
}

impl Default for ResourceKind {
    fn default() -> Self {
        Self::Custom
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Semantic scope of a resource.
///
/// Scope describes where the resource belongs in a potentially hierarchical
/// execution system.
///
/// The scheduler does not assume that a target consists of one flat device.
///
/// A resource can therefore be associated with:
///
/// - one local device;
/// - one module;
/// - one node;
/// - one cluster;
/// - one network;
/// - a global execution domain.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceScope {
    /// Resource local to one quantum processing unit.
    Device,

    /// Resource local to one module/chiplet.
    Module,

    /// Resource local to one node in a distributed system.
    Node,

    /// Resource shared by a cluster.
    Cluster,

    /// Resource shared across an execution network.
    Network,

    /// Resource whose scope is the complete execution target.
    Global,

    /// Scope is supplied by a target-specific adapter.
    Custom,
}

impl ResourceScope {
    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Module => "module",
            Self::Node => "node",
            Self::Cluster => "cluster",
            Self::Network => "network",
            Self::Global => "global",
            Self::Custom => "custom",
        }
    }
}

impl Default for ResourceScope {
    fn default() -> Self {
        Self::Device
    }
}

impl fmt::Display for ResourceScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource capacity
// =============================================================================

/// Capacity of a schedulable resource.
///
/// Capacity is deliberately not represented as a fixed machine-specific
/// constant.
///
/// Two fundamental states are supported:
///
/// - finite capacity;
/// - unlimited capacity.
///
/// `Unlimited` is useful for resources whose availability is not represented by
/// a fixed scalar, such as a logically shareable service or a scheduler-level
/// abstract resource.
///
/// The scheduler remains responsible for deciding whether an "unlimited"
/// resource is actually executable on a concrete target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceCapacity {
    /// Finite capacity expressed in abstract resource units.
    Finite(u128),

    /// No scalar capacity limit is imposed by this resource model.
    Unlimited,
}

impl ResourceCapacity {
    /// Creates a finite capacity.
    #[must_use]
    pub const fn finite(value: u128) -> Self {
        Self::Finite(value)
    }

    /// Returns an unlimited capacity.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self::Unlimited
    }

    /// Returns whether the capacity is finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        matches!(self, Self::Finite(_))
    }

    /// Returns whether the capacity is unlimited.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited)
    }

    /// Returns the finite value, if applicable.
    #[must_use]
    pub const fn finite_value(self) -> Option<u128> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unlimited => None,
        }
    }

    /// Checks whether this capacity can satisfy the requested amount.
    ///
    /// Unlimited capacity satisfies every non-negative request.
    #[must_use]
    pub const fn can_satisfy(self, requested: u128) -> bool {
        match self {
            Self::Finite(capacity) => requested <= capacity,
            Self::Unlimited => true,
        }
    }

    /// Checked capacity addition.
    ///
    /// Adding to an unlimited capacity remains unlimited.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Self::Unlimited, _) | (_, Self::Unlimited) => Some(Self::Unlimited),
            (Self::Finite(left), Self::Finite(right)) => {
                match left.checked_add(right) {
                    Some(value) => Some(Self::Finite(value)),
                    None => None,
                }
            }
        }
    }

    /// Returns whether this capacity is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Finite(0))
    }
}

impl Default for ResourceCapacity {
    fn default() -> Self {
        Self::Finite(1)
    }
}

impl fmt::Display for ResourceCapacity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finite(value) => write!(formatter, "{value}"),
            Self::Unlimited => formatter.write_str("unlimited"),
        }
    }
}

// =============================================================================
// Resource quantity
// =============================================================================

/// Quantity of resource capacity requested or consumed.
///
/// This is deliberately a separate type from `ResourceCapacity` because:
///
/// - capacity describes what exists;
/// - quantity describes what an operation requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ResourceQuantity(u128);

impl ResourceQuantity {
    /// One resource unit.
    pub const ONE: Self = Self(1);

    /// Zero resource units.
    pub const ZERO: Self = Self(0);

    /// Creates a resource quantity.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the numeric quantity.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Returns whether this quantity is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether this quantity is one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1
    }

    /// Checked addition.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u128> for ResourceQuantity {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<ResourceQuantity> for u128 {
    fn from(value: ResourceQuantity) -> Self {
        value.value()
    }
}

impl fmt::Display for ResourceQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Resource sharing mode
// =============================================================================

/// Defines how a resource may be used concurrently.
///
/// This is a semantic property of the resource itself.
///
/// Operation-specific quantities are represented separately by
/// `ResourceRequirement`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceSharing {
    /// Only one compatible use may occupy the resource at a time.
    Exclusive,

    /// Multiple operations may use the resource subject to its capacity.
    Shared,

    /// Usage consumes capacity for the lifetime of the execution request.
    Consumable,

    /// Resource may be reused after an operation releases it.
    Reusable,

    /// Resource has nested or delegated capacity.
    Hierarchical,
}

impl ResourceSharing {
    /// Returns whether simultaneous use is potentially permitted.
    #[must_use]
    pub const fn permits_concurrency(self) -> bool {
        matches!(self, Self::Shared | Self::Hierarchical)
    }

    /// Returns whether the resource is reusable after release.
    #[must_use]
    pub const fn is_reusable(self) -> bool {
        matches!(self, Self::Exclusive | Self::Shared | Self::Reusable)
    }

    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::Consumable => "consumable",
            Self::Reusable => "reusable",
            Self::Hierarchical => "hierarchical",
        }
    }
}

impl Default for ResourceSharing {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for ResourceSharing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource affinity
// =============================================================================

/// Identifies a quantum object to which a resource is attached.
///
/// This is deliberately backed by the canonical quantum IR identities.
///
/// The scheduling resource model does not invent a second `QubitId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceAffinity {
    /// Resource belongs to a logical qubit.
    LogicalQubit(QubitId),

    /// Resource belongs to a physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Resource has no qubit-specific affinity.
    None,
}

impl ResourceAffinity {
    /// Creates logical-qubit affinity.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates physical-qubit affinity.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates an unbound affinity.
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Returns the logical qubit when this is logical affinity.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(qubit),
            Self::PhysicalQubit(_) | Self::None => None,
        }
    }

    /// Returns the physical qubit when this is physical affinity.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(qubit) => Some(qubit),
            Self::LogicalQubit(_) | Self::None => None,
        }
    }

    /// Returns whether the resource is attached to a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether the resource is attached to a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns whether the resource has no qubit affinity.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

impl Default for ResourceAffinity {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// Resource lifecycle
// =============================================================================

/// Availability/lifecycle state of a resource.
///
/// This state is descriptive. The scheduler's availability/calendar subsystem
/// determines whether a resource is usable at a particular time.
///
/// A resource can therefore be globally `Available` while temporarily blocked
/// by a reservation or calibration window.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceState {
    /// Resource may be used subject to scheduling constraints.
    Available,

    /// Resource is currently reserved or occupied.
    Busy,

    /// Resource is intentionally unavailable.
    Disabled,

    /// Resource is undergoing calibration or maintenance.
    Maintenance,

    /// Resource is available but has degraded capability.
    Degraded,

    /// Resource state cannot currently be established.
    Unknown,
}

impl ResourceState {
    /// Returns whether the state represents nominal availability.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns whether the resource is unusable without additional information.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(
            self,
            Self::Busy | Self::Disabled | Self::Maintenance | Self::Unknown
        )
    }

    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Busy => "busy",
            Self::Disabled => "disabled",
            Self::Maintenance => "maintenance",
            Self::Degraded => "degraded",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for ResourceState {
    fn default() -> Self {
        Self::Available
    }
}

impl fmt::Display for ResourceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource ownership
// =============================================================================

/// Semantic ownership domain of a resource.
///
/// Ownership is not authentication or authorization. It tells the scheduler
/// which execution domain owns the resource so that distributed scheduling can
/// distinguish local from remote resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceOwnership {
    /// Resource belongs to the local scheduling domain.
    Local,

    /// Resource belongs to another execution domain.
    Remote,

    /// Resource is globally shared.
    Shared,

    /// Ownership is target-defined.
    External,
}

impl Default for ResourceOwnership {
    fn default() -> Self {
        Self::Local
    }
}

// =============================================================================
// Resource identifier reference
// =============================================================================

/// Lightweight resource identity wrapper used by resource requirements.
///
/// The underlying identity remains the canonical IR `ResourceId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRef {
    id: ResourceId,
}

impl ResourceRef {
    /// Creates a resource reference from the canonical IR resource identity.
    #[must_use]
    pub const fn new(id: ResourceId) -> Self {
        Self { id }
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn id(self) -> ResourceId {
        self.id
    }
}

impl From<ResourceId> for ResourceRef {
    fn from(id: ResourceId) -> Self {
        Self::new(id)
    }
}

impl From<ResourceRef> for ResourceId {
    fn from(resource: ResourceRef) -> Self {
        resource.id()
    }
}

impl fmt::Display for ResourceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id.fmt(formatter)
    }
}

// =============================================================================
// Resource requirement mode
// =============================================================================

/// Defines how an operation uses a resource.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceUse {
    /// Resource must be occupied exclusively for the operation interval.
    Exclusive,

    /// Resource may be shared subject to capacity.
    Shared,

    /// Resource capacity is consumed and not returned by normal release.
    Consume,

    /// Resource is required only as a dependency/availability condition.
    Observe,
}

impl ResourceUse {
    /// Returns whether this use occupies schedulable capacity.
    #[must_use]
    pub const fn consumes_capacity(self) -> bool {
        matches!(self, Self::Exclusive | Self::Shared | Self::Consume)
    }

    /// Returns a stable textual name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Shared => "shared",
            Self::Consume => "consume",
            Self::Observe => "observe",
        }
    }
}

impl Default for ResourceUse {
    fn default() -> Self {
        Self::Exclusive
    }
}

impl fmt::Display for ResourceUse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource requirement
// =============================================================================

/// Requirement for one resource during scheduling.
///
/// A requirement does not contain a time interval. Timing belongs to the
/// scheduling result/reservation layer.
///
/// This distinction allows the same requirement to be evaluated by different
/// scheduling algorithms.
///
/// # Example
///
/// A two-qubit operation may require:
///
/// ```text
/// physical-qubit-7   exclusive × 1
/// physical-qubit-12  exclusive × 1
/// control-channel-3  shared    × 1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRequirement {
    resource: ResourceRef,
    quantity: ResourceQuantity,
    use_mode: ResourceUse,
}

impl ResourceRequirement {
    /// Creates an exclusive one-unit requirement.
    #[must_use]
    pub const fn exclusive(resource: ResourceRef) -> Self {
        Self {
            resource,
            quantity: ResourceQuantity::ONE,
            use_mode: ResourceUse::Exclusive,
        }
    }

    /// Creates an exclusive requirement with an explicit quantity.
    #[must_use]
    pub const fn exclusive_units(
        resource: ResourceRef,
        quantity: ResourceQuantity,
    ) -> Self {
        Self {
            resource,
            quantity,
            use_mode: ResourceUse::Exclusive,
        }
    }

    /// Creates a shared requirement.
    #[must_use]
    pub const fn shared(
        resource: ResourceRef,
        quantity: ResourceQuantity,
    ) -> Self {
        Self {
            resource,
            quantity,
            use_mode: ResourceUse::Shared,
        }
    }

    /// Creates a consumable requirement.
    #[must_use]
    pub const fn consumable(
        resource: ResourceRef,
        quantity: ResourceQuantity,
    ) -> Self {
        Self {
            resource,
            quantity,
            use_mode: ResourceUse::Consume,
        }
    }

    /// Creates an observation-only requirement.
    #[must_use]
    pub const fn observe(resource: ResourceRef) -> Self {
        Self {
            resource,
            quantity: ResourceQuantity::ZERO,
            use_mode: ResourceUse::Observe,
        }
    }

    /// Returns the referenced resource.
    #[must_use]
    pub const fn resource(self) -> ResourceRef {
        self.resource
    }

    /// Returns the required quantity.
    #[must_use]
    pub const fn quantity(self) -> ResourceQuantity {
        self.quantity
    }

    /// Returns the usage mode.
    #[must_use]
    pub const fn use_mode(self) -> ResourceUse {
        self.use_mode
    }

    /// Returns whether this requirement consumes capacity.
    #[must_use]
    pub const fn consumes_capacity(self) -> bool {
        self.use_mode.consumes_capacity()
    }

    /// Returns whether the requirement is observation-only.
    #[must_use]
    pub const fn is_observation(self) -> bool {
        matches!(self.use_mode, ResourceUse::Observe)
    }
}

// =============================================================================
// Resource descriptor
// =============================================================================

/// Immutable semantic description of one schedulable resource.
///
/// `Resource` is intentionally independent from reservation state. A resource
/// says what exists; the availability/calendar layer says when it can be used.
///
/// # Invariants
///
/// A valid resource must satisfy:
///
/// - its canonical `ResourceId` identifies the resource;
/// - its kind describes the semantic category;
/// - its capacity is explicit;
/// - its sharing mode is explicit;
/// - its initial state is explicit;
/// - its scope is explicit;
/// - no machine-size assumption is embedded.
///
/// Zero capacity is valid. It represents a known resource that currently
/// cannot satisfy any positive requirement.
///
/// Unlimited capacity is valid and does not imply that a concrete hardware
/// target can execute arbitrary work. Hardware adapters must only expose
/// unlimited capacity when that semantic model is actually correct.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    id: ResourceId,
    kind: ResourceKind,
    capacity: ResourceCapacity,
    sharing: ResourceSharing,
    state: ResourceState,
    scope: ResourceScope,
    affinity: ResourceAffinity,
    ownership: ResourceOwnership,
}

impl Resource {
    /// Creates a resource with explicit semantic properties.
    #[must_use]
    pub const fn new(
        id: ResourceId,
        kind: ResourceKind,
        capacity: ResourceCapacity,
        sharing: ResourceSharing,
        state: ResourceState,
        scope: ResourceScope,
        affinity: ResourceAffinity,
        ownership: ResourceOwnership,
    ) -> Self {
        Self {
            id,
            kind,
            capacity,
            sharing,
            state,
            scope,
            affinity,
            ownership,
        }
    }

    /// Creates a simple exclusive resource with one unit of capacity.
    ///
    /// This convenience constructor contains no hardware-specific assumption.
    /// It is equivalent to describing a resource that has one schedulable unit.
    #[must_use]
    pub const fn unit(id: ResourceId, kind: ResourceKind) -> Self {
        Self::new(
            id,
            kind,
            ResourceCapacity::Finite(1),
            ResourceSharing::Exclusive,
            ResourceState::Available,
            ResourceScope::Device,
            ResourceAffinity::None,
            ResourceOwnership::Local,
        )
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn id(&self) -> ResourceId {
        self.id
    }

    /// Returns the resource category.
    #[must_use]
    pub const fn kind(&self) -> ResourceKind {
        self.kind
    }

    /// Returns the resource capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceCapacity {
        self.capacity
    }

    /// Returns the resource sharing mode.
    #[must_use]
    pub const fn sharing(&self) -> ResourceSharing {
        self.sharing
    }

    /// Returns the current coarse resource state.
    #[must_use]
    pub const fn state(&self) -> ResourceState {
        self.state
    }

    /// Returns the resource scope.
    #[must_use]
    pub const fn scope(&self) -> ResourceScope {
        self.scope
    }

    /// Returns the resource affinity.
    #[must_use]
    pub const fn affinity(&self) -> ResourceAffinity {
        self.affinity
    }

    /// Returns the ownership domain.
    #[must_use]
    pub const fn ownership(&self) -> ResourceOwnership {
        self.ownership
    }

    /// Returns whether the resource is currently nominally available.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Returns whether the resource has finite capacity.
    #[must_use]
    pub const fn has_finite_capacity(&self) -> bool {
        self.capacity.is_finite()
    }

    /// Returns whether the resource has unlimited capacity.
    #[must_use]
    pub const fn has_unlimited_capacity(&self) -> bool {
        self.capacity.is_unlimited()
    }

    /// Checks whether a requirement can fit within the resource's declared
    /// capacity.
    ///
    /// This method checks only static resource semantics. It does not check
    /// temporal reservations, calendars, calibration windows, or conflicts.
    #[must_use]
    pub const fn can_satisfy(&self, requirement: ResourceRequirement) -> bool {
        if matches!(requirement.use_mode(), ResourceUse::Observe) {
            return true;
        }

        self.capacity.can_satisfy(requirement.quantity().value())
    }

    /// Returns whether this resource can be used concurrently according to its
    /// static sharing semantics.
    #[must_use]
    pub const fn permits_concurrent_use(&self) -> bool {
        self.sharing.permits_concurrency()
    }

    /// Returns whether this resource is bound to a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit_resource(&self) -> bool {
        matches!(self.kind, ResourceKind::LogicalQubit)
            && self.affinity.is_logical_qubit()
    }

    /// Returns whether this resource is bound to a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit_resource(&self) -> bool {
        matches!(self.kind, ResourceKind::PhysicalQubit)
            && self.affinity.is_physical_qubit()
    }
}

// =============================================================================
// Resource builder
// =============================================================================

/// Builder for constructing a `Resource`.
///
/// The builder is intentionally allocation-free and contains no global state.
///
/// It allows target adapters to construct resources declaratively without
/// requiring the scheduler core to know how the target was discovered.
#[derive(Debug, Clone, Copy)]
pub struct ResourceBuilder {
    id: ResourceId,
    kind: ResourceKind,
    capacity: ResourceCapacity,
    sharing: ResourceSharing,
    state: ResourceState,
    scope: ResourceScope,
    affinity: ResourceAffinity,
    ownership: ResourceOwnership,
}

impl ResourceBuilder {
    /// Creates a builder for a resource with default scheduling properties.
    #[must_use]
    pub const fn new(id: ResourceId, kind: ResourceKind) -> Self {
        Self {
            id,
            kind,
            capacity: ResourceCapacity::Finite(1),
            sharing: ResourceSharing::Exclusive,
            state: ResourceState::Available,
            scope: ResourceScope::Device,
            affinity: ResourceAffinity::None,
            ownership: ResourceOwnership::Local,
        }
    }

    /// Sets the capacity.
    #[must_use]
    pub const fn capacity(mut self, capacity: ResourceCapacity) -> Self {
        self.capacity = capacity;
        self
    }

    /// Sets finite capacity.
    #[must_use]
    pub const fn finite_capacity(mut self, capacity: u128) -> Self {
        self.capacity = ResourceCapacity::Finite(capacity);
        self
    }

    /// Sets unlimited capacity.
    #[must_use]
    pub const fn unlimited_capacity(mut self) -> Self {
        self.capacity = ResourceCapacity::Unlimited;
        self
    }

    /// Sets sharing behavior.
    #[must_use]
    pub const fn sharing(mut self, sharing: ResourceSharing) -> Self {
        self.sharing = sharing;
        self
    }

    /// Sets resource state.
    #[must_use]
    pub const fn state(mut self, state: ResourceState) -> Self {
        self.state = state;
        self
    }

    /// Sets resource scope.
    #[must_use]
    pub const fn scope(mut self, scope: ResourceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets logical-qubit affinity.
    #[must_use]
    pub const fn logical_qubit(mut self, qubit: QubitId) -> Self {
        self.affinity = ResourceAffinity::LogicalQubit(qubit);
        self
    }

    /// Sets physical-qubit affinity.
    #[must_use]
    pub const fn physical_qubit(mut self, qubit: PhysicalQubitId) -> Self {
        self.affinity = ResourceAffinity::PhysicalQubit(qubit);
        self
    }

    /// Removes qubit affinity.
    #[must_use]
    pub const fn no_qubit_affinity(mut self) -> Self {
        self.affinity = ResourceAffinity::None;
        self
    }

    /// Sets ownership.
    #[must_use]
    pub const fn ownership(mut self, ownership: ResourceOwnership) -> Self {
        self.ownership = ownership;
        self
    }

    /// Builds the immutable resource descriptor.
    #[must_use]
    pub const fn build(self) -> Resource {
        Resource::new(
            self.id,
            self.kind,
            self.capacity,
            self.sharing,
            self.state,
            self.scope,
            self.affinity,
            self.ownership,
        )
    }
}

// =============================================================================
// Resource capability
// =============================================================================

/// Capability metadata attached to a resource.
///
/// This type intentionally does not describe quantum gate semantics. It only
/// describes resource-level properties that a scheduler may need when deciding
/// whether an operation can use the resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceCapability {
    /// Whether the resource may participate in concurrent scheduling.
    pub concurrent: bool,

    /// Whether the resource can be used remotely.
    pub remote: bool,

    /// Whether the resource supports dynamic availability.
    pub dynamic_availability: bool,

    /// Whether the resource can be subdivided into capacity units.
    pub divisible: bool,
}

impl ResourceCapability {
    /// Creates a capability descriptor.
    #[must_use]
    pub const fn new(
        concurrent: bool,
        remote: bool,
        dynamic_availability: bool,
        divisible: bool,
    ) -> Self {
        Self {
            concurrent,
            remote,
            dynamic_availability,
            divisible,
        }
    }

    /// Derives conservative capabilities from a resource descriptor.
    #[must_use]
    pub const fn from_resource(resource: &Resource) -> Self {
        Self {
            concurrent: resource.permits_concurrent_use(),
            remote: matches!(resource.ownership(), ResourceOwnership::Remote),
            dynamic_availability: true,
            divisible: !matches!(
                resource.sharing(),
                ResourceSharing::Exclusive | ResourceSharing::Reusable
            ),
        }
    }
}

// =============================================================================
// Resource validation
// =============================================================================

/// Validation error for a malformed resource descriptor.
///
/// This is deliberately local to the resource model. Scheduling-level errors
/// can wrap or translate it without requiring the resource type to depend on
/// the scheduler error hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceValidationError {
    /// A logical-qubit resource lacks logical-qubit affinity.
    MissingLogicalQubitAffinity,

    /// A physical-qubit resource lacks physical-qubit affinity.
    MissingPhysicalQubitAffinity,

    /// A qubit resource has the wrong affinity kind.
    MismatchedQubitAffinity,

    /// A consumable resource cannot expose zero capacity.
    ZeroConsumableCapacity,

    /// An exclusive resource cannot expose a capacity other than one.
    InvalidExclusiveCapacity,

    /// A requirement quantity cannot be zero for a capacity-consuming mode.
    ZeroRequirementQuantity,
}

impl fmt::Display for ResourceValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingLogicalQubitAffinity => {
                "logical-qubit resource requires logical-qubit affinity"
            }
            Self::MissingPhysicalQubitAffinity => {
                "physical-qubit resource requires physical-qubit affinity"
            }
            Self::MismatchedQubitAffinity => {
                "qubit resource has mismatched qubit affinity"
            }
            Self::ZeroConsumableCapacity => {
                "consumable resource cannot have zero capacity"
            }
            Self::InvalidExclusiveCapacity => {
                "exclusive resource must have capacity one or unlimited capacity"
            }
            Self::ZeroRequirementQuantity => {
                "capacity-consuming resource requirement must have non-zero quantity"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ResourceValidationError {}

impl Resource {
    /// Validates the static semantic consistency of this resource.
    ///
    /// This method does not validate target-specific constraints. Those belong
    /// to hardware compatibility and target validation.
    pub fn validate(&self) -> Result<(), ResourceValidationError> {
        match self.kind {
            ResourceKind::LogicalQubit => {
                if !self.affinity.is_logical_qubit() {
                    return Err(
                        ResourceValidationError::MissingLogicalQubitAffinity,
                    );
                }
            }

            ResourceKind::PhysicalQubit => {
                if !self.affinity.is_physical_qubit() {
                    return Err(
                        ResourceValidationError::MissingPhysicalQubitAffinity,
                    );
                }
            }

            _ => {
                if self.affinity.is_logical_qubit()
                    && !matches!(self.kind, ResourceKind::QuantumMemory)
                {
                    return Err(
                        ResourceValidationError::MismatchedQubitAffinity,
                    );
                }

                if self.affinity.is_physical_qubit()
                    && !matches!(
                        self.kind,
                        ResourceKind::PhysicalQubit
                            | ResourceKind::QuantumMemory
                            | ResourceKind::ControlChannel
                            | ResourceKind::MeasurementChannel
                            | ResourceKind::ReadoutResonator
                            | ResourceKind::Coupler
                            | ResourceKind::Ancilla
                    )
                {
                    return Err(
                        ResourceValidationError::MismatchedQubitAffinity,
                    );
                }
            }
        }

        if matches!(self.sharing, ResourceSharing::Consumable)
            && self.capacity.is_zero()
        {
            return Err(ResourceValidationError::ZeroConsumableCapacity);
        }

        if matches!(self.sharing, ResourceSharing::Exclusive) {
            if let ResourceCapacity::Finite(capacity) = self.capacity {
                if capacity != 1 {
                    return Err(
                        ResourceValidationError::InvalidExclusiveCapacity,
                    );
                }
            }
        }

        Ok(())
    }
}

impl ResourceRequirement {
    /// Validates the static semantic consistency of this requirement.
    pub fn validate(&self) -> Result<(), ResourceValidationError> {
        if self.use_mode.consumes_capacity() && self.quantity.is_zero() {
            return Err(ResourceValidationError::ZeroRequirementQuantity);
        }

        Ok(())
    }
}

// =============================================================================
// Resource set relation
// =============================================================================

/// Static relationship between two resource descriptors.
///
/// This does not inspect time or reservations.
///
/// It is useful for planners that need to reason about resource equivalence or
/// incompatibility before constructing a schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceRelation {
    /// Both references identify the same canonical resource.
    Same,

    /// Resources are distinct but semantically independent.
    Independent,

    /// Resources represent different views of the same physical capability.
    Aliased,

    /// Resources cannot be safely treated as independent.
    Conflicting,
}

impl ResourceRelation {
    /// Returns whether the relationship permits treating resources as
    /// independent for static analysis.
    #[must_use]
    pub const fn is_independent(self) -> bool {
        matches!(self, Self::Independent)
    }

    /// Returns whether the relationship identifies the same resource.
    #[must_use]
    pub const fn is_same(self) -> bool {
        matches!(self, Self::Same)
    }

    /// Returns whether the relationship requires conflict handling.
    #[must_use]
    pub const fn requires_conflict_handling(self) -> bool {
        matches!(self, Self::Conflicting | Self::Aliased)
    }
}

// =============================================================================
// Static resource compatibility
// =============================================================================

impl Resource {
    /// Determines the static relation between this resource and another.
    ///
    /// This method intentionally does not attempt to infer arbitrary hardware
    /// topology. Such inference belongs to the hardware/routing adapter.
    #[must_use]
    pub fn relation_to(&self, other: &Self) -> ResourceRelation {
        if self.id == other.id {
            return ResourceRelation::Same;
        }

        if self.affinity == other.affinity
            && !self.affinity.is_none()
        {
            return ResourceRelation::Aliased;
        }

        if self.kind == other.kind
            && self.scope == other.scope
            && self.ownership == other.ownership
        {
            return ResourceRelation::Independent;
        }

        ResourceRelation::Independent
    }
}

// =============================================================================
// Resource identity helpers
// =============================================================================

/// Returns the canonical resource identity from a resource.
///
/// This free function exists as a small integration helper for adapters and
/// generic collection code.
#[must_use]
pub const fn resource_id(resource: &Resource) -> ResourceId {
    resource.id()
}

/// Returns the canonical resource identity from a requirement.
#[must_use]
pub const fn requirement_resource_id(
    requirement: ResourceRequirement,
) -> ResourceId {
    requirement.resource().id()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_capacity_satisfies_valid_quantity() {
        let capacity = ResourceCapacity::finite(4);

        assert!(capacity.can_satisfy(0));
        assert!(capacity.can_satisfy(1));
        assert!(capacity.can_satisfy(4));
        assert!(!capacity.can_satisfy(5));
    }

    #[test]
    fn unlimited_capacity_has_no_static_scalar_ceiling() {
        let capacity = ResourceCapacity::unlimited();

        assert!(capacity.can_satisfy(0));
        assert!(capacity.can_satisfy(1));
        assert!(capacity.can_satisfy(u128::MAX));
    }

    #[test]
    fn capacity_checked_addition_does_not_wrap() {
        let left = ResourceCapacity::finite(u128::MAX);
        let right = ResourceCapacity::finite(1);

        assert_eq!(left.checked_add(right), None);
    }

    #[test]
    fn unlimited_capacity_remains_unlimited() {
        let left = ResourceCapacity::unlimited();
        let right = ResourceCapacity::finite(u128::MAX);

        assert_eq!(
            left.checked_add(right),
            Some(ResourceCapacity::Unlimited)
        );
    }

    #[test]
    fn quantity_checked_arithmetic_does_not_wrap() {
        let value = ResourceQuantity::new(u128::MAX);

        assert_eq!(
            value.checked_add(ResourceQuantity::ONE),
            None
        );
    }

    #[test]
    fn exclusive_unit_resource_is_valid() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::ControlChannel,
        );

        assert_eq!(
            resource.capacity(),
            ResourceCapacity::Finite(1)
        );

        assert_eq!(
            resource.sharing(),
            ResourceSharing::Exclusive
        );

        assert!(resource.validate().is_ok());
    }

    #[test]
    fn logical_qubit_resource_requires_logical_affinity() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::LogicalQubit,
        );

        assert_eq!(
            resource.validate(),
            Err(
                ResourceValidationError::MissingLogicalQubitAffinity
            )
        );
    }

    #[test]
    fn physical_qubit_resource_requires_physical_affinity() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::PhysicalQubit,
        );

        assert_eq!(
            resource.validate(),
            Err(
                ResourceValidationError::MissingPhysicalQubitAffinity
            )
        );
    }

    #[test]
    fn observation_does_not_require_capacity() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::MeasurementChannel,
        );

        let requirement =
            ResourceRequirement::observe(ResourceRef::new(
                resource.id(),
            ));

        assert!(requirement.validate().is_ok());
        assert!(!requirement.consumes_capacity());
        assert!(resource.can_satisfy(requirement));
    }

    #[test]
    fn shared_resource_can_expose_multiple_units() {
        let resource = ResourceBuilder::new(
            ResourceId::new(7),
            ResourceKind::MeasurementChannel,
        )
        .finite_capacity(8)
        .sharing(ResourceSharing::Shared)
        .build();

        let requirement = ResourceRequirement::shared(
            ResourceRef::new(resource.id()),
            ResourceQuantity::new(4),
        );

        assert!(resource.validate().is_ok());
        assert!(requirement.validate().is_ok());
        assert!(resource.can_satisfy(requirement));
        assert!(resource.permits_concurrent_use());
    }

    #[test]
    fn too_large_requirement_is_rejected_by_static_capacity() {
        let resource = ResourceBuilder::new(
            ResourceId::new(7),
            ResourceKind::MeasurementChannel,
        )
        .finite_capacity(4)
        .sharing(ResourceSharing::Shared)
        .build();

        let requirement = ResourceRequirement::shared(
            ResourceRef::new(resource.id()),
            ResourceQuantity::new(5),
        );

        assert!(!resource.can_satisfy(requirement));
    }

    #[test]
    fn physical_qubit_affinity_is_canonical() {
        let qubit = PhysicalQubitId::new(11);

        let resource = ResourceBuilder::new(
            ResourceId::new(100),
            ResourceKind::PhysicalQubit,
        )
        .physical_qubit(qubit)
        .build();

        assert!(resource.validate().is_ok());
        assert_eq!(
            resource.affinity().physical_qubit(),
            Some(qubit)
        );
    }

    #[test]
    fn logical_qubit_affinity_is_canonical() {
        let qubit = QubitId::new(11);

        let resource = ResourceBuilder::new(
            ResourceId::new(100),
            ResourceKind::LogicalQubit,
        )
        .logical_qubit(qubit)
        .build();

        assert!(resource.validate().is_ok());
        assert_eq!(
            resource.affinity().logical_qubit(),
            Some(qubit)
        );
    }

    #[test]
    fn same_resource_has_same_relation() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::ControlChannel,
        );

        assert_eq!(
            resource.relation_to(&resource),
            ResourceRelation::Same
        );
    }

    #[test]
    fn different_resources_are_not_identical() {
        let left = Resource::unit(
            ResourceId::new(1),
            ResourceKind::ControlChannel,
        );

        let right = Resource::unit(
            ResourceId::new(2),
            ResourceKind::ControlChannel,
        );

        assert_eq!(
            left.relation_to(&right),
            ResourceRelation::Independent
        );
    }

    #[test]
    fn resource_requirement_uses_canonical_resource_id() {
        let resource_id = ResourceId::new(42);
        let requirement =
            ResourceRequirement::exclusive(ResourceRef::new(resource_id));

        assert_eq!(
            requirement_resource_id(requirement),
            resource_id
        );
    }

    #[test]
    fn capability_is_derived_without_global_state() {
        let resource = ResourceBuilder::new(
            ResourceId::new(1),
            ResourceKind::MeasurementChannel,
        )
        .finite_capacity(8)
        .sharing(ResourceSharing::Shared)
        .build();

        let capability =
            ResourceCapability::from_resource(&resource);

        assert!(capability.concurrent);
        assert!(capability.divisible);
        assert!(!capability.remote);
        assert!(capability.dynamic_availability);
    }

    #[test]
    fn resource_state_available_is_detected() {
        let resource = Resource::unit(
            ResourceId::new(1),
            ResourceKind::ControlChannel,
        );

        assert!(resource.is_available());
        assert!(!resource.state().is_unavailable());
    }

    #[test]
    fn zero_capacity_is_valid_for_non_consumable_resource() {
        let resource = ResourceBuilder::new(
            ResourceId::new(1),
            ResourceKind::ControlChannel,
        )
        .finite_capacity(0)
        .sharing(ResourceSharing::Shared)
        .build();

        assert!(resource.validate().is_ok());

        let requirement = ResourceRequirement::observe(
            ResourceRef::new(resource.id()),
        );

        assert!(resource.can_satisfy(requirement));
    }
}