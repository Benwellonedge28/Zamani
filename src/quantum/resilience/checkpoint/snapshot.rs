//! Zamani Quantum Resilience — Snapshot Contract
//!
//! Path:
//!     src/quantum/resilience/checkpoint/snapshot.rs
//!
//! Purpose:
//!     Canonical, backend-independent representation of execution snapshots
//!     used by the quantum-resilience checkpoint subsystem.
//!
//! Architectural ownership:
//!
//!     snapshot.rs
//!         Owns:
//!             - snapshot identity;
//!             - snapshot semantic kind;
//!             - snapshot boundary;
//!             - snapshot scope;
//!             - quantum-state restoration semantics;
//!             - runtime/provider snapshot metadata;
//!             - payload descriptors;
//!             - capture metadata;
//!             - restoration requirements;
//!             - structural validation;
//!             - snapshot-source and snapshot-reader contracts.
//!
//!     checkpoint.rs
//!         Owns:
//!             - checkpoint lifecycle;
//!             - checkpoint identity;
//!             - checkpoint-level metadata;
//!             - checkpoint composition;
//!             - checkpoint validation.
//!
//!     manifest.rs
//!         Owns:
//!             - artifact enumeration;
//!             - complete checkpoint manifests.
//!
//!     storage.rs
//!         Owns:
//!             - physical persistence;
//!             - storage-provider implementations;
//!             - reads/writes/deletes/listing.
//!
//!     integrity.rs
//!         Owns:
//!             - digest computation;
//!             - signature verification;
//!             - authenticity policy.
//!
//!     compatibility.rs
//!         Owns:
//!             - target compatibility;
//!             - schema compatibility;
//!             - restoration negotiation.
//!
//!     recovery/*
//!         Owns:
//!             - restore orchestration;
//!             - rollback;
//!             - resume;
//!             - migration.
//!
//! Important quantum-state rule:
//!
//!     A snapshot is NOT automatically a copy of an arbitrary quantum state.
//!
//!     Unknown quantum states cannot be made reconstructible merely by storing
//!     bytes. A snapshot must explicitly declare what semantics it represents
//!     and whether the target runtime/provider supports restoration.
//!
//! Supported snapshot semantics include:
//!
//!     - classical execution state;
//!     - replayable program state;
//!     - compiled execution state;
//!     - measurement-boundary state;
//!     - logical/QEC state;
//!     - reconstructible runtime state;
//!     - explicitly provider-supported quantum state;
//!     - opaque provider state that is not portable.
//!
//! Scalability:
//!
//!     This module imposes no architectural maximum on:
//!
//!         qubits
//!         logical qubits
//!         physical qubits
//!         operations
//!         payload bytes
//!         snapshots
//!         artifacts
//!         devices
//!         execution stages
//!
//!     Actual limits are supplied by explicit resource, security, runtime,
//!     storage, hardware, provider and policy layers.
//!
//!     "Infinity" therefore means no artificial finite machine-size ceiling,
//!     not an impossible claim that finite computers can contain infinite data.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code
//!
//! Integration:
//!
//!     This file intentionally contains no filesystem, network, provider SDK,
//!     hardware driver, allocator, serializer implementation or cryptographic
//!     implementation.
//!
//!     Concrete implementations are injected through the contracts below.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::checkpoint::{
    CheckpointBoundary,
    CheckpointId,
    CheckpointStateKind,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable snapshot schema namespace.
pub const SNAPSHOT_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint.snapshot";

/// Semantic version of this snapshot contract.
pub const SNAPSHOT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Snapshot identifiers
// =============================================================================

/// Opaque identity for a snapshot.
///
/// The value must not encode:
///
/// - hardware size;
/// - provider;
/// - physical qubit numbering;
/// - storage location;
/// - retry count.
///
/// Those concerns belong to their respective layers.
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
pub struct SnapshotId(String);

impl SnapshotId {
    /// Creates a validated snapshot identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, SnapshotError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SnapshotError::InvalidIdentifier {
                field: "snapshot_id",
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

impl fmt::Display for SnapshotId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Snapshot semantic kind
// =============================================================================

/// Describes the semantic content represented by a snapshot.
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
pub enum SnapshotKind {
    /// Classical execution state only.
    ClassicalExecution,

    /// Program plus sufficient information to replay/reconstruct execution.
    ReplayableProgram,

    /// Compiled execution representation.
    CompiledExecution,

    /// State at a measurement-defined semantic boundary.
    MeasurementBoundary,

    /// Logical state represented at an explicitly supported QEC boundary.
    LogicalQec,

    /// Runtime state reconstructed from explicit deterministic information.
    ReconstructibleRuntime,

    /// Quantum state snapshot explicitly supported by the runtime/provider.
    ProviderSupportedQuantum,

    /// Opaque provider/runtime state.
    ///
    /// This is intentionally distinct from portable quantum state.
    OpaqueProviderState,
}

impl SnapshotKind {
    /// Returns the corresponding checkpoint-level state classification.
    #[must_use]
    pub const fn checkpoint_state_kind(self) -> CheckpointStateKind {
        match self {
            Self::ClassicalExecution => CheckpointStateKind::Classical,
            Self::ReplayableProgram => CheckpointStateKind::ReplayableProgram,
            Self::CompiledExecution => CheckpointStateKind::CompiledExecution,
            Self::MeasurementBoundary => {
                CheckpointStateKind::MeasurementBoundary
            }
            Self::LogicalQec => CheckpointStateKind::LogicalQec,
            Self::ReconstructibleRuntime => {
                CheckpointStateKind::ReconstructibleRuntime
            }
            Self::ProviderSupportedQuantum => {
                CheckpointStateKind::SupportedQuantumSnapshot
            }
            Self::OpaqueProviderState => {
                CheckpointStateKind::SupportedQuantumSnapshot
            }
        }
    }

    /// Returns whether the snapshot is explicitly quantum-state-bearing.
    #[must_use]
    pub const fn contains_quantum_state(self) -> bool {
        matches!(
            self,
            Self::ProviderSupportedQuantum | Self::OpaqueProviderState
        )
    }

    /// Returns whether portability requires target/provider negotiation.
    #[must_use]
    pub const fn requires_provider_support(self) -> bool {
        matches!(
            self,
            Self::ProviderSupportedQuantum | Self::OpaqueProviderState
        )
    }
}

// =============================================================================
// Restoration semantics
// =============================================================================

/// Describes what can actually happen during restoration.
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
pub enum RestorationSemantics {
    /// State is restored directly from persisted classical data.
    DirectClassical,

    /// State is reconstructed by replaying the program.
    DeterministicReplay,

    /// State is reconstructed from a compiled representation.
    CompiledReplay,

    /// State is reconstructed at a measurement boundary.
    MeasurementReconstruction,

    /// Logical/QEC state is restored through an explicitly supported
    /// logical-state restoration protocol.
    LogicalQecRestore,

    /// Runtime reconstructs state through a documented runtime protocol.
    RuntimeReconstruction,

    /// Provider restores its own quantum state.
    ProviderNativeRestore,

    /// Provider restores an opaque state object.
    ProviderOpaqueRestore,
}

impl RestorationSemantics {
    /// Returns whether the restoration semantics are portable in principle.
    #[must_use]
    pub const fn is_portable(self) -> bool {
        matches!(
            self,
            Self::DirectClassical
                | Self::DeterministicReplay
                | Self::CompiledReplay
                | Self::MeasurementReconstruction
                | Self::LogicalQecRestore
                | Self::RuntimeReconstruction
        )
    }

    /// Returns whether explicit target support is required.
    #[must_use]
    pub const fn requires_target_support(self) -> bool {
        !self.is_portable()
    }
}

// =============================================================================
// Snapshot scope
// =============================================================================

/// Defines which quantum resources are represented by the snapshot.
///
/// The canonical IR identities are used. This module does not introduce a
/// resilience-specific qubit identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotScope {
    /// Entire execution state.
    Execution,

    /// Explicit logical-qubit state.
    LogicalQubits(Arc<[QubitId]>),

    /// Explicit physical-qubit state.
    PhysicalQubits(Arc<[PhysicalQubitId]>),

    /// A logical execution region.
    LogicalRegion {
        /// Stable region identity.
        region_id: String,
    },

    /// A runtime-defined resource set.
    RuntimeDefined {
        /// Stable runtime resource-set identity.
        resource_set_id: String,
    },
}

impl SnapshotScope {
    /// Creates an execution-wide scope.
    #[must_use]
    pub const fn execution() -> Self {
        Self::Execution
    }

    /// Creates a logical-qubit scope.
    pub fn logical_qubits(
        qubits: impl IntoIterator<Item = QubitId>,
    ) -> Result<Self, SnapshotError> {
        let qubits: Vec<QubitId> = qubits.into_iter().collect();

        validate_unique_logical_qubits(&qubits)?;

        Ok(Self::LogicalQubits(Arc::from(qubits)))
    }

    /// Creates a physical-qubit scope.
    pub fn physical_qubits(
        qubits: impl IntoIterator<Item = PhysicalQubitId>,
    ) -> Result<Self, SnapshotError> {
        let qubits: Vec<PhysicalQubitId> = qubits.into_iter().collect();

        validate_unique_physical_qubits(&qubits)?;

        Ok(Self::PhysicalQubits(Arc::from(qubits)))
    }

    /// Returns explicitly enumerated logical or physical qubits.
    #[must_use]
    pub fn qubit_counts(&self) -> Option<usize> {
        match self {
            Self::LogicalQubits(qubits) => Some(qubits.len()),
            Self::PhysicalQubits(qubits) => Some(qubits.len()),
            Self::Execution
            | Self::LogicalRegion { .. }
            | Self::RuntimeDefined { .. } => None,
        }
    }

    /// Returns true when the scope refers to logical qubits.
    #[must_use]
    pub const fn is_logical(&self) -> bool {
        matches!(self, Self::LogicalQubits(_))
    }

    /// Returns true when the scope refers to physical qubits.
    #[must_use]
    pub const fn is_physical(&self) -> bool {
        matches!(self, Self::PhysicalQubits(_))
    }
}

fn validate_unique_logical_qubits(
    qubits: &[QubitId],
) -> Result<(), SnapshotError> {
    for index in 1..qubits.len() {
        if qubits[..index].contains(&qubits[index]) {
            return Err(SnapshotError::DuplicateLogicalQubit);
        }
    }

    Ok(())
}

fn validate_unique_physical_qubits(
    qubits: &[PhysicalQubitId],
) -> Result<(), SnapshotError> {
    for index in 1..qubits.len() {
        if qubits[..index].contains(&qubits[index]) {
            return Err(SnapshotError::DuplicatePhysicalQubit);
        }
    }

    Ok(())
}

// =============================================================================
// Payload location
// =============================================================================

/// Describes where snapshot bytes or reconstructible information reside.
///
/// No actual payload is stored here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotPayloadLocation {
    /// Stored externally through the checkpoint storage layer.
    External {
        /// Opaque storage object reference.
        object_reference: String,
    },

    /// Stored by the runtime.
    Runtime {
        /// Opaque runtime object reference.
        object_reference: String,
    },

    /// No payload bytes are required because the snapshot is reconstructed
    /// from canonical program/execution metadata.
    Reconstructible,
}

impl SnapshotPayloadLocation {
    /// Creates an external payload reference.
    pub fn external(
        object_reference: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let object_reference = object_reference.into();

        validate_non_empty(
            &object_reference,
            "object_reference",
        )?;

        Ok(Self::External { object_reference })
    }

    /// Creates a runtime payload reference.
    pub fn runtime(
        object_reference: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let object_reference = object_reference.into();

        validate_non_empty(
            &object_reference,
            "object_reference",
        )?;

        Ok(Self::Runtime { object_reference })
    }

    /// Creates a reconstructible payload descriptor.
    #[must_use]
    pub const fn reconstructible() -> Self {
        Self::Reconstructible
    }

    /// Returns true if no payload bytes are required.
    #[must_use]
    pub const fn is_reconstructible(&self) -> bool {
        matches!(self, Self::Reconstructible)
    }
}

// =============================================================================
// Payload descriptor
// =============================================================================

/// Describes one snapshot artifact.
///
/// The descriptor is intentionally independent of any concrete storage
/// provider, hash algorithm, compression format or encryption implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotPayload {
    /// Stable artifact identity.
    pub artifact_id: String,

    /// Semantic media/content type.
    pub content_type: String,

    /// Location of the payload.
    pub location: SnapshotPayloadLocation,

    /// Exact payload byte length when materialized.
    ///
    /// `None` means the payload is reconstructible or its size is intentionally
    /// determined by the storage provider.
    pub byte_length: Option<u64>,

    /// Optional integrity descriptor.
    ///
    /// The integrity subsystem interprets the algorithm and value.
    pub integrity: Option<IntegrityDescriptor>,

    /// Optional compression descriptor.
    pub compression: Option<CompressionDescriptor>,

    /// Optional encryption descriptor.
    pub encryption: Option<EncryptionDescriptor>,

    /// Provider/runtime-defined metadata.
    pub metadata: BTreeMap<String, String>,
}

impl SnapshotPayload {
    /// Creates a new payload descriptor.
    pub fn new(
        artifact_id: impl Into<String>,
        content_type: impl Into<String>,
        location: SnapshotPayloadLocation,
    ) -> Result<Self, SnapshotError> {
        let artifact_id = artifact_id.into();
        let content_type = content_type.into();

        validate_non_empty(&artifact_id, "artifact_id")?;
        validate_non_empty(&content_type, "content_type")?;

        Ok(Self {
            artifact_id,
            content_type,
            location,
            byte_length: None,
            integrity: None,
            compression: None,
            encryption: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Sets the materialized payload length.
    #[must_use]
    pub const fn with_byte_length(mut self, byte_length: u64) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Adds an integrity descriptor.
    #[must_use]
    pub fn with_integrity(
        mut self,
        integrity: IntegrityDescriptor,
    ) -> Self {
        self.integrity = Some(integrity);
        self
    }

    /// Adds compression metadata.
    #[must_use]
    pub fn with_compression(
        mut self,
        compression: CompressionDescriptor,
    ) -> Self {
        self.compression = Some(compression);
        self
    }

    /// Adds encryption metadata.
    #[must_use]
    pub fn with_encryption(
        mut self,
        encryption: EncryptionDescriptor,
    ) -> Self {
        self.encryption = Some(encryption);
        self
    }

    /// Adds provider/runtime metadata.
    ///
    /// A `BTreeMap` is deliberately used to preserve deterministic ordering.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), SnapshotError> {
        let key = key.into();

        validate_non_empty(&key, "metadata_key")?;

        self.metadata.insert(key, value.into());

        Ok(())
    }

    /// Validates the payload descriptor.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(&self.artifact_id, "artifact_id")?;
        validate_non_empty(&self.content_type, "content_type")?;

        match &self.location {
            SnapshotPayloadLocation::External {
                object_reference,
            }
            | SnapshotPayloadLocation::Runtime {
                object_reference,
            } => {
                validate_non_empty(
                    object_reference,
                    "object_reference",
                )?;
            }
            SnapshotPayloadLocation::Reconstructible => {}
        }

        if let Some(integrity) = &self.integrity {
            integrity.validate()?;
        }

        if let Some(compression) = &self.compression {
            compression.validate()?;
        }

        if let Some(encryption) = &self.encryption {
            encryption.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Integrity metadata
// =============================================================================

/// Algorithm-independent integrity descriptor.
///
/// The actual cryptographic implementation belongs to `checkpoint::integrity`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityDescriptor {
    /// Algorithm identifier, for example a registry-defined digest name.
    pub algorithm: String,

    /// Encoded digest value.
    pub digest: String,

    /// Optional signer/key identity.
    pub signer: Option<String>,
}

impl IntegrityDescriptor {
    /// Creates an integrity descriptor.
    pub fn new(
        algorithm: impl Into<String>,
        digest: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let algorithm = algorithm.into();
        let digest = digest.into();

        validate_non_empty(&algorithm, "integrity_algorithm")?;
        validate_non_empty(&digest, "integrity_digest")?;

        Ok(Self {
            algorithm,
            digest,
            signer: None,
        })
    }

    /// Associates a signer/key identity.
    pub fn with_signer(
        mut self,
        signer: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let signer = signer.into();

        validate_non_empty(&signer, "integrity_signer")?;

        self.signer = Some(signer);

        Ok(self)
    }

    /// Validates the descriptor.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(
            &self.algorithm,
            "integrity_algorithm",
        )?;

        validate_non_empty(
            &self.digest,
            "integrity_digest",
        )?;

        if let Some(signer) = &self.signer {
            validate_non_empty(signer, "integrity_signer")?;
        }

        Ok(())
    }
}

// =============================================================================
// Compression
// =============================================================================

/// Compression metadata.
///
/// The implementation is intentionally external to the snapshot contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompressionDescriptor {
    /// Compression algorithm identifier.
    pub algorithm: String,

    /// Optional algorithm version.
    pub version: Option<String>,

    /// Whether the compression is lossless.
    pub lossless: bool,
}

impl CompressionDescriptor {
    /// Creates compression metadata.
    pub fn new(
        algorithm: impl Into<String>,
        lossless: bool,
    ) -> Result<Self, SnapshotError> {
        let algorithm = algorithm.into();

        validate_non_empty(
            &algorithm,
            "compression_algorithm",
        )?;

        Ok(Self {
            algorithm,
            version: None,
            lossless,
        })
    }

    /// Sets an algorithm version.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let version = version.into();

        validate_non_empty(
            &version,
            "compression_version",
        )?;

        self.version = Some(version);

        Ok(self)
    }

    /// Validates compression metadata.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(
            &self.algorithm,
            "compression_algorithm",
        )?;

        if let Some(version) = &self.version {
            validate_non_empty(
                version,
                "compression_version",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Encryption
// =============================================================================

/// Encryption metadata.
///
/// Keys and plaintext are never stored here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionDescriptor {
    /// Encryption scheme identifier.
    pub algorithm: String,

    /// Key-management reference.
    pub key_reference: String,

    /// Optional encryption version.
    pub version: Option<String>,
}

impl EncryptionDescriptor {
    /// Creates encryption metadata.
    pub fn new(
        algorithm: impl Into<String>,
        key_reference: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let algorithm = algorithm.into();
        let key_reference = key_reference.into();

        validate_non_empty(
            &algorithm,
            "encryption_algorithm",
        )?;

        validate_non_empty(
            &key_reference,
            "encryption_key_reference",
        )?;

        Ok(Self {
            algorithm,
            key_reference,
            version: None,
        })
    }

    /// Sets an encryption version.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let version = version.into();

        validate_non_empty(
            &version,
            "encryption_version",
        )?;

        self.version = Some(version);

        Ok(self)
    }

    /// Validates encryption metadata.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(
            &self.algorithm,
            "encryption_algorithm",
        )?;

        validate_non_empty(
            &self.key_reference,
            "encryption_key_reference",
        )?;

        if let Some(version) = &self.version {
            validate_non_empty(
                version,
                "encryption_version",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Runtime/provider metadata
// =============================================================================

/// Metadata identifying the runtime/provider snapshot contract.
///
/// This does not itself identify credentials or network endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRuntimeMetadata {
    /// Provider/runtime family identifier.
    pub implementation: String,

    /// Provider/runtime implementation version.
    pub version: String,

    /// Snapshot format identifier understood by the implementation.
    pub format: String,

    /// Optional opaque implementation state version.
    pub state_version: Option<String>,

    /// Additional deterministic metadata.
    pub metadata: BTreeMap<String, String>,
}

impl SnapshotRuntimeMetadata {
    /// Creates runtime/provider metadata.
    pub fn new(
        implementation: impl Into<String>,
        version: impl Into<String>,
        format: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let implementation = implementation.into();
        let version = version.into();
        let format = format.into();

        validate_non_empty(
            &implementation,
            "runtime_implementation",
        )?;

        validate_non_empty(
            &version,
            "runtime_version",
        )?;

        validate_non_empty(&format, "snapshot_format")?;

        Ok(Self {
            implementation,
            version,
            format,
            state_version: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Sets a state-format version.
    pub fn with_state_version(
        mut self,
        state_version: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let state_version = state_version.into();

        validate_non_empty(
            &state_version,
            "state_version",
        )?;

        self.state_version = Some(state_version);

        Ok(self)
    }

    /// Inserts deterministic implementation metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), SnapshotError> {
        let key = key.into();

        validate_non_empty(&key, "metadata_key")?;

        self.metadata.insert(key, value.into());

        Ok(())
    }

    /// Validates runtime/provider metadata.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(
            &self.implementation,
            "runtime_implementation",
        )?;

        validate_non_empty(
            &self.version,
            "runtime_version",
        )?;

        validate_non_empty(
            &self.format,
            "snapshot_format",
        )?;

        if let Some(state_version) = &self.state_version {
            validate_non_empty(
                state_version,
                "state_version",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Execution position
// =============================================================================

/// Identifies the semantic execution position represented by a snapshot.
///
/// This intentionally does not assume that every quantum program is a
/// `Vec<Gate>` or that every execution has a linear instruction index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotPosition {
    /// Program-level position.
    ProgramBoundary {
        /// Stable semantic boundary identifier.
        boundary_id: String,
    },

    /// Operation-level position.
    OperationBoundary {
        /// Canonical/opaque operation identity.
        operation_id: String,
    },

    /// Region-level position.
    RegionBoundary {
        /// Stable semantic region identity.
        region_id: String,
    },

    /// Measurement boundary.
    MeasurementBoundary {
        /// Stable measurement-boundary identity.
        measurement_id: String,
    },

    /// Runtime-defined execution position.
    RuntimeDefined {
        /// Opaque runtime position.
        position: String,
    },
}

impl SnapshotPosition {
    /// Validates the position.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        match self {
            Self::ProgramBoundary { boundary_id }
            | Self::OperationBoundary {
                operation_id: boundary_id,
            }
            | Self::RegionBoundary {
                region_id: boundary_id,
            }
            | Self::MeasurementBoundary {
                measurement_id: boundary_id,
            }
            | Self::RuntimeDefined {
                position: boundary_id,
            } => validate_non_empty(
                boundary_id,
                "snapshot_position",
            ),
        }
    }
}

// =============================================================================
// Snapshot provenance
// =============================================================================

/// Provenance describing how a snapshot was produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotProvenance {
    /// Parent checkpoint when this snapshot belongs to a checkpoint chain.
    pub parent_checkpoint: Option<CheckpointId>,

    /// Canonical program identity/hash reference.
    pub program_reference: Option<String>,

    /// Canonical IR identity/hash reference.
    pub ir_reference: Option<String>,

    /// Compilation identity/hash reference.
    pub compilation_reference: Option<String>,

    /// Target identity/reference.
    pub target_reference: Option<String>,

    /// Optional routing/mapping identity.
    pub mapping_reference: Option<String>,

    /// Optional schedule identity.
    pub schedule_reference: Option<String>,

    /// Optional QEC configuration identity.
    pub qec_reference: Option<String>,

    /// Additional deterministic provenance fields.
    pub metadata: BTreeMap<String, String>,
}

impl SnapshotProvenance {
    /// Creates empty provenance.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            parent_checkpoint: None,
            program_reference: None,
            ir_reference: None,
            compilation_reference: None,
            target_reference: None,
            mapping_reference: None,
            schedule_reference: None,
            qec_reference: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Validates provenance.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_optional_string(
            &self.program_reference,
            "program_reference",
        )?;

        validate_optional_string(
            &self.ir_reference,
            "ir_reference",
        )?;

        validate_optional_string(
            &self.compilation_reference,
            "compilation_reference",
        )?;

        validate_optional_string(
            &self.target_reference,
            "target_reference",
        )?;

        validate_optional_string(
            &self.mapping_reference,
            "mapping_reference",
        )?;

        validate_optional_string(
            &self.schedule_reference,
            "schedule_reference",
        )?;

        validate_optional_string(
            &self.qec_reference,
            "qec_reference",
        )?;

        Ok(())
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Immutable semantic description of a quantum execution snapshot.
///
/// This is the central object consumed by:
///
/// - `checkpoint.rs`;
/// - `manifest.rs`;
/// - `storage.rs`;
/// - `integrity.rs`;
/// - `compatibility.rs`;
/// - `recovery::checkpoint`;
/// - `recovery::resume`;
/// - `recovery::rollback`;
/// - `verification::*`.
///
/// It does not contain arbitrary snapshot bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Stable snapshot identity.
    pub id: SnapshotId,

    /// Snapshot schema namespace.
    pub schema_id: String,

    /// Snapshot schema version.
    pub schema_version: u16,

    /// Semantic snapshot kind.
    pub kind: SnapshotKind,

    /// Corresponding checkpoint boundary.
    pub boundary: CheckpointBoundary,

    /// Explicit restoration semantics.
    pub restoration: RestorationSemantics,

    /// Resources represented by the snapshot.
    pub scope: SnapshotScope,

    /// Semantic execution position.
    pub position: SnapshotPosition,

    /// Payload artifacts.
    pub payloads: Arc<[SnapshotPayload]>,

    /// Provider/runtime metadata when required.
    pub runtime: Option<SnapshotRuntimeMetadata>,

    /// Snapshot provenance.
    pub provenance: SnapshotProvenance,

    /// Snapshot-level metadata.
    pub metadata: BTreeMap<String, String>,
}

impl Snapshot {
    /// Creates a new snapshot.
    pub fn new(
        id: SnapshotId,
        kind: SnapshotKind,
        boundary: CheckpointBoundary,
        restoration: RestorationSemantics,
        scope: SnapshotScope,
        position: SnapshotPosition,
    ) -> Result<Self, SnapshotError> {
        position.validate()?;

        let snapshot = Self {
            id,
            schema_id: SNAPSHOT_SCHEMA_ID.to_owned(),
            schema_version: SNAPSHOT_SCHEMA_VERSION,
            kind,
            boundary,
            restoration,
            scope,
            position,
            payloads: Arc::from(Vec::<SnapshotPayload>::new()),
            runtime: None,
            provenance: SnapshotProvenance::empty(),
            metadata: BTreeMap::new(),
        };

        snapshot.validate()?;

        Ok(snapshot)
    }

    /// Adds one payload without mutating an existing snapshot.
    ///
    /// Returning a new snapshot keeps the semantic snapshot immutable after
    /// publication.
    pub fn with_payload(
        mut self,
        payload: SnapshotPayload,
    ) -> Result<Self, SnapshotError> {
        payload.validate()?;

        let mut payloads =
            self.payloads.iter().cloned().collect::<Vec<_>>();

        payloads.push(payload);

        self.payloads = Arc::from(payloads);

        self.validate()?;

        Ok(self)
    }

    /// Adds runtime/provider metadata.
    pub fn with_runtime(
        mut self,
        runtime: SnapshotRuntimeMetadata,
    ) -> Result<Self, SnapshotError> {
        runtime.validate()?;

        self.runtime = Some(runtime);

        self.validate()?;

        Ok(self)
    }

    /// Adds provenance.
    pub fn with_provenance(
        mut self,
        provenance: SnapshotProvenance,
    ) -> Result<Self, SnapshotError> {
        provenance.validate()?;

        self.provenance = provenance;

        self.validate()?;

        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let key = key.into();

        validate_non_empty(&key, "metadata_key")?;

        self.metadata.insert(key, value.into());

        self.validate()?;

        Ok(self)
    }

    /// Returns true if the snapshot is portable without provider-native
    /// snapshot restoration.
    #[must_use]
    pub const fn is_portable(&self) -> bool {
        self.restoration.is_portable()
    }

    /// Returns true if explicit provider/runtime support is required.
    #[must_use]
    pub const fn requires_provider_support(&self) -> bool {
        self.restoration.requires_target_support()
    }

    /// Returns true if this snapshot explicitly claims to contain quantum
    /// state.
    #[must_use]
    pub const fn contains_quantum_state(&self) -> bool {
        self.kind.contains_quantum_state()
    }

    /// Returns the number of payload descriptors.
    #[must_use]
    pub fn payload_count(&self) -> usize {
        self.payloads.len()
    }

    /// Validates the complete snapshot contract.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(self.id.as_str(), "snapshot_id")?;

        if self.schema_id != SNAPSHOT_SCHEMA_ID {
            return Err(SnapshotError::UnsupportedSchema {
                schema_id: self.schema_id.clone(),
            });
        }

        if self.schema_version == 0 {
            return Err(SnapshotError::InvalidSchemaVersion);
        }

        self.position.validate()?;
        self.provenance.validate()?;

        for payload in self.payloads.iter() {
            payload.validate()?;
        }

        validate_payload_uniqueness(&self.payloads)?;

        self.validate_semantics()?;

        Ok(())
    }

    fn validate_semantics(&self) -> Result<(), SnapshotError> {
        let expected_state = self.kind.checkpoint_state_kind();

        if expected_state == CheckpointStateKind::Invalid
            || expected_state == CheckpointStateKind::Unknown
        {
            return Err(SnapshotError::InvalidStateSemantics);
        }

        match self.kind {
            SnapshotKind::ProviderSupportedQuantum
            | SnapshotKind::OpaqueProviderState => {
                if self.runtime.is_none() {
                    return Err(
                        SnapshotError::RuntimeMetadataRequired,
                    );
                }

                if !self.restoration.requires_target_support() {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::LogicalQec => {
                if !self.scope.is_logical() {
                    return Err(
                        SnapshotError::LogicalQecRequiresLogicalScope,
                    );
                }

                if self.restoration
                    != RestorationSemantics::LogicalQecRestore
                {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::MeasurementBoundary => {
                if !matches!(
                    self.boundary,
                    CheckpointBoundary::Measurement
                ) {
                    return Err(
                        SnapshotError::MeasurementBoundaryMismatch,
                    );
                }

                if !matches!(
                    self.restoration,
                    RestorationSemantics::MeasurementReconstruction
                        | RestorationSemantics::DeterministicReplay
                ) {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::ClassicalExecution => {
                if !matches!(
                    self.restoration,
                    RestorationSemantics::DirectClassical
                        | RestorationSemantics::DeterministicReplay
                ) {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::ReplayableProgram => {
                if self.restoration
                    != RestorationSemantics::DeterministicReplay
                {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::CompiledExecution => {
                if self.restoration
                    != RestorationSemantics::CompiledReplay
                {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
            SnapshotKind::ReconstructibleRuntime => {
                if self.restoration
                    != RestorationSemantics::RuntimeReconstruction
                {
                    return Err(
                        SnapshotError::InvalidRestorationSemantics,
                    );
                }
            }
        }

        if self.boundary.is_provider_snapshot()
            && self.runtime.is_none()
        {
            return Err(SnapshotError::RuntimeMetadataRequired);
        }

        if self.restoration.is_portable()
            && self.kind == SnapshotKind::OpaqueProviderState
        {
            return Err(
                SnapshotError::OpaqueStateCannotBePortable,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Payload uniqueness
// =============================================================================

fn validate_payload_uniqueness(
    payloads: &[SnapshotPayload],
) -> Result<(), SnapshotError> {
    for index in 1..payloads.len() {
        if payloads[..index]
            .iter()
            .any(|previous| {
                previous.artifact_id == payloads[index].artifact_id
            })
        {
            return Err(SnapshotError::DuplicateArtifact);
        }
    }

    Ok(())
}

// =============================================================================
// Snapshot capture request
// =============================================================================

/// Describes a request to create a snapshot.
///
/// This is intentionally separate from `Snapshot`, because a requested
/// snapshot has not necessarily been successfully captured yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRequest {
    /// Requested semantic kind.
    pub kind: SnapshotKind,

    /// Requested boundary.
    pub boundary: CheckpointBoundary,

    /// Requested restoration semantics.
    pub restoration: RestorationSemantics,

    /// Resources to capture.
    pub scope: SnapshotScope,

    /// Semantic execution position.
    pub position: SnapshotPosition,

    /// Optional parent checkpoint.
    pub parent_checkpoint: Option<CheckpointId>,

    /// Whether provider-native state may be used.
    pub allow_provider_state: bool,

    /// Whether opaque provider state is acceptable.
    pub allow_opaque_provider_state: bool,

    /// Additional capture requirements.
    pub requirements: BTreeMap<String, String>,
}

impl SnapshotRequest {
    /// Creates a request.
    pub fn new(
        kind: SnapshotKind,
        boundary: CheckpointBoundary,
        restoration: RestorationSemantics,
        scope: SnapshotScope,
        position: SnapshotPosition,
    ) -> Result<Self, SnapshotError> {
        let request = Self {
            kind,
            boundary,
            restoration,
            scope,
            position,
            parent_checkpoint: None,
            allow_provider_state: false,
            allow_opaque_provider_state: false,
            requirements: BTreeMap::new(),
        };

        request.validate()?;

        Ok(request)
    }

    /// Allows provider-native state.
    #[must_use]
    pub const fn allowing_provider_state(mut self) -> Self {
        self.allow_provider_state = true;
        self
    }

    /// Allows opaque provider state.
    #[must_use]
    pub const fn allowing_opaque_provider_state(mut self) -> Self {
        self.allow_provider_state = true;
        self.allow_opaque_provider_state = true;
        self
    }

    /// Sets a parent checkpoint.
    #[must_use]
    pub fn with_parent_checkpoint(
        mut self,
        parent_checkpoint: CheckpointId,
    ) -> Self {
        self.parent_checkpoint = Some(parent_checkpoint);
        self
    }

    /// Adds a capture requirement.
    pub fn with_requirement(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let key = key.into();

        validate_non_empty(&key, "requirement_key")?;

        self.requirements.insert(key, value.into());

        Ok(self)
    }

    /// Validates the request.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        self.position.validate()?;

        if self.kind.contains_quantum_state()
            && !self.allow_provider_state
        {
            return Err(
                SnapshotError::ProviderStateNotAllowed,
            );
        }

        if self.kind == SnapshotKind::OpaqueProviderState
            && !self.allow_opaque_provider_state
        {
            return Err(
                SnapshotError::OpaqueProviderStateNotAllowed,
            );
        }

        if self.kind == SnapshotKind::LogicalQec
            && !self.scope.is_logical()
        {
            return Err(
                SnapshotError::LogicalQecRequiresLogicalScope,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Snapshot capture result
// =============================================================================

/// Result returned by a snapshot capture implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotCapture {
    /// Captured snapshot.
    pub snapshot: Snapshot,

    /// Whether the snapshot was captured atomically according to the runtime
    /// contract.
    pub atomic: bool,

    /// Optional capture sequence/reference.
    pub capture_reference: Option<String>,

    /// Capture metadata.
    pub metadata: BTreeMap<String, String>,
}

impl SnapshotCapture {
    /// Creates a successful capture result.
    pub fn new(
        snapshot: Snapshot,
        atomic: bool,
    ) -> Result<Self, SnapshotError> {
        snapshot.validate()?;

        Ok(Self {
            snapshot,
            atomic,
            capture_reference: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Adds an implementation-defined capture reference.
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let reference = reference.into();

        validate_non_empty(
            &reference,
            "capture_reference",
        )?;

        self.capture_reference = Some(reference);

        Ok(self)
    }
}

// =============================================================================
// Snapshot source contract
// =============================================================================

/// Produces snapshots from an execution/runtime.
///
/// Implementations belong to runtime/provider adapters.
///
/// This trait deliberately does not prescribe:
///
/// - threads;
/// - async runtime;
/// - network stack;
/// - storage;
/// - provider SDK;
/// - memory layout.
pub trait SnapshotSource {
    /// Error type produced by the implementation.
    type Error;

    /// Captures a snapshot according to the request.
    fn capture(
        &self,
        request: &SnapshotRequest,
    ) -> Result<SnapshotCapture, Self::Error>;
}

// =============================================================================
// Snapshot restoration source contract
// =============================================================================

/// Reads/restores provider/runtime snapshot state.
///
/// This trait does not perform recovery orchestration. It only exposes the
/// implementation-specific restoration primitive required by the recovery
/// subsystem.
pub trait SnapshotRestorer {
    /// Error type produced by the implementation.
    type Error;

    /// Returns whether this implementation supports the supplied snapshot.
    fn supports(
        &self,
        snapshot: &Snapshot,
    ) -> Result<bool, Self::Error>;

    /// Restores the snapshot.
    ///
    /// The returned value is deliberately implementation-defined.
    fn restore(
        &self,
        snapshot: &Snapshot,
    ) -> Result<(), Self::Error>;
}

// =============================================================================
// Snapshot compatibility descriptor
// =============================================================================

/// Requirements that a target must satisfy before a snapshot may be restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotCompatibilityRequirements {
    /// Required schema namespace.
    pub schema_id: String,

    /// Minimum supported schema version.
    pub minimum_schema_version: u16,

    /// Optional exact runtime/provider implementation.
    pub implementation: Option<String>,

    /// Optional minimum runtime/provider version.
    pub minimum_implementation_version: Option<String>,

    /// Optional snapshot format.
    pub format: Option<String>,

    /// Whether provider-native restoration is mandatory.
    pub provider_native_restore: bool,

    /// Whether portability is mandatory.
    pub portable: bool,

    /// Additional capability requirements.
    pub capabilities: BTreeMap<String, String>,
}

impl SnapshotCompatibilityRequirements {
    /// Creates requirements derived from a snapshot.
    #[must_use]
    pub fn from_snapshot(
        snapshot: &Snapshot,
    ) -> Self {
        Self {
            schema_id: snapshot.schema_id.clone(),
            minimum_schema_version: snapshot.schema_version,
            implementation: snapshot
                .runtime
                .as_ref()
                .map(|runtime| runtime.implementation.clone()),
            minimum_implementation_version: snapshot
                .runtime
                .as_ref()
                .map(|runtime| runtime.version.clone()),
            format: snapshot
                .runtime
                .as_ref()
                .map(|runtime| runtime.format.clone()),
            provider_native_restore: snapshot
                .restoration
                .requires_target_support(),
            portable: snapshot.is_portable(),
            capabilities: BTreeMap::new(),
        }
    }

    /// Adds a capability requirement.
    pub fn with_capability(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, SnapshotError> {
        let key = key.into();

        validate_non_empty(
            &key,
            "capability_key",
        )?;

        self.capabilities.insert(key, value.into());

        Ok(self)
    }

    /// Validates the requirements.
    pub fn validate(&self) -> Result<(), SnapshotError> {
        validate_non_empty(
            &self.schema_id,
            "schema_id",
        )?;

        if self.minimum_schema_version == 0 {
            return Err(SnapshotError::InvalidSchemaVersion);
        }

        validate_optional_string(
            &self.implementation,
            "implementation",
        )?;

        validate_optional_string(
            &self.minimum_implementation_version,
            "minimum_implementation_version",
        )?;

        validate_optional_string(
            &self.format,
            "format",
        )?;

        Ok(())
    }
}

// =============================================================================
// Error
// =============================================================================

/// Snapshot-contract errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotError {
    /// An opaque identifier was empty.
    InvalidIdentifier {
        /// Identifier field name.
        field: &'static str,
    },

    /// A required string was empty.
    InvalidField {
        /// Field name.
        field: &'static str,
    },

    /// Snapshot schema is unsupported.
    UnsupportedSchema {
        /// Supplied schema identifier.
        schema_id: String,
    },

    /// Schema version is zero/invalid.
    InvalidSchemaVersion,

    /// Timestamp/position/state information was malformed.
    InvalidStateSemantics,

    /// Two payloads used the same artifact identity.
    DuplicateArtifact,

    /// Two logical qubits were repeated.
    DuplicateLogicalQubit,

    /// Two physical qubits were repeated.
    DuplicatePhysicalQubit,

    /// Logical/QEC state was not scoped to logical qubits.
    LogicalQecRequiresLogicalScope,

    /// Measurement kind did not use a measurement boundary.
    MeasurementBoundaryMismatch,

    /// Runtime/provider metadata is required.
    RuntimeMetadataRequired,

    /// Restoration semantics do not match the snapshot kind.
    InvalidRestorationSemantics,

    /// An opaque provider state was incorrectly marked portable.
    OpaqueStateCannotBePortable,

    /// Provider state was requested without permission.
    ProviderStateNotAllowed,

    /// Opaque provider state was requested without permission.
    OpaqueProviderStateNotAllowed,
}

impl fmt::Display for SnapshotError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid snapshot identifier: {field}")
            }
            Self::InvalidField { field } => {
                write!(formatter, "invalid snapshot field: {field}")
            }
            Self::UnsupportedSchema { schema_id } => {
                write!(
                    formatter,
                    "unsupported snapshot schema: {schema_id}"
                )
            }
            Self::InvalidSchemaVersion => {
                formatter.write_str("invalid snapshot schema version")
            }
            Self::InvalidStateSemantics => {
                formatter.write_str("invalid snapshot state semantics")
            }
            Self::DuplicateArtifact => {
                formatter.write_str("duplicate snapshot artifact")
            }
            Self::DuplicateLogicalQubit => {
                formatter.write_str("duplicate logical qubit")
            }
            Self::DuplicatePhysicalQubit => {
                formatter.write_str("duplicate physical qubit")
            }
            Self::LogicalQecRequiresLogicalScope => {
                formatter.write_str(
                    "logical/QEC snapshots require a logical-qubit scope",
                )
            }
            Self::MeasurementBoundaryMismatch => {
                formatter.write_str(
                    "measurement snapshot requires a measurement boundary",
                )
            }
            Self::RuntimeMetadataRequired => {
                formatter.write_str(
                    "runtime/provider metadata is required",
                )
            }
            Self::InvalidRestorationSemantics => {
                formatter.write_str(
                    "snapshot kind and restoration semantics are incompatible",
                )
            }
            Self::OpaqueStateCannotBePortable => {
                formatter.write_str(
                    "opaque provider state cannot be marked portable",
                )
            }
            Self::ProviderStateNotAllowed => {
                formatter.write_str(
                    "provider quantum state was requested without permission",
                )
            }
            Self::OpaqueProviderStateNotAllowed => {
                formatter.write_str(
                    "opaque provider state was requested without permission",
                )
            }
        }
    }
}

impl std::error::Error for SnapshotError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_non_empty(
    value: &str,
    field: &'static str,
) -> Result<(), SnapshotError> {
    if value.trim().is_empty() {
        return Err(SnapshotError::InvalidField { field });
    }

    Ok(())
}

fn validate_optional_string(
    value: &Option<String>,
    field: &'static str,
) -> Result<(), SnapshotError> {
    if let Some(value) = value {
        validate_non_empty(value, field)?;
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_id_rejects_empty_value() {
        assert!(SnapshotId::new("").is_err());
        assert!(SnapshotId::new("   ").is_err());
    }

    #[test]
    fn snapshot_id_accepts_opaque_value() {
        let id = SnapshotId::new("snapshot-any-size-system").unwrap();

        assert_eq!(id.as_str(), "snapshot-any-size-system");
    }

    #[test]
    fn logical_scope_rejects_duplicate_qubits() {
        let qubit = QubitId::new(0);

        assert!(
            SnapshotScope::logical_qubits([
                qubit,
                qubit,
            ])
            .is_err()
        );
    }

    #[test]
    fn payload_rejects_empty_artifact_id() {
        assert!(
            SnapshotPayload::new(
                "",
                "application/octet-stream",
                SnapshotPayloadLocation::reconstructible(),
            )
            .is_err()
        );
    }

    #[test]
    fn classical_snapshot_is_valid() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-1").unwrap(),
            SnapshotKind::ClassicalExecution,
            CheckpointBoundary::ClassicalExecution,
            RestorationSemantics::DirectClassical,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary-1".to_owned(),
            },
        );

        assert!(snapshot.is_ok());
    }

    #[test]
    fn measurement_snapshot_requires_measurement_boundary() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-2").unwrap(),
            SnapshotKind::MeasurementBoundary,
            CheckpointBoundary::ClassicalExecution,
            RestorationSemantics::MeasurementReconstruction,
            SnapshotScope::execution(),
            SnapshotPosition::MeasurementBoundary {
                measurement_id: "measurement-1".to_owned(),
            },
        );

        assert!(matches!(
            snapshot,
            Err(SnapshotError::MeasurementBoundaryMismatch)
        ));
    }

    #[test]
    fn provider_snapshot_requires_runtime_metadata() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-provider").unwrap(),
            SnapshotKind::ProviderSupportedQuantum,
            CheckpointBoundary::ProviderSupportedSnapshot,
            RestorationSemantics::ProviderNativeRestore,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary-provider".to_owned(),
            },
        );

        assert!(matches!(
            snapshot,
            Err(SnapshotError::RuntimeMetadataRequired)
        ));
    }

    #[test]
    fn logical_qec_requires_logical_scope() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-qec").unwrap(),
            SnapshotKind::LogicalQec,
            CheckpointBoundary::LogicalQec,
            RestorationSemantics::LogicalQecRestore,
            SnapshotScope::execution(),
            SnapshotPosition::RegionBoundary {
                region_id: "logical-region".to_owned(),
            },
        );

        assert!(matches!(
            snapshot,
            Err(SnapshotError::LogicalQecRequiresLogicalScope)
        ));
    }

    #[test]
    fn opaque_provider_state_is_not_portable() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-opaque").unwrap(),
            SnapshotKind::OpaqueProviderState,
            CheckpointBoundary::ProviderSupportedSnapshot,
            RestorationSemantics::ProviderOpaqueRestore,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary-opaque".to_owned(),
            },
        );

        assert!(matches!(
            snapshot,
            Err(SnapshotError::RuntimeMetadataRequired)
        ));
    }

    #[test]
    fn payload_artifact_ids_must_be_unique() {
        let payload_a = SnapshotPayload::new(
            "artifact",
            "application/octet-stream",
            SnapshotPayloadLocation::reconstructible(),
        )
        .unwrap();

        let payload_b = SnapshotPayload::new(
            "artifact",
            "application/octet-stream",
            SnapshotPayloadLocation::reconstructible(),
        )
        .unwrap();

        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-duplicate").unwrap(),
            SnapshotKind::ClassicalExecution,
            CheckpointBoundary::ClassicalExecution,
            RestorationSemantics::DirectClassical,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary".to_owned(),
            },
        )
        .unwrap()
        .with_payload(payload_a)
        .unwrap()
        .with_payload(payload_b);

        assert!(matches!(
            snapshot,
            Err(SnapshotError::DuplicateArtifact)
        ));
    }

    #[test]
    fn provider_snapshot_requires_explicit_runtime() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-provider-valid").unwrap(),
            SnapshotKind::ProviderSupportedQuantum,
            CheckpointBoundary::ProviderSupportedSnapshot,
            RestorationSemantics::ProviderNativeRestore,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary".to_owned(),
            },
        )
        .unwrap()
        .with_runtime(
            SnapshotRuntimeMetadata::new(
                "runtime",
                "1",
                "snapshot-v1",
            )
            .unwrap(),
        );

        assert!(snapshot.is_ok());
        assert!(
            snapshot
                .as_ref()
                .unwrap()
                .requires_provider_support()
        );
    }

    #[test]
    fn snapshot_serialization_is_deterministic_at_metadata_level() {
        let snapshot = Snapshot::new(
            SnapshotId::new("snapshot-deterministic").unwrap(),
            SnapshotKind::ClassicalExecution,
            CheckpointBoundary::ClassicalExecution,
            RestorationSemantics::DirectClassical,
            SnapshotScope::execution(),
            SnapshotPosition::ProgramBoundary {
                boundary_id: "boundary".to_owned(),
            },
        )
        .unwrap();

        let first =
            serde_json::to_string(&snapshot).unwrap();

        let second =
            serde_json::to_string(&snapshot).unwrap();

        assert_eq!(first, second);
    }
}