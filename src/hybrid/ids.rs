
#![forbid(unsafe_code)]

//! Typed identities for Zamani's hybrid-computation subsystem.
//!
//! # Architectural contract
//!
//! This module owns the identities of entities introduced by the hybrid
//! composition layer. It does not own source-language, semantic, classical
//! IR, quantum IR, hardware, backend, or runtime identities.
//!
//! The hybrid layer is a composition boundary, not a second IR. Accordingly,
//! identities for entities owned by another subsystem must remain those of
//! that subsystem and be referenced through explicit adapters.
//!
//! # Identity model
//!
//! - `HybridGraphId` identifies a hybrid graph.
//! - `DomainInstanceId` identifies a domain instance within a graph.
//! - `HybridNodeId` identifies a node within a graph.
//! - `HybridEdgeId` identifies an edge within a graph.
//! - `CrossDomainValueId` identifies a cross-domain value within a graph.
//! - `HybridDependencyId` identifies a dependency within a graph.
//! - `SynchronizationEventId` identifies a synchronization event within
//!   a graph.
//!
//! Graph-scoped identities contain their graph identity. Thus, two nodes
//! allocated independently in different graphs cannot compare equal merely
//! because their local numeric components happen to be equal.
//!
//! # Allocation and scalability
//!
//! IDs use non-zero `u128` components. Allocation is checked and never wraps,
//! silently reuses an identifier, or substitutes a fallback identity.
//!
//! `IdAllocator` is cloneable; clones share one synchronized allocation
//! sequence. Callers that require uniqueness across concurrently constructed
//! graphs must share the same allocator, or use an external namespace and
//! identity-management policy.
//!
//! The representable identifier space is finite, as every concrete data type
//! is finite. Exhaustion is reported explicitly. This is not a hardware,
//! graph-size, domain-count, or language-semantic limit.
//!
//! Allocation order is deterministic for a given serialized sequence of
//! allocation requests. Concurrent callers are safe, but the order in which
//! they obtain IDs depends on synchronization order. IDs are not content
//! hashes and must not be treated as reproducible across arbitrary execution
//! schedules.
//!
//! # Serialization
//!
//! Serde serialization preserves the typed structure. Deserialized IDs are
//! values, not proof that the referenced graph or entity exists. Graph and
//! reference validity must be established by the owning model's verifier.
//!
//! # Integration
//!
//! - `graph/model.rs` uses `HybridGraphId` and `HybridNodeId`.
//! - `graph/edges.rs` uses `HybridEdgeId` and `HybridDependencyId`.
//! - `domain/composition.rs` uses `DomainInstanceId`.
//! - `data/values.rs` uses `CrossDomainValueId`.
//! - `sync/model.rs` and related synchronization modules use
//!   `SynchronizationEventId`.
//! - `interchange/` serializes these IDs without replacing the canonical
//!   identifiers belonging to other IRs.
//! - `integration/semantic.rs`, `integration/quantum.rs`, and
//!   `integration/classical.rs` retain upstream identities and translate
//!   references only at explicit ownership boundaries.
//!
//! These are integration contracts for the proposed hybrid modules, not a
//! claim that those modules already exist.
//!
//! # Safety
//!
//! This module forbids unsafe Rust and uses safe standard-library
//! synchronization primitives.

use core::fmt;
use core::num::NonZeroU128;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

/// An error encountered while constructing or allocating hybrid IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IdError {
    /// Zero is not a valid identity component.
    Zero,

    /// The allocator has exhausted its `u128` identity space.
    Exhausted,

    /// The allocator's synchronization primitive was poisoned.
    ///
    /// Allocation is not resumed automatically because doing so would hide
    /// a failure in the shared allocation state.
    AllocatorPoisoned,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => f.write_str("hybrid identifiers must be non-zero"),
            Self::Exhausted => {
                f.write_str("hybrid identifier space exhausted")
            }
            Self::AllocatorPoisoned => {
                f.write_str("hybrid identifier allocator is poisoned")
            }
        }
    }
}

impl std::error::Error for IdError {}

/// A non-zero numeric identity component.
///
/// This type is useful when importing or adapting IDs from another system.
/// It does not by itself establish ownership or global uniqueness.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct IdComponent(NonZeroU128);

impl IdComponent {
    /// Construct a component from a non-zero integer.
    pub const fn new(value: u128) -> Result<Self, IdError> {
        match NonZeroU128::new(value) {
            Some(value) => Ok(Self(value)),
            None => Err(IdError::Zero),
        }
    }

    /// Return the numeric component.
    pub const fn get(self) -> u128 {
        self.0.get()
    }
}

impl fmt::Display for IdComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}

/// Identity of one hybrid computation graph.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct HybridGraphId(IdComponent);

impl HybridGraphId {
    /// Construct a graph ID from an existing non-zero component.
    ///
    /// This is intended for validated imports and explicit identity
    /// restoration. It does not check uniqueness against other graphs.
    pub const fn new(value: u128) -> Result<Self, IdError> {
        match IdComponent::new(value) {
            Ok(component) => Ok(Self(component)),
            Err(error) => Err(error),
        }
    }

    /// Return the numeric component.
    pub const fn get(self) -> u128 {
        self.0.get()
    }

    /// Return the underlying non-zero component.
    pub const fn component(self) -> IdComponent {
        self.0
    }
}

impl fmt::Display for HybridGraphId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hybrid-graph:{}", self.get())
    }
}

/// A graph-scoped identity component.
///
/// Keeping the graph identity in the value prevents accidental equality
/// between entities from different graphs.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize,
    Deserialize,
)]
pub struct GraphScopedId {
    graph: HybridGraphId,
    local: IdComponent,
}

impl GraphScopedId {
    /// Construct a graph-scoped ID from existing validated components.
    ///
    /// The caller is responsible for ensuring that `local` is unique among
    /// entities of the same ID type in the given graph.
    pub const fn new(
        graph: HybridGraphId,
        local: IdComponent,
    ) -> Self {
        Self { graph, local }
    }

    /// Return the owning graph.
    pub const fn graph(self) -> HybridGraphId {
        self.graph
    }

    /// Return the local identity component.
    pub const fn local(self) -> IdComponent {
        self.local
    }
}

impl fmt::Display for GraphScopedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.graph.get(), self.local.get())
    }
}

macro_rules! define_graph_scoped_id {
    (
        $(#[$meta:meta])*
        $name:ident, $label:literal
    ) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
            Serialize, Deserialize
        )]
        #[serde(transparent)]
        pub struct $name(GraphScopedId);

        impl $name {
            /// Construct an ID from an existing graph-scoped identity.
            ///
            /// This does not verify that the referenced entity exists.
            pub const fn new(id: GraphScopedId) -> Self {
                Self(id)
            }

            /// Return the owning graph.
            pub const fn graph(self) -> HybridGraphId {
                self.0.graph()
            }

            /// Return the local identity component.
            pub const fn local(self) -> IdComponent {
                self.0.local()
            }

            /// Return the underlying graph-scoped identity.
            pub const fn scoped(self) -> GraphScopedId {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($label, ":{}:{}"),
                    self.graph().get(),
                    self.local().get()
                )
            }
        }
    };
}

define_graph_scoped_id!(
    /// Identity of a participating domain instance within a hybrid graph.
    DomainInstanceId,
    "hybrid-domain"
);

define_graph_scoped_id!(
    /// Identity of a node within a hybrid graph.
    HybridNodeId,
    "hybrid-node"
);

define_graph_scoped_id!(
    /// Identity of an edge within a hybrid graph.
    HybridEdgeId,
    "hybrid-edge"
);

define_graph_scoped_id!(
    /// Identity of a value exchanged across computational domains.
    CrossDomainValueId,
    "hybrid-value"
);

define_graph_scoped_id!(
    /// Identity of an explicit dependency within a hybrid graph.
    HybridDependencyId,
    "hybrid-dependency"
);

define_graph_scoped_id!(
    /// Identity of a synchronization event within a hybrid graph.
    SynchronizationEventId,
    "hybrid-sync"
);

/// Shared, checked allocator for hybrid identities.
///
/// Cloning this allocator shares its state. The allocator uses a mutex
/// instead of an atomic `u128`, because portable atomic support for `u128`
/// is not guaranteed across the Rust targets Zamani may support.
///
/// The allocation state contains the next available component. `None`
/// means that the entire non-zero `u128` sequence has been exhausted.
#[derive(Debug, Clone)]
pub struct IdAllocator {
    next: Arc<Mutex<Option<u128>>>,
}

impl Default for IdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdAllocator {
    /// Create an allocator starting at component one.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next: Arc::new(Mutex::new(Some(1))),
        }
    }

    /// Allocate the next unused component.
    ///
    /// Allocation of `u128::MAX` is valid. The following request returns
    /// `IdError::Exhausted` rather than wrapping or reusing an ID.
    pub fn allocate_component(&self) -> Result<IdComponent, IdError> {
        let mut state = self
            .next
            .lock()
            .map_err(|_| IdError::AllocatorPoisoned)?;

        let current = state.ok_or(IdError::Exhausted)?;
        *state = current.checked_add(1);

        // `current` starts at one and is never zero.
        match IdComponent::new(current) {
            Ok(component) => Ok(component),
            Err(error) => Err(error),
        }
    }

    /// Allocate a new hybrid graph identity.
    pub fn allocate_graph_id(&self) -> Result<HybridGraphId, IdError> {
        let component = self.allocate_component()?;
        Ok(HybridGraphId(component))
    }

    /// Allocate a domain-instance identity in the supplied graph.
    pub fn allocate_domain_instance_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<DomainInstanceId, IdError> {
        self.allocate_scoped(graph).map(DomainInstanceId::new)
    }

    /// Allocate a node identity in the supplied graph.
    pub fn allocate_node_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<HybridNodeId, IdError> {
        self.allocate_scoped(graph).map(HybridNodeId::new)
    }

    /// Allocate an edge identity in the supplied graph.
    pub fn allocate_edge_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<HybridEdgeId, IdError> {
        self.allocate_scoped(graph).map(HybridEdgeId::new)
    }

    /// Allocate a cross-domain value identity in the supplied graph.
    pub fn allocate_cross_domain_value_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<CrossDomainValueId, IdError> {
        self.allocate_scoped(graph).map(CrossDomainValueId::new)
    }

    /// Allocate a dependency identity in the supplied graph.
    pub fn allocate_dependency_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<HybridDependencyId, IdError> {
        self.allocate_scoped(graph).map(HybridDependencyId::new)
    }

    /// Allocate a synchronization-event identity in the supplied graph.
    pub fn allocate_synchronization_event_id(
        &self,
        graph: HybridGraphId,
    ) -> Result<SynchronizationEventId, IdError> {
        self.allocate_scoped(graph).map(SynchronizationEventId::new)
    }

    fn allocate_scoped(
        &self,
        graph: HybridGraphId,
    ) -> Result<GraphScopedId, IdError> {
        let local = self.allocate_component()?;
        Ok(GraphScopedId::new(graph, local))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;

    #[test]
    fn rejects_zero_components() {
        assert_eq!(IdComponent::new(0), Err(IdError::Zero));
        assert_eq!(HybridGraphId::new(0), Err(IdError::Zero));
    }

    #[test]
    fn allocates_distinct_graph_ids() {
        let allocator = IdAllocator::new();

        let first = allocator.allocate_graph_id().unwrap();
        let second = allocator.allocate_graph_id().unwrap();

        assert_ne!(first, second);
        assert!(first.get() > 0);
        assert!(second.get() > 0);
    }

    #[test]
    fn allocator_clones_share_the_sequence() {
        let allocator = IdAllocator::new();
        let clone = allocator.clone();

        let first = allocator.allocate_component().unwrap();
        let second = clone.allocate_component().unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn scoped_ids_include_the_graph_identity() {
        let allocator = IdAllocator::new();
        let graph_a = allocator.allocate_graph_id().unwrap();
        let graph_b = allocator.allocate_graph_id().unwrap();

        let local = IdComponent::new(42).unwrap();
        let node_a = HybridNodeId::new(GraphScopedId::new(graph_a, local));
        let node_b = HybridNodeId::new(GraphScopedId::new(graph_b, local));

        assert_ne!(node_a, node_b);
        assert_eq!(node_a.local(), node_b.local());
        assert_ne!(node_a.graph(), node_b.graph());
    }

    #[test]
    fn distinct_entity_kinds_are_distinct_types() {
        let allocator = IdAllocator::new();
        let graph = allocator.allocate_graph_id().unwrap();

        let node = allocator.allocate_node_id(graph).unwrap();
        let edge = allocator.allocate_edge_id(graph).unwrap();

        assert_eq!(node.graph(), edge.graph());
        assert_ne!(node.local(), edge.local());
    }

    #[test]
    fn display_values_are_graph_qualified() {
        let graph = HybridGraphId::new(7).unwrap();
        let local = IdComponent::new(12).unwrap();
        let node = HybridNodeId::new(GraphScopedId::new(graph, local));

        assert_eq!(graph.to_string(), "hybrid-graph:7");
        assert_eq!(node.to_string(), "hybrid-node:7:12");
    }

    #[test]
    fn graph_scoped_ids_round_trip_through_serde_json() {
        let allocator = IdAllocator::new();
        let graph = allocator.allocate_graph_id().unwrap();
        let node = allocator.allocate_node_id(graph).unwrap();

        let encoded = serde_json::to_string(&node).unwrap();
        let decoded: HybridNodeId =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(node, decoded);
    }

    #[test]
    fn allocator_is_safe_for_concurrent_callers() {
        let allocator = IdAllocator::new();
        let mut workers = Vec::new();

        for _ in 0..8 {
            let shared = allocator.clone();
            workers.push(thread::spawn(move || {
                (0..128)
                    .map(|_| shared.allocate_component().unwrap())
                    .collect::<Vec<_>>()
            }));
        }

        let mut ids = HashSet::new();

        for worker in workers {
            for id in worker.join().unwrap() {
                assert!(ids.insert(id));
            }
        }

        assert_eq!(ids.len(), 8 * 128);
    }

    #[test]
    fn allocator_exhaustion_is_explicit() {
        let allocator = IdAllocator {
            next: Arc::new(Mutex::new(Some(u128::MAX))),
        };

        assert_eq!(
            allocator.allocate_component().unwrap().get(),
            u128::MAX
        );
        assert_eq!(
            allocator.allocate_component(),
            Err(IdError::Exhausted)
        );
        assert_eq!(
            allocator.allocate_component(),
            Err(IdError::Exhausted)
        );
    }

    #[test]
    fn identifiers_are_orderable_and_hashable() {
        let allocator = IdAllocator::new();
        let graph = allocator.allocate_graph_id().unwrap();

        let first = allocator.allocate_node_id(graph).unwrap();
        let second = allocator.allocate_node_id(graph).unwrap();

        assert!(first < second);

        let ids = HashSet::from([first, second]);
        assert_eq!(ids.len(), 2);
    }
}
