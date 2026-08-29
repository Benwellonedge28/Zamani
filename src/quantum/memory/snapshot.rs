//! Zamani Quantum Memory — Versioned Quantum-State Snapshots
//!
//! This module defines the provider-neutral snapshot contract for
//! `quantum::memory`.
//!
//! # Purpose
//!
//! A [`QuantumSnapshot`] is an immutable, self-describing representation of
//! quantum-memory state suitable for:
//!
//! - local save/restore;
//! - simulator pause/resume;
//! - migration between execution environments;
//! - CPU/GPU state transfer;
//! - distributed-memory checkpoint preparation;
//! - QPU/backend state handles;
//! - debugging and reproducibility;
//! - deterministic tests;
//! - long-running quantum workloads;
//! - future Danga-managed persistence;
//! - offline storage;
//! - transport between Zamani components.
//!
//! This module deliberately does **not** implement a particular quantum-state
//! representation. It does not know how a state vector, density matrix,
//! stabilizer tableau, sparse state, tensor network, GPU buffer, distributed
//! partition, or QPU-native state is internally stored.
//!
//! Instead, it defines a stable envelope around an opaque representation
//! payload.
//!
//! # Architectural boundary
//!
//! ```text
//!                    quantum::ir
//!                        |
//!                        v
//!                 execution/runtime
//!                        |
//!                        v
//!                 quantum::memory
//!                        |
//!          +-------------+-------------+
//!          |             |             |
//!          v             v             v
//!     StateVector   DensityMatrix   BackendState
//!          |             |             |
//!          +-------------+-------------+
//!                        |
//!                        v
//!                  QuantumSnapshot
//!                        |
//!             +----------+----------+
//!             |                     |
//!             v                     v
//!       local persistence       transport
//! ```
//!
//! `snapshot.rs` owns the persistence-level identity and invariants.
//! Representation implementations own the meaning of their payload bytes.
//!
//! # Critical design rule
//!
//! A snapshot MUST NOT contain raw pointers, device pointers, references,
//! allocator-specific addresses, thread handles, mutexes, or other
//! process-local resources.
//!
//! A snapshot may contain an opaque provider-defined identifier or serialized
//! provider state, but restoring that state is always an explicit operation
//! performed by the corresponding provider.
//!
//! # Hardware neutrality
//!
//! The snapshot contract is intentionally independent of:
//!
//! - IBM QPUs;
//! - Google Quantum hardware;
//! - Quantinuum;
//! - IonQ;
//! - Rigetti;
//! - IQM;
//! - D-Wave;
//! - Pasqal;
//! - neutral-atom systems;
//! - photonic systems;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - MPI;
//! - RDMA;
//! - any particular simulator.
//!
//! Hardware-specific adapters belong under the hardware/backend subsystem.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//!
//! # Integration contract
//!
//! This module intentionally depends only on foundational memory contracts:
//!
//! - `types.rs` for `SnapshotId`, `ByteCount`, and `QubitCount`;
//! - `errors.rs` for `MemoryError`.
//!
//! Future modules integrate with this file through:
//!
//! - [`SnapshotMetadata`];
//! - [`SnapshotHeader`];
//! - [`SnapshotPayload`];
//! - [`QuantumSnapshot`];
//! - [`SnapshotBuilder`];
//! - [`SnapshotValidationPolicy`];
//! - [`SnapshotRestorePolicy`];
//! - [`SnapshotProvider`];
//!
//! This means later implementations of `state_vector.rs`,
//! `density_matrix.rs`, `stabilizer.rs`, `sparse.rs`, `tensor_network.rs`,
//! `backend_state.rs`, `gpu.rs`, and `distributed.rs` do not require changes
//! to this file merely to become snapshot-capable.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use std::fmt;

use super::errors::{MemoryError, MemoryErrorCode};
use super::types::{ByteCount, QubitCount, SnapshotId};

// =============================================================================
// Public schema constants
// =============================================================================

/// Stable identifier for the Zamani quantum-memory snapshot format.
pub const SNAPSHOT_SCHEMA_ID: &str = "zamani.quantum.memory.snapshot";

/// Current semantic snapshot format version.
///
/// The major version identifies incompatible structural changes.
///
/// Minor/patch evolution is handled by the compatibility policy documented
/// below rather than by changing the envelope's fundamental interpretation.
pub const SNAPSHOT_FORMAT_MAJOR: u16 = 1;
pub const SNAPSHOT_FORMAT_MINOR: u16 = 0;

/// A stable human-readable format name.
pub const SNAPSHOT_FORMAT_NAME: &str = "Zamani Quantum Memory Snapshot";

/// Four-byte binary magic value for an encoded snapshot.
///
/// A serializer may prepend/encode this value before its own payload. The
/// snapshot object itself remains format-neutral.
pub const SNAPSHOT_MAGIC: [u8; 4] = *b"ZQMS";

/// Maximum supported snapshot schema major version.
///
/// A reader must reject versions newer than the maximum it explicitly
/// understands rather than guessing their meaning.
pub const MAX_SUPPORTED_MAJOR_VERSION: u16 = SNAPSHOT_FORMAT_MAJOR;

/// Maximum length of a representation/provider identifier.
pub const MAX_IDENTIFIER_LENGTH: usize = 256;

/// Maximum length of a snapshot description.
pub const MAX_DESCRIPTION_LENGTH: usize = 4096;

/// Maximum number of metadata labels.
pub const MAX_LABELS: usize = 128;

/// Maximum label length.
pub const MAX_LABEL_LENGTH: usize = 256;

/// Maximum payload size accepted by the snapshot layer when no external
/// resource policy overrides it.
///
/// This is intentionally conservative. Production callers should normally
/// supply their own [`SnapshotValidationPolicy`] derived from
/// `MemoryLimits`.
pub const DEFAULT_MAX_PAYLOAD_BYTES: u64 = 16 * 1024 * 1024 * 1024;

// =============================================================================
// Snapshot format version
// =============================================================================

/// Semantic version of the snapshot envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotFormatVersion {
    /// Major schema version.
    pub major: u16,

    /// Minor schema version.
    pub minor: u16,
}

impl SnapshotFormatVersion {
    /// Current supported snapshot format.
    pub const CURRENT: Self = Self {
        major: SNAPSHOT_FORMAT_MAJOR,
        minor: SNAPSHOT_FORMAT_MINOR,
    };

    /// Creates a format version.
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns whether this version is readable by this implementation.
    ///
    /// Older major versions may be accepted only when explicitly declared
    /// compatible. Newer major versions are always rejected.
    pub const fn is_supported_major(self) -> bool {
        self.major <= MAX_SUPPORTED_MAJOR_VERSION
    }

    /// Returns whether this version is the current version.
    pub const fn is_current(self) -> bool {
        self.major == Self::CURRENT.major && self.minor == Self::CURRENT.minor
    }
}

impl Default for SnapshotFormatVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for SnapshotFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

// =============================================================================
// State representation
// =============================================================================

/// Representation-independent identifier for the state stored in a snapshot.
///
/// This enum intentionally describes *storage semantics*, not implementation
/// types. A backend may support additional representations through
/// [`StateRepresentation::BackendNative`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateRepresentation {
    /// Dense pure-state vector.
    StateVector,

    /// Full density matrix.
    DensityMatrix,

    /// Stabilizer/tableau representation.
    Stabilizer,

    /// Sparse state representation.
    Sparse,

    /// Tensor-network representation.
    TensorNetwork,

    /// Provider-native state representation.
    ///
    /// `provider` identifies the provider using a stable logical name, never
    /// a pointer or process-local address.
    BackendNative {
        provider: String,
    },

    /// A representation supplied by an extension module.
    ///
    /// The extension name must be stable and versioned by the provider.
    Extension {
        name: String,
    },
}

impl StateRepresentation {
    /// Returns a stable representation identifier.
    pub fn identifier(&self) -> String {
        match self {
            Self::StateVector => "state-vector".to_owned(),
            Self::DensityMatrix => "density-matrix".to_owned(),
            Self::Stabilizer => "stabilizer".to_owned(),
            Self::Sparse => "sparse".to_owned(),
            Self::TensorNetwork => "tensor-network".to_owned(),
            Self::BackendNative { provider } => {
                format!("backend-native:{provider}")
            }
            Self::Extension { name } => format!("extension:{name}"),
        }
    }

    /// Returns true when the representation requires a provider to restore.
    pub const fn is_backend_native(&self) -> bool {
        matches!(self, Self::BackendNative { .. })
    }
}

// =============================================================================
// Storage location
// =============================================================================

/// Location from which a snapshot was captured.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotStorageLocation {
    /// Ordinary host memory.
    Host,

    /// Pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared memory.
    Unified,

    /// Distributed memory across multiple execution nodes.
    Distributed,

    /// Remote/provider-managed quantum hardware.
    Remote,

    /// Provider-defined location.
    Custom(String),
}

impl SnapshotStorageLocation {
    /// Returns a stable identifier.
    pub fn identifier(&self) -> String {
        match self {
            Self::Host => "host".to_owned(),
            Self::PinnedHost => "pinned-host".to_owned(),
            Self::Device => "device".to_owned(),
            Self::Unified => "unified".to_owned(),
            Self::Distributed => "distributed".to_owned(),
            Self::Remote => "remote".to_owned(),
            Self::Custom(value) => format!("custom:{value}"),
        }
    }
}

// =============================================================================
// Scalar precision
// =============================================================================

/// Numeric precision associated with a snapshot payload.
///
/// Backend-native representations may use `BackendDefined`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotPrecision {
    /// 32-bit real components.
    F32,

    /// 64-bit real components.
    F64,

    /// Extended precision supplied by a provider.
    Extended,

    /// Arbitrary/provider-defined precision.
    BackendDefined,
}

// =============================================================================
// Endianness
// =============================================================================

/// Byte order declared by an encoded payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotEndianness {
    Little,
    Big,
    Native,
}

impl SnapshotEndianness {
    /// Returns the host's actual byte order.
    pub const fn host() -> Self {
        if cfg!(target_endian = "little") {
            Self::Little
        } else {
            Self::Big
        }
    }

    /// Returns true when this declaration is portable and explicit.
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::Native)
    }
}

// =============================================================================
// Payload encoding
// =============================================================================

/// Encoding used for the opaque snapshot payload.
///
/// The snapshot layer does not implement the encoder itself. The actual
/// encoding belongs to `serialization.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotPayloadEncoding {
    /// Raw canonical bytes.
    Raw,

    /// A serialization format identified by a stable name and version.
    Named {
        name: String,
        version: String,
    },

    /// Provider-defined encoding.
    ProviderDefined {
        provider: String,
        version: String,
    },
}

impl SnapshotPayloadEncoding {
    /// Returns a stable human-readable identifier.
    pub fn identifier(&self) -> String {
        match self {
            Self::Raw => "raw".to_owned(),
            Self::Named { name, version } => {
                format!("{name}@{version}")
            }
            Self::ProviderDefined { provider, version } => {
                format!("provider:{provider}@{version}")
            }
        }
    }
}

// =============================================================================
// Integrity
// =============================================================================

/// Integrity algorithm used to protect a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotIntegrityAlgorithm {
    /// No integrity value is present.
    None,

    /// SHA-256 digest.
    Sha256,

    /// SHA-512 digest.
    Sha512,

    /// Provider-defined integrity mechanism.
    ProviderDefined {
        provider: String,
        algorithm: String,
    },
}

/// Integrity information for a snapshot.
///
/// The digest itself is stored as bytes and is not interpreted by this module.
/// The serialization layer is responsible for computing/verifying it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotIntegrity {
    /// Algorithm used to compute the digest.
    pub algorithm: SnapshotIntegrityAlgorithm,

    /// Digest bytes.
    pub digest: Vec<u8>,
}

impl SnapshotIntegrity {
    /// Creates an integrity descriptor.
    pub fn new(
        algorithm: SnapshotIntegrityAlgorithm,
        digest: Vec<u8>,
    ) -> Result<Self, MemoryError> {
        let value = Self { algorithm, digest };
        value.validate()?;
        Ok(value)
    }

    /// Validates the digest length for algorithms with fixed digest sizes.
    pub fn validate(&self) -> Result<(), MemoryError> {
        match &self.algorithm {
            SnapshotIntegrityAlgorithm::None => {
                if !self.digest.is_empty() {
                    return Err(MemoryError::validation(
                        MemoryErrorCode::InvalidSnapshot,
                        "integrity digest must be empty when integrity algorithm is None",
                    ));
                }
            }

            SnapshotIntegrityAlgorithm::Sha256 => {
                if self.digest.len() != 32 {
                    return Err(MemoryError::validation(
                        MemoryErrorCode::IntegrityError,
                        "SHA-256 snapshot integrity value must contain 32 bytes",
                    ));
                }
            }

            SnapshotIntegrityAlgorithm::Sha512 => {
                if self.digest.len() != 64 {
                    return Err(MemoryError::validation(
                        MemoryErrorCode::IntegrityError,
                        "SHA-512 snapshot integrity value must contain 64 bytes",
                    ));
                }
            }

            SnapshotIntegrityAlgorithm::ProviderDefined { .. } => {
                // Provider-defined algorithms own their digest-size contract.
            }
        }

        Ok(())
    }
}

// =============================================================================
// Snapshot metadata
// =============================================================================

/// Immutable descriptive metadata attached to a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// Optional user/application description.
    pub description: Option<String>,

    /// Stable labels useful for filtering or provenance.
    pub labels: Vec<String>,

    /// Optional Zamani language/runtime version.
    pub zamani_version: Option<String>,

    /// Optional source-program/circuit identity.
    ///
    /// This is an opaque identifier. The snapshot layer does not interpret it.
    pub program_identity: Option<String>,

    /// Optional execution identity.
    pub execution_identity: Option<String>,

    /// Optional backend/provider name.
    pub provider: Option<String>,

    /// Optional backend/provider version.
    pub provider_version: Option<String>,
}

impl Default for SnapshotMetadata {
    fn default() -> Self {
        Self {
            description: None,
            labels: Vec::new(),
            zamani_version: None,
            program_identity: None,
            execution_identity: None,
            provider: None,
            provider_version: None,
        }
    }
}

impl SnapshotMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates metadata without allocating additional resources.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if let Some(description) = &self.description {
            validate_string_length(
                description,
                MAX_DESCRIPTION_LENGTH,
                "snapshot description",
            )?;
        }

        if self.labels.len() > MAX_LABELS {
            return Err(MemoryError::validation(
                MemoryErrorCode::InvalidSnapshot,
                "snapshot contains too many labels",
            ));
        }

        for label in &self.labels {
            validate_string_length(label, MAX_LABEL_LENGTH, "snapshot label")?;
        }

        for (name, value) in [
            ("Zamani version", self.zamani_version.as_deref()),
            ("program identity", self.program_identity.as_deref()),
            ("execution identity", self.execution_identity.as_deref()),
            ("provider", self.provider.as_deref()),
            ("provider version", self.provider_version.as_deref()),
        ] {
            if let Some(value) = value {
                validate_string_length(value, MAX_IDENTIFIER_LENGTH, name)?;
            }
        }

        Ok(())
    }
}

// =============================================================================
// Snapshot header
// =============================================================================

/// Self-describing immutable snapshot header.
///
/// The header contains enough information for a loader to decide whether it
/// can safely interpret a snapshot before allocating the payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotHeader {
    /// Binary magic value.
    pub magic: [u8; 4],

    /// Snapshot schema identifier.
    pub schema_id: String,

    /// Snapshot schema version.
    pub format_version: SnapshotFormatVersion,

    /// Unique snapshot identity.
    pub snapshot_id: SnapshotId,

    /// Number of logical qubits represented.
    pub qubit_count: QubitCount,

    /// State representation.
    pub representation: StateRepresentation,

    /// Storage location at capture time.
    pub storage_location: SnapshotStorageLocation,

    /// Numeric precision.
    pub precision: SnapshotPrecision,

    /// Byte order used by the encoded payload.
    pub endianness: SnapshotEndianness,

    /// Payload encoding.
    pub payload_encoding: SnapshotPayloadEncoding,

    /// Exact payload size.
    pub payload_size: ByteCount,

    /// Optional integrity metadata.
    pub integrity: SnapshotIntegrity,

    /// Additional descriptive metadata.
    pub metadata: SnapshotMetadata,
}

impl SnapshotHeader {
    /// Validates the complete header.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.magic != SNAPSHOT_MAGIC {
            return Err(MemoryError::validation(
                MemoryErrorCode::InvalidSnapshot,
                "snapshot magic does not identify a Zamani quantum-memory snapshot",
            ));
        }

        if self.schema_id != SNAPSHOT_SCHEMA_ID {
            return Err(MemoryError::validation(
                MemoryErrorCode::InvalidSnapshot,
                "unsupported snapshot schema identifier",
            ));
        }

        if !self.format_version.is_supported_major() {
            return Err(MemoryError::validation(
                MemoryErrorCode::UnsupportedSchemaVersion,
                "snapshot uses a newer unsupported major schema version",
            ));
        }

        validate_representation(&self.representation)?;

        validate_storage_location(&self.storage_location)?;

        validate_payload_encoding(&self.payload_encoding)?;

        self.integrity.validate()?;

        self.metadata.validate()?;

        Ok(())
    }
}

// =============================================================================
// Snapshot payload
// =============================================================================

/// Opaque immutable state payload.
///
/// The payload is deliberately represented as bytes rather than a concrete
/// state-vector/density-matrix/etc. type.
///
/// The representation-specific module owns interpretation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotPayload {
    /// Encoded state bytes.
    pub bytes: Vec<u8>,
}

impl SnapshotPayload {
    /// Creates a payload after checking its declared maximum size.
    pub fn new(
        bytes: Vec<u8>,
        max_size: ByteCount,
    ) -> Result<Self, MemoryError> {
        let actual = u64::try_from(bytes.len()).map_err(|_| {
            MemoryError::arithmetic_overflow(
                "snapshot payload length cannot be represented safely",
            )
        })?;

        if actual > max_size.get() {
            return Err(MemoryError::resource_limit(
                MemoryErrorCode::MemoryLimitExceeded,
                format!(
                    "snapshot payload of {actual} bytes exceeds configured limit of {} bytes",
                    max_size.get()
                ),
            ));
        }

        Ok(Self { bytes })
    }

    /// Returns the exact payload size.
    pub fn size(&self) -> ByteCount {
        ByteCount::new(self.bytes.len() as u64)
    }

    /// Returns whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a read-only view of the payload.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

// =============================================================================
// Complete snapshot
// =============================================================================

/// Immutable quantum-memory snapshot.
///
/// A snapshot is logically immutable after construction. The Rust API exposes
/// no mutation operation that can change the header or payload in place.
///
/// A new state therefore requires a new snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantumSnapshot {
    /// Self-describing header.
    pub header: SnapshotHeader,

    /// Opaque representation payload.
    pub payload: SnapshotPayload,
}

impl QuantumSnapshot {
    /// Constructs and validates a complete snapshot.
    pub fn new(
        header: SnapshotHeader,
        payload: SnapshotPayload,
    ) -> Result<Self, MemoryError> {
        let snapshot = Self { header, payload };
        snapshot.validate()?;
        Ok(snapshot)
    }

    /// Returns the snapshot identity.
    pub const fn id(&self) -> SnapshotId {
        self.header.snapshot_id
    }

    /// Returns the represented qubit count.
    pub const fn qubit_count(&self) -> QubitCount {
        self.header.qubit_count
    }

    /// Returns the state representation.
    pub const fn representation(&self) -> &StateRepresentation {
        &self.header.representation
    }

    /// Returns the payload size.
    pub fn payload_size(&self) -> ByteCount {
        self.payload.size()
    }

    /// Validates header/payload consistency.
    pub fn validate(&self) -> Result<(), MemoryError> {
        self.header.validate()?;

        if self.header.payload_size != self.payload.size() {
            return Err(MemoryError::validation(
                MemoryErrorCode::InvalidSnapshot,
                "snapshot header payload size does not match actual payload size",
            ));
        }

        if self.header.payload_size.get() > DEFAULT_MAX_PAYLOAD_BYTES {
            return Err(MemoryError::resource_limit(
                MemoryErrorCode::MemoryLimitExceeded,
                "snapshot payload exceeds the default snapshot resource limit",
            ));
        }

        Ok(())
    }

    /// Validates the snapshot against an explicit policy.
    pub fn validate_with(
        &self,
        policy: &SnapshotValidationPolicy,
    ) -> Result<(), MemoryError> {
        self.validate()?;

        policy.validate_header(&self.header)?;
        policy.validate_payload(&self.payload)?;

        Ok(())
    }

    /// Returns a read-only payload view.
    pub fn payload_bytes(&self) -> &[u8] {
        self.payload.as_bytes()
    }
}

// =============================================================================
// Snapshot validation policy
// =============================================================================

/// Resource and compatibility policy used when accepting a snapshot.
///
/// This policy exists so callers can validate a snapshot *before* handing it
/// to a state representation or backend provider.
///
/// The policy does not allocate memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotValidationPolicy {
    /// Maximum accepted payload size.
    pub max_payload_bytes: ByteCount,

    /// Maximum accepted qubit count.
    pub max_qubits: QubitCount,

    /// Whether backend-native snapshots are permitted.
    pub allow_backend_native: bool,

    /// Whether provider-defined encodings are permitted.
    pub allow_provider_defined_encoding: bool,

    /// Whether custom storage locations are permitted.
    pub allow_custom_storage_location: bool,

    /// Whether snapshots without integrity metadata are permitted.
    pub require_integrity: bool,

    /// Whether `Native` byte order is accepted.
    pub allow_native_endianness: bool,
}

impl Default for SnapshotValidationPolicy {
    fn default() -> Self {
        Self {
            max_payload_bytes: ByteCount::new(DEFAULT_MAX_PAYLOAD_BYTES),
            max_qubits: QubitCount::new(usize::MAX),
            allow_backend_native: true,
            allow_provider_defined_encoding: true,
            allow_custom_storage_location: true,
            require_integrity: false,
            allow_native_endianness: false,
        }
    }
}

impl SnapshotValidationPolicy {
    /// Creates a conservative policy suitable for portable persistence.
    pub fn portable() -> Self {
        Self {
            allow_backend_native: false,
            allow_provider_defined_encoding: false,
            allow_custom_storage_location: false,
            require_integrity: true,
            allow_native_endianness: false,
            ..Self::default()
        }
    }

    /// Validates a snapshot header.
    pub fn validate_header(
        &self,
        header: &SnapshotHeader,
    ) -> Result<(), MemoryError> {
        if header.qubit_count.get() > self.max_qubits.get() {
            return Err(MemoryError::resource_limit(
                MemoryErrorCode::MemoryLimitExceeded,
                format!(
                    "snapshot contains {} qubits but policy permits at most {}",
                    header.qubit_count.get(),
                    self.max_qubits.get()
                ),
            ));
        }

        if header.payload_size.get() > self.max_payload_bytes.get() {
            return Err(MemoryError::resource_limit(
                MemoryErrorCode::MemoryLimitExceeded,
                format!(
                    "snapshot contains {} payload bytes but policy permits at most {}",
                    header.payload_size.get(),
                    self.max_payload_bytes.get()
                ),
            ));
        }

        if header.representation.is_backend_native()
            && !self.allow_backend_native
        {
            return Err(MemoryError::unsupported(
                MemoryErrorCode::UnsupportedRepresentation,
                "backend-native snapshots are disabled by the validation policy",
            ));
        }

        if matches!(
            header.payload_encoding,
            SnapshotPayloadEncoding::ProviderDefined { .. }
        ) && !self.allow_provider_defined_encoding
        {
            return Err(MemoryError::unsupported(
                MemoryErrorCode::UnsupportedOperation,
                "provider-defined snapshot encodings are disabled by the validation policy",
            ));
        }

        if matches!(
            header.storage_location,
            SnapshotStorageLocation::Custom(_)
        ) && !self.allow_custom_storage_location
        {
            return Err(MemoryError::unsupported(
                MemoryErrorCode::UnsupportedStorageLocation,
                "custom snapshot storage locations are disabled by the validation policy",
            ));
        }

        if !header.endianness.is_explicit() && !self.allow_native_endianness {
            return Err(MemoryError::validation(
                MemoryErrorCode::InvalidSnapshot,
                "portable snapshots must declare an explicit byte order",
            ));
        }

        if self.require_integrity
            && matches!(
                header.integrity.algorithm,
                SnapshotIntegrityAlgorithm::None
            )
        {
            return Err(MemoryError::validation(
                MemoryErrorCode::IntegrityError,
                "snapshot integrity is required by the validation policy",
            ));
        }

        Ok(())
    }

    /// Validates payload size without interpreting its contents.
    pub fn validate_payload(
        &self,
        payload: &SnapshotPayload,
    ) -> Result<(), MemoryError> {
        if payload.size().get() > self.max_payload_bytes.get() {
            return Err(MemoryError::resource_limit(
                MemoryErrorCode::MemoryLimitExceeded,
                "snapshot payload exceeds validation-policy limit",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Restore policy
// =============================================================================

/// Policy governing restoration of a snapshot.
///
/// Restoration is intentionally separate from validation. A snapshot can be
/// valid but still unsuitable for a particular execution environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRestorePolicy {
    /// Whether representation conversion is allowed during restore.
    pub allow_representation_conversion: bool,

    /// Whether storage migration is allowed during restore.
    pub allow_storage_migration: bool,

    /// Whether backend-native state can be restored only by the same provider.
    pub require_matching_backend_provider: bool,

    /// Whether precision conversion is allowed.
    pub allow_precision_conversion: bool,

    /// Whether the restore operation must preserve exact payload bytes.
    pub require_exact_payload: bool,
}

impl Default for SnapshotRestorePolicy {
    fn default() -> Self {
        Self {
            allow_representation_conversion: false,
            allow_storage_migration: true,
            require_matching_backend_provider: true,
            allow_precision_conversion: false,
            require_exact_payload: true,
        }
    }
}

impl SnapshotRestorePolicy {
    /// Strict policy for deterministic restore.
    pub const fn strict() -> Self {
        Self {
            allow_representation_conversion: false,
            allow_storage_migration: false,
            require_matching_backend_provider: true,
            allow_precision_conversion: false,
            require_exact_payload: true,
        }
    }

    /// Returns whether a representation change is permitted.
    pub const fn permits_representation_conversion(&self) -> bool {
        self.allow_representation_conversion
    }
}

// =============================================================================
// Snapshot builder
// =============================================================================

/// Builder for constructing a validated snapshot without exposing mutable
/// snapshot state after construction.
#[derive(Debug, Clone)]
pub struct SnapshotBuilder {
    snapshot_id: SnapshotId,
    qubit_count: QubitCount,
    representation: StateRepresentation,
    storage_location: SnapshotStorageLocation,
    precision: SnapshotPrecision,
    endianness: SnapshotEndianness,
    payload_encoding: SnapshotPayloadEncoding,
    integrity: SnapshotIntegrity,
    metadata: SnapshotMetadata,
    max_payload_bytes: ByteCount,
}

impl SnapshotBuilder {
    /// Creates a builder using current snapshot-format defaults.
    pub fn new(
        snapshot_id: SnapshotId,
        qubit_count: QubitCount,
        representation: StateRepresentation,
    ) -> Self {
        Self {
            snapshot_id,
            qubit_count,
            representation,
            storage_location: SnapshotStorageLocation::Host,
            precision: SnapshotPrecision::F64,
            endianness: SnapshotEndianness::host(),
            payload_encoding: SnapshotPayloadEncoding::Raw,
            integrity: SnapshotIntegrity {
                algorithm: SnapshotIntegrityAlgorithm::None,
                digest: Vec::new(),
            },
            metadata: SnapshotMetadata::default(),
            max_payload_bytes: ByteCount::new(DEFAULT_MAX_PAYLOAD_BYTES),
        }
    }

    /// Sets the storage location.
    pub fn storage_location(
        mut self,
        value: SnapshotStorageLocation,
    ) -> Self {
        self.storage_location = value;
        self
    }

    /// Sets the scalar precision.
    pub fn precision(mut self, value: SnapshotPrecision) -> Self {
        self.precision = value;
        self
    }

    /// Sets the payload byte order.
    pub fn endianness(mut self, value: SnapshotEndianness) -> Self {
        self.endianness = value;
        self
    }

    /// Sets the payload encoding.
    pub fn payload_encoding(
        mut self,
        value: SnapshotPayloadEncoding,
    ) -> Self {
        self.payload_encoding = value;
        self
    }

    /// Sets integrity metadata.
    pub fn integrity(mut self, value: SnapshotIntegrity) -> Self {
        self.integrity = value;
        self
    }

    /// Sets descriptive metadata.
    pub fn metadata(mut self, value: SnapshotMetadata) -> Self {
        self.metadata = value;
        self
    }

    /// Sets the construction-time payload limit.
    pub fn max_payload_bytes(mut self, value: ByteCount) -> Self {
        self.max_payload_bytes = value;
        self
    }

    /// Builds a validated immutable snapshot.
    pub fn build(self, bytes: Vec<u8>) -> Result<QuantumSnapshot, MemoryError> {
        let payload = SnapshotPayload::new(bytes, self.max_payload_bytes)?;

        let header = SnapshotHeader {
            magic: SNAPSHOT_MAGIC,
            schema_id: SNAPSHOT_SCHEMA_ID.to_owned(),
            format_version: SnapshotFormatVersion::CURRENT,
            snapshot_id: self.snapshot_id,
            qubit_count: self.qubit_count,
            representation: self.representation,
            storage_location: self.storage_location,
            precision: self.precision,
            endianness: self.endianness,
            payload_encoding: self.payload_encoding,
            payload_size: payload.size(),
            integrity: self.integrity,
            metadata: self.metadata,
        };

        QuantumSnapshot::new(header, payload)
    }
}

// =============================================================================
// Provider integration
// =============================================================================

/// Provider-neutral interface for snapshot capture.
///
/// State representations, simulator backends, GPU providers, distributed
/// execution engines, and QPU adapters can implement this trait.
///
/// The trait deliberately returns an owned snapshot. No borrow from a live
/// provider state can escape through the snapshot.
pub trait SnapshotProvider {
    /// Provider-specific state type.
    type State;

    /// Captures an immutable snapshot.
    fn snapshot(
        &self,
        state: &Self::State,
    ) -> Result<QuantumSnapshot, MemoryError>;

    /// Validates whether this provider can restore the snapshot.
    fn can_restore(
        &self,
        snapshot: &QuantumSnapshot,
        policy: &SnapshotRestorePolicy,
    ) -> Result<(), MemoryError>;

    /// Restores the snapshot into provider-owned state.
    ///
    /// The provider owns interpretation of the opaque payload.
    fn restore(
        &self,
        snapshot: &QuantumSnapshot,
        policy: &SnapshotRestorePolicy,
    ) -> Result<Self::State, MemoryError>;
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_string_length(
    value: &str,
    max: usize,
    field: &str,
) -> Result<(), MemoryError> {
    if value.len() > max {
        return Err(MemoryError::validation(
            MemoryErrorCode::InvalidSnapshot,
            format!("{field} exceeds the maximum permitted length"),
        ));
    }

    if value.chars().any(|character| character == '\0') {
        return Err(MemoryError::validation(
            MemoryErrorCode::InvalidSnapshot,
            format!("{field} contains a NUL character"),
        ));
    }

    Ok(())
}

fn validate_representation(
    representation: &StateRepresentation,
) -> Result<(), MemoryError> {
    match representation {
        StateRepresentation::BackendNative { provider } => {
            validate_string_length(
                provider,
                MAX_IDENTIFIER_LENGTH,
                "backend provider identifier",
            )?;
        }

        StateRepresentation::Extension { name } => {
            validate_string_length(
                name,
                MAX_IDENTIFIER_LENGTH,
                "extension representation name",
            )?;
        }

        _ => {}
    }

    Ok(())
}

fn validate_storage_location(
    location: &SnapshotStorageLocation,
) -> Result<(), MemoryError> {
    if let SnapshotStorageLocation::Custom(value) = location {
        validate_string_length(
            value,
            MAX_IDENTIFIER_LENGTH,
            "custom storage-location identifier",
        )?;
    }

    Ok(())
}

fn validate_payload_encoding(
    encoding: &SnapshotPayloadEncoding,
) -> Result<(), MemoryError> {
    match encoding {
        SnapshotPayloadEncoding::Raw => {}

        SnapshotPayloadEncoding::Named { name, version } => {
            validate_string_length(
                name,
                MAX_IDENTIFIER_LENGTH,
                "snapshot encoding name",
            )?;

            validate_string_length(
                version,
                MAX_IDENTIFIER_LENGTH,
                "snapshot encoding version",
            )?;
        }

        SnapshotPayloadEncoding::ProviderDefined {
            provider,
            version,
        } => {
            validate_string_length(
                provider,
                MAX_IDENTIFIER_LENGTH,
                "snapshot encoding provider",
            )?;

            validate_string_length(
                version,
                MAX_IDENTIFIER_LENGTH,
                "snapshot encoding version",
            )?;
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot_id() -> SnapshotId {
        SnapshotId::new(1)
    }

    #[test]
    fn current_format_is_supported() {
        assert!(SnapshotFormatVersion::CURRENT.is_supported_major());
        assert!(SnapshotFormatVersion::CURRENT.is_current());
    }

    #[test]
    fn builder_creates_valid_snapshot() {
        let snapshot = SnapshotBuilder::new(
            snapshot_id(),
            QubitCount::new(2),
            StateRepresentation::StateVector,
        )
        .build(vec![1, 2, 3, 4])
        .expect("snapshot construction should succeed");

        assert_eq!(snapshot.id(), snapshot_id());
        assert_eq!(snapshot.qubit_count(), QubitCount::new(2));
        assert_eq!(snapshot.payload_size(), ByteCount::new(4));
        assert_eq!(snapshot.header.magic, SNAPSHOT_MAGIC);
        assert_eq!(snapshot.header.schema_id, SNAPSHOT_SCHEMA_ID);
    }

    #[test]
    fn header_and_payload_sizes_must_match() {
        let payload = SnapshotPayload {
            bytes: vec![1, 2, 3],
        };

        let header = SnapshotHeader {
            magic: SNAPSHOT_MAGIC,
            schema_id: SNAPSHOT_SCHEMA_ID.to_owned(),
            format_version: SnapshotFormatVersion::CURRENT,
            snapshot_id: snapshot_id(),
            qubit_count: QubitCount::new(2),
            representation: StateRepresentation::StateVector,
            storage_location: SnapshotStorageLocation::Host,
            precision: SnapshotPrecision::F64,
            endianness: SnapshotEndianness::Little,
            payload_encoding: SnapshotPayloadEncoding::Raw,
            payload_size: ByteCount::new(4),
            integrity: SnapshotIntegrity {
                algorithm: SnapshotIntegrityAlgorithm::None,
                digest: Vec::new(),
            },
            metadata: SnapshotMetadata::default(),
        };

        let result = QuantumSnapshot::new(header, payload);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let mut snapshot = SnapshotBuilder::new(
            snapshot_id(),
            QubitCount::new(1),
            StateRepresentation::StateVector,
        )
        .build(vec![1])
        .expect("snapshot construction should succeed");

        snapshot.header.magic = *b"BAD!";

        assert!(snapshot.validate().is_err());
    }

    #[test]
    fn sha256_requires_32_bytes() {
        let result = SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha256,
            vec![0; 31],
        );

        assert!(result.is_err());
    }

    #[test]
    fn sha512_requires_64_bytes() {
        let result = SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::Sha512,
            vec![0; 63],
        );

        assert!(result.is_err());
    }

    #[test]
    fn none_integrity_requires_empty_digest() {
        let result = SnapshotIntegrity::new(
            SnapshotIntegrityAlgorithm::None,
            vec![0],
        );

        assert!(result.is_err());
    }

    #[test]
    fn portable_policy_rejects_backend_native_state() {
        let snapshot = SnapshotBuilder::new(
            snapshot_id(),
            QubitCount::new(1),
            StateRepresentation::BackendNative {
                provider: "example".to_owned(),
            },
        )
        .integrity(
            SnapshotIntegrity::new(
                SnapshotIntegrityAlgorithm::Sha256,
                vec![0; 32],
            )
            .expect("valid integrity"),
        )
        .build(vec![1])
        .expect("snapshot construction should succeed");

        assert!(
            snapshot
                .validate_with(&SnapshotValidationPolicy::portable())
                .is_err()
        );
    }

    #[test]
    fn payload_limit_is_enforced() {
        let result = SnapshotPayload::new(
            vec![1, 2, 3, 4],
            ByteCount::new(3),
        );

        assert!(result.is_err());
    }

    #[test]
    fn metadata_limits_are_enforced() {
        let mut metadata = SnapshotMetadata::default();
        metadata.description = Some("x".repeat(MAX_DESCRIPTION_LENGTH + 1));

        assert!(metadata.validate().is_err());
    }

    #[test]
    fn provider_native_representation_has_stable_identifier() {
        let representation = StateRepresentation::BackendNative {
            provider: "zamani-qpu".to_owned(),
        };

        assert_eq!(
            representation.identifier(),
            "backend-native:zamani-qpu"
        );
    }

    #[test]
    fn snapshot_is_cloneable_and_value_semantic() {
        let snapshot = SnapshotBuilder::new(
            snapshot_id(),
            QubitCount::new(1),
            StateRepresentation::StateVector,
        )
        .build(vec![0, 1])
        .expect("snapshot construction should succeed");

        let cloned = snapshot.clone();

        assert_eq!(snapshot, cloned);
    }

    #[test]
    fn explicit_endianness_is_portable() {
        assert!(SnapshotEndianness::Little.is_explicit());
        assert!(SnapshotEndianness::Big.is_explicit());
        assert!(!SnapshotEndianness::Native.is_explicit());
    }

    #[test]
    fn strict_restore_policy_forbids_representation_conversion() {
        let policy = SnapshotRestorePolicy::strict();

        assert!(!policy.permits_representation_conversion());
        assert!(policy.require_exact_payload);
    }
}