//! Zamani Quantum Optimization — Verification Certificates
//!
//! Production-grade, deterministic, tamper-evident verification certificates
//! for quantum-circuit optimization.
//!
//! # Architectural position
//!
//! ```text
//!                         canonical Quantum IR
//!                                  │
//!                                  ▼
//!                         optimization pipeline
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    │                           │
//!              original circuit           optimized circuit
//!                    │                           │
//!                    └─────────────┬─────────────┘
//!                                  ▼
//!                         verification engines
//!                    ┌─────────────┼─────────────┐
//!                    ▼             ▼             ▼
//!                 semantic      randomized    exhaustive
//!                    │             │             │
//!                    └─────────────┼─────────────┘
//!                                  ▼
//!                       verification certificate
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    ▼                           ▼
//!              local validation            external audit
//!                    │                           │
//!                    └─────────────┬─────────────┘
//!                                  ▼
//!                         trusted compiler result
//! ```
//!
//! # Purpose
//!
//! This module defines the certificate boundary for the Zamani quantum
//! optimization verifier.
//!
//! A certificate records:
//!
//! - what was verified;
//! - which circuit identities were verified;
//! - which verification engine produced the evidence;
//! - what exact verdict was produced;
//! - what resource policy was active;
//! - what numerical/error information was observed;
//! - whether the result is a proof or statistical evidence;
//! - a deterministic certificate digest;
//! - optional parent/chain information for composable audit trails;
//! - optional optimizer/pass metadata;
//! - schema and verifier versions.
//!
//! The certificate is intentionally an observational artifact.
//!
//! It MUST NOT:
//!
//! - execute a QPU;
//! - perform backend I/O;
//! - mutate a QuantumCircuit;
//! - contain a second Quantum IR;
//! - become part of the optimizer transformation algorithm;
//! - claim proof when only randomized evidence exists;
//! - treat an inconclusive verification as success;
//! - depend on a particular optimizer pass implementation.
//!
//! # Critical soundness contract
//!
//! ```text
//! Proven
//!     = a verification engine explicitly established the requested relation.
//!
//! Disproven
//!     = a verification engine explicitly established non-equivalence.
//!
//! EvidenceOnly
//!     = empirical/statistical evidence exists, but mathematical equivalence
//!       was not proven.
//!
//! Inconclusive
//!     = the verification process could not establish a definitive result.
//!
//! Invalid
//!     = the certificate itself failed integrity/schema validation.
//! ```
//!
//! In particular:
//!
//! ```text
//! randomized "no counterexample"
//!             !=
//! exact semantic equivalence
//! ```
//!
//! This distinction is mandatory for a production compiler.
//!
//! # Canonical IR
//!
//! Certificates identify Quantum IR circuits through cryptographic content
//! identities/fingerprints. They do not define another circuit representation.
//!
//! Logical qubits remain owned by:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! This module does not duplicate or reinterpret logical/physical qubit IDs.
//!
//! # Cryptographic integrity
//!
//! Certificates use SHA-256 through the repository's existing `sha2`
//! dependency.
//!
//! The certificate digest is calculated over a deterministic serialized form
//! with the digest field itself omitted from the digest input.
//!
//! This provides tamper evidence and stable content identity.
//!
//! It does NOT constitute a digital signature.
//!
//! A future signing layer may sign the certificate digest without changing the
//! certificate's semantic model.
//!
//! # Digital signatures
//!
//! Private-key signing does not belong in this file.
//!
//! This module deliberately does not:
//!
//! - generate private keys;
//! - access key stores;
//! - contact identity services;
//! - perform network operations;
//! - silently sign certificates.
//!
//! A future trust/signature subsystem can sign the canonical certificate
//! digest using Ed25519 or another explicitly selected scheme.
//!
//! # Scaling
//!
//! The certificate model does not impose a fixed quantum-circuit size limit.
//!
//! Circuit contents are represented by fixed-size SHA-256 digests rather than
//! embedding the circuit itself.
//!
//! Counters use `u128` where practical so certificate metadata can represent
//! workloads substantially larger than ordinary machine-scale jobs.
//!
//! Metadata and evidence strings remain explicitly bounded by the builder's
//! limits.
//!
//! "Tiny to infinity" therefore means:
//!
//! > this certificate layer does not impose an artificial circuit-size limit;
//! > actual verification remains constrained only by the verification engine,
//! > configured resource policy, and available resources.
//!
//! No finite machine can execute literally infinite work.
//!
//! # Determinism
//!
//! Certificate serialization is deterministic because the public certificate
//! representation uses ordered struct fields and scalar/string values rather
//! than unordered maps.
//!
//! Certificate IDs are content-derived, not timestamp/randomness-derived.
//!
//! Timestamps are therefore optional metadata and are never part of the
//! certificate identity unless the caller explicitly includes them in the
//! certificate metadata.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//! - no unsafe code
//!
//! # Dependencies
//!
//! Existing repository dependencies:
//!
//! - serde
//! - serde_json
//! - sha2
//!
//! No additional dependency is required.
//!
//! # Integration contract
//!
//! `verification/mod.rs` should expose:
//!
//! ```text
//! pub mod certificates;
//! ```
//!
//! `verification/semantic.rs` may create certificates from
//! `SemanticVerificationReport` using:
//!
//! ```text
//! VerificationCertificate::from_semantic_report(...)
//! ```
//!
//! `verification/randomized.rs` may create evidence certificates using:
//!
//! ```text
//! VerificationCertificate::from_randomized_report(...)
//! ```
//!
//! Randomized evidence MUST remain `EvidenceOnly` unless another verifier
//! explicitly supplies an exact proof.
//!
//! `verification/exhaustive.rs` may use:
//!
//! ```text
//! VerificationCertificate::from_exhaustive_evidence(...)
//! ```
//!
//! `provenance.rs` can store the resulting certificate digest and certificate
//! identity without depending on certificate internals.
//!
//! `serialization/report.rs` can serialize a certificate through
//! [`VerificationCertificate::to_json`].
//!
//! `result.rs` can store the certificate or its digest as verification
//! evidence.
//!
//! `pipeline.rs` should create certificates only after verification completes.
//!
//! No optimizer pass should need to modify this file when a new pass is added.
//!
//! # Security properties
//!
//! This module:
//!
//! - contains `#![forbid(unsafe_code)]`;
//! - never executes quantum hardware;
//! - never performs network I/O;
//! - never mutates circuits;
//! - never treats statistical evidence as proof;
//! - never treats inconclusive verification as success;
//! - rejects malformed certificate data;
//! - validates certificate integrity before accepting imported certificates;
//! - uses checked arithmetic for externally supplied counters;
//! - bounds metadata sizes;
//! - does not store complete circuits;
//! - does not store secrets or private keys;
//! - does not manufacture signatures;
//! - does not use ambient randomness.
//!
//! # Important limitation
//!
//! A certificate is only as trustworthy as the verification evidence from
//! which it was constructed.
//!
//! A certificate containing a SHA-256 digest proves that the certificate data
//! has not changed since hashing. It does not independently prove that the
//! optimizer, verifier, Quantum IR, compiler binary, or machine was honest.
//!
//! For a stronger trusted-computing-base model, certificate validation should
//! eventually be combined with:
//!
//! - signed compiler manifests;
//! - reproducible builds;
//! - verified verifier implementations;
//! - independent verification engines;
//! - signed provenance;
//! - remote/audited attestation where appropriate.
//!
//! Those concerns belong outside this file.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! The intended lifecycle is:
//!
//! ```text
//! verification report
//!        │
//!        ▼
//! VerificationCertificate::builder()
//!        │
//!        ▼
//! add evidence
//!        │
//!        ▼
//! build()
//!        │
//!        ▼
//! finalize()
//!        │
//!        ▼
//! verify_integrity()
//!        │
//!        ▼
//! canonical JSON / audit storage
//! ```
//!
//! The certificate can be independently reconstructed and checked without
//! executing the optimized circuit.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

// =============================================================================
// Stable public identifiers
// =============================================================================

/// Stable identifier for this verification subsystem.
pub const CERTIFICATE_ID: &str =
    "quantum.optimization.verification.certificates";

/// Public certificate schema version.
///
/// This is independent from the Quantum IR version and optimizer version.
pub const CERTIFICATE_SCHEMA_VERSION: u32 = 1;

/// Hash algorithm used for certificate and circuit identities.
pub const CERTIFICATE_HASH_ALGORITHM: &str = "sha256";

/// Maximum default certificate metadata field size in bytes.
pub const DEFAULT_MAX_FIELD_BYTES: u64 = 16 * 1024;

/// Maximum default number of evidence records.
pub const DEFAULT_MAX_EVIDENCE: usize = 1024;

/// Maximum default number of parent certificate digests.
pub const DEFAULT_MAX_PARENTS: usize = 1024;

// =============================================================================
// Result aliases
// =============================================================================

/// Result returned by certificate operations.
pub type CertificateResult<T> = Result<T, CertificateError>;

// =============================================================================
// Certificate errors
// =============================================================================

/// Errors produced by certificate creation, validation, serialization, or
/// integrity checking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificateError {
    /// A required textual field was empty.
    EmptyField {
        /// Name of the offending field.
        field: String,
    },

    /// A textual field exceeded the configured limit.
    FieldTooLarge {
        /// Name of the offending field.
        field: String,

        /// Supplied byte length.
        actual_bytes: u64,

        /// Maximum allowed byte length.
        maximum_bytes: u64,
    },

    /// Too many evidence records were supplied.
    EvidenceLimitExceeded {
        /// Supplied count.
        actual: usize,

        /// Maximum permitted count.
        maximum: usize,
    },

    /// Too many parent certificate references were supplied.
    ParentLimitExceeded {
        /// Supplied count.
        actual: usize,

        /// Maximum permitted count.
        maximum: usize,
    },

    /// The certificate contains an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Supplied schema version.
        version: u32,
    },

    /// The certificate uses an unsupported hash algorithm.
    UnsupportedHashAlgorithm {
        /// Supplied algorithm.
        algorithm: String,
    },

    /// A digest has an invalid hexadecimal representation.
    InvalidDigestEncoding,

    /// A digest does not have the required SHA-256 length.
    InvalidDigestLength {
        /// Number of decoded bytes.
        actual: usize,

        /// Required number of bytes.
        expected: usize,
    },

    /// The certificate's stored digest does not match its canonical content.
    IntegrityMismatch {
        /// Stored digest.
        expected: String,

        /// Recomputed digest.
        actual: String,
    },

    /// The certificate's semantic status is inconsistent with its evidence.
    InvalidStatus {
        /// Human-readable explanation.
        reason: String,
    },

    /// An evidence record is inconsistent with the certificate.
    InvalidEvidence {
        /// Human-readable explanation.
        reason: String,
    },

    /// A numeric value is invalid.
    InvalidNumber {
        /// Name of the invalid field.
        field: String,
    },

    /// Canonical JSON serialization failed.
    Serialization(String),

    /// JSON deserialization failed.
    Deserialization(String),
}

impl fmt::Display for CertificateError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "certificate field `{field}` must not be empty"
                )
            }

            Self::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "certificate field `{field}` is too large: \
                     {actual_bytes} bytes, maximum {maximum_bytes}"
                )
            }

            Self::EvidenceLimitExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "certificate evidence limit exceeded: \
                     {actual} records, maximum {maximum}"
                )
            }

            Self::ParentLimitExceeded {
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "certificate parent limit exceeded: \
                     {actual} records, maximum {maximum}"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    formatter,
                    "unsupported verification certificate schema version {version}"
                )
            }

            Self::UnsupportedHashAlgorithm { algorithm } => {
                write!(
                    formatter,
                    "unsupported certificate hash algorithm `{algorithm}`"
                )
            }

            Self::InvalidDigestEncoding => {
                formatter.write_str(
                    "certificate digest contains invalid hexadecimal encoding",
                )
            }

            Self::InvalidDigestLength {
                actual,
                expected,
            } => {
                write!(
                    formatter,
                    "certificate digest has {actual} bytes, expected {expected}"
                )
            }

            Self::IntegrityMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "certificate integrity mismatch: stored {expected}, \
                     recomputed {actual}"
                )
            }

            Self::InvalidStatus { reason } => {
                write!(
                    formatter,
                    "invalid certificate status: {reason}"
                )
            }

            Self::InvalidEvidence { reason } => {
                write!(
                    formatter,
                    "invalid certificate evidence: {reason}"
                )
            }

            Self::InvalidNumber { field } => {
                write!(
                    formatter,
                    "invalid numeric certificate field `{field}`"
                )
            }

            Self::Serialization(error) => {
                write!(
                    formatter,
                    "certificate serialization failed: {error}"
                )
            }

            Self::Deserialization(error) => {
                write!(
                    formatter,
                    "certificate deserialization failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for CertificateError {}

// =============================================================================
// Certificate status
// =============================================================================

/// High-level trust/status classification of a certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CertificateStatus {
    /// Exact semantic equivalence was proven.
    ///
    /// This status is reserved for explicit proof evidence.
    Proven,

    /// Non-equivalence was explicitly established.
    ///
    /// This status is useful for audit artifacts and failed compiler
    /// transformations. It MUST NOT be interpreted as a successful optimization.
    Disproven,

    /// Statistical or empirical evidence exists but mathematical equivalence
    /// was not proven.
    EvidenceOnly,

    /// Verification could not establish a definitive result.
    Inconclusive,

    /// The certificate itself is invalid.
    Invalid,
}

impl CertificateStatus {
    /// Returns true only for proof-level equivalence.
    #[must_use]
    pub const fn is_proven(self) -> bool {
        matches!(self, Self::Proven)
    }

    /// Returns true only for explicitly disproven equivalence.
    #[must_use]
    pub const fn is_disproven(self) -> bool {
        matches!(self, Self::Disproven)
    }

    /// Returns true for statistical/evidence-only certificates.
    #[must_use]
    pub const fn is_evidence_only(self) -> bool {
        matches!(self, Self::EvidenceOnly)
    }

    /// Returns true for inconclusive verification.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }

    /// Returns true when the certificate is not a successful equivalence proof.
    #[must_use]
    pub const fn is_not_proof(self) -> bool {
        !self.is_proven()
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proven => "proven",
            Self::Disproven => "disproven",
            Self::EvidenceOnly => "evidence_only",
            Self::Inconclusive => "inconclusive",
            Self::Invalid => "invalid",
        }
    }
}

impl fmt::Display for CertificateStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evidence kind
// =============================================================================

/// Type of verification evidence contained in a certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceKind {
    /// Structural equality of canonical IR.
    Structural,

    /// Exact semantic verification.
    ExactSemantic,

    /// Exact semantic verification up to global phase.
    ExactSemanticUpToGlobalPhase,

    /// Exhaustive finite-domain verification.
    Exhaustive,

    /// Randomized/differential verification.
    Randomized,

    /// Independent/custom verifier.
    External,

    /// Composite evidence produced by multiple independent verifiers.
    Composite,
}

impl EvidenceKind {
    /// Returns whether this evidence kind can independently represent an exact
    /// equivalence proof.
    ///
    /// Randomized evidence deliberately returns false.
    #[must_use]
    pub const fn can_prove_exact_equivalence(self) -> bool {
        matches!(
            self,
            Self::Structural
                | Self::ExactSemantic
                | Self::ExactSemanticUpToGlobalPhase
                | Self::Exhaustive
        )
    }

    /// Returns whether this evidence is statistical/empirical.
    #[must_use]
    pub const fn is_statistical(self) -> bool {
        matches!(self, Self::Randomized)
    }

    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "structural",
            Self::ExactSemantic => "exact_semantic",
            Self::ExactSemanticUpToGlobalPhase => {
                "exact_semantic_up_to_global_phase"
            }
            Self::Exhaustive => "exhaustive",
            Self::Randomized => "randomized",
            Self::External => "external",
            Self::Composite => "composite",
        }
    }
}

impl fmt::Display for EvidenceKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Hash identity
// =============================================================================

/// Fixed cryptographic content identity.
///
/// The textual representation is lowercase hexadecimal SHA-256.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash {
    /// Hash algorithm identifier.
    pub algorithm: String,

    /// Lowercase hexadecimal digest.
    pub value: String,
}

impl ContentHash {
    /// Creates a SHA-256 identity from raw bytes.
    #[must_use]
    pub fn sha256(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);

        Self {
            algorithm: CERTIFICATE_HASH_ALGORITHM.to_owned(),
            value: encode_hex(&digest),
        }
    }

    /// Creates a SHA-256 identity from UTF-8 text.
    #[must_use]
    pub fn sha256_text(text: &str) -> Self {
        Self::sha256(text.as_bytes())
    }

    /// Parses and validates an externally supplied SHA-256 identity.
    pub fn parse_sha256(value: impl Into<String>) -> CertificateResult<Self> {
        let value = value.into();

        if value.len() != 64 {
            return Err(CertificateError::InvalidDigestLength {
                actual: value.len() / 2,
                expected: 32,
            });
        }

        let decoded = decode_hex(&value)?;

        if decoded.len() != 32 {
            return Err(CertificateError::InvalidDigestLength {
                actual: decoded.len(),
                expected: 32,
            });
        }

        Ok(Self {
            algorithm: CERTIFICATE_HASH_ALGORITHM.to_owned(),
            value: value.to_ascii_lowercase(),
        })
    }

    /// Validates this content hash.
    pub fn validate(&self) -> CertificateResult<()> {
        if self.algorithm != CERTIFICATE_HASH_ALGORITHM {
            return Err(CertificateError::UnsupportedHashAlgorithm {
                algorithm: self.algorithm.clone(),
            });
        }

        if self.value.len() != 64 {
            return Err(CertificateError::InvalidDigestLength {
                actual: self.value.len() / 2,
                expected: 32,
            });
        }

        let decoded = decode_hex(&self.value)?;

        if decoded.len() != 32 {
            return Err(CertificateError::InvalidDigestLength {
                actual: decoded.len(),
                expected: 32,
            });
        }

        Ok(())
    }

    /// Returns the canonical hexadecimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

// =============================================================================
// Circuit identities
// =============================================================================

/// Identity information for the original and optimized circuits.
///
/// The circuit itself is intentionally not embedded in the certificate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitIdentities {
    /// Original/input circuit fingerprint.
    pub original: ContentHash,

    /// Optimized/output circuit fingerprint.
    pub optimized: ContentHash,

    /// Logical qubit count when known.
    pub logical_qubits: Option<u128>,

    /// Original operation count when known.
    pub original_operations: Option<u128>,

    /// Optimized operation count when known.
    pub optimized_operations: Option<u128>,
}

impl CircuitIdentities {
    /// Creates identities from externally supplied hashes.
    pub fn new(
        original: ContentHash,
        optimized: ContentHash,
    ) -> CertificateResult<Self> {
        original.validate()?;
        optimized.validate()?;

        Ok(Self {
            original,
            optimized,
            logical_qubits: None,
            original_operations: None,
            optimized_operations: None,
        })
    }

    /// Sets the logical qubit count.
    #[must_use]
    pub const fn with_logical_qubits(
        mut self,
        value: u128,
    ) -> Self {
        self.logical_qubits = Some(value);
        self
    }

    /// Sets the operation counts.
    #[must_use]
    pub const fn with_operation_counts(
        mut self,
        original: u128,
        optimized: u128,
    ) -> Self {
        self.original_operations = Some(original);
        self.optimized_operations = Some(optimized);
        self
    }

    /// Validates circuit identities.
    pub fn validate(&self) -> CertificateResult<()> {
        self.original.validate()?;
        self.optimized.validate()?;

        Ok(())
    }
}

// =============================================================================
// Verification evidence
// =============================================================================

/// Generic verification evidence.
///
/// This type deliberately stores normalized summaries instead of borrowing
/// verifier implementation types. That keeps certificates stable even when
/// individual verification implementations evolve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationEvidence {
    /// Evidence kind.
    pub kind: EvidenceKind,

    /// Stable verifier identifier.
    pub verifier_id: String,

    /// Verifier API/contract version.
    pub verifier_version: u32,

    /// Machine-readable verdict returned by the verifier.
    pub verdict: String,

    /// Whether this evidence is proof-level.
    pub proof: bool,

    /// Whether global phase was ignored.
    pub ignores_global_phase: bool,

    /// Maximum observed numerical error, when applicable.
    pub max_error: Option<f64>,

    /// Number of logical qubits checked, when applicable.
    pub logical_qubits: Option<u128>,

    /// Number of operations in the original circuit, when applicable.
    pub original_operations: Option<u128>,

    /// Number of operations in the optimized circuit, when applicable.
    pub optimized_operations: Option<u128>,

    /// Number of randomized/exhaustive trials, when applicable.
    pub trials: Option<u128>,

    /// Number of successful comparisons, when applicable.
    pub matches: Option<u128>,

    /// Number of mismatches, when applicable.
    pub mismatches: Option<u128>,

    /// Number of inconclusive checks, when applicable.
    pub inconclusive: Option<u128>,

    /// Human-readable verifier reason.
    pub reason: Option<String>,

    /// Optional stable evidence digest supplied by an external verifier.
    pub evidence_digest: Option<ContentHash>,
}

impl VerificationEvidence {
    /// Creates generic evidence.
    pub fn new(
        kind: EvidenceKind,
        verifier_id: impl Into<String>,
        verifier_version: u32,
        verdict: impl Into<String>,
        proof: bool,
    ) -> CertificateResult<Self> {
        let verifier_id = verifier_id.into();
        let verdict = verdict.into();

        validate_text(
            "verifier_id",
            &verifier_id,
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_text(
            "verdict",
            &verdict,
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        if proof && !kind.can_prove_exact_equivalence() {
            return Err(CertificateError::InvalidEvidence {
                reason: format!(
                    "evidence kind `{kind}` cannot independently represent \
                     an exact equivalence proof"
                ),
            });
        }

        Ok(Self {
            kind,
            verifier_id,
            verifier_version,
            verdict,
            proof,
            ignores_global_phase: false,
            max_error: None,
            logical_qubits: None,
            original_operations: None,
            optimized_operations: None,
            trials: None,
            matches: None,
            mismatches: None,
            inconclusive: None,
            reason: None,
            evidence_digest: None,
        })
    }

    /// Marks the evidence as global-phase-insensitive.
    #[must_use]
    pub const fn with_global_phase_ignored(
        mut self,
        ignored: bool,
    ) -> Self {
        self.ignores_global_phase = ignored;
        self
    }

    /// Adds a maximum numerical error.
    pub fn with_max_error(
        mut self,
        error: f64,
    ) -> CertificateResult<Self> {
        if !error.is_finite() || error < 0.0 {
            return Err(CertificateError::InvalidNumber {
                field: "max_error".to_owned(),
            });
        }

        self.max_error = Some(error);
        Ok(self)
    }

    /// Adds logical qubit count.
    #[must_use]
    pub const fn with_logical_qubits(
        mut self,
        value: u128,
    ) -> Self {
        self.logical_qubits = Some(value);
        self
    }

    /// Adds operation counts.
    #[must_use]
    pub const fn with_operation_counts(
        mut self,
        original: u128,
        optimized: u128,
    ) -> Self {
        self.original_operations = Some(original);
        self.optimized_operations = Some(optimized);
        self
    }

    /// Adds randomized/exhaustive trial statistics.
    #[must_use]
    pub const fn with_trial_statistics(
        mut self,
        trials: u128,
        matches: u128,
        mismatches: u128,
        inconclusive: u128,
    ) -> Self {
        self.trials = Some(trials);
        self.matches = Some(matches);
        self.mismatches = Some(mismatches);
        self.inconclusive = Some(inconclusive);
        self
    }

    /// Adds an explanatory reason.
    pub fn with_reason(
        mut self,
        reason: impl Into<String>,
    ) -> CertificateResult<Self> {
        let reason = reason.into();

        validate_text(
            "reason",
            &reason,
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        self.reason = Some(reason);
        Ok(self)
    }

    /// Adds an externally supplied evidence digest.
    pub fn with_evidence_digest(
        mut self,
        digest: ContentHash,
    ) -> CertificateResult<Self> {
        digest.validate()?;
        self.evidence_digest = Some(digest);
        Ok(self)
    }

    /// Validates this evidence record.
    pub fn validate(&self) -> CertificateResult<()> {
        validate_text(
            "verifier_id",
            &self.verifier_id,
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_text(
            "verdict",
            &self.verdict,
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        if let Some(error) = self.max_error {
            if !error.is_finite() || error < 0.0 {
                return Err(CertificateError::InvalidNumber {
                    field: "max_error".to_owned(),
                });
            }
        }

        if self.proof && !self.kind.can_prove_exact_equivalence() {
            return Err(CertificateError::InvalidEvidence {
                reason: format!(
                    "statistical evidence `{}` cannot be marked as proof",
                    self.kind
                ),
            });
        }

        if let Some(digest) = &self.evidence_digest {
            digest.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Certificate metadata
// =============================================================================

/// Optional compiler/optimizer metadata.
///
/// This is intentionally generic so new optimizer passes do not require a
/// certificate schema change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateMetadata {
    /// Compiler/optimizer name.
    pub producer: Option<String>,

    /// Compiler/optimizer version.
    pub producer_version: Option<String>,

    /// Optimization profile.
    pub profile: Option<String>,

    /// Target profile.
    pub target: Option<String>,

    /// Optimization objective.
    pub objective: Option<String>,

    /// Deterministic compilation seed, if one was used.
    pub seed: Option<u64>,

    /// Optional source/compilation identity.
    pub compilation_id: Option<String>,

    /// Optional human-readable note.
    pub note: Option<String>,
}

impl Default for CertificateMetadata {
    fn default() -> Self {
        Self {
            producer: None,
            producer_version: None,
            profile: None,
            target: None,
            objective: None,
            seed: None,
            compilation_id: None,
            note: None,
        }
    }
}

impl CertificateMetadata {
    /// Validates textual metadata using the default certificate field limit.
    pub fn validate(&self) -> CertificateResult<()> {
        validate_optional_text(
            "producer",
            self.producer.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "producer_version",
            self.producer_version.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "profile",
            self.profile.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "target",
            self.target.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "objective",
            self.objective.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "compilation_id",
            self.compilation_id.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        validate_optional_text(
            "note",
            self.note.as_deref(),
            DEFAULT_MAX_FIELD_BYTES,
        )?;

        Ok(())
    }
}

// =============================================================================
// Certificate limits
// =============================================================================

/// Resource limits protecting certificate construction and validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificateLimits {
    /// Maximum evidence records.
    pub max_evidence: usize,

    /// Maximum parent certificate references.
    pub max_parents: usize,

    /// Maximum textual field size in bytes.
    pub max_field_bytes: u64,

    /// Maximum serialized certificate size accepted by import.
    pub max_serialized_bytes: u64,
}

impl Default for CertificateLimits {
    fn default() -> Self {
        Self {
            max_evidence: DEFAULT_MAX_EVIDENCE,
            max_parents: DEFAULT_MAX_PARENTS,
            max_field_bytes: DEFAULT_MAX_FIELD_BYTES,
            max_serialized_bytes: 16 * 1024 * 1024,
        }
    }
}

impl CertificateLimits {
    /// Creates explicit certificate limits.
    pub const fn new(
        max_evidence: usize,
        max_parents: usize,
        max_field_bytes: u64,
        max_serialized_bytes: u64,
    ) -> Self {
        Self {
            max_evidence,
            max_parents,
            max_field_bytes,
            max_serialized_bytes,
        }
    }

    /// Validates the limits themselves.
    pub fn validate(&self) -> CertificateResult<()> {
        if self.max_evidence == 0 {
            return Err(CertificateError::InvalidNumber {
                field: "max_evidence".to_owned(),
            });
        }

        if self.max_parents == 0 {
            return Err(CertificateError::InvalidNumber {
                field: "max_parents".to_owned(),
            });
        }

        if self.max_field_bytes == 0 {
            return Err(CertificateError::InvalidNumber {
                field: "max_field_bytes".to_owned(),
            });
        }

        if self.max_serialized_bytes == 0 {
            return Err(CertificateError::InvalidNumber {
                field: "max_serialized_bytes".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Certificate
// =============================================================================

/// Immutable, serializable verification certificate.
///
/// The `certificate_digest` is calculated over the certificate with that field
/// omitted. It therefore protects all other certificate contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationCertificate {
    /// Certificate schema version.
    pub schema_version: u32,

    /// Stable certificate subsystem identifier.
    pub certificate_type: String,

    /// Certificate status.
    pub status: CertificateStatus,

    /// Original and optimized circuit identities.
    pub circuits: CircuitIdentities,

    /// Verification evidence records.
    pub evidence: Vec<VerificationEvidence>,

    /// Optional compiler/optimizer metadata.
    pub metadata: CertificateMetadata,

    /// Optional parent certificate identities.
    ///
    /// Parent references allow multiple optimization/verification stages to
    /// form an append-only certificate chain.
    pub parents: Vec<ContentHash>,

    /// Optional deterministic certificate creation sequence.
    ///
    /// This is metadata only and does not establish wall-clock chronology.
    pub sequence: Option<u128>,

    /// SHA-256 digest of the canonical certificate content excluding this
    /// field.
    pub certificate_digest: ContentHash,
}

impl VerificationCertificate {
    /// Creates a new certificate builder.
    #[must_use]
    pub fn builder(
        circuits: CircuitIdentities,
    ) -> VerificationCertificateBuilder {
        VerificationCertificateBuilder::new(circuits)
    }

    /// Returns the stable certificate digest.
    #[must_use]
    pub fn digest(&self) -> &ContentHash {
        &self.certificate_digest
    }

    /// Returns true only when this certificate represents a proof.
    #[must_use]
    pub const fn is_proven(&self) -> bool {
        self.status.is_proven()
    }

    /// Returns true if the certificate is only statistical/empirical evidence.
    #[must_use]
    pub const fn is_evidence_only(&self) -> bool {
        self.status.is_evidence_only()
    }

    /// Returns true if verification was inconclusive.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.status.is_inconclusive()
    }

    /// Returns true if the certificate is not an exact proof.
    #[must_use]
    pub const fn is_not_proof(&self) -> bool {
        !self.is_proven()
    }

    /// Validates the complete certificate and its internal integrity digest.
    pub fn validate(&self) -> CertificateResult<()> {
        validate_certificate(self)?;

        let recomputed = self.recompute_digest()?;

        if recomputed != self.certificate_digest {
            return Err(CertificateError::IntegrityMismatch {
                expected: self.certificate_digest.value.clone(),
                actual: recomputed.value,
            });
        }

        Ok(())
    }

    /// Recomputes the certificate digest.
    pub fn recompute_digest(&self) -> CertificateResult<ContentHash> {
        let unsigned = UnsignedCertificate::from_certificate(self);

        let bytes = serde_json::to_vec(&unsigned)
            .map_err(|error| {
                CertificateError::Serialization(error.to_string())
            })?;

        Ok(ContentHash::sha256(&bytes))
    }

    /// Serializes this certificate into deterministic compact JSON.
    pub fn to_json(&self) -> CertificateResult<String> {
        self.validate()?;

        serde_json::to_string(self)
            .map_err(|error| {
                CertificateError::Serialization(error.to_string())
            })
    }

    /// Serializes this certificate into deterministic pretty JSON.
    pub fn to_json_pretty(&self) -> CertificateResult<String> {
        self.validate()?;

        serde_json::to_string_pretty(self)
            .map_err(|error| {
                CertificateError::Serialization(error.to_string())
            })
    }

    /// Imports and validates a certificate from JSON.
    pub fn from_json(
        json: &str,
        limits: CertificateLimits,
    ) -> CertificateResult<Self> {
        limits.validate()?;

        let byte_length = json.as_bytes().len() as u64;

        if byte_length > limits.max_serialized_bytes {
            return Err(CertificateError::FieldTooLarge {
                field: "serialized_certificate".to_owned(),
                actual_bytes: byte_length,
                maximum_bytes: limits.max_serialized_bytes,
            });
        }

        let certificate: Self = serde_json::from_str(json)
            .map_err(|error| {
                CertificateError::Deserialization(error.to_string())
            })?;

        certificate.validate_with_limits(limits)?;

        Ok(certificate)
    }

    /// Validates a certificate using explicit resource limits.
    pub fn validate_with_limits(
        &self,
        limits: CertificateLimits,
    ) -> CertificateResult<()> {
        limits.validate()?;

        if self.schema_version != CERTIFICATE_SCHEMA_VERSION {
            return Err(CertificateError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        if self.evidence.len() > limits.max_evidence {
            return Err(CertificateError::EvidenceLimitExceeded {
                actual: self.evidence.len(),
                maximum: limits.max_evidence,
            });
        }

        if self.parents.len() > limits.max_parents {
            return Err(CertificateError::ParentLimitExceeded {
                actual: self.parents.len(),
                maximum: limits.max_parents,
            });
        }

        validate_text(
            "certificate_type",
            &self.certificate_type,
            limits.max_field_bytes,
        )?;

        self.circuits.validate()?;
        self.metadata.validate()?;

        for evidence in &self.evidence {
            evidence.validate()?;

            validate_evidence_text(
                evidence,
                limits.max_field_bytes,
            )?;
        }

        for parent in &self.parents {
            parent.validate()?;
        }

        validate_status(self)?;

        let recomputed = self.recompute_digest()?;

        if recomputed != self.certificate_digest {
            return Err(CertificateError::IntegrityMismatch {
                expected: self.certificate_digest.value.clone(),
                actual: recomputed.value,
            });
        }

        Ok(())
    }

    /// Adds an exact semantic proof to a new certificate.
    ///
    /// This constructor consumes only normalized evidence so the certificate
    /// layer does not become coupled to a specific semantic-verifier struct.
    pub fn from_exact_evidence(
        circuits: CircuitIdentities,
        evidence: VerificationEvidence,
    ) -> CertificateResult<Self> {
        if !evidence.kind.can_prove_exact_equivalence() {
            return Err(CertificateError::InvalidEvidence {
                reason: format!(
                    "evidence kind `{}` cannot represent an exact proof",
                    evidence.kind
                ),
            });
        }

        if !evidence.proof {
            return Err(CertificateError::InvalidEvidence {
                reason: "exact certificate evidence must explicitly mark \
                         proof=true"
                    .to_owned(),
            });
        }

        let mut builder = Self::builder(circuits);

        builder.add_evidence(evidence)?;
        builder.set_status(CertificateStatus::Proven);

        builder.build()
    }

    /// Creates an evidence-only certificate.
    ///
    /// This is the correct representation for randomized verification that
    /// completed without finding a counterexample.
    pub fn from_evidence(
        circuits: CircuitIdentities,
        evidence: VerificationEvidence,
    ) -> CertificateResult<Self> {
        let mut builder = Self::builder(circuits);

        let status = if evidence.proof {
            CertificateStatus::Proven
        } else if evidence.verdict.eq_ignore_ascii_case(
            "inconclusive",
        ) {
            CertificateStatus::Inconclusive
        } else {
            CertificateStatus::EvidenceOnly
        };

        builder.add_evidence(evidence)?;
        builder.set_status(status);

        builder.build()
    }
}

// =============================================================================
// Unsigned canonical representation
// =============================================================================

/// Internal certificate representation used exclusively for digest calculation.
///
/// Keeping this representation separate from the public certificate prevents
/// accidental self-referential hashing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UnsignedCertificate {
    schema_version: u32,
    certificate_type: String,
    status: CertificateStatus,
    circuits: CircuitIdentities,
    evidence: Vec<VerificationEvidence>,
    metadata: CertificateMetadata,
    parents: Vec<ContentHash>,
    sequence: Option<u128>,
}

impl UnsignedCertificate {
    fn from_certificate(
        certificate: &VerificationCertificate,
    ) -> Self {
        Self {
            schema_version: certificate.schema_version,
            certificate_type: certificate.certificate_type.clone(),
            status: certificate.status,
            circuits: certificate.circuits.clone(),
            evidence: certificate.evidence.clone(),
            metadata: certificate.metadata.clone(),
            parents: certificate.parents.clone(),
            sequence: certificate.sequence,
        }
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`VerificationCertificate`].
///
/// The builder allows callers to construct a certificate incrementally while
/// enforcing resource limits before the final certificate is allocated.
#[derive(Debug, Clone)]
pub struct VerificationCertificateBuilder {
    circuits: CircuitIdentities,
    status: Option<CertificateStatus>,
    evidence: Vec<VerificationEvidence>,
    metadata: CertificateMetadata,
    parents: Vec<ContentHash>,
    sequence: Option<u128>,
    limits: CertificateLimits,
}

impl VerificationCertificateBuilder {
    /// Creates a builder using default certificate limits.
    #[must_use]
    pub fn new(
        circuits: CircuitIdentities,
    ) -> Self {
        Self {
            circuits,
            status: None,
            evidence: Vec::new(),
            metadata: CertificateMetadata::default(),
            parents: Vec::new(),
            sequence: None,
            limits: CertificateLimits::default(),
        }
    }

    /// Applies explicit certificate limits.
    pub fn with_limits(
        mut self,
        limits: CertificateLimits,
    ) -> CertificateResult<Self> {
        limits.validate()?;
        self.limits = limits;
        Ok(self)
    }

    /// Sets the certificate status.
    #[must_use]
    pub const fn set_status(
        &mut self,
        status: CertificateStatus,
    ) {
        self.status = Some(status);
    }

    /// Sets producer metadata.
    pub fn producer(
        &mut self,
        producer: impl Into<String>,
        version: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let producer = producer.into();
        let version = version.into();

        validate_text(
            "producer",
            &producer,
            self.limits.max_field_bytes,
        )?;

        validate_text(
            "producer_version",
            &version,
            self.limits.max_field_bytes,
        )?;

        self.metadata.producer = Some(producer);
        self.metadata.producer_version = Some(version);

        Ok(self)
    }

    /// Sets optimization profile metadata.
    pub fn profile(
        &mut self,
        profile: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let profile = profile.into();

        validate_text(
            "profile",
            &profile,
            self.limits.max_field_bytes,
        )?;

        self.metadata.profile = Some(profile);

        Ok(self)
    }

    /// Sets target metadata.
    pub fn target(
        &mut self,
        target: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let target = target.into();

        validate_text(
            "target",
            &target,
            self.limits.max_field_bytes,
        )?;

        self.metadata.target = Some(target);

        Ok(self)
    }

    /// Sets optimization objective metadata.
    pub fn objective(
        &mut self,
        objective: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let objective = objective.into();

        validate_text(
            "objective",
            &objective,
            self.limits.max_field_bytes,
        )?;

        self.metadata.objective = Some(objective);

        Ok(self)
    }

    /// Sets a deterministic compilation seed.
    pub const fn seed(
        &mut self,
        seed: u64,
    ) -> &mut Self {
        self.metadata.seed = Some(seed);
        self
    }

    /// Sets a compilation identity.
    pub fn compilation_id(
        &mut self,
        compilation_id: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let compilation_id = compilation_id.into();

        validate_text(
            "compilation_id",
            &compilation_id,
            self.limits.max_field_bytes,
        )?;

        self.metadata.compilation_id = Some(compilation_id);

        Ok(self)
    }

    /// Sets an audit note.
    pub fn note(
        &mut self,
        note: impl Into<String>,
    ) -> CertificateResult<&mut Self> {
        let note = note.into();

        validate_text(
            "note",
            &note,
            self.limits.max_field_bytes,
        )?;

        self.metadata.note = Some(note);

        Ok(self)
    }

    /// Adds a parent certificate reference.
    pub fn add_parent(
        &mut self,
        parent: ContentHash,
    ) -> CertificateResult<&mut Self> {
        parent.validate()?;

        if self.parents.len() >= self.limits.max_parents {
            return Err(CertificateError::ParentLimitExceeded {
                actual: self.parents.len() + 1,
                maximum: self.limits.max_parents,
            });
        }

        self.parents.push(parent);

        Ok(self)
    }

    /// Adds verification evidence.
    pub fn add_evidence(
        &mut self,
        evidence: VerificationEvidence,
    ) -> CertificateResult<&mut Self> {
        evidence.validate()?;

        if self.evidence.len() >= self.limits.max_evidence {
            return Err(CertificateError::EvidenceLimitExceeded {
                actual: self.evidence.len() + 1,
                maximum: self.limits.max_evidence,
            });
        }

        self.evidence.push(evidence);

        Ok(self)
    }

    /// Sets a deterministic sequence number.
    #[must_use]
    pub const fn sequence(
        &mut self,
        sequence: u128,
    ) -> &mut Self {
        self.sequence = Some(sequence);
        self
    }

    /// Builds and cryptographically finalizes the certificate.
    pub fn build(
        self,
    ) -> CertificateResult<VerificationCertificate> {
        self.limits.validate()?;

        let status = self
            .status
            .unwrap_or(CertificateStatus::Inconclusive);

        let certificate = VerificationCertificate {
            schema_version: CERTIFICATE_SCHEMA_VERSION,
            certificate_type: CERTIFICATE_ID.to_owned(),
            status,
            circuits: self.circuits,
            evidence: self.evidence,
            metadata: self.metadata,
            parents: self.parents,
            sequence: self.sequence,
            certificate_digest: ContentHash::sha256(&[]),
        };

        validate_certificate_with_limits(
            &certificate,
            self.limits,
            false,
        )?;

        let digest = certificate.recompute_digest()?;

        let finalized = VerificationCertificate {
            certificate_digest: digest,
            ..certificate
        };

        validate_certificate_with_limits(
            &finalized,
            self.limits,
            true,
        )?;

        Ok(finalized)
    }
}

// =============================================================================
// Evidence constructors
// =============================================================================

/// Creates normalized exact-semantic evidence.
///
/// This helper is intentionally independent of the concrete semantic verifier
/// report so `certificates.rs` remains stable when that implementation grows.
pub fn exact_semantic_evidence(
    verifier_id: impl Into<String>,
    verifier_version: u32,
    verdict: impl Into<String>,
    ignores_global_phase: bool,
    max_error: Option<f64>,
    logical_qubits: Option<u128>,
    original_operations: Option<u128>,
    optimized_operations: Option<u128>,
) -> CertificateResult<VerificationEvidence> {
    let verdict = verdict.into();

    let proven = verdict.eq_ignore_ascii_case("equivalent");

    let kind = if ignores_global_phase {
        EvidenceKind::ExactSemanticUpToGlobalPhase
    } else {
        EvidenceKind::ExactSemantic
    };

    let mut evidence = VerificationEvidence::new(
        kind,
        verifier_id,
        verifier_version,
        verdict,
        proven,
    )?;

    evidence.ignores_global_phase = ignores_global_phase;
    evidence.logical_qubits = logical_qubits;
    evidence.original_operations = original_operations;
    evidence.optimized_operations = optimized_operations;

    if let Some(error) = max_error {
        evidence = evidence.with_max_error(error)?;
    }

    Ok(evidence)
}

/// Creates structural-equivalence evidence.
pub fn structural_evidence(
    verifier_id: impl Into<String>,
    verifier_version: u32,
    equivalent: bool,
    logical_qubits: Option<u128>,
    original_operations: Option<u128>,
    optimized_operations: Option<u128>,
) -> CertificateResult<VerificationEvidence> {
    let verdict = if equivalent {
        "equivalent"
    } else {
        "not_equivalent"
    };

    VerificationEvidence::new(
        EvidenceKind::Structural,
        verifier_id,
        verifier_version,
        verdict,
        equivalent,
    )
    .map(|evidence| VerificationEvidence {
        logical_qubits,
        original_operations,
        optimized_operations,
        ..evidence
    })
}

/// Creates randomized verification evidence.
///
/// This function deliberately returns `proof = false`, even when every trial
/// matches. Randomized verification is statistical evidence, not exact proof.
pub fn randomized_evidence(
    verifier_id: impl Into<String>,
    verifier_version: u32,
    verdict: impl Into<String>,
    trials: u128,
    matches: u128,
    mismatches: u128,
    inconclusive: u128,
    logical_qubits: Option<u128>,
) -> CertificateResult<VerificationEvidence> {
    let verdict = verdict.into();

    let mut evidence = VerificationEvidence::new(
        EvidenceKind::Randomized,
        verifier_id,
        verifier_version,
        verdict,
        false,
    )?;

    evidence.trials = Some(trials);
    evidence.matches = Some(matches);
    evidence.mismatches = Some(mismatches);
    evidence.inconclusive = Some(inconclusive);
    evidence.logical_qubits = logical_qubits;

    Ok(evidence)
}

/// Creates exhaustive verification evidence.
///
/// `proof` is caller-controlled only through the explicit `complete_domain`
/// flag. An incomplete exhaustive sample is treated as evidence only.
pub fn exhaustive_evidence(
    verifier_id: impl Into<String>,
    verifier_version: u32,
    verdict: impl Into<String>,
    complete_domain: bool,
    trials: u128,
    matches: u128,
    mismatches: u128,
    logical_qubits: Option<u128>,
) -> CertificateResult<VerificationEvidence> {
    let verdict = verdict.into();

    let proof = complete_domain
        && verdict.eq_ignore_ascii_case("equivalent");

    let mut evidence = VerificationEvidence::new(
        EvidenceKind::Exhaustive,
        verifier_id,
        verifier_version,
        verdict,
        proof,
    )?;

    evidence.trials = Some(trials);
    evidence.matches = Some(matches);
    evidence.mismatches = Some(mismatches);
    evidence.logical_qubits = logical_qubits;

    if !complete_domain {
        evidence.reason = Some(
            "exhaustive evidence did not cover the complete semantic domain"
                .to_owned(),
        );
    }

    Ok(evidence)
}

// =============================================================================
// Certificate validation
// =============================================================================

fn validate_certificate(
    certificate: &VerificationCertificate,
) -> CertificateResult<()> {
    validate_certificate_with_limits(
        certificate,
        CertificateLimits::default(),
        true,
    )
}

fn validate_certificate_with_limits(
    certificate: &VerificationCertificate,
    limits: CertificateLimits,
    verify_digest: bool,
) -> CertificateResult<()> {
    limits.validate()?;

    if certificate.schema_version != CERTIFICATE_SCHEMA_VERSION {
        return Err(CertificateError::UnsupportedSchemaVersion {
            version: certificate.schema_version,
        });
    }

    if certificate.certificate_type != CERTIFICATE_ID {
        return Err(CertificateError::InvalidEvidence {
            reason: format!(
                "unexpected certificate type `{}`",
                certificate.certificate_type
            ),
        });
    }

    validate_text(
        "certificate_type",
        &certificate.certificate_type,
        limits.max_field_bytes,
    )?;

    certificate.circuits.validate()?;
    certificate.metadata.validate()?;

    if certificate.evidence.len() > limits.max_evidence {
        return Err(CertificateError::EvidenceLimitExceeded {
            actual: certificate.evidence.len(),
            maximum: limits.max_evidence,
        });
    }

    if certificate.parents.len() > limits.max_parents {
        return Err(CertificateError::ParentLimitExceeded {
            actual: certificate.parents.len(),
            maximum: limits.max_parents,
        });
    }

    for evidence in &certificate.evidence {
        evidence.validate()?;

        validate_evidence_text(
            evidence,
            limits.max_field_bytes,
        )?;
    }

    for parent in &certificate.parents {
        parent.validate()?;
    }

    validate_status(certificate)?;

    if verify_digest {
        let recomputed = certificate.recompute_digest()?;

        if recomputed != certificate.certificate_digest {
            return Err(CertificateError::IntegrityMismatch {
                expected: certificate.certificate_digest.value.clone(),
                actual: recomputed.value,
            });
        }
    }

    Ok(())
}

fn validate_status(
    certificate: &VerificationCertificate,
) -> CertificateResult<()> {
    match certificate.status {
        CertificateStatus::Proven => {
            if certificate.evidence.is_empty() {
                return Err(CertificateError::InvalidStatus {
                    reason: "a proven certificate requires proof evidence"
                        .to_owned(),
                });
            }

            if !certificate
                .evidence
                .iter()
                .any(|evidence| {
                    evidence.proof
                        && evidence.kind.can_prove_exact_equivalence()
                        && evidence
                            .verdict
                            .eq_ignore_ascii_case("equivalent")
                })
            {
                return Err(CertificateError::InvalidStatus {
                    reason: "proven certificate has no exact equivalence proof"
                        .to_owned(),
                });
            }
        }

        CertificateStatus::Disproven => {
            if !certificate.evidence.iter().any(|evidence| {
                evidence.verdict.eq_ignore_ascii_case(
                    "not_equivalent",
                ) || evidence.verdict.eq_ignore_ascii_case(
                    "counterexample_found",
                )
            }) {
                return Err(CertificateError::InvalidStatus {
                    reason: "disproven certificate has no explicit \
                             non-equivalence evidence"
                        .to_owned(),
                });
            }
        }

        CertificateStatus::EvidenceOnly => {
            if certificate.evidence.is_empty() {
                return Err(CertificateError::InvalidStatus {
                    reason: "evidence-only certificate contains no evidence"
                        .to_owned(),
                });
            }

            if certificate.evidence.iter().any(|evidence| {
                evidence.proof
                    && evidence.kind.can_prove_exact_equivalence()
                    && evidence
                        .verdict
                        .eq_ignore_ascii_case("equivalent")
            }) {
                return Err(CertificateError::InvalidStatus {
                    reason: "evidence-only certificate contains exact proof \
                             evidence; status should be proven"
                        .to_owned(),
                });
            }
        }

        CertificateStatus::Inconclusive => {
            if certificate.evidence.is_empty() {
                return Err(CertificateError::InvalidStatus {
                    reason: "inconclusive certificate should retain at least \
                             one verification record"
                        .to_owned(),
                });
            }
        }

        CertificateStatus::Invalid => {
            return Err(CertificateError::InvalidStatus {
                reason: "invalid certificates cannot be finalized"
                    .to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_evidence_text(
    evidence: &VerificationEvidence,
    maximum: u64,
) -> CertificateResult<()> {
    validate_text(
        "evidence.verifier_id",
        &evidence.verifier_id,
        maximum,
    )?;

    validate_text(
        "evidence.verdict",
        &evidence.verdict,
        maximum,
    )?;

    if let Some(reason) = evidence.reason.as_deref() {
        validate_text(
            "evidence.reason",
            reason,
            maximum,
        )?;
    }

    Ok(())
}

// =============================================================================
// Text validation
// =============================================================================

fn validate_text(
    field: &str,
    value: &str,
    maximum: u64,
) -> CertificateResult<()> {
    if value.is_empty() {
        return Err(CertificateError::EmptyField {
            field: field.to_owned(),
        });
    }

    let actual = value.as_bytes().len() as u64;

    if actual > maximum {
        return Err(CertificateError::FieldTooLarge {
            field: field.to_owned(),
            actual_bytes: actual,
            maximum_bytes: maximum,
        });
    }

    Ok(())
}

fn validate_optional_text(
    field: &str,
    value: Option<&str>,
    maximum: u64,
) -> CertificateResult<()> {
    if let Some(value) = value {
        validate_text(field, value, maximum)?;
    }

    Ok(())
}

// =============================================================================
// Hexadecimal encoding
// =============================================================================

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let capacity = bytes
        .len()
        .checked_mul(2)
        .unwrap_or(0);

    let mut output = String::with_capacity(capacity);

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

fn decode_hex(value: &str) -> CertificateResult<Vec<u8>> {
    if value.len() % 2 != 0 {
        return Err(CertificateError::InvalidDigestEncoding);
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    let raw = value.as_bytes();

    let mut index = 0usize;

    while index < raw.len() {
        let high = hex_value(raw[index])
            .ok_or(CertificateError::InvalidDigestEncoding)?;

        let low = hex_value(raw[index + 1])
            .ok_or(CertificateError::InvalidDigestEncoding)?;

        bytes.push((high << 4) | low);

        index += 2;
    }

    Ok(bytes)
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn circuit_identities() -> CircuitIdentities {
        CircuitIdentities::new(
            ContentHash::sha256_text("original-circuit"),
            ContentHash::sha256_text("optimized-circuit"),
        )
        .expect("valid circuit identities")
        .with_logical_qubits(3)
        .with_operation_counts(10, 6)
    }

    #[test]
    fn sha256_identity_is_64_hex_characters() {
        let hash = ContentHash::sha256_text("zamani");

        assert_eq!(hash.algorithm, CERTIFICATE_HASH_ALGORITHM);
        assert_eq!(hash.value.len(), 64);
        assert!(hash.validate().is_ok());
    }

    #[test]
    fn randomized_evidence_is_never_proof() {
        let evidence = randomized_evidence(
            "test.randomized",
            1,
            "no_counterexample",
            100,
            100,
            0,
            0,
            Some(3),
        )
        .expect("valid evidence");

        assert!(!evidence.proof);
        assert_eq!(evidence.kind, EvidenceKind::Randomized);
    }

    #[test]
    fn exact_evidence_can_prove() {
        let evidence = exact_semantic_evidence(
            "test.semantic",
            1,
            "equivalent",
            true,
            Some(0.0),
            Some(2),
            Some(4),
            Some(2),
        )
        .expect("valid exact evidence");

        assert!(evidence.proof);
        assert_eq!(
            evidence.kind,
            EvidenceKind::ExactSemanticUpToGlobalPhase
        );
    }

    #[test]
    fn exact_certificate_round_trips() {
        let evidence = exact_semantic_evidence(
            "test.semantic",
            1,
            "equivalent",
            true,
            Some(0.0),
            Some(2),
            Some(4),
            Some(2),
        )
        .expect("valid evidence");

        let certificate =
            VerificationCertificate::from_exact_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("valid certificate");

        assert!(certificate.is_proven());
        assert!(certificate.validate().is_ok());

        let json = certificate
            .to_json()
            .expect("serializable certificate");

        let restored =
            VerificationCertificate::from_json(
                &json,
                CertificateLimits::default(),
            )
            .expect("restored certificate");

        assert_eq!(certificate, restored);
    }

    #[test]
    fn randomized_certificate_is_evidence_only() {
        let evidence = randomized_evidence(
            "test.randomized",
            1,
            "no_counterexample",
            1_000,
            1_000,
            0,
            0,
            Some(4),
        )
        .expect("valid randomized evidence");

        let certificate =
            VerificationCertificate::from_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("valid certificate");

        assert!(certificate.is_evidence_only());
        assert!(!certificate.is_proven());
        assert!(certificate.validate().is_ok());
    }

    #[test]
    fn inconclusive_evidence_is_not_success() {
        let evidence = randomized_evidence(
            "test.randomized",
            1,
            "inconclusive",
            10,
            8,
            0,
            2,
            Some(4),
        )
        .expect("valid evidence");

        let certificate =
            VerificationCertificate::from_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("valid certificate");

        assert!(certificate.is_inconclusive());
        assert!(!certificate.is_proven());
    }

    #[test]
    fn tampering_is_detected() {
        let evidence = exact_semantic_evidence(
            "test.semantic",
            1,
            "equivalent",
            false,
            Some(0.0),
            Some(1),
            Some(2),
            Some(2),
        )
        .expect("valid evidence");

        let mut certificate =
            VerificationCertificate::from_exact_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("valid certificate");

        certificate.metadata.profile =
            Some("tampered".to_owned());

        assert!(matches!(
            certificate.validate(),
            Err(CertificateError::IntegrityMismatch { .. })
        ));
    }

    #[test]
    fn invalid_randomized_proof_is_rejected() {
        let result = VerificationEvidence::new(
            EvidenceKind::Randomized,
            "test",
            1,
            "equivalent",
            true,
        );

        assert!(matches!(
            result,
            Err(CertificateError::InvalidEvidence { .. })
        ));
    }

    #[test]
    fn parent_chain_is_supported() {
        let evidence = exact_semantic_evidence(
            "test.semantic",
            1,
            "equivalent",
            false,
            Some(0.0),
            Some(1),
            Some(2),
            Some(2),
        )
        .expect("valid evidence");

        let parent = ContentHash::sha256_text("previous-certificate");

        let mut builder =
            VerificationCertificate::builder(circuit_identities());

        builder
            .add_parent(parent)
            .expect("parent")
            .add_evidence(evidence)
            .expect("evidence");

        builder.set_status(CertificateStatus::Proven);

        let certificate =
            builder.build().expect("certificate");

        assert_eq!(certificate.parents.len(), 1);
        assert!(certificate.validate().is_ok());
    }

    #[test]
    fn certificate_digest_is_stable() {
        let evidence = exact_semantic_evidence(
            "test.semantic",
            1,
            "equivalent",
            false,
            Some(0.0),
            Some(1),
            Some(2),
            Some(2),
        )
        .expect("valid evidence");

        let certificate_a =
            VerificationCertificate::from_exact_evidence(
                circuit_identities(),
                evidence.clone(),
            )
            .expect("certificate A");

        let certificate_b =
            VerificationCertificate::from_exact_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("certificate B");

        assert_eq!(
            certificate_a.certificate_digest,
            certificate_b.certificate_digest
        );
    }

    #[test]
    fn evidence_limit_is_enforced() {
        let limits = CertificateLimits::new(
            1,
            DEFAULT_MAX_PARENTS,
            DEFAULT_MAX_FIELD_BYTES,
            1024 * 1024,
        );

        let mut builder =
            VerificationCertificate::builder(circuit_identities())
                .with_limits(limits)
                .expect("limits");

        let evidence = randomized_evidence(
            "test.randomized",
            1,
            "no_counterexample",
            1,
            1,
            0,
            0,
            Some(1),
        )
        .expect("evidence");

        builder
            .add_evidence(evidence.clone())
            .expect("first evidence");

        let second =
            builder.add_evidence(evidence);

        assert!(matches!(
            second,
            Err(CertificateError::EvidenceLimitExceeded { .. })
        ));
    }

    #[test]
    fn malformed_hash_is_rejected() {
        let result = ContentHash::parse_sha256("abcd");

        assert!(matches!(
            result,
            Err(CertificateError::InvalidDigestLength { .. })
        ));
    }

    #[test]
    fn status_never_converts_statistical_evidence_to_proof() {
        let evidence = randomized_evidence(
            "test.randomized",
            1,
            "no_counterexample",
            10_000,
            10_000,
            0,
            0,
            Some(20),
        )
        .expect("evidence");

        let result =
            VerificationCertificate::from_evidence(
                circuit_identities(),
                evidence,
            )
            .expect("certificate");

        assert_eq!(
            result.status,
            CertificateStatus::EvidenceOnly
        );
        assert!(!result.is_proven());
    }
}