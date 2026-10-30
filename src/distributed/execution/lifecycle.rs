
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Distributed execution lifecycle management.
//!
//! Path: `src/distributed/execution/lifecycle.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Responsibility
//!
//! This module owns the authoritative in-process lifecycle state and
//! transition journal for a logical distributed execution.
//!
//! It tracks:
//! - Accepted
//! - Running
//! - Recovering
//! - Completed
//! - Failed
//! - Cancelled
//!
//! It does not:
//! - create or validate execution plans;
//! - partition, place, route, or schedule work;
//! - execute workloads;
//! - implement cancellation propagation;
//! - implement checkpointing or recovery;
//! - implement network transport;
//! - define task, participant, or plan identity;
//! - own distributed semantic intent;
//! - own backend or hardware execution.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! # Integration
//!
//! - `crate::distributed::error`: canonical error and result types.
//! - `execution::executor`: records execution acceptance and execution
//!   transitions.
//! - `execution::cancellation`: requests cancellation and records confirmed
//!   cancellation.
//! - `resilience::recovery`: records recovery entry and recovery outcomes.
//! - `observability`: consumes lifecycle events through the event-draining
//!   interface; this module does not define a second metrics or tracing system.
//! - `runtime`: owns the lifecycle manager and its shutdown policy.
//!
//! The manager is generic over the caller's existing logical execution ID.
//! It does not introduce a competing ID type.
//!
//! # Persistence and delivery guarantees
//!
//! This implementation is an in-process lifecycle authority. Its state and
//! journal are not durable across process termination. A production runtime
//! that requires crash recovery must persist lifecycle events and execution
//! state through the repository's durable storage/checkpoint authority.
//!
//! A successful transition means the in-process state and journal were
//! updated. It does not prove that a remote side effect completed unless the
//! execution provider has independently confirmed that result.
//!
//! # Scalability
//!
//! No fixed limit is imposed on executions or lifecycle events. Actual growth
//! is constrained by available memory and the caller's operational policy.
//! The runtime may drain events and persist them externally. No universal
//! task, node, worker, or execution ceiling is introduced.

use crate::distributed::error::{DistributedError, DistributedResult};

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::{Mutex, MutexGuard};

/// Current lifecycle state of one logical distributed execution.
///
/// Terminal states cannot transition to another state. A new execution
/// attempt after a terminal state must use the appropriate higher-level
/// retry/reconciliation policy and its established logical identity rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionState {
    /// The execution has been accepted by the runtime but has not started.
    Accepted,

    /// Execution has started.
    Running,

    /// Execution is being reconciled or restored by the recovery subsystem.
    Recovering {
        /// Explanation supplied by the recovery subsystem.
        reason: String,
    },

    /// The execution completed successfully.
    Completed,

    /// The execution failed.
    Failed {
        /// Stable distributed error code, when available.
        error_code: String,

        /// Human-readable failure explanation.
        message: String,
    },

    /// Cancellation was confirmed by the responsible execution provider.
    Cancelled {
        /// Cancellation reason.
        reason: String,
    },
}

impl ExecutionState {
    /// Returns whether this state is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed { .. } | Self::Cancelled { .. }
        )
    }

    /// Returns a stable lifecycle state name.
    ///
    /// This is a lifecycle identifier, not an error code or wire-protocol
    /// version. If serialized externally, its compatibility must be governed
    /// by the transport protocol.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Running => "running",
            Self::Recovering { .. } => "recovering",
            Self::Completed => "completed",
            Self::Failed { .. } => "failed",
            Self::Cancelled { .. } => "cancelled",
        }
    }
}

/// A successful lifecycle transition.
///
/// The revision is monotonically increasing for an individual execution.
/// It is not a wall-clock timestamp and does not establish ordering between
/// different executions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleEvent<K> {
    /// Logical execution identifier supplied by the caller.
    pub execution_id: K,

    /// State before the transition.
    pub previous: Option<ExecutionState>,

    /// State after the transition.
    pub current: ExecutionState,

    /// Per-execution revision after this transition.
    pub revision: u64,
}

/// Immutable snapshot of one execution's lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionSnapshot<K> {
    /// Logical execution identifier.
    pub execution_id: K,

    /// Current state.
    pub state: ExecutionState,

    /// Current per-execution revision.
    pub revision: u64,
}

/// Internal state for one execution.
#[derive(Debug, Clone)]
struct ExecutionRecord {
    state: ExecutionState,
    revision: u64,
}

/// Shared manager state.
///
/// A single mutex makes state changes and journal appends atomic with respect
/// to other callers using this manager. It does not provide distributed
/// consensus or cross-process coordination.
#[derive(Debug)]
struct LifecycleStore<K> {
    executions: HashMap<K, ExecutionRecord>,
    events: Vec<LifecycleEvent<K>>,
}

/// Thread-safe lifecycle manager.
///
/// `K` must be the logical execution identifier already used by the caller.
/// The manager does not assume that an execution ID is a machine ID, process
/// ID, hostname, device ID, or network address.
///
/// The manager is intentionally independent of any specific runtime,
/// executor, backend, transport, or domain.
#[derive(Debug)]
pub struct ExecutionLifecycle<K> {
    store: Mutex<LifecycleStore<K>>,
}

impl<K> Default for ExecutionLifecycle<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> ExecutionLifecycle<K>
where
    K: Clone + Eq + Hash + fmt::Debug,
{
    /// Creates an empty lifecycle manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Mutex::new(LifecycleStore {
                executions: HashMap::new(),
                events: Vec::new(),
            }),
        }
    }

    /// Registers a newly accepted execution.
    ///
    /// An execution identifier may be registered only once in this manager.
    /// Duplicate registration is a conflict, not an implicit reset.
    pub fn accept(&self, execution_id: K) -> DistributedResult<LifecycleEvent<K>> {
        let mut store = self.lock_store()?;

        if store.executions.contains_key(&execution_id) {
            return Err(DistributedError::Conflict {
                subject: format!("execution {:?}", execution_id),
                message: "execution is already registered".to_owned(),
            });
        }

        let event = LifecycleEvent {
            execution_id: execution_id.clone(),
            previous: None,
            current: ExecutionState::Accepted,
            revision: 0,
        };

        // No fallible operation follows the journal append before insertion.
        store.events.push(event.clone());
        store.executions.insert(
            execution_id,
            ExecutionRecord {
                state: ExecutionState::Accepted,
                revision: 0,
            },
        );

        Ok(event)
    }

    /// Marks an accepted or recovering execution as running.
    ///
    /// Recovery code must call this only after it has established that
    /// execution may safely resume.
    pub fn start(&self, execution_id: &K) -> DistributedResult<LifecycleEvent<K>> {
        self.transition(execution_id, ExecutionState::Running)
    }

    /// Marks a running execution as recovering.
    ///
    /// This records lifecycle state only. It does not create a checkpoint,
    /// restore state, migrate work, or establish that recovery is possible.
    pub fn begin_recovery(
        &self,
        execution_id: &K,
        reason: impl Into<String>,
    ) -> DistributedResult<LifecycleEvent<K>> {
        self.transition(
            execution_id,
            ExecutionState::Recovering {
                reason: reason.into(),
            },
        )
    }

    /// Records successful completion.
    ///
    /// Call this only after the execution provider has confirmed successful
    /// completion and the result has passed the caller's required validation.
    pub fn complete(&self, execution_id: &K) -> DistributedResult<LifecycleEvent<K>> {
        self.transition(execution_id, ExecutionState::Completed)
    }

    /// Records execution failure.
    ///
    /// The canonical distributed error classification is retained as its
    /// stable code and display explanation. The original error should still
    /// be propagated to the caller by the executor; this lifecycle record is
    /// not a replacement for the error object or its source chain.
    pub fn fail(
        &self,
        execution_id: &K,
        error: &DistributedError,
    ) -> DistributedResult<LifecycleEvent<K>> {
        self.transition(
            execution_id,
            ExecutionState::Failed {
                error_code: error.code().to_owned(),
                message: error.to_string(),
            },
        )
    }

    /// Records confirmed cancellation.
    ///
    /// A cancellation request alone is not confirmation. The caller must
    /// first obtain the appropriate acknowledgement or definitive local
    /// execution result from the responsible provider.
    pub fn confirm_cancelled(
        &self,
        execution_id: &K,
        reason: impl Into<String>,
    ) -> DistributedResult<LifecycleEvent<K>> {
        self.transition(
            execution_id,
            ExecutionState::Cancelled {
                reason: reason.into(),
            },
        )
    }

    /// Returns the current snapshot for an execution.
    pub fn get(&self, execution_id: &K) -> DistributedResult<Option<ExecutionSnapshot<K>>> {
        let store = self.lock_store()?;

        Ok(store.executions.get(execution_id).map(|record| {
            ExecutionSnapshot {
                execution_id: execution_id.clone(),
                state: record.state.clone(),
                revision: record.revision,
            }
        }))
    }

    /// Returns whether an execution is currently registered.
    pub fn contains(&self, execution_id: &K) -> DistributedResult<bool> {
        let store = self.lock_store()?;
        Ok(store.executions.contains_key(execution_id))
    }

    /// Returns the number of executions currently tracked by this manager.
    ///
    /// This is an observation of current in-memory state, not a global
    /// distributed cluster count.
    pub fn execution_count(&self) -> DistributedResult<usize> {
        let store = self.lock_store()?;
        Ok(store.executions.len())
    }

    /// Returns the number of lifecycle events waiting to be consumed.
    pub fn pending_event_count(&self) -> DistributedResult<usize> {
        let store = self.lock_store()?;
        Ok(store.events.len())
    }

    /// Drains the currently pending lifecycle events.
    ///
    /// A runtime or observability adapter should consume these events and
    /// persist or export them as required. Draining does not delete execution
    /// state. Events appended after this operation remain available for the
    /// next drain.
    ///
    /// This method transfers ownership of the events to the caller. If the
    /// caller fails to persist them, it is responsible for retaining or
    /// reconciling them. This in-memory journal is not a durable event log.
    pub fn drain_events(&self) -> DistributedResult<Vec<LifecycleEvent<K>>> {
        let mut store = self.lock_store()?;
        Ok(std::mem::take(&mut store.events))
    }

    /// Returns a snapshot of all currently tracked executions.
    ///
    /// Hash-map iteration order is intentionally unspecified. Callers that
    /// require deterministic presentation or serialization must sort by
    /// their own logical identifier contract.
    pub fn snapshots(&self) -> DistributedResult<Vec<ExecutionSnapshot<K>>> {
        let store = self.lock_store()?;

        Ok(store
            .executions
            .iter()
            .map(|(execution_id, record)| ExecutionSnapshot {
                execution_id: execution_id.clone(),
                state: record.state.clone(),
                revision: record.revision,
            })
            .collect())
    }

    /// Applies a validated lifecycle transition.
    ///
    /// Only transitions defined by `is_transition_allowed` are accepted.
    /// Terminal states are immutable.
    pub fn transition(
        &self,
        execution_id: &K,
        next: ExecutionState,
    ) -> DistributedResult<LifecycleEvent<K>> {
        let mut store = self.lock_store()?;

        let record = store.executions.get(execution_id).ok_or_else(|| {
            DistributedError::invalid_input(format!(
                "execution {:?} has not been accepted",
                execution_id
            ))
        })?;

        if !is_transition_allowed(&record.state, &next) {
            return Err(DistributedError::Conflict {
                subject: format!("execution {:?}", execution_id),
                message: format!(
                    "lifecycle transition from {} to {} is not permitted",
                    record.state.as_str(),
                    next.as_str()
                ),
            });
        }

        let revision = record.revision.checked_add(1).ok_or_else(|| {
            DistributedError::InternalInvariantViolation {
                invariant: "execution_lifecycle_revision".to_owned(),
                message: format!(
                    "revision counter exhausted for execution {:?}",
                    execution_id
                ),
            }
        })?;

        let event = LifecycleEvent {
            execution_id: execution_id.clone(),
            previous: Some(record.state.clone()),
            current: next.clone(),
            revision,
        };

        // The mutex protects both operations from concurrent transitions.
        // The append occurs before the state update; after the append, the
        // update is an infallible assignment.
        store.events.push(event.clone());

        let record = store
            .executions
            .get_mut(execution_id)
            .ok_or_else(|| DistributedError::InternalInvariantViolation {
                invariant: "execution_lifecycle_record".to_owned(),
                message: "execution record disappeared while the lifecycle lock was held"
                    .to_owned(),
            })?;

        record.state = next;
        record.revision = revision;

        Ok(event)
    }

    fn lock_store(&self) -> DistributedResult<MutexGuard<'_, LifecycleStore<K>>> {
        self.store.lock().map_err(|_| {
            DistributedError::InternalInvariantViolation {
                invariant: "execution_lifecycle_mutex".to_owned(),
                message: "lifecycle state lock was poisoned".to_owned(),
            }
        })
    }
}

/// Returns whether a transition is valid under the distributed lifecycle
/// contract.
///
/// Allowed transitions:
///
/// - Accepted -> Running
/// - Accepted -> Failed
/// - Accepted -> Cancelled
/// - Running -> Completed
/// - Running -> Failed
/// - Running -> Cancelled
/// - Running -> Recovering
/// - Recovering -> Running
/// - Recovering -> Failed
/// - Recovering -> Cancelled
///
/// No terminal state may transition again.
///
/// In particular:
/// - completion cannot be declared before execution starts;
/// - recovery cannot begin before execution starts;
/// - a failed or cancelled execution cannot be silently restarted;
/// - a recovery attempt must explicitly return to Running before completion.
#[must_use]
pub fn is_transition_allowed(current: &ExecutionState, next: &ExecutionState) -> bool {
    match current {
        ExecutionState::Accepted => matches!(
            next,
            ExecutionState::Running
                | ExecutionState::Failed { .. }
                | ExecutionState::Cancelled { .. }
        ),

        ExecutionState::Running => matches!(
            next,
            ExecutionState::Completed
                | ExecutionState::Failed { .. }
                | ExecutionState::Cancelled { .. }
                | ExecutionState::Recovering { .. }
        ),

        ExecutionState::Recovering { .. } => matches!(
            next,
            ExecutionState::Running
                | ExecutionState::Failed { .. }
                | ExecutionState::Cancelled { .. }
        ),

        ExecutionState::Completed
        | ExecutionState::Failed { .. }
        | ExecutionState::Cancelled { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> String {
        value.to_owned()
    }

    #[test]
    fn accepts_execution_and_records_initial_event() {
        let lifecycle = ExecutionLifecycle::<String>::new();

        let event = lifecycle.accept(id("logical-execution-a")).unwrap();

        assert_eq!(event.current, ExecutionState::Accepted);
        assert_eq!(event.previous, None);
        assert_eq!(event.revision, 0);

        let snapshot = lifecycle.get(&id("logical-execution-a")).unwrap().unwrap();

        assert_eq!(snapshot.state, ExecutionState::Accepted);
        assert_eq!(snapshot.revision, 0);
    }

    #[test]
    fn rejects_duplicate_acceptance() {
        let lifecycle = ExecutionLifecycle::<String>::new();

        lifecycle.accept(id("execution")).unwrap();

        let error = lifecycle.accept(id("execution")).unwrap_err();

        assert_eq!(
            error.kind(),
            crate::distributed::error::DistributedErrorKind::Conflict
        );
    }

    #[test]
    fn supports_normal_execution_lifecycle() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();
        lifecycle.complete(&execution_id).unwrap();

        let snapshot = lifecycle.get(&execution_id).unwrap().unwrap();

        assert_eq!(snapshot.state, ExecutionState::Completed);
        assert_eq!(snapshot.revision, 2);
        assert!(snapshot.state.is_terminal());
    }

    #[test]
    fn supports_recovery_then_resumption() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();
        lifecycle
            .begin_recovery(&execution_id, "worker became unavailable")
            .unwrap();

        let recovering = lifecycle.get(&execution_id).unwrap().unwrap();
        assert!(matches!(
            recovering.state,
            ExecutionState::Recovering { .. }
        ));

        lifecycle.start(&execution_id).unwrap();
        lifecycle.complete(&execution_id).unwrap();

        let completed = lifecycle.get(&execution_id).unwrap().unwrap();
        assert_eq!(completed.state, ExecutionState::Completed);
    }

    #[test]
    fn rejects_completion_before_start() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();

        let error = lifecycle.complete(&execution_id).unwrap_err();

        assert_eq!(
            error.kind(),
            crate::distributed::error::DistributedErrorKind::Conflict
        );
    }

    #[test]
    fn rejects_transitions_after_terminal_state() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();
        lifecycle.complete(&execution_id).unwrap();

        assert!(lifecycle.start(&execution_id).is_err());
        assert!(lifecycle.confirm_cancelled(&execution_id, "late cancellation").is_err());
        assert!(lifecycle
            .begin_recovery(&execution_id, "late recovery")
            .is_err());
    }

    #[test]
    fn records_failure_with_canonical_error_code() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();

        let failure = DistributedError::external_message(
            "test-provider",
            "provider execution failed",
        );

        lifecycle.fail(&execution_id, &failure).unwrap();

        let snapshot = lifecycle.get(&execution_id).unwrap().unwrap();

        match snapshot.state {
            ExecutionState::Failed {
                error_code,
                message,
            } => {
                assert_eq!(error_code, failure.code());
                assert!(message.contains("provider execution failed"));
            }
            other => panic!("expected failed state, got {other:?}"),
        }
    }

    #[test]
    fn cancellation_is_terminal_and_explicit() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();
        lifecycle
            .confirm_cancelled(&execution_id, "caller requested cancellation")
            .unwrap();

        let snapshot = lifecycle.get(&execution_id).unwrap().unwrap();

        assert!(matches!(
            snapshot.state,
            ExecutionState::Cancelled { .. }
        ));
        assert!(snapshot.state.is_terminal());
    }

    #[test]
    fn event_revisions_increase_per_execution() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();
        lifecycle.start(&execution_id).unwrap();
        lifecycle.complete(&execution_id).unwrap();

        let events = lifecycle.drain_events().unwrap();

        assert_eq!(events.len(), 3);
        assert_eq!(events[0].revision, 0);
        assert_eq!(events[1].revision, 1);
        assert_eq!(events[2].revision, 2);
    }

    #[test]
    fn event_drain_does_not_delete_execution_state() {
        let lifecycle = ExecutionLifecycle::<String>::new();
        let execution_id = id("execution");

        lifecycle.accept(execution_id.clone()).unwrap();

        let events = lifecycle.drain_events().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(lifecycle.pending_event_count().unwrap(), 0);

        let snapshot = lifecycle.get(&execution_id).unwrap().unwrap();
        assert_eq!(snapshot.state, ExecutionState::Accepted);
    }

    #[test]
    fn independent_executions_have_independent_revisions() {
        let lifecycle = ExecutionLifecycle::<String>::new();

        lifecycle.accept(id("first")).unwrap();
        lifecycle.accept(id("second")).unwrap();

        lifecycle.start(&id("first")).unwrap();

        assert_eq!(
            lifecycle.get(&id("first")).unwrap().unwrap().revision,
            1
        );
        assert_eq!(
            lifecycle.get(&id("second")).unwrap().unwrap().revision,
            0
        );
    }

    #[test]
    fn transition_table_rejects_all_terminal_transitions() {
        let terminal_states = [
            ExecutionState::Completed,
            ExecutionState::Failed {
                error_code: "distributed.execution_failed".to_owned(),
                message: "failure".to_owned(),
            },
            ExecutionState::Cancelled {
                reason: "cancelled".to_owned(),
            },
        ];

        let possible_next_states = [
            ExecutionState::Accepted,
            ExecutionState::Running,
            ExecutionState::Recovering {
                reason: "recover".to_owned(),
            },
            ExecutionState::Completed,
            ExecutionState::Failed {
                error_code: "distributed.execution_failed".to_owned(),
                message: "failure".to_owned(),
            },
            ExecutionState::Cancelled {
                reason: "cancelled".to_owned(),
            },
        ];

        for current in &terminal_states {
            for next in &possible_next_states {
                assert!(
                    !is_transition_allowed(current, next),
                    "{} must not transition to {}",
                    current.as_str(),
                    next.as_str()
                );
            }
        }
    }

    #[test]
    fn unknown_execution_is_not_implicitly_created() {
        let lifecycle = ExecutionLifecycle::<String>::new();

        let error = lifecycle.start(&id("unknown")).unwrap_err();

        assert_eq!(
            error.kind(),
            crate::distributed::error::DistributedErrorKind::InvalidInput
        );
        assert_eq!(lifecycle.execution_count().unwrap(), 0);
    }
}
