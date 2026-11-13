
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Distributed Execution Sessions
//!
//! Path: src/zur/distributed/session.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the ZUR integration contract for sessions associated with
//! canonical distributed execution plans.
//!
//! A DistributedExecutionSession associates:
//!
//! - An existing ZUR ExecutionSession.
//! - An existing canonical distributed ExecutablePlan.
//! - The canonical distributed PlanId.
//! - Session-scoped cancellation.
//! - Runtime operation context.
//! - Session diagnostics.
//! - Existing service and resource lease references.
//!
//! It provides an operational session boundary for distributed
//! computation without introducing another distributed execution engine.
//!
//! # ARCHITECTURAL AUTHORITIES
//!
//! Source-language distributed semantics:
//!     grammar/spec/distributed.md
//!
//! Distributed grammar:
//!     grammar/distributed/
//!
//! Distributed execution grammar:
//!     grammar/execution/distributed-execution.g4
//!
//! Canonical distributed execution plan:
//!     crate::distributed::execution::executor::ExecutablePlan
//!
//! Distributed plan identity:
//!     crate::distributed::ids::PlanId
//!
//! Distributed execution implementation:
//!     crate::distributed::execution
//!
//! ZUR distributed integration:
//!     crate::zur::distributed
//!
//! Canonical execution session:
//!     crate::zur::execution::session::ExecutionSession
//!
//! Canonical session identity:
//!     crate::zur::ids::SessionId
//!
//! Runtime context:
//!     crate::zur::core::context::RuntimeContext
//!
//! Runtime operation context:
//!     crate::stdlib::context::OperationContext
//!
//! Runtime cancellation:
//!     crate::zur::execution::cancellation
//!
//! Runtime resource leases:
//!     crate::zur::resources::lease
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Distributed execution session association.
//! - Distributed plan/session binding.
//! - Distributed session validation.
//! - Distributed session operational access.
//! - Distributed session snapshots.
//! - Distributed session cancellation delegation.
//! - Distributed session lifecycle delegation.
//!
//! This module does NOT own:
//!
//! - Distributed computation semantics.
//! - Distributed task identities.
//! - Distributed participant identities.
//! - Distributed execution plans.
//! - Distributed task graphs.
//! - Distributed placement.
//! - Distributed partitioning.
//! - Distributed routing.
//! - Distributed scheduling.
//! - Distributed membership.
//! - Node discovery.
//! - Capability definitions.
//! - Capability evidence.
//! - Resource requirements.
//! - Resource feasibility.
//! - Resource reservation.
//! - Physical resource allocation.
//! - Networking.
//! - Transport.
//! - Serialization protocols.
//! - Authentication.
//! - Authorization.
//! - Consensus.
//! - Replication.
//! - Checkpoint implementation.
//! - Distributed recovery.
//! - Provider implementation.
//! - Provider discovery.
//! - Provider selection.
//! - Actual distributed execution.
//!
//! These responsibilities remain with their existing authorities.
//!
//! # IMPORTANT EXECUTION DISTINCTIONS
//!
//! Session creation != execution submission.
//! Session activation != execution submission.
//! Plan validation != resource feasibility.
//! Resource feasibility != resource reservation.
//! Resource reservation != resource acquisition.
//! Resource acquisition != execution.
//! Execution submission != execution acceptance.
//! Execution acceptance != execution completion.
//! Cancellation request != cancellation confirmation.
//! Timeout != proof of non-execution.
//! Missing response != proof of failure.
//!
//! This module preserves these distinctions.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Distributed sessions.
//! - Distributed plans.
//! - Execution participants.
//! - Distributed tasks.
//! - Computational domains.
//! - Devices.
//! - Resource leases.
//! - Service leases.
//! - Execution contexts.
//!
//! The underlying distributed plan and runtime session determine
//! their own storage requirements.
//!
//! Actual scalability depends on available memory, processors,
//! communication facilities, provider implementations and other
//! real execution resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! # SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw pointers.
//! No static mutable state.
//! No global session registry.
//! No global execution singleton.
//! No fabricated execution results.
//! No fabricated cancellation confirmation.
//! No implicit resource allocation.
//! No arbitrary hardware ceilings.
//!
//! # INTEGRATION
//!
//! Runtime composition constructs and activates an ExecutionSession.
//!
//! Distributed execution integration supplies an existing canonical
//! ExecutablePlan.
//!
//! This module associates the two.
//!
//! The distributed execution executor receives:
//!
//! - The canonical plan.
//! - The session operation context.
//! - The session cancellation state.
//! - Existing runtime service and resource references.
//!
//! Actual execution remains owned by distributed/execution.
//!
//! The session does not independently select or invoke providers.

use std::fmt;
use std::sync::Arc;

use crate::distributed::execution::executor::ExecutablePlan;

use crate::distributed::ids::PlanId;

use crate::stdlib::context::OperationContext;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::execution::session::{
    DiagnosticLevel,
    ExecutionSession,
    SessionDiagnostic,
    SessionSnapshot,
    SessionStatus,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.distributed.session";

// ============================================================================
// DISTRIBUTED EXECUTION SESSION
// ============================================================================

/// An operational session bound to an existing canonical distributed
/// execution plan.
///
/// This type does not create another session lifecycle.
///
/// It composes the existing ZUR ExecutionSession and associates it
/// with an immutable shared reference to a canonical distributed plan.
///
/// Cloning this object shares the same execution session and plan.
///
/// It does not duplicate execution state.
///
/// # Concurrency
///
/// DistributedExecutionSession is safe to share between threads
/// because the canonical ExecutablePlan contract requires Send + Sync
/// and the underlying ExecutionSession provides thread-safe state.
///
/// # Execution
///
/// Constructing this object does not submit or execute the plan.
///
/// The distributed execution subsystem remains responsible for
/// actual execution.
#[derive(Clone)]
pub struct DistributedExecutionSession<P>
where
    P: ExecutablePlan,
{
    /// Canonical ZUR execution session.
    execution_session: ExecutionSession,

    /// Shared reference to the canonical distributed execution plan.
    plan: Arc<P>,
}

impl<P> DistributedExecutionSession<P>
where
    P: ExecutablePlan,
{
    // ========================================================================
    // CONSTRUCTION
    // ========================================================================

    /// Creates a distributed execution session.
    ///
    /// The supplied execution session must already be active.
    ///
    /// The canonical distributed plan is validated before the
    /// association is accepted.
    ///
    /// No new session identity is allocated.
    ///
    /// No physical resource is acquired.
    ///
    /// No provider is discovered or selected.
    ///
    /// No execution is submitted.
    ///
    /// # Arguments
    ///
    /// * `execution_session` - Existing active ZUR execution session.
    /// * `plan` - Shared canonical distributed executable plan.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The supplied session is not active.
    /// - The session has been cancelled.
    /// - The canonical distributed plan is invalid.
    pub fn new(
        execution_session: ExecutionSession,
        plan: Arc<P>,
    ) -> ZurResult<Self> {

        let status = execution_session.status()?;

        if status != SessionStatus::Active {
            return Err(
                session_error(
                    "distributed execution requires an active ZUR session",
                    "new",
                ),
            );
        }

        execution_session.check_cancelled()?;

        crate::zur::distributed::validate_distributed_plan(
            plan.as_ref(),
        )?;

        Ok(Self {
            execution_session,
            plan,
        })
    }

    // ========================================================================
    // IDENTITY
    // ========================================================================

    /// Returns the canonical ZUR session identity.
    ///
    /// No distributed-specific SessionId is introduced.
    #[must_use]
    pub fn id(
        &self,
    ) -> &crate::zur::ids::SessionId {

        self.execution_session.id()
    }

    /// Returns the canonical distributed plan identity.
    ///
    /// The identity belongs to the distributed subsystem.
    #[must_use]
    pub fn plan_id(
        &self,
    ) -> PlanId {

        self.plan.plan_id()
    }

    /// Returns the canonical distributed plan.
    ///
    /// The returned reference does not transfer ownership.
    #[must_use]
    pub fn plan(
        &self,
    ) -> &P {

        self.plan.as_ref()
    }

    /// Returns the shared canonical plan reference.
    #[must_use]
    pub fn shared_plan(
        &self,
    ) -> &Arc<P> {

        &self.plan
    }

    // ========================================================================
    // CANONICAL SESSION ACCESS
    // ========================================================================

    /// Returns the underlying canonical ZUR execution session.
    ///
    /// This provides access to existing:
    ///
    /// - Session lifecycle.
    /// - Service lease management.
    /// - Resource-accounting references.
    /// - Cancellation.
    /// - Diagnostics.
    /// - Session snapshots.
    ///
    /// It does not create a second implementation of these services.
    #[must_use]
    pub fn execution_session(
        &self,
    ) -> &ExecutionSession {

        &self.execution_session
    }

    /// Returns the canonical runtime operation context.
    ///
    /// Distributed execution operations should receive this context
    /// so they inherit the runtime and session cancellation signals.
    #[must_use]
    pub fn operation_context(
        &self,
    ) -> &OperationContext {

        self.execution_session.operation_context()
    }

    /// Returns the current session lifecycle status.
    pub fn status(
        &self,
    ) -> ZurResult<SessionStatus> {

        self.execution_session.status()
    }

    // ========================================================================
    // SESSION VALIDATION
    // ========================================================================

    /// Checks whether this distributed session is usable.
    ///
    /// This verifies:
    ///
    /// - The underlying session is active.
    /// - Cancellation has not been requested.
    /// - The canonical distributed plan remains valid.
    ///
    /// This does not establish:
    ///
    /// - Resource availability.
    /// - Resource reservation.
    /// - Provider availability.
    /// - Authorization.
    /// - Execution feasibility.
    /// - Successful execution.
    ///
    /// Volatile operational checks remain the responsibility of
    /// their respective authorities.
    pub fn validate(
        &self,
    ) -> ZurResult<()> {

        self.execution_session.check_cancelled()?;

        let status = self.execution_session.status()?;

        if status != SessionStatus::Active {
            return Err(
                session_error(
                    "distributed execution session is not active",
                    "validate",
                ),
            );
        }

        crate::zur::distributed::validate_distributed_plan(
            self.plan.as_ref(),
        )?;

        Ok(())
    }

    /// Revalidates the canonical distributed plan.
    ///
    /// This should be performed immediately before submitting
    /// the plan to the distributed execution authority.
    ///
    /// The canonical distributed subsystem remains responsible
    /// for any additional volatile resource, provider,
    /// authorization and execution-environment validation.
    pub fn revalidate_plan(
        &self,
    ) -> ZurResult<()> {

        self.execution_session.check_cancelled()?;

        if self.execution_session.status()? != SessionStatus::Active {
            return Err(
                session_error(
                    "cannot revalidate a plan for an inactive session",
                    "revalidate_plan",
                ),
            );
        }

        crate::zur::distributed::validate_distributed_plan(
            self.plan.as_ref(),
        )
    }

    // ========================================================================
    // CANCELLATION
    // ========================================================================

    /// Requests cooperative cancellation of this distributed session.
    ///
    /// Cancellation propagates through the canonical ZUR execution
    /// session's OperationContext.
    ///
    /// This does not independently send distributed cancellation
    /// messages to remote participants.
    ///
    /// Actual remote cancellation belongs to:
    ///
    ///     crate::distributed::execution::cancellation
    ///
    /// A successful return means only that the local session
    /// accepted the cancellation request.
    ///
    /// It does not prove that remote execution stopped.
    ///
    /// # Arguments
    ///
    /// * `reason` - Non-empty diagnostic cancellation reason.
    ///
    /// # Returns
    ///
    /// `true` when this call established a new cancellation request.
    ///
    /// `false` when cancellation was already requested.
    pub fn request_cancellation(
        &self,
        reason: impl Into<String>,
    ) -> ZurResult<bool> {

        self.execution_session.request_cancellation(reason)
    }

    /// Returns whether the runtime or distributed session
    /// has requested cancellation.
    ///
    /// This is an observation, not an execution-stop confirmation.
    #[must_use]
    pub fn is_cancelled(
        &self,
    ) -> bool {

        self.execution_session.is_cancelled()
    }

    /// Checks cancellation and returns the canonical structured error
    /// when cancellation has been requested.
    pub fn check_cancelled(
        &self,
    ) -> ZurResult<()> {

        self.execution_session.check_cancelled()
    }

    // ========================================================================
    // LIFECYCLE
    // ========================================================================

    /// Begins local session shutdown.
    ///
    /// This delegates lifecycle and cancellation to the canonical
    /// ExecutionSession.
    ///
    /// It does not claim that distributed work has terminated.
    pub fn begin_close(
        &self,
    ) -> ZurResult<()> {

        self.execution_session.begin_close()
    }

    /// Completes local session shutdown.
    ///
    /// This delegates cleanup and reference release to the canonical
    /// ExecutionSession.
    ///
    /// IMPORTANT:
    ///
    /// Closing this local session does not prove that:
    ///
    /// - Remote workers have stopped.
    /// - Remote processes have terminated.
    /// - Distributed tasks have completed.
    /// - Remote resource reservations have been released.
    /// - Checkpoints have been persisted.
    ///
    /// Those operations require confirmation from their
    /// authoritative distributed subsystems.
    ///
    /// Repeated calls are idempotent through the canonical
    /// ExecutionSession implementation.
    pub fn close(
        &self,
    ) -> ZurResult<()> {

        self.execution_session.close()
    }

    // ========================================================================
    // DIAGNOSTICS
    // ========================================================================

    /// Records a structured diagnostic against the underlying
    /// canonical execution session.
    ///
    /// Diagnostics are local operational observations.
    ///
    /// They do not establish execution success or failure.
    pub fn record_diagnostic(
        &self,
        diagnostic: SessionDiagnostic,
    ) -> ZurResult<()> {

        self.execution_session.record_diagnostic(diagnostic)
    }

    /// Records an informational diagnostic.
    pub fn record_information(
        &self,
        message: impl Into<String>,
    ) -> ZurResult<()> {

        let diagnostic = SessionDiagnostic::new(
            DiagnosticLevel::Info,
            COMPONENT,
            message,
        )?;

        self.record_diagnostic(diagnostic)
    }

    /// Records a warning diagnostic.
    pub fn record_warning(
        &self,
        message: impl Into<String>,
    ) -> ZurResult<()> {

        let diagnostic = SessionDiagnostic::new(
            DiagnosticLevel::Warning,
            COMPONENT,
            message,
        )?;

        self.record_diagnostic(diagnostic)
    }

    /// Records an error diagnostic.
    ///
    /// Recording a diagnostic does not automatically transition
    /// the execution session into Failed.
    ///
    /// The caller must explicitly invoke the appropriate lifecycle
    /// operation when a session-level failure has actually occurred.
    pub fn record_error(
        &self,
        message: impl Into<String>,
    ) -> ZurResult<()> {

        let diagnostic = SessionDiagnostic::new(
            DiagnosticLevel::Error,
            COMPONENT,
            message,
        )?;

        self.record_diagnostic(diagnostic)
    }

    /// Returns all diagnostics currently retained by the
    /// canonical execution session.
    pub fn diagnostics(
        &self,
    ) -> ZurResult<Vec<SessionDiagnostic>> {

        self.execution_session.diagnostics()
    }

    // ========================================================================
    // OBSERVABILITY
    // ========================================================================

    /// Returns an immutable snapshot of the distributed session.
    ///
    /// The snapshot combines:
    ///
    /// - Canonical distributed plan identity.
    /// - Canonical ZUR session snapshot.
    ///
    /// It does not represent a distributed execution result.
    pub fn snapshot(
        &self,
    ) -> ZurResult<DistributedSessionSnapshot> {

        Ok(DistributedSessionSnapshot {
            plan_id: self.plan.plan_id().to_string(),
            session: self.execution_session.snapshot()?,
        })
    }
}

// ============================================================================
// DISTRIBUTED SESSION SNAPSHOT
// ============================================================================

/// Immutable observation of a distributed execution session.
///
/// This is an operational snapshot, not a distributed execution result.
///
/// Remote execution state must be obtained from the canonical
/// distributed execution subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedSessionSnapshot {

    /// Canonical distributed plan identity.
    plan_id: String,

    /// Snapshot from the canonical ZUR execution session.
    session: SessionSnapshot,
}

impl DistributedSessionSnapshot {

    /// Returns the canonical distributed plan identity.
    #[must_use]
    pub fn plan_id(
        &self,
    ) -> &str {

        &self.plan_id
    }

    /// Returns the underlying ZUR session snapshot.
    #[must_use]
    pub fn session(
        &self,
    ) -> &SessionSnapshot {

        &self.session
    }

    /// Returns the canonical ZUR session identity.
    #[must_use]
    pub fn session_id(
        &self,
    ) -> &crate::zur::ids::SessionId {

        self.session.session_id()
    }

    /// Returns the observed lifecycle state.
    #[must_use]
    pub fn status(
        &self,
    ) -> SessionStatus {

        self.session.status()
    }

    /// Returns whether cancellation has been requested.
    ///
    /// This does not establish that distributed execution stopped.
    #[must_use]
    pub fn cancellation_requested(
        &self,
    ) -> bool {

        self.session.cancellation_requested()
    }

    /// Returns the number of retained service lease references.
    #[must_use]
    pub fn service_lease_count(
        &self,
    ) -> usize {

        self.session.service_lease_count()
    }

    /// Returns the number of retained resource references.
    #[must_use]
    pub fn resource_reference_count(
        &self,
    ) -> usize {

        self.session.resource_reference_count()
    }

    /// Returns the number of retained diagnostics.
    #[must_use]
    pub fn diagnostic_count(
        &self,
    ) -> usize {

        self.session.diagnostic_count()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl<P> fmt::Debug for DistributedExecutionSession<P>
where
    P: ExecutablePlan,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter
            .debug_struct("DistributedExecutionSession")
            .field(
                "session_id",
                self.execution_session.id(),
            )
            .field(
                "plan_id",
                &self.plan.plan_id(),
            )
            .field(
                "status",
                &self.execution_session.status().ok(),
            )
            .field(
                "cancelled",
                &self.execution_session.is_cancelled(),
            )
            .finish()
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn session_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {

    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn snapshot_contract_is_consistent() {

        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<DistributedSessionSnapshot>();
    }

    #[test]
    fn snapshot_is_cloneable_and_comparable() {

        fn assert_clone_eq<T: Clone + PartialEq + Eq>() {}

        assert_clone_eq::<DistributedSessionSnapshot>();
    }

    #[test]
    fn session_type_is_send_and_sync() {

        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<
            DistributedExecutionSession<
                TestPlanPlaceholder
            >
        >();
    }

    // The concrete plan fixture is intentionally not duplicated here.
    //
    // Integration tests must use the canonical distributed execution
    // plan fixtures supplied by crate::distributed::tests.
    //
    // A second test plan implementation would create unnecessary
    // competing test infrastructure.
    //
    // The placeholder below is never instantiated. It exists only
    // for compile-time trait-bound verification.
    struct TestPlanPlaceholder;

    impl crate::distributed::execution::executor::ExecutablePlan
        for TestPlanPlaceholder
    {
        type Work = ();

        fn plan_id(&self) -> crate::distributed::ids::PlanId {
            panic!("compile-time-only test fixture")
        }

        fn work_items(
            &self,
        ) -> &[Self::Work] {
            &[]
        }

        fn validate_for_execution(
            &self,
        ) -> crate::distributed::error::DistributedResult<()> {
            Ok(())
        }
    }
}
