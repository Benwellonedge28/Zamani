
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Cancellation Contracts
//!
//! Path: `src/hybrid/sync/cancellation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Represent cancellation requests and their observed outcomes for
//! synchronization events in a hybrid computation.
//!
//! This module defines cancellation intent, propagation policy, request
//! lifecycle, validation, and deterministic in-memory bookkeeping. It does
//! not execute cancellation or claim that an operation has stopped merely
//! because a request was issued or accepted.
//!
//! ## Canonical ownership
//!
//! This module owns:
//! - Cancellation propagation policy at the hybrid boundary.
//! - Cancellation request records and their lifecycle.
//! - Structural validation of cancellation requests.
//! - A graph-scoped cancellation-request tracker.
//! - Explicit failure information associated with cancellation requests.
//!
//! This module does not own:
//! - Synchronization-event identity or graph identity.
//! - The canonical completion lifecycle.
//! - Dependency edges or graph traversal.
//! - Language ownership, borrowing, or lifetime checking.
//! - Domain operation semantics or domain IRs.
//! - Runtime execution, vendor jobs, device sessions, or hardware handles.
//! - Actual resource destruction or physical cancellation mechanisms.
//!
//! ## Semantic guarantees
//!
//! A cancellation request is not proof of cancellation.
//!
//! `Issued` means that the responsible execution integration reports that
//! it issued the request. `Confirmed` means the runtime or authoritative
//! execution owner has confirmed cancellation. It does not independently
//! update `CompletionTracker`; the runtime integration must reconcile the
//! observed outcome with the canonical completion lifecycle.
//!
//! If work has already completed, cancellation must not rewrite successful
//! completion as cancellation. A runtime must report the actual outcome.
//!
//! Partial results, cleanup, ownership release, and dependent-operation
//! handling remain governed by canonical semantic, ownership, completion,
//! and runtime contracts.
//!
//! ## Scalability
//!
//! No fixed maximum number of cancellation records or graph events is
//! imposed. Records use a growable, ordered collection. Allocation and
//! execution remain subject to actual resources and explicitly configured
//! operational policies.
//!
//! A finite identifier space is a representation constraint, not a
//! hardware-capacity limit or language-semantic restriction.
//!
//! ## Concurrency
//!
//! `CancellationTracker` is a mutable in-memory model, not a concurrent
//! or distributed cancellation service. Callers sharing it across threads
//! must synchronize access. Distributed runtimes must establish an
//! authoritative protocol for request identity, retries, and outcomes.
//!
//! ## Integration
//!
//! - `super::super::ids` owns graph-scoped event identities.
//! - `super::super::errors` owns structured hybrid errors.
//! - `super::super::data::ownership` owns `CancellationRequirement`.
//! - `super::completion` owns the canonical completion-state lifecycle.
//! - `super::model` owns synchronization declarations.
//! - `crate::hybrid::graph::edges` remains authoritative for dependencies.
//! - `crate::runtime::hybrid_kernel` must perform and report actual
//!   cancellation when a real execution implementation is available.
//! - `crate::compiler::hybrid_pipeline` retains its existing compatibility
//!   API and adapts synchronization intent at the integration boundary.
//! - The normative hybrid and concurrency specifications govern semantics.
//!
//! This file does not create a second completion state machine, dependency
//! graph, runtime engine, or physical resource manager.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::data::ownership::CancellationRequirement;
use super::super::errors::{HybridError, HybridErrorCode};
use super::super::ids::{HybridGraphId, SynchronizationEventId};

/// Describes how a cancellation request is allowed to propagate.
///
/// This is a policy declaration, not a graph traversal algorithm. The
/// integration layer must resolve the relevant canonical dependencies
/// and establish which operations are actually eligible for propagation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CancellationPropagation {
    /// Request cancellation only for the explicitly identified event.
    ///
    /// This does not cancel dependencies or dependents implicitly.
    None,

    /// Permit propagation to dependent operations where the canonical
    /// dependency and runtime contracts require or allow it.
    ///
    /// The graph and runtime integration must determine the affected events.
    DeclaredDependents,

    /// Permit propagation across the explicitly established cancellation
    /// scope, as determined by the authoritative caller.
    ///
    /// This does not mean every event in the graph is automatically
    /// cancelled, nor does it override ownership or failure policies.
    DeclaredScope,
}

/// The observed lifecycle of a cancellation request.
///
/// These states describe the request, not the operation's completion state.
/// In particular, `Issued` and `Failed` must not be confused with the
/// operation being cancelled or the operation having failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CancellationStatus {
    /// A cancellation request has been registered locally but not issued.
    Pending,

    /// The responsible execution integration reports that it issued
    /// the cancellation request.
    Issued,

    /// The execution owner has confirmed that cancellation took effect.
    Confirmed,

    /// The execution owner explicitly rejected the cancellation request.
    Rejected,

    /// The execution owner reports that cancellation was too late,
    /// for example because the operation had already completed.
    TooLate,

    /// The cancellation attempt failed without establishing cancellation.
    Failed,
}

impl CancellationStatus {
    /// Returns whether this request status is terminal.
    ///
    /// Terminal request outcomes cannot be overwritten. A later retry or
    /// a new execution attempt must be represented explicitly by the
    /// authoritative runtime protocol.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Confirmed | Self::Rejected | Self::TooLate | Self::Failed
        )
    }

    /// Returns whether the operation itself is known to have been cancelled.
    ///
    /// This reports the cancellation-request observation only. The runtime
    /// must still reconcile the outcome with `CompletionTracker`.
    #[must_use]
    pub const fn confirms_cancellation(self) -> bool {
        matches!(self, Self::Confirmed)
    }

    /// Returns whether a transition to `next` is permitted.
    #[must_use]
    pub const fn permits_transition_to(self, next: Self) -> bool {
        match self {
            Self::Pending => matches!(
                next,
                Self::Issued
                    | Self::Rejected
                    | Self::TooLate
                    | Self::Failed
            ),
            Self::Issued => matches!(
                next,
                Self::Confirmed
                    | Self::Rejected
                    | Self::TooLate
                    | Self::Failed
            ),
            Self::Confirmed
            | Self::Rejected
            | Self::TooLate
            | Self::Failed => false,
        }
    }
}

/// Structured information about a failed cancellation attempt.
///
/// The error code is a stable identifier supplied by the responsible
/// subsystem. The message is explanatory text and must not be parsed to
/// determine the failure category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationFailure {
    code: String,
    message: String,
}

impl CancellationFailure {
    /// Constructs validated cancellation failure information.
    ///
    /// There is no arbitrary fixed string-length limit. Deployments may
    /// enforce explicit metadata budgets at their input boundary.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            code: code.into(),
            message: message.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Adapts a canonical hybrid error without replacing its category.
    pub fn from_hybrid_error(error: &HybridError) -> Self {
        Self {
            code: error.code_str().to_owned(),
            message: error.message().to_owned(),
        }
    }

    /// Returns the stable machine-readable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Validates this value, including values obtained by deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.code.trim().is_empty() {
            return Err(invalid_representation(
                "cancellation failure code must not be empty",
            ));
        }

        if self.message.trim().is_empty() {
            return Err(invalid_representation(
                "cancellation failure message must not be empty",
            ));
        }

        Ok(())
    }
}

/// A validated cancellation intent for one synchronization event.
///
/// This is a declaration of intent. It contains no runtime job handle,
/// physical device identifier, or cancellation callback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationRequest {
    event_id: SynchronizationEventId,
    requirement: CancellationRequirement,
    propagation: CancellationPropagation,
    reason: Option<String>,
}

impl CancellationRequest {
    /// Constructs a cancellation request.
    ///
    /// `NotApplicable` is rejected because it explicitly declares that
    /// no cancellable asynchronous operation is present. `BestEffort`,
    /// `Propagate`, and `External` retain their meanings from the existing
    /// canonical ownership contract.
    pub fn new(
        event_id: SynchronizationEventId,
        requirement: CancellationRequirement,
        propagation: CancellationPropagation,
        reason: Option<String>,
    ) -> Result<Self, HybridError> {
        let request = Self {
            event_id,
            requirement,
            propagation,
            reason,
        };

        request.validate()?;
        Ok(request)
    }

    /// Returns the synchronization event targeted by this request.
    #[must_use]
    pub const fn event_id(&self) -> SynchronizationEventId {
        self.event_id
    }

    /// Returns the declared cancellation guarantee.
    #[must_use]
    pub const fn requirement(&self) -> CancellationRequirement {
        self.requirement
    }

    /// Returns the propagation policy.
    #[must_use]
    pub const fn propagation(&self) -> CancellationPropagation {
        self.propagation
    }

    /// Returns the optional explanatory reason.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Validates the request's local structure.
    ///
    /// This does not establish that the event exists, is cancellable at
    /// runtime, or has not already completed. Those facts belong to the
    /// canonical graph and execution authorities.
    pub fn validate(&self) -> Result<(), HybridError> {
        if matches!(
            self.requirement,
            CancellationRequirement::NotApplicable
        ) {
            return Err(HybridError::new(
                HybridErrorCode::CancellationFailure,
                "cannot request cancellation when the contract declares it not applicable",
            ));
        }

        if self
            .reason
            .as_deref()
            .is_some_and(|reason| reason.trim().is_empty())
        {
            return Err(invalid_representation(
                "cancellation reason must be non-empty when provided",
            ));
        }

        Ok(())
    }
}

/// A snapshot of the cancellation-request lifecycle for one event.
///
/// The revision begins at zero and increases after each accepted state
/// transition. It is a version counter, not a timestamp or event identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationRecord {
    request: CancellationRequest,
    status: CancellationStatus,
    revision: u128,
    failure: Option<CancellationFailure>,
}

impl CancellationRecord {
    fn new(request: CancellationRequest) -> Self {
        Self {
            request,
            status: CancellationStatus::Pending,
            revision: 0,
            failure: None,
        }
    }

    /// Returns the original cancellation request.
    #[must_use]
    pub fn request(&self) -> &CancellationRequest {
        &self.request
    }

    /// Returns the targeted event identity.
    #[must_use]
    pub const fn event_id(&self) -> SynchronizationEventId {
        self.request.event_id
    }

    /// Returns the observed request status.
    #[must_use]
    pub const fn status(&self) -> CancellationStatus {
        self.status
    }

    /// Returns the current state revision.
    #[must_use]
    pub const fn revision(&self) -> u128 {
        self.revision
    }

    /// Returns failure information when the request status is `Failed`.
    #[must_use]
    pub fn failure(&self) -> Option<&CancellationFailure> {
        self.failure.as_ref()
    }

    /// Validates invariants, including after deserialization.
    pub fn validate(&self) -> Result<(), HybridError> {
        self.request.validate()?;

        match (self.status, self.failure.as_ref()) {
            (CancellationStatus::Failed, Some(failure)) => failure.validate(),
            (CancellationStatus::Failed, None) => Err(invalid_representation(
                "failed cancellation request must include failure information",
            )),
            (_, Some(_)) => Err(invalid_representation(
                "cancellation failure information is only valid for failed requests",
            )),
            (_, None) => Ok(()),
        }
    }

    fn transition(
        &mut self,
        next: CancellationStatus,
        failure: Option<CancellationFailure>,
    ) -> Result<(), HybridError> {
        if !self.status.permits_transition_to(next) {
            return Err(HybridError::new(
                HybridErrorCode::CancellationFailure,
                format!(
                    "invalid cancellation-request transition for event {}: {:?} -> {:?}",
                    self.event_id(),
                    self.status,
                    next
                ),
            ));
        }

        match (next, failure.as_ref()) {
            (CancellationStatus::Failed, Some(value)) => value.validate()?,
            (CancellationStatus::Failed, None) => {
                return Err(HybridError::new(
                    HybridErrorCode::CancellationFailure,
                    "failed cancellation request requires failure information",
                ));
            }
            (_, Some(_)) => {
                return Err(invalid_representation(
                    "failure information is only permitted for a failed cancellation request",
                ));
            }
            (_, None) => {}
        }

        // Compute the revision before mutating the record. Overflow therefore
        // leaves the existing record unchanged.
        let revision = self.revision.checked_add(1).ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::InternalInvariantViolation,
                "cancellation-request revision counter exhausted",
            )
        })?;

        self.status = next;
        self.revision = revision;
        self.failure = failure;

        Ok(())
    }
}

/// Graph-scoped in-memory tracker for cancellation requests.
///
/// This type records requests and caller-reported outcomes. It does not
/// submit work, call vendor APIs, cancel runtime jobs, update completion
/// records, or release resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationTracker {
    graph_id: HybridGraphId,
    records: BTreeMap<SynchronizationEventId, CancellationRecord>,
}

impl CancellationTracker {
    /// Creates an empty tracker for the specified hybrid graph.
    #[must_use]
    pub fn new(graph_id: HybridGraphId) -> Self {
        Self {
            graph_id,
            records: BTreeMap::new(),
        }
    }

    /// Returns the graph whose event identities this tracker accepts.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Returns the number of tracked cancellation requests.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether this tracker contains no requests.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns records in deterministic event-identity order.
    ///
    /// Iteration order is not execution order.
    pub fn records(
        &self,
    ) -> impl Iterator<Item = &CancellationRecord> {
        self.records.values()
    }

    /// Looks up a cancellation record by event identity.
    ///
    /// An event from another graph is rejected rather than treated as
    /// an absent event.
    pub fn record(
        &self,
        event_id: SynchronizationEventId,
    ) -> Result<Option<&CancellationRecord>, HybridError> {
        self.validate_event_graph(event_id)?;
        Ok(self.records.get(&event_id))
    }

    /// Registers a cancellation request for an event.
    ///
    /// A single active record is retained per event identity. Duplicate
    /// requests are rejected; retry and idempotency policies belong to the
    /// runtime's authoritative request protocol and must not be inferred
    /// from this local tracker.
    pub fn request(
        &mut self,
        request: CancellationRequest,
    ) -> Result<&CancellationRecord, HybridError> {
        request.validate()?;
        self.validate_event_graph(request.event_id)?;

        let event_id = request.event_id;

        if self.records.contains_key(&event_id) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!(
                    "a cancellation request is already registered for event {event_id}"
                ),
            ));
        }

        self.records
            .insert(event_id, CancellationRecord::new(request));

        // The record was inserted above and cannot be removed by this
        // exclusive mutable borrow, so this lookup must succeed.
        self.records.get(&event_id).ok_or_else(|| {
            HybridError::internal_invariant_violation(
                "cancellation record disappeared immediately after insertion",
            )
        })
    }

    /// Records that the responsible integration issued the cancellation.
    pub fn mark_issued(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CancellationStatus::Issued, None)
    }

    /// Records cancellation confirmed by the authoritative execution owner.
    ///
    /// The runtime must separately reconcile this observation with the
    /// canonical completion tracker and ownership/resource-release contracts.
    pub fn confirm_cancelled(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CancellationStatus::Confirmed, None)
    }

    /// Records an explicit rejection by the execution owner.
    pub fn reject(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CancellationStatus::Rejected, None)
    }

    /// Records that cancellation could not take effect because it was too
    /// late, for example because execution had already completed.
    pub fn mark_too_late(
        &mut self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        self.transition(event_id, CancellationStatus::TooLate, None)
    }

    /// Records a cancellation-attempt failure.
    pub fn mark_failed(
        &mut self,
        event_id: SynchronizationEventId,
        failure: CancellationFailure,
    ) -> Result<(), HybridError> {
        self.transition(
            event_id,
            CancellationStatus::Failed,
            Some(failure),
        )
    }

    /// Returns whether a cancellation outcome has been confirmed.
    ///
    /// `false` means only that this tracker has not recorded confirmation;
    /// it does not prove that the underlying operation is still running.
    pub fn cancellation_confirmed(
        &self,
        event_id: SynchronizationEventId,
    ) -> Result<bool, HybridError> {
        Ok(self
            .record(event_id)?
            .is_some_and(|record| record.status.confirms_cancellation()))
    }

    /// Validates the tracker's graph identity and all stored records.
    ///
    /// This is structural validation only. It does not resolve graph events
    /// or prove that any external runtime outcome is truthful.
    pub fn validate(&self) -> Result<(), HybridError> {
        for (event_id, record) in &self.records {
            self.validate_event_graph(*event_id)?;

            if record.event_id() != *event_id {
                return Err(invalid_representation(
                    "cancellation record key does not match its event identity",
                ));
            }

            record.validate()?;
        }

        Ok(())
    }

    fn transition(
        &mut self,
        event_id: SynchronizationEventId,
        next: CancellationStatus,
        failure: Option<CancellationFailure>,
    ) -> Result<(), HybridError> {
        self.validate_event_graph(event_id)?;

        let record = self.records.get_mut(&event_id).ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "no cancellation request is registered for event {event_id}"
                ),
            )
        })?;

        record.transition(next, failure)
    }

    fn validate_event_graph(
        &self,
        event_id: SynchronizationEventId,
    ) -> Result<(), HybridError> {
        if event_id.graph() != self.graph_id {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "cancellation event {event_id} belongs to graph {}, not graph {}",
                    event_id.graph(),
                    self.graph_id
                ),
            ));
        }

        Ok(())
    }
}

fn invalid_representation(message: impl Into<String>) -> HybridError {
    HybridError::new(HybridErrorCode::InvalidRepresentation, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::IdAllocator;

    fn setup() -> (
        IdAllocator,
        HybridGraphId,
        SynchronizationEventId,
    ) {
        let allocator = IdAllocator::new();
        let graph_id = allocator.allocate_graph_id().unwrap();
        let event_id = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();

        (allocator, graph_id, event_id)
    }

    fn request(
        event_id: SynchronizationEventId,
    ) -> CancellationRequest {
        CancellationRequest::new(
            event_id,
            CancellationRequirement::BestEffort,
            CancellationPropagation::None,
            Some("test cancellation".to_owned()),
        )
        .unwrap()
    }

    #[test]
    fn request_starts_pending_and_is_not_confirmation() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        let record = tracker.request(request(event_id)).unwrap();

        assert_eq!(record.status(), CancellationStatus::Pending);
        assert_eq!(record.revision(), 0);
        assert!(!record.status().confirms_cancellation());
        assert!(!tracker.cancellation_confirmed(event_id).unwrap());
    }

    #[test]
    fn valid_request_lifecycle_can_be_recorded() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();
        tracker.mark_issued(event_id).unwrap();
        tracker.confirm_cancelled(event_id).unwrap();

        let record = tracker.record(event_id).unwrap().unwrap();
        assert_eq!(record.status(), CancellationStatus::Confirmed);
        assert_eq!(record.revision(), 2);
        assert!(tracker.cancellation_confirmed(event_id).unwrap());
    }

    #[test]
    fn terminal_outcomes_cannot_be_overwritten() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();
        tracker.reject(event_id).unwrap();

        let result = tracker.confirm_cancelled(event_id);

        assert!(result.is_err());
        assert_eq!(
            tracker.record(event_id).unwrap().unwrap().status(),
            CancellationStatus::Rejected
        );
    }

    #[test]
    fn failed_request_requires_structured_failure() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();

        let failure = CancellationFailure::new(
            "RUNTIME_CANCEL_ERROR",
            "execution owner could not process cancellation",
        )
        .unwrap();

        tracker.mark_failed(event_id, failure).unwrap();

        let record = tracker.record(event_id).unwrap().unwrap();
        assert_eq!(record.status(), CancellationStatus::Failed);
        assert!(record.failure().is_some());
        assert!(record.validate().is_ok());
    }

    #[test]
    fn duplicate_requests_are_rejected() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();

        assert_eq!(
            tracker.request(request(event_id)).unwrap_err().code(),
            HybridErrorCode::DuplicateIdentity
        );
    }

    #[test]
    fn events_from_another_graph_are_rejected() {
        let allocator = IdAllocator::new();
        let graph_a = allocator.allocate_graph_id().unwrap();
        let graph_b = allocator.allocate_graph_id().unwrap();
        let event_b = allocator
            .allocate_synchronization_event_id(graph_b)
            .unwrap();

        let mut tracker = CancellationTracker::new(graph_a);

        let result = tracker.request(request(event_b));

        assert_eq!(
            result.unwrap_err().code(),
            HybridErrorCode::InvalidReference
        );
        assert!(tracker.is_empty());
    }

    #[test]
    fn not_applicable_cancellation_is_rejected() {
        let (_, _, event_id) = setup();

        let result = CancellationRequest::new(
            event_id,
            CancellationRequirement::NotApplicable,
            CancellationPropagation::None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn blank_reason_is_rejected() {
        let (_, _, event_id) = setup();

        let result = CancellationRequest::new(
            event_id,
            CancellationRequirement::BestEffort,
            CancellationPropagation::None,
            Some("   ".to_owned()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn failed_request_without_failure_is_invalid_after_deserialization() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();

        let record = tracker.records.get_mut(&event_id).unwrap();
        record.status = CancellationStatus::Failed;
        record.failure = None;

        assert!(record.validate().is_err());
        assert!(tracker.validate().is_err());
    }

    #[test]
    fn tracker_iteration_is_deterministic_by_event_identity() {
        let allocator = IdAllocator::new();
        let graph_id = allocator.allocate_graph_id().unwrap();
        let first = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();
        let second = allocator
            .allocate_synchronization_event_id(graph_id)
            .unwrap();

        let mut tracker = CancellationTracker::new(graph_id);
        tracker.request(request(second)).unwrap();
        tracker.request(request(first)).unwrap();

        let ids = tracker
            .records()
            .map(CancellationRecord::event_id)
            .collect::<Vec<_>>();

        assert_eq!(ids, vec![first, second]);
    }

    #[test]
    fn tracker_round_trips_through_serde_json() {
        let (_, graph_id, event_id) = setup();
        let mut tracker = CancellationTracker::new(graph_id);

        tracker.request(request(event_id)).unwrap();
        tracker.mark_issued(event_id).unwrap();

        let encoded = serde_json::to_string(&tracker).unwrap();
        let decoded: CancellationTracker =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, tracker);
        assert!(decoded.validate().is_ok());
    }
}
