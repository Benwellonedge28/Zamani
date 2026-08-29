//! Zamani Quantum Memory — Checkpoints
//!
//! Production-grade, provider-neutral checkpoint contracts for quantum
//! execution and quantum memory.
//!
//! # Purpose
//!
//! A checkpoint is a restartable execution artifact. It is stronger than a
//! state snapshot:
//!
//! ```text
//! snapshot
//!     = quantum/classical memory state
//!
//! checkpoint
//!     = memory state
//!     + execution cursor
//!     + classical execution state
//!     + RNG state where applicable
//!     + workload identity
//!     + representation metadata
//!     + hardware/backend provenance
//!     + compatibility metadata
//!     + integrity protection
//! ```
//!
//! This module intentionally does NOT implement:
//!
//! - a particular quantum-state representation;
//! - a state vector;
//! - a density matrix;
//! - a stabilizer tableau;
//! - tensor-network storage;
//! - GPU memory;
//! - CUDA/HIP/Metal/Vulkan APIs;
//! - QPU SDKs;
//! - provider credentials;
//! - network communication;
//! - filesystem I/O;
//! - hardware routing;
//! - scheduling;
//! - circuit optimization;
//! - QEC decoding;
//! - benchmarking.
//!
//! Those responsibilities belong to their respective Zamani subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! compiler / runtime / executor
//!      │
//!      ▼
//! quantum::memory
//!      │
//!      ├── state representation
//!      ├── classical memory
//!      ├── checkpoint
//!      └── snapshot
//!
//! checkpoint
//!      │
//!      ├──────────────► local simulator
//!      ├──────────────► GPU simulator
//!      ├──────────────► distributed simulator
//!      ├──────────────► logical/FTQC execution
//!      └──────────────► remote QPU execution
//! ```
//!
//! The checkpoint contract is deliberately provider-neutral. A checkpoint may
//! contain provenance for IBM, IonQ, Quantinuum, Rigetti, AWS Braket,
//! superconducting, trapped-ion, neutral-atom, photonic, spin,
//! topological, annealing, analog, simulator, emulator, distributed or future
//! quantum execution systems without this module importing any provider SDK.
//!
//! # Critical semantic rule
//!
//! A checkpoint MUST NOT claim that an arbitrary remote QPU can be restored
//! from a local quantum-memory image.
//!
//! For a real QPU, the provider may support only:
//!
//! - logical execution cursor restoration;
//! - classical execution restoration;
//! - job/session continuation;
//! - provider-native checkpoint handles;
//! - or no restoration at all.
//!
//! The `RestoreCapability` field explicitly records this distinction.
//!
//! # Security boundary
//!
//! This module never stores credentials.
//!
//! Checkpoints MUST NOT contain:
//!
//! - API keys;
//! - bearer tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - raw secret environment variables.
//!
//! Authentication references are opaque identifiers only.
//!
//! # Integrity
//!
//! Checkpoints use SHA-256 for integrity verification of the canonical
//! serialized payload. SHA-256 is used here as an integrity/deduplication
//! primitive, not as encryption.
//!
//! Confidential checkpoint data must be encrypted by a higher-level security
//!/storage layer.
//!
//! # Persistence boundary
//!
//! This file provides serialization/deserialization bytes but deliberately
//! performs no filesystem or network I/O.
//!
//! Callers may persist the returned bytes to:
//!
//! - local disk;
//! - object storage;
//! - encrypted device storage;
//! - distributed storage;
//! - Danga-managed package/artifact storage;
//! - provider-managed persistence.
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
//! - no `unsafe`.
//!
//! # File-completion invariant
//!
//! Once this file is complete, later implementation of:
//!
//! - `state.rs`;
//! - `serialization.rs`;
//! - `backend_state.rs`;
//! - `distributed.rs`;
//! - `hardware`;
//! - `runtime`;
//! - `snapshot.rs`;
//!
//! does NOT require changing the checkpoint contract merely to integrate.
//!
//! Later modules integrate by implementing the traits defined here or by
//! constructing the provider-neutral data structures directly.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Four-byte checkpoint magic.
pub const CHECKPOINT_MAGIC: [u8; 4] = *b"ZQCP";

/// Current checkpoint wire-format major version.
pub const CHECKPOINT_FORMAT_MAJOR: u16 = 1;

/// Current checkpoint wire-format minor version.
pub const CHECKPOINT_FORMAT_MINOR: u16 = 0;

/// Current checkpoint wire-format patch version.
pub const CHECKPOINT_FORMAT_PATCH: u16 = 0;

/// Maximum serialized checkpoint size accepted by this module.
///
/// This is a defensive parser limit, not a statement that every Zamani
/// deployment must use the same maximum. Larger deployments can perform
/// transport-level chunking above this layer.
pub const DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES: usize = 16 * 1024 * 1024 * 1024;

/// Maximum number of metadata entries accepted by default.
pub const DEFAULT_MAX_METADATA_ENTRIES: usize = 4096;

/// Maximum size of a single metadata key.
pub const DEFAULT_MAX_METADATA_KEY_BYTES: usize = 4096;

/// Maximum size of a single metadata value.
pub const DEFAULT_MAX_METADATA_VALUE_BYTES: usize = 1024 * 1024;

/// Maximum size of a textual identifier.
pub const DEFAULT_MAX_IDENTIFIER_BYTES: usize = 4096;

/// Maximum number of classical-memory bytes accepted by default.
pub const DEFAULT_MAX_CLASSICAL_MEMORY_BYTES: usize = 1024 * 1024 * 1024;

/// Maximum opaque RNG-state size accepted by default.
pub const DEFAULT_MAX_RNG_STATE_BYTES: usize = 1024 * 1024;

/// Maximum opaque provider-state reference size accepted by default.
pub const DEFAULT_MAX_PROVIDER_STATE_BYTES: usize = 16 * 1024 * 1024;

/// Maximum number of qubits represented by a checkpoint metadata declaration.
///
/// This is deliberately conservative. Actual quantum-state allocation limits
/// belong to `memory::limits`.
pub const DEFAULT_MAX_QUBITS: u64 = 1_000_000;

/// Maximum number of classical bits represented by a checkpoint metadata
/// declaration.
pub const DEFAULT_MAX_CLASSICAL_BITS: u64 = 100_000_000;

/// Maximum number of execution metadata entries.
pub const DEFAULT_MAX_EXECUTION_METADATA_ENTRIES: usize = 4096;

// =============================================================================
// Error taxonomy
// =============================================================================

/// Result type for checkpoint operations.
pub type CheckpointResult<T> = Result<T, CheckpointError>;

/// Structured checkpoint failure taxonomy.
///
/// The error type is intentionally independent from future `memory::errors`
/// so this file can be completed and tested before the rest of the memory
/// subsystem exists.
#[derive(Debug)]
pub enum CheckpointError {
    /// The checkpoint magic header is invalid.
    InvalidMagic {
        expected: [u8; 4],
        actual: [u8; 4],
    },

    /// The checkpoint format version is unsupported.
    UnsupportedFormatVersion {
        major: u16,
        minor: u16,
        patch: u16,
    },

    /// Serialized input is malformed.
    InvalidEncoding(String),

    /// Serialization failed.
    Serialization(String),

    /// Deserialization failed.
    Deserialization(String),

    /// A required field is invalid.
    InvalidField {
        field: &'static str,
        reason: String,
    },

    /// A configured limit was exceeded.
    LimitExceeded {
        resource: &'static str,
        actual: u128,
        maximum: u128,
    },

    /// The checkpoint integrity digest does not match its contents.
    IntegrityMismatch {
        expected: String,
        actual: String,
    },

    /// The checkpoint has no usable restore capability.
    RestoreUnavailable(String),

    /// The requested restore mode is not supported by the checkpoint.
    RestoreUnsupported {
        requested: RestoreMode,
        capability: RestoreCapability,
    },

    /// The target environment is incompatible with the checkpoint.
    IncompatibleTarget {
        reason: String,
    },

    /// The checkpoint was created by a newer incompatible schema.
    SchemaMismatch {
        expected: String,
        actual: String,
    },

    /// A checkpoint contains forbidden secret-like metadata.
    SecretMaterialRejected {
        field: String,
    },
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMagic { expected, actual } => write!(
                f,
                "invalid quantum checkpoint magic: expected {:?}, got {:?}",
                expected, actual
            ),

            Self::UnsupportedFormatVersion {
                major,
                minor,
                patch,
            } => write!(
                f,
                "unsupported quantum checkpoint format version {major}.{minor}.{patch}"
            ),

            Self::InvalidEncoding(reason) => {
                write!(f, "invalid checkpoint encoding: {reason}")
            }

            Self::Serialization(reason) => {
                write!(f, "checkpoint serialization failed: {reason}")
            }

            Self::Deserialization(reason) => {
                write!(f, "checkpoint deserialization failed: {reason}")
            }

            Self::InvalidField { field, reason } => {
                write!(f, "invalid checkpoint field `{field}`: {reason}")
            }

            Self::LimitExceeded {
                resource,
                actual,
                maximum,
            } => write!(
                f,
                "checkpoint resource `{resource}` exceeds limit: {actual} > {maximum}"
            ),

            Self::IntegrityMismatch { expected, actual } => write!(
                f,
                "checkpoint integrity mismatch: expected {expected}, got {actual}"
            ),

            Self::RestoreUnavailable(reason) => {
                write!(f, "checkpoint restoration unavailable: {reason}")
            }

            Self::RestoreUnsupported {
                requested,
                capability,
            } => write!(
                f,
                "restore mode {:?} is incompatible with checkpoint capability {:?}",
                requested, capability
            ),

            Self::IncompatibleTarget { reason } => {
                write!(f, "checkpoint target is incompatible: {reason}")
            }

            Self::SchemaMismatch { expected, actual } => write!(
                f,
                "checkpoint schema mismatch: expected {expected}, got {actual}"
            ),

            Self::SecretMaterialRejected { field } => {
                write!(f, "secret material is not permitted in checkpoint field `{field}`")
            }
        }
    }
}

impl std::error::Error for CheckpointError {}

// =============================================================================
// Version
// =============================================================================

/// Three-component checkpoint format version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckpointFormatVersion {
    /// Breaking format version.
    pub major: u16,

    /// Backwards-compatible format additions.
    pub minor: u16,

    /// Non-semantic corrections.
    pub patch: u16,
}

impl CheckpointFormatVersion {
    /// Current supported format.
    pub const CURRENT: Self = Self {
        major: CHECKPOINT_FORMAT_MAJOR,
        minor: CHECKPOINT_FORMAT_MINOR,
        patch: CHECKPOINT_FORMAT_PATCH,
    };

    /// Creates a format version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns whether this version can be decoded by the current reader.
    pub const fn is_compatible_with_current(self) -> bool {
        self.major == CHECKPOINT_FORMAT_MAJOR
            && self.minor <= CHECKPOINT_FORMAT_MINOR
    }
}

impl Default for CheckpointFormatVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl fmt::Display for CheckpointFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Restore semantics
// =============================================================================

/// What can actually be restored from a checkpoint.
///
/// This distinction is essential for supporting both simulators and real
/// QPUs without falsely treating remote hardware as a locally serializable
/// quantum-state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestoreCapability {
    /// Full local quantum and classical memory restoration.
    FullMemory,

    /// Quantum memory can be restored, but execution metadata requires
    /// reconstruction.
    QuantumMemory,

    /// Only classical execution state can be restored.
    ClassicalOnly,

    /// A provider/backend-native opaque continuation handle can be restored.
    BackendContinuation,

    /// The checkpoint can only be inspected or used for provenance.
    MetadataOnly,

    /// No restoration is possible.
    None,
}

impl RestoreCapability {
    /// Returns whether the capability contains quantum-memory restoration.
    pub const fn includes_quantum_memory(self) -> bool {
        matches!(
            self,
            Self::FullMemory | Self::QuantumMemory
        )
    }

    /// Returns whether the capability contains classical-state restoration.
    pub const fn includes_classical_memory(self) -> bool {
        matches!(
            self,
            Self::FullMemory
                | Self::QuantumMemory
                | Self::ClassicalOnly
                | Self::BackendContinuation
        )
    }

    /// Returns whether a backend continuation can be restored.
    pub const fn includes_backend_continuation(self) -> bool {
        matches!(self, Self::BackendContinuation)
    }
}

/// Requested restoration mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestoreMode {
    /// Restore everything that the checkpoint and target can safely restore.
    BestEffort,

    /// Require quantum memory restoration.
    RequireQuantumMemory,

    /// Require full local memory restoration.
    RequireFullMemory,

    /// Require classical execution restoration only.
    RequireClassicalOnly,

    /// Require backend-native continuation.
    RequireBackendContinuation,

    /// Only validate the checkpoint without restoring state.
    ValidateOnly,
}

impl RestoreMode {
    /// Checks whether a checkpoint capability satisfies this mode.
    pub const fn is_satisfied_by(self, capability: RestoreCapability) -> bool {
        match self {
            Self::BestEffort | Self::ValidateOnly => true,

            Self::RequireQuantumMemory => capability.includes_quantum_memory(),

            Self::RequireFullMemory => {
                matches!(capability, RestoreCapability::FullMemory)
            }

            Self::RequireClassicalOnly => {
                capability.includes_classical_memory()
            }

            Self::RequireBackendContinuation => {
                capability.includes_backend_continuation()
            }
        }
    }
}

// =============================================================================
// Representation and storage metadata
// =============================================================================

/// Generic representation identifier.
///
/// This is intentionally a stable string rather than an enum from a future
/// `memory::representation` module. It permits this file to be completed now
/// without creating a second competing representation enum.
///
/// Examples:
//!
//! - `"state_vector"`
//! - `"density_matrix"`
//! - `"stabilizer"`
//! - `"sparse"`
//! - `"mps"`
//! - `"backend_native"`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepresentationId(String);

impl RepresentationId {
    /// Creates a representation identifier.
    pub fn new(value: impl Into<String>) -> CheckpointResult<Self> {
        let value = value.into();

        validate_identifier("representation", &value)?;

        Ok(Self(value))
    }

    /// Returns the representation identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepresentationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Generic storage-location identifier.
///
/// Examples:
//!
//! - `"host"`
//! - `"pinned_host"`
//! - `"device"`
//! - `"unified"`
//! - `"distributed"`
//! - `"remote"`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StorageLocationId(String);

impl StorageLocationId {
    /// Creates a storage location identifier.
    pub fn new(value: impl Into<String>) -> CheckpointResult<Self> {
        let value = value.into();

        validate_identifier("storage_location", &value)?;

        Ok(Self(value))
    }

    /// Returns the storage location identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StorageLocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Numeric precision used by a checkpointed representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NumericPrecision {
    /// 32-bit real components.
    F32,

    /// 64-bit real components.
    F64,

    /// Extended/custom precision.
    Custom,
}

// =============================================================================
// Quantum memory image
// =============================================================================

/// Opaque quantum-memory image.
///
/// The checkpoint layer does not interpret the bytes. The corresponding
/// representation implementation is responsible for defining their format.
///
/// This permits:
//!
//! - state-vector checkpoints;
//! - density-matrix checkpoints;
//! - stabilizer checkpoints;
//! - sparse-state checkpoints;
//! - tensor-network checkpoints;
//! - provider-native opaque continuation payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantumMemoryImage {
    /// Quantum-state representation.
    pub representation: RepresentationId,

    /// Where the representation was materialized when captured.
    pub storage_location: StorageLocationId,

    /// Numerical precision, if applicable.
    pub precision: NumericPrecision,

    /// Number of logical qubits represented.
    pub qubit_count: u64,

    /// Logical-to-storage layout identifier.
    ///
    /// The layout itself is owned by `memory::layout`.
    pub layout_id: Option<String>,

    /// Opaque representation-specific serialized state.
    pub payload: Vec<u8>,
}

impl QuantumMemoryImage {
    /// Creates an opaque quantum-memory image.
    pub fn new(
        representation: RepresentationId,
        storage_location: StorageLocationId,
        precision: NumericPrecision,
        qubit_count: u64,
        layout_id: Option<String>,
        payload: Vec<u8>,
    ) -> CheckpointResult<Self> {
        validate_optional_identifier("layout_id", layout_id.as_deref())?;

        if qubit_count > DEFAULT_MAX_QUBITS {
            return Err(CheckpointError::LimitExceeded {
                resource: "qubit_count",
                actual: qubit_count as u128,
                maximum: DEFAULT_MAX_QUBITS as u128,
            });
        }

        Ok(Self {
            representation,
            storage_location,
            precision,
            qubit_count,
            layout_id,
            payload,
        })
    }

    /// Returns the payload size.
    pub fn payload_len(&self) -> usize {
        self.payload.len()
    }
}

// =============================================================================
// Classical execution memory
// =============================================================================

/// Classical execution memory captured by a checkpoint.
///
/// The format is intentionally opaque to this layer. The classical-memory
/// implementation may later provide a richer typed representation while
/// preserving this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassicalMemoryImage {
    /// Number of classical bits represented.
    pub bit_count: u64,

    /// Opaque serialized classical memory.
    pub payload: Vec<u8>,

    /// Optional implementation identifier.
    pub format_id: Option<String>,
}

impl ClassicalMemoryImage {
    /// Creates classical memory image data.
    pub fn new(
        bit_count: u64,
        payload: Vec<u8>,
        format_id: Option<String>,
    ) -> CheckpointResult<Self> {
        if bit_count > DEFAULT_MAX_CLASSICAL_BITS {
            return Err(CheckpointError::LimitExceeded {
                resource: "classical_bit_count",
                actual: bit_count as u128,
                maximum: DEFAULT_MAX_CLASSICAL_BITS as u128,
            });
        }

        validate_optional_identifier("classical_format_id", format_id.as_deref())?;

        Ok(Self {
            bit_count,
            payload,
            format_id,
        })
    }
}

// =============================================================================
// RNG state
// =============================================================================

/// Opaque random-number-generator state.
///
/// Quantum measurements, Monte Carlo algorithms, randomized compilation,
/// randomized benchmarking, stochastic noise models and sampling workloads
/// may depend on RNG state. The checkpoint layer therefore preserves it
/// without choosing a specific RNG implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RngState {
    /// Stable RNG implementation identifier.
    pub algorithm: String,

    /// Opaque serialized RNG state.
    pub state: Vec<u8>,
}

impl RngState {
    /// Creates RNG state.
    pub fn new(
        algorithm: impl Into<String>,
        state: Vec<u8>,
    ) -> CheckpointResult<Self> {
        let algorithm = algorithm.into();

        validate_identifier("rng_algorithm", &algorithm)?;

        if state.len() > DEFAULT_MAX_RNG_STATE_BYTES {
            return Err(CheckpointError::LimitExceeded {
                resource: "rng_state_bytes",
                actual: state.len() as u128,
                maximum: DEFAULT_MAX_RNG_STATE_BYTES as u128,
            });
        }

        Ok(Self { algorithm, state })
    }
}

// =============================================================================
// Execution cursor
// =============================================================================

/// Position within a quantum execution.
///
/// A cursor must be explicit because restoring memory without restoring the
/// execution position can cause an executor to replay already-applied
/// operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionCursor {
    /// Stable workload/circuit identifier.
    pub workload_id: String,

    /// Zero-based operation index.
    pub operation_index: u64,

    /// Optional block/region identifier for structured execution.
    pub region_id: Option<String>,

    /// Optional repetition/shot index.
    pub shot_index: Option<u64>,

    /// Optional branch identifier for dynamic-circuit execution.
    pub branch_id: Option<String>,
}

impl ExecutionCursor {
    /// Creates an execution cursor.
    pub fn new(
        workload_id: impl Into<String>,
        operation_index: u64,
    ) -> CheckpointResult<Self> {
        let workload_id = workload_id.into();

        validate_identifier("workload_id", &workload_id)?;

        Ok(Self {
            workload_id,
            operation_index,
            region_id: None,
            shot_index: None,
            branch_id: None,
        })
    }

    /// Sets an execution region.
    pub fn with_region(
        mut self,
        region_id: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let region_id = region_id.into();

        validate_identifier("region_id", &region_id)?;

        self.region_id = Some(region_id);

        Ok(self)
    }

    /// Sets a shot/repetition index.
    pub fn with_shot(mut self, shot_index: u64) -> Self {
        self.shot_index = Some(shot_index);
        self
    }

    /// Sets a dynamic-circuit branch identifier.
    pub fn with_branch(
        mut self,
        branch_id: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let branch_id = branch_id.into();

        validate_identifier("branch_id", &branch_id)?;

        self.branch_id = Some(branch_id);

        Ok(self)
    }
}

// =============================================================================
// Workload identity
// =============================================================================

/// Stable identity of the workload from which a checkpoint originated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkloadIdentity {
    /// Human-readable or compiler-generated workload identifier.
    pub workload_id: String,

    /// Cryptographic digest of the canonical workload representation.
    ///
    /// The checkpoint layer does not prescribe how the workload is canonicalized.
    pub workload_digest_sha256: Option<String>,

    /// Optional source/module identity.
    pub module_id: Option<String>,

    /// Optional compiler/IR schema identity.
    pub ir_schema: Option<String>,
}

impl WorkloadIdentity {
    /// Creates a workload identity.
    pub fn new(workload_id: impl Into<String>) -> CheckpointResult<Self> {
        let workload_id = workload_id.into();

        validate_identifier("workload_id", &workload_id)?;

        Ok(Self {
            workload_id,
            workload_digest_sha256: None,
            module_id: None,
            ir_schema: None,
        })
    }

    /// Attaches a workload digest.
    pub fn with_digest(
        mut self,
        digest: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let digest = digest.into();

        validate_sha256_hex("workload_digest_sha256", &digest)?;

        self.workload_digest_sha256 = Some(digest);

        Ok(self)
    }

    /// Attaches a module identifier.
    pub fn with_module(
        mut self,
        module_id: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let module_id = module_id.into();

        validate_identifier("module_id", &module_id)?;

        self.module_id = Some(module_id);

        Ok(self)
    }

    /// Attaches an IR schema identifier.
    pub fn with_ir_schema(
        mut self,
        ir_schema: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let ir_schema = ir_schema.into();

        validate_identifier("ir_schema", &ir_schema)?;

        self.ir_schema = Some(ir_schema);

        Ok(self)
    }
}

// =============================================================================
// Hardware provenance
// =============================================================================

/// Provider-neutral hardware/backend provenance.
///
/// This is metadata only. It is deliberately not a hardware SDK object.
///
/// The structure is broad enough for:
//!
//! - QPUs;
//! - simulators;
//! - emulators;
//! - analog systems;
//! - annealers;
//! - logical/FTQC systems;
//! - distributed quantum systems;
//! - future hardware.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareProvenance {
    /// Provider identity, if applicable.
    pub provider_id: Option<String>,

    /// Backend identity.
    pub backend_id: Option<String>,

    /// Device identity.
    pub device_id: Option<String>,

    /// Physical technology identifier.
    ///
    /// Examples:
//!
//! - `superconducting`
//! - `trapped_ion`
//! - `neutral_atom`
//! - `photonic`
//! - `spin`
//! - `topological`
//! - `analog`
//! - `annealing`
//! - `simulator`
    pub technology: Option<String>,

    /// Execution model.
    ///
    /// Examples:
//!
//! - `gate`
//! - `dynamic_circuit`
//! - `pulse`
//! - `analog`
//! - `annealing`
//! - `logical`
    pub execution_model: Option<String>,

    /// Hardware revision.
    pub hardware_revision: Option<String>,

    /// Firmware version.
    pub firmware_version: Option<String>,

    /// Instruction-set/schema version.
    pub instruction_set_version: Option<String>,

    /// Adapter version.
    pub adapter_version: Option<String>,

    /// Provider API version.
    pub provider_api_version: Option<String>,

    /// Calibration snapshot identity.
    pub calibration_id: Option<String>,

    /// Calibration timestamp in Unix nanoseconds, when available.
    pub calibration_timestamp_unix_ns: Option<u64>,

    /// Topology revision.
    pub topology_version: Option<String>,

    /// Provider-managed continuation/job/session reference.
    ///
    /// This MUST be an opaque non-secret identifier. It is not an API token.
    pub continuation_reference: Option<String>,
}

impl Default for HardwareProvenance {
    fn default() -> Self {
        Self {
            provider_id: None,
            backend_id: None,
            device_id: None,
            technology: None,
            execution_model: None,
            hardware_revision: None,
            firmware_version: None,
            instruction_set_version: None,
            adapter_version: None,
            provider_api_version: None,
            calibration_id: None,
            calibration_timestamp_unix_ns: None,
            topology_version: None,
            continuation_reference: None,
        }
    }
}

impl HardwareProvenance {
    /// Validates all provenance identifiers.
    pub fn validate(&self) -> CheckpointResult<()> {
        validate_optional_identifier("provider_id", self.provider_id.as_deref())?;
        validate_optional_identifier("backend_id", self.backend_id.as_deref())?;
        validate_optional_identifier("device_id", self.device_id.as_deref())?;
        validate_optional_identifier("technology", self.technology.as_deref())?;
        validate_optional_identifier(
            "execution_model",
            self.execution_model.as_deref(),
        )?;
        validate_optional_identifier(
            "hardware_revision",
            self.hardware_revision.as_deref(),
        )?;
        validate_optional_identifier(
            "firmware_version",
            self.firmware_version.as_deref(),
        )?;
        validate_optional_identifier(
            "instruction_set_version",
            self.instruction_set_version.as_deref(),
        )?;
        validate_optional_identifier(
            "adapter_version",
            self.adapter_version.as_deref(),
        )?;
        validate_optional_identifier(
            "provider_api_version",
            self.provider_api_version.as_deref(),
        )?;
        validate_optional_identifier(
            "calibration_id",
            self.calibration_id.as_deref(),
        )?;
        validate_optional_identifier(
            "topology_version",
            self.topology_version.as_deref(),
        )?;

        if let Some(reference) = self.continuation_reference.as_deref() {
            validate_identifier("continuation_reference", reference)?;
            reject_secret_like_value(
                "continuation_reference",
                reference,
            )?;
        }

        Ok(())
    }

    /// Returns whether the provenance identifies a real hardware/backend
    /// execution environment.
    pub fn identifies_execution_target(&self) -> bool {
        self.backend_id.is_some()
            || self.device_id.is_some()
            || self.provider_id.is_some()
    }
}

// =============================================================================
// Compatibility requirements
// =============================================================================

/// Requirements that a restoration target must satisfy.
///
/// These requirements are deliberately descriptive rather than tied to a
/// particular hardware API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreRequirements {
    /// Required representation identifier.
    pub representation: Option<RepresentationId>,

    /// Required storage location.
    pub storage_location: Option<StorageLocationId>,

    /// Required qubit count.
    pub qubit_count: Option<u64>,

    /// Required classical bit count.
    pub classical_bit_count: Option<u64>,

    /// Required provider identifier.
    pub provider_id: Option<String>,

    /// Required backend identifier.
    pub backend_id: Option<String>,

    /// Required device identifier.
    pub device_id: Option<String>,

    /// Required technology.
    pub technology: Option<String>,

    /// Required execution model.
    pub execution_model: Option<String>,

    /// Whether calibration identity must match exactly.
    pub require_calibration_match: bool,

    /// Whether hardware revision must match exactly.
    pub require_hardware_revision_match: bool,

    /// Whether topology version must match exactly.
    pub require_topology_match: bool,
}

impl Default for RestoreRequirements {
    fn default() -> Self {
        Self {
            representation: None,
            storage_location: None,
            qubit_count: None,
            classical_bit_count: None,
            provider_id: None,
            backend_id: None,
            device_id: None,
            technology: None,
            execution_model: None,
            require_calibration_match: false,
            require_hardware_revision_match: false,
            require_topology_match: false,
        }
    }
}

// =============================================================================
// Checkpoint metadata
// =============================================================================

/// General checkpoint metadata.
///
/// Arbitrary metadata uses `BTreeMap` to keep serialization deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    /// Zamani schema identifier.
    pub schema_id: String,

    /// Zamani compiler/runtime version.
    pub zamani_version: String,

    /// Optional source-language version.
    pub language_version: Option<String>,

    /// Optional runtime version.
    pub runtime_version: Option<String>,

    /// Creation timestamp in Unix nanoseconds.
    pub created_at_unix_ns: u64,

    /// Human-readable checkpoint label.
    pub label: Option<String>,

    /// Additional non-secret metadata.
    pub attributes: BTreeMap<String, String>,
}

impl Default for CheckpointMetadata {
    fn default() -> Self {
        Self {
            schema_id: "zamani.quantum.memory.checkpoint".to_owned(),
            zamani_version: env!("CARGO_PKG_VERSION").to_owned(),
            language_version: None,
            runtime_version: None,
            created_at_unix_ns: 0,
            label: None,
            attributes: BTreeMap::new(),
        }
    }
}

impl CheckpointMetadata {
    /// Validates metadata and rejects secret-like values.
    pub fn validate(&self) -> CheckpointResult<()> {
        validate_identifier("schema_id", &self.schema_id)?;
        validate_identifier("zamani_version", &self.zamani_version)?;

        validate_optional_identifier(
            "language_version",
            self.language_version.as_deref(),
        )?;

        validate_optional_identifier(
            "runtime_version",
            self.runtime_version.as_deref(),
        )?;

        if let Some(label) = self.label.as_deref() {
            if label.as_bytes().len() > DEFAULT_MAX_IDENTIFIER_BYTES {
                return Err(CheckpointError::LimitExceeded {
                    resource: "checkpoint_label_bytes",
                    actual: label.as_bytes().len() as u128,
                    maximum: DEFAULT_MAX_IDENTIFIER_BYTES as u128,
                });
            }

            reject_secret_like_value("label", label)?;
        }

        if self.attributes.len() > DEFAULT_MAX_METADATA_ENTRIES {
            return Err(CheckpointError::LimitExceeded {
                resource: "metadata_entries",
                actual: self.attributes.len() as u128,
                maximum: DEFAULT_MAX_METADATA_ENTRIES as u128,
            });
        }

        for (key, value) in &self.attributes {
            if key.as_bytes().len() > DEFAULT_MAX_METADATA_KEY_BYTES {
                return Err(CheckpointError::LimitExceeded {
                    resource: "metadata_key_bytes",
                    actual: key.as_bytes().len() as u128,
                    maximum: DEFAULT_MAX_METADATA_KEY_BYTES as u128,
                });
            }

            if value.as_bytes().len() > DEFAULT_MAX_METADATA_VALUE_BYTES {
                return Err(CheckpointError::LimitExceeded {
                    resource: "metadata_value_bytes",
                    actual: value.as_bytes().len() as u128,
                    maximum: DEFAULT_MAX_METADATA_VALUE_BYTES as u128,
                });
            }

            reject_secret_like_value(key, value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Checkpoint payload
// =============================================================================

/// Complete logical checkpoint payload.
///
/// This structure contains everything needed by the checkpoint layer to
/// describe a restartable execution state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointPayload {
    /// General checkpoint metadata.
    pub metadata: CheckpointMetadata,

    /// Workload identity.
    pub workload: WorkloadIdentity,

    /// Current execution position.
    pub cursor: ExecutionCursor,

    /// Quantum-memory image, if local quantum memory was captured.
    pub quantum_memory: Option<QuantumMemoryImage>,

    /// Classical-memory image, if captured.
    pub classical_memory: Option<ClassicalMemoryImage>,

    /// RNG state, if deterministic continuation requires it.
    pub rng_state: Option<RngState>,

    /// Hardware/backend provenance.
    pub hardware: HardwareProvenance,

    /// What restoration is semantically possible.
    pub restore_capability: RestoreCapability,

    /// Optional provider/backend continuation metadata.
    pub provider_state: Option<ProviderStateReference>,

    /// User/runtime-defined restoration requirements.
    pub restore_requirements: RestoreRequirements,

    /// Additional non-secret execution metadata.
    pub execution_attributes: BTreeMap<String, String>,
}

impl CheckpointPayload {
    /// Validates the complete payload.
    pub fn validate(&self) -> CheckpointResult<()> {
        self.metadata.validate()?;

        validate_workload(&self.workload)?;

        validate_execution_cursor(&self.cursor)?;

        self.hardware.validate()?;

        validate_optional_provider_state(self.provider_state.as_ref())?;

        if self.execution_attributes.len()
            > DEFAULT_MAX_EXECUTION_METADATA_ENTRIES
        {
            return Err(CheckpointError::LimitExceeded {
                resource: "execution_metadata_entries",
                actual: self.execution_attributes.len() as u128,
                maximum: DEFAULT_MAX_EXECUTION_METADATA_ENTRIES as u128,
            });
        }

        for (key, value) in &self.execution_attributes {
            reject_secret_like_value(key, value)?;
        }

        if let Some(image) = &self.quantum_memory {
            validate_quantum_memory_image(image)?;
        }

        if let Some(classical) = &self.classical_memory {
            validate_classical_memory_image(classical)?;
        }

        if let Some(rng) = &self.rng_state {
            validate_rng_state(rng)?;
        }

        validate_restore_capability_consistency(self)?;

        Ok(())
    }
}

// =============================================================================
// Provider-native continuation
// =============================================================================

/// Opaque provider/backend continuation state.
///
/// This is deliberately separate from `QuantumMemoryImage`.
///
/// For a real QPU, the provider may return a session/job/continuation handle
/// instead of serializable qubit amplitudes.
///
/// The reference MUST NOT be a credential.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStateReference {
    /// Provider identifier.
    pub provider_id: String,

    /// Backend/device identifier.
    pub backend_id: Option<String>,

    /// Provider-native state/continuation kind.
    pub state_kind: String,

    /// Opaque non-secret provider state.
    pub payload: Vec<u8>,

    /// Optional provider schema version.
    pub schema_version: Option<String>,
}

impl ProviderStateReference {
    /// Creates a provider-state reference.
    pub fn new(
        provider_id: impl Into<String>,
        state_kind: impl Into<String>,
        payload: Vec<u8>,
    ) -> CheckpointResult<Self> {
        let provider_id = provider_id.into();
        let state_kind = state_kind.into();

        validate_identifier("provider_id", &provider_id)?;
        validate_identifier("state_kind", &state_kind)?;

        if payload.len() > DEFAULT_MAX_PROVIDER_STATE_BYTES {
            return Err(CheckpointError::LimitExceeded {
                resource: "provider_state_bytes",
                actual: payload.len() as u128,
                maximum: DEFAULT_MAX_PROVIDER_STATE_BYTES as u128,
            });
        }

        Ok(Self {
            provider_id,
            backend_id: None,
            state_kind,
            payload,
            schema_version: None,
        })
    }

    /// Sets the backend identifier.
    pub fn with_backend(
        mut self,
        backend_id: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let backend_id = backend_id.into();

        validate_identifier("backend_id", &backend_id)?;

        self.backend_id = Some(backend_id);

        Ok(self)
    }

    /// Sets the provider state schema version.
    pub fn with_schema_version(
        mut self,
        version: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let version = version.into();

        validate_identifier("provider_state_schema_version", &version)?;

        self.schema_version = Some(version);

        Ok(self)
    }
}

// =============================================================================
// Checkpoint envelope
// =============================================================================

/// Integrity digest stored alongside the checkpoint payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointIntegrity {
    /// Digest algorithm.
    pub algorithm: String,

    /// Lowercase hexadecimal SHA-256 digest.
    pub digest_hex: String,
}

impl CheckpointIntegrity {
    /// Creates a SHA-256 integrity record.
    pub fn sha256(payload: &[u8]) -> Self {
        let digest = Sha256::digest(payload);

        Self {
            algorithm: "sha256".to_owned(),
            digest_hex: hex::encode(digest),
        }
    }

    /// Verifies a payload against this integrity record.
    pub fn verify(&self, payload: &[u8]) -> CheckpointResult<()> {
        if self.algorithm != "sha256" {
            return Err(CheckpointError::InvalidField {
                field: "integrity.algorithm",
                reason: "unsupported integrity algorithm".to_owned(),
            });
        }

        validate_sha256_hex("integrity.digest_hex", &self.digest_hex)?;

        let actual = Sha256::digest(payload);
        let actual_hex = hex::encode(actual);

        if actual_hex != self.digest_hex {
            return Err(CheckpointError::IntegrityMismatch {
                expected: self.digest_hex.clone(),
                actual: actual_hex,
            });
        }

        Ok(())
    }
}

/// Complete checkpoint envelope.
///
/// The envelope is the persisted object. The payload is hashed independently
/// so integrity verification can occur before the checkpoint is accepted by
/// a runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuantumCheckpoint {
    /// Checkpoint wire-format version.
    pub format_version: CheckpointFormatVersion,

    /// Stable magic identifier represented as UTF-8 text in the JSON envelope.
    pub magic: String,

    /// Checkpoint payload.
    pub payload: CheckpointPayload,

    /// Integrity of the canonical JSON representation of `payload`.
    pub integrity: CheckpointIntegrity,
}

impl QuantumCheckpoint {
    /// Constructs a validated checkpoint from a payload.
    pub fn new(payload: CheckpointPayload) -> CheckpointResult<Self> {
        payload.validate()?;

        let payload_bytes = canonical_payload_bytes(&payload)?;

        let checkpoint = Self {
            format_version: CheckpointFormatVersion::CURRENT,
            magic: String::from_utf8_lossy(&CHECKPOINT_MAGIC).into_owned(),
            payload,
            integrity: CheckpointIntegrity::sha256(&payload_bytes),
        };

        checkpoint.validate()?;

        Ok(checkpoint)
    }

    /// Returns the checkpoint format version.
    pub const fn format_version(&self) -> CheckpointFormatVersion {
        self.format_version
    }

    /// Returns the restoration capability.
    pub const fn restore_capability(&self) -> RestoreCapability {
        self.payload.restore_capability
    }

    /// Returns the workload identity.
    pub fn workload(&self) -> &WorkloadIdentity {
        &self.payload.workload
    }

    /// Returns the execution cursor.
    pub fn cursor(&self) -> &ExecutionCursor {
        &self.payload.cursor
    }

    /// Returns whether quantum memory is present.
    pub fn contains_quantum_memory(&self) -> bool {
        self.payload.quantum_memory.is_some()
    }

    /// Returns whether classical memory is present.
    pub fn contains_classical_memory(&self) -> bool {
        self.payload.classical_memory.is_some()
    }

    /// Validates the complete checkpoint including integrity.
    pub fn validate(&self) -> CheckpointResult<()> {
        validate_magic(&self.magic)?;

        if !self.format_version.is_compatible_with_current() {
            return Err(CheckpointError::UnsupportedFormatVersion {
                major: self.format_version.major,
                minor: self.format_version.minor,
                patch: self.format_version.patch,
            });
        }

        self.payload.validate()?;

        let payload_bytes = canonical_payload_bytes(&self.payload)?;

        self.integrity.verify(&payload_bytes)?;

        Ok(())
    }

    /// Serializes the checkpoint into the canonical JSON wire format.
    ///
    /// No filesystem or network I/O occurs.
    pub fn to_bytes(&self) -> CheckpointResult<Vec<u8>> {
        self.validate()?;

        let bytes = serde_json::to_vec(self)
            .map_err(|error| CheckpointError::Serialization(error.to_string()))?;

        if bytes.len() > DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES {
            return Err(CheckpointError::LimitExceeded {
                resource: "serialized_checkpoint_bytes",
                actual: bytes.len() as u128,
                maximum: DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES as u128,
            });
        }

        Ok(bytes)
    }

    /// Deserializes and validates a checkpoint.
    pub fn from_bytes(bytes: &[u8]) -> CheckpointResult<Self> {
        if bytes.len() > DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES {
            return Err(CheckpointError::LimitExceeded {
                resource: "serialized_checkpoint_bytes",
                actual: bytes.len() as u128,
                maximum: DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES as u128,
            });
        }

        let checkpoint: Self = serde_json::from_slice(bytes)
            .map_err(|error| CheckpointError::Deserialization(error.to_string()))?;

        checkpoint.validate()?;

        Ok(checkpoint)
    }

    /// Serializes the payload canonically for integrity computation.
    pub fn payload_digest_sha256(&self) -> CheckpointResult<String> {
        let bytes = canonical_payload_bytes(&self.payload)?;
        Ok(hex::encode(Sha256::digest(bytes)))
    }

    /// Checks whether a requested restore mode can be satisfied by this
    /// checkpoint's declared semantics.
    pub fn check_restore_mode(
        &self,
        mode: RestoreMode,
    ) -> CheckpointResult<()> {
        self.validate()?;

        if mode.is_satisfied_by(self.restore_capability()) {
            Ok(())
        } else {
            Err(CheckpointError::RestoreUnsupported {
                requested: mode,
                capability: self.restore_capability(),
            })
        }
    }

    /// Validates the checkpoint against generic restoration requirements.
    ///
    /// This does not inspect hardware SDK objects. The target environment
    /// supplies provider-neutral descriptors.
    pub fn check_target(
        &self,
        target: &RestoreTarget,
    ) -> CheckpointResult<()> {
        self.validate()?;

        validate_restore_target(target)?;

        check_requirements(
            &self.payload,
            target,
        )
    }
}

// =============================================================================
// Restore target
// =============================================================================

/// Provider-neutral description of the environment into which a checkpoint
/// would be restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreTarget {
    /// Available state representation.
    pub representation: Option<RepresentationId>,

    /// Available storage location.
    pub storage_location: Option<StorageLocationId>,

    /// Number of available logical/physical qubits.
    pub qubit_count: Option<u64>,

    /// Number of available classical bits.
    pub classical_bit_count: Option<u64>,

    /// Provider identity.
    pub provider_id: Option<String>,

    /// Backend identity.
    pub backend_id: Option<String>,

    /// Device identity.
    pub device_id: Option<String>,

    /// Physical technology.
    pub technology: Option<String>,

    /// Execution model.
    pub execution_model: Option<String>,

    /// Hardware revision.
    pub hardware_revision: Option<String>,

    /// Calibration identity.
    pub calibration_id: Option<String>,

    /// Topology version.
    pub topology_version: Option<String>,
}

impl Default for RestoreTarget {
    fn default() -> Self {
        Self {
            representation: None,
            storage_location: None,
            qubit_count: None,
            classical_bit_count: None,
            provider_id: None,
            backend_id: None,
            device_id: None,
            technology: None,
            execution_model: None,
            hardware_revision: None,
            calibration_id: None,
            topology_version: None,
        }
    }
}

// =============================================================================
// Checkpoint policy
// =============================================================================

/// Resource limits used while creating or reading checkpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointLimits {
    /// Maximum serialized checkpoint size.
    pub max_serialized_bytes: usize,

    /// Maximum metadata entries.
    pub max_metadata_entries: usize,

    /// Maximum metadata key size.
    pub max_metadata_key_bytes: usize,

    /// Maximum metadata value size.
    pub max_metadata_value_bytes: usize,

    /// Maximum classical memory size.
    pub max_classical_memory_bytes: usize,

    /// Maximum RNG state size.
    pub max_rng_state_bytes: usize,

    /// Maximum provider state size.
    pub max_provider_state_bytes: usize,

    /// Maximum declared qubit count.
    pub max_qubits: u64,

    /// Maximum declared classical bit count.
    pub max_classical_bits: u64,
}

impl Default for CheckpointLimits {
    fn default() -> Self {
        Self {
            max_serialized_bytes:
                DEFAULT_MAX_SERIALIZED_CHECKPOINT_BYTES,
            max_metadata_entries: DEFAULT_MAX_METADATA_ENTRIES,
            max_metadata_key_bytes: DEFAULT_MAX_METADATA_KEY_BYTES,
            max_metadata_value_bytes:
                DEFAULT_MAX_METADATA_VALUE_BYTES,
            max_classical_memory_bytes:
                DEFAULT_MAX_CLASSICAL_MEMORY_BYTES,
            max_rng_state_bytes: DEFAULT_MAX_RNG_STATE_BYTES,
            max_provider_state_bytes:
                DEFAULT_MAX_PROVIDER_STATE_BYTES,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_classical_bits: DEFAULT_MAX_CLASSICAL_BITS,
        }
    }
}

/// Policy governing checkpoint creation and restoration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointPolicy {
    /// Resource limits.
    pub limits: CheckpointLimits,

    /// Whether integrity verification is mandatory.
    pub require_integrity: bool,

    /// Whether provider continuation references are allowed.
    pub allow_provider_continuation: bool,

    /// Whether opaque provider state is allowed.
    pub allow_provider_state: bool,

    /// Whether hardware-bound checkpoints may be restored onto a different
    /// target.
    pub allow_cross_target_restore: bool,
}

impl Default for CheckpointPolicy {
    fn default() -> Self {
        Self {
            limits: CheckpointLimits::default(),
            require_integrity: true,
            allow_provider_continuation: true,
            allow_provider_state: true,
            allow_cross_target_restore: false,
        }
    }
}

// =============================================================================
// Checkpoint builder
// =============================================================================

/// Builder for constructing checkpoints without partially initialized
/// checkpoint objects.
///
/// The builder is intentionally provider-neutral.
#[derive(Debug, Default)]
pub struct CheckpointBuilder {
    metadata: Option<CheckpointMetadata>,
    workload: Option<WorkloadIdentity>,
    cursor: Option<ExecutionCursor>,
    quantum_memory: Option<QuantumMemoryImage>,
    classical_memory: Option<ClassicalMemoryImage>,
    rng_state: Option<RngState>,
    hardware: HardwareProvenance,
    restore_capability: Option<RestoreCapability>,
    provider_state: Option<ProviderStateReference>,
    restore_requirements: RestoreRequirements,
    execution_attributes: BTreeMap<String, String>,
}

impl CheckpointBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets metadata.
    pub fn metadata(mut self, metadata: CheckpointMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Sets workload identity.
    pub fn workload(mut self, workload: WorkloadIdentity) -> Self {
        self.workload = Some(workload);
        self
    }

    /// Sets execution cursor.
    pub fn cursor(mut self, cursor: ExecutionCursor) -> Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sets quantum-memory image.
    pub fn quantum_memory(
        mut self,
        image: QuantumMemoryImage,
    ) -> Self {
        self.quantum_memory = Some(image);
        self
    }

    /// Sets classical-memory image.
    pub fn classical_memory(
        mut self,
        image: ClassicalMemoryImage,
    ) -> Self {
        self.classical_memory = Some(image);
        self
    }

    /// Sets RNG state.
    pub fn rng_state(mut self, rng_state: RngState) -> Self {
        self.rng_state = Some(rng_state);
        self
    }

    /// Sets hardware provenance.
    pub fn hardware(
        mut self,
        hardware: HardwareProvenance,
    ) -> Self {
        self.hardware = hardware;
        self
    }

    /// Sets restoration capability.
    pub fn restore_capability(
        mut self,
        capability: RestoreCapability,
    ) -> Self {
        self.restore_capability = Some(capability);
        self
    }

    /// Sets provider-native continuation state.
    pub fn provider_state(
        mut self,
        provider_state: ProviderStateReference,
    ) -> Self {
        self.provider_state = Some(provider_state);
        self
    }

    /// Sets restoration requirements.
    pub fn restore_requirements(
        mut self,
        requirements: RestoreRequirements,
    ) -> Self {
        self.restore_requirements = requirements;
        self
    }

    /// Adds a non-secret execution attribute.
    pub fn execution_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let key = key.into();
        let value = value.into();

        reject_secret_like_value(&key, &value)?;

        self.execution_attributes.insert(key, value);

        Ok(self)
    }

    /// Builds and validates the checkpoint.
    pub fn build(self) -> CheckpointResult<QuantumCheckpoint> {
        let workload = self.workload.ok_or(CheckpointError::InvalidField {
            field: "workload",
            reason: "workload identity is required".to_owned(),
        })?;

        let cursor = self.cursor.ok_or(CheckpointError::InvalidField {
            field: "cursor",
            reason: "execution cursor is required".to_owned(),
        })?;

        let capability =
            self.restore_capability.ok_or(CheckpointError::InvalidField {
                field: "restore_capability",
                reason: "restore capability is required".to_owned(),
            })?;

        let payload = CheckpointPayload {
            metadata: self.metadata.unwrap_or_default(),
            workload,
            cursor,
            quantum_memory: self.quantum_memory,
            classical_memory: self.classical_memory,
            rng_state: self.rng_state,
            hardware: self.hardware,
            restore_capability: capability,
            provider_state: self.provider_state,
            restore_requirements: self.restore_requirements,
            execution_attributes: self.execution_attributes,
        };

        QuantumCheckpoint::new(payload)
    }
}

// =============================================================================
// Trait integration boundaries
// =============================================================================

/// Trait implemented by a quantum-state representation that can export
/// checkpoint data.
///
/// `state.rs` can implement this trait later without changing this file.
pub trait CheckpointQuantumState {
    /// Exports the state as a representation-specific checkpoint image.
    fn checkpoint_image(&self) -> CheckpointResult<QuantumMemoryImage>;
}

/// Trait implemented by classical memory implementations.
pub trait CheckpointClassicalState {
    /// Exports classical memory.
    fn checkpoint_image(&self) -> CheckpointResult<ClassicalMemoryImage>;
}

/// Trait implemented by execution engines that can expose an execution
/// cursor.
pub trait CheckpointExecutionCursor {
    /// Returns the current execution cursor.
    fn checkpoint_cursor(&self) -> CheckpointResult<ExecutionCursor>;
}

/// Trait implemented by RNG providers whose state can be serialized.
pub trait CheckpointRng {
    /// Returns opaque RNG state.
    fn checkpoint_rng_state(&self) -> CheckpointResult<RngState>;
}

/// Trait implemented by provider-neutral hardware adapters.
///
/// This trait deliberately returns metadata, not provider SDK types.
pub trait CheckpointHardware {
    /// Returns provider-neutral hardware provenance.
    fn checkpoint_hardware_provenance(
        &self,
    ) -> CheckpointResult<HardwareProvenance>;

    /// Returns provider-native continuation state where supported.
    fn checkpoint_provider_state(
        &self,
    ) -> CheckpointResult<Option<ProviderStateReference>>;

    /// Returns what kind of restoration the backend can support.
    fn checkpoint_restore_capability(
        &self,
    ) -> CheckpointResult<RestoreCapability>;
}

/// Trait implemented by a runtime/executor capable of creating checkpoints.
///
/// This is the intended integration point for `quantum::runtime` once that
/// subsystem is connected.
pub trait CheckpointSource {
    /// Builds a complete checkpoint.
    fn create_checkpoint(&self) -> CheckpointResult<QuantumCheckpoint>;
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_magic(magic: &str) -> CheckpointResult<()> {
    let expected = String::from_utf8_lossy(&CHECKPOINT_MAGIC);

    if magic != expected {
        return Err(CheckpointError::InvalidMagic {
            expected: CHECKPOINT_MAGIC,
            actual: {
                let bytes = magic.as_bytes();

                if bytes.len() >= 4 {
                    [bytes[0], bytes[1], bytes[2], bytes[3]]
                } else {
                    let mut actual = [0u8; 4];
                    for (index, byte) in bytes.iter().enumerate() {
                        actual[index] = *byte;
                    }
                    actual
                }
            },
        });
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> CheckpointResult<()> {
    let bytes = value.as_bytes();

    if bytes.is_empty() {
        return Err(CheckpointError::InvalidField {
            field,
            reason: "identifier cannot be empty".to_owned(),
        });
    }

    if bytes.len() > DEFAULT_MAX_IDENTIFIER_BYTES {
        return Err(CheckpointError::LimitExceeded {
            resource: field,
            actual: bytes.len() as u128,
            maximum: DEFAULT_MAX_IDENTIFIER_BYTES as u128,
        });
    }

    if value.chars().any(|character| character.is_control()) {
        return Err(CheckpointError::InvalidField {
            field,
            reason: "identifier contains a control character".to_owned(),
        });
    }

    Ok(())
}

fn validate_optional_identifier(
    field: &'static str,
    value: Option<&str>,
) -> CheckpointResult<()> {
    if let Some(value) = value {
        validate_identifier(field, value)?;
    }

    Ok(())
}

fn validate_sha256_hex(
    field: &'static str,
    value: &str,
) -> CheckpointResult<()> {
    if value.len() != 64 {
        return Err(CheckpointError::InvalidField {
            field,
            reason: "SHA-256 digest must contain exactly 64 hexadecimal characters"
                .to_owned(),
        });
    }

    if !value
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(CheckpointError::InvalidField {
            field,
            reason: "SHA-256 digest contains non-hexadecimal characters"
                .to_owned(),
        });
    }

    Ok(())
}

fn validate_workload(
    workload: &WorkloadIdentity,
) -> CheckpointResult<()> {
    validate_identifier("workload_id", &workload.workload_id)?;

    if let Some(digest) =
        workload.workload_digest_sha256.as_deref()
    {
        validate_sha256_hex(
            "workload_digest_sha256",
            digest,
        )?;
    }

    validate_optional_identifier(
        "module_id",
        workload.module_id.as_deref(),
    )?;

    validate_optional_identifier(
        "ir_schema",
        workload.ir_schema.as_deref(),
    )?;

    Ok(())
}

fn validate_execution_cursor(
    cursor: &ExecutionCursor,
) -> CheckpointResult<()> {
    validate_identifier("workload_id", &cursor.workload_id)?;

    validate_optional_identifier(
        "region_id",
        cursor.region_id.as_deref(),
    )?;

    validate_optional_identifier(
        "branch_id",
        cursor.branch_id.as_deref(),
    )?;

    Ok(())
}

fn validate_quantum_memory_image(
    image: &QuantumMemoryImage,
) -> CheckpointResult<()> {
    validate_identifier(
        "quantum_memory.representation",
        image.representation.as_str(),
    )?;

    validate_identifier(
        "quantum_memory.storage_location",
        image.storage_location.as_str(),
    )?;

    if image.qubit_count > DEFAULT_MAX_QUBITS {
        return Err(CheckpointError::LimitExceeded {
            resource: "quantum_memory.qubit_count",
            actual: image.qubit_count as u128,
            maximum: DEFAULT_MAX_QUBITS as u128,
        });
    }

    validate_optional_identifier(
        "quantum_memory.layout_id",
        image.layout_id.as_deref(),
    )?;

    Ok(())
}

fn validate_classical_memory_image(
    image: &ClassicalMemoryImage,
) -> CheckpointResult<()> {
    if image.bit_count > DEFAULT_MAX_CLASSICAL_BITS {
        return Err(CheckpointError::LimitExceeded {
            resource: "classical_memory.bit_count",
            actual: image.bit_count as u128,
            maximum: DEFAULT_MAX_CLASSICAL_BITS as u128,
        });
    }

    if image.payload.len() > DEFAULT_MAX_CLASSICAL_MEMORY_BYTES {
        return Err(CheckpointError::LimitExceeded {
            resource: "classical_memory.bytes",
            actual: image.payload.len() as u128,
            maximum: DEFAULT_MAX_CLASSICAL_MEMORY_BYTES as u128,
        });
    }

    validate_optional_identifier(
        "classical_memory.format_id",
        image.format_id.as_deref(),
    )?;

    Ok(())
}

fn validate_rng_state(
    rng: &RngState,
) -> CheckpointResult<()> {
    validate_identifier(
        "rng_algorithm",
        &rng.algorithm,
    )?;

    if rng.state.len() > DEFAULT_MAX_RNG_STATE_BYTES {
        return Err(CheckpointError::LimitExceeded {
            resource: "rng_state_bytes",
            actual: rng.state.len() as u128,
            maximum: DEFAULT_MAX_RNG_STATE_BYTES as u128,
        });
    }

    Ok(())
}

fn validate_optional_provider_state(
    state: Option<&ProviderStateReference>,
) -> CheckpointResult<()> {
    let Some(state) = state else {
        return Ok(());
    };

    validate_identifier(
        "provider_state.provider_id",
        &state.provider_id,
    )?;

    validate_identifier(
        "provider_state.state_kind",
        &state.state_kind,
    )?;

    validate_optional_identifier(
        "provider_state.backend_id",
        state.backend_id.as_deref(),
    )?;

    validate_optional_identifier(
        "provider_state.schema_version",
        state.schema_version.as_deref(),
    )?;

    if state.payload.len() > DEFAULT_MAX_PROVIDER_STATE_BYTES {
        return Err(CheckpointError::LimitExceeded {
            resource: "provider_state_bytes",
            actual: state.payload.len() as u128,
            maximum: DEFAULT_MAX_PROVIDER_STATE_BYTES as u128,
        });
    }

    Ok(())
}

fn validate_restore_capability_consistency(
    payload: &CheckpointPayload,
) -> CheckpointResult<()> {
    match payload.restore_capability {
        RestoreCapability::FullMemory => {
            if payload.quantum_memory.is_none() {
                return Err(CheckpointError::InvalidField {
                    field: "quantum_memory",
                    reason:
                        "FullMemory requires a quantum-memory image"
                            .to_owned(),
                });
            }

            if payload.classical_memory.is_none() {
                return Err(CheckpointError::InvalidField {
                    field: "classical_memory",
                    reason:
                        "FullMemory requires classical memory"
                            .to_owned(),
                });
            }
        }

        RestoreCapability::QuantumMemory => {
            if payload.quantum_memory.is_none() {
                return Err(CheckpointError::InvalidField {
                    field: "quantum_memory",
                    reason:
                        "QuantumMemory capability requires a quantum-memory image"
                            .to_owned(),
                });
            }
        }

        RestoreCapability::BackendContinuation => {
            if payload.provider_state.is_none() {
                return Err(CheckpointError::InvalidField {
                    field: "provider_state",
                    reason:
                        "BackendContinuation requires provider state"
                            .to_owned(),
                });
            }
        }

        RestoreCapability::ClassicalOnly
        | RestoreCapability::MetadataOnly
        | RestoreCapability::None => {}
    }

    Ok(())
}

fn validate_restore_target(
    target: &RestoreTarget,
) -> CheckpointResult<()> {
    validate_optional_identifier(
        "target.provider_id",
        target.provider_id.as_deref(),
    )?;

    validate_optional_identifier(
        "target.backend_id",
        target.backend_id.as_deref(),
    )?;

    validate_optional_identifier(
        "target.device_id",
        target.device_id.as_deref(),
    )?;

    validate_optional_identifier(
        "target.technology",
        target.technology.as_deref(),
    )?;

    validate_optional_identifier(
        "target.execution_model",
        target.execution_model.as_deref(),
    )?;

    validate_optional_identifier(
        "target.hardware_revision",
        target.hardware_revision.as_deref(),
    )?;

    validate_optional_identifier(
        "target.calibration_id",
        target.calibration_id.as_deref(),
    )?;

    validate_optional_identifier(
        "target.topology_version",
        target.topology_version.as_deref(),
    )?;

    Ok(())
}

fn check_requirements(
    payload: &CheckpointPayload,
    target: &RestoreTarget,
) -> CheckpointResult<()> {
    let requirements = &payload.restore_requirements;

    if let Some(required) = &requirements.representation {
        let actual = target
            .representation
            .as_ref()
            .ok_or_else(|| CheckpointError::IncompatibleTarget {
                reason:
                    "target does not declare a state representation"
                        .to_owned(),
            })?;

        if required != actual {
            return Err(CheckpointError::IncompatibleTarget {
                reason: format!(
                    "required representation `{required}` but target provides `{actual}`"
                ),
            });
        }
    }

    if let Some(required) = &requirements.storage_location {
        let actual = target
            .storage_location
            .as_ref()
            .ok_or_else(|| CheckpointError::IncompatibleTarget {
                reason:
                    "target does not declare a storage location"
                        .to_owned(),
            })?;

        if required != actual {
            return Err(CheckpointError::IncompatibleTarget {
                reason: format!(
                    "required storage location `{required}` but target provides `{actual}`"
                ),
            });
        }
    }

    if let Some(required) = requirements.qubit_count {
        let actual = target
            .qubit_count
            .ok_or_else(|| CheckpointError::IncompatibleTarget {
                reason:
                    "target does not declare qubit capacity"
                        .to_owned(),
            })?;

        if actual < required {
            return Err(CheckpointError::IncompatibleTarget {
                reason: format!(
                    "target has {actual} qubits but {required} are required"
                ),
            });
        }
    }

    if let Some(required) = requirements.classical_bit_count {
        let actual = target
            .classical_bit_count
            .ok_or_else(|| CheckpointError::IncompatibleTarget {
                reason:
                    "target does not declare classical-memory capacity"
                        .to_owned(),
            })?;

        if actual < required {
            return Err(CheckpointError::IncompatibleTarget {
                reason: format!(
                    "target has {actual} classical bits but {required} are required"
                ),
            });
        }
    }

    check_optional_equal_identifier(
        "provider_id",
        requirements.provider_id.as_deref(),
        target.provider_id.as_deref(),
    )?;

    check_optional_equal_identifier(
        "backend_id",
        requirements.backend_id.as_deref(),
        target.backend_id.as_deref(),
    )?;

    check_optional_equal_identifier(
        "device_id",
        requirements.device_id.as_deref(),
        target.device_id.as_deref(),
    )?;

    check_optional_equal_identifier(
        "technology",
        requirements.technology.as_deref(),
        target.technology.as_deref(),
    )?;

    check_optional_equal_identifier(
        "execution_model",
        requirements.execution_model.as_deref(),
        target.execution_model.as_deref(),
    )?;

    if requirements.require_calibration_match {
        check_optional_equal_identifier(
            "calibration_id",
            payload.hardware.calibration_id.as_deref(),
            target.calibration_id.as_deref(),
        )?;
    }

    if requirements.require_hardware_revision_match {
        check_optional_equal_identifier(
            "hardware_revision",
            payload.hardware.hardware_revision.as_deref(),
            target.hardware_revision.as_deref(),
        )?;
    }

    if requirements.require_topology_match {
        check_optional_equal_identifier(
            "topology_version",
            payload.hardware.topology_version.as_deref(),
            target.topology_version.as_deref(),
        )?;
    }

    Ok(())
}

fn check_optional_equal_identifier(
    field: &'static str,
    required: Option<&str>,
    actual: Option<&str>,
) -> CheckpointResult<()> {
    let Some(required) = required else {
        return Ok(());
    };

    let actual = actual.ok_or_else(|| CheckpointError::IncompatibleTarget {
        reason: format!(
            "target does not provide required `{field}`"
        ),
    })?;

    if required != actual {
        return Err(CheckpointError::IncompatibleTarget {
            reason: format!(
                "`{field}` mismatch: required `{required}`, got `{actual}`"
            ),
        });
    }

    Ok(())
}

/// Reject obvious credential/secret fields.
///
/// This is intentionally conservative. A false positive should cause the
/// caller to rename the metadata key rather than risk persisting credentials.
fn reject_secret_like_value(
    field: &str,
    value: &str,
) -> CheckpointResult<()> {
    let normalized = field.to_ascii_lowercase();

    const FORBIDDEN_FIELD_TERMS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "auth_token",
        "bearer_token",
        "password",
        "passwd",
        "private_key",
        "secret_key",
        "client_secret",
        "authorization",
        "cookie",
    ];

    if FORBIDDEN_FIELD_TERMS
        .iter()
        .any(|term| normalized.contains(term))
    {
        return Err(CheckpointError::SecretMaterialRejected {
            field: field.to_owned(),
        });
    }

    let trimmed = value.trim();

    // Reject common bearer-token material even when a caller gives it an
    // innocuous metadata key.
    if trimmed.len() >= 20
        && (trimmed.starts_with("Bearer ")
            || trimmed.starts_with("sk-")
            || trimmed.starts_with("ghp_")
            || trimmed.starts_with("github_pat_"))
    {
        return Err(CheckpointError::SecretMaterialRejected {
            field: field.to_owned(),
        });
    }

    Ok(())
}

/// Serializes the payload in a deterministic struct-field order.
///
/// `BTreeMap` is used for free-form attributes, so map ordering is stable.
fn canonical_payload_bytes(
    payload: &CheckpointPayload,
) -> CheckpointResult<Vec<u8>> {
    serde_json::to_vec(payload)
        .map_err(|error| CheckpointError::Serialization(error.to_string()))
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Creates a checkpoint from a payload.
pub fn create_checkpoint(
    payload: CheckpointPayload,
) -> CheckpointResult<QuantumCheckpoint> {
    QuantumCheckpoint::new(payload)
}

/// Serializes a checkpoint.
pub fn serialize_checkpoint(
    checkpoint: &QuantumCheckpoint,
) -> CheckpointResult<Vec<u8>> {
    checkpoint.to_bytes()
}

/// Deserializes and validates a checkpoint.
pub fn deserialize_checkpoint(
    bytes: &[u8],
) -> CheckpointResult<QuantumCheckpoint> {
    QuantumCheckpoint::from_bytes(bytes)
}

/// Verifies a serialized checkpoint without exposing its payload to callers.
pub fn verify_checkpoint_bytes(
    bytes: &[u8],
) -> CheckpointResult<()> {
    QuantumCheckpoint::from_bytes(bytes).map(|_| ())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_workload() -> WorkloadIdentity {
        WorkloadIdentity::new("test-workload")
            .expect("valid workload")
    }

    fn basic_cursor() -> ExecutionCursor {
        ExecutionCursor::new("test-workload", 12)
            .expect("valid cursor")
    }

    fn state_image() -> QuantumMemoryImage {
        QuantumMemoryImage::new(
            RepresentationId::new("state_vector")
                .expect("valid representation"),
            StorageLocationId::new("host")
                .expect("valid location"),
            NumericPrecision::F64,
            3,
            Some("little_endian_contiguous".to_owned()),
            vec![0, 1, 2, 3, 4, 5],
        )
        .expect("valid state image")
    }

    fn classical_image() -> ClassicalMemoryImage {
        ClassicalMemoryImage::new(
            8,
            vec![0; 1],
            Some("zamani.classical.v1".to_owned()),
        )
        .expect("valid classical image")
    }

    fn full_checkpoint() -> QuantumCheckpoint {
        CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .quantum_memory(state_image())
            .classical_memory(classical_image())
            .restore_capability(RestoreCapability::FullMemory)
            .build()
            .expect("valid checkpoint")
    }

    #[test]
    fn current_format_is_compatible() {
        assert!(
            CheckpointFormatVersion::CURRENT
                .is_compatible_with_current()
        );
    }

    #[test]
    fn creates_full_checkpoint() {
        let checkpoint = full_checkpoint();

        assert_eq!(
            checkpoint.restore_capability(),
            RestoreCapability::FullMemory
        );

        assert!(checkpoint.contains_quantum_memory());
        assert!(checkpoint.contains_classical_memory());
    }

    #[test]
    fn round_trip_preserves_checkpoint() {
        let checkpoint = full_checkpoint();

        let bytes = checkpoint
            .to_bytes()
            .expect("serialization must succeed");

        let restored =
            QuantumCheckpoint::from_bytes(&bytes)
                .expect("deserialization must succeed");

        assert_eq!(checkpoint, restored);
    }

    #[test]
    fn integrity_detects_modified_payload() {
        let mut checkpoint = full_checkpoint();

        checkpoint
            .payload
            .cursor
            .operation_index = 13;

        let result = checkpoint.validate();

        assert!(matches!(
            result,
            Err(CheckpointError::IntegrityMismatch { .. })
        ));
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let mut checkpoint = full_checkpoint();

        checkpoint.magic = "BAD!".to_owned();

        let result = checkpoint.validate();

        assert!(matches!(
            result,
            Err(CheckpointError::InvalidMagic { .. })
        ));
    }

    #[test]
    fn full_memory_requires_quantum_memory() {
        let payload = CheckpointPayload {
            metadata: CheckpointMetadata::default(),
            workload: basic_workload(),
            cursor: basic_cursor(),
            quantum_memory: None,
            classical_memory: Some(classical_image()),
            rng_state: None,
            hardware: HardwareProvenance::default(),
            restore_capability: RestoreCapability::FullMemory,
            provider_state: None,
            restore_requirements: RestoreRequirements::default(),
            execution_attributes: BTreeMap::new(),
        };

        let result = QuantumCheckpoint::new(payload);

        assert!(result.is_err());
    }

    #[test]
    fn full_memory_requires_classical_memory() {
        let payload = CheckpointPayload {
            metadata: CheckpointMetadata::default(),
            workload: basic_workload(),
            cursor: basic_cursor(),
            quantum_memory: Some(state_image()),
            classical_memory: None,
            rng_state: None,
            hardware: HardwareProvenance::default(),
            restore_capability: RestoreCapability::FullMemory,
            provider_state: None,
            restore_requirements: RestoreRequirements::default(),
            execution_attributes: BTreeMap::new(),
        };

        let result = QuantumCheckpoint::new(payload);

        assert!(result.is_err());
    }

    #[test]
    fn quantum_memory_capability_requires_quantum_image() {
        let payload = CheckpointPayload {
            metadata: CheckpointMetadata::default(),
            workload: basic_workload(),
            cursor: basic_cursor(),
            quantum_memory: None,
            classical_memory: None,
            rng_state: None,
            hardware: HardwareProvenance::default(),
            restore_capability: RestoreCapability::QuantumMemory,
            provider_state: None,
            restore_requirements: RestoreRequirements::default(),
            execution_attributes: BTreeMap::new(),
        };

        assert!(QuantumCheckpoint::new(payload).is_err());
    }

    #[test]
    fn backend_continuation_requires_provider_state() {
        let payload = CheckpointPayload {
            metadata: CheckpointMetadata::default(),
            workload: basic_workload(),
            cursor: basic_cursor(),
            quantum_memory: None,
            classical_memory: None,
            rng_state: None,
            hardware: HardwareProvenance::default(),
            restore_capability: RestoreCapability::BackendContinuation,
            provider_state: None,
            restore_requirements: RestoreRequirements::default(),
            execution_attributes: BTreeMap::new(),
        };

        assert!(QuantumCheckpoint::new(payload).is_err());
    }

    #[test]
    fn backend_continuation_can_be_created() {
        let provider_state = ProviderStateReference::new(
            "example-provider",
            "job-continuation",
            vec![1, 2, 3],
        )
        .expect("valid provider state");

        let checkpoint = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .provider_state(provider_state)
            .restore_capability(
                RestoreCapability::BackendContinuation,
            )
            .build()
            .expect("valid backend continuation");

        checkpoint
            .check_restore_mode(
                RestoreMode::RequireBackendContinuation,
            )
            .expect("continuation should be supported");
    }

    #[test]
    fn secret_metadata_is_rejected() {
        let result = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .restore_capability(RestoreCapability::MetadataOnly)
            .execution_attribute(
                "api_key",
                "must-never-be-stored",
            );

        assert!(matches!(
            result,
            Err(CheckpointError::SecretMaterialRejected { .. })
        ));
    }

    #[test]
    fn bearer_token_like_value_is_rejected() {
        let result = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .restore_capability(RestoreCapability::MetadataOnly)
            .execution_attribute(
                "provider_reference",
                "Bearer abcdefghijklmnopqrstuvwxyz",
            );

        assert!(matches!(
            result,
            Err(CheckpointError::SecretMaterialRejected { .. })
        ));
    }

    #[test]
    fn restore_requirements_are_checked() {
        let mut requirements = RestoreRequirements::default();

        requirements.representation =
            Some(RepresentationId::new("state_vector").unwrap());

        requirements.storage_location =
            Some(StorageLocationId::new("host").unwrap());

        requirements.qubit_count = Some(3);

        let checkpoint = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .quantum_memory(state_image())
            .restore_capability(RestoreCapability::QuantumMemory)
            .restore_requirements(requirements)
            .build()
            .expect("valid checkpoint");

        let target = RestoreTarget {
            representation: Some(
                RepresentationId::new("state_vector").unwrap(),
            ),
            storage_location: Some(
                StorageLocationId::new("host").unwrap(),
            ),
            qubit_count: Some(8),
            ..RestoreTarget::default()
        };

        checkpoint
            .check_target(&target)
            .expect("target should satisfy requirements");
    }

    #[test]
    fn restore_requirements_reject_wrong_representation() {
        let mut requirements = RestoreRequirements::default();

        requirements.representation =
            Some(RepresentationId::new("state_vector").unwrap());

        let checkpoint = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .quantum_memory(state_image())
            .restore_capability(RestoreCapability::QuantumMemory)
            .restore_requirements(requirements)
            .build()
            .expect("valid checkpoint");

        let target = RestoreTarget {
            representation: Some(
                RepresentationId::new("density_matrix").unwrap(),
            ),
            ..RestoreTarget::default()
        };

        assert!(checkpoint.check_target(&target).is_err());
    }

    #[test]
    fn provider_provenance_is_not_provider_specific() {
        let hardware = HardwareProvenance {
            provider_id: Some("provider-x".to_owned()),
            backend_id: Some("backend-y".to_owned()),
            device_id: Some("device-z".to_owned()),
            technology: Some("trapped_ion".to_owned()),
            execution_model: Some("gate".to_owned()),
            hardware_revision: Some("rev-4".to_owned()),
            firmware_version: Some("fw-9".to_owned()),
            instruction_set_version: Some("isa-2".to_owned()),
            adapter_version: Some("adapter-1".to_owned()),
            provider_api_version: Some("api-7".to_owned()),
            calibration_id: Some("cal-2026-08-29".to_owned()),
            calibration_timestamp_unix_ns: Some(1_000),
            topology_version: Some("topology-3".to_owned()),
            continuation_reference: Some(
                "opaque-continuation-id".to_owned(),
            ),
        };

        hardware
            .validate()
            .expect("provider-neutral provenance should validate");
    }

    #[test]
    fn rng_state_round_trip() {
        let rng = RngState::new(
            "zamani-rng-v1",
            vec![10, 20, 30, 40],
        )
        .expect("valid RNG state");

        let checkpoint = CheckpointBuilder::new()
            .workload(basic_workload())
            .cursor(basic_cursor())
            .rng_state(rng)
            .restore_capability(RestoreCapability::ClassicalOnly)
            .build()
            .expect("valid checkpoint");

        let bytes = checkpoint
            .to_bytes()
            .expect("serialize");

        let restored =
            QuantumCheckpoint::from_bytes(&bytes)
                .expect("deserialize");

        assert_eq!(
            restored.payload.rng_state
                .expect("rng state")
                .state,
            vec![10, 20, 30, 40]
        );
    }

    #[test]
    fn metadata_order_is_deterministic() {
        let mut metadata = CheckpointMetadata::default();

        metadata
            .attributes
            .insert("z".to_owned(), "3".to_owned());

        metadata
            .attributes
            .insert("a".to_owned(), "1".to_owned());

        metadata
            .attributes
            .insert("m".to_owned(), "2".to_owned());

        let payload = CheckpointPayload {
            metadata,
            workload: basic_workload(),
            cursor: basic_cursor(),
            quantum_memory: None,
            classical_memory: None,
            rng_state: None,
            hardware: HardwareProvenance::default(),
            restore_capability: RestoreCapability::MetadataOnly,
            provider_state: None,
            restore_requirements: RestoreRequirements::default(),
            execution_attributes: BTreeMap::new(),
        };

        let first =
            canonical_payload_bytes(&payload)
                .expect("serialization");

        let second =
            canonical_payload_bytes(&payload)
                .expect("serialization");

        assert_eq!(first, second);
    }

    #[test]
    fn validate_only_does_not_require_restoration_capability() {
        let checkpoint = full_checkpoint();

        checkpoint
            .check_restore_mode(RestoreMode::ValidateOnly)
            .expect("validation mode should always be accepted");
    }

    #[test]
    fn workload_digest_requires_sha256() {
        let result = WorkloadIdentity::new("workload")
            .unwrap()
            .with_digest("abc");

        assert!(result.is_err());
    }

    #[test]
    fn cursor_branch_is_supported() {
        let cursor = ExecutionCursor::new("dynamic", 100)
            .unwrap()
            .with_branch("branch-7")
            .unwrap();

        assert_eq!(
            cursor.branch_id.as_deref(),
            Some("branch-7")
        );
    }
}