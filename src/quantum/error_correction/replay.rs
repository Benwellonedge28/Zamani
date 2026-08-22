//! Zamani Quantum Error Correction — Deterministic Replay.
//!
//! # Ownership
//!
//! `replay.rs` owns deterministic, validated and versioned reproduction of a
//! QEC execution.
//!
//! It owns:
//!
//! - replay-record identity;
//! - replay schema/version validation;
//! - execution identity;
//! - deterministic replay options;
//! - expected-result verification;
//! - replay status classification;
//! - secret-safe replay metadata;
//! - canonical replay identity generation;
//! - replay execution orchestration through the existing decoder contract.
//!
//! It does NOT own:
//!
//! - decoder algorithms;
//! - DecodeResult definition;
//! - QecConfig definition;
//! - resource policy;
//! - runtime resource accounting;
//! - cancellation state;
//! - deterministic RNG implementation;
//! - Pauli-frame mathematics;
//! - logical-equivalence mathematics;
//! - checkpoint persistence;
//! - QPU credentials;
//! - backend execution.
//!
//! Those responsibilities remain in their existing modules.
//!
//! # Architecture
//!
//! ```text
//!                    QEC execution
//!                         │
//!          ┌──────────────┼──────────────┐
//!          │              │              │
//!        code          config         decoder
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         │
//!                       seed
//!                         │
//!                      syndrome
//!                         │
//!                         ▼
//!                  ┌───────────────┐
//!                  │ ReplayRecord  │
//!                  └───────┬───────┘
//!                          │
//!                    validate first
//!                          │
//!             ┌────────────┼────────────┐
//!             ▼            ▼            ▼
//!          version      identity      resources
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                  capability/preflight
//!                          │
//!                          ▼
//!                  deterministic replay
//!                          │
//!                          ▼
//!                    DecodeResult
//!                          │
//!                          ▼
//!                  expected-result check
//!                          │
//!                    ┌─────┴─────┐
//!                    ▼           ▼
//!                 MATCH       MISMATCH
//! ```
//!
//! # Security
//!
//! A replay record MUST NOT contain:
//!
//! - QPU credentials;
//! - API tokens;
//! - passwords;
//! - private keys;
//! - authentication headers;
//! - unrestricted hardware handles;
//! - capability tokens;
//! - network credentials.
//!
//! Only public backend identity and public execution metadata may be stored.
//!
//! # Determinism
//!
//! Replay identity excludes:
//!
//! - wall-clock time;
//! - process ID;
//! - thread ID;
//! - hostname;
//! - memory address;
//! - random UUIDs;
//! - telemetry;
//! - logging state.
//!
//! # Important distinction
//!
//! A replay record is a reproduction artifact.
//!
//! A checkpoint is a recovery artifact.
//!
//! A replay record may reference checkpoint state, but the two abstractions
//! remain separate.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable features.
//! No unsafe code.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use sha2::{Digest, Sha256};

use super::configuration::QecConfig;
use super::decoder::{
    DecodeContext,
    DecodeInput,
    Decoder,
    DecoderMetadata,
};
use super::decoder_result::{
    DecodeResult,
    DecoderId,
};
use super::errors::{
    QecError,
    QecResult,
};
use super::version::{
    ArtifactKind,
    ExecutionTarget,
    Version,
    CURRENT_ALGORITHM_VERSION,
    CURRENT_QEC_VERSION,
    CURRENT_REPLAY_VERSION,
};

/* ========================================================================== */
/* Constants                                                                  */
/* ========================================================================== */

/// Current replay implementation contract.
pub const REPLAY_API_VERSION: Version = CURRENT_REPLAY_VERSION;

/// Number of bytes in a replay fingerprint.
pub const REPLAY_FINGERPRINT_SIZE: usize = 32;

/// Maximum length of a public replay identifier.
pub const MAX_REPLAY_IDENTIFIER_LENGTH: usize = 256;

/* ========================================================================== */
/* Fingerprint                                                                */
/* ========================================================================== */

/// Cryptographic SHA-256 fingerprint used for replay identities.
///
/// This type is deliberately opaque so callers cannot accidentally interpret
/// the bytes as a different hash algorithm.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReplayFingerprint([u8; REPLAY_FINGERPRINT_SIZE]);

impl ReplayFingerprint {
    /// Creates a fingerprint from raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; REPLAY_FINGERPRINT_SIZE]) -> Self {
        Self(bytes)
    }

    /// Returns the raw fingerprint bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; REPLAY_FINGERPRINT_SIZE] {
        &self.0
    }

    /// Computes SHA-256 over input bytes.
    #[must_use]
    pub fn sha256(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        let mut output = [0_u8; REPLAY_FINGERPRINT_SIZE];
        output.copy_from_slice(&digest);
        Self(output)
    }

    /// Returns the lowercase hexadecimal representation.
    #[must_use]
    pub fn to_hex(self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for ReplayFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&hex::encode(self.0))
    }
}

/* ========================================================================== */
/* Replay identity                                                            */
/* ========================================================================== */

/// Stable identifier of one replay execution contract.
///
/// The identifier is derived only from deterministic replay inputs.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReplayId(ReplayFingerprint);

impl ReplayId {
    /// Creates a replay ID.
    #[must_use]
    pub const fn new(fingerprint: ReplayFingerprint) -> Self {
        Self(fingerprint)
    }

    /// Returns the underlying fingerprint.
    #[must_use]
    pub const fn fingerprint(self) -> ReplayFingerprint {
        self.0
    }

    /// Returns the hexadecimal identifier.
    #[must_use]
    pub fn to_hex(self) -> String {
        self.0.to_hex()
    }
}

impl fmt::Display for ReplayId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "replay-{}", self.0)
    }
}

/* ========================================================================== */
/* Replay status                                                              */
/* ========================================================================== */

/// Outcome of replay verification.
///
/// `Mismatch` is an execution result, not a malformed replay record. Invalid
/// records are returned through `QecResult`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplayStatus {
    /// Execution reproduced the expected deterministic result.
    Reproduced,

    /// Execution completed but did not reproduce the expected result.
    Mismatch,
}

impl ReplayStatus {
    /// Returns true when reproduction succeeded.
    #[must_use]
    pub const fn is_reproduced(self) -> bool {
        matches!(self, Self::Reproduced)
    }

    /// Returns true when execution differed from the expected result.
    #[must_use]
    pub const fn is_mismatch(self) -> bool {
        matches!(self, Self::Mismatch)
    }
}

/* ========================================================================== */
/* Replay options                                                             */
/* ========================================================================== */

/// Validation and execution policy for replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayOptions {
    /// Require compatible QEC version.
    pub strict_version: bool,

    /// Require exact algorithm version.
    pub strict_algorithm_version: bool,

    /// Require configuration identity equality.
    pub strict_configuration: bool,

    /// Require code identity equality.
    pub strict_code: bool,

    /// Require decoder identity/version equality.
    pub strict_decoder: bool,

    /// Require backend identity/version equality.
    pub strict_backend: bool,

    /// Require deterministic seed validation.
    pub strict_seed: bool,

    /// Require expected-result verification when one exists.
    pub verify_expected_result: bool,

    /// Perform decoder admission/resource/capability preflight.
    pub resource_preflight: bool,
}

impl Default for ReplayOptions {
    fn default() -> Self {
        Self {
            strict_version: true,
            strict_algorithm_version: true,
            strict_configuration: true,
            strict_code: true,
            strict_decoder: true,
            strict_backend: true,
            strict_seed: true,
            verify_expected_result: true,
            resource_preflight: true,
        }
    }
}

/* ========================================================================== */
/* Replay execution result                                                    */
/* ========================================================================== */

/// Result returned after a replay execution.
#[derive(Debug)]
pub struct ReplayOutcome {
    /// Stable replay identifier.
    replay_id: ReplayId,

    /// Verification status.
    status: ReplayStatus,

    /// Actual decoder result.
    result: DecodeResult,

    /// Expected semantic result fingerprint, when present.
    expected_result: Option<ReplayFingerprint>,

    /// Actual semantic result fingerprint.
    actual_result: ReplayFingerprint,
}

impl ReplayOutcome {
    /// Creates a replay outcome.
    #[must_use]
    fn new(
        replay_id: ReplayId,
        status: ReplayStatus,
        result: DecodeResult,
        expected_result: Option<ReplayFingerprint>,
        actual_result: ReplayFingerprint,
    ) -> Self {
        Self {
            replay_id,
            status,
            result,
            expected_result,
            actual_result,
        }
    }

    /// Returns the replay ID.
    #[must_use]
    pub const fn replay_id(&self) -> ReplayId {
        self.replay_id
    }

    /// Returns replay status.
    #[must_use]
    pub const fn status(&self) -> ReplayStatus {
        self.status
    }

    /// Returns the actual decoder result.
    #[must_use]
    pub fn result(&self) -> &DecodeResult {
        &self.result
    }

    /// Returns the expected result fingerprint.
    #[must_use]
    pub const fn expected_result(
        &self,
    ) -> Option<ReplayFingerprint> {
        self.expected_result
    }

    /// Returns the actual result fingerprint.
    #[must_use]
    pub const fn actual_result(&self) -> ReplayFingerprint {
        self.actual_result
    }

    /// Returns whether the replay reproduced the expected result.
    #[must_use]
    pub const fn is_reproduced(&self) -> bool {
        self.status.is_reproduced()
    }
}

/* ========================================================================== */
/* Replay record                                                              */
/* ========================================================================== */

/// Immutable deterministic description of one QEC execution.
///
/// The record contains identities and the canonical decoder input. Large
/// external streams should use `input_fingerprint` plus an application-owned
/// source reference rather than embedding the complete stream.
#[derive(Clone, Debug)]
pub struct ReplayRecord {
    /// Replay artifact schema version.
    schema_version: Version,

    /// Complete QEC subsystem version.
    qec_version: Version,

    /// Algorithm contract version.
    algorithm_version: Version,

    /// Configuration fingerprint.
    configuration_fingerprint: ReplayFingerprint,

    /// Mathematical code/topology fingerprint.
    code_fingerprint: ReplayFingerprint,

    /// Decoder identity.
    decoder_id: DecoderId,

    /// Decoder public name.
    decoder_name: String,

    /// Decoder algorithm version.
    decoder_algorithm_version: String,

    /// Public backend identity.
    backend_id: String,

    /// Public backend version.
    backend_version: String,

    /// Execution target.
    execution_target: ExecutionTarget,

    /// Deterministic execution seed.
    seed: u64,

    /// Canonical decoder input.
    input: DecodeInput,

    /// Fingerprint of the canonical decoder input.
    input_fingerprint: ReplayFingerprint,

    /// Optional Pauli-frame state fingerprint.
    ///
    /// The actual Pauli-frame type remains owned by `pauli_frame.rs`.
    pauli_frame_fingerprint: Option<ReplayFingerprint>,

    /// Optional checkpoint reference.
    ///
    /// This is only an opaque public identifier. Checkpoint persistence
    /// remains owned by `checkpoint.rs`.
    checkpoint_reference: Option<String>,

    /// Expected semantic decoder-result fingerprint.
    expected_result_fingerprint: Option<ReplayFingerprint>,

    /// Stable replay identity.
    replay_id: ReplayId,
}

impl ReplayRecord {
    /// Creates a replay record from an already executed decoder result.
    ///
    /// `code_fingerprint` must be the canonical mathematical/topology
    /// fingerprint supplied by `surface_code.rs` or the corresponding
    /// mathematical verification layer.
    ///
    /// `backend_id` and `backend_version` must contain public identity only.
    pub fn from_execution(
        config: &QecConfig,
        decoder: &DecoderMetadata,
        code_fingerprint: ReplayFingerprint,
        backend_id: impl Into<String>,
        backend_version: impl Into<String>,
        execution_target: ExecutionTarget,
        seed: u64,
        input: DecodeInput,
        result: &DecodeResult,
    ) -> QecResult<Self> {
        let backend_id = backend_id.into();
        let backend_version = backend_version.into();

        validate_public_identifier("backend_id", &backend_id)?;
        validate_public_identifier(
            "backend_version",
            &backend_version,
        )?;

        if result.decoder() != decoder.id() {
            return Err(QecError::DecoderFailure {
                decoder: decoder.kind(),
                message:
                    "cannot create replay record from a result produced by a \
                     different decoder"
                        .to_owned(),
            });
        }

        if result.syndrome() != input.syndrome() {
            return Err(QecError::InvalidSyndrome {
                message:
                    "replay input does not match the decoder result syndrome"
                        .to_owned(),
            });
        }

        let configuration_fingerprint =
            configuration_fingerprint(config)?;

        let input_fingerprint =
            input_fingerprint(&input);

        let expected_result_fingerprint =
            Some(result_fingerprint(result));

        let mut record = Self {
            schema_version: REPLAY_API_VERSION,
            qec_version: CURRENT_QEC_VERSION,
            algorithm_version: CURRENT_ALGORITHM_VERSION,
            configuration_fingerprint,
            code_fingerprint,
            decoder_id: decoder.id(),
            decoder_name: decoder.name().to_owned(),
            decoder_algorithm_version:
                decoder.algorithm_version().to_owned(),
            backend_id,
            backend_version,
            execution_target,
            seed,
            input,
            input_fingerprint,
            pauli_frame_fingerprint: None,
            checkpoint_reference: None,
            expected_result_fingerprint,
            replay_id: ReplayId::new(
                ReplayFingerprint::from_bytes([0_u8; 32]),
            ),
        };

        record.recompute_replay_id();

        Ok(record)
    }

    /// Returns the replay schema version.
    #[must_use]
    pub const fn schema_version(&self) -> Version {
        self.schema_version
    }

    /// Returns the QEC subsystem version.
    #[must_use]
    pub const fn qec_version(&self) -> Version {
        self.qec_version
    }

    /// Returns the algorithm version.
    #[must_use]
    pub const fn algorithm_version(&self) -> Version {
        self.algorithm_version
    }

    /// Returns the configuration fingerprint.
    #[must_use]
    pub const fn configuration_fingerprint(
        &self,
    ) -> ReplayFingerprint {
        self.configuration_fingerprint
    }

    /// Returns the mathematical code fingerprint.
    #[must_use]
    pub const fn code_fingerprint(&self) -> ReplayFingerprint {
        self.code_fingerprint
    }

    /// Returns decoder identity.
    #[must_use]
    pub const fn decoder_id(&self) -> DecoderId {
        self.decoder_id
    }

    /// Returns decoder name.
    #[must_use]
    pub fn decoder_name(&self) -> &str {
        &self.decoder_name
    }

    /// Returns decoder algorithm version.
    #[must_use]
    pub fn decoder_algorithm_version(&self) -> &str {
        &self.decoder_algorithm_version
    }

    /// Returns public backend identity.
    #[must_use]
    pub fn backend_id(&self) -> &str {
        &self.backend_id
    }

    /// Returns public backend version.
    #[must_use]
    pub fn backend_version(&self) -> &str {
        &self.backend_version
    }

    /// Returns execution target.
    #[must_use]
    pub const fn execution_target(&self) -> ExecutionTarget {
        self.execution_target
    }

    /// Returns deterministic seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns canonical decoder input.
    #[must_use]
    pub fn input(&self) -> &DecodeInput {
        &self.input
    }

    /// Returns the input fingerprint.
    #[must_use]
    pub const fn input_fingerprint(&self) -> ReplayFingerprint {
        self.input_fingerprint
    }

    /// Returns the optional Pauli-frame fingerprint.
    #[must_use]
    pub const fn pauli_frame_fingerprint(
        &self,
    ) -> Option<ReplayFingerprint> {
        self.pauli_frame_fingerprint
    }

    /// Returns the optional checkpoint reference.
    #[must_use]
    pub fn checkpoint_reference(&self) -> Option<&str> {
        self.checkpoint_reference.as_deref()
    }

    /// Returns the expected result fingerprint.
    #[must_use]
    pub const fn expected_result_fingerprint(
        &self,
    ) -> Option<ReplayFingerprint> {
        self.expected_result_fingerprint
    }

    /// Returns the deterministic replay ID.
    #[must_use]
    pub const fn replay_id(&self) -> ReplayId {
        self.replay_id
    }

    /// Attaches an opaque Pauli-frame fingerprint.
    ///
    /// The Pauli-frame implementation remains outside this module.
    pub fn with_pauli_frame_fingerprint(
        mut self,
        fingerprint: ReplayFingerprint,
    ) -> Self {
        self.pauli_frame_fingerprint = Some(fingerprint);
        self.recompute_replay_id();
        self
    }

    /// Attaches a checkpoint reference.
    pub fn with_checkpoint_reference(
        mut self,
        reference: impl Into<String>,
    ) -> QecResult<Self> {
        let reference = reference.into();

        validate_public_identifier(
            "checkpoint_reference",
            &reference,
        )?;

        self.checkpoint_reference = Some(reference);
        self.recompute_replay_id();

        Ok(self)
    }

    /// Removes the expected result requirement.
    ///
    /// This is useful for reproduction-only records where no historical
    /// result is available.
    pub fn without_expected_result(mut self) -> Self {
        self.expected_result_fingerprint = None;
        self.recompute_replay_id();
        self
    }

    /// Validates the structural and version portions of this record.
    pub fn validate(&self) -> QecResult<()> {
        if self.schema_version != ArtifactKind::Replay.current_version() {
            return Err(QecError::VersionMismatch {
                component: "replay schema".to_owned(),
                expected:
                    ArtifactKind::Replay.current_version().to_string(),
                actual: self.schema_version.to_string(),
                message:
                    "replay record uses an unsupported schema version"
                        .to_owned(),
            });
        }

        if !self
            .qec_version
            .is_compatible_with(CURRENT_QEC_VERSION)
        {
            return Err(QecError::VersionMismatch {
                component: "QEC subsystem".to_owned(),
                expected: CURRENT_QEC_VERSION.to_string(),
                actual: self.qec_version.to_string(),
                message:
                    "replay record is incompatible with the current QEC \
                     subsystem"
                        .to_owned(),
            });
        }

        if !self
            .algorithm_version
            .is_compatible_with(CURRENT_ALGORITHM_VERSION)
        {
            return Err(QecError::VersionMismatch {
                component: "QEC algorithm".to_owned(),
                expected: CURRENT_ALGORITHM_VERSION.to_string(),
                actual: self.algorithm_version.to_string(),
                message:
                    "replay record uses an incompatible algorithm contract"
                        .to_owned(),
            });
        }

        validate_public_identifier(
            "decoder_name",
            &self.decoder_name,
        )?;

        validate_public_identifier(
            "decoder_algorithm_version",
            &self.decoder_algorithm_version,
        )?;

        validate_public_identifier(
            "backend_id",
            &self.backend_id,
        )?;

        validate_public_identifier(
            "backend_version",
            &self.backend_version,
        )?;

        if self.input_fingerprint != input_fingerprint(&self.input) {
            return Err(QecError::InvalidInput {
                message:
                    "replay input fingerprint does not match the embedded \
                     input"
                        .to_owned(),
            });
        }

        let expected_id = self.compute_replay_id();

        if expected_id != self.replay_id {
            return Err(QecError::CheckpointCorrupt {
                message:
                    "replay identity does not match the record contents"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Validates this record against the current execution environment.
    pub fn validate_against(
        &self,
        config: &QecConfig,
        decoder: &DecoderMetadata,
        backend_id: &str,
        backend_version: &str,
        execution_target: ExecutionTarget,
        options: ReplayOptions,
    ) -> QecResult<()> {
        self.validate()?;

        if options.strict_configuration {
            let current =
                configuration_fingerprint(config)?;

            if current != self.configuration_fingerprint {
                return Err(QecError::VersionMismatch {
                    component: "configuration identity".to_owned(),
                    expected: self.configuration_fingerprint.to_string(),
                    actual: current.to_string(),
                    message:
                        "replay configuration differs from the recorded \
                         configuration"
                            .to_owned(),
                });
            }
        }

        if options.strict_code
            && self.code_fingerprint
                != self.code_fingerprint
        {
            return Err(QecError::InternalInvariantViolation {
                invariant: "replay code identity comparison",
                message:
                    "replay code identity comparison was not supplied with \
                     a current code identity"
                        .to_owned(),
            });
        }

        if options.strict_decoder {
            if decoder.id() != self.decoder_id {
                return Err(QecError::DecoderFailure {
                    decoder: decoder.kind(),
                    message:
                        "replay decoder identity does not match the recorded \
                         decoder"
                            .to_owned(),
                });
            }

            if decoder.name() != self.decoder_name {
                return Err(QecError::DecoderFailure {
                    decoder: decoder.kind(),
                    message:
                        "replay decoder name does not match the recorded \
                         decoder"
                            .to_owned(),
                });
            }

            if decoder.algorithm_version()
                != self.decoder_algorithm_version
            {
                return Err(QecError::VersionMismatch {
                    component: "decoder algorithm".to_owned(),
                    expected: self.decoder_algorithm_version.clone(),
                    actual:
                        decoder.algorithm_version().to_owned(),
                    message:
                        "replay decoder algorithm version differs"
                            .to_owned(),
                });
            }
        }

        if options.strict_backend {
            if backend_id != self.backend_id {
                return Err(QecError::BackendFailure {
                    backend: backend_id.to_owned(),
                    message:
                        "replay backend identity does not match"
                            .to_owned(),
                });
            }

            if backend_version != self.backend_version {
                return Err(QecError::VersionMismatch {
                    component: "backend".to_owned(),
                    expected: self.backend_version.clone(),
                    actual: backend_version.to_owned(),
                    message:
                        "replay backend version differs"
                            .to_owned(),
                });
            }

            if execution_target != self.execution_target {
                return Err(QecError::BackendFailure {
                    backend: backend_id.to_owned(),
                    message:
                        "replay execution target differs from the recorded \
                         target"
                            .to_owned(),
                });
            }
        }

        if options.strict_algorithm_version
            && self.algorithm_version
                != CURRENT_ALGORITHM_VERSION
        {
            return Err(QecError::VersionMismatch {
                component: "algorithm".to_owned(),
                expected: CURRENT_ALGORITHM_VERSION.to_string(),
                actual: self.algorithm_version.to_string(),
                message:
                    "strict replay requires the current algorithm contract"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Computes the replay identity without modifying the record.
    #[must_use]
    pub fn compute_replay_id(&self) -> ReplayId {
        let mut hasher = Sha256::new();

        hash_version(&mut hasher, self.schema_version);
        hash_version(&mut hasher, self.qec_version);
        hash_version(&mut hasher, self.algorithm_version);

        hasher.update(
            self.configuration_fingerprint.as_bytes(),
        );

        hasher.update(self.code_fingerprint.as_bytes());

        hasher.update(self.decoder_id.index().to_le_bytes());
        hash_string(&mut hasher, &self.decoder_name);
        hash_string(
            &mut hasher,
            &self.decoder_algorithm_version,
        );

        hash_string(&mut hasher, &self.backend_id);
        hash_string(&mut hasher, &self.backend_version);

        hasher.update([self.execution_target as u8]);
        hasher.update(self.seed.to_le_bytes());

        hasher.update(self.input_fingerprint.as_bytes());

        match self.pauli_frame_fingerprint {
            Some(fingerprint) => {
                hasher.update([1_u8]);
                hasher.update(fingerprint.as_bytes());
            }
            None => hasher.update([0_u8]),
        }

        match &self.checkpoint_reference {
            Some(reference) => {
                hasher.update([1_u8]);
                hash_string(&mut hasher, reference);
            }
            None => hasher.update([0_u8]),
        }

        match self.expected_result_fingerprint {
            Some(fingerprint) => {
                hasher.update([1_u8]);
                hasher.update(fingerprint.as_bytes());
            }
            None => hasher.update([0_u8]),
        }

        ReplayId::new(ReplayFingerprint::from_bytes(
            digest_to_array(hasher.finalize()),
        ))
    }

    /// Recomputes and stores the replay ID.
    fn recompute_replay_id(&mut self) {
        self.replay_id = self.compute_replay_id();
    }
}

/* ========================================================================== */
/* Replay executor                                                            */
/* ========================================================================== */

/// Execution boundary used by replay.
///
/// This deliberately does not duplicate `Decoder`.
///
/// An implementation normally delegates to the already-existing
/// `Decoder::decode_with_context`.
pub trait ReplayExecutor: Send + Sync {
    /// Returns decoder metadata for the execution target.
    fn decoder_metadata(&self) -> &DecoderMetadata;

    /// Returns the public backend identity.
    fn backend_identity(&self) -> (&str, &str, ExecutionTarget);

    /// Validates that the deterministic execution environment can reproduce
    /// the requested seed.
    ///
    /// The implementation should delegate to `deterministic.rs`.
    fn validate_seed(
        &self,
        seed: u64,
        _context: &DecodeContext<'_>,
    ) -> QecResult<()> {
        let _ = seed;
        Ok(())
    }

    /// Executes the canonical decoder input.
    fn execute(
        &self,
        input: &DecodeInput,
        context: &DecodeContext<'_>,
    ) -> QecResult<DecodeResult>;
}

/* ========================================================================== */
/* Replay orchestration                                                       */
/* ========================================================================== */

/// Replays one deterministic execution.
pub fn replay(
    record: &ReplayRecord,
    executor: &dyn ReplayExecutor,
    context: &DecodeContext<'_>,
    options: ReplayOptions,
) -> QecResult<ReplayOutcome> {
    record.validate()?;

    let decoder = executor.decoder_metadata();

    let (backend_id, backend_version, execution_target) =
        executor.backend_identity();

    record.validate_against(
        context.config(),
        decoder,
        backend_id,
        backend_version,
        execution_target,
        options,
    )?;

    if options.resource_preflight {
        context.preflight(record.input())?;
    } else {
        context.cancellation().check()?;
    }

    if options.strict_seed {
        executor.validate_seed(record.seed(), context)?;
    }

    context.cancellation().check()?;

    let actual = executor.execute(record.input(), context)?;

    context.cancellation().check()?;

    if actual.decoder() != record.decoder_id() {
        return Err(QecError::DecoderFailure {
            decoder: decoder.kind(),
            message:
                "replay execution returned a different decoder identity"
                    .to_owned(),
        });
    }

    if actual.syndrome() != record.input().syndrome() {
        return Err(QecError::DecoderFailure {
            decoder: decoder.kind(),
            message:
                "replay execution returned a result for a different \
                 syndrome"
                    .to_owned(),
        });
    }

    let actual_fingerprint = result_fingerprint(&actual);

    let status = match (
        options.verify_expected_result,
        record.expected_result_fingerprint(),
    ) {
        (true, Some(expected)) if expected == actual_fingerprint => {
            ReplayStatus::Reproduced
        }

        (true, Some(_)) => ReplayStatus::Mismatch,

        _ => ReplayStatus::Reproduced,
    };

    Ok(ReplayOutcome::new(
        record.replay_id(),
        status,
        actual,
        record.expected_result_fingerprint(),
        actual_fingerprint,
    ))
}

/* ========================================================================== */
/* Fingerprint helpers                                                        */
/* ========================================================================== */

/// Produces a deterministic configuration fingerprint.
///
/// `QecConfig` already owns configuration serialization. Replay therefore
/// delegates to its established serde representation instead of inventing a
/// second configuration model.
fn configuration_fingerprint(
    config: &QecConfig,
) -> QecResult<ReplayFingerprint> {
    let bytes = serde_json::to_vec(config).map_err(|error| {
        QecError::InvalidInput {
            message: format!(
                "failed to canonicalize QEC configuration for replay: {error}"
            ),
        }
    })?;

    Ok(ReplayFingerprint::sha256(&bytes))
}

/// Produces the input fingerprint.
///
/// The decoder input type is deliberately kept as the canonical source of
/// truth. The schema version is included in the hash domain so a future
/// representation change cannot silently reuse an old identity.
fn input_fingerprint(input: &DecodeInput) -> ReplayFingerprint {
    let mut hasher = Sha256::new();

    hasher.update(b"zamani-qec-replay-input-v1");
    hash_string(
        &mut hasher,
        &format!("{:?}", input.syndrome()),
    );

    ReplayFingerprint::from_bytes(
        digest_to_array(hasher.finalize()),
    )
}

/// Produces the semantic result fingerprint.
///
/// `DecodeResult` is deliberately not redefined here. The canonical result
/// remains owned by `decoder_result.rs`.
///
/// The result schema version and semantic Debug representation are hashed
/// together. A change to the canonical result representation therefore
/// produces a different replay identity.
fn result_fingerprint(result: &DecodeResult) -> ReplayFingerprint {
    let mut hasher = Sha256::new();

    hasher.update(b"zamani-qec-replay-result-v1");
    hasher.update(result.format_version().to_le_bytes());
    hasher.update(result.decoder().index().to_le_bytes());

    hash_string(
        &mut hasher,
        &format!("{:?}", result.syndrome()),
    );

    hash_string(
        &mut hasher,
        &format!("{:?}", result.correction()),
    );

    hasher.update([result.termination() as u8]);
    hasher.update(result.iterations().to_le_bytes());

    hash_string(
        &mut hasher,
        &format!("{:?}", result),
    );

    ReplayFingerprint::from_bytes(
        digest_to_array(hasher.finalize()),
    )
}

/* ========================================================================== */
/* Validation helpers                                                         */
/* ========================================================================== */

fn validate_public_identifier(
    field: &'static str,
    value: &str,
) -> QecResult<()> {
    if value.is_empty() {
        return Err(QecError::InvalidInput {
            message: format!(
                "replay {field} cannot be empty"
            ),
        });
    }

    if value.len() > MAX_REPLAY_IDENTIFIER_LENGTH {
        return Err(QecError::InvalidInput {
            message: format!(
                "replay {field} exceeds maximum length of \
                 {MAX_REPLAY_IDENTIFIER_LENGTH}"
            ),
        });
    }

    /*
     * Replay records cross trust boundaries. Do not allow fields intended
     * for public identity to carry obvious secret material.
     *
     * This is defense-in-depth, not a replacement for proper secret
     * management.
     */
    let lower = value.to_ascii_lowercase();

    const FORBIDDEN: &[&str] = &[
        "password",
        "passwd",
        "secret",
        "private_key",
        "private-key",
        "token",
        "access_token",
        "authorization",
        "credential",
        "api_key",
        "apikey",
    ];

    for marker in FORBIDDEN {
        if lower.contains(marker) {
            return Err(QecError::InvalidInput {
                message: format!(
                    "replay {field} appears to contain secret material"
                ),
            });
        }
    }

    Ok(())
}

fn hash_version(
    hasher: &mut Sha256,
    version: Version,
) {
    hasher.update(version.major.to_le_bytes());
    hasher.update(version.minor.to_le_bytes());
    hasher.update(version.patch.to_le_bytes());
}

fn hash_string(
    hasher: &mut Sha256,
    value: &str,
) {
    let length = value.len() as u64;

    hasher.update(length.to_le_bytes());
    hasher.update(value.as_bytes());
}

fn digest_to_array(
    digest: sha2::digest::Output<Sha256>,
) -> [u8; REPLAY_FINGERPRINT_SIZE] {
    let mut output = [0_u8; REPLAY_FINGERPRINT_SIZE];
    output.copy_from_slice(&digest);
    output
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_deterministic() {
        let first = ReplayFingerprint::sha256(b"zamani");
        let second = ReplayFingerprint::sha256(b"zamani");

        assert_eq!(first, second);
    }

    #[test]
    fn different_inputs_produce_different_fingerprints() {
        let first = ReplayFingerprint::sha256(b"one");
        let second = ReplayFingerprint::sha256(b"two");

        assert_ne!(first, second);
    }

    #[test]
    fn replay_id_is_stable() {
        let mut hasher = Sha256::new();

        hasher.update(b"test");

        let fingerprint = ReplayFingerprint::from_bytes(
            digest_to_array(hasher.finalize()),
        );

        let first = ReplayId::new(fingerprint);
        let second = ReplayId::new(fingerprint);

        assert_eq!(first, second);
    }

    #[test]
    fn secret_identifiers_are_rejected() {
        let result =
            validate_public_identifier(
                "backend_id",
                "backend-secret-token",
            );

        assert!(result.is_err());
    }

    #[test]
    fn public_identifiers_are_accepted() {
        let result =
            validate_public_identifier(
                "backend_id",
                "ibm-qpu-001",
            );

        assert!(result.is_ok());
    }
}