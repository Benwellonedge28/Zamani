//! Zamani Quantum Noise (ZQN) — Core Provenance
//!
//! Production-grade provenance and lineage for ZQN artifacts.
//!
//! # Ownership
//!
//! This module owns provenance describing the origin, derivation, identity,
//! reproducibility context, and observational lineage of ZQN artifacts.
//!
//! In particular it records lineage for:
//!
//! - noise models;
//! - noise specifications;
//! - quantum channels;
//! - fault models;
//! - calibration references;
//! - characterization results;
//! - noise observations;
//! - target associations;
//! - deterministic generation/execution context;
//! - derived noise artifacts;
//! - ZQN transformations.
//!
//! # Non-ownership
//!
//! This module does NOT own:
//!
//! - canonical Quantum IR semantics;
//! - source-language parsing;
//! - quantum gates;
//! - qubit identity;
//! - hardware topology;
//! - hardware credentials;
//! - calibration payloads;
//! - quantum state;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC algorithms;
//! - benchmarking algorithms;
//! - cryptographic signing;
//! - cryptographic key management;
//! - content hashing implementation.
//!
//! The canonical Quantum IR remains the semantic boundary.
//!
//! The canonical logical/physical qubit identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN does not define replacement qubit identifiers.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                 +------------+-------------+
//!                 |                          |
//!                 v                          v
//!          semantic program                ZQN
//!                                            |
//!                  +-------------------------+----------------------+
//!                  |                         |                      |
//!                  v                         v                      v
//!             noise model              calibration          characterization
//!                  |                         |                      |
//!                  +-------------------------+----------------------+
//!                                            |
//!                                            v
//!                                      noise realization
//!                                            |
//!                                  +---------+---------+
//!                                  |                   |
//!                                  v                   v
//!                              simulator            hardware
//! ```
//!
//! Provenance describes these relationships but never performs the work.
//!
//! # IR provenance versus ZQN provenance
//!
//! `quantum::ir::metadata::provenance` answers:
//!
//! > How did this semantic IR artifact come into existence?
//!
//! This module answers:
//!
//! > How did this ZQN noise artifact come into existence, from which model,
//! > calibration, characterization, deterministic context, and target context?
//!
//! The two systems may reference one another, but neither duplicates the
//! other's ownership.
//!
//! # Write once, scale everywhere
//!
//! ZQN provenance contains no architectural maximum for:
//!
//! - qubits;
//! - operations;
//! - noise events;
//! - transformations;
//! - calibration references;
//! - characterization references;
//! - artifacts;
//! - metadata entries;
//! - correlation domains;
//! - execution records;
//! - target resources.
//!
//! All collections are dynamically sized.
//!
//! A concrete provenance object remains finite because every concrete process
//! has finite available memory, address space, storage, and execution time.
//!
//! The architecture itself does not impose a quantum-machine-size ceiling.
//!
//! # Determinism
//!
//! Provenance is divided conceptually into two classes:
//!
//! ```text
//! deterministic provenance
//!     |
//!     +-- semantic artifact identity
//!     +-- model identity
//!     +-- transformation lineage
//!     +-- configuration identity
//!     +-- deterministic seed
//!     +-- IR/program identity
//!     +-- calibration identity
//!
//! observational provenance
//!     |
//!     +-- wall-clock timestamps
//!     +-- execution/job identifiers
//!     +-- runtime observations
//!     +-- deployment observations
//! ```
//!
//! Observational fields MUST NOT be treated as semantic identity unless an
//! explicitly versioned higher-level contract chooses to do so.
//!
//! This distinction is essential for reproducible quantum experiments.
//!
//! # Security
//!
//! Provenance is metadata, not authentication.
//!
//! It MUST NOT contain:
//!
//! - passwords;
//! - API keys;
//! - access tokens;
//! - private keys;
//! - signing keys;
//! - credentials;
//! - raw secrets;
//! - confidential calibration payloads.
//!
//! References to external artifacts are permitted.
//!
//! A digest/reference proves only that an artifact identity was recorded. It
//! does not prove authorship, authorization, authenticity, or trust.
//!
//! Digital signatures belong to a separate security subsystem.
//!
//! # Content identity
//!
//! This module deliberately does not implement hashing.
//!
//! When content identity is required, callers should use the canonical hashing
//! subsystem:
//!
//! ```text
//! quantum::ir::hashing
//! ```
//!
//! ZQN provenance stores the resulting canonical digest/reference.
//!
//! This prevents ZQN from creating a second cryptographic hashing contract.
//!
//! # Canonical ordering
//!
//! Provenance collections that represent sets or indexes use deterministic
//! ordering.
//!
//! `BTreeMap` is used for metadata so serialized or inspected provenance does
//! not depend on hash-map iteration order.
//!
//! Ordered lineage records preserve explicit sequence numbers.
//!
//! # Resource safety
//!
//! This module does not impose global semantic limits.
//!
//! Callers may enforce limits externally through the ZQN resource-policy
//! subsystem.
//!
//! The optional `validate_field_size` helper is explicitly policy-oriented:
//! its caller supplies the limit.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This file is intentionally independent from future ZQN implementation
//! files.
//!
//! Downstream modules may consume this module from:
//!
//! ```text
//! zqn::noise
//! zqn::channel
//! zqn::fault
//! zqn::calibration
//! zqn::characterization
//! zqn::simulation
//! zqn::target
//! zqn::integration
//! zqn::io
//! ```
//!
//! Those modules must not redefine:
//!
//! - provenance IDs;
//! - provenance timestamps;
//! - artifact references;
//! - transformation lineage;
//! - ZQN provenance semantics.
//!
//! # Qubit integration
//!
//! Mapping records use the canonical IR types directly:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No `ZqnQubitId`, `NoiseQubitId`, or other competing identity is introduced.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. it compiles independently of future ZQN implementation files;
//! 2. canonical IR identity types are used;
//! 3. no duplicate qubit identity exists;
//! 4. no hashing implementation is duplicated;
//! 5. deterministic and observational provenance are distinguishable;
//! 6. lineage is deterministically ordered;
//! 7. mappings are internally consistent;
//! 8. future schema versions are rejected rather than guessed;
//! 9. no semantic machine-size limit exists;
//! 10. resource limits remain caller policy;
//! 11. no secrets are required or stored;
//! 12. no unsafe Rust exists;
//! 13. the public API does not require later modification merely because
//!     downstream ZQN modules are added.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::core::identity
//! quantum::ir::qubit
//! quantum::ir::hashing
//!          |
//!          v
//! zqn::core::provenance
//!          |
//!          +--> noise
//!          +--> channel
//!          +--> fault
//!          +--> calibration
//!          +--> characterization
//!          +--> simulation
//!          +--> target
//!          +--> integration
//!          +--> io
//! ```
//!
//! ZQN provenance must never depend on those downstream implementations.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::quantum::ir::core::identity::{
    CalibrationId,
    IrVersion,
    OperationId,
    ProgramId,
    ProvenanceId,
};

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Schema
// =============================================================================

/// Current ZQN provenance schema version.
///
/// This version is independent from:
///
/// - Zamani language version;
/// - Quantum IR version;
/// - ZQN implementation version;
/// - compiler version;
/// - hardware version;
/// - calibration version;
/// - serialization version.
///
/// A schema change that changes the meaning of persisted provenance MUST
/// increment this version through an explicit compatibility decision.
pub const ZQN_PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Maximum digest size currently accepted by this representation.
///
/// This is a representation property, not a machine-size limit.
pub const DIGEST_BYTES: usize = 32;

/// Number of hexadecimal characters required to represent [`DIGEST_BYTES`].
pub const DIGEST_HEX_BYTES: usize = DIGEST_BYTES * 2;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating ZQN provenance.
///
/// The error type is intentionally local to this foundational file so this
/// file does not depend on a future `zqn::core::error` implementation.
///
/// A future ZQN error façade may convert this type without changing this file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    /// A required field is empty.
    EmptyField {
        /// Stable field name.
        field: &'static str,
    },

    /// A field exceeds a caller-selected policy limit.
    FieldTooLarge {
        /// Stable field name.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Caller-selected maximum.
        maximum_bytes: usize,
    },

    /// A digest has an invalid textual or binary length.
    InvalidDigestLength {
        /// Actual length.
        actual: usize,

        /// Expected length.
        expected: usize,
    },

    /// A digest contains a non-hexadecimal character.
    InvalidDigestCharacter {
        /// Byte/character position.
        position: usize,
    },

    /// A lineage relationship violates a provenance invariant.
    InvalidRelationship {
        /// Human-readable diagnostic.
        message: String,
    },

    /// A schema version is newer than this implementation understands.
    UnsupportedSchemaVersion {
        /// Encountered schema version.
        found: u16,

        /// Highest supported version.
        supported: u16,
    },

    /// A timestamp contains an invalid nanosecond component.
    InvalidTimestamp {
        /// Seconds since Unix epoch.
        seconds: u64,

        /// Nanoseconds component.
        nanoseconds: u32,
    },

    /// System time could not be represented as Unix time.
    ClockBeforeUnixEpoch,
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "ZQN provenance field `{field}` must not be empty"
                )
            }

            Self::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "ZQN provenance field `{field}` is {actual_bytes} bytes; \
                     maximum policy size is {maximum_bytes}"
                )
            }

            Self::InvalidDigestLength { actual, expected } => {
                write!(
                    formatter,
                    "invalid provenance digest length {actual}; expected {expected}"
                )
            }

            Self::InvalidDigestCharacter { position } => {
                write!(
                    formatter,
                    "invalid hexadecimal digest character at position {position}"
                )
            }

            Self::InvalidRelationship { message } => {
                write!(
                    formatter,
                    "invalid ZQN provenance relationship: {message}"
                )
            }

            Self::UnsupportedSchemaVersion {
                found,
                supported,
            } => {
                write!(
                    formatter,
                    "unsupported ZQN provenance schema version {found}; \
                     implementation supports through {supported}"
                )
            }

            Self::InvalidTimestamp {
                seconds,
                nanoseconds,
            } => {
                write!(
                    formatter,
                    "invalid provenance timestamp {seconds}.{nanoseconds:09}"
                )
            }

            Self::ClockBeforeUnixEpoch => {
                formatter.write_str(
                    "system clock is before the Unix epoch",
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

// =============================================================================
// Digest
// =============================================================================

/// Fixed-width content digest stored by provenance.
///
/// The cryptographic hashing operation is deliberately outside this module.
/// This type stores the resulting digest and provides deterministic
/// representation/validation.
///
/// The current representation is 32 bytes, matching the repository's
/// SHA-256-oriented canonical hashing boundary.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceDigest {
    bytes: [u8; DIGEST_BYTES],
}

impl ProvenanceDigest {
    /// Creates a digest from its exact binary representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; DIGEST_BYTES]) -> Self {
        Self { bytes }
    }

    /// Returns the exact binary representation.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; DIGEST_BYTES] {
        &self.bytes
    }

    /// Copies the binary representation.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; DIGEST_BYTES] {
        self.bytes
    }

    /// Parses a hexadecimal digest.
    ///
    /// Uppercase and lowercase hexadecimal are both accepted.
    pub fn from_hex(value: &str) -> Result<Self, ProvenanceError> {
        if value.len() != DIGEST_HEX_BYTES {
            return Err(ProvenanceError::InvalidDigestLength {
                actual: value.len(),
                expected: DIGEST_HEX_BYTES,
            });
        }

        let bytes = value.as_bytes();
        let mut output = [0u8; DIGEST_BYTES];

        let mut index = 0usize;

        while index < DIGEST_BYTES {
            let high = hex_value(bytes[index * 2]).ok_or(
                ProvenanceError::InvalidDigestCharacter {
                    position: index * 2,
                },
            )?;

            let low = hex_value(bytes[index * 2 + 1]).ok_or(
                ProvenanceError::InvalidDigestCharacter {
                    position: index * 2 + 1,
                },
            )?;

            output[index] = (high << 4) | low;

            index += 1;
        }

        Ok(Self::from_bytes(output))
    }

    /// Returns lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(DIGEST_HEX_BYTES);

        for byte in self.bytes {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte & 0x0f));
        }

        output
    }
}

impl fmt::Debug for ProvenanceDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ProvenanceDigest")
            .field(&self.to_hex())
            .finish()
    }
}

impl fmt::Display for ProvenanceDigest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

fn hex_digit(value: u8) -> char {
    match value & 0x0f {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        _ => 'f',
    }
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
// Digest algorithm
// =============================================================================

/// Cryptographic digest algorithm identifier.
///
/// The implementation itself belongs to the canonical IR hashing subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DigestAlgorithm {
    /// SHA-256.
    Sha256,
}

impl DigestAlgorithm {
    /// Returns a stable schema identifier.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
        }
    }
}

impl fmt::Display for DigestAlgorithm {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Content reference
// =============================================================================

/// Content-addressed reference to an external or internal artifact.
///
/// This is a reference only. It does not contain the referenced payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentReference {
    algorithm: DigestAlgorithm,
    digest: ProvenanceDigest,
}

impl ContentReference {
    /// Creates a content reference.
    #[must_use]
    pub const fn new(
        algorithm: DigestAlgorithm,
        digest: ProvenanceDigest,
    ) -> Self {
        Self {
            algorithm,
            digest,
        }
    }

    /// Creates a SHA-256 content reference.
    #[must_use]
    pub const fn sha256(
        digest: ProvenanceDigest,
    ) -> Self {
        Self::new(
            DigestAlgorithm::Sha256,
            digest,
        )
    }

    /// Returns the digest algorithm.
    #[must_use]
    pub const fn algorithm(
        &self,
    ) -> DigestAlgorithm {
        self.algorithm
    }

    /// Returns the digest.
    #[must_use]
    pub const fn digest(
        &self,
    ) -> ProvenanceDigest {
        self.digest
    }
}

impl fmt::Display for ContentReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.algorithm,
            self.digest
        )
    }
}

// =============================================================================
// Artifact kind
// =============================================================================

/// Semantic kind of an artifact referenced by ZQN provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactKind {
    /// A complete noise model.
    NoiseModel,

    /// A declarative noise specification.
    NoiseSpecification,

    /// A quantum channel.
    Channel,

    /// A fault model.
    FaultModel,

    /// A calibration snapshot/reference.
    Calibration,

    /// A characterization experiment.
    CharacterizationExperiment,

    /// A characterization result.
    CharacterizationResult,

    /// A noise observation.
    Observation,

    /// A generated noise realization.
    NoiseRealization,

    /// A deterministic sampling configuration.
    SamplingConfiguration,

    /// A target capability/descriptor.
    TargetDescriptor,

    /// A simulation result.
    SimulationResult,

    /// A hardware execution result.
    ExecutionResult,

    /// A compiled artifact.
    CompiledArtifact,

    /// An IR artifact.
    IrArtifact,

    /// A generic transformation output.
    Transformation,

    /// A user-defined extension.
    Extension,
}

// =============================================================================
// Artifact reference
// =============================================================================

/// Stable reference to a ZQN artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactReference {
    kind: ArtifactKind,
    name: String,
    content: Option<ContentReference>,
}

impl ArtifactReference {
    /// Creates an artifact reference without a content digest.
    pub fn new(
        kind: ArtifactKind,
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "artifact.name",
            &name,
        )?;

        Ok(Self {
            kind,
            name,
            content: None,
        })
    }

    /// Creates an artifact reference with content identity.
    pub fn with_content(
        kind: ArtifactKind,
        name: impl Into<String>,
        content: ContentReference,
    ) -> Result<Self, ProvenanceError> {
        let mut reference =
            Self::new(kind, name)?;

        reference.content = Some(content);

        Ok(reference)
    }

    /// Returns artifact kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> ArtifactKind {
        self.kind
    }

    /// Returns artifact name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns content identity.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Adds or replaces content identity.
    pub fn set_content(
        &mut self,
        content: ContentReference,
    ) {
        self.content = Some(content);
    }
}

// =============================================================================
// Source reference
// =============================================================================

/// Reference to the source or upstream artifact from which a ZQN artifact was
/// derived.
///
/// The source payload itself is not stored here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceReference {
    name: String,
    content: Option<ContentReference>,
    language: Option<String>,
    language_version: Option<String>,
    revision: Option<String>,
    location: Option<String>,
}

impl SourceReference {
    /// Creates a source reference.
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "source.name",
            &name,
        )?;

        Ok(Self {
            name,
            content: None,
            language: None,
            language_version: None,
            revision: None,
            location: None,
        })
    }

    /// Associates source content identity.
    #[must_use]
    pub fn with_content(
        mut self,
        content: ContentReference,
    ) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets source language.
    pub fn set_language(
        &mut self,
        language: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let language = language.into();

        validate_non_empty(
            "source.language",
            &language,
        )?;

        self.language = Some(language);

        Ok(())
    }

    /// Sets source-language version.
    pub fn set_language_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();

        validate_non_empty(
            "source.language_version",
            &version,
        )?;

        self.language_version = Some(version);

        Ok(())
    }

    /// Sets source revision.
    pub fn set_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let revision = revision.into();

        validate_non_empty(
            "source.revision",
            &revision,
        )?;

        self.revision = Some(revision);

        Ok(())
    }

    /// Sets source location.
    ///
    /// This is descriptive metadata only.
    #[must_use]
    pub fn with_location(
        mut self,
        location: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let location = location.into();

        validate_non_empty(
            "source.location",
            &location,
        )?;

        self.location = Some(location);

        Ok(self)
    }

    /// Returns source name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns source content identity.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Returns source language.
    #[must_use]
    pub fn language(
        &self,
    ) -> Option<&str> {
        self.language.as_deref()
    }

    /// Returns source-language version.
    #[must_use]
    pub fn language_version(
        &self,
    ) -> Option<&str> {
        self.language_version.as_deref()
    }

    /// Returns source revision.
    #[must_use]
    pub fn revision(
        &self,
    ) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Returns source location.
    #[must_use]
    pub fn location(
        &self,
    ) -> Option<&str> {
        self.location.as_deref()
    }
}

// =============================================================================
// Tool identity
// =============================================================================

/// Identity of a tool, compiler, estimator, simulator, or other implementation
/// that contributed to a ZQN artifact.
///
/// This is descriptive identity. It does not authenticate the implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToolIdentity {
    name: String,
    version: String,
    build: Option<String>,
    source_revision: Option<String>,
}

impl ToolIdentity {
    /// Creates a tool identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();
        let version = version.into();

        validate_non_empty(
            "tool.name",
            &name,
        )?;

        validate_non_empty(
            "tool.version",
            &version,
        )?;

        Ok(Self {
            name,
            version,
            build: None,
            source_revision: None,
        })
    }

    /// Sets build identity.
    pub fn set_build(
        &mut self,
        build: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let build = build.into();

        validate_non_empty(
            "tool.build",
            &build,
        )?;

        self.build = Some(build);

        Ok(())
    }

    /// Sets source revision.
    pub fn set_source_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let revision = revision.into();

        validate_non_empty(
            "tool.source_revision",
            &revision,
        )?;

        self.source_revision = Some(revision);

        Ok(())
    }

    /// Returns tool name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns tool version.
    #[must_use]
    pub fn version(
        &self,
    ) -> &str {
        &self.version
    }

    /// Returns build identity.
    #[must_use]
    pub fn build(
        &self,
    ) -> Option<&str> {
        self.build.as_deref()
    }

    /// Returns source revision.
    #[must_use]
    pub fn source_revision(
        &self,
    ) -> Option<&str> {
        self.source_revision.as_deref()
    }
}

// =============================================================================
// Noise model identity
// =============================================================================

/// Identity reference for a ZQN noise model.
///
/// The model implementation remains outside provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseModelReference {
    name: String,
    version: Option<String>,
    artifact: Option<ArtifactReference>,
}

impl NoiseModelReference {
    /// Creates a noise-model reference.
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "noise_model.name",
            &name,
        )?;

        Ok(Self {
            name,
            version: None,
            artifact: None,
        })
    }

    /// Sets model version.
    pub fn set_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();

        validate_non_empty(
            "noise_model.version",
            &version,
        )?;

        self.version = Some(version);

        Ok(())
    }

    /// Associates a model artifact.
    pub fn set_artifact(
        &mut self,
        artifact: ArtifactReference,
    ) -> Result<(), ProvenanceError> {
        if artifact.kind()
            != ArtifactKind::NoiseModel
            && artifact.kind()
                != ArtifactKind::NoiseSpecification
        {
            return Err(
                ProvenanceError::InvalidRelationship {
                    message: String::from(
                        "noise model reference must point to a \
                         noise model or noise specification artifact",
                    ),
                },
            );
        }

        self.artifact = Some(artifact);

        Ok(())
    }

    /// Returns model name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns model version.
    #[must_use]
    pub fn version(
        &self,
    ) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns model artifact.
    #[must_use]
    pub fn artifact(
        &self,
    ) -> Option<&ArtifactReference> {
        self.artifact.as_ref()
    }
}

// =============================================================================
// Target reference
// =============================================================================

/// Descriptive reference to a target.
///
/// This intentionally does not contain hardware topology, credentials, or
/// provider-specific runtime state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetReference {
    name: String,
    version: Option<String>,
    descriptor: Option<ContentReference>,
}

impl TargetReference {
    /// Creates a target reference.
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "target.name",
            &name,
        )?;

        Ok(Self {
            name,
            version: None,
            descriptor: None,
        })
    }

    /// Sets target version.
    pub fn set_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();

        validate_non_empty(
            "target.version",
            &version,
        )?;

        self.version = Some(version);

        Ok(())
    }

    /// Associates target descriptor content identity.
    #[must_use]
    pub fn with_descriptor(
        mut self,
        descriptor: ContentReference,
    ) -> Self {
        self.descriptor = Some(descriptor);
        self
    }

    /// Returns target name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns target version.
    #[must_use]
    pub fn version(
        &self,
    ) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns target descriptor.
    #[must_use]
    pub const fn descriptor(
        &self,
    ) -> Option<&ContentReference> {
        self.descriptor.as_ref()
    }
}

// =============================================================================
// Calibration reference
// =============================================================================

/// Reference to calibration information.
///
/// The calibration payload remains outside provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalibrationReference {
    id: CalibrationId,
    name: Option<String>,
    content: Option<ContentReference>,
}

impl CalibrationReference {
    /// Creates a calibration reference using the canonical IR calibration ID.
    #[must_use]
    pub const fn new(
        id: CalibrationId,
    ) -> Self {
        Self {
            id,
            name: None,
            content: None,
        }
    }

    /// Sets a descriptive calibration name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "calibration.name",
            &name,
        )?;

        self.name = Some(name);

        Ok(())
    }

    /// Associates calibration content identity.
    #[must_use]
    pub fn with_content(
        mut self,
        content: ContentReference,
    ) -> Self {
        self.content = Some(content);
        self
    }

    /// Returns calibration identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> CalibrationId {
        self.id
    }

    /// Returns calibration name.
    #[must_use]
    pub fn name(
        &self,
    ) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns calibration content identity.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }
}

// =============================================================================
// Characterization reference
// =============================================================================

/// Reference to a noise-characterization artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterizationReference {
    name: String,
    protocol: Option<String>,
    artifact: Option<ArtifactReference>,
}

impl CharacterizationReference {
    /// Creates a characterization reference.
    pub fn new(
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "characterization.name",
            &name,
        )?;

        Ok(Self {
            name,
            protocol: None,
            artifact: None,
        })
    }

    /// Sets characterization protocol.
    pub fn set_protocol(
        &mut self,
        protocol: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let protocol = protocol.into();

        validate_non_empty(
            "characterization.protocol",
            &protocol,
        )?;

        self.protocol = Some(protocol);

        Ok(())
    }

    /// Associates a characterization artifact.
    pub fn set_artifact(
        &mut self,
        artifact: ArtifactReference,
    ) -> Result<(), ProvenanceError> {
        match artifact.kind() {
            ArtifactKind::CharacterizationExperiment
            | ArtifactKind::CharacterizationResult
            | ArtifactKind::Observation => {}

            _ => {
                return Err(
                    ProvenanceError::InvalidRelationship {
                        message: String::from(
                            "characterization reference must point to \
                             an experiment, result, or observation artifact",
                        ),
                    },
                );
            }
        }

        self.artifact = Some(artifact);

        Ok(())
    }

    /// Returns characterization name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns protocol.
    #[must_use]
    pub fn protocol(
        &self,
    ) -> Option<&str> {
        self.protocol.as_deref()
    }

    /// Returns artifact.
    #[must_use]
    pub fn artifact(
        &self,
    ) -> Option<&ArtifactReference> {
        self.artifact.as_ref()
    }
}

// =============================================================================
// Operation reference
// =============================================================================

/// Records an IR operation affected by or associated with a ZQN artifact.
///
/// The operation identity is owned by canonical Quantum IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationReference {
    operation: OperationId,
}

impl OperationReference {
    /// Creates an operation reference.
    #[must_use]
    pub const fn new(
        operation: OperationId,
    ) -> Self {
        Self { operation }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(
        &self,
    ) -> OperationId {
        self.operation
    }
}

// =============================================================================
// Qubit mapping
// =============================================================================

/// Records one logical-to-physical relationship.
///
/// This record is provenance evidence only. It does not perform routing or
/// establish hardware validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitMappingRecord {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitMappingRecord {
    /// Creates a mapping using canonical IR qubit identities.
    #[must_use]
    pub const fn new(
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Self {
        Self {
            logical,
            physical,
        }
    }

    /// Returns logical qubit.
    #[must_use]
    pub const fn logical(
        &self,
    ) -> QubitId {
        self.logical
    }

    /// Returns physical qubit.
    #[must_use]
    pub const fn physical(
        &self,
    ) -> PhysicalQubitId {
        self.physical
    }
}

// =============================================================================
// Deterministic execution context
// =============================================================================

/// Deterministic context identifying a reproducible stochastic realization.
///
/// A seed alone is insufficient to reproduce a distributed or parallel
/// execution. The optional identities allow downstream systems to derive
/// stable per-operation/per-resource randomness without storing hidden global
/// RNG state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicContext {
    master_seed: u64,
    shot_index: Option<u64>,
    program_id: Option<ProgramId>,
    model: Option<NoiseModelReference>,
    calibration_ids: Vec<CalibrationId>,
}

impl DeterministicContext {
    /// Creates a deterministic context from a caller-owned master seed.
    #[must_use]
    pub const fn new(
        master_seed: u64,
    ) -> Self {
        Self {
            master_seed,
            shot_index: None,
            program_id: None,
            model: None,
            calibration_ids: Vec::new(),
        }
    }

    /// Sets the execution shot/index.
    #[must_use]
    pub fn with_shot_index(
        mut self,
        shot_index: u64,
    ) -> Self {
        self.shot_index = Some(shot_index);
        self
    }

    /// Associates the canonical program identity.
    #[must_use]
    pub fn with_program_id(
        mut self,
        program_id: ProgramId,
    ) -> Self {
        self.program_id = Some(program_id);
        self
    }

    /// Associates the noise model.
    #[must_use]
    pub fn with_model(
        mut self,
        model: NoiseModelReference,
    ) -> Self {
        self.model = Some(model);
        self
    }

    /// Adds a calibration identity.
    ///
    /// Duplicate identities are ignored and the collection remains
    /// deterministically sorted.
    pub fn add_calibration(
        &mut self,
        calibration: CalibrationId,
    ) {
        match self.calibration_ids.binary_search(
            &calibration,
        ) {
            Ok(_) => {}

            Err(index) => {
                self.calibration_ids
                    .insert(index, calibration);
            }
        }
    }

    /// Returns master seed.
    #[must_use]
    pub const fn master_seed(
        &self,
    ) -> u64 {
        self.master_seed
    }

    /// Returns shot index.
    #[must_use]
    pub const fn shot_index(
        &self,
    ) -> Option<u64> {
        self.shot_index
    }

    /// Returns program identity.
    #[must_use]
    pub const fn program_id(
        &self,
    ) -> Option<ProgramId> {
        self.program_id
    }

    /// Returns noise model reference.
    #[must_use]
    pub const fn model(
        &self,
    ) -> Option<&NoiseModelReference> {
        self.model.as_ref()
    }

    /// Returns calibration identities.
    #[must_use]
    pub fn calibration_ids(
        &self,
    ) -> &[CalibrationId] {
        &self.calibration_ids
    }
}

// =============================================================================
// Execution reference
// =============================================================================

/// Observational reference to an external execution.
///
/// This intentionally stores identifiers/references rather than execution
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionReference {
    job_id: Option<String>,
    attempt: Option<u64>,
    result: Option<ContentReference>,
}

impl ExecutionReference {
    /// Creates an empty execution reference.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            job_id: None,
            attempt: None,
            result: None,
        }
    }

    /// Sets an external job identifier.
    pub fn set_job_id(
        &mut self,
        job_id: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let job_id = job_id.into();

        validate_non_empty(
            "execution.job_id",
            &job_id,
        )?;

        self.job_id = Some(job_id);

        Ok(())
    }

    /// Sets an execution attempt number.
    #[must_use]
    pub fn with_attempt(
        mut self,
        attempt: u64,
    ) -> Self {
        self.attempt = Some(attempt);
        self
    }

    /// Associates the execution result.
    #[must_use]
    pub fn with_result(
        mut self,
        result: ContentReference,
    ) -> Self {
        self.result = Some(result);
        self
    }

    /// Returns job identifier.
    #[must_use]
    pub fn job_id(
        &self,
    ) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Returns attempt number.
    #[must_use]
    pub const fn attempt(
        &self,
    ) -> Option<u64> {
        self.attempt
    }

    /// Returns result identity.
    #[must_use]
    pub const fn result(
        &self,
    ) -> Option<&ContentReference> {
        self.result.as_ref()
    }
}

impl Default for ExecutionReference {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Timestamp
// =============================================================================

/// Explicit wall-clock timestamp.
///
/// This is observational provenance. It MUST NOT be included automatically in
/// semantic/content identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceTimestamp {
    seconds_since_unix_epoch: u64,
    nanoseconds: u32,
}

impl ProvenanceTimestamp {
    /// Creates a timestamp from Unix epoch seconds and nanoseconds.
    pub const fn new(
        seconds_since_unix_epoch: u64,
        nanoseconds: u32,
    ) -> Result<Self, ProvenanceError> {
        if nanoseconds >= 1_000_000_000 {
            return Err(
                ProvenanceError::InvalidTimestamp {
                    seconds: seconds_since_unix_epoch,
                    nanoseconds,
                },
            );
        }

        Ok(Self {
            seconds_since_unix_epoch,
            nanoseconds,
        })
    }

    /// Captures the current system time.
    ///
    /// This must only be used for observational provenance.
    pub fn now() -> Result<Self, ProvenanceError> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| {
                ProvenanceError::ClockBeforeUnixEpoch
            })?;

        Self::new(
            duration.as_secs(),
            duration.subsec_nanos(),
        )
    }

    /// Returns seconds since Unix epoch.
    #[must_use]
    pub const fn seconds(
        &self,
    ) -> u64 {
        self.seconds_since_unix_epoch
    }

    /// Returns nanoseconds.
    #[must_use]
    pub const fn nanoseconds(
        &self,
    ) -> u32 {
        self.nanoseconds
    }
}

impl fmt::Display for ProvenanceTimestamp {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}Z",
            self.seconds_since_unix_epoch,
            self.nanoseconds
        )
    }
}

// =============================================================================
// Transformation
// =============================================================================

/// Semantic category of a ZQN transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationKind {
    /// Construction from source/input data.
    Construction,

    /// Characterization-derived model generation.
    Characterization,

    /// Calibration-derived update.
    CalibrationUpdate,

    /// Model composition.
    Composition,

    /// Model simplification.
    Simplification,

    /// Exact representation conversion.
    RepresentationConversion,

    /// Explicit approximation.
    Approximation,

    /// Noise attachment to IR operations.
    Application,

    /// Fault generation.
    FaultGeneration,

    /// Deterministic sampling.
    Sampling,

    /// Target lowering.
    TargetLowering,

    /// Serialization/deserialization migration.
    SchemaMigration,

    /// User-defined extension.
    Extension,
}

/// Records one transformation in ZQN lineage.
///
/// Sequence numbers are explicit and must be strictly increasing within a
/// provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformationRecord {
    sequence: u64,
    kind: TransformationKind,
    name: String,
    version: Option<String>,
    implementation: Option<ToolIdentity>,
    input: Option<ContentReference>,
    output: Option<ContentReference>,
    operation: Option<OperationId>,
    deterministic_context: Option<DeterministicContext>,
}

impl TransformationRecord {
    /// Creates a transformation record.
    pub fn new(
        sequence: u64,
        kind: TransformationKind,
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty(
            "transformation.name",
            &name,
        )?;

        Ok(Self {
            sequence,
            kind,
            name,
            version: None,
            implementation: None,
            input: None,
            output: None,
            operation: None,
            deterministic_context: None,
        })
    }

    /// Sets transformation version.
    pub fn set_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();

        validate_non_empty(
            "transformation.version",
            &version,
        )?;

        self.version = Some(version);

        Ok(())
    }

    /// Associates the implementation/tool.
    #[must_use]
    pub fn with_implementation(
        mut self,
        implementation: ToolIdentity,
    ) -> Self {
        self.implementation = Some(implementation);
        self
    }

    /// Associates input content.
    #[must_use]
    pub fn with_input(
        mut self,
        input: ContentReference,
    ) -> Self {
        self.input = Some(input);
        self
    }

    /// Associates output content.
    #[must_use]
    pub fn with_output(
        mut self,
        output: ContentReference,
    ) -> Self {
        self.output = Some(output);
        self
    }

    /// Associates an IR operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Associates deterministic execution context.
    #[must_use]
    pub fn with_deterministic_context(
        mut self,
        context: DeterministicContext,
    ) -> Self {
        self.deterministic_context = Some(context);
        self
    }

    /// Returns sequence number.
    #[must_use]
    pub const fn sequence(
        &self,
    ) -> u64 {
        self.sequence
    }

    /// Returns transformation kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> TransformationKind {
        self.kind
    }

    /// Returns transformation name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns transformation version.
    #[must_use]
    pub fn version(
        &self,
    ) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns implementation identity.
    #[must_use]
    pub const fn implementation(
        &self,
    ) -> Option<&ToolIdentity> {
        self.implementation.as_ref()
    }

    /// Returns input content.
    #[must_use]
    pub const fn input(
        &self,
    ) -> Option<&ContentReference> {
        self.input.as_ref()
    }

    /// Returns output content.
    #[must_use]
    pub const fn output(
        &self,
    ) -> Option<&ContentReference> {
        self.output.as_ref()
    }

    /// Returns associated operation.
    #[must_use]
    pub const fn operation(
        &self,
    ) -> Option<OperationId> {
        self.operation
    }

    /// Returns deterministic context.
    #[must_use]
    pub const fn deterministic_context(
        &self,
    ) -> Option<&DeterministicContext> {
        self.deterministic_context.as_ref()
    }
}

// =============================================================================
// Deterministic metadata
// =============================================================================

/// Deterministically ordered metadata.
///
/// Metadata is descriptive. It is not automatically semantic.
///
/// The serialization layer must explicitly decide which metadata fields, if
/// any, participate in semantic identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProvenanceMetadata {
    entries: BTreeMap<String, String>,
}

impl ProvenanceMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts or replaces metadata.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let key = key.into();

        validate_non_empty(
            "metadata.key",
            &key,
        )?;

        self.entries.insert(
            key,
            value.into(),
        );

        Ok(())
    }

    /// Removes a metadata value.
    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.entries.remove(key)
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.entries
            .get(key)
            .map(String::as_str)
    }

    /// Returns number of entries.
    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.entries.len()
    }

    /// Returns whether there are no entries.
    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.entries.is_empty()
    }

    /// Returns deterministic key/value iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.entries.iter().map(
            |(key, value)| {
                (
                    key.as_str(),
                    value.as_str(),
                )
            },
        )
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Complete provenance record for a ZQN artifact.
///
/// The object intentionally separates deterministic lineage from observational
/// execution information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Provenance {
    schema_version: u16,

    /// Canonical provenance identity owned by the IR identity subsystem.
    id: ProvenanceId,

    /// Canonical IR version associated with the artifact.
    ir_version: IrVersion,

    /// Canonical program identity when applicable.
    program_id: Option<ProgramId>,

    source: Option<SourceReference>,

    tool: Option<ToolIdentity>,

    model: Option<NoiseModelReference>,

    input_artifact: Option<ContentReference>,

    output_artifact: Option<ContentReference>,

    transformations: Vec<TransformationRecord>,

    target: Option<TargetReference>,

    calibrations: Vec<CalibrationReference>,

    characterizations: Vec<CharacterizationReference>,

    mappings: Vec<QubitMappingRecord>,

    operations: Vec<OperationId>,

    deterministic_context: Option<DeterministicContext>,

    execution: Option<ExecutionReference>,

    created_at: Option<ProvenanceTimestamp>,

    completed_at: Option<ProvenanceTimestamp>,

    metadata: ProvenanceMetadata,
}

impl Provenance {
    /// Creates current-schema ZQN provenance.
    #[must_use]
    pub const fn new(
        id: ProvenanceId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            schema_version:
                ZQN_PROVENANCE_SCHEMA_VERSION,
            id,
            ir_version,
            program_id: None,
            source: None,
            tool: None,
            model: None,
            input_artifact: None,
            output_artifact: None,
            transformations: Vec::new(),
            target: None,
            calibrations: Vec::new(),
            characterizations: Vec::new(),
            mappings: Vec::new(),
            operations: Vec::new(),
            deterministic_context: None,
            execution: None,
            created_at: None,
            completed_at: None,
            metadata: ProvenanceMetadata::new(),
        }
    }

    /// Creates current-schema provenance associated with a canonical program.
    #[must_use]
    pub const fn for_program(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        let mut provenance =
            Self::new(
                id,
                ir_version,
            );

        provenance.program_id =
            Some(program_id);

        provenance
    }

    /// Returns schema version.
    #[must_use]
    pub const fn schema_version(
        &self,
    ) -> u16 {
        self.schema_version
    }

    /// Returns provenance identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> ProvenanceId {
        self.id
    }

    /// Returns IR version.
    #[must_use]
    pub const fn ir_version(
        &self,
    ) -> IrVersion {
        self.ir_version
    }

    /// Returns associated program identity.
    #[must_use]
    pub const fn program_id(
        &self,
    ) -> Option<ProgramId> {
        self.program_id
    }

    /// Sets canonical program identity.
    #[must_use]
    pub fn with_program_id(
        mut self,
        program_id: ProgramId,
    ) -> Self {
        self.program_id =
            Some(program_id);

        self
    }

    /// Sets source reference.
    #[must_use]
    pub fn with_source(
        mut self,
        source: SourceReference,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets primary tool identity.
    #[must_use]
    pub fn with_tool(
        mut self,
        tool: ToolIdentity,
    ) -> Self {
        self.tool = Some(tool);
        self
    }

    /// Sets noise model reference.
    #[must_use]
    pub fn with_model(
        mut self,
        model: NoiseModelReference,
    ) -> Self {
        self.model = Some(model);
        self
    }

    /// Sets input content identity.
    #[must_use]
    pub fn with_input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.input_artifact =
            Some(artifact);

        self
    }

    /// Sets output content identity.
    #[must_use]
    pub fn with_output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.output_artifact =
            Some(artifact);

        self
    }

    /// Sets target reference.
    #[must_use]
    pub fn with_target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets deterministic context.
    #[must_use]
    pub fn with_deterministic_context(
        mut self,
        context: DeterministicContext,
    ) -> Self {
        self.deterministic_context =
            Some(context);

        self
    }

    /// Adds a transformation.
    pub fn add_transformation(
        &mut self,
        record: TransformationRecord,
    ) -> Result<(), ProvenanceError> {
        if let Some(previous) =
            self.transformations.last()
        {
            if record.sequence()
                <= previous.sequence()
            {
                return Err(
                    ProvenanceError::InvalidRelationship {
                        message: format!(
                            "transformation sequence {} \
                             is not greater than {}",
                            record.sequence(),
                            previous.sequence()
                        ),
                    },
                );
            }
        }

        self.transformations.push(record);

        Ok(())
    }

    /// Adds a calibration reference.
    ///
    /// Exact duplicates are ignored.
    pub fn add_calibration(
        &mut self,
        calibration: CalibrationReference,
    ) {
        if !self.calibrations.contains(
            &calibration,
        ) {
            self.calibrations.push(
                calibration,
            );

            self.calibrations
                .sort_by_key(|entry| {
                    (
                        entry.id(),
                        entry.name()
                            .unwrap_or(""),
                    )
                });
        }
    }

    /// Adds a characterization reference.
    ///
    /// Exact duplicates are ignored.
    pub fn add_characterization(
        &mut self,
        characterization: CharacterizationReference,
    ) {
        if !self.characterizations
            .contains(&characterization)
        {
            self.characterizations.push(
                characterization,
            );

            self.characterizations
                .sort_by_key(|entry| {
                    (
                        entry.name(),
                        entry.protocol()
                            .unwrap_or(""),
                    )
                });
        }
    }

    /// Adds a logical-to-physical mapping.
    ///
    /// A provenance snapshot represents a mapping relation, not a routing
    /// history. Therefore each logical and physical resource can occur at most
    /// once in the snapshot.
    pub fn add_mapping(
        &mut self,
        mapping: QubitMappingRecord,
    ) -> Result<(), ProvenanceError> {
        match self.mappings
            .binary_search_by_key(
                &mapping.logical(),
                |entry| entry.logical(),
            )
        {
            Ok(index) => {
                let existing =
                    self.mappings[index];

                if existing.physical()
                    == mapping.physical()
                {
                    return Ok(());
                }

                Err(
                    ProvenanceError::InvalidRelationship {
                        message: format!(
                            "logical qubit {} is already mapped \
                             to physical qubit {}",
                            existing.logical(),
                            existing.physical()
                        ),
                    },
                )
            }

            Err(index) => {
                if self.mappings.iter().any(
                    |entry| {
                        entry.physical()
                            == mapping.physical()
                    },
                ) {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: format!(
                                "physical qubit {} is already mapped",
                                mapping.physical()
                            ),
                        },
                    );
                }

                self.mappings.insert(
                    index,
                    mapping,
                );

                Ok(())
            }
        }
    }

    /// Records a canonical IR operation identity.
    ///
    /// Duplicate identities are ignored.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) {
        match self.operations
            .binary_search(&operation)
        {
            Ok(_) => {}

            Err(index) => {
                self.operations
                    .insert(index, operation);
            }
        }
    }

    /// Adds deterministic metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        self.metadata.insert(
            key,
            value,
        )
    }

    /// Sets execution observation.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.execution =
            Some(execution);

        self
    }

    /// Sets observational creation timestamp.
    #[must_use]
    pub fn with_created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.created_at =
            Some(timestamp);

        self
    }

    /// Sets observational completion timestamp.
    #[must_use]
    pub fn with_completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.completed_at =
            Some(timestamp);

        self
    }

    /// Returns source reference.
    #[must_use]
    pub const fn source(
        &self,
    ) -> Option<&SourceReference> {
        self.source.as_ref()
    }

    /// Returns primary tool identity.
    #[must_use]
    pub const fn tool(
        &self,
    ) -> Option<&ToolIdentity> {
        self.tool.as_ref()
    }

    /// Returns noise model reference.
    #[must_use]
    pub const fn model(
        &self,
    ) -> Option<&NoiseModelReference> {
        self.model.as_ref()
    }

    /// Returns input artifact identity.
    #[must_use]
    pub const fn input_artifact(
        &self,
    ) -> Option<&ContentReference> {
        self.input_artifact.as_ref()
    }

    /// Returns output artifact identity.
    #[must_use]
    pub const fn output_artifact(
        &self,
    ) -> Option<&ContentReference> {
        self.output_artifact.as_ref()
    }

    /// Returns transformations in execution/derivation order.
    #[must_use]
    pub fn transformations(
        &self,
    ) -> &[TransformationRecord] {
        &self.transformations
    }

    /// Returns target reference.
    #[must_use]
    pub const fn target(
        &self,
    ) -> Option<&TargetReference> {
        self.target.as_ref()
    }

    /// Returns calibration references.
    #[must_use]
    pub fn calibrations(
        &self,
    ) -> &[CalibrationReference] {
        &self.calibrations
    }

    /// Returns characterization references.
    #[must_use]
    pub fn characterizations(
        &self,
    ) -> &[CharacterizationReference] {
        &self.characterizations
    }

    /// Returns qubit mappings.
    #[must_use]
    pub fn mappings(
        &self,
    ) -> &[QubitMappingRecord] {
        &self.mappings
    }

    /// Returns operation identities.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &[OperationId] {
        &self.operations
    }

    /// Returns deterministic context.
    #[must_use]
    pub const fn deterministic_context(
        &self,
    ) -> Option<&DeterministicContext> {
        self.deterministic_context
            .as_ref()
    }

    /// Returns execution observation.
    #[must_use]
    pub const fn execution(
        &self,
    ) -> Option<&ExecutionReference> {
        self.execution.as_ref()
    }

    /// Returns creation timestamp.
    #[must_use]
    pub const fn created_at(
        &self,
    ) -> Option<ProvenanceTimestamp> {
        self.created_at
    }

    /// Returns completion timestamp.
    #[must_use]
    pub const fn completed_at(
        &self,
    ) -> Option<ProvenanceTimestamp> {
        self.completed_at
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &ProvenanceMetadata {
        &self.metadata
    }

    /// Returns whether observational information is present.
    #[must_use]
    pub const fn contains_observational_data(
        &self,
    ) -> bool {
        self.execution.is_some()
            || self.created_at.is_some()
            || self.completed_at.is_some()
    }

    /// Returns whether the provenance object itself contains no observational
    /// execution/time information.
    ///
    /// This does not claim that referenced artifacts are deterministic.
    #[must_use]
    pub const fn is_deterministic(
        &self,
    ) -> bool {
        !self.contains_observational_data()
    }

    /// Returns whether deterministic execution context is present.
    #[must_use]
    pub const fn has_deterministic_context(
        &self,
    ) -> bool {
        self.deterministic_context
            .is_some()
    }

    /// Validates the complete provenance object.
    pub fn validate(
        &self,
    ) -> Result<(), ProvenanceError> {
        if self.schema_version
            > ZQN_PROVENANCE_SCHEMA_VERSION
        {
            return Err(
                ProvenanceError::UnsupportedSchemaVersion {
                    found: self.schema_version,
                    supported:
                        ZQN_PROVENANCE_SCHEMA_VERSION,
                },
            );
        }

        if let Some(source) =
            &self.source
        {
            validate_non_empty(
                "source.name",
                source.name(),
            )?;
        }

        if let Some(tool) =
            &self.tool
        {
            validate_non_empty(
                "tool.name",
                tool.name(),
            )?;

            validate_non_empty(
                "tool.version",
                tool.version(),
            )?;
        }

        if let Some(model) =
            &self.model
        {
            validate_non_empty(
                "noise_model.name",
                model.name(),
            )?;
        }

        if let Some(target) =
            &self.target
        {
            validate_non_empty(
                "target.name",
                target.name(),
            )?;
        }

        if let (
            Some(created),
            Some(completed),
        ) = (
            self.created_at,
            self.completed_at,
        ) {
            if completed < created {
                return Err(
                    ProvenanceError::InvalidRelationship {
                        message: String::from(
                            "completed_at precedes created_at",
                        ),
                    },
                );
            }
        }

        if let Some(context) =
            &self.deterministic_context
        {
            let mut previous =
                None;

            for calibration
                in context.calibration_ids()
            {
                if let Some(previous_id) =
                    previous
                {
                    if *calibration
                        <= previous_id
                    {
                        return Err(
                            ProvenanceError::InvalidRelationship {
                                message: String::from(
                                    "deterministic calibration identities \
                                     are not strictly ordered",
                                ),
                            },
                        );
                    }
                }

                previous =
                    Some(*calibration);
            }
        }

        let mut previous_sequence =
            None;

        for transformation
            in &self.transformations
        {
            if let Some(previous) =
                previous_sequence
            {
                if transformation.sequence()
                    <= previous
                {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: String::from(
                                "transformation sequence is not strictly increasing",
                            ),
                        },
                    );
                }
            }

            validate_non_empty(
                "transformation.name",
                transformation.name(),
            )?;

            previous_sequence =
                Some(transformation.sequence());
        }

        let mut previous_operation =
            None;

        for operation
            in &self.operations
        {
            if let Some(previous) =
                previous_operation
            {
                if *operation <= previous {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: String::from(
                                "operation identities are not strictly ordered",
                            ),
                        },
                    );
                }
            }

            previous_operation =
                Some(*operation);
        }

        let mut previous_logical =
            None;

        for mapping
            in &self.mappings
        {
            if let Some(previous) =
                previous_logical
            {
                if mapping.logical()
                    <= previous
                {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: String::from(
                                "logical qubit mappings are not strictly ordered",
                            ),
                        },
                    );
                }
            }

            previous_logical =
                Some(mapping.logical());
        }

        let mut previous_physical =
            None;

        for mapping
            in &self.mappings
        {
            if let Some(previous) =
                previous_physical
            {
                if mapping.physical()
                    == previous
                {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: format!(
                                "physical qubit {} is mapped more than once",
                                mapping.physical()
                            ),
                        },
                    );
                }
            }

            previous_physical =
                Some(mapping.physical());
        }

        for calibration
            in &self.calibrations
        {
            if let Some(name) =
                calibration.name()
            {
                validate_non_empty(
                    "calibration.name",
                    name,
                )?;
            }
        }

        for characterization
            in &self.characterizations
        {
            validate_non_empty(
                "characterization.name",
                characterization.name(),
            )?;
        }

        Ok(())
    }

    /// Returns the number of transformations.
    #[must_use]
    pub fn transformation_count(
        &self,
    ) -> usize {
        self.transformations.len()
    }

    /// Returns number of calibration references.
    #[must_use]
    pub fn calibration_count(
        &self,
    ) -> usize {
        self.calibrations.len()
    }

    /// Returns number of characterization references.
    #[must_use]
    pub fn characterization_count(
        &self,
    ) -> usize {
        self.characterizations.len()
    }

    /// Returns number of mappings.
    #[must_use]
    pub fn mapping_count(
        &self,
    ) -> usize {
        self.mappings.len()
    }

    /// Returns number of associated operations.
    #[must_use]
    pub fn operation_count(
        &self,
    ) -> usize {
        self.operations.len()
    }

    /// Returns whether an operation is associated.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operations
            .binary_search(&operation)
            .is_ok()
    }

    /// Returns the physical qubit associated with a logical qubit.
    #[must_use]
    pub fn physical_for_logical(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.mappings
            .binary_search_by_key(
                &logical,
                |entry| entry.logical(),
            )
            .ok()
            .map(
                |index| {
                    self.mappings[index]
                        .physical()
                },
            )
    }

    /// Returns the logical qubit associated with a physical qubit.
    #[must_use]
    pub fn logical_for_physical(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.mappings
            .iter()
            .find(
                |entry| {
                    entry.physical()
                        == physical
                },
            )
            .map(
                QubitMappingRecord::logical,
            )
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for validated ZQN provenance.
///
/// Every mutating operation maintains the local collection invariants where
/// possible. `build()` performs complete validation before returning the final
/// record.
#[derive(Debug, Clone)]
pub struct ProvenanceBuilder {
    provenance: Provenance,
}

impl ProvenanceBuilder {
    /// Creates a provenance builder.
    #[must_use]
    pub const fn new(
        id: ProvenanceId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            provenance:
                Provenance::new(
                    id,
                    ir_version,
                ),
        }
    }

    /// Creates a provenance builder associated with a canonical program.
    #[must_use]
    pub const fn for_program(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            provenance:
                Provenance::for_program(
                    id,
                    program_id,
                    ir_version,
                ),
        }
    }

    /// Sets source.
    #[must_use]
    pub fn source(
        mut self,
        source: SourceReference,
    ) -> Self {
        self.provenance.source =
            Some(source);

        self
    }

    /// Sets primary tool.
    #[must_use]
    pub fn tool(
        mut self,
        tool: ToolIdentity,
    ) -> Self {
        self.provenance.tool =
            Some(tool);

        self
    }

    /// Sets noise model.
    #[must_use]
    pub fn model(
        mut self,
        model: NoiseModelReference,
    ) -> Self {
        self.provenance.model =
            Some(model);

        self
    }

    /// Sets input artifact.
    #[must_use]
    pub fn input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.input_artifact =
            Some(artifact);

        self
    }

    /// Sets output artifact.
    #[must_use]
    pub fn output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.output_artifact =
            Some(artifact);

        self
    }

    /// Sets target.
    #[must_use]
    pub fn target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.provenance.target =
            Some(target);

        self
    }

    /// Adds a transformation.
    pub fn transformation(
        mut self,
        transformation: TransformationRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_transformation(
                transformation,
            )?;

        Ok(self)
    }

    /// Adds calibration.
    #[must_use]
    pub fn calibration(
        mut self,
        calibration: CalibrationReference,
    ) -> Self {
        self.provenance
            .add_calibration(
                calibration,
            );

        self
    }

    /// Adds characterization.
    #[must_use]
    pub fn characterization(
        mut self,
        characterization: CharacterizationReference,
    ) -> Self {
        self.provenance
            .add_characterization(
                characterization,
            );

        self
    }

    /// Adds qubit mapping.
    pub fn mapping(
        mut self,
        mapping: QubitMappingRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_mapping(mapping)?;

        Ok(self)
    }

    /// Adds an IR operation identity.
    #[must_use]
    pub fn operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.provenance
            .add_operation(operation);

        self
    }

    /// Sets deterministic context.
    #[must_use]
    pub fn deterministic_context(
        mut self,
        context: DeterministicContext,
    ) -> Self {
        self.provenance
            .deterministic_context =
            Some(context);

        self
    }

    /// Sets observational execution reference.
    #[must_use]
    pub fn execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.provenance.execution =
            Some(execution);

        self
    }

    /// Sets creation timestamp.
    #[must_use]
    pub fn created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance.created_at =
            Some(timestamp);

        self
    }

    /// Sets completion timestamp.
    #[must_use]
    pub fn completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance.completed_at =
            Some(timestamp);

        self
    }

    /// Adds metadata.
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_metadata(
                key,
                value,
            )?;

        Ok(self)
    }

    /// Builds and fully validates provenance.
    pub fn build(
        self,
    ) -> Result<Provenance, ProvenanceError> {
        self.provenance.validate()?;

        Ok(self.provenance)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates that a required textual field is non-empty.
///
/// This intentionally does not trim whitespace: normalization belongs to the
/// owning schema/serialization layer and silently changing user-provided data
/// here would make provenance lossy.
fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Err(
            ProvenanceError::EmptyField {
                field,
            },
        );
    }

    Ok(())
}

/// Validates a textual field against a caller-selected resource policy.
///
/// This is NOT an architectural limit.
pub fn validate_field_size(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ProvenanceError> {
    if value.len() > maximum_bytes {
        return Err(
            ProvenanceError::FieldTooLarge {
                field,
                actual_bytes: value.len(),
                maximum_bytes,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    fn provenance() -> Provenance {
        Provenance::new(
            ProvenanceId::new(1),
            IrVersion::CURRENT,
        )
    }

    #[test]
    fn digest_binary_hex_round_trip() {
        let mut bytes =
            [0u8; DIGEST_BYTES];

        for (index, byte)
            in bytes.iter_mut().enumerate()
        {
            *byte = index as u8;
        }

        let digest =
            ProvenanceDigest::from_bytes(
                bytes,
            );

        let encoded =
            digest.to_hex();

        let decoded =
            ProvenanceDigest::from_hex(
                &encoded,
            )
            .expect(
                "valid digest must decode",
            );

        assert_eq!(
            digest,
            decoded
        );
    }

    #[test]
    fn digest_accepts_uppercase_hex() {
        let digest =
            ProvenanceDigest::from_hex(
                &"AB".repeat(DIGEST_BYTES),
            )
            .expect(
                "uppercase hexadecimal is valid",
            );

        assert_eq!(
            digest.as_bytes()[0],
            0xab
        );
    }

    #[test]
    fn digest_rejects_wrong_length() {
        let result =
            ProvenanceDigest::from_hex(
                "00",
            );

        assert!(result.is_err());
    }

    #[test]
    fn digest_rejects_non_hexadecimal_input() {
        let mut value =
            "00".repeat(DIGEST_BYTES);

        value.replace_range(
            0..1,
            "g",
        );

        assert!(
            matches!(
                ProvenanceDigest::from_hex(
                    &value
                ),
                Err(
                    ProvenanceError::InvalidDigestCharacter {
                        position: 0
                    }
                )
            )
        );
    }

    #[test]
    fn artifact_requires_name() {
        assert!(
            ArtifactReference::new(
                ArtifactKind::NoiseModel,
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn source_requires_name() {
        assert!(
            SourceReference::new("")
                .is_err()
        );
    }

    #[test]
    fn tool_requires_name_and_version() {
        assert!(
            ToolIdentity::new(
                "",
                "1.0.0",
            )
            .is_err()
        );

        assert!(
            ToolIdentity::new(
                "zamani",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn noise_model_requires_name() {
        assert!(
            NoiseModelReference::new(
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn target_requires_name() {
        assert!(
            TargetReference::new("")
                .is_err()
        );
    }

    #[test]
    fn characterization_requires_name() {
        assert!(
            CharacterizationReference::new(
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn transformations_are_strictly_ordered() {
        let mut record =
            provenance();

        record
            .add_transformation(
                TransformationRecord::new(
                    1,
                    TransformationKind::Construction,
                    "construct",
                )
                .expect(
                    "valid transformation",
                ),
            )
            .expect(
                "first transformation",
            );

        record
            .add_transformation(
                TransformationRecord::new(
                    2,
                    TransformationKind::CalibrationUpdate,
                    "calibrate",
                )
                .expect(
                    "valid transformation",
                ),
            )
            .expect(
                "second transformation",
            );

        assert_eq!(
            record.transformation_count(),
            2
        );

        assert!(
            record.validate().is_ok()
        );
    }

    #[test]
    fn duplicate_transformation_sequence_is_rejected() {
        let mut record =
            provenance();

        record
            .add_transformation(
                TransformationRecord::new(
                    1,
                    TransformationKind::Construction,
                    "construct",
                )
                .expect(
                    "valid transformation",
                ),
            )
            .expect(
                "first transformation",
            );

        assert!(
            record
                .add_transformation(
                    TransformationRecord::new(
                        1,
                        TransformationKind::Sampling,
                        "sample",
                    )
                    .expect(
                        "valid transformation",
                    ),
                )
                .is_err()
        );
    }

    #[test]
    fn canonical_qubit_types_are_used() {
        let mapping =
            QubitMappingRecord::new(
                QubitId::new(4),
                PhysicalQubitId::new(17),
            );

        assert_eq!(
            mapping.logical().index(),
            4
        );

        assert_eq!(
            mapping.physical().index(),
            17
        );
    }

    #[test]
    fn mappings_are_unique() {
        let mut record =
            provenance();

        record
            .add_mapping(
                QubitMappingRecord::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
            )
            .expect("first mapping");

        assert!(
            record
                .add_mapping(
                    QubitMappingRecord::new(
                        QubitId::new(0),
                        PhysicalQubitId::new(2),
                    ),
                )
                .is_err()
        );

        assert!(
            record
                .add_mapping(
                    QubitMappingRecord::new(
                        QubitId::new(1),
                        PhysicalQubitId::new(1),
                    ),
                )
                .is_err()
        );
    }

    #[test]
    fn identical_mapping_is_idempotent() {
        let mut record =
            provenance();

        let mapping =
            QubitMappingRecord::new(
                QubitId::new(0),
                PhysicalQubitId::new(1),
            );

        record
            .add_mapping(mapping)
            .expect("first mapping");

        record
            .add_mapping(mapping)
            .expect(
                "identical mapping is harmless",
            );

        assert_eq!(
            record.mapping_count(),
            1
        );
    }

    #[test]
    fn operation_ids_are_sorted_and_unique() {
        let mut record =
            provenance();

        record.add_operation(
            OperationId::new(9),
        );

        record.add_operation(
            OperationId::new(2),
        );

        record.add_operation(
            OperationId::new(9),
        );

        assert_eq!(
            record.operations(),
            &[
                OperationId::new(2),
                OperationId::new(9),
            ]
        );
    }

    #[test]
    fn deterministic_context_sorts_calibrations() {
        let mut context =
            DeterministicContext::new(
                1234,
            );

        context.add_calibration(
            CalibrationId::new(9),
        );

        context.add_calibration(
            CalibrationId::new(2),
        );

        context.add_calibration(
            CalibrationId::new(9),
        );

        assert_eq!(
            context.calibration_ids(),
            &[
                CalibrationId::new(2),
                CalibrationId::new(9),
            ]
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata =
            ProvenanceMetadata::new();

        metadata
            .insert("z", "last")
            .expect("metadata");

        metadata
            .insert("a", "first")
            .expect("metadata");

        let values: Vec<_> =
            metadata.iter().collect();

        assert_eq!(
            values,
            vec![
                ("a", "first"),
                ("z", "last"),
            ]
        );
    }

    #[test]
    fn metadata_replaces_existing_key() {
        let mut metadata =
            ProvenanceMetadata::new();

        metadata
            .insert("key", "old")
            .expect("metadata");

        metadata
            .insert("key", "new")
            .expect("metadata");

        assert_eq!(
            metadata.get("key"),
            Some("new"),
        );

        assert_eq!(
            metadata.len(),
            1,
        );
    }

    #[test]
    fn deterministic_provenance_contains_no_observations() {
        let record =
            provenance();

        assert!(
            record.is_deterministic()
        );

        assert!(
            !record
                .contains_observational_data()
        );
    }

    #[test]
    fn deterministic_context_is_not_observational() {
        let record =
            provenance()
                .with_deterministic_context(
                    DeterministicContext::new(
                        42,
                    ),
                );

        assert!(
            record.is_deterministic()
        );

        assert!(
            record
                .has_deterministic_context()
        );
    }

    #[test]
    fn timestamps_make_provenance_observational() {
        let timestamp =
            ProvenanceTimestamp::new(
                100,
                0,
            )
            .expect(
                "valid timestamp",
            );

        let record =
            provenance()
                .with_created_at(
                    timestamp,
                );

        assert!(
            !record.is_deterministic()
        );

        assert!(
            record
                .contains_observational_data()
        );
    }

    #[test]
    fn timestamp_order_is_validated() {
        let created =
            ProvenanceTimestamp::new(
                20,
                0,
            )
            .expect(
                "timestamp",
            );

        let completed =
            ProvenanceTimestamp::new(
                10,
                0,
            )
            .expect(
                "timestamp",
            );

        let record =
            provenance()
                .with_created_at(
                    created,
                )
                .with_completed_at(
                    completed,
                );

        assert!(
            record.validate().is_err()
        );
    }

    #[test]
    fn invalid_nanoseconds_are_rejected() {
        assert!(
            ProvenanceTimestamp::new(
                0,
                1_000_000_000,
            )
            .is_err()
        );
    }

    #[test]
    fn mapping_lookup_is_available() {
        let mut record =
            provenance();

        record
            .add_mapping(
                QubitMappingRecord::new(
                    QubitId::new(7),
                    PhysicalQubitId::new(21),
                ),
            )
            .expect("mapping");

        assert_eq!(
            record.physical_for_logical(
                QubitId::new(7),
            ),
            Some(
                PhysicalQubitId::new(21)
            )
        );

        assert_eq!(
            record.logical_for_physical(
                PhysicalQubitId::new(21),
            ),
            Some(QubitId::new(7))
        );
    }

    #[test]
    fn operation_lookup_is_available() {
        let mut record =
            provenance();

        record.add_operation(
            OperationId::new(42),
        );

        assert!(
            record.contains_operation(
                OperationId::new(42)
            )
        );

        assert!(
            !record.contains_operation(
                OperationId::new(43)
            )
        );
    }

    #[test]
    fn field_size_is_explicit_policy() {
        assert!(
            validate_field_size(
                "test",
                "abcd",
                4,
            )
            .is_ok()
        );

        assert!(
            validate_field_size(
                "test",
                "abcde",
                4,
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_characterization_artifact_kind_is_rejected() {
        let mut reference =
            CharacterizationReference::new(
                "rb",
            )
            .expect(
                "characterization",
            );

        let artifact =
            ArtifactReference::new(
                ArtifactKind::NoiseModel,
                "wrong-kind",
            )
            .expect("artifact");

        assert!(
            reference
                .set_artifact(artifact)
                .is_err()
        );
    }

    #[test]
    fn invalid_noise_model_artifact_kind_is_rejected() {
        let mut reference =
            NoiseModelReference::new(
                "model",
            )
            .expect("model");

        let artifact =
            ArtifactReference::new(
                ArtifactKind::Calibration,
                "wrong-kind",
            )
            .expect("artifact");

        assert!(
            reference
                .set_artifact(artifact)
                .is_err()
        );
    }

    #[test]
    fn builder_produces_valid_provenance() {
        let tool =
            ToolIdentity::new(
                "zamani-zqn",
                "1.0.0",
            )
            .expect("tool");

        let source =
            SourceReference::new(
                "program.zm",
            )
            .expect("source");

        let model =
            NoiseModelReference::new(
                "example-noise-model",
            )
            .expect("model");

        let target =
            TargetReference::new(
                "abstract-target",
            )
            .expect("target");

        let result =
            ProvenanceBuilder::for_program(
                ProvenanceId::new(10),
                ProgramId::new(20),
                IrVersion::CURRENT,
            )
            .source(source)
            .tool(tool)
            .model(model)
            .target(target)
            .operation(
                OperationId::new(1),
            )
            .deterministic_context(
                DeterministicContext::new(
                    123,
                ),
            )
            .metadata(
                "purpose",
                "production",
            )
            .expect("metadata")
            .build()
            .expect(
                "valid provenance",
            );

        assert_eq!(
            result.program_id(),
            Some(
                ProgramId::new(20)
            )
        );

        assert_eq!(
            result.operation_count(),
            1
        );

        assert_eq!(
            result.metadata().get(
                "purpose",
            ),
            Some("production")
        );

        assert!(
            result.is_deterministic()
        );
    }

    #[test]
    fn program_identity_uses_canonical_ir_identity() {
        let result =
            Provenance::for_program(
                ProvenanceId::new(1),
                ProgramId::new(2),
                IrVersion::CURRENT,
            );

        assert_eq!(
            result.program_id(),
            Some(
                ProgramId::new(2)
            )
        );
    }

    #[test]
    fn current_schema_is_supported() {
        let record =
            provenance();

        assert_eq!(
            record.schema_version(),
            ZQN_PROVENANCE_SCHEMA_VERSION
        );

        assert!(
            record.validate().is_ok()
        );
    }
}