//! Distributed quantum scheduling node model.
//!
//! This module defines the scheduler-facing description of a distributed
//! execution node.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "What scheduling resources and capabilities does this distributed
//! > execution node expose?"
//!
//! It does NOT:
//!
//! - discover hardware;
//! - perform routing;
//! - establish network connections;
//! - execute quantum operations;
//! - perform calibration;
//! - allocate physical hardware;
//! - schedule operations;
//! - select a vendor;
//! - communicate with a QPU;
//! - define quantum semantics;
//! - define a second `QubitId` or `PhysicalQubitId`;
//! - impose a maximum number of nodes;
//! - impose a maximum number of qubits;
//! - impose a fixed topology.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      │ semantic program
//!      ▼
//! quantum::routing
//!      │
//!      │ placement / mapping
//!      ▼
//! quantum::scheduling
//!      │
//!      ├── distributed::node  ← this module
//!      ├── distributed::link
//!      ├── distributed::communication
//!      └── distributed::network
//!      │
//!      ▼
//! quantum::hardware
//!      │
//!      ▼
//! runtime / backend
//! ```
//!
//! # Canonical identity
//!
//! The canonical quantum identities are:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module intentionally uses those types rather than introducing local
//! qubit identifiers.
//!
//! The canonical distributed semantic node identity is:
//!
//! ```text
//! quantum::ir::model::distributed::NodeId
//! ```
//!
//! This scheduler module does not create another semantic `NodeId`.
//!
//! # Write once, scale everywhere
//!
//! A node may represent:
//!
//! - a tiny quantum processor;
//! - a single QPU;
//! - one chip in a multi-chip system;
//! - one module in a modular quantum computer;
//! - a quantum-memory domain;
//! - a simulator partition;
//! - a network execution domain;
//! - a future quantum architecture.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_CORES
//! MAX_MEMORY
//! MAX_NODES
//! ```
//!
//! Quantities are represented explicitly and are constrained only by the
//! actual value supplied by the target or by an external resource policy.
//!
//! "Infinity" means that this module imposes no artificial finite architectural
//! ceiling. Actual executions remain bounded by available resources,
//! address-space capacity, target capability and explicit policy.
//!
//! # Determinism
//!
//! Resource collections use ordered containers where collection ordering is
//! observable. This makes diagnostics, hashing inputs, testing and scheduling
//! decisions reproducible.
//!
//! # Safety
//!
//! This module contains no unsafe code.
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
//! # Integration contract
//!
//! This file is intentionally independent of:
//!
//! - `quantum::scheduling::distributed::link`;
//! - `quantum::scheduling::distributed::network`;
//! - `quantum::scheduling::distributed::communication`;
//! - `quantum::hardware`;
//! - `quantum::routing`;
//! - scheduler algorithms;
//! - runtime implementations.
//!
//! Those modules consume this node description through its public API.
//!
//! No change to this file is required merely because one of those downstream
//! implementations is added.
//!
//! # Important distinction
//!
//! A `QuantumNode` is a scheduler-facing *description*.
//!
//! Constructing one does not claim that a real machine exists.
//! Hardware validation remains the responsibility of the hardware adapter.
//!
//! A node may therefore safely be created during compilation from a target
//! description, simulation target, test fixture or distributed planning
//! model.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::ir::model::distributed::NodeId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Local scheduler resource identifiers
// ============================================================================

/// Stable identifier for a resource owned or exposed by a scheduling node.
///
/// This identifier is deliberately independent of qubit identity.
///
/// A resource may represent a control channel, measurement channel, classical
/// processing capacity, memory capacity, or another scheduler-visible
/// consumable/exclusive resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeResourceId(u64);

impl NodeResourceId {
    /// Creates a resource identifier from an externally assigned value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next representable identifier, if one exists.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for NodeResourceId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<NodeResourceId> for u64 {
    fn from(value: NodeResourceId) -> Self {
        value.value()
    }
}

impl fmt::Display for NodeResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "resource{}", self.0)
    }
}

// ============================================================================
// Resource quantities
// ============================================================================

/// Scheduler-visible resource capacity.
///
/// `Unbounded` is explicit rather than being encoded as `u64::MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceCapacity {
    /// A finite capacity.
    Finite(u64),

    /// No finite capacity is declared by the target description.
    Unbounded,
}

impl ResourceCapacity {
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

    /// Returns `true` when the capacity is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns the finite capacity, if present.
    #[must_use]
    pub const fn as_finite(self) -> Option<u64> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded => None,
        }
    }

    /// Checks whether a required quantity can fit within this capacity.
    #[must_use]
    pub const fn can_satisfy(self, required: u64) -> bool {
        match self {
            Self::Finite(capacity) => required <= capacity,
            Self::Unbounded => true,
        }
    }
}

impl Default for ResourceCapacity {
    fn default() -> Self {
        Self::Finite(0)
    }
}

// ============================================================================
// Node lifecycle/state
// ============================================================================

/// Scheduler-visible availability state of a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeAvailability {
    /// The node is available for scheduling.
    Available,

    /// The node is temporarily unavailable.
    Unavailable,

    /// The node is available but degraded.
    Degraded,

    /// The target has not supplied enough information to determine state.
    Unknown,
}

impl Default for NodeAvailability {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============================================================================
// Node technology abstraction
// ============================================================================

/// Technology-neutral classification of a scheduling node.
///
/// This is intentionally descriptive rather than prescriptive. A target may
/// use a technology not represented by a named variant through `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeTechnology {
    /// Superconducting quantum processor.
    Superconducting,

    /// Trapped-ion quantum processor.
    TrappedIon,

    /// Neutral-atom quantum processor.
    NeutralAtom,

    /// Photonic quantum processor.
    Photonic,

    /// Spin-based quantum processor.
    Spin,

    /// Semiconductor or quantum-dot architecture.
    QuantumDot,

    /// Topological architecture.
    Topological,

    /// Annealing-oriented quantum system.
    Annealing,

    /// Continuous-variable or bosonic architecture.
    ContinuousVariable,

    /// Quantum-memory-oriented node.
    QuantumMemory,

    /// Classical simulator or emulator execution domain.
    Simulator,

    /// Technology not covered by the built-in taxonomy.
    Other(String),
}

// ============================================================================
// Node capabilities
// ============================================================================

/// A capability exposed by a distributed scheduling node.
///
/// Capabilities describe what the target claims to support. They do not cause
/// the scheduler to execute anything.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeCapability {
    /// General quantum instruction execution.
    QuantumExecution,

    /// Quantum measurement.
    Measurement,

    /// Quantum reset/reinitialization.
    Reset,

    /// Mid-circuit measurement.
    MidCircuitMeasurement,

    /// Classical control based on measurement results.
    ClassicalFeedback,

    /// Dynamic conditional execution.
    DynamicControl,

    /// Local entangling operations.
    LocalEntanglement,

    /// Inter-node entanglement generation.
    RemoteEntanglement,

    /// Quantum state transfer/teleportation support.
    QuantumStateTransfer,

    /// Classical communication.
    ClassicalCommunication,

    /// Local quantum memory.
    QuantumMemory,

    /// Error-correction support.
    ErrorCorrection,

    /// Fault-tolerant logical operation support.
    FaultTolerantExecution,

    /// Pulse-level execution.
    PulseExecution,

    /// Analog execution.
    AnalogExecution,

    /// A target-specific extensible capability.
    Other(String),
}

// ============================================================================
// Resource kinds
// ============================================================================

/// Kind of resource that can be consumed by operations scheduled on a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeResourceKind {
    /// A quantum processing resource.
    QuantumProcessing,

    /// A physical qubit resource.
    PhysicalQubit,

    /// A measurement/readout resource.
    MeasurementChannel,

    /// A quantum-control resource.
    ControlChannel,

    /// A classical processing resource.
    ClassicalProcessing,

    /// A classical communication resource.
    ClassicalCommunication,

    /// A quantum communication resource.
    QuantumCommunication,

    /// An entanglement-generation resource.
    EntanglementGeneration,

    /// Quantum memory.
    QuantumMemory,

    /// Generic memory.
    ClassicalMemory,

    /// Energy or power budget.
    Energy,

    /// A target-specific resource.
    Other(String),
}

// ============================================================================
// Resource descriptor
// ============================================================================

/// Scheduler-visible description of a node resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeResource {
    id: NodeResourceId,
    kind: NodeResourceKind,
    capacity: ResourceCapacity,
    label: Option<String>,
    exclusive: bool,
}

impl NodeResource {
    /// Creates a resource descriptor.
    #[must_use]
    pub const fn new(
        id: NodeResourceId,
        kind: NodeResourceKind,
        capacity: ResourceCapacity,
        exclusive: bool,
    ) -> Self {
        Self {
            id,
            kind,
            capacity,
            label: None,
            exclusive,
        }
    }

    /// Adds an optional descriptive label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Returns the resource identifier.
    #[must_use]
    pub const fn id(&self) -> NodeResourceId {
        self.id
    }

    /// Returns the resource kind.
    #[must_use]
    pub const fn kind(&self) -> &NodeResourceKind {
        &self.kind
    }

    /// Returns the resource capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceCapacity {
        self.capacity
    }

    /// Returns whether the resource is exclusive.
    #[must_use]
    pub const fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    /// Returns the optional label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

// ============================================================================
// Qubit placement
// ============================================================================

/// Scheduler-visible association between a logical qubit and a physical qubit
/// on this node.
///
/// This is a *description* of an already-established placement/mapping
/// decision. It does not perform routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitPlacement {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitPlacement {
    /// Creates a logical-to-physical placement.
    #[must_use]
    pub const fn new(logical: QubitId, physical: PhysicalQubitId) -> Self {
        Self { logical, physical }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(self) -> PhysicalQubitId {
        self.physical
    }
}

// ============================================================================
// Node identity/metadata
// ============================================================================

/// Scheduler-facing distributed execution node.
///
/// The node owns no scheduler algorithm and contains no hardware SDK object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumNode {
    id: NodeId,
    label: Option<String>,
    technology: Option<NodeTechnology>,
    availability: NodeAvailability,
    capabilities: BTreeSet<NodeCapability>,
    resources: BTreeMap<NodeResourceId, NodeResource>,
    placements: BTreeMap<QubitId, PhysicalQubitId>,
}

impl QuantumNode {
    /// Creates an empty node description.
    ///
    /// No real-world resource is assumed to exist.
    #[must_use]
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            label: None,
            technology: None,
            availability: NodeAvailability::Unknown,
            capabilities: BTreeSet::new(),
            resources: BTreeMap::new(),
            placements: BTreeMap::new(),
        }
    }

    /// Creates a node with a descriptive label.
    #[must_use]
    pub fn with_label(id: NodeId, label: impl Into<String>) -> Self {
        Self::new(id).label(label)
    }

    /// Sets the descriptive label.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the node technology classification.
    #[must_use]
    pub fn technology(mut self, technology: NodeTechnology) -> Self {
        self.technology = Some(technology);
        self
    }

    /// Sets the scheduler-visible availability state.
    #[must_use]
    pub const fn availability(mut self, availability: NodeAvailability) -> Self {
        self.availability = availability;
        self
    }

    /// Adds a capability.
    #[must_use]
    pub fn capability(mut self, capability: NodeCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds a resource.
    ///
    /// If a resource with the same identifier already exists, it is replaced.
    /// Resource identity therefore remains unique and deterministic.
    #[must_use]
    pub fn resource(mut self, resource: NodeResource) -> Self {
        self.resources.insert(resource.id(), resource);
        self
    }

    /// Adds a logical-to-physical qubit placement.
    ///
    /// This does not perform routing or validate hardware connectivity.
    #[must_use]
    pub fn placement(mut self, placement: QubitPlacement) -> Self {
        self.placements
            .insert(placement.logical(), placement.physical());
        self
    }

    /// Returns the canonical distributed node identifier.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the optional descriptive label.
    #[must_use]
    pub fn label_ref(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the optional technology classification.
    #[must_use]
    pub fn technology_ref(&self) -> Option<&NodeTechnology> {
        self.technology.as_ref()
    }

    /// Returns the availability state.
    #[must_use]
    pub const fn availability_ref(&self) -> NodeAvailability {
        self.availability
    }

    /// Returns all capabilities in deterministic order.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<NodeCapability> {
        &self.capabilities
    }

    /// Returns all resources in deterministic identifier order.
    #[must_use]
    pub fn resources(&self) -> &BTreeMap<NodeResourceId, NodeResource> {
        &self.resources
    }

    /// Returns all logical-to-physical placements in deterministic order.
    #[must_use]
    pub fn placements(&self) -> &BTreeMap<QubitId, PhysicalQubitId> {
        &self.placements
    }

    /// Returns whether the node declares a capability.
    #[must_use]
    pub fn supports(&self, capability: &NodeCapability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Returns a resource by its scheduler-visible identifier.
    #[must_use]
    pub fn resource_by_id(&self, id: NodeResourceId) -> Option<&NodeResource> {
        self.resources.get(&id)
    }

    /// Returns all resources of a particular kind.
    ///
    /// The iterator remains lazy and does not allocate a temporary collection.
    pub fn resources_of_kind(
        &self,
        kind: &NodeResourceKind,
    ) -> impl Iterator<Item = &NodeResource> {
        self.resources
            .values()
            .filter(move |resource| resource.kind() == kind)
    }

    /// Returns the physical qubit associated with a logical qubit.
    #[must_use]
    pub fn physical_qubit(&self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.placements.get(&logical).copied()
    }

    /// Returns the logical qubit associated with a physical qubit.
    ///
    /// This is an O(n) lookup because the canonical representation is optimized
    /// for logical-qubit lookup and deterministic storage. Callers performing
    /// large-scale reverse lookups should build their own derived index rather
    /// than changing the canonical representation.
    #[must_use]
    pub fn logical_qubit(&self, physical: PhysicalQubitId) -> Option<QubitId> {
        self.placements
            .iter()
            .find_map(|(logical, mapped)| {
                if *mapped == physical {
                    Some(*logical)
                } else {
                    None
                }
            })
    }

    /// Returns the number of logical-to-physical placements.
    #[must_use]
    pub fn placement_count(&self) -> usize {
        self.placements.len()
    }

    /// Returns the number of declared resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether this node is currently schedulable.
    ///
    /// `Available` and `Degraded` are considered schedulable. `Unknown` is not
    /// treated as available because silently scheduling onto an unknown target
    /// would turn missing information into an unsafe assumption.
    #[must_use]
    pub const fn is_schedulable(&self) -> bool {
        matches!(
            self.availability,
            NodeAvailability::Available | NodeAvailability::Degraded
        )
    }

    /// Validates internal structural invariants.
    ///
    /// This method validates only the node description itself. It does not
    /// validate hardware existence or target topology.
    pub fn validate(&self) -> Result<(), NodeError> {
        for (resource_id, resource) in &self.resources {
            if *resource_id != resource.id() {
                return Err(NodeError::ResourceKeyMismatch {
                    key: *resource_id,
                    resource: resource.id(),
                });
            }
        }

        // A physical qubit may not be assigned to two logical qubits within the
        // same placement snapshot. Such a state would make scheduling
        // ambiguous and must be rejected before planning.
        let mut physical_qubits = BTreeSet::new();

        for (logical, physical) in &self.placements {
            if !physical_qubits.insert(*physical) {
                return Err(NodeError::DuplicatePhysicalQubit {
                    physical: *physical,
                    logical: *logical,
                });
            }
        }

        Ok(())
    }
}

impl Default for QuantumNode {
    fn default() -> Self {
        Self::new(NodeId::new(0))
    }
}

// ============================================================================
// Node collections
// ============================================================================

/// Deterministic collection of distributed scheduling nodes.
///
/// This type deliberately does not encode a maximum node count.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NodeSet {
    nodes: BTreeMap<NodeId, QuantumNode>,
}

impl NodeSet {
    /// Creates an empty node set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Inserts a node.
    ///
    /// Returns the previous node with the same identity, if one existed.
    pub fn insert(&mut self, node: QuantumNode) -> Option<QuantumNode> {
        self.nodes.insert(node.id(), node)
    }

    /// Removes a node by identity.
    pub fn remove(&mut self, id: NodeId) -> Option<QuantumNode> {
        self.nodes.remove(&id)
    }

    /// Returns a node by identity.
    #[must_use]
    pub fn get(&self, id: NodeId) -> Option<&QuantumNode> {
        self.nodes.get(&id)
    }

    /// Returns a mutable node by identity.
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut QuantumNode> {
        self.nodes.get_mut(&id)
    }

    /// Returns whether a node exists.
    #[must_use]
    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Iterates over nodes in deterministic identifier order.
    pub fn iter(&self) -> impl Iterator<Item = (&NodeId, &QuantumNode)> {
        self.nodes.iter()
    }

    /// Iterates over node values in deterministic identifier order.
    pub fn values(&self) -> impl Iterator<Item = &QuantumNode> {
        self.nodes.values()
    }

    /// Returns the underlying deterministic map.
    #[must_use]
    pub fn as_map(&self) -> &BTreeMap<NodeId, QuantumNode> {
        &self.nodes
    }

    /// Validates every node.
    pub fn validate(&self) -> Result<(), NodeError> {
        for node in self.nodes.values() {
            node.validate()?;
        }

        Ok(())
    }

    /// Returns all schedulable nodes.
    ///
    /// The returned iterator is lazy and allocation-free.
    pub fn schedulable_nodes(&self) -> impl Iterator<Item = &QuantumNode> {
        self.nodes.values().filter(|node| node.is_schedulable())
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while validating scheduler node descriptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeError {
    /// The map key and resource's internal identifier differ.
    ResourceKeyMismatch {
        /// Identifier used as the map key.
        key: NodeResourceId,

        /// Identifier stored in the resource.
        resource: NodeResourceId,
    },

    /// Two logical qubits refer to the same physical qubit.
    DuplicatePhysicalQubit {
        /// Physical qubit assigned more than once.
        physical: PhysicalQubitId,

        /// Logical qubit involved in the duplicate assignment.
        logical: QubitId,
    },
}

impl fmt::Display for NodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceKeyMismatch { key, resource } => write!(
                formatter,
                "node resource map key {} does not match resource identifier {}",
                key.value(),
                resource.value()
            ),

            Self::DuplicatePhysicalQubit { physical, logical } => write!(
                formatter,
                "physical qubit {:?} is assigned to more than one logical qubit; \
                 conflicting logical qubit {:?}",
                physical,
                logical
            ),
        }
    }
}

impl std::error::Error for NodeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn logical_qubit(value: u64) -> QubitId {
        QubitId::new(value as usize).expect("test logical qubit ID must be valid")
    }

    fn physical_qubit(value: u64) -> PhysicalQubitId {
        PhysicalQubitId::new(value as usize)
            .expect("test physical qubit ID must be valid")
    }

    #[test]
    fn node_identity_is_stable() {
        let node = QuantumNode::new(node_id(42));

        assert_eq!(node.id(), node_id(42));
        assert_eq!(node.label_ref(), None);
        assert_eq!(node.availability_ref(), NodeAvailability::Unknown);
        assert!(!node.is_schedulable());
    }

    #[test]
    fn node_can_be_built_without_machine_size_assumptions() {
        let node = QuantumNode::new(node_id(u64::MAX))
            .availability(NodeAvailability::Available)
            .capability(NodeCapability::QuantumExecution);

        assert_eq!(node.id(), node_id(u64::MAX));
        assert!(node.is_schedulable());
        assert!(node.supports(&NodeCapability::QuantumExecution));
    }

    #[test]
    fn resource_capacity_is_explicitly_unbounded() {
        let capacity = ResourceCapacity::unbounded();

        assert!(capacity.is_unbounded());
        assert!(capacity.can_satisfy(u64::MAX));
        assert_eq!(capacity.as_finite(), None);
    }

    #[test]
    fn finite_resource_capacity_is_checked() {
        let capacity = ResourceCapacity::finite(8);

        assert!(capacity.can_satisfy(8));
        assert!(!capacity.can_satisfy(9));
    }

    #[test]
    fn resources_are_deterministic() {
        let first = NodeResource::new(
            NodeResourceId::new(10),
            NodeResourceKind::ControlChannel,
            ResourceCapacity::finite(1),
            true,
        );

        let second = NodeResource::new(
            NodeResourceId::new(2),
            NodeResourceKind::MeasurementChannel,
            ResourceCapacity::finite(4),
            false,
        );

        let node = QuantumNode::new(node_id(1))
            .resource(first)
            .resource(second);

        let ids: Vec<NodeResourceId> = node.resources().keys().copied().collect();

        assert_eq!(
            ids,
            vec![NodeResourceId::new(2), NodeResourceId::new(10)]
        );
    }

    #[test]
    fn placement_uses_canonical_qubit_types() {
        let logical = logical_qubit(7);
        let physical = physical_qubit(13);

        let node = QuantumNode::new(node_id(1))
            .availability(NodeAvailability::Available)
            .placement(QubitPlacement::new(logical, physical));

        assert_eq!(node.physical_qubit(logical), Some(physical));
        assert_eq!(node.logical_qubit(physical), Some(logical));
    }

    #[test]
    fn duplicate_physical_placement_is_rejected() {
        let node = QuantumNode::new(node_id(1))
            .placement(QubitPlacement::new(
                logical_qubit(1),
                physical_qubit(7),
            ))
            .placement(QubitPlacement::new(
                logical_qubit(2),
                physical_qubit(7),
            ));

        assert!(matches!(
            node.validate(),
            Err(NodeError::DuplicatePhysicalQubit { .. })
        ));
    }

    #[test]
    fn valid_node_passes_validation() {
        let node = QuantumNode::new(node_id(1))
            .availability(NodeAvailability::Available)
            .capability(NodeCapability::QuantumExecution)
            .capability(NodeCapability::Measurement)
            .resource(NodeResource::new(
                NodeResourceId::new(1),
                NodeResourceKind::QuantumProcessing,
                ResourceCapacity::finite(1),
                true,
            ))
            .placement(QubitPlacement::new(
                logical_qubit(1),
                physical_qubit(1),
            ))
            .placement(QubitPlacement::new(
                logical_qubit(2),
                physical_qubit(2),
            ));

        assert!(node.validate().is_ok());
    }

    #[test]
    fn unknown_nodes_are_not_silently_schedulable() {
        let node = QuantumNode::new(node_id(1));

        assert!(!node.is_schedulable());
    }

    #[test]
    fn degraded_nodes_remain_schedulable() {
        let node = QuantumNode::new(node_id(1))
            .availability(NodeAvailability::Degraded);

        assert!(node.is_schedulable());
    }

    #[test]
    fn node_set_is_deterministic() {
        let mut nodes = NodeSet::new();

        nodes.insert(QuantumNode::new(node_id(20)));
        nodes.insert(QuantumNode::new(node_id(2)));
        nodes.insert(QuantumNode::new(node_id(11)));

        let ids: Vec<NodeId> = nodes.iter().map(|(id, _)| *id).collect();

        assert_eq!(ids, vec![node_id(2), node_id(11), node_id(20)]);
    }

    #[test]
    fn node_set_replaces_by_identity() {
        let mut nodes = NodeSet::new();

        nodes.insert(QuantumNode::new(node_id(1)));
        let previous = nodes.insert(
            QuantumNode::new(node_id(1))
                .label("replacement")
                .availability(NodeAvailability::Available),
        );

        assert!(previous.is_some());
        assert_eq!(
            nodes
                .get(node_id(1))
                .and_then(QuantumNode::label_ref),
            Some("replacement")
        );
    }

    #[test]
    fn node_set_validation_covers_all_nodes() {
        let mut nodes = NodeSet::new();

        nodes.insert(
            QuantumNode::new(node_id(1))
                .placement(QubitPlacement::new(
                    logical_qubit(1),
                    physical_qubit(1),
                )),
        );

        nodes.insert(
            QuantumNode::new(node_id(2))
                .placement(QubitPlacement::new(
                    logical_qubit(2),
                    physical_qubit(2),
                )),
        );

        assert!(nodes.validate().is_ok());
    }

    #[test]
    fn resources_of_kind_is_lazy() {
        let node = QuantumNode::new(node_id(1))
            .resource(NodeResource::new(
                NodeResourceId::new(1),
                NodeResourceKind::ControlChannel,
                ResourceCapacity::finite(1),
                true,
            ))
            .resource(NodeResource::new(
                NodeResourceId::new(2),
                NodeResourceKind::MeasurementChannel,
                ResourceCapacity::finite(1),
                true,
            ));

        let count = node
            .resources_of_kind(&NodeResourceKind::ControlChannel)
            .count();

        assert_eq!(count, 1);
    }

    #[test]
    fn checked_identifier_progression_does_not_wrap() {
        let id = NodeResourceId::new(u64::MAX);

        assert_eq!(id.checked_next(), None);
    }

    #[test]
    fn explicit_unbounded_capacity_does_not_use_sentinel_values() {
        assert_ne!(
            ResourceCapacity::unbounded(),
            ResourceCapacity::finite(u64::MAX)
        );
    }
}