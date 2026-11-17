
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Manifest Metadata
//!
//! Path: `src/toolchain/packages/metadata.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the versioned serialization boundary between the canonical
//! in-memory Package model and the manifest bytes stored in a .zpkg
//! archive.
//!
//! The manifest format is independent of the source-language package
//! grammar. Source-language package declarations are owned by
//! grammar/modules/packages.g4. This module serializes the validated
//! Package model.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Package manifest schema identification and versioning.
//! - Manifest wire representation.
//! - Deterministic manifest serialization.
//! - Manifest deserialization.
//! - Duplicate object-key detection.
//! - Unknown structural-field rejection.
//! - Preservation of explicitly declared extension data.
//! - Conversion between manifest data and the canonical Package model.
//! - Manifest input/output operational options.
//!
//! This module does not own:
//!
//! - Package identity or package metadata model definitions.
//! - Package archive framing.
//! - Source-language parsing.
//! - Project TOML parsing.
//! - Dependency resolution.
//! - Package content hashing.
//! - Signature verification.
//! - Publisher authentication.
//! - Trust decisions.
//! - Provenance verification.
//! - Package extraction or installation.
//! - Registry access.
//! - Target or hardware compatibility evaluation.
//!
//! ## Canonical authorities
//!
//! Package model:
//!     super::model
//!
//! Package identity:
//!     crate::toolchain::ids
//!
//! Dependency names:
//!     crate::toolchain::dependencies::model
//!
//! Version schemes:
//!     crate::toolchain::dependencies::constraints
//!
//! Archive framing:
//!     super::archive
//!
//! Package integrity and authenticity:
//!     packages::verification and the canonical security authority
//!
//! ## Manifest format
//!
//! The manifest is compact JSON with the following logical structure:
//!
//! {
//!   "format": "zamani.package-manifest",
//!   "schema_version": "1.0",
//!   "package": { ... },
//!   "extensions": { ... }
//! }
//!
//! The package object contains the complete serializable package model.
//! Extensions are explicitly separated from known structural fields.
//!
//! Unknown structural fields are rejected. Extension values are retained
//! as JSON values and are never silently discarded by PackageManifest.
//!
//! ## Determinism
//!
//! - Metadata maps use BTreeMap.
//! - Content and artifact arrays are emitted in canonical path order.
//! - Set-backed values are emitted in deterministic order.
//! - JSON output is compact.
//! - No timestamps, random values, host paths, or environment values are
//!   introduced by serialization.
//!
//! Deterministic serialization is necessary but is not itself proof of
//! reproducible package creation. Archive and provenance authorities
//! retain their respective responsibilities.
//!
//! ## Scalability
//!
//! This module introduces no application-level maximum for:
//!
//! - Package metadata entries.
//! - Metadata list elements.
//! - Metadata map entries.
//! - Package contents.
//! - Package artifacts.
//! - Capabilities.
//! - Compatibility declarations.
//! - Signature references.
//! - Provenance references.
//! - Extension entries.
//! - Package-name length.
//! - Version text length.
//! - Computing domains.
//! - Hardware resources.
//!
//! Manifest byte budgets are optional operational policy.
//!
//! Rust address-space and allocator constraints still apply. No software
//! can promise literal infinite memory or computation.
//!
//! ## Security
//!
//! A successfully decoded manifest is structurally valid data only.
//!
//! It is not proof of:
//!
//! - Content integrity.
//! - Signature validity.
//! - Publisher identity.
//! - Publisher trust.
//! - Provenance authenticity.
//! - Target compatibility.
//! - Execution feasibility.
//!
//! Those checks must be performed by their authoritative subsystems.
//!
//! No credentials, private keys, or secret material belong in manifest
//! extension data.
//!
//! ## Integration
//!
//! archive.rs:
//!     Receives manifest bytes and their exact length.
//!
//! creation.rs:
//!     Serializes the Package model before calling archive writing.
//!
//! extraction.rs:
//!     Passes the manifest to a verifier; it does not decode it itself.
//!
//! verification.rs:
//!     Decodes the manifest and delegates integrity/authenticity checks.
//!
//! installation.rs:
//!     Uses the same canonical decoder through its verifier.
//!
//! registries:
//!     Consume the same manifest format when downloading or publishing.
//!
//! No consumer should implement a second package-manifest parser.

use std::collections::BTreeMap;
use std::io::{Read, Write};

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serde_json::Value;

use crate::toolchain::dependencies::constraints::VersionSchemeId;
use crate::toolchain::dependencies::model::DependencyName;
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    PackageId,
    SourceId,
};
use crate::toolchain::result::Result;

use super::model::{
    ArtifactProvenanceReference,
    ArtifactSignatureReference,
    ContentDigest,
    Package,
    PackageArtifact,
    PackageCompatibilityRequirement,
    PackageContent,
    PackageContentKind,
    PackageIdentity,
    PackageMetadata,
    PackageMetadataValue,
    PackageOrigin,
    PackagePath,
    PackageType,
    PackageVersion,
};

// ============================================================================
// FORMAT IDENTITY
// ============================================================================

/// Canonical package-manifest format identifier.
pub const PACKAGE_MANIFEST_FORMAT: &str =
    "zamani.package-manifest";

/// Current package-manifest schema version.
///
/// This is independent of:
///
/// - Package release versions.
/// - .zpkg archive-format versions.
/// - Zamani language versions.
/// - Compiler versions.
/// - Toolchain API versions.
/// - IR schema versions.
pub const PACKAGE_MANIFEST_SCHEMA_VERSION: &str = "1.0";

const OPERATION_ENCODE: &str = "package.metadata.encode";
const OPERATION_DECODE: &str = "package.metadata.decode";
const OPERATION_VALIDATE: &str = "package.metadata.validate";

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn invalid_manifest(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(OPERATION_DECODE)
}

fn serialization_error(
    error: serde_json::Error,
    operation: &'static str,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::PackageOperationFailed,
        format!("package manifest serialization failed: {error}"),
    )
    .with_operation(operation)
    .with_source(error)
}

fn allocation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::resource_exhausted(message)
        .with_operation(OPERATION_DECODE)
}

// ============================================================================
// DUPLICATE-REJECTING MAP
// ============================================================================

/// A deterministic map that rejects duplicate keys during deserialization.
///
/// Standard map deserialization can overwrite duplicate keys depending on
/// the map implementation. Package manifests must not silently accept
/// ambiguous duplicate metadata or extension fields.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct UniqueMap<V>(BTreeMap<String, V>);

impl<V> UniqueMap<V> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn insert(
        &mut self,
        key: String,
        value: V,
    ) -> std::result::Result<(), String> {
        if self.0.contains_key(&key) {
            return Err(format!(
                "duplicate package manifest object key `{key}`"
            ));
        }

        self.0.insert(key, value);
        Ok(())
    }

    fn into_inner(self) -> BTreeMap<String, V> {
        self.0
    }

    fn from_inner(values: BTreeMap<String, V>) -> Self {
        Self(values)
    }
}

impl<V> Serialize for UniqueMap<V>
where
    V: Serialize,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (key, value) in &self.0 {
            map.serialize_entry(key, value)?;
        }

        map.end()
    }
}

impl<'de, V> Deserialize<'de> for UniqueMap<V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UniqueMapVisitor<V>(std::marker::PhantomData<V>);

        impl<'de, V> Visitor<'de> for UniqueMapVisitor<V>
        where
            V: Deserialize<'de>,
        {
            type Value = UniqueMap<V>;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "a JSON object with unique string keys",
                )
            }

            fn visit_map<A>(
                self,
                mut access: A,
            ) -> std::result::Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut result = UniqueMap::new();

                while let Some((key, value)) =
                    access.next_entry::<String, V>()?
                {
                    result
                        .insert(key, value)
                        .map_err(de::Error::custom)?;
                }

                Ok(result)
            }
        }

        deserializer.deserialize_map(
            UniqueMapVisitor(std::marker::PhantomData),
        )
    }
}

// ============================================================================
// MANIFEST DOCUMENT
// ============================================================================

/// A decoded package manifest.
///
/// This wrapper retains extension data that is not part of the canonical
/// Package model. Callers that need lossless decode/encode round trips
/// must retain this wrapper rather than extracting only the Package.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageManifest {
    package: Package,
    extensions: BTreeMap<String, Value>,
}

impl PackageManifest {
    /// Construct a manifest from a validated package with no extensions.
    pub fn new(package: Package) -> Result<Self> {
        package
            .validate()
            .map_err(|error| {
                error
                    .with_operation(OPERATION_VALIDATE)
                    .with_context(
                        "validating package before manifest construction",
                    )
            })?;

        Ok(Self {
            package,
            extensions: BTreeMap::new(),
        })
    }

    /// Return the canonical package model.
    #[must_use]
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Return retained extension data.
    ///
    /// Extensions are ordered by key for deterministic serialization.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }

    /// Add one extension value.
    ///
    /// Existing keys are rejected rather than overwritten.
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Result<Self> {
        let key = key.into();

        validate_extension_key(&key)?;

        if self.extensions.contains_key(&key) {
            return Err(
                ToolchainError::validation_failed(
                    format!(
                        "duplicate package manifest extension `{key}`"
                    ),
                )
                .with_operation(OPERATION_VALIDATE),
            );
        }

        self.extensions.insert(key, value);

        Ok(self)
    }

    /// Consume the wrapper and return its package model.
    ///
    /// This explicitly discards extension data. Callers should use this
    /// only when extension data is not needed by their workflow.
    #[must_use]
    pub fn into_package(self) -> Package {
        self.package
    }

    /// Validate the package and extension map.
    pub fn validate(&self) -> Result<()> {
        self.package.validate()?;

        for key in self.extensions.keys() {
            validate_extension_key(key)?;
        }

        Ok(())
    }
}

fn validate_extension_key(key: &str) -> Result<()> {
    if key.is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
    {
        return Err(
            ToolchainError::invalid_input(
                "package manifest extension keys must be nonempty, \
                 trimmed text without control characters",
            )
            .with_operation(OPERATION_VALIDATE),
        );
    }

    Ok(())
}

// ============================================================================
// WIRE MODEL
// ============================================================================

/// Complete serialized manifest envelope.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestWire {
    format: String,
    schema_version: String,
    package: PackageWire,

    #[serde(default)]
    extensions: UniqueMap<Value>,
}

/// Serialized package model.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageWire {
    identity: PackageIdentityWire,
    version: PackageVersionWire,
    package_type: String,
    metadata: UniqueMap<MetadataValueWire>,
    contents: Vec<ContentWire>,
    artifacts: Vec<ArtifactWire>,
    capabilities: Vec<String>,
    origin: Option<OriginWire>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageIdentityWire {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageVersionWire {
    scheme: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum MetadataValueWire {
    String(String),
    Integer(String),
    Decimal(String),
    Character(char),
    Boolean(bool),
    Name(String),
    List(Vec<MetadataValueWire>),
    Map(UniqueMap<MetadataValueWire>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentWire {
    path: String,
    kind: String,
    media_type: Option<String>,
    declared_size: Option<u128>,
    digest: Option<DigestWire>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DigestWire {
    algorithm: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactWire {
    content_path: String,
    format: String,
    compatibility: Vec<CompatibilityWire>,
    capabilities: Vec<String>,
    digest: Option<DigestWire>,
    signatures: Vec<SignatureWire>,
    provenance: Vec<ProvenanceWire>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompatibilityWire {
    namespace: String,
    expression: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignatureWire {
    algorithm: String,
    signer: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProvenanceWire {
    kind: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OriginWire {
    source: String,
    revision: Option<String>,
}

// ============================================================================
// MODEL -> WIRE CONVERSION
// ============================================================================

fn metadata_to_wire(
    value: &PackageMetadataValue,
) -> MetadataValueWire {
    match value {
        PackageMetadataValue::String(value) => {
            MetadataValueWire::String(value.clone())
        }

        PackageMetadataValue::Integer(value) => {
            MetadataValueWire::Integer(value.clone())
        }

        PackageMetadataValue::Decimal(value) => {
            MetadataValueWire::Decimal(value.clone())
        }

        PackageMetadataValue::Character(value) => {
            MetadataValueWire::Character(*value)
        }

        PackageMetadataValue::Boolean(value) => {
            MetadataValueWire::Boolean(*value)
        }

        PackageMetadataValue::Name(value) => {
            MetadataValueWire::Name(value.clone())
        }

        PackageMetadataValue::List(values) => {
            MetadataValueWire::List(
                values.iter().map(metadata_to_wire).collect(),
            )
        }

        PackageMetadataValue::Map(values) => {
            MetadataValueWire::Map(
                UniqueMap::from_inner(
                    values
                        .iter()
                        .map(|(key, value)| {
                            (key.clone(), metadata_to_wire(value))
                        })
                        .collect(),
                ),
            )
        }
    }
}

fn metadata_from_wire(
    value: MetadataValueWire,
) -> Result<PackageMetadataValue> {
    let value = match value {
        MetadataValueWire::String(value) => {
            PackageMetadataValue::string(value)?
        }

        MetadataValueWire::Integer(value) => {
            PackageMetadataValue::integer(value)?
        }

        MetadataValueWire::Decimal(value) => {
            PackageMetadataValue::decimal(value)?
        }

        MetadataValueWire::Character(value) => {
            PackageMetadataValue::character(value)
        }

        MetadataValueWire::Boolean(value) => {
            PackageMetadataValue::boolean(value)
        }

        MetadataValueWire::Name(value) => {
            PackageMetadataValue::name(value)?
        }

        MetadataValueWire::List(values) => {
            let mut converted = Vec::new();

            converted.try_reserve(values.len()).map_err(|error| {
                allocation_error(format!(
                    "unable to allocate package metadata list: {error}"
                ))
            })?;

            for value in values {
                converted.push(metadata_from_wire(value)?);
            }

            PackageMetadataValue::list(converted)
        }

        MetadataValueWire::Map(values) => {
            let mut converted = BTreeMap::new();

            for (key, value) in values.into_inner() {
                let value = metadata_from_wire(value)?;

                if converted.insert(key.clone(), value).is_some() {
                    return Err(invalid_manifest(format!(
                        "duplicate nested metadata key `{key}`"
                    )));
                }
            }

            PackageMetadataValue::Map(converted)
        }
    };

    value.validate()?;

    Ok(value)
}

fn digest_to_wire(digest: &ContentDigest) -> DigestWire {
    DigestWire {
        algorithm: digest.algorithm().to_owned(),
        value: digest.value().to_owned(),
    }
}

fn digest_from_wire(wire: DigestWire) -> Result<ContentDigest> {
    ContentDigest::new(wire.algorithm, wire.value)
}

fn package_to_wire(package: &Package) -> PackageWire {
    let metadata = package
        .metadata()
        .entries()
        .iter()
        .map(|(key, value)| {
            (key.clone(), metadata_to_wire(value))
        })
        .collect();

    let contents = package
        .contents()
        .values()
        .map(|content| ContentWire {
            path: content.path().as_str().to_owned(),
            kind: content.kind().as_str().to_owned(),
            media_type: content.media_type().map(str::to_owned),
            declared_size: content.declared_size(),
            digest: content.digest().map(digest_to_wire),
        })
        .collect();

    let artifacts = package
        .artifacts()
        .values()
        .map(|artifact| ArtifactWire {
            content_path: artifact.content_path().as_str().to_owned(),
            format: artifact.format().to_owned(),

            compatibility: artifact
                .compatibility()
                .iter()
                .map(|requirement| CompatibilityWire {
                    namespace: requirement.namespace().to_owned(),
                    expression: requirement.expression().to_owned(),
                })
                .collect(),

            capabilities: artifact.capabilities().iter().cloned().collect(),

            digest: artifact.digest().map(digest_to_wire),

            signatures: artifact
                .signatures()
                .iter()
                .map(|signature| SignatureWire {
                    algorithm: signature.algorithm().to_owned(),
                    signer: signature.signer().to_owned(),
                    signature: signature.signature().to_owned(),
                })
                .collect(),

            provenance: artifact
                .provenance()
                .iter()
                .map(|provenance| ProvenanceWire {
                    kind: provenance.kind().to_owned(),
                    value: provenance.value().to_owned(),
                })
                .collect(),
        })
        .collect();

    PackageWire {
        identity: PackageIdentityWire {
            id: package.identity().id().to_string(),
            name: package.identity().name().as_str().to_owned(),
        },

        version: PackageVersionWire {
            scheme: package.version().scheme().as_str().to_owned(),
            value: package.version().value().to_owned(),
        },

        package_type: package.package_type().as_str().to_owned(),

        metadata: UniqueMap::from_inner(metadata),

        contents,
        artifacts,

        capabilities: package.capabilities().iter().cloned().collect(),

        origin: package.origin().map(|origin| OriginWire {
            source: origin.source().to_string(),
            revision: origin.revision().map(str::to_owned),
        }),
    }
}

fn manifest_to_wire(manifest: &PackageManifest) -> ManifestWire {
    ManifestWire {
        format: PACKAGE_MANIFEST_FORMAT.to_owned(),
        schema_version: PACKAGE_MANIFEST_SCHEMA_VERSION.to_owned(),
        package: package_to_wire(&manifest.package),
        extensions: UniqueMap::from_inner(
            manifest.extensions.clone(),
        ),
    }
}

// ============================================================================
// WIRE -> MODEL CONVERSION
// ============================================================================

fn package_from_wire(wire: PackageWire) -> Result<Package> {
    let package_id = PackageId::parse(&wire.identity.id)
        .map_err(|error| {
            invalid_manifest(format!(
                "invalid package identity: {error}"
            ))
        })?;

    let name = DependencyName::new(wire.identity.name)?;

    let identity = PackageIdentity::new(package_id, name);

    let scheme = VersionSchemeId::new(wire.version.scheme)?;

    let version = PackageVersion::new(
        scheme,
        wire.version.value,
    )?;

    let package_type = PackageType::new(wire.package_type)?;

    let mut package = Package::new(
        identity,
        version,
        package_type,
    );

    // Metadata keys are unique by construction in UniqueMap.
    for (key, value) in wire.metadata.into_inner() {
        package = package.with_metadata(
            key,
            metadata_from_wire(value)?,
        )?;
    }

    // Content paths are unique by the Package model's builder contract.
    for content in wire.contents {
        let path = PackagePath::new(content.path)?;

        let kind = PackageContentKind::new(content.kind)?;

        let mut descriptor = PackageContent::new(path, kind);

        if let Some(media_type) = content.media_type {
            descriptor = descriptor.with_media_type(media_type)?;
        }

        if let Some(size) = content.declared_size {
            descriptor = descriptor.with_declared_size(size);
        }

        if let Some(digest) = content.digest {
            descriptor = descriptor.with_digest(
                digest_from_wire(digest)?,
            );
        }

        package = package.with_content(descriptor)?;
    }

    // Artifact paths and set entries are validated by their model builders.
    for artifact in wire.artifacts {
        let path = PackagePath::new(artifact.content_path)?;

        let mut descriptor = PackageArtifact::new(
            path,
            artifact.format,
        )?;

        for requirement in artifact.compatibility {
            descriptor = descriptor.with_compatibility(
                PackageCompatibilityRequirement::new(
                    requirement.namespace,
                    requirement.expression,
                )?,
            )?;
        }

        for capability in artifact.capabilities {
            descriptor = descriptor.with_capability(capability)?;
        }

        if let Some(digest) = artifact.digest {
            descriptor = descriptor.with_digest(
                digest_from_wire(digest)?,
            );
        }

        for signature in artifact.signatures {
            descriptor = descriptor.with_signature(
                ArtifactSignatureReference::new(
                    signature.algorithm,
                    signature.signer,
                    signature.signature,
                )?,
            )?;
        }

        for provenance in artifact.provenance {
            descriptor = descriptor.with_provenance(
                ArtifactProvenanceReference::new(
                    provenance.kind,
                    provenance.value,
                )?,
            )?;
        }

        package = package.with_artifact(descriptor)?;
    }

    for capability in wire.capabilities {
        package = package.with_capability(capability)?;
    }

    if let Some(origin) = wire.origin {
        let source = SourceId::parse(&origin.source)
            .map_err(|error| {
                invalid_manifest(format!(
                    "invalid package source identity: {error}"
                ))
            })?;

        package = package.with_origin(
            PackageOrigin::new(source, origin.revision)?,
        );
    }

    package.validate().map_err(|error| {
        error
            .with_operation(OPERATION_VALIDATE)
            .with_context(
                "validating reconstructed package model",
            )
    })?;

    Ok(package)
}

fn manifest_from_wire(wire: ManifestWire) -> Result<PackageManifest> {
    if wire.format != PACKAGE_MANIFEST_FORMAT {
        return Err(
            ToolchainError::incompatible_version(format!(
                "unsupported package manifest format `{}`",
                wire.format
            ))
            .with_operation(OPERATION_DECODE),
        );
    }

    if wire.schema_version != PACKAGE_MANIFEST_SCHEMA_VERSION {
        return Err(
            ToolchainError::incompatible_version(format!(
                "unsupported package manifest schema version `{}`",
                wire.schema_version
            ))
            .with_operation(OPERATION_DECODE),
        );
    }

    let package = package_from_wire(wire.package)?;

    let extensions = wire.extensions.into_inner();

    for key in extensions.keys() {
        validate_extension_key(key)?;
    }

    Ok(PackageManifest {
        package,
        extensions,
    })
}

// ============================================================================
// PUBLIC SERIALIZATION API
// ============================================================================

/// Serialize a package into canonical compact manifest bytes.
///
/// The package is validated before serialization.
///
/// This convenience function creates a manifest without extension data.
/// Use [`encode_manifest`] when extension data must be preserved.
///
/// # Errors
///
/// Returns an error if the package is invalid or serialization fails.
pub fn encode_package(package: &Package) -> Result<Vec<u8>> {
    let manifest = PackageManifest::new(package.clone())?;
    encode_manifest(&manifest)
}

/// Serialize a complete manifest into canonical compact JSON bytes.
///
/// Ordered package collections and extension maps make the output
/// deterministic for equivalent manifest models.
///
/// # Errors
///
/// Returns an error if validation or serialization fails.
pub fn encode_manifest(
    manifest: &PackageManifest,
) -> Result<Vec<u8>> {
    manifest.validate()?;

    let wire = manifest_to_wire(manifest);

    serde_json::to_vec(&wire).map_err(|error| {
        serialization_error(error, OPERATION_ENCODE)
    })
}

/// Serialize a complete manifest directly to a writer.
///
/// This avoids requiring a second in-memory copy of the serialized
/// manifest. The writer is not flushed automatically.
///
/// A successful return means the serializer completed its writes.
/// It does not mean the destination was persisted atomically.
///
/// # Errors
///
/// Returns an error if validation or output fails.
pub fn write_manifest<W>(
    writer: &mut W,
    manifest: &PackageManifest,
) -> Result<()>
where
    W: Write,
{
    manifest.validate()?;

    let wire = manifest_to_wire(manifest);

    serde_json::to_writer(writer, &wire).map_err(|error| {
        serialization_error(error, OPERATION_ENCODE)
    })
}

// ============================================================================
// PUBLIC DESERIALIZATION API
// ============================================================================

/// Decode a complete manifest from a byte slice.
///
/// Unknown structural fields, duplicate object keys, unsupported schema
/// versions, invalid identifiers, duplicate package entries and invalid
/// package models are rejected.
///
/// Extension data is preserved in the returned PackageManifest.
///
/// # Errors
///
/// Returns an error if the input is malformed or incompatible.
pub fn decode_manifest(
    bytes: &[u8],
) -> Result<PackageManifest> {
    let wire: ManifestWire =
        serde_json::from_slice(bytes).map_err(|error| {
            serialization_error(error, OPERATION_DECODE)
        })?;

    manifest_from_wire(wire)
}

/// Decode a manifest and return only its package model.
///
/// This function rejects manifests containing extension data. This
/// prevents a caller from unknowingly discarding extension information.
///
/// Use [`decode_manifest`] if extensions must be retained.
///
/// # Errors
///
/// Returns an error if decoding fails or extensions are present.
pub fn decode_package(bytes: &[u8]) -> Result<Package> {
    let manifest = decode_manifest(bytes)?;

    if !manifest.extensions().is_empty() {
        return Err(
            ToolchainError::validation_failed(
                "manifest contains extension data; use decode_manifest \
                 to preserve it",
            )
            .with_operation(OPERATION_DECODE),
        );
    }

    Ok(manifest.into_package())
}

// ============================================================================
// LENGTH-AWARE INPUT
// ============================================================================

/// Operational limits for manifest input.
///
/// `max_bytes = None` means that this module does not impose an
/// application-level manifest-size budget. Host address-space and
/// allocator constraints still apply.
///
/// A configured limit is an operational safety policy, not a package
/// language or package-format capability ceiling.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ManifestReadOptions {
    max_bytes: Option<u128>,
}

impl ManifestReadOptions {
    /// Construct manifest input options.
    ///
    /// `None` means no application-level byte budget.
    #[must_use]
    pub const fn new(max_bytes: Option<u128>) -> Self {
        Self { max_bytes }
    }

    /// Return the optional maximum manifest byte budget.
    #[must_use]
    pub const fn max_bytes(self) -> Option<u128> {
        self.max_bytes
    }
}

/// Read and decode exactly the declared number of manifest bytes.
///
/// The reader may contain more data after the manifest. Those bytes are
/// not consumed. This is required when the reader is positioned inside
/// a larger archive stream.
///
/// The declared length must fit the host address space because the
/// current serde-based wire decoder materializes the manifest bytes.
///
/// # Errors
///
/// Returns an error for:
///
/// - A configured manifest-byte budget being exceeded.
/// - A length not representable by the host.
/// - Allocation failure.
/// - Truncated input.
/// - Invalid manifest data.
pub fn read_manifest<R>(
    reader: &mut R,
    manifest_length: u128,
    options: ManifestReadOptions,
) -> Result<PackageManifest>
where
    R: Read,
{
    if let Some(max_bytes) = options.max_bytes {
        if manifest_length > max_bytes {
            return Err(
                ToolchainError::resource_exhausted(format!(
                    "package manifest length {manifest_length} \
                     exceeds configured byte budget {max_bytes}"
                ))
                .with_operation(OPERATION_DECODE),
            );
        }
    }

    let length = usize::try_from(manifest_length).map_err(|_| {
        allocation_error(
            "manifest length cannot be represented by this host",
        )
    })?;

    let mut bytes = Vec::new();

    bytes.try_reserve_exact(length).map_err(|error| {
        allocation_error(format!(
            "unable to reserve package manifest buffer: {error}"
        ))
    })?;

    bytes.resize(length, 0);

    reader.read_exact(&mut bytes).map_err(|error| {
        ToolchainError::from_io(OPERATION_DECODE, error)
            .with_context(
                "reading the declared package manifest byte length",
            )
    })?;

    decode_manifest(&bytes)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    use std::io::Cursor;

    fn fixture_package() -> Package {
        let package_id = PackageId::new(
            IdNamespace::new("zamani.package")
                .expect("valid namespace"),
            1,
        )
        .expect("valid package ID");

        let identity = PackageIdentity::new(
            package_id,
            DependencyName::new("org::zamani::example")
                .expect("valid package name"),
        );

        let version = PackageVersion::new(
            VersionSchemeId::new("semver")
                .expect("valid version scheme"),
            "1.0.0",
        )
        .expect("valid version");

        let mut package = Package::new(
            identity,
            version,
            PackageType::new("library")
                .expect("valid package type"),
        );

        package = package
            .with_metadata(
                "domain",
                PackageMetadataValue::name("classical")
                    .expect("valid metadata name"),
            )
            .expect("add metadata");

        package = package
            .with_metadata(
                "description",
                PackageMetadataValue::string(
                    "A portable Zamani package",
                )
                .expect("valid metadata string"),
            )
            .expect("add metadata");

        let path = PackagePath::new("src/lib.zm")
            .expect("valid package path");

        let content = PackageContent::new(
            path,
            PackageContentKind::new("source")
                .expect("valid content kind"),
        )
        .with_declared_size(12);

        package
            .with_content(content)
            .expect("add package content")
    }

    #[test]
    fn package_round_trip_preserves_model() {
        let package = fixture_package();

        let encoded = encode_package(&package)
            .expect("encode package");

        let decoded = decode_package(&encoded)
            .expect("decode package");

        assert_eq!(decoded, package);
    }

    #[test]
    fn manifest_round_trip_preserves_extensions() {
        let package = fixture_package();

        let manifest = PackageManifest::new(package)
            .expect("manifest");

        let manifest = manifest
            .with_extension(
                "org.example.future",
                serde_json::json!({
                    "feature": "future-domain",
                    "enabled": true
                }),
            )
            .expect("add extension");

        let encoded = encode_manifest(&manifest)
            .expect("encode manifest");

        let decoded = decode_manifest(&encoded)
            .expect("decode manifest");

        assert_eq!(decoded, manifest);
    }

    #[test]
    fn package_only_decoder_does_not_discard_extensions() {
        let manifest = PackageManifest::new(fixture_package())
            .expect("manifest")
            .with_extension(
                "org.example.extension",
                Value::Bool(true),
            )
            .expect("extension");

        let encoded = encode_manifest(&manifest)
            .expect("encode");

        assert!(decode_package(&encoded).is_err());
    }

    #[test]
    fn duplicate_top_level_json_keys_are_rejected() {
        let input = br#"{
            "format": "zamani.package-manifest",
            "format": "zamani.package-manifest",
            "schema_version": "1.0",
            "package": {},
            "extensions": {}
        }"#;

        assert!(decode_manifest(input).is_err());
    }

    #[test]
    fn unknown_structural_fields_are_rejected() {
        let input = br#"{
            "format": "zamani.package-manifest",
            "schema_version": "1.0",
            "package": {},
            "extensions": {},
            "unexpected": true
        }"#;

        assert!(decode_manifest(input).is_err());
    }

    #[test]
    fn unsupported_schema_versions_are_rejected() {
        let input = br#"{
            "format": "zamani.package-manifest",
            "schema_version": "999.0",
            "package": {},
            "extensions": {}
        }"#;

        assert!(decode_manifest(input).is_err());
    }

    #[test]
    fn manifest_reader_consumes_only_declared_bytes() {
        let package = fixture_package();

        let encoded = encode_package(&package)
            .expect("encode package");

        let length = encoded.len() as u128;

        let mut combined = encoded.clone();
        combined.extend_from_slice(b"archive-tail");

        let mut reader = Cursor::new(combined);

        let decoded = read_manifest(
            &mut reader,
            length,
            ManifestReadOptions::default(),
        )
        .expect("read manifest");

        assert_eq!(decoded.package(), &package);

        assert_eq!(
            reader.position(),
            length as u64,
        );
    }

    #[test]
    fn manifest_byte_budget_is_enforced() {
        let package = fixture_package();

        let encoded = encode_package(&package)
            .expect("encode package");

        let mut reader = Cursor::new(encoded.clone());

        let result = read_manifest(
            &mut reader,
            encoded.len() as u128,
            ManifestReadOptions::new(Some(1)),
        );

        assert!(result.is_err());
    }

    #[test]
    fn truncated_manifest_is_rejected() {
        let package = fixture_package();

        let encoded = encode_package(&package)
            .expect("encode package");

        let mut reader = Cursor::new(encoded);

        let result = read_manifest(
            &mut reader,
            u128::MAX,
            ManifestReadOptions::default(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn encoding_is_deterministic() {
        let package = fixture_package();

        let first = encode_package(&package)
            .expect("first encoding");

        let second = encode_package(&package)
            .expect("second encoding");

        assert_eq!(first, second);
    }
}
