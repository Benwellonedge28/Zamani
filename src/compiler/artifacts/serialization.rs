
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Artifact Serialization
//!
//! Path: src/compiler/artifacts/serialization.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Provide the stable, versioned wire representation for canonical
//! compiler artifacts and artifact manifests.
//!
//! This module serializes the existing canonical types. It does not
//! introduce a second artifact, manifest, metadata, or IR model.
//!
//! OWNS
//!
//! - Artifact and manifest wire envelopes.
//! - Serialization format versioning.
//! - Deterministic JSON encoding.
//! - Strict document decoding.
//! - Inline and external payload representation.
//! - Configurable input and output budgets.
//! - Serialization-specific errors.
//!
//! DOES NOT OWN
//!
//! - Artifact or manifest in-memory representations.
//! - Metadata storage or metadata policy.
//! - Artifact validation policy.
//! - Portability classification.
//! - Semantic or IR verification.
//! - ZUIR serialization.
//! - External storage resolution.
//! - Cryptographic signing or trust decisions.
//! - Compression or encryption.
//! - Runtime execution.
//!
//! TRUST CONTRACT
//!
//! Successful decoding means that the wire document was decoded and
//! reconstructed according to this serialization contract.
//!
//! It does not prove that:
//! - The artifact is semantically valid.
//! - Its IR has passed its authoritative verifier.
//! - Its payload is safe to execute.
//! - Its external content exists.
//! - Its external content matches its declared digest.
//! - Its portability has been established.
//! - Its producer is trusted.
//!
//! Callers must invoke the applicable validation, IR verification,
//! security and portability authorities before relying on those claims.
//!
//! SCALABILITY
//!
//! No universal limit is imposed on:
//! - Artifact count.
//! - Manifest artifact count.
//! - Dependency count.
//! - Root count.
//! - Identifier length.
//! - Metadata entry count.
//! - Payload size.
//! - Hardware count.
//! - Device count.
//! - Computing domain count.
//!
//! Optional byte budgets are operational safeguards.
//! They are not semantic or hardware ceilings.
//!
//! Actual operation remains constrained by available memory, address
//! space, storage, processing time and destination capacity.
//!
//! Large payloads should use ArtifactPayload::External so that their
//! contents do not have to be loaded into compiler memory.
//!
//! DETERMINISM
//!
//! - Wire structure fields have a stable declaration order.
//! - Metadata uses the canonical BTreeMap ordering.
//! - Manifest artifacts, dependencies and roots use canonical ordering.
//! - Array order is preserved.
//! - No timestamps or random identifiers are generated here.
//!
//! COMPATIBILITY
//!
//! The serialization format version is independent of:
//! - Zamani language version.
//! - Compiler release version.
//! - Artifact model API version.
//! - Manifest schema version.
//! - ZUIR format version.
//! - Backend version.
//! - Target/runtime version.
//!
//! A future incompatible wire change requires a new major format
//! version or an explicit migration implementation.
//!
//! INTEGRATION
//!
//! artifact.rs:
//!     Supplies canonical artifact types.
//!
//! manifest.rs:
//!     Supplies canonical manifest types and graph validation.
//!
//! metadata.rs:
//!     Supplies metadata policy and validation, not wire storage.
//!
//! validation.rs:
//!     Performs artifact and manifest validation after decoding.
//!
//! portability.rs:
//!     Performs portability assessment independently.
//!
//! src/zuir/serialization/:
//!     Remains responsible for ZUIR document serialization.
//!
//! grammar/data/serialization.g4:
//!     Remains responsible for source-language syntax.
//!
//! FREEZE CRITERIA
//!
//! - Existing canonical types are reused.
//! - Artifact and manifest round trips preserve all represented fields.
//! - Inline payload digests are checked during decoding.
//! - External payloads remain explicitly unresolved.
//! - Manifest references and dependency structure are validated.
//! - Input and output budgets are optional and configurable.
//! - Unknown envelope fields are rejected.
//! - Unsupported format versions are rejected.
//! - No unsafe code or arbitrary hardware ceilings are introduced.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{self, Read, Write};

use super::artifact::{
    Artifact,
    ArtifactContentId,
    ArtifactDescriptor,
    ArtifactDigest,
    ArtifactError,
    ArtifactFormatId,
    ArtifactId,
    ArtifactKindId,
    ArtifactMetadata,
    ArtifactPayload,
    ArtifactPayloadRef,
    ArtifactProducerId,
    ArtifactProvenance,
    ArtifactStoreId,
    ArtifactTargetRef,
    DigestAlgorithm,
};

use super::manifest::{
    ArtifactDependency,
    ArtifactManifest,
    DependencyKind,
    ManifestError,
    ManifestId,
    ManifestVersion,
};

// ============================================================================
// Wire format identity and version
// ============================================================================

/// Stable identifier of the artifact serialization document schema.
pub const ARTIFACT_DOCUMENT_SCHEMA_ID: &str =
    "zamani.compiler.artifact-document";

/// Stable identifier of the manifest serialization document schema.
pub const MANIFEST_DOCUMENT_SCHEMA_ID: &str =
    "zamani.compiler.artifact-manifest-document";

/// Current major version of this wire format.
pub const SERIALIZATION_FORMAT_MAJOR: u32 = 1;

/// Current minor version of this wire format.
pub const SERIALIZATION_FORMAT_MINOR: u32 = 0;

/// Current patch version of this wire format.
pub const SERIALIZATION_FORMAT_PATCH: u32 = 0;

/// Current wire format version.
pub const CURRENT_SERIALIZATION_FORMAT_VERSION: SerializationVersion =
    SerializationVersion::new(
        SERIALIZATION_FORMAT_MAJOR,
        SERIALIZATION_FORMAT_MINOR,
        SERIALIZATION_FORMAT_PATCH,
    );

/// Version of the artifact serialization wire contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SerializationVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl SerializationVersion {
    /// Creates a version.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn major(self) -> u32 {
        self.major
    }

    pub const fn minor(self) -> u32 {
        self.minor
    }

    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Conservative compatibility rule.
    ///
    /// A reader supports a writer when both have the same major
    /// version and the writer is not newer than the reader.
    pub const fn supports(self, writer: Self) -> bool {
        self.major == writer.major
            && (
                writer.minor < self.minor
                    || (
                        writer.minor == self.minor
                            && writer.patch <= self.patch
                    )
            )
    }
}

impl Default for SerializationVersion {
    fn default() -> Self {
        CURRENT_SERIALIZATION_FORMAT_VERSION
    }
}

// ============================================================================
// Operation options
// ============================================================================

/// Options for encoding artifact and manifest documents.
///
/// `max_output_bytes = None` means that this module imposes no
/// additional output-size budget.
///
/// This does not promise unlimited memory or destination capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodeOptions {
    max_output_bytes: Option<usize>,
    pretty: bool,
}

impl EncodeOptions {
    pub const fn new() -> Self {
        Self {
            max_output_bytes: None,
            pretty: false,
        }
    }

    /// Sets an optional maximum encoded-document size.
    #[must_use]
    pub const fn with_max_output_bytes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_output_bytes = maximum;
        self
    }

    /// Enables human-readable indentation.
    ///
    /// Pretty output is deterministic, but has a different byte
    /// representation from compact output.
    #[must_use]
    pub const fn with_pretty_output(mut self, enabled: bool) -> Self {
        self.pretty = enabled;
        self
    }

    pub const fn max_output_bytes(&self) -> Option<usize> {
        self.max_output_bytes
    }

    pub const fn is_pretty(&self) -> bool {
        self.pretty
    }
}

impl Default for EncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for decoding artifact and manifest documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeOptions {
    max_input_bytes: Option<usize>,
}

impl DecodeOptions {
    pub const fn new() -> Self {
        Self {
            max_input_bytes: None,
        }
    }

    /// Sets an optional maximum input-document size.
    #[must_use]
    pub const fn with_max_input_bytes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_input_bytes = maximum;
        self
    }

    pub const fn max_input_bytes(&self) -> Option<usize> {
        self.max_input_bytes
    }
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by artifact serialization and deserialization.
#[derive(Debug)]
#[non_exhaustive]
pub enum SerializationError {
    /// JSON encoding or decoding failed.
    Json(serde_json::Error),

    /// The document does not use a supported schema identity.
    UnsupportedSchema {
        expected: &'static str,
        actual: String,
    },

    /// The document is not the requested document kind.
    WrongDocumentKind {
        expected: &'static str,
        actual: String,
    },

    /// The wire format version is unsupported.
    UnsupportedFormatVersion {
        reader: SerializationVersion,
        writer: SerializationVersion,
    },

    /// The configured output budget was exceeded.
    OutputBudgetExceeded {
        limit: usize,
    },

    /// The configured input budget was exceeded.
    InputBudgetExceeded {
        limit: usize,
    },

    /// The artifact representation is inconsistent.
    InvalidArtifact(String),

    /// The manifest representation is inconsistent.
    InvalidManifest(String),

    /// An artifact operation failed.
    Artifact(ArtifactError),

    /// A manifest operation failed.
    Manifest(ManifestError),

    /// An I/O operation failed.
    Io(io::Error),

    /// An allocation request failed.
    Allocation(String),

    /// An encoded length could not be represented.
    LengthOverflow,
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(f, "artifact serialization JSON error: {error}")
            }
            Self::UnsupportedSchema { expected, actual } => {
                write!(
                    f,
                    "unsupported document schema `{actual}`; expected `{expected}`"
                )
            }
            Self::WrongDocumentKind { expected, actual } => {
                write!(
                    f,
                    "wrong document kind `{actual}`; expected `{expected}`"
                )
            }
            Self::UnsupportedFormatVersion { reader, writer } => {
                write!(
                    f,
                    "serialization format {writer:?} is not supported by reader {reader:?}"
                )
            }
            Self::OutputBudgetExceeded { limit } => {
                write!(
                    f,
                    "encoded document exceeds the configured output budget of {limit} bytes"
                )
            }
            Self::InputBudgetExceeded { limit } => {
                write!(
                    f,
                    "input document exceeds the configured input budget of {limit} bytes"
                )
            }
            Self::InvalidArtifact(reason) => {
                write!(f, "invalid serialized artifact: {reason}")
            }
            Self::InvalidManifest(reason) => {
                write!(f, "invalid serialized manifest: {reason}")
            }
            Self::Artifact(error) => {
                write!(f, "artifact operation failed: {error}")
            }
            Self::Manifest(error) => {
                write!(f, "manifest operation failed: {error}")
            }
            Self::Io(error) => {
                write!(f, "artifact serialization I/O failed: {error}")
            }
            Self::Allocation(reason) => {
                write!(f, "artifact serialization allocation failed: {reason}")
            }
            Self::LengthOverflow => {
                f.write_str("serialized document length overflow")
            }
        }
    }
}

impl Error for SerializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Artifact(error) => Some(error),
            Self::Manifest(error) => Some(error),
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for SerializationError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<ArtifactError> for SerializationError {
    fn from(error: ArtifactError) -> Self {
        Self::Artifact(error)
    }
}

impl From<ManifestError> for SerializationError {
    fn from(error: ManifestError) -> Self {
        Self::Manifest(error)
    }
}

/// Result type used by this module.
pub type SerializationResult<T> = Result<T, SerializationError>;

// ============================================================================
// Encoded document
// ============================================================================

/// An owned, completely encoded artifact or manifest document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedDocument {
    bytes: Vec<u8>,
}

impl EncodedDocument {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Writes the complete staged document to a destination.
    ///
    /// A destination may still fail partway through this operation.
    pub fn write_to<W: Write>(
        &self,
        destination: &mut W,
    ) -> SerializationResult<()> {
        destination
            .write_all(&self.bytes)
            .map_err(SerializationError::Io)
    }
}

// ============================================================================
// Wire envelope
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl From<SerializationVersion> for WireVersion {
    fn from(version: SerializationVersion) -> Self {
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }
}

impl From<WireVersion> for SerializationVersion {
    fn from(version: WireVersion) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

#[derive(Debug, Serialize)]
struct EnvelopeRef<'a, T: Serialize> {
    format_version: WireVersion,
    schema_id: &'a str,
    document_kind: &'a str,
    payload: &'a T,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope<T> {
    format_version: WireVersion,
    schema_id: String,
    document_kind: String,
    payload: T,
}

// ============================================================================
// Digest wire representation
// ============================================================================

#[derive(Debug, Serialize)]
struct DigestWireRef<'a> {
    algorithm: &'a str,
    bytes: &'a [u8],
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DigestWire {
    algorithm: String,
    bytes: Vec<u8>,
}

impl<'a> From<&'a ArtifactDigest> for DigestWireRef<'a> {
    fn from(digest: &'a ArtifactDigest) -> Self {
        Self {
            algorithm: digest.algorithm().as_str(),
            bytes: digest.as_bytes(),
        }
    }
}

impl TryFrom<DigestWire> for ArtifactDigest {
    type Error = SerializationError;

    fn try_from(wire: DigestWire) -> SerializationResult<Self> {
        let algorithm = DigestAlgorithm::new(wire.algorithm)?;
        ArtifactDigest::new(algorithm, wire.bytes).map_err(Into::into)
    }
}

// ============================================================================
// Provenance wire representation
// ============================================================================

#[derive(Debug, Serialize)]
struct ProvenanceWireRef<'a> {
    producer: &'a str,
    source_fingerprint: Option<DigestWireRef<'a>>,
    transformations: Vec<DigestWireRef<'a>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenanceWire {
    producer: String,
    source_fingerprint: Option<DigestWire>,
    transformations: Vec<DigestWire>,
}

impl<'a> From<&'a ArtifactProvenance> for ProvenanceWireRef<'a> {
    fn from(provenance: &'a ArtifactProvenance) -> Self {
        Self {
            producer: provenance.producer().as_str(),
            source_fingerprint: provenance
                .source_fingerprint()
                .map(DigestWireRef::from),
            transformations: provenance
                .transformations()
                .iter()
                .map(DigestWireRef::from)
                .collect(),
        }
    }
}

impl TryFrom<ProvenanceWire> for ArtifactProvenance {
    type Error = SerializationError;

    fn try_from(wire: ProvenanceWire) -> SerializationResult<Self> {
        let producer = ArtifactProducerId::new(wire.producer)?;
        let mut provenance = ArtifactProvenance::new(producer);

        if let Some(source) = wire.source_fingerprint {
            provenance = provenance.with_source_fingerprint(
                ArtifactDigest::try_from(source)?,
            );
        }

        for transformation in wire.transformations {
            provenance = provenance.with_transformation(
                ArtifactDigest::try_from(transformation)?,
            );
        }

        Ok(provenance)
    }
}

// ============================================================================
// Artifact payload wire representation
// ============================================================================

#[derive(Debug, Serialize)]
#[serde(tag = "storage", rename_all = "snake_case")]
enum PayloadWireRef<'a> {
    Inline {
        bytes: &'a [u8],
    },
    External {
        store: &'a str,
        content: &'a str,
        expected_size_bytes: Option<u64>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "storage", rename_all = "snake_case", deny_unknown_fields)]
enum PayloadWire {
    Inline {
        bytes: Vec<u8>,
    },
    External {
        store: String,
        content: String,
        expected_size_bytes: Option<u64>,
    },
}

impl<'a> TryFrom<&'a ArtifactPayload> for PayloadWireRef<'a> {
    type Error = SerializationError;

    fn try_from(payload: &'a ArtifactPayload) -> SerializationResult<Self> {
        match payload {
            ArtifactPayload::Inline(bytes) => {
                Ok(Self::Inline {
                    bytes: bytes.as_ref(),
                })
            }
            ArtifactPayload::External(reference) => {
                Ok(Self::External {
                    store: reference.store().as_str(),
                    content: reference.content().as_str(),
                    expected_size_bytes: reference.expected_size_bytes(),
                })
            }
        }
    }
}

// ============================================================================
// Artifact wire representation
// ============================================================================

#[derive(Debug, Serialize)]
struct ArtifactWireRef<'a> {
    id: &'a str,
    kind: &'a str,
    format: &'a str,
    target: Option<&'a str>,
    metadata: &'a BTreeMap<String, String>,
    provenance: ProvenanceWireRef<'a>,
    digest: DigestWireRef<'a>,
    payload: PayloadWireRef<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactWire {
    id: String,
    kind: String,
    format: String,
    target: Option<String>,
    metadata: BTreeMap<String, String>,
    provenance: ProvenanceWire,
    digest: DigestWire,
    payload: PayloadWire,
}

impl<'a> TryFrom<&'a Artifact> for ArtifactWireRef<'a> {
    type Error = SerializationError;

    fn try_from(artifact: &'a Artifact) -> SerializationResult<Self> {
        let descriptor = artifact.descriptor();

        Ok(Self {
            id: artifact.id().as_str(),
            kind: artifact.kind().as_str(),
            format: artifact.format().as_str(),
            target: descriptor.target().map(|target| target.as_str()),
            metadata: descriptor.metadata().entries(),
            provenance: ProvenanceWireRef::from(
                descriptor.provenance(),
            ),
            digest: DigestWireRef::from(artifact.digest()),
            payload: PayloadWireRef::try_from(artifact.payload())?,
        })
    }
}

impl TryFrom<ArtifactWire> for Artifact {
    type Error = SerializationError;

    fn try_from(wire: ArtifactWire) -> SerializationResult<Self> {
        let id = ArtifactId::new(wire.id)?;
        let kind = ArtifactKindId::new(wire.kind)?;
        let format = ArtifactFormatId::new(wire.format)?;

        let provenance =
            ArtifactProvenance::try_from(wire.provenance)?;

        let mut metadata = ArtifactMetadata::new();

        for (key, value) in wire.metadata {
            metadata.insert(key, value)?;
        }

        let mut descriptor = ArtifactDescriptor::new(
            id,
            kind,
            format,
            provenance,
        )
        .with_metadata(metadata);

        if let Some(target) = wire.target {
            descriptor = descriptor.with_target(
                ArtifactTargetRef::new(target)?,
            );
        }

        let digest = ArtifactDigest::try_from(wire.digest)?;

        match wire.payload {
            PayloadWire::Inline { bytes } => {
                let artifact = Artifact::new_inline(
                    descriptor,
                    bytes,
                );

                if artifact.digest() != &digest {
                    return Err(
                        SerializationError::InvalidArtifact(
                            "inline payload does not match its declared digest"
                                .to_owned(),
                        ),
                    );
                }

                Ok(artifact)
            }

            PayloadWire::External {
                store,
                content,
                expected_size_bytes,
            } => {
                let reference = ArtifactPayloadRef::new(
                    ArtifactStoreId::new(store)?,
                    ArtifactContentId::new(content)?,
                    expected_size_bytes,
                );

                Artifact::new_external(
                    descriptor,
                    reference,
                    digest,
                )
                .map_err(Into::into)
            }
        }
    }
}

// ============================================================================
// Manifest wire representation
// ============================================================================

#[derive(Debug, Serialize)]
struct DependencyWireRef<'a> {
    dependent: &'a str,
    prerequisite: &'a str,
    kind: &'a str,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencyWire {
    dependent: String,
    prerequisite: String,
    kind: String,
}

#[derive(Debug, Serialize)]
struct ManifestWireRef<'a> {
    id: &'a str,
    version: ManifestVersionWire,
    artifacts: Vec<ArtifactWireRef<'a>>,
    dependencies: Vec<DependencyWireRef<'a>>,
    roots: Vec<&'a str>,
    metadata: &'a BTreeMap<String, String>,
    provenance: ProvenanceWireRef<'a>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestWire {
    id: String,
    version: ManifestVersionWire,
    artifacts: Vec<ArtifactWire>,
    dependencies: Vec<DependencyWire>,
    roots: Vec<String>,
    metadata: BTreeMap<String, String>,
    provenance: ProvenanceWire,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestVersionWire {
    major: u32,
    minor: u32,
    patch: u32,
}

impl From<ManifestVersion> for ManifestVersionWire {
    fn from(version: ManifestVersion) -> Self {
        Self {
            major: version.major(),
            minor: version.minor(),
            patch: version.patch(),
        }
    }
}

impl From<ManifestVersionWire> for ManifestVersion {
    fn from(version: ManifestVersionWire) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

fn manifest_to_wire(
    manifest: &ArtifactManifest,
) -> SerializationResult<ManifestWireRef<'_>> {
    // A structurally invalid manifest must not be emitted as a valid
    // canonical manifest document.
    manifest
        .validate()
        .map_err(SerializationError::Manifest)?;

    let artifacts = manifest
        .artifacts()
        .map(|(_, artifact)| ArtifactWireRef::try_from(artifact))
        .collect::<SerializationResult<Vec<_>>>()?;

    let dependencies = manifest
        .dependencies()
        .map(|dependency| DependencyWireRef {
            dependent: dependency.dependent().as_str(),
            prerequisite: dependency.prerequisite().as_str(),
            kind: dependency.kind().as_str(),
        })
        .collect();

    let roots = manifest
        .roots()
        .map(ArtifactId::as_str)
        .collect();

    Ok(ManifestWireRef {
        id: manifest.id().as_str(),
        version: manifest.version().into(),
        artifacts,
        dependencies,
        roots,
        metadata: manifest.metadata().entries(),
        provenance: ProvenanceWireRef::from(manifest.provenance()),
    })
}

fn manifest_from_wire(
    wire: ManifestWire,
) -> SerializationResult<ArtifactManifest> {
    let id = ManifestId::new(wire.id)?;
    let version = ManifestVersion::from(wire.version);
    let provenance = ArtifactProvenance::try_from(wire.provenance)?;

    let mut manifest = ArtifactManifest::with_version(
        id,
        version,
        provenance,
    );

    for (key, value) in wire.metadata {
        manifest.metadata_mut().insert(key, value)?;
    }

    for artifact_wire in wire.artifacts {
        manifest.insert_artifact(
            Artifact::try_from(artifact_wire)?,
        )?;
    }

    for dependency in wire.dependencies {
        let dependent = ArtifactId::new(dependency.dependent)?;
        let prerequisite = ArtifactId::new(dependency.prerequisite)?;
        let kind = DependencyKind::new(dependency.kind)?;

        manifest.add_dependency(
            ArtifactDependency::new(
                dependent,
                prerequisite,
                kind,
            ),
        );
    }

    for root in wire.roots {
        manifest.add_root(ArtifactId::new(root)?);
    }

    manifest
        .validate()
        .map_err(SerializationError::Manifest)?;

    Ok(manifest)
}

// ============================================================================
// Encoding
// ============================================================================

/// Encodes one canonical artifact.
pub fn encode_artifact(
    artifact: &Artifact,
) -> SerializationResult<EncodedDocument> {
    encode_artifact_with_options(artifact, &EncodeOptions::default())
}

/// Encodes one canonical artifact with explicit operational options.
pub fn encode_artifact_with_options(
    artifact: &Artifact,
    options: &EncodeOptions,
) -> SerializationResult<EncodedDocument> {
    // Verify inline payload integrity before writing it.
    //
    // External payloads cannot be verified here because their bytes
    // belong to an external storage provider.
    if artifact.has_inline_payload() {
        artifact.verify_inline_payload()?;
    }

    let payload = ArtifactWireRef::try_from(artifact)?;

    let envelope = EnvelopeRef {
        format_version: CURRENT_SERIALIZATION_FORMAT_VERSION.into(),
        schema_id: ARTIFACT_DOCUMENT_SCHEMA_ID,
        document_kind: "artifact",
        payload: &payload,
    };

    encode_value(&envelope, options)
}

/// Encodes one canonical manifest.
pub fn encode_manifest(
    manifest: &ArtifactManifest,
) -> SerializationResult<EncodedDocument> {
    encode_manifest_with_options(manifest, &EncodeOptions::default())
}

/// Encodes one canonical manifest with explicit operational options.
pub fn encode_manifest_with_options(
    manifest: &ArtifactManifest,
    options: &EncodeOptions,
) -> SerializationResult<EncodedDocument> {
    let payload = manifest_to_wire(manifest)?;

    let envelope = EnvelopeRef {
        format_version: CURRENT_SERIALIZATION_FORMAT_VERSION.into(),
        schema_id: MANIFEST_DOCUMENT_SCHEMA_ID,
        document_kind: "manifest",
        payload: &payload,
    };

    encode_value(&envelope, options)
}

/// Encodes an artifact and writes the complete staged document.
///
/// The destination is not touched until encoding has succeeded.
pub fn encode_artifact_to_writer<W: Write>(
    artifact: &Artifact,
    destination: &mut W,
) -> SerializationResult<()> {
    encode_artifact(artifact)?.write_to(destination)
}

/// Encodes a manifest and writes the complete staged document.
pub fn encode_manifest_to_writer<W: Write>(
    manifest: &ArtifactManifest,
    destination: &mut W,
) -> SerializationResult<()> {
    encode_manifest(manifest)?.write_to(destination)
}

fn encode_value<T: Serialize>(
    value: &T,
    options: &EncodeOptions,
) -> SerializationResult<EncodedDocument> {
    let mut writer = BudgetedVecWriter::new(
        options.max_output_bytes(),
    );

    let result = if options.is_pretty() {
        serde_json::to_writer_pretty(&mut writer, value)
    } else {
        serde_json::to_writer(&mut writer, value)
    };

    match result {
        Ok(()) => Ok(EncodedDocument {
            bytes: writer.into_inner(),
        }),
        Err(error) if writer.exceeded_budget() => {
            Err(SerializationError::OutputBudgetExceeded {
                limit: options.max_output_bytes().unwrap_or(0),
            })
        }
        Err(error) => Err(SerializationError::Json(error)),
    }
}

// ============================================================================
// Bounded staging writer
// ============================================================================

/// Private staging writer with an optional operational byte budget.
///
/// It does not impose a semantic or hardware limit.
struct BudgetedVecWriter {
    bytes: Vec<u8>,
    maximum: Option<usize>,
    exceeded: bool,
}

impl BudgetedVecWriter {
    fn new(maximum: Option<usize>) -> Self {
        Self {
            bytes: Vec::new(),
            maximum,
            exceeded: false,
        }
    }

    fn exceeded_budget(&self) -> bool {
        self.exceeded
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BudgetedVecWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        let new_length = self
            .bytes
            .len()
            .checked_add(input.len())
            .ok_or_else(|| {
                self.exceeded = true;
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "serialized document length overflow",
                )
            })?;

        if let Some(maximum) = self.maximum {
            if new_length > maximum {
                self.exceeded = true;

                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "configured output budget exceeded",
                ));
            }
        }

        self.bytes
            .try_reserve(input.len())
            .map_err(|error| {
                io::Error::other(
                    format!("unable to reserve output memory: {error}"),
                )
            })?;

        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ============================================================================
// Decoding
// ============================================================================

/// Decodes an artifact from a byte slice.
pub fn decode_artifact(
    input: &[u8],
) -> SerializationResult<Artifact> {
    decode_artifact_with_options(input, &DecodeOptions::default())
}

/// Decodes an artifact using explicit input limits.
pub fn decode_artifact_with_options(
    input: &[u8],
    options: &DecodeOptions,
) -> SerializationResult<Artifact> {
    check_input_budget(input.len(), options)?;

    let envelope: Envelope<ArtifactWire> =
        serde_json::from_slice(input)?;

    validate_envelope(
        &envelope.schema_id,
        &envelope.document_kind,
        envelope.format_version.into(),
        ARTIFACT_DOCUMENT_SCHEMA_ID,
        "artifact",
    )?;

    Artifact::try_from(envelope.payload)
}

/// Decodes a manifest from a byte slice.
pub fn decode_manifest(
    input: &[u8],
) -> SerializationResult<ArtifactManifest> {
    decode_manifest_with_options(input, &DecodeOptions::default())
}

/// Decodes a manifest using explicit input limits.
pub fn decode_manifest_with_options(
    input: &[u8],
    options: &DecodeOptions,
) -> SerializationResult<ArtifactManifest> {
    check_input_budget(input.len(), options)?;

    let envelope: Envelope<ManifestWire> =
        serde_json::from_slice(input)?;

    validate_envelope(
        &envelope.schema_id,
        &envelope.document_kind,
        envelope.format_version.into(),
        MANIFEST_DOCUMENT_SCHEMA_ID,
        "manifest",
    )?;

    manifest_from_wire(envelope.payload)
}

/// Decodes an artifact from a reader.
///
/// The input is staged privately before decoding. If a budget is
/// configured, reading stops as soon as the next byte would exceed it.
pub fn decode_artifact_from_reader<R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> SerializationResult<Artifact> {
    let bytes = read_document(reader, options)?;
    decode_artifact_with_options(&bytes, options)
}

/// Decodes a manifest from a reader.
pub fn decode_manifest_from_reader<R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> SerializationResult<ArtifactManifest> {
    let bytes = read_document(reader, options)?;
    decode_manifest_with_options(&bytes, options)
}

fn validate_envelope(
    actual_schema: &str,
    actual_kind: &str,
    writer_version: SerializationVersion,
    expected_schema: &'static str,
    expected_kind: &'static str,
) -> SerializationResult<()> {
    if actual_schema != expected_schema {
        return Err(SerializationError::UnsupportedSchema {
            expected: expected_schema,
            actual: actual_schema.to_owned(),
        });
    }

    if actual_kind != expected_kind {
        return Err(SerializationError::WrongDocumentKind {
            expected: expected_kind,
            actual: actual_kind.to_owned(),
        });
    }

    if !CURRENT_SERIALIZATION_FORMAT_VERSION.supports(writer_version) {
        return Err(
            SerializationError::UnsupportedFormatVersion {
                reader: CURRENT_SERIALIZATION_FORMAT_VERSION,
                writer: writer_version,
            },
        );
    }

    Ok(())
}

fn check_input_budget(
    length: usize,
    options: &DecodeOptions,
) -> SerializationResult<()> {
    if let Some(maximum) = options.max_input_bytes() {
        if length > maximum {
            return Err(
                SerializationError::InputBudgetExceeded {
                    limit: maximum,
                },
            );
        }
    }

    Ok(())
}

/// Reads a complete document while enforcing the optional input budget.
fn read_document<R: Read>(
    reader: &mut R,
    options: &DecodeOptions,
) -> SerializationResult<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 8192];

    loop {
        let count = reader
            .read(&mut chunk)
            .map_err(SerializationError::Io)?;

        if count == 0 {
            break;
        }

        let new_length = bytes
            .len()
            .checked_add(count)
            .ok_or(SerializationError::LengthOverflow)?;

        if let Some(maximum) = options.max_input_bytes() {
            if new_length > maximum {
                return Err(
                    SerializationError::InputBudgetExceeded {
                        limit: maximum,
                    },
                );
            }
        }

        bytes.try_reserve(count).map_err(|error| {
            SerializationError::Allocation(error.to_string())
        })?;

        bytes.extend_from_slice(&chunk[..count]);
    }

    Ok(bytes)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance() -> ArtifactProvenance {
        ArtifactProvenance::new(
            ArtifactProducerId::new("zamani.compiler.tests")
                .expect("valid producer"),
        )
        .with_source_fingerprint(
            ArtifactDigest::sha256(b"source"),
        )
        .with_transformation(
            ArtifactDigest::sha256(b"transformation"),
        )
    }

    fn sample_artifact() -> Artifact {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert("domain", "classical")
            .expect("valid metadata");

        metadata
            .insert("purpose", "test")
            .expect("valid metadata");

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("artifact.test")
                .expect("valid ID"),
            ArtifactKindId::new("portable-ir")
                .expect("valid kind"),
            ArtifactFormatId::new("application/x-zamani-zuir")
                .expect("valid format"),
            provenance(),
        )
        .with_metadata(metadata);

        Artifact::new_inline(
            descriptor,
            b"canonical test payload".to_vec(),
        )
    }

    fn sample_manifest() -> ArtifactManifest {
        let first = sample_artifact();

        let second_descriptor = ArtifactDescriptor::new(
            ArtifactId::new("artifact.application")
                .expect("valid ID"),
            ArtifactKindId::new("application")
                .expect("valid kind"),
            ArtifactFormatId::new("application/octet-stream")
                .expect("valid format"),
            provenance(),
        );

        let second = Artifact::new_inline(
            second_descriptor,
            b"application payload".to_vec(),
        );

        let first_id = first.id().clone();
        let second_id = second.id().clone();

        let mut manifest = ArtifactManifest::new(
            ManifestId::new("manifest.test")
                .expect("valid manifest ID"),
            provenance(),
        );

        manifest
            .insert_artifact(first)
            .expect("first artifact");

        manifest
            .insert_artifact(second)
            .expect("second artifact");

        manifest.add_dependency_between(
            second_id.clone(),
            first_id.clone(),
            DependencyKind::new("link-time")
                .expect("valid dependency kind"),
        );

        manifest.add_root(second_id);

        manifest
    }

    #[test]
    fn artifact_round_trip_preserves_fields() {
        let artifact = sample_artifact();

        let encoded = encode_artifact(&artifact)
            .expect("artifact encoding");

        let decoded = decode_artifact(encoded.as_bytes())
            .expect("artifact decoding");

        assert_eq!(decoded, artifact);
    }

    #[test]
    fn manifest_round_trip_preserves_fields() {
        let manifest = sample_manifest();

        let encoded = encode_manifest(&manifest)
            .expect("manifest encoding");

        let decoded = decode_manifest(encoded.as_bytes())
            .expect("manifest decoding");

        assert_eq!(decoded, manifest);
    }

    #[test]
    fn artifact_encoding_is_deterministic() {
        let artifact = sample_artifact();

        let first = encode_artifact(&artifact)
            .expect("first encoding");

        let second = encode_artifact(&artifact)
            .expect("second encoding");

        assert_eq!(first.as_bytes(), second.as_bytes());
    }

    #[test]
    fn manifest_encoding_is_deterministic() {
        let manifest = sample_manifest();

        let first = encode_manifest(&manifest)
            .expect("first encoding");

        let second = encode_manifest(&manifest)
            .expect("second encoding");

        assert_eq!(first.as_bytes(), second.as_bytes());
    }

    #[test]
    fn inline_payload_is_integrity_checked() {
        let artifact = sample_artifact();

        let encoded = encode_artifact(&artifact)
            .expect("encoding");

        let decoded = decode_artifact(encoded.as_bytes())
            .expect("decoding");

        assert!(decoded.verify_inline_payload().is_ok());
    }

    #[test]
    fn external_payload_is_preserved_without_claiming_verification() {
        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new("artifact.external")
                .expect("valid ID"),
            ArtifactKindId::new("large-model")
                .expect("valid kind"),
            ArtifactFormatId::new("application/octet-stream")
                .expect("valid format"),
            provenance(),
        );

        let digest = ArtifactDigest::sha256(b"external content");

        let artifact = Artifact::new_external(
            descriptor,
            ArtifactPayloadRef::new(
                ArtifactStoreId::new("content-store")
                    .expect("valid store"),
                ArtifactContentId::new("content-address")
                    .expect("valid content ID"),
                Some(16),
            ),
            digest,
        )
        .expect("external artifact");

        let encoded = encode_artifact(&artifact)
            .expect("encoding");

        let decoded = decode_artifact(encoded.as_bytes())
            .expect("decoding");

        assert!(decoded.has_external_payload());
        assert_eq!(decoded.digest(), artifact.digest());
        assert!(decoded.verify_inline_payload().is_err());
    }

    #[test]
    fn output_budget_is_enforced() {
        let options = EncodeOptions::default()
            .with_max_output_bytes(Some(1));

        let result = encode_artifact_with_options(
            &sample_artifact(),
            &options,
        );

        assert!(matches!(
            result,
            Err(SerializationError::OutputBudgetExceeded {
                limit: 1
            })
        ));
    }

    #[test]
    fn input_budget_is_enforced() {
        let encoded = encode_artifact(&sample_artifact())
            .expect("encoding");

        let options = DecodeOptions::default()
            .with_max_input_bytes(Some(1));

        let result = decode_artifact_with_options(
            encoded.as_bytes(),
            &options,
        );

        assert!(matches!(
            result,
            Err(SerializationError::InputBudgetExceeded {
                limit: 1
            })
        ));
    }

    #[test]
    fn wrong_schema_is_rejected() {
        let encoded = encode_artifact(&sample_artifact())
            .expect("encoding");

        let mut value: serde_json::Value =
            serde_json::from_slice(encoded.as_bytes())
                .expect("JSON");

        value["schema_id"] = serde_json::Value::String(
            "zamani.unknown.schema".to_owned(),
        );

        let bytes = serde_json::to_vec(&value)
            .expect("JSON encoding");

        assert!(matches!(
            decode_artifact(&bytes),
            Err(SerializationError::UnsupportedSchema { .. })
        ));
    }

    #[test]
    fn unknown_envelope_fields_are_rejected() {
        let encoded = encode_artifact(&sample_artifact())
            .expect("encoding");

        let mut value: serde_json::Value =
            serde_json::from_slice(encoded.as_bytes())
                .expect("JSON");

        value["unrecognized"] = serde_json::json!(true);

        let bytes = serde_json::to_vec(&value)
            .expect("JSON encoding");

        assert!(matches!(
            decode_artifact(&bytes),
            Err(SerializationError::Json(_))
        ));
    }

    #[test]
    fn writer_output_matches_owned_output() {
        let artifact = sample_artifact();

        let expected = encode_artifact(&artifact)
            .expect("encoding")
            .into_bytes();

        let mut destination = Vec::new();

        encode_artifact_to_writer(
            &artifact,
            &mut destination,
        )
        .expect("writer encoding");

        assert_eq!(destination, expected);
    }

    #[test]
    fn reader_decodes_manifest() {
        let manifest = sample_manifest();

        let encoded = encode_manifest(&manifest)
            .expect("encoding");

        let mut reader = encoded.as_bytes();

        let decoded = decode_manifest_from_reader(
            &mut reader,
            &DecodeOptions::default(),
        )
        .expect("reader decoding");

        assert_eq!(decoded, manifest);
    }

    #[test]
    fn unsupported_future_major_version_is_rejected() {
        let encoded = encode_artifact(&sample_artifact())
            .expect("encoding");

        let mut value: serde_json::Value =
            serde_json::from_slice(encoded.as_bytes())
                .expect("JSON");

        value["format_version"]["major"] =
            serde_json::json!(SERIALIZATION_FORMAT_MAJOR + 1);

        let bytes = serde_json::to_vec(&value)
            .expect("JSON encoding");

        assert!(matches!(
            decode_artifact(&bytes),
            Err(SerializationError::UnsupportedFormatVersion { .. })
        ));
    }

    #[test]
    fn pretty_and_compact_modes_decode_identically() {
        let artifact = sample_artifact();

        let compact = encode_artifact(&artifact)
            .expect("compact");

        let pretty = encode_artifact_with_options(
            &artifact,
            &EncodeOptions::default().with_pretty_output(true),
        )
        .expect("pretty");

        assert_ne!(compact.as_bytes(), pretty.as_bytes());

        assert_eq!(
            decode_artifact(compact.as_bytes()).expect("compact decode"),
            decode_artifact(pretty.as_bytes()).expect("pretty decode"),
        );
    }
}
