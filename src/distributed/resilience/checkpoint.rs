
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! Durable, domain-aware checkpoint contracts for distributed execution.
//!
//! This module coordinates checkpoint creation, integrity verification,
//! restoration, and deletion through an injected storage provider.
//!
//! It does not implement a storage engine, serialize a competing Zamani
//! value representation, execute workloads, or perform recovery.
//!
//! # Architectural ownership
//!
//! Reuses:
//! - `distributed::ids` for checkpoint, task, and execution identities.
//! - `distributed::model::state` for logical state descriptors and versions.
//! - Domain/runtime providers for checkpointability and payload capture.
//! - The storage provider for durable persistence.
//! - `distributed::resilience::recovery` when recovery is implemented.
//!
//! Does not own:
//! - Source semantics, AST, or canonical IR.
//! - Quantum state representation or QEC.
//! - Physical hardware state or device allocation.
//! - Execution lifecycle or recovery decisions.
//! - A concrete storage engine or serialization format.
//!
//! # Safety
//!
//! Safe Rust only.

use core::fmt;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use sha2::{Digest, Sha256};

use crate::distributed::ids::{
    CheckpointId,
    ExecutionContextId,
    TaskId,
};
use crate::distributed::model::state::{
    CheckpointPolicy,
    DistributedStateDescriptor,
    StateVersion,
};

/// Stable identifier for this checkpoint contract.
pub const CHECKPOINT_SCHEMA_ID: &str =
    "zamani.distributed.resilience.checkpoint";

/// Version of this checkpoint manifest contract.
pub const CHECKPOINT_SCHEMA_VERSION: u16 = 1;

/// Boxed error returned by an external checkpoint provider.
pub type CheckpointProviderError =
    Box<dyn Error + Send + Sync + 'static>;

/// The digest algorithm used by this checkpoint contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckpointDigestAlgorithm {
    /// SHA-256.
    Sha256,
}

/// A content digest for a checkpoint payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckpointDigest {
    algorithm: CheckpointDigestAlgorithm,
    bytes: [u8; 32],
}

impl CheckpointDigest {
    /// Constructs a digest from an already computed SHA-256 value.
    ///
    /// The caller must ensure that the digest was computed over the exact
    /// payload that the storage provider will persist.
    #[must_use]
    pub const fn sha256(bytes: [u8; 32]) -> Self {
        Self {
            algorithm: CheckpointDigestAlgorithm::Sha256,
            bytes,
        }
    }

    /// Returns the digest algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> CheckpointDigestAlgorithm {
        self.algorithm
    }

    /// Returns the digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

/// Evidence that a particular execution boundary is safe to checkpoint.
///
/// This is an assertion supplied by the authoritative domain/runtime
/// provider. Constructing this value does not independently prove that
/// the provider is truthful or that the underlying state is valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointSafetyEvidence {
    domain: String,
    boundary: String,
    provider: String,
    checkpointable: bool,
    execution_quiescent: bool,
    side_effects_reconciled: bool,
}

impl CheckpointSafetyEvidence {
    /// Constructs checkpoint-safety evidence.
    ///
    /// `checkpointable` must be true only when the domain/runtime provider
    /// has established that the requested state can be captured at this
    /// boundary without violating its semantic guarantees.
    ///
    /// `execution_quiescent` must be true when the provider has established
    /// a consistent capture boundary.
    ///
    /// `side_effects_reconciled` must be true when externally observable
    /// effects are either included in the checkpoint contract or have been
    /// reconciled with their authoritative systems.
    ///
    /// # Errors
    ///
    /// Returns an error when an identifying field is invalid.
    pub fn new(
        domain: impl Into<String>,
        boundary: impl Into<String>,
        provider: impl Into<String>,
        checkpointable: bool,
        execution_quiescent: bool,
        side_effects_reconciled: bool,
    ) -> Result<Self, CheckpointError> {
        let domain = domain.into();
        let boundary = boundary.into();
        let provider = provider.into();

        validate_name("domain", &domain)?;
        validate_name("boundary", &boundary)?;
        validate_name("provider", &provider)?;

        Ok(Self {
            domain,
            boundary,
            provider,
            checkpointable,
            execution_quiescent,
            side_effects_reconciled,
        })
    }

    /// Returns the domain identifier.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the execution boundary identifier.
    #[must_use]
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns whether the provider permits checkpointing.
    #[must_use]
    pub const fn is_checkpointable(&self) -> bool {
        self.checkpointable
    }

    fn validate(&self) -> Result<(), CheckpointError> {
        if !self.checkpointable {
            return Err(CheckpointError::NotCheckpointable {
                domain: self.domain.clone(),
                boundary: self.boundary.clone(),
            });
        }

        if !self.execution_quiescent {
            return Err(CheckpointError::InconsistentCaptureBoundary);
        }

        if !self.side_effects_reconciled {
            return Err(CheckpointError::UnreconciledSideEffects);
        }

        Ok(())
    }
}

/// Logical state included in a checkpoint.
///
/// This references the existing distributed state model. It does not
/// contain or duplicate the state's payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointedState {
    state_id: crate::distributed::ids::DistributedStateId,
    version: StateVersion,
}

impl CheckpointedState {
    /// Creates checkpoint metadata from an existing state descriptor.
    ///
    /// A state explicitly marked `Prohibited` cannot be included.
    ///
    /// # Errors
    ///
    /// Returns an error if the descriptor is invalid or prohibits
    /// checkpointing.
    pub fn from_descriptor(
        descriptor: &DistributedStateDescriptor,
    ) -> Result<Self, CheckpointError> {
        descriptor
            .validate()
            .map_err(|error| CheckpointError::InvalidState {
                message: error.to_string(),
            })?;

        if descriptor.checkpoint_policy() == CheckpointPolicy::Prohibited {
            return Err(CheckpointError::StateCheckpointProhibited {
                state_id: descriptor.id().to_string(),
            });
        }

        Ok(Self {
            state_id: descriptor.id(),
            version: descriptor.version(),
        })
    }

    /// Returns the logical state identity.
    #[must_use]
    pub const fn state_id(
        &self,
    ) -> crate::distributed::ids::DistributedStateId {
        self.state_id
    }

    /// Returns the captured logical state version.
    #[must_use]
    pub const fn version(&self) -> StateVersion {
        self.version
    }
}

/// Immutable metadata describing one checkpoint.
///
/// This manifest deliberately contains no physical storage address,
/// host identifier, device identifier, network endpoint, or provider handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointManifest {
    schema_id: String,
    schema_version: u16,
    checkpoint_id: CheckpointId,
    task_id: TaskId,
    execution_context_id: ExecutionContextId,
    domain: String,
    boundary: String,
    provider: String,
    format: String,
    format_version: u32,
    states: Vec<CheckpointedState>,
    payload_digest: CheckpointDigest,
    payload_length: Option<u128>,
}

impl CheckpointManifest {
    /// Creates a validated checkpoint manifest.
    ///
    /// `states` may contain any number of state descriptors representable
    /// by the process and its available memory. No application-defined
    /// maximum is imposed.
    ///
    /// The caller must obtain the IDs from the existing distributed ID
    /// allocator and the digest from the authoritative payload producer.
    ///
    /// # Errors
    ///
    /// Returns an error when required metadata is invalid.
    pub fn new(
        checkpoint_id: CheckpointId,
        task_id: TaskId,
        execution_context_id: ExecutionContextId,
        safety: &CheckpointSafetyEvidence,
        format: impl Into<String>,
        format_version: u32,
        states: Vec<CheckpointedState>,
        payload_digest: CheckpointDigest,
        payload_length: Option<u128>,
    ) -> Result<Self, CheckpointError> {
        safety.validate()?;

        let format = format.into();
        validate_name("format", &format)?;

        Ok(Self {
            schema_id: CHECKPOINT_SCHEMA_ID.to_owned(),
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            checkpoint_id,
            task_id,
            execution_context_id,
            domain: safety.domain.clone(),
            boundary: safety.boundary.clone(),
            provider: safety.provider.clone(),
            format,
            format_version,
            states,
            payload_digest,
            payload_length,
        })
    }

    /// Returns the checkpoint identity.
    #[must_use]
    pub const fn checkpoint_id(&self) -> CheckpointId {
        self.checkpoint_id
    }

    /// Returns the logical task identity.
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    /// Returns the execution context identity.
    #[must_use]
    pub const fn execution_context_id(&self) -> ExecutionContextId {
        self.execution_context_id
    }

    /// Returns the checkpoint's domain.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Returns the capture boundary.
    #[must_use]
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Returns the provider identifier.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns the payload format identifier.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns the payload format version.
    #[must_use]
    pub const fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Returns the captured state metadata.
    #[must_use]
    pub fn states(&self) -> &[CheckpointedState] {
        &self.states
    }

    /// Returns the expected payload digest.
    #[must_use]
    pub const fn payload_digest(&self) -> &CheckpointDigest {
        &self.payload_digest
    }

    /// Returns the expected payload length, if known.
    #[must_use]
    pub const fn payload_length(&self) -> Option<u128> {
        self.payload_length
    }

    /// Returns the checkpoint schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Validates the manifest's internal compatibility and metadata.
    ///
    /// This does not prove that a provider supports the payload format.
    ///
    /// # Errors
    ///
    /// Returns an error for an unsupported manifest schema or invalid
    /// metadata.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.schema_id != CHECKPOINT_SCHEMA_ID
            || self.schema_version != CHECKPOINT_SCHEMA_VERSION
        {
            return Err(CheckpointError::IncompatibleSchema {
                schema_id: self.schema_id.clone(),
                version: self.schema_version,
            });
        }

        validate_name("domain", &self.domain)?;
        validate_name("boundary", &self.boundary)?;
        validate_name("provider", &self.provider)?;
        validate_name("format", &self.format)?;

        Ok(())
    }
}

/// A payload producer capable of streaming checkpoint data.
///
/// The implementation may read from memory, files, a device runtime,
/// a database, or another provider. It must not require the complete
/// checkpoint to be materialized in one in-memory byte vector.
pub trait CheckpointPayload: Send + Sync + fmt::Debug {
    /// Streams the payload to the supplied writer.
    ///
    /// The producer must write exactly the payload described by the
    /// manifest's digest and optional length.
    ///
    /// # Errors
    ///
    /// Returns the original provider error if capture or streaming fails.
    fn write_to(
        &self,
        writer: &mut dyn Write,
    ) -> Result<(), CheckpointProviderError>;
}

/// A target that stages restored payload data.
///
/// Implementations must keep staged data unpublished until `commit`
/// succeeds. `abort` must discard the staged result.
pub trait CheckpointRestoreTarget: fmt::Debug {
    /// Writes another portion of the restored payload into staging.
    fn write_chunk(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), CheckpointProviderError>;

    /// Makes the fully verified staged payload available to the caller.
    fn commit(&mut self) -> Result<(), CheckpointProviderError>;

    /// Discards any staged payload.
    fn abort(&mut self) -> Result<(), CheckpointProviderError>;
}

/// Capabilities advertised by a checkpoint storage provider.
///
/// The manager refuses to use a provider that cannot establish the
/// durability and atomicity guarantees required by this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointStoreCapabilities {
    /// The provider atomically creates complete checkpoint records.
    pub atomic_create: bool,

    /// Successful creation means the checkpoint is durably committed
    /// according to the provider's documented durability contract.
    pub durable_commit: bool,

    /// The provider can stream a checkpoint for integrity verification
    /// before the restore target is committed.
    pub verified_restore: bool,
}

/// Result of attempting to create a checkpoint with a particular ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointCreateOutcome {
    /// A new checkpoint was atomically created and durably committed.
    Created,

    /// A checkpoint with this ID already exists.
    ///
    /// The manager verifies that its manifest matches before treating
    /// this as an idempotent repeat.
    AlreadyExists,
}

/// Storage-provider contract for durable checkpoint records.
///
/// Implementations must:
///
/// - Atomically publish a complete manifest and payload.
/// - Verify the supplied payload digest and length before committing.
/// - Return success only after the documented durable-commit point.
/// - Never expose partially written checkpoints as complete.
/// - Preserve manifests and payloads across the failures covered by
///   their declared durability guarantee.
/// - Verify the stored payload while restoring.
/// - Never commit a restore target before integrity verification succeeds.
///
/// This is an interface, not a storage engine. A provider that cannot
/// meet these guarantees must not advertise the corresponding capability.
pub trait CheckpointStore: Send + Sync + fmt::Debug {
    /// Returns the provider's verified capabilities.
    fn capabilities(&self) -> CheckpointStoreCapabilities;

    /// Atomically creates a checkpoint.
    ///
    /// `AlreadyExists` must not overwrite the existing checkpoint.
    fn create_atomic(
        &self,
        manifest: &CheckpointManifest,
        payload: &dyn CheckpointPayload,
    ) -> Result<CheckpointCreateOutcome, CheckpointProviderError>;

    /// Loads the manifest for an existing checkpoint.
    fn load_manifest(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<Option<CheckpointManifest>, CheckpointProviderError>;

    /// Streams the payload into the supplied writer.
    ///
    /// The manager performs independent digest and length verification
    /// before committing the restore target.
    fn stream_payload(
        &self,
        checkpoint_id: CheckpointId,
        writer: &mut dyn Write,
    ) -> Result<(), CheckpointProviderError>;

    /// Deletes a checkpoint.
    ///
    /// Implementations must make deletion semantics explicit and must
    /// not report success when the checkpoint remains accessible.
    fn delete(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<bool, CheckpointProviderError>;
}

/// Checkpoint manager using an injected durable storage provider.
///
/// It is intentionally stateless. Storage consistency, concurrency control,
/// and durability are implemented by the provider.
#[derive(Debug)]
pub struct CheckpointManager<S> {
    store: S,
}

impl<S> CheckpointManager<S>
where
    S: CheckpointStore,
{
    /// Creates a manager after validating storage capabilities.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot guarantee atomic creation,
    /// durable commit, or verified restoration.
    pub fn new(store: S) -> Result<Self, CheckpointError> {
        let capabilities = store.capabilities();

        if !capabilities.atomic_create {
            return Err(CheckpointError::InsufficientStoreGuarantees {
                capability: "atomic_create",
            });
        }

        if !capabilities.durable_commit {
            return Err(CheckpointError::InsufficientStoreGuarantees {
                capability: "durable_commit",
            });
        }

        if !capabilities.verified_restore {
            return Err(CheckpointError::InsufficientStoreGuarantees {
                capability: "verified_restore",
            });
        }

        Ok(Self { store })
    }

    /// Creates a durable checkpoint.
    ///
    /// If the checkpoint ID already exists, the existing manifest must
    /// exactly match the requested manifest. A different manifest under
    /// the same ID is a conflict, never an overwrite.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid metadata, unsafe capture evidence,
    /// provider failure, or an ID collision.
    pub fn create(
        &self,
        manifest: &CheckpointManifest,
        payload: &dyn CheckpointPayload,
    ) -> Result<CheckpointCreateOutcome, CheckpointError> {
        manifest.validate()?;

        match self.store.create_atomic(manifest, payload)
            .map_err(|source| CheckpointError::Provider {
                operation: "create_atomic",
                source,
            })?
        {
            CheckpointCreateOutcome::Created => {
                let persisted = self.load_required_manifest(
                    manifest.checkpoint_id,
                )?;

                if persisted != *manifest {
                    return Err(CheckpointError::PersistedManifestMismatch);
                }

                Ok(CheckpointCreateOutcome::Created)
            }

            CheckpointCreateOutcome::AlreadyExists => {
                let existing = self.load_required_manifest(
                    manifest.checkpoint_id,
                )?;

                if existing != *manifest {
                    return Err(CheckpointError::IdentityConflict {
                        checkpoint_id: manifest.checkpoint_id.to_string(),
                    });
                }

                Ok(CheckpointCreateOutcome::AlreadyExists)
            }
        }
    }

    /// Restores a checkpoint into a staged target.
    ///
    /// Payload bytes are streamed; the entire checkpoint is not required
    /// to fit in memory. The target is committed only after the manager
    /// independently verifies the digest and optional payload length.
    ///
    /// On any failure, the manager attempts to abort the staged target.
    ///
    /// # Errors
    ///
    /// Returns an error if the checkpoint is absent, incompatible,
    /// corrupted, or the provider/target fails.
    pub fn restore(
        &self,
        checkpoint_id: CheckpointId,
        target: &mut dyn CheckpointRestoreTarget,
    ) -> Result<CheckpointManifest, CheckpointError> {
        let manifest = self.load_required_manifest(checkpoint_id)?;
        manifest.validate()?;

        let mut writer = VerifiedRestoreWriter::new(target);

        let stream_result = self.store.stream_payload(
            checkpoint_id,
            &mut writer,
        );

        if let Err(source) = stream_result {
            writer.abort();
            return Err(CheckpointError::Provider {
                operation: "stream_payload",
                source,
            });
        }

        let (digest, length) = writer.finish_digest();

        if digest != *manifest.payload_digest.as_bytes() {
            writer.abort();
            return Err(CheckpointError::DigestMismatch);
        }

        if let Some(expected_length) = manifest.payload_length {
            if length != expected_length {
                writer.abort();
                return Err(CheckpointError::PayloadLengthMismatch {
                    expected: expected_length,
                    actual: length,
                });
            }
        }

        if let Err(source) = writer.commit() {
            writer.abort();
            return Err(CheckpointError::Provider {
                operation: "restore_target_commit",
                source,
            });
        }

        Ok(manifest)
    }

    /// Deletes a checkpoint by its logical identity.
    ///
    /// Returns `true` only when the provider confirms that the checkpoint
    /// was deleted. Returns `false` when it was already absent.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot confirm deletion.
    pub fn delete(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<bool, CheckpointError> {
        self.store
            .delete(checkpoint_id)
            .map_err(|source| CheckpointError::Provider {
                operation: "delete",
                source,
            })
    }

    /// Returns the underlying storage provider.
    ///
    /// This is intended for provider-specific administration and
    /// observability. It does not expose mutable manager state.
    #[must_use]
    pub const fn store(&self) -> &S {
        &self.store
    }

    fn load_required_manifest(
        &self,
        checkpoint_id: CheckpointId,
    ) -> Result<CheckpointManifest, CheckpointError> {
        self.store
            .load_manifest(checkpoint_id)
            .map_err(|source| CheckpointError::Provider {
                operation: "load_manifest",
                source,
            })?
            .ok_or_else(|| CheckpointError::NotFound {
                checkpoint_id: checkpoint_id.to_string(),
            })
    }
}

/// Writer that computes an independent digest and length while streaming
/// restored payload data into a staging target.
#[derive(Debug)]
struct VerifiedRestoreWriter<'a> {
    target: &'a mut dyn CheckpointRestoreTarget,
    hasher: Sha256,
    length: u128,
    failed: bool,
}

impl<'a> VerifiedRestoreWriter<'a> {
    fn new(target: &'a mut dyn CheckpointRestoreTarget) -> Self {
        Self {
            target,
            hasher: Sha256::new(),
            length: 0,
            failed: false,
        }
    }

    fn finish_digest(self) -> ([u8; 32], u128) {
        let digest = self.hasher.finalize();
        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(&digest);
        (bytes, self.length)
    }

    fn commit(&mut self) -> Result<(), CheckpointProviderError> {
        self.target.commit()
    }

    fn abort(&mut self) {
        let _ = self.target.abort();
    }
}

impl Write for VerifiedRestoreWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.failed {
            return Err(io::Error::other(
                "checkpoint restore writer is already failed",
            ));
        }

        let next_length = self.length
            .checked_add(bytes.len() as u128)
            .ok_or_else(|| io::Error::other(
                "checkpoint payload length overflow",
            ))?;

        match self.target.write_chunk(bytes) {
            Ok(()) => {
                self.hasher.update(bytes);
                self.length = next_length;
                Ok(bytes.len())
            }
            Err(error) => {
                self.failed = true;
                Err(io::Error::other(error.to_string()))
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn validate_name(
    field: &'static str,
    value: &str,
) -> Result<(), CheckpointError> {
    if value.trim().is_empty() {
        return Err(CheckpointError::InvalidMetadata {
            field,
            reason: "must not be empty or whitespace-only",
        });
    }

    if value.chars().any(char::is_control) {
        return Err(CheckpointError::InvalidMetadata {
            field,
            reason: "must not contain control characters",
        });
    }

    Ok(())
}

/// Errors produced by checkpoint validation, storage, and restoration.
#[derive(Debug)]
#[non_exhaustive]
pub enum CheckpointError {
    /// A metadata field is invalid.
    InvalidMetadata {
        /// Name of the invalid field.
        field: &'static str,
        /// Reason the field is invalid.
        reason: &'static str,
    },

    /// The domain provider says the execution boundary cannot be checkpointed.
    NotCheckpointable {
        /// Domain identifier.
        domain: String,
        /// Execution boundary identifier.
        boundary: String,
    },

    /// The execution was not at a consistent capture boundary.
    InconsistentCaptureBoundary,

    /// External side effects have not been reconciled.
    UnreconciledSideEffects,

    /// The state descriptor is invalid.
    InvalidState {
        /// Validation explanation.
        message: String,
    },

    /// The state policy explicitly prohibits checkpointing.
    StateCheckpointProhibited {
        /// Logical state identifier.
        state_id: String,
    },

    /// The manifest schema is not supported.
    IncompatibleSchema {
        /// Schema identifier.
        schema_id: String,
        /// Observed schema version.
        version: u16,
    },

    /// The selected storage provider lacks a required guarantee.
    InsufficientStoreGuarantees {
        /// Missing capability name.
        capability: &'static str,
    },

    /// The checkpoint does not exist.
    NotFound {
        /// Requested checkpoint identifier.
        checkpoint_id: String,
    },

    /// The checkpoint ID already refers to different immutable metadata.
    IdentityConflict {
        /// Conflicting checkpoint identity.
        checkpoint_id: String,
    },

    /// The persisted manifest differs from the requested manifest.
    PersistedManifestMismatch,

    /// The payload digest does not match the manifest.
    DigestMismatch,

    /// The restored payload length does not match the manifest.
    PayloadLengthMismatch {
        /// Expected payload length.
        expected: u128,
        /// Actual payload length.
        actual: u128,
    },

    /// An external provider operation failed.
    Provider {
        /// Operation that failed.
        operation: &'static str,
        /// Original provider error.
        source: CheckpointProviderError,
    },
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetadata { field, reason } =>
                write!(f, "invalid checkpoint metadata field {field}: {reason}"),
            Self::NotCheckpointable { domain, boundary } =>
                write!(f, "domain {domain} does not permit checkpointing at {boundary}"),
            Self::InconsistentCaptureBoundary =>
                f.write_str("checkpoint capture is not at a consistent execution boundary"),
            Self::UnreconciledSideEffects =>
                f.write_str("checkpoint capture has unreconciled external side effects"),
            Self::InvalidState { message } =>
                write!(f, "invalid checkpoint state descriptor: {message}"),
            Self::StateCheckpointProhibited { state_id } =>
                write!(f, "state {state_id} explicitly prohibits checkpointing"),
            Self::IncompatibleSchema { schema_id, version } =>
                write!(f, "unsupported checkpoint schema {schema_id} version {version}"),
            Self::InsufficientStoreGuarantees { capability } =>
                write!(f, "checkpoint store lacks required guarantee: {capability}"),
            Self::NotFound { checkpoint_id } =>
                write!(f, "checkpoint {checkpoint_id} was not found"),
            Self::IdentityConflict { checkpoint_id } =>
                write!(f, "checkpoint identity conflict for {checkpoint_id}"),
            Self::PersistedManifestMismatch =>
                f.write_str("persisted checkpoint manifest does not match the requested manifest"),
            Self::DigestMismatch =>
                f.write_str("checkpoint payload digest verification failed"),
            Self::PayloadLengthMismatch { expected, actual } =>
                write!(f, "checkpoint payload length mismatch: expected {expected}, got {actual}"),
            Self::Provider { operation, source } =>
                write!(f, "checkpoint provider operation {operation} failed: {source}"),
        }
    }
}

impl Error for CheckpointError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Provider { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_metadata() {
        let result = validate_name("domain", "  ");
        assert!(matches!(
            result,
            Err(CheckpointError::InvalidMetadata { .. })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        let result = validate_name("provider", "provider\nname");
        assert!(matches!(
            result,
            Err(CheckpointError::InvalidMetadata { .. })
        ));
    }

    #[test]
    fn digest_algorithm_is_explicit() {
        let digest = CheckpointDigest::sha256([7_u8; 32]);
        assert_eq!(
            digest.algorithm(),
            CheckpointDigestAlgorithm::Sha256
        );
        assert_eq!(digest.as_bytes(), &[7_u8; 32]);
    }

    #[test]
    fn rejects_uncheckpointable_domain_evidence() {
        let evidence = CheckpointSafetyEvidence::new(
            "quantum",
            "mid_circuit",
            "provider",
            false,
            true,
            true,
        )
        .expect("metadata should be valid");

        assert!(matches!(
            evidence.validate(),
            Err(CheckpointError::NotCheckpointable { .. })
        ));
    }

    #[test]
    fn rejects_unreconciled_side_effects() {
        let evidence = CheckpointSafetyEvidence::new(
            "classical",
            "task_boundary",
            "provider",
            true,
            true,
            false,
        )
        .expect("metadata should be valid");

        assert!(matches!(
            evidence.validate(),
            Err(CheckpointError::UnreconciledSideEffects)
        ));
    }
}
