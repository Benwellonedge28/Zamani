
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Instance
//!
//! Path: src/zur/execution/instance.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Represents one loaded program whose execution has been accepted by an
//! execution authority.
//!
//! An ExecutionInstance retains:
//!
//! - The canonical execution handle.
//! - The canonical loaded artifact.
//! - The execution session associated with the request.
//! - The canonical ZUR execution lifecycle state.
//! - The most recent execution observation.
//!
//! It is an ownership and observation boundary. It does not execute code,
//! select providers, allocate resources, schedule domain operations,
//! or implement provider-specific behavior.
//!
//! ## Authoritative contracts
//!
//! - zur::contracts::execution owns ExecutionRequest, ExecutionHandle,
//!   ExecutionState, ExecutionStatus and ExecutionObservation.
//! - zur::execution::session owns session lifecycle and cancellation.
//! - zur::execution::loader owns LoadedArtifact and its platform resources.
//! - zur::ids owns ExecutionId and SessionId.
//! - zur::error owns ZurError and ZurResult.
//!
//! ## Lifecycle
//!
//! Construction is permitted only after:
//!
//! 1. A structurally valid ExecutionRequest exists.
//! 2. The requested artifact has been validated and loaded.
//! 3. An execution authority has accepted the submission and supplied
//!    the corresponding ExecutionHandle.
//! 4. The associated ExecutionSession is active.
//!
//! The initial canonical execution state is Submitted because the
//! constructor requires the loaded artifact and accepted handle.
//!
//! Subsequent state changes are accepted only through observations
//! supplied by the execution authority.
//!
//! ## Ownership
//!
//! The instance owns its LoadedArtifact. The loaded artifact in turn
//! retains its platform handle and any service leases transferred to it.
//!
//! The execution authority must retain the instance, or another strong
//! owner of its loaded resources, until execution reaches a terminal state.
//! Dropping the last owner while execution is still active can release
//! resources; this type cannot forcibly prevent its owner from being dropped.
//!
//! ## Cancellation
//!
//! Cancellation remains cooperative and session-scoped through the
//! existing ExecutionSession and OperationContext.
//!
//! An instance does not turn a cancellation request into a claim that
//! the provider, process, device or remote computation has stopped.
//!
//! ## Scalability and safety
//!
//! No arbitrary limit is imposed on:
//!
//! - Execution instances.
//! - Sessions.
//! - Artifacts.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Resource quantities.
//!
//! Safe Rust only. No raw pointers, unsafe implementations, global
//! mutable state or fabricated execution success.

use core::fmt;
use std::sync::{Mutex, MutexGuard};

use crate::stdlib::context::OperationContext;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::contracts::execution::{
    ExecutionHandle,
    ExecutionObservation,
    ExecutionRequest,
    ExecutionState,
    ExecutionStatus,
};

use crate::zur::execution::loader::LoadedArtifact;
use crate::zur::execution::session::{
    ExecutionSession,
    SessionStatus,
};

use crate::zur::ids::{ExecutionId, SessionId};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

const COMPONENT: &str = "zur.execution.instance";

// =============================================================================
// INTERNAL STATE
// =============================================================================

/// Mutable observation state shared by the instance.
///
/// The canonical execution state is stored directly. This module does not
/// define a competing execution lifecycle.
struct InstanceInner {
    state: ExecutionState,
    observation: Option<ExecutionObservation>,
}

impl InstanceInner {
    fn new_submitted() -> ZurResult<Self> {
        let mut state = ExecutionState::new();

        // The constructor requires a successfully loaded artifact and an
        // execution handle that represents an accepted submission.
        //
        // These transitions record the stages established by those inputs;
        // they do not execute or submit the program themselves.
        state.transition(ExecutionStatus::Validating)?;
        state.transition(ExecutionStatus::Prepared)?;
        state.transition(ExecutionStatus::Submitted)?;

        Ok(Self {
            state,
            observation: None,
        })
    }
}

// =============================================================================
// SHARED INSTANCE STORAGE
// =============================================================================

/// Resources and operational state owned by one execution instance.
struct InstanceShared {
    session: ExecutionSession,
    handle: ExecutionHandle,
    loaded_artifact: LoadedArtifact,
    inner: Mutex<InstanceInner>,
}

// =============================================================================
// EXECUTION INSTANCE
// =============================================================================

/// An owned, session-associated loaded program execution.
///
/// The instance is created after the execution authority has accepted
/// submission. It retains the loaded artifact until the instance itself
/// is dropped.
///
/// This type is deliberately not `Clone`: duplicating the instance owner
/// would make resource-retention responsibility less obvious. Callers
/// needing shared access should place it behind an `Arc` and ensure that
/// the owning execution manager retains a strong reference until terminal
/// completion.
pub struct ExecutionInstance {
    shared: InstanceShared,
}

impl ExecutionInstance {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an instance from a request, accepted handle and loaded artifact.
    ///
    /// The request must identify the supplied session. The handle must
    /// identify the same execution, artifact reference and plan reference.
    /// The loaded artifact must have the ABI version and entry point selected
    /// by the request.
    ///
    /// The session must be active and not cancelled.
    ///
    /// This constructor does not independently authenticate the execution
    /// authority that produced the handle. The caller must only provide a
    /// handle after actual acceptance by that authority.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied objects disagree, if the session is
    /// not active, or if cancellation has been requested.
    pub fn new(
        session: ExecutionSession,
        request: &ExecutionRequest<'_>,
        handle: ExecutionHandle,
        loaded_artifact: LoadedArtifact,
    ) -> ZurResult<Self> {
        session.check_cancelled()?;

        if session.status()? != SessionStatus::Active {
            return Err(instance_state_error(
                "an execution instance requires an active session",
                request.execution_id(),
            ));
        }

        if request.session_id() != Some(session.id()) {
            return Err(instance_compatibility_error(
                "execution request does not identify the supplied session",
                request.execution_id(),
            ));
        }

        if request.execution_id() != handle.execution_id() {
            return Err(instance_compatibility_error(
                "execution request and accepted handle have different execution identities",
                request.execution_id(),
            ));
        }

        if request.artifact() != handle.artifact() {
            return Err(instance_compatibility_error(
                "execution request and accepted handle identify different artifacts",
                request.execution_id(),
            ));
        }

        if request.plan() != handle.plan() {
            return Err(instance_compatibility_error(
                "execution request and accepted handle identify different execution plans",
                request.execution_id(),
            ));
        }

        if loaded_artifact.abi().version() != request.abi_version() {
            return Err(instance_compatibility_error(
                "loaded artifact ABI version does not match the execution request",
                request.execution_id(),
            ));
        }

        if loaded_artifact.entry_point().symbol()
            != request.entry_point().symbol()
        {
            return Err(instance_compatibility_error(
                "loaded artifact entry point does not match the execution request",
                request.execution_id(),
            ));
        }

        if request.context().is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "execution request was cancelled before instance construction",
                )
                .with_component(COMPONENT)
                .with_operation("new")
                .with_execution(request.execution_id().to_string()),
            );
        }

        let inner = InstanceInner::new_submitted()?;

        Ok(Self {
            shared: InstanceShared {
                session,
                handle,
                loaded_artifact,
                inner: Mutex::new(inner),
            },
        })
    }

    // -------------------------------------------------------------------------
    // Canonical identity
    // -------------------------------------------------------------------------

    /// Returns the canonical execution identity.
    #[must_use]
    pub fn id(&self) -> &ExecutionId {
        self.shared.handle.execution_id()
    }

    /// Returns the canonical session identity.
    #[must_use]
    pub fn session_id(&self) -> &SessionId {
        self.shared.session.id()
    }

    /// Returns the execution session retained by this instance.
    #[must_use]
    pub fn session(&self) -> &ExecutionSession {
        &self.shared.session
    }

    // -------------------------------------------------------------------------
    // Canonical execution handle
    // -------------------------------------------------------------------------

    /// Returns the accepted canonical execution handle.
    ///
    /// The handle is evidence supplied by the execution authority. Its
    /// presence does not imply that execution has completed.
    #[must_use]
    pub fn handle(&self) -> &ExecutionHandle {
        &self.shared.handle
    }

    // -------------------------------------------------------------------------
    // Loaded artifact ownership
    // -------------------------------------------------------------------------

    /// Returns the loaded artifact retained by this instance.
    ///
    /// The returned reference does not transfer ownership. The instance
    /// retains the artifact and its platform/service resources.
    #[must_use]
    pub fn loaded_artifact(&self) -> &LoadedArtifact {
        &self.shared.loaded_artifact
    }

    /// Returns the canonical artifact identity of the loaded artifact.
    #[must_use]
    pub fn artifact_id(
        &self,
    ) -> &crate::compiler::artifacts::ArtifactId {
        self.shared.loaded_artifact.artifact_id()
    }

    // -------------------------------------------------------------------------
    // Operation context and cancellation
    // -------------------------------------------------------------------------

    /// Returns the immutable session-scoped operation context.
    ///
    /// It includes parent-runtime and session cancellation propagation.
    #[must_use]
    pub fn operation_context(&self) -> &OperationContext {
        self.shared.session.operation_context()
    }

    /// Returns whether the associated session or parent runtime
    /// has requested cancellation.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.shared.session.is_cancelled()
    }

    /// Checks the associated session and parent-runtime cancellation state.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        self.shared.session.check_cancelled()
    }

    // -------------------------------------------------------------------------
    // Execution state
    // -------------------------------------------------------------------------

    /// Returns the latest locally recorded canonical execution status.
    ///
    /// This is an observation of the latest state supplied to
    /// `record_observation`; it is not a live query of a remote provider.
    pub fn status(&self) -> ZurResult<ExecutionStatus> {
        Ok(self.lock_inner()?.state.status())
    }

    /// Returns the latest canonical execution state.
    pub fn state(&self) -> ZurResult<ExecutionState> {
        Ok(self.lock_inner()?.state)
    }

    /// Returns whether the latest recorded state is terminal.
    pub fn is_terminal(&self) -> ZurResult<bool> {
        Ok(self.lock_inner()?.state.is_terminal())
    }

    /// Returns the most recently accepted execution observation.
    ///
    /// `None` means that no observation has been recorded since the
    /// accepted submission represented by the initial state.
    pub fn observation(&self) -> ZurResult<Option<ExecutionObservation>> {
        Ok(self.lock_inner()?.observation.clone())
    }

    /// Records a state observation supplied by the execution authority.
    ///
    /// The observation must identify this exact execution. Its state must
    /// be a valid transition from the last locally recorded state.
    ///
    /// An identical repeated observation is idempotent. A different
    /// observation that repeats the same state is rejected rather than
    /// silently replacing the existing observation.
    ///
    /// This method validates identity and lifecycle consistency. It does
    /// not authenticate the source of the observation; that responsibility
    /// belongs to the execution engine/provider integration.
    ///
    /// Returns `true` when a new observation is recorded and `false` when
    /// the exact same observation was already recorded.
    pub fn record_observation(
        &self,
        observation: ExecutionObservation,
    ) -> ZurResult<bool> {
        if observation.execution_id() != self.id() {
            return Err(instance_compatibility_error(
                "execution observation identifies a different execution",
                self.id(),
            ));
        }

        // Do not accept new state after the owning session has fully closed.
        if self.shared.session.status()? == SessionStatus::Closed {
            return Err(instance_state_error(
                "cannot record an execution observation after its session is closed",
                self.id(),
            ));
        }

        let mut inner = self.lock_inner()?;

        if let Some(previous) = &inner.observation {
            if previous == &observation {
                return Ok(false);
            }

            if previous.state().status() == observation.state().status() {
                return Err(instance_state_error(
                    "a different observation cannot replace an observation at the same execution state",
                    self.id(),
                ));
            }
        } else if inner.state.status() == observation.state().status() {
            // The initial state is Submitted. Permit the first authority
            // observation to confirm that same state.
            inner.observation = Some(observation);
            return Ok(true);
        }

        let mut next_state = inner.state;
        next_state.transition(observation.state().status())?;

        if next_state != observation.state() {
            return Err(instance_compatibility_error(
                "execution observation contains an inconsistent canonical state",
                self.id(),
            ));
        }

        inner.state = next_state;
        inner.observation = Some(observation);

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Snapshot
    // -------------------------------------------------------------------------

    /// Returns an immutable snapshot of this instance's locally owned state.
    ///
    /// The snapshot does not prove that an external execution authority
    /// is currently available or that a remote operation has completed.
    pub fn snapshot(&self) -> ZurResult<ExecutionInstanceSnapshot> {
        let inner = self.lock_inner()?;

        Ok(ExecutionInstanceSnapshot {
            execution_id: self.id().clone(),
            session_id: self.session_id().clone(),
            status: inner.state.status(),
            terminal: inner.state.is_terminal(),
            has_observation: inner.observation.is_some(),
            artifact_id: self.artifact_id().to_string(),
        })
    }

    // -------------------------------------------------------------------------
    // Internal synchronization
    // -------------------------------------------------------------------------

    fn lock_inner(&self) -> ZurResult<MutexGuard<'_, InstanceInner>> {
        self.shared.inner.lock().map_err(|_| {
            ZurError::internal(
                "execution instance state lock was poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("lock_inner")
            .with_execution(self.id().to_string())
        })
    }
}

// =============================================================================
// INSTANCE SNAPSHOT
// =============================================================================

/// Immutable snapshot of one execution instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionInstanceSnapshot {
    execution_id: ExecutionId,
    session_id: SessionId,
    status: ExecutionStatus,
    terminal: bool,
    has_observation: bool,
    artifact_id: String,
}

impl ExecutionInstanceSnapshot {
    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the associated session identity.
    #[must_use]
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Returns the latest recorded execution status.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.status
    }

    /// Returns whether the latest recorded state is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.terminal
    }

    /// Returns whether an execution observation has been recorded.
    #[must_use]
    pub const fn has_observation(&self) -> bool {
        self.has_observation
    }

    /// Returns the canonical artifact identity as a string.
    #[must_use]
    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}

// =============================================================================
// ERROR HELPERS
// =============================================================================

fn instance_state_error(
    message: impl Into<String>,
    execution_id: &ExecutionId,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation("validate_instance_state")
        .with_execution(execution_id.to_string())
}

fn instance_compatibility_error(
    message: impl Into<String>,
    execution_id: &ExecutionId,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_instance_compatibility")
        .with_execution(execution_id.to_string())
}

// =============================================================================
// DEBUG
// =============================================================================

impl fmt::Debug for ExecutionInstance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = self.status();

        formatter
            .debug_struct("ExecutionInstance")
            .field("execution_id", self.id())
            .field("session_id", self.session_id())
            .field("status", &status)
            .field("artifact_id", &self.artifact_id())
            .finish_non_exhaustive()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_initial_state_is_submitted() {
        let inner = InstanceInner::new_submitted()
            .expect("valid canonical lifecycle transitions");

        assert_eq!(
            inner.state.status(),
            ExecutionStatus::Submitted,
        );

        assert!(!inner.state.is_terminal());
        assert!(inner.observation.is_none());
    }

    #[test]
    fn canonical_execution_state_rejects_terminal_reopening() {
        let mut state = ExecutionState::new();

        state
            .transition(ExecutionStatus::Validating)
            .expect("valid transition");

        state
            .transition(ExecutionStatus::Failed)
            .expect("valid terminal transition");

        assert!(
            state
                .transition(ExecutionStatus::Running)
                .is_err()
        );
    }

    #[test]
    fn canonical_execution_state_rejects_skipped_transitions() {
        let mut state = ExecutionState::new();

        assert!(
            state
                .transition(ExecutionStatus::Completed)
                .is_err()
        );
    }

    #[test]
    fn submitted_execution_can_enter_running_state() {
        let mut inner = InstanceInner::new_submitted()
            .expect("submitted state");

        inner
            .state
            .transition(ExecutionStatus::Running)
            .expect("submitted to running is valid");

        assert_eq!(
            inner.state.status(),
            ExecutionStatus::Running,
        );
    }

    #[test]
    fn submitted_execution_can_fail() {
        let mut inner = InstanceInner::new_submitted()
            .expect("submitted state");

        inner
            .state
            .transition(ExecutionStatus::Failed)
            .expect("submitted to failed is valid");

        assert!(inner.state.is_terminal());
    }
}
