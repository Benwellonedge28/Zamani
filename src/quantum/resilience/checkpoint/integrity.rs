//! Zamani Quantum Resilience — Checkpoint Integrity
//!
//! Path:
//!     src/quantum/resilience/checkpoint/integrity.rs
//!
//! Purpose:
//!     Production-grade cryptographic integrity and authenticity contracts for
//!     checkpoint metadata and checkpoint payload artifacts.
//!
//! ============================================================================
//! Architectural ownership
//! ============================================================================
//!
//! checkpoint.rs
//!     Owns:
//!         - checkpoint semantic identity;
//!         - checkpoint lifecycle;
//!         - checkpoint boundary;
//!         - checkpoint state classification;
//!         - payload descriptors;
//!         - resource scope;
//!         - checkpoint-level validation.
//!
//! manifest.rs
//!     Owns:
//!         - complete checkpoint manifests;
//!         - artifact enumeration;
//!         - artifact ordering;
//!         - manifest structure.
//!
//! snapshot.rs
//!     Owns:
//!         - snapshot-specific state descriptors;
//!         - provider/runtime snapshot metadata.
//!
//! storage.rs
//!     Owns:
//!         - physical persistence;
//!         - object addressing;
//!         - streaming storage I/O;
//!         - conditional writes;
//!         - storage lifecycle.
//!
//! integrity.rs
//!     Owns:
//!         - cryptographic digest algorithms;
//!         - streaming digest calculation;
//!         - digest verification;
//!         - authenticated integrity records;
//!         - signature verification;
//!         - integrity-policy evaluation;
//!         - canonical integrity comparison;
//!         - integrity evidence.
//!
//! compatibility.rs
//!     Owns:
//!         - checkpoint restore compatibility;
//!         - target capability compatibility;
//!         - schema compatibility.
//!
//! recovery/*
//!     Owns:
//!         - restore ordering;
//!         - rollback;
//!         - resume;
//!         - recovery execution.
//!
//! verification/*
//!     Owns:
//!         - semantic verification;
//!         - result verification;
//!         - final acceptance.
//!
//! ============================================================================
//! Critical semantic boundary
//! ============================================================================
//!
//! A valid digest proves only that supplied bytes match the supplied digest.
//!
//! It does NOT prove:
//!
//!     - that the producer was trustworthy;
//!     - that the checkpoint is semantically correct;
//!     - that the quantum state is physically reconstructible;
//!     - that the target can restore the checkpoint;
//!     - that the checkpoint represents the intended program.
//!
//! Therefore integrity verification MUST remain separate from:
//!
//!     semantic verification
//!     checkpoint compatibility
//!     recovery authorization
//!     result acceptance
//!
//! ============================================================================
//! Quantum-state safety
//! ============================================================================
//!
//! This module hashes bytes or byte streams.
//!
//! It does NOT imply that an arbitrary unknown quantum state can be converted
//! into bytes. Only explicitly supplied/reconstructible checkpoint artifacts
//! may be hashed.
//!
//! Provider-managed quantum snapshots are valid integrity subjects only when
//! the provider/runtime supplies the corresponding byte representation or
//! digestable artifact.
//!
//! ============================================================================
//! Write once / scale everywhere
//! ============================================================================
//!
//! This module has NO architectural limits on:
//!
//!     qubits
//!     logical qubits
//!     physical qubits
//!     operations
//!     artifacts
//!     checkpoints
//!     payload size
//!     number of chunks
//!
//! Large checkpoint artifacts MUST be processed incrementally through the
//! streaming interfaces rather than materialized in memory.
//!
//! `u64` is used for byte counts and stream positions.
//!
//! No provider name, machine size, qubit index, or fixed artifact count is
//! encoded in this module.
//!
//! ============================================================================
//! Cryptographic design
//! ============================================================================
//!
//! Built-in digest algorithms:
//!
//!     SHA-256
//!     SHA3-256
//!
//! Built-in signature algorithm:
//!
//!     Ed25519
//!
//! These are concrete algorithms supported by this implementation, while the
//! integrity model itself remains algorithm-explicit and versionable.
//!
//! A future algorithm can be added without changing the checkpoint semantic
//! model.
//!
//! Signing keys are NEVER stored by this module.
//!
//! Verification requires only public-key material supplied by the caller or
//! trusted key-management layer.
//!
//! ============================================================================
//! Determinism
//! ============================================================================
//!
//! Digest calculation is deterministic for identical byte sequences.
//!
//! This module:
//!
//!     - does not read the clock;
//!     - does not generate identifiers;
//!     - does not access the filesystem;
//!     - does not access the network;
//!     - does not access environment variables;
//!     - does not use randomness;
//!     - does not access global mutable state.
//!
//! Timestamps and identities are supplied by higher layers.
//!
//! ============================================================================
//! Security model
//! ============================================================================
//!
//! Integrity verification is fail-closed.
//!
//! Invalid:
//!
//!     algorithms
//!     digest lengths
//!     signatures
//!     public keys
//!     integrity records
//!     policy requirements
//!
//! are errors, never silently downgraded to "unverified" success.
//!
//! Signature verification is distinct from digest verification.
//!
//! Digest:
//!     "Did these bytes change?"
//!
//! Signature:
//!     "Was this digest/value authorized by the holder of the signing key?"
//!
//! Both may be required by policy.
//!
//! ============================================================================
//! Rust contract
//! ============================================================================
//!
//!     Rust 1.97 / 1.97.1
//!     Rust 2021
//!     stable Rust
//!     no nightly features
//!     no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::fmt;
use std::io::{self, Read};
use std::sync::Arc;

use ed25519_dalek::{
    Signature,
    VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as Sha2Digest, Sha256};
use sha3::Sha3_256;
use thiserror::Error;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use super::checkpoint::ArtifactId;

// ============================================================================
// Schema identity
// ============================================================================

/// Stable schema identifier for checkpoint integrity contracts.
pub const CHECKPOINT_INTEGRITY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint.integrity";

/// Semantic version of the integrity schema.
pub const CHECKPOINT_INTEGRITY_SCHEMA_VERSION: u16 = 1;

/// SHA-256 digest size in bytes.
pub const SHA256_DIGEST_BYTES: usize = 32;

/// SHA3-256 digest size in bytes.
pub const SHA3_256_DIGEST_BYTES: usize = 32;

/// Ed25519 public-key size in bytes.
pub const ED25519_PUBLIC_KEY_BYTES: usize = 32;

/// Ed25519 signature size in bytes.
pub const ED25519_SIGNATURE_BYTES: usize = 64;

// ============================================================================
// Integrity-specific error
// ============================================================================

/// Integrity-specific failure.
///
/// Most resilience code should expose [`ResilienceError`]. This error exists
/// for operations where callers need the precise integrity failure while still
/// allowing conversion into the canonical resilience error model.
#[derive(Debug, Error)]
pub enum IntegrityError {
    /// Invalid digest representation.
    #[error("invalid digest representation")]
    InvalidDigest,

    /// Unsupported digest algorithm.
    #[error("unsupported digest algorithm: {0}")]
    UnsupportedDigestAlgorithm(String),

    /// Invalid signature representation.
    #[error("invalid signature representation")]
    InvalidSignature,

    /// Invalid public key representation.
    #[error("invalid public key representation")]
    InvalidPublicKey,

    /// Signature verification failed.
    #[error("signature verification failed")]
    SignatureVerificationFailed,

    /// Digest verification failed.
    #[error("digest verification failed")]
    DigestMismatch,

    /// Integrity record is structurally invalid.
    #[error("invalid integrity record: {0}")]
    InvalidRecord(String),

    /// Integrity policy is structurally invalid.
    #[error("invalid integrity policy: {0}")]
    InvalidPolicy(String),

    /// Required integrity evidence is missing.
    #[error("required integrity evidence is missing")]
    MissingEvidence,

    /// Input/output error while streaming bytes.
    #[error("integrity stream I/O error")]
    Io(#[source] io::Error),

    /// Arithmetic overflow.
    #[error("integrity byte-count arithmetic overflow")]
    ArithmeticOverflow,

    /// Underlying resilience error.
    #[error("resilience error")]
    Resilience(#[from] ResilienceError),
}

impl IntegrityError {
    /// Converts the integrity failure into the canonical resilience error.
    #[must_use]
    pub fn into_resilience_error(self) -> ResilienceError {
        match self {
            Self::Resilience(error) => error,

            Self::DigestMismatch => ResilienceError::new(
                ResilienceErrorCode::CheckpointIntegrityFailed,
                "checkpoint digest verification failed",
            ),

            Self::SignatureVerificationFailed => ResilienceError::new(
                ResilienceErrorCode::CheckpointIntegrityFailed,
                "checkpoint signature verification failed",
            ),

            Self::MissingEvidence => ResilienceError::new(
                ResilienceErrorCode::CheckpointIntegrityFailed,
                "required checkpoint integrity evidence is missing",
            ),

            Self::UnsupportedDigestAlgorithm(_) => ResilienceError::new(
                ResilienceErrorCode::CheckpointIntegrityFailed,
                "checkpoint uses an unsupported integrity algorithm",
            ),

            Self::InvalidDigest
            | Self::InvalidSignature
            | Self::InvalidPublicKey
            | Self::InvalidRecord(_)
            | Self::InvalidPolicy(_) => ResilienceError::new(
                ResilienceErrorCode::InvalidCheckpoint,
                "checkpoint integrity metadata is invalid",
            ),

            Self::Io(_) => ResilienceError::new(
                ResilienceErrorCode::CheckpointStorageUnavailable,
                "checkpoint integrity input could not be read",
            ),

            Self::ArithmeticOverflow => ResilienceError::new(
                ResilienceErrorCode::ArithmeticOverflow,
                "checkpoint integrity byte count overflowed",
            ),
        }
    }
}

impl From<IntegrityError> for ResilienceError {
    fn from(error: IntegrityError) -> Self {
        error.into_resilience_error()
    }
}

// ============================================================================
// Digest algorithm
// ============================================================================

/// Cryptographic digest algorithm.
///
/// The algorithm is part of the digest identity. A digest MUST NOT be
/// interpreted without its algorithm.
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
pub enum DigestAlgorithm {
    /// SHA-256.
    Sha256,

    /// SHA3-256.
    Sha3_256,
}

impl DigestAlgorithm {
    /// Stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha-256",
            Self::Sha3_256 => "sha3-256",
        }
    }

    /// Digest length in bytes.
    #[must_use]
    pub const fn digest_length(self) -> usize {
        match self {
            Self::Sha256 | Self::Sha3_256 => 32,
        }
    }

    /// Whether the implementation currently supports this algorithm.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        true
    }
}

impl fmt::Display for DigestAlgorithm {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Digest bytes
// ============================================================================

/// Fixed-size cryptographic digest.
///
/// SHA-256 and SHA3-256 both produce 32-byte digests, allowing a single
/// representation without heap allocation.
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
pub struct DigestBytes([u8; 32]);

impl DigestBytes {
    /// Creates a digest from exactly 32 bytes.
    #[must_use]
    pub const fn from_array(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns a copied digest array.
    #[must_use]
    pub const fn into_array(self) -> [u8; 32] {
        self.0
    }

    /// Creates a digest from a byte slice.
    pub fn from_slice(
        bytes: &[u8],
    ) -> Result<Self, IntegrityError> {
        if bytes.len() != 32 {
            return Err(IntegrityError::InvalidDigest);
        }

        let mut value = [0_u8; 32];
        value.copy_from_slice(bytes);

        Ok(Self(value))
    }

    /// Returns lowercase hexadecimal representation.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parses a hexadecimal digest.
    pub fn from_hex(value: &str) -> Result<Self, IntegrityError> {
        let decoded =
            hex::decode(value).map_err(|_| IntegrityError::InvalidDigest)?;

        Self::from_slice(&decoded)
    }
}

impl fmt::Display for DigestBytes {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

// ============================================================================
// Digest identity
// ============================================================================

/// Complete cryptographic digest identity.
///
/// Algorithm and digest are inseparable.
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
pub struct Digest {
    /// Algorithm used to produce the digest.
    pub algorithm: DigestAlgorithm,

    /// Resulting digest.
    pub value: DigestBytes,
}

impl Digest {
    /// Creates a digest identity.
    #[must_use]
    pub const fn new(
        algorithm: DigestAlgorithm,
        value: DigestBytes,
    ) -> Self {
        Self { algorithm, value }
    }

    /// Returns the algorithm.
    #[must_use]
    pub const fn algorithm(self) -> DigestAlgorithm {
        self.algorithm
    }

    /// Returns the digest bytes.
    #[must_use]
    pub const fn value(self) -> DigestBytes {
        self.value
    }

    /// Returns the digest as hexadecimal.
    #[must_use]
    pub fn to_hex(&self) -> String {
        self.value.to_hex()
    }
}

// ============================================================================
// Digest hasher abstraction
// ============================================================================

/// Incremental cryptographic digest calculator.
///
/// This trait is intentionally independent of storage.
///
/// A storage backend can feed chunks into this interface without materializing
/// a complete checkpoint artifact in memory.
pub trait DigestHasher: Send {
    /// Returns the algorithm used by this hasher.
    fn algorithm(&self) -> DigestAlgorithm;

    /// Adds bytes to the digest.
    fn update(
        &mut self,
        bytes: &[u8],
    );

    /// Finalizes the digest.
    fn finalize(self: Box<Self>) -> Digest;
}

/// SHA-256 streaming hasher.
#[derive(Debug)]
pub struct Sha256Hasher {
    hasher: Sha256,
}

impl Sha256Hasher {
    /// Creates a new SHA-256 hasher.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }
}

impl Default for Sha256Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl DigestHasher for Sha256Hasher {
    fn algorithm(&self) -> DigestAlgorithm {
        DigestAlgorithm::Sha256
    }

    fn update(
        &mut self,
        bytes: &[u8],
    ) {
        Sha2Digest::update(&mut self.hasher, bytes);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let output = Sha2Digest::finalize(self.hasher);

        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(output.as_slice());

        Digest::new(
            DigestAlgorithm::Sha256,
            DigestBytes::from_array(bytes),
        )
    }
}

/// SHA3-256 streaming hasher.
#[derive(Debug)]
pub struct Sha3_256Hasher {
    hasher: Sha3_256,
}

impl Sha3_256Hasher {
    /// Creates a new SHA3-256 hasher.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hasher: Sha3_256::new(),
        }
    }
}

impl Default for Sha3_256Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl DigestHasher for Sha3_256Hasher {
    fn algorithm(&self) -> DigestAlgorithm {
        DigestAlgorithm::Sha3_256
    }

    fn update(
        &mut self,
        bytes: &[u8],
    ) {
        sha3::Digest::update(&mut self.hasher, bytes);
    }

    fn finalize(self: Box<Self>) -> Digest {
        let output = sha3::Digest::finalize(self.hasher);

        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(output.as_slice());

        Digest::new(
            DigestAlgorithm::Sha3_256,
            DigestBytes::from_array(bytes),
        )
    }
}

// ============================================================================
// Hasher factory
// ============================================================================

/// Creates an incremental digest calculator.
pub fn hasher_for(
    algorithm: DigestAlgorithm,
) -> Box<dyn DigestHasher> {
    match algorithm {
        DigestAlgorithm::Sha256 => Box::new(Sha256Hasher::new()),
        DigestAlgorithm::Sha3_256 => Box::new(Sha3_256Hasher::new()),
    }
}

// ============================================================================
// Digest calculation
// ============================================================================

/// Calculates the digest of a byte slice.
///
/// This is suitable for metadata or small artifacts.
#[must_use]
pub fn digest_bytes(
    algorithm: DigestAlgorithm,
    bytes: &[u8],
) -> Digest {
    let mut hasher = hasher_for(algorithm);

    hasher.update(bytes);

    hasher.finalize()
}

/// Calculates the digest of a reader incrementally.
///
/// The reader is consumed in bounded chunks. No complete artifact is
/// materialized in memory.
pub fn digest_reader<R: Read>(
    algorithm: DigestAlgorithm,
    mut reader: R,
) -> Result<StreamDigestResult, IntegrityError> {
    let mut hasher = hasher_for(algorithm);

    let mut buffer = [0_u8; 16 * 1024];
    let mut byte_count = 0_u64;

    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(IntegrityError::Io)?;

        if read == 0 {
            break;
        }

        hasher.update(&buffer[..read]);

        byte_count = byte_count
            .checked_add(
                u64::try_from(read)
                    .map_err(|_| IntegrityError::ArithmeticOverflow)?,
            )
            .ok_or(IntegrityError::ArithmeticOverflow)?;
    }

    Ok(StreamDigestResult {
        digest: hasher.finalize(),
        byte_count,
    })
}

/// Result of streaming digest calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamDigestResult {
    /// Calculated digest.
    pub digest: Digest,

    /// Number of bytes consumed.
    pub byte_count: u64,
}

// ============================================================================
// Constant-time digest comparison
// ============================================================================

/// Compares digest bytes without early exit.
///
/// This avoids making digest mismatch timing depend on the first differing
/// byte.
#[must_use]
pub fn constant_time_digest_eq(
    left: &DigestBytes,
    right: &DigestBytes,
) -> bool {
    let mut difference = 0_u8;

    for index in 0..32 {
        difference |= left.0[index] ^ right.0[index];
    }

    difference == 0
}

/// Verifies an expected digest against actual digest bytes.
#[must_use]
pub fn verify_digest(
    expected: &Digest,
    actual: &Digest,
) -> bool {
    expected.algorithm == actual.algorithm
        && constant_time_digest_eq(
            &expected.value,
            &actual.value,
        )
}

// ============================================================================
// Artifact integrity
// ============================================================================

/// Integrity evidence for one checkpoint artifact.
///
/// This is the type `manifest.rs` should use for artifact-level integrity
/// references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactIntegrity {
    /// Semantic artifact identity.
    pub artifact_id: ArtifactId,

    /// Cryptographic identity of the artifact bytes.
    pub digest: Digest,

    /// Declared byte length, when known.
    pub byte_length: Option<u64>,

    /// Optional content-generation/version identity supplied by storage.
    ///
    /// This value is opaque and MUST NOT be interpreted as a cryptographic
    /// digest.
    pub content_version: Option<Arc<str>>,
}

impl ArtifactIntegrity {
    /// Creates artifact integrity evidence.
    #[must_use]
    pub fn new(
        artifact_id: ArtifactId,
        digest: Digest,
    ) -> Self {
        Self {
            artifact_id,
            digest,
            byte_length: None,
            content_version: None,
        }
    }

    /// Adds a declared byte length.
    #[must_use]
    pub fn with_byte_length(
        mut self,
        byte_length: u64,
    ) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Adds an opaque storage/content version.
    #[must_use]
    pub fn with_content_version(
        mut self,
        version: impl Into<Arc<str>>,
    ) -> Result<Self, IntegrityError> {
        let version = version.into();

        if version.trim().is_empty() {
            return Err(IntegrityError::InvalidRecord(
                "content version must not be empty".to_owned(),
            ));
        }

        self.content_version = Some(version);

        Ok(self)
    }

    /// Verifies a calculated digest.
    #[must_use]
    pub fn verify(
        &self,
        actual: &Digest,
    ) -> bool {
        verify_digest(&self.digest, actual)
    }

    /// Validates structural invariants.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        if !self.digest.algorithm.is_supported() {
            return Err(
                IntegrityError::UnsupportedDigestAlgorithm(
                    self.digest.algorithm.as_str().to_owned(),
                ),
            );
        }

        if let Some(version) = &self.content_version {
            if version.trim().is_empty() {
                return Err(IntegrityError::InvalidRecord(
                    "content version must not be empty".to_owned(),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Checkpoint integrity root
// ============================================================================

/// Integrity commitment for a complete checkpoint manifest.
///
/// The actual manifest remains owned by `manifest.rs`.
///
/// This type stores only the resulting commitment. It does not prescribe a
/// particular Merkle-tree construction, allowing the manifest/integrity
/// implementation to evolve without changing checkpoint semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointIntegrityRoot {
    /// Cryptographic digest of the canonical manifest commitment.
    pub digest: Digest,

    /// Number of artifacts represented by the commitment, when known.
    pub artifact_count: Option<u64>,

    /// Total bytes represented by the commitment, when known.
    pub total_byte_length: Option<u64>,
}

impl CheckpointIntegrityRoot {
    /// Creates a root commitment.
    #[must_use]
    pub const fn new(
        digest: Digest,
    ) -> Self {
        Self {
            digest,
            artifact_count: None,
            total_byte_length: None,
        }
    }

    /// Adds artifact count metadata.
    #[must_use]
    pub const fn with_artifact_count(
        mut self,
        artifact_count: u64,
    ) -> Self {
        self.artifact_count = Some(artifact_count);
        self
    }

    /// Adds total byte metadata.
    #[must_use]
    pub const fn with_total_byte_length(
        mut self,
        total_byte_length: u64,
    ) -> Self {
        self.total_byte_length = Some(total_byte_length);
        self
    }

    /// Validates the commitment metadata.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        if !self.digest.algorithm.is_supported() {
            return Err(
                IntegrityError::UnsupportedDigestAlgorithm(
                    self.digest.algorithm.as_str().to_owned(),
                ),
            );
        }

        Ok(())
    }
}

// ============================================================================
// Signature algorithm
// ============================================================================

/// Supported digital-signature algorithm.
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
pub enum SignatureAlgorithm {
    /// Ed25519.
    Ed25519,
}

impl SignatureAlgorithm {
    /// Stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for SignatureAlgorithm {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Public-key identity
// ============================================================================

/// Public verification key.
///
/// Private signing keys never enter this data structure.
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
pub struct VerificationKey {
    /// Signature algorithm.
    pub algorithm: SignatureAlgorithm,

    /// Opaque stable key identity.
    pub key_id: Arc<str>,

    /// Raw public key bytes.
    pub public_key: [u8; ED25519_PUBLIC_KEY_BYTES],
}

impl VerificationKey {
    /// Creates an Ed25519 verification key.
    pub fn ed25519(
        key_id: impl Into<Arc<str>>,
        public_key: [u8; ED25519_PUBLIC_KEY_BYTES],
    ) -> Result<Self, IntegrityError> {
        let key_id = key_id.into();

        if key_id.trim().is_empty() {
            return Err(IntegrityError::InvalidRecord(
                "verification key ID must not be empty".to_owned(),
            ));
        }

        VerifyingKey::from_bytes(&public_key)
            .map_err(|_| IntegrityError::InvalidPublicKey)?;

        Ok(Self {
            algorithm: SignatureAlgorithm::Ed25519,
            key_id,
            public_key,
        })
    }

    /// Returns the public key bytes.
    #[must_use]
    pub const fn public_key(&self) -> &[u8; ED25519_PUBLIC_KEY_BYTES] {
        &self.public_key
    }

    /// Validates the public key.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        match self.algorithm {
            SignatureAlgorithm::Ed25519 => {
                VerifyingKey::from_bytes(&self.public_key)
                    .map_err(|_| IntegrityError::InvalidPublicKey)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// Signature
// ============================================================================

/// Digital signature over explicitly defined integrity material.
///
/// The signed message is NOT implicitly "whatever happened to be available".
/// Callers must construct the canonical signing bytes using the surrounding
/// serialization/manifest contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegritySignature {
    /// Signature algorithm.
    pub algorithm: SignatureAlgorithm,

    /// Key identity used for verification.
    pub key_id: Arc<str>,

    /// Public verification key.
    pub public_key: [u8; ED25519_PUBLIC_KEY_BYTES],

    /// Signature bytes.
    pub signature: [u8; ED25519_SIGNATURE_BYTES],
}

impl IntegritySignature {
    /// Creates an Ed25519 integrity signature record.
    pub fn ed25519(
        key_id: impl Into<Arc<str>>,
        public_key: [u8; ED25519_PUBLIC_KEY_BYTES],
        signature: [u8; ED25519_SIGNATURE_BYTES],
    ) -> Result<Self, IntegrityError> {
        let key_id = key_id.into();

        if key_id.trim().is_empty() {
            return Err(IntegrityError::InvalidRecord(
                "signature key ID must not be empty".to_owned(),
            ));
        }

        VerifyingKey::from_bytes(&public_key)
            .map_err(|_| IntegrityError::InvalidPublicKey)?;

        Ok(Self {
            algorithm: SignatureAlgorithm::Ed25519,
            key_id,
            public_key,
            signature,
        })
    }

    /// Verifies this signature over the supplied message.
    pub fn verify(
        &self,
        message: &[u8],
    ) -> Result<(), IntegrityError> {
        match self.algorithm {
            SignatureAlgorithm::Ed25519 => {
                let key = VerifyingKey::from_bytes(&self.public_key)
                    .map_err(|_| IntegrityError::InvalidPublicKey)?;

                let signature = Signature::from_bytes(&self.signature);

                ed25519_dalek::Verifier::verify(
                    &key,
                    message,
                    &signature,
                )
                .map_err(|_| IntegrityError::SignatureVerificationFailed)
            }
        }
    }

    /// Validates the structural representation.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        if self.key_id.trim().is_empty() {
            return Err(IntegrityError::InvalidRecord(
                "signature key ID must not be empty".to_owned(),
            ));
        }

        VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_| IntegrityError::InvalidPublicKey)?;

        Ok(())
    }
}

// ============================================================================
// Integrity evidence
// ============================================================================

/// Complete integrity evidence for a checkpoint or checkpoint commitment.
///
/// This type deliberately does not own checkpoint payload bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityEvidence {
    /// Primary cryptographic commitment.
    pub digest: Digest,

    /// Optional digital signature over canonical integrity material.
    pub signature: Option<IntegritySignature>,

    /// Optional artifact identity if the evidence is artifact-specific.
    pub artifact_id: Option<ArtifactId>,

    /// Declared byte length if the subject has a byte representation.
    pub byte_length: Option<u64>,
}

impl IntegrityEvidence {
    /// Creates unsigned integrity evidence.
    #[must_use]
    pub const fn unsigned(
        digest: Digest,
    ) -> Self {
        Self {
            digest,
            signature: None,
            artifact_id: None,
            byte_length: None,
        }
    }

    /// Adds an artifact identity.
    #[must_use]
    pub fn for_artifact(
        mut self,
        artifact_id: ArtifactId,
    ) -> Self {
        self.artifact_id = Some(artifact_id);
        self
    }

    /// Adds byte-length evidence.
    #[must_use]
    pub const fn with_byte_length(
        mut self,
        byte_length: u64,
    ) -> Self {
        self.byte_length = Some(byte_length);
        self
    }

    /// Adds a signature.
    #[must_use]
    pub fn with_signature(
        mut self,
        signature: IntegritySignature,
    ) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Validates the evidence structure.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        if !self.digest.algorithm.is_supported() {
            return Err(
                IntegrityError::UnsupportedDigestAlgorithm(
                    self.digest.algorithm.as_str().to_owned(),
                ),
            );
        }

        if let Some(signature) = &self.signature {
            signature.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// Integrity requirements
// ============================================================================

/// Required level of integrity protection.
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
pub enum IntegrityRequirement {
    /// Integrity is optional.
    Optional,

    /// Digest verification is mandatory.
    DigestRequired,

    /// A trusted digital signature is mandatory.
    SignatureRequired,

    /// Both digest and signature are mandatory.
    DigestAndSignatureRequired,
}

impl IntegrityRequirement {
    /// Returns whether a digest is required.
    #[must_use]
    pub const fn requires_digest(self) -> bool {
        matches!(
            self,
            Self::DigestRequired
                | Self::DigestAndSignatureRequired
        )
    }

    /// Returns whether a signature is required.
    #[must_use]
    pub const fn requires_signature(self) -> bool {
        matches!(
            self,
            Self::SignatureRequired
                | Self::DigestAndSignatureRequired
        )
    }
}

// ============================================================================
// Integrity policy
// ============================================================================

/// Policy controlling whether integrity evidence is sufficient.
///
/// This is descriptive and evaluative. It does not perform storage or
/// recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityPolicy {
    /// Minimum required protection.
    pub requirement: IntegrityRequirement,

    /// Allowed digest algorithms.
    ///
    /// An empty collection is invalid because it would make digest policy
    /// ambiguous.
    pub allowed_digest_algorithms: Arc<[DigestAlgorithm]>,

    /// Allowed signature algorithms.
    ///
    /// An empty collection is valid when signatures are not required.
    pub allowed_signature_algorithms: Arc<[SignatureAlgorithm]>,

    /// Whether a signature key must match the expected key identity.
    pub require_expected_key_id: bool,

    /// Expected signing key identity, when required.
    pub expected_key_id: Option<Arc<str>>,
}

impl IntegrityPolicy {
    /// Creates a digest-only policy for one or more algorithms.
    pub fn new(
        requirement: IntegrityRequirement,
        allowed_digest_algorithms: impl IntoIterator<Item = DigestAlgorithm>,
    ) -> Result<Self, IntegrityError> {
        let mut algorithms: Vec<DigestAlgorithm> =
            allowed_digest_algorithms.into_iter().collect();

        algorithms.sort_unstable();
        algorithms.dedup();

        if algorithms.is_empty() {
            return Err(IntegrityError::InvalidPolicy(
                "at least one digest algorithm must be allowed"
                    .to_owned(),
            ));
        }

        Ok(Self {
            requirement,
            allowed_digest_algorithms: algorithms.into(),
            allowed_signature_algorithms: Arc::from(
                Vec::<SignatureAlgorithm>::new(),
            ),
            require_expected_key_id: false,
            expected_key_id: None,
        })
    }

    /// Adds allowed signature algorithms.
    pub fn with_signature_algorithms(
        mut self,
        algorithms: impl IntoIterator<Item = SignatureAlgorithm>,
    ) -> Result<Self, IntegrityError> {
        let mut values: Vec<SignatureAlgorithm> =
            algorithms.into_iter().collect();

        values.sort_unstable();
        values.dedup();

        self.allowed_signature_algorithms = values.into();

        Ok(self)
    }

    /// Requires a particular verification-key identity.
    pub fn with_expected_key_id(
        mut self,
        key_id: impl Into<Arc<str>>,
    ) -> Result<Self, IntegrityError> {
        let key_id = key_id.into();

        if key_id.trim().is_empty() {
            return Err(IntegrityError::InvalidPolicy(
                "expected key ID must not be empty".to_owned(),
            ));
        }

        self.require_expected_key_id = true;
        self.expected_key_id = Some(key_id);

        Ok(self)
    }

    /// Validates policy invariants.
    pub fn validate(&self) -> Result<(), IntegrityError> {
        if self.allowed_digest_algorithms.is_empty() {
            return Err(IntegrityError::InvalidPolicy(
                "no digest algorithm is allowed".to_owned(),
            ));
        }

        if self.requirement.requires_signature()
            && self.allowed_signature_algorithms.is_empty()
        {
            return Err(IntegrityError::InvalidPolicy(
                "signature is required but no signature algorithm is allowed"
                    .to_owned(),
            ));
        }

        if self.require_expected_key_id
            && self.expected_key_id.is_none()
        {
            return Err(IntegrityError::InvalidPolicy(
                "expected key ID is required but not supplied"
                    .to_owned(),
            ));
        }

        Ok(())
    }

    /// Determines whether a digest algorithm is allowed.
    #[must_use]
    pub fn allows_digest(
        &self,
        algorithm: DigestAlgorithm,
    ) -> bool {
        self.allowed_digest_algorithms
            .contains(&algorithm)
    }

    /// Determines whether a signature algorithm is allowed.
    #[must_use]
    pub fn allows_signature(
        &self,
        algorithm: SignatureAlgorithm,
    ) -> bool {
        self.allowed_signature_algorithms
            .contains(&algorithm)
    }
}

// ============================================================================
// Integrity verification outcome
// ============================================================================

/// Result of integrity-policy evaluation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum IntegrityStatus {
    /// All required integrity checks succeeded.
    Verified,

    /// Integrity was not required.
    NotRequired,

    /// Digest was verified but no signature was required.
    DigestVerified,

    /// Signature was verified but digest was not required.
    SignatureVerified,

    /// Evidence was present but insufficient for the requested policy.
    Insufficient,
}

impl IntegrityStatus {
    /// Whether this status satisfies a strict acceptance gate.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::Verified
                | Self::DigestVerified
                | Self::SignatureVerified
        )
    }
}

// ============================================================================
// Verification result
// ============================================================================

/// Detailed integrity verification result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityVerification {
    /// Final policy status.
    pub status: IntegrityStatus,

    /// Expected digest, if one was supplied.
    pub expected_digest: Option<Digest>,

    /// Calculated/observed digest, if available.
    pub actual_digest: Option<Digest>,

    /// Whether signature verification succeeded.
    pub signature_verified: bool,

    /// Number of bytes processed, when streaming verification was used.
    pub byte_length: Option<u64>,
}

impl IntegrityVerification {
    /// Returns whether the result satisfies the requested policy.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.status.is_verified()
    }
}

// ============================================================================
// Integrity verifier
// ============================================================================

/// Provider-neutral integrity verifier.
///
/// It has no storage dependency and no checkpoint recovery dependency.
#[derive(Debug, Clone)]
pub struct IntegrityVerifier {
    policy: IntegrityPolicy,
}

impl IntegrityVerifier {
    /// Creates a verifier from a validated policy.
    pub fn new(
        policy: IntegrityPolicy,
    ) -> Result<Self, IntegrityError> {
        policy.validate()?;

        Ok(Self { policy })
    }

    /// Returns the policy.
    #[must_use]
    pub fn policy(&self) -> &IntegrityPolicy {
        &self.policy
    }

    /// Verifies digest evidence for an in-memory byte slice.
    pub fn verify_bytes(
        &self,
        bytes: &[u8],
        expected: Option<&Digest>,
    ) -> Result<IntegrityVerification, IntegrityError> {
        let algorithm = expected
            .map(|digest| digest.algorithm)
            .or_else(|| {
                self.policy
                    .allowed_digest_algorithms
                    .first()
                    .copied()
            })
            .ok_or_else(|| {
                IntegrityError::InvalidPolicy(
                    "no digest algorithm available".to_owned(),
                )
            })?;

        if !self.policy.allows_digest(algorithm) {
            return Err(IntegrityError::InvalidPolicy(
                "digest algorithm is not allowed by policy".to_owned(),
            ));
        }

        let actual = digest_bytes(algorithm, bytes);

        self.evaluate(
            expected,
            Some(actual),
            None,
            Some(
                u64::try_from(bytes.len())
                    .map_err(|_| IntegrityError::ArithmeticOverflow)?,
            ),
        )
    }

    /// Verifies a streamed artifact.
    pub fn verify_reader<R: Read>(
        &self,
        reader: R,
        expected: Option<&Digest>,
    ) -> Result<IntegrityVerification, IntegrityError> {
        let algorithm = expected
            .map(|digest| digest.algorithm)
            .or_else(|| {
                self.policy
                    .allowed_digest_algorithms
                    .first()
                    .copied()
            })
            .ok_or_else(|| {
                IntegrityError::InvalidPolicy(
                    "no digest algorithm available".to_owned(),
                )
            })?;

        if !self.policy.allows_digest(algorithm) {
            return Err(IntegrityError::InvalidPolicy(
                "digest algorithm is not allowed by policy".to_owned(),
            ));
        }

        let result = digest_reader(algorithm, reader)?;

        self.evaluate(
            expected,
            Some(result.digest),
            None,
            Some(result.byte_count),
        )
    }

    /// Verifies a complete integrity evidence record against bytes.
    pub fn verify_evidence(
        &self,
        bytes: &[u8],
        evidence: &IntegrityEvidence,
        signing_message: Option<&[u8]>,
    ) -> Result<IntegrityVerification, IntegrityError> {
        evidence.validate()?;

        if !self.policy.allows_digest(evidence.digest.algorithm) {
            return Err(IntegrityError::InvalidPolicy(
                "evidence digest algorithm is not allowed"
                    .to_owned(),
            ));
        }

        let actual = digest_bytes(
            evidence.digest.algorithm,
            bytes,
        );

        let signature_verified =
            if let Some(signature) = &evidence.signature {
                if !self
                    .policy
                    .allows_signature(signature.algorithm)
                {
                    return Err(IntegrityError::InvalidPolicy(
                        "signature algorithm is not allowed"
                            .to_owned(),
                    ));
                }

                if self.policy.require_expected_key_id
                    && self.policy.expected_key_id.as_deref()
                        != Some(signature.key_id.as_ref())
                {
                    return Err(
                        IntegrityError::SignatureVerificationFailed,
                    );
                }

                let message = signing_message.ok_or(
                    IntegrityError::MissingEvidence,
                )?;

                signature.verify(message)?;

                true
            } else {
                false
            };

        let digest_matches =
            verify_digest(&evidence.digest, &actual);

        if !digest_matches {
            return Err(IntegrityError::DigestMismatch);
        }

        if self.policy.requirement.requires_signature()
            && !signature_verified
        {
            return Err(IntegrityError::MissingEvidence);
        }

        let status = match (
            self.policy.requirement,
            signature_verified,
        ) {
            (
                IntegrityRequirement::Optional,
                false,
            ) => IntegrityStatus::DigestVerified,

            (
                IntegrityRequirement::DigestRequired,
                false,
            ) => IntegrityStatus::DigestVerified,

            (
                IntegrityRequirement::SignatureRequired,
                true,
            ) => IntegrityStatus::SignatureVerified,

            (
                IntegrityRequirement::DigestAndSignatureRequired,
                true,
            ) => IntegrityStatus::Verified,

            _ => IntegrityStatus::Insufficient,
        };

        Ok(IntegrityVerification {
            status,
            expected_digest: Some(evidence.digest),
            actual_digest: Some(actual),
            signature_verified,
            byte_length: Some(
                u64::try_from(bytes.len())
                    .map_err(|_| IntegrityError::ArithmeticOverflow)?,
            ),
        })
    }

    /// Evaluates digest/signature evidence against policy.
    fn evaluate(
        &self,
        expected: Option<&Digest>,
        actual: Option<Digest>,
        signature_verified: Option<bool>,
        byte_length: Option<u64>,
    ) -> Result<IntegrityVerification, IntegrityError> {
        if self.policy.requirement == IntegrityRequirement::Optional {
            return Ok(IntegrityVerification {
                status: IntegrityStatus::NotRequired,
                expected_digest: expected.copied(),
                actual_digest: actual,
                signature_verified: signature_verified
                    .unwrap_or(false),
                byte_length,
            });
        }

        let digest_verified =
            match (expected, actual.as_ref()) {
                (Some(expected), Some(actual)) => {
                    if !verify_digest(expected, actual) {
                        return Err(IntegrityError::DigestMismatch);
                    }

                    true
                }

                (None, _) if self.policy.requirement.requires_digest() => {
                    return Err(IntegrityError::MissingEvidence);
                }

                _ => false,
            };

        let signature_verified =
            signature_verified.unwrap_or(false);

        if self.policy.requirement.requires_signature()
            && !signature_verified
        {
            return Err(IntegrityError::MissingEvidence);
        }

        let status = match (
            digest_verified,
            signature_verified,
            self.policy.requirement,
        ) {
            (
                true,
                true,
                IntegrityRequirement::DigestAndSignatureRequired,
            ) => IntegrityStatus::Verified,

            (
                true,
                false,
                IntegrityRequirement::DigestRequired,
            ) => IntegrityStatus::DigestVerified,

            (
                false,
                true,
                IntegrityRequirement::SignatureRequired,
            ) => IntegrityStatus::SignatureVerified,

            _ => IntegrityStatus::Insufficient,
        };

        Ok(IntegrityVerification {
            status,
            expected_digest: expected.copied(),
            actual_digest: actual,
            signature_verified,
            byte_length,
        })
    }
}

// ============================================================================
// Canonical signing material
// ============================================================================

/// Domain separator used when constructing checkpoint integrity signatures.
///
/// The caller must include this exact domain separator in canonical signing
/// material. It prevents accidentally signing an unrelated object with the
/// same raw bytes.
pub const CHECKPOINT_INTEGRITY_SIGNATURE_DOMAIN: &[u8] =
    b"ZAMANI-CHECKPOINT-INTEGRITY-V1";

/// Builds canonical signing material from an integrity digest.
///
/// The caller may append additional canonical fields outside this function,
/// but the digest domain must always be represented explicitly.
#[must_use]
pub fn canonical_digest_signing_message(
    digest: &Digest,
) -> Vec<u8> {
    let algorithm = digest.algorithm.as_str().as_bytes();
    let digest_bytes = digest.value.as_bytes();

    let mut message = Vec::with_capacity(
        CHECKPOINT_INTEGRITY_SIGNATURE_DOMAIN.len()
            + algorithm.len()
            + digest_bytes.len()
            + 2,
    );

    message.extend_from_slice(
        CHECKPOINT_INTEGRITY_SIGNATURE_DOMAIN,
    );

    message.push(0);

    message.extend_from_slice(algorithm);

    message.push(0);

    message.extend_from_slice(digest_bytes);

    message
}

// ============================================================================
// Integrity commitment builder
// ============================================================================

/// Streaming builder for an ordered integrity commitment.
///
/// This builder is intentionally independent of `manifest.rs`.
///
/// `manifest.rs` can enumerate canonical artifacts and feed their already
/// calculated integrity records into this builder.
///
/// The builder does not impose an artifact-count limit.
#[derive(Debug)]
pub struct IntegrityCommitmentBuilder {
    algorithm: DigestAlgorithm,
    hasher: Box<dyn DigestHasher>,
    artifact_count: u64,
    total_byte_length: u64,
}

impl IntegrityCommitmentBuilder {
    /// Creates an empty commitment builder.
    #[must_use]
    pub fn new(
        algorithm: DigestAlgorithm,
    ) -> Self {
        Self {
            algorithm,
            hasher: hasher_for(algorithm),
            artifact_count: 0,
            total_byte_length: 0,
        }
    }

    /// Returns the selected algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> DigestAlgorithm {
        self.algorithm
    }

    /// Adds one canonical artifact digest to the commitment.
    ///
    /// The caller is responsible for supplying artifacts in the canonical
    /// manifest order.
    pub fn add_artifact(
        &mut self,
        artifact: &ArtifactIntegrity,
    ) -> Result<(), IntegrityError> {
        artifact.validate()?;

        if artifact.digest.algorithm != self.algorithm {
            return Err(
                IntegrityError::InvalidRecord(
                    "artifact digest algorithm does not match commitment algorithm"
                        .to_owned(),
                ),
            );
        }

        let artifact_id =
            artifact.artifact_id.as_str().as_bytes();

        let digest =
            artifact.digest.value.as_bytes();

        // Length-prefix each variable field so concatenation is
        // unambiguous.
        let artifact_id_length =
            u64::try_from(artifact_id.len())
                .map_err(|_| IntegrityError::ArithmeticOverflow)?;

        self.hasher.update(
            &artifact_id_length.to_be_bytes(),
        );

        self.hasher.update(artifact_id);

        self.hasher.update(
            digest,
        );

        let declared_length =
            artifact.byte_length.unwrap_or(0);

        self.hasher.update(
            &declared_length.to_be_bytes(),
        );

        self.artifact_count =
            self.artifact_count
                .checked_add(1)
                .ok_or(IntegrityError::ArithmeticOverflow)?;

        if let Some(length) = artifact.byte_length {
            self.total_byte_length =
                self.total_byte_length
                    .checked_add(length)
                    .ok_or(IntegrityError::ArithmeticOverflow)?;
        }

        Ok(())
    }

    /// Finalizes the commitment.
    #[must_use]
    pub fn finalize(
        self,
    ) -> CheckpointIntegrityRoot {
        let digest = self.hasher.finalize();

        CheckpointIntegrityRoot::new(digest)
            .with_artifact_count(self.artifact_count)
            .with_total_byte_length(
                self.total_byte_length,
            )
    }
}

// ============================================================================
// Integrity utility functions
// ============================================================================

/// Calculates an artifact digest and returns complete artifact integrity
/// evidence.
pub fn calculate_artifact_integrity(
    artifact_id: ArtifactId,
    algorithm: DigestAlgorithm,
    bytes: &[u8],
) -> Result<ArtifactIntegrity, IntegrityError> {
    let digest = digest_bytes(algorithm, bytes);

    let byte_length =
        u64::try_from(bytes.len())
            .map_err(|_| IntegrityError::ArithmeticOverflow)?;

    ArtifactIntegrity::new(
        artifact_id,
        digest,
    )
    .with_byte_length(byte_length)
    .validate()
    .map(|_| {
        ArtifactIntegrity::new(
            artifact_id,
            digest,
        )
        .with_byte_length(byte_length)
    })
}

/// Calculates artifact integrity from a stream.
pub fn calculate_artifact_integrity_reader<R: Read>(
    artifact_id: ArtifactId,
    algorithm: DigestAlgorithm,
    reader: R,
) -> Result<ArtifactIntegrity, IntegrityError> {
    let result = digest_reader(
        algorithm,
        reader,
    )?;

    ArtifactIntegrity::new(
        artifact_id,
        result.digest,
    )
    .with_byte_length(result.byte_count)
    .validate()
    .map(|_| {
        ArtifactIntegrity::new(
            artifact_id,
            result.digest,
        )
        .with_byte_length(result.byte_count)
    })
}

/// Verifies a byte slice against an expected digest.
pub fn verify_bytes_against_digest(
    bytes: &[u8],
    expected: &Digest,
) -> Result<IntegrityVerification, IntegrityError> {
    let actual =
        digest_bytes(expected.algorithm, bytes);

    if !verify_digest(expected, &actual) {
        return Err(IntegrityError::DigestMismatch);
    }

    Ok(IntegrityVerification {
        status: IntegrityStatus::DigestVerified,
        expected_digest: Some(*expected),
        actual_digest: Some(actual),
        signature_verified: false,
        byte_length: Some(
            u64::try_from(bytes.len())
                .map_err(|_| IntegrityError::ArithmeticOverflow)?,
        ),
    })
}

/// Verifies a reader against an expected digest.
pub fn verify_reader_against_digest<R: Read>(
    reader: R,
    expected: &Digest,
) -> Result<IntegrityVerification, IntegrityError> {
    let result =
        digest_reader(expected.algorithm, reader)?;

    if !verify_digest(
        expected,
        &result.digest,
    ) {
        return Err(IntegrityError::DigestMismatch);
    }

    Ok(IntegrityVerification {
        status: IntegrityStatus::DigestVerified,
        expected_digest: Some(*expected),
        actual_digest: Some(result.digest),
        signature_verified: false,
        byte_length: Some(result.byte_count),
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_deterministic() {
        let first =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"zamani",
            );

        let second =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"zamani",
            );

        assert_eq!(first, second);
    }

    #[test]
    fn sha3_is_deterministic() {
        let first =
            digest_bytes(
                DigestAlgorithm::Sha3_256,
                b"zamani",
            );

        let second =
            digest_bytes(
                DigestAlgorithm::Sha3_256,
                b"zamani",
            );

        assert_eq!(first, second);
    }

    #[test]
    fn different_algorithms_have_distinct_identity() {
        let sha256 =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"zamani",
            );

        let sha3 =
            digest_bytes(
                DigestAlgorithm::Sha3_256,
                b"zamani",
            );

        assert_ne!(sha256.algorithm, sha3.algorithm);
    }

    #[test]
    fn digest_verification_accepts_equal_digest() {
        let digest =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"checkpoint",
            );

        assert!(
            verify_digest(
                &digest,
                &digest,
            )
        );
    }

    #[test]
    fn digest_verification_rejects_modified_content() {
        let expected =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"checkpoint",
            );

        let actual =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"modified",
            );

        assert!(
            !verify_digest(
                &expected,
                &actual,
            )
        );
    }

    #[test]
    fn digest_reader_matches_direct_digest() {
        let bytes =
            b"large checkpoint content";

        let direct =
            digest_bytes(
                DigestAlgorithm::Sha256,
                bytes,
            );

        let streamed =
            digest_reader(
                DigestAlgorithm::Sha256,
                &bytes[..],
            )
            .expect("stream digest should succeed");

        assert_eq!(
            direct,
            streamed.digest
        );

        assert_eq!(
            streamed.byte_count,
            u64::try_from(bytes.len())
                .expect("test length fits")
        );
    }

    #[test]
    fn hexadecimal_round_trip_is_lossless() {
        let digest =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"zamani",
            );

        let encoded =
            digest.to_hex();

        let decoded =
            DigestBytes::from_hex(&encoded)
                .expect("valid hexadecimal");

        assert_eq!(
            digest.value,
            decoded
        );
    }

    #[test]
    fn empty_integrity_policy_is_rejected() {
        let result =
            IntegrityPolicy::new(
                IntegrityRequirement::DigestRequired,
                std::iter::empty(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn signature_requirement_requires_algorithm() {
        let policy =
            IntegrityPolicy::new(
                IntegrityRequirement::SignatureRequired,
                [DigestAlgorithm::Sha256],
            )
            .expect("digest algorithm");

        assert!(
            policy.validate().is_err()
        );
    }

    #[test]
    fn artifact_integrity_is_streaming_capable() {
        let artifact_id =
            ArtifactId::new("artifact-1")
                .expect("valid artifact ID");

        let evidence =
            calculate_artifact_integrity_reader(
                artifact_id,
                DigestAlgorithm::Sha256,
                &b"artifact"[..],
            )
            .expect("artifact digest should succeed");

        assert_eq!(
            evidence.byte_length,
            Some(8)
        );
    }

    #[test]
    fn commitment_builder_is_order_sensitive() {
        let first_id =
            ArtifactId::new("artifact-a")
                .expect("valid artifact ID");

        let second_id =
            ArtifactId::new("artifact-b")
                .expect("valid artifact ID");

        let first =
            calculate_artifact_integrity(
                first_id,
                DigestAlgorithm::Sha256,
                b"a",
            )
            .expect("first artifact");

        let second =
            calculate_artifact_integrity(
                second_id,
                DigestAlgorithm::Sha256,
                b"b",
            )
            .expect("second artifact");

        let mut left =
            IntegrityCommitmentBuilder::new(
                DigestAlgorithm::Sha256,
            );

        left.add_artifact(&first)
            .expect("first artifact");

        left.add_artifact(&second)
            .expect("second artifact");

        let mut right =
            IntegrityCommitmentBuilder::new(
                DigestAlgorithm::Sha256,
            );

        right.add_artifact(&second)
            .expect("second artifact");

        right.add_artifact(&first)
            .expect("first artifact");

        assert_ne!(
            left.finalize().digest,
            right.finalize().digest
        );
    }

    #[test]
    fn canonical_signing_material_is_deterministic() {
        let digest =
            digest_bytes(
                DigestAlgorithm::Sha256,
                b"zamani",
            );

        let first =
            canonical_digest_signing_message(
                &digest,
            );

        let second =
            canonical_digest_signing_message(
                &digest,
            );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn integrity_evidence_rejects_modified_bytes() {
        let artifact_id =
            ArtifactId::new("artifact-1")
                .expect("valid artifact ID");

        let evidence =
            calculate_artifact_integrity(
                artifact_id,
                DigestAlgorithm::Sha256,
                b"original",
            )
            .expect("integrity evidence");

        let result =
            verify_bytes_against_digest(
                b"modified",
                &evidence.digest,
            );

        assert!(result.is_err());
    }
}