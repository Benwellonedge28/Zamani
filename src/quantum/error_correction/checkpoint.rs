//! Production-grade checkpointing for Zamani Quantum Error Correction.
//!
//! Checkpoints are untrusted serialized execution artifacts. They must be:
//!
//! - bounded before allocation;
//! - versioned;
//! - deterministically serialized;
//! - integrity protected;
//! - associated with the exact execution context;
//! - validated before restoration;
//! - cancellation aware;
//! - resource-policy aware;
//! - atomically persisted;
//! - safe to reject when incompatible.
//!
//! Architectural position:
//!
//! ```text
//! QecConfig
//!    │
//!    ├── QecLimits ───────────────┐
//!    ├── DeterministicContext     │
//!    ├── CancellationToken        │
//!    └── CapabilitySet            │
//!                                 ▼
//!                         CheckpointPolicy
//!                                 │
//!                                 ▼
//!                    Resource / structural preflight
//!                                 │
//!                                 ▼
//!                         canonical payload
//!                                 │
//!                                 ▼
//!                         SHA-256 integrity
//!                                 │
//!                                 ▼
//!                         bounded envelope
//!                                 │
//!                                 ▼
//!                         atomic persistence
//! ```
//!
//! Restore path:
//!
//! ```text
//! Untrusted bytes/path
//!       │
//!       ▼
//! bounded size check
//!       │
//!       ▼
//! fixed binary envelope
//!       │
//!       ▼
//! declared-length validation
//!       │
//!       ▼
//! integrity verification
//!       │
//!       ▼
//! canonical payload decoding
//!       │
//!       ▼
//! schema/API validation
//!       │
//!       ▼
//! execution-context validation
//!       │
//!       ▼
//! domain-state validation
//!       │
//!       ▼
//! trusted resumable state
//! ```
//!
//! Integrity is not authenticity. SHA-256 detects corruption and accidental
//! modification. Authenticity against an active attacker requires a higher
//! level signature/MAC mechanism and must never be inferred from this module.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult};
use super::limits::QecLimits;

// ============================================================================
// Format
// ============================================================================

/// Stable checkpoint magic.
const MAGIC: &[u8; 8] = b"ZMQECCHK";

/// Current binary envelope format.
///
/// Version 2 intentionally replaces the previous JSON envelope because an
/// untrusted JSON document can force allocations before the envelope's
/// declared payload size has been safely validated.
pub const CHECKPOINT_FORMAT_VERSION: u16 = 2;

/// Current logical checkpoint schema.
pub const CHECKPOINT_SCHEMA_VERSION: u16 = 2;

/// SHA-256 digest size.
pub const SHA256_BYTES: usize = 32;

/// SHA-256 hexadecimal representation length.
pub const SHA256_HEX_BYTES: usize = 64;

/// Defensive path-length limit.
pub const DEFAULT_MAX_PATH_BYTES: usize = 4096;

/// Maximum algorithm identifier.
pub const MAX_ALGORITHM_NAME_BYTES: usize = 256;

/// Maximum algorithm version.
pub const MAX_ALGORITHM_VERSION_BYTES: usize = 128;

/// Maximum configuration identifier.
pub const MAX_CONFIGURATION_ID_BYTES: usize = 512;

/// Maximum API version.
pub const MAX_API_VERSION_BYTES: usize = 128;

/// Maximum backend identity.
pub const MAX_BACKEND_ID_BYTES: usize = 256;

/// Maximum decoder identity.
pub const MAX_DECODER_ID_BYTES: usize = 256;

/// Maximum execution fingerprint.
pub const MAX_FINGERPRINT_BYTES: usize = 256;

/// Fixed binary header size.
///
/// ```text
/// magic             8
/// format            2
/// schema            2
/// payload length    8
/// payload SHA-256  32
/// ------------------
/// total             52
/// ```
pub const HEADER_BYTES: usize = 52;

// ============================================================================
// Errors
// ============================================================================

/// Checkpoint-specific error.
///
/// Public QEC boundaries can convert this into `QecError`. Keeping the
/// specialized type internally preserves precise diagnostics.
#[derive(Debug)]
pub enum CheckpointError {
    InvalidInput(String),

    ResourceLimitExceeded {
        resource: &'static str,
        requested: u64,
        limit: u64,
    },

    Malformed(String),

    IntegrityMismatch {
        expected: String,
        actual: String,
    },

    UnsupportedFormatVersion {
        found: u16,
        supported: u16,
    },

    UnsupportedSchemaVersion {
        found: u16,
        supported: u16,
    },

    IncompatibleApiVersion {
        checkpoint: String,
        runtime: String,
    },

    AlgorithmMismatch {
        checkpoint: String,
        expected: String,
    },

    ConfigurationMismatch {
        checkpoint: String,
        expected: String,
    },

    CodeMismatch {
        checkpoint: String,
        expected: String,
    },

    BackendMismatch {
        checkpoint: String,
        expected: String,
    },

    DecoderMismatch {
        checkpoint: String,
        expected: String,
    },

    DeterminismMismatch {
        checkpoint: String,
        expected: String,
    },

    ResourcePolicyMismatch {
        checkpoint: String,
        expected: String,
    },

    InvalidState(String),

    CancellationRequested,

    FilesystemDisabled,

    Io(io::Error),

    Serialization(String),

    Deserialization(String),

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
                    "checkpoint resource limit exceeded: \
                     {resource}, requested={requested}, limit={limit}"
                )
            }

            Self::Malformed(message) => {
                write!(f, "malformed checkpoint: {message}")
            }

            Self::IntegrityMismatch { expected, actual } => {
                write!(
                    f,
                    "checkpoint integrity mismatch: \
                     expected={expected}, actual={actual}"
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
                    "incompatible QEC API version: \
                     checkpoint={checkpoint}, runtime={runtime}"
                )
            }

            Self::AlgorithmMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint algorithm mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::ConfigurationMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint configuration mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::CodeMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint code identity mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::BackendMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint backend mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::DecoderMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint decoder mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::DeterminismMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint determinism policy mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::ResourcePolicyMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "checkpoint resource policy mismatch: \
                     checkpoint={checkpoint}, expected={expected}"
                )
            }

            Self::InvalidState(message) => {
                write!(f, "invalid checkpoint state: {message}")
            }

            Self::CancellationRequested => {
                write!(f, "checkpoint operation cancelled")
            }

            Self::FilesystemDisabled => {
                write!(f, "filesystem checkpoint persistence is disabled")
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

impl From<CheckpointError> for QecError {
    fn from(error: CheckpointError) -> Self {
        match error {
            CheckpointError::ResourceLimitExceeded {
                resource,
                requested,
                limit,
            } => QecError::resource_limit(
                super::errors::ResourceKind::CheckpointSize,
                requested as u128,
                0,
                limit as u128,
                format!("checkpoint {resource} limit exceeded"),
            ),

            CheckpointError::CancellationRequested => {
                QecError::cancelled("checkpoint operation cancelled")
            }

            CheckpointError::Io(error) => {
                QecError::invalid_input(format!(
                    "checkpoint I/O failure: {error}"
                ))
            }

            CheckpointError::InvalidInput(message)
            | CheckpointError::Malformed(message)
            | CheckpointError::InvalidState(message)
            | CheckpointError::Serialization(message)
            | CheckpointError::Deserialization(message)
            | CheckpointError::Time(message) => {
                QecError::invalid_input(message)
            }

            CheckpointError::IntegrityMismatch {
                expected,
                actual,
            } => QecError::invalid_input(format!(
                "checkpoint integrity mismatch: expected={expected}, actual={actual}"
            )),

            CheckpointError::UnsupportedFormatVersion {
                found,
                supported,
            } => QecError::unsupported(
                "checkpoint_format",
                format!(
                    "checkpoint format {found} is unsupported; \
                     supported={supported}"
                ),
            ),

            CheckpointError::UnsupportedSchemaVersion {
                found,
                supported,
            } => QecError::unsupported(
                "checkpoint_schema",
                format!(
                    "checkpoint schema {found} is unsupported; \
                     supported={supported}"
                ),
            ),

            CheckpointError::IncompatibleApiVersion {
                checkpoint,
                runtime,
            } => QecError::unsupported(
                "checkpoint_api_version",
                format!(
                    "checkpoint={checkpoint}, runtime={runtime}"
                ),
            ),

            CheckpointError::AlgorithmMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::ConfigurationMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::CodeMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::BackendMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::DecoderMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::DeterminismMismatch {
                checkpoint,
                expected,
            }
            | CheckpointError::ResourcePolicyMismatch {
                checkpoint,
                expected,
            } => QecError::unsupported(
                "checkpoint_resume",
                format!(
                    "checkpoint={checkpoint}; expected={expected}"
                ),
            ),

            CheckpointError::FilesystemDisabled => {
                QecError::unsupported(
                    "checkpoint_filesystem",
                    "filesystem checkpoint persistence is disabled",
                )
            }
        }
    }
}

/// Specialized checkpoint result.
pub type CheckpointResult<T> = Result<T, CheckpointError>;

// ============================================================================
// Policy
// ============================================================================

/// Checkpoint policy.
///
/// Resource ceilings come from the canonical `QecLimits`. This structure
/// contains only checkpoint-specific policy that cannot belong in QecLimits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointPolicy {
    /// Canonical QEC resource policy.
    pub limits: QecLimits,

    /// Maximum path length.
    pub max_path_bytes: usize,

    /// Permit filesystem persistence.
    pub allow_filesystem: bool,
}

impl Default for CheckpointPolicy {
    fn default() -> Self {
        Self {
            limits: QecLimits::default(),
            max_path_bytes: DEFAULT_MAX_PATH_BYTES,
            allow_filesystem: true,
        }
    }
}

impl CheckpointPolicy {
    /// Creates a checkpoint policy from the canonical QEC resource policy.
    pub fn from_limits(limits: QecLimits) -> CheckpointResult<Self> {
        limits
            .validate()
            .map_err(|error| {
                CheckpointError::InvalidInput(error.to_string())
            })?;

        Ok(Self {
            limits,
            ..Self::default()
        })
    }

    /// Creates a policy with explicit filesystem behavior.
    pub fn with_filesystem(
        limits: QecLimits,
        allow_filesystem: bool,
    ) -> CheckpointResult<Self> {
        let mut policy = Self::from_limits(limits)?;
        policy.allow_filesystem = allow_filesystem;
        Ok(policy)
    }

    fn validate(&self) -> CheckpointResult<()> {
        self.limits
            .validate()
            .map_err(|error| {
                CheckpointError::InvalidInput(error.to_string())
            })?;

        if self.max_path_bytes == 0 {
            return Err(CheckpointError::InvalidInput(
                "max_path_bytes must be non-zero".into(),
            ));
        }

        Ok(())
    }

    fn max_checkpoint_bytes(&self) -> u64 {
        self.limits.max_checkpoint_size_bytes
    }
}

// ============================================================================
// Identity
// ============================================================================

/// Stable algorithm identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlgorithmIdentity {
    pub name: String,
    pub version: String,
}

impl AlgorithmIdentity {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let identity = Self {
            name: name.into(),
            version: version.into(),
        };

        validate_string(
            "algorithm name",
            &identity.name,
            MAX_ALGORITHM_NAME_BYTES,
        )?;

        validate_string(
            "algorithm version",
            &identity.version,
            MAX_ALGORITHM_VERSION_BYTES,
        )?;

        Ok(identity)
    }
}

/// Configuration identity.
///
/// The identifier should normally be a hash of canonical `QecConfig`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigurationIdentity {
    pub id: String,
}

impl ConfigurationIdentity {
    pub fn new(id: impl Into<String>) -> CheckpointResult<Self> {
        let identity = Self { id: id.into() };

        validate_string(
            "configuration id",
            &identity.id,
            MAX_CONFIGURATION_ID_BYTES,
        )?;

        Ok(identity)
    }
}

/// Execution identity.
///
/// These fields prevent accidentally resuming a checkpoint with a different
/// code, backend, decoder, determinism policy, or resource policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionIdentity {
    /// Hash/identity of the code/topology.
    pub code_hash: String,

    /// Physical or simulated backend identity.
    pub backend_id: String,

    /// Decoder identity.
    pub decoder_id: String,

    /// Determinism policy fingerprint.
    pub determinism_fingerprint: String,

    /// Resource-policy fingerprint.
    pub resource_policy_fingerprint: String,
}

impl Default for ExecutionIdentity {
    fn default() -> Self {
        Self {
            code_hash: "unspecified".into(),
            backend_id: "unspecified".into(),
            decoder_id: "unspecified".into(),
            determinism_fingerprint: "unspecified".into(),
            resource_policy_fingerprint: "unspecified".into(),
        }
    }
}

impl ExecutionIdentity {
    pub fn validate(&self) -> CheckpointResult<()> {
        validate_string(
            "code hash",
            &self.code_hash,
            MAX_FINGERPRINT_BYTES,
        )?;

        validate_string(
            "backend id",
            &self.backend_id,
            MAX_BACKEND_ID_BYTES,
        )?;

        validate_string(
            "decoder id",
            &self.decoder_id,
            MAX_DECODER_ID_BYTES,
        )?;

        validate_string(
            "determinism fingerprint",
            &self.determinism_fingerprint,
            MAX_FINGERPRINT_BYTES,
        )?;

        validate_string(
            "resource policy fingerprint",
            &self.resource_policy_fingerprint,
            MAX_FINGERPRINT_BYTES,
        )?;

        Ok(())
    }
}

// ============================================================================
// Resume state
// ============================================================================

/// Resumable execution position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumePosition {
    pub round: u64,
    pub events_processed: u64,
    pub decoder_iterations: u64,
    pub partition_id: Option<u64>,
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

/// Observed resource usage at checkpoint time.
///
/// This is diagnostic state, not authorization. The live `ResourceManager`
/// remains authoritative.
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
                "peak memory cannot be smaller than allocated memory"
                    .into(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// State
// ============================================================================

/// Serializable resumable QEC state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointState {
    pub qec_api_version: String,
    pub format_version: u16,
    pub schema_version: u16,

    pub algorithm: AlgorithmIdentity,
    pub configuration: ConfigurationIdentity,
    pub execution: ExecutionIdentity,

    pub seed: Option<u64>,
    pub position: ResumePosition,
    pub resources: CheckpointResourceUsage,

    /// Opaque deterministic decoder/stream/partition state.
    pub state: Vec<u8>,

    /// Optional non-sensitive metadata.
    pub metadata: Vec<u8>,
}

impl CheckpointState {
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
            execution: ExecutionIdentity::default(),
            seed,
            position,
            resources,
            state,
            metadata,
        };

        checkpoint.validate(&CheckpointPolicy::default())?;
        Ok(checkpoint)
    }

    /// Attaches the execution identity after construction.
    pub fn with_execution_identity(
        mut self,
        execution: ExecutionIdentity,
    ) -> CheckpointResult<Self> {
        execution.validate()?;
        self.execution = execution;
        Ok(self)
    }

    pub fn validate(
        &self,
        policy: &CheckpointPolicy,
    ) -> CheckpointResult<()> {
        policy.validate()?;

        validate_string(
            "QEC API version",
            &self.qec_api_version,
            MAX_API_VERSION_BYTES,
        )?;

        if self.format_version != CHECKPOINT_FORMAT_VERSION {
            return Err(
                CheckpointError::UnsupportedFormatVersion {
                    found: self.format_version,
                    supported: CHECKPOINT_FORMAT_VERSION,
                },
            );
        }

        if self.schema_version != CHECKPOINT_SCHEMA_VERSION {
            return Err(
                CheckpointError::UnsupportedSchemaVersion {
                    found: self.schema_version,
                    supported: CHECKPOINT_SCHEMA_VERSION,
                },
            );
        }

        self.algorithm.validate()?;

        validate_string(
            "configuration id",
            &self.configuration.id,
            MAX_CONFIGURATION_ID_BYTES,
        )?;

        self.execution.validate()?;
        self.position.validate()?;
        self.resources.validate()?;

        let state_len = checked_len(self.state.len())?;

        if state_len > policy.max_checkpoint_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "state bytes",
                    requested: state_len,
                    limit: policy.max_checkpoint_bytes(),
                },
            );
        }

        let metadata_len = checked_len(self.metadata.len())?;

        if metadata_len > policy.max_checkpoint_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "metadata bytes",
                    requested: metadata_len,
                    limit: policy.max_checkpoint_bytes(),
                },
            );
        }

        Ok(())
    }

    /// Canonical payload.
    ///
    /// The state is serialized only after validation.
    pub fn canonical_bytes(
        &self,
        policy: &CheckpointPolicy,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Vec<u8>> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        self.validate(policy)?;

        let payload = serde_json::to_vec(self)
            .map_err(|error| {
                CheckpointError::Serialization(error.to_string())
            })?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let length = checked_len(payload.len())?;

        if length > policy.max_checkpoint_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "canonical payload",
                    requested: length,
                    limit: policy.max_checkpoint_bytes(),
                },
            );
        }

        Ok(payload)
    }
}

// ============================================================================
// Binary envelope
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct CheckpointEnvelope {
    payload_length: u64,
    digest: [u8; SHA256_BYTES],
    payload: Vec<u8>,
}

impl CheckpointEnvelope {
    fn from_state(
        state: &CheckpointState,
        policy: &CheckpointPolicy,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Self> {
        let payload =
            state.canonical_bytes(policy, cancellation)?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let digest = sha256_digest(&payload, cancellation)?;

        Ok(Self {
            payload_length: checked_len(payload.len())?,
            digest,
            payload,
        })
    }

    fn encoded_len(&self) -> CheckpointResult<u64> {
        let payload = self.payload_length;

        HEADER_BYTES
            .checked_add(
                usize::try_from(payload)
                    .map_err(|_| {
                        CheckpointError::ResourceLimitExceeded {
                            resource: "payload length",
                            requested: payload,
                            limit: u64::MAX,
                        }
                    })?,
            )
            .and_then(|length| u64::try_from(length).ok())
            .ok_or_else(|| CheckpointError::Malformed(
                "checkpoint encoded length overflow".into(),
            ))
    }

    fn encode(
        &self,
        policy: &CheckpointPolicy,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Vec<u8>> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let encoded_len = self.encoded_len()?;

        if encoded_len > policy.max_checkpoint_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "encoded checkpoint",
                    requested: encoded_len,
                    limit: policy.max_checkpoint_bytes(),
                },
            );
        }

        let capacity = usize::try_from(encoded_len)
            .map_err(|_| CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint allocation",
                requested: encoded_len,
                limit: usize::MAX as u64,
            })?;

        let mut output = Vec::with_capacity(capacity);

        output.extend_from_slice(MAGIC);
        output.extend_from_slice(
            &CHECKPOINT_FORMAT_VERSION.to_le_bytes(),
        );
        output.extend_from_slice(
            &CHECKPOINT_SCHEMA_VERSION.to_le_bytes(),
        );
        output.extend_from_slice(
            &self.payload_length.to_le_bytes(),
        );
        output.extend_from_slice(&self.digest);
        output.extend_from_slice(&self.payload);

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        Ok(output)
    }

    fn parse(
        bytes: &[u8],
        policy: &CheckpointPolicy,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Self> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        enforce_encoded_size(bytes.len(), policy)?;

        if bytes.len() < HEADER_BYTES {
            return Err(CheckpointError::Malformed(
                "checkpoint is shorter than fixed header".into(),
            ));
        }

        if &bytes[..MAGIC.len()] != MAGIC {
            return Err(CheckpointError::Malformed(
                "invalid checkpoint magic".into(),
            ));
        }

        let format =
            u16::from_le_bytes([bytes[8], bytes[9]]);

        if format != CHECKPOINT_FORMAT_VERSION {
            return Err(
                CheckpointError::UnsupportedFormatVersion {
                    found: format,
                    supported: CHECKPOINT_FORMAT_VERSION,
                },
            );
        }

        let schema =
            u16::from_le_bytes([bytes[10], bytes[11]]);

        if schema != CHECKPOINT_SCHEMA_VERSION {
            return Err(
                CheckpointError::UnsupportedSchemaVersion {
                    found: schema,
                    supported: CHECKPOINT_SCHEMA_VERSION,
                },
            );
        }

        let payload_length = u64::from_le_bytes([
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
            bytes[16],
            bytes[17],
            bytes[18],
            bytes[19],
        ]);

        if payload_length > policy.max_checkpoint_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint payload",
                    requested: payload_length,
                    limit: policy.max_checkpoint_bytes(),
                },
            );
        }

        let expected_total =
            HEADER_BYTES
                .checked_add(
                    usize::try_from(payload_length)
                        .map_err(|_| {
                            CheckpointError::ResourceLimitExceeded {
                                resource: "checkpoint payload",
                                requested: payload_length,
                                limit: usize::MAX as u64,
                            }
                        })?,
                )
                .ok_or_else(|| {
                    CheckpointError::Malformed(
                        "checkpoint length overflow".into(),
                    )
                })?;

        if expected_total != bytes.len() {
            return Err(CheckpointError::Malformed(format!(
                "checkpoint length mismatch: \
                 declared={payload_length}, actual={}",
                bytes.len().saturating_sub(HEADER_BYTES)
            )));
        }

        let mut expected_digest = [0u8; SHA256_BYTES];
        expected_digest.copy_from_slice(
            &bytes[20..20 + SHA256_BYTES],
        );

        let payload = bytes[HEADER_BYTES..].to_vec();

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let actual_digest =
            sha256_digest(&payload, cancellation)?;

        if actual_digest != expected_digest {
            return Err(
                CheckpointError::IntegrityMismatch {
                    expected: hex_digest(&expected_digest),
                    actual: hex_digest(&actual_digest),
                },
            );
        }

        Ok(Self {
            payload_length,
            digest: expected_digest,
            payload,
        })
    }

    fn restore(
        self,
        policy: &CheckpointPolicy,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<CheckpointState> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let state: CheckpointState =
            serde_json::from_slice(&self.payload)
                .map_err(|error| {
                    CheckpointError::Deserialization(
                        error.to_string(),
                    )
                })?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        state.validate(policy)?;

        Ok(state)
    }
}

// ============================================================================
// Public serialization API
// ============================================================================

/// Encodes a checkpoint using a fresh cancellation token.
pub fn encode(
    state: &CheckpointState,
    policy: &CheckpointPolicy,
) -> CheckpointResult<Vec<u8>> {
    let token = CancellationToken::new();

    encode_with_cancellation(state, policy, &token)
}

/// Encodes a checkpoint while honoring cancellation.
pub fn encode_with_cancellation(
    state: &CheckpointState,
    policy: &CheckpointPolicy,
    cancellation: &CancellationToken,
) -> CheckpointResult<Vec<u8>> {
    let envelope =
        CheckpointEnvelope::from_state(
            state,
            policy,
            cancellation,
        )?;

    envelope.encode(policy, cancellation)
}

/// Decodes a checkpoint using a fresh cancellation token.
pub fn decode(
    bytes: &[u8],
    policy: &CheckpointPolicy,
) -> CheckpointResult<CheckpointState> {
    let token = CancellationToken::new();

    decode_with_cancellation(bytes, policy, &token)
}

/// Decodes a checkpoint safely while honoring cancellation.
///
/// Crucially, the fixed-size binary header is inspected before the payload is
/// allocated or deserialized.
pub fn decode_with_cancellation(
    bytes: &[u8],
    policy: &CheckpointPolicy,
    cancellation: &CancellationToken,
) -> CheckpointResult<CheckpointState> {
    let envelope =
        CheckpointEnvelope::parse(
            bytes,
            policy,
            cancellation,
        )?;

    envelope.restore(policy, cancellation)
}

// ============================================================================
// Integrity
// ============================================================================

/// SHA-256 digest.
pub fn sha256_digest(
    data: &[u8],
    cancellation: &CancellationToken,
) -> CheckpointResult<[u8; SHA256_BYTES]> {
    const CHUNK: usize = 1024 * 1024;

    let mut hasher = Sha256::new();

    for chunk in data.chunks(CHUNK) {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        hasher.update(chunk);
    }

    let result = hasher.finalize();

    let mut digest = [0u8; SHA256_BYTES];
    digest.copy_from_slice(&result);

    Ok(digest)
}

/// Convenience SHA-256 hexadecimal digest.
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    hex_digest(&digest)
}

fn hex_digest(data: &[u8]) -> String {
    let mut output =
        String::with_capacity(data.len() * 2);

    for byte in data {
        use std::fmt::Write;

        let _ = write!(&mut output, "{byte:02x}");
    }

    output
}

// ============================================================================
// Resume compatibility
// ============================================================================

/// Complete execution compatibility validation.
pub fn validate_resume_compatibility(
    state: &CheckpointState,
    runtime_qec_api_version: &str,
    expected_algorithm: &AlgorithmIdentity,
    expected_configuration: &ConfigurationIdentity,
    expected_execution: &ExecutionIdentity,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let token = CancellationToken::new();

    validate_resume_compatibility_with_cancellation(
        state,
        runtime_qec_api_version,
        expected_algorithm,
        expected_configuration,
        expected_execution,
        policy,
        &token,
    )
}

/// Cancellation-aware resume validation.
pub fn validate_resume_compatibility_with_cancellation(
    state: &CheckpointState,
    runtime_qec_api_version: &str,
    expected_algorithm: &AlgorithmIdentity,
    expected_configuration: &ConfigurationIdentity,
    expected_execution: &ExecutionIdentity,
    policy: &CheckpointPolicy,
    cancellation: &CancellationToken,
) -> CheckpointResult<()> {
    cancellation
        .check()
        .map_err(|_| CheckpointError::CancellationRequested)?;

    state.validate(policy)?;

    if state.qec_api_version != runtime_qec_api_version {
        return Err(
            CheckpointError::IncompatibleApiVersion {
                checkpoint: state.qec_api_version.clone(),
                runtime: runtime_qec_api_version.to_owned(),
            },
        );
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
        return Err(
            CheckpointError::ConfigurationMismatch {
                checkpoint: state.configuration.id.clone(),
                expected: expected_configuration.id.clone(),
            },
        );
    }

    if state.execution.code_hash != expected_execution.code_hash {
        return Err(CheckpointError::CodeMismatch {
            checkpoint: state.execution.code_hash.clone(),
            expected: expected_execution.code_hash.clone(),
        });
    }

    if state.execution.backend_id != expected_execution.backend_id {
        return Err(CheckpointError::BackendMismatch {
            checkpoint: state.execution.backend_id.clone(),
            expected: expected_execution.backend_id.clone(),
        });
    }

    if state.execution.decoder_id != expected_execution.decoder_id {
        return Err(CheckpointError::DecoderMismatch {
            checkpoint: state.execution.decoder_id.clone(),
            expected: expected_execution.decoder_id.clone(),
        });
    }

    if state.execution.determinism_fingerprint
        != expected_execution.determinism_fingerprint
    {
        return Err(
            CheckpointError::DeterminismMismatch {
                checkpoint: state
                    .execution
                    .determinism_fingerprint
                    .clone(),
                expected: expected_execution
                    .determinism_fingerprint
                    .clone(),
            },
        );
    }

    if state.execution.resource_policy_fingerprint
        != expected_execution.resource_policy_fingerprint
    {
        return Err(
            CheckpointError::ResourcePolicyMismatch {
                checkpoint: state
                    .execution
                    .resource_policy_fingerprint
                    .clone(),
                expected: expected_execution
                    .resource_policy_fingerprint
                    .clone(),
            },
        );
    }

    cancellation
        .check()
        .map_err(|_| CheckpointError::CancellationRequested)?;

    Ok(())
}

// ============================================================================
// Filesystem persistence
// ============================================================================

/// Writes a checkpoint atomically.
pub fn write_atomic(
    path: impl AsRef<Path>,
    state: &CheckpointState,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let token = CancellationToken::new();

    write_atomic_with_cancellation(
        path,
        state,
        policy,
        &token,
    )
}

/// Cancellation-aware atomic checkpoint write.
pub fn write_atomic_with_cancellation(
    path: impl AsRef<Path>,
    state: &CheckpointState,
    policy: &CheckpointPolicy,
    cancellation: &CancellationToken,
) -> CheckpointResult<()> {
    ensure_filesystem_allowed(policy)?;
    validate_path(path.as_ref(), policy)?;

    cancellation
        .check()
        .map_err(|_| CheckpointError::CancellationRequested)?;

    let encoded =
        encode_with_cancellation(
            state,
            policy,
            cancellation,
        )?;

    let path = path.as_ref();
    let parent =
        path.parent().unwrap_or_else(|| Path::new("."));

    let temporary =
        temporary_path(path)?;

    let result = write_file_atomically(
        parent,
        &temporary,
        path,
        &encoded,
        cancellation,
    );

    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }

    result
}

fn write_file_atomically(
    parent: &Path,
    temporary: &Path,
    destination: &Path,
    bytes: &[u8],
    cancellation: &CancellationToken,
) -> CheckpointResult<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temporary)?;

    for chunk in bytes.chunks(1024 * 1024) {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        file.write_all(chunk)?;
    }

    file.flush()?;
    file.sync_all()?;
    drop(file);

    cancellation
        .check()
        .map_err(|_| CheckpointError::CancellationRequested)?;

    fs::rename(temporary, destination)?;

    /*
     * Directory fsync is best effort because platform support varies.
     * The rename itself is atomic on filesystems that provide normal rename
     * semantics.
     */
    if let Ok(directory) = File::open(parent) {
        let _ = directory.sync_all();
    }

    Ok(())
}

/// Reads a checkpoint from disk.
pub fn read(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<CheckpointState> {
    let token = CancellationToken::new();

    read_with_cancellation(path, policy, &token)
}

/// Cancellation-aware checkpoint read.
pub fn read_with_cancellation(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
    cancellation: &CancellationToken,
) -> CheckpointResult<CheckpointState> {
    ensure_filesystem_allowed(policy)?;
    validate_path(path.as_ref(), policy)?;

    cancellation
        .check()
        .map_err(|_| CheckpointError::CancellationRequested)?;

    let metadata = fs::metadata(path.as_ref())?;
    let file_size = metadata.len();

    if file_size > policy.max_checkpoint_bytes() {
        return Err(
            CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint file",
                requested: file_size,
                limit: policy.max_checkpoint_bytes(),
            },
        );
    }

    let allocation_size =
        usize::try_from(file_size).map_err(|_| {
            CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint allocation",
                requested: file_size,
                limit: usize::MAX as u64,
            }
        })?;

    let mut file = File::open(path.as_ref())?;
    let mut bytes =
        Vec::with_capacity(allocation_size);

    let mut buffer = [0u8; 1024 * 1024];

    loop {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let read = file.read(&mut buffer)?;

        if read == 0 {
            break;
        }

        bytes.extend_from_slice(&buffer[..read]);

        enforce_encoded_size(bytes.len(), policy)?;
    }

    decode_with_cancellation(
        &bytes,
        policy,
        cancellation,
    )
}

/// Removes a checkpoint.
pub fn remove(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    ensure_filesystem_allowed(policy)?;
    validate_path(path.as_ref(), policy)?;

    fs::remove_file(path.as_ref())?;

    Ok(())
}

/// Returns whether a checkpoint exists.
pub fn exists(
    path: impl AsRef<Path>,
    policy: &CheckpointPolicy,
) -> CheckpointResult<bool> {
    ensure_filesystem_allowed(policy)?;
    validate_path(path.as_ref(), policy)?;

    Ok(path.as_ref().exists())
}

// ============================================================================
// Resumable state integration
// ============================================================================

/// Domain-specific resumable state.
///
/// The generic checkpoint layer never interprets decoder-specific bytes.
pub trait ResumableState: Sized {
    fn algorithm_identity(
        &self,
    ) -> CheckpointResult<AlgorithmIdentity>;

    fn configuration_identity(
        &self,
    ) -> CheckpointResult<ConfigurationIdentity>;

    fn encode_state(&self) -> CheckpointResult<Vec<u8>>;

    fn decode_state(
        bytes: &[u8],
    ) -> CheckpointResult<Self>;

    fn validate_state(&self) -> CheckpointResult<()>;
}

/// Builds a generic checkpoint from domain state.
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

    let algorithm =
        state.algorithm_identity()?;

    let configuration =
        state.configuration_identity()?;

    let encoded_state =
        state.encode_state()?;

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

/// Restores domain-specific state.
pub fn restore_state<T: ResumableState>(
    checkpoint: &CheckpointState,
    runtime_qec_api_version: &str,
    expected_algorithm: &AlgorithmIdentity,
    expected_configuration: &ConfigurationIdentity,
    expected_execution: &ExecutionIdentity,
    policy: &CheckpointPolicy,
) -> CheckpointResult<T> {
    validate_resume_compatibility(
        checkpoint,
        runtime_qec_api_version,
        expected_algorithm,
        expected_configuration,
        expected_execution,
        policy,
    )?;

    let state =
        T::decode_state(&checkpoint.state)?;

    state.validate_state()?;

    Ok(state)
}

// ============================================================================
// Helpers
// ============================================================================

fn checked_len(length: usize) -> CheckpointResult<u64> {
    u64::try_from(length).map_err(|_| {
        CheckpointError::ResourceLimitExceeded {
            resource: "checkpoint length",
            requested: u64::MAX,
            limit: u64::MAX,
        }
    })
}

fn enforce_encoded_size(
    length: usize,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let requested = checked_len(length)?;

    if requested > policy.max_checkpoint_bytes() {
        return Err(
            CheckpointError::ResourceLimitExceeded {
                resource: "encoded checkpoint",
                requested,
                limit: policy.max_checkpoint_bytes(),
            },
        );
    }

    Ok(())
}

fn validate_string(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> CheckpointResult<()> {
    if value.is_empty() {
        return Err(CheckpointError::InvalidInput(
            format!("{field} must not be empty"),
        ));
    }

    if value.len() > maximum {
        return Err(
            CheckpointError::ResourceLimitExceeded {
                resource: field,
                requested: value.len() as u64,
                limit: maximum as u64,
            },
        );
    }

    Ok(())
}

fn validate_path(
    path: &Path,
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    let bytes = path.as_os_str().to_string_lossy();

    if bytes.is_empty() {
        return Err(CheckpointError::InvalidInput(
            "checkpoint path must not be empty".into(),
        ));
    }

    let length =
        checked_len(bytes.len())?;

    let limit =
        u64::try_from(policy.max_path_bytes)
            .unwrap_or(u64::MAX);

    if length > limit {
        return Err(
            CheckpointError::ResourceLimitExceeded {
                resource: "checkpoint path",
                requested: length,
                limit,
            },
        );
    }

    Ok(())
}

fn ensure_filesystem_allowed(
    policy: &CheckpointPolicy,
) -> CheckpointResult<()> {
    policy.validate()?;

    if !policy.allow_filesystem {
        return Err(
            CheckpointError::FilesystemDisabled,
        );
    }

    Ok(())
}

fn temporary_path(
    path: &Path,
) -> CheckpointResult<PathBuf> {
    let parent =
        path.parent().unwrap_or_else(|| Path::new("."));

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            CheckpointError::InvalidInput(
                "checkpoint path has no valid filename"
                    .into(),
            )
        })?;

    let timestamp =
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                CheckpointError::Time(
                    error.to_string(),
                )
            })?;

    let nonce =
        timestamp
            .as_nanos()
            .checked_add(
                u128::from(
                    std::process::id(),
                ),
            )
            .ok_or_else(|| {
                CheckpointError::Time(
                    "temporary path nonce overflow"
                        .into(),
                )
            })?;

    Ok(parent.join(format!(
        ".{file_name}.zamani-{nonce}.tmp"
    )))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn algorithm() -> AlgorithmIdentity {
        AlgorithmIdentity::new(
            "mwpm",
            "2.0.0",
        )
        .unwrap()
    }

    fn configuration() -> ConfigurationIdentity {
        ConfigurationIdentity::new(
            "config-sha256:test",
        )
        .unwrap()
    }

    fn execution() -> ExecutionIdentity {
        ExecutionIdentity {
            code_hash: "code-sha256:test".into(),
            backend_id: "simulator:test".into(),
            decoder_id: "mwpm@2.0.0".into(),
            determinism_fingerprint:
                "deterministic:test".into(),
            resource_policy_fingerprint:
                "limits:test".into(),
        }
    }

    fn position() -> ResumePosition {
        ResumePosition {
            round: 42,
            events_processed: 1024,
            decoder_iterations: 17,
            partition_id: Some(2),
            stream_offset: Some(8192),
        }
    }

    fn resources() -> CheckpointResourceUsage {
        CheckpointResourceUsage {
            allocated_bytes: 1024,
            peak_bytes: 2048,
            cpu_time_ns: 50_000,
            wall_time_ns: 70_000,
            graph_nodes: 100,
            graph_edges: 250,
            syndrome_events: 1024,
            decoder_iterations: 17,
            parallel_workers: 4,
        }
    }

    fn state() -> CheckpointState {
        CheckpointState::new(
            "2.0.0",
            algorithm(),
            configuration(),
            Some(12345),
            position(),
            resources(),
            b"decoder-state".to_vec(),
            b"metadata".to_vec(),
        )
        .unwrap()
        .with_execution_identity(
            execution(),
        )
        .unwrap()
    }

    fn policy() -> CheckpointPolicy {
        CheckpointPolicy::default()
    }

    #[test]
    fn round_trip_is_lossless() {
        let original = state();

        let encoded =
            encode(&original, &policy())
                .unwrap();

        let restored =
            decode(&encoded, &policy())
                .unwrap();

        assert_eq!(
            original,
            restored
        );
    }

    #[test]
    fn encoding_is_deterministic() {
        let original = state();

        let first =
            encode(&original, &policy())
                .unwrap();

        let second =
            encode(&original, &policy())
                .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn header_is_binary_and_bounded() {
        let encoded =
            encode(&state(), &policy())
                .unwrap();

        assert_eq!(
            &encoded[..MAGIC.len()],
            MAGIC
        );

        assert!(
            encoded.len() >= HEADER_BYTES
        );
    }

    #[test]
    fn corruption_is_detected() {
        let mut encoded =
            encode(&state(), &policy())
                .unwrap();

        let last =
            encoded.last_mut().unwrap();

        *last ^= 0x01;

        assert!(
            decode(&encoded, &policy())
                .is_err()
        );
    }

    #[test]
    fn truncated_header_is_rejected() {
        let encoded =
            encode(&state(), &policy())
                .unwrap();

        let truncated =
            &encoded[..HEADER_BYTES - 1];

        assert!(
            decode(truncated, &policy())
                .is_err()
        );
    }

    #[test]
    fn declared_payload_length_is_checked() {
        let mut encoded =
            encode(&state(), &policy())
                .unwrap();

        encoded[12] =
            encoded[12].wrapping_add(1);

        assert!(
            decode(&encoded, &policy())
                .is_err()
        );
    }

    #[test]
    fn algorithm_mismatch_is_rejected() {
        let expected =
            AlgorithmIdentity::new(
                "union-find",
                "2.0.0",
            )
            .unwrap();

        let result =
            validate_resume_compatibility(
                &state(),
                "2.0.0",
                &expected,
                &configuration(),
                &execution(),
                &policy(),
            );

        assert!(
            matches!(
                result,
                Err(
                    CheckpointError::AlgorithmMismatch {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn code_mismatch_is_rejected() {
        let mut expected =
            execution();

        expected.code_hash =
            "different-code".into();

        let result =
            validate_resume_compatibility(
                &state(),
                "2.0.0",
                &algorithm(),
                &configuration(),
                &expected,
                &policy(),
            );

        assert!(
            matches!(
                result,
                Err(
                    CheckpointError::CodeMismatch {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn backend_mismatch_is_rejected() {
        let mut expected =
            execution();

        expected.backend_id =
            "different-backend".into();

        let result =
            validate_resume_compatibility(
                &state(),
                "2.0.0",
                &algorithm(),
                &configuration(),
                &expected,
                &policy(),
            );

        assert!(
            matches!(
                result,
                Err(
                    CheckpointError::BackendMismatch {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn resource_policy_is_canonical() {
        let limits =
            QecLimits::default();

        let policy =
            CheckpointPolicy::from_limits(
                limits,
            )
            .unwrap();

        assert_eq!(
            policy.max_checkpoint_bytes(),
            limits.max_checkpoint_size_bytes
        );
    }

    #[test]
    fn cancellation_is_honored() {
        let token =
            CancellationToken::new();

        token.request();

        let result =
            encode_with_cancellation(
                &state(),
                &policy(),
                &token,
            );

        assert!(
            matches!(
                result,
                Err(
                    CheckpointError::CancellationRequested
                )
            )
        );
    }

    #[test]
    fn oversized_checkpoint_is_rejected() {
        let mut limits =
            QecLimits::default();

        limits.max_checkpoint_size_bytes = 64;

        let policy =
            CheckpointPolicy::from_limits(
                limits,
            )
            .unwrap();

        let result =
            encode(&state(), &policy);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_resource_usage_is_rejected() {
        let mut usage =
            resources();

        usage.allocated_bytes = 200;
        usage.peak_bytes = 100;

        let result =
            CheckpointState::new(
                "2.0.0",
                algorithm(),
                configuration(),
                None,
                position(),
                usage,
                Vec::new(),
                Vec::new(),
            );

        assert!(result.is_err());
    }
}