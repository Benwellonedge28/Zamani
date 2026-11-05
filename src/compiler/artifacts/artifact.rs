
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Canonical Artifact Model
//!
//! Path: src/compiler/artifacts/artifact.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the canonical in-memory representation of compiler artifacts.
//!
//! An artifact is an immutable description of a compilation product,
//! its content identity, its representation format, its provenance,
//! its metadata, and the location or storage form of its payload.
//!
//! Artifacts may represent:
//!
//! - Portable verified intermediate representations.
//! - Classical intermediate representations.
//! - Quantum intermediate representations.
//! - HDL and hardware representations.
//! - AI and tensor representations.
//! - Distributed computation packages.
//! - LLVM IR.
//! - WebAssembly modules.
//! - Native executables and libraries.
//! - FPGA bitstreams.
//! - Quantum execution packages.
//! - Hybrid deployment bundles.
//! - Future computational representations.
//!
//! This module is extensible. It does not maintain a closed inventory
//! of supported computing domains, target architectures, or formats.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//! - grammar/validation/semantic-boundaries.md
//! - src/compiler/artifacts/
//! - src/backend/
//! - src/zuir/
//! - src/zuir/serialization/
//!
//! The grammar defines source syntax.
//! Semantic analysis establishes source meaning.
//! ZUIR and domain IRs own computational representations.
//! Backends generate target-specific output.
//! This module represents the resulting artifacts.
//!
//! ## OWNS
//!
//! - Artifact identity.
//! - Artifact kind and format identifiers.
//! - Artifact descriptors.
//! - Artifact payload references.
//! - Inline artifact payload storage.
//! - Artifact metadata.
//! - Artifact provenance.
//! - Artifact content digest representation.
//! - Basic content-integrity verification.
//!
//! ## DOES NOT OWN
//!
//! - Source parsing or semantic analysis.
//! - ZUIR or domain IR definitions.
//! - Hardware discovery or physical allocation.
//! - Backend selection or code generation.
//! - Artifact dependency graph construction.
//! - Linking or relocation.
//! - Portability classification.
//! - Whole-program semantic verification.
//! - Stable wire serialization.
//! - External artifact storage implementation.
//! - Cryptographic signing or supply-chain authorization.
//! - Runtime execution.
//!
//! ## POCO-REAF
//!
//! The same semantic program may produce different artifacts for
//! different compatible execution environments.
//!
//! A portable representation and a target-specific realization
//! are distinct artifact products.
//!
//! An absent target reference does NOT prove portability.
//! Portability must be established by the portability subsystem.
//!
//! A valid content digest does NOT prove semantic validity.
//! Semantic validity must be established by the authoritative IR
//! verifier and applicable compilation stages.
//!
//! ## SCALABILITY
//!
//! No universal limits are imposed on:
//!
//! - Artifact count.
//! - Hardware count.
//! - Processor count.
//! - Quantum processor count.
//! - Qubit count.
//! - Device count.
//! - Memory capacity.
//! - Tensor rank.
//! - Register width.
//! - Domain count.
//! - Distributed node count.
//! - Payload size.
//!
//! Inline payloads use dynamically allocated shared byte storage.
//! External payload references allow large artifacts to remain in
//! external content stores without requiring their complete contents
//! to reside in compiler memory.
//!
//! Actual process and storage capacities remain finite.
//! Optional operational budgets belong to the responsible operations.
//!
//! ## DETERMINISM
//!
//! Artifact identity is supplied by the caller.
//! Metadata uses ordered maps.
//! Content digests are independent of memory addresses.
//! No wall-clock timestamp is automatically introduced.
//!
//! ## THREAD SAFETY
//!
//! Artifact payloads use Arc-backed immutable byte storage.
//! Artifact data can therefore be shared across threads without
//! mutable access to its contents.
//!
//! ## INTEGRATION
//!
//! manifest.rs:
//!     References ArtifactId and ArtifactDigest.
//!
//! metadata.rs:
//!     Reuses ArtifactMetadata rather than defining a second metadata model.
//!
//! portability.rs:
//!     Classifies artifacts using ArtifactDescriptor and target references.
//!
//! validation.rs:
//!     Checks payload integrity and coordinates authoritative verification.
//!
//! serialization.rs:
//!     Defines the stable serialized representation of artifacts.
//!
//! src/backend/:
//!     Produces artifacts from verified representations.
//!
//! src/zuir/:
//!     Supplies canonical representations that may become artifacts.
//!
//! src/compiler/linker.rs:
//!     Consumes artifact and manifest information for linking.
//!
//! The artifact model must not depend on those downstream modules.
//!
//! ## FREEZE CRITERIA
//!
//! - All public types have explicit ownership.
//! - Identifiers are extensible.
//! - Inline and external payloads are supported.
//! - Content integrity can be checked.
//! - External payloads are not falsely considered verified.
//! - Metadata is deterministic.
//! - No unsafe Rust is used.
//! - No arbitrary hardware ceilings are introduced.
//! - Unit and integration tests pass.
//! - Serialization and manifest integrations preserve this contract.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::sync::Arc;

use sha2::{Digest as ShaDigest, Sha256};

// ============================================================================
// Constants
// ============================================================================

/// Canonical algorithm identifier used for computed SHA-256 digests.
pub const SHA256_ALGORITHM: &str = "sha-256";

/// Version of the artifact model API.
///
/// This is independent of the Zamani language version, compiler release,
/// IR schema version, and serialized artifact format version.
pub const ARTIFACT_MODEL_API_VERSION: u16 = 1;

// ============================================================================
// Error model
// ============================================================================

/// Errors raised by artifact construction and inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArtifactError {
    InvalidIdentifier {
        field: &'static str,
        reason: String,
    },

    InvalidMetadataKey {
        key: String,
    },

    InvalidDigestAlgorithm {
        algorithm: String,
    },

    InvalidDigestLength {
        algorithm: String,
        actual: usize,
        expected: usize,
    },

    EmptyDigest,

    DigestMismatch,

    ExternalPayloadNotAvailable,

    UnsupportedDigestAlgorithm {
        algorithm: String,
    },

    PayloadSizeMismatch {
        expected: u64,
        actual: u64,
    },

    PayloadSizeOverflow,
}

impl fmt::Display for ArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason } => {
                write!(f, "invalid artifact {field}: {reason}")
            }

            Self::InvalidMetadataKey { key } => {
                write!(f, "invalid artifact metadata key: {key}")
            }

            Self::InvalidDigestAlgorithm { algorithm } => {
                write!(f, "invalid digest algorithm: {algorithm}")
            }

            Self::InvalidDigestLength {
                algorithm,
                actual,
                expected,
            } => {
                write!(
                    f,
                    "invalid {algorithm} digest length: \
                     expected {expected} bytes, received {actual}"
                )
            }

            Self::EmptyDigest => {
                write!(f, "artifact digest cannot be empty")
            }

            Self::DigestMismatch => {
                write!(f, "artifact content digest mismatch")
            }

            Self::ExternalPayloadNotAvailable => {
                write!(
                    f,
                    "external artifact payload must be resolved \
                     by its storage provider"
                )
            }

            Self::UnsupportedDigestAlgorithm { algorithm } => {
                write!(
                    f,
                    "digest verification is unavailable for algorithm \
                     {algorithm}"
                )
            }

            Self::PayloadSizeMismatch { expected, actual } => {
                write!(
                    f,
                    "artifact payload size mismatch: \
                     expected {expected}, received {actual}"
                )
            }

            Self::PayloadSizeOverflow => {
                write!(f, "artifact payload size cannot be represented")
            }
        }
    }
}

impl Error for ArtifactError {}

/// Result type used by this module.
pub type ArtifactResult<T> = Result<T, ArtifactError>;

// ============================================================================
// Identifier validation
// ============================================================================

/// Validate a nonempty, stable textual identifier.
///
/// No arbitrary identifier-length ceiling is imposed.
///
/// Identifiers are preserved exactly as supplied. They are not silently
/// truncated, case-folded, or normalized.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> ArtifactResult<()> {
    if value.is_empty() {
        return Err(ArtifactError::InvalidIdentifier {
            field,
            reason: "identifier cannot be empty".into(),
        });
    }

    if value.trim() != value {
        return Err(ArtifactError::InvalidIdentifier {
            field,
            reason: "leading or trailing whitespace is not permitted".into(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ArtifactError::InvalidIdentifier {
            field,
            reason: "control characters are not permitted".into(),
        });
    }

    Ok(())
}

// ============================================================================
// Extensible identifiers
// ============================================================================

/// Defines the common contract for stable artifact identifiers.
pub trait ArtifactIdentifier:
    Clone + Eq + Ord + fmt::Debug + fmt::Display + Send + Sync + 'static
{
    /// Returns the underlying identifier.
    fn as_str(&self) -> &str;
}

/// Defines an extensible textual artifact identifier.
macro_rules! define_artifact_identifier {
    ($name:ident, $field:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// Constructs a validated identifier.
            pub fn new(value: impl Into<String>) -> ArtifactResult<Self> {
                let value = value.into();

                validate_identifier(&value, $field)?;

                Ok(Self(value))
            }

            /// Returns the identifier text.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier.
            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl ArtifactIdentifier for $name {
            fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl TryFrom<String> for $name {
            type Error = ArtifactError;

            fn try_from(value: String) -> ArtifactResult<Self> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ArtifactError;

            fn try_from(value: &str) -> ArtifactResult<Self> {
                Self::new(value)
            }
        }
    };
}

// ============================================================================
// Artifact identities
// ============================================================================

/// Globally scoped or repository-scoped artifact identity.
///
/// The identity is opaque to this module. Generation and collision
/// avoidance are responsibilities of the artifact identity provider.
define_artifact_identifier!(ArtifactId, "identity");

/// Extensible artifact category.
///
/// Examples:
///
/// - `portable-ir`
/// - `classical-ir`
/// - `quantum-ir`
/// - `hdl-ir`
/// - `tensor-ir`
/// - `ai-model`
/// - `llvm-ir`
/// - `wasm-module`
/// - `native-executable`
/// - `fpga-bitstream`
/// - `quantum-package`
/// - `hybrid-bundle`
define_artifact_identifier!(ArtifactKindId, "kind");

/// Extensible artifact representation format.
///
/// Examples:
///
/// - `application/wasm`
/// - `application/x-llvm-ir`
/// - `application/json`
/// - `application/x-zamani-zuir`
/// - `application/x-qasm`
define_artifact_identifier!(ArtifactFormatId, "format");

/// Opaque target identity.
///
/// This does not define a hardware inventory or hardware capability model.
define_artifact_identifier!(ArtifactTargetRef, "target");

/// External content-store provider identity.
define_artifact_identifier!(ArtifactStoreId, "store");

/// Opaque identifier within an external content store.
define_artifact_identifier!(ArtifactContentId, "content");

/// Producer identity associated with an artifact.
define_artifact_identifier!(ArtifactProducerId, "producer");

// ============================================================================
// Digest algorithm
// ============================================================================

/// Extensible digest algorithm identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DigestAlgorithm(String);

impl DigestAlgorithm {
    /// Constructs an algorithm identifier.
    pub fn new(value: impl Into<String>) -> ArtifactResult<Self> {
        let value = value.into();

        validate_identifier(&value, "digest algorithm")?;

        Ok(Self(value))
    }

    /// Returns the algorithm name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the canonical SHA-256 identifier.
    pub fn sha256() -> Self {
        Self(SHA256_ALGORITHM.to_owned())
    }
}

impl fmt::Display for DigestAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// Artifact digest
// ============================================================================

/// Content digest.
///
/// A digest identifies bytes. It does not prove that those bytes are
/// semantically valid, executable, trustworthy, or portable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactDigest {
    algorithm: DigestAlgorithm,
    bytes: Vec<u8>,
}

impl ArtifactDigest {
    /// Constructs a digest from its algorithm and raw digest bytes.
    ///
    /// SHA-256 digests are validated to contain exactly 32 bytes.
    /// Other algorithms may be represented for forward compatibility,
    /// but cannot be verified unless an implementation is provided.
    pub fn new(
        algorithm: DigestAlgorithm,
        bytes: Vec<u8>,
    ) -> ArtifactResult<Self> {
        if bytes.is_empty() {
            return Err(ArtifactError::EmptyDigest);
        }

        if algorithm.as_str() == SHA256_ALGORITHM && bytes.len() != 32 {
            return Err(ArtifactError::InvalidDigestLength {
                algorithm: algorithm.to_string(),
                actual: bytes.len(),
                expected: 32,
            });
        }

        Ok(Self { algorithm, bytes })
    }

    /// Computes SHA-256 over arbitrary bytes.
    pub fn sha256(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);

        Self {
            algorithm: DigestAlgorithm::sha256(),
            bytes: digest.to_vec(),
        }
    }

    /// Returns the digest algorithm.
    pub fn algorithm(&self) -> &DigestAlgorithm {
        &self.algorithm
    }

    /// Returns raw digest bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns a lowercase hexadecimal representation.
    pub fn to_hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        let mut output = String::with_capacity(
            self.bytes.len().saturating_mul(2),
        );

        for byte in &self.bytes {
            output.push(HEX[(byte >> 4) as usize] as char);
            output.push(HEX[(byte & 0x0f) as usize] as char);
        }

        output
    }

    /// Verifies content against this digest.
    ///
    /// Returns an explicit error if this implementation does not support
    /// the declared digest algorithm.
    pub fn verify(&self, content: &[u8]) -> ArtifactResult<()> {
        if self.algorithm.as_str() != SHA256_ALGORITHM {
            return Err(
                ArtifactError::UnsupportedDigestAlgorithm {
                    algorithm: self.algorithm.to_string(),
                },
            );
        }

        let computed = Self::sha256(content);

        if computed.bytes != self.bytes {
            return Err(ArtifactError::DigestMismatch);
        }

        Ok(())
    }
}

impl fmt::Display for ArtifactDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.to_hex())
    }
}

// ============================================================================
// Artifact metadata
// ============================================================================

/// Deterministically ordered, extensible artifact metadata.
///
/// Metadata is descriptive. It must never be interpreted as proof
/// of semantic validity, authorization, or target compatibility.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactMetadata {
    entries: BTreeMap<String, String>,
}

impl ArtifactMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces an entry.
    ///
    /// Keys must be nonempty and cannot contain control characters.
    /// Values are unrestricted UTF-8 strings.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ArtifactResult<Option<String>> {
        let key = key.into();

        if key.is_empty()
            || key.trim() != key
            || key.chars().any(char::is_control)
        {
            return Err(ArtifactError::InvalidMetadataKey { key });
        }

        Ok(self.entries.insert(key, value.into()))
    }

    /// Returns a metadata value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Removes a metadata entry.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key)
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether metadata is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }

    /// Returns the underlying ordered entries.
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Artifact production provenance.
///
/// This records origin and transformation identities.
///
/// It does not independently certify that a transformation was correct.
/// Verification evidence belongs to the validation and provenance
/// integration layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactProvenance {
    producer: ArtifactProducerId,

    source_fingerprint: Option<ArtifactDigest>,

    transformation_fingerprints: Vec<ArtifactDigest>,
}

impl ArtifactProvenance {
    /// Creates provenance for an artifact producer.
    pub fn new(producer: ArtifactProducerId) -> Self {
        Self {
            producer,
            source_fingerprint: None,
            transformation_fingerprints: Vec::new(),
        }
    }

    /// Associates the original source fingerprint.
    #[must_use]
    pub fn with_source_fingerprint(
        mut self,
        fingerprint: ArtifactDigest,
    ) -> Self {
        self.source_fingerprint = Some(fingerprint);
        self
    }

    /// Adds a transformation fingerprint.
    #[must_use]
    pub fn with_transformation(
        mut self,
        fingerprint: ArtifactDigest,
    ) -> Self {
        self.transformation_fingerprints.push(fingerprint);
        self
    }

    /// Returns the producer identity.
    pub fn producer(&self) -> &ArtifactProducerId {
        &self.producer
    }

    /// Returns the source fingerprint.
    pub fn source_fingerprint(&self) -> Option<&ArtifactDigest> {
        self.source_fingerprint.as_ref()
    }

    /// Returns transformation fingerprints in execution order.
    pub fn transformations(&self) -> &[ArtifactDigest] {
        &self.transformation_fingerprints
    }
}

// ============================================================================
// External payload reference
// ============================================================================

/// Reference to an externally stored artifact payload.
///
/// External storage is important for artifacts that are too large or
/// expensive to retain in compiler memory.
///
/// This reference does not open files, access networks, or resolve storage.
/// Those responsibilities belong to a storage provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPayloadRef {
    store: ArtifactStoreId,
    content: ArtifactContentId,
    expected_size_bytes: Option<u64>,
}

impl ArtifactPayloadRef {
    /// Creates an external payload reference.
    pub fn new(
        store: ArtifactStoreId,
        content: ArtifactContentId,
        expected_size_bytes: Option<u64>,
    ) -> Self {
        Self {
            store,
            content,
            expected_size_bytes,
        }
    }

    /// Returns the external store identity.
    pub fn store(&self) -> &ArtifactStoreId {
        &self.store
    }

    /// Returns the external content identity.
    pub fn content(&self) -> &ArtifactContentId {
        &self.content
    }

    /// Returns the expected content size, if known.
    pub const fn expected_size_bytes(&self) -> Option<u64> {
        self.expected_size_bytes
    }
}

// ============================================================================
// Artifact payload
// ============================================================================

/// Artifact payload representation.
///
/// Inline payloads are immutable and shared through Arc.
///
/// External payloads are references resolved by an independent
/// artifact storage provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactPayload {
    /// Content stored in memory.
    Inline(Arc<[u8]>),

    /// Content stored externally.
    External(ArtifactPayloadRef),
}

impl ArtifactPayload {
    /// Constructs an inline payload.
    pub fn inline(bytes: impl Into<Vec<u8>>) -> Self {
        let bytes: Vec<u8> = bytes.into();

        Self::Inline(Arc::<[u8]>::from(bytes))
    }

    /// Constructs an external payload.
    pub fn external(reference: ArtifactPayloadRef) -> Self {
        Self::External(reference)
    }

    /// Returns inline bytes, if available.
    pub fn inline_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Inline(bytes) => Some(bytes.as_ref()),
            Self::External(_) => None,
        }
    }

    /// Returns the external reference, if available.
    pub fn external_reference(&self) -> Option<&ArtifactPayloadRef> {
        match self {
            Self::External(reference) => Some(reference),
            Self::Inline(_) => None,
        }
    }

    /// Whether the payload is stored inline.
    pub const fn is_inline(&self) -> bool {
        matches!(self, Self::Inline(_))
    }

    /// Whether the payload is stored externally.
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::External(_))
    }

    /// Returns the known payload size.
    ///
    /// Unknown external sizes return None.
    pub fn size_bytes(&self) -> Option<u64> {
        match self {
            Self::Inline(bytes) => u64::try_from(bytes.len()).ok(),

            Self::External(reference) => {
                reference.expected_size_bytes()
            }
        }
    }

    /// Writes inline bytes to a writer.
    ///
    /// External payloads must first be resolved through their
    /// registered storage provider.
    pub fn write_inline_to<W: Write>(
        &self,
        destination: &mut W,
    ) -> ArtifactResult<()> {
        match self {
            Self::Inline(bytes) => {
                destination
                    .write_all(bytes)
                    .map_err(|_| ArtifactError::ExternalPayloadNotAvailable)?;

                Ok(())
            }

            Self::External(_) => {
                Err(ArtifactError::ExternalPayloadNotAvailable)
            }
        }
    }
}

// ============================================================================
// Artifact descriptor
// ============================================================================

/// Describes an artifact independently of its payload storage location.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactDescriptor {
    id: ArtifactId,
    kind: ArtifactKindId,
    format: ArtifactFormatId,

    target: Option<ArtifactTargetRef>,

    metadata: ArtifactMetadata,

    provenance: ArtifactProvenance,
}

impl ArtifactDescriptor {
    /// Creates a complete artifact descriptor.
    pub fn new(
        id: ArtifactId,
        kind: ArtifactKindId,
        format: ArtifactFormatId,
        provenance: ArtifactProvenance,
    ) -> Self {
        Self {
            id,
            kind,
            format,
            target: None,
            metadata: ArtifactMetadata::new(),
            provenance,
        }
    }

    /// Associates a target identity.
    ///
    /// This does not establish target compatibility or feasibility.
    #[must_use]
    pub fn with_target(
        mut self,
        target: ArtifactTargetRef,
    ) -> Self {
        self.target = Some(target);
        self
    }

    /// Associates metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: ArtifactMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns artifact identity.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Returns artifact kind.
    pub fn kind(&self) -> &ArtifactKindId {
        &self.kind
    }

    /// Returns artifact format.
    pub fn format(&self) -> &ArtifactFormatId {
        &self.format
    }

    /// Returns target identity, if explicitly associated.
    pub fn target(&self) -> Option<&ArtifactTargetRef> {
        self.target.as_ref()
    }

    /// Returns artifact metadata.
    pub fn metadata(&self) -> &ArtifactMetadata {
        &self.metadata
    }

    /// Returns artifact provenance.
    pub fn provenance(&self) -> &ArtifactProvenance {
        &self.provenance
    }
}

// ============================================================================
// Canonical artifact
// ============================================================================

/// Immutable compiler artifact.
///
/// The artifact is the canonical pairing of:
///
/// - A validated descriptor.
/// - A payload representation.
/// - A content digest.
///
/// Cloning an inline artifact shares its immutable payload allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    descriptor: ArtifactDescriptor,

    payload: ArtifactPayload,

    digest: ArtifactDigest,
}

impl Artifact {
    /// Creates an artifact from inline bytes.
    ///
    /// The content digest is computed automatically.
    pub fn new_inline(
        descriptor: ArtifactDescriptor,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        let payload = ArtifactPayload::inline(bytes);

        let digest = ArtifactDigest::sha256(
            payload
                .inline_bytes()
                .expect("inline payload must expose its bytes"),
        );

        Self {
            descriptor,
            payload,
            digest,
        }
    }

    /// Creates an artifact backed by an external payload.
    ///
    /// The caller must provide the expected content digest.
    ///
    /// This constructor does not claim that the external content
    /// has already been retrieved or verified.
    pub fn new_external(
        descriptor: ArtifactDescriptor,
        reference: ArtifactPayloadRef,
        expected_digest: ArtifactDigest,
    ) -> ArtifactResult<Self> {
        if let Some(expected_size) = reference.expected_size_bytes() {
            let digest_size = expected_digest.as_bytes().len();

            if digest_size == 0 {
                return Err(ArtifactError::EmptyDigest);
            }

            let _ = digest_size;
            let _ = expected_size;
        }

        Ok(Self {
            descriptor,
            payload: ArtifactPayload::external(reference),
            digest: expected_digest,
        })
    }

    /// Returns the complete artifact descriptor.
    pub fn descriptor(&self) -> &ArtifactDescriptor {
        &self.descriptor
    }

    /// Returns the artifact identity.
    pub fn id(&self) -> &ArtifactId {
        self.descriptor.id()
    }

    /// Returns the artifact kind.
    pub fn kind(&self) -> &ArtifactKindId {
        self.descriptor.kind()
    }

    /// Returns the artifact format.
    pub fn format(&self) -> &ArtifactFormatId {
        self.descriptor.format()
    }

    /// Returns the artifact payload.
    pub fn payload(&self) -> &ArtifactPayload {
        &self.payload
    }

    /// Returns the content digest.
    pub fn digest(&self) -> &ArtifactDigest {
        &self.digest
    }

    /// Returns the known payload size.
    pub fn size_bytes(&self) -> Option<u64> {
        self.payload.size_bytes()
    }

    /// Returns whether the artifact has inline content.
    pub fn has_inline_payload(&self) -> bool {
        self.payload.is_inline()
    }

    /// Returns whether the artifact uses external content storage.
    pub fn has_external_payload(&self) -> bool {
        self.payload.is_external()
    }

    /// Returns inline content when available.
    pub fn inline_bytes(&self) -> Option<&[u8]> {
        self.payload.inline_bytes()
    }

    /// Verifies inline content against its digest.
    ///
    /// External payloads must first be resolved by the storage provider.
    pub fn verify_inline_payload(&self) -> ArtifactResult<()> {
        let bytes = self
            .payload
            .inline_bytes()
            .ok_or(ArtifactError::ExternalPayloadNotAvailable)?;

        if let Some(expected_size) = self.size_bytes() {
            let actual_size = u64::try_from(bytes.len())
                .map_err(|_| ArtifactError::PayloadSizeOverflow)?;

            if actual_size != expected_size {
                return Err(ArtifactError::PayloadSizeMismatch {
                    expected: expected_size,
                    actual: actual_size,
                });
            }
        }

        self.digest.verify(bytes)
    }

    /// Writes inline content.
    ///
    /// The artifact must be integrity-verified separately before it is
    /// accepted as a trusted compilation or deployment input.
    pub fn write_inline_to<W: Write>(
        &self,
        destination: &mut W,
    ) -> ArtifactResult<()> {
        match self.payload.inline_bytes() {
            Some(bytes) => {
                destination
                    .write_all(bytes)
                    .map_err(|_| ArtifactError::ExternalPayloadNotAvailable)?;

                Ok(())
            }

            None => Err(ArtifactError::ExternalPayloadNotAvailable),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn descriptor() -> ArtifactDescriptor {
        ArtifactDescriptor::new(
            ArtifactId::new("artifact.test").unwrap(),
            ArtifactKindId::new("portable-ir").unwrap(),
            ArtifactFormatId::new("application/x-zamani-zuir").unwrap(),
            ArtifactProvenance::new(
                ArtifactProducerId::new("zamani.compiler").unwrap(),
            ),
        )
    }

    #[test]
    fn identifiers_are_extensible() {
        let kind = ArtifactKindId::new(
            "future.computational.domain",
        )
        .unwrap();

        let format = ArtifactFormatId::new(
            "application/x-future-format",
        )
        .unwrap();

        assert_eq!(
            kind.as_str(),
            "future.computational.domain",
        );

        assert_eq!(
            format.as_str(),
            "application/x-future-format",
        );
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(ArtifactId::new("").is_err());

        assert!(ArtifactKindId::new("").is_err());

        assert!(ArtifactFormatId::new("").is_err());
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert!(ArtifactId::new("artifact\ninvalid").is_err());
    }

    #[test]
    fn empty_inline_payload_is_valid() {
        let artifact = Artifact::new_inline(
            descriptor(),
            Vec::<u8>::new(),
        );

        assert_eq!(artifact.size_bytes(), Some(0));

        assert!(artifact.verify_inline_payload().is_ok());
    }

    #[test]
    fn inline_artifact_calculates_digest() {
        let artifact = Artifact::new_inline(
            descriptor(),
            b"zamani".to_vec(),
        );

        assert_eq!(
            artifact.digest(),
            &ArtifactDigest::sha256(b"zamani"),
        );

        assert!(artifact.verify_inline_payload().is_ok());
    }

    #[test]
    fn digest_verification_detects_modified_content() {
        let digest = ArtifactDigest::sha256(b"original");

        assert!(
            digest.verify(b"original").is_ok()
        );

        assert_eq!(
            digest.verify(b"modified"),
            Err(ArtifactError::DigestMismatch),
        );
    }

    #[test]
    fn sha256_digest_has_expected_length() {
        let digest = ArtifactDigest::sha256(b"test");

        assert_eq!(digest.as_bytes().len(), 32);

        assert_eq!(
            digest.algorithm().as_str(),
            SHA256_ALGORITHM,
        );
    }

    #[test]
    fn invalid_sha256_length_is_rejected() {
        let result = ArtifactDigest::new(
            DigestAlgorithm::sha256(),
            vec![1, 2, 3],
        );

        assert!(matches!(
            result,
            Err(ArtifactError::InvalidDigestLength { .. })
        ));
    }

    #[test]
    fn external_artifact_preserves_reference() {
        let reference = ArtifactPayloadRef::new(
            ArtifactStoreId::new("content-store.primary").unwrap(),
            ArtifactContentId::new("content.123").unwrap(),
            Some(1024),
        );

        let artifact = Artifact::new_external(
            descriptor(),
            reference,
            ArtifactDigest::sha256(b"external content"),
        )
        .unwrap();

        assert!(artifact.has_external_payload());

        assert_eq!(artifact.size_bytes(), Some(1024));

        assert_eq!(
            artifact.verify_inline_payload(),
            Err(ArtifactError::ExternalPayloadNotAvailable),
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = ArtifactMetadata::new();

        metadata.insert("z.key", "last").unwrap();

        metadata.insert("a.key", "first").unwrap();

        let keys: Vec<&str> = metadata
            .iter()
            .map(|(key, _)| key)
            .collect();

        assert_eq!(keys, vec!["a.key", "z.key"]);
    }

    #[test]
    fn metadata_rejects_invalid_keys() {
        let mut metadata = ArtifactMetadata::new();

        assert!(
            metadata.insert("", "value").is_err()
        );

        assert!(
            metadata.insert("bad\nkey", "value").is_err()
        );
    }

    #[test]
    fn provenance_preserves_transformation_order() {
        let provenance = ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.compiler").unwrap(),
        )
        .with_transformation(
            ArtifactDigest::sha256(b"first"),
        )
        .with_transformation(
            ArtifactDigest::sha256(b"second"),
        );

        assert_eq!(provenance.transformations().len(), 2);

        assert_eq!(
            provenance.transformations()[0],
            ArtifactDigest::sha256(b"first"),
        );
    }

    #[test]
    fn target_reference_is_not_a_hardware_enum() {
        let target = ArtifactTargetRef::new(
            "future.accelerator.architecture",
        )
        .unwrap();

        let artifact = Artifact::new_inline(
            descriptor().with_target(target.clone()),
            b"output".to_vec(),
        );

        assert_eq!(
            artifact.descriptor().target(),
            Some(&target),
        );
    }

    #[test]
    fn inline_payload_can_be_written() {
        let artifact = Artifact::new_inline(
            descriptor(),
            b"zamani artifact".to_vec(),
        );

        let mut output = Vec::new();

        artifact.write_inline_to(&mut output).unwrap();

        assert_eq!(output, b"zamani artifact");
    }

    #[test]
    fn cloned_artifact_preserves_content() {
        let original = Artifact::new_inline(
            descriptor(),
            b"shared payload".to_vec(),
        );

        let cloned = original.clone();

        assert_eq!(original, cloned);

        assert_eq!(
            original.inline_bytes(),
            cloned.inline_bytes(),
        );
    }

    #[test]
    fn large_identifiers_are_not_rejected_by_arbitrary_limits() {
        let identifier = "a".repeat(16_384);

        let result = ArtifactId::new(identifier.clone());

        assert!(result.is_ok());

        assert_eq!(
            result.unwrap().as_str(),
            identifier,
        );
    }

    #[test]
    fn unknown_digest_algorithm_is_not_silently_accepted() {
        let digest = ArtifactDigest::new(
            DigestAlgorithm::new("future-hash").unwrap(),
            vec![1, 2, 3, 4],
        )
        .unwrap();

        assert!(matches!(
            digest.verify(b"content"),
            Err(ArtifactError::UnsupportedDigestAlgorithm { .. })
        ));
    }
}
