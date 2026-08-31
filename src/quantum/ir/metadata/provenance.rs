//! Zamani Quantum IR — Metadata / Provenance
//!
//! Production-grade provenance and lineage model for the canonical Zamani
//! Quantum Intermediate Representation.
//!
//! # Purpose
//!
//! This module records the lineage of semantic IR artifacts across the Zamani
//! quantum compilation pipeline.
//!
//! Provenance answers:
//!
//! > Where did this artifact come from, what deterministic transformations
//! > produced it, and what external target/execution artifacts are associated
//! > with it?
//!
//! Provenance does NOT:
//!
//! - execute quantum programs;
//! - schedule programs;
//! - route qubits;
//! - optimize programs;
//! - select hardware;
//! - store hardware topology;
//! - store calibration payloads;
//! - contain credentials or secrets;
//! - represent simulator state;
//! - replace source maps;
//! - replace compiler logs;
//! - perform hashing;
//! - perform digital signing.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! frontend
//!       |
//!       v
//! canonical Zamani Quantum IR
//!       |
//!       +------------------+------------------+
//!       |                  |                  |
//!       v                  v                  v
//! optimization          routing           scheduling
//!       |                  |                  |
//!       +------------------+------------------+
//!                          |
//!                          v
//!                       hardware
//!                          |
//!                          v
//!                       backend
//!                          |
//!                          v
//!                      execution
//! ```
//!
//! Provenance may describe each transition, but it never owns the transition.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are target-independent.
//!
//! The same semantic program must be capable of being lowered to a compatible
//! target ranging from a very small machine to an arbitrarily large finite
//! machine, limited only by available resources, explicit policy limits,
//! integer/address-space constraints, target capabilities, and execution
//! infrastructure.
//!
//! This module therefore contains NO fixed quantum-machine size.
//!
//! In particular, there is no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum mapping size;
//! - fixed topology size;
//! - vendor-specific resource limit.
//!
//! Collection sizes are dynamic.
//!
//! # Canonical qubit identity boundary
//!
//! Logical and physical qubit identities are owned exclusively by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module deliberately does not define replacement qubit identifiers.
//!
//! # Determinism
//!
//! Provenance contains two fundamentally different classes of information:
//!
//! ```text
//! semantic provenance
//!     |
//!     +-- deterministic
//!     +-- suitable for reproducibility
//!     +-- suitable for canonical serialization
//!     +-- suitable for inclusion in semantic fingerprints
//!
//! observational provenance
//!     |
//!     +-- timestamps
//!     +-- execution IDs
//!     +-- host observations
//!     +-- runtime job information
//!     +-- deployment observations
//! ```
//!
//! Observational information must never automatically contaminate semantic
//! content identity.
//!
//! # Security
//!
//! Provenance is not authentication.
//!
//! It must never contain:
//!
//! - passwords;
//! - API keys;
//! - access tokens;
//! - private keys;
//! - signing keys;
//! - credentials;
//! - secret calibration payloads.
//!
//! A content digest is an identity/reference mechanism, not proof of ownership
//! or authenticity. Digital signatures belong to a separate security layer.
//!
//! # Scalability
//!
//! This module uses dynamically sized collections and does not impose semantic
//! limits on the number of:
//!
//! - transformations;
//! - mappings;
//! - operations;
//! - calibration references;
//! - metadata entries;
//! - artifacts.
//!
//! Practical limits belong to the explicit IR limits/policy subsystem.
//!
//! # Dependency boundary
//!
//! This module may depend on stable identity and qubit primitives.
//!
//! It must not depend on:
//!
//! - frontend;
//! - optimizer;
//! - routing implementation;
//! - scheduler implementation;
//! - hardware implementation;
//! - backend implementation;
//! - simulator implementation;
//! - QEC implementation.
//!
//! Downstream systems may depend on this module.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! The module explicitly forbids unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::quantum::ir::identity::{
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

/// Current provenance schema version.
///
/// This is intentionally independent from `IrVersion`.
pub const PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Number of bytes in the currently supported digest representation.
pub const DIGEST_BYTES: usize = 32;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by provenance construction or validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    /// A required field is empty.
    EmptyField {
        /// Field name.
        field: &'static str,
    },

    /// A field exceeds a caller-selected policy limit.
    FieldTooLarge {
        /// Field name.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Maximum permitted UTF-8 byte length.
        maximum_bytes: usize,
    },

    /// A digest does not have the expected representation length.
    InvalidDigestLength {
        /// Actual byte/string length.
        actual: usize,

        /// Expected length.
        expected: usize,
    },

    /// A provenance relationship is invalid.
    InvalidRelationship {
        /// Human-readable explanation.
        message: String,
    },

    /// The provenance schema is newer than this implementation supports.
    UnsupportedSchemaVersion {
        /// Encountered schema.
        found: u16,

        /// Highest supported schema.
        supported: u16,
    },

    /// An observational timestamp is malformed.
    InvalidTimestamp {
        /// Seconds component.
        seconds: u64,

        /// Nanoseconds component.
        nanoseconds: u32,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "provenance field `{field}` must not be empty"
                )
            }

            Self::FieldTooLarge {
                field,
                actual_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "provenance field `{field}` is {actual_bytes} bytes; \
                     maximum is {maximum_bytes}"
                )
            }

            Self::InvalidDigestLength { actual, expected } => {
                write!(
                    formatter,
                    "invalid provenance digest length {actual}; \
                     expected {expected}"
                )
            }

            Self::InvalidRelationship { message } => {
                write!(
                    formatter,
                    "invalid provenance relationship: {message}"
                )
            }

            Self::UnsupportedSchemaVersion {
                found,
                supported,
            } => {
                write!(
                    formatter,
                    "unsupported provenance schema version {found}; \
                     supported through {supported}"
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
        }
    }
}

impl std::error::Error for ProvenanceError {}

// =============================================================================
// Digest
// =============================================================================

/// Fixed-size digest stored by provenance.
///
/// The hashing subsystem determines the actual cryptographic hashing process.
/// Provenance only stores the resulting content identity.
///
/// The current canonical Zamani hash contract uses SHA-256.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceDigest {
    bytes: [u8; DIGEST_BYTES],
}

impl ProvenanceDigest {
    /// Creates a digest from exactly 32 bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; DIGEST_BYTES]) -> Self {
        Self { bytes }
    }

    /// Returns the digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; DIGEST_BYTES] {
        &self.bytes
    }

    /// Copies the digest bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; DIGEST_BYTES] {
        self.bytes
    }

    /// Parses a hexadecimal digest.
    ///
    /// Both uppercase and lowercase hexadecimal characters are accepted.
    pub fn from_hex(value: &str) -> Result<Self, ProvenanceError> {
        if value.len() != DIGEST_BYTES * 2 {
            return Err(ProvenanceError::InvalidDigestLength {
                actual: value.len(),
                expected: DIGEST_BYTES * 2,
            });
        }

        let bytes = value.as_bytes();
        let mut output = [0u8; DIGEST_BYTES];

        let mut index = 0usize;

        while index < DIGEST_BYTES {
            let high = hex_value(bytes[index * 2]).ok_or_else(|| {
                ProvenanceError::InvalidRelationship {
                    message: format!(
                        "invalid hexadecimal digest character at position {}",
                        index * 2
                    ),
                }
            })?;

            let low = hex_value(bytes[index * 2 + 1]).ok_or_else(|| {
                ProvenanceError::InvalidRelationship {
                    message: format!(
                        "invalid hexadecimal digest character at position {}",
                        index * 2 + 1
                    ),
                }
            })?;

            output[index] = (high << 4) | low;

            index += 1;
        }

        Ok(Self::from_bytes(output))
    }

    /// Returns lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(DIGEST_BYTES * 2);

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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Algorithm associated with a provenance digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DigestAlgorithm {
    /// SHA-256.
    Sha256,
}

impl DigestAlgorithm {
    /// Stable numeric identifier for serialization.
    #[must_use]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Stable algorithm name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
        }
    }
}

impl fmt::Display for DigestAlgorithm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Content reference
// =============================================================================

/// Content-addressed reference to another artifact.
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

    /// Creates a SHA-256 reference.
    #[must_use]
    pub const fn sha256(digest: ProvenanceDigest) -> Self {
        Self::new(DigestAlgorithm::Sha256, digest)
    }

    /// Returns the algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> DigestAlgorithm {
        self.algorithm
    }

    /// Returns the digest.
    #[must_use]
    pub const fn digest(&self) -> ProvenanceDigest {
        self.digest
    }
}

impl fmt::Display for ContentReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.algorithm,
            self.digest
        )
    }
}

// =============================================================================
// Artifact kinds
// =============================================================================

/// Semantic category of an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactKind {
    /// Complete quantum program.
    Program,

    /// Quantum circuit.
    Circuit,

    /// IR operation.
    Operation,

    /// Source artifact.
    Source,

    /// Compiler/toolchain.
    Compiler,

    /// Generic transformation.
    Transformation,

    /// Routing artifact.
    Routing,

    /// Mapping artifact.
    Mapping,

    /// Schedule artifact.
    Schedule,

    /// Hardware target descriptor.
    HardwareTarget,

    /// Calibration reference.
    Calibration,

    /// Backend artifact.
    BackendArtifact,

    /// Execution result.
    ExecutionResult,

    /// Benchmark artifact.
    Benchmark,

    /// Other extensible artifact.
    Other,
}

// =============================================================================
// Artifact reference
// =============================================================================

/// Stable reference to an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactReference {
    kind: ArtifactKind,
    name: String,
    content: Option<ContentReference>,
}

impl ArtifactReference {
    /// Creates an artifact reference.
    pub fn new(
        kind: ArtifactKind,
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty("artifact.name", &name)?;

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
        let mut reference = Self::new(kind, name)?;
        reference.content = Some(content);
        Ok(reference)
    }

    /// Returns the artifact kind.
    #[must_use]
    pub const fn kind(&self) -> ArtifactKind {
        self.kind
    }

    /// Returns the artifact name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the content identity.
    #[must_use]
    pub const fn content(&self) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Attaches content identity.
    pub fn set_content(&mut self, content: ContentReference) {
        self.content = Some(content);
    }
}

// =============================================================================
// Source reference
// =============================================================================

/// Identifies the source artifact from which IR originated.
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
    pub fn new(name: impl Into<String>) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty("source.name", &name)?;

        Ok(Self {
            name,
            content: None,
            language: None,
            language_version: None,
            revision: None,
            location: None,
        })
    }

    /// Sets source content identity.
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
        validate_non_empty("source.language", &language)?;
        self.language = Some(language);
        Ok(())
    }

    /// Sets source-language version.
    pub fn set_language_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();
        validate_non_empty("source.language_version", &version)?;
        self.language_version = Some(version);
        Ok(())
    }

    /// Sets source revision.
    pub fn set_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let revision = revision.into();
        validate_non_empty("source.revision", &revision)?;
        self.revision = Some(revision);
        Ok(())
    }

    /// Sets source location.
    pub fn set_location(
        &mut self,
        location: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let location = location.into();
        validate_non_empty("source.location", &location)?;
        self.location = Some(location);
        Ok(())
    }

    /// Returns source name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns source content identity.
    #[must_use]
    pub const fn content(&self) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Returns source language.
    #[must_use]
    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    /// Returns source-language version.
    #[must_use]
    pub fn language_version(&self) -> Option<&str> {
        self.language_version.as_deref()
    }

    /// Returns source revision.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Returns source location.
    #[must_use]
    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }
}

// =============================================================================
// Compiler identity
// =============================================================================

/// Identifies a compiler/toolchain implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompilerIdentity {
    name: String,
    version: String,
    build: Option<String>,
    source_revision: Option<String>,
}

impl CompilerIdentity {
    /// Creates a compiler identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();
        let version = version.into();

        validate_non_empty("compiler.name", &name)?;
        validate_non_empty("compiler.version", &version)?;

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
        validate_non_empty("compiler.build", &build)?;
        self.build = Some(build);
        Ok(())
    }

    /// Sets compiler source revision.
    pub fn set_source_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let revision = revision.into();
        validate_non_empty("compiler.source_revision", &revision)?;
        self.source_revision = Some(revision);
        Ok(())
    }

    /// Returns compiler name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns compiler version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns build identity.
    #[must_use]
    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    /// Returns compiler source revision.
    #[must_use]
    pub fn source_revision(&self) -> Option<&str> {
        self.source_revision.as_deref()
    }
}

// =============================================================================
// Transformation
// =============================================================================

/// Semantic category of a transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationKind {
    /// Frontend lowering.
    Lowering,

    /// Canonicalization.
    Canonicalization,

    /// Optimization.
    Optimization,

    /// Decomposition.
    Decomposition,

    /// Synthesis.
    Synthesis,

    /// Routing.
    Routing,

    /// Mapping.
    Mapping,

    /// Scheduling.
    Scheduling,

    /// Pulse lowering.
    PulseLowering,

    /// Hardware lowering.
    HardwareLowering,

    /// Fault-tolerant transformation.
    ErrorCorrection,

    /// Backend lowering.
    BackendLowering,

    /// Serialization transformation.
    Serialization,

    /// User-defined extension.
    Extension,
}

/// A single deterministic transformation in a provenance chain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformationRecord {
    sequence: u64,
    kind: TransformationKind,
    name: String,
    version: Option<String>,
    implementation: Option<CompilerIdentity>,
    input: Option<ContentReference>,
    output: Option<ContentReference>,
    parent_operation: Option<OperationId>,
    deterministic_seed: Option<u64>,
}

impl TransformationRecord {
    /// Creates a transformation record.
    pub fn new(
        sequence: u64,
        kind: TransformationKind,
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty("transformation.name", &name)?;

        Ok(Self {
            sequence,
            kind,
            name,
            version: None,
            implementation: None,
            input: None,
            output: None,
            parent_operation: None,
            deterministic_seed: None,
        })
    }

    /// Sets transformation version.
    pub fn set_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let version = version.into();
        validate_non_empty("transformation.version", &version)?;
        self.version = Some(version);
        Ok(())
    }

    /// Associates an implementation.
    #[must_use]
    pub fn with_implementation(
        mut self,
        implementation: CompilerIdentity,
    ) -> Self {
        self.implementation = Some(implementation);
        self
    }

    /// Records the input content.
    #[must_use]
    pub fn with_input(
        mut self,
        input: ContentReference,
    ) -> Self {
        self.input = Some(input);
        self
    }

    /// Records the output content.
    #[must_use]
    pub fn with_output(
        mut self,
        output: ContentReference,
    ) -> Self {
        self.output = Some(output);
        self
    }

    /// Associates an operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.parent_operation = Some(operation);
        self
    }

    /// Records a deterministic seed.
    #[must_use]
    pub fn with_deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.deterministic_seed = Some(seed);
        self
    }

    /// Returns sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns transformation kind.
    #[must_use]
    pub const fn kind(&self) -> TransformationKind {
        self.kind
    }

    /// Returns name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns implementation.
    #[must_use]
    pub const fn implementation(&self) -> Option<&CompilerIdentity> {
        self.implementation.as_ref()
    }

    /// Returns input content.
    #[must_use]
    pub const fn input(&self) -> Option<&ContentReference> {
        self.input.as_ref()
    }

    /// Returns output content.
    #[must_use]
    pub const fn output(&self) -> Option<&ContentReference> {
        self.output.as_ref()
    }

    /// Returns associated operation.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.parent_operation
    }

    /// Returns deterministic seed.
    #[must_use]
    pub const fn deterministic_seed(&self) -> Option<u64> {
        self.deterministic_seed
    }
}

// =============================================================================
// Qubit mapping provenance
// =============================================================================

/// Records one logical-to-physical relationship.
///
/// This is provenance evidence only.
///
/// It does NOT perform routing or establish that either qubit exists on a
/// target machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitMappingRecord {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitMappingRecord {
    /// Creates a mapping record.
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
    pub const fn logical(&self) -> QubitId {
        self.logical
    }

    /// Returns physical qubit.
    #[must_use]
    pub const fn physical(&self) -> PhysicalQubitId {
        self.physical
    }
}

// =============================================================================
// Target reference
// =============================================================================

/// Reference to a compilation/execution target.
///
/// This is intentionally descriptive rather than a hardware model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetReference {
    name: String,
    version: Option<String>,
    descriptor: Option<ContentReference>,
}

impl TargetReference {
    /// Creates a target reference.
    pub fn new(name: impl Into<String>) -> Result<Self, ProvenanceError> {
        let name = name.into();

        validate_non_empty("target.name", &name)?;

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
        validate_non_empty("target.version", &version)?;
        self.version = Some(version);
        Ok(())
    }

    /// Associates target descriptor content.
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
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns target version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns target descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> Option<&ContentReference> {
        self.descriptor.as_ref()
    }
}

// =============================================================================
// Calibration reference
// =============================================================================

/// Reference to calibration information.
///
/// The calibration payload itself remains outside canonical IR provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalibrationReference {
    id: CalibrationId,
    name: Option<String>,
    content: Option<ContentReference>,
}

impl CalibrationReference {
    /// Creates a calibration reference.
    #[must_use]
    pub const fn new(id: CalibrationId) -> Self {
        Self {
            id,
            name: None,
            content: None,
        }
    }

    /// Sets a human-readable name.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let name = name.into();
        validate_non_empty("calibration.name", &name)?;
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
    pub const fn id(&self) -> CalibrationId {
        self.id
    }

    /// Returns calibration name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns calibration content.
    #[must_use]
    pub const fn content(&self) -> Option<&ContentReference> {
        self.content.as_ref()
    }
}

// =============================================================================
// Execution reference
// =============================================================================

/// External execution identity.
///
/// This intentionally contains references rather than execution state.
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

    /// Sets an external job ID.
    pub fn set_job_id(
        &mut self,
        job_id: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let job_id = job_id.into();
        validate_non_empty("execution.job_id", &job_id)?;
        self.job_id = Some(job_id);
        Ok(())
    }

    /// Sets execution attempt.
    #[must_use]
    pub fn with_attempt(
        mut self,
        attempt: u64,
    ) -> Self {
        self.attempt = Some(attempt);
        self
    }

    /// Associates execution-result content.
    #[must_use]
    pub fn with_result(
        mut self,
        result: ContentReference,
    ) -> Self {
        self.result = Some(result);
        self
    }

    /// Returns job ID.
    #[must_use]
    pub fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Returns attempt.
    #[must_use]
    pub const fn attempt(&self) -> Option<u64> {
        self.attempt
    }

    /// Returns result content.
    #[must_use]
    pub const fn result(&self) -> Option<&ContentReference> {
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
/// This is observational data and is never semantic by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceTimestamp {
    seconds_since_unix_epoch: u64,
    nanoseconds: u32,
}

impl ProvenanceTimestamp {
    /// Creates a timestamp.
    pub const fn new(
        seconds_since_unix_epoch: u64,
        nanoseconds: u32,
    ) -> Result<Self, ProvenanceError> {
        if nanoseconds >= 1_000_000_000 {
            return Err(ProvenanceError::InvalidTimestamp {
                seconds: seconds_since_unix_epoch,
                nanoseconds,
            });
        }

        Ok(Self {
            seconds_since_unix_epoch,
            nanoseconds,
        })
    }

    /// Captures current system time.
    ///
    /// This must only be used for observational provenance.
    pub fn now() -> Result<Self, ProvenanceError> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ProvenanceError::InvalidRelationship {
                message: error.to_string(),
            })?;

        Self::new(
            duration.as_secs(),
            duration.subsec_nanos(),
        )
    }

    /// Returns seconds.
    #[must_use]
    pub const fn seconds(&self) -> u64 {
        self.seconds_since_unix_epoch
    }

    /// Returns nanoseconds.
    #[must_use]
    pub const fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
}

impl fmt::Display for ProvenanceTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}Z",
            self.seconds_since_unix_epoch,
            self.nanoseconds
        )
    }
}

// =============================================================================
// Deterministic metadata
// =============================================================================

/// Deterministically ordered metadata.
///
/// `BTreeMap` is intentional. Provenance serialization must never depend on
/// hash-map iteration order.
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

        validate_non_empty("metadata.key", &key)?;

        self.entries.insert(key, value.into());

        Ok(())
    }

    /// Removes a metadata entry.
    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.entries.remove(key)
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns deterministic key/value iteration order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(key, value)| {
                (key.as_str(), value.as_str())
            })
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Complete provenance record for a Quantum IR artifact.
///
/// The structure deliberately separates:
///
/// - semantic lineage;
/// - target references;
/// - mapping evidence;
/// - calibration references;
/// - operation membership;
/// - execution observations;
/// - timestamps;
/// - deterministic metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Provenance {
    schema_version: u16,
    id: ProvenanceId,
    ir_version: IrVersion,
    program_id: ProgramId,

    source: Option<SourceReference>,
    compiler: Option<CompilerIdentity>,

    input_artifact: Option<ContentReference>,
    output_artifact: Option<ContentReference>,

    transformations: Vec<TransformationRecord>,

    target: Option<TargetReference>,
    calibrations: Vec<CalibrationReference>,
    mappings: Vec<QubitMappingRecord>,
    operations: Vec<OperationId>,

    execution: Option<ExecutionReference>,

    deterministic_seed: Option<u64>,

    created_at: Option<ProvenanceTimestamp>,
    completed_at: Option<ProvenanceTimestamp>,

    metadata: ProvenanceMetadata,
}

impl Provenance {
    /// Creates a provenance record.
    #[must_use]
    pub const fn new(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            schema_version: PROVENANCE_SCHEMA_VERSION,
            id,
            ir_version,
            program_id,
            source: None,
            compiler: None,
            input_artifact: None,
            output_artifact: None,
            transformations: Vec::new(),
            target: None,
            calibrations: Vec::new(),
            mappings: Vec::new(),
            operations: Vec::new(),
            execution: None,
            deterministic_seed: None,
            created_at: None,
            completed_at: None,
            metadata: ProvenanceMetadata::new(),
        }
    }

    /// Returns current-schema provenance.
    #[must_use]
    pub const fn current(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        Self::new(id, program_id, ir_version)
    }

    /// Returns schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns provenance identity.
    #[must_use]
    pub const fn id(&self) -> ProvenanceId {
        self.id
    }

    /// Returns associated program identity.
    #[must_use]
    pub const fn program_id(&self) -> ProgramId {
        self.program_id
    }

    /// Returns IR version.
    #[must_use]
    pub const fn ir_version(&self) -> IrVersion {
        self.ir_version
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

    /// Sets compiler identity.
    #[must_use]
    pub fn with_compiler(
        mut self,
        compiler: CompilerIdentity,
    ) -> Self {
        self.compiler = Some(compiler);
        self
    }

    /// Sets input artifact.
    #[must_use]
    pub fn with_input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.input_artifact = Some(artifact);
        self
    }

    /// Sets output artifact.
    #[must_use]
    pub fn with_output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.output_artifact = Some(artifact);
        self
    }

    /// Sets target.
    #[must_use]
    pub fn with_target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets deterministic seed.
    #[must_use]
    pub fn with_deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.deterministic_seed = Some(seed);
        self
    }

    /// Sets creation timestamp.
    ///
    /// This makes provenance observational.
    #[must_use]
    pub fn with_created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.created_at = Some(timestamp);
        self
    }

    /// Sets completion timestamp.
    ///
    /// This makes provenance observational.
    #[must_use]
    pub fn with_completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.completed_at = Some(timestamp);
        self
    }

    /// Sets execution reference.
    ///
    /// This makes provenance observational.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.execution = Some(execution);
        self
    }

    /// Adds a transformation.
    ///
    /// Sequence numbers must be strictly increasing.
    pub fn add_transformation(
        &mut self,
        record: TransformationRecord,
    ) -> Result<(), ProvenanceError> {
        if let Some(previous) = self.transformations.last() {
            if record.sequence() <= previous.sequence() {
                return Err(ProvenanceError::InvalidRelationship {
                    message: format!(
                        "transformation sequence {} is not greater than {}",
                        record.sequence(),
                        previous.sequence()
                    ),
                });
            }
        }

        self.transformations.push(record);

        Ok(())
    }

    /// Adds a calibration reference.
    ///
    /// Exact duplicate references are ignored.
    pub fn add_calibration(
        &mut self,
        calibration: CalibrationReference,
    ) {
        if !self.calibrations.contains(&calibration) {
            self.calibrations.push(calibration);

            self.calibrations.sort_by_key(|entry| {
                (
                    entry.id(),
                    entry.name().unwrap_or(""),
                )
            });
        }
    }

    /// Adds a logical-to-physical mapping.
    ///
    /// Each logical qubit may have at most one mapping and each physical qubit
    /// may have at most one logical mapping within one provenance snapshot.
    pub fn add_mapping(
        &mut self,
        mapping: QubitMappingRecord,
    ) -> Result<(), ProvenanceError> {
        match self.mappings.binary_search_by_key(
            &mapping.logical(),
            |entry| entry.logical(),
        ) {
            Ok(index) => {
                let existing = self.mappings[index];

                if existing.physical() == mapping.physical() {
                    return Ok(());
                }

                return Err(ProvenanceError::InvalidRelationship {
                    message: format!(
                        "logical qubit {} is already mapped to {}",
                        existing.logical(),
                        existing.physical()
                    ),
                });
            }

            Err(index) => {
                for existing in &self.mappings {
                    if existing.physical() == mapping.physical() {
                        return Err(
                            ProvenanceError::InvalidRelationship {
                                message: format!(
                                    "physical qubit {} is already mapped",
                                    mapping.physical()
                                ),
                            },
                        );
                    }
                }

                self.mappings.insert(index, mapping);
            }
        }

        Ok(())
    }

    /// Records an operation.
    ///
    /// Operations are stored in deterministic identity order.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) {
        match self.operations.binary_search(&operation) {
            Ok(_) => {}
            Err(index) => {
                self.operations.insert(index, operation);
            }
        }
    }

    /// Adds deterministic metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        self.metadata.insert(key, value)
    }

    /// Returns source.
    #[must_use]
    pub const fn source(&self) -> Option<&SourceReference> {
        self.source.as_ref()
    }

    /// Returns compiler.
    #[must_use]
    pub const fn compiler(&self) -> Option<&CompilerIdentity> {
        self.compiler.as_ref()
    }

    /// Returns input artifact.
    #[must_use]
    pub const fn input_artifact(&self) -> Option<&ContentReference> {
        self.input_artifact.as_ref()
    }

    /// Returns output artifact.
    #[must_use]
    pub const fn output_artifact(&self) -> Option<&ContentReference> {
        self.output_artifact.as_ref()
    }

    /// Returns transformations.
    #[must_use]
    pub fn transformations(&self) -> &[TransformationRecord] {
        &self.transformations
    }

    /// Returns target.
    #[must_use]
    pub const fn target(&self) -> Option<&TargetReference> {
        self.target.as_ref()
    }

    /// Returns calibrations.
    #[must_use]
    pub fn calibrations(&self) -> &[CalibrationReference] {
        &self.calibrations
    }

    /// Returns mappings.
    #[must_use]
    pub fn mappings(&self) -> &[QubitMappingRecord] {
        &self.mappings
    }

    /// Returns operation identities.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns execution reference.
    #[must_use]
    pub const fn execution(&self) -> Option<&ExecutionReference> {
        self.execution.as_ref()
    }

    /// Returns deterministic seed.
    #[must_use]
    pub const fn deterministic_seed(&self) -> Option<u64> {
        self.deterministic_seed
    }

    /// Returns creation timestamp.
    #[must_use]
    pub const fn created_at(&self) -> Option<ProvenanceTimestamp> {
        self.created_at
    }

    /// Returns completion timestamp.
    #[must_use]
    pub const fn completed_at(&self) -> Option<ProvenanceTimestamp> {
        self.completed_at
    }

    /// Returns deterministic metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ProvenanceMetadata {
        &self.metadata
    }

    /// Returns whether observational information exists.
    #[must_use]
    pub const fn contains_observational_data(&self) -> bool {
        self.created_at.is_some()
            || self.completed_at.is_some()
            || self.execution.is_some()
    }

    /// Returns whether this record is purely deterministic.
    ///
    /// This does not mean every referenced artifact has content identity;
    /// it only means this provenance object itself contains no observational
    /// execution/time fields.
    #[must_use]
    pub const fn is_deterministic(&self) -> bool {
        !self.contains_observational_data()
    }

    /// Validates the complete provenance object.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        if self.schema_version > PROVENANCE_SCHEMA_VERSION {
            return Err(
                ProvenanceError::UnsupportedSchemaVersion {
                    found: self.schema_version,
                    supported: PROVENANCE_SCHEMA_VERSION,
                },
            );
        }

        if let Some(source) = &self.source {
            validate_non_empty(
                "source.name",
                source.name(),
            )?;
        }

        if let Some(compiler) = &self.compiler {
            validate_non_empty(
                "compiler.name",
                compiler.name(),
            )?;

            validate_non_empty(
                "compiler.version",
                compiler.version(),
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

        let mut previous_sequence = None;

        for transformation in &self.transformations {
            if let Some(previous) = previous_sequence {
                if transformation.sequence() <= previous {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: String::from(
                                "transformation sequence is not strictly increasing",
                            ),
                        },
                    );
                }
            }

            if transformation.name().is_empty() {
                return Err(
                    ProvenanceError::EmptyField {
                        field: "transformation.name",
                    },
                );
            }

            previous_sequence =
                Some(transformation.sequence());
        }

        let mut previous_operation = None;

        for operation in &self.operations {
            if let Some(previous) = previous_operation {
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

            previous_operation = Some(*operation);
        }

        let mut previous_logical = None;
        let mut previous_physical = None;

        for mapping in &self.mappings {
            if let Some(previous) = previous_logical {
                if mapping.logical() <= previous {
                    return Err(
                        ProvenanceError::InvalidRelationship {
                            message: String::from(
                                "logical qubit mappings are not strictly ordered",
                            ),
                        },
                    );
                }
            }

            if let Some(previous) = previous_physical {
                if mapping.physical() == previous {
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

            previous_logical = Some(mapping.logical());
            previous_physical = Some(mapping.physical());
        }

        for calibration in &self.calibrations {
            if let Some(name) = calibration.name() {
                validate_non_empty(
                    "calibration.name",
                    name,
                )?;
            }
        }

        if let Some(target) = &self.target {
            validate_non_empty(
                "target.name",
                target.name(),
            )?;
        }

        Ok(())
    }

    /// Returns the number of recorded transformations.
    #[must_use]
    pub fn transformation_count(&self) -> usize {
        self.transformations.len()
    }

    /// Returns the number of mapped logical qubits.
    #[must_use]
    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }

    /// Returns the number of associated operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether an operation is recorded.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operations.binary_search(&operation).is_ok()
    }

    /// Returns whether a logical qubit has a mapping.
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
            .map(|index| self.mappings[index].physical())
    }

    /// Returns whether a physical qubit has a logical mapping.
    #[must_use]
    pub fn logical_for_physical(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.mappings
            .iter()
            .find(|entry| entry.physical() == physical)
            .map(QubitMappingRecord::logical)
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for provenance.
///
/// The builder guarantees that `build()` performs complete validation before
/// returning the final provenance object.
#[derive(Debug, Clone)]
pub struct ProvenanceBuilder {
    provenance: Provenance,
}

impl ProvenanceBuilder {
    /// Creates a builder.
    #[must_use]
    pub const fn new(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            provenance: Provenance::new(
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
        self.provenance.source = Some(source);
        self
    }

    /// Sets compiler.
    #[must_use]
    pub fn compiler(
        mut self,
        compiler: CompilerIdentity,
    ) -> Self {
        self.provenance.compiler = Some(compiler);
        self
    }

    /// Sets input artifact.
    #[must_use]
    pub fn input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.input_artifact = Some(artifact);
        self
    }

    /// Sets output artifact.
    #[must_use]
    pub fn output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.output_artifact = Some(artifact);
        self
    }

    /// Sets target.
    #[must_use]
    pub fn target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.provenance.target = Some(target);
        self
    }

    /// Adds transformation.
    pub fn transformation(
        mut self,
        transformation: TransformationRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_transformation(transformation)?;

        Ok(self)
    }

    /// Adds calibration.
    #[must_use]
    pub fn calibration(
        mut self,
        calibration: CalibrationReference,
    ) -> Self {
        self.provenance
            .add_calibration(calibration);

        self
    }

    /// Adds mapping.
    pub fn mapping(
        mut self,
        mapping: QubitMappingRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_mapping(mapping)?;

        Ok(self)
    }

    /// Adds operation.
    #[must_use]
    pub fn operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.provenance
            .add_operation(operation);

        self
    }

    /// Adds deterministic seed.
    #[must_use]
    pub fn deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.provenance
            .deterministic_seed = Some(seed);

        self
    }

    /// Adds deterministic metadata.
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_metadata(key, value)?;

        Ok(self)
    }

    /// Adds creation timestamp.
    #[must_use]
    pub fn created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance
            .created_at = Some(timestamp);

        self
    }

    /// Adds completion timestamp.
    #[must_use]
    pub fn completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance
            .completed_at = Some(timestamp);

        self
    }

    /// Adds execution reference.
    #[must_use]
    pub fn execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.provenance
            .execution = Some(execution);

        self
    }

    /// Builds and validates provenance.
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

fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Err(ProvenanceError::EmptyField { field });
    }

    Ok(())
}

/// Validates a UTF-8 field against an explicit caller-selected limit.
///
/// This helper exists for integration with the IR limits subsystem. It does
/// not establish an architectural limit.
pub fn validate_field_size(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ProvenanceError> {
    if value.len() > maximum_bytes {
        return Err(ProvenanceError::FieldTooLarge {
            field,
            actual_bytes: value.len(),
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

    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    fn provenance() -> Provenance {
        Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(2),
            IrVersion::CURRENT,
        )
    }

    #[test]
    fn digest_round_trip() {
        let mut bytes = [0u8; DIGEST_BYTES];

        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = index as u8;
        }

        let digest =
            ProvenanceDigest::from_bytes(bytes);

        let encoded = digest.to_hex();

        let decoded =
            ProvenanceDigest::from_hex(&encoded)
                .expect("digest must decode");

        assert_eq!(digest, decoded);
    }

    #[test]
    fn digest_rejects_wrong_length() {
        assert!(
            ProvenanceDigest::from_hex("00")
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
    fn compiler_requires_name_and_version() {
        assert!(
            CompilerIdentity::new(
                "",
                "1.0.0",
            )
            .is_err()
        );

        assert!(
            CompilerIdentity::new(
                "zamani",
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
    fn transformations_are_ordered() {
        let mut record = provenance();

        record
            .add_transformation(
                TransformationRecord::new(
                    1,
                    TransformationKind::Lowering,
                    "frontend.lower",
                )
                .expect("valid transformation"),
            )
            .expect("first transformation");

        record
            .add_transformation(
                TransformationRecord::new(
                    2,
                    TransformationKind::Optimization,
                    "optimize",
                )
                .expect("valid transformation"),
            )
            .expect("second transformation");

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
        let mut record = provenance();

        record
            .add_transformation(
                TransformationRecord::new(
                    1,
                    TransformationKind::Lowering,
                    "lower",
                )
                .expect("valid transformation"),
            )
            .expect("first transformation");

        assert!(
            record
                .add_transformation(
                    TransformationRecord::new(
                        1,
                        TransformationKind::Optimization,
                        "optimize",
                    )
                    .expect("valid transformation"),
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
        let mut record = provenance();

        record
            .add_mapping(
                QubitMappingRecord::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
            )
            .expect("mapping");

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
        let mut record = provenance();

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
            .expect("identical mapping is harmless");

        assert_eq!(
            record.mapping_count(),
            1
        );
    }

    #[test]
    fn operation_ids_are_sorted_and_unique() {
        let mut record = provenance();

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
    fn metadata_is_deterministic() {
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
            Some("new")
        );

        assert_eq!(
            metadata.len(),
            1
        );
    }

    #[test]
    fn deterministic_provenance_has_no_observations() {
        let record = provenance();

        assert!(
            record.is_deterministic()
        );

        assert!(
            !record.contains_observational_data()
        );
    }

    #[test]
    fn timestamps_make_provenance_observational() {
        let timestamp =
            ProvenanceTimestamp::new(
                100,
                0,
            )
            .expect("valid timestamp");

        let record =
            provenance()
                .with_created_at(timestamp);

        assert!(
            !record.is_deterministic()
        );

        assert!(
            record.contains_observational_data()
        );
    }

    #[test]
    fn timestamps_are_order_checked() {
        let created =
            ProvenanceTimestamp::new(
                20,
                0,
            )
            .expect("timestamp");

        let completed =
            ProvenanceTimestamp::new(
                10,
                0,
            )
            .expect("timestamp");

        let record =
            provenance()
                .with_created_at(created)
                .with_completed_at(completed);

        assert!(
            record.validate().is_err()
        );
    }

    #[test]
    fn timestamp_rejects_invalid_nanoseconds() {
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
        let mut record = provenance();

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
        let mut record = provenance();

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
    fn field_size_validation_is_explicit_policy() {
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
    fn builder_produces_valid_provenance() {
        let compiler =
            CompilerIdentity::new(
                "zamani",
                "1.0.0",
            )
            .expect("compiler");

        let source =
            SourceReference::new(
                "main.zm",
            )
            .expect("source");

        let target =
            TargetReference::new(
                "abstract-target",
            )
            .expect("target");

        let result =
            ProvenanceBuilder::new(
                ProvenanceId::new(10),
                ProgramId::new(20),
                IrVersion::CURRENT,
            )
            .source(source)
            .compiler(compiler)
            .target(target)
            .operation(
                OperationId::new(1),
            )
            .metadata(
                "purpose",
                "production",
            )
            .expect("metadata")
            .build()
            .expect("valid provenance");

        assert_eq!(
            result.program_id(),
            ProgramId::new(20)
        );

        assert_eq!(
            result.operation_count(),
            1
        );

        assert_eq!(
            result.metadata().get("purpose"),
            Some("production")
        );
    }
}