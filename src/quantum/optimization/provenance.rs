//! Zamani Quantum Optimization — Reproducible Optimization Provenance
//!
//! This module defines the canonical provenance model for the quantum
//! optimization subsystem.
//!
//! # Architectural role
//!
//! Provenance answers:
//!
//! - What was optimized?
//! - Which optimizer/compiler produced the result?
//! - Which optimization configuration/profile was used?
//! - Which target constraints were active?
//! - Which passes ran?
//! - In what order did they run?
//! - Which rewrite rules were applied?
//! - Which analyses were requested?
//! - Which verification procedures were performed?
//! - Which deterministic/random seed was used?
//! - Which input/output content identities were observed?
//! - Which resource limits were active?
//! - Was optimization complete, partial, skipped, or limit-bound?
//!
//! The dependency direction is intentionally:
//!
//! ```text
//!                 quantum::ir
//!                     │
//!                     ▼
//!               optimization
//!                     │
//!          ┌──────────┼──────────┐
//!          ▼          ▼          ▼
//!       passes      verifier   analyses
//!          │          │          │
//!          └──────────┼──────────┘
//!                     ▼
//!                provenance
//!                     │
//!          ┌──────────┼──────────┐
//!          ▼          ▼          ▼
//!       result    reporting   benchmarking
//! ```
//!
//! `provenance.rs` is an observational subsystem. It never changes the
//! semantics of the optimized circuit.
//!
//! # Design principles
//!
//! ## 1. Provenance is append-oriented
//!
//! Optimization can contain thousands, millions, or more individual events.
//! Provenance therefore uses append-oriented records rather than a structure
//! requiring the entire optimization history to be represented by a deeply
//! nested object.
//!
//! ## 2. Provenance is deterministic when the compilation is deterministic
//!
//! The provenance model records the deterministic/randomization policy and seed
//! explicitly. It does not manufacture nondeterministic identifiers and does
//! not use process-global state.
//!
//! ## 3. Provenance does not invent hashes
//!
//! A content identity is represented by [`ContentHash`]. The hashing
//! implementation belongs to the repository's canonical hashing facility.
//! This module validates and records supplied digests but does not implement a
//! competing cryptographic hash.
//!
//! ## 4. Provenance is safe to serialize
//!
//! All public data structures derive `Serialize` and `Deserialize`.
//!
//! Serialization is representation-only. This module performs no filesystem,
//! network, Git, compiler, or hardware I/O.
//!
//! ## 5. Provenance is bounded
//!
//! Extremely large optimization jobs must not cause provenance itself to grow
//! without control. [`ProvenanceLimits`] provides explicit limits and a
//! [`ProvenanceMode`] allowing callers to choose full, bounded, summary, or
//! disabled collection.
//!
//! ## 6. Provenance is forward-extensible
//!
//! Future optimizer modules can identify themselves using strings and stable
//! identifiers without requiring this foundational module to import every
//! future optimization module.
//!
//! ## 7. No unsafe code
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//!
//! # Integration contract
//!
//! `context.rs` may own an instance of [`OptimizationProvenance`] or a
//! provenance service built around it.
//!
//! `pass.rs` records pass start/completion.
//!
//! `rewrite.rs` records rewrite applications.
//!
//! `pipeline.rs` records pipeline stages.
//!
//! `verification/*` records verification events.
//!
//! `result.rs` embeds the final provenance snapshot.
//!
//! `serialization/*` serializes this module directly.
//!
//! `benchmarking` consumes the resulting provenance but is not a dependency
//! of this module.
//!
//! No future module needs to modify this file merely because a new pass,
//! analysis, target, verifier, or optimization algorithm is added.
//!
//! # Important semantic rule
//!
//! Provenance is not proof.
//!
//! A provenance record saying that verification was requested does not itself
//! prove semantic equivalence. Verification modules own that proof/evidence.
//! Provenance records what verification actually reported.
//!
//! Similarly, an input hash does not prove that the source text, IR, compiler,
//! or backend was trustworthy. It provides an identity that can be checked by
//! an independent hashing implementation.
//!
//! # Scaling
//!
//! The model uses `u128` for counters and explicit bounded event collection.
//! It can represent workloads far beyond practical machine-scale execution
//! while still allowing memory usage to remain bounded.
//!
//! The implementation avoids recursive provenance structures and therefore
//! does not create a call-stack dependency proportional to pipeline depth.
//!
//! # Example
//!
//! ```ignore
//! let mut provenance = OptimizationProvenance::new(
//!     ProvenanceMode::Bounded,
//!     ProvenanceLimits::default(),
//! )?;
//!
//! provenance.set_optimizer(
//!     "zamani-quantum-optimizer",
//!     "1.0.0",
//! )?;
//!
//! provenance.set_input_hash(ContentHash::sha256(
//!     "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
//! )?)?;
//!
//! let pass = provenance.begin_pass(
//!     "local.cancellation",
//!     "local",
//!     1,
//! )?;
//!
//! provenance.record_rule(RuleApplication::new(
//!     "identity.self_inverse",
//!     pass,
//!     2,
//!     2,
//! ))?;
//!
//! provenance.end_pass(
//!     pass,
//!     PassProvenanceOutcome::Changed,
//! )?;
//!
//! provenance.set_output_hash(ContentHash::sha256(
//!     "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
//! )?)?;
//!
//! let snapshot = provenance.snapshot();
//! assert_eq!(snapshot.pass_count(), 1);
//! ```
//!
//! The exact optimizer pipeline is intentionally not coupled to this example.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// Result aliases
// =============================================================================

/// Result type returned by provenance operations.
pub type ProvenanceResult<T> = Result<T, ProvenanceError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the provenance subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceError {
    /// An identifier or textual field was empty.
    EmptyField {
        /// Name of the offending field.
        field: &'static str,
    },

    /// A textual field exceeded its configured size limit.
    FieldTooLarge {
        /// Name of the offending field.
        field: &'static str,

        /// Number of bytes supplied.
        actual_bytes: u64,

        /// Maximum permitted bytes.
        maximum_bytes: u64,
    },

    /// Too many events were requested.
    EventLimitExceeded {
        /// Event category.
        category: ProvenanceEventKind,

        /// Existing number of retained events.
        current: u64,

        /// Maximum number permitted.
        maximum: u64,
    },

    /// Too many passes were requested.
    PassLimitExceeded {
        /// Existing number of retained passes.
        current: u64,

        /// Maximum number permitted.
        maximum: u64,
    },

    /// Too many rewrite records were requested.
    RewriteLimitExceeded {
        /// Existing number of retained rewrites.
        current: u64,

        /// Maximum number permitted.
        maximum: u64,
    },

    /// Too many verification records were requested.
    VerificationLimitExceeded {
        /// Existing number of retained verification records.
        current: u64,

        /// Maximum number permitted.
        maximum: u64,
    },

    /// A pass identifier was reused while active.
    DuplicateActivePass {
        /// Pass identifier.
        pass_id: String,
    },

    /// An unknown pass handle was supplied.
    UnknownPass {
        /// Pass handle.
        handle: u64,
    },

    /// A pass was ended more than once.
    PassAlreadyEnded {
        /// Pass handle.
        handle: u64,
    },

    /// A hash algorithm was given a digest of an invalid size.
    InvalidDigestLength {
        /// Algorithm.
        algorithm: HashAlgorithm,

        /// Actual number of bytes represented by the digest.
        actual_bytes: u16,

        /// Required digest length.
        expected_bytes: u16,
    },

    /// A digest contained invalid hexadecimal characters.
    InvalidDigestEncoding {
        /// Algorithm.
        algorithm: HashAlgorithm,
    },

    /// A supplied timestamp was invalid.
    InvalidTimestamp {
        /// Timestamp value.
        value: i128,
    },

    /// An event sequence number could not be allocated.
    SequenceOverflow,

    /// An active-pass counter could not be incremented.
    PassHandleOverflow,

    /// Provenance was disabled and an operation attempted to require a
    /// retained provenance record.
    Disabled {
        /// Operation attempted.
        operation: &'static str,
    },

    /// An operation required retained detail but the configured mode does not
    /// retain that detail.
    DetailUnavailable {
        /// Requested detail.
        detail: &'static str,
    },

    /// A provenance state transition was invalid.
    InvalidState {
        /// Human-readable reason.
        reason: &'static str,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "provenance field `{field}` must not be empty")
            }

            Self::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "provenance field `{field}` is too large: \
                     {actual_bytes} bytes, maximum {maximum_bytes}"
                )
            }

            Self::EventLimitExceeded {
                category,
                current,
                maximum,
            } => {
                write!(
                    formatter,
                    "provenance event limit exceeded for `{category}`: \
                     current {current}, maximum {maximum}"
                )
            }

            Self::PassLimitExceeded { current, maximum } => {
                write!(
                    formatter,
                    "provenance pass limit exceeded: \
                     current {current}, maximum {maximum}"
                )
            }

            Self::RewriteLimitExceeded { current, maximum } => {
                write!(
                    formatter,
                    "provenance rewrite limit exceeded: \
                     current {current}, maximum {maximum}"
                )
            }

            Self::VerificationLimitExceeded { current, maximum } => {
                write!(
                    formatter,
                    "provenance verification limit exceeded: \
                     current {current}, maximum {maximum}"
                )
            }

            Self::DuplicateActivePass { pass_id } => {
                write!(
                    formatter,
                    "provenance pass `{pass_id}` is already active"
                )
            }

            Self::UnknownPass { handle } => {
                write!(
                    formatter,
                    "provenance pass handle `{handle}` is unknown"
                )
            }

            Self::PassAlreadyEnded { handle } => {
                write!(
                    formatter,
                    "provenance pass handle `{handle}` has already ended"
                )
            }

            Self::InvalidDigestLength {
                algorithm,
                actual_bytes,
                expected_bytes,
            } => {
                write!(
                    formatter,
                    "invalid {algorithm} digest length: \
                     {actual_bytes} bytes, expected {expected_bytes}"
                )
            }

            Self::InvalidDigestEncoding { algorithm } => {
                write!(
                    formatter,
                    "invalid hexadecimal encoding for {algorithm} digest"
                )
            }

            Self::InvalidTimestamp { value } => {
                write!(
                    formatter,
                    "invalid provenance timestamp `{value}`"
                )
            }

            Self::SequenceOverflow => {
                formatter.write_str("provenance event sequence overflow")
            }

            Self::PassHandleOverflow => {
                formatter.write_str("provenance pass handle overflow")
            }

            Self::Disabled { operation } => {
                write!(
                    formatter,
                    "provenance is disabled; operation `{operation}` \
                     cannot retain provenance"
                )
            }

            Self::DetailUnavailable { detail } => {
                write!(
                    formatter,
                    "requested provenance detail `{detail}` \
                     is unavailable in the current provenance mode"
                )
            }

            Self::InvalidState { reason } => {
                write!(
                    formatter,
                    "invalid provenance state: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

// =============================================================================
// Provenance mode
// =============================================================================

/// Controls how much provenance information is retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProvenanceMode {
    /// No provenance records are retained.
    ///
    /// This is appropriate when the caller explicitly wants provenance
    /// disabled to minimize memory overhead.
    Disabled,

    /// Retain a bounded amount of detailed provenance.
    ///
    /// Once a configured category reaches its limit, new records are rejected.
    /// This is the safest mode when provenance completeness matters.
    Bounded,

    /// Retain detailed provenance up to configured limits, but permit
    /// individual optional details to be omitted by callers.
    Summary,

    /// Intended for audit/reproducibility builds where detailed provenance is
    /// expected. Limits still apply to prevent accidental unbounded memory
    /// consumption.
    Full,
}

impl Default for ProvenanceMode {
    fn default() -> Self {
        Self::Bounded
    }
}

impl ProvenanceMode {
    /// Returns true if provenance collection is enabled.
    #[must_use]
    pub const fn enabled(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns true if detailed records should be retained.
    #[must_use]
    pub const fn retains_details(self) -> bool {
        matches!(self, Self::Bounded | Self::Full)
    }
}

// =============================================================================
// Provenance limits
// =============================================================================

/// Explicit limits protecting provenance memory usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceLimits {
    /// Maximum number of retained pass records.
    pub max_passes: u64,

    /// Maximum number of retained rewrite records.
    pub max_rewrites: u64,

    /// Maximum number of retained verification records.
    pub max_verifications: u64,

    /// Maximum number of retained analysis records.
    pub max_analyses: u64,

    /// Maximum number of retained pipeline-stage records.
    pub max_pipeline_stages: u64,

    /// Maximum number of retained generic events.
    pub max_events: u64,

    /// Maximum number of bytes for one textual field.
    pub max_field_bytes: u64,

    /// Maximum number of bytes for the entire provenance snapshot's optional
    /// textual metadata.
    pub max_metadata_bytes: u64,
}

impl Default for ProvenanceLimits {
    fn default() -> Self {
        Self {
            max_passes: 1_000_000,
            max_rewrites: 10_000_000,
            max_verifications: 1_000_000,
            max_analyses: 1_000_000,
            max_pipeline_stages: 1_000_000,
            max_events: 10_000_000,
            max_field_bytes: 1 << 20,
            max_metadata_bytes: 64 << 20,
        }
    }
}

impl ProvenanceLimits {
    /// A compact configuration useful for small compilation jobs.
    #[must_use]
    pub const fn compact() -> Self {
        Self {
            max_passes: 1_024,
            max_rewrites: 16_384,
            max_verifications: 1_024,
            max_analyses: 1_024,
            max_pipeline_stages: 1_024,
            max_events: 32_768,
            max_field_bytes: 64 * 1024,
            max_metadata_bytes: 4 * 1024 * 1024,
        }
    }

    /// A large-workload configuration.
    #[must_use]
    pub const fn large() -> Self {
        Self {
            max_passes: 10_000_000,
            max_rewrites: 100_000_000,
            max_verifications: 10_000_000,
            max_analyses: 10_000_000,
            max_pipeline_stages: 10_000_000,
            max_events: 100_000_000,
            max_field_bytes: 4 << 20,
            max_metadata_bytes: 512 << 20,
        }
    }

    /// An audit-oriented configuration.
    ///
    /// Limits remain finite intentionally. "Unlimited" provenance is unsafe
    /// from a memory-management perspective.
    #[must_use]
    pub const fn audit() -> Self {
        Self {
            max_passes: 50_000_000,
            max_rewrites: 500_000_000,
            max_verifications: 50_000_000,
            max_analyses: 50_000_000,
            max_pipeline_stages: 50_000_000,
            max_events: 1_000_000_000,
            max_field_bytes: 16 << 20,
            max_metadata_bytes: 2 << 30,
        }
    }
}

// =============================================================================
// Hashes
// =============================================================================

/// Supported content-hash algorithms.
///
/// The provenance layer records hashes but intentionally does not implement
/// hashing. This avoids having provenance become the canonical hashing library.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256.
    Sha256,

    /// SHA-512.
    Sha512,

    /// BLAKE3.
    Blake3,
}

impl HashAlgorithm {
    /// Returns the digest size in bytes.
    #[must_use]
    pub const fn digest_bytes(self) -> u16 {
        match self {
            Self::Sha256 => 32,
            Self::Sha512 => 64,
            Self::Blake3 => 32,
        }
    }

    /// Returns the canonical lowercase algorithm name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Sha512 => "sha512",
            Self::Blake3 => "blake3",
        }
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validated hexadecimal content digest.
///
/// The digest is stored in lowercase hexadecimal form. The constructor
/// validates both the character set and the expected digest size.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash {
    algorithm: HashAlgorithm,
    hexadecimal: String,
}

impl ContentHash {
    /// Creates a validated SHA-256 digest.
    pub fn sha256(value: impl Into<String>) -> ProvenanceResult<Self> {
        Self::new(HashAlgorithm::Sha256, value)
    }

    /// Creates a validated SHA-512 digest.
    pub fn sha512(value: impl Into<String>) -> ProvenanceResult<Self> {
        Self::new(HashAlgorithm::Sha512, value)
    }

    /// Creates a validated BLAKE3 digest.
    pub fn blake3(value: impl Into<String>) -> ProvenanceResult<Self> {
        Self::new(HashAlgorithm::Blake3, value)
    }

    /// Creates a digest for the supplied algorithm.
    pub fn new(
        algorithm: HashAlgorithm,
        value: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let value = value.into();

        let expected_chars =
            usize::from(algorithm.digest_bytes()) * 2;

        if value.len() != expected_chars {
            return Err(ProvenanceError::InvalidDigestLength {
                algorithm,
                actual_bytes: (value.len() / 2) as u16,
                expected_bytes: algorithm.digest_bytes(),
            });
        }

        if !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ProvenanceError::InvalidDigestEncoding {
                algorithm,
            });
        }

        Ok(Self {
            algorithm,
            hexadecimal: value.to_ascii_lowercase(),
        })
    }

    /// Returns the algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }

    /// Returns the hexadecimal digest.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.hexadecimal
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.algorithm.as_str(),
            self.hexadecimal
        )
    }
}

// =============================================================================
// Determinism
// =============================================================================

/// Determinism policy recorded in provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeterminismMode {
    /// No deterministic guarantee was requested.
    Nondeterministic,

    /// Deterministic execution was requested.
    Deterministic,

    /// Deterministic execution was requested and a fixed seed was supplied.
    Seeded,

    /// The optimizer was deterministic without a random seed.
    DeterministicWithoutRandomness,
}

impl Default for DeterminismMode {
    fn default() -> Self {
        Self::DeterministicWithoutRandomness
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Portable Unix timestamp represented as nanoseconds.
///
/// `SystemTime` itself is intentionally not serialized in this model. This
/// representation is stable and platform-independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProvenanceTimestamp {
    /// Nanoseconds since the Unix epoch.
    nanos_since_epoch: u64,
}

impl ProvenanceTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    #[must_use]
    pub const fn from_unix_nanos(nanos_since_epoch: u64) -> Self {
        Self { nanos_since_epoch }
    }

    /// Returns Unix nanoseconds.
    #[must_use]
    pub const fn unix_nanos(self) -> u64 {
        self.nanos_since_epoch
    }

    /// Captures the current system time.
    ///
    /// If the platform clock is before the Unix epoch, this returns a
    /// `SystemTime` error instead of manufacturing a timestamp.
    pub fn now() -> ProvenanceResult<Self> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ProvenanceError::InvalidTimestamp { value: -1 })?;

        let nanos = duration
            .as_nanos()
            .try_into()
            .map_err(|_| ProvenanceError::InvalidTimestamp {
                value: i128::MAX,
            })?;

        Ok(Self::from_unix_nanos(nanos))
    }
}

// =============================================================================
// Provenance identity
// =============================================================================

/// Stable identity of one optimization invocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProvenanceId(String);

impl ProvenanceId {
    /// Creates a validated provenance identifier.
    pub fn new(value: impl Into<String>) -> ProvenanceResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "provenance_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProvenanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Pass provenance
// =============================================================================

/// Outcome recorded when a pass finishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassProvenanceOutcome {
    /// Circuit changed.
    Changed,

    /// Circuit did not change.
    Unchanged,

    /// Pass was skipped.
    Skipped,

    /// Resource limit stopped the pass.
    LimitReached,

    /// Verification failed.
    VerificationFailed,

    /// Pass completed partially.
    PartiallyCompleted,

    /// Pass failed.
    Failed,
}

/// One optimization-pass provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassProvenance {
    /// Monotonically allocated pass handle within this provenance instance.
    pub handle: u64,

    /// Stable pass identifier.
    pub pass_id: String,

    /// Broad optimization phase.
    pub phase: String,

    /// Pipeline position.
    pub ordinal: u64,

    /// Start timestamp.
    pub started_at: ProvenanceTimestamp,

    /// End timestamp, when complete.
    pub ended_at: Option<ProvenanceTimestamp>,

    /// Pass outcome.
    pub outcome: Option<PassProvenanceOutcome>,

    /// Number of rewrites attributed to this pass.
    pub rewrites: u128,

    /// Number of analyses requested by this pass.
    pub analyses: u128,

    /// Number of verification operations requested by this pass.
    pub verification_operations: u128,

    /// Optional pass configuration identity.
    pub configuration_hash: Option<ContentHash>,
}

impl PassProvenance {
    /// Returns true when this pass has ended.
    #[must_use]
    pub const fn ended(&self) -> bool {
        self.ended_at.is_some() && self.outcome.is_some()
    }
}

// =============================================================================
// Rewrite provenance
// =============================================================================

/// One applied rewrite rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleApplication {
    /// Stable rewrite-rule identifier.
    pub rule_id: String,

    /// Pass handle responsible for the rewrite.
    pub pass_handle: u64,

    /// Number of operations before the rewrite.
    pub operations_before: u128,

    /// Number of operations after the rewrite.
    pub operations_after: u128,

    /// Optional semantic-equivalence verification reference.
    pub verification_reference: Option<String>,
}

impl RuleApplication {
    /// Creates a rewrite record.
    pub fn new(
        rule_id: impl Into<String>,
        pass_handle: u64,
        operations_before: u128,
        operations_after: u128,
    ) -> ProvenanceResult<Self> {
        let rule_id = rule_id.into();

        if rule_id.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "rule_id",
            });
        }

        Ok(Self {
            rule_id,
            pass_handle,
            operations_before,
            operations_after,
            verification_reference: None,
        })
    }

    /// Attaches a verification reference.
    pub fn with_verification_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let reference = reference.into();

        if reference.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "verification_reference",
            });
        }

        self.verification_reference = Some(reference);
        Ok(self)
    }
}

// =============================================================================
// Analysis provenance
// =============================================================================

/// One analysis request/observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisProvenance {
    /// Stable analysis identifier.
    pub analysis_id: String,

    /// Analysis generation before the request.
    pub generation: u64,

    /// Whether a cached value was used.
    pub cache_hit: bool,

    /// Number of logical work units reported.
    pub work_units: u128,
}

impl AnalysisProvenance {
    /// Creates an analysis record.
    pub fn new(
        analysis_id: impl Into<String>,
        generation: u64,
        cache_hit: bool,
        work_units: u128,
    ) -> ProvenanceResult<Self> {
        let analysis_id = analysis_id.into();

        if analysis_id.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "analysis_id",
            });
        }

        Ok(Self {
            analysis_id,
            generation,
            cache_hit,
            work_units,
        })
    }
}

// =============================================================================
// Verification provenance
// =============================================================================

/// Verification result classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VerificationProvenanceStatus {
    /// Verification succeeded.
    Passed,

    /// Verification failed.
    Failed,

    /// Verification was inconclusive.
    Inconclusive,

    /// Verification was not performed.
    NotPerformed,

    /// Verification was skipped due to configured policy.
    Skipped,

    /// Verification was stopped by resource limits.
    LimitReached,
}

/// Verification method.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerificationMethod(String);

impl VerificationMethod {
    /// Creates a verification-method identifier.
    pub fn new(value: impl Into<String>) -> ProvenanceResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "verification_method",
            });
        }

        Ok(Self(value))
    }

    /// Returns the method name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One verification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationProvenance {
    /// Verification method.
    pub method: VerificationMethod,

    /// Verification status.
    pub status: VerificationProvenanceStatus,

    /// Number of operations examined.
    pub operations_examined: u128,

    /// Number of qubits examined.
    pub qubits_examined: u128,

    /// Number of samples used, where applicable.
    pub samples: u128,

    /// Optional verifier-specific reference.
    pub reference: Option<String>,

    /// Optional verification evidence hash.
    pub evidence_hash: Option<ContentHash>,
}

impl VerificationProvenance {
    /// Creates a verification record.
    pub fn new(
        method: VerificationMethod,
        status: VerificationProvenanceStatus,
        operations_examined: u128,
        qubits_examined: u128,
        samples: u128,
    ) -> Self {
        Self {
            method,
            status,
            operations_examined,
            qubits_examined,
            samples,
            reference: None,
            evidence_hash: None,
        }
    }

    /// Attaches a reference.
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let reference = reference.into();

        if reference.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "verification_reference",
            });
        }

        self.reference = Some(reference);
        Ok(self)
    }

    /// Attaches an evidence hash.
    #[must_use]
    pub fn with_evidence_hash(
        mut self,
        hash: ContentHash,
    ) -> Self {
        self.evidence_hash = Some(hash);
        self
    }
}

// =============================================================================
// Pipeline provenance
// =============================================================================

/// One pipeline-stage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineStageProvenance {
    /// Stable stage identifier.
    pub stage_id: String,

    /// Pipeline ordinal.
    pub ordinal: u64,

    /// Whether the stage changed the circuit.
    pub changed: bool,

    /// Number of passes invoked by the stage.
    pub pass_count: u64,
}

impl PipelineStageProvenance {
    /// Creates a pipeline-stage record.
    pub fn new(
        stage_id: impl Into<String>,
        ordinal: u64,
        changed: bool,
        pass_count: u64,
    ) -> ProvenanceResult<Self> {
        let stage_id = stage_id.into();

        if stage_id.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "stage_id",
            });
        }

        Ok(Self {
            stage_id,
            ordinal,
            changed,
            pass_count,
        })
    }
}

// =============================================================================
// Generic events
// =============================================================================

/// Broad category for a generic provenance event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProvenanceEventKind {
    /// Optimization invocation started.
    InvocationStarted,

    /// Optimization invocation completed.
    InvocationCompleted,

    /// Pass started.
    PassStarted,

    /// Pass completed.
    PassCompleted,

    /// Rewrite applied.
    RewriteApplied,

    /// Analysis requested.
    AnalysisRequested,

    /// Verification executed.
    VerificationExecuted,

    /// Pipeline stage executed.
    PipelineStage,

    /// Resource limit reached.
    LimitReached,

    /// Cancellation observed.
    Cancelled,

    /// Custom optimizer event.
    Custom,
}

impl fmt::Display for ProvenanceEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::InvocationStarted => "invocation_started",
            Self::InvocationCompleted => "invocation_completed",
            Self::PassStarted => "pass_started",
            Self::PassCompleted => "pass_completed",
            Self::RewriteApplied => "rewrite_applied",
            Self::AnalysisRequested => "analysis_requested",
            Self::VerificationExecuted => "verification_executed",
            Self::PipelineStage => "pipeline_stage",
            Self::LimitReached => "limit_reached",
            Self::Cancelled => "cancelled",
            Self::Custom => "custom",
        };

        formatter.write_str(name)
    }
}

/// Generic compact event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceEvent {
    /// Monotonically increasing event sequence.
    pub sequence: u64,

    /// Event kind.
    pub kind: ProvenanceEventKind,

    /// Event timestamp.
    pub timestamp: ProvenanceTimestamp,

    /// Optional stable component identifier.
    pub component: Option<String>,

    /// Optional pass handle.
    pub pass_handle: Option<u64>,

    /// Optional compact message.
    pub message: Option<String>,
}

impl ProvenanceEvent {
    /// Creates an event.
    pub fn new(
        sequence: u64,
        kind: ProvenanceEventKind,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        Self {
            sequence,
            kind,
            timestamp,
            component: None,
            pass_handle: None,
            message: None,
        }
    }

    /// Adds a component identifier.
    pub fn with_component(
        mut self,
        component: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let component = component.into();

        if component.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "event_component",
            });
        }

        self.component = Some(component);
        Ok(self)
    }

    /// Adds a pass handle.
    #[must_use]
    pub const fn with_pass_handle(
        mut self,
        handle: u64,
    ) -> Self {
        self.pass_handle = Some(handle);
        self
    }

    /// Adds a message.
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let message = message.into();

        if message.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "event_message",
            });
        }

        self.message = Some(message);
        Ok(self)
    }
}

// =============================================================================
// Compiler identity
// =============================================================================

/// Identity of the compiler/optimizer implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizerIdentity {
    /// Product/compiler name.
    pub name: String,

    /// Optimizer implementation version.
    pub version: String,

    /// Optional source revision.
    pub source_revision: Option<String>,

    /// Optional source-tree content hash.
    pub source_hash: Option<ContentHash>,
}

impl OptimizerIdentity {
    /// Creates an optimizer identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let name = name.into();
        let version = version.into();

        if name.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "optimizer_name",
            });
        }

        if version.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "optimizer_version",
            });
        }

        Ok(Self {
            name,
            version,
            source_revision: None,
            source_hash: None,
        })
    }

    /// Attaches a source revision.
    pub fn with_source_revision(
        mut self,
        revision: impl Into<String>,
    ) -> ProvenanceResult<Self> {
        let revision = revision.into();

        if revision.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "source_revision",
            });
        }

        self.source_revision = Some(revision);
        Ok(self)
    }

    /// Attaches a source-tree hash.
    #[must_use]
    pub fn with_source_hash(
        mut self,
        hash: ContentHash,
    ) -> Self {
        self.source_hash = Some(hash);
        self
    }
}

// =============================================================================
// Invocation metadata
// =============================================================================

/// High-level identity/configuration metadata for one optimization invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationMetadata {
    /// Provenance schema version.
    pub schema_version: u32,

    /// Unique invocation identifier.
    pub provenance_id: ProvenanceId,

    /// Optimizer implementation identity.
    pub optimizer: OptimizerIdentity,

    /// Optimization profile identifier.
    pub profile: Option<String>,

    /// Optimization configuration identity.
    pub configuration_hash: Option<ContentHash>,

    /// Target identifier.
    pub target: Option<String>,

    /// Target configuration identity.
    pub target_hash: Option<ContentHash>,

    /// Determinism mode.
    pub determinism: DeterminismMode,

    /// Optional deterministic/random seed.
    pub seed: Option<u64>,

    /// Invocation start timestamp.
    pub started_at: ProvenanceTimestamp,

    /// Invocation completion timestamp.
    pub completed_at: Option<ProvenanceTimestamp>,

    /// Input IR content identity.
    pub input_hash: Option<ContentHash>,

    /// Output IR content identity.
    pub output_hash: Option<ContentHash>,
}

impl InvocationMetadata {
    /// Current provenance schema version.
    pub const CURRENT_SCHEMA_VERSION: u32 = 1;

    /// Creates invocation metadata.
    pub fn new(
        provenance_id: ProvenanceId,
        optimizer: OptimizerIdentity,
    ) -> ProvenanceResult<Self> {
        Ok(Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            provenance_id,
            optimizer,
            profile: None,
            configuration_hash: None,
            target: None,
            target_hash: None,
            determinism: DeterminismMode::default(),
            seed: None,
            started_at: ProvenanceTimestamp::now()?,
            completed_at: None,
            input_hash: None,
            output_hash: None,
        })
    }
}

// =============================================================================
// Final status
// =============================================================================

/// Overall optimization provenance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProvenanceStatus {
    /// Invocation is currently running.
    Running,

    /// Optimization completed successfully.
    Completed,

    /// Optimization completed but made no changes.
    Unchanged,

    /// Optimization completed partially.
    PartiallyCompleted,

    /// Optimization stopped at a configured limit.
    LimitReached,

    /// Optimization was cancelled.
    Cancelled,

    /// Optimization failed.
    Failed,
}

impl Default for ProvenanceStatus {
    fn default() -> Self {
        Self::Running
    }
}

// =============================================================================
// Counters
// =============================================================================

/// Lightweight counters for provenance collection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceCounters {
    /// Number of passes recorded.
    pub passes: u128,

    /// Number of rewrites recorded.
    pub rewrites: u128,

    /// Number of analyses recorded.
    pub analyses: u128,

    /// Number of verification records.
    pub verifications: u128,

    /// Number of pipeline stages.
    pub pipeline_stages: u128,

    /// Number of generic events.
    pub events: u128,

    /// Number of retained bytes in optional metadata estimates.
    pub metadata_bytes: u128,
}

impl ProvenanceCounters {
    fn increment_passes(&mut self) {
        self.passes = self.passes.saturating_add(1);
    }

    fn increment_rewrites(&mut self) {
        self.rewrites = self.rewrites.saturating_add(1);
    }

    fn increment_analyses(&mut self) {
        self.analyses = self.analyses.saturating_add(1);
    }

    fn increment_verifications(&mut self) {
        self.verifications = self.verifications.saturating_add(1);
    }

    fn increment_pipeline_stages(&mut self) {
        self.pipeline_stages =
            self.pipeline_stages.saturating_add(1);
    }

    fn increment_events(&mut self) {
        self.events = self.events.saturating_add(1);
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Immutable-style serializable snapshot of optimization provenance.
///
/// The structure contains owned collections intentionally: the snapshot can be
/// detached from the live optimizer context and passed to `result.rs`,
/// serialization, diagnostics, benchmarking, or external reproducibility
/// tooling without borrowing the optimizer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationProvenanceSnapshot {
    /// Invocation metadata.
    pub metadata: InvocationMetadata,

    /// Overall status.
    pub status: ProvenanceStatus,

    /// Collection mode used to produce this snapshot.
    pub mode: ProvenanceMode,

    /// Active limits.
    pub limits: ProvenanceLimits,

    /// Aggregate counters.
    pub counters: ProvenanceCounters,

    /// Pass records.
    pub passes: Vec<PassProvenance>,

    /// Rewrite records.
    pub rewrites: Vec<RuleApplication>,

    /// Analysis records.
    pub analyses: Vec<AnalysisProvenance>,

    /// Verification records.
    pub verifications: Vec<VerificationProvenance>,

    /// Pipeline-stage records.
    pub pipeline_stages: Vec<PipelineStageProvenance>,

    /// Generic chronological events.
    pub events: Vec<ProvenanceEvent>,

    /// Whether any collection limit was reached.
    pub truncated: bool,
}

impl OptimizationProvenanceSnapshot {
    /// Returns the number of recorded passes.
    #[must_use]
    pub fn pass_count(&self) -> usize {
        self.passes.len()
    }

    /// Returns the number of recorded rewrites.
    #[must_use]
    pub fn rewrite_count(&self) -> usize {
        self.rewrites.len()
    }

    /// Returns true if provenance was truncated.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }
}

// =============================================================================
// Live provenance collector
// =============================================================================

/// Mutable provenance collector used during one optimization invocation.
#[derive(Debug, Clone)]
pub struct OptimizationProvenance {
    metadata: InvocationMetadata,
    status: ProvenanceStatus,
    mode: ProvenanceMode,
    limits: ProvenanceLimits,
    counters: ProvenanceCounters,

    passes: Vec<PassProvenance>,
    rewrites: Vec<RuleApplication>,
    analyses: Vec<AnalysisProvenance>,
    verifications: Vec<VerificationProvenance>,
    pipeline_stages: Vec<PipelineStageProvenance>,
    events: Vec<ProvenanceEvent>,

    next_pass_handle: u64,
    next_event_sequence: u64,

    active_passes: Vec<u64>,

    truncated: bool,
}

impl OptimizationProvenance {
    /// Creates a provenance collector.
    pub fn new(
        mode: ProvenanceMode,
        limits: ProvenanceLimits,
    ) -> ProvenanceResult<Self> {
        let provenance_id = Self::generate_local_id()?;

        let optimizer = OptimizerIdentity::new(
            "unknown",
            "unknown",
        )?;

        let metadata =
            InvocationMetadata::new(provenance_id, optimizer)?;

        let mut result = Self {
            metadata,
            status: ProvenanceStatus::Running,
            mode,
            limits,
            counters: ProvenanceCounters::default(),
            passes: Vec::new(),
            rewrites: Vec::new(),
            analyses: Vec::new(),
            verifications: Vec::new(),
            pipeline_stages: Vec::new(),
            events: Vec::new(),
            next_pass_handle: 0,
            next_event_sequence: 0,
            active_passes: Vec::new(),
            truncated: false,
        };

        result.record_event(
            ProvenanceEventKind::InvocationStarted,
            None,
            None,
            None,
        )?;

        Ok(result)
    }

    /// Creates a provenance collector with a caller-supplied invocation ID.
    pub fn with_id(
        provenance_id: ProvenanceId,
        optimizer: OptimizerIdentity,
        mode: ProvenanceMode,
        limits: ProvenanceLimits,
    ) -> ProvenanceResult<Self> {
        let metadata =
            InvocationMetadata::new(provenance_id, optimizer)?;

        Ok(Self {
            metadata,
            status: ProvenanceStatus::Running,
            mode,
            limits,
            counters: ProvenanceCounters::default(),
            passes: Vec::new(),
            rewrites: Vec::new(),
            analyses: Vec::new(),
            verifications: Vec::new(),
            pipeline_stages: Vec::new(),
            events: Vec::new(),
            next_pass_handle: 0,
            next_event_sequence: 0,
            active_passes: Vec::new(),
            truncated: false,
        })
    }

    /// Returns the invocation metadata.
    #[must_use]
    pub fn metadata(&self) -> &InvocationMetadata {
        &self.metadata
    }

    /// Returns the current provenance status.
    #[must_use]
    pub const fn status(&self) -> ProvenanceStatus {
        self.status
    }

    /// Returns the provenance mode.
    #[must_use]
    pub const fn mode(&self) -> ProvenanceMode {
        self.mode
    }

    /// Returns the configured limits.
    #[must_use]
    pub const fn limits(&self) -> ProvenanceLimits {
        self.limits
    }

    /// Returns aggregate counters.
    #[must_use]
    pub const fn counters(&self) -> ProvenanceCounters {
        self.counters
    }

    /// Returns whether collection has been truncated.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Sets the optimizer identity.
    pub fn set_optimizer(
        &mut self,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_optimizer")?;

        let mut identity =
            OptimizerIdentity::new(name, version)?;

        if let Some(revision) =
            self.metadata.optimizer.source_revision.clone()
        {
            identity =
                identity.with_source_revision(revision)?;
        }

        identity.source_hash =
            self.metadata.optimizer.source_hash.clone();

        self.metadata.optimizer = identity;
        Ok(())
    }

    /// Sets the optimizer source revision.
    pub fn set_source_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_source_revision")?;

        let revision = revision.into();

        if revision.trim().is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "source_revision",
            });
        }

        self.metadata.optimizer.source_revision =
            Some(revision);

        Ok(())
    }

    /// Sets the optimizer source hash.
    pub fn set_source_hash(
        &mut self,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_source_hash")?;

        self.metadata.optimizer.source_hash = Some(hash);

        Ok(())
    }

    /// Sets the optimization profile identifier.
    pub fn set_profile(
        &mut self,
        profile: impl Into<String>,
    ) -> ProvenanceResult<()> {
        self.set_optional_string(
            "profile",
            profile.into(),
            |metadata, value| {
                metadata.profile = Some(value);
            },
        )
    }

    /// Sets the target identifier.
    pub fn set_target(
        &mut self,
        target: impl Into<String>,
    ) -> ProvenanceResult<()> {
        self.set_optional_string(
            "target",
            target.into(),
            |metadata, value| {
                metadata.target = Some(value);
            },
        )
    }

    /// Sets the optimization configuration hash.
    pub fn set_configuration_hash(
        &mut self,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_configuration_hash")?;

        self.metadata.configuration_hash = Some(hash);

        Ok(())
    }

    /// Sets the target configuration hash.
    pub fn set_target_hash(
        &mut self,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_target_hash")?;

        self.metadata.target_hash = Some(hash);

        Ok(())
    }

    /// Sets the input IR hash.
    pub fn set_input_hash(
        &mut self,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_input_hash")?;

        self.metadata.input_hash = Some(hash);

        Ok(())
    }

    /// Sets the output IR hash.
    pub fn set_output_hash(
        &mut self,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_output_hash")?;

        self.metadata.output_hash = Some(hash);

        Ok(())
    }

    /// Sets the determinism mode.
    pub fn set_determinism(
        &mut self,
        mode: DeterminismMode,
        seed: Option<u64>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_determinism")?;

        if matches!(mode, DeterminismMode::Seeded)
            && seed.is_none()
        {
            return Err(ProvenanceError::InvalidState {
                reason:
                    "seeded determinism requires an explicit seed",
            });
        }

        self.metadata.determinism = mode;
        self.metadata.seed = seed;

        Ok(())
    }

    /// Begins a pass.
    pub fn begin_pass(
        &mut self,
        pass_id: impl Into<String>,
        phase: impl Into<String>,
        ordinal: u64,
    ) -> ProvenanceResult<u64> {
        self.ensure_enabled("begin_pass")?;

        let pass_id = pass_id.into();
        let phase = phase.into();

        validate_field(
            "pass_id",
            &pass_id,
            self.limits.max_field_bytes,
        )?;

        validate_field(
            "phase",
            &phase,
            self.limits.max_field_bytes,
        )?;

        if self
            .active_passes
            .iter()
            .copied()
            .any(|handle| {
                self.passes
                    .iter()
                    .find(|pass| pass.handle == handle)
                    .map(|pass| pass.pass_id == pass_id)
                    .unwrap_or(false)
            })
        {
            return Err(
                ProvenanceError::DuplicateActivePass {
                    pass_id,
                },
            );
        }

        self.ensure_pass_capacity()?;

        let handle = self
            .next_pass_handle
            .checked_add(1)
            .ok_or(ProvenanceError::PassHandleOverflow)?;

        self.next_pass_handle = handle;

        let timestamp = ProvenanceTimestamp::now()?;

        self.passes.push(PassProvenance {
            handle,
            pass_id: pass_id.clone(),
            phase,
            ordinal,
            started_at: timestamp,
            ended_at: None,
            outcome: None,
            rewrites: 0,
            analyses: 0,
            verification_operations: 0,
            configuration_hash: None,
        });

        self.active_passes.push(handle);

        self.counters.increment_passes();

        self.record_event(
            ProvenanceEventKind::PassStarted,
            Some(pass_id),
            Some(handle),
            None,
        )?;

        Ok(handle)
    }

    /// Attaches a pass-specific configuration hash.
    pub fn set_pass_configuration_hash(
        &mut self,
        handle: u64,
        hash: ContentHash,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("set_pass_configuration_hash")?;

        let pass = self
            .passes
            .iter_mut()
            .find(|pass| pass.handle == handle)
            .ok_or(ProvenanceError::UnknownPass { handle })?;

        if pass.ended() {
            return Err(ProvenanceError::PassAlreadyEnded {
                handle,
            });
        }

        pass.configuration_hash = Some(hash);

        Ok(())
    }

    /// Records completion of a pass.
    pub fn end_pass(
        &mut self,
        handle: u64,
        outcome: PassProvenanceOutcome,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("end_pass")?;

        let position = self
            .passes
            .iter()
            .position(|pass| pass.handle == handle)
            .ok_or(ProvenanceError::UnknownPass { handle })?;

        if self.passes[position].ended() {
            return Err(ProvenanceError::PassAlreadyEnded {
                handle,
            });
        }

        let ended_at = ProvenanceTimestamp::now()?;

        self.passes[position].ended_at = Some(ended_at);
        self.passes[position].outcome = Some(outcome);

        self.active_passes
            .retain(|active| *active != handle);

        let pass_id = self.passes[position].pass_id.clone();

        self.record_event(
            ProvenanceEventKind::PassCompleted,
            Some(pass_id),
            Some(handle),
            None,
        )?;

        Ok(())
    }

    /// Records a rewrite rule application.
    pub fn record_rule(
        &mut self,
        rule: RuleApplication,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("record_rule")?;

        self.ensure_rewrite_capacity()?;

        if !self.passes.iter().any(|pass| {
            pass.handle == rule.pass_handle
        }) {
            return Err(ProvenanceError::UnknownPass {
                handle: rule.pass_handle,
            });
        }

        self.rewrites.push(rule.clone());
        self.counters.increment_rewrites();

        if let Some(pass) =
            self.passes.iter_mut().find(|pass| {
                pass.handle == rule.pass_handle
            })
        {
            pass.rewrites =
                pass.rewrites.saturating_add(1);
        }

        self.record_event(
            ProvenanceEventKind::RewriteApplied,
            Some(rule.rule_id),
            Some(rule.pass_handle),
            None,
        )?;

        Ok(())
    }

    /// Records an analysis request.
    pub fn record_analysis(
        &mut self,
        analysis: AnalysisProvenance,
        pass_handle: Option<u64>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("record_analysis")?;

        self.ensure_analysis_capacity()?;

        if let Some(handle) = pass_handle {
            if !self.passes.iter().any(|pass| {
                pass.handle == handle
            }) {
                return Err(ProvenanceError::UnknownPass {
                    handle,
                });
            }

            if let Some(pass) =
                self.passes.iter_mut().find(|pass| {
                    pass.handle == handle
                })
            {
                pass.analyses =
                    pass.analyses.saturating_add(1);
            }
        }

        let analysis_id = analysis.analysis_id.clone();

        self.analyses.push(analysis);
        self.counters.increment_analyses();

        self.record_event(
            ProvenanceEventKind::AnalysisRequested,
            Some(analysis_id),
            pass_handle,
            None,
        )?;

        Ok(())
    }

    /// Records a verification event.
    pub fn record_verification(
        &mut self,
        verification: VerificationProvenance,
        pass_handle: Option<u64>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("record_verification")?;

        self.ensure_verification_capacity()?;

        if let Some(handle) = pass_handle {
            if !self.passes.iter().any(|pass| {
                pass.handle == handle
            }) {
                return Err(ProvenanceError::UnknownPass {
                    handle,
                });
            }

            if let Some(pass) =
                self.passes.iter_mut().find(|pass| {
                    pass.handle == handle
                })
            {
                pass.verification_operations =
                    pass.verification_operations.saturating_add(
                        verification.operations_examined,
                    );
            }
        }

        let method =
            verification.method.as_str().to_owned();

        self.verifications.push(verification);
        self.counters.increment_verifications();

        self.record_event(
            ProvenanceEventKind::VerificationExecuted,
            Some(method),
            pass_handle,
            None,
        )?;

        Ok(())
    }

    /// Records a pipeline stage.
    pub fn record_pipeline_stage(
        &mut self,
        stage: PipelineStageProvenance,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("record_pipeline_stage")?;

        self.ensure_pipeline_capacity()?;

        let stage_id = stage.stage_id.clone();

        self.pipeline_stages.push(stage);
        self.counters.increment_pipeline_stages();

        self.record_event(
            ProvenanceEventKind::PipelineStage,
            Some(stage_id),
            None,
            None,
        )?;

        Ok(())
    }

    /// Records a generic provenance event.
    pub fn record_event(
        &mut self,
        kind: ProvenanceEventKind,
        component: Option<String>,
        pass_handle: Option<u64>,
        message: Option<String>,
    ) -> ProvenanceResult<()> {
        if !self.mode.enabled() {
            return Ok(());
        }

        self.ensure_event_capacity()?;

        let sequence = self
            .next_event_sequence
            .checked_add(1)
            .ok_or(ProvenanceError::SequenceOverflow)?;

        self.next_event_sequence = sequence;

        if let Some(value) = component.as_ref() {
            validate_field(
                "event_component",
                value,
                self.limits.max_field_bytes,
            )?;
        }

        if let Some(value) = message.as_ref() {
            validate_field(
                "event_message",
                value,
                self.limits.max_field_bytes,
            )?;
        }

        let timestamp = ProvenanceTimestamp::now()?;

        self.events.push(ProvenanceEvent {
            sequence,
            kind,
            timestamp,
            component,
            pass_handle,
            message,
        });

        self.counters.increment_events();

        Ok(())
    }

    /// Marks the invocation as successfully completed.
    pub fn complete(
        &mut self,
        status: ProvenanceStatus,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("complete")?;

        if matches!(status, ProvenanceStatus::Running) {
            return Err(ProvenanceError::InvalidState {
                reason:
                    "a completed invocation cannot use Running status",
            });
        }

        self.status = status;
        self.metadata.completed_at =
            Some(ProvenanceTimestamp::now()?);

        self.record_event(
            ProvenanceEventKind::InvocationCompleted,
            None,
            None,
            None,
        )?;

        Ok(())
    }

    /// Marks the invocation as cancelled.
    pub fn cancel(&mut self) -> ProvenanceResult<()> {
        self.ensure_enabled("cancel")?;

        self.status = ProvenanceStatus::Cancelled;
        self.metadata.completed_at =
            Some(ProvenanceTimestamp::now()?);

        self.record_event(
            ProvenanceEventKind::Cancelled,
            None,
            None,
            None,
        )?;

        Ok(())
    }

    /// Marks the invocation as limit-bound.
    pub fn mark_limit_reached(
        &mut self,
        component: Option<String>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("mark_limit_reached")?;

        self.status = ProvenanceStatus::LimitReached;

        self.record_event(
            ProvenanceEventKind::LimitReached,
            component,
            None,
            None,
        )?;

        Ok(())
    }

    /// Marks the invocation as failed.
    pub fn fail(
        &mut self,
        message: Option<String>,
    ) -> ProvenanceResult<()> {
        self.ensure_enabled("fail")?;

        self.status = ProvenanceStatus::Failed;
        self.metadata.completed_at =
            Some(ProvenanceTimestamp::now()?);

        self.record_event(
            ProvenanceEventKind::Custom,
            None,
            None,
            message,
        )?;

        Ok(())
    }

    /// Returns an owned snapshot suitable for `OptimizationResult`.
    #[must_use]
    pub fn snapshot(&self) -> OptimizationProvenanceSnapshot {
        OptimizationProvenanceSnapshot {
            metadata: self.metadata.clone(),
            status: self.status,
            mode: self.mode,
            limits: self.limits,
            counters: self.counters,
            passes: self.passes.clone(),
            rewrites: self.rewrites.clone(),
            analyses: self.analyses.clone(),
            verifications: self.verifications.clone(),
            pipeline_stages: self.pipeline_stages.clone(),
            events: self.events.clone(),
            truncated: self.truncated,
        }
    }

    /// Returns the number of currently active passes.
    #[must_use]
    pub fn active_pass_count(&self) -> usize {
        self.active_passes.len()
    }

    /// Returns true when a pass handle is currently active.
    #[must_use]
    pub fn is_pass_active(&self, handle: u64) -> bool {
        self.active_passes
            .iter()
            .any(|active| *active == handle)
    }

    /// Returns a pass record by handle.
    #[must_use]
    pub fn pass(&self, handle: u64) -> Option<&PassProvenance> {
        self.passes
            .iter()
            .find(|pass| pass.handle == handle)
    }

    // -------------------------------------------------------------------------
    // Internal capacity helpers
    // -------------------------------------------------------------------------

    fn ensure_enabled(
        &self,
        operation: &'static str,
    ) -> ProvenanceResult<()> {
        if self.mode.enabled() {
            Ok(())
        } else {
            Err(ProvenanceError::Disabled { operation })
        }
    }

    fn ensure_pass_capacity(&self) -> ProvenanceResult<()> {
        if self.passes.len() as u64
            >= self.limits.max_passes
        {
            return Err(
                ProvenanceError::PassLimitExceeded {
                    current: self.passes.len() as u64,
                    maximum: self.limits.max_passes,
                },
            );
        }

        Ok(())
    }

    fn ensure_rewrite_capacity(
        &self,
    ) -> ProvenanceResult<()> {
        if self.rewrites.len() as u64
            >= self.limits.max_rewrites
        {
            return Err(
                ProvenanceError::RewriteLimitExceeded {
                    current: self.rewrites.len() as u64,
                    maximum: self.limits.max_rewrites,
                },
            );
        }

        Ok(())
    }

    fn ensure_verification_capacity(
        &self,
    ) -> ProvenanceResult<()> {
        if self.verifications.len() as u64
            >= self.limits.max_verifications
        {
            return Err(
                ProvenanceError::VerificationLimitExceeded {
                    current: self.verifications.len() as u64,
                    maximum: self.limits.max_verifications,
                },
            );
        }

        Ok(())
    }

    fn ensure_analysis_capacity(
        &self,
    ) -> ProvenanceResult<()> {
        if self.analyses.len() as u64
            >= self.limits.max_analyses
        {
            return Err(
                ProvenanceError::EventLimitExceeded {
                    category:
                        ProvenanceEventKind::AnalysisRequested,
                    current: self.analyses.len() as u64,
                    maximum: self.limits.max_analyses,
                },
            );
        }

        Ok(())
    }

    fn ensure_pipeline_capacity(
        &self,
    ) -> ProvenanceResult<()> {
        if self.pipeline_stages.len() as u64
            >= self.limits.max_pipeline_stages
        {
            return Err(
                ProvenanceError::EventLimitExceeded {
                    category:
                        ProvenanceEventKind::PipelineStage,
                    current: self.pipeline_stages.len() as u64,
                    maximum: self.limits.max_pipeline_stages,
                },
            );
        }

        Ok(())
    }

    fn ensure_event_capacity(
        &self,
    ) -> ProvenanceResult<()> {
        if self.events.len() as u64
            >= self.limits.max_events
        {
            return Err(
                ProvenanceError::EventLimitExceeded {
                    category: ProvenanceEventKind::Custom,
                    current: self.events.len() as u64,
                    maximum: self.limits.max_events,
                },
            );
        }

        Ok(())
    }

    fn set_optional_string<F>(
        &mut self,
        field: &'static str,
        value: String,
        setter: F,
    ) -> ProvenanceResult<()>
    where
        F: FnOnce(&mut InvocationMetadata, String),
    {
        self.ensure_enabled(field)?;

        validate_field(
            field,
            &value,
            self.limits.max_field_bytes,
        )?;

        setter(&mut self.metadata, value);

        Ok(())
    }

    fn generate_local_id() -> ProvenanceResult<ProvenanceId> {
        let timestamp = ProvenanceTimestamp::now()?;

        ProvenanceId::new(format!(
            "optimization-{}",
            timestamp.unix_nanos()
        ))
    }
}

// =============================================================================
// Utility functions
// =============================================================================

/// Validates the size and non-empty invariant for a provenance text field.
fn validate_field(
    field: &'static str,
    value: &str,
    maximum_bytes: u64,
) -> ProvenanceResult<()> {
    if value.trim().is_empty() {
        return Err(ProvenanceError::EmptyField { field });
    }

    let actual =
        u64::try_from(value.len()).unwrap_or(u64::MAX);

    if actual > maximum_bytes {
        return Err(ProvenanceError::FieldTooLarge {
            field,
            actual_bytes: actual,
            maximum_bytes,
        });
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
    fn sha256_hash_is_validated() {
        let hash = ContentHash::sha256(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .expect("valid SHA-256 digest");

        assert_eq!(hash.algorithm(), HashAlgorithm::Sha256);
        assert_eq!(hash.as_str().len(), 64);
    }

    #[test]
    fn invalid_hash_length_is_rejected() {
        let result = ContentHash::sha256("abcd");

        assert!(matches!(
            result,
            Err(ProvenanceError::InvalidDigestLength {
                algorithm: HashAlgorithm::Sha256,
                ..
            })
        ));
    }

    #[test]
    fn invalid_hash_encoding_is_rejected() {
        let result = ContentHash::sha256(
            "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",
        );

        assert!(matches!(
            result,
            Err(ProvenanceError::InvalidDigestEncoding {
                algorithm: HashAlgorithm::Sha256,
            })
        ));
    }

    #[test]
    fn timestamp_is_non_negative() {
        let timestamp =
            ProvenanceTimestamp::now().expect("system time");

        assert!(timestamp.unix_nanos() > 0);
    }

    #[test]
    fn provenance_can_record_a_pass() {
        let mut provenance =
            OptimizationProvenance::new(
                ProvenanceMode::Bounded,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        provenance
            .set_optimizer(
                "zamani-quantum-optimizer",
                "1.0.0",
            )
            .expect("optimizer identity");

        let handle = provenance
            .begin_pass(
                "local.cancellation",
                "local",
                0,
            )
            .expect("begin pass");

        provenance
            .record_rule(
                RuleApplication::new(
                    "identity.self_inverse",
                    handle,
                    2,
                    0,
                )
                .expect("rule"),
            )
            .expect("record rule");

        provenance
            .end_pass(
                handle,
                PassProvenanceOutcome::Changed,
            )
            .expect("end pass");

        assert_eq!(provenance.pass_count(), 1);
        assert_eq!(provenance.counters().rewrites, 1);
        assert!(!provenance.is_pass_active(handle));
    }

    #[test]
    fn provenance_records_analysis_and_verification() {
        let mut provenance =
            OptimizationProvenance::new(
                ProvenanceMode::Bounded,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        let handle = provenance
            .begin_pass(
                "simplify",
                "local",
                0,
            )
            .expect("pass");

        provenance
            .record_analysis(
                AnalysisProvenance::new(
                    "dependency",
                    1,
                    false,
                    100,
                )
                .expect("analysis"),
                Some(handle),
            )
            .expect("analysis record");

        provenance
            .record_verification(
                VerificationProvenance::new(
                    VerificationMethod::new(
                        "exact_unitary",
                    )
                    .expect("method"),
                    VerificationProvenanceStatus::Passed,
                    100,
                    4,
                    0,
                ),
                Some(handle),
            )
            .expect("verification");

        provenance
            .end_pass(
                handle,
                PassProvenanceOutcome::Unchanged,
            )
            .expect("end");

        assert_eq!(provenance.counters().analyses, 1);
        assert_eq!(
            provenance.counters().verifications,
            1
        );
    }

    #[test]
    fn provenance_snapshot_is_serializable() {
        let provenance =
            OptimizationProvenance::new(
                ProvenanceMode::Bounded,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        let snapshot = provenance.snapshot();

        let encoded =
            serde_json::to_string(&snapshot)
                .expect("JSON serialization");

        assert!(!encoded.is_empty());

        let decoded:
            OptimizationProvenanceSnapshot =
            serde_json::from_str(&encoded)
                .expect("JSON deserialization");

        assert_eq!(
            decoded.metadata.schema_version,
            InvocationMetadata::CURRENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn deterministic_mode_requires_seed() {
        let optimizer =
            OptimizerIdentity::new(
                "zamani",
                "1.0",
            )
            .expect("identity");

        let id =
            ProvenanceId::new("test").expect("id");

        let mut provenance =
            OptimizationProvenance::with_id(
                id,
                optimizer,
                ProvenanceMode::Bounded,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        let result = provenance.set_determinism(
            DeterminismMode::Seeded,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn disabled_mode_does_not_retain_records() {
        let optimizer =
            OptimizerIdentity::new(
                "zamani",
                "1.0",
            )
            .expect("identity");

        let id =
            ProvenanceId::new("disabled-test")
                .expect("id");

        let mut provenance =
            OptimizationProvenance::with_id(
                id,
                optimizer,
                ProvenanceMode::Disabled,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        assert!(provenance
            .begin_pass("x", "local", 0)
            .is_err());

        assert_eq!(provenance.pass_count(), 0);
    }

    #[test]
    fn pass_limit_is_enforced() {
        let limits = ProvenanceLimits {
            max_passes: 1,
            ..ProvenanceLimits::compact()
        };

        let mut provenance =
            OptimizationProvenance::new(
                ProvenanceMode::Bounded,
                limits,
            )
            .expect("provenance");

        provenance
            .begin_pass("first", "local", 0)
            .expect("first pass");

        let second =
            provenance.begin_pass(
                "second",
                "local",
                1,
            );

        assert!(matches!(
            second,
            Err(ProvenanceError::PassLimitExceeded {
                current: 1,
                maximum: 1
            })
        ));
    }

    #[test]
    fn rewrite_requires_known_pass() {
        let mut provenance =
            OptimizationProvenance::new(
                ProvenanceMode::Bounded,
                ProvenanceLimits::compact(),
            )
            .expect("provenance");

        let rule = RuleApplication::new(
            "test.rule",
            999,
            2,
            1,
        )
        .expect("rule");

        assert!(matches!(
            provenance.record_rule(rule),
            Err(ProvenanceError::UnknownPass {
                handle: 999
            })
        ));
    }

    #[test]
    fn hash_display_is_canonical() {
        let hash = ContentHash::sha256(
            "ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789",
        )
        .expect("hash");

        assert_eq!(
            hash.to_string(),
            "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789"
        );
    }
}