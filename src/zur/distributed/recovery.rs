
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Distributed Recovery Integration
//!
//! Path:
//!     src/zur/distributed/recovery.rs
//!
//! Rust:
//!     1.97+
//!
//! Edition:
//!     2021
//!
//! Safety:
//!     Safe Rust only.
//!
//! # Purpose
//!
//! Provides the ZUR integration boundary for canonical distributed
//! recovery.
//!
//! This module does not implement another recovery engine. It delegates
//! recovery lifecycle, durable journaling, action execution,
//! reconciliation and verification to:
//!
//!     crate::distributed::resilience::recovery
//!
//! # Ownership
//!
//! This module owns:
//!
//! - ZUR-facing distributed recovery coordination.
//! - ZUR error translation.
//! - Access to canonical distributed recovery operations.
//! - A stable ZUR integration entry point.
//!
//! This module does NOT own:
//!
//! - Recovery planning algorithms.
//! - Recovery action definitions.
//! - Distributed execution identities.
//! - Distributed recovery identities.
//! - Distributed checkpoint storage.
//! - Checkpoint serialization.
//! - Retry policies.
//! - Idempotency storage.
//! - Distributed membership.
//! - Failure detection.
//! - Distributed transport.
//! - Scheduling or placement.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware discovery.
//! - Quantum state serialization.
//! - Quantum error correction.
//! - Authentication or authorization.
//! - Actual execution-provider implementations.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! # Fundamental distinctions
//!
//! Recovery requested != recovery started.
//! Recovery started != recovery completed.
//! Recovery action completed != computation recovered.
//! Recovery completed != computation verified.
//! Recovery cancellation recorded != remote execution stopped.
//! Checkpoint reference != checkpoint validity.
//! Checkpoint validity != restore success.
//! Retry advice != authorization to retry.
//! Submission accepted != execution completed.
//! Unknown outcome != failed execution.
//!
//! # Recovery lifecycle
//!
//! This module delegates the canonical lifecycle:
//!
//!     Incident
//!        |
//!        v
//!     Planning
//!        |
//!        v
//!     PlanValidated
//!        |
//!        v
//!     Recovering
//!        |
//!        v
//!     Verifying
//!        |
//!        v
//!     Succeeded
//!
//! The canonical recovery engine persists lifecycle transitions
//! through an injected RecoveryJournal.
//!
//! # Crash consistency
//!
//! The canonical recovery journal must provide durable atomic
//! compare-and-swap updates.
//!
//! Recovery action identities must remain stable when an interrupted
//! recovery operation is reconciled.
//!
//! The recovery executor must deduplicate repeated action submissions
//! or authoritatively reconcile an interrupted action before allowing
//! resubmission.
//!
//! This adapter never automatically retries an uncertain action.
//!
//! # Cancellation
//!
//! ZUR execution sessions remain the authority for session cancellation.
//!
//! The canonical RecoveryExecutor must be implemented or adapted so
//! that its operations observe the appropriate session/runtime
//! cancellation context.
//!
//! The canonical recovery engine's cancel operation records a
//! recovery-lifecycle cancellation. It does not independently prove
//! that an external execution provider stopped running.
//!
//! # Quantum and heterogeneous execution
//!
//! Recovery delegates domain-specific state validation to the
//! canonical execution and checkpoint providers.
//!
//! This module never assumes that quantum state, hardware state,
//! accelerator state, foreign state or arbitrary provider state can
//! be serialized, transferred or reconstructed.
//!
//! # Scalability
//!
//! This module introduces no fixed maximum for:
//!
//! - Recovery incidents.
//! - Recovery plans.
//! - Recovery actions.
//! - Distributed participants.
//! - Execution contexts.
//! - Computational domains.
//! - Devices.
//! - Checkpoints.
//! - Recovery generations.
//! - Resources.
//!
//! Actual limits remain governed by available memory, durable storage,
//! provider capabilities, numeric representation, configured budgets,
//! deadlines and operating-system constraints.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Grammar authority
//!
//! Source-level recovery intent:
//!     grammar/execution/recovery.g4
//!
//! Resilience intent:
//!     grammar/execution/resilience.g4
//!
//! Checkpoint intent:
//!     grammar/execution/checkpointing.g4
//!
//! Distributed fault tolerance:
//!     grammar/distributed/fault-tolerance.g4
//!
//! Normative distributed semantics:
//!     grammar/spec/distributed.md
//!
//! Grammar expresses intent. This module does not parse or reinterpret
//! source declarations.
//!
//! # Integration
//!
//! Canonical recovery:
//!     crate::distributed::resilience::recovery
//!
//! Canonical checkpointing:
//!     crate::distributed::resilience::checkpoint
//!
//! Canonical retry evaluation:
//!     crate::distributed::resilience::retry
//!
//! Canonical distributed identities:
//!     crate::distributed::ids
//!
//! Canonical distributed errors:
//!     crate::distributed::error
//!
//! ZUR sessions:
//!     crate::zur::execution::session
//!
//! ZUR execution results:
//!     crate::zur::execution::result
//!
//! ZUR cancellation:
//!     crate::zur::execution::cancellation
//!
//! ZUR resource leases:
//!     crate::zur::resources::lease
//!
//! ZUR errors:
//!     crate::zur::error

use crate::distributed::error::{
    DistributedError,
    DistributedErrorKind,
};

use crate::distributed::ids::OperationId;

use crate::distributed::resilience::recovery::{
    RecoveryEngine,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// CANONICAL TYPE RE-EXPORTS
// ============================================================================

/// Canonical distributed recovery action.
pub use crate::distributed::resilience::recovery::RecoveryAction;

/// Canonical distributed recovery action identity.
pub use crate::distributed::resilience::recovery::RecoveryActionId;

/// Canonical distributed recovery action category.
pub use crate::distributed::resilience::recovery::RecoveryActionKind;

/// Canonical distributed recovery action lifecycle status.
pub use crate::distributed::resilience::recovery::RecoveryActionStatus;

/// Canonical durable recovery journal write result.
pub use crate::distributed::resilience::recovery::JournalWrite;

/// Canonical distributed recovery executor contract.
pub use crate::distributed::resilience::recovery::RecoveryExecutor;

/// Canonical distributed recovery incident.
pub use crate::distributed::resilience::recovery::RecoveryIncident;

/// Canonical distributed recovery incident classification.
pub use crate::distributed::resilience::recovery::RecoveryIncidentKind;

/// Canonical distributed recovery journal contract.
pub use crate::distributed::resilience::recovery::RecoveryJournal;

/// Canonical distributed recovery plan.
pub use crate::distributed::resilience::recovery::RecoveryPlan;

/// Canonical distributed recovery record.
pub use crate::distributed::resilience::recovery::RecoveryRecord;

/// Canonical distributed recovery lifecycle status.
///
/// The explicit name avoids confusion with the separate ZUR execution
/// result recovery-status model.
pub use crate::distributed::resilience::recovery::RecoveryStatus
    as DistributedRecoveryStatus;

/// Canonical distributed recovery schema identifier.
pub use crate::distributed::resilience::recovery::DISTRIBUTED_RECOVERY_SCHEMA_ID;

/// Canonical distributed recovery schema version.
pub use crate::distributed::resilience::recovery::DISTRIBUTED_RECOVERY_SCHEMA_VERSION;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.distributed.recovery";

// ============================================================================
// ERROR TRANSLATION
// ============================================================================

/// Converts a canonical distributed error into a structured ZUR error.
///
/// The original distributed error is preserved as the source error.
/// Its human-readable message is not used as a substitute for the
/// canonical source error.
///
/// Error classification is retained as accurately as the two
/// established error models allow.
fn map_distributed_error(
    error: DistributedError,
    operation: &'static str,
) -> ZurError {
    let message = error.to_string();

    let translated = match error.kind() {
        DistributedErrorKind::InvalidInput => {
            ZurError::invalid_input(message)
        }

        DistributedErrorKind::InvalidConfiguration => {
            ZurError::configuration(message)
        }

        DistributedErrorKind::SemanticInvalid
        | DistributedErrorKind::InvalidMessage => {
            ZurError::invalid_data(message)
        }

        DistributedErrorKind::CapabilityUnsatisfied => {
            ZurError::capability_unavailable(message)
        }

        DistributedErrorKind::ResourceUnsatisfied => {
            ZurError::resource_unavailable(message)
        }

        DistributedErrorKind::Unrepresentable
        | DistributedErrorKind::PlanningFailed
        | DistributedErrorKind::PlacementFailed
        | DistributedErrorKind::SchedulingFailed
        | DistributedErrorKind::RoutingFailed => {
            ZurError::infeasible(message)
        }

        DistributedErrorKind::UnsupportedOperation => {
            ZurError::unsupported(message)
        }

        DistributedErrorKind::ProtocolIncompatible => {
            ZurError::compatibility(message)
        }

        DistributedErrorKind::AuthenticationFailed
        | DistributedErrorKind::AuthorizationDenied => {
            ZurError::permission_denied(message)
        }

        DistributedErrorKind::DeadlineExceeded => {
            ZurError::timed_out(message)
        }

        DistributedErrorKind::Cancelled => {
            ZurError::cancelled(message)
        }

        DistributedErrorKind::Unavailable
        | DistributedErrorKind::RuntimeUnavailable => {
            ZurError::service_unavailable(message)
        }

        DistributedErrorKind::Conflict => {
            ZurError::compatibility(message)
        }

        // These failures may involve external side effects, partially
        // completed operations, uncertain outcomes, or recovery state.
        //
        // Preserve their original canonical classification in the
        // source chain rather than incorrectly claiming that they are
        // ordinary input or feasibility failures.
        DistributedErrorKind::TransportFailed
        | DistributedErrorKind::SerializationFailed
        | DistributedErrorKind::OperationOutcomeUnknown
        | DistributedErrorKind::ExecutionFailed
        | DistributedErrorKind::StateTransferFailed
        | DistributedErrorKind::RecoveryFailed
        | DistributedErrorKind::InternalInvariantViolation
        | DistributedErrorKind::ExternalFailure
        | DistributedErrorKind::Context => {
            ZurError::internal(message)
        }
    };

    translated
        .with_component(COMPONENT)
        .with_operation(operation)
        .with_source(error)
}

// ============================================================================
// ADAPTER
// ============================================================================

/// Stateless ZUR adapter for canonical distributed recovery.
///
/// The adapter owns no journal, execution provider, retry policy,
/// checkpoint store, recovery state or mutable global data.
///
/// All operational authorities are explicitly supplied to each call.
///
/// This permits multiple runtime instances to use different journals,
/// providers and execution environments without sharing hidden state.
#[derive(Debug, Default, Clone, Copy)]
pub struct DistributedRecoveryAdapter;

impl DistributedRecoveryAdapter {
    /// Creates a distributed recovery adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Loads the durable recovery record for a logical operation.
    ///
    /// The returned record is the canonical journal record.
    ///
    /// `None` means that the journal has no record for this operation.
    ///
    /// # Errors
    ///
    /// Returns a translated ZUR error if the canonical journal fails.
    pub fn load_record(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
    ) -> ZurResult<Option<RecoveryRecord>> {
        journal
            .load(operation_id)
            .map_err(|error| {
                map_distributed_error(error, "load_record")
            })
    }

    /// Registers a new recovery incident.
    ///
    /// Duplicate incident registration is rejected by the canonical
    /// recovery engine. The journal remains authoritative.
    ///
    /// # Errors
    ///
    /// Returns an error if the incident already exists, the journal
    /// cannot commit it, or concurrent registration conflicts.
    pub fn register_incident(
        &self,
        journal: &dyn RecoveryJournal,
        incident: RecoveryIncident,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .register_incident(
                journal,
                incident,
            )
            .map_err(|error| {
                map_distributed_error(error, "register_incident")
            })
    }

    /// Begins a recovery generation using a canonical recovery plan.
    ///
    /// The canonical executor validates the plan against current
    /// semantic, security, capability, resource, membership,
    /// checkpoint and execution assumptions.
    ///
    /// The adapter does not create or modify the recovery plan.
    ///
    /// # Errors
    ///
    /// Returns an error if the plan is stale, invalid, infeasible,
    /// unauthorized, or cannot be durably committed.
    pub fn begin_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        plan: RecoveryPlan,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .begin_recovery(
                journal,
                executor,
                plan,
            )
            .map_err(|error| {
                map_distributed_error(error, "begin_recovery")
            })
    }

    /// Executes a previously validated recovery plan.
    ///
    /// The canonical engine:
    ///
    /// - Persists action intent before submission.
    /// - Uses stable recovery action identities.
    /// - Reconciles interrupted actions before resubmission.
    /// - Preserves unknown outcomes.
    /// - Does not automatically retry uncertain operations.
    ///
    /// If this method returns an error, callers must reload the
    /// durable recovery record before deciding what to do next.
    ///
    /// An error does not prove that no recovery action was executed.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails, an outcome is uncertain,
    /// cancellation is recorded, or a journal transition conflicts.
    pub fn execute_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .execute_recovery(
                journal,
                executor,
                operation_id,
            )
            .map_err(|error| {
                map_distributed_error(error, "execute_recovery")
            })
    }

    /// Verifies the recovered computation and commits recovery success.
    ///
    /// The canonical executor must establish the required semantic
    /// invariants. Merely submitting or completing a recovery action
    /// is insufficient.
    ///
    /// # Errors
    ///
    /// Returns an error unless every recovery action is complete,
    /// the plan remains valid, and authoritative verification succeeds.
    pub fn verify_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .verify_recovery(
                journal,
                executor,
                operation_id,
            )
            .map_err(|error| {
                map_distributed_error(error, "verify_recovery")
            })
    }

    /// Marks a recovery lifecycle as requiring external reconciliation.
    ///
    /// This is appropriate when an executor, journal, provider or
    /// process failure prevents the outcome from being established.
    ///
    /// The record remains incomplete until the authoritative
    /// reconciliation procedure resolves the uncertainty.
    ///
    /// # Errors
    ///
    /// Returns an error if the journal cannot commit the transition
    /// or if the canonical lifecycle rejects the transition.
    pub fn require_reconciliation(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
        reason: impl Into<String>,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .require_reconciliation(
                journal,
                operation_id,
                reason,
            )
            .map_err(|error| {
                map_distributed_error(
                    error,
                    "require_reconciliation",
                )
            })
    }

    /// Records cancellation of a recovery lifecycle.
    ///
    /// This delegates to the canonical recovery engine.
    ///
    /// IMPORTANT:
    ///
    /// Recording recovery cancellation does not establish that an
    /// external distributed execution provider stopped its work.
    ///
    /// Actual cancellation must be propagated through the existing
    /// ZUR execution-session and provider cancellation contracts.
    ///
    /// # Errors
    ///
    /// Returns an error if the recovery lifecycle is already successful
    /// or the canonical journal cannot commit the cancellation.
    pub fn cancel_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
    ) -> ZurResult<RecoveryRecord> {
        RecoveryEngine::new()
            .cancel(
                journal,
                operation_id,
            )
            .map_err(|error| {
                map_distributed_error(error, "cancel_recovery")
            })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_is_stateless_and_constructible() {
        let first = DistributedRecoveryAdapter::new();
        let second = DistributedRecoveryAdapter::new();

        assert_eq!(
            core::mem::size_of_val(&first),
            core::mem::size_of_val(&second),
        );
    }

    #[test]
    fn distributed_error_translation_preserves_source() {
        let source = DistributedError::invalid_input(
            "invalid recovery input",
        );

        let translated = map_distributed_error(
            source,
            "test_translation",
        );

        assert_eq!(
            translated.component(),
            Some(COMPONENT),
        );

        assert_eq!(
            translated.operation(),
            Some("test_translation"),
        );

        assert!(
            translated.source_error().is_some(),
            "the canonical distributed error must remain available",
        );
    }

    #[test]
    fn uncertain_outcome_is_not_mapped_to_success() {
        let source = DistributedError::outcome_unknown(
            None,
            crate::distributed::error::DeliveryState::Unknown,
            "remote execution outcome is unknown",
        );

        let translated = map_distributed_error(
            source,
            "test_uncertain_outcome",
        );

        assert!(
            translated.source_error().is_some(),
        );

        assert!(
            translated.message().contains("unknown"),
        );
    }
}
