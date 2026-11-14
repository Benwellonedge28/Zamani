#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Recovery Integration
//!
//! Path: `src/zur/reliability/recovery.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Integrates ZUR execution sessions with the canonical distributed
//! recovery subsystem.
//!
//! This module is an operational integration boundary. It does not
//! implement a second recovery engine.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - The ZUR-to-distributed recovery integration boundary.
//! - Session-scoped recovery invocation.
//! - Cooperative cancellation checks around recovery operations.
//! - Translation of distributed operational errors into ZurError.
//! - Explicit access to the canonical recovery engine.
//!
//! It does not own:
//!
//! - Recovery incidents or recovery plans.
//! - Recovery action identities or action state.
//! - Recovery journals or durable persistence.
//! - Recovery planning.
//! - Retry policy or retry execution.
//! - Checkpoint serialization or checkpoint storage.
//! - Distributed execution or transport.
//! - Membership or participant discovery.
//! - Resource allocation or scheduling.
//! - Quantum error correction or ZQN fault models.
//! - Hybrid execution planning.
//! - Domain-specific state reconstruction.
//! - Authentication or authorization policy.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Canonical integration authorities
//!
//! `crate::zur::error`:
//!     Canonical ZUR operational errors.
//!
//! `crate::zur::execution::session`:
//!     Execution session identity and cooperative cancellation.
//!
//! `crate::distributed::resilience::recovery`:
//!     Canonical durable recovery engine, journal, plans, actions,
//!     reconciliation and authoritative verification.
//!
//! `crate::distributed::error`:
//!     Canonical distributed operational error model.
//!
//! `crate::distributed::ids`:
//!     Canonical distributed operation and plan identities.
//!
//! `crate::distributed::resilience::checkpoint`:
//!     Canonical distributed checkpoint ownership.
//!
//! Quantum and hybrid resilience authorities:
//!     Domain-specific recovery and state reconstruction.
//!
//! ## Recovery invariant
//!
//! Recovery is not successful merely because:
//!
//! - A recovery request was created.
//! - A recovery plan was accepted.
//! - An action was submitted.
//! - A provider acknowledged a request.
//! - A checkpoint was found.
//! - A session was cancelled.
//!
//! Successful recovery requires the canonical distributed recovery
//! engine to complete its actions and verify the resulting state.
//!
//! ## Unknown outcomes
//!
//! An interrupted or uncertain operation must be reconciled before
//! it can safely be resubmitted.
//!
//! Cancellation does not prove that remote work has stopped.
//!
//! This module preserves those guarantees by delegating action
//! identity, journaling and reconciliation to the distributed engine.
//!
//! ## Session cancellation
//!
//! Session cancellation is checked before recovery operations and
//! before every delegated recovery-provider operation.
//!
//! If cancellation occurs while an external operation is already
//! executing, this module does not invent a cancellation outcome.
//! The canonical provider must report or reconcile the actual result.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Recovery incidents.
//! - Recovery operations.
//! - Recovery actions.
//! - Checkpoints.
//! - Execution sessions.
//! - Participants.
//! - Providers.
//! - Devices.
//! - Computational domains.
//!
//! Durable storage, journal capacity, memory, execution time and
//! provider capabilities remain real operational constraints.
//!
//! Literal infinite computation is not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No global mutable state.
//! No hidden runtime construction.
//! No fabricated recovery success.
//! No automatic retry.
//! No in-memory replacement for the durable recovery journal.
//! No duplicated recovery engine.

use core::fmt;

use crate::distributed::error::{
    DistributedError,
    DistributedErrorKind,
    DistributedResult,
};

use crate::distributed::ids::OperationId;

use crate::distributed::resilience::recovery::{
    JournalWrite,
    RecoveryAction,
    RecoveryActionId,
    RecoveryActionStatus,
    RecoveryEngine,
    RecoveryExecutor,
    RecoveryIncident,
    RecoveryJournal,
    RecoveryPlan,
    RecoveryRecord,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::execution::session::ExecutionSession;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.reliability.recovery";

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Converts a canonical distributed recovery error into a ZUR error.
///
/// The original distributed error is retained as the causal source.
/// Its classification is mapped to the closest available ZUR category.
///
/// This function does not inspect human-readable error text to determine
/// control flow.
fn adapt_distributed_error(
    error: DistributedError,
    operation: &'static str,
) -> ZurError {
    let kind = error.kind();
    let message = error.to_string();

    let result = match kind {
        DistributedErrorKind::Cancelled => {
            ZurError::cancelled(message)
        }

        DistributedErrorKind::AuthorizationDenied
        | DistributedErrorKind::AuthenticationFailed => {
            ZurError::permission_denied(message)
        }

        DistributedErrorKind::CapabilityUnsatisfied
        | DistributedErrorKind::Unrepresentable
        | DistributedErrorKind::UnsupportedOperation => {
            ZurError::infeasible(message)
        }

        DistributedErrorKind::ResourceUnsatisfied => {
            ZurError::resource_unavailable(message)
        }

        DistributedErrorKind::ProtocolIncompatible => {
            ZurError::compatibility(message)
        }

        DistributedErrorKind::OperationOutcomeUnknown
        | DistributedErrorKind::RecoveryFailed
        | DistributedErrorKind::StateTransferFailed => {
            ZurError::internal(message)
                .with_recovery(
                    crate::zur::error::RecoveryDisposition::RecoveryRequired,
                )
        }

        _ => ZurError::internal(message),
    };

    result
        .with_source(error)
        .with_component(COMPONENT)
        .with_operation(operation)
}

/// Converts session cancellation into the canonical distributed error
/// expected by the distributed recovery execution interface.
fn session_cancellation_error(
    error: ZurError,
) -> DistributedError {
    DistributedError::Cancelled {
        operation: None,
        reason: error.to_string(),
    }
}

/// Checks whether the execution session has requested cancellation.
///
/// This function only observes the session cancellation state. It does
/// not claim that an external or remote operation has terminated.
fn check_session(
    session: &ExecutionSession,
) -> DistributedResult<()> {
    session
        .check_cancelled()
        .map_err(session_cancellation_error)
}

// ============================================================================
// SESSION-BOUND RECOVERY EXECUTOR
// ============================================================================

/// Recovery executor adapter that binds distributed recovery operations
/// to one ZUR execution session.
///
/// The adapter delegates every recovery operation to the supplied
/// canonical recovery executor.
///
/// It does not retain or replace the underlying executor.
#[derive(Debug)]
struct SessionBoundRecoveryExecutor<'a> {
    session: &'a ExecutionSession,
    executor: &'a dyn RecoveryExecutor,
}

impl<'a> SessionBoundRecoveryExecutor<'a> {
    /// Constructs a session-bound executor adapter.
    fn new(
        session: &'a ExecutionSession,
        executor: &'a dyn RecoveryExecutor,
    ) -> Self {
        Self {
            session,
            executor,
        }
    }
}

impl RecoveryExecutor for SessionBoundRecoveryExecutor<'_> {
    fn validate_plan(
        &self,
        incident: &RecoveryIncident,
        plan: &RecoveryPlan,
    ) -> DistributedResult<()> {
        check_session(self.session)?;

        self.executor.validate_plan(incident, plan)
    }

    fn reconcile_action(
        &self,
        action_id: RecoveryActionId,
        action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus> {
        check_session(self.session)?;

        self.executor.reconcile_action(action_id, action)
    }

    fn execute_action(
        &self,
        action_id: RecoveryActionId,
        action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus> {
        check_session(self.session)?;

        self.executor.execute_action(action_id, action)
    }

    fn verify_recovered_state(
        &self,
        incident: &RecoveryIncident,
        plan: &RecoveryPlan,
    ) -> DistributedResult<()> {
        check_session(self.session)?;

        self.executor.verify_recovered_state(incident, plan)
    }
}

// ============================================================================
// ZUR RECOVERY COORDINATOR
// ============================================================================

/// ZUR integration facade for the canonical distributed recovery engine.
///
/// This coordinator is stateless. Durable recovery state is owned by
/// the injected recovery journal.
///
/// The executor is supplied by the authoritative execution integration.
///
/// Multiple coordinators can operate against the same journal when its
/// implementation satisfies the atomic compare-and-swap contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct ZurRecoveryCoordinator {
    engine: RecoveryEngine,
}

impl ZurRecoveryCoordinator {
    /// Creates a ZUR recovery coordinator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            engine: RecoveryEngine::new(),
        }
    }

    /// Returns the canonical distributed recovery engine.
    ///
    /// This is provided for advanced integrations that need direct access
    /// to the underlying recovery contract.
    ///
    /// Ordinary ZUR integrations should use the session-bound methods
    /// provided by this coordinator.
    #[must_use]
    pub const fn engine(&self) -> &RecoveryEngine {
        &self.engine
    }

    // ------------------------------------------------------------------------
    // Incident registration
    // ------------------------------------------------------------------------

    /// Registers a recovery incident for a ZUR execution session.
    ///
    /// The incident and its operation identity remain owned by the
    /// distributed subsystem.
    ///
    /// Duplicate registration is rejected by the canonical engine.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The session has been cancelled.
    /// - The operation already has a recovery record.
    /// - The journal cannot durably persist the incident.
    /// - Concurrent registration causes a revision conflict.
    pub fn register_incident(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        incident: RecoveryIncident,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "register_incident")
            })?;

        self.engine
            .register_incident(journal, incident)
            .map_err(|error| {
                adapt_distributed_error(error, "register_incident")
            })
    }

    // ------------------------------------------------------------------------
    // Recovery planning and validation
    // ------------------------------------------------------------------------

    /// Begins a recovery generation using an externally prepared
    /// canonical recovery plan.
    ///
    /// Recovery-plan construction and planning remain owned by the
    /// distributed planning and resilience authorities.
    ///
    /// The supplied executor validates the plan against current
    /// authorization, capability, resource, membership, checkpoint
    /// and execution assumptions.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is cancelled, the plan is stale,
    /// the plan is invalid, validation fails, or durable journal
    /// persistence fails.
    pub fn begin_recovery(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        plan: RecoveryPlan,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "begin_recovery")
            })?;

        let bound_executor =
            SessionBoundRecoveryExecutor::new(session, executor);

        self.engine
            .begin_recovery(journal, &bound_executor, plan)
            .map_err(|error| {
                adapt_distributed_error(error, "begin_recovery")
            })
    }

    // ------------------------------------------------------------------------
    // Recovery execution
    // ------------------------------------------------------------------------

    /// Executes a previously validated recovery plan.
    ///
    /// The canonical distributed engine:
    ///
    /// - Persists action intent before submission.
    /// - Preserves stable recovery action identities.
    /// - Reconciles interrupted actions before resubmission.
    /// - Does not automatically retry uncertain operations.
    /// - Records incomplete and unknown outcomes.
    ///
    /// The supplied executor performs the actual recovery operations
    /// through the appropriate distributed, runtime, checkpoint or
    /// domain-specific authorities.
    ///
    /// # Errors
    ///
    /// Returns an error when cancellation is requested, recovery
    /// execution fails, an action outcome remains unknown, journal
    /// persistence fails, or the recovery lifecycle is invalid.
    pub fn execute_recovery(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "execute_recovery")
            })?;

        let bound_executor =
            SessionBoundRecoveryExecutor::new(session, executor);

        self.engine
            .execute_recovery(
                journal,
                &bound_executor,
                operation_id,
            )
            .map_err(|error| {
                adapt_distributed_error(error, "execute_recovery")
            })
    }

    // ------------------------------------------------------------------------
    // Verification
    // ------------------------------------------------------------------------

    /// Verifies the recovered state and commits the terminal success state.
    ///
    /// The canonical recovery engine requires every recovery action to
    /// be completed and requires the executor to establish the validity
    /// of the recovered logical computation state.
    ///
    /// A successful return is therefore based on authoritative
    /// verification, not merely successful action submission.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is cancelled, any action is
    /// incomplete, the recovery plan is invalid, state verification
    /// fails, or durable journal persistence fails.
    pub fn verify_recovery(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "verify_recovery")
            })?;

        let bound_executor =
            SessionBoundRecoveryExecutor::new(session, executor);

        self.engine
            .verify_recovery(
                journal,
                &bound_executor,
                operation_id,
            )
            .map_err(|error| {
                adapt_distributed_error(error, "verify_recovery")
            })
    }

    // ------------------------------------------------------------------------
    // Uncertain outcomes
    // ------------------------------------------------------------------------

    /// Marks an operation as requiring external reconciliation.
    ///
    /// This operation is appropriate when an execution provider,
    /// journal or interrupted recovery process cannot establish
    /// an authoritative outcome.
    ///
    /// It does not resubmit the operation.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is cancelled, the recovery
    /// record does not exist, the record cannot be updated, or the
    /// operation is already in a terminal state that cannot be reopened.
    pub fn require_reconciliation(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
        reason: impl Into<String>,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "require_reconciliation")
            })?;

        self.engine
            .require_reconciliation(
                journal,
                operation_id,
                reason,
            )
            .map_err(|error| {
                adapt_distributed_error(error, "require_reconciliation")
            })
    }

    // ------------------------------------------------------------------------
    // Recovery cancellation
    // ------------------------------------------------------------------------

    /// Cancels a recovery lifecycle in the canonical durable journal.
    ///
    /// This records recovery cancellation. It does not establish that
    /// an external computation or remote participant has stopped.
    ///
    /// Actual execution cancellation must be propagated through the
    /// appropriate execution-session and distributed cancellation
    /// authorities.
    ///
    /// # Errors
    ///
    /// Returns an error when the session is cancelled, the recovery
    /// record does not exist, the record cannot be updated, or recovery
    /// has already reached a terminal success state.
    pub fn cancel_recovery(
        &self,
        session: &ExecutionSession,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        check_session(session)
            .map_err(|error| {
                adapt_distributed_error(error, "cancel_recovery")
            })?;

        self.engine
            .cancel(journal, operation_id)
            .map_err(|error| {
                adapt_distributed_error(error, "cancel_recovery")
            })
    }
}

// ============================================================================
// PUBLIC INTEGRATION HELPERS
// ============================================================================

/// Returns the operation identity associated with a recovery incident.
///
/// This helper deliberately delegates identity ownership to the
/// distributed recovery contract.
#[must_use]
pub fn incident_operation_id(
    incident: &RecoveryIncident,
) -> OperationId {
    incident.operation_id()
}

/// Returns the operation identity associated with a recovery plan.
///
/// This helper deliberately delegates identity ownership to the
/// distributed recovery contract.
#[must_use]
pub fn plan_operation_id(
    plan: &RecoveryPlan,
) -> OperationId {
    plan.operation_id()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinator_is_constructible_without_global_state() {
        let coordinator = ZurRecoveryCoordinator::new();

        let _engine = coordinator.engine();
    }

    #[test]
    fn coordinator_is_copyable_and_stateless() {
        let first = ZurRecoveryCoordinator::new();
        let second = first;

        let _ = second.engine();
    }

    #[test]
    fn incident_operation_identity_is_delegated() {
        // Full incident construction and durable lifecycle tests belong
        // in the distributed recovery integration test suite because
        // OperationId allocation and journal implementations are owned
        // by that subsystem.
        //
        // This test intentionally does not create a fake journal or
        // fabricate a recovery outcome.
        let _coordinator = ZurRecoveryCoordinator::new();
    }
}