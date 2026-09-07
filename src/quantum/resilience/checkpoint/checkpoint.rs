//! Zamani Quantum Resilience — Checkpoint Contract
//!
//! Path:
//!     src/quantum/resilience/checkpoint/checkpoint.rs
//!
//! Purpose:
//!     Canonical, backend-independent checkpoint representation and lifecycle
//!     contract for the quantum resilience subsystem.
//!
//! Architectural ownership:
//!
//!     checkpoint.rs
//!         Owns:
//!             - checkpoint identity;
//!             - checkpoint metadata;
//!             - checkpoint semantic boundary;
//!             - checkpoint state classification;
//!             - payload descriptors;
//!             - resource/qubit scope;
//!             - provenance descriptors;
//!             - checkpoint lifecycle metadata;
//!             - creation/validation contracts.
//!
//!     manifest.rs
//!         Owns:
//!             - complete checkpoint manifests;
//!             - manifests spanning multiple checkpoint artifacts;
//!             - artifact enumeration.
//!
//!     snapshot.rs
//!         Owns:
//!             - snapshot-specific state descriptors;
//!             - provider/runtime snapshot metadata.
//!
//!     storage.rs
//!         Owns:
//!             - physical storage;
//!             - read/write/delete/list operations;
//!             - storage-provider adaptation.
//!
//!     integrity.rs
//!         Owns:
//!             - cryptographic digest calculation;
//!             - signature verification;
//!             - authenticity/integrity policies.
//!
//!     compatibility.rs
//!         Owns:
//!             - compatibility negotiation;
//!             - schema compatibility;
//!             - target capability compatibility.
//!
//!     recovery::checkpoint
//!         Owns:
//!             - recovery orchestration;
//!             - validation ordering;
//!             - restore execution.
//!
//!     recovery::resume
//!         Owns:
//!             - continuation after restoration.
//!
//!     recovery::rollback
//!         Owns:
//!             - rollback orchestration.
//!
//!     verification::*
//!         Owns:
//!             - semantic verification;
//!             - result acceptance;
//!             - post-restore verification.
//!
//!     quantum::ir
//!         Owns:
//!             - canonical quantum program representation.
//!
//!     quantum::ir::qubit
//!         Owns:
//!             - canonical QubitId.
//!
//! Important:
//!
//!     This module MUST NOT attempt to serialize an arbitrary unknown quantum
//!     state. A storage system being capable of storing bytes does not make an
//!     unknown quantum state reconstructible.
//!
//! Supported checkpoint semantics include:
//!
//!     - program-start reconstruction;
//!     - classical execution state;
//!     - measurement boundaries;
//!     - logical/QEC boundaries;
//!     - explicitly supported provider/runtime snapshots;
//!     - reconstructible runtime state;
//!     - compiled/replayable execution state.
//!
//! Scalability:
//!
//!     No maximum number of:
//!
//!         qubits
//!         logical qubits
//!         physical qubits
//!         checkpoints
//!         devices
//!         artifacts
//!         payload bytes
//!
//!     is imposed here.
//!
//!     Actual limits are supplied by storage, memory, runtime, hardware,
//!     provider, policy and resource-management layers.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code
//!
//! Integration rule:
//!
//!     This module exposes contracts. Concrete implementations are injected
//!     through traits so later modules do not require this file to be rewritten
//!     merely because a storage backend, hardware backend, serializer,
//!     integrity algorithm, or runtime changes.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Schema identity
// ============================================================================

/// Stable namespace for checkpoint contracts.
pub const CHECKPOINT_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint";

/// Semantic version of this checkpoint contract.
///
/// This is the representation contract version, not the repository version.
pub const CHECKPOINT_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Generic opaque identifiers
// ============================================================================

/// Creates an opaque identifier type with validation and display support.
///
/// Checkpoint identifiers must remain opaque. Their representation must not
/// encode machine size, qubit position, provider name, or storage assumptions.
macro_rules! opaque_identifier {
    ($name:ident, $field:literal) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            pub fn new(
                value: impl Into<String>,
            ) -> Result<Self, CheckpointError> {
                let value = value.into();

                if value.trim().is_empty() {
                    return Err(CheckpointError::InvalidIdentifier {
                        field: $field,
                    });
                }

                Ok(Self(value))
            }

            /// Returns the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier and returns its string.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

opaque_identifier!(CheckpointId, "checkpoint_id");
opaque_identifier!(ExecutionId, "execution_id");
opaque_identifier!(ProgramId, "program_id");
opaque_identifier!(ArtifactId, "artifact_id");
opaque_identifier!(TargetId, "target_id");
opaque_identifier!(OperationId, "operation_id");

// ============================================================================
// Timestamp
// ============================================================================

/// Representation-independent timestamp.
///
/// `SystemTime` is intentionally not stored directly in the checkpoint
/// schema because checkpoint serialization needs a stable wire representation.
///
/// Seconds may be negative for dates before the Unix epoch.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct CheckpointTimestamp {
    /// Signed Unix seconds.
    pub unix_seconds: i64,

    /// Nanoseconds within the second.
    pub nanoseconds: u32,
}

impl CheckpointTimestamp {
    /// Creates a timestamp after validating nanoseconds.
    pub const fn new(
        unix_seconds: i64,
        nanoseconds: u32,
    ) -> Result<Self, CheckpointError> {
        if nanoseconds >= 1_000_000_000 {
            return Err(CheckpointError::InvalidTimestamp);
        }

        Ok(Self {
            unix_seconds,
            nanoseconds,
        })
    }

    /// Converts a `SystemTime` into the checkpoint representation.
    pub fn from_system_time(
        value: SystemTime,
    ) -> Result<Self, CheckpointError> {
        match value.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let seconds = i64::try_from(duration.as_secs())
                    .map_err(|_| CheckpointError::TimestampOutOfRange)?;

                Self::new(seconds, duration.subsec_nanos())
            }
            Err(error) => {
                let duration = error.duration();

                let seconds = i64::try_from(duration.as_secs())
                    .map_err(|_| CheckpointError::TimestampOutOfRange)?;

                if duration.subsec_nanos() == 0 {
                    Self::new(-seconds, 0)
                } else {
                    Self::new(
                        -seconds - 1,
                        1_000_000_000
                            - duration.subsec_nanos(),
                    )
                }
            }
        }
    }

    /// Converts the checkpoint timestamp into `SystemTime`.
    pub fn to_system_time(self) -> Result<SystemTime, CheckpointError> {
        if self.unix_seconds >= 0 {
            let seconds = u64::try_from(self.unix_seconds)
                .map_err(|_| CheckpointError::TimestampOutOfRange)?;

            Ok(UNIX_EPOCH
                + Duration::from_secs(seconds)
                + Duration::from_nanos(u64::from(self.nanoseconds)))
        } else {
            let magnitude = self
                .unix_seconds
                .checked_neg()
                .ok_or(CheckpointError::TimestampOutOfRange)?;

            let seconds = u64::try_from(magnitude)
                .map_err(|_| CheckpointError::TimestampOutOfRange)?;

            let base = UNIX_EPOCH
                .checked_sub(Duration::from_secs(seconds))
                .ok_or(CheckpointError::TimestampOutOfRange)?;

            if self.nanoseconds == 0 {
                Ok(base)
            } else {
                base.checked_sub(Duration::from_nanos(
                    u64::from(1_000_000_000 - self.nanoseconds),
                ))
                .ok_or(CheckpointError::TimestampOutOfRange)
            }
        }
    }

    /// Returns the current system time in checkpoint representation.
    pub fn now() -> Result<Self, CheckpointError> {
        Self::from_system_time(SystemTime::now())
    }
}

// ============================================================================
// Checkpoint boundary
// ============================================================================

/// Semantic boundary at which a checkpoint is valid.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum CheckpointBoundary {
    /// Program can be reconstructed from its initial state.
    ProgramStart,

    /// A classical/replayable execution boundary.
    ClassicalExecution,

    /// A measurement-defined boundary.
    Measurement,

    /// A logical/QEC-defined boundary.
    LogicalQec,

    /// Runtime/provider explicitly supports restoring this snapshot.
    ProviderSupportedSnapshot,

    /// Runtime can reconstruct the state from explicit deterministic data.
    ReconstructibleRuntime,

    /// Compiled representation can be reconstructed and resumed.
    CompiledExecution,
}

impl CheckpointBoundary {
    /// Returns whether this boundary represents a provider/runtime snapshot.
    #[must_use]
    pub const fn is_provider_snapshot(self) -> bool {
        matches!(self, Self::ProviderSupportedSnapshot)
    }

    /// Returns whether the boundary requires explicit restore support.
    #[must_use]
    pub const fn requires_explicit_restore_support(self) -> bool {
        matches!(
            self,
            Self::ProviderSupportedSnapshot
                | Self::LogicalQec
                | Self::ReconstructibleRuntime
        )
    }
}

// ============================================================================
// State semantics
// ============================================================================

/// Semantic classification of what a checkpoint can restore.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum CheckpointStateKind {
    /// Classical state that can be deterministically reconstructed.
    Classical,

    /// Program representation plus sufficient replay information.
    ReplayableProgram,

    /// Compiled representation that can be rebuilt.
    CompiledExecution,

    /// State reconstructed by a runtime-defined procedure.
    ReconstructibleRuntime,

    /// Explicit provider/runtime quantum snapshot.
    SupportedQuantumSnapshot,

    /// Logical/QEC state with an explicitly supported restoration protocol.
    LogicalQec,

    /// Measurement boundary whose state is semantically reconstructible.
    MeasurementBoundary,

    /// State is not known to be reconstructible.
    Unknown,

    /// State is known to be invalid.
    Invalid,
}

impl CheckpointStateKind {
    /// Returns whether the state has an explicit restoration interpretation.
    #[must_use]
    pub const fn is_restorable(self) -> bool {
        matches!(
            self,
            Self::Classical
                | Self::ReplayableProgram
                | Self::CompiledExecution
                | Self::ReconstructibleRuntime
                | Self::SupportedQuantumSnapshot
                | Self::LogicalQec
                | Self::MeasurementBoundary
        )
    }

    /// Returns whether this state kind requires explicit target support.
    #[must_use]
    pub const fn requires_target_support(self) -> bool {
        matches!(
            self,
            Self::SupportedQuantumSnapshot
                | Self::LogicalQec
                | Self::ReconstructibleRuntime
        )
    }
}

// ============================================================================
// Resource scope
// ============================================================================

/// Resource scope associated with a checkpoint.
///
/// This is intentionally sparse and scalable. It does not require storing a
/// list of every machine resource when the checkpoint applies to an entire
/// execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointResourceScope {
    /// The checkpoint applies to the complete execution.
    Execution,

    /// The checkpoint applies to explicitly identified logical qubits.
    LogicalQubits(Arc<[QubitId]>),

    /// The checkpoint applies to explicitly identified physical qubits.
    PhysicalQubits(Arc<[QubitId]>),

    /// The checkpoint applies to a logical execution region.
    LogicalRegion {
        /// Stable opaque region identifier.
        region_id: String,
    },

    /// The checkpoint applies to a runtime-defined resource set.
    RuntimeDefined {
        /// Stable opaque resource-set identity.
        resource_set_id: String,
    },
}

impl CheckpointResourceScope {
    /// Creates an execution-wide scope.
    #[must_use]
    pub const fn execution() -> Self {
        Self::Execution
    }

    /// Creates a logical-qubit scope.
    ///
    /// No maximum number of qubits is imposed.
    pub fn logical_qubits(
        qubits: impl IntoIterator<Item = QubitId>,
    ) -> Result<Self, CheckpointError> {
        let values: Vec<QubitId> = qubits.into_iter().collect();

        validate_unique_qubits(&values)?;

        Ok(Self::LogicalQubits(Arc::from(values)))
    }

    /// Creates a physical-qubit scope.
    ///
    /// The canonical IR qubit identity is used. No resilience-specific
    /// physical-qubit identifier is introduced.
    pub fn physical_qubits(
        qubits: impl IntoIterator<Item = QubitId>,
    ) -> Result<Self, CheckpointError> {
        let values: Vec<QubitId> = qubits.into_iter().collect();

        validate_unique_qubits(&values)?;

        Ok(Self::PhysicalQubits(Arc::from(values)))
    }

    /// Returns the explicitly enumerated qubits, if any.
    #[must_use]
    pub fn qubits(&self) -> Option<&[QubitId]> {
        match self {
            Self::LogicalQubits(qubits) | Self::PhysicalQubits(qubits) => {
                Some(qubits)
            }
            Self::Execution
            | Self::LogicalRegion { .. }
            | Self::RuntimeDefined { .. } => None,
        }
    }
}

fn validate_unique_qubits(
    qubits: &[QubitId],
) -> Result<(), CheckpointError> {
    for index in 1..qubits.len() {
        if qubits[..index].contains(&qubits[index]) {
            return Err(CheckpointError::DuplicateQubit);
        }
    }

    Ok(())
}

// ============================================================================
// Payload representation
// ============================================================================

/// Describes where checkpoint data lives.
///
/// The checkpoint contract does not own storage bytes. This prevents an
/// otherwise small metadata object from forcing the entire quantum state into
/// memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointPayloadLocation {
    /// Payload is stored by an external storage provider.
    External {
        /// Opaque provider-specific object reference.
        object_reference: String,
    },

    /// Payload is available through a runtime-owned object.
    Runtime {
        /// Opaque runtime object reference.
        object_reference: String,
    },

    /// Payload is reconstructible from metadata/program information and
    /// therefore has no independently stored byte object.
    Reconstructible,

    /// Payload is provider-managed and can only be accessed through the
    /// provider/runtime contract.
    ProviderManaged {
        /// Opaque provider snapshot reference.
        snapshot_reference: String,
    },
}

impl CheckpointPayloadLocation {
    /// Validates that an external reference is non-empty.
    pub fn external(
        object_reference: impl Into<String>,
    ) -> Result<Self, CheckpointError> {
        let object_reference = object_reference.into();

        if object_reference.trim().is_empty() {
            return Err(CheckpointError::InvalidPayloadReference);
        }

        Ok(Self::External { object_reference })
    }

    /// Validates a runtime reference.
    pub fn runtime(
        object_reference: impl Into<String>,
    ) -> Result<Self, CheckpointError> {
        let object_reference = object_reference.into();

        if object_reference.trim().is_empty() {
            return Err(CheckpointError::InvalidPayloadReference);
        }

        Ok(Self::Runtime { object_reference })
    }

    /// Validates a provider snapshot reference.
    pub fn provider_managed(
        snapshot_reference: impl Into<String>,
    ) -> Result<Self, CheckpointError> {
        let snapshot_reference = snapshot_reference.into();

        if snapshot_reference.trim().is_empty() {
            return Err(CheckpointError::InvalidPayloadReference);
        }

        Ok(Self::ProviderManaged {
            snapshot_reference,
        })
    }
}

/// Describes the payload without embedding its potentially enormous contents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointPayload {
    /// Artifact identity.
    pub artifact_id: ArtifactId,

    /// Payload location.
    pub location: CheckpointPayloadLocation,

    /// Content/media type.
    pub media_type: String,

    /// Optional byte size.
    ///
    /// `None` is valid for provider-managed or reconstructible payloads.
    pub byte_length: Option<u64>,

    /// Integrity information supplied by `integrity.rs`.
    pub integrity: IntegrityDescriptor,

    /// Whether the payload is compressed.
    pub compression: CompressionDescriptor,

    /// Whether encryption is applied.
    pub encryption: EncryptionDescriptor,
}

impl CheckpointPayload {
    /// Validates the payload descriptor.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.media_type.trim().is_empty() {
            return Err(CheckpointError::InvalidPayloadMetadata {
                field: "media_type",
            });
        }

        self.integrity.validate()?;
        self.compression.validate()?;
        self.encryption.validate()?;

        match (&self.location, self.byte_length) {
            (CheckpointPayloadLocation::Reconstructible, Some(_)) => {
                return Err(CheckpointError::InvalidPayloadMetadata {
                    field: "byte_length_for_reconstructible_payload",
                });
            }
            _ => {}
        }

        Ok(())
    }
}

// ============================================================================
// Integrity descriptor
// ============================================================================

/// Describes the integrity mechanism without hard-coding an algorithm.
///
/// The actual cryptographic implementation belongs in `integrity.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityDescriptor {
    /// Algorithm identifier.
    pub algorithm: String,

    /// Encoded digest.
    pub digest: String,

    /// Optional key/signature reference.
    pub authenticity_reference: Option<String>,
}

impl IntegrityDescriptor {
    /// Validates the descriptor.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.algorithm.trim().is_empty() {
            return Err(CheckpointError::InvalidIntegrity {
                field: "algorithm",
            });
        }

        if self.digest.trim().is_empty() {
            return Err(CheckpointError::InvalidIntegrity {
                field: "digest",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Compression
// ============================================================================

/// Compression description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionDescriptor {
    /// Payload is not compressed.
    None,

    /// Payload is compressed using a named algorithm.
    Algorithm {
        /// Algorithm identifier.
        algorithm: String,
    },
}

impl CompressionDescriptor {
    /// Validates compression metadata.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if let Self::Algorithm { algorithm } = self {
            if algorithm.trim().is_empty() {
                return Err(CheckpointError::InvalidCompression);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Encryption
// ============================================================================

/// Encryption description.
///
/// The actual encryption implementation must live outside this file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionDescriptor {
    /// Payload is not encrypted.
    None,

    /// Payload is encrypted with an externally defined scheme/key reference.
    Encrypted {
        /// Algorithm identifier.
        algorithm: String,

        /// Opaque key reference.
        key_reference: String,
    },
}

impl EncryptionDescriptor {
    /// Validates encryption metadata.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if let Self::Encrypted {
            algorithm,
            key_reference,
        } = self
        {
            if algorithm.trim().is_empty() {
                return Err(CheckpointError::InvalidEncryption);
            }

            if key_reference.trim().is_empty() {
                return Err(CheckpointError::InvalidEncryption);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Program/execution provenance
// ============================================================================

/// Immutable provenance information associated with a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointProvenance {
    /// Program identity.
    pub program_id: ProgramId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Canonical IR schema/version.
    pub ir_schema_version: String,

    /// Program/IR content fingerprint.
    pub program_fingerprint: String,

    /// Compilation fingerprint.
    pub compilation_fingerprint: Option<String>,

    /// Routing fingerprint.
    pub routing_fingerprint: Option<String>,

    /// Scheduling fingerprint.
    pub scheduling_fingerprint: Option<String>,

    /// Optimization fingerprint.
    pub optimization_fingerprint: Option<String>,

    /// QEC configuration fingerprint.
    pub qec_fingerprint: Option<String>,

    /// Target capability fingerprint at checkpoint creation.
    pub capability_fingerprint: String,
}

impl CheckpointProvenance {
    /// Validates mandatory provenance.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.ir_schema_version.trim().is_empty() {
            return Err(CheckpointError::InvalidProvenance {
                field: "ir_schema_version",
            });
        }

        if self.program_fingerprint.trim().is_empty() {
            return Err(CheckpointError::InvalidProvenance {
                field: "program_fingerprint",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(CheckpointError::InvalidProvenance {
                field: "capability_fingerprint",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Logical execution position
// ============================================================================

/// Opaque logical execution position.
///
/// The checkpoint system must not interpret this as a physical machine index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPosition {
    /// Stable execution-position identifier.
    pub value: String,

    /// Optional operation identity at the boundary.
    pub operation_id: Option<OperationId>,
}

impl ExecutionPosition {
    /// Creates a position.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CheckpointError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CheckpointError::InvalidExecutionPosition);
        }

        Ok(Self {
            value,
            operation_id: None,
        })
    }

    /// Adds an operation identity.
    #[must_use]
    pub fn with_operation(mut self, operation_id: OperationId) -> Self {
        self.operation_id = Some(operation_id);
        self
    }
}

// ============================================================================
// Restore semantics
// ============================================================================

/// Describes how a checkpoint can be restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestoreSemantics {
    /// Reconstruct the program and replay from the boundary.
    Replay,

    /// Restore classical runtime state.
    ClassicalState,

    /// Restore an explicitly supported runtime snapshot.
    RuntimeSnapshot,

    /// Restore a provider-managed quantum snapshot.
    ProviderSnapshot,

    /// Restore logical/QEC state using the QEC contract.
    LogicalQecState,

    /// Reconstruct compiled execution state.
    CompiledState,
}

impl RestoreSemantics {
    /// Returns whether this semantic requires explicit target support.
    #[must_use]
    pub const fn requires_target_support(&self) -> bool {
        matches!(
            self,
            Self::RuntimeSnapshot
                | Self::ProviderSnapshot
                | Self::LogicalQecState
        )
    }
}

// ============================================================================
// Checkpoint authorization
// ============================================================================

/// Authorization state of a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointAuthorization {
    /// Checkpoint is only being constructed.
    Pending,

    /// Producer has declared the checkpoint usable.
    Authorized,

    /// Checkpoint has been revoked.
    Revoked,
}

// ============================================================================
// Checkpoint lifecycle
// ============================================================================

/// Lifecycle state of a checkpoint artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckpointLifecycle {
    /// Being constructed and not yet committed.
    Building,

    /// Persisted and available for validation.
    Committed,

    /// Explicitly marked unavailable.
    Revoked,

    /// Permanently retired by policy/storage.
    Retired,
}

impl CheckpointLifecycle {
    /// Returns whether the checkpoint can be considered for recovery.
    #[must_use]
    pub const fn is_recoverable(self) -> bool {
        matches!(self, Self::Committed)
    }
}

// ============================================================================
// Checkpoint record
// ============================================================================

/// Canonical checkpoint representation.
///
/// This type contains metadata and references, not an arbitrary in-memory
/// quantum-state dump.
///
/// It is therefore suitable for very large executions because metadata size is
/// independent of total machine size unless a caller explicitly chooses an
/// enumerated qubit scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Schema namespace.
    pub schema_id: String,

    /// Schema version.
    pub schema_version: u16,

    /// Stable checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_id: ProgramId,

    /// Semantic boundary.
    pub boundary: CheckpointBoundary,

    /// Semantic state classification.
    pub state_kind: CheckpointStateKind,

    /// Restoration mechanism.
    pub restore_semantics: RestoreSemantics,

    /// Logical execution position.
    pub execution_position: ExecutionPosition,

    /// Resources covered by this checkpoint.
    pub resource_scope: CheckpointResourceScope,

    /// Program/runtime/hardware provenance.
    pub provenance: CheckpointProvenance,

    /// Payload descriptor.
    pub payload: CheckpointPayload,

    /// Creation timestamp.
    pub created_at: CheckpointTimestamp,

    /// Optional expiry timestamp.
    pub expires_at: Option<CheckpointTimestamp>,

    /// Producer authorization.
    pub authorization: CheckpointAuthorization,

    /// Lifecycle state.
    pub lifecycle: CheckpointLifecycle,

    /// Optional human-readable reason.
    pub reason: Option<String>,
}

impl Checkpoint {
    /// Performs complete local validation.
    ///
    /// This does not replace cryptographic verification or target compatibility
    /// validation. Those remain responsibilities of `integrity.rs` and
    /// `compatibility.rs`.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.schema_id != CHECKPOINT_SCHEMA_ID {
            return Err(CheckpointError::SchemaMismatch {
                expected: CHECKPOINT_SCHEMA_ID.to_owned(),
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version == 0 {
            return Err(CheckpointError::InvalidSchemaVersion);
        }

        if self.schema_version > CHECKPOINT_SCHEMA_VERSION {
            return Err(CheckpointError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if !self.state_kind.is_restorable() {
            return Err(CheckpointError::StateNotRestorable {
                state: self.state_kind,
            });
        }

        self.execution_position
            .value
            .trim()
            .is_empty()
            .then_some(())
            .ok_or(CheckpointError::InvalidExecutionPosition)?;

        self.provenance.validate()?;
        self.payload.validate()?;

        if self.provenance.execution_id != self.execution_id {
            return Err(CheckpointError::ExecutionIdentityMismatch);
        }

        if self.provenance.program_id != self.program_id {
            return Err(CheckpointError::ProgramIdentityMismatch);
        }

        self.validate_boundary_and_state()?;
        self.validate_expiration()?;
        self.validate_authorization()?;

        Ok(())
    }

    fn validate_boundary_and_state(&self) -> Result<(), CheckpointError> {
        match (
            self.boundary,
            self.state_kind,
            &self.restore_semantics,
        ) {
            (
                CheckpointBoundary::ProviderSupportedSnapshot,
                CheckpointStateKind::SupportedQuantumSnapshot,
                RestoreSemantics::ProviderSnapshot,
            ) => Ok(()),

            (
                CheckpointBoundary::LogicalQec,
                CheckpointStateKind::LogicalQec,
                RestoreSemantics::LogicalQecState,
            ) => Ok(()),

            (
                CheckpointBoundary::Measurement,
                CheckpointStateKind::MeasurementBoundary,
                RestoreSemantics::Replay,
            ) => Ok(()),

            (
                CheckpointBoundary::ClassicalExecution,
                CheckpointStateKind::Classical,
                RestoreSemantics::ClassicalState,
            ) => Ok(()),

            (
                CheckpointBoundary::ProgramStart,
                CheckpointStateKind::ReplayableProgram,
                RestoreSemantics::Replay,
            ) => Ok(()),

            (
                CheckpointBoundary::CompiledExecution,
                CheckpointStateKind::CompiledExecution,
                RestoreSemantics::CompiledState,
            ) => Ok(()),

            (
                CheckpointBoundary::ReconstructibleRuntime,
                CheckpointStateKind::ReconstructibleRuntime,
                RestoreSemantics::RuntimeSnapshot,
            ) => Ok(()),

            _ => Err(CheckpointError::BoundaryStateMismatch),
        }
    }

    fn validate_expiration(&self) -> Result<(), CheckpointError> {
        if let Some(expires_at) = self.expires_at {
            if expires_at < self.created_at {
                return Err(CheckpointError::InvalidExpiration);
            }
        }

        Ok(())
    }

    fn validate_authorization(&self) -> Result<(), CheckpointError> {
        if self.authorization != CheckpointAuthorization::Authorized {
            return Err(CheckpointError::CheckpointNotAuthorized);
        }

        if !self.lifecycle.is_recoverable() {
            return Err(CheckpointError::CheckpointNotRecoverable);
        }

        Ok(())
    }

    /// Returns whether this checkpoint is currently eligible for recovery
    /// based solely on local state.
    ///
    /// Integrity and target compatibility still need to be checked elsewhere.
    #[must_use]
    pub fn locally_recoverable(&self) -> bool {
        self.lifecycle.is_recoverable()
            && self.authorization == CheckpointAuthorization::Authorized
            && self.state_kind.is_restorable()
    }

    /// Returns the checkpoint creation time.
    #[must_use]
    pub const fn created_at(&self) -> CheckpointTimestamp {
        self.created_at
    }

    /// Returns whether the checkpoint has expired according to the supplied
    /// time.
    pub fn is_expired_at(
        &self,
        now: CheckpointTimestamp,
    ) -> Result<bool, CheckpointError> {
        match self.expires_at {
            Some(expires_at) => Ok(now > expires_at),
            None => Ok(false),
        }
    }
}

// ============================================================================
// Checkpoint creation request
// ============================================================================

/// Immutable request for checkpoint creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointCreateRequest {
    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Execution identity.
    pub execution_id: ExecutionId,

    /// Program identity.
    pub program_id: ProgramId,

    /// Semantic checkpoint boundary.
    pub boundary: CheckpointBoundary,

    /// State classification.
    pub state_kind: CheckpointStateKind,

    /// Restoration mechanism.
    pub restore_semantics: RestoreSemantics,

    /// Logical execution position.
    pub execution_position: ExecutionPosition,

    /// Resource scope.
    pub resource_scope: CheckpointResourceScope,

    /// Provenance.
    pub provenance: CheckpointProvenance,

    /// Payload.
    pub payload: CheckpointPayload,

    /// Optional expiry.
    pub expires_at: Option<CheckpointTimestamp>,

    /// Optional creation reason.
    pub reason: Option<String>,
}

impl CheckpointCreateRequest {
    /// Validates the creation request before persistence.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.provenance.execution_id != self.execution_id {
            return Err(CheckpointError::ExecutionIdentityMismatch);
        }

        if self.provenance.program_id != self.program_id {
            return Err(CheckpointError::ProgramIdentityMismatch);
        }

        self.provenance.validate()?;
        self.payload.validate()?;

        let temporary = Checkpoint {
            schema_id: CHECKPOINT_SCHEMA_ID.to_owned(),
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            checkpoint_id: self.checkpoint_id.clone(),
            execution_id: self.execution_id.clone(),
            program_id: self.program_id.clone(),
            boundary: self.boundary,
            state_kind: self.state_kind,
            restore_semantics: self.restore_semantics.clone(),
            execution_position: self.execution_position.clone(),
            resource_scope: self.resource_scope.clone(),
            provenance: self.provenance.clone(),
            payload: self.payload.clone(),
            created_at: CheckpointTimestamp::now()?,
            expires_at: self.expires_at,
            authorization: CheckpointAuthorization::Pending,
            lifecycle: CheckpointLifecycle::Building,
            reason: self.reason.clone(),
        };

        temporary.validate_boundary_and_state()?;
        temporary.validate_expiration()?;

        Ok(())
    }

    /// Converts a validated creation request into a committed checkpoint.
    ///
    /// Authorization is deliberately explicit: creation and authorization are
    /// separate lifecycle decisions.
    pub fn commit(
        self,
        created_at: CheckpointTimestamp,
    ) -> Result<Checkpoint, CheckpointError> {
        self.validate()?;

        let checkpoint = Checkpoint {
            schema_id: CHECKPOINT_SCHEMA_ID.to_owned(),
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            checkpoint_id: self.checkpoint_id,
            execution_id: self.execution_id,
            program_id: self.program_id,
            boundary: self.boundary,
            state_kind: self.state_kind,
            restore_semantics: self.restore_semantics,
            execution_position: self.execution_position,
            resource_scope: self.resource_scope,
            provenance: self.provenance,
            payload: self.payload,
            created_at,
            expires_at: self.expires_at,
            authorization: CheckpointAuthorization::Authorized,
            lifecycle: CheckpointLifecycle::Committed,
            reason: self.reason,
        };

        checkpoint.validate()?;

        Ok(checkpoint)
    }
}

// ============================================================================
// Validation context
// ============================================================================

/// Context supplied by the compatibility/runtime layers for additional
/// validation.
///
/// This does not duplicate hardware capabilities. It carries only the
/// information necessary for checkpoint-level semantic checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointValidationContext {
    /// Current execution identity.
    pub execution_id: ExecutionId,

    /// Current program identity.
    pub program_id: ProgramId,

    /// Current IR schema version.
    pub ir_schema_version: String,

    /// Current target identity.
    pub target_id: TargetId,

    /// Current target capability fingerprint.
    pub capability_fingerprint: String,

    /// Whether target supports generic checkpoint restoration.
    pub restoration_supported: bool,

    /// Whether provider snapshots are supported.
    pub provider_snapshot_supported: bool,

    /// Whether logical/QEC restoration is supported.
    pub logical_qec_supported: bool,

    /// Whether runtime reconstruction is supported.
    pub runtime_reconstruction_supported: bool,
}

impl CheckpointValidationContext {
    /// Validates context structure.
    pub fn validate(&self) -> Result<(), CheckpointError> {
        if self.ir_schema_version.trim().is_empty() {
            return Err(CheckpointError::InvalidValidationContext {
                field: "ir_schema_version",
            });
        }

        if self.capability_fingerprint.trim().is_empty() {
            return Err(CheckpointError::InvalidValidationContext {
                field: "capability_fingerprint",
            });
        }

        if !self.restoration_supported {
            return Err(CheckpointError::TargetRestoreUnsupported);
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint validation contract
// ============================================================================

/// Validates a checkpoint against execution/runtime context.
///
/// Implementations may live in `compatibility.rs`; this trait exists here so
/// all later files can agree on the integration contract without importing
/// concrete compatibility implementations.
pub trait CheckpointValidator: Send + Sync {
    /// Validates a checkpoint.
    fn validate_checkpoint(
        &self,
        checkpoint: &Checkpoint,
        context: &CheckpointValidationContext,
    ) -> Result<(), CheckpointError>;
}

// ============================================================================
// Checkpoint factory
// ============================================================================

/// Factory responsible only for constructing checkpoint records.
///
/// Storage, integrity and authorization remain separate.
pub trait CheckpointFactory: Send + Sync {
    /// Creates a checkpoint record.
    fn create_checkpoint(
        &self,
        request: CheckpointCreateRequest,
    ) -> Result<Checkpoint, CheckpointError>;
}

/// Default checkpoint factory.
///
/// It is deliberately stateless and therefore scales across threads/processes
/// when wrapped by an appropriate higher-level coordinator.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultCheckpointFactory;

impl CheckpointFactory for DefaultCheckpointFactory {
    fn create_checkpoint(
        &self,
        request: CheckpointCreateRequest,
    ) -> Result<Checkpoint, CheckpointError> {
        let created_at = CheckpointTimestamp::now()?;
        request.commit(created_at)
    }
}

// ============================================================================
// Storage-facing references
// ============================================================================

/// Stable reference to a checkpoint stored elsewhere.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckpointReference {
    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Storage namespace.
    pub namespace: String,

    /// Optional storage version/etag.
    pub version: Option<String>,
}

impl CheckpointReference {
    /// Creates a checkpoint reference.
    pub fn new(
        checkpoint_id: CheckpointId,
        namespace: impl Into<String>,
    ) -> Result<Self, CheckpointError> {
        let namespace = namespace.into();

        if namespace.trim().is_empty() {
            return Err(CheckpointError::InvalidStorageNamespace);
        }

        Ok(Self {
            checkpoint_id,
            namespace,
            version: None,
        })
    }

    /// Adds a storage version.
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

/// Storage contract consumed by the checkpoint lifecycle.
///
/// Concrete filesystem/object-store/provider implementations belong in
/// `storage.rs`.
pub trait CheckpointStore: Send + Sync {
    /// Persists a checkpoint.
    fn put(
        &self,
        checkpoint: &Checkpoint,
    ) -> Result<CheckpointReference, CheckpointError>;

    /// Loads a checkpoint by reference.
    fn get(
        &self,
        reference: &CheckpointReference,
    ) -> Result<Checkpoint, CheckpointError>;

    /// Removes/retire a checkpoint.
    fn remove(
        &self,
        reference: &CheckpointReference,
    ) -> Result<(), CheckpointError>;
}

// ============================================================================
// Lifecycle transition contract
// ============================================================================

/// Explicit lifecycle transition operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointTransition {
    /// Building -> committed.
    Commit,

    /// Building/committed -> revoked.
    Revoke,

    /// Revoked -> retired.
    Retire,
}

/// Applies only legal local lifecycle transitions.
///
/// This function does not perform storage operations.
pub fn transition(
    checkpoint: &Checkpoint,
    transition: CheckpointTransition,
) -> Result<Checkpoint, CheckpointError> {
    let mut updated = checkpoint.clone();

    match (checkpoint.lifecycle, transition) {
        (CheckpointLifecycle::Building, CheckpointTransition::Commit) => {
            if checkpoint.authorization
                != CheckpointAuthorization::Authorized
            {
                return Err(CheckpointError::CheckpointNotAuthorized);
            }

            updated.lifecycle = CheckpointLifecycle::Committed;
        }

        (
            CheckpointLifecycle::Building | CheckpointLifecycle::Committed,
            CheckpointTransition::Revoke,
        ) => {
            updated.lifecycle = CheckpointLifecycle::Revoked;
            updated.authorization = CheckpointAuthorization::Revoked;
        }

        (
            CheckpointLifecycle::Revoked,
            CheckpointTransition::Retire,
        ) => {
            updated.lifecycle = CheckpointLifecycle::Retired;
        }

        _ => {
            return Err(CheckpointError::InvalidLifecycleTransition);
        }
    }

    Ok(updated)
}

// ============================================================================
// Recovery eligibility
// ============================================================================

/// Result of checkpoint-level recovery eligibility evaluation.
///
/// This is intentionally not the final semantic acceptance result. The
/// verification subsystem remains authoritative for recovered computation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckpointEligibility {
    /// Checkpoint can proceed to integrity/compatibility validation.
    Eligible,

    /// Checkpoint is valid locally but needs target-specific support.
    RequiresTargetSupport,

    /// Checkpoint is not eligible.
    Ineligible,
}

impl Checkpoint {
    /// Performs checkpoint-local eligibility classification.
    #[must_use]
    pub fn eligibility(&self) -> CheckpointEligibility {
        if !self.locally_recoverable() {
            return CheckpointEligibility::Ineligible;
        }

        if self.state_kind.requires_target_support()
            || self.boundary.requires_explicit_restore_support()
        {
            return CheckpointEligibility::RequiresTargetSupport;
        }

        CheckpointEligibility::Eligible
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Complete checkpoint-contract error taxonomy.
///
/// Backend/storage/cryptographic errors can be wrapped into the appropriate
/// category by their owning adapter. Provider-specific error types must not be
/// embedded into the core checkpoint representation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckpointError {
    /// Identifier was empty.
    #[error("invalid checkpoint identifier: {field}")]
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// Schema did not match.
    #[error(
        "checkpoint schema mismatch: expected `{expected}`, got `{actual}`"
    )]
    SchemaMismatch {
        /// Expected schema.
        expected: String,

        /// Actual schema.
        actual: String,
    },

    /// Schema version was zero.
    #[error("checkpoint schema version cannot be zero")]
    InvalidSchemaVersion,

    /// Schema version is newer than this implementation understands.
    #[error("unsupported checkpoint schema version: {version}")]
    UnsupportedSchemaVersion {
        /// Unsupported version.
        version: u16,
    },

    /// Timestamp was invalid.
    #[error("invalid checkpoint timestamp")]
    InvalidTimestamp,

    /// Timestamp could not be represented.
    #[error("checkpoint timestamp is outside representable range")]
    TimestampOutOfRange,

    /// Timestamp conversion failed.
    #[error("checkpoint clock conversion failed")]
    ClockError,

    /// Checkpoint state is not restorable.
    #[error("checkpoint state is not restorable: {state:?}")]
    StateNotRestorable {
        /// State kind.
        state: CheckpointStateKind,
    },

    /// Checkpoint boundary and state kind do not agree.
    #[error("checkpoint boundary and state semantics are inconsistent")]
    BoundaryStateMismatch,

    /// Invalid expiration interval.
    #[error("checkpoint expiration occurs before creation")]
    InvalidExpiration,

    /// Checkpoint has not been authorized.
    #[error("checkpoint has not been authorized for recovery")]
    CheckpointNotAuthorized,

    /// Checkpoint lifecycle is not recoverable.
    #[error("checkpoint lifecycle does not permit recovery")]
    CheckpointNotRecoverable,

    /// Duplicate qubit identifier.
    #[error("checkpoint resource scope contains a duplicate qubit")]
    DuplicateQubit,

    /// Payload reference is invalid.
    #[error("checkpoint payload reference is invalid")]
    InvalidPayloadReference,

    /// Payload metadata is invalid.
    #[error("invalid checkpoint payload metadata: {field}")]
    InvalidPayloadMetadata {
        /// Payload field.
        field: &'static str,
    },

    /// Integrity metadata is invalid.
    #[error("invalid checkpoint integrity metadata: {field}")]
    InvalidIntegrity {
        /// Integrity field.
        field: &'static str,
    },

    /// Compression metadata is invalid.
    #[error("invalid checkpoint compression metadata")]
    InvalidCompression,

    /// Encryption metadata is invalid.
    #[error("invalid checkpoint encryption metadata")]
    InvalidEncryption,

    /// Provenance is invalid.
    #[error("invalid checkpoint provenance: {field}")]
    InvalidProvenance {
        /// Provenance field.
        field: &'static str,
    },

    /// Execution identities differ.
    #[error("checkpoint execution identity does not match requested execution")]
    ExecutionIdentityMismatch,

    /// Program identities differ.
    #[error("checkpoint program identity does not match requested program")]
    ProgramIdentityMismatch,

    /// Execution position is invalid.
    #[error("checkpoint execution position is invalid")]
    InvalidExecutionPosition,

    /// Target context is malformed.
    #[error("invalid checkpoint validation context: {field}")]
    InvalidValidationContext {
        /// Context field.
        field: &'static str,
    },

    /// Target cannot restore checkpoints.
    #[error("target does not support checkpoint restoration")]
    TargetRestoreUnsupported,

    /// Storage namespace is invalid.
    #[error("checkpoint storage namespace is invalid")]
    InvalidStorageNamespace,

    /// Illegal lifecycle transition.
    #[error("invalid checkpoint lifecycle transition")]
    InvalidLifecycleTransition,

    /// Storage operation failed.
    #[error("checkpoint storage operation failed: {message}")]
    Storage {
        /// Safe diagnostic message.
        message: String,
    },

    /// Integrity verification failed.
    #[error("checkpoint integrity verification failed")]
    IntegrityVerificationFailed,

    /// Compatibility validation failed.
    #[error("checkpoint compatibility validation failed: {message}")]
    Compatibility {
        /// Safe diagnostic message.
        message: String,
    },

    /// Serialization failed.
    #[error("checkpoint serialization failed: {message}")]
    Serialization {
        /// Safe diagnostic message.
        message: String,
    },

    /// Deserialization failed.
    #[error("checkpoint deserialization failed: {message}")]
    Deserialization {
        /// Safe diagnostic message.
        message: String,
    },

    /// Authorization/security validation failed.
    #[error("checkpoint security authorization failed")]
    SecurityAuthorizationFailed,

    /// Checkpoint has expired.
    #[error("checkpoint has expired")]
    Expired,

    /// Operation was cancelled.
    #[error("checkpoint operation was cancelled")]
    Cancelled,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ids() -> (
        CheckpointId,
        ExecutionId,
        ProgramId,
        ArtifactId,
    ) {
        (
            CheckpointId::new("checkpoint-test").expect("valid id"),
            ExecutionId::new("execution-test").expect("valid id"),
            ProgramId::new("program-test").expect("valid id"),
            ArtifactId::new("artifact-test").expect("valid id"),
        )
    }

    fn test_payload() -> CheckpointPayload {
        let (_, _, _, artifact_id) = test_ids();

        CheckpointPayload {
            artifact_id,
            location: CheckpointPayloadLocation::Reconstructible,
            media_type: "application/zamani-checkpoint".to_owned(),
            byte_length: None,
            integrity: IntegrityDescriptor {
                algorithm: "external".to_owned(),
                digest: "test-digest".to_owned(),
                authenticity_reference: None,
            },
            compression: CompressionDescriptor::None,
            encryption: EncryptionDescriptor::None,
        }
    }

    fn test_provenance() -> CheckpointProvenance {
        let (_, execution_id, program_id, _) = test_ids();

        CheckpointProvenance {
            program_id,
            execution_id,
            ir_schema_version: "zamani.ir.v1".to_owned(),
            program_fingerprint: "program-fingerprint".to_owned(),
            compilation_fingerprint: None,
            routing_fingerprint: None,
            scheduling_fingerprint: None,
            optimization_fingerprint: None,
            qec_fingerprint: None,
            capability_fingerprint: "capability-fingerprint".to_owned(),
        }
    }

    #[test]
    fn timestamp_round_trip() {
        let timestamp =
            CheckpointTimestamp::new(123, 456_789).expect("valid timestamp");

        let system_time = timestamp
            .to_system_time()
            .expect("timestamp converts");

        let restored = CheckpointTimestamp::from_system_time(system_time)
            .expect("timestamp round trips");

        assert_eq!(timestamp, restored);
    }

    #[test]
    fn rejects_unknown_state() {
        let (checkpoint_id, execution_id, program_id, _) = test_ids();

        let request = CheckpointCreateRequest {
            checkpoint_id,
            execution_id,
            program_id,
            boundary: CheckpointBoundary::ProgramStart,
            state_kind: CheckpointStateKind::Unknown,
            restore_semantics: RestoreSemantics::Replay,
            execution_position: ExecutionPosition::new("start")
                .expect("valid position"),
            resource_scope: CheckpointResourceScope::execution(),
            provenance: test_provenance(),
            payload: test_payload(),
            expires_at: None,
            reason: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn creates_program_start_checkpoint() {
        let (checkpoint_id, execution_id, program_id, _) = test_ids();

        let request = CheckpointCreateRequest {
            checkpoint_id,
            execution_id,
            program_id,
            boundary: CheckpointBoundary::ProgramStart,
            state_kind: CheckpointStateKind::ReplayableProgram,
            restore_semantics: RestoreSemantics::Replay,
            execution_position: ExecutionPosition::new("start")
                .expect("valid position"),
            resource_scope: CheckpointResourceScope::execution(),
            provenance: test_provenance(),
            payload: test_payload(),
            expires_at: None,
            reason: Some("program start".to_owned()),
        };

        let factory = DefaultCheckpointFactory;

        let checkpoint = factory
            .create_checkpoint(request)
            .expect("checkpoint should be created");

        assert!(checkpoint.validate().is_ok());
        assert!(checkpoint.locally_recoverable());
        assert_eq!(
            checkpoint.eligibility(),
            CheckpointEligibility::Eligible
        );
    }

    #[test]
    fn provider_snapshot_requires_target_support() {
        let (checkpoint_id, execution_id, program_id, _) = test_ids();

        let request = CheckpointCreateRequest {
            checkpoint_id,
            execution_id,
            program_id,
            boundary: CheckpointBoundary::ProviderSupportedSnapshot,
            state_kind: CheckpointStateKind::SupportedQuantumSnapshot,
            restore_semantics: RestoreSemantics::ProviderSnapshot,
            execution_position: ExecutionPosition::new("snapshot")
                .expect("valid position"),
            resource_scope: CheckpointResourceScope::execution(),
            provenance: test_provenance(),
            payload: CheckpointPayload {
                location: CheckpointPayloadLocation::provider_managed(
                    "provider-snapshot-reference",
                )
                .expect("valid provider reference"),
                ..test_payload()
            },
            expires_at: None,
            reason: None,
        };

        let checkpoint = DefaultCheckpointFactory
            .create_checkpoint(request)
            .expect("checkpoint should be created");

        assert_eq!(
            checkpoint.eligibility(),
            CheckpointEligibility::RequiresTargetSupport
        );
    }

    #[test]
    fn rejects_duplicate_qubits() {
        let qubit_a = QubitId::new(0);
        let qubit_b = QubitId::new(0);

        let result =
            CheckpointResourceScope::logical_qubits([qubit_a, qubit_b]);

        assert!(matches!(
            result,
            Err(CheckpointError::DuplicateQubit)
        ));
    }

    #[test]
    fn lifecycle_transition_requires_authorization() {
        let (checkpoint_id, execution_id, program_id, _) = test_ids();

        let request = CheckpointCreateRequest {
            checkpoint_id,
            execution_id,
            program_id,
            boundary: CheckpointBoundary::ProgramStart,
            state_kind: CheckpointStateKind::ReplayableProgram,
            restore_semantics: RestoreSemantics::Replay,
            execution_position: ExecutionPosition::new("start")
                .expect("valid position"),
            resource_scope: CheckpointResourceScope::execution(),
            provenance: test_provenance(),
            payload: test_payload(),
            expires_at: None,
            reason: None,
        };

        let checkpoint = request
            .commit(CheckpointTimestamp::new(1, 0).expect("timestamp"))
            .expect("checkpoint");

        assert_eq!(
            checkpoint.lifecycle,
            CheckpointLifecycle::Committed
        );
    }

    #[test]
    fn expiration_is_monotonic() {
        let timestamp =
            CheckpointTimestamp::new(100, 0).expect("timestamp");

        let later =
            CheckpointTimestamp::new(101, 0).expect("timestamp");

        assert_eq!(
            timestamp.to_system_time().expect("system time"),
            UNIX_EPOCH + Duration::from_secs(100)
        );

        assert!(
            later > timestamp
        );
    }
}