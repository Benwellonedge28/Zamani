//! # Zamani Hybrid Computing — Completion Semantics
//!
//! Path: `src/hybrid/sync/completion.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only. Unsafe code is forbidden.
//!
//! ## Purpose
//!
//! Represent the lifecycle and result-availability state of a hybrid
//! synchronization event. This module is concerned with logical completion,
//! not with executing work or implementing a scheduler.
//!
//! A submitted operation is not necessarily running, and a running
//! operation is not necessarily complete. Consumers must not treat a
//! result as available until the corresponding operation reaches `Completed`.
//!
//! ## Canonical ownership
//!
//! This module owns:
//! - The completion lifecycle state vocabulary.
//! - Completion records associated with existing synchronization-event IDs.
//! - Validation of legal lifecycle transitions.
//! - Explicit failure information associated with failed completion.
//! - A graph-scoped, in-memory completion tracker.
//!
//! This module does not own:
//! - Synchronization-event identity (`super::super::ids`).
//! - The canonical hybrid error vocabulary (`super::super::errors`).
//! - Dependency or edge definitions (`super::super::graph::edges`).
//! - Source syntax, AST, semantic analysis, or a second IR.
//! - Quantum measurement, classical execution, or domain operation semantics.
//! - Scheduling, target selection, physical synchronization, or resource
//!   allocation.
//! - Runtime submission, polling, blocking, cancellation, or device sessions.
//! - The result value itself. Result values remain owned by their canonical
//!   domain or cross-domain data representation.
//!
//! ## Lifecycle contract
//!
//! The permitted transitions are:
//!
//! ```text
//! Pending ──> Submitted ──> Running ──> Completed
//!    │             │           │
//!    ├─────────────┼───────────┼──────> Failed
//!    │             │           │
//!    └─────────────┴───────────┴──────> Cancelled
//!
//! Submitted ─────────────────────────> Completed
//! ```
//!
//! `Pending` represents a registered operation not yet submitted.
//! `Submitted` means submission has been accepted by the responsible
//! execution layer; it does not prove that execution has started.
//! `Running` means the execution layer reports that work has started.
//! `Completed` means the operation has completed successfully and its
//! result is logically available under the operation's result contract.
//! `Failed` means execution or submission failed.
//! `Cancelled` means the operation was cancelled before successful
//! completion.
//!
//! `Completed`, `Failed`, and `Cancelled` are terminal states. A terminal
//! state cannot be changed by this tracker. Retries must use a separately
//! identified execution attempt or synchronization event, according to the
//! owning runtime/graph contract.
//!
//! Direct `Submitted -> Completed` is permitted for execution systems that
//! can establish successful completion without reporting a distinct
//! `Running` state. The tracker does not invent intermediate observations.
//!
//! ## Error and consistency contract
//!
//! - An event must belong to the tracker's graph.
//! - An event can be registered only once in a tracker.
//! - Every state change must follow the lifecycle transition table.
//! - `Failed` requires structured failure information.
//! - Non-failed states must not carry failure information.
//! - State revisions increase with every accepted transition.
//! - Revision overflow is reported rather than wrapped.
//! - Invalid transitions leave the record unchanged.
//! - Results are available only in the `Completed` state.
//!
//! This module uses `BTreeMap` to provide deterministic iteration order.
//! It does not impose an arbitrary maximum number of completion records.
//! Actual memory and allocation limits remain operational constraints.
//!
//! ## Concurrency contract
//!
//! `CompletionTracker` is a mutable, in-memory model. It does not itself
//! provide concurrent mutation or distributed consistency. A runtime that
//! shares a tracker across threads must provide synchronization around it;
//! distributed runtimes must establish their own authoritative event and
//! state-update protocol.
//!
//! The tracker records observations supplied by its caller. It does not
//! poll devices, wait for futures, block a thread, or execute cancellation.
//!
//! ## Integration
//!
//! - Uses `SynchronizationEventId` from `super::super::ids`.
//! - Uses `HybridError` and `HybridErrorCode` from `super::super::errors`.
//! - `sync/model.rs` associates declared synchronization events with their
//!   completion obligations.
//! - `sync/ordering.rs` consumes completion observations when evaluating
//!   ordering requirements.
//! - `sync/cancellation.rs` requests cancellation from the runtime and
//!   records cancellation only after that outcome is established.
//! - `graph/edges.rs` remains the authority for graph dependency edges.
//! - `compiler/hybrid_pipeline.rs` remains the owner of its existing
//!   `SynchronizationMode` until a deliberate compatibility-preserving
//!   integration replaces or adapts it.
//! - `runtime/hybrid_kernel.rs` remains responsible for actual execution.
//!   Its current implementation is a logging-oriented stub and does not
//!   yet provide authoritative lifecycle observations.
//! - `grammar/spec/hybrid.md`, `grammar/spec/concurrency.md`, and
//!   `grammar/hybrid/synchronization.g4` define the upstream semantics.
//!   This file implements lifecycle bookkeeping, not grammar rules.
//!
//! ## Completion criteria
//!
//! This module is complete when legal transitions, terminal-state behavior,
//! graph ownership, failure consistency, result availability, revision
//! overflow, and deterministic iteration are tested, and the parent modules
//! expose it through the canonical hybrid module tree.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::errors::{HybridError, HybridErrorCode};
use super::super::ids::{HybridGraphId, SynchronizationEventId};

/// The logical lifecycle state of one hybrid operation.
///
/// These states describe observed lifecycle facts. They do not prescribe
/// a particular executor, physical resource, or scheduling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CompletionStatus {
    /// Registered but not yet submitted to an execution layer.
    Pending,

    /// Accepted for execution; actual execution may not have started.
    Submitted,

    /// The execution layer reports that the operation has started.
    Running,

    /// Successful completion has been established.
    Completed,

    /// Submission or execution failed.
    Failed,

    /// Cancellation has been established.
    Cancelled,
}

impl CompletionStatus {
    /// Returns whether this state is terminal.
    ///
    /// Terminal states cannot transition to another state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// Returns whether the operation's result is available under this
    /// lifecycle contract.
    ///
    /// This is a lifecycle guarantee, not a guarantee that a result payload
    /// exists for operations whose canonical contract has no result.
    #[must_use]
    pub const fn result_is_available(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether a transition to `next` is permitted by the lifecycle
    /// contract.
    #[must_use]
    pub const fn permits_transition_to(self, next: Self) -> bool {
        match self {
            Self::Pending => {
                matches!(next, Self::Submitted | Self::Failed | Self::Cancelled)
            }
            Self::Submitted => {
                matches!(
                    next,
                    Self::Running | Self::Completed | Self::Failed | Self::Cancelled
                )
            }
            Self::Running => {
                matches!(next, Self::Completed | Self::Failed | Self::Cancelled)
            }
            Self::Completed | Self::Failed | Self::Cancelled => false,
        }
    }
}

/// Structured failure information attached to a failed completion.
///
/// `code` must be a stable, machine-readable identifier supplied by the
/// owning subsystem. `message` is explanatory text and must not be parsed
/// to identify the failure category.
///
/// This type intentionally does not own or replace `HybridError`. Use
/// [`CompletionFailure::from_hybrid_error`] when adapting a hybrid error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionFailure {
    code: String,
    message: String,
}

impl CompletionFailure {
    /// Constructs failure information from a stable code and message.
    ///
    /// Empty or whitespace-only values are rejected. No fixed maximum string
    /// length is imposed here; deployments may apply explicit metadata
    /// budgets at their input-validation boundary.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let code = code.into();
        let message = message.into();

        if code.trim().is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "completion failure code must not be empty",
            ));
        }

        if message.trim().is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "completion failure message must not be empty",
            ));
        }

        Ok(Self { code, message })
    }

    /// Adapts the stable code and message from the canonical hybrid error.
    pub fn from_hybrid_error(error: &HybridError) -> Self {
        Self {
            code: error.code_str().to_owned(),
            message: error.message().to_owned(),
        }
    }

    /// Returns the stable machine-readable failure code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the explanatory failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Validates the value after deserialization or external construction.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.code.trim().is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "completion failure code must not be empty",
            ));
        }

        if self.message.trim().is_empty() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "completion failure message must not be empty",
            ));
        }

        Ok(())
    }
}

/// A snapshot of one synchronization event's completion lifecycle.
///
/// `revision` starts at zero when the event is registered and increases
/// after each accepted transition. It is an update revision, not a timestamp
/// and not a globally unique event identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRecord {
    event_id: SynchronizationEventId,
    status: CompletionStatus,
    revision: u128,
    failure: Option<CompletionFailure>,
}

impl CompletionRecord {
    fn new(event_id: SynchronizationEventId) -> Self {
        Self {
            event_id,
            status: CompletionStatus::Pending,
            revision: 0,
            failure: None,
        }
    }

    /// Returns the synchronization event associated with this record.
    #[must_use]
    pub const fn event_id(&self) -> SynchronizationEventId {
        self.event_id
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn status(&self) -> CompletionStatus {
        self.status
    }

    /// Returns the current state revision.
    #[must_use]
    pub const fn revision(&self) -> u128 {
        self.revision
    }

    /// Returns failure information when the state is `Failed`.
    #[must_use]
    pub fn failure(&self) -> Option<&CompletionFailure> {
        self.failure.as_ref()
    }

    /// Returns whether successful completion has made the result available.
    #[must_use]
    pub const fn result_is_available(&self) -> bool {
        self.status.result_is_available()
    }

    /// Validates invariants that must also hold for deserialized records.
    pub fn validate(&self) -> Result<(), HybridError> {
        match (self.status, self.failure.as_ref()) {
            (CompletionStatus::Failed, Some(failure)) => failure.validate(),
            (CompletionStatus::Failed, None) => Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "failed completion must include failure information",
            )),
            (_, Some(_)) => Err(HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "failure information is only valid for failed completion",
            )),
            (_, None) => Ok(()),
        }
    }

    fn transition(
        &mut self,
        next: CompletionStatus,
        failure: Option<CompletionFailure>,
    ) -> Result<(), HybridError> {
        if !self.status.permits_transition_to(next) {
            return Err(HybridError::new(
                HybridErrorCode::SynchronizationFailure,
                format!(
                    "invalid completion transition for event {}: {:?} -> {:?}",
                    self.event_id, self.status, next
                ),
            ));
        }

        match (next, failure.as_ref()) {
            (CompletionStatus::Failed, Some(value)) => value.validate()?,
            (CompletionStatus::Failed, None) => {
                return Err(HybridError::new(
                    HybridErrorCode::CompletionFailure,
                    "failed completion requires structured failure information",
                ));
            }
            (_, Some(_)) => {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidRepresentation,
                    "failure information may only accompany a failed transition",
                ));
            }
            (_, None) => {}
        }

        let next_revision = self.revision.checked_add(1).ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::CompletionFailure,
                format!(
                    "completion revision exhausted for event {}",
                    self.event_id
                ),
            )
        })?;

        // Mutate only after every validation has succeeded.
        self.status = next;
        self.revision = next_revision;
        self.failure = failure;

        Ok(())
    }
}

/// Graph-scoped completion bookkeeping for hybrid synchronization events.
///
/// The tracker does not allocate event IDs, create graph nodes, or execute
/// operations. The caller supplies event IDs from the canonical hybrid ID
/// allocator and updates lifecycle state when the responsible execution
/// layer reports an observation.
///
/// Iteration follows the natural ordering of `SynchronizationEventId`.
/// Mutation requires exclusive access to the tracker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionTracker {
    graph_id: HybridGraphId,
    records: BTreeMap<SynchronizationEventId, CompletionRecord>,
}

impl CompletionTracker {
    /// Creates an empty tracker for one hybrid graph.
    #[must_use]
    pub fn new(graph_id: HybridGraphId) -> Self {
        Self {
            graph_id,
            records: BTreeMap::new(),
        }
    }

    /// Returns the graph identity owned by this tracker.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Registers an event in the initial `Pending` state.
    ///
    /// The event must belong to this tracker's graph and must not already
    /// have a record in this tracker.
    pub fn register(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.validate_event_graph(event_id)?;

        if self.records.contains_key(&event_id) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!("completion record already exists for event {event_id}"),
            ));
        }

        self.records
            .insert(event_id, CompletionRecord::new(event_id));

        Ok(())
    }

    /// Returns the completion record for an event, if registered.
    #[must_use]
    pub fn get(
        &self,
        event_id: SynchronizationEventId,
    ) -> Option<&CompletionRecord> {
        self.records.get(&event_id)
    }

    /// Returns the number of registered completion records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the tracker contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Iterates over records in deterministic event-ID order.
    pub fn records(
        &self,
    ) -> impl ExactSizeIterator<Item = &CompletionRecord> {
        self.records.values()
    }

    /// Records a lifecycle transition.
    ///
    /// A failure value is required exactly when `next` is `Failed`.
    /// An unknown event, an event from another graph, an invalid transition,
    /// or an inconsistent failure value produces an error.
    ///
    /// On error, the existing record remains unchanged.
    pub fn transition(
        &mut self,
        event_id: SynchronizationEventId,
        next: CompletionStatus,
        failure: Option<CompletionFailure>,
    ) -> Result<(), HybridError> {
        self.validate_event_graph(event_id)?;

        let record = self.records.get_mut(&event_id).ok_or_else(|| {
            HybridError::invalid_reference(format!(
                "no completion record is registered for event {event_id}"
            ))
        })?;

        record.transition(next, failure)
    }

    /// Records successful submission.
    pub fn mark_submitted(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CompletionStatus::Submitted, None)
    }

    /// Records that execution has started.
    pub fn mark_running(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CompletionStatus::Running, None)
    }

    /// Records successful completion.
    ///
    /// This does not create or transfer a result payload. The owning domain
    /// contract remains responsible for the result's representation and
    /// lifetime.
    pub fn mark_completed(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CompletionStatus::Completed, None)
    }

    /// Records a failure reported by the responsible subsystem.
    pub fn mark_failed(
        &mut self,
        event_id: SynchronizationEventId,
        failure: CompletionFailure,
    ) -> Result<(), HybridError> {
        self.transition(
            event_id,
            CompletionStatus::Failed,
            Some(failure),
        )
    }

    /// Records cancellation after the responsible execution layer confirms
    /// that the operation has been cancelled.
    ///
    /// Calling this method does not itself issue a cancellation request.
    pub fn mark_cancelled(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CompletionStatus::Cancelled, None)
    }

    fn validate_event_graph(
        &self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        if event_id.graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "event {event_id} belongs to graph {}, not tracker graph {}",
                event_id.graph(),
                self.graph_id
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::IdAllocator;

    fn fixture() -> (IdAllocator, HybridGraphId, SynchronizationEventId) {
        let allocator = IdAllocator::new();
        let graph_id = allocator.allocate_graph_id().unwrap();
        let event_id = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();

        (allocator, graph_id, event_id)
    }

    fn failure() -> CompletionFailure {
        CompletionFailure::new("HYB034", "execution failed").unwrap()
    }

    #[test]
    fn registered_event_starts_pending_and_has_no_result() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();

        let record = tracker.get(event_id).unwrap();
        assert_eq!(record.status(), CompletionStatus::Pending);
        assert_eq!(record.revision(), 0);
        assert!(!record.result_is_available());
        assert_eq!(record.failure(), None);
    }

    #[test]
    fn successful_lifecycle_makes_result_available_only_at_completion() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();

        assert!(!tracker.get(event_id).unwrap().result_is_available());

        tracker.mark_running(event_id).unwrap();
        assert!(!tracker.get(event_id).unwrap().result_is_available());

        tracker.mark_completed(event_id).unwrap();

        let record = tracker.get(event_id).unwrap();
        assert_eq!(record.status(), CompletionStatus::Completed);
        assert_eq!(record.revision(), 3);
        assert!(record.result_is_available());
        assert!(record.status().is_terminal());
    }

    #[test]
    fn direct_submitted_to_completed_is_permitted() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();
        tracker.mark_completed(event_id).unwrap();

        assert!(tracker.get(event_id).unwrap().result_is_available());
    }

    #[test]
    fn failed_completion_requires_failure_information() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();

        let error = tracker
            .transition(event_id, CompletionStatus::Failed, None)
            .unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::CompletionFailure);
        assert_eq!(
            tracker.get(event_id).unwrap().status(),
            CompletionStatus::Submitted
        );

        tracker.mark_failed(event_id, failure()).unwrap();

        let record = tracker.get(event_id).unwrap();
        assert_eq!(record.status(), CompletionStatus::Failed);
        assert_eq!(record.failure().unwrap().code(), "HYB034");
        assert!(!record.result_is_available());
    }

    #[test]
    fn cancellation_does_not_claim_successful_completion() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();
        tracker.mark_cancelled(event_id).unwrap();

        let record = tracker.get(event_id).unwrap();
        assert_eq!(record.status(), CompletionStatus::Cancelled);
        assert!(record.status().is_terminal());
        assert!(!record.result_is_available());
    }

    #[test]
    fn terminal_states_cannot_be_rewritten() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();
        tracker.mark_completed(event_id).unwrap();

        let before = tracker.get(event_id).unwrap().clone();
        let error = tracker.mark_cancelled(event_id).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::SynchronizationFailure);
        assert_eq!(*tracker.get(event_id).unwrap(), before);
    }

    #[test]
    fn cannot_skip_submission_or_start_from_pending() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();

        assert!(tracker.mark_running(event_id).is_err());
        assert_eq!(
            tracker.get(event_id).unwrap().status(),
            CompletionStatus::Pending
        );
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        let error = tracker.register(event_id).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::DuplicateIdentity);
        assert_eq!(tracker.len(), 1);
    }

    #[test]
    fn foreign_graph_events_are_rejected() {
        let allocator = IdAllocator::new();
        let graph_a = allocator.allocate_graph_id().unwrap();
        let graph_b = allocator.allocate_graph_id().unwrap();
        let foreign_event = allocator
            .allocate_synchronization_event_id(graph_b)
            .unwrap();

        let mut tracker = CompletionTracker::new(graph_a);
        let error = tracker.register(foreign_event).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
        assert!(tracker.is_empty());
    }

    #[test]
    fn unknown_event_cannot_be_transitioned() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        let error = tracker.mark_submitted(event_id).unwrap_err();

        assert_eq!(error.code(), HybridErrorCode::InvalidReference);
        assert!(tracker.is_empty());
    }

    #[test]
    fn failure_information_is_validated() {
        assert!(CompletionFailure::new("", "message").is_err());
        assert!(CompletionFailure::new("HYB034", " ").is_err());

        let valid = CompletionFailure::new("HYB034", "failed").unwrap();
        assert_eq!(valid.code(), "HYB034");
    }

    #[test]
    fn records_iterate_in_deterministic_id_order() {
        let allocator = IdAllocator::new();
        let graph_id = allocator.allocate_graph_id().unwrap();
        let first = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();
        let second = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();

        let mut tracker = CompletionTracker::new(graph_id);
        tracker.register(second).unwrap();
        tracker.register(first).unwrap();

        let ids: Vec<_> = tracker.records().map(CompletionRecord::event_id).collect();

        assert_eq!(ids, vec![first, second]);
    }

    #[test]
    fn failure_record_round_trips_through_serde_json() {
        let (_, graph_id, event_id) = fixture();
        let mut tracker = CompletionTracker::new(graph_id);

        tracker.register(event_id).unwrap();
        tracker.mark_submitted(event_id).unwrap();
        tracker.mark_failed(event_id, failure()).unwrap();

        let record = tracker.get(event_id).unwrap();
        let encoded = serde_json::to_string(record).unwrap();
        let decoded: CompletionRecord = serde_json::from_str(&encoded).unwrap();

        assert_eq!(&decoded, record);
        decoded.validate().unwrap();
    }

    #[test]
    fn malformed_deserialized_record_is_rejected_by_validation() {
        let (_, _, event_id) = fixture();

        let malformed = CompletionRecord {
            event_id,
            status: CompletionStatus::Failed,
            revision: 1,
            failure: None,
        };

        assert!(malformed.validate().is_err());
    }
}