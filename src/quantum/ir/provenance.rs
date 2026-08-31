//! Zamani Quantum IR — Provenance
//!
//! Production-grade, deterministic provenance representation for the canonical
//! Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `provenance.rs` records the lineage and reproducibility metadata of an IR
//! artifact without becoming a compiler log, hardware database, scheduler,
//! optimizer, runtime trace, or source-language AST.
//!
//! The canonical dependency direction is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend/compiler
//!      |
//!      v
//! quantum::ir
//!      |
//!      +--------------------+-------------------+
//!      |                    |                   |
//!      v                    v                   v
//! optimization          routing             scheduling
//!      |                    |                   |
//!      +--------------------+-------------------+
//!                           |
//!                           v
//!                        hardware
//!                           |
//!                           v
//!                         backend
//! ```
//!
//! Provenance records the identity of the artifacts and transformations that
//! occur along that pipeline. It does not execute those transformations.
//!
//! # Core principle
//!
//! A quantum program must be reproducible independently of the machine on which
//! it is eventually executed.
//!
//! Provenance therefore distinguishes:
//!
//! ```text
//! semantic identity
//!     !=
//! source identity
//!     !=
//! compiler identity
//!     !=
//! transformation identity
//!     !=
//! target identity
//!     !=
//! calibration identity
//!     !=
//! execution identity
//! ```
//!
//! These identities may be related by provenance, but must never be conflated.
//!
//! # Universal-program principle
//!
//! Zamani is intended to allow one quantum program to target machines ranging
//! from tiny systems to arbitrarily large finite systems subject only to the
//! resources and capabilities available to the selected target.
//!
//! Therefore this module deliberately contains no architectural quantum-size
//! limit.
//!
//! In particular, this module does NOT use:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as quantum-machine limits.
//!
//! Qubit identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Provenance merely records those identities when a transformation actually
//! needs to document them.
//!
//! # Determinism
//!
//! Provenance has two different classes of information:
//!
//! 1. deterministic semantic provenance;
//! 2. observational/runtime provenance.
//!
//! Deterministic semantic provenance includes:
//!
//! - IR version;
//! - program identity;
//! - source identity;
//! - compiler identity;
//! - transformation identifiers;
//! - optimization pass identifiers;
//! - mapping references;
//! - schedule references;
//! - target descriptors;
//! - calibration references;
//! - content digests;
//! - deterministic seeds.
//!
//! Runtime/observational provenance may include:
//!
//! - wall-clock timestamps;
//! - execution identifiers;
//! - host information;
//! - backend job identifiers.
//!
//! Such observational data must never be included automatically in canonical
//! semantic hashing unless the caller explicitly chooses to hash it.
//!
//! # Security
//!
//! Provenance is not an authorization mechanism.
//!
//! It does not contain:
//!
//! - private keys;
//! - passwords;
//! - access tokens;
//! - API credentials;
//! - secrets;
//! - authentication material.
//!
//! Content digests are references to content identity, not signatures.
//! Authenticity is provided by a separate signing/security subsystem.
//!
//! # Scalability
//!
//! The representation uses dynamically sized collections rather than fixed-size
//! arrays for lineage records.
//!
//! This allows:
//!
//! - tiny programs;
//! - large programs;
//! - distributed compilation;
//! - very long optimization pipelines;
//! - many target mappings;
//! - many calibration references;
//! - large transformation histories.
//!
//! Practical limits are imposed by the explicit IR resource-policy layer and
//! by available memory/storage, not by this module.
//!
//! # Qubit integration
//!
//! This module intentionally imports the canonical types from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! It never defines duplicate logical or physical qubit identifiers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no unsafe code;
//! - standard library only;
//! - deterministic data structures where ordering matters.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use super::identity::{
    CalibrationId,
    IrVersion,
    OperationId,
    ProgramId,
    ProvenanceId,
};

use super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Constants
// =============================================================================

/// Current provenance schema version.
///
/// This is independent from the Quantum IR semantic version.
pub const PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Fixed digest size used by the IR hashing contract.
///
/// SHA-256 is currently the canonical hash algorithm used by `hash.rs`.
///
/// This module intentionally stores digests as raw fixed-size bytes so it does
/// not create a circular dependency on the hashing implementation.
pub const DIGEST_BYTES: usize = 32;

// =============================================================================
// Provenance error
// =============================================================================

/// Errors produced while constructing or validating provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    /// A required textual field was empty.
    EmptyField {
        /// Name of the field.
        field: &'static str,
    },

    /// A textual field exceeded its caller-specified maximum.
    FieldTooLarge {
        /// Name of the field.
        field: &'static str,

        /// Actual UTF-8 byte length.
        actual_bytes: usize,

        /// Maximum allowed UTF-8 byte length.
        maximum_bytes: usize,
    },

    /// A digest had an invalid length.
    InvalidDigestLength {
        /// Actual length.
        actual: usize,

        /// Required length.
        expected: usize,
    },

    /// A provenance record contains an invalid relationship.
    InvalidRelationship {
        /// Description of the invalid relationship.
        message: String,
    },

    /// A provenance record contains an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Encountered schema version.
        found: u16,

        /// Maximum supported schema version.
        supported: u16,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
                    "provenance field `{field}` is {actual_bytes} bytes; maximum is {maximum_bytes}"
                )
            }

            Self::InvalidDigestLength {
                actual,
                expected,
            } => {
                write!(
                    formatter,
                    "invalid provenance digest length {actual}; expected {expected}"
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
                    "unsupported provenance schema version {found}; supported through {supported}"
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

// =============================================================================
// Digest
// =============================================================================

/// A fixed-size content digest used by provenance.
///
/// This is intentionally algorithm-neutral at the storage level. The hashing
/// subsystem supplies the algorithm identifier separately.
///
/// The current Zamani canonical hash implementation uses SHA-256.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceDigest {
    bytes: [u8; DIGEST_BYTES],
}

impl ProvenanceDigest {
    /// Creates a digest from exactly 32 bytes.
    #[must_use]
    pub const fn from_bytes(
        bytes: [u8; DIGEST_BYTES],
    ) -> Self {
        Self { bytes }
    }

    /// Returns the raw digest bytes.
    #[must_use]
    pub const fn as_bytes(
        &self,
    ) -> &[u8; DIGEST_BYTES] {
        &self.bytes
    }

    /// Copies the digest bytes.
    #[must_use]
    pub const fn to_bytes(
        self,
    ) -> [u8; DIGEST_BYTES] {
        self.bytes
    }

    /// Parses a lowercase or uppercase hexadecimal digest.
    pub fn from_hex(
        value: &str,
    ) -> Result<Self, ProvenanceError> {
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

    /// Returns the digest as lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(
        self,
    ) -> String {
        let mut output = String::with_capacity(DIGEST_BYTES * 2);

        for byte in self.bytes {
            output.push(hex_digit(byte >> 4));
            output.push(hex_digit(byte & 0x0f));
        }

        output
    }
}

impl fmt::Debug for ProvenanceDigest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

fn hex_digit(
    value: u8,
) -> char {
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

fn hex_value(
    value: u8,
) -> Option<u8> {
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

/// Hash algorithm associated with a provenance digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DigestAlgorithm {
    /// SHA-256.
    Sha256,
}

impl DigestAlgorithm {
    /// Stable algorithm identifier.
    #[must_use]
    pub const fn id(
        self,
    ) -> u8 {
        match self {
            Self::Sha256 => 1,
        }
    }

    /// Stable algorithm name.
    #[must_use]
    pub const fn name(
        self,
    ) -> &'static str {
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

/// A content-addressed reference to an IR artifact.
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
        Self::new(DigestAlgorithm::Sha256, digest)
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

// =============================================================================
// Artifact kind
// =============================================================================

/// Kind of artifact referenced by provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactKind {
    /// Complete Quantum IR program.
    Program,

    /// Quantum circuit.
    Circuit,

    /// Individual IR operation.
    Operation,

    /// Source program/input.
    Source,

    /// Compiler/toolchain.
    Compiler,

    /// Optimization transformation.
    Transformation,

    /// Routing transformation.
    Routing,

    /// Schedule.
    Schedule,

    /// Mapping.
    Mapping,

    /// Hardware target description.
    HardwareTarget,

    /// Calibration reference.
    Calibration,

    /// Backend artifact.
    BackendArtifact,

    /// Execution result.
    ExecutionResult,

    /// Benchmark artifact.
    Benchmark,

    /// Generic IR artifact.
    Other,
}

// =============================================================================
// Artifact reference
// =============================================================================

/// Stable provenance reference to an artifact.
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

    /// Creates an artifact reference with a content digest.
    pub fn with_content(
        kind: ArtifactKind,
        name: impl Into<String>,
        content: ContentReference,
    ) -> Result<Self, ProvenanceError> {
        let mut reference = Self::new(
            kind,
            name,
        )?;

        reference.content = Some(content);

        Ok(reference)
    }

    /// Returns the artifact kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> ArtifactKind {
        self.kind
    }

    /// Returns the artifact name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns the optional content identity.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Attaches a content identity.
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

/// Identifies the source from which an IR artifact originated.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceReference {
    /// Human-readable source identifier.
    name: String,

    /// Optional source content digest.
    content: Option<ContentReference>,

    /// Optional source-language version.
    language_version: Option<String>,

    /// Optional source location/path.
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
            language_version: None,
            location: None,
        })
    }

    /// Sets the source content identity.
    #[must_use]
    pub fn with_content(
        mut self,
        content: ContentReference,
    ) -> Self {
        self.content = Some(content);
        self
    }

    /// Sets the source language version.
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

    /// Sets a source location.
    pub fn set_location(
        &mut self,
        location: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let location = location.into();

        validate_non_empty(
            "source.location",
            &location,
        )?;

        self.location = Some(location);

        Ok(())
    }

    /// Returns the source name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns the source content reference.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }

    /// Returns the source-language version.
    #[must_use]
    pub fn language_version(
        &self,
    ) -> Option<&str> {
        self.language_version.as_deref()
    }

    /// Returns the source location.
    #[must_use]
    pub fn location(
        &self,
    ) -> Option<&str> {
        self.location.as_deref()
    }
}

// =============================================================================
// Compiler identity
// =============================================================================

/// Identifies the compiler/toolchain responsible for an IR transformation.
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

        validate_non_empty(
            "compiler.name",
            &name,
        )?;

        validate_non_empty(
            "compiler.version",
            &version,
        )?;

        Ok(Self {
            name,
            version,
            build: None,
            source_revision: None,
        })
    }

    /// Sets a build identifier.
    pub fn set_build(
        &mut self,
        build: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let build = build.into();

        validate_non_empty(
            "compiler.build",
            &build,
        )?;

        self.build = Some(build);

        Ok(())
    }

    /// Sets a source revision.
    pub fn set_source_revision(
        &mut self,
        revision: impl Into<String>,
    ) -> Result<(), ProvenanceError> {
        let revision = revision.into();

        validate_non_empty(
            "compiler.source_revision",
            &revision,
        )?;

        self.source_revision = Some(revision);

        Ok(())
    }

    /// Returns the compiler name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns the compiler version.
    #[must_use]
    pub fn version(
        &self,
    ) -> &str {
        &self.version
    }

    /// Returns the optional build identifier.
    #[must_use]
    pub fn build(
        &self,
    ) -> Option<&str> {
        self.build.as_deref()
    }

    /// Returns the optional compiler source revision.
    #[must_use]
    pub fn source_revision(
        &self,
    ) -> Option<&str> {
        self.source_revision.as_deref()
    }
}

// =============================================================================
// Transformation kind
// =============================================================================

/// Category of transformation recorded in provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransformationKind {
    /// Frontend/source lowering.
    Lowering,

    /// Canonicalization.
    Canonicalization,

    /// Optimization.
    Optimization,

    /// Gate decomposition.
    Decomposition,

    /// Gate synthesis.
    Synthesis,

    /// Routing.
    Routing,

    /// Scheduling.
    Scheduling,

    /// Pulse lowering.
    PulseLowering,

    /// Hardware lowering.
    HardwareLowering,

    /// Error-correction transformation.
    ErrorCorrection,

    /// Backend lowering.
    BackendLowering,

    /// Serialization/deserialization transformation.
    Serialization,

    /// User-defined/extension transformation.
    Extension,
}

// =============================================================================
// Transformation record
// =============================================================================

/// Immutable description of one transformation stage.
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
            parent_operation: None,
            deterministic_seed: None,
        })
    }

    /// Sets the transformation implementation version.
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

    /// Sets the compiler implementation.
    #[must_use]
    pub fn with_implementation(
        mut self,
        implementation: CompilerIdentity,
    ) -> Self {
        self.implementation = Some(implementation);
        self
    }

    /// Records the input artifact content.
    #[must_use]
    pub fn with_input(
        mut self,
        input: ContentReference,
    ) -> Self {
        self.input = Some(input);
        self
    }

    /// Records the output artifact content.
    #[must_use]
    pub fn with_output(
        mut self,
        output: ContentReference,
    ) -> Self {
        self.output = Some(output);
        self
    }

    /// Associates the transformation with an operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.parent_operation = Some(operation);
        self
    }

    /// Records a deterministic transformation seed.
    ///
    /// A seed is semantic only when the transformation explicitly defines its
    /// algorithm in terms of the seed.
    #[must_use]
    pub fn with_deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.deterministic_seed = Some(seed);
        self
    }

    /// Returns the transformation sequence number.
    #[must_use]
    pub const fn sequence(
        &self,
    ) -> u64 {
        self.sequence
    }

    /// Returns the transformation kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> TransformationKind {
        self.kind
    }

    /// Returns the transformation name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns the transformation version.
    #[must_use]
    pub fn version(
        &self,
    ) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the implementation identity.
    #[must_use]
    pub const fn implementation(
        &self,
    ) -> Option<&CompilerIdentity> {
        self.implementation.as_ref()
    }

    /// Returns the input content reference.
    #[must_use]
    pub const fn input(
        &self,
    ) -> Option<&ContentReference> {
        self.input.as_ref()
    }

    /// Returns the output content reference.
    #[must_use]
    pub const fn output(
        &self,
    ) -> Option<&ContentReference> {
        self.output.as_ref()
    }

    /// Returns the associated operation.
    #[must_use]
    pub const fn operation(
        &self,
    ) -> Option<OperationId> {
        self.parent_operation
    }

    /// Returns the deterministic seed.
    #[must_use]
    pub const fn deterministic_seed(
        &self,
    ) -> Option<u64> {
        self.deterministic_seed
    }
}

// =============================================================================
// Qubit provenance
// =============================================================================

/// Records a logical-to-physical qubit relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QubitMappingRecord {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl QubitMappingRecord {
    /// Creates a logical-to-physical mapping record.
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

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(
        self,
    ) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(
        self,
    ) -> PhysicalQubitId {
        self.physical
    }
}

// =============================================================================
// Target reference
// =============================================================================

/// Hardware-independent reference to a compilation/execution target.
///
/// This intentionally does not model hardware topology, calibration data,
/// physical channels, or device internals.
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

    /// Sets the target version.
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

    /// Sets a canonical content reference for the target descriptor.
    #[must_use]
    pub fn with_descriptor(
        mut self,
        descriptor: ContentReference,
    ) -> Self {
        self.descriptor = Some(descriptor);
        self
    }

    /// Returns the target name.
    #[must_use]
    pub fn name(
        &self,
    ) -> &str {
        &self.name
    }

    /// Returns the target version.
    #[must_use]
    pub fn version(
        &self,
    ) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the target descriptor reference.
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

/// Reference to calibration information used by a downstream target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalibrationReference {
    id: CalibrationId,
    name: Option<String>,
    content: Option<ContentReference>,
}

impl CalibrationReference {
    /// Creates a calibration reference.
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

    /// Sets a human-readable calibration name.
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

    /// Associates a content digest.
    #[must_use]
    pub fn with_content(
        mut self,
        content: ContentReference,
    ) -> Self {
        self.content = Some(content);
        self
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> CalibrationId {
        self.id
    }

    /// Returns the calibration name.
    #[must_use]
    pub fn name(
        &self,
    ) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the calibration content reference.
    #[must_use]
    pub const fn content(
        &self,
    ) -> Option<&ContentReference> {
        self.content.as_ref()
    }
}

// =============================================================================
// Execution identity
// =============================================================================

/// Identifies an execution without making execution a responsibility of IR.
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

    /// Sets an external backend/runtime job identifier.
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

    /// Sets the execution attempt number.
    #[must_use]
    pub fn with_attempt(
        mut self,
        attempt: u64,
    ) -> Self {
        self.attempt = Some(attempt);
        self
    }

    /// Associates an execution-result content identity.
    #[must_use]
    pub fn with_result(
        mut self,
        result: ContentReference,
    ) -> Self {
        self.result = Some(result);
        self
    }

    /// Returns the backend/runtime job identifier.
    #[must_use]
    pub fn job_id(
        &self,
    ) -> Option<&str> {
        self.job_id.as_deref()
    }

    /// Returns the attempt number.
    #[must_use]
    pub const fn attempt(
        &self,
    ) -> Option<u64> {
        self.attempt
    }

    /// Returns the execution-result reference.
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
/// This value is observational metadata and must not be treated as semantic
/// program identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceTimestamp {
    seconds_since_unix_epoch: u64,
    nanoseconds: u32,
}

impl ProvenanceTimestamp {
    /// Creates a timestamp from Unix epoch components.
    ///
    /// `nanoseconds` must be below one second.
    pub const fn new(
        seconds_since_unix_epoch: u64,
        nanoseconds: u32,
    ) -> Result<Self, ProvenanceError> {
        if nanoseconds >= 1_000_000_000 {
            return Err(ProvenanceError::InvalidRelationship {
                message: String::new(),
            });
        }

        Ok(Self {
            seconds_since_unix_epoch,
            nanoseconds,
        })
    }

    /// Returns the Unix timestamp in seconds.
    #[must_use]
    pub const fn seconds(
        self,
    ) -> u64 {
        self.seconds_since_unix_epoch
    }

    /// Returns the fractional nanoseconds.
    #[must_use]
    pub const fn nanoseconds(
        self,
    ) -> u32 {
        self.nanoseconds
    }

    /// Captures the current system time.
    ///
    /// This is explicitly observational and should not be included in a
    /// deterministic program hash.
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
// Provenance metadata
// =============================================================================

/// Additional non-semantic provenance metadata.
///
/// This is intentionally represented as ordered key/value entries rather than
/// an unordered hash map so serialization can remain deterministic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ProvenanceMetadata {
    entries: Vec<MetadataEntry>,
}

/// A deterministic metadata key/value pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetadataEntry {
    key: String,
    value: String,
}

impl MetadataEntry {
    /// Creates a metadata entry.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_non_empty(
            "metadata.key",
            &key,
        )?;

        Ok(Self {
            key,
            value,
        })
    }

    /// Returns the key.
    #[must_use]
    pub fn key(
        &self,
    ) -> &str {
        &self.key
    }

    /// Returns the value.
    #[must_use]
    pub fn value(
        &self,
    ) -> &str {
        &self.value
    }
}

impl ProvenanceMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Inserts a metadata entry.
    ///
    /// Keys are maintained in lexicographic order. If a key already exists,
    /// its value is replaced.
    pub fn insert(
        &mut self,
        entry: MetadataEntry,
    ) {
        match self
            .entries
            .binary_search_by(|existing| existing.key.cmp(&entry.key))
        {
            Ok(index) => {
                self.entries[index] = entry;
            }

            Err(index) => {
                self.entries.insert(index, entry);
            }
        }
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.entries.len()
    }

    /// Returns whether there are no metadata entries.
    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over entries in deterministic key order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &MetadataEntry> {
        self.entries.iter()
    }

    /// Returns a metadata value by key.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.entries
            .binary_search_by(|entry| entry.key.as_str().cmp(key))
            .ok()
            .map(|index| self.entries[index].value.as_str())
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Complete provenance record for a canonical Quantum IR artifact.
///
/// `Provenance` is deliberately a data model rather than a logging system.
///
/// The record can therefore be:
///
/// - serialized;
/// - hashed;
/// - persisted;
/// - transported across compiler processes;
/// - attached to a `QuantumProgram`;
/// - consumed by benchmarking;
/// - consumed by hardware adapters;
/// - inspected by debugging tools.
///
/// It contains no execution logic.
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
    calibration: Vec<CalibrationReference>,

    mappings: Vec<QubitMappingRecord>,

    operations: Vec<OperationId>,

    execution: Option<ExecutionReference>,

    deterministic_seed: Option<u64>,

    created_at: Option<ProvenanceTimestamp>,
    completed_at: Option<ProvenanceTimestamp>,

    metadata: ProvenanceMetadata,
}

impl Provenance {
    /// Creates a new provenance record for a program.
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
            calibration: Vec::new(),
            mappings: Vec::new(),
            operations: Vec::new(),
            execution: None,
            deterministic_seed: None,
            created_at: None,
            completed_at: None,
            metadata: ProvenanceMetadata::new(),
        }
    }

    /// Returns the current provenance schema version.
    #[must_use]
    pub const fn current(
        id: ProvenanceId,
        program_id: ProgramId,
        ir_version: IrVersion,
    ) -> Self {
        Self::new(
            id,
            program_id,
            ir_version,
        )
    }

    /// Returns the provenance schema version.
    #[must_use]
    pub const fn schema_version(
        &self,
    ) -> u16 {
        self.schema_version
    }

    /// Returns the provenance identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> ProvenanceId {
        self.id
    }

    /// Returns the associated program identity.
    #[must_use]
    pub const fn program_id(
        &self,
    ) -> ProgramId {
        self.program_id
    }

    /// Returns the Quantum IR version.
    #[must_use]
    pub const fn ir_version(
        &self,
    ) -> IrVersion {
        self.ir_version
    }

    /// Sets the source reference.
    #[must_use]
    pub fn with_source(
        mut self,
        source: SourceReference,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the compiler identity.
    #[must_use]
    pub fn with_compiler(
        mut self,
        compiler: CompilerIdentity,
    ) -> Self {
        self.compiler = Some(compiler);
        self
    }

    /// Sets the input artifact content identity.
    #[must_use]
    pub fn with_input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.input_artifact = Some(artifact);
        self
    }

    /// Sets the output artifact content identity.
    #[must_use]
    pub fn with_output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.output_artifact = Some(artifact);
        self
    }

    /// Sets the target reference.
    #[must_use]
    pub fn with_target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets an explicit deterministic seed.
    #[must_use]
    pub fn with_deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.deterministic_seed = Some(seed);
        self
    }

    /// Sets the creation timestamp.
    ///
    /// This is observational metadata and is not automatically semantic.
    #[must_use]
    pub fn with_created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.created_at = Some(timestamp);
        self
    }

    /// Sets the completion timestamp.
    ///
    /// This is observational metadata and is not automatically semantic.
    #[must_use]
    pub fn with_completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.completed_at = Some(timestamp);
        self
    }

    /// Sets execution information.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.execution = Some(execution);
        self
    }

    /// Adds a transformation record.
    ///
    /// Sequence numbers must be monotonically increasing. This prevents an
    /// apparently valid provenance chain from silently changing order.
    pub fn add_transformation(
        &mut self,
        record: TransformationRecord,
    ) -> Result<(), ProvenanceError> {
        if let Some(previous) = self.transformations.last() {
            if record.sequence() <= previous.sequence() {
                return Err(ProvenanceError::InvalidRelationship {
                    message: format!(
                        "transformation sequence {} is not greater than previous sequence {}",
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
    pub fn add_calibration(
        &mut self,
        calibration: CalibrationReference,
    ) {
        self.calibration.push(calibration);
    }

    /// Adds a logical-to-physical mapping record.
    ///
    /// Duplicate logical or physical assignments are rejected.
    pub fn add_mapping(
        &mut self,
        mapping: QubitMappingRecord,
    ) -> Result<(), ProvenanceError> {
        for existing in &self.mappings {
            if existing.logical() == mapping.logical() {
                return Err(ProvenanceError::InvalidRelationship {
                    message: format!(
                        "logical qubit {} already has a provenance mapping",
                        mapping.logical()
                    ),
                });
            }

            if existing.physical() == mapping.physical() {
                return Err(ProvenanceError::InvalidRelationship {
                    message: format!(
                        "physical qubit {} is already mapped in provenance",
                        mapping.physical()
                    ),
                });
            }
        }

        self.mappings.push(mapping);

        self.mappings.sort_by_key(|entry| {
            (
                entry.logical(),
                entry.physical(),
            )
        });

        Ok(())
    }

    /// Records an operation participating in the provenance scope.
    ///
    /// Duplicate operation identities are ignored.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) {
        if !self.operations.contains(&operation) {
            self.operations.push(operation);

            self.operations.sort();
        }
    }

    /// Adds metadata.
    pub fn add_metadata(
        &mut self,
        entry: MetadataEntry,
    ) {
        self.metadata.insert(entry);
    }

    /// Returns the source reference.
    #[must_use]
    pub const fn source(
        &self,
    ) -> Option<&SourceReference> {
        self.source.as_ref()
    }

    /// Returns the compiler identity.
    #[must_use]
    pub const fn compiler(
        &self,
    ) -> Option<&CompilerIdentity> {
        self.compiler.as_ref()
    }

    /// Returns the input content identity.
    #[must_use]
    pub const fn input_artifact(
        &self,
    ) -> Option<&ContentReference> {
        self.input_artifact.as_ref()
    }

    /// Returns the output content identity.
    #[must_use]
    pub const fn output_artifact(
        &self,
    ) -> Option<&ContentReference> {
        self.output_artifact.as_ref()
    }

    /// Returns the transformations in deterministic execution order.
    #[must_use]
    pub fn transformations(
        &self,
    ) -> &[TransformationRecord] {
        &self.transformations
    }

    /// Returns the target reference.
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
        &self.calibration
    }

    /// Returns logical-to-physical mappings.
    #[must_use]
    pub fn mappings(
        &self,
    ) -> &[QubitMappingRecord] {
        &self.mappings
    }

    /// Returns operation identities associated with this provenance.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &[OperationId] {
        &self.operations
    }

    /// Returns execution information.
    #[must_use]
    pub const fn execution(
        &self,
    ) -> Option<&ExecutionReference> {
        self.execution.as_ref()
    }

    /// Returns the deterministic seed.
    #[must_use]
    pub const fn deterministic_seed(
        &self,
    ) -> Option<u64> {
        self.deterministic_seed
    }

    /// Returns the creation timestamp.
    #[must_use]
    pub const fn created_at(
        &self,
    ) -> Option<ProvenanceTimestamp> {
        self.created_at
    }

    /// Returns the completion timestamp.
    #[must_use]
    pub const fn completed_at(
        &self,
    ) -> Option<ProvenanceTimestamp> {
        self.completed_at
    }

    /// Returns metadata.
    #[must_use]
    pub const fn metadata(
        &self,
    ) -> &ProvenanceMetadata {
        &self.metadata
    }

    /// Validates the complete provenance relationship graph.
    pub fn validate(
        &self,
    ) -> Result<(), ProvenanceError> {
        if self.schema_version > PROVENANCE_SCHEMA_VERSION {
            return Err(ProvenanceError::UnsupportedSchemaVersion {
                found: self.schema_version,
                supported: PROVENANCE_SCHEMA_VERSION,
            });
        }

        validate_non_empty_if_present(
            "compiler.name",
            self.compiler.as_ref().map(|value| value.name()),
        )?;

        if let (
            Some(created),
            Some(completed),
        ) = (
            self.created_at,
            self.completed_at,
        ) {
            if completed < created {
                return Err(ProvenanceError::InvalidRelationship {
                    message: String::from(
                        "completed_at precedes created_at",
                    ),
                });
            }
        }

        let mut previous_sequence = None;

        for transformation in &self.transformations {
            if let Some(previous) = previous_sequence {
                if transformation.sequence() <= previous {
                    return Err(ProvenanceError::InvalidRelationship {
                        message: String::from(
                            "transformation sequence is not strictly increasing",
                        ),
                    });
                }
            }

            previous_sequence = Some(transformation.sequence());
        }

        let mut previous_operation = None;

        for operation in &self.operations {
            if let Some(previous) = previous_operation {
                if *operation <= previous {
                    return Err(ProvenanceError::InvalidRelationship {
                        message: String::from(
                            "operation identities are not strictly ordered or are duplicated",
                        ),
                    });
                }
            }

            previous_operation = Some(*operation);
        }

        let mut previous_logical = None;
        let mut previous_physical = None;

        for mapping in &self.mappings {
            if let Some(previous) = previous_logical {
                if mapping.logical() <= previous {
                    return Err(ProvenanceError::InvalidRelationship {
                        message: String::from(
                            "logical qubit mappings are not strictly ordered or are duplicated",
                        ),
                    });
                }
            }

            if let Some(previous) = previous_physical {
                if mapping.physical() == previous {
                    return Err(ProvenanceError::InvalidRelationship {
                        message: format!(
                            "physical qubit {} occurs more than once",
                            mapping.physical()
                        ),
                    });
                }
            }

            previous_logical = Some(mapping.logical());
            previous_physical = Some(mapping.physical());
        }

        Ok(())
    }

    /// Returns whether this provenance is suitable for deterministic semantic
    /// hashing.
    ///
    /// Runtime timestamps and execution observations make a provenance record
    /// observational rather than purely semantic. They therefore make this
    /// return `false`.
    #[must_use]
    pub const fn is_deterministic(
        &self,
    ) -> bool {
        self.created_at.is_none()
            && self.completed_at.is_none()
            && self.execution.is_none()
    }

    /// Returns a deterministic view of the provenance conceptually suitable for
    /// canonical serialization.
    ///
    /// This method does not clone or mutate the record. The serialization layer
    /// should explicitly omit observational fields when producing semantic
    /// hashes.
    #[must_use]
    pub const fn contains_observational_data(
        &self,
    ) -> bool {
        self.created_at.is_some()
            || self.completed_at.is_some()
            || self.execution.is_some()
    }
}

// =============================================================================
// Provenance builder
// =============================================================================

/// Builder for constructing provenance without exposing partially initialized
/// semantic state.
///
/// The builder is useful at compiler boundaries where provenance is assembled
/// incrementally.
#[derive(Debug, Clone)]
pub struct ProvenanceBuilder {
    provenance: Provenance,
}

impl ProvenanceBuilder {
    /// Creates a provenance builder.
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

    /// Adds source information.
    #[must_use]
    pub fn source(
        mut self,
        source: SourceReference,
    ) -> Self {
        self.provenance.source = Some(source);
        self
    }

    /// Adds compiler information.
    #[must_use]
    pub fn compiler(
        mut self,
        compiler: CompilerIdentity,
    ) -> Self {
        self.provenance.compiler = Some(compiler);
        self
    }

    /// Adds an input content identity.
    #[must_use]
    pub fn input_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.input_artifact = Some(artifact);
        self
    }

    /// Adds an output content identity.
    #[must_use]
    pub fn output_artifact(
        mut self,
        artifact: ContentReference,
    ) -> Self {
        self.provenance.output_artifact = Some(artifact);
        self
    }

    /// Adds target information.
    #[must_use]
    pub fn target(
        mut self,
        target: TargetReference,
    ) -> Self {
        self.provenance.target = Some(target);
        self
    }

    /// Adds a transformation.
    pub fn transformation(
        mut self,
        transformation: TransformationRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance
            .add_transformation(transformation)?;

        Ok(self)
    }

    /// Adds a calibration reference.
    #[must_use]
    pub fn calibration(
        mut self,
        calibration: CalibrationReference,
    ) -> Self {
        self.provenance.add_calibration(calibration);
        self
    }

    /// Adds a qubit mapping.
    pub fn mapping(
        mut self,
        mapping: QubitMappingRecord,
    ) -> Result<Self, ProvenanceError> {
        self.provenance.add_mapping(mapping)?;
        Ok(self)
    }

    /// Adds an operation.
    #[must_use]
    pub fn operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.provenance.add_operation(operation);
        self
    }

    /// Adds a deterministic seed.
    #[must_use]
    pub fn deterministic_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.provenance.deterministic_seed = Some(seed);
        self
    }

    /// Adds metadata.
    #[must_use]
    pub fn metadata(
        mut self,
        entry: MetadataEntry,
    ) -> Self {
        self.provenance.add_metadata(entry);
        self
    }

    /// Adds an observational creation timestamp.
    #[must_use]
    pub fn created_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance.created_at = Some(timestamp);
        self
    }

    /// Adds an observational completion timestamp.
    #[must_use]
    pub fn completed_at(
        mut self,
        timestamp: ProvenanceTimestamp,
    ) -> Self {
        self.provenance.completed_at = Some(timestamp);
        self
    }

    /// Adds execution information.
    #[must_use]
    pub fn execution(
        mut self,
        execution: ExecutionReference,
    ) -> Self {
        self.provenance.execution = Some(execution);
        self
    }

    /// Finishes construction and validates the provenance graph.
    pub fn build(
        self,
    ) -> Result<Provenance, ProvenanceError> {
        self.provenance.validate()?;
        Ok(self.provenance)
    }
}

// =============================================================================
// Utility validation
// =============================================================================

fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Err(ProvenanceError::EmptyField {
            field,
        });
    }

    Ok(())
}

fn validate_non_empty_if_present(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), ProvenanceError> {
    if let Some(value) = value {
        validate_non_empty(
            field,
            value,
        )?;
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

    #[test]
    fn digest_round_trip() {
        let mut bytes = [0u8; DIGEST_BYTES];

        for (index, byte) in bytes.iter_mut().enumerate() {
            *byte = index as u8;
        }

        let digest = ProvenanceDigest::from_bytes(bytes);
        let encoded = digest.to_hex();
        let decoded =
            ProvenanceDigest::from_hex(&encoded).expect(
                "digest must decode",
            );

        assert_eq!(digest, decoded);
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
    fn transformations_are_strictly_ordered() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        let first = TransformationRecord::new(
            1,
            TransformationKind::Lowering,
            "lower",
        )
        .expect("valid transformation");

        let second = TransformationRecord::new(
            2,
            TransformationKind::Optimization,
            "optimize",
        )
        .expect("valid transformation");

        provenance
            .add_transformation(first)
            .expect("first transformation");

        provenance
            .add_transformation(second)
            .expect("second transformation");

        assert_eq!(
            provenance.transformations().len(),
            2
        );

        assert!(
            provenance.validate().is_ok()
        );
    }

    #[test]
    fn duplicate_transformation_sequence_is_rejected() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        let first = TransformationRecord::new(
            1,
            TransformationKind::Lowering,
            "lower",
        )
        .expect("valid transformation");

        let second = TransformationRecord::new(
            1,
            TransformationKind::Optimization,
            "optimize",
        )
        .expect("valid transformation");

        provenance
            .add_transformation(first)
            .expect("first transformation");

        assert!(
            provenance
                .add_transformation(second)
                .is_err()
        );
    }

    #[test]
    fn qubit_mapping_uses_canonical_ir_qubit_types() {
        let mapping = QubitMappingRecord::new(
            QubitId::new(0),
            PhysicalQubitId::new(17),
        );

        assert_eq!(
            mapping.logical().index(),
            0
        );

        assert_eq!(
            mapping.physical().index(),
            17
        );
    }

    #[test]
    fn duplicate_logical_mapping_is_rejected() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        provenance
            .add_mapping(
                QubitMappingRecord::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
            )
            .expect("first mapping");

        assert!(
            provenance
                .add_mapping(
                    QubitMappingRecord::new(
                        QubitId::new(0),
                        PhysicalQubitId::new(2),
                    ),
                )
                .is_err()
        );
    }

    #[test]
    fn duplicate_physical_mapping_is_rejected() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        provenance
            .add_mapping(
                QubitMappingRecord::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
            )
            .expect("first mapping");

        assert!(
            provenance
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
    fn operation_ids_are_deduplicated_and_sorted() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        provenance.add_operation(
            OperationId::new(4),
        );

        provenance.add_operation(
            OperationId::new(2),
        );

        provenance.add_operation(
            OperationId::new(4),
        );

        assert_eq!(
            provenance.operations(),
            &[
                OperationId::new(2),
                OperationId::new(4),
            ]
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = ProvenanceMetadata::new();

        metadata.insert(
            MetadataEntry::new(
                "z",
                "last",
            )
            .expect("valid metadata"),
        );

        metadata.insert(
            MetadataEntry::new(
                "a",
                "first",
            )
            .expect("valid metadata"),
        );

        assert_eq!(
            metadata
                .iter()
                .next()
                .expect("entry")
                .key(),
            "a"
        );

        assert_eq!(
            metadata.get("z"),
            Some("last")
        );
    }

    #[test]
    fn observational_data_is_not_deterministic() {
        let provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        assert!(
            provenance.is_deterministic()
        );

        let timestamp =
            ProvenanceTimestamp::new(
                1,
                0,
            )
            .expect("valid timestamp");

        let provenance =
            provenance.with_created_at(timestamp);

        assert!(
            !provenance.is_deterministic()
        );

        assert!(
            provenance.contains_observational_data()
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
    fn timestamps_validate_order() {
        let mut provenance = Provenance::new(
            ProvenanceId::new(1),
            ProgramId::new(1),
            IrVersion::CURRENT,
        );

        provenance = provenance.with_created_at(
            ProvenanceTimestamp::new(
                20,
                0,
            )
            .expect("valid timestamp"),
        );

        provenance = provenance.with_completed_at(
            ProvenanceTimestamp::new(
                10,
                0,
            )
            .expect("valid timestamp"),
        );

        assert!(
            provenance.validate().is_err()
        );
    }

    #[test]
    fn builder_creates_valid_provenance() {
        let compiler =
            CompilerIdentity::new(
                "zamani",
                "1.0.0",
            )
            .expect("valid compiler");

        let source =
            SourceReference::new(
                "main.zm",
            )
            .expect("valid source");

        let provenance =
            ProvenanceBuilder::new(
                ProvenanceId::new(10),
                ProgramId::new(20),
                IrVersion::CURRENT,
            )
            .source(source)
            .compiler(compiler)
            .mapping(
                QubitMappingRecord::new(
                    QubitId::new(0),
                    PhysicalQubitId::new(0),
                ),
            )
            .expect("valid mapping")
            .operation(
                OperationId::new(1),
            )
            .deterministic_seed(42)
            .build()
            .expect("valid provenance");

        assert_eq!(
            provenance.program_id(),
            ProgramId::new(20)
        );

        assert_eq!(
            provenance.ir_version(),
            IrVersion::CURRENT
        );

        assert_eq!(
            provenance.mappings().len(),
            1
        );
    }

    #[test]
    fn no_fixed_qubit_limit_is_encoded() {
        let large_logical_id =
            QubitId::new(
                usize::MAX,
            );

        let large_physical_id =
            PhysicalQubitId::new(
                usize::MAX,
            );

        let mapping =
            QubitMappingRecord::new(
                large_logical_id,
                large_physical_id,
            );

        assert_eq!(
            mapping.logical().index(),
            usize::MAX
        );

        assert_eq!(
            mapping.physical().index(),
            usize::MAX
        );
    }

    #[test]
    fn program_identity_is_not_qubit_capacity() {
        let provenance =
            Provenance::new(
                ProvenanceId::new(
                    u64::MAX,
                ),
                ProgramId::new(
                    u64::MAX,
                ),
                IrVersion::CURRENT,
            );

        assert_eq!(
            provenance.id().value(),
            u64::MAX
        );

        assert_eq!(
            provenance.program_id().value(),
            u64::MAX
        );
    }
}