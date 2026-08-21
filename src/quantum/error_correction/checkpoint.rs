//! Zamani Quantum Error Correction — Production Checkpointing
//!
//! # Responsibility
//!
//! `checkpoint.rs` owns durable, resumable execution state for the QEC
//! subsystem.
//!
//! A checkpoint is an UNTRUSTED artifact until every validation stage has
//! completed.
//!
//! The checkpoint lifecycle is:
//!
//! ```text
//! execution
//!     |
//!     v
//! validate state
//!     |
//!     v
//! resource preflight
//!     |
//!     v
//! deterministic canonical payload
//!     |
//!     v
//! SHA-256 integrity digest
//!     |
//!     v
//! bounded binary envelope
//!     |
//!     v
//! atomic persistence
//!
//! Restore:
//!
//! untrusted bytes/path
//!     |
//!     v
//! bounded input check
//!     |
//!     v
//! envelope validation
//!     |
//!     v
//! declared-length validation
//!     |
//!     v
//! integrity verification
//!     |
//!     v
//! bounded payload deserialization
//!     |
//!     v
//! version validation
//!     |
//!     v
//! execution-context validation
//!     |
//!     v
//! state validation
//!     |
//!     v
//! trusted checkpoint
//! ```
//!
//! # Architectural ownership
//!
//! `limits.rs`
//!     owns canonical resource ceilings.
//!
//! `resources.rs`
//!     owns runtime resource accounting.
//!
//! `memory.rs`
//!     owns allocation reservations.
//!
//! `cancellation.rs`
//!     owns cancellation.
//!
//! `version.rs`
//!     owns version compatibility.
//!
//! `configuration.rs`
//!     owns complete QEC configuration.
//!
//! `scheduler.rs`
//!     owns execution lifecycle.
//!
//! `checkpoint.rs`
//!     owns persistence and restoration of resumable execution state.
//!
//! `cache.rs`
//!     owns reusable computation, not execution state.
//!
//! `replay.rs`
//!     owns deterministic replay packages.
//!
//! # Security model
//!
//! SHA-256 provides integrity/corruption detection.
//!
//! SHA-256 DOES NOT provide authenticity against an attacker who can replace
//! the entire checkpoint and its digest.
//!
//! Authenticity must be provided by a higher-level authenticated storage or
//! signing layer.
//!
//! This module therefore never claims that a valid digest means that a
//! checkpoint is trusted.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! # Dependencies
//!
//! Expected crate dependencies already used by the QEC subsystem:
//!
//! - serde
//! - serde_json
//! - sha2
//!
//! No unsafe code is required.

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
use super::version::{
    ArtifactKind,
    Version,
    CURRENT_CHECKPOINT_VERSION,
    CURRENT_QEC_VERSION,
};

// ============================================================================
// Format constants
// ============================================================================

/// Eight-byte checkpoint magic.
pub const CHECKPOINT_MAGIC: &[u8; 8] = b"ZMQECCHK";

/// Binary envelope format version.
///
/// This is deliberately separate from the semantic checkpoint schema version.
pub const CHECKPOINT_FORMAT_VERSION: u16 = 3;

/// Current logical checkpoint schema.
pub const CHECKPOINT_SCHEMA_VERSION: u16 = 3;

/// SHA-256 digest length.
pub const SHA256_BYTES: usize = 32;

/// Maximum path length accepted by filesystem operations.
pub const DEFAULT_MAX_PATH_BYTES: usize = 4096;

/// Maximum algorithm identifier.
pub const MAX_ALGORITHM_NAME_BYTES: usize = 256;

/// Maximum algorithm-version identifier.
pub const MAX_ALGORITHM_VERSION_BYTES: usize = 128;

/// Maximum configuration identity.
pub const MAX_CONFIGURATION_ID_BYTES: usize = 512;

/// Maximum API version string.
pub const MAX_API_VERSION_BYTES: usize = 64;

/// Maximum backend identity.
pub const MAX_BACKEND_ID_BYTES: usize = 256;

/// Maximum decoder identity.
pub const MAX_DECODER_ID_BYTES: usize = 256;

/// Maximum code identity.
pub const MAX_CODE_HASH_BYTES: usize = 256;

/// Maximum determinism fingerprint.
pub const MAX_DETERMINISM_FINGERPRINT_BYTES: usize = 256;

/// Maximum resource-policy fingerprint.
pub const MAX_RESOURCE_POLICY_FINGERPRINT_BYTES: usize = 256;

/// Maximum state metadata.
pub const MAX_METADATA_BYTES: usize = 64 * 1024;

/// Fixed envelope header.
///
/// Layout:
///
/// ```text
/// magic             8
/// format_version    2
/// schema_version    2
/// payload_length    8
/// payload_sha256   32
/// ------------------
/// total             52
/// ```
pub const CHECKPOINT_HEADER_BYTES: usize = 52;

// ============================================================================
// Error model
// ============================================================================

/// Checkpoint-specific error.
///
/// Public callers may convert this to `QecError`.
#[derive(Debug)]
pub enum CheckpointError {
    InvalidInput(String),

    Malformed(String),

    InvalidState(String),

    ResourceLimitExceeded {
        resource: &'static str,
        requested: u64,
        maximum: u64,
    },

    IntegrityMismatch {
        expected: [u8; SHA256_BYTES],
        actual: [u8; SHA256_BYTES],
    },

    UnsupportedFormatVersion {
        found: u16,
        supported: u16,
    },

    UnsupportedSchemaVersion {
        found: u16,
        supported: u16,
    },

    VersionMismatch {
        artifact: String,
        expected: String,
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

    TargetMismatch {
        checkpoint: String,
        expected: String,
    },

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

            Self::Malformed(message) => {
                write!(f, "malformed checkpoint: {message}")
            }

            Self::InvalidState(message) => {
                write!(f, "invalid checkpoint state: {message}")
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "checkpoint resource limit exceeded: \
                     {resource}, requested={requested}, maximum={maximum}"
                )
            }

            Self::IntegrityMismatch { .. } => {
                write!(f, "checkpoint integrity verification failed")
            }

            Self::UnsupportedFormatVersion { found, supported } => {
                write!(
                    f,
                    "unsupported checkpoint format version {found}; \
                     supported={supported}"
                )
            }

            Self::UnsupportedSchemaVersion { found, supported } => {
                write!(
                    f,
                    "unsupported checkpoint schema version {found}; \
                     supported={supported}"
                )
            }

            Self::VersionMismatch { artifact, expected } => {
                write!(
                    f,
                    "checkpoint version mismatch: \
                     checkpoint={artifact}, expected={expected}"
                )
            }

            Self::AlgorithmMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "algorithm mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::ConfigurationMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "configuration mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::CodeMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "code identity mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::BackendMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "backend mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::DecoderMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "decoder mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::DeterminismMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "determinism policy mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::ResourcePolicyMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "resource policy mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::TargetMismatch {
                checkpoint,
                expected,
            } => {
                write!(
                    f,
                    "execution target mismatch: checkpoint={checkpoint}, \
                     expected={expected}"
                )
            }

            Self::CancellationRequested => {
                write!(f, "checkpoint operation cancelled")
            }

            Self::FilesystemDisabled => {
                write!(f, "checkpoint filesystem persistence is disabled")
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

/// Specialized checkpoint result.
pub type CheckpointResult<T> = Result<T, CheckpointError>;

// ============================================================================
// Canonical QEC error integration
// ============================================================================

impl From<CheckpointError> for QecError {
    fn from(error: CheckpointError) -> Self {
        match error {
            CheckpointError::CancellationRequested => {
                QecError::cancelled("checkpoint operation cancelled")
            }

            CheckpointError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => QecError::invalid_input(format!(
                "checkpoint resource limit exceeded: \
                 {resource}, requested={requested}, maximum={maximum}"
            )),

            CheckpointError::Io(error) => QecError::invalid_input(format!(
                "checkpoint I/O failure: {error}"
            )),

            CheckpointError::IntegrityMismatch { .. } => {
                QecError::invalid_input(
                    "checkpoint integrity verification failed",
                )
            }

            CheckpointError::UnsupportedFormatVersion {
                found,
                supported,
            } => QecError::unsupported(
                "checkpoint_format",
                format!(
                    "found={found}, supported={supported}"
                ),
            ),

            CheckpointError::UnsupportedSchemaVersion {
                found,
                supported,
            } => QecError::unsupported(
                "checkpoint_schema",
                format!(
                    "found={found}, supported={supported}"
                ),
            ),

            CheckpointError::VersionMismatch {
                artifact,
                expected,
            } => QecError::unsupported(
                "checkpoint_version",
                format!(
                    "checkpoint={artifact}, expected={expected}"
                ),
            ),

            CheckpointError::FilesystemDisabled => {
                QecError::unsupported(
                    "checkpoint_filesystem",
                    "filesystem persistence disabled",
                )
            }

            CheckpointError::InvalidInput(message)
            | CheckpointError::Malformed(message)
            | CheckpointError::InvalidState(message)
            | CheckpointError::Serialization(message)
            | CheckpointError::Deserialization(message)
            | CheckpointError::Time(message) => {
                QecError::invalid_input(message)
            }

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
            }
            | CheckpointError::TargetMismatch {
                checkpoint,
                expected,
            } => QecError::unsupported(
                "checkpoint_resume",
                format!(
                    "checkpoint={checkpoint}, expected={expected}"
                ),
            ),
        }
    }
}

// ============================================================================
// Policy
// ============================================================================

/// Checkpoint-specific policy.
///
/// `QecLimits` remains the authoritative resource policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointPolicy {
    pub limits: QecLimits,

    pub max_path_bytes: usize,

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
    pub fn new(
        limits: QecLimits,
    ) -> CheckpointResult<Self> {
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

    pub fn filesystem_disabled(
        limits: QecLimits,
    ) -> CheckpointResult<Self> {
        let mut policy = Self::new(limits)?;
        policy.allow_filesystem = false;
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
                "max_path_bytes must be greater than zero"
                    .into(),
            ));
        }

        Ok(())
    }

    fn max_bytes(&self) -> u64 {
        self.limits.max_checkpoint_size_bytes
    }
}

// ============================================================================
// Execution identity
// ============================================================================

/// Identity of the algorithm that produced the checkpoint.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct AlgorithmIdentity {
    pub name: String,
    pub version: String,
}

impl AlgorithmIdentity {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> CheckpointResult<Self> {
        let value = Self {
            name: name.into(),
            version: version.into(),
        };

        validate_string(
            "algorithm name",
            &value.name,
            MAX_ALGORITHM_NAME_BYTES,
        )?;

        validate_string(
            "algorithm version",
            &value.version,
            MAX_ALGORITHM_VERSION_BYTES,
        )?;

        Ok(value)
    }
}

/// Canonical configuration identity.
///
/// This should normally be a hash of the canonical validated `QecConfig`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ConfigurationIdentity {
    pub id: String,
}

impl ConfigurationIdentity {
    pub fn new(id: impl Into<String>) -> CheckpointResult<Self> {
        let value = Self { id: id.into() };

        validate_string(
            "configuration identity",
            &value.id,
            MAX_CONFIGURATION_ID_BYTES,
        )?;

        Ok(value)
    }
}

/// Identity of the execution environment.
///
/// These values are checked before restoration.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ExecutionIdentity {
    /// Hash or canonical identity of the code/topology.
    pub code_hash: String,

    /// Backend identity.
    pub backend_id: String,

    /// Decoder identity.
    pub decoder_id: String,

    /// Determinism-policy fingerprint.
    pub determinism_fingerprint: String,

    /// Resource-policy fingerprint.
    pub resource_policy_fingerprint: String,

    /// Execution target, for example CPU, simulator, GPU or QPU.
    pub execution_target: String,
}

impl Default for ExecutionIdentity {
    fn default() -> Self {
        Self {
            code_hash: "unspecified".into(),
            backend_id: "unspecified".into(),
            decoder_id: "unspecified".into(),
            determinism_fingerprint: "unspecified".into(),
            resource_policy_fingerprint: "unspecified".into(),
            execution_target: "unspecified".into(),
        }
    }
}

impl ExecutionIdentity {
    pub fn validate(&self) -> CheckpointResult<()> {
        validate_string(
            "code hash",
            &self.code_hash,
            MAX_CODE_HASH_BYTES,
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
            MAX_DETERMINISM_FINGERPRINT_BYTES,
        )?;

        validate_string(
            "resource policy fingerprint",
            &self.resource_policy_fingerprint,
            MAX_RESOURCE_POLICY_FINGERPRINT_BYTES,
        )?;

        validate_string(
            "execution target",
            &self.execution_target,
            128,
        )?;

        Ok(())
    }
}

// ============================================================================
// Resume position
// ============================================================================

/// Position from which execution can resume.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ResumePosition {
    pub round: u64,

    pub events_processed: u64,

    pub decoder_iterations: u64,

    pub partition_id: Option<u64>,

    pub stream_offset: Option<u64>,
}

impl Default for ResumePosition {
    fn default() -> Self {
        Self {
            round: 0,
            events_processed: 0,
            decoder_iterations: 0,
            partition_id: None,
            stream_offset: None,
        }
    }
}

impl ResumePosition {
    pub fn validate(&self) -> CheckpointResult<()> {
        if self.round == u64::MAX {
            return Err(CheckpointError::InvalidState(
                "round value u64::MAX is reserved".into(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Runtime resource snapshot
// ============================================================================

/// Runtime resource information captured in a checkpoint.
///
/// This is observational data only. It does not replace `ResourceManager`.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
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
                "peak_bytes cannot be smaller than allocated_bytes"
                    .into(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint state
// ============================================================================

/// Serializable resumable execution state.
///
/// The domain-specific modules serialize their state into the bounded fields
/// here. This prevents checkpoint.rs from becoming coupled to a particular
/// decoder implementation.
///
/// `pauli_frame` and `logical_state` are intentionally byte-oriented so that
/// `checkpoint.rs` does not own Pauli algebra or logical-equivalence logic.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct CheckpointState {
    /// Current execution position.
    pub position: ResumePosition,

    /// Deterministic seed.
    pub seed: u64,

    /// Serialized Pauli-frame state.
    pub pauli_frame: Vec<u8>,

    /// Serialized logical-state information.
    pub logical_state: Vec<u8>,

    /// Serialized decoder state.
    pub decoder_state: Vec<u8>,

    /// Serialized stream/partition state.
    pub execution_state: Vec<u8>,

    /// Optional application metadata.
    pub metadata: Vec<u8>,
}

impl Default for CheckpointState {
    fn default() -> Self {
        Self {
            position: ResumePosition::default(),
            seed: 0,
            pauli_frame: Vec::new(),
            logical_state: Vec::new(),
            decoder_state: Vec::new(),
            execution_state: Vec::new(),
            metadata: Vec::new(),
        }
    }
}

impl CheckpointState {
    /// Validates state independent of resource limits.
    pub fn validate(&self) -> CheckpointResult<()> {
        self.position.validate()?;

        if self.metadata.len() > MAX_METADATA_BYTES {
            return Err(CheckpointError::InvalidState(
                "checkpoint metadata exceeds its hard safety bound"
                    .into(),
            ));
        }

        Ok(())
    }

    /// Total variable state bytes.
    pub fn state_bytes(&self) -> CheckpointResult<u64> {
        let mut total = 0u64;

        total = checked_add(
            total,
            self.pauli_frame.len() as u64,
        )?;

        total = checked_add(
            total,
            self.logical_state.len() as u64,
        )?;

        total = checked_add(
            total,
            self.decoder_state.len() as u64,
        )?;

        total = checked_add(
            total,
            self.execution_state.len() as u64,
        )?;

        total = checked_add(
            total,
            self.metadata.len() as u64,
        )?;

        Ok(total)
    }
}

// ============================================================================
// Persisted checkpoint
// ============================================================================

/// Complete checkpoint payload.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct Checkpoint {
    /// Artifact kind from the canonical versioning layer.
    pub artifact_kind: String,

    /// Current QEC version.
    pub qec_version: String,

    /// Checkpoint schema version.
    pub checkpoint_version: String,

    /// Algorithm identity.
    pub algorithm: AlgorithmIdentity,

    /// Configuration identity.
    pub configuration: ConfigurationIdentity,

    /// Exact execution identity.
    pub execution: ExecutionIdentity,

    /// Creation timestamp in Unix nanoseconds.
    pub created_unix_nanos: u64,

    /// Runtime resource snapshot.
    pub resource_usage: CheckpointResourceUsage,

    /// Resumable execution state.
    pub state: CheckpointState,
}

impl Checkpoint {
    /// Creates a new checkpoint using canonical QEC version metadata.
    pub fn new(
        algorithm: AlgorithmIdentity,
        configuration: ConfigurationIdentity,
        execution: ExecutionIdentity,
        resource_usage: CheckpointResourceUsage,
        state: CheckpointState,
    ) -> CheckpointResult<Self> {
        execution.validate()?;
        resource_usage.validate()?;
        state.validate()?;

        let created_unix_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| {
                CheckpointError::Time(error.to_string())
            })?
            .as_nanos()
            .try_into()
            .map_err(|_| {
                CheckpointError::Time(
                    "system timestamp exceeds u64".into(),
                )
            })?;

        Ok(Self {
            artifact_kind:
                ArtifactKind::Checkpoint.as_str().to_string(),

            qec_version: CURRENT_QEC_VERSION.to_string(),

            checkpoint_version:
                CURRENT_CHECKPOINT_VERSION.to_string(),

            algorithm,

            configuration,

            execution,

            created_unix_nanos,

            resource_usage,

            state,
        })
    }

    /// Validates structural correctness without comparing against a live
    /// execution context.
    pub fn validate_structure(&self) -> CheckpointResult<()> {
        if self.artifact_kind
            != ArtifactKind::Checkpoint.as_str()
        {
            return Err(CheckpointError::Malformed(
                "artifact kind is not checkpoint".into(),
            ));
        }

        let qec_version =
            parse_version(&self.qec_version)?;

        if !qec_version.is_compatible_with(CURRENT_QEC_VERSION) {
            return Err(CheckpointError::VersionMismatch {
                artifact: self.qec_version.clone(),
                expected: CURRENT_QEC_VERSION.to_string(),
            });
        }

        let checkpoint_version =
            parse_version(&self.checkpoint_version)?;

        if !checkpoint_version
            .is_compatible_with(CURRENT_CHECKPOINT_VERSION)
        {
            return Err(CheckpointError::UnsupportedSchemaVersion {
                found: checkpoint_version.major,
                supported: CURRENT_CHECKPOINT_VERSION.major,
            });
        }

        validate_string(
            "algorithm name",
            &self.algorithm.name,
            MAX_ALGORITHM_NAME_BYTES,
        )?;

        validate_string(
            "algorithm version",
            &self.algorithm.version,
            MAX_ALGORITHM_VERSION_BYTES,
        )?;

        validate_string(
            "configuration identity",
            &self.configuration.id,
            MAX_CONFIGURATION_ID_BYTES,
        )?;

        self.execution.validate()?;

        self.resource_usage.validate()?;

        self.state.validate()?;

        Ok(())
    }

    /// Validates this checkpoint against the expected live execution.
    pub fn validate_resume(
        &self,
        expected: &ResumeContext<'_>,
    ) -> CheckpointResult<()> {
        self.validate_structure()?;

        if self.algorithm != *expected.algorithm {
            return Err(CheckpointError::AlgorithmMismatch {
                checkpoint: format!(
                    "{}@{}",
                    self.algorithm.name,
                    self.algorithm.version
                ),
                expected: format!(
                    "{}@{}",
                    expected.algorithm.name,
                    expected.algorithm.version
                ),
            });
        }

        if self.configuration != *expected.configuration {
            return Err(CheckpointError::ConfigurationMismatch {
                checkpoint: self.configuration.id.clone(),
                expected: expected.configuration.id.clone(),
            });
        }

        if self.execution.code_hash
            != expected.execution.code_hash
        {
            return Err(CheckpointError::CodeMismatch {
                checkpoint: self.execution.code_hash.clone(),
                expected: expected.execution.code_hash.clone(),
            });
        }

        if self.execution.backend_id
            != expected.execution.backend_id
        {
            return Err(CheckpointError::BackendMismatch {
                checkpoint: self.execution.backend_id.clone(),
                expected: expected.execution.backend_id.clone(),
            });
        }

        if self.execution.decoder_id
            != expected.execution.decoder_id
        {
            return Err(CheckpointError::DecoderMismatch {
                checkpoint: self.execution.decoder_id.clone(),
                expected: expected.execution.decoder_id.clone(),
            });
        }

        if self.execution.determinism_fingerprint
            != expected
                .execution
                .determinism_fingerprint
        {
            return Err(
                CheckpointError::DeterminismMismatch {
                    checkpoint: self.execution
                        .determinism_fingerprint
                        .clone(),
                    expected: expected
                        .execution
                        .determinism_fingerprint
                        .clone(),
                },
            );
        }

        if self.execution.resource_policy_fingerprint
            != expected
                .execution
                .resource_policy_fingerprint
        {
            return Err(
                CheckpointError::ResourcePolicyMismatch {
                    checkpoint: self.execution
                        .resource_policy_fingerprint
                        .clone(),
                    expected: expected
                        .execution
                        .resource_policy_fingerprint
                        .clone(),
                },
            );
        }

        if self.execution.execution_target
            != expected.execution.execution_target
        {
            return Err(CheckpointError::TargetMismatch {
                checkpoint: self.execution
                    .execution_target
                    .clone(),
                expected: expected
                    .execution_target
                    .clone(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Resume context
// ============================================================================

/// Live execution identity used when restoring a checkpoint.
///
/// The scheduler, decoder, backend and configuration layer should construct
/// this value rather than allowing checkpoint.rs to inspect those modules
/// directly.
#[derive(Debug, Clone, Copy)]
pub struct ResumeContext<'a> {
    pub algorithm: &'a AlgorithmIdentity,

    pub configuration: &'a ConfigurationIdentity,

    pub execution: &'a ExecutionIdentity,
}

// ============================================================================
// Envelope
// ============================================================================

/// Fixed-size untrusted envelope.
///
/// The payload itself is never interpreted until the envelope has been
/// validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Envelope {
    format_version: u16,

    schema_version: u16,

    payload_length: u64,

    payload_hash: [u8; SHA256_BYTES],
}

impl Envelope {
    fn encode(&self) -> [u8; CHECKPOINT_HEADER_BYTES] {
        let mut output = [0u8; CHECKPOINT_HEADER_BYTES];

        output[..8].copy_from_slice(CHECKPOINT_MAGIC);

        output[8..10]
            .copy_from_slice(&self.format_version.to_le_bytes());

        output[10..12]
            .copy_from_slice(&self.schema_version.to_le_bytes());

        output[12..20]
            .copy_from_slice(&self.payload_length.to_le_bytes());

        output[20..52].copy_from_slice(&self.payload_hash);

        output
    }

    fn decode(
        bytes: &[u8; CHECKPOINT_HEADER_BYTES],
    ) -> CheckpointResult<Self> {
        if &bytes[..8] != CHECKPOINT_MAGIC {
            return Err(CheckpointError::Malformed(
                "invalid checkpoint magic".into(),
            ));
        }

        let format_version =
            u16::from_le_bytes([bytes[8], bytes[9]]);

        let schema_version =
            u16::from_le_bytes([bytes[10], bytes[11]]);

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

        let mut payload_hash = [0u8; SHA256_BYTES];

        payload_hash.copy_from_slice(&bytes[20..52]);

        Ok(Self {
            format_version,
            schema_version,
            payload_length,
            payload_hash,
        })
    }

    fn validate(
        &self,
        policy: &CheckpointPolicy,
    ) -> CheckpointResult<()> {
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

        let maximum = policy.max_bytes();

        if self.payload_length > maximum {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_payload_bytes",
                    requested: self.payload_length,
                    maximum,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint manager
// ============================================================================

/// Main checkpoint persistence service.
///
/// It contains policy but not scheduler state.
#[derive(Debug, Clone)]
pub struct CheckpointManager {
    policy: CheckpointPolicy,
}

impl CheckpointManager {
    pub fn new(
        policy: CheckpointPolicy,
    ) -> CheckpointResult<Self> {
        policy.validate()?;

        Ok(Self { policy })
    }

    pub fn policy(&self) -> &CheckpointPolicy {
        &self.policy
    }

    /// Serializes and bounds a checkpoint without performing filesystem I/O.
    pub fn encode(
        &self,
        checkpoint: &Checkpoint,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Vec<u8>> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        checkpoint.validate_structure()?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let payload =
            serde_json::to_vec(checkpoint).map_err(|error| {
                CheckpointError::Serialization(error.to_string())
            })?;

        self.validate_payload_size(payload.len())?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let hash = sha256(&payload);

        let envelope = Envelope {
            format_version: CHECKPOINT_FORMAT_VERSION,
            schema_version: CHECKPOINT_SCHEMA_VERSION,
            payload_length: payload.len() as u64,
            payload_hash: hash,
        };

        let header = envelope.encode();

        let total_size =
            checked_add(header.len() as u64, payload.len() as u64)?;

        self.validate_total_size(total_size)?;

        let total_capacity =
            usize::try_from(total_size).map_err(|_| {
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_memory",
                    requested: total_size,
                    maximum: self.policy.max_bytes(),
                }
            })?;

        let mut output = Vec::with_capacity(total_capacity);

        output.extend_from_slice(&header);
        output.extend_from_slice(&payload);

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        Ok(output)
    }

    /// Restores a checkpoint from an in-memory artifact.
    ///
    /// The input is treated as completely untrusted.
    pub fn decode(
        &self,
        bytes: &[u8],
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Checkpoint> {
        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        self.validate_total_size(bytes.len() as u64)?;

        if bytes.len() < CHECKPOINT_HEADER_BYTES {
            return Err(CheckpointError::Malformed(
                "checkpoint is smaller than its fixed header"
                    .into(),
            ));
        }

        let header: [u8; CHECKPOINT_HEADER_BYTES] =
            bytes[..CHECKPOINT_HEADER_BYTES]
                .try_into()
                .map_err(|_| {
                    CheckpointError::Malformed(
                        "failed to read checkpoint header"
                            .into(),
                    )
                })?;

        let envelope = Envelope::decode(&header)?;

        envelope.validate(&self.policy)?;

        let payload_length =
            usize::try_from(envelope.payload_length)
                .map_err(|_| {
                    CheckpointError::Malformed(
                        "payload length does not fit usize"
                            .into(),
                    )
                })?;

        let expected_total =
            CHECKPOINT_HEADER_BYTES
                .checked_add(payload_length)
                .ok_or_else(|| {
                    CheckpointError::Malformed(
                        "checkpoint size arithmetic overflow"
                            .into(),
                    )
                })?;

        if expected_total != bytes.len() {
            return Err(CheckpointError::Malformed(
                "declared payload length does not match \
                 actual checkpoint length"
                    .into(),
            ));
        }

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let payload =
            &bytes[CHECKPOINT_HEADER_BYTES..expected_total];

        let actual_hash = sha256(payload);

        if actual_hash != envelope.payload_hash {
            return Err(
                CheckpointError::IntegrityMismatch {
                    expected: envelope.payload_hash,
                    actual: actual_hash,
                },
            );
        }

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let checkpoint: Checkpoint =
            serde_json::from_slice(payload).map_err(|error| {
                CheckpointError::Deserialization(error.to_string())
            })?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        checkpoint.validate_structure()?;

        Ok(checkpoint)
    }

    /// Saves a checkpoint atomically.
    ///
    /// The temporary file is created with `create_new`, written completely,
    /// flushed and then renamed into place.
    pub fn save(
        &self,
        path: impl AsRef<Path>,
        checkpoint: &Checkpoint,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<PathBuf> {
        if !self.policy.allow_filesystem {
            return Err(CheckpointError::FilesystemDisabled);
        }

        let path = path.as_ref();

        validate_path(path, self.policy.max_path_bytes)?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let encoded =
            self.encode(checkpoint, cancellation)?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let temporary_path =
            temporary_path(path)?;

        let write_result = self.write_atomic_temp(
            &temporary_path,
            &encoded,
            cancellation,
        );

        if let Err(error) = write_result {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        fs::rename(&temporary_path, path)?;

        Ok(path.to_path_buf())
    }

    /// Loads and validates a checkpoint from disk.
    pub fn load(
        &self,
        path: impl AsRef<Path>,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<Checkpoint> {
        if !self.policy.allow_filesystem {
            return Err(CheckpointError::FilesystemDisabled);
        }

        let path = path.as_ref();

        validate_path(path, self.policy.max_path_bytes)?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        let metadata = fs::metadata(path)?;

        let maximum_total =
            self.policy.max_bytes()
                .checked_add(CHECKPOINT_HEADER_BYTES as u64)
                .ok_or_else(|| {
                    CheckpointError::InvalidInput(
                        "checkpoint size limit overflow".into(),
                    )
                })?;

        if metadata.len() > maximum_total {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_file_bytes",
                    requested: metadata.len(),
                    maximum: maximum_total,
                },
            );
        }

        let file_size =
            usize::try_from(metadata.len()).map_err(|_| {
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_file_bytes",
                    requested: metadata.len(),
                    maximum: maximum_total,
                }
            })?;

        let mut file = File::open(path)?;

        let mut bytes = Vec::with_capacity(file_size);

        let mut remaining = file_size;

        let mut buffer = [0u8; 64 * 1024];

        while remaining > 0 {
            cancellation
                .check()
                .map_err(|_| {
                    CheckpointError::CancellationRequested
                })?;

            let requested = remaining.min(buffer.len());

            let read = file.read(&mut buffer[..requested])?;

            if read == 0 {
                break;
            }

            bytes.extend_from_slice(&buffer[..read]);

            remaining -= read;
        }

        if bytes.len() != file_size {
            return Err(CheckpointError::Malformed(
                "checkpoint file ended before declared \
                 file length"
                    .into(),
            ));
        }

        self.decode(&bytes, cancellation)
    }

    /// Deletes a checkpoint after validating the path.
    pub fn remove(
        &self,
        path: impl AsRef<Path>,
        cancellation: &CancellationToken,
    ) -> CheckpointResult<()> {
        if !self.policy.allow_filesystem {
            return Err(CheckpointError::FilesystemDisabled);
        }

        let path = path.as_ref();

        validate_path(path, self.policy.max_path_bytes)?;

        cancellation
            .check()
            .map_err(|_| CheckpointError::CancellationRequested)?;

        fs::remove_file(path)?;

        Ok(())
    }

    fn write_atomic_temp(
        &self,
        path: &Path,
        bytes: &[u8],
        cancellation: &CancellationToken,
    ) -> CheckpointResult<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;

        let mut offset = 0usize;

        while offset < bytes.len() {
            cancellation
                .check()
                .map_err(|_| {
                    CheckpointError::CancellationRequested
                })?;

            let written =
                file.write(&bytes[offset..])?;

            if written == 0 {
                return Err(CheckpointError::Io(
                    io::Error::new(
                        io::ErrorKind::WriteZero,
                        "checkpoint write returned zero",
                    ),
                ));
            }

            offset = offset
                .checked_add(written)
                .ok_or_else(|| {
                    CheckpointError::InvalidState(
                        "checkpoint write offset overflow"
                            .into(),
                    )
                })?;
        }

        file.sync_all()?;

        Ok(())
    }

    fn validate_payload_size(
        &self,
        payload_size: usize,
    ) -> CheckpointResult<()> {
        let requested = payload_size as u64;

        if requested > self.policy.max_bytes() {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_payload_bytes",
                    requested,
                    maximum: self.policy.max_bytes(),
                },
            );
        }

        Ok(())
    }

    fn validate_total_size(
        &self,
        total_size: u64,
    ) -> CheckpointResult<()> {
        let maximum = self
            .policy
            .max_bytes()
            .checked_add(CHECKPOINT_HEADER_BYTES as u64)
            .ok_or_else(|| {
                CheckpointError::InvalidInput(
                    "checkpoint maximum size overflow".into(),
                )
            })?;

        if total_size > maximum {
            return Err(
                CheckpointError::ResourceLimitExceeded {
                    resource: "checkpoint_total_bytes",
                    requested: total_size,
                    maximum,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Deterministic payload utilities
// ============================================================================

/// Computes the canonical SHA-256 digest.
pub fn sha256(bytes: &[u8]) -> [u8; SHA256_BYTES] {
    let mut hasher = Sha256::new();

    hasher.update(bytes);

    let digest = hasher.finalize();

    let mut result = [0u8; SHA256_BYTES];

    result.copy_from_slice(&digest);

    result
}

/// Returns a lowercase hexadecimal digest.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = sha256(bytes);

    let mut output =
        String::with_capacity(SHA256_BYTES * 2);

    for byte in digest {
        use std::fmt::Write as _;

        let _ = write!(&mut output, "{byte:02x}");
    }

    output
}

// ============================================================================
// Validation helpers
// ============================================================================

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
        return Err(CheckpointError::InvalidInput(
            format!(
                "{field} exceeds maximum length {maximum}"
            ),
        ));
    }

    if value
        .as_bytes()
        .iter()
        .any(|byte| *byte == 0)
    {
        return Err(CheckpointError::InvalidInput(
            format!("{field} contains NUL"),
        ));
    }

    Ok(())
}

fn validate_path(
    path: &Path,
    maximum_bytes: usize,
) -> CheckpointResult<()> {
    let text = path.to_string_lossy();

    if text.is_empty() {
        return Err(CheckpointError::InvalidInput(
            "checkpoint path must not be empty".into(),
        ));
    }

    if text.len() > maximum_bytes {
        return Err(CheckpointError::InvalidInput(
            "checkpoint path exceeds configured limit".into(),
        ));
    }

    if text.contains('\0') {
        return Err(CheckpointError::InvalidInput(
            "checkpoint path contains NUL".into(),
        ));
    }

    Ok(())
}

fn temporary_path(
    destination: &Path,
) -> CheckpointResult<PathBuf> {
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let file_name = destination
        .file_name()
        .ok_or_else(|| {
            CheckpointError::InvalidInput(
                "checkpoint destination has no file name"
                    .into(),
            )
        })?
        .to_string_lossy();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            CheckpointError::Time(error.to_string())
        })?
        .as_nanos();

    let process_id = std::process::id();

    let temporary_name = format!(
        ".{file_name}.{}.{}.tmp",
        process_id,
        timestamp
    );

    Ok(parent.join(temporary_name))
}

fn checked_add(
    left: u64,
    right: u64,
) -> CheckpointResult<u64> {
    left.checked_add(right).ok_or_else(|| {
        CheckpointError::InvalidState(
            "checkpoint size arithmetic overflow".into(),
        )
    })
}

fn parse_version(
    value: &str,
) -> CheckpointResult<Version> {
    let parts: Vec<&str> = value.split('.').collect();

    if parts.len() != 3 {
        return Err(CheckpointError::Malformed(
            "version must use MAJOR.MINOR.PATCH".into(),
        ));
    }

    let major = parts[0].parse::<u16>().map_err(|_| {
        CheckpointError::Malformed(
            "invalid major version".into(),
        )
    })?;

    let minor = parts[1].parse::<u16>().map_err(|_| {
        CheckpointError::Malformed(
            "invalid minor version".into(),
        )
    })?;

    let patch = parts[2].parse::<u16>().map_err(|_| {
        CheckpointError::Malformed(
            "invalid patch version".into(),
        )
    })?;

    Ok(Version::new(major, minor, patch))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_policy() -> CheckpointPolicy {
        CheckpointPolicy::new(QecLimits::default())
            .expect("default QEC limits must be valid")
    }

    fn cancellation() -> CancellationToken {
        CancellationToken::new()
    }

    fn sample_checkpoint() -> Checkpoint {
        let algorithm =
            AlgorithmIdentity::new("mwpm", "3.0.0")
                .expect("valid algorithm");

        let configuration =
            ConfigurationIdentity::new(
                "config-sha256:example",
            )
            .expect("valid configuration");

        let execution = ExecutionIdentity {
            code_hash: "code-sha256:example".into(),
            backend_id: "simulator".into(),
            decoder_id: "mwpm".into(),
            determinism_fingerprint:
                "deterministic-v3".into(),
            resource_policy_fingerprint:
                "limits-v3".into(),
            execution_target: "simulator".into(),
        };

        let state = CheckpointState {
            position: ResumePosition {
                round: 4,
                events_processed: 100,
                decoder_iterations: 8,
                partition_id: Some(2),
                stream_offset: Some(4096),
            },

            seed: 42,

            pauli_frame: vec![1, 2, 3],

            logical_state: vec![4, 5],

            decoder_state: vec![6, 7, 8, 9],

            execution_state: vec![10, 11],

            metadata: b"test".to_vec(),
        };

        Checkpoint::new(
            algorithm,
            configuration,
            execution,
            CheckpointResourceUsage {
                allocated_bytes: 1024,
                peak_bytes: 2048,
                cpu_time_ns: 100,
                wall_time_ns: 200,
                graph_nodes: 20,
                graph_edges: 40,
                syndrome_events: 100,
                decoder_iterations: 8,
                parallel_workers: 1,
            },
            state,
        )
        .expect("sample checkpoint must be valid")
    }

    #[test]
    fn round_trip_memory() {
        let manager =
            CheckpointManager::new(test_policy())
                .expect("valid policy");

        let source = cancellation();

        let checkpoint = sample_checkpoint();

        let encoded = manager
            .encode(&checkpoint, &source)
            .expect("encoding must succeed");

        let restored = manager
            .decode(&encoded, &source)
            .expect("decoding must succeed");

        assert_eq!(checkpoint, restored);
    }

    #[test]
    fn corrupted_payload_is_rejected() {
        let manager =
            CheckpointManager::new(test_policy())
                .expect("valid policy");

        let source = cancellation();

        let checkpoint = sample_checkpoint();

        let mut encoded = manager
            .encode(&checkpoint, &source)
            .expect("encoding must succeed");

        let last =
            encoded.len().checked_sub(1).unwrap();

        encoded[last] ^= 0xFF;

        let result =
            manager.decode(&encoded, &source);

        assert!(matches!(
            result,
            Err(CheckpointError::IntegrityMismatch { .. })
        ));
    }

    #[test]
    fn truncated_header_is_rejected() {
        let manager =
            CheckpointManager::new(test_policy())
                .expect("valid policy");

        let source = cancellation();

        let result =
            manager.decode(&[1, 2, 3], &source);

        assert!(matches!(
            result,
            Err(CheckpointError::Malformed(_))
        ));
    }

    #[test]
    fn declared_length_mismatch_is_rejected() {
        let manager =
            CheckpointManager::new(test_policy())
                .expect("valid policy");

        let source = cancellation();

        let checkpoint = sample_checkpoint();

        let mut encoded = manager
            .encode(&checkpoint, &source)
            .expect("encoding must succeed");

        encoded[12] = encoded[12].wrapping_add(1);

        let result =
            manager.decode(&encoded, &source);

        assert!(result.is_err());
    }

    #[test]
    fn cancellation_is_honored_before_encoding() {
        let manager =
            CheckpointManager::new(test_policy())
                .expect("valid policy");

        let source =
            super::super::cancellation::CancellationSource::new();

        source.cancel();

        let checkpoint = sample_checkpoint();

        let result =
            manager.encode(&checkpoint, &source.token());

        assert!(matches!(
            result,
            Err(CheckpointError::CancellationRequested)
        ));
    }

    #[test]
    fn resume_identity_is_enforced() {
        let checkpoint = sample_checkpoint();

        let algorithm =
            AlgorithmIdentity::new("union_find", "3.0.0")
                .expect("valid algorithm");

        let configuration =
            ConfigurationIdentity::new(
                "different-config",
            )
            .expect("valid configuration");

        let execution = ExecutionIdentity {
            code_hash: "different-code".into(),
            backend_id: "different-backend".into(),
            decoder_id: "different-decoder".into(),
            determinism_fingerprint:
                "different-determinism".into(),
            resource_policy_fingerprint:
                "different-policy".into(),
            execution_target: "cpu".into(),
        };

        let context = ResumeContext {
            algorithm: &algorithm,
            configuration: &configuration,
            execution: &execution,
        };

        assert!(checkpoint
            .validate_resume(&context)
            .is_err());
    }

    #[test]
    fn state_resource_accounting_is_checked() {
        let state = CheckpointState {
            pauli_frame: vec![0; 16],
            logical_state: vec![0; 32],
            decoder_state: vec![0; 64],
            execution_state: vec![0; 128],
            metadata: vec![0; 256],
            ..CheckpointState::default()
        };

        assert_eq!(
            state.state_bytes()
                .expect("state size calculation"),
            16 + 32 + 64 + 128 + 256
        );
    }

    #[test]
    fn digest_is_deterministic() {
        let first = sha256_hex(b"zamani-qec");

        let second = sha256_hex(b"zamani-qec");

        assert_eq!(first, second);
        assert_eq!(first.len(), 64);
    }
}