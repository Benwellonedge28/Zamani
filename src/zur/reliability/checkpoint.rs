
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Checkpoint Integration
//!
//! Path: src/zur/reliability/checkpoint.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the ZUR-facing integration boundary for checkpoint
//! creation, restoration, compatibility validation and deletion.
//!
//! This module coordinates existing checkpoint authorities.
//! It does not implement a second checkpoint storage engine.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! - crate::zur::execution::session
//!     Canonical execution sessions and cancellation.
//!
//! - crate::zur::ids
//!     Canonical runtime identities.
//!
//! - crate::zur::error
//!     Canonical ZUR errors.
//!
//! - crate::distributed::resilience::checkpoint
//!     Durable checkpoint manifests, storage, integrity and restoration.
//!
//! - crate::distributed::resilience::recovery
//!     Distributed recovery.
//!
//! - crate::quantum::resilience::checkpoint
//!     Quantum-specific checkpointing.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - ZUR checkpoint coordination.
//! - Execution-session binding.
//! - Authorization integration.
//! - Compatibility-verification integration.
//! - Session cancellation propagation.
//! - Checkpoint operation error translation.
//!
//! This module does NOT own:
//!
//! - Checkpoint serialization.
//! - Checkpoint storage.
//! - Checkpoint hashing algorithms.
//! - Checkpoint manifest schemas.
//! - Quantum-state serialization.
//! - Quantum error correction.
//! - Distributed recovery algorithms.
//! - Resource allocation.
//! - Provider discovery.
//! - Semantic checkpoint policies.
//!
//! SCALABILITY
//!
//! No application-defined maximum is imposed on:
//!
//! - Checkpoints.
//! - Execution sessions.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Payload lengths.
//! - Resource quantities.
//!
//! Streaming is delegated to the canonical checkpoint implementation.
//!
//! Actual limits depend on available memory, storage, provider capabilities,
//! configured budgets and physical resource availability.
//!
//! Literal infinite storage or computation is not promised.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No raw pointers.
//! No unsafe implementations.
//! No global mutable state.
//! No fabricated checkpoint success.
//! No implicit resource allocation.
//! No automatic recovery.
//!
//! IMPORTANT
//!
//! A successfully created checkpoint does not prove that a computation
//! has been recovered.
//!
//! A successfully restored payload does not independently prove that
//! execution can safely resume.
//!
//! Recovery and semantic verification remain separate operations.

use core::fmt;

use std::error::Error as StdError;

use crate::distributed::ids::{
    CheckpointId,
    ExecutionContextId,
    TaskId,
};

use crate::distributed::resilience::checkpoint::{
    CheckpointCreateOutcome,
    CheckpointError,
    CheckpointManager,
    CheckpointManifest,
    CheckpointPayload,
    CheckpointProviderError,
    CheckpointRestoreTarget,
    CheckpointStore,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::execution::session::ExecutionSession;

use crate::zur::ids::SessionId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.reliability.checkpoint";

// ============================================================================
// CHECKPOINT OPERATIONS
// ============================================================================

/// An operation performed through the ZUR checkpoint integration.
///
/// This enum describes checkpoint operations, not computational domains.
///
/// Future domains reuse the same operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CheckpointOperation {
    /// Create a checkpoint.
    Create,

    /// Restore a checkpoint into a staging target.
    Restore,

    /// Delete a checkpoint.
    Delete,
}

impl CheckpointOperation {
    /// Returns the stable operation identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Restore => "restore",
            Self::Delete => "delete",
        }
    }
}

// ============================================================================
// SESSION IDENTITY BINDING
// ============================================================================

/// Maps a ZUR execution session to existing distributed execution identities.
///
/// This adapter deliberately reuses distributed identities instead of
/// inventing another task or execution-context identity.
///
/// Implementations must obtain these identities from their authoritative
/// execution and distributed subsystems.
///
/// They must not infer identity from hardware, provider names or addresses.
pub trait CheckpointSessionBinding: Send + Sync + fmt::Debug {
    /// Returns the canonical distributed task identity.
    ///
    /// The mapping must correspond to the supplied ZUR session.
    fn task_id(
        &self,
        session: &ExecutionSession,
    ) -> ZurResult<TaskId>;

    /// Returns the canonical distributed execution-context identity.
    ///
    /// The mapping must correspond to the supplied ZUR session.
    fn execution_context_id(
        &self,
        session: &ExecutionSession,
    ) -> ZurResult<ExecutionContextId>;
}

// ============================================================================
// AUTHORIZATION
// ============================================================================

/// Authorization boundary for checkpoint operations.
///
/// Implementations must delegate to the existing ZUR security and
/// permission authorities.
///
/// This trait does not implement an independent permission system.
///
/// Authorization must be evaluated for the actual session, operation
/// and checkpoint identity.
///
/// Unknown authorization must be rejected.
pub trait CheckpointAuthorizer: Send + Sync + fmt::Debug {
    /// Authorizes a checkpoint operation.
    ///
    /// `checkpoint_id` is absent only when the checkpoint identity
    /// has not yet been established.
    ///
    /// Returns an error when the operation is not authorized.
    fn authorize(
        &self,
        session_id: &SessionId,
        operation: CheckpointOperation,
        checkpoint_id: Option<&CheckpointId>,
    ) -> ZurResult<()>;
}

// ============================================================================
// COMPATIBILITY VERIFICATION
// ============================================================================

/// Authoritative checkpoint compatibility verification boundary.
///
/// Implementations delegate to the appropriate existing compatibility,
/// artifact, domain and provider authorities.
///
/// Depending on the checkpoint, verification may include:
///
/// - Checkpoint manifest compatibility.
/// - Runtime ABI compatibility.
/// - Payload format compatibility.
/// - Execution-plan compatibility.
/// - Domain compatibility.
/// - Provider compatibility.
/// - Resource feasibility.
/// - Hardware and device compatibility.
/// - Security requirements.
/// - Quantum reconstruction requirements.
///
/// This trait does not establish compatibility merely because two
/// version numbers are equal.
///
/// It does not perform automatic migration.
pub trait CheckpointCompatibilityVerifier: Send + Sync + fmt::Debug {
    /// Verifies that a checkpoint can be restored for the supplied session.
    ///
    /// This must fail when required compatibility evidence is absent,
    /// stale, contradictory or insufficient.
    fn verify_restore(
        &self,
        session: &ExecutionSession,
        manifest: &CheckpointManifest,
    ) -> ZurResult<()>;
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Translates a canonical checkpoint error into a ZUR operational error.
///
/// The original checkpoint error remains available through the source chain.
fn map_checkpoint_error(
    error: CheckpointError,
    operation: &'static str,
) -> ZurError {
    let message = error.to_string();

    let translated = match &error {
        CheckpointError::InvalidMetadata { .. }
        | CheckpointError::InvalidState { .. }
        | CheckpointError::StateCheckpointProhibited { .. }
        | CheckpointError::NotCheckpointable { .. }
        | CheckpointError::InconsistentCaptureBoundary
        | CheckpointError::UnreconciledSideEffects => {
            ZurError::invalid_data(message)
        }

        CheckpointError::IncompatibleSchema { .. } => {
            ZurError::compatibility(message)
        }

        CheckpointError::InsufficientStoreGuarantees { .. } => {
            ZurError::service_unavailable(message)
        }

        CheckpointError::NotFound { .. } => {
            ZurError::invalid_input(message)
        }

        CheckpointError::IdentityConflict { .. }
        | CheckpointError::PersistedManifestMismatch => {
            ZurError::compatibility(message)
        }

        CheckpointError::DigestMismatch
        | CheckpointError::PayloadLengthMismatch { .. } => {
            ZurError::invalid_data(message)
        }

        CheckpointError::Provider { .. } => {
            ZurError::internal(message)
        }
    };

    translated
        .with_component(COMPONENT)
        .with_operation(operation)
        .with_source(error)
}

/// Translates a storage-provider failure into a ZUR error.
///
/// The original provider error is preserved.
fn map_provider_error(
    error: CheckpointProviderError,
    operation: &'static str,
) -> ZurError {
    ZurError::internal(
        format!("checkpoint provider failed during {operation}"),
    )
    .with_component(COMPONENT)
    .with_operation(operation)
    .with_source(error)
}

/// Creates a structured checkpoint integration error.
fn checkpoint_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::invalid_data(message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// SESSION VALIDATION
// ============================================================================

/// Validates that a checkpoint manifest belongs to the supplied session.
///
/// Distributed task and execution-context identities remain authoritative
/// in the distributed subsystem.
///
/// A session identity alone does not establish checkpoint ownership.
fn validate_session_binding(
    session: &ExecutionSession,
    binding: &dyn CheckpointSessionBinding,
    manifest: &CheckpointManifest,
    operation: &'static str,
) -> ZurResult<()> {
    session.check_cancelled()?;

    manifest
        .validate()
        .map_err(|error| map_checkpoint_error(error, operation))?;

    let expected_task = binding.task_id(session)?;

    let expected_context = binding.execution_context_id(session)?;

    if manifest.task_id() != expected_task {
        return Err(
            checkpoint_error(
                "checkpoint task identity does not match the execution session",
                operation,
            )
            .with_execution(session.id().to_string()),
        );
    }

    if manifest.execution_context_id() != expected_context {
        return Err(
            checkpoint_error(
                "checkpoint execution context does not match the execution session",
                operation,
            )
            .with_execution(session.id().to_string()),
        );
    }

    Ok(())
}

// ============================================================================
// RESTORE FAILURE PRESERVATION
// ============================================================================

/// Preserves a failed restoration and its failed cleanup.
///
/// The primary restoration failure and cleanup failure are distinct.
/// Neither is silently discarded.
#[derive(Debug)]
struct RestoreAbortFailure {
    primary: String,
    cleanup: CheckpointProviderError,
}

impl fmt::Display for RestoreAbortFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "checkpoint restoration failed: {}; staging cleanup also failed: {}",
            self.primary,
            self.cleanup,
        )
    }
}

impl StdError for RestoreAbortFailure {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.cleanup.as_ref())
    }
}

// ============================================================================
// SESSION-BOUND RESTORE TARGET
// ============================================================================

/// Adds cooperative session cancellation to an existing restore target.
///
/// The canonical distributed checkpoint manager still owns:
///
/// - Streaming.
/// - Digest calculation.
/// - Payload-length verification.
/// - Restore commit ordering.
///
/// This wrapper ensures cancellation is checked between streamed chunks
/// and immediately before publishing restored state.
struct SessionBoundRestoreTarget<'a> {
    session: &'a ExecutionSession,

    target: &'a mut dyn CheckpointRestoreTarget,

    cleanup_failure: Option<CheckpointProviderError>,
}

impl fmt::Debug for SessionBoundRestoreTarget<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SessionBoundRestoreTarget")
            .field("session_id", self.session.id())
            .field(
                "cleanup_failed",
                &self.cleanup_failure.is_some(),
            )
            .finish()
    }
}

impl<'a> SessionBoundRestoreTarget<'a> {
    fn new(
        session: &'a ExecutionSession,
        target: &'a mut dyn CheckpointRestoreTarget,
    ) -> Self {
        Self {
            session,
            target,
            cleanup_failure: None,
        }
    }

    fn cleanup_failure(
        &mut self,
    ) -> Option<CheckpointProviderError> {
        self.cleanup_failure.take()
    }
}

impl CheckpointRestoreTarget for SessionBoundRestoreTarget<'_> {
    fn write_chunk(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), CheckpointProviderError> {
        self.session
            .check_cancelled()
            .map_err(|error| {
                Box::new(error) as CheckpointProviderError
            })?;

        self.target.write_chunk(bytes)
    }

    fn commit(
        &mut self,
    ) -> Result<(), CheckpointProviderError> {
        self.session
            .check_cancelled()
            .map_err(|error| {
                Box::new(error) as CheckpointProviderError
            })?;

        self.target.commit()
    }

    fn abort(
        &mut self,
    ) -> Result<(), CheckpointProviderError> {
        match self.target.abort() {
            Ok(()) => Ok(()),

            Err(error) => {
                let message = error.to_string();

                self.cleanup_failure = Some(error);

                Err(Box::new(
                    std::io::Error::other(message),
                ))
            }
        }
    }
}

// ============================================================================
// ZUR CHECKPOINT ADAPTER
// ============================================================================

/// Stateless ZUR checkpoint integration adapter.
///
/// This adapter does not own a checkpoint store, checkpoint journal,
/// serialization engine, provider registry or global state.
///
/// All dependencies are explicitly supplied by the caller.
///
/// The underlying checkpoint manager remains the storage authority.
///
/// This allows independent runtime instances to use different checkpoint
/// providers and security configurations.
#[derive(Debug, Default, Clone, Copy)]
pub struct ZurCheckpointAdapter;

impl ZurCheckpointAdapter {
    /// Creates the checkpoint integration adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    // ========================================================================
    // CREATE
    // ========================================================================

    /// Creates a checkpoint for an existing ZUR execution session.
    ///
    /// The supplied manifest must already have been constructed from
    /// authoritative checkpoint-safety evidence.
    ///
    /// The canonical checkpoint manager verifies:
    ///
    /// - Manifest validity.
    /// - Checkpoint identity conflicts.
    /// - Atomic creation.
    /// - Payload integrity.
    /// - Durable commit.
    ///
    /// This method does not capture arbitrary execution state itself.
    ///
    /// The payload producer must represent a genuinely checkpointable
    /// and consistent execution boundary.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The session is cancelled.
    /// - The manifest belongs to another execution.
    /// - Authorization fails.
    /// - The checkpoint is incompatible.
    /// - The storage provider fails.
    /// - Integrity verification fails.
    /// - Durable creation cannot be established.
    pub fn create<S>(
        &self,
        manager: &CheckpointManager<S>,
        session: &ExecutionSession,
        binding: &dyn CheckpointSessionBinding,
        authorizer: &dyn CheckpointAuthorizer,
        manifest: &CheckpointManifest,
        payload: &dyn CheckpointPayload,
    ) -> ZurResult<CheckpointCreateOutcome>
    where
        S: CheckpointStore,
    {
        session.check_cancelled()?;

        validate_session_binding(
            session,
            binding,
            manifest,
            "create",
        )?;

        authorizer.authorize(
            session.id(),
            CheckpointOperation::Create,
            Some(&manifest.checkpoint_id()),
        )?;

        manager
            .create(manifest, payload)
            .map_err(|error| {
                map_checkpoint_error(error, "create")
            })
    }

    // ========================================================================
    // RESTORE
    // ========================================================================

    /// Restores a checkpoint into a staged target.
    ///
    /// Restoration does not itself resume execution.
    ///
    /// The sequence is:
    ///
    /// 1. Validate session cancellation.
    /// 2. Authorize restoration.
    /// 3. Load the existing manifest.
    /// 4. Validate session ownership.
    /// 5. Validate checkpoint compatibility.
    /// 6. Stream the payload into staging.
    /// 7. Verify payload digest and length.
    /// 8. Check cancellation before publication.
    /// 9. Commit the staged target.
    ///
    /// The underlying checkpoint manager performs the actual integrity
    /// verification and staged restoration.
    ///
    /// The supplied target must not publish partial state before commit.
    ///
    /// # Errors
    ///
    /// Returns an error if any validation, authorization, compatibility,
    /// storage, integrity, cancellation or restoration step fails.
    pub fn restore<S>(
        &self,
        manager: &CheckpointManager<S>,
        session: &ExecutionSession,
        binding: &dyn CheckpointSessionBinding,
        authorizer: &dyn CheckpointAuthorizer,
        compatibility: &dyn CheckpointCompatibilityVerifier,
        checkpoint_id: CheckpointId,
        target: &mut dyn CheckpointRestoreTarget,
    ) -> ZurResult<CheckpointManifest>
    where
        S: CheckpointStore,
    {
        session.check_cancelled()?;

        authorizer.authorize(
            session.id(),
            CheckpointOperation::Restore,
            Some(&checkpoint_id),
        )?;

        // Obtain the existing immutable manifest before restoration.
        // Compatibility must be checked before staged state is published.
        let manifest = manager
            .store()
            .load_manifest(checkpoint_id.clone())
            .map_err(|error| {
                map_provider_error(error, "load_manifest")
            })?
            .ok_or_else(|| {
                checkpoint_error(
                    format!(
                        "checkpoint {checkpoint_id} does not exist"
                    ),
                    "restore",
                )
            })?;

        validate_session_binding(
            session,
            binding,
            &manifest,
            "restore",
        )?;

        compatibility.verify_restore(
            session,
            &manifest,
        )?;

        session.check_cancelled()?;

        let mut guarded_target =
            SessionBoundRestoreTarget::new(session, target);

        let restoration = manager.restore(
            checkpoint_id,
            &mut guarded_target,
        );

        let cleanup_failure =
            guarded_target.cleanup_failure();

        match (restoration, cleanup_failure) {
            (Ok(restored_manifest), None) => {
                if restored_manifest != manifest {
                    return Err(
                        ZurError::compatibility(
                            "checkpoint manifest changed during restoration",
                        )
                        .with_component(COMPONENT)
                        .with_operation("restore"),
                    );
                }

                Ok(restored_manifest)
            }

            (Err(primary), None) => {
                Err(map_checkpoint_error(primary, "restore"))
            }

            (Ok(_), Some(cleanup)) => {
                Err(
                    ZurError::internal(
                        "checkpoint restoration cleanup failed",
                    )
                    .with_component(COMPONENT)
                    .with_operation("restore")
                    .with_source(cleanup),
                )
            }

            (Err(primary), Some(cleanup)) => {
                let combined = RestoreAbortFailure {
                    primary: primary.to_string(),
                    cleanup,
                };

                Err(
                    ZurError::internal(
                        "checkpoint restoration and staging cleanup both failed",
                    )
                    .with_component(COMPONENT)
                    .with_operation("restore")
                    .with_source(combined),
                )
            }
        }
    }

    // ========================================================================
    // DELETE
    // ========================================================================

    /// Deletes an existing checkpoint after authorization and ownership
    /// validation.
    ///
    /// The canonical storage provider remains responsible for deletion.
    ///
    /// Returns:
    ///
    /// - `true` when the provider confirms deletion.
    /// - `false` when the checkpoint was already absent.
    ///
    /// Deletion does not imply that execution has stopped or that
    /// recovery has completed.
    ///
    /// # Errors
    ///
    /// Returns an error when authorization fails, the checkpoint belongs
    /// to another session, or storage cannot confirm deletion.
    pub fn delete<S>(
        &self,
        manager: &CheckpointManager<S>,
        session: &ExecutionSession,
        binding: &dyn CheckpointSessionBinding,
        authorizer: &dyn CheckpointAuthorizer,
        checkpoint_id: CheckpointId,
    ) -> ZurResult<bool>
    where
        S: CheckpointStore,
    {
        session.check_cancelled()?;

        authorizer.authorize(
            session.id(),
            CheckpointOperation::Delete,
            Some(&checkpoint_id),
        )?;

        let manifest = manager
            .store()
            .load_manifest(checkpoint_id.clone())
            .map_err(|error| {
                map_provider_error(error, "load_manifest")
            })?;

        let Some(manifest) = manifest else {
            return Ok(false);
        };

        validate_session_binding(
            session,
            binding,
            &manifest,
            "delete",
        )?;

        session.check_cancelled()?;

        manager
            .delete(checkpoint_id)
            .map_err(|error| {
                map_checkpoint_error(error, "delete")
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
    fn adapter_is_stateless() {
        let first = ZurCheckpointAdapter::new();
        let second = ZurCheckpointAdapter::new();

        assert_eq!(
            core::mem::size_of_val(&first),
            core::mem::size_of_val(&second),
        );
    }

    #[test]
    fn checkpoint_operations_have_stable_names() {
        assert_eq!(
            CheckpointOperation::Create.as_str(),
            "create",
        );

        assert_eq!(
            CheckpointOperation::Restore.as_str(),
            "restore",
        );

        assert_eq!(
            CheckpointOperation::Delete.as_str(),
            "delete",
        );
    }

    #[test]
    fn checkpoint_operation_classification_is_not_domain_specific() {
        // Future computational domains reuse the same operation model.
        //
        // No domain inventory is required here.
        assert_ne!(
            CheckpointOperation::Create,
            CheckpointOperation::Restore,
        );
    }

    #[test]
    fn adapter_construction_does_not_initialize_global_state() {
        let _first = ZurCheckpointAdapter::new();
        let _second = ZurCheckpointAdapter::new();

        // Construction is intentionally side-effect-free.
    }
}
