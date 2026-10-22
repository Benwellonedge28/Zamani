#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Synchronization Model
//!
//! Path: `src/hybrid/sync/model.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only. Unsafe code is forbidden.
//!
//! ## Purpose
//!
//! Define the canonical, target-independent vocabulary and representation
//! for synchronization requirements in hybrid computations.
//!
//! A synchronization declaration specifies guarantees that participating
//! computations must observe. It does not prescribe whether those guarantees
//! are implemented by threads, futures, events, queues, device mechanisms,
//! distributed protocols, or another execution mechanism.
//!
//! ## Ownership
//!
//! This module owns:
//! - Synchronization kinds and their semantic distinctions.
//! - Execution-mode intent at the hybrid boundary.
//! - Abstract synchronization scope.
//! - Graph-scoped synchronization event declarations.
//! - Structural validation of synchronization declarations.
//! - The synchronization-model container and its invariants.
//!
//! This module does not own:
//! - Source syntax, parsing, ASTs, or type checking.
//! - General concurrency semantics or the language-wide memory model.
//! - The canonical dependency graph or its edge definitions.
//! - Completion-state tracking or runtime futures.
//! - Physical barriers, locks, queues, device events, or transport protocols.
//! - Scheduling, placement, device selection, routing, or code generation.
//! - Classical IR, quantum IR, ZUIR, ZCIR, or HDLIR.
//!
//! ## Canonical integration
//!
//! - `crate::hybrid::ids` supplies graph-scoped synchronization-event and
//!   node identifiers.
//! - `crate::hybrid::errors` supplies the shared structured error type.
//! - `crate::hybrid::graph::edges` remains authoritative for dependency
//!   relationships. This module does not construct a second edge graph.
//! - `crate::hybrid::sync::ordering` consumes ordering-related guarantees.
//! - `crate::hybrid::sync::completion` consumes completion requirements.
//! - `crate::hybrid::sync::cancellation` consumes cancellation-related
//!   synchronization obligations.
//! - `crate::compiler::hybrid_pipeline` may adapt its legacy synchronization
//!   mode to this model at the integration boundary.
//! - `crate::runtime::hybrid_kernel` and future runtime adapters realize
//!   these requirements without changing their meaning.
//! - `quantum::ir` remains the canonical quantum semantic boundary.
//!
//! ## Semantics
//!
//! `SynchronizationKind` describes WHAT must be guaranteed.
//! `ExecutionMode` describes HOW the computation is permitted to progress
//! at the abstract boundary. Neither selects a physical implementation.
//!
//! An asynchronous operation does not imply that its result is available.
//! A completion requirement does not imply that a particular thread blocks.
//! A barrier declaration does not imply a hardware barrier.
//! An ordering guarantee does not authorize reordering unrelated operations.
//!
//! The dependency graph and the canonical concurrency/memory model determine
//! causal relationships and observable ordering. This module records
//! synchronization intent and validates its structural references.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on synchronization events or participants.
//! Collections grow according to available resources.
//!
//! The model accepts an explicit event identity from the caller. Identity
//! allocation is delegated to `IdAllocator`, which reports exhaustion rather
//! than wrapping or reusing an identifier.
//!
//! A finite identifier space and finite machine resources remain real limits.
//! They are not language-level hardware ceilings.
//!
//! This module does not allocate execution resources. Operational budgets
//! must be enforced by the caller through the repository's configured
//! policies.
//!
//! ## Determinism
//!
//! Events are stored in a `BTreeMap`, providing deterministic iteration by
//! event identity. Participant order is preserved because a caller may use
//! it as provenance or presentation order; it must not automatically be
//! interpreted as execution order.
//!
//! ## Compatibility
//!
//! Serialized enum variants and struct fields form an interchange contract.
//! Changes to their representation require a deliberate schema-compatibility
//! decision in `hybrid::version` and `hybrid::interchange`.
//!
//! The compiler pipeline's existing `SynchronizationMode` remains available
//! for compatibility until a separate integration change migrates callers.
//!
//! ## Completion criteria
//!
//! - All event identities and participants belong to the model's graph.
//! - Duplicate event identities and duplicate participants are rejected.
//! - Event ordering is deterministic.
//! - No physical synchronization mechanism is selected.
//! - Invalid input returns structured errors rather than panicking.
//! - No unsafe Rust or arbitrary universal size ceiling is introduced.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{
    HybridGraphId,
    HybridNodeId,
    IdAllocator,
    IdError,
    SynchronizationEventId,
};

/// The semantic guarantee required at a hybrid synchronization boundary.
///
/// These variants describe distinct observable obligations. They are not
/// implementation primitives and must not be interpreted as device commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SynchronizationKind {
    /// A value or result produced by a participant must be available before
    /// a dependent consumer uses it.
    DataAvailability,

    /// A control decision must be available before the computation it
    /// governs can proceed past the relevant dependency.
    ControlAvailability,

    /// A participant must satisfy a declared completion condition before
    /// the dependent computation proceeds.
    Completion,

    /// Observable execution order required by the canonical semantics
    /// must be preserved.
    Ordering,

    /// Writes or results must be observable by the participants covered by
    /// the applicable language memory and visibility model.
    Visibility,

    /// Ownership or access responsibility must be transferred according to
    /// the canonical ownership and lifetime contract.
    OwnershipTransfer,

    /// A resource-release obligation must be satisfied before a dependent
    /// computation relies on that resource being released.
    ResourceRelease,

    /// Participating computations must reach the declared abstract
    /// coordination point before execution continues beyond it.
    ///
    /// This is a semantic barrier requirement, not a hardware barrier.
    Barrier,
}

impl SynchronizationKind {
    /// Returns the stable textual identifier used for diagnostics and
    /// integrations. This is not a source-language parser.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataAvailability => "data_availability",
            Self::ControlAvailability => "control_availability",
            Self::Completion => "completion",
            Self::Ordering => "ordering",
            Self::Visibility => "visibility",
            Self::OwnershipTransfer => "ownership_transfer",
            Self::ResourceRelease => "resource_release",
            Self::Barrier => "barrier",
        }
    }
}

/// Abstract execution intent at a synchronization boundary.
///
/// This enum describes permitted progress and coordination semantics. It
/// does not select a worker, thread, device, queue, or physical mechanism.
///
/// In particular, `Asynchronous` does not imply that a result is ready,
/// that execution is detached, or that cancellation is impossible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionMode {
    /// The dependent operation must observe the completion condition before
    /// proceeding beyond this boundary.
    Synchronous,

    /// The operation may progress asynchronously. Dependencies and explicit
    /// completion requirements still determine when results can be consumed.
    Asynchronous,

    /// The declared participants must satisfy an abstract barrier contract.
    ///
    /// This does not mandate a hardware or thread-level barrier.
    Barrier,
}

impl ExecutionMode {
    /// Returns a stable identifier suitable for diagnostics and manifests.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Synchronous => "synchronous",
            Self::Asynchronous => "asynchronous",
            Self::Barrier => "barrier",
        }
    }
}

/// The abstract extent of a synchronization declaration.
///
/// Scope describes which declared participants are covered. It does not
/// identify physical machines, devices, networks, or runtime sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SynchronizationScope {
    /// A coordination obligation associated with one logical participant.
    ///
    /// This scope is useful for recording local visibility, release, or
    /// completion obligations. It does not imply inter-thread ordering.
    Participant,

    /// A coordination obligation across participants associated with a
    /// logical domain boundary.
    Domain,

    /// A coordination obligation across participants in one hybrid graph.
    HybridGraph,

    /// A coordination obligation that may span independently realized
    /// execution locations, including distributed implementations.
    ///
    /// No network protocol or consistency model is implied by this variant.
    Distributed,
}

impl SynchronizationScope {
    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Participant => "participant",
            Self::Domain => "domain",
            Self::HybridGraph => "hybrid_graph",
            Self::Distributed => "distributed",
        }
    }
}

/// One target-independent synchronization declaration.
///
/// The event records the synchronization requirement and its participants.
/// It does not contain runtime state, dependency edges, physical handles,
/// completion futures, or domain operation payloads.
///
/// `participants` are references to canonical graph nodes. Their actual
/// operations remain owned by the corresponding domain IRs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynchronizationEvent {
    /// Unique, graph-scoped identity of this event.
    pub id: SynchronizationEventId,

    /// Semantic guarantee required by this event.
    pub kind: SynchronizationKind,

    /// Permitted progress and coordination intent.
    pub mode: ExecutionMode,

    /// Abstract extent of the synchronization obligation.
    pub scope: SynchronizationScope,

    /// Graph nodes covered by this declaration.
    ///
    /// Declaration order is preserved. It is not an execution schedule.
    pub participants: Vec<HybridNodeId>,
}

impl SynchronizationEvent {
    /// Constructs an event value.
    ///
    /// This constructor validates only the event's local structure. The
    /// owning model must still validate graph identity and duplicate
    /// participants before accepting the event.
    #[must_use]
    pub fn new(
        id: SynchronizationEventId,
        kind: SynchronizationKind,
        mode: ExecutionMode,
        scope: SynchronizationScope,
        participants: Vec<HybridNodeId>,
    ) -> Self {
        Self {
            id,
            kind,
            mode,
            scope,
            participants,
        }
    }

    /// Returns whether the event's declared scope is local to one participant.
    #[must_use]
    pub const fn is_participant_local(&self) -> bool {
        matches!(self.scope, SynchronizationScope::Participant)
    }

    /// Returns whether this event explicitly requires barrier semantics.
    #[must_use]
    pub const fn requires_barrier(&self) -> bool {
        matches!(self.kind, SynchronizationKind::Barrier)
            || matches!(self.mode, ExecutionMode::Barrier)
    }
}

/// The canonical collection of synchronization declarations for one graph.
///
/// This is a declaration model, not a runtime synchronization engine.
/// Dependency relationships remain owned by `graph::edges`; this container
/// must not be used as a second dependency graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynchronizationModel {
    /// Graph whose nodes and synchronization events are referenced.
    pub graph_id: HybridGraphId,

    /// Events indexed by their stable, graph-scoped identity.
    ///
    /// `BTreeMap` gives deterministic iteration independent of insertion
    /// order. It does not imply that event identity determines execution
    /// order.
    events: BTreeMap<SynchronizationEventId, SynchronizationEvent>,
}

impl SynchronizationModel {
    /// Creates an empty synchronization model for a graph.
    #[must_use]
    pub fn new(graph_id: HybridGraphId) -> Self {
        Self {
            graph_id,
            events: BTreeMap::new(),
        }
    }

    /// Creates an event identity using the repository's shared allocator,
    /// constructs the declaration, and inserts it after validation.
    ///
    /// Callers should share an `IdAllocator` where identity uniqueness across
    /// concurrently constructed graph entities is required.
    pub fn declare(
        &mut self,
        allocator: &IdAllocator,
        kind: SynchronizationKind,
        mode: ExecutionMode,
        scope: SynchronizationScope,
        participants: Vec<HybridNodeId>,
    ) -> Result<SynchronizationEventId, HybridError> {
        let id = allocator
            .allocate_synchronization_event_id(self.graph_id)
            .map_err(map_id_error)?;

        let event = SynchronizationEvent::new(
            id,
            kind,
            mode,
            scope,
            participants,
        );

        self.insert(event)?;
        Ok(id)
    }

    /// Inserts a previously constructed event after validating all structural
    /// invariants. An invalid event never mutates the model.
    pub fn insert(
        &mut self,
        event: SynchronizationEvent,
    ) -> Result<(), HybridError> {
        self.validate_event(&event)?;

        if self.events.contains_key(&event.id) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!(
                    "synchronization event identity '{}' already exists",
                    event.id
                ),
            ));
        }

        self.events.insert(event.id, event);
        Ok(())
    }

    /// Returns an event by identity.
    #[must_use]
    pub fn get(
        &self,
        id: SynchronizationEventId,
    ) -> Option<&SynchronizationEvent> {
        self.events.get(&id)
    }

    /// Returns the events in deterministic identity order.
    #[must_use]
    pub fn events(
        &self,
    ) -> impl ExactSizeIterator<Item = &SynchronizationEvent> {
        self.events.values()
    }

    /// Returns the number of declared synchronization events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the model contains no events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Removes an event by identity.
    ///
    /// This operation removes only the declaration. The caller must ensure
    /// that no external dependency or contract still refers to the event.
    pub fn remove(
        &mut self,
        id: SynchronizationEventId,
    ) -> Option<SynchronizationEvent> {
        self.events.remove(&id)
    }

    /// Validates every stored event.
    ///
    /// This method checks structural invariants only. It does not prove that
    /// ordering, visibility, completion, ownership, or resource-release
    /// obligations are satisfied by a proposed execution plan.
    pub fn validate(&self) -> Result<(), HybridError> {
        for (key, event) in &self.events {
            if *key != event.id {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidRepresentation,
                    format!(
                        "synchronization map key '{}' does not match event identity '{}'",
                        key, event.id
                    ),
                ));
            }

            self.validate_event(event)?;
        }

        Ok(())
    }

    fn validate_event(
        &self,
        event: &SynchronizationEvent,
    ) -> Result<(), HybridError> {
        if event.id.graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "synchronization event '{}' belongs to graph '{}', not graph '{}'",
                event.id,
                event.id.graph().get(),
                self.graph_id.get()
            )));
        }

        if event.participants.is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                format!(
                    "synchronization event '{}' must reference at least one participant",
                    event.id
                ),
            ));
        }

        let mut seen = std::collections::BTreeSet::new();

        for participant in &event.participants {
            if participant.graph() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "participant '{}' of synchronization event '{}' belongs to a different graph",
                    participant, event.id
                )));
            }

            if !seen.insert(*participant) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "participant '{}' occurs more than once in synchronization event '{}'",
                        participant, event.id
                    ),
                ));
            }
        }

        if event.requires_barrier()
            && event.scope == SynchronizationScope::Participant
            && event.participants.len() > 1
        {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                format!(
                    "barrier event '{}' cannot have participant-local scope with multiple participants",
                    event.id
                ),
            ));
        }

        if event.scope == SynchronizationScope::Participant
            && event.participants.len() != 1
        {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                format!(
                    "participant-local synchronization event '{}' must reference exactly one participant",
                    event.id
                ),
            ));
        }

        Ok(())
    }
}

fn map_id_error(error: IdError) -> HybridError {
    HybridError::new(
        HybridErrorCode::InternalInvariantViolation,
        format!(
            "could not allocate synchronization event identity: {error}"
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{GraphScopedId, HybridGraphId, HybridNodeId, IdComponent};

    fn graph_id(value: u128) -> HybridGraphId {
        HybridGraphId::new(value).expect("test graph identity must be non-zero")
    }

    fn node_id(graph: HybridGraphId, local: u128) -> HybridNodeId {
        HybridNodeId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("test node identity must be non-zero"),
        ))
    }

    fn event_id(graph: HybridGraphId, local: u128) -> SynchronizationEventId {
        SynchronizationEventId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("test event identity must be non-zero"),
        ))
    }

    #[test]
    fn creates_an_empty_model() {
        let model = SynchronizationModel::new(graph_id(1));

        assert!(model.is_empty());
        assert_eq!(model.len(), 0);
        assert!(model.validate().is_ok());
    }

    #[test]
    fn inserts_and_retrieves_a_valid_event() {
        let graph = graph_id(1);
        let node = node_id(graph, 2);
        let id = event_id(graph, 3);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            id,
            SynchronizationKind::Completion,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node],
        );

        assert!(model.insert(event).is_ok());
        assert_eq!(model.len(), 1);
        assert_eq!(model.get(id).map(|value| value.id), Some(id));
        assert!(model.validate().is_ok());
    }

    #[test]
    fn rejects_an_event_from_another_graph() {
        let graph = graph_id(1);
        let other_graph = graph_id(2);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(other_graph, 3),
            SynchronizationKind::Ordering,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node_id(other_graph, 4)],
        );

        assert!(model.insert(event).is_err());
        assert!(model.is_empty());
    }

    #[test]
    fn rejects_participants_from_another_graph() {
        let graph = graph_id(1);
        let other_graph = graph_id(2);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(graph, 3),
            SynchronizationKind::Completion,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node_id(other_graph, 4)],
        );

        assert!(model.insert(event).is_err());
        assert!(model.is_empty());
    }

    #[test]
    fn rejects_empty_participant_lists() {
        let graph = graph_id(1);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(graph, 2),
            SynchronizationKind::Barrier,
            ExecutionMode::Barrier,
            SynchronizationScope::HybridGraph,
            Vec::new(),
        );

        assert!(model.insert(event).is_err());
    }

    #[test]
    fn rejects_duplicate_participants() {
        let graph = graph_id(1);
        let node = node_id(graph, 2);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(graph, 3),
            SynchronizationKind::Ordering,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node, node],
        );

        assert!(model.insert(event).is_err());
    }

    #[test]
    fn rejects_duplicate_event_identity() {
        let graph = graph_id(1);
        let node = node_id(graph, 2);
        let id = event_id(graph, 3);
        let mut model = SynchronizationModel::new(graph);

        let first = SynchronizationEvent::new(
            id,
            SynchronizationKind::Completion,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node],
        );

        let second = SynchronizationEvent::new(
            id,
            SynchronizationKind::Ordering,
            ExecutionMode::Synchronous,
            SynchronizationScope::HybridGraph,
            vec![node],
        );

        assert!(model.insert(first).is_ok());
        assert!(model.insert(second).is_err());
        assert_eq!(model.len(), 1);
    }

    #[test]
    fn participant_local_scope_requires_one_participant() {
        let graph = graph_id(1);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(graph, 3),
            SynchronizationKind::Visibility,
            ExecutionMode::Synchronous,
            SynchronizationScope::Participant,
            vec![node_id(graph, 4), node_id(graph, 5)],
        );

        assert!(model.insert(event).is_err());
    }

    #[test]
    fn barrier_is_semantic_not_a_physical_mechanism() {
        let graph = graph_id(1);
        let mut model = SynchronizationModel::new(graph);

        let event = SynchronizationEvent::new(
            event_id(graph, 3),
            SynchronizationKind::Barrier,
            ExecutionMode::Barrier,
            SynchronizationScope::HybridGraph,
            vec![node_id(graph, 4), node_id(graph, 5)],
        );

        assert!(model.insert(event).is_ok());
        assert!(model
            .get(event_id(graph, 3))
            .expect("event was inserted")
            .requires_barrier());
    }

    #[test]
    fn iteration_order_is_deterministic_by_event_identity() {
        let graph = graph_id(1);
        let mut model = SynchronizationModel::new(graph);

        for local in [9, 3, 7] {
            let event = SynchronizationEvent::new(
                event_id(graph, local),
                SynchronizationKind::Ordering,
                ExecutionMode::Synchronous,
                SynchronizationScope::HybridGraph,
                vec![node_id(graph, local + 100)],
            );

            model.insert(event).expect("event should be valid");
        }

        let ids: Vec<_> = model.events().map(|event| event.id).collect();

        assert_eq!(
            ids,
            vec![
                event_id(graph, 3),
                event_id(graph, 7),
                event_id(graph, 9),
            ]
        );
    }

    #[test]
    fn removes_an_existing_event() {
        let graph = graph_id(1);
        let node = node_id(graph, 2);
        let id = event_id(graph, 3);
        let mut model = SynchronizationModel::new(graph);

        model
            .insert(SynchronizationEvent::new(
                id,
                SynchronizationKind::ResourceRelease,
                ExecutionMode::Synchronous,
                SynchronizationScope::Participant,
                vec![node],
            ))
            .expect("event should be valid");

        assert!(model.remove(id).is_some());
        assert!(model.get(id).is_none());
        assert!(model.is_empty());
    }

    #[test]
    fn allocator_declares_events_in_the_owning_graph() {
        let allocator = IdAllocator::new();
        let graph = allocator.allocate_graph_id().expect("graph ID");
        let node = allocator
            .allocate_node_id(graph)
            .expect("node ID");
        let mut model = SynchronizationModel::new(graph);

        let id = model
            .declare(
                &allocator,
                SynchronizationKind::Completion,
                ExecutionMode::Asynchronous,
                SynchronizationScope::HybridGraph,
                vec![node],
            )
            .expect("event declaration should succeed");

        assert_eq!(id.graph(), graph);
        assert!(model.get(id).is_some());
    }
}