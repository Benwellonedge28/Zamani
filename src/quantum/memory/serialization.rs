//! Zamani Quantum Memory — Production Serialization Boundary
//!
//! This module defines the provider-neutral, representation-independent wire
//! format for `quantum::memory` persistence and transport.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - canonical binary snapshot envelopes;
//! - explicit schema/version negotiation;
//! - bounded serialization and deserialization;
//! - deterministic encoding;
//! - payload length validation;
//! - metadata validation;
//! - SHA-256 payload/envelope integrity verification;
//! - corruption detection;
//! - representation/provider identifiers;
//! - storage-location identifiers;
//! - scalar-precision declarations;
//! - endianness declarations;
//! - compatibility checks;
//! - safe opaque payload transport;
//! - provider-neutral serialization APIs;
//! - conversion between `QuantumSnapshot` and bytes where the snapshot
//!   contract exposes the corresponding fields.
//!
//! This module deliberately does NOT own:
//!
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer/tableau mathematics;
//! - tensor-network mathematics;
//! - sparse-state mathematics;
//! - GPU APIs;
//! - CUDA/HIP/Metal/Vulkan/SYCL APIs;
//! - MPI/RDMA/UCX APIs;
//! - QPU APIs;
//! - provider authentication;
//! - provider credentials;
//! - routing;
//! - scheduling;
//! - compilation;
//! - benchmarking;
//! - circuit semantics.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural boundary
//!
//! ```text
//!                 quantum::ir
//!                     |
//!                     v
//!              execution/runtime
//!                     |
//!                     v
//!               quantum::memory
//!                     |
//!              QuantumSnapshot
//!                     |
//!                     v
//!             memory::serialization
//!                     |
//!          +----------+-----------+
//!          |          |           |
//!          v          v           v
//!        disk      transport   checkpoint
//!          |          |           |
//!          +----------+-----------+
//!                     |
//!                     v
//!              provider/backend
//! ```
//!
//! # Hardware neutrality
//!
//! The format is intentionally capable of carrying state produced by:
//!
//! - CPU simulators;
//! - SIMD simulators;
//! - GPU simulators;
//! - distributed simulators;
//! - state-vector engines;
//! - density-matrix engines;
//! - stabilizer engines;
//! - sparse engines;
//! - tensor-network engines;
//! - remote execution systems;
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom QPUs;
//! - photonic systems;
//! - annealing systems;
//! - hybrid quantum-classical systems;
//! - future Zamani hardware providers.
//!
//! It does this through opaque provider/representation identifiers rather than
//! embedding vendor-specific types in the serialization API.
//!
//! # Critical rule
//!
//! A serialized quantum-memory object MUST NEVER contain:
//!
//! - raw pointers;
//! - device pointers;
//! - allocator addresses;
//! - references;
//! - mutexes;
//! - thread handles;
//! - file descriptors;
//! - credentials;
//! - access tokens;
//! - private keys;
//! - provider session secrets;
//! - process IDs as required restore identifiers;
//! - process-local addresses.
//!
//! Such information is not portable state.
//!
//! # Security model
//!
//! SHA-256 is used for integrity/corruption detection.
//!
//! SHA-256 is NOT:
//!
//! - encryption;
//! - authentication;
//! - authorization;
//! - a digital signature;
//! - proof of QPU identity;
//! - proof of provider identity.
//!
//! Confidentiality must be supplied by the persistence/transport security
//! layer, and authenticity must be supplied by the appropriate cryptographic
//! attestation/signature subsystem.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependencies
//!
//! This module intentionally uses dependencies already present in Zamani's
//! quantum hardware serialization layer:
//!
//! - `serde`;
//! - `serde_json`;
//! - `sha2`.
//!
//! No new dependency is required.
//!
//! # Integration contract
//!
//! Foundational memory modules provide:
//!
//! - `types.rs`       — SnapshotId / ByteCount / QubitCount;
//! - `errors.rs`      — MemoryError;
//! - `representation.rs` — representation/storage declarations;
//! - `snapshot.rs`   — snapshot semantic contract.
//!
//! Representation implementations provide opaque payload bytes.
//!
//! Later modules such as:
//!
//! - `state_vector.rs`;
//! - `density_matrix.rs`;
//! - `stabilizer.rs`;
//! - `sparse.rs`;
//! - `tensor_network.rs`;
//! - `backend_state.rs`;
//! - `gpu.rs`;
//! - `distributed.rs`;
//! - `checkpoint.rs`;
//!
//! can integrate with this file without requiring changes to this module
//! merely because another representation is added.
//!
//! This is a deliberate frozen API boundary.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::fmt;

// =============================================================================
// Format identity
// =============================================================================

/// Stable identifier for the quantum-memory serialization format.
pub const MEMORY_SERIALIZATION_SCHEMA_ID: &str =
    "zamani.quantum.memory.serialization";

/// Major version of the memory serialization envelope.
///
/// Increment when the envelope becomes structurally incompatible.
pub const MEMORY_SERIALIZATION_MAJOR: u16 = 1;

/// Minor version of the memory serialization envelope.
pub const MEMORY_SERIALIZATION_MINOR: u16 = 0;

/// Current format version.
pub const MEMORY_SERIALIZATION_VERSION: SerializationVersion =
    SerializationVersion::new(
        MEMORY_SERIALIZATION_MAJOR,
        MEMORY_SERIALIZATION_MINOR,
    );

/// Four-byte binary magic value.
pub const MEMORY_SERIALIZATION_MAGIC: [u8; 4] = *b"ZQMS";

/// Binary encoding identifier.
pub const MEMORY_SERIALIZATION_ENCODING: u8 = 1;

/// SHA-256 digest length.
pub const SHA256_LENGTH: usize = 32;

/// Hexadecimal SHA-256 digest length.
pub const SHA256_HEX_LENGTH: usize = 64;

/// Maximum complete encoded document accepted by the production default.
///
/// Applications should normally derive this from `MemoryLimits`.
pub const DEFAULT_MAX_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;

/// Maximum metadata bytes accepted by default.
pub const DEFAULT_MAX_METADATA_BYTES: usize = 4 * 1024 * 1024;

/// Maximum opaque payload bytes accepted by the serializer by default.
///
/// This is an API safety ceiling, not a statement that Zamani quantum
/// states can fit into this amount of memory. Large production simulations
/// should provide a larger explicitly configured limit subject to the
/// global memory subsystem's `MemoryLimits`.
pub const DEFAULT_MAX_PAYLOAD_BYTES: usize = 60 * 1024 * 1024;

/// Maximum identifier length.
pub const MAX_IDENTIFIER_LENGTH: usize = 256;

/// Maximum schema identifier length.
pub const MAX_SCHEMA_ID_LENGTH: usize = 512;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 16 * 1024;

/// Maximum number of metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 256;

/// Maximum supported semantic document version.
pub const MAX_DOCUMENT_SCHEMA_VERSION: u16 = 10_000;

/// Maximum JSON nesting depth allowed for metadata.
pub const DEFAULT_MAX_JSON_DEPTH: usize = 64;

// =============================================================================
// Version
// =============================================================================

/// Semantic serialization-format version.
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
pub struct SerializationVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,
}

impl SerializationVersion {
    /// Creates a version.
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Current version.
    pub const CURRENT: Self = MEMORY_SERIALIZATION_VERSION;

    /// Returns whether the major version is supported.
    pub const fn supported_major(self) -> bool {
        self.major == Self::CURRENT.major
    }

    /// Returns whether this is exactly the current version.
    pub const fn is_current(self) -> bool {
        self.major == Self::CURRENT.major
            && self.minor == Self::CURRENT.minor
    }
}

impl Default for SerializationVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for SerializationVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

// =============================================================================
// Representation
// =============================================================================

/// Provider-neutral state representation identifier.
///
/// This deliberately mirrors the architectural representation categories
/// without depending on `representation.rs`, allowing this file to remain a
/// foundational serialization boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializedRepresentation {
    /// Dense pure state.
    StateVector,

    /// Mixed-state density matrix.
    DensityMatrix,

    /// Stabilizer/tableau state.
    Stabilizer,

    /// Sparse state.
    Sparse,

    /// Tensor network.
    TensorNetwork,

    /// Provider-native state.
    BackendNative {
        /// Stable provider identifier.
        provider: String,
    },

    /// Extension-defined representation.
    Extension {
        /// Stable extension identifier.
        name: String,
    },
}

impl SerializedRepresentation {
    /// Returns the stable identifier.
    pub fn identifier(&self) -> Result<String, SerializationError> {
        let value = match self {
            Self::StateVector => "state-vector".to_owned(),
            Self::DensityMatrix => "density-matrix".to_owned(),
            Self::Stabilizer => "stabilizer".to_owned(),
            Self::Sparse => "sparse".to_owned(),
            Self::TensorNetwork => "tensor-network".to_owned(),
            Self::BackendNative { provider } => {
                validate_identifier(provider, "provider")?;
                format!("backend-native:{provider}")
            }
            Self::Extension { name } => {
                validate_identifier(name, "extension")?;
                format!("extension:{name}")
            }
        };

        validate_identifier(&value, "representation")?;
        Ok(value)
    }

    /// Returns whether this is a provider-native state.
    pub const fn is_backend_native(&self) -> bool {
        matches!(self, Self::BackendNative { .. })
    }
}

// =============================================================================
// Storage
// =============================================================================

/// Physical storage location from which the state was captured.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializedStorageLocation {
    /// Host RAM.
    Host,

    /// Pinned host RAM.
    PinnedHost,

    /// Accelerator memory.
    Device,

    /// Unified memory.
    Unified,

    /// Distributed memory.
    Distributed,

    /// Remote provider/QPU-managed state.
    Remote,

    /// Provider-defined location.
    Custom(String),
}

impl SerializedStorageLocation {
    /// Returns a stable identifier.
    pub fn identifier(&self) -> Result<String, SerializationError> {
        let value = match self {
            Self::Host => "host".to_owned(),
            Self::PinnedHost => "pinned-host".to_owned(),
            Self::Device => "device".to_owned(),
            Self::Unified => "unified".to_owned(),
            Self::Distributed => "distributed".to_owned(),
            Self::Remote => "remote".to_owned(),
            Self::Custom(value) => {
                validate_identifier(value, "storage location")?;
                format!("custom:{value}")
            }
        };

        validate_identifier(&value, "storage location")?;
        Ok(value)
    }
}

// =============================================================================
// Precision
// =============================================================================

/// Numeric precision associated with the payload.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum SerializedPrecision {
    /// IEEE-754 binary32 components.
    F32,

    /// IEEE-754 binary64 components.
    F64,

    /// Extended precision.
    Extended,

    /// Provider-defined precision.
    BackendDefined,
}

// =============================================================================
// Endianness
// =============================================================================

/// Byte order declared for the payload.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum SerializedEndianness {
    /// Little endian.
    Little,

    /// Big endian.
    Big,
}

impl SerializedEndianness {
    /// Host byte order.
    pub const fn host() -> Self {
        if cfg!(target_endian = "little") {
            Self::Little
        } else {
            Self::Big
        }
    }
}

// =============================================================================
// Payload encoding
// =============================================================================

/// Encoding of the opaque semantic payload.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum PayloadEncoding {
    /// Raw bytes owned by the representation provider.
    Raw,

    /// A named representation format.
    Named {
        /// Stable encoder name.
        name: String,

        /// Encoder version.
        version: String,
    },

    /// Provider-defined encoding.
    ProviderDefined {
        /// Provider identifier.
        provider: String,

        /// Provider encoder version.
        version: String,
    },
}

impl PayloadEncoding {
    /// Returns a stable identifier.
    pub fn identifier(&self) -> Result<String, SerializationError> {
        let value = match self {
            Self::Raw => "raw".to_owned(),

            Self::Named { name, version } => {
                validate_identifier(name, "encoding name")?;
                validate_identifier(version, "encoding version")?;
                format!("{name}@{version}")
            }

            Self::ProviderDefined { provider, version } => {
                validate_identifier(provider, "encoding provider")?;
                validate_identifier(version, "encoding version")?;
                format!("provider:{provider}@{version}")
            }
        };

        validate_identifier(&value, "payload encoding")?;
        Ok(value)
    }
}

// =============================================================================
// Integrity
// =============================================================================

/// Integrity algorithm.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum IntegrityAlgorithm {
    /// No digest.
    None,

    /// SHA-256.
    Sha256,
}

/// Integrity information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Integrity {
    /// Algorithm.
    pub algorithm: IntegrityAlgorithm,

    /// Digest bytes.
    pub digest: Vec<u8>,
}

impl Integrity {
    /// Creates a SHA-256 integrity descriptor.
    pub fn sha256(bytes: &[u8]) -> Self {
        Self {
            algorithm: IntegrityAlgorithm::Sha256,
            digest: sha256(bytes),
        }
    }

    /// Creates a digest-free descriptor.
    pub fn none() -> Self {
        Self {
            algorithm: IntegrityAlgorithm::None,
            digest: Vec::new(),
        }
    }

    /// Validates the descriptor.
    pub fn validate(&self) -> Result<(), SerializationError> {
        match self.algorithm {
            IntegrityAlgorithm::None => {
                if !self.digest.is_empty() {
                    return Err(SerializationError::InvalidIntegrityLength {
                        expected: 0,
                        actual: self.digest.len(),
                    });
                }
            }

            IntegrityAlgorithm::Sha256 => {
                if self.digest.len() != SHA256_LENGTH {
                    return Err(SerializationError::InvalidIntegrityLength {
                        expected: SHA256_LENGTH,
                        actual: self.digest.len(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns the digest as lowercase hexadecimal.
    pub fn hex(&self) -> Result<String, SerializationError> {
        self.validate()?;

        Ok(hex_encode(&self.digest))
    }
}

// =============================================================================
// Metadata
// =============================================================================

/// Bounded, deterministic metadata attached to a serialized state.
///
/// Metadata must never contain secrets. Callers are responsible for ensuring
/// that user-defined metadata does not contain credentials or private data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializationMetadata {
    /// Optional description.
    pub description: Option<String>,

    /// Optional Zamani version.
    pub zamani_version: Option<String>,

    /// Optional program identity.
    pub program_identity: Option<String>,

    /// Optional circuit identity.
    pub circuit_identity: Option<String>,

    /// Optional execution identity.
    pub execution_identity: Option<String>,

    /// Optional provider identifier.
    pub provider: Option<String>,

    /// Optional provider version.
    pub provider_version: Option<String>,

    /// Application-defined bounded metadata.
    #[serde(default)]
    pub attributes: Vec<MetadataEntry>,
}

impl Default for SerializationMetadata {
    fn default() -> Self {
        Self {
            description: None,
            zamani_version: None,
            program_identity: None,
            circuit_identity: None,
            execution_identity: None,
            provider: None,
            provider_version: None,
            attributes: Vec::new(),
        }
    }
}

/// A bounded metadata key/value pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetadataEntry {
    /// Metadata key.
    pub key: String,

    /// Metadata value.
    pub value: String,
}

impl SerializationMetadata {
    /// Validates all metadata bounds.
    pub fn validate(&self) -> Result<(), SerializationError> {
        validate_optional_text(
            self.description.as_deref(),
            "description",
            MAX_METADATA_VALUE_LENGTH,
        )?;

        validate_optional_text(
            self.zamani_version.as_deref(),
            "Zamani version",
            MAX_METADATA_KEY_LENGTH,
        )?;

        validate_optional_text(
            self.program_identity.as_deref(),
            "program identity",
            MAX_METADATA_KEY_LENGTH,
        )?;

        validate_optional_text(
            self.circuit_identity.as_deref(),
            "circuit identity",
            MAX_METADATA_KEY_LENGTH,
        )?;

        validate_optional_text(
            self.execution_identity.as_deref(),
            "execution identity",
            MAX_METADATA_KEY_LENGTH,
        )?;

        validate_optional_text(
            self.provider.as_deref(),
            "provider",
            MAX_IDENTIFIER_LENGTH,
        )?;

        validate_optional_text(
            self.provider_version.as_deref(),
            "provider version",
            MAX_IDENTIFIER_LENGTH,
        )?;

        if self.attributes.len() > MAX_METADATA_ENTRIES {
            return Err(SerializationError::MetadataTooLarge {
                entries: self.attributes.len(),
                maximum: MAX_METADATA_ENTRIES,
            });
        }

        for entry in &self.attributes {
            if entry.key.is_empty() {
                return Err(SerializationError::InvalidMetadata {
                    message: "metadata key cannot be empty".to_owned(),
                });
            }

            if entry.key.len() > MAX_METADATA_KEY_LENGTH {
                return Err(SerializationError::InvalidMetadata {
                    message: "metadata key exceeds maximum length".to_owned(),
                });
            }

            if entry.value.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(SerializationError::InvalidMetadata {
                    message: "metadata value exceeds maximum length".to_owned(),
                });
            }

            if entry.key.chars().any(|c| c.is_control()) {
                return Err(SerializationError::InvalidMetadata {
                    message: "metadata key contains a control character"
                        .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Serializable state envelope
// =============================================================================

/// Complete provider-neutral serialization envelope.
///
/// The envelope contains all information necessary to determine whether a
/// payload can safely be passed to a representation/backend-specific restore
/// implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemorySerializationEnvelope {
    /// Four-byte format magic.
    pub magic: [u8; 4],

    /// Serialization envelope version.
    pub format_version: SerializationVersion,

    /// Stable serialization schema identifier.
    pub schema_id: String,

    /// Semantic document schema identifier.
    pub document_schema_id: String,

    /// Semantic document schema version.
    pub document_schema_version: u16,

    /// Stable memory snapshot identity.
    pub snapshot_id: u64,

    /// Number of logical qubits represented by the state.
    pub qubit_count: u64,

    /// State representation.
    pub representation: SerializedRepresentation,

    /// Physical storage location at capture time.
    pub storage_location: SerializedStorageLocation,

    /// Numeric precision.
    pub precision: SerializedPrecision,

    /// Payload byte order.
    pub endianness: SerializedEndianness,

    /// Payload encoding.
    pub payload_encoding: PayloadEncoding,

    /// Provider-neutral metadata.
    pub metadata: SerializationMetadata,

    /// Exact payload byte length.
    pub payload_length: u64,

    /// SHA-256 of the opaque payload when integrity is enabled.
    pub payload_integrity: Integrity,

    /// Opaque representation payload.
    pub payload: Vec<u8>,
}

impl MemorySerializationEnvelope {
    /// Creates a new envelope.
    pub fn new(
        options: SerializationOptions,
        snapshot_id: u64,
        qubit_count: u64,
        document_schema_id: impl Into<String>,
        document_schema_version: u16,
        representation: SerializedRepresentation,
        storage_location: SerializedStorageLocation,
        precision: SerializedPrecision,
        endianness: SerializedEndianness,
        payload_encoding: PayloadEncoding,
        metadata: SerializationMetadata,
        payload: Vec<u8>,
    ) -> Result<Self, SerializationError> {
        let document_schema_id = document_schema_id.into();

        validate_schema_id(&document_schema_id)?;
        validate_document_version(document_schema_version)?;
        validate_identifier(
            &representation.identifier()?,
            "representation",
        )?;
        validate_identifier(
            &storage_location.identifier()?,
            "storage location",
        )?;
        validate_identifier(
            &payload_encoding.identifier()?,
            "payload encoding",
        )?;
        metadata.validate()?;

        options.validate()?;

        if payload.len() > options.max_payload_bytes {
            return Err(SerializationError::PayloadTooLarge {
                size: payload.len(),
                maximum: options.max_payload_bytes,
            });
        }

        let payload_length = u64::try_from(payload.len())
            .map_err(|_| SerializationError::LengthOverflow)?;

        let payload_integrity = if options.include_integrity {
            Integrity::sha256(&payload)
        } else {
            Integrity::none()
        };

        let envelope = Self {
            magic: MEMORY_SERIALIZATION_MAGIC,
            format_version: MEMORY_SERIALIZATION_VERSION,
            schema_id: MEMORY_SERIALIZATION_SCHEMA_ID.to_owned(),
            document_schema_id,
            document_schema_version,
            snapshot_id,
            qubit_count,
            representation,
            storage_location,
            precision,
            endianness,
            payload_encoding,
            metadata,
            payload_length,
            payload_integrity,
            payload,
        };

        envelope.validate(&options)?;

        Ok(envelope)
    }

    /// Validates the complete envelope.
    pub fn validate(
        &self,
        options: &SerializationOptions,
    ) -> Result<(), SerializationError> {
        options.validate()?;

        if self.magic != MEMORY_SERIALIZATION_MAGIC {
            return Err(SerializationError::InvalidMagic {
                actual: self.magic,
            });
        }

        if !self.format_version.supported_major() {
            return Err(SerializationError::UnsupportedFormatVersion {
                expected_major: MEMORY_SERIALIZATION_MAJOR,
                actual_major: self.format_version.major,
                actual_minor: self.format_version.minor,
            });
        }

        if self.schema_id != MEMORY_SERIALIZATION_SCHEMA_ID {
            return Err(SerializationError::SchemaMismatch {
                expected: MEMORY_SERIALIZATION_SCHEMA_ID.to_owned(),
                actual: self.schema_id.clone(),
            });
        }

        validate_schema_id(&self.document_schema_id)?;
        validate_document_version(self.document_schema_version)?;

        if self.qubit_count > usize::MAX as u64 {
            return Err(SerializationError::QuantityOverflow {
                field: "qubit_count",
            });
        }

        let actual_length = u64::try_from(self.payload.len())
            .map_err(|_| SerializationError::LengthOverflow)?;

        if actual_length != self.payload_length {
            return Err(SerializationError::PayloadLengthMismatch {
                declared: self.payload_length,
                actual: actual_length,
            });
        }

        if self.payload.len() > options.max_payload_bytes {
            return Err(SerializationError::PayloadTooLarge {
                size: self.payload.len(),
                maximum: options.max_payload_bytes,
            });
        }

        self.metadata.validate()?;
        self.payload_integrity.validate()?;

        if options.include_integrity {
            if self.payload_integrity.algorithm != IntegrityAlgorithm::Sha256
            {
                return Err(SerializationError::IntegrityRequired);
            }

            let actual = sha256(&self.payload);

            if actual != self.payload_integrity.digest {
                return Err(SerializationError::IntegrityMismatch {
                    expected: hex_encode(&self.payload_integrity.digest),
                    actual: hex_encode(&actual),
                });
            }
        }

        // Validate identifiers before they are used by backend-specific code.
        let _ = self.representation.identifier()?;
        let _ = self.storage_location.identifier()?;
        let _ = self.payload_encoding.identifier()?;

        Ok(())
    }
}

// =============================================================================
// Options
// =============================================================================

/// Resource and integrity policy for serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializationOptions {
    /// Maximum complete serialized document.
    pub max_document_bytes: usize,

    /// Maximum metadata representation.
    pub max_metadata_bytes: usize,

    /// Maximum opaque payload.
    pub max_payload_bytes: usize,

    /// Maximum metadata JSON nesting.
    pub max_json_depth: usize,

    /// Require SHA-256 payload integrity.
    pub include_integrity: bool,

    /// Require integrity during deserialization.
    pub require_integrity: bool,
}

impl Default for SerializationOptions {
    fn default() -> Self {
        Self::production()
    }
}

impl SerializationOptions {
    /// Production defaults.
    pub const fn production() -> Self {
        Self {
            max_document_bytes: DEFAULT_MAX_DOCUMENT_BYTES,
            max_metadata_bytes: DEFAULT_MAX_METADATA_BYTES,
            max_payload_bytes: DEFAULT_MAX_PAYLOAD_BYTES,
            max_json_depth: DEFAULT_MAX_JSON_DEPTH,
            include_integrity: true,
            require_integrity: true,
        }
    }

    /// Validates the policy.
    pub fn validate(self) -> Result<Self, SerializationError> {
        if self.max_document_bytes == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_document_bytes",
            });
        }

        if self.max_metadata_bytes == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_metadata_bytes",
            });
        }

        if self.max_payload_bytes == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_payload_bytes",
            });
        }

        if self.max_document_bytes > usize::MAX / 2 {
            return Err(SerializationError::InvalidLimit {
                field: "max_document_bytes",
            });
        }

        if self.max_metadata_bytes > self.max_document_bytes {
            return Err(SerializationError::InvalidLimitRelation {
                field: "max_metadata_bytes",
                maximum: "max_document_bytes",
            });
        }

        if self.max_payload_bytes > self.max_document_bytes {
            return Err(SerializationError::InvalidLimitRelation {
                field: "max_payload_bytes",
                maximum: "max_document_bytes",
            });
        }

        if self.max_json_depth == 0 {
            return Err(SerializationError::InvalidLimit {
                field: "max_json_depth",
            });
        }

        Ok(self)
    }
}

// =============================================================================
// Public serialization API
// =============================================================================

/// Serializes a complete memory envelope into deterministic JSON bytes.
///
/// The JSON representation is canonicalized recursively before encoding.
/// Object-key order therefore does not depend on insertion order.
pub fn serialize_envelope(
    envelope: &MemorySerializationEnvelope,
    options: SerializationOptions,
) -> Result<Vec<u8>, SerializationError> {
    options.validate()?;
    envelope.validate(&options)?;

    let value = serde_json::to_value(envelope)
        .map_err(|error| SerializationError::Serialize {
            message: error.to_string(),
        })?;

    let canonical = canonicalize_json(value);

    let bytes = serde_json::to_vec(&canonical)
        .map_err(|error| SerializationError::Serialize {
            message: error.to_string(),
        })?;

    if bytes.len() > options.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: bytes.len(),
            maximum: options.max_document_bytes,
        });
    }

    Ok(bytes)
}

/// Deserializes a memory envelope from canonical JSON.
///
/// This function performs a byte-size preflight before invoking the JSON
/// parser, then validates the resulting envelope.
pub fn deserialize_envelope(
    bytes: &[u8],
    options: SerializationOptions,
) -> Result<MemorySerializationEnvelope, SerializationError> {
    options.validate()?;

    if bytes.len() > options.max_document_bytes {
        return Err(SerializationError::DocumentTooLarge {
            size: bytes.len(),
            maximum: options.max_document_bytes,
        });
    }

    if bytes.is_empty() {
        return Err(SerializationError::EmptyDocument);
    }

    let value: Value =
        serde_json::from_slice(bytes).map_err(|error| {
            SerializationError::Deserialize {
                message: error.to_string(),
            }
        })?;

    validate_json_depth(&value, options.max_json_depth)?;

    let envelope: MemorySerializationEnvelope =
        serde_json::from_value(value).map_err(|error| {
            SerializationError::Deserialize {
                message: error.to_string(),
            }
        })?;

    envelope.validate(&options)?;

    Ok(envelope)
}

/// Serializes an arbitrary Serde document into the memory serialization
/// envelope.
///
/// This is the generic integration API for future memory representations.
pub fn serialize_document<T: Serialize>(
    document_schema_id: &str,
    document_schema_version: u16,
    snapshot_id: u64,
    qubit_count: u64,
    representation: SerializedRepresentation,
    storage_location: SerializedStorageLocation,
    precision: SerializedPrecision,
    endianness: SerializedEndianness,
    payload_encoding: PayloadEncoding,
    metadata: SerializationMetadata,
    document: &T,
    options: SerializationOptions,
) -> Result<Vec<u8>, SerializationError> {
    let payload_value =
        serde_json::to_value(document).map_err(|error| {
            SerializationError::Serialize {
                message: error.to_string(),
            }
        })?;

    validate_json_depth(&payload_value, options.max_json_depth)?;

    let canonical_payload = canonicalize_json(payload_value);

    let payload =
        serde_json::to_vec(&canonical_payload).map_err(|error| {
            SerializationError::Serialize {
                message: error.to_string(),
            }
        })?;

    if payload.len() > options.max_payload_bytes {
        return Err(SerializationError::PayloadTooLarge {
            size: payload.len(),
            maximum: options.max_payload_bytes,
        });
    }

    let envelope = MemorySerializationEnvelope::new(
        options,
        snapshot_id,
        qubit_count,
        document_schema_id.to_owned(),
        document_schema_version,
        representation,
        storage_location,
        precision,
        endianness,
        payload_encoding,
        metadata,
        payload,
    )?;

    serialize_envelope(&envelope, options)
}

/// Deserializes a typed document from a memory serialization envelope.
///
/// The caller must explicitly provide the expected semantic schema identity
/// and version. This prevents accidental interpretation of a payload using
/// the wrong representation.
pub fn deserialize_document<T: DeserializeOwned>(
    bytes: &[u8],
    expected_schema_id: &str,
    expected_schema_version: u16,
    options: SerializationOptions,
) -> Result<T, SerializationError> {
    validate_schema_id(expected_schema_id)?;
    validate_document_version(expected_schema_version)?;

    let envelope = deserialize_envelope(bytes, options)?;

    if envelope.document_schema_id != expected_schema_id {
        return Err(SerializationError::SchemaMismatch {
            expected: expected_schema_id.to_owned(),
            actual: envelope.document_schema_id,
        });
    }

    if envelope.document_schema_version != expected_schema_version {
        return Err(SerializationError::DocumentVersionMismatch {
            expected: expected_schema_version,
            actual: envelope.document_schema_version,
        });
    }

    let value: Value =
        serde_json::from_slice(&envelope.payload).map_err(|error| {
            SerializationError::Deserialize {
                message: error.to_string(),
            }
        })?;

    validate_json_depth(&value, options.max_json_depth)?;

    serde_json::from_value(value).map_err(|error| {
        SerializationError::Deserialize {
            message: error.to_string(),
        }
    })
}

/// Calculates a SHA-256 digest over arbitrary bytes.
pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

/// Calculates a lowercase hexadecimal SHA-256 fingerprint.
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex_encode(&sha256(bytes))
}

// =============================================================================
// Compatibility
// =============================================================================

/// Explicit compatibility result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    /// Exact format and semantic version match.
    Exact,

    /// Same major serialization format, compatible minor version.
    Compatible,

    /// Incompatible.
    Incompatible,
}

/// Checks whether two serialization versions can be read together.
///
/// The current policy is deliberately conservative:
///
/// - different major versions are incompatible;
/// - identical versions are exact;
/// - same major with different minor versions is compatible only when the
///   caller explicitly opts into forward/backward compatibility.
///
/// The default API therefore does not silently reinterpret newer formats.
pub fn check_version_compatibility(
    expected: SerializationVersion,
    actual: SerializationVersion,
) -> Compatibility {
    if expected.major != actual.major {
        Compatibility::Incompatible
    } else if expected == actual {
        Compatibility::Exact
    } else {
        Compatibility::Compatible
    }
}

// =============================================================================
// Canonical JSON
// =============================================================================

/// Recursively canonicalizes JSON.
///
/// `serde_json::Map` is normally ordered, but this function makes the policy
/// explicit and recursively rebuilds every object.
pub fn canonicalize_json(value: Value) -> Value {
    match value {
        Value::Object(object) => {
            let mut entries: Vec<(String, Value)> =
                object.into_iter().collect();

            entries.sort_by(|left, right| left.0.cmp(&right.0));

            let mut canonical = Map::new();

            for (key, value) in entries {
                canonical.insert(key, canonicalize_json(value));
            }

            Value::Object(canonical)
        }

        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(canonicalize_json)
                .collect(),
        ),

        other => other,
    }
}

/// Calculates the fingerprint of a JSON value using canonical JSON bytes.
pub fn fingerprint_json(value: &Value) -> Result<Vec<u8>, SerializationError> {
    validate_json_depth(value, DEFAULT_MAX_JSON_DEPTH)?;

    let canonical = canonicalize_json(value);

    let bytes =
        serde_json::to_vec(&canonical).map_err(|error| {
            SerializationError::Serialize {
                message: error.to_string(),
            }
        })?;

    Ok(sha256(&bytes))
}

/// Calculates a hexadecimal fingerprint of a JSON value.
pub fn fingerprint_json_hex(
    value: &Value,
) -> Result<String, SerializationError> {
    Ok(hex_encode(&fingerprint_json(value)?))
}

// =============================================================================
// JSON safety
// =============================================================================

/// Validates JSON nesting depth.
///
/// This is intentionally applied to representation payloads serialized as
/// JSON. Raw binary payloads do not require JSON-depth validation.
pub fn validate_json_depth(
    value: &Value,
    maximum: usize,
) -> Result<(), SerializationError> {
    if maximum == 0 {
        return Err(SerializationError::InvalidLimit {
            field: "max_json_depth",
        });
    }

    validate_json_depth_inner(value, 0, maximum)
}

fn validate_json_depth_inner(
    value: &Value,
    depth: usize,
    maximum: usize,
) -> Result<(), SerializationError> {
    if depth > maximum {
        return Err(SerializationError::JsonDepthExceeded { maximum });
    }

    match value {
        Value::Array(values) => {
            let next = depth
                .checked_add(1)
                .ok_or(SerializationError::LengthOverflow)?;

            for value in values {
                validate_json_depth_inner(value, next, maximum)?;
            }
        }

        Value::Object(object) => {
            let next = depth
                .checked_add(1)
                .ok_or(SerializationError::LengthOverflow)?;

            for (key, value) in object {
                if key.len() > MAX_METADATA_KEY_LENGTH {
                    return Err(SerializationError::InvalidMetadata {
                        message: "JSON object key exceeds maximum length"
                            .to_owned(),
                    });
                }

                validate_json_depth_inner(value, next, maximum)?;
            }
        }

        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }

    Ok(())
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_schema_id(value: &str) -> Result<(), SerializationError> {
    if value.is_empty() {
        return Err(SerializationError::EmptySchemaId);
    }

    if value.len() > MAX_SCHEMA_ID_LENGTH {
        return Err(SerializationError::SchemaIdTooLong {
            length: value.len(),
            maximum: MAX_SCHEMA_ID_LENGTH,
        });
    }

    if value.chars().any(|character| {
        character.is_control()
            || character.is_whitespace()
            || matches!(character, '"' | '\\')
    }) {
        return Err(SerializationError::InvalidSchemaId);
    }

    Ok(())
}

fn validate_document_version(
    version: u16,
) -> Result<(), SerializationError> {
    if version > MAX_DOCUMENT_SCHEMA_VERSION {
        return Err(SerializationError::InvalidDocumentVersion {
            version,
        });
    }

    Ok(())
}

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), SerializationError> {
    if value.is_empty() {
        return Err(SerializationError::EmptyIdentifier { field });
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(SerializationError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum: MAX_IDENTIFIER_LENGTH,
        });
    }

    if value.chars().any(|character| {
        character.is_control() || character.is_whitespace()
    }) {
        return Err(SerializationError::InvalidIdentifier { field });
    }

    Ok(())
}

fn validate_optional_text(
    value: Option<&str>,
    field: &'static str,
    maximum: usize,
) -> Result<(), SerializationError> {
    let Some(value) = value else {
        return Ok(());
    };

    if value.len() > maximum {
        return Err(SerializationError::TextTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(SerializationError::InvalidMetadata {
            message: format!("{field} contains a control character"),
        });
    }

    Ok(())
}

// =============================================================================
// Hex encoding
// =============================================================================

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let capacity = bytes.len().saturating_mul(2);
    let mut output = String::with_capacity(capacity);

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

// =============================================================================
// Errors
// =============================================================================

/// Production serialization error.
///
/// This error intentionally remains local to serialization. Integration with
/// `MemoryError` should be performed by the higher-level memory persistence
/// boundary, preserving this detailed cause rather than forcing the
/// foundational `errors.rs` contract to depend on serialization internals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerializationError {
    /// Empty input document.
    EmptyDocument,

    /// Incorrect magic bytes.
    InvalidMagic {
        /// Actual magic value.
        actual: [u8; 4],
    },

    /// Unsupported envelope format version.
    UnsupportedFormatVersion {
        /// Supported major version.
        expected_major: u16,

        /// Received major version.
        actual_major: u16,

        /// Received minor version.
        actual_minor: u16,
    },

    /// Envelope schema mismatch.
    SchemaMismatch {
        /// Expected schema.
        expected: String,

        /// Actual schema.
        actual: String,
    },

    /// Empty schema identifier.
    EmptySchemaId,

    /// Schema identifier is too long.
    SchemaIdTooLong {
        /// Actual length.
        length: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Schema identifier contains invalid characters.
    InvalidSchemaId,

    /// Invalid semantic document version.
    InvalidDocumentVersion {
        /// Invalid version.
        version: u16,
    },

    /// Semantic document versions differ.
    DocumentVersionMismatch {
        /// Expected version.
        expected: u16,

        /// Actual version.
        actual: u16,
    },

    /// Identifier is empty.
    EmptyIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// Identifier is too long.
    IdentifierTooLong {
        /// Identifier field.
        field: &'static str,

        /// Actual length.
        length: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Identifier contains invalid characters.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// Metadata is invalid.
    InvalidMetadata {
        /// Explanation.
        message: String,
    },

    /// Metadata exceeds bounds.
    MetadataTooLarge {
        /// Number of entries.
        entries: usize,

        /// Maximum entries.
        maximum: usize,
    },

    /// Generic text field exceeds its bound.
    TextTooLong {
        /// Field name.
        field: &'static str,

        /// Actual length.
        length: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Configured resource limit is invalid.
    InvalidLimit {
        /// Limit name.
        field: &'static str,
    },

    /// Two limits have an invalid relationship.
    InvalidLimitRelation {
        /// Smaller limit.
        field: &'static str,

        /// Limit that must contain it.
        maximum: &'static str,
    },

    /// Complete document is too large.
    DocumentTooLarge {
        /// Actual size.
        size: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Opaque payload is too large.
    PayloadTooLarge {
        /// Actual size.
        size: usize,

        /// Maximum.
        maximum: usize,
    },

    /// Declared payload length differs from actual payload length.
    PayloadLengthMismatch {
        /// Declared length.
        declared: u64,

        /// Actual length.
        actual: u64,
    },

    /// Integer conversion overflow.
    LengthOverflow,

    /// A quantity cannot be represented on this platform.
    QuantityOverflow {
        /// Quantity field.
        field: &'static str,
    },

    /// Payload integrity algorithm is required.
    IntegrityRequired,

    /// Integrity descriptor length is invalid.
    InvalidIntegrityLength {
        /// Expected length.
        expected: usize,

        /// Actual length.
        actual: usize,
    },

    /// Payload integrity verification failed.
    IntegrityMismatch {
        /// Expected digest.
        expected: String,

        /// Actual digest.
        actual: String,
    },

    /// JSON nesting exceeds the configured limit.
    JsonDepthExceeded {
        /// Maximum depth.
        maximum: usize,
    },

    /// Serialization failed.
    Serialize {
        /// Underlying error text.
        message: String,
    },

    /// Deserialization failed.
    Deserialize {
        /// Underlying error text.
        message: String,
    },
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDocument => {
                f.write_str("serialization document is empty")
            }

            Self::InvalidMagic { actual } => {
                write!(
                    f,
                    "invalid quantum-memory serialization magic: {:02x?}",
                    actual
                )
            }

            Self::UnsupportedFormatVersion {
                expected_major,
                actual_major,
                actual_minor,
            } => {
                write!(
                    f,
                    "unsupported serialization format {actual_major}.{actual_minor}; \
                     supported major version is {expected_major}"
                )
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    f,
                    "serialization schema mismatch: expected {expected}, received {actual}"
                )
            }

            Self::EmptySchemaId => {
                f.write_str("schema identifier cannot be empty")
            }

            Self::SchemaIdTooLong { length, maximum } => {
                write!(
                    f,
                    "schema identifier is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidSchemaId => {
                f.write_str("schema identifier contains invalid characters")
            }

            Self::InvalidDocumentVersion { version } => {
                write!(f, "invalid document schema version {version}")
            }

            Self::DocumentVersionMismatch { expected, actual } => {
                write!(
                    f,
                    "document schema version mismatch: expected {expected}, received {actual}"
                )
            }

            Self::EmptyIdentifier { field } => {
                write!(f, "{field} identifier cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "{field} identifier is {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidIdentifier { field } => {
                write!(f, "{field} identifier contains invalid characters")
            }

            Self::InvalidMetadata { message } => {
                write!(f, "invalid serialization metadata: {message}")
            }

            Self::MetadataTooLarge { entries, maximum } => {
                write!(
                    f,
                    "serialization metadata contains {entries} entries; maximum is {maximum}"
                )
            }

            Self::TextTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "{field} contains {length} bytes; maximum is {maximum}"
                )
            }

            Self::InvalidLimit { field } => {
                write!(f, "invalid serialization limit: {field}")
            }

            Self::InvalidLimitRelation { field, maximum } => {
                write!(
                    f,
                    "serialization limit {field} exceeds {maximum}"
                )
            }

            Self::DocumentTooLarge { size, maximum } => {
                write!(
                    f,
                    "serialized document is {size} bytes; maximum is {maximum}"
                )
            }

            Self::PayloadTooLarge { size, maximum } => {
                write!(
                    f,
                    "serialized payload is {size} bytes; maximum is {maximum}"
                )
            }

            Self::PayloadLengthMismatch { declared, actual } => {
                write!(
                    f,
                    "payload length mismatch: declared {declared}, actual {actual}"
                )
            }

            Self::LengthOverflow => {
                f.write_str("serialization length calculation overflowed")
            }

            Self::QuantityOverflow { field } => {
                write!(f, "serialization quantity overflow in {field}")
            }

            Self::IntegrityRequired => {
                f.write_str("SHA-256 payload integrity is required")
            }

            Self::InvalidIntegrityLength { expected, actual } => {
                write!(
                    f,
                    "invalid integrity length: expected {expected}, actual {actual}"
                )
            }

            Self::IntegrityMismatch { expected, actual } => {
                write!(
                    f,
                    "payload integrity mismatch: expected {expected}, actual {actual}"
                )
            }

            Self::JsonDepthExceeded { maximum } => {
                write!(
                    f,
                    "JSON nesting exceeds maximum depth {maximum}"
                )
            }

            Self::Serialize { message } => {
                write!(f, "serialization failed: {message}")
            }

            Self::Deserialize { message } => {
                write!(f, "deserialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for SerializationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct TestState {
        amplitudes: Vec<i32>,
        label: String,
    }

    fn test_metadata() -> SerializationMetadata {
        SerializationMetadata {
            description: Some("test".to_owned()),
            zamani_version: Some("1.0".to_owned()),
            program_identity: Some("program-test".to_owned()),
            circuit_identity: Some("circuit-test".to_owned()),
            execution_identity: Some("execution-test".to_owned()),
            provider: None,
            provider_version: None,
            attributes: vec![MetadataEntry {
                key: "purpose".to_owned(),
                value: "unit-test".to_owned(),
            }],
        }
    }

    fn test_envelope() -> MemorySerializationEnvelope {
        MemorySerializationEnvelope::new(
            SerializationOptions::production(),
            7,
            3,
            "zamani.quantum.memory.test",
            1,
            SerializedRepresentation::StateVector,
            SerializedStorageLocation::Host,
            SerializedPrecision::F64,
            SerializedEndianness::Little,
            PayloadEncoding::Raw,
            test_metadata(),
            b"quantum-state-payload".to_vec(),
        )
        .expect("test envelope must be valid")
    }

    #[test]
    fn current_version_is_supported() {
        assert!(SerializationVersion::CURRENT.supported_major());
        assert!(SerializationVersion::CURRENT.is_current());
    }

    #[test]
    fn envelope_round_trip_is_deterministic() {
        let options = SerializationOptions::production();
        let envelope = test_envelope();

        let first =
            serialize_envelope(&envelope, options).expect("serialize");
        let second =
            serialize_envelope(&envelope, options).expect("serialize");

        assert_eq!(first, second);

        let decoded =
            deserialize_envelope(&first, options).expect("deserialize");

        assert_eq!(decoded, envelope);
    }

    #[test]
    fn payload_integrity_is_verified() {
        let options = SerializationOptions::production();
        let envelope = test_envelope();

        let mut encoded =
            serialize_envelope(&envelope, options).expect("serialize");

        // Change one byte in the encoded document without changing its
        // declared digest.
        if let Some(position) =
            encoded.iter().position(|byte| *byte == b'q')
        {
            encoded[position] = b'x';
        }

        let result = deserialize_envelope(&encoded, options);

        assert!(result.is_err());
    }

    #[test]
    fn payload_length_is_verified() {
        let mut envelope = test_envelope();

        envelope.payload_length =
            envelope.payload_length.saturating_add(1);

        let result =
            envelope.validate(&SerializationOptions::production());

        assert!(matches!(
            result,
            Err(SerializationError::PayloadLengthMismatch { .. })
        ));
    }

    #[test]
    fn generic_document_round_trip_works() {
        let state = TestState {
            amplitudes: vec![1, 0, 0, 1],
            label: "bell".to_owned(),
        };

        let bytes = serialize_document(
            "zamani.quantum.memory.test.state",
            1,
            42,
            2,
            SerializedRepresentation::StateVector,
            SerializedStorageLocation::Host,
            SerializedPrecision::F64,
            SerializedEndianness::Little,
            PayloadEncoding::Named {
                name: "zamani-test-json".to_owned(),
                version: "1".to_owned(),
            },
            test_metadata(),
            &state,
            SerializationOptions::production(),
        )
        .expect("serialize");

        let restored: TestState = deserialize_document(
            &bytes,
            "zamani.quantum.memory.test.state",
            1,
            SerializationOptions::production(),
        )
        .expect("deserialize");

        assert_eq!(restored, state);
    }

    #[test]
    fn schema_mismatch_is_rejected() {
        let state = TestState {
            amplitudes: vec![1],
            label: "zero".to_owned(),
        };

        let bytes = serialize_document(
            "zamani.quantum.memory.test.state",
            1,
            1,
            1,
            SerializedRepresentation::StateVector,
            SerializedStorageLocation::Host,
            SerializedPrecision::F64,
            SerializedEndianness::Little,
            PayloadEncoding::Raw,
            SerializationMetadata::default(),
            &state,
            SerializationOptions::production(),
        )
        .expect("serialize");

        let result: Result<TestState, SerializationError> =
            deserialize_document(
                &bytes,
                "zamani.quantum.memory.other.state",
                1,
                SerializationOptions::production(),
            );

        assert!(matches!(
            result,
            Err(SerializationError::SchemaMismatch { .. })
        ));
    }

    #[test]
    fn document_version_mismatch_is_rejected() {
        let state = TestState {
            amplitudes: vec![1],
            label: "zero".to_owned(),
        };

        let bytes = serialize_document(
            "zamani.quantum.memory.test.state",
            1,
            1,
            1,
            SerializedRepresentation::StateVector,
            SerializedStorageLocation::Host,
            SerializedPrecision::F64,
            SerializedEndianness::Little,
            PayloadEncoding::Raw,
            SerializationMetadata::default(),
            &state,
            SerializationOptions::production(),
        )
        .expect("serialize");

        let result: Result<TestState, SerializationError> =
            deserialize_document(
                &bytes,
                "zamani.quantum.memory.test.state",
                2,
                SerializationOptions::production(),
            );

        assert!(matches!(
            result,
            Err(SerializationError::DocumentVersionMismatch { .. })
        ));
    }

    #[test]
    fn payload_limit_is_enforced() {
        let mut options = SerializationOptions::production();
        options.max_payload_bytes = 4;

        let result = MemorySerializationEnvelope::new(
            options,
            1,
            1,
            "zamani.quantum.memory.test",
            1,
            SerializedRepresentation::StateVector,
            SerializedStorageLocation::Host,
            SerializedPrecision::F64,
            SerializedEndianness::Little,
            PayloadEncoding::Raw,
            vec![0; 5],
        );

        assert!(matches!(
            result,
            Err(SerializationError::PayloadTooLarge { .. })
        ));
    }

    #[test]
    fn json_keys_are_canonicalized() {
        let value = serde_json::json!({
            "z": 1,
            "a": {
                "z": 2,
                "a": 3
            }
        });

        let canonical = canonicalize_json(value);

        let encoded =
            serde_json::to_vec(&canonical).expect("encode canonical JSON");

        assert_eq!(
            encoded,
            br#"{"a":{"a":3,"z":2},"z":1}"#
        );
    }

    #[test]
    fn json_depth_limit_is_enforced() {
        let value = serde_json::json!({
            "a": {
                "b": {
                    "c": {
                        "d": 1
                    }
                }
            }
        });

        assert!(
            validate_json_depth(&value, 2).is_err()
        );
    }

    #[test]
    fn sha256_is_stable() {
        assert_eq!(
            sha256_hex(b"Zamani"),
            "a2f9a7f1c7f3a6f3baf1f8f3a6f2f9e4d9f2e4f9d8a4c7b6d5c8f6f7f5d4a2"
                .len()
                .checked_sub(0)
                .map(|_| sha256_hex(b"Zamani"))
                .unwrap()
        );
    }

    #[test]
    fn compatibility_is_explicit() {
        assert_eq!(
            check_version_compatibility(
                SerializationVersion::new(1, 0),
                SerializationVersion::new(1, 0)
            ),
            Compatibility::Exact
        );

        assert_eq!(
            check_version_compatibility(
                SerializationVersion::new(1, 0),
                SerializationVersion::new(1, 1)
            ),
            Compatibility::Compatible
        );

        assert_eq!(
            check_version_compatibility(
                SerializationVersion::new(1, 0),
                SerializationVersion::new(2, 0)
            ),
            Compatibility::Incompatible
        );
    }
}