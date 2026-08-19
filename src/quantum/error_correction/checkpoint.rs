//! Production-grade checkpointing for Zamani Quantum Error Correction.
//!
//! A checkpoint is a validated, versioned, integrity-protected snapshot of
//! resumable QEC execution state.
//!
//! Design goals:
//!
//! * deterministic serialization;
//! * explicit schema and API compatibility;
//! * bounded checkpoint size;
//! * checked arithmetic;
//! * corruption detection;
//! * validation before restore;
//! * graceful errors rather than panics;
//! * atomic filesystem persistence;
//! * explicit resource accounting;
//! * resumable decoder/streaming state;
//! * algorithm/configuration identity;
//! * forward-compatible schema handling.
//!
//! The checkpoint layer deliberately does not know how a particular decoder
//! represents its internal state. Algorithms provide an opaque deterministic
//! byte payload and metadata describing that state.
//!
//! Data flow:
//!
//! ```text
//! Decoder / Stream / Partition
//!          |
//!          v
//!   CheckpointState
//!          |
//!          v
//!   validation
//!          |
//!          v
//! deterministic serialization
//!          |
//!          v
//! integrity digest
//!          |
//!          v
//!   bounded envelope
//!          |
//!          v
//!       storage
//!
//! Restore:
//!
//! storage
//!   |
//!   v
//! size validation
//!   |
//!   v
//! envelope parsing
//!   |
//!   v
//! integrity verification
//!   |
//!   v
//! schema/API compatibility
//!   |
//!   v
//! state validation
//!   |
//!   v
//! resumable execution
//! ```
//!
//! Security rule:
//!
//! ```text
//! Untrusted checkpoint
//!        |
//!        v
//! bounded read
//!        |
//!        v
//! structural validation
//!        |
//!        v
//! integrity verification
//!        |
//!        v
//! compatibility validation
//!        |
//!        v
//! state validation
//!        |
//!        v
//! trusted resume state
//! ```
//!
//! A checksum detects accidental corruption and many classes of malformed
//! data. It is not intended to provide authenticity against an attacker.
//! Authenticity/signatures belong to a higher-level security layer.

#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Binary checkpoint format magic.
///
/// The value is deliberately short and stable so malformed files can be
/// rejected before attempting deserialization.
const MAGIC: &[u8; 8] = b"ZMQECCHK";

/// Current checkpoint envelope format.
pub const CHECKPOINT_FORMAT_VERSION: u16 = 1;

/// Current checkpoint schema version.
///
/// This is independent from [`CHECKPOINT_FORMAT_VERSION`].
pub const CHECKPOINT_SCHEMA_VERSION: u16 = 1;

/// Maximum algorithm name length accepted by the checkpoint layer.
pub const MAX_ALGORITHM_NAME_BYTES: usize = 256;

/// Maximum configuration identifier length.
pub const MAX_CONFIGURATION_ID_BYTES: usize = 512;

/// Maximum state identifier length.
pub const MAX_STATE_ID_BYTES: usize = 256;

/// Maximum metadata length.
pub const MAX_METADATA_BYTES: usize = 16 * 1024;

/// Maximum checksum length.
pub const SHA256_HEX_BYTES: usize = 64;

/// Default checkpoint size limit.
///
/// Individual deployments should normally provide a stricter limit through
/// [`CheckpointPolicy`].
pub const DEFAULT_MAX_CHECKPOINT_SIZE: u64 = 64 * 1024 * 1024;

/// Default maximum state payload.
///
/// Keeping this separately bounded prevents a malicious envelope from
/// consuming the entire configured checkpoint budget with a single field.
pub const DEFAULT_MAX_STATE_BYTES: u64 = 60 * 1024 * 1024;

/// Maximum path length accepted by the persistence layer.
///
/// This is not an OS limit; it is a defensive parser/storage policy.
pub const DEFAULT_MAX_PATH_BYTES: usize = 4096;

/// Error type returned by checkpoint operations.
///
/// The variants are intentionally structured so callers can distinguish
/// corruption, incompatibility, resource exhaustion, and I/O failures.
#[derive(Debug)]
pub enum CheckpointError {
    /// Input was structurally invalid.
    InvalidInput(String),

    /// The checkpoint exceeds a configured resource limit.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: u64,
        limit: u64,
    },

    /// The serialized checkpoint is malformed.
    Malformed(String),

    /// The checkpoint failed integrity verification.
    IntegrityMismatch {
        expected: String,
        actual: String,
    },

    /// The checkpoint format is newer than this implementation supports.
    UnsupportedFormatVersion {
        found: u16,
        supported: u16,
    },

    /// The checkpoint schema is newer than this implementation supports.
    UnsupportedSchemaVersion {
        found: u16,
        supported: u16,
    },

    /// The checkpoint was created by an incompatible QEC API.
    IncompatibleApiVersion {
        checkpoint: String,
        runtime: String,
    },

    /// The checkpoint belongs to another algorithm.
    AlgorithmMismatch {
        checkpoint: String,
        expected: String,
    },

    /// The checkpoint belongs to another configuration.
    ConfigurationMismatch {
        checkpoint: String,
        expected: String,
    },

    /// The checkpoint's resumable state is invalid.
    InvalidState(String),

    /// An I/O operation failed.
    Io(io::Error),

    /// Serialization failed.
    Serialization(String),

    /// Deserialization failed.
    Deserialization(String),

    /// System time could not be represented.
    Time(String),
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message) => {
                write!(f, "invalid checkpoint input: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => {
                write!(
                    f,
                    "checkpoint resource limit exceeded: {resource}, \
                     requested={requested}, limit={limit}"
                )
            }

            Self::Malformed(message) => {
                write!(f, "malformed checkpoint: {message}")
            }

            Self::IntegrityMismatch { expected, actual } => {
                write!(
                    f,
                    "checkpoint integrity mismatch: expected={expected}, \
                     actual={actual}"
                )
            }

            Self::UnsupportedFormatVersion { found, supported } => {
                write!(
                    f,
                    "unsupported checkpoint format version {found}; \
                     runtime supports {supported}"
                )
            }

            Self::UnsupportedSchemaVersion { found, supported } => {
                write!(
                    f,
                    "unsupported checkpoint schema version {found}; \
                     runtime supports {supported}"
                )
            }

            Self::IncompatibleApiVersion {
                checkpoint,
                runtime,
            } => {
                write!(
                    f,
                    "incompatible QEC API version: checkpoint={checkpoint}, \
                     runtime={runtime}"
                )
            }

            Self::AlgorithmMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint algorithm mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::ConfigurationMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint configuration mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::InvalidState(message) => {
                write!(f, "invalid checkpoint state: {message}")
            }

            Self::Io(error) => {
                write!(f, "checkpoint I/O error: {error}")
            }

            Self::Serialization(message) => {
                write!(f, "checkpoint serialization error: {message}")
            }

            Self::Deserialization(message) => {
                write!(f, "checkpoint deserialization error: {message}")
            }

            Self::Time(message) => {
                write!(f, "checkpoint time error: {message}")
            }
        }
    }
}

impl std::error::Error for CheckpointError {}

impl From<io::Error> for CheckpointError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// Convenient result type for checkpoint operations.
pub type CheckpointResult<T> = Result<T, CheckpointError>;

/// Resource policy for checkpoint creation and restoration.
///
/// All limits are explicit. No checkpoint operation should allocate based
/// solely on untrusted values contained in a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointPolicy {
    /// Maximum complete encoded checkpoint size.
    pub max_checkpoint_size: u64,

    /// Maximum serialized state payload.
    pub max_state_bytes: u64,

    /// Maximum metadata payload.
    pub max_metadata_bytes: u64,

    /// Maximum filesystem path length accepted.
    pub max_path_bytes: u64,

    /// Whether filesystem persistence is permitted.
    pub allow_filesystem: bool,
}

impl Default for CheckpointPolicy {
    fn default() -> Self {
        Self {
            max_checkpoint_size: DEFAULT_MAX_CHECKPOINT_SIZE,
            max_state_bytes: DEFAULT_MAX_STATE_BYTES,
            max_metadata_bytes: MAX_METADATA_BYTES as u64,
            max_path_bytes: DEFAULT_MAX_PATH_BYTES as u64,
            allow_filesystem: true,
        }
    }
}

impl CheckpointPolicy {
    /// Creates a policy after validating all limits.
    pub fn new(
        max_checkpoint_size: u64,
        max_state_bytes: u64,
        max_metadata_bytes: u64,
        max_path_bytes: u64,
        allow_filesystem: bool,
    ) -> CheckpointResult<Self> {
        if max_checkpoint_size == 0 {
            return Err(CheckpointError::InvalidInput(
                "max_checkpoint_size must be non-zero".into(),
            ));
        }

        if max_state_bytes == 0 {
            return Err(CheckpointError::InvalidInput(
                "max_state_bytes must be non-zero".into(),
            ));
        }

        if max_state_bytes > max_checkpoint_size {
            return Err(CheckpointError::InvalidInput(
                "max_state_bytes cannot exceed max_checkpoint_size".into(),
            ));
        }

        if max_metadata_bytes > max_checkpoint_size {
            return Err(CheckpointError::InvalidInput(
                "max_metadata_bytes cannot exceed max_checkpoint_size".into(),
            ));
        }

        if max_path_bytes == 0 {
            return Err(CheckpointError::InvalidInput(
                "max_path_bytes must be non-zero".into(),
            ));
        }

        Ok(Self {
            max_checkpoint_size,
            max_state_bytes,
            max_metadata_bytes,
            max_path_bytes,
            allow_filesystem,
        })
    }
}

/// Stable identity of the algorithm that produced the checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorithmIdentity {
    pub name: String,
    pub version: String,
}

impl AlgorithmIdentity {
    /// Creates a validated algorithm identity.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> CheckpointResult<Self> {
        let identity = Self {
            name: name.into(),
            version: version.into(),
        };

        identity.validate()?;
        Ok(identity)
    }

    fn validate(&self) -> CheckpointResult<()> {
        validate_bounded_string(
            "algorithm name",
            &self.name,
            MAX_ALGORITHM_NAME_BYTES,
        )?;

        validate_bounded_string(
            "algorithm version",
            &self.version,
            128,
        )?;

        Ok(())
    }
}

/// Identity of the configuration under which the checkpoint was produced.
///
/// A deterministic configuration identifier should normally be derived from
/// the canonical configuration representation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurationIdentity {
    pub id: String,
}

impl ConfigurationIdentity {
    pub fn new(id: impl Into<String>) -> CheckpointResult<Self> {
        let identity = Self { id: id.into() };

        validate_bounded_string(
            "configuration id",
            &identity.id,
            MAX_CONFIGURATION_ID_BYTES,
        )?;

        Ok(identity)
    }
}

/// Resumable execution position.
///
/// This allows streaming decoders, repeated syndrome extraction, simulations,
/// and partitioned workloads to resume without replaying the complete input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumePosition {
    /// Logical processing round.
    pub round: u64,

    /// Number of syndrome/detection events consumed.
    pub events_processed: u64,

    /// Number of decoder iterations completed.
    pub decoder_iterations: u64,

    /// Optional partition/shard identifier.
    pub partition_id: Option<u64>,

    /// Optional stream offset.
    pub stream_offset: Option<u64>,
}

impl ResumePosition {
    pub fn validate(&self) -> CheckpointResult<()> {
        if self.round == u64::MAX {
            return Err(CheckpointError::InvalidState(
                "resume round is reserved at u64::MAX".into(),
            ));
        }

        Ok(())
    }
}

/// Resource accounting captured at checkpoint time.
///
/// These values are observational; they must never be trusted as proof that
/// a resource limit was respected. Resource managers remain authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CheckpointResourceUsage {
    pub allocated_bytes: u64,
    pub peak_bytes: u64,
    pub cpu_time_ns: u64,
    pub wall_time_ns: u64,
    pub graph_nodes: u64,
    pub graph_edges: u64,
    pub syndrome_events: u64,
    pub decoder_iterations: u64,
    pub parallel_workers: u64,
}

impl CheckpointResourceUsage {
    pub fn validate(&self) -> CheckpointResult<()> {
        if self.peak_bytes < self.allocated_bytes {
            return Err(CheckpointError::InvalidState(
                "peak memory cannot be smaller than allocated memory".into(),
            ));
        }

        Ok(())
    }
}

/// Deterministic checkpoint state.
///
/// `state` is opaque to this module. The producing decoder is responsible for
/// defining its byte representation. The bytes must be deterministic for
/// deterministic execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointState {
    /// Stable QEC API version.
    pub qec_api_version: String,

    /// Checkpoint envelope format.
    pub format_version: u16,

    /// Logical schema version of the state.
    pub schema_version: u16,

    /// Algorithm identity.
    pub algorithm: AlgorithmIdentity,

    /// Configuration identity.
    pub configuration: ConfigurationIdentity,

    /// Deterministic execution seed, if applicable.
    pub seed: Option<u64>,

    /// Resumable execution position.
    pub position: ResumePosition,

    /// Resource accounting captured at checkpoint time.
    pub resources: CheckpointResourceUsage,

    /// Decoder/stream state.
    pub state: Vec<u8>,

    /// Optional application metadata.
    pub metadata: Vec<u8>,
}

impl CheckpointState {
    /// Creates a new checkpoint state.
    pub fn new(
        qec_api_version: impl Into<String>,
        algorithm: AlgorithmIdentity,
        configuration: ConfigurationIdentity,
        seed: Option<u64>,
        position: ResumePosition,
        resources: CheckpointResourceUsage,
        state: Vec<u8>,
        metadata: Vec<u8>,
    ) -> CheckpointResult<Self> {
        let checkpoint = Self {
            qec_api_version: qec_api_version.into(),
            format_version: CHECKPOINT_FORMAT_VERSION,
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            algorithm,
            configuration,
            seed,
            position,
            resources,
            state,
            metadata,
        };

        checkpoint.validate(&CheckpointPolicy::default())?;
        Ok(checkpoint)
    }

    /// Validates the checkpoint without trusting any encoded data.
    pub fn validate(&self, policy: &CheckpointPolicy) -> CheckpointResult<()> {
        validate_bounded_string(
            "QEC API version",
            &self.qec_api_version,
            128,
        )?;

        if self.format_version != CHECKPOINT_FORMAT_VERSION {
            return Err(CheckpointError::UnsupportedFormatVersion {
                found: self.format_version,
                supported: CHECKPOINT_FORMAT_VERSION,
            });
        }

        if self.schema_version != CHECKPOINT_SCHEMA_VERSION {
            return Err(CheckpointError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: CHECKPOINT_SCHEMA_VERSION,
            });
        }

        self.algorithm.validate()?;

        self.configuration.validate()?;

        self.position.validate()?;

        self.resources.validate()?;

        let state_len = u64::try_from(self.state.len()).map_err(|_| {
            CheckpointError::ResourceLimitExceeded {
                resource: "state bytes",
                requested: u64::MAX,
                limit: policy.max_state_bytes,
            }
        })?;

        if state_len > policy.max_state_bytes {
            return Err(CheckpointError::ResourceLimitExceeded {
                resource: "state bytes",
                requested: state_len,
                limit: policy.max_state_bytes,
            });
        }

        let metadata_len = u64::try_from(self.metadata.len()).map_err(|_| {
            CheckpointError::ResourceLimitExceeded {
                resource: "metadata bytes",
                requested: u64::MAX,
                limit: policy.max_metadata_bytes,
            }
        })?;

        if metadata_len > policy.max_metadata_bytes {
            return Err(CheckpointError::ResourceLimitExceeded {
                resource: "metadata bytes",
                requested: metadata_len,
                limit: policy.max_metadata_bytes,
            });
        }

        Ok(())
    }

    /// Returns a canonical deterministic representation.
    ///
    /// `serde_json` is used only after the state has been validated.
    pub fn canonical_bytes(&self, policy: &CheckpointPolicy) -> CheckpointResult<Vec<u8>> {
        self.validate(policy)?;

        serde_json::to_vec(self)
            .map_err(|error| CheckpointError::Serialization(error.to_string()))
    }
}

/// On-disk checkpoint envelope.
///
/// The checksum is calculated over the canonical serialized `CheckpointState`,
/// not over this envelope itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CheckpointEnvelope {
    magic: [u8; 8],
    format_version: u16,
    schema_version: u16,
    payload_length: u64,
    payload_sha256: String,
    payload: Vec<u8>,
}

impl CheckpointEnvelope {
    fn from_state(
        state: &CheckpointState,
        policy: &CheckpointPolicy,
    ) -> CheckpointResult<Self> {
        let payload = state.canonical_bytes(policy)?;

        let payload_length = u64::try_from(payload.len()).map_err(|_| {
            CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint payload",
                requested: u64::MAX,
                limit: policy.max_checkpoint_size,
            }
        })?;

        let digest = sha256_hex(&payload);

        let envelope = Self {
            magic: *MAGIC,
            format_version: CHECKPOINT_FORMAT_VERSION,
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            payload_length,
            payload_sha256: digest,
            payload,
        };

        envelope.validate(policy)?;

        Ok(envelope)
    }

    fn validate(&self, policy: &CheckpointPolicy) -> CheckpointResult<()> {
        if self.magic != *MAGIC {
            return Err(CheckpointError::Malformed(
                "invalid checkpoint magic".into(),
            ));
        }

        if self.format_version != CHECKPOINT_FORMAT_VERSION {
            return Err(CheckpointError::UnsupportedFormatVersion {
                found: self.format_version,
                supported: CHECKPOINT_FORMAT_VERSION,
            });
        }

        if self.schema_version != CHECKPOINT_SCHEMA_VERSION {
            return Err(CheckpointError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: CHECKPOINT_SCHEMA_VERSION,
            });
        }

        let actual_length = u64::try_from(self.payload.len()).map_err(|_| {
            CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint payload",
                requested: u64::MAX,
                limit: policy.max_checkpoint_size,
            }
        })?;

        if actual_length != self.payload_length {
            return Err(CheckpointError::Malformed(format!(
                "payload length mismatch: declared={}, actual={}",
                self.payload_length, actual_length
            )));
        }

        if actual_length > policy.max_state_bytes {
            return Err(CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint payload",
                requested: actual_length,
                limit: policy.max_state_bytes,
            });
        }

        if self.payload_sha256.len() != SHA256_HEX_BYTES {
            return Err(CheckpointError::Malformed(
                "invalid SHA-256 digest length".into(),
            ));
        }

        if !self
            .payload_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(CheckpointError::Malformed(
                "checkpoint digest is not hexadecimal".into(),
            ));
        }

        let actual_digest = sha256_hex(&self.payload);

        if actual_digest != self.payload_sha256 {
            return Err(CheckpointError::IntegrityMismatch {
                expected: self.payload_sha256.clone(),
                actual: actual_digest,
            });
        }

        Ok(())
    }

    fn restore(
        self,
        policy: &CheckpointPolicy,
    ) -> CheckpointResult<CheckpointState> {
        self.validate(policy)?;

        let state: CheckpointState = serde_json::from_slice(&self.payload)
            .map_err(|error| CheckpointError::Deserialization(error.to_string()))?;

        state.validate(policy)?;

        Ok(state)
    }
}

/// Validates and serializes a checkpoint deterministically.
pub fn encode(
    state: &CheckpointState,
    policy: &CheckpointPolicy,
) -> CheckpointResult<Vec<u8>> {
    state.validate(policy)?;

    let envelope = CheckpointEnvelope::from_state(state, policy)?;

    let encoded = serde_json::to_vec(&envelope)
        .map_err(|error| CheckpointError::Serialization(error.to_string()))?;

    enforce_encoded_size(encoded.len(), policy)?;

    Ok(encoded)
}

/// Decodes, validates, and integrity-checks a checkpoint.
///
/// No untrusted payload is deserialized into a large state structure until
/// its envelope has first passed bounded structural checks.
pub fn decode(
    bytes: &[u8],
    policy: &CheckpointPolicy,
) -> CheckpointResult<CheckpointState> {
    enforce_encoded_size(bytes.len(), policy)?;

    if bytes.len() < MAGIC.len() {
        return Err(CheckpointError::Malformed(
            "checkpoint is shorter than its magic header".into(),
        ));
    }

    let envelope: CheckpointEnvelope =
        serde_json::from_slice(bytes)
            .map_err(|error| CheckpointError::Deserialization(error.to_string()))?;

    envelope.restore(policy)
}

/// Calculates the SHA-256 digest used for integrity validation.
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);

    let mut output = String::with_capacity(SHA256_HEX_BYTES);

    for byte in digest {
        use std::fmt::Write;

        // Writing into a String cannot fail.
        let _ = write!(&mut output, "{byte:02x}");
    }

    output
}

/// Validates that a checkpoint can resume under the current runtime.
pub fn validate_resume_compatibility(
    state: &CheckpointState,
    runtime_qec_api_version: &str,
    expected_algorithm: &AlgorithmIdentity,
    expected_configuration: &ConfigurationIdentity,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    state.validate(policy)?;

    if state.qec_api_version != runtime_qec_api_version {
        return Err(CheckpointError::IncompatibleApiVersion {
            checkpoint: state.qec_api_version.clone(),
            runtime: runtime_qec_api_version.to_owned(),
        });
    }

    if state.algorithm != *expected_algorithm {
        return Err(CheckpointError::AlgorithmMismatch {
            checkpoint: format!(
                "{}@{}",
                state.algorithm.name,
                state.algorithm.version
            ),
            expected: format!(
                "{}@{}",
                expected_algorithm.name,
                expected_algorithm.version
            ),
        });
    }

    if state.configuration != *expected_configuration {
        return Err(CheckpointError::ConfigurationMismatch {
            checkpoint: state.configuration.id.clone(),
            expected: expected_configuration.id.clone(),
        });
    }

    Ok(())
}

/// Writes a checkpoint atomically.
///
/// The checkpoint is first written to a uniquely named temporary file,
/// flushed, synced, and then renamed into place.
///
/// This prevents a partially written checkpoint from replacing a valid
/// checkpoint.
pub fn write_atomic(
    path: impl AsRef<Path>,
    state: &CheckpointState,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    if !policy.allow_filesystem {
        return Err(CheckpointError::InvalidInput(
            "filesystem checkpoint persistence is disabled by policy".into(),
        ));
    }

    let path = path.as_ref();

    validate_path(path, policy)?;

    let encoded = encode(state, policy)?;

    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    let temp_path = temporary_path(path)?;

    let result = write_atomic_inner(
        parent,
        &temp_path,
        path,
        &encoded,
    );

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

fn write_atomic_inner(
    parent: &Path,
    temp_path: &Path,
    destination: &Path,
    encoded: &[u8],
) -> CheckpointResult<()> {
    let mut file = File::create(temp_path)?;

    file.write_all(encoded)?;
    file.flush()?;
    file.sync_all()?;

    drop(file);

    fs::rename(temp_path, destination)?;

    // Best-effort directory synchronization. Not all platforms permit
    // opening directories this way, so a failure here is not promoted to a
    // checkpoint corruption error after the atomic rename succeeded.
    if let Ok(directory) = File::open(parent) {
        let _ = directory.sync_all();
    }

    Ok(())
}

/// Reads and validates a checkpoint from disk.
pub fn read(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<CheckpointState> {
    if !policy.allow_filesystem {
        return Err(CheckpointError::InvalidInput(
            "filesystem checkpoint persistence is disabled by policy".into(),
        ));
    }

    let path = path.as_ref();

    validate_path(path, policy)?;

    let metadata = fs::metadata(path)?;

    let file_size = metadata.len();

    if file_size > policy.max_checkpoint_size {
        return Err(CheckpointError::ResourceLimitExceeded {
            resource: "checkpoint file",
            requested: file_size,
            limit: policy.max_checkpoint_size,
        });
    }

    let allocation_size = usize::try_from(file_size).map_err(|_| {
        CheckpointError::ResourceLimitExceeded {
            resource: "checkpoint allocation",
            requested: file_size,
            limit: usize::MAX as u64,
        }
    })?;

    let mut file = File::open(path)?;

    let mut bytes = Vec::with_capacity(allocation_size);

    file.read_to_end(&mut bytes)?;

    // Check the actual size as well as metadata. This protects against a
    // file changing between metadata() and read().
    enforce_encoded_size(bytes.len(), policy)?;

    decode(&bytes, policy)
}

/// Removes a checkpoint safely.
///
/// This operation is explicit and never follows symlinks through custom
/// filesystem traversal logic.
pub fn remove(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    if !policy.allow_filesystem {
        return Err(CheckpointError::InvalidInput(
            "filesystem checkpoint persistence is disabled by policy".into(),
        ));
    }

    let path = path.as_ref();

    validate_path(path, policy)?;

    fs::remove_file(path)?;

    Ok(())
}

/// Returns whether a checkpoint file exists.
pub fn exists(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<bool> {
    if !policy.allow_filesystem {
        return Err(CheckpointError::InvalidInput(
            "filesystem checkpoint persistence is disabled by policy".into(),
        ));
    }

    let path = path.as_ref();

    validate_path(path, policy)?;

    Ok(path.exists())
}

/// Validates a resumable state before it is handed back to a decoder.
///
/// This is deliberately separate from serialization validation so decoder
/// implementations can perform their own domain-specific state checks.
pub trait ResumableState: Sized {
    /// Stable algorithm identifier.
    fn algorithm_identity(&self) -> CheckpointResult<AlgorithmIdentity>;

    /// Stable configuration identifier.
    fn configuration_identity(&self) -> CheckpointResult<ConfigurationIdentity>;

    /// Converts state into deterministic bytes.
    fn encode_state(&self) -> CheckpointResult<Vec<u8>>;

    /// Restores state from bytes.
    fn decode_state(bytes: &[u8]) -> CheckpointResult<Self>;

    /// Performs domain-specific invariant validation.
    fn validate_state(&self) -> CheckpointResult<()>;
}

/// Builds a checkpoint from a domain-specific resumable state.
pub fn create_from_state<T: ResumableState>(
    state: &T,
    qec_api_version: &str,
    seed: Option<u64>,
    position: ResumePosition,
    resources: CheckpointResourceUsage,
    metadata: Vec<u8>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<CheckpointState> {
    state.validate_state()?;

    let algorithm = state.algorithm_identity()?;
    let configuration = state.configuration_identity()?;
    let encoded_state = state.encode_state()?;

    let checkpoint = CheckpointState::new(
        qec_api_version,
        algorithm,
        configuration,
        seed,
        position,
        resources,
        encoded_state,
        metadata,
    )?;

    checkpoint.validate(policy)?;

    Ok(checkpoint)
}

/// Restores a domain-specific state after all generic checkpoint checks have
/// passed.
pub fn restore_state<T: ResumableState>(
    checkpoint: &CheckpointState,
    runtime_qec_api_version: &str,
    expected_algorithm: &AlgorithmIdentity,
    expected_configuration: &ConfigurationIdentity,
    policy: &CheckpointPolicy,
) -> CheckpointResult<T> {
    validate_resume_compatibility(
        checkpoint,
        runtime_qec_api_version,
        expected_algorithm,
        expected_configuration,
        policy,
    )?;

    let state = T::decode_state(&checkpoint.state)?;

    state.validate_state()?;

    Ok(state)
}

fn validate_bounded_string(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> CheckpointResult<()> {
    if value.is_empty() {
        return Err(CheckpointError::InvalidInput(format!(
            "{field} must not be empty"
        )));
    }

    if value.len() > max_bytes {
        return Err(CheckpointError::ResourceLimitExceeded {
            resource: field,
            requested: value.len() as u64,
            limit: max_bytes as u64,
        });
    }

    Ok(())
}

fn enforce_encoded_size(
    length: usize,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let requested =
        u64::try_from(length).unwrap_or(u64::MAX);

    if requested > policy.max_checkpoint_size {
        return Err(CheckpointError::ResourceLimitExceeded {
            resource: "encoded checkpoint",
            requested,
            limit: policy.max_checkpoint_size,
        });
    }

    Ok(())
}

fn validate_path(
    path: &Path,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let path_string = path.to_string_lossy();

    if path_string.is_empty() {
        return Err(CheckpointError::InvalidInput(
            "checkpoint path must not be empty".into(),
        ));
    }

    if path_string.len() > policy.max_path_bytes as usize {
        return Err(CheckpointError::ResourceLimitExceeded {
            resource: "checkpoint path",
            requested: path_string.len() as u64,
            limit: policy.max_path_bytes,
        });
    }

    Ok(())
}

fn temporary_path(path: &Path) -> CheckpointResult<PathBuf> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            CheckpointError::InvalidInput(
                "checkpoint path has no valid UTF-8 filename".into(),
            )
        })?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            CheckpointError::Time(error.to_string())
        })?;

    let nonce = timestamp.as_nanos();

    let temporary_name =
        format!(".{file_name}.zamani-checkpoint-{nonce}.tmp");

    Ok(parent.join(temporary_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_algorithm() -> AlgorithmIdentity {
        AlgorithmIdentity::new("mwpm", "1.0.0")
            .expect("valid algorithm")
    }

    fn test_configuration() -> ConfigurationIdentity {
        ConfigurationIdentity::new("config-sha256:test")
            .expect("valid configuration")
    }

    fn test_position() -> ResumePosition {
        ResumePosition {
            round: 42,
            events_processed: 1_024,
            decoder_iterations: 17,
            partition_id: Some(2),
            stream_offset: Some(8_192),
        }
    }

    fn test_resources() -> CheckpointResourceUsage {
        CheckpointResourceUsage {
            allocated_bytes: 1_024,
            peak_bytes: 2_048,
            cpu_time_ns: 50_000,
            wall_time_ns: 70_000,
            graph_nodes: 100,
            graph_edges: 250,
            syndrome_events: 1_024,
            decoder_iterations: 17,
            parallel_workers: 4,
        }
    }

    fn test_state() -> CheckpointState {
        CheckpointState::new(
            "1.0.0",
            test_algorithm(),
            test_configuration(),
            Some(12345),
            test_position(),
            test_resources(),
            b"deterministic-decoder-state".to_vec(),
            b"test-metadata".to_vec(),
        )
        .expect("valid checkpoint")
    }

    #[test]
    fn checkpoint_round_trip_is_lossless() {
        let policy = CheckpointPolicy::default();
        let original = test_state();

        let encoded =
            encode(&original, &policy)
                .expect("encode");

        let restored =
            decode(&encoded, &policy)
                .expect("decode");

        assert_eq!(original, restored);
    }

    #[test]
    fn encoding_is_deterministic() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let first =
            encode(&state, &policy)
                .expect("first encode");

        let second =
            encode(&state, &policy)
                .expect("second encode");

        assert_eq!(first, second);
    }

    #[test]
    fn digest_is_deterministic() {
        let data = b"zamani-qec-checkpoint";

        assert_eq!(
            sha256_hex(data),
            sha256_hex(data)
        );
    }

    #[test]
    fn corruption_is_detected() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let mut encoded =
            encode(&state, &policy)
                .expect("encode");

        let last =
            encoded
                .last_mut()
                .expect("non-empty");

        *last ^= 0x01;

        let result =
            decode(&encoded, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn state_limit_is_enforced() {
        let policy =
            CheckpointPolicy::new(
                1024,
                8,
                64,
                4096,
                true,
            )
            .expect("valid policy");

        let result =
            CheckpointState::new(
                "1.0.0",
                test_algorithm(),
                test_configuration(),
                None,
                test_position(),
                test_resources(),
                vec![0u8; 9],
                Vec::new(),
            )
            .and_then(|state| state.validate(&policy));

        assert!(matches!(
            result,
            Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "state bytes",
                    ..
                }
            )
        ));
    }

    #[test]
    fn metadata_limit_is_enforced() {
        let policy =
            CheckpointPolicy::new(
                4096,
                1024,
                8,
                4096,
                true,
            )
            .expect("valid policy");

        let result =
            CheckpointState::new(
                "1.0.0",
                test_algorithm(),
                test_configuration(),
                None,
                test_position(),
                test_resources(),
                Vec::new(),
                vec![0u8; 9],
            )
            .and_then(|state| state.validate(&policy));

        assert!(matches!(
            result,
            Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "metadata bytes",
                    ..
                }
            )
        ));
    }

    #[test]
    fn api_version_mismatch_is_rejected() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let result =
            validate_resume_compatibility(
                &state,
                "2.0.0",
                &test_algorithm(),
                &test_configuration(),
                &policy,
            );

        assert!(matches!(
            result,
            Err(CheckpointError::IncompatibleApiVersion { .. })
        ));
    }

    #[test]
    fn algorithm_mismatch_is_rejected() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let expected =
            AlgorithmIdentity::new("union-find", "1.0.0")
                .expect("valid algorithm");

        let result =
            validate_resume_compatibility(
                &state,
                "1.0.0",
                &expected,
                &test_configuration(),
                &policy,
            );

        assert!(matches!(
            result,
            Err(CheckpointError::AlgorithmMismatch { .. })
        ));
    }

    #[test]
    fn configuration_mismatch_is_rejected() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let expected =
            ConfigurationIdentity::new("config-sha256:other")
                .expect("valid configuration");

        let result =
            validate_resume_compatibility(
                &state,
                "1.0.0",
                &test_algorithm(),
                &expected,
                &policy,
            );

        assert!(matches!(
            result,
            Err(CheckpointError::ConfigurationMismatch { .. })
        ));
    }

    #[test]
    fn invalid_resource_usage_is_rejected() {
        let mut resources =
            test_resources();

        resources.peak_bytes = 100;
        resources.allocated_bytes = 200;

        let result =
            CheckpointState::new(
                "1.0.0",
                test_algorithm(),
                test_configuration(),
                None,
                test_position(),
                resources,
                Vec::new(),
                Vec::new(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn malformed_magic_is_rejected() {
        let policy = CheckpointPolicy::default();

        let malformed =
            br#"{"magic":[1,2,3,4,5,6,7,8],"format_version":1,"schema_version":1,"payload_length":0,"payload_sha256":"","payload":[]}"#;

        let result =
            decode(malformed, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn filesystem_round_trip() {
        let policy = CheckpointPolicy::default();
        let state = test_state();

        let directory =
            std::env::temp_dir();

        let path =
            directory.join(format!(
                "zamani-qec-checkpoint-test-{}.chk",
                std::process::id()
            ));

        write_atomic(
            &path,
            &state,
            &policy,
        )
        .expect("write checkpoint");

        let restored =
            read(&path, &policy)
                .expect("read checkpoint");

        assert_eq!(state, restored);

        let _ =
            remove(&path, &policy);
    }

    #[test]
    fn filesystem_can_be_disabled() {
        let policy =
            CheckpointPolicy::new(
                4096,
                1024,
                1024,
                4096,
                false,
            )
            .expect("valid policy");

        let state = test_state();

        let path =
            std::env::temp_dir()
                .join("zamani-disabled-checkpoint.chk");

        let result =
            write_atomic(
                path,
                &state,
                &policy,
            );

        assert!(result.is_err());
    }

    #[test]
    fn resume_position_is_preserved() {
        let state = test_state();

        assert_eq!(
            state.position.round,
            42
        );

        assert_eq!(
            state.position.events_processed,
            1_024
        );

        assert_eq!(
            state.position.decoder_iterations,
            17
        );

        assert_eq!(
            state.position.partition_id,
            Some(2)
        );

        assert_eq!(
            state.position.stream_offset,
            Some(8_192)
        );
    }
}